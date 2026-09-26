//! o200k_base token counts, computed the way tiktoken's
//! `encode_ordinary` computes them: split the text with the o200k
//! pre-tokenizer pattern, then byte-pair merge each piece.
//! `o200k_base.tiktoken` is OpenAI's rank file, unmodified.

use std::cell::RefCell;
use std::sync::OnceLock;

use regex::Regex;
use rustc_hash::FxHashMap;

/// OpenAI's o200k pre-tokenizer pattern, anchored, with its `\s+(?!\S)`
/// alternative dropped: [`Encoding::count`] recovers that alternative
/// from the `\s+` match that takes its place. Every character starts a
/// match, so anchored matches tile the text.
const PIECE_PATTERN: &str = concat!(
    r"^(?:[^\r\n\p{L}\p{N}]?[\p{Lu}\p{Lt}\p{Lm}\p{Lo}\p{M}]*[\p{Ll}\p{Lm}\p{Lo}\p{M}]+(?i:'s|'t|'re|'ve|'m|'ll|'d)?",
    r"|[^\r\n\p{L}\p{N}]?[\p{Lu}\p{Lt}\p{Lm}\p{Lo}\p{M}]+[\p{Ll}\p{Lm}\p{Lo}\p{M}]*(?i:'s|'t|'re|'ve|'m|'ll|'d)?",
    r"|\p{N}{1,3}",
    r"| ?[^\s\p{L}\p{N}]+[\r\n/]*",
    r"|\s*[\r\n]+",
    r"|\s+)",
);

struct Encoding {
    ranks: FxHashMap<&'static [u8], u32>,
    pieces: Regex,
}

fn encoding() -> &'static Encoding {
    static ENCODING: OnceLock<Encoding> = OnceLock::new();
    ENCODING.get_or_init(|| {
        let ranks = load_ranks(include_bytes!("o200k_base.tiktoken"));
        assert_eq!(
            ranks.len(),
            199_998,
            "o200k_base.tiktoken is not the vendored file"
        );
        Encoding {
            ranks,
            pieces: Regex::new(PIECE_PATTERN).expect("o200k piece pattern"),
        }
    })
}

/// Build the o200k tables on a background thread; a [`count`] that
/// arrives first waits for them.
pub fn warm_up() {
    std::thread::spawn(encoding);
}

thread_local! {
    /// Per-thread memoization of `count(text)` — the scheduler
    /// tokenizes the same rendered line many times per pass.
    static CACHE: RefCell<FxHashMap<String, usize>> = RefCell::default();
}

/// Token count under o200k_base (ordinary encoding).
pub fn count(text: &str) -> usize {
    CACHE.with(|c| {
        if let Some(&v) = c.borrow().get(text) {
            return v;
        }
        let v = encoding().count(text);
        c.borrow_mut().insert(text.to_string(), v);
        v
    })
}

/// Rank-file lines are `<base64 token bytes> <rank>`. Scanned as bytes:
/// `str::lines` and `str::parse` cost more than the decoding.
fn load_ranks(rank_file: &[u8]) -> FxHashMap<&'static [u8], u32> {
    let mut bytes = Vec::with_capacity(rank_file.len());
    let mut tokens = Vec::new();
    for line in rank_file.split(|&byte| byte == b'\n') {
        let Some(space) = line.iter().position(|&byte| byte == b' ') else {
            continue;
        };
        let start = bytes.len();
        decode_base64(&line[..space], &mut bytes);
        let rank = line[space + 1..]
            .iter()
            .take_while(|symbol| symbol.is_ascii_digit())
            .fold(0, |rank, digit| rank * 10 + u32::from(digit - b'0'));
        tokens.push((start..bytes.len(), rank));
    }
    let bytes: &'static [u8] = bytes.leak();
    let mut ranks = FxHashMap::with_capacity_and_hasher(tokens.len(), Default::default());
    for (range, rank) in tokens {
        ranks.insert(&bytes[range], rank);
    }
    ranks
}

/// Padded base64: whole 4-symbol groups, `=` decoding as zero bits
/// that are dropped at the end.
fn decode_base64(text: &[u8], out: &mut Vec<u8>) {
    let value = |symbol: u8| match symbol {
        b'A'..=b'Z' => symbol - b'A',
        b'a'..=b'z' => symbol - b'a' + 26,
        b'0'..=b'9' => symbol - b'0' + 52,
        b'+' => 62,
        b'/' => 63,
        _ => 0,
    };
    for group in text.chunks(4) {
        let bits = group
            .iter()
            .fold(0u32, |bits, &symbol| bits << 6 | u32::from(value(symbol)));
        out.extend_from_slice(&bits.to_be_bytes()[1..]);
    }
    let padding = text
        .iter()
        .rev()
        .take_while(|&&symbol| symbol == b'=')
        .count();
    out.truncate(out.len() - padding);
}

impl Encoding {
    fn count(&self, text: &str) -> usize {
        let mut tokens = 0;
        let mut rest = text;
        while let Some(found) = self.pieces.find(rest) {
            let mut piece = found.as_str();
            // A whitespace run that `\s+` took whole, short of the end of
            // the text: `\s+(?!\S)` ranks first and leaves its last
            // character to the piece after it.
            if piece.len() < rest.len()
                && let Some(last) = piece.chars().next_back()
                && last.is_whitespace()
                && !matches!(last, '\r' | '\n')
                && piece.len() > last.len_utf8()
            {
                piece = &piece[..piece.len() - last.len_utf8()];
            }
            tokens += self.piece_tokens(piece.as_bytes());
            rest = &rest[piece.len()..];
        }
        debug_assert!(rest.is_empty(), "o200k pieces must tile the text");
        tokens
    }

    /// tiktoken's byte-pair merge: repeatedly join the adjacent pair
    /// whose joined bytes rank lowest, the leftmost on a tie.
    fn piece_tokens(&self, piece: &[u8]) -> usize {
        if self.ranks.contains_key(piece) {
            return 1;
        }
        let rank = |bytes: &[u8]| self.ranks.get(bytes).copied().unwrap_or(u32::MAX);
        // (start of part, rank of the part joined with the next one)
        let mut parts: Vec<(usize, u32)> = (0..piece.len() - 1)
            .map(|start| (start, rank(&piece[start..start + 2])))
            .collect();
        parts.push((piece.len() - 1, u32::MAX));
        parts.push((piece.len(), u32::MAX));
        let joined_rank = |parts: &[(usize, u32)], index: usize| {
            if index + 3 < parts.len() {
                rank(&piece[parts[index].0..parts[index + 3].0])
            } else {
                u32::MAX
            }
        };
        loop {
            let (index, lowest) = parts[..parts.len() - 1].iter().enumerate().fold(
                (0, u32::MAX),
                |(best_index, best), (index, &(_, rank))| {
                    if rank < best {
                        (index, rank)
                    } else {
                        (best_index, best)
                    }
                },
            );
            if lowest == u32::MAX {
                return parts.len() - 1;
            }
            if index > 0 {
                parts[index - 1].1 = joined_rank(&parts, index - 1);
            }
            parts[index].1 = joined_rank(&parts, index);
            parts.remove(index + 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    /// tiktoken-rs is the implementation this module replaces.
    fn assert_matches_tiktoken(text: &str) {
        static REFERENCE: OnceLock<tiktoken_rs::CoreBPE> = OnceLock::new();
        let reference = REFERENCE.get_or_init(|| tiktoken_rs::o200k_base().unwrap());
        let expected = reference.encode_ordinary(text).len();
        assert_eq!(encoding().count(text), expected, "{text:?}");
    }

    #[test]
    fn tokenizer_matches_tiktoken_on_random_text() {
        let alphabet: Vec<char> = " \t\r\n\u{b}\u{c}\u{85}\u{a0}\u{2028}\u{2029}\u{3000}\u{feff}\
            aZéÉßǅʰ中文٣0129'sSdDllveRE/.,;:-_(){}[]<>\"`~!@#$%^&*=+|\\?😀\u{301}\u{200d}"
            .chars()
            .collect();
        let mut state = 0x2545_F491_4F6C_DD1Du64;
        let mut next = || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state as usize
        };
        for _ in 0..200_000 {
            let text: String = (0..next() % 40)
                .map(|_| alphabet[next() % alphabet.len()])
                .collect();
            assert_matches_tiktoken(&text);
        }
    }

    #[test]
    fn tokenizer_matches_tiktoken_on_edge_cases() {
        let whitespace = [
            " ", "\t", "\u{a0}", "\u{85}", "\u{1680}", "\u{2000}", "\u{2005}", "\u{200a}",
            "\u{2028}", "\u{202f}", "\u{205f}", "\u{3000}",
        ];
        for space in whitespace {
            for run in [1, 2, 5] {
                let spaces = space.repeat(run);
                assert_matches_tiktoken(&format!("a{spaces}b"));
                assert_matches_tiktoken(&format!("a{spaces}"));
                assert_matches_tiktoken(&format!("{spaces} {spaces}x"));
                assert_matches_tiktoken(&format!("x\u{3000}{spaces}\n{spaces}"));
            }
        }
        for text in [
            "it'\u{17f} he'\u{17f}\u{17f}",
            "IT'\u{17f} We'LL",
            "\u{2160}\u{2161}\u{2162}\u{2163}\u{2164}",
            "\u{bd}\u{bc}\u{b2}\u{b3}\u{2460}\u{2461}",
            "\u{663}\u{664}\u{665}\u{666}\u{667}\u{968}\u{969}\u{96a}\u{96b}",
            "\u{ff11}\u{ff12}\u{ff13}\u{ff14}\u{3007}\u{3007}",
            "line one\r\nline two\r\n\r\n  \r\n",
            " \r\n\t\r\nx\r",
        ] {
            assert_matches_tiktoken(text);
        }
        for unit in ["=", " ", "ab", "\u{3000}", "😀", "e\u{301}"] {
            assert_matches_tiktoken(&unit.repeat(2000));
            assert_matches_tiktoken(&format!("x{}y", unit.repeat(2000)));
        }
    }

    #[test]
    fn tokenizer_matches_tiktoken_on_crate_sources() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let mut dirs = vec![root.join("src")];
        let mut files = vec![root.join("README.md")];
        while let Some(dir) = dirs.pop() {
            for entry in std::fs::read_dir(dir).unwrap() {
                let path = entry.unwrap().path();
                if path.is_dir() {
                    dirs.push(path);
                } else if path.extension().is_some_and(|ext| ext == "rs") {
                    files.push(path);
                }
            }
        }
        for file in files {
            let text = std::fs::read_to_string(file).unwrap();
            assert_matches_tiktoken(&text);
            for line in text.lines() {
                assert_matches_tiktoken(line);
                assert_matches_tiktoken(&format!("{line}\n"));
            }
        }
    }
}

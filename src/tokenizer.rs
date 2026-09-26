//! o200k_base token counts, computed the way tiktoken's
//! `encode_ordinary` computes them: split the text with the o200k
//! pre-tokenizer pattern, then byte-pair merge each piece.
//! `o200k_base.tiktoken` is OpenAI's rank file, unmodified.

use std::cell::RefCell;
use std::collections::HashMap;
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
    ENCODING.get_or_init(|| Encoding {
        ranks: load_ranks(include_str!("o200k_base.tiktoken")),
        pieces: Regex::new(PIECE_PATTERN).expect("o200k piece pattern"),
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
    static CACHE: RefCell<HashMap<String, usize>> = RefCell::new(HashMap::new());
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

/// Rank-file lines are `<base64 token bytes> <rank>`.
fn load_ranks(rank_file: &str) -> FxHashMap<&'static [u8], u32> {
    let mut bytes = Vec::with_capacity(rank_file.len());
    let mut tokens = Vec::new();
    for line in rank_file.lines() {
        let (token, rank) = line.split_once(' ').expect("rank line");
        let start = bytes.len();
        decode_base64(token, &mut bytes);
        tokens.push((start..bytes.len(), rank.parse().expect("rank")));
    }
    let bytes: &'static [u8] = bytes.leak();
    let mut ranks = FxHashMap::with_capacity_and_hasher(tokens.len(), Default::default());
    for (range, rank) in tokens {
        ranks.insert(&bytes[range], rank);
    }
    ranks
}

fn decode_base64(text: &str, out: &mut Vec<u8>) {
    let mut bits = 0u32;
    let mut bit_count = 0;
    for symbol in text.bytes() {
        let value = match symbol {
            b'A'..=b'Z' => symbol - b'A',
            b'a'..=b'z' => symbol - b'a' + 26,
            b'0'..=b'9' => symbol - b'0' + 52,
            b'+' => 62,
            b'/' => 63,
            _ => break,
        };
        bits = bits << 6 | u32::from(value);
        bit_count += 6;
        if bit_count >= 8 {
            bit_count -= 8;
            out.push((bits >> bit_count) as u8);
        }
    }
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

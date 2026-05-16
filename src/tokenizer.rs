use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::OnceLock;

use tiktoken_rs::CoreBPE;

/// Bytes-per-token constant for the approximate estimator. Global
/// median over the 66-fixture corpus, fit by
/// `scripts/calibrate_bytes_per_token.sh`.
pub const BYTES_PER_TOKEN_K: f64 = 3.6;

fn bpe() -> &'static CoreBPE {
    static BPE: OnceLock<CoreBPE> = OnceLock::new();
    BPE.get_or_init(|| tiktoken_rs::o200k_base().expect("o200k_base init"))
}

/// Approximate token count from byte length. Used by the scheduler's
/// approx-ranking pass; ceil + clamp-to-1 ensures non-empty rows never
/// score as zero-cost (which would compute as `f64::INFINITY` ratio).
pub fn approx_count(text: &str) -> usize {
    if text.is_empty() {
        return 0;
    }
    ((text.len() as f64 / BYTES_PER_TOKEN_K).ceil() as usize).max(1)
}

thread_local! {
    /// Per-thread memoization of `count(text)`. The scheduler calls
    /// `format_line_row` + `count` many times for the same rendered line
    /// (every `best_exact` pass), so keying on the exact string and
    /// avoiding re-tokenization is a meaningful speedup on large repos.
    static CACHE: RefCell<HashMap<String, usize>> = RefCell::new(HashMap::new());
}

/// Token count under o200k_base, using ordinary (no-special-token) encoding so
/// it matches what `scripts/count-tokens.py` reports to the North Star author.
pub fn count(text: &str) -> usize {
    #[cfg(feature = "timing")]
    let _start = std::time::Instant::now();
    CACHE.with(|c| {
        if let Some(&v) = c.borrow().get(text) {
            #[cfg(feature = "timing")]
            crate::timing::record(|c| &mut c.tokenizer, _start.elapsed(), Some(true));
            return v;
        }
        let v = bpe().encode_ordinary(text).len();
        c.borrow_mut().insert(text.to_string(), v);
        #[cfg(feature = "timing")]
        crate::timing::record(|c| &mut c.tokenizer, _start.elapsed(), Some(false));
        v
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizer_approx_count_empty_is_zero() {
        assert_eq!(approx_count(""), 0);
    }

    #[test]
    fn tokenizer_approx_count_one_byte_is_at_least_one() {
        assert!(approx_count("x") >= 1);
    }

    #[test]
    fn tokenizer_approx_count_short_rows_never_zero() {
        for len in 1..=10 {
            let s = "a".repeat(len);
            assert!(approx_count(&s) >= 1, "len {len} got zero");
        }
    }
}

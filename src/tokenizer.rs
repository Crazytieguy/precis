use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::OnceLock;

use tiktoken_rs::CoreBPE;

/// Bytes-per-token for the approx estimator (corpus median).
pub const BYTES_PER_TOKEN_K: f64 = 3.6;

fn bpe() -> &'static CoreBPE {
    static BPE: OnceLock<CoreBPE> = OnceLock::new();
    BPE.get_or_init(|| tiktoken_rs::o200k_base().expect("o200k_base init"))
}

/// Approximate token count by byte length. Clamp-to-1 keeps non-empty
/// rows from scoring as `INFINITY` ratio.
pub fn approx_count(text: &str) -> usize {
    if text.is_empty() {
        return 0;
    }
    ((text.len() as f64 / BYTES_PER_TOKEN_K).ceil() as usize).max(1)
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
        let v = bpe().encode_ordinary(text).len();
        c.borrow_mut().insert(text.to_string(), v);
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

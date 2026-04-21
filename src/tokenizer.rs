use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::OnceLock;

use tiktoken_rs::CoreBPE;

fn bpe() -> &'static CoreBPE {
    static BPE: OnceLock<CoreBPE> = OnceLock::new();
    BPE.get_or_init(|| tiktoken_rs::o200k_base().expect("o200k_base init"))
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
    CACHE.with(|c| {
        if let Some(&v) = c.borrow().get(text) {
            return v;
        }
        let v = bpe().encode_ordinary(text).len();
        c.borrow_mut().insert(text.to_string(), v);
        v
    })
}

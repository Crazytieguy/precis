use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::OnceLock;

use tiktoken_rs::CoreBPE;

fn bpe() -> &'static CoreBPE {
    static BPE: OnceLock<CoreBPE> = OnceLock::new();
    BPE.get_or_init(|| tiktoken_rs::o200k_base().expect("o200k_base init"))
}

/// Build the o200k tables (tens of milliseconds) on a background thread;
/// a [`count`] that arrives first waits for them.
pub fn warm_up() {
    std::thread::spawn(bpe);
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

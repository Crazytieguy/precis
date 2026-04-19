use std::sync::OnceLock;

use tiktoken_rs::CoreBPE;

fn bpe() -> &'static CoreBPE {
    static BPE: OnceLock<CoreBPE> = OnceLock::new();
    BPE.get_or_init(|| tiktoken_rs::o200k_base().expect("o200k_base init"))
}

/// Token count under o200k_base, using ordinary (no-special-token) encoding so
/// it matches what `scripts/count-tokens.py` reports to the North Star author.
pub fn count(text: &str) -> usize {
    bpe().encode_ordinary(text).len()
}

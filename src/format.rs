use std::path::Path;
use std::sync::LazyLock;

use tiktoken_rs::CoreBPE;

/// Shared BPE tokenizer instance (o200k_base).
static BPE: LazyLock<CoreBPE> = LazyLock::new(|| tiktoken_rs::o200k_base().unwrap());

/// Format a single source line with its line number.
/// Line numbers are 1-indexed; `line_idx_0` is the 0-based index.
pub fn fmt_line(line_idx_0: usize, line: &str) -> String {
    format!("{:>6}→{}\n", line_idx_0 + 1, line)
}

/// Plain truncation marker.
pub const TRUNCATION_MARKER: &str = "      →…\n";

/// Count BPE tokens in text using the o200k_base tokenizer.
pub fn count_tokens(text: &str) -> usize {
    BPE.encode_with_special_tokens(text).len()
}

/// Format a file header line from a path.
/// This is the single source of truth for file headers (V3).
pub fn header_line(path: &Path) -> String {
    format!("{}\n", path.display())
}

/// Format a folder header line (trailing slash).
pub fn folder_line(path: &Path) -> String {
    format!("{}/\n", path.display())
}

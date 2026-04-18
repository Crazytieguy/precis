use anyhow::Result;
use std::path::Path;

pub fn render(
    _paths: &[impl AsRef<Path>],
    _token_budget: usize,
    _char_budget: Option<usize>,
) -> Result<String> {
    todo!("v0.2 implementation pending")
}

use anyhow::{Result, bail};
use clap::Parser;
use std::path::PathBuf;

/// Claude Code hook `additionalContext` is capped at 10,000 bytes.
/// The plugin wrapper adds header/help/fences around precis output —
/// 570 bytes measured; the margin below absorbs help-text drift.
const PLUGIN_BYTE_BUDGET: usize = 9350;

#[derive(Parser)]
#[command(about = "Extract a token-efficient summary of a directory", version)]
struct Cli {
    /// Directory to summarize
    #[arg(default_value = ".")]
    path: PathBuf,

    /// Token budget for output
    #[arg(long, default_value = "3000", visible_alias = "budget")]
    token_budget: usize,

    /// Byte budget for output (hard upper bound; tokens are still the optimization target)
    #[arg(long, visible_alias = "char-budget")]
    byte_budget: Option<usize>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let byte_budget = cli.byte_budget.or_else(|| {
        std::env::var("CLAUDE_PLUGIN_ROOT")
            .ok()
            .map(|_| PLUGIN_BYTE_BUDGET)
    });

    if !cli.path.exists() {
        bail!("{:?} does not exist", cli.path);
    }

    let output = precis::render(&[&cli.path], cli.token_budget, byte_budget)?;
    print!("{}", output);
    Ok(())
}

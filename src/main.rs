use anyhow::{Result, bail};
use clap::Parser;
use std::path::PathBuf;

/// Claude Code hook `additionalContext` is capped at 10,000 characters.
/// The plugin wrapper adds ~400 chars of header/help/fences around precis output.
const PLUGIN_CHAR_BUDGET: usize = 9500;

#[derive(Parser)]
#[command(
    about = "Extract a token-efficient summary of one or more paths",
    version
)]
struct Cli {
    /// Directories or files to summarize (defaults to the current directory)
    paths: Vec<PathBuf>,

    /// Token budget for output
    #[arg(long, default_value = "3000")]
    token_budget: usize,

    /// Character budget for output
    #[arg(long)]
    char_budget: Option<usize>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let paths = if cli.paths.is_empty() {
        vec![PathBuf::from(".")]
    } else {
        cli.paths
    };
    let char_budget = cli.char_budget.or_else(|| {
        std::env::var("CLAUDE_PLUGIN_ROOT")
            .ok()
            .map(|_| PLUGIN_CHAR_BUDGET)
    });

    for path in &paths {
        if !path.exists() {
            bail!("{:?} does not exist", path);
        }
    }

    let output = precis::render(&paths, cli.token_budget, char_budget)?;
    print!("{}", output);
    Ok(())
}

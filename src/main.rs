use clap::Parser;
use std::path::PathBuf;

/// Claude Code hook `additionalContext` is capped at 10,000 characters.
/// The plugin wrapper adds ~400 chars of header/help/fences around precis output.
const PLUGIN_CHAR_BUDGET: usize = 9500;

#[derive(Parser)]
#[command(about = "Extract a token-efficient summary of a path", version)]
struct Cli {
    /// Directory or file to summarize
    #[arg(default_value = ".")]
    path: PathBuf,

    /// Token budget for output
    #[arg(long, default_value = "4000")]
    budget: usize,

    /// Character budget for output
    #[arg(long)]
    char_budget: Option<usize>,
}

fn main() {
    let cli = Cli::parse();
    let path = &cli.path;
    let budget = cli.budget;
    let char_budget = cli.char_budget.or_else(|| {
        std::env::var("CLAUDE_PLUGIN_ROOT")
            .ok()
            .map(|_| PLUGIN_CHAR_BUDGET)
    });

    if !path.exists() {
        eprintln!("Error: {:?} does not exist", path);
        std::process::exit(1);
    }

    let output = precis::render(path, budget, char_budget);
    print!("{}", output);
}

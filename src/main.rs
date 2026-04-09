use clap::Parser;
use precis::{format, walk};
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
    let char_budget = cli
        .char_budget
        .or_else(|| std::env::var("CLAUDE_PLUGIN_ROOT").ok().map(|_| PLUGIN_CHAR_BUDGET));

    let output = if path.is_file() {
        let source = match std::fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Error reading {:?}: {}", path, e);
                std::process::exit(1);
            }
        };
        let root = path.parent().unwrap_or(path);
        format::render_file_with_budget(budget, char_budget, path, root, &source)
    } else if path.is_dir() {
        let files = walk::discover_source_files(path);
        let sources = format::read_sources(&files);
        format::render_with_budget(budget, char_budget, path, &files, &sources)
    } else {
        eprintln!("Error: {:?} is not a file or directory", path);
        std::process::exit(1);
    };
    print!("{}", output);
}

use clap::Parser;
use precis::{format, walk};
use std::path::PathBuf;

#[derive(Parser)]
#[command(about = "Extract a token-efficient summary of a codebase")]
struct Cli {
    /// Directory or file to summarize
    #[arg(default_value = ".")]
    path: PathBuf,

    /// Token budget for output
    #[arg(long, default_value = "4000")]
    budget: usize,

    /// Output as JSON
    #[arg(long)]
    json: bool,
}

fn main() {
    let cli = Cli::parse();
    let path = &cli.path;
    let budget = cli.budget;

    let (output, n_files, tokens) = if path.is_file() {
        let source = match std::fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Error reading {:?}: {}", path, e);
                std::process::exit(1);
            }
        };
        let root = path.parent().unwrap_or(path);
        let output = format::render_file_with_budget(budget, path, root, &source);
        let tokens = format::count_tokens(&output);
        (output, 1, tokens)
    } else if path.is_dir() {
        let files = walk::discover_source_files(path);
        let sources = format::read_sources(&files);
        let n_files = files.len();
        let (output, tokens) = format::render_with_budget_stats(budget, path, &files, &sources);
        (output, n_files, tokens)
    } else {
        eprintln!("Error: {:?} is not a file or directory", path);
        std::process::exit(1);
    };
    if cli.json {
        print!("{}", format::to_json(&output, budget, tokens));
    } else {
        print!("{}", output);
    }
}

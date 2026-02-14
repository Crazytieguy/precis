use clap::Parser;
use std::path::PathBuf;
use precis::{format, walk};

#[derive(Parser)]
#[command(about = "Extract a token-efficient summary of a codebase")]
struct Cli {
    /// Directory or file to summarize
    path: PathBuf,

    /// Token budget for output
    #[arg(long)]
    budget: Option<usize>,
}

fn main() {
    let cli = Cli::parse();

    let path = &cli.path;
    if !path.is_dir() {
        eprintln!("Error: {:?} is not a directory", path);
        std::process::exit(1);
    }

    let files = walk::discover_source_files(path);
    let level = if let Some(budget) = cli.budget {
        format::budget_level(budget, path)
    } else {
        format::MAX_LEVEL.min(1)
    };
    let output = format::render_directory(level, path);
    let words = format::count_words(&output);
    print!("{}", output);
    if let Some(budget) = cli.budget {
        eprintln!("({} files, level {}, {} words, budget {})", files.len(), level, words, budget);
    } else {
        eprintln!("({} files found)", files.len());
    }
}

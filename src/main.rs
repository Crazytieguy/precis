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
    if path.is_file() {
        let source = match std::fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Error reading {:?}: {}", path, e);
                std::process::exit(1);
            }
        };
        let root = path.parent().unwrap_or(path);
        let level = if let Some(budget) = cli.budget {
            format::budget_level_file(budget, path, root, &source)
        } else {
            format::MAX_LEVEL.min(1)
        };
        let output = format::render_file(level, path, root, &source);
        let words = format::count_words(&output);
        print!("{}", output);
        if let Some(budget) = cli.budget {
            eprintln!("(1 file, level {}, {} words, budget {})", level, words, budget);
        } else {
            eprintln!("(1 file)");
        }
    } else if path.is_dir() {
        let files = walk::discover_source_files(path);
        let level = if let Some(budget) = cli.budget {
            format::budget_level(budget, path)
        } else {
            format::MAX_LEVEL.min(1)
        };
        let output = format::render_files(level, path, &files);
        let words = format::count_words(&output);
        print!("{}", output);
        if let Some(budget) = cli.budget {
            eprintln!("({} files, level {}, {} words, budget {})", files.len(), level, words, budget);
        } else {
            eprintln!("({} files found)", files.len());
        }
    } else {
        eprintln!("Error: {:?} is not a file or directory", path);
        std::process::exit(1);
    }
}

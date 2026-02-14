use clap::Parser;
use std::path::PathBuf;
use precis::{format, walk};

#[derive(Parser)]
#[command(about = "Extract a token-efficient summary of a codebase")]
struct Cli {
    /// Directory or file to summarize
    path: PathBuf,

    /// Token budget for output (mutually exclusive with --level)
    #[arg(long, conflicts_with = "level")]
    budget: Option<usize>,

    /// Granularity level (0=paths, 1=names, 2=signatures, 3=+docs, 4=+type bodies, 5=full source)
    #[arg(long, value_parser = clap::value_parser!(u8).range(0..=5))]
    level: Option<u8>,
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
        let level = if let Some(l) = cli.level {
            l
        } else if let Some(budget) = cli.budget {
            format::budget_level_file(budget, path, root, &source)
        } else {
            format::MAX_LEVEL.min(1)
        };
        let output = format::render_file(level, path, root, &source);
        let words = format::count_words(&output);
        print!("{}", output);
        eprintln!("(1 file, level {}, {} words)", level, words);
    } else if path.is_dir() {
        let files = walk::discover_source_files(path);
        let sources = format::read_sources(&files);
        let level = if let Some(l) = cli.level {
            l
        } else if let Some(budget) = cli.budget {
            format::budget_level(budget, path, &files, &sources)
        } else {
            format::MAX_LEVEL.min(1)
        };
        let output = format::render_files(level, path, &files, &sources);
        let words = format::count_words(&output);
        print!("{}", output);
        eprintln!("({} files, level {}, {} words)", files.len(), level, words);
    } else {
        eprintln!("Error: {:?} is not a file or directory", path);
        std::process::exit(1);
    }
}

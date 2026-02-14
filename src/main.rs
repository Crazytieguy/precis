use clap::Parser;
use precis::{format, walk};
use std::path::PathBuf;

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
        let output = if let Some(l) = cli.level {
            let output = format::render_file(l, path, root, &source);
            let words = format::count_words(&output);
            eprintln!("(1 file, level {}, {} words)", l, words);
            output
        } else if let Some(budget) = cli.budget {
            let (level, symbols) = format::budget_level_file(budget, path, root, &source);
            let output = format::render_file_with_symbols(level, path, root, &source, &symbols);
            let words = format::count_words(&output);
            eprintln!("(1 file, level {}, {} words)", level, words);
            output
        } else {
            let level = format::MAX_LEVEL.min(1);
            let output = format::render_file(level, path, root, &source);
            let words = format::count_words(&output);
            eprintln!("(1 file, level {}, {} words)", level, words);
            output
        };
        print!("{}", output);
    } else if path.is_dir() {
        let files = walk::discover_source_files(path);
        let sources = format::read_sources(&files);
        let output = if let Some(l) = cli.level {
            let output = format::render_files(l, path, &files, &sources);
            let words = format::count_words(&output);
            eprintln!("({} files, level {}, {} words)", files.len(), l, words);
            output
        } else if let Some(budget) = cli.budget {
            let (level, all_symbols) = format::budget_level(budget, path, &files, &sources);
            let output =
                format::render_files_with_symbols(level, path, &files, &sources, &all_symbols);
            let words = format::count_words(&output);
            eprintln!("({} files, level {}, {} words)", files.len(), level, words);
            output
        } else {
            let level = format::MAX_LEVEL.min(1);
            let output = format::render_files(level, path, &files, &sources);
            let words = format::count_words(&output);
            eprintln!("({} files, level {}, {} words)", files.len(), level, words);
            output
        };
        print!("{}", output);
    } else {
        eprintln!("Error: {:?} is not a file or directory", path);
        std::process::exit(1);
    }
}

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

    /// Granularity level (0=paths, 1=names, 2=full signatures, 3=+docs, 4=+type bodies, 5=full source)
    #[arg(long, value_parser = clap::value_parser!(u8).range(0..=(format::MAX_LEVEL as i64)))]
    level: Option<u8>,

    /// Output as JSON
    #[arg(long)]
    json: bool,
}

fn main() {
    let cli = Cli::parse();
    let path = &cli.path;

    let (output, n_files, level) = if path.is_file() {
        render_file(path, cli.level, cli.budget)
    } else if path.is_dir() {
        render_dir(path, cli.level, cli.budget)
    } else {
        eprintln!("Error: {:?} is not a file or directory", path);
        std::process::exit(1);
    };

    let words = format::count_words(&output);
    eprintln!(
        "({} {}, level {}, {} words)",
        n_files,
        if n_files == 1 { "file" } else { "files" },
        level,
        words,
    );

    if cli.json {
        print!("{}", format::to_json(&output, level, words));
    } else {
        print!("{}", output);
    }
}

fn render_file(path: &std::path::Path, level: Option<u8>, budget: Option<usize>) -> (String, usize, u8) {
    let source = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading {:?}: {}", path, e);
            std::process::exit(1);
        }
    };
    let root = path.parent().unwrap_or(path);

    let (output, level) = if let Some(l) = level {
        (format::render_file(l, path, root, &source), l)
    } else if let Some(budget) = budget {
        let (l, symbols) = format::budget_level_file(budget, path, root, &source);
        (format::render_file_with_symbols(l, path, root, &source, &symbols), l)
    } else {
        let l = 1;
        (format::render_file(l, path, root, &source), l)
    };

    (output, 1, level)
}

fn render_dir(path: &std::path::Path, level: Option<u8>, budget: Option<usize>) -> (String, usize, u8) {
    let files = walk::discover_source_files(path);
    let sources = format::read_sources(&files);
    let n_files = files.len();

    let (output, level) = if let Some(l) = level {
        (format::render_files(l, path, &files, &sources), l)
    } else if let Some(budget) = budget {
        let (l, all_symbols) = format::budget_level(budget, path, &files, &sources);
        (format::render_files_with_symbols(l, path, &files, &sources, &all_symbols), l)
    } else {
        let l = 1;
        (format::render_files(l, path, &files, &sources), l)
    };

    (output, n_files, level)
}

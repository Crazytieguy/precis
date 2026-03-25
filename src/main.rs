use clap::Parser;
use precis::{format, walk};
use std::path::PathBuf;

#[derive(Parser)]
#[command(about = "Extract a token-efficient summary of a path", version)]
struct Cli {
    /// Directory or file to summarize
    #[arg(default_value = ".")]
    path: PathBuf,

    /// Token budget for output
    #[arg(long, default_value = "4000")]
    budget: usize,
}

fn main() {
    let cli = Cli::parse();
    let path = &cli.path;
    let budget = cli.budget;

    let output = if path.is_file() {
        let source = match std::fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Error reading {:?}: {}", path, e);
                std::process::exit(1);
            }
        };
        let root = path.parent().unwrap_or(path);
        format::render_file_with_budget(budget, path, root, &source)
    } else if path.is_dir() {
        let files = walk::discover_source_files(path);
        let sources = format::read_sources(&files);
        format::render_with_budget(budget, path, &files, &sources)
    } else {
        eprintln!("Error: {:?} is not a file or directory", path);
        std::process::exit(1);
    };
    print!("{}", output);
}

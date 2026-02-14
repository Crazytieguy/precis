use clap::Parser;
use std::path::PathBuf;
use symbols::{format, walk};

#[derive(Parser)]
#[command(about = "Extract symbols from a codebase for LLM context")]
struct Cli {
    /// Directory to extract symbols from
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
    print!("{}", format::format_directory(path));
    if let Some(budget) = cli.budget {
        eprintln!("({} files found, budget: {} tokens)", files.len(), budget);
    } else {
        eprintln!("({} files found)", files.len());
    }
}

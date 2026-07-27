use std::io::{self, Write};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use clap::Parser;

/// Claude Code hook `additionalContext` is capped at 10,000 bytes. The
/// plugin's session-start hook injects `--help` *and* a summary into that
/// one field, so long help text is paid for out of the summary's budget —
/// `main_plugin_context_fits_hook_cap` pins the arithmetic.
const PLUGIN_BYTE_BUDGET: usize = 9050;

/// clap does not wrap help text, so the lines below are pre-wrapped.
const ABOUT: &str = "\
Extract a token-efficient summary of a directory.

Notation: `N→` prefixes are source line numbers, so every entry is a jump
target. `…` means \"there is source here that isn't shown\" — alone on a line
for a gap inside a file, trailing a tree entry whose contents are hidden.
A name with no `…` and nothing under it is empty.";

#[derive(Parser)]
#[command(about = ABOUT, version)]
struct Cli {
    /// Directory to summarize
    #[arg(default_value = ".")]
    path: PathBuf,

    /// Token budget for output (o200k_base BPE tokens)
    #[arg(long, default_value = "3000", visible_alias = "budget")]
    token_budget: usize,

    /// Hard byte cap on output; rendering stops at whichever budget binds first
    #[arg(long, visible_alias = "char-budget")]
    byte_budget: Option<usize>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let byte_budget = cli
        .byte_budget
        .or_else(|| std::env::var_os("CLAUDE_PLUGIN_ROOT").map(|_| PLUGIN_BYTE_BUDGET));

    if !cli.path.exists() {
        bail!("{} does not exist", cli.path.display());
    }

    let output = precis::render(&[&cli.path], cli.token_budget, byte_budget)?;
    if output.is_empty() {
        warn_empty_output(&cli.path, cli.token_budget, byte_budget);
    }
    write_stdout(&output)
}

/// Empty output and exit 0 is indistinguishable from success in a script,
/// so say which of the two causes applies.
fn warn_empty_output(path: &Path, token_budget: usize, byte_budget: Option<usize>) {
    let shown = path.display();
    if std::fs::read_dir(path).is_ok_and(|mut dir| dir.next().is_none()) {
        eprintln!("precis: {shown} is empty");
        return;
    }
    let budgets = match byte_budget {
        Some(bytes) => format!("--token-budget {token_budget} --byte-budget {bytes}"),
        None => format!("--token-budget {token_budget}"),
    };
    eprintln!("precis: nothing fit in {budgets}; raise it, or everything under {shown} is ignored");
}

/// A reader that stops early (`precis . | head`) closes the pipe under us;
/// that is normal termination, not a panic.
fn write_stdout(output: &str) -> Result<()> {
    let mut stdout = io::stdout().lock();
    match stdout
        .write_all(output.as_bytes())
        .and_then(|()| stdout.flush())
    {
        Err(e) if e.kind() == io::ErrorKind::BrokenPipe => Ok(()),
        result => result.context("failed writing to stdout"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    /// Markdown headings and code fences the session-start hook wraps around
    /// `--help` and the summary (`plugins/precis/hooks/session-start.sh`).
    const PLUGIN_WRAPPER_BYTES: usize = 80;

    #[test]
    fn main_plugin_context_fits_hook_cap() {
        let help = Cli::command().render_help().to_string();
        let total = help.len() + PLUGIN_WRAPPER_BYTES + PLUGIN_BYTE_BUDGET;
        assert!(
            total <= 10_000,
            "help {} + wrapper {PLUGIN_WRAPPER_BYTES} + budget {PLUGIN_BYTE_BUDGET} = {total} \
             exceeds the 10,000-byte additionalContext cap; trim help or lower PLUGIN_BYTE_BUDGET",
            help.len()
        );
    }
}

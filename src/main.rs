use std::io::{self, Write};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use clap::{CommandFactory, Parser};

/// Claude Code keeps a hook's `additionalContext` inline only up to
/// 10,000 JavaScript string units (UTF-16 code units); past that it
/// swaps the text for a file path and a short preview.
const HOOK_CONTEXT_CAP: usize = 10_000;

/// The text `plugins/precis/hooks/session-start.sh` puts around `--help`
/// and the summary in that one field.
const HOOK_WRAPPER: [&str; 3] = [
    "## precis\n\nOutput of `precis --help`:\n\n```\n",
    "\n```\n\nOutput of `precis .`:\n\n```\n",
    "\n```",
];

/// clap does not wrap help text, so the lines below are pre-wrapped.
const ABOUT: &str = "\
Summarize a directory within a token budget.

`N→` rows are source lines, N the line number. A `…` row marks hidden
source in a file, or hidden entries in a directory. An entry with nothing
under it wasn't expanded, unless marked `(empty)`.";

#[derive(Parser)]
#[command(about = ABOUT, version)]
struct Cli {
    /// Directory to summarize
    #[arg(default_value = ".")]
    path: PathBuf,

    /// Output budget in o200k_base tokens
    #[arg(long, value_name = "N", default_value = "3000", alias = "budget")]
    token_budget: usize,

    /// Also cap output at N characters (UTF-16 units)
    #[arg(long, value_name = "N")]
    char_budget: Option<usize>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let char_budget = cli
        .char_budget
        .or_else(|| std::env::var_os("CLAUDE_PLUGIN_ROOT").map(|_| plugin_char_budget()));

    if !cli.path.exists() {
        bail!("{} does not exist", cli.path.display());
    }

    let output = precis::render(&cli.path, cli.token_budget, char_budget)?;
    if output.is_empty() {
        warn_empty_output(&cli.path, cli.token_budget, char_budget);
    }
    write_stdout(&output)
}

/// What is left of the hook cap once the session-start hook has spent
/// its share on the wrapper and on `--help`.
fn plugin_char_budget() -> usize {
    let help = Cli::command().render_help().to_string();
    let spent: usize = HOOK_WRAPPER
        .iter()
        .chain([&help.as_str()])
        .map(|text| precis::char_units(text))
        .sum();
    HOOK_CONTEXT_CAP.saturating_sub(spent)
}

/// Empty output and exit 0 is indistinguishable from success in a script,
/// so say which of the two causes applies.
fn warn_empty_output(path: &Path, token_budget: usize, char_budget: Option<usize>) {
    let shown = path.display();
    if std::fs::read_dir(path).is_ok_and(|mut dir| dir.next().is_none()) {
        eprintln!("precis: {shown} is empty");
        return;
    }
    let budgets = match char_budget {
        Some(chars) => format!("--token-budget {token_budget} --char-budget {chars}"),
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

    #[test]
    fn main_hook_wrapper_matches_session_start_script() {
        let script = include_str!("../plugins/precis/hooks/session-start.sh");
        for piece in HOOK_WRAPPER {
            let jq_literal = piece.replace('\n', "\\n");
            assert!(
                script.contains(&jq_literal),
                "session-start.sh no longer wraps with {jq_literal:?}; update HOOK_WRAPPER"
            );
        }
    }

    /// Rebuilds `additionalContext` the way the hook does on a fixture
    /// big enough for the character cap to bind.
    #[test]
    fn main_plugin_context_fits_hook_cap() {
        let fixture = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/toasty");
        let uncapped = precis::render(&fixture, 3000, None).unwrap();
        assert!(precis::char_units(&uncapped) > plugin_char_budget());
        let output = precis::render(&fixture, 3000, Some(plugin_char_budget())).unwrap();
        let help = Cli::command().render_help().to_string();
        let context = [
            HOOK_WRAPPER[0],
            help.trim_end_matches('\n'),
            HOOK_WRAPPER[1],
            output.trim_end_matches('\n'),
            HOOK_WRAPPER[2],
        ]
        .concat();
        let units = precis::char_units(&context);
        assert!(units <= HOOK_CONTEXT_CAP, "{units} units");
    }
}

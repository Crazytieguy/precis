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

/// Set by that script on its `precis .` run.
const SESSION_HOOK_VAR: &str = "PRECIS_SESSION_HOOK";

/// clap does not wrap help text, so the lines below are pre-wrapped.
const ABOUT: &str = "\
Summarize a directory or file within a token budget.

`N→` rows are source line N. An entry with nothing under it wasn't
expanded, unless marked `(empty)`. An `a/b/` row is a directory `a/`
holding only `b/`.";

#[derive(Parser)]
#[command(about = ABOUT, version)]
struct Cli {
    /// Directory or file to summarize
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
        .or_else(|| run_by_session_hook().then(plugin_char_budget));

    if !cli.path.exists() {
        bail!("{} does not exist", cli.path.display());
    }

    let output = precis::render(&cli.path, cli.token_budget, char_budget)?;
    if output.is_empty() {
        warn_empty_output(&cli.path, cli.token_budget, char_budget);
    }
    write_stdout(&output)
}

/// Whether precis's own session-start hook is running us, so the output
/// must fit the hook cap. Hooks from plugin versions before 0.2 don't set
/// [`SESSION_HOOK_VAR`] but still download this binary; they are
/// recognized by the plugin they run from. Claude Code sets
/// `CLAUDE_PLUGIN_ROOT` for every plugin's hooks, so that alone would
/// cap other plugins' runs too.
fn run_by_session_hook() -> bool {
    std::env::var_os(SESSION_HOOK_VAR).is_some()
        || std::env::var_os("CLAUDE_PLUGIN_ROOT").is_some_and(|root| {
            std::fs::read_to_string(Path::new(&root).join(".claude-plugin/plugin.json"))
                .is_ok_and(|manifest| is_precis_plugin_manifest(&manifest))
        })
}

fn is_precis_plugin_manifest(manifest: &str) -> bool {
    manifest.contains("\"name\": \"precis\"")
}

/// What is left of the hook cap once the session-start hook has spent
/// its share on the wrapper and on `--help`.
fn plugin_char_budget() -> usize {
    let help = help_output();
    let spent: usize = HOOK_WRAPPER
        .iter()
        .chain([&help.as_str()])
        .map(|text| precis::char_units(text))
        .sum();
    HOOK_CONTEXT_CAP.saturating_sub(spent)
}

/// What `precis --help` prints. clap switches `--help` to the long form
/// once any argument carries long help, which `render_help` would miss.
fn help_output() -> String {
    Cli::command()
        .try_get_matches_from(["precis", "--help"])
        .map_or_else(|help| help.to_string(), |_| String::new())
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
    if path.is_file() {
        eprintln!("precis: nothing fit in {budgets}; raise it");
    } else {
        eprintln!(
            "precis: nothing fit in {budgets}; raise it, or nothing under {shown} is listed (it is ignored, or links outside it)"
        );
    }
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
        let [before_help, before_output, after_output] =
            HOOK_WRAPPER.map(|piece| format!("\"{}\"", piece.replace('\n', "\\n")));
        let jq_expression =
            format!("{before_help} + $help + {before_output} + $output + {after_output}");
        assert!(
            script.contains(&jq_expression),
            "session-start.sh no longer builds {jq_expression:?}; update HOOK_WRAPPER"
        );
        let summary_run = format!("{SESSION_HOOK_VAR}=1 \"$PRECIS_BIN\" .");
        assert!(
            script.contains(&summary_run),
            "session-start.sh no longer runs {summary_run:?}"
        );
    }

    /// The 0.1 plugin's manifest, which sets no [`SESSION_HOOK_VAR`], and
    /// not another plugin's whose name starts the same.
    #[test]
    fn main_recognizes_legacy_plugin_manifest() {
        let legacy = r#"{
  "name": "precis",
  "version": "0.1.0",
  "description": "Automatic codebase structure context via precis"
}
"#;
        assert!(is_precis_plugin_manifest(legacy));
        let other = legacy.replace("\"precis\"", "\"precis-notes\"");
        assert!(!is_precis_plugin_manifest(&other));
    }
}

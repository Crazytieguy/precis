//! The binary's own branches: exit status on a bad path, and the stderr
//! notes that say why a summary came out empty.

use std::ffi::OsStr;
use std::process::{Command, Output};

fn precis<I: IntoIterator<Item = S>, S: AsRef<OsStr>>(args: I) -> Output {
    Command::new(env!("CARGO_BIN_EXE_precis"))
        .args(args)
        .env_remove("PRECIS_SESSION_HOOK")
        .env_remove("CLAUDE_PLUGIN_ROOT")
        .output()
        .unwrap()
}

/// Asserts a successful run printed nothing and returns its stderr.
fn empty_run_note(output: Output) -> String {
    assert!(output.status.success(), "{output:?}");
    assert!(output.stdout.is_empty(), "{output:?}");
    String::from_utf8(output.stderr).unwrap()
}

#[test]
fn cli_explains_missing_paths_and_empty_output() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path();
    let missing = precis([root.join("missing")]);
    assert!(!missing.status.success());
    assert!(
        String::from_utf8_lossy(&missing.stderr).contains("does not exist"),
        "{missing:?}"
    );

    let empty = root.join("empty");
    std::fs::create_dir(&empty).unwrap();
    assert!(empty_run_note(precis([&empty])).contains("is empty"));

    let project = root.join("project");
    std::fs::create_dir(&project).unwrap();
    std::fs::write(project.join("main.rs"), "fn main() {}\n").unwrap();
    let starved_dir = empty_run_note(precis([OsStr::new("--budget=0"), project.as_os_str()]));
    assert!(
        starved_dir.contains("nothing fit in --token-budget 0; raise it, or everything under"),
        "{starved_dir}"
    );
    let starved_file = empty_run_note(precis([
        OsStr::new("--char-budget=0"),
        project.join("main.rs").as_os_str(),
    ]));
    assert!(
        starved_file.ends_with("--char-budget 0; raise it\n"),
        "{starved_file}"
    );
}

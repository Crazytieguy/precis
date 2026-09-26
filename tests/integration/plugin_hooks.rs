//! The plugin's shell hooks, run as Claude Code runs them: JSON on stdin,
//! a decision (or nothing) on stdout.

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn hook_script(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("plugins/precis")
        .join(relative)
}

fn json_string(text: &str) -> String {
    format!("\"{}\"", text.replace('\\', "\\\\").replace('"', "\\\""))
}

/// Runs the permission hook on `command` from `cwd`; true when it
/// approves, false when it leaves the decision to the user's prompt.
fn approves(cwd: Option<&Path>, command: &str, home: &Path) -> bool {
    assert!(
        Command::new("jq").arg("--version").output().is_ok(),
        "the permission hook needs jq on PATH"
    );
    let mut input = format!("{{\"tool_input\":{{\"command\":{}}}", json_string(command));
    if let Some(cwd) = cwd {
        input.push_str(&format!(",\"cwd\":{}", json_string(&cwd.to_string_lossy())));
    }
    input.push('}');
    let mut child = Command::new("bash")
        .arg(hook_script("hooks/auto-approve-precis.sh"))
        .env("HOME", home)
        .env("CLAUDE_PLUGIN_DATA", home.join("plugin-data"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success(), "hook failed on {input}");
    let stdout = String::from_utf8(output.stdout).unwrap();
    if stdout.trim().is_empty() {
        return false;
    }
    assert!(stdout.contains("\"allow\""), "unexpected decision {stdout}");
    true
}

/// A project holding `sub/` and a `123` link to a directory outside it.
fn project_with_outside_link() -> (tempfile::TempDir, PathBuf) {
    let temp = tempfile::tempdir().unwrap();
    let project = temp.path().join("project");
    std::fs::create_dir_all(project.join("sub")).unwrap();
    std::fs::create_dir(temp.path().join("outside")).unwrap();
    std::os::unix::fs::symlink(temp.path().join("outside"), project.join("123")).unwrap();
    (temp, project)
}

#[test]
fn plugin_hooks_approve_paths_inside_project() {
    let (temp, project) = project_with_outside_link();
    let cwd = Some(project.as_path());
    let home = temp.path();
    for command in [
        "precis",
        "precis .",
        "precis sub",
        "precis ./sub --token-budget 8000",
        "precis --budget=500 sub",
        "precis sub --char-budget 9000 2>&1",
        "precis --help",
        "precis ~/project/sub",
    ] {
        assert!(approves(cwd, command, home), "{command}");
    }
}

#[test]
fn plugin_hooks_approve_numeric_path_only_inside_project() {
    let (temp, project) = project_with_outside_link();
    let home = temp.path();
    assert!(!approves(Some(&project), "precis 123", home));
    assert!(!approves(
        Some(&project),
        "precis --token-budget 8000 123",
        home
    ));
    std::fs::create_dir(project.join("456")).unwrap();
    assert!(approves(Some(&project), "precis 456", home));
}

#[test]
fn plugin_hooks_prompt_outside_project_or_unknown_syntax() {
    let (temp, project) = project_with_outside_link();
    let cwd = Some(project.as_path());
    let home = temp.path();
    for command in [
        "precis ..",
        "precis ../outside",
        "precis /",
        "precis ~",
        "precis ~root",
        "precis --output=x sub",
        "precis sub --token-budget",
        "precis --token-budget sub",
        "precis -- sub",
        "precis sub; cat /etc/passwd",
        "precis $HOME",
        "precis . | head",
    ] {
        assert!(!approves(cwd, command, home), "{command}");
    }
}

#[test]
fn plugin_hooks_prompt_without_cwd() {
    let (temp, _project) = project_with_outside_link();
    let home = temp.path();
    assert!(!approves(None, "precis /", home));
    assert!(!approves(Some(Path::new("")), "precis /", home));
    assert!(!approves(None, "precis", home));
}

#[test]
fn plugin_hooks_approve_through_symlinked_cwd() {
    let (temp, project) = project_with_outside_link();
    let link = temp.path().join("link");
    std::os::unix::fs::symlink(&project, &link).unwrap();
    assert!(approves(Some(&link), "precis sub", temp.path()));
    assert!(!approves(Some(&link), "precis 123", temp.path()));
}

//! The plugin's shell hooks, run as Claude Code runs them: JSON on stdin,
//! a decision (or nothing) on stdout.

use std::ffi::OsStr;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
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

/// A plugin script run as Claude Code runs it, outside any plugin root and
/// without the caller's session environment file.
fn hook(script: &str) -> Command {
    let mut command = Command::new("bash");
    command
        .arg(hook_script(script))
        .env_remove("CLAUDE_PLUGIN_ROOT")
        .env_remove("CLAUDE_ENV_FILE");
    command
}

/// Runs `command` with `input` on stdin and returns its stdout.
fn stdout_of(command: &mut Command, input: &str) -> String {
    let mut child = command
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
    assert!(output.status.success(), "{command:?} failed on {input}");
    String::from_utf8(output.stdout).unwrap()
}

/// Runs a plugin script with `input` on stdin and returns its stdout.
fn run_hook(script: &str, input: &str, env: &[(&str, &OsStr)]) -> String {
    stdout_of(hook(script).envs(env.iter().copied()), input)
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
    let plugin_data = home.join("plugin-data");
    let stdout = run_hook(
        "hooks/auto-approve-precis.sh",
        &input,
        &[
            ("HOME", home.as_os_str()),
            ("CLAUDE_PLUGIN_DATA", plugin_data.as_os_str()),
        ],
    );
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

/// The session hook without a binary to run says why, and adds the
/// plugin's data directory to PATH once however often it runs.
#[test]
fn plugin_hooks_session_start_without_binary() {
    let temp = tempfile::tempdir().unwrap();
    let plugin_data = temp.path().join("plugin-data");
    let env_file = temp.path().join("env");
    let env = [
        ("CLAUDE_PLUGIN_DATA", plugin_data.as_os_str()),
        ("CLAUDE_ENV_FILE", env_file.as_os_str()),
    ];
    let input = r#"{"source":"startup"}"#;
    for _ in 0..2 {
        let stdout = run_hook("hooks/session-start.sh", input, &env);
        assert!(stdout.contains("error, see"), "{stdout}");
    }
    let exports = std::fs::read_to_string(&env_file).unwrap();
    assert_eq!(exports.lines().count(), 1, "{exports}");
    assert!(
        exports.contains(&*plugin_data.to_string_lossy()),
        "{exports}"
    );

    let path = path_with_fakes(
        &temp.path().join("bin"),
        &[("uname", "echo MINGW64_NT-10.0")],
    );
    let stdout = run_hook(
        "hooks/session-start.sh",
        input,
        &[env[0], env[1], ("PATH", path.as_os_str())],
    );
    assert!(stdout.contains("unsupported platform"), "{stdout}");
}

/// A binary that is present but doesn't run (a truncated download, the
/// wrong architecture) is an error, not "available".
#[test]
fn plugin_hooks_session_start_with_broken_binary() {
    let temp = tempfile::tempdir().unwrap();
    let plugin_data = temp.path().join("plugin-data");
    std::fs::create_dir_all(&plugin_data).unwrap();
    let binary = plugin_data.join("precis");
    std::fs::write(&binary, "#!/bin/sh\necho 'broken binary' >&2\nexit 126\n").unwrap();
    std::fs::set_permissions(&binary, PermissionsExt::from_mode(0o755)).unwrap();
    let stdout = run_hook(
        "hooks/session-start.sh",
        r#"{"source":"startup"}"#,
        &[("CLAUDE_PLUGIN_DATA", plugin_data.as_os_str())],
    );
    assert!(stdout.contains("error, see"), "{stdout}");
    let log = std::fs::read_to_string(plugin_data.join("error.log")).unwrap();
    assert!(log.contains("broken binary"), "{log}");
}

/// The session hook runs the real binary on a fixture whose uncapped
/// summary would push `additionalContext` past Claude Code's 10,000-unit
/// inline limit; the binary caps itself so the whole context stays inline.
#[test]
fn plugin_hooks_session_start_context_fits_hook_cap() {
    const HOOK_CONTEXT_CAP: usize = 10_000;
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/toasty");
    assert!(
        fixture.is_dir(),
        "{} is missing; run `cargo run --example clone_fixtures`",
        fixture.display()
    );
    let temp = tempfile::tempdir().unwrap();
    let plugin_data = temp.path().join("plugin-data");
    std::fs::create_dir_all(&plugin_data).unwrap();
    std::os::unix::fs::symlink(env!("CARGO_BIN_EXE_precis"), plugin_data.join("precis")).unwrap();

    let hook_output = stdout_of(
        hook("hooks/session-start.sh")
            .current_dir(&fixture)
            .env("CLAUDE_PLUGIN_DATA", &plugin_data),
        r#"{"source":"startup"}"#,
    );
    let context = stdout_of(
        Command::new("jq").args(["-j", ".hookSpecificOutput.additionalContext"]),
        &hook_output,
    );
    assert!(context.contains("Output of `precis .`"), "{context}");

    let uncapped = Command::new(env!("CARGO_BIN_EXE_precis"))
        .current_dir(&fixture)
        .env_remove("PRECIS_SESSION_HOOK")
        .env_remove("CLAUDE_PLUGIN_ROOT")
        .output()
        .unwrap();
    assert!(precis::char_units(&String::from_utf8(uncapped.stdout).unwrap()) > HOOK_CONTEXT_CAP);
    let units = precis::char_units(&context);
    assert!(units <= HOOK_CONTEXT_CAP, "{units} units");
}

/// A directory of executable shell scripts, prepended to PATH.
fn path_with_fakes(dir: &Path, fakes: &[(&str, &str)]) -> std::ffi::OsString {
    std::fs::create_dir_all(dir).unwrap();
    for (name, body) in fakes {
        let fake = dir.join(name);
        std::fs::write(&fake, format!("#!/bin/sh\n{body}\n")).unwrap();
        std::fs::set_permissions(&fake, PermissionsExt::from_mode(0o755)).unwrap();
    }
    let search_path = std::env::var_os("PATH").unwrap();
    std::env::join_paths(
        std::iter::once(dir.to_path_buf()).chain(std::env::split_paths(&search_path)),
    )
    .unwrap()
}

/// The installer logs why it gave up, since the session hook's only
/// message on a missing binary points at the log.
#[test]
fn plugin_hooks_install_logs_unsupported_arch() {
    let temp = tempfile::tempdir().unwrap();
    let plugin_data = temp.path().join("plugin-data");
    let path = path_with_fakes(
        &temp.path().join("bin"),
        &[(
            "uname",
            r#"case "$1" in -s) echo Linux ;; -m) echo armv7l ;; esac"#,
        )],
    );
    let status = Command::new("bash")
        .arg(hook_script("scripts/ensure-precis.sh"))
        .arg("--install")
        .env("CLAUDE_PLUGIN_DATA", &plugin_data)
        .env("PATH", &path)
        .status()
        .unwrap();
    assert!(status.success());
    let log = std::fs::read_to_string(plugin_data.join("error.log")).unwrap();
    assert!(log.contains("no release binary for linux-armv7l"), "{log}");
}

/// An installed plugin asks GitHub for the latest release at most once
/// an hour, however many sessions start.
#[test]
fn plugin_hooks_update_checks_at_most_hourly() {
    let temp = tempfile::tempdir().unwrap();
    let plugin_data = temp.path().join("plugin-data");
    std::fs::create_dir_all(&plugin_data).unwrap();
    let binary = plugin_data.join("precis");
    std::fs::write(&binary, "#!/bin/sh\n").unwrap();
    std::fs::set_permissions(&binary, PermissionsExt::from_mode(0o755)).unwrap();
    std::fs::write(plugin_data.join("version"), "v9.9.9\n").unwrap();
    let calls = temp.path().join("curl-calls");
    let path = path_with_fakes(
        &temp.path().join("bin"),
        &[(
            "curl",
            &format!(
                r#"echo "$*" >> '{}'; echo '{{"tag_name":"v9.9.9"}}'"#,
                calls.display()
            ),
        )],
    );
    for _ in 0..3 {
        let status = Command::new("bash")
            .arg(hook_script("scripts/ensure-precis.sh"))
            .env("CLAUDE_PLUGIN_DATA", &plugin_data)
            .env("PATH", &path)
            .status()
            .unwrap();
        assert!(status.success());
    }
    let calls = std::fs::read_to_string(&calls).unwrap();
    assert_eq!(calls.lines().count(), 1, "{calls}");
}

/// Work dirs left by killed updates are cleared once they are old
/// enough not to belong to an update still running.
#[test]
fn plugin_hooks_update_clears_stale_work_dirs() {
    let temp = tempfile::tempdir().unwrap();
    let plugin_data = temp.path().join("plugin-data");
    let stale = plugin_data.join("update.stale");
    let recent = plugin_data.join("update.recent");
    std::fs::create_dir_all(&stale).unwrap();
    std::fs::create_dir_all(&recent).unwrap();
    let touched = Command::new("touch")
        .args(["-t", "202001010000"])
        .arg(&stale)
        .status()
        .unwrap();
    assert!(touched.success());
    let path = path_with_fakes(
        &temp.path().join("bin"),
        &[(
            "curl",
            r#"case "$*" in *api.github.com*) echo '{"tag_name":"v9.9.9"}' ;; *) exit 22 ;; esac"#,
        )],
    );
    let status = Command::new("bash")
        .arg(hook_script("scripts/ensure-precis.sh"))
        .arg("--install")
        .env("CLAUDE_PLUGIN_DATA", &plugin_data)
        .env("PATH", &path)
        .status()
        .unwrap();
    assert!(status.success());
    assert!(!stale.exists());
    assert!(recent.exists());
}

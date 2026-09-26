//! Real checkouts hold files and trees nobody designs for. On each hazard
//! precis must finish, not panic, and render what it can: the rest of the
//! tree as usual, the hazard as a bare row or not at all. Every tree is
//! built in a tempdir.
//!
//! precis summarizes *a path*, so nothing outside it may reach the output
//! however the checkout's links are shaped, and the walk must terminate
//! whatever cycles they form.

use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::time::Duration;

fn write(path: &Path, contents: impl AsRef<[u8]>) {
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, contents).unwrap();
}

fn render(path: &Path, token_budget: usize) -> anyhow::Result<String> {
    render_within(path, token_budget, Duration::from_secs(60))
}

/// Renders on a worker, so a walk that never terminates fails as an
/// assertion instead of wedging the suite.
fn render_within(path: &Path, token_budget: usize, limit: Duration) -> anyhow::Result<String> {
    let (sender, receiver) = mpsc::channel();
    let target = path.to_path_buf();
    std::thread::spawn(move || {
        let _ = sender.send(precis::render(&target, token_budget, None));
    });
    receiver
        .recv_timeout(limit)
        .unwrap_or_else(|_| panic!("no render of {} within {limit:?}", path.display()))
}

#[cfg(unix)]
fn mkfifo(path: &Path) {
    let status = std::process::Command::new("mkfifo")
        .arg(path)
        .status()
        .unwrap();
    assert!(status.success());
}

#[test]
fn robustness_empty_trees_render_nothing() {
    let temp = tempfile::tempdir().unwrap();
    let empty = temp.path().join("empty");
    std::fs::create_dir(&empty).unwrap();
    assert_eq!(render(&empty, 3000).unwrap(), "");

    let repo = temp.path().join("repo");
    std::fs::create_dir(&repo).unwrap();
    let status = std::process::Command::new("git")
        .arg("init")
        .arg("--quiet")
        .arg(&repo)
        .status()
        .unwrap();
    assert!(status.success());
    assert_eq!(render(&repo, 3000).unwrap(), "");
}

#[test]
fn robustness_root_with_only_a_license_lists_it() {
    let temp = tempfile::tempdir().unwrap();
    write(
        &temp.path().join("LICENSE"),
        "MIT License\n\nCopyright (c) 2026 Someone\n\nPermission is hereby granted, free of charge.\n",
    );
    let out = render(temp.path(), 3000).unwrap();
    assert!(out.starts_with("LICENSE\n"), "{out}");
}

/// Reading a FIFO blocks until a writer appears, and a socket cannot be
/// read at all: listings leave both out, links to them included, and a
/// named one is an error.
#[cfg(unix)]
#[test]
fn robustness_special_files_are_never_read() {
    let temp = tempfile::tempdir().unwrap();
    write(&temp.path().join("main.py"), "def main():\n    return 1\n");
    let fifo = temp.path().join("events.log");
    mkfifo(&fifo);
    std::os::unix::fs::symlink("events.log", temp.path().join("latest.log")).unwrap();
    let _socket = std::os::unix::net::UnixListener::bind(temp.path().join("app.sock")).unwrap();

    let out = render(temp.path(), 3000).unwrap();
    assert_eq!(out, "main.py\n  1→def main():\n  2→    return 1\n");

    for special in [&fifo, &temp.path().join("latest.log")] {
        let error = render(special, 3000).unwrap_err();
        assert!(error.to_string().contains("regular file"), "{error}");
    }
}

/// Workspace discovery opens manifests by fixed name rather than from a
/// listing. A FIFO or a link to a device wearing such a name reads as no
/// workspace declaration, not a read that blocks or never ends.
#[cfg(unix)]
#[test]
fn robustness_special_files_named_like_workspace_manifests_are_never_read() {
    use std::os::unix::fs::symlink;

    for (manifest, is_fifo) in [
        ("pnpm-workspace.yaml", true),
        ("pnpm-workspace.yaml", false),
        ("Cargo.toml", false),
    ] {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path();
        write(
            &root.join("package.json"),
            "{\"name\": \"shell\", \"workspaces\": [\"packages/*\"]}\n",
        );
        write(
            &root.join("packages/member/package.json"),
            "{\"name\": \"member\"}\n",
        );
        write(
            &root.join("crates/member/Cargo.toml"),
            "[package]\nname = \"member\"\n",
        );
        if is_fifo {
            mkfifo(&root.join(manifest));
        } else {
            symlink("/dev/zero", root.join(manifest)).unwrap();
        }
        let out = render_within(root, 3000, Duration::from_secs(20)).unwrap();
        assert!(out.contains("\"member\""), "{out}");
    }
}

/// Git's pointer files (`.git` in a linked worktree, `commondir` in its
/// git dir) are read before any listing. A FIFO or a link to a device
/// wearing either name reads as no repository, not a read that blocks or
/// never ends.
#[cfg(unix)]
#[test]
fn robustness_special_files_named_like_git_pointers_are_never_read() {
    use std::os::unix::fs::symlink;

    for case in ["fifo .git", "device .git", "fifo commondir"] {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path();
        write(&root.join("main.py"), "def main():\n    return 1\n");
        match case {
            "fifo .git" => mkfifo(&root.join(".git")),
            "device .git" => symlink("/dev/zero", root.join(".git")).unwrap(),
            _ => {
                write(&root.join(".git"), "gitdir: meta\n");
                std::fs::create_dir(root.join("meta")).unwrap();
                mkfifo(&root.join("meta/commondir"));
            }
        }
        let out = render_within(root, 3000, Duration::from_secs(20)).unwrap();
        assert!(out.contains("1→def main():"), "{case}: {out}");
    }
}

#[test]
fn robustness_binary_files_render_only_their_rows() {
    let temp = tempfile::tempdir().unwrap();
    write(&temp.path().join("README.md"), "# Demo\n\nA demo.\n");
    write(
        &temp.path().join("logo.png"),
        b"\x89PNG\r\n\x1a\n\0\0\0\rIHDR",
    );
    write(&temp.path().join("data.txt"), "alpha\0beta\n");

    let out = render(temp.path(), 3000).unwrap();
    assert!(out.contains("1→# Demo"), "{out}");
    for binary in ["logo.png", "data.txt"] {
        assert!(out.contains(&format!("{binary}\n")), "{out}");
    }
    assert!(!out.contains("alpha") && !out.contains("IHDR"), "{out}");

    assert_eq!(
        render(&temp.path().join("data.txt"), 3000).unwrap(),
        "data.txt\n"
    );
}

/// One byte that isn't UTF-8 (a Latin-1 name in a license header) must
/// not hide a source file: it reads as U+FFFD and the file renders.
#[test]
fn robustness_non_utf8_text_renders_with_replacement_characters() {
    let temp = tempfile::tempdir().unwrap();
    write(
        &temp.path().join("src/legacy.c"),
        b"/* Copyright Jos\xe9 */\nint legacy_entry(int value) { return value; }\n",
    );
    write(
        &temp.path().join("src/modern.c"),
        "/* Copyright Jos\u{e9} */\nint modern_entry(int value) { return value; }\n",
    );

    let out = render(temp.path(), 3000).unwrap();
    for shown in ["int legacy_entry(int value)", "int modern_entry(int value)"] {
        assert!(out.contains(shown), "no `{shown}` in:\n{out}");
    }

    let out = render(&temp.path().join("src/legacy.c"), 3000).unwrap();
    assert!(out.contains("Jos\u{FFFD}"), "{out}");
}

/// A file too large to parse is not parsed: in a directory only the
/// floor's head of it renders; named, it renders its head, and bytes past
/// the head that are not UTF-8 do not stop it.
#[test]
fn robustness_huge_files_render_at_most_their_head() {
    let temp = tempfile::tempdir().unwrap();
    let mut huge: Vec<u8> = (0..600_000)
        .flat_map(|index| format!("int value_{index} = {index};\n").into_bytes())
        .collect();
    let huge_path = temp.path().join("table.c");
    write(&huge_path, &huge);
    write(
        &temp.path().join("small.c"),
        "int small(void) { return 1; }\n",
    );

    let out = render(temp.path(), 50_000).unwrap();
    assert!(out.contains("small.c\n  1→int small(void)"), "{out}");
    assert!(out.contains("table.c\n  1→int value_0 = 0;\n"), "{out}");
    assert!(!out.contains("value_1000 "), "{out}");

    huge.extend_from_slice(b"\xff\xfe\n");
    write(&huge_path, huge);
    let out = render(&huge_path, 1000).unwrap();
    assert!(out.starts_with("table.c\n  1→int value_0 = 0;\n"), "{out}");
    assert!(out.ends_with("…\n"), "{out}");
}

/// A long run without a break (an embedded base64 blob of zeros) costs
/// the tokenizer time quadratic in its length, so nothing may price a
/// source line past the prefix its row renders.
#[test]
fn robustness_long_lines_render_as_a_prefix() {
    let temp = tempfile::tempdir().unwrap();
    let blob = "A".repeat(200_000);
    write(
        &temp.path().join("data.py"),
        format!("DATA = \"{blob}\"\n\n\ndef load():\n    return DATA\n"),
    );
    write(
        &temp.path().join("README.md"),
        format!("# Demo\n\n![logo](data:image/png;base64,{blob})\n"),
    );
    write(
        &temp.path().join("package.json"),
        format!(
            "{{\n  \"name\": \"demo\",\n  \"scripts\": {{\n    \"build\": \"{blob}\"\n  }}\n}}\n"
        ),
    );

    let out = render_within(temp.path(), 3000, Duration::from_secs(10)).unwrap();
    assert!(out.contains("def load():"), "{out}");
    for prefix in ["1→DATA = \"AAA", "3→![logo]"] {
        let row = out
            .lines()
            .find(|row| row.contains(prefix))
            .unwrap_or_else(|| panic!("no `{prefix}` row in:\n{out}"));
        assert!(row.ends_with('…') && row.len() < blob.len() / 10, "{row}");
    }
}

#[test]
fn robustness_crlf_files_render_without_carriage_returns() {
    let temp = tempfile::tempdir().unwrap();
    write(
        &temp.path().join("README.md"),
        "# Demo\r\n\r\nA demo project.\r\n\r\n## Usage\r\n\r\nRun it.\r\n",
    );
    write(
        &temp.path().join("app.py"),
        "def main():\r\n    return 1\r\n\r\n\r\nclass Runner:\r\n    pass\r\n",
    );
    write(
        &temp.path().join("Cargo.toml"),
        "[package]\r\nname = \"demo\"\r\nversion = \"0.1.0\"\r\n",
    );

    let out = render(temp.path(), 3000).unwrap();
    for row in [
        "1→# Demo\n",
        "7→Run it.\n",
        "1→def main():\n",
        "5→class Runner:\n",
        "2→name = \"demo\"\n",
    ] {
        assert!(out.contains(row), "no `{row}` in:\n{out}");
    }
    assert!(!out.contains('\r'), "{out:?}");
}

#[test]
fn robustness_deep_directory_chain_renders_as_one_row() {
    let temp = tempfile::tempdir().unwrap();
    let chain = "d/".repeat(200);
    write(
        &temp.path().join(&chain).join("leaf.py"),
        "def leaf():\n    return 1\n",
    );

    let out = render(temp.path(), 3000).unwrap();
    assert_eq!(
        out,
        format!("{chain}\n  leaf.py\n    1→def leaf():\n    2→    return 1\n")
    );
}

#[test]
fn robustness_unicode_and_control_character_names_render_on_their_rows() {
    let temp = tempfile::tempdir().unwrap();
    write(&temp.path().join("日本語.md"), "# 見出し\n");
    write(&temp.path().join("café.py"), "def f():\n    pass\n");
    write(&temp.path().join("🚀 rocket/notes.txt"), "launch\n");
    write(&temp.path().join("new\nline.txt"), "z\n");
    write(&temp.path().join("tab\tname.txt"), "w\n");

    let out = render(temp.path(), 3000).unwrap();
    for row in [
        "café.py\n",
        "日本語.md\n",
        "🚀 rocket/\n",
        "new\\nline.txt\n",
        "tab\\tname.txt\n",
    ] {
        assert!(out.contains(row), "no `{row}` in:\n{out}");
    }
    assert!(!out.lines().any(|row| row == "line.txt"), "{out}");
}

/// A credential file's content never renders, whichever walker would read
/// it: one named as a credential file, a link to one, or anything holding
/// a private key. Dotenv samples and code about credentials still render.
#[cfg(unix)]
#[test]
fn robustness_credential_files_render_only_their_rows() {
    const SECRET: &str = "precis-credential-marker-51c7";
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path();
    let key = format!(
        "-----BEGIN RSA PRIVATE KEY-----\n{SECRET}{}\n-----END RSA PRIVATE KEY-----\n",
        "A".repeat(64)
    );
    for (path, text) in [
        ("README.md", "# demo\n".to_string()),
        (
            "credentials.json",
            format!("{{\"private_key\": \"{SECRET}\"}}\n"),
        ),
        ("secrets.json", format!("{{\"token\": \"{SECRET}\"}}\n")),
        (
            "service-account.json",
            format!("{{\"key\": \"{SECRET}\"}}\n"),
        ),
        ("production.env", format!("API_KEY={SECRET}\n")),
        ("docker.env", format!("API_KEY={SECRET}\n")),
        (".env", format!("API_KEY={SECRET}\n")),
        (".env.local.sh", format!("export API_KEY={SECRET}\n")),
        (
            "scripts/secrets.prod.sh",
            format!("export TOKEN={SECRET}\n"),
        ),
        (
            "scripts/secrets",
            format!("#!/bin/sh\nexport TOKEN={SECRET}\n"),
        ),
        (
            "config/credentials-dev.ini",
            format!("[auth]\npassword={SECRET}\n"),
        ),
        ("config/creds_staging.conf", format!("password={SECRET}\n")),
        ("infra/prod.tfvars", format!("db_password = \"{SECRET}\"\n")),
        (
            "schema.prisma",
            format!(
                "datasource db {{\n  provider = \"postgresql\"\n  \
                 url      = \"postgresql://admin:{SECRET}@db/app\"\n}}\n\n\
                 model User {{\n  id Int @id\n}}\n"
            ),
        ),
        ("notes.txt", key.clone()),
        ("src/keys.rs", format!("pub const KEY: &str = \"{key}\";\n")),
        (".env.example", "API_KEY=changeme\n".to_string()),
        (
            "src/credentials.py",
            "def load_credentials():\n    return {}\n".to_string(),
        ),
    ] {
        write(&root.join(path), text);
    }
    std::os::unix::fs::symlink(".env", root.join("deploy.conf")).unwrap();

    for budget in [3000, 100_000] {
        let out = render(root, budget).unwrap();
        assert!(!out.contains(SECRET), "{out}");
        for shown in [
            "API_KEY=changeme",
            "def load_credentials():",
            "credentials.json",
        ] {
            assert!(out.contains(shown), "no `{shown}` in:\n{out}");
        }
    }
}

/// Distinctive enough that finding it anywhere in the output is proof,
/// not coincidence.
#[cfg(unix)]
const OUT_OF_ROOT_MARKER: &str = "precis-out-of-root-marker-9d3f";

/// A checkout carrying every shape of link at once: escaping, contained,
/// dangling, and cyclic. Returns the walk root.
#[cfg(unix)]
fn linked_checkout(base: &Path) -> PathBuf {
    use std::os::unix::fs::symlink;

    let outside = base.join("outside");
    let root = base.join("repo");
    write(
        &outside.join("secrets/credentials.ini"),
        format!("api_token = {OUT_OF_ROOT_MARKER}\npassword = hunter2\n"),
    );
    write(
        &outside.join("notes.md"),
        format!("# Notes\n\nAnother {OUT_OF_ROOT_MARKER} sitting outside the walk.\n"),
    );
    write(&root.join("README.md"), "# demo\n\nA demo repository.\n");
    write(&root.join("src/main.py"), "def main():\n    return 1\n");
    write(&root.join("docs/guide.md"), "# Guide\n\nHow to use it.\n");

    // Out of root: a file link, a directory link, and a relative one.
    symlink(
        outside.join("secrets/credentials.ini"),
        root.join("config.ini"),
    )
    .unwrap();
    symlink(&outside, root.join("vendor")).unwrap();
    symlink("../outside/notes.md", root.join("docs/notes.md")).unwrap();
    // Dangling: unresolvable, so containment can't be established.
    symlink("nowhere.md", root.join("docs/broken.md")).unwrap();

    // In root: the idiom real repositories use.
    symlink("README.md", root.join("CLAUDE.md")).unwrap();
    symlink("../src", root.join("docs/source")).unwrap();

    // Cycles: self, parent, and a mutual pair.
    symlink(".", root.join("selfloop")).unwrap();
    symlink("..", root.join("src/up")).unwrap();
    symlink("../docs", root.join("src/to_docs")).unwrap();
    symlink("../src", root.join("docs/to_src")).unwrap();

    root
}

#[cfg(unix)]
#[test]
fn robustness_symlinks_never_render_content_from_outside_the_walk_root() {
    let temp = tempfile::tempdir().unwrap();
    let root = linked_checkout(temp.path());

    // Far above what this tree needs: at a tight budget an absent leak
    // proves only that the budget ran out.
    let out = render(&root, 100_000).unwrap();

    assert!(!out.contains(OUT_OF_ROOT_MARKER), "{out}");
    for escaping in ["config.ini", "vendor", "notes.md", "broken.md"] {
        assert!(
            !out.contains(escaping),
            "`{escaping}` resolves outside the walk root (or nowhere) and must not be listed:\n{out}"
        );
    }
    // Links that stay inside the root keep their rows, and a linked
    // directory is still a directory.
    for contained in ["README.md", "CLAUDE.md", "docs/", "source/", "selfloop/"] {
        assert!(out.contains(contained), "`{contained}` missing:\n{out}");
    }

    let out = render(&root, 3000).unwrap();
    assert!(!out.contains(OUT_OF_ROOT_MARKER), "{out}");
}

/// A named file gets the rule a listing applies to a linked entry: a
/// link resolving outside its own directory is refused, one staying
/// inside renders what it points at.
#[cfg(unix)]
#[test]
fn robustness_symlinks_named_directly_stay_contained() {
    let temp = tempfile::tempdir().unwrap();
    let root = linked_checkout(temp.path());

    for escaping in ["config.ini", "docs/notes.md", "docs/broken.md"] {
        assert!(
            render(&root.join(escaping), 100_000).is_err(),
            "`{escaping}` resolves outside its directory (or nowhere) and must not render"
        );
    }

    let out = render(&root.join("CLAUDE.md"), 100_000).unwrap();
    assert!(out.starts_with("README.md\n"), "{out}");
    assert!(out.contains("# demo"), "{out}");
}

/// A link that stays inside the root but lands on something the walk
/// hides is an alias for nothing it may show: neither the link nor what
/// it points at renders, whatever name the link wears.
#[cfg(unix)]
#[test]
fn robustness_links_to_hidden_entries_render_nothing_of_them() {
    use std::os::unix::fs::symlink;

    let temp = tempfile::tempdir().unwrap();
    let root = temp.path();
    write(
        &root.join(".git/config"),
        format!("[user]\n{OUT_OF_ROOT_MARKER}\n"),
    );
    write(&root.join(".gitignore"), "notes.txt\nprivate/\n");
    write(
        &root.join("notes.txt"),
        format!("token = {OUT_OF_ROOT_MARKER}\n"),
    );
    write(
        &root.join("private/plan.md"),
        format!("# Plan\n\n{OUT_OF_ROOT_MARKER}\n"),
    );
    write(&root.join("main.py"), "def main():\n    return 1\n");
    symlink("notes.txt", root.join("README.md")).unwrap();
    symlink("private/plan.md", root.join("GUIDE.md")).unwrap();
    symlink("private", root.join("docs")).unwrap();
    symlink(".git/config", root.join("setup.cfg")).unwrap();

    let out = render(root, 100_000).unwrap();
    assert!(!out.contains(OUT_OF_ROOT_MARKER), "{out}");
    assert!(out.contains("1→def main():"), "{out}");
}

/// Every consumer reopens an entry by the name its listing gives, so a
/// name that isn't UTF-8 must not list under a lossy spelling that
/// another entry — here a link out of the root — really has. Filesystems
/// that refuse such names (APFS) have nothing to test.
#[cfg(unix)]
#[test]
fn robustness_non_utf8_names_never_alias_another_entry() {
    use std::os::unix::ffi::OsStrExt;
    use std::os::unix::fs::symlink;

    let temp = tempfile::tempdir().unwrap();
    let outside = temp.path().join("outside.json");
    let root = temp.path().join("repo");
    write(
        &outside,
        format!("{{\"token\": \"{OUT_OF_ROOT_MARKER}\"}}\n"),
    );
    write(&root.join("README.md"), "# demo\n");
    let raw_name = std::ffi::OsStr::from_bytes(b"config-\xff.json");
    if std::fs::write(root.join(raw_name), "{}\n").is_err() {
        return;
    }
    symlink(&outside, root.join("config-\u{FFFD}.json")).unwrap();

    let out = render(&root, 100_000).unwrap();
    assert!(!out.contains(OUT_OF_ROOT_MARKER), "{out}");
}

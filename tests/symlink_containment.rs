//! precis summarizes *a path*. Nothing outside that path may reach the
//! output, however the checkout is shaped — and the walk must terminate
//! no matter what links it is pointed at. Both are properties of the
//! listing layer (`fs_util::list_dir`), so this exercises them
//! end-to-end through the real renderer rather than unit-testing the
//! rule that happens to implement them.

#![cfg(unix)]

use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::time::Duration;

/// Distinctive enough that finding it anywhere in the output is proof,
/// not coincidence.
const OUT_OF_ROOT_MARKER: &str = "precis-out-of-root-marker-9d3f";

/// A checkout carrying every shape of link at once: escaping, contained,
/// dangling, and cyclic. Returns the walk root.
fn linked_checkout(base: &Path) -> PathBuf {
    let outside = base.join("outside");
    let root = base.join("repo");
    std::fs::create_dir_all(outside.join("secrets")).unwrap();
    std::fs::create_dir_all(root.join("src")).unwrap();
    std::fs::create_dir(root.join("docs")).unwrap();

    std::fs::write(
        outside.join("secrets/credentials.ini"),
        format!("api_token = {OUT_OF_ROOT_MARKER}\npassword = hunter2\n"),
    )
    .unwrap();
    std::fs::write(
        outside.join("notes.md"),
        format!("# Notes\n\nAnother {OUT_OF_ROOT_MARKER} sitting outside the walk.\n"),
    )
    .unwrap();
    std::fs::write(root.join("README.md"), "# demo\n\nA demo repository.\n").unwrap();
    std::fs::write(root.join("src/main.py"), "def main():\n    return 1\n").unwrap();
    std::fs::write(root.join("docs/guide.md"), "# Guide\n\nHow to use it.\n").unwrap();

    // (a) Out of root: a plain file link, a whole-directory link, and a
    // relative one — the shapes a hostile or careless checkout ships.
    symlink(
        outside.join("secrets/credentials.ini"),
        root.join("config.ini"),
    )
    .unwrap();
    symlink(&outside, root.join("vendor")).unwrap();
    symlink("../outside/notes.md", root.join("docs/notes.md")).unwrap();
    // Dangling: unresolvable, so containment can't be established.
    symlink("nowhere.md", root.join("docs/broken.md")).unwrap();

    // (b) In root: the idiom real repositories use.
    symlink("README.md", root.join("CLAUDE.md")).unwrap();
    symlink("../src", root.join("docs/source")).unwrap();

    // (c) Cycles: self, parent, and a mutual pair — none of which any
    // depth or visited-set bookkeeping should be needed to survive.
    symlink(".", root.join("selfloop")).unwrap();
    symlink("..", root.join("src/up")).unwrap();
    symlink("../docs", root.join("src/to_docs")).unwrap();
    symlink("../src", root.join("docs/to_src")).unwrap();

    root
}

/// Render on a worker so a traversal that never terminates fails as an
/// assertion instead of wedging the whole suite.
fn render_within(root: &Path, budget: usize, limit: Duration) -> String {
    let (tx, rx) = mpsc::channel();
    let root = root.to_path_buf();
    std::thread::spawn(move || {
        let _ = tx.send(precis::render(&[root], budget, None));
    });
    rx.recv_timeout(limit)
        .expect("the walk must terminate on a checkout full of link cycles")
        .expect("render failed")
}

#[test]
fn symlink_containment_never_renders_content_from_outside_the_walk_root() {
    let temp = tempfile::tempdir().unwrap();
    let root = linked_checkout(temp.path());

    // Deliberately far above what this tree needs: at a tight budget an
    // absent leak proves only that the budget ran out.
    let out = render_within(&root, 100_000, Duration::from_secs(60));

    assert!(
        !out.contains(OUT_OF_ROOT_MARKER),
        "out-of-root content reached the output:\n{out}"
    );
    for escaping in ["config.ini", "vendor", "notes.md", "broken.md"] {
        assert!(
            !out.contains(escaping),
            "`{escaping}` resolves outside the walk root (or nowhere at all) and must not be \
             listed:\n{out}"
        );
    }

    // Containment is not an excuse to drop real repository structure:
    // links that stay inside the root keep their rows, and a linked
    // directory is still reported as a directory.
    for contained in ["README.md", "CLAUDE.md", "docs/", "source/", "selfloop/"] {
        assert!(
            out.contains(contained),
            "`{contained}` is in-root structure and must still be listed:\n{out}"
        );
    }
}

/// The same tree with no room to spare: the budget picks winners
/// differently, so re-run the containment assertion at the budget every
/// auto-injection actually hits.
#[test]
fn symlink_containment_holds_at_the_headline_budget() {
    let temp = tempfile::tempdir().unwrap();
    let root = linked_checkout(temp.path());

    let out = render_within(&root, 3000, Duration::from_secs(60));

    assert!(
        !out.contains(OUT_OF_ROOT_MARKER),
        "out-of-root content reached the output:\n{out}"
    );
}

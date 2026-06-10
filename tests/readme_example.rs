//! The README's fenced example block must be the verbatim output of
//! `precis` on the named fixture at the named budget — the README
//! showcases real output, and "verbatim subset of the source" is the
//! tool's core promise.

use std::path::{Path, PathBuf};

const README_EXAMPLE_FIXTURE: &str = "mitt";
const README_EXAMPLE_BUDGET: usize = 900;

#[test]
fn readme_example_matches_output() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let fixture_root: PathBuf = manifest_dir
        .join("tests/fixtures")
        .join(README_EXAMPLE_FIXTURE);
    assert!(
        fixture_root.exists(),
        "fixture `{README_EXAMPLE_FIXTURE}` not present at {}; run `cargo run --bin clone_fixtures`",
        fixture_root.display()
    );
    let output = precis::render(&[&fixture_root], README_EXAMPLE_BUDGET, None)
        .expect("render README example fixture");

    let readme =
        std::fs::read_to_string(manifest_dir.join("README.md")).expect("failed to read README.md");
    let start_marker = "<!-- precis-example-start -->";
    let end_marker = "<!-- precis-example-end -->";
    let start = readme
        .find(start_marker)
        .expect("README.md missing start marker")
        + start_marker.len();
    let end = readme[start..]
        .find(end_marker)
        .expect("README.md missing end marker")
        + start;
    let block = readme[start..end].trim();
    let lines: Vec<&str> = block.lines().collect();
    assert!(
        lines.len() >= 3,
        "README example block too short ({} lines) — expected a fenced code block",
        lines.len(),
    );
    // Strip the ``` fence lines.
    let from_readme = lines[1..lines.len() - 1].join("\n");
    assert_eq!(
        output.trim_end(),
        from_readme.trim_end(),
        "precis output for {README_EXAMPLE_FIXTURE}@{README_EXAMPLE_BUDGET} doesn't match the README example — regenerate the block with `cargo run --release -- tests/fixtures/{README_EXAMPLE_FIXTURE} --token-budget {README_EXAMPLE_BUDGET}`",
    );
}

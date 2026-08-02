//! Module-scope script statements are peers of the declaration and
//! export batches around them, so their line ownership has to be
//! disjoint from all of them. The scheduler asserts that in its hot
//! path — it panics in debug and silently drops the loser in release,
//! which is how these two shapes were found. Both are exercised
//! end-to-end through the real renderer so the assertion actually runs.

use std::path::Path;

/// A statement sharing a physical line with a declaration: the
/// declaration's `ModuleItem` and the call's `ModuleStatements` batch
/// would both claim row 1.
const SHARED_LINE_SCRIPT: &str = "\
const state = {}; run(state);

function run(target) {
  target.started = true;
}
";

/// Enough private declarations to build a `ModuleItemNames` gate, plus an
/// export whose courtesy ellipsis lands on the statement's row.
const ELLIPSIS_COLLISION_SCRIPT: &str = "\
const alpha = 1;
const beta = 2;
const gamma = 3;
const delta = 4;
const epsilon = 5;
const zeta = 6;

export const total = alpha + beta;
boot(total);

function boot(value) {
  return value;
}
";

/// A script that publishes no surface of any kind — no declarations, no
/// imports, no exports, no module doc. There is nothing for its
/// statements to gate on.
const SURFACELESS_SCRIPT: &str = "\
boot();
start();
";

fn render_project(dir: &Path, entry: &str) -> String {
    std::fs::create_dir_all(dir).unwrap();
    std::fs::write(dir.join("README.md"), "# demo\n\nA demo package.\n").unwrap();
    std::fs::write(
        dir.join("package.json"),
        "{\n  \"name\": \"demo\",\n  \"version\": \"1.0.0\",\n  \"main\": \"index.js\"\n}\n",
    )
    .unwrap();
    std::fs::write(dir.join("index.js"), entry).unwrap();
    // A generous budget so nothing is dropped for cost reasons — the
    // point is that every emitted batch is schedulable at all.
    precis::render(&[dir], 20_000, None).unwrap()
}

#[test]
fn js_statement_ownership_statement_sharing_a_declaration_line_is_schedulable() {
    let tmp = tempfile::tempdir().unwrap();
    let out = render_project(&tmp.path().join("repo"), SHARED_LINE_SCRIPT);
    // The declaration's batch renders the row, so the text is present
    // exactly once and no peer batch fights it for ownership.
    assert!(
        out.contains("const state = {}; run(state);"),
        "shared-line statement row missing from output:\n{out}"
    );
    assert_eq!(
        out.matches("const state = {}; run(state);").count(),
        1,
        "shared-line row rendered more than once:\n{out}"
    );
}

#[test]
fn js_statement_ownership_surfaceless_script_emits_no_statement_batches() {
    let tmp = tempfile::tempdir().unwrap();
    let out = render_project(&tmp.path().join("repo"), SURFACELESS_SCRIPT);
    // Statements gate on the file's first admitted surface. With no
    // surface to buy first, admitting them would put root-level crumbs
    // ahead of every gated batch in the walk.
    assert!(
        !out.contains("boot();"),
        "a script with no admitted surface must not emit ungated statement \
         batches:\n{out}"
    );
}

#[test]
fn js_statement_ownership_export_ellipsis_never_lands_on_a_statement_row() {
    let tmp = tempfile::tempdir().unwrap();
    let out = render_project(&tmp.path().join("repo"), ELLIPSIS_COLLISION_SCRIPT);
    // In release the collision silently dropped the statement; the
    // executable's flow has to survive.
    assert!(
        out.contains("boot(total);"),
        "script-flow statement dropped from output:\n{out}"
    );
    assert!(
        out.contains("export const total"),
        "export surface missing from output:\n{out}"
    );
}

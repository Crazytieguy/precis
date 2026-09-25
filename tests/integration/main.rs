//! All integration tests, in one binary so a `src/` change relinks once.

mod fixture_baselines;
mod js_statement_ownership;
mod literal_roster_ownership;
mod ns_simulate;
mod readme_example;
mod scheduler_invariants;
mod symlink_containment;

use std::path::Path;

/// Renders a one-file JS package whose entry point is `entry`, at a budget
/// generous enough that nothing is dropped for cost: the point is that
/// every emitted batch is schedulable at all.
fn render_js_project(dir: &Path, entry: &str) -> String {
    std::fs::create_dir_all(dir).unwrap();
    std::fs::write(dir.join("README.md"), "# demo\n\nA demo package.\n").unwrap();
    std::fs::write(
        dir.join("package.json"),
        "{\n  \"name\": \"demo\",\n  \"version\": \"1.0.0\",\n  \"main\": \"index.js\"\n}\n",
    )
    .unwrap();
    std::fs::write(dir.join("index.js"), entry).unwrap();
    precis::render(dir, 20_000, None).unwrap()
}

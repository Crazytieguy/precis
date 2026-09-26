//! `ns_simulate::simulate_ns` on synthetic North Stars over the `log`
//! fixture: each authoring rule fires on its violation and only there.

use std::path::Path;

use precis::north_star::NorthStar;
use precis::ns_simulate::simulate_ns;

const LOG_FIXTURE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/log");

/// A `[[batches]]` table with the given `[batches.content]` body.
fn batch(id: &str, content: &str) -> String {
    format!(
        "[[batches]]\nid = \"{id}\"\ndescriptor = \"d\"\njustification = \"j\"\n\
         [batches.content]\n{content}\n"
    )
}

/// A lines batch over `src/lib.rs` rendering `start..=end` with `render`.
fn lines(id: &str, start: usize, end: usize, render: &str) -> String {
    batch(
        id,
        &format!(
            "kind = \"lines\"\nspans = [{{ path = \"src/lib.rs\", start = {start}, end = {end}, render = {render} }}]"
        ),
    )
}

/// `batch` with a `predecessor` edge to `predecessor`.
fn with_predecessor(batch: String, predecessor: &str) -> String {
    batch.replacen(
        "descriptor",
        &format!("predecessor = \"{predecessor}\"\ndescriptor"),
        1,
    )
}

const FULL: &str = "{ kind = \"full\" }";
const ELLIPSIS: &str = "{ kind = \"ellipsis\" }";

/// Each batch's violation messages.
fn violations(batches: &[String]) -> Vec<Vec<String>> {
    violations_in(Path::new(LOG_FIXTURE), batches)
}

/// [`violations`] over the fixture at `root`.
fn violations_in(root: &Path, batches: &[String]) -> Vec<Vec<String>> {
    let toml = format!(
        "fixture = \"log\"\nrevision_pin = \"unchecked\"\n\n{}",
        batches.concat()
    );
    let ns: NorthStar = toml::from_str(&toml).expect("valid ns toml");
    simulate_ns(&ns, root)
        .into_iter()
        .map(|b| b.violations)
        .collect()
}

#[track_caller]
fn assert_flags(violations: &[String], needle: &str) {
    assert!(
        violations.iter().any(|v| v.contains(needle)),
        "expected `{needle}`, got {violations:?}"
    );
}

#[test]
fn ns_simulate_detects_growth_envelope_violation() {
    let found = violations(&[lines("1", 1, 3, FULL), lines("2", 10, 400, FULL)]);
    assert!(found[0].is_empty(), "{:?}", found[0]);
    assert_flags(&found[1], "growth envelope");
}

/// The envelope binds even with nothing before it (`100 + 0.3·0`).
#[test]
fn ns_simulate_first_batch_obeys_envelope() {
    assert_flags(
        &violations(&[lines("1", 1, 400, FULL)])[0],
        "growth envelope",
    );
}

/// The clash is left out, so it neither claims lines nor stands in for
/// the first occurrence as a successor's predecessor.
#[test]
fn ns_simulate_detects_duplicate_batch_id() {
    let found = violations(&[
        lines("1.1", 1, 3, FULL),
        lines("1.1", 1, 3, FULL),
        with_predecessor(lines("1.2", 3, 3, FULL), "1.1"),
    ]);
    assert!(found[0].is_empty(), "{:?}", found[0]);
    assert_eq!(found[1].len(), 1, "{:?}", found[1]);
    assert_flags(&found[1], "duplicate batch id \"1.1\"");
    assert!(found[2].is_empty(), "{:?}", found[2]);
}

#[test]
fn ns_simulate_detects_overlapping_spans_within_batch() {
    let spans = "kind = \"lines\"\nspans = [\
        { path = \"src/lib.rs\", start = 1, end = 5, render = { kind = \"full\" } },\
        { path = \"src/lib.rs\", start = 5, end = 10, render = { kind = \"full\" } }]";
    assert_flags(
        &violations(&[batch("1", spans)])[0],
        "overlapping spans within one batch at src/lib.rs:5",
    );
}

#[test]
fn ns_simulate_detects_overlapping_fs_entries_across_batches() {
    let found = violations(&[
        batch(
            "1",
            "kind = \"fs\"\ngroups = [{ parent = \".\", entries = [\"src\"] }]",
        ),
        batch(
            "2",
            "kind = \"fs\"\ngroups = [{ parent = \".\", entries = [\"src\", \"Cargo.toml\"] }]",
        ),
    ]);
    assert!(found[0].is_empty(), "{:?}", found[0]);
    assert_flags(&found[1], ". lists \"src\", already owned by 1");
}

/// The renderer and the grader read an entry by its last component, so a
/// nested path lists that name under the group's parent.
#[test]
fn ns_simulate_detects_fs_entry_paths() {
    let found = violations(&[batch(
        "1",
        "kind = \"fs\"\ngroups = [{ parent = \".\", entries = [\"nope/src\"] }]",
    )]);
    assert_flags(&found[0], "is a path, not a name");
}

#[test]
fn ns_simulate_detects_multi_line_ellipsis() {
    assert_flags(
        &violations(&[lines("1", 5, 8, ELLIPSIS)])[0],
        "multi-line Ellipsis span at src/lib.rs:5..=8",
    );
}

#[test]
fn ns_simulate_accepts_single_line_ellipsis() {
    let found = violations(&[lines("1", 5, 5, ELLIPSIS)]);
    assert!(found[0].is_empty(), "{:?}", found[0]);
}

/// Past EOF is a violation, not a render panic.
#[test]
fn ns_simulate_detects_span_out_of_range() {
    assert_flags(
        &violations(&[lines("1", 1, 99999, FULL)])[0],
        "span out of range",
    );
}

#[test]
fn ns_simulate_detects_empty_lines_batch() {
    let found = violations(&[batch("1", "kind = \"lines\"\nspans = []")]);
    assert_flags(&found[0], "zero atoms");
}

#[test]
fn ns_simulate_detects_empty_fs_batch() {
    let found = violations(&[batch(
        "1",
        "kind = \"fs\"\ngroups = [{ parent = \".\", entries = [] }]",
    )]);
    assert_flags(&found[0], "zero atoms");
}

#[test]
fn ns_simulate_detects_span_paths_escaping_the_root() {
    for path in ["/etc/passwd", "../Cargo.toml"] {
        let content = format!(
            "kind = \"lines\"\nspans = [{{ path = \"{path}\", start = 1, end = 1, render = {FULL} }}]"
        );
        assert_flags(
            &violations(&[batch("1", &content)])[0],
            "escapes the fixture root",
        );
    }
}

/// src/lib.rs:827 is `///        println!("{}:{} -- {}",`: truncating after
/// `println` saves tokens but elides no word character.
#[test]
fn ns_simulate_detects_truncation_eliding_only_punctuation() {
    let truncated = "{ kind = \"truncated\", pattern = \"^.*println\" }";
    assert_flags(
        &violations(&[lines("1", 827, 827, truncated)])[0],
        "never elides a word character",
    );
}

/// A batch left out of the simulation for a bad span yields one violation
/// on its successor, not a phantom "not found" plus an overlap with the
/// lines it would have inherited.
#[test]
fn ns_simulate_left_out_predecessor_does_not_cascade() {
    let found = violations(&[
        lines("1", 1, 5, FULL),
        with_predecessor(lines("2", 1, 99999, FULL), "1"),
        with_predecessor(lines("3", 1, 1, ELLIPSIS), "2"),
    ]);
    assert_flags(&found[1], "span out of range");
    assert_eq!(
        found[2],
        vec![
            "predecessor \"2\" was left out of the simulation (fix its violations first)"
                .to_string()
        ]
    );
}

#[test]
fn ns_simulate_reports_the_cap_once() {
    let found = violations(&[
        lines("1", 1, 700, FULL),
        lines("2", 701, 1400, FULL),
        lines("3", 1401, 2010, FULL),
    ]);
    let capped = found
        .iter()
        .filter(|batch| batch.iter().any(|v| v.contains("cap exceeded")))
        .count();
    assert_eq!(capped, 1, "{found:?}");
}

/// A span is held to the listing the walker reads through, so content no
/// walker can show (the pin file, a directory) fails validation rather
/// than validating clean and scoring zero forever.
#[test]
fn ns_simulate_detects_spans_on_files_precis_never_lists() {
    for path in [precis::fs_util::PRECIS_PIN_FILE, "src"] {
        let content = format!(
            "kind = \"lines\"\nspans = [{{ path = \"{path}\", start = 1, end = 1, render = {FULL} }}]"
        );
        assert_flags(
            &violations(&[batch("1", &content)])[0],
            "is not a file precis lists",
        );
    }
}

/// The renderer shows a `Full` line's first 500 characters, so a
/// truncation that keeps all of them saves nothing however long the line.
#[test]
fn ns_simulate_prices_truncation_against_the_visible_line() {
    let fixture = tempfile::tempdir().unwrap();
    std::fs::write(fixture.path().join("long.txt"), "word ".repeat(2000)).unwrap();
    let content = "kind = \"lines\"\nspans = [{ path = \"long.txt\", start = 1, end = 1, \
        render = { kind = \"truncated\", pattern = \"^.{500}\" } }]";
    assert_flags(
        &violations_in(fixture.path(), &[batch("1", content)])[0],
        "saves no tokens",
    );
}

/// The rules without a dedicated test above: each case's last batch
/// carries exactly the one violation named, and every earlier batch none.
#[test]
fn ns_simulate_flags_each_remaining_rule_on_its_batch_alone() {
    let truncated = |pattern: &str| format!("{{ kind = \"truncated\", pattern = \"{pattern}\" }}");
    let cases = [
        (
            vec![lines("1", 1, 3, FULL), lines("2", 3, 5, FULL)],
            "non-ancestor overlap: src/lib.rs:3 already owned by 1",
        ),
        (
            vec![with_predecessor(lines("1", 1, 3, FULL), "9")],
            "predecessor \"9\" not found in prior batches",
        ),
        (
            vec![lines("1", 1, 1, &truncated("("))],
            "invalid Truncated regex",
        ),
        (vec![lines("1", 1, 1, &truncated("^.*"))], "saves no tokens"),
        (
            vec![lines("1", 1, 1, &truncated("no such text"))],
            "produced no/empty match",
        ),
        (vec![lines("1", 5, 3, FULL)], "span range inverted"),
        (vec![lines("1", 0, 3, FULL)], "span range inverted"),
        (
            vec![batch(
                "1",
                &format!(
                    "kind = \"lines\"\nspans = [{{ path = \"src/nope.rs\", start = 1, end = 1, render = {FULL} }}]"
                ),
            )],
            "span file missing: src/nope.rs",
        ),
        (
            vec![batch(
                "1",
                "kind = \"fs\"\ngroups = [{ parent = \"nope\", entries = \"all\" }]",
            )],
            "fs content resolution failed",
        ),
    ];
    for (batches, needle) in cases {
        let found = violations(&batches);
        let (last, earlier) = found.split_last().unwrap();
        assert!(earlier.iter().all(Vec::is_empty), "{needle}: {found:?}");
        assert_eq!(last.len(), 1, "{needle}: {last:?}");
        assert_flags(last, needle);
    }
}

/// A predecessor edge licenses the overlap the rule above forbids.
#[test]
fn ns_simulate_accepts_overlap_along_a_predecessor_edge() {
    let found = violations(&[
        lines("1", 1, 3, FULL),
        with_predecessor(lines("2", 3, 5, FULL), "1"),
        with_predecessor(lines("3", 1, 1, ELLIPSIS), "2"),
    ]);
    assert!(found.iter().all(Vec::is_empty), "{found:?}");
}

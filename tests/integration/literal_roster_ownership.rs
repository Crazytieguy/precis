//! A data literal's key roster renders rows that also belong to the
//! literal's own body slice, so the two must sit on one ancestor chain.
//! When an exported JS class is split into per-member branches, each
//! member owns its body — a roster hung off the class export instead
//! would be a *sibling* of the owning member's body batch, and the
//! scheduler's ownership assertion catches that: it panics in debug and
//! silently drops the loser in release. Exercised end-to-end through the
//! real renderer so the assertion actually runs.

/// Twelve members puts the class in the per-member split range, and the
/// constructor's option table qualifies for a key roster. Pre-fix the
/// roster chained to the class `Export` while `handlers`' rows also
/// belonged to the constructor's `ExportBody` under `ExportMember`.
const SPLIT_CLASS_WITH_MEMBER_TABLE: &str = "\
export class Registry {
  constructor() {
    this.handlers = [
      {
        name: 'socketPath',
        alias: 's'
      },
      {
        name: 'host',
        alias: 'H'
      },
      {
        name: 'port',
        alias: 'P'
      },
      {
        name: 'protocol',
        alias: 'T'
      },
      {
        name: 'theme',
        alias: 't'
      }
    ];
  }

  start() {
    return this.handlers.length;
  }

  stop() {
    return 0;
  }

  reset() {
    this.handlers = [];
  }

  count() {
    return this.handlers.length;
  }

  first() {
    return this.handlers[0];
  }

  last() {
    return this.handlers[this.handlers.length - 1];
  }

  names() {
    return this.handlers.map((h) => h.name);
  }

  aliases() {
    return this.handlers.map((h) => h.alias);
  }

  has(name) {
    return this.names().includes(name);
  }

  find(name) {
    return this.handlers.find((h) => h.name === name);
  }

  clear() {
    this.handlers.length = 0;
  }
}
";

#[test]
fn literal_roster_ownership_split_class_member_table_is_schedulable() {
    let tmp = tempfile::tempdir().unwrap();
    let out = crate::render_js_project(&tmp.path().join("repo"), SPLIT_CLASS_WITH_MEMBER_TABLE);
    // Rendering at all is the assertion: in debug the sibling overlap
    // panicked inside the scheduler before this call could return.
    assert!(
        out.contains("class Registry"),
        "class surface missing from output:\n{out}"
    );
    // Whichever batch owns them, the table's rows arrive exactly once —
    // in release the overlap was resolved by dropping a batch, which
    // showed up as a duplicated or missing row.
    for row in ["name: 'socketPath'", "name: 'theme'"] {
        assert_eq!(
            out.matches(row).count(),
            1,
            "table row `{row}` should render exactly once:\n{out}"
        );
    }
}

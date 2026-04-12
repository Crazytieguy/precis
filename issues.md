# Post-rewrite issues

Outstanding issues from the architecture rewrite that need addressing
before calibration.

## Must fix (design violations)

### Missing invariant tests (plan Step 10)

The plan called for `tests/invariants.rs` with property-based tests:
- R1 pointer-range: every Complete/Truncated content slice lies inside
  the source string's byte range
- Line-number monotonicity: line numbers strictly increasing per file
- D4/D5 overlap checks
- R4 override assertion survival (no panics)

Currently only debug_asserts exist. The dedicated test file was not
created.


## Should fix (quality/performance)

### Remove semantically dishonest `name` field and `name.rs`

`ExtractedItem.name` and `src/parse/name.rs` compute a "name" for
every item kind, but most items don't have a meaningful name (e.g.
impl blocks, imports, module docs). The field is only genuinely used
by Name-level groups for `find_name_prefix`. Everything else that
touches `name` is either working around its existence or using it
as a proxy for something else (like `mark_reexports` checking
`name.starts_with("self::")`).

Fix: remove `name.rs` and the `name: String` field from
`ExtractedItem`. For logic that genuinely needs a name (Name-level
groups), compute it on demand from the node and source. For logic
using `name` as a proxy (reexport detection, dedup), operate on
the node or source text directly. Don't leave a vestigial "name"
concept encoded where it doesn't belong.

### Slim TsItem to `{ path: &'s Path, node: Node<'s> }`

Currently `TsItem` carries `path: PathBuf`, `source: &str`, `name:
String`, `start_line: usize`, `end_line: usize`. Most are redundant:

- `path` should be `&'s Path` borrowed from the store via
  `FrozenMap::get_key_value`. Eliminates PathBuf cloning.
- `source` can be derived from path via the store when needed.
- `name` is only used by Name-level groups for `find_name_prefix`. Could
  be computed from the node and source on demand. Evaluate whether the
  `@name` query capture is still needed at all.
- `start_line`/`end_line` are available from `node.start_position().row`
  and `node.end_position().row`.

### clone_ts_item clones per item per child group

Every `children()` call clones `PathBuf` + `String` for each item.
Slimming TsItem (above) eliminates this since `&Path` and `Node` are
Copy.


# Post-rewrite issues

Outstanding issues from the architecture rewrite that need addressing
before calibration.

## Must fix (design violations)

### Heading nesting not implemented

Headings of all levels enter the frontier simultaneously. The design
says sub-headings should be gated behind their parent heading:

- `FilesGroup::children()` should only produce Heading groups at the
  shallowest level present (usually h1).
- `TsGroup::children()` for `Heading(level=N)` should spawn
  `Heading(level=N+1)` by walking each item's tree-sitter node
  siblings, stopping at same-or-lower level.
- May also want a `MarkdownBody` group for content before the first
  heading.

### Class/Impl method spawning not implemented

The design says methods inside classes, interfaces, traits, and impl
blocks should be spawned as `FunctionName` children of the enclosing
type. Currently `ClassName::children()` only spawns doc/body groups,
not method children.

Wiring site: `TsGroup::children()` for `ClassName`, `InterfaceName`,
`TraitName`, `ImplBlock` keys. Walk each item's tree-sitter node to
find method/function child nodes and produce `FunctionName` groups.

### FoldersGroup doesn't carry items

The design §3.2 specifies `Folders { parent_dir, items: Vec<PathBuf> }`
where `items` are the child folders of `parent_dir`, scheduled and
rendered as a unit (D2, A3). The current implementation creates one
`FoldersGroup` per individual sub-folder with no items — each
sub-folder is scheduled independently. This breaks atomicity and
honest semantics: the group doesn't represent what the design says it
represents.

The fix: `FoldersGroup::children()` should return a single
`FoldersGroup` with `items = discovered_sub_folders` (and `FilesGroup`s
alongside it). Rendering a `FoldersGroup` shows all its items as
path-only lines. The childless-folder map tracks the items of
committed `FoldersGroup`s.

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

### Unnecessary comments

Several comments explain what the code does rather than why:
- "Drain dependent siblings" (self-evident from `std::mem::take`)
- "Spawn FunctionSig children" (restates the next line)
- Most `render_item` match arm comments

### Path relativization helper

The pattern `.strip_prefix(&ctx.root).unwrap_or(&path).to_path_buf()`
appears ~8 times. A `ScheduleCtx::relative_path` helper or
constructing relative paths at discovery time would clean this up.

### `FileCache::assemble` re-formats headers

`format::header_line(path)` is called in `assemble()` even though the
header was already computed in `header_cost_for`. Cache the formatted
string alongside the cost.

### `is_source_file` accepts binary files

`is_source_file` returns `true` for any file with an extension that
isn't a lockfile. This means `.png`, `.jpg`, `.wasm`, `.exe`, `.zip`
etc. all pass the filter. It should use an allowlist of text/source
extensions or at minimum a denylist of known binary extensions.

### `classify_file` called on directories

`folders.rs:59` calls `classify_file(&rel_dir)` on directory paths to
get a `FileCategory` for `FoldersGroup`. The function was designed for
files (checks file stems for `.test`/`.spec` suffixes, etc.). It happens
to work for directories because the path-component checks fire first,
but the intent is unclear and the file-stem checks are wasted work.
Consider a `classify_dir` variant or documenting that the function
handles both.

### `is_config_file` scope is too broad

The function classifies CSS/SCSS/HTML/SVG files (line 420) and
everything under `scripts/`/`tools/` directories (line 434) as config.
These aren't config files — they're low-priority source or assets. The
function name doesn't match what it actually tests (more like
"is_deprioritized_file"). Either rename or split the concerns.

### `is_generated` detection never affects scheduling

`files_contribution()` in `heuristics.rs:122` accepts `is_generated`
and applies a 0.1x factor — but the callsite in `folders.rs:96` always
passes `false` because detection requires reading file content, which
only happens later in `FilesGroup::children()`. After detection, the
flag is stored on `FilesGroup` (`files.rs:55`) but never propagated
to child TsGroup modifiers — `compute_item_modifier` doesn't read it.
Generated files get the same priority as non-generated ones.

Fix: either (a) apply the generated factor in `compute_item_modifier`
using `g.is_generated` when constructing child TsGroups, or (b) defer
the full modifier computation for the FilesGroup itself until
`children()` time, when the source has been read.

### Per-file properties computed from first file only

`folders.rs:80-84` computes `is_config`, `is_type_declaration`, and
`is_header` from `files[0]` and applies the result to the entire
`FilesGroup`. If a role group mixes files with different properties
(e.g. one `.h` and one `.c` file both classified as `Normal`), the
contribution is wrong for all but the first.


### `is_generated` taints entire FilesGroup from a single file

`files.rs:49-56` sets `g.is_generated = true` as soon as any file in
the group is detected as generated. Since `is_generated` is a
group-level flag, one generated file deprioritizes all files in the
group. This is incorrect when a `FilesGroup` contains a mix of
generated and hand-written files (e.g. a directory with both
`schema.generated.ts` and `schema.ts`).

Fix: track generated status per-file rather than per-group. When
constructing child TsGroups, split items from generated files into
separate groups with a lower inherited modifier (or apply the factor
per-item during `compute_item_modifier`).

### No Lua inline sample tests

Every supported language has an inline sample test (`rust_sample`,
`python_sample`, `go_sample`, etc.) except Lua. Since Lua has known
issues (function assignments classified as Const, doc comments not
detected), having sample tests would help catch regressions during
fixes.

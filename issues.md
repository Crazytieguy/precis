# Post-rewrite issues

Outstanding issues from the architecture rewrite that need addressing
before calibration.

## Must fix (design violations)

### dependent_siblings not wired up

The design §4 taxonomy specifies dependent sibling relationships (`⇢`)
that gate lower-priority groups behind higher-priority ones. None are
currently wired up — all groups enter the frontier simultaneously when
their parent commits.

Required gating relationships:
- `FunctionName(public=true)` ⇢ `FunctionName(public=false)` (same for
  all Name groups: Struct, Enum, Class, Interface, Trait, TypeAlias,
  Const, Macro)
- `Import(first_party=true)` ⇢ `Import(first_party=false)`

Wiring site: `FilesGroup::children()` in `src/group/files.rs`. After
bucketing items into TsGroups, pair up public/private variants and
place the private one in `public_group.dependent_siblings`.

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

### is_documented flag misses text-heuristic doc comments

The `is_documented` flag on `ExtractedItem` uses AST-based doc
detection (`compute_doc_start_line`) which only finds doc comments
visible to tree-sitter. Languages where tree-sitter doesn't label
doc comments — Go (`//`), Lua (`--`), Python (`#`) — fall back to
text heuristics (`doc_comment_start` from the old layout/doc.rs, now
in `group/ts.rs`). But those text heuristics run at render time, not
at extraction time. So items in Go/Lua/Python may be classified as
undocumented and placed in lower-value groups even when they have doc
comments.

Fix: run the text heuristic at extraction time to set
`is_documented` correctly.

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

### `format::truncation_marker_plain` allocates per call

Returns `"      →…\n".to_string()` — could be `&'static str`.

### `FileCache::assemble` re-formats headers

`format::header_line(path)` is called in `assemble()` even though the
header was already computed in `header_cost_for`. Cache the formatted
string alongside the cost.

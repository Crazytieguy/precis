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

### is_documented flag broken for Java and Lua, late for others

Two separate gaps in doc-comment detection:

1. **AST detection missing Java and Lua entirely.**
   `is_doc_comment_node` in `parse/ast.rs:114` has a `_ => false`
   catch-all. Java (`/** ... */` javadoc) and Lua (`--- ...` LDoc)
   are never recognized as doc comments at the AST level. Java should
   match `/**` like JsTs; Lua should match `---`.

2. **Text heuristics run at render time, not extraction time.**
   Languages where tree-sitter doesn't label doc comments — Lua
   (`--`), Python (docstrings are strings, not comments) — fall back
   to text heuristics (`doc_comment_start` in `group/ts.rs`). But
   those run at render time, not at extraction time. So items may be
   classified as undocumented and placed in lower-value groups even
   when they have doc comments.

Note: Go is NOT affected — `is_doc_comment_node` returns `true` for
all Go comments, which is correct since Go convention treats any
comment immediately before a declaration as documentation.

Fix: (a) add Java and Lua arms to `is_doc_comment_node`, (b) run the
text heuristic at extraction time to set `is_documented` correctly.

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

### `compute_sig_end` used for structs and enums (no signature)

`render_item` at `ts.rs:424` handles `StructBody` and `EnumBody`
by calling `compute_sig_end(item)` to find where the "body" starts.
But structs and enums don't have signatures — they have a
declaration line and then fields/variants. Reusing the
signature-finding function is semantically dishonest: it happens to
work because `compute_sig_end_line` finds the `body` field's start
row, which is the `{` line for both functions and structs. But the
abstraction is wrong — a struct's `{` is not the end of a signature.

Fix: rename or split the helper. A `compute_body_start_line`
function that returns the first line of the body block would be
semantically correct for all three cases (functions, structs, enums).

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

### `format::truncation_marker_plain` allocates per call

Returns `"      →…\n".to_string()` — could be `&'static str`.

Additionally, `format::truncation_marker` (the indentation-aware variant at
line 16) is dead code — never called anywhere in the codebase. Remove it.

### `FileCache::assemble` re-formats headers

`format::header_line(path)` is called in `assemble()` even though the
header was already computed in `header_cost_for`. Cache the formatted
string alongside the cost.

### `is_source_file` accepts binary files

`is_source_file` returns `true` for any file with an extension that
isn't a lockfile. This means `.png`, `.jpg`, `.wasm`, `.exe`, `.zip`
etc. all pass the filter. It should use an allowlist of text/source
extensions or at minimum a denylist of known binary extensions.

### `detect_doc_site_dirs` is dead code

Defined in `classify.rs:307` but never called anywhere in the codebase.
Remove it (and the helper `is_docs_dir_name` at line 303 which is only
used by `detect_doc_site_dirs` and `is_config_file`).

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

### `mark_reexports` misses `crate::` and `super::` paths

`postprocess.rs:24` checks `self::` but not `crate::` or `super::`.
A `pub use crate::foo::Bar` is a reexport of an item from the same
crate, as is `pub use super::Bar`. Both should set `is_reexport = true`.

Fix: add `item.name.starts_with("crate::")` and
`item.name.starts_with("super::")` checks alongside the `self::` one.

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

### `Module` items mapped to `ConstName`

`item_to_group_keys` at `files.rs:167` maps `ItemKind::Module` (Rust
`mod` declarations, C++ namespaces, Java modules) to `ConstName`.
These aren't constants — they're namespace/module declarations. The
design taxonomy doesn't have a `ModuleName` variant, but silently
misclassifying them as constants is semantically dishonest and gives
them `ConstName` value heuristics (which are tuned for actual
constants).

Fix: either add a dedicated group key (e.g. `ModuleName`) with
appropriate heuristics, or if module declarations don't carry enough
information to warrant their own group, filter them out at extraction
time (they're already low-signal — just `mod foo;` one-liners).

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

### `readme_example_matches_output` silently passes when markers missing

`tests/snapshots.rs:1011` uses `if let` to find the README markers.
If someone removes or renames the `<!-- precis-example-start -->` /
`<!-- precis-example-end -->` markers, the test body never executes
and the test passes silently. This should be a hard failure — the
markers are expected to exist.

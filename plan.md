# Precis Rewrite — Implementation Plan

## Context

`design.md` describes a full rewrite of precis's internal data model, scheduling,
and rendering. The current code is structured around a single `Symbol` struct
whose fields only make sense for certain kinds, a stage-based scheduler whose
precomputed cumulative costs can drift from what the renderer produces, and
"Symbol-granular" scheduling that forces hacks for finer (doc lines, composites)
and coarser (folders) levels.

The new design replaces this with a polymorphic group taxonomy (`Folders`,
`Files`, tree-sitter groups), a greedy frontier scheduler driven by
`value / cost`, and a "render and count" cost model backed by a per-file cache.

This plan covers the concrete implementation choices that are **not** prescribed
in `design.md`. The design document is the source of truth for everything else
(types, taxonomy, invariants). Read sections in `design.md` as indicated before
working on each part.

**Invariant audit (completed up-front, then hardened after seven parallel
red-team reviews).** All of §10 D/A/R/V/P invariants are believed upholdable
by the strategy below. The red-team findings have been folded in. If a fresh
contradiction surfaces during implementation, stop and report — do not work
around it.

## Session preferences (read before starting)

- **Work in the main worktree.** Do not call `EnterWorktree` for this task.
- **Stopping rule for smaller ambiguities.** Small ambiguities (module
  naming, error type shape, where a heuristic should land when two slots
  fit) are the implementer's call — decide based on local clarity and
  move on. Only escalate when continuing would require violating an
  invariant from design.md §10, dropping the CLI contract, or making a
  structural decision that contradicts something explicit in this plan.

## Scope and non-goals

- **In scope:** Single-commit rewrite of all internal modules. Keep the CLI
  contract (`precis [PATH] [--budget N] [--char-budget N]`), the snapshot
  test infrastructure (`tests/snapshots.rs` + its samples, `test/fixtures.rs`,
  `src/bin/clone_fixtures.rs`), and the plugin hook wiring
  (`CLAUDE_PLUGIN_ROOT` → char budget, lives in `src/main.rs` unchanged).
- **Tests are expected to be rewritten.** Only the *snapshot* tests are
  load-bearing. The plan does **not** try to preserve the old public
  library helpers (`render_with_budget`, `render_file_with_budget`,
  `read_sources`, `build_file_data`, `Corpus`, `FileData`). Expose a new,
  minimal public API that makes sense for the new architecture — see
  "Public library surface" below — and rewrite `tests/snapshots.rs` (and
  `benches/hot_path.rs`, `src/bin/profile.rs`) against it. Anything that
  smells like a vestigial remnant of the old architecture should be
  rewritten or deleted, not kept for continuity.
- **In scope:** Regenerate all insta snapshots (with a representative
  hand review — not blind acceptance), rewrite `benches/hot_path.rs`
  against the new pipeline, rewrite `tests/snapshots.rs` against the
  new public API, and delete in-file `#[cfg(test)]` modules that test
  removed types.
- **Out of scope this session:** the /batch-driven calibration sweep
  (§12 of `design.md`). That is a separate session owned by the user.
  This session is done when the design is implemented, snapshots are
  regenerated with no egregious bugs, invariant tests pass, benches
  produce plausible numbers, and simplification has settled.
- **Do not bump the version or publish.** The user gates publishing
  explicitly.

### Public library surface (new)

The CLI in `src/main.rs` calls into `lib.rs` via one function:

```rust
pub fn render(path: &Path, budget: usize, char_budget: Option<usize>) -> String;
```

Dispatches on `path.is_file()` vs `path.is_dir()` internally. Returns
the rendered string. Tests that need to verify budget compliance call
`format::count_tokens` on the output themselves — the token count is
a test concern, not a library concern, so there's no `render_stats`.

That is the entire public surface; no `Corpus`, no `FileData`, no
`build_file_data`, no `read_sources`, no separate file-vs-directory
entry points exposed to callers. `format::count_tokens` is re-exported
for the tests.

## Implementation choices (concrete decisions beyond `design.md`)

### Module layout

```
src/
├── main.rs              # CLI (unchanged; houses CLAUDE_PLUGIN_ROOT detection)
├── lib.rs               # public API — render(path, budget, char_budget)
├── format.rs            # count_tokens, fmt_line, truncation_marker, header_line
├── store.rs             # ParseStore — FrozenMap of sources and trees
├── parse.rs             # tree-sitter queries, Lang enum, cross-kind nest filter,
│                        #   + local helpers for doc/sig/body range extraction
│                        #   (see "Text helpers — redistribution" below)
├── classify.rs          # FileRole, FileCategory, is_config, doc-site detection,
│                        #   effective_depth, is_boilerplate_heading — all the
│                        #   pure-data path/content heuristics
├── group/
│   ├── mod.rs           # `enum Group { Folders, Files, Ts }` + dispatch
│   ├── folders.rs       # Folders group: items, children(), render()
│   ├── files.rs         # Files group: items, role, children(), render()
│   └── ts.rs            # TsGroup: key enum, items, children(), render()
│                        #   (may be split into sub-modules during implementation)
├── schedule.rs          # greedy frontier scheduler (rayon probe of candidates)
├── render.rs            # LineEntry, per-file cache, assembly, override assertion
├── heuristics.rs        # all base_value() + contribution() functions, one place
└── bin/
    ├── clone_fixtures.rs   # unchanged
    └── profile.rs          # rewritten against new pipeline stages
```

No top-level `walk.rs` and no `dir_index.rs`. File discovery happens
*inside* `Folders::children()` using the `ignore` crate directly,
scoped to the folder being expanded (gitignore honoring comes for
free). The seed frontier for a directory input is a **single
`FoldersGroup` at the input root — nothing pre-expanded**. When the
greedy loop schedules it, its `children()` walks one directory level
with `ignore::WalkBuilder::new(parent_dir).max_depth(Some(1))` (or
equivalent) and classifies the immediate entries into sub-folders
and sub-files by `FileRole`, producing the root's immediate
sub-`FoldersGroup`s and `FilesGroup`s. There is no eager whole-tree
walk and no `DirIndex` structure.

This saves a chunk of complexity: no `DirIndex` type, no degenerate
single-file `DirIndex`, no `ScheduleCtx.dir_index` field, no
`discover_source_files` pre-walk. `ScheduleCtx` carries only
`&ParseStore` and the two budgets.

Non-src paths touched by the rewrite:

- `benches/hot_path.rs` — **rewritten** to bench the new stages.
- `tests/snapshots.rs` — **helper functions and monotonicity tests
  rewritten** against the new `precis::render` API (see "Testing
  and benchmarks"); the sample/fixture test *declarations* stay, but
  their helpers change underneath.
- `tests/invariants.rs` — **new file** added in step 10.
- `Cargo.toml` — add `elsa`.

### Text helpers — redistribution

The current `src/layout/` module mixes two concerns: (a) precomputing
`SymbolLayout` line ranges for the old `Symbol` struct (gone) and (b)
text-level heuristics (doc-comment range detection, signature-end
fallback, markdown leading-noise skipping, heading-badge stripping,
etc.). The (a) code is deleted outright. The (b) helpers are **not
bundled into a new `text_heuristics.rs` module** — that framing was
wrong because it grouped code by provenance rather than by the role
each helper plays in the new architecture.

In the new architecture, almost all of these helpers run at
`children()` time to compute *item ranges* (doc start line, sig end
line, body start/end), which are baked into the `TsItem` at
construction. Only one helper (`strip_heading_badges`) runs at
`render()` time because it modifies the *content* of an emitted line.

None of these are "fallbacks for when tree-sitter fails." They handle
concerns tree-sitter fundamentally doesn't cover: delimiter trimming
(tree-sitter gives the full comment node including delimiters),
semantic noise detection (no grammar understands markdown-inside-
comments), bodyless declarations (type aliases, interface methods,
constants have no `body` child node to pin a boundary), and Python
docstrings (which are expression statements *inside* the function body,
not preceding-sibling comment nodes).

All helpers redistribute into `group/ts.rs` as private functions near
their callsites:

| helper | role | called from |
|---|---|---|
| `doc_comment_start` | range-finding: where does the doc block start? Go/C `//`, Lua `--`, Java `/**`, Python `#` need text-walking (their tree-sitter grammars don't label these as doc-comment node types). | `children()` of doc-spawning kinds |
| `trim_doc_delimiters` | range-narrowing: trim `/**` / `*/` delimiter lines. Inherently post-parse. | `children()`, right after doc_comment_start |
| `docstring_end` | range-finding (Python): find `"""..."""` end. Python docstrings are body-internal, not sibling nodes. | `children()` of Python function/class kinds |
| `skip_doc_leading_noise` | range-narrowing: advance past badges/HTML in module docs so the "name" line is real content. | `children()` of ModuleDocFirst |
| `strip_doc_line_prefix` | per-line stripping of `///`/`//`/`* ` before noise check. Pure helper. | private, used by `skip_doc_leading_noise` |
| `signature_end_line` + `has_multiline_signature` + `strip_c_line_comment` + `strip_python_line_comment` | **Deleted entirely.** Tree-sitter gives the full node range. Each kind's `children()` checks if the node spans multiple lines and spawns a body group if so — no delimiter scanning needed. The body group emits all lines of the node; override resolution handles the first-line overlap with the name group. These four functions have no replacement. |
| `is_markdown_leading_noise` | per-line: badge, ToC link, HTML tag, horizontal rule? Semantic — no grammar can tell badges from screenshots. | `Heading::children()` to compute body start |
| `is_toc_link` | per-line sub-helper. | private, used by noise check |
| `trim_trailing_blank_lines` | range-narrowing one-liner. | inline in Heading/DataSection `children()` |
| `is_config_trailing_noise` | per-line: blank or `}`/`]`. | inline in DataSection `children()` |
| `strip_heading_badges` | **render-time**: strip trailing badge URLs from heading line. Returns subslice. | `Heading::render()` — the only helper at render time |

All helpers keep their existing behavior (carried over unchanged,
revisited during calibration). They live near their callsites in
`group/ts.rs`, not in a shared module.

**Pointer-range sub-invariant (R1).** Every `&str`-returning helper
(currently `strip_heading_badges`, `strip_doc_line_prefix`) returns
a pointer-range subslice of its input. This is true today (they all
use `&s[..pos]`, `&s[pos..]`, or `trim_*`) and must stay true — any
future helper that would need to synthesize a `String` is
disallowed. Step 10's invariant test checks the pointer-range
property directly for `LineEntry::Complete` / `Truncated` slices.

**In-file tests from `layout/signature.rs`** are **deleted** — the
functions they test (`signature_end_line` and its helpers) are not
carried over. Tests for the carried-over helpers
(`doc_comment_start`, noise checks, etc.) live alongside them in
`group/ts.rs` as `#[cfg(test)]` modules.

No behavior change in these helpers is in scope beyond the
pure-data refactor where needed — they're carried over and will be
revisited during calibration.

### Files and types to delete outright

Deletion is aggressive. The user's standing direction: "if any part of
the existing codebase doesn't fit the design intent precisely, it should
be deleted."

**Directories deleted in two phases.**

*Phase A — up front in Step 2*, because the new module names collide
with the old directory names and the crate must be able to build once
the new files exist:
- `src/schedule/` — replaced by top-level `schedule.rs` and `group/`.
  `classify` and `file_info` are **not** carried over as modules; see
  the type blocklist below.
- `src/parse/` — sub-modules (`ast`, `classify`, `module_doc`, `name`,
  `postprocess`, `visibility`) are mostly bookkeeping around the old
  `Symbol` struct. Before running `rm -rf`, grep `parse/module_doc.rs`
  and `parse/visibility.rs` for any heuristic still valuable
  (module-doc detection patterns, Rust `pub mod` tracking) and copy
  the *logic* (not the shapes) into short notes alongside the scaffold
  files you're about to create. Once the notes are written, delete
  the directory.

*Phase B — inside Step 4*, after its helpers are redistributed:
- `src/layout/` — its text helpers are moved into `src/group/ts.rs`
  as described in "Text helpers — redistribution" above.
  `signature_end_line` and its private helpers are **not** carried
  over (tree-sitter node ranges replace them). Only after the
  needed helpers have been copied does `src/layout/` get deleted.
  `SymbolLayout` struct and `fill_layouts` go with it. The name
  `src/layout/` does not collide with anything new, so there is no
  pressure to delete it in Phase A.

Overwritten (not deleted) later:
- `src/render.rs` — overwritten wholesale in Step 5 with the new
  `LineEntry`-based assembly.
- `src/format.rs` — overwritten in Step 5 with the trimmed surface
  (`count_tokens`, `fmt_line`, `truncation_marker`, `header_line`).

**Type blocklist (must not survive in any form, including renamed or
"temporarily kept as data type"):**

- `parse::Symbol`, `parse::SymbolKind`, `parse::LanguageConfig` (keep the
  *concept* but rename/reshape), `layout::SymbolLayout`.
- `schedule::KindCategory`, `schedule::StageKind`, `schedule::GroupKey`
  (the 18-field everything-bag anti-pattern), `schedule::Cost` (may be
  reintroduced as a tiny two-field struct inside the new render cache
  *only* if it earns its keep — prefer `struct FileCost { tokens,
  chars }` colocated with the cache).
- `schedule::SymbolRef`, `schedule::CumulativeEntry`,
  `schedule::StageCumulatives`, `schedule::Group` (old version),
  `schedule::BuiltGroups`, `schedule::SolverResult`.
- `schedule::plan::{Schedule, RenderPlanItem, SymbolRenderSpec}` — the
  render-plan intermediate layer is gone; rendering reads scheduled
  groups directly.
- `schedule::FileInfo` — the per-file property bag is **not** reborn as
  a struct passed to `children()`. See "FileInfo blob risk" below.
- `lib::Corpus`, `lib::FileData` — the whole-file containers are gone.
- `format::format_symbol_name`, `format::find_word` — tied to `Symbol`;
  delete with prejudice.

**`format.rs` inventory (explicit):** keep `count_tokens`, `fmt_line`,
`truncation_marker`, and a new shared helper `header_line(path: &Path)
-> String` (used by both the render cache's `header_cost` pre-computation
and the final-assembly file/folder header emission — same function so the
two cannot drift; addresses V3). Delete `format_symbol_name`,
`find_word`, and the old `BPE` static (or keep the static and just
remove the helpers that consume `Symbol`).

**FileInfo blob risk.** `FilesGroup` carries `parent_dir: PathBuf`,
`role: FileRole`, and the concrete `items: Vec<PathBuf>` — the
enumerated set of files it was constructed from (per design §3.2 —
this is the minimal deterministic payload). What it does **not** carry
is a pre-materialized `FileInfo`-like blob of file properties.
Properties like `is_config`, `is_header`, `is_generated`,
`is_type_declaration` are queried on demand inside `Files::children()`
from the items' paths (pure `classify.rs` calls) or a lightweight
source peek (only when needed for modifier computation). The `items`
list is not a blob — it's the stable set of files the scheduler
evaluated when the group was picked, and it must stay attached to the
group for determinism (re-walking the filesystem inside a scheduled
group's `children()` would expose the scheduler to mid-run FS churn).

**`#[cfg(test)]` cleanup.** For every deleted file, enumerate its
in-file test modules and delete them (they test old-model behavior).
Exceptions — migrate test *logic* to the new home:

- `src/schedule/value.rs::tests::effective_depth_skips_source_roots` —
  the heuristic survives (see `classify::effective_depth` below); the
  test moves with it, to `src/classify.rs`.
- `src/layout/signature.rs::tests::*` — rewritten as noted above.
- `src/lib.rs::tests::budget_monotonicity` (if present) — deleted
  outright; its job is taken over by the rewritten
  `budget_monotonicity_fixture` in `tests/snapshots.rs`.

### Polymorphism over groups

Plain Rust enum `Group { Folders(FoldersGroup), Files(FilesGroup),
Ts(TsGroup) }` with `fn value`, `fn touches_files`, `fn children`,
`fn render` implemented by matching. No `enum_dispatch` or trait object.

**D2 homogeneity assertion.** `TsGroup` construction asserts via
`debug_assert!` that every item's underlying node kind is compatible
with `self.kind: TsGroupKey`. Construction goes through a private
`TsGroup::new(kind, inherited_modifier, items)` that performs this
check; callers in `children()` implementations never construct a
`TsGroup` directly with a struct literal.

**D3 no-empty-groups assertion.** Every `children()` implementation
partitions its inputs into buckets and returns only non-empty buckets.
`FoldersGroup::new`, `FilesGroup::new`, and `TsGroup::new` each assert
`debug_assert!(!items.is_empty())`. The `dependent_siblings` field is
an owned `Vec<Group>` that can be empty (the common case — most groups
have no dependent siblings); "empty" means "no siblings to reveal."
What is *not* allowed is a dependent sibling entry that is itself a
group with zero items; that's caught by the `new()` assertion on the
inner group. Empty buckets are a programming error, not a runtime
condition.

**D5 parent/descendant spawning chain.** Within a single source item,
the groups that render it form a strict chain, never siblings. Concrete
chains (each `→` is "produced by the parent's `children()` call"):

- Function: `FunctionName → FunctionSig → FunctionBody`.
- Function doc: `FunctionDocFirst → FunctionDocRest` (produced by
  `FunctionName.children()` alongside `FunctionSig`; `FunctionDocFirst`
  is a parent of `FunctionDocRest` in its own sub-chain; the two chains
  `FunctionName → FunctionSig → FunctionBody` and `FunctionName →
  FunctionDocFirst → FunctionDocRest` both share `FunctionName` as root
  but never co-emit on the same line because `FunctionSig` emits on the
  declaration line+, `FunctionDocFirst` on doc lines strictly before it).
- Struct: `StructName → StructBody`; `StructName → StructDocFirst →
  StructDocRest`.
- Enum, Class, Interface, Trait, TypeAlias, Const, Macro: same pattern.
- Heading: `Heading → HeadingBody`; `Heading(level=N) → Heading(level=N+1)`
  for nested subheadings (they attach as dependent siblings of the parent).
- ImplBlock: `ImplBlock → <FunctionName children for each method>`.
- ModuleDoc: `ModuleDocFirst → ModuleDocRest`.
- Import: `Import → ImportedItems`.

With this spawning chain, D5 is enforced by construction: any two groups
belonging to the same source item stand in a strict ancestor/descendant
relationship, never sibling. A `debug_assert!` in the per-file render
override path can cross-check: if two entries at the same line have
different "origin item" markers, that's a bug; if they share an origin
item, their group types must trace to a parent/descendant pair in the
list above.

**D4 non-overlapping top-level items — cross-kind filter.** The nesting
filter in `src/parse.rs` drops a match if any ancestor `@symbol` node
(**of any kind**, not just the same kind) is also a match in the same
file's extracted set. The filter runs once over the *union* of all
kind matches per file. Example: a `fn` inside another `fn` is dropped
(same kind); a `struct` inside a `fn` body is also dropped (cross-kind).
Accepted trade-off: nested items are not individually surfacable. A
`debug_assert!` after filtering sorts all top-level items by
`start_line` and verifies `prev.end_line < next.start_line`.

**D7 cross-file aggregation.** `FilesGroup::children()` parses every
file in the group, extracts top-level items, and aggregates items
**across all files** into one `TsGroup` per `(TsGroupKey,
inherited_modifier)` bucket. Per-file splitting happens only when two
files would contribute items with different inherited modifiers (e.g. a
`pub mod` file and a private `mod` file inside the same `Files` group).
In that case, partition items by modifier and emit one `TsGroup` per
modifier bucket — still aggregating items from all files that share
that modifier. **Do not emit per-file groups for structural reasons
alone**; that would silently violate D7.

**P3 partitioning rule.** A `children()` implementation may only
partition items into sub-groups when the partitions carry different
`inherited_modifier` values (or different `TsGroupKey` discriminants
when the design calls for them — e.g., `documented: bool`,
`public: bool`, `is_trait_impl: bool`). Partitioning for any other
reason is a P3 violation.

### Parse storage (§3.5)

Add `elsa` as a dependency. Introduce `src/store.rs`:

```rust
pub struct ParseStore {
    sources: elsa::FrozenMap<PathBuf, Box<String>>,
    trees:   elsa::FrozenMap<PathBuf, Box<tree_sitter::Tree>>,
    configs: HashMap<Lang, LanguageConfig>,   // pre-compiled queries
}
```

`ParseStore` has **two separate methods** so A5 (lazy parsing) is
enforced by types, not a `debug_assert!`:

```rust
impl ParseStore {
    /// Read, tokenize, and store. Allowed only from `FilesGroup::children()`.
    /// Returns `None` if the file fails to read / parse.
    pub fn parse(&self, path: &Path) -> Option<(&str, &Tree)>;

    /// Look up previously-parsed state. Returns `None` if not present.
    /// `render()` and `heuristics.rs` use this; never `parse`.
    pub fn get(&self, path: &Path) -> Option<(&str, &Tree)>;
}
```

`TsGroup::render()` and every heuristics callsite take an immutable
borrow and call `get`, which cannot trigger a parse. `FilesGroup::
children()` is the only site that calls `parse`. A5 is now a
consequence of which method a callsite can reach, not a runtime
check.

`TsItem<'t>` borrows `&'t Path` and `Node<'t>` from the stored
source/tree. All `LineEntry::Complete` / `Truncated`'s `&'src str` slices
borrow from the `String` stored here. Threaded through `schedule()`
and `render()` as `&ParseStore`.

### Greedy scheduler

`src/schedule.rs`:

```rust
/// Read-only shared context threaded through `children()` calls.
pub struct ScheduleCtx<'s> {
    pub store: &'s ParseStore,
    pub budget: usize,
    pub char_budget: Option<usize>,
}

pub struct ScheduledSet<'s> { /* owns scheduled groups + per-file cache */ }

pub fn schedule<'s>(
    seed: Vec<Group<'s>>, ctx: &ScheduleCtx<'s>,
) -> ScheduledSet<'s>;
```

Every `Group::children(&ctx)` method receives this context:
`FoldersGroup::children` does a one-level `ignore::WalkBuilder` walk
of its `parent_dir`, `FilesGroup::children` calls
`ctx.store.parse(path)` for each of its files. `src/lib.rs` constructs
`ParseStore` and `ScheduleCtx`, builds the seed frontier as described
above, calls `schedule`, then hands the same `ctx.store` to the
renderer for final line-slice borrowing.

Internally:
1. `frontier: Vec<Group>` starts from `seed`.
2. Loop: `par_iter()` frontier (by index, so we can remove by index
   after), compute marginal cost (see below) and the ratio (see
   "Tiebreaking") for each candidate that fits. Reduce to the best by
   the total-order key, carrying its frontier index.
3. **Commit is one atomic block.** In order: (a) **remove** the winning
   group from the frontier (by its carried index — typically
   `swap_remove`, compensating for non-determinism from `swap_remove` by
   re-sorting at the top of each iteration if needed, or
   `frontier.remove(idx)` for stability at higher cost). (b) Call
   `best.children(&ctx)` and hold the returned new groups in a local
   variable. (c) Push the winning group into `scheduled` and update
   the per-file cache (this is where the file/folder header becomes
   part of the cache entry for the touched file). (d) Extend
   `frontier` with the children produced in (b). All four happen
   before any probe runs again. P4 relies on this ordering: by the
   time a `Files` group's header is reflected in the cache, its
   children have been spawned and are in the frontier for the next
   iteration to probe.
4. Break when no frontier group fits. A group that has been committed
   is never reconsidered because it is no longer in the frontier —
   there is no path to an infinite zero-cost loop.

**A3 purity.** Probe and commit share **exactly one** render function —
a pure `fn render_file(entries: &[LineEntry], source: &str, header: &str)
-> String`. Both paths sort `(line, scheduling_order)`, apply override
resolution, and tokenize through the same code. There is never a "cost
model" version that differs from the "commit" version.

Probes are **pure**: each rayon thread takes `&FileCache` (the
committed state) as read-only, computes a marginal-cost delta, and
**returns** its result — the rendered strings, token counts, and
`LineEntry` lists for the files it touched — without writing to any
shared state. The main thread collects these results and picks the
best. On commit, the main thread writes the winning candidate's
returned data into the cache. No interior mutability, no `DashMap`,
no `OnceCell`. The cache is single-writer (main thread only),
multi-reader (rayon probes).

Commit is serial.

### Tiebreaking / total-order comparator (A1)

The rayon `par_iter().reduce(...)` comparator must be a **total order**.
Cross-multiplication on floats is not transitive under rounding and can
produce rock-paper-scissors cycles, breaking determinism. Instead,
compute `ratio = value / cost` as `f64` once per candidate during the
probe and order by `(ratio.to_bits()` descending as bits of a sign-aware
representation — using `f64::total_cmp` descending for robustness,
first_path ascending, first_line ascending, kind_ordinal ascending).
`f64::total_cmp` gives a deterministic total order over all `f64`
including NaN, so even pathological ratios sort stably.

**Zero-cost candidates (cost == 0).** A candidate whose marginal cost is
zero (because its emissions are already fully overridden by earlier-
scheduled groups) is treated as `ratio = f64::INFINITY` and scheduled
before any positive-cost candidate. Ties among zero-cost candidates fall
through to the path/line/kind key. Zero-cost candidates are still
subject to the `char_budget` acceptance gate. This is correct behavior
(free content, take it); A3 atomicity still holds because the commit
re-runs the same render code and records the same zero delta.

### Per-file cache & cost (§6.2, §8.3)

```rust
struct FileCache<'s> {
    entries:     HashMap<PathBuf, FileEntries<'s>>,
    header_cost: HashMap<PathBuf, FileCost>,   // precomputed once per path
    /// Insertion-ordered list of paths for deterministic final assembly.
    /// Entries are appended when a path first appears in the cache.
    path_order:  Vec<PathBuf>,
    total_tokens: usize,
    total_chars:  usize,
}

struct FileEntries<'s> {
    /// Accumulated entries from all scheduled groups for this file.
    lines: Vec<LineEntry<'s>>,
    /// Rendered output of this file (header + sorted entries), cached.
    rendered: String,
    tokens: usize,
    chars: usize,
}

#[derive(Copy, Clone, Default)]
struct FileCost { tokens: usize, chars: usize }
```

**Marginal cost.** For each file F touched by the candidate:
- Build hypothetical entries = `existing(F) + candidate.emit(F)`.
- Sort and override-resolve (§6.2) via the same pure `render_file` used
  at commit time.
- Tokenize the resulting string, produce `new_tokens`.
- Delta for F = `new_tokens - existing_tokens(F)`.

Plus: on first touch of a file or folder path whose header is not yet
in the cache, include `header_cost[path]` in the delta. Header costs
are memoized on first request via `format::header_line(path)` — the
**same** helper used at final assembly, so there is no drift between
the precomputed cost and the rendered output (V3).

A file header is not an independently schedulable unit. A candidate
whose content would fit but whose first-touch header pushes the total
over budget is rejected outright (A4 is preserved; no implicit
"header-only" floor).

**Group-level emission caching.** Each group's `render()` is called
at most once. The first time a group is probed, the rayon thread
calls `render()` on it and the result (`Vec<LineEntry>`) is stored
on the group struct (via a simple `Option<Vec<LineEntry<'s>>>`
field set by the main thread after collecting the probe result).
Subsequent probes of the same group — whether in later iterations
because the group keeps losing but stays in the frontier — reuse
the cached entries directly. `render()` is pure (R3), so this is
free correctness. In the rayon probe phase, the main thread
partitions frontier groups into "already rendered" (skip `render()`,
just use cached entries) vs "not yet rendered" (call `render()` and
return entries to the main thread for storage). No thread ever
writes to the group struct during the parallel phase.

**Explicit change-tracking for per-file re-rendering.** The per-file
cache (`FileEntries`) has a `dirty: bool` flag. On commit, the main
thread marks as dirty only the files touched by the committed group
(the files whose entry list just grew). During the next probe phase,
a probe for candidate C on file F checks:
- If F is **not dirty** (no new entries since C's last probe of F)
  and C's own `LineEntry`s for F are already cached (C was probed
  before and hasn't changed), the per-file delta for F is the same
  as last time — return the cached delta directly, no re-render.
- If F **is dirty** or this is C's first probe of F, build the
  hypothetical entry list `existing(F) + C.emit(F)`, run
  `render_file`, tokenize, and return `(rendered, tokens, chars)`
  to the main thread. The main thread updates C's cached delta for
  F and clears F's dirty flag for subsequent candidates.

This avoids re-rendering any file whose committed entries haven't
changed since the last iteration, which is the common case (each
iteration commits one group, touching a small number of files;
most frontier groups touch files that didn't change).

Sum over all touched files → `marginal_cost` (token cost). Char cost is
computed on the same hypothetical strings and used **only** as an
acceptance gate, never in the ratio (V4).

**Commit.** When the best candidate is committed, the scheduler re-runs
the same `render_file` on exactly the touched files, writes the result
back into the cache, and updates `total_tokens` / `total_chars`.

**Final assembly.** After scheduling halts, iterate `path_order`
(the insertion-ordered list of paths) and concatenate each path's
cached render with blank-line separators. Because paths are appended
to `path_order` when they first appear in the cache (which happens
in scheduling order, i.e. the order `Folders::children()` discovered
them), the output is deterministic and reflects the directory tree's
natural structure — parent dirs before children, files sorted within
each dir by the `ignore` crate's traversal order (which is
lexicographic by default). Folders that
were scheduled but whose descendant files didn't contribute any
emissions render as a single path-only line (their `Folders::render()`
already emitted just the path string). See P2 below for how these
survive in the output.

### Override-resolution assertion (R4)

The renderer's per-file sort is by `(line, scheduling_order)`. After
sorting, when two consecutive entries share a line, a `debug_assert!`
checks content-rank monotonicity:

- Content rank: `Ellipsis < Truncated < Complete`. (The design doc
  §10 uses `Full` and `Prefix` for the same concepts; this plan
  renames them for semantic clarity. The design doc names are
  non-binding per §10's own preamble — "exact type shapes, naming"
  are guidance.)
- If later_rank > earlier_rank: OK (strict growth, e.g. `Ellipsis`
  overwritten by `Truncated`).
- If later_rank == earlier_rank and both are `Truncated`: OK iff
  later.content.len() >= earlier.content.len() (equal is allowed — the
  render output is unchanged).
- If later_rank == earlier_rank and both are `Complete`: OK iff the two
  slices are byte-equal. This *will* happen in practice at cross-group
  boundary lines (e.g., `FunctionSig` and `FunctionBody` touching at
  the `{` line; `Heading` and `HeadingBody` on the heading line). The
  assertion explicitly permits the no-op duplicate case.
- If later_rank < earlier_rank (small content after larger): assertion
  trips. This should not occur given A2 (parent-before-child) and the
  spawning chains listed under D5 — children that override parent
  emissions always grow the content. If it does trip, that's a real bug.
  **Cross-group line-sharing pairs are enumerated in D5 above and every
  pair is checked by inspection to be safe-by-construction**; the
  assertion is a backstop.

### Lifetimes

Pervasive `'s` lifetime bound to the `ParseStore`. `Group<'s>`,
`TsItem<'s>`, `LineEntry<'s>`, `FileCache<'s>`, `FileEntries<'s>`,
`ScheduleCtx<'s>`, `ScheduledSet<'s>`. The CLI entry point constructs
the store, calls `schedule()` and `render()` with a borrow, then drops
the store at the end. `Group::render()` takes only `&self` and
`&ParseStore` — no access to the scheduled set, no access to the
frontier (R3). Merging and override resolution happen outside
`render()`, in the per-file assembly loop.

### Tree-sitter nesting filter (§7.2)

Post-query pass in `src/parse.rs`. Collect all `@symbol` matches from
all kinds in a single set per file, then drop any match whose
`@symbol` node has any ancestor that is also in the set. Cross-kind,
not same-kind. Each surviving `TsItem`'s node is the top-level one we
pass to the renderer. See D4 above.

### Folder/file discovery

Discovery happens *inside* `Folders::children()`, one directory level at
a time. No pre-walk, no flat file list, no `DirIndex`. Concretely, when
a `FoldersGroup(parent_dir)` is scheduled and its `children()` runs:

1. Walk `parent_dir` with `ignore::WalkBuilder::new(parent_dir)
   .max_depth(Some(1)).build()` — this honors `.gitignore` and returns
   only the immediate children of `parent_dir`.
2. For each immediate entry:
   - If it's a directory that isn't vendored/fixture-excluded (reuse the
     existing `is_vendored_or_fixture` predicate, moved into
     `classify.rs`), add it to a `subdirs: Vec<PathBuf>` list.
   - If it's a file that passes the "source file" filter (reuse
     `is_source_file` / `is_lockfile` logic, moved into `classify.rs`),
     classify it by `FileRole::from_path` and push it into the bucket
     for that role.
3. Return one `FoldersGroup(subdir)` per subdir, plus one
   `FilesGroup(parent_dir, role)` per non-empty role bucket. Each of
   these has its `inherited_modifier` computed from the parent
   `FoldersGroup`'s contribution (per the heuristics table).

**Seed frontier construction.** For a directory input, the seed is a
single `FoldersGroup` at the input root — nothing pre-expanded. The
greedy loop schedules it like any other group; because the root
folder's "header" is either empty (the input path is the current
directory `.`) or cheap, and because heuristics give `FoldersGroup`
high enough value, the root typically wins the first iteration.
When it commits, its `children()` runs per the atomic-commit order,
spawning the root's immediate sub-folders and file groups into the
frontier. From that point the rest of the tree expands through the
normal greedy loop.

For a single-file input, the seed is a single `FilesGroup` for that
file's parent dir and role, with `items = vec![that one file]`. Its
`children()` call then parses the file and spawns TsGroup children.
The outer folder is not seeded, so no directory header appears in
the output (per design §5.2, path is implicit from the CLI
argument).

A5 (lazy parsing) is preserved: `Folders::children()` only walks the
filesystem; `FilesGroup::children()` is still the only place
`ParseStore::parse` is called.

The file-discovery helpers currently in `src/walk.rs` — `is_source_file`,
`is_lockfile`, `is_vendored_or_fixture` — move into `src/classify.rs`
as pure path predicates. `src/walk.rs` itself is deleted (its only
existing export, `discover_source_files`, has no caller in the new
architecture).

### Preserving current output format

- File header line = relative path, no leading marker, appears exactly
  once per file.
- Folder header = `path/` (trailing slash), emitted when a folder
  appears as path-only between files.
- Blank line between files/folders.
- Tree-sitter group rendering emits `"    N→<content>"` / `"      →…"`
  using the helpers in `format.rs`.
- Line-number gap between independently-scheduled items = natural. No
  inter-item `      →…` marker. This is a user-visible diff vs. current
  behavior and is called out in Step 9.
- **Intra-group ellipses are preserved** (e.g., `FunctionDocFirst`
  emits one content line plus an `Ellipsis` entry when the doc block
  has more than one line; `Truncated` entries append `…`). R5 forbids
  *inter-group* ellipses between independently scheduled items, not
  intra-group ones.

### P2 path-only directory survival

Deprioritized directories appear in the output as path-only entries
(per design §6.2 and the current `test/` case in the mitt example).

- **Folders groups are cheap and high-value.** The heuristics give
  `FoldersGroup` a high enough base_value that the greedy scheduler
  typically picks them early — but this is not guaranteed by the
  design. If a Folders group is never scheduled, its path simply
  doesn't appear, which is acceptable (budget was too tight even
  for a cheap entry).
- **Childless-folder map.** A separate `HashMap<PathBuf, usize>`
  (path → header token cost) tracks folders that have been
  scheduled but don't yet have any committed children. The
  lifecycle:
  1. **On `FoldersGroup` commit:** its `children()` returns
     sub-`FoldersGroup`s and `FilesGroup`s. Each sub-folder's path
     is added to the map with its header token cost. (A single
     `FoldersGroup` commit typically adds multiple entries — one
     per immediate sub-folder.)
  2. **During probe of a `FoldersGroup` or `FilesGroup`:** check
     whether the group's `parent_dir` is in the map. If so,
     subtract the parent's header cost from the marginal cost (the
     parent's path-only entry will disappear from the output when
     this child commits, since the child's file paths subsume it).
     This discount applies once to the whole group regardless of
     how many items (files/sub-folders) it contains, because they
     all share the same parent.
  3. **On `FoldersGroup` or `FilesGroup` commit:** if `parent_dir`
     is in the map, remove it and apply the discounted marginal
     cost. The first child to commit under a folder gets the
     discount; subsequent siblings under the same parent don't
     (parent already removed from the map).
  4. **Final assembly:** any paths still in the map at the end are
     folders whose children were all deprioritized. Emit them as
     path-only entries in the output (e.g., `test/`). Folders
     whose children were committed are not in the map and don't
     appear as standalone path entries — their descendant file
     paths already show the folder structure.
- **No refunds, no negative deltas.** Budget accounting is always
  forward — the discount is applied at probe/commit time, not
  retroactively. `remaining_budget` is monotonically decreasing.
  The mechanism is a simple HashMap lookup, not part of the
  per-file render cache.

### P4 defensibility of path-only entries

When a file appears in the final output with just its header and no
contents, the reader infers "this file was considered and its contents
were deprioritized." That claim is defensible **because of the atomic
commit order** described under "Greedy scheduler" above: when a
`Files` group is picked, commit step (b) runs `children()` and step
(d) extends the frontier with those children **before any subsequent
probe**. So by the time the loop terminates and final assembly reads
the cache to build the output, every committed `Files` group has had
its `TsGroup` descendants sit in the frontier for at least one
iteration, competing against everything else. If none of them won, the
"considered and deprioritized" reading is accurate.

The degenerate cases — a `Files::children()` call that returns no
groups at all (file has no extractable content), or a file whose
children were spawned but then rejected because the header itself
consumed the last of the remaining budget — both remain defensible:
in both cases the children were in the frontier and provably could
not contribute. A `Files` group header never reaches the final output
without its children having been given a competitive slot.

### Heuristics mapping (§9)

All `base_value()` and `contribution()` functions live in
`src/heuristics.rs`, one per group kind, in clearly-labeled sections.
Every existing heuristic from `src/schedule/value.rs` maps to one of
three places (group-key discriminator, `base_value` shape, or
`contribution` modifier). The **general rule** is:

- **Discriminator** when items with different values of the property
  should be able to appear *independently* in the output (one class
  schedulable without forcing the other).
- **Contribution** when the property is "the same kind of thing,
  just weighted differently" — items co-schedule within a modifier
  bucket.

A handful of design-doc `TsGroupKey` sketches need to be extended for
the discriminator side; those additions are listed under "TsGroupKey
extensions" below.

| current factor | new home |
|---|---|
| `visibility` (pub vs private) | `TsGroupKey::FunctionName { ..., public: bool }` / same on StructName/etc. Non-public variants have a lower `base_value` (names cost less to reveal) **and** contribute a lower `inherited_modifier` to their sig/body/doc children (private *detail* deprioritized more than private *names*). Two independent tuning knobs — don't derive one from the other. |
| `documented` | `TsGroupKey` discriminant (`documented: bool`). Keep. |
| `effective_depth` | `FoldersGroup::new` computes `classify::effective_depth(parent_dir)` once from the *full* parent_dir — not per-step — and sets it as a multiplier on the new group's `inherited_modifier`. Non-geometric because `effective_depth` skips conventional source roots (`src`, `lib`, `packages`, `crates`) and handles the monorepo double-root case. V2 composition is preserved: this is one absolute multiplier contribution at one spawn point. |
| `file_role_factor` + `is_root_dir` bonus | **FilesGroup discriminator** (already is — `FilesGroup` has `role: FileRole`). The root-README / root-Architecture `×1.5` bonus is a one-shot contribution computed in `FoldersGroup::contribution` when spawning a `FilesGroup` at the root (`parent_dir.is_empty()`). Both properties are visible at the spawn site. |
| `config_factor` | **FilesGroup discriminator.** Extend `FileRole` (or add `is_config` to the `FilesGroup` key) so `Files{config=Cargo.toml}` and `Files{source=src/foo.rs}` schedule independently. Otherwise, pure contribution would let the scheduler merge "config section names" and "source function names" across `Files` boundaries in ways that are hard to reason about. |
| `file_category_factor` | **FoldersGroup discriminator.** Extend `FoldersGroup` with `category: FileCategory` so a `tests/` subtree and a `src/` subtree schedule independently at the folder level. Currently-implicit today; make it explicit. |
| `type_declaration_factor` | **FilesGroup discriminator.** `.d.ts` files form their own `Files` group (role or a `type_declaration: bool` flag on the key). Competes independently with `.ts` source. |
| `header_factor` | **FilesGroup discriminator** — C/C++ headers are a distinct role (public API surface). Add a `Header` role to `FileRole` (or a `is_header: bool` on the key). |
| `heading_depth_factor` | `TsGroupKey::Heading { level, ... }` already discriminates on level. Put the depth multiplier **entirely in `base_value`** as a per-level constant (1.0, 0.6, 0.15, 0.08 for h1–h6). No per-step `contribution` chain — headings do not compose their level-factor multiplicatively. The sketched ratio-to-absolute trick is dropped in favor of the simpler direct base_value. |
| `trait_impl_factor` | `ImplBlock(is_trait_impl=true)` group discriminator (already is, per design §4). `contribution` to its method children is lower so trait-impl methods are deprioritized — not applied to the ImplBlock's own `base_value`. |
| `boilerplate_factor` | **`TsGroupKey::Heading` discriminator** — add `boilerplate: bool` to the Heading key. A boilerplate "License" h2 and a substantive "Design" h2 schedule independently instead of sharing one averaged `base_value`. Detection via `classify::is_boilerplate_heading` at item-classification time. |
| `generated_factor` | **FoldersGroup / FilesGroup discriminator.** Auto-generated files (codegen markers, protobuf output, mockery mocks, auto-generated API doc READMEs) form their own groups so they schedule independently from hand-written code. A `generated: bool` flag on `FilesGroup` (and propagated to the enclosing `FoldersGroup`) is the cleanest form. |
| `reexport_factor` | **`TsGroupKey::ImportedItems` discriminator** — add `reexport: bool` to `ImportedItems` (and symmetrically to `Import`). Re-exports aggregate separately from normal imports and have their own lower `base_value`. |
| `stage_value` (Name/Sig/Body/Doc priority) | Each stage is its own `TsGroupKey` variant; the priority lives in each kind's `base_value` as a per-kind constant (e.g., `FunctionSig.base_value` is lower than `FunctionName.base_value`). No per-step `contribution` chain — this is *self-value*, not inherited. |
| `private_detail_penalty` | `FunctionName(public=false).contribution` to its Sig/Body/Doc children. One modifier knob on the chain, not `base_value` shape. |
| `count_factor` | **Applies broadly**, not just to name-like groups. Every group's `base_value(items)` is sublinear in `items.len()` — start with `items.len().powf(0.75) * per_kind_constant` and tune per kind. The sub-linear shape reflects diminishing information as patterns become clear; it has no special affinity for names vs bodies. |
| `n_decay` | **Gone, no replacement.** Atomicity means a body group is all-or-nothing; line-count affects *cost* (via tokens of the rendered output), not value. Each body kind gets a single reasonable `base_value` and the line count falls out of cost naturally. |

**TsGroupKey extensions.** The design-doc sketch in §3.3 is advisory;
during implementation extend it as follows (all three are additional
discriminators driven by the table above):

- `Heading { level: u8, boilerplate: bool }`
- `Import { first_party: bool, reexport: bool }` and
  `ImportedItems { first_party: bool, reexport: bool }`
- Optionally: extend `FileRole` with variants for config / header /
  type_declaration, or keep them as separate flags on `FilesGroup`.
  Implementer's choice — the plan doesn't care whether they're enum
  variants or boolean flags, only that they partition `FilesGroup`
  groups.

**Heuristics not previously in the mapping table** (flagged by red-team
scrap audit):

| current helper | new home |
|---|---|
| `schedule::value::effective_depth` | `classify::effective_depth` (with its unit test) |
| `schedule::classify::detect_doc_site_dirs` | `classify::detect_doc_site_dirs` — called in `FoldersGroup::children()` on first folder expansion to mark descendant dirs as doc-site, which then feeds the `category` discriminator |
| `schedule::classify::detect_heading_depth` | inline inside `group/ts.rs` heading-item construction (a parser helper, not a heuristic) |
| `schedule::classify::is_boilerplate_heading` | `classify::is_boilerplate_heading` (feeds the Heading discriminator) |
| `schedule::classify::is_autogen_api_doc` | `classify::is_autogen_api_doc` |
| `schedule::classify::is_generated_file` / `is_generated_filename` | `classify::{is_generated_file, is_generated_filename}` |
| `schedule::classify::is_config_file` | `classify::is_config_file` |
| `schedule::parse::postprocess::merge_shared_line_symbols` | **Not reborn as composite symbols.** Top-level items that share a source line with other tree-sitter siblings (e.g. a rust `use foo; use bar;` on one line, or grouped JSON object keys) can gate those siblings via `dependent_siblings`, with a bounded recursion-depth cap on that dependency chain. This recovers the progressive-disclosure behavior for same-line constructs without a parallel composite-symbol data type. |

### Dependencies

- **Add:** `elsa` (^1.10) for `FrozenMap`.
- **Keep:** `clap`, `ignore`, `rayon`, `streaming-iterator`, `tiktoken-rs`,
  all `tree-sitter-*` language crates, `insta`, `criterion`, `tempfile`.
- **Remove if unused after rewrite:** audit during the simplification
  loop (Step 13). Do not touch `Cargo.lock` beyond what `cargo build`
  produces.

## Testing and benchmarks

**`tests/snapshots.rs` is rewritten** against the new public API
(`precis::render`, `precis::format::count_tokens`). The sample tests
at the top of the file stay — they iterate the language fixtures at
various budgets and call the public entry point. The helper functions
at the bottom (`render_fixture`, `render_with_budget`,
`render_with_char_budget`, `render_fixture_with_char_budget`) are
rewritten to call `precis::render` and count tokens via
`format::count_tokens` for budget assertions. The two monotonicity
tests (`budget_monotonicity_fixture`, `char_budget_monotonicity`)
become straightforward two-call comparisons through
`precis::render` + `count_tokens`. Do not preserve any helper that
only exists to hold the old `Corpus`/`FileData` surface together.

`test/fixtures.rs`, `test/perf_fixtures.rs`, and
`src/bin/clone_fixtures.rs` are unchanged.

**`benches/hot_path.rs` is rewritten** against the new pipeline.
Current bench functions (`bench_extract_symbols`,
`bench_build_file_data`, `bench_build_groups`, `bench_schedule`,
`bench_render_with_budget`) are replaced by:

- `bench_parse_store` — build a `ParseStore` and parse every file
  under a fixture (end-to-end parse path).
- `bench_schedule_small` / `bench_schedule_large` — full
  `render` at 2000 / 8000 budgets on the existing pluggy /
  commander fixtures.
- `bench_render_end_to_end` — full `render` start-to-finish
  on a larger fixture (e.g., `sps` at 8000).

Additional perf-fixture spot benches (Step 11): run `render` on
a couple of the large perf fixtures (e.g., TypeScript, Django) at
their typical budgets and record the numbers in `PERF.md`. These
don't need to be criterion-driven — a simple `time cargo run
--release` is fine for this session.

## Critical files

- **Create:** `src/store.rs`, `src/classify.rs`, `src/heuristics.rs`,
  `src/group/mod.rs`, `src/group/folders.rs`, `src/group/files.rs`,
  `src/group/ts.rs`, `tests/invariants.rs`.
- **Rewrite:** `src/lib.rs`, `src/parse.rs` (replaces `src/parse/`),
  `src/schedule.rs` (replaces `src/schedule/`), `src/render.rs`,
  `src/format.rs`, `src/bin/profile.rs`, `benches/hot_path.rs`,
  `tests/snapshots.rs` (fixture tests stay; helpers and
  monotonicity tests are rewritten against the new API).
- **Delete (Phase A, up front in Step 2):** `src/schedule/` and
  `src/parse/` directories (name-collide with new modules), and
  `src/walk.rs` (its helpers move to `classify.rs`; its
  `discover_source_files` export has no caller in the new
  architecture).
- **Delete (Phase B, inside Step 3 after its text helpers are
  distributed):** `src/layout/` directory. Its helpers are moved
  into `src/parse.rs` and `src/group/ts.rs` as described in "Text
  helpers — redistribution" below. Only after that move is complete
  does `src/layout/` get removed.
- Note: `src/render.rs` and `src/format.rs` are overwritten in
  Step 5, not deleted — do not `rm` them.
- **Minimal edit:** `src/main.rs` keeps its CLI surface and
  `CLAUDE_PLUGIN_ROOT` detection but gets a small update to call
  `precis::render` instead of the old entry points.
- **Unchanged:** `src/bin/clone_fixtures.rs`, `test/fixtures.rs`,
  `test/perf_fixtures.rs`, all `queries/*.scm`, all plugin files,
  `Cargo.toml` (only the `elsa` add).

## Execution order

**Compilation policy for Steps 1–7.** This is a single-commit rewrite.
Because several new files share names with deleted directories
(`src/parse.rs` vs `src/parse/`, `src/schedule.rs` vs `src/schedule/`),
old code must be **removed before** new code with colliding module
names is introduced. The crate will not compile between Step 2 and
Step 8 — that is expected and acceptable. Do not try to keep the crate
building at every intermediate step; the compilation gate is Step 8.

1. **Add `elsa` dependency.** `cargo add elsa`, then `cargo doc-md` to
   refresh `target/doc-md/` with the new crate's docs. Verify
   `test/fixtures/` is populated (run `cargo run --bin clone_fixtures`
   if not). Commit nothing yet — the whole rewrite is one commit at
   the end.

2. **Phase A deletions (up front).** `rm -rf src/schedule/ src/parse/`
   — these names collide with the new top-level `src/schedule.rs` and
   `src/parse.rs` and must go first. Also `rm src/walk.rs` — its
   helpers move to `src/classify.rs` (see Step 3) and its export has
   no caller in the new architecture. Do **not** yet delete
   `src/layout/`; its helpers are redistributed in Step 3, after
   which it's deleted in Phase B. Do **not** delete `src/render.rs`
   or `src/format.rs` — they will be overwritten in Step 5. Also
   delete in-file `#[cfg(test)]` modules that test removed types
   (notably any `src/lib.rs::tests::budget_monotonicity`). The crate
   does not compile from this point until Step 8 — old references in
   `src/lib.rs`, `src/render.rs`, `src/format.rs`, `src/bin/profile.rs`,
   `benches/hot_path.rs`, `src/layout/` (which depends on the
   now-gone `parse::Symbol`), and `tests/snapshots.rs` are broken
   temporarily; they will be fixed as the rewrite proceeds.

3. **Bottom-up scaffold — leaf modules.** Create and fully implement
   the leaf modules. The order below matters for the `src/layout/`
   Phase B deletion:

   - `src/classify.rs` — all migrated pure-data classifiers
     (`FileRole`, `FileCategory`, `effective_depth`, `classify_file`,
     `is_config_file`, `is_type_declaration_file`,
     `is_header_extension`, `is_generated_file`,
     `is_generated_filename`, `is_autogen_api_doc`,
     `is_boilerplate_heading`, `detect_doc_site_dirs`), plus the
     migrated unit test for `effective_depth`. Also absorbs
     `src/walk.rs`'s `is_source_file`, `is_lockfile`, and
     `is_vendored_or_fixture` as pure path predicates — these are
     called by `FoldersGroup::children()` during per-level walks.
   - `src/store.rs` — `ParseStore` with the two-method API
     (`parse` for creators, `get` for readers).
   - `src/parse.rs` — `Lang` enum, `LanguageConfig`, per-language
     query set, `TsGroupKey` enum, `TsItem<'t>` struct, an
     `extract_top_level` function that returns `Vec<(TsGroupKey,
     Node<'t>)>` per file, and the cross-kind nesting filter (D4).
     The text helpers (doc_comment_start, noise checks, etc.) live in
     `group/ts.rs`, not here — they're created in Step 4.
   - `src/heuristics.rs` — function *signatures* for every kind's
     `base_value(items, ...)` and `contribution(parent_item, ...)`,
     with `todo!()` bodies to be filled in during Step 4.
   - `src/layout/` stays on disk during this step since all its
     text helpers are migrated into `group/ts.rs` in Step 4. **Phase B
     deletion** (`rm -rf src/layout/`) happens at the end of Step 4
     once all helpers have been copied out and deleted modules no longer
     reference it. The name `src/layout/` doesn't collide with any new
     module, so there is no pressure to delete it early. The important
     thing is that it's gone before Step 8's compile gate.

4. **Group taxonomy.** Write `src/group/mod.rs` with the `Group` enum
   skeleton and its `value` / `touches_files` / `children` / `render`
   match arms, then `folders.rs`, `files.rs`, `ts.rs`. Fill in
   `children()` and `render()` for each kind (start with enough for
   Rust + Markdown to pass; extend to other languages as you go).
   Implement `heuristics.rs` function bodies in lockstep with each
   group kind so `todo!()`s disappear as soon as their first caller
   appears. Add `debug_assert!`s on construction for D2
   (kind-item match), D3 (non-empty items), D5 (spawning-chain
   conformance when detectable).

   When writing `group/ts.rs`, copy the text helpers from
   `src/layout/` into `ts.rs` as private functions near their
   callsites (see the inventory in "Text helpers — redistribution"
   above): `doc_comment_start`, `trim_doc_delimiters`,
   `docstring_end`, `skip_doc_leading_noise`, `strip_doc_line_prefix`,
   `is_markdown_leading_noise`, `is_toc_link`,
   `trim_trailing_blank_lines`, `is_config_trailing_noise`,
   `strip_heading_badges`. The `signature_end_line` family is **not**
   carried over — tree-sitter node ranges replace it entirely.
   Once all needed helpers have been copied out, **Phase B
   deletion**: `rm -rf src/layout/`.

5. **Render assembly.** Overwrite `src/render.rs` with the new
   `LineEntry` enum, the per-file cache types (`FileCache<'s>` /
   `FileEntries<'s>` / `FileCost`), the pure `render_file` function
   (used by both probe and commit), the override-resolution sort +
   debug assertion, and the final-assembly concat loop. Overwrite
   `src/format.rs` with the trimmed surface: `count_tokens`,
   `fmt_line`, `truncation_marker`, and the new shared `header_line`
   helper.

6. **Scheduler.** Write `src/schedule.rs` with the greedy loop,
   `ScheduleCtx`, `ScheduledSet`, marginal-cost probing via the
   shared `render_file`, rayon in the probe phase, `f64::total_cmp`-
   based total-order reduction, zero-cost handling, char-budget
   acceptance gating separated from prioritization, and the atomic
   commit order (remove-best → children → push → update cache →
   extend frontier).

7. **Public API and consumers.** Rewrite `src/lib.rs` to expose the
   new public surface: `render(path, budget, char_budget) -> String`
   plus a re-export of `format::count_tokens`. Internally it
   dispatches on `path.is_file()` vs `path.is_dir()`, constructs the
   `ParseStore` and `ScheduleCtx`, builds the seed frontier (a
   `FoldersGroup` for directory input, a synthetic `FilesGroup` for
   single-file input), runs `schedule`, and concatenates the cached
   per-file renders into the final output. Also in this step:
   rewrite `src/bin/profile.rs`, `benches/hot_path.rs`, and
   `tests/snapshots.rs` (the helper functions and the two
   monotonicity tests) against the new API. The old public helpers
   (`render_with_budget`, `render_file_with_budget`, `read_sources`,
   `walk::discover_source_files`, `Corpus`, `FileData`,
   `build_file_data`) are all **gone** — no compatibility shims.
   `src/main.rs` also needs a small update to call `precis::render`
   instead of the old entry points.

8. **Compile green.** `cargo build --release` and `cargo build
   --release --all-targets`. This is the first point at which the
   crate is expected to build. Fix everything that breaks. Do not
   move on until clean. If a compilation error traces to something
   the plan didn't anticipate, decide locally (per the "Stopping
   rule for smaller ambiguities" in Session preferences) and note it
   in the final summary.

9. **Snapshot regeneration (hand review, not blind acceptance).**
   `cargo test --release` will fail across the board since the rewrite
   legitimately changes outputs. Do **not** blindly run
   `INSTA_UPDATE=always`. Instead:

   - **Expected-diff classes** (don't reject these): (a) body groups
     are atomic — you'll see "full body shown" or "body omitted
     entirely", not "first 3 lines of body"; (b) doc "rest" groups
     are also atomic (either the full rest of a doc comment block
     appears, or none of it) — the current line-by-line doc growth
     is gone.
   - Run `cargo insta test --release --review` and step through the
     diffs of **five to ten** representative fixtures by hand: `mitt`,
     `pluggy`, `sps`, `anyhow`, `toasty`, `mdbook`, `cmdk`, `xlstm`
     (markdown-heavy, Python, Rust, TypeScript, large, small).
   - **Concrete rejection criteria** (fix the bug, not the snapshot):
     (1) a symbol the old output showed is gone **and** nothing of
     comparable value took its place; (2) a rendered line is
     structurally broken (wrong file header, truncated mid-word,
     overlapping numbers, off-by-one line number); (3) a debug
     assertion panics; (4) budget assertion trips; (5) a file that
     existed in the old output is missing its path entirely in the
     new output (not just its contents).
   - **Accept** stylistic/ordering differences, atomic-body/doc
     changes, and the expected-diff classes above. This is not
     calibration.
   - Once the reviewed set is clean, bulk-accept remaining fixtures
     with `cargo insta accept`. The `tests/snapshots.rs` budget
     assertions run for every fixture regardless of acceptance; any
     trip there is a cost-model bug and blocks the rewrite.

10. **Invariant-level sanity tests.** Create `tests/invariants.rs`
    with `#[test]` functions that run precis on 2-3 fixtures and
    check properties independent of snapshot content:
    - (a) no panic from R4 override assertion or D4/D5 `debug_assert!`s
    - (b) line numbers in each rendered file are strictly increasing
    - (c) every line-number prefix `"    N→"` emitted for a given
      file corresponds to a real source line
      (`N <= source.lines().count()`)
    - (d) **R1 pointer-range**: walk the `LineEntry`s produced during
      a small fixture render (hook into the renderer via a debug-only
      capture mode or a crate-internal test) and check that every
      `Complete` / `Truncated` content slice lies inside the corresponding
      source string's byte range. This is the tightest R1 check
      compatible with the preserved sanitizers.

    The budget-compliance check is already asserted inside
    `tests/snapshots.rs` for every fixture, so it is not duplicated
    here.

11. **Spot checks.** Run `cargo run --release -- test/fixtures/mitt`,
    `-- test/fixtures/pluggy`, `-- test/fixtures/sps`, and `-- .`
    (this repo). Sanity-read each for: budget respected, no panics,
    file headers present, line numbers increasing, no overlap
    asserts, no obviously missing high-value content.

12. **Simplification loop.** Invoke the `simplify` Claude Code skill
    via the `Skill` tool (`skill: "simplify"`). Iterate **two or
    three times** until the only remaining notes are non-issues or
    deferred to calibration. Track recurring themes and address
    them. This runs **before** the benchmarking step because
    `simplify` includes efficiency improvements that can move the
    bench numbers.

13. **Benchmark check.** `cargo bench --bench hot_path -- --quick`,
    plus a couple of perf-fixture end-to-end runs: `time cargo run
    --release -- test/perf-fixtures/typescript` and `time cargo run
    --release -- test/perf-fixtures/django` at their typical
    budgets. Compare the hot_path results to `PERF.md`. A 2×
    regression is acceptable for now (the old cumulative-cost trick
    was a heavy optimization); a 10× regression means the cache or
    probe loop is wrong. Record the perf-fixture numbers in
    `PERF.md`.

14. **Maintenance pass.** Check `CLAUDE.md`, `README.md`, `PERF.md`
    for statements about the old architecture that are now stale.
    Update. **Do not bump the version or publish.**

## Verification

- **Compile & test:** `cargo test --release` passes.
- **Budget compliance:** snapshot tests' built-in budget assertion
  (`assert!(tokens <= budget)`) passes for every fixture.
- **Invariant tests:** `tests/invariants.rs` passes (R1 pointer-range,
  line-number monotonicity, budget, D4/D5 overlap checks).
- **Benchmark:** `cargo bench --bench hot_path -- --quick` within 2×
  of the current `PERF.md` numbers.
- **Spot checks:** `cargo run --release -- .` on this repo produces a
  plausible overview. Manual eyeball of 3-4 diverse fixtures (`mitt`,
  `pluggy`, `sps`, `toasty`).
- **Debug-assert survival:** no panics from R4 override-resolution,
  D2 item-kind match, D3 non-empty groups, D4 cross-kind overlap
  check, D5 same-item sibling check. (A5 is enforced by the
  `ParseStore` method split, not a runtime check.)

## Stopping conditions

Stop and report to the user instead of compromising if any of the
following surface during implementation:

- **A concrete invariant from §10 provably cannot hold.** Report which
  invariant, what blocks it, and why no local fix works.
- **Tiebreaking turns out to be non-deterministic** in a way the
  `f64::total_cmp` + path/line/kind key can't repair.
- **Per-file cache cost dominates the hot loop** such that a whole-repo
  fixture takes >30s at budget 8000. Profile, identify the bottleneck,
  and escalate rather than adding a shadow cost model (which would
  re-introduce V3's exact failure mode).
- **A snapshot rejection criterion in Step 9 keeps tripping on the same
  fixture** despite fixes, and you can't localize the bug — that's a
  signal the new cost/value model is miscalibrated in a way this
  session isn't empowered to fix; report with a minimal reproducer
  and stop.

Small ambiguities (module naming, error type shape, where a heuristic
should land when two slots fit) are the implementer's call — decide
based on local clarity and move on.

**Partial implementations, "first-pass versions", or feature flags
hiding incomplete paths are not acceptable outcomes.** Either the
design is implemented end-to-end or the user gets a concrete list of
invariant violations to feed back into the next design session.

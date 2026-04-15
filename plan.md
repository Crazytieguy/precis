# Batch: TsGroupKey trait-of-types refactor + parse colocation

## Status

Stages 1–6 and 8 are landed (commit `TsGroupKey trait refactor: stages
1-6, 8 landed`). Stage 5 was dropped after an attempt showed its
motivation rested on a misreading of `filter_nested_items`. The
**heuristics.rs → calibration.rs rename** landed 2026-04-15 along
with the **skip-generated-files-at-extraction** piece of stage 7
(zero snapshot diffs). Remaining: the big stage 7 architectural
piece (ParseTsGroup pipeline + cross-file aggregation + per-kind
gating + heading nesting + DataSection routing + dedup_overloads
per-kind + dedup_line_overlaps chainer + finalize_group), which
must land as one coherent change with Agent snapshot review.

See `ignore/refactor_status.md` for landed-stage notes and constraints
that still bind future work (hand-written dispatch over `enum_dispatch`,
schedule.rs pattern gotcha, per-kind query lessons, etc.).

## Context

The 2026-04-13 architecture audit (`issues.md`) flagged `TsGroupKey` as
a god enum (#14): 40+ variants forming a partial product, with edits
required across ~9 parallel match statements per new construct. The
same audit flagged `parse/mod.rs` as a kitchen sink (#18): 1266 lines
with language-specific quirks scattered through nominally-generic
functions. Both were convergent — independently raised by Claude and
Codex.

The chosen direction:

1. Each variant becomes its own struct, colocated in symbol-family
   files under `src/group/ts/`.
2. A narrow `TsGroupKindMethods` trait owns the per-kind operations
   (`render_item`, `children`), with a separate `TsGroupKindParse`
   trait for `parse_strategy` / `from_parse`.
3. **Per-kind parsing.** Each kind owns its own tree-sitter query per
   supported language; no central classifier.
4. Cross-kind concerns reduce to **generic central infrastructure** —
   no per-kind logic outside `from_parse` and `children`.
5. Generated files are skipped entirely at extraction time.
6. **Cross-file gating disappears.** Gating becomes per-file (inside
   `from_parse`). Cross-file aggregation merges by key. The case
   "file A has only privates, file B has only publics" now produces
   visible privates instead of hidden ones. This is a deliberate
   behavioral change accepted as an expected diff in stage 7.

Stages 1–6 and 8 have landed (1); (2); (3); `render_item` and
`children` from (4). Stage 7 completes (4), (5), and (6) and makes
the cross-file gating behavior change real.

## ⚠️ Halt-and-ask discipline

If at any stage you discover that a design decision in this plan is
invalidated by the actual code — a load-bearing concern we missed, an
API constraint we didn't see, an interaction that breaks the refactor —
**stop, write up what you found, and ask before continuing.** Do not
improvise an alternative design mid-implementation. Examples:

- A normalization pass we said could move into `from_parse` turns out
  to need cross-kind context.
- Per-kind query design hits a tree-sitter limitation.
- A snapshot diff appears that isn't explained by the cross-file
  gating change and can't be traced to a refactor bug.
- Java/TypeScript fields actually appear in some snapshots today
  (believed silently dropped, not audited on every fixture).

The session has full freedom inside the design. It does **not** have
freedom to redesign on the fly when surprised.

## Selected issues closed by this batch

- **#14** — TsGroupKey god enum (primary, landed in stages 1–6/8)
- **#18** — Kitchen-sink `parse/mod.rs` + `FilesGroup::children`
  (mostly landed in stages 3/6; stage 7 finishes the files.rs
  restructure)
- **#13** — `unwrap_or("")` fallbacks in `render_item` (landed)
- **#5**  — `LineEntry::Full/Prefix` → `Complete/Truncated` in
  `design.md` (landed)
- **Stage 7 also fixes a TOML/JSON/YAML routing bug:** today's
  `parse/mod.rs` routes TOML `table`/`table_array_element` and
  JSON `pair` / YAML `block_mapping_pair` to `TsGroupKey::Heading`
  instead of the `DataSection` variants that already exist. Fix
  lands as part of the stage 7 diff set.

## Stage 7 — Skip generated, cross-file aggregation, gating removal

### Design reference

#### `ParseTsGroup` — pre-calibration group

```rust
struct ParseTsGroup<'s> {
    key: TsGroupKey,
    items: Vec<TsItem<'s>>,
    /// Sub-groups attached to this group. Populated inside `from_parse` for
    /// in-file relationships (heading nesting, private/public function
    /// gating within a single file). Calibration recurses into these.
    dependent_siblings: Vec<ParseTsGroup<'s>>,
}
```

This is `TsGroup` minus `inherited_modifier` and `cached_render`. It exists
because `from_parse` runs before cross-file aggregation and calibration; only
after those steps can we compute the final `inherited_modifier`.

The dispatcher converts `ParseTsGroup` → `TsGroup` after aggregation, in the
calibration step.

#### Outer `Group::children` (drain)

```rust
pub fn children<'a>(&'a mut self, ctx: &GroupCtx<'s>) -> Vec<Group<'s>>
where 's: 'a
{
    match self {
        Group::Folders(g) => folders::children(g, ctx),
        Group::Files(g)   => files::children(g, ctx),
        Group::Ts(g) => {
            // Drain dependent_siblings first — they were attached at parse
            // time and must be released as new frontier groups when the
            // parent is scheduled.
            let mut result = std::mem::take(&mut g.dependent_siblings);
            result.extend(g.key.children(g));
            result
        }
    }
}
```

The drain logic lives **once**, at the outer dispatch site, not in each
per-kind impl. (Already implemented in stage 8, but the dependent_siblings
field on `TsGroup` is empty until stage 7 starts populating it inside
`from_parse`.)

#### Cross-kind passes — what survives, what dissolves

| Pass | Today | New architecture |
|---|---|---|
| `is_inside_function` | central pre-filter | **stays** as `is_inside_scope_boundary` in `src/parse/ast.rs` (stage 5 dropped the wrapper-list replacement; see the §Stage 5 — DROPPED note below) |
| `filter_nested_items` | central post-pass | **stays** — already kind-agnostic (pure byte-range containment), no restructure needed |
| `extend_section_ranges` | central post-pass (heading-only) | **moves into** `Heading::from_parse`. Prerequisite: the TOML/JSON/YAML → `DataSection` routing fix. After the fix, `Heading` is produced only by markdown files, markdown files contain only heading items (grammar requires newlines between headings), so `dedup_line_overlaps` is a no-op on markdown files. The two passes then operate on genuinely disjoint file sets and their ordering is irrelevant. Without the routing fix this move would be unsafe — see `dedup_line_overlaps` notes. |
| `dedup_overloads` | central post-pass (per-kind, per-language) | **moves into per-kind `from_parse` impls** via **tree-sibling adjacency** instead of flat-stream adjacency. Two nodes are "adjacent" if they share a parent in the AST AND nothing (of any kind) sits between them in `parent.children()`. Each per-kind impl implements this check using its own tree-sitter nodes, with no need to see other kinds' items. See "`dedup_overloads` notes" below. |
| `dedup_line_overlaps` | central post-pass (drops second item) | **per-file post-pass, generic**: depth-capped sibling chainer (cap 5), runs after `dispatch_kinds` for each file but BEFORE cross-file aggregation. See below. |
| Module-doc detection | special pre-query step at `parse/mod.rs:44` | **already moved** in stage 6 into `ModuleDocFirst::from_parse` (SourceOnly strategy) |
| Cross-file aggregation | implicit in `FilesGroup::children` bucketing by `(key, is_generated)` | **stays central, generic**: merge by `key`, recursively merge `dependent_siblings`. See below. |
| File-level gating loop (`is_gated_by` + same-generated preference) | `src/group/files.rs:79-100` | **deleted**. Cross-file gating is gone (deliberate). In-file gating is produced inside `from_parse` as `dependent_siblings`. |
| `nest_heading_groups` | `src/group/files.rs:113-205` | **moves into** `Heading::from_parse`. Per-individual-heading parent selection (not per-group), so the boilerplate-tie problem dissolves. |
| `compute_item_modifier` | `src/group/files.rs:209-260` | **moves into** `calibration.rs`. Same semantics: applied at modifier construction, propagates to children via `inherited_modifier`. |

**Net result:** every cross-kind concern is either fully generic central
infrastructure (acceptable per the user's criterion) or per-kind logic inside
`from_parse`. There is no per-kind logic outside `from_parse` and `children`.

#### File-level pipeline

`FilesGroup::children` becomes (sketch — actual lifetimes will use
store-owned data):

```rust
pub fn children<'s>(g: &mut FilesGroup, ctx: &GroupCtx<'s>) -> Vec<Group<'s>> {
    let per_file_groups: Vec<Vec<ParseTsGroup<'s>>> = g.items.iter()
        .filter_map(|path| {
            let source: &'s str = ctx.store.read_source(path);
            // Generated detection needs source + role + path
            let role = file_role(path);
            let relative = ctx.rel_path(path);
            if classify::is_generated_file(source)
                || classify::is_autogen_api_doc(source, role)
                || classify::is_generated_filename(relative)
            {
                return None;
            }
            let config: &'s LanguageConfig = ctx.store.config_for(path);
            let tree: &'s Tree = ctx.store.parse(path);
            let lines: &'s [&'s str] = ctx.store.lines(path);  // may need new store API
            let extras = build_file_ctx_extras(config, tree, source, lines);
            let file_ctx = FileCtx {
                config,
                lang: config.lang,
                display_path: ctx.store.intern_path(relative),  // root-relative
                source_path: path,                              // raw walked
                source,
                lines,
                extras,
            };
            let mut groups = dispatch_kinds(tree, &file_ctx);
            // Per-file pass order. The only deliberate behavior change
            // is that line-overlap "drop" becomes "chain". Heading
            // nesting AND range extension now happen inside
            // Heading::from_parse (safe because markdown files have no
            // non-heading items, so line-overlap pass is a no-op on them).
            // dedup_overloads is also gone as a central pass — it's
            // decomposed per-kind via tree-sibling adjacency inside
            // FunctionName::from_parse and similar.
            apply_dedup_line_overlaps_chainer(&mut groups);
            Some(groups)
        })
        .collect();

    let aggregated = aggregate_across_files(per_file_groups);

    // parent_modifier is the FilesGroup's inherited_modifier, which the
    // Folders group already computed for us. No new helper needed.
    aggregated.into_iter()
        .map(|pg| calibration::finalize_group(pg, g.inherited_modifier))
        .map(Group::Ts)
        .collect()
}
```

Stages, in order:
1. Skip generated files
2. Parse each remaining file → tree + per-language pre-pass `FileCtx`
3. `dispatch_kinds` → per-kind `from_parse` → `Vec<ParseTsGroup>` per file.
   Inside `from_parse` impls:
   - `Heading::from_parse` extends section ranges and nests headings
   - `FunctionName::from_parse` / similar do tree-sibling-adjacency
     overload dedup
4. **Per-file** depth-capped sibling chainer for line overlaps (cap 5)
5. Aggregate across files (generic merge by key)
6. Calibrate: convert `ParseTsGroup` → `TsGroup` with `inherited_modifier`
   set, recursing through dependent_siblings (generic)

**Every step except (3) is fully generic — no per-kind logic.** The
architecture achieves the user's criterion: per-kind logic is confined to
`from_parse` (and `children`, `render_item`), and everything outside those
methods is kind-agnostic infrastructure.

### Skip generated files at extraction — LANDED 2026-04-15

`FilesGroup::children` now bails on generated files before parsing:
filename check first, then source-content check via the new
`ParseStore::read_source`, then `parse()` only for survivors.
`is_generated` is gone from the bucket key, from
`compute_item_modifier`, and `GENERATED_FACTOR` is deleted. The
gating-attach loop simplified (no same-generated preference).
`(Group, bool)` tuple plumbing through `nest_heading_groups` removed.

Zero snapshot diffs across all 141 fixtures: the existing
same-generated-preference logic in the old gating loop already
prevented non-generated items from being trapped behind generated
parents in every fixture, so removing it surfaced nothing.

### Cross-file aggregation

After every file's `dispatch_kinds` produces `Vec<ParseTsGroup>`, a generic
aggregator merges across files. The merge rule: for groups with the same
`key`, concatenate `items` and recursively merge `dependent_siblings`.

```rust
use std::collections::btree_map::{BTreeMap, Entry};

fn aggregate_across_files<'s>(
    per_file: Vec<Vec<ParseTsGroup<'s>>>,
) -> Vec<ParseTsGroup<'s>> {
    let mut by_key: BTreeMap<TsGroupKey, ParseTsGroup<'s>> = BTreeMap::new();
    for file_groups in per_file {
        for g in file_groups {
            insert_or_merge(&mut by_key, g);
        }
    }
    by_key.into_values().collect()
}

fn insert_or_merge<'s>(
    map: &mut BTreeMap<TsGroupKey, ParseTsGroup<'s>>,
    g: ParseTsGroup<'s>,
) {
    match map.entry(g.key.clone()) {
        Entry::Occupied(mut e) => merge_into(e.get_mut(), g),
        Entry::Vacant(e)       => { e.insert(g); }
    }
}

fn merge_into<'s>(dest: &mut ParseTsGroup<'s>, src: ParseTsGroup<'s>) {
    dest.items.extend(src.items);
    // Recursively merge dependent_siblings by key, moving (no clone)
    let mut sibling_map: BTreeMap<TsGroupKey, ParseTsGroup<'s>> = BTreeMap::new();
    for s in dest.dependent_siblings.drain(..) {
        insert_or_merge(&mut sibling_map, s);
    }
    for s in src.dependent_siblings {
        insert_or_merge(&mut sibling_map, s);
    }
    dest.dependent_siblings = sibling_map.into_values().collect();
}
```

Fully generic — no kind awareness, no field inspection. Just merge by key.

**No `Clone` needed on `ParseTsGroup` or `TsItem`.** Aggregation moves
values throughout. The only `.clone()` is on `g.key` (a `TsGroupKey`,
which already derives `Clone` for cheap discriminant + small-field copies).
`Vec::extend` and `Vec::drain` move element ownership. `TsItem` does not
need a `Clone` derive — today's code uses an explicit `clone_ts_item`
helper for the cases that genuinely need cloning (per-item duplication in
`children()` spawn logic), and that helper carries forward unchanged into
the new architecture.

Lives in `src/group/aggregate.rs` (new file).

### Same-file gating moves into per-kind `from_parse`

The current loop at `src/group/files.rs:79-100` does *both* same-file
and cross-file gating. We want to lose only the cross-file part. For each
kind that today has gating relationships (`is_gated`/`is_gated_by` at
`src/group/ts.rs:187` and `:214`):

- **Function/Method** kinds: `FunctionName::from_parse` partitions the
  captures it produces by `(documented, public)` into separate buckets,
  then attaches the private-bucket groups as `dependent_siblings` of the
  public-bucket group from the same file. (Cross-file: a private from
  file A can no longer attach to a public from file B — that's the
  intended cross-file removal.)
- **Struct/Enum/Class/Interface/Trait/TypeAlias/Const/Macro** kinds: same
  pattern as Function — partition by visibility, attach private as
  dependent_sibling of public *within the same file*.
- **Import** kinds: gate third-party imports as dependent_siblings of
  first-party imports of the same `reexport` value, within the same file.
- **ImplBlock**: trait impls are gated behind inherent impls of the same
  type, within the same file. (Note: per the user's discussion note,
  trait ImplBlock should ultimately not be gated at all — but that's
  deferred as a snapshot-changing cleanup outside this batch.)

Per-kind impls reuse the same shared helper for the partition-and-attach
pattern (lives in `src/group/ts/spawn.rs` or similar — same file as the
cross-kind helpers from stage 8).

Delete the file-level gating loop at `src/group/files.rs:79-100`.

### Heading nesting moves into `Heading::from_parse`

Per-individual-heading parent selection (not per-group), so the
boilerplate-tie problem dissolves. The heading nesting now lives inside
the kind that owns headings. Delete `nest_heading_groups` at
`src/group/files.rs:113-205`.

### TOML/JSON/YAML → `DataSection` routing fix

Today's `classify` function at `src/parse/mod.rs:418-447` routed four
different node kinds to `TsGroupKey::Heading`:

```rust
"atx_heading" | "setext_heading" => Some(Heading { ... }),     // markdown
"table" | "table_array_element"  => Some(Heading { ... }),     // TOML  -- BUG
"pair" | "block_mapping_pair"    => Some(Heading { ... }),     // JSON/YAML -- BUG
```

(Stage 6 preserved this routing via a combined `Heading::parse_strategy`
that branches on `ctx.lang`; the bug still exists, waiting for the fix.)

The TOML/JSON/YAML cases should produce `DataSection` / `DataSectionBody`
(variants which already exist in `TsGroupKey` with appropriate values).
Under the refactor:

- `Heading::parse_strategy` returns `Query(MARKDOWN_HEADING_QUERY)` only
  for `Lang::Markdown`. All other languages return `None`. `Heading` is
  markdown-only after the fix.
- `DataSection::parse_strategy` returns `Query(TOML_TABLE_QUERY)` for
  `Lang::Toml`, `Query(JSON_PAIR_QUERY)` for `Lang::Json`,
  `Query(YAML_PAIR_QUERY)` for `Lang::Yaml`, and `None` elsewhere.
- **`DataSection` and `DataSectionBody` gain a `level: u8` field.**
  Today's `DataSection` / `DataSectionBody` are unit variants with flat
  base values (`0.8` / `0.3`). Today's TOML routes through `Heading`
  with a level computed from dot-count, getting level-based decay
  (`level 3 → 0.15`). If we moved TOML to a fieldless `DataSection`,
  deep TOML tables would jump from 0.15 to 0.8 — a significant value
  change beyond the intended routing fix.

  To keep the routing fix as close to a pure kind-rename as possible,
  add `level` to both DataSection variants:
  ```rust
  struct DataSection { level: u8 }       // was unit
  struct DataSectionBody { level: u8 }   // was unit
  ```
  **Value-curve decision: replicate today's `Heading`/`HeadingBody`
  level-based values exactly for `DataSection`/`DataSectionBody`.** In
  `calibration.rs`:
  ```rust
  DataSection { level } => match level {
      1 => 1.0,   // same as today's Heading { level: 1 }
      2 => 0.6,
      3 => 0.15,
      _ => 0.08,
  },
  DataSectionBody { level } => match level {
      1 => 1.2,
      2 => 0.5,
      3 => 0.1,
      _ => 0.05,
  },
  ```
  Today's flat `0.8 / 0.3` values are discarded — but they were unused
  until now because nothing produced `DataSection` items (TOML/JSON/YAML
  routed through `Heading`, and `DataSection` had no query matcher of
  its own). The routing fix makes them reachable, and copying the
  Heading curve means **TOML snapshots should show minimal value
  shifts**. The `boilerplate_heading` factor doesn't come into play
  because TOML/JSON/YAML captures always set `boilerplate: false` today,
  and `documented_contribution` already treats both `Heading` and
  `DataSection` as inherently documented. Expected diffs are strictly
  limited to the mechanisms in Snapshot Policy category 4 (kind name,
  body rendering, TOML body splitting).

  JSON and YAML items always have `level: 1` (the grammars produce flat
  key-value structures), so they get the same `level 1` value as today's
  Heading — `1.0 / 1.2`. Same as today.

  This makes the TOML/JSON/YAML routing fix a **targeted bugfix**, not a
  calibration overhaul. Any stage 7 diffs in TOML/JSON/YAML fixtures
  should be limited to what the kind rename and modifier-function
  differences produce, not a wholesale value shift.
- `DataSection::from_parse` walks captures and produces `DataSection`
  items with the right level (TOML dot-count, JSON/YAML flat level 1).
- `DataSection::children` (the spawn logic — the body-renderer is
  `DataSectionBody`) must preserve today's TOML per-item body splitting
  from `Heading::children` at `src/group/ts.rs:405-437`. Today's TOML
  path partitions items and emits one `HeadingBody` per TOML item; the
  new `DataSection::children` should emit one `DataSectionBody` per TOML
  item, matching that behavior. For JSON/YAML, emit a single
  `DataSectionBody` with all items (matching today's `DataSection`
  behavior at `src/group/ts.rs:439-448`). Stage 7 ports this logic from
  the `Heading` branch to the `DataSection` branch.
- **Port `HeadingBody::render_item`'s body-start and noise handling**
  (today at `src/group/ts.rs:976`) into `DataSectionBody::render_item` —
  today's `DataSectionBody::render_item` starts at `start_line + 1`
  without the markdown-safe behavior, so diffs would appear unless
  the behavior is brought into alignment.

After the fix, `Heading` is markdown-only (just like the prose semantics
imply), and `extend_section_ranges` can move into `Heading::from_parse`
safely.

**Snapshot impact**: TOML/JSON/YAML fixtures may show narrow diffs at
stage 7, but each one must be tied to a specific mechanism (kind
routing, body rendering, body splitting) — see Snapshot Policy category
4 for the exhaustive list. A data-file fixture diff that doesn't match
one of those mechanisms is a real regression and halts.

### `extend_section_ranges` moves into `Heading::from_parse`

Safe now because after the routing fix, `Heading` is markdown-only and
the ordering hazard is genuinely irrelevant (markdown and non-markdown
files are disjoint from the line-overlap pass's perspective).

### `dedup_overloads` decomposes per-kind via AST-sibling adjacency

`dedup_overloads` (`src/parse/mod.rs:1080`) has two behaviors today:

1. **Adjacent same-name/same-kind drop**: scan the full per-file item
   stream in source order; if two consecutive items have the same name and
   same kind discriminant, drop the previous one. Catches C decl+defn pairs
   and TS overload signatures.
2. **Set dedup for type declarations**: for `StructName`/`EnumName`/
   `TypeAliasName`/`MacroName`, dedup all duplicates by `(name, kind)` over
   the whole stream. Catches Rust cfg'd dupes.

**Both behaviors decompose per-kind cleanly using tree-sibling
adjacency instead of flat-stream adjacency.** The insight: today's "adjacent
in the full source-ordered stream across all kinds" check is equivalent to
"siblings in the AST with nothing between them in `parent.children()`"
— and the AST sibling check works without needing to see other kinds' items,
because we check the AST directly.

Walking through the cases:

- **C decl+defn**: `int foo(int);` then `int foo(int x) { ... }` — both
  siblings under `translation_unit`. The decl and defn are AST-adjacent.
  `FunctionName::from_parse` (which owns both) checks its function_item
  nodes for AST adjacency and drops the decl.
- **TS overload signatures**: three `function foo(...)` siblings under
  `program`. AST-adjacent. Per-kind check catches consecutive ones.
- **Rust cfg dupes**: `#[cfg(unix)] fn foo() {}` + `#[cfg(windows)] fn foo() {}`
  — in the Rust tree-sitter grammar, attributes are children of the
  function_item (not siblings of it), so the two function_items are
  sibling children of `source_file` with nothing between them.
  AST-adjacent. Per-kind check catches it.
- **Disambiguation**: `fn foo / const X / fn foo` — three siblings under
  `source_file`. The two `fn foo`s are NOT adjacent in `parent.children()`
  because `const X` sits between them. Tree-sibling check correctly does
  NOT dedup. Matches today's flat-stream behavior.

**Known semantic difference from today's flat-stream check** — the user
chose the AST-sibling approach over flat-stream preservation, on the
reasoning that today's flat-stream behavior is a coincidence of "what
happens to be a captured item" rather than a designed semantic. The
approaches diverge on edge cases like:

- Two same-name functions separated by a line comment in a grammar where
  comments appear as sibling AST children (not extras). Today's flat-stream
  check: dedups them (comment isn't captured). AST-sibling check: does
  NOT dedup (comment is an intervening sibling).
- Two same-name functions separated by an uncaptured construct (e.g., a
  macro invocation we don't classify).

These cases are **rare** in realistic code, but may exist in fixtures. If
they do, stage 7 will produce snapshot diffs. The diffs fall into the
already-accepted stage 7 category 5 (`dedup_overloads` flat-stream →
AST-sibling semantic change). The stage 7 Agent review handles them. If
they're widespread or produce clearly-wrong output, reconsider the
approach (revert to central flat-stream as a one-off exception).

**Implementation** (sketch, inside each affected kind's `from_parse`):

```rust
impl TsGroupKindParse for FunctionName {
    fn from_parse<'s>(
        matches: &[OwnedQueryMatch<'s>],
        ctx: &FileCtx<'s>,
    ) -> Vec<ParseTsGroup<'s>> {
        if !matches!(ctx.lang, Lang::Rust | Lang::C | Lang::Cpp
                               | Lang::TypeScript | Lang::Tsx) {
            // Extract normally, no overload dedup for this language
            return extract_functions(matches, ctx);
        }

        // Extract all candidate function nodes with their names
        let mut candidates: Vec<(Node, &str)> = matches.iter()
            .filter_map(|m| {
                let symbol = m.capture("symbol")?;
                let name = m.capture("name")?.utf8_text(ctx.source.as_bytes()).ok()?;
                Some((symbol, name))
            })
            .collect();

        // For each pair, check AST sibling adjacency: drop the earlier
        // one if they share a parent and nothing else lies between them
        // in parent.children(). See `is_ast_adjacent` helper.
        let mut drop: HashSet<usize> = HashSet::new();
        for i in 0..candidates.len() {
            for j in (i+1)..candidates.len() {
                let (a_node, a_name) = candidates[i];
                let (b_node, b_name) = candidates[j];
                if a_name == b_name && is_ast_adjacent(a_node, b_node) {
                    drop.insert(i);
                }
            }
        }
        // ... build ParseTsGroups from surviving candidates
    }
}
```

`is_ast_adjacent(a, b)` is a free helper in `src/parse/ast.rs`: returns
true iff `a.parent() == b.parent()` AND no other child of that parent has
a byte range between `a.end_byte()` and `b.start_byte()`. Fully generic
across kinds.

For behavior (2) — set dedup for type declarations — each affected kind
(`StructName`, `EnumName`, `TypeAliasName`, `MacroName`) does a simple
"dedup by name within my own captures" inside its `from_parse`. No
adjacency check needed, no cross-kind awareness needed.

**This is the only user-design-driven exception to the flat pipeline: per-kind `from_parse` impls may need the `is_ast_adjacent` helper for overload
dedup.** The helper itself is generic — lives in `parse/ast.rs`, not
tied to any kind.

Delete the central `dedup_overloads` function entirely. (The nested
`flat_view` helper still exists for the line-overlap chainer, but it's
not used by any overload-dedup logic.)

### `dedup_line_overlaps` — depth-capped sibling chainer

Today's behavior: drops the second item that overlaps a previous item's line
range (e.g., Lua's `local x = {}; x.foo = bar`).

New behavior: instead of dropping, attach the second item as a
`dependent_sibling` of the first. Cap chain depth at **5** to prevent
minified-line worst-case ballooning. Items beyond the cap are dropped (matches
today's behavior for those positions).

The depth cap is fully generic — no per-kind logic — and the worst-case
symptom (minified line truncated early) is acceptable.

**Important: this runs per-file, before cross-file aggregation.** Two
unrelated symbols in different files that happen to share a `start_line`
must NOT be chained together. The chainer is invoked for each file's
`dispatch_kinds` output, then the per-file results are merged across files
by the aggregator.

**Item-level mechanics**. The chainer runs over a nested flat view of all
items across direct groups and dependent_siblings:

```rust
/// Stable address of an item inside a nested ParseTsGroup tree.
/// `group_path` is the sequence of indices to navigate through
/// `dependent_siblings` from the root; the empty path means a top-level
/// group. `item_idx` indexes into that group's `items`.
type ItemLoc = (Vec<usize>, usize);

fn flat_view<'s, 'a>(
    groups: &'a [ParseTsGroup<'s>],
) -> Vec<(ItemLoc, &'a TsItem<'s>, &'a TsGroupKey)> {
    let mut out = Vec::new();
    fn walk<'s, 'a>(
        groups: &'a [ParseTsGroup<'s>],
        prefix: &mut Vec<usize>,
        out: &mut Vec<(ItemLoc, &'a TsItem<'s>, &'a TsGroupKey)>,
    ) {
        for (gi, g) in groups.iter().enumerate() {
            prefix.push(gi);
            for (ii, item) in g.items.iter().enumerate() {
                out.push(((prefix.clone(), ii), item, &g.key));
            }
            walk(&g.dependent_siblings, prefix, out);
            prefix.pop();
        }
    }
    walk(groups, &mut Vec::new(), &mut out);
    out
}

fn apply_dedup_line_overlaps_chainer<'s>(groups: &mut Vec<ParseTsGroup<'s>>) {
    let mut flat = flat_view(groups);
    flat.sort_by_key(|(_, item, _)| (item.start_line(), item.node.start_byte()));

    // Walk in source order. For each item that overlaps the previous
    // item's line range, schedule it for extraction:
    //   - Remove it from its source group at its ItemLoc
    //   - Wrap it in a single-item ParseTsGroup with the same key
    //   - Attach the wrapped group as a dependent_sibling of the
    //     PREVIOUS item's containing group (or extend an existing
    //     same-key dependent_sibling on that group)
    //   - Track chain depth from the original "first" item; if depth >= 5,
    //     drop instead of attach
    //
    // After scheduling, apply the moves in reverse order so ItemLoc
    // indices remain valid.
}
```

Lifting a single item out of its source group (instead of attaching the
whole source group as a dependent sibling) is what prevents over-chaining of
unrelated items that happen to share the source group with the
overlap-victim. Each individual `TsItem` is moved, not its containing kind
group.

### Calibration (`calibration.rs`, renamed from `heuristics.rs`)

`ts_base_value(key: &TsGroupKey, items: usize) -> f64` stays as one big match
on the aggregate enum, returning **base value only**. Each arm does whatever
that variant needs: table lookup, kind-specific math.

**Critical:** `visibility` and `documented` factors do **not** move into
`ts_base_value`. They stay in `compute_item_modifier` (now in
`calibration.rs`), which constructs `inherited_modifier` and propagates to
spawned children whose keys do not carry `public`/`documented`. Moving them
into `ts_base_value` would silently drop the penalty on children. Moving
them to both places would double-count.

```rust
// calibration.rs
const PRIVATE_FACTOR: f64 = 0.3;        // was visibility_contribution(false)
const UNDOCUMENTED_FACTOR: f64 = 0.5;   // was documented_contribution(false, _)
const BOILERPLATE_HEADING_FACTOR: f64 = 0.1;
const REEXPORT_FACTOR: f64 = 0.1;
// generated_contribution dies — generated files are skipped at extraction
```

(Consts already extracted in stage 9 partial landing.)

`compute_item_modifier(key, parent_modifier) -> f64` reads the relevant
factors based on the key's variant and applies them. **`is_generated` is
gone from this function** because generated files no longer reach
extraction.

#### Rename mechanics

- Rename `src/heuristics.rs` → `src/calibration.rs`.
- Update `pub mod heuristics;` → `pub mod calibration;` at
  `src/lib.rs:4`.
- Update every `crate::heuristics::` callsite (including
  `src/group/mod.rs:87`, `src/schedule.rs:161`, and anywhere else
  `cargo check` complains) to `crate::calibration::`.
- Move `compute_item_modifier` from `src/group/files.rs` into
  `src/calibration.rs`. Update its callers.
- Add `finalize_group(parse_group, parent_modifier) -> TsGroup` that
  turns `ParseTsGroup` → `TsGroup` with `inherited_modifier` computed.

#### `finalize_group` dependent-sibling semantics

Dependent siblings are *siblings*, not children — they each get their
own `inherited_modifier` computed from the same `parent_modifier` (the
file/folder-level modifier), NOT from the containing group's
`inherited_modifier`. This matches today's behavior
(`src/group/files.rs:61` computes each gated group's modifier
independently before attaching them as `dependent_siblings` at line
73). Walking `dependent_siblings` recursively in `finalize_group` is
fine, but each recursion uses `parent_modifier`, not the containing
group's already-finalized modifier.

### Restructured `FilesGroup::children` pipeline summary

```
1. Skip generated files
2. Per file: parse → dispatch_kinds (Heading::from_parse now produces
   extended-and-nested headings; FunctionName::from_parse and friends
   now do their own tree-sibling-adjacency dedup_overloads)
   → per-file dedup_line_overlaps chainer
3. Cross-file aggregate (merge by key, recursively merge dependent_siblings)
4. Calibrate (finalize_group) — adds inherited_modifier
```

**No central `extend_section_ranges` or `dedup_overloads` passes.** Both
have moved into per-kind `from_parse` impls. The central pipeline only
has the generic line-overlap chainer left (plus aggregation and
calibration).

Delete the `extract_items` legacy adapter introduced in stage 6.
`FilesGroup::children` now consumes `Vec<ParseTsGroup>` directly.

### Verification

- `cargo test-release` — expect snapshot diffs explained by one of
  the six categories in the snapshot policy below.
- Spawn an Agent to review every diff. Each must be explained by
  exactly one category; the new output must not be worse as a
  reader's experience (per CLAUDE.md). Halt if any diff is
  unexplained or any reading regression surfaces.
- `cargo bench-hot` — investigate any regression.
- `/simplify` on the resulting diff.

## Stage 9 — Calibration polish (mostly tied to stage 7)

- **Const extraction — landed.** `PRIVATE_FACTOR`,
  `UNDOCUMENTED_FACTOR`, `BOILERPLATE_HEADING_FACTOR`,
  `REEXPORT_FACTOR` are inline consts in `src/calibration.rs`.
  `GENERATED_FACTOR` deleted 2026-04-15.
- **`heuristics.rs` → `calibration.rs` rename — landed 2026-04-15.**
  All callsites updated. Doc comment updated.
- **Grounding comments — dropped** per
  `feedback_no_inferable_or_grounding_comments.md`.
- **`finalize_group` dependent-sibling semantics — pending stage 7**
  (see above).

## Stage 10 — Cleanup

- **`unwrap_or("")` in `render_*_lines` — landed.**
- **`design.md` `LineEntry::Full/Prefix` → `Complete/Truncated` —
  landed.**
- **`design.md` §3.3 / §4 trait architecture — pending.** The
  structural doc still describes the pre-refactor shape.
- **Issue follow-ups — filed** (#27 ordinal magic table, #28
  ImplBlock-trait gating, #29 Mod::children re-run idea;
  `output-issues.md` #45 Java fields silently dropped). Stage 10
  will file any additional follow-ups that surface.
- `/simplify` on the resulting diff per
  `feedback_simplify_step.md`.
- `cargo bench-hot` — investigate any regression.

## Stage 5 — DROPPED

Attempted 2026-04-15 and reverted. The plan called for a per-language
wrapper-kind list + `is_top_level` to replace both `is_inside_function`
and `filter_nested_items`. Two findings forced the revert:

1. `filter_nested_items` is already kind-agnostic — it's a pure
   byte-range containment check on captured items, with zero
   grammar-kind pattern matching. The plan's motivation for deleting
   it ("forces pattern-matching on specific grammar node kinds") was
   based on a misreading.
2. The wrapper-list approach couldn't fully replace
   `filter_nested_items` anyway (xlstm template-specialization case
   forces an edge in the kind-vs-item distinction), and for the
   case `filter_nested_items` doesn't cover — symbols inside
   *uncaptured* scope boundaries like
   `describe('x', () => { const map = ... })` callbacks — ~90 lines
   of per-language wrapper tables is more code than the existing
   ~25-line scope-boundary rejection list for the same behavior.

Net change: `is_inside_function` renamed to
`is_inside_scope_boundary` with a doc comment explaining its
complementary role to `filter_nested_items`. If revisited, rename or
inline the scope-boundary list rather than inflating to wrapper
tables.

## Snapshot policy

**Strict byte-identical at every stage EXCEPT stage 7.** Any diff
outside stage 7 is a bug.

**Stage 7** diffs are accepted when they fall into one of these six
categories. An Agent reviews every diff, reads each diffed fixture's
before/after as a fresh user, and halts if any diff is unexplained or
any reading regression surfaces.

1. **Cross-file gating removal** — privates previously hidden
   behind cross-file publics now appear directly.
2. **Generated-file skipping unhiding gated groups** — a
   non-generated gated group previously attached to a generated
   parent (invisible because parent modifier was 0) now appears
   directly.
3. **`dedup_line_overlaps` chainer change** — overlaps now appear as
   dependent_siblings instead of being dropped.
4. **TOML/JSON/YAML → `DataSection` routing fix.** Base values are
   **preserved** by copying the Heading level curve into DataSection.
   Diffs in this category must be tied to a specific mechanism, not
   accepted by file extension alone:
   - (4a) kind name appearing directly in output;
   - (4b) render-behavior differences between
     `HeadingBody::render_item` and `DataSectionBody::render_item`
     if the port isn't perfect;
   - (4c) TOML per-item body-splitting if the port produces grouping
     changes.
   - **(4d) modifier factors**: nothing should come from
     `boilerplate_heading` (always false for TOML/JSON/YAML today) or
     `documented_contribution` (both kinds are treated as inherently
     documented). If a data-fixture diff can't be tied to (4a)–(4c),
     it's likely a real regression — halt.
   
   **Scheduler auto-commit note**: today's `is_auto_commit_body` at
   `src/schedule.rs:167-170` only fires `HeadingBody { level: 1 }`
   auto-commit for README H1 bodies at the root directory (condition
   requires `FileRole::Readme`). Data files have role `Normal`, so
   they don't hit this path today even though they're keyed as
   `HeadingBody`. After the routing fix, `HeadingBody { level: 1 }`
   is exclusively markdown README bodies — which matches the
   condition's intent. `is_auto_commit_body` only needs a
   pattern-match syntax update, no semantic addition.
5. **`dedup_overloads` flat-stream → AST-sibling semantic change** —
   rare edge cases where two same-name items were separated only by
   uncaptured AST nodes (comments, etc.). Expected to be rare; halt
   if widespread.
6. **Heading nesting: per-group → per-individual parent selection.**
   Today's `nest_heading_groups` picks one parent per *group* with a
   non-boilerplate tiebreak. After the refactor, each heading picks
   its parent in source order. Example:
   ```
   # Title           # H1
   ## Setup          # H2 non-boilerplate
   ### Step 1        # H3 → picks Setup
   ## License        # H2 boilerplate
   ### Terms         # H3 → today picks Setup, new picks License
   ```
   The new behavior is more semantically correct but produces diffs
   on markdown fixtures with interleaved boilerplate headings.
   Widespread diffs may indicate a real improvement — Agent flags
   them for review.

`debug_assert!` is **not** a verification step on its own because
`cargo test-release` doesn't run debug assertions. Anywhere the plan
adds a debug assert, also rely on a positive test that would catch
the same condition.

## Excluded from this batch

- **ImplBlock-trait gating cleanup** ("trait ImplBlock should not
  be gated") — snapshot-changing; deferred. Trivial follow-up once
  gating moves into per-kind `from_parse`: ImplBlock simply doesn't
  produce dependent_siblings.
- **Architecture cluster** (#21 Group→trait, #23 childless_folders
  refund, #25 render↔group cycle, #26 SchedulerRenderer asymmetric
  surface) — coherent on its own, next batch.
- **#19 ParseStore lifecycle, #20 classification string-matching,
  #24 silent `unwrap_or` fallbacks elsewhere** — separate concerns.
- **Bugs** (#3 V4 byte budget pre-check, #4 A1 auto-commit bypass)
  — touch the scheduler/value model, not this batch.
- **Java/TS field surfacing** — silent drop is a real bug
  (documented in `output-issues.md`), but fixing it requires
  `ClassName::children` to extract fields the way it extracts
  methods. Deferred per user instruction.
- **Mod::children re-running from_parse** (user idea: re-run
  `from_parse` with a filter for nodes inside the module, no
  duplication) — out of scope, noted in `issues.md`.

## Known unknowns

- **Cross-file aggregation with mismatched dependent_sibling
  structure.** If two files produce groups with the same key but
  structurally different `dependent_siblings` (e.g., heading nesting
  trees that don't merge cleanly), the recursive merge may produce
  surprising results. Audit during stage 7 — if surprising, halt.
- **`dedup_line_overlaps` cap = 5 may be too aggressive or too lax.**
  First pick is 5; revisit if fixtures show the wrong cliff.
- **`ParseStore` source-read API.** May need `read_source(path) ->
  &'s str` separate from `parse(path) -> &'s Tree` to classify
  generated files without parsing. Verify at the start of stage 7.

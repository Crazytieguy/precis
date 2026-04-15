# Precis Rewrite — Design

A full redesign of precis's internal data model, scheduling, and rendering. The
goal is a semantically honest architecture where the data model cannot represent
invalid states, where domain-specific concerns are encapsulated by item kind,
and where the scheduler and renderer are simple because the heavy lifting is
done by the taxonomy.

This document is self-contained. It assumes familiarity with the project's
design principles (see `CLAUDE.md`) but not with the current code.

**Only the invariants in §10 are binding.** Everything else in this document —
exact type shapes, naming, module layout, API signatures, pseudocode, the
specific taxonomy sketches — is guidance or illustration. When something
outside §10 is infeasible or better done differently during implementation,
fix it in the implementation without coming back to edit this document. When
an invariant itself proves infeasible, stop and return to design.

---

## 1. Motivation

The current architecture uses one `Symbol` struct to represent functions,
structs, imports, markdown headings, and module-level docs. That struct carries
`doc_start`, `doc_end`, `sig_end`, `body_start`, `body_end`, `is_trait_impl`,
`is_first_party`, `composed_prefix_lens`, and more. Most fields are meaningless
for most kinds. The renderer has to pattern-match on kind to avoid reading
fields that don't apply. The scheduler treats everything as `(group, stage)` and
precomputes cumulative stage costs that can drift from what the renderer
actually produces.

Three concrete problems motivate the rewrite:

- **Invalid states are representable.** An `Import` symbol has a `sig_end_line`
  field. A `ModuleDoc` has a `name`. A markdown heading has a `body_start`. Some
  of these happen to work because we never read the wrong field, but the type
  system offers no protection.
- **Scheduling is at "Symbol" granularity.** This forces hacks for finer levels
  (doc lines, body lines, composite symbols) and coarser levels (folders as
  pseudo-symbols). Nesting inside a class is expressed flatly.
- **Cost is precomputed, rendering is separate.** The precomputed cumulative
  stage costs encode an assumption about what rendering will do. When the
  renderer changes, the cost model silently drifts.

The rewrite replaces the `Symbol` struct with a polymorphic group taxonomy
where each kind carries exactly the data it needs, replaces the stage-based
scheduler with a greedy tree walk, and replaces the cost model with "render and
count" backed by a per-file cache.

## 2. Glossary

**Group.** The scheduling and rendering unit. A group holds one or more items
of the same kind. Groups form a conceptual tree via the spawning process —
parents produce children via `children()` calls — but groups do not hold
references to other groups.

**Item.** The data for one "thing" inside a group: one function, one struct,
one file, one import. Items within a group are scheduled and rendered
together.

**Kind.** The classification of a group's content. Concretely, a group is one
of `Folders`, `Files`, or a tree-sitter group (see §3.3); the tree-sitter case
has many sub-variants (`FunctionName`, `StructName`, `ClassName`, `ImplBlock`,
`Heading`, `Import`, and so on). There is no single unified enum — the three
domains have distinct data needs. "Kind" is used as a conceptual term across
this document, not as a Rust type.

**LineEntry.** One contribution from one group to the render output for one
source line, in one file. Three variants: full line, line prefix (partial
content with trailing `…`), and ellipsis-only (placeholder with no content).

**dependent_siblings.** Groups that become reachable after a given group is
scheduled but are not scope-nested inside it. Example: the private-functions
group is a dependent sibling of the public-functions group. Structurally these
are handled identically to children — they are revealed by the same
`children()` call. The name captures the semantic distinction. Dependent
siblings are *owned* by construction, not referenced.

**base_value.** A group's local contribution to its own value, computed from
its items. Sublinear in item count: additional items of the same kind add
diminishing information.

**inherited_modifier.** A scalar multiplier on the group's value, set at group
construction time by multiplying the spawning parent's `inherited_modifier` by
a contribution derived from the parent item the child was spawned from.

**value.** `inherited_modifier × base_value(items)`. Used as the numerator of
the scheduler's ratio.

**cost.** The number of tokens the scheduled set would add to the output if
this group were included. Measured by actually rendering the scheduled set,
with per-file caching so that unchanged files don't re-render and don't
re-count.

**FrozenMap.** An append-only map that supports handing out stable references
across later insertions. Used via the `elsa` crate to store parsed trees and
source strings so that borrowed content remains valid.

**Seed frontier.** The initial set of groups constructed from the CLI input
path, before any scheduling decision is made.

**Tree-sitter group.** A group whose items reference tree-sitter nodes.
Distinct from `Folders` and `Files` groups, which do not.

## 3. Data Model

### 3.1 Group representation

A group's exact representation is left to implementation. What matters:

- Each group carries its items, its `inherited_modifier`, and its
  `dependent_siblings` (the gated groups revealed when this one is scheduled).
- Groups fall into three top-level categories: a `Folders` group, a `Files`
  group, or a tree-sitter group. The three have meaningfully different data
  needs (filesystem entries vs. parsed-source items), so they are modeled as
  distinct shapes.
- Polymorphism over the three categories can be handled via a plain Rust enum,
  `enum_dispatch`, or whatever fits best. This design does not prescribe.
- **Groups do not hold references to other groups.** `dependent_siblings` is
  the only exception, and it holds owned sub-groups (produced at construction
  time) rather than references. See invariant D6.

### 3.2 Folders and Files

These groups represent filesystem entries and have no tree-sitter data.

```rust
pub struct Folders {
    pub parent_dir: PathBuf,
    pub items: Vec<PathBuf>,   // child folders of parent_dir
}

pub struct Files {
    pub parent_dir: PathBuf,
    pub role: FileRole,
    pub items: Vec<PathBuf>,   // files of this role under parent_dir
}
```

`FileRole` is a classification enum (entry point, source, tests, docs, config,
etc.). Its exact variants are left to implementation — the current `classify`
module has most of this logic already.

`Folders` discriminates on `parent_dir`. `Files` discriminates on `(parent_dir,
role)`. The `parent_dir` here is a filesystem path, not a reference to any
group.

Other properties that affect modifier contribution to children (e.g. module
visibility for Rust files, or any language-specific property that differs
between items) are not required to be group-level discriminants. A
`children()` call is free to partition its items and produce multiple child
groups of the same kind with different inherited modifiers. See §8.2.

### 3.3 Tree-sitter groups

A tree-sitter group carries a group key identifying its kind plus a list of
items. Each item references one tree-sitter node and the file it came from:

```rust
pub struct TsItem<'t> {
    pub path: &'t Path,
    pub node: tree_sitter::Node<'t>,
}
```

The single tree-sitter node per item is almost always enough: the renderer can
reach doc comments, signature range, body block, and so on via tree-sitter
traversal on demand. The `path: &'t Path` is borrowed from the parse-store
keys, and the node borrows from the corresponding tree.

**Items in a tree-sitter group routinely come from multiple files.** When a
`Files` group is scheduled, all its files are parsed, and the resulting
top-level items across those files are grouped *across file boundaries*.
"All first doc-comment lines of public documented functions in all `Source`
files under `foo/`" is a single group with one item per qualifying function,
regardless of which file that function lives in. Finer per-file splitting is
only introduced when items from different files would be prioritized
differently (e.g., different inherited modifiers due to different parent-file
properties). See D7 in §10.

Sketched group-key enum (non-exhaustive; exact shape TBD in implementation):

```rust
pub enum TsGroupKey {
    // Module-level
    ModuleDocFirst,
    ModuleDocRest,

    // Imports
    Import { first_party: bool },
    ImportedItems { first_party: bool },

    // Function-like
    FunctionName { documented: bool, private: bool },
    FunctionDocFirst,
    FunctionDocRest,
    FunctionSig,
    FunctionBody,

    // Structs
    StructName { documented: bool, private: bool },
    StructDocFirst,
    StructDocRest,
    StructBody,

    // Enums
    EnumName { documented: bool, private: bool },
    EnumDocFirst,
    EnumDocRest,
    EnumBody,

    // Classes
    ClassName { documented: bool, private: bool },
    ClassDocFirst,
    ClassDocRest,
    // Methods are spawned as FunctionName groups whose inherited modifier
    // comes from the parent class chain.

    // Interfaces
    InterfaceName { documented: bool, private: bool },
    InterfaceDocFirst,
    InterfaceDocRest,

    // Rust traits
    TraitName { documented: bool, private: bool },
    TraitDocFirst,
    TraitDocRest,

    // Rust impl blocks
    ImplBlock { is_trait_impl: bool },

    // Type aliases
    TypeAliasName { documented: bool, private: bool },
    TypeAliasDocFirst,
    TypeAliasDocRest,

    // Consts and statics (merged)
    ConstName { documented: bool, private: bool },
    ConstDocFirst,
    ConstDocRest,

    // Macros
    MacroName { documented: bool, private: bool },
    MacroDocFirst,
    MacroDocRest,

    // Markdown
    Heading { level: u8 },
    HeadingBody,

    // JSON / TOML / YAML top-level and nested keys
    DataSection,
    DataSectionBody,
}
```

### 3.4 LineEntry

```rust
pub enum LineEntry<'src> {
    /// Full source line. Rendered as "NNNN→<content>".
    Full { line: u32, content: &'src str },

    /// Prefix of a source line, the rest omitted. Rendered as "NNNN→<content>…".
    Prefix { line: u32, content: &'src str },

    /// Ellipsis placeholder at a specific source line. Rendered as "    →…".
    /// The line number is used for ordering and override resolution, not
    /// for display.
    Ellipsis { line: u32 },
}
```

All content is a `&'src str` borrowed directly from the parsed source. The
renderer assembles the final output by iterating sorted entries and appending
to a single `String`; no intermediate allocations.

Override ordering by "content amount": `Ellipsis < Prefix < Full`. When two
prefixes collide at the same line, the one with more characters wins. A
`debug_assert!` enforces this at override time: a later group emitting at an
already-occupied line must strictly increase content.

### 3.5 Parse storage and lifetimes

Parsed source strings and tree-sitter trees both need owned, append-only
storage so that borrowed references remain valid across later insertions.
`elsa::FrozenMap` is a natural fit for both. A shared parse-store is
constructed at the start of a `precis` run and threaded through scheduling and
rendering as an immutable reference. All `TsItem` nodes and all `LineEntry`
content slices borrow from it.

## 4. Taxonomy

The full tree of group relationships. `→` means structural child (requires
parent scheduled first); entries after `⇢` are dependent siblings revealed
when the main group is scheduled.

```
── Folders(parent_dir)
│   └── (each folder item expands into its own Folders + Files children)
│
── Files(parent_dir, role)
    │
    ├── ModuleDocFirst
    │    └── ModuleDocRest
    │
    ├── Import(first_party=true)
    │    ├── ImportedItems(first_party=true)
    │    ⇢ Import(first_party=false)
    │         └── ImportedItems(first_party=false)
    │
    ├── FunctionName(documented=true, private=false)
    │    ├── FunctionDocFirst → FunctionDocRest
    │    ├── FunctionSig → FunctionBody
    │    ⇢ FunctionName(documented=true, private=true)
    │
    ├── FunctionName(documented=false, private=false)
    │    ├── FunctionSig → FunctionBody
    │    ⇢ FunctionName(documented=false, private=true)
    │
    ├── StructName(documented, private)
    │    ├── StructDocFirst → StructDocRest
    │    ├── StructBody         (fields, atomic)
    │    ⇢ (private variant)
    │
    ├── EnumName(documented, private)
    │    ├── EnumDocFirst → EnumDocRest
    │    ├── EnumBody           (variants, atomic)
    │    ⇢ (private variant)
    │
    ├── ClassName(documented, private)
    │    ├── ClassDocFirst → ClassDocRest
    │    ⇢ (private variant)
    │
    ├── InterfaceName(documented, private)
    │    ├── InterfaceDocFirst → InterfaceDocRest
    │    ⇢ (private variant)
    │
    ├── TraitName(documented, private)
    │    ├── TraitDocFirst → TraitDocRest
    │    ⇢ (private variant)
    │
    ├── ImplBlock(is_trait_impl=false)
    │    ⇢ ImplBlock(is_trait_impl=true)
    │
    ├── TypeAliasName(documented, private)
    │    └── TypeAliasDocFirst → TypeAliasDocRest
    │
    ├── ConstName(documented, private)       (const + static merged)
    │    └── ConstDocFirst → ConstDocRest
    │
    ├── MacroName(documented, private)
    │    └── MacroDocFirst → MacroDocRest
    │
    ├── Heading(level=1)                     (markdown only)
    │    ├── HeadingBody
    │    └── Heading(level=2)
    │         └── ... nested
    │
    └── DataSection                          (JSON / TOML / YAML)
         ├── DataSectionBody
         └── DataSection                     (nested)
```

Notes:

- **Private promotion.** If a file has no public items of a given kind but
  does have private items, the private group is promoted to be a direct child
  of `Files` rather than gated. Detection happens at group-construction time
  in the `Files::children()` call.
- **Methods as `FunctionName`.** Methods inside classes, interfaces, traits,
  and impl blocks are represented by ordinary `FunctionName` groups, spawned
  as children of the enclosing type. The modifier difference comes from the
  parent-chain `inherited_modifier`, not from a separate `MethodName` kind.
  These method children are not drawn inline in the tree above; they are a
  direct consequence of this note.
- **Trait impl methods are boilerplate.** An `ImplBlock(is_trait_impl=true)`
  contributes a lower modifier to its method children than
  `ImplBlock(is_trait_impl=false)` does. The modifier does not depress the
  impl block's own value — only its descendants'.
- **Nested items filtered at query time.** If a tree-sitter query match is a
  descendant of another match of the same kind (e.g. a nested function inside
  a function), the nested one is dropped. This preserves "function body is
  atomic" while losing the ability to surface nested functions separately.
  Accepted trade-off.
- **Plain-text / fallback files** have no children. Their file name appears in
  the output but nothing else.
- **Markdown and data-file sections are distinct kinds.** `Heading` is
  markdown-specific; `DataSection` covers JSON/TOML/YAML. Same structural
  shape but separate value heuristics.

## 5. Scheduling

### 5.1 Algorithm

The following is illustrative; specific API names are not prescribed.

```
scheduled := []
frontier := seed_groups(input_path)

loop:
    # Find the best candidate that fits the budget.
    best := None
    best_cost := None
    best_ratio := -inf
    for candidate in frontier:
        cost := marginal_cost(scheduled, candidate)
        if cost > remaining_token_budget: continue
        if char_budget_set and char_cost > remaining_char_budget: continue
        ratio := value(candidate) / cost
        if ratio > best_ratio:
            best_ratio, best, best_cost := ratio, candidate, cost

    if best is None: break

    scheduled.push(best)
    frontier.remove(best)
    frontier.extend(best.children(parse_store))
    remaining_token_budget -= best_cost

return assemble_output(scheduled)
```

- **Frontier evaluation** can be parallelized with `rayon`: each candidate's
  cost probe against the current `scheduled` set is independent. The chosen
  candidate is applied serially.
- **Deterministic tiebreak.** When two groups have equal `value / cost`, the
  scheduler picks the one with the earliest path (lexicographic) and earliest
  source line within a file. Stability is required for reproducible output.
- **`children(parse_store)`** is called exactly once per scheduled group. It
  may parse files (when invoked on a `Files` group), construct new groups,
  compute their `inherited_modifier` values, and return them for addition to
  the frontier.

### 5.2 Seed frontier construction

- **Input is a directory.** Seed consists of one `Folders` group for the
  directory (since `Folders` only discriminates on `parent_dir`, there is
  exactly one) plus one `Files` group per role found among its immediate
  file children.
- **Input is a file.** Seed consists of the groups that would be children of a
  `Files` group containing just that file. The file name is not rendered —
  context is implicit from the CLI argument.

### 5.3 Laziness

Parsing happens only inside `Files::children()` calls. A single such call may
parse multiple files when that's what classification requires (for example, a
Rust file plus its parent module to determine public-vs-private module
visibility). This does not require a separate invariant — it's what "inside a
children() call" already means.

Group construction is equally lazy: a group's children are built only when the
group is scheduled. Nothing beyond the seed frontier is eagerly expanded.

### 5.4 `children()` contract

`children()` is responsible for all partitioning logic and all inherited
modifier assignment for the groups it produces. Each kind of group has its
own `children()` behavior tailored to its data. Because modifier computation
happens per-item inside `children()`, there is no separate
`descendant_value_modifier` method — partitioning and modifier assignment
are unified.

Note that `children()` is *not* the only place heuristics live: each group
also has a `base_value()` function (see §9) that computes its local value
from its items. `children()` handles what happens *between* groups;
`base_value` handles a group's own contribution.

`dependent_siblings` is a property of each returned group. When a group is
constructed, any siblings that should be unlocked by its scheduling are
placed in this field. When that group is later scheduled and `children()` is
called on it, the implementation empties `dependent_siblings` into the
return value alongside any newly-computed structural children. The scheduler
has nothing to distinguish — a returned group is a returned group.

### 5.5 Termination

The scheduler halts when no frontier group fits in the remaining budget.
There is no minimum-value-density floor — scheduling stops only when nothing
can be added.

## 6. Rendering

### 6.1 Per-group `render()` contract

This section covers *tree-sitter groups only*. `Folders` and `Files` render
as path strings, not `LineEntry` values — see §6.2 for their rendering.

Each tree-sitter group produces, per file it touches, a list of `LineEntry`
values. The contract:

- **Context-free.** `render()` never inspects the scheduled set or any other
  group. It emits entries as if no child group would ever be scheduled. If a
  parent group wants to indicate "more content below," it emits an
  `Ellipsis` entry at the relevant line. If a child is later scheduled, its
  entries will override that line.
- **Local ellipsis decisions.** A group decides whether to emit an ellipsis
  based on its own data, not on what's scheduled. Example: `FunctionDocFirst`
  emits one content line plus an `Ellipsis` entry if and only if the
  underlying doc block has more than one line. That is local information from
  the tree-sitter node.
- **`FunctionName` alone.** Emits one `Prefix` entry on the function's
  declaration line: the `fn foo` prefix with the trailing `…` added at render
  time.
- **`FunctionSig` alone.** Emits one or more `Full` entries for each line of
  the signature, from the declaration line through the line containing `{`
  inclusive.
- **`FunctionBody` alone.** Emits `Full` entries for every line strictly after
  the signature's closing brace line through the line containing the matching
  `}`.
- **`Import` alone.** Emits a `Prefix` entry for the import statement's
  leading portion (module path) with a trailing `…`.
- **`ImportedItems`.** Emits a `Full` entry for the full import statement,
  which overrides the prefix emitted by `Import`.

### 6.2 Override and assembly

After scheduling, the renderer assembles the final output:

```
output := empty string
for (path, entries) in per-file emissions:
    output.push(file_header(path))
    for entry in sorted by (line, scheduling-order):
        output.push(format_entry(entry))
```

- **File header.** The file's path, shown once before its emissions.
- **Per-file sorting.** Entries are sorted by source line. Line-number gaps in
  the output reflect source-line gaps naturally; no ellipsis marker is
  inserted between independently-scheduled items.
- **Override resolution.** For same-`(file, line)` entries, the later
  scheduling order wins — the stable sort preserves this.
- **Per-file render cache.** Rendering and token counting are both cached
  per file. When a new group is scheduled (or probed as a candidate), only
  files that group touches have their per-file render and their per-file
  token count invalidated and recomputed. Final concatenation of per-file
  outputs happens once, after scheduling terminates. Per-file files and
  folders are separated by an empty line in the final output so that
  token-counting the pieces independently does not produce misleading
  joins.
- **Folders and Files rendering** is a separate path: folder and file items
  render as path strings (e.g. `tests/`, `src/main.rs`), not as `LineEntry`
  sequences. A folder disappears from the output if any file inside it
  contributes emissions (the file path suffices). A file header appears
  exactly once when any of its contents are scheduled, or when the file is
  shown as a name-only entry.

### 6.3 Output format

Same as current precis:

```
path/to/file.rs
     10→fn foo() {
     11→    bar();
     12→}
     14→fn other_thing …
```

Line numbers in source order. Ellipsis lines print as `      →…` (no line
number). Prefix entries append `…` to the content.

## 7. Parsing

### 7.1 Tree-sitter queries

Each tree-sitter group kind owns its per-language query strings, colocated
with its `TsGroupKindParse` impl in `src/group/ts/<family>.rs`. At parse
time, the dispatcher concatenates every kind's query for the current
language into one combined `tree_sitter::Query`, walks the tree once, and
buckets matches back to their originating kind by `pattern_index`.

### 7.2 Nested-item filtering

Nested items (e.g., a function defined inside another function) should be
filtered out of the top-level item pool. A post-query pass drops matches
whose captured `@symbol` node has a same-kind `@symbol` ancestor. Behavior
must be consistent across languages.

### 7.3 Classification

Per-file properties are computed at whichever stage needs them. Properties
that are *group-level discriminants* (used to split items into different
groups at construction time) must be computed before those groups are
created. Properties that only affect inherited modifiers passed to children,
without changing the group structure, can be computed later.

Concretely: `FileRole` is a group-level discriminant on `Files`, so it is
computed during `Folders::children()` from path and filename alone (no
parsing needed). Rust module visibility (`pub mod foo` vs `mod foo`) affects
the modifier that files contribute to their own children, but is not used to
split `Files` groups themselves; it is computed during `Files::children()`,
when the file is being parsed anyway, and may require parsing the parent
module to see the `mod` declaration.

This split is a guideline, not a rule. An implementation is free to move
classification earlier or later as long as nothing depends on unavailable
information.

## 8. Value and Cost

### 8.1 Value formula

```
value(group) = inherited_modifier × base_value(items)
```

- `inherited_modifier: f32` is set at group construction time.
- `base_value(items) -> f32` is a function of the items. Sublinear in
  `items.len()`: each additional item contributes less than the previous one.

### 8.2 Modifier composition

When a group is constructed inside a parent's `children()` call, its
`inherited_modifier` is computed as:

```
child.inherited_modifier = parent.inherited_modifier × contribution(parent_item)
```

`contribution(parent_item)` is computed per-parent-item, inside the parent's
`children()` implementation. Because different parent items may contribute
different modifiers, a single parent's `children()` call is free to produce
multiple child groups of the same kind — each with its own
`inherited_modifier`, partitioned by which subset of parent items contributed
to it. This is how, for example, a `Files` group with mixed public and
private modules produces distinct `FunctionName` child groups for each
visibility bucket, without visibility being a discriminant on `Files` itself.

Partitioning logic and modifier logic live together in `children()`. This
replaces any separate `descendant_value_modifier` method: they cannot drift
from each other because they are the same code.

### 8.3 Cost

Cost is measured by rendering:

```
marginal_cost(candidate | scheduled)
    = tokens(render(scheduled + candidate)) - tokens(render(scheduled))
```

- **Per-file render cache.** The render cache holds assembled output per file.
  When a candidate is probed, only files that candidate touches need to
  re-render; unchanged files' token counts are reused.
- **Token counting** uses `tiktoken-rs` with the `o200k_base` encoding.
- **No parallel cost model.** Cost is derived from the same rendering the
  final output uses. There is no precomputed cumulative cost that could drift
  from actual output.

### 8.4 Budget gates

- **Token budget (`--budget`).** Authoritative. A candidate is rejected if
  including it would push total tokens over this limit. Used for
  prioritization.
- **Byte budget.** Fast pre-check before running the token counter, to
  cheaply prune candidates that can't possibly fit.
- **Char budget (`--char-budget`).** Optional second acceptance gate. When
  set, a candidate must fit both the token budget and the char budget. The
  binary detects when it is running under the Claude Code plugin and
  overrides the default to enable the char budget in that context. Only
  token cost is used for prioritization.

## 9. Heuristics location

All per-kind value functions and modifier contribution functions should live
in one findable location, so that the calibration sweep (§12) can reference
every heuristic in a single place. The exact module layout and API are left
to implementation.

## 10. Invariants

This is the core of the design. **The invariants in this section are the
only hard constraints on the rewrite.** Everything else in this document is
guidance or illustration; if a conflict arises during implementation, the
invariants win and the other text should be treated as advisory. If an
invariant itself proves infeasible, stop and return to design rather than
compromising around it.

### Data model (structural)

- **D1 — Tree shape.** Each group belongs to exactly one spawning parent (or
  is in the seed frontier). The spawning relationships form a tree.
- **D2 — Item co-scheduling.** All items in a group are scheduled and
  rendered together as a unit. Items share the group's kind and
  `inherited_modifier`. Items may differ in what they contribute when
  spawning their own children.
- **D3 — No empty groups.** A group with zero items is never constructed.
- **D4 — Non-overlapping top-level items.** Within a single file, the
  top-level items extracted from it (after nested-item filtering at query
  time) have non-overlapping source line ranges.
- **D5 — Within-item containment.** Within the group sub-tree rooted at a
  single item, if two groups overlap at the same source line, they must have
  a parent/descendant relationship in the group tree. Two sibling groups
  belonging to the same item never share a source line.
- **D6 — No cross-group referencing.** A group's data holds no references or
  IDs of other groups. `dependent_siblings` is the only mechanism that
  carries other groups, and it owns them rather than referencing them. The
  tree structure exists in the scheduler's history, not in the data.
- **D7 — Cross-file aggregation.** Tree-sitter groups aggregate items across
  all files in their scope (i.e., across every file in the parent `Files`
  group). A single tree-sitter group's items may come from many different
  files. Items are split into separate groups only when they would be
  prioritized differently — for example, when a grouping discriminant
  differs (documented vs. undocumented) or when parent items contribute
  different inherited modifiers to them. Per-file groupings are not the
  default and should not be introduced for structural reasons alone.

### Scheduling (algorithmic)

- **A1 — Greedy choice.** At each step, the scheduler picks the frontier
  group with the best `value / cost` ratio that fits the remaining budget.
  Ties broken deterministically by path and source-line order.
- **A2 — Parent-before-child.** A child group (including dependent siblings)
  enters the frontier only after its parent has been scheduled.
- **A3 — Atomicity.** Scheduling a group commits all of its items to the
  output. A group is never partially scheduled.
- **A4 — Termination.** Scheduling halts when no frontier group fits the
  remaining budget. There is no density floor.
- **A5 — Lazy parsing.** Parsing happens only inside `Files::children()`
  calls.

### Rendering

- **R1 — Source fidelity.** Every `LineEntry::Complete` and `LineEntry::Truncated`
  holds a `&str` that is a slice of the original parsed source. `Ellipsis` is
  the only non-source output from tree-sitter group rendering. Folder and
  file groups render paths separately; that carve-out is the only other
  synthesis permitted in output.
- **R2 — Line correspondence.** An entry with `line == N` for file `F`
  corresponds to source line `N` of file `F`.
- **R3 — Context-free rendering.** A group's `render()` depends only on its
  own items and the tree-sitter data they reference. It never inspects the
  scheduled set or the frontier.
- **R4 — Override resolution by content growth.** When multiple groups emit
  at the same `(file, line)`, the later group's entry wins. Enforced by
  scheduling order (A2). A debug assertion checks that a later entry has
  strictly more content than the entry it overrides: `Ellipsis < Prefix < Full`,
  and for two `Prefix` entries, the later one's content must be at least as
  long as the earlier's.
- **R5 — Intra-group ellipses only.** Ellipsis markers appear only within the
  rendering of a single group or item. Gaps between independently scheduled
  items are communicated by line-number discontinuity alone.

### Value and cost

- **V1 — Value decomposition.** Group value is
  `inherited_modifier × base_value(items)`.
- **V2 — Multiplicative modifier composition.** Each child group's
  `inherited_modifier` is set at `children()` time as
  `parent.inherited_modifier × contribution(parent_item)`. Modifiers compose
  multiplicatively along the spawning chain.
- **V3 — Cost via rendering.** Candidate cost is measured by actually
  rendering the scheduled set plus the candidate and counting tokens. The
  per-file render cache avoids redundant work. There is no parallel
  precomputed cost model. **Approved deviation:** the rewrite tokenizes
  each `RenderedEntry` individually (once, cached) and sums their costs,
  rather than tokenizing the full assembled file. This is near-exact
  (<1 token BPE boundary effect empirically) and avoids re-tokenizing
  unchanged files on every iteration. The spirit of V3 is preserved:
  cost derives from real rendered output, not a parallel estimate.
  Cached marginal costs are reused across later probes under D4/D5's
  non-overlap guarantees: once rendered, a candidate's source lines cannot be
  made cheaper by an unrelated sibling commit. If a future group type weakens
  that sibling non-overlap property, marginal costs must be invalidated when
  overlapping lines commit.
- **V4 — Budget gates.** Token budget is authoritative and used for
  prioritization. Byte budget is a fast pre-check. When `--char-budget` is
  set (including by the plugin), it is a second acceptance gate but does not
  affect prioritization.

### Design principles

- **P1 — Semantic honesty of the data model.** The data model must represent
  the actual content of the source, not an abstraction that obscures it.
  Invalid combinations should not be representable. This is a statement about
  what the types express, not about how heuristics are calibrated.
- **P2 — Visible omissions.** When content is omitted, the reader must be
  able to tell: path-only entries for deprioritized directories, line-number
  gaps for skipped source, and typed ellipsis markers for truncated doc or
  sig.
- **P3 — Grouped equivalence.** Items that receive identical rankings under
  the heuristics must be shown or omitted as a unit, never split. This is
  why atomicity (A3) is structural, not incidental.
- **P4 — Don't confuse the reader.** The output inevitably makes implicit
  claims; every claim should be defensible from the heuristics actually
  applied. We strive to uphold this rather than treat it as a hard
  constraint.

## 11. Rewrite plan

The rewrite is a single commit. New modules replace old, snapshots are
regenerated once, and nothing ships until the calibration pass (§12)
stabilizes. The current codebase is tightly interlocked around the `Symbol`
struct, which makes an incremental migration as much work as the rewrite
itself.

## 12. Calibration

The rewrite may change snapshot output significantly. Eyeballing every
change does not scale. The calibration process:

### 12.1 Snapshot review sweep

Run the snapshot tests. For every fixture whose output changed, spawn an
independent review agent (via the `/batch` skill — this is a natural first
use of it). Each agent receives:

- The fixture's source tree (the raw files of the project being summarized).
- The new precis output for that fixture at its configured budget.

Each agent inspects the snapshot critically, checking against the underlying
fixture, and reports on two questions:

1. Is there something in the source tree that was more valuable than what
   made it into the output and should have been included? Name it
   concretely (file, symbol, line range).
2. Is there something in the new output that is low-signal and should not
   have been included? Name it concretely.

Each agent appends its findings as entries in a shared findings document.
**Entries are not deduplicated.** When the same issue is raised by multiple
agents inspecting different fixtures (or even the same fixture from a
different angle), those entries are counted as separate occurrences. Issues
that come up repeatedly carry more weight.

### 12.2 Aggregation and tuning

After the sweep:

- Read through the shared findings document, grouping repeated issues and
  noting their frequency.
- Classify each issue as a heuristic tuning concern (adjust a value or
  modifier function) or as a design bug (invariant violation; needs a real
  fix, possibly back to design).
- Apply the fixes. Re-run the snapshot tests and spawn a fresh review sweep.
  Repeat until the output is stable and no high-frequency issues remain.

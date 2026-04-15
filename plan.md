# Batch: TsGroupKey trait-of-types refactor + parse colocation

## Context

The 2026-04-13 architecture audit (`issues.md`) flagged `TsGroupKey` as a god
enum (#14): 40+ variants forming a partial product, with edits required across
~9 parallel match statements per new construct. The same audit flagged
`parse/mod.rs` as a kitchen sink (#18): 1266 lines with language-specific
quirks scattered through nominally-generic functions. Both were convergent —
independently raised by Claude and Codex.

After an extensive interactive design discussion plus four Codex plan-review
passes, the chosen direction is:

1. Each current variant becomes its own struct, colocated in symbol-family
   files.
2. A narrow `TsGroupKind` trait owns the per-kind operations (`query`,
   `from_parse`, `render_item`, `children`).
3. **Per-kind parsing.** Each kind owns its own tree-sitter query (one per
   language it cares about, via exhaustive-match) and walks its own captures
   inside `from_parse`. There is no central classifier or registry of
   `from_node` calls.
4. Cross-kind concerns reduce to **fully generic central infrastructure** —
   no per-kind logic outside `from_parse` and `children`. The pre-classify
   wrapper-list filter, cross-file aggregation, calibration, and
   `dependent_siblings` drain all match-on-nothing.
5. `is_generated` files are skipped entirely at extraction time, removing the
   concept from `TsGroup`/`TsGroupKey`/the gating loop.
6. **Cross-file gating disappears.** Today a private function in file A can
   be attached to a public function in file B inside the same directory.
   Under the new architecture, gating is per-file (inside `from_parse`).
   Aggregation across files preserves the "bigger group with more items"
   effect via item merging, but the case "fileA has only privates, fileB has
   only publics" now produces visible privates instead of hidden ones. **This
   is a deliberate behavioral change. Snapshot diffs explained by it are
   acceptable; any other diff is a bug.**

## ⚠️ Halt-and-ask discipline

If at any stage you discover that a design decision in this plan is invalidated
by the actual code (a load-bearing concern we missed, an API constraint we
didn't see, an interaction that breaks the refactor), **STOP, write up what
you found, and ask the user before continuing.** Do not improvise an
alternative design mid-implementation. Examples of "invalidated":

- A normalization pass we said could move into `from_parse` turns out to need
  cross-kind context we hadn't accounted for.
- `enum_dispatch` (or its modern equivalent) doesn't support the trait shape
  we need.
- Per-kind query design hits a tree-sitter limitation that forces falling
  back to filter_nested_items.
- A snapshot diff appears that isn't explained by the cross-file gating
  change and can't be traced to a refactor bug.
- Java/TypeScript fields actually do appear in some snapshots today (we
  believe they're silently dropped, but we haven't audited every fixture).

The session has full freedom inside the design. It does **not** have freedom
to redesign on the fly when surprised. Pause and surface the issue.

## Selected issues

- **#14** — TsGroupKey god enum (primary)
- **#18** — Kitchen-sink modules (`parse/mod.rs` + `FilesGroup::children`)
- **#17** — Calibration grounding (inline comments only)
- **#13** — `unwrap_or("")` fallbacks in `render_item`
- **#5**  — `LineEntry::Full/Prefix` → `Complete/Truncated` in `design.md`
- *(Previously planned: kill the hand-numbered `ordinal()` table. Dropped
  after review — see stage 0.)*
- **Plus:** fix TOML/JSON/YAML → `Heading` routing bug. Today
  `parse/mod.rs:429-447` routes TOML `table`/`table_array_element` and
  JSON `pair` / YAML `block_mapping_pair` to `TsGroupKey::Heading`, even
  though `DataSection` and `DataSectionBody` variants already exist for
  structured data. This produces the wrong base values and modifiers on
  data files. The refactor naturally fixes this by routing those captures
  through `DataSection::from_parse` instead. **Snapshot-changing bugfix
  accepted as an expected-diff category at stage 7.**

## Architecture

### Per-variant structs

Each current `TsGroupKey` variant becomes its own struct. Structs for one
symbol family colocate in one file under `src/group/ts/`:

| File | Structs |
|---|---|
| `function.rs` | `FunctionName`, `FunctionSig`, `FunctionDocFirst`, `FunctionDocRest`, `FunctionBody` |
| `struct_.rs` | `StructName`, `StructDocFirst`, `StructDocRest`, `StructBody` |
| `enum_.rs` | `EnumName`, `EnumDocFirst`, `EnumDocRest`, `EnumBody` |
| `class.rs` | `ClassName`, `ClassDocFirst`, `ClassDocRest`, `ClassBody` |
| `interface.rs` | `InterfaceName`, `InterfaceDocFirst`, `InterfaceDocRest` |
| `trait_.rs` | `TraitName`, `TraitDocFirst`, `TraitDocRest` |
| `type_alias.rs` | `TypeAliasName`, `TypeAliasDocFirst`, `TypeAliasDocRest` |
| `const_.rs` | `ConstName`, `ConstDocFirst`, `ConstDocRest` |
| `macro_.rs` | `MacroName`, `MacroDocFirst`, `MacroDocRest` |
| `module.rs` | `ModuleDocFirst`, `ModuleDocRest` (also owns `module_doc::detect_module_doc` logic moved from `parse/module_doc.rs`) |
| `import.rs` | `Import`, `ImportedItems` |
| `impl_block.rs` | `ImplBlock` |
| `heading.rs` | `Heading`, `HeadingBody` |
| `data_section.rs` | `DataSection`, `DataSectionBody` |

Each struct holds whatever fields it actually needs:

- `FunctionName { documented: bool, public: bool }`
- `Heading { level: HeadingLevel, boilerplate: bool }`
- `HeadingBody { level: HeadingLevel }`
- `DataSection { level: u8 }` — **new field** added in stage 7 as part
  of the TOML/JSON/YAML routing fix; see that section.
- `DataSectionBody { level: u8 }` — **new field**, same reason.
- `ImplBlock { is_trait_impl: bool, is_boilerplate_trait: bool }`
- `Import { first_party: bool, reexport: bool }`
- `MacroName { documented: bool, public: bool, preproc: bool }`
- `FunctionBody`, `FunctionSig`, etc. — unit structs (no fields)

No shared `*Key` base type. No `Part` axis. Variants without per-kind data
are unit structs.

### Aggregate enum

```rust
#[enum_dispatch(TsGroupKindMethods)]
#[derive(Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
enum TsGroupKey {
    FunctionName(FunctionName),
    FunctionSig(FunctionSig),
    FunctionDocFirst(FunctionDocFirst),
    FunctionDocRest(FunctionDocRest),
    FunctionBody(FunctionBody),
    // ... all other variants ...
    ImplBlock(ImplBlock),
    Heading(Heading),
    HeadingBody(HeadingBody),
    DataSection(DataSection),
    DataSectionBody(DataSectionBody),
}
```

`enum_dispatch` (or modern equivalent — verify before depending) generates
delegating matches for receiver methods (`render_item`, `children`) at
compile time.

**Tiebreak unchanged.** The scheduler tuple `(first_path, first_line,
kind_ordinal)` at `src/schedule.rs:73-83` and the `Group::kind_ordinal()`
function at `src/group/mod.rs:123-130` stay exactly as today. The
hand-numbered `TsGroupKey::ordinal()` also stays. Killing them was
originally stage 0 but we dropped it — see stage 0 in the implementation
sequence for the rationale. Future cleanup batch can revisit.

> **Verify before depending:** check `target/doc-md` and crates.io for the
> currently-recommended `enum_dispatch` version. If `enum_dispatch` is
> unmaintained, pick the modern equivalent (`enum_delegate`, `auto_enums`,
> etc.) and confirm it supports our trait shape — receiver methods on the
> enum dispatching to the inner struct, with no associated functions on
> the dispatched trait. **`query` and `from_parse` are NOT on the
> dispatched trait** (they're called on concrete struct types via a
> hand-written dispatcher; see "Dispatcher" below).

### Traits — split into receiver-methods and parse-constructors

`enum_dispatch` is for receiver methods on the enum delegating to inner
structs. Associated functions (`query`, `from_parse` — no `self`) don't fit
the model and risk macro-shape compile failures. We split into two traits:

```rust
/// Receiver methods. Enum-dispatched on TsGroupKey.
/// Receiver methods. Enum-dispatched on TsGroupKey via the
/// `#[enum_dispatch(TsGroupKindMethods)]` attribute on the enum below.
trait TsGroupKindMethods {
    /// Render this kind's items into LineEntries.
    fn render_item<'s>(&self, item: &TsItem<'s>) -> Vec<LineEntry<'s>>;

    /// Spawn child groups when this group is committed.
    ///
    /// Important: the outer `Group::children` (in `src/group/mod.rs`) drains
    /// `parent.dependent_siblings` and prepends them to the result *before*
    /// delegating to this method. The trait method only sees an immutable
    /// `&TsGroup` after the drain.
    ///
    /// The signature is rich enough to express today's spawn behavior:
    /// creating concrete `TsGroup`s, cloning / partitioning items, propagating
    /// inherited_modifier, extracting method nodes, bucketing methods by
    /// doc/visibility, splitting TOML heading bodies per item, boosting
    /// README H1 bodies.
    ///
    /// Default: empty (most kinds spawn nothing).
    fn children<'s>(&self, parent: &TsGroup<'s>) -> Vec<Group<'s>> { vec![] }
}

/// Parse-time constructors. NOT enum-dispatched — called on concrete struct
/// types by the hand-written dispatcher.
trait TsGroupKindParse {
    /// How does this kind parse for this file's language?
    /// After the `Lang` split (see stage 6), `Lang` is grammar-1-to-1 —
    /// `Lang::C` and `Lang::Cpp` are distinct, `Lang::TypeScript` and
    /// `Lang::Tsx` are distinct. Exhaustive `match lang { ... }` enforces
    /// per-grammar declarations at compile time.
    fn parse_strategy(lang: Lang) -> KindParseStrategy
        where Self: Sized;

    /// Walk the (already top-level-filtered, when applicable) query matches
    /// and produce zero or more groups, optionally with dependent_siblings
    /// already attached. Returns `ParseTsGroup`s (pre-calibration).
    ///
    /// **Capture contract:** receives a slice of `OwnedQueryMatch` values, an
    /// owned lightweight wrapper that preserves match grouping without
    /// the cursor-lifetime dependency of tree-sitter's raw `QueryMatch`
    /// (which is `QueryMatch<'cursor, 'tree>` with streaming-iterator
    /// semantics — it cannot be stored in a `Vec` or passed by slice).
    ///
    /// Each `OwnedQueryMatch` has one `@symbol` capture and may have related
    /// captures like `@name`. The kind impl pulls captures by index from
    /// each match. This matches today's loop at `src/parse/mod.rs:61-65`,
    /// where each match yields one symbol node and any associated metadata
    /// captures.
    ///
    /// `is_top_level` is applied by the dispatcher to the symbol capture
    /// of each match before passing the match through.
    ///
    /// For source-only kinds (like ModuleDocFirst), `matches` is empty and
    /// the impl reads `ctx.source` / `ctx.lines` directly.
    fn from_parse<'q, 's>(
        matches: &[OwnedQueryMatch<'q, 's>],
        ctx: &FileCtx<'s>,
    ) -> Vec<ParseTsGroup<'s>>
        where Self: Sized;
}

/// Owned (in the sense of cursor-independent) wrapper around
/// `tree_sitter::QueryMatch`. Captures borrow their name from the
/// compiled `Query` object, which is a fundamentally different lifetime
/// from tree-sitter's cursor — the cursor invalidates each match on
/// `.next()`, but the query lives for the whole parse, so borrows from
/// it compose cleanly with `Vec` storage.
struct OwnedQueryMatch<'q, 's> {
    pattern_index: usize,
    captures: Vec<(&'q str, Node<'s>)>,
}

impl<'q, 's> OwnedQueryMatch<'q, 's> {
    fn capture(&self, name: &str) -> Option<Node<'s>> {
        self.captures.iter().find(|(n, _)| *n == name).map(|(_, node)| *node)
    }
}

/// How a kind extracts items from a file.
enum KindParseStrategy {
    /// This kind doesn't apply to this language config.
    None,
    /// This kind reads source/lines directly, no tree-sitter query.
    /// Used by ModuleDocFirst (module-doc detection looks at file prefix).
    SourceOnly,
    /// Standard query-based extraction.
    Query(&'static str),
}
```

Each per-variant struct implements **both** traits. `enum_dispatch` only
generates delegation for `TsGroupKindMethods`, which is the receiver-only
trait it's designed for. `TsGroupKindParse` is called via the hand-written
dispatcher on concrete struct types (`FunctionName::parse_strategy`,
`FunctionName::from_parse`).

This eliminates the macro-shape risk and uses `enum_dispatch` only for what
it's designed for.

### `ParseTsGroup` — pre-calibration group

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

### Outer `Group::children` (drain)

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
per-kind impl.

### Dispatcher

A free function `dispatch_kinds` calls each registered kind in declaration
order. Adding a new kind = one new line + the new file:

```rust
fn dispatch_kinds<'s>(tree: &'s Tree, ctx: &FileCtx<'s>) -> Vec<ParseTsGroup<'s>> {
    let mut out = Vec::new();
    run_kind::<FunctionName>(tree, ctx, &mut out);
    run_kind::<StructName>(tree, ctx, &mut out);
    run_kind::<EnumName>(tree, ctx, &mut out);
    run_kind::<ClassName>(tree, ctx, &mut out);
    run_kind::<InterfaceName>(tree, ctx, &mut out);
    run_kind::<TraitName>(tree, ctx, &mut out);
    run_kind::<TypeAliasName>(tree, ctx, &mut out);
    run_kind::<ConstName>(tree, ctx, &mut out);
    run_kind::<MacroName>(tree, ctx, &mut out);
    run_kind::<ModuleDocFirst>(tree, ctx, &mut out);  // SourceOnly strategy
    run_kind::<Import>(tree, ctx, &mut out);
    run_kind::<ImplBlock>(tree, ctx, &mut out);
    run_kind::<Heading>(tree, ctx, &mut out);
    run_kind::<DataSection>(tree, ctx, &mut out);
    out
}

fn run_kind<'s, K: TsGroupKindParse>(
    tree: &'s Tree,
    ctx: &FileCtx<'s>,
    out: &mut Vec<ParseTsGroup<'s>>,
) {
    match K::parse_strategy(ctx.lang) {
        KindParseStrategy::None => {
            // Kind doesn't apply to this language config.
        }
        KindParseStrategy::SourceOnly => {
            // No tree-sitter query; kind reads ctx.source directly.
            // ModuleDocFirst is the canonical example.
            out.extend(K::from_parse(&[], ctx));
        }
        KindParseStrategy::Query(query_src) => {
            let query = compile_query(ctx.config, query_src);  // LAZY: compiled on first demand per (config, kind), cached thereafter
            let capture_names: &[&str] = query.capture_names();
            let symbol_idx = query.capture_index_for_name("symbol")
                .expect("every per-kind query must define a @symbol capture");

            // Convert tree-sitter's streaming QueryMatches into
            // OwnedQueryMatch values inside the cursor loop. We can't
            // store raw QueryMatch values because they borrow cursor
            // state and get invalidated on every .next(). But capture
            // names borrow from the Query itself — that lifetime is
            // fine because the Query outlives this whole run_kind scope.
            let mut cursor = QueryCursor::new();
            let mut stream = cursor.matches(&query, tree.root_node(), ctx.source.as_bytes());
            let mut matches: Vec<OwnedQueryMatch<'_, 's>> = Vec::new();
            while let Some(m) = stream.next() {
                let symbol_node = match m.captures.iter().find(|c| c.index == symbol_idx) {
                    Some(c) => c.node,
                    None => continue,
                };
                if !is_top_level(symbol_node, ctx.lang) {
                    continue;
                }
                matches.push(OwnedQueryMatch {
                    pattern_index: m.pattern_index,
                    captures: m.captures.iter()
                        .map(|c| (capture_names[c.index as usize], c.node))
                        .collect(),
                });
            }
            out.extend(K::from_parse(&matches, ctx));
        }
    }
}
```

Key properties:
- **Per-kind queries** keyed by `Lang`. After the `Lang` split (see stage
  6), `Lang` is grammar-1-to-1 — `Lang::C` and `Lang::Cpp` are distinct,
  `Lang::TypeScript` and `Lang::Tsx` are distinct — so the kind impl can
  exhaustively match on `Lang` alone without needing additional config
  fields.
- **Three explicit strategies**: `None` (skip), `SourceOnly` (no query,
  read source), `Query(&str)` (run query). The same `None` no longer
  ambiguously means both "kind doesn't apply" and "kind has no query."
- **`is_top_level` filter is centralized**, not per-kind. See "Wrapper-list
  filter" below.
- **No `from_node` registry**, no "first Some wins" first-match logic.
- **Compile-time enforcement**: each kind must implement `parse_strategy`
  with an exhaustive match on `Lang`. Adding a new `Lang` variant forces
  compile errors in every kind file.
- **Hand-listed dispatcher**: adding a new kind = add one line. No registry
  magic, no procedural macro, no runtime registration.

### Wrapper-list filter (`is_top_level`)

Replaces both `is_inside_function` and `filter_nested_items`. Walks a captured
node's parent chain; every ancestor up to and including the root must be in
the language's wrapper-kind allowlist. First non-wrapper ancestor → return
false.

```rust
fn is_top_level(node: Node, lang: Lang) -> bool {
    let mut current = node.parent();
    while let Some(parent) = current {
        if !lang.is_wrapper_kind(parent.kind()) {
            return false;
        }
        current = parent.parent();
    }
    true
}

impl Lang {
    fn is_wrapper_kind(self, kind: &str) -> bool {
        match self {
            Lang::Rust => matches!(kind, "source_file" | "mod_item" | "declaration_list"),
            // C: plain C has no templates, namespaces, or linkage specs.
            Lang::C => matches!(kind, "translation_unit" | "declaration_list"),
            // C++: template_declaration must be a wrapper because today's
            // parser captures the inner function/class but uses the outer
            // template node as the rendered range (src/parse/mod.rs:87).
            // namespace_definition and linkage_specification are transparent
            // containers whose members should still surface as top-level.
            Lang::Cpp => matches!(
                kind,
                "translation_unit"
                    | "namespace_definition"
                    | "linkage_specification"
                    | "declaration_list"
                    | "template_declaration"
            ),
            // Python: `decorated_definition` wraps decorated classes/functions
            // (today's `is_documented` / classify path treats it as a
            // wrapper at `src/parse/ast.rs:106` and `:111`). `block` is the
            // body of `if_statement` / module-level guards in the grammar,
            // so guarded top-level decls (`if __name__ == "__main__":`) need
            // it too. Stage 5 must verify against the test_python fixture
            // (`tests/snapshots.rs:266`) which has a top-level decorated
            // class — without `decorated_definition` in the allowlist, that
            // class disappears and snapshots fail.
            Lang::Python => matches!(kind, "module" | "if_statement" | "block" | "decorated_definition"),
            Lang::Java => matches!(kind, "program" | "package_declaration"),
            // TypeScript/TSX: `internal_module` is the tree-sitter node
            // for `namespace X { ... }` / `module X { ... }`. Today's
            // parser classifies it as None (`src/parse/mod.rs:260`), so
            // declarations inside namespaces survive as top-level items.
            // The wrapper list must include `internal_module` and its
            // body kind (`statement_block` in the TS grammar) to match
            // today's behavior. Fixture reference:
            // `tests/snapshots.rs:156` has `export namespace Utils {
            // function format(...) { ... } }` and the current snapshot
            // at `snapshots__typescript_sample_budget_10000.snap:36`
            // includes the nested `format` function — Stage 5 must
            // preserve this.
            Lang::TypeScript | Lang::Tsx => matches!(
                kind,
                "program" | "export_statement" | "internal_module" | "statement_block"
            ),
            Lang::Go => matches!(kind, "source_file"),
            Lang::Lua => matches!(kind, "chunk"),
            Lang::Markdown => matches!(kind, "document"),
            Lang::Json | Lang::Toml | Lang::Yaml => true,  // structured data; all "wrappers"
        }
    }
}
```

The exact wrapper lists per language must be verified during stage 5 by
inspecting the tree-sitter grammar for each language and the existing
fixtures. The list above is the *initial guess* — confirm against actual
parse trees before relying on it.

**Default-deny semantics:** unfamiliar ancestor kinds are treated as
containers (return false). This is the safer failure mode — a missing
wrapper kind produces a snapshot diff (loud), not a silent over-capture.

### Per-language pre-pass (`FileCtx`)

`FileCtx` carries `lang` (grammar-specific after the stage 1 `Lang`
split), `path`, `source`, `lines`, plus per-language extras. `Lang`
itself is what distinguishes `C` from `Cpp` and `TypeScript` from `Tsx`
post-split, so `parse_strategy` and `from_parse` impls can match on
`lang` directly. The `config: &'s LanguageConfig` reference is still
available for tree-sitter-level plumbing (the actual grammar handle,
cached query objects, etc.) but the per-kind `parse_strategy` decisions
key off `lang`. Per-language extras (e.g., JS exported-names set) live
in a side enum:

```rust
struct FileCtx<'s> {
    config: &'s LanguageConfig,    // tree-sitter plumbing (language handle, etc.)
    lang: Lang,                    // grammar-specific after stage 1
    /// The display path used in TsItem.path and rendered headers — this is
    /// the interned root-relative path, NOT the raw walked path.
    /// `HeadingBody` auto-commit (`src/schedule.rs:167-170`) checks
    /// `item.path.parent().is_some_and(|p| p.as_os_str().is_empty())` to
    /// detect root-level README, so display_path must be root-relative.
    display_path: &'s Path,
    /// The actual on-disk path used for source reads. May differ from
    /// display_path. Generated-file detection uses source_path.
    source_path: &'s Path,
    source: &'s str,
    lines: &'s [&'s str],
    extras: FileCtxExtras<'s>,
}

enum FileCtxExtras<'s> {
    None,
    Rust(RustFileCtx<'s>),     // mod_names, etc.
    Js(JsFileCtx<'s>),         // collected exported-names set
    Python(PythonFileCtx<'s>),
    Markdown(MarkdownFileCtx<'s>),
    // ... add only when a language needs extras ...
}
```

`from_parse` impls always have `lang` and `path` available (Java field
handling, C/C++ checks, Rust crate-root reexport detection all need them).
The per-language pre-pass that populates `extras` runs in `parse/mod.rs`
before `dispatch_kinds`.

### Cross-kind passes — what survives, what dissolves

| Pass | Today | New architecture |
|---|---|---|
| `is_inside_function` | central pre-filter | **dissolves** into `is_top_level` (wrapper-list filter) |
| `filter_nested_items` | central post-pass | **dissolves** into `is_top_level` |
| `extend_section_ranges` | central post-pass (heading-only) | **moves into** `Heading::from_parse`. Prerequisite: the TOML/JSON/YAML → `DataSection` routing fix. After the fix, `Heading` is produced only by markdown files, markdown files contain only heading items (grammar requires newlines between headings), so `dedup_line_overlaps` is a no-op on markdown files. The two passes then operate on genuinely disjoint file sets and their ordering is irrelevant. Without the routing fix this move would be unsafe — see `dedup_line_overlaps` notes. |
| `dedup_overloads` | central post-pass (per-kind, per-language) | **moves into per-kind `from_parse` impls** via **tree-sibling adjacency** instead of flat-stream adjacency. Two nodes are "adjacent" if they share a parent in the AST AND nothing (of any kind) sits between them in `parent.children()`. Each per-kind impl implements this check using its own tree-sitter nodes, with no need to see other kinds' items. See "`dedup_overloads` notes" below. |
| `dedup_line_overlaps` | central post-pass (drops second item) | **per-file post-pass, generic**: depth-capped sibling chainer (cap 5), runs after `dispatch_kinds` for each file but BEFORE cross-file aggregation. See below. |
| Module-doc detection | special pre-query step at `parse/mod.rs:44` | **moves into** `ModuleDocFirst::from_parse` (the function reads the source directly; no query captures involved) |
| Cross-file aggregation | implicit in `FilesGroup::children` bucketing by `(key, is_generated)` | **stays central, generic**: merge by `key`, recursively merge `dependent_siblings`. See below. |
| File-level gating loop (`is_gated_by` + same-generated preference) | `src/group/files.rs:79-100` | **deleted**. Cross-file gating is gone (deliberate). In-file gating is produced inside `from_parse` as `dependent_siblings`. |
| `nest_heading_groups` | `src/group/files.rs:113-205` | **moves into** `Heading::from_parse`. Per-individual-heading parent selection (not per-group), so the boilerplate-tie problem dissolves. |
| `compute_item_modifier` | `src/group/files.rs:209-260` | **moves into** `calibration.rs`. Same semantics: applied at modifier construction, propagates to children via `inherited_modifier`. |

**Net result:** every cross-kind concern is either fully generic central
infrastructure (acceptable per the user's criterion) or per-kind logic inside
`from_parse`. There is no per-kind logic outside `from_parse` and `children`.

#### `dedup_overloads` notes

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
they do, **stage 7** will produce snapshot diffs (stage 6 keeps the central
flat-stream pass; the per-kind AST-sibling decomposition lands in stage 7
alongside the other semantic changes). The diffs fall into the
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

No central `dedup_overloads` function exists in the new architecture.
(The nested `flat_view` helper still exists for the line-overlap chainer,
but it's not used by any overload-dedup logic.)

#### `dedup_line_overlaps` — depth-capped sibling chainer

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

#### Cross-file aggregation

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

Each const gets a one-line grounding comment per #17.

`compute_item_modifier(key, parent_modifier) -> f64` reads the relevant
factors based on the key's variant and applies them. **`is_generated` is
gone from this function** because generated files no longer reach
extraction.

Inside `ts_base_value`, each numeric literal gets a one-line grounding
comment per #17 (no declarative table).

### Skip generated files at extraction

Today: `is_generated` is computed per-file in `FilesGroup::children`,
threaded through bucketing, and produces TsGroups with `inherited_modifier
= 0.0` (which means they're never scheduled).

New: in `FilesGroup::children`, after determining a file is generated, **skip
it entirely** — do not parse, do not dispatch, do not produce TsGroups. The
file header still appears via `FilesGroup` (file headers are committed
independently).

**Equivalence is partial**: for a generated file's *own scheduled content*,
new and old behavior are equivalent (nothing scheduled either way). However,
for *gated relationships involving generated groups*, removing the generated
groups changes parent selection in the gating loop, which can unhide
previously-hidden private groups. This is a deliberate behavioral change,
and the snapshot diffs it produces are explicitly accepted at stage 7.

Removed: `is_generated` from `compute_item_modifier`, `is_generated`
threading through `(key, is_generated)` bucketing tuples, the same-generated
preference logic in the gating loop (along with the gating loop itself).

### TOML/JSON/YAML routing fix (bugfix, stage 7 diff)

Today's `classify` function at `src/parse/mod.rs:418-447` routes four
different node kinds to `TsGroupKey::Heading`:

```rust
"atx_heading" | "setext_heading" => Some(Heading { ... }),     // markdown
"table" | "table_array_element"  => Some(Heading { ... }),     // TOML  -- BUG
"pair" | "block_mapping_pair"    => Some(Heading { ... }),     // JSON/YAML -- BUG
```

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

After the fix, `Heading` is markdown-only (just like the prose semantics
imply), and `extend_section_ranges` can move into `Heading::from_parse`
safely.

**Snapshot impact**: TOML/JSON/YAML fixtures may show narrow diffs at
stage 7, but each one must be tied to a specific mechanism (kind
routing, body rendering, body splitting) — see Snapshot Policy category
4 for the exhaustive list. A data-file fixture diff that doesn't match
one of those mechanisms is a real regression and halts.

### Module-doc detection moves into `ModuleDocFirst::from_parse`

Today `module_doc::detect_module_doc` runs as a special pre-query step at
`src/parse/mod.rs:44`. Module docs aren't produced by any tree-sitter query
capture.

New: `ModuleDocFirst::parse_strategy(lang)` returns
`KindParseStrategy::SourceOnly` for every language config that supports
module docs (Rust, Python, Markdown, etc.) and `KindParseStrategy::None`
elsewhere. `ModuleDocFirst::from_parse(&[], ctx)` ignores its empty captures
parameter and instead reads `ctx.source` / `ctx.lines` directly to detect
the module doc, returning a `Vec<ParseTsGroup>` with at most one group
containing one item. The current `module_doc.rs` helpers (`detect_module_doc`,
etc.) move into `src/group/ts/module.rs` as private helpers used by
`from_parse`.

The `SourceOnly` strategy is uniform — the dispatcher (see "Dispatcher"
above) handles it explicitly with no special-case branching for module-doc
detection. ModuleDocFirst is just one kind that opts into the SourceOnly
strategy; any future "look at the file as a whole, not query captures" kind
would use the same mechanism.

## File-level pipeline

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

## Implementation sequence

Each stage compiles and passes snapshot tests on its own. Stages that touch
the enum update **all** cross-file pattern-match sites for the variants they
change in one compile-step — partial conversions won't compile.

### Stage 0 — (removed)

The original plan had stage 0 = "kill `ordinal()` by deriving
`EnumDiscriminants` on `TsGroupKey` and defining a `GroupKindRank` enum
for the tiebreak tuple." Codex correctly pointed out that today's
`Group::kind_ordinal()` uses a *single numeric namespace* across
`Folders=0`, `Files=1`, and `TsGroupKey` variants starting at 0 — so
`Group::Ts(ModuleDocFirst)` and `Group::Folders` share the same tiebreak
rank today. A clean `GroupKindRank { Folders, Files, Ts(...) }` enum with
derived `Ord` puts all `Ts` variants strictly after `Files`, which changes
the tiebreak ordering across groups in a way that *could* surface on any
fixture where the tiebreak fires. Not worth fighting under a strict
zero-diff policy.

**Decision: keep the hand-numbered `ordinal()` alive.** It's ugly, but
it's correct and not particularly load-bearing for the refactor. Killing
it can happen as a separate cleanup batch after this refactor lands,
with its own explicit design (and its own tiebreak-preservation audit).

Stage 0 slot is kept empty for numbering consistency with the earlier
plan drafts.

### Stage 1 — Split `Lang` to match tree-sitter grammars 1-to-1

(This slot previously held "skip generated files" — moved to stage 7
because its snapshot diffs aren't zero.)

The `Lang` split is an independent prerequisite for stages 5 and 6 (both
the wrapper-list filter and per-kind `parse_strategy` need `Lang::Cpp` /
`Lang::TypeScript` / `Lang::Tsx` to exist). Do it early, in its own stage,
to avoid entangling with the trait refactor:

- Replace `Lang::C` with `Lang::C` + `Lang::Cpp`.
- Replace `Lang::JsTs` with `Lang::TypeScript` + `Lang::Tsx`. (TypeScript
  grammar parses plain JS, so `.js`/`.mjs`/`.cjs` go to `Lang::TypeScript`;
  `.jsx`/`.tsx` go to `Lang::Tsx`.)
- Update `Lang::from_extension` in `src/lib.rs` (this is the entry point
  called by `ParseStore::config_for` at `src/store.rs:63`; `from_path`
  delegates to it) with the new mapping: `.c` → `Lang::C`;
  `.h`/`.cpp`/`.cc`/`.cxx`/`.hpp`/`.hxx`/`.hh` → `Lang::Cpp`; etc.
- Update `language_for_lang` in `src/store.rs:130` to drop
  extension-based disambiguation and become a simple 1-to-1 match on
  `Lang` → `tree_sitter::Language` + query file.
- Update every existing `match` / `matches!` / `==` on `Lang::C` and
  `Lang::JsTs` across the codebase, per the agent risk assessment. Most
  become `Lang::C | Lang::Cpp` or `Lang::TypeScript | Lang::Tsx` unions.
  Load-bearing callsites to audit:
  1. `src/parse/mod.rs:90` template_declaration — change to
     `Lang::Cpp` (no effect on `.c` files since they don't have
     template_declaration nodes, but clarifies intent).
  2. `src/parse/mod.rs:361` `"declaration"` arm — `Lang::C | Lang::Cpp`
     if the handling is identical for both, otherwise split into two
     arms.
  3. `src/parse/mod.rs:1081` `dedup_overloads` `matches!` — extract a
     `should_dedup_overloads(lang: Lang) -> bool` helper to avoid
     pattern explosion.
  4. Import classification (`src/parse/mod.rs:457`, `:936`, `:952`),
     visibility rules (`src/parse/visibility.rs:27`), doc-comment style
     check (`src/parse/ast.rs:160`), method detection
     (`src/group/ts.rs:528`, `:729`, `:746`).
- Test fixtures don't need renaming — snapshot tests are keyed by path.
- `cargo test-release` — must show **zero snapshot diffs**. The split is
  a pure refactor of the type-level representation; runtime behavior is
  unchanged because every extension still routes to the same grammar it
  routes to today.

### Stage 2 — Add `enum_dispatch` and the empty receiver-methods trait

- Verify `enum_dispatch` (or modern equivalent) via `cargo doc-md` /
  crates.io. **If unmaintained or doesn't support our shape, halt and ask
  the user for direction before continuing.**
- Add to `Cargo.toml`. Run `cargo doc-md`.
- Define the empty `TsGroupKindMethods` trait in `src/group/ts/kind.rs`.
  This is the **only** trait `enum_dispatch` will see. `TsGroupKindParse`
  is added in stage 6 and is a separate, non-dispatched trait.
- Define `ParseTsGroup` in the same file.
- `cargo test-release` — zero diffs.

### Stage 3 — Convert variants to wrapping tuple variants, family by family

**Trait requirements for every wrapped struct** (including unit structs
without fields):

```rust
#[derive(Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Debug)]
struct FunctionName { documented: bool, public: bool }
```

`TsGroupKey` derives `Clone, Hash, Eq, PartialEq, Ord, PartialOrd`, and
those derives only compile if every variant payload also derives them.
Stage 7's cross-file aggregator uses `BTreeMap<TsGroupKey, _>`, so `Ord`
is load-bearing — no skipping it.

For each symbol family (`Function`, `Struct`, `Enum`, `Class`, `Interface`,
`Trait`, `TypeAlias`, `Const`, `Macro`, `Module`, `Import`, `ImplBlock`,
`Heading`, `DataSection`):

a. Create `src/group/ts/<family>.rs` containing one struct per current
   variant, with the fields the current variant has and the derives above.
b. Change the corresponding `TsGroupKey` variant from struct-like
   (`FunctionName { documented, public }`) to tuple-like
   (`FunctionName(FunctionName)`).
c. **Update every match site that destructures these variants**, in one
   compile-step. Every one of these needs its match arms rewritten to
   handle the new tuple-variant shape (`FunctionName(FunctionName)`
   instead of `FunctionName { documented, public }`):
   - `src/group/ts.rs` — `ordinal` (still alive; stage 0 was dropped),
     `is_gated`, `is_gated_by`, `heading_level`, `doc_rest_key`,
     `children`, `render_item`
   - `src/heuristics.rs` — `ts_base_value`, `documented_contribution`
   - `src/group/files.rs` — `compute_item_modifier`, `nest_heading_groups`,
     gating callsite
   - `src/schedule.rs` — `is_auto_commit_body`
   - Anywhere else `cargo check` complains
d. Implement empty `TsGroupKindMethods` for the new structs (still empty
   trait at this stage).
e. `cargo test-release` — zero diffs.

This is mechanical but tedious. Family-by-family because each family is small
enough to debug. **Trait remains empty** at this stage — no required methods
to implement.

### Stage 4 — Move `render_item` into the trait (one compile-step)

Adding a required trait method makes every existing impl incomplete at once,
so this stage is **one compile-step**, not subdivided per-family:

- Add `fn render_item<'s>(&self, item: &TsItem<'s>) -> Vec<LineEntry<'s>>;`
  to `TsGroupKindMethods`.
- Move every family's match arms out of the central `render_item` in
  `src/group/ts.rs` and into the trait impl in `src/group/ts/<family>.rs`,
  all at once.
- Delete the now-empty central `render_item`.
- `cargo test-release` — zero diffs.

If the size of this step is uncomfortable, an alternative is to give the
trait method a default body that delegates to the existing central function
(`fn render_item(&self, item) { ts::render_item_legacy(self, item) }`), then
migrate family by family by overriding the default, and finally delete the
legacy function. Either way, the trait must compile at every intermediate
state.

### Stage 5 — Wrapper-list pre-filter

- Define `is_top_level(node, lang)` and `Lang::is_wrapper_kind` in
  `src/parse/ast.rs` (or a new `src/parse/top_level.rs`).
- Initial wrapper lists per language are *guesses*; verify against actual
  parse trees of fixture files before relying on them. If the lists need
  refinement to preserve byte-identical snapshots, refine them now.
- Replace the existing `is_inside_function` call at `src/parse/mod.rs:67`
  with `is_top_level`.
- Delete `filter_nested_items` (`src/parse/mod.rs:981-1031`).
- Delete `is_inside_function` (`src/parse/ast.rs:168`).
- `cargo test-release` — zero diffs. **Any diff that isn't a missing-wrapper
  bug is a halt-and-ask signal.**

### Stage 6 — Per-kind queries, `from_parse`, dispatcher (one compile-step)

This is the biggest stage. Halt and ask if you discover anything that
invalidates the design.

(The `Lang` split happened in stage 1, so by stage 6 the new variants
exist and per-kind `parse_strategy` can exhaustively match on `Lang`.)
- Define `KindParseStrategy` enum and `TsGroupKindParse` trait in
  `src/group/ts/kind.rs` (alongside the existing `TsGroupKindMethods` from
  stage 2).
- **Important:** do NOT add `parse_strategy` / `from_parse` to
  `TsGroupKindMethods`. Those are associated functions on
  `TsGroupKindParse`, called by the dispatcher on concrete struct types.
  `enum_dispatch` only sees `TsGroupKindMethods`.
- Define `FileCtx` struct + `FileCtxExtras` enum in `src/parse/file_ctx.rs`.
  Carries `&'s LanguageConfig`, `lang`, `path`, `source`, `lines`, `extras`.
- Build per-language pre-pass producing `FileCtx`. Most languages start
  with `extras: FileCtxExtras::None`; JS gets exported-names collection;
  Rust gets mod_names; others get whatever pre-pass state their `from_parse`
  impls need.
- For each kind, write its `parse_strategy(lang) -> KindParseStrategy`
  (exhaustive match on `Lang`, which is now grammar-1-to-1 after the
  `Lang` split) and its `from_parse()` (porting
  the relevant arms of the central `classify` function and any per-kind
  quirk handling).
- For per-kind query strings: split the existing `queries/<lang>.scm` files
  by kind into per-kind query strings or const literals colocated with the
  kind file. Each kind's `parse_strategy` returns `Query(&str)` for the
  grammars it supports.
- **Stage 6 keeps ALL existing behavior changes for stage 7.** Three
  semantic changes — `extend_section_ranges` move into
  `Heading::from_parse`, `dedup_overloads` flat-stream → AST-sibling
  decomposition, and TOML/JSON/YAML → `DataSection` routing fix — each
  produce expected snapshot diffs. They land together in stage 7, where
  diffs are reviewed. Stage 6 is strict zero-diff and keeps the existing
  central passes exactly:
  - `Heading::from_parse` in stage 6 produces **flat, unextended**
    heading items. The file-level `nest_heading_groups` and the central
    `extend_section_ranges` continue to run (through the adapter —
    see below).
  - `dedup_overloads` stays as a **central flat-stream** pass, called
    from the stage 6 adapter in the same position as today.
  - TOML/JSON/YAML still route to `Heading` in stage 6 (the routing fix
    waits for stage 7). Each grammar gets its own query string, since
    tree-sitter compiles queries against one concrete grammar per
    language — a cross-grammar union query string wouldn't compile.
    Concretely, `Heading::parse_strategy(lang)` returns:
    - `Lang::Markdown` → `Query(MARKDOWN_HEADING_QUERY)` (atx/setext)
    - `Lang::Toml` → `Query(TOML_TABLE_QUERY)` (table/table_array_element)
    - `Lang::Json` → `Query(JSON_PAIR_QUERY)` (pair)
    - `Lang::Yaml` → `Query(YAML_PAIR_QUERY)` (block_mapping_pair)
    - every other `Lang` → `KindParseStrategy::None`
  - `Heading::from_parse` in stage 6 branches on `ctx.lang` internally
    to build the right heading shape per language (markdown level
    detection, TOML dot-count, JSON/YAML level 1) — matching today's
    `classify` logic exactly.
  - Stage 7 then moves TOML/JSON/YAML queries OUT of
    `Heading::parse_strategy` and INTO `DataSection::parse_strategy`,
    producing `DataSection` items instead of `Heading`. The queries
    themselves just relocate to the DataSection kind file.
- Move `module_doc` detection from `parse/module_doc.rs` into
  `src/group/ts/module.rs`. `ModuleDocFirst::parse_strategy` returns
  `KindParseStrategy::SourceOnly` for languages with module docs;
  `ModuleDocFirst::from_parse` reads `ctx.source` directly.
- Write `dispatch_kinds` and `run_kind` in `src/parse/mod.rs`.
- Replace the central capture loop in `parse/mod.rs:55-141` with
  `dispatch_kinds`. The function returns `Vec<ParseTsGroup>` for the file.
- **Adapter to preserve the existing `extract_items` API.** Stage 7 will
  restructure `FilesGroup::children` to consume `Vec<ParseTsGroup>` directly,
  but stage 6 must keep the old flat `Vec<(TsGroupKey, TsItem)>` interface
  alive so the rest of the codebase compiles. The current API
  (`src/parse/mod.rs:21`) is:
  ```rust
  pub fn extract_items<'s>(
      display_path: &'s Path,
      source: &'s str,
      tree: &'s Tree,
      config: &LanguageConfig,
  ) -> Vec<(TsGroupKey, TsItem<'s>)>
  ```
  Keep this exact signature in stage 6. Internally, the new implementation
  must reproduce today's per-file normalization order at
  `src/parse/mod.rs:128-131` exactly:
  1. Builds a `FileCtx` from the parameters (in stage 6 `display_path` and
     `source_path` are the same since the legacy API takes one path).
  2. Calls `dispatch_kinds(tree, &file_ctx)` to get `Vec<ParseTsGroup>`.
     Stage 6 `from_parse` impls produce **flat, unextended** output — no
     heading nesting, no range extension, no per-kind dedup_overloads.
     TOML/JSON/YAML still produce `Heading` (routing fix waits for stage 7).
  3. **Runs the OLD drop-style `dedup_line_overlaps`** (not the chainer)
     over a flat source-ordered view of the items. Stage 7 swaps this
     for the chainer.
  4. **Runs central `extend_section_ranges`** in the same position as
     today, mutating heading items' `end_line` in place. Stage 7 moves
     this into `Heading::from_parse` after the routing fix.
  5. **Runs central flat-stream `dedup_overloads`** (today's logic,
     moved to a helper in `src/parse/normalize.rs`). Stage 7 replaces
     this with per-kind AST-sibling dedup.
  6. Flattens `Vec<ParseTsGroup>` back into `Vec<(TsGroupKey, TsItem)>`
     by iterating each group's items and pairing with the group key.
  7. Sort the flat result by source position to match today's behavior.
  
  No call-site change needed in `FilesGroup::children` — it still calls
  `extract_items(display_path, source, tree, config)`. The function body
  is the only thing that changes.
  
  **Critical:** today's normalization order is
  `dedup_line_overlaps` → `extend_section_ranges` → `dedup_overloads`
  (`src/parse/mod.rs:128-131`). The stage 6 adapter preserves this order
  exactly.
- The adapter exists only between stage 6 and stage 7. Stage 7 deletes it
  (and the legacy `extract_items` signature) along with the rest of the
  old shape.
- **Method extraction inside `ClassName::children`** stays as-is (uses
  existing tree-sitter traversal, not query). `spawn_method_children` moves
  into `class.rs` as part of `ClassName::children` impl.
- **Add a release-mode duplicate-emission check, not a `debug_assert`.**
  Several current node kinds are polymorphic across kinds (JS
  `lexical_declaration` can be import/function/const, C `declaration` can
  be function/const, Go `type_spec` can be struct/interface/type alias).
  Per-kind queries with imperfect predicates could double-emit. **The
  check must run under `cargo test-release`**, so use a regular `assert!`
  (not `debug_assert!`) inside `dispatch_kinds` *or* add a focused
  integration test that runs the dispatch over each fixture and asserts
  no two emitted `TsItem`s have the same `(path, byte_start, byte_end)`
  triple. **If the check fires on any fixture, halt and investigate** —
  either tighten the per-kind query predicates or add an explicit
  "winning kind" rule for the polymorphic node.
- `cargo test-release` — must show **zero snapshot diffs**. If diffs appear
  here that aren't explained by the cross-file gating change (which only
  surfaces in stage 7), halt and investigate.

### Stage 7 — Skip generated, cross-file aggregation, cross-file gating removal

- **Skip generated files** in `FilesGroup::children` after reading source
  (current detection at `src/group/files.rs:33-37` calls three things:
  `classify::is_generated_file(source)`, `is_autogen_api_doc(source, role)`,
  and `is_generated_filename(relative)`). The skip needs the source content
  *and* the file role *and* the path. Pseudocode:
  ```rust
  let source = ctx.store.read_source(path);
  let role = file_role(path);
  if classify::is_generated_file(source)
      || classify::is_autogen_api_doc(source, role)
      || classify::is_generated_filename(relative_path)
  {
      continue;  // skip parsing, dispatching, all of it
  }
  let tree = ctx.store.parse(path);  // store-owned
  ```
  This may require a small `ParseStore` API change to read the source
  without committing to a parse — `read_source(path) -> &'s str` separate
  from `parse(path) -> &'s Tree`. Verify the existing store API and adjust
  if needed.
- Delete `generated_contribution` and `is_generated` threading through
  `compute_item_modifier`.
- Define `aggregate_across_files` and `merge_into` in
  `src/group/aggregate.rs` (new file).
- Define `apply_dedup_line_overlaps_chainer` (depth cap 5) as a per-file
  post-pass — runs **before** cross-file aggregation, so it can only chain
  symbols within the same file. Each per-file `dispatch_kinds` result goes
  through the chainer before being merged into the cross-file aggregator.
  This prevents two unrelated symbols in different files that happen to
  share a start_line from being chained together.
- **Move heading nesting from the file-level `nest_heading_groups` into
  `Heading::from_parse`.** Per-individual-heading parent selection (not
  per-group), so the boilerplate-tie problem dissolves. The heading
  nesting now lives inside the kind that owns headings.
- **Implement same-file gating inside per-kind `from_parse` impls before
  deleting the file-level gating loop.** The current loop at
  `src/group/files.rs:79-100` does *both* same-file and cross-file gating;
  we want to lose only the cross-file part. For each kind that today has
  gating relationships (`is_gated`/`is_gated_by` at `src/group/ts.rs:187`
  and `:214`):
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
- **Semantic changes landing at stage 7** (all produce expected diffs):
  1. **TOML/JSON/YAML → `DataSection` routing fix.** Split the combined
     stage 6 `Heading::parse_strategy` so `Heading` only claims markdown
     node kinds. Move TOML `table`/`table_array_element`, JSON `pair`,
     and YAML `block_mapping_pair` to `DataSection::parse_strategy`.
     Build a new `DataSection::from_parse` that produces `DataSection`
     items with the right level (TOML dot-count, JSON/YAML level 1).
     Port today's TOML per-item body-spawn logic from `Heading::children`
     (`src/group/ts.rs:405-437`) into `DataSection::children`. **Port
     `HeadingBody::render_item`'s body-start and noise handling (today
     at `src/group/ts.rs:976`) into `DataSectionBody::render_item` —
     today's `DataSectionBody::render_item` starts at `start_line + 1`
     without the markdown-safe behavior, so diffs would appear unless
     the behavior is brought into alignment.** Add level-based values
     to `DataSection`/`DataSectionBody` in `calibration.rs` matching
     today's Heading curve (see routing-fix notes).
  2. **`extend_section_ranges` moves into `Heading::from_parse`.** Safe
     now because after the routing fix, `Heading` is markdown-only and
     the ordering hazard is genuinely irrelevant (markdown and
     non-markdown files are disjoint from the line-overlap pass's
     perspective).
  3. **`dedup_overloads` decomposes per-kind via AST-sibling adjacency.**
     Delete the central `dedup_overloads` from `src/parse/normalize.rs`.
     `FunctionName::from_parse` and friends implement their own
     AST-sibling adjacency check using the `is_ast_adjacent` helper.
     `StructName`/`EnumName`/`TypeAliasName`/`MacroName::from_parse`
     implement their own name-set dedup within their captures.
  4. **`dedup_line_overlaps` drop → chain.** Replace the central drop
     pass with the per-file depth-capped chainer.
  5. **Heading nesting moves into `Heading::from_parse`.** Per-individual-
     heading parent selection.
  6. **Same-file gating moves into per-kind `from_parse`.** See next
     bullet.
- **Rename `src/heuristics.rs` → `src/calibration.rs`**:
  - Rename the file.
  - Update `pub mod heuristics;` → `pub mod calibration;` at
    `src/lib.rs:4`.
  - Update every `crate::heuristics::` callsite (including
    `src/group/mod.rs:87`, `src/schedule.rs:161`, and anywhere else
    `cargo check` complains) to `crate::calibration::`.
  - Move `compute_item_modifier` from `src/group/files.rs` into
    `src/calibration.rs`. Update its callers.
  - Add `finalize_group(parse_group, parent_modifier) -> TsGroup` that
    turns `ParseTsGroup` → `TsGroup` with `inherited_modifier` computed
    (semantics per the "Calibration" architecture section).
  - `finalize_group` is called from stage 7's new `FilesGroup::children`
    pipeline, so the infrastructure must exist here, not at stage 9.
  - Stage 9 polishes the renamed file with grounding comments and const
    extraction.
- Restructure `FilesGroup::children` to follow the pipeline:
  1. Skip generated files
  2. Per file: parse → `dispatch_kinds` (`Heading::from_parse` now
     produces extended-and-nested headings; `FunctionName::from_parse`
     and friends now do their own tree-sibling-adjacency
     `dedup_overloads`) → per-file `dedup_line_overlaps` chainer
  3. Cross-file aggregate (merge by key, recursively merge dependent_siblings)
  4. Calibrate (`finalize_group`) — adds inherited_modifier
- **No central `extend_section_ranges` or `dedup_overloads` passes.** Both
  have moved into per-kind `from_parse` impls. The central pipeline only
  has the generic line-overlap chainer left (plus aggregation and
  calibration).
- **Delete the file-level gating loop** at `src/group/files.rs:79-100`.
- **Delete `nest_heading_groups`** at `src/group/files.rs:113-205`
  (now lives inside `Heading::from_parse`).
- **Delete the `extract_items_legacy_adapter`** introduced in stage 6.
  `FilesGroup::children` now consumes `Vec<ParseTsGroup>` directly.
- `cargo test-release` — **expect snapshot diffs explained by one of six
  categories** (see "Snapshot policy"):
  1. Cross-file gating removal.
  2. Generated-file skipping unhiding gated groups.
  3. The `dedup_line_overlaps` chainer change.
  4. TOML/JSON/YAML → `DataSection` routing fix.
  5. `dedup_overloads` flat-stream → AST-sibling semantic change.
  6. Heading nesting per-group → per-individual parent selection.
- Spawn an Agent to review every diff and confirm:
  - Each diff is explained by exactly one of the six accepted
    categories above.
  - The new output is not worse as a reader's experience — read each diffed
    fixture's before/after as a fresh user. Look for understanding
    regressions hiding in the "expected" diffs (per CLAUDE.md).
- If any diff is unexplained, or if any reading regression surfaces, halt
  and ask the user.

### Stage 8 — Move `children` (spawn) into the trait (one compile-step)

Same constraint as stage 4. Add the `children` trait method (with empty
default), move the central `ts::children` match arms into per-family
overrides all at once. `Group::children` in `src/group/mod.rs` keeps the
`dependent_siblings` drain wrapping the dispatch.

Cross-kind shared helpers (extract method nodes, bucket by doc/visibility)
move to `src/group/ts/spawn.rs` as free functions, called by the kind impls
that need them.

`cargo test-release` — zero diffs.

### Stage 9 — Calibration grounding (`calibration.rs`)

Stage 7 already renamed `heuristics.rs` → `calibration.rs`, moved
`compute_item_modifier` into it, and created `finalize_group`. Stage 9
just polishes the content:

- Replace `visibility_contribution`, `documented_contribution`,
  `boilerplate_heading_contribution`, `reexport_contribution` with consts
  (`PRIVATE_FACTOR`, `UNDOCUMENTED_FACTOR`, `BOILERPLATE_HEADING_FACTOR`,
  `REEXPORT_FACTOR`). Consumed inside `compute_item_modifier`. Note:
  `generated_contribution` is **not** in this list — it was deleted in
  stage 7 when generated-file skipping moved to extraction time, so
  there's no factor left to extract.
- Add a one-line grounding comment per literal in `ts_base_value` and per
  const (#17 — inline comments only, not a declarative table).
- **`finalize_group` dependent-sibling semantics** (already implemented
  in stage 7; documenting here for clarity): dependent siblings are
  *siblings*, not children — they each get their own `inherited_modifier`
  computed from the same `parent_modifier` (the file/folder-level
  modifier), NOT from the containing group's `inherited_modifier`. This
  matches today's behavior (`src/group/files.rs:61` computes each gated
  group's modifier independently before attaching them as
  `dependent_siblings` at line 73). Walking `dependent_siblings`
  recursively in `finalize_group` is fine, but each recursion uses
  `parent_modifier`, not the containing group's already-finalized
  modifier.
- `cargo test-release` — zero diffs.

### Stage 10 — Cleanup

- `unwrap_or("")` → `debug_assert!` + `lines[idx]` in per-kind `render_item`
  impls (#13). At this point `render_item` lives per-family file, so the
  bounds-check cleanup is per-file.
- `design.md`: `LineEntry::Full/Prefix` → `Complete/Truncated` (#5).
- `design.md`: §3.3 / §4 updated to describe the trait architecture.
- **File new `issues.md` entry: "Remove hand-numbered `ordinal()` magic
  table"**. The batch deliberately kept `TsGroupKey::ordinal()` and
  `Group::kind_ordinal()` alive (see stage 0 rationale) because the
  cross-group tiebreak namespace (Folders=0, Files=1, Ts variants at
  0-120) is load-bearing and not trivially replaceable by derived
  `EnumDiscriminants` without changing the Folders/Files/Ts relative
  ordering. Cleanup approach to note in the issue: either (a) design a
  single `GroupKindRank` enum with hand-maintained variants that
  preserve today's exact ordering across Folders/Files/Ts, or (b)
  restructure the tiebreak to use `(path, line, lexicographic-kind)`
  and accept whatever fallout. Separate batch.
- File `Java fields silently dropped` and similar in `output-issues.md`
  (the user explicitly deferred fixing these but wants them documented).
- File "`ImplBlock` trait impl should not be gated" in `issues.md`
  (user note from the design discussion; deferred because it's a
  snapshot-changing cleanup).
- File "Mod::children could re-run `from_parse` with a filter for nodes
  inside the module, instead of duplicating extraction logic per-kind"
  in `issues.md` (user idea, out of scope for this batch).
- Run `/simplify` on the resulting diff per `feedback_simplify_step.md`.
- `cargo bench-hot` — investigate any regression.

## Snapshot policy

**Strict byte-identical at every stage EXCEPT stage 7.** Stage 6 is
structural-only and produces zero diffs; any diff there is a bug and
halts the stage.

**Stage 7** snapshot diffs are accepted when they fall into one of these
five explicitly-enumerated categories:

1. **Cross-file gating removal** — privates previously hidden behind
   cross-file publics now appear directly, because parent selection is
   now per-file.
2. **Generated-file skipping unhiding gated groups** — a non-generated
   gated group previously attached to a generated parent (and thus
   invisible because the parent had modifier 0) now appears directly
   because the generated parent doesn't exist.
3. **`dedup_line_overlaps` chainer change** — line overlaps now appear as
   dependent_siblings instead of being dropped.
4. **TOML/JSON/YAML → `DataSection` routing fix** — TsGroupKey shifted
   from `Heading` to `DataSection`. Base values are **preserved** by
   copying the Heading level curve into DataSection (see the routing-fix
   section). Diffs in this category must be tied to a **specific
   mechanism**, not accepted by file extension alone:
   - **(4a)** Kind name changing in any output that mentions the kind
     directly (mostly internal, rarely user-visible).
   - **(4b)** Render behavior differences between
     `HeadingBody::render_item` and `DataSectionBody::render_item` — if
     stage 7's port of the markdown-safe behavior isn't perfect, residual
     rendering shifts appear here. Fixture diffs should be tied to
     specific lines in body content.
   - **(4c)** TOML per-item body splitting, if stage 7's port from
     `Heading::children` to `DataSection::children` produces any slight
     grouping changes.
   - **(4d)** Modifier factors that genuinely differ — today's
     `boilerplate_heading` factor never fires for TOML/JSON/YAML (they
     always have `boilerplate: false` per `src/parse/mod.rs:429-447`),
     so *nothing* in category 4 should come from that factor.
     `documented_contribution` treats both `Heading` and `DataSection`
     as inherently documented, so no diff from that either. If a data
     fixture diff isn't attributable to (4a)–(4c), it's likely a real
     regression and the Agent should halt-and-ask.
   
   **Scheduler auto-commit note**: today's `is_auto_commit_body` at
   `src/schedule.rs:167-170` only fires `HeadingBody { level: 1 }`
   auto-commit for README H1 bodies at the root directory (the condition
   requires `FileRole::Readme`). Data files have role `Normal` (or
   similar), so they don't hit this path today even though they're
   keyed as `HeadingBody` currently. After the routing fix,
   `HeadingBody { level: 1 }` is exclusively markdown README bodies —
   which matches the condition's intent. `is_auto_commit_body` needs
   only a pattern-match syntax update for the new struct shape, no
   semantic addition.
   
   Affected fixtures: any with `.toml`, `.json`, `.yaml`, `.yml` files.
5. **`dedup_overloads` flat-stream → AST-sibling semantic change** —
   in rare cases where two same-name captured items were previously
   separated only by uncaptured AST nodes (comments, unrelated
   constructs), today's flat-stream check treats them as adjacent and
   dedups one, while AST-sibling check treats them as non-adjacent and
   keeps both. Expected to be rare; if fixture diffs are widespread,
   halt and reconsider.
6. **Heading nesting: per-group → per-individual parent selection.**
   Today's `nest_heading_groups` (`src/group/files.rs:113-205`) operates
   on heading *groups*: all level-3 headings in a file form one group,
   and that group picks a single parent group at level 2. Parent
   selection prefers non-boilerplate on ties. After the refactor,
   `Heading::from_parse` picks parents **per individual heading** in
   source order, so a file like:
   ```
   # Title           # H1
   ## Setup          # H2 non-boilerplate
   ### Step 1        # H3 → picks Setup (previous H2)
   ### Step 2        # H3 → picks Setup
   ## License        # H2 boilerplate
   ### Terms         # H3 → picks License (previous H2)
   ```
   Today Terms nests under Setup (group-level non-boilerplate tiebreak).
   New: Terms nests under License (previous H2 in source order).
   The new behavior is more semantically correct — Terms IS a
   sub-heading of License in the source — but produces a real snapshot
   diff on markdown fixtures with interleaved boilerplate headings.
   Widespread diffs here indicate the markdown fixture structure
   actually cares about the change and the Agent should flag them for
   review as potential *improvements* (the new output may communicate
   the markdown structure better). If the Agent judges any diff as a
   regression, halt.

An Agent reviews every stage 7 diff and confirms each is explained by one
of these six categories. Agent reads as a fresh user, checks for
understanding regressions, not just diff existence. Halt if any diff is
unexplained or any concern surfaces.

At every other stage, **zero snapshot diffs**. Any diff is a bug.

`debug_assert!` is **not** considered a verification step on its own,
because `cargo test-release` doesn't run debug assertions. Anywhere the
plan adds a debug assert, also rely on a positive test that would catch the
same condition.

## Critical files

- `src/group/ts.rs` → split into
  `src/group/ts/{mod,kind,spawn,function,struct_,enum_,class,interface,trait_,type_alias,const_,macro_,module,import,impl_block,heading,data_section}.rs`
- `src/heuristics.rs` → renamed to `src/calibration.rs`; central match +
  grounding comments + consts + `compute_item_modifier` + `finalize_group`
- `src/parse/mod.rs` — shrunk to `dispatch_kinds` + per-language pre-pass
- `src/parse/file_ctx.rs` (new) — `FileCtx` + `FileCtxExtras`
- `src/parse/ast.rs` — `is_top_level` + `Lang::is_wrapper_kind` (or new file)
- `src/parse/module_doc.rs` — content moves into `src/group/ts/module.rs`,
  file deleted
- `src/schedule.rs` — `is_auto_commit_body` pattern updates for the new
  struct shapes. (Tiebreak tuple unchanged — see stage 0.)
- `src/group/files.rs` — restructured pipeline; gating loop and
  `nest_heading_groups` deleted; `compute_item_modifier` moved out
- `src/group/mod.rs` — `Group::children` drain (wraps the trait dispatch
  to drain `dependent_siblings`)
- `src/group/aggregate.rs` (new) — `aggregate_across_files`, `merge_into`,
  `apply_dedup_line_overlaps_chainer`
- `Cargo.toml` — add `enum_dispatch` (verify modern equivalent). `strum`
  is no longer needed since stage 0 was dropped.
- `design.md` — §3.3/§4 trait shape; §10 LineEntry name fix
- `issues.md` — close out #14, #17 (partial), #18, #13, #5; possibly file
  follow-ups (ImplBlock-trait gating, Mod::children re-running from_parse)
- `output-issues.md` — file Java fields silently dropped, possibly TS class
  fields too

## Verification

1. `cargo test-release` after **every** stage. Strict byte-identical except
   stage 7.
2. **Stage 7 Agent review**: spawn an Agent to read every snapshot diff
   and confirm each is explained by one of the six accepted categories
   (see "Snapshot policy"). Agent reads as a fresh user, checks for
   understanding regressions, not just diff existence. Halt if any diff
   is unexplained or any concern surfaces.
3. `cargo bench-hot` at the end. No regression expected.
4. `cargo run --release -- .` and read precis's self-output as a fresh
   reader, per CLAUDE.md.
5. `cargo run --release -- src/group/ts/function.rs` and similar — confirm
   the new per-kind files render reasonably under precis.
6. `/simplify` on the resulting diff per `feedback_simplify_step.md`.
7. `codex-companion plan-review` of this plan file before exiting plan mode
   (per session-start hook). **Already run four times during plan
   construction; this is the comprehensive revision.** Re-run after writing
   this version.

## Excluded from this batch

- **ImplBlock-trait gating cleanup** ("trait ImplBlock should not be
  gated") — snapshot-changing; deferred. Will be trivial to do as a
  follow-up because `gates`/`is_gated_by` logic moves into per-kind
  `from_parse`, where ImplBlock can simply not produce dependent_siblings.
- **Architecture cluster** (#21 Group→trait, #23 childless_folders refund,
  #25 render↔group cycle, #26 SchedulerRenderer asymmetric surface) — coherent
  on its own, saving for the next batch.
- **#19 ParseStore lifecycle, #20 classification string-matching, #24
  silent unwrap_or fallbacks elsewhere** — separate concerns.
- **Bugs** (#3 V4 byte budget pre-check, #4 A1 auto-commit bypass) — both
  touch the scheduler/value model, not this batch.
- **Java/TS field surfacing** — silent drop is a real bug (documented in
  `output-issues.md` during stage 10), but fixing it requires
  `ClassName::children` to extract fields the way it currently extracts
  methods. Deferred per user instruction.
- **Mod::children re-running from_parse** (user's idea: re-run from_parse
  with a filter for nodes inside the module, no duplication) — interesting
  but out of scope. Note in `issues.md` for future consideration.

## Known unknowns

These don't block the plan but should be resolved during implementation. If
any turn out to be load-bearing in a way that invalidates the design, **halt
and ask the user.**

- **`enum_dispatch` modernity.** Verify before depending. If the chosen
  crate doesn't support our trait shape (receiver methods only, no
  associated functions), halt.
- **Exact wrapper-kind list per language.** The lists in this plan are
  guesses based on tree-sitter grammar conventions. Verify against actual
  parse trees of fixture files in stage 5. If a fixture parse tree contains
  a wrapper-kind we didn't list, snapshot tests will catch it loudly
  (default-deny means missing wrappers cause regressions, not silent
  over-capture).
- **Exact contents of `FileCtxExtras` per language.** JS needs exported
  names; Rust may need crate-root context; Python decorated-def handling
  probably stays per-node and needs no extras. Verify during stage 6 by
  inspecting which language-specific helpers each ported `from_parse` ends
  up calling.
- **Whether `children` should take `&TsGroup<'s>` or `(&TsGroup<'s>, &GroupCtx<'s>)`.**
  Depends on whether spawn logic needs `GroupCtx`. Check during stage 8.
- **Cross-file aggregation with mismatched dependent_sibling structure.**
  If two files produce groups with the same key but structurally different
  dependent_siblings (e.g., heading nesting trees that don't merge cleanly),
  the recursive merge may produce surprising results. Audit during stage 7
  — if surprising, halt.
- **`dedup_line_overlaps` cap = 5 may be too aggressive or too lax.** First
  pick is 5; revisit if fixtures show the wrong cliff.

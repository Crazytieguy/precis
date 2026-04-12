# Post-rewrite issues

Outstanding issues from the architecture rewrite that need addressing
before calibration.

## Must fix (design violations)


## Should fix (quality/performance)

### Remove semantically dishonest `name` field and `name.rs`

`ExtractedItem.name` and `src/parse/name.rs` compute a "name" for
every item kind, but most items don't have a meaningful name. Impl
blocks don't have a name — they have a type (and maybe a trait).
Imports don't have a name — they have a module path. Module docs
don't have a name at all. Go grouped `const (...)` declarations
don't have a name either. The `name: String` field papers over
these distinctions with a universal abstraction that then has to be
special-cased everywhere it's consumed.

The fix is NOT to replace `name: String` with a `find_name_child_node`
helper that centralizes "give me the name-ish thing" logic. That
keeps the same bad abstraction and just moves it — it still pretends
an impl block has a name and hides the dispatch inside a catch-all
function. The resulting code is hard to verify because each branch
is triggered by tree-sitter grammar strings (`impl_item`,
`type_definition`, `lexical_declaration`) rather than by the caller
that actually needs the information.

Fix: push the logic down to each consumer, and extend the group
taxonomy when it doesn't fit. Both `ExtractedItem.name` and
`TsItem.name` go away — TsItem should not store a name either. Any
name-like information needed by rendering is computed at render time
from the item's node.

- **Rendering.** `render_item` in `src/group/ts.rs` currently has one
  big match arm that handles `FunctionName | StructName | ... | ImplBlock`
  together and calls `find_name_prefix(line, &item.name)`. Break this
  up. Each group kind's rendering inlines the node traversal it needs.
  `FunctionName` reads the function's name identifier. `StructName`
  reads the struct's name. `ImplBlock` is not about a name at all —
  it renders `impl Trait for Type` (or `impl Type`), which is its
  own concern and should have its own branch. Splitting `TsGroupKey`
  further is fine — and often helpful — when it narrows down the
  node type of items in a group so render code can assume a
  predictable shape (language-based splits are the obvious example).
  But `TsGroupKey` is a grouping discriminant, not a place for
  per-item data: a group holds many items, so anything that varies
  between items in the same bucket belongs on the item, not the key.
- **Visibility.** `determine_visibility` takes `name: &str` for Go
  uppercase checks, Python `_` checks, C `_` checks, Lua dot checks.
  These are all language-specific and the check is trivial — inline
  it at the call site inside `extract_items`, where the identifier
  text is easy to get (either from the `@name` capture that's in
  scope or by reading the relevant child node — your call).
- **Reexport detection.** `mark_reexports` uses
  `name.starts_with("self::")` as a proxy for "is this a relative
  import path". It should operate on the import node's text directly,
  not on a computed name.
- **Dedup.** `dedup_overloads` in `postprocess.rs` does two things
  for Rust, C, and JsTs: (a) if two consecutive items have the same
  name and kind, drop the earlier one — this handles function
  overload sequences like a C header's forward declarations followed
  by the definition, where keeping the last one gives the most
  complete version; (b) if two non-consecutive Struct/Enum/TypeAlias
  items share a name, drop the later one — this handles Rust
  `#[cfg(...)]`-gated duplicates of the same type. Both uses want a
  hashable identifier, not a "name" abstraction. Compute it inline
  from the node where dedup happens, or store a narrower field if
  that's simpler.
- **Section classification.** TOML depth and markdown boilerplate
  detection currently go through `name`. TOML has a key node with
  dots to count; markdown has `inline` content to classify. Read
  those directly.

After the fix, `ExtractedItem.name` and `TsItem.name` should both be
gone, `src/parse/name.rs` should be deleted, and no single function
should encapsulate "the name of an item" — there isn't one.

**Verification.** Scheduling is sensitive to subtle cost changes,
so check snapshot diffs carefully after the rewrite. A prior attempt
produced changes worth looking at: thiserror's `src/private.rs`
rendered empty (the four `pub use` re-exports were all gone),
superstruct's `src/struct.ts` lost the full `Struct` constructor
props signature, and enclosed lost ~20 TypeScript import lines
across several files. These may be real regressions from cost
redistribution, or they may be uncovering pre-existing scheduler
sensitivity that the old name-based prefix was accidentally masking.
Worth diagnosing either way before committing.


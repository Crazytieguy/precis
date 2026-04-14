# Issues

General bug / design / cleanup tracker, distinct from `output-issues.md` (which
tracks output-quality regressions specifically). Most entries here came out of
the 2026-04-13 design-invariant audit against `design.md` §10.

## Bugs

### 1. D5 violation — Python function docstrings rendered twice

**Invariant:** D5 — within-item containment. Sibling groups under the same item must not share source lines.

For Python functions with a docstring, `FunctionDocFirst` / `FunctionDocRest` and `FunctionBody` both render the docstring lines. They are cousins in the spawning tree (not parent/descendant), so D5 is violated for every documented Python function.

Root cause: `FunctionBody` renders `[body_start .. body_end)` with no docstring skip, unlike `ClassBody` which calls `skip_leading_docstring` and caps `effective_end` at `find_first_method_line` (see `src/group/ts.rs:568-611`, used only in the `ClassBody` render branch at `:755-781`).

**Fix:** Apply the same guard to `FunctionBody`. The helper already exists; extend its call-site to cover `FunctionBody` when the item is Python and has a docstring.

### 2. V3 stale `marginal_cost` when cached across iterations

**Invariant:** V3 — candidate cost measured by actually rendering the scheduled set plus the candidate.

`CachedGroupRender.marginal_cost` is computed once in `ensure_cached` (`src/schedule.rs:370`) and reused on every subsequent probe (`:213`) without refreshing. If another group commits lines between first probe and eventual commit of this candidate, the cached cost is stale relative to the true "scheduled set plus candidate" cost.

**Known assumption:** The current design relies on sibling groups not outputting the same source lines. Under that assumption the staleness doesn't matter, because no later commit can overlap this candidate's lines. This assumption should be made explicit in `design.md` (either in V3 or in R4/R5's context), since it's load-bearing for the caching optimization.

**Action:** Document the "siblings don't overlap lines" assumption in `design.md`. If the assumption is ever weakened (e.g. a new group type that could overlap siblings), the cached `marginal_cost` will need invalidation on overlapping commits.

### 3. V4 clause (b) not implemented — byte-budget fast pre-check missing

**Invariant:** V4 — "Byte budget is a fast pre-check" before running the token counter.

No `byte_budget` pre-check exists anywhere in the crate. Tokens and chars are computed together inside `FileCache::marginal_cost`, and `char_budget` is consulted only *after* tokenization at `src/schedule.rs:58-65` (as a secondary acceptance gate, correctly implementing clause c).

**Desired behavior (from discussion):**
1. Before running the tokenizer over a group's `LineEntry`s, sum their byte lengths and use a conservative heuristic (e.g. 16 bytes/token) to estimate whether the group could possibly fit in the remaining token budget. If not, reject without tokenizing.
2. When `--char-budget` is set, also check char-budget fit before tokenizing — a group that can't fit under the char budget shouldn't incur tokenization cost either.

Both checks should live in the render/probe path (`src/schedule.rs` probe_cost → `src/render.rs` marginal_cost), before any `format::count_tokens` call.

### 4. A1 scheduler encapsulation — auto-commit bypass

**Invariant:** A1 — greedy choice: the scheduler picks the frontier group with the best `value / cost` ratio.

`src/schedule.rs:123-150` auto-commits `EnumBody` and root `HeadingBody{1}` groups bypassing ratio ordering entirely. This is a real violation of A1's letter.

The mechanism has four gates: kind match (`EnumBody` or root `HeadingBody{1}`), value ≥ 1.0, >75% budget remaining, and for `EnumBody` a compactness cap (≤25 lines). Of these, only the budget-phase gate is genuinely time-dependent; kind/value/compactness are all expressible via `base_value`.

The root cause is calibration: multi-line bodies have ~10× worse ratios than cheap `FunctionName` entries (1.5 / 20 ≈ 0.075 vs 1.0 / 2 = 0.5), so they lose ratio competition. A `base_value` boost proportional to the ratio gap would fix this cleanly, but cascades across snapshots unpredictably (see `output-issues.md` #18 and #9 for the cascade history — any broad-impact value change tends to regress ~20-50 snapshots before it helps the target ones). The bypass was adopted as a targeted escape hatch that only fires in narrow conditions, avoiding the cascade at the cost of A1.

**Preferred fix:** Boost `base_value` for `EnumBody` and root `HeadingBody{1}` until they win ratio competition honestly, then delete the bypass. The snapshot cascade from the value change is the real obstacle — any fix must be paired with a careful calibration pass. The budget-phase gate (>75% remaining) is the only piece that a value model can't express naturally; if it turns out to be load-bearing (i.e. these bodies should commit *early* but not late, even at a better ratio), that's a genuinely new constraint on the model worth discussing.

## Design-doc drift

### 5. `LineEntry` variant names out of sync with code

§10 (R1, R4, R5) and §3.4 use `LineEntry::Full` / `LineEntry::Prefix`. The code uses `Complete` / `Truncated` (`src/render.rs:13-20`). Semantics match; names don't. Fix by updating `design.md` to match the code.

### 6. V3 cost-caching assumption not documented

See bug #2. The "siblings don't overlap lines" assumption that makes cached `marginal_cost` correct is not stated anywhere in `design.md`. Add it as part of V3 or in the R-section context.

## Cleanup

## Minor / nice-to-have

### 10. `Heading { level: u8 }` not constrained to 1–6

`TsGroupKey::Heading { level: u8 }` and `HeadingBody { level: u8 }` (`src/group/ts.rs:80-81`) accept any `u8`. A dedicated `HeadingLevel` enum or a `NonZeroU8` with runtime clamping at construction would make invalid levels unrepresentable (P1 flavor).

### 12. Wire up granular profiling

Performance improved substantially for big repos after the rewrite, but detailed profiling was never re-wired against the new architecture.

Done: `cargo nextest` release-mode test wiring is in place, and `cargo bench-hot` now covers regular fixtures plus large perf-fixture repos.

Remaining: add per-stage profiling so `profile` reports more than wall-clock render time.

### 13. Out-of-bounds line fallback in `render_item`

Several branches of `render_item` use `lines.get(idx).copied().unwrap_or("")` when a line index is out of bounds (`src/group/ts.rs:703,712,729,746,774,790`). The `""` fallback is a `&'static str`, not a slice of `item.source`, and silently papers over a bug that shouldn't occur in well-formed data. Replace with `debug_assert!` or reshape the loops to avoid needing a fallback.

## Architecture

Items in this section came out of the 2026-04-13 "cold architecture roast" — one fresh Claude agent and one Codex task, both reading `src/` without `design.md`. Each item below was flagged by at least one of them; items marked **[convergent]** were raised independently by both.

### 14. `TsGroupKey` is a god enum [convergent]

44 hand-maintained variants covering every combination of syntactic construct × doc/body/sig part × visibility × documented-ness × a few more discriminants. Every new construct or language feature requires edits in 3-6 parallel match statements: `ordinal()` (hand-assigned magic numbers `0, 1, 2, 10, ..., 110, 120`), `doc_rest_key`, `is_gated`, `is_gated_by`, `ts_base_value` in `heuristics.rs`, the spawn logic in `TsGroup::children`, and the render branches in `render_item`. Forgetting one is silent.

The structure is really a product: `(SymbolKind, Part, Visibility, Documented)` with a few specializations. Decomposing it would collapse `ordinal()` entirely (derive from the tuple), eliminate `doc_rest_key` (trivial), shrink `is_gated*` to a single predicate on `Part`, and let `ts_base_value` become a small lookup table. `src/group/ts.rs:17-86` is the enum definition; the parallel matches are spread across that file and `src/heuristics.rs:34-93`.

### 17. `heuristics.rs` is ungrounded magic numbers [convergent]

Sixty-odd float literals with no provenance: `powf(0.75)`, depth factors jumping `1.0 → 0.7 → 0.55 → 0.4`, `deprioritized_factor = 0.2`, `type_declaration_factor = 0.15`, `header_factor = 2.5`, `companion_header_factor = 0.3`, `boilerplate_heading_contribution = 0.1`, `reexport_contribution = 0.1`, `COMPACT_BODY_LINE_LIMIT = 25`. No comments explaining why that number and not another.

This directly violates the project's own design principle (CLAUDE.md: "Every value judgment must correspond to a real, articulable difference"). A reader debugging a bad precis output has no way to tell which numbers are load-bearing versus guesses that stuck. At minimum: add a short justification comment next to each literal (a sentence about what the number is doing and what alternatives were considered/ruled out). Better: move the table to a declarative data structure where the rationale is a field.

### 18. Kitchen-sink modules [convergent]

Two modules have absorbed too much responsibility:

- **`FilesGroup::children`** (`src/group/files.rs:11-113`) is ~100 lines that read files, apply generated-file heuristics, run tree-sitter, bucket items, wire dependent siblings, and nest markdown headings. Parsing, semantic classification, prioritization, and gating are fused into one loop, which makes the hot path hard to profile or swap piecemeal.
- **`parse/mod.rs`** is 1266 lines, larger than the scheduler + renderer + `lib.rs` combined. It hosts language-specific branches (`if lang == Lang::X { ... }`) scattered through functions claiming to be generic — C++ template handling, Rust test detection, Rust anon-const, Go/Rust `_` skip, JS export name collection, markdown-only allocation gate. It wants to be a per-language dispatch table.

Split `FilesGroup::children` into a parse/classify/bucket pipeline with distinct stages. Extract per-language quirks from `parse/mod.rs` into a `LangHandler` trait with one impl per language.

### 19. `ParseStore` memory lifecycle

`store_source` interns file contents even on parse failure, and the `FrozenMap`s are append-only with no eviction — fine for a one-shot CLI, but broken for any daemon or long-lived process. The plugin path (`precis` running under Claude Code as a hook) plausibly hits this.

Done: parsers are cached per extension inside `ParseStore`, so `parse_source` no longer constructs a fresh `tree_sitter::Parser` for every file.

Remaining: add an eviction strategy for the source/tree maps (either LRU or scoped per render call).

### 20. Classification via inline string-matching

`src/classify.rs:21` and surrounding lines identify file roles by grepping filename substrings in code (`"claude"`, `"windsurf"`, etc.). Adding a new AI config convention, new CI tool, new changelog format, etc. means editing classification code and recompiling. The classification tables should be externalized — either as a declarative const table at the top of `classify.rs` for easy amendment, or (bigger change) as a data file the binary loads at startup.

### 21. `Group` enum should be a trait

`Group` is a 3-variant enum (`Folders`, `Files`, `Ts`). Every method is a 3-arm match: `value`, `children`, `first_path`, `first_line`, `kind_ordinal`. The scheduler adds four more matches of its own (`probe_cost`, `commit_group`, `ensure_cached`, `is_auto_commit_body`), each with slightly different field access patterns and `.unwrap()` calls on variant-specific `Option`s.

This is a textbook case of a sum type pretending to be a trait. Convert to `trait Group { fn value(&self) -> f64; fn children(...) -> Vec<Box<dyn Group>>; ... }` with three impl structs. Related to #15 and #16: all three depend on the scheduler not needing to know the concrete variant to do its job.

### 23. `childless_folders` refund hashmap

`TextRenderer` maintains `childless_folders: HashMap<PathBuf, FileCost>` as a bookkeeping structure to "refund" folder line costs when a descendant file group commits and the folder's bare entry is subsumed. This exists because folder lines are still committed eagerly against the budget and later removed when child content supersedes them. The renderer's `probe_cost` for `Files` has to peek at this map, and `commit` for `Files` has to remove-and-refund.

The underlying issue is that folder costs are modeled wrong: a folder line should be a placeholder whose cost converts into real content when a child is scheduled, not a committed entry that needs a refund mechanism. Fixing this likely means deferring folder-line charging until assembly, where the final set of scheduled groups is known.

### 24. Silent `unwrap_or` fallbacks on invariant paths

Several invariant-sensitive paths degrade silently when an operation fails:

- `path.canonicalize().unwrap_or_else(...)` in `src/lib.rs` — falls back to the un-canonicalized path on IO failure, producing different output than the canonical case.
- `ScheduleCtx::rel_path` returns `unwrap_or(path)` — falls back to the absolute path when relativization fails.

For a tool whose core job is grounded prioritization and reproducible output, "if this fails, use whatever" is the wrong default. These should either bubble the error up (most honest) or log/assert at least in debug builds (fail loud in tests, degrade in prod). The current behavior is a quiet correctness hole.

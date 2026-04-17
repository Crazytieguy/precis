# Issues

General bug / design / cleanup tracker, distinct from `output-issues.md` (which
tracks output-quality regressions specifically). Most entries here came out of
the 2026-04-13 design-invariant audit against `design.md` §10.

## Bugs

### 3. V4 clause (b) not implemented — byte-budget fast pre-check missing

**Invariant:** V4 — "Byte budget is a fast pre-check" before running the token counter.

No `byte_budget` pre-check exists anywhere in the crate. Tokens and chars are computed together inside `FileCache::marginal_cost`, and `char_budget` is consulted only *after* tokenization at `src/schedule.rs:58-65` (as a secondary acceptance gate, correctly implementing clause c).

**Desired behavior (from discussion):**
1. Before running the tokenizer over a group's `LineEntry`s, sum their byte lengths and use a conservative heuristic (e.g. 16 bytes/token) to estimate whether the group could possibly fit in the remaining token budget. If not, reject without tokenizing.
2. When `--char-budget` is set, also check char-budget fit before tokenizing — a group that can't fit under the char budget shouldn't incur tokenization cost either.

Both checks should live in the render/probe path (`src/schedule.rs` probe_cost → `src/render.rs` marginal_cost), before any `format::count_tokens` call.

### 4. A1 scheduler encapsulation — auto-commit bypass [expanded 2026-04-16]

**Invariant:** A1 — greedy choice: the scheduler picks the frontier group with the best `value / cost` ratio.

`is_auto_commit_body` in `src/schedule.rs` now covers five kinds: `EnumBody`, root `HeadingBody{1}`, `TypeAliasBody`, `ConstBody`, and `FunctionSig`. It bypasses ratio ordering entirely — a direct violation of A1.

**Hardcoded caps of the same family:** `MARKDOWN_H1_BODY_LINE_CAP = 12` in `src/group/ts.rs` and `COMPACT_BODY_LINE_LIMIT = 25` in `src/calibration.rs` are rendering/scheduling caps that serve the same purpose: making bodies win when they wouldn't under the value/cost model. Both are escape valves on top of a miscalibrated model.

**Root cause:** `ts_base_value` gives each body kind a constant (EnumBody = 1.5, StructBody = 1.2, etc.) *independent of content size*. Cost scales linearly with lines, so ratio ≈ constant / lines — it drops as bodies grow, and no constant boost can fix both small and large bodies at once. Prior attempts at per-kind base_value boosts cascaded across 20–50 snapshots because a boost that makes big bodies competitive makes small ones dominate.

**Preferred fix — principled replacement:** Make body value a function of rendered content size, not a per-kind constant. Candidates to iterate on:

- Linear in lines: `value = per_kind × rendered_lines`. Ratio is constant across sizes, kind-ranked. Probably too aggressive for big bodies.
- Sublinear in lines (e.g. `sqrt`): diminishing returns per line. Matches intuition that line 20 adds less than line 2.

Use the *rendered* line count, not the full AST extent — bodies that the renderer caps should cost and value based on what's actually emitted.

The right test of principledness: if the curve shape is right, `is_auto_commit_body` and both hardcoded line caps become redundant and can be deleted. Retuning the per-kind constants against the fixture set is fine (that's a semantic preference statement, inherently empirical) — *adding branches/caps/hatches is not*.

Big enough that it deserves its own session with a full A/B pass. Do not extend the auto-commit mechanism further in the meantime.

## Minor / nice-to-have

### 12. Wire up granular profiling

Performance improved substantially for big repos after the rewrite, but detailed profiling was never re-wired against the new architecture.

Done: `cargo nextest` release-mode test wiring is in place, and `cargo bench-hot` now covers regular fixtures plus large perf-fixture repos.

Remaining: add per-stage profiling so `profile` reports more than wall-clock render time.

## Architecture

Items in this section came out of the 2026-04-13 "cold architecture roast" — one fresh Claude agent and one Codex task, both reading `src/` without `design.md`. Each item below was flagged by at least one of them; items marked **[convergent]** were raised independently by both.

### 14. `TsGroupKey` is a god enum [convergent — largely done]

Stages 1-8 of the TsGroupKey refactor (see commits `0fd1fc5`, `e178646`) decomposed the parallel matches: each variant now owns a type implementing `TsGroupKindMethods`/`TsGroupKindParse`, and the dispatch is a thin one-line call per variant. What's left is the ordinal table (still hand-maintained — see #27) and the `base_value` lookup in `calibration.rs` (still flat, not derived from `(Part, Visibility, Documented)`). The remaining simplification is real but much smaller than the original diagnosis.

### 17. `calibration.rs` tuning is empirical, not principled [convergent]

Sixty-odd float literals with no provenance: `powf(0.75)`, depth factors jumping `1.0 → 0.7 → 0.55 → 0.4`, `deprioritized_factor = 0.2`, `type_declaration_factor = 0.15`, `header_factor = 2.5`, `companion_header_factor = 0.3`, `boilerplate_heading_contribution = 0.1`, `reexport_contribution = 0.1`, `COMPACT_BODY_LINE_LIMIT = 25`. They were walked to a local optimum against the fixture snapshot set; per-number justifications would be post-hoc.

**Partial fix (2026-04-16):** The file's docstring now flags the values as empirically tuned and warns that retuning tends to cascade across 5–40 snapshots — a caveat the tool was missing. Per-literal grounding is out of scope; the honest answer is "A/B the snapshots before touching any of these," which is what the docstring now says.

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
- `GroupCtx::rel_path` returns `unwrap_or(path)` — falls back to the absolute path when relativization fails.

For a tool whose core job is grounded prioritization and reproducible output, "if this fails, use whatever" is the wrong default. These should either bubble the error up (most honest) or log/assert at least in debug builds (fail loud in tests, degrade in prod). The current behavior is a quiet correctness hole.

### 25. `render` ↔ `group` modules cross-import

After the scheduler/renderer split decoupled `schedule` from `group`, the same circular shape now exists between `render` and `group`:

- `src/render.rs` imports `Group`, `GroupCtx`, and reaches into `crate::group::ts::render_entries(g)` directly inside `TextRenderer::prepare`.
- `src/group/mod.rs` imports `crate::render::{CachedGroupRender, FileCost}`; `src/group/ts.rs` imports `crate::render::LineEntry`.

So the data model and the renderer mutually depend on each other, and a hypothetical alternative renderer would still have to reach into `group::ts` to extract line entries. The previous trait extraction relocated the smell rather than eliminating it.

### 26. `SchedulerRenderer` trait surface is asymmetric

`probe_cost` returns `FileCost`; `commit` returns `BudgetDelta`. `FileCost` is an internal `FileCache` cost type that the trait should not be exposing in its public surface, and the asymmetry means refunds can only be expressed at commit time — a renderer with cost discounts can't communicate them at probe time, so the scheduler can't see them while choosing the next group. A non-text renderer that has no concept of refunds still has to construct `BudgetDelta`s on every commit.

### 27. Hand-numbered `ordinal()` magic table

`TsGroupKey::ordinal()` (`src/group/ts.rs`) and `Group::kind_ordinal()` (`src/group/mod.rs:123-130`) are a single numeric namespace used by the scheduler tiebreak tuple at `src/schedule.rs:73-83`. `Folders = 0`, `Files = 1`, and every `TsGroupKey` variant is hand-assigned a magic number (0..120 with gaps), so adding or reordering variants means editing a hand-maintained table.

The naive fix — derive `EnumDiscriminants` on `TsGroupKey` and wrap it in a `GroupKindRank { Folders, Files, Ts(...) }` enum with derived `Ord` — was explored in planning round 21 of the TsGroupKey refactor (Codex finding: see `ignore/refactor_status.md`). Ordered-derive puts all `Ts` variants strictly after `Files`, which changes the tiebreak ordering across groups wherever `(path, line, kind)` ties fire today. Not zero-diff safe under the current snapshot policy.

Cleanup approaches to explore:

1. Define a single `GroupKindRank` enum with hand-maintained variants that preserve today's exact ordering across `Folders`/`Files`/`Ts`. Deletes the magic numbers at the cost of one hand-maintained variant list instead of several parallel number tables.
2. Restructure the tiebreak to use `(path, line, lexicographic-kind)` and accept whatever fallout the ordering change produces. Needs a deliberate snapshot-review pass.

Either approach is a separate batch — keep it behind its own explicit tiebreak-preservation audit.

### 28. `ImplBlock` trait impl should not be gated

`is_gated_by` at `src/group/ts.rs:233` gates trait-impl `ImplBlock` groups behind inherent `ImplBlock` groups of the same type. This was load-bearing under the pre-rewrite cross-file gating loop, but the semantics it produces are wrong: a trait impl is a public API surface that deserves to be shown on its own merits, not hidden behind an inherent impl. Noted during the TsGroupKey refactor design discussion.

The fix is to stop producing the gating relationship in the first place. Under the post-refactor architecture (plan Stage 7), in-file gating lives inside per-kind `from_parse` impls as `dependent_siblings`, and `ImplBlock::from_parse` can simply never attach trait-impl groups as dependent siblings of inherent-impl groups.

Snapshot-changing cleanup; deferred from the TsGroupKey refactor batch because fixing it required the per-kind gating refactor that Stage 7 delivers. Once Stage 7 lands, this becomes trivial.

### 29. `Mod::children` could re-run `from_parse` with an in-module filter

Today each per-kind `from_parse` traverses the whole file once and applies `accept_top_level_symbol` to filter. When `Mod::children` (or any other container kind that spawns in-body child groups — Rust `mod_item` with body, TS `namespace`, etc.) needs to enumerate nested top-level items, it has to re-derive them with a modified scope.

Idea worth exploring: instead of duplicating extraction logic per container kind, re-run `dispatch_kinds` with a "treat `self` as the root" constraint — the same combined query walked against a subtree instead of the full tree, with `accept_top_level_symbol` reinterpreted relative to the subtree boundary. The filter logic already exists in `ast::is_inside_function` and the wrapper-list approach in Stage 5 makes this cleaner still.

Out of scope for the TsGroupKey refactor batch. Noted for a future session once Stage 5 lands.

### 31. CLAUDE.md Ownership section removed — monitor for regressions

Removed 2026-04-16. The section pushed "dedicate time every session to
maintenance," "the more work you do without needing intervention the
better," and similar nudges aimed at earlier Claude versions that were too
myopic about the immediate task. Current-session evidence suggests those
nudges aren't needed and may even crowd out signal.

If future sessions show behavior regressions — e.g. ignoring pre-existing
problems that deserve cleanup, or not updating README.md/CLAUDE.md when they
drift — reconsider reintroducing a lighter version. The specific lines worth
keeping if needed: "don't ignore problems because they're pre-existing," and
"keep README.md and CLAUDE.md current as you work." The rest was padding.

## Tooling

### 30. Agent-based A/B snapshot review workflow

When a refactor produces snapshot diffs across many fixtures, a blind pairwise
review decides net quality impact without the author reading each diff.

Ran twice to date:
- TsGroupKey refactor (2026-04-13): 21 wins / 7 losses / 8 ties
- TypeAliasBody / ConstBody / FunctionSig additions (2026-04-16): 33 wins / 9
  losses / 28 ties across 70 fixtures

Worth turning into a reusable skill. Rough opinions, open for discussion:

- Git worktrees are overkill; `git show <ref>:<path>` baseline fetch works.
- Scaffolding was ad-hoc Bash + Deno; if it becomes a reusable skill,
  scaffolding beyond a skill prompt should be a Rust bin or stay inline.
- One Agent per fixture; prompt-caching keeps the per-fixture cost reasonable.
- A skill could orchestrate: pick baseline ref, generate pair files with
  randomized A/B assignment, spawn per-fixture Agents, decode verdicts.

**Agent prompt iteration (open):** current prompts are generic ("which builds
a better mental model?") and agents project their own aesthetics. The
2026-04-16 hidden-files A/B round showed this plainly — agents flagged
`.vscode/` and `.claude/` as noise even though existence of such files is
genuine project-shape signal. Specific things the prompt should eventually
cover:

- A concrete grading rubric mapped onto the CLAUDE.md principles (follow-up
  actionability, upfront semantic grounding, presence-is-signal).
- How to check claims against the fixture ground truth, not against the
  reviewer's aesthetics.
- When to prefer inclusion over exclusion on marginal cases.
- Explicit pushback on "this is noise" without an articulated reason.

Once this workflow is solid, the "Improvement process" line in CLAUDE.md
should be updated to reference the A/B rubric instead of "look at real
output for real projects" as the primary test.

Not a priority until the next big refactor is ready for review, but the
prompt work is the first thing to tackle when it is.

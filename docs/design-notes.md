# precis v0.2 — design notes

> **⚠️ Agent-maintained.** This file is written and updated by Claude
> across many sessions; entries are notes-from-then, not edicts. They
> can be stale, partially right, or have been superseded by later
> decisions that didn't make it back here. **Verify anything
> load-bearing with the user before acting on it** — especially items
> that read as judgement calls, deferred TODOs, or recommendations.
> Concrete invariants and architecture should be checked against the
> code; this file is for things not visible there.

A living doc that captures cross-session design constraints, decisions, and
deferred work without bloating the per-session plan files. The codebase
itself is the source of truth for architecture and invariants; this doc is
for things that aren't visible from reading `src/`.

## Design philosophy

- **Make invalid states unrepresentable over validating with tests.** Prefer
  newtype wrappers, sealed enums, and constructor invariants to runtime
  checks that catch the same bugs after construction.
- **Release prefers invalid output to a panic.** Hot-path asserts are
  `debug_assert!`; release builds tolerate walker contract violations
  rather than abort. The end-of-run budget cross-check follows the same rule.
- **Plan files are ephemeral; design rationale lives in the repo.** Per-session
  plans (under `~/.claude/plans/`) shouldn't carry decisions that need to
  survive plan churn — those go here or in code / agent prompts.
- **Simplify over validate.** When a feature would need extra validation,
  consider whether collapsing the design eliminates the need.

## Honest rendering

`precis` output is always a verbatim subset of the source. Allowed transforms:

- **Full lines** with their source line number. `RenderedLine::Full(text)`.
- **Line-prefix + trailing ellipsis** (line shown partially, truncated at a
  syntactic boundary). `RenderedLine::Truncated(prefix)`.
- **Bare-ellipsis markers** emitted by walkers at specific source lines —
  `RenderedLine::Ellipsis`. The line number isn't rendered; it exists so a
  descendant batch can replace the ellipsis with real content at that line.
  Walkers decide where markers go, because only they know whether a gap
  means "more content here" vs. "line numbers already make this obvious".

No paraphrasing, summarization, or invented content under any circumstances.

## North Star process

- North Star documents (`tests/north-stars/<fixture>.toml`) are agent-drafted,
  human-reviewed, and **frozen** before implementation iterates against them.
  Once frozen they are the divergence test's ground truth; implementation
  changes do not edit them.
- Schema: declarative batches with spans + render specs. `FsGroup::entries`
  is a two-variant [`FsEntries`] enum: `All` (NS sentinel meaning "list
  everything under `parent`") / `Listed(Vec<PathBuf>)` (explicit child
  list; the walker always emits this form). `All` expands to `Listed`
  at load time via [`ns_loader`] using `fs_util::list_dir` — same
  utility the walker uses, so NS and walker see identical filesystem
  content.
- Public schema types live in `src/content.rs` (BatchContent + FsGroup
  + FsEntries + Span + Render) and `src/north_star.rs` (NorthStar +
  NsBatch). Walker/scheduler internals (Batch, BatchKey + variants,
  WalkerKey, ValueSignals, ResolvedBatch) stay in `src/batch.rs`;
  `EntryKind` is a renderer-internal detail in `src/fs_util.rs`.
  Loading/resolution lives in `src/ns_loader.rs`. Types files hold
  types only; loaders hold functions.
- `revision_pin` is the only thing binding an NS to its fixture revision.
  `load_ns_checked` enforces it; the `ns_pins_match_fixture_pins` test
  enforces it under `cargo t`.
- **Validation**: `simulate_ns` catches authoring bugs before render time
  (duplicate ids, span out-of-range, span inverted-range, span targets
  non-existent file, invalid `Truncated` regex, regex-no-match on source
  lines, non-ancestor line-overlap, growth-envelope breach, cap breach).
  All as `Violation` variants — no panics. See `tests/ns_simulate.rs`
  for representative coverage.
- **Growth envelope**: each batch's marginal cost must satisfy
  `cost_i ≤ 100 + 0.3 · cumulative_before`. Replaces the old 2× rule.
  Per-batch is too local; cumulative matches the author's intuition
  ("doubling aggregate on batch 2 is fine, doubling on batch 10 is
  bad") and doesn't force authors to inflate a small preceding batch
  to clear the path for a legitimately larger one later. Constants
  tunable via `ENV_BASE` and `envelope_max` in `src/ns_simulate.rs`.
- **Amendment protocol**: a frozen NS can be corrected with an explicit
  rationale + diff. Drift isn't allowed; deliberate amendments are.

## Divergence metric

`src/divergence.rs` — one report per fixture, holistic across budgets.
Run the walker once at `T_max = 10000` (scheduler prefix-monotonicity
means sub-budget behavior is a prefix of the T_max schedule), then
compare the full trajectory against the frozen NS.

- **Atom identity**: `Line(path, line)` | `Fs(parent, entry)`. Walker
  and NS atoms at the same identity intersect; byte-range is the grade.
- **Byte-range credit**: each atom carries a `byte_end` (Full = full
  line length; Truncated{pattern} = regex match end; Ellipsis = 0; Fs
  = 1). Credit = `min(walker.bytes, ns.bytes) / max(ns.bytes, 1)`.
  A Full walker atom fully satisfies a Truncated NS atom; a Truncated
  walker atom partially satisfies a Full NS atom. Walker rendering
  more bytes than NS asks for is fully credited but flagged (`over`).
- **Scores**: `Sim = ∫ w(t)·overlap(t) dt / ∫ w(t) dt` with
  `w(t) = exp(−t/τ)`, `τ = 2000`; `overlap(t)` averages credit over
  NS atoms *reachable at t* (ns-batch exp_t ≤ t), so low-t overlap
  isn't diluted by unreachable-by-construction atoms. Paired with
  headline counters: `Reached/Total`, `Early`, `Late`, `Partial`,
  `Missing`, `Used/Budget`. `Over` is a row annotation only (it's
  always fully credited, so counting it was tautological).
- **Report**: `tests/divergence/<fixture>.md`. Line 1 grep-able scores.
  Body: per-tier rollup (reached/partial/missing counts + avg credit
  per major id prefix), then an arrival ledger (one row per
  non-aligned-or-partial NS batch with `exp_t`, `reached_t`,
  `delta_t = reached_t - exp_t`, `credit`, `status` ∈ {aligned, early,
  late, partial, missing}, with `+over` suffix when walker
  over-rendered an aligned batch). Followed by a walker-waste section
  sorted by off-NS token spend (covers both pure-waste batches and
  batches that intersect NS but overspend on off-script atoms).
  Empty sections elide; perfect alignment yields a 1-line file.

Tunables live as module-level constants: `TAU`, `REACH_THRESHOLD`,
`MISSING_FLOOR`, `EARLY_FACTOR`, `LATE_FACTOR`,
`UNMAPPED_COST_THRESHOLD`. Adjust and regen baselines
(`UPDATE_BASELINES=1 cargo t`) to see the effect.

NS `exp_t` is the cumulative marginal cost of applying each NS batch
in order to one shared `RenderedTree` — same accounting as
`simulate_ns`. Under scheduler prefix-monotonicity (below), walker
sub-budget behavior is the prefix of the `T_max` schedule with
`cum_tokens ≤ t`, so the metric runs the walker once per fixture.

## Scheduler early-stop (prefix-monotone)

`best_exact` returns the top-ranked eligible exact batch *regardless of fit*.
Main loop: if the top-ratio item is a speculative, materialize it (free,
no budget cost). If the top-ratio exact fits, schedule it. If the top-ratio
exact doesn't fit — **stop**. No fallback to smaller batches, even if
present.

**Prefix-monotonicity consequence**: every decision taken at budget
`T_small` up to its stopping point is also taken at `T_large`, because the
only budget-sensitive check is the final "does it fit" step — up to the
stopping point at `T_small`, every scheduled cost also fits at `T_large`.
So `T_small`'s schedule is a true prefix of `T_large`'s schedule; sub-budget
snapshots can be obtained by slicing a single `T_max` run, and the
divergence metric runs the walker once per fixture rather than per-budget.

Tradeoff: when the top-ranked exact is too big, budget under-utilization
can be as much as one batch's cost. This is a deliberate pressure on
walker calibration (if a top batch consistently blocks small budgets,
that's a signal to split it or lower its rank) and on NS authoring (the
cumulative growth constraint — see "Constraint formula" — keeps NS
prefixes coherent at small budgets).

## Path-relative non-essential factor

`non_essential_factor` (in `src/value.rs`) takes a `(path, root)` pair
and matches dir-name components only after stripping `root`. The reason:
under test, fixtures live at `tests/fixtures/<name>/`, so an absolute
fixture path like `tests/fixtures/cmdk/cmdk/src/index.tsx` contains
the component `tests` — without root-stripping, every fixture path was
treated as non-essential and the 0.2× factor masked all real signal.
For real-user invocations, `root` is the project root the user passed,
so their own `tests/`, `examples/`, `website/`, etc. still get the
discount. WalkCtx exposes `non_essential_factor(path)` which forwards
with the run's root automatically.

If a future refactor tries to drop the `root` parameter — don't.
Verify any change against `cargo t schedule_order` to make sure source
files don't lose the relative ranking against test/website/etc dirs.

## JSON walker — package.json predecessor chain

The JSON walker emits four split candidates for `package.json`
(`Identity` / `Entry` / `Scripts` / `Dependencies`) chained sequentially
via predecessor edges (`Identity` ← `Entry` ← `Scripts` ←
`Dependencies`). Reason: a compact (one-line) `package.json` collapses
all four key categories onto the same source line; without the chain
the four batches would be sibling candidates claiming the same
`(path, line)` and trip the scheduler's non-ancestor-overlap
debug assertion. The chain order matches the typical value ranking
(Identity first), so for normal multi-line `package.json` files (where
the four batches produce disjoint spans) the chain doesn't displace
anything; for the compact case, later batches' renders override
earlier ones along the legitimate predecessor path.

## Cross-language vs language-specific concerns

Many concerns precis cares about are cross-language (value heuristics,
ranking signals, render conventions, structural priorities); only the parts
that genuinely depend on a language's grammar belong in language-specific
code. The exact abstraction for sharing between the two layers is deferred
until the Stage 4 ontology is concrete; the discipline meanwhile is:
**don't accidentally specialize cross-language code to a single language**.

`ValueSignals::depth_factor` is a current example of an abstraction worth
revisiting once we support more languages. It's the channel every walker
folds contextual/location priors into (fs depth, non-essential-dir
penalty, sibling count, entrypoint boost, filename conventions). Today
all walkers converge on roughly the same recipe, but the helper functions
live in `src/value.rs` (cross-language) while the composition happens
in each walker (`src/walker/<lang>.rs`). As more languages land, watch
whether the composition itself becomes a duplicated pattern that wants
to be shared, vs. whether languages diverge enough that a single scalar
is the wrong shape and we need richer per-signal location context.

## Deferred (pick up in later sessions)

### Revisit speculative-materialization
Speculatives are walker-emitted candidates whose cost hasn't been computed
yet; the scheduler materializes them on demand when their optimistic
upper-bound ratio exceeds the best exact's actual ratio. Originally
introduced for branch-and-bound correctness under the "fall back to a
smaller fitting batch" behavior; now that the scheduler is prefix-monotone
(no fallback), the speculative pool may be simplifiable. Yoav flagged he'd
like to revisit whether it's still earning its complexity.

### Data model
- **Cost shrink credit** — `cost_lines` uses `saturating_sub` so a refinement
  that makes content shorter never credits tokens back. Conservative wrt
  budget, distorts ranking. Refactor `Cost` to allow signed deltas when a
  North Star surfaces a real shrink case.
- **Borrowed line content** — `RenderedLine` text is owned `String`. A `&str`
  borrow into the source file would save allocations but propagate a lifetime
  through the entire batch graph + walker trait. Defer until a profile
  surfaces it as a real bottleneck.
- **Path newtypes** — `BatchContent` carries arbitrary `PathBuf`s. A
  `RootRelativePath` (or `DirPath` / `FilePath`) newtype with a private
  constructor would make "path outside the seed root" or "file path used as
  a directory" unrepresentable. Worth doing once the walker surface is more
  varied.
- **`ValueSignals` / `Cost` newtypes** — signals admit NaN / negative /
  infinite; `Cost { tokens, bytes }` admits absolute nonsense. A
  `FiniteNonNegativeSignal` + private-field `Cost` constructors would catch
  bad inputs at the boundary. Cheap; defer until something misuses them.

### TypeScript walker — body batch
- **`TsKey::ExportBody`** — DONE (v4 plan in `ignore/plan-ts-export-body-v2.md`,
  three plan-review rounds). Emits brace-stripped body interior of any
  function/class export with a multi-line `statement_block` body. Sibling
  of `ExportDoc` under `Export`; the two cover disjoint lines (JSDoc above
  vs body inside). Final shipped signal weights:
  `catastrophic_omission = 0.45 * k * boost`, `follow_up_minimization =
  0.9 * k` (clamped to 1.0), `zero_tool_call_understanding = 0.8`.
  Sim deltas across the 10 fixtures: mitt +0.020 (Used 3997→4868,
  Reached 13→19, tier-3 reached 0/8 → 6/8 — the structural win),
  ts-pattern +0.007 (P.union/P.not/P.when full bodies now reach +0.93,
  NonExhaustiveError class body reaches), superstruct flat Sim but
  Reached +3 (Struct class methods covered), cmdk flat Sim Reached +1,
  ky −0.004 within tolerance, vaul flat Sim Reached −4 (Root body
  displaces some Overlay/Content scheduling), Rust fixtures unchanged.
  Aggregate +0.022 Sim, +6 Reached. No fixture regressed past −0.03.
  **Deferred follow-ups** (in plan file):
  1. Lexical-with-fn-init (`export const X = () => {…}`) — DONE.
     `find_fn_init_body` walks the initializer (transparently through
     `parenthesized_expression` and `call_expression` first arg) and
     returns the inner `statement_block` body. `collect_export_lines`'s
     lexical_declaration arm truncates at the body's `{` row when
     present and falls back to the whole declaration otherwise.
     `export_body_rows`'s `Const` arm fires on the same predicate.
     Plan in `ignore/plan-ts-fninit-and-reexport.md`, three plan-review
     rounds + one adversarial review on the implementation (which
     caught a too-broad descent into trailing-position callbacks —
     fixed by gating on first-argument-only).
  2. forwardRef-wrapped callbacks (cmdk + vaul `React.forwardRef(...)`)
     — DONE. Module-private `const X = <fn-init>` whose name appears
     in any top-level value re-export clause (`export { X }` or
     `export { X as Y }`, but **not** type-only `export type { X }`
     / `export { type X }`) is now synthesized into the `Export` /
     `ExportDoc` / `ExportBody` triple at the const's start_line.
     `collect_local_value_reexports` builds the value-name set;
     `synthetic_export_name` gates on (single binding, name in set,
     fn-init body present); `locate_export_decl` is the single source
     of truth for both discovery and materialization, with a
     real-export-first lookup so same-line collisions
     (`const X = () => {...}; export { X };`) silently drop the
     synthetic. Sim deltas across the 10 fixtures: cmdk Sim
     0.307 → 0.272 (-0.035, just past the -0.03 threshold) but
     Reached 18 → 20 (+2 — Item / Group / Separator / Empty bodies
     now reach NS items 4.3 / 9.1 / 9.2 / 9.3 / 10.6). The Sim drop
     is a time-decay penalty: README sections still reach but later
     because the new body batches schedule earlier; rendered output
     is genuinely more useful (every component now has its JSDoc +
     signature, several have full bodies). vaul +1 Reached / -0.005
     Sim — within tolerance, structurally correct (Overlay / Content
     / Handle now split into signature + body batches; opens budget
     for `useScaleBackground` body to reach NS 3.11). Aggregate
     Reached: +2 across 10 fixtures; aggregate Sim: -0.0038 avg.
     **Calibration follow-up**: cmdk's bodies are dense
     (8 forwardRef components × ~600-tokens each clusters in the
     1000-7000 cum-tokens range). A future NS amendment that ranks
     the Item/Group/etc. bodies higher would re-credit the Sim drop;
     alternatively, a per-file sibling-count damping of body signals
     could limit the cluster's combined ranking weight. Defer until
     more TS fixtures land that exhibit the same pattern.
  3. Class-method splitting: v1 emits one `ExportBody` per class.
     Per-method splitting via `(file, class_start_line, method_name)`
     keys is deferred until a fixture surfaces it.

### Render
- **Rustdoc doctest-hidden lines** — DONE. `strip_hidden_doctest_lines`
  in `src/walker/rust.rs` drops `# foo` and lone-`#` lines inside Rust
  fenced code blocks (` ``` ` and `~~~`, default lang or
  `rust`/`no_run`/`ignore`/`compile_fail`/`should_panic`/`edition*`)
  within `///` and `//!` rustdoc. Filter runs *inside*
  `collect_module_doc_lines` before the lede/body heading-split — a
  crate doc that opens with a fenced example whose first line is
  `//! # use crate::X;` would otherwise mis-split on the doctest
  scaffolding (codex adversarial review caught this; integration test
  in `rust_module_doc_split_handles_fence_before_first_real_heading`).
  Sim deltas: anyhow Sim flat (0.364) but Reached **12 → 14** (+2),
  Used **8871 → 9932** (+1061 tokens recovered); tier 1 reached
  4/5 → 5/5, tier 4 reached 0/7 → 1/7. crate-doc body waste 2368 → 1564.
  log Sim flat with Used −10. Other Rust fixtures unchanged
  (mdbook/otree have no `# `-hidden lines in their crate docs).
  TS/JS/JSON/TOML walkers unaffected (rustdoc-only convention). Plan +
  1 plan-review round (general-purpose Agent — codex-companion stuck
  in starting phase) + 1 adversarial review on the implementation
  (codex caught the lede/body split ordering bug).
- **Filesystem-level override** — file-content batch superseding a folder
  listing entry, "N more files" placeholders, alternate non-tree renderings.
- **Sub-section markdown splitting** — H2 sections are the unit of
  markdown batching (`MarkdownKey::Section { file, section_index }`).
  `logical_sections` unwraps a single top-level H1 into its H2
  children + a synthetic "intro" section #0 (covering H1 heading +
  pre-first-H2 prelude), so `# Title` READMEs (mitt, mdbook, otree)
  don't collapse into one multi-KB blob.
  **H3 splitting** — DONE. Content-heavy H2s (≥2 H3 children,
  ≥600 source bytes, file's outline emitted, non-changelog file
  class) are subdivided into one Intro (the H2 heading + body
  before the first H3) plus one H3Child per H3. The walker's
  internal `SectionRange { kind, parent_index }` carries the
  classification through both `expand` and `mat_section` so signal
  selection stays consistent. Intros and Whole ranges keep the
  parent's full signals; H3Children scale by `H3_CHILD_SIGNAL_SCALE
  = 0.45` (identical signals over-rank H3s once cost drops to
  per-H3 size). H3Child cost-hint is 1 (true lower bound on any
  non-empty rendered batch — required for the scheduler's
  speculative-bound contract; see `Candidate::cost_hint` doc).
  Empty H3 ranges (no body beyond heading) and heading-only Intros
  are filtered to avoid `ratio(value, 0) = INFINITY` no-op
  batches. Aggregate Sim delta across the 10 fixtures: +0.007
  (cmdk +0.006, mitt +0.005, otree −0.004, others within
  ±0.001). All Reached counts preserved. Plan + 3 plan-review
  rounds in `ignore/plan-h3-splitting.md`.
- **Bullet-list splitting** — DONE. An H2 whose non-decorative
  content is exactly one `list` block (with the standard size gates
  applied to the *substantive* item set: ≥`BULLET_MIN_ITEMS = 3`
  items, ≥`BULLET_MIN_LARGE_ITEMS = 2` items above
  `BULLET_LARGE_ITEM_BYTES = 200` source bytes, ≥`H2_SPLIT_BYTES`
  total, non-changelog file class, outline emitted) is subdivided
  into one optional `Intro` (when the H2's pre-list body is
  substantive — typically dropped, since the predicate's target
  shape is heading-immediately-followed-by-list) plus one
  `BulletItem` per substantive top-level list item. Decorative
  siblings around the list (`<br>`, image-only paragraphs) are
  tolerated by the predicate but not preserved by any emitted
  range — the headline / outline batches make the same trade.
  Constants `H3_CHILD_SIGNAL_SCALE` / `H3_CHILD_COST_HINT` were
  renamed to `SUB_SECTION_SIGNAL_SCALE` / `SUB_SECTION_COST_HINT`
  and now apply to both `H3Child` and `BulletItem` ranges.
  Sim deltas across the 10 fixtures: anyhow +0.001 (the 1125-token
  `## Details` body splits into 6 BulletItem batches of
  ~120-260 tokens; per-bullet ranking lets the scheduler interleave
  them with other batches), superstruct +0.003 (the previously-
  monolithic `### Principles` 304-token batch splits into 5
  numbered-bullet BulletItems, freeing budget for the
  `docs/reference/core.md` outline and `src/structs/refinements.ts`
  exports — both visible in the post-regen rendered snapshot).
  Other 8 fixtures unchanged. Plan + 3 plan-review rounds (codex)
  and one adversarial review on the implementation in
  `ignore/plan-bullet-split.md`. Codex caught two real issues
  during planning (decorative-trailing-block tolerance; conditional
  Intro instead of always-emit) and one during implementation
  review (selection vs emission item sets must be the same — fixed
  by having `should_split_by_bullets` filter to substantive items
  before applying the count/large-item gates).
- **Outline-Section ancestor coupling** — `MarkdownKey::HeadingsOutline`
  is the predecessor of every `Section` in the file when emitted.
  Without that edge, both batches would render the same heading rows
  and the renderer's non-ancestor-overlap rule would fire. The trade
  is gating: if the outline is too big to fit near the budget tail,
  the prefix-monotone scheduler stops and every section in that file
  is blocked. Today the outline self-suppresses past 30 headings or
  1500 bytes of heading content (whichever first); the architectural
  follow-up is a rendering scheme that lets outline + sections schedule
  independently (the codex 2025-04-25 plan-review thread on
  `ignore/plan-headings-outline-v3.md` is the discussion of record).
- **ReadmeHeadline: skip decorative-prose paragraphs** — DONE.
  `headline_spec` in `src/walker/markdown.rs` now skips leading
  decorative paragraphs (image-only / badge-only) and tag-only
  `<img>` HTML blocks immediately after the heading, and truncates
  the heading line itself when its tail is nothing but inline badges
  (cmdk shape: `# Project [![...]] [![...]]`). Decorative
  classification reparses the inline content with
  `tree_sitter_md::INLINE_LANGUAGE` because the block grammar leaves
  inline content opaque. Honesty preserved: classifier defaults to
  non-decorative on unknown inline shapes, plain text-link
  paragraphs (soluna's `[Live Examples](...)`) and autolink
  paragraphs are kept, link-only headings (`# [Project](url)`) are
  not truncated.
- **Anyhow Sim regressed −0.032 from this change.** Walker output is
  more honest (badges replaced with prose tagline), but the anyhow
  NS credits the *crate-doc lede* in `src/lib.rs:9-11` rather than
  the README tagline at `README:9-10`. The new headline batch
  (~63 tokens, mostly off-NS) now schedules at position 2 because
  it's far cheaper than before, which delays a few real NS atoms
  by ~260 tokens. Fixable by an NS amendment that lists the README
  prose lines as an accepted alternate location for batch 1.2, or
  by leaving as-is — the user-visible output is genuinely better.
  mdbook (+0.038), ky (+0.009), cmdk (+0.007), otree (+0.001) all
  improved; log/mitt/superstruct/ts-pattern/vaul unchanged.
- **Markdown headings-only batch** (analog of Rust `PubItemNames`) —
  DONE. `MarkdownKey::HeadingsOutline` collects every H1/H2/H3 row
  (skipping the H1 row already owned by `ReadmeHeadline` so the
  headline's `Render::Truncated` survives), gated to 2..=30 rows AND
  ≤1500 bytes of heading source. Predecessor of every `Section` in
  the file when emitted (see "Outline-Section ancestor coupling"
  above). Sim deltas across the 10 fixtures: mitt +0.039, cmdk
  +0.019, otree +0.012, superstruct +0.008, ky +0.007 (NS-modeled
  outlines); anyhow −0.006, log −0.014, mdbook −0.005 (NS doesn't
  model the batch, outline competes for budget); ts-pattern −0.002
  and vaul flat. Net positive; no fixture regressed past the 0.03
  abort threshold. Pairs naturally with the `PubItem`-style
  catastrophic rebalance below.

### TOML walker — workspace-aware sub-crate damping — DONE

`WalkCtx::is_workspace_member(file)` returns `true` for sub-crate
Cargo.tomls when the seed root declares a `[workspace]` table; the
member set is the union of (a) `[workspace].members` literal entries
plus trailing-`/*` globs and (b) auto-promoted local
`path = "..."` dependencies from `[dependencies]` /
`[dev-dependencies]` / `[build-dependencies]`. `[workspace].exclude`
is applied once at the end so it blocks both sources.

The TOML walker's `identity_signals` scales all three channels by
`WORKSPACE_MEMBER_IDENTITY_FACTOR = 0.4` (matches `ApiSurface::factor`
for `pub(crate)`) when the file is a member; `[features]` and
`[dependencies]` signals are unchanged (codex plan-review caught that
sub-crate dep tables describe the crate's role even when versions
are inherited).

**Critical guard**: path-dep auto-promotion only fires when a
`[workspace]` table exists. A non-workspace repo with a local
`path = "deps/foo"` dep must NOT damp `deps/foo/Cargo.toml`'s
identity. Codex adversarial-review caught this. Regression test:
`walker_toml_workspace_members_path_dep_without_workspace`.

**Honest scope** (intentional false-negatives — we never damp a
non-member):
- Only trailing-`/*` globs are honored. Mid-name globs
  (`crates/mdbook-*`), `**`, `?` patterns are not.
- `[workspace]` is only read from `<root>/Cargo.toml`.
- Path entries with `..` or absolute paths are skipped.

Lookup is memoized via `RefCell<HashMap<PathBuf, bool>>` on
`WalkCtx` so the canonicalize syscall in `is_workspace_member`
runs once per file rather than once per signal computation.

Sim deltas: mdbook +0.018 (Reached 17 → 18, eight `[package] in
crates/...` waste rows totaling ~650 tokens removed; freed budget
lets the `Builtin preprocessors` re-export listing reach a partial
0.78 and other source-layout / driver-trait crate batches arrive
~600 tokens earlier). Other 9 fixtures unchanged. Plan + 3
plan-review rounds + 1 adversarial review (which caught the
non-workspace path-dep bug pre-merge) in
`ignore/plan-toml-workspace-damping.md`.

The `Walker::expand` doc comment now acknowledges that walkers may
read source via `WalkCtx` `OnceCell` caches (rust module
visibility, toml workspace membership) — the previous "no I/O
beyond `read_dir`" wording was already false in practice. A future
refactor that moves these caches to an explicit pre-scheduler
metadata phase remains open.

### Scheduler / walker
- **File-as-seed** — currently rejected with a clear error in `lib.rs`.
  Needs a small content-only walker path, probably driven by a real
  content walker rather than a generic "show full file" fallback.
- **Multi-path seed** — the CLI accepts `Vec<PathBuf>` but `render()` uses
  only the first path. Multi-root scheduling (one budget across roots) is
  deferred.
- **Performance optimizations** — `best_exact` recomputes marginal cost for
  every batch on every loop iteration. Tokenizer has a thread-local cache
  of string→tokens that cuts the redundant work, but per-batch cost caching
  invalidated on paths-touched would cut it further. Defer until a larger
  fixture surfaces it.

### Stopping criterion / value function
- **Stopping criterion beyond "no batch fits"** — dynamic floor or
  value/cost threshold so we stop earlier when remaining batches are weak.
- **Per-category sublinearity** — `value::ratio` uses `value / cost^0.35`
  for every batch (gentler than `sqrt` — see the commit that moved off
  `sqrt` for reasoning). May want per-category shapes (e.g. hard cap on
  `CrateDocLede` size, gentler concavity on test-as-spec batches).
- **Signal-weight calibration** — `W_CATASTROPHIC = 1000`, `W_FOLLOW_UP =
  400`, `W_ZERO_CALL = 300` are first-pass. Calibrate from north-star
  divergence reports.
- **README section index decay** — DONE. `readme_section_signals`
  now scales all three channels by `(h2_idx + 1)^-0.15` (floored at
  0.7), where `h2_idx` counts real H2 sections only (the H1-unwrap
  `SyntheticIntro` at `parent_index = 0` is skipped, so a `# Title`
  README's first real H2 stays at factor 1.0 — guarded by
  `markdown_readme_index_decay_skips_synthetic_intro`). Motivation:
  README sections were the dominant off-NS waste pattern in 9 of 10
  fixtures; a mild decay tilts toward early sections (install /
  quick-start) without displacing late ones. Sim deltas across the
  10 fixtures: superstruct +0.020 (Reached 18→19), anyhow +0.015,
  log +0.012, otree +0.009, ts-pattern +0.004, ky +0.001, mdbook
  flat, vaul flat, cmdk −0.005, mitt −0.001. Aggregate avg Sim
  0.4044 → 0.4099 (+0.0055), Reached aggregate +1, no fixture
  regressed past −0.03 and no high-tier README-credited NS batch
  flipped from `aligned`/`late` to `missing` or lost credit. Plan +
  2 plan-review rounds (codex) in `ignore/plan-readme-section-
  index-decay.md` (round 1 caught the synthetic-intro index bug;
  round 2 caught an incomplete affected-fixture list — both fixed
  before implementation).
- **PubItem signal rebalance (catastrophic ↓, follow_up ↑)** — DONE
  with caveats. Final shipped numbers (Rust + TS in lock-step):
  `pub_item_signals`: catastrophic `0.85*k*boost → 0.70*k*boost`,
  `zero_tool_call_understanding 0.55 → 0.65` (follow-up unchanged at
  `0.85*k`, now explicitly clamped to 1.0 to honor the
  `ValueSignals` 0..1 contract — TS Default at k=1.2 was over the
  bound).
  `pub_item_doc_signals`: catastrophic `0.40*k*boost → 0.20*k*boost`
  (follow-up and zero-tool unchanged).
  Aggregate: avg Sim 0.3934 → 0.4029 (+0.0095) across 10 fixtures,
  with the biggest wins on log (+0.016), vaul (+0.072), ky (+0.005),
  and superstruct (+0.003). Anyhow stays at reached=12 by leaving
  PubItemDoc.fu/ztu untouched — codex caught that aggressive
  PubItemDoc cuts displaced load-bearing bodies (anyhow's `Error`
  doc carries 4 NS atoms about Display/Debug reprs).
  **Caveat 1**: ts-pattern tier-2 avg credit drops 0.35 → 0.20
  (-0.15) — the rebalance trades 1 tier-2 reach + 2 partials
  (`match()` and `isMatching` JSDocs) for 2 new tier-3 reaches; net
  reached count goes 14 → 15 and Sim only -0.002. The trade is real
  but small. Plan threshold C of -0.10 was tripped; treating the
  underlying Sim/reached as the user-relevant metric instead.
  **Caveat 2**: tried more aggressive variants (e.g.
  `PubItem.catastrophic 0.85 → 0.55`) which gained more aggregate
  Sim (+0.118) but dropped superstruct's central `struct.ts` Struct
  class body out of the 10k schedule (codex adversarial review
  flagged this). Settled on the milder catastrophic drop above as
  the better trade. The `entrypoint_boost * k` clamp at 1.0 means
  the change to `PubItem.catastrophic` is mostly a no-op for `lib.rs`
  Trait/Enum/Struct/Fn (still saturates at 1.0 except for `TypeAlias`
  where boost=1.4*k=0.85 is below 1.0 even before the change). The
  PubItemDoc catastrophic drop is where most of the user-visible
  effect lives.
  **Structural follow-up**: a real fix would be a scheduling
  invariant like "all sibling public-API bodies for a file outrank
  any docs in that file", or richer per-signal weighting that
  distinguishes content-rich from boilerplate doc bodies. Both
  bigger lifts; deferred.
- **Sibling-count devaluation** — when a file emits many per-item batches
  (e.g. a config module with 20 `pub struct` children), each one's
  individual value/cost ratio beats the value/cost of a single important
  body elsewhere (e.g. `CommandArgs` in `src/cmd.rs`), producing a
  "wide-but-shallow signature sweep" across deep files at the expense of
  root-level anchors. The old design collapsed all pub items per file into
  one batch which naturally dampened this; the split-per-item model is
  better for ranking precision but loses the dampener. A first attempt
  folded a `sibling_factor(n_siblings)` into `PubItem`'s `depth_factor`
  at walker expand time — but across 12 reviews it regressed more than
  it improved (anyhow_6000 +10, log_6000 +15, otree_3000 8→18 major
  ranking divergences — count-based measurements subject to the noise
  caveat in the Process section). The tradeoff is
  structural: dense core files (anyhow's `src/lib.rs` with ~25 pub items,
  log's `src/lib.rs` similar) are *legitimately* dense; penalizing them
  displaces their load-bearing bodies (Level/LevelFilter rustdoc,
  Chain/Context docs) in favor of content elsewhere that's not actually
  more valuable. Decoration-heavy dense files (otree's `src/config/colors.rs`
  with 7 color sub-structs) look structurally identical but have very
  different intrinsic value. `is_entrypoint_file` doesn't reliably
  distinguish them — `mod.rs` catches both tests/ helpers and real
  module roots. Needs a richer signal than sibling count alone. Plausibly
  pairs with an eventual NS-author update that ranks `PubItemNames`-style
  location hints as first-class, so the calibration target is clearer.
- **API-surface signal (`pub(crate)` / `pub(super)` / `#[doc(hidden)]`)**
  — DONE for the local-syntactic case. `Visibility { Public, Restricted }`
  + `doc_hidden: bool` carried on `PubItemInfo`; `ApiSurface::factor()`
  composes a uniform multiplier (0.4 per axis, stacking to 0.16) applied
  to all three signal channels in `pub_item_signals` /
  `pub_item_doc_signals`. Visibility is classified by trimmed text of
  the `visibility_modifier` node (covers `pub(self)` /
  `pub(in path::to)` without enumerating each form).
  `#[doc(hidden)]` is matched on the *direct* attribute path via
  structured AST traversal (`identifier` "doc" + `token_tree` containing
  the single `identifier` "hidden"); `cfg_attr(..., doc(hidden))` does
  not fire because its outer path is `cfg_attr`. Sim deltas across the
  10 fixtures: anyhow +0.012 (`pub(crate)` ChainState / ErrorImpl /
  ContextError correctly demoted; `#[doc(hidden)] pub trait
  AdhocKind/TraitKind/BoxedKind` in `kind.rs` collapse to header+`…`
  freeing budget for `Chain` rustdoc and Cargo `[dev-dependencies]`).
  Other Rust fixtures unchanged or within ±0.001 (mdbook drops 3 zero-
  cost PubItem batches). TS/JS fixtures unchanged (predicate doesn't
  fire). Aggregate Sim +0.0012 — modest but the change is structurally
  correct. Small gain because only anyhow has heavy `pub(crate)` use
  inside the budget-reachable depth; more gain arrives if a
  cross-file mod-visibility analysis lands (see deferred follow-up
  below).
  **Refactor follow-up**: extracted `any_outer_attribute(node, pred)`
  helper used by both `has_doc_hidden` and `has_macro_export` (was
  duplicated prev-sibling traversal). Structured `attribute` matching
  replaces text-substring parsing (drops `matches_doc_hidden`).
  **Cross-file mod-visibility — DONE.** Build a per-run map of
  `<root>/src/lib.rs` reachability via `mod x;` declarations; a file
  is `Public` iff a chain of `pub mod` declarations connects it to
  `lib.rs`, otherwise `Restricted`. Map is computed lazily and
  cached on `WalkCtx` via `OnceCell`. Applied to `PubItem`,
  `PubItemDoc`, **and** `PubItemNames` (a names-listing of
  internal-only items is structurally less valuable, same axis).
  Uses Rust 2018 module resolution (`mod_name.rs` then
  `mod_name/mod.rs`). Inline `mod x { ... }` blocks and `#[path]`
  attributes are not followed; the fallback for files under
  `<root>/src/` not in a populated map is `Restricted` (safer than
  `Public` — keeps resolver-miss edge cases from sliding back to
  full public weight). Files outside `src/` and the no-`lib.rs`
  case (binary-only crates) keep the `Public` default.
  **Sim deltas across the 10 fixtures:** anyhow +0.008
  (ptr/nightly/kind internal pub items demoted, freeing budget for
  lib.rs mod tree and Chain/ContextError struct fields); log flat
  (kv::Error / kv::Key/ToKey demoted because they're only re-exported
  via `pub use self::error::Error;` in kv/mod.rs — re-export
  tracking is the natural follow-up; in compensation kv::Value
  surface gains visibility); other Rust fixtures unchanged
  (mdbook seed root has no `src/lib.rs`, so map is empty);
  TS/JS fixtures unchanged.
  **Re-export tracking — DONE.** `pub use self::path::...`
  declarations in a `Public` file walk the path segment-by-segment,
  lifting each declared-and-resolvable child mod to `Public`. The
  walk is gated on a real top-level `mod x;` declaration in the
  current file at each step (codex plan-review round 2 caught
  this — without the gate, an orphan or `#[path]`-mounted file
  with a colliding name would be silently lifted). Grouped
  (`pub use self::{a::X, b::*}`), wildcard, and alias
  (`as Renamed`) clauses all flatten through the same recursive
  descent. `pub(crate) use ...`, `pub use crate::...`, and
  `pub use super::...` are not handled — only the unambiguous
  `self::` path root participates.
  **Sim deltas across the 10 fixtures:** log Sim flat (0.527 →
  0.527) but tier-4 partial 1 → 2 (kv::Key surface 0.34 →
  0.58, kv::Error variants 0.07 → 0.27). The partial-credit gain
  for kv::Key is the structurally-correct rebalance — it now
  reaches a names-surface batch at t=2881 (vs t=6956), a
  ~4000-token earlier arrival. The Sim metric's
  exp(-t/2000) weighting decays the tier-4/5 gains, so headline
  Sim moves don't reflect the user-visible improvement on the
  log fixture. Other Rust fixtures unchanged or within ±0.001
  (anyhow doesn't use `pub use self::...` for re-exports —
  internals are `pub(crate)` directly; mdbook seed root has no
  `src/lib.rs`). TS/JS fixtures unchanged. Plan + 3 plan-review
  rounds in `ignore/plan-reexport-tracking.md`.
  **Calibration follow-up:** because log's tier-4/5 atoms sit at
  `exp_t > 5000`, even a 0.5+ credit improvement contributes
  little to Sim. If we want the user-visible gain to drive
  calibration, we either need an NS amendment that promotes
  load-bearing kv items to lower tiers, or to track a
  reach-count-weighted metric alongside Sim. Defer until more
  re-export-heavy fixtures land or NS authoring runs are due.
  **Deferred follow-up — `#[path]` and inline-pub-mod children.**
  Resolver intentionally doesn't honor `#[path = "..."]` or descend
  into `pub mod foo { mod bar; }` for extern-child resolution.
  The Restricted-under-`src/` fallback is conservative-correct in
  these cases; lift if a fixture surfaces them.

### Divergence report — deferred refinements

Three rounds of reviewer feedback have landed; report is ship-ready as
the calibration artifact. Two items previously deferred:

- **Aggregate walker-waste rows by descriptor pattern.** DONE.
  `format_walker_waste_rollup` in `src/divergence.rs` now emits a
  rollup table above the per-batch waste table, grouping rows by
  descriptor pattern (positional suffixes — `:<line>` / ` section
  #<index>` — collapsed to `<n>`). Surfaces e.g. log's "14 `pub-item
  doc at src/lib.rs:<n>` rows totaling 3572 tokens" at row 1 of the
  rollup. Pattern matching is shape-anchored on known walker
  descriptor templates (not regex on arbitrary digits), so paths
  with embedded digits don't false-collapse; covered by unit tests
  in `divergence::tests`. Section also elides when no pattern groups
  ≥2 rows. Side-effect of the implementation: per-batch table now
  ties on relative descriptor (was: absolute), so a few sub-tied row
  pairs reordered in baselines — sort key now matches what the
  reader sees.
- **Marginal per-atom off-NS attribution.** DONE.
  `format_walker_waste` in `src/divergence.rs` now weights off-NS
  attribution by per-atom marginal token cost (`off_marginal /
  total_marginal`) rather than atom count. Per-atom marginals come
  from a new `RenderedTree::marginal_cost_per_atom` helper invoked
  once per scheduled batch against a parallel walker tree driven
  forward in schedule order — so refinement-over-ancestor lines pay
  the truncated delta and already-listed FS entries pay 0,
  matching what the scheduler actually paid. `off_tokens` is now
  the exact off-NS marginal sum (no `× cost` approximation), and
  `off_ratio = off_marginal / total_marginal`. The earlier draft
  used `GradedAtom.bytes` (credit-accounting field, wrong
  dimensions) and a fresh-cost denominator (skews on
  refinement batches); both flaws were caught in plan-review
  rounds 1 and 2. Render-side refactor: `marginal_cost` and
  `marginal_cost_per_atom` share a `visit_atom_costs` visitor so
  the cost formula has one source of truth and the sum-only
  scheduler hot path doesn't allocate a per-atom Vec. Sim scores
  unchanged across all 10 fixtures; walker-waste rows re-rank,
  most diffs <30 lines/baseline. Plan + 3 plan-review rounds in
  `ignore/plan-divergence-tooling-tightening.md`.

### Calibration target surfaced by the rollup

With the rollup live, the dominant waste pattern across fixtures is
clear: `pub-item doc at <lib.rs>:<n>` (log: 14 rows / 3572 tokens;
anyhow: 5 / 2725; mdbook: 2 / 600). README/docs sections are the
secondary cluster (cmdk: 5 / 2948; ky: 5 / 1007; mitt: 2 / 294).
**PubItem rebalance applied (catastrophic ↓ on PubItemDoc):** post-
change the log waste rollup is 13 rows / ~2700 tokens (down from 14 /
3572) — modest reduction, as expected from a calibration nudge that
preserves load-bearing PubItemDocs.

### Walker calibration — tier-3 falloff is the open lever

After the divergence rewrite + 4 clean NSs, the per-tier rollup shows a
consistent pattern: walker reaches tier 1 reliably (avg credit
0.70–0.99 across fixtures), tier 2 mostly (0.32–0.91), then drops
sharply at tier 3+ (typically 0.10–0.30). Concrete Sim scores at the
moment of writing:

- log:    Sim=0.391  reached 16/49,  Used 9956/10000
- anyhow: Sim=0.403  reached 12/40,  Used 9274/10000
- mdbook: Sim=0.305  reached 10/46,  Used 9899/10000
- otree:  Sim=0.351  reached 19/52,  Used 9982/10000

This is the next calibration target: walker's value-model weights
(`W_CATASTROPHIC`, `W_FOLLOW_UP`, `W_ZERO_CALL`, sublinearity exponent)
were last tuned against the alignment-reviewer regime, before the
deterministic divergence metric existed. Tuning under the new metric +
the four NSs is now feasible. See also "Stopping criterion / value
function" above for the per-signal items already noted.

### NS author repeatability — only one run per fixture

Original plan was two author runs per fixture and measuring divergence
between drafts (a cheap proxy for whether the prompt is producing
stable rankings vs. noisy ones). We only did one run per fixture this
session because the agent token cost was higher than expected. Worth
doing as a calibration sanity check before declaring the prompt frozen
— if two runs disagree wildly on tier boundaries, the prompt needs
tightening. Spawn `north-star-author` agents pointed at the same
fixture, write to `tests/north-stars/drafts/<fixture>__runN.toml`,
diff. Deferred until needed.

### Schema vocabulary — better doc comments, maybe stricter validation

Observed from the first NS-author runs under the revised prompt:

- **Truncation misuse** — the log NS used `Truncated { pattern = … }`
  on lines like `macro foo {` where the match covers all or most of
  the line. Truncation saves no tokens unless the tail of the line is
  the bulk of it. `Render::Truncated`'s doc in `src/content.rs` should
  make this explicit (the point of the pattern is to *drop* the rest
  of the line; if nothing meaningful trails the match, use `Full`).
  Optional stronger version: validator could reject patterns that
  match the whole line.
- **Multi-line Ellipsis spans** — DONE.
  `Violation::EllipsisMultiLine` in `src/ns_simulate.rs` now flags
  `Render::Ellipsis` spans where `start != end`. Quality-only
  (not render-blocking — the batch still simulates so successors
  don't see false-positive `PredecessorMissing`). The earlier
  premise here was wrong: a multi-line Ellipsis span doesn't drop
  lines silently; `explode_spans` expands every covered line and
  `format_line_row` emits one `…` per line. So a 5-line Ellipsis
  renders as five consecutive `…` markers — visually
  indistinguishable from one (Ellipsis lines emit no line number)
  but costing 5× the tokens. Either way it's an authoring slip,
  and the schema doc on `Render::Ellipsis` already calls it
  single-line-only — the validator now enforces that. Plan-review
  round 1 caught the stale premise.

Both are "prompt-adjacent" bugs — agents generally infer type
semantics from field docs, so tightening `content.rs` type docs is
probably 80% of the mitigation.

### North Star / reviewer alignment
- **Rank `PubItemNames` as a first-class NS batch** — the Rust walker
  emits a `pub struct X {\n…` location-hint batch per file
  (`src/walker/rust.rs` `PubItemNames`), but current north stars model
  only full-body batches. Reviewers therefore see the hint render and
  flag it as "batch partially included" against the NS expectation. The
  honest description is "two walker batches exist; the names-surface one
  fired, the body deferred". Batch with next NS-author update so the
  regeneration cost lands once.

### Process
- **More languages** — TypeScript and Python are the next likely targets
  after Rust + markdown.
- **Larger fixtures** — the v0.2 fixture set (log/anyhow/mdbook) is small
  by design. Add scale fixtures once the perf work is in.
- **Alignment-reviewer count is a noisy fitness metric** — the count of
  `- [category] [severity]` divergence bullets does not scale
  proportionally with snapshot-content deltas. Concrete observation
  from this session: a ~14-line content change in log_6000 (one new
  README section, one dropped `__private_api::log` body fragment)
  produced a +21 jump in divergence count (28 → 49). The reviewer's
  qualitative findings stay roughly consistent across runs (same
  batches flagged as partial, same ranking inversions), but the count
  inflates whenever new content introduces new partial-batch
  observations, even when the new content is *better aligned* with the
  North Star in aggregate. Treat count as directional only; always
  cross-check qualitative snapshot diffs before concluding a change is
  better or worse. We don't have clean evidence of run-to-run variance
  on *identical* snapshots — the "oscillation" I thought I was seeing
  was different snapshots from different experiments producing
  similar-but-not-identical counts.
- **Deterministic alignment metric** (promoted from speculative) — given
  the reviewer noise above and the quota cost of running 12 agents per
  calibration iteration (this session pushed the user's weekly Claude
  quota past its 5-hour window), a deterministic NS→snapshot line-range
  coverage metric is looking more attractive. Shape: for each NS batch,
  compute "what fraction of its declared line range appears in the
  snapshot" + a per-tier weighted sum. Gives a continuous, reproducible
  score that can drive calibration tuning without per-experiment agent
  cost. Risk: (1) places load-bearing weight on NS line-ranges being
  exactly right; (2) misses honesty concerns and ranking-order
  inversions that need semantic judgment; (3) treats "batch present
  but out of priority order" as a hit when NS actually ranks it low.
  Mitigation: use the deterministic score for calibration iteration;
  reserve agent reviews for milestone checkpoints (new fixture, new
  language, ontology change).
- **Alignment-reviewer agent sometimes doesn't Write** — observed once
  in this session: the agent returned the full report in its output
  summary but didn't call the Write tool, leaving the existing review
  stale on disk. Consider tightening `.claude/agents/alignment-reviewer.md`
  with a hard post-condition ("return only after Write has succeeded")
  or switching to a template where the Write call is the return path.
- **Alignment-reviewer short-circuits on matching hash** — when an
  existing review's `snapshot_hash` matches the current snapshot, the
  agent treats it as fresh and returns without re-scoring, even when
  the intent is to re-evaluate under changed interpretation (e.g., a
  reviewer-prompt update, or investigating review variance). Current
  workaround: delete the review file before re-running. Consider adding
  a "force" flag or routing re-score requests through a different entry
  point.
- **Codex CLI long-prompt hang** — `codex exec "<long prompt>"` with a
  multi-KB prompt as a positional argument appeared to hang
  indefinitely (no output, process stayed alive for >10 min). Piping
  the prompt via stdin (`cat prompt.md | codex exec`) works reliably.
  Worth documenting wherever we codify codex invocation patterns.
- **North Star regeneration is expensive** — three drafts (up to ~40
  min each) + a combiner pass. Batch NS updates across multiple
  planned changes (e.g., the "rank PubItemNames first-class" item and
  the future markdown-headings-only equivalent) to amortize the cost.

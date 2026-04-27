# precis v0.2 — design notes

> **⚠️ Agent-maintained.** This file is written and updated by Claude
> across many sessions; entries are notes-from-then, not edicts. They
> can be stale, partially right, or have been superseded by later
> decisions that didn't make it back here. **Verify anything
> load-bearing with the user before acting on it** — especially items
> that read as judgement calls, deferred TODOs, or recommendations.
> Concrete invariants and architecture should be checked against the
> code; this file is for things not visible there.
>
> **When a deferred item ships, delete the entry rather than marking
> it DONE.** The code is the source of truth; keeping completion
> writeups here defeats the doc's purpose and grows it indefinitely.
> Likewise, drop historical narrative (which calibrations were tried,
> sim deltas at the moment of writing, alignment-reviewer-era
> observations) — git log is the ledger.

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
Verify any change against `cargo t fixture_baselines` to make sure
source files don't lose the relative ranking against test/website/etc
dirs.

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
code. Discipline: **don't accidentally specialize cross-language code to a
single language**.

Current shape:

- The `Walker` trait emits [`Batch<K>`] units carrying `key`,
  `predecessor`, fully-built `BatchContent`, and a scalar `value: f64`.
  No `expand`/`materialize` split — walkers compute content at emit time
  (parse trees cached on `WalkCtx`).
- The `FsWalker` (`src/walker/mod.rs`) is the top-level `Walker` impl;
  it owns directory recursion and dispatches per-extension hooks
  (`rust::expand_in_dir`, `markdown::expand_in_dir`, …) when a
  directory listing is scheduled. Closed-set enum dispatch — adding a
  language touches `BatchKey`, the language module, and `FsWalker::expand`.
- Path-relative location priors live in shared helpers
  (`walker::path_depth_factor`, `walker::file_depth_factor`,
  `value::non_essential_factor`, `value::depth_factor`).
- Value composition uses the `value::mix_signals(cat, fu, ztu, depth)`
  helper. The triple is a useful *expressive* convention at the call
  site (per-batch `(catastrophic, follow-up, zero-tool-call)` tuning
  reads naturally) but the trait surface is just `value: f64`. Walkers
  are free to skip the helper and compute their value any other way.
- Per-walker run state (cross-file analyses worth memoizing) lives in
  named fields on `WalkCtx` — today only `rust_state: rust::RustState`
  (module visibility, workspace membership, exported macros). New
  languages add a typed field rather than smuggling state through a
  `TypeId` bag.

Tradeoff worth recording: collapsing `ValueSignals` (3-axis tuple +
shared mix) to `value: f64` lost a global tuning knob —
`W_CATASTROPHIC=1000 / W_FOLLOW_UP=400 / W_ZERO_CALL=300` used to be a
single edit-site. Per-batch tuning reads the same as before
(constants carried at the call site via `mix_signals`), but global
re-balancing now means editing every walker. The phantom-knob
calibration in git history was first-pass and never retuned, so the
loss is small in practice; if a future calibration push wants a
per-axis global knob back, the right shape is probably per-walker
mix functions rather than a re-introduced central type.

## Deferred (pick up in later sessions)

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
- **`Cost` newtype** — `Cost { tokens, bytes }` admits absolute nonsense
  (negative values, mismatched units). A private-field constructor would
  catch bad inputs at the boundary. Cheap; defer until something misuses
  it.

### Render
- **Filesystem-level override** — file-content batch superseding a folder
  listing entry, "N more files" placeholders, alternate non-tree renderings.
- **Outline-Section ancestor coupling** — `MarkdownKey::HeadingsOutline`
  is the predecessor of every `Section` in the file when emitted.
  Without that edge, both batches would render the same heading rows
  and the renderer's non-ancestor-overlap rule would fire. The trade
  is gating: if the outline is too big to fit near the budget tail,
  the prefix-monotone scheduler stops and every section in that file
  is blocked. Today the outline self-suppresses past 30 headings or
  1500 bytes of heading content (whichever first); the architectural
  follow-up is a rendering scheme that lets outline + sections schedule
  independently.

### Scheduler / walker
- **File-as-seed** — currently rejected with a clear error in `lib.rs`.
  Needs a small content-only walker path, probably driven by a real
  content walker rather than a generic "show full file" fallback.
- **Multi-path seed** — the CLI accepts `Vec<PathBuf>` but `render()`
  uses only the first path. Multi-root scheduling (one budget across
  roots) is deferred.
- **Eligible-id tracking in the scheduler hot path** — `best_exact`
  scans the full `entries` vector twice per scheduling iteration and
  filters out ineligible batches via `self.eligible(...)` HashMap
  lookups. Pre-collapse the candidate pool was separate from the
  ranked pool, so ineligible batches didn't participate in the rank
  scan. Concrete fix: maintain `eligible_ids: Vec<BatchId>` updated in
  `absorb` (push when no predecessor) and in `schedule` (after marking
  scheduled, push dependents whose predecessor just became scheduled
  — needs a `pred_key_to_dependents` reverse index). Then `best_exact`
  iterates only over eligible ids. Benign on current fixture sizes
  (~140 batches) but real with deep predecessor chains. Defer until a
  profile or a wide-frontier fixture motivates it.
- **Cheap min-tokens lower bound for early pruning** — every emitted
  batch carries fully-built content, and `RenderedTree::marginal_cost`
  is the only path to a real cost number. A tight lower-bound estimator
  on `BatchContent` (e.g., line count × min-row-overhead, possibly
  taking already-rendered overlap into account) would let the scheduler
  prune obviously-too-big batches without touching the render tree.
  Goal-only — the right interface is open (line count alone misses
  predecessor-overlap savings; full marginal cost is too expensive).
  Defer until a profile or a wide-frontier fixture motivates it.
- **Rust mod-visibility resolver — `#[path]` and inline-pub-mod
  children.** The resolver intentionally doesn't honor
  `#[path = "..."]` attributes or descend into `pub mod foo { mod
  bar; }` for extern-child resolution. The Restricted-under-`src/`
  fallback is conservative-correct in these cases; lift if a fixture
  surfaces them.

### Value/cost ranking — high priority, experimentation territory

The single biggest open lever on NS divergence. Per-tier rollup across
fixtures shows a consistent pattern: walker reaches tier 1 reliably
(avg credit ~0.70–0.99), tier 2 mostly (~0.32–0.91), then drops sharply
at tier 3+ (~0.10–0.30). Two sub-symptoms that *seem* distinct but are
plausibly the same problem and worth investigating together:

- **Cost-side concavity**: `value::ratio` is `value / cost^0.35` for
  every batch (gentler than `sqrt`; see the commit that moved off
  `sqrt`). May want per-category shapes (hard cap on `CrateDocLede`
  size, gentler concavity on test-as-spec batches), or a different
  functional form entirely.
- **Sibling-count devaluation**: when a file emits many per-item
  batches (a config module with 20 `pub struct` children), each one's
  individual value/cost ratio beats the value/cost of a single
  important body elsewhere (`CommandArgs` in `src/cmd.rs`), producing
  a "wide-but-shallow signature sweep" across deep files at the
  expense of root-level anchors. A first attempt folded a
  `sibling_factor(n_siblings)` into `PubItem`'s depth factor —
  regressed more than it improved. Dense core files (anyhow's
  `src/lib.rs` with ~25 pub items) are *legitimately* dense;
  decoration-heavy dense files (otree's `src/config/colors.rs` with 7
  color sub-structs) look structurally identical but have very
  different intrinsic value. `is_entrypoint_file` doesn't reliably
  distinguish them.

**This area is high-priority and explicitly experimentation territory.**
Multiple creative approaches are likely needed — different formulas,
per-category shapes, richer sibling/density signals, NS-author updates
that rank `PubItemNames`-style location hints as first-class.
Calibration drives divergence; expect to iterate against the metric
across the fixture set rather than expect the first try to land.

Tunables: per-batch values (set in each walker module's `*_value`
fns); `mix_signals` weights in `src/value.rs`; cost concavity exponent
in `value::ratio`; tier weights in `src/divergence.rs` (`TAU`,
`REACH_THRESHOLD`, `MISSING_FLOOR`, `EARLY_FACTOR`, `LATE_FACTOR`,
`UNMAPPED_COST_THRESHOLD`).

### Stopping criterion / value function
- **Stopping criterion beyond "no batch fits"** — dynamic floor or
  value/cost threshold so we stop earlier when remaining batches are weak.

### NS author repeatability — only one run per fixture

Worth doing two author runs per fixture and measuring divergence
between drafts as a sanity check that the prompt produces stable
rankings vs. noisy ones. Spawn `north-star-author` agents pointed at
the same fixture, write to `tests/north-stars/drafts/<fixture>__runN.toml`,
diff. Deferred — agent token cost is high; do this when calibration
stability becomes a question.

### Schema vocabulary — better doc comments, maybe stricter validation

- **Truncation misuse** — the log NS used `Truncated { pattern = … }`
  on lines like `macro foo {` where the match covers all or most of
  the line. Truncation saves no tokens unless the tail of the line is
  the bulk of it. `Render::Truncated`'s doc in `src/content.rs` should
  make this explicit (the point of the pattern is to *drop* the rest
  of the line; if nothing meaningful trails the match, use `Full`).
  Optional stronger version: validator could reject patterns that
  match the whole line.

### Process
- **More languages** — Python is the next likely target after Rust /
  markdown / TypeScript / JSON / TOML / plaintext.
- **Larger fixtures** — the current fixture set is small by design. Add
  scale fixtures once the perf work is in.
- **North Star regeneration is expensive** — three drafts (up to ~40
  min each) + a combiner pass. Batch NS updates across multiple
  planned changes to amortize the cost.

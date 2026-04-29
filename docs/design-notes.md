# precis v0.2 — design notes

> **⚠️ Agent-maintained.** This file is written and updated by Claude
> across many sessions; entries are notes-from-then, not edicts. They
> can be stale, partially right, or have been superseded by later
> decisions that didn't make it back here. **Verify anything
> load-bearing with the user before acting on it** — especially items
> that read as judgement calls, open questions, or recommendations.
> Concrete invariants and architecture should be checked against the
> code; this file is for things not visible there.
>
> **When work captured here ships, delete the entry rather than
> marking it DONE.** The code is the source of truth; keeping
> completion writeups here defeats the doc's purpose and grows it
> indefinitely. Likewise, drop historical narrative (which
> calibrations were tried, sim deltas at the moment of writing,
> alignment-reviewer-era observations) — git log is the ledger.
>
> **Contracts on a specific interface belong as doc comments on that
> interface, not here.** This file is for cross-cutting design choices,
> meta-process, and open questions that don't sit on any single fn or
> type.

A living doc for cross-session design constraints, decisions, and
open questions that aren't visible from reading `src/`. The codebase
itself is the source of truth for architecture and invariants.

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
- **Output is a verbatim subset of the source.** No paraphrasing,
  summarization, or invented content under any circumstances. Allowed
  transforms (full lines, prefix+ellipsis truncation, bare-ellipsis
  markers) are documented on `Render` in `src/content.rs`.

## North Star process

- North Star documents (`tests/north-stars/<fixture>.toml`) are agent-drafted,
  human-reviewed, and **frozen** before implementation iterates against them.
  Once frozen they are the divergence test's ground truth; implementation
  changes do not edit them.
- `revision_pin` is the only thing binding an NS to its fixture revision.
  `load_ns_checked` enforces it; the `ns_pins_match_fixture_pins` test
  enforces it under `cargo t`.
- **Growth envelope**: each batch's marginal cost must satisfy
  `cost_i ≤ 100 + 0.3 · cumulative_before`. Per-batch is too local;
  cumulative matches the author's intuition ("doubling aggregate on
  batch 2 is fine, doubling on batch 10 is bad") and doesn't force
  authors to inflate a small preceding batch to clear the path for a
  legitimately larger one later. Constants tunable via `ENV_BASE` and
  `envelope_max` in `src/ns_simulate.rs`.

## Scheduler prefix-monotonicity (consequence for divergence)

The scheduler stops on first ill-fit (`src/scheduler.rs` module doc has
the algorithm). The consequence worth recording here, because it
shapes the divergence metric and NS-authoring constraints:

**Every decision taken at budget `T_small` up to its stopping point is
also taken at `T_large`.** The only budget-sensitive check is the
final "does it fit" step; up to `T_small`'s stopping point, every
scheduled cost also fits at `T_large`. So `T_small`'s schedule is a
true prefix of `T_large`'s schedule, sub-budget snapshots can be
obtained by slicing a single `T_max` run, and the divergence metric
runs the walker once per fixture rather than per-budget.

**Tradeoff**: when the top-ranked exact is too big, budget
under-utilization can be as much as one batch's cost. This is
deliberate pressure on walker calibration (if a top batch consistently
blocks small budgets, split it or lower its rank) and on NS authoring
(the growth envelope keeps NS prefixes coherent at small budgets).

## Cross-language vs language-specific concerns

Many concerns precis cares about are cross-language (value heuristics,
ranking signals, render conventions, structural priorities); only the parts
that genuinely depend on a language's grammar belong in language-specific
code. Discipline: **don't accidentally specialize cross-language code to a
single language**.

Two related conventions worth resisting drift on:

- Walker dispatch is a closed-set enum. Resist adding extension points
  unless multiple languages actually want them.
- Per-walker run state goes in named fields on `WalkCtx` (today only
  `rust_state`), not a `TypeId` bag or thread-local.

## Value/cost ranking — open lever on NS divergence

One of the open levers on NS divergence; current per-fixture priority
should be read from `tests/divergence/OVERVIEW.md` and the per-fixture
verdict blocks rather than from this section. Historical context: per-
tier rollups across fixtures showed a consistent shape — walker reaches
tier 1 reliably, tier 2 mostly, drops sharply at tier 3+. The cost
concavity is now per-key via
`WalkerKey::concavity_exponent` (default `0.35`) — the hook is in place
for further calibration. Open sub-symptoms:

- **Sibling-count devaluation**: when a file emits many per-item
  batches (a config module with 20 `pub struct` children), each one's
  individual value/cost ratio beats the value/cost of a single
  important body elsewhere (`CommandArgs` in `src/cmd.rs`), producing
  a "wide-but-shallow signature sweep" across deep files at the
  expense of root-level anchors. The naive fix — folding a uniform
  `sibling_factor(n_siblings)` into `PubItem`'s depth factor —
  regresses, because dense core files (anyhow's `src/lib.rs` with
  ~25 pub items) are *legitimately* dense and decoration-heavy dense
  files (otree's `src/config/colors.rs` with 7 color sub-structs)
  look structurally identical. `is_entrypoint_file` doesn't reliably
  distinguish them; a working version needs a signal that does.
- **Prefix-stop tail effects on calibration tweaks**: any change that
  shifts a big batch's rank can leave it stuck near the budget tail
  where it no longer fits. The scheduler's prefix-monotone stop then
  truncates the schedule, dropping the trailing `walker_used`. Saw
  this on the per-key concavity bump: cmdk dropped from
  `walker_used`=9484 to 7302 at B=10K. Score(3000) is invisible to
  this (the prefix is identical at small budgets), but Score(9000)
  and the `walker_used` column at high B get thinner. Mitigation
  lever exists if needed — walker-side filter on absolute-cost — but
  it's a separate change.

Explicit experimentation territory — different exponents per key,
richer sibling/density signals, NS-author updates that rank
`PubItemNames`-style location hints as first-class. Calibration drives
divergence; expect to iterate against the metric across the fixture
set rather than land it on the first try.

## Un-shipping candidates from Sim-era iterations

Walker / value commits up through `38f63e3` were calibrated against
the old `Sim` metric, and may not be paying their own complexity
cost under `Score(3000)`. The retrospective in
`ignore/retro/FINDINGS.md` has the per-fixture deltas. Candidates:

- Commits whose targeted fixtures landed flat or negative on
  `Score(3000)` over the full sequence: thiserror, tomli,
  microbootstrap, bareiron, typeguard, anyhow, log, vaul, tock,
  go-multierror.
- `#20 d4d479b` pyproject `[project]` Identity (single-fixture
  target).
- Long-tail-targeted parts of `#1 302b0b5` and `#6 029f13e`.

Drop entries as candidates resolve; the doc shouldn't accumulate
post-hoc verdicts.

## Divergence diagnostic — deferred architectural items

Two known approximations in the divergence report's diagnostic layer
(`src/divergence/{diagnosis,synthesis}.rs`). Worth fixing before serious
calibration that depends on distinguishing them; not blocking for the first
walker/value pass.

- **Loss reasons use post-hoc budget state.** `candidate_loss` checks
  `row.final_cost.tokens > remaining_tokens` against the *final* remaining
  budget, not the budget at first eligibility. Marginal cost itself is
  pinned by walker invariants (non-ancestor line overlap is rejected at
  apply, FS atom overlap is rejected at absorb in debug builds, ancestors
  are scheduled before a dependent first becomes eligible — see
  `src/scheduler.rs`), so cost-side drift isn't a concern. But budget is
  consumed by later wins, so a candidate that fit when first eligible —
  and lost the `value/cost^k` ratio race to competing batches — gets
  labeled `too expensive at final margin` once those later wins consumed
  the headroom. That conflates a true budget-pressure case (candidate
  never fit) with a ranking-race case (candidate fit when eligible, lost
  the rank fight, then ran out of room). The two want different
  interventions: demote low-value spend vs. tune the value/cost ratio so
  the candidate wins earlier. Right fix is scheduler-side instrumentation:
  record eligibility, marginal cost, fit status, and rank at decision
  time, and attribute losses against that. Lower urgency if next work is
  walker granularity (which the current corpus mostly says is the lever).

- **`Schedule.candidates` is `#[serde(skip)]`.** The candidate pool
  needed by unscheduled-bbox / predecessor-gating / coverage-gap
  diagnostics doesn't survive TOML serialization. Scheduled-bbox and
  scheduled-same-file hints still work from `schedule.batches` alone, so
  basic ledger rows aren't affected — but anyone calling
  `generate_divergence_report` from a deserialized schedule silently loses
  the unscheduled-candidate signal entirely (rows that would have shown
  `[unscheduled bbox exact=...]` or `predecessor not scheduled` collapse
  to `no discovered candidate`). In-process baseline tests pass the
  in-memory `Schedule` so they're unaffected. Either persist enough
  candidate metadata, or split the API/type so reports require a
  candidate-bearing schedule and fail loudly when the pool is empty.

## Min-tokens lower bound

`RenderedTree::marginal_cost` is the only path to a real per-batch cost
number, and every emitted batch carries fully-built content. A tight
lower-bound estimator on `BatchContent` (e.g., line count ×
min-row-overhead, possibly accounting for already-rendered overlap)
would let the scheduler prune obviously-too-big batches without
touching the render tree. The right interface is open: line count
alone misses predecessor-overlap savings; full marginal cost is too
expensive. Benign on current fixture sizes (~140 batches), real with
deep predecessor chains or wide frontiers.

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
- **Training vs validation tier.** Fixtures are split into a training set
  (full divergence report at `tests/divergence/<name>.md`, drives
  calibration) and a held-out validation set (one-line score at
  `tests/validation/<name>.md`, no per-row diff). The validation set is
  sampled to match the GitHub language distribution within supported
  languages; the calibration loop is forbidden from opening it. The
  point is overfit detection — if a value/walker change wins on the
  training set but tanks on validation, the rule isn't general.
  Enforcement is by process (the `iterate-divergence` skill), not by
  the type system; treat the holdout as load-bearing convention. The
  holdout covers more than `tests/validation/`: it also covers the
  validation fixtures' NS files (which share `tests/north-stars/`
  with training NSes) and their source under `tests/fixtures/`.
  Storing validation NSes alongside training NSes is a deliberate
  process-bloat tradeoff; the alternative — a parallel directory
  tree — would not strengthen the convention, since both surfaces
  are equally readable to anyone disregarding the skill.
- **Validation debugging surface is intentionally thin.** A
  regression on a validation fixture reports only a headline
  delta — there is no schedule TOML, rendered snapshot, or per-row
  diff for held-out fixtures. The acceptable responses to a
  validation move are: improve the walker generally against the
  *training* reports and re-run, or accept the move as a real
  generalization signal. Adding diagnostic artifacts (a validation
  schedule TOML, etc.) would re-expose the surface the holdout
  exists to hide.
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

## Auto-injected docs don't belong in precis output

Files the host harness already loads into the model's context —
top-level `AGENTS.md`, `CLAUDE.md`, and text files under
`.claude/skills/`, `.agent/skills/`, `.cursor/rules/` — should not
have their bodies scheduled by precis. The file paths *should*
remain discoverable via fs listings (so the agent knows the file
exists and can read it if not auto-injected), but the prose-body
batches (`MarkdownKey::Section`, `MarkdownKey::SummaryWhole`) are
walker-side suppressed. Residual structural batches
(`HeadingsOutline`, `ReadmeHeadline`) carry a 0.1× value discount.

User framing (verbatim): "precis isn't meant to guarantee that all
content is reachable, it's meant to provide a value-per-token
summary that lets follow up tool calls do the rest."

If a future NS fixture surfaces these files' content as primary
atoms (pluggy's `AGENTS.md`, toasty's `CLAUDE.md` historically),
that's an NS-author error to flag — don't move the goalpost by
un-suppressing the walker.

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
should be read from the per-fixture reports
(`tests/divergence/<fixture>.md`) rather than from this section. Cross-
fixture survey is via shell — see the survey commands in the
`iterate-divergence` skill. Open sub-symptoms:

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
  A second variant — devaluing the per-file *names surface* (rather
  than per-decl) by `sqrt(K / n_siblings_in_dir)` across C / Python /
  Go / Lua / Rust / TS walkers — was tested at K=5, K=10, K=20 with
  matching floors. All three regressed the corpus average: K=5
  delivered htop +nothing / cobra -0.118 / bubbletea -0.093 / vaul
  -0.231; K=10 still hit bubbletea -0.100; K=20 was flat with no
  meaningful wins. The "uniform demotion across all surfaces in a
  dir" doesn't change the *relative* order among those surfaces, but
  it lets non-surface batches (`package.json`, `tsconfig.json`,
  README sections) jump ahead of the dir's load-bearing primary
  (vaul's `src/index.tsx`, bubbletea's `tea.go`). The same dir-shape
  hosts both "wide-but-shallow sweep" patterns (htop's `darwin/`)
  and "one primary surrounded by helpers" patterns (vaul's `src/`);
  sibling count alone can't distinguish them.
- **The early-budget ratio wall (why Score(3000) expansion stalls).**
  Two independent scheduler probes (2026-05) measured it: in the first
  ~3000 tokens the winners are cheap `Fs` dir-listings / `Json` /
  package-identity orientation batches with `value/cost^0.35` ratios of
  ~120–291, while a deep source file's names-surface sits at ratio ~36 —
  a 3–8× gap. Deep source (a require-hub class, a `src/` table) often
  doesn't schedule within 10000 tokens, not just 3000. Closing the gap
  by value tuning needs a 6–7× boost, which fires on *every* sibling at
  that tier and reorders destructively (e.g. boosting entrypoint-required
  classes regressed commander −0.138 / dockly −0.092 while the target
  didn't move). The orientation batches it would displace are themselves
  NS-wanted (NS authors front-load the file tree), so this isn't noise to
  cut — it's a genuine local optimum *for re-ranking already-emitted
  content*. The wall does NOT cap new-content recall: levers that surface
  content the walker emitted nowhere keep paying, and the highest-yield
  ones target **early / high-importance-weight (rank-1/2) atoms** —
  Importance is `Σ damped/rank`, so a rank-1 atom is worth ~6× a rank-6
  one. Cleared it (2026-05): RST sections, prisma bodies, small-header
  bodies, C platform-port demotion, man-page NAME/DESC extraction
  (`plaintext.rs`), decorative-H1 lede descent (`markdown.rs`) — the last
  three took htop 0.182 → 0.263 and the corpus 0.5876 → 0.5890. So the
  walker-side ceiling is NOT fixed; only *re-ranking* is at a local
  optimum. Remaining headroom is more early-atom recall (plus a
  core-header in-degree boost for OOP-spine ratio-wall fixtures like
  htop, whose `Object/Row/Process/Meter/Panel` headers still lose the
  ratio race). A lift via *re-ranking* would need a structural change
  (FS-descent-order signal, per-tier rebalance), not per-key nudges.
  Measured
  (2026-05): the global `DEFAULT_CONCAVITY_EXPONENT` is already at its
  peak — 0.35 → corpus avg 0.5876; 0.30 → 0.5739; 0.40 → 0.5551 — and the
  win/loss split is by fixture *structure* (deep-method-heavy like
  nano-vllm +0.100 vs orientation-heavy like sqlite-vec −0.120),
  crossing languages, so neither a single exponent nor a per-language
  override captures it. A real rebalance needs a per-batch-shape (not
  per-key) cost model or a budget-tier-aware scheduler. The per-batch-
  shape model was also tested and is walled: an additive ranking-cost
  floor `value/(cost+C0)^k` (models fixed per-batch framing overhead,
  demotes the ~5-token orientation flood) regressed at C0=20 → 0.5838
  (otree +0.078 / toasty +0.049 vs tock −0.130 / mcphost −0.058); C0=0
  is optimal. So all three accessible ranking knobs — multiplicative
  exponent, additive cost floor, FS source-dir value — sit at their
  optimum. A lift past 0.5876 needs a genuinely different scheduling
  *algorithm* (explicit orientation-vs-source budget tiers), not the
  value/cost greedy.
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
- **Uncalibrated v0.2 JavaScript class-member levers**: the first JS
  class-member split pass introduced seed values that still need a
  calibration sweep: `JS_CLASS_MEMBER_SPLIT_MIN = 12`,
  `ExportMember` concavity `0.45`, split names factor `1.12`, and
  `export_member_value` weights `0.62 / 0.95 / 0.55`.

Explicit experimentation territory — different exponents per key,
richer sibling/density signals, NS-author updates that rank
`PubItemNames`-style location hints as first-class. Calibration drives
divergence; expect to iterate against the metric across the fixture
set rather than land it on the first try.

## Divergence open items

- **`Schedule.candidates` is `#[serde(skip)]`.** The candidate pool
  doesn't survive TOML serialization. The current schedule-centric
  divergence report doesn't read `candidates` at all (NS predecessor
  comes from the NS file, not the candidate pool), so the regen path
  is unaffected. Worth noting for any future code that loads a schedule
  from TOML and wants candidate-derived signals — it would silently
  see an empty pool. Either persist enough candidate metadata, or
  fail loudly at the call site.

- **Per-row Score column noise.** The schedule-centric report shows
  `Score(B=cum)` per row at three decimals. Small walker tweaks
  cascade through every later row's Score. Watch corpus diff noise
  in practice; if trailing-decimal flicker dominates, revisit (two
  decimals, or a delta-from-prev-row column).

- **Single Score column can't decompose I × C.** Headline includes
  `I=` and `C=` at B=3000, but the per-row Score is single-valued.
  Loses the rank-order-miss vs partial-delivery distinction. Revisit
  if iteration shows the single column loses signal.

## Walker / value open items

- **Names-surface chunking can break unified-batch expectations.** Names
  surfaces are sometimes split into `... #1 in <path>`, `#2`, etc. NS
  authors typically expect the names surface as a single unit; the
  walker's chunked version creates a catastrophic-omission failure mode
  where partial delivery scores poorly. Investigate when chunking fires
  and whether the granularity is worth the cost. Walker / value tuning
  question, not a divergence-report one.

- **Schedule TOML `key` / `parent` / `path` fields still embed
  absolute paths.** The walker descriptor fix (2026-05) made the
  `descriptor` field root-relative, but `ScheduledBatch.key` is the
  debug-formatted `BatchKey` enum which holds canonical `PathBuf`s,
  `FsGroup.parent` is the raw absolute path, and `Span.path` inside
  `BatchContent::Lines` likewise. These persist in
  `tests/snapshots/schedule/*.toml` but aren't user-facing through the
  new divergence reports. Fix would require either custom
  `Display`/`Serialize` impls on each `BatchKey` variant, or holding
  fixture-root-relative paths in the walker state. Not blocking; the
  user-visible artifact (divergence reports) is clean.

## NS-rank vs walker-rank: the 3K headline measures what the NS ranks, not what the walker delivers

`Score(3000)` evaluates only NS rows whose **NS** cumulative-tokens fall
within 3000 (`A_3K`). Pushing content earlier in the walker's schedule
does not lift the score if the matching NS row is ranked past 3K.

**NS placement decisions are sticky.** NSes are frozen; the walker
iterates against them. When the metric is misaligned with what
walker work is feasible at a given budget, the answer is either (a)
pick walker work that matches the metric's view of the budget, or
(b) accept the score gap and surface the new content at larger
budgets. Don't bump `value` to force a batch into a budget tier
where it doesn't earn `A_B` credit — it just displaces walker
batches that do.

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

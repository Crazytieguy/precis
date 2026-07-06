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
`AGENTS.md` / `CLAUDE.md` at any depth (Claude Code's CLAUDE.md
hierarchy is recursive), and text files under `.claude/skills/`,
`.agent/skills/`, `.cursor/rules/` — should not
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
- Per-walker run state goes in named fields on `WalkCtx` (`rust_state`,
  `typescript_state`, …), not a `TypeId` bag or thread-local.

## ⚠️ Score-history break at the 2026-07-05 NS re-freeze (22d2f7b3)

All 93 North Stars were re-authored (Sonnet, audited repair pipeline)
and re-frozen. Every Score/sweep/failed-lever magnitude recorded below
predates the new answer key unless said otherwise: **directions are
plausible hypotheses, magnitudes are void.** The settled-knob sweeps
(concavity, C0, mix_signals, …) are UNLOCKED — old optima no longer
bind. New zero point: training mean 0.5250 (old key: 0.6285). Start
any new calibration from the post-refreeze decomposition rather than
the entries below.

## Value/cost ranking — settled vs open

Per-fixture priority lives in the per-fixture reports
(`tests/divergence/<fixture>.md`); cross-fixture survey via
`head -1 tests/divergence/*.md`.

### Settled — re-swept on the post-refreeze keys (2026-07-05); don't re-sweep

All global ranking knobs re-swept against the new answer keys after the
wave-3 recall levers landed:

- **Concavity exponent 0.35** (`DEFAULT_CONCAVITY_EXPONENT`): confirmed
  optimal on new keys (0.30 → 0.52248, 0.33 → 0.52375, 0.35 → 0.52503,
  0.38 → 0.51913, 0.42 → 0.50501; pre-recall baseline).
- **Additive ranking-cost floor** `value/(cost+C0)^k`: C0=0 still
  optimal — C0=5 won in isolation (+0.0011) but lost to no-floor in the
  combined post-knob state. Interaction-check before believing any
  isolated C0 win.
- **`mix_signals` weights** now `1000·cat + 280·fu + 300·ztu`: cat and
  ztu regress in both directions; fu is **non-monotone** (280 → 0.53654,
  400 → 0.53569, 340 → 0.53480 on the post-fix state) — a midpoint
  re-sweep will mislead.
- **`ORIENTATION_TIER_WINDOW` 300 / boost 1.4**: 300 edges 500, beats
  0 and 900; boosts ≥1.7 regress.
- **`ROSTER_MASS_FACTOR_CAP` 2.2**: flat across 1.8/2.2/2.8.

Common overfit signature: shifting any of these toward the validation
optimum raises validation but lowers training — training is the
objective; don't chase it (see
`feedback_dont_decide_on_validation_holdout`).

~~Consequence: 0.65 is unreachable below the NS answer key / metric by
re-ranking.~~ **Superseded 2026-06-12.** The loss-decomposition
diagnostic (`cargo run --release --bin diagnose_loss`, see its module
doc) showed 0.37 of A_3K mass was emitted-but-unbought and the NS-aware
oracle ceiling over the existing pool was 0.8046 — the "~0.59 ceiling"
was an artifact of *knob* sweeps, not of re-ranking as such. The binding
bias was **roster-mass blindness**: roster batches (names surfaces,
listings, member catalogs) had size-invariant value but linear cost, so
`value/cost^k` always bought tiny rosters over the complete catalogs NS
authors anchor on. `value::roster_mass_factor` (= `(n/12)^k` clamped
`[1, 2.2]`, boost-only) is the fix class; wave-1 adoption across
fs/python/C/markdown/rust/go/ts walkers plus spine-centrality signals
(C include-graph hubs, python re-export pins, package-main visibility)
moved training 0.5921 → 0.6105 in one day. Recall levers still pay,
best on early / rank-1/2 atoms (Importance is `Σ damped/rank`, so a
rank-1 atom ≈ 6× a rank-6 one).

Adoption discipline learned the hard way: roster/centrality boosts must
be gated to a *structurally selective* tier. Class-wide gates measured
large single-fixture collapses before tightening (listings: plain
`src/`-named or Go-subpackage gates → soluna −0.210 / beszel −0.073;
inventory-only gate kept monaco +0.213 with one −0.006).

### Tested-and-failed lever shapes (specifics block re-tries)

- **Entrypoint-named `.d.ts` promotion (2026-06-12)**, two variants:
  (a) global `.d`-stem strip in `is_entrypoint_file` — chalk −0.249 /
  commander −0.165 (nested `source/index.d.ts` twins are type plumbing);
  (b) root-scoped only (surface-seed + machinery-exempt the package
  root's entrypoint-named `.d.ts`) — the target fixture itself regressed
  (axios −0.086): whole-file promotion floods the early budget with the
  wrong exports and displaces NS-first orientation. The axios
  `index.d.ts` mass (0.506 of its A_3K) needs per-export / chunk-level
  granularity driven by its divergence report, not a file-level flip.
- **Sibling-count devaluation**, two variants: (a) uniform
  `sibling_factor(n_siblings)` folded into `PubItem` depth factor —
  regresses; anyhow's dense `src/lib.rs` (~25 pub items) is
  *legitimately* dense while otree's `src/config/colors.rs` isn't, and
  they look structurally identical. (b) per-file names-surface
  `sqrt(K / n_siblings_in_dir)` across C/Python/Go/Lua/Rust/TS at
  K=5/10/20 — all regress (K=5: cobra −0.118 / bubbletea −0.093 / vaul
  −0.231; K=10: bubbletea −0.100; K=20 flat, no wins). Uniform demotion
  preserves relative order *within* the dir but lets `package.json` /
  README sections jump the dir's load-bearing primary (vaul's
  `src/index.tsx`, bubbletea's `tea.go`). A working version needs a
  signal that distinguishes "wide-but-shallow sweep" (htop's `darwin/`)
  from "one primary + helpers" (vaul's `src/`); sibling count alone
  can't.
- **Empty-vs-elided dir marker — measured and reverted (2026-07-04).**
  Rendering a synthesized `…` child under every childless-but-non-empty
  listed directory (so it can't be misread as empty) costs −0.0031
  corpus with severe flat-tree hits (tinyusb −0.140, migrate −0.086,
  mdbook −0.054, chalk −0.045) against scattered wins (semver +0.057,
  vite +0.028): the marker taxes every shallow listing row in wide
  trees where no misleading contrast exists. The correct gate — marker
  only when a *sibling* dir in the same parent is expanded (the actual
  confusion case: monaco `ini/` childless between expanded `html/` and
  `java/`) — makes cost accounting non-local: expanding one dir flips
  sibling markers on, so every FsGroup application would need to
  re-price sibling rows. If retried, prefer a walker-side shape (e.g.
  force one child entry into the parent's listing) over a renderer
  invariant. Implementation note for the retry: the WIP's cost path
  probed children by bare span name — `list_dir(p)` resolved against
  the process CWD; probe `parent.join(name)` as `apply_fs_group` does.
- **Go spine centrality — four proxies measured and failed (2026-07-04)**,
  targeting the mcphost/bubbletea/gin unscheduled cluster: (a)
  `roster_mass_factor` on `GoKey::DeclNames` — grouped const/var specs
  inflate name counts, trinket rosters outbid the spine (cobra −0.102 /
  gin −0.022); (b) `gated_descendant_value_weight` on DeclNames with
  decl-count + exported-share gates — big exported-but-NS-peripheral
  files saturate the bonus (mcphost −0.18 / bubbletea −0.14 / cobra
  −0.16); (c) same-package cross-file type-reference counting — fails
  on 3 of 5 NS spine files (gin routergroup.go = 1 ref, mcphost
  config.go = 1, migrate.go = 0); (d) struct-field-group ordinal decay
  — the demote-siblings failure again (cobra −0.111); its
  promotion-shaped variant is a zero-sum in-budget swap (cobra gain =
  bubbletea loss). No walk-time signal measured so far separates spine
  from trinket in flat Go packages; the cluster needs a different
  mechanism class (or NS-side reality check at the next re-freeze).
- **Early-budget ratio wall**: in the first ~3K tokens, cheap
  orientation batches win the `value/cost^0.35` race 3–8× over deep
  names surfaces (~120–291 vs ~36). Boosting deep source to compete
  fires on every sibling at that tier and reorders destructively
  (entrypoint-required-class boost: commander −0.138 / dockly −0.092,
  target unmoved). The displaced orientation is NS-wanted, so this is a
  genuine local optimum for re-ranking *already-emitted* content — only
  new-content recall moves it.

### Shipped: budget-tier scheduler (2026-05-31)

`WalkerKey::is_orientation()` tags orientation-class batches
(authoritative set = the `is_orientation()` impls in `src/batch.rs`:
Markdown, man-page ledes, Toml, non-`Whole` Json, Lua module identity);
the scheduler multiplies their ratio by `ORIENTATION_TIER_BOOST` while
`consumed.tokens < ORIENTATION_TIER_WINDOW` (500/1.4), at both the
approx contender pass and the exact pass. **`FsKey` is deliberately
excluded** — boosting the cheap dir-listing flood is the wrong
direction (incl-FsKey window=1000/boost=2.0 measured training
−0.0088). Calibrated to hold training Score(3000) flat (0.5908); the
sub-primary budgets take a tiny training nick that buys a held-out
gain at every budget (train Δ −0.0019 at 1000 → 0.0000 at 3000+;
valid Δ +0.0295 at 1000 → +0.0094 at 3000). The trade is inherent
(front-loaded orientation displaces some training fixtures' rank-1
code atoms under 1K) and was **shipped on the user's explicit call**
after surfacing it. Headroom: a per-fixture-structure-aware tier —
deep-method-heavy fixtures (nano-vllm) want source early,
orientation-heavy ones (sqlite-vec) don't, and the split crosses
languages, so neither a global exponent nor a per-language override
captures it.

### Open

- **Prefix-stop tail effects**: a rank shift can strand a big batch at
  the budget tail where it no longer fits (per-key concavity bump:
  cmdk `walker_used` 9484 → 7302 at B=10K). Score(3000) is blind to
  this; check Score(9000) and high-B `walker_used`. Mitigation lever
  if needed: walker-side filter on absolute cost.
- **JS class-member seeds swept (2026-06-10): inert at the primary
  budget; keep the first-pass values.** Every direction tested leaves
  training avg at 0.5921 with flat per-fixture headlines (commander /
  dockly reorder sub-3K rows only): `ExportMember` concavity 0.35 /
  0.40 / 0.55, `JS_CLASS_MEMBER_SPLIT_MIN` 8 / 16 (16 → dockly +0.001,
  noise), split names factor 1.0 / 1.3, `export_member_value` weights
  +cat 0.80 and all-down 0.45/0.70/0.40. The knobs sit in a flat
  region of the training objective — don't re-sweep; a lift on the
  JS-class fixtures needs new recall, not these values.
- **htop-class OOP-spine recall**: a core-header in-degree boost
  (htop's `Object/Row/Process/Meter/Panel` headers still lose the
  ratio race) is the one identified lever class still viable —
  structural pattern recognition, not value/ordering tuning.

## Wave-3 residue (2026-07-05): ops/config recall landed; what's still open

The post-refreeze absent bucket was dominated by ops/config files the
walkers deliberately excluded; wave 3 added CI/tooling YAML, Makefile /
Dockerfile / build scripts, legacy Python packaging, go.mod indirect
deps, manifest TOML config sections, and dev-doc markdown promotion
(training 0.5250 → 0.5365). Known remaining gaps, all needing
**structural extraction rather than whole-file/line-head emission**
(blunt variants measured negative and were reverted):

- `.env.sample` / `*.example` config templates (linkwarden 0.370,
  sqlite-vec `reference.yaml` 0.285 absent-loss) — want env-var-name /
  config-key rosters, not raw bodies.
- Large CI workflows — want trigger/job/run-command summaries; capped
  line-heads only cover compact workflows.
- `.pre-commit-config.yaml` / `.golangci.yaml` enter the pool but rarely
  get bought — likely need hook/linter rosters.
- package.json ranking: broad root-manifest boosts measured express
  +0.099 / monaco +0.058 **against** vaul −0.151 / linkwarden −0.098;
  needs an app/library role discriminator before retry.
- README semantic-mass promotion (content-shape signal on section
  bodies): big targeted wins (dockly +0.127, debug +0.074) but evicts
  rank-1 source atoms in small single-file libraries (mitt −0.215,
  p-queue −0.152); the missing piece is a repo-shape gate, not a better
  section signal.

## Known pre-existing walker contract violation (debug-only)

`precis --token-budget 1000000 tests/fixtures/rich` panics at
`src/scheduler.rs:560`: `Python(MethodSigs { markdown.py, chunk 3 })`
overlaps `Python(DeclNames { chunk 1 })` on line 415. Present at
915c1824 (predates wave 3); release builds tolerate it by design.
Surfaces only at budgets far beyond the tested range. Fix belongs in
the Python walker's chunk-boundary construction.

## Divergence open items

- **Ellipsis atoms are credited on schedule content, not rendered
  output (codex adversarial finding, 2026-07-04).** Since the renderer
  invariant landed (markers synthesized from the anchor set; author
  Ellipsis records render no row of their own), an NS Ellipsis atom can
  earn its 1-byte credit while the renderer emits one shared gap marker
  — or nothing, for a blank-only gap. Deliberately NOT changed:
  aligning atomization with rendered deltas would re-price every frozen
  NS's ellipsis rows (goalpost move mid-calibration), and the credit
  quantum is 1 byte per atom. The walker delivering an Ellipsis record
  at the NS's (path, line) does satisfy the NS's semantic want
  ("elision is signaled here"). Revisit as a deliberate metric revision
  at the next NS re-freeze.

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

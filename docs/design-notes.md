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
  Note: the *gated-descendant-value* form of this (scheduler routing
  descendant value into hub `DeclNames`) was measured dead on the
  post-refreeze keys and removed 2026-07-06 — htop was unmoved by its
  removal, and the TS type-only variant was net negative (axios +0.031
  / monaco +0.059 / vite +0.022 vs d2ts −0.051, corpus +0.0009). Any
  retry needs a different mechanism, not a revert.

## Post-refreeze calibration state (waves 3–5, 2026-07-05/06)

Training 0.5250 → 0.5442; oracle 0.8489; buckets late 0.228 / unsched
0.219 / absent 0.107. The absent bucket is largely spent — remaining
loss is ordering/purchase, where the measured pattern is: **recall and
coherence levers keep paying small; broad ranking boosts keep failing
guards**. Session ledgers: `ignore/session-2026-07-05-wave3.md`,
`ignore/session-2026-07-06-wave45.md`.

Measured-dead this cycle (specifics block retries):
- **Config-template extraction** for `.env.sample`-class files: env
  assignment-line selection measured flat (+0.00001) — the content
  enters the pool but never wins purchase within 3K. The one surviving
  piece shipped: root reference/spec YAML key rosters (sqlite-vec
  +0.070).
- **Workflow structural summaries** and **hook/linter rosters**: flat
  to negative; compact whole workflows already cover what NS buys.
- **Broad roster promotion (Go/C)**: neco −0.42 / gin −0.19; only the
  Python signature-roster **ancestor** form (typeguard +0.069) and the
  JS oversize-class analog (commander +0.019) survived, and both needed
  the sibling-ellipsis suppression + predecessor-gating discipline.
- **Require/import reachability tier (JS)**: dockly +0.108 but
  vaul −0.249 / p-queue −0.078 — reachability alone over-promotes in
  small libraries; needs the package-role signal folded in before any
  retry.
- **Script-flow batches** (entry-file expression statements): built,
  measured zero movement, reverted.
- **Dev-doc routing past the peripheral damp** (mdbook/vite/enclosed
  CONTRIBUTING): both variants negative (peepdb −0.066 unflagged);
  mdbook's test-command section is outside the frontier even at 30K —
  treat as NS-rank-vs-feasibility mismatch, revisit at next re-freeze.
- **superstruct struct.ts**: not a ranking miss — the walker emits a
  class slab where NS wants compact field/member/body batches; the fix
  class is TS class-member batch granularity, not value tuning.
  **Mechanical extension measured dead (2026-07-06):** flipping the
  existing JS class-member split (`should_split_js_class_export` +
  member collection) from `is_js_file` to all non-declaration sources
  does split `Struct` into member batches, but they lose the purchase
  race — superstruct headline unmoved, while de-slabbing classes that
  were being bought whole costs ky −0.026 / json-server −0.025 /
  d2ts −0.016. A retry needs member batches that *win purchase inside
  3K* (value-side or scheduler-tier treatment), not just granularity.

## Solo calibration session (2026-07-06/07): 0.5465 → 0.6029 — GOAL MET

Final numbers: training 0.6029 (goal 0.6), validation 0.5007 →
0.5525 (+0.0518, tracking training's +0.0564 — no overfit). The
session-ending lever was the **dir-listing tier re-sweep** (9b701867,
+0.0253 alone; validation +0.0280): the listing (cat, fu, ztu)
presets predated every structural change and were the largest stale
calibration in the tree. Sequence that found it: structural
mechanisms first (map floor, catalogs, breadth pressure), then
neighborhood knob re-sweeps, then the tier presets themselves —
each ship shifted what the next re-sweep could see. Remaining
per-fixture regressions worth a future look: linkwarden −0.111 /
monaco-editor −0.107 / commander −0.048 (listing mass displacing
their config/API anchors).

**Interaction re-sweep (2026-07-07)**: after the session's six
structural changes, the "settled" knobs were re-swept on the new
state — four moved, four+ confirmed: ORIENTATION_TIER_WINDOW
300→500 (+0.0004, tinyusb +0.028), depth_factor slope 0.3→0.35
(+0.0012, never previously swept), PROSE_MASS_BOOST 1.3→1.5
(+0.0010, dockly +0.077), CANONICAL_USAGE_SECTION_FACTOR 1.5→2.2
(+0.0009, tinyusb +0.063). Confirmed at optimum on the new state:
concavity 0.35 (sharp), ORIENTATION_TIER_BOOST 1.4, roster cap 2.2,
PROSE_MASS_WINDOW_FRACTION 0.25, TRAIN_PRESSURE_K 0.15,
BODY_BLOCK/README_SUB scales, GO_ENTRY_FACTOR 1.4, go unexported
0.6, REFERENCE_USAGE 1.3, python init factor 3.0. Lesson: every
structural ship moves nearby knob optima — re-sweep the neighborhood
after each mechanism lands, not once per re-freeze.

Final state after the breadth-pressure mechanism (see its entry
below): training 0.5741, validation 0.5284 — the two moved in
lockstep all session (+0.0276 / +0.0277), no overfit signature.
Remaining gap to the 0.6 goal: 0.0259. The measured-dead lists in
this section and the shipped-guards pattern on the breadth mechanism
are the starting point for the next session; the highest-leverage
open direction is discriminating guards for the two known bimodal
classes (CI bodies, README tail confetti at +0.0001) and the
below-frontier structural buys (member manifests, cmd/root.go —
retried once post-frontier-shift, still inert).

Six commits (`7a9a85a7..11ace1c0`), all training-measured; codex review
clean. The three biggest wins share one theme: **the walker's
tiny-batch bias vs the NS convention of complete maps and
breadth-first surfaces.**

- **Depth-1 listing-map floor** (+0.0134): floor the non-essential
  component of a top-level dir listing's prior at 0.5 — the map entry
  is orientation even when the contents are discounted. Extending the
  floor to all depths measured inert (+0.001 sqlite-vec only): floored
  deep listings still sit below the frontier.
- **Un-ship `small_listing_decay`** (+0.0046): pre-refreeze damp on
  tiny deep listings; on frozen keys the map keeps its tiny entries.
  Known collateral: an early `cmdk/src` listing unlocks per-file
  walkers sooner (see module-item catalog below, which repaid it).
- **TS module-item first-line catalog** (+0.0020, cmdk +0.067 /
  audiobookshelf +0.070): private-emitting entrypoints shipped ~40
  per-item ~15-token batches that ate the early budget; unified
  roster + gated 0-cost items, mirroring every other decl class.
- **Python `concrete_impl_sibling_factor` role split** (+0.0012,
  chronos +0.086): surface roles keep the base.py boost but never the
  sibling damp; depth roles keep the damp but never the boost.
  Uniform application inverted the NS's breadth-first order.
- **Dotenv-sample recall + 100-line build-entrypoint cap** (+0.0003
  at 3K but positive at all 7 budgets; oracle 0.848 → 0.851). Dotenv
  samples ship as a 12-line mandatory-head batch + gated tail — the
  earlier flat "config-template extraction" attempt failed because a
  60-line lump can't win purchase; the head-split is what landed it.
- **Un-ship adaptive long-README decay** (+0.0004, debug +0.031).

Measured-dead this session (don't re-test without new evidence):
- Un-ships that LOST on frozen keys: names-surface chunk falloff
  (−0.0073), TEST_INDEX_LISTING_BOOST (−0.0023), catalog-child
  suppression (−0.0014), python depth≥3 names demotion (−0.0039),
  docs-site subtree damp (−0.0018), dense_md_sibling_factor
  (−0.0008), TS secondary-subpackage damp (−0.0022),
  WORKSPACE_MEMBER_IDENTITY_FACTOR (−0.0004).
- **ci_value de-saturation** (3.2→1.0 cat: −0.0053; xxhash −0.164,
  middleclass −0.100, debug −0.057): NS placement of CI bodies is
  bimodal — several NSes want the ~340-token workflow body ≤3K, and
  they outweigh the p-queue/cmdk-class fixtures where the early body
  displaces tier-2 content. The depth-1 floor made workflow dirs
  reachable early everywhere, so this trade is now live in ~15
  fixtures; a future lever must discriminate, not rescale.
- **README section-mass factor** (boost / two-sided / demote-only:
  −0.0089 / −0.0002 / +0.0025): NSes are split on tiny sections
  (htmy/svgo anchor them; commander/superstruct treat them as
  confetti). Demote-only was net-positive but too small to pay for
  the code.
- **`cmd/root.go` entry treatment (Go)**: principled (visibility
  waiver + entry factor) but metric-inert — root.go batches still sit
  below mcphost's 10K frontier. mcphost cmd/root.go remains the
  single biggest one-file loss (0.313, all unscheduled).
- **Workspace-member manifest ne-exemption (JSON)**: inert — member
  manifest values rise but stay below the 3K frontier (vite −0.022
  the only mover). cmdk's tier-2 manifests remain unbought.
- **Scheduler breadth pressure — blanket variant**: dynamic ratio
  divisor `1/(1 + K·n)` per predecessor-train root on ALL batches:
  K=0.08 → −0.0043, K=0.03 → −0.0063. Superseded: the discriminated
  form SHIPPED at +0.0058 (see c35e8ca8) with three measured guards —
  follow-up classes only (`is_depth_follow_up`), doc-lede and
  entrypoint-body exemptions, and the unopened-substantial-trains
  breadth gate. Guard evidence: no gate → mitt −0.223 /
  go-multierror −0.135; EntryItemBody pressured → otree −0.131;
  Go DeclDoc pressured → bubbletea −0.048. Post-ship knob probes all
  flat or negative: K plateau [0.15, 0.25], FREE 3/6 worse, BREADTH_MIN
  1 flat, adding ClassBody/StructFieldGroup −0.0022 (field rosters are
  surface-like). root.go entry treatment retried on the new frontier:
  still inert.
- **Ops-config class boost (oracle-gap driven)**: the NS-aware oracle
  buys more config/manifest batches ≤3K than the walker (382 vs 282;
  the only class where oracle > walker — from classifying
  `--oracle-schedules` output against walker rows). A 1.3× class
  boost measured −0.0013 blanket, +0.0005 with appendix-class guards
  (TomlKey::Config, Json IdentityMeta/Entry/Runtime excluded),
  +0.0003 value-side root-gated. Irreducible bimodality: chibicc
  +0.086 / linkwarden +0.070 / peepdb +0.054 want root config ≤3K;
  audiobookshelf −0.102 / chronos −0.079 / tock −0.050 want source
  there instead, and both sides include self-hosted apps — no
  walk-time discriminator found. The oracle-vs-walker class
  aggregation method itself is sound and cheap; reuse it after the
  next mechanism shifts the frontier. A README-emphasis router
  (boost only config files name-mentioned in the root README text,
  depth ≤ 1) also measured exactly flat — the mention signal doesn't
  separate the modes either.
- **Post-3K recall extensions all price below the frontier**: TOML
  `Whole` for non-manifest configs (~flat, 1000-budget −0.0013),
  Makefile >100-line head-sample + dotenv tail to 120 (sqlite-vec
  −0.001 only), TS primary-member depth re-rooting (vite −0.040,
  d2ts −0.012, cmdk +0.022), README index-decay floor 0.7 → 0.55
  (exactly flat everywhere). The 3K frontier is the binding
  constraint; absent-bucket recall only pays at B ≥ 4327.

## Extreme-budget contract sweep (2026-07-06): corpus clean

`for f in tests/fixtures/*/; do cargo run -q -- --budget 1000000 $f;
done` (debug build) is the cheap way to shake out walker overlap
contract violations that only surface far past the tested budget
range. As of this session all 93 fixtures pass. Three violations of
this class have been found and fixed so far: rich Python chunk
overlap (wave-4 disjointness fix), tinyusb markdown per-bullet ranges
(`node_end_row_trimmed` whitespace trim), xonsh Python
`block_child_parts` trailing-comment row. Re-run the sweep when
touching span-boundary construction in any walker.

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

- **tomli const-lump acceptance (2026-07-06).** Unifying names surfaces
  (one catalog per file) cost tomli −0.034: `_parser.py`'s const-heavy
  surface now buys all const name lines as one lump where chunking let
  the scheduler defer the low-value tail. Accepted as the price of the
  net-positive simplification (axios +0.071 / pluggy +0.059). If a
  const-heavy-file pattern shows up more broadly, the lever is a
  const/decl split of the names surface, not a return to chunking.

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

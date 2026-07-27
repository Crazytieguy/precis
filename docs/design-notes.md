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

## Gitignored content doesn't belong in precis output either

v0.1 filtered discovery through the `ignore` crate; the v0.2 rewrite
dropped it, and `src/fs_util.rs` justified that with a property of the
*fixture corpus* ("gitignored files don't appear in the fixture set")
rather than of a real working tree. On a live checkout v0.2 spent its
budget on `.git/`, `target/`, virtualenvs and editor state — measured
5.2× slower and unusable on precis's own repo. Restored via
`fs_util::DirFilter`, built once per run and threaded through
`WalkCtx`.

Three decisions worth not re-litigating:

**Gitignore rules apply only when the walk root is itself a repository
root.** Not "an ancestor is a repo". This is the `ignore` crate's
`require_git` default and what ripgrep and `fd` do: ignore rules are a
repository's statement about its own contents, and a tree that isn't a
repository may still carry a stale `.gitignore` describing a build that
never ran there. Ancestor search would also make output depend on rules
outside the summarized tree — `tests/fixtures/<f>` lives *inside*
precis's own repo, so the frozen corpus would start reading precis's
`.gitignore` and each contributor's `core.excludesFile`, and baselines
would stop being reproducible. The cost is that `precis packages/web`
inside a monorepo gets no gitignore filtering; if that becomes a real
complaint, the fix is a repo-root search plus an explicit escape for
roots under the corpus, not silently widening the gate.

**Matching is pattern-only; the index is never read.** A force-added
tracked file that matches an ignore pattern is hidden even though git
doesn't consider it ignored. Same limitation as every ignore-crate
consumer and as precis v0.1. Consulting the index would mean shelling
out to `git ls-files` on every run — not worth it for a fast CLI, and
the failure mode (a checked-in generated asset going unlisted) is rare.

**The heavy-directory blocklist (`walker::fs::should_skip_dir`) stays.**
It is not redundant with the filter: inside a repository these names are
gitignored anyway and it never fires, but precis also runs on trees that
aren't repositories (extracted archives, vendored snapshots, the fixture
corpus) where the filter is inert by design. `.git` is separately and
unconditionally dropped in `list_dir` — git special-cases it rather than
listing it in any ignore file, so gitignore matching alone never hides
it.

The subtle bug in this shape, worth remembering: **a scan that starts
somewhere other than the walk root must ask about ancestors.** A
directory-only pattern (`examples/`) matches the directory and *not* the
files inside it, so per-entry filtering alone lets any traversal seeded
below the ignored directory read and parse the whole subtree. That is
exactly what `walker::rust`'s Cargo source dirs do (they jump straight to
`<package>/examples`): measured 847 ms vs 77 ms on a repo with a
gitignored `examples/` of 400 files, with *no* visible output either way
— pure invisible waste. `DirFilter::excludes_tree` is the ancestor-aware
form; use it at traversal entry points, and plain `excludes` per entry
inside a walk that already pruned its parents.

Corpus impact of the restore: exactly zero — no fixture has a `.git`,
so every baseline regenerated byte-identical.

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

### Settled knobs

The authoritative knob state is the **"Interaction re-sweep
(2026-07-07)"** entry below — earlier "settled, don't re-sweep" lists
(2026-07-05 and before) were each partially overturned by the next
structural ship. Standing rule: settled values are only settled relative
to the batch mix they were measured on; re-sweep the neighborhood after
every structural ship. Two durable sweep caveats: `mix_signals` fu is
non-monotone (midpoint re-sweeps mislead), and isolated single-knob wins
(e.g. a C0 ranking-cost floor) can lose in the combined state —
interaction-check before shipping. Overfit signature to watch: a shift
that raises validation but lowers training — training is the objective
(see `feedback_dont_decide_on_validation_holdout`).

Historical note (mechanism now in code): the old "~0.59 re-ranking
ceiling" belief was a knob-sweep artifact. The loss-decomposition
diagnostic exposed **roster-mass blindness** (size-invariant batch value
vs linear cost bought tiny rosters over the complete catalogs NS authors
anchor on); `value::roster_mass_factor` is the fix class. Durable
adoption discipline: roster/centrality boosts must be gated to a
*structurally selective* tier — class-wide gates measured large
single-fixture collapses before tightening (soluna −0.210 vs
inventory-only monaco +0.213).

### Tested-and-failed lever shapes (specifics block re-tries)

- **(old-key, 2026-06-12) Entrypoint-named `.d.ts` promotion**: both
  global and root-scoped variants regressed, including the target
  fixture — whole-file promotion floods the early budget with the wrong
  exports; the axios `index.d.ts` mass needs per-export granularity, not
  a file-level flip.
- **(old-key) Sibling-count devaluation**: uniform sibling damps regress
  in every tested form — demotion preserves order within the dir but
  lets manifests/README jump the dir's load-bearing primary. A working
  version needs a signal separating "wide-but-shallow sweep" from "one
  primary + helpers"; sibling count alone can't.
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

### Budget-tier scheduler — un-shipped 2026-07-18 (`ef80cfe5`)

The early-budget `ORIENTATION_TIER_*` ratio boost (shipped 2026-05-31)
was removed as exactly neutral — all 71 training headlines identical to
six decimals. Its constants are gone; only `WalkerKey::is_orientation()`
survives, and its meaning changed: it now only keeps an orientation-
rooted train out of the scheduler's `substantial_unopened` count. Two
findings from the tier that still constrain future work: **`FsKey` must
stay excluded** from any orientation promotion — boosting the cheap
dir-listing flood measured negative — and a flat orientation tier is the
wrong shape, because deep-method-heavy fixtures want source early while
orientation-heavy ones don't, and the split crosses languages, so
neither a global exponent nor a per-language override captures it.

### Open

- **Prefix-stop tail effects**: a rank shift can strand a big batch at
  the budget tail where it no longer fits (per-key concavity bump:
  cmdk `walker_used` 9484 → 7302 at B=10K). Score(3000) is blind to
  this; check Score(9000) and high-B `walker_used`. Mitigation lever
  if needed: walker-side filter on absolute cost.
- **(old-key, 2026-06-10) JS class-member seed knobs swept inert** at
  the primary budget in every tested direction — treat as an unswept
  hypothesis on the new keys, but the shape of the result (flat region;
  a JS-class lift needs new recall, not these values) likely holds.
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

## Orchestrated batch-shape session (2026-07-18): 0.6029 → 0.6266

Training +0.0237 / validation +0.0116 (0.5525 → 0.5640; positive but
~half the training rate — this batch leans more training-specific
than the near-1:1 2026-07-07 session; watch the next re-freeze).
~15 shipped mechanisms, all on one unifying diagnosis: **NS keys buy
mid-grained 100–450-token slices (the growth-envelope size); walker
batch classes far above lose the purchase race wholesale, far below
queue-jump as crumbs or divide value away.** Mechanism families that
paid: oversize head/tail splits (markdown sections, Rust crate docs,
TS catalogs/class slabs, Python rosters, Dockerfile/compose/dotenv
ops files — all value-CONSERVED after the adversarial review caught
1.6–1.8× replication in the first two splitters), crumb coalescing
(C struct groups, Rust nested-entry fragments; Go wanted comment
elision instead), manifest de-chaining + dependency-class splits,
structural role recognition (primary workspace member by name-match
— WRONG signal for Rust workspaces, sps/toasty need dependency
centrality; config-surface headers; operational README sections;
manifest-entry public-surface seeding), and two knob re-sweeps
(listing tiers stalest again; concavity 0.35 razor-sharp both
passes). Durable process lessons: (a) re-measure every lane's win on
the COMBINED tree — two clean lanes interacted −0.067 on superstruct
via a boost tuned pre-manifest-landscape; (b) adversarial review
keeps paying (value replication, tail-deletes-head rows, config-
floor amplification, manifest path traversal); (c) per-fixture
regen means compose almost perfectly additively when mechanisms are
file-disjoint. Dead-lever specifics are in the measured-dead
sections above; next-session queue: ignore/next-session-queue-
2026-07-18.md.

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
concavity 0.35 (sharp), ORIENTATION_TIER_BOOST 1.4 (both
ORIENTATION_TIER_* since deleted — see the budget-tier entry), roster cap 2.2,
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

## Markdown README prelude recall (2026-07-26): 0.6302 → 0.6321

The pre-first-heading region of a README was read only by
`ReadmeHeadline`, which takes at most two blocks of it (one lede plus
a tagline extension). No `Section` range reaches above the first
heading, so the rest of the prelude was **pool-absent** — 875 NS
tokens ≤3K across 11 training fixtures in the 2026-07-26 recall
census. Shipped as `MarkdownKey::Prelude { file }`: the *substantive*
prelude blocks the headline left behind, predecessor `ReadmeHeadline`,
priced at 1.3× the README index-0 section value. Movers: microbootstrap
+0.080, audiobookshelf +0.030, cobra +0.029, ts-pattern −0.004.

- **Chrome must not ride along — measured, not assumed.** The first
  cut emitted *every* uncovered prelude row (badges, logo wrappers,
  screenshots) because several NSes rank the hero region whole (chalk
  1.10 is literally "lede + badges + screenshot", middleclass 1.3 is
  "README badges"). Corpus mean −0.0061: mitt −0.226, dockly −0.083,
  tomli −0.057, neco −0.054, linkding −0.044 against chalk +0.028,
  swarm/soluna/cmdk ≈+0.006 each. A badge wall is ~40 tokens of URL
  per row at the very top of the schedule; the NSes that want it are
  a minority and the ones that don't pay for it immediately. Whether
  an NS ranks the hero is not a walker-visible signal — don't retry a
  discriminator for it.
- **The same lesson kills the section-0 half of the hole.** README
  section 0 starts at `max(headline_last_row + 1, range.start)`, so
  rows the headline *stepped over* inside its own range (middleclass's
  badge rows between the setext title and the lede) are dropped too.
  Replacing the prefix cut with a set difference over
  `covered_rows` measured −0.0033 (xlstm −0.064, anyhow −0.050,
  requests −0.050, thiserror −0.040, go-multierror −0.038 vs xxhash
  +0.012, sqlite-vec +0.009, middleclass +0.008): the recovered rows
  are chrome, and inflating a high-value early section's cost with
  them costs more than they return. The prefix cut is correct; its
  comment now says why.
- **Two chrome classifiers were too narrow and both fixes paid.**
  (a) `is_html_nav_block` only matched in-page anchors (`href="#"`),
  so an absolute-URL link bar (`<a>Demo</a> · <a>Docs</a> · <a>CLI</a>`)
  read as prose — it is now recognized by stripping the anchors and
  checking the residue is separator punctuation. This also fires in
  `ReadmeHeadline`, which is where audiobookshelf's +0.030 comes from:
  its headline was spending 10 rows on a banner + link bar instead of
  the "what is this" sentence under `# About`. (b) A badge wall
  written as raw HTML inside a *markdown paragraph*
  (`<a …><img …></a>` per line, linkwarden) was not decorative because
  `is_decorative_paragraph` only inspects markdown inline nodes; the
  tag-stripping test now applies to paragraphs too, gated on seeing a
  real element tag so `<https://…>` autolinks stay content.
- **Value factor is flat over [1.3, 1.6]** (identical per-fixture rows);
  1.0 loses cobra (its 211-token prelude prices at 4889 instead of
  ≤3K) for −0.0004. Shipped at 1.3.
- **Known residue.** cobra's prelude drags in an eight-row Warp
  sponsorship `<div>` — an `<img>` with a 13-character caption, which
  is decoration but not tag-only. A "short caption + image ⇒ chrome"
  rule would need a character threshold that separates it from
  linkwarden's 29-character `<h1>Linkwarden</h1><h3>Bookmarks,
  Evolved</h3>` title block; two data points is not enough to set one.
  ts-pattern's −0.004 is its top-of-README `tsx` demo fence, which no
  NS row ranks — kept deliberately: it is the same construct as
  microbootstrap's fence, which is worth +0.080.
- **Under-heading README body prose is NOT this class.** Every H2
  becomes a `Section` batch, so a README that renders as headings with
  elided bodies (chalk: 38 section batches spanning cum 1227–9428) is
  *priced-out*, not pool-absent — a different, harder problem.

## Markdown oversize head-split (2026-07-18): 0.6029 → 0.6047

Root-README sections ≥ 400 tokens that no structural split catches
now emit head (kind `Whole`, keeps flags) + chained `OversizeTail`
chunks (0.85× parent), cut at blank lines outside fences, ~200-token
target. Movers: sqlite-vec +0.065, chibicc +0.039, sds +0.033;
worst −0.004. Calibration knowledge from the tuning loop:

- **Token-gate, not byte-gate.** go-multierror's 966-token Migrating
  section is only 2.3 KB — a byte threshold sized for prose misses
  fence-heavy sections entirely.
- **Scope = root README only, measured.** Splitting UPDATING.md
  (middleclass) handed a cheap full-value head to NS-late content:
  the head displaced 370 tokens of source inside 3K (−0.11 on the
  fixture). Adding UPDATING to changelog-class was worse (−0.196):
  the guide cat 0.5 > peripheral 0.3 pulled the whole lump ≤3K.
  Peripheral-doc lumps are correctly priced by their size.
- **Tail factor 0.85, not BodyBlock's 0.60.** At 0.60 the tails
  strand past the window their head opened (middleclass README tail
  at 3618 vs the old whole-lump at 1634); 0.85 restores near-lump
  train completion while the head still buys early. Tails are also
  exempt from the `deferred_mass_prose` pass — deferring a mid-train
  tail strands everything gated behind it.
- **README-late fixtures with content past ~4K stayed flat at 3K**
  (go-multierror, debug, cmdk, commander): the split re-orders their
  README delivery but the chunks still price past the 3K frontier.
  Their gains show up at 5–7K rows.

## C-cluster session (2026-07-18): crumb coalescing + dominant-binary roster

Two shipped C-walker changes, mean 0.6029 → 0.6034:

- **Struct field-group crumb coalescing** (+0.0003; htop +0.025,
  chibicc −0.004 at 3K but up at every later NS row): merge adjacent
  blank-line field groups below `AGGREGATE_STRUCT_GROUP_MIN_ROWS` (5)
  content rows. 1–2-line crumbs are near-free and queue-jump the
  `value/cost^0.35` race. MIN swept 4/5/6: 4 leaves htop confetti
  partially intact; 5 ≡ 6.
- **Dominant-binary names-surface promotion** (+0.0002; krep +0.010
  at 3K, larger gains at 5–6K): a non-test `.c` with ≥60% of C source
  lines *and* a default-configuration `main` prices its DeclNames at
  the header tier. Measured guards: (a) library variant (no main
  gate) → neco −0.230 / sds −0.004 — a dominant *library* impl must
  not out-bid its API header; (b) main polarity matters — sds's
  `#ifdef SDS_TEST_MAIN` main is not a binary marker, krep's
  `#if !defined(TESTING)` is; (c) promoting `decl_value` too is a
  no-op on the gated set (train rows are ~free once the surface
  lands); (d) extending `roster_mass_factor` to the dominant file:
  krep 0.384 → 0.377 — earlier arrival displaces NS-wanted README
  orientation.

## Rust wave (2026-07-18): 0.6047 → 0.6067

Two ships (crate-doc oversize split bfd4076b, crate-attribute recall
23e0c914 + review fixes 05916e36) and one measured-dead attempt:

- **Crate-doc chunk placement is bimodal across NSes**: thiserror
  ranks doc slices ≤2.5K, anyhow ranks the identical shape ~7K. The
  shipped balance (head 0.9×, tails 0.75×) holds both headlines; the
  residual trade is anyhow −0.03..−0.07 on 4.7-6.5K rows vs thiserror
  gains at 3-3.6K and 6.2-7.4K. `CrateDocTail` scheduler-trait
  defaults (flat concavity, no breadth pressure) are unswept — see
  the key's doc for the candidate levers if the tail train over-buys.
- **MethodSigs per-impl-group split — measured dead (reverted, three
  variants)**: splitting oversize (≥400-token) catalogs per self type
  (All-scope full sigs / first-line roster / ExportedOnly membership)
  never moved Score(3000) — log's per-type rosters still price at
  3.5-6.5K, past the NS windows that want them (the early-budget
  ratio wall again), and the mechanism only fires on log among
  training fixtures (toasty/otree/hyperfine catalogs are single-self-
  type). The ExportedOnly variant was curve-positive on log
  (+0.066 summed, mixed on the budget grid) but flat-primary +
  single-fixture doesn't pay for ~180 lines. A retry needs the
  groups to *win purchase ≤3K* (value/tier treatment), not more
  granularity shapes.
- **CrateAttrs gate provenance**: 150-token floor and primary-crate-
  root scope are each backed by a measured failure (hyperfine −0.050
  / toasty −0.013 ungated; log 140-token lint list −0.02..−0.05 on
  4-8K rows; thiserror secondary impl crate −0.04 even damped).

## Rust per-impl-method recall (2026-07-26): 0.6302 → 0.6305

Closed the largest pool-absent class in the corpus: `RustKey` had no
per-method key and `item_kind_of` no `impl_item` arm, so **no inherent-
impl method body was reachable at any budget in any Rust fixture** (950
NS atoms, 8 of 8 Rust training fixtures). Shipped `ImplMethod` /
`ImplMethodBody`, predecessor-gated on the file's existing `MethodSigs`
roster — the roster already names every method it renders, so it is a
genuine entry ticket instead of a dead end.

Corpus means on the 7-budget grid (71 training fixtures):
`0.6302→0.6305` @3K, `0.5924→0.5932` @4.3K, `0.5619→0.5627` @6.2K,
`0.5429→0.5436` @9K; ≤2K unchanged. Only thiserror moves the 3K
headline (+0.017); anyhow gains +0.044/+0.039/+0.030 at 4.3/6.2/9K.
sps −0.012 @9K is the only regression at any budget.

Load-bearing findings, in rough order of how much they'd cost to
rediscover:

- **The ≤3K Rust impl-method mass is roster-shaped, not body-shaped.**
  Every ≤3K NS row on the affected fixtures (anyhow "Error impl method
  roster" @2366, log "Level & LevelFilter public method roster" @1945
  and "Record accessor method roster" @2557, sps keg.rs @2147) wants
  *name-only, truncated-at-the-paren signatures*, and every one of them
  is **priced-out, not absent** — `MethodSigs` for anyhow `src/error.rs`
  lands at ~3.4K against an NS slot of 2366. The bodies this lane made
  reachable sit at NS ranks 3.5K+ (otree cmd.rs `update_config` @4283,
  `get_content_type` @3824), which is why a 950-atom recall win buys
  +0.0003 at the primary budget. Whoever picks up the roster-pricing
  half should not expect it to be blocked by recall.
- **Visibility: inherent vs trait impls are filtered differently, and
  the NSes say so.** `is_own_api_impl` (was `is_exported_method`) now
  admits *every* method of an inherent impl regardless of `pub`, and
  still requires a crate-public trait for a trait impl. anyhow's NS row
  is explicit — "every method on `Error`/`ErrorImpl`, public or
  private … Trait-impl methods (Display/Debug/Drop/Deref/From/AsRef)
  are a separate, self-evident class and are deliberately not part of
  this roster" — and thiserror `impl/src/prop.rs` ranks a roster of
  `pub(crate)` methods. Worth +0.0006 at 4.3–9K, flat at 3K, and it
  *removes* a filter.

Measured-dead in the same lane (specifics block retries):

- **Uniform one-line-per-method roster shape** (drop the entrypoint
  `All` scope's full multi-line signatures now that `ImplMethod` renders
  them): hyperfine −0.076 @3K, mdbook −0.016, mean −0.0013, nothing
  gained. An entrypoint roster is doing signature work, not index work.
- **Oversize-roster fallback to the one-line shape** (≥400 rendered
  tokens, aimed at log's 2016-token `lib.rs` roster): flat @3K,
  hyperfine −0.032 @4.3K, toasty −0.073 @9K. log was unmoved — the
  fallback shape is still large because `All` membership includes every
  trait impl.
- **`ModUse` gate widening to `is_package_source_file`** (the second,
  separable half of the diagnosis's recommendation): mean −0.0013,
  anyhow −0.051, thiserror −0.038. Independently confirms the existing
  in-code note that opening `ModUse` up floods the mid-budget.
- **Trait-impl value damp** (0.6 on the signature / 0.8 on the body,
  on the theory that a trait impl's method set is predictable from the
  trait): byte-identical to no damp at all 7 budgets. The
  inherent/trait distinction is real for *membership* and for the
  visibility axis, inert for *value*.
- **`ImplMethodDoc`** (Python's `MethodDoc` analog): moved no fixture at
  any budget. Same outcome as the `StructFieldGroupDoc` follow-up in the
  Go ledger — a per-member doc key does not pay for itself.

Incidental correctness fix: `collect_item_lines` emitted a body-elision
marker at `sig_end + 2` for a one-line body (`fn f() {}`), i.e. on the
*next* item's line. Latent for `PubItem`; per-method batches hit it
immediately (log `src/lib.rs:1290`/`1291` panicked the non-ancestor
overlap assert). Regression test
`rust_one_line_fn_body_emits_no_elision_marker`.

Still absent after this lane (from the same diagnosis): otree's
`cmd.rs` clap struct prices as one atomic 1047-token `PubItem` because
Rust has no `PubItem` size splitter — a pricing/granularity gap, and
note the adjacent "oversized public Rust struct/enum head-split" is
already measured dead, so a retry needs a value mechanism.

## Python roster toll gate (2026-07-26): the gate is not the lever, the race is

Diagnosis said Python per-decl batches are stranded behind an expensive
`DeclNames` roster (88 files, 16 of 18 Python fixtures). Measured on
`rich` with a standalone pool ranking (empty tree, `value/cost^0.35`):
`console.py`'s roster is value 899.6 / cost **591** / ratio **96.4**,
rank 2037 of 5932, while the 3K frontier sits at ratio **~182** — the
one-line rosters of trinket modules (`measure.py` 562.2/25/182.2,
`pager.py`, `_null_file.py`) — and the file's own decls rank 9–170
(ratio 300–393). **The gap is ~1.9× and every roster-side lever is
bounded below it**: a conserving split lifts the head by
`1.4 · share^0.02` ≈ 1.35×, truncating the roster's rendering ≈1.33×,
the roster-mass cap raise is already dead (2026-07-19). Chunk-gating
was already dead. So the toll gate is closed as a *ranking* lever.

Measured dead this lane (specifics block retries):

- **Principal-declaration promotion** — for a file whose roster costs
  >250 tokens, drop its leading `N` roster entries and emit those
  `Decl` batches ungated (roster ∪ principals still names everything;
  no value minted, only a predecessor edge and a few roster lines
  moved). Overlap-safe by construction, so it is *not* the dead
  head-chunk gate. N=4: mean **−0.0055** (beets −0.141, requests
  −0.087, linkding −0.059, tomli −0.046). N=1: **−0.0006**. Failure
  mode is structural, not magnitude: `decl_value` (908.7 on rich) is
  calibrated for a batch bought *behind* a roster at ~0 marginal cost,
  so a freed decl out-ranks nearly everything and delivers an isolated
  `class Foo:` line. **A free-standing decl needs different pricing
  than a gated one; any retry must reprice, not just re-gate.** The
  target fixture did not even move (rich flat at both N).

Shipped instead — **two-sided `python_roster_mass_factor`** (drop the
`≥ 1.0` clamp, keeping baseline 6 and cap 1.6). Don't raise the
flagship, demote the trinkets: a 1-decl roster now prices at 0.53×, a
5-decl one at 0.94×. 3K mean **0.6340 → 0.6353**; across the grid
+0.0003 / **+0.0048** / +0.0014 / +0.0013 / +0.0008 / −0.0016 / −0.0005
at 1000–9000. Movers: linkding +0.085 (README feature overview lands
instead of six 1-class module rosters), nano-vllm +0.023, peepdb
+0.007, htmy +0.003, **xlstm −0.019** — the honest cost: in a repo
whose flagship modules genuinely are 1–2-decl config dataclasses, the
demotion hits the NS-wanted rosters. Baseline 8 (two-sided) is the
same curve shifted: 3K flat (0.6339) but 2080 +0.0031 and 6240 +0.0007
— sweep baseline and cap together if this is revisited.

Instrument note: the Python names-surface splitter is **inert** —
`DeclNamesChunk` has 0 scheduled rows at any budget ≤10K across all 71
training fixtures, and no Python file in rich/beets/flask/click splits
at all (the tiny-tail fold at `python.rs` absorbs the second chunk when
total ≤ ~675 tokens). `DECL_NAMES_SPLIT_THRESHOLD_TOKENS` /
`DECL_NAMES_CHUNK_TARGET_TOKENS` / `conserved_catalog_chunk_factors` on
the Python path are removable dead weight, not a live knob.

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

## Go struct field-group elision ledger (2026-07-18, wave3-go)

Shipped: comment-only rows elided from chunked Go struct field groups
(50b9487d) — the C walker's struct-render convention, moved to a
shared `comment_only_rows` in `walker/mod.rs`. Mean 0.605155 →
0.605465; bubbletea +0.028 (up at every budget), gin +0.033 at 3K,
cobra −0.039. Accepted divergence, same as C: struct-interior comment
rows of chunked structs belong to no batch at ANY budget (codex
adversarial flagged this; see rejected recovery below). cobra/gin
NSes are bimodal about struct docs — cobra 2.4/2.5 and gin Engine
part 1 rank doc-inclusive slices, which costs cobra ~0.03–0.05 on
rows past 3K and gin ~0.02–0.03 at 3.4–4.7K, offset corpus-wide by
bubbletea's +0.026 mean per-row.

Measured-dead in the same session (don't re-test without new
evidence):
- **Crumb coalescing on top of elision** (C-ship mirror,
  `≥5`/`≥3`-row minimums): identical to worse vs plain elision on
  every fixture — post-elision crumbs ARE the NS roster lines, so
  merging only delays delivery. Coalescing WITHOUT elision is worse
  than base (cobra −0.052): Go groups carry doc rows, so merged
  groups inflate with bytes no NS roster wants (unlike C, where
  elision predated the coalesce ship).
- **`StructFieldGroupDoc` gated follow-up** (re-ships elided rows,
  DeclDoc-priced, predecessor = its field group): Score(3000)-neutral
  on all three movers, slightly negative at 4–7K, positive only on
  cobra's ≥7.4K tail — does not pay for a new key variant. This is
  exactly codex's recommended recovery; measured before rejection.
- **Size-targeted DeclNames chunk partition** (greedy byte-mass
  ranges, 800/1200-byte targets ≈ NS roster band): large loss at 3K
  on every chunked fixture (bubbletea −0.13, cobra −0.09, gin −0.09,
  migrate −0.06 vs post-elision state). Mechanism: per-batch DeclNames
  value is size-invariant (roster_mass on DeclNames is measured-dead)
  so meatier chunks always lose the early-budget ratio race. The
  8-decl fixed chunk is at/past the ratio-optimal size;
  kind-grouped partitions would be coarser still — same wall.

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

## Measured-dead, 2026-07-18 codex lanes (specifics block retries)

- **Python body-statement coalescing** (swarm run / nano-vllm
  generate+step targets): five variants — value-share-conserving
  merge at 200–350 tok, equal split over fewer units, general
  coalesced-body keys (−0.0002, swarm −0.011), blank-line boundaries
  with 700-tok ceiling, coalesced pricing only ≥200-tok units
  (nano-vllm −0.019) — best exactly neutral. Coalesced bodies still
  price below the ≤10K frontier: the blocker is the predecessor/value
  model (1/n-valued body parts under a surface gate), not
  granularity. A retry needs a value/tier mechanism that lets a
  known-NS-anchored body win purchase, not another batch shape.
- **JS/TS private non-class statement recall via reference gate**
  (svgo resolvePluginConfig target): JS+TS gate −0.001 mean with
  json-server −0.069; JS-only retune +0.00001 with svgo flat and
  mixed high-budget regressions. The referenced-private recall class
  doesn't pay at current pricing; svgo's helper stays absent.
- **TS full-declaration ≥450 split (all exported decls)**: p-queue
  −0.058, mean −0.0008 — only the class-surface variant with the
  export-roster boost survives (shipped). Interface/JSDoc-slice
  granularity for p-queue remains an open recall gap, not a split
  problem.

- **C conditional include-switchboard ungate** (tinyusb 3.1 target):
  split-by-conditional-block + names-tier pricing + ungate measured
  tinyusb 0.530 → 0.504 — the host block surfaces early but
  displaces README-aligned content; the 665-tok device block still
  misses 3K. Plain-list gating confirmed right.
- **C non-essential source names floor 0.5** (krep test rosters):
  unguarded costs neco −0.031; with a ≥4-decl guard exactly flat —
  the target rosters stay at ~8K regardless. The floor can't beat
  the 0.2 non-essential damp's distance from the frontier.

- **Python roster-chunk decl gating** (post-conservation): head-chunk
  gate violates ancestor-only overlap (panics — later chunks overlap
  the decl subtree); declaration-owning-chunk gate is headline-flat
  and tomli-curve-negative (−0.03..−0.04 at 3.6K/5.1K/10K). Gating
  behind the LAST chunk is the measured optimum.
- **Primary-member subtree depth re-root (JS/TS), post-gate retry**:
  dead at full/50%/10% strength (full: d2ts 0.405 → 0.330, mean
  −0.0008) — promoted subtree rosters displace already-reached
  anchors without scheduling the wanted class bodies. Confirms the
  pre-refreeze dead result under the new frontier; d2ts's residual
  loss is body-purchase, not rank.
- **Oversized public Rust struct/enum head-split**: at a 400-token
  gate nothing in the corpus crosses (the cited mdbook 562-token row
  is two decls of 218+317); at 200 log −0.045; at 320 all targets
  flat — chained field/variant tails never schedule. The Rust
  whole-decl atomicity loss (sps Formula/Cask class) needs a value
  mechanism, not granularity.

- **README-usage symbol-match decl promotion (2026-07-18, the
  novel-discriminator attempt on the primary-body cluster)**: dead in
  three variants (uniform 1.25x/2x, cost-ramped 1-3x, cap 3;
  −0.0008..−0.0031). Failure modes: generic demo identifiers consume
  the cap (superstruct: string/number/is, not assert/create);
  exact-name routing picks wrapper files (svgo-node.js over svgo.js);
  README evidence goes stale across fixture revisions (cobra); and
  even a successfully promoted signature leaves its BODY unpurchased
  (swarm run sig 3247→1121, target row 0.619→0.518). Seventh
  confirmation: this cluster is body-purchase/shape, not rank. Also
  measured dead same day: go.mod dep split — conserved splits of an
  ALREADY-PURCHASABLE batch reduce aggregate purchasing power under
  concave pricing (the split family only pays where the fused batch
  cannot win at all).

- **2026-07-18 evening batch (ideation-driven, mostly dead)**: Go
  import-crumb suppression (7 variants — freed tokens never buy
  NS-aligned replacements); declared-member manifest tier (best
  +0.0002 curve-mixed, single-member workspaces −0.232 edge, ~100
  LOC doesn't pay — cmdk's oracle-visible manifest gap does not
  convert); Python __init__ re-export depth waiver (directionally
  correct — xlstm targets 8K→1.5K — but displaces 3K aggregates;
  caps can't separate xlstm's 7 targets from chronos's transitive 9).
  Shipped from the batch: root SQL contract walker (+0.0009,
  sqlite-vec +0.067). Meta-lesson: at ~0.630 the tree is at a local
  optimum for value-side re-ranking; only new recall or purchasable
  shape pays.

## Ideation sweep (2026-07-19): next queue drafted, three recall holes verified

Session scope: adopted `integration` onto `v0.2-rewrite` (ff to
c2e200ff; 405/405 tests, clippy clean, training mean 0.6302), then a
four-agent read-only sweep over the lowest-Score and lowest-I training
fixtures. Ranked queue + full evidence in
`ignore/next-session-queue-2026-07-19.md` and
`ignore/ideation-2026-07-19/*.md` (gitignored — regenerate from the
divergence reports if lost). Durable facts found (verified in src, not
yet fixed):

- **`.mts`/`.cts` files were never walked** (`is_ts_or_tsx_file`,
  src/walker/typescript.rs) — FIXED same session (linkwarden +0.001,
  vitest.config.mts now schedules at its NS slot; corpus otherwise
  unchanged).
- **`LICENSE.md` has no owning walker** while bare `LICENSE` is
  value-floored; cmdk's rank-1 NS row has zero walker rows at any
  budget.
- **Root tool-config files (`.flake8`, `.pylintrc`, `tox.ini`,
  `.readthedocs.yml`) have no owning walker** (`classify_plaintext` /
  `yaml_class` both pass on them).
- The low-I cluster (htop 0.42 / cmdk 0.46 / rich 0.59 / flask 0.65)
  is dominated by the `roster_mass_factor` [1.0, 1.6] clamp losing the
  `value/cost^0.35` race exactly at NS-primary flagship catalogs
  (htop linux/ @9523 vs NS @1299 needs ≈2.09). Probed same session:
  blanket cap 1.6→2.2 is −0.0025 mean (lo −0.112, thiserror −0.040,
  linkwarden −0.028) and htop stays FLAT — the 07-18 down-sweep to
  1.6 is confirmed on this frontier, the blanket raise is dead, and
  htop's listing race is evidently not cap-controlled (diagnose
  before any discriminated variant).

## Walker / value open items

- **Rust primary-member election by dependency centrality: SHIPPED
  2026-07-26** as `MemberFacts` (`src/walker/toml.rs`) +
  `MemberRole` / `RustState::primary_member` /
  `CENTRALITY_PRIMARY_MEMBER_FACTOR` (`src/walker/rust.rs`), replacing
  the name-match-only signal the
  2026-07-18 session flagged as wrong for Rust workspaces. Rule: when
  the repo-basename-matched member has no sibling dependents (or there
  is no name match), elect the member with the strictly highest
  in-degree among intra-workspace `[dependencies]` edges, ≥2 dependents.
  **An edge must RESOLVE to a sibling member, never merely share its
  name.** Three accepted forms: the entry's own `path`; `workspace =
  true` looked up through the root `[workspace.dependencies]` table;
  and a registry entry that a root `[patch.*]` table redirects at a
  member by path. All three occur in the corpus (mdbook and toasty use
  the second, sps the third — sps declares siblings by version, so a
  `path`-only reader sees an edgeless workspace). Matching on package
  name alone — the first cut — counts a genuine crates.io dependency
  that happens to share a member's name as internal, and two such
  consumers are enough to elect a hub nothing depends on; that is a
  silent wrong answer on repos the corpus doesn't contain, so name-only
  matching is not an acceptable shortcut here. Each dependent→dependee
  pair counts once regardless of how many entries route to it.
  **The lift is partial (0.85, not the name match's 1.0)** and that is
  the load-bearing part: the sps trade curve is sharply non-linear.
  Measured Score(3000) / worst 7.5–10K `Score(B=cum)` dip on sps:
  0.70 (base) 0.430 / —; 0.85 **0.451 / −0.014**; 0.90 0.456 / −0.075;
  1.00 0.456 / −0.102. The full lift buys no more early budget than the
  partial one and displaces seven times more already-scheduled content
  (NS 5.x `sps/src/cli/*` structs) out of the tail. Corpus +0.0003
  (0.6302 → 0.6305); only sps moves, mdbook re-orders at a flat
  headline, toasty/thiserror untouched.
  Also measured and rejected: hub *replaces* the name match
  unconditionally (toasty → toasty-core; Score(3000) identical, −0.001
  to −0.002 on toasty's 9K rows — pure churn, so a name match with
  dependents keeps the slot); hub promoted *alongside* the name match
  (sps 0.454, and the same −0.09 tail collapse — the displacement is
  the promotion's volume, not the demotion of the shell crate).
  Residual: sps NS 5.4 (`cli.rs` Command enum, NS cum 6830) no longer
  renders at the 10K snapshot budget. The generalization to JS/TS
  workspaces and Go modules is untouched and unmeasured — deliberately
  out of scope so the Rust measurement stayed readable.
  **Role identity and value factor are separate axes, deliberately.**
  `MemberRole::is_primary` gates the primary-only batch classes (today
  just `CrateAttrs`); `MemberRole::value_factor` prices confidence.
  A first cut derived the predicate from the factor
  (`factor < 1.0 == secondary`), which silently classified the elected
  hub as secondary and withheld `CrateAttrs` from the very crate it had
  just called primary — caught in adversarial review. Never re-derive
  the role from the factor.
  Note this also means the 1.0-vs-0.85 sweep above conflated two
  changes: at 1.0 the hub cleared the `< 1.0` predicate and was
  `CrateAttrs`-eligible, at 0.85 it wasn't. Granting the hub
  `CrateAttrs` at 0.85 was then measured separately and is **exactly
  inert on this corpus** — not one baseline byte moves. Cause is
  absence of input, not absence of effect: `collect_crate_attr_lines`
  only fires on inner `#![…]` items, and both elected hubs
  (`sps-common/src/lib.rs`, `mdbook-core/src/lib.rs`) have zero of
  them; no Rust workspace fixture emits a `CrateAttrs` batch at all.
  So the "crate attrs are orientation-dense, this should pay" prior is
  untested here rather than refuted — it needs a fixture whose hub
  carries a ≥`CRATE_ATTRS_MIN_TOKENS` attribute block.
  **Two further correctness rules, both fail-closed.** (a) The name
  match resolves on package name when several member *dirs* share the
  repo basename (`crates/acme` + `tools/acme`); picking whichever the
  member `HashSet` yields first let process hash order decide which
  subtree got damped, which is a bug on its own terms whichever crate
  is the better answer. (b) If any member manifest fails to parse or
  declares no package, `MemberFacts::complete` is false and the
  centrality election is skipped entirely, falling back to name-match
  behaviour — the unreadable member's edges are exactly the ones that
  could have changed the answer.
  **Corpus-safety caveat, acknowledged not resolved:** ≥2 and 0.85 were
  tuned on sps, and four training workspaces (sps, toasty, mdbook,
  thiserror) do not establish field safety. Mitigation is that the
  wrong answers are now bounded by construction — unresolvable edges,
  colliding names, and partial manifests all fail closed to prior
  behaviour rather than electing something. The stricter edge
  resolution left every elected crate in the corpus unchanged (zero
  baseline movement), so the 0.85 knee measured under name-only
  matching still stands; re-sweep if a future fixture's election moves.

- **Config-surface header role (bareiron/tinyusb class): SHIPPED
  2026-07-18** as `is_configuration_surface_header` +
  `CONFIGURATION_SURFACE_VALUE_FLOOR` in `src/walker/c.rs` (initial
  role commit, then bounded-promotion repair capping floored batches
  to the leading names/doc/aggregate slots; floor re-swept 1073 ->
  1250). Original diagnosis: macro-dense headers with
  comment-annotated object-like config runs and few function decls
  (bareiron globals.h, tusb_option.h) left their aggregate + config-doc
  batches at low value. Kept here for the residual: promotion is
  positional (first-N batches), so headers that open with preamble
  instead of settings still miss the floor.

- **Split-batch invariant: a descendant must not emit Ellipsis records
  on lines its ancestor renders as content (codex adversarial finding,
  2026-07-18).** `RenderedTree::apply_spans` replaces ancestor-owned
  records unconditionally, so a tail batch's gap ellipsis on a
  head-owned line demotes the already-paid-for row to `…` in the final
  render. Divergence scoring atomizes schedule content, not rendered
  output, so this deletion is invisible to `Score` — it only shows in
  the render. The Dockerfile split's complementary tail therefore
  ships full-lines-only (head ∪ tail covers the file; the renderer
  synthesizes gap markers from the anchor set). The dotenv tail was
  never affected (its single ellipsis points past both slices).
  Regression test: `plaintext_dockerfile_split_head_plus_tail_renders_whole_file`.
- **Dockerfile head/tail split high-cum curve (2026-07-18, open).**
  The split (head = stage/contract skeleton, tail = build mechanics at
  0.85×, files >40 lines) nets +0.0004 corpus mean at 3K (linkwarden
  +0.057, audiobookshelf −0.026, enclosed 0.000) but leaves
  `Score(B=cum)` dips on the changed fixtures' 3–6K rows (worst:
  linkwarden 1.16 @4020 0.383→0.341, enclosed 2.3 @3143 0.616→0.588,
  audiobookshelf 2.6 @5274 0.742→0.722; audiobookshelf's 6.6K+ rows
  gain). The dips are cost displacement from the mechanism itself, not
  the ellipsis bug above — they were unchanged by that fix. Per-row
  data before retuning `DOCKERFILE_TAIL_FACTOR` / `DOCKERFILE_SPLIT_MIN_LINES`;
  don't retune blind.
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

## Tree-level elision marker (2026-07-26): magnitude measured dead

A blind-usability probe (6 agents, 6 languages, answering real questions
from precis output alone) found the top defect was that a pruned tree
entry rendered byte-identically to an empty one — three confidently
*wrong* answers came from readers taking marker-absence as proof of
emptiness. Fixed by trailing ` …` on any entry row that shows none of
its contents while holding some; contract lives on `format_entry_row`.

Carrying the magnitude alongside the marker (`linux/ …36` for
directories, files left bare) is **measured dead**: mean 0.6280 vs
0.6323 for the bare marker, over the same 71 fixtures. The count costs
+3 tokens per directory row against +1 for the bare glyph, and the
extra two tokens land early enough in the schedule to push `vaul`'s
1170-token batch over the 3K line (−0.232 on that fixture alone) and
cost monaco-editor −0.065. A per-file magnitude is worse still — files
are the bulk of entry rows, so the +2 applies everywhere.

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

## Batch code-review pass (2026-07-18): conserved catalog-chunk allocation

Codex adversarial review flagged that both new catalog splitters
replicated value across chunks (python `DeclNamesChunk` factors
`1.0/0.8/0.67`, TS `ExportMemberNamesChunk` `0.9/0.72/0.6` — a 2-chunk
split carried 1.6–1.8× the unsplit catalog's aggregate value).
Shipped `conserved_catalog_chunk_factors` (value.rs): factors sum to
exactly 1; head = `share^k × 1.4` capped at `0.9` (`k` =
`CATALOG_ROSTER_CONCAVITY_EXPONENT = 0.37`, now shared with the
batch-key exponents); tails split the remainder cost-proportionally
with the names-surface falloff.

Measured frontier (training corpus): mean 0.6151 → 0.6147.
- Pure cost-share conservation: 0.6139 (tomli −0.064: `_parser.py`
  roster head slipped 1224 → 2726 cum, out of its NS window).
- Head ratio-parity (`share^k`, no premium): 0.6143.
- Premium 1.4/cap 0.9 (shipped): head back at 1256; commander fully
  recovered; htmy +0.008; tomli −0.038 residual.

**`CATALOG_HEAD_PREMIUM` is live, not inert** (measured 2026-07-26 —
corrects a read-only audit that assumed every conserved split is
2-chunk). The cap binds only for `share₀ ≥ 0.370`; instrumenting
`conserved_catalog_chunk_factors` over the corpus gives 232 invocations,
of which **68 are below the cap** — splits run 2 to 27 chunks and every
n ≥ 3 split leaves the premium uncapped. Sweep the premium and the cap
together, not the cap alone.

The tomli residual is the direct cost of removing replication: its old
chunk #1 bought at 2256 cum with inflated value and earned mid-budget
roster credit; conserved tails price at ~0.05–0.2 of base and buy at
~9.5K. Its high-budget rows improved (5.3–5.6 up +0.01–0.03), as did
commander's late curve. Recovering tomli's mid-budget tail credit
requires giving tails meaningful mass again, i.e. re-approaching
replication — tension accepted and reported, not tuned away.

Also measured this pass: merging small tool-config families **across**
large families (instead of flushing on each large family) is worse —
the old occasional solo sub-minimum pack is a cheap early buy, the
merged pack schedules later (htmy/tomli). The flush-on-large behavior
is deliberate now; see the comment in `pack_small_tool_config_families`.

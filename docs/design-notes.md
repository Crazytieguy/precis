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
`iterate-divergence` skill. Historical context: per-
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
does not lift the score if the matching NS row is ranked past 3K. Two
load-bearing instances observed during the v0.2 batch:

- **sqlite-vec aFunc[] / aMod[] are at NS tier 2.7 / 2.8** (NS_cum
  3242 / 3380). The Phase-4 `CKey::InitTableRows` recognizer surfaces
  them at walker_cum < 200, but the 3K score doesn't change — those
  rows aren't in `A_3K`. The recognizer pays off at the user-facing
  4K+ budgets, not the 3K headline.

  Implication: any future "lift sqlite-vec's 3K score" work should
  target the actual `A_3K` rows (mostly README / ARCHITECTURE.md / site
  docs / TODO + two "location roster" rows for scalar-fn definitions
  and the init entrypoint). Registration tables are the wrong shape.

- **NS placement decisions are sticky.** NSes are frozen; the walker
  iterates against them. When the metric is misaligned with what
  walker work is feasible at a given budget, the answer is either (a)
  pick walker work that matches the metric's view of the budget, or
  (b) accept the score gap and surface the new content at larger
  budgets. Don't bump `value` to force a batch into a budget tier
  where it doesn't earn `A_B` credit — it just displaces walker
  batches that do.

## Python facade-API method-sig surfacing — simple value lever is too narrow

For py3xui-style facade APIs (`ClientApi` / `AsyncClientApi`),
`PythonKey::MethodSigs` batches currently land at rank ~225 /
cum_tokens ~8500, far outside any reasonable 3K prefix. A pre-commit
sweep over `k ∈ {0.3 … 2.0}` of a `tanh`-bounded arg-richness multiplier
on `method_sigs_value` failed to lift py3xui's Score(3000) while
canary Python fixtures (`pluggy`, `microbootstrap`, `typeguard`,
`htmy`, `tomli`) regressed.

Concretely: a +50% value bump on MethodSigs cannot move a 215-token
batch from rank 225 (cum 8488) to inside rank ~50 (cum < 3000) when
the surrounding 50 batches share comparable value/cost ratios — the
gradient is too narrow.

If revisited, options to consider before another sweep:
- **Drop `concrete_impl_sibling_factor` on the MethodSigs path** when
  the chunk's classes look facade-shaped (≥6 methods with non-self
  args). That 0.6 multiplier is the largest single demotion currently
  applied; py3xui's API directory has no `base.py` but does have a
  parallel async sibling layout that may trigger it.
- **Restructure MethodSigs predecessor** for arg-rich classes — make
  it a top-level sibling of `DeclNames` rather than a descendant, so
  it doesn't have to wait for the DeclNames chunk to schedule first.
  Larger architectural change.

Don't re-attempt the simple multiplier in isolation; the sweep already
demonstrated it's structurally insufficient.

## v0.2-rewrite post-fixture-add batch (May 2026)

After 28 fixtures landed, several patterns surfaced that were
addressable by *generalizing existing logic* rather than adding
language-specific knobs. Worth noting because the wins were broader
than a single fixture but the gradient is shallow:

- **Dense-`.md`-siblings damp** in `walker::markdown` (`sqrt(threshold/N)`
  factor on `HeadingsOutline` when a dir has > 6 `.md` files). Helps
  click / axios / sqlite-vec by getting per-file outline batches out
  of the way of source content. Threshold is calibrated so
  superstruct's `docs/guides` (6 siblings) keeps full weight on its
  per-file outlines.
- **Adaptive README index decay** (`readme_index_decay` switches to
  steeper falloff when total H2 count >= 18). Helps long-tail READMEs
  (debug 18 H2s, axios 31) without regressing short-README fixtures
  (chalk 12, mitt 7 — these all regressed under a blanket steeper
  decay).
- **Root-level vendor-dir classifier** (`is_root_level_vendor_dir_name`)
  applies a 0.2× factor to `deps` / `vendor` / `vendored` / `third_party`
  / `external` / `sig` / `signatures` / `tap-snapshots` / `3rd*` at
  depth 1. Depth-1-only is load-bearing: chalk's `source/vendor/` is
  the project's own implementation; a path-anywhere rule regressed it.
- **Generic dot-dir demotion** at any depth (with `.github/workflows`
  exception) — picks up `site/.vitepress`, `docs/.vitepress`,
  `packages/foo/.changeset`, root `.husky` / `.devcontainer` / etc.
  Replaces a per-name enumeration that was drifting.
- **Python `_test.py` / `test_*.py` and `tests_*` dir patterns** —
  Python test conventions were missing from the test-file classifier
  even though Go and TS/JS were covered.
- **Plaintext `VERSION` / `TODO`** — common extensionless orientation
  files; was missing from the plaintext walker's narrow whitelist.
- **Localized README whitelist** rebuilt as `<lang>-<region>` form
  rather than enumerated region pairs.

What did **not** generalize cleanly and is unlikely to without more
structural work:
- **JS / TS `module.exports` CommonJS** — audiobookshelf's
  `server/Server.js` is CommonJS and the TS walker doesn't recognize
  `class Server` from a `class_declaration` outside an `export`
  context. NS expects this content but the walker can't deliver it.
- **`identity_meta_value` lowered** — regressed enclosed / commander /
  ts-pattern / ky beyond the wins on p-queue / svgo. Reverted.
- **`scripts_value` bumped** — regressed axios (somehow). Reverted.
- **README range-count trigger** for audiobookshelf's H1-only-with-
  H3-splits structure (60+ ranges, 7 H2-equivalents). The proposed
  `effective_range_count >= 35` regressed mcphost / chalk; the
  threshold can't distinguish "lots of medium-depth H3 splits" from
  "kitchen-sink documentation". Audiobookshelf's 0.45-ish ceiling
  remains a structural limit.

## v0.2 Go-walker batch (May 2026)

A second wave of changes focused on the Go walker after the broader
fixture corpus surfaced several Go-shaped patterns the walker couldn't
deliver cleanly. Each generalizes an existing pattern from another
walker rather than adding fixture-specific knobs:

- **`go_entry_factor`** in `walker::go` parallels Rust's
  `entrypoint_boost`. A root-level Go file qualifies for a 1.4×
  multiplier on every per-file batch value when either (a) the stem
  matches the file's `package` clause (`tea.go` in `package tea`,
  `cobra.go` in `package cobra`) or (b) the file holds an exported
  single-spec `type X struct { … }` with ≥ 60 body lines and ≥ 3
  blank-line-separated field groups (cobra's `command.go` is the
  canonical case — package-name doesn't match but the API anchor
  lives there). Depth-1-only is load-bearing: monorepos with many
  internal/sub-packages each have a package-name-matched file at
  depth 2+ and a blanket boost crowds the early budget.
- **`GoKey::PackageDocLede`** — the contiguous `//` comment block
  above a file's `package` clause emits as its own batch (analogous
  to TS's `ModuleDocLede`, C's `HeaderBanner`). Folding it into
  `PackageImports` made the combined batch too big to schedule
  early on API-anchor files. Gated to entry-shaped files + `doc.go`
  so internal subpackage doc ledes (which carry low orientation
  value) don't crowd the schedule.
- **`GoKey::DeclNames` chunking** — re-introduced with a dual gate
  (decls > 30 AND lines > 850). Past attempts split at 24 alone and
  regressed `xxhash` (29 decls); the line-count co-threshold keeps
  moderately-sized files (gin.go 57 × 832, migrate.go 35 × 979)
  unchunked. Chunk size 8 (smaller than the universal 12) because
  Go method-decl name rows render long (`func (c *Cmd) Foo(...)`
  ~22 tokens each).
- **`GoKey::StructFieldGroup`** — when `grouped_type_info` finds an
  oversized struct with multiple blank-line groups, emit per-group
  batches with the parent `Decl` as predecessor; the parent Decl's
  rendered span is trimmed to just the `type X struct {` header
  row + `}` closer row so child group spans don't overlap as
  non-ancestor.

What did **not** generalize:

- **Per-group "leading bump"** for the first 1-3 field groups —
  cobra's leading `Use` field group has a doc comment so big that
  even with the bump the group is too costly to fit early; bubbletea
  regressed because its `View` struct's leading group also has a
  big doc and the bump displaced NS-aligned content from sibling
  files. Reverted.
- **TS class member splitting for TS files (not just JS)** — split
  logic is gated on `is_js_file`; enabling for TS didn't fire the
  split (max class-member count 40 < p-queue's PQueue 41) and a
  bump to 50 risks over-fragmenting other TS classes.
- **App-style JS entrypoint surface fallback** — when an entrypoint
  has zero re-exports AND zero exports, expand surface via all
  requires. Helps dockly (+0.06) and audiobookshelf (+0.02) but
  regressed vaul (-0.23), d2ts (-0.07), semver (-0.15) — vaul has
  local `export function` declarations without re-exports, so
  detecting "app" vs "lib" by no-exports is too coarse. Reverted.
- **Python boost based on `from .X import` re-exports** — regressed
  pluggy / typeguard / htmy / peepdb where the re-exported modules
  weren't NS-relevant anchors.
- **Asset-only-subtree damp** at depth ≥ 3 — regressed beets
  (test/rsrc subtree had NS-relevant per-fixture markers).

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

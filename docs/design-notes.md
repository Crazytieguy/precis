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

**The one exception: an unaffordable seed (2026-07-26).** Under-
utilization at the *first* round isn't under-utilization, it's total
failure — every batch a walker emits is gated, directly or
transitively, on the seed listing, so a seed that doesn't fit leaves
the pool permanently empty and the caller with an empty string. That
is repo-dependent — the threshold is the root listing's own cost, 729
tokens for htop, and 4 of the 93 fixtures returned nothing at budget
200 — and reads as a crash from a script or agent loop.
`Scheduler::schedule_partial_seed` degrades that one round to the
longest affordable prefix of the seed's entry rows and then stops.
Listings only: entry rows are independent, so a prefix of one is a
smaller listing, where a prefix of a line batch is severed source.

**Which entries survive is a value judgment, and name order is the
wrong one.** Alphabetical truncation spends a 100–300 token budget on
dotfiles and README translations and never reaches `src/` — better
than the empty string it replaced, and still close to the least useful
subset on offer. `rank_seed_entries` orders the entries first and cuts
the tail, on three conventions that hold on any repository rather than
on a list of names: hidden entries last (a leading `.` is the filesystem's own
"outside the ordinary view"), directories before files (a directory row
stands for a subtree and is the only row that says how the repo is
organized), and all-caps root documents after other files (the
convention that makes `README`/`LICENSE`/`CONTRIBUTING` recognizable
everywhere is what makes their *names* uninformative — a reader assumes
they are there — while the manifest, entrypoint and build file names do
say something; within the documents, `README.md` outranks
`README.ja.md`). The ranking is a function of the listing alone, never
of the budget, so a smaller budget's surviving set stays a subset of a
larger budget's. Ranking is *only* consulted on this degraded path: a
listing that fits renders every row in name order regardless, and the
render order of a degraded listing is name order too, so no other
scheduling decision anywhere can see it. Measured: rendered output is
byte-identical across all 93 fixtures at every budget on the
1000–9000 grid, and no fixture degrades at 1000 or above.

The remedy this section prescribes above — *split the blocking batch or
lower its rank* — is the right lever when a batch blocks *some* budgets,
and it stays the first thing to reach for. It doesn't apply here: the
seed is the only batch there is at round 0, so lowering its rank changes
nothing, and chunking the root listing in the walker moves scheduling at
every budget and forces a corpus-wide recalibration to fix a failure
that only exists below the root listing's own cost. The scheduler-side
degradation is provably a no-op above that threshold.

The prefix property survives in the form that matters — `output(T₁) ⊆
output(T₂)` for `T₁ < T₂` — and the *batch* prefix property is
untouched above the first round, so slicing a `T_max` run still
reproduces every sub-budget schedule the corpus is measured at. A
partial listing renders a trailing `…` row (`RenderedTree::
listing_partial`), on the same argument as the entry-row marker: a
listing cut short is otherwise byte-identical to a complete one. NS
batches list directories incrementally, so that marker also shows up
in NS simulation — it shifted `exp_t` by 1–2 tokens on 9 fixtures at
the 2026-07-26 freeze, with no `Score(3000)` movement anywhere.

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

The mirror-image bug, found after the first version merged: **a directory
can be ignored by a pattern that lives inside it.** The self-ignoring
directory (`.gitignore` holding a single `*`, the usual idiom for a
scratch, cache or vendor dir) matches nothing from its parent's vantage
point, so the parent listed it — precis's own `.claude/scripts/` was one.
`DirFilter::hides_everything_in` answers "does the filter admit anything
anywhere beneath this", and `list_dir` drops a directory child for which
it says no. It has to be recursive: a directory holding only such
directories is ignored too, which is what `git status --ignored` reports
(`!! outer/`) even though `git check-ignore outer/` — a one-level,
pattern-matching question — calls it visible. **Use git's walk, not
check-ignore, as the oracle for this**; the parity test in
`src/fs_util.rs` uses `git ls-files --others --exclude-standard` for
exactly that reason.

A **genuinely empty** directory is deliberately *not* treated as ignored.
It is real repository structure, and dropping its row would trade one
misleading output for another. That is the one place precis renders
something git cannot represent at all (git doesn't track empty dirs), and
the parity test encodes it as the single expected difference.

Cost is one extra `read_dir` per directory child, early-exiting at the
first surviving entry and memoized on the filter: +10% on a synthetic
repo that is 2049 directories deep-and-wide, ~0% on a real one, exactly 0
when the filter is inert. That memo is why `RenderedTree` holds
`Rc<DirFilter>` rather than building its own — a scheduler cost probe
constructs a tree per call, and an owned filter threw the answers away
every time.

Corpus impact: exactly zero, for both the original restore and this
follow-up — no fixture has a `.git`, so every baseline regenerated
byte-identical.

## Content outside the walk root doesn't belong in it at all (2026-07-26)

precis summarizes *a path*, and it runs on untrusted checkouts whose
output is pasted into chat contexts and agent loops. Until this change
`fs_util::list_dir` classified every non-directory entry as `File`, so a
repository shipping `config.ini -> ~/.config/app/credentials.ini` had its
contents read and rendered verbatim by the plaintext fallback —
reproduced, marker and all. The typed walkers were already right
(`fs::files_with_any_extension` rejects non-following file types); the
rule existed and the fallback simply never went through it.

The fix is deliberately *not* another per-walker check. Containment is
now a property of the listing layer, so every consumer inherits it:

**`DirFilter` always knows its walk root**, canonical form included —
`tests/fixtures` is a symlink, so every fixture walk reaches its root
through one, and comparing an entry's canonicalized target against a
non-canonical root would reject every in-repo link in the corpus. The
literal root still bounds the `.gitignore` ancestor walk; the canonical
one is only for resolving links. `DirFilter::none()` became
`DirFilter::unfiltered(root)` for this reason: a filter with no root can
express no containment, so there is no way to construct one.

**A link surfaces only when it resolves inside the root**, and then with
the kind of what it resolves to. Escaping and dangling links are dropped
— a dangling one can be neither classified nor cleared. In-root links
keep their rows: `CLAUDE.md -> AGENTS.md` and `README -> README.md` are
real structure that several fixtures carry, and hiding them would trade
one misreport for another.

**Listing *through* a link yields nothing.** This is what makes the walk
finite, and it replaces cycle bookkeeping rather than adding it: no
traversal path can contain a link component, so the walk is exactly the
real directory tree and `link -> .` / `link -> ..` are unreachable rather
than merely bounded. Depth caps and visited-sets were both considered;
both are state that a stateless listing call has no good place to keep,
and an ancestor-only check misses mutual `a/x -> b`, `b/y -> a` loops.

The same non-following entry type went into `walker/fs.rs`'s recursive
extension walk and `walker/c.rs`'s source enumeration, which reach files
without going through a listing.

**Not reached, and still open:** walkers that probe a *named* path
directly — `dir.join("Cargo.toml").is_file()`, `package.json`,
`__init__.py`, `mod.rs` — follow links and then read. A checkout shipping
`Cargo.toml -> /etc/passwd` still gets that file parsed and rendered.
Closing it properly wants one contained-read helper adopted across
~8 walker modules; `SourceCache` is not the chokepoint it looks like
(≈10 walkers call `read_to_string` directly).

Corpus impact: `Score(B=cum)` identical at every budget on the 7-point
grid. One rendered row moved — `enclosed`'s
`packages/deploy-cloudflare/.nvmrc` is `.nvmrc -> .nvmrc`, a genuine
upstream ELOOP link, and dropping it stops precis promising a Node
version pin that does not exist.

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

## ⚠️ Score-history break at the 2026-07-28 NS re-freeze v2 (2fbbe8d4)

All 93 North Stars re-authored again (Opus authors, de-anchored
guidelines, blind A/B gate 12/12, mechanical audit; see commit
2fbbe8d4 and ignore/ns-refreeze-2026-07-28/). New zero point: training
mean 0.6074 (old key scored this same tree 0.6412); validation
baseline 0.5554. The v1 corpus is kept at `tests/north-stars-v1/` for
two-column history scoring. Every magnitude recorded below this line
predates the v2 key unless said otherwise. The v2 key demands source
much earlier (median first-source position 397 vs 1147), so
pre-freeze dead-lever verdicts about early-budget orientation vs
source are the most likely to have flipped — re-measure before
trusting any of them.

### Post-refreeze re-sweep curves (2026-07-28, v2 key, zero 0.6074)

Two full-corpus sweep lanes re-measured the settled knobs against the
v2 key. All numbers are Score(3000) means over the 71 training
fixtures; the code comments on these constants point here rather than
restating the shapes.

**Combined-tree grid** (un-ships + concavity 0.38 + listing 1.13
merged together, measured fresh — isolated deltas summed to +0.0094;
the combined state delivers +0.0076 at 3000, interaction cost
−0.0018): B=1000 0.6150→0.6128 · 1442 0.6246→0.6233 · 2080
0.6259→0.6277 · **3000 0.6074→0.6150** · 4327 0.5801→0.5845 · 6240
0.5614→0.5649 · 9000 0.5606→0.5604. The early-budget give-back equals
lane S1's isolated measurement (−0.0023 at 1000), i.e. it comes from
the listing scale alone, not from lane interaction.

**Listing / roster lane** (shipped `LISTING_TIER_SCALE = 1.13` plus
deleting `catalog_roster_mass_factor`; combined +0.0046, 0.6074 →
0.6120):

- Class-wide listing scale: 0.80→0.5928 · 0.90→0.6047 · 0.95→0.6067 ·
  1.03→0.6077 · 1.06→0.6069 · 1.10→0.6089 · 1.11→0.6089 · 1.12→0.6092 ·
  1.13→0.6103 · 1.135→0.6107 · 1.14→0.6105 · 1.15→0.6111 · 1.17→0.6112 ·
  1.20→0.6081 · 1.25→0.6046
- `LIST_SOURCE` tier alone: 0.90→0.6050 · 1.05→0.6076 · 1.10→0.6085 ·
  1.12→0.6090 · 1.13→0.6092 · 1.14→0.6091 · 1.15→0.6091 · 1.16→0.6091 ·
  1.17→0.6090 · 1.18→0.6052 · 1.20→0.6052 · 1.25→0.6016 · 1.30→0.6011 ·
  1.40→0.6002
- Per-tier at ×1.15: root→0.6074 (bit-identical everywhere) ·
  module→0.6086 · source→0.6091 · catch-all→0.6084
- `CATALOG_ROSTER_MASS_NEUTRALIZATION`: 0.0→0.6086 · 0.40→0.6079 ·
  0.60→0.6079 · 0.90→0.6071 · 1.00→0.6071 (monotone; 0 best)
- `ROSTER_MASS_FACTOR_CAP`: 1.28→0.6067 · 1.44→0.6068 · 1.6→0.6074 ·
  1.76→0.6070 · 2.00→0.6068 · 2.40→0.6054
- `ROSTER_MASS_BASELINE`: 8.8→0.6072 · 11.0→0.6074 · 13.75→0.6087
- `TEST_INDEX_LISTING_BOOST`: 1.0 / 1.44 / 1.62 / 1.98 / 2.25 / 4.00 all
  →0.6074 (inert)
- `CHUNKED_NAMES_FIRST_CHUNK_FACTOR`: 0.80→0.6065 · 0.85→0.6051;
  `CHUNKED_NAMES_FALLOFF`: 0.35→0.6056
- Combined: all1.13+cn0.6→0.6108 · all1.14+cn0.6→0.6111 ·
  all1.16+cn0.6→0.6116 · all1.17+cn0.6→0.6117 · **all1.13+cn0→0.6120
  (shipped)** · all1.12+cn0→0.6109 · all1.14+cn0→0.6121 ·
  all1.145+cn0→0.6121 · all1.15+cn0→0.6124 · all1.16+cn0→0.6123 ·
  +`ROSTER_CAP` 1.76→0.6111 · +`ROSTER_BASELINE` 13.75→0.6122 ·
  +`NAMES_FIRST` 0.85→0.6097 · +`CAT_CHILD` 0.025/0.10→0.6120 (inert)
- 1.15/1.16 score higher at 3000 (0.6124/0.6123) but drop budget-1000
  by ~0.007; 1.13 was chosen for holding the rest of the grid.
- DEAD: `ROSTER_MASS_FACTOR_CAP`, `ROSTER_MASS_BASELINE` (mixed, loses
  1000/4327), `TEST_INDEX_LISTING_BOOST` (inert at the primary budget;
  ~25 removable lines behind it, costs −0.0016 at 1442),
  `CATALOG_CHILD_LISTING_SUPPRESSION` (fully inert 0.025–0.10 — gate may
  no longer fire), `CHUNKED_NAMES_FALLOFF`, per-tier `LIST_ROOT`
  (provably rank-invariant). `CHUNKED_NAMES_FIRST_CHUNK_FACTOR` is a
  real early-budget lever but loses at the primary budget.

**Value-mix lane** (shipped `CATALOG_ROSTER_CONCAVITY_EXPONENT`
0.37 → 0.38; +0.0027, 0.6074 → 0.6101, gain concentrated at 2080–3000):

- `mix_signals` fu: 0→0.5998 · 100→0.6041 · 160→0.6044 · 200→0.6045 ·
  240→0.6054 · 260→0.6055 · **280→0.6074** · 290→0.6073 · 300→0.6071 ·
  320→0.6073 · 360→0.6071 · 400→0.6071 · 440→0.6070 · 500→0.6068 ·
  600→0.6060
- `mix_signals` cat: 800→0.6065 · 900→0.6067 · 950→0.6071 ·
  **1000→0.6074** · 1050→0.6068 · 1100→0.6054 · 1200→0.6052 ·
  1400→0.6020
- `mix_signals` ztu: 180→0.6036 · 230→0.6054 · 265→0.6065 ·
  **300→0.6074** · 335→0.6068 · 370→0.6069 · 420→0.6068 · 500→0.6067
- `DEFAULT_CONCAVITY_EXPONENT`: 0.30→0.5977 · 0.32→0.5952 · 0.33→0.5954 ·
  0.34→0.6001 · **0.35→0.6074** · 0.355→0.6074 · 0.36→0.6078 ·
  0.365→0.6058 · 0.37→0.6057 · 0.38→0.5996 · 0.40→0.6017 · 0.42→0.5963
- `CATALOG_ROSTER_CONCAVITY_EXPONENT`: 0.28→0.6021 · 0.31→0.6047 ·
  0.34→0.6069 · 0.355→0.6064 · 0.37→0.6074 · 0.372→0.6080 ·
  0.375→0.6102 · 0.378→0.6101 · **0.38→0.6101 (shipped)** · 0.385→0.6095 ·
  0.39→0.6093 · 0.395→0.6095 · 0.40→0.6094 · 0.41→0.6093 · 0.42→0.6096 ·
  0.43→0.6091 · 0.46→0.6068 · 0.50→0.6050
- `depth_factor` slope: 0.20→0.5945 · 0.25→0.5979 · 0.30→0.6044 ·
  0.325→0.6058 · **0.35→0.6074** · 0.375→0.6053 · 0.40→0.6050 ·
  0.45→0.6052 · 0.50→0.6046 · 0.60→0.6006
- Combined on cexp=0.38: alone→**0.6101** · +dexp0.36→0.6091 ·
  +dexp0.34→0.6004 · +fu320→0.6100 · +fu240→0.6085 · +ztu335→0.6088 ·
  +ztu265→0.6091 · +cat950→0.6091 · +cat1050→0.6095 ·
  +dslope0.375→0.6085 · +dslope0.325→0.6102 (ties at 3000, loses at
  1000/4327/9000 — rejected)
- DEAD: all three `mix_signals` axes, `DEFAULT_CONCAVITY_EXPONENT`
  (0.36's +0.0004 is a knife edge — 0.355/0.365 sit at-or-below base),
  `depth_factor` slope, and every combination stacked on cexp=0.38.

Caveat carried from both lanes: nearly every large per-fixture mover
sits within ~40 tokens of the 3000 cliff, so individual fixture
magnitudes are not trustworthy. The ship decisions rest on the plateau
shapes above and on whole-NS-row bucket changes.

## Dominant source file (2026-07-29): 0.6151 → 0.6179

The largest v2-key loss class was "unscheduled": one implementation
file per repo that the walker can emit but never schedules at 10K
(tomli `_parser.py` 0.515 of |A_3K|, sqlite-vec.c 0.440, act
`cmd/root.go` 0.505, xlstm `model.py` 0.478). The NS wants *more of
the same file*; the walker spends the marginal token on breadth.

Shipped mechanism, two pieces:

- `WalkCtx::dominant_source_file` — walk-time detection of the file a
  repository is *about*: the largest essential source file, when it
  holds ≥ 20% of the tree's essential source bytes. Guards that all
  earned their place on the corpus probe: non-essential subtrees plus a
  `dist`/`vendor`/`spec`/`testdata`/`libs` exclusion list (both numerator
  and denominator); a 400 KB ceiling and a 200 bytes/line mean ceiling
  (minified and generated bundles — healthchecks `zxcvbn.min.js`,
  monaco `typescriptServices.js`, rqlite `testdata/chinook/db.go`,
  rough-viz `dist/roughviz.es.js` all won the raw mass race); and a
  primary-language gate (act's spine is Go, but a vendored
  `pkg/runner/hashfiles/index.js` is its biggest single file). Fires on
  39/93 fixtures.
- `Scheduler::dominant_file_boost` — a ×1.35 ratio premium on that
  file's *surface* batches, **gated on the file already having been
  entered** on its own merits.

  "Surface" is `WalkerKey::is_dominant_file_surface`, a positive
  per-walker opt-in (declaration/name rosters and catalogs,
  public-surface item heads, imports-level surface). The lane originally
  spelled it `!is_depth_follow_up()`, which is the wrong instrument:
  `is_depth_follow_up` is a *breadth-pressure* opt-in, so a walker that
  never needed pressure never listed anything and the premium fell
  through to everything it emitted — Lua and Prisma have no override at
  all (middleclass.lua is ~85% of its repo's source mass, so its whole
  depth train was boosted), Go excluded only `DeclBody` (doc ledes/bodies,
  `DeclDoc`, struct field groups all boosted), C excluded only
  `DeclDoc`/`DeclBody` (aggregate member groups boosted). Two taxonomies
  with different purposes must not share one predicate.

Mass-share was explicitly not among the four proxies in the dead "Go
spine centrality" entry, and the entered-gate is what makes it work.
The gate is the whole result: every variant that let the premium pull
the spine file *forward* traded orientation for depth and lost.

- Boost magnitude, entered-gate, `!is_depth_follow_up` surface (the
  lane's original sweep, isolated on 0.6151): 1.15→0.6155 · 1.30→0.6170 ·
  1.40→0.6173 · 1.45→0.6179 · 1.50→0.6179 · 1.55→0.6179 · 1.60→0.6160
  (vaul −0.138 crosses a cliff)
- Re-swept on the true surface predicate over the merged integration
  tree, since the boosted set shrank — grid means, `B=1000 … 9000`:
  1.35 → 0.6134/0.6239/0.6307/**0.62094**/0.5916/0.5665/0.5643 ·
  1.45 → 0.6146/0.6231/0.6306/**0.62087**/0.5916/0.5656/0.5636 ·
  1.55 → 0.6147/0.6209/0.6292/**0.62123**/0.5918/0.5656/0.5630.
  Flat at the primary within 0.0004; **1.35 shipped** — it wins or ties
  six of seven budgets and 1.55 pays for its primary sliver at 1442
  (−0.0030) and 2080 (−0.0015). The lever is a plateau, not a peak;
  don't re-tune it on sub-0.001 primary moves.
- Including depth follow-ups in the premium: 1.50→0.6169 (−0.0010).
  Rosters are the under-bought class; the file's own docs and bodies
  already rank once its train is open.
- **DEAD — budget-window gate** (premium suppressed until
  `consumed ≥ 0.25 × budget`, no entered-gate): 0.20→0.6163 ·
  0.25→0.6170 · 0.28→0.6151 (inert). Scores at 3000 but is an artifact:
  0.25 × the 10K schedule budget lands the premium in the 2500–3000
  slice of the scored prefix, and the grid gives it all back
  (4327 −0.0029, 6240 −0.0015, 9000 −0.0008). It also behaves
  differently at a real user budget than at the measured one. Do not
  re-try budget-fraction gates on this lever.
- **DEAD — ungated premium** (no window, no entered-gate): surface-only
  1.30→0.6144, all batches 1.30→0.6137. Big targeted wins
  (express +0.117, tomli +0.061) fully cancelled by front-loading
  collapses (cmdk −0.192, neco −0.107 where the boosted `neco.c` roster
  displaced the `neco.h` public API, anyhow −0.036, pluggy −0.031).
- **DEAD — per-file concavity relief** (rank at `cost^(k−0.10)` inside
  the dominant file, so the premium grows with batch size): 0.6071
  ungated, 0.6152 with the 0.25 window. Strictly worse than the flat
  premium at every gate tried.

Shipped grid (before → after): B=1000 0.6132→0.6150 · 1442
0.6217→0.6204 · 2080 0.6277→0.6266 · **3000 0.6151→0.6179** · 4327
0.5851→0.5863 · 6240 0.5649→0.5635 · 9000 0.5606→0.5585. Net-neutral
off-primary. Eight movers at 3000, all positive, no fixture regresses:
log +0.050, bubbletea +0.046, swarm +0.038, cobra +0.025, xxhash
+0.013, middleclass +0.012, cmdk +0.009, debug +0.006. swarm and cobra
buy their new content at cum 2966–2983 — cliff-adjacent, discount them;
log and bubbletea re-order at 1094–2020 and are the robust evidence.

The detector's original largest-file tie-break mis-targeted anyhow
`src/ensure.rs`, thiserror `impl/src/expand.rs`, and express
`lib/response.js`. A public-surface-density discriminator now retargets
among files that independently clear the 20% mass floor. The count is
syntax-backed: real public top-level Rust items and `macro_export`
definitions; real JS/TS exports plus method assignments on a receiver
proven to flow to `module.exports`. Comments, raw strings, template
literals, and internal object assignments do not count. It retargets
thiserror and express; anyhow remains on `src/ensure.rs` because its
preferred `src/lib.rs` does not independently clear the mass floor. The
largest file remains the default; a challenger must be at least 20%
denser and must match the largest file's semantic type-machinery class —
`.d.*` files and ordinary TS modules whose exports are all type-only stay
separate from runtime implementations. The guards came from the measured variants: unconditionally
choosing the densest candidate moved express +0.088 but log −0.050 at
3000; adding the 20% margin removed log, and the declaration-stub guard
removed commander −0.081 at 2080. Final training grid versus the wave-1
baseline: 0.6134/0.6239/0.6307/**0.6221**/0.5911/0.5662/0.5638 — only
the primary budget moves (+0.0012), through express. Its last walker row
lands at 2980, so treat the fixture magnitude as cliff-adjacent even
though the newly credited application roster itself ends at 2917.

### Declarative Python data-model surfaces (2026-07-29)

A source-mass detector cannot identify a compact schema/settings module
inside a larger Python package. The Python walker therefore treats a file
with at least four public top-level classes, at least eight class-field
rows, and at most two methods as a declarative data-model surface, and
lifts only its declaration roster by ×1.30. The field and method counts
reuse the same syntax-backed collectors that emit `ClassBody` and
`MethodSigs`; filenames and package names are not inputs. This keeps the
mechanism on a cheap location map rather than promoting every field body.
The measured factor neighborhood was 1.20→0.6242 and
1.30/1.35/1.50→0.6249 at Score(3000), from a 0.6224 baseline; 1.30 is the
lowest point on the plateau.

### v2-key un-ship generalization losses (2026-07-28, corpus-invisible)

The same lanes deleted mechanisms whose corpus contribution was flat or
negative. Two classes of loss don't show up in Score(3000) at all and
are recorded here so they aren't rediscovered as regressions:

- **Wide source catalogs outside conventional source-root names.**
  Removing the catalog roster-mass neutralization net-demotes wide
  catalogs that live somewhere other than a recognized source root —
  a repository-root `include/` or `components/`, for instance. No
  training fixture has that layout, so the corpus can't see it. Queue a
  non-corpus fixture with a root-level wide catalog before treating the
  removal as fully validated.
- **Content classes with no remaining reachable path.** The un-shipped
  mechanisms were the only way to reach: TS interface-member JSDoc;
  YAML API-spec *body* content (openapi / swagger / schema /
  reference.yaml beyond `TopLevelKeys`); and TS test / describe / bench
  label rosters. These are out-of-corpus-axis losses, not walker
  regressions — cover them via the OOC protocol's pinned repos rather
  than by re-adding the mechanisms on corpus evidence.

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
PROSE_MASS_WINDOW_FRACTION 0.25 (both PROSE_MASS_* since deleted —
see the README-ordering entry), TRAIN_PRESSURE_K 0.15,
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
  train completion while the head still buys early. (Tails were also
  exempt from the `deferred_mass_prose` pass, which was un-shipped
  2026-07-29 as inert — see the README-ordering entry below.)
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

## TOML manifest recall (2026-07-26): 0.6351 → 0.6360

Closed the widest pool-absent class by fixture count (876 NS tokens ≤3K
over 16 training fixtures in the recall census): a Python identity table
contributed only six whitelisted single-line scalars, and `Config` took
only an enumerated list of table names, so author rosters, project URLs,
keywords, `[lib]`, `[lints.*]` and `[patch.*]` were unreachable at any
budget. Shipped as `TomlKey::PackageMetadata` (predecessor `Identity`,
priced at the manifest-config tier), plus an inverted `is_config_section`
— every table of a manifest that no other batch claims — and a
structural `is_manifest_toml` (declares `[package]`/`[workspace]`, or is
a Python manifest) so `sqlite-dist.toml`-style dialects are recognized.
PEP 735 `[dependency-groups]` moved to `DevelopmentDependencies`, where
its Cargo analogue already lives. Movers: rich +0.064, peepdb +0.021,
click +0.020, py3xui +0.017, beets +0.013 vs xlstm −0.050, tomli −0.037,
requests −0.018. Positive at every budget measured (2080 +0.0022, 4327
+0.0024, 6000 +0.0028, 9000 +0.0014).

- **Widening an existing batch to close a recall hole costs that batch
  its slot — measured twice in one lane.** Taking the whole `[project]`
  table into `Identity` (the obvious fix, and a net code *deletion*)
  measured **−0.0010**: rich's Identity went 81 → 132 tokens and fell
  from cum 1382 to 5748, xlstm 1358 → 3523, microbootstrap 2456 → 5692.
  An 18% ratio drop is enough to lose an early slot outright, and the
  fixtures that lose it are the ones whose NS ranks manifest identity at
  1.x. Folding the same rows into the existing `Config` batch instead
  measured **−0.0020** (peepdb −0.052, typeguard −0.040) for the same
  reason. Only the *additive* form — new content in its own batch, no
  existing batch's cost changed — paid. Same failure visible inside the
  shipped version: otree's `Config` grew 36 → 123 tokens with
  `[lints.clippy]` and fell from 2021 to 5842, so the 107 NS tokens that
  motivated the row still don't land. This is not a split (nothing
  already purchasable was cut) — it is the rule that recall must arrive
  as a new batch, not as a bigger one.
- **Trove classifiers and archive globs earn no place.** `classifiers`,
  `packages`, `include`, `exclude` are excluded from `PackageMetadata`:
  the classifier list restates `license` / `requires-python` /
  `description` in a fixed registry vocabulary, and it is the single
  largest key in most PEP 621 manifests. Including them measured
  **+0.0001** vs **+0.0008** for excluding them (xlstm −0.050 either
  way, but rich +0.064 and beets +0.013 only appear once the classifier
  wall is gone). Only py3xui's NS ranks classifiers.
- **The metadata value is on a plateau.** `config_value` × {0.7, 1.0,
  1.3, 1.8} → 0.6354 / 0.6359 / 0.6359 / 0.6359. Shipped at 1.0 (no
  constant). 0.7 loses rich/beets/py3xui/chronos without recovering
  xlstm.
- **Cargo `[package]` must stay whole.** Applying the packaging-mechanics
  filter to it as well (dropping `exclude`, `include`) cost log −0.007
  on its own — those rows are NS-wanted. The lede/metadata split is a
  Python-manifest rule because only PEP 621 tables carry four times
  their lede in metadata.
- Residual losses are cliff artifacts, not signal: tomli's slack at 3000
  is 4 tokens with a 464-token names surface ending at cum 2947, so any
  58-token insertion anywhere earlier costs it −0.037.

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

Instrument note: `DeclNamesChunk` has 0 scheduled rows at any budget
≤10K across all 71 training fixtures — true, and re-verified 2026-07-26.
**The inference that the splitter is therefore removable dead weight is
false** (corrected 2026-07-27; see below).

## Python DeclNames splitter is live, not dead (2026-07-27): flat

Correction to the instrument note above. Both halves of the 2026-07-26
evidence are individually true and the "dead weight" conclusion does not
follow from them.

- **Instrument (probe on `decl_names_chunk_ranges`, 71 training
  fixtures).** 7 Python files split into ≥2 chunks: htmy `html.py`
  (114 decls / 1626 tok → 4), linkding `settings/base.py` (61 / 1292 →
  3), beets `util/__init__.py` (57 / 937 → 2), sqlite-vec
  `test-loadable.py` (79 / 1135 → 2), requests `utils.py` (50 / 972 →
  2), tomli `_parser.py` (43 / 906 → 2), click `_compat.py` (39 / 809 →
  2). 20 more files enter the chunker and fold back to one range.
- **Why "0 scheduled `DeclNamesChunk` rows" does not mean inert.** Chunk
  0 ships under the plain `DeclNames` key; only chunks ≥1 carry the
  `DeclNamesChunk` key. So the measurement says the *tail* never
  delivers ≤10K — the split's live effect is on the head, which reaches
  the frontier as a cheaper slice repriced by
  `conserved_catalog_chunk_factors`.
- **Measured.** Unifying every roster regardless of size
  (`threshold = usize::MAX`) moves 3K 0.6412 → 0.6405, tomli −0.053,
  no other training fixture's report changes. Reproduces the earlier
  −0.0008 and settles it: do not delete the splitter.
- **What was genuinely redundant, and is now deleted.** The
  `DECL_NAMES_SPLIT_THRESHOLD_TOKENS = 400` early exit.
  `budget_chunk_ranges` cuts only when a range's cost reaches its
  `target` (450), so every roster the early exit short-circuited already
  came back as the single range `0..len`. Threshold < target ⇒ pure
  redundancy — which is also why the earlier "fix the inverted
  threshold" attempt measured exactly flat: any value ≤ ~675 is a no-op.
  Removal is byte-identical across all 71 divergence reports, all
  schedule TOMLs and all rendered snapshots. The chunk target is the
  one live knob.
- **Standing hazard, measured but not shipped.** `names_gate` is the
  *last* chunk, so in a split file `MethodSigs` and every per-decl batch
  gate on a batch that never schedules ≤10K — an ordering guarantee
  nothing can satisfy. Gating on chunk 0 instead
  (`names_keys.first()`) is non-negative at every grid budget —
  1000/1442/2080 identical, 3K +0.0001, 4327 +0.0001, 6240 +0.0003,
  9000 +0.0002 — and only tomli's report changes (+0.004 @3K). Left
  unshipped: it weakens "the whole roster precedes per-decl detail" to
  "the head does", on a single-fixture move inside the noise band, and
  it also invalidates the tiny-tail fold's stated rationale (a crumb
  tail would no longer gate anything). Decide the semantics first, then
  re-measure.

## C internal-linkage recall (2026-07-26): flat @3K, +0.0006 @6.2K

Closed the corpus's #2 pool-absent class: `classify_decl` returned
`None` for every file-scope `static` in a `.c` file, so **no
internal-linkage decl, body, or doc was reachable at any budget in any
C fixture** (1837 NS tokens inside 3K, 9860 overall, 8 fixtures). This
also subsumes the standing "C option-table / argument-parsing roster"
queue item — those `strcmp` chains and `take_arg[]`/`long_opts[]`
tables were missing because they sit inside `static` functions, not
because they are tables.

Shipped as `DeclLinkage` + `admits_internal_decls` +
`CProjectScan::builds_a_program`. Grid means (71 training fixtures):
`0.6266 / 0.6226 / 0.6270 / 0.6351 / 0.5966→0.5969 / 0.5653→0.5659 /
0.5457→0.5454` at 1000/1442/2080/3000/4327/6240/9000. Movers: krep
+0.0266 @4.3K, +0.0437 @6.2K, −0.0218 @9K; chibicc +0.0017 @9K. The
3000-budget rendered snapshots are byte-identical corpus-wide.

Load-bearing findings:

- **The discriminator is library-vs-program and it is project-wide,
  not per-file.** Every per-file proxy fails: htop `CommandLine.c` and
  krep `krep.c` both have a same-stem sibling header, so "no own
  header contract" excludes exactly the fixtures that pay. A project
  is a library when it declares an installed header set
  (`include_HEADERS`, jq) or when nothing outside its test and example
  trees defines an unconditional `main` (neco, sds, tinyusb,
  sqlite-vec). Ungated, those libraries regress: **neco −0.0405 @9K,
  jq −0.0090, sqlite-vec −0.0067** — their `static` implementation
  displaces the header API their NSes anchor on. Gated, every one of
  those deltas goes to exactly zero and both gains survive. The
  `examples/`-dir exclusion is load-bearing on its own: without it
  sqlite-vec's `examples/simple-c/demo.c` main makes an extension
  library look like a program.
- **A restricted value axis is strictly worse here, unlike the Rust
  private-method case.** Swept ×0.5 and ×0.75 on the whole internal
  train (roster + decl + doc + body): at 0.5 nothing arrives at any
  budget and the corpus is byte-identical to the ungated baseline; at
  0.75 krep keeps +0.0437 @6.2K but loses +0.0266 @4.3K; at 1.0 both
  land. Internal decls are now priced by kind exactly like external
  ones — the linkage split decides membership and roster grouping
  only, and the damp constant was removed.
- **Splitting the internal roster finer is unstable.** The internal
  names surface chunks at `C_DECL_NAMES_CHUNK_SIZE` (24) like the
  external one. Dropping to 8 does move the headline (krep +0.0100
  @3K, mean +0.0002) but costs krep −0.0406 @1442; at 12 it is krep
  −0.1174 @1442. Summed over the grid both are worse than 24. The
  small early chunks queue-jump exactly as the struct-field-crumb and
  early-budget-ratio-wall entries predict.
- **The external and internal groups are two independent chunk
  series under one `DeclNames` key** — separate `index_in_group` /
  `group_chunk_count`, so admitting an internal roster never reprices
  the file's public one.

Measured dead in the same lane (specifics block retries):

- **Entry-point names-surface promotion** (extend the dominant-binary
  header-tier promotion in `names_surface_catastrophic_factor` to any
  project's sole `main`-defining `.c`): chibicc −0.1058 @1442,
  −0.0013 @3K, −0.0171 @4.3K. Same failure mode the C-cluster ledger
  records for extending `roster_mass_factor` to the dominant file —
  earlier roster arrival displaces NS-wanted README orientation.
- **`secondary_root_pair_factor` tightening** (require the eponymous
  `<repo>.c` to exist, not just `<repo>.h`, before damping depth-1
  siblings — chibicc's `chibicc.h` is a shared *internal* header over
  main/parse/codegen, not half of a primary pair): chibicc −0.0961
  @1442, −0.0524 @2080, −0.1282 @4.3K, −0.1442 @6.2K. The damp is
  load-bearing on multi-module flat programs regardless of its stated
  rationale; do not "fix" it without a replacement damp.

Still out of reach after this lane: the ≤3K half of the class. chibicc
ranks main.c's flag dispatch as *individual `strcmp` lines* across six
NS rows ≤1900, and krep ranks six krep.c location rosters at 236–1520;
the internal roster lands at 3942 (krep) or past 10K (chibicc, whose
main.c also carries the `secondary_root_pair_factor` 0.5 damp). This is
the early-budget ratio wall again — the recall is now in the pool, so
it is a pricing problem, not an absence problem.

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

## Unparsed-language fallback (2026-07-26): the corpus cannot see this axis

`Score(3000)` is computed over 71 fixtures that are Rust / Python /
JS-TS / Go / C / Lua / Markdown only. **No fixture is written in any
of the ~28 file types v0.1 rendered and v0.2 did not** (Java, C++,
Ruby, PHP, Swift, Kotlin, C#, Scala, Elixir, Haskell, Zig, Dart, Vue,
Svelte, CSS, reST, plain text …), so a total absence of content for
those languages scored exactly the same as full coverage. The
`ignore/session-2026-07-26/v01-vs-v02-audit.md` F2 finding measured
28 of 30 common extensions rendering as a bare filename.

Consequences for anyone working near this:

- **A flat corpus mean is not evidence of no effect here.** Judge
  fallback changes on out-of-corpus repositories at budget 3000; the
  corpus number is a *guardrail only*. The lane that shipped this
  used gson / guava / JSON-java (Java), fmt (C++), devise (Ruby),
  laravel (PHP), Alamofire (Swift), vitepress (Vue), pico (CSS).
- **Pure indentation-zero extraction is not enough on its own.**
  Ruby (`module Foo`), C++ (`namespace`), Kotlin (`object`) put one
  wrapper line at column zero and everything real below it — a
  column-zero-only surface renders the *same line* for every file in
  the project. The shipped rule descends indentation levels until the
  roster is non-trivial (`SOURCE_TEXT_MIN_DECLS` /
  `SOURCE_TEXT_MAX_INDENT_LEVELS`).
- **Fully-nested markup has no surface at all.** XML/POM/XSD and HTML
  yield `<?xml …>` / `<!DOCTYPE html>` / the root element and nothing
  else, at ~27–120 tokens apiece. They are excluded on purpose; they
  need a nesting-aware walker. Same for template dialects.
- **Measured-dead in this lane: removing the depth damp for fallback
  batches.** With `path_depth_factor` forced to 1.0, gson's Java
  surfaces do schedule at 3K — but the ones that win are the
  *cheapest* files (`package-info.java`, one-method interfaces, and
  `test-shrinker/` fixtures), not `Gson.java` / `GsonBuilder.java`.
  Depth is not the lever for the deep-package problem.
- **Open: multi-module Java/JVM at 3K.** After the source-root
  generalization below, a Maven reactor renders the full class-name
  inventory of each module package, but no declaration surfaces —
  the budget legitimately goes to the repo map plus README. Guava at
  3000 is 197 listing batches out of ~200. That is the F3
  listing-dominance problem, not a recall problem.
- **Pricing discipline that held the corpus flat.** Three extension
  tiers, not one: languages (`SOURCE_TEXT_LANGUAGE_EXTENSIONS`, also
  the source-inventory signal) → contract formats (proto/graphql/tf/
  gradle) → flat text (prose, config, shell, build glue, and
  stylesheets). Putting stylesheets or shell in the language tier
  costs real score: stylesheet dirs promoted to source inventories
  measured dockly −0.078, and stylesheets at language pricing
  measured linkding −0.044 (nine 10-token selector slices displacing
  a package's Python decl surfaces).

## Source roots are module-relative, not root-relative (2026-07-26)

`has_root_adjacent_source_ancestor` used to require the `src`/`lib`
dir to be a **direct child of the repo root**, so in any multi-module
repo (Maven/Gradle reactors, Cargo workspaces, npm monorepos) no
directory under `<module>/src/` was ever a source inventory. Now a
`src`/`lib`/`source`/`sources` dir also counts when its parent holds
a package manifest (`is_package_root_dir` / `PACKAGE_MANIFEST_FILES`).
Corpus effect is real and two-sided: vite +0.167, beszel +0.108, sps
+0.101 against monaco-editor −0.064, linkwarden −0.054, mdbook
−0.015 (net +0.0031 at 3000). The losers are repos whose *peripheral*
sub-packages (`monaco-lsp-client/src/adapters`, `webpack-plugin/src`)
now surface early; the winners are repos whose primary package lives
one level down. If this is ever retuned, the discriminator wanted is
primary-vs-peripheral module, not the root-adjacency test it replaced.

### It sold the early budgets, and why (2026-07-26 follow-up)

Ungated, the promotion bought 3000–6240 and **sold 1000–2080**. The
whole effect is this commit — the unparsed-language fallback is
early-budget-neutral to four decimals. Corpus means over the 71
training fixtures, at every grid budget:

| variant | 1000 | 1442 | 2080 | **3000** | 4327 | 6240 | 9000 |
|---|---|---|---|---|---|---|---|
| promotion off (pre-commit) | 0.6266 | 0.6226 | 0.6270 | 0.6351 | 0.5963 | 0.5650 | 0.5428 |
| ungated (as merged) | 0.6249 | 0.6170 | 0.6245 | **0.6377** | 0.5986 | 0.5680 | 0.5442 |
| ≥10 entries | 0.6255 | 0.6200 | 0.6259 | 0.6371 | 0.5982 | 0.5677 | 0.5437 |
| ≥16 entries | 0.6255 | 0.6206 | 0.6260 | 0.6368 | 0.5986 | 0.5674 | 0.5434 |
| **≥20 entries (shipped)** | 0.6255 | 0.6206 | 0.6275 | **0.6378** | 0.5994 | 0.5673 | 0.5436 |
| ≥25 entries | 0.6266 | 0.6227 | 0.6271 | 0.6361 | 0.5972 | 0.5663 | 0.5436 |
| ≥35 entries | 0.6266 | 0.6226 | 0.6270 | 0.6351 | 0.5963 | 0.5656 | 0.5428 |
| distance ≤1 below src | 0.6254 | 0.6188 | 0.6274 | 0.6348 | 0.5954 | 0.5647 | 0.5420 |
| distance ≤2 below src | 0.6255 | 0.6174 | 0.6257 | 0.6358 | 0.5966 | 0.5667 | 0.5436 |
| module depth un-pinned | 0.6254 | 0.6169 | 0.6245 | 0.6378 | 0.5997 | 0.5691 | 0.5451 |

**Cause.** Not one large listing — *many small* ones. A monorepo has
one source root per package, so the promotion fires on every directory
beneath every package at once: 35 listings before the first line of
code in `enclosed` (six co-equal packages), ~570 tokens of
`internal/site/src/components/**` in `beszel` (an embedded React app
with its own `package.json`). Each is 3–60 tokens, which is precisely
why they win the `value/cost^k` race, and precisely why they are the
wrong first purchase: at 1000–2000 there is no budget left for the
files they name. `enclosed` −0.234 and `beszel` −0.196 at 1442 are
essentially the entire −0.0057; no other fixture moves more than
0.005.

**What did *not* work, and what it rules out.**

- **The depth pin is not the driver.** Routing module-relative
  inventories to the clamped `inventory_depth_factor` instead of the
  depth-1 pin is inert (−0.0001 at 1442). At the shipped ≥20 gate it
  is *exactly* inert — identical grid to six decimals. So the
  promotion's cost lives in the tier + `roster_mass_factor`, not in
  the depth pin. Do not re-probe the pin.
- **Distance below the source root is the wrong gate.** Both ≤1 and
  ≤2 are dominated — worse early *and* worse at 3000 than the size
  gate. The deep listings that cost early are the same ones that pay
  at 3000, so cutting by depth cuts both.

**The trade is real but small, and the size gate buys most of it for
free.** ≥20 is ≥ the ungated tree at 1000/1442/2080/3000/4327 and
−0.0007/−0.0006 at 6240/9000 — i.e. it recovers two thirds of the
early loss while keeping the 3000 headline whole. The residual
(−0.0011 at 1000, −0.0020 at 1442 versus not promoting at all) is
irreducible on these levers: buying it out needs ≥25, which zeroes
the early cost and is Pareto-≥ the pre-commit tree at *every* budget,
but gives back 0.0017 of the 3000 gain. Shipped at 20 because
`Score(3000)` is the stated priority; **≥25 is the one-constant
change if early budgets are ever weighted higher.**

Out-of-corpus behaviour is insensitive to the constant anywhere in
[0, 25] — Java/Ruby/PHP/Swift content-line counts are identical, and
gson is slightly *better* at 25 (127 vs 113 content lines, the
suppressed package listings freeing budget). So the choice is a pure
corpus-policy call, not a capability one.

Also fixed alongside: `is_source_dir` and the non-essential directory
classifier were **case-sensitive**, so `Source/`, `Sources/`,
`Tests/`, `Scripts/` — the spelling used across Swift, C#,
Objective-C and Java trees — matched nothing. Alamofire's `Tests/`
outranked its `Source/` as a result. Corpus-flat (fixtures are
lowercase), out-of-corpus decisive.

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

## GitHub workflow recall + the YAML reference-slice un-ship (2026-07-26)

Baseline 0.6389 / 71 fixtures. Two questions, both answered by
measurement rather than by the audit's estimate.

**Superseded 2026-07-28 — see "Un-ship sweep against the v2 NS key"
below. The YAML defence recorded here was measured on the pre-refreeze
key and reverses under v2 (+0.0016, sqlite-vec +0.117).**

**YAML reference deep-leaf slices: DEFENDED, do not un-ship.** The
2026-07-26 un-ship audit rated the mechanism at +0.0002 and recommended
keeping only the `TopLevelKeys` roster. Measured removal (roster kept,
slices + the five bounded-extraction sub-rules deleted, −409 lines)
costs **−0.0009 mean** with sqlite-vec 0.548 → 0.481 (coverage
0.420 → 0.332, one NS row reached → missing). Curve: 0.0000 / 0.0000 /
−0.0003 / **−0.0009** / −0.0006 / +0.0001 / 0.0000. It is worth ~5× the
audit's estimate on the current frontier. Two facts that also blunt the
audit's generalization worry: sqlite-vec is the *only* fixture with a
root reference-map-named YAML, so nothing else can move either way; and
`conserved_reference_slice_factors` conserves one file-level value
across all slices, so a huge `openapi.yaml` cannot emit "a long train of
2.75×-priced slices" — per-slice value falls as the count rises and the
train loses the `value/cost^k` race by construction. The 2.75 factor and
the five caps remain un-swept, which is the real open item.

**Non-elected GitHub workflows (recall-census class 4): the recall
hypothesis is dead; the byte gate was the actual hole.**

- *Elect a second workflow / rank by informativeness* — implemented as a
  fallback election when no filename ranks: elect the non-delivery
  workflow with the most `run:` steps (bot/scanner/template workflows
  delegate to `uses:` and carry nothing project-specific). The
  discriminator is *accurate* — it picks exactly the NS-wanted file on
  all three fixtures where name-ranking currently elects nobody
  (act `checks.yml`, rich `pythonpackage.yml`, enclosed
  `ci-app-server.yaml`) — and **exactly 0.0000 at every budget**, zero
  fixtures moved, zero schedule-snapshot diffs. The newly admitted
  content never wins purchase at `peripheral_ci_value`. ~55 LOC, not
  shipped.
- *Blanket raise of `WORKFLOW_HEAD_BYTE_GATE` 3000 → 12000*: **−0.0083**
  (migrate −0.154, anyhow −0.135, log −0.105, toasty −0.080, tomli
  −0.060, thiserror −0.030, pluggy −0.024) — every mover a full-`ci_value`
  workflow in a ≤2-file dir. Reproduces the recorded flooding result.
- *Demote a truncated head to the peripheral tier* ("the full CI tier is
  for workflows we can render whole"): −0.0005 at 3000 (peepdb −0.046,
  d2ts +0.010) but **+0.0026 / +0.0017 / +0.0028 at 1000 / 1442 / 2080**.
  Identical with or without the gate raise and with or without the
  fallback election, i.e. it is the *only* live knob of the three. Not
  shipped: peepdb's NS row 2.7 wants `test.yml` lines 1–44 — a truncated
  head — at tier 2, so this is a rescale against a bimodal target, the
  shape the `ci_value` de-saturation entry above already records as
  failing. A truncation-*fraction* threshold cannot rescue it: peepdb
  (116 lines) and migrate (111 lines) sit on the same side of every cut.

Shipped instead: the head byte gate is now **per tier**. It stays 3000
for `Workflow` (there it is load-bearing as a flooding guard, per the
−0.0083 above) and becomes `WORKFLOW_HEAD_LINE_CAP * 200` for
`WorkflowPeripheral`, matching `TOOLING_HEAD_BYTE_GATE`'s convention.
Corpus-exactly-neutral (0.0000 at all 7 budgets) and it closes a real
invisibility cliff: **18 of 71 training fixtures** currently render
nothing but the filename for their elected CI workflow at *every*
budget, because a pre-flight read guard was stricter than the 60-line
content bound it protects. Same category as the `431650dd` over-cap
fixes — corpus-neutral by construction, motivated by fixtures being a
sample.

Still uncovered and NOT addressed: NS rows wanting workflow content
*past* line 60 (vite `ci.yml` 116–131, express `ci.yml` line 80, tomli's
scattered job-name lines through line 145). That is a head-shape
question (job-name roster vs raw prefix), and the pre-refreeze
"workflow structural summaries: flat to negative" result applies to it.

## Directory listings ranked by *ascending* entry count (2026-07-26)

Round-2 usability probes found the biggest directory in a repository is
the last thing precis ever shows: at budget 9000 htop expanded `m4/`
(1 entry) and never `linux/` (36 files, the repo's largest subsystem).
Mechanism: `dir_listing_value` is size-invariant while a listing's cost
is linear in entries, so `value/cost^k` orders rosters smallest-first —
and it gets monotonically worse with budget. `roster_mass_factor` is the
existing cure, and it *was* live on the fs path, but only for the
source-**inventory** tier, whose gate is
`non_essential < 1.0 || under_root_source_ancestor`. A normal
root-adjacent source directory satisfies neither, so the one class of
directory the fix was written for never received it. (This reconciles
the two earlier claims in the ledger: the blanket cap raise really is
dead, *and* the fs catch-all really had no roster mass.)

Shipped: any directory the structural catalog test recognizes
(`is_source_inventory_dir`, ≥3 source files) is neutralized, excluding
only named source roots and module dirs — those remain ledger-dead
(soluna −0.210). Two departures from the inventory tier's call, both
measured:

| variant | 1000 | 1442 | 2080 | **3000** | 4327 | 6240 | 9000 |
|---|---|---|---|---|---|---|---|
| base (`d516cd1a`) | 0.6269 | 0.6249 | 0.6311 | **0.6413** | 0.6031 | 0.5696 | 0.5454 |
| all entries, full mass | 0.6317 | 0.6245 | 0.6300 | 0.6377 | 0.6036 | 0.5705 | 0.5471 |
| + catalog-child suppression extended the same way | 0.6237 | 0.6080 | 0.6059 | 0.6148 | 0.5739 | 0.5423 | 0.5214 |
| terminal entries, full mass | 0.6332 | 0.6249 | 0.6331 | 0.6406 | 0.6045 | 0.5711 | 0.5468 |
| terminal entries, 0.5 mass | 0.6300 | 0.6255 | 0.6325 | 0.6410 | 0.6044 | 0.5705 | 0.5468 |
| **terminal entries, 0.75 mass (shipped)** | 0.6332 | 0.6253 | 0.6334 | **0.6412** | 0.6040 | 0.5711 | 0.5468 |

- **Count only terminal (file) entries.** Counting subdirectory rows too
  neutralizes package roots whose content lives one level down, and
  buying that index seeds the swarm below it: vite's
  `packages/create-vite` pulled **1751 tokens across 85 rows** of
  `template-*/` scaffolding listings inside 3000 (−0.057), and
  linkwarden's `apps/mobile` + `apps/web` the same way (−0.055). Both go
  to exactly zero when the mass counts files only.
- **Partial neutralization (0.75).** Non-monotone in strength: 0.75 is
  above both 0.5 and 1.0 at 3000 and ties 1.0 at 1000/2080.
- **Extending `parent_is_high_fanout_catalog`'s gate the same way is
  measured dead** (−0.0265 at 3000, negative at all seven budgets). The
  child-suppression precondition is load-bearing; do not re-run.

The residual 3000 loser is **beets −0.055**, and it is a timing
exposure, not a cost of this rule: the `beetsplug/` listing is NS row
1.18 at `exp_t=1708` and the walker now delivers it at 1513 (was 2710),
but scheduling it drags the same `python imports in beetsplug/*/
__init__.py` cascade — 315/349/151/428/489/506 tokens of import blocks —
from just past 3000 to just inside it. The identical cascade is present
in the baseline schedule at 3079+. That is the probe's D2 ("a file's
first admitted chunk is its least informative region") priced through
`imports_value`'s `__init__.py` tier, not a listing-value problem.

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

## Import blocks are refinements, not entry points (2026-07-26)

**Shipped** as `PackageImports`'s `DeclNames`-head predecessor in
`src/walker/go.rs` and `ModUse`'s declaration-surface predecessor in
`src/walker/rust.rs`. Corpus-flat at every grid budget
(Score(3000) 0.6389 → 0.6388, Score(9000) +0.0002); shipped for the
render, not the headline.

**The defect.** A blind usability probe found the only class where
*more* budget makes the reader more confidently wrong: an ungated
import block is the cheapest batch a source file offers, so the first
marginal token ever spent on a file buys its least informative region.
gin's `routergroup.go` at 9000 rendered `package gin` + four std
imports and nothing else — "examined, it's plumbing" — while the same
file at 3000 rendered bare and correctly read as "not covered". Every
ranking improvement that shifts budget toward a file first buys that
file's imports, so this silently taxes future work.

**Suppression is dead; reordering is not.** The 2026-07-18 "Go
import-crumb suppression (7 variants)" result stands — deleting the
content frees tokens that never buy NS-aligned replacements, and that
is confirmed again here: the metric does not move. The mechanisms
differ in what they predict. Gating keeps the batch purchasable (the
dependency surface really is informative on some files) and only
denies it the *first* slot; the win is that a file's first admitted
content is now a declaration roster, and a file that cannot afford one
stays honestly bare. Measured by rendering the 12 Go + 8 Rust training
fixtures at 3000 and 9000 and counting files whose entire rendered
content is import plumbing: **41 → 5**, and 4 of the 5 residual are
files that declare nothing at all (nothing to gate on) or a genuine
`doc.go`. Dense-budget band means (50-token steps) are +0.0002 at
0.5–1.5K, +0.0001 at 1.5–3K, flat above.

**Rule.** Go: every `PackageImports` gates on its file's `DeclNames`
head chunk; decl-less files stay ungated. Rust: `ModUse` gates on
`PubItemNames` (or the lone `PubItem` when no roster is emitted) **only
when the block declares nothing** — a top-level `mod` item or a
`pub use` re-export publishes names rather than importing them, so
those blocks are declaration surface and stay entry points. The Rust
half is near-inert on this corpus (one file, sps `model/artifact.rs`);
it is carried because the class is identical and the guard is what
keeps crate-root module tables and re-export surfaces unaffected.

**Measured and rejected: splitting the `package` clause out of the
imports batch** (clause rides the package-doc lede, `PackageImports`
becomes imports-only). Motivated by xxhash, whose NS 1.4 "package doc
comment" spans `xxhash.go:1-3` — line 3 is the clause, so gating the
fused batch flips that row from reached to partial (−0.005). The split
costs more than it recovers: gin −0.064 @2080, act −0.036 @1442,
corpus −0.0008 @2080 / −0.0004 @9000. `package X` carries real
NS-aligned mass on files with no package comment; it is not plumbing.
The xxhash −0.005 is accepted as the price of the gate.

**Residual class, not addressed:** a Go file that declares nothing
renders its bare `package X` clause as its whole content
(`lo/simd.go`, `mcphost/internal/tokens/anthropic.go`). Same shape in
miniature, but there is no roster to gate on and the batch is ~3
tokens. Other walkers were out of this lane's file ownership; C
include guards (probe-reported, ~7 occurrences at 3K in htop, ~25 at
9K) are the same defect in `src/walker/c.rs` and are unfixed.

## C header guards and identity slots (2026-07-26)

Follow-up lane to "Import blocks are refinements, not entry points",
which flagged C include guards as the same defect. Both changes are
corpus-flat at 3000 and shipped for the render.

**C's include block was already gated.** `Includes` has taken the
file's first `DeclNames` chunk as its predecessor since 2026-06-12, so
the Go/Rust fix has no C analogue to port. What the probe actually saw
(`2→#define HEADER_ZfsArcStats` as a header's entire rendered content)
came from a different leak: the guard's `#define` was recognized by
*name shape* — all-caps, no value — while the `#ifndef` half was
recognized structurally. Guards named `HEADER_CamelCase` (htop) or
`soluna_version_h` (soluna) failed the shape test and classified as
public macros, landing first on the file's roster because they sit on
line 2. Matching the define against the file's own guard symbol fixes
it: 25 lines leave the corpus rendering, mean flat, 2080 +0.0003.

**The all-caps fallback is load-bearing.** Dropping it and relying on
the structural match alone costs **bareiron −0.170 @3000** — value-less
all-caps `#define`s that are build switches rather than guards then
flood the roster. Both rules stay.

**C has no polarity-qualifier population.** The Rust/Go/TS roster-tier
qualifier lanes have no C counterpart to port: `__attribute__((
deprecated))` and `__attribute__((visibility))` appear in zero training
`.c`/`.h` decls outside a vendored tinyusb BSP's Doxygen prose. Do not
re-run "bind the C disavowal qualifier to the roster" — there is
nothing to bind.

**What C has instead is the identity slot.** A struct big enough to
split renders as `typedef struct Process_ { … } Process;` — a name and
nothing else — and on the names roster it renders as its opening line
alone. C's two ways of saying what a type *is* both live in the first
member: the embedded supertype of the vtable idiom (`Row super;`) and
the discriminant of a tagged union (`NodeKind kind;`). Hoisting a
single-line, non-pointer, non-array, non-bitfield leading member of a
named type onto both the roster and the trimmed `Decl` header
(`base_object_member_line`) is corpus-flat at 3000 and +0.0002 @4327
(htop +0.015, tinyusb −0.001). Six fixtures have any qualifying
aggregate. htop's rendering trades `ProcessMergedCommand`'s four leaf
fields for the fact that `Process` extends `Row`.

## Un-ship sweep against the v2 NS key (2026-07-28)

Zero point 0.6074 / 71 training fixtures. Every un-ship verdict
recorded before the 2026-07-05→07-28 NS re-freeze v2 was measured on a
different key and does not transfer — three of the five candidates
re-measured here reversed sign. **Re-measure, do not cite.**

Shipped (each its own commit, grid in the message):

| mechanism | Δ@3000 | mover | non-test LOC |
|---|---|---|---|
| dominant-C-binary names promotion | +0.0002 | krep +0.012 (+0.143 @4327) | −78 |
| YAML reference deep-leaf slices | +0.0016 | sqlite-vec +0.117 | −326 |
| TS test/benchmark name rosters | +0.0000 | none @3000 | −228 |
| TS documented-member doc slices | +0.0003 | p-queue +0.019 | −363 |

Cumulative: 0.6074 → 0.6095 @3000; +0.0001 / 0.0000 / 0.0000 /
**+0.0021** / +0.0015 / +0.0009 / −0.0001 across the grid, for −995
non-test lines.

**The recurring shape is frontier packing, not content quality.** Each
of the three winning removals was leaving 200–440 tokens of the 3K
budget stranded behind a batch the mechanism had made too expensive to
buy (krep 349, sqlite-vec 437, p-queue 207). The mechanisms were not
rendering *wrong* content — they were rendering content in
indivisible-enough units that the scheduler could not fill the primary
budget. A candidate whose fixture shows large 3K slack is worth
re-measuring for that reason alone.

**The TS test-roster removal is the one that cost something**: exactly
0.0000 at every budget except 9000, where mitt −0.017 and debug −0.015
give −0.0005. Accepted for −228 LOC and for removing the tree's least
bounded content class (test-label mass has no cap and a repo's test
tree can dwarf its source). The separate `BENCHMARK_NAMES_EARLY_TIER_MAX
= 8` value cliff measured flat on its own, so it was buying nothing
either.

**Dead-code inventory: already clean.** The 2026-07-26 inventory's
never-read `diagnose.rs` fields, `oracle_relevant_count`, the python
tiny-tail fold, `TsKey::ExportMemberDocTail` and `LuaKey::Banner` are
all gone from the tree. A fresh key census over all 71
`tests/snapshots/schedule/*.toml` leaves three keys never scheduled
≤10K — and none of them is dead code:

- `Python::DeclNamesChunk` — the *head* split is the live effect; the
  module doc already records that unifying every roster costs −0.0007.
- `Python::DeclDocRest` — removing it is a recall hole (a long
  docstring's remainder would never be emitted at any budget), not a
  deletion.
`Python::TestNames` has *one* scheduled batch now (the inventory said
zero) — the census must be re-run, not inherited.

### Second wave (zero point 0.6150, after the week's merges)

The four candidates the first wave did not cover. Same protocol: revert
the mechanism, measure the whole grid, keep the revert only if 3000 is
flat-or-better with no material higher-budget loss.

| mechanism | Δ@3000 on removal | mover | verdict |
|---|---|---|---|
| Makefile target skeleton | −0.00003 | beszel −0.002 | **removed**, −384 LOC |
| compose topology skeleton + tail | **+0.00009** | linkwarden +0.007 (+0.049 @4327) | **removed**, −404 LOC |
| C configuration-surface header role | −0.00137 | bareiron −0.097 | kept |
| Rust nested-entry coalescing | −0.00103 | sps −0.041, mdbook −0.032 | kept |

Combined removals: 0.61502 → 0.61508 @3000; +0.0000 / −0.0000 /
−0.0000 / **+0.0001** / +0.0006 / −0.0001 / +0.0002 across the grid.

**Frontier slack predicted both outcomes.** Measuring peak
`Score(B=cum)` in the 1500–3000 window against `Score(3000)`:
linkwarden carried +0.255 of slack and its sole-beneficiary mechanism
measured *negative*; bareiron carried +0.000 (monotone to the frontier)
and its mechanism is genuinely earning. Compute the slack in a window
near the frontier, not over the whole prefix — the early-schedule
maxima (krep peaks at 1.000 by cum=103) are an artifact of scoring a
two-batch prefix, not real slack.

**Both removals restore a strictly simpler contract**, not a different
one: an over-cap Makefile and an over-cap compose file are once again
suppressed entirely rather than parsed into a bounded skeleton. Two
hand-rolled structural parsers (a four-statement-kind Make parser with
logical-line continuation; a compose service/field indent walker), two
chunking paths that never bought a second chunk anywhere in the corpus,
and `MAKEFILE_TARGET_MASS_BASELINE`/`_FLOOR` — a second roster-mass
model competing with `value::roster_mass_factor` — are gone.

**The C config-surface floor stays, and the multiplier alternative the
inventory proposed does not fix it.** Three variants were measured
against the shipped absolute floor of 1250:

| variant | @2080 | @3000 | @4327 | @6240 |
|---|---|---|---|---|
| removed entirely | −0.0009 | −0.0014 | −0.0019 | −0.0009 |
| ×1.4, names chunks only | −0.0003 | +0.0000 | +0.0000 | −0.0008 |
| ×1.4, positional selection kept | −0.0009 | +0.0000 | +0.0000 | +0.0001 |
| ×2.0, positional selection kept | −0.0031 | +0.0017 | +0.0014 | +0.0001 |

The ×1.4 rows are flat at 3000 because 1.4 is what the absolute floor
already worked out to at bareiron's `include/` depth — the multiplier
form re-expresses the same fit rather than generalizing it. Pushing to
×2.0 buys +0.0017 @3000 entirely from bareiron (+0.119) while costing
tinyusb −0.224 @2080, which is exactly the wrong-header promotion the
inventory warned the role would cause on unseen repos. Do not sweep
this constant upward on training score.

## README orientation-vs-source ordering: the value knobs are exhausted (2026-07-29, v2 key)

Lane target was the README-class "late" loss — 52 of 71 training
fixtures carry it and ~64% of that mass is content that *is* scheduled,
just past 3K (go-multierror README 0.415, mitt 0.289, cmdk 0.251, krep
0.212). The oracle-vs-walker class gap says the walker under-buys
substantive README `Section` mass (22.0K vs 33.3K) while over-buying
`ReadmeHeadline` and `HeadingsOutline`. Every knob that could close
that gap was re-swept on the v2 key and **every one already sits at its
local optimum**; the zero point was 0.6151 over the 71 training
fixtures throughout.

- `CANONICAL_USAGE_SECTION_FACTOR`: 1.8→0.6135 · **2.2→0.6151** ·
  2.4→0.6149 · 2.6→0.6149 · 3.0→0.6149. The pre-refreeze 2.2 survives
  the re-freeze; above it the curve is a flat plateau slightly below.
- **Uniform README-section value factor** (all sections × k, the level
  axis the old "section-mass factor" entry never isolated):
  0.9→0.6081 · **1.0→0.6151** · 1.12→0.6145 · 1.25→0.6110 ·
  1.5→0.6043. Clean single peak at the shipped value.
- `readme_index_decay`: exponent 0.0→0.6087 · 0.08→0.6148 ·
  **0.15→0.6151** · 0.25→0.6142; floor 0.7 (shipped) →0.6151 ·
  0.8→0.6148 · 0.85→0.6155 · 0.9→0.6132 · 0.95→0.6095. The 0.85 point
  is a knife edge — both neighbours sit below base — same signature as
  the `DEFAULT_CONCAVITY_EXPONENT` 0.36 artifact.
- **README section-mass factor, re-measured on the v2 key** (value ×
  `(tokens/100)^e`, baseline 100): boost-only e0.10→0.6148 ·
  e0.20→0.6155 · e0.35→0.6136; demote-only e0.10/floor0.7→0.6138 ·
  e0.35/floor0.7→0.6144 · e0.35/floor0.5→0.6148. **The old key's
  demote-only +0.0025 has flipped negative.** The boost-only peak is
  not a lever: it moves 12 fixtures and is dominated by commander
  −0.139 against beszel +0.096 — the same tiny-section bimodality the
  old entry recorded, with the signs reshuffled.
- `MarkdownKey::Section` concavity exponent (index ≥1, non-reference):
  0.40→0.6107 · 0.42→0.6150 · **0.45→0.6151** · 0.48→0.6103. Sharp
  peak; this is the knob that makes tiny sections beat big ones in the
  ratio race, and it cannot be relaxed.
- **Canonical-usage sections exempted from the steep prose exponent**
  (the `reference_shaped` argument applied to code-dominant demo
  sections): −0.0007. Dead.
- **README sections delivered in document order** (each root-README
  section gated on its predecessor, the `chained_to_previous` shape
  generalized): all sections →0.6119, prose-only (reference-shaped and
  roster ranges stay free) →0.6118, first-4-prose-only →0.6132. The
  winners are the fixtures whose walker was buying scattered tiny
  sections (mitt +0.060, tomli +0.068, p-queue +0.055); the losers are
  reference READMEs whose valuable sections are late-index and get
  blocked (json-server −0.115, sqlite-vec −0.111, krep −0.094 — krep's
  oracle buys sections #45/#48/#43 first). No walk-time signal
  separates "read top-down" from "addressable reference" READMEs.

Conclusion for the next session: the README-late bucket is not
reachable by value or ordering knobs on the markdown side. The oracle
funds its extra README mass out of the classes the walker over-buys —
listing (+21.6K), config (+8.4K), manifest (+6.6K) — so the lever lives
in *those* lanes, or in new recall, not here. This is the same shape as
the "early-budget ratio wall" verdict, re-confirmed on the v2 key.

### Un-ship: the deferred-mass-prose scheduler tier (inert at 3000)

`prose_mass_tier_multiplier` gave operationally dense README/dev-doc
prose sections a 1.5× ratio boost once 25% of the budget was consumed,
and excluded them from `is_orientation`. **Both halves are inert at the
primary budget**: boost 1.0 (mechanism off) / 1.2 / 1.35 / 1.5
(shipped) all score 0.6151, and 1.9 costs −0.0006. Zeroing the
`deferred_mass_prose` flag as well — which also flips `is_orientation`
back to unconditionally true for markdown — gives the identical result.
Only dockly and microbootstrap move, both +0.001.

Grid on removal: 1000/1442/2080/3000 bit-identical (0.6132 / 0.6217 /
0.6277 / 0.6151), 4327 0.5851→0.5839, 6240 0.5649→0.5654, 9000
0.5606→0.5604. The −0.0012 at 4327 is the only real cost, against
−264/+19 lines: the `DensitySignal` two-mode plumbing collapses to the
single dev-workflow path, and `looks_like_option_or_env_row` plus its
two exclusive helpers go with it. Precedent for accepting the
off-primary cost: `TEST_INDEX_LISTING_BOOST` (−0.0016 at 1442, ~25
lines). If a future lane wants a late-prose tier back, note that the
window fraction is *also* inert (0.15 / 0.25 / 0.35 all →0.6151), so
the shape — not the constants — is what failed.

## Contributor-toolchain demotion (2026-07-29): 0.6151 → 0.6165

Lane premise: at B=3000 the walker over-buys against the NS-aware
oracle by class — listings −21.6K tokens, config −8.4K, manifest
−6.6K, imports −2.4K. Cross-referencing every walker row ≤3000 in the
divergence reports against the oracle's purchase list at the same
budget gives a *credited vs uncredited* histogram per class, and the
config/manifest over-buy turns out not to be uniform. Two sub-classes
are essentially never credited:

| sub-class | oracle-also (tok) | uncredited (tok) |
|---|--:|--:|
| `tool.<name>` config families in a Python manifest | 236 | 2734 |
| dev / build / target / peer dependency rosters | 0 | 1022 |
| whole `config` class for comparison | 3982 | 12382 |

The 236 credited tool-config tokens are `tool.poe` (a task runner) and
`tool.setuptools` (a build backend) — not one linter, formatter, type
checker, test runner or coverage table is credited anywhere in the
corpus. That is the discriminator: a table that configures the
**contributor's checking toolchain** says nothing about the project,
while a build-backend or task-runner table says how it is built and
invoked. Both shipped mechanisms are that one rule.

- `walker::toml::CHECKING_TOOLCHAIN_CONFIG_SCALE = 0.35` — demotes a
  `ToolConfig` batch when *every* family in its pack is a checking
  tool. A mixed or unrecognized pack keeps full value, so the rule
  never fires on an appendix it cannot classify.
- `value::DEV_DEPENDENCY_ROSTER_SCALE = 0.3` — the same distinction one
  class out, applied at both dev-roster sites (Cargo dev/build/target
  tables + PEP 735 groups; `package.json` `devDependencies`).

  **Not** `peerDependencies`. `JsonKey::DevDependencies` spans
  `devDependencies` + `peerDependencies` + `peerDependenciesMeta` for
  batching reasons, but a peer roster is a consumer-facing compatibility
  contract, not a contributor toolchain — the demotion applies only to a
  peer-free batch (`walker::json::dev_dependencies_value`, corrected
  2026-07-29). Cargo and PEP 735 have no peer analogue, so that site is
  unaffected.

Grid (71 training fixtures): 1000 0.6132→0.6132 · 1442 0.6217→0.6234 ·
2080 0.6277→0.6279 · **3000 0.6151→0.6165** · 4327 0.5851→0.5882 ·
6240 0.5649→0.5653 · 9000 0.5606→0.5632. Positive or flat at all seven
budgets. Movers @3000: tomli +0.068 (295 tokens of frontier slack, not
a cliff artifact), requests +0.020\*, typeguard +0.006\*, pluggy
+0.004, debug +0.002, click −0.001\* (\* = last walker row within 40
tokens of 3000).

Sweeps, all measured on the full corpus:

- `CHECKING_TOOLCHAIN_CONFIG_SCALE`: 1.0→0.6151 · 0.60→0.6164 ·
  0.35→0.6164 · 0.15→0.6164. Broad plateau from 0.6 down; 0.35 chosen
  as its centre.
- `DEV_DEPENDENCY_ROSTER_SCALE` (on the shipped config scale):
  1.0→0.6164 · 0.6→0.6165 · 0.3→0.6165 · 0.15→0.6165. Flat at 3000;
  0.3 chosen on the higher-budget grid (+0.0008 @4327, +0.0021 @9000
  over 1.0, and 0.15 adds nothing further).

**Measured dead: the undiscriminated version of the same lever.** A
blanket `config_value` rescale to 0.80 across the whole manifest config
appendix costs −0.0010 (htmy −0.051, requests −0.015, beets −0.010
against peepdb +0.008, pluggy +0.004) — the credited fixtures lose more
than the uncredited ones gain. The tool-name axis is what makes the
demotion pay; do not retry the class-wide form.

**Measured inert: extending the rule to the unpartitioned `Config`
batch.** Applying the same all-checking-tools test to the generic
`TomlKey::Config` appendix (which owns the tool tables when they total
≤ `TOOL_CONFIG_FAMILY_MAX_TOKENS`) produced a **byte-identical** corpus
— every divergence report unchanged. No training manifest has a small
config appendix that is purely checking tooling, so the extra branch
was pure dead code and was dropped.

## Data-model schemas and C interface headers (2026-07-29)

Zero point 0.6151 / 71 training fixtures. Two narrow ships, plus five
dead lever shapes on the C side.

**A data-model schema is priced at root tier.** A file declaring the
application's persistent entities is spine — NS authors rank its catalog
beside the root manifest — while its filesystem depth records only which
workspace package owns the ORM client. Pinning the Prisma walker's depth
factor to root (the clamp `file_depth_factor` already gives entrypoints)
is +0.0033 corpus, all of it linkwarden 0.457 → 0.693 @3000, with the
whole grid up except 6240 (−0.0003). Measured as a multiplier over the
un-pinned factor (the pin is ×1.7 at that file's depth): 1.2 0.450 ·
1.35 0.532 · 1.5 0.614 · 1.65 0.676 · **1.7 (pin) 0.693** · 1.9 0.707 ·
2.2 0.707 · 3.0 0.767, with budget-1000 collapsing above ~2.0. Adding
`roster_mass_factor` on the catalog is bit-inert on top of the pin.

The gate is the data model, not configuration: this is deliberately
narrower than the measured-dead ops-config class boost. Deploy / CI /
tool config describes how a project is built and run, and its depth does
track its scope. As first shipped the gate was only the *filename*,
which let a `schema.prisma` holding nothing but `datasource` /
`generator` blocks take the pin — the config half of a multi-file Prisma
layout, or a generated client's copy. Corrected 2026-07-29 to require at
least one `model` / `enum` declaration; corpus-inert (no training
fixture has a config-only schema), so it is a statement about what the
rule means, not a score move. **Only linkwarden carries a schema in
training** —
drizzle-orm is holdout, no fixture has SQL migration or GraphQL schema
files the oracle wants (sqlite-vec's `SQL schema contracts` chunk has
oracle mass 0), so the rule generalizes on its statement, not on corpus
breadth.

**C header centrality ranks on total include in-degree.** The public
header of a library is included by implementations, examples and board
support, not by other headers, so the header-to-header ranking
under-rated it: tinyusb `usbh.h`/`usbd.h` sit at h2h 1 / total 20 and
were unscheduled at every budget ≤10K. Switching only the *boost basis*
to total is +0.0001 @3000 but +0.0008 @2080, +0.0005 @6240, +0.0011
@9000, and the two movers rise almost monotonically (tinyusb +0.079
@9000, htop +0.052 @2080). The two graphs answer different questions and
both are load-bearing: on the ten-fixture C subset at 3000 (base 0.6196),
moving `is_top_include_hub`'s roster-mass test to total is 0.5856 and
moving the spine gate is 0.5970.

Measured dead in the same lane (C subset means at 3000, base 0.6196):

- **`roster_mass_factor` on C `DeclNames` outside top include hubs**
  — 0.6006 class-wide (chibicc −0.123, bareiron −0.097, neco −0.031),
  0.6103 gated to the repo-eponymous header (chibicc −0.123, neco
  −0.031, krep −0.007, jq +0.068). The pre-refreeze verdict survives the
  v2 key unchanged; the failure is the same early-budget displacement of
  NS tier-1 orientation. tinyusb is bit-identical under both — its
  public headers lose on cost, and mass alone does not close the gap.
- **Flattening the names-surface chunk decay for C headers** (so a big
  roster arrives complete): falloff 0.25 (shipped) 0.6196 · 0.15 0.6194 ·
  0.10 0.6164 · 0.05 0.6088 · 0.0 0.6028. Gains at 4327–9000, loses at
  the primary budget, and tinyusb never moves — the tail chunks are not
  what blocks it.
- **Damping `AggregateMemberGroup`** despite oracle mass 0 for the class
  corpus-wide against 858 walker tokens (all chibicc): ×0.8 0.6171 ·
  ×0.6/0.45/0.3/0.15 all 0.6163. The freed budget does not go anywhere
  the NS wants; the over-buy is real but the demotion is not the lever.
- **Boost magnitude on the total-in-degree basis**: 0.4 0.6209 ·
  **0.6 (shipped) 0.6204** · 0.9 0.6204 · 1.2 0.6176 · 1.6 0.5987 ·
  2.2 0.5986 — flat plateau, keep the existing constant.

Two shape findings for whoever retries the C cluster. sqlite-vec is the
counter-example to "header first": it is a single-`.c` library and its
oracle buys `sqlite-vec.c` names surfaces #3/#4, no header at all. And
no single structural proxy selects the NS-wanted header across the
corpus — largest project roster picks a vendored `lib/networking/ndis.h`
in tinyusb, highest total in-degree picks `globals.h` over `packets.h` in
bareiron, and the repo-eponymous test does not exist in bareiron, htop or
tinyusb. The value-ranking boost above is deliberately graded rather than
a selection.

## Wave-1 integration (2026-07-29): 0.6151 → 0.6209

Four calibration lanes merged onto one tree, then an adversarial review
gated the ship on five correctness findings before the combined
measurement was allowed to count. All five are fixed; this entry records
the combined result, which is what the next lane's zero point should be.

Grid (71 training fixtures), `v0.2-rewrite` → integrated: 1000
0.6132→0.6134 · 1442 0.6217→0.6239 · 2080 0.6277→0.6307 · **3000
0.6151→0.6209** · 4327 0.5851→0.5911 · 6240 0.5649→0.5662 · 9000
0.5606→0.5638. Up at all seven budgets.

**The lanes do not add.** Isolated deltas summed to +0.0076 at 3000; the
tree delivers +0.0058. The gap is not a merge defect — the review's
fixes cost some of it on purpose (the dominant-file premium's boosted
set shrank to its intended surface classes), and the lanes overlap on
the same 3000-token frontier, so two lanes that each move a fixture past
the same cliff are credited once. Always re-measure a merged tree;
never report a sum of isolated lane deltas as a tree result.

Eighteen fixtures move the headline, seventeen up: linkwarden +0.236
(the schema pin), tomli +0.067, cmdk −0.060, log +0.050, swarm +0.038,
cobra +0.025, requests +0.020\*, xxhash +0.016, tinyusb +0.008,
microbootstrap +0.006\*, typeguard +0.006\*, pluggy +0.004, bubbletea
−0.004, click −0.001, thiserror/htop/dockly/ky ±0.001 (\* = last walker
row within 40 tokens of the 3000 cliff, so the sign is frontier
placement rather than ranking).

cmdk is the one real regression and it is the surface predicate working
as specified: `TsKey::ExportDoc` was taking the premium through the old
`!is_depth_follow_up` spelling, and cmdk's NS credits the per-export
JSDoc train on `cmdk/src/index.tsx` (its spine file, 32 tokens of
frontier slack at 3000). Probed the obvious accommodation — adding
item-head doc classes (`ExportDoc`, `ModuleDocLede`, Go/C/Python/Lua
`DeclDoc`, `PubItemDocLede`, `CrateDocLede`) back to
`is_dominant_file_surface`: +0.0005 at 3000 against −0.0007 at 2080,
−0.0019 at 6240 and −0.0009 at 9000. Not taken — it buys the primary
sliver by re-blurring the roster/doc distinction the predicate exists to
draw, and pays for it across the rest of the grid.

## Wave-2 combined state (2026-07-29)

Two lanes merged on top of the wave-1 tree (0.6209): interior-listing
suppression 0.85 (defer a listing when the directory AND its immediate
parent are non-essential) and public-surface-density dominant-file
retargeting (challenger needs ≥20% density margin + same semantic
type-machinery class; density = span-deduplicated top-level AST count).
Combined grid vs the wave-1 tree:

B=1000 0.6134→0.6159 · 1442 0.6239→0.6233 · 2080 0.6307→0.6311 ·
**3000 0.6209→0.6224** · 4327 0.5911→0.5909 · 6240 0.5662→0.5657 ·
9000 0.5638→0.5637 — the two ships compose additively (isolated
+0.0003 and +0.0012 at 3000).

Knob re-sweep on this frontier confirmed the settled values again:
LISTING_TIER_SCALE 1.13 (1.14/1.16 microscopically higher at 3000 but
−0.005 at 1000), CATALOG_ROSTER_CONCAVITY_EXPONENT 0.38 (0.385 is a
cliff-concentrated numerical peak — full-diff review rejected it; 0.39
trades five real regressions for two concentrated wins that OVERLAP the
suppression lever's superstruct win), DEFAULT_CONCAVITY_EXPONENT 0.35
(0.355/0.36 sum −0.053/−0.071, near-all cliff-adjacent).

Measured-dead this wave (specifics block retries): dominant-file
first-roster-chunk entry factors — Go ×1.5–3.0 flat with act/lo rosters
still absent at 10K, Python ×1.5–3.0 flat (click +0.023 vs pluggy/
typeguard losses), C ×1.5 −0.0037 (sqlite-vec −0.116, neco −0.107, the
implementation-roster-displaces-header failure again); extra
dominant-surface opt-ins (Python ClassBody, Go StructFieldGroup) inert;
peer/dev dependency split or peer-bearing demotion (debug-only, flat
curve); tiny depth-follow-up demotion (8-tok gate neutral, 16-tok
regresses — cost alone cannot separate crumbs from credited API
detail). The late-displacer histogram (ignore/session-2026-07-29/)
identified secondary-module breadth (helper modules bought before the
primary API/data model, 5/8 fixtures) as the largest unaddressed
displacer class; the dominant-file detector is the candidate
primary-vs-helper discriminator uniform sibling damps lacked.

## Wave-3 state (2026-07-29, session close)

One ship: Python data-model catalog roster promotion (19df2070) —
0.6224 → 0.6249 at 3000, grid flat elsewhere. The census lane measured
DEAD an entire candidate class: **contributor scaffolding is not a
coherent demotable value class.** Its census (method: oracle-purchase
vs walker-purchase mass + direct training-NS credit checks): contributor
templates are credited by many NSes (go-multierror/mitt credit bodies;
8+ fixtures credit the directory listings); .github structure listings
carry 724 oracle tokens across 35 fixtures (the 1.3K uncredited excess
is real but inseparable from credited topology closure at batch
granularity); changelog tails and sub-100-line Makefiles are UNDER-
bought per the oracle; manifest identity appendices are bimodal (3.0K
oracle tokens / 20 fixtures). Do not re-attempt scaffolding demotion at
these boundaries; a retry needs a finer-than-batch listing
representation.

Measured this wave on the data-model shape: rendered-row field counts
and a per-class fields≥2×methods ratio both fail (the ratio admits
ORM model files — linkding −0.041 — and drops mixin-shell settings
classes whose model is the bases list). The shipped gate is file-level:
≥4 public classes, ≥4 AST field assignments, ≤2 AST methods across
them, per-chunk majority-class application, entrypoint-exempt.

Open (lane died to route errors before reaching them): the two other
helper-breadth shapes — scheduler-side breadth gate on the dominant
train, and value-side sibling roster damp gated on the dominant-file
detector.

## Session 2026-08-01 (final polish + unshipping): 0.6249 → 0.6320

Grid (71 training fixtures), 07-29 close → this close: 1000
0.6159→0.6106 · 1442 0.6233→0.6264 · 2080 0.6311→0.6315 · **3000
0.6249→0.6319** · 4327 0.5908→0.5959 · 6240 0.5657→0.5690 · 9000
0.5637→0.5660. Up at six of seven budgets; the 1000 regression is a
recorded trade — see "budget-grid discipline" below, and the
Go entry-slice item (measured to repay it, reverted on a review
blocker). Session artifacts in `ignore/session-2026-08-01/` (plan.md is
the running log; probe crate and ranking traces under `probe/` and
`out/`).

### Ships (each merge re-measured; two adversarial review passes)

- **TS/JS export-surface recall** (+0.0028; ky +0.106, chalk +0.029,
  ts-pattern +0.024, p-queue +0.016, dockly +0.016, cmdk +0.009, zero
  drops): local `export { name };` clauses synthesize Exports at
  function/class/type declarations; CJS receivers resolve through
  `new`/call expressions (`module.exports = new Cli()`); trailing bare
  re-export blocks get `TsKey::ReexportTail` (gated behind ExportNames,
  roster-priced, surface class, concavity 0.38). Review fixes, all
  corpus-neutral generality completions: tail-only files emit the tail
  (chained behind the roster when present, else module_predecessor),
  ambient `.d.ts` declarations unwrap through the synthesis path,
  statement-level and per-specifier type-only clauses synthesize their
  type declarations without overstating the runtime surface. Recall
  lens that made this the session's biggest ship where past recall
  extensions died: the content is 30–70-token, NS tier-1/2, and lands
  ≤3K (verified per sub-lever).
- **Small root build-file promotion** (+0.0034 with the Taskfile claim):
  Makefile/build.sh/configure.ac at root, ≤100 lines, on
  BuildEntrypoint/BuildScript, ×2.0 (3000-plateau ×1.8–4.5). chibicc
  +0.091, cobra +0.049, bareiron +0.033, lo +0.026. Root
  Taskfile.yaml/Taskfile.yml claimed by classify_plaintext (was claimed
  by no walker; bubbletea +0.042 — emit-only was inert, the promotion
  is what pulls it inside budget). Measured dead alongside: lifting
  SourceProse-floor build files (microbootstrap −0.055 via its
  Justfile); over-cap Makefile head-samples AT the promoted tier
  (−0.0066/−0.0074 — the 60–100-line head is creditless variable
  preamble, and promotion makes buying it catastrophic; unpromoted
  reproduces the old ±0 verdict).
- **Python roster gating** (+0.0004; tomli +0.030, sole mover): chunked
  rosters gated the whole per-decl train on the LAST chunk — conserved
  head-heavy chunk values mean the tail carries ~11% of the value at
  ~95% of the head's cost, prices past the horizon, and forfeits the
  file's depth. Now each roster decl gates on the chunk OWNING its name
  line (Go's spelling); the method-sig catalog keeps the full-chain
  gate (it spans classes across chunks). The intermediate head-chunk
  form left overlap ancestry ranking-dependent — a probe lane's
  reordering panicked on beets — so owning-chunk is also a contract
  repair, not just a preference.
- **Dependency-roster saturation** (+0.0004; chronos +0.024 — the
  corpus-worst fixture finally moved; posting +0.005, express −0.002,
  bimodal config trio provably flat): value saturates at 200 measured
  content tokens, (200/mass)^0.5 past it, on package.json
  dependencies/devDependencies + TOML Dependencies. Damp-only,
  size-keyed; threshold plateau 160–240. Ceiling is structural: of ~22
  fixtures with >200-token tables, most schedule past 3K already. Use
  the real tokenizer for any mass gate — bytes/3.6 undercounts
  dependency rosters ~1.8×. Related one-liner: the dev-roster
  demotion's peer exemption now requires a real `peerDependencies`
  table — `peerDependenciesMeta` alone is an optionality annotation,
  and a 5-line Meta stub was exempting whole dev toolchains (debug
  +0.002).
- **Go entry-slice — measured KEEP, review-BLOCKED, reverted
  (e73d10ae); first item for next session.** The mechanism: in a Go
  directory, the file declaring strictly more top-level names than any
  sibling, whose roster the chunker left whole and which still costs
  >250 tokens, gets a gate slice of its ≤8 highest-rank declarations
  (exported + doc-carrying, ≤80 tok); the remainder follows with its
  unsplit value factors; decl trains gate on the owning chunk. Measured
  (lane V5, 9 variants): +0.0001 @3000, **+0.0048 @1000 true grid** —
  it repays the build-promotion's B=1000 debt with no fixture down.
  Adversarial review then found: a reproduced BLOCKER (cross-slice
  ellipsis ownership — a gate decl adjacent to a remainder decl's
  comment yields a non-ancestor overlap: debug panic, silent row
  omission in release; also co-located decls split across chunks), the
  80-tok cap unenforced for a single over-cap decl, spine selection
  counting DeclInfo nodes not surfaced names (grouped blocks
  undercounted) with no generated-file exclusion, and a deliberate 1.9×
  value non-conservation (gate 0.9 + remainder 1.0 — RULED
  measured-deliberate, conserved forms V1–V3 measured worse, xxhash
  −0.045; document, don't "fix"). Three fix-pass agents were lost to
  API stalls at session close, so the merge was reverted rather than
  shipped with a known silent-omission bug. Everything needed to finish
  is preserved: lane worktree `agent-aee06211fdeab0682` (branch at
  dd3a8ec4 + partial finding-1/4 edits uncommitted), the four findings
  with fix guidance in the session log, and the V-grid. Measured
  constraints for the redo: remainder must KEEP pre-carve value factors
  (re-indexing demotes the catalog the gate exists to reach); confine
  to the directory spine with unchunked rosters (cheap gates repo-wide
  admit half-read depth trains — gin −0.228; lo's decl-count "spine" is
  tuples.go boilerplate). V9 variant for a small-budget session:
  remainder chained behind gate + depth gated on remainder reaches
  B1000 +0.0100 for −0.0015 @3000.

### Un-ship audit (re-measured on this session's base — verdicts do not transfer across re-freezes)

Nothing un-ships; all marginals positive. Dominant-file stack +0.0022
TOTAL and it pays ONLY via the scheduler premium (detector has no other
consumer — stubbing it changes nothing else); carriers express/log/swarm
are frontier-packed (slack 20/8/21); costs cmdk −0.060; ~330 LOC.
Re-audit after the next NS re-freeze. Interior-listing 0.85: +0.0002
@3000 (superstruct, slack 4) / +0.0025 @1000 (cmdk, slack 17). Prisma
pin +0.0033 (100% linkwarden, NOT packed — slack 88). C total-in-degree
+0.0001 @3000 (pays at 2080/9000 per its ship grid). Python data-model
promotion +0.0025 (~99% microbootstrap, not packed). Catalog concavity
0.38 → 0.35 would cost 0.0029 across nine fixtures — the only broadly
supported constant.

Knob re-sweep on the close frontier: all six settled constants
CONFIRMED (LISTING_TIER_SCALE 1.13, catalog exponent 0.38, default
exponent 0.35, dominant boost 1.35, suppression 0.85, TRAIN_PRESSURE_K
0.15). LISTING_TIER_SCALE 1.19 (+0.0007) and boost 1.20 (+0.0005) are
cliff-shuffle, rejected on the full diff (cliff-packed count 37→39/40,
±1-row flips, mdbook landing 6 tokens from the cliff). Unexplored if
the frontier moves: LISTING_TIER_SCALE × suppression 2-D sweep (both
price listings in fs.rs).

### Helper-breadth: both queued shapes measured DEAD (queue closed)

Scheduler-side breadth gate: best +0.0001 across an 8-point grid.
Structural kill via instrumented duty cycle — migrate/peepdb detect NO
dominant file; superstruct/chronos detect one that NEVER ENTERS
(entered=false through 10K, every surface unscheduled), so
entered-gating makes premium and damp live only where the primary
already got in. Ungated damp −0.0047 with the targets WORSE —
"ungated breadth damp" joins "ungated premium" in the dead list.
Same-subtree scoping silently degenerates to repo-wide when the
dominant file sits at the root (mkcert, cobra). Value-side sibling
roster damp: all sweep points negative, non-monotone; the same-dir gate
catches the INTERFACE ANCHOR the NS ranks first (sds.h damped under
sds.c −0.051, gin.go under context.go −0.074, nano-vllm engine breadth
−0.079). The detector fires on 35/71 fixtures and frequently names the
wrong file (C: the big .c over the NS-wanted header — krep.c/krep.h,
parse.c/chibicc.h; chronos: chronos2/pipeline.py over chronos.py).
chronos is unreachable by any sibling rule (its NS primaries are in the
PARENT dir of the detected file).

### The never-entered class: ranking side closed, emission side open

Instrumented trace (probe crate; per-round ratio/eligibility dumps):
file-entry rosters carry class-flat value against N-linear cost, so a
spine file's entry ratio decays like N^-k and ranks LAST within its
class; the roster is a gate, so losing the race forfeits the file (act
root.go 673/374tok ratio 84.7 vs a same-value 34-token sibling at 196;
superstruct types.ts eligible from cum 107 and never bought at 10K).
Breadth pressure and the dominant boost are not involved (pressure 1.00
on every trace row; the boost is entered-gated and provably inert on
never-entered files). The whole ranking-side fix family is now measured
dead: class-constant entry factors (three kills, incl. capped
roster-mass extension −0.0009 and uncapped cost-graded −0.0112 with gin
−0.325); ADDITIVE bundle credit (rank a gate by value + covered
dependents' values: −0.18/−0.14/−0.04 at w=1.0/0.25/0.05 — a fat
roster inherits 5–10× its own value, no constant weight reconciles);
BOUNDED self-normalizing bundle (1 + c·bundle/(bundle+value):
−0.013/−0.024/−0.033 at c=0.2/0.35/0.5 — the saturation term measures
0.38–0.97 on real rosters, so the multiplier is a near-uniform
×(1+~0.85c) = the class-constant family again; act needs 1.63×
within-class separation ⇒ c≥1.8 ⇒ inside the dead band). Two decisive
observations for whoever returns here: entry without credit is
DISPLACEMENT (lo's spine entered at cum 2438 under the bundle and lo
LOST 0.077), and the oracle's +0.26 on this class comes from chain-cost
ranking the scheduler cannot approximate with any per-batch factor. The
live direction is emission-side entry-unit shrink (the Go entry-slice
above is the beachhead; rank-chosen slices only — source-order slices
are completion-damped, measured on lo).

### Other dead levers this session (extend the existing entries)

- README section-mass factor: a section-COUNT gate separates shattered
  from anchored READMEs cleanly (krep 35 tiny sections, lo 114 vs
  commander 1, svgo 3) and still loses (−0.0024/−0.0012/−0.0007 at
  N=8/15/25). Mechanism: shattered-README micro-sections are
  COVERAGE-POSITIVE at 3K (they partially cover NS tier-2 README rows);
  what replaces them is NS tier-3 API detail. The bucket is unreachable
  by re-pricing tiny sections in either direction.
- AggregateMemberGroup damp: re-probed post-build-promotion at
  ×0.8/0.6/0.4 — still dead (best +0.000014); the promotion already
  took the chibicc win directly and the ≤3K member-group wall fell 858
  → 215 tokens.
- Oversized names-roster chunk damp (size-graded, per-chunk): no
  threshold separates chibicc (win needs ≤213 content-mass tokens) from
  sds (NS-anchor chunks damaged below 263 — same band). The +0.092
  chibicc effect is mid-budget; converting it to Score(3000) requires
  damping the whole train = the dead class-wide lever. State units for
  any mass gate: content mass ≈0.62–0.72× rendered cost (gutter).
- Roster-before-depth same-file ordering damp: −0.0003..−0.0005; the
  per-item surface heads never exhaust so the gate is near-always-on (=
  blanket damp, ground held by train_pressure), and the actual chibicc
  displacers aren't in `is_depth_follow_up` at all. Budget-leftover
  affordability gates are structurally inert under the 10K harness.

### Budget-grid discipline (new, learned the expensive way)

Lane contracts that measure only the Score(3000) mean hide small-budget
displacement: the ×2.0 build promotion silently cost −0.0053 at B=1000
(rich −0.44, go-multierror −0.18, requests −0.15 — Makefiles bought
inside the first thousand tokens). Caught only by the close-out
diagnose_loss --budgets. Two rules: (1) lane contracts carry a B=1000
check next to the 3000 mean; (2) budget trade decisions need the TRUE
grid — a last-full-row Score(B=cum) proxy overstated a B=1000 recovery
5× and briefly shipped a wrong retune (reverted in 719f6160; the ×1.5
"recovery" was cliff roulette — chibicc −0.16 against go-multierror
+0.18). The session closes with the −0.0053 standing as a recorded
trade; the Go entry-slice (above) is the measured repayment
(+0.0048 @1000) once its review blocker is fixed.

## Rust private-item recall lane (2026-08-02, W1-RS): 0.6320 → 0.6325

The absent-census E4 class (NS atoms with no emitter concentrated in
Rust private/internal content). Three shipped shapes, one measured-dead
appendage:

- **`PrivateItemNames`** — a private-fn *wall* roster: non-entrypoint
  package source file, top-level private fns (test-marked excluded)
  outnumber pub items AND ≥5 survive after dropping RegistrationRoster
  fns. Priced on the impl-roster tier (`mix(0.6,0.7,0.35,depth) ×
  roster_mass_factor`), **no per-item visibility damp** — at
  `pub_item_names_value` (which folds the 0.4 file-visibility axis) the
  one roster the NS wants landed @5705 while six 2-fn helper rosters
  queue-jumped a cliff (hyperfine −0.083 @3K). The wall gate + tier
  pricing turned that into thiserror +0.037 @3K (roster @2627 vs NS slot
  2529) with every other fixture byte-identical.
- **`ModuleState`** — an entrypoint file's top-level private
  `static`/`const` items as ONE grouped batch (per-item batches would be
  crumbs; a 20-token VERSION-const group bought at cum 501 cost mdbook
  −0.006 @B1000 until the `CRATE_ATTRS_MIN_TOKENS` floor was applied).
  log +0.027 @4327 / +0.015 @6200; flat @3K — the block lands @3678
  against an NS slot of 1936, and a cat bump 0.42→0.55 measured
  byte-identical (the sub-3K band outprices it; don't re-sweep).
- **`Default`-impl admission** in `is_own_api_impl`: `impl Default` on a
  pub type the file declares joins the OwnApiOnly surface. Recall works
  (hyperfine absent 0.241→0.169, oracle +0.012) but pays only at 10K
  (+0.011): the dive chains behind `MethodSigs {options.rs}` at cum
  7863 — the roster-pricing blocker from the 2026-07-26 per-impl-method
  entry, not a recall failure. Widening past `Default` (Display/From) is
  contra-indicated by NS text (anyhow: trait-impl methods "deliberately
  not part of this roster").

Measured dead in-lane: **per-fn sig/body dive behind the private roster**
(`PrivateItem`/`PrivateItemBody`, pub_item_value pricing with the
Restricted 0.4 axis) — never bought ≤10K anywhere, and its only grid
effect was anyhow −0.005 @10K; removed. The ≤3K NS mass of the class is
roster-shaped here exactly as the 2026-07-26 impl-method entry predicted
for anyhow/log/sps; the body mass (thiserror 3.2/3.3 quote! tails,
hyperfine builder-chain detail) stays absent pending either undamped
body pricing (unswept — flood-shaped) or the roster-pricing lever.

Sub-lever grid rows (each measured alone) live in
ignore/session-2026-08-02/lane-w1-rs-recall.md. Also noted there: the
census's "no crate-doc-body key" claim was stale — `CrateDocBody`/
`CrateDocTail` already ship and schedule (thiserror @2113/@2717).

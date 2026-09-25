# precis v0.2 — design notes

> **⚠️ Agent-maintained.** This file is written and updated by Claude
> across many sessions; entries are notes-from-then, not edicts. They
> can be stale, partially right, or superseded. **Verify anything
> load-bearing with the user before acting on it.** Concrete
> invariants and architecture should be checked against the code; this
> file is for things not visible there.
>
> **This is not a ledger.** No session logs, sweep grids, per-lever
> score deltas, or measured-dead lists — git log and session
> transcripts hold those. When work captured here ships, delete the
> entry. Contracts on a specific interface belong as doc comments on
> that interface.
>
> The calibration ledger that accumulated here through 2026-09-23
> (knob sweep grids, measured-dead lever shapes, per-session ship
> notes) was removed as stale; it survives at
> `git show a90ee9b6:docs/design-notes.md`. Its magnitudes predate the
> current answer key or were derived from the deleted `diagnose_loss`
> oracle — treat them as leads to re-measure, never as settled.

A living doc for cross-session design constraints, decisions, and
open questions that aren't visible from reading `src/`.

## Design philosophy

- **Make invalid states unrepresentable over validating with tests.** Prefer
  newtype wrappers, sealed enums, and constructor invariants to runtime
  checks that catch the same bugs after construction.
- **Release prefers invalid output to a panic.** Hot-path asserts are
  `debug_assert!`; release builds tolerate walker contract violations
  rather than abort. The end-of-run budget cross-check follows the same rule.
- **Plan files are ephemeral; design rationale lives in the repo.** Per-session
  plans shouldn't carry decisions that need to survive plan churn — those
  go here or in code / agent prompts.
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
  `load_ns_checked` enforces it in every fixture test.
- **The corpus has been re-frozen wholesale twice** (22d2f7b3, 2026-07-05;
  2fbbe8d4, 2026-07-28). Scores are only comparable within one key; the
  previous corpus is `git show 2fbbe8d4^:tests/north-stars/<fixture>.toml`.
- **Training vs validation tier.** Training fixtures get a full divergence
  report at `tests/divergence/<name>.md` and drive calibration; validation
  fixtures get a one-line score at `tests/validation/<name>.md` and are
  held out. The validation set is sampled to match the GitHub language
  distribution within supported languages. The point is overfit
  detection — if a change wins on training but tanks on validation, the
  rule isn't general. Enforcement is by process (the `iterate-divergence`
  skill), not the type system. The holdout also covers the validation
  fixtures' NS files (which share `tests/north-stars/` with training)
  and their source under `tests/fixtures/`; a parallel directory tree
  would not strengthen the convention, since both surfaces are equally
  readable to anyone disregarding the skill.
- **Validation debugging surface is intentionally thin.** No rendered
  snapshot or per-row diff for held-out fixtures. The
  acceptable responses to a validation move are: improve the walker
  generally against the *training* reports and re-run, or accept the
  move as a real generalization signal. Adding diagnostic artifacts
  would re-expose the surface the holdout exists to hide.
- **Growth envelope**: each batch's marginal cost must satisfy
  `cost_i ≤ 100 + 0.3 · cumulative_before`. Per-batch is too local;
  cumulative matches the author's intuition ("doubling aggregate on
  batch 2 is fine, doubling on batch 10 is bad") and doesn't force
  authors to inflate a small preceding batch to clear the path for a
  legitimately larger one later. Constants live in `src/ns_simulate.rs`.

## Scheduler prefix-monotonicity (consequence for divergence)

The scheduler stops on first ill-fit (`src/scheduler.rs` module doc has
the algorithm). **Every decision taken at budget `T_small` up to its
stopping point is also taken at `T_large`**, so `T_small`'s schedule is
a true prefix of `T_large`'s, sub-budget snapshots are slices of a
single `T_max` run, and the divergence metric runs the walker once per
fixture rather than per budget.

**Tradeoff**: when the top-ranked exact is too big, budget
under-utilization can be as much as one batch's cost. This is
deliberate pressure on walker calibration (if a top batch consistently
blocks small budgets, split it or lower its rank) and on NS authoring
(the growth envelope keeps NS prefixes coherent at small budgets).

**The one exception is an unaffordable seed.** At round 0 a seed that
doesn't fit would leave the pool permanently empty and return an empty
string. `Scheduler::schedule_partial_seed` degrades that one round to
the longest affordable prefix of the seed listing's entry rows, ordered
by `rank_seed_entries` (hidden entries last, directories before files,
all-caps root documents after other files — conventions that hold on
any repository, not a list of names), and then stops. Listings only: a
prefix of a listing is a smaller listing, where a prefix of a line
batch is severed source. The ranking is a function of the listing
alone, never of the budget, so `output(T₁) ⊆ output(T₂)` still holds;
it is consulted only on this degraded path, and a partial listing
renders a trailing `…` row so it can't be mistaken for a complete one.
Chunking the root listing in the walker instead would move scheduling
at every budget to fix a failure that only exists below the root
listing's own cost.

## NS-rank vs walker-rank

`Score(3000)` evaluates only NS rows whose **NS** cumulative tokens fall
within 3000 (`A_3K`). Pushing content earlier in the walker's schedule
does not lift the score if the matching NS row is ranked past 3K. When
the metric's view of a budget doesn't match what walker work is
feasible there, either pick walker work that matches it, or accept the
gap and surface the content at larger budgets. Don't bump `value` to
force a batch into a budget tier where it earns no `A_B` credit — it
just displaces walker batches that do.

A flat `Score(3000)` does not show a change is safe at other budgets —
read the `grid(…)` in the report headlines (`scripts/grid-means.sh`),
not a last-full-row `Score(B=cum)` proxy, which is unreliable near the
budget cliffs.

## Auto-injected docs don't belong in precis output

Files the host harness already loads into the model's context —
`AGENTS.md` / `CLAUDE.md` at any depth (Claude Code's CLAUDE.md
hierarchy is recursive), and text files under `.claude/skills/`,
`.agent/skills/`, `.cursor/rules/` — should not have their bodies
scheduled by precis. Their paths stay discoverable via fs listings, but
the prose-body batches (`MarkdownKey::Section`,
`MarkdownKey::SummaryWhole`) are walker-side suppressed. Residual
structural batches (`HeadingsOutline`, `ReadmeHeadline`) carry a 0.1×
value discount.

User framing (verbatim): "precis isn't meant to guarantee that all
content is reachable, it's meant to provide a value-per-token
summary that lets follow up tool calls do the rest."

If an NS surfaces these files' content as primary atoms, that's an
NS-author error to flag — don't move the goalpost by un-suppressing the
walker.

## Gitignored content doesn't belong in precis output either

Filtering is `fs_util::DirFilter`, built once per run and threaded
through `WalkCtx`. Decisions worth not re-litigating:

- **Gitignore rules apply only when the walk root is itself a repository
  root**, not when an ancestor is (the `ignore` crate's `require_git`
  default; what ripgrep and `fd` do). Ancestor search would make output
  depend on rules outside the summarized tree — `tests/fixtures/<f>`
  lives inside precis's own repo, so the frozen corpus would start
  reading precis's `.gitignore` and each contributor's
  `core.excludesFile`. The cost is that `precis packages/web` inside a
  monorepo gets no gitignore filtering; if that becomes a real
  complaint, the fix is a repo-root search plus an explicit escape for
  roots under the corpus, not silently widening the gate.
- **Matching is pattern-only; the index is never read.** A force-added
  tracked file matching an ignore pattern is hidden. Consulting the
  index would mean shelling out to `git ls-files` on every run.
- **The heavy-directory blocklist (`should_skip_dir`) stays.** Inside a
  repository it never fires, but precis also runs on trees that aren't
  repositories (extracted archives, vendored snapshots, the fixture
  corpus) where the filter is inert by design. `.git` is dropped
  unconditionally in `list_dir`.
- **A genuinely empty directory is not treated as ignored.** It is real
  structure; the parity test in `src/fs_util.rs` encodes it as the single
  expected difference from git.

Two gotchas in this shape:

- **A scan that starts below the walk root must ask about ancestors.** A
  directory-only pattern (`examples/`) matches the directory, not the
  files in it, so a traversal seeded inside an ignored directory (as
  Rust's Cargo source dirs are) reads the whole subtree unless it uses
  `DirFilter::excludes_tree` at its entry point.
- **A directory can be ignored by a pattern inside it** (a `.gitignore`
  holding `*`). `DirFilter::hides_everything_in` answers this
  recursively. Use git's walk (`git ls-files --others
  --exclude-standard`), not `git check-ignore`, as the oracle — the
  latter is a one-level pattern question.

## Content outside the walk root doesn't belong in it at all

precis runs on untrusted checkouts whose output is pasted into agent
contexts, so containment is a property of the listing layer and every
consumer inherits it:

- **`DirFilter` always knows its walk root**, canonical form included
  (`tests/fixtures` is a symlink). There is no rootless constructor —
  a filter with no root can express no containment.
- **A link surfaces only when it resolves inside the root**, with the
  kind of what it resolves to. Escaping and dangling links are dropped;
  in-root links (`CLAUDE.md -> AGENTS.md`) keep their rows.
- **Listing *through* a link yields nothing.** That makes the walk
  exactly the real directory tree, so link cycles are unreachable
  rather than bounded — no depth caps or visited-sets.

**Still open:** walkers that probe a *named* path directly —
`dir.join("Cargo.toml").is_file()`, `package.json`, `__init__.py`,
`mod.rs` — follow links and then read, so a checkout shipping
`Cargo.toml -> /etc/passwd` still gets that file rendered. Closing it
wants one contained-read helper adopted across the walker modules;
`SourceCache` is not the chokepoint it looks like, since many walkers
call `read_to_string` directly.

## Cross-language vs language-specific concerns

Many concerns are cross-language (value heuristics, ranking signals,
render conventions, structural priorities); only the parts that
genuinely depend on a language's grammar belong in language-specific
code. **Don't accidentally specialize cross-language code to a single
language.**

- Walker dispatch is a closed-set enum. Resist adding extension points
  unless multiple languages actually want them.
- Per-walker run state goes in named fields on `WalkCtx` (`rust_state`,
  `typescript_state`, …), not a `TypeId` bag or thread-local.

## Open items

- **Ellipsis atoms are credited on schedule content, not rendered
  output.** An NS Ellipsis atom can earn its 1-byte credit while the
  renderer emits one shared gap marker, or nothing for a blank-only
  gap. Deliberately unchanged: aligning atomization with rendered
  deltas would re-price every frozen NS mid-calibration. Revisit as a
  deliberate metric revision at the next NS re-freeze.
- **Split batches: a descendant must not emit Ellipsis records on lines
  its ancestor renders as content.** `RenderedTree::apply_spans`
  replaces ancestor-owned records unconditionally, so the paid-for row
  would demote to `…` in the render — invisible to `Score`, which
  atomizes schedule content. Head/tail splits (e.g. the Dockerfile
  split) ship their tail full-lines-only for this reason.
- **Per-row Score column can't decompose I × C**, and small walker
  tweaks cascade decimal noise through every later row. Revisit if
  iteration shows the single column loses signal.
- **Prefix-stop tail effects.** A rank shift can strand a big batch at
  the budget tail where it no longer fits; `Score(3000)` is blind to
  this — check the high budgets of the grid.
- **Min-tokens lower bound.** A cheap lower-bound cost estimator on
  `BatchContent` would let the scheduler prune obviously-too-big batches
  without touching the render tree. Line count alone misses
  predecessor-overlap savings; full marginal cost is too expensive.
  Benign at current fixture sizes.

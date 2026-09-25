# precis v0.2 — design notes

> **Agent-maintained.** Entries are notes from past sessions, not edicts;
> verify anything load-bearing against the code or with the user. This is
> for decisions not visible in `src/`, not a ledger: no session logs or
> score deltas (git history has them), and delete an entry once its work
> ships. Contracts on an interface belong in its doc comment.

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
the prose-body batches (`MarkdownKey::Prelude`, `MarkdownKey::Section`)
are walker-side suppressed. Residual
structural batches (`HeadingsOutline`, `ReadmeHeadline`) carry a 0.1×
value discount.

User framing (verbatim): "precis isn't meant to guarantee that all
content is reachable, it's meant to provide a value-per-token
summary that lets follow up tool calls do the rest."

If an NS surfaces these files' content as primary atoms, that's an
NS-author error to flag — don't move the goalpost by un-suppressing the
walker.

Peripheral admin markdown (`is_peripheral_doc`: changelogs, contributing
guides, security policies, migration guides, …) gets the same body
suppression. Unlike the auto-injected case this rests on measurement,
not policy: suppressing them (dd69d57e) left every Score(3000)
unchanged, and their bodies had been split into large candidate sets
(express's `History.md` alone into ~2500) that the schedule never bought.

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
  files in it, so a traversal seeded inside an ignored directory reads
  the whole subtree unless it uses
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
- Per-walker run state goes in named fields on `WalkCtx`
  (`json_state`, `ctx.code.<lang>`, …), not a `TypeId` bag or
  thread-local.
- Manifests (`Cargo.toml`, `pyproject.toml`, `package.json`) share one
  ontology: identity, operational (entry points / scripts / features /
  runtime constraints), runtime dependencies, appendix. Their prices live
  once in `value.rs` (`manifest_*_value`, `dependency_roster_value`);
  walkers only map their tables/keys onto those kinds. Development and
  peer rosters are not emitted. A workspace's primary member is the one
  member directory named after the repository, for Cargo and JS alike.
- Files no parser claims go through the plaintext walker: named classes
  (build entrypoints incl. compose files, dotenv samples, tooling config
  incl. CI YAML, …) render whole or as a head slice at one of four value
  tiers; everything else falls to the column-0 declaration surface.

## Output notation and the plugin cap

- **Score is a poor judge of row formatting.** Re-pricing rows
  re-prices the NS too, and the corpus Score curve falls above 3000
  tokens, so a format that fits more content per token scores roughly
  as if the budget had grown: putting source rows at column 0 (about
  14% fewer tokens) measured −0.027 at 3000 while showing more. Open:
  whether to take that trade.
- **o200k charges for leading spaces only in steps:** a run of two or
  more spaces before a digit costs two tokens whatever its length, and
  ` …\n` costs the same one token as `\n`. Indent width is therefore a
  character cost, not a token cost.
- **Line numbers are not padded.** The same step pricing made
  right-aligning `N→` free in tokens, so it was dropped for the plugin
  cap's characters. The cost is that the `→` column shifts by one at
  9/10 and 99/100; indentation stays readable relative to `→`.
- **The plugin cap is in UTF-16 code units.** Claude Code keeps a hook's
  `additionalContext` inline only up to 10,000 JavaScript string units
  and otherwise replaces it with a file path and a preview, so the
  default `--char-budget` is derived at runtime from the rendered help
  and the hook's wrapper text (`src/main.rs`).

## Code engine (`walker::code`)

Source languages share one declaration ladder: a language module's
`extract` returns a `FileModel` (contract in `walker/code/model.rs`),
and the engine alone builds batches from it. Decisions a new port
must not undo:

- **Five rungs:** `ModuleDoc`, `Names`, `Decl`, `Doc`, `Body`. A
  `CodeKey` is identified by `(rung, file, decl index, chunk)`, never
  by source line: a container and its first member can share a row.
- **Engine-side normalization** (sort, dedup, blank-row drop,
  `module_doc` strip, same-first-row merge, trim at the next sibling,
  part disjointness) so `extract` can list rows loosely. The
  next-sibling trim is load-bearing: without it a node that spills into
  the next declaration claims its roster row and becomes its
  predecessor.
- **Ownership ledger, not assertions:** a row claimed outside the
  claiming batch's predecessor chain is dropped and counted (asserted
  zero in unit tests), so one extraction quirk loses a row instead of
  failing a fixture run. A batch whose rows its ancestors already
  render is skipped, and its descendants gate on the ancestor.
- **Full-line spans only:** the engine emits no Ellipsis records.
- **One value table:** `value::code_rung_value` per rung and one chunk
  exponent (`DEFAULT_CONCAVITY_EXPONENT`). `Names` was designed at or
  above `Decl` (breadth first); the corpus grid put it well below
  (1150 → 750: +0.016 at 3000, +0.022 over the 7 budgets), because a
  roster in every file then outranked entry-file declarations, docs
  and manifests. A roster is priced per entry (`entries^k`, no floor
  or cap), so its ratio is its tokens per entry: small files' 1–3 row
  rosters no longer win on size alone, and a long public roster is
  not capped below its short peers (+0.006 at 3000 over the capped
  `roster_mass_factor`). Per-language pricing enters only through
  `is_entrypoint` (a depth pin, no extra factor), `file_weight` and
  what `extract` hides. Once rosters were priced per entry, an
  entry-file factor, a private-declaration factor, a member factor, a
  roster head premium and a chunk tail decay each measured neutral or
  negative on the grid and were removed (2026-09-25); re-adding one
  needs a fresh measurement.

## Threads

- **Only parsing is parallel.** `WalkCtx::parse_trees` parses a
  directory's code and markdown files (and each TypeScript re-export
  level) on scoped worker threads into the tree cache; everything that
  decides output runs on the main thread in walk order. Output is
  independent of thread timing only while a parse stays a pure function
  of the file.
- **Tokenizing on workers doesn't pay** (measured 2026-09-25).
  Pre-counting every line of a directory's code files cost about 4× the
  main-thread counting it replaced; pre-counting exactly the model's
  item rows cut wall time 2–10% on the slowest repos but added 70–150 ms
  of CPU per run: the same lines took about twice the CPU on workers as
  on the main thread.
- **The o200k table build (~55 ms) is fixed per run** and dominates
  small repos. It happens inside `tiktoken_rs::o200k_base()`, so only
  replacing the tokenizer would shrink it; the scheduler starts it on a
  background thread and runs the essential-source scan meanwhile, unless
  the seed listing's approximate cost already exceeds the budget (then
  no source batch is ever absorbed and the scan would be wasted I/O).

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
  atomizes schedule content. Head/tail splits (e.g. the Prisma
  wide model split) ship their tail full-lines-only for this reason.
- **Per-row Score column can't decompose I × C**, and small walker
  tweaks cascade decimal noise through every later row. Revisit if
  iteration shows the single column loses signal.
- **Prefix-stop tail effects.** A rank shift can strand a big batch at
  the budget tail where it no longer fits; `Score(3000)` is blind to
  this — check the high budgets of the grid.
- **The plugin can show less than `Score(3000)` measures.** Under
  `CLAUDE_PLUGIN_ROOT`, `precis .` is capped at `plugin_char_budget()`
  (about 9,330 UTF-16 units after `--help` and the hook wrapper), and
  31 of the 71 training fixtures' 3000-token outputs (`tests/rendered/`)
  exceed it (measured 2026-09-25), so on those the auto-injected
  summary is a shorter prefix than the primary budget scores.
- **Min-tokens lower bound.** A cheap lower-bound cost estimator on
  `BatchContent` would let the scheduler prune obviously-too-big batches
  without touching the render tree. Line count alone misses
  predecessor-overlap savings; full marginal cost is too expensive.
  Benign at current fixture sizes.

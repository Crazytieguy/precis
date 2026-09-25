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
the algorithm), after spending what is left on the longest affordable
prefix of that batch's entries or lines. **Every decision taken at
budget `T_small` is also taken at `T_large`**, so `T_small`'s output is
a subset of `T_large`'s, sub-budget outputs are replays of a single
`T_max` schedule plus the head of the batch it stops on, and the
divergence metric runs the walker once per fixture rather than per
budget. Skipping an ill-fitting batch for a smaller one would break
this.

## Auto-injected docs don't belong in precis output

Files the host harness already loads into the model's context —
`AGENTS.md` / `CLAUDE.md` at any depth (Claude Code's CLAUDE.md
hierarchy is recursive), and text files under `.claude/skills/`,
`.agent/skills/`, `.cursor/rules/` — should not have their bodies
scheduled by precis. Their paths stay discoverable via fs listings, but
the prose-body batches (`MarkdownKey::Prelude`, `MarkdownKey::Section`)
are walker-side suppressed. Residual
structural batches (`HeadingsOutline`, `ReadmeHeadline`) carry a 0.1×
value discount. precis is a value-per-token summary for follow-up tool
calls to build on, not a guarantee that all content is reachable.

If an NS surfaces these files' content as primary atoms, that's an
NS-author error to flag — don't move the goalpost by un-suppressing the
walker.

Peripheral admin markdown (`is_peripheral_doc`: changelogs, contributing
guides, security policies, migration guides, …) gets the same body
suppression, on measurement rather than policy: the schedule never
bought those bodies.

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
- **A named file walks its directory with only that file admitted**
  (`DirFilter::single_file`), so it gets every walker unchanged. If it
  is a link, it must resolve inside the directory it was named in —
  the rule a listing applies to a linked entry.

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

## Output notation and the plugin cap

- **Score is a poor judge of row formatting.** Re-pricing rows
  re-prices the NS too, and the corpus Score curve falls above 3000
  tokens, so a format that fits more content per token scores roughly
  as if the budget had grown: putting source rows at column 0 (about
  14% fewer tokens) measured −0.027 at 3000 while showing more.
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

- A `CodeKey` is identified by `(rung, file, decl index, chunk)`, never
  by source line: a container and its first member can share a row.
- **Ownership ledger, not assertions:** a row claimed outside the
  claiming batch's predecessor chain is dropped and counted, so one
  extraction quirk loses a row instead of failing a run. A batch whose
  rows its ancestors already render is skipped, and its descendants
  gate on the ancestor. The engine emits no Ellipsis records.
- **One value table:** `value::code_rung_value` per rung and one chunk
  exponent. `Names` sits well below `Decl`: a roster in every file
  otherwise outranks entry-file declarations, docs and manifests. A
  roster is priced per entry (`entries^k`), so its ratio is its tokens
  per entry. Per-language pricing enters only through `is_entrypoint`
  (a depth pin), `file_weight` and what `extract` hides.
- **Removed after measuring neutral or better on the grid
  (2026-09-25); re-adding one needs a fresh measurement:** entry-file,
  private-declaration and member factors, a roster head premium, a
  chunk tail decay; the TS/JS module doc, JSDoc paragraph splits,
  unexported declarations in TS/JS entry files, publishing a published
  handle's factory; Rust badge-paragraph skipping, rustdoc fence-aware
  paragraphs, hiding impls of hidden types; C decoration-row paragraph
  splits.

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
- **Per-row Score column can't decompose I × C**, and small walker
  tweaks cascade decimal noise through every later row. Revisit if
  iteration shows the single column loses signal.
- **Min-tokens lower bound.** A cheap lower-bound cost estimator on
  `BatchContent` would let the scheduler prune obviously-too-big batches
  without touching the render tree. Line count alone misses
  predecessor-overlap savings; full marginal cost is too expensive.
  Benign at current fixture sizes.

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
- **Plan files are ephemeral; design rationale lives in the repo.**
- **Simplify over validate.** When a feature would need extra validation,
  consider whether collapsing the design eliminates the need.
- **Output is a verbatim subset of the source.** No paraphrasing,
  summarization, or invented content under any circumstances. Allowed
  transforms (full lines, prefix+ellipsis truncation, bare-ellipsis
  markers) are documented on `Render` in `src/content.rs`.

## North Star process

- North Star documents (`tests/north-stars/<fixture>.toml`) are agent-drafted,
  human-reviewed, and **frozen** before implementation iterates against them.
  Implementation changes do not edit them. Scores are only comparable within
  one freeze of the corpus.
- `revision_pin` is the only thing binding an NS to its fixture revision.
  `load_ns_checked` enforces it in every fixture test.
- **Training vs validation tier.** Training fixtures get a full divergence
  report at `tests/divergence/<name>.md` and drive calibration; validation
  fixtures get a one-line score at `tests/validation/<name>.md` and are
  held out for overfit detection. The holdout (their NS files and sources
  too) is enforced by process — the `iterate-divergence` skill — and the
  validation surface stays thin on purpose: no rendered snapshot or
  per-row diff, since those would re-expose what the holdout hides.
- **Growth envelope**: each NS batch's marginal cost must satisfy
  `cost_i ≤ 100 + 0.3 · cumulative_before` (constants in
  `src/ns_simulate.rs`), so NS prefixes stay coherent at small budgets
  without forcing authors to inflate an early batch.

## Scheduler prefix-monotonicity

The scheduler stops on the first top-ranked batch that doesn't fit, after
spending what is left on the longest affordable prefix of that batch's
entries or lines (`src/scheduler.rs`). **Every decision taken at budget
`T_small` is also taken at `T_large`**, so `T_small`'s output is a subset
of `T_large`'s, and the divergence metric replays one `T_max` schedule
per fixture instead of running the walker per budget. Skipping an
ill-fitting batch for a smaller one would break this.

## Auto-injected docs don't belong in precis output

Files the host harness already loads into the model's context —
`CLAUDE.md` at any depth, `AGENTS.md` where Claude Code loads it (a
directory's AGENTS.md only when it has no CLAUDE.md, or its CLAUDE.md is
the same text or imports `@AGENTS.md`), and text files under
`.claude/skills/`, `.agent/skills/`, `.cursor/rules/` — keep their
listing rows, but their prose bodies are never scheduled and their
structural batches carry a 0.1× discount. An NS that ranks their content
as primary is an NS-author error, not a reason to un-suppress.
Peripheral admin markdown (`is_peripheral_doc`) gets the same body
suppression because the schedule never bought those bodies anyway.

## Gitignored content doesn't belong in precis output either

- **Gitignore rules apply only when the walk root is itself a repository
  root** (the `ignore` crate's `require_git` default; what ripgrep and
  `fd` do). Ancestor search would make output depend on rules outside
  the summarized tree — the corpus lives inside precis's own repo. The
  cost is that `precis packages/web` inside a monorepo gets no gitignore
  filtering.
- **The corpus runs with the filter inert**: `clone_fixtures` strips
  `.git`, so gitignore handling is covered only by unit tests, and the
  heavy-directory blocklist (`should_skip_dir`) is what bounds traversal
  on every fixture and on any tree that isn't a repository.
- **Matching is pattern-only; the index is never read**, so a force-added
  file matching an ignore pattern is hidden.
- **A directory can be ignored by a pattern inside it** (a `.gitignore`
  holding `*`); a genuinely empty directory is not. The parity test in
  `src/fs_util.rs` uses git's walk (`git ls-files --others
  --exclude-standard`) as the oracle, not `git check-ignore`, which only
  answers a one-level pattern question.

## Content outside the walk root doesn't belong in it at all

precis runs on untrusted checkouts whose output is pasted into agent
contexts, so containment is a property of the listing layer and every
consumer inherits it (`fs_util::list_dir`, `resolved_kind`): a link
surfaces only when it resolves inside the root, listing through a link
yields nothing (so link cycles are unreachable), and a named file walks
its directory with only that file admitted. Content batches come from
link-rejecting enumeration. The exception is workspace-membership
parsing, which reads `package.json`, `pnpm-workspace.yaml` and
`Cargo.toml` by name and follows links; it parses member lists and never
renders the text.

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
  re-prices the NS too, so a denser format scores roughly as if the
  budget had changed: source rows at column 0 (about 14% fewer tokens)
  measured −0.027 at 3000, and lost a pairwise A/B eval on fresh repos,
  where judges found flush-left excerpts hard to attach to their files.
  Rows stay nested under their file.
- **o200k charges for leading spaces only in steps:** a run of two or
  more spaces before a digit costs two tokens whatever its length, and
  ` …\n` costs the same one token as `\n`. Indent width is therefore a
  character cost, not a token cost — which is also why line numbers are
  not right-aligned.
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
  directory's files on scoped worker threads into the tree cache;
  everything that decides output runs on the main thread in walk order,
  so output is independent of thread timing only while a parse stays a
  pure function of the file.
- **Tokenizing on workers doesn't pay:** the same lines cost about
  twice the CPU on workers as on the main thread.
- **The o200k table build (~55 ms) is fixed per run.** The scheduler
  starts it on a background thread and runs the essential-source scan
  meanwhile, unless the seed listing's approximate cost already exceeds
  the budget.

## Open items

- **Ellipsis atoms are credited on schedule content, not rendered
  output.** An NS Ellipsis atom can earn its 1-byte credit while the
  renderer emits one shared gap marker, or nothing for a blank-only
  gap. Aligning atomization with rendered deltas would re-price every
  frozen NS; revisit at the next re-freeze.
- **The answer key favours complete listings more than eval judges do.**
  Judges on fresh repos most often fault budget spent on inventories of
  test files, media and CI directories, but every listing demotion tried
  (splitting co-located tests and media into a discounted follow-up
  listing, a media-share discount, a steeper listing cost exponent)
  lost on the grid, because NS authors rank those listings early.
  Moving that trade needs an answer-key revision, not a walker tweak.

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

## Only the root README is read as a document

The root README is the only document the markdown walker reads (one
file: Markdown, then reST, then AsciiDoc; a single named file reads as
one); every other `.md` is a listing row.
That includes the files the host harness already loads into the model's
context — `CLAUDE.md`, `AGENTS.md`, skill and rules files — so they need
no special case: an NS that ranks their content as primary is an
NS-author error. Pricing other docs' sections, then their outlines, at
any tier measured flat to negative on the grid (2026-09-25): the
schedule bought the wrong pages (translations, migration guides, nested
package READMEs) and never reached the ones NS ranks.

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
  runtime constraints), runtime dependencies. Their prices live once in
  `value.rs` (`manifest_*_value`, `dependency_roster_value`); walkers
  only map their tables/keys onto those kinds. Development and peer
  rosters, author/URL metadata and tool config are not emitted. A
  workspace's primary member is the one member directory named after the
  repository, for Cargo and JS alike.
- License texts and CI workflow YAML render no content; their listing
  rows name them.
- Whether precis has a walker for a language must not decide which part
  of a repository wins. The plaintext fallback's declaration surface
  prices like a parsed declaration (`code_rung_value(Decl)`) when its
  file is in the tree's primary language (most essential source bytes),
  so a core written in an unparsed language (a Zig database with a Rust
  client) is not crowded out by side clients the code engine parses.
  Every other fallback surface prices below a parsed declaration.
  The language families (`language_group`) cover the fallback's
  languages too, and the dominant-file rule reads the same primary
  language, so a repository whose most essential bytes are in `.vue`
  or `.php` has its spine file chosen from that language, not from
  the parsed JS beside it.

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
  per entry. An entry is a roster row, not a statement or declaration:
  a multi-line `export { … }` or `from x import (…)` lists one name per
  row, and a Go grouped declaration one per spec. Pricing a
  re-export-only roster at the `Decl` tier measured worse (2026-09-25):
  barrels then outrank root listings at small budgets. Per-language pricing enters only through `is_entrypoint`
  (a depth pin), `file_weight` and what `extract` hides.
- **Removed after measuring neutral or better on the grid
  (2026-09-25); re-adding one needs a fresh measurement:** entry-file,
  private-declaration and member factors, a roster head premium, a
  chunk tail decay; the TS/JS module doc, JSDoc paragraph splits,
  unexported declarations in TS/JS entry files, publishing a published
  handle's factory; Rust badge-paragraph skipping, rustdoc fence-aware
  paragraphs, hiding impls of hidden types; C decoration-row paragraph
  splits; the C banner cutoff on declaration docs.
- **Rows that condition or define a file's exports join its roster as
  re-export rows**: a Go `//go:build` constraint (otherwise platform
  variants list the same declarations with no condition), and a Lua
  module's top-level `return` and `setmetatable(…)` call.
- **Front doors and program flow are extraction decisions.** A file
  named after its project (a TS package's `lib/<package>.js`, a C
  repository's `<repo>.h`) is an entrypoint: the depth pin only;
  front-door file weights on top measured flat or negative (2026-09-25).
  For Go, a root library file named after its package at ×1.3–1.4
  measured +0.0014–0.0017 at 3000, under the bar: a helper file named
  after its package opens ahead of the central one. Weighting the file
  that holds the package comment instead measured negative.
  A program's `main`, and the file's other functions when `main` has at
  most two top-level statements, are `Whole` declarations, so their
  bodies price at the `Decl` tier. A TS/JS entry script that exports
  nothing declares its top-level control-flow statements the same way
  (`if (…) module.exports = require(…)`, a bootstrap promise chain).
  Emitting them as the file's `ModuleDoc` instead measured better at
  small budgets but worse at 3000: every sample app's entry script then
  outranked library code (2026-09-25).
- **Python's central modules are found through package `__init__`
  imports.** A module an enclosing `__init__.py` imports names from gets
  a ×1.2 file weight. Signals that rank every batch of a big module
  higher lost badly at 3000 (2026-09-25): a line-count file weight
  (−0.018), counting class members as `Names` entries (−0.020), listing
  members in the file roster (−0.002). They buy the big module's
  declaration dive, not its roster.

## Threads

- **Only parsing is parallel.** `WalkCtx::parse_each` parses a
  directory's files on one scoped worker per core and hands the trees
  back in file order; everything that decides output runs on the main thread in walk order,
  so output is independent of thread timing only while a parse stays a
  pure function of the file.
- **Tokenizing on workers doesn't pay:** the same lines cost about
  twice the CPU on workers as on the main thread.
- **The o200k table build (~55 ms) is fixed per run.** The scheduler
  starts it on a background thread and runs the essential-source scan
  meanwhile, unless the seed listing's approximate cost already exceeds
  the budget.

## Resource bounds

Measured on the 186-repo robustness sweep (`ignore/robust/`), not the
corpus, which none of these bounds touch.

- **Parse trees live only while their directory expands.** A tree is
  several times its source's size and nothing reads one after its
  file's batches are built, so none is cached. A file starts parsing
  only while the not-yet-visited parses' sources total at most
  `PARSE_BYTE_CAP` (one file always may), so the trees alive at once are
  bounded by bytes on any core count, not by a directory's size. Source
  text stays cached: rendering and cost probes read it.
- **Nothing over 8 MiB is parsed** (`PARSE_BYTE_CAP`): such a file yields
  no batches, like a generated or minified one. The sweep's largest
  hand-written single-file library is `miniaudio.h` at 4.1 MB; a 25.9 MB
  generated `parser.c` cost 900 MB and seconds.
- **Whole-tree probes read at most 20k entries each**
  (`PROBE_ENTRY_CAP`): the spine survey and the source-inventory probes
  look below what the summary shows, and on a tree with little source
  (a home directory, `~/projects`) nothing else bounded them; a non-git
  `~/projects` took 49 s. The budget is per probe rather than per run so
  a directory's inventory answer depends only on that directory, not on
  what was probed before it; an answer the budget cut short is not cached.
  The spine survey skips non-essential directories. It reads on past
  the point where no file could be the spine, because the primary
  language it also measures needs the whole tree's byte mass; that costs
  under 0.1 s on the sweep's largest trees. `hides_everything_in` is deliberately uncapped: its recursion
  only descends through directories with nothing visible, and a capped
  answer lists every such directory as `(empty)` rows.

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
- **Unopened rosters rank by row length, not centrality.** A roster's
  ratio is `350 · prior / (tokens per row)^k`, so a directory's
  rosters sit in a narrow band (about 115–145 in a Go root) and open
  shortest-rows first; a central file whose rows are long method
  signatures (`func (engine *Engine) …`) opens last, and once any
  roster opens, that file's `Decl` and `Doc` batches (ratio 200–400)
  drain before the next roster. Per-language file weights and hiding
  rules in `extract` measured flat or split by fixture (2026-09-25); a
  fix belongs in the value model.

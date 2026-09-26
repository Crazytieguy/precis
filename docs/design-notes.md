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
- **Training vs validation tier.** Training fixtures get a full divergence
  report and drive calibration; validation fixtures get a one-line score
  and are held out for overfit detection. The holdout (their NS files and
  sources too) is enforced by process — the `iterate-divergence` skill —
  and the validation surface stays thin on purpose: a rendered snapshot
  or per-row diff would re-expose what the holdout hides.

## The subset property

The scheduler stops on the first top-ranked batch that doesn't fit, after
spending what is left on the longest affordable prefix of that batch
(`src/scheduler.rs`): its first listing entries, its first source rows, or,
for a code batch, its first items taken whole, so the cut never lands
inside a signature, statement or field. Every decision taken at a smaller
token budget is also taken at a larger one, so the smaller output is a
subset of the larger, and the divergence metric replays one schedule per
fixture instead of running the walker per budget. Skipping an ill-fitting batch
for a smaller one would break this.

Under a char budget, batches rank on the budget they draw down faster,
because the plugin's char cap binds before its token budget on large
repos and token-only ranking underpriced deep indentation and long
listings exactly there. The subset property therefore holds across runs
that share one chars-per-token rate, not across runs that vary only one
of the two budgets. The grid runs uncapped; this was judged on
plugin-mode output of external repos.

## Only the root README is read as a document

The markdown walker reads one document: the root README (Markdown, then
reST, then AsciiDoc), or a single named file. Every other `.md` is a
listing row, including the files the host harness already loads
(`CLAUDE.md`, `AGENTS.md`, skills, rules), so they need no special
case: an NS that ranks their content as primary is an NS-author error.
Pricing other docs' sections or outlines at any tier bought the wrong
pages (translations, migration guides, nested package READMEs) and
never reached the ones answer keys rank.
A repository that exhausts the pool before the budget gets one more
round: the head of every listed file no batch touches
(`plaintext::floor_batches`), ranked among themselves only, so it never
displaces a priced batch.

## Gitignore and containment

- **The corpus runs with the gitignore filter inert**: `clone_fixtures`
  strips `.git`, so gitignore handling is covered only by unit tests
  (their oracle is git's walk, `git ls-files --others
  --exclude-standard`), and the heavy-directory blocklist
  (`should_skip_dir`) is what bounds traversal on every fixture and on
  any tree that isn't a repository.
- **Matching is pattern-only; the index is never read**, so a force-added
  file matching an ignore pattern is hidden.
- **Nothing outside the walk root surfaces.** precis runs on untrusted
  checkouts whose output is pasted into agent contexts, so containment
  belongs to the listing layer (`fs_util::list_dir`, `resolved_kind`)
  and every consumer inherits it. The one exception is
  workspace-membership parsing, which follows links to read member
  lists and never renders the text.

## Cross-language vs language-specific concerns

Value heuristics, ranking signals and render conventions are
cross-language; only what depends on a grammar belongs in
language-specific code. **Don't accidentally specialize cross-language
code to a single language.**

- Walker dispatch is a closed-set enum. Per-walker run state goes in
  named fields on `WalkCtx`, not a `TypeId` bag or thread-local.
- Manifests (`Cargo.toml`, `pyproject.toml`, `package.json`) share one
  ontology — identity, operational, runtime dependencies — priced once
  in `value.rs`; walkers only map their tables and keys onto it.
  Development and peer rosters, author/URL metadata and tool config are
  not emitted. A workspace's primary member is the member directory
  named after the repository, for Cargo and JS alike.
- A root manifest in any other format (Maven, Composer, Cabal, sbt,
  CMake, MSBuild, GitHub Actions, …) gets no walker of its own: the
  plaintext fallback renders its flat surface, which in these formats is
  the identity block, at the build-file tier. Nested module manifests
  are left to the listing, since they repeat the root's identity.
- Whether precis has a parser for a language must not decide which part
  of a repository wins. The plaintext fallback's declaration surface
  prices like a parsed declaration when its file is in the tree's
  primary language (most essential source bytes), and the dominant-file
  rule reads the same primary language, so a core written in an
  unparsed language is not crowded out by side clients the code engine
  parses.

## Output notation

- **Score is a poor judge of row formatting.** Re-pricing rows
  re-prices the NS too, so a denser format scores roughly as if the
  budget had changed. Source rows at column 0 lost on the grid and in a
  pairwise A/B eval on fresh repos, where judges found flush-left
  excerpts hard to attach to their files. Rows stay nested under their
  file.
- **o200k charges for leading spaces only in steps:** a run of two or
  more spaces before a digit costs two tokens whatever its length, and
  ` …\n` costs the same one token as `\n`. Indent width is therefore a
  character cost, not a token cost — which is also why line numbers are
  not right-aligned.

## Code engine (`walker::code`)

Source languages share one declaration ladder: a language module's
`extract` returns a `FileModel` (contract in `walker/code/model.rs`),
and the engine alone builds batches from it. Decisions a new port
must not undo:

- A `CodeKey` is identified by `(rung, file, decl index, chunk)`, never
  by source line (a container and its first member can share a row), and
  row ownership is a ledger that drops a conflicting row, not an
  assertion that fails the run. The engine emits no Ellipsis records.
- **One value table:** `value::code_rung_value` per rung and one chunk
  exponent. `Names` sits well below `Decl`, or a roster in every file
  outranks entry-file declarations, docs and manifests. A roster is
  priced per entry, and an entry is a roster row. A `Whole`
  declaration's `Decl` is a roster too (a struct's fields, a class's
  member names): it scales by `roster_mass` of its body entries, or a
  large class ranks below its own members' docs. A re-export-only
  roster stays at the `Names` tier, or barrels outrank root listings at
  small budgets.
- **Per-language pricing enters only through `is_entrypoint`** (a depth
  pin), `file_weight` and what `extract` hides. Entry-file,
  private-declaration, member and front-door file factors, a roster head
  premium and a chunk tail decay were each removed at flat or better
  score, as were extraction refinements; re-adding one needs a fresh
  measurement:
  - the TS/JS module doc and JSDoc paragraph splits;
  - unexported declarations in TS/JS entry files;
  - Rust badge-paragraph skipping and rustdoc fence-aware paragraphs;
  - hiding impls of hidden types;
  - C decoration-row paragraph splits and the C banner cutoff on
    declaration docs.
- **Rows that condition or define a file's exports join its roster**:
  a Go `//go:build` constraint, a Lua module's top-level `return` and
  `setmetatable(…)` call.
- **Program flow is an extraction decision.** A program's `main` (and
  the file's other functions when `main` has at most two top-level
  statements) and an export-free TS/JS entry script's top-level control
  flow are `Whole` declarations, priced at the `Decl` tier. As a
  `ModuleDoc` they outranked library code at the default budget.
- **Signals that rank every batch of a big module higher** (line-count
  weights, class members as roster entries) buy the module's
  declaration dive, not its roster, and lose.
- **A directory's rosters open in a chain.** A roster's ratio is
  `value / (tokens per row)^k`, so unchained, a directory's rosters sit
  in a narrow band and open shortest-rows first, whatever each file's
  role. Each non-entry file's `Names` head chunk is instead gated on the
  previous file's: most sibling references first (Python imports, Go
  names another file of the package declares), then largest, with
  non-essential files last. Values are untouched: every value-model fix
  tried (a prior for sibling mentions, a steeper roster exponent, a
  depth pin beside entry files) lifted code over README and manifest
  batches and lost. Reference signals for C `#include`, Rust
  `crate::`/`mod` and TS/JS relative imports ranked no better than size
  (C's most-included headers are utility headers) and were dropped.

## Threads and resource bounds

- **Only parsing is parallel.** Everything that decides output runs on
  the main thread in walk order, so output is independent of thread
  timing only while a parse stays a pure function of the file.
  Tokenizing on workers costs about twice the CPU and doesn't pay.
- **The probe budget is per probe, not per run**, so a directory's
  inventory answer depends only on that directory; an answer the budget
  cut short is not cached. `hides_everything_in` is uncapped on purpose:
  it only descends through directories with nothing visible, and a
  capped answer would list each of them as `(empty)`.

## Open items

- **Ellipsis atoms are credited on schedule content, not rendered
  output.** An NS Ellipsis atom can earn its 1-byte credit while the
  renderer emits one shared gap marker, or nothing for a blank-only
  gap. Aligning atomization with rendered deltas would re-price every
  frozen NS; revisit at the next re-freeze.
- **The answer key favours complete listings more than eval judges do.**
  Judges on fresh repos most often fault budget spent on inventories of
  test files, media and CI directories, but every listing demotion tried
  lost on the grid because NS authors rank those listings early.
  Moving that trade needs an answer-key revision, not a walker tweak.
  Listings are never split: a head-and-rest split of long listings
  was grid-neutral and spent real-world budget on the first 40 names
  of man-page, test-data and generated-code directories.

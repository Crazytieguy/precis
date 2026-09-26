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
  markers) are documented on `Render` in `src/content.rs`. Walkers emit
  `Full` lines only; `Truncated` and `Ellipsis` serve the North Stars.

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
inside a signature, statement or field. The exception is an item too
large to chunk (a function body that is one long `match`, a constant
table): it is split into rows, since whole it showed nothing below the
budget that held all of it. Every decision taken at a smaller
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

- **Root build and contributing guides** (`BUILDING.md`, `INSTALL.md`,
  `CONTRIBUTING.md`, …), the documents a README sends builders to,
  yield one `CommandBlock` each and nothing else, taken only under a
  command-titled heading: a guide's first block is as often a commit
  template, a fork's clone or a package-manager install as a build
  step. The grid barely sees guides, so they were judged on real-world
  output. `AGENTS.md` stays out: Claude Code loads it in place of a
  missing `CLAUDE.md`, and in all 26 repositories of the eval,
  robustness and breadth corpora that have both, one is a symlink to,
  an `@`-import of or a copy of the other. Eval answerers see neither
  file, so judges asking for `AGENTS.md` content measure the eval, not
  the product.
- **Only sections titled for building, testing, running or development
  sell a `CommandBlock`**: their leading shell blocks, one per top-level
  section, behind the outline, with the section gated on it (the
  scheduler allows overlap only with ancestors). Install and setup
  titles stay out: they buy library `npm install x` blocks, dev-server
  commands and benchmark configs that answer keys rank late.
- **Back matter** (license, contributing, sponsors, …) emits no section
  text but keeps a command-titled subsection's block, since a
  Contributing section's Testing is the dev workflow. The outline keeps
  back-matter headings: answer keys read it as the README's table of
  contents, `Star History` included.
- **Chrome** (badges, logos, rules, link definitions, nav menus, and
  tables of contents: lists whose items mostly open with an in-document
  link) is left out wherever it sits, section bodies included. Catalog
  READMEs lose their category contents list with it.
- **A document batch cut short by the budget never leaves a fence or
  `<pre>` open**: a verbatim block's opening and closing rows are one
  unit, taken before its body. This covers README sections and the
  head of any other Markdown file (`plaintext::floor_batches`). Taking the block whole instead cost the
  grid .003 at 1000 tokens, because the partial then lost the head of
  the example.
- A repository that exhausts the pool before the budget gets one more
  round: the head of every listed file no batch touches
  (`plaintext::floor_batches`), ranked among themselves only, so it
  never displaces a priced batch.

## Gitignore and containment

- **The corpus runs with the gitignore filter inert**: `clone_fixtures`
  strips `.git`, so gitignore handling is covered only by unit tests
  (their oracle is git's walk, `git ls-files --others
  --exclude-standard`), and the heavy-directory blocklist
  (`should_skip_dir`) is what bounds traversal on every fixture and on
  any tree that isn't a repository.
- **Matching is pattern-only; the index is never read**, so a force-added
  file matching an ignore pattern is hidden.
- **Nothing outside the walk root, and nothing it hides, surfaces.**
  precis runs unattended on untrusted checkouts whose output is pasted
  into agent contexts, so containment belongs to the listing layer
  (`fs_util::list_dir`, `resolved_kind`) and every consumer inherits
  it. A link lists only when its resolved target is inside the root and
  nothing on the way down to it is internal or ignored; only regular
  files and directories list; a non-UTF-8 name doesn't list, since
  consumers reopen entries by their listed name. A link to a file in its
  own directory (`CLAUDE.md -> AGENTS.md`) lists but is never read
  through (`walker::is_refused_by_name`): its text shows under the
  target's row, not twice. A link to a file elsewhere in the tree
  (`readme.md -> packages/next/README.md`) still reads, since the
  target's row may be deep below anything the budget reaches.
- **Every content read is admitted and bounded.** `WalkCtx::read_source`
  reads a path only when its directory's listing admits it as a file,
  which holds the manifests workspace discovery and Python's
  `__init__.py` ancestry open by fixed name to the listing's rules
  (refused reads as no declaration). `SourceCache::get` then reads only
  a regular file of at most `MAX_SOURCE_BYTES` with no NUL byte,
  decoding bytes that aren't UTF-8 as U+FFFD. Everything the cache
  holds, text and line index, is charged to one run-wide
  `SOURCE_CACHE_BYTE_CAP`, reads (`get`) and handed-in heads (`insert`)
  alike: the parse cap alone left the reads walkers make without
  parsing (the C++-header probe, fallback-language and prose files)
  unbounded. The spine survey reads its
  candidates through `read_source` too; the floor's head read
  (`plaintext::file_head`) has its own capped reader but sees only
  listed files. Workspace membership
  canonicalizes member manifest paths without reading them, and the TS
  engine's nearest-`package.json` probe is a stat, which follows links
  but reads nothing.
- **Credential files never render, whichever walker reads them.**
  `walker::is_refused` — a credential file name (of the path or its link
  target), or a PEM/PGP private-key block with key material under its
  armor — is applied by `SourceCache` to everything it reads or is
  handed, so a refused file lists by name only — a single-file walk of
  one included, since naming the file doesn't make its secrets safe to
  paste; no walker carries a check of its own, except that the floor,
  which reads only a file's head, also refuses a head cut off inside a
  private-key armor (`walker::head_holds_private_key`). The name rule exempts samples (`*.example`,
  `*.sample`, `*.template`, `*.dist`), source code and documents, which
  are about credentials rather than holding them. A URL's password
  (`scheme://user:PASSWORD@host`) and the quoted literal of a
  credential-named key (`password: "…"`, `API_KEY = "…"`) are redacted
  where rows are formatted (`render::redact_secrets`), so it holds for
  every walker and the floor, and for costing as well as output. Only
  a quoted, space-free literal with a letter, or of digits alone,
  counts: an unquoted value can't be told from a variable or a type by
  the line alone. Punctuation doesn't exempt a literal (generated
  passwords hold `$`, `%`, brackets); only a placeholder's whole shape
  or a whole interpolation inside it (`${…}`, `{name}`, `{{ … }}`, `%s`,
  `env(…)`, `<…>`) does. Documents (`.md`, `.mdx`, `.rst`, `.adoc`) keep their
  literals, which are placeholders (`API_KEY='your-key'`). Other
  secrets in ordinarily named config are not detected.

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
  named after the repository, for Cargo and JS alike. A private root
  `package.json`'s entry-point scripts (build/test/lint/…) price at its
  identity value, since that identity is nearly empty; pricing every
  root's scripts that way lost 0.001–0.0025 at 1000–1442.
- A Makefile or justfile short enough to render whole keeps every
  recipe body but its housekeeping ones' (release, dist, install, clean,
  …). Keeping only build/test/run bodies lost 0.001 at 3000: NS keys buy
  dev-workflow recipes with arbitrary names (init, serve, e2e), and
  format recipes too.
- A root manifest in any other format (Maven, Composer, Cabal, sbt,
  CMake, GitHub Actions, …) gets no walker of its own: the
  plaintext fallback renders its flat surface, which in these formats is
  the identity block, priced as one. Besides the root, only
  `src/` and the directory named after the repository count as the
  project's own; other nested module manifests are left to the listing,
  since they repeat the root's identity.
- Whether precis has a parser for a language must not decide which part
  of a repository wins. The plaintext fallback's declaration surface
  prices like a parsed declaration when its file is in the tree's
  primary language (most essential source bytes), and the dominant-file
  rule reads the same primary language, so a core written in an
  unparsed language is not crowded out by side clients the code engine
  parses. Its depth excludes the directories that spell the file's
  declared `package`/`namespace` (JVM, .NET and PHP layouts mirror the
  package path), or a Java library prices below its root build script.
  A .NET project directory named `Root.Suffix` over `namespace Root`
  counts as one of those directories; one spelling the whole namespace
  (`CliFx.Tests/` for `CliFx.Tests`) does not, since peripheral projects
  are named that way and no role rule damps them. Under a non-essential
  directory the discount does not apply: a test tree mirrors the
  namespace it tests, and discounting it let test surfaces take a
  library's budget. Nested Gradle scripts are module manifests, left to
  the listing.
- A fallback surface carries no imports: they say what a file uses, and
  in a roster among many files their rows cost the next file's
  declarations. Its four-declaration level stop keeps a roster compact;
  only the single-file outline descends past it, and only into type
  bodies. Extending the dir-mode roster the same way cost library-file
  breadth (8,000-token renders of fallback repos: 1,616 -> 1,490 files
  with rows) at a flat grid.

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
  assertion that fails the run.
- **One value table:** `value::code_rung_value` per rung and one chunk
  exponent. `Names` sits well below `Decl`, or a roster in every file
  outranks entry-file declarations, docs and manifests. A roster is
  priced per entry, and an entry is a roster row. A `Whole`
  declaration's `Decl` is a roster too (a struct's fields, a class's
  member names): it scales by `roster_mass` of its body entries, or a
  large class ranks below its own members' docs. A re-export-only
  roster stays at the `Names` tier, or barrels outrank root listings at
  small budgets, and its re-export rows stop adding value past
  `MAX_REEXPORT_ENTRIES`: counting every row of a hundreds-of-names
  barrel ranked each of its chunks like a declaration roster, ahead of
  the modules it re-exports. Counting a statement as one entry instead
  cost 3000 −.0024 (answer keys rank short export blocks early). A
  Rust re-export's attribute rows (`FileModel::reexport_attribute_rows`)
  count outside the cap: a feature-gated facade spends two to three
  rows per `pub use`, and capped with them it fell below its crates.
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
    declaration docs;
  - rostering a Python `__init__`'s third-party imports spelled
    `X as X` or listed in `__all__` (0 of 572 real-world renders used
    it);
  - keeping Go methods on an unexported type an exported function
    returns (0 of 572 real-world renders changed).
- **TS/JS docs skip linter / coverage directives** (`// @ts-ignore`,
  `/* istanbul ignore next */`): without the skip, a directive that is a
  declaration's only comment became its whole doc, at doc priority.
- **Lua file-local functions are listed**: hiding `local function`
  declarations measured 3000 −.0020.
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
  non-essential files last: by path, or by content when the language
  says so (`FileModel::non_essential`: a Go file whose only exports are
  methods on unexported types; ranked by its sibling references, it
  opened ahead of the package's API files). Values are untouched: every
  value-model fix tried (a prior for sibling mentions, a steeper roster
  exponent, a depth pin beside entry files) lifted code over README and
  manifest batches and lost. Reference signals for C `#include`, Rust
  `crate::`/`mod` and TS/JS relative imports ranked no better than size
  (C's most-included headers are utility headers) and were dropped.

## Copies are named, not listed

A directory whose entries repeat names kept elsewhere appears in its
parent's listing but is never expanded: a translated mirror
(`pages.ar/` beside `pages/`), an unpacked upstream release
(`prism-master/` with its own license), and each project under a
third-party directory. Wide siblings that repeat an earlier sibling's
three or more entry names are deferred rather than cut; a repeated
`Cargo.toml` + `src/` pair is a crate, and deferring those left a
toolchain's standard library unlisted while an embedded upstream beside
it was listed and parsed (20 MB of generated source). Declared workspace members
are exempt from the shape deferral and the unpacked-release cut, and
package modules (a directory with its own entry file) from the shape
deferral, because a workspace's crates or a Django project's apps share
a layout but not their code, and a listing gates discovery of the
source under it. A
single-child listing run stops at a third-party directory so that its
key, not a vendored child's, is what expansion checks. These rules came
from the robustness corpus; the grid was neutral or better for each
once workspace members were exempt.

## The source spine's listings are priced up

A listing's value does not grow with the source under it, while its
cost grows with its entries, so a central directory of a few dozen
entries over a huge subtree (a monorepo's main package, a framework's
module tree) lost to small peripheral listings and stayed a bare name
even at 8000 tokens. The directories that each hold more than half of
the survey's essential source bytes form one chain down from the root,
and each one's listing head is valued twice (`fs::dir_listing_batches`).
Alternatives tried on the robustness and eval corpora and the grid:
- Weighing by file count instead of bytes made test playgrounds and
  board-support trees the spine (3000 −0.0044).
- Boosting every directory in proportion to its share of the files, or
  every directory holding 30% of the bytes, bought breadth the answer
  keys don't reward (3000 −0.0076 and −0.0021).
- Boosting a split listing's tail too spent huge spine directories'
  budget on names.
- 2× to 5× opened the same directories; 1.5× opened fewer.

Past the survey's entry cap there is no spine. Opening a spine
directory also opens its subdirectories' listings and its files at their
plain values, so a repository whose spine was bare trades some peripheral
rows for its names, and parses more.

## Threads and resource bounds

- **Only parsing is parallel.** Everything that decides output runs on
  the main thread in walk order, so output is independent of thread
  timing only while a parse stays a pure function of the file.
  Tokenizing on workers costs about twice the CPU and doesn't pay.
- **A parse has no deadline**: a timeout would make which files extract
  depend on machine load. Inputs a grammar parses superlinearly are
  refused before the parse by a count over the source instead, as
  `python::has_costly_comment_runs` refuses long runs of `#` rows below
  a statement, which tree-sitter-python's scanner rereads at every row.
- **precis counts o200k tokens itself** (`src/tokenizer.rs`, OpenAI's
  rank file vendored) rather than through tiktoken-rs, whose `CoreBPE`
  took ~70 ms of CPU to build per run (a decoder map and a sorted token
  list precis never reads), more than the whole walk of a small repo,
  and whose backtracking pre-tokenizer made counting the main thread's
  largest cost. Tests pin the counts to tiktoken-rs.
- **The probe budget is per probe, not per run**, so a directory's
  inventory answer depends only on that directory; an answer the budget
  cut short is not cached. `hides_everything_in` is uncapped on purpose:
  it only descends through directories with nothing visible, and a
  capped answer would list each of them as `(empty)`. A run-wide cap
  was tried: the rows it keeps expose the subtree to
  listings and inventory probes, which doubled user time on 0.3M- and
  1.2M-entry trees of ignored build objects.

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
  The exception is size-gated: pricing a listing by its non-media share
  cost the grid while it reached a three-file `media/`, and was neutral
  once it applied only from eleven media files up.
  No answer key ranks a listing past the split size, so the long-listing
  split (`fs::listing_parts`) is invisible to the grid; it is judged on
  real repositories, where a big source directory delivered whole
  (`Lib/`, `drivers/`, `src/`) is never bought and its files never open.
  Whole listings of huge test directories still cost a render probe
  apiece.
- **Pre-0.2 plugin hooks are recognized by their plugin manifest**
  (`run_by_session_hook` in `src/main.rs`): they don't set
  `PRECIS_SESSION_HOOK`, yet their update script installs the latest
  binary, and uncapped output overflows the hook cap. Drop that branch
  once those plugin installs have had time to update.
- **Scheduling grows super-linearly at very large budgets.** Runs at
  3000 tokens and in the plugin stay under 1.5 s on the largest
  corpus repositories, but zig takes 19.6 s at 100k tokens and llvm
  27 s at 1M. `Scheduler::top_ranked` rescans every eligible batch per
  pick, and scheduling a batch drops the cached cost of every other
  batch on its file, so a file with hundreds of batches is re-costed
  per pick. Only the CLI reaches these budgets.

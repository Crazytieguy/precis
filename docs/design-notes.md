# precis v0.2 — design notes

A living doc that captures cross-session design decisions and deferred work
without bloating the per-session plan files. Add entries when you make a
decision worth remembering or defer something a later session will need.

## Architecture (current)

- **Batch**: atomic scheduling unit. Either `FileSystemEntries(Vec<FsEntry>)`
  (declares files/folders into the rendered tree) or `Lines(Vec<FileLineSet>)`
  (adds line content to one or more files). Has a single optional
  `predecessor` and a heuristic `value`.
- **RenderedTree**: a path-keyed tree the scheduler accumulates batch
  contributions into. Renders a hierarchical text output with 4-space indent.
  Tracks line-level ownership so non-ancestor overlap can be detected.
- **Scheduler**: greedy `value / cost.tokens` loop over a `Vec<Batch>`
  indexed by `BatchId(usize)`. Tie-break on lower `BatchId` for determinism.
  Stops when no remaining batch fits the budget.
- **Walker**: language- and scope-specific code that emits seed batches
  for the run's root and successor batches when a batch is scheduled.
  Single implementation in v0.2 first pass: `GenericWalker` (folders/files,
  no content).
- **Tokenizer**: `tiktoken-rs` o200k_base, matching the helper script the
  north-star-author agent uses (`scripts/count-tokens.py`).

## Invariants

- Asserts inside the scheduler/render hot path are **debug-only** (`debug_assert!`).
  In release we'd rather emit a possibly-out-of-budget output than panic.
- Non-ancestor overlap of file lines (a batch's owner not in the new batch's
  predecessor chain) panics in debug; in release it silently overrides.
- Predecessor-chain visited set guards against cycles introduced by buggy walkers.

## Deferred (pick up in later sessions)

### Data model
- **Cost shrink credit** — `cost_lines` uses `saturating_sub` so a refinement
  that makes content shorter never credits tokens back. Conservative wrt
  budget, distorts ranking. Refactor `Cost` to allow signed deltas when a
  North Star surfaces a real shrink case.
- **Borrowed line content** — `RenderedLine` text is owned `String`. A `&str`
  borrow into the source file would save allocations but propagate a lifetime
  through the entire batch graph + walker trait. Defer until a Stage 7+ profile
  surfaces it as a real bottleneck.
- **Predecessor expressivity** — currently a single `Option<BatchId>`. If
  Stage 7 walkers need to express "this batch has structural scope X but must
  also wait for Y", reintroduce a separate `parent` (scope) vs a list of
  ordering predecessors (or a small DAG representation).
- **Path newtypes** — `BatchContent` carries arbitrary `PathBuf`s. A
  `RootRelativePath` (or `DirPath` / `FilePath`) newtype with a private
  constructor would make "path outside the seed root" or "file path used as
  a directory" unrepresentable. Worth doing once the walker surface is more
  varied (Stage 7+).
- **`f64` value / `usize` Cost newtypes** — `Batch.value` admits NaN /
  negative / infinite; `Cost { tokens, bytes }` admits absolute nonsense.
  A `FiniteNonNegativeValue` newtype + private-field `Cost` constructors
  would catch bad inputs at the boundary. Cheap; defer until something
  actually misuses them.

### Scheduler / walker
- **Filesystem-level override** — file-content batch superseding a folder
  listing entry, "N more files" placeholders, alternate non-tree renderings.
- **File-as-seed** — currently rejected with a clear error in `lib.rs`.
  Needs a small content-only walker path. Probably driven by a real
  Stage 7 content walker rather than a generic "show full file" fallback.
- **Multi-path seed** — the CLI accepts `Vec<PathBuf>` but `render()` uses
  only the first path. Multi-root scheduling (one budget across roots) is
  deferred.
- **Walker contract enforcement** — dangling/duplicate batch IDs are
  debug-asserted; cycle handling is silent (visited-set termination).
  Promote to release-asserted walker contract errors when Stage 7 graphs
  get richer.
- **Rich performance optimizations** — `pick_best` is `O(F × tokenize)`
  per scheduling step. Cache per-row token counts, maintain an explicit
  frontier set, avoid retokenizing replaced content. Defer until Stage 7
  fixture sizes surface actual slowness.

### Stopping criterion / value function
- **Stopping criterion beyond "no batch fits"** — dynamic floor or
  value/cost threshold so we stop earlier when remaining batches are weak.
- **Sublinearity formula** — first-pass `value` is linear in batch size;
  the architectural commitment is just that the value function takes batch
  size as input. Pick a concave form when the ontology surfaces concrete
  cases. May differ by batch type.
- **Cross-language vs language-specific value sharing interface** —
  finalize once the ontology is concrete. Today the heuristic lives in
  the walker; that may not scale to many walkers.

### Honesty / verification
- **Reviewer-staleness test** — once divergence reports exist (Stage 7
  iteration), add a test that asserts every committed snapshot has a
  divergence report whose frontmatter `snapshot_hash` matches the
  current snapshot's hash.

### Process
- **More languages** — TypeScript and Python are the next likely targets
  after Rust + markdown.
- **Larger fixtures** — the v0.2 fixture set (log/anyhow/mdbook) is small
  by design. Add scale fixtures once the perf work is in.
- **Alignment-reviewer precision** — calibrate after first real reports
  if it actually drifts.

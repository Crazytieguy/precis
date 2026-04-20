# precis v0.2 — design notes

A living doc that captures cross-session design constraints, decisions, and
deferred work without bloating the per-session plan files. The codebase
itself is the source of truth for architecture and invariants; this doc is
for things that aren't visible from reading `src/`.

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

## Honest rendering

`precis` output is always a verbatim subset of the source. Allowed transforms:

- **Full lines** with their source line number. `RenderedLine::Full(text)`.
- **Line-prefix + trailing ellipsis** (line shown partially, truncated at a
  syntactic boundary). `RenderedLine::Truncated(prefix)`.
- **Bare-ellipsis without a line number** at the start of a file (when the
  first rendered line isn't 1) and between gaps in rendered lines. Inserted
  automatically by `render_file` when scheduling produces non-contiguous
  content. **Tail-of-file elisions are not yet supported** — see deferred
  list below.

No paraphrasing, summarization, or invented content under any circumstances.

## North Star process

- North Star documents (`tests/north-stars/<fixture>.md`) are agent-drafted,
  human-reviewed, and **frozen** before implementation iterates against them.
  Once frozen they are the reviewer's reference standard; implementation
  changes do not edit them.
- **Amendment protocol**: a frozen North Star can be corrected via an
  explicit "defect" review with rationale, diff, and ontology impact. Drift
  is not allowed; deliberate amendments are.

## Cross-language vs language-specific concerns

Many concerns precis cares about are cross-language (value heuristics,
ranking signals, render conventions, structural priorities); only the parts
that genuinely depend on a language's grammar belong in language-specific
code. The exact abstraction for sharing between the two layers is deferred
until the Stage 4 ontology is concrete; the discipline meanwhile is:
**don't accidentally specialize cross-language code to a single language**.

## Deferred (pick up in later sessions)

### Data model
- **Cost shrink credit** — `cost_lines` uses `saturating_sub` so a refinement
  that makes content shorter never credits tokens back. Conservative wrt
  budget, distorts ranking. Refactor `Cost` to allow signed deltas when a
  North Star surfaces a real shrink case.
- **Borrowed line content** — `RenderedLine` text is owned `String`. A `&str`
  borrow into the source file would save allocations but propagate a lifetime
  through the entire batch graph + walker trait. Defer until a Stage 7+
  profile surfaces it as a real bottleneck.
- **Predecessor expressivity** — currently a single `Option<BatchId>`. If
  Stage 7 walkers need to express "this batch has structural scope X but
  must also wait for Y", reintroduce a separate `parent` (scope) vs a list
  of ordering predecessors (or a small DAG representation).
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

### Render
- **Tail elisions** — `render_file` emits a bare ellipsis at the start of a
  file (when the first rendered line isn't 1) and between gaps in rendered
  lines, but never at the end — `BatchContent::Lines` has no source-line-
  count metadata so the renderer can't tell whether more source exists
  past the last rendered line. Either thread the file's total line count
  through the data model, or have walkers emit an explicit tail marker
  when they truncate.
- **Filesystem-level override** — file-content batch superseding a folder
  listing entry, "N more files" placeholders, alternate non-tree renderings.

### Scheduler / walker
- **File-as-seed** — currently rejected with a clear error in `lib.rs`.
  Needs a small content-only walker path, probably driven by a real Stage 7
  content walker rather than a generic "show full file" fallback.
- **Multi-path seed** — the CLI accepts `Vec<PathBuf>` but `render()` uses
  only the first path. Multi-root scheduling (one budget across roots) is
  deferred.
- **Performance optimizations** — `pick_best` is `O(F × tokenize)` per
  scheduling step. Cache per-row token counts, maintain an explicit frontier
  set, avoid re-tokenizing replaced content. Defer until Stage 7 fixture
  sizes surface actual slowness.

### Stopping criterion / value function
- **Stopping criterion beyond "no batch fits"** — dynamic floor or
  value/cost threshold so we stop earlier when remaining batches are weak.
- **Sublinearity formula** — first-pass `value` is linear in batch size;
  the architectural commitment is just that the value function takes batch
  size as input. Pick a concave form when the ontology surfaces concrete
  cases. May differ by batch type.

### Honesty / verification
- **Reviewer-staleness test** — once divergence reports exist (Stage 7
  iteration), add a test that asserts every committed snapshot has a
  divergence report whose frontmatter `snapshot_hash` matches the current
  snapshot's hash.

### Process
- **More languages** — TypeScript and Python are the next likely targets
  after Rust + markdown.
- **Larger fixtures** — the v0.2 fixture set (log/anyhow/mdbook) is small
  by design. Add scale fixtures once the perf work is in.
- **Alignment-reviewer precision** — calibrate after first real reports if
  it actually drifts.

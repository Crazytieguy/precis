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
- **Bare-ellipsis markers** emitted by walkers at specific source lines —
  `RenderedLine::Ellipsis`. The line number isn't rendered; it exists so a
  descendant batch can replace the ellipsis with real content at that line.
  Walkers decide where markers go, because only they know whether a gap
  means "more content here" vs. "line numbers already make this obvious".

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
  through the entire batch graph + walker trait. Defer until a profile
  surfaces it as a real bottleneck.
- **Path newtypes** — `BatchContent` carries arbitrary `PathBuf`s. A
  `RootRelativePath` (or `DirPath` / `FilePath`) newtype with a private
  constructor would make "path outside the seed root" or "file path used as
  a directory" unrepresentable. Worth doing once the walker surface is more
  varied.
- **`ValueSignals` / `Cost` newtypes** — signals admit NaN / negative /
  infinite; `Cost { tokens, bytes }` admits absolute nonsense. A
  `FiniteNonNegativeSignal` + private-field `Cost` constructors would catch
  bad inputs at the boundary. Cheap; defer until something misuses them.

### Render
- **Filesystem-level override** — file-content batch superseding a folder
  listing entry, "N more files" placeholders, alternate non-tree renderings.

### Scheduler / walker
- **File-as-seed** — currently rejected with a clear error in `lib.rs`.
  Needs a small content-only walker path, probably driven by a real
  content walker rather than a generic "show full file" fallback.
- **Multi-path seed** — the CLI accepts `Vec<PathBuf>` but `render()` uses
  only the first path. Multi-root scheduling (one budget across roots) is
  deferred.
- **Performance optimizations** — `best_exact` recomputes marginal cost for
  every batch on every loop iteration. Tokenizer has a thread-local cache
  of string→tokens that cuts the redundant work, but per-batch cost caching
  invalidated on paths-touched would cut it further. Defer until a larger
  fixture surfaces it.

### Stopping criterion / value function
- **Stopping criterion beyond "no batch fits"** — dynamic floor or
  value/cost threshold so we stop earlier when remaining batches are weak.
- **Per-category sublinearity** — `value::ratio` uses `value / sqrt(cost)` for
  every batch. May want per-category shapes (e.g. hard cap on CrateDocLede
  size, gentler concavity on test-as-spec batches).
- **Signal-weight calibration** — `W_CATASTROPHIC = 1000`, `W_FOLLOW_UP =
  400`, `W_ZERO_CALL = 300` are first-pass. Calibrate from north-star
  divergence reports.

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

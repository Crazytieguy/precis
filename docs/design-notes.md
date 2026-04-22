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

`ValueSignals::depth_factor` is a current example of an abstraction worth
revisiting once we support more languages. It's the channel every walker
folds contextual/location priors into (fs depth, non-essential-dir
penalty, sibling count, entrypoint boost, filename conventions). Today
all walkers converge on roughly the same recipe, but the helper functions
live in `src/value.rs` (cross-language) while the composition happens
in each walker (`src/walker/<lang>.rs`). As more languages land, watch
whether the composition itself becomes a duplicated pattern that wants
to be shared, vs. whether languages diverge enough that a single scalar
is the wrong shape and we need richer per-signal location context.

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
- **Sub-section markdown splitting** — H2 sections are now the unit of
  markdown batching (`MarkdownKey::Section { file, section_index }`), but
  some H2 sections are themselves big enough not to fit (anyhow's README
  `## Details` is 104 lines, ~700 tokens; otree's `docs/actions.md` is a
  single H1 with a 350-token table). Splitting further by H3 boundaries —
  or by bullet-list item for content-style sections — would let those
  bodies land piece by piece. Defer until a fixture surfaces the gap as
  load-bearing; today's behavior fits the smaller H2 sections and skips
  the giant ones.
- **Markdown headings-only batch** (analog of Rust `PubItemNames`) — the
  Rust walker's `PubItemNames` is a cheap existence hedge: one line per
  pub item, low cost, high catastrophic-omission weight. Markdown has no
  equivalent today. A headings-only batch per file (H1/H2/H3 titles, no
  prose, predecessor = nothing or the file's headline) would let the
  scheduler tell "does this doc have an X section" even when no section
  body fits. Especially useful for guide-style documents (mdbook book,
  otree docs) where the structure itself is informative. Pairs naturally
  with the `PubItem`-style catastrophic rebalance below.

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
- **Per-category sublinearity** — `value::ratio` uses `value / cost^0.35`
  for every batch (gentler than `sqrt` — see the commit that moved off
  `sqrt` for reasoning). May want per-category shapes (e.g. hard cap on
  `CrateDocLede` size, gentler concavity on test-as-spec batches).
- **Signal-weight calibration** — `W_CATASTROPHIC = 1000`, `W_FOLLOW_UP =
  400`, `W_ZERO_CALL = 300` are first-pass. Calibrate from north-star
  divergence reports.
- **PubItem signal rebalance (catastrophic ↓, follow_up ↑)** — codex
  pointed out that `PubItemNames` already carries the existence hedge
  ("does X exist in this file"), so `PubItem` bodies shouldn't
  double-count catastrophic-omission. Lowering `PubItem.catastrophic`
  from 0.85 is structurally right, but a naive drop (tried 0.3 and 0.5
  in isolation) pushed important bodies out alongside noise — the
  cumulative loss of body-level value wasn't absorbed anywhere. The
  right form is probably `catastrophic ↓` **paired** with
  `follow_up_minimization ↑` and `zero_tool_call_understanding ↑`, so
  the total score stays roughly constant but the ontological accounting
  matches what each signal means. Try as a simultaneous adjustment, not
  a one-axis reduction. Pairs cleanly with the markdown headings-only
  batch (same "cheap existence hedge up-front, bodies later" shape).
- **Sibling-count devaluation** — when a file emits many per-item batches
  (e.g. a config module with 20 `pub struct` children), each one's
  individual value/cost ratio beats the value/cost of a single important
  body elsewhere (e.g. `CommandArgs` in `src/cmd.rs`), producing a
  "wide-but-shallow signature sweep" across deep files at the expense of
  root-level anchors. The old design collapsed all pub items per file into
  one batch which naturally dampened this; the split-per-item model is
  better for ranking precision but loses the dampener. A first attempt
  folded a `sibling_factor(n_siblings)` into `PubItem`'s `depth_factor`
  at walker expand time — but across 12 reviews it regressed more than
  it improved (anyhow_6000 +10, log_6000 +15, otree_3000 8→18 major
  ranking divergences — count-based measurements subject to the noise
  caveat in the Process section). The tradeoff is
  structural: dense core files (anyhow's `src/lib.rs` with ~25 pub items,
  log's `src/lib.rs` similar) are *legitimately* dense; penalizing them
  displaces their load-bearing bodies (Level/LevelFilter rustdoc,
  Chain/Context docs) in favor of content elsewhere that's not actually
  more valuable. Decoration-heavy dense files (otree's `src/config/colors.rs`
  with 7 color sub-structs) look structurally identical but have very
  different intrinsic value. `is_entrypoint_file` doesn't reliably
  distinguish them — `mod.rs` catches both tests/ helpers and real
  module roots. Needs a richer signal than sibling count alone. Plausibly
  pairs with an eventual NS-author update that ranks `PubItemNames`-style
  location hints as first-class, so the calibration target is clearer.
- **API-surface signal (`pub(crate)` / `pub(super)` / `#[doc(hidden)]`)**
  — these visibility levels and attribute mark items that are
  compiler-visible inside the crate but aren't part of the external API
  surface. An experiment added `is_api_surface` to `PubItemInfo` (cheap
  AST predicates) and applied a 0.3× multiplier to PubItem/PubItemDoc
  bodies for non-API items. Effect was ambiguous given reviewer noise:
  the main concrete win was a ~12-divergence improvement on anyhow_1500
  (ChainState / ErrorImpl / ContextError `pub(crate)` bodies correctly
  demoted to signature-only); other fixtures shifted in both directions
  within what looked like review-run variance. Structurally sound —
  those items genuinely shouldn't compete with public-API bodies — but
  the cascade effects muddied the measurement. Worth retrying with a
  deterministic metric, and ideally with a richer cross-file
  mod-visibility analysis (private `mod x;` in lib.rs makes all `pub`
  items inside `x.rs` effectively pub(crate), which the current local
  check misses entirely).

### North Star / reviewer alignment
- **Rank `PubItemNames` as a first-class NS batch** — the Rust walker
  emits a `pub struct X {\n…` location-hint batch per file
  (`src/walker/rust.rs` `PubItemNames`), but current north stars model
  only full-body batches. Reviewers therefore see the hint render and
  flag it as "batch partially included" against the NS expectation. The
  honest description is "two walker batches exist; the names-surface one
  fired, the body deferred". Batch with next NS-author update so the
  regeneration cost lands once.

### Process
- **More languages** — TypeScript and Python are the next likely targets
  after Rust + markdown.
- **Larger fixtures** — the v0.2 fixture set (log/anyhow/mdbook) is small
  by design. Add scale fixtures once the perf work is in.
- **Alignment-reviewer count is a noisy fitness metric** — the count of
  `- [category] [severity]` divergence bullets does not scale
  proportionally with snapshot-content deltas. Concrete observation
  from this session: a ~14-line content change in log_6000 (one new
  README section, one dropped `__private_api::log` body fragment)
  produced a +21 jump in divergence count (28 → 49). The reviewer's
  qualitative findings stay roughly consistent across runs (same
  batches flagged as partial, same ranking inversions), but the count
  inflates whenever new content introduces new partial-batch
  observations, even when the new content is *better aligned* with the
  North Star in aggregate. Treat count as directional only; always
  cross-check qualitative snapshot diffs before concluding a change is
  better or worse. We don't have clean evidence of run-to-run variance
  on *identical* snapshots — the "oscillation" I thought I was seeing
  was different snapshots from different experiments producing
  similar-but-not-identical counts.
- **Deterministic alignment metric** (promoted from speculative) — given
  the reviewer noise above and the quota cost of running 12 agents per
  calibration iteration (this session pushed the user's weekly Claude
  quota past its 5-hour window), a deterministic NS→snapshot line-range
  coverage metric is looking more attractive. Shape: for each NS batch,
  compute "what fraction of its declared line range appears in the
  snapshot" + a per-tier weighted sum. Gives a continuous, reproducible
  score that can drive calibration tuning without per-experiment agent
  cost. Risk: (1) places load-bearing weight on NS line-ranges being
  exactly right; (2) misses honesty concerns and ranking-order
  inversions that need semantic judgment; (3) treats "batch present
  but out of priority order" as a hit when NS actually ranks it low.
  Mitigation: use the deterministic score for calibration iteration;
  reserve agent reviews for milestone checkpoints (new fixture, new
  language, ontology change).
- **Alignment-reviewer agent sometimes doesn't Write** — observed once
  in this session: the agent returned the full report in its output
  summary but didn't call the Write tool, leaving the existing review
  stale on disk. Consider tightening `.claude/agents/alignment-reviewer.md`
  with a hard post-condition ("return only after Write has succeeded")
  or switching to a template where the Write call is the return path.
- **Alignment-reviewer short-circuits on matching hash** — when an
  existing review's `snapshot_hash` matches the current snapshot, the
  agent treats it as fresh and returns without re-scoring, even when
  the intent is to re-evaluate under changed interpretation (e.g., a
  reviewer-prompt update, or investigating review variance). Current
  workaround: delete the review file before re-running. Consider adding
  a "force" flag or routing re-score requests through a different entry
  point.
- **Codex CLI long-prompt hang** — `codex exec "<long prompt>"` with a
  multi-KB prompt as a positional argument appeared to hang
  indefinitely (no output, process stayed alive for >10 min). Piping
  the prompt via stdin (`cat prompt.md | codex exec`) works reliably.
  Worth documenting wherever we codify codex invocation patterns.
- **North Star regeneration is expensive** — three drafts (up to ~40
  min each) + a combiner pass. Batch NS updates across multiple
  planned changes (e.g., the "rank PubItemNames first-class" item and
  the future markdown-headings-only equivalent) to amortize the cost.

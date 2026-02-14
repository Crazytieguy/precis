# Issues

## Snapshot coverage

- 5-per-language target achieved: 5 Rust, 5 TypeScript, 5 JavaScript, 5 TSX, 5 Python — each with level 1, level 2, level 3, and level 4 snapshots
- 8 fixtures have subdirectory snapshot tests; remaining fixtures (either, debug, mitt, etc.) have flat source trees with no meaningful subdirectories to test
- All 25 fixtures have level 1, 2, 3, and 4 snapshots; levels 0 and 5 tested via samples only
- Budget-based snapshot tests added: mitt (5 budgets hitting levels 0–3 and 5), ini (3 budgets), neverthrow (2 budgets, multi-file). Each snapshot includes metadata header showing budget → level → word count.
- Level 4 fixture snapshots now cover all 25 fixtures; spot-check confirmed type bodies (struct fields, enum variants, trait members, class bodies) expand correctly
- Python fixtures: pluggy, tomli, humanize, python-dotenv, typeguard — exercises classes, decorators, type hints, docstrings

## Codebase quality

- `streaming-iterator` is a direct dependency only because tree-sitter v0.25 uses `StreamingIterator` for `QueryMatches`. If tree-sitter changes this API, the dep can be removed.

## Current state

- Parsing works for Rust, TypeScript, JavaScript, TSX, Python — extracts symbol names, kinds, visibility
- Python: module-level constants extracted (type-annotated assignments, UPPER_CASE names, dunder attributes like `__all__`)
- Output supports 6 granularity levels: 0 (file paths), 1 (symbol names), 2 (full signature lines), 3 (signatures with doc comments), 4 (type bodies expanded), 5 (full source)
- Monotonicity invariant (higher level = more words) tested against all fixtures
- `--budget` flag works: binary search over levels selects highest level fitting within word budget
- `--level` flag allows selecting a specific granularity level directly (mutually exclusive with `--budget`)
- `path` arg accepts both files and directories
- Default output (no flags) is level 1 (symbol names truncated)

## Feature development

- Make levels depth-aware and file-size-aware (currently uniform across all files)
- Priority languages: Markdown (Python now supported)
- Add `--json` output flag
- Doc comment detection (level 3) is text-based heuristic; does not use tree-sitter comment nodes. Handles `///`, `//!`, `/** */` blocks, Python `#` comments, and Python docstrings (`"""..."""` / `'''...'''`). Skips `#[attr]` and `@decorator` lines between doc comments and symbols; decorators/attributes are always shown at level 3+.
- Python docstrings (triple-quoted strings after `def`/`class` lines) are captured at levels 3 and 4 via text-based heuristic. Handles single-line, multi-line, and raw (`r"""`) docstrings.
- Python: module-level constants are captured if type-annotated (`VERSION: str = "0.1.0"`), UPPER_CASE (`MAX_SIZE = 100`), or dunder (`__all__ = [...]`). Lowercase untyped assignments (e.g. `logger = ...`) are excluded to reduce noise. `TypeAlias` annotations are mapped to `Const` kind (could be `TypeAlias` in the future).

## Design decisions

- Token counting: word count (not tokenizer or char count)
- Entrypoint detection: not needed — depth in file tree is a sufficient heuristic. Users zoom into subdirectories by running the tool on them directly with a larger budget.
- Output lines are line-prefixes of the original source, preserving original whitespace. Nesting (e.g. impl blocks) is represented naturally by the source's own indentation.
- Line number format: right-aligned with `→` separator (e.g. `    12→    pub fn new(`).
- Budgeting algorithm: single `render(level, path, content)` function with `MAX_LEVEL` constant. Binary search over levels to fit word budget. Monotonicity invariant (higher level = more words) tested against fixtures. Levels are depth- and file-size-aware. Start crude, add granularity over time.

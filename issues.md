# Issues

## Snapshot coverage

- 5-per-language target achieved: 5 Rust, 5 TypeScript, 5 JavaScript, 5 TSX, 5 Python, 5 Go — each with level 1, level 2, level 3, and level 4 snapshots. Go fixtures: go-multierror, xxhash, color, go-version, structs. Markdown: 1 fixture (mdbook) with levels 1-4 + subdirectory tests + budget tests.
- 9 fixtures have subdirectory snapshot tests; remaining fixtures (either, debug, mitt, etc.) have flat source trees with no meaningful subdirectories to test
- All 31 fixtures have level 1, 2, 3, and 4 snapshots; levels 0 and 5 tested via samples only
- Budget-based snapshot tests added: mitt (5 budgets hitting levels 0–3 and 5), ini (3 budgets), neverthrow (2 budgets, multi-file), either (4 budgets, Rust, hitting levels 0/1/3/4), pluggy (4 budgets, Python, hitting levels 0/1/2/3), typeguard (4 budgets, Python, hitting levels 0/1/3/4), mdbook (5 budgets, Markdown, hitting levels 0/1/3/4/5), sonner (4 budgets, TSX, hitting levels 0/1/3/4), go-multierror (4 budgets, Go, hitting levels 0/1/3/4), xxhash (4 budgets, Go, hitting levels 0/1/3/4), color (4 budgets, Go, hitting levels 0/1/3/4), go-version (4 budgets, Go, hitting levels 0/1/3/4), structs (4 budgets, Go, hitting levels 0/1/3/4). Each snapshot includes metadata header showing budget → level → word count. All supported language families (Rust, JS/TS, TSX, Python, Go, Markdown) now have budget test coverage.
- Level 4 fixture snapshots now cover all 31 fixtures; spot-check confirmed type bodies (struct fields, enum variants, trait members, class bodies) expand correctly
- No current fixture exercises TypeScript abstract classes — covered by unit test only
- Python fixtures: pluggy, tomli, humanize, python-dotenv, typeguard — exercises classes, decorators, type hints, docstrings

## Codebase quality

- `streaming-iterator` is a direct dependency only because tree-sitter v0.25 uses `StreamingIterator` for `QueryMatches`. If tree-sitter changes this API, the dep can be removed.

## Current state

- Parsing works for Rust, TypeScript, JavaScript, TSX, Python, Go, Markdown — extracts symbol names, kinds, visibility
- Go: functions, methods (with receiver), structs, interfaces, type aliases, consts, vars. Visibility via uppercase first letter (Go export convention). Doc comments use plain `//` (godoc style).
- TypeScript: abstract classes (`abstract class`) and abstract method signatures captured correctly
- Python: module-level constants extracted (type-annotated assignments, UPPER_CASE names, dunder attributes like `__all__`)
- Output supports 6 granularity levels: 0 (file paths), 1 (symbol names), 2 (full multi-line signatures), 3 (signatures with doc comments; Markdown: headings + first paragraph), 4 (type bodies expanded; Markdown: all content between headings), 5 (full source)
- Levels are depth-aware and file-size-aware: deeper files have their effective level reduced (penalty = depth/2), and large files (1000+ lines) have their effective level reduced by 1 at levels 3+. Both penalties stack; budget mode automatically prioritises shallow, focused files.
- Monotonicity invariant (higher level = more words) tested against all fixtures
- `--budget` flag works: binary search over levels selects highest level fitting within word budget
- `--level` flag allows selecting a specific granularity level directly (mutually exclusive with `--budget`)
- `path` arg accepts both files and directories
- Default output (no flags) is level 1 (symbol names truncated)
- `--json` flag outputs structured JSON with `level`, `words`, and per-file `path`/`content` fields
- File discovery filters out non-source directories (`tests/`, `test/`, `testing/`, `__tests__/`, `benches/`, `benchmark/`, `benchmarks/`, `testdata/`, `vendor/`) and test file patterns (`*.test.*`, `*.spec.*`, `test_*`, `*_test`, `conftest.py`); uses relative paths so parent directories of the walk root don't trigger false positives

## Feature development

- No pending features.

## Implementation notes

- Doc comment detection (level 3) is text-based heuristic; does not use tree-sitter comment nodes. Handles `///`, `//!`, `/** */` blocks, Go `//` comments (godoc), Python `#` comments, and Python docstrings (`"""..."""` / `'''...'''`). Skips `#[attr]` and `@decorator` lines between doc comments and symbols; also skips blank lines between doc comment `*/` and symbol (common JS/TS style). Decorators/attributes are always shown at level 3+. Go `//` doc comments are language-gated via `is_go` flag to avoid false positives in other languages where `//` is not a doc comment convention.
- Python docstrings (triple-quoted strings after `def`/`class` lines) are captured at levels 3 and 4 via text-based heuristic. Handles single-line, multi-line, and all string prefixes (`r"""`, `u"""`, `f"""`, `b"""`, `rf"""`, `br"""`, etc.).
- Python: module-level constants are captured if type-annotated (`VERSION: str = "0.1.0"`), UPPER_CASE (`MAX_SIZE = 100`), or dunder (`__all__ = [...]`). Lowercase untyped assignments (e.g. `logger = ...`) are excluded to reduce noise. `TypeAlias` annotations are mapped to `Const` kind (could be `TypeAlias` in the future).
- Multi-line signature detection (level 2+) is text-based heuristic: scans forward from the symbol line for `{`/`;` (C-like) or `:` (Python). Applies to functions, impl blocks, traits, structs, enums, classes, and interfaces — any symbol kind that can have multi-line declarations (generic parameters, `where` clauses, bounds). Expression-bodied arrow functions without semicolons fall back to single-line display.
- Markdown: levels 1 and 2 produce identical output (heading lines are the same truncated or full since heading text IS the line content). Not a significant issue — same happens for simple single-line code symbols.
- Markdown: setext headings (underlined with `===` or `---`) are supported. Symbol names are trimmed (tree-sitter's paragraph node includes trailing newlines). Content rendering at levels 3-4 uses `max(sym_line + 1, end_line - 1)` to correctly skip underlines for setext headings while preserving behavior for ATX headings.

## Design decisions

- Token counting: word count (not tokenizer or char count)
- Entrypoint detection: not needed — depth in file tree is a sufficient heuristic. Users zoom into subdirectories by running the tool on them directly with a larger budget.
- Output lines are line-prefixes of the original source, preserving original whitespace. Nesting (e.g. impl blocks) is represented naturally by the source's own indentation.
- Line number format: right-aligned with `→` separator (e.g. `    12→    pub fn new(`).
- Budgeting algorithm: single `render(level, path, content)` function with `MAX_LEVEL` constant. Binary search over levels to fit word budget. Monotonicity invariant (higher level = more words) tested against fixtures. Depth-aware and size-aware rendering reduces effective level for deeper/larger files, so the budget search naturally picks higher nominal levels since the cost per level is lower.

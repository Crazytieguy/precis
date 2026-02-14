# Issues

## Snapshot coverage

- Add more fixture snapshot tests — aiming for 5+ per language, currently 5 Rust (`either`, `anyhow`, `once_cell`, `thiserror`, `log`), 5 TypeScript (`neverthrow`, `ts-pattern`, `superstruct`, `mitt`, `ky`), 5 JavaScript (`semver`, `dotenv`, `commander`, `debug`, `ini`), 5 TSX (`cmdk`, `react-hot-toast`, `sonner`, `vaul`, `input-otp`)

## Codebase quality

- `streaming-iterator` is a direct dependency only because tree-sitter v0.25 uses `StreamingIterator` for `QueryMatches`. If tree-sitter changes this API, the dep can be removed.

## Current state

- Parsing works for Rust, TypeScript, JavaScript, TSX — extracts symbol names, kinds, visibility
- Output supports 4 granularity levels: 0 (file paths), 1 (symbol names), 2 (full signature lines), 3 (full source)
- Monotonicity invariant (higher level = more words) tested against all fixtures
- `--budget` flag is parsed but not wired up — no budget algorithm yet (need binary search over levels)
- `path` arg only accepts directories, not files
- Default output is level 1 (symbol names truncated)

## Feature development

- Wire up `--budget` flag: implement binary search over levels to fit word budget
- Make levels depth-aware and file-size-aware (currently uniform across all files)
- Accept file paths in addition to directories
- Add more language grammars (Python, Go)
- Add `--json` output flag

## Design decisions

- Token counting: word count (not tokenizer or char count)
- Entrypoint detection: not needed — depth in file tree is a sufficient heuristic. Users zoom into subdirectories by running the tool on them directly with a larger budget.
- Output lines are line-prefixes of the original source, preserving original whitespace. Nesting (e.g. impl blocks) is represented naturally by the source's own indentation.
- Line number format: right-aligned with `→` separator (e.g. `    12→    pub fn new(`).
- Budgeting algorithm: single `render(level, path, content)` function with `MAX_LEVEL` constant. Binary search over levels to fit word budget. Monotonicity invariant (higher level = more words) tested against fixtures. Levels are depth- and file-size-aware. Start crude, add granularity over time.

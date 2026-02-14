# Issues

## Snapshot coverage

- Add more fixture snapshot tests — aiming for 5+ per language, currently 5 Rust (`either`, `anyhow`, `once_cell`, `thiserror`, `log`), 5 TypeScript (`neverthrow`, `ts-pattern`, `superstruct`, `mitt`, `ky`), 5 JavaScript (`semver`, `dotenv`, `commander`, `debug`, `ini`), 5 TSX (`cmdk`, `react-hot-toast`, `sonner`, `vaul`, `input-otp`)

## Codebase quality

- `streaming-iterator` is a direct dependency only because tree-sitter v0.25 uses `StreamingIterator` for `QueryMatches`. If tree-sitter changes this API, the dep can be removed.

## Current state

- Parsing works for Rust, TypeScript, JavaScript, TSX — extracts symbol names, kinds, visibility
- Output shows symbol lines truncated at the symbol name, with a legacy format (`  pub fn new :12`) that doesn't yet match the target format (`    12→    pub fn new`)
- `--budget` flag is parsed but not wired up — no granularity/budgeting logic yet
- `path` arg only accepts directories, not files

## Feature development

- Update output format to match README (line-prefixes with right-aligned line numbers and `→` separator, preserving original indentation)
- Implement signature-level output (extend line-prefix truncation past the symbol name)
- Implement the granularity/budgeting system (see README for algorithm design)
- Accept file paths in addition to directories
- Add more language grammars (Python, Go)
- Add `--json` output flag

## Design decisions

- Token counting: word count (not tokenizer or char count)
- Entrypoint detection: not needed — depth in file tree is a sufficient heuristic. Users zoom into subdirectories by running the tool on them directly with a larger budget.
- Output lines are line-prefixes of the original source, preserving original whitespace. Nesting (e.g. impl blocks) is represented naturally by the source's own indentation.
- Line number format: right-aligned with `→` separator (e.g. `    12→    pub fn new(`).
- Budgeting algorithm: single `render(level, path, content)` function with `MAX_LEVEL` constant. Binary search over levels to fit word budget. Monotonicity invariant (higher level = more words) tested against fixtures. Levels are depth- and file-size-aware. Start crude, add granularity over time.

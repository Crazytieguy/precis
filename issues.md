# Issues

## Feature development

- **Budget-based testing** — snapshot tests should primarily use `--budget` rather than `--level`, since levels are an internal implementation detail that will change as we expand to ~32 levels. Existing per-level fixture snapshots should be migrated to budget-based snapshots. Budget snapshots are stable across level refactors and directly test the user-facing behavior. The `--level` flag exists for debugging and manual investigation, not as a primary test parameter.
- **Root-level fixture testing** — most fixture snapshots currently target subdirectories (e.g. `either/src`, `pluggy/src/pluggy`), but the most common use case is running precis on a repo root. Snapshots should be migrated to include more root-level targets, with a mix of root and subdirectory targets for variety. Root-level testing exercises depth penalties, file discovery filtering, and multi-language repos more realistically.
- **Tree-sitter reuse in tests** — the monotonicity test currently re-parses with tree-sitter at every level for every file. Symbols could be extracted once and reused across levels. Not a problem now (~5 seconds) but will matter as we expand to ~32 levels.
- **Default to current directory** — `precis` with no path argument should run on `.` by default, since that's the most common use case.
- **Import statements** — show at a low level (between current levels 1 and 2). Distinguish 1st-party imports (relative paths like `./`, `../`; Rust `crate::`, `super::`; Go module path) from 3rd-party. 3rd-party imports are lower priority (repeated noise like `import React` in every file). 1st-party imports are high signal for understanding a file's role.
- **Test/vendor/example directories** — instead of excluding entirely during file discovery, render at a lower effective level (apply a large penalty). They appear as paths at tight budgets, content at generous budgets. More honest than invisible filtering.
- **README boost** — render `README.md` / `Readme.md` / `readme.md` at a higher effective level (reduce depth penalty or apply negative penalty). Other markdown files follow normal depth rules.
- **File omission** — at low levels, the render function can return nothing for some files (e.g. deep, large, or low-priority files). This is not a special level — just the render function's per-file decision based on `(level, path, content)`. Useful for repos with many files where showing all paths at level 0 is already too much.
- **Config file support** — support json/yaml/toml files with different rendering heuristics than code. Files with "config" in the name get higher priority. JSON file content is generally less useful than code at equivalent depth.
- **Visibility-aware rendering** — use the already-computed `is_public` field in rendering prioritization. Public symbols shown before private ones as levels increase.
- **Re-evaluate tree-sitter for doc comments and multi-line signatures** — both are currently text-based heuristics (scanning for comment prefixes, scanning for `{`/`;`/`:`). Tree-sitter grammars model comments as sibling nodes (not children of declarations), making association with symbols harder via queries alone. Text heuristics work uniformly across languages. Worth re-evaluating whether tree-sitter-based approaches would be more robust, especially as more languages are added.

## Current state

- Parsing: Rust, TypeScript, JavaScript, TSX, Python, Go, Markdown. 6 granularity levels (0–5). Depth-aware and file-size-aware rendering. `--budget`, `--level`, `--json` flags.
- File discovery filters out test/benchmark/vendor/example directories and test file patterns. Uses relative paths so parent directories don't trigger false positives.
- 31 test fixtures across all supported languages with per-level and budget-based snapshot tests. Monotonicity invariant tested across representative fixtures.
- `cargo run --bin budget_util` measures budget utilization across all budget snapshots.

## Implementation notes

- Doc comment and multi-line signature detection are text-based heuristics (see feature development for re-evaluation issue). Doc comments handle `///`, `//!`, `/** */`, Go `//` (godoc, language-gated), Python `#` and docstrings. Multi-line signatures scan for `{`/`;`/`:` delimiters.
- Overload dedup: consecutive function symbols with the same name are collapsed (keeping the last). Skipped for Go (`init()` functions).
- Type aliases at level 2+ show their full definition (the definition IS the signature). Uses `emitted_up_to` to avoid duplicating lines when type alias ranges encompass nested symbols.
- Markdown levels 1 and 2 produce identical output (heading text is already the full line content).

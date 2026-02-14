# symbols

A CLI tool that extracts symbols from a directory and returns a token-efficient breakdown, designed for feeding codebase context into LLM prompts.

## Design (Draft)

These are initial design directions, not final decisions. Expect them to evolve during implementation.

### Parsing

Uses **tree-sitter** for language-agnostic symbol extraction. Each supported language has a small query file (`.scm`) that maps language-specific node types to a common symbol model (functions, types, modules, exports, etc.).

### Token Budgeting

Takes a `--budget` flag (in tokens). Uses a **granularity hierarchy** to fit output within the budget:

1. Folder names only
2. Folder entrypoints — exported symbols only
3. All files — names only
4. All files — signatures (params, return types)
5. Signatures + docstrings
6. Full source for small files, signatures for large

Granularity is chosen **per-file/folder**, not globally. Small files get promoted to higher detail levels when budget allows. Folders with clear entrypoints (`index.ts`, `__init__.py`, `mod.rs`, etc.) are collapsed to just their public API when budget is tight.

The tool does a binary search across granularity levels to find the most detailed output that fits the budget.

### Output Format

Plain text / markdown, optimized for direct use in LLM prompts.

**Substring constraint:** Other than line numbers, each line in the output should be a substring of the actual original line in the source file. The tool extracts from the source rather than synthesizing new representations — no cross-language normalization of keywords.

```
src/
  parser/
    index.ts: parseFile(path: string): Symbol[], parseDirectory(path: string): FileMap
    types.ts: Symbol, FileMap, GranularityLevel
  budget/
    index.ts: fitToBudget(symbols: Map, budget: number): string
```

A `--json` flag may be added later for machine consumption.

## Supported Languages

Per-language support requires a tree-sitter grammar and a query file defining:
- Which node types count as symbols
- How to extract signatures
- What signals "public" (e.g., `export` in JS, `pub` in Rust, capitalization in Go)
- Entrypoint file conventions (e.g., `index.ts`, `__init__.py`, `mod.rs`)

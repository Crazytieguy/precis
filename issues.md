# Issues

## Snapshot coverage

- Add more fixture snapshot tests — Rust: `either`, `anyhow`; TypeScript: `neverthrow`; JavaScript: `semver`; TSX: `cmdk`; aiming for 5+ per language, currently 2 Rust and 1 each for the rest

## Codebase quality

- `streaming-iterator` is a direct dependency only because tree-sitter v0.25 uses `StreamingIterator` for `QueryMatches`. If tree-sitter changes this API, the dep can be removed.

## Feature development

- Add more language grammars (Python, Go)
- Implement signature extraction (params, return types) — currently only symbol names
- Implement the granularity hierarchy for token budgeting
- Add `--json` output flag

## Design questions

- Token counting strategy: use a tokenizer crate (e.g. tiktoken-rs) or approximate by character count?
- How to handle entrypoint detection across languages — hardcoded list vs configurable?
- Impl blocks show nested functions (e.g. `impl Foo` then `fn new`). Should these be nested in output, or is flat fine?

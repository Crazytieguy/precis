# Issues

## Output uses synthesized keywords instead of source substrings

**Priority: do first.** The output currently normalizes keywords across languages (e.g. `export` → `pub`, `function` → `fn`, `interface` → `type` in TypeScript). This violates the substring constraint: each output line should be a substring of the original source line. Remove the normalization — this should be done by deleting code, not adding code.

## Snapshot quality

- TypeScript: `lexical_declaration` captures both `const` and `let` as `const`. Should `let` exports be shown differently?

## Snapshot coverage

- Add more fixture snapshot tests — `either` (Rust) and `neverthrow` (TypeScript) done; could use more diverse projects

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

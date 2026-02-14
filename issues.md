# Issues

## Snapshot quality

- TypeScript: method overload signatures (e.g. `fn andThen` appearing 3 times) are captured as separate symbols. Should overloads be deduplicated or collapsed?
- TypeScript: arrow functions assigned to `const` (e.g. `export const foo = () => ...`) show as `const` not `fn`. Should we detect and reclassify these?
- TypeScript: `lexical_declaration` captures both `const` and `let` as `const`. Should `let` exports be shown differently?

## Snapshot coverage

- Add more fixture snapshot tests — `either` (Rust) and `neverthrow` (TypeScript) done; could use more diverse projects
- Should `SOURCE_EXTENSIONS` in walk.rs be derived from which tree-sitter grammars are available, rather than hardcoded? (Currently py/go are walked but not parsed.)

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

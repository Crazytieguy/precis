# Issues

## Next steps

- Add more language grammars (Python, Go) — TypeScript/TSX done
- Implement signature extraction (params, return types) — currently only symbol names
- Implement the granularity hierarchy for token budgeting
- Add more fixture snapshot tests — `either` (Rust) and `neverthrow` (TypeScript) done; could use more diverse projects
- Add `--json` output flag
- Filter out test files from default output — done for Rust (`#[test]`, `#[cfg(test)]`) and JS/TS (`*.test.*`, `*.spec.*`, `__tests__/`)

## Design questions

- Token counting strategy: use a tokenizer crate (e.g. tiktoken-rs) or approximate by character count?
- How to handle entrypoint detection across languages — hardcoded list vs configurable?
- Should `SOURCE_EXTENSIONS` in walk.rs be derived from which tree-sitter grammars are available, rather than hardcoded?
- Impl blocks show nested functions (e.g. `impl Foo` then `fn new`). Should these be nested in output, or is flat fine?
- TypeScript: arrow functions assigned to `const` (e.g. `export const foo = () => ...`) show as `const` not `fn`. Should we detect and reclassify these?
- TypeScript: `lexical_declaration` captures both `const` and `let` as `const`. Should `let` exports be shown differently?
- TypeScript: method overload signatures (e.g. `fn andThen` appearing 3 times) are captured as separate symbols. Should overloads be deduplicated or collapsed?

## Technical debt

- `streaming-iterator` is a direct dependency only because tree-sitter v0.25 uses `StreamingIterator` for `QueryMatches`. If tree-sitter changes this API, the dep can be removed.
- The `self_snapshot` test will break whenever source files in `src/` change. This is intentional (catches unintended changes) but means snapshots need updating with `cargo insta review` after any code change. Consider whether fixture-only snapshots would be less noisy.

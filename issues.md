# Issues

## Next steps

- Add more language grammars (TypeScript, Python, Go)
- Implement signature extraction (params, return types) — currently only symbol names
- Implement the granularity hierarchy for token budgeting
- Add snapshot tests using real open-source fixture repos (infra is set up with `insta`, needs real fixtures)
- Add `--json` output flag
- Filter out test functions / `#[cfg(test)]` modules from default output

## Design questions

- Token counting strategy: use a tokenizer crate (e.g. tiktoken-rs) or approximate by character count?
- How to handle entrypoint detection across languages — hardcoded list vs configurable?
- Should `SOURCE_EXTENSIONS` in walk.rs be derived from which tree-sitter grammars are available, rather than hardcoded?
- Impl blocks show nested functions (e.g. `impl Foo` then `fn new`). Should these be nested in output, or is flat fine?
- Should `#[test]` functions and `#[cfg(test)]` modules be excluded by default?

## Technical debt

- `streaming-iterator` is a direct dependency only because tree-sitter v0.25 uses `StreamingIterator` for `QueryMatches`. If tree-sitter changes this API, the dep can be removed.
- Trait method signatures (no body, e.g. `fn visit_token(&self)`) are not captured — tree-sitter classifies these as `function_signature_item`, not `function_item`. The Rust query only matches `function_item`.
- The `self_snapshot` test will break whenever source files in `src/` change. This is intentional (catches unintended changes) but means snapshots need updating with `cargo insta review` after any code change. Consider whether fixture-only snapshots would be less noisy.

# Issues

## Remaining work

- **Per-group stage value tuning** — the initial stage values (1.0, 0.7, 0.6, etc.) are starting points. Review snapshots across languages and tune per-kind values for better output quality.

## Implementation notes

- **Doc comment detection** is partially migrated to tree-sitter. `Symbol.doc_start_line` is computed from AST sibling navigation, with text heuristic fallback. Handles `///`, `/**` (Rust), `/**` JSDoc (TypeScript), `//`/`/*` (Go, C), `#` (Python). Also handles wrapper nodes (`export_statement`, `decorated_definition`) for exported/decorated symbols. Python docstrings remain text-based heuristics. (Multi-line signature detection was also migrated to tree-sitter — `Symbol.sig_end_line` is computed from the AST body/block child, with text heuristic fallback for nodes without detectable body like type aliases and method signatures.)
- **Rust `//!` inner doc comments** — the text heuristic fallback still treats `//!` as a doc comment for the next item, but `//!` documents the containing module. The tree-sitter path correctly excludes `//!`. The text heuristic should be updated to match, but this requires a separate snapshot update for Rust fixtures.
- **Overload dedup** — consecutive function symbols with the same name are collapsed (keeping the last). Skipped for Go (`init()` functions).
- **Python doc truncation marker mismatch** — Python symbols with both pre-symbol `#` comments and post-signature docstrings concatenate them into a flat `doc_line_words` vector. The scheduler counts truncation markers based on the flat length, but the renderer emits the two sections separately with independent truncation. At certain `doc_n` values the marker count can differ by 1 word. Minor and rare in practice.

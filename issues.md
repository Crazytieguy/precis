# Issues

## Remaining work

- **Per-group stage value tuning** — the initial stage values (1.0, 0.7, 0.6, etc.) are starting points. Review snapshots across languages and tune per-kind values for better output quality.
- **Try tiktoken-based budgeting** — replace word-count budgeting with tiktoken token counts for more accurate LLM budget estimation. Keep if performance is viable, otherwise revert and document findings.

## Implementation notes

- **Doc comment detection** is partially migrated to tree-sitter. `Symbol.doc_start_line` is computed from AST sibling navigation, with text heuristic fallback. Handles `///`, `/**` (Rust), `/**` JSDoc (TypeScript), `//`/`/*` (Go, C), `#` (Python). Also handles wrapper nodes (`export_statement`, `decorated_definition`) for exported/decorated symbols. Python docstrings remain text-based heuristics. (Multi-line signature detection was also migrated to tree-sitter — `Symbol.sig_end_line` is computed from the AST body/block child, with text heuristic fallback for nodes without detectable body like type aliases and method signatures.)
- **Overload dedup** — consecutive function symbols with the same name are collapsed (keeping the last). Skipped for Go (`init()` functions).

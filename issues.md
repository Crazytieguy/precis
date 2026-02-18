# Issues

## Remaining work

- **Per-group stage value tuning** — the initial stage values (1.0, 0.7, 0.6, etc.) are starting points. Review snapshots across languages and tune per-kind values for better output quality.
- **Tiktoken build_groups performance** — switching from word counting to tiktoken BPE encoding made `build_groups` ~25x slower (from ~0.6ms to ~17ms for either_src). The full pipeline is ~1.5x slower overall (still under 250ms for the largest fixture). If this becomes a bottleneck for larger codebases, consider: (a) a cheaper token approximation for scheduling with tiktoken only for final output counting, or (b) batch encoding optimizations.
- **Spurious truncation markers** — files/sections that aren't actually truncated sometimes show `…` markers. E.g. `CLAUDE.md` at Names stage shows `# Precis …`, `Cargo.toml` shows `[package …` / `[dependencies …`. The `…` should only appear when content was actually cut short, not on every rendered line.
- **Truncation marker indentation** — when a truncation marker (`…`) is emitted during Body(N) or Doc(N) stages, it should be indented to match the body/doc content level, not left at the signature's indentation. This makes nested content (e.g. struct fields, function bodies) read more naturally.

## Implementation notes

- **Doc comment detection** is partially migrated to tree-sitter. `Symbol.doc_start_line` is computed from AST sibling navigation, with text heuristic fallback. Handles `///`, `/**` (Rust), `/**` JSDoc (TypeScript), `//`/`/*` (Go, C), `#` (Python). Also handles wrapper nodes (`export_statement`, `decorated_definition`) for exported/decorated symbols. Python docstrings remain text-based heuristics. (Multi-line signature detection was also migrated to tree-sitter — `Symbol.sig_end_line` is computed from the AST body/block child, with text heuristic fallback for nodes without detectable body like type aliases and method signatures.)
- **Overload dedup** — consecutive function symbols with the same name are collapsed (keeping the last). Skipped for Go (`init()` functions).

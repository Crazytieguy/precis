# Issues

## Remaining work

- **Per-group stage value tuning** — the initial stage values (1.0, 0.7, 0.6, etc.) are starting points. Review snapshots across languages and tune per-kind values for better output quality.
## Implementation notes

- **Doc comment and multi-line signature detection** are text-based heuristics. Doc comments handle `///`, `//!`, `/** */`, Go `//` (godoc, language-gated), Python `#` and docstrings. Multi-line signatures scan for `{`/`;`/`:` delimiters. Worth re-evaluating tree-sitter for these as more languages are added.
- **Overload dedup** — consecutive function symbols with the same name are collapsed (keeping the last). Skipped for Go (`init()` functions).
- **Python doc truncation marker mismatch** — Python symbols with both pre-symbol `#` comments and post-signature docstrings concatenate them into a flat `doc_line_words` vector. The scheduler counts truncation markers based on the flat length, but the renderer emits the two sections separately with independent truncation. At certain `doc_n` values the marker count can differ by 1 word. Minor and rare in practice.

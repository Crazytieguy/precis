# Issues

## Architectural

- ~~**Scheduler/renderer split-brain**~~ — resolved. The scheduler and renderer now share a `SymbolLayout` struct (computed once per symbol) that captures all derived line ranges. Parent body ranges are truncated at the first child's doc_start, and TypeAlias signatures stop at `{` for complex types. No overlap between symbols, so `emitted_up_to` was removed entirely. Budget inflation hack also removed. Mean utilization: 90.5% (up from 88.5% pre-hack).

- **Heuristic proliferation** — priority adjustments have accumulated as ad-hoc code scattered across the codebase: file role detection, build script deprioritization, tool config detection, AI assistant files, community health files, translated docs, locale directories, badge stripping, HTML tag filtering, conventional source root skipping, heading depth grouping, `_prefix` privacy, restricted visibility, etc. Each is individually reasonable, but there's no single place to see all priority adjustments or reason about interactions between them. A more data-driven approach (e.g., a declarative pattern → adjustment table) would make the system more auditable and reduce the risk of conflicting heuristics.

## Remaining work

- ~~**Residual cost mismatch from emitted_up_to**~~ — resolved. Unified `SymbolLayout` eliminates the scheduler/renderer mismatch. No budget inflation needed.
- ~~**File paths as stages**~~ — done. FilePath is now the first stage in every progression. Groups at the FilePath stage show only the file path (no symbols), providing structural context at tight budgets.
- ~~**Import statement rendering**~~ — done. Imports are now extracted and rendered as their own `Import` kind category with stage progression `FilePath → Names → Signatures`. All languages supported (Rust `use`, TypeScript/JS `import`, Go `import`, Python `import`/`from`). Rust `pub use` re-exports are treated as public; all other imports are private.
- **Import 1st-party vs 3rd-party distinction** — imports could be split into 1st-party (relative paths, `crate::`, `super::`, Go module path) vs 3rd-party groups. 1st-party imports are higher signal for understanding a file's role. Requires a `is_first_party` field on `GroupKey` and per-language heuristics.
- ~~**Test/vendor/example directories**~~ — done. Test/benchmark/example files are now included in discovery but deprioritized (0.15x value factor). Vendor and testdata directories remain excluded. At tight budgets, test files appear as paths only; at generous budgets, their symbols render normally.
- **Per-group stage value tuning** — the initial stage values (1.0, 0.7, 0.6, etc.) are starting points. Review snapshots across languages and tune per-kind values for better output quality.
- **Config file support** — support json/yaml/toml files with different rendering heuristics than code.

## Implementation notes

- **Doc comment and multi-line signature detection** are text-based heuristics. Doc comments handle `///`, `//!`, `/** */`, Go `//` (godoc, language-gated), Python `#` and docstrings. Multi-line signatures scan for `{`/`;`/`:` delimiters. Worth re-evaluating tree-sitter for these as more languages are added.
- **Overload dedup** — consecutive function symbols with the same name are collapsed (keeping the last). Skipped for Go (`init()` functions).
- **Python doc truncation marker mismatch** — Python symbols with both pre-symbol `#` comments and post-signature docstrings concatenate them into a flat `doc_line_words` vector. The scheduler counts truncation markers based on the flat length, but the renderer emits the two sections separately with independent truncation. At certain `doc_n` values the marker count can differ by 1 word. Minor and rare in practice.

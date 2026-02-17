# Issues

## Known bugs

- **Cost accounting mismatch** — the scheduling algorithm's cost tracking diverges from actual rendered word counts. Some entries significantly exceed their budget (e.g., 390% at budget 500 for pluggy). The greedy algorithm deducts estimated costs, but the actual rendering can produce more words than estimated.
- **Python doc cost double-application** — the scheduler puts all doc lines (pre-symbol `#` comments + post-signature docstrings) into a single `doc_line_words` vector and budgets `Doc(N)` as N lines from it. But the renderer applies the N limit independently to both pre-symbol comments and docstrings — so `Doc(4)` can render up to 4 comment lines AND 4 docstring lines (8 total), double what was budgeted. This is a concrete source of cost overruns for Python files.

## Remaining work

- **Fix cost accounting** — the schedule algorithm's cost estimates must match actual rendered word counts. Until this is fixed, budget utilization metrics are unreliable.
- **Performance** — fixture tests take ~9 minutes in debug mode, ~55 seconds in release. Most fixture snapshot tests are commented out until this improves. Run tests in release mode (`cargo test --release`) until debug performance is addressed. Re-introduce fixture snapshots progressively as performance improves. This is high priority - we can't iterate if we can't run the tests quickly.
- **Performance benchmarking** — add benchmarks (e.g., `criterion`) for the scheduling hot path (`build_groups`, `schedule`, `render_scheduled`) to measure the impact of changes and identify optimization targets. Currently relying on wall-clock test times which are noisy and coarse.
- **Truncation markers** — `…` inline for truncated lines (e.g. name only instead of full signature), standalone `…` line for omitted content (e.g. remaining doc lines, body). Makes it visually clear what's summarized vs complete. The old rendering system had this; the new one doesn't yet.
- **File paths as stages** — currently file paths are shown automatically when any symbol content is included. Making file paths an explicit stage would allow showing just file paths for low-value groups at tight budgets, and would create cheap items that improve budget utilization by filling gaps at the end of scheduling.
- **Import statement rendering** — show imports as a new group kind with its own stage progression. Distinguish 1st-party imports (relative paths, `crate::`, `super::`, Go module path) from 3rd-party. 1st-party imports are high signal for understanding a file's role.
- **README.md priority boost** — render `README.md` at higher priority than other markdown files. Previously attempted with the old level-based system and was utilization-neutral. Should be revisited now that value is per-group.
- **Test/vendor/example directories** — instead of excluding entirely during file discovery, render at lower priority. They appear as paths at tight budgets, content at generous budgets.
- **Per-group stage value tuning** — the initial stage values (1.0, 0.7, 0.6, etc.) are starting points. Review snapshots across languages and tune per-kind values for better output quality.
- **Config file support** — support json/yaml/toml files with different rendering heuristics than code.

## Implementation notes

- **Doc comment and multi-line signature detection** are text-based heuristics. Doc comments handle `///`, `//!`, `/** */`, Go `//` (godoc, language-gated), Python `#` and docstrings. Multi-line signatures scan for `{`/`;`/`:` delimiters. Worth re-evaluating tree-sitter for these as more languages are added.
- **Overload dedup** — consecutive function symbols with the same name are collapsed (keeping the last). Skipped for Go (`init()` functions).

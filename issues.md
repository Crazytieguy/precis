# Issues

## Testing infrastructure

~210 of ~216 fixture snapshot tests are commented out in `tests/snapshots.rs` (lines 591–865). Only 6 fixture tests are active. Until these are re-introduced, changes to scheduling, rendering, or value heuristics have almost no regression coverage on real codebases.

**Current performance:** ~45 seconds in debug mode, ~2 seconds in release. Always run tests in release mode (`cargo test --release`). Do not run tests in debug mode until all fixture tests are re-introduced and the full suite runs quickly in release.

**Next steps:**
1. ~~Add criterion benchmarking infrastructure~~ Done — `cargo bench --bench hot_path` benchmarks `extract_all_symbols`, `build_groups`, `schedule`, and `render_with_budget` across three fixture sizes (either/src, pluggy/src/pluggy, commander/lib). Use `--quick` flag for fast runs (5 samples).
2. ~~Profile and optimize schedule hot path~~ Done — the greedy loop's update step iterated all groups on every inclusion. Two fixes: (a) file→groups reverse index to only visit groups sharing files with the included item, (b) track *newly shown* files so groups are only re-pushed when their file_path_cost actually changes. Result: schedule is 4-19x faster (either: 500ms→26ms, pluggy: 115ms→18ms, commander: 1.3s→318ms).
3. Continue optimizing — commander_lib schedule is still 318ms. The remaining cost is likely from re-pushing all items in the same group on every prereq change, and from groups that share many files (single-directory codebases). Consider caching per-group max_n, precomputing stage positions, or batch-updating prereqs.
4. Re-introduce remaining fixture snapshots as performance improves
5. Once performance is much better, re-introduce all remaining snapshots

## Known bugs

- **Cost accounting mismatch** — ~~the scheduling algorithm's cost tracking diverges from actual rendered word counts. Some entries significantly exceed their budget (e.g., 418% at budget 500 for pluggy).~~ The major source of cost divergence was a **double-deduction bug**: when a high-level item (e.g., Doc(3)) was included before a lower-level item (e.g., Names), the high-level item paid for Names as a prerequisite, but the original Names heap entry retained a valid generation number. Popping that stale entry deducted its cost again, wasting budget. Two fixes: (a) skip queue items whose stage is already covered by the group's current included stage, (b) when computing prerequisites for a later stage, correctly handle partially-included prerequisite stages (e.g., when included at Doc(2), don't skip all Doc lines — only skip Doc(1) and Doc(2)). Budget utilization improved from mean 49% to 86%. Remaining small divergences (1-5%) come from the renderer's `emitted_up_to` dedup skipping content that was charged for, particularly when parent symbol bodies overlap with nested symbol ranges.

## Remaining work

- **Residual cost mismatch from emitted_up_to** — the renderer skips symbols whose start line falls within already-emitted ranges (e.g., nested symbols inside trait/impl/function bodies). The scheduler charges for these symbols but they don't render. This causes small overestimation (1-5%). A proper fix would either exclude overlapping body lines from parent symbol costs or teach the scheduler about line-level deduplication.
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

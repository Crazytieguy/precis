# Issues

## Planned refactor: per-symbol rendering

The current implementation uses an "effective level" (nominal level minus penalties) to select a single rendering policy per file. This conflates levels (the parameter the binary search sweeps over) with rendering policies (the set of content decisions). All symbols in a file get the same treatment.

The new approach: each symbol has an independent content progression (hidden → name → signature → first-line doc → full doc → body), and transitions between stages are scheduled per-symbol based on cost and value. See README.md for the full design. Implementation details TBD.

## Feature development

- **Budget-based testing (complete)** — all fixture snapshot tests use `--budget` at standardized values (500, 1000, 2000, 4000). Budget snapshots are stable across level refactors and directly test the user-facing behavior. The `--level` flag exists for debugging, not as a primary test parameter. Inline sample tests still use specific levels since they test the rendering function directly.
- **Tree-sitter reuse in tests (complete)** — the monotonicity test extracts symbols once per file and reuses them across all levels via `render_file_with_symbols`.
- **Import statements** — show at a low level. Distinguish 1st-party imports (relative paths like `./`, `../`; Rust `crate::`, `super::`; Go module path) from 3rd-party. 1st-party imports are high signal for understanding a file's role; 3rd-party are lower priority.
- **Test/vendor/example directories** — instead of excluding entirely during file discovery, render at lower priority (transitions happen at higher levels). They appear as paths at tight budgets, content at generous budgets. More honest than invisible filtering.
- **README boost** — render `README.md` at higher priority than other markdown files. Attempted with the old penalty system: +1/+2 bonus was utilization-neutral or slightly worse. Should be revisited with per-symbol scheduling where finer-grained level control may make it net-positive.
- **File omission** — at low levels, the render function can return nothing for some files (e.g. deep, large, or low-priority files). Useful for repos with many files where showing all paths at level 0 is already too much.
- **Config file support** — support json/yaml/toml files with different rendering heuristics than code.
- **Visibility-aware rendering (partial)** — symbol names, signatures, doc comments, and type body expansion are all visibility-gated. In the new model, visibility becomes a factor in per-symbol value: public symbols transition earlier than private ones. Python dunder methods (`__init__`, `__repr__`, `__all__`, etc.) are treated as public.
- **Truncation markers** — `…` inline for truncated lines (e.g. name only instead of full signature), standalone `…` line for omitted content (e.g. remaining doc lines). Not yet implemented.
- **Re-evaluate tree-sitter for doc comments and multi-line signatures** — both are currently text-based heuristics (scanning for comment prefixes, scanning for `{`/`;`/`:`). Tree-sitter grammars model comments as sibling nodes (not children of declarations), making association with symbols harder via queries alone. Text heuristics work uniformly across languages. Worth re-evaluating as more languages are added.

## Current state

- Parsing: Rust, TypeScript, JavaScript, TSX, Python, Go, Markdown. 13 granularity levels (0–12). `--budget`, `--level`, `--json` flags. Defaults to current directory when no path given.
- File discovery filters out test/benchmark/vendor/example directories and test file patterns. Uses relative paths so parent directories don't trigger false positives.
- 31 test fixtures across all supported languages with budget-based snapshot tests (276 budget snapshots at standardized budgets: 500, 1000, 2000, 4000). All fixtures tested at source directory, root-level, and subdirectory entrypoints. Monotonicity invariant tested across all fixtures including root-level targets. Inline sample tests cover levels 0–12 across all supported languages.
- `cargo run --bin budget_util` measures budget utilization across all budget snapshots.
- Rendering uses the old per-file "effective level" model (see planned refactor above). The per-symbol scheduling described in README.md is the target design.

## Implementation notes

These notes describe the current implementation. Many will be superseded by the per-symbol refactor.

- Doc comment and multi-line signature detection are text-based heuristics (see feature development for re-evaluation issue). Doc comments handle `///`, `//!`, `/** */`, Go `//` (godoc, language-gated), Python `#` and docstrings. Multi-line signatures scan for `{`/`;`/`:` delimiters.
- Overload dedup: consecutive function symbols with the same name are collapsed (keeping the last). Skipped for Go (`init()` functions).
- Type aliases show their full definition (the definition IS the signature) at any level where signatures are shown. Uses `emitted_up_to` to avoid duplicating lines when type alias ranges encompass nested symbols.
- Markdown: headings are symbols, body text is "doc content." Consecutive levels can produce identical output (heading text is already the full line content). These levels still help budget utilization because the binary search can land on them when the next level would overshoot.
- **Penalty design lesson** — subtractive penalties that reduce output at existing levels tend to hurt utilization because the binary search can't always compensate. Additive changes that introduce new content at existing levels can push totals over budget, causing level drops. The safest level additions are ones that insert truly NEW levels (bump MAX_LEVEL) rather than modifying output at existing levels. The per-symbol scheduling approach should largely resolve this by spreading transitions smoothly.

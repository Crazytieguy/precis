# precis

A CLI tool that extracts a token-efficient summary of a codebase.

## Design

This section describes the target design. See `issues.md` for current implementation status and remaining work.

### Parsing

Uses **tree-sitter** for language-agnostic symbol extraction. Each supported language has a small query file (`.scm`) that maps language-specific node types to a common symbol model (functions, types, modules, exports, etc.).

### Token Budgeting

Takes a `--budget` flag (in words) or a `--level` flag to select a specific granularity level directly. The two flags are mutually exclusive.

**Granularity function:** A single function `render(level, path, content) -> output` maps each file to its output at a given granularity level. The level is a single global integer parameter; the function uses file properties (depth, size, file type, name patterns) to make per-file rendering decisions. For example, a shallow small file may show full signatures at a given level while a deep large file shows only symbol names — or nothing at all. The function is free to make arbitrary per-file decisions as long as the monotonicity invariant holds. A `MAX_LEVEL` constant defines the highest available level.

The current implementation computes an "effective level" by subtracting depth and size penalties from the nominal level. This is a convenient shortcut but not a hard constraint — the render function could use any logic as long as output is monotonically non-decreasing per file path across levels.

**Monotonicity invariant:** For any given file path and content, a higher level must never produce fewer words than a lower level. This is tested against fixture files across all levels.

**Budget algorithm:** Binary search over `0..=MAX_LEVEL` to find the highest level where the total word count across all files fits within the budget.

**Rendering policies:** The render function combines several policies depending on the level and file properties: showing file paths only, symbol names (truncated to identifier), full multi-line signatures (parameters, `where` clauses, return types), first-line doc comments for public symbols only (a single summary line per symbol), full doc comments for public symbols only, full doc comments for all symbols (`///`, `/** */`, docstrings), type definition bodies for public types only (struct fields, enum variants, trait/interface/class members), type definition bodies for all types, and full source. Markdown files use headings as symbols, with paragraph content at higher levels. At any given level, different files may receive different policies — a shallow file might show full signatures while a deep file shows only symbol names or nothing at all. Property-based decisions further differentiate files: for example, files with verbose doc comments (high average doc lines per symbol) delay showing docs relative to files with concise docs at the same nominal level.

Shallower and smaller files are prioritized over deeper and larger ones when budget is tight. Depth penalty reduces the effective level by `depth/2` (where depth counts directory components). Size penalty reduces the effective level by 1 for files with 1000+ lines, but only when the depth-adjusted level is 3+. Users can zoom into subdirectories by running the tool on them directly with a larger budget.

**Expanding levels:** Targeting ~32 levels for smoother budget curves. There are three sources of new levels, which complement each other:

1. **New rendering policies** — adding genuinely new output types (e.g. import statements, visibility-gated symbols). Each new policy creates intermediate content between existing policies.
2. **Finer property-based thresholds** — making different choices among existing policies based on file properties (file type, name patterns, content characteristics like symbol count or density). For example, a JSON config file might stay at "paths only" longer than a Rust file at the same depth; a file with many small functions is more expensive to show signatures for than a file with few large functions, so it transitions to signatures at a higher level.
3. **Spreading transitions across levels** — with more levels and finer-grained penalties, different files transition between policies at different nominal levels, so fewer files upgrade simultaneously at each step. This multiplies the effect of the other two sources.

Adding levels should be incremental: add a new policy or tuning improvement, bump `MAX_LEVEL` by however many levels the change needs, measure utilization.

### Measuring Quality

Two dimensions of output quality:

1. **Budget utilization** — how well the tool uses the available word budget. Measurable: `actual_words / budget`. With few discrete levels, utilization can be poor (large gaps between adjacent levels). More levels improve utilization. Run `cargo run --bin budget_util` to measure utilization across all budget snapshots.

2. **Output quality** — whether the content shown is actually useful for understanding the codebase. Not directly measurable. Showing low-value content (junk) can *improve* budget utilization while making the output worse. Both dimensions matter: high utilization with high-quality content is the goal.

The improvement loop: run the budget utilization script, identify snapshots with low utilization, examine the snapshot and the next level's output, design rendering improvements that fill the gap with useful content. Improvements must not overfit to a specific fixture: each change must have reasoning for why it applies broadly to many codebases. Prefer improvements parameterized by file properties (depth, size, type, name patterns) over fixture-specific tweaks. Verify that mean utilization improves without regressions — otherwise drop the change.

Note: `budget_util` reads snapshot files, not live output. Run `cargo test` (or `cargo insta test`) to update snapshots before running `budget_util`, otherwise it reports stale data.

### Output Format

Plain text, optimized for readability and token efficiency. A `--json` flag outputs structured JSON with per-file entries.

**Line-prefix constraint:** Each output line is a prefix of the actual line in the source file, preserving original whitespace and indentation. The tool extracts from the source rather than synthesizing new representations — no cross-language normalization of keywords. This means nesting (e.g. methods inside a Rust `impl` block) is represented naturally by the source's own indentation.

Line numbers use a right-aligned format with an arrow separator (e.g. `    12→    pub fn new`). Token counting uses word count (not tokenizer or character count).

## Supported Languages

- **Rust** — functions, structs, enums, traits, impls, type aliases, consts, statics, macros, modules
- **TypeScript / JavaScript / TSX** (`.ts`, `.tsx`, `.js`, `.jsx`, `.mts`, `.cts`, `.mjs`, `.cjs`) — functions, classes, interfaces, enums, type aliases, consts, namespaces
- **Go** — functions, methods, structs, interfaces, type aliases, consts, vars (exported = uppercase)
- **Python** — functions, classes, module-level constants (typed, UPPER_CASE, dunder)
- **Markdown** — ATX headings (`#`, `##`, etc.) and setext headings as section structure

Per-language support requires a tree-sitter grammar and a query file defining:
- Which node types count as symbols
- How to extract signatures
- What signals "public" (e.g., `export` in JS, `pub` in Rust, capitalization in Go)

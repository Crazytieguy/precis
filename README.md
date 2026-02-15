# precis

A CLI tool that extracts a token-efficient summary of a codebase.

## Design

See `issues.md` for current implementation status and remaining work.

### Parsing

Uses **tree-sitter** for language-agnostic symbol extraction. Each supported language has a small query file (`.scm`) that maps language-specific node types to a common symbol model (functions, types, modules, exports, etc.).

### Token Budgeting

Takes a `--budget` flag (in words). A `--level` flag exists for debugging but is not the primary interface.

**Per-symbol progression:** Each symbol in a file has a natural content progression, where each stage strictly adds to the previous:

1. Hidden (file path shown, symbol omitted)
2. Name only (truncated to identifier)
3. Full signature (parameters, return types, `where` clauses)
4. First-line doc comment
5. Full doc comment
6. Body (struct fields, enum variants, function body)

Dependencies are implicit: showing a doc comment implies showing the signature and name.

**Per-symbol scheduling:** At a given level, different symbols — even within the same file — can be at different stages of this progression. The stage assigned to each symbol depends on:

- **Cost** — how many words this stage adds for this specific symbol. A function with a 50-line docstring is expensive to upgrade to "full doc"; a function with a 1-line docstring is cheap.
- **Value** — how useful this content is. Public symbols are more valuable than private ones. Depth, file type, and language conventions also factor in.

Symbols with cheap, high-value upgrades transition to later stages at lower levels. Expensive or low-value upgrades are deferred to higher levels. This means at any given level, the tool shows the most useful content that fits in the budget.

**Monotonicity invariant:** For any given file path and content, a higher level must never produce fewer words than a lower level. Per-symbol progression guarantees this: stages only add content, and symbols only advance (never regress) as the level increases. This is tested against fixture files across all levels.

**Budget algorithm:** Binary search over `0..=MAX_LEVEL` to find the highest level where the total word count across all files fits within the budget. A `MAX_LEVEL` of 64 provides smooth budget curves — per-symbol scheduling spreads transitions across many levels, so the total word count grows gradually rather than in large jumps.

**Smooth growth:** The goal is roughly geometric growth in word count per file across levels. Expensive upgrades are spaced further apart (a symbol might stay at "signature" for several levels before its costly docstring is added), while cheap upgrades are introduced early. This means most budget values land on a level with good utilization.

### Measuring Quality

Two dimensions of output quality:

1. **Budget utilization** — how well the tool uses the available word budget. Measurable: `actual_words / budget`. With smooth per-symbol transitions across many levels, the binary search can find a level close to any budget. Run `cargo run --bin budget_util` to measure utilization across all budget snapshots.

2. **Output quality** — whether the content shown is actually useful for understanding the codebase. Not directly measurable. Showing low-value content (junk) can *improve* budget utilization while making the output worse. Both dimensions matter: high utilization with high-quality content is the goal.

The improvement loop: run the budget utilization script, identify budget tiers with poor utilization across many fixtures, examine the output, and design scheduling improvements. Improvements must not overfit to a specific fixture: each change must have reasoning for why it applies broadly to many codebases. Verify that mean utilization improves without regressions — otherwise drop the change.

Note: `budget_util` reads snapshot files, not live output. Run `cargo test` (or `cargo insta test`) to update snapshots before running `budget_util`, otherwise it reports stale data.

### Output Format

Plain text, optimized for readability and token efficiency. A `--json` flag outputs structured JSON with per-file entries.

**Line-prefix constraint:** Each output line is a prefix of the actual line in the source file, preserving original whitespace and indentation. The tool extracts from the source rather than synthesizing new representations — no cross-language normalization of keywords. This means nesting (e.g. methods inside a Rust `impl` block) is represented naturally by the source's own indentation.

**Truncation markers:** When content is truncated, the output makes this visible:
- A line truncated to a prefix (e.g. name only instead of full signature) ends with `…`
- Omitted lines (e.g. remaining doc comment lines, body) are shown as a standalone `…` on its own line, with no line number

This lets readers instantly see whether they're looking at complete content or a summary.

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

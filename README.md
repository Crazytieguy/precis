# precis

A CLI tool that extracts a token-efficient summary of a codebase.

## Design

This section describes the target design. See `issues.md` for current implementation status and remaining work.

### Parsing

Uses **tree-sitter** for language-agnostic symbol extraction. Each supported language has a small query file (`.scm`) that maps language-specific node types to a common symbol model (functions, types, modules, exports, etc.).

### Token Budgeting

Takes a `--budget` flag (in words).

**Granularity function:** A single function `render(level, path, content) -> output` maps each file to its output at a given granularity level. The function is depth-aware (via `path`) and file-size-aware (via `content`), so a single level can behave differently for shallow vs deep files or small vs large files. A `MAX_LEVEL` constant defines the highest available level.

**Monotonicity invariant:** For any given file, a higher level must never produce fewer words than a lower level. This is tested against fixture files across all levels.

**Budget algorithm:** Binary search over `0..=MAX_LEVEL` to find the highest level where the total word count across all files fits within the budget.

**Starting levels** (expected to evolve — levels differ in which lines are included and how much of each line is shown, but all file content is line-prefixes with line numbers):

0. File paths only
1. Symbol lines, truncated to symbol name (e.g. `pub fn new`)
2. Symbol lines, full line-prefix including signature (e.g. `pub fn new(lang: Language) -> Self {`)
3. Symbol lines with preceding doc comments (`///` in Rust, `/** */` JSDoc in JS/TS)
4. Like level 3, but type definition bodies (struct fields, enum variants, trait/interface members) shown in full
5. Full source (all lines)

More intermediate levels can be added over time (e.g. multi-line signatures).

Shallower files are prioritized over deeper ones when budget is tight. Users can zoom into subdirectories by running the tool on them directly with a larger budget.

### Output Format

Plain text, optimized for readability and token efficiency.

**Line-prefix constraint:** Each output line is a prefix of the actual line in the source file, preserving original whitespace and indentation. The tool extracts from the source rather than synthesizing new representations — no cross-language normalization of keywords. This means nesting (e.g. methods inside a Rust `impl` block) is represented naturally by the source's own indentation.

Line numbers use a right-aligned format with an arrow separator. At name-only level:

```
src/parser.rs
     1→impl Parser
    12→    pub fn new
    45→    pub fn parse
```

At signature level:

```
src/parser.rs
     1→impl Parser {
    12→    pub fn new(lang: Language) -> Self {
    45→    pub fn parse(&self, source: &str) -> Tree {
```

A `--json` flag may be added later for machine consumption.

## Supported Languages

Per-language support requires a tree-sitter grammar and a query file defining:
- Which node types count as symbols
- How to extract signatures
- What signals "public" (e.g., `export` in JS, `pub` in Rust, capitalization in Go)

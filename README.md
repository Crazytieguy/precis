# precis

A CLI tool that extracts a token-efficient summary of a codebase.

## Design

See `issues.md` for current implementation status and remaining work.

### Parsing

Uses **tree-sitter** for language-agnostic symbol extraction. Each supported language has a small query file (`.scm`) that maps language-specific node types to a common symbol model (functions, types, modules, exports, etc.).

### Token Budgeting

Takes a `--budget` flag (in words, default 1000).

**Groups:** Symbols are bucketed into groups by shared properties: **visibility** (public/private), **kind category** (Function, Type, Constant, Module, Section, Macro, Impl), **parent directory**, **file extension**, and **documented** (has doc comment or not). All symbols in a group always receive the same rendering treatment — this prevents confusing output where similar symbols are at different detail levels.

**Per-kind stage progressions:** Each group has an ordered progression of rendering stages. Different kind categories have different progressions:

- **Functions / Constants / Modules / Macros / Impl:** Names → Signatures → Doc(N) → Body(N)
- **Types (struct, enum, trait, interface, class):** Names → Signatures → Body(N) → Doc(N) — body before doc because struct fields and enum variants ARE the useful content
- **Sections (markdown headings):** Names → Body(N)

Doc(N) and Body(N) are continuous: each additional line is a separate scheduling item. A group might show 3 lines of body before the next line becomes too expensive relative to other available content.

**File paths** are not a stage. They are shown automatically when any content from a file is included. The cost of a file path line is absorbed into the first item that includes content from that file.

**Value and cost:** Each (group, stage) item has a value and a cost. Value comes from composable heuristic factors: visibility (public > private), documentation status, file depth, and per-kind stage values. Cost is the measured word count. The value computation takes the full group (not just the key), so heuristics can reference aggregate properties like symbol count.

**Greedy scheduling:** A priority queue ranks all (group, stage) items by `priority = (own_value + unmet_prerequisite_values) / (own_cost + unmet_prerequisite_costs)`. The algorithm greedily includes the highest-priority item, deducts its cost from the budget, updates affected items (prerequisites now met, file paths now shown for shared files), and repeats until nothing fits.

**Monotonicity:** More budget → lower priority cutoff → more items included → more words. Automatic — no special invariant enforcement needed.

**Extensibility:** The scheduling system has several extension points:

- **Value factors** — add a function that adjusts group value based on any property (symbol count, file role, etc.). No architectural changes needed.
- **Grouping dimensions** — add fields to GroupKey to split groups more finely (or merge to coarsen).
- **Kind categories** — add KindCategory variants with their own stage progressions.
- **Stage types** — add new stages beyond Names/Signatures/Doc/Body.
- **Stage progression ordering** — change per-kind orderings (e.g., body before doc for types).
- **Stage granularity** — finer-grained stages create more items in the priority queue, improving budget utilization by filling gaps at the end of scheduling.

### Measuring Quality

Two dimensions of output quality:

1. **Budget utilization** — how well the tool uses the available word budget. Measurable: `actual_words / budget`. `cargo run --bin budget_util` measures utilization across all budget snapshots (run `cargo test` first to update snapshots).

2. **Output quality** — whether the content shown is actually useful for understanding the codebase. Not directly measurable. Showing low-value content (junk) can *improve* budget utilization while making the output worse. Both dimensions matter: high utilization with high-quality content is the goal.

The improvement loop: inspect snapshots (both low- and high-utilization), hypothesize a broadly applicable improvement, implement it, and verify across fixtures. Changes must not overfit to a specific fixture — each change needs reasoning for why it applies to many codebases.

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

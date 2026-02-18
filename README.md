# precis

A CLI tool that extracts a token-efficient summary of a codebase.

## Design

See `issues.md` for current implementation status and remaining work.

### Parsing

Uses **tree-sitter** for language-agnostic symbol extraction. Each supported language has a small query file (`.scm`) that maps language-specific node types to a common symbol model (functions, types, modules, exports, etc.).

### Token Budgeting

Takes a `--budget` flag (in words, default 1000).

**Groups:** Symbols are bucketed into groups by shared properties (visibility, kind, directory, file role, etc.). All symbols in a group always receive the same rendering treatment — this prevents confusing output where similar symbols are at different detail levels.

**Per-kind stage progressions:** Each group has an ordered progression of rendering stages. Different kind categories have different progressions:

- **Functions / Constants / Modules / Macros / Impl:** FilePath → Names → Signatures → Doc(N) → Body(N)
- **Types (struct, enum, trait, interface, class):** FilePath → Names → Signatures → Body(N) → Doc(N) — body before doc because struct fields and enum variants ARE the useful content
- **Sections (markdown headings):** FilePath → Names → Body(N)

FilePath is the cheapest stage — it shows just the file path with no symbol content. At tight budgets, low-priority groups may only reach FilePath, providing structural context about what files exist without consuming budget on symbol details.

Doc(N) and Body(N) are continuous: each additional line is a separate scheduling item. A group might show 3 lines of body before the next line becomes too expensive relative to other available content.

**Value and cost:** Each (group, stage) item has a value and a cost. Value comes from composable heuristic factors (visibility, documentation status, file role, etc.). Cost is the measured word count.

**Greedy scheduling:** A priority queue ranks all (group, stage) items by `priority = (own_value + unmet_prerequisite_values) / (own_cost + unmet_prerequisite_costs)`. The algorithm greedily includes the highest-priority item, deducts its cost from the budget, updates affected items, and repeats until nothing fits.

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
- **C** (`.c`, `.h`) — functions, structs, unions, enums, typedefs, macros, includes (static = private, `_`-prefix = private)
- **Python** — functions, classes, module-level constants (typed, UPPER_CASE, dunder)
- **Markdown** — ATX headings (`#`, `##`, etc.) and setext headings as section structure
- **JSON** (`.json`) — top-level object keys as sections (lockfiles excluded)
- **TOML** (`.toml`) — section headers (`[section]`, `[[array]]`)
- **YAML** (`.yaml`, `.yml`) — top-level keys as sections (lockfiles excluded)

Code languages and JSON use tree-sitter grammars with query files defining which node types count as symbols, how to extract signatures, and what signals "public". Config file formats (TOML, YAML) use text-based heuristics to extract top-level structure as sections.

## Development

**Dogfooding:** Use precis itself for codebase exploration during development (e.g. `cargo run -- .` to get an overview before making changes). The main intended use case is replacing general-purpose "explore the codebase" workflows with a structured summary. If precis output is inconvenient for a task, that's a signal the tool can be improved.

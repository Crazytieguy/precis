# precis

A CLI tool that extracts a token-efficient summary of a codebase, designed to give AI coding agents fast structural context without reading every file.

## Usage

```
precis ./src              # summarize a directory
precis ./src --budget 4000  # with a larger token budget
precis main.rs            # summarize a single file
precis . --json           # structured JSON output
```

The default budget is 2000 BPE tokens (o200k_base tokenizer). Output is plain text with line numbers and source-faithful indentation.

## How it works

Uses **tree-sitter** for language-agnostic symbol extraction. Each supported language has a query file (`.scm`) mapping syntax nodes to a common symbol model. Symbols are grouped and scheduled into a token budget using a greedy value/cost priority queue.

## Supported languages

- **Rust** — functions, structs, enums, traits, impls, type aliases, consts, statics, macros, modules
- **TypeScript / JavaScript / TSX** — functions, classes, interfaces, enums, type aliases, consts, namespaces
- **Go** — functions, methods, structs, interfaces, type aliases, consts, vars
- **C** — functions, structs, unions, enums, typedefs, macros, includes
- **Python** — functions, classes, module-level constants
- **Markdown** — heading structure
- **JSON / TOML / YAML** — top-level keys and sections

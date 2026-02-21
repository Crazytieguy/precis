# precis

A CLI tool that extracts a token-efficient summary of a codebase, designed to give AI coding agents fast structural context without reading every file.

## Install

```
cargo install precis
```

Or with Homebrew:

```
brew install Crazytieguy/tap/precis
```

Or with the install script:

```
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/Crazytieguy/precis/releases/latest/download/precis-installer.sh | sh
```

## Usage

```
precis .                   # summarize the current directory
precis ./src --budget 4000 # with a larger token budget
```

The default budget is 2000 BPE tokens (o200k_base tokenizer). Output is plain text with line numbers preserving source indentation.

### Give your AI agent codebase context

Add this to your `CLAUDE.md` or `AGENTS.md`:

```markdown
## Codebase exploration

Always use `precis` for codebase exploration. Run `precis .` for a full overview, or `precis src/some/directory` to zoom into a specific area.
```

## How it works

Uses **tree-sitter** for language-agnostic symbol extraction. Each supported language has a query file (`.scm`) mapping syntax nodes to a common symbol model. Symbols are grouped and scheduled into a token budget using a greedy value/cost priority queue — public API surfaces, type definitions, and documentation get priority over private implementation details.

## Supported languages

- **Rust** — functions, structs, enums, traits, impls, type aliases, consts, statics, macros, modules
- **TypeScript / JavaScript / TSX** — functions, classes, interfaces, enums, type aliases, consts, namespaces
- **Go** — functions, methods, structs, interfaces, type aliases, consts, vars
- **C** — functions, structs, unions, enums, typedefs, macros, includes
- **Python** — functions, classes, module-level constants
- **Markdown** — heading structure with body content
- **JSON / TOML / YAML** — top-level keys and sections
- **Plain text fallback** — files in ~40 additional languages (C++, Java, Kotlin, Lua, Ruby, Swift, etc.) are included as plain text

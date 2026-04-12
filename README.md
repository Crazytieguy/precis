# precis

A CLI tool that extracts a token-efficient summary of a path, designed to replace most Explore agent use with a single fast command. It uses tree-sitter to parse source files, extracts structural symbols (functions, types, interfaces, headings), and ranks them by importance to fit within a token budget.

## Example

Here's what `precis` shows for [developit/mitt](https://github.com/developit/mitt), a tiny TypeScript event emitter, at a 400-token budget:

<!-- precis-example-start -->
```
README.md
     9→# Mitt
    30→## Install
    56→## Usage
    81→### Typescript
   113→## Examples & Demos
   123→## API
   138→### mitt
   144→### all
   148→### on
   152→#### Parameters
   157→### off
   162→#### Parameters
   167→### emit
   174→#### Parameters
   184→### Reporting Issues
   189→### Submitting pull requests

src/index.ts
     1→export type EventType …
     5→export type Handler …
     6→export type WildcardHandler …
    12→export type EventHandlerList …
    13→export type WildCardEventHandlerList …
    18→export type EventHandlerMap …
    23→export interface Emitter …
    26→	on<Key extends keyof Events>(type: Key, handler: Handler<Events[Key]>): void;
    27→	on(type: '*', handler: WildcardHandler<Events>): void;
    29→	off<Key extends keyof Events>(
    33→	off(type: '*', handler: WildcardHandler<Events>): void;
    35→	emit<Key extends keyof Events>(type: Key, event: Events[Key]): void;
    36→	emit<Key extends keyof Events>(
    42→ * Mitt: Tiny (~200b) functional event emitter / pubsub.
    43→ * @name mitt
    44→ * @returns {Mitt}
    46→export default function mitt<Events extends Record<EventType, unknown>>(
    47→	all?: EventHandlerMap<Events>
    48→): Emitter<Events> {

package.json

tsconfig.json

test/
```
<!-- precis-example-end -->

The README headings give the document structure. The source file shows type aliases (truncated), the `Emitter` interface with its method signatures (`on`, `off`, `emit`), doc comments, and the main function signature. Config files and test directories appear as path-only entries, deprioritized relative to core source.

## Installation

### Claude Code plugin (recommended)

The precis plugin automatically injects a structural overview of your project into Claude's context at the start of every session. This eliminates the need for long, manually-maintained `CLAUDE.md` files describing your codebase, and removes the overhead of Explore agents. The plugin also gives Claude the `precis` CLI so it can zoom into specific directories on demand.

```
claude plugin marketplace add Crazytieguy/precis
claude plugin install precis
```

Or add to your `.claude/settings.json` manually:

```json
{
  "enabledPlugins": {
    "precis@precis": true
  },
  "extraKnownMarketplaces": {
    "precis": {
      "source": {
        "source": "github",
        "repo": "Crazytieguy/precis"
      }
    }
  }
}
```

The plugin automatically downloads and updates the binary — no manual install needed.

### Standalone CLI

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

#### Configure your AI agent

Add this to your `CLAUDE.md`, `AGENTS.md`, or equivalent:

```markdown
## Codebase exploration

Always use `precis` for codebase exploration. Run `precis .` for a full overview, or `precis src/some/directory` to zoom into a specific area.
```

## Usage

```
precis .                    # summarize the current directory
precis ./src                # zoom into a subdirectory
precis . --budget 8000      # with a larger token budget
```

The default budget is 4000 BPE tokens (o200k_base tokenizer). Output is plain text with line numbers preserving source indentation.

## Supported languages

- **Rust** — functions, structs, enums, traits, impls, type aliases, consts, statics, macros, modules
- **TypeScript / JavaScript / TSX** — functions, classes, interfaces, enums, type aliases, consts, namespaces
- **Go** — functions, methods, structs, interfaces, type aliases, consts, vars
- **C / C++** — functions, classes, structs, unions, enums, namespaces, typedefs, type aliases, macros, includes
- **Java** — classes, interfaces, enums, records, annotations, methods, constructors, constants, modules
- **Python** — functions, classes, module-level constants
- **Markdown** — heading structure with body content
- **JSON / TOML / YAML** — top-level keys and sections
- **Lua** — functions, module-level variables
- **Plain text fallback** — files in any other language are included as plain text (binary files excluded automatically)

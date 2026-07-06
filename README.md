# precis

A CLI tool that extracts a token-efficient summary of a path, designed to replace most Explore agent use with a single fast command. It uses tree-sitter to parse source files, extracts structural symbols (functions, types, interfaces, headings), and ranks them by importance to fit within a token budget.

## Example

Here's what `precis` shows for [developit/mitt](https://github.com/developit/mitt), a tiny TypeScript event emitter, at a 900-token budget:

<!-- precis-example-start -->

```
.editorconfig
.eslintrc
.github/
.gitignore
LICENSE
README.md
    …
       9→# Mitt
      10→
      11→> Tiny 200b functional event emitter / pubsub.
      12→
      13→-   **Microscopic:** weighs less than 200 bytes gzipped
      14→-   **Useful:** a wildcard `"*"` event type listens to all events
      15→-   **Familiar:** same names & ideas as [Node's EventEmitter](https://nodejs.org/api/events.html#events_class_eventemitter)
      16→-   **Functional:** methods don't rely on `this`
      17→-   **Great Name:** somehow [mitt](https://npm.im/mitt) wasn't taken
      18→
      19→Mitt was made for the browser, but works in any JavaScript runtime. It has no dependencies and supports IE9+.
      21→## Table of Contents
    …
      30→## Install
    …
      56→## Usage
    …
      81→### Typescript
    …
     113→## Examples & Demos
    …
     123→## API
    …
     138→### mitt
    …
     144→### all
    …
     148→### on
    …
     157→### off
    …
     167→### emit
    …
     179→## Contribute
    …
     184→### Reporting Issues
    …
     189→### Submitting pull requests
    …
     203→## License
    …
package.json
    …
       2→  "name": "mitt",
       3→  "version": "3.0.1",
       4→  "description": "Tiny 200b functional Event Emitter / pubsub.",
    …
      41→  "license": "MIT",
    …
src/
    index.ts
           1→export type EventType = string | symbol;
        …
           5→export type Handler<T = unknown> = (event: T) => void;
           6→export type WildcardHandler<T = Record<string, unknown>> = (
           7→	type: keyof T,
           8→	event: T[keyof T]
           9→) => void;
        …
          12→export type EventHandlerList<T = unknown> = Array<Handler<T>>;
          13→export type WildCardEventHandlerList<T = Record<string, unknown>> = Array<
          14→	WildcardHandler<T>
          15→>;
        …
          18→export type EventHandlerMap<Events extends Record<EventType, unknown>> = Map<
          19→	keyof Events | '*',
          20→	EventHandlerList<Events[keyof Events]> | WildCardEventHandlerList<Events>
          21→>;
          23→export interface Emitter<Events extends Record<EventType, unknown>> {
          24→	all: EventHandlerMap<Events>;
          25→
          26→	on<Key extends keyof Events>(type: Key, handler: Handler<Events[Key]>): void;
          27→	on(type: '*', handler: WildcardHandler<Events>): void;
          28→
          29→	off<Key extends keyof Events>(
          30→		type: Key,
          31→		handler?: Handler<Events[Key]>
          32→	): void;
          33→	off(type: '*', handler: WildcardHandler<Events>): void;
          34→
          35→	emit<Key extends keyof Events>(type: Key, event: Events[Key]): void;
          36→	emit<Key extends keyof Events>(
          37→		type: undefined extends Events[Key] ? Key : never
          38→	): void;
          39→}
        …
          46→export default function mitt<Events extends Record<EventType, unknown>>(
          47→	all?: EventHandlerMap<Events>
          48→): Emitter<Events> {
        …
test/
tsconfig.json
```

<!-- precis-example-end -->

The file tree shows everything that exists; the README lede and heading outline give orientation; `package.json` identifies the package; and `src/index.ts` shows the full exported type surface with signatures. Line numbers make every entry a precise jump target for follow-up reads.

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
precis .                           # summarize the current directory
precis ./src                       # zoom into a subdirectory
precis . --token-budget 8000       # with a larger token budget
```

The default budget is 3000 BPE tokens (o200k_base tokenizer). Output is plain text with line numbers preserving source indentation.

## Supported languages

- **Rust** — public item signatures, module docs, manifest + workspace structure
- **TypeScript / JavaScript** (`.ts`/`.tsx`/`.js`/`.mjs`/`.cjs`) — exported types, classes, functions; package entry points and workspace layout
- **Go** — exported functions, methods, types; package doc ledes
- **C** (`.c`/`.h`) — functions, aggregates, macros; header surfaces
- **Python** — classes, functions, `__init__.py` re-exports, module constants
- **Lua** — module functions and identity tables
- **Markdown / reStructuredText** — README lede, heading outlines, key section bodies
- **JSON / TOML** — manifests (`package.json`, `Cargo.toml`, `pyproject.toml`, …) and workspace config
- **YAML** — `docker-compose` files
- **Prisma** — schema models and enums
- **Plain text** — man pages, extensionless config files

Files outside these get directory-listing coverage: every file appears in the tree even when its contents aren't summarized.

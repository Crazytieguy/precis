# precis

A CLI tool that extracts a token-efficient summary of a path, designed to replace most Explore agent use with a single fast command. It uses tree-sitter to parse source files, extracts structural symbols (functions, types, interfaces, headings), and ranks them by importance to fit within a token budget.

Upgrading from v0.1? The flags and the output notation changed; see [CHANGELOG.md](CHANGELOG.md).

## Example

Here's what `precis` shows for [developit/mitt](https://github.com/developit/mitt), a tiny TypeScript event emitter, at a 900-token budget:

<!-- precis-example-start -->
```
.editorconfig
.eslintrc
.github/
  PULL_REQUEST_TEMPLATE.md
  workflows/
    compressed-size.yml
    main.yml
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
  22→
  23→-   [Install](#install)
  24→-   [Usage](#usage)
  25→-   [Examples & Demos](#examples--demos)
  26→-   [API](#api)
  27→-   [Contribute](#contribute)
  28→-   [License](#license)
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
  5→  "module": "dist/mitt.mjs",
  6→  "main": "dist/mitt.js",
  7→  "jsnext:main": "dist/mitt.mjs",
  8→  "umd:main": "dist/mitt.umd.js",
  9→  "source": "src/index.ts",
  10→  "typings": "index.d.ts",
  11→  "exports": {
  12→    "types": "./index.d.ts",
  13→    "module": "./dist/mitt.mjs",
  14→    "import": "./dist/mitt.mjs",
  15→    "require": "./dist/mitt.js",
  16→    "default": "./dist/mitt.mjs"
  17→  },
  18→  "scripts": {
  19→    "test": "npm-run-all --silent typecheck lint mocha test-types",
  20→    "mocha": "mocha test",
  21→    "test-types": "tsc test/test-types-compilation.ts --noEmit --strict",
  22→    "lint": "eslint src test --ext ts --ext js",
  23→    "typecheck": "tsc --noEmit",
  24→    "bundle": "microbundle -f es,cjs,umd",
  25→    "build": "npm-run-all --silent clean -p bundle -s docs",
  …
  41→  "license": "MIT",
  42→  "files": [
  43→    "dist",
  44→    "index.d.ts"
  45→  ],
  …
src/
  index.ts
test/
  index_test.ts
  test-types-compilation.ts
tsconfig.json
```
<!-- precis-example-end -->

The file tree shows everything that exists; the README's lede and headings say what the package is and how its docs are organized; and `package.json` identifies the package and its entry points. At larger budgets `src/index.ts` follows with its exported types and signatures. Line numbers make every entry a precise jump target for follow-up reads.

A `…` row means "there is more here that isn't shown": source inside a file, or further entries in a directory. A directory other than the root with more than 120 entries may show only 40 of them, subdirectories picked before files, then that `…`. A source line longer than 500 characters is cut short with `…`. A row like `src/main/java/` is a chain of directories that each hold only the next. An entry with nothing under it wasn't expanded; the rare entry that is genuinely empty is marked `(empty)`.

## Installation

### Claude Code plugin (recommended)

The precis plugin automatically injects a structural overview of your project into Claude's context at the start of every session. This eliminates the need for long, manually-maintained `CLAUDE.md` files describing your codebase, and removes the overhead of Explore agents. The plugin also gives Claude the `precis` CLI so it can zoom into specific directories on demand, and lets `precis` runs on paths inside the project skip the permission prompt.

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
precis ./src/main.rs               # or into a single file
precis . --token-budget 8000       # with a larger token budget
precis . --char-budget 9000        # also cap the output's length
```

The default budget is 3000 BPE tokens (o200k_base tokenizer). Output is plain text with line numbers preserving source indentation. `--char-budget` counts UTF-16 code units, the unit Claude Code measures hook output in; when run from the plugin's hook (`CLAUDE_PLUGIN_ROOT` set), precis derives a default so the injected context stays within Claude Code's 10,000-unit limit.

Given a single file, precis shows its structure first, then spends whatever budget is left on the file's text from the top, so a file that fits prints whole. A binary file prints only its name.

When the path is the root of a git repository, `precis` honours `.gitignore` (including nested ones, `.git/info/exclude`, and your global excludes file), so build output, virtualenvs and dependency trees don't eat the budget. In any tree, `target`, `node_modules`, `dist`, `build`, `.next` and `__pycache__` directories are listed but never expanded (except a `build` that holds Rust source files). `.git/` itself never appears. Non-ignored dotfiles such as `.github/` and `.gitignore` are repository content and are treated like any other file.

## Supported languages

- **Parsed source** — Rust, TypeScript / JavaScript (`.ts`, `.tsx`, `.mts`, `.cts`, `.js`, `.jsx`, `.mjs`, `.cjs`), Python, Go, C (`.c`, `.h`) and Lua: module docs, declaration names and signatures, then doc comments and bodies as the budget allows.
- **README** — the root README in Markdown, reStructuredText, AsciiDoc or plain text (`README.md`, `README.rst`, `README.adoc`, an extensionless `README`, …): its lede, heading outline and section bodies. Other documents appear in the directory tree; name one directly (`precis docs/guide.md`) to summarize it.
- **Manifests** — `package.json`, `Cargo.toml`, `pyproject.toml` (and any TOML that declares a package), `go.mod` / `go.work` and Prisma schemas: identity, entry points, scripts and dependencies. Small root JSON configs render whole.
- **Build and ops files** — Makefile, Taskfile, justfile, Dockerfile, compose files, dotenv samples, runtime version pins (`.nvmrc`, `.python-version`, `.tool-versions`) and `pnpm-workspace.yaml`.
- **Every other source language** — Java, Kotlin, Swift, C++, C#, Ruby, PHP, Scala, Elixir, Zig, Solidity, Verilog, Vue, Svelte and more: each file's top-level declaration lines, or the whole file when it is short. In a repository written mostly in one of these languages, its declaration lines rank like parsed declarations. Stylesheets show their top-level selector lines, and shell scripts, including extensionless ones that start with a shebang, their top-level lines; both are priced below parsed declarations.

Other files, such as YAML, XML, templates and documents other than the root README, appear in the directory tree by name only until the budget outlasts everything above. What is left over then shows their first lines, except for licenses, credential files, generated files and anything under a dot-prefixed name (CI workflows, ignore lists, editor and lint config).

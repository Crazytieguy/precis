# precis

A CLI tool that extracts a token-efficient summary of a path, designed to replace most Explore agent use with a single fast command. It uses tree-sitter to parse source files, extracts structural symbols (functions, types, interfaces, headings), and ranks them by importance to fit within a token budget.

Upgrading from v0.1? The flags and the output notation changed; see [CHANGELOG.md](CHANGELOG.md).

## Example

Here's what `precis` shows for [developit/mitt](https://github.com/developit/mitt), a tiny TypeScript event emitter, at a 1200-token budget:

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
  127→#### Table of Contents
  …
  138→### mitt
  139→
  140→Mitt: Tiny (~200b) functional event emitter / pubsub.
  141→
  142→Returns **Mitt** 
  143→
  144→### all
  145→
  146→A Map of event names to registered handler functions.
  147→
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
  26→    "clean": "rimraf dist",
  27→    "docs": "documentation readme src/index.ts --section API -q --parse-extension ts",
  28→    "release": "npm run -s build -s && npm t && git commit -am $npm_package_version && git tag $npm_package_version && git push && git push --tags && npm publish"
  29→  },
  …
  41→  "license": "MIT",
  42→  "files": [
  43→    "dist",
  44→    "index.d.ts"
  45→  ],
  …
src/
  index.ts
    1→export type EventType = string | symbol;
    …
    5→export type Handler<T = unknown> = (event: T) => void;
    6→export type WildcardHandler<T = Record<string, unknown>> = (
    …
    12→export type EventHandlerList<T = unknown> = Array<Handler<T>>;
    13→export type WildCardEventHandlerList<T = Record<string, unknown>> = Array<
    14→	WildcardHandler<T>
    15→>;
    …
    18→export type EventHandlerMap<Events extends Record<EventType, unknown>> = Map<
    …
    23→export interface Emitter<Events extends Record<EventType, unknown>> {
    …
    46→export default function mitt<Events extends Record<EventType, unknown>>(
    47→	all?: EventHandlerMap<Events>
    48→): Emitter<Events> {
    …
test/
  index_test.ts
  test-types-compilation.ts
tsconfig.json
```
<!-- precis-example-end -->

The file tree shows every file in the project; the README's lede and headings say what the package is and how its docs are organized; `package.json` identifies the package and its entry points; and `src/index.ts` shows its exported types and the signature of `mitt` itself. Line numbers make every entry a precise jump target for follow-up reads.

A `…` row means "there is more here that isn't shown": source inside a file, or further entries in a directory. A source line longer than 500 characters is cut short with `…`. A row like `src/main/java/` is a chain of directories that each hold only the next. An entry with nothing under it wasn't expanded, unless it is marked `(empty)`: a zero-byte file, or a directory holding nothing precis lists. Symlinks that resolve outside the tree, broken symlinks and special files such as FIFOs are omitted.

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

The plugin automatically downloads and updates the binary — no manual install needed. It runs on macOS and Linux (x86_64 and arm64); on other platforms, install the standalone CLI and configure your agent as below.

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

The default budget is 3000 BPE tokens (o200k_base tokenizer). Output is plain text with line numbers preserving source indentation. `--char-budget` counts UTF-16 code units, the unit Claude Code measures hook output in; when run from the plugin's session-start hook, precis derives a default so the injected context stays within Claude Code's 10,000-unit limit.

Given a single file, precis shows its structure first, then spends whatever budget is left on the file's text from the top, so a file that fits prints whole. A binary file prints only its name.

When the path is the root of a git repository, `precis` honours `.gitignore` (including nested ones, `.git/info/exclude`, and your global excludes file), so build output, virtualenvs and dependency trees don't eat the budget. In any tree, `target`, `node_modules`, `dist`, `build`, `.next` and `__pycache__` directories are listed but never expanded (except a `build` that holds Rust source files). `.git/` itself never appears. Non-ignored dotfiles such as `.github/` and `.gitignore` are repository content and are treated like any other file.

Nothing outside the path appears: a link that resolves outside it, or onto an ignored entry, is left out of the tree, as are FIFOs, sockets and devices. A credential file appears by name only, whichever kind of file precis would read it as and even when it is the path given: dotenv files (`.env`, `.env.local`, `production.env`), data, config and script files named for secrets or credentials (`secrets.yml`, `credentials.json`, `creds_staging.conf`, an extensionless `secrets` script), Terraform `.tfvars`, service-account JSON keys, tool auth files (`.npmrc`, `.netrc`, `.pypirc`, `.git-credentials`, `.htpasswd`), links to any of these, and any file holding a PEM or PGP private key. Samples (`.env.example`, or any name ending `.sample`, `.template` or `.dist`) and source code or documents named after credentials (`credentials.py`, `secrets.md`) show as usual. A password written into a URL (`postgresql://admin:PASSWORD@db/app`) shows as `…`. Secrets inside files named otherwise (an `apiKey` in `settings.json`) are not detected.

## Supported languages

- **Parsed source** — Rust, TypeScript / JavaScript (`.ts`, `.tsx`, `.mts`, `.cts`, `.js`, `.jsx`, `.mjs`, `.cjs`), Python (`.py`, `.pyi`), Go, C (`.c`, and `.h` unless it declares C++) and Lua: module docs, declaration names and signatures, then doc comments and bodies as the budget allows.
- **README** — the root README in Markdown, reStructuredText or AsciiDoc (`README.md`, `README.rst`, `README.adoc`, or an extensionless `README`): its lede, heading outline, the commands under its build, test, run and development headings, and section bodies. A root Markdown build or contributing guide (`BUILDING.md`, `INSTALL.md`, `TESTING.md`, `CONTRIBUTING.md`, …) shows its first build or test commands. Other documents appear in the directory tree; name one directly (`precis docs/guide.md`) to summarize it.
- **Manifests** — `package.json`, `Cargo.toml`, `pyproject.toml` (and any TOML that declares a package), `go.mod` / `go.work` and Prisma schemas: identity, entry points, scripts and dependencies. Small root JSON configs render whole. A manifest in another format at the root, in `src/` or in the directory named after the repository (`pom.xml`, `composer.json`, `*.cabal`, `build.sbt`, `CMakeLists.txt`, `action.yml`, R's `DESCRIPTION`, `rebar.config`, `dune-project`, `deps.edn`, `pubspec.yaml`, `shard.yml`, `*.nimble`) shows its leading fields, or the whole file when it is short.
- **Build and ops files** — Makefile, Taskfile, justfile, Dockerfile, compose files, dotenv samples, runtime version pins (`.nvmrc`, `.python-version`, `.tool-versions`) and `pnpm-workspace.yaml`.
- **Every other source language** — Java, Kotlin, Swift, C++, C#, Ruby, PHP, Scala, Elixir, Zig, Solidity, Verilog, Vue, Svelte and more: each file's top-level declaration lines, or the whole file when it is short. In a repository written mostly in one of these languages, its declaration lines rank like parsed declarations. Stylesheets show their top-level selector lines, and shell scripts, including extensionless ones that start with a shebang, their top-level lines; both are priced below parsed declarations.

Other files, such as YAML, XML other than such a manifest, templates and documents other than the root README, appear in the directory tree by name only until the budget outlasts everything above. What is left over then shows their first lines, except for licenses, credential files, generated files and anything under a dot-prefixed name (CI workflows, ignore lists, editor and lint config).

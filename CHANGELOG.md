# Changelog

## 0.2.0

A rewrite of how precis decides what to show. The CLI and the plugin install the same way as before.

### Breaking changes

- `--budget` is now `--token-budget`. `--budget` still works as a hidden alias.
- The default token budget is 3000, down from 4000.
- `--char-budget` counts UTF-16 code units, the unit Claude Code measures hook output in.
- New output notation:
  - Output is a nested tree of directories and files with a 2-space indent.
  - Source lines are unpadded `N→` rows nested under their file.
  - A `…` row marks hidden source in a file, or hidden entries in a directory.
  - A source line longer than 500 characters is cut short with `…`.
  - A row like `src/main/java/` is a chain of directories that each hold only the next.
  - `(empty)` marks a zero-byte file, or a directory holding nothing precis lists. An entry with nothing under it wasn't expanded, or couldn't be read.
  - Symlinks that resolve outside the tree, broken symlinks and special files such as FIFOs are omitted. A link to a file in its own directory (`CLAUDE.md -> AGENTS.md`) is listed, and its text shows only under the target.
- Dotfiles such as `.github/` and `.gitignore` are listed. v0.1 hid every hidden entry.
- `.gitignore` rules apply when the path is the root of a git repository. They are no longer inherited from an enclosing repository.
- YAML and other JSON/TOML files are no longer summarized by their top-level keys. Manifests, small root JSON configs, compose files and Taskfiles are covered instead. Other YAML files appear in the tree only.
- Only the root README is summarized, in Markdown, reST, AsciiDoc or an extensionless `README`; a root Markdown build or contributing guide (`BUILDING.md`, `INSTALL.md`, `TESTING.md`, `CONTRIBUTING.md`, …) shows only the commands under its first build, test, run or development heading. Other documents appear in the tree by name; pass one as the path to summarize it.
- Rust 1.88 or newer is required to build from source.

### Highlights

- One code engine serves Rust, TypeScript/JavaScript, Python, Go, C and Lua. It shows module docs, then declaration names, signatures, doc comments and bodies as the budget allows.
- Walkers for `package.json`, `Cargo.toml`, `pyproject.toml`, `go.mod`/`go.work`, Prisma schemas, READMEs in reST and AsciiDoc, and build and ops files (Makefile, Taskfile, justfile, Dockerfile, compose files, dotenv samples).
- A root manifest in a format precis doesn't parse (`pom.xml`, `composer.json`, `*.cabal`, `build.sbt`, `CMakeLists.txt`, `action.yml`, R's `DESCRIPTION`, …) shows its leading fields, such as the project's name, version and description.
- Source files in other languages (Java, Kotlin, Swift, C++, C#, Ruby, PHP, Zig, Solidity, Vue, …) show their top-level declaration lines instead of plain text. In a repository written mostly in one of these languages, those lines rank like parsed declarations. Stylesheets show their top-level selector lines and shell scripts their top-level lines, both priced below parsed declarations; extensionless scripts that start with a shebang are read as scripts.
- When a repository is small enough that the budget outlasts everything precis ranks, the rest of it shows the first lines of files it otherwise lists by name only (documents, templates, YAML, data), except licenses, credential files, generated files and anything under a dot-prefixed name.
- A credential file appears by name only, whichever part of precis would read it: dotenv files (not samples such as `.env.example`), data, config and script files named for secrets, credentials, tokens, passwords or API keys, `.key` and `.pem` files, `.tfvars`, service-account keys, tool auth files, links to any of these, and any file holding a private key. A password written into a URL, and a quoted literal assigned to a credential-named key in code or config (`password: "…"`, `API_KEY = "…"`), shows as `…`. Links that resolve outside the path or onto an ignored entry are left out.
- A single file can still be passed as the path. precis shows its structure first, then spends whatever budget is left on the file's text from the top, so a file that fits prints whole.
- Translations, archived subtrees, third-party code, and game-engine `.meta`/`.import`/`.uid` sidecars no longer crowd out a repository's own code.
- Without `--char-budget`, every entry and source line the output at a smaller token budget shows, the output at a larger one shows too, though a directory-chain row may run on further (`.claude/` becomes `.claude/skills/review/`).
- Faster and lighter on large trees. Files over 8 MiB are not parsed, parse trees are freed once rendered, and probes below the visible tree stop after 20,000 entries, so a non-git tree as large as a home directory no longer stalls the session hook.
- The plugin sizes its `--char-budget` at runtime to what is left of Claude Code's 10,000-unit hook cap after the help text, and ranks content by whichever of the two budgets it uses up faster. Previously it used a fixed 9,500 characters.
- Fixed: a panic (stack overflow in the tokenizer) on very large non-git trees.
- Fixed: a panic when stdout closes early, as in `precis . | head`.
- When nothing fits the budget, precis says so on stderr instead of printing nothing.

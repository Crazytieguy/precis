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
  - `(empty)` marks entries that are genuinely empty. An entry with nothing under it wasn't expanded.
- Dotfiles such as `.github/` and `.gitignore` are listed. v0.1 hid every hidden entry.
- `.gitignore` rules apply when the path is the root of a git repository. They are no longer inherited from an enclosing repository.
- YAML and other JSON/TOML files are no longer summarized by their top-level keys. Manifests, small root JSON configs, compose files and Taskfiles are covered instead. Other YAML files appear in the tree only.
- Only the root `README.md` or `README.rst` is summarized. Other Markdown, `.mdx` and reST documents appear in the tree by name; pass a `.md` or `.rst` file as the path to summarize it.
- Rust 1.88 or newer is required to build from source.

### Highlights

- One code engine serves Rust, TypeScript/JavaScript, Python, Go, C and Lua. It shows module docs, then declaration names, signatures, doc comments and bodies as the budget allows.
- Walkers for `package.json`, `Cargo.toml`, `pyproject.toml`, `go.mod`/`go.work`, Prisma schemas, `README.rst`, and build and ops files (Makefile, Taskfile, Dockerfile, compose files, dotenv samples).
- Source files in other languages (Java, Kotlin, Swift, C++, C#, Ruby, PHP, Vue, CSS, …) show their top-level declaration lines instead of plain text.
- A single file can still be passed as the path.
- The output at a smaller budget is always a subset of the output at a larger one.
- Faster on large C and TypeScript trees.
- The plugin sizes its `--char-budget` at runtime to what is left of Claude Code's 10,000-unit hook cap after the help text. Previously it used a fixed 9,500 characters.
- Fixed: a panic (stack overflow in the tokenizer) on very large non-git trees.
- Fixed: a panic when stdout closes early, as in `precis . | head`.
- When nothing fits the budget, precis says so on stderr instead of printing nothing.

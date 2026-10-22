# Changelog

## 0.2.2

### Plugin

- A path the Read tool would ask about now goes to your permission mode, as any MCP tool call does, instead of to a question from the plugin: in auto mode the classifier decides, and the permission prompt asks only where Claude Code would ask anyway.

## 0.2.1

### Plugin

- The plugin now comes in one build per machine: `precis-aarch64-apple-darwin`, `precis-x86_64-apple-darwin`, `precis-aarch64-unknown-linux-gnu` and `precis-x86_64-unknown-linux-gnu`. Each carries its machine's precis binary, so nothing is downloaded at session start; the binary now updates with the plugin instead of from GitHub, so turn on auto-update for the `precis` marketplace to keep getting new versions. A build installed on the wrong machine shows a row above the prompt whose **Fix** installs the right one.
- The `precis` plugin keeps working as before, and offers the build for your machine in a row above the prompt, whose **Switch** installs it and takes `precis` out.
- The plugin is a Claude Code hooks module instead of shell hooks. The overview of the working directory still starts each session and comes back after `/clear`, but no longer comes back after compaction, and subagents don't get it. Claude Code's 10,000-character hook limit no longer applies, so it is a plain 3000-token overview instead of one cut to fit beside the help text.
- Claude zooms in with a `summarize` tool (`mcp__precis__summarize`) instead of running `precis` through Bash, and the plugin no longer puts `precis` on Bash's `PATH`: if your `CLAUDE.md` tells Claude to run `precis`, point it at the tool, or install the standalone CLI. The tool summarizes a path the Read tool may read without asking, refuses one a Read deny rule covers, and asks you about any other. Like the old auto-approval, it checks only the path it is given, not each file under a directory. Its description is the `--help` notation legend.
- A `token_budget` option (in `/config`, or `/plugin configure <plugin>@precis`, `<plugin>` being your build) sets the overview's budget and the tool's default.
- The "precis: available" line at session start is gone; errors still show.
- The plugin no longer needs `jq`.
- Where an organization manages Claude Code, a plugin a person installs can't add to a session's starting context, so it provides the tool alone there.

## 0.2.0

A rewrite of how precis decides what to show. The CLI and the plugin install the same way as before.

### Breaking changes

- `--budget` is now `--token-budget`. `--budget` still works as a hidden alias.
- The default token budget is 3000, down from 4000.
- `--char-budget` counts UTF-16 code units, the unit Claude Code measures hook output in.
- New output notation:
  - Output is a nested tree of directories and files with a 2-space indent.
  - Source lines are unpadded `N→` rows nested under their file.
  - A `…` row marks hidden source in a file, or hidden entries in a directory. A jump in line numbers without one skips only blank lines.
  - A source line longer than 500 characters is cut short with `…`.
  - A row like `src/main/java/` is a chain of directories that each hold only the next.
  - `(empty)` marks a zero-byte file, or a directory holding nothing precis lists. An entry with nothing under it wasn't expanded, or couldn't be read.
  - `(submodule)` marks an empty directory the root `.gitmodules` names: usually a submodule that isn't checked out.
  - Symlinks that resolve outside the tree or onto an ignored entry, broken symlinks and special files such as FIFOs are omitted. A link to a file in its own directory (`CLAUDE.md -> AGENTS.md`) is listed, and its text shows only under the target.
- Dotfiles such as `.github/` and `.gitignore` are listed. v0.1 hid every hidden entry.
- `.gitignore` rules apply when the path is the root of a git repository. They are no longer inherited from an enclosing repository.
- YAML and other JSON/TOML files are no longer summarized by their top-level keys. Manifests, small root JSON configs, compose files and Taskfiles are covered instead. Other YAML files appear in the tree only.
- Only the root README is summarized, in Markdown, reST, AsciiDoc or an extensionless `README`; a root build or contributing guide (`BUILDING.md`, `INSTALL.md`, `CONTRIBUTING.md`, `README.DEVELOPER`, …) shows only the commands under its first build, test, run or development heading, else the checkout commands (a clone, `make`, a build, test or run step) under a setup or install heading, preferring a block whose heading doesn't name a platform (`Building on Windows`). Other documents appear in the tree by name; pass one as the path to summarize it.
- Rust 1.88 or newer is required to build from source. The answer-key tooling sits behind an `eval` feature, so `cargo install` no longer compiles serde or toml.

### Highlights

- One code engine serves Rust, TypeScript/JavaScript, Python, Go, C and Lua. It shows module docs, then declaration names, signatures, doc comments and bodies as the budget allows. When an entry file only re-exports, the modules it names open right after it.
- Walkers for `package.json`, `Cargo.toml`, `pyproject.toml`, `go.mod`/`go.work`, Prisma schemas, READMEs in reST and AsciiDoc, and build and ops files (Makefile, Taskfile, justfile, Dockerfile, compose files, dotenv samples).
- A root manifest in a format precis doesn't parse (`pom.xml`, `composer.json`, `*.cabal`, `build.sbt`, `CMakeLists.txt`, `action.yml`, R's `DESCRIPTION`, …) shows its leading fields, such as the project's name, version and description.
- Source files in other languages (Java, Kotlin, Swift, C++, C#, Ruby, PHP, Zig, Solidity, Vue, …) show their top-level declaration lines instead of plain text. In a repository written mostly in one of these languages, those lines rank like parsed declarations. Stylesheets show their top-level selector lines and shell scripts their top-level lines, both priced below parsed declarations; extensionless scripts that start with a shebang are read as scripts.
- When a repository is small enough that the budget outlasts everything precis ranks, the rest of it shows the first lines of files it otherwise lists by name only (documents, templates, YAML, data), except licenses, credential files, generated files and anything under a dot-prefixed name. A line an earlier file already showed is left out as `…`.
- Example and sample trees and a root `support/` directory rank with tests and scripts, below the repository's own code. A listing made mostly of many files sharing a name prefix and extension (`issue-*.md`) is priced like a folder of images, and sibling folders of non-source files that repeat an earlier one's file names are listed after the rest.
- A credential file appears by name only, whichever part of precis would read it: dotenv files (not samples such as `.env.example`), data, config and script files named for secrets, credentials, tokens, passwords or API keys, `.key` and `.pem` files, `.tfvars`, service-account keys, tool auth files, links to any of these, and any file holding a private key. A password written into a URL, and a quoted literal assigned to a credential-named key in code or config (`password: "…"`, `API_KEY = "…"`), shows as `…`; documents keep theirs, which are placeholders.
- A single file can still be passed as the path. precis shows its structure first, then spends whatever budget is left on the file's text from the top, so a file that fits prints whole.
- The root manifest or build file (`build.zig`, `build.gradle.kts`, a Makefile) is read before the listings below its main source directories. A long root Makefile shows the rules and recipes of its conventional targets (`all`, `build`, `test`, `check`, `install`, `help`) instead of its `.PHONY` list.
- Translations, archived subtrees, generated documentation sites (Javadoc, Dokka, odoc, Doxygen, Jazzy or Sphinx output), third-party code (including directories the root `.gitattributes` marks `linguist-vendored`), and game-engine `.meta`/`.import`/`.uid` sidecars no longer crowd out a repository's own code.
- Without `--char-budget`, every entry and source line the output at a smaller token budget shows, the output at a larger one shows too, though a directory-chain row may run on further (`.claude/` becomes `.claude/skills/review/`).
- Faster and lighter on large trees. Files parse on every core, and precis counts tokens with its own o200k tokenizer instead of building tiktoken's (about 70 ms per run). Files over 8 MiB are not read, a run parses at most 32 MiB of source, parse trees are freed once extracted, and a Python file with a run of comment lines long enough to stall its parser is not parsed. Probes below the visible tree stop after 20,000 entries, so a non-git tree as large as a home directory no longer stalls the session hook.
- The plugin sizes its `--char-budget` at runtime to what is left of Claude Code's 10,000-unit hook cap after the help text, and ranks content by whichever of the two budgets it uses up faster. Previously it used a fixed 9,500 characters.
- The plugin auto-approves only `precis` runs that use its own flags on paths inside the project; any other form, including a path that doesn't exist, gets the normal permission prompt. It checks for a new binary at most hourly, verifies the release checksum before installing it, and reports an unsupported platform instead of a generic error.
- Fixed: a panic (stack overflow in the tokenizer) on very large non-git trees.
- Fixed: a panic when stdout closes early, as in `precis . | head`.
- When nothing fits the budget, precis says so on stderr instead of printing nothing.

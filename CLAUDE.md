# Precis

A CLI tool that extracts a token-efficient summary of a codebase.

## Stack

- Rust
- Tree-sitter for parsing

## Architecture

**Groups:** Symbols are bucketed into groups by shared properties (visibility, kind, file path, documented, config, heading depth, first-party import, trait impl, boilerplate section, reexport). File-level properties (file role, test, type declaration) are stored on the group for value computation. All symbols in a group receive the same rendering treatment.

**Stage progressions:** Each group has an ordered progression of rendering stages. Different kind categories have different progressions (e.g. types show body before doc, sections skip signatures). Doc and Body stages are continuous — each additional line is a separate scheduling item.

**Greedy scheduling:** A priority queue ranks all (group, stage) items by value/cost ratio including prerequisite costs. The algorithm greedily includes the highest-priority item, deducts its cost from the budget, and repeats until nothing fits.

**Line-prefix constraint:** Each output line is a prefix of the actual source line. The tool extracts from source rather than synthesizing representations.

## Codebase Exploration

Use precis itself to explore this codebase instead of spawning Task agents. Run `cargo run -- .` or `cargo run -- src/` to get an overview. If precis output isn't sufficient for a task, that's a signal the tool should be improved — note it in `issues.md`.

## Documentation

API docs for this crate and its dependencies are at @target/doc-md/index.md. Always run `cargo doc-md` after changing dependencies in Cargo.toml, or if `target/doc-md/` is missing docs for an installed crate.

## Bash Operations

All commands must complete within 1 minute. If a command would take longer, use different parameters (e.g. fewer benchmark samples) or a different approach.

Complex bash syntax is hard for Claude Code to permission correctly. Keep commands simple.

Simple operations are fine: `|`, `||`, `&&`, `>` redirects.

For bulk operations on multiple files, use xargs:
- Plain: `ls *.md | xargs wc -l`
- With placeholder: `ls *.md | xargs -I{} head -1 {}`

Avoid string interpolation (`$()`, backticks, `${}`), heredocs, loops, and advanced xargs flags (`-P`, `-L`, `-n`) - these require scripts or simpler alternatives.

**Patterns:**
- File creation: Write tool, not `cat << 'EOF' > file`
- Env vars: `export VAR=val && command`, not `VAR=val command` or `env VAR=val command`
- Bulk operations: `ls *.md | xargs wc -l`, not `for f in *.md; do cmd "$f"; done`
- Parallel/batched xargs: use scripts, not `xargs -P4` or `xargs -L1`
- Per-item shell logic: use scripts, not `xargs sh -c '...'`
- Git: `git <command>`, not `git -C <path> <command>` (breaks permissions)

For complex operations that don't fit simple bash, write a temporary Rust binary (e.g. in `src/bin/`) and run it with `cargo run --bin <name>`.

If a command that should be allowed is denied, or if project structure changes significantly, ask about running `/mats:permissions` to update settings.

## Testing

- Fixture data is defined once in `test/fixtures.rs`, shared by snapshot tests and the clone binary
- Run `cargo run --bin clone_fixtures` to clone all missing fixtures
- Each entry has a single budget matching its real use case (1000/2000/4000)
- Always run tests in release mode: `cargo test --release` (debug mode is much slower)
- Always run `cargo bench --bench hot_path -- --quick` after changes to catch performance regressions
- When inspecting snapshot changes, read the diffs as a user would — check for regressions (lost useful content, gained noise)

## Ownership

You are the sole maintainer of this codebase. You have full freedom with the code — refactor, simplify, rewrite as needed.

## Code Health

Log any issues noticed during work in `issues.md`, even if tiny — concrete bugs, DRY violations, stale heuristics, unclear code. Keeping this file current is important for long-term codebase health. When an issue is resolved, remove it from the file.

If you notice that `README.md` or `CLAUDE.md` have become stale or inaccurate during your work, update them.

# Symbols

A CLI tool that extracts symbols from a codebase and returns a token-efficient breakdown.

## Stack

- Rust
- Tree-sitter for parsing

## Architecture

See @README.md for design decisions.

## Documentation

If implementation diverges from what's documented in the README, update the README to match.

API docs for this crate and its dependencies are at @target/doc-md/index.md. Re-run `cargo doc-md` after adding or removing a dependency.

## Bash Operations

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

- Snapshot tests using real open-source projects (cloned into `test/fixtures/`, gitignored)
- Token budget tests reuse the same test cases
- Fixtures organized by language and project size

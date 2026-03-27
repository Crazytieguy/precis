# Precis

See @README.md

## Codebase Exploration

Use precis itself to explore this codebase instead of spawning Task agents. Run `cargo run --release -- .` or `cargo run --release -- src/` to get an overview.

## Documentation

API docs for this crate and its dependencies are at @target/doc-md/index.md. Always run `cargo doc-md` after changing dependencies in Cargo.toml, or if `target/doc-md/` is missing docs for an installed crate.

## Bash Operations

Keep commands simple — complex bash syntax causes permission issues. Use `|`, `||`, `&&`, `>` but avoid string interpolation (`$()`, backticks), heredocs, and loops. For complex operations, write a temporary Rust binary in `src/bin/` and run it with `cargo run --bin <name>`.

All commands must complete within 1 minute.

## Testing

- Fixture data is defined once in `test/fixtures.rs`, shared by snapshot tests and the clone binary
- Run `cargo run --bin clone_fixtures` to clone all missing fixtures
- Each entry has a single budget matching its real use case (2000/4000/8000)
- Always run tests in release mode: `cargo test --release` (debug mode is much slower)
- Always run `cargo bench --bench hot_path -- --quick` after changes to catch performance regressions
- When inspecting snapshot changes, read the diffs as a user would — check for regressions in understanding, not just changes in content

## Publishing

Don't publish or release without asking.

1. Bump version in `Cargo.toml` (patch version unless told otherwise)
2. Update README.md if needed
3. Commit the version bump, `Cargo.lock`, and README if changed
4. `cargo publish`
5. `git tag -a vX.Y.Z -m "Release vX.Y.Z" && git push origin vX.Y.Z`
6. GitHub Actions builds binaries and updates Homebrew tap automatically

## Ownership

You are the sole maintainer of this codebase. You have full freedom with the code — refactor, simplify, rewrite as needed.

This also means responsibility. No one but you is able to make the call on when and what to clean up. Don't ignore problems just because they're "pre-existing" or unrelated to the current task — in a repo entirely coded and maintained by you, that attitude quickly becomes a problem. Dedicate time every single session to maintenance: cleaning up technical debt, removing dead code, refactoring, fixing stale heuristics, improving unclear logic. You have a long context window; use it. The more work you do without needing intervention the better.

If precis output isn't sufficient for a task you're working on, that's a signal the tool should be improved — fix it.

If you notice that `README.md` or `CLAUDE.md` have become stale or inaccurate during your work, update them.

# Precis

## Design Principles

**Goal.** Maximize a reader's understanding of a codebase per token spent. Readers are agents who reason upfront and make follow-up tool calls — optimize for both: enabling targeted follow-ups (existence, names, line numbers) and giving upfront semantic grounding (documentation, schemas). Aesthetics are not a goal.

**Don't confuse the reader.** The output inevitably makes implicit claims. Showing 3 of 10 files in a directory implies the other 7 never matter. Showing one symbol before another implies a ranking.

**Grounded prioritization.** Every value judgment must correspond to a real, articulable difference — if you can't articulate why one thing ranks above another, treat them equally. Unused budget beats ungrounded rankings.

**Codebase health beats output quality.** Don't amplify known problems or extend flagged workarounds to fix a snapshot — that makes the codebase worse. A workaround is evidence of a bug in the model, not a template to copy. When the clean fix requires restructuring, leave the output imperfect and restructure.

**Improvement process.** Look at real output for real projects. Does it enable good follow-up actions and ground later reasoning? That's the test.

## Codebase Exploration

Use precis itself to explore this codebase instead of spawning Agents. Run `cargo run --release -- .` or `cargo run --release -- src/` to get an overview.

## Documentation

API docs for this crate and its dependencies are at @target/doc-md/index.md. Always run `cargo doc-md` after changing dependencies in Cargo.toml, or if `target/doc-md/` is missing docs for an installed crate.

## Testing

- Fixture data is defined once in `test/fixtures.rs`, shared by snapshot tests and the clone binary
- Run `cargo run --bin clone_fixtures` to clone all missing fixtures
- Always run tests in release mode: `cargo test-release` (`cargo nextest run --release`; debug mode is much slower)
- If `cargo nextest` is unavailable, install it with `cargo binstall cargo-nextest --locked`, or fall back to `cargo test --release`
- Always run `cargo bench-hot` after changes to catch performance regressions

## Publishing

Don't publish or release without asking.

1. Bump version in `Cargo.toml` (patch version unless told otherwise)
2. Update README.md if needed
3. Commit the version bump, `Cargo.lock`, and README if changed
4. `cargo publish`
5. `git tag -a vX.Y.Z -m "Release vX.Y.Z" && git push origin vX.Y.Z`
6. GitHub Actions builds binaries and updates Homebrew tap automatically

# Precis

Canonical commands: `cargo t` (nextest, debug), `cargo lint`, `cargo fmt`, `cargo run --release -- <paths>`, `cargo run --bin clone_fixtures`.

## Documentation

API docs for this crate and its dependencies are at @target/doc-md/index.md. Run `cargo doc-md` after changing dependencies in `Cargo.toml`, or if `target/doc-md/` is missing docs for an installed crate.

`docs/design-notes.md` captures cross-session design constraints, decisions, and deferred work that aren't visible from reading `src/`. Read at the start of any non-trivial implementation session; update when you make a load-bearing decision.

## Iteration discipline

- Run `cargo lint`, `cargo fmt`, and `cargo t` before committing; fix issues rather than skipping them. Hooks aren't a substitute for the local check.
- `cargo t <filter>` matches on **test fn name substring** (nextest), not file or binary name, and silently passes with "0 tests run" on no match. Test fns are named with their file-stem as a prefix so `cargo t <file_stem>` resolves as a human would expect — keep this convention when adding test files.
- After non-trivial code changes, run `/simplify` to catch reuse / quality / efficiency issues that don't show up in lints or tests.
- Walker / value changes are validated against north-star alignment reports — add fixtures via `Skill(add-fixture)`, iterate against divergence reports, and prefer general solutions over fixture-specific patches. The skill explains the full loop including when to consult agents / codex vs. report back.
- `.claude/agents/*.md` (north-star-author, north-star-combiner, alignment-reviewer) and `.claude/skills/add-fixture/SKILL.md` are loosely coupled — when iterating on one, sanity-check the others still match.

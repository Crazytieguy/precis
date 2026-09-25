---
name: release
description: Publish a new precis release to crates.io and tag it for the GitHub release workflow. Use when the user asks to release, publish, or cut a version.
---

Don't publish without explicit user confirmation for this specific release.

Pre-flight checks (all must pass):

1. `cargo lint`
2. `cargo t`
3. `cargo fmt -- --check`
4. Working tree is clean (`git status` shows no unstaged/uncommitted changes).

Release steps:

1. Bump the version in `Cargo.toml` (patch unless the user says otherwise) and set `plugins/precis/.claude-plugin/plugin.json` to the same version (marketplace clients cache by it).
2. Update `README.md` if needed. Its example block is pinned by `tests/integration/readme_example.rs`; when that test fails, regenerate the block with the command its failure message prints.
3. Commit the version bump along with `Cargo.lock`, `plugin.json` and any README changes.
4. `cargo publish` (user will see the ask prompt).
5. `git tag -a vX.Y.Z -m "Release vX.Y.Z"` then `git push origin vX.Y.Z` (user will see the ask prompt).
6. The GitHub Actions release workflow builds binaries and updates the Homebrew tap automatically.

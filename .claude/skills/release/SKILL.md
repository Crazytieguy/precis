---
name: release
description: Publish a new precis release to crates.io and tag it for the GitHub release workflow. Use when the user asks to release, publish, or cut a version.
---

Don't publish without explicit user confirmation for this specific release.

Pre-flight checks (all must pass):

1. `cargo lint`
2. `cargo t` (needs `tests/fixtures`, which is gitignored; on a fresh checkout run `cargo run --example clone_fixtures` first)
3. `cargo fmt -- --check`
4. Working tree is clean (`git status` shows no unstaged/uncommitted changes).

Release steps:

1. Set the release version in `Cargo.toml` and set `plugins/precis/.claude-plugin/plugin.json` to the same version (marketplace clients cache by it). Bump the patch unless the user says otherwise. If `Cargo.toml` already carries an unreleased target version (for example a minor bump made on the branch; check with `git tag -l 'v*'`), keep it and don't bump again.
2. Add a section for the version to `CHANGELOG.md`; the GitHub release body is built from it. Update `README.md` if needed. Its example block is pinned by `tests/integration/readme_example.rs`; when that test fails, regenerate the block with the command its failure message prints.
3. Commit the version, `Cargo.lock`, `plugin.json`, changelog and README changes. Skip the commit if nothing changed.
4. Fast-forward `main` to the release commit: `git checkout main && git merge --ff-only <release-branch>`. The GitHub README and the plugin marketplace read `main`, so it must carry the release. Then `git fetch origin && git merge-base --is-ancestor origin/main main`; if that fails, `main` has diverged from the remote, so reconcile it and redo the pre-flight checks before publishing.
5. `cargo publish` (user will see the ask prompt).
6. On `main`, `git tag -a vX.Y.Z -m "Release vX.Y.Z"` so the tag points at the release commit, then `git push --atomic origin main vX.Y.Z` (user will see the ask prompt). Without `--atomic`, a rejected `main` update can still create the tag and start the release workflow. If the push is still rejected, stop and ask the user: the crate is already published from the tagged commit.
7. The GitHub Actions release workflow builds binaries and updates the Homebrew tap automatically. Plugin users' binaries auto-update from the latest GitHub release, so once the workflow is green check that `curl -s https://api.github.com/repos/Crazytieguy/precis/releases/latest | jq -r .tag_name` prints `vX.Y.Z`.

---
name: add-fixture
description: End-to-end process for bringing a new test fixture into precis — from declaring the repo, through drafting and freezing its North Star, through generating the regression baselines. Use when adding a new fixture or when running the NS + regression cycle on an already-declared fixture that isn't yet active.
---

# add-fixture

Pass only the fixture name (e.g. `thiserror`) through the flow; paths
are derived from it. The flow runs end-to-end unless the invocation asks
for a human checkpoint on the NS ("let me review the north star first"),
in which case pause after step 2.

Skip any step whose output already exists (`tests/fixtures/<name>/`,
`tests/north-stars/<name>.toml`). Baseline regeneration is idempotent.

## 1. Declare and clone

Add a `<name> "<git-url>" "<commit-sha>",` row to `tests/data/fixtures.rs`,
under the language comment in the `training` block (the default) or the
`validation` block (only when the invocation says so). A `-` in the
directory name is written `_` there. Then:

```bash
cargo run --example clone_fixtures
```

Clones missing fixtures to `tests/fixtures/<name>/` at the pinned SHA,
strips `.git`, and writes `.precis-pin`.

Declaring the row also registers the test `fixture_baselines_<name>`
(training) or `fixture_baselines_validation_<name>` (validation), which
fails until step 2's NS exists.

## 2. Author the North Star

Spawn one `north-star-author` agent with the fixture root
(`tests/fixtures/<name>/`), the output path
(`tests/north-stars/<name>.toml`), and the revision pin (the SHA from
step 1). It iterates against `cargo run --example validate_ns -- <path>`
until clean; expect 15–30 minutes.

The result is **frozen**: from here on nobody edits it. The calibration
loop moves the walker toward the NS, never the reverse.

## 3. Generate baselines

```bash
UPDATE_BASELINES=1 cargo t fixture_baselines
```

A training fixture produces `tests/divergence/<name>.md` and
`tests/rendered/<name>.txt`; a validation fixture produces only the
one-line `tests/validation/<name>.md`. Check the training headline with
`head -1 tests/divergence/<name>.md`. Don't open a validation fixture's
files; they are held out (see `Skill(iterate-divergence)`).

If the new training report shows NS-authoring problems before any
iteration (empty descriptors, rows that miss what the repo is for),
surface them; they may need a fresh step 2, not walker work.

Walker iteration against the new fixture is `Skill(iterate-divergence)`.

## Don't

- Hand-write an NS. The agent's drafts are better than a direct one.
- Edit a frozen NS.

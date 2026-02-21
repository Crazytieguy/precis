Improve precis by comparing its output against what an Explore agent produces for real codebases.

## Fixture entries

`test/fixtures.rs` defines entries — each is a (name, path, budget) tuple pointing to a cloned repo under `test/fixtures/`. Entries without an `// inspected:` comment haven't been inspected yet.

## Inspection process

For each uninspected entry:

1. **Read the snapshot** (`test/snapshots/snapshots__<name>.snap`). Take notes in `scratch.md`: what do you understand about this codebase from precis output alone? Where would you look next if starting a task?

2. **Run an Explore agent** on `test/fixtures/<path>` to understand the same codebase. Make it very clear which folder it should explore, it sometimes gets confused and explores the precis codebase.

3. **Compare.** Did the Explore agent surface crucial details that precis missed? Did precis waste budget on content the Explore agent ignored? Is precis output misleading?

4. **Decide:**
   - **Exceeds Explore:** Precis output significantly exceeds the Explore agent output in valuable information density. Mark the entry `// inspected: exceeds explore`.
   - **Issue noted:** The Explore agent surfaced important details that precis missed, but you don't have a concrete improvement idea. Log the gap in `issues.md` under "Observations" with specifics. Mark the entry `// inspected: logged observation`.
   - **Clear general improvement:** You identified a concrete, general change that would improve precis output. Implement it (see below). Mark the entry `// inspected: <short description of change>`.

The bar for "exceeds explore" is high. If precis didn't give you meaningfully better orientation than the Explore agent did, that's an observation worth logging even if you don't know how to fix it.

If multiple logged observations point to the same underlying issue, that's also a good time to implement.

## Implementation

When you have a concrete improvement:

1. Implement the change.
2. Run `cargo test --release`.
3. **Inspect every changed snapshot.** Read the diffs. If the change isn't a general improvement — if snapshots regressed (lost useful content, gained noise, became confusing) and there's no clear fix — revert with `git checkout` and move on.
4. Run `cargo bench --bench hot_path -- --quick`. If benchmarks regressed significantly, note it in `issues.md` or revert.

## When all entries are inspected

If no uninspected entries remain in `test/fixtures.rs`, switch to resolving issues from `issues.md` one at a time, following the `## implementation` procedure.

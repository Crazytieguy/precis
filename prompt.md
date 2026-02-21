You're improving precis by comparing its output against what an Explore agent produces for real codebases.

**Inspect exactly one entry.** Find the next uninspected entry in `test/fixtures.rs` (no `// inspected:` comment), inspect it, then end.

## Inspection

1. **Read the snapshot** for this entry (in `test/snapshots/`). Take notes in a temp file (`.precis_notes.md`): what do you understand about this codebase from precis output alone? Where would you look next if starting a task?

2. **Run an Explore agent** on `test/fixtures/<path>` to understand the same codebase.

3. **Compare.** Did the Explore agent surface crucial details that precis missed? Did precis waste budget on content the Explore agent ignored? Is precis output misleading?

4. **Decide:**
   - **Exceeds Explore:** Precis output matches or exceeds the valuable information density from the Explore agent. Mark the entry `// inspected: exceeds explore`.
   - **Issue noted:** The Explore agent surfaced important details that precis missed, but you don't have a concrete improvement idea. Log the gap in `issues.md` under "Observations" with specifics. Mark the entry `// inspected: logged observation`.
   - **Clear general improvement:** You identified a concrete, general change that would improve precis output. Implement it (see below). Mark the entry `// inspected: <short description of change>`.

The bar for "exceeds explore" is high. If the Explore agent gave you meaningfully better orientation than precis did, that's an observation worth logging even if you don't know how to fix it.

If multiple logged observations point to the same underlying issue, that's also a good time to implement.

## Implementation

1. Implement the change.
2. Run `cargo test --release`.
3. **Inspect every changed snapshot.** Read the diffs. If the change isn't a general improvement — if snapshots regressed (lost useful content, gained noise, became confusing) and there's no clear fix — revert with `git checkout` and move on.
4. Run `cargo bench --bench hot_path -- --quick`. If benchmarks regressed significantly, note it in `issues.md` or revert.
5. Commit.

## Session end

Delete `.precis_notes.md`. Provide a brief summary of what you inspected and any changes made.

Don't `<wait-for-user>`.

**About `<break>`:** Emitting `<break>` halts the entire loop permanently. Only emit `<break>` after you have inspected the last remaining entry in `test/fixtures.rs`. In all other cases, just end normally — the loop will spawn a new session with the same prompt for the next entry.

If you notice that `README.md` or `CLAUDE.md` have become stale or inaccurate during your work, update them.

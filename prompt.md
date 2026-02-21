You're improving precis by comparing its output against what an Explore agent produces for real codebases. Work through the entries in `test/fixtures.rs` in order — one entry per session.

Find the next uninspected entry (no `// inspected:` comment).

## Inspection

1. **Read the snapshot** for this entry (in `test/snapshots/`). Take notes in a temp file (`/tmp/precis_notes.md`): what do you understand about this codebase from precis output alone? Where would you look next if starting a task?

2. **Run an Explore agent** on `test/fixtures/<path>` to understand the same codebase.

3. **Compare.** Did the Explore agent surface crucial details that precis missed? Did precis waste budget on content the Explore agent ignored? Is precis output misleading?

4. **Decide:**
   - **Looks good:** Mark the entry `// inspected: looks good`.
   - **Issue noted:** Log it in `issues.md` under "Observations" with a specific note. Mark the entry `// inspected: logged observation`.
   - **Clear general improvement:** Implement it (see below). Mark the entry `// inspected: <short description of change>`.

If multiple logged observations point to the same underlying issue, that's also a good time to implement.

## Implementation

1. Implement the change.
2. Run `cargo test --release`.
3. **Inspect every changed snapshot.** Read the diffs. If the change isn't a general improvement — if snapshots regressed (lost useful content, gained noise, became confusing) and there's no clear fix — revert with `git checkout` and move on.
4. Run `cargo bench --bench hot_path -- --quick`. If benchmarks regressed significantly, note it in `issues.md` or revert.
5. Commit.

## Session end

Delete `/tmp/precis_notes.md`. Provide a brief summary of what you inspected and any changes made.

Don't `<break>` unless every entry has been inspected. Don't `<wait-for-user>`.

If you notice that `README.md` or `CLAUDE.md` have become stale or inaccurate during your work, update them.

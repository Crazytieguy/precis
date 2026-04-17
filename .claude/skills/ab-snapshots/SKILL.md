---
name: ab-snapshots
description: Run a blind pairwise A/B review over precis snapshot changes. Use after making changes that affect `.snap` outputs to judge net quality impact with independent per-fixture verdicts.
---

# A/B snapshot review

When the working tree has precis changes that affect snapshot tests, run this
skill to get an independent per-fixture verdict on whether each change is a
win, loss, or tie — without reading every diff yourself.

## Flow

1. **Prepare pairs.** Run the bundled script:

   ```bash
   python3 ${CLAUDE_SKILL_DIR}/scripts/prepare.py
   ```

   This runs `cargo nextest run --release`, finds every `.snap.new`, pairs it
   with its committed `.snap`, randomizes A/B labels, writes one pair file per
   fixture, and prints a manifest path to stdout (all progress to stderr).

   Read the manifest with `cat <manifest-path>`. It's a JSON list of entries:

   ```json
   {
     "fixture": "go_sample_budget_200",
     "pair_file": "/tmp/precis-ab/pairs/go_sample_budget_200.pair.md",
     "fixture_path": "test/fixtures/...",
     "a_is": "committed",
     "b_is": "new"
   }
   ```

   Keep the `a_is` / `b_is` mapping **private** — never show it to subagents.

2. **Spawn reviewers.** For each manifest entry, spawn one
   `precis-snapshot-reviewer` subagent with a prompt like:

   > Fixture: `<fixture>`. Fixture repo path: `<fixture_path>` (or
   > "determine from snap content" if null). Pair file:
   > `<pair_file>`. Read the pair file and return a verdict.

   Send all subagent calls in a single message (one Agent tool call per
   entry, parallel). Subagents share prompt caching, so this is cheap —
   don't batch serially.

3. **Collect + decode.** Each subagent's last line is `VERDICT: A|B|TIE`.
   Parse it, then use the manifest to decode: if `a_is = "new"` and verdict
   is A, that's a **win for new**. If `a_is = "committed"` and verdict is A,
   that's a **loss**. TIE stays TIE.

4. **Aggregate.** Report wins / losses / ties. Optionally note per-fixture
   verdicts for ones that flipped or where confidence was notable.

## Notes

- No changed snapshots → manifest is empty, skip the rest.
- New snapshots without committed baselines are skipped with a stderr note.
- The script blows away `AB_OUT_DIR` (default `/tmp/precis-ab`) on each run,
  so don't put anything important there.
- If you need to A/B against a non-HEAD baseline (e.g. `main`), `git stash`
  your changes, `git checkout <ref>`, regenerate snapshots with `cargo
  nextest run --release` (to produce committed `.snap`), then restore the
  working tree and run this skill. This skill does not do worktree magic.

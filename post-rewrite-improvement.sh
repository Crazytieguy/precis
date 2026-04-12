#!/usr/bin/env bash
set -euo pipefail

# Usage: post-rewrite-improvement.sh [START_PHASE]
#   START_PHASE: 1-4 (default 1). Skips to the given phase.
START_PHASE="${1:-1}"

echo "=== Precis post-rewrite improvement pipeline ==="
echo "Started at $(date) (starting at phase $START_PHASE)"

# ── Phase 1: Source file review ───────────────────────────────────────
if [ "$START_PHASE" -le 1 ]; then
echo ""
echo "=== Phase 1: Source file review ==="

# Structural order: leaf modules first, orchestrators last.
# Each session benefits from issues already recorded by earlier sessions.
SOURCE_FILES=(
    src/format.rs
    src/classify.rs
    src/parse/ast.rs
    src/parse/classify.rs
    src/parse/module_doc.rs
    src/parse/name.rs
    src/parse/visibility.rs
    src/parse/postprocess.rs
    src/parse/mod.rs
    src/store.rs
    src/heuristics.rs
    src/group/mod.rs
    src/group/ts.rs
    src/group/files.rs
    src/group/folders.rs
    src/render.rs
    src/schedule.rs
    src/lib.rs
    src/main.rs
    tests/snapshots.rs
)

for file in "${SOURCE_FILES[@]}"; do
    echo ""
    echo "--- Reviewing: $file ---"
    coven ralph --iterations 1 --no-break --no-wait "CONTEXT: We just finished a rough first pass implementing design.md — a full architecture rewrite of precis. We're now reviewing each source file to find issues before fixing them.

YOUR TASK: Review the source file '$file' for issues and record any you find in issues.md.

STEPS:
1. Read issues.md first — if an issue you find already exists or is very similar, merge your observations into the existing entry rather than adding a duplicate.
2. Read '$file' carefully.
3. Look for:
   - Deviations from what design.md mandates
   - Code quality issues
   - Unnecessary #[allow(clippy::...)] attributes
   - Bugs or incorrect logic
   - Anything improvable without violating design.md invariants
4. You may read other source files for context — some issues are cross-file.
5. Finding no issues is completely fine. Don't force them.
6. If you added or updated issues, commit your changes.

Do NOT fix any issues — only document them. Be concise and actionable." \
    || echo "WARNING: Review of $file exited with error, continuing..."
done

echo ""
echo "Phase 1 complete."
fi

# ── Phase 2: Fix issues ──────────────────────────────────────────────
if [ "$START_PHASE" -le 2 ]; then
echo ""
echo "=== Phase 2: Fix issues ==="

coven ralph "CONTEXT: precis just had a full architecture rewrite (see design.md). Issues have been documented in issues.md. You are fixing them one at a time.

YOUR TASK: Pick ONE issue from issues.md and fix it.

STEPS:
1. Read issues.md and pick one issue to work on.
2. Implement the fix.
3. Run 'cargo test --release'. Check any modified snapshots for unintended changes.
4. Run 'cargo clippy --all-targets -- -D warnings' and fix any warnings, including pre-existing ones.
5. Run /simplify
6. Spawn an Agent to independently verify your fix doesn't violate design.md invariants or introduce new bugs. Address any concerns raised.
7. If the fix looks good: remove the addressed issue from issues.md and commit.
   If you're not confident (see below): revert and mark it '[needs human review]' instead.

ON REVERTING:
Sometimes a fix turns out harder than expected — tests break in ways that aren't straightforward to resolve, the fix requires compromises that feel wrong, or you're unsure whether the change is actually correct. In those cases, reverting and marking the issue as '[needs human review]' with a note on what you tried is the right call. A clean revert preserves the codebase for the next attempt, while a shaky fix can compound into harder problems later. Don't feel pressure to push through — the goal is steady improvement, not completion at any cost.

ENDING THE LOOP:
Iff issues.md has no actionable issues remaining (empty sections, or only items marked '[needs human review]'), output <break> and stop. Be careful not to break prematurely — outputting <break> while actionable issues remain means they won't get addressed."

echo ""
echo "Phase 2 complete."
fi

# ── Phase 3: Snapshot review ─────────────────────────────────────────
if [ "$START_PHASE" -le 3 ]; then
echo ""
echo "=== Phase 3: Snapshot review ==="

for snap in test/snapshots/snapshots__*.snap; do
    snap_name=$(basename "$snap" .snap | sed 's/^snapshots__//')
    echo ""
    echo "--- Reviewing snapshot: $snap_name ---"
    coven ralph --iterations 1 --no-break --no-wait "CONTEXT: precis recently had a full architecture rewrite. We're reviewing each fixture snapshot for output quality issues.

YOUR TASK: Review the snapshot '$snap_name' for output quality.

STEPS:
1. Read the snapshot file at '$snap'.
2. Check test/fixtures.rs for the fixture path, then explore the fixture directory yourself rather than using precis or an Agent — the goal is to form your own holistic view of what matters, unbiased by the tool's existing output.
3. Read src/heuristics.rs to understand current value/cost assignments.
4. Evaluate the output: are there concrete issues? For example, something of low value taking up budget that something of high value needed, or important content being omitted while less important content is shown.
5. Also compare against the origin/main version of this snapshot (git show origin/main:'$snap' 2>/dev/null). The rewrite may have introduced regressions — if the old output was better in specific ways, note that too.
6. If you find issues, record them in output-issues.md (create if needed). If a similar issue already exists, merge your observations — include all affected snapshots and context. Be concise.
7. Finding no issues is completely fine.
8. Commit if you made changes.

Do NOT fix any issues — only document them." \
    || echo "WARNING: Review of $snap_name exited with error, continuing..."
done

echo ""
echo "Phase 3 complete."
fi

# ── Phase 4: Fix output issues ───────────────────────────────────────
if [ "$START_PHASE" -le 4 ]; then
echo ""
echo "=== Phase 4: Fix output issues ==="

coven ralph "CONTEXT: We've reviewed precis snapshot output and documented quality issues in output-issues.md. You are fixing them one at a time by tuning heuristics or modifying code.

YOUR TASK: Pick ONE issue from output-issues.md and fix it.

STEPS:
1. Read output-issues.md and pick one issue.
2. Implement the fix — tune heuristics in src/heuristics.rs, or modify other code as needed.
3. Run 'cargo test --release'. The affected snapshots (listed in the issue) should improve — concretely, the output should give a reader a better mental model. Other snapshots should not regress on average.
4. Run 'cargo clippy --all-targets -- -D warnings' and fix any warnings, including pre-existing ones.
5. Run /simplify
6. Spawn an Agent to independently evaluate the snapshot changes. The Agent should read the snapshot diffs and judge whether affected snapshots genuinely improved and whether others regressed. Address any concerns raised.
7. If the fix looks good: remove the addressed issue from output-issues.md and commit.
   If you're not confident (see below): revert and mark it '[needs human review]' instead.

ON REVERTING:
If the affected snapshots don't clearly improve, or other snapshots regress, or the change requires compromising on design invariants — revert and mark the issue as '[needs human review]' with a note on what you tried. Output quality is subjective and some issues may need a human eye. A clean revert is better than a change you're not confident in.

ENDING THE LOOP:
Iff output-issues.md has no actionable issues remaining (empty, or only items marked '[needs human review]'), output <break> and stop. Be careful not to break prematurely — outputting <break> while actionable issues remain means they won't get addressed."

echo ""
echo "Phase 4 complete."
fi

echo ""
echo "=== All phases complete ==="
echo "Finished at $(date)"

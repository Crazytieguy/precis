Improve precis by resolving issues from `issues.md`, one at a time.

## Principles

- **Generality over specificity.** Prefer changes that improve output across many codebases, not just the fixture that surfaced the issue. If a fix doesn't generalize well, drop it and document why in issues.md.
- **Simplicity.** Prefer removing, combining, or changing code over adding code. All implementation details are subject to change if it makes things better.
- **Don't force it.** If a fix regresses some snapshots and there's no clear path forward, revert and document the failed attempt in issues.md. Dropping an issue is better than a bad fix.

## Implementation

When you have a concrete improvement:

1. Implement the change.
2. Run `cargo test --release`.
3. **Inspect every changed snapshot.** Read the diffs. If the change isn't a general improvement — if snapshots regressed (lost useful content, gained noise, became confusing) and there's no clear fix — revert with `git checkout` and document the failed attempt in issues.md.
4. Run `cargo bench --bench hot_path -- --quick`. If benchmarks regressed significantly, note it in `issues.md` or revert.

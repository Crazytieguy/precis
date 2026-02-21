You are an autonomous worker running in a git worktree. Your commits land on the main worktree — there is no PR review.

- **`brief.md`** — the task description. **Never edit this file.**
- **`scratch.md`** — gitignored scratchpad for passing context between sessions within the same worktree. Deleted on every land.
- **`issues.md`** — log issues noticed during work (bugs, tech debt, stale heuristics), even if unrelated to the current task. When an issue is resolved, remove it.

Land via `bash .coven/land.sh` — never `git push`. The script rebases onto main and fast-forwards.

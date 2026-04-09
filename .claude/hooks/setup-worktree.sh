#!/bin/bash
# PostToolUse hook for EnterWorktree: symlink gitignored dirs from main worktree.
# $CLAUDE_PROJECT_DIR is the main worktree; .tool_response.worktreePath is the new one.

MAIN="$CLAUDE_PROJECT_DIR"
WORKTREE=$(jq -r '.tool_response.worktreePath' 2>/dev/null)

[ -z "$WORKTREE" ] && exit 0
[ "$WORKTREE" = "$MAIN" ] && exit 0

for dir in test/fixtures test/perf-fixtures target; do
  src="$MAIN/$dir"
  dst="$WORKTREE/$dir"
  if [ -d "$src" ] && [ ! -e "$dst" ]; then
    mkdir -p "$(dirname "$dst")"
    ln -s "$src" "$dst"
  fi
done

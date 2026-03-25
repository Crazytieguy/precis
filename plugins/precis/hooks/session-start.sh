#!/bin/bash

PLUGIN_DATA="${CLAUDE_PLUGIN_DATA:-$HOME/.cache/precis}"
LOG_FILE="$PLUGIN_DATA/error.log"
mkdir -p "$PLUGIN_DATA" 2>/dev/null

# Capture all stderr to the log file
exec 2>>"$LOG_FILE"

# On any error, log and inform user
trap 'echo "$0: line $LINENO: unexpected error" >&2; echo "{\"systemMessage\":\"\u001b[1;32mprecis:\u001b[0m error in session-start hook, see $LOG_FILE\"}"' ERR
set -euo pipefail

PRECIS_BIN="$PLUGIN_DATA/precis"

# Find jq
if command -v jq >/dev/null 2>&1; then
  JQ="jq"
elif [ -x "$PLUGIN_DATA/jq" ]; then
  JQ="$PLUGIN_DATA/jq"
else
  JQ=""
fi

# --- Parse hook input ---
HOOK_INPUT=$(cat)

# Parse source — use jq if available, otherwise grep
if [ -n "$JQ" ]; then
  SOURCE=$(echo "$HOOK_INPUT" | "$JQ" -r '.source // ""')
else
  SOURCE=$(echo "$HOOK_INPUT" | grep -o '"source"[[:space:]]*:[[:space:]]*"[^"]*"' | head -1 | grep -o '"[^"]*"$' | tr -d '"')
fi

# On resume, context window is intact and CLAUDE_ENV_FILE can't be overwritten
if [ "$SOURCE" = "resume" ]; then
  exit 0
fi

# Add plugin data dir to PATH so Claude can run precis manually
if [ -n "${CLAUDE_ENV_FILE:-}" ]; then
  echo "export PATH=\"$PLUGIN_DATA:\$PATH\"" >> "$CLAUDE_ENV_FILE"
fi

# Bootstrap binaries synchronously on first install.
# ensure-precis.sh requires --install to run when the binary is missing
# (without it, the async hook skips first-time install to avoid racing).
if { [ -z "$JQ" ] || [ ! -x "$PRECIS_BIN" ]; } && [ -n "${CLAUDE_PLUGIN_ROOT:-}" ]; then
  if [ -z "$JQ" ]; then
    bash "$CLAUDE_PLUGIN_ROOT/scripts/ensure-jq.sh" 2>>"$LOG_FILE"
    if command -v jq >/dev/null 2>&1; then
      JQ="jq"
    elif [ -x "$PLUGIN_DATA/jq" ]; then
      JQ="$PLUGIN_DATA/jq"
    fi
  fi
  if [ ! -x "$PRECIS_BIN" ]; then
    bash "$CLAUDE_PLUGIN_ROOT/scripts/ensure-precis.sh" --install 2>>"$LOG_FILE"
  fi
fi

# If both precis and jq are available, run precis and emit additionalContext
if [ -x "$PRECIS_BIN" ] && [ -n "$JQ" ]; then
  HELP_OUTPUT=$("$PRECIS_BIN" --help 2>/dev/null) || HELP_OUTPUT=""
  PRECIS_OUTPUT=$("$PRECIS_BIN" . 2>/dev/null) || PRECIS_OUTPUT=""

  if [ -n "$PRECIS_OUTPUT" ]; then
    "$JQ" -n --arg help "$HELP_OUTPUT" --arg output "$PRECIS_OUTPUT" '{
      systemMessage: "\u001b[1;32mprecis:\u001b[0m available",
      hookSpecificOutput: {
        hookEventName: "SessionStart",
        additionalContext: ("## precis\n\nOutput of `precis --help`:\n\n```\n" + $help + "\n```\n\nOutput of `precis .`:\n\n```\n" + $output + "\n```")
      }
    }'
  else
    echo '{"systemMessage":"\u001b[1;32mprecis:\u001b[0m available"}'
  fi
else
  echo '{"systemMessage":"\u001b[1;32mprecis:\u001b[0m error, see '"$LOG_FILE"'"}'
fi

#!/bin/bash

# On any error, fall through to normal permission prompt
trap 'exit 0' ERR
set -euo pipefail

input=$(cat)

# Fast-path: skip jq entirely if input doesn't mention precis
case "$input" in
  *precis*) ;;
  *) exit 0 ;;
esac

PLUGIN_DATA="${CLAUDE_PLUGIN_DATA:-$HOME/.cache/precis}"

# Find jq
if command -v jq >/dev/null 2>&1; then
  JQ="jq"
elif [ -x "$PLUGIN_DATA/jq" ]; then
  JQ="$PLUGIN_DATA/jq"
else
  exit 0
fi

# Extract fields
eval "$(echo "$input" | "$JQ" -r '
  "command=" + (.tool_input.command // "" | @sh),
  "cwd=" + (.cwd // "" | @sh)
')"

# Paths are checked against the canonical cwd, so a missing cwd or one
# reached through a symlink can't widen or break the check.
[ -n "$cwd" ] || exit 0
cwd=$(realpath "$cwd" 2>/dev/null) || exit 0
[ -d "$cwd" ] || exit 0

# Any newline/carriage return means the command is multi-line or has been
# crafted to hide content past the tokenizer — fall through to a user prompt.
case "$command" in
  *$'\n'*|*$'\r'*) exit 0 ;;
esac

# Tokenize command by whitespace
read -ra tokens <<< "$command"

# Must have at least one token, and it must be "precis"
if [ ${#tokens[@]} -eq 0 ] || [ "${tokens[0]}" != "precis" ]; then
  exit 0
fi

# Parse precis's own grammar; anything else falls through to a prompt.
# Every positional argument is a path, whatever it looks like.
paths=()

for ((i = 1; i < ${#tokens[@]}; i++)); do
  token="${tokens[$i]}"

  case "$token" in
    -h|--help|-V|--version) continue ;;
    --token-budget|--budget|--char-budget)
      i=$((i + 1))
      [[ "${tokens[$i]:-}" =~ ^[0-9]+$ ]] || exit 0
      continue
      ;;
  esac

  if [[ "$token" =~ ^--(token-budget|budget|char-budget)=[0-9]+$ ]]; then
    continue
  fi

  # Redirect: fd-to-fd only (e.g. 2>&1), no file paths
  if [[ "$token" =~ ^[0-9]*'>&'[0-9]+$ ]]; then
    continue
  fi

  # Path: safe filesystem characters, not starting with a dash (an unknown
  # flag) or with a `~` the shell would expand as `~user`.
  if [[ "$token" =~ ^[a-zA-Z0-9_./~-]+$ && "$token" != -* ]]; then
    case "$token" in
      "~") token="$HOME" ;;
      "~/"*) token="$HOME/${token#\~/}" ;;
      "~"*) exit 0 ;;
    esac
    paths+=("$token")
    continue
  fi

  exit 0
done

# precis with no path summarizes the working directory
[ ${#paths[@]} -gt 0 ] || paths=(.)

# Every path must resolve to something inside cwd
for path_value in "${paths[@]}"; do
  # Can't resolve (nonexistent, unusual token, etc) — fall through to a prompt.
  resolved=$(cd "$cwd" && realpath "$path_value" 2>/dev/null) || exit 0

  if [[ "$resolved" != "$cwd" && "$resolved" != "$cwd/"* ]]; then
    exit 0
  fi
done

"$JQ" -n '{
  hookSpecificOutput: {
    hookEventName: "PermissionRequest",
    decision: { behavior: "allow" }
  }
}'

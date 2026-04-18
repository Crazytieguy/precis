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

# Collect every path-like token after "precis"
paths=()

for ((i = 1; i < ${#tokens[@]}; i++)); do
  token="${tokens[$i]}"

  # Flag: -x, --long, --long=value (value restricted to same safe chars as paths)
  if [[ "$token" =~ ^--?[-a-zA-Z0-9]+(=[a-zA-Z0-9_./~-]*)?$ ]]; then
    continue
  fi

  # Number: purely digits (flag value like 4000)
  if [[ "$token" =~ ^[0-9]+$ ]]; then
    continue
  fi

  # Redirect: fd-to-fd only (e.g. 2>&1), no file paths
  if [[ "$token" =~ ^[0-9]*'>&'[0-9]+$ ]]; then
    continue
  fi

  # Path: safe filesystem characters
  if [[ "$token" =~ ^[a-zA-Z0-9_./~-]+$ ]]; then
    paths+=("$token")
    continue
  fi

  # Unrecognized token — fall through to normal prompt
  exit 0
done

allow_response() {
  "$JQ" -n '{
    hookSpecificOutput: {
      hookEventName: "PermissionRequest",
      decision: { behavior: "allow" }
    }
  }'
}

# No paths: precis alone, precis --help, etc — all safe
if [ ${#paths[@]} -eq 0 ]; then
  allow_response
  exit 0
fi

# Every path must resolve to something inside cwd
for path_value in "${paths[@]}"; do
  # Expand tilde (not expanded inside double quotes)
  if [[ "$path_value" == "~/"* ]]; then
    path_value="$HOME/${path_value#\~/}"
  elif [[ "$path_value" == "~" ]]; then
    path_value="$HOME"
  fi

  # Can't resolve (nonexistent, unusual token, etc) — fall through to a prompt.
  resolved=$(cd "$cwd" && realpath "$path_value" 2>/dev/null) || exit 0

  if [[ "$resolved" != "$cwd" && "$resolved" != "$cwd/"* ]]; then
    # At least one path is outside cwd — fall through to normal prompt
    exit 0
  fi
done

# All paths inside cwd — allow
allow_response

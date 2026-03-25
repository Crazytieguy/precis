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

# Tokenize command by whitespace
read -ra tokens <<< "$command"

# Must have at least one token, and it must be "precis"
if [ ${#tokens[@]} -eq 0 ] || [ "${tokens[0]}" != "precis" ]; then
  exit 0
fi

# Classify every token after "precis"
path_count=0
path_value=""

for ((i = 1; i < ${#tokens[@]}; i++)); do
  token="${tokens[$i]}"

  # Flag: starts with -
  if [[ "$token" =~ ^-[-a-zA-Z0-9]+$ ]]; then
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
    path_count=$((path_count + 1))
    path_value="$token"
    continue
  fi

  # Unrecognized token — fall through to normal prompt
  exit 0
done

# Decide based on path count
if [ "$path_count" -eq 0 ]; then
  # No path: precis alone, precis --help, etc — all safe
  "$JQ" -n '{
    hookSpecificOutput: {
      hookEventName: "PermissionRequest",
      decision: { behavior: "allow" }
    }
  }'
  exit 0
fi

if [ "$path_count" -ge 2 ]; then
  # Ambiguous — fall through
  exit 0
fi

# Expand tilde (not expanded inside double quotes)
if [[ "$path_value" == "~/"* ]]; then
  path_value="$HOME/${path_value#\~/}"
elif [[ "$path_value" == "~" ]]; then
  path_value="$HOME"
fi

# Exactly one path — resolve and validate
resolved=$(cd "$cwd" && realpath "$path_value" 2>/dev/null) || {
  "$JQ" -n '{
    hookSpecificOutput: {
      hookEventName: "PermissionRequest",
      decision: {
        behavior: "deny",
        message: "precis: path does not exist"
      }
    }
  }'
  exit 0
}

# Check if resolved path is inside cwd
if [[ "$resolved" == "$cwd" || "$resolved" == "$cwd/"* ]]; then
  "$JQ" -n '{
    hookSpecificOutput: {
      hookEventName: "PermissionRequest",
      decision: { behavior: "allow" }
    }
  }'
else
  # Outside cwd — fall through to normal prompt (user may want to allow)
  exit 0
fi

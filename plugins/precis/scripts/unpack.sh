#!/bin/bash
set -euo pipefail

# Unpacks the precis binary from <bundle>, a cargo-dist archive the
# plugin's archive build puts under bin/, into a directory keyed by the
# plugin's version, and prints the binary's path.
#
# Usage: unpack.sh <bundle>

BUNDLE="${1:?usage: unpack.sh <bundle>}"
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

VERSION=$(sed -n 's/.*"version"[[:space:]]*:[[:space:]]*"\([^"]*\)".*/\1/p' "$ROOT/.claude-plugin/plugin.json" | head -1)
DIR="${XDG_CACHE_HOME:-$HOME/.cache}/precis/v$VERSION"
BINARY="$DIR/precis"

if [ ! -x "$BINARY" ]; then
  mkdir -p "$DIR"
  # Unpack beside the binary and rename it into place, so a concurrent
  # session or a killed run never leaves a partial executable at $BINARY.
  STAGING=$(mktemp -d "$DIR/staging.XXXXXX")
  trap 'rm -rf "$STAGING"' EXIT
  tar -xf "$BUNDLE" -C "$STAGING"
  # cargo-dist nests the binary in a directory named after the archive.
  FOUND=$(find "$STAGING" -name precis -type f | head -1)
  [ -n "$FOUND" ] || { echo "no precis binary in $BUNDLE" >&2; exit 1; }
  chmod +x "$FOUND"
  mv -f "$FOUND" "$BINARY"
fi

echo "$BINARY"

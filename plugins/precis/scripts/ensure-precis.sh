#!/bin/bash
set -euo pipefail

PLUGIN_DATA="${CLAUDE_PLUGIN_DATA:-$HOME/.cache/precis}"
mkdir -p "$PLUGIN_DATA"

LOG_FILE="$PLUGIN_DATA/error.log"
exec 2>>"$LOG_FILE"

PRECIS_BIN="$PLUGIN_DATA/precis"

# Find jq (prefer global, fall back to bootstrapped)
if command -v jq >/dev/null 2>&1; then
  JQ="jq"
elif [ -x "$PLUGIN_DATA/jq" ]; then
  JQ="$PLUGIN_DATA/jq"
else
  JQ=""
fi

# Get latest release tag
RELEASE_JSON=$(curl -fSs https://api.github.com/repos/Crazytieguy/precis/releases/latest 2>/dev/null) || exit 0

if [ -n "$JQ" ]; then
  TAG=$(echo "$RELEASE_JSON" | "$JQ" -r '.tag_name // ""')
else
  TAG=$(echo "$RELEASE_JSON" | grep -o '"tag_name"[[:space:]]*:[[:space:]]*"[^"]*"' | head -1 | grep -o '"[^"]*"$' | tr -d '"')
fi

[ -n "$TAG" ] || exit 0

# Check if already up to date
if [ -x "$PRECIS_BIN" ] && [ -f "$PLUGIN_DATA/version" ]; then
  CURRENT=$(cat "$PLUGIN_DATA/version")
  if [ "$CURRENT" = "$TAG" ]; then
    exit 0
  fi
fi

# Detect platform
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "$ARCH" in
  aarch64|arm64) ARCH="aarch64" ;;
  x86_64)        ;;
  *)             exit 0 ;;
esac

case "$OS-$ARCH" in
  darwin-aarch64)  TARGET="aarch64-apple-darwin" ;;
  darwin-x86_64)   TARGET="x86_64-apple-darwin" ;;
  linux-aarch64)   TARGET="aarch64-unknown-linux-gnu" ;;
  linux-x86_64)    TARGET="x86_64-unknown-linux-gnu" ;;
  *)               exit 0 ;;
esac

DOWNLOAD_URL="https://github.com/Crazytieguy/precis/releases/download/${TAG}/precis-${TARGET}.tar.xz"

TMPFILE=$(mktemp /tmp/precis-download-XXXXXXXX.tar.xz)
TMPDIR=$(mktemp -d /tmp/precis-extract-XXXXXXXX)
trap 'rm -rf "$TMPFILE" "$TMPDIR"' EXIT

curl -fSL "$DOWNLOAD_URL" -o "$TMPFILE" || exit 0
tar xf "$TMPFILE" -C "$TMPDIR" || exit 0

# cargo-dist nests the binary in a subdirectory
mv "$TMPDIR"/*/precis "$PRECIS_BIN" 2>/dev/null || mv "$TMPDIR"/precis "$PRECIS_BIN" || exit 0
chmod +x "$PRECIS_BIN"

if ! "$PRECIS_BIN" --help >/dev/null 2>&1; then
  rm -f "$PRECIS_BIN"
  exit 0
fi

echo "$TAG" > "$PLUGIN_DATA/version"

#!/bin/bash
set -euo pipefail

PLUGIN_DATA="${CLAUDE_PLUGIN_DATA:-$HOME/.cache/precis}"
mkdir -p "$PLUGIN_DATA"

LOG_FILE="$PLUGIN_DATA/error.log"
exec 2>>"$LOG_FILE"

PRECIS_BIN="$PLUGIN_DATA/precis"

# Binary missing: only install when called with --install (synchronously
# from session-start.sh). The async hook skips first-time install.
if [ ! -x "$PRECIS_BIN" ] && [ "${1:-}" != "--install" ]; then
  exit 0
fi

# Find jq (prefer global, fall back to bootstrapped)
if command -v jq >/dev/null 2>&1; then
  JQ="jq"
elif [ -x "$PLUGIN_DATA/jq" ]; then
  JQ="$PLUGIN_DATA/jq"
else
  JQ=""
fi

# Detect platform
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "$ARCH" in
  arm64) ARCH="aarch64" ;;
esac

case "$OS-$ARCH" in
  darwin-aarch64)  TARGET="aarch64-apple-darwin" ;;
  darwin-x86_64)   TARGET="x86_64-apple-darwin" ;;
  linux-aarch64)   TARGET="aarch64-unknown-linux-gnu" ;;
  linux-x86_64)    TARGET="x86_64-unknown-linux-gnu" ;;
  *)
    echo "precis: no release binary for $OS-$ARCH" >&2
    exit 0
    ;;
esac

# Ask GitHub at most hourly once installed: unauthenticated API requests
# are limited to 60 an hour, and every session start runs this hook.
CHECKED="$PLUGIN_DATA/update-checked"
if [ -x "$PRECIS_BIN" ] && [ -n "$(find "$CHECKED" -mmin -60 2>/dev/null)" ]; then
  exit 0
fi
touch "$CHECKED"

# Get latest release tag
RELEASE_JSON=$(curl -fSs https://api.github.com/repos/Crazytieguy/precis/releases/latest) || exit 0

if [ -n "$JQ" ]; then
  TAG=$(echo "$RELEASE_JSON" | "$JQ" -r '.tag_name // ""')
else
  TAG=$(echo "$RELEASE_JSON" | grep -o '"tag_name"[[:space:]]*:[[:space:]]*"[^"]*"' | head -1 | grep -o '"[^"]*"$' | tr -d '"')
fi

if [ -z "$TAG" ]; then
  echo "precis: no tag_name in the latest release response" >&2
  exit 0
fi

# Check if already up to date
if [ -x "$PRECIS_BIN" ] && [ -f "$PLUGIN_DATA/version" ]; then
  CURRENT=$(cat "$PLUGIN_DATA/version")
  if [ "$CURRENT" = "$TAG" ]; then
    exit 0
  fi
fi

ARCHIVE="precis-${TARGET}.tar.xz"
DOWNLOAD_URL="https://github.com/Crazytieguy/precis/releases/download/${TAG}/${ARCHIVE}"

# A run killed before its EXIT trap leaves its work dir behind.
find "$PLUGIN_DATA" -maxdepth 1 -name 'update.*' -mmin +60 -exec rm -rf {} +
# Unpack beside the binary so the final mv is a rename on one filesystem:
# the sync session hook may exec the binary while this async hook replaces it.
WORK_DIR=$(mktemp -d "$PLUGIN_DATA/update.XXXXXXXX")
trap 'rm -rf "$WORK_DIR"' EXIT

curl -fsSL "$DOWNLOAD_URL" -o "$WORK_DIR/$ARCHIVE" || exit 0
curl -fsSL "$DOWNLOAD_URL.sha256" -o "$WORK_DIR/$ARCHIVE.sha256" || exit 0

if command -v sha256sum >/dev/null 2>&1; then
  ACTUAL=$(sha256sum "$WORK_DIR/$ARCHIVE")
else
  ACTUAL=$(shasum -a 256 "$WORK_DIR/$ARCHIVE")
fi
EXPECTED=$(cut -d ' ' -f 1 "$WORK_DIR/$ARCHIVE.sha256")
if [ -z "$EXPECTED" ] || [ "${ACTUAL%% *}" != "$EXPECTED" ]; then
  echo "precis: checksum mismatch for $DOWNLOAD_URL" >&2
  exit 0
fi

tar xf "$WORK_DIR/$ARCHIVE" -C "$WORK_DIR" || exit 0

# cargo-dist nests the binary in a subdirectory
if [ -f "$WORK_DIR/precis" ]; then
  NEW_BIN="$WORK_DIR/precis"
else
  NEW_BIN=$(find "$WORK_DIR" -mindepth 2 -maxdepth 2 -type f -name precis | head -1)
fi
if [ -z "$NEW_BIN" ]; then
  echo "precis: no precis binary in $DOWNLOAD_URL" >&2
  exit 0
fi
chmod +x "$NEW_BIN"
if ! "$NEW_BIN" --help >/dev/null; then
  echo "precis: the $TARGET binary does not run here" >&2
  exit 0
fi

mv -f "$NEW_BIN" "$PRECIS_BIN"
echo "$TAG" > "$PLUGIN_DATA/version"

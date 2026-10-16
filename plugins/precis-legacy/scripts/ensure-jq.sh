#!/bin/bash
set -euo pipefail

# If jq is available globally, nothing to do
if command -v jq >/dev/null 2>&1; then
  exit 0
fi

PLUGIN_DATA="${CLAUDE_PLUGIN_DATA:-$HOME/.cache/precis}"
BINARY="$PLUGIN_DATA/jq"

# Check if we already bootstrapped it
if [ -x "$BINARY" ]; then
  exit 0
fi

mkdir -p "$PLUGIN_DATA"
exec 2>>"$PLUGIN_DATA/error.log"

# Detect platform
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "$OS" in
  linux)  OS_NAME="linux" ;;
  darwin) OS_NAME="macos" ;;
  *)      exit 0 ;;
esac

case "$ARCH" in
  x86_64)        ARCH_NAME="amd64" ;;
  aarch64|arm64) ARCH_NAME="arm64" ;;
  *)             exit 0 ;;
esac

JQ_VERSION="1.7.1"
DOWNLOAD_URL="https://github.com/jqlang/jq/releases/download/jq-${JQ_VERSION}/jq-${OS_NAME}-${ARCH_NAME}"

curl -fSL "$DOWNLOAD_URL" -o "$BINARY" || { rm -f "$BINARY"; exit 0; }
chmod +x "$BINARY"

if ! "$BINARY" --version >/dev/null 2>&1; then
  rm -f "$BINARY"
fi

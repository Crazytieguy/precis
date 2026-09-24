#!/usr/bin/env bash
# Prepares a lane worktree for building and measuring. Run from the worktree
# root before the first cargo command:
#
#   bash scripts/lane-setup.sh
#
# 1. Refuses to run in the main checkout (worktree spawns have landed there).
# 2. Symlinks tests/fixtures to the main checkout's clone.
# 3. Seeds target/ with an APFS copy-on-write clone of the main checkout's
#    target/, so the first build recompiles only the precis crate instead of
#    every tree-sitter grammar. The clone is instant and shares blocks until
#    written. Skipped if target/ already exists. The clone only helps if the
#    main target/ was built by the current rustc — after a toolchain update,
#    run one build in the main checkout before spawning lanes.
set -euo pipefail

MAIN=$(dirname "$(git rev-parse --path-format=absolute --git-common-dir)")
HERE=$(git rev-parse --show-toplevel)

if [ "$HERE" = "$MAIN" ]; then
    echo "lane-setup: $HERE is the main checkout, not a worktree" >&2
    exit 1
fi

[ -e tests/fixtures ] || ln -s "$MAIN/tests/fixtures" tests/fixtures

if [ -e target ]; then
    echo "lane-setup: target/ exists, leaving it"
elif [ -d "$MAIN/target" ]; then
    if ! grep -qF "$(rustc -V)" "$MAIN/target/.rustc_info.json" 2>/dev/null; then
        echo "lane-setup: $MAIN/target was built by another rustc, first build will be cold" >&2
    fi
    cp -Rc "$MAIN/target" target
    echo "lane-setup: cloned $MAIN/target"
else
    echo "lane-setup: $MAIN/target missing, first build will be cold" >&2
fi

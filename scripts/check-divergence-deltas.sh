#!/usr/bin/env bash
# Compares regenerated Score(3000) per fixture against the immutable pre-batch
# baseline ref captured in .precis-batch-base-ref. Exits non-zero on |delta| >
# THRESHOLD for any fixture EXCEPT those named in TARGETS (comma-separated).
#
# Usage:
#   bash scripts/check-divergence-deltas.sh
#   TARGETS=soluna,py3xui bash scripts/check-divergence-deltas.sh
#   THRESHOLD=0.015 TARGETS=py3xui bash scripts/check-divergence-deltas.sh
set -euo pipefail

THRESHOLD="${THRESHOLD:-0.02}"
TARGETS="${TARGETS:-}"

[ -f .precis-batch-base-ref ] || { echo ".precis-batch-base-ref missing — capture before Phase 1" >&2; exit 2; }
BASE_REF=$(cat .precis-batch-base-ref)
[ -n "$BASE_REF" ] || { echo ".precis-batch-base-ref empty" >&2; exit 2; }

WORK=$(mktemp -d)
trap 'rm -rf "$WORK"' EXIT
BEFORE="$WORK/before.tsv"
AFTER="$WORK/after.tsv"
: > "$BEFORE"
: > "$AFTER"

# 1. Pre-state: read scores at BASE_REF, not HEAD. Skip fixtures that didn't
#    exist there (`git show` will fail; we tolerate that).
for f in tests/divergence/*.md; do
    name=$(basename "$f" .md)
    git show "$BASE_REF:$f" 2>/dev/null | \
        awk -v n="$name" '
            /Score\(3000\)=/ {
                match($0, /Score\(3000\)=[0-9.]+/);
                print n "\t" substr($0, RSTART+13, RLENGTH-13);
                exit
            }
        ' >> "$BEFORE" || true
done

# 2. Regenerate baselines against the current working tree.
echo "Regenerating baselines (UPDATE_BASELINES=1 cargo t)…" >&2
UPDATE_BASELINES=1 cargo t --quiet 2>&1 | tail -3 >&2

# 3. Post-state: read regenerated scores.
for f in tests/divergence/*.md; do
    name=$(basename "$f" .md)
    head -1 "$f" | awk -v n="$name" '
        /Score\(3000\)=/ {
            match($0, /Score\(3000\)=[0-9.]+/);
            print n "\t" substr($0, RSTART+13, RLENGTH-13)
        }
    ' >> "$AFTER"
done

# 4. Diff and gate. `join -a 2 -e MISSING` keeps fixtures that didn't exist at
#    BASE_REF (e.g. new fixtures' reports) so we can report them without false-
#    positiving the gate.
sort "$BEFORE" -o "$BEFORE"
sort "$AFTER" -o "$AFTER"
join -t $'\t' -a 2 -e MISSING -o 0,1.2,2.2 "$BEFORE" "$AFTER" | \
    awk -F'\t' -v thresh="$THRESHOLD" -v targets="$TARGETS" '
        BEGIN {
            n = split(targets, t, ",");
            for (i = 1; i <= n; i++) if (t[i] != "") target[t[i]] = 1;
            failed = 0
        }
        {
            if ($2 == "MISSING") {
                printf "%-26s before=NEW    after=%s   (new fixture, skipping gate)\n", $1, $3;
                next
            }
            delta = $3 - $2;
            absdelta = (delta < 0) ? -delta : delta;
            if (absdelta > thresh) {
                tag = ($1 in target) ? "ALLOWED" : "REGRESSION";
            } else {
                tag = "OK"
            }
            printf "%-26s before=%s after=%s delta=%+.3f %s\n", $1, $2, $3, delta, tag;
            if (tag == "REGRESSION") failed = 1
        }
        END { exit failed }
    '

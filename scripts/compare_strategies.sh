#!/usr/bin/env bash
# Compare ContenderPool::AbsoluteK(K) vs ContenderPool::RelativeRatio(r)
# on all 66 fixtures, recording exact-tokenizations-per-best_exact-call
# under --features timing. Uses PRECIS_CONTENDER_POOL env override.
#
# Usage: scripts/compare_strategies.sh <K> <r>
#   e.g. scripts/compare_strategies.sh 66 0.55
set -euo pipefail

K="${1:-66}"
R="${2:-0.55}"

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
FIXTURES="$ROOT/tests/fixtures"
OUT="$ROOT/ignore/perf-2026-05-15/strategy-compare.tsv"

cd "$ROOT"
cargo build --release --features timing >/dev/null 2>&1
BIN="$ROOT/target/release/precis"

FIXTURE_LIST=$(mktemp)
trap 'rm -f "$FIXTURE_LIST"' EXIT
# shellcheck source=scripts/lib/list_fixtures.sh
source "$ROOT/scripts/lib/list_fixtures.sh"
PRECIS_ROOT="$ROOT" list_fixtures | cut -f1 > "$FIXTURE_LIST"

echo "fixture	iters	A_total	A_mean	B_total	B_mean" > "$OUT"

while IFS= read -r name; do
  [[ -d "$FIXTURES/$name" ]] || continue
  a=$(PRECIS_CONTENDER_POOL="absolute:$K" "$BIN" "$FIXTURES/$name" 2>&1 >/dev/null | grep pool_stats)
  b=$(PRECIS_CONTENDER_POOL="relative:$R" "$BIN" "$FIXTURES/$name" 2>&1 >/dev/null | grep pool_stats)
  a_total=$(echo "$a" | grep -oE 'exact_misses_total=[0-9]+' | head -1 | cut -d= -f2)
  a_mean=$(echo "$a" | grep -oE 'exact_misses_mean_per_iter=[0-9.]+' | head -1 | cut -d= -f2)
  iters=$(echo "$a" | grep -oE 'iterations=[0-9]+' | head -1 | cut -d= -f2)
  b_total=$(echo "$b" | grep -oE 'exact_misses_total=[0-9]+' | head -1 | cut -d= -f2)
  b_mean=$(echo "$b" | grep -oE 'exact_misses_mean_per_iter=[0-9.]+' | head -1 | cut -d= -f2)
  printf "%s\t%s\t%s\t%s\t%s\t%s\n" "$name" "$iters" "$a_total" "$a_mean" "$b_total" "$b_mean" >> "$OUT"
done < "$FIXTURE_LIST"

python3 - "$OUT" <<'PY'
import sys, csv
with open(sys.argv[1]) as f:
    rdr = csv.reader(f, delimiter="\t")
    header = next(rdr)
    rows = list(rdr)
a_total = sum(int(r[2]) for r in rows)
b_total = sum(int(r[4]) for r in rows)
a_wins = sum(1 for r in rows if int(r[2]) < int(r[4]))
b_wins = sum(1 for r in rows if int(r[4]) < int(r[2]))
ties = sum(1 for r in rows if int(r[2]) == int(r[4]))
worst_a = max(rows, key=lambda r: int(r[2]))
worst_b = max(rows, key=lambda r: int(r[4]))
print(f"\n=== Strategy comparison (n={len(rows)}) ===")
print(f"A (Absolute K) total exact tokenizations: {a_total}")
print(f"B (Relative R) total exact tokenizations: {b_total}")
print(f"A wins per-fixture: {a_wins}, B wins: {b_wins}, ties: {ties}")
print(f"A worst-case fixture: {worst_a[0]} = {worst_a[2]}")
print(f"B worst-case fixture: {worst_b[0]} = {worst_b[4]}")
PY

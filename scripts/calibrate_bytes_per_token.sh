#!/usr/bin/env bash
# Calibrate `bytes / tokens` for the precis approximate token estimator.
# Runs precis across all 66 fixtures with PRECIS_CALIBRATE=1 and parses
# `[calib] bytes=X tokens=Y` lines emitted from the marginal-cost visitor.
# Prints the global median (and per-language medians for sanity-check).
#
# Requires --features timing (the calibration hook is gated).
#
# Fixtures are read from tests/fixtures (pinned by clone_fixtures; the
# existing `tests/fixture_baselines.rs` pin test validates contents).
# Fresh checkouts must run `cargo run --bin clone_fixtures` first; the
# script aborts if any expected fixture dir is missing.
#
# Usage: scripts/calibrate_bytes_per_token.sh
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
FIXTURES="$ROOT/tests/fixtures"
OUT_DIR="$ROOT/ignore/perf-2026-05-15"
RAW="$OUT_DIR/calibration-raw.tsv"
SUMMARY="$OUT_DIR/calibration-summary.md"

mkdir -p "$OUT_DIR"

FIXTURE_LIST=$(mktemp)
trap 'rm -f "$FIXTURE_LIST"' EXIT
# shellcheck source=scripts/lib/list_fixtures.sh
source "$ROOT/scripts/lib/list_fixtures.sh"
PRECIS_ROOT="$ROOT" list_fixtures > "$FIXTURE_LIST"

# Pre-flight: every fixture present, every pin file present.
missing=0
while IFS=$'\t' read -r name _lang; do
  [[ -d "$FIXTURES/$name" ]] || { echo "MISSING fixture dir: $name"; missing=$((missing+1)); }
  [[ -f "$FIXTURES/$name/.precis-pin" ]] || { echo "MISSING .precis-pin: $name"; missing=$((missing+1)); }
done < "$FIXTURE_LIST"
if (( missing > 0 )); then
  echo "Aborting: $missing fixture(s) missing or unpinned. Run:"
  echo "  cargo run --bin clone_fixtures"
  exit 1
fi

echo "Building precis (release + timing) ..."
(cd "$ROOT" && cargo build --release --features timing)
BIN="$ROOT/target/release/precis"

echo "language	bytes	tokens" > "$RAW"
total=$(wc -l < "$FIXTURE_LIST" | tr -d ' ')

i=0
while IFS=$'\t' read -r name lang; do
  i=$((i+1))
  echo "[$i/$total] $name ($lang)"
  PRECIS_CALIBRATE=1 "$BIN" "$FIXTURES/$name" 2>&1 >/dev/null \
    | awk -v lang="$lang" '
        /^\[calib\] / {
          b=""; t=""
          for (j=1;j<=NF;j++) {
            if ($j ~ /^bytes=/)  { sub(/^bytes=/, "", $j);  b=$j }
            if ($j ~ /^tokens=/) { sub(/^tokens=/, "", $j); t=$j }
          }
          if (b != "" && t != "" && t+0 > 0) print lang "\t" b "\t" t
        }
      ' >> "$RAW"
done < "$FIXTURE_LIST"

echo "Computing medians ..."
python3 - "$RAW" "$SUMMARY" <<'PY'
import sys, statistics, collections
raw, summary = sys.argv[1], sys.argv[2]
ratios = []
by_lang = collections.defaultdict(list)
with open(raw) as f:
    f.readline()
    for line in f:
        parts = line.rstrip("\n").split("\t")
        if len(parts) != 3:
            continue
        lang, b, t = parts
        b, t = int(b), int(t)
        if t == 0:
            continue
        r = b / t
        ratios.append(r)
        by_lang[lang].append(r)
median = statistics.median(ratios)
with open(summary, "w") as out:
    out.write("# Calibration summary\n\n")
    out.write(f"Rows recorded: {len(ratios)}\n\n")
    out.write(f"**Global median bytes/token: {median:.4f}**\n\n")
    out.write("| language | rows | median | mean |\n|---|---:|---:|---:|\n")
    for lang in sorted(by_lang):
        rs = by_lang[lang]
        out.write(f"| {lang} | {len(rs)} | {statistics.median(rs):.4f} | {statistics.mean(rs):.4f} |\n")
print(f"Wrote {summary}")
print(f"Global median bytes/token = {median:.4f}")
PY
echo "Done."

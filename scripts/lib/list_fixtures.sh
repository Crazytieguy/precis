#!/usr/bin/env bash
# Emit `name\tlang` for every fixture declared in tests/data/fixtures.rs.
# Used by sweep / calibration / strategy-compare scripts so the list
# never drifts between consumers.
#
# Usage:  source list_fixtures.sh; list_fixtures > fixtures.tsv
list_fixtures() {
  local root fixtures_rs
  root="${PRECIS_ROOT:-$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)}"
  fixtures_rs="$root/tests/data/fixtures.rs"
  awk '
    /^[[:space:]]*\/\/ / { lang = substr($0, index($0, "//") + 3); next }
    /^[[:space:]]*\("/ {
      match($0, /"[^"]+"/)
      name = substr($0, RSTART+1, RLENGTH-2)
      print name "\t" lang
    }
  ' "$fixtures_rs"
}

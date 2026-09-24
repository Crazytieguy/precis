#!/usr/bin/env bash
# Prints the corpus-mean Score at every grid budget, averaged over the
# training divergence reports' `grid(…)=…` headlines. Reads the committed
# (or freshly regenerated) reports only — run `UPDATE_BASELINES=1 cargo t
# fixture_baselines` first to measure the working tree.
#
# Usage:
#   bash scripts/grid-means.sh
set -euo pipefail

awk '
    FNR == 1 && match($0, /grid\([0-9\/]+\)=[0-9.\/]+/) {
        field = substr($0, RSTART + 5, RLENGTH - 5)
        split(field, halves, "=")
        header = substr(halves[1], 1, length(halves[1]) - 1)
        k = split(halves[2], scores, "/")
        for (i = 1; i <= k; i++) sum[i] += scores[i]
        n++
    }
    END {
        if (n == 0) { print "grid-means: no grid headlines found" > "/dev/stderr"; exit 1 }
        gsub("/", "\t", header)
        print header
        line = ""
        for (i = 1; i <= k; i++) line = line (i > 1 ? "\t" : "") sprintf("%.4f", sum[i] / n)
        print line
        printf "(%d fixtures)\n", n
    }
' tests/divergence/*.md

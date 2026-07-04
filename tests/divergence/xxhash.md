Score(3000)=0.724 I=0.907 C=0.577 ns_rows≤3K=20/40 (reached=11 partial=5 missing=4)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 83 | 83 | listing of '.' |  |  | 1.000 |
| ns | 83 |  | 83 | Fixture top-level listing | 1.1 |  | 1.000 |
| walker |  | 145 | 62 | README headline in README.md |  |  | 1.000 |
| ns | 145 |  | 62 | README — title and one-sentence positioning | 1.2 |  | 1.000 |
| walker |  | 176 | 31 | headings outline in README.md |  |  | 1.000 |
| walker |  | 207 | 31 | go module identity in go.mod |  |  | 1.000 |
| walker |  | 207 | 0 | go module file go.mod |  |  | 1.000 |
| ns | 221 |  | 76 | Package doc + module path | 1.3 |  | 0.883 |
| walker |  | 244 | 37 | go package doc lede in xxhash.go |  |  | 0.969 |
| walker |  | 255 | 11 | listing of 'xxhsum' |  |  | 0.969 |
| walker |  | 269 | 14 | listing of 'dynamic' |  |  | 0.971 |
| walker |  | 277 | 8 | go package + imports in xxhash_asm.go |  |  | 0.971 |
| walker |  | 285 | 8 | go package + imports in xxhash_safe.go |  |  | 0.971 |
| walker |  | 300 | 15 | listing of 'xxhashbench' |  |  | 0.975 |
| walker |  | 333 | 33 | go module identity in xxhashbench/go.mod |  |  | 0.975 |
| walker |  | 366 | 33 | go decl names surface in xxhash_asm.go |  |  | 0.975 |
| walker |  | 366 | 0 | go decl at xxhash_asm.go:12 |  |  | 0.975 |
| walker |  | 366 | 0 | go decl at xxhash_asm.go:15 |  |  | 0.975 |
| ns | 387 |  | 166 | README — public API code-fence sketch | 1.4 |  | 0.744 |
| walker |  | 407 | 41 | go decl names surface in xxhash_safe.go |  |  | 0.744 |
| walker |  | 407 | 0 | go decl at xxhash_safe.go:9 |  |  | 0.744 |
| walker |  | 407 | 0 | go decl at xxhash_safe.go:14 |  |  | 0.744 |
| walker |  | 416 | 9 | go decl body at xxhash_safe.go:9 |  |  | 0.744 |
| walker |  | 425 | 9 | go decl body at xxhash_safe.go:14 |  |  | 0.744 |
| ns | 463 |  | 76 | README — purego/asm note | 1.5 |  | 0.698 |
| walker |  | 477 | 52 | go decl names surface in xxhash_unsafe.go |  |  | 0.698 |
| walker |  | 477 | 0 | go decl at xxhash_unsafe.go:38 |  |  | 0.698 |
| walker |  | 477 | 0 | go decl at xxhash_unsafe.go:45 |  |  | 0.698 |
| walker |  | 496 | 19 | go decl at xxhash_unsafe.go:55 |  |  | 0.698 |
| walker |  | 505 | 9 | go decl doc at xxhash_asm.go:15 |  |  | 0.698 |
| walker |  | 528 | 23 | go decl doc at xxhash_safe.go:9 |  |  | 0.699 |
| walker |  | 549 | 21 | go decl doc at xxhash_safe.go:14 |  |  | 0.699 |
| walker |  | 599 | 50 | go package + imports in xxhash.go |  |  | 0.721 |
| ns | 603 |  | 140 | Digest struct + zero-value caveat | 2.1 |  | 0.627 |
| walker |  | 643 | 44 | go decl names surface in xxhsum/xxhsum.go |  |  | 0.627 |
| walker |  | 643 | 0 | go decl at xxhsum/xxhsum.go:11 |  |  | 0.627 |
| walker |  | 643 | 0 | go decl at xxhsum/xxhsum.go:34 |  |  | 0.627 |
| walker |  | 643 | 0 | go decl at xxhsum/xxhsum.go:43 |  |  | 0.627 |
| walker |  | 679 | 36 | go decl doc at xxhash_asm.go:12 |  |  | 0.629 |
| walker |  | 708 | 29 | go decl body at xxhash_unsafe.go:38 |  |  | 0.630 |
| ns | 712 |  | 109 | Constructors — New and NewWithSeed | 2.2 | 2.1 | 0.571 |
| walker |  | 740 | 32 | go package + imports in xxhash_unsafe.go |  |  | 0.571 |
| walker |  | 784 | 44 | go decl doc at xxhash_unsafe.go:38 |  |  | 0.572 |
| walker |  | 825 | 41 | go decl doc at xxhash_unsafe.go:45 |  |  | 0.574 |
| ns | 907 |  | 195 | Reset / ResetWithSeed — full bodies | 2.3 | 2.1 | 0.511 |
| walker |  | 940 | 115 | go module file xxhashbench/go.mod |  |  | 0.511 |
| walker |  | 1018 | 78 | go decl names surface in dynamic/plugin.go |  |  | 0.511 |
| walker |  | 1018 | 0 | go decl at dynamic/plugin.go:19 |  |  | 0.511 |
| walker |  | 1018 | 0 | go decl at dynamic/plugin.go:26 |  |  | 0.511 |
| walker |  | 1027 | 9 | go decl at dynamic/plugin.go:14 |  |  | 0.511 |
| ns | 1077 |  | 170 | Remaining Digest methods — Size, BlockSize, Write, Sum, Sum64 signatures | 2.4 | 2.1 | 0.480 |
| ns | 1196 |  | 119 | Sum64 / writeBlocks signatures (asm build) | 2.5 |  | 0.491 |
| walker |  | 1274 | 247 | README.md section #0 |  |  | 0.665 |
| walker |  | 1282 | 8 | go package + imports in xxhash_other.go |  |  | 0.665 |
| walker |  | 1324 | 42 | go decl doc at xxhash_unsafe.go:55 |  |  | 0.666 |
| ns | 1341 |  | 145 | Sum64String / WriteString signatures (unsafe build) | 2.6 |  | 0.669 |
| walker |  | 1359 | 35 | go decl names surface in xxhash_other.go |  |  | 0.669 |
| walker |  | 1359 | 0 | go decl at xxhash_other.go:7 |  |  | 0.669 |
| walker |  | 1359 | 0 | go decl at xxhash_other.go:64 |  |  | 0.669 |
| ns | 1460 |  | 119 | MarshalBinary / UnmarshalBinary signatures + magic constants | 2.7 | 2.1 | 0.643 |
| ns | 1507 |  | 47 | Sub-directory listings (.github, dynamic, xxhsum, xxhashbench) | 3.1 | 1.1 | 0.646 |
| ns | 1648 |  | 141 | All test/benchmark function names across the repo | 3.2 |  | 0.606 |
| ns | 1765 |  | 117 | testall.sh — full body | 3.3 |  | 0.583 |
| walker |  | 2074 | 715 | go decl names surface in xxhash.go |  |  | 0.618 |
| walker |  | 2074 | 0 | go decl at xxhash.go:23 |  |  | 0.618 |
| walker |  | 2074 | 0 | go decl at xxhash.go:40 |  |  | 0.618 |
| walker |  | 2074 | 0 | go decl at xxhash.go:45 |  |  | 0.618 |
| walker |  | 2074 | 0 | go decl at xxhash.go:53 |  |  | 0.618 |
| walker |  | 2074 | 0 | go decl at xxhash.go:59 |  |  | 0.618 |
| walker |  | 2074 | 0 | go decl at xxhash.go:69 |  |  | 0.618 |
| walker |  | 2074 | 0 | go decl at xxhash.go:72 |  |  | 0.618 |
| walker |  | 2074 | 0 | go decl at xxhash.go:75 |  |  | 0.618 |
| walker |  | 2074 | 0 | go decl at xxhash.go:113 |  |  | 0.618 |
| walker |  | 2074 | 0 | go decl at xxhash.go:129 |  |  | 0.618 |
| walker |  | 2074 | 0 | go decl at xxhash.go:176 |  |  | 0.618 |
| walker |  | 2074 | 0 | go decl at xxhash.go:190 |  |  | 0.618 |
| walker |  | 2074 | 0 | go decl at xxhash.go:208 |  |  | 0.618 |
| walker |  | 2074 | 0 | go decl at xxhash.go:214 |  |  | 0.618 |
| walker |  | 2074 | 0 | go decl at xxhash.go:222 |  |  | 0.618 |
| walker |  | 2074 | 0 | go decl at xxhash.go:229 |  |  | 0.618 |
| ns | 2081 |  | 316 | xxhsum CLI — main + usage | 3.4 |  | 0.555 |
| walker |  | 2083 | 9 | go decl body at xxhash.go:40 |  |  | 0.558 |
| walker |  | 2092 | 9 | go decl body at xxhash.go:53 |  |  | 0.559 |
| walker |  | 2101 | 9 | go decl at xxhash.go:11 |  |  | 0.559 |
| walker |  | 2110 | 9 | go decl at xxhash.go:170 |  |  | 0.562 |
| walker |  | 2122 | 12 | go decl doc at xxhash.go:69 |  |  | 0.566 |
| walker |  | 2134 | 12 | go decl doc at xxhash.go:129 |  |  | 0.571 |
| walker |  | 2147 | 13 | go decl doc at xxhash.go:72 |  |  | 0.577 |
| walker |  | 2162 | 15 | go decl doc at xxhash.go:40 |  |  | 0.582 |
| walker |  | 2179 | 17 | go decl doc at xxhash.go:45 |  |  | 0.590 |
| walker |  | 2195 | 16 | go decl doc at xxhash.go:176 |  |  | 0.599 |
| walker |  | 2213 | 18 | go decl doc at xxhash.go:190 |  |  | 0.610 |
| walker |  | 2232 | 19 | go decl doc at xxhash.go:113 |  |  | 0.619 |
| walker |  | 2252 | 20 | go decl doc at xxhash.go:75 |  |  | 0.628 |
| walker |  | 2276 | 24 | go decl body at xxhash.go:45 |  |  | 0.645 |
| walker |  | 2307 | 31 | go decl doc at xxhash.go:53 |  |  | 0.652 |
| walker |  | 2384 | 77 | go decl at xxhash.go:29 |  |  | 0.669 |
| ns | 2400 |  | 319 | CI matrix — what configurations are tested | 3.5 |  | 0.615 |
| walker |  | 2419 | 35 | go decl doc at xxhash.go:59 |  |  | 0.627 |
| walker |  | 2472 | 53 | go decl doc at xxhash.go:29 |  |  | 0.664 |
| walker |  | 2491 | 19 | go decl body at xxhash.go:214 |  |  | 0.664 |
| walker |  | 2561 | 70 | go decl body at xxhash.go:59 |  |  | 0.693 |
| ns | 2592 |  | 192 | Prime constants + primes array | 4.1 |  | 0.684 |
| walker |  | 2596 | 35 | go decl body at xxhash.go:222 |  |  | 0.685 |
| walker |  | 2633 | 37 | go decl body at xxhash.go:208 |  |  | 0.686 |
| walker |  | 2688 | 55 | go decl doc at xxhash.go:23 |  |  | 0.707 |
| ns | 2716 |  | 124 | round + mergeRound — the core mixer | 4.2 | 4.1 | 0.696 |
| walker |  | 2727 | 39 | go decl body at xxhash.go:229 |  |  | 0.712 |
| walker |  | 2833 | 106 | go decl body at xxhash.go:113 |  |  | 0.715 |
| ns | 2918 |  | 202 | rol* one-line helpers (locations) | 4.3 |  | 0.722 |
| walker |  | 2976 | 143 | go decl body at xxhash.go:176 |  |  | 0.724 |
| ns | 3084 |  | 166 | Little-endian / append / consume byte helpers | 4.4 |  | 0.724 |
| walker |  | 3112 | 136 | README.md section #1 |  |  | 0.725 |
| walker |  | 3134 | 22 | go decl doc at xxhash_other.go:7 |  |  | 0.725 |
| walker |  | 3183 | 49 | go decl body at xxhsum/xxhsum.go:34 |  |  | 0.725 |
| walker |  | 3265 | 82 | go decl body at xxhash_unsafe.go:45 |  |  | 0.726 |
| walker |  | 3330 | 65 | go package + imports in dynamic/plugin.go |  |  | 0.726 |
| walker |  | 3395 | 65 | go package + imports in xxhsum/xxhsum.go |  |  | 0.730 |
| walker |  | 3449 | 54 | go decl body at dynamic/plugin.go:19 |  |  | 0.730 |
| walker |  | 3452 | 3 | listing of '.github' |  |  | 0.735 |
| walker |  | 3456 | 4 | listing of '.github/workflows' |  |  | 0.740 |
| ns | 3473 |  | 389 | Write — streaming entry body | 4.5 | 2.4 | 0.687 |
| walker |  | 3658 | 202 | go decl body at xxhash.go:190 |  |  | 0.690 |
| walker |  | 3805 | 147 | README.md section #3 |  |  | 0.690 |
| walker |  | 3878 | 73 | go decl body at xxhsum/xxhsum.go:43 |  |  | 0.690 |
| ns | 3949 |  | 476 | Digest.Sum64 — finalize body | 4.6 | 2.4 | 0.642 |
| ns | 4062 |  | 113 | Digest.Sum — append big-endian bytes | 4.7 | 2.4 | 0.653 |
| ns | 4212 |  | 150 | MarshalBinary body — wire format | 4.8 | 2.7 | 0.661 |
| walker |  | 4260 | 382 | go decl body at xxhash.go:75 |  |  | 0.729 |
| ns | 4421 |  | 209 | UnmarshalBinary body — validation + parse | 4.9 | 2.7 | 0.736 |
| walker |  | 4571 | 311 | README.md section #2 |  |  | 0.738 |
| walker |  | 5040 | 469 | go decl body at xxhash.go:129 |  |  | 0.805 |
| ns | 5119 |  | 698 | Pure-Go Sum64 body (xxhash_other.go) | 4.10 | 2.5 | 0.746 |
| walker |  | 5271 | 231 | go decl body at xxhsum/xxhsum.go:11 |  |  | 0.793 |
| ns | 5355 |  | 236 | Pure-Go writeBlocks body | 4.11 | 2.5 | 0.780 |
| ns | 5535 |  | 180 | xxhash_safe.go full body — the appengine fallback | 4.12 | 2.6 | 0.771 |
| walker |  | 5545 | 274 | go decl body at dynamic/plugin.go:26 |  |  | 0.771 |
| walker |  | 5553 | 8 | plaintext config dynamic/.gitignore |  |  | 0.771 |
| walker |  | 5568 | 15 | go test names surface in xxhashbench/xxhashbench_test.go |  |  | 0.771 |
| walker |  | 5600 | 32 | go test names surface in xxhash_unsafe_test.go |  |  | 0.773 |
| walker |  | 5609 | 9 | plaintext config xxhsum/.gitignore |  |  | 0.773 |
| walker |  | 5821 | 212 | go decl body at xxhash_other.go:64 |  |  | 0.789 |
| walker |  | 5851 | 30 | go test names surface in dynamic/dynamic_test.go |  |  | 0.791 |
| walker |  | 5916 | 65 | go test names surface in bench_test.go |  |  | 0.799 |
| ns | 5940 |  | 405 | Unsafe string conversion — inlining commentary + sliceHeader | 4.13 | 2.6 | 0.775 |
| ns | 6070 |  | 130 | Unsafe Sum64String / WriteString bodies | 4.14 | 2.6 | 0.774 |
| ns | 6391 |  | 321 | README — Benchmarks section (perf table) | 5.1 |  | 0.779 |
| walker |  | 6593 | 677 | go decl body at xxhash_other.go:7 |  |  | 0.845 |
| walker |  | 6674 | 81 | go test names surface in xxhash_test.go |  |  | 0.857 |
| ns | 6724 |  | 333 | TestAll — known-answer test vectors | 5.2 | 3.2 | 0.841 |
| walker |  | 6986 | 312 | plaintext config LICENSE.txt |  |  | 0.841 |
| ns | 7016 |  | 292 | bench_test.go — input-size grid + benchmark shape | 5.3 | 3.2 | 0.814 |
| ns | 7396 |  | 380 | xxhashbench — structure of the cross-library benchmark | 5.4 | 3.2 | 0.788 |
| ns | 8093 |  | 697 | amd64 asm — register defs and macros | 5.5 |  | 0.749 |
| ns | 8652 |  | 559 | amd64 asm — Sum64 prologue, state setup, and main block loop | 5.6 | 5.5 | 0.721 |
| ns | 9358 |  | 706 | amd64 asm — tail handlers (loop8 / try4 / try1) + finalize | 5.7 | 5.5 | 0.685 |
| ns | 9801 |  | 443 | amd64 asm — writeBlocks body | 5.8 | 5.5 | 0.669 |
| ns | 9946 |  | 145 | README — Compatibility (supported Go versions) | 6.1 |  | 0.673 |

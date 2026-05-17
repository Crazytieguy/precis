Score(3000)=0.700 I=0.880 C=0.557 ns_rows≤3K=20/40 (reached=10 partial=6 missing=4)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 83 | 83 | listing of '.' |  |  | 1.000 |
| ns | 83 |  | 83 | Fixture top-level listing | 1.1 |  | 1.000 |
| walker |  | 109 | 26 | go module identity in go.mod |  |  | 1.000 |
| walker |  | 109 | 0 | go module file go.mod |  |  | 1.000 |
| ns | 145 |  | 62 | README — title and one-sentence positioning | 1.2 |  | 0.889 |
| walker |  | 146 | 37 | go package doc lede in xxhash.go |  |  | 0.903 |
| walker |  | 157 | 11 | listing of 'xxhsum' |  |  | 0.903 |
| walker |  | 219 | 62 | README headline in README.md |  |  | 1.000 |
| ns | 221 |  | 76 | Package doc + module path | 1.3 |  | 0.948 |
| walker |  | 250 | 31 | headings outline in README.md |  |  | 0.948 |
| walker |  | 264 | 14 | listing of 'dynamic' |  |  | 0.950 |
| walker |  | 272 | 8 | go package + imports in xxhash_asm.go |  |  | 0.950 |
| walker |  | 280 | 8 | go package + imports in xxhash_safe.go |  |  | 0.950 |
| walker |  | 295 | 15 | listing of 'xxhashbench' |  |  | 0.953 |
| walker |  | 323 | 28 | go module identity in xxhashbench/go.mod |  |  | 0.953 |
| walker |  | 356 | 33 | go decl names surface in xxhash_asm.go |  |  | 0.954 |
| walker |  | 356 | 0 | go decl at xxhash_asm.go:12 |  |  | 0.954 |
| walker |  | 356 | 0 | go decl at xxhash_asm.go:15 |  |  | 0.954 |
| ns | 387 |  | 166 | README — public API code-fence sketch | 1.4 |  | 0.727 |
| walker |  | 397 | 41 | go decl names surface in xxhash_safe.go |  |  | 0.728 |
| walker |  | 397 | 0 | go decl at xxhash_safe.go:9 |  |  | 0.728 |
| walker |  | 397 | 0 | go decl at xxhash_safe.go:14 |  |  | 0.728 |
| walker |  | 406 | 9 | go decl body at xxhash_safe.go:9 |  |  | 0.728 |
| walker |  | 415 | 9 | go decl body at xxhash_safe.go:14 |  |  | 0.728 |
| ns | 463 |  | 76 | README — purego/asm note | 1.5 |  | 0.682 |
| walker |  | 467 | 52 | go decl names surface in xxhash_unsafe.go |  |  | 0.683 |
| walker |  | 467 | 0 | go decl at xxhash_unsafe.go:38 |  |  | 0.683 |
| walker |  | 467 | 0 | go decl at xxhash_unsafe.go:45 |  |  | 0.683 |
| walker |  | 486 | 19 | go decl at xxhash_unsafe.go:55 |  |  | 0.683 |
| walker |  | 495 | 9 | go decl doc at xxhash_asm.go:15 |  |  | 0.683 |
| walker |  | 518 | 23 | go decl doc at xxhash_safe.go:9 |  |  | 0.683 |
| walker |  | 539 | 21 | go decl doc at xxhash_safe.go:14 |  |  | 0.684 |
| walker |  | 584 | 45 | go package + imports in xxhash.go |  |  | 0.705 |
| ns | 603 |  | 140 | Digest struct + zero-value caveat | 2.1 |  | 0.613 |
| walker |  | 628 | 44 | go decl names surface in xxhsum/xxhsum.go |  |  | 0.613 |
| walker |  | 628 | 0 | go decl at xxhsum/xxhsum.go:11 |  |  | 0.613 |
| walker |  | 628 | 0 | go decl at xxhsum/xxhsum.go:34 |  |  | 0.613 |
| walker |  | 628 | 0 | go decl at xxhsum/xxhsum.go:43 |  |  | 0.613 |
| walker |  | 664 | 36 | go decl doc at xxhash_asm.go:12 |  |  | 0.615 |
| walker |  | 691 | 27 | go package + imports in xxhash_unsafe.go |  |  | 0.615 |
| ns | 712 |  | 109 | Constructors — New and NewWithSeed | 2.2 | 2.1 | 0.558 |
| walker |  | 720 | 29 | go decl body at xxhash_unsafe.go:38 |  |  | 0.558 |
| walker |  | 764 | 44 | go decl doc at xxhash_unsafe.go:38 |  |  | 0.559 |
| walker |  | 869 | 105 | go module file xxhashbench/go.mod |  |  | 0.559 |
| ns | 907 |  | 195 | Reset / ResetWithSeed — full bodies | 2.3 | 2.1 | 0.497 |
| walker |  | 910 | 41 | go decl doc at xxhash_unsafe.go:45 |  |  | 0.499 |
| walker |  | 988 | 78 | go decl names surface in dynamic/plugin.go |  |  | 0.499 |
| walker |  | 988 | 0 | go decl at dynamic/plugin.go:19 |  |  | 0.499 |
| walker |  | 988 | 0 | go decl at dynamic/plugin.go:26 |  |  | 0.499 |
| walker |  | 997 | 9 | go decl at dynamic/plugin.go:14 |  |  | 0.499 |
| ns | 1077 |  | 170 | Remaining Digest methods — Size, BlockSize, Write, Sum, Sum64 signatures | 2.4 | 2.1 | 0.469 |
| ns | 1196 |  | 119 | Sum64 / writeBlocks signatures (asm build) | 2.5 |  | 0.481 |
| walker |  | 1219 | 222 | README.md section #0 |  |  | 0.623 |
| walker |  | 1227 | 8 | go package + imports in xxhash_other.go |  |  | 0.623 |
| walker |  | 1269 | 42 | go decl doc at xxhash_unsafe.go:55 |  |  | 0.623 |
| walker |  | 1304 | 35 | go decl names surface in xxhash_other.go |  |  | 0.623 |
| walker |  | 1304 | 0 | go decl at xxhash_other.go:7 |  |  | 0.623 |
| walker |  | 1304 | 0 | go decl at xxhash_other.go:64 |  |  | 0.623 |
| ns | 1341 |  | 145 | Sum64String / WriteString signatures (unsafe build) | 2.6 |  | 0.629 |
| ns | 1460 |  | 119 | MarshalBinary / UnmarshalBinary signatures + magic constants | 2.7 | 2.1 | 0.605 |
| ns | 1507 |  | 47 | Sub-directory listings (.github, dynamic, xxhsum, xxhashbench) | 3.1 | 1.1 | 0.611 |
| ns | 1648 |  | 141 | All test/benchmark function names across the repo | 3.2 |  | 0.573 |
| ns | 1765 |  | 117 | testall.sh — full body | 3.3 |  | 0.551 |
| walker |  | 2016 | 712 | go decl names surface in xxhash.go |  |  | 0.586 |
| walker |  | 2016 | 0 | go decl at xxhash.go:23 |  |  | 0.586 |
| walker |  | 2016 | 0 | go decl at xxhash.go:40 |  |  | 0.586 |
| walker |  | 2016 | 0 | go decl at xxhash.go:45 |  |  | 0.586 |
| walker |  | 2016 | 0 | go decl at xxhash.go:53 |  |  | 0.586 |
| walker |  | 2016 | 0 | go decl at xxhash.go:59 |  |  | 0.586 |
| walker |  | 2016 | 0 | go decl at xxhash.go:69 |  |  | 0.586 |
| walker |  | 2016 | 0 | go decl at xxhash.go:72 |  |  | 0.586 |
| walker |  | 2016 | 0 | go decl at xxhash.go:75 |  |  | 0.586 |
| walker |  | 2016 | 0 | go decl at xxhash.go:113 |  |  | 0.586 |
| walker |  | 2016 | 0 | go decl at xxhash.go:129 |  |  | 0.586 |
| walker |  | 2016 | 0 | go decl at xxhash.go:176 |  |  | 0.586 |
| walker |  | 2016 | 0 | go decl at xxhash.go:190 |  |  | 0.586 |
| walker |  | 2016 | 0 | go decl at xxhash.go:208 |  |  | 0.586 |
| walker |  | 2016 | 0 | go decl at xxhash.go:214 |  |  | 0.586 |
| walker |  | 2016 | 0 | go decl at xxhash.go:222 |  |  | 0.586 |
| walker |  | 2016 | 0 | go decl at xxhash.go:229 |  |  | 0.586 |
| walker |  | 2025 | 9 | go decl body at xxhash.go:40 |  |  | 0.589 |
| walker |  | 2034 | 9 | go decl body at xxhash.go:53 |  |  | 0.590 |
| walker |  | 2043 | 9 | go decl at xxhash.go:11 |  |  | 0.591 |
| walker |  | 2052 | 9 | go decl at xxhash.go:170 |  |  | 0.594 |
| walker |  | 2064 | 12 | go decl doc at xxhash.go:69 |  |  | 0.598 |
| walker |  | 2076 | 12 | go decl doc at xxhash.go:129 |  |  | 0.604 |
| ns | 2081 |  | 316 | xxhsum CLI — main + usage | 3.4 |  | 0.543 |
| walker |  | 2089 | 13 | go decl doc at xxhash.go:72 |  |  | 0.549 |
| walker |  | 2104 | 15 | go decl doc at xxhash.go:40 |  |  | 0.554 |
| walker |  | 2121 | 17 | go decl doc at xxhash.go:45 |  |  | 0.562 |
| walker |  | 2137 | 16 | go decl doc at xxhash.go:176 |  |  | 0.571 |
| walker |  | 2155 | 18 | go decl doc at xxhash.go:190 |  |  | 0.582 |
| walker |  | 2174 | 19 | go decl doc at xxhash.go:113 |  |  | 0.591 |
| walker |  | 2194 | 20 | go decl doc at xxhash.go:75 |  |  | 0.600 |
| walker |  | 2218 | 24 | go decl body at xxhash.go:45 |  |  | 0.617 |
| walker |  | 2249 | 31 | go decl doc at xxhash.go:53 |  |  | 0.624 |
| walker |  | 2326 | 77 | go decl at xxhash.go:29 |  |  | 0.642 |
| walker |  | 2361 | 35 | go decl doc at xxhash.go:59 |  |  | 0.655 |
| ns | 2400 |  | 319 | CI matrix — what configurations are tested | 3.5 |  | 0.602 |
| walker |  | 2414 | 53 | go decl doc at xxhash.go:29 |  |  | 0.639 |
| walker |  | 2433 | 19 | go decl body at xxhash.go:214 |  |  | 0.639 |
| walker |  | 2503 | 70 | go decl body at xxhash.go:59 |  |  | 0.668 |
| walker |  | 2538 | 35 | go decl body at xxhash.go:222 |  |  | 0.669 |
| walker |  | 2575 | 37 | go decl body at xxhash.go:208 |  |  | 0.670 |
| ns | 2592 |  | 192 | Prime constants + primes array | 4.1 |  | 0.662 |
| walker |  | 2630 | 55 | go decl doc at xxhash.go:23 |  |  | 0.684 |
| walker |  | 2669 | 39 | go decl body at xxhash.go:229 |  |  | 0.685 |
| ns | 2716 |  | 124 | round + mergeRound — the core mixer | 4.2 | 4.1 | 0.690 |
| walker |  | 2775 | 106 | go decl body at xxhash.go:113 |  |  | 0.692 |
| walker |  | 2898 | 123 | README.md section #1 |  |  | 0.693 |
| ns | 2918 |  | 202 | rol* one-line helpers (locations) | 4.3 |  | 0.700 |
| walker |  | 3041 | 143 | go decl body at xxhash.go:176 |  |  | 0.702 |
| ns | 3084 |  | 166 | Little-endian / append / consume byte helpers | 4.4 |  | 0.703 |
| walker |  | 3096 | 55 | go package + imports in dynamic/plugin.go |  |  | 0.703 |
| walker |  | 3151 | 55 | go package + imports in xxhsum/xxhsum.go |  |  | 0.706 |
| walker |  | 3173 | 22 | go decl doc at xxhash_other.go:7 |  |  | 0.706 |
| walker |  | 3255 | 82 | go decl body at xxhash_unsafe.go:45 |  |  | 0.707 |
| walker |  | 3309 | 54 | go decl body at dynamic/plugin.go:19 |  |  | 0.707 |
| walker |  | 3312 | 3 | listing of '.github' |  |  | 0.712 |
| walker |  | 3316 | 4 | listing of '.github/workflows' |  |  | 0.717 |
| ns | 3473 |  | 389 | Write — streaming entry body | 4.5 | 2.4 | 0.666 |
| walker |  | 3518 | 202 | go decl body at xxhash.go:190 |  |  | 0.669 |
| walker |  | 3662 | 144 | README.md section #3 |  |  | 0.669 |
| ns | 3949 |  | 476 | Digest.Sum64 — finalize body | 4.6 | 2.4 | 0.623 |
| walker |  | 4014 | 352 | go decl body at xxhash.go:75 |  |  | 0.685 |
| ns | 4062 |  | 113 | Digest.Sum — append big-endian bytes | 4.7 | 2.4 | 0.693 |
| walker |  | 4063 | 49 | go decl body at xxhsum/xxhsum.go:34 |  |  | 0.693 |
| ns | 4212 |  | 150 | MarshalBinary body — wire format | 4.8 | 2.7 | 0.699 |
| walker |  | 4356 | 293 | README.md section #2 |  |  | 0.701 |
| ns | 4421 |  | 209 | UnmarshalBinary body — validation + parse | 4.9 | 2.7 | 0.709 |
| walker |  | 4800 | 444 | go decl body at xxhash.go:129 |  |  | 0.767 |
| walker |  | 4873 | 73 | go decl body at xxhsum/xxhsum.go:43 |  |  | 0.767 |
| ns | 5119 |  | 698 | Pure-Go Sum64 body (xxhash_other.go) | 4.10 | 2.5 | 0.711 |
| walker |  | 5147 | 274 | go decl body at dynamic/plugin.go:26 |  |  | 0.711 |
| ns | 5355 |  | 236 | Pure-Go writeBlocks body | 4.11 | 2.5 | 0.699 |
| walker |  | 5378 | 231 | go decl body at xxhsum/xxhsum.go:11 |  |  | 0.743 |
| walker |  | 5386 | 8 | plaintext config dynamic/.gitignore |  |  | 0.743 |
| walker |  | 5401 | 15 | go test names surface in xxhashbench/xxhashbench_test.go |  |  | 0.743 |
| walker |  | 5433 | 32 | go test names surface in xxhash_unsafe_test.go |  |  | 0.745 |
| walker |  | 5442 | 9 | plaintext config xxhsum/.gitignore |  |  | 0.745 |
| ns | 5535 |  | 180 | xxhash_safe.go full body — the appengine fallback | 4.12 | 2.6 | 0.737 |
| walker |  | 5654 | 212 | go decl body at xxhash_other.go:64 |  |  | 0.753 |
| walker |  | 5684 | 30 | go test names surface in dynamic/dynamic_test.go |  |  | 0.755 |
| walker |  | 5749 | 65 | go test names surface in bench_test.go |  |  | 0.764 |
| ns | 5940 |  | 405 | Unsafe string conversion — inlining commentary + sliceHeader | 4.13 | 2.6 | 0.740 |
| ns | 6070 |  | 130 | Unsafe Sum64String / WriteString bodies | 4.14 | 2.6 | 0.740 |
| ns | 6391 |  | 321 | README — Benchmarks section (perf table) | 5.1 |  | 0.743 |
| walker |  | 6396 | 647 | go decl body at xxhash_other.go:7 |  |  | 0.802 |
| walker |  | 6477 | 81 | go test names surface in xxhash_test.go |  |  | 0.814 |
| ns | 6724 |  | 333 | TestAll — known-answer test vectors | 5.2 | 3.2 | 0.798 |
| walker |  | 6769 | 292 | plaintext config LICENSE.txt |  |  | 0.798 |
| ns | 7016 |  | 292 | bench_test.go — input-size grid + benchmark shape | 5.3 | 3.2 | 0.773 |
| ns | 7396 |  | 380 | xxhashbench — structure of the cross-library benchmark | 5.4 | 3.2 | 0.748 |
| ns | 8093 |  | 697 | amd64 asm — register defs and macros | 5.5 |  | 0.711 |
| ns | 8652 |  | 559 | amd64 asm — Sum64 prologue, state setup, and main block loop | 5.6 | 5.5 | 0.684 |
| ns | 9358 |  | 706 | amd64 asm — tail handlers (loop8 / try4 / try1) + finalize | 5.7 | 5.5 | 0.651 |
| ns | 9801 |  | 443 | amd64 asm — writeBlocks body | 5.8 | 5.5 | 0.636 |
| ns | 9946 |  | 145 | README — Compatibility (supported Go versions) | 6.1 |  | 0.638 |

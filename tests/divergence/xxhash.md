Score(3000)=0.678 I=0.854 C=0.538 ns_rows≤3K=20/40 (reached=10 partial=4 missing=6)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 83 | 83 | listing of '.' |  |  | 1.000 |
| ns | 83 |  | 83 | Fixture top-level listing | 1.1 |  | 1.000 |
| walker |  | 109 | 26 | go module file go.mod |  |  | 1.000 |
| walker |  | 120 | 11 | listing of 'xxhsum' |  |  | 1.000 |
| ns | 145 |  | 62 | README — title and one-sentence positioning | 1.2 |  | 0.889 |
| walker |  | 182 | 62 | README headline in README.md |  |  | 1.000 |
| walker |  | 213 | 31 | headings outline in README.md |  |  | 1.000 |
| ns | 221 |  | 76 | Package doc + module path | 1.3 |  | 0.877 |
| walker |  | 227 | 14 | listing of 'dynamic' |  |  | 0.879 |
| walker |  | 235 | 8 | go package + imports in xxhash_asm.go |  |  | 0.879 |
| walker |  | 243 | 8 | go package + imports in xxhash_safe.go |  |  | 0.879 |
| walker |  | 258 | 15 | listing of 'xxhashbench' |  |  | 0.883 |
| walker |  | 291 | 33 | go decl names surface in xxhash_asm.go |  |  | 0.883 |
| walker |  | 291 | 0 | go decl at xxhash_asm.go:12 |  |  | 0.883 |
| walker |  | 291 | 0 | go decl at xxhash_asm.go:15 |  |  | 0.883 |
| walker |  | 332 | 41 | go decl names surface in xxhash_safe.go |  |  | 0.884 |
| walker |  | 332 | 0 | go decl at xxhash_safe.go:9 |  |  | 0.884 |
| walker |  | 332 | 0 | go decl at xxhash_safe.go:14 |  |  | 0.884 |
| walker |  | 341 | 9 | go decl body at xxhash_safe.go:9 |  |  | 0.884 |
| walker |  | 350 | 9 | go decl body at xxhash_safe.go:14 |  |  | 0.884 |
| ns | 387 |  | 166 | README — public API code-fence sketch | 1.4 |  | 0.674 |
| walker |  | 402 | 52 | go decl names surface in xxhash_unsafe.go |  |  | 0.674 |
| walker |  | 402 | 0 | go decl at xxhash_unsafe.go:38 |  |  | 0.674 |
| walker |  | 402 | 0 | go decl at xxhash_unsafe.go:45 |  |  | 0.674 |
| walker |  | 421 | 19 | go decl at xxhash_unsafe.go:55 |  |  | 0.674 |
| walker |  | 430 | 9 | go decl doc at xxhash_asm.go:15 |  |  | 0.674 |
| walker |  | 453 | 23 | go decl doc at xxhash_safe.go:9 |  |  | 0.675 |
| ns | 463 |  | 76 | README — purego/asm note | 1.5 |  | 0.633 |
| walker |  | 474 | 21 | go decl doc at xxhash_safe.go:14 |  |  | 0.633 |
| walker |  | 518 | 44 | go decl names surface in xxhsum/xxhsum.go |  |  | 0.633 |
| walker |  | 518 | 0 | go decl at xxhsum/xxhsum.go:11 |  |  | 0.633 |
| walker |  | 518 | 0 | go decl at xxhsum/xxhsum.go:34 |  |  | 0.633 |
| walker |  | 518 | 0 | go decl at xxhsum/xxhsum.go:43 |  |  | 0.633 |
| walker |  | 554 | 36 | go decl doc at xxhash_asm.go:12 |  |  | 0.636 |
| walker |  | 581 | 27 | go package + imports in xxhash_unsafe.go |  |  | 0.636 |
| ns | 603 |  | 140 | Digest struct + zero-value caveat | 2.1 |  | 0.552 |
| walker |  | 610 | 29 | go decl body at xxhash_unsafe.go:38 |  |  | 0.552 |
| walker |  | 654 | 44 | go decl doc at xxhash_unsafe.go:38 |  |  | 0.554 |
| walker |  | 695 | 41 | go decl doc at xxhash_unsafe.go:45 |  |  | 0.555 |
| ns | 712 |  | 109 | Constructors — New and NewWithSeed | 2.2 | 2.1 | 0.504 |
| walker |  | 828 | 133 | go module file xxhashbench/go.mod |  |  | 0.504 |
| walker |  | 906 | 78 | go decl names surface in dynamic/plugin.go |  |  | 0.504 |
| walker |  | 906 | 0 | go decl at dynamic/plugin.go:19 |  |  | 0.504 |
| walker |  | 906 | 0 | go decl at dynamic/plugin.go:26 |  |  | 0.504 |
| ns | 907 |  | 195 | Reset / ResetWithSeed — full bodies | 2.3 | 2.1 | 0.448 |
| walker |  | 915 | 9 | go decl at dynamic/plugin.go:14 |  |  | 0.448 |
| ns | 1077 |  | 170 | Remaining Digest methods — Size, BlockSize, Write, Sum, Sum64 signatures | 2.4 | 2.1 | 0.421 |
| walker |  | 1137 | 222 | README.md section #0 |  |  | 0.581 |
| walker |  | 1182 | 45 | go package + imports in xxhash.go |  |  | 0.586 |
| walker |  | 1190 | 8 | go package + imports in xxhash_other.go |  |  | 0.586 |
| ns | 1196 |  | 119 | Sum64 / writeBlocks signatures (asm build) | 2.5 |  | 0.588 |
| walker |  | 1232 | 42 | go decl doc at xxhash_unsafe.go:55 |  |  | 0.588 |
| walker |  | 1267 | 35 | go decl names surface in xxhash_other.go |  |  | 0.588 |
| walker |  | 1267 | 0 | go decl at xxhash_other.go:7 |  |  | 0.588 |
| walker |  | 1267 | 0 | go decl at xxhash_other.go:64 |  |  | 0.588 |
| ns | 1341 |  | 145 | Sum64String / WriteString signatures (unsafe build) | 2.6 |  | 0.596 |
| walker |  | 1390 | 123 | README.md section #1 |  |  | 0.597 |
| walker |  | 1445 | 55 | go package + imports in dynamic/plugin.go |  |  | 0.597 |
| ns | 1460 |  | 119 | MarshalBinary / UnmarshalBinary signatures + magic constants | 2.7 | 2.1 | 0.574 |
| walker |  | 1500 | 55 | go package + imports in xxhsum/xxhsum.go |  |  | 0.575 |
| ns | 1507 |  | 47 | Sub-directory listings (.github, dynamic, xxhsum, xxhashbench) | 3.1 | 1.1 | 0.582 |
| walker |  | 1522 | 22 | go decl doc at xxhash_other.go:7 |  |  | 0.582 |
| walker |  | 1604 | 82 | go decl body at xxhash_unsafe.go:45 |  |  | 0.583 |
| ns | 1648 |  | 141 | All test/benchmark function names across the repo | 3.2 |  | 0.547 |
| walker |  | 1658 | 54 | go decl body at dynamic/plugin.go:19 |  |  | 0.547 |
| walker |  | 1661 | 3 | listing of '.github' |  |  | 0.556 |
| walker |  | 1665 | 4 | listing of '.github/workflows' |  |  | 0.565 |
| ns | 1765 |  | 117 | testall.sh — full body | 3.3 |  | 0.545 |
| ns | 2081 |  | 316 | xxhsum CLI — main + usage | 3.4 |  | 0.495 |
| walker |  | 2377 | 712 | go decl names surface in xxhash.go |  |  | 0.525 |
| walker |  | 2377 | 0 | go decl at xxhash.go:23 |  |  | 0.525 |
| walker |  | 2377 | 0 | go decl at xxhash.go:40 |  |  | 0.525 |
| walker |  | 2377 | 0 | go decl at xxhash.go:45 |  |  | 0.525 |
| walker |  | 2377 | 0 | go decl at xxhash.go:53 |  |  | 0.525 |
| walker |  | 2377 | 0 | go decl at xxhash.go:59 |  |  | 0.525 |
| walker |  | 2377 | 0 | go decl at xxhash.go:69 |  |  | 0.525 |
| walker |  | 2377 | 0 | go decl at xxhash.go:72 |  |  | 0.525 |
| walker |  | 2377 | 0 | go decl at xxhash.go:75 |  |  | 0.525 |
| walker |  | 2377 | 0 | go decl at xxhash.go:113 |  |  | 0.525 |
| walker |  | 2377 | 0 | go decl at xxhash.go:129 |  |  | 0.525 |
| walker |  | 2377 | 0 | go decl at xxhash.go:176 |  |  | 0.525 |
| walker |  | 2377 | 0 | go decl at xxhash.go:190 |  |  | 0.525 |
| walker |  | 2377 | 0 | go decl at xxhash.go:208 |  |  | 0.525 |
| walker |  | 2377 | 0 | go decl at xxhash.go:214 |  |  | 0.525 |
| walker |  | 2377 | 0 | go decl at xxhash.go:222 |  |  | 0.525 |
| walker |  | 2377 | 0 | go decl at xxhash.go:229 |  |  | 0.525 |
| walker |  | 2386 | 9 | go decl body at xxhash.go:40 |  |  | 0.528 |
| walker |  | 2395 | 9 | go decl body at xxhash.go:53 |  |  | 0.529 |
| ns | 2400 |  | 319 | CI matrix — what configurations are tested | 3.5 |  | 0.486 |
| walker |  | 2404 | 9 | go decl at xxhash.go:11 |  |  | 0.487 |
| walker |  | 2413 | 9 | go decl at xxhash.go:170 |  |  | 0.489 |
| walker |  | 2425 | 12 | go decl doc at xxhash.go:69 |  |  | 0.493 |
| walker |  | 2437 | 12 | go decl doc at xxhash.go:129 |  |  | 0.497 |
| walker |  | 2450 | 13 | go decl doc at xxhash.go:72 |  |  | 0.503 |
| walker |  | 2465 | 15 | go decl doc at xxhash.go:40 |  |  | 0.507 |
| walker |  | 2482 | 17 | go decl doc at xxhash.go:45 |  |  | 0.515 |
| walker |  | 2498 | 16 | go decl doc at xxhash.go:176 |  |  | 0.523 |
| walker |  | 2516 | 18 | go decl doc at xxhash.go:190 |  |  | 0.533 |
| walker |  | 2535 | 19 | go decl doc at xxhash.go:113 |  |  | 0.541 |
| walker |  | 2555 | 20 | go decl doc at xxhash.go:75 |  |  | 0.550 |
| walker |  | 2579 | 24 | go decl body at xxhash.go:45 |  |  | 0.565 |
| ns | 2592 |  | 192 | Prime constants + primes array | 4.1 |  | 0.562 |
| walker |  | 2610 | 31 | go decl doc at xxhash.go:53 |  |  | 0.568 |
| walker |  | 2687 | 77 | go decl at xxhash.go:29 |  |  | 0.583 |
| ns | 2716 |  | 124 | round + mergeRound — the core mixer | 4.2 | 4.1 | 0.568 |
| walker |  | 2722 | 35 | go decl doc at xxhash.go:59 |  |  | 0.578 |
| walker |  | 2775 | 53 | go decl doc at xxhash.go:29 |  |  | 0.612 |
| walker |  | 2794 | 19 | go decl body at xxhash.go:214 |  |  | 0.613 |
| walker |  | 2864 | 70 | go decl body at xxhash.go:59 |  |  | 0.639 |
| walker |  | 2899 | 35 | go decl body at xxhash.go:222 |  |  | 0.649 |
| ns | 2918 |  | 202 | rol* one-line helpers (locations) | 4.3 |  | 0.657 |
| walker |  | 2936 | 37 | go decl body at xxhash.go:208 |  |  | 0.658 |
| walker |  | 2991 | 55 | go decl doc at xxhash.go:23 |  |  | 0.678 |
| walker |  | 3030 | 39 | go decl body at xxhash.go:229 |  |  | 0.693 |
| ns | 3084 |  | 166 | Little-endian / append / consume byte helpers | 4.4 |  | 0.694 |
| walker |  | 3136 | 106 | go decl body at xxhash.go:113 |  |  | 0.697 |
| walker |  | 3280 | 144 | README.md section #3 |  |  | 0.697 |
| walker |  | 3423 | 143 | go decl body at xxhash.go:176 |  |  | 0.699 |
| walker |  | 3472 | 49 | go decl body at xxhsum/xxhsum.go:34 |  |  | 0.699 |
| ns | 3473 |  | 389 | Write — streaming entry body | 4.5 | 2.4 | 0.649 |
| walker |  | 3765 | 293 | README.md section #2 |  |  | 0.651 |
| ns | 3949 |  | 476 | Digest.Sum64 — finalize body | 4.6 | 2.4 | 0.606 |
| walker |  | 3967 | 202 | go decl body at xxhash.go:190 |  |  | 0.608 |
| walker |  | 4040 | 73 | go decl body at xxhsum/xxhsum.go:43 |  |  | 0.608 |
| ns | 4062 |  | 113 | Digest.Sum — append big-endian bytes | 4.7 | 2.4 | 0.620 |
| ns | 4212 |  | 150 | MarshalBinary body — wire format | 4.8 | 2.7 | 0.629 |
| walker |  | 4392 | 352 | go decl body at xxhash.go:75 |  |  | 0.685 |
| ns | 4421 |  | 209 | UnmarshalBinary body — validation + parse | 4.9 | 2.7 | 0.693 |
| walker |  | 4836 | 444 | go decl body at xxhash.go:129 |  |  | 0.752 |
| walker |  | 5110 | 274 | go decl body at dynamic/plugin.go:26 |  |  | 0.752 |
| ns | 5119 |  | 698 | Pure-Go Sum64 body (xxhash_other.go) | 4.10 | 2.5 | 0.696 |
| walker |  | 5341 | 231 | go decl body at xxhsum/xxhsum.go:11 |  |  | 0.741 |
| walker |  | 5349 | 8 | plaintext config dynamic/.gitignore |  |  | 0.741 |
| ns | 5355 |  | 236 | Pure-Go writeBlocks body | 4.11 | 2.5 | 0.729 |
| walker |  | 5364 | 15 | go test names surface in xxhashbench/xxhashbench_test.go |  |  | 0.729 |
| walker |  | 5396 | 32 | go test names surface in xxhash_unsafe_test.go |  |  | 0.731 |
| walker |  | 5405 | 9 | plaintext config xxhsum/.gitignore |  |  | 0.731 |
| ns | 5535 |  | 180 | xxhash_safe.go full body — the appengine fallback | 4.12 | 2.6 | 0.723 |
| walker |  | 5617 | 212 | go decl body at xxhash_other.go:64 |  |  | 0.739 |
| walker |  | 5647 | 30 | go test names surface in dynamic/dynamic_test.go |  |  | 0.741 |
| walker |  | 5712 | 65 | go test names surface in bench_test.go |  |  | 0.749 |
| ns | 5940 |  | 405 | Unsafe string conversion — inlining commentary + sliceHeader | 4.13 | 2.6 | 0.727 |
| ns | 6070 |  | 130 | Unsafe Sum64String / WriteString bodies | 4.14 | 2.6 | 0.726 |
| walker |  | 6359 | 647 | go decl body at xxhash_other.go:7 |  |  | 0.787 |
| ns | 6391 |  | 321 | README — Benchmarks section (perf table) | 5.1 |  | 0.788 |
| walker |  | 6440 | 81 | go test names surface in xxhash_test.go |  |  | 0.800 |
| ns | 6724 |  | 333 | TestAll — known-answer test vectors | 5.2 | 3.2 | 0.785 |
| walker |  | 6732 | 292 | plaintext config LICENSE.txt |  |  | 0.785 |
| ns | 7016 |  | 292 | bench_test.go — input-size grid + benchmark shape | 5.3 | 3.2 | 0.760 |
| ns | 7396 |  | 380 | xxhashbench — structure of the cross-library benchmark | 5.4 | 3.2 | 0.735 |
| ns | 8093 |  | 697 | amd64 asm — register defs and macros | 5.5 |  | 0.699 |
| ns | 8652 |  | 559 | amd64 asm — Sum64 prologue, state setup, and main block loop | 5.6 | 5.5 | 0.673 |
| ns | 9358 |  | 706 | amd64 asm — tail handlers (loop8 / try4 / try1) + finalize | 5.7 | 5.5 | 0.640 |
| ns | 9801 |  | 443 | amd64 asm — writeBlocks body | 5.8 | 5.5 | 0.625 |
| ns | 9946 |  | 145 | README — Compatibility (supported Go versions) | 6.1 |  | 0.627 |

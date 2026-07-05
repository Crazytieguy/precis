Score(3000)=0.722 I=0.902 C=0.577 ns_rows≤3K=20/40 (reached=11 partial=5 missing=4)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 83 | 83 | listing of '.' |  |  | 1.000 |
| ns | 83 |  | 83 | Fixture top-level listing | 1.1 |  | 1.000 |
| walker |  | 149 | 66 | README headline in README.md |  |  | 1.000 |
| ns | 149 |  | 66 | README — title and one-sentence positioning | 1.2 |  | 1.000 |
| walker |  | 180 | 31 | headings outline in README.md |  |  | 1.000 |
| walker |  | 211 | 31 | go module identity in go.mod |  |  | 1.000 |
| walker |  | 211 | 0 | go module file go.mod |  |  | 1.000 |
| ns | 227 |  | 78 | Package doc + module path | 1.3 |  | 0.883 |
| walker |  | 250 | 39 | go package doc lede in xxhash.go |  |  | 0.969 |
| walker |  | 261 | 11 | listing of 'xxhsum' |  |  | 0.969 |
| walker |  | 275 | 14 | listing of 'dynamic' |  |  | 0.971 |
| walker |  | 290 | 15 | listing of 'xxhashbench' |  |  | 0.975 |
| walker |  | 325 | 35 | go module identity in xxhashbench/go.mod |  |  | 0.975 |
| walker |  | 360 | 35 | go decl names surface in xxhash_asm.go |  |  | 0.975 |
| walker |  | 360 | 0 | go decl at xxhash_asm.go:12 |  |  | 0.975 |
| walker |  | 360 | 0 | go decl at xxhash_asm.go:15 |  |  | 0.975 |
| walker |  | 370 | 10 | go package + imports in xxhash_asm.go |  |  | 0.975 |
| ns | 393 |  | 166 | README — public API code-fence sketch | 1.4 |  | 0.744 |
| walker |  | 413 | 43 | go decl names surface in xxhash_safe.go |  |  | 0.744 |
| walker |  | 413 | 0 | go decl at xxhash_safe.go:9 |  |  | 0.744 |
| walker |  | 413 | 0 | go decl at xxhash_safe.go:14 |  |  | 0.744 |
| walker |  | 424 | 11 | go decl body at xxhash_safe.go:9 |  |  | 0.744 |
| walker |  | 434 | 10 | go package + imports in xxhash_safe.go |  |  | 0.744 |
| walker |  | 445 | 11 | go decl body at xxhash_safe.go:14 |  |  | 0.744 |
| walker |  | 452 | 7 | go decl doc at xxhash_asm.go:15 |  |  | 0.744 |
| ns | 469 |  | 76 | README — purego/asm note | 1.5 |  | 0.698 |
| walker |  | 506 | 54 | go decl names surface in xxhash_unsafe.go |  |  | 0.698 |
| walker |  | 506 | 0 | go decl at xxhash_unsafe.go:38 |  |  | 0.698 |
| walker |  | 506 | 0 | go decl at xxhash_unsafe.go:45 |  |  | 0.698 |
| walker |  | 525 | 19 | go decl at xxhash_unsafe.go:55 |  |  | 0.698 |
| walker |  | 546 | 21 | go decl doc at xxhash_safe.go:9 |  |  | 0.699 |
| walker |  | 567 | 21 | go decl doc at xxhash_safe.go:14 |  |  | 0.699 |
| ns | 611 |  | 142 | Digest struct + zero-value caveat | 2.1 |  | 0.607 |
| walker |  | 617 | 50 | go package + imports in xxhash.go |  |  | 0.627 |
| walker |  | 651 | 34 | go decl doc at xxhash_asm.go:12 |  |  | 0.629 |
| walker |  | 697 | 46 | go decl names surface in xxhsum/xxhsum.go |  |  | 0.629 |
| walker |  | 697 | 0 | go decl at xxhsum/xxhsum.go:11 |  |  | 0.629 |
| walker |  | 697 | 0 | go decl at xxhsum/xxhsum.go:34 |  |  | 0.629 |
| walker |  | 697 | 0 | go decl at xxhsum/xxhsum.go:43 |  |  | 0.629 |
| ns | 720 |  | 109 | Constructors — New and NewWithSeed | 2.2 | 2.1 | 0.571 |
| walker |  | 741 | 44 | go decl doc at xxhash_unsafe.go:38 |  |  | 0.572 |
| walker |  | 782 | 41 | go decl doc at xxhash_unsafe.go:45 |  |  | 0.574 |
| walker |  | 816 | 34 | go package + imports in xxhash_unsafe.go |  |  | 0.574 |
| walker |  | 847 | 31 | go decl body at xxhash_unsafe.go:38 |  |  | 0.574 |
| ns | 915 |  | 195 | Reset / ResetWithSeed — full bodies | 2.3 | 2.1 | 0.511 |
| walker |  | 960 | 113 | go module file xxhashbench/go.mod |  |  | 0.511 |
| walker |  | 1040 | 80 | go decl names surface in dynamic/plugin.go |  |  | 0.511 |
| walker |  | 1040 | 0 | go decl at dynamic/plugin.go:19 |  |  | 0.511 |
| walker |  | 1040 | 0 | go decl at dynamic/plugin.go:26 |  |  | 0.511 |
| walker |  | 1049 | 9 | go decl at dynamic/plugin.go:14 |  |  | 0.511 |
| ns | 1089 |  | 174 | Remaining Digest methods — Size, BlockSize, Write, Sum, Sum64 signatures | 2.4 | 2.1 | 0.480 |
| ns | 1210 |  | 121 | Sum64 / writeBlocks signatures (asm build) | 2.5 |  | 0.491 |
| walker |  | 1294 | 245 | README.md section #0 |  |  | 0.665 |
| walker |  | 1336 | 42 | go decl doc at xxhash_unsafe.go:55 |  |  | 0.666 |
| ns | 1361 |  | 151 | Sum64String / WriteString signatures (unsafe build) | 2.6 |  | 0.669 |
| walker |  | 1373 | 37 | go decl names surface in xxhash_other.go |  |  | 0.669 |
| walker |  | 1373 | 0 | go decl at xxhash_other.go:7 |  |  | 0.669 |
| walker |  | 1373 | 0 | go decl at xxhash_other.go:64 |  |  | 0.669 |
| ns | 1484 |  | 123 | MarshalBinary / UnmarshalBinary signatures + magic constants | 2.7 | 2.1 | 0.643 |
| ns | 1531 |  | 47 | Sub-directory listings (.github, dynamic, xxhsum, xxhashbench) | 3.1 | 1.1 | 0.646 |
| ns | 1716 |  | 185 | All test/benchmark function names across the repo | 3.2 |  | 0.606 |
| ns | 1833 |  | 117 | testall.sh — full body | 3.3 |  | 0.583 |
| walker |  | 2088 | 715 | go decl names surface in xxhash.go |  |  | 0.618 |
| walker |  | 2088 | 0 | go decl at xxhash.go:23 |  |  | 0.618 |
| walker |  | 2088 | 0 | go decl at xxhash.go:40 |  |  | 0.618 |
| walker |  | 2088 | 0 | go decl at xxhash.go:45 |  |  | 0.618 |
| walker |  | 2088 | 0 | go decl at xxhash.go:53 |  |  | 0.618 |
| walker |  | 2088 | 0 | go decl at xxhash.go:59 |  |  | 0.618 |
| walker |  | 2088 | 0 | go decl at xxhash.go:69 |  |  | 0.618 |
| walker |  | 2088 | 0 | go decl at xxhash.go:72 |  |  | 0.618 |
| walker |  | 2088 | 0 | go decl at xxhash.go:75 |  |  | 0.618 |
| walker |  | 2088 | 0 | go decl at xxhash.go:113 |  |  | 0.618 |
| walker |  | 2088 | 0 | go decl at xxhash.go:129 |  |  | 0.618 |
| walker |  | 2088 | 0 | go decl at xxhash.go:176 |  |  | 0.618 |
| walker |  | 2088 | 0 | go decl at xxhash.go:190 |  |  | 0.618 |
| walker |  | 2088 | 0 | go decl at xxhash.go:208 |  |  | 0.618 |
| walker |  | 2088 | 0 | go decl at xxhash.go:214 |  |  | 0.618 |
| walker |  | 2088 | 0 | go decl at xxhash.go:222 |  |  | 0.618 |
| walker |  | 2088 | 0 | go decl at xxhash.go:229 |  |  | 0.618 |
| walker |  | 2099 | 11 | go decl doc at xxhash.go:72 |  |  | 0.622 |
| walker |  | 2110 | 11 | go decl body at xxhash.go:40 |  |  | 0.625 |
| walker |  | 2119 | 9 | go decl at xxhash.go:11 |  |  | 0.625 |
| walker |  | 2131 | 12 | go decl doc at xxhash.go:69 |  |  | 0.631 |
| walker |  | 2143 | 12 | go decl doc at xxhash.go:129 |  |  | 0.637 |
| ns | 2151 |  | 318 | xxhsum CLI — main + usage | 3.4 |  | 0.573 |
| walker |  | 2154 | 11 | go decl body at xxhash.go:53 |  |  | 0.574 |
| walker |  | 2169 | 15 | go decl doc at xxhash.go:40 |  |  | 0.579 |
| walker |  | 2180 | 11 | go decl at xxhash.go:170 |  |  | 0.582 |
| walker |  | 2194 | 14 | go decl doc at xxhash.go:176 |  |  | 0.591 |
| walker |  | 2211 | 17 | go decl doc at xxhash.go:45 |  |  | 0.599 |
| walker |  | 2229 | 18 | go decl doc at xxhash.go:75 |  |  | 0.607 |
| walker |  | 2247 | 18 | go decl doc at xxhash.go:190 |  |  | 0.618 |
| walker |  | 2266 | 19 | go decl doc at xxhash.go:113 |  |  | 0.628 |
| walker |  | 2297 | 31 | go decl doc at xxhash.go:53 |  |  | 0.635 |
| walker |  | 2323 | 26 | go decl body at xxhash.go:45 |  |  | 0.652 |
| walker |  | 2400 | 77 | go decl at xxhash.go:29 |  |  | 0.669 |
| walker |  | 2435 | 35 | go decl doc at xxhash.go:59 |  |  | 0.682 |
| ns | 2472 |  | 321 | CI matrix — what configurations are tested | 3.5 |  | 0.627 |
| walker |  | 2486 | 51 | go decl doc at xxhash.go:29 |  |  | 0.664 |
| walker |  | 2507 | 21 | go decl body at xxhash.go:214 |  |  | 0.664 |
| walker |  | 2579 | 72 | go decl body at xxhash.go:59 |  |  | 0.693 |
| walker |  | 2632 | 53 | go decl doc at xxhash.go:23 |  |  | 0.695 |
| ns | 2664 |  | 192 | Prime constants + primes array | 4.1 |  | 0.705 |
| walker |  | 2669 | 37 | go decl body at xxhash.go:222 |  |  | 0.706 |
| walker |  | 2708 | 39 | go decl body at xxhash.go:208 |  |  | 0.707 |
| walker |  | 2749 | 41 | go decl body at xxhash.go:229 |  |  | 0.709 |
| walker |  | 2759 | 10 | go package + imports in xxhash_other.go |  |  | 0.709 |
| ns | 2790 |  | 126 | round + mergeRound — the core mixer | 4.2 | 4.1 | 0.712 |
| walker |  | 2867 | 108 | go decl body at xxhash.go:113 |  |  | 0.715 |
| walker |  | 2887 | 20 | go decl doc at xxhash_other.go:7 |  |  | 0.715 |
| ns | 2990 |  | 200 | rol* one-line helpers (locations) | 4.3 |  | 0.722 |
| walker |  | 3032 | 145 | go decl body at xxhash.go:176 |  |  | 0.724 |
| ns | 3156 |  | 166 | Little-endian / append / consume byte helpers | 4.4 |  | 0.724 |
| walker |  | 3168 | 136 | README.md section #1 |  |  | 0.725 |
| walker |  | 3231 | 63 | go package + imports in xxhsum/xxhsum.go |  |  | 0.729 |
| walker |  | 3296 | 65 | go package + imports in dynamic/plugin.go |  |  | 0.729 |
| walker |  | 3347 | 51 | go decl body at xxhsum/xxhsum.go:34 |  |  | 0.729 |
| walker |  | 3431 | 84 | go decl body at xxhash_unsafe.go:45 |  |  | 0.730 |
| walker |  | 3487 | 56 | go decl body at dynamic/plugin.go:19 |  |  | 0.730 |
| walker |  | 3490 | 3 | listing of '.github' |  |  | 0.735 |
| walker |  | 3494 | 4 | listing of '.github/workflows' |  |  | 0.740 |
| ns | 3543 |  | 387 | Write — streaming entry body | 4.5 | 2.4 | 0.687 |
| walker |  | 3698 | 204 | go decl body at xxhash.go:190 |  |  | 0.690 |
| walker |  | 3845 | 147 | README.md section #3 |  |  | 0.690 |
| walker |  | 3920 | 75 | go decl body at xxhsum/xxhsum.go:43 |  |  | 0.690 |
| ns | 4017 |  | 474 | Digest.Sum64 — finalize body | 4.6 | 2.4 | 0.642 |
| ns | 4128 |  | 111 | Digest.Sum — append big-endian bytes | 4.7 | 2.4 | 0.653 |
| ns | 4276 |  | 148 | MarshalBinary body — wire format | 4.8 | 2.7 | 0.661 |
| walker |  | 4304 | 384 | go decl body at xxhash.go:75 |  |  | 0.729 |
| ns | 4483 |  | 207 | UnmarshalBinary body — validation + parse | 4.9 | 2.7 | 0.736 |
| walker |  | 4615 | 311 | README.md section #2 |  |  | 0.738 |
| walker |  | 5086 | 471 | go decl body at xxhash.go:129 |  |  | 0.805 |
| ns | 5185 |  | 702 | Pure-Go Sum64 body (xxhash_other.go) | 4.10 | 2.5 | 0.746 |
| walker |  | 5319 | 233 | go decl body at xxhsum/xxhsum.go:11 |  |  | 0.793 |
| ns | 5419 |  | 234 | Pure-Go writeBlocks body | 4.11 | 2.5 | 0.780 |
| walker |  | 5595 | 276 | go decl body at dynamic/plugin.go:26 |  |  | 0.780 |
| ns | 5599 |  | 180 | xxhash_safe.go full body — the appengine fallback | 4.12 | 2.6 | 0.771 |
| walker |  | 5603 | 8 | plaintext config dynamic/.gitignore |  |  | 0.771 |
| walker |  | 5637 | 34 | go test names surface in xxhash_unsafe_test.go |  |  | 0.772 |
| walker |  | 5646 | 9 | plaintext config xxhsum/.gitignore |  |  | 0.772 |
| walker |  | 5663 | 17 | go test names surface in xxhashbench/xxhashbench_test.go |  |  | 0.773 |
| walker |  | 5877 | 214 | go decl body at xxhash_other.go:64 |  |  | 0.789 |
| walker |  | 5944 | 67 | go test names surface in bench_test.go |  |  | 0.796 |
| walker |  | 5976 | 32 | go test names surface in dynamic/dynamic_test.go |  |  | 0.799 |
| ns | 6004 |  | 405 | Unsafe string conversion — inlining commentary + sliceHeader | 4.13 | 2.6 | 0.775 |
| ns | 6130 |  | 126 | Unsafe Sum64String / WriteString bodies | 4.14 | 2.6 | 0.774 |
| ns | 6453 |  | 323 | README — Benchmarks section (perf table) | 5.1 |  | 0.779 |
| walker |  | 6655 | 679 | go decl body at xxhash_other.go:7 |  |  | 0.845 |
| walker |  | 6738 | 83 | go test names surface in xxhash_test.go |  |  | 0.857 |
| ns | 6786 |  | 333 | TestAll — known-answer test vectors | 5.2 | 3.2 | 0.841 |
| walker |  | 7050 | 312 | plaintext config LICENSE.txt |  |  | 0.841 |
| ns | 7074 |  | 288 | bench_test.go — input-size grid + benchmark shape | 5.3 | 3.2 | 0.814 |
| ns | 7456 |  | 382 | xxhashbench — structure of the cross-library benchmark | 5.4 | 3.2 | 0.788 |
| ns | 8155 |  | 699 | amd64 asm — register defs and macros | 5.5 |  | 0.749 |
| ns | 8714 |  | 559 | amd64 asm — Sum64 prologue, state setup, and main block loop | 5.6 | 5.5 | 0.721 |
| ns | 9420 |  | 706 | amd64 asm — tail handlers (loop8 / try4 / try1) + finalize | 5.7 | 5.5 | 0.685 |
| ns | 9861 |  | 441 | amd64 asm — writeBlocks body | 5.8 | 5.5 | 0.669 |
| ns | 10004 |  | 143 | README — Compatibility (supported Go versions) | 6.1 |  | 0.673 |

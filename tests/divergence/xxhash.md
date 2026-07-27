Score(3000)=0.737 I=0.897 C=0.605 ns_rows≤3K=18/48 (reached=9 partial=3 missing=6)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 87 | 87 | listing of '.' |  |  | 1.000 |
| ns | 87 |  | 87 | Root directory listing | 1.1 |  | 1.000 |
| walker |  | 97 | 10 | listing of 'xxhsum' |  |  | 1.000 |
| walker |  | 110 | 13 | listing of 'dynamic' |  |  | 1.000 |
| walker |  | 124 | 14 | listing of 'xxhashbench' |  |  | 1.000 |
| walker |  | 127 | 3 | listing of '.github' |  |  | 1.000 |
| ns | 128 |  | 41 | Subdirectory listings | 1.2 |  | 0.955 |
| walker |  | 130 | 3 | listing of '.github/workflows' |  |  | 1.000 |
| ns | 159 |  | 31 | go.mod (module identity) | 1.3 |  | 0.938 |
| walker |  | 161 | 31 | go module identity in go.mod |  |  | 1.000 |
| walker |  | 161 | 0 | go module file go.mod |  |  | 1.000 |
| walker |  | 200 | 39 | go package doc lede in xxhash.go |  |  | 1.000 |
| ns | 206 |  | 47 | Package doc comment | 1.4 |  | 0.977 |
| walker |  | 266 | 66 | README headline in README.md |  |  | 0.982 |
| walker |  | 297 | 31 | headings outline in README.md |  |  | 0.982 |
| ns | 310 |  | 104 | README heading + badges | 1.5 |  | 0.921 |
| walker |  | 332 | 35 | go decl names surface in xxhash_asm.go |  |  | 0.922 |
| walker |  | 332 | 0 | go decl at xxhash_asm.go:12 |  |  | 0.922 |
| walker |  | 332 | 0 | go decl at xxhash_asm.go:15 |  |  | 0.922 |
| walker |  | 367 | 35 | go module identity in xxhashbench/go.mod |  |  | 0.922 |
| ns | 369 |  | 59 | README pitch | 1.6 |  | 0.906 |
| ns | 535 |  | 166 | README API surface snippet | 1.7 |  | 0.744 |
| ns | 616 |  | 81 | README build-tag mention + xxHash link | 1.8 |  | 0.701 |
| ns | 761 |  | 145 | README compatibility section | 1.9 |  | 0.638 |
| ns | 937 |  | 176 | Digest type + exported method signature roster | 1.10 |  | 0.583 |
| walker |  | 991 | 624 | YAML config at .github/workflows/test.yml |  |  | 0.617 |
| walker |  | 1034 | 43 | go decl names surface in xxhash_safe.go |  |  | 0.617 |
| walker |  | 1034 | 0 | go decl at xxhash_safe.go:9 |  |  | 0.617 |
| walker |  | 1034 | 0 | go decl at xxhash_safe.go:14 |  |  | 0.617 |
| walker |  | 1045 | 11 | go decl body at xxhash_safe.go:9 |  |  | 0.617 |
| walker |  | 1055 | 10 | go package + imports in xxhash_asm.go |  |  | 0.617 |
| walker |  | 1065 | 10 | go package + imports in xxhash_safe.go |  |  | 0.617 |
| walker |  | 1076 | 11 | go decl body at xxhash_safe.go:14 |  |  | 0.617 |
| walker |  | 1083 | 7 | go decl doc at xxhash_asm.go:15 |  |  | 0.618 |
| walker |  | 1137 | 54 | go decl names surface in xxhash_unsafe.go |  |  | 0.618 |
| walker |  | 1137 | 0 | go decl at xxhash_unsafe.go:38 |  |  | 0.618 |
| walker |  | 1137 | 0 | go decl at xxhash_unsafe.go:45 |  |  | 0.618 |
| walker |  | 1156 | 19 | go decl at xxhash_unsafe.go:55 |  |  | 0.618 |
| walker |  | 1177 | 21 | go decl doc at xxhash_safe.go:9 |  |  | 0.618 |
| walker |  | 1198 | 21 | go decl doc at xxhash_safe.go:14 |  |  | 0.619 |
| walker |  | 1232 | 34 | go decl doc at xxhash_asm.go:12 |  |  | 0.621 |
| ns | 1253 |  | 316 | CI workflow: test job | 2.1 |  | 0.703 |
| walker |  | 1278 | 46 | go decl names surface in xxhsum/xxhsum.go |  |  | 0.703 |
| walker |  | 1278 | 0 | go decl at xxhsum/xxhsum.go:11 |  |  | 0.703 |
| walker |  | 1278 | 0 | go decl at xxhsum/xxhsum.go:34 |  |  | 0.703 |
| walker |  | 1278 | 0 | go decl at xxhsum/xxhsum.go:43 |  |  | 0.703 |
| walker |  | 1395 | 117 | plaintext config testall.sh |  |  | 0.708 |
| walker |  | 1439 | 44 | go decl doc at xxhash_unsafe.go:38 |  |  | 0.709 |
| walker |  | 1480 | 41 | go decl doc at xxhash_unsafe.go:45 |  |  | 0.709 |
| walker |  | 1514 | 34 | go package + imports in xxhash_unsafe.go |  |  | 0.710 |
| ns | 1556 |  | 303 | CI workflow: qemu cross-arch job | 2.2 |  | 0.745 |
| walker |  | 1627 | 113 | go module file xxhashbench/go.mod |  |  | 0.745 |
| ns | 1673 |  | 117 | testall.sh | 2.3 |  | 0.754 |
| walker |  | 1707 | 80 | go decl names surface in dynamic/plugin.go |  |  | 0.755 |
| walker |  | 1707 | 0 | go decl at dynamic/plugin.go:19 |  |  | 0.755 |
| walker |  | 1707 | 0 | go decl at dynamic/plugin.go:26 |  |  | 0.755 |
| walker |  | 1716 | 9 | go decl at dynamic/plugin.go:14 |  |  | 0.755 |
| ns | 1840 |  | 167 | Build-tag map across the four platform-variant files | 3.1 |  | 0.724 |
| ns | 1937 |  | 97 | xxhash_asm.go body | 3.2 |  | 0.726 |
| walker |  | 1961 | 245 | README.md section #0 |  |  | 0.830 |
| walker |  | 1992 | 31 | go decl body at xxhash_unsafe.go:38 |  |  | 0.831 |
| walker |  | 2034 | 42 | go decl doc at xxhash_unsafe.go:55 |  |  | 0.832 |
| walker |  | 2071 | 37 | go decl names surface in xxhash_other.go |  |  | 0.832 |
| walker |  | 2071 | 0 | go decl at xxhash_other.go:7 |  |  | 0.832 |
| walker |  | 2071 | 0 | go decl at xxhash_other.go:64 |  |  | 0.832 |
| ns | 2186 |  | 249 | xxhash_other.go: portable-Go Sum64 (distinctive parts) | 3.3 |  | 0.766 |
| ns | 2422 |  | 236 | xxhash_other.go: portable-Go writeBlocks | 3.4 |  | 0.739 |
| walker |  | 2786 | 715 | go decl names surface in xxhash.go |  |  | 0.787 |
| walker |  | 2786 | 0 | go decl at xxhash.go:23 |  |  | 0.787 |
| walker |  | 2786 | 0 | go decl at xxhash.go:40 |  |  | 0.787 |
| walker |  | 2786 | 0 | go decl at xxhash.go:45 |  |  | 0.787 |
| walker |  | 2786 | 0 | go decl at xxhash.go:53 |  |  | 0.787 |
| walker |  | 2786 | 0 | go decl at xxhash.go:59 |  |  | 0.787 |
| walker |  | 2786 | 0 | go decl at xxhash.go:69 |  |  | 0.787 |
| walker |  | 2786 | 0 | go decl at xxhash.go:72 |  |  | 0.787 |
| walker |  | 2786 | 0 | go decl at xxhash.go:75 |  |  | 0.787 |
| walker |  | 2786 | 0 | go decl at xxhash.go:113 |  |  | 0.787 |
| walker |  | 2786 | 0 | go decl at xxhash.go:129 |  |  | 0.787 |
| walker |  | 2786 | 0 | go decl at xxhash.go:176 |  |  | 0.787 |
| walker |  | 2786 | 0 | go decl at xxhash.go:190 |  |  | 0.787 |
| walker |  | 2786 | 0 | go decl at xxhash.go:208 |  |  | 0.787 |
| walker |  | 2786 | 0 | go decl at xxhash.go:214 |  |  | 0.787 |
| walker |  | 2786 | 0 | go decl at xxhash.go:222 |  |  | 0.787 |
| walker |  | 2786 | 0 | go decl at xxhash.go:229 |  |  | 0.787 |
| walker |  | 2797 | 11 | go decl doc at xxhash.go:72 |  |  | 0.787 |
| walker |  | 2808 | 11 | go decl body at xxhash.go:40 |  |  | 0.787 |
| walker |  | 2820 | 12 | go decl doc at xxhash.go:69 |  |  | 0.787 |
| walker |  | 2832 | 12 | go decl doc at xxhash.go:129 |  |  | 0.787 |
| walker |  | 2843 | 11 | go decl body at xxhash.go:53 |  |  | 0.788 |
| walker |  | 2854 | 11 | go decl at xxhash.go:11 |  |  | 0.788 |
| walker |  | 2865 | 11 | go decl at xxhash.go:170 |  |  | 0.788 |
| walker |  | 2880 | 15 | go decl doc at xxhash.go:40 |  |  | 0.788 |
| walker |  | 2894 | 14 | go decl doc at xxhash.go:176 |  |  | 0.788 |
| walker |  | 2911 | 17 | go decl doc at xxhash.go:45 |  |  | 0.788 |
| ns | 2923 |  | 501 | xxhash_unsafe.go body (default !appengine path) | 3.5 |  | 0.736 |
| walker |  | 2929 | 18 | go decl doc at xxhash.go:75 |  |  | 0.736 |
| walker |  | 2947 | 18 | go decl doc at xxhash.go:190 |  |  | 0.736 |
| walker |  | 2966 | 19 | go decl doc at xxhash.go:113 |  |  | 0.736 |
| walker |  | 2997 | 31 | go decl doc at xxhash.go:53 |  |  | 0.737 |
| walker |  | 3074 | 77 | go decl at xxhash.go:29 |  |  | 0.738 |
| ns | 3080 |  | 157 | xxhash_safe.go body (appengine path) | 3.6 |  | 0.728 |
| walker |  | 3109 | 35 | go decl doc at xxhash.go:59 |  |  | 0.728 |
| walker |  | 3135 | 26 | go decl body at xxhash.go:45 |  |  | 0.729 |
| walker |  | 3186 | 51 | go decl doc at xxhash.go:29 |  |  | 0.731 |
| walker |  | 3234 | 48 | go package + imports in xxhash.go |  |  | 0.736 |
| walker |  | 3255 | 21 | go decl body at xxhash.go:214 |  |  | 0.736 |
| ns | 3274 |  | 194 | Prime constants | 4.1 |  | 0.727 |
| walker |  | 3308 | 53 | go decl doc at xxhash.go:23 |  |  | 0.745 |
| walker |  | 3380 | 72 | go decl body at xxhash.go:59 |  |  | 0.746 |
| ns | 3404 |  | 130 | Digest struct fields | 4.2 | 1.10 | 0.754 |
| walker |  | 3417 | 37 | go decl body at xxhash.go:222 |  |  | 0.755 |
| walker |  | 3456 | 39 | go decl body at xxhash.go:208 |  |  | 0.755 |
| walker |  | 3497 | 41 | go decl body at xxhash.go:229 |  |  | 0.755 |
| walker |  | 3507 | 10 | go package + imports in xxhash_other.go |  |  | 0.756 |
| walker |  | 3615 | 108 | go decl body at xxhash.go:113 |  |  | 0.758 |
| ns | 3662 |  | 258 | New / NewWithSeed / Reset / ResetWithSeed bodies | 4.3 | 1.10 | 0.758 |
| ns | 3688 |  | 26 | Size / BlockSize doc comments | 4.4 | 1.10 | 0.761 |
| walker |  | 3751 | 136 | README.md section #1 |  |  | 0.791 |
| walker |  | 3771 | 20 | go decl doc at xxhash_other.go:7 |  |  | 0.792 |
| walker |  | 3916 | 145 | go decl body at xxhash.go:176 |  |  | 0.793 |
| walker |  | 4000 | 84 | go decl body at xxhash_unsafe.go:45 |  |  | 0.807 |
| walker |  | 4063 | 63 | go package + imports in xxhsum/xxhsum.go |  |  | 0.807 |
| ns | 4106 |  | 418 | Write method body | 4.5 | 1.10 | 0.759 |
| walker |  | 4128 | 65 | go package + imports in dynamic/plugin.go |  |  | 0.760 |
| walker |  | 4179 | 51 | go decl body at xxhsum/xxhsum.go:34 |  |  | 0.760 |
| ns | 4242 |  | 136 | Sum method body | 4.6 | 1.10 | 0.765 |
| walker |  | 4383 | 204 | go decl body at xxhash.go:190 |  |  | 0.768 |
| walker |  | 4439 | 56 | go decl body at dynamic/plugin.go:19 |  |  | 0.768 |
| walker |  | 4586 | 147 | README.md section #3 |  |  | 0.768 |
| walker |  | 4661 | 75 | go decl body at xxhsum/xxhsum.go:43 |  |  | 0.769 |
| ns | 4731 |  | 489 | Sum64 method body (finalize) | 4.7 | 1.10 | 0.724 |
| walker |  | 5045 | 384 | go decl body at xxhash.go:75 |  |  | 0.782 |
| ns | 5188 |  | 457 | MarshalBinary / UnmarshalBinary + magic const | 4.8 | 1.10 | 0.788 |
| ns | 5258 |  | 70 | Marshal helpers + u64/u32 readers roster | 4.9 |  | 0.789 |
| walker |  | 5356 | 311 | README.md section #2 |  |  | 0.790 |
| ns | 5587 |  | 329 | round / mergeRound / rolN mixing primitives | 4.10 |  | 0.790 |
| ns | 5676 |  | 89 | xxhash_test.go function roster | 5.1 |  | 0.783 |
| walker |  | 5827 | 471 | go decl body at xxhash.go:129 |  |  | 0.835 |
| walker |  | 6060 | 233 | go decl body at xxhsum/xxhsum.go:11 |  |  | 0.837 |
| walker |  | 6336 | 276 | go decl body at dynamic/plugin.go:26 |  |  | 0.837 |
| ns | 6342 |  | 666 | TestAll: the canonical XXH64 test-vector table | 5.2 | 5.1 | 0.796 |
| walker |  | 6370 | 34 | go test names surface in xxhash_unsafe_test.go |  |  | 0.796 |
| walker |  | 6387 | 17 | go test names surface in xxhashbench/xxhashbench_test.go |  |  | 0.796 |
| walker |  | 6395 | 8 | plaintext config dynamic/.gitignore |  |  | 0.796 |
| walker |  | 6404 | 9 | plaintext config xxhsum/.gitignore |  |  | 0.796 |
| walker |  | 6618 | 214 | go decl body at xxhash_other.go:64 |  |  | 0.812 |
| walker |  | 6685 | 67 | go test names surface in bench_test.go |  |  | 0.813 |
| walker |  | 6717 | 32 | go test names surface in dynamic/dynamic_test.go |  |  | 0.813 |
| walker |  | 6800 | 83 | go test names surface in xxhash_test.go |  |  | 0.817 |
| ns | 6898 |  | 556 | testDigest / testSum bodies | 5.3 | 5.1 | 0.785 |
| ns | 7073 |  | 175 | TestReset | 5.4 | 5.1 | 0.773 |
| walker |  | 7479 | 679 | go decl body at xxhash_other.go:7 |  |  | 0.803 |
| ns | 7487 |  | 414 | TestBinaryMarshaling | 5.5 | 5.1 | 0.777 |
| walker |  | 7791 | 312 | plaintext config LICENSE.txt |  |  | 0.777 |
| ns | 7843 |  | 356 | TestAllocs + testAllocs helper | 5.6 | 5.1 | 0.756 |
| ns | 8295 |  | 452 | xxhash_unsafe_test.go: header + TestInlining | 5.7 |  | 0.728 |
| ns | 8342 |  | 47 | bench_test.go benchmark roster | 5.8 |  | 0.730 |
| ns | 8589 |  | 247 | BenchmarkSum64 body (representative) + benchmarks table | 5.9 | 5.8 | 0.715 |
| ns | 8932 |  | 343 | xxhsum/xxhsum.go: main + printHash | 6.1 |  | 0.723 |
| ns | 8949 |  | 17 | Build-artifact .gitignore entries | 6.2 |  | 0.723 |
| ns | 9120 |  | 171 | dynamic/plugin.go: header + test-function roster | 6.3 |  | 0.724 |
| ns | 9422 |  | 302 | dynamic/dynamic_test.go: header + TestMain + TestDynamic roster | 6.4 |  | 0.707 |
| ns | 9515 |  | 93 | xxhashbench_test.go: TODO note + function roster | 6.5 |  | 0.705 |
| ns | 9590 |  | 75 | xxhashbench_test.go: comparison-target names | 6.6 |  | 0.702 |
| ns | 9754 |  | 164 | README benchmarks table | 7.1 |  | 0.704 |
| ns | 9782 |  | 28 | LICENSE.txt (identity only) | 7.2 |  | 0.705 |
| ns | 9866 |  | 84 | xxhash_amd64.s: macro + function entry-point locations | 8.1 |  | 0.702 |
| ns | 9950 |  | 84 | xxhash_arm64.s: macro + function entry-point locations | 8.2 |  | 0.700 |

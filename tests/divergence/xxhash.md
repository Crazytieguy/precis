Score(3000)=0.739 I=0.897 C=0.608 ns_rows≤3K=18/48 (reached=9 partial=3 missing=6)

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
| walker |  | 332 | 35 | go module identity in xxhashbench/go.mod |  |  | 0.921 |
| ns | 369 |  | 59 | README pitch | 1.6 |  | 0.905 |
| ns | 535 |  | 166 | README API surface snippet | 1.7 |  | 0.743 |
| ns | 616 |  | 81 | README build-tag mention + xxHash link | 1.8 |  | 0.700 |
| ns | 761 |  | 145 | README compatibility section | 1.9 |  | 0.637 |
| ns | 937 |  | 176 | Digest type + exported method signature roster | 1.10 |  | 0.583 |
| walker |  | 956 | 624 | YAML config at .github/workflows/test.yml |  |  | 0.616 |
| walker |  | 1010 | 54 | go decl names surface in xxhash_safe.go |  |  | 0.616 |
| walker |  | 1010 | 0 | go decl at xxhash_safe.go:9 |  |  | 0.616 |
| walker |  | 1010 | 0 | go decl at xxhash_safe.go:14 |  |  | 0.616 |
| walker |  | 1021 | 11 | go decl body at xxhash_safe.go:9 |  |  | 0.616 |
| walker |  | 1031 | 10 | go package + imports in xxhash_safe.go |  |  | 0.617 |
| walker |  | 1042 | 11 | go decl body at xxhash_safe.go:14 |  |  | 0.617 |
| walker |  | 1103 | 61 | go decl names surface in xxhash_asm.go |  |  | 0.617 |
| walker |  | 1103 | 0 | go decl at xxhash_asm.go:12 |  |  | 0.617 |
| walker |  | 1103 | 0 | go decl at xxhash_asm.go:15 |  |  | 0.617 |
| walker |  | 1113 | 10 | go package + imports in xxhash_asm.go |  |  | 0.618 |
| walker |  | 1120 | 7 | go decl doc at xxhash_asm.go:15 |  |  | 0.618 |
| walker |  | 1186 | 66 | go decl names surface in xxhash_unsafe.go |  |  | 0.618 |
| walker |  | 1186 | 0 | go decl at xxhash_unsafe.go:38 |  |  | 0.618 |
| walker |  | 1186 | 0 | go decl at xxhash_unsafe.go:45 |  |  | 0.618 |
| walker |  | 1205 | 19 | go decl at xxhash_unsafe.go:55 |  |  | 0.618 |
| walker |  | 1226 | 21 | go decl doc at xxhash_safe.go:9 |  |  | 0.619 |
| walker |  | 1247 | 21 | go decl doc at xxhash_safe.go:14 |  |  | 0.619 |
| ns | 1253 |  | 316 | CI workflow: test job | 2.1 |  | 0.701 |
| walker |  | 1281 | 34 | go decl doc at xxhash_asm.go:12 |  |  | 0.704 |
| walker |  | 1327 | 46 | go decl names surface in xxhsum/xxhsum.go |  |  | 0.704 |
| walker |  | 1327 | 0 | go decl at xxhsum/xxhsum.go:11 |  |  | 0.704 |
| walker |  | 1327 | 0 | go decl at xxhsum/xxhsum.go:34 |  |  | 0.704 |
| walker |  | 1327 | 0 | go decl at xxhsum/xxhsum.go:43 |  |  | 0.704 |
| walker |  | 1444 | 117 | plaintext config testall.sh |  |  | 0.709 |
| walker |  | 1488 | 44 | go decl doc at xxhash_unsafe.go:38 |  |  | 0.709 |
| walker |  | 1529 | 41 | go decl doc at xxhash_unsafe.go:45 |  |  | 0.709 |
| ns | 1556 |  | 303 | CI workflow: qemu cross-arch job | 2.2 |  | 0.745 |
| walker |  | 1563 | 34 | go package + imports in xxhash_unsafe.go |  |  | 0.746 |
| ns | 1673 |  | 117 | testall.sh | 2.3 |  | 0.755 |
| walker |  | 1676 | 113 | go module file xxhashbench/go.mod |  |  | 0.755 |
| ns | 1840 |  | 167 | Build-tag map across the four platform-variant files | 3.1 |  | 0.728 |
| walker |  | 1921 | 245 | README.md section #0 |  |  | 0.837 |
| ns | 1937 |  | 97 | xxhash_asm.go body | 3.2 |  | 0.833 |
| walker |  | 2011 | 90 | go decl names surface in dynamic/plugin.go |  |  | 0.834 |
| walker |  | 2011 | 0 | go decl at dynamic/plugin.go:19 |  |  | 0.834 |
| walker |  | 2011 | 0 | go decl at dynamic/plugin.go:26 |  |  | 0.834 |
| walker |  | 2020 | 9 | go decl at dynamic/plugin.go:14 |  |  | 0.834 |
| walker |  | 2051 | 31 | go decl body at xxhash_unsafe.go:38 |  |  | 0.834 |
| walker |  | 2093 | 42 | go decl doc at xxhash_unsafe.go:55 |  |  | 0.835 |
| ns | 2186 |  | 249 | xxhash_other.go: portable-Go Sum64 (distinctive parts) | 3.3 |  | 0.769 |
| ns | 2422 |  | 236 | xxhash_other.go: portable-Go writeBlocks | 3.4 |  | 0.741 |
| walker |  | 2808 | 715 | go decl names surface in xxhash.go |  |  | 0.789 |
| walker |  | 2808 | 0 | go decl at xxhash.go:23 |  |  | 0.789 |
| walker |  | 2808 | 0 | go decl at xxhash.go:40 |  |  | 0.789 |
| walker |  | 2808 | 0 | go decl at xxhash.go:45 |  |  | 0.789 |
| walker |  | 2808 | 0 | go decl at xxhash.go:53 |  |  | 0.789 |
| walker |  | 2808 | 0 | go decl at xxhash.go:59 |  |  | 0.789 |
| walker |  | 2808 | 0 | go decl at xxhash.go:69 |  |  | 0.789 |
| walker |  | 2808 | 0 | go decl at xxhash.go:72 |  |  | 0.789 |
| walker |  | 2808 | 0 | go decl at xxhash.go:75 |  |  | 0.789 |
| walker |  | 2808 | 0 | go decl at xxhash.go:113 |  |  | 0.789 |
| walker |  | 2808 | 0 | go decl at xxhash.go:129 |  |  | 0.789 |
| walker |  | 2808 | 0 | go decl at xxhash.go:176 |  |  | 0.789 |
| walker |  | 2808 | 0 | go decl at xxhash.go:190 |  |  | 0.789 |
| walker |  | 2808 | 0 | go decl at xxhash.go:208 |  |  | 0.789 |
| walker |  | 2808 | 0 | go decl at xxhash.go:214 |  |  | 0.789 |
| walker |  | 2808 | 0 | go decl at xxhash.go:222 |  |  | 0.789 |
| walker |  | 2808 | 0 | go decl at xxhash.go:229 |  |  | 0.789 |
| walker |  | 2819 | 11 | go decl doc at xxhash.go:72 |  |  | 0.789 |
| walker |  | 2830 | 11 | go decl body at xxhash.go:40 |  |  | 0.790 |
| walker |  | 2842 | 12 | go decl doc at xxhash.go:69 |  |  | 0.790 |
| walker |  | 2854 | 12 | go decl doc at xxhash.go:129 |  |  | 0.790 |
| walker |  | 2865 | 11 | go decl body at xxhash.go:53 |  |  | 0.790 |
| walker |  | 2876 | 11 | go decl at xxhash.go:11 |  |  | 0.790 |
| walker |  | 2887 | 11 | go decl at xxhash.go:170 |  |  | 0.790 |
| walker |  | 2902 | 15 | go decl doc at xxhash.go:40 |  |  | 0.791 |
| walker |  | 2916 | 14 | go decl doc at xxhash.go:176 |  |  | 0.791 |
| ns | 2923 |  | 501 | xxhash_unsafe.go body (default !appengine path) | 3.5 |  | 0.738 |
| walker |  | 2933 | 17 | go decl doc at xxhash.go:45 |  |  | 0.738 |
| walker |  | 2951 | 18 | go decl doc at xxhash.go:75 |  |  | 0.738 |
| walker |  | 2969 | 18 | go decl doc at xxhash.go:190 |  |  | 0.739 |
| walker |  | 2988 | 19 | go decl doc at xxhash.go:113 |  |  | 0.739 |
| walker |  | 3019 | 31 | go decl doc at xxhash.go:53 |  |  | 0.739 |
| ns | 3080 |  | 157 | xxhash_safe.go body (appengine path) | 3.6 |  | 0.729 |
| walker |  | 3096 | 77 | go decl at xxhash.go:29 |  |  | 0.730 |
| walker |  | 3131 | 35 | go decl doc at xxhash.go:59 |  |  | 0.731 |
| walker |  | 3157 | 26 | go decl body at xxhash.go:45 |  |  | 0.731 |
| walker |  | 3208 | 51 | go decl doc at xxhash.go:29 |  |  | 0.733 |
| walker |  | 3256 | 48 | go package + imports in xxhash.go |  |  | 0.739 |
| ns | 3274 |  | 194 | Prime constants | 4.1 |  | 0.729 |
| walker |  | 3277 | 21 | go decl body at xxhash.go:214 |  |  | 0.729 |
| walker |  | 3330 | 53 | go decl doc at xxhash.go:23 |  |  | 0.747 |
| walker |  | 3402 | 72 | go decl body at xxhash.go:59 |  |  | 0.748 |
| ns | 3404 |  | 130 | Digest struct fields | 4.2 | 1.10 | 0.756 |
| walker |  | 3439 | 37 | go decl body at xxhash.go:222 |  |  | 0.757 |
| walker |  | 3478 | 39 | go decl body at xxhash.go:208 |  |  | 0.757 |
| walker |  | 3519 | 41 | go decl body at xxhash.go:229 |  |  | 0.757 |
| walker |  | 3627 | 108 | go decl body at xxhash.go:113 |  |  | 0.760 |
| ns | 3662 |  | 258 | New / NewWithSeed / Reset / ResetWithSeed bodies | 4.3 | 1.10 | 0.760 |
| ns | 3688 |  | 26 | Size / BlockSize doc comments | 4.4 | 1.10 | 0.762 |
| walker |  | 3763 | 136 | README.md section #1 |  |  | 0.792 |
| walker |  | 3826 | 63 | go decl names surface in xxhash_other.go |  |  | 0.795 |
| walker |  | 3826 | 0 | go decl at xxhash_other.go:7 |  |  | 0.795 |
| walker |  | 3826 | 0 | go decl at xxhash_other.go:64 |  |  | 0.795 |
| walker |  | 3836 | 10 | go package + imports in xxhash_other.go |  |  | 0.795 |
| walker |  | 3856 | 20 | go decl doc at xxhash_other.go:7 |  |  | 0.797 |
| walker |  | 4001 | 145 | go decl body at xxhash.go:176 |  |  | 0.798 |
| walker |  | 4085 | 84 | go decl body at xxhash_unsafe.go:45 |  |  | 0.812 |
| ns | 4106 |  | 418 | Write method body | 4.5 | 1.10 | 0.763 |
| walker |  | 4148 | 63 | go package + imports in xxhsum/xxhsum.go |  |  | 0.764 |
| walker |  | 4213 | 65 | go package + imports in dynamic/plugin.go |  |  | 0.764 |
| ns | 4242 |  | 136 | Sum method body | 4.6 | 1.10 | 0.769 |
| walker |  | 4264 | 51 | go decl body at xxhsum/xxhsum.go:34 |  |  | 0.769 |
| walker |  | 4468 | 204 | go decl body at xxhash.go:190 |  |  | 0.772 |
| walker |  | 4524 | 56 | go decl body at dynamic/plugin.go:19 |  |  | 0.772 |
| walker |  | 4671 | 147 | README.md section #3 |  |  | 0.772 |
| ns | 4731 |  | 489 | Sum64 method body (finalize) | 4.7 | 1.10 | 0.728 |
| walker |  | 4746 | 75 | go decl body at xxhsum/xxhsum.go:43 |  |  | 0.728 |
| walker |  | 5130 | 384 | go decl body at xxhash.go:75 |  |  | 0.786 |
| ns | 5188 |  | 457 | MarshalBinary / UnmarshalBinary + magic const | 4.8 | 1.10 | 0.792 |
| ns | 5258 |  | 70 | Marshal helpers + u64/u32 readers roster | 4.9 |  | 0.793 |
| walker |  | 5441 | 311 | README.md section #2 |  |  | 0.793 |
| ns | 5587 |  | 329 | round / mergeRound / rolN mixing primitives | 4.10 |  | 0.794 |
| ns | 5676 |  | 89 | xxhash_test.go function roster | 5.1 |  | 0.786 |
| walker |  | 5912 | 471 | go decl body at xxhash.go:129 |  |  | 0.838 |
| walker |  | 6145 | 233 | go decl body at xxhsum/xxhsum.go:11 |  |  | 0.840 |
| ns | 6342 |  | 666 | TestAll: the canonical XXH64 test-vector table | 5.2 | 5.1 | 0.799 |
| walker |  | 6421 | 276 | go decl body at dynamic/plugin.go:26 |  |  | 0.799 |
| walker |  | 6455 | 34 | go test names surface in xxhash_unsafe_test.go |  |  | 0.799 |
| walker |  | 6472 | 17 | go test names surface in xxhashbench/xxhashbench_test.go |  |  | 0.799 |
| walker |  | 6480 | 8 | plaintext config dynamic/.gitignore |  |  | 0.799 |
| walker |  | 6489 | 9 | plaintext config xxhsum/.gitignore |  |  | 0.799 |
| walker |  | 6703 | 214 | go decl body at xxhash_other.go:64 |  |  | 0.816 |
| walker |  | 6770 | 67 | go test names surface in bench_test.go |  |  | 0.816 |
| walker |  | 6802 | 32 | go test names surface in dynamic/dynamic_test.go |  |  | 0.816 |
| walker |  | 6885 | 83 | go test names surface in xxhash_test.go |  |  | 0.820 |
| ns | 6898 |  | 556 | testDigest / testSum bodies | 5.3 | 5.1 | 0.788 |
| ns | 7073 |  | 175 | TestReset | 5.4 | 5.1 | 0.776 |
| ns | 7487 |  | 414 | TestBinaryMarshaling | 5.5 | 5.1 | 0.750 |
| walker |  | 7564 | 679 | go decl body at xxhash_other.go:7 |  |  | 0.780 |
| ns | 7843 |  | 356 | TestAllocs + testAllocs helper | 5.6 | 5.1 | 0.758 |
| walker |  | 7876 | 312 | plaintext config LICENSE.txt |  |  | 0.759 |
| ns | 8295 |  | 452 | xxhash_unsafe_test.go: header + TestInlining | 5.7 |  | 0.731 |
| ns | 8342 |  | 47 | bench_test.go benchmark roster | 5.8 |  | 0.732 |
| ns | 8589 |  | 247 | BenchmarkSum64 body (representative) + benchmarks table | 5.9 | 5.8 | 0.718 |
| ns | 8932 |  | 343 | xxhsum/xxhsum.go: main + printHash | 6.1 |  | 0.725 |
| ns | 8949 |  | 17 | Build-artifact .gitignore entries | 6.2 |  | 0.726 |
| ns | 9120 |  | 171 | dynamic/plugin.go: header + test-function roster | 6.3 |  | 0.727 |
| ns | 9422 |  | 302 | dynamic/dynamic_test.go: header + TestMain + TestDynamic roster | 6.4 |  | 0.711 |
| ns | 9515 |  | 93 | xxhashbench_test.go: TODO note + function roster | 6.5 |  | 0.708 |
| ns | 9590 |  | 75 | xxhashbench_test.go: comparison-target names | 6.6 |  | 0.706 |
| ns | 9754 |  | 164 | README benchmarks table | 7.1 |  | 0.708 |
| ns | 9782 |  | 28 | LICENSE.txt (identity only) | 7.2 |  | 0.709 |
| ns | 9866 |  | 84 | xxhash_amd64.s: macro + function entry-point locations | 8.1 |  | 0.706 |
| ns | 9950 |  | 84 | xxhash_arm64.s: macro + function entry-point locations | 8.2 |  | 0.703 |

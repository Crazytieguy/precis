Score(3000)=0.438 I=0.754 C=0.254 ns_rows≤3K=18/60 grid(1000/1442/2080/3000/4327/6240/9000)=0.696/0.635/0.576/0.438/0.344/0.359/0.429

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 60 |  | 60 | README identity: name + one-line pitch | 1.1 |  | 0.000 |
| walker |  | 74 | 74 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 134 | 60 | Markdown::ReadmeHeadline { file: README.md } |  |  | 1.000 |
| walker |  | 143 | 9 | Fs::DirListing { dir: bindings } |  |  | 1.000 |
| walker |  | 148 | 5 | Fs::DirListing { dir: bindings/python } |  |  | 0.655 |
| ns | 148 |  | 88 | README capability bullets | 1.2 |  | 0.655 |
| walker |  | 154 | 6 | Fs::DirListing { dir: .github } |  |  | 0.655 |
| walker |  | 166 | 12 | Fs::DirListing { dir: .github/workflows } |  |  | 0.655 |
| walker |  | 177 | 11 | Fs::DirListing { dir: bindings/go/ncruces } |  |  | 0.655 |
| walker |  | 202 | 25 | Plaintext::Whole { file: Makefile } |  |  | 0.655 |
| ns | 222 |  | 74 | Repository root listing (complete) | 1.3 |  | 0.836 |
| walker |  | 229 | 27 | Fs::DirListing { dir: bindings/rust } |  |  | 0.837 |
| walker |  | 233 | 4 | Fs::DirListing { dir: bindings/rust/src } |  |  | 0.837 |
| ns | 320 |  | 98 | sqlite-vec.c region map | 1.4 |  | 0.745 |
| walker |  | 337 | 104 | Toml::Identity { file: sqlite-dist.toml } |  |  | 0.745 |
| walker |  | 375 | 38 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.745 |
| walker |  | 388 | 13 | Fs::DirListing { dir: scripts } |  |  | 0.745 |
| walker |  | 409 | 21 | Fs::DirListing { dir: benchmarks } |  |  | 0.745 |
| ns | 465 |  | 145 | The three public C entrypoints (complete set) | 1.5 |  | 0.666 |
| walker |  | 566 | 157 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.797 |
| walker |  | 579 | 13 | Fs::DirListing { dir: benchmarks/self-params } |  |  | 0.797 |
| ns | 592 |  | 127 | ARCHITECTURE.md: status line and section map | 1.6 |  | 0.692 |
| ns | 638 |  | 46 | README sample usage: loading and creating a vec0 table | 1.7 |  | 0.655 |
| walker |  | 651 | 72 | Fs::DirListing { dir: site } |  |  | 0.656 |
| walker |  | 659 | 8 | Fs::DirListing { dir: site/.vitepress } |  |  | 0.656 |
| walker |  | 667 | 8 | Fs::DirListing { dir: site/getting-started } |  |  | 0.656 |
| walker |  | 677 | 10 | Fs::DirListing { dir: site/features } |  |  | 0.656 |
| walker |  | 694 | 17 | Fs::DirListing { dir: site/.vitepress/theme } |  |  | 0.656 |
| ns | 874 |  | 236 | README sample usage: inserting vectors | 1.8 | 1.7 | 0.612 |
| ns | 978 |  | 104 | README sample usage: the KNN query | 1.9 | 1.8 | 0.571 |
| ns | 1194 |  | 216 | vec0Module: read-side method table (iVersion through xRowid) | 2.1 |  | 0.514 |
| walker |  | 1217 | 523 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: true } |  |  | 0.694 |
| walker |  | 1297 | 80 | Fs::DirListing { dir: examples } |  |  | 0.696 |
| walker |  | 1321 | 24 | Fs::DirListing { dir: benchmarks/micro } |  |  | 0.696 |
| walker |  | 1325 | 4 | Fs::DirListing { dir: benchmarks/micro/src } |  |  | 0.696 |
| walker |  | 1331 | 6 | Fs::DirListing { dir: benchmarks/micro/benches } |  |  | 0.696 |
| ns | 1434 |  | 240 | vec0Module: write-side slots and the unimplemented ones | 2.2 | 2.1 | 0.633 |
| walker |  | 1475 | 144 | Fs::DirListing { dir: tests } |  |  | 0.636 |
| walker |  | 1486 | 11 | Fs::DirListing { dir: tests/correctness } |  |  | 0.636 |
| walker |  | 1499 | 13 | Fs::DirListing { dir: tests/minimum } |  |  | 0.636 |
| walker |  | 1523 | 24 | Fs::DirListing { dir: tests/afbd } |  |  | 0.636 |
| walker |  | 1554 | 31 | Fs::DirListing { dir: benchmarks/exhaustive-memory } |  |  | 0.636 |
| ns | 1629 |  | 195 | vec0 hard limits and column-index constants | 2.3 |  | 0.595 |
| walker |  | 1710 | 156 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.595 |
| walker |  | 1758 | 48 | Fs::DirListing { dir: tests/fuzz } |  |  | 0.595 |
| walker |  | 1807 | 49 | Fs::DirListing { dir: site/using } |  |  | 0.597 |
| ns | 1829 |  | 200 | Shadow table name macros (complete set) | 2.4 |  | 0.576 |
| walker |  | 1858 | 51 | Fs::DirListing { dir: site/guides } |  |  | 0.576 |
| walker |  | 2015 | 157 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.576 |
| ns | 2141 |  | 312 | Shadow table CREATE TABLE DDL | 2.5 | 2.4 | 0.524 |
| ns | 2239 |  | 98 | vec0 query plan enum | 2.6 |  | 0.509 |
| walker |  | 2286 | 271 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.509 |
| walker |  | 2296 | 10 | Code::CodeKey { rung: Names, file: bindings/rust/build.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.509 |
| ns | 2392 |  | 153 | vec0BestIndex query-plan selection rules | 2.7 |  | 0.489 |
| walker |  | 2516 | 220 | Plaintext::Whole { file: bindings/rust/Makefile } |  |  | 0.489 |
| walker |  | 2528 | 12 | Fs::DirListing { dir: benchmarks/profiling } |  |  | 0.489 |
| ns | 2566 |  | 174 | vec0 user column kinds | 2.8 |  | 0.467 |
| walker |  | 2634 | 106 | Code::CodeKey { rung: Names, file: bindings/python/extra_init.py, decl: 0, sub: 0, line: 0 } |  |  | 0.467 |
| walker |  | 2643 | 9 | Code::CodeKey { rung: Doc, file: bindings/python/extra_init.py, decl: 3, sub: 0, line: 16 } |  |  | 0.467 |
| walker |  | 2657 | 14 | Code::CodeKey { rung: Body, file: bindings/python/extra_init.py, decl: 4, sub: 0, line: 42 } |  |  | 0.467 |
| walker |  | 2681 | 24 | Code::CodeKey { rung: Doc, file: bindings/python/extra_init.py, decl: 1, sub: 0, line: 6 } |  |  | 0.467 |
| walker |  | 2696 | 15 | Code::CodeKey { rung: Body, file: bindings/python/extra_init.py, decl: 1, sub: 0, line: 6 } |  |  | 0.467 |
| walker |  | 2720 | 24 | Code::CodeKey { rung: Doc, file: bindings/python/extra_init.py, decl: 2, sub: 0, line: 11 } |  |  | 0.467 |
| walker |  | 2735 | 15 | Code::CodeKey { rung: Body, file: bindings/python/extra_init.py, decl: 2, sub: 0, line: 11 } |  |  | 0.467 |
| walker |  | 2750 | 15 | Fs::DirListing { dir: tests/leak-fixtures } |  |  | 0.467 |
| ns | 2873 |  | 307 | idxStr block kinds (complete enum) | 2.9 |  | 0.438 |
| walker |  | 2940 | 190 | Code::CodeKey { rung: Names, file: sqlite-vec.c, decl: 0, sub: 0, line: 0 } |  |  | 0.438 |
| walker |  | 2966 | 26 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 3, sub: 0, line: 70 } |  |  | 0.438 |
| walker |  | 3011 | 45 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 1, sub: 0, line: 64 } |  |  | 0.438 |
| ns | 3045 |  | 172 | Partition key operator encoding | 2.10 |  | 0.424 |
| walker |  | 3147 | 136 | Code::CodeKey { rung: Names, file: tmp-static.py, decl: 0, sub: 0, line: 0 } |  |  | 0.424 |
| ns | 3277 |  | 232 | Metadata and distance-constraint operator encodings | 2.11 | 2.10 | 0.404 |
| walker |  | 3297 | 150 | Code::CodeKey { rung: Decl, file: tmp-static.py, decl: 4, sub: 0, line: 13 } |  |  | 0.404 |
| walker |  | 3465 | 168 | Code::CodeKey { rung: Names, file: sqlite-vec.c, decl: 0, sub: 1, line: 0 } |  |  | 0.404 |
| walker |  | 3477 | 12 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 16, sub: 0, line: 361 } |  |  | 0.404 |
| walker |  | 3557 | 80 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 15, sub: 0, line: 115 } |  |  | 0.405 |
| ns | 3608 |  | 331 | struct vec0_vtab: field roster | 2.12 |  | 0.381 |
| walker |  | 3719 | 162 | Code::CodeKey { rung: Names, file: sqlite-vec.c, decl: 0, sub: 2, line: 0 } |  |  | 0.381 |
| ns | 3720 |  | 112 | struct vec0_vtab: cached prepared statements | 2.13 | 2.12 | 0.371 |
| walker |  | 3731 | 12 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 24, sub: 0, line: 463 } |  |  | 0.371 |
| walker |  | 3743 | 12 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 25, sub: 0, line: 481 } |  |  | 0.371 |
| walker |  | 3913 | 170 | Code::CodeKey { rung: Names, file: sqlite-vec.c, decl: 0, sub: 3, line: 0 } |  |  | 0.372 |
| ns | 3934 |  | 214 | Column definition structs and distance metrics | 2.14 |  | 0.354 |
| ns | 4027 |  | 93 | Metadata column kinds | 2.15 |  | 0.349 |
| walker |  | 4099 | 186 | Code::CodeKey { rung: Names, file: sqlite-vec.c, decl: 0, sub: 4, line: 0 } |  |  | 0.350 |
| walker |  | 4141 | 42 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 36, sub: 0, line: 598 } |  |  | 0.350 |
| ns | 4157 |  | 130 | vec0 constructor parser: function roster | 2.16 |  | 0.343 |
| walker |  | 4302 | 161 | Code::CodeKey { rung: Names, file: sqlite-vec.c, decl: 0, sub: 5, line: 0 } |  |  | 0.344 |
| walker |  | 4328 | 26 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 45, sub: 0, line: 811 } |  |  | 0.344 |
| walker |  | 4354 | 26 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 46, sub: 0, line: 831 } |  |  | 0.344 |
| walker |  | 4381 | 27 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 44, sub: 0, line: 687 } |  |  | 0.344 |
| walker |  | 4411 | 30 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 47, sub: 0, line: 964 } |  |  | 0.344 |
| ns | 4439 |  | 282 | chunk_size table option: validation and default | 2.17 |  | 0.331 |
| walker |  | 4465 | 54 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 48, sub: 0, line: 998 } |  |  | 0.331 |
| ns | 4542 |  | 103 | vec0 constructor and lifecycle functions (roster) | 2.18 |  | 0.327 |
| walker |  | 4651 | 186 | Code::CodeKey { rung: Names, file: sqlite-vec.c, decl: 0, sub: 6, line: 0 } |  |  | 0.327 |
| walker |  | 4664 | 13 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 55, sub: 0, line: 1131 } |  |  | 0.327 |
| walker |  | 4677 | 13 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 56, sub: 0, line: 1151 } |  |  | 0.327 |
| walker |  | 4690 | 13 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 57, sub: 0, line: 1193 } |  |  | 0.327 |
| walker |  | 4715 | 25 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 50, sub: 0, line: 1050 } |  |  | 0.327 |
| ns | 4736 |  | 194 | vec0 column dispatch helpers (roster) | 2.19 |  | 0.319 |
| walker |  | 4884 | 169 | Code::CodeKey { rung: Names, file: sqlite-vec.c, decl: 0, sub: 7, line: 0 } |  |  | 0.320 |
| walker |  | 4897 | 13 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 58, sub: 0, line: 1234 } |  |  | 0.320 |
| walker |  | 4910 | 13 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 59, sub: 0, line: 1275 } |  |  | 0.320 |
| walker |  | 4923 | 13 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 62, sub: 0, line: 1346 } |  |  | 0.320 |
| walker |  | 4936 | 13 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 63, sub: 0, line: 1415 } |  |  | 0.320 |
| ns | 4950 |  | 214 | vec0 storage helpers (roster) | 2.20 |  | 0.312 |
| walker |  | 5119 | 183 | Code::CodeKey { rung: Names, file: sqlite-vec.c, decl: 0, sub: 8, line: 0 } |  |  | 0.313 |
| walker |  | 5132 | 13 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 66, sub: 0, line: 1574 } |  |  | 0.313 |
| walker |  | 5145 | 13 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 67, sub: 0, line: 1690 } |  |  | 0.313 |
| walker |  | 5158 | 13 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 68, sub: 0, line: 1740 } |  |  | 0.313 |
| walker |  | 5171 | 13 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 69, sub: 0, line: 1789 } |  |  | 0.313 |
| ns | 5186 |  | 236 | vec0 cursor and per-query-plan state | 2.21 | 2.6 | 0.304 |
| walker |  | 5209 | 38 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 71, sub: 0, line: 1806 } |  |  | 0.304 |
| walker |  | 5278 | 69 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 70, sub: 0, line: 1798 } |  |  | 0.304 |
| ns | 5304 |  | 118 | KNN primitives: bitmaps and merge (roster) | 2.22 |  | 0.300 |
| walker |  | 5485 | 207 | Code::CodeKey { rung: Names, file: sqlite-vec.c, decl: 0, sub: 9, line: 0 } |  |  | 0.307 |
| walker |  | 5496 | 11 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 80, sub: 0, line: 1886 } |  |  | 0.307 |
| walker |  | 5530 | 34 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 79, sub: 0, line: 1880 } |  |  | 0.300 |
| ns | 5530 |  | 226 | vec0 read path (roster) | 2.23 |  | 0.300 |
| walker |  | 5565 | 35 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 82, sub: 0, line: 1900 } |  |  | 0.300 |
| walker |  | 5603 | 38 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 83, sub: 0, line: 1956 } |  |  | 0.300 |
| ns | 5816 |  | 286 | vec0 write path (roster) | 2.24 |  | 0.291 |
| walker |  | 5826 | 223 | Code::CodeKey { rung: Names, file: sqlite-vec.c, decl: 0, sub: 10, line: 0 } |  |  | 0.317 |
| walker |  | 5837 | 11 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 93, sub: 0, line: 2267 } |  |  | 0.317 |
| walker |  | 5852 | 15 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 95, sub: 0, line: 2294 } |  |  | 0.317 |
| walker |  | 5885 | 33 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 90, sub: 0, line: 2250 } |  |  | 0.320 |
| walker |  | 5918 | 33 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 91, sub: 0, line: 2256 } |  |  | 0.322 |
| walker |  | 5955 | 37 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 92, sub: 0, line: 2261 } |  |  | 0.325 |
| walker |  | 5993 | 38 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 84, sub: 0, line: 2033 } |  |  | 0.325 |
| ns | 5995 |  | 179 | Registered SQL scalar functions (complete list) | 3.1 |  | 0.318 |
| walker |  | 6031 | 38 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 87, sub: 0, line: 2171 } |  |  | 0.318 |
| walker |  | 6073 | 42 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 86, sub: 0, line: 2111 } |  |  | 0.318 |
| ns | 6096 |  | 101 | Registered virtual table modules | 3.2 |  | 0.315 |
| walker |  | 6132 | 59 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 88, sub: 0, line: 2236 } |  |  | 0.328 |
| walker |  | 6193 | 61 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 89, sub: 0, line: 2242 } |  |  | 0.355 |
| ns | 6205 |  | 109 | vec_debug build string | 3.3 |  | 0.352 |
| walker |  | 6262 | 69 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 85, sub: 0, line: 2091 } |  |  | 0.367 |
| ns | 6439 |  | 234 | Scalar function implementations (roster) | 3.4 |  | 0.394 |
| walker |  | 6441 | 179 | Code::CodeKey { rung: Names, file: sqlite-vec.c, decl: 0, sub: 11, line: 0 } |  |  | 0.395 |
| walker |  | 6457 | 16 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 97, sub: 0, line: 2430 } |  |  | 0.395 |
| walker |  | 6473 | 16 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 104, sub: 0, line: 2493 } |  |  | 0.395 |
| walker |  | 6507 | 34 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 100, sub: 0, line: 2444 } |  |  | 0.395 |
| ns | 6530 |  | 91 | Vector element types | 3.5 |  | 0.404 |
| walker |  | 6579 | 72 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 99, sub: 0, line: 2435 } |  |  | 0.404 |
| walker |  | 6773 | 194 | Code::CodeKey { rung: Names, file: sqlite-vec.c, decl: 0, sub: 12, line: 0 } |  |  | 0.405 |
| walker |  | 6783 | 10 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 109, sub: 0, line: 2561 } |  |  | 0.405 |
| walker |  | 6797 | 14 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 113, sub: 0, line: 2641 } |  |  | 0.405 |
| ns | 6805 |  | 275 | Distance functions, scalar and SIMD (roster) | 3.6 |  | 0.413 |
| walker |  | 6819 | 22 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 105, sub: 0, line: 2522 } |  |  | 0.413 |
| walker |  | 6859 | 40 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 112, sub: 0, line: 2635 } |  |  | 0.413 |
| ns | 6981 |  | 176 | Vector value conversion helpers (roster) | 3.7 |  | 0.427 |
| walker |  | 6993 | 134 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 111, sub: 0, line: 2622 } |  |  | 0.427 |
| ns | 7189 |  | 208 | vec_each table function: structs, columns, methods, module | 3.8 |  | 0.437 |
| ns | 7329 |  | 140 | vec_npy_each: .npy readers and vtab entry points | 3.9 |  | 0.432 |
| walker |  | 7389 | 396 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 110, sub: 0, line: 2588 } |  |  | 0.432 |
| ns | 7395 |  | 66 | vec_static_blobs / vec_static_blob_entries: entry points | 3.10 |  | 0.430 |
| walker |  | 7585 | 196 | Code::CodeKey { rung: Names, file: sqlite-vec.c, decl: 0, sub: 13, line: 0 } |  |  | 0.431 |
| walker |  | 7596 | 11 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 115, sub: 0, line: 2724 } |  |  | 0.431 |
| walker |  | 7612 | 16 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 120, sub: 0, line: 2877 } |  |  | 0.431 |
| ns | 7623 |  | 228 | Makefile: complete target roster | 4.1 |  | 0.422 |
| walker |  | 7640 | 28 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 121, sub: 0, line: 2881 } |  |  | 0.424 |
| walker |  | 7677 | 37 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 114, sub: 0, line: 2718 } |  |  | 0.424 |
| walker |  | 7734 | 57 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 118, sub: 0, line: 2740 } |  |  | 0.424 |
| ns | 7801 |  | 178 | Makefile: platform and SIMD detection | 4.2 |  | 0.418 |
| walker |  | 7906 | 172 | Code::CodeKey { rung: Names, file: sqlite-vec.c, decl: 0, sub: 14, line: 0 } |  |  | 0.421 |
| walker |  | 7940 | 34 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 126, sub: 0, line: 3078 } |  |  | 0.421 |
| walker |  | 7987 | 47 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 125, sub: 0, line: 3026 } |  |  | 0.421 |
| ns | 7992 |  | 191 | Compile-time options | 4.3 |  | 0.418 |
| ns | 8136 |  | 144 | Public header template | 4.4 |  | 0.412 |
| walker |  | 8217 | 230 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 123, sub: 0, line: 2887 } |  |  | 0.412 |
| ns | 8331 |  | 195 | Build recipes for the shipped artifacts | 4.5 | 4.1 | 0.407 |
| walker |  | 8437 | 220 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 123, sub: 1, line: 2887 } |  |  | 0.407 |
| ns | 8490 |  | 159 | Distribution manifest | 4.6 |  | 0.401 |
| walker |  | 8569 | 132 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 123, sub: 2, line: 2887 } |  |  | 0.401 |
| ns | 8655 |  | 165 | TODO / roadmap | 4.7 |  | 0.397 |
| ns | 8667 |  | 12 | CI workflows | 4.8 |  | 0.400 |
| walker |  | 8754 | 185 | Code::CodeKey { rung: Names, file: sqlite-vec.c, decl: 0, sub: 15, line: 0 } |  |  | 0.402 |
| walker |  | 8770 | 16 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 130, sub: 0, line: 3137 } |  |  | 0.402 |
| walker |  | 8786 | 16 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 135, sub: 0, line: 3273 } |  |  | 0.402 |
| walker |  | 8802 | 16 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 136, sub: 0, line: 3302 } |  |  | 0.402 |
| ns | 8811 |  | 144 | tests/ listing (complete) | 5.1 |  | 0.426 |
| walker |  | 8830 | 28 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 131, sub: 0, line: 3166 } |  |  | 0.426 |
| ns | 8898 |  | 87 | pytest fixture: how the tests load the extension | 5.2 |  | 0.423 |
| walker |  | 9047 | 217 | Code::CodeKey { rung: Names, file: sqlite-vec.c, decl: 0, sub: 16, line: 0 } |  |  | 0.430 |
| walker |  | 9063 | 16 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 137, sub: 0, line: 3329 } |  |  | 0.430 |
| walker |  | 9155 | 92 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 145, sub: 0, line: 3384 } |  |  | 0.433 |
| walker |  | 9173 | 18 | Code::CodeKey { rung: Doc, file: sqlite-vec.c, decl: 145, sub: 0, line: 3384 } |  |  | 0.433 |
| ns | 9200 |  | 302 | Feature test suites: complete test function roster | 5.3 |  | 0.424 |
| ns | 9280 |  | 80 | examples/ listing (complete) | 5.4 |  | 0.436 |
| ns | 9451 |  | 171 | Python example, end to end | 5.5 |  | 0.430 |
| ns | 9499 |  | 48 | bindings/ listings (complete) | 5.6 |  | 0.438 |
| walker |  | 9589 | 416 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 138, sub: 0, line: 3341 } |  |  | 0.438 |
| ns | 9638 |  | 139 | Documentation site listings (complete) | 5.7 |  | 0.458 |
| walker |  | 9777 | 188 | Code::CodeKey { rung: Names, file: sqlite-vec.c, decl: 0, sub: 17, line: 0 } |  |  | 0.469 |
| walker |  | 9833 | 56 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 150, sub: 0, line: 3417 } |  |  | 0.475 |
| ns | 9838 |  | 200 | vec0 column-type guide: the three non-vector options | 5.8 |  | 0.469 |
| walker |  | 9917 | 84 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 147, sub: 0, line: 3394 } |  |  | 0.481 |
| ns | 9951 |  | 113 | KNN query guide: the two supported forms | 5.9 |  | 0.478 |

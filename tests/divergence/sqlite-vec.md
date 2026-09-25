Score(3000)=0.448 I=0.761 C=0.264 ns_rows≤3K=18/60 grid(1000/1442/2080/3000/4327/6240/9000)=0.590/0.645/0.589/0.448/0.351/0.286/0.427

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 60 |  | 60 | README identity: name + one-line pitch | 1.1 |  | 0.000 |
| walker |  | 74 | 74 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 83 | 9 | Fs::DirListing { dir: bindings } |  |  | 0.000 |
| walker |  | 88 | 5 | Fs::DirListing { dir: bindings/go } |  |  | 0.000 |
| walker |  | 93 | 5 | Fs::DirListing { dir: bindings/python } |  |  | 0.000 |
| walker |  | 101 | 8 | Fs::DirListing { dir: bindings/go/ncruces } |  |  | 0.000 |
| walker |  | 107 | 6 | Fs::DirListing { dir: .github } |  |  | 0.000 |
| walker |  | 119 | 12 | Fs::DirListing { dir: .github/workflows } |  |  | 0.000 |
| ns | 148 |  | 88 | README capability bullets | 1.2 |  | 0.000 |
| walker |  | 179 | 60 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.655 |
| walker |  | 206 | 27 | Fs::DirListing { dir: bindings/rust } |  |  | 0.655 |
| walker |  | 210 | 4 | Fs::DirListing { dir: bindings/rust/src } |  |  | 0.655 |
| ns | 222 |  | 74 | Repository root listing (complete) | 1.3 |  | 0.837 |
| walker |  | 235 | 25 | Plaintext::Whole { file: Makefile } |  |  | 0.837 |
| ns | 320 |  | 98 | sqlite-vec.c region map | 1.4 |  | 0.745 |
| walker |  | 339 | 104 | Toml::Identity { file: sqlite-dist.toml } |  |  | 0.745 |
| walker |  | 377 | 38 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.745 |
| walker |  | 390 | 13 | Fs::DirListing { dir: scripts } |  |  | 0.745 |
| walker |  | 438 | 48 | Markdown::HeadingsOutline { file: ARCHITECTURE.md } |  |  | 0.748 |
| walker |  | 459 | 21 | Fs::DirListing { dir: benchmarks } |  |  | 0.748 |
| ns | 465 |  | 145 | The three public C entrypoints (complete set) | 1.5 |  | 0.669 |
| walker |  | 472 | 13 | Fs::DirListing { dir: benchmarks/self-params } |  |  | 0.669 |
| ns | 592 |  | 127 | ARCHITECTURE.md: status line and section map | 1.6 |  | 0.594 |
| walker |  | 629 | 157 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.708 |
| ns | 638 |  | 46 | README sample usage: loading and creating a vec0 table | 1.7 |  | 0.670 |
| walker |  | 701 | 72 | Fs::DirListing { dir: site } |  |  | 0.671 |
| walker |  | 709 | 8 | Fs::DirListing { dir: site/.vitepress } |  |  | 0.671 |
| walker |  | 726 | 17 | Fs::DirListing { dir: site/.vitepress/theme } |  |  | 0.671 |
| walker |  | 806 | 80 | Fs::DirListing { dir: examples } |  |  | 0.673 |
| walker |  | 830 | 24 | Fs::DirListing { dir: benchmarks/micro } |  |  | 0.673 |
| walker |  | 834 | 4 | Fs::DirListing { dir: benchmarks/micro/src } |  |  | 0.673 |
| ns | 874 |  | 236 | README sample usage: inserting vectors | 1.8 | 1.7 | 0.628 |
| walker |  | 978 | 144 | Fs::DirListing { dir: tests } |  |  | 0.589 |
| ns | 978 |  | 104 | README sample usage: the KNN query | 1.9 | 1.8 | 0.589 |
| ns | 1194 |  | 216 | vec0Module: read-side method table (iVersion through xRowid) | 2.1 |  | 0.530 |
| ns | 1434 |  | 240 | vec0Module: write-side slots and the unimplemented ones | 2.2 | 2.1 | 0.482 |
| walker |  | 1501 | 523 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: true } |  |  | 0.645 |
| walker |  | 1549 | 48 | Fs::DirListing { dir: tests/fuzz } |  |  | 0.645 |
| walker |  | 1598 | 49 | Fs::DirListing { dir: site/using } |  |  | 0.647 |
| ns | 1629 |  | 195 | vec0 hard limits and column-index constants | 2.3 |  | 0.605 |
| walker |  | 1649 | 51 | Fs::DirListing { dir: site/guides } |  |  | 0.605 |
| walker |  | 1785 | 136 | Code::CodeKey { rung: Names, file: tmp-static.py, decl: 0, sub: 0, line: 0 } |  |  | 0.605 |
| ns | 1829 |  | 200 | Shadow table name macros (complete set) | 2.4 |  | 0.584 |
| walker |  | 1935 | 150 | Code::CodeKey { rung: Decl, file: tmp-static.py, decl: 4, sub: 0, line: 13 } |  |  | 0.584 |
| walker |  | 2056 | 121 | Markdown::Section { file: ARCHITECTURE.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.589 |
| ns | 2141 |  | 312 | Shadow table CREATE TABLE DDL | 2.5 | 2.4 | 0.536 |
| walker |  | 2212 | 156 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.536 |
| walker |  | 2220 | 8 | Fs::DirListing { dir: site/getting-started } |  |  | 0.536 |
| ns | 2239 |  | 98 | vec0 query plan enum | 2.6 |  | 0.521 |
| ns | 2392 |  | 153 | vec0BestIndex query-plan selection rules | 2.7 |  | 0.500 |
| walker |  | 2431 | 211 | Code::CodeKey { rung: Body, file: tmp-static.py, decl: 5, sub: 0, line: 25 } |  |  | 0.500 |
| walker |  | 2441 | 10 | Code::CodeKey { rung: Names, file: bindings/rust/build.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.500 |
| walker |  | 2473 | 32 | Code::CodeKey { rung: Body, file: bindings/rust/build.rs, decl: 1, sub: 0, line: 1 } |  |  | 0.500 |
| walker |  | 2478 | 5 | Fs::DirListing { dir: tests/fuzz/corpus } |  |  | 0.500 |
| walker |  | 2488 | 10 | Fs::DirListing { dir: site/features } |  |  | 0.500 |
| walker |  | 2499 | 11 | Fs::DirListing { dir: tests/correctness } |  |  | 0.500 |
| ns | 2566 |  | 174 | vec0 user column kinds | 2.8 |  | 0.478 |
| walker |  | 2656 | 157 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.478 |
| walker |  | 2662 | 6 | Fs::DirListing { dir: benchmarks/micro/benches } |  |  | 0.478 |
| walker |  | 2674 | 12 | Fs::DirListing { dir: benchmarks/profiling } |  |  | 0.478 |
| ns | 2873 |  | 307 | idxStr block kinds (complete enum) | 2.9 |  | 0.448 |
| walker |  | 2894 | 220 | Plaintext::Whole { file: bindings/rust/Makefile } |  |  | 0.448 |
| ns | 3045 |  | 172 | Partition key operator encoding | 2.10 |  | 0.434 |
| walker |  | 3165 | 271 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.434 |
| walker |  | 3178 | 13 | Fs::DirListing { dir: tests/minimum } |  |  | 0.434 |
| ns | 3277 |  | 232 | Metadata and distance-constraint operator encodings | 2.11 | 2.10 | 0.414 |
| walker |  | 3373 | 195 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.414 |
| walker |  | 3388 | 15 | Fs::DirListing { dir: tests/leak-fixtures } |  |  | 0.414 |
| walker |  | 3427 | 39 | Code::CodeKey { rung: Names, file: bindings/python/extra_init.py, decl: 0, sub: 0, line: 0 } |  |  | 0.414 |
| walker |  | 3444 | 17 | Code::CodeKey { rung: Body, file: bindings/python/extra_init.py, decl: 1, sub: 0, line: 6 } |  |  | 0.414 |
| walker |  | 3463 | 19 | Code::CodeKey { rung: Body, file: bindings/python/extra_init.py, decl: 2, sub: 0, line: 11 } |  |  | 0.414 |
| walker |  | 3485 | 22 | Code::CodeKey { rung: Doc, file: bindings/python/extra_init.py, decl: 1, sub: 0, line: 6 } |  |  | 0.414 |
| walker |  | 3507 | 22 | Code::CodeKey { rung: Doc, file: bindings/python/extra_init.py, decl: 2, sub: 0, line: 11 } |  |  | 0.414 |
| ns | 3608 |  | 331 | struct vec0_vtab: field roster | 2.12 |  | 0.389 |
| ns | 3720 |  | 112 | struct vec0_vtab: cached prepared statements | 2.13 | 2.12 | 0.379 |
| walker |  | 3741 | 234 | Code::CodeKey { rung: Names, file: sqlite-vec.c, decl: 0, sub: 0, line: 0 } |  |  | 0.379 |
| walker |  | 3753 | 12 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 16, sub: 0, line: 361 } |  |  | 0.379 |
| walker |  | 3779 | 26 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 3, sub: 0, line: 70 } |  |  | 0.379 |
| walker |  | 3824 | 45 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 1, sub: 0, line: 64 } |  |  | 0.379 |
| walker |  | 3904 | 80 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 15, sub: 0, line: 115 } |  |  | 0.380 |
| ns | 3934 |  | 214 | Column definition structs and distance metrics | 2.14 |  | 0.362 |
| ns | 4027 |  | 93 | Metadata column kinds | 2.15 |  | 0.357 |
| walker |  | 4094 | 190 | Code::CodeKey { rung: Names, file: sqlite-vec.c, decl: 0, sub: 1, line: 0 } |  |  | 0.357 |
| ns | 4157 |  | 130 | vec0 constructor parser: function roster | 2.16 |  | 0.351 |
| walker |  | 4298 | 204 | Code::CodeKey { rung: Names, file: sqlite-vec.c, decl: 0, sub: 2, line: 0 } |  |  | 0.351 |
| walker |  | 4310 | 12 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 24, sub: 0, line: 463 } |  |  | 0.351 |
| walker |  | 4322 | 12 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 25, sub: 0, line: 481 } |  |  | 0.351 |
| walker |  | 4421 | 99 | Code::CodeKey { rung: Doc, file: sqlite-vec.c, decl: 29, sub: 0, line: 556 } |  |  | 0.351 |
| ns | 4439 |  | 282 | chunk_size table option: validation and default | 2.17 |  | 0.338 |
| ns | 4542 |  | 103 | vec0 constructor and lifecycle functions (roster) | 2.18 |  | 0.333 |
| walker |  | 4679 | 258 | Code::CodeKey { rung: Names, file: sqlite-vec.c, decl: 0, sub: 3, line: 0 } |  |  | 0.334 |
| walker |  | 4721 | 42 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 36, sub: 0, line: 598 } |  |  | 0.334 |
| ns | 4736 |  | 194 | vec0 column dispatch helpers (roster) | 2.19 |  | 0.326 |
| walker |  | 4811 | 90 | Code::CodeKey { rung: Doc, file: sqlite-vec.c, decl: 37, sub: 0, line: 614 } |  |  | 0.326 |
| ns | 4950 |  | 214 | vec0 storage helpers (roster) | 2.20 |  | 0.318 |
| walker |  | 5040 | 229 | Code::CodeKey { rung: Names, file: sqlite-vec.c, decl: 0, sub: 4, line: 0 } |  |  | 0.319 |
| walker |  | 5065 | 25 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 50, sub: 0, line: 1050 } |  |  | 0.319 |
| walker |  | 5091 | 26 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 45, sub: 0, line: 811 } |  |  | 0.319 |
| walker |  | 5117 | 26 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 46, sub: 0, line: 831 } |  |  | 0.319 |
| walker |  | 5144 | 27 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 44, sub: 0, line: 687 } |  |  | 0.319 |
| walker |  | 5174 | 30 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 47, sub: 0, line: 964 } |  |  | 0.319 |
| ns | 5186 |  | 236 | vec0 cursor and per-query-plan state | 2.21 | 2.6 | 0.310 |
| walker |  | 5228 | 54 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 48, sub: 0, line: 998 } |  |  | 0.310 |
| ns | 5304 |  | 118 | KNN primitives: bitmaps and merge (roster) | 2.22 |  | 0.305 |
| walker |  | 5446 | 218 | Code::CodeKey { rung: Names, file: sqlite-vec.c, decl: 0, sub: 5, line: 0 } |  |  | 0.306 |
| walker |  | 5459 | 13 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 55, sub: 0, line: 1131 } |  |  | 0.306 |
| walker |  | 5472 | 13 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 56, sub: 0, line: 1151 } |  |  | 0.306 |
| walker |  | 5485 | 13 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 57, sub: 0, line: 1193 } |  |  | 0.306 |
| walker |  | 5498 | 13 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 58, sub: 0, line: 1234 } |  |  | 0.306 |
| walker |  | 5511 | 13 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 59, sub: 0, line: 1275 } |  |  | 0.306 |
| ns | 5530 |  | 226 | vec0 read path (roster) | 2.23 |  | 0.298 |
| walker |  | 5741 | 230 | Code::CodeKey { rung: Names, file: sqlite-vec.c, decl: 0, sub: 6, line: 0 } |  |  | 0.299 |
| walker |  | 5754 | 13 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 62, sub: 0, line: 1346 } |  |  | 0.299 |
| walker |  | 5767 | 13 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 63, sub: 0, line: 1415 } |  |  | 0.299 |
| walker |  | 5780 | 13 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 66, sub: 0, line: 1574 } |  |  | 0.299 |
| walker |  | 5793 | 13 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 67, sub: 0, line: 1690 } |  |  | 0.299 |
| walker |  | 5806 | 13 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 68, sub: 0, line: 1740 } |  |  | 0.299 |
| ns | 5816 |  | 286 | vec0 write path (roster) | 2.24 |  | 0.291 |
| walker |  | 5819 | 13 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 69, sub: 0, line: 1789 } |  |  | 0.291 |
| walker |  | 5857 | 38 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 71, sub: 0, line: 1806 } |  |  | 0.291 |
| walker |  | 5926 | 69 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 70, sub: 0, line: 1798 } |  |  | 0.291 |
| ns | 5995 |  | 179 | Registered SQL scalar functions (complete list) | 3.1 |  | 0.285 |
| ns | 6096 |  | 101 | Registered virtual table modules | 3.2 |  | 0.282 |
| walker |  | 6161 | 235 | Code::CodeKey { rung: Names, file: sqlite-vec.c, decl: 0, sub: 7, line: 0 } |  |  | 0.289 |
| walker |  | 6172 | 11 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 80, sub: 0, line: 1886 } |  |  | 0.289 |
| ns | 6205 |  | 109 | vec_debug build string | 3.3 |  | 0.286 |
| walker |  | 6206 | 34 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 79, sub: 0, line: 1880 } |  |  | 0.286 |
| walker |  | 6241 | 35 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 82, sub: 0, line: 1900 } |  |  | 0.286 |
| walker |  | 6279 | 38 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 83, sub: 0, line: 1956 } |  |  | 0.286 |
| ns | 6439 |  | 234 | Scalar function implementations (roster) | 3.4 |  | 0.323 |
| ns | 6530 |  | 91 | Vector element types | 3.5 |  | 0.336 |
| walker |  | 6547 | 268 | Code::CodeKey { rung: Names, file: sqlite-vec.c, decl: 0, sub: 8, line: 0 } |  |  | 0.356 |
| walker |  | 6558 | 11 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 93, sub: 0, line: 2267 } |  |  | 0.356 |
| walker |  | 6573 | 15 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 95, sub: 0, line: 2294 } |  |  | 0.356 |
| walker |  | 6589 | 16 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 97, sub: 0, line: 2430 } |  |  | 0.356 |
| walker |  | 6622 | 33 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 90, sub: 0, line: 2250 } |  |  | 0.358 |
| walker |  | 6655 | 33 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 91, sub: 0, line: 2256 } |  |  | 0.360 |
| walker |  | 6692 | 37 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 92, sub: 0, line: 2261 } |  |  | 0.362 |
| walker |  | 6730 | 38 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 84, sub: 0, line: 2033 } |  |  | 0.362 |
| walker |  | 6768 | 38 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 87, sub: 0, line: 2171 } |  |  | 0.362 |
| ns | 6805 |  | 275 | Distance functions, scalar and SIMD (roster) | 3.6 |  | 0.373 |
| walker |  | 6810 | 42 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 86, sub: 0, line: 2111 } |  |  | 0.373 |
| walker |  | 6869 | 59 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 88, sub: 0, line: 2236 } |  |  | 0.383 |
| walker |  | 6930 | 61 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 89, sub: 0, line: 2242 } |  |  | 0.404 |
| ns | 6981 |  | 176 | Vector value conversion helpers (roster) | 3.7 |  | 0.419 |
| walker |  | 6999 | 69 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 85, sub: 0, line: 2091 } |  |  | 0.431 |
| ns | 7189 |  | 208 | vec_each table function: structs, columns, methods, module | 3.8 |  | 0.424 |
| walker |  | 7215 | 216 | Code::CodeKey { rung: Names, file: sqlite-vec.c, decl: 0, sub: 9, line: 0 } |  |  | 0.434 |
| walker |  | 7231 | 16 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 104, sub: 0, line: 2493 } |  |  | 0.434 |
| walker |  | 7253 | 22 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 105, sub: 0, line: 2522 } |  |  | 0.434 |
| walker |  | 7287 | 34 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 100, sub: 0, line: 2444 } |  |  | 0.434 |
| ns | 7329 |  | 140 | vec_npy_each: .npy readers and vtab entry points | 3.9 |  | 0.429 |
| walker |  | 7359 | 72 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 99, sub: 0, line: 2435 } |  |  | 0.429 |
| ns | 7395 |  | 66 | vec_static_blobs / vec_static_blob_entries: entry points | 3.10 |  | 0.427 |
| walker |  | 7605 | 246 | Code::CodeKey { rung: Names, file: sqlite-vec.c, decl: 0, sub: 10, line: 0 } |  |  | 0.435 |
| walker |  | 7615 | 10 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 109, sub: 0, line: 2561 } |  |  | 0.435 |
| ns | 7623 |  | 228 | Makefile: complete target roster | 4.1 |  | 0.425 |
| walker |  | 7626 | 11 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 115, sub: 0, line: 2724 } |  |  | 0.425 |
| walker |  | 7640 | 14 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 113, sub: 0, line: 2641 } |  |  | 0.425 |
| walker |  | 7677 | 37 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 114, sub: 0, line: 2718 } |  |  | 0.425 |
| walker |  | 7717 | 40 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 112, sub: 0, line: 2635 } |  |  | 0.425 |
| walker |  | 7774 | 57 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 118, sub: 0, line: 2740 } |  |  | 0.425 |
| ns | 7801 |  | 178 | Makefile: platform and SIMD detection | 4.2 |  | 0.419 |
| walker |  | 7908 | 134 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 111, sub: 0, line: 2622 } |  |  | 0.419 |
| ns | 7992 |  | 191 | Compile-time options | 4.3 |  | 0.415 |
| ns | 8136 |  | 144 | Public header template | 4.4 |  | 0.410 |
| walker |  | 8304 | 396 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 110, sub: 0, line: 2588 } |  |  | 0.410 |
| ns | 8331 |  | 195 | Build recipes for the shipped artifacts | 4.5 | 4.1 | 0.405 |
| ns | 8490 |  | 159 | Distribution manifest | 4.6 |  | 0.399 |
| walker |  | 8538 | 234 | Code::CodeKey { rung: Names, file: sqlite-vec.c, decl: 0, sub: 11, line: 0 } |  |  | 0.402 |
| walker |  | 8554 | 16 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 120, sub: 0, line: 2877 } |  |  | 0.402 |
| walker |  | 8582 | 28 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 121, sub: 0, line: 2881 } |  |  | 0.406 |
| walker |  | 8616 | 34 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 126, sub: 0, line: 3078 } |  |  | 0.406 |
| ns | 8655 |  | 165 | TODO / roadmap | 4.7 |  | 0.402 |
| walker |  | 8663 | 47 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 125, sub: 0, line: 3026 } |  |  | 0.402 |
| ns | 8667 |  | 12 | CI workflows | 4.8 |  | 0.405 |
| ns | 8811 |  | 144 | tests/ listing (complete) | 5.1 |  | 0.428 |
| walker |  | 8871 | 208 | Code::CodeKey { rung: Names, file: sqlite-vec.c, decl: 0, sub: 12, line: 0 } |  |  | 0.430 |
| walker |  | 8887 | 16 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 130, sub: 0, line: 3137 } |  |  | 0.430 |
| ns | 8898 |  | 87 | pytest fixture: how the tests load the extension | 5.2 |  | 0.427 |
| walker |  | 8903 | 16 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 135, sub: 0, line: 3273 } |  |  | 0.427 |
| walker |  | 8919 | 16 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 136, sub: 0, line: 3302 } |  |  | 0.427 |
| walker |  | 8935 | 16 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 137, sub: 0, line: 3329 } |  |  | 0.427 |
| walker |  | 8963 | 28 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 131, sub: 0, line: 3166 } |  |  | 0.427 |
| ns | 9200 |  | 302 | Feature test suites: complete test function roster | 5.3 |  | 0.418 |
| ns | 9280 |  | 80 | examples/ listing (complete) | 5.4 |  | 0.430 |
| walker |  | 9288 | 325 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 123, sub: 0, line: 2887 } |  |  | 0.430 |
| ns | 9451 |  | 171 | Python example, end to end | 5.5 |  | 0.424 |
| ns | 9501 |  | 50 | bindings/ listings (complete) | 5.6 |  | 0.433 |
| walker |  | 9551 | 263 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 123, sub: 1, line: 2887 } |  |  | 0.433 |
| ns | 9640 |  | 139 | Documentation site listings (complete) | 5.7 |  | 0.453 |
| walker |  | 9817 | 266 | Code::CodeKey { rung: Names, file: sqlite-vec.c, decl: 0, sub: 13, line: 0 } |  |  | 0.460 |
| ns | 9840 |  | 200 | vec0 column-type guide: the three non-vector options | 5.8 |  | 0.454 |
| walker |  | 9901 | 84 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 147, sub: 0, line: 3394 } |  |  | 0.459 |
| ns | 9953 |  | 113 | KNN query guide: the two supported forms | 5.9 |  | 0.455 |
| walker |  | 9989 | 88 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 148, sub: 0, line: 3405 } |  |  | 0.456 |
| walker |  | 9989 | 0 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 145, sub: 0, line: 3384 } |  |  | 0.456 |

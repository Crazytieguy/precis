Score(3000)=0.438 I=0.754 C=0.254 ns_rows≤3K=18/60 grid(1000/1442/2080/3000/4327/6240/9000)=0.699/0.635/0.576/0.438/0.344/0.285/0.434

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
| walker |  | 204 | 27 | Fs::DirListing { dir: bindings/rust } |  |  | 0.655 |
| walker |  | 208 | 4 | Fs::DirListing { dir: bindings/rust/src } |  |  | 0.655 |
| ns | 222 |  | 74 | Repository root listing (complete) | 1.3 |  | 0.837 |
| walker |  | 312 | 104 | Toml::Identity { file: sqlite-dist.toml } |  |  | 0.838 |
| ns | 320 |  | 98 | sqlite-vec.c region map | 1.4 |  | 0.745 |
| walker |  | 350 | 38 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.745 |
| walker |  | 388 | 38 | Plaintext::Whole { file: Makefile } |  |  | 0.745 |
| walker |  | 401 | 13 | Fs::DirListing { dir: scripts } |  |  | 0.745 |
| ns | 465 |  | 145 | The three public C entrypoints (complete set) | 1.5 |  | 0.666 |
| walker |  | 523 | 122 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.797 |
| walker |  | 544 | 21 | Fs::DirListing { dir: benchmarks } |  |  | 0.797 |
| walker |  | 557 | 13 | Fs::DirListing { dir: benchmarks/self-params } |  |  | 0.797 |
| ns | 592 |  | 127 | ARCHITECTURE.md: status line and section map | 1.6 |  | 0.692 |
| walker |  | 629 | 72 | Fs::DirListing { dir: site } |  |  | 0.693 |
| walker |  | 637 | 8 | Fs::DirListing { dir: site/.vitepress } |  |  | 0.693 |
| ns | 638 |  | 46 | README sample usage: loading and creating a vec0 table | 1.7 |  | 0.656 |
| walker |  | 645 | 8 | Fs::DirListing { dir: site/getting-started } |  |  | 0.656 |
| walker |  | 655 | 10 | Fs::DirListing { dir: site/features } |  |  | 0.656 |
| walker |  | 672 | 17 | Fs::DirListing { dir: site/.vitepress/theme } |  |  | 0.656 |
| ns | 874 |  | 236 | README sample usage: inserting vectors | 1.8 | 1.7 | 0.612 |
| ns | 978 |  | 104 | README sample usage: the KNN query | 1.9 | 1.8 | 0.571 |
| ns | 1194 |  | 216 | vec0Module: read-side method table (iVersion through xRowid) | 2.1 |  | 0.514 |
| walker |  | 1195 | 523 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: true } |  |  | 0.694 |
| walker |  | 1275 | 80 | Fs::DirListing { dir: examples } |  |  | 0.696 |
| walker |  | 1299 | 24 | Fs::DirListing { dir: benchmarks/micro } |  |  | 0.696 |
| walker |  | 1303 | 4 | Fs::DirListing { dir: benchmarks/micro/src } |  |  | 0.696 |
| walker |  | 1309 | 6 | Fs::DirListing { dir: benchmarks/micro/benches } |  |  | 0.696 |
| ns | 1434 |  | 240 | vec0Module: write-side slots and the unimplemented ones | 2.2 | 2.1 | 0.633 |
| walker |  | 1453 | 144 | Fs::DirListing { dir: tests } |  |  | 0.636 |
| walker |  | 1464 | 11 | Fs::DirListing { dir: tests/correctness } |  |  | 0.636 |
| walker |  | 1477 | 13 | Fs::DirListing { dir: tests/minimum } |  |  | 0.636 |
| walker |  | 1501 | 24 | Fs::DirListing { dir: tests/afbd } |  |  | 0.636 |
| walker |  | 1514 | 13 | Code::CodeKey { rung: Names, file: bindings/rust/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.636 |
| walker |  | 1542 | 28 | Code::CodeKey { rung: Decl, file: bindings/rust/src/lib.rs, decl: 1, sub: 0, line: 1 } |  |  | 0.636 |
| walker |  | 1573 | 31 | Fs::DirListing { dir: benchmarks/exhaustive-memory } |  |  | 0.636 |
| walker |  | 1621 | 48 | Fs::DirListing { dir: tests/fuzz } |  |  | 0.636 |
| ns | 1629 |  | 195 | vec0 hard limits and column-index constants | 2.3 |  | 0.595 |
| walker |  | 1670 | 49 | Fs::DirListing { dir: site/using } |  |  | 0.597 |
| walker |  | 1721 | 51 | Fs::DirListing { dir: site/guides } |  |  | 0.597 |
| ns | 1829 |  | 200 | Shadow table name macros (complete set) | 2.4 |  | 0.576 |
| walker |  | 1878 | 157 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.576 |
| ns | 2141 |  | 312 | Shadow table CREATE TABLE DDL | 2.5 | 2.4 | 0.524 |
| walker |  | 2151 | 273 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.524 |
| walker |  | 2161 | 10 | Code::CodeKey { rung: Names, file: bindings/rust/build.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.524 |
| ns | 2239 |  | 98 | vec0 query plan enum | 2.6 |  | 0.509 |
| walker |  | 2381 | 220 | Plaintext::Whole { file: bindings/rust/Makefile } |  |  | 0.509 |
| ns | 2392 |  | 153 | vec0BestIndex query-plan selection rules | 2.7 |  | 0.489 |
| walker |  | 2393 | 12 | Fs::DirListing { dir: benchmarks/profiling } |  |  | 0.489 |
| walker |  | 2499 | 106 | Code::CodeKey { rung: Names, file: bindings/python/extra_init.py, decl: 0, sub: 0, line: 0 } |  |  | 0.489 |
| walker |  | 2508 | 9 | Code::CodeKey { rung: Doc, file: bindings/python/extra_init.py, decl: 3, sub: 0, line: 16 } |  |  | 0.489 |
| walker |  | 2522 | 14 | Code::CodeKey { rung: Body, file: bindings/python/extra_init.py, decl: 4, sub: 0, line: 42 } |  |  | 0.489 |
| walker |  | 2537 | 15 | Fs::DirListing { dir: tests/leak-fixtures } |  |  | 0.489 |
| walker |  | 2561 | 24 | Code::CodeKey { rung: Doc, file: bindings/python/extra_init.py, decl: 1, sub: 0, line: 6 } |  |  | 0.489 |
| ns | 2566 |  | 174 | vec0 user column kinds | 2.8 |  | 0.467 |
| walker |  | 2576 | 15 | Code::CodeKey { rung: Body, file: bindings/python/extra_init.py, decl: 1, sub: 0, line: 6 } |  |  | 0.467 |
| walker |  | 2600 | 24 | Code::CodeKey { rung: Doc, file: bindings/python/extra_init.py, decl: 2, sub: 0, line: 11 } |  |  | 0.467 |
| walker |  | 2615 | 15 | Code::CodeKey { rung: Body, file: bindings/python/extra_init.py, decl: 2, sub: 0, line: 11 } |  |  | 0.467 |
| walker |  | 2825 | 210 | Code::CodeKey { rung: Names, file: sqlite-vec.c, decl: 0, sub: 0, line: 0 } |  |  | 0.467 |
| walker |  | 2830 | 5 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 1, sub: 0, line: 60 } |  |  | 0.467 |
| walker |  | 2856 | 26 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 4, sub: 0, line: 70 } |  |  | 0.467 |
| ns | 2873 |  | 307 | idxStr block kinds (complete enum) | 2.9 |  | 0.438 |
| walker |  | 2992 | 136 | Code::CodeKey { rung: Names, file: tmp-static.py, decl: 0, sub: 0, line: 0 } |  |  | 0.438 |
| ns | 3045 |  | 172 | Partition key operator encoding | 2.10 |  | 0.424 |
| walker |  | 3142 | 150 | Code::CodeKey { rung: Decl, file: tmp-static.py, decl: 4, sub: 0, line: 13 } |  |  | 0.424 |
| ns | 3277 |  | 232 | Metadata and distance-constraint operator encodings | 2.11 | 2.10 | 0.404 |
| walker |  | 3376 | 234 | Code::CodeKey { rung: Names, file: sqlite-vec.c, decl: 0, sub: 1, line: 0 } |  |  | 0.404 |
| walker |  | 3381 | 5 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 15, sub: 0, line: 96 } |  |  | 0.404 |
| walker |  | 3386 | 5 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 16, sub: 0, line: 100 } |  |  | 0.404 |
| walker |  | 3391 | 5 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 17, sub: 0, line: 104 } |  |  | 0.404 |
| walker |  | 3396 | 5 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 18, sub: 0, line: 108 } |  |  | 0.404 |
| walker |  | 3403 | 7 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 14, sub: 0, line: 86 } |  |  | 0.404 |
| walker |  | 3481 | 78 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 21, sub: 0, line: 115 } |  |  | 0.405 |
| ns | 3608 |  | 331 | struct vec0_vtab: field roster | 2.12 |  | 0.380 |
| walker |  | 3662 | 181 | Code::CodeKey { rung: Names, file: sqlite-vec.c, decl: 0, sub: 2, line: 0 } |  |  | 0.381 |
| walker |  | 3674 | 12 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 26, sub: 0, line: 169 } |  |  | 0.381 |
| walker |  | 3686 | 12 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 27, sub: 0, line: 227 } |  |  | 0.381 |
| walker |  | 3698 | 12 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 28, sub: 0, line: 266 } |  |  | 0.381 |
| walker |  | 3717 | 19 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 24, sub: 0, line: 128 } |  |  | 0.381 |
| ns | 3720 |  | 112 | struct vec0_vtab: cached prepared statements | 2.13 | 2.12 | 0.371 |
| walker |  | 3874 | 157 | Code::CodeKey { rung: Names, file: sqlite-vec.c, decl: 0, sub: 3, line: 0 } |  |  | 0.371 |
| walker |  | 3886 | 12 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 30, sub: 0, line: 361 } |  |  | 0.371 |
| walker |  | 3905 | 19 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 29, sub: 0, line: 326 } |  |  | 0.371 |
| ns | 3934 |  | 214 | Column definition structs and distance metrics | 2.14 |  | 0.354 |
| ns | 4027 |  | 93 | Metadata column kinds | 2.15 |  | 0.349 |
| walker |  | 4056 | 151 | Code::CodeKey { rung: Names, file: sqlite-vec.c, decl: 0, sub: 4, line: 0 } |  |  | 0.349 |
| walker |  | 4068 | 12 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 38, sub: 0, line: 463 } |  |  | 0.349 |
| walker |  | 4090 | 22 | Code::CodeKey { rung: Doc, file: sqlite-vec.c, decl: 26, sub: 0, line: 169 } |  |  | 0.349 |
| ns | 4157 |  | 130 | vec0 constructor parser: function roster | 2.16 |  | 0.343 |
| walker |  | 4274 | 184 | Code::CodeKey { rung: Names, file: sqlite-vec.c, decl: 0, sub: 5, line: 0 } |  |  | 0.344 |
| walker |  | 4286 | 12 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 39, sub: 0, line: 481 } |  |  | 0.344 |
| walker |  | 4298 | 12 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 43, sub: 0, line: 534 } |  |  | 0.344 |
| ns | 4439 |  | 282 | chunk_size table option: validation and default | 2.17 |  | 0.331 |
| walker |  | 4493 | 195 | Code::CodeKey { rung: Names, file: sqlite-vec.c, decl: 0, sub: 6, line: 0 } |  |  | 0.331 |
| walker |  | 4535 | 42 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 52, sub: 0, line: 598 } |  |  | 0.331 |
| ns | 4542 |  | 103 | vec0 constructor and lifecycle functions (roster) | 2.18 |  | 0.326 |
| walker |  | 4719 | 184 | Code::CodeKey { rung: Names, file: sqlite-vec.c, decl: 0, sub: 7, line: 0 } |  |  | 0.327 |
| ns | 4736 |  | 194 | vec0 column dispatch helpers (roster) | 2.19 |  | 0.319 |
| walker |  | 4745 | 26 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 61, sub: 0, line: 811 } |  |  | 0.319 |
| walker |  | 4771 | 26 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 62, sub: 0, line: 831 } |  |  | 0.319 |
| walker |  | 4798 | 27 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 60, sub: 0, line: 687 } |  |  | 0.319 |
| ns | 4950 |  | 214 | vec0 storage helpers (roster) | 2.20 |  | 0.312 |
| walker |  | 4997 | 199 | Code::CodeKey { rung: Names, file: sqlite-vec.c, decl: 0, sub: 8, line: 0 } |  |  | 0.313 |
| walker |  | 5020 | 23 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 68, sub: 0, line: 1056 } |  |  | 0.313 |
| walker |  | 5045 | 25 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 66, sub: 0, line: 1050 } |  |  | 0.313 |
| walker |  | 5075 | 30 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 63, sub: 0, line: 964 } |  |  | 0.313 |
| walker |  | 5129 | 54 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 64, sub: 0, line: 998 } |  |  | 0.313 |
| ns | 5186 |  | 236 | vec0 cursor and per-query-plan state | 2.21 | 2.6 | 0.304 |
| walker |  | 5299 | 170 | Code::CodeKey { rung: Names, file: sqlite-vec.c, decl: 0, sub: 9, line: 0 } |  |  | 0.304 |
| ns | 5304 |  | 118 | KNN primitives: bitmaps and merge (roster) | 2.22 |  | 0.299 |
| walker |  | 5312 | 13 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 72, sub: 0, line: 1131 } |  |  | 0.299 |
| walker |  | 5325 | 13 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 73, sub: 0, line: 1151 } |  |  | 0.299 |
| walker |  | 5338 | 13 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 74, sub: 0, line: 1193 } |  |  | 0.299 |
| walker |  | 5351 | 13 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 75, sub: 0, line: 1234 } |  |  | 0.299 |
| walker |  | 5364 | 13 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 76, sub: 0, line: 1275 } |  |  | 0.299 |
| ns | 5530 |  | 226 | vec0 read path (roster) | 2.23 |  | 0.292 |
| walker |  | 5536 | 172 | Code::CodeKey { rung: Names, file: sqlite-vec.c, decl: 0, sub: 10, line: 0 } |  |  | 0.293 |
| walker |  | 5549 | 13 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 79, sub: 0, line: 1346 } |  |  | 0.293 |
| walker |  | 5562 | 13 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 80, sub: 0, line: 1415 } |  |  | 0.293 |
| walker |  | 5575 | 13 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 83, sub: 0, line: 1574 } |  |  | 0.293 |
| walker |  | 5774 | 199 | Code::CodeKey { rung: Names, file: sqlite-vec.c, decl: 0, sub: 11, line: 0 } |  |  | 0.293 |
| walker |  | 5787 | 13 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 84, sub: 0, line: 1690 } |  |  | 0.293 |
| walker |  | 5800 | 13 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 85, sub: 0, line: 1740 } |  |  | 0.293 |
| walker |  | 5813 | 13 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 86, sub: 0, line: 1789 } |  |  | 0.293 |
| ns | 5816 |  | 286 | vec0 write path (roster) | 2.24 |  | 0.285 |
| walker |  | 5851 | 38 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 88, sub: 0, line: 1806 } |  |  | 0.285 |
| walker |  | 5920 | 69 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 87, sub: 0, line: 1798 } |  |  | 0.285 |
| ns | 5995 |  | 179 | Registered SQL scalar functions (complete list) | 3.1 |  | 0.279 |
| ns | 6096 |  | 101 | Registered virtual table modules | 3.2 |  | 0.276 |
| walker |  | 6104 | 184 | Code::CodeKey { rung: Names, file: sqlite-vec.c, decl: 0, sub: 12, line: 0 } |  |  | 0.288 |
| walker |  | 6115 | 11 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 97, sub: 0, line: 1886 } |  |  | 0.288 |
| walker |  | 6149 | 34 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 96, sub: 0, line: 1880 } |  |  | 0.288 |
| walker |  | 6184 | 35 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 99, sub: 0, line: 1900 } |  |  | 0.288 |
| ns | 6205 |  | 109 | vec_debug build string | 3.3 |  | 0.285 |
| walker |  | 6222 | 38 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 100, sub: 0, line: 1956 } |  |  | 0.285 |
| walker |  | 6260 | 38 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 101, sub: 0, line: 2033 } |  |  | 0.285 |
| walker |  | 6329 | 69 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 102, sub: 0, line: 2091 } |  |  | 0.303 |
| ns | 6439 |  | 234 | Scalar function implementations (roster) | 3.4 |  | 0.338 |
| ns | 6530 |  | 91 | Vector element types | 3.5 |  | 0.350 |
| walker |  | 6533 | 204 | Code::CodeKey { rung: Names, file: sqlite-vec.c, decl: 0, sub: 13, line: 0 } |  |  | 0.366 |
| walker |  | 6544 | 11 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 110, sub: 0, line: 2267 } |  |  | 0.366 |
| walker |  | 6559 | 15 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 112, sub: 0, line: 2294 } |  |  | 0.366 |
| walker |  | 6577 | 18 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 114, sub: 0, line: 2430 } |  |  | 0.366 |
| walker |  | 6610 | 33 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 107, sub: 0, line: 2250 } |  |  | 0.368 |
| walker |  | 6643 | 33 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 108, sub: 0, line: 2256 } |  |  | 0.370 |
| walker |  | 6680 | 37 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 109, sub: 0, line: 2261 } |  |  | 0.372 |
| walker |  | 6718 | 38 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 104, sub: 0, line: 2171 } |  |  | 0.372 |
| walker |  | 6760 | 42 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 103, sub: 0, line: 2111 } |  |  | 0.372 |
| ns | 6805 |  | 275 | Distance functions, scalar and SIMD (roster) | 3.6 |  | 0.397 |
| walker |  | 6819 | 59 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 105, sub: 0, line: 2236 } |  |  | 0.407 |
| walker |  | 6880 | 61 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 106, sub: 0, line: 2242 } |  |  | 0.427 |
| ns | 6981 |  | 176 | Vector value conversion helpers (roster) | 3.7 |  | 0.444 |
| walker |  | 7056 | 176 | Code::CodeKey { rung: Names, file: sqlite-vec.c, decl: 0, sub: 14, line: 0 } |  |  | 0.444 |
| walker |  | 7072 | 16 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 121, sub: 0, line: 2493 } |  |  | 0.444 |
| walker |  | 7094 | 22 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 122, sub: 0, line: 2522 } |  |  | 0.444 |
| walker |  | 7128 | 34 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 117, sub: 0, line: 2444 } |  |  | 0.444 |
| ns | 7189 |  | 208 | vec_each table function: structs, columns, methods, module | 3.8 |  | 0.443 |
| walker |  | 7200 | 72 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 116, sub: 0, line: 2435 } |  |  | 0.443 |
| ns | 7329 |  | 140 | vec_npy_each: .npy readers and vtab entry points | 3.9 |  | 0.438 |
| ns | 7395 |  | 66 | vec_static_blobs / vec_static_blob_entries: entry points | 3.10 |  | 0.436 |
| walker |  | 7404 | 204 | Code::CodeKey { rung: Names, file: sqlite-vec.c, decl: 0, sub: 15, line: 0 } |  |  | 0.447 |
| walker |  | 7414 | 10 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 126, sub: 0, line: 2561 } |  |  | 0.447 |
| walker |  | 7425 | 11 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 132, sub: 0, line: 2724 } |  |  | 0.447 |
| walker |  | 7439 | 14 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 130, sub: 0, line: 2641 } |  |  | 0.447 |
| walker |  | 7476 | 37 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 131, sub: 0, line: 2718 } |  |  | 0.447 |
| walker |  | 7516 | 40 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 129, sub: 0, line: 2635 } |  |  | 0.447 |
| ns | 7623 |  | 228 | Makefile: complete target roster | 4.1 |  | 0.437 |
| walker |  | 7650 | 134 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 128, sub: 0, line: 2622 } |  |  | 0.437 |
| ns | 7801 |  | 178 | Makefile: platform and SIMD detection | 4.2 |  | 0.430 |
| ns | 7992 |  | 191 | Compile-time options | 4.3 |  | 0.427 |
| walker |  | 8046 | 396 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 127, sub: 0, line: 2588 } |  |  | 0.427 |
| ns | 8136 |  | 144 | Public header template | 4.4 |  | 0.421 |
| walker |  | 8239 | 193 | Code::CodeKey { rung: Names, file: sqlite-vec.c, decl: 0, sub: 16, line: 0 } |  |  | 0.422 |
| walker |  | 8255 | 16 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 137, sub: 0, line: 2877 } |  |  | 0.422 |
| walker |  | 8283 | 28 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 138, sub: 0, line: 2881 } |  |  | 0.424 |
| ns | 8331 |  | 195 | Build recipes for the shipped artifacts | 4.5 | 4.1 | 0.419 |
| walker |  | 8340 | 57 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 135, sub: 0, line: 2740 } |  |  | 0.419 |
| ns | 8490 |  | 159 | Distribution manifest | 4.6 |  | 0.413 |
| walker |  | 8564 | 224 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 140, sub: 0, line: 2887 } |  |  | 0.413 |
| ns | 8655 |  | 165 | TODO / roadmap | 4.7 |  | 0.409 |
| ns | 8667 |  | 12 | CI workflows | 4.8 |  | 0.412 |
| walker |  | 8784 | 220 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 140, sub: 1, line: 2887 } |  |  | 0.412 |
| ns | 8811 |  | 144 | tests/ listing (complete) | 5.1 |  | 0.435 |
| ns | 8898 |  | 87 | pytest fixture: how the tests load the extension | 5.2 |  | 0.432 |
| walker |  | 8922 | 138 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 140, sub: 2, line: 2887 } |  |  | 0.432 |
| walker |  | 9083 | 161 | Code::CodeKey { rung: Names, file: sqlite-vec.c, decl: 0, sub: 17, line: 0 } |  |  | 0.436 |
| walker |  | 9091 | 8 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 142, sub: 0, line: 2935 } |  |  | 0.436 |
| walker |  | 9125 | 34 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 144, sub: 0, line: 3078 } |  |  | 0.436 |
| walker |  | 9172 | 47 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 143, sub: 0, line: 3026 } |  |  | 0.436 |
| ns | 9200 |  | 302 | Feature test suites: complete test function roster | 5.3 |  | 0.426 |
| ns | 9280 |  | 80 | examples/ listing (complete) | 5.4 |  | 0.438 |
| walker |  | 9357 | 185 | Code::CodeKey { rung: Names, file: sqlite-vec.c, decl: 0, sub: 18, line: 0 } |  |  | 0.440 |
| walker |  | 9373 | 16 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 148, sub: 0, line: 3137 } |  |  | 0.440 |
| walker |  | 9389 | 16 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 153, sub: 0, line: 3273 } |  |  | 0.440 |
| walker |  | 9417 | 28 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 149, sub: 0, line: 3166 } |  |  | 0.440 |
| ns | 9451 |  | 171 | Python example, end to end | 5.5 |  | 0.434 |
| ns | 9499 |  | 48 | bindings/ listings (complete) | 5.6 |  | 0.442 |
| walker |  | 9630 | 213 | Code::CodeKey { rung: Names, file: sqlite-vec.c, decl: 0, sub: 19, line: 0 } |  |  | 0.448 |
| ns | 9638 |  | 139 | Documentation site listings (complete) | 5.7 |  | 0.466 |
| walker |  | 9646 | 16 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 154, sub: 0, line: 3302 } |  |  | 0.466 |
| walker |  | 9662 | 16 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 155, sub: 0, line: 3329 } |  |  | 0.466 |
| walker |  | 9756 | 94 | Code::CodeKey { rung: Decl, file: sqlite-vec.c, decl: 163, sub: 0, line: 3384 } |  |  | 0.469 |
| ns | 9838 |  | 200 | vec0 column-type guide: the three non-vector options | 5.8 |  | 0.463 |
| ns | 9951 |  | 113 | KNN query guide: the two supported forms | 5.9 |  | 0.460 |

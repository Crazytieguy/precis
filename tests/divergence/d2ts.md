Score(3000)=0.670 I=0.851 C=0.528 ns_rows≤3K=24/55 grid(1000/1442/2080/3000/4327/6240/9000)=0.505/0.648/0.756/0.670/0.661/0.625/0.566

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 54 | 54 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 65 | 11 | Code::CodeKey { rung: Names, file: eslint.base.mjs, decl: 0, sub: 0, line: 0 } |  |  | 0.000 |
| walker |  | 68 | 3 | Fs::DirListing { dir: .github } |  |  | 0.000 |
| walker |  | 72 | 4 | Fs::DirListing { dir: .github/workflows } |  |  | 0.000 |
| ns | 81 |  | 81 | Repository identity — README title + one-line definition | 1.1 |  | 0.000 |
| walker |  | 94 | 22 | Fs::DirListing { dir: packages } |  |  | 0.000 |
| ns | 135 |  | 54 | Complete repository root listing | 1.2 |  | 0.633 |
| walker |  | 139 | 45 | Json::Identity { file: package.json } |  |  | 0.636 |
| walker |  | 156 | 17 | Fs::DirListing { dir: packages/d2ts-benchmark } |  |  | 0.637 |
| walker |  | 168 | 12 | Fs::DirListing { dir: packages/d2ts-benchmark/src } |  |  | 0.637 |
| ns | 192 |  | 57 | Workspace membership — packages/ and examples/ listings | 1.3 |  | 0.476 |
| walker |  | 206 | 38 | Code::CodeKey { rung: Names, file: packages/d2ts-benchmark/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.476 |
| walker |  | 214 | 8 | Fs::DirListing { dir: .changeset } |  |  | 0.476 |
| ns | 218 |  | 26 | pnpm workspace globs | 1.4 |  | 0.447 |
| walker |  | 239 | 25 | Fs::DirListing { dir: packages/d2ql } |  |  | 0.451 |
| walker |  | 269 | 30 | Fs::DirListing { dir: packages/d2mini } |  |  | 0.459 |
| ns | 288 |  | 70 | README lede — what incremental execution buys you | 1.5 | 1.1 | 0.451 |
| walker |  | 350 | 81 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.701 |
| walker |  | 386 | 36 | Fs::DirListing { dir: packages/d2ts } |  |  | 0.713 |
| ns | 387 |  | 99 | README lede — ElectricSQL ShapeStreams and pipeline type inference | 1.6 | 1.1 | 0.676 |
| walker |  | 471 | 85 | Json::Identity { file: packages/d2ts/package.json } |  |  | 0.678 |
| walker |  | 545 | 74 | Json::Dependencies { file: packages/d2ts/package.json } |  |  | 0.678 |
| walker |  | 578 | 33 | Fs::DirListing { dir: packages/d2mini/src } |  |  | 0.557 |
| ns | 578 |  | 191 | Root package.json — identity and every workspace script | 1.7 |  | 0.557 |
| walker |  | 625 | 47 | Code::CodeKey { rung: Names, file: packages/d2mini/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.557 |
| ns | 728 |  | 150 | Published package identity: d2ts and d2mini | 1.8 |  | 0.532 |
| walker |  | 729 | 104 | Fs::DirListing { dir: packages/d2mini/src/operators } |  |  | 0.542 |
| walker |  | 785 | 56 | Code::CodeKey { rung: Decl, file: packages/d2ts-benchmark/src/index.ts, decl: 2, sub: 0, line: 25 } |  |  | 0.542 |
| walker |  | 829 | 44 | Fs::DirListing { dir: packages/d2ts/src } |  |  | 0.547 |
| walker |  | 833 | 4 | Fs::DirListing { dir: packages/d2ts/src/electric } |  |  | 0.547 |
| walker |  | 853 | 20 | Fs::DirListing { dir: packages/d2ts/src/sqlite } |  |  | 0.548 |
| ns | 871 |  | 143 | Private package identity: d2ql and d2ts-benchmark | 1.9 | 1.8 | 0.503 |
| walker |  | 923 | 70 | Code::CodeKey { rung: Names, file: packages/d2ts/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.505 |
| walker |  | 1021 | 98 | Fs::DirListing { dir: packages/d2ts/src/operators } |  |  | 0.514 |
| ns | 1032 |  | 161 | Root README section map (every H2) | 1.10 |  | 0.453 |
| walker |  | 1034 | 13 | Code::CodeKey { rung: Names, file: packages/d2mini/src/operators/filterBy.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.453 |
| walker |  | 1047 | 13 | Code::CodeKey { rung: Names, file: packages/d2ts/src/operators/filterBy.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.453 |
| walker |  | 1134 | 87 | Code::CodeKey { rung: Decl, file: packages/d2ts-benchmark/src/index.ts, decl: 3, sub: 0, line: 496 } |  |  | 0.453 |
| walker |  | 1169 | 35 | Fs::DirListing { dir: examples } |  |  | 0.587 |
| walker |  | 1228 | 59 | Markdown::ReadmeHeadline { file: packages/d2mini/README.md } |  |  | 0.587 |
| ns | 1284 |  | 252 | README "Implementation Details" — provenance and the four core data structures | 1.11 | 1.10 | 0.523 |
| walker |  | 1297 | 69 | Fs::DirListing { dir: packages/d2ql/src } |  |  | 0.526 |
| walker |  | 1306 | 9 | Fs::DirListing { dir: packages/d2ql/src/query-builder } |  |  | 0.527 |
| walker |  | 1344 | 38 | Code::CodeKey { rung: Names, file: packages/d2ql/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.527 |
| ns | 1392 |  | 108 | Every package's own root listing | 2.1 |  | 0.621 |
| walker |  | 1397 | 53 | Code::CodeKey { rung: ModuleDoc, file: packages/d2ql/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.622 |
| walker |  | 1418 | 21 | Code::CodeKey { rung: Names, file: packages/d2ql/src/compiler.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.622 |
| ns | 1436 |  | 44 | packages/d2ts/src — complete module roster | 2.2 |  | 0.648 |
| walker |  | 1495 | 77 | Markdown::ReadmeHeadline { file: packages/d2ql/README.md } |  |  | 0.648 |
| ns | 1506 |  | 70 | packages/d2ts/src/index.ts — the public export barrel | 2.3 |  | 0.659 |
| walker |  | 1528 | 33 | Markdown::Section { file: packages/d2ql/README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.659 |
| ns | 1604 |  | 98 | packages/d2ts/src/operators — complete operator file roster | 2.4 |  | 0.691 |
| walker |  | 1606 | 78 | Markdown::ReadmeHeadline { file: packages/d2ts/README.md } |  |  | 0.692 |
| walker |  | 1645 | 39 | Markdown::Section { file: packages/d2mini/README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.692 |
| walker |  | 1818 | 173 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.764 |
| ns | 1844 |  | 240 | packages/d2ts/src/operators/index.ts — exported operator set | 2.5 | 2.4 | 0.715 |
| ns | 1981 |  | 137 | packages/d2mini/src and its operators directory | 2.6 |  | 0.737 |
| walker |  | 1990 | 172 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.756 |
| walker |  | 2025 | 35 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/compiler.ts, decl: 1, sub: 0, line: 16 } |  |  | 0.756 |
| walker |  | 2063 | 38 | Code::CodeKey { rung: Names, file: packages/d2ts/src/version-index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.756 |
| walker |  | 2089 | 26 | Plaintext::Whole { file: pnpm-workspace.yaml } |  |  | 0.771 |
| walker |  | 2115 | 26 | Code::CodeKey { rung: Names, file: packages/d2mini/src/operators/topK.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.771 |
| walker |  | 2141 | 26 | Code::CodeKey { rung: Names, file: packages/d2ts/src/operators/topK.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.771 |
| walker |  | 2170 | 29 | Fs::DirListing { dir: packages/d2mini/tests } |  |  | 0.772 |
| ns | 2246 |  | 265 | packages/d2mini export barrels | 2.7 | 2.6 | 0.731 |
| walker |  | 2306 | 136 | Json::Scripts { file: package.json } |  |  | 0.779 |
| ns | 2324 |  | 78 | packages/d2ql/src and query-builder/ listings | 2.8 |  | 0.787 |
| walker |  | 2337 | 31 | Code::CodeKey { rung: Names, file: packages/d2mini/src/operators/debug.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.787 |
| walker |  | 2364 | 27 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/operators/debug.ts, decl: 1, sub: 0, line: 12 } |  |  | 0.787 |
| walker |  | 2395 | 31 | Code::CodeKey { rung: Names, file: packages/d2mini/src/operators/output.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.787 |
| walker |  | 2422 | 27 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/operators/output.ts, decl: 1, sub: 0, line: 13 } |  |  | 0.787 |
| ns | 2441 |  | 117 | packages/d2ql export barrels | 2.9 | 2.8 | 0.783 |
| walker |  | 2453 | 31 | Code::CodeKey { rung: Names, file: packages/d2ts/src/operators/debug.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.783 |
| walker |  | 2480 | 27 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/debug.ts, decl: 1, sub: 0, line: 18 } |  |  | 0.783 |
| ns | 2518 |  | 77 | packages/d2ts/src/sqlite — complete tree listing | 2.10 |  | 0.758 |
| ns | 2701 |  | 183 | SQLite subpath export barrels | 2.11 | 2.10 | 0.736 |
| walker |  | 2863 | 383 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: true } |  |  | 0.738 |
| ns | 2886 |  | 185 | Test suite layout — immediate children of every tests/ directory | 2.12 |  | 0.695 |
| walker |  | 2893 | 30 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/operators/output.ts, decl: 4, sub: 0, line: 38 } |  |  | 0.695 |
| walker |  | 2947 | 54 | Code::CodeKey { rung: Names, file: packages/d2mini/src/d2.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.695 |
| walker |  | 2973 | 26 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/d2.ts, decl: 37, sub: 0, line: 146 } |  |  | 0.695 |
| ns | 2982 |  | 96 | Example and benchmark source trees | 2.13 | 1.3 | 0.670 |
| walker |  | 3006 | 33 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/operators/debug.ts, decl: 4, sub: 0, line: 42 } |  |  | 0.670 |
| walker |  | 3039 | 33 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/debug.ts, decl: 4, sub: 0, line: 74 } |  |  | 0.670 |
| walker |  | 3063 | 24 | Fs::DirListing { dir: examples/electric } |  |  | 0.677 |
| walker |  | 3071 | 8 | Fs::DirListing { dir: examples/electric/src } |  |  | 0.680 |
| walker |  | 3126 | 55 | Code::CodeKey { rung: Names, file: packages/d2ts/src/order.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.680 |
| walker |  | 3143 | 17 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/order.ts, decl: 33, sub: 0, line: 277 } |  |  | 0.680 |
| ns | 3172 |  | 190 | Per-package scripts: build, test, lint, typecheck, format | 3.1 | 1.7 | 0.670 |
| ns | 3259 |  | 87 | d2ts-benchmark scripts — the benchmark entry points | 3.2 | 3.1 | 0.664 |
| walker |  | 3318 | 175 | Json::Entry { file: packages/d2ts/package.json } |  |  | 0.664 |
| walker |  | 3374 | 56 | Code::CodeKey { rung: Names, file: packages/d2mini/src/multiset.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.664 |
| walker |  | 3430 | 56 | Code::CodeKey { rung: Names, file: packages/d2ts/src/multiset.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.664 |
| ns | 3452 |  | 193 | CI pipeline stages | 3.3 |  | 0.642 |
| walker |  | 3466 | 36 | Code::CodeKey { rung: Names, file: packages/d2mini/src/operators/count.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.642 |
| walker |  | 3480 | 14 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/operators/count.ts, decl: 1, sub: 0, line: 9 } |  |  | 0.642 |
| walker |  | 3516 | 36 | Code::CodeKey { rung: Names, file: packages/d2ts/src/operators/count.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.642 |
| walker |  | 3530 | 14 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/count.ts, decl: 1, sub: 0, line: 10 } |  |  | 0.642 |
| walker |  | 3567 | 37 | Code::CodeKey { rung: Names, file: packages/d2ts/src/operators/distinct.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.642 |
| walker |  | 3581 | 14 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/distinct.ts, decl: 1, sub: 0, line: 10 } |  |  | 0.642 |
| walker |  | 3622 | 41 | Fs::DirListing { dir: packages/d2ts/tests } |  |  | 0.651 |
| ns | 3656 |  | 204 | README: constructing a D2 graph and sending to an input stream | 4.1 | 1.10 | 0.643 |
| walker |  | 3810 | 188 | Json::Scripts { file: packages/d2ts/package.json } |  |  | 0.659 |
| walker |  | 3850 | 40 | Code::CodeKey { rung: Names, file: packages/d2ts/src/operators/buffer.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.659 |
| walker |  | 3868 | 18 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/buffer.ts, decl: 1, sub: 0, line: 17 } |  |  | 0.659 |
| walker |  | 3925 | 57 | Fs::DirListing { dir: packages/d2ts/src/sqlite/operators } |  |  | 0.687 |
| walker |  | 3966 | 41 | Code::CodeKey { rung: Names, file: packages/d2mini/src/operators/concat.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.687 |
| ns | 3981 |  | 325 | d2.ts — D2Options and the complete D2 class method roster | 4.2 |  | 0.664 |
| walker |  | 3982 | 16 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/operators/concat.ts, decl: 1, sub: 0, line: 9 } |  |  | 0.664 |
| walker |  | 4011 | 29 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/operators/concat.ts, decl: 3, sub: 0, line: 25 } |  |  | 0.664 |
| walker |  | 4052 | 41 | Code::CodeKey { rung: Names, file: packages/d2mini/src/operators/consolidate.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.664 |
| walker |  | 4068 | 16 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/operators/consolidate.ts, decl: 1, sub: 0, line: 9 } |  |  | 0.664 |
| walker |  | 4109 | 41 | Code::CodeKey { rung: Names, file: packages/d2ts/src/operators/concat.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.664 |
| walker |  | 4125 | 16 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/concat.ts, decl: 1, sub: 0, line: 11 } |  |  | 0.664 |
| walker |  | 4154 | 29 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/concat.ts, decl: 3, sub: 0, line: 54 } |  |  | 0.664 |
| ns | 4171 |  | 190 | README: what versions and frontiers actually mean | 4.3 | 1.10 | 0.661 |
| walker |  | 4195 | 41 | Code::CodeKey { rung: Names, file: packages/d2ts/src/operators/consolidate.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.661 |
| walker |  | 4213 | 18 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/consolidate.ts, decl: 1, sub: 0, line: 16 } |  |  | 0.661 |
| walker |  | 4254 | 41 | Code::CodeKey { rung: Names, file: packages/d2ts/src/operators/topKWithFractionalIndex.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.661 |
| walker |  | 4321 | 67 | Code::CodeKey { rung: Names, file: packages/d2ts/src/d2.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.661 |
| walker |  | 4343 | 22 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/d2.ts, decl: 1, sub: 0, line: 11 } |  |  | 0.662 |
| walker |  | 4381 | 38 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/d2.ts, decl: 41, sub: 0, line: 166 } |  |  | 0.662 |
| walker |  | 4424 | 43 | Code::CodeKey { rung: Names, file: packages/d2ts/src/operators/reduce.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.662 |
| walker |  | 4451 | 27 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/reduce.ts, decl: 1, sub: 0, line: 15 } |  |  | 0.662 |
| walker |  | 4495 | 44 | Code::CodeKey { rung: Names, file: packages/d2mini/src/operators/negate.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.662 |
| ns | 4517 |  | 346 | order.ts — v() factory and the complete Version method roster | 4.4 |  | 0.638 |
| walker |  | 4519 | 24 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/operators/negate.ts, decl: 1, sub: 0, line: 10 } |  |  | 0.638 |
| walker |  | 4563 | 44 | Code::CodeKey { rung: Names, file: packages/d2ts/src/operators/negate.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.639 |
| walker |  | 4587 | 24 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/negate.ts, decl: 1, sub: 0, line: 10 } |  |  | 0.639 |
| walker |  | 4633 | 46 | Code::CodeKey { rung: Names, file: packages/d2mini/src/operators/reduce.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.639 |
| walker |  | 4660 | 27 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/operators/reduce.ts, decl: 1, sub: 0, line: 14 } |  |  | 0.639 |
| walker |  | 4692 | 32 | Fs::DirListing { dir: examples/d2ql } |  |  | 0.653 |
| walker |  | 4730 | 38 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/d2.ts, decl: 42, sub: 0, line: 167 } |  |  | 0.653 |
| walker |  | 4780 | 50 | Code::CodeKey { rung: Names, file: packages/d2ts/src/operators/output.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.654 |
| walker |  | 4807 | 27 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/output.ts, decl: 1, sub: 0, line: 19 } |  |  | 0.635 |
| ns | 4807 |  | 290 | order.ts — Antichain method roster and Frontier | 4.5 | 4.4 | 0.635 |
| walker |  | 4858 | 51 | Code::CodeKey { rung: Names, file: packages/d2mini/src/operators/distinct.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.635 |
| walker |  | 4885 | 27 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/operators/distinct.ts, decl: 1, sub: 0, line: 17 } |  |  | 0.635 |
| walker |  | 4937 | 52 | Code::CodeKey { rung: Names, file: packages/d2mini/src/operators/filter.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.635 |
| ns | 4952 |  | 145 | README: MultiSet as a changeset, and keyed multisets | 4.6 | 1.10 | 0.632 |
| walker |  | 4972 | 35 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/operators/filter.ts, decl: 1, sub: 0, line: 10 } |  |  | 0.632 |
| walker |  | 5024 | 52 | Code::CodeKey { rung: Names, file: packages/d2ts/src/operators/filter.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.632 |
| walker |  | 5059 | 35 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/filter.ts, decl: 1, sub: 0, line: 11 } |  |  | 0.632 |
| walker |  | 5109 | 50 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/topKWithFractionalIndex.ts, decl: 1, sub: 0, line: 29 } |  |  | 0.632 |
| walker |  | 5162 | 53 | Code::CodeKey { rung: Names, file: packages/d2ts/src/operators/orderBy.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.632 |
| walker |  | 5218 | 56 | Code::CodeKey { rung: Names, file: packages/d2mini/src/operators/map.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.632 |
| walker |  | 5253 | 35 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/operators/map.ts, decl: 1, sub: 0, line: 10 } |  |  | 0.632 |
| walker |  | 5309 | 56 | Code::CodeKey { rung: Names, file: packages/d2ts/src/operators/map.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.633 |
| ns | 5310 |  | 358 | multiset.ts — MultiSetArray, KeyedData and every MultiSet method | 4.7 |  | 0.617 |
| walker |  | 5344 | 35 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/map.ts, decl: 1, sub: 0, line: 11 } |  |  | 0.617 |
| walker |  | 5459 | 115 | Fs::DirListing { dir: packages/d2ql/tests } |  |  | 0.659 |
| ns | 5610 |  | 300 | types.ts — the operator message protocol | 4.8 |  | 0.637 |
| walker |  | 5817 | 358 | Code::CodeKey { rung: Decl, file: eslint.base.mjs, decl: 1, sub: 0, line: 6 } |  |  | 0.637 |
| walker |  | 5858 | 41 | Fs::DirListing { dir: packages/d2ts/tests/operators-sqlite } |  |  | 0.637 |
| ns | 5868 |  | 258 | types.ts — PipedOperator and the 20-deep pipe overload wall | 4.9 | 4.8 | 0.625 |
| walker |  | 5871 | 13 | Code::CodeKey { rung: Names, file: packages/d2ql/src/select.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.625 |
| walker |  | 6146 | 275 | Code::CodeKey { rung: Names, file: packages/d2ql/src/schema.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.625 |
| walker |  | 6161 | 15 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/schema.ts, decl: 7, sub: 0, line: 48 } |  |  | 0.625 |
| walker |  | 6177 | 16 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/schema.ts, decl: 5, sub: 0, line: 36 } |  |  | 0.625 |
| walker |  | 6211 | 34 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/schema.ts, decl: 18, sub: 0, line: 152 } |  |  | 0.625 |
| ns | 6251 |  | 383 | README Key Features — every operator with a one-line description | 5.1 | 1.10 | 0.634 |
| walker |  | 6255 | 44 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/schema.ts, decl: 15, sub: 0, line: 123 } |  |  | 0.634 |
| walker |  | 6301 | 46 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/schema.ts, decl: 9, sub: 0, line: 65 } |  |  | 0.634 |
| walker |  | 6349 | 48 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/schema.ts, decl: 11, sub: 0, line: 80 } |  |  | 0.634 |
| walker |  | 6399 | 50 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/schema.ts, decl: 3, sub: 0, line: 24 } |  |  | 0.634 |
| walker |  | 6453 | 54 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/schema.ts, decl: 6, sub: 0, line: 39 } |  |  | 0.634 |
| walker |  | 6515 | 62 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/schema.ts, decl: 10, sub: 0, line: 71 } |  |  | 0.634 |
| walker |  | 6579 | 64 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/schema.ts, decl: 2, sub: 0, line: 13 } |  |  | 0.634 |
| walker |  | 6656 | 77 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/schema.ts, decl: 17, sub: 0, line: 143 } |  |  | 0.634 |
| ns | 6681 |  | 430 | packages/d2ts/README.md — complete heading map (1081-line API reference) | 5.2 |  | 0.610 |
| walker |  | 6735 | 79 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/schema.ts, decl: 12, sub: 0, line: 94 } |  |  | 0.610 |
| walker |  | 6817 | 82 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/schema.ts, decl: 16, sub: 0, line: 130 } |  |  | 0.610 |
| ns | 6844 |  | 163 | Operator factory roster — single-stream operators | 5.3 | 2.5 | 0.614 |
| walker |  | 6875 | 58 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/operators/count.ts, decl: 3, sub: 0, line: 30 } |  |  | 0.614 |
| walker |  | 6933 | 58 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/count.ts, decl: 3, sub: 0, line: 32 } |  |  | 0.614 |
| walker |  | 6991 | 58 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/distinct.ts, decl: 3, sub: 0, line: 37 } |  |  | 0.614 |
| walker |  | 6999 | 8 | Code::CodeKey { rung: Body, file: packages/d2ts/src/order.ts, decl: 34, sub: 0, line: 278 } |  |  | 0.614 |
| walker |  | 7058 | 59 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/orderBy.ts, decl: 1, sub: 0, line: 23 } |  |  | 0.614 |
| walker |  | 7151 | 93 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/schema.ts, decl: 8, sub: 0, line: 53 } |  |  | 0.614 |
| walker |  | 7165 | 14 | Code::CodeKey { rung: Names, file: packages/d2ql/src/key-by.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.614 |
| walker |  | 7179 | 14 | Code::CodeKey { rung: Names, file: packages/d2ql/src/order-by.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.614 |
| ns | 7244 |  | 400 | Operator factory roster — keyed, join, ordering and aggregate families | 5.4 | 5.3 | 0.598 |
| walker |  | 7248 | 69 | Code::CodeKey { rung: Names, file: packages/d2ts/src/operators/iterate.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.600 |
| walker |  | 7264 | 16 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/iterate.ts, decl: 1, sub: 0, line: 15 } |  |  | 0.600 |
| walker |  | 7280 | 16 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/iterate.ts, decl: 3, sub: 0, line: 46 } |  |  | 0.600 |
| walker |  | 7307 | 27 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/iterate.ts, decl: 5, sub: 0, line: 80 } |  |  | 0.600 |
| walker |  | 7334 | 27 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/iterate.ts, decl: 8, sub: 0, line: 225 } |  |  | 0.600 |
| walker |  | 7445 | 111 | Code::CodeKey { rung: Names, file: packages/d2mini/src/types.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.600 |
| ns | 7446 |  | 202 | Operator implementation classes and the LinearUnaryOperator base | 5.5 | 5.4 | 0.605 |
| walker |  | 7466 | 21 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/types.ts, decl: 7, sub: 0, line: 83 } |  |  | 0.605 |
| walker |  | 7489 | 23 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/types.ts, decl: 2, sub: 0, line: 6 } |  |  | 0.605 |
| walker |  | 7514 | 25 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/types.ts, decl: 3, sub: 0, line: 11 } |  |  | 0.605 |
| walker |  | 7552 | 38 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/types.ts, decl: 4, sub: 0, line: 16 } |  |  | 0.605 |
| ns | 7622 |  | 176 | groupBy.ts — the AggregateFunction contract | 5.6 | 5.4 | 0.598 |
| walker |  | 7631 | 79 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/types.ts, decl: 5, sub: 0, line: 21 } |  |  | 0.598 |
| walker |  | 7739 | 108 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/schema.ts, decl: 13, sub: 0, line: 105 } |  |  | 0.598 |
| walker |  | 7755 | 16 | Code::CodeKey { rung: Names, file: packages/d2mini/src/indexes.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.598 |
| walker |  | 7809 | 54 | Fs::DirListing { dir: packages/d2ql/tests/query-builder } |  |  | 0.598 |
| ns | 7842 |  | 220 | D2QL identity and README section map | 6.1 |  | 0.593 |
| walker |  | 7890 | 81 | Code::CodeKey { rung: Names, file: packages/d2mini/src/operators/keying.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.593 |
| walker |  | 7923 | 33 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/operators/keying.ts, decl: 2, sub: 0, line: 13 } |  |  | 0.593 |
| walker |  | 7963 | 40 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/operators/keying.ts, decl: 4, sub: 0, line: 30 } |  |  | 0.593 |
| walker |  | 8044 | 81 | Code::CodeKey { rung: Names, file: packages/d2ts/src/operators/keying.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.596 |
| walker |  | 8077 | 33 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/keying.ts, decl: 2, sub: 0, line: 13 } |  |  | 0.596 |
| ns | 8084 |  | 242 | D2QL "Current Features" — the supported SQL subset | 6.2 | 6.1 | 0.588 |
| walker |  | 8117 | 40 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/keying.ts, decl: 4, sub: 0, line: 30 } |  |  | 0.588 |
| walker |  | 8195 | 78 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/operators/filterBy.ts, decl: 1, sub: 0, line: 14 } |  |  | 0.588 |
| walker |  | 8273 | 78 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/filterBy.ts, decl: 1, sub: 0, line: 14 } |  |  | 0.588 |
| walker |  | 8291 | 18 | Code::CodeKey { rung: Names, file: packages/d2ts-benchmark/src/graph.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.588 |
| walker |  | 8321 | 30 | Markdown::Section { file: packages/d2ql/README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.588 |
| walker |  | 8405 | 84 | Code::CodeKey { rung: Names, file: packages/d2mini/src/operators/orderBy.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.588 |
| ns | 8442 |  | 358 | schema.ts — the Query interface family | 6.3 |  | 0.576 |
| walker |  | 8447 | 42 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/operators/orderBy.ts, decl: 1, sub: 0, line: 9 } |  |  | 0.576 |
| walker |  | 8506 | 59 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/operators/orderBy.ts, decl: 2, sub: 0, line: 23 } |  |  | 0.576 |
| walker |  | 8517 | 11 | Code::CodeKey { rung: Body, file: packages/d2mini/src/d2.ts, decl: 38, sub: 0, line: 147 } |  |  | 0.576 |
| ns | 8604 |  | 162 | schema.ts — the Comparator and LogicalOperator vocabularies | 6.4 | 6.3 | 0.578 |
| walker |  | 8648 | 131 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/d2.ts, decl: 1, sub: 0, line: 10 } |  |  | 0.578 |
| walker |  | 8733 | 85 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/operators/reduce.ts, decl: 4, sub: 0, line: 106 } |  |  | 0.578 |
| ns | 8735 |  | 131 | compileQuery — the package's single entry point | 6.5 |  | 0.575 |
| walker |  | 8818 | 85 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/reduce.ts, decl: 4, sub: 0, line: 127 } |  |  | 0.575 |
| walker |  | 8905 | 87 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/operators/orderBy.ts, decl: 3, sub: 0, line: 76 } |  |  | 0.575 |
| ns | 8978 |  | 243 | query-builder.ts — the fluent builder's complete method set | 6.6 |  | 0.566 |
| walker |  | 8992 | 87 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/operators/orderBy.ts, decl: 5, sub: 0, line: 199 } |  |  | 0.566 |
| ns | 9076 |  | 98 | d2mini README — how it differs from d2ts | 7.1 |  | 0.567 |
| walker |  | 9079 | 87 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/orderBy.ts, decl: 2, sub: 0, line: 76 } |  |  | 0.567 |
| ns | 9165 |  | 89 | d2mini/src/d2.ts — the versionless D2 constructor | 7.2 | 7.1 | 0.564 |
| walker |  | 9166 | 87 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/orderBy.ts, decl: 3, sub: 0, line: 140 } |  |  | 0.564 |
| walker |  | 9305 | 139 | Fs::DirListing { dir: packages/d2mini/tests/operators } |  |  | 0.564 |
| walker |  | 9354 | 49 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/operators/count.ts, decl: 2, sub: 0, line: 10 } |  |  | 0.564 |
| walker |  | 9367 | 13 | Code::CodeKey { rung: Body, file: packages/d2ts/src/d2.ts, decl: 42, sub: 0, line: 167 } |  |  | 0.564 |
| ns | 9401 |  | 236 | README: using the SQLite backend | 8.1 | 1.10 | 0.559 |
| ns | 9529 |  | 128 | sqlite/database.ts — the SQLiteDb driver interface | 8.2 | 8.1 | 0.554 |
| ns | 9683 |  | 154 | withSQLite, SQLIndex and the Electric bridge exports | 8.3 | 8.2 | 0.550 |
| walker |  | 9717 | 350 | Code::CodeKey { rung: Decl, file: eslint.base.mjs, decl: 1, sub: 1, line: 6 } |  |  | 0.550 |
| ns | 9827 |  | 144 | README: what each example demonstrates | 9.1 | 1.10 | 0.548 |
| walker |  | 9900 | 183 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.548 |
| walker |  | 9917 | 17 | Code::CodeKey { rung: Doc, file: packages/d2mini/src/operators/distinct.ts, decl: 4, sub: 0, line: 85 } |  |  | 0.548 |
| ns | 9977 |  | 150 | examples/electric — what the demo app does | 9.2 | 2.13 | 0.545 |

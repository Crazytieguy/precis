Score(3000)=0.698 I=0.857 C=0.569 ns_rows≤3K=24/55 grid(1000/1442/2080/3000/4327/6240/9000)=0.656/0.648/0.757/0.698/0.676/0.625/0.566

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 54 | 54 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 65 | 11 | Code::CodeKey { rung: Names, file: eslint.base.mjs, decl: 0, sub: 0, line: 0 } |  |  | 0.000 |
| walker |  | 68 | 3 | Fs::DirListing { dir: .github } |  |  | 0.000 |
| walker |  | 72 | 4 | Fs::DirListing { dir: .github/workflows } |  |  | 0.000 |
| ns | 81 |  | 81 | Repository identity — README title + one-line definition | 1.1 |  | 0.000 |
| walker |  | 94 | 22 | Fs::DirListing { dir: packages } |  |  | 0.000 |
| walker |  | 129 | 35 | Fs::DirListing { dir: examples } |  |  | 0.000 |
| ns | 135 |  | 54 | Complete repository root listing | 1.2 |  | 0.714 |
| walker |  | 174 | 45 | Json::Identity { file: package.json } |  |  | 0.716 |
| walker |  | 191 | 17 | Fs::DirListing { dir: packages/d2ts-benchmark } |  |  | 0.717 |
| ns | 192 |  | 57 | Workspace membership — packages/ and examples/ listings | 1.3 |  | 0.697 |
| walker |  | 203 | 12 | Fs::DirListing { dir: packages/d2ts-benchmark/src } |  |  | 0.697 |
| ns | 218 |  | 26 | pnpm workspace globs | 1.4 |  | 0.656 |
| walker |  | 241 | 38 | Code::CodeKey { rung: Names, file: packages/d2ts-benchmark/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.656 |
| walker |  | 249 | 8 | Fs::DirListing { dir: .changeset } |  |  | 0.656 |
| walker |  | 274 | 25 | Fs::DirListing { dir: packages/d2ql } |  |  | 0.660 |
| ns | 288 |  | 70 | README lede — what incremental execution buys you | 1.5 | 1.1 | 0.648 |
| walker |  | 304 | 30 | Fs::DirListing { dir: packages/d2mini } |  |  | 0.657 |
| walker |  | 385 | 81 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.936 |
| ns | 387 |  | 99 | README lede — ElectricSQL ShapeStreams and pipeline type inference | 1.6 | 1.1 | 0.887 |
| walker |  | 421 | 36 | Fs::DirListing { dir: packages/d2ts } |  |  | 0.898 |
| walker |  | 506 | 85 | Json::Identity { file: packages/d2ts/package.json } |  |  | 0.898 |
| ns | 578 |  | 191 | Root package.json — identity and every workspace script | 1.7 |  | 0.736 |
| walker |  | 580 | 74 | Json::Dependencies { file: packages/d2ts/package.json } |  |  | 0.736 |
| walker |  | 613 | 33 | Fs::DirListing { dir: packages/d2mini/src } |  |  | 0.737 |
| walker |  | 660 | 47 | Code::CodeKey { rung: Names, file: packages/d2mini/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.737 |
| ns | 728 |  | 150 | Published package identity: d2ts and d2mini | 1.8 |  | 0.694 |
| walker |  | 764 | 104 | Fs::DirListing { dir: packages/d2mini/src/operators } |  |  | 0.705 |
| walker |  | 820 | 56 | Code::CodeKey { rung: Decl, file: packages/d2ts-benchmark/src/index.ts, decl: 2, sub: 0, line: 25 } |  |  | 0.705 |
| walker |  | 864 | 44 | Fs::DirListing { dir: packages/d2ts/src } |  |  | 0.712 |
| walker |  | 868 | 4 | Fs::DirListing { dir: packages/d2ts/src/electric } |  |  | 0.712 |
| ns | 871 |  | 143 | Private package identity: d2ql and d2ts-benchmark | 1.9 | 1.8 | 0.653 |
| walker |  | 888 | 20 | Fs::DirListing { dir: packages/d2ts/src/sqlite } |  |  | 0.653 |
| walker |  | 958 | 70 | Code::CodeKey { rung: Names, file: packages/d2ts/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.656 |
| ns | 1032 |  | 161 | Root README section map (every H2) | 1.10 |  | 0.578 |
| walker |  | 1056 | 98 | Fs::DirListing { dir: packages/d2ts/src/operators } |  |  | 0.587 |
| walker |  | 1069 | 13 | Code::CodeKey { rung: Names, file: packages/d2mini/src/operators/filterBy.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.587 |
| walker |  | 1082 | 13 | Code::CodeKey { rung: Names, file: packages/d2ts/src/operators/filterBy.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.587 |
| walker |  | 1169 | 87 | Code::CodeKey { rung: Decl, file: packages/d2ts-benchmark/src/index.ts, decl: 3, sub: 0, line: 496 } |  |  | 0.587 |
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
| walker |  | 1630 | 24 | Fs::DirListing { dir: examples/electric } |  |  | 0.692 |
| walker |  | 1638 | 8 | Fs::DirListing { dir: examples/electric/src } |  |  | 0.693 |
| walker |  | 1677 | 39 | Markdown::Section { file: packages/d2mini/README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.693 |
| ns | 1844 |  | 240 | packages/d2ts/src/operators/index.ts — exported operator set | 2.5 | 2.4 | 0.648 |
| walker |  | 1850 | 173 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.716 |
| ns | 1981 |  | 137 | packages/d2mini/src and its operators directory | 2.6 |  | 0.738 |
| walker |  | 2022 | 172 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.757 |
| walker |  | 2057 | 35 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/compiler.ts, decl: 1, sub: 0, line: 16 } |  |  | 0.757 |
| walker |  | 2095 | 38 | Code::CodeKey { rung: Names, file: packages/d2ts/src/version-index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.757 |
| walker |  | 2121 | 26 | Plaintext::Whole { file: pnpm-workspace.yaml } |  |  | 0.773 |
| walker |  | 2147 | 26 | Code::CodeKey { rung: Names, file: packages/d2mini/src/operators/topK.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.773 |
| walker |  | 2173 | 26 | Code::CodeKey { rung: Names, file: packages/d2ts/src/operators/topK.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.773 |
| walker |  | 2202 | 29 | Fs::DirListing { dir: packages/d2mini/tests } |  |  | 0.773 |
| ns | 2246 |  | 265 | packages/d2mini export barrels | 2.7 | 2.6 | 0.732 |
| ns | 2324 |  | 78 | packages/d2ql/src and query-builder/ listings | 2.8 |  | 0.743 |
| walker |  | 2338 | 136 | Json::Scripts { file: package.json } |  |  | 0.788 |
| walker |  | 2370 | 32 | Fs::DirListing { dir: examples/d2ql } |  |  | 0.790 |
| walker |  | 2401 | 31 | Code::CodeKey { rung: Names, file: packages/d2mini/src/operators/debug.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.790 |
| walker |  | 2428 | 27 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/operators/debug.ts, decl: 1, sub: 0, line: 12 } |  |  | 0.790 |
| ns | 2441 |  | 117 | packages/d2ql export barrels | 2.9 | 2.8 | 0.786 |
| walker |  | 2459 | 31 | Code::CodeKey { rung: Names, file: packages/d2mini/src/operators/output.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.786 |
| walker |  | 2486 | 27 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/operators/output.ts, decl: 1, sub: 0, line: 13 } |  |  | 0.786 |
| walker |  | 2517 | 31 | Code::CodeKey { rung: Names, file: packages/d2ts/src/operators/debug.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.786 |
| ns | 2518 |  | 77 | packages/d2ts/src/sqlite — complete tree listing | 2.10 |  | 0.761 |
| walker |  | 2544 | 27 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/debug.ts, decl: 1, sub: 0, line: 18 } |  |  | 0.761 |
| ns | 2701 |  | 183 | SQLite subpath export barrels | 2.11 | 2.10 | 0.738 |
| ns | 2886 |  | 185 | Test suite layout — immediate children of every tests/ directory | 2.12 |  | 0.695 |
| walker |  | 2927 | 383 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: true } |  |  | 0.697 |
| walker |  | 2957 | 30 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/operators/output.ts, decl: 4, sub: 0, line: 38 } |  |  | 0.697 |
| ns | 2982 |  | 96 | Example and benchmark source trees | 2.13 | 1.3 | 0.698 |
| walker |  | 3011 | 54 | Code::CodeKey { rung: Names, file: packages/d2mini/src/d2.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.698 |
| walker |  | 3037 | 26 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/d2.ts, decl: 37, sub: 0, line: 146 } |  |  | 0.698 |
| walker |  | 3070 | 33 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/operators/debug.ts, decl: 4, sub: 0, line: 42 } |  |  | 0.698 |
| walker |  | 3103 | 33 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/debug.ts, decl: 4, sub: 0, line: 74 } |  |  | 0.698 |
| walker |  | 3158 | 55 | Code::CodeKey { rung: Names, file: packages/d2ts/src/order.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.698 |
| ns | 3172 |  | 190 | Per-package scripts: build, test, lint, typecheck, format | 3.1 | 1.7 | 0.688 |
| walker |  | 3175 | 17 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/order.ts, decl: 33, sub: 0, line: 277 } |  |  | 0.688 |
| ns | 3259 |  | 87 | d2ts-benchmark scripts — the benchmark entry points | 3.2 | 3.1 | 0.682 |
| walker |  | 3350 | 175 | Json::Entry { file: packages/d2ts/package.json } |  |  | 0.682 |
| walker |  | 3406 | 56 | Code::CodeKey { rung: Names, file: packages/d2mini/src/multiset.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.682 |
| ns | 3452 |  | 193 | CI pipeline stages | 3.3 |  | 0.659 |
| walker |  | 3462 | 56 | Code::CodeKey { rung: Names, file: packages/d2ts/src/multiset.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.659 |
| walker |  | 3498 | 36 | Code::CodeKey { rung: Names, file: packages/d2mini/src/operators/count.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.659 |
| walker |  | 3512 | 14 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/operators/count.ts, decl: 1, sub: 0, line: 9 } |  |  | 0.659 |
| walker |  | 3548 | 36 | Code::CodeKey { rung: Names, file: packages/d2ts/src/operators/count.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.659 |
| walker |  | 3562 | 14 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/count.ts, decl: 1, sub: 0, line: 10 } |  |  | 0.659 |
| walker |  | 3599 | 37 | Code::CodeKey { rung: Names, file: packages/d2ts/src/operators/distinct.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.659 |
| walker |  | 3613 | 14 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/distinct.ts, decl: 1, sub: 0, line: 10 } |  |  | 0.659 |
| walker |  | 3654 | 41 | Fs::DirListing { dir: packages/d2ts/tests } |  |  | 0.668 |
| ns | 3656 |  | 204 | README: constructing a D2 graph and sending to an input stream | 4.1 | 1.10 | 0.659 |
| walker |  | 3842 | 188 | Json::Scripts { file: packages/d2ts/package.json } |  |  | 0.675 |
| walker |  | 3882 | 40 | Code::CodeKey { rung: Names, file: packages/d2ts/src/operators/buffer.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.675 |
| walker |  | 3900 | 18 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/buffer.ts, decl: 1, sub: 0, line: 17 } |  |  | 0.675 |
| walker |  | 3957 | 57 | Fs::DirListing { dir: packages/d2ts/src/sqlite/operators } |  |  | 0.703 |
| ns | 3981 |  | 325 | d2.ts — D2Options and the complete D2 class method roster | 4.2 |  | 0.679 |
| walker |  | 3998 | 41 | Code::CodeKey { rung: Names, file: packages/d2mini/src/operators/concat.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.679 |
| walker |  | 4014 | 16 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/operators/concat.ts, decl: 1, sub: 0, line: 9 } |  |  | 0.679 |
| walker |  | 4043 | 29 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/operators/concat.ts, decl: 3, sub: 0, line: 25 } |  |  | 0.679 |
| walker |  | 4084 | 41 | Code::CodeKey { rung: Names, file: packages/d2mini/src/operators/consolidate.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.679 |
| walker |  | 4100 | 16 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/operators/consolidate.ts, decl: 1, sub: 0, line: 9 } |  |  | 0.679 |
| walker |  | 4141 | 41 | Code::CodeKey { rung: Names, file: packages/d2ts/src/operators/concat.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.680 |
| walker |  | 4157 | 16 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/concat.ts, decl: 1, sub: 0, line: 11 } |  |  | 0.680 |
| ns | 4171 |  | 190 | README: what versions and frontiers actually mean | 4.3 | 1.10 | 0.676 |
| walker |  | 4186 | 29 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/concat.ts, decl: 3, sub: 0, line: 54 } |  |  | 0.676 |
| walker |  | 4227 | 41 | Code::CodeKey { rung: Names, file: packages/d2ts/src/operators/consolidate.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.676 |
| walker |  | 4245 | 18 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/consolidate.ts, decl: 1, sub: 0, line: 16 } |  |  | 0.676 |
| walker |  | 4286 | 41 | Code::CodeKey { rung: Names, file: packages/d2ts/src/operators/topKWithFractionalIndex.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.676 |
| walker |  | 4353 | 67 | Code::CodeKey { rung: Names, file: packages/d2ts/src/d2.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.676 |
| walker |  | 4375 | 22 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/d2.ts, decl: 1, sub: 0, line: 11 } |  |  | 0.677 |
| walker |  | 4413 | 38 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/d2.ts, decl: 41, sub: 0, line: 166 } |  |  | 0.677 |
| walker |  | 4456 | 43 | Code::CodeKey { rung: Names, file: packages/d2ts/src/operators/reduce.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.677 |
| walker |  | 4483 | 27 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/reduce.ts, decl: 1, sub: 0, line: 15 } |  |  | 0.677 |
| ns | 4517 |  | 346 | order.ts — v() factory and the complete Version method roster | 4.4 |  | 0.653 |
| walker |  | 4527 | 44 | Code::CodeKey { rung: Names, file: packages/d2mini/src/operators/negate.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.653 |
| walker |  | 4551 | 24 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/operators/negate.ts, decl: 1, sub: 0, line: 10 } |  |  | 0.653 |
| walker |  | 4595 | 44 | Code::CodeKey { rung: Names, file: packages/d2ts/src/operators/negate.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.653 |
| walker |  | 4619 | 24 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/negate.ts, decl: 1, sub: 0, line: 10 } |  |  | 0.653 |
| walker |  | 4665 | 46 | Code::CodeKey { rung: Names, file: packages/d2mini/src/operators/reduce.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.653 |
| walker |  | 4692 | 27 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/operators/reduce.ts, decl: 1, sub: 0, line: 14 } |  |  | 0.653 |
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

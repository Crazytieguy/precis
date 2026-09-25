Score(3000)=0.699 I=0.861 C=0.569 ns_rows≤3K=24/55 grid(1000/1442/2080/3000/4327/6240/9000)=0.511/0.648/0.772/0.699/0.721/0.622/0.563

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 54 | 54 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 57 | 3 | Fs::DirListing { dir: .github } |  |  | 0.000 |
| walker |  | 61 | 4 | Fs::DirListing { dir: .github/workflows } |  |  | 0.000 |
| ns | 81 |  | 81 | Repository identity — README title + one-line definition | 1.1 |  | 0.000 |
| walker |  | 83 | 22 | Fs::DirListing { dir: packages } |  |  | 0.000 |
| walker |  | 128 | 45 | Json::Identity { file: package.json } |  |  | 0.000 |
| ns | 135 |  | 54 | Complete repository root listing | 1.2 |  | 0.636 |
| walker |  | 145 | 17 | Fs::DirListing { dir: packages/d2ts-benchmark } |  |  | 0.637 |
| walker |  | 156 | 11 | Code::CodeKey { rung: Names, file: eslint.base.mjs, decl: 0, sub: 0, line: 0 } |  |  | 0.637 |
| walker |  | 168 | 12 | Fs::DirListing { dir: packages/d2ts-benchmark/src } |  |  | 0.637 |
| walker |  | 176 | 8 | Fs::DirListing { dir: .changeset } |  |  | 0.637 |
| ns | 192 |  | 57 | Workspace membership — packages/ and examples/ listings | 1.3 |  | 0.476 |
| walker |  | 201 | 25 | Fs::DirListing { dir: packages/d2ql } |  |  | 0.480 |
| ns | 218 |  | 26 | pnpm workspace globs | 1.4 |  | 0.451 |
| walker |  | 231 | 30 | Fs::DirListing { dir: packages/d2mini } |  |  | 0.459 |
| ns | 288 |  | 70 | README lede — what incremental execution buys you | 1.5 | 1.1 | 0.451 |
| walker |  | 312 | 81 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.701 |
| walker |  | 348 | 36 | Fs::DirListing { dir: packages/d2ts } |  |  | 0.713 |
| ns | 387 |  | 99 | README lede — ElectricSQL ShapeStreams and pipeline type inference | 1.6 | 1.1 | 0.676 |
| walker |  | 433 | 85 | Json::Identity { file: packages/d2ts/package.json } |  |  | 0.678 |
| walker |  | 471 | 38 | Code::CodeKey { rung: Names, file: packages/d2ts-benchmark/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.678 |
| walker |  | 545 | 74 | Json::Dependencies { file: packages/d2ts/package.json } |  |  | 0.678 |
| walker |  | 578 | 33 | Fs::DirListing { dir: packages/d2mini/src } |  |  | 0.557 |
| ns | 578 |  | 191 | Root package.json — identity and every workspace script | 1.7 |  | 0.557 |
| walker |  | 682 | 104 | Fs::DirListing { dir: packages/d2mini/src/operators } |  |  | 0.567 |
| ns | 728 |  | 150 | Published package identity: d2ts and d2mini | 1.8 |  | 0.541 |
| walker |  | 729 | 47 | Code::CodeKey { rung: Names, file: packages/d2mini/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.542 |
| walker |  | 785 | 56 | Code::CodeKey { rung: Decl, file: packages/d2ts-benchmark/src/index.ts, decl: 2, sub: 0, line: 25 } |  |  | 0.542 |
| walker |  | 829 | 44 | Fs::DirListing { dir: packages/d2ts/src } |  |  | 0.547 |
| walker |  | 833 | 4 | Fs::DirListing { dir: packages/d2ts/src/electric } |  |  | 0.547 |
| walker |  | 853 | 20 | Fs::DirListing { dir: packages/d2ts/src/sqlite } |  |  | 0.548 |
| ns | 871 |  | 143 | Private package identity: d2ql and d2ts-benchmark | 1.9 | 1.8 | 0.503 |
| walker |  | 951 | 98 | Fs::DirListing { dir: packages/d2ts/src/operators } |  |  | 0.511 |
| walker |  | 1021 | 70 | Code::CodeKey { rung: Names, file: packages/d2ts/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.514 |
| ns | 1032 |  | 161 | Root README section map (every H2) | 1.10 |  | 0.453 |
| walker |  | 1108 | 87 | Code::CodeKey { rung: Decl, file: packages/d2ts-benchmark/src/index.ts, decl: 3, sub: 0, line: 496 } |  |  | 0.453 |
| walker |  | 1143 | 35 | Fs::DirListing { dir: examples } |  |  | 0.587 |
| walker |  | 1202 | 59 | Markdown::ReadmeHeadline { file: packages/d2mini/README.md } |  |  | 0.587 |
| walker |  | 1271 | 69 | Fs::DirListing { dir: packages/d2ql/src } |  |  | 0.591 |
| walker |  | 1280 | 9 | Fs::DirListing { dir: packages/d2ql/src/query-builder } |  |  | 0.592 |
| ns | 1284 |  | 252 | README "Implementation Details" — provenance and the four core data structures | 1.11 | 1.10 | 0.527 |
| walker |  | 1337 | 57 | Code::CodeKey { rung: ModuleDoc, file: packages/d2ql/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.527 |
| walker |  | 1371 | 34 | Code::CodeKey { rung: Names, file: packages/d2ql/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.528 |
| ns | 1392 |  | 108 | Every package's own root listing | 2.1 |  | 0.622 |
| ns | 1436 |  | 44 | packages/d2ts/src — complete module roster | 2.2 |  | 0.648 |
| walker |  | 1448 | 77 | Markdown::ReadmeHeadline { file: packages/d2ql/README.md } |  |  | 0.648 |
| walker |  | 1481 | 33 | Markdown::Section { file: packages/d2ql/README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.648 |
| ns | 1506 |  | 70 | packages/d2ts/src/index.ts — the public export barrel | 2.3 |  | 0.659 |
| walker |  | 1559 | 78 | Markdown::ReadmeHeadline { file: packages/d2ts/README.md } |  |  | 0.659 |
| walker |  | 1598 | 39 | Markdown::Section { file: packages/d2mini/README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.659 |
| ns | 1604 |  | 98 | packages/d2ts/src/operators — complete operator file roster | 2.4 |  | 0.692 |
| walker |  | 1771 | 173 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.764 |
| ns | 1844 |  | 240 | packages/d2ts/src/operators/index.ts — exported operator set | 2.5 | 2.4 | 0.715 |
| walker |  | 1943 | 172 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.736 |
| walker |  | 1967 | 24 | Fs::DirListing { dir: examples/electric } |  |  | 0.736 |
| walker |  | 1975 | 8 | Fs::DirListing { dir: examples/electric/src } |  |  | 0.737 |
| ns | 1981 |  | 137 | packages/d2mini/src and its operators directory | 2.6 |  | 0.757 |
| walker |  | 2001 | 26 | Plaintext::Whole { file: pnpm-workspace.yaml } |  |  | 0.772 |
| walker |  | 2137 | 136 | Json::Scripts { file: package.json } |  |  | 0.823 |
| walker |  | 2166 | 29 | Fs::DirListing { dir: packages/d2mini/tests } |  |  | 0.823 |
| ns | 2246 |  | 265 | packages/d2mini export barrels | 2.7 | 2.6 | 0.780 |
| ns | 2324 |  | 78 | packages/d2ql/src and query-builder/ listings | 2.8 |  | 0.788 |
| ns | 2441 |  | 117 | packages/d2ql export barrels | 2.9 | 2.8 | 0.784 |
| ns | 2518 |  | 77 | packages/d2ts/src/sqlite — complete tree listing | 2.10 |  | 0.759 |
| walker |  | 2549 | 383 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: true } |  |  | 0.761 |
| walker |  | 2581 | 32 | Fs::DirListing { dir: examples/d2ql } |  |  | 0.763 |
| ns | 2701 |  | 183 | SQLite subpath export barrels | 2.11 | 2.10 | 0.740 |
| walker |  | 2756 | 175 | Json::Entry { file: packages/d2ts/package.json } |  |  | 0.740 |
| ns | 2886 |  | 185 | Test suite layout — immediate children of every tests/ directory | 2.12 |  | 0.697 |
| walker |  | 2944 | 188 | Json::Scripts { file: packages/d2ts/package.json } |  |  | 0.699 |
| ns | 2982 |  | 96 | Example and benchmark source trees | 2.13 | 1.3 | 0.699 |
| walker |  | 3001 | 57 | Fs::DirListing { dir: packages/d2ts/src/sqlite/operators } |  |  | 0.730 |
| walker |  | 3042 | 41 | Fs::DirListing { dir: packages/d2ts/tests } |  |  | 0.739 |
| walker |  | 3083 | 41 | Fs::DirListing { dir: packages/d2ts/tests/operators-sqlite } |  |  | 0.739 |
| walker |  | 3096 | 13 | Code::CodeKey { rung: Names, file: packages/d2mini/src/operators/filterBy.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.739 |
| walker |  | 3109 | 13 | Code::CodeKey { rung: Names, file: packages/d2ts/src/operators/filterBy.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.739 |
| walker |  | 3130 | 21 | Code::CodeKey { rung: Names, file: packages/d2ql/src/compiler.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.739 |
| walker |  | 3165 | 35 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/compiler.ts, decl: 1, sub: 0, line: 16 } |  |  | 0.739 |
| ns | 3172 |  | 190 | Per-package scripts: build, test, lint, typecheck, format | 3.1 | 1.7 | 0.743 |
| ns | 3259 |  | 87 | d2ts-benchmark scripts — the benchmark entry points | 3.2 | 3.1 | 0.736 |
| ns | 3452 |  | 193 | CI pipeline stages | 3.3 |  | 0.712 |
| walker |  | 3523 | 358 | Code::CodeKey { rung: Decl, file: eslint.base.mjs, decl: 1, sub: 0, line: 6 } |  |  | 0.712 |
| walker |  | 3638 | 115 | Fs::DirListing { dir: packages/d2ql/tests } |  |  | 0.760 |
| ns | 3656 |  | 204 | README: constructing a D2 graph and sending to an input stream | 4.1 | 1.10 | 0.750 |
| walker |  | 3692 | 54 | Fs::DirListing { dir: packages/d2ql/tests/query-builder } |  |  | 0.750 |
| walker |  | 3831 | 139 | Fs::DirListing { dir: packages/d2mini/tests/operators } |  |  | 0.750 |
| ns | 3981 |  | 325 | d2.ts — D2Options and the complete D2 class method roster | 4.2 |  | 0.725 |
| walker |  | 3990 | 159 | Fs::DirListing { dir: packages/d2ts/tests/operators } |  |  | 0.725 |
| walker |  | 4068 | 78 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/operators/filterBy.ts, decl: 1, sub: 0, line: 14 } |  |  | 0.725 |
| walker |  | 4146 | 78 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/filterBy.ts, decl: 1, sub: 0, line: 14 } |  |  | 0.725 |
| ns | 4171 |  | 190 | README: what versions and frontiers actually mean | 4.3 | 1.10 | 0.721 |
| walker |  | 4184 | 38 | Code::CodeKey { rung: Names, file: packages/d2ts/src/version-index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.721 |
| walker |  | 4214 | 30 | Markdown::Section { file: packages/d2ql/README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.721 |
| walker |  | 4240 | 26 | Code::CodeKey { rung: Names, file: packages/d2mini/src/operators/topK.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.721 |
| walker |  | 4266 | 26 | Code::CodeKey { rung: Names, file: packages/d2ts/src/operators/topK.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.721 |
| ns | 4517 |  | 346 | order.ts — v() factory and the complete Version method roster | 4.4 |  | 0.696 |
| walker |  | 4616 | 350 | Code::CodeKey { rung: Decl, file: eslint.base.mjs, decl: 1, sub: 1, line: 6 } |  |  | 0.696 |
| walker |  | 4799 | 183 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.696 |
| ns | 4807 |  | 290 | order.ts — Antichain method roster and Frontier | 4.5 | 4.4 | 0.675 |
| ns | 4952 |  | 145 | README: MultiSet as a changeset, and keyed multisets | 4.6 | 1.10 | 0.672 |
| walker |  | 4955 | 156 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/version-index.ts, decl: 1, sub: 0, line: 8 } |  |  | 0.672 |
| walker |  | 5130 | 175 | Markdown::HeadingsOutline { file: packages/d2ts/README.md } |  |  | 0.673 |
| walker |  | 5302 | 172 | Markdown::Section { file: packages/d2ts/README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.673 |
| ns | 5310 |  | 358 | multiset.ts — MultiSetArray, KeyedData and every MultiSet method | 4.7 |  | 0.656 |
| walker |  | 5333 | 31 | Code::CodeKey { rung: Names, file: packages/d2mini/src/operators/debug.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.656 |
| walker |  | 5360 | 27 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/operators/debug.ts, decl: 1, sub: 0, line: 12 } |  |  | 0.656 |
| walker |  | 5393 | 33 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/operators/debug.ts, decl: 4, sub: 0, line: 42 } |  |  | 0.656 |
| walker |  | 5424 | 31 | Code::CodeKey { rung: Names, file: packages/d2mini/src/operators/output.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.656 |
| walker |  | 5451 | 27 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/operators/output.ts, decl: 1, sub: 0, line: 13 } |  |  | 0.656 |
| walker |  | 5481 | 30 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/operators/output.ts, decl: 4, sub: 0, line: 38 } |  |  | 0.656 |
| walker |  | 5512 | 31 | Code::CodeKey { rung: Names, file: packages/d2ts/src/operators/debug.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.656 |
| walker |  | 5539 | 27 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/debug.ts, decl: 1, sub: 0, line: 18 } |  |  | 0.656 |
| walker |  | 5572 | 33 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/debug.ts, decl: 4, sub: 0, line: 74 } |  |  | 0.656 |
| ns | 5610 |  | 300 | types.ts — the operator message protocol | 4.8 |  | 0.634 |
| walker |  | 5675 | 103 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/operators/topK.ts, decl: 1, sub: 0, line: 22 } |  |  | 0.634 |
| walker |  | 5778 | 103 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/topK.ts, decl: 1, sub: 0, line: 22 } |  |  | 0.634 |
| walker |  | 5836 | 58 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/operators/output.ts, decl: 2, sub: 0, line: 16 } |  |  | 0.634 |
| ns | 5868 |  | 258 | types.ts — PipedOperator and the 20-deep pipe overload wall | 4.9 | 4.8 | 0.622 |
| walker |  | 5890 | 54 | Code::CodeKey { rung: Names, file: packages/d2mini/src/d2.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.622 |
| walker |  | 5916 | 26 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/d2.ts, decl: 37, sub: 0, line: 146 } |  |  | 0.622 |
| walker |  | 5927 | 11 | Code::CodeKey { rung: Body, file: packages/d2mini/src/d2.ts, decl: 38, sub: 0, line: 147 } |  |  | 0.622 |
| walker |  | 6058 | 131 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/d2.ts, decl: 1, sub: 0, line: 10 } |  |  | 0.622 |
| walker |  | 6163 | 105 | Markdown::Section { file: README.md, section_index: 18, keeps_default_concavity: false } |  |  | 0.622 |
| ns | 6251 |  | 383 | README Key Features — every operator with a one-line description | 5.1 | 1.10 | 0.632 |
| walker |  | 6275 | 112 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/operators/topK.ts, decl: 2, sub: 0, line: 60 } |  |  | 0.632 |
| walker |  | 6387 | 112 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/topK.ts, decl: 2, sub: 0, line: 60 } |  |  | 0.632 |
| walker |  | 6442 | 55 | Code::CodeKey { rung: Names, file: packages/d2ts/src/order.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.632 |
| walker |  | 6459 | 17 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/order.ts, decl: 33, sub: 0, line: 277 } |  |  | 0.632 |
| walker |  | 6467 | 8 | Code::CodeKey { rung: Body, file: packages/d2ts/src/order.ts, decl: 34, sub: 0, line: 278 } |  |  | 0.632 |
| walker |  | 6523 | 56 | Code::CodeKey { rung: Names, file: packages/d2mini/src/multiset.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.632 |
| walker |  | 6579 | 56 | Code::CodeKey { rung: Names, file: packages/d2ts/src/multiset.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.633 |
| walker |  | 6640 | 61 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/operators/debug.ts, decl: 2, sub: 0, line: 16 } |  |  | 0.633 |
| walker |  | 6676 | 36 | Code::CodeKey { rung: Names, file: packages/d2mini/src/operators/count.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.633 |
| ns | 6681 |  | 430 | packages/d2ts/README.md — complete heading map (1081-line API reference) | 5.2 |  | 0.618 |
| walker |  | 6690 | 14 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/operators/count.ts, decl: 1, sub: 0, line: 9 } |  |  | 0.618 |
| walker |  | 6748 | 58 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/operators/count.ts, decl: 3, sub: 0, line: 30 } |  |  | 0.618 |
| walker |  | 6797 | 49 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/operators/count.ts, decl: 2, sub: 0, line: 10 } |  |  | 0.618 |
| walker |  | 6833 | 36 | Code::CodeKey { rung: Names, file: packages/d2ts/src/operators/count.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.618 |
| ns | 6844 |  | 163 | Operator factory roster — single-stream operators | 5.3 | 2.5 | 0.611 |
| walker |  | 6847 | 14 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/count.ts, decl: 1, sub: 0, line: 10 } |  |  | 0.611 |
| walker |  | 6905 | 58 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/count.ts, decl: 3, sub: 0, line: 32 } |  |  | 0.611 |
| walker |  | 6967 | 62 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/count.ts, decl: 2, sub: 0, line: 11 } |  |  | 0.611 |
| walker |  | 7004 | 37 | Code::CodeKey { rung: Names, file: packages/d2ts/src/operators/distinct.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.612 |
| walker |  | 7018 | 14 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/distinct.ts, decl: 1, sub: 0, line: 10 } |  |  | 0.612 |
| walker |  | 7076 | 58 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/distinct.ts, decl: 3, sub: 0, line: 37 } |  |  | 0.612 |
| walker |  | 7094 | 18 | Code::CodeKey { rung: Doc, file: packages/d2ts/src/operators/distinct.ts, decl: 3, sub: 0, line: 37 } |  |  | 0.612 |
| walker |  | 7156 | 62 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/distinct.ts, decl: 2, sub: 0, line: 11 } |  |  | 0.612 |
| walker |  | 7177 | 21 | Code::CodeKey { rung: Doc, file: packages/d2ts/src/operators/count.ts, decl: 3, sub: 0, line: 32 } |  |  | 0.612 |
| ns | 7244 |  | 400 | Operator factory roster — keyed, join, ordering and aggregate families | 5.4 | 5.3 | 0.594 |
| walker |  | 7379 | 202 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/order.ts, decl: 2, sub: 0, line: 29 } |  |  | 0.604 |
| walker |  | 7389 | 10 | Code::CodeKey { rung: Body, file: packages/d2ts/src/order.ts, decl: 16, sub: 0, line: 128 } |  |  | 0.604 |
| walker |  | 7400 | 11 | Code::CodeKey { rung: Body, file: packages/d2ts/src/order.ts, decl: 6, sub: 0, line: 50 } |  |  | 0.604 |
| walker |  | 7440 | 40 | Code::CodeKey { rung: Names, file: packages/d2ts/src/operators/buffer.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.605 |
| ns | 7446 |  | 202 | Operator implementation classes and the LinearUnaryOperator base | 5.5 | 5.4 | 0.597 |
| walker |  | 7458 | 18 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/buffer.ts, decl: 1, sub: 0, line: 17 } |  |  | 0.597 |
| ns | 7622 |  | 176 | groupBy.ts — the AggregateFunction contract | 5.6 | 5.4 | 0.590 |
| walker |  | 7662 | 204 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/multiset.ts, decl: 3, sub: 0, line: 14 } |  |  | 0.590 |
| walker |  | 7681 | 19 | Code::CodeKey { rung: Doc, file: packages/d2mini/src/multiset.ts, decl: 3, sub: 0, line: 14 } |  |  | 0.590 |
| walker |  | 7691 | 10 | Code::CodeKey { rung: Body, file: packages/d2mini/src/multiset.ts, decl: 14, sub: 0, line: 212 } |  |  | 0.590 |
| walker |  | 7702 | 11 | Code::CodeKey { rung: Body, file: packages/d2mini/src/multiset.ts, decl: 4, sub: 0, line: 17 } |  |  | 0.590 |
| ns | 7842 |  | 220 | D2QL identity and README section map | 6.1 |  | 0.586 |
| walker |  | 7848 | 146 | Code::CodeKey { rung: Body, file: packages/d2ts-benchmark/src/index.ts, decl: 1, sub: 0, line: 9 } |  |  | 0.586 |
| walker |  | 7889 | 41 | Code::CodeKey { rung: Names, file: packages/d2mini/src/operators/concat.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.586 |
| walker |  | 7905 | 16 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/operators/concat.ts, decl: 1, sub: 0, line: 9 } |  |  | 0.586 |
| walker |  | 7934 | 29 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/operators/concat.ts, decl: 3, sub: 0, line: 25 } |  |  | 0.586 |
| walker |  | 7975 | 41 | Code::CodeKey { rung: Names, file: packages/d2mini/src/operators/consolidate.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.586 |
| walker |  | 7991 | 16 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/operators/consolidate.ts, decl: 1, sub: 0, line: 9 } |  |  | 0.586 |
| walker |  | 8012 | 21 | Code::CodeKey { rung: Doc, file: packages/d2mini/src/operators/consolidate.ts, decl: 1, sub: 0, line: 9 } |  |  | 0.586 |
| walker |  | 8033 | 21 | Code::CodeKey { rung: Doc, file: packages/d2mini/src/operators/consolidate.ts, decl: 3, sub: 0, line: 35 } |  |  | 0.586 |
| walker |  | 8074 | 41 | Code::CodeKey { rung: Names, file: packages/d2ts/src/operators/concat.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.588 |
| ns | 8084 |  | 242 | D2QL "Current Features" — the supported SQL subset | 6.2 | 6.1 | 0.580 |
| walker |  | 8090 | 16 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/concat.ts, decl: 1, sub: 0, line: 11 } |  |  | 0.580 |
| walker |  | 8119 | 29 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/concat.ts, decl: 3, sub: 0, line: 54 } |  |  | 0.580 |
| walker |  | 8160 | 41 | Code::CodeKey { rung: Names, file: packages/d2ts/src/operators/consolidate.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.582 |
| walker |  | 8178 | 18 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/consolidate.ts, decl: 1, sub: 0, line: 16 } |  |  | 0.582 |
| walker |  | 8199 | 21 | Code::CodeKey { rung: Doc, file: packages/d2ts/src/operators/consolidate.ts, decl: 3, sub: 0, line: 61 } |  |  | 0.582 |
| walker |  | 8240 | 41 | Code::CodeKey { rung: Names, file: packages/d2ts/src/operators/topKWithFractionalIndex.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.583 |
| walker |  | 8290 | 50 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/topKWithFractionalIndex.ts, decl: 1, sub: 0, line: 29 } |  |  | 0.583 |
| walker |  | 8406 | 116 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/topKWithFractionalIndex.ts, decl: 4, sub: 0, line: 372 } |  |  | 0.583 |
| walker |  | 8429 | 23 | Code::CodeKey { rung: Doc, file: packages/d2mini/src/operators/concat.ts, decl: 1, sub: 0, line: 9 } |  |  | 0.583 |
| ns | 8442 |  | 358 | schema.ts — the Query interface family | 6.3 |  | 0.570 |
| walker |  | 8452 | 23 | Code::CodeKey { rung: Doc, file: packages/d2ts/src/operators/concat.ts, decl: 1, sub: 0, line: 11 } |  |  | 0.570 |
| walker |  | 8601 | 149 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.571 |
| ns | 8604 |  | 162 | schema.ts — the Comparator and LogicalOperator vocabularies | 6.4 | 6.3 | 0.563 |
| walker |  | 8668 | 67 | Code::CodeKey { rung: Names, file: packages/d2ts/src/d2.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.563 |
| walker |  | 8690 | 22 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/d2.ts, decl: 1, sub: 0, line: 11 } |  |  | 0.564 |
| walker |  | 8728 | 38 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/d2.ts, decl: 41, sub: 0, line: 166 } |  |  | 0.564 |
| ns | 8735 |  | 131 | compileQuery — the package's single entry point | 6.5 |  | 0.561 |
| walker |  | 8766 | 38 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/d2.ts, decl: 42, sub: 0, line: 167 } |  |  | 0.561 |
| walker |  | 8779 | 13 | Code::CodeKey { rung: Body, file: packages/d2ts/src/d2.ts, decl: 42, sub: 0, line: 167 } |  |  | 0.561 |
| walker |  | 8964 | 185 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/d2.ts, decl: 2, sub: 0, line: 15 } |  |  | 0.572 |
| ns | 8978 |  | 243 | query-builder.ts — the fluent builder's complete method set | 6.6 |  | 0.563 |
| walker |  | 9007 | 43 | Code::CodeKey { rung: Names, file: packages/d2ts/src/operators/reduce.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.565 |
| walker |  | 9034 | 27 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/reduce.ts, decl: 1, sub: 0, line: 15 } |  |  | 0.565 |
| ns | 9076 |  | 98 | d2mini README — how it differs from d2ts | 7.1 |  | 0.566 |
| walker |  | 9119 | 85 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/reduce.ts, decl: 4, sub: 0, line: 127 } |  |  | 0.566 |
| walker |  | 9140 | 21 | Code::CodeKey { rung: Doc, file: packages/d2ts/src/operators/reduce.ts, decl: 1, sub: 0, line: 15 } |  |  | 0.566 |
| walker |  | 9163 | 23 | Code::CodeKey { rung: Doc, file: packages/d2ts/src/operators/reduce.ts, decl: 4, sub: 0, line: 127 } |  |  | 0.566 |
| ns | 9165 |  | 89 | d2mini/src/d2.ts — the versionless D2 constructor | 7.2 | 7.1 | 0.563 |
| walker |  | 9187 | 24 | Code::CodeKey { rung: Doc, file: packages/d2mini/src/operators/count.ts, decl: 3, sub: 0, line: 30 } |  |  | 0.563 |
| walker |  | 9211 | 24 | Code::CodeKey { rung: Doc, file: packages/d2mini/src/operators/debug.ts, decl: 1, sub: 0, line: 12 } |  |  | 0.563 |
| walker |  | 9235 | 24 | Code::CodeKey { rung: Doc, file: packages/d2mini/src/operators/output.ts, decl: 1, sub: 0, line: 13 } |  |  | 0.563 |
| walker |  | 9259 | 24 | Code::CodeKey { rung: Doc, file: packages/d2ts/src/operators/consolidate.ts, decl: 1, sub: 0, line: 16 } |  |  | 0.563 |
| walker |  | 9283 | 24 | Code::CodeKey { rung: Doc, file: packages/d2ts/src/operators/debug.ts, decl: 1, sub: 0, line: 18 } |  |  | 0.563 |
| walker |  | 9302 | 19 | Code::CodeKey { rung: Body, file: packages/d2mini/src/d2.ts, decl: 9, sub: 0, line: 62 } |  |  | 0.563 |
| walker |  | 9376 | 74 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/debug.ts, decl: 2, sub: 0, line: 22 } |  |  | 0.563 |
| ns | 9401 |  | 236 | README: using the SQLite backend | 8.1 | 1.10 | 0.558 |
| ns | 9529 |  | 128 | sqlite/database.ts — the SQLiteDb driver interface | 8.2 | 8.1 | 0.553 |
| walker |  | 9598 | 222 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/order.ts, decl: 17, sub: 0, line: 138 } |  |  | 0.566 |
| walker |  | 9628 | 30 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/order.ts, decl: 19, sub: 0, line: 148 } |  |  | 0.566 |
| walker |  | 9672 | 44 | Code::CodeKey { rung: Names, file: packages/d2mini/src/operators/negate.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.566 |
| ns | 9683 |  | 154 | withSQLite, SQLIndex and the Electric bridge exports | 8.3 | 8.2 | 0.562 |
| walker |  | 9696 | 24 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/operators/negate.ts, decl: 1, sub: 0, line: 10 } |  |  | 0.562 |
| walker |  | 9706 | 10 | Code::CodeKey { rung: Body, file: packages/d2mini/src/operators/negate.ts, decl: 2, sub: 0, line: 11 } |  |  | 0.562 |
| walker |  | 9729 | 23 | Code::CodeKey { rung: Doc, file: packages/d2mini/src/operators/negate.ts, decl: 3, sub: 0, line: 19 } |  |  | 0.562 |
| walker |  | 9773 | 44 | Code::CodeKey { rung: Names, file: packages/d2ts/src/operators/negate.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.564 |
| walker |  | 9797 | 24 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/negate.ts, decl: 1, sub: 0, line: 10 } |  |  | 0.564 |
| walker |  | 9807 | 10 | Code::CodeKey { rung: Body, file: packages/d2ts/src/operators/negate.ts, decl: 2, sub: 0, line: 11 } |  |  | 0.564 |
| ns | 9827 |  | 144 | README: what each example demonstrates | 9.1 | 1.10 | 0.567 |
| walker |  | 9830 | 23 | Code::CodeKey { rung: Doc, file: packages/d2ts/src/operators/negate.ts, decl: 3, sub: 0, line: 19 } |  |  | 0.567 |
| ns | 9977 |  | 150 | examples/electric — what the demo app does | 9.2 | 2.13 | 0.565 |

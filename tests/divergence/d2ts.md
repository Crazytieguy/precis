Score(3000)=0.730 I=0.868 C=0.614 ns_rows≤3K=24/55 grid(1000/1442/2080/3000/4327/6240/9000)=0.670/0.758/0.824/0.730/0.714/0.686/0.627

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
| walker |  | 157 | 12 | Fs::DirListing { dir: packages/d2ts-benchmark/src } |  |  | 0.637 |
| walker |  | 165 | 8 | Fs::DirListing { dir: .changeset } |  |  | 0.637 |
| walker |  | 190 | 25 | Fs::DirListing { dir: packages/d2ql } |  |  | 0.642 |
| ns | 192 |  | 57 | Workspace membership — packages/ and examples/ listings | 1.3 |  | 0.480 |
| ns | 218 |  | 26 | pnpm workspace globs | 1.4 |  | 0.451 |
| walker |  | 220 | 30 | Fs::DirListing { dir: packages/d2mini } |  |  | 0.459 |
| ns | 288 |  | 70 | README lede — what incremental execution buys you | 1.5 | 1.1 | 0.451 |
| walker |  | 301 | 81 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.701 |
| walker |  | 337 | 36 | Fs::DirListing { dir: packages/d2ts } |  |  | 0.713 |
| ns | 387 |  | 99 | README lede — ElectricSQL ShapeStreams and pipeline type inference | 1.6 | 1.1 | 0.676 |
| walker |  | 422 | 85 | Json::Identity { file: packages/d2ts/package.json } |  |  | 0.678 |
| walker |  | 455 | 33 | Fs::DirListing { dir: packages/d2mini/src } |  |  | 0.679 |
| walker |  | 559 | 104 | Fs::DirListing { dir: packages/d2mini/src/operators } |  |  | 0.691 |
| ns | 578 |  | 191 | Root package.json — identity and every workspace script | 1.7 |  | 0.567 |
| walker |  | 603 | 44 | Fs::DirListing { dir: packages/d2ts/src } |  |  | 0.572 |
| walker |  | 607 | 4 | Fs::DirListing { dir: packages/d2ts/src/electric } |  |  | 0.572 |
| walker |  | 627 | 20 | Fs::DirListing { dir: packages/d2ts/src/sqlite } |  |  | 0.573 |
| walker |  | 725 | 98 | Fs::DirListing { dir: packages/d2ts/src/operators } |  |  | 0.583 |
| ns | 728 |  | 150 | Published package identity: d2ts and d2mini | 1.8 |  | 0.557 |
| walker |  | 799 | 74 | Json::Dependencies { file: packages/d2ts/package.json } |  |  | 0.557 |
| walker |  | 834 | 35 | Fs::DirListing { dir: examples } |  |  | 0.723 |
| ns | 871 |  | 143 | Private package identity: d2ql and d2ts-benchmark | 1.9 | 1.8 | 0.663 |
| walker |  | 903 | 69 | Fs::DirListing { dir: packages/d2ql/src } |  |  | 0.668 |
| walker |  | 912 | 9 | Fs::DirListing { dir: packages/d2ql/src/query-builder } |  |  | 0.669 |
| walker |  | 944 | 32 | Markdown::HeadingsOutline { file: packages/d2mini/README.md } |  |  | 0.669 |
| ns | 1032 |  | 161 | Root README section map (every H2) | 1.10 |  | 0.589 |
| walker |  | 1117 | 173 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.740 |
| ns | 1284 |  | 252 | README "Implementation Details" — provenance and the four core data structures | 1.11 | 1.10 | 0.659 |
| walker |  | 1289 | 172 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.695 |
| ns | 1392 |  | 108 | Every package's own root listing | 2.1 |  | 0.744 |
| ns | 1436 |  | 44 | packages/d2ts/src — complete module roster | 2.2 |  | 0.758 |
| ns | 1506 |  | 70 | packages/d2ts/src/index.ts — the public export barrel | 2.3 |  | 0.739 |
| ns | 1604 |  | 98 | packages/d2ts/src/operators — complete operator file roster | 2.4 |  | 0.760 |
| walker |  | 1672 | 383 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: true } |  |  | 0.763 |
| walker |  | 1696 | 24 | Fs::DirListing { dir: examples/electric } |  |  | 0.764 |
| walker |  | 1704 | 8 | Fs::DirListing { dir: examples/electric/src } |  |  | 0.764 |
| walker |  | 1730 | 26 | Plaintext::Whole { file: pnpm-workspace.yaml } |  |  | 0.782 |
| walker |  | 1741 | 11 | Code::CodeKey { rung: Names, file: eslint.base.mjs, decl: 0, sub: 0, line: 0 } |  |  | 0.782 |
| ns | 1844 |  | 240 | packages/d2ts/src/operators/index.ts — exported operator set | 2.5 | 2.4 | 0.731 |
| walker |  | 1877 | 136 | Json::Scripts { file: package.json } |  |  | 0.788 |
| walker |  | 1906 | 29 | Fs::DirListing { dir: packages/d2mini/tests } |  |  | 0.789 |
| walker |  | 1976 | 70 | Code::CodeKey { rung: Names, file: packages/d2ts/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.811 |
| ns | 1981 |  | 137 | packages/d2mini/src and its operators directory | 2.6 |  | 0.824 |
| walker |  | 2023 | 47 | Code::CodeKey { rung: Names, file: packages/d2mini/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.824 |
| walker |  | 2061 | 38 | Code::CodeKey { rung: Names, file: packages/d2ql/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.824 |
| walker |  | 2099 | 38 | Code::CodeKey { rung: Names, file: packages/d2ts-benchmark/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.824 |
| walker |  | 2155 | 56 | Code::CodeKey { rung: Decl, file: packages/d2ts-benchmark/src/index.ts, decl: 2, sub: 0, line: 25 } |  |  | 0.824 |
| walker |  | 2242 | 87 | Code::CodeKey { rung: Decl, file: packages/d2ts-benchmark/src/index.ts, decl: 3, sub: 0, line: 496 } |  |  | 0.824 |
| ns | 2246 |  | 265 | packages/d2mini export barrels | 2.7 | 2.6 | 0.780 |
| walker |  | 2274 | 32 | Fs::DirListing { dir: examples/d2ql } |  |  | 0.782 |
| ns | 2324 |  | 78 | packages/d2ql/src and query-builder/ listings | 2.8 |  | 0.791 |
| ns | 2441 |  | 117 | packages/d2ql export barrels | 2.9 | 2.8 | 0.777 |
| walker |  | 2449 | 175 | Json::Entry { file: packages/d2ts/package.json } |  |  | 0.777 |
| ns | 2518 |  | 77 | packages/d2ts/src/sqlite — complete tree listing | 2.10 |  | 0.753 |
| walker |  | 2637 | 188 | Json::Scripts { file: packages/d2ts/package.json } |  |  | 0.754 |
| walker |  | 2694 | 57 | Fs::DirListing { dir: packages/d2ts/src/sqlite/operators } |  |  | 0.790 |
| ns | 2701 |  | 183 | SQLite subpath export barrels | 2.11 | 2.10 | 0.767 |
| walker |  | 2735 | 41 | Fs::DirListing { dir: packages/d2ts/tests } |  |  | 0.768 |
| walker |  | 2776 | 41 | Fs::DirListing { dir: packages/d2ts/tests/operators-sqlite } |  |  | 0.768 |
| ns | 2886 |  | 185 | Test suite layout — immediate children of every tests/ directory | 2.12 |  | 0.731 |
| ns | 2982 |  | 96 | Example and benchmark source trees | 2.13 | 1.3 | 0.730 |
| walker |  | 3134 | 358 | Code::CodeKey { rung: Decl, file: eslint.base.mjs, decl: 1, sub: 0, line: 6 } |  |  | 0.730 |
| ns | 3172 |  | 190 | Per-package scripts: build, test, lint, typecheck, format | 3.1 | 1.7 | 0.734 |
| ns | 3259 |  | 87 | d2ts-benchmark scripts — the benchmark entry points | 3.2 | 3.1 | 0.727 |
| walker |  | 3317 | 183 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.727 |
| walker |  | 3432 | 115 | Fs::DirListing { dir: packages/d2ql/tests } |  |  | 0.778 |
| ns | 3452 |  | 193 | CI pipeline stages | 3.3 |  | 0.752 |
| walker |  | 3486 | 54 | Fs::DirListing { dir: packages/d2ql/tests/query-builder } |  |  | 0.752 |
| ns | 3656 |  | 204 | README: constructing a D2 graph and sending to an input stream | 4.1 | 1.10 | 0.742 |
| walker |  | 3836 | 350 | Code::CodeKey { rung: Decl, file: eslint.base.mjs, decl: 1, sub: 1, line: 6 } |  |  | 0.742 |
| walker |  | 3975 | 139 | Fs::DirListing { dir: packages/d2mini/tests/operators } |  |  | 0.742 |
| ns | 3981 |  | 325 | d2.ts — D2Options and the complete D2 class method roster | 4.2 |  | 0.718 |
| walker |  | 4080 | 105 | Markdown::Section { file: README.md, section_index: 18, keeps_default_concavity: false } |  |  | 0.718 |
| ns | 4171 |  | 190 | README: what versions and frontiers actually mean | 4.3 | 1.10 | 0.714 |
| walker |  | 4239 | 159 | Fs::DirListing { dir: packages/d2ts/tests/operators } |  |  | 0.714 |
| walker |  | 4385 | 146 | Code::CodeKey { rung: Body, file: packages/d2ts-benchmark/src/index.ts, decl: 1, sub: 0, line: 9 } |  |  | 0.714 |
| ns | 4517 |  | 346 | order.ts — v() factory and the complete Version method roster | 4.4 |  | 0.688 |
| walker |  | 4534 | 149 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.689 |
| walker |  | 4733 | 199 | Markdown::HeadingsOutline { file: packages/d2ts/README.md } |  |  | 0.690 |
| ns | 4807 |  | 290 | order.ts — Antichain method roster and Frontier | 4.5 | 4.4 | 0.669 |
| walker |  | 4929 | 196 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.673 |
| ns | 4952 |  | 145 | README: MultiSet as a changeset, and keyed multisets | 4.6 | 1.10 | 0.670 |
| walker |  | 5103 | 174 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.680 |
| walker |  | 5165 | 62 | Json::Identity { file: packages/d2ts-benchmark/package.json } |  |  | 0.684 |
| walker |  | 5252 | 87 | Json::Scripts { file: packages/d2ts-benchmark/package.json } |  |  | 0.692 |
| ns | 5310 |  | 358 | multiset.ts — MultiSetArray, KeyedData and every MultiSet method | 4.7 |  | 0.675 |
| walker |  | 5317 | 65 | Json::Dependencies { file: packages/d2ts-benchmark/package.json } |  |  | 0.675 |
| walker |  | 5396 | 79 | Markdown::Section { file: packages/d2mini/README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.675 |
| ns | 5610 |  | 300 | types.ts — the operator message protocol | 4.8 |  | 0.652 |
| walker |  | 5653 | 257 | Markdown::Section { file: README.md, section_index: 17, keeps_default_concavity: false } |  |  | 0.685 |
| walker |  | 5685 | 32 | Markdown::Section { file: packages/d2ql/README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.685 |
| walker |  | 5720 | 35 | Code::CodeKey { rung: Names, file: packages/d2ts-benchmark/src/base.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.685 |
| walker |  | 5777 | 57 | Code::CodeKey { rung: Decl, file: packages/d2ts-benchmark/src/base.ts, decl: 2, sub: 0, line: 19 } |  |  | 0.685 |
| walker |  | 5853 | 76 | Code::CodeKey { rung: Decl, file: packages/d2ts-benchmark/src/base.ts, decl: 1, sub: 0, line: 4 } |  |  | 0.685 |
| ns | 5868 |  | 258 | types.ts — PipedOperator and the 20-deep pipe overload wall | 4.9 | 4.8 | 0.672 |
| walker |  | 6018 | 165 | Code::CodeKey { rung: Decl, file: packages/d2ts-benchmark/src/base.ts, decl: 3, sub: 0, line: 27 } |  |  | 0.672 |
| walker |  | 6026 | 8 | Code::CodeKey { rung: Body, file: packages/d2ts-benchmark/src/base.ts, decl: 6, sub: 0, line: 48 } |  |  | 0.672 |
| walker |  | 6043 | 17 | Code::CodeKey { rung: Body, file: packages/d2ts-benchmark/src/base.ts, decl: 5, sub: 0, line: 44 } |  |  | 0.672 |
| walker |  | 6134 | 91 | Json::Identity { file: packages/d2mini/package.json } |  |  | 0.686 |
| walker |  | 6217 | 83 | Json::Entry { file: packages/d2mini/package.json } |  |  | 0.686 |
| ns | 6251 |  | 383 | README Key Features — every operator with a one-line description | 5.1 | 1.10 | 0.693 |
| walker |  | 6405 | 188 | Json::Scripts { file: packages/d2mini/package.json } |  |  | 0.693 |
| walker |  | 6476 | 71 | Json::Dependencies { file: packages/d2mini/package.json } |  |  | 0.693 |
| walker |  | 6570 | 94 | Json::Identity { file: packages/d2ql/package.json } |  |  | 0.708 |
| ns | 6681 |  | 430 | packages/d2ts/README.md — complete heading map (1081-line API reference) | 5.2 |  | 0.690 |
| walker |  | 6742 | 172 | Json::Entry { file: packages/d2ql/package.json } |  |  | 0.690 |
| ns | 6844 |  | 163 | Operator factory roster — single-stream operators | 5.3 | 2.5 | 0.682 |
| walker |  | 6930 | 188 | Json::Scripts { file: packages/d2ql/package.json } |  |  | 0.682 |
| walker |  | 6943 | 13 | Code::CodeKey { rung: Names, file: packages/d2ql/src/select.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.682 |
| walker |  | 7003 | 60 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/select.ts, decl: 1, sub: 0, line: 7 } |  |  | 0.682 |
| walker |  | 7029 | 26 | Code::CodeKey { rung: Names, file: packages/d2ql/src/group-by.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.682 |
| walker |  | 7069 | 40 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/group-by.ts, decl: 2, sub: 0, line: 98 } |  |  | 0.682 |
| walker |  | 7110 | 41 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/group-by.ts, decl: 1, sub: 0, line: 20 } |  |  | 0.682 |
| walker |  | 7137 | 27 | Code::CodeKey { rung: Doc, file: packages/d2ql/src/group-by.ts, decl: 1, sub: 0, line: 20 } |  |  | 0.682 |
| walker |  | 7165 | 28 | Code::CodeKey { rung: Doc, file: packages/d2ql/src/group-by.ts, decl: 2, sub: 0, line: 98 } |  |  | 0.682 |
| walker |  | 7191 | 26 | Code::CodeKey { rung: Names, file: packages/d2ql/src/joins.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.682 |
| walker |  | 7240 | 49 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/joins.ts, decl: 2, sub: 0, line: 124 } |  |  | 0.682 |
| ns | 7244 |  | 400 | Operator factory roster — keyed, join, ordering and aggregate families | 5.4 | 5.3 | 0.661 |
| walker |  | 7320 | 80 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/joins.ts, decl: 1, sub: 0, line: 16 } |  |  | 0.661 |
| walker |  | 7343 | 23 | Code::CodeKey { rung: Doc, file: packages/d2ql/src/joins.ts, decl: 1, sub: 0, line: 16 } |  |  | 0.661 |
| walker |  | 7366 | 23 | Code::CodeKey { rung: Doc, file: packages/d2ql/src/joins.ts, decl: 2, sub: 0, line: 124 } |  |  | 0.661 |
| walker |  | 7421 | 55 | Code::CodeKey { rung: Names, file: packages/d2ts/src/order.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.662 |
| walker |  | 7438 | 17 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/order.ts, decl: 33, sub: 0, line: 277 } |  |  | 0.662 |
| walker |  | 7446 | 8 | Code::CodeKey { rung: Body, file: packages/d2ts/src/order.ts, decl: 34, sub: 0, line: 278 } |  |  | 0.652 |
| ns | 7446 |  | 202 | Operator implementation classes and the LinearUnaryOperator base | 5.5 | 5.4 | 0.652 |
| ns | 7622 |  | 176 | groupBy.ts — the AggregateFunction contract | 5.6 | 5.4 | 0.645 |
| walker |  | 7648 | 202 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/order.ts, decl: 2, sub: 0, line: 29 } |  |  | 0.655 |
| walker |  | 7658 | 10 | Code::CodeKey { rung: Body, file: packages/d2ts/src/order.ts, decl: 16, sub: 0, line: 128 } |  |  | 0.655 |
| walker |  | 7669 | 11 | Code::CodeKey { rung: Body, file: packages/d2ts/src/order.ts, decl: 6, sub: 0, line: 50 } |  |  | 0.655 |
| walker |  | 7683 | 14 | Code::CodeKey { rung: Body, file: packages/d2ts/src/order.ts, decl: 5, sub: 0, line: 46 } |  |  | 0.655 |
| ns | 7842 |  | 220 | D2QL identity and README section map | 6.1 |  | 0.648 |
| walker |  | 7905 | 222 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/order.ts, decl: 17, sub: 0, line: 138 } |  |  | 0.661 |
| walker |  | 7935 | 30 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/order.ts, decl: 19, sub: 0, line: 148 } |  |  | 0.661 |
| walker |  | 7973 | 38 | Code::CodeKey { rung: Doc, file: packages/d2ts/src/order.ts, decl: 17, sub: 0, line: 138 } |  |  | 0.667 |
| walker |  | 8030 | 57 | Code::CodeKey { rung: Doc, file: packages/d2ts/src/order.ts, decl: 1, sub: 0, line: 10 } |  |  | 0.672 |
| walker |  | 8044 | 14 | Code::CodeKey { rung: Names, file: packages/d2ql/src/key-by.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.672 |
| ns | 8084 |  | 242 | D2QL "Current Features" — the supported SQL subset | 6.2 | 6.1 | 0.663 |
| walker |  | 8100 | 56 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/key-by.ts, decl: 1, sub: 0, line: 4 } |  |  | 0.663 |
| walker |  | 8114 | 14 | Code::CodeKey { rung: Names, file: packages/d2ql/src/order-by.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.663 |
| walker |  | 8181 | 67 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/order-by.ts, decl: 1, sub: 0, line: 16 } |  |  | 0.663 |
| walker |  | 8294 | 113 | Markdown::Section { file: packages/d2ql/README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.665 |
| walker |  | 8386 | 92 | Code::CodeKey { rung: Names, file: packages/d2ql/src/utils.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.665 |
| walker |  | 8410 | 24 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/utils.ts, decl: 3, sub: 0, line: 36 } |  |  | 0.665 |
| ns | 8442 |  | 358 | schema.ts — the Query interface family | 6.3 |  | 0.651 |
| walker |  | 8447 | 37 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/utils.ts, decl: 6, sub: 0, line: 192 } |  |  | 0.651 |
| walker |  | 8491 | 44 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/utils.ts, decl: 4, sub: 0, line: 55 } |  |  | 0.651 |
| walker |  | 8520 | 29 | Code::CodeKey { rung: Doc, file: packages/d2ql/src/utils.ts, decl: 1, sub: 0, line: 4 } |  |  | 0.651 |
| walker |  | 8549 | 29 | Code::CodeKey { rung: Doc, file: packages/d2ql/src/utils.ts, decl: 2, sub: 0, line: 24 } |  |  | 0.651 |
| walker |  | 8604 | 55 | Code::CodeKey { rung: Doc, file: packages/d2ql/src/utils.ts, decl: 5, sub: 0, line: 131 } |  |  | 0.642 |
| ns | 8604 |  | 162 | schema.ts — the Comparator and LogicalOperator vocabularies | 6.4 | 6.3 | 0.642 |
| walker |  | 8669 | 65 | Code::CodeKey { rung: Doc, file: packages/d2ql/src/utils.ts, decl: 3, sub: 0, line: 36 } |  |  | 0.642 |
| walker |  | 8700 | 31 | Code::CodeKey { rung: Names, file: packages/d2ql/src/evaluators.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.642 |
| ns | 8735 |  | 131 | compileQuery — the package's single entry point | 6.5 |  | 0.637 |
| walker |  | 8751 | 51 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/evaluators.ts, decl: 1, sub: 0, line: 14 } |  |  | 0.637 |
| walker |  | 8822 | 71 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/evaluators.ts, decl: 2, sub: 0, line: 126 } |  |  | 0.637 |
| walker |  | 8847 | 25 | Code::CodeKey { rung: Doc, file: packages/d2ql/src/evaluators.ts, decl: 1, sub: 0, line: 14 } |  |  | 0.637 |
| walker |  | 8873 | 26 | Code::CodeKey { rung: Doc, file: packages/d2ql/src/evaluators.ts, decl: 2, sub: 0, line: 126 } |  |  | 0.637 |
| ns | 8978 |  | 243 | query-builder.ts — the fluent builder's complete method set | 6.6 |  | 0.627 |
| walker |  | 8984 | 111 | Code::CodeKey { rung: Names, file: packages/d2mini/src/types.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.627 |
| walker |  | 9005 | 21 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/types.ts, decl: 7, sub: 0, line: 83 } |  |  | 0.627 |
| walker |  | 9028 | 23 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/types.ts, decl: 2, sub: 0, line: 6 } |  |  | 0.627 |
| walker |  | 9053 | 25 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/types.ts, decl: 3, sub: 0, line: 11 } |  |  | 0.627 |
| ns | 9076 |  | 98 | d2mini README — how it differs from d2ts | 7.1 |  | 0.629 |
| walker |  | 9091 | 38 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/types.ts, decl: 4, sub: 0, line: 16 } |  |  | 0.629 |
| ns | 9165 |  | 89 | d2mini/src/d2.ts — the versionless D2 constructor | 7.2 | 7.1 | 0.625 |
| walker |  | 9170 | 79 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/types.ts, decl: 5, sub: 0, line: 21 } |  |  | 0.625 |
| walker |  | 9186 | 16 | Code::CodeKey { rung: Names, file: packages/d2mini/src/indexes.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.625 |
| walker |  | 9370 | 184 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/indexes.ts, decl: 1, sub: 0, line: 9 } |  |  | 0.625 |
| walker |  | 9381 | 11 | Code::CodeKey { rung: Body, file: packages/d2mini/src/indexes.ts, decl: 6, sub: 0, line: 38 } |  |  | 0.625 |
| walker |  | 9392 | 11 | Code::CodeKey { rung: Body, file: packages/d2mini/src/indexes.ts, decl: 7, sub: 0, line: 42 } |  |  | 0.625 |
| ns | 9401 |  | 236 | README: using the SQLite backend | 8.1 | 1.10 | 0.620 |
| walker |  | 9403 | 11 | Code::CodeKey { rung: Body, file: packages/d2mini/src/indexes.ts, decl: 9, sub: 0, line: 50 } |  |  | 0.620 |
| walker |  | 9415 | 12 | Code::CodeKey { rung: Body, file: packages/d2mini/src/indexes.ts, decl: 8, sub: 0, line: 46 } |  |  | 0.620 |
| walker |  | 9441 | 26 | Code::CodeKey { rung: Body, file: packages/d2mini/src/indexes.ts, decl: 4, sub: 0, line: 28 } |  |  | 0.620 |
| walker |  | 9510 | 69 | Code::CodeKey { rung: Doc, file: packages/d2mini/src/indexes.ts, decl: 1, sub: 0, line: 9 } |  |  | 0.620 |
| ns | 9529 |  | 128 | sqlite/database.ts — the SQLiteDb driver interface | 8.2 | 8.1 | 0.614 |
| ns | 9683 |  | 154 | withSQLite, SQLIndex and the Electric bridge exports | 8.3 | 8.2 | 0.610 |
| walker |  | 9783 | 273 | Code::CodeKey { rung: Names, file: packages/d2ql/src/types.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.610 |
| walker |  | 9802 | 19 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/types.ts, decl: 7, sub: 0, line: 183 } |  |  | 0.610 |
| walker |  | 9821 | 19 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/types.ts, decl: 9, sub: 0, line: 192 } |  |  | 0.610 |
| ns | 9827 |  | 144 | README: what each example demonstrates | 9.1 | 1.10 | 0.613 |
| walker |  | 9840 | 19 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/types.ts, decl: 11, sub: 0, line: 201 } |  |  | 0.613 |
| walker |  | 9861 | 21 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/types.ts, decl: 4, sub: 0, line: 24 } |  |  | 0.613 |
| walker |  | 9889 | 28 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/types.ts, decl: 10, sub: 0, line: 196 } |  |  | 0.613 |
| walker |  | 9918 | 29 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/types.ts, decl: 14, sub: 0, line: 247 } |  |  | 0.613 |
| walker |  | 9949 | 31 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/types.ts, decl: 6, sub: 0, line: 178 } |  |  | 0.613 |
| ns | 9977 |  | 150 | examples/electric — what the demo app does | 9.2 | 2.13 | 0.611 |
| walker |  | 9980 | 31 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/types.ts, decl: 8, sub: 0, line: 187 } |  |  | 0.611 |
| walker |  | 9999 | 19 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/types.ts, decl: 3, sub: 0, line: 15 } |  |  | 0.611 |

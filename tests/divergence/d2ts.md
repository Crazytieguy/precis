Score(3000)=0.812 I=0.897 C=0.735 ns_rows≤3K=24/55 grid(1000/1442/2080/3000/4327/6240/9000)=0.673/0.762/0.810/0.812/0.751/0.708/0.660

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 54 | 54 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 81 |  | 81 | Repository identity — README title + one-line definition | 1.1 |  | 0.000 |
| walker |  | 135 | 81 | Markdown::ReadmeHeadline { file: README.md } |  |  | 1.000 |
| ns | 135 |  | 54 | Complete repository root listing | 1.2 |  | 1.000 |
| walker |  | 157 | 22 | Fs::DirListing { dir: packages } |  |  | 1.000 |
| ns | 192 |  | 57 | Workspace membership — packages/ and examples/ listings | 1.3 |  | 0.748 |
| walker |  | 202 | 45 | Json::Identity { file: package.json } |  |  | 0.750 |
| ns | 218 |  | 26 | pnpm workspace globs | 1.4 |  | 0.705 |
| walker |  | 219 | 17 | Fs::DirListing { dir: packages/d2ts-benchmark } |  |  | 0.706 |
| walker |  | 231 | 12 | Fs::DirListing { dir: packages/d2ts-benchmark/src } |  |  | 0.706 |
| walker |  | 237 | 6 | Fs::DirListing { dir: .github/workflows } |  |  | 0.706 |
| walker |  | 245 | 8 | Fs::DirListing { dir: .changeset } |  |  | 0.706 |
| walker |  | 270 | 25 | Fs::DirListing { dir: packages/d2ql } |  |  | 0.709 |
| ns | 288 |  | 70 | README lede — what incremental execution buys you | 1.5 | 1.1 | 0.695 |
| walker |  | 339 | 69 | Fs::DirListing { dir: packages/d2ql/src } |  |  | 0.701 |
| walker |  | 348 | 9 | Fs::DirListing { dir: packages/d2ql/src/query-builder } |  |  | 0.702 |
| walker |  | 378 | 30 | Fs::DirListing { dir: packages/d2mini } |  |  | 0.708 |
| ns | 387 |  | 99 | README lede — ElectricSQL ShapeStreams and pipeline type inference | 1.6 | 1.1 | 0.671 |
| walker |  | 411 | 33 | Fs::DirListing { dir: packages/d2mini/src } |  |  | 0.672 |
| walker |  | 447 | 36 | Fs::DirListing { dir: packages/d2ts } |  |  | 0.683 |
| walker |  | 491 | 44 | Fs::DirListing { dir: packages/d2ts/src } |  |  | 0.691 |
| walker |  | 495 | 4 | Fs::DirListing { dir: packages/d2ts/src/electric } |  |  | 0.691 |
| walker |  | 515 | 20 | Fs::DirListing { dir: packages/d2ts/src/sqlite } |  |  | 0.691 |
| ns | 578 |  | 191 | Root package.json — identity and every workspace script | 1.7 |  | 0.566 |
| walker |  | 600 | 85 | Json::Identity { file: packages/d2ts/package.json } |  |  | 0.569 |
| walker |  | 657 | 57 | Fs::DirListing { dir: packages/d2ts/src/sqlite/operators } |  |  | 0.573 |
| ns | 728 |  | 150 | Published package identity: d2ts and d2mini | 1.8 |  | 0.547 |
| walker |  | 755 | 98 | Fs::DirListing { dir: packages/d2ts/src/operators } |  |  | 0.557 |
| walker |  | 859 | 104 | Fs::DirListing { dir: packages/d2mini/src/operators } |  |  | 0.566 |
| ns | 871 |  | 143 | Private package identity: d2ql and d2ts-benchmark | 1.9 | 1.8 | 0.519 |
| walker |  | 933 | 74 | Json::Dependencies { file: packages/d2ts/package.json } |  |  | 0.519 |
| walker |  | 968 | 35 | Fs::DirListing { dir: examples } |  |  | 0.673 |
| ns | 1032 |  | 161 | Root README section map (every H2) | 1.10 |  | 0.592 |
| walker |  | 1141 | 173 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.744 |
| ns | 1284 |  | 252 | README "Implementation Details" — provenance and the four core data structures | 1.11 | 1.10 | 0.663 |
| walker |  | 1313 | 172 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.699 |
| ns | 1392 |  | 108 | Every package's own root listing | 2.1 |  | 0.748 |
| ns | 1436 |  | 44 | packages/d2ts/src — complete module roster | 2.2 |  | 0.762 |
| ns | 1506 |  | 70 | packages/d2ts/src/index.ts — the public export barrel | 2.3 |  | 0.743 |
| ns | 1604 |  | 98 | packages/d2ts/src/operators — complete operator file roster | 2.4 |  | 0.765 |
| walker |  | 1696 | 383 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: true } |  |  | 0.767 |
| walker |  | 1722 | 26 | Plaintext::Whole { file: pnpm-workspace.yaml } |  |  | 0.785 |
| walker |  | 1733 | 11 | Code::CodeKey { rung: Names, file: eslint.base.mjs, decl: 0, sub: 0, line: 0 } |  |  | 0.785 |
| ns | 1844 |  | 240 | packages/d2ts/src/operators/index.ts — exported operator set | 2.5 | 2.4 | 0.734 |
| walker |  | 1869 | 136 | Json::Scripts { file: package.json } |  |  | 0.791 |
| walker |  | 1893 | 24 | Fs::DirListing { dir: examples/electric } |  |  | 0.792 |
| walker |  | 1901 | 8 | Fs::DirListing { dir: examples/electric/src } |  |  | 0.792 |
| walker |  | 1914 | 13 | Fs::DirListing { dir: examples/electric/db } |  |  | 0.793 |
| walker |  | 1960 | 46 | Code::CodeKey { rung: Names, file: packages/d2ts-benchmark/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.793 |
| ns | 1981 |  | 137 | packages/d2mini/src and its operators directory | 2.6 |  | 0.810 |
| walker |  | 2016 | 56 | Code::CodeKey { rung: Decl, file: packages/d2ts-benchmark/src/index.ts, decl: 2, sub: 0, line: 25 } |  |  | 0.810 |
| walker |  | 2103 | 87 | Code::CodeKey { rung: Decl, file: packages/d2ts-benchmark/src/index.ts, decl: 3, sub: 0, line: 496 } |  |  | 0.810 |
| walker |  | 2173 | 70 | Code::CodeKey { rung: Names, file: packages/d2ts/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.828 |
| walker |  | 2208 | 35 | Code::CodeKey { rung: Names, file: packages/d2ts/src/sqlite/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.829 |
| ns | 2246 |  | 265 | packages/d2mini export barrels | 2.7 | 2.6 | 0.783 |
| walker |  | 2255 | 47 | Code::CodeKey { rung: Names, file: packages/d2mini/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.785 |
| ns | 2324 |  | 78 | packages/d2ql/src and query-builder/ listings | 2.8 |  | 0.793 |
| ns | 2441 |  | 117 | packages/d2ql export barrels | 2.9 | 2.8 | 0.778 |
| walker |  | 2495 | 240 | Code::CodeKey { rung: Names, file: packages/d2ts/src/operators/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.828 |
| ns | 2518 |  | 77 | packages/d2ts/src/sqlite — complete tree listing | 2.10 |  | 0.833 |
| ns | 2701 |  | 183 | SQLite subpath export barrels | 2.11 | 2.10 | 0.809 |
| walker |  | 2713 | 218 | Code::CodeKey { rung: Names, file: packages/d2mini/src/operators/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.855 |
| walker |  | 2751 | 38 | Code::CodeKey { rung: Names, file: packages/d2ql/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.857 |
| walker |  | 2780 | 29 | Fs::DirListing { dir: packages/d2mini/tests } |  |  | 0.857 |
| ns | 2886 |  | 185 | Test suite layout — immediate children of every tests/ directory | 2.12 |  | 0.807 |
| walker |  | 2955 | 175 | Json::Entry { file: packages/d2ts/package.json } |  |  | 0.807 |
| ns | 2982 |  | 96 | Example and benchmark source trees | 2.13 | 1.3 | 0.792 |
| walker |  | 2987 | 32 | Fs::DirListing { dir: examples/d2ql } |  |  | 0.812 |
| ns | 3172 |  | 190 | Per-package scripts: build, test, lint, typecheck, format | 3.1 | 1.7 | 0.800 |
| walker |  | 3175 | 188 | Json::Scripts { file: packages/d2ts/package.json } |  |  | 0.816 |
| walker |  | 3216 | 41 | Fs::DirListing { dir: packages/d2ts/tests } |  |  | 0.824 |
| walker |  | 3257 | 41 | Fs::DirListing { dir: packages/d2ts/tests/operators-sqlite } |  |  | 0.824 |
| ns | 3259 |  | 87 | d2ts-benchmark scripts — the benchmark entry points | 3.2 | 3.1 | 0.816 |
| walker |  | 3339 | 82 | Code::CodeKey { rung: Names, file: packages/d2ts/src/electric/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.816 |
| walker |  | 3407 | 68 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/electric/index.ts, decl: 3, sub: 0, line: 222 } |  |  | 0.816 |
| ns | 3452 |  | 193 | CI pipeline stages | 3.3 |  | 0.789 |
| walker |  | 3481 | 74 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/electric/index.ts, decl: 7, sub: 0, line: 328 } |  |  | 0.789 |
| walker |  | 3562 | 81 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/electric/index.ts, decl: 4, sub: 0, line: 227 } |  |  | 0.789 |
| ns | 3656 |  | 204 | README: constructing a D2 graph and sending to an input stream | 4.1 | 1.10 | 0.779 |
| walker |  | 3683 | 121 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/electric/index.ts, decl: 2, sub: 0, line: 103 } |  |  | 0.779 |
| walker |  | 3934 | 251 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/electric/index.ts, decl: 1, sub: 0, line: 40 } |  |  | 0.779 |
| walker |  | 3955 | 21 | Code::CodeKey { rung: Names, file: packages/d2ql/src/query-builder/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.781 |
| ns | 3981 |  | 325 | d2.ts — D2Options and the complete D2 class method roster | 4.2 |  | 0.756 |
| ns | 4171 |  | 190 | README: what versions and frontiers actually mean | 4.3 | 1.10 | 0.751 |
| walker |  | 4313 | 358 | Code::CodeKey { rung: Decl, file: eslint.base.mjs, decl: 1, sub: 0, line: 6 } |  |  | 0.751 |
| walker |  | 4496 | 183 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.751 |
| ns | 4517 |  | 346 | order.ts — v() factory and the complete Version method roster | 4.4 |  | 0.724 |
| ns | 4807 |  | 290 | order.ts — Antichain method roster and Frontier | 4.5 | 4.4 | 0.703 |
| walker |  | 4846 | 350 | Code::CodeKey { rung: Decl, file: eslint.base.mjs, decl: 1, sub: 1, line: 6 } |  |  | 0.703 |
| walker |  | 4951 | 105 | Markdown::Section { file: README.md, section_index: 18, keeps_default_concavity: false } |  |  | 0.703 |
| ns | 4952 |  | 145 | README: MultiSet as a changeset, and keyed multisets | 4.6 | 1.10 | 0.700 |
| walker |  | 5066 | 115 | Fs::DirListing { dir: packages/d2ql/tests } |  |  | 0.741 |
| walker |  | 5120 | 54 | Fs::DirListing { dir: packages/d2ql/tests/query-builder } |  |  | 0.741 |
| walker |  | 5259 | 139 | Fs::DirListing { dir: packages/d2mini/tests/operators } |  |  | 0.741 |
| walker |  | 5302 | 43 | Code::CodeKey { rung: Doc, file: packages/d2ts/src/electric/index.ts, decl: 7, sub: 0, line: 328 } |  |  | 0.741 |
| ns | 5310 |  | 358 | multiset.ts — MultiSetArray, KeyedData and every MultiSet method | 4.7 |  | 0.722 |
| walker |  | 5461 | 159 | Fs::DirListing { dir: packages/d2ts/tests/operators } |  |  | 0.722 |
| walker |  | 5610 | 149 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.698 |
| ns | 5610 |  | 300 | types.ts — the operator message protocol | 4.8 |  | 0.698 |
| walker |  | 5806 | 196 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.702 |
| ns | 5868 |  | 258 | types.ts — PipedOperator and the 20-deep pipe overload wall | 4.9 | 4.8 | 0.689 |
| walker |  | 5980 | 174 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.697 |
| walker |  | 6051 | 71 | Code::CodeKey { rung: Doc, file: packages/d2ts/src/electric/index.ts, decl: 3, sub: 0, line: 222 } |  |  | 0.697 |
| walker |  | 6113 | 62 | Json::Identity { file: packages/d2ts-benchmark/package.json } |  |  | 0.701 |
| walker |  | 6200 | 87 | Json::Scripts { file: packages/d2ts-benchmark/package.json } |  |  | 0.708 |
| ns | 6251 |  | 383 | README Key Features — every operator with a one-line description | 5.1 | 1.10 | 0.714 |
| walker |  | 6265 | 65 | Json::Dependencies { file: packages/d2ts-benchmark/package.json } |  |  | 0.714 |
| walker |  | 6522 | 257 | Markdown::Section { file: README.md, section_index: 17, keeps_default_concavity: false } |  |  | 0.745 |
| walker |  | 6557 | 35 | Code::CodeKey { rung: Names, file: packages/d2ts-benchmark/src/base.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.745 |
| walker |  | 6614 | 57 | Code::CodeKey { rung: Decl, file: packages/d2ts-benchmark/src/base.ts, decl: 2, sub: 0, line: 19 } |  |  | 0.745 |
| ns | 6681 |  | 430 | packages/d2ts/README.md — complete heading map (1081-line API reference) | 5.2 |  | 0.716 |
| walker |  | 6690 | 76 | Code::CodeKey { rung: Decl, file: packages/d2ts-benchmark/src/base.ts, decl: 1, sub: 0, line: 4 } |  |  | 0.716 |
| ns | 6844 |  | 163 | Operator factory roster — single-stream operators | 5.3 | 2.5 | 0.707 |
| walker |  | 6855 | 165 | Code::CodeKey { rung: Decl, file: packages/d2ts-benchmark/src/base.ts, decl: 3, sub: 0, line: 27 } |  |  | 0.707 |
| walker |  | 6946 | 91 | Json::Identity { file: packages/d2mini/package.json } |  |  | 0.720 |
| walker |  | 7029 | 83 | Json::Entry { file: packages/d2mini/package.json } |  |  | 0.720 |
| walker |  | 7217 | 188 | Json::Scripts { file: packages/d2mini/package.json } |  |  | 0.720 |
| ns | 7244 |  | 400 | Operator factory roster — keyed, join, ordering and aggregate families | 5.4 | 5.3 | 0.699 |
| walker |  | 7288 | 71 | Json::Dependencies { file: packages/d2mini/package.json } |  |  | 0.699 |
| walker |  | 7382 | 94 | Json::Identity { file: packages/d2ql/package.json } |  |  | 0.712 |
| ns | 7446 |  | 202 | Operator implementation classes and the LinearUnaryOperator base | 5.5 | 5.4 | 0.702 |
| walker |  | 7554 | 172 | Json::Entry { file: packages/d2ql/package.json } |  |  | 0.702 |
| ns | 7622 |  | 176 | groupBy.ts — the AggregateFunction contract | 5.6 | 5.4 | 0.694 |
| walker |  | 7742 | 188 | Json::Scripts { file: packages/d2ql/package.json } |  |  | 0.694 |
| walker |  | 7755 | 13 | Code::CodeKey { rung: Names, file: packages/d2ql/src/select.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.694 |
| walker |  | 7815 | 60 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/select.ts, decl: 1, sub: 0, line: 7 } |  |  | 0.694 |
| walker |  | 7841 | 26 | Code::CodeKey { rung: Names, file: packages/d2ql/src/group-by.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.694 |
| ns | 7842 |  | 220 | D2QL identity and README section map | 6.1 |  | 0.686 |
| walker |  | 7881 | 40 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/group-by.ts, decl: 2, sub: 0, line: 98 } |  |  | 0.686 |
| walker |  | 7922 | 41 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/group-by.ts, decl: 1, sub: 0, line: 20 } |  |  | 0.686 |
| walker |  | 7948 | 26 | Code::CodeKey { rung: Names, file: packages/d2ql/src/joins.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.686 |
| walker |  | 7997 | 49 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/joins.ts, decl: 2, sub: 0, line: 124 } |  |  | 0.686 |
| walker |  | 8077 | 80 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/joins.ts, decl: 1, sub: 0, line: 16 } |  |  | 0.686 |
| ns | 8084 |  | 242 | D2QL "Current Features" — the supported SQL subset | 6.2 | 6.1 | 0.677 |
| walker |  | 8132 | 55 | Code::CodeKey { rung: Names, file: packages/d2ts/src/order.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.677 |
| walker |  | 8149 | 17 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/order.ts, decl: 33, sub: 0, line: 277 } |  |  | 0.677 |
| walker |  | 8351 | 202 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/order.ts, decl: 2, sub: 0, line: 29 } |  |  | 0.687 |
| ns | 8442 |  | 358 | schema.ts — the Query interface family | 6.3 |  | 0.672 |
| walker |  | 8573 | 222 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/order.ts, decl: 17, sub: 0, line: 138 } |  |  | 0.685 |
| walker |  | 8603 | 30 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/order.ts, decl: 19, sub: 0, line: 148 } |  |  | 0.685 |
| ns | 8604 |  | 162 | schema.ts — the Comparator and LogicalOperator vocabularies | 6.4 | 6.3 | 0.676 |
| walker |  | 8617 | 14 | Code::CodeKey { rung: Names, file: packages/d2ql/src/key-by.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.676 |
| walker |  | 8673 | 56 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/key-by.ts, decl: 1, sub: 0, line: 4 } |  |  | 0.676 |
| walker |  | 8687 | 14 | Code::CodeKey { rung: Names, file: packages/d2ql/src/order-by.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.676 |
| ns | 8735 |  | 131 | compileQuery — the package's single entry point | 6.5 |  | 0.671 |
| walker |  | 8754 | 67 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/order-by.ts, decl: 1, sub: 0, line: 16 } |  |  | 0.671 |
| walker |  | 8762 | 8 | Code::CodeKey { rung: Body, file: packages/d2ts-benchmark/src/base.ts, decl: 6, sub: 0, line: 48 } |  |  | 0.671 |
| walker |  | 8790 | 28 | Code::CodeKey { rung: Body, file: packages/d2ts/src/electric/index.ts, decl: 4, sub: 0, line: 227 } |  |  | 0.671 |
| walker |  | 8882 | 92 | Code::CodeKey { rung: Names, file: packages/d2ql/src/utils.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.671 |
| walker |  | 8906 | 24 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/utils.ts, decl: 3, sub: 0, line: 36 } |  |  | 0.671 |
| walker |  | 8943 | 37 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/utils.ts, decl: 6, sub: 0, line: 192 } |  |  | 0.671 |
| ns | 8978 |  | 243 | query-builder.ts — the fluent builder's complete method set | 6.6 |  | 0.660 |
| walker |  | 8987 | 44 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/utils.ts, decl: 4, sub: 0, line: 55 } |  |  | 0.660 |
| walker |  | 9018 | 31 | Code::CodeKey { rung: Names, file: packages/d2ql/src/evaluators.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.660 |
| walker |  | 9069 | 51 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/evaluators.ts, decl: 1, sub: 0, line: 14 } |  |  | 0.660 |
| ns | 9076 |  | 98 | d2mini README — how it differs from d2ts | 7.1 |  | 0.658 |
| walker |  | 9140 | 71 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/evaluators.ts, decl: 2, sub: 0, line: 126 } |  |  | 0.658 |
| ns | 9165 |  | 89 | d2mini/src/d2.ts — the versionless D2 constructor | 7.2 | 7.1 | 0.655 |
| walker |  | 9251 | 111 | Code::CodeKey { rung: Names, file: packages/d2mini/src/types.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.655 |
| walker |  | 9272 | 21 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/types.ts, decl: 7, sub: 0, line: 83 } |  |  | 0.655 |
| walker |  | 9295 | 23 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/types.ts, decl: 2, sub: 0, line: 6 } |  |  | 0.655 |
| walker |  | 9320 | 25 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/types.ts, decl: 3, sub: 0, line: 11 } |  |  | 0.655 |
| walker |  | 9358 | 38 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/types.ts, decl: 4, sub: 0, line: 16 } |  |  | 0.655 |
| ns | 9401 |  | 236 | README: using the SQLite backend | 8.1 | 1.10 | 0.649 |
| walker |  | 9437 | 79 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/types.ts, decl: 5, sub: 0, line: 21 } |  |  | 0.649 |
| walker |  | 9453 | 16 | Code::CodeKey { rung: Names, file: packages/d2mini/src/indexes.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.649 |
| ns | 9529 |  | 128 | sqlite/database.ts — the SQLiteDb driver interface | 8.2 | 8.1 | 0.643 |
| walker |  | 9637 | 184 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/indexes.ts, decl: 1, sub: 0, line: 9 } |  |  | 0.643 |
| walker |  | 9648 | 11 | Code::CodeKey { rung: Body, file: packages/d2mini/src/indexes.ts, decl: 6, sub: 0, line: 38 } |  |  | 0.643 |
| walker |  | 9659 | 11 | Code::CodeKey { rung: Body, file: packages/d2mini/src/indexes.ts, decl: 7, sub: 0, line: 42 } |  |  | 0.643 |
| walker |  | 9670 | 11 | Code::CodeKey { rung: Body, file: packages/d2mini/src/indexes.ts, decl: 9, sub: 0, line: 50 } |  |  | 0.643 |
| ns | 9683 |  | 154 | withSQLite, SQLIndex and the Electric bridge exports | 8.3 | 8.2 | 0.644 |
| ns | 9827 |  | 144 | README: what each example demonstrates | 9.1 | 1.10 | 0.647 |
| walker |  | 9943 | 273 | Code::CodeKey { rung: Names, file: packages/d2ql/src/types.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.647 |
| walker |  | 9962 | 19 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/types.ts, decl: 7, sub: 0, line: 183 } |  |  | 0.647 |
| ns | 9977 |  | 150 | examples/electric — what the demo app does | 9.2 | 2.13 | 0.644 |
| walker |  | 9981 | 19 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/types.ts, decl: 9, sub: 0, line: 192 } |  |  | 0.644 |
| walker |  | 10000 | 19 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/types.ts, decl: 11, sub: 0, line: 201 } |  |  | 0.644 |
| walker |  | 10000 | 0 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/types.ts, decl: 4, sub: 0, line: 24 } |  |  | 0.644 |

Score(3000)=0.812 I=0.897 C=0.735 ns_rows≤3K=24/55 grid(1000/1442/2080/3000/4327/6240/9000)=0.673/0.762/0.810/0.812/0.751/0.708/0.660

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
| walker |  | 259 | 69 | Fs::DirListing { dir: packages/d2ql/src } |  |  | 0.459 |
| walker |  | 268 | 9 | Fs::DirListing { dir: packages/d2ql/src/query-builder } |  |  | 0.461 |
| ns | 288 |  | 70 | README lede — what incremental execution buys you | 1.5 | 1.1 | 0.452 |
| walker |  | 298 | 30 | Fs::DirListing { dir: packages/d2mini } |  |  | 0.460 |
| walker |  | 331 | 33 | Fs::DirListing { dir: packages/d2mini/src } |  |  | 0.461 |
| ns | 387 |  | 99 | README lede — ElectricSQL ShapeStreams and pipeline type inference | 1.6 | 1.1 | 0.437 |
| walker |  | 412 | 81 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.672 |
| walker |  | 448 | 36 | Fs::DirListing { dir: packages/d2ts } |  |  | 0.683 |
| walker |  | 492 | 44 | Fs::DirListing { dir: packages/d2ts/src } |  |  | 0.691 |
| walker |  | 496 | 4 | Fs::DirListing { dir: packages/d2ts/src/electric } |  |  | 0.691 |
| walker |  | 516 | 20 | Fs::DirListing { dir: packages/d2ts/src/sqlite } |  |  | 0.691 |
| walker |  | 573 | 57 | Fs::DirListing { dir: packages/d2ts/src/sqlite/operators } |  |  | 0.696 |
| ns | 578 |  | 191 | Root package.json — identity and every workspace script | 1.7 |  | 0.571 |
| walker |  | 658 | 85 | Json::Identity { file: packages/d2ts/package.json } |  |  | 0.573 |
| ns | 728 |  | 150 | Published package identity: d2ts and d2mini | 1.8 |  | 0.547 |
| walker |  | 756 | 98 | Fs::DirListing { dir: packages/d2ts/src/operators } |  |  | 0.557 |
| walker |  | 860 | 104 | Fs::DirListing { dir: packages/d2mini/src/operators } |  |  | 0.566 |
| ns | 871 |  | 143 | Private package identity: d2ql and d2ts-benchmark | 1.9 | 1.8 | 0.519 |
| walker |  | 934 | 74 | Json::Dependencies { file: packages/d2ts/package.json } |  |  | 0.519 |
| walker |  | 969 | 35 | Fs::DirListing { dir: examples } |  |  | 0.673 |
| ns | 1032 |  | 161 | Root README section map (every H2) | 1.10 |  | 0.592 |
| walker |  | 1142 | 173 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.744 |
| ns | 1284 |  | 252 | README "Implementation Details" — provenance and the four core data structures | 1.11 | 1.10 | 0.663 |
| walker |  | 1314 | 172 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.699 |
| ns | 1392 |  | 108 | Every package's own root listing | 2.1 |  | 0.748 |
| ns | 1436 |  | 44 | packages/d2ts/src — complete module roster | 2.2 |  | 0.762 |
| ns | 1506 |  | 70 | packages/d2ts/src/index.ts — the public export barrel | 2.3 |  | 0.743 |
| ns | 1604 |  | 98 | packages/d2ts/src/operators — complete operator file roster | 2.4 |  | 0.765 |
| walker |  | 1697 | 383 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: true } |  |  | 0.767 |
| walker |  | 1721 | 24 | Fs::DirListing { dir: examples/electric } |  |  | 0.768 |
| walker |  | 1729 | 8 | Fs::DirListing { dir: examples/electric/src } |  |  | 0.768 |
| walker |  | 1742 | 13 | Fs::DirListing { dir: examples/electric/db } |  |  | 0.769 |
| walker |  | 1768 | 26 | Plaintext::Whole { file: pnpm-workspace.yaml } |  |  | 0.787 |
| walker |  | 1779 | 11 | Code::CodeKey { rung: Names, file: eslint.base.mjs, decl: 0, sub: 0, line: 0 } |  |  | 0.787 |
| ns | 1844 |  | 240 | packages/d2ts/src/operators/index.ts — exported operator set | 2.5 | 2.4 | 0.736 |
| walker |  | 1915 | 136 | Json::Scripts { file: package.json } |  |  | 0.793 |
| walker |  | 1961 | 46 | Code::CodeKey { rung: Names, file: packages/d2ts-benchmark/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.793 |
| ns | 1981 |  | 137 | packages/d2mini/src and its operators directory | 2.6 |  | 0.810 |
| walker |  | 2017 | 56 | Code::CodeKey { rung: Decl, file: packages/d2ts-benchmark/src/index.ts, decl: 2, sub: 0, line: 25 } |  |  | 0.810 |
| walker |  | 2104 | 87 | Code::CodeKey { rung: Decl, file: packages/d2ts-benchmark/src/index.ts, decl: 3, sub: 0, line: 496 } |  |  | 0.810 |
| walker |  | 2133 | 29 | Fs::DirListing { dir: packages/d2mini/tests } |  |  | 0.810 |
| walker |  | 2203 | 70 | Code::CodeKey { rung: Names, file: packages/d2ts/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.829 |
| walker |  | 2238 | 35 | Code::CodeKey { rung: Names, file: packages/d2ts/src/sqlite/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.829 |
| ns | 2246 |  | 265 | packages/d2mini export barrels | 2.7 | 2.6 | 0.783 |
| walker |  | 2285 | 47 | Code::CodeKey { rung: Names, file: packages/d2mini/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.785 |
| ns | 2324 |  | 78 | packages/d2ql/src and query-builder/ listings | 2.8 |  | 0.794 |
| ns | 2441 |  | 117 | packages/d2ql export barrels | 2.9 | 2.8 | 0.778 |
| ns | 2518 |  | 77 | packages/d2ts/src/sqlite — complete tree listing | 2.10 |  | 0.785 |
| walker |  | 2525 | 240 | Code::CodeKey { rung: Names, file: packages/d2ts/src/operators/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.833 |
| ns | 2701 |  | 183 | SQLite subpath export barrels | 2.11 | 2.10 | 0.809 |
| walker |  | 2743 | 218 | Code::CodeKey { rung: Names, file: packages/d2mini/src/operators/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.855 |
| walker |  | 2781 | 38 | Code::CodeKey { rung: Names, file: packages/d2ql/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.857 |
| walker |  | 2813 | 32 | Fs::DirListing { dir: examples/d2ql } |  |  | 0.860 |
| ns | 2886 |  | 185 | Test suite layout — immediate children of every tests/ directory | 2.12 |  | 0.809 |
| ns | 2982 |  | 96 | Example and benchmark source trees | 2.13 | 1.3 | 0.812 |
| walker |  | 2988 | 175 | Json::Entry { file: packages/d2ts/package.json } |  |  | 0.812 |
| ns | 3172 |  | 190 | Per-package scripts: build, test, lint, typecheck, format | 3.1 | 1.7 | 0.800 |
| walker |  | 3176 | 188 | Json::Scripts { file: packages/d2ts/package.json } |  |  | 0.816 |
| walker |  | 3217 | 41 | Fs::DirListing { dir: packages/d2ts/tests } |  |  | 0.824 |
| walker |  | 3258 | 41 | Fs::DirListing { dir: packages/d2ts/tests/operators-sqlite } |  |  | 0.824 |
| ns | 3259 |  | 87 | d2ts-benchmark scripts — the benchmark entry points | 3.2 | 3.1 | 0.816 |
| walker |  | 3340 | 82 | Code::CodeKey { rung: Names, file: packages/d2ts/src/electric/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.816 |
| walker |  | 3408 | 68 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/electric/index.ts, decl: 3, sub: 0, line: 222 } |  |  | 0.816 |
| ns | 3452 |  | 193 | CI pipeline stages | 3.3 |  | 0.789 |
| walker |  | 3482 | 74 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/electric/index.ts, decl: 7, sub: 0, line: 328 } |  |  | 0.789 |
| walker |  | 3563 | 81 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/electric/index.ts, decl: 4, sub: 0, line: 227 } |  |  | 0.789 |
| ns | 3656 |  | 204 | README: constructing a D2 graph and sending to an input stream | 4.1 | 1.10 | 0.779 |
| walker |  | 3684 | 121 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/electric/index.ts, decl: 2, sub: 0, line: 103 } |  |  | 0.779 |
| walker |  | 3935 | 251 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/electric/index.ts, decl: 1, sub: 0, line: 40 } |  |  | 0.779 |
| walker |  | 3956 | 21 | Code::CodeKey { rung: Names, file: packages/d2ql/src/query-builder/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.781 |
| ns | 3981 |  | 325 | d2.ts — D2Options and the complete D2 class method roster | 4.2 |  | 0.756 |
| ns | 4171 |  | 190 | README: what versions and frontiers actually mean | 4.3 | 1.10 | 0.751 |
| walker |  | 4314 | 358 | Code::CodeKey { rung: Decl, file: eslint.base.mjs, decl: 1, sub: 0, line: 6 } |  |  | 0.751 |
| walker |  | 4497 | 183 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.751 |
| ns | 4517 |  | 346 | order.ts — v() factory and the complete Version method roster | 4.4 |  | 0.724 |
| walker |  | 4612 | 115 | Fs::DirListing { dir: packages/d2ql/tests } |  |  | 0.767 |
| walker |  | 4666 | 54 | Fs::DirListing { dir: packages/d2ql/tests/query-builder } |  |  | 0.767 |
| ns | 4807 |  | 290 | order.ts — Antichain method roster and Frontier | 4.5 | 4.4 | 0.744 |
| ns | 4952 |  | 145 | README: MultiSet as a changeset, and keyed multisets | 4.6 | 1.10 | 0.741 |
| walker |  | 5016 | 350 | Code::CodeKey { rung: Decl, file: eslint.base.mjs, decl: 1, sub: 1, line: 6 } |  |  | 0.741 |
| walker |  | 5155 | 139 | Fs::DirListing { dir: packages/d2mini/tests/operators } |  |  | 0.741 |
| walker |  | 5260 | 105 | Markdown::Section { file: README.md, section_index: 18, keeps_default_concavity: false } |  |  | 0.741 |
| ns | 5310 |  | 358 | multiset.ts — MultiSetArray, KeyedData and every MultiSet method | 4.7 |  | 0.722 |
| walker |  | 5419 | 159 | Fs::DirListing { dir: packages/d2ts/tests/operators } |  |  | 0.722 |
| walker |  | 5462 | 43 | Code::CodeKey { rung: Doc, file: packages/d2ts/src/electric/index.ts, decl: 7, sub: 0, line: 328 } |  |  | 0.722 |
| ns | 5610 |  | 300 | types.ts — the operator message protocol | 4.8 |  | 0.698 |
| walker |  | 5611 | 149 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.698 |
| walker |  | 5807 | 196 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.702 |
| ns | 5868 |  | 258 | types.ts — PipedOperator and the 20-deep pipe overload wall | 4.9 | 4.8 | 0.689 |
| walker |  | 5981 | 174 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.697 |
| walker |  | 6052 | 71 | Code::CodeKey { rung: Doc, file: packages/d2ts/src/electric/index.ts, decl: 3, sub: 0, line: 222 } |  |  | 0.697 |
| walker |  | 6114 | 62 | Json::Identity { file: packages/d2ts-benchmark/package.json } |  |  | 0.701 |
| walker |  | 6201 | 87 | Json::Scripts { file: packages/d2ts-benchmark/package.json } |  |  | 0.708 |
| ns | 6251 |  | 383 | README Key Features — every operator with a one-line description | 5.1 | 1.10 | 0.714 |
| walker |  | 6266 | 65 | Json::Dependencies { file: packages/d2ts-benchmark/package.json } |  |  | 0.714 |
| walker |  | 6523 | 257 | Markdown::Section { file: README.md, section_index: 17, keeps_default_concavity: false } |  |  | 0.745 |
| walker |  | 6551 | 28 | Code::CodeKey { rung: Body, file: packages/d2ts/src/electric/index.ts, decl: 4, sub: 0, line: 227 } |  |  | 0.745 |
| walker |  | 6586 | 35 | Code::CodeKey { rung: Names, file: packages/d2ts-benchmark/src/base.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.745 |
| walker |  | 6643 | 57 | Code::CodeKey { rung: Decl, file: packages/d2ts-benchmark/src/base.ts, decl: 2, sub: 0, line: 19 } |  |  | 0.745 |
| ns | 6681 |  | 430 | packages/d2ts/README.md — complete heading map (1081-line API reference) | 5.2 |  | 0.716 |
| walker |  | 6719 | 76 | Code::CodeKey { rung: Decl, file: packages/d2ts-benchmark/src/base.ts, decl: 1, sub: 0, line: 4 } |  |  | 0.716 |
| ns | 6844 |  | 163 | Operator factory roster — single-stream operators | 5.3 | 2.5 | 0.707 |
| walker |  | 6884 | 165 | Code::CodeKey { rung: Decl, file: packages/d2ts-benchmark/src/base.ts, decl: 3, sub: 0, line: 27 } |  |  | 0.707 |
| walker |  | 6892 | 8 | Code::CodeKey { rung: Body, file: packages/d2ts-benchmark/src/base.ts, decl: 6, sub: 0, line: 48 } |  |  | 0.707 |
| walker |  | 6983 | 91 | Json::Identity { file: packages/d2mini/package.json } |  |  | 0.720 |
| walker |  | 7066 | 83 | Json::Entry { file: packages/d2mini/package.json } |  |  | 0.720 |
| ns | 7244 |  | 400 | Operator factory roster — keyed, join, ordering and aggregate families | 5.4 | 5.3 | 0.699 |
| walker |  | 7254 | 188 | Json::Scripts { file: packages/d2mini/package.json } |  |  | 0.699 |
| walker |  | 7325 | 71 | Json::Dependencies { file: packages/d2mini/package.json } |  |  | 0.699 |
| walker |  | 7419 | 94 | Json::Identity { file: packages/d2ql/package.json } |  |  | 0.712 |
| ns | 7446 |  | 202 | Operator implementation classes and the LinearUnaryOperator base | 5.5 | 5.4 | 0.702 |
| walker |  | 7591 | 172 | Json::Entry { file: packages/d2ql/package.json } |  |  | 0.702 |
| ns | 7622 |  | 176 | groupBy.ts — the AggregateFunction contract | 5.6 | 5.4 | 0.694 |
| walker |  | 7779 | 188 | Json::Scripts { file: packages/d2ql/package.json } |  |  | 0.694 |
| walker |  | 7792 | 13 | Code::CodeKey { rung: Names, file: packages/d2ql/src/select.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.694 |
| ns | 7842 |  | 220 | D2QL identity and README section map | 6.1 |  | 0.686 |
| walker |  | 7852 | 60 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/select.ts, decl: 1, sub: 0, line: 7 } |  |  | 0.686 |
| walker |  | 7878 | 26 | Code::CodeKey { rung: Names, file: packages/d2ql/src/group-by.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.686 |
| walker |  | 7918 | 40 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/group-by.ts, decl: 2, sub: 0, line: 98 } |  |  | 0.686 |
| walker |  | 7959 | 41 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/group-by.ts, decl: 1, sub: 0, line: 20 } |  |  | 0.686 |
| walker |  | 7985 | 26 | Code::CodeKey { rung: Names, file: packages/d2ql/src/joins.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.686 |
| walker |  | 8034 | 49 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/joins.ts, decl: 2, sub: 0, line: 124 } |  |  | 0.686 |
| ns | 8084 |  | 242 | D2QL "Current Features" — the supported SQL subset | 6.2 | 6.1 | 0.677 |
| walker |  | 8114 | 80 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/joins.ts, decl: 1, sub: 0, line: 16 } |  |  | 0.677 |
| walker |  | 8169 | 55 | Code::CodeKey { rung: Names, file: packages/d2ts/src/order.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.677 |
| walker |  | 8186 | 17 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/order.ts, decl: 33, sub: 0, line: 277 } |  |  | 0.677 |
| walker |  | 8194 | 8 | Code::CodeKey { rung: Body, file: packages/d2ts/src/order.ts, decl: 34, sub: 0, line: 278 } |  |  | 0.677 |
| walker |  | 8396 | 202 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/order.ts, decl: 2, sub: 0, line: 29 } |  |  | 0.687 |
| ns | 8442 |  | 358 | schema.ts — the Query interface family | 6.3 |  | 0.672 |
| ns | 8604 |  | 162 | schema.ts — the Comparator and LogicalOperator vocabularies | 6.4 | 6.3 | 0.663 |
| walker |  | 8618 | 222 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/order.ts, decl: 17, sub: 0, line: 138 } |  |  | 0.676 |
| walker |  | 8648 | 30 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/order.ts, decl: 19, sub: 0, line: 148 } |  |  | 0.676 |
| walker |  | 8662 | 14 | Code::CodeKey { rung: Names, file: packages/d2ql/src/key-by.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.676 |
| walker |  | 8718 | 56 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/key-by.ts, decl: 1, sub: 0, line: 4 } |  |  | 0.676 |
| walker |  | 8732 | 14 | Code::CodeKey { rung: Names, file: packages/d2ql/src/order-by.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.676 |
| ns | 8735 |  | 131 | compileQuery — the package's single entry point | 6.5 |  | 0.671 |
| walker |  | 8799 | 67 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/order-by.ts, decl: 1, sub: 0, line: 16 } |  |  | 0.671 |
| walker |  | 8945 | 146 | Code::CodeKey { rung: Body, file: packages/d2ts-benchmark/src/index.ts, decl: 1, sub: 0, line: 9 } |  |  | 0.671 |
| ns | 8978 |  | 243 | query-builder.ts — the fluent builder's complete method set | 6.6 |  | 0.660 |
| walker |  | 9037 | 92 | Code::CodeKey { rung: Names, file: packages/d2ql/src/utils.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.660 |
| walker |  | 9061 | 24 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/utils.ts, decl: 3, sub: 0, line: 36 } |  |  | 0.660 |
| ns | 9076 |  | 98 | d2mini README — how it differs from d2ts | 7.1 |  | 0.658 |
| walker |  | 9098 | 37 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/utils.ts, decl: 6, sub: 0, line: 192 } |  |  | 0.658 |
| walker |  | 9142 | 44 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/utils.ts, decl: 4, sub: 0, line: 55 } |  |  | 0.658 |
| ns | 9165 |  | 89 | d2mini/src/d2.ts — the versionless D2 constructor | 7.2 | 7.1 | 0.655 |
| walker |  | 9173 | 31 | Code::CodeKey { rung: Names, file: packages/d2ql/src/evaluators.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.655 |
| walker |  | 9224 | 51 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/evaluators.ts, decl: 1, sub: 0, line: 14 } |  |  | 0.655 |
| walker |  | 9295 | 71 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/evaluators.ts, decl: 2, sub: 0, line: 126 } |  |  | 0.655 |
| ns | 9401 |  | 236 | README: using the SQLite backend | 8.1 | 1.10 | 0.649 |
| walker |  | 9406 | 111 | Code::CodeKey { rung: Names, file: packages/d2mini/src/types.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.649 |
| walker |  | 9427 | 21 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/types.ts, decl: 7, sub: 0, line: 83 } |  |  | 0.649 |
| walker |  | 9450 | 23 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/types.ts, decl: 2, sub: 0, line: 6 } |  |  | 0.649 |
| walker |  | 9475 | 25 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/types.ts, decl: 3, sub: 0, line: 11 } |  |  | 0.649 |
| walker |  | 9513 | 38 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/types.ts, decl: 4, sub: 0, line: 16 } |  |  | 0.649 |
| ns | 9529 |  | 128 | sqlite/database.ts — the SQLiteDb driver interface | 8.2 | 8.1 | 0.643 |
| walker |  | 9592 | 79 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/types.ts, decl: 5, sub: 0, line: 21 } |  |  | 0.643 |
| walker |  | 9608 | 16 | Code::CodeKey { rung: Names, file: packages/d2mini/src/indexes.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.643 |
| ns | 9683 |  | 154 | withSQLite, SQLIndex and the Electric bridge exports | 8.3 | 8.2 | 0.644 |
| walker |  | 9792 | 184 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/indexes.ts, decl: 1, sub: 0, line: 9 } |  |  | 0.644 |
| walker |  | 9803 | 11 | Code::CodeKey { rung: Body, file: packages/d2mini/src/indexes.ts, decl: 6, sub: 0, line: 38 } |  |  | 0.644 |
| walker |  | 9814 | 11 | Code::CodeKey { rung: Body, file: packages/d2mini/src/indexes.ts, decl: 7, sub: 0, line: 42 } |  |  | 0.644 |
| walker |  | 9825 | 11 | Code::CodeKey { rung: Body, file: packages/d2mini/src/indexes.ts, decl: 9, sub: 0, line: 50 } |  |  | 0.644 |
| ns | 9827 |  | 144 | README: what each example demonstrates | 9.1 | 1.10 | 0.647 |
| walker |  | 9837 | 12 | Code::CodeKey { rung: Body, file: packages/d2mini/src/indexes.ts, decl: 8, sub: 0, line: 46 } |  |  | 0.647 |
| ns | 9977 |  | 150 | examples/electric — what the demo app does | 9.2 | 2.13 | 0.644 |
| walker |  | 9992 | 155 | Code::CodeKey { rung: Names, file: packages/d2ql/src/types.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.644 |

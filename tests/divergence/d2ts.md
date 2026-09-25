Score(3000)=0.802 I=0.895 C=0.720 ns_rows≤3K=24/55 grid(1000/1442/2080/3000/4327/6240/9000)=0.673/0.762/0.809/0.802/0.743/0.701/0.655

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
| walker |  | 1755 | 26 | Plaintext::Whole { file: pnpm-workspace.yaml } |  |  | 0.786 |
| walker |  | 1766 | 11 | Code::CodeKey { rung: Names, file: eslint.base.mjs, decl: 0, sub: 0, line: 0 } |  |  | 0.786 |
| ns | 1844 |  | 240 | packages/d2ts/src/operators/index.ts — exported operator set | 2.5 | 2.4 | 0.735 |
| walker |  | 1902 | 136 | Json::Scripts { file: package.json } |  |  | 0.792 |
| walker |  | 1948 | 46 | Code::CodeKey { rung: Names, file: packages/d2ts-benchmark/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.792 |
| ns | 1981 |  | 137 | packages/d2mini/src and its operators directory | 2.6 |  | 0.809 |
| walker |  | 2004 | 56 | Code::CodeKey { rung: Decl, file: packages/d2ts-benchmark/src/index.ts, decl: 2, sub: 0, line: 25 } |  |  | 0.809 |
| walker |  | 2091 | 87 | Code::CodeKey { rung: Decl, file: packages/d2ts-benchmark/src/index.ts, decl: 3, sub: 0, line: 496 } |  |  | 0.809 |
| walker |  | 2120 | 29 | Fs::DirListing { dir: packages/d2mini/tests } |  |  | 0.809 |
| walker |  | 2190 | 70 | Code::CodeKey { rung: Names, file: packages/d2ts/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.828 |
| walker |  | 2225 | 35 | Code::CodeKey { rung: Names, file: packages/d2ts/src/sqlite/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.828 |
| ns | 2246 |  | 265 | packages/d2mini export barrels | 2.7 | 2.6 | 0.782 |
| walker |  | 2272 | 47 | Code::CodeKey { rung: Names, file: packages/d2mini/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.784 |
| ns | 2324 |  | 78 | packages/d2ql/src and query-builder/ listings | 2.8 |  | 0.793 |
| ns | 2441 |  | 117 | packages/d2ql export barrels | 2.9 | 2.8 | 0.777 |
| walker |  | 2512 | 240 | Code::CodeKey { rung: Names, file: packages/d2ts/src/operators/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.828 |
| ns | 2518 |  | 77 | packages/d2ts/src/sqlite — complete tree listing | 2.10 |  | 0.832 |
| ns | 2701 |  | 183 | SQLite subpath export barrels | 2.11 | 2.10 | 0.809 |
| walker |  | 2730 | 218 | Code::CodeKey { rung: Names, file: packages/d2mini/src/operators/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.855 |
| walker |  | 2768 | 38 | Code::CodeKey { rung: Names, file: packages/d2ql/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.857 |
| walker |  | 2800 | 32 | Fs::DirListing { dir: examples/d2ql } |  |  | 0.859 |
| ns | 2886 |  | 185 | Test suite layout — immediate children of every tests/ directory | 2.12 |  | 0.808 |
| walker |  | 2975 | 175 | Json::Entry { file: packages/d2ts/package.json } |  |  | 0.808 |
| ns | 2982 |  | 96 | Example and benchmark source trees | 2.13 | 1.3 | 0.802 |
| walker |  | 3163 | 188 | Json::Scripts { file: packages/d2ts/package.json } |  |  | 0.804 |
| ns | 3172 |  | 190 | Per-package scripts: build, test, lint, typecheck, format | 3.1 | 1.7 | 0.806 |
| walker |  | 3204 | 41 | Fs::DirListing { dir: packages/d2ts/tests } |  |  | 0.815 |
| walker |  | 3245 | 41 | Fs::DirListing { dir: packages/d2ts/tests/operators-sqlite } |  |  | 0.815 |
| ns | 3259 |  | 87 | d2ts-benchmark scripts — the benchmark entry points | 3.2 | 3.1 | 0.807 |
| walker |  | 3327 | 82 | Code::CodeKey { rung: Names, file: packages/d2ts/src/electric/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.807 |
| walker |  | 3395 | 68 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/electric/index.ts, decl: 3, sub: 0, line: 222 } |  |  | 0.807 |
| ns | 3452 |  | 193 | CI pipeline stages | 3.3 |  | 0.780 |
| walker |  | 3469 | 74 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/electric/index.ts, decl: 7, sub: 0, line: 328 } |  |  | 0.780 |
| walker |  | 3550 | 81 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/electric/index.ts, decl: 4, sub: 0, line: 227 } |  |  | 0.780 |
| ns | 3656 |  | 204 | README: constructing a D2 graph and sending to an input stream | 4.1 | 1.10 | 0.770 |
| walker |  | 3671 | 121 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/electric/index.ts, decl: 2, sub: 0, line: 103 } |  |  | 0.770 |
| walker |  | 3922 | 251 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/electric/index.ts, decl: 1, sub: 0, line: 40 } |  |  | 0.771 |
| walker |  | 3943 | 21 | Code::CodeKey { rung: Names, file: packages/d2ql/src/query-builder/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.773 |
| ns | 3981 |  | 325 | d2.ts — D2Options and the complete D2 class method roster | 4.2 |  | 0.747 |
| ns | 4171 |  | 190 | README: what versions and frontiers actually mean | 4.3 | 1.10 | 0.743 |
| walker |  | 4301 | 358 | Code::CodeKey { rung: Decl, file: eslint.base.mjs, decl: 1, sub: 0, line: 6 } |  |  | 0.743 |
| walker |  | 4484 | 183 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.743 |
| ns | 4517 |  | 346 | order.ts — v() factory and the complete Version method roster | 4.4 |  | 0.716 |
| walker |  | 4599 | 115 | Fs::DirListing { dir: packages/d2ql/tests } |  |  | 0.759 |
| walker |  | 4653 | 54 | Fs::DirListing { dir: packages/d2ql/tests/query-builder } |  |  | 0.759 |
| ns | 4807 |  | 290 | order.ts — Antichain method roster and Frontier | 4.5 | 4.4 | 0.737 |
| ns | 4952 |  | 145 | README: MultiSet as a changeset, and keyed multisets | 4.6 | 1.10 | 0.733 |
| walker |  | 5003 | 350 | Code::CodeKey { rung: Decl, file: eslint.base.mjs, decl: 1, sub: 1, line: 6 } |  |  | 0.733 |
| walker |  | 5142 | 139 | Fs::DirListing { dir: packages/d2mini/tests/operators } |  |  | 0.733 |
| walker |  | 5247 | 105 | Markdown::Section { file: README.md, section_index: 18, keeps_default_concavity: false } |  |  | 0.733 |
| ns | 5310 |  | 358 | multiset.ts — MultiSetArray, KeyedData and every MultiSet method | 4.7 |  | 0.715 |
| walker |  | 5406 | 159 | Fs::DirListing { dir: packages/d2ts/tests/operators } |  |  | 0.715 |
| walker |  | 5449 | 43 | Code::CodeKey { rung: Doc, file: packages/d2ts/src/electric/index.ts, decl: 7, sub: 0, line: 328 } |  |  | 0.715 |
| walker |  | 5598 | 149 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.715 |
| ns | 5610 |  | 300 | types.ts — the operator message protocol | 4.8 |  | 0.691 |
| walker |  | 5794 | 196 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.695 |
| ns | 5868 |  | 258 | types.ts — PipedOperator and the 20-deep pipe overload wall | 4.9 | 4.8 | 0.682 |
| walker |  | 5968 | 174 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.690 |
| walker |  | 6039 | 71 | Code::CodeKey { rung: Doc, file: packages/d2ts/src/electric/index.ts, decl: 3, sub: 0, line: 222 } |  |  | 0.690 |
| walker |  | 6101 | 62 | Json::Identity { file: packages/d2ts-benchmark/package.json } |  |  | 0.694 |
| walker |  | 6188 | 87 | Json::Scripts { file: packages/d2ts-benchmark/package.json } |  |  | 0.701 |
| ns | 6251 |  | 383 | README Key Features — every operator with a one-line description | 5.1 | 1.10 | 0.708 |
| walker |  | 6253 | 65 | Json::Dependencies { file: packages/d2ts-benchmark/package.json } |  |  | 0.708 |
| walker |  | 6510 | 257 | Markdown::Section { file: README.md, section_index: 17, keeps_default_concavity: false } |  |  | 0.738 |
| walker |  | 6538 | 28 | Code::CodeKey { rung: Body, file: packages/d2ts/src/electric/index.ts, decl: 4, sub: 0, line: 227 } |  |  | 0.738 |
| walker |  | 6573 | 35 | Code::CodeKey { rung: Names, file: packages/d2ts-benchmark/src/base.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.738 |
| walker |  | 6630 | 57 | Code::CodeKey { rung: Decl, file: packages/d2ts-benchmark/src/base.ts, decl: 2, sub: 0, line: 19 } |  |  | 0.738 |
| ns | 6681 |  | 430 | packages/d2ts/README.md — complete heading map (1081-line API reference) | 5.2 |  | 0.710 |
| walker |  | 6706 | 76 | Code::CodeKey { rung: Decl, file: packages/d2ts-benchmark/src/base.ts, decl: 1, sub: 0, line: 4 } |  |  | 0.710 |
| ns | 6844 |  | 163 | Operator factory roster — single-stream operators | 5.3 | 2.5 | 0.701 |
| walker |  | 6871 | 165 | Code::CodeKey { rung: Decl, file: packages/d2ts-benchmark/src/base.ts, decl: 3, sub: 0, line: 27 } |  |  | 0.701 |
| walker |  | 6879 | 8 | Code::CodeKey { rung: Body, file: packages/d2ts-benchmark/src/base.ts, decl: 6, sub: 0, line: 48 } |  |  | 0.701 |
| walker |  | 6970 | 91 | Json::Identity { file: packages/d2mini/package.json } |  |  | 0.714 |
| walker |  | 7053 | 83 | Json::Entry { file: packages/d2mini/package.json } |  |  | 0.714 |
| walker |  | 7241 | 188 | Json::Scripts { file: packages/d2mini/package.json } |  |  | 0.714 |
| ns | 7244 |  | 400 | Operator factory roster — keyed, join, ordering and aggregate families | 5.4 | 5.3 | 0.693 |
| walker |  | 7312 | 71 | Json::Dependencies { file: packages/d2mini/package.json } |  |  | 0.693 |
| walker |  | 7406 | 94 | Json::Identity { file: packages/d2ql/package.json } |  |  | 0.706 |
| ns | 7446 |  | 202 | Operator implementation classes and the LinearUnaryOperator base | 5.5 | 5.4 | 0.696 |
| walker |  | 7578 | 172 | Json::Entry { file: packages/d2ql/package.json } |  |  | 0.696 |
| ns | 7622 |  | 176 | groupBy.ts — the AggregateFunction contract | 5.6 | 5.4 | 0.688 |
| walker |  | 7766 | 188 | Json::Scripts { file: packages/d2ql/package.json } |  |  | 0.688 |
| walker |  | 7779 | 13 | Code::CodeKey { rung: Names, file: packages/d2ql/src/select.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.688 |
| walker |  | 7839 | 60 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/select.ts, decl: 1, sub: 0, line: 7 } |  |  | 0.688 |
| ns | 7842 |  | 220 | D2QL identity and README section map | 6.1 |  | 0.680 |
| walker |  | 7865 | 26 | Code::CodeKey { rung: Names, file: packages/d2ql/src/group-by.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.680 |
| walker |  | 7905 | 40 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/group-by.ts, decl: 2, sub: 0, line: 98 } |  |  | 0.680 |
| walker |  | 7946 | 41 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/group-by.ts, decl: 1, sub: 0, line: 20 } |  |  | 0.680 |
| walker |  | 7972 | 26 | Code::CodeKey { rung: Names, file: packages/d2ql/src/joins.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.680 |
| walker |  | 8021 | 49 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/joins.ts, decl: 2, sub: 0, line: 124 } |  |  | 0.680 |
| ns | 8084 |  | 242 | D2QL "Current Features" — the supported SQL subset | 6.2 | 6.1 | 0.671 |
| walker |  | 8101 | 80 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/joins.ts, decl: 1, sub: 0, line: 16 } |  |  | 0.671 |
| walker |  | 8156 | 55 | Code::CodeKey { rung: Names, file: packages/d2ts/src/order.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.672 |
| walker |  | 8173 | 17 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/order.ts, decl: 33, sub: 0, line: 277 } |  |  | 0.672 |
| walker |  | 8181 | 8 | Code::CodeKey { rung: Body, file: packages/d2ts/src/order.ts, decl: 34, sub: 0, line: 278 } |  |  | 0.672 |
| walker |  | 8383 | 202 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/order.ts, decl: 2, sub: 0, line: 29 } |  |  | 0.681 |
| ns | 8442 |  | 358 | schema.ts — the Query interface family | 6.3 |  | 0.667 |
| ns | 8604 |  | 162 | schema.ts — the Comparator and LogicalOperator vocabularies | 6.4 | 6.3 | 0.658 |
| walker |  | 8605 | 222 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/order.ts, decl: 17, sub: 0, line: 138 } |  |  | 0.671 |
| walker |  | 8635 | 30 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/order.ts, decl: 19, sub: 0, line: 148 } |  |  | 0.671 |
| walker |  | 8649 | 14 | Code::CodeKey { rung: Names, file: packages/d2ql/src/key-by.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.671 |
| walker |  | 8705 | 56 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/key-by.ts, decl: 1, sub: 0, line: 4 } |  |  | 0.671 |
| walker |  | 8719 | 14 | Code::CodeKey { rung: Names, file: packages/d2ql/src/order-by.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.671 |
| ns | 8735 |  | 131 | compileQuery — the package's single entry point | 6.5 |  | 0.665 |
| walker |  | 8786 | 67 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/order-by.ts, decl: 1, sub: 0, line: 16 } |  |  | 0.665 |
| walker |  | 8932 | 146 | Code::CodeKey { rung: Body, file: packages/d2ts-benchmark/src/index.ts, decl: 1, sub: 0, line: 9 } |  |  | 0.665 |
| ns | 8978 |  | 243 | query-builder.ts — the fluent builder's complete method set | 6.6 |  | 0.655 |
| walker |  | 9024 | 92 | Code::CodeKey { rung: Names, file: packages/d2ql/src/utils.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.655 |
| walker |  | 9048 | 24 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/utils.ts, decl: 3, sub: 0, line: 36 } |  |  | 0.655 |
| ns | 9076 |  | 98 | d2mini README — how it differs from d2ts | 7.1 |  | 0.653 |
| walker |  | 9085 | 37 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/utils.ts, decl: 6, sub: 0, line: 192 } |  |  | 0.653 |
| walker |  | 9129 | 44 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/utils.ts, decl: 4, sub: 0, line: 55 } |  |  | 0.653 |
| walker |  | 9160 | 31 | Code::CodeKey { rung: Names, file: packages/d2ql/src/evaluators.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.653 |
| ns | 9165 |  | 89 | d2mini/src/d2.ts — the versionless D2 constructor | 7.2 | 7.1 | 0.649 |
| walker |  | 9211 | 51 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/evaluators.ts, decl: 1, sub: 0, line: 14 } |  |  | 0.649 |
| walker |  | 9282 | 71 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/evaluators.ts, decl: 2, sub: 0, line: 126 } |  |  | 0.649 |
| walker |  | 9393 | 111 | Code::CodeKey { rung: Names, file: packages/d2mini/src/types.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.649 |
| ns | 9401 |  | 236 | README: using the SQLite backend | 8.1 | 1.10 | 0.644 |
| walker |  | 9414 | 21 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/types.ts, decl: 7, sub: 0, line: 83 } |  |  | 0.644 |
| walker |  | 9437 | 23 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/types.ts, decl: 2, sub: 0, line: 6 } |  |  | 0.644 |
| walker |  | 9462 | 25 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/types.ts, decl: 3, sub: 0, line: 11 } |  |  | 0.644 |
| walker |  | 9500 | 38 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/types.ts, decl: 4, sub: 0, line: 16 } |  |  | 0.644 |
| ns | 9529 |  | 128 | sqlite/database.ts — the SQLiteDb driver interface | 8.2 | 8.1 | 0.638 |
| walker |  | 9579 | 79 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/types.ts, decl: 5, sub: 0, line: 21 } |  |  | 0.638 |
| walker |  | 9595 | 16 | Code::CodeKey { rung: Names, file: packages/d2mini/src/indexes.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.638 |
| ns | 9683 |  | 154 | withSQLite, SQLIndex and the Electric bridge exports | 8.3 | 8.2 | 0.639 |
| walker |  | 9779 | 184 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/indexes.ts, decl: 1, sub: 0, line: 9 } |  |  | 0.639 |
| walker |  | 9790 | 11 | Code::CodeKey { rung: Body, file: packages/d2mini/src/indexes.ts, decl: 6, sub: 0, line: 38 } |  |  | 0.639 |
| walker |  | 9801 | 11 | Code::CodeKey { rung: Body, file: packages/d2mini/src/indexes.ts, decl: 7, sub: 0, line: 42 } |  |  | 0.639 |
| walker |  | 9812 | 11 | Code::CodeKey { rung: Body, file: packages/d2mini/src/indexes.ts, decl: 9, sub: 0, line: 50 } |  |  | 0.639 |
| walker |  | 9824 | 12 | Code::CodeKey { rung: Body, file: packages/d2mini/src/indexes.ts, decl: 8, sub: 0, line: 46 } |  |  | 0.639 |
| ns | 9827 |  | 144 | README: what each example demonstrates | 9.1 | 1.10 | 0.642 |
| ns | 9977 |  | 150 | examples/electric — what the demo app does | 9.2 | 2.13 | 0.639 |
| walker |  | 9996 | 172 | Code::CodeKey { rung: Names, file: packages/d2ql/src/types.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.639 |

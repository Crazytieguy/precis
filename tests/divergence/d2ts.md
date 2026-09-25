Score(3000)=0.730 I=0.869 C=0.614 ns_rows≤3K=24/55 grid(1000/1442/2080/3000/4327/6240/9000)=0.669/0.738/0.822/0.730/0.714/0.616/0.570

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
| walker |  | 496 | 74 | Json::Dependencies { file: packages/d2ts/package.json } |  |  | 0.678 |
| walker |  | 529 | 33 | Fs::DirListing { dir: packages/d2mini/src } |  |  | 0.679 |
| ns | 578 |  | 191 | Root package.json — identity and every workspace script | 1.7 |  | 0.557 |
| walker |  | 633 | 104 | Fs::DirListing { dir: packages/d2mini/src/operators } |  |  | 0.567 |
| walker |  | 677 | 44 | Fs::DirListing { dir: packages/d2ts/src } |  |  | 0.572 |
| walker |  | 681 | 4 | Fs::DirListing { dir: packages/d2ts/src/electric } |  |  | 0.572 |
| walker |  | 701 | 20 | Fs::DirListing { dir: packages/d2ts/src/sqlite } |  |  | 0.573 |
| ns | 728 |  | 150 | Published package identity: d2ts and d2mini | 1.8 |  | 0.547 |
| walker |  | 799 | 98 | Fs::DirListing { dir: packages/d2ts/src/operators } |  |  | 0.557 |
| walker |  | 834 | 35 | Fs::DirListing { dir: examples } |  |  | 0.723 |
| ns | 871 |  | 143 | Private package identity: d2ql and d2ts-benchmark | 1.9 | 1.8 | 0.663 |
| walker |  | 893 | 59 | Markdown::ReadmeHeadline { file: packages/d2mini/README.md } |  |  | 0.664 |
| walker |  | 962 | 69 | Fs::DirListing { dir: packages/d2ql/src } |  |  | 0.668 |
| walker |  | 971 | 9 | Fs::DirListing { dir: packages/d2ql/src/query-builder } |  |  | 0.669 |
| ns | 1032 |  | 161 | Root README section map (every H2) | 1.10 |  | 0.589 |
| walker |  | 1048 | 77 | Markdown::ReadmeHeadline { file: packages/d2ql/README.md } |  |  | 0.589 |
| walker |  | 1081 | 33 | Markdown::Section { file: packages/d2ql/README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.589 |
| walker |  | 1159 | 78 | Markdown::ReadmeHeadline { file: packages/d2ts/README.md } |  |  | 0.589 |
| walker |  | 1198 | 39 | Markdown::Section { file: packages/d2mini/README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.589 |
| ns | 1284 |  | 252 | README "Implementation Details" — provenance and the four core data structures | 1.11 | 1.10 | 0.525 |
| walker |  | 1371 | 173 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.660 |
| ns | 1392 |  | 108 | Every package's own root listing | 2.1 |  | 0.716 |
| ns | 1436 |  | 44 | packages/d2ts/src — complete module roster | 2.2 |  | 0.732 |
| ns | 1506 |  | 70 | packages/d2ts/src/index.ts — the public export barrel | 2.3 |  | 0.714 |
| walker |  | 1543 | 172 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.739 |
| walker |  | 1567 | 24 | Fs::DirListing { dir: examples/electric } |  |  | 0.740 |
| walker |  | 1575 | 8 | Fs::DirListing { dir: examples/electric/src } |  |  | 0.740 |
| walker |  | 1601 | 26 | Plaintext::Whole { file: pnpm-workspace.yaml } |  |  | 0.760 |
| ns | 1604 |  | 98 | packages/d2ts/src/operators — complete operator file roster | 2.4 |  | 0.780 |
| walker |  | 1612 | 11 | Code::CodeKey { rung: Names, file: eslint.base.mjs, decl: 0, sub: 0, line: 0 } |  |  | 0.780 |
| walker |  | 1748 | 136 | Json::Scripts { file: package.json } |  |  | 0.841 |
| walker |  | 1777 | 29 | Fs::DirListing { dir: packages/d2mini/tests } |  |  | 0.841 |
| ns | 1844 |  | 240 | packages/d2ts/src/operators/index.ts — exported operator set | 2.5 | 2.4 | 0.787 |
| walker |  | 1847 | 70 | Code::CodeKey { rung: Names, file: packages/d2ts/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.809 |
| walker |  | 1894 | 47 | Code::CodeKey { rung: Names, file: packages/d2mini/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.809 |
| walker |  | 1932 | 38 | Code::CodeKey { rung: Names, file: packages/d2ql/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.809 |
| walker |  | 1970 | 38 | Code::CodeKey { rung: Names, file: packages/d2ts-benchmark/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.809 |
| ns | 1981 |  | 137 | packages/d2mini/src and its operators directory | 2.6 |  | 0.822 |
| walker |  | 2026 | 56 | Code::CodeKey { rung: Decl, file: packages/d2ts-benchmark/src/index.ts, decl: 2, sub: 0, line: 25 } |  |  | 0.822 |
| walker |  | 2113 | 87 | Code::CodeKey { rung: Decl, file: packages/d2ts-benchmark/src/index.ts, decl: 3, sub: 0, line: 496 } |  |  | 0.822 |
| ns | 2246 |  | 265 | packages/d2mini export barrels | 2.7 | 2.6 | 0.778 |
| ns | 2324 |  | 78 | packages/d2ql/src and query-builder/ listings | 2.8 |  | 0.787 |
| ns | 2441 |  | 117 | packages/d2ql export barrels | 2.9 | 2.8 | 0.773 |
| walker |  | 2496 | 383 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: true } |  |  | 0.776 |
| ns | 2518 |  | 77 | packages/d2ts/src/sqlite — complete tree listing | 2.10 |  | 0.751 |
| walker |  | 2528 | 32 | Fs::DirListing { dir: examples/d2ql } |  |  | 0.753 |
| ns | 2701 |  | 183 | SQLite subpath export barrels | 2.11 | 2.10 | 0.731 |
| walker |  | 2703 | 175 | Json::Entry { file: packages/d2ts/package.json } |  |  | 0.731 |
| ns | 2886 |  | 185 | Test suite layout — immediate children of every tests/ directory | 2.12 |  | 0.688 |
| walker |  | 2891 | 188 | Json::Scripts { file: packages/d2ts/package.json } |  |  | 0.690 |
| walker |  | 2948 | 57 | Fs::DirListing { dir: packages/d2ts/src/sqlite/operators } |  |  | 0.722 |
| ns | 2982 |  | 96 | Example and benchmark source trees | 2.13 | 1.3 | 0.721 |
| walker |  | 2989 | 41 | Fs::DirListing { dir: packages/d2ts/tests } |  |  | 0.730 |
| walker |  | 3030 | 41 | Fs::DirListing { dir: packages/d2ts/tests/operators-sqlite } |  |  | 0.730 |
| ns | 3172 |  | 190 | Per-package scripts: build, test, lint, typecheck, format | 3.1 | 1.7 | 0.735 |
| ns | 3259 |  | 87 | d2ts-benchmark scripts — the benchmark entry points | 3.2 | 3.1 | 0.728 |
| walker |  | 3388 | 358 | Code::CodeKey { rung: Decl, file: eslint.base.mjs, decl: 1, sub: 0, line: 6 } |  |  | 0.728 |
| ns | 3452 |  | 193 | CI pipeline stages | 3.3 |  | 0.704 |
| walker |  | 3503 | 115 | Fs::DirListing { dir: packages/d2ql/tests } |  |  | 0.752 |
| walker |  | 3557 | 54 | Fs::DirListing { dir: packages/d2ql/tests/query-builder } |  |  | 0.752 |
| ns | 3656 |  | 204 | README: constructing a D2 graph and sending to an input stream | 4.1 | 1.10 | 0.743 |
| walker |  | 3907 | 350 | Code::CodeKey { rung: Decl, file: eslint.base.mjs, decl: 1, sub: 1, line: 6 } |  |  | 0.743 |
| ns | 3981 |  | 325 | d2.ts — D2Options and the complete D2 class method roster | 4.2 |  | 0.718 |
| walker |  | 4046 | 139 | Fs::DirListing { dir: packages/d2mini/tests/operators } |  |  | 0.718 |
| ns | 4171 |  | 190 | README: what versions and frontiers actually mean | 4.3 | 1.10 | 0.714 |
| walker |  | 4205 | 159 | Fs::DirListing { dir: packages/d2ts/tests/operators } |  |  | 0.714 |
| walker |  | 4351 | 146 | Code::CodeKey { rung: Body, file: packages/d2ts-benchmark/src/index.ts, decl: 1, sub: 0, line: 9 } |  |  | 0.714 |
| walker |  | 4381 | 30 | Markdown::Section { file: packages/d2ql/README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.714 |
| ns | 4517 |  | 346 | order.ts — v() factory and the complete Version method roster | 4.4 |  | 0.689 |
| walker |  | 4564 | 183 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.689 |
| walker |  | 4739 | 175 | Markdown::HeadingsOutline { file: packages/d2ts/README.md } |  |  | 0.689 |
| ns | 4807 |  | 290 | order.ts — Antichain method roster and Frontier | 4.5 | 4.4 | 0.669 |
| walker |  | 4911 | 172 | Markdown::Section { file: packages/d2ts/README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.669 |
| ns | 4952 |  | 145 | README: MultiSet as a changeset, and keyed multisets | 4.6 | 1.10 | 0.666 |
| walker |  | 5016 | 105 | Markdown::Section { file: README.md, section_index: 18, keeps_default_concavity: false } |  |  | 0.666 |
| walker |  | 5165 | 149 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.667 |
| ns | 5310 |  | 358 | multiset.ts — MultiSetArray, KeyedData and every MultiSet method | 4.7 |  | 0.650 |
| ns | 5610 |  | 300 | types.ts — the operator message protocol | 4.8 |  | 0.628 |
| walker |  | 5707 | 542 | Markdown::Section { file: packages/d2ql/README.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.628 |
| ns | 5868 |  | 258 | types.ts — PipedOperator and the 20-deep pipe overload wall | 4.9 | 4.8 | 0.616 |
| ns | 6251 |  | 383 | README Key Features — every operator with a one-line description | 5.1 | 1.10 | 0.626 |
| walker |  | 6282 | 575 | Markdown::Section { file: packages/d2mini/README.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.626 |
| walker |  | 6344 | 62 | Json::Identity { file: packages/d2ts-benchmark/package.json } |  |  | 0.629 |
| walker |  | 6431 | 87 | Json::Scripts { file: packages/d2ts-benchmark/package.json } |  |  | 0.637 |
| walker |  | 6496 | 65 | Json::Dependencies { file: packages/d2ts-benchmark/package.json } |  |  | 0.637 |
| walker |  | 6653 | 157 | Json::IdentityMeta { file: packages/d2ts/package.json } |  |  | 0.637 |
| ns | 6681 |  | 430 | packages/d2ts/README.md — complete heading map (1081-line API reference) | 5.2 |  | 0.622 |
| walker |  | 6688 | 35 | Code::CodeKey { rung: Names, file: packages/d2ts-benchmark/src/base.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.622 |
| walker |  | 6745 | 57 | Code::CodeKey { rung: Decl, file: packages/d2ts-benchmark/src/base.ts, decl: 2, sub: 0, line: 19 } |  |  | 0.622 |
| walker |  | 6821 | 76 | Code::CodeKey { rung: Decl, file: packages/d2ts-benchmark/src/base.ts, decl: 1, sub: 0, line: 4 } |  |  | 0.622 |
| ns | 6844 |  | 163 | Operator factory roster — single-stream operators | 5.3 | 2.5 | 0.614 |
| walker |  | 6986 | 165 | Code::CodeKey { rung: Decl, file: packages/d2ts-benchmark/src/base.ts, decl: 3, sub: 0, line: 27 } |  |  | 0.614 |
| walker |  | 6994 | 8 | Code::CodeKey { rung: Body, file: packages/d2ts-benchmark/src/base.ts, decl: 6, sub: 0, line: 48 } |  |  | 0.614 |
| walker |  | 7011 | 17 | Code::CodeKey { rung: Body, file: packages/d2ts-benchmark/src/base.ts, decl: 5, sub: 0, line: 44 } |  |  | 0.614 |
| walker |  | 7207 | 196 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.618 |
| ns | 7244 |  | 400 | Operator factory roster — keyed, join, ordering and aggregate families | 5.4 | 5.3 | 0.599 |
| walker |  | 7381 | 174 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.607 |
| ns | 7446 |  | 202 | Operator implementation classes and the LinearUnaryOperator base | 5.5 | 5.4 | 0.598 |
| walker |  | 7472 | 91 | Json::Identity { file: packages/d2mini/package.json } |  |  | 0.611 |
| walker |  | 7555 | 83 | Json::Entry { file: packages/d2mini/package.json } |  |  | 0.611 |
| ns | 7622 |  | 176 | groupBy.ts — the AggregateFunction contract | 5.6 | 5.4 | 0.603 |
| walker |  | 7626 | 71 | Json::Dependencies { file: packages/d2mini/package.json } |  |  | 0.603 |
| walker |  | 7814 | 188 | Json::Scripts { file: packages/d2mini/package.json } |  |  | 0.603 |
| ns | 7842 |  | 220 | D2QL identity and README section map | 6.1 |  | 0.600 |
| walker |  | 7971 | 157 | Json::IdentityMeta { file: packages/d2mini/package.json } |  |  | 0.600 |
| walker |  | 8065 | 94 | Json::Identity { file: packages/d2ql/package.json } |  |  | 0.613 |
| ns | 8084 |  | 242 | D2QL "Current Features" — the supported SQL subset | 6.2 | 6.1 | 0.605 |
| walker |  | 8237 | 172 | Json::Entry { file: packages/d2ql/package.json } |  |  | 0.605 |
| walker |  | 8425 | 188 | Json::Scripts { file: packages/d2ql/package.json } |  |  | 0.605 |
| ns | 8442 |  | 358 | schema.ts — the Query interface family | 6.3 |  | 0.592 |
| walker |  | 8582 | 157 | Json::IdentityMeta { file: packages/d2ql/package.json } |  |  | 0.592 |
| walker |  | 8595 | 13 | Code::CodeKey { rung: Names, file: packages/d2ql/src/select.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.592 |
| ns | 8604 |  | 162 | schema.ts — the Comparator and LogicalOperator vocabularies | 6.4 | 6.3 | 0.584 |
| walker |  | 8655 | 60 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/select.ts, decl: 1, sub: 0, line: 7 } |  |  | 0.584 |
| walker |  | 8681 | 26 | Code::CodeKey { rung: Names, file: packages/d2ql/src/group-by.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.584 |
| walker |  | 8721 | 40 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/group-by.ts, decl: 2, sub: 0, line: 98 } |  |  | 0.584 |
| ns | 8735 |  | 131 | compileQuery — the package's single entry point | 6.5 |  | 0.579 |
| walker |  | 8762 | 41 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/group-by.ts, decl: 1, sub: 0, line: 20 } |  |  | 0.579 |
| walker |  | 8789 | 27 | Code::CodeKey { rung: Doc, file: packages/d2ql/src/group-by.ts, decl: 1, sub: 0, line: 20 } |  |  | 0.579 |
| walker |  | 8817 | 28 | Code::CodeKey { rung: Doc, file: packages/d2ql/src/group-by.ts, decl: 2, sub: 0, line: 98 } |  |  | 0.579 |
| walker |  | 8843 | 26 | Code::CodeKey { rung: Names, file: packages/d2ql/src/joins.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.579 |
| walker |  | 8892 | 49 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/joins.ts, decl: 2, sub: 0, line: 124 } |  |  | 0.579 |
| walker |  | 8972 | 80 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/joins.ts, decl: 1, sub: 0, line: 16 } |  |  | 0.579 |
| ns | 8978 |  | 243 | query-builder.ts — the fluent builder's complete method set | 6.6 |  | 0.570 |
| walker |  | 8995 | 23 | Code::CodeKey { rung: Doc, file: packages/d2ql/src/joins.ts, decl: 1, sub: 0, line: 16 } |  |  | 0.570 |
| walker |  | 9018 | 23 | Code::CodeKey { rung: Doc, file: packages/d2ql/src/joins.ts, decl: 2, sub: 0, line: 124 } |  |  | 0.570 |
| walker |  | 9073 | 55 | Code::CodeKey { rung: Names, file: packages/d2ts/src/order.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.570 |
| ns | 9076 |  | 98 | d2mini README — how it differs from d2ts | 7.1 |  | 0.571 |
| walker |  | 9090 | 17 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/order.ts, decl: 33, sub: 0, line: 277 } |  |  | 0.571 |
| walker |  | 9098 | 8 | Code::CodeKey { rung: Body, file: packages/d2ts/src/order.ts, decl: 34, sub: 0, line: 278 } |  |  | 0.571 |
| ns | 9165 |  | 89 | d2mini/src/d2.ts — the versionless D2 constructor | 7.2 | 7.1 | 0.568 |
| walker |  | 9300 | 202 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/order.ts, decl: 2, sub: 0, line: 29 } |  |  | 0.578 |
| walker |  | 9310 | 10 | Code::CodeKey { rung: Body, file: packages/d2ts/src/order.ts, decl: 16, sub: 0, line: 128 } |  |  | 0.578 |
| walker |  | 9321 | 11 | Code::CodeKey { rung: Body, file: packages/d2ts/src/order.ts, decl: 6, sub: 0, line: 50 } |  |  | 0.578 |
| walker |  | 9335 | 14 | Code::CodeKey { rung: Body, file: packages/d2ts/src/order.ts, decl: 5, sub: 0, line: 46 } |  |  | 0.578 |
| ns | 9401 |  | 236 | README: using the SQLite backend | 8.1 | 1.10 | 0.572 |
| ns | 9529 |  | 128 | sqlite/database.ts — the SQLiteDb driver interface | 8.2 | 8.1 | 0.567 |
| walker |  | 9557 | 222 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/order.ts, decl: 17, sub: 0, line: 138 } |  |  | 0.580 |
| walker |  | 9587 | 30 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/order.ts, decl: 19, sub: 0, line: 148 } |  |  | 0.580 |
| walker |  | 9625 | 38 | Code::CodeKey { rung: Doc, file: packages/d2ts/src/order.ts, decl: 17, sub: 0, line: 138 } |  |  | 0.585 |
| walker |  | 9682 | 57 | Code::CodeKey { rung: Doc, file: packages/d2ts/src/order.ts, decl: 1, sub: 0, line: 10 } |  |  | 0.590 |
| ns | 9683 |  | 154 | withSQLite, SQLIndex and the Electric bridge exports | 8.3 | 8.2 | 0.586 |
| walker |  | 9696 | 14 | Code::CodeKey { rung: Names, file: packages/d2ql/src/key-by.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.586 |
| walker |  | 9752 | 56 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/key-by.ts, decl: 1, sub: 0, line: 4 } |  |  | 0.586 |
| walker |  | 9766 | 14 | Code::CodeKey { rung: Names, file: packages/d2ql/src/order-by.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.586 |
| ns | 9827 |  | 144 | README: what each example demonstrates | 9.1 | 1.10 | 0.589 |
| walker |  | 9833 | 67 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/order-by.ts, decl: 1, sub: 0, line: 16 } |  |  | 0.589 |
| walker |  | 9925 | 92 | Code::CodeKey { rung: Names, file: packages/d2ql/src/utils.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.589 |
| walker |  | 9949 | 24 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/utils.ts, decl: 3, sub: 0, line: 36 } |  |  | 0.589 |
| ns | 9977 |  | 150 | examples/electric — what the demo app does | 9.2 | 2.13 | 0.586 |
| walker |  | 9986 | 37 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/utils.ts, decl: 6, sub: 0, line: 192 } |  |  | 0.586 |
| walker |  | 9995 | 9 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/utils.ts, decl: 4, sub: 0, line: 55 } |  |  | 0.586 |

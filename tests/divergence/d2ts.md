Score(3000)=0.793 I=0.896 C=0.703 ns_rows≤3K=24/55 grid(1000/1442/2080/3000/4327/6240/9000)=0.670/0.817/0.810/0.793/0.742/0.648/0.601

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 54 | 54 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 76 | 22 | Fs::DirListing { dir: packages } |  |  | 0.000 |
| ns | 81 |  | 81 | Repository identity — README title + one-line definition | 1.1 |  | 0.000 |
| ns | 135 |  | 54 | Complete repository root listing | 1.2 |  | 0.633 |
| walker |  | 157 | 81 | Markdown::ReadmeHeadline { file: README.md } |  |  | 1.000 |
| ns | 192 |  | 57 | Workspace membership — packages/ and examples/ listings | 1.3 |  | 0.748 |
| walker |  | 193 | 36 | Fs::DirListing { dir: packages/d2ts } |  |  | 0.751 |
| ns | 218 |  | 26 | pnpm workspace globs | 1.4 |  | 0.706 |
| walker |  | 237 | 44 | Fs::DirListing { dir: packages/d2ts/src } |  |  | 0.714 |
| walker |  | 282 | 45 | Json::Identity { file: package.json } |  |  | 0.716 |
| walker |  | 286 | 4 | Fs::DirListing { dir: packages/d2ts/src/electric } |  |  | 0.716 |
| ns | 288 |  | 70 | README lede — what incremental execution buys you | 1.5 | 1.1 | 0.702 |
| walker |  | 306 | 20 | Fs::DirListing { dir: packages/d2ts/src/sqlite } |  |  | 0.703 |
| walker |  | 323 | 17 | Fs::DirListing { dir: packages/d2ts-benchmark } |  |  | 0.705 |
| walker |  | 335 | 12 | Fs::DirListing { dir: packages/d2ts-benchmark/src } |  |  | 0.706 |
| walker |  | 341 | 6 | Fs::DirListing { dir: .github/workflows } |  |  | 0.706 |
| ns | 387 |  | 99 | README lede — ElectricSQL ShapeStreams and pipeline type inference | 1.6 | 1.1 | 0.669 |
| walker |  | 412 | 71 | Json::Scripts { file: package.json } |  |  | 0.677 |
| walker |  | 497 | 85 | Json::Identity { file: packages/d2ts/package.json } |  |  | 0.680 |
| walker |  | 554 | 57 | Fs::DirListing { dir: packages/d2ts/src/sqlite/operators } |  |  | 0.685 |
| walker |  | 562 | 8 | Fs::DirListing { dir: .changeset } |  |  | 0.685 |
| ns | 578 |  | 191 | Root package.json — identity and every workspace script | 1.7 |  | 0.614 |
| walker |  | 587 | 25 | Fs::DirListing { dir: packages/d2ql } |  |  | 0.619 |
| walker |  | 656 | 69 | Fs::DirListing { dir: packages/d2ql/src } |  |  | 0.624 |
| walker |  | 665 | 9 | Fs::DirListing { dir: packages/d2ql/src/query-builder } |  |  | 0.625 |
| walker |  | 695 | 30 | Fs::DirListing { dir: packages/d2mini } |  |  | 0.633 |
| walker |  | 728 | 33 | Fs::DirListing { dir: packages/d2mini/src } |  |  | 0.602 |
| ns | 728 |  | 150 | Published package identity: d2ts and d2mini | 1.8 |  | 0.602 |
| walker |  | 826 | 98 | Fs::DirListing { dir: packages/d2ts/src/operators } |  |  | 0.612 |
| ns | 871 |  | 143 | Private package identity: d2ql and d2ts-benchmark | 1.9 | 1.8 | 0.562 |
| walker |  | 930 | 104 | Fs::DirListing { dir: packages/d2mini/src/operators } |  |  | 0.571 |
| walker |  | 995 | 65 | Json::ScriptsTail { file: package.json } |  |  | 0.670 |
| ns | 1032 |  | 161 | Root README section map (every H2) | 1.10 |  | 0.590 |
| walker |  | 1069 | 74 | Json::Dependencies { file: packages/d2ts/package.json } |  |  | 0.590 |
| walker |  | 1104 | 35 | Fs::DirListing { dir: examples } |  |  | 0.714 |
| walker |  | 1277 | 173 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.837 |
| walker |  | 1277 | 0 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.837 |
| ns | 1284 |  | 252 | README "Implementation Details" — provenance and the four core data structures | 1.11 | 1.10 | 0.760 |
| ns | 1392 |  | 108 | Every package's own root listing | 2.1 |  | 0.797 |
| ns | 1436 |  | 44 | packages/d2ts/src — complete module roster | 2.2 |  | 0.807 |
| walker |  | 1449 | 172 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.833 |
| ns | 1506 |  | 70 | packages/d2ts/src/index.ts — the public export barrel | 2.3 |  | 0.812 |
| ns | 1604 |  | 98 | packages/d2ts/src/operators — complete operator file roster | 2.4 |  | 0.826 |
| walker |  | 1832 | 383 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: true } |  |  | 0.828 |
| ns | 1844 |  | 240 | packages/d2ts/src/operators/index.ts — exported operator set | 2.5 | 2.4 | 0.775 |
| walker |  | 1954 | 122 | Json::Scripts { file: packages/d2ts/package.json } |  |  | 0.776 |
| ns | 1981 |  | 137 | packages/d2mini/src and its operators directory | 2.6 |  | 0.793 |
| walker |  | 2022 | 68 | Json::ScriptsTail { file: packages/d2ts/package.json } |  |  | 0.794 |
| walker |  | 2048 | 26 | Plaintext::DeclSurface { file: pnpm-workspace.yaml } |  |  | 0.810 |
| walker |  | 2059 | 11 | Code::CodeKey { rung: Names, file: eslint.base.mjs, decl: 0, sub: 0, line: 0 } |  |  | 0.810 |
| walker |  | 2083 | 24 | Fs::DirListing { dir: examples/electric } |  |  | 0.810 |
| walker |  | 2091 | 8 | Fs::DirListing { dir: examples/electric/src } |  |  | 0.811 |
| walker |  | 2104 | 13 | Fs::DirListing { dir: examples/electric/db } |  |  | 0.811 |
| walker |  | 2150 | 46 | Code::CodeKey { rung: Names, file: packages/d2ts-benchmark/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.811 |
| walker |  | 2206 | 56 | Code::CodeKey { rung: Decl, file: packages/d2ts-benchmark/src/index.ts, decl: 2, sub: 0, line: 25 } |  |  | 0.811 |
| ns | 2246 |  | 265 | packages/d2mini export barrels | 2.7 | 2.6 | 0.767 |
| walker |  | 2293 | 87 | Code::CodeKey { rung: Decl, file: packages/d2ts-benchmark/src/index.ts, decl: 3, sub: 0, line: 496 } |  |  | 0.767 |
| walker |  | 2300 | 7 | Code::CodeKey { rung: Doc, file: packages/d2ts-benchmark/src/index.ts, decl: 4, sub: 0, line: 504 } |  |  | 0.767 |
| walker |  | 2309 | 9 | Code::CodeKey { rung: Doc, file: packages/d2ts-benchmark/src/index.ts, decl: 1, sub: 0, line: 9 } |  |  | 0.767 |
| ns | 2324 |  | 78 | packages/d2ql/src and query-builder/ listings | 2.8 |  | 0.777 |
| walker |  | 2379 | 70 | Code::CodeKey { rung: Names, file: packages/d2ts/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.793 |
| walker |  | 2414 | 35 | Code::CodeKey { rung: Names, file: packages/d2ts/src/sqlite/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.793 |
| ns | 2441 |  | 117 | packages/d2ql export barrels | 2.9 | 2.8 | 0.778 |
| walker |  | 2461 | 47 | Code::CodeKey { rung: Names, file: packages/d2mini/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.779 |
| ns | 2518 |  | 77 | packages/d2ts/src/sqlite — complete tree listing | 2.10 |  | 0.787 |
| walker |  | 2701 | 240 | Code::CodeKey { rung: Names, file: packages/d2ts/src/operators/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.811 |
| ns | 2701 |  | 183 | SQLite subpath export barrels | 2.11 | 2.10 | 0.811 |
| ns | 2886 |  | 185 | Test suite layout — immediate children of every tests/ directory | 2.12 |  | 0.761 |
| walker |  | 2919 | 218 | Code::CodeKey { rung: Names, file: packages/d2mini/src/operators/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.805 |
| walker |  | 2957 | 38 | Code::CodeKey { rung: Names, file: packages/d2ql/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.807 |
| walker |  | 2971 | 14 | Code::CodeKey { rung: Doc, file: packages/d2ts-benchmark/src/index.ts, decl: 3, sub: 0, line: 496 } |  |  | 0.807 |
| ns | 2982 |  | 96 | Example and benchmark source trees | 2.13 | 1.3 | 0.792 |
| walker |  | 3000 | 29 | Fs::DirListing { dir: packages/d2mini/tests } |  |  | 0.793 |
| ns | 3172 |  | 190 | Per-package scripts: build, test, lint, typecheck, format | 3.1 | 1.7 | 0.796 |
| walker |  | 3173 | 173 | Json::Entry { file: packages/d2ts/package.json } |  |  | 0.796 |
| walker |  | 3205 | 32 | Fs::DirListing { dir: examples/d2ql } |  |  | 0.816 |
| ns | 3259 |  | 87 | d2ts-benchmark scripts — the benchmark entry points | 3.2 | 3.1 | 0.808 |
| ns | 3452 |  | 193 | CI pipeline stages | 3.3 |  | 0.781 |
| walker |  | 3478 | 273 | Code::CodeKey { rung: Names, file: packages/d2ql/src/types.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.781 |
| walker |  | 3497 | 19 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/types.ts, decl: 7, sub: 0, line: 183 } |  |  | 0.781 |
| walker |  | 3516 | 19 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/types.ts, decl: 9, sub: 0, line: 192 } |  |  | 0.781 |
| walker |  | 3535 | 19 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/types.ts, decl: 11, sub: 0, line: 201 } |  |  | 0.781 |
| walker |  | 3556 | 21 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/types.ts, decl: 4, sub: 0, line: 24 } |  |  | 0.781 |
| walker |  | 3584 | 28 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/types.ts, decl: 10, sub: 0, line: 196 } |  |  | 0.781 |
| walker |  | 3613 | 29 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/types.ts, decl: 14, sub: 0, line: 247 } |  |  | 0.781 |
| walker |  | 3644 | 31 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/types.ts, decl: 6, sub: 0, line: 178 } |  |  | 0.781 |
| ns | 3656 |  | 204 | README: constructing a D2 graph and sending to an input stream | 4.1 | 1.10 | 0.771 |
| walker |  | 3675 | 31 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/types.ts, decl: 8, sub: 0, line: 187 } |  |  | 0.771 |
| walker |  | 3721 | 46 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/types.ts, decl: 3, sub: 0, line: 15 } |  |  | 0.771 |
| walker |  | 3778 | 57 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/types.ts, decl: 5, sub: 0, line: 48 } |  |  | 0.771 |
| walker |  | 3845 | 67 | Code::CodeKey { rung: Names, file: packages/d2ts/src/d2.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.771 |
| walker |  | 3867 | 22 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/d2.ts, decl: 1, sub: 0, line: 11 } |  |  | 0.771 |
| walker |  | 3905 | 38 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/d2.ts, decl: 41, sub: 0, line: 166 } |  |  | 0.771 |
| walker |  | 3943 | 38 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/d2.ts, decl: 42, sub: 0, line: 167 } |  |  | 0.771 |
| ns | 3981 |  | 325 | d2.ts — D2Options and the complete D2 class method roster | 4.2 |  | 0.746 |
| walker |  | 4147 | 204 | Code::CodeKey { rung: Names, file: packages/d2mini/src/operators/topKWithFractionalIndex.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.746 |
| walker |  | 4168 | 21 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/operators/topKWithFractionalIndex.ts, decl: 1, sub: 0, line: 14 } |  |  | 0.746 |
| ns | 4171 |  | 190 | README: what versions and frontiers actually mean | 4.3 | 1.10 | 0.742 |
| walker |  | 4198 | 30 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/operators/topKWithFractionalIndex.ts, decl: 11, sub: 0, line: 322 } |  |  | 0.742 |
| walker |  | 4233 | 35 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/operators/topKWithFractionalIndex.ts, decl: 3, sub: 0, line: 30 } |  |  | 0.742 |
| walker |  | 4287 | 54 | Code::CodeKey { rung: Names, file: packages/d2mini/src/d2.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.742 |
| walker |  | 4313 | 26 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/d2.ts, decl: 37, sub: 0, line: 146 } |  |  | 0.742 |
| walker |  | 4350 | 37 | Code::CodeKey { rung: Names, file: packages/d2ts/src/sqlite/version-index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.742 |
| walker |  | 4406 | 56 | Code::CodeKey { rung: Names, file: packages/d2ts/src/operators/topKWithFractionalIndex.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.742 |
| walker |  | 4429 | 23 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/topKWithFractionalIndex.ts, decl: 1, sub: 0, line: 19 } |  |  | 0.742 |
| walker |  | 4499 | 70 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/types.ts, decl: 16, sub: 0, line: 263 } |  |  | 0.742 |
| ns | 4517 |  | 346 | order.ts — v() factory and the complete Version method roster | 4.4 |  | 0.716 |
| walker |  | 4540 | 41 | Fs::DirListing { dir: packages/d2ts/tests } |  |  | 0.723 |
| walker |  | 4581 | 41 | Fs::DirListing { dir: packages/d2ts/tests/operators-sqlite } |  |  | 0.723 |
| walker |  | 4663 | 82 | Code::CodeKey { rung: Names, file: packages/d2ts/src/electric/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.723 |
| walker |  | 4731 | 68 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/electric/index.ts, decl: 3, sub: 0, line: 222 } |  |  | 0.723 |
| walker |  | 4805 | 74 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/electric/index.ts, decl: 7, sub: 0, line: 328 } |  |  | 0.723 |
| ns | 4807 |  | 290 | order.ts — Antichain method roster and Frontier | 4.5 | 4.4 | 0.702 |
| walker |  | 4886 | 81 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/electric/index.ts, decl: 4, sub: 0, line: 227 } |  |  | 0.702 |
| ns | 4952 |  | 145 | README: MultiSet as a changeset, and keyed multisets | 4.6 | 1.10 | 0.699 |
| walker |  | 5007 | 121 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/electric/index.ts, decl: 2, sub: 0, line: 103 } |  |  | 0.699 |
| walker |  | 5258 | 251 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/electric/index.ts, decl: 1, sub: 0, line: 40 } |  |  | 0.699 |
| walker |  | 5279 | 21 | Code::CodeKey { rung: Names, file: packages/d2ql/src/query-builder/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.701 |
| ns | 5310 |  | 358 | multiset.ts — MultiSetArray, KeyedData and every MultiSet method | 4.7 |  | 0.683 |
| walker |  | 5314 | 35 | Code::CodeKey { rung: Names, file: packages/d2ql/src/query-builder/query-builder.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.683 |
| walker |  | 5364 | 50 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/topKWithFractionalIndex.ts, decl: 2, sub: 0, line: 29 } |  |  | 0.683 |
| walker |  | 5447 | 83 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/types.ts, decl: 17, sub: 0, line: 274 } |  |  | 0.683 |
| walker |  | 5501 | 54 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/query-builder/query-builder.ts, decl: 2, sub: 0, line: 619 } |  |  | 0.683 |
| ns | 5610 |  | 300 | types.ts — the operator message protocol | 4.8 |  | 0.660 |
| walker |  | 5859 | 358 | Code::CodeKey { rung: Decl, file: eslint.base.mjs, decl: 1, sub: 0, line: 6 } |  |  | 0.660 |
| ns | 5868 |  | 258 | types.ts — PipedOperator and the 20-deep pipe overload wall | 4.9 | 4.8 | 0.648 |
| walker |  | 5954 | 95 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/types.ts, decl: 15, sub: 0, line: 251 } |  |  | 0.648 |
| ns | 6251 |  | 383 | README Key Features — every operator with a one-line description | 5.1 | 1.10 | 0.656 |
| walker |  | 6304 | 350 | Code::CodeKey { rung: Decl, file: eslint.base.mjs, decl: 1, sub: 1, line: 6 } |  |  | 0.656 |
| walker |  | 6409 | 105 | Markdown::Section { file: README.md, section_index: 18, keeps_default_concavity: false } |  |  | 0.656 |
| walker |  | 6477 | 68 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/operators/topKWithFractionalIndex.ts, decl: 4, sub: 0, line: 165 } |  |  | 0.656 |
| walker |  | 6535 | 58 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/operators/topKWithFractionalIndex.ts, decl: 7, sub: 0, line: 229 } |  |  | 0.656 |
| walker |  | 6650 | 115 | Fs::DirListing { dir: packages/d2ql/tests } |  |  | 0.693 |
| ns | 6681 |  | 430 | packages/d2ts/README.md — complete heading map (1081-line API reference) | 5.2 |  | 0.666 |
| walker |  | 6704 | 54 | Fs::DirListing { dir: packages/d2ql/tests/query-builder } |  |  | 0.666 |
| walker |  | 6843 | 139 | Fs::DirListing { dir: packages/d2mini/tests/operators } |  |  | 0.666 |
| ns | 6844 |  | 163 | Operator factory roster — single-stream operators | 5.3 | 2.5 | 0.658 |
| walker |  | 6917 | 74 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/operators/topKWithFractionalIndex.ts, decl: 2, sub: 0, line: 19 } |  |  | 0.658 |
| walker |  | 7076 | 159 | Fs::DirListing { dir: packages/d2ts/tests/operators } |  |  | 0.658 |
| walker |  | 7225 | 149 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.658 |
| ns | 7244 |  | 400 | Operator factory roster — keyed, join, ordering and aggregate families | 5.4 | 5.3 | 0.639 |
| walker |  | 7356 | 131 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/d2.ts, decl: 1, sub: 0, line: 10 } |  |  | 0.639 |
| ns | 7446 |  | 202 | Operator implementation classes and the LinearUnaryOperator base | 5.5 | 5.4 | 0.630 |
| walker |  | 7494 | 138 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/types.ts, decl: 13, sub: 0, line: 234 } |  |  | 0.630 |
| walker |  | 7584 | 90 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/operators/topKWithFractionalIndex.ts, decl: 5, sub: 0, line: 177 } |  |  | 0.630 |
| ns | 7622 |  | 176 | groupBy.ts — the AggregateFunction contract | 5.6 | 5.4 | 0.622 |
| walker |  | 7627 | 43 | Code::CodeKey { rung: Doc, file: packages/d2ts/src/electric/index.ts, decl: 7, sub: 0, line: 328 } |  |  | 0.622 |
| walker |  | 7633 | 6 | Code::CodeKey { rung: Doc, file: packages/d2ql/src/types.ts, decl: 4, sub: 0, line: 24 } |  |  | 0.622 |
| walker |  | 7736 | 103 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/topKWithFractionalIndex.ts, decl: 3, sub: 0, line: 39 } |  |  | 0.622 |
| ns | 7842 |  | 220 | D2QL identity and README section map | 6.1 |  | 0.615 |
| walker |  | 7921 | 185 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/d2.ts, decl: 2, sub: 0, line: 15 } |  |  | 0.627 |
| walker |  | 8037 | 116 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/operators/topKWithFractionalIndex.ts, decl: 8, sub: 0, line: 288 } |  |  | 0.627 |
| ns | 8084 |  | 242 | D2QL "Current Features" — the supported SQL subset | 6.2 | 6.1 | 0.619 |
| walker |  | 8153 | 116 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/operators/topKWithFractionalIndex.ts, decl: 5, sub: 0, line: 372 } |  |  | 0.619 |
| walker |  | 8349 | 196 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.621 |
| ns | 8442 |  | 358 | schema.ts — the Query interface family | 6.3 |  | 0.608 |
| walker |  | 8523 | 174 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.615 |
| ns | 8604 |  | 162 | schema.ts — the Comparator and LogicalOperator vocabularies | 6.4 | 6.3 | 0.607 |
| ns | 8735 |  | 131 | compileQuery — the package's single entry point | 6.5 |  | 0.602 |
| walker |  | 8753 | 230 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/types.ts, decl: 12, sub: 0, line: 209 } |  |  | 0.602 |
| walker |  | 8824 | 71 | Code::CodeKey { rung: Doc, file: packages/d2ts/src/electric/index.ts, decl: 3, sub: 0, line: 222 } |  |  | 0.602 |
| walker |  | 8886 | 62 | Json::Identity { file: packages/d2ts-benchmark/package.json } |  |  | 0.605 |
| walker |  | 8913 | 27 | Json::Scripts { file: packages/d2ts-benchmark/package.json } |  |  | 0.605 |
| walker |  | 8973 | 60 | Json::ScriptsTail { file: packages/d2ts-benchmark/package.json } |  |  | 0.611 |
| ns | 8978 |  | 243 | query-builder.ts — the fluent builder's complete method set | 6.6 |  | 0.601 |
| walker |  | 9038 | 65 | Json::Dependencies { file: packages/d2ts-benchmark/package.json } |  |  | 0.601 |
| ns | 9076 |  | 98 | d2mini README — how it differs from d2ts | 7.1 |  | 0.600 |
| ns | 9165 |  | 89 | d2mini/src/d2.ts — the versionless D2 constructor | 7.2 | 7.1 | 0.597 |
| walker |  | 9295 | 257 | Markdown::Section { file: README.md, section_index: 17, keeps_default_concavity: false } |  |  | 0.622 |
| walker |  | 9307 | 12 | Code::CodeKey { rung: Doc, file: packages/d2ql/src/types.ts, decl: 3, sub: 0, line: 15 } |  |  | 0.622 |
| walker |  | 9352 | 45 | Code::CodeKey { rung: Names, file: packages/d2ts-benchmark/src/base.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.622 |
| walker |  | 9396 | 44 | Code::CodeKey { rung: Decl, file: packages/d2ts-benchmark/src/base.ts, decl: 2, sub: 0, line: 12 } |  |  | 0.622 |
| ns | 9401 |  | 236 | README: using the SQLite backend | 8.1 | 1.10 | 0.616 |
| walker |  | 9453 | 57 | Code::CodeKey { rung: Decl, file: packages/d2ts-benchmark/src/base.ts, decl: 3, sub: 0, line: 19 } |  |  | 0.616 |
| walker |  | 9527 | 74 | Code::CodeKey { rung: Decl, file: packages/d2ts-benchmark/src/base.ts, decl: 1, sub: 0, line: 4 } |  |  | 0.616 |
| ns | 9529 |  | 128 | sqlite/database.ts — the SQLiteDb driver interface | 8.2 | 8.1 | 0.611 |
| ns | 9683 |  | 154 | withSQLite, SQLIndex and the Electric bridge exports | 8.3 | 8.2 | 0.613 |
| walker |  | 9692 | 165 | Code::CodeKey { rung: Decl, file: packages/d2ts-benchmark/src/base.ts, decl: 4, sub: 0, line: 27 } |  |  | 0.613 |
| ns | 9827 |  | 144 | README: what each example demonstrates | 9.1 | 1.10 | 0.616 |
| ns | 9977 |  | 150 | examples/electric — what the demo app does | 9.2 | 2.13 | 0.614 |
| walker |  | 9987 | 295 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/sqlite/version-index.ts, decl: 2, sub: 0, line: 60 } |  |  | 0.614 |

Score(3000)=0.730 I=0.868 C=0.614 ns_rows≤3K=24/55 grid(1000/1442/2080/3000/4327/6240/9000)=0.670/0.802/0.822/0.730/0.714/0.656/0.602

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
| walker |  | 1313 | 24 | Fs::DirListing { dir: examples/electric } |  |  | 0.695 |
| walker |  | 1321 | 8 | Fs::DirListing { dir: examples/electric/src } |  |  | 0.696 |
| walker |  | 1347 | 26 | Plaintext::Whole { file: pnpm-workspace.yaml } |  |  | 0.723 |
| walker |  | 1358 | 11 | Code::CodeKey { rung: Names, file: eslint.base.mjs, decl: 0, sub: 0, line: 0 } |  |  | 0.723 |
| ns | 1392 |  | 108 | Every package's own root listing | 2.1 |  | 0.767 |
| ns | 1436 |  | 44 | packages/d2ts/src — complete module roster | 2.2 |  | 0.779 |
| walker |  | 1494 | 136 | Json::Scripts { file: package.json } |  |  | 0.850 |
| ns | 1506 |  | 70 | packages/d2ts/src/index.ts — the public export barrel | 2.3 |  | 0.828 |
| walker |  | 1523 | 29 | Fs::DirListing { dir: packages/d2mini/tests } |  |  | 0.829 |
| walker |  | 1593 | 70 | Code::CodeKey { rung: Names, file: packages/d2ts/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.856 |
| ns | 1604 |  | 98 | packages/d2ts/src/operators — complete operator file roster | 2.4 |  | 0.864 |
| walker |  | 1640 | 47 | Code::CodeKey { rung: Names, file: packages/d2mini/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.864 |
| walker |  | 1678 | 38 | Code::CodeKey { rung: Names, file: packages/d2ql/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.864 |
| walker |  | 1716 | 38 | Code::CodeKey { rung: Names, file: packages/d2ts-benchmark/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.864 |
| walker |  | 1772 | 56 | Code::CodeKey { rung: Decl, file: packages/d2ts-benchmark/src/index.ts, decl: 2, sub: 0, line: 25 } |  |  | 0.864 |
| ns | 1844 |  | 240 | packages/d2ts/src/operators/index.ts — exported operator set | 2.5 | 2.4 | 0.809 |
| walker |  | 1859 | 87 | Code::CodeKey { rung: Decl, file: packages/d2ts-benchmark/src/index.ts, decl: 3, sub: 0, line: 496 } |  |  | 0.809 |
| ns | 1981 |  | 137 | packages/d2mini/src and its operators directory | 2.6 |  | 0.822 |
| walker |  | 2242 | 383 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: true } |  |  | 0.824 |
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
| walker |  | 3249 | 115 | Fs::DirListing { dir: packages/d2ql/tests } |  |  | 0.785 |
| ns | 3259 |  | 87 | d2ts-benchmark scripts — the benchmark entry points | 3.2 | 3.1 | 0.778 |
| walker |  | 3303 | 54 | Fs::DirListing { dir: packages/d2ql/tests/query-builder } |  |  | 0.778 |
| ns | 3452 |  | 193 | CI pipeline stages | 3.3 |  | 0.752 |
| walker |  | 3653 | 350 | Code::CodeKey { rung: Decl, file: eslint.base.mjs, decl: 1, sub: 1, line: 6 } |  |  | 0.752 |
| ns | 3656 |  | 204 | README: constructing a D2 graph and sending to an input stream | 4.1 | 1.10 | 0.742 |
| walker |  | 3792 | 139 | Fs::DirListing { dir: packages/d2mini/tests/operators } |  |  | 0.742 |
| walker |  | 3951 | 159 | Fs::DirListing { dir: packages/d2ts/tests/operators } |  |  | 0.742 |
| ns | 3981 |  | 325 | d2.ts — D2Options and the complete D2 class method roster | 4.2 |  | 0.718 |
| walker |  | 4097 | 146 | Code::CodeKey { rung: Body, file: packages/d2ts-benchmark/src/index.ts, decl: 1, sub: 0, line: 9 } |  |  | 0.718 |
| ns | 4171 |  | 190 | README: what versions and frontiers actually mean | 4.3 | 1.10 | 0.714 |
| walker |  | 4280 | 183 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.714 |
| walker |  | 4385 | 105 | Markdown::Section { file: README.md, section_index: 18, keeps_default_concavity: false } |  |  | 0.714 |
| ns | 4517 |  | 346 | order.ts — v() factory and the complete Version method roster | 4.4 |  | 0.688 |
| walker |  | 4584 | 199 | Markdown::HeadingsOutline { file: packages/d2ts/README.md } |  |  | 0.689 |
| walker |  | 4733 | 149 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.690 |
| walker |  | 4795 | 62 | Json::Identity { file: packages/d2ts-benchmark/package.json } |  |  | 0.693 |
| ns | 4807 |  | 290 | order.ts — Antichain method roster and Frontier | 4.5 | 4.4 | 0.673 |
| walker |  | 4882 | 87 | Json::Scripts { file: packages/d2ts-benchmark/package.json } |  |  | 0.682 |
| walker |  | 4947 | 65 | Json::Dependencies { file: packages/d2ts-benchmark/package.json } |  |  | 0.682 |
| ns | 4952 |  | 145 | README: MultiSet as a changeset, and keyed multisets | 4.6 | 1.10 | 0.679 |
| walker |  | 5026 | 79 | Markdown::Section { file: packages/d2mini/README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.679 |
| walker |  | 5058 | 32 | Markdown::Section { file: packages/d2ql/README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.679 |
| walker |  | 5093 | 35 | Code::CodeKey { rung: Names, file: packages/d2ts-benchmark/src/base.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.679 |
| walker |  | 5150 | 57 | Code::CodeKey { rung: Decl, file: packages/d2ts-benchmark/src/base.ts, decl: 2, sub: 0, line: 19 } |  |  | 0.679 |
| walker |  | 5226 | 76 | Code::CodeKey { rung: Decl, file: packages/d2ts-benchmark/src/base.ts, decl: 1, sub: 0, line: 4 } |  |  | 0.679 |
| ns | 5310 |  | 358 | multiset.ts — MultiSetArray, KeyedData and every MultiSet method | 4.7 |  | 0.662 |
| walker |  | 5391 | 165 | Code::CodeKey { rung: Decl, file: packages/d2ts-benchmark/src/base.ts, decl: 3, sub: 0, line: 27 } |  |  | 0.662 |
| walker |  | 5399 | 8 | Code::CodeKey { rung: Body, file: packages/d2ts-benchmark/src/base.ts, decl: 6, sub: 0, line: 48 } |  |  | 0.662 |
| walker |  | 5416 | 17 | Code::CodeKey { rung: Body, file: packages/d2ts-benchmark/src/base.ts, decl: 5, sub: 0, line: 44 } |  |  | 0.662 |
| ns | 5610 |  | 300 | types.ts — the operator message protocol | 4.8 |  | 0.640 |
| walker |  | 5612 | 196 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.643 |
| walker |  | 5786 | 174 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.652 |
| ns | 5868 |  | 258 | types.ts — PipedOperator and the 20-deep pipe overload wall | 4.9 | 4.8 | 0.640 |
| walker |  | 5877 | 91 | Json::Identity { file: packages/d2mini/package.json } |  |  | 0.654 |
| walker |  | 5960 | 83 | Json::Entry { file: packages/d2mini/package.json } |  |  | 0.654 |
| walker |  | 6148 | 188 | Json::Scripts { file: packages/d2mini/package.json } |  |  | 0.654 |
| walker |  | 6219 | 71 | Json::Dependencies { file: packages/d2mini/package.json } |  |  | 0.654 |
| ns | 6251 |  | 383 | README Key Features — every operator with a one-line description | 5.1 | 1.10 | 0.662 |
| walker |  | 6313 | 94 | Json::Identity { file: packages/d2ql/package.json } |  |  | 0.677 |
| walker |  | 6485 | 172 | Json::Entry { file: packages/d2ql/package.json } |  |  | 0.677 |
| walker |  | 6673 | 188 | Json::Scripts { file: packages/d2ql/package.json } |  |  | 0.677 |
| ns | 6681 |  | 430 | packages/d2ts/README.md — complete heading map (1081-line API reference) | 5.2 |  | 0.660 |
| walker |  | 6686 | 13 | Code::CodeKey { rung: Names, file: packages/d2ql/src/select.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.660 |
| walker |  | 6746 | 60 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/select.ts, decl: 1, sub: 0, line: 7 } |  |  | 0.660 |
| walker |  | 6772 | 26 | Code::CodeKey { rung: Names, file: packages/d2ql/src/group-by.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.660 |
| walker |  | 6812 | 40 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/group-by.ts, decl: 2, sub: 0, line: 98 } |  |  | 0.660 |
| ns | 6844 |  | 163 | Operator factory roster — single-stream operators | 5.3 | 2.5 | 0.652 |
| walker |  | 6853 | 41 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/group-by.ts, decl: 1, sub: 0, line: 20 } |  |  | 0.652 |
| walker |  | 6880 | 27 | Code::CodeKey { rung: Doc, file: packages/d2ql/src/group-by.ts, decl: 1, sub: 0, line: 20 } |  |  | 0.652 |
| walker |  | 6908 | 28 | Code::CodeKey { rung: Doc, file: packages/d2ql/src/group-by.ts, decl: 2, sub: 0, line: 98 } |  |  | 0.652 |
| walker |  | 6934 | 26 | Code::CodeKey { rung: Names, file: packages/d2ql/src/joins.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.652 |
| walker |  | 6983 | 49 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/joins.ts, decl: 2, sub: 0, line: 124 } |  |  | 0.652 |
| walker |  | 7063 | 80 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/joins.ts, decl: 1, sub: 0, line: 16 } |  |  | 0.652 |
| walker |  | 7086 | 23 | Code::CodeKey { rung: Doc, file: packages/d2ql/src/joins.ts, decl: 1, sub: 0, line: 16 } |  |  | 0.652 |
| walker |  | 7109 | 23 | Code::CodeKey { rung: Doc, file: packages/d2ql/src/joins.ts, decl: 2, sub: 0, line: 124 } |  |  | 0.652 |
| walker |  | 7164 | 55 | Code::CodeKey { rung: Names, file: packages/d2ts/src/order.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.653 |
| walker |  | 7181 | 17 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/order.ts, decl: 33, sub: 0, line: 277 } |  |  | 0.653 |
| walker |  | 7189 | 8 | Code::CodeKey { rung: Body, file: packages/d2ts/src/order.ts, decl: 34, sub: 0, line: 278 } |  |  | 0.653 |
| ns | 7244 |  | 400 | Operator factory roster — keyed, join, ordering and aggregate families | 5.4 | 5.3 | 0.633 |
| walker |  | 7391 | 202 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/order.ts, decl: 2, sub: 0, line: 29 } |  |  | 0.644 |
| walker |  | 7401 | 10 | Code::CodeKey { rung: Body, file: packages/d2ts/src/order.ts, decl: 16, sub: 0, line: 128 } |  |  | 0.644 |
| walker |  | 7412 | 11 | Code::CodeKey { rung: Body, file: packages/d2ts/src/order.ts, decl: 6, sub: 0, line: 50 } |  |  | 0.644 |
| walker |  | 7426 | 14 | Code::CodeKey { rung: Body, file: packages/d2ts/src/order.ts, decl: 5, sub: 0, line: 46 } |  |  | 0.644 |
| ns | 7446 |  | 202 | Operator implementation classes and the LinearUnaryOperator base | 5.5 | 5.4 | 0.635 |
| ns | 7622 |  | 176 | groupBy.ts — the AggregateFunction contract | 5.6 | 5.4 | 0.627 |
| walker |  | 7648 | 222 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/order.ts, decl: 17, sub: 0, line: 138 } |  |  | 0.642 |
| walker |  | 7678 | 30 | Code::CodeKey { rung: Decl, file: packages/d2ts/src/order.ts, decl: 19, sub: 0, line: 148 } |  |  | 0.642 |
| walker |  | 7716 | 38 | Code::CodeKey { rung: Doc, file: packages/d2ts/src/order.ts, decl: 17, sub: 0, line: 138 } |  |  | 0.648 |
| walker |  | 7773 | 57 | Code::CodeKey { rung: Doc, file: packages/d2ts/src/order.ts, decl: 1, sub: 0, line: 10 } |  |  | 0.652 |
| walker |  | 7787 | 14 | Code::CodeKey { rung: Names, file: packages/d2ql/src/key-by.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.652 |
| ns | 7842 |  | 220 | D2QL identity and README section map | 6.1 |  | 0.645 |
| walker |  | 7843 | 56 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/key-by.ts, decl: 1, sub: 0, line: 4 } |  |  | 0.645 |
| walker |  | 7857 | 14 | Code::CodeKey { rung: Names, file: packages/d2ql/src/order-by.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.645 |
| walker |  | 7924 | 67 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/order-by.ts, decl: 1, sub: 0, line: 16 } |  |  | 0.645 |
| walker |  | 8037 | 113 | Markdown::Section { file: packages/d2ql/README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.647 |
| ns | 8084 |  | 242 | D2QL "Current Features" — the supported SQL subset | 6.2 | 6.1 | 0.639 |
| walker |  | 8129 | 92 | Code::CodeKey { rung: Names, file: packages/d2ql/src/utils.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.639 |
| walker |  | 8153 | 24 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/utils.ts, decl: 3, sub: 0, line: 36 } |  |  | 0.639 |
| walker |  | 8190 | 37 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/utils.ts, decl: 6, sub: 0, line: 192 } |  |  | 0.639 |
| walker |  | 8234 | 44 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/utils.ts, decl: 4, sub: 0, line: 55 } |  |  | 0.639 |
| walker |  | 8263 | 29 | Code::CodeKey { rung: Doc, file: packages/d2ql/src/utils.ts, decl: 1, sub: 0, line: 4 } |  |  | 0.639 |
| walker |  | 8292 | 29 | Code::CodeKey { rung: Doc, file: packages/d2ql/src/utils.ts, decl: 2, sub: 0, line: 24 } |  |  | 0.639 |
| walker |  | 8347 | 55 | Code::CodeKey { rung: Doc, file: packages/d2ql/src/utils.ts, decl: 5, sub: 0, line: 131 } |  |  | 0.639 |
| walker |  | 8412 | 65 | Code::CodeKey { rung: Doc, file: packages/d2ql/src/utils.ts, decl: 3, sub: 0, line: 36 } |  |  | 0.639 |
| ns | 8442 |  | 358 | schema.ts — the Query interface family | 6.3 |  | 0.625 |
| walker |  | 8443 | 31 | Code::CodeKey { rung: Names, file: packages/d2ql/src/evaluators.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.625 |
| walker |  | 8494 | 51 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/evaluators.ts, decl: 1, sub: 0, line: 14 } |  |  | 0.625 |
| walker |  | 8565 | 71 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/evaluators.ts, decl: 2, sub: 0, line: 126 } |  |  | 0.625 |
| walker |  | 8590 | 25 | Code::CodeKey { rung: Doc, file: packages/d2ql/src/evaluators.ts, decl: 1, sub: 0, line: 14 } |  |  | 0.625 |
| ns | 8604 |  | 162 | schema.ts — the Comparator and LogicalOperator vocabularies | 6.4 | 6.3 | 0.617 |
| walker |  | 8616 | 26 | Code::CodeKey { rung: Doc, file: packages/d2ql/src/evaluators.ts, decl: 2, sub: 0, line: 126 } |  |  | 0.617 |
| walker |  | 8727 | 111 | Code::CodeKey { rung: Names, file: packages/d2mini/src/types.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.617 |
| ns | 8735 |  | 131 | compileQuery — the package's single entry point | 6.5 |  | 0.612 |
| walker |  | 8748 | 21 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/types.ts, decl: 7, sub: 0, line: 83 } |  |  | 0.612 |
| walker |  | 8771 | 23 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/types.ts, decl: 2, sub: 0, line: 6 } |  |  | 0.612 |
| walker |  | 8796 | 25 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/types.ts, decl: 3, sub: 0, line: 11 } |  |  | 0.612 |
| walker |  | 8834 | 38 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/types.ts, decl: 4, sub: 0, line: 16 } |  |  | 0.612 |
| walker |  | 8913 | 79 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/types.ts, decl: 5, sub: 0, line: 21 } |  |  | 0.612 |
| walker |  | 8929 | 16 | Code::CodeKey { rung: Names, file: packages/d2mini/src/indexes.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.612 |
| ns | 8978 |  | 243 | query-builder.ts — the fluent builder's complete method set | 6.6 |  | 0.602 |
| ns | 9076 |  | 98 | d2mini README — how it differs from d2ts | 7.1 |  | 0.604 |
| walker |  | 9113 | 184 | Code::CodeKey { rung: Decl, file: packages/d2mini/src/indexes.ts, decl: 1, sub: 0, line: 9 } |  |  | 0.604 |
| walker |  | 9124 | 11 | Code::CodeKey { rung: Body, file: packages/d2mini/src/indexes.ts, decl: 6, sub: 0, line: 38 } |  |  | 0.604 |
| walker |  | 9135 | 11 | Code::CodeKey { rung: Body, file: packages/d2mini/src/indexes.ts, decl: 7, sub: 0, line: 42 } |  |  | 0.604 |
| walker |  | 9146 | 11 | Code::CodeKey { rung: Body, file: packages/d2mini/src/indexes.ts, decl: 9, sub: 0, line: 50 } |  |  | 0.604 |
| walker |  | 9158 | 12 | Code::CodeKey { rung: Body, file: packages/d2mini/src/indexes.ts, decl: 8, sub: 0, line: 46 } |  |  | 0.604 |
| ns | 9165 |  | 89 | d2mini/src/d2.ts — the versionless D2 constructor | 7.2 | 7.1 | 0.600 |
| walker |  | 9184 | 26 | Code::CodeKey { rung: Body, file: packages/d2mini/src/indexes.ts, decl: 4, sub: 0, line: 28 } |  |  | 0.600 |
| walker |  | 9253 | 69 | Code::CodeKey { rung: Doc, file: packages/d2mini/src/indexes.ts, decl: 1, sub: 0, line: 9 } |  |  | 0.600 |
| ns | 9401 |  | 236 | README: using the SQLite backend | 8.1 | 1.10 | 0.595 |
| walker |  | 9526 | 273 | Code::CodeKey { rung: Names, file: packages/d2ql/src/types.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.595 |
| ns | 9529 |  | 128 | sqlite/database.ts — the SQLiteDb driver interface | 8.2 | 8.1 | 0.590 |
| walker |  | 9545 | 19 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/types.ts, decl: 7, sub: 0, line: 183 } |  |  | 0.590 |
| walker |  | 9564 | 19 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/types.ts, decl: 9, sub: 0, line: 192 } |  |  | 0.590 |
| walker |  | 9583 | 19 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/types.ts, decl: 11, sub: 0, line: 201 } |  |  | 0.590 |
| walker |  | 9604 | 21 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/types.ts, decl: 4, sub: 0, line: 24 } |  |  | 0.590 |
| walker |  | 9632 | 28 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/types.ts, decl: 10, sub: 0, line: 196 } |  |  | 0.590 |
| walker |  | 9661 | 29 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/types.ts, decl: 14, sub: 0, line: 247 } |  |  | 0.590 |
| ns | 9683 |  | 154 | withSQLite, SQLIndex and the Electric bridge exports | 8.3 | 8.2 | 0.586 |
| walker |  | 9692 | 31 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/types.ts, decl: 6, sub: 0, line: 178 } |  |  | 0.586 |
| walker |  | 9723 | 31 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/types.ts, decl: 8, sub: 0, line: 187 } |  |  | 0.586 |
| walker |  | 9769 | 46 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/types.ts, decl: 3, sub: 0, line: 15 } |  |  | 0.586 |
| walker |  | 9826 | 57 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/types.ts, decl: 5, sub: 0, line: 48 } |  |  | 0.586 |
| ns | 9827 |  | 144 | README: what each example demonstrates | 9.1 | 1.10 | 0.589 |
| walker |  | 9896 | 70 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/types.ts, decl: 16, sub: 0, line: 263 } |  |  | 0.589 |
| ns | 9977 |  | 150 | examples/electric — what the demo app does | 9.2 | 2.13 | 0.587 |
| walker |  | 9979 | 83 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/types.ts, decl: 17, sub: 0, line: 274 } |  |  | 0.587 |
| walker |  | 9998 | 19 | Code::CodeKey { rung: Decl, file: packages/d2ql/src/types.ts, decl: 15, sub: 0, line: 251 } |  |  | 0.587 |

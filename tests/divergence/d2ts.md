Score(3000)=0.729 I=0.836 C=0.636 ns_rows≤3K=24/55 (reached=15 partial=1 missing=8)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 58 | 58 | listing of '.' |  |  | 0.000 |
| walker |  | 61 | 3 | listing of '.github' |  |  | 0.000 |
| walker |  | 64 | 3 | listing of '.github/workflows' |  |  | 0.000 |
| ns | 81 |  | 81 | Repository identity — README title + one-line definition | 1.1 |  | 0.000 |
| walker |  | 89 | 25 | listing of 'packages' |  |  | 0.000 |
| walker |  | 125 | 36 | listing of 'examples' |  |  | 0.000 |
| ns | 139 |  | 58 | Complete repository root listing | 1.2 |  | 0.714 |
| walker |  | 142 | 17 | listing of 'packages/d2ts-benchmark' |  |  | 0.715 |
| walker |  | 153 | 11 | listing of 'packages/d2ts-benchmark/src' |  |  | 0.715 |
| walker |  | 160 | 7 | listing of '.changeset' |  |  | 0.715 |
| walker |  | 186 | 26 | listing of 'packages/d2ql' |  |  | 0.720 |
| ns | 200 |  | 61 | Workspace membership — packages/ and examples/ listings | 1.3 |  | 0.700 |
| walker |  | 217 | 31 | listing of 'packages/d2mini' |  |  | 0.710 |
| ns | 226 |  | 26 | pnpm workspace globs | 1.4 |  | 0.668 |
| ns | 296 |  | 70 | README lede — what incremental execution buys you | 1.5 | 1.1 | 0.655 |
| walker |  | 298 | 81 | README headline in README.md |  |  | 0.934 |
| walker |  | 335 | 37 | listing of 'packages/d2ts' |  |  | 0.937 |
| ns | 395 |  | 99 | README lede — ElectricSQL ShapeStreams and pipeline type inference | 1.6 | 1.1 | 0.898 |
| walker |  | 420 | 85 | package identity in packages/d2ts/package.json |  |  | 0.898 |
| ns | 586 |  | 191 | Root package.json — identity and every workspace script | 1.7 |  | 0.728 |
| ns | 736 |  | 150 | Published package identity: d2ts and d2mini | 1.8 |  | 0.686 |
| ns | 879 |  | 143 | Private package identity: d2ql and d2ts-benchmark | 1.9 | 1.8 | 0.629 |
| ns | 1040 |  | 161 | Root README section map (every H2) | 1.10 |  | 0.554 |
| walker |  | 1072 | 652 | YAML config at .github/workflows/ci.yml |  |  | 0.555 |
| walker |  | 1105 | 33 | listing of 'packages/d2mini/src' |  |  | 0.556 |
| walker |  | 1208 | 103 | listing of 'packages/d2mini/src/operators' |  |  | 0.565 |
| walker |  | 1282 | 74 | package runtime dependencies in packages/d2ts/package.json |  |  | 0.565 |
| ns | 1292 |  | 252 | README "Implementation Details" — provenance and the four core data structures | 1.11 | 1.10 | 0.503 |
| walker |  | 1328 | 46 | listing of 'packages/d2ts/src' |  |  | 0.507 |
| walker |  | 1331 | 3 | listing of 'packages/d2ts/src/electric' |  |  | 0.507 |
| walker |  | 1351 | 20 | listing of 'packages/d2ts/src/sqlite' |  |  | 0.508 |
| ns | 1403 |  | 111 | Every package's own root listing | 2.1 |  | 0.601 |
| walker |  | 1421 | 70 | imports in packages/d2ts/src/index.ts |  |  | 0.603 |
| ns | 1449 |  | 46 | packages/d2ts/src — complete module roster | 2.2 |  | 0.629 |
| walker |  | 1518 | 97 | listing of 'packages/d2ts/src/operators' |  |  | 0.638 |
| ns | 1519 |  | 70 | packages/d2ts/src/index.ts — the public export barrel | 2.3 |  | 0.650 |
| walker |  | 1529 | 11 | export names surface in eslint.base.mjs |  |  | 0.650 |
| ns | 1616 |  | 97 | packages/d2ts/src/operators — complete operator file roster | 2.4 |  | 0.682 |
| walker |  | 1769 | 240 | imports in packages/d2ts/src/operators/index.ts |  |  | 0.691 |
| walker |  | 1828 | 59 | README headline in packages/d2mini/README.md |  |  | 0.691 |
| ns | 1856 |  | 240 | packages/d2ts/src/operators/index.ts — exported operator set | 2.5 | 2.4 | 0.711 |
| walker |  | 1897 | 69 | listing of 'packages/d2ql/src' |  |  | 0.715 |
| walker |  | 1905 | 8 | listing of 'packages/d2ql/src/query-builder' |  |  | 0.716 |
| walker |  | 1962 | 57 | module-doc lede in packages/d2ql/src/index.ts |  |  | 0.717 |
| walker |  | 1975 | 13 | export names surface in packages/d2ts/src/operators/filterBy.ts |  |  | 0.717 |
| ns | 1993 |  | 137 | packages/d2mini/src and its operators directory | 2.6 |  | 0.737 |
| walker |  | 2052 | 77 | README headline in packages/d2ql/README.md |  |  | 0.737 |
| walker |  | 2085 | 33 | packages/d2ql/README.md section #0 |  |  | 0.738 |
| walker |  | 2163 | 78 | README headline in packages/d2ts/README.md |  |  | 0.738 |
| walker |  | 2202 | 39 | packages/d2mini/README.md section #0 |  |  | 0.738 |
| ns | 2258 |  | 265 | packages/d2mini export barrels | 2.7 | 2.6 | 0.697 |
| ns | 2336 |  | 78 | packages/d2ql/src and query-builder/ listings | 2.8 |  | 0.708 |
| walker |  | 2375 | 173 | headings outline in README.md |  |  | 0.759 |
| ns | 2453 |  | 117 | packages/d2ql export barrels | 2.9 | 2.8 | 0.747 |
| ns | 2530 |  | 77 | packages/d2ts/src/sqlite — complete tree listing | 2.10 |  | 0.723 |
| walker |  | 2547 | 172 | README.md section #0 |  |  | 0.740 |
| walker |  | 2572 | 25 | listing of 'examples/electric' |  |  | 0.740 |
| walker |  | 2579 | 7 | listing of 'examples/electric/src' |  |  | 0.741 |
| walker |  | 2605 | 26 | plaintext config pnpm-workspace.yaml |  |  | 0.754 |
| walker |  | 2634 | 29 | listing of 'packages/d2mini/tests' |  |  | 0.754 |
| walker |  | 2665 | 31 | listing of 'examples/d2ql' |  |  | 0.756 |
| ns | 2713 |  | 183 | SQLite subpath export barrels | 2.11 | 2.10 | 0.734 |
| walker |  | 2840 | 175 | package entrypoints in packages/d2ts/package.json |  |  | 0.734 |
| walker |  | 2882 | 42 | listing of 'packages/d2ts/tests' |  |  | 0.735 |
| ns | 2899 |  | 186 | Test suite layout — immediate children of every tests/ directory | 2.12 |  | 0.700 |
| walker |  | 2922 | 40 | listing of 'packages/d2ts/tests/operators-sqlite' |  |  | 0.700 |
| walker |  | 2978 | 56 | listing of 'packages/d2ts/src/sqlite/operators' |  |  | 0.732 |
| ns | 2995 |  | 96 | Example and benchmark source trees | 2.13 | 1.3 | 0.729 |
| walker |  | 3126 | 148 | imports in packages/d2ts/src/sqlite/operators/index.ts |  |  | 0.747 |
| walker |  | 3139 | 13 | export names surface in packages/d2ts/src/sqlite/operators/filterBy.ts |  |  | 0.747 |
| walker |  | 3161 | 22 | README.md section #5 |  |  | 0.747 |
| ns | 3185 |  | 190 | Per-package scripts: build, test, lint, typecheck, format | 3.1 | 1.7 | 0.736 |
| walker |  | 3228 | 67 | export names surface in packages/d2ts/src/d2.ts |  |  | 0.736 |
| walker |  | 3249 | 21 | export at packages/d2ts/src/d2.ts:93 |  |  | 0.736 |
| walker |  | 3271 | 22 | export at packages/d2ts/src/d2.ts:11 |  |  | 0.736 |
| ns | 3272 |  | 87 | d2ts-benchmark scripts — the benchmark entry points | 3.2 | 3.1 | 0.729 |
| walker |  | 3309 | 38 | export names surface in packages/d2ts/src/version-index.ts |  |  | 0.729 |
| walker |  | 3335 | 26 | export names surface in packages/d2ts/src/operators/topK.ts |  |  | 0.729 |
| walker |  | 3406 | 71 | export at packages/d2ts/src/d2.ts:166 |  |  | 0.729 |
| ns | 3465 |  | 193 | CI pipeline stages | 3.3 |  | 0.714 |
| walker |  | 3594 | 188 | package scripts in packages/d2ts/package.json |  |  | 0.729 |
| walker |  | 3618 | 24 | README.md section #2 |  |  | 0.729 |
| ns | 3669 |  | 204 | README: constructing a D2 graph and sending to an input stream | 4.1 | 1.10 | 0.720 |
| walker |  | 3767 | 149 | README.md section #11 |  |  | 0.721 |
| walker |  | 3798 | 31 | export names surface in packages/d2ts/src/operators/debug.ts |  |  | 0.721 |
| walker |  | 3831 | 33 | export at packages/d2ts/src/operators/debug.ts:74 |  |  | 0.721 |
| walker |  | 3858 | 27 | README.md section #4 |  |  | 0.721 |
| walker |  | 3882 | 24 | README.md section #10 |  |  | 0.721 |
| walker |  | 3908 | 26 | export body at packages/d2ts/src/d2.ts:166 body 171 |  |  | 0.721 |
| walker |  | 3932 | 24 | export names surface in packages/d2ts/src/sqlite/operators/groupBy.ts |  |  | 0.721 |
| ns | 3994 |  | 325 | d2.ts — D2Options and the complete D2 class method roster | 4.2 |  | 0.698 |
| walker |  | 4047 | 115 | listing of 'packages/d2ql/tests' |  |  | 0.742 |
| walker |  | 4100 | 53 | listing of 'packages/d2ql/tests/query-builder' |  |  | 0.742 |
| walker |  | 4136 | 36 | export names surface in packages/d2ts/src/operators/count.ts |  |  | 0.742 |
| ns | 4184 |  | 190 | README: what versions and frontiers actually mean | 4.3 | 1.10 | 0.738 |
| walker |  | 4191 | 55 | export names surface in packages/d2ts/src/order.ts |  |  | 0.738 |
| walker |  | 4191 | 0 | export at packages/d2ts/src/order.ts:10 |  |  | 0.738 |
| walker |  | 4203 | 12 | export at packages/d2ts/src/order.ts:277 |  |  | 0.738 |
| walker |  | 4211 | 8 | export body at packages/d2ts/src/order.ts:277 body 279 |  |  | 0.738 |
| walker |  | 4349 | 138 | listing of 'packages/d2mini/tests/operators' |  |  | 0.738 |
| walker |  | 4405 | 56 | export names surface in packages/d2ts/src/multiset.ts |  |  | 0.738 |
| walker |  | 4442 | 37 | export names surface in packages/d2ts/src/operators/distinct.ts |  |  | 0.738 |
| walker |  | 4468 | 26 | export names surface in packages/d2ts/src/sqlite/operators/topK.ts |  |  | 0.738 |
| ns | 4530 |  | 346 | order.ts — v() factory and the complete Version method roster | 4.4 |  | 0.712 |
| walker |  | 4537 | 69 | export at packages/d2ts/src/operators/count.ts:10 |  |  | 0.712 |
| walker |  | 4606 | 69 | export at packages/d2ts/src/operators/distinct.ts:10 |  |  | 0.712 |
| walker |  | 4764 | 158 | listing of 'packages/d2ts/tests/operators' |  |  | 0.712 |
| walker |  | 4804 | 40 | export names surface in packages/d2ts/src/operators/buffer.ts |  |  | 0.712 |
| walker |  | 4804 | 0 | export at packages/d2ts/src/operators/buffer.ts:62 |  |  | 0.712 |
| walker |  | 4815 | 11 | export at packages/d2ts/src/operators/buffer.ts:17 |  |  | 0.712 |
| ns | 4820 |  | 290 | order.ts — Antichain method roster and Frontier | 4.5 | 4.4 | 0.692 |
| walker |  | 4873 | 58 | export at packages/d2ts/src/operators/count.ts:32 |  |  | 0.692 |
| walker |  | 4931 | 58 | export at packages/d2ts/src/operators/distinct.ts:37 |  |  | 0.692 |
| ns | 4965 |  | 145 | README: MultiSet as a changeset, and keyed multisets | 4.6 | 1.10 | 0.689 |
| walker |  | 4972 | 41 | export names surface in packages/d2ts/src/operators/concat.ts |  |  | 0.689 |
| walker |  | 4981 | 9 | export at packages/d2ts/src/operators/concat.ts:11 |  |  | 0.689 |
| walker |  | 5010 | 29 | export at packages/d2ts/src/operators/concat.ts:54 |  |  | 0.689 |
| walker |  | 5051 | 41 | export names surface in packages/d2ts/src/operators/consolidate.ts |  |  | 0.689 |
| walker |  | 5051 | 0 | export at packages/d2ts/src/operators/consolidate.ts:61 |  |  | 0.689 |
| walker |  | 5062 | 11 | export at packages/d2ts/src/operators/consolidate.ts:16 |  |  | 0.689 |
| walker |  | 5103 | 41 | export names surface in packages/d2ts/src/operators/topKWithFractionalIndex.ts |  |  | 0.689 |
| walker |  | 5146 | 43 | export names surface in packages/d2ts/src/operators/reduce.ts |  |  | 0.689 |
| walker |  | 5190 | 44 | export names surface in packages/d2ts/src/operators/negate.ts |  |  | 0.689 |
| walker |  | 5190 | 0 | export at packages/d2ts/src/operators/negate.ts:19 |  |  | 0.689 |
| walker |  | 5207 | 17 | export at packages/d2ts/src/operators/negate.ts:10 |  |  | 0.689 |
| walker |  | 5250 | 43 | export at packages/d2ts/src/sqlite/operators/groupBy.ts:49 |  |  | 0.689 |
| walker |  | 5282 | 32 | export names surface in packages/d2ts/src/sqlite/operators/count.ts |  |  | 0.689 |
| walker |  | 5315 | 33 | export names surface in packages/d2ts/src/sqlite/operators/distinct.ts |  |  | 0.689 |
| ns | 5323 |  | 358 | multiset.ts — MultiSetArray, KeyedData and every MultiSet method | 4.7 |  | 0.673 |
| walker |  | 5330 | 15 | imports in packages/d2ts/src/order.ts |  |  | 0.673 |
| walker |  | 5486 | 156 | export at packages/d2ts/src/version-index.ts:8 |  |  | 0.673 |
| walker |  | 5516 | 30 | packages/d2ql/README.md section #4 |  |  | 0.673 |
| walker |  | 5566 | 50 | export names surface in packages/d2ts/src/operators/output.ts |  |  | 0.673 |
| walker |  | 5566 | 0 | export at packages/d2ts/src/operators/output.ts:61 |  |  | 0.673 |
| ns | 5623 |  | 300 | types.ts — the operator message protocol | 4.8 |  | 0.650 |
| walker |  | 5656 | 90 | export at packages/d2ts/src/operators/output.ts:19 |  |  | 0.650 |
| walker |  | 5691 | 35 | export names surface in packages/d2ts/src/sqlite/operators/reduce.ts |  |  | 0.650 |
| walker |  | 5721 | 30 | README.md section #20 |  |  | 0.651 |
| walker |  | 5815 | 94 | export at packages/d2ts/src/operators/debug.ts:18 |  |  | 0.651 |
| walker |  | 5867 | 52 | export names surface in packages/d2ts/src/operators/filter.ts |  |  | 0.651 |
| walker |  | 5867 | 0 | export at packages/d2ts/src/operators/filter.ts:34 |  |  | 0.651 |
| ns | 5881 |  | 258 | types.ts — PipedOperator and the 20-deep pipe overload wall | 4.9 | 4.8 | 0.639 |
| walker |  | 5897 | 30 | README.md section #6 |  |  | 0.639 |
| walker |  | 5994 | 97 | export at packages/d2ts/src/operators/filter.ts:11 |  |  | 0.639 |
| walker |  | 6025 | 31 | README.md section #12 |  |  | 0.639 |
| walker |  | 6078 | 53 | export names surface in packages/d2ts/src/operators/orderBy.ts |  |  | 0.639 |
| walker |  | 6137 | 59 | export at packages/d2ts/src/operators/orderBy.ts:23 |  |  | 0.639 |
| walker |  | 6215 | 78 | export at packages/d2ts/src/operators/filterBy.ts:14 |  |  | 0.639 |
| ns | 6264 |  | 383 | README Key Features — every operator with a one-line description | 5.1 | 1.10 | 0.627 |
| walker |  | 6271 | 56 | export names surface in packages/d2ts/src/operators/map.ts |  |  | 0.627 |
| walker |  | 6271 | 0 | export at packages/d2ts/src/operators/map.ts:34 |  |  | 0.627 |
| walker |  | 6368 | 97 | export at packages/d2ts/src/operators/map.ts:11 |  |  | 0.627 |
| walker |  | 6402 | 34 | imports in packages/d2ql/src/index.ts |  |  | 0.632 |
| walker |  | 6487 | 85 | export at packages/d2ts/src/operators/reduce.ts:127 |  |  | 0.632 |
| walker |  | 6596 | 109 | export at packages/d2ts/src/operators/reduce.ts:15 |  |  | 0.632 |
| walker |  | 6637 | 41 | export names surface in packages/d2ts/src/sqlite/operators/topKWithFractionalIndex.ts |  |  | 0.632 |
| ns | 6694 |  | 430 | packages/d2ts/README.md — complete heading map (1081-line API reference) | 5.2 |  | 0.607 |
| walker |  | 6724 | 87 | export at packages/d2ts/src/operators/orderBy.ts:76 |  |  | 0.607 |
| walker |  | 6811 | 87 | export at packages/d2ts/src/operators/orderBy.ts:140 |  |  | 0.607 |
| walker |  | 6830 | 19 | imports in packages/d2ts/src/multiset.ts |  |  | 0.607 |
| ns | 6857 |  | 163 | Operator factory roster — single-stream operators | 5.3 | 2.5 | 0.612 |
| walker |  | 7013 | 183 | README.md section #1 |  |  | 0.612 |
| walker |  | 7188 | 175 | headings outline in packages/d2ts/README.md |  |  | 0.621 |
| ns | 7257 |  | 400 | Operator factory roster — keyed, join, ordering and aggregate families | 5.4 | 5.3 | 0.604 |
| walker |  | 7360 | 172 | packages/d2ts/README.md section #0 |  |  | 0.604 |
| walker |  | 7421 | 61 | export at packages/d2ts/src/sqlite/operators/groupBy.ts:101 |  |  | 0.604 |
| ns | 7459 |  | 202 | Operator implementation classes and the LinearUnaryOperator base | 5.5 | 5.4 | 0.604 |
| walker |  | 7601 | 180 | export at packages/d2ts/src/d2.ts:15 |  |  | 0.615 |
| ns | 7635 |  | 176 | groupBy.ts — the AggregateFunction contract | 5.6 | 5.4 | 0.608 |
| ns | 7855 |  | 220 | D2QL identity and README section map | 6.1 |  | 0.604 |
| walker |  | 7858 | 257 | README.md section #43 |  |  | 0.631 |
| walker |  | 7921 | 63 | export at packages/d2ts/src/sqlite/operators/count.ts:39 |  |  | 0.631 |
| walker |  | 7984 | 63 | export at packages/d2ts/src/sqlite/operators/distinct.ts:44 |  |  | 0.631 |
| walker |  | 8029 | 45 | export names surface in packages/d2ts/src/sqlite/operators/buffer.ts |  |  | 0.631 |
| walker |  | 8029 | 0 | export at packages/d2ts/src/sqlite/operators/buffer.ts:149 |  |  | 0.631 |
| walker |  | 8075 | 46 | export names surface in packages/d2ts/src/sqlite/operators/consolidate.ts |  |  | 0.631 |
| walker |  | 8075 | 0 | export at packages/d2ts/src/sqlite/operators/consolidate.ts:148 |  |  | 0.631 |
| ns | 8097 |  | 242 | D2QL "Current Features" — the supported SQL subset | 6.2 | 6.1 | 0.622 |
| walker |  | 8159 | 84 | export at packages/d2ts/src/sqlite/operators/buffer.ts:32 |  |  | 0.622 |
| walker |  | 8243 | 84 | export at packages/d2ts/src/sqlite/operators/consolidate.ts:31 |  |  | 0.622 |
| walker |  | 8438 | 195 | export at packages/d2ts/src/order.ts:29 |  |  | 0.633 |
| ns | 8455 |  | 358 | schema.ts — the Query interface family | 6.3 |  | 0.619 |
| walker |  | 8543 | 105 | README.md section #44 |  |  | 0.619 |
| walker |  | 8564 | 21 | export doc at packages/d2ts/src/operators/reduce.ts:15 |  |  | 0.619 |
| ns | 8617 |  | 162 | schema.ts — the Comparator and LogicalOperator vocabularies | 6.4 | 6.3 | 0.611 |
| walker |  | 8633 | 69 | export names surface in packages/d2ts/src/operators/iterate.ts |  |  | 0.617 |
| walker |  | 8642 | 9 | export at packages/d2ts/src/operators/iterate.ts:15 |  |  | 0.617 |
| walker |  | 8651 | 9 | export at packages/d2ts/src/operators/iterate.ts:46 |  |  | 0.617 |
| walker |  | 8678 | 27 | export at packages/d2ts/src/operators/iterate.ts:225 |  |  | 0.617 |
| ns | 8748 |  | 131 | compileQuery — the package's single entry point | 6.5 |  | 0.612 |
| walker |  | 8761 | 83 | export at packages/d2ts/src/operators/iterate.ts:80 |  |  | 0.612 |
| walker |  | 8864 | 103 | export at packages/d2ts/src/operators/topK.ts:22 |  |  | 0.612 |
| walker |  | 8886 | 22 | export doc at packages/d2ts/src/operators/count.ts:10 |  |  | 0.612 |
| walker |  | 8908 | 22 | export doc at packages/d2ts/src/operators/distinct.ts:10 |  |  | 0.612 |
| walker |  | 8931 | 23 | export doc at packages/d2ts/src/operators/concat.ts:11 |  |  | 0.612 |
| ns | 8991 |  | 243 | query-builder.ts — the fluent builder's complete method set | 6.6 |  | 0.603 |
| walker |  | 9043 | 112 | export at packages/d2ts/src/operators/topK.ts:60 |  |  | 0.603 |
| ns | 9089 |  | 98 | d2mini README — how it differs from d2ts | 7.1 |  | 0.604 |
| walker |  | 9096 | 53 | export names surface in packages/d2ts/src/sqlite/operators/orderBy.ts |  |  | 0.604 |
| walker |  | 9155 | 59 | export at packages/d2ts/src/sqlite/operators/orderBy.ts:26 |  |  | 0.604 |
| ns | 9178 |  | 89 | d2mini/src/d2.ts — the versionless D2 constructor | 7.2 | 7.1 | 0.600 |
| walker |  | 9202 | 47 | imports in packages/d2mini/src/index.ts |  |  | 0.601 |
| walker |  | 9222 | 20 | export doc at packages/d2ts/src/operators/distinct.ts:37 |  |  | 0.601 |
| walker |  | 9368 | 146 | export at packages/d2ts/src/operators/topKWithFractionalIndex.ts:29 |  |  | 0.601 |
| walker |  | 9392 | 24 | export doc at packages/d2ts/src/operators/consolidate.ts:16 |  |  | 0.601 |
| ns | 9414 |  | 236 | README: using the SQLite backend | 8.1 | 1.10 | 0.596 |
| walker |  | 9416 | 24 | export doc at packages/d2ts/src/operators/debug.ts:18 |  |  | 0.596 |
| walker |  | 9440 | 24 | export doc at packages/d2ts/src/operators/filter.ts:11 |  |  | 0.596 |
| walker |  | 9464 | 24 | export doc at packages/d2ts/src/operators/output.ts:19 |  |  | 0.596 |
| ns | 9542 |  | 128 | sqlite/database.ts — the SQLiteDb driver interface | 8.2 | 8.1 | 0.590 |
| walker |  | 9580 | 116 | export at packages/d2ts/src/operators/topKWithFractionalIndex.ts:372 |  |  | 0.590 |
| ns | 9696 |  | 154 | withSQLite, SQLIndex and the Electric bridge exports | 8.3 | 8.2 | 0.586 |
| walker |  | 9816 | 236 | export at packages/d2ts/src/version-index.ts:28 |  |  | 0.586 |
| walker |  | 9837 | 21 | export doc at packages/d2ts/src/operators/iterate.ts:225 |  |  | 0.586 |
| ns | 9840 |  | 144 | README: what each example demonstrates | 9.1 | 1.10 | 0.590 |
| walker |  | 9862 | 25 | export doc at packages/d2ts/src/operators/iterate.ts:15 |  |  | 0.590 |
| walker |  | 9887 | 25 | export doc at packages/d2ts/src/operators/iterate.ts:46 |  |  | 0.590 |
| walker |  | 9968 | 81 | export names surface in packages/d2ts/src/operators/keying.ts |  |  | 0.592 |
| walker |  | 9968 | 0 | export at packages/d2ts/src/operators/keying.ts:7 |  |  | 0.592 |
| walker |  | 9968 | 0 | export at packages/d2ts/src/operators/keying.ts:22 |  |  | 0.592 |
| ns | 9990 |  | 150 | examples/electric — what the demo app does | 9.2 | 2.13 | 0.589 |

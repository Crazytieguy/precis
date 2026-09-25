Score(3000)=0.700 I=0.827 C=0.592 ns_rows≤3K=24/55 grid(1000/1442/2080/3000/4327/6240/9000)=0.629/0.629/0.738/0.700/0.738/0.643/0.578

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 54 | 54 | listing of '.' |  |  | 0.000 |
| walker |  | 57 | 3 | listing of '.github' |  |  | 0.000 |
| walker |  | 61 | 4 | listing of '.github/workflows' |  |  | 0.000 |
| ns | 81 |  | 81 | Repository identity — README title + one-line definition | 1.1 |  | 0.000 |
| walker |  | 83 | 22 | listing of 'packages' |  |  | 0.000 |
| walker |  | 118 | 35 | listing of 'examples' |  |  | 0.000 |
| walker |  | 135 | 17 | listing of 'packages/d2ts-benchmark' |  |  | 0.715 |
| ns | 135 |  | 54 | Complete repository root listing | 1.2 |  | 0.715 |
| walker |  | 147 | 12 | listing of 'packages/d2ts-benchmark/src' |  |  | 0.715 |
| walker |  | 155 | 8 | listing of '.changeset' |  |  | 0.715 |
| walker |  | 180 | 25 | listing of 'packages/d2ql' |  |  | 0.720 |
| ns | 192 |  | 57 | Workspace membership — packages/ and examples/ listings | 1.3 |  | 0.700 |
| walker |  | 210 | 30 | listing of 'packages/d2mini' |  |  | 0.710 |
| ns | 218 |  | 26 | pnpm workspace globs | 1.4 |  | 0.668 |
| ns | 288 |  | 70 | README lede — what incremental execution buys you | 1.5 | 1.1 | 0.655 |
| walker |  | 291 | 81 | README headline in README.md |  |  | 0.934 |
| walker |  | 327 | 36 | listing of 'packages/d2ts' |  |  | 0.937 |
| ns | 387 |  | 99 | README lede — ElectricSQL ShapeStreams and pipeline type inference | 1.6 | 1.1 | 0.898 |
| walker |  | 412 | 85 | package identity in packages/d2ts/package.json |  |  | 0.898 |
| ns | 578 |  | 191 | Root package.json — identity and every workspace script | 1.7 |  | 0.728 |
| ns | 728 |  | 150 | Published package identity: d2ts and d2mini | 1.8 |  | 0.686 |
| ns | 871 |  | 143 | Private package identity: d2ql and d2ts-benchmark | 1.9 | 1.8 | 0.629 |
| ns | 1032 |  | 161 | Root README section map (every H2) | 1.10 |  | 0.554 |
| walker |  | 1064 | 652 | YAML config at .github/workflows/ci.yml |  |  | 0.555 |
| walker |  | 1097 | 33 | listing of 'packages/d2mini/src' |  |  | 0.556 |
| walker |  | 1201 | 104 | listing of 'packages/d2mini/src/operators' |  |  | 0.565 |
| walker |  | 1275 | 74 | package runtime dependencies in packages/d2ts/package.json |  |  | 0.565 |
| ns | 1284 |  | 252 | README "Implementation Details" — provenance and the four core data structures | 1.11 | 1.10 | 0.503 |
| walker |  | 1319 | 44 | listing of 'packages/d2ts/src' |  |  | 0.507 |
| walker |  | 1323 | 4 | listing of 'packages/d2ts/src/electric' |  |  | 0.507 |
| walker |  | 1343 | 20 | listing of 'packages/d2ts/src/sqlite' |  |  | 0.508 |
| ns | 1392 |  | 108 | Every package's own root listing | 2.1 |  | 0.601 |
| walker |  | 1413 | 70 | imports in packages/d2ts/src/index.ts |  |  | 0.603 |
| ns | 1436 |  | 44 | packages/d2ts/src — complete module roster | 2.2 |  | 0.629 |
| ns | 1506 |  | 70 | packages/d2ts/src/index.ts — the public export barrel | 2.3 |  | 0.640 |
| walker |  | 1511 | 98 | listing of 'packages/d2ts/src/operators' |  |  | 0.650 |
| walker |  | 1522 | 11 | export names surface in eslint.base.mjs |  |  | 0.650 |
| ns | 1604 |  | 98 | packages/d2ts/src/operators — complete operator file roster | 2.4 |  | 0.682 |
| walker |  | 1762 | 240 | imports in packages/d2ts/src/operators/index.ts |  |  | 0.691 |
| walker |  | 1821 | 59 | README headline in packages/d2mini/README.md |  |  | 0.691 |
| ns | 1844 |  | 240 | packages/d2ts/src/operators/index.ts — exported operator set | 2.5 | 2.4 | 0.711 |
| walker |  | 1890 | 69 | listing of 'packages/d2ql/src' |  |  | 0.715 |
| walker |  | 1899 | 9 | listing of 'packages/d2ql/src/query-builder' |  |  | 0.716 |
| walker |  | 1956 | 57 | module-doc lede in packages/d2ql/src/index.ts |  |  | 0.717 |
| walker |  | 1969 | 13 | export names surface in packages/d2ts/src/operators/filterBy.ts |  |  | 0.717 |
| ns | 1981 |  | 137 | packages/d2mini/src and its operators directory | 2.6 |  | 0.737 |
| walker |  | 2046 | 77 | README headline in packages/d2ql/README.md |  |  | 0.737 |
| walker |  | 2079 | 33 | packages/d2ql/README.md section #0 |  |  | 0.738 |
| walker |  | 2157 | 78 | README headline in packages/d2ts/README.md |  |  | 0.738 |
| walker |  | 2181 | 24 | listing of 'examples/electric' |  |  | 0.738 |
| walker |  | 2189 | 8 | listing of 'examples/electric/src' |  |  | 0.739 |
| walker |  | 2228 | 39 | packages/d2mini/README.md section #0 |  |  | 0.739 |
| ns | 2246 |  | 265 | packages/d2mini export barrels | 2.7 | 2.6 | 0.698 |
| ns | 2324 |  | 78 | packages/d2ql/src and query-builder/ listings | 2.8 |  | 0.710 |
| walker |  | 2401 | 173 | headings outline in README.md |  |  | 0.760 |
| ns | 2441 |  | 117 | packages/d2ql export barrels | 2.9 | 2.8 | 0.748 |
| ns | 2518 |  | 77 | packages/d2ts/src/sqlite — complete tree listing | 2.10 |  | 0.725 |
| walker |  | 2573 | 172 | README.md section #0 |  |  | 0.741 |
| walker |  | 2599 | 26 | plaintext config pnpm-workspace.yaml |  |  | 0.754 |
| walker |  | 2628 | 29 | listing of 'packages/d2mini/tests' |  |  | 0.754 |
| walker |  | 2660 | 32 | listing of 'examples/d2ql' |  |  | 0.756 |
| ns | 2701 |  | 183 | SQLite subpath export barrels | 2.11 | 2.10 | 0.734 |
| walker |  | 2835 | 175 | package entrypoints in packages/d2ts/package.json |  |  | 0.734 |
| walker |  | 2876 | 41 | listing of 'packages/d2ts/tests' |  |  | 0.735 |
| ns | 2886 |  | 185 | Test suite layout — immediate children of every tests/ directory | 2.12 |  | 0.700 |
| walker |  | 2898 | 22 | README.md section #5 |  |  | 0.700 |
| walker |  | 2965 | 67 | export names surface in packages/d2ts/src/d2.ts |  |  | 0.700 |
| ns | 2982 |  | 96 | Example and benchmark source trees | 2.13 | 1.3 | 0.700 |
| walker |  | 2986 | 21 | export at packages/d2ts/src/d2.ts:93 |  |  | 0.700 |
| walker |  | 3008 | 22 | export at packages/d2ts/src/d2.ts:11 |  |  | 0.700 |
| walker |  | 3065 | 57 | listing of 'packages/d2ts/src/sqlite/operators' |  |  | 0.729 |
| ns | 3172 |  | 190 | Per-package scripts: build, test, lint, typecheck, format | 3.1 | 1.7 | 0.719 |
| walker |  | 3213 | 148 | imports in packages/d2ts/src/sqlite/operators/index.ts |  |  | 0.736 |
| walker |  | 3226 | 13 | export names surface in packages/d2ts/src/sqlite/operators/filterBy.ts |  |  | 0.736 |
| ns | 3259 |  | 87 | d2ts-benchmark scripts — the benchmark entry points | 3.2 | 3.1 | 0.729 |
| walker |  | 3264 | 38 | export names surface in packages/d2ts/src/version-index.ts |  |  | 0.729 |
| walker |  | 3290 | 26 | export names surface in packages/d2ts/src/operators/topK.ts |  |  | 0.729 |
| walker |  | 3361 | 71 | export at packages/d2ts/src/d2.ts:166 |  |  | 0.729 |
| ns | 3452 |  | 193 | CI pipeline stages | 3.3 |  | 0.714 |
| walker |  | 3549 | 188 | package scripts in packages/d2ts/package.json |  |  | 0.729 |
| walker |  | 3573 | 24 | README.md section #2 |  |  | 0.729 |
| walker |  | 3604 | 31 | export names surface in packages/d2ts/src/operators/debug.ts |  |  | 0.730 |
| walker |  | 3637 | 33 | export at packages/d2ts/src/operators/debug.ts:74 |  |  | 0.730 |
| ns | 3656 |  | 204 | README: constructing a D2 graph and sending to an input stream | 4.1 | 1.10 | 0.720 |
| walker |  | 3664 | 27 | README.md section #4 |  |  | 0.720 |
| walker |  | 3688 | 24 | README.md section #10 |  |  | 0.720 |
| walker |  | 3714 | 26 | export body at packages/d2ts/src/d2.ts:166 body 171 |  |  | 0.720 |
| walker |  | 3738 | 24 | export names surface in packages/d2ts/src/sqlite/operators/groupBy.ts |  |  | 0.720 |
| walker |  | 3853 | 115 | listing of 'packages/d2ql/tests' |  |  | 0.766 |
| walker |  | 3889 | 36 | export names surface in packages/d2ts/src/operators/count.ts |  |  | 0.766 |
| walker |  | 3930 | 41 | listing of 'packages/d2ts/tests/operators-sqlite' |  |  | 0.766 |
| ns | 3981 |  | 325 | d2.ts — D2Options and the complete D2 class method roster | 4.2 |  | 0.742 |
| walker |  | 3985 | 55 | export names surface in packages/d2ts/src/order.ts |  |  | 0.742 |
| walker |  | 3985 | 0 | export at packages/d2ts/src/order.ts:10 |  |  | 0.742 |
| walker |  | 3997 | 12 | export at packages/d2ts/src/order.ts:277 |  |  | 0.742 |
| walker |  | 4005 | 8 | export body at packages/d2ts/src/order.ts:277 body 279 |  |  | 0.742 |
| walker |  | 4061 | 56 | export names surface in packages/d2ts/src/multiset.ts |  |  | 0.742 |
| walker |  | 4098 | 37 | export names surface in packages/d2ts/src/operators/distinct.ts |  |  | 0.742 |
| walker |  | 4124 | 26 | export names surface in packages/d2ts/src/sqlite/operators/topK.ts |  |  | 0.742 |
| ns | 4171 |  | 190 | README: what versions and frontiers actually mean | 4.3 | 1.10 | 0.738 |
| walker |  | 4193 | 69 | export at packages/d2ts/src/operators/count.ts:10 |  |  | 0.738 |
| walker |  | 4262 | 69 | export at packages/d2ts/src/operators/distinct.ts:10 |  |  | 0.738 |
| walker |  | 4302 | 40 | export names surface in packages/d2ts/src/operators/buffer.ts |  |  | 0.738 |
| walker |  | 4302 | 0 | export at packages/d2ts/src/operators/buffer.ts:62 |  |  | 0.738 |
| walker |  | 4313 | 11 | export at packages/d2ts/src/operators/buffer.ts:17 |  |  | 0.738 |
| walker |  | 4371 | 58 | export at packages/d2ts/src/operators/count.ts:32 |  |  | 0.738 |
| walker |  | 4429 | 58 | export at packages/d2ts/src/operators/distinct.ts:37 |  |  | 0.738 |
| walker |  | 4470 | 41 | export names surface in packages/d2ts/src/operators/concat.ts |  |  | 0.738 |
| walker |  | 4479 | 9 | export at packages/d2ts/src/operators/concat.ts:11 |  |  | 0.738 |
| walker |  | 4508 | 29 | export at packages/d2ts/src/operators/concat.ts:54 |  |  | 0.738 |
| ns | 4517 |  | 346 | order.ts — v() factory and the complete Version method roster | 4.4 |  | 0.712 |
| walker |  | 4549 | 41 | export names surface in packages/d2ts/src/operators/consolidate.ts |  |  | 0.712 |
| walker |  | 4549 | 0 | export at packages/d2ts/src/operators/consolidate.ts:61 |  |  | 0.712 |
| walker |  | 4560 | 11 | export at packages/d2ts/src/operators/consolidate.ts:16 |  |  | 0.712 |
| walker |  | 4601 | 41 | export names surface in packages/d2ts/src/operators/topKWithFractionalIndex.ts |  |  | 0.712 |
| walker |  | 4644 | 43 | export names surface in packages/d2ts/src/operators/reduce.ts |  |  | 0.712 |
| walker |  | 4688 | 44 | export names surface in packages/d2ts/src/operators/negate.ts |  |  | 0.713 |
| walker |  | 4688 | 0 | export at packages/d2ts/src/operators/negate.ts:19 |  |  | 0.713 |
| walker |  | 4705 | 17 | export at packages/d2ts/src/operators/negate.ts:10 |  |  | 0.713 |
| walker |  | 4748 | 43 | export at packages/d2ts/src/sqlite/operators/groupBy.ts:49 |  |  | 0.713 |
| walker |  | 4780 | 32 | export names surface in packages/d2ts/src/sqlite/operators/count.ts |  |  | 0.713 |
| ns | 4807 |  | 290 | order.ts — Antichain method roster and Frontier | 4.5 | 4.4 | 0.692 |
| walker |  | 4834 | 54 | listing of 'packages/d2ql/tests/query-builder' |  |  | 0.692 |
| walker |  | 4867 | 33 | export names surface in packages/d2ts/src/sqlite/operators/distinct.ts |  |  | 0.692 |
| walker |  | 4882 | 15 | imports in packages/d2ts/src/order.ts |  |  | 0.692 |
| ns | 4952 |  | 145 | README: MultiSet as a changeset, and keyed multisets | 4.6 | 1.10 | 0.689 |
| walker |  | 5038 | 156 | export at packages/d2ts/src/version-index.ts:8 |  |  | 0.689 |
| walker |  | 5068 | 30 | packages/d2ql/README.md section #4 |  |  | 0.689 |
| walker |  | 5118 | 50 | export names surface in packages/d2ts/src/operators/output.ts |  |  | 0.689 |
| walker |  | 5118 | 0 | export at packages/d2ts/src/operators/output.ts:61 |  |  | 0.689 |
| walker |  | 5208 | 90 | export at packages/d2ts/src/operators/output.ts:19 |  |  | 0.689 |
| walker |  | 5243 | 35 | export names surface in packages/d2ts/src/sqlite/operators/reduce.ts |  |  | 0.689 |
| walker |  | 5273 | 30 | README.md section #20 |  |  | 0.690 |
| ns | 5310 |  | 358 | multiset.ts — MultiSetArray, KeyedData and every MultiSet method | 4.7 |  | 0.673 |
| walker |  | 5367 | 94 | export at packages/d2ts/src/operators/debug.ts:18 |  |  | 0.673 |
| walker |  | 5419 | 52 | export names surface in packages/d2ts/src/operators/filter.ts |  |  | 0.673 |
| walker |  | 5419 | 0 | export at packages/d2ts/src/operators/filter.ts:34 |  |  | 0.673 |
| walker |  | 5449 | 30 | README.md section #6 |  |  | 0.673 |
| walker |  | 5546 | 97 | export at packages/d2ts/src/operators/filter.ts:11 |  |  | 0.673 |
| walker |  | 5577 | 31 | README.md section #12 |  |  | 0.673 |
| ns | 5610 |  | 300 | types.ts — the operator message protocol | 4.8 |  | 0.650 |
| walker |  | 5630 | 53 | export names surface in packages/d2ts/src/operators/orderBy.ts |  |  | 0.650 |
| walker |  | 5689 | 59 | export at packages/d2ts/src/operators/orderBy.ts:23 |  |  | 0.650 |
| walker |  | 5767 | 78 | export at packages/d2ts/src/operators/filterBy.ts:14 |  |  | 0.650 |
| walker |  | 5823 | 56 | export names surface in packages/d2ts/src/operators/map.ts |  |  | 0.651 |
| walker |  | 5823 | 0 | export at packages/d2ts/src/operators/map.ts:34 |  |  | 0.651 |
| ns | 5868 |  | 258 | types.ts — PipedOperator and the 20-deep pipe overload wall | 4.9 | 4.8 | 0.638 |
| walker |  | 5920 | 97 | export at packages/d2ts/src/operators/map.ts:11 |  |  | 0.638 |
| walker |  | 5954 | 34 | imports in packages/d2ql/src/index.ts |  |  | 0.643 |
| walker |  | 6093 | 139 | listing of 'packages/d2mini/tests/operators' |  |  | 0.643 |
| walker |  | 6178 | 85 | export at packages/d2ts/src/operators/reduce.ts:127 |  |  | 0.643 |
| ns | 6251 |  | 383 | README Key Features — every operator with a one-line description | 5.1 | 1.10 | 0.631 |
| walker |  | 6287 | 109 | export at packages/d2ts/src/operators/reduce.ts:15 |  |  | 0.631 |
| walker |  | 6328 | 41 | export names surface in packages/d2ts/src/sqlite/operators/topKWithFractionalIndex.ts |  |  | 0.631 |
| walker |  | 6415 | 87 | export at packages/d2ts/src/operators/orderBy.ts:76 |  |  | 0.631 |
| walker |  | 6502 | 87 | export at packages/d2ts/src/operators/orderBy.ts:140 |  |  | 0.631 |
| walker |  | 6521 | 19 | imports in packages/d2ts/src/multiset.ts |  |  | 0.631 |
| ns | 6681 |  | 430 | packages/d2ts/README.md — complete heading map (1081-line API reference) | 5.2 |  | 0.607 |
| walker |  | 6704 | 183 | README.md section #1 |  |  | 0.607 |
| ns | 6844 |  | 163 | Operator factory roster — single-stream operators | 5.3 | 2.5 | 0.611 |
| walker |  | 6879 | 175 | headings outline in packages/d2ts/README.md |  |  | 0.621 |
| walker |  | 7051 | 172 | packages/d2ts/README.md section #0 |  |  | 0.621 |
| walker |  | 7112 | 61 | export at packages/d2ts/src/sqlite/operators/groupBy.ts:101 |  |  | 0.621 |
| ns | 7244 |  | 400 | Operator factory roster — keyed, join, ordering and aggregate families | 5.4 | 5.3 | 0.604 |
| walker |  | 7271 | 159 | listing of 'packages/d2ts/tests/operators' |  |  | 0.604 |
| ns | 7446 |  | 202 | Operator implementation classes and the LinearUnaryOperator base | 5.5 | 5.4 | 0.603 |
| walker |  | 7451 | 180 | export at packages/d2ts/src/d2.ts:15 |  |  | 0.615 |
| walker |  | 7514 | 63 | export at packages/d2ts/src/sqlite/operators/count.ts:39 |  |  | 0.615 |
| walker |  | 7577 | 63 | export at packages/d2ts/src/sqlite/operators/distinct.ts:44 |  |  | 0.615 |
| walker |  | 7622 | 45 | export names surface in packages/d2ts/src/sqlite/operators/buffer.ts |  |  | 0.608 |
| walker |  | 7622 | 0 | export at packages/d2ts/src/sqlite/operators/buffer.ts:149 |  |  | 0.608 |
| ns | 7622 |  | 176 | groupBy.ts — the AggregateFunction contract | 5.6 | 5.4 | 0.608 |
| walker |  | 7668 | 46 | export names surface in packages/d2ts/src/sqlite/operators/consolidate.ts |  |  | 0.608 |
| walker |  | 7668 | 0 | export at packages/d2ts/src/sqlite/operators/consolidate.ts:148 |  |  | 0.608 |
| walker |  | 7752 | 84 | export at packages/d2ts/src/sqlite/operators/buffer.ts:32 |  |  | 0.608 |
| walker |  | 7836 | 84 | export at packages/d2ts/src/sqlite/operators/consolidate.ts:31 |  |  | 0.608 |
| ns | 7842 |  | 220 | D2QL identity and README section map | 6.1 |  | 0.603 |
| walker |  | 8031 | 195 | export at packages/d2ts/src/order.ts:29 |  |  | 0.614 |
| ns | 8084 |  | 242 | D2QL "Current Features" — the supported SQL subset | 6.2 | 6.1 | 0.606 |
| walker |  | 8136 | 105 | README.md section #44 |  |  | 0.606 |
| walker |  | 8157 | 21 | export doc at packages/d2ts/src/operators/reduce.ts:15 |  |  | 0.606 |
| walker |  | 8226 | 69 | export names surface in packages/d2ts/src/operators/iterate.ts |  |  | 0.612 |
| walker |  | 8235 | 9 | export at packages/d2ts/src/operators/iterate.ts:15 |  |  | 0.612 |
| walker |  | 8244 | 9 | export at packages/d2ts/src/operators/iterate.ts:46 |  |  | 0.612 |
| walker |  | 8271 | 27 | export at packages/d2ts/src/operators/iterate.ts:225 |  |  | 0.612 |
| walker |  | 8354 | 83 | export at packages/d2ts/src/operators/iterate.ts:80 |  |  | 0.612 |
| ns | 8442 |  | 358 | schema.ts — the Query interface family | 6.3 |  | 0.599 |
| walker |  | 8457 | 103 | export at packages/d2ts/src/operators/topK.ts:22 |  |  | 0.599 |
| walker |  | 8479 | 22 | export doc at packages/d2ts/src/operators/count.ts:10 |  |  | 0.599 |
| walker |  | 8501 | 22 | export doc at packages/d2ts/src/operators/distinct.ts:10 |  |  | 0.599 |
| walker |  | 8524 | 23 | export doc at packages/d2ts/src/operators/concat.ts:11 |  |  | 0.599 |
| ns | 8604 |  | 162 | schema.ts — the Comparator and LogicalOperator vocabularies | 6.4 | 6.3 | 0.591 |
| walker |  | 8636 | 112 | export at packages/d2ts/src/operators/topK.ts:60 |  |  | 0.591 |
| walker |  | 8689 | 53 | export names surface in packages/d2ts/src/sqlite/operators/orderBy.ts |  |  | 0.591 |
| ns | 8735 |  | 131 | compileQuery — the package's single entry point | 6.5 |  | 0.587 |
| walker |  | 8748 | 59 | export at packages/d2ts/src/sqlite/operators/orderBy.ts:26 |  |  | 0.587 |
| walker |  | 8795 | 47 | imports in packages/d2mini/src/index.ts |  |  | 0.587 |
| walker |  | 8815 | 20 | export doc at packages/d2ts/src/operators/distinct.ts:37 |  |  | 0.587 |
| walker |  | 8961 | 146 | export at packages/d2ts/src/operators/topKWithFractionalIndex.ts:29 |  |  | 0.587 |
| ns | 8978 |  | 243 | query-builder.ts — the fluent builder's complete method set | 6.6 |  | 0.578 |
| walker |  | 8985 | 24 | export doc at packages/d2ts/src/operators/consolidate.ts:16 |  |  | 0.578 |
| walker |  | 9009 | 24 | export doc at packages/d2ts/src/operators/debug.ts:18 |  |  | 0.578 |
| walker |  | 9033 | 24 | export doc at packages/d2ts/src/operators/filter.ts:11 |  |  | 0.578 |
| walker |  | 9057 | 24 | export doc at packages/d2ts/src/operators/output.ts:19 |  |  | 0.578 |
| ns | 9076 |  | 98 | d2mini README — how it differs from d2ts | 7.1 |  | 0.579 |
| ns | 9165 |  | 89 | d2mini/src/d2.ts — the versionless D2 constructor | 7.2 | 7.1 | 0.576 |
| walker |  | 9173 | 116 | export at packages/d2ts/src/operators/topKWithFractionalIndex.ts:372 |  |  | 0.576 |
| ns | 9401 |  | 236 | README: using the SQLite backend | 8.1 | 1.10 | 0.571 |
| walker |  | 9409 | 236 | export at packages/d2ts/src/version-index.ts:28 |  |  | 0.571 |
| walker |  | 9430 | 21 | export doc at packages/d2ts/src/operators/iterate.ts:225 |  |  | 0.571 |
| walker |  | 9455 | 25 | export doc at packages/d2ts/src/operators/iterate.ts:15 |  |  | 0.571 |
| walker |  | 9480 | 25 | export doc at packages/d2ts/src/operators/iterate.ts:46 |  |  | 0.571 |
| ns | 9529 |  | 128 | sqlite/database.ts — the SQLiteDb driver interface | 8.2 | 8.1 | 0.566 |
| walker |  | 9561 | 81 | export names surface in packages/d2ts/src/operators/keying.ts |  |  | 0.568 |
| walker |  | 9561 | 0 | export at packages/d2ts/src/operators/keying.ts:7 |  |  | 0.568 |
| walker |  | 9561 | 0 | export at packages/d2ts/src/operators/keying.ts:22 |  |  | 0.568 |
| walker |  | 9594 | 33 | export at packages/d2ts/src/operators/keying.ts:13 |  |  | 0.568 |
| walker |  | 9634 | 40 | export at packages/d2ts/src/operators/keying.ts:30 |  |  | 0.568 |
| walker |  | 9655 | 21 | export doc at packages/d2ts/src/operators/keying.ts:7 |  |  | 0.568 |
| ns | 9683 |  | 154 | withSQLite, SQLIndex and the Electric bridge exports | 8.3 | 8.2 | 0.564 |
| walker |  | 9804 | 149 | README.md section #11 |  |  | 0.564 |
| ns | 9827 |  | 144 | README: what each example demonstrates | 9.1 | 1.10 | 0.568 |
| walker |  | 9910 | 106 | export at packages/d2ts/src/sqlite/operators/count.ts:9 |  |  | 0.568 |
| ns | 9977 |  | 150 | examples/electric — what the demo app does | 9.2 | 2.13 | 0.565 |

Score(3000)=0.698 I=0.857 C=0.569 ns_rows≤3K=24/55 grid(1000/1442/2080/3000/4327/6240/9000)=0.656/0.648/0.757/0.698/0.676/0.625/0.566

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 54 | 54 | listing of '.' |  |  | 0.000 |
| walker |  | 65 | 11 | ts names eslint.base.mjs |  |  | 0.000 |
| walker |  | 68 | 3 | listing of '.github' |  |  | 0.000 |
| walker |  | 72 | 4 | listing of '.github/workflows' |  |  | 0.000 |
| ns | 81 |  | 81 | Repository identity — README title + one-line definition | 1.1 |  | 0.000 |
| walker |  | 94 | 22 | listing of 'packages' |  |  | 0.000 |
| walker |  | 129 | 35 | listing of 'examples' |  |  | 0.000 |
| ns | 135 |  | 54 | Complete repository root listing | 1.2 |  | 0.714 |
| walker |  | 174 | 45 | package identity in package.json |  |  | 0.716 |
| walker |  | 191 | 17 | listing of 'packages/d2ts-benchmark' |  |  | 0.717 |
| ns | 192 |  | 57 | Workspace membership — packages/ and examples/ listings | 1.3 |  | 0.697 |
| walker |  | 203 | 12 | listing of 'packages/d2ts-benchmark/src' |  |  | 0.697 |
| ns | 218 |  | 26 | pnpm workspace globs | 1.4 |  | 0.656 |
| walker |  | 241 | 38 | ts names packages/d2ts-benchmark/src/index.ts |  |  | 0.656 |
| walker |  | 249 | 8 | listing of '.changeset' |  |  | 0.656 |
| walker |  | 274 | 25 | listing of 'packages/d2ql' |  |  | 0.660 |
| ns | 288 |  | 70 | README lede — what incremental execution buys you | 1.5 | 1.1 | 0.648 |
| walker |  | 304 | 30 | listing of 'packages/d2mini' |  |  | 0.657 |
| walker |  | 385 | 81 | README headline in README.md |  |  | 0.936 |
| ns | 387 |  | 99 | README lede — ElectricSQL ShapeStreams and pipeline type inference | 1.6 | 1.1 | 0.887 |
| walker |  | 421 | 36 | listing of 'packages/d2ts' |  |  | 0.898 |
| walker |  | 506 | 85 | package identity in packages/d2ts/package.json |  |  | 0.898 |
| ns | 578 |  | 191 | Root package.json — identity and every workspace script | 1.7 |  | 0.736 |
| walker |  | 580 | 74 | package runtime dependencies in packages/d2ts/package.json |  |  | 0.736 |
| walker |  | 613 | 33 | listing of 'packages/d2mini/src' |  |  | 0.737 |
| walker |  | 660 | 47 | ts names packages/d2mini/src/index.ts |  |  | 0.737 |
| ns | 728 |  | 150 | Published package identity: d2ts and d2mini | 1.8 |  | 0.694 |
| walker |  | 764 | 104 | listing of 'packages/d2mini/src/operators' |  |  | 0.705 |
| walker |  | 820 | 56 | ts decl packages/d2ts-benchmark/src/index.ts:25 |  |  | 0.705 |
| walker |  | 864 | 44 | listing of 'packages/d2ts/src' |  |  | 0.712 |
| walker |  | 868 | 4 | listing of 'packages/d2ts/src/electric' |  |  | 0.712 |
| ns | 871 |  | 143 | Private package identity: d2ql and d2ts-benchmark | 1.9 | 1.8 | 0.653 |
| walker |  | 888 | 20 | listing of 'packages/d2ts/src/sqlite' |  |  | 0.653 |
| walker |  | 958 | 70 | ts names packages/d2ts/src/index.ts |  |  | 0.656 |
| ns | 1032 |  | 161 | Root README section map (every H2) | 1.10 |  | 0.578 |
| walker |  | 1056 | 98 | listing of 'packages/d2ts/src/operators' |  |  | 0.587 |
| walker |  | 1069 | 13 | ts names packages/d2mini/src/operators/filterBy.ts |  |  | 0.587 |
| walker |  | 1082 | 13 | ts names packages/d2ts/src/operators/filterBy.ts |  |  | 0.587 |
| walker |  | 1169 | 87 | ts decl packages/d2ts-benchmark/src/index.ts:496 |  |  | 0.587 |
| walker |  | 1228 | 59 | README headline in packages/d2mini/README.md |  |  | 0.587 |
| ns | 1284 |  | 252 | README "Implementation Details" — provenance and the four core data structures | 1.11 | 1.10 | 0.523 |
| walker |  | 1297 | 69 | listing of 'packages/d2ql/src' |  |  | 0.526 |
| walker |  | 1306 | 9 | listing of 'packages/d2ql/src/query-builder' |  |  | 0.527 |
| walker |  | 1344 | 38 | ts names packages/d2ql/src/index.ts |  |  | 0.527 |
| ns | 1392 |  | 108 | Every package's own root listing | 2.1 |  | 0.621 |
| walker |  | 1397 | 53 | ts module doc packages/d2ql/src/index.ts |  |  | 0.622 |
| walker |  | 1418 | 21 | ts names packages/d2ql/src/compiler.ts |  |  | 0.622 |
| ns | 1436 |  | 44 | packages/d2ts/src — complete module roster | 2.2 |  | 0.648 |
| walker |  | 1495 | 77 | README headline in packages/d2ql/README.md |  |  | 0.648 |
| ns | 1506 |  | 70 | packages/d2ts/src/index.ts — the public export barrel | 2.3 |  | 0.659 |
| walker |  | 1528 | 33 | packages/d2ql/README.md section #0 |  |  | 0.659 |
| ns | 1604 |  | 98 | packages/d2ts/src/operators — complete operator file roster | 2.4 |  | 0.691 |
| walker |  | 1606 | 78 | README headline in packages/d2ts/README.md |  |  | 0.692 |
| walker |  | 1630 | 24 | listing of 'examples/electric' |  |  | 0.692 |
| walker |  | 1638 | 8 | listing of 'examples/electric/src' |  |  | 0.693 |
| walker |  | 1677 | 39 | packages/d2mini/README.md section #0 |  |  | 0.693 |
| ns | 1844 |  | 240 | packages/d2ts/src/operators/index.ts — exported operator set | 2.5 | 2.4 | 0.648 |
| walker |  | 1850 | 173 | headings outline in README.md |  |  | 0.716 |
| ns | 1981 |  | 137 | packages/d2mini/src and its operators directory | 2.6 |  | 0.738 |
| walker |  | 2022 | 172 | README.md section #0 |  |  | 0.757 |
| walker |  | 2057 | 35 | ts decl packages/d2ql/src/compiler.ts:16 |  |  | 0.757 |
| walker |  | 2095 | 38 | ts names packages/d2ts/src/version-index.ts |  |  | 0.757 |
| walker |  | 2121 | 26 | plaintext config pnpm-workspace.yaml |  |  | 0.773 |
| walker |  | 2147 | 26 | ts names packages/d2mini/src/operators/topK.ts |  |  | 0.773 |
| walker |  | 2173 | 26 | ts names packages/d2ts/src/operators/topK.ts |  |  | 0.773 |
| walker |  | 2202 | 29 | listing of 'packages/d2mini/tests' |  |  | 0.773 |
| ns | 2246 |  | 265 | packages/d2mini export barrels | 2.7 | 2.6 | 0.732 |
| ns | 2324 |  | 78 | packages/d2ql/src and query-builder/ listings | 2.8 |  | 0.743 |
| walker |  | 2338 | 136 | package scripts in package.json |  |  | 0.788 |
| walker |  | 2370 | 32 | listing of 'examples/d2ql' |  |  | 0.790 |
| walker |  | 2401 | 31 | ts names packages/d2mini/src/operators/debug.ts |  |  | 0.790 |
| walker |  | 2428 | 27 | ts decl packages/d2mini/src/operators/debug.ts:12 |  |  | 0.790 |
| ns | 2441 |  | 117 | packages/d2ql export barrels | 2.9 | 2.8 | 0.786 |
| walker |  | 2459 | 31 | ts names packages/d2mini/src/operators/output.ts |  |  | 0.786 |
| walker |  | 2486 | 27 | ts decl packages/d2mini/src/operators/output.ts:13 |  |  | 0.786 |
| walker |  | 2517 | 31 | ts names packages/d2ts/src/operators/debug.ts |  |  | 0.786 |
| ns | 2518 |  | 77 | packages/d2ts/src/sqlite — complete tree listing | 2.10 |  | 0.761 |
| walker |  | 2544 | 27 | ts decl packages/d2ts/src/operators/debug.ts:18 |  |  | 0.761 |
| ns | 2701 |  | 183 | SQLite subpath export barrels | 2.11 | 2.10 | 0.738 |
| ns | 2886 |  | 185 | Test suite layout — immediate children of every tests/ directory | 2.12 |  | 0.695 |
| walker |  | 2927 | 383 | README.md section #2 |  |  | 0.697 |
| walker |  | 2957 | 30 | ts decl packages/d2mini/src/operators/output.ts:38 |  |  | 0.697 |
| ns | 2982 |  | 96 | Example and benchmark source trees | 2.13 | 1.3 | 0.698 |
| walker |  | 3011 | 54 | ts names packages/d2mini/src/d2.ts |  |  | 0.698 |
| walker |  | 3037 | 26 | ts decl packages/d2mini/src/d2.ts:146 |  |  | 0.698 |
| walker |  | 3070 | 33 | ts decl packages/d2mini/src/operators/debug.ts:42 |  |  | 0.698 |
| walker |  | 3103 | 33 | ts decl packages/d2ts/src/operators/debug.ts:74 |  |  | 0.698 |
| walker |  | 3158 | 55 | ts names packages/d2ts/src/order.ts |  |  | 0.698 |
| ns | 3172 |  | 190 | Per-package scripts: build, test, lint, typecheck, format | 3.1 | 1.7 | 0.688 |
| walker |  | 3175 | 17 | ts decl packages/d2ts/src/order.ts:277 |  |  | 0.688 |
| ns | 3259 |  | 87 | d2ts-benchmark scripts — the benchmark entry points | 3.2 | 3.1 | 0.682 |
| walker |  | 3350 | 175 | package entrypoints in packages/d2ts/package.json |  |  | 0.682 |
| walker |  | 3406 | 56 | ts names packages/d2mini/src/multiset.ts |  |  | 0.682 |
| ns | 3452 |  | 193 | CI pipeline stages | 3.3 |  | 0.659 |
| walker |  | 3462 | 56 | ts names packages/d2ts/src/multiset.ts |  |  | 0.659 |
| walker |  | 3498 | 36 | ts names packages/d2mini/src/operators/count.ts |  |  | 0.659 |
| walker |  | 3512 | 14 | ts decl packages/d2mini/src/operators/count.ts:9 |  |  | 0.659 |
| walker |  | 3548 | 36 | ts names packages/d2ts/src/operators/count.ts |  |  | 0.659 |
| walker |  | 3562 | 14 | ts decl packages/d2ts/src/operators/count.ts:10 |  |  | 0.659 |
| walker |  | 3599 | 37 | ts names packages/d2ts/src/operators/distinct.ts |  |  | 0.659 |
| walker |  | 3613 | 14 | ts decl packages/d2ts/src/operators/distinct.ts:10 |  |  | 0.659 |
| walker |  | 3654 | 41 | listing of 'packages/d2ts/tests' |  |  | 0.668 |
| ns | 3656 |  | 204 | README: constructing a D2 graph and sending to an input stream | 4.1 | 1.10 | 0.659 |
| walker |  | 3842 | 188 | package scripts in packages/d2ts/package.json |  |  | 0.675 |
| walker |  | 3882 | 40 | ts names packages/d2ts/src/operators/buffer.ts |  |  | 0.675 |
| walker |  | 3900 | 18 | ts decl packages/d2ts/src/operators/buffer.ts:17 |  |  | 0.675 |
| walker |  | 3957 | 57 | listing of 'packages/d2ts/src/sqlite/operators' |  |  | 0.703 |
| ns | 3981 |  | 325 | d2.ts — D2Options and the complete D2 class method roster | 4.2 |  | 0.679 |
| walker |  | 3998 | 41 | ts names packages/d2mini/src/operators/concat.ts |  |  | 0.679 |
| walker |  | 4014 | 16 | ts decl packages/d2mini/src/operators/concat.ts:9 |  |  | 0.679 |
| walker |  | 4043 | 29 | ts decl packages/d2mini/src/operators/concat.ts:25 |  |  | 0.679 |
| walker |  | 4084 | 41 | ts names packages/d2mini/src/operators/consolidate.ts |  |  | 0.679 |
| walker |  | 4100 | 16 | ts decl packages/d2mini/src/operators/consolidate.ts:9 |  |  | 0.679 |
| walker |  | 4141 | 41 | ts names packages/d2ts/src/operators/concat.ts |  |  | 0.680 |
| walker |  | 4157 | 16 | ts decl packages/d2ts/src/operators/concat.ts:11 |  |  | 0.680 |
| ns | 4171 |  | 190 | README: what versions and frontiers actually mean | 4.3 | 1.10 | 0.676 |
| walker |  | 4186 | 29 | ts decl packages/d2ts/src/operators/concat.ts:54 |  |  | 0.676 |
| walker |  | 4227 | 41 | ts names packages/d2ts/src/operators/consolidate.ts |  |  | 0.676 |
| walker |  | 4245 | 18 | ts decl packages/d2ts/src/operators/consolidate.ts:16 |  |  | 0.676 |
| walker |  | 4286 | 41 | ts names packages/d2ts/src/operators/topKWithFractionalIndex.ts |  |  | 0.676 |
| walker |  | 4353 | 67 | ts names packages/d2ts/src/d2.ts |  |  | 0.676 |
| walker |  | 4375 | 22 | ts decl packages/d2ts/src/d2.ts:11 |  |  | 0.677 |
| walker |  | 4413 | 38 | ts decl packages/d2ts/src/d2.ts:166 |  |  | 0.677 |
| walker |  | 4456 | 43 | ts names packages/d2ts/src/operators/reduce.ts |  |  | 0.677 |
| walker |  | 4483 | 27 | ts decl packages/d2ts/src/operators/reduce.ts:15 |  |  | 0.677 |
| ns | 4517 |  | 346 | order.ts — v() factory and the complete Version method roster | 4.4 |  | 0.653 |
| walker |  | 4527 | 44 | ts names packages/d2mini/src/operators/negate.ts |  |  | 0.653 |
| walker |  | 4551 | 24 | ts decl packages/d2mini/src/operators/negate.ts:10 |  |  | 0.653 |
| walker |  | 4595 | 44 | ts names packages/d2ts/src/operators/negate.ts |  |  | 0.653 |
| walker |  | 4619 | 24 | ts decl packages/d2ts/src/operators/negate.ts:10 |  |  | 0.653 |
| walker |  | 4665 | 46 | ts names packages/d2mini/src/operators/reduce.ts |  |  | 0.653 |
| walker |  | 4692 | 27 | ts decl packages/d2mini/src/operators/reduce.ts:14 |  |  | 0.653 |
| walker |  | 4730 | 38 | ts decl packages/d2ts/src/d2.ts:167 |  |  | 0.653 |
| walker |  | 4780 | 50 | ts names packages/d2ts/src/operators/output.ts |  |  | 0.654 |
| walker |  | 4807 | 27 | ts decl packages/d2ts/src/operators/output.ts:19 |  |  | 0.635 |
| ns | 4807 |  | 290 | order.ts — Antichain method roster and Frontier | 4.5 | 4.4 | 0.635 |
| walker |  | 4858 | 51 | ts names packages/d2mini/src/operators/distinct.ts |  |  | 0.635 |
| walker |  | 4885 | 27 | ts decl packages/d2mini/src/operators/distinct.ts:17 |  |  | 0.635 |
| walker |  | 4937 | 52 | ts names packages/d2mini/src/operators/filter.ts |  |  | 0.635 |
| ns | 4952 |  | 145 | README: MultiSet as a changeset, and keyed multisets | 4.6 | 1.10 | 0.632 |
| walker |  | 4972 | 35 | ts decl packages/d2mini/src/operators/filter.ts:10 |  |  | 0.632 |
| walker |  | 5024 | 52 | ts names packages/d2ts/src/operators/filter.ts |  |  | 0.632 |
| walker |  | 5059 | 35 | ts decl packages/d2ts/src/operators/filter.ts:11 |  |  | 0.632 |
| walker |  | 5083 | 24 | README.md section #7 |  |  | 0.632 |
| walker |  | 5133 | 50 | ts decl packages/d2ts/src/operators/topKWithFractionalIndex.ts:29 |  |  | 0.632 |
| walker |  | 5186 | 53 | ts names packages/d2ts/src/operators/orderBy.ts |  |  | 0.632 |
| walker |  | 5242 | 56 | ts names packages/d2mini/src/operators/map.ts |  |  | 0.632 |
| walker |  | 5277 | 35 | ts decl packages/d2mini/src/operators/map.ts:10 |  |  | 0.632 |
| ns | 5310 |  | 358 | multiset.ts — MultiSetArray, KeyedData and every MultiSet method | 4.7 |  | 0.617 |
| walker |  | 5333 | 56 | ts names packages/d2ts/src/operators/map.ts |  |  | 0.617 |
| walker |  | 5368 | 35 | ts decl packages/d2ts/src/operators/map.ts:11 |  |  | 0.617 |
| walker |  | 5483 | 115 | listing of 'packages/d2ql/tests' |  |  | 0.659 |
| ns | 5610 |  | 300 | types.ts — the operator message protocol | 4.8 |  | 0.637 |
| walker |  | 5841 | 358 | ts decl eslint.base.mjs:6 |  |  | 0.637 |
| ns | 5868 |  | 258 | types.ts — PipedOperator and the 20-deep pipe overload wall | 4.9 | 4.8 | 0.625 |
| walker |  | 5882 | 41 | listing of 'packages/d2ts/tests/operators-sqlite' |  |  | 0.625 |
| walker |  | 5895 | 13 | ts names packages/d2ql/src/select.ts |  |  | 0.625 |
| walker |  | 6170 | 275 | ts names packages/d2ql/src/schema.ts |  |  | 0.625 |
| walker |  | 6185 | 15 | ts decl packages/d2ql/src/schema.ts:48 |  |  | 0.625 |
| walker |  | 6201 | 16 | ts decl packages/d2ql/src/schema.ts:36 |  |  | 0.625 |
| walker |  | 6235 | 34 | ts decl packages/d2ql/src/schema.ts:152 |  |  | 0.625 |
| ns | 6251 |  | 383 | README Key Features — every operator with a one-line description | 5.1 | 1.10 | 0.634 |
| walker |  | 6279 | 44 | ts decl packages/d2ql/src/schema.ts:123 |  |  | 0.634 |
| walker |  | 6325 | 46 | ts decl packages/d2ql/src/schema.ts:65 |  |  | 0.634 |
| walker |  | 6373 | 48 | ts decl packages/d2ql/src/schema.ts:80 |  |  | 0.634 |
| walker |  | 6423 | 50 | ts decl packages/d2ql/src/schema.ts:24 |  |  | 0.634 |
| walker |  | 6477 | 54 | ts decl packages/d2ql/src/schema.ts:39 |  |  | 0.634 |
| walker |  | 6539 | 62 | ts decl packages/d2ql/src/schema.ts:71 |  |  | 0.634 |
| walker |  | 6603 | 64 | ts decl packages/d2ql/src/schema.ts:13 |  |  | 0.634 |
| walker |  | 6680 | 77 | ts decl packages/d2ql/src/schema.ts:143 |  |  | 0.634 |
| ns | 6681 |  | 430 | packages/d2ts/README.md — complete heading map (1081-line API reference) | 5.2 |  | 0.610 |
| walker |  | 6759 | 79 | ts decl packages/d2ql/src/schema.ts:94 |  |  | 0.610 |
| walker |  | 6841 | 82 | ts decl packages/d2ql/src/schema.ts:130 |  |  | 0.610 |
| ns | 6844 |  | 163 | Operator factory roster — single-stream operators | 5.3 | 2.5 | 0.614 |
| walker |  | 6899 | 58 | ts decl packages/d2mini/src/operators/count.ts:30 |  |  | 0.614 |
| walker |  | 6957 | 58 | ts decl packages/d2ts/src/operators/count.ts:32 |  |  | 0.614 |
| walker |  | 7015 | 58 | ts decl packages/d2ts/src/operators/distinct.ts:37 |  |  | 0.614 |
| walker |  | 7023 | 8 | ts body packages/d2ts/src/order.ts:278 |  |  | 0.614 |
| walker |  | 7082 | 59 | ts decl packages/d2ts/src/operators/orderBy.ts:23 |  |  | 0.614 |
| walker |  | 7175 | 93 | ts decl packages/d2ql/src/schema.ts:53 |  |  | 0.614 |
| walker |  | 7189 | 14 | ts names packages/d2ql/src/key-by.ts |  |  | 0.614 |
| walker |  | 7203 | 14 | ts names packages/d2ql/src/order-by.ts |  |  | 0.614 |
| ns | 7244 |  | 400 | Operator factory roster — keyed, join, ordering and aggregate families | 5.4 | 5.3 | 0.598 |
| walker |  | 7272 | 69 | ts names packages/d2ts/src/operators/iterate.ts |  |  | 0.600 |
| walker |  | 7288 | 16 | ts decl packages/d2ts/src/operators/iterate.ts:15 |  |  | 0.600 |
| walker |  | 7304 | 16 | ts decl packages/d2ts/src/operators/iterate.ts:46 |  |  | 0.600 |
| walker |  | 7331 | 27 | ts decl packages/d2ts/src/operators/iterate.ts:80 |  |  | 0.600 |
| walker |  | 7358 | 27 | ts decl packages/d2ts/src/operators/iterate.ts:225 |  |  | 0.600 |
| ns | 7446 |  | 202 | Operator implementation classes and the LinearUnaryOperator base | 5.5 | 5.4 | 0.605 |
| walker |  | 7469 | 111 | ts names packages/d2mini/src/types.ts |  |  | 0.605 |
| walker |  | 7490 | 21 | ts decl packages/d2mini/src/types.ts:83 |  |  | 0.605 |
| walker |  | 7513 | 23 | ts decl packages/d2mini/src/types.ts:6 |  |  | 0.605 |
| walker |  | 7538 | 25 | ts decl packages/d2mini/src/types.ts:11 |  |  | 0.605 |
| walker |  | 7576 | 38 | ts decl packages/d2mini/src/types.ts:16 |  |  | 0.605 |
| ns | 7622 |  | 176 | groupBy.ts — the AggregateFunction contract | 5.6 | 5.4 | 0.598 |
| walker |  | 7655 | 79 | ts decl packages/d2mini/src/types.ts:21 |  |  | 0.598 |
| walker |  | 7763 | 108 | ts decl packages/d2ql/src/schema.ts:105 |  |  | 0.598 |
| walker |  | 7779 | 16 | ts names packages/d2mini/src/indexes.ts |  |  | 0.598 |
| walker |  | 7833 | 54 | listing of 'packages/d2ql/tests/query-builder' |  |  | 0.598 |
| ns | 7842 |  | 220 | D2QL identity and README section map | 6.1 |  | 0.593 |
| walker |  | 7914 | 81 | ts names packages/d2mini/src/operators/keying.ts |  |  | 0.593 |
| walker |  | 7947 | 33 | ts decl packages/d2mini/src/operators/keying.ts:13 |  |  | 0.593 |
| walker |  | 7987 | 40 | ts decl packages/d2mini/src/operators/keying.ts:30 |  |  | 0.593 |
| walker |  | 8068 | 81 | ts names packages/d2ts/src/operators/keying.ts |  |  | 0.596 |
| ns | 8084 |  | 242 | D2QL "Current Features" — the supported SQL subset | 6.2 | 6.1 | 0.588 |
| walker |  | 8101 | 33 | ts decl packages/d2ts/src/operators/keying.ts:13 |  |  | 0.588 |
| walker |  | 8141 | 40 | ts decl packages/d2ts/src/operators/keying.ts:30 |  |  | 0.588 |
| walker |  | 8219 | 78 | ts decl packages/d2mini/src/operators/filterBy.ts:14 |  |  | 0.588 |
| walker |  | 8297 | 78 | ts decl packages/d2ts/src/operators/filterBy.ts:14 |  |  | 0.588 |
| walker |  | 8315 | 18 | ts names packages/d2ts-benchmark/src/graph.ts |  |  | 0.588 |
| walker |  | 8345 | 30 | packages/d2ql/README.md section #4 |  |  | 0.588 |
| walker |  | 8429 | 84 | ts names packages/d2mini/src/operators/orderBy.ts |  |  | 0.588 |
| ns | 8442 |  | 358 | schema.ts — the Query interface family | 6.3 |  | 0.576 |
| walker |  | 8471 | 42 | ts decl packages/d2mini/src/operators/orderBy.ts:9 |  |  | 0.576 |
| walker |  | 8530 | 59 | ts decl packages/d2mini/src/operators/orderBy.ts:23 |  |  | 0.576 |
| walker |  | 8541 | 11 | ts body packages/d2mini/src/d2.ts:147 |  |  | 0.576 |
| walker |  | 8571 | 30 | README.md section #17 |  |  | 0.576 |
| ns | 8604 |  | 162 | schema.ts — the Comparator and LogicalOperator vocabularies | 6.4 | 6.3 | 0.578 |
| walker |  | 8702 | 131 | ts decl packages/d2mini/src/d2.ts:10 |  |  | 0.578 |
| ns | 8735 |  | 131 | compileQuery — the package's single entry point | 6.5 |  | 0.575 |
| walker |  | 8787 | 85 | ts decl packages/d2mini/src/operators/reduce.ts:106 |  |  | 0.575 |
| walker |  | 8872 | 85 | ts decl packages/d2ts/src/operators/reduce.ts:127 |  |  | 0.575 |
| walker |  | 8902 | 30 | README.md section #3 |  |  | 0.575 |
| walker |  | 8933 | 31 | README.md section #9 |  |  | 0.575 |
| ns | 8978 |  | 243 | query-builder.ts — the fluent builder's complete method set | 6.6 |  | 0.566 |
| walker |  | 9020 | 87 | ts decl packages/d2mini/src/operators/orderBy.ts:76 |  |  | 0.566 |
| ns | 9076 |  | 98 | d2mini README — how it differs from d2ts | 7.1 |  | 0.567 |
| walker |  | 9107 | 87 | ts decl packages/d2mini/src/operators/orderBy.ts:199 |  |  | 0.567 |
| ns | 9165 |  | 89 | d2mini/src/d2.ts — the versionless D2 constructor | 7.2 | 7.1 | 0.564 |
| walker |  | 9194 | 87 | ts decl packages/d2ts/src/operators/orderBy.ts:76 |  |  | 0.564 |
| walker |  | 9281 | 87 | ts decl packages/d2ts/src/operators/orderBy.ts:140 |  |  | 0.564 |
| ns | 9401 |  | 236 | README: using the SQLite backend | 8.1 | 1.10 | 0.559 |
| walker |  | 9420 | 139 | listing of 'packages/d2mini/tests/operators' |  |  | 0.559 |
| walker |  | 9469 | 49 | ts decl packages/d2mini/src/operators/count.ts:10 |  |  | 0.559 |
| walker |  | 9482 | 13 | ts body packages/d2ts/src/d2.ts:167 |  |  | 0.559 |
| ns | 9529 |  | 128 | sqlite/database.ts — the SQLiteDb driver interface | 8.2 | 8.1 | 0.554 |
| ns | 9683 |  | 154 | withSQLite, SQLIndex and the Electric bridge exports | 8.3 | 8.2 | 0.551 |
| ns | 9827 |  | 144 | README: what each example demonstrates | 9.1 | 1.10 | 0.548 |
| walker |  | 9832 | 350 | ts decl eslint.base.mjs:6 #1 |  |  | 0.548 |
| ns | 9977 |  | 150 | examples/electric — what the demo app does | 9.2 | 2.13 | 0.545 |

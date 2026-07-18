Score(3000)=0.493 I=0.787 C=0.310 ns_rows≤3K=21/42 (reached=8 partial=3 missing=10)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 39 | 39 | listing of '.' |  |  | 1.000 |
| ns | 39 |  | 39 | Repo root listing | 1.1 |  | 1.000 |
| walker |  | 45 | 6 | listing of 'test-d' |  |  | 1.000 |
| ns | 62 |  | 23 | source/ listing | 1.2 |  | 0.811 |
| ns | 110 |  | 48 | test/, test-d/, .github/ listings | 1.3 |  | 0.602 |
| walker |  | 113 | 68 | package identity in package.json |  |  | 0.612 |
| ns | 137 |  | 27 | readme.md title + tagline | 2.1 |  | 0.575 |
| walker |  | 180 | 67 | README headline in readme.md |  |  | 0.648 |
| walker |  | 203 | 23 | listing of 'source' |  |  | 0.776 |
| walker |  | 234 | 31 | package runtime metadata in package.json |  |  | 0.777 |
| walker |  | 248 | 14 | export at source/options.ts:111 |  |  | 0.777 |
| ns | 256 |  | 119 | readme.md — maintenance-mode notice | 2.2 | 2.1 | 0.716 |
| walker |  | 261 | 13 | export at source/options.ts:97 |  |  | 0.716 |
| ns | 315 |  | 59 | package.json — name, version, description, license | 2.3 |  | 0.721 |
| walker |  | 324 | 63 | export names surface in source/index.ts |  |  | 0.721 |
| walker |  | 357 | 33 | export at source/index.ts:16 |  |  | 0.721 |
| walker |  | 364 | 7 | listing of '.github' |  |  | 0.734 |
| walker |  | 368 | 4 | listing of '.github/workflows' |  |  | 0.745 |
| ns | 448 |  | 133 | package.json — repository, funding, type, exports, sideEffects, engines | 2.4 | 2.3 | 0.656 |
| ns | 554 |  | 106 | package.json — scripts | 2.5 | 2.4 | 0.615 |
| walker |  | 569 | 201 | YAML config at .github/workflows/main.yml |  |  | 0.615 |
| walker |  | 589 | 20 | export doc at source/index.ts:16 |  |  | 0.615 |
| walker |  | 619 | 30 | export at source/options.ts:27 |  |  | 0.615 |
| ns | 652 |  | 98 | readme.md — H2 section map | 2.6 | 2.1 | 0.559 |
| walker |  | 656 | 37 | export names surface in source/priority-queue.ts |  |  | 0.559 |
| walker |  | 673 | 17 | export at source/priority-queue.ts:7 |  |  | 0.560 |
| walker |  | 749 | 76 | benchmark names surface in bench.ts |  |  | 0.564 |
| ns | 855 |  | 203 | readme.md — Usage example | 2.7 | 2.6 | 0.484 |
| ns | 931 |  | 76 | bench.ts — benchmark-name roster | 2.8 |  | 0.509 |
| walker |  | 967 | 218 | export tail #1 at source/index.ts:16 |  |  | 0.509 |
| ns | 1080 |  | 149 | source/queue.ts (Queue interface + RunFunction) | 3.1 |  | 0.477 |
| walker |  | 1088 | 121 | headings outline in readme.md |  |  | 0.556 |
| walker |  | 1217 | 129 | export at source/priority-queue.ts:11 |  |  | 0.556 |
| walker |  | 1227 | 10 | readme.md section #5 |  |  | 0.556 |
| ns | 1352 |  | 272 | source/options.ts — TimeoutOptions | 3.2 |  | 0.490 |
| walker |  | 1516 | 289 | export tail #2 at source/index.ts:16 |  |  | 0.493 |
| ns | 1587 |  | 235 | source/options.ts — Options (concurrency, autoStart, queueClass) | 3.3 | 3.2 | 0.453 |
| walker |  | 1608 | 92 | package entrypoints in package.json |  |  | 0.483 |
| walker |  | 1639 | 31 | listing of 'test' |  |  | 0.555 |
| ns | 1732 |  | 145 | source/options.ts — Options (intervalCap, interval) | 3.4 | 3.3 | 0.521 |
| walker |  | 1743 | 104 | package scripts in package.json |  |  | 0.554 |
| walker |  | 1758 | 15 | readme.md section #24 |  |  | 0.554 |
| walker |  | 1776 | 18 | imports in source/options.ts |  |  | 0.554 |
| ns | 1847 |  | 115 | source/options.ts — Options (carryoverIntervalCount) | 3.5 | 3.4 | 0.532 |
| walker |  | 1860 | 84 | imports in source/index.ts |  |  | 0.533 |
| walker |  | 1980 | 120 | export member docs #1 at source/options.ts:97 |  |  | 0.536 |
| walker |  | 2061 | 81 | json config tsconfig.json |  |  | 0.536 |
| walker |  | 2108 | 47 | readme.md section #41 |  |  | 0.536 |
| ns | 2165 |  | 318 | source/options.ts — Options (strict) | 3.6 | 3.5 | 0.508 |
| ns | 2308 |  | 143 | source/options.ts — QueueAddOptions | 3.7 | 3.3 | 0.531 |
| ns | 2397 |  | 89 | source/options.ts — TaskOptions (signal field doc) | 3.8 | 3.2 | 0.526 |
| walker |  | 2492 | 384 | readme.md section #0 |  |  | 0.545 |
| walker |  | 2517 | 25 | readme.md section #9 |  |  | 0.545 |
| walker |  | 2542 | 25 | readme.md section #25 |  |  | 0.545 |
| walker |  | 2596 | 54 | package runtime dependencies in package.json |  |  | 0.545 |
| ns | 2715 |  | 318 | source/options.ts — TaskOptions (AbortSignal cancellation example) | 3.9 | 3.8 | 0.499 |
| walker |  | 2725 | 129 | readme.md section #1 |  |  | 0.499 |
| walker |  | 2755 | 30 | readme.md section #22 |  |  | 0.499 |
| walker |  | 2802 | 47 | imports in source/priority-queue.ts |  |  | 0.499 |
| walker |  | 2835 | 33 | readme.md section #19 |  |  | 0.499 |
| walker |  | 2869 | 34 | readme.md section #3 |  |  | 0.499 |
| ns | 2932 |  | 217 | source/priority-queue.ts — type + class fields | 3.10 |  | 0.493 |
| walker |  | 3108 | 239 | package identity metadata in package.json |  |  | 0.516 |
| walker |  | 3148 | 40 | export names surface in source/lower-bound.ts |  |  | 0.516 |
| walker |  | 3148 | 0 | export at source/lower-bound.ts:3 |  |  | 0.516 |
| walker |  | 3197 | 49 | readme.md section #8 |  |  | 0.516 |
| ns | 3314 |  | 382 | source/priority-queue.ts — enqueue (binary insertion) | 3.11 | 3.10 | 0.483 |
| walker |  | 3477 | 280 | export member docs #1 at source/options.ts:27 |  |  | 0.556 |
| ns | 3717 |  | 403 | source/priority-queue.ts — setPriority + remove | 3.12 | 3.11 | 0.526 |
| walker |  | 3871 | 394 | export member docs #1 at source/options.ts:111 |  |  | 0.616 |
| ns | 3951 |  | 234 | source/priority-queue.ts — dequeue | 3.13 | 3.10 | 0.596 |
| walker |  | 4074 | 203 | readme.md section #2 |  |  | 0.650 |
| walker |  | 4138 | 64 | readme.md section #26 |  |  | 0.650 |
| walker |  | 4211 | 73 | readme.md section #10 |  |  | 0.650 |
| ns | 4225 |  | 274 | source/priority-queue.ts — filter/size/#compact | 3.14 | 3.13 | 0.624 |
| walker |  | 4246 | 35 | export names surface in source/queue.ts |  |  | 0.626 |
| walker |  | 4413 | 167 | readme.md section #42 |  |  | 0.626 |
| ns | 4445 |  | 220 | source/index.ts — imports, Task/EventName types, class doc | 4.1 |  | 0.619 |
| ns | 4749 |  | 304 | source/index.ts — PQueue class fields (interval/strict/rate-limit state) | 4.2 |  | 0.596 |
| ns | 5002 |  | 253 | source/index.ts — PQueue class fields (queue/concurrency/task-tracking state) | 4.3 | 4.2 | 0.578 |
| ns | 5289 |  | 287 | source/index.ts — public member roster (locations) | 4.4 |  | 0.597 |
| walker |  | 5524 | 1111 | export body at source/priority-queue.ts:11 body 18 |  |  | 0.746 |
| ns | 5575 |  | 286 | source/index.ts — private member roster (locations) | 4.5 |  | 0.727 |
| walker |  | 5679 | 155 | export body at source/lower-bound.ts:3 body 4 |  |  | 0.727 |
| walker |  | 5738 | 59 | imports in bench.ts |  |  | 0.727 |
| ns | 5742 |  | 167 | source/index.ts — constructor (defaults merge) | 4.6 | 4.4 | 0.714 |
| walker |  | 5996 | 258 | package dev/peer dependencies in package.json |  |  | 0.714 |
| walker |  | 6263 | 267 | readme.md section #35 |  |  | 0.714 |
| ns | 6314 |  | 572 | source/index.ts — constructor (validation + field assignment) | 4.7 | 4.6 | 0.686 |
| ns | 6456 |  | 142 | source/index.ts — admission-control getters | 4.8 | 4.5 | 0.675 |
| ns | 6959 |  | 503 | test/basic.ts — test-name roster (part 1 of 2) | 5.1 |  | 0.655 |
| ns | 7491 |  | 532 | test/basic.ts — test-name roster (part 2 of 2) | 5.2 | 5.1 | 0.637 |
| ns | 8122 |  | 631 | test/advanced.ts — test-name roster (part 1 of 2) | 5.3 |  | 0.617 |
| ns | 8772 |  | 650 | test/advanced.ts — test-name roster (part 2 of 2) | 5.4 | 5.3 | 0.600 |
| ns | 9271 |  | 499 | test/strict.ts — test-name roster | 5.5 |  | 0.585 |
| ns | 9451 |  | 180 | test/priority-queue.ts — test-name roster | 5.6 |  | 0.581 |
| ns | 9578 |  | 127 | test/validation.ts — test-name roster | 5.7 |  | 0.578 |
| ns | 9750 |  | 172 | test/rate-limit.ts — test-name roster | 5.8 |  | 0.574 |
| ns | 9937 |  | 187 | test/debug.ts — test-name roster | 5.9 |  | 0.569 |

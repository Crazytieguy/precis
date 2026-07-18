Score(3000)=0.460 I=0.788 C=0.268 ns_rows≤3K=21/42 (reached=7 partial=2 missing=12)

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
| ns | 256 |  | 119 | readme.md — maintenance-mode notice | 2.2 | 2.1 | 0.716 |
| walker |  | 297 | 63 | export names surface in source/index.ts |  |  | 0.716 |
| ns | 315 |  | 59 | package.json — name, version, description, license | 2.3 |  | 0.721 |
| walker |  | 330 | 33 | export at source/index.ts:16 |  |  | 0.721 |
| walker |  | 337 | 7 | listing of '.github' |  |  | 0.734 |
| walker |  | 341 | 4 | listing of '.github/workflows' |  |  | 0.745 |
| ns | 448 |  | 133 | package.json — repository, funding, type, exports, sideEffects, engines | 2.4 | 2.3 | 0.656 |
| walker |  | 542 | 201 | YAML config at .github/workflows/main.yml |  |  | 0.656 |
| ns | 554 |  | 106 | package.json — scripts | 2.5 | 2.4 | 0.615 |
| walker |  | 562 | 20 | export doc at source/index.ts:16 |  |  | 0.615 |
| walker |  | 599 | 37 | export names surface in source/priority-queue.ts |  |  | 0.615 |
| walker |  | 616 | 17 | export at source/priority-queue.ts:7 |  |  | 0.615 |
| ns | 652 |  | 98 | readme.md — H2 section map | 2.6 | 2.1 | 0.559 |
| walker |  | 834 | 218 | export tail #1 at source/index.ts:16 |  |  | 0.560 |
| ns | 855 |  | 203 | readme.md — Usage example | 2.7 | 2.6 | 0.480 |
| ns | 931 |  | 76 | bench.ts — benchmark-name roster | 2.8 |  | 0.465 |
| walker |  | 955 | 121 | headings outline in readme.md |  |  | 0.554 |
| ns | 1080 |  | 149 | source/queue.ts (Queue interface + RunFunction) | 3.1 |  | 0.519 |
| walker |  | 1084 | 129 | export at source/priority-queue.ts:11 |  |  | 0.519 |
| walker |  | 1094 | 10 | readme.md section #5 |  |  | 0.519 |
| ns | 1352 |  | 272 | source/options.ts — TimeoutOptions | 3.2 |  | 0.457 |
| walker |  | 1383 | 289 | export tail #2 at source/index.ts:16 |  |  | 0.460 |
| walker |  | 1475 | 92 | package entrypoints in package.json |  |  | 0.494 |
| walker |  | 1506 | 31 | listing of 'test' |  |  | 0.574 |
| ns | 1587 |  | 235 | source/options.ts — Options (concurrency, autoStart, queueClass) | 3.3 | 3.2 | 0.525 |
| walker |  | 1610 | 104 | package scripts in package.json |  |  | 0.561 |
| walker |  | 1625 | 15 | readme.md section #24 |  |  | 0.561 |
| walker |  | 1645 | 20 | imports in source/options.ts |  |  | 0.562 |
| walker |  | 1729 | 84 | imports in source/index.ts |  |  | 0.563 |
| ns | 1732 |  | 145 | source/options.ts — Options (intervalCap, interval) | 3.4 | 3.3 | 0.528 |
| walker |  | 1810 | 81 | json config tsconfig.json |  |  | 0.528 |
| ns | 1847 |  | 115 | source/options.ts — Options (carryoverIntervalCount) | 3.5 | 3.4 | 0.507 |
| walker |  | 1857 | 47 | readme.md section #41 |  |  | 0.507 |
| ns | 2165 |  | 318 | source/options.ts — Options (strict) | 3.6 | 3.5 | 0.481 |
| walker |  | 2241 | 384 | readme.md section #0 |  |  | 0.502 |
| walker |  | 2266 | 25 | readme.md section #9 |  |  | 0.502 |
| walker |  | 2291 | 25 | readme.md section #25 |  |  | 0.502 |
| ns | 2308 |  | 143 | source/options.ts — QueueAddOptions | 3.7 | 3.3 | 0.483 |
| walker |  | 2345 | 54 | package runtime dependencies in package.json |  |  | 0.483 |
| ns | 2397 |  | 89 | source/options.ts — TaskOptions (signal field doc) | 3.8 | 3.2 | 0.479 |
| walker |  | 2474 | 129 | readme.md section #1 |  |  | 0.479 |
| walker |  | 2504 | 30 | readme.md section #22 |  |  | 0.479 |
| walker |  | 2551 | 47 | imports in source/priority-queue.ts |  |  | 0.480 |
| walker |  | 2584 | 33 | readme.md section #19 |  |  | 0.480 |
| walker |  | 2618 | 34 | readme.md section #3 |  |  | 0.480 |
| ns | 2715 |  | 318 | source/options.ts — TaskOptions (AbortSignal cancellation example) | 3.9 | 3.8 | 0.439 |
| walker |  | 2857 | 239 | package identity metadata in package.json |  |  | 0.464 |
| walker |  | 2897 | 40 | export names surface in source/lower-bound.ts |  |  | 0.464 |
| walker |  | 2897 | 0 | export at source/lower-bound.ts:3 |  |  | 0.464 |
| ns | 2932 |  | 217 | source/priority-queue.ts — type + class fields | 3.10 |  | 0.460 |
| walker |  | 2946 | 49 | readme.md section #8 |  |  | 0.460 |
| walker |  | 3149 | 203 | readme.md section #2 |  |  | 0.534 |
| walker |  | 3213 | 64 | readme.md section #26 |  |  | 0.534 |
| walker |  | 3286 | 73 | readme.md section #10 |  |  | 0.534 |
| ns | 3314 |  | 382 | source/priority-queue.ts — enqueue (binary insertion) | 3.11 | 3.10 | 0.500 |
| walker |  | 3321 | 35 | export names surface in source/queue.ts |  |  | 0.502 |
| walker |  | 3488 | 167 | readme.md section #42 |  |  | 0.502 |
| ns | 3717 |  | 403 | source/priority-queue.ts — setPriority + remove | 3.12 | 3.11 | 0.476 |
| ns | 3951 |  | 234 | source/priority-queue.ts — dequeue | 3.13 | 3.10 | 0.461 |
| ns | 4225 |  | 274 | source/priority-queue.ts — filter/size/#compact | 3.14 | 3.13 | 0.442 |
| ns | 4445 |  | 220 | source/index.ts — imports, Task/EventName types, class doc | 4.1 |  | 0.442 |
| walker |  | 4599 | 1111 | export body at source/priority-queue.ts:11 body 18 |  |  | 0.641 |
| ns | 4749 |  | 304 | source/index.ts — PQueue class fields (interval/strict/rate-limit state) | 4.2 |  | 0.617 |
| walker |  | 4754 | 155 | export body at source/lower-bound.ts:3 body 4 |  |  | 0.617 |
| walker |  | 4815 | 61 | imports in bench.ts |  |  | 0.617 |
| ns | 5002 |  | 253 | source/index.ts — PQueue class fields (queue/concurrency/task-tracking state) | 4.3 | 4.2 | 0.598 |
| walker |  | 5073 | 258 | package dev/peer dependencies in package.json |  |  | 0.598 |
| walker |  | 5128 | 55 | export names surface in source/options.ts |  |  | 0.599 |
| ns | 5289 |  | 287 | source/index.ts — public member roster (locations) | 4.4 |  | 0.616 |
| walker |  | 5395 | 267 | readme.md section #35 |  |  | 0.616 |
| ns | 5575 |  | 286 | source/index.ts — private member roster (locations) | 4.5 |  | 0.600 |
| ns | 5742 |  | 167 | source/index.ts — constructor (defaults merge) | 4.6 | 4.4 | 0.590 |
| ns | 6314 |  | 572 | source/index.ts — constructor (validation + field assignment) | 4.7 | 4.6 | 0.567 |
| ns | 6456 |  | 142 | source/index.ts — admission-control getters | 4.8 | 4.5 | 0.557 |
| ns | 6959 |  | 503 | test/basic.ts — test-name roster (part 1 of 2) | 5.1 |  | 0.541 |
| ns | 7491 |  | 532 | test/basic.ts — test-name roster (part 2 of 2) | 5.2 | 5.1 | 0.526 |
| ns | 8122 |  | 631 | test/advanced.ts — test-name roster (part 1 of 2) | 5.3 |  | 0.510 |
| ns | 8772 |  | 650 | test/advanced.ts — test-name roster (part 2 of 2) | 5.4 | 5.3 | 0.495 |
| ns | 9271 |  | 499 | test/strict.ts — test-name roster | 5.5 |  | 0.483 |
| ns | 9451 |  | 180 | test/priority-queue.ts — test-name roster | 5.6 |  | 0.480 |
| ns | 9578 |  | 127 | test/validation.ts — test-name roster | 5.7 |  | 0.477 |
| ns | 9750 |  | 172 | test/rate-limit.ts — test-name roster | 5.8 |  | 0.474 |
| ns | 9937 |  | 187 | test/debug.ts — test-name roster | 5.9 |  | 0.470 |

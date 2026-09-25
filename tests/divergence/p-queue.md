Score(3000)=0.630 I=0.871 C=0.456 ns_rows≤3K=21/56 grid(1000/1442/2080/3000/4327/6240/9000)=0.661/0.663/0.639/0.630/0.638/0.562/0.521

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 39 | 39 | listing of '.' |  |  | 0.000 |
| walker |  | 45 | 6 | listing of 'test-d' |  |  | 0.000 |
| ns | 67 |  | 67 | Readme title, tagline and what p-queue is useful for | 1.1 |  | 0.000 |
| walker |  | 68 | 23 | listing of 'source' |  |  | 0.000 |
| walker |  | 136 | 68 | package identity in package.json |  |  | 0.000 |
| walker |  | 143 | 7 | listing of '.github' |  |  | 0.000 |
| ns | 146 |  | 79 | Readme scope note: not for server job queues, and the project is feature-complete | 1.2 |  | 0.000 |
| walker |  | 147 | 4 | listing of '.github/workflows' |  |  | 0.000 |
| ns | 185 |  | 39 | Complete root directory listing | 1.3 |  | 0.426 |
| walker |  | 214 | 67 | README headline in readme.md |  |  | 0.888 |
| ns | 245 |  | 60 | Complete listings of source/, test/ and test-d/ | 1.4 |  | 0.705 |
| walker |  | 268 | 54 | package runtime dependencies in package.json |  |  | 0.705 |
| walker |  | 299 | 31 | package runtime metadata in package.json |  |  | 0.708 |
| walker |  | 334 | 35 | ts names source/queue.ts |  |  | 0.709 |
| walker |  | 371 | 37 | ts names source/priority-queue.ts |  |  | 0.709 |
| ns | 380 |  | 135 | PQueue class declaration, its two type parameters, and the complete event-name union | 1.5 |  | 0.651 |
| walker |  | 388 | 17 | ts decl source/priority-queue.ts:7 |  |  | 0.651 |
| ns | 496 |  | 116 | Every section heading in readme.md (all H2s plus the two API H3s) | 1.6 |  | 0.553 |
| ns | 564 |  | 68 | The package's complete public export surface (end of source/index.ts) | 1.7 |  | 0.528 |
| walker |  | 586 | 198 | ts names source/index.ts |  |  | 0.608 |
| walker |  | 620 | 34 | ts decl source/index.ts:7 |  |  | 0.609 |
| ns | 713 |  | 149 | source/queue.ts in full: the pluggable Queue contract | 1.8 |  | 0.552 |
| walker |  | 796 | 176 | ts decl source/index.ts:16 |  |  | 0.552 |
| walker |  | 814 | 18 | ts doc source/index.ts:16 |  |  | 0.591 |
| walker |  | 871 | 57 | ts names source/options.ts |  |  | 0.591 |
| walker |  | 902 | 31 | listing of 'test' |  |  | 0.661 |
| ns | 902 |  | 189 | package.json identity block: name, version, description, module type, exports, engines | 1.9 |  | 0.661 |
| walker |  | 1023 | 121 | headings outline in readme.md |  |  | 0.770 |
| ns | 1036 |  | 134 | source/index.ts imports and the Task type | 1.10 |  | 0.729 |
| walker |  | 1115 | 92 | package entrypoints in package.json |  |  | 0.773 |
| ns | 1275 |  | 239 | Name-only roster of every public member of PQueue (24 declarations, complete) | 2.1 |  | 0.672 |
| walker |  | 1347 | 232 | ts decl source/index.ts:16 #1 |  |  | 0.716 |
| walker |  | 1387 | 40 | ts decl source/index.ts:581 |  |  | 0.717 |
| walker |  | 1429 | 42 | ts decl source/index.ts:585 |  |  | 0.717 |
| ns | 1436 |  | 161 | Readme Usage: the canonical concurrency-1 example | 2.2 |  | 0.663 |
| walker |  | 1440 | 11 | ts body source/index.ts:382 |  |  | 0.663 |
| walker |  | 1544 | 104 | package scripts in package.json |  |  | 0.664 |
| ns | 1620 |  | 184 | Full signatures and doc comments for .add() and .addAll() | 2.3 | 2.1 | 0.642 |
| walker |  | 1658 | 114 | ts decl source/queue.ts:3 |  |  | 0.698 |
| walker |  | 1670 | 12 | ts body source/index.ts:609 |  |  | 0.698 |
| ns | 1792 |  | 172 | Readme .add() semantics and its two warnings | 2.4 |  | 0.674 |
| walker |  | 1800 | 130 | ts decl source/options.ts:97 |  |  | 0.675 |
| ns | 1897 |  | 105 | Doc comments for the lifecycle controls: .start(), .pause(), .clear() | 2.5 | 2.1 | 0.647 |
| walker |  | 1934 | 134 | ts decl source/priority-queue.ts:11 |  |  | 0.648 |
| ns | 2002 |  | 105 | Readme warning: .clear() leaves queued .add() promises unsettled | 2.6 |  | 0.639 |
| walker |  | 2093 | 159 | ts decl source/index.ts:16 #2 |  |  | 0.713 |
| walker |  | 2145 | 52 | ts decl source/index.ts:952 |  |  | 0.713 |
| walker |  | 2160 | 15 | ts body source/priority-queue.ts:115 |  |  | 0.713 |
| walker |  | 2170 | 10 | ts body source/index.ts:802 |  |  | 0.713 |
| walker |  | 2217 | 47 | readme.md section #27 |  |  | 0.713 |
| ns | 2249 |  | 247 | Doc comments distinguishing .onEmpty(), .onIdle() and .onPendingZero() | 2.7 | 2.1 | 0.673 |
| ns | 2456 |  | 207 | Doc comments for .onSizeLessThan(), .onRateLimit() and .onRateLimitCleared() | 2.8 | 2.1 | 0.643 |
| ns | 2579 |  | 123 | .onError() contract, with its example elided | 2.9 | 2.1 | 0.629 |
| walker |  | 2601 | 384 | readme.md section #0 |  |  | 0.667 |
| walker |  | 2612 | 11 | ts body source/index.ts:785 |  |  | 0.667 |
| walker |  | 2652 | 40 | ts names source/lower-bound.ts |  |  | 0.667 |
| ns | 2743 |  | 164 | Doc comments for the .size, .sizeBy(), .pending and .isPaused introspection members | 2.10 | 2.1 | 0.639 |
| walker |  | 2891 | 239 | package identity metadata in package.json |  |  | 0.659 |
| ns | 2972 |  | 229 | .isRateLimited, .isSaturated and .runningTasks, including the runningTasks element shape | 2.11 | 2.1 | 0.630 |
| walker |  | 3020 | 129 | readme.md section #1 |  |  | 0.630 |
| walker |  | 3031 | 11 | ts body source/index.ts:809 |  |  | 0.630 |
| ns | 3044 |  | 72 | .setPriority(id, priority) signature and contract | 2.12 | 2.1 | 0.622 |
| ns | 3291 |  | 247 | Every field of every option type in source/options.ts (complete) | 3.1 |  | 0.602 |
| walker |  | 3428 | 397 | ts decl source/options.ts:111 |  |  | 0.606 |
| walker |  | 3441 | 13 | ts body source/index.ts:888 |  |  | 0.606 |
| walker |  | 3522 | 81 | json config tsconfig.json |  |  | 0.607 |
| ns | 3604 |  | 313 | One-line description, minimum and @default for each constructor option | 3.2 | 3.1 | 0.585 |
| walker |  | 3725 | 203 | readme.md section #2 |  |  | 0.631 |
| ns | 3868 |  | 264 | The strict option explained: sliding window vs fixed window | 3.3 | 3.1 | 0.621 |
| walker |  | 4014 | 289 | ts decl source/options.ts:27 |  |  | 0.644 |
| ns | 4015 |  | 147 | Per-task option docs: priority, id, and the AbortSignal contract | 3.4 | 3.1 | 0.648 |
| walker |  | 4035 | 21 | ts doc source/index.ts:616 |  |  | 0.651 |
| walker |  | 4056 | 21 | ts body source/index.ts:585 |  |  | 0.651 |
| ns | 4155 |  | 140 | Constructor default-options literal | 3.5 |  | 0.638 |
| walker |  | 4281 | 225 | readme.md section #13 |  |  | 0.638 |
| ns | 4496 |  | 341 | Every constructor validation rule and its error message | 3.6 |  | 0.618 |
| walker |  | 4607 | 326 | readme.md section #14 |  |  | 0.618 |
| ns | 4629 |  | 133 | The concurrency getter/setter, including runtime mutation and its validation | 3.7 | 2.1 | 0.606 |
| walker |  | 4774 | 167 | readme.md section #28 |  |  | 0.606 |
| ns | 4781 |  | 152 | Every FAQ question in readme.md (complete, eight questions) | 3.8 |  | 0.598 |
| ns | 5035 |  | 254 | Name-only roster of all 21 private methods and private getters of PQueue | 4.1 |  | 0.579 |
| walker |  | 5273 | 499 | ts decl source/options.ts:27 #1 |  |  | 0.623 |
| ns | 5332 |  | 297 | Every private field of PQueue with its type: the complete instance state | 4.2 |  | 0.603 |
| walker |  | 5380 | 107 | ts body source/priority-queue.ts:102 |  |  | 0.603 |
| walker |  | 5403 | 23 | ts doc source/index.ts:609 |  |  | 0.607 |
| ns | 5589 |  | 257 | #tryToStartAnother: the admission decision and task dispatch | 4.3 | 4.1 | 0.584 |
| walker |  | 5603 | 200 | readme.md section #21 |  |  | 0.585 |
| ns | 5749 |  | 160 | add(): option normalization and automatic id assignment | 4.4 | 2.3 | 0.574 |
| walker |  | 5822 | 219 | readme.md section #22 |  |  | 0.576 |
| walker |  | 5846 | 24 | ts doc source/index.ts:809 |  |  | 0.579 |
| ns | 5900 |  | 151 | add(): the run() prologue — pending accounting and runningTasks tracking | 4.5 | 4.4 | 0.568 |
| walker |  | 6113 | 267 | readme.md section #20 |  |  | 0.570 |
| walker |  | 6141 | 28 | ts doc source/index.ts:802 |  |  | 0.574 |
| ns | 6158 |  | 258 | add(): invoking the task, wrapping it in p-timeout, and racing the abort signal | 4.6 | 4.5 | 0.559 |
| walker |  | 6171 | 30 | ts doc source/index.ts:888 |  |  | 0.562 |
| walker |  | 6315 | 144 | ts body source/priority-queue.ts:50 |  |  | 0.562 |
| walker |  | 6346 | 31 | ts doc source/index.ts:785 |  |  | 0.568 |
| ns | 6456 |  | 298 | add(): settlement, completed/error events, and the finally block that defers #next | 4.7 | 4.1 | 0.548 |
| ns | 6741 |  | 285 | add(): enqueueing and the queued-task abort path | 4.8 | 4.4 | 0.532 |
| walker |  | 6832 | 486 | readme.md section #3 |  |  | 0.532 |
| ns | 6963 |  | 222 | #isIntervalPausedAt: the strict sliding-window branch | 4.9 | 4.1 | 0.522 |
| walker |  | 7307 | 475 | readme.md section #4 |  |  | 0.522 |
| ns | 7319 |  | 356 | #isIntervalPausedAt: the default fixed-window branch | 4.10 | 4.9 | 0.508 |
| ns | 7536 |  | 217 | PriorityQueue: header, options type, class declaration and the head-cursor invariant | 5.1 |  | 0.502 |
| ns | 7608 |  | 72 | Name-only roster of every PriorityQueue member (complete) | 5.2 |  | 0.507 |
| walker |  | 7756 | 449 | readme.md section #5 |  |  | 0.521 |
| ns | 7940 |  | 332 | PriorityQueue.enqueue: the priority insertion algorithm | 5.3 | 5.2 | 0.509 |
| walker |  | 8255 | 499 | readme.md section #6 |  |  | 0.509 |
| ns | 8283 |  | 343 | PriorityQueue.dequeue, size and #compact: the consumed-prefix machinery | 5.4 | 5.2 | 0.495 |
| ns | 8363 |  | 80 | source/lower-bound.ts: provenance and signature | 5.5 |  | 0.494 |
| ns | 8600 |  | 237 | Readme: the Custom QueueClass section with a complete worked implementation | 6.1 |  | 0.516 |
| walker |  | 8662 | 407 | readme.md section #7 |  |  | 0.516 |
| ns | 8751 |  | 151 | Readme FAQ: how to cancel or remove a queued task | 6.2 | 3.8 | 0.513 |
| ns | 8934 |  | 183 | Readme FAQ: backpressure, and how concurrency relates to intervalCap | 6.3 | 3.8 | 0.521 |
| ns | 9040 |  | 106 | package.json scripts: how to build, test and benchmark | 7.1 |  | 0.525 |
| walker |  | 9157 | 495 | readme.md section #8 |  |  | 0.525 |
| ns | 9216 |  | 176 | Test titles in test/debug.ts (all 11) | 7.2 |  | 0.520 |
| ns | 9396 |  | 180 | Test titles in test/priority-queue.ts (all 8) | 7.3 |  | 0.517 |
| walker |  | 9620 | 463 | readme.md section #9 |  |  | 0.517 |
| ns | 9680 |  | 284 | Test titles in test/rate-limit.ts (all 9) and test/validation.ts (all 7) | 7.4 |  | 0.511 |
| ns | 9691 |  | 11 | CI workflow and the remaining .github files | 7.5 |  | 0.513 |
| ns | 9836 |  | 145 | The CI job definition itself | 7.6 | 7.5 | 0.506 |
| ns | 9907 |  | 71 | bench.ts: the five benchmark cases | 7.7 |  | 0.505 |
| ns | 9988 |  | 81 | tsconfig.json: the whole build configuration | 7.8 |  | 0.510 |

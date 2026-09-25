Score(3000)=0.633 I=0.872 C=0.460 ns_rows≤3K=21/56 grid(1000/1442/2080/3000/4327/6240/9000)=0.822/0.685/0.716/0.633/0.656/0.574/0.529

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 39 | 39 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 45 | 6 | Fs::DirListing { dir: test-d } |  |  | 0.000 |
| ns | 67 |  | 67 | Readme title, tagline and what p-queue is useful for | 1.1 |  | 0.000 |
| walker |  | 68 | 23 | Fs::DirListing { dir: source } |  |  | 0.000 |
| walker |  | 136 | 68 | Json::Identity { file: package.json } |  |  | 0.000 |
| walker |  | 143 | 7 | Fs::DirListing { dir: .github } |  |  | 0.000 |
| ns | 146 |  | 79 | Readme scope note: not for server job queues, and the project is feature-complete | 1.2 |  | 0.000 |
| walker |  | 147 | 4 | Fs::DirListing { dir: .github/workflows } |  |  | 0.000 |
| ns | 185 |  | 39 | Complete root directory listing | 1.3 |  | 0.426 |
| walker |  | 214 | 67 | Markdown::ReadmeHeadline { file: readme.md } |  |  | 0.888 |
| ns | 245 |  | 60 | Complete listings of source/, test/ and test-d/ | 1.4 |  | 0.705 |
| walker |  | 268 | 54 | Json::Dependencies { file: package.json } |  |  | 0.705 |
| walker |  | 299 | 31 | Json::Runtime { file: package.json } |  |  | 0.708 |
| walker |  | 330 | 31 | Fs::DirListing { dir: test } |  |  | 0.910 |
| ns | 380 |  | 135 | PQueue class declaration, its two type parameters, and the complete event-name union | 1.5 |  | 0.836 |
| walker |  | 451 | 121 | Markdown::HeadingsOutline { file: readme.md } |  |  | 0.865 |
| ns | 496 |  | 116 | Every section heading in readme.md (all H2s plus the two API H3s) | 1.6 |  | 0.864 |
| walker |  | 543 | 92 | Json::Entry { file: package.json } |  |  | 0.873 |
| ns | 564 |  | 68 | The package's complete public export surface (end of source/index.ts) | 1.7 |  | 0.834 |
| walker |  | 647 | 104 | Json::Scripts { file: package.json } |  |  | 0.834 |
| walker |  | 682 | 35 | Code::CodeKey { rung: Names, file: source/queue.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.836 |
| ns | 713 |  | 149 | source/queue.ts in full: the pluggable Queue contract | 1.8 |  | 0.754 |
| walker |  | 796 | 114 | Code::CodeKey { rung: Decl, file: source/queue.ts, decl: 2, sub: 0, line: 3 } |  |  | 0.856 |
| walker |  | 833 | 37 | Code::CodeKey { rung: Names, file: source/priority-queue.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.856 |
| walker |  | 850 | 17 | Code::CodeKey { rung: Decl, file: source/priority-queue.ts, decl: 1, sub: 0, line: 7 } |  |  | 0.856 |
| ns | 902 |  | 189 | package.json identity block: name, version, description, module type, exports, engines | 1.9 |  | 0.821 |
| walker |  | 984 | 134 | Code::CodeKey { rung: Decl, file: source/priority-queue.ts, decl: 2, sub: 0, line: 11 } |  |  | 0.822 |
| ns | 1036 |  | 134 | source/index.ts imports and the Task type | 1.10 |  | 0.772 |
| walker |  | 1182 | 198 | Code::CodeKey { rung: Names, file: source/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.818 |
| walker |  | 1216 | 34 | Code::CodeKey { rung: Decl, file: source/index.ts, decl: 1, sub: 0, line: 7 } |  |  | 0.825 |
| ns | 1275 |  | 239 | Name-only roster of every public member of PQueue (24 declarations, complete) | 2.1 |  | 0.716 |
| walker |  | 1392 | 176 | Code::CodeKey { rung: Decl, file: source/index.ts, decl: 3, sub: 0, line: 16 } |  |  | 0.717 |
| walker |  | 1410 | 18 | Code::CodeKey { rung: Doc, file: source/index.ts, decl: 3, sub: 0, line: 16 } |  |  | 0.740 |
| ns | 1436 |  | 161 | Readme Usage: the canonical concurrency-1 example | 2.2 |  | 0.685 |
| ns | 1620 |  | 184 | Full signatures and doc comments for .add() and .addAll() | 2.3 | 2.1 | 0.644 |
| walker |  | 1642 | 232 | Code::CodeKey { rung: Decl, file: source/index.ts, decl: 3, sub: 1, line: 16 } |  |  | 0.687 |
| walker |  | 1682 | 40 | Code::CodeKey { rung: Decl, file: source/index.ts, decl: 10, sub: 0, line: 581 } |  |  | 0.699 |
| walker |  | 1724 | 42 | Code::CodeKey { rung: Decl, file: source/index.ts, decl: 11, sub: 0, line: 585 } |  |  | 0.699 |
| walker |  | 1735 | 11 | Code::CodeKey { rung: Body, file: source/index.ts, decl: 5, sub: 0, line: 382 } |  |  | 0.699 |
| walker |  | 1747 | 12 | Code::CodeKey { rung: Body, file: source/index.ts, decl: 13, sub: 0, line: 609 } |  |  | 0.699 |
| walker |  | 1768 | 21 | Code::CodeKey { rung: Doc, file: source/index.ts, decl: 14, sub: 0, line: 616 } |  |  | 0.700 |
| walker |  | 1789 | 21 | Code::CodeKey { rung: Body, file: source/index.ts, decl: 11, sub: 0, line: 585 } |  |  | 0.700 |
| ns | 1792 |  | 172 | Readme .add() semantics and its two warnings | 2.4 |  | 0.675 |
| ns | 1897 |  | 105 | Doc comments for the lifecycle controls: .start(), .pause(), .clear() | 2.5 | 2.1 | 0.651 |
| walker |  | 1948 | 159 | Code::CodeKey { rung: Decl, file: source/index.ts, decl: 3, sub: 2, line: 16 } |  |  | 0.726 |
| walker |  | 2000 | 52 | Code::CodeKey { rung: Decl, file: source/index.ts, decl: 28, sub: 0, line: 952 } |  |  | 0.726 |
| ns | 2002 |  | 105 | Readme warning: .clear() leaves queued .add() promises unsettled | 2.6 |  | 0.716 |
| walker |  | 2010 | 10 | Code::CodeKey { rung: Body, file: source/index.ts, decl: 24, sub: 0, line: 802 } |  |  | 0.716 |
| walker |  | 2021 | 11 | Code::CodeKey { rung: Body, file: source/index.ts, decl: 22, sub: 0, line: 785 } |  |  | 0.716 |
| walker |  | 2036 | 15 | Code::CodeKey { rung: Body, file: source/priority-queue.ts, decl: 10, sub: 0, line: 115 } |  |  | 0.716 |
| walker |  | 2093 | 57 | Code::CodeKey { rung: Names, file: source/options.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.716 |
| walker |  | 2223 | 130 | Code::CodeKey { rung: Decl, file: source/options.ts, decl: 2, sub: 0, line: 97 } |  |  | 0.717 |
| walker |  | 2234 | 11 | Code::CodeKey { rung: Body, file: source/index.ts, decl: 25, sub: 0, line: 809 } |  |  | 0.717 |
| ns | 2249 |  | 247 | Doc comments distinguishing .onEmpty(), .onIdle() and .onPendingZero() | 2.7 | 2.1 | 0.677 |
| walker |  | 2281 | 47 | Markdown::Section { file: readme.md, section_index: 27, keeps_default_concavity: false } |  |  | 0.677 |
| ns | 2456 |  | 207 | Doc comments for .onSizeLessThan(), .onRateLimit() and .onRateLimitCleared() | 2.8 | 2.1 | 0.647 |
| ns | 2579 |  | 123 | .onError() contract, with its example elided | 2.9 | 2.1 | 0.633 |
| walker |  | 2665 | 384 | Markdown::Section { file: readme.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.670 |
| walker |  | 2678 | 13 | Code::CodeKey { rung: Body, file: source/index.ts, decl: 26, sub: 0, line: 888 } |  |  | 0.670 |
| ns | 2743 |  | 164 | Doc comments for the .size, .sizeBy(), .pending and .isPaused introspection members | 2.10 | 2.1 | 0.642 |
| walker |  | 2917 | 239 | Json::IdentityMeta { file: package.json } |  |  | 0.663 |
| ns | 2972 |  | 229 | .isRateLimited, .isSaturated and .runningTasks, including the runningTasks element shape | 2.11 | 2.1 | 0.633 |
| ns | 3044 |  | 72 | .setPriority(id, priority) signature and contract | 2.12 | 2.1 | 0.625 |
| walker |  | 3046 | 129 | Markdown::Section { file: readme.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.625 |
| ns | 3291 |  | 247 | Every field of every option type in source/options.ts (complete) | 3.1 |  | 0.605 |
| walker |  | 3443 | 397 | Code::CodeKey { rung: Decl, file: source/options.ts, decl: 3, sub: 0, line: 111 } |  |  | 0.609 |
| walker |  | 3466 | 23 | Code::CodeKey { rung: Doc, file: source/index.ts, decl: 13, sub: 0, line: 609 } |  |  | 0.614 |
| walker |  | 3490 | 24 | Code::CodeKey { rung: Doc, file: source/index.ts, decl: 25, sub: 0, line: 809 } |  |  | 0.619 |
| walker |  | 3571 | 81 | Json::Whole { file: tsconfig.json } |  |  | 0.619 |
| ns | 3604 |  | 313 | One-line description, minimum and @default for each constructor option | 3.2 | 3.1 | 0.597 |
| walker |  | 3774 | 203 | Markdown::Section { file: readme.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.642 |
| ns | 3868 |  | 264 | The strict option explained: sliding window vs fixed window | 3.3 | 3.1 | 0.633 |
| ns | 4015 |  | 147 | Per-task option docs: priority, id, and the AbortSignal contract | 3.4 | 3.1 | 0.637 |
| walker |  | 4063 | 289 | Code::CodeKey { rung: Decl, file: source/options.ts, decl: 1, sub: 0, line: 27 } |  |  | 0.659 |
| walker |  | 4091 | 28 | Code::CodeKey { rung: Doc, file: source/index.ts, decl: 24, sub: 0, line: 802 } |  |  | 0.665 |
| walker |  | 4131 | 40 | Code::CodeKey { rung: Names, file: source/lower-bound.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.665 |
| ns | 4155 |  | 140 | Constructor default-options literal | 3.5 |  | 0.651 |
| walker |  | 4161 | 30 | Code::CodeKey { rung: Doc, file: source/index.ts, decl: 26, sub: 0, line: 888 } |  |  | 0.656 |
| walker |  | 4386 | 225 | Markdown::Section { file: readme.md, section_index: 13, keeps_default_concavity: false } |  |  | 0.656 |
| ns | 4496 |  | 341 | Every constructor validation rule and its error message | 3.6 |  | 0.636 |
| ns | 4629 |  | 133 | The concurrency getter/setter, including runtime mutation and its validation | 3.7 | 2.1 | 0.624 |
| walker |  | 4712 | 326 | Markdown::Section { file: readme.md, section_index: 14, keeps_default_concavity: true } |  |  | 0.624 |
| walker |  | 4743 | 31 | Code::CodeKey { rung: Doc, file: source/index.ts, decl: 22, sub: 0, line: 785 } |  |  | 0.631 |
| ns | 4781 |  | 152 | Every FAQ question in readme.md (complete, eight questions) | 3.8 |  | 0.623 |
| walker |  | 4910 | 167 | Markdown::Section { file: readme.md, section_index: 28, keeps_default_concavity: false } |  |  | 0.623 |
| ns | 5035 |  | 254 | Name-only roster of all 21 private methods and private getters of PQueue | 4.1 |  | 0.603 |
| ns | 5332 |  | 297 | Every private field of PQueue with its type: the complete instance state | 4.2 |  | 0.583 |
| walker |  | 5409 | 499 | Code::CodeKey { rung: Decl, file: source/options.ts, decl: 1, sub: 1, line: 27 } |  |  | 0.625 |
| walker |  | 5516 | 107 | Code::CodeKey { rung: Body, file: source/priority-queue.ts, decl: 9, sub: 0, line: 102 } |  |  | 0.625 |
| ns | 5589 |  | 257 | #tryToStartAnother: the admission decision and task dispatch | 4.3 | 4.1 | 0.602 |
| walker |  | 5716 | 200 | Markdown::Section { file: readme.md, section_index: 21, keeps_default_concavity: false } |  |  | 0.603 |
| ns | 5749 |  | 160 | add(): option normalization and automatic id assignment | 4.4 | 2.3 | 0.592 |
| ns | 5900 |  | 151 | add(): the run() prologue — pending accounting and runningTasks tracking | 4.5 | 4.4 | 0.580 |
| walker |  | 5935 | 219 | Markdown::Section { file: readme.md, section_index: 22, keeps_default_concavity: true } |  |  | 0.582 |
| walker |  | 5967 | 32 | Code::CodeKey { rung: Doc, file: source/index.ts, decl: 8, sub: 0, line: 443 } |  |  | 0.588 |
| walker |  | 6000 | 33 | Code::CodeKey { rung: Doc, file: source/index.ts, decl: 20, sub: 0, line: 718 } |  |  | 0.590 |
| walker |  | 6032 | 32 | Code::CodeKey { rung: Body, file: source/index.ts, decl: 23, sub: 0, line: 794 } |  |  | 0.590 |
| ns | 6158 |  | 258 | add(): invoking the task, wrapping it in p-timeout, and racing the abort signal | 4.6 | 4.5 | 0.574 |
| walker |  | 6299 | 267 | Markdown::Section { file: readme.md, section_index: 20, keeps_default_concavity: false } |  |  | 0.576 |
| walker |  | 6334 | 35 | Code::CodeKey { rung: Doc, file: source/index.ts, decl: 19, sub: 0, line: 707 } |  |  | 0.580 |
| ns | 6456 |  | 298 | add(): settlement, completed/error events, and the finally block that defers #next | 4.7 | 4.1 | 0.559 |
| walker |  | 6478 | 144 | Code::CodeKey { rung: Body, file: source/priority-queue.ts, decl: 4, sub: 0, line: 50 } |  |  | 0.559 |
| walker |  | 6515 | 37 | Code::CodeKey { rung: Body, file: source/index.ts, decl: 28, sub: 0, line: 952 } |  |  | 0.559 |
| ns | 6741 |  | 285 | add(): enqueueing and the queued-task abort path | 4.8 | 4.4 | 0.543 |
| ns | 6963 |  | 222 | #isIntervalPausedAt: the strict sliding-window branch | 4.9 | 4.1 | 0.533 |
| walker |  | 7001 | 486 | Markdown::Section { file: readme.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.533 |
| ns | 7319 |  | 356 | #isIntervalPausedAt: the default fixed-window branch | 4.10 | 4.9 | 0.518 |
| walker |  | 7476 | 475 | Markdown::Section { file: readme.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.518 |
| ns | 7536 |  | 217 | PriorityQueue: header, options type, class declaration and the head-cursor invariant | 5.1 |  | 0.512 |
| ns | 7608 |  | 72 | Name-only roster of every PriorityQueue member (complete) | 5.2 |  | 0.517 |
| walker |  | 7925 | 449 | Markdown::Section { file: readme.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.531 |
| ns | 7940 |  | 332 | PriorityQueue.enqueue: the priority insertion algorithm | 5.3 | 5.2 | 0.518 |
| ns | 8283 |  | 343 | PriorityQueue.dequeue, size and #compact: the consumed-prefix machinery | 5.4 | 5.2 | 0.504 |
| ns | 8363 |  | 80 | source/lower-bound.ts: provenance and signature | 5.5 |  | 0.503 |
| walker |  | 8424 | 499 | Markdown::Section { file: readme.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.503 |
| ns | 8600 |  | 237 | Readme: the Custom QueueClass section with a complete worked implementation | 6.1 |  | 0.524 |
| ns | 8751 |  | 151 | Readme FAQ: how to cancel or remove a queued task | 6.2 | 3.8 | 0.522 |
| walker |  | 8831 | 407 | Markdown::Section { file: readme.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.522 |
| ns | 8934 |  | 183 | Readme FAQ: backpressure, and how concurrency relates to intervalCap | 6.3 | 3.8 | 0.529 |
| ns | 9040 |  | 106 | package.json scripts: how to build, test and benchmark | 7.1 |  | 0.533 |
| ns | 9216 |  | 176 | Test titles in test/debug.ts (all 11) | 7.2 |  | 0.529 |
| walker |  | 9326 | 495 | Markdown::Section { file: readme.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.529 |
| ns | 9396 |  | 180 | Test titles in test/priority-queue.ts (all 8) | 7.3 |  | 0.525 |
| ns | 9680 |  | 284 | Test titles in test/rate-limit.ts (all 9) and test/validation.ts (all 7) | 7.4 |  | 0.519 |
| ns | 9691 |  | 11 | CI workflow and the remaining .github files | 7.5 |  | 0.521 |
| walker |  | 9789 | 463 | Markdown::Section { file: readme.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.521 |
| ns | 9836 |  | 145 | The CI job definition itself | 7.6 | 7.5 | 0.514 |
| ns | 9907 |  | 71 | bench.ts: the five benchmark cases | 7.7 |  | 0.513 |
| ns | 9988 |  | 81 | tsconfig.json: the whole build configuration | 7.8 |  | 0.518 |

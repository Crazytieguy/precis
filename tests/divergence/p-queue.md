Score(3000)=0.806 I=0.910 C=0.714 ns_rows≤3K=21/56 grid(1000/1442/2080/3000/4327/6240/9000)=0.790/0.626/0.696/0.806/0.759/0.671/0.606

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 39 | 39 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 45 | 6 | Fs::DirListing { dir: test-d } |  |  | 0.000 |
| ns | 67 |  | 67 | Readme title, tagline and what p-queue is useful for | 1.1 |  | 0.000 |
| walker |  | 112 | 67 | Markdown::ReadmeHeadline { file: readme.md } |  |  | 1.000 |
| walker |  | 135 | 23 | Fs::DirListing { dir: source } |  |  | 1.000 |
| ns | 146 |  | 79 | Readme scope note: not for server job queues, and the project is feature-complete | 1.2 |  | 0.791 |
| ns | 185 |  | 39 | Complete root directory listing | 1.3 |  | 0.884 |
| walker |  | 203 | 68 | Json::Identity { file: package.json } |  |  | 0.887 |
| walker |  | 210 | 7 | Fs::DirListing { dir: .github } |  |  | 0.887 |
| walker |  | 214 | 4 | Fs::DirListing { dir: .github/workflows } |  |  | 0.888 |
| walker |  | 245 | 31 | Json::Runtime { file: package.json } |  |  | 0.708 |
| ns | 245 |  | 60 | Complete listings of source/, test/ and test-d/ | 1.4 |  | 0.708 |
| walker |  | 299 | 54 | Json::Dependencies { file: package.json } |  |  | 0.708 |
| walker |  | 330 | 31 | Fs::DirListing { dir: test } |  |  | 0.910 |
| ns | 380 |  | 135 | PQueue class declaration, its two type parameters, and the complete event-name union | 1.5 |  | 0.836 |
| walker |  | 451 | 121 | Markdown::HeadingsOutline { file: readme.md } |  |  | 0.865 |
| ns | 496 |  | 116 | Every section heading in readme.md (all H2s plus the two API H3s) | 1.6 |  | 0.864 |
| walker |  | 543 | 92 | Json::Entry { file: package.json } |  |  | 0.873 |
| ns | 564 |  | 68 | The package's complete public export surface (end of source/index.ts) | 1.7 |  | 0.834 |
| walker |  | 647 | 104 | Json::Scripts { file: package.json } |  |  | 0.834 |
| ns | 713 |  | 149 | source/queue.ts in full: the pluggable Queue contract | 1.8 |  | 0.747 |
| ns | 902 |  | 189 | package.json identity block: name, version, description, module type, exports, engines | 1.9 |  | 0.731 |
| walker |  | 1031 | 384 | Markdown::Section { file: readme.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.790 |
| ns | 1036 |  | 134 | source/index.ts imports and the Task type | 1.10 |  | 0.742 |
| walker |  | 1160 | 129 | Markdown::Section { file: readme.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.742 |
| ns | 1275 |  | 239 | Name-only roster of every public member of PQueue (24 declarations, complete) | 2.1 |  | 0.644 |
| walker |  | 1291 | 131 | Code::CodeKey { rung: Names, file: source/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.677 |
| ns | 1436 |  | 161 | Readme Usage: the canonical concurrency-1 example | 2.2 |  | 0.626 |
| walker |  | 1467 | 176 | Code::CodeKey { rung: Decl, file: source/index.ts, decl: 1, sub: 0, line: 16 } |  |  | 0.627 |
| ns | 1620 |  | 184 | Full signatures and doc comments for .add() and .addAll() | 2.3 | 2.1 | 0.590 |
| walker |  | 1652 | 185 | Code::CodeKey { rung: Decl, file: source/index.ts, decl: 1, sub: 1, line: 16 } |  |  | 0.620 |
| walker |  | 1692 | 40 | Code::CodeKey { rung: Decl, file: source/index.ts, decl: 8, sub: 0, line: 581 } |  |  | 0.633 |
| walker |  | 1734 | 42 | Code::CodeKey { rung: Decl, file: source/index.ts, decl: 9, sub: 0, line: 585 } |  |  | 0.633 |
| ns | 1792 |  | 172 | Readme .add() semantics and its two warnings | 2.4 |  | 0.611 |
| ns | 1897 |  | 105 | Doc comments for the lifecycle controls: .start(), .pause(), .clear() | 2.5 | 2.1 | 0.586 |
| walker |  | 1940 | 206 | Code::CodeKey { rung: Decl, file: source/index.ts, decl: 1, sub: 2, line: 16 } |  |  | 0.680 |
| walker |  | 1992 | 52 | Code::CodeKey { rung: Decl, file: source/index.ts, decl: 26, sub: 0, line: 952 } |  |  | 0.681 |
| ns | 2002 |  | 105 | Readme warning: .clear() leaves queued .add() promises unsettled | 2.6 |  | 0.671 |
| walker |  | 2012 | 20 | Code::CodeKey { rung: Doc, file: source/index.ts, decl: 1, sub: 0, line: 16 } |  |  | 0.683 |
| walker |  | 2033 | 21 | Code::CodeKey { rung: Doc, file: source/index.ts, decl: 12, sub: 0, line: 616 } |  |  | 0.687 |
| walker |  | 2056 | 23 | Code::CodeKey { rung: Doc, file: source/index.ts, decl: 11, sub: 0, line: 609 } |  |  | 0.695 |
| walker |  | 2080 | 24 | Code::CodeKey { rung: Doc, file: source/index.ts, decl: 23, sub: 0, line: 809 } |  |  | 0.696 |
| walker |  | 2108 | 28 | Code::CodeKey { rung: Doc, file: source/index.ts, decl: 22, sub: 0, line: 802 } |  |  | 0.696 |
| walker |  | 2138 | 30 | Code::CodeKey { rung: Doc, file: source/index.ts, decl: 24, sub: 0, line: 888 } |  |  | 0.697 |
| walker |  | 2169 | 31 | Code::CodeKey { rung: Doc, file: source/index.ts, decl: 20, sub: 0, line: 785 } |  |  | 0.698 |
| walker |  | 2201 | 32 | Code::CodeKey { rung: Doc, file: source/index.ts, decl: 6, sub: 0, line: 443 } |  |  | 0.711 |
| walker |  | 2234 | 33 | Code::CodeKey { rung: Doc, file: source/index.ts, decl: 18, sub: 0, line: 718 } |  |  | 0.711 |
| ns | 2249 |  | 247 | Doc comments distinguishing .onEmpty(), .onIdle() and .onPendingZero() | 2.7 | 2.1 | 0.672 |
| walker |  | 2269 | 35 | Code::CodeKey { rung: Doc, file: source/index.ts, decl: 17, sub: 0, line: 707 } |  |  | 0.672 |
| walker |  | 2324 | 55 | Code::CodeKey { rung: Doc, file: source/index.ts, decl: 8, sub: 0, line: 581 } |  |  | 0.700 |
| walker |  | 2379 | 55 | Code::CodeKey { rung: Doc, file: source/index.ts, decl: 10, sub: 0, line: 595 } |  |  | 0.732 |
| walker |  | 2437 | 58 | Code::CodeKey { rung: Doc, file: source/index.ts, decl: 13, sub: 0, line: 652 } |  |  | 0.740 |
| ns | 2456 |  | 207 | Doc comments for .onSizeLessThan(), .onRateLimit() and .onRateLimitCleared() | 2.8 | 2.1 | 0.717 |
| walker |  | 2498 | 61 | Code::CodeKey { rung: Doc, file: source/index.ts, decl: 21, sub: 0, line: 794 } |  |  | 0.720 |
| walker |  | 2572 | 74 | Code::CodeKey { rung: Doc, file: source/index.ts, decl: 16, sub: 0, line: 696 } |  |  | 0.737 |
| ns | 2579 |  | 123 | .onError() contract, with its example elided | 2.9 | 2.1 | 0.721 |
| walker |  | 2675 | 103 | Code::CodeKey { rung: Doc, file: source/index.ts, decl: 15, sub: 0, line: 682 } |  |  | 0.755 |
| ns | 2743 |  | 164 | Doc comments for the .size, .sizeBy(), .pending and .isPaused introspection members | 2.10 | 2.1 | 0.766 |
| walker |  | 2878 | 203 | Markdown::Section { file: readme.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.815 |
| walker |  | 2913 | 35 | Code::CodeKey { rung: Names, file: source/queue.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.818 |
| ns | 2972 |  | 229 | .isRateLimited, .isSaturated and .runningTasks, including the runningTasks element shape | 2.11 | 2.1 | 0.784 |
| walker |  | 3027 | 114 | Code::CodeKey { rung: Decl, file: source/queue.ts, decl: 2, sub: 0, line: 3 } |  |  | 0.819 |
| ns | 3044 |  | 72 | .setPriority(id, priority) signature and contract | 2.12 | 2.1 | 0.808 |
| walker |  | 3151 | 124 | Code::CodeKey { rung: Doc, file: source/index.ts, decl: 14, sub: 0, line: 668 } |  |  | 0.839 |
| walker |  | 3188 | 37 | Code::CodeKey { rung: Names, file: source/priority-queue.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.839 |
| walker |  | 3205 | 17 | Code::CodeKey { rung: Decl, file: source/priority-queue.ts, decl: 1, sub: 0, line: 7 } |  |  | 0.840 |
| ns | 3291 |  | 247 | Every field of every option type in source/options.ts (complete) | 3.1 |  | 0.804 |
| walker |  | 3339 | 134 | Code::CodeKey { rung: Decl, file: source/priority-queue.ts, decl: 2, sub: 0, line: 11 } |  |  | 0.805 |
| walker |  | 3396 | 57 | Code::CodeKey { rung: Names, file: source/options.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.807 |
| walker |  | 3526 | 130 | Code::CodeKey { rung: Decl, file: source/options.ts, decl: 2, sub: 0, line: 97 } |  |  | 0.810 |
| ns | 3604 |  | 313 | One-line description, minimum and @default for each constructor option | 3.2 | 3.1 | 0.782 |
| ns | 3868 |  | 264 | The strict option explained: sliding window vs fixed window | 3.3 | 3.1 | 0.770 |
| walker |  | 3923 | 397 | Code::CodeKey { rung: Decl, file: source/options.ts, decl: 3, sub: 0, line: 111 } |  |  | 0.773 |
| walker |  | 3938 | 15 | Code::CodeKey { rung: Body, file: source/priority-queue.ts, decl: 10, sub: 0, line: 115 } |  |  | 0.773 |
| walker |  | 3948 | 10 | Code::CodeKey { rung: Body, file: source/index.ts, decl: 22, sub: 0, line: 802 } |  |  | 0.773 |
| ns | 4015 |  | 147 | Per-task option docs: priority, id, and the AbortSignal contract | 3.4 | 3.1 | 0.775 |
| ns | 4155 |  | 140 | Constructor default-options literal | 3.5 |  | 0.759 |
| walker |  | 4173 | 225 | Markdown::Section { file: readme.md, section_index: 13, keeps_default_concavity: false } |  |  | 0.759 |
| walker |  | 4184 | 11 | Code::CodeKey { rung: Body, file: source/index.ts, decl: 3, sub: 0, line: 382 } |  |  | 0.759 |
| walker |  | 4351 | 167 | Markdown::Section { file: readme.md, section_index: 27, keeps_default_concavity: false } |  |  | 0.759 |
| walker |  | 4362 | 11 | Code::CodeKey { rung: Body, file: source/index.ts, decl: 20, sub: 0, line: 785 } |  |  | 0.759 |
| walker |  | 4373 | 11 | Code::CodeKey { rung: Body, file: source/index.ts, decl: 23, sub: 0, line: 809 } |  |  | 0.759 |
| walker |  | 4454 | 81 | Json::Whole { file: tsconfig.json } |  |  | 0.760 |
| ns | 4496 |  | 341 | Every constructor validation rule and its error message | 3.6 |  | 0.736 |
| ns | 4629 |  | 133 | The concurrency getter/setter, including runtime mutation and its validation | 3.7 | 2.1 | 0.722 |
| ns | 4781 |  | 152 | Every FAQ question in readme.md (complete, eight questions) | 3.8 |  | 0.712 |
| ns | 5035 |  | 254 | Name-only roster of all 21 private methods and private getters of PQueue | 4.1 |  | 0.689 |
| walker |  | 5247 | 793 | Code::CodeKey { rung: Decl, file: source/options.ts, decl: 1, sub: 0, line: 27 } |  |  | 0.747 |
| walker |  | 5259 | 12 | Code::CodeKey { rung: Body, file: source/index.ts, decl: 11, sub: 0, line: 609 } |  |  | 0.747 |
| ns | 5332 |  | 297 | Every private field of PQueue with its type: the complete instance state | 4.2 |  | 0.722 |
| walker |  | 5459 | 200 | Markdown::Section { file: readme.md, section_index: 21, keeps_default_concavity: false } |  |  | 0.723 |
| walker |  | 5472 | 13 | Code::CodeKey { rung: Body, file: source/index.ts, decl: 24, sub: 0, line: 888 } |  |  | 0.723 |
| ns | 5589 |  | 257 | #tryToStartAnother: the admission decision and task dispatch | 4.3 | 4.1 | 0.696 |
| walker |  | 5691 | 219 | Markdown::Section { file: readme.md, section_index: 22, keeps_default_concavity: false } |  |  | 0.698 |
| ns | 5749 |  | 160 | add(): option normalization and automatic id assignment | 4.4 | 2.3 | 0.685 |
| ns | 5900 |  | 151 | add(): the run() prologue — pending accounting and runningTasks tracking | 4.5 | 4.4 | 0.672 |
| walker |  | 5958 | 267 | Markdown::Section { file: readme.md, section_index: 20, keeps_default_concavity: false } |  |  | 0.674 |
| ns | 6158 |  | 258 | add(): invoking the task, wrapping it in p-timeout, and racing the abort signal | 4.6 | 4.5 | 0.656 |
| walker |  | 6239 | 281 | Code::CodeKey { rung: Doc, file: source/index.ts, decl: 25, sub: 0, line: 918 } |  |  | 0.671 |
| walker |  | 6279 | 40 | Code::CodeKey { rung: Names, file: source/lower-bound.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.671 |
| ns | 6456 |  | 298 | add(): settlement, completed/error events, and the finally block that defers #next | 4.7 | 4.1 | 0.646 |
| walker |  | 6605 | 326 | Markdown::Section { file: readme.md, section_index: 14, keeps_default_concavity: false } |  |  | 0.646 |
| ns | 6741 |  | 285 | add(): enqueueing and the queued-task abort path | 4.8 | 4.4 | 0.628 |
| walker |  | 6944 | 339 | Code::CodeKey { rung: Doc, file: source/index.ts, decl: 26, sub: 0, line: 952 } |  |  | 0.641 |
| ns | 6963 |  | 222 | #isIntervalPausedAt: the strict sliding-window branch | 4.9 | 4.1 | 0.629 |
| ns | 7319 |  | 356 | #isIntervalPausedAt: the default fixed-window branch | 4.10 | 4.9 | 0.613 |
| walker |  | 7430 | 486 | Markdown::Section { file: readme.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.613 |
| ns | 7536 |  | 217 | PriorityQueue: header, options type, class declaration and the head-cursor invariant | 5.1 |  | 0.605 |
| ns | 7608 |  | 72 | Name-only roster of every PriorityQueue member (complete) | 5.2 |  | 0.608 |
| walker |  | 7905 | 475 | Markdown::Section { file: readme.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.608 |
| ns | 7940 |  | 332 | PriorityQueue.enqueue: the priority insertion algorithm | 5.3 | 5.2 | 0.593 |
| ns | 8283 |  | 343 | PriorityQueue.dequeue, size and #compact: the consumed-prefix machinery | 5.4 | 5.2 | 0.577 |
| walker |  | 8354 | 449 | Markdown::Section { file: readme.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.590 |
| ns | 8363 |  | 80 | source/lower-bound.ts: provenance and signature | 5.5 |  | 0.588 |
| ns | 8600 |  | 237 | Readme: the Custom QueueClass section with a complete worked implementation | 6.1 |  | 0.604 |
| ns | 8751 |  | 151 | Readme FAQ: how to cancel or remove a queued task | 6.2 | 3.8 | 0.601 |
| walker |  | 8853 | 499 | Markdown::Section { file: readme.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.601 |
| ns | 8934 |  | 183 | Readme FAQ: backpressure, and how concurrency relates to intervalCap | 6.3 | 3.8 | 0.606 |
| ns | 9040 |  | 106 | package.json scripts: how to build, test and benchmark | 7.1 |  | 0.609 |
| ns | 9216 |  | 176 | Test titles in test/debug.ts (all 11) | 7.2 |  | 0.604 |
| walker |  | 9260 | 407 | Markdown::Section { file: readme.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.604 |
| ns | 9396 |  | 180 | Test titles in test/priority-queue.ts (all 8) | 7.3 |  | 0.600 |
| ns | 9680 |  | 284 | Test titles in test/rate-limit.ts (all 9) and test/validation.ts (all 7) | 7.4 |  | 0.593 |
| ns | 9691 |  | 11 | CI workflow and the remaining .github files | 7.5 |  | 0.595 |
| walker |  | 9755 | 495 | Markdown::Section { file: readme.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.595 |
| ns | 9836 |  | 145 | The CI job definition itself | 7.6 | 7.5 | 0.587 |
| ns | 9907 |  | 71 | bench.ts: the five benchmark cases | 7.7 |  | 0.585 |
| ns | 9988 |  | 81 | tsconfig.json: the whole build configuration | 7.8 |  | 0.589 |
| walker |  | 9991 | 236 | Markdown::Section { file: readme.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.589 |

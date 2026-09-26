Score(3000)=0.604 I=0.875 C=0.417 ns_rows≤3K=18/47 grid(1000/1442/2080/3000/4327/6240/9000)=0.757/0.806/0.736/0.604/0.502/0.490/0.565

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 49 |  | 49 | README tagline + what act is | 1.1 |  | 0.000 |
| walker |  | 127 | 127 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 135 |  | 86 | main.go: process entry point | 1.2 |  | 0.000 |
| walker |  | 168 | 41 | Fs::DirListing { dir: pkg } |  |  | 0.000 |
| ns | 176 |  | 41 | pkg/: complete package list | 1.3 |  | 0.417 |
| walker |  | 200 | 32 | GoMod::Identity { file: go.mod } |  |  | 0.421 |
| walker |  | 209 | 9 | Fs::DirListing { dir: pkg/gh } |  |  | 0.421 |
| walker |  | 222 | 13 | Fs::DirListing { dir: pkg/filecollector } |  |  | 0.333 |
| ns | 222 |  | 46 | cmd/: complete file list | 1.4 |  | 0.333 |
| walker |  | 238 | 16 | Fs::DirListing { dir: pkg/workflowpattern } |  |  | 0.333 |
| walker |  | 257 | 19 | Fs::DirListing { dir: pkg/schema } |  |  | 0.333 |
| ns | 349 |  | 127 | Repository root: complete entry list | 1.5 |  | 0.429 |
| ns | 405 |  | 56 | pkg/model/: complete file list | 1.6 |  | 0.388 |
| walker |  | 442 | 185 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.578 |
| walker |  | 464 | 22 | Fs::DirListing { dir: pkg/exprparser } |  |  | 0.578 |
| walker |  | 488 | 24 | Fs::DirListing { dir: pkg/artifacts } |  |  | 0.579 |
| walker |  | 513 | 25 | Fs::DirListing { dir: pkg/artifactcache } |  |  | 0.580 |
| walker |  | 546 | 33 | Fs::DirListing { dir: pkg/lookpath } |  |  | 0.581 |
| ns | 591 |  | 186 | pkg/runner/: complete file list | 1.7 |  | 0.467 |
| walker |  | 592 | 46 | Fs::DirListing { dir: cmd } |  |  | 0.552 |
| walker |  | 671 | 79 | Fs::DirListing { dir: pkg/common } |  |  | 0.555 |
| walker |  | 680 | 9 | Fs::DirListing { dir: pkg/common/git } |  |  | 0.555 |
| ns | 701 |  | 110 | README: how act works, end to end | 1.8 |  | 0.552 |
| walker |  | 736 | 56 | Fs::DirListing { dir: pkg/model } |  |  | 0.622 |
| ns | 743 |  | 42 | Module path, Go version, released version | 1.9 |  | 0.621 |
| walker |  | 744 | 8 | Fs::DirListing { dir: .vscode } |  |  | 0.621 |
| ns | 858 |  | 115 | cmd.Execute: CLI entry | 2.1 |  | 0.590 |
| walker |  | 930 | 186 | Fs::DirListing { dir: pkg/runner } |  |  | 0.757 |
| walker |  | 934 | 4 | Fs::DirListing { dir: pkg/runner/hashfiles } |  |  | 0.757 |
| walker |  | 938 | 4 | Fs::DirListing { dir: pkg/runner/res } |  |  | 0.757 |
| ns | 1070 |  | 212 | Root cobra.Command definition | 2.2 |  | 0.721 |
| walker |  | 1098 | 160 | Fs::DirListing { dir: pkg/container } |  |  | 0.725 |
| walker |  | 1153 | 55 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.725 |
| walker |  | 1181 | 28 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.725 |
| walker |  | 1201 | 20 | Fs::DirListing { dir: .github } |  |  | 0.725 |
| walker |  | 1227 | 26 | Fs::DirListing { dir: .github/workflows } |  |  | 0.726 |
| walker |  | 1296 | 69 | Plaintext::Whole { file: Makefile } |  |  | 0.726 |
| walker |  | 1321 | 25 | Code::CodeKey { rung: Names, file: main.go, decl: 0, sub: 0, line: 0 } |  |  | 0.738 |
| walker |  | 1331 | 10 | Code::CodeKey { rung: Decl, file: main.go, decl: 1, sub: 0, line: 10 } |  |  | 0.755 |
| ns | 1371 |  | 301 | Command-local flags: names and shorthands | 2.3 |  | 0.674 |
| walker |  | 1382 | 51 | Code::CodeKey { rung: Decl, file: main.go, decl: 2, sub: 0, line: 13 } |  |  | 0.806 |
| walker |  | 1516 | 134 | Code::CodeKey { rung: ModuleDoc, file: pkg/artifactcache/doc.go, decl: 0, sub: 0, line: 0 } |  |  | 0.806 |
| walker |  | 1660 | 144 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.811 |
| walker |  | 1688 | 28 | Code::CodeKey { rung: Names, file: cmd/root.go, decl: 0, sub: 0, line: 0 } |  |  | 0.811 |
| ns | 1721 |  | 350 | Persistent flags: names and shorthands | 2.4 |  | 0.730 |
| walker |  | 1740 | 52 | Code::CodeKey { rung: Decl, file: cmd/root.go, decl: 1, sub: 0, line: 37 } |  |  | 0.730 |
| walker |  | 1755 | 15 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 2, sub: 0, line: 47 } |  |  | 0.732 |
| ns | 1867 |  | 146 | Input struct + path accessor roster | 2.5 |  | 0.710 |
| walker |  | 1887 | 132 | Code::CodeKey { rung: Names, file: cmd/input.go, decl: 0, sub: 0, line: 0 } |  |  | 0.729 |
| walker |  | 1894 | 7 | Code::CodeKey { rung: Decl, file: cmd/input.go, decl: 1, sub: 0, line: 10 } |  |  | 0.729 |
| walker |  | 1906 | 12 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 3, sub: 0, line: 90 } |  |  | 0.729 |
| walker |  | 1919 | 13 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 2, sub: 0, line: 85 } |  |  | 0.729 |
| walker |  | 1932 | 13 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 5, sub: 0, line: 99 } |  |  | 0.729 |
| walker |  | 1946 | 14 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 1, sub: 0, line: 10 } |  |  | 0.736 |
| walker |  | 1960 | 14 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 7, sub: 0, line: 109 } |  |  | 0.736 |
| walker |  | 1975 | 15 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 6, sub: 0, line: 104 } |  |  | 0.736 |
| walker |  | 1990 | 15 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 8, sub: 0, line: 114 } |  |  | 0.736 |
| walker |  | 1999 | 9 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 5, sub: 0, line: 99 } |  |  | 0.736 |
| walker |  | 2033 | 34 | Code::CodeKey { rung: Names, file: cmd/notices.go, decl: 0, sub: 0, line: 0 } |  |  | 0.736 |
| walker |  | 2062 | 29 | Code::CodeKey { rung: Decl, file: cmd/notices.go, decl: 1, sub: 0, line: 17 } |  |  | 0.736 |
| ns | 2145 |  | 278 | Config-file discovery: .actrc locations | 2.6 |  | 0.698 |
| walker |  | 2330 | 268 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.698 |
| walker |  | 2341 | 11 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 2, sub: 0, line: 85 } |  |  | 0.698 |
| walker |  | 2352 | 11 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 3, sub: 0, line: 90 } |  |  | 0.698 |
| walker |  | 2363 | 11 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 4, sub: 0, line: 94 } |  |  | 0.698 |
| walker |  | 2374 | 11 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 7, sub: 0, line: 109 } |  |  | 0.698 |
| walker |  | 2385 | 11 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 8, sub: 0, line: 114 } |  |  | 0.698 |
| walker |  | 2406 | 21 | Code::CodeKey { rung: Names, file: cmd/platforms.go, decl: 0, sub: 0, line: 0 } |  |  | 0.698 |
| walker |  | 2436 | 30 | Code::CodeKey { rung: Names, file: cmd/dir.go, decl: 0, sub: 0, line: 0 } |  |  | 0.698 |
| walker |  | 2441 | 5 | Code::CodeKey { rung: Decl, file: cmd/dir.go, decl: 1, sub: 0, line: 10 } |  |  | 0.698 |
| walker |  | 2459 | 18 | Code::CodeKey { rung: Names, file: cmd/list.go, decl: 0, sub: 0, line: 0 } |  |  | 0.698 |
| ns | 2461 |  | 316 | cmd package: complete function roster | 2.7 |  | 0.657 |
| walker |  | 2477 | 18 | Code::CodeKey { rung: Names, file: cmd/graph.go, decl: 0, sub: 0, line: 0 } |  |  | 0.657 |
| walker |  | 2646 | 169 | Code::CodeKey { rung: Names, file: pkg/exprparser/interpreter.go, decl: 0, sub: 0, line: 0 } |  |  | 0.657 |
| walker |  | 2649 | 3 | Code::CodeKey { rung: Decl, file: pkg/exprparser/interpreter.go, decl: 5, sub: 0, line: 43 } |  |  | 0.657 |
| walker |  | 2675 | 26 | Code::CodeKey { rung: Decl, file: pkg/exprparser/interpreter.go, decl: 7, sub: 0, line: 65 } |  |  | 0.657 |
| walker |  | 2705 | 30 | Code::CodeKey { rung: Decl, file: pkg/exprparser/interpreter.go, decl: 2, sub: 0, line: 30 } |  |  | 0.657 |
| walker |  | 2735 | 30 | Code::CodeKey { rung: Decl, file: pkg/exprparser/interpreter.go, decl: 3, sub: 0, line: 35 } |  |  | 0.657 |
| ns | 2808 |  | 347 | pkg/model: complete type roster | 3.1 |  | 0.616 |
| walker |  | 2897 | 162 | Code::CodeKey { rung: Decl, file: pkg/exprparser/interpreter.go, decl: 1, sub: 0, line: 14 } |  |  | 0.616 |
| walker |  | 2909 | 12 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 6, sub: 0, line: 104 } |  |  | 0.616 |
| ns | 2918 |  | 110 | Workflow struct | 3.2 | 3.1 | 0.604 |
| ns | 3148 |  | 230 | Planner interface + Plan/Stage/Run | 3.3 | 3.1 | 0.575 |
| ns | 3410 |  | 262 | Job struct: every supported job key | 3.4 | 3.1 | 0.557 |
| ns | 3634 |  | 224 | Step struct: every supported step key | 3.5 | 3.1 | 0.543 |
| walker |  | 3718 | 809 | GoMod::File { file: go.mod } |  |  | 0.544 |
| ns | 3966 |  | 332 | StepType constants | 3.6 | 3.1 | 0.524 |
| walker |  | 3975 | 257 | Code::CodeKey { rung: Names, file: pkg/schema/schema.go, decl: 0, sub: 0, line: 0 } |  |  | 0.524 |
| walker |  | 3978 | 3 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 9, sub: 0, line: 92 } |  |  | 0.524 |
| walker |  | 3981 | 3 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 10, sub: 0, line: 95 } |  |  | 0.524 |
| walker |  | 3984 | 3 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 11, sub: 0, line: 98 } |  |  | 0.524 |
| walker |  | 3991 | 7 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 15, sub: 0, line: 119 } |  |  | 0.524 |
| walker |  | 4005 | 14 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 1, sub: 0, line: 25 } |  |  | 0.524 |
| walker |  | 4021 | 16 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 7, sub: 0, line: 83 } |  |  | 0.524 |
| walker |  | 4040 | 19 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 5, sub: 0, line: 70 } |  |  | 0.524 |
| walker |  | 4065 | 25 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 8, sub: 0, line: 87 } |  |  | 0.524 |
| walker |  | 4094 | 29 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 14, sub: 0, line: 113 } |  |  | 0.524 |
| walker |  | 4145 | 51 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 4, sub: 0, line: 64 } |  |  | 0.524 |
| ns | 4187 |  | 221 | GithubContext: complete field-name roster | 3.7 | 3.1 | 0.502 |
| walker |  | 4251 | 106 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 3, sub: 0, line: 52 } |  |  | 0.502 |
| walker |  | 4281 | 30 | Code::CodeKey { rung: Names, file: pkg/lookpath/env.go, decl: 0, sub: 0, line: 0 } |  |  | 0.502 |
| walker |  | 4297 | 16 | Code::CodeKey { rung: Decl, file: pkg/lookpath/env.go, decl: 1, sub: 0, line: 5 } |  |  | 0.502 |
| walker |  | 4325 | 28 | Code::CodeKey { rung: Names, file: pkg/lookpath/error.go, decl: 0, sub: 0, line: 0 } |  |  | 0.502 |
| walker |  | 4343 | 18 | Code::CodeKey { rung: Decl, file: pkg/lookpath/error.go, decl: 1, sub: 0, line: 3 } |  |  | 0.502 |
| walker |  | 4352 | 9 | Code::CodeKey { rung: Body, file: pkg/lookpath/error.go, decl: 2, sub: 0, line: 8 } |  |  | 0.502 |
| walker |  | 4366 | 14 | Code::CodeKey { rung: Body, file: pkg/lookpath/env.go, decl: 2, sub: 0, line: 16 } |  |  | 0.502 |
| ns | 4470 |  | 283 | Executor type + complete combinator roster | 4.1 |  | 0.487 |
| walker |  | 4586 | 220 | Code::CodeKey { rung: Names, file: pkg/model/workflow.go, decl: 0, sub: 0, line: 0 } |  |  | 0.492 |
| walker |  | 4599 | 13 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 13, sub: 0, line: 171 } |  |  | 0.492 |
| walker |  | 4619 | 20 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 8, sub: 0, line: 113 } |  |  | 0.492 |
| walker |  | 4645 | 26 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 11, sub: 0, line: 161 } |  |  | 0.492 |
| walker |  | 4684 | 39 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 12, sub: 0, line: 166 } |  |  | 0.492 |
| walker |  | 4739 | 55 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 10, sub: 0, line: 154 } |  |  | 0.492 |
| walker |  | 4808 | 69 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 7, sub: 0, line: 105 } |  |  | 0.492 |
| walker |  | 4895 | 87 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 1, sub: 0, line: 19 } |  |  | 0.505 |
| walker |  | 4904 | 9 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 2, sub: 0, line: 29 } |  |  | 0.505 |
| ns | 5005 |  | 535 | pkg/runner: complete type roster | 4.2 |  | 0.482 |
| walker |  | 5106 | 202 | Code::CodeKey { rung: Names, file: pkg/model/workflow.go, decl: 0, sub: 1, line: 0 } |  |  | 0.487 |
| ns | 5115 |  | 110 | Runner interface and New() | 4.3 | 4.2 | 0.481 |
| walker |  | 5123 | 17 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 17, sub: 0, line: 225 } |  |  | 0.481 |
| walker |  | 5155 | 32 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 18, sub: 0, line: 230 } |  |  | 0.481 |
| walker |  | 5227 | 72 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 16, sub: 0, line: 216 } |  |  | 0.481 |
| walker |  | 5473 | 246 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 15, sub: 0, line: 196 } |  |  | 0.512 |
| walker |  | 5481 | 8 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 16, sub: 0, line: 216 } |  |  | 0.512 |
| walker |  | 5491 | 10 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 24, sub: 0, line: 311 } |  |  | 0.512 |
| walker |  | 5502 | 11 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 23, sub: 0, line: 293 } |  |  | 0.512 |
| ns | 5503 |  | 388 | runner.Config: fields (first half, rest elided) | 4.4 | 4.2 | 0.498 |
| walker |  | 5513 | 11 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 25, sub: 0, line: 330 } |  |  | 0.498 |
| walker |  | 5526 | 13 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 27, sub: 0, line: 388 } |  |  | 0.498 |
| walker |  | 5540 | 14 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 18, sub: 0, line: 230 } |  |  | 0.498 |
| walker |  | 5556 | 16 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 15, sub: 0, line: 196 } |  |  | 0.501 |
| walker |  | 5572 | 16 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 26, sub: 0, line: 383 } |  |  | 0.501 |
| walker |  | 5589 | 17 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 17, sub: 0, line: 225 } |  |  | 0.501 |
| walker |  | 5606 | 17 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 19, sub: 0, line: 236 } |  |  | 0.501 |
| ns | 5805 |  | 302 | RunContext struct | 4.5 | 4.2 | 0.487 |
| walker |  | 5833 | 227 | Code::CodeKey { rung: Names, file: pkg/model/workflow.go, decl: 0, sub: 2, line: 0 } |  |  | 0.492 |
| walker |  | 5944 | 111 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 30, sub: 0, line: 503 } |  |  | 0.492 |
| walker |  | 6073 | 129 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 33, sub: 0, line: 555 } |  |  | 0.492 |
| ns | 6160 |  | 355 | stepFactory: StepType -> implementation | 4.6 | 4.2 | 0.477 |
| walker |  | 6281 | 208 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 34, sub: 0, line: 569 } |  |  | 0.499 |
| walker |  | 6291 | 10 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 35, sub: 0, line: 586 } |  |  | 0.499 |
| walker |  | 6304 | 13 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 32, sub: 0, line: 530 } |  |  | 0.499 |
| walker |  | 6318 | 14 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 34, sub: 0, line: 569 } |  |  | 0.502 |
| walker |  | 6332 | 14 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 37, sub: 0, line: 603 } |  |  | 0.502 |
| walker |  | 6346 | 14 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 38, sub: 0, line: 615 } |  |  | 0.502 |
| walker |  | 6362 | 16 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 36, sub: 0, line: 598 } |  |  | 0.502 |
| ns | 6438 |  | 278 | step interface and stage enum | 4.7 | 4.2 | 0.487 |
| walker |  | 6595 | 233 | Code::CodeKey { rung: Names, file: pkg/model/workflow.go, decl: 0, sub: 3, line: 0 } |  |  | 0.491 |
| walker |  | 6626 | 31 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 46, sub: 0, line: 750 } |  |  | 0.491 |
| walker |  | 6835 | 209 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 40, sub: 0, line: 647 } |  |  | 0.521 |
| walker |  | 6848 | 13 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 42, sub: 0, line: 691 } |  |  | 0.521 |
| walker |  | 6865 | 17 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 44, sub: 0, line: 726 } |  |  | 0.521 |
| ns | 6933 |  | 495 | run_context.go: method roster (container lifecycle) | 4.8 |  | 0.506 |
| walker |  | 7043 | 178 | Code::CodeKey { rung: Names, file: pkg/model/planner.go, decl: 0, sub: 0, line: 0 } |  |  | 0.515 |
| walker |  | 7050 | 7 | Code::CodeKey { rung: Decl, file: pkg/model/planner.go, decl: 7, sub: 0, line: 53 } |  |  | 0.515 |
| walker |  | 7064 | 14 | Code::CodeKey { rung: Decl, file: pkg/model/planner.go, decl: 2, sub: 0, line: 25 } |  |  | 0.516 |
| walker |  | 7078 | 14 | Code::CodeKey { rung: Decl, file: pkg/model/planner.go, decl: 3, sub: 0, line: 30 } |  |  | 0.517 |
| ns | 7079 |  | 146 | ActionCache implementations | 4.9 | 4.2 | 0.508 |
| walker |  | 7100 | 22 | Code::CodeKey { rung: Decl, file: pkg/model/planner.go, decl: 4, sub: 0, line: 35 } |  |  | 0.510 |
| walker |  | 7160 | 60 | Code::CodeKey { rung: Decl, file: pkg/model/planner.go, decl: 1, sub: 0, line: 17 } |  |  | 0.517 |
| walker |  | 7173 | 13 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 1, sub: 0, line: 17 } |  |  | 0.520 |
| walker |  | 7186 | 13 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 6, sub: 0, line: 49 } |  |  | 0.522 |
| walker |  | 7200 | 14 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 2, sub: 0, line: 25 } |  |  | 0.525 |
| walker |  | 7214 | 14 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 3, sub: 0, line: 30 } |  |  | 0.529 |
| walker |  | 7230 | 16 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 4, sub: 0, line: 35 } |  |  | 0.533 |
| ns | 7239 |  | 160 | pkg/container: complete file list | 5.1 |  | 0.552 |
| walker |  | 7248 | 18 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 10, sub: 0, line: 304 } |  |  | 0.552 |
| walker |  | 7266 | 18 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 11, sub: 0, line: 318 } |  |  | 0.552 |
| walker |  | 7284 | 18 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 1, sub: 0, line: 19 } |  |  | 0.555 |
| walker |  | 7302 | 18 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 20, sub: 0, line: 253 } |  |  | 0.555 |
| walker |  | 7320 | 18 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 29, sub: 0, line: 501 } |  |  | 0.555 |
| walker |  | 7338 | 18 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 39, sub: 0, line: 645 } |  |  | 0.559 |
| walker |  | 7356 | 18 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 45, sub: 0, line: 742 } |  |  | 0.559 |
| walker |  | 7375 | 19 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 33, sub: 0, line: 555 } |  |  | 0.559 |
| walker |  | 7394 | 19 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 43, sub: 0, line: 714 } |  |  | 0.559 |
| ns | 7516 |  | 277 | Container interface | 5.2 |  | 0.550 |
| ns | 7658 |  | 142 | ExecutionsEnvironment interface | 5.3 |  | 0.544 |
| walker |  | 7759 | 365 | Code::CodeKey { rung: Names, file: pkg/common/executor.go, decl: 0, sub: 0, line: 0 } |  |  | 0.563 |
| walker |  | 7771 | 12 | Code::CodeKey { rung: Decl, file: pkg/common/executor.go, decl: 1, sub: 0, line: 12 } |  |  | 0.563 |
| walker |  | 7780 | 9 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 2, sub: 0, line: 17 } |  |  | 0.563 |
| walker |  | 7791 | 11 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 3, sub: 0, line: 22 } |  |  | 0.563 |
| walker |  | 7802 | 11 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 5, sub: 0, line: 33 } |  |  | 0.566 |
| walker |  | 7813 | 11 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 20, sub: 0, line: 237 } |  |  | 0.566 |
| walker |  | 7826 | 13 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 6, sub: 0, line: 36 } |  |  | 0.566 |
| walker |  | 7840 | 14 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 13, sub: 0, line: 141 } |  |  | 0.566 |
| walker |  | 7854 | 14 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 14, sub: 0, line: 160 } |  |  | 0.566 |
| walker |  | 7868 | 14 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 15, sub: 0, line: 179 } |  |  | 0.566 |
| walker |  | 7883 | 15 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 4, sub: 0, line: 30 } |  |  | 0.569 |
| walker |  | 7898 | 15 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 7, sub: 0, line: 45 } |  |  | 0.569 |
| walker |  | 7913 | 15 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 16, sub: 0, line: 198 } |  |  | 0.569 |
| ns | 7923 |  | 265 | Docker backend vs docker-less stub: the build-tag split | 5.4 |  | 0.560 |
| walker |  | 7928 | 15 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 19, sub: 0, line: 225 } |  |  | 0.560 |
| walker |  | 7944 | 16 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 1, sub: 0, line: 12 } |  |  | 0.560 |
| walker |  | 7960 | 16 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 9, sub: 0, line: 72 } |  |  | 0.560 |
| walker |  | 7976 | 16 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 17, sub: 0, line: 208 } |  |  | 0.560 |
| walker |  | 7992 | 16 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 18, sub: 0, line: 218 } |  |  | 0.560 |
| walker |  | 8009 | 17 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 10, sub: 0, line: 88 } |  |  | 0.560 |
| walker |  | 8029 | 20 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 8, sub: 0, line: 54 } |  |  | 0.560 |
| walker |  | 8049 | 20 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 11, sub: 0, line: 95 } |  |  | 0.560 |
| ns | 8100 |  | 177 | Docker socket discovery | 5.5 |  | 0.554 |
| walker |  | 8184 | 135 | Code::CodeKey { rung: Names, file: pkg/common/job_error.go, decl: 0, sub: 0, line: 0 } |  |  | 0.554 |
| ns | 8190 |  | 90 | Supporting packages: complete file lists (A) | 6.1 |  | 0.564 |
| walker |  | 8199 | 15 | Code::CodeKey { rung: Doc, file: pkg/common/job_error.go, decl: 2, sub: 0, line: 16 } |  |  | 0.564 |
| walker |  | 8336 | 137 | Code::CodeKey { rung: Names, file: pkg/artifacts/artifacts_v4.go, decl: 0, sub: 0, line: 0 } |  |  | 0.564 |
| walker |  | 8341 | 5 | Code::CodeKey { rung: Decl, file: pkg/artifacts/artifacts_v4.go, decl: 1, sub: 0, line: 105 } |  |  | 0.564 |
| ns | 8349 |  | 159 | Supporting packages: complete file lists (B) | 6.2 |  | 0.580 |
| walker |  | 8367 | 26 | Code::CodeKey { rung: Decl, file: pkg/artifacts/artifacts_v4.go, decl: 2, sub: 0, line: 118 } |  |  | 0.580 |
| walker |  | 8493 | 126 | Code::CodeKey { rung: Names, file: pkg/artifacts/server.go, decl: 0, sub: 0, line: 0 } |  |  | 0.575 |
| ns | 8493 |  | 144 | Context names the expression interpreter resolves | 6.3 |  | 0.575 |
| walker |  | 8504 | 11 | Code::CodeKey { rung: Decl, file: pkg/artifacts/server.go, decl: 7, sub: 0, line: 49 } |  |  | 0.575 |
| walker |  | 8518 | 14 | Code::CodeKey { rung: Decl, file: pkg/artifacts/server.go, decl: 6, sub: 0, line: 45 } |  |  | 0.575 |
| walker |  | 8534 | 16 | Code::CodeKey { rung: Decl, file: pkg/artifacts/server.go, decl: 5, sub: 0, line: 41 } |  |  | 0.575 |
| walker |  | 8554 | 20 | Code::CodeKey { rung: Decl, file: pkg/artifacts/server.go, decl: 1, sub: 0, line: 21 } |  |  | 0.575 |
| walker |  | 8586 | 32 | Code::CodeKey { rung: Decl, file: pkg/artifacts/server.go, decl: 2, sub: 0, line: 25 } |  |  | 0.575 |
| walker |  | 8618 | 32 | Code::CodeKey { rung: Decl, file: pkg/artifacts/server.go, decl: 3, sub: 0, line: 30 } |  |  | 0.570 |
| ns | 8618 |  | 125 | Expression functions act implements | 6.4 |  | 0.570 |
| walker |  | 8656 | 38 | Code::CodeKey { rung: Decl, file: pkg/artifacts/server.go, decl: 8, sub: 0, line: 53 } |  |  | 0.570 |
| walker |  | 8698 | 42 | Code::CodeKey { rung: Decl, file: pkg/artifacts/server.go, decl: 4, sub: 0, line: 35 } |  |  | 0.570 |
| walker |  | 8709 | 11 | Code::CodeKey { rung: Body, file: pkg/artifacts/artifacts_v4.go, decl: 3, sub: 0, line: 129 } |  |  | 0.570 |
| walker |  | 8720 | 11 | Code::CodeKey { rung: Body, file: pkg/artifacts/artifacts_v4.go, decl: 4, sub: 0, line: 133 } |  |  | 0.570 |
| ns | 8842 |  | 224 | Artifact cache server: routes and lifecycle | 6.5 |  | 0.565 |
| walker |  | 8975 | 255 | Code::CodeKey { rung: Names, file: pkg/filecollector/file_collector.go, decl: 0, sub: 0, line: 0 } |  |  | 0.565 |
| walker |  | 8978 | 3 | Code::CodeKey { rung: Decl, file: pkg/filecollector/file_collector.go, decl: 8, sub: 0, line: 100 } |  |  | 0.565 |
| walker |  | 8990 | 12 | Code::CodeKey { rung: Decl, file: pkg/filecollector/file_collector.go, decl: 4, sub: 0, line: 62 } |  |  | 0.565 |
| walker |  | 9002 | 12 | Code::CodeKey { rung: Decl, file: pkg/filecollector/file_collector.go, decl: 13, sub: 0, line: 127 } |  |  | 0.565 |
| ns | 9007 |  | 165 | Artifact server: Serve + the v4 route base | 6.6 |  | 0.562 |
| walker |  | 9029 | 27 | Code::CodeKey { rung: Decl, file: pkg/filecollector/file_collector.go, decl: 1, sub: 0, line: 20 } |  |  | 0.562 |
| walker |  | 9071 | 42 | Code::CodeKey { rung: Decl, file: pkg/filecollector/file_collector.go, decl: 2, sub: 0, line: 24 } |  |  | 0.562 |
| walker |  | 9123 | 52 | Code::CodeKey { rung: Decl, file: pkg/filecollector/file_collector.go, decl: 6, sub: 0, line: 85 } |  |  | 0.562 |
| walker |  | 9190 | 67 | Code::CodeKey { rung: Decl, file: pkg/filecollector/file_collector.go, decl: 7, sub: 0, line: 93 } |  |  | 0.562 |
| walker |  | 9289 | 99 | Code::CodeKey { rung: Names, file: pkg/artifactcache/handler.go, decl: 0, sub: 0, line: 0 } |  |  | 0.563 |
| walker |  | 9294 | 5 | Code::CodeKey { rung: Decl, file: pkg/artifactcache/handler.go, decl: 1, sub: 0, line: 32 } |  |  | 0.563 |
| ns | 9312 |  | 305 | pkg/common: complete exported-function roster | 6.7 |  | 0.555 |
| walker |  | 9316 | 22 | Code::CodeKey { rung: Doc, file: pkg/common/job_error.go, decl: 4, sub: 0, line: 31 } |  |  | 0.555 |
| walker |  | 9438 | 122 | Code::CodeKey { rung: Names, file: pkg/workflowpattern/workflow_pattern.go, decl: 0, sub: 0, line: 0 } |  |  | 0.555 |
| walker |  | 9450 | 12 | Code::CodeKey { rung: Decl, file: pkg/workflowpattern/workflow_pattern.go, decl: 3, sub: 0, line: 37 } |  |  | 0.555 |
| ns | 9475 |  | 163 | pkg/common/git: repo detection and cloning | 6.8 |  | 0.550 |
| walker |  | 9482 | 32 | Code::CodeKey { rung: Decl, file: pkg/workflowpattern/workflow_pattern.go, decl: 1, sub: 0, line: 9 } |  |  | 0.550 |
| walker |  | 9499 | 17 | Code::CodeKey { rung: Doc, file: pkg/workflowpattern/workflow_pattern.go, decl: 5, sub: 0, line: 151 } |  |  | 0.550 |
| walker |  | 9584 | 85 | Code::CodeKey { rung: Names, file: pkg/workflowpattern/trace_writer.go, decl: 0, sub: 0, line: 0 } |  |  | 0.550 |
| walker |  | 9587 | 3 | Code::CodeKey { rung: Decl, file: pkg/workflowpattern/trace_writer.go, decl: 3, sub: 0, line: 11 } |  |  | 0.550 |
| walker |  | 9601 | 14 | Code::CodeKey { rung: Decl, file: pkg/workflowpattern/trace_writer.go, decl: 1, sub: 0, line: 5 } |  |  | 0.545 |
| ns | 9601 |  | 126 | CI, issue-template and editor directories | 7.1 |  | 0.545 |
| walker |  | 9613 | 12 | Code::CodeKey { rung: Body, file: pkg/workflowpattern/trace_writer.go, decl: 5, sub: 0, line: 16 } |  |  | 0.545 |
| walker |  | 9632 | 19 | Code::CodeKey { rung: Doc, file: pkg/workflowpattern/workflow_pattern.go, decl: 6, sub: 0, line: 177 } |  |  | 0.545 |
| walker |  | 9740 | 108 | Code::CodeKey { rung: Names, file: pkg/container/docker_run.go, decl: 0, sub: 0, line: 0 } |  |  | 0.546 |
| walker |  | 9754 | 14 | Code::CodeKey { rung: Doc, file: pkg/container/docker_run.go, decl: 1, sub: 0, line: 45 } |  |  | 0.547 |
| ns | 9854 |  | 253 | Key direct dependencies | 7.2 |  | 0.555 |
| walker |  | 9980 | 226 | Code::CodeKey { rung: Names, file: pkg/runner/run_context.go, decl: 0, sub: 0, line: 0 } |  |  | 0.557 |

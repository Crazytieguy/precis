Score(3000)=0.606 I=0.876 C=0.419 ns_rows≤3K=18/47 grid(1000/1442/2080/3000/4327/6240/9000)=0.757/0.806/0.724/0.606/0.506/0.469/0.567

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 49 |  | 49 | README tagline + what act is | 1.1 |  | 0.000 |
| walker |  | 127 | 127 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 135 |  | 86 | main.go: process entry point | 1.2 |  | 0.000 |
| walker |  | 159 | 32 | GoMod::Identity { file: go.mod } |  |  | 0.000 |
| ns | 176 |  | 41 | pkg/: complete package list | 1.3 |  | 0.000 |
| ns | 222 |  | 46 | cmd/: complete file list | 1.4 |  | 0.000 |
| walker |  | 344 | 185 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.173 |
| ns | 349 |  | 127 | Repository root: complete entry list | 1.5 |  | 0.479 |
| walker |  | 385 | 41 | Fs::DirListing { dir: pkg } |  |  | 0.638 |
| walker |  | 394 | 9 | Fs::DirListing { dir: pkg/gh } |  |  | 0.638 |
| ns | 405 |  | 56 | pkg/model/: complete file list | 1.6 |  | 0.578 |
| walker |  | 407 | 13 | Fs::DirListing { dir: pkg/filecollector } |  |  | 0.578 |
| walker |  | 423 | 16 | Fs::DirListing { dir: pkg/workflowpattern } |  |  | 0.578 |
| walker |  | 442 | 19 | Fs::DirListing { dir: pkg/schema } |  |  | 0.578 |
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
| walker |  | 1252 | 25 | Code::CodeKey { rung: Names, file: main.go, decl: 0, sub: 0, line: 0 } |  |  | 0.738 |
| walker |  | 1262 | 10 | Code::CodeKey { rung: Decl, file: main.go, decl: 1, sub: 0, line: 10 } |  |  | 0.755 |
| walker |  | 1313 | 51 | Code::CodeKey { rung: Decl, file: main.go, decl: 2, sub: 0, line: 13 } |  |  | 0.903 |
| ns | 1371 |  | 301 | Command-local flags: names and shorthands | 2.3 |  | 0.806 |
| walker |  | 1580 | 267 | Plaintext::Whole { file: Makefile } |  |  | 0.806 |
| walker |  | 1714 | 134 | Code::CodeKey { rung: ModuleDoc, file: pkg/artifactcache/doc.go, decl: 0, sub: 0, line: 0 } |  |  | 0.806 |
| ns | 1721 |  | 350 | Persistent flags: names and shorthands | 2.4 |  | 0.725 |
| walker |  | 1858 | 144 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.729 |
| ns | 1867 |  | 146 | Input struct + path accessor roster | 2.5 |  | 0.707 |
| walker |  | 1886 | 28 | Code::CodeKey { rung: Names, file: cmd/root.go, decl: 0, sub: 0, line: 0 } |  |  | 0.708 |
| walker |  | 1938 | 52 | Code::CodeKey { rung: Decl, file: cmd/root.go, decl: 1, sub: 0, line: 37 } |  |  | 0.708 |
| walker |  | 1953 | 15 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 2, sub: 0, line: 47 } |  |  | 0.710 |
| walker |  | 2085 | 132 | Code::CodeKey { rung: Names, file: cmd/input.go, decl: 0, sub: 0, line: 0 } |  |  | 0.729 |
| ns | 2145 |  | 278 | Config-file discovery: .actrc locations | 2.6 |  | 0.690 |
| ns | 2461 |  | 316 | cmd package: complete function roster | 2.7 |  | 0.648 |
| walker |  | 2668 | 583 | Code::CodeKey { rung: Decl, file: cmd/input.go, decl: 1, sub: 0, line: 10 } |  |  | 0.651 |
| walker |  | 2680 | 12 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 3, sub: 0, line: 90 } |  |  | 0.651 |
| walker |  | 2693 | 13 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 2, sub: 0, line: 85 } |  |  | 0.651 |
| walker |  | 2706 | 13 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 5, sub: 0, line: 99 } |  |  | 0.651 |
| walker |  | 2720 | 14 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 1, sub: 0, line: 10 } |  |  | 0.658 |
| walker |  | 2734 | 14 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 7, sub: 0, line: 109 } |  |  | 0.658 |
| walker |  | 2749 | 15 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 6, sub: 0, line: 104 } |  |  | 0.658 |
| walker |  | 2764 | 15 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 8, sub: 0, line: 114 } |  |  | 0.658 |
| walker |  | 2798 | 34 | Code::CodeKey { rung: Names, file: cmd/notices.go, decl: 0, sub: 0, line: 0 } |  |  | 0.658 |
| ns | 2808 |  | 347 | pkg/model: complete type roster | 3.1 |  | 0.617 |
| walker |  | 2827 | 29 | Code::CodeKey { rung: Decl, file: cmd/notices.go, decl: 1, sub: 0, line: 17 } |  |  | 0.617 |
| walker |  | 2878 | 51 | Code::CodeKey { rung: Names, file: cmd/secrets.go, decl: 0, sub: 0, line: 0 } |  |  | 0.617 |
| walker |  | 2885 | 7 | Code::CodeKey { rung: Body, file: cmd/secrets.go, decl: 3, sub: 0, line: 40 } |  |  | 0.617 |
| ns | 2918 |  | 110 | Workflow struct | 3.2 | 3.1 | 0.606 |
| ns | 3148 |  | 230 | Planner interface + Plan/Stage/Run | 3.3 | 3.1 | 0.577 |
| walker |  | 3153 | 268 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.577 |
| walker |  | 3174 | 21 | Code::CodeKey { rung: Names, file: cmd/platforms.go, decl: 0, sub: 0, line: 0 } |  |  | 0.578 |
| walker |  | 3204 | 30 | Code::CodeKey { rung: Names, file: cmd/dir.go, decl: 0, sub: 0, line: 0 } |  |  | 0.578 |
| walker |  | 3209 | 5 | Code::CodeKey { rung: Decl, file: cmd/dir.go, decl: 1, sub: 0, line: 10 } |  |  | 0.578 |
| walker |  | 3227 | 18 | Code::CodeKey { rung: Names, file: cmd/list.go, decl: 0, sub: 0, line: 0 } |  |  | 0.579 |
| walker |  | 3245 | 18 | Code::CodeKey { rung: Names, file: cmd/graph.go, decl: 0, sub: 0, line: 0 } |  |  | 0.580 |
| ns | 3410 |  | 262 | Job struct: every supported job key | 3.4 | 3.1 | 0.562 |
| walker |  | 3414 | 169 | Code::CodeKey { rung: Names, file: pkg/exprparser/interpreter.go, decl: 0, sub: 0, line: 0 } |  |  | 0.562 |
| walker |  | 3417 | 3 | Code::CodeKey { rung: Decl, file: pkg/exprparser/interpreter.go, decl: 5, sub: 0, line: 43 } |  |  | 0.562 |
| walker |  | 3443 | 26 | Code::CodeKey { rung: Decl, file: pkg/exprparser/interpreter.go, decl: 7, sub: 0, line: 65 } |  |  | 0.562 |
| walker |  | 3473 | 30 | Code::CodeKey { rung: Decl, file: pkg/exprparser/interpreter.go, decl: 2, sub: 0, line: 30 } |  |  | 0.562 |
| walker |  | 3503 | 30 | Code::CodeKey { rung: Decl, file: pkg/exprparser/interpreter.go, decl: 3, sub: 0, line: 35 } |  |  | 0.562 |
| ns | 3634 |  | 224 | Step struct: every supported step key | 3.5 | 3.1 | 0.547 |
| walker |  | 3665 | 162 | Code::CodeKey { rung: Decl, file: pkg/exprparser/interpreter.go, decl: 1, sub: 0, line: 14 } |  |  | 0.547 |
| ns | 3966 |  | 332 | StepType constants | 3.6 | 3.1 | 0.527 |
| ns | 4187 |  | 221 | GithubContext: complete field-name roster | 3.7 | 3.1 | 0.505 |
| ns | 4470 |  | 283 | Executor type + complete combinator roster | 4.1 |  | 0.490 |
| walker |  | 4474 | 809 | GoMod::File { file: go.mod } |  |  | 0.491 |
| walker |  | 4731 | 257 | Code::CodeKey { rung: Names, file: pkg/schema/schema.go, decl: 0, sub: 0, line: 0 } |  |  | 0.491 |
| walker |  | 4734 | 3 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 9, sub: 0, line: 92 } |  |  | 0.491 |
| walker |  | 4737 | 3 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 10, sub: 0, line: 95 } |  |  | 0.491 |
| walker |  | 4740 | 3 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 11, sub: 0, line: 98 } |  |  | 0.491 |
| walker |  | 4754 | 14 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 1, sub: 0, line: 25 } |  |  | 0.491 |
| walker |  | 4770 | 16 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 7, sub: 0, line: 83 } |  |  | 0.491 |
| walker |  | 4789 | 19 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 5, sub: 0, line: 70 } |  |  | 0.491 |
| walker |  | 4814 | 25 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 8, sub: 0, line: 87 } |  |  | 0.491 |
| walker |  | 4842 | 28 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 15, sub: 0, line: 119 } |  |  | 0.491 |
| walker |  | 4871 | 29 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 14, sub: 0, line: 113 } |  |  | 0.491 |
| walker |  | 4922 | 51 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 4, sub: 0, line: 64 } |  |  | 0.491 |
| ns | 5005 |  | 535 | pkg/runner: complete type roster | 4.2 |  | 0.469 |
| walker |  | 5028 | 106 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 3, sub: 0, line: 52 } |  |  | 0.469 |
| walker |  | 5058 | 30 | Code::CodeKey { rung: Names, file: pkg/lookpath/env.go, decl: 0, sub: 0, line: 0 } |  |  | 0.469 |
| walker |  | 5074 | 16 | Code::CodeKey { rung: Decl, file: pkg/lookpath/env.go, decl: 1, sub: 0, line: 5 } |  |  | 0.469 |
| walker |  | 5102 | 28 | Code::CodeKey { rung: Names, file: pkg/lookpath/error.go, decl: 0, sub: 0, line: 0 } |  |  | 0.469 |
| ns | 5115 |  | 110 | Runner interface and New() | 4.3 | 4.2 | 0.462 |
| walker |  | 5120 | 18 | Code::CodeKey { rung: Decl, file: pkg/lookpath/error.go, decl: 1, sub: 0, line: 3 } |  |  | 0.462 |
| walker |  | 5129 | 9 | Code::CodeKey { rung: Body, file: pkg/lookpath/error.go, decl: 2, sub: 0, line: 8 } |  |  | 0.462 |
| walker |  | 5143 | 14 | Code::CodeKey { rung: Body, file: pkg/lookpath/env.go, decl: 2, sub: 0, line: 16 } |  |  | 0.462 |
| walker |  | 5363 | 220 | Code::CodeKey { rung: Names, file: pkg/model/workflow.go, decl: 0, sub: 0, line: 0 } |  |  | 0.467 |
| walker |  | 5376 | 13 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 13, sub: 0, line: 171 } |  |  | 0.467 |
| walker |  | 5396 | 20 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 8, sub: 0, line: 113 } |  |  | 0.467 |
| walker |  | 5422 | 26 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 11, sub: 0, line: 161 } |  |  | 0.467 |
| walker |  | 5461 | 39 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 12, sub: 0, line: 166 } |  |  | 0.467 |
| ns | 5503 |  | 388 | runner.Config: fields (first half, rest elided) | 4.4 | 4.2 | 0.455 |
| walker |  | 5516 | 55 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 10, sub: 0, line: 154 } |  |  | 0.455 |
| walker |  | 5585 | 69 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 7, sub: 0, line: 105 } |  |  | 0.455 |
| walker |  | 5672 | 87 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 1, sub: 0, line: 19 } |  |  | 0.466 |
| walker |  | 5681 | 9 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 2, sub: 0, line: 29 } |  |  | 0.466 |
| ns | 5805 |  | 302 | RunContext struct | 4.5 | 4.2 | 0.453 |
| walker |  | 5883 | 202 | Code::CodeKey { rung: Names, file: pkg/model/workflow.go, decl: 0, sub: 1, line: 0 } |  |  | 0.458 |
| walker |  | 5900 | 17 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 17, sub: 0, line: 225 } |  |  | 0.458 |
| walker |  | 5932 | 32 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 18, sub: 0, line: 230 } |  |  | 0.458 |
| walker |  | 6004 | 72 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 16, sub: 0, line: 216 } |  |  | 0.458 |
| ns | 6160 |  | 355 | stepFactory: StepType -> implementation | 4.6 | 4.2 | 0.444 |
| walker |  | 6250 | 246 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 15, sub: 0, line: 196 } |  |  | 0.472 |
| walker |  | 6258 | 8 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 16, sub: 0, line: 216 } |  |  | 0.472 |
| walker |  | 6268 | 10 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 24, sub: 0, line: 311 } |  |  | 0.472 |
| walker |  | 6279 | 11 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 23, sub: 0, line: 293 } |  |  | 0.472 |
| walker |  | 6290 | 11 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 25, sub: 0, line: 330 } |  |  | 0.472 |
| walker |  | 6303 | 13 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 27, sub: 0, line: 388 } |  |  | 0.472 |
| walker |  | 6317 | 14 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 18, sub: 0, line: 230 } |  |  | 0.472 |
| walker |  | 6333 | 16 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 15, sub: 0, line: 196 } |  |  | 0.476 |
| walker |  | 6349 | 16 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 26, sub: 0, line: 383 } |  |  | 0.476 |
| walker |  | 6366 | 17 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 17, sub: 0, line: 225 } |  |  | 0.476 |
| walker |  | 6383 | 17 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 19, sub: 0, line: 236 } |  |  | 0.476 |
| ns | 6438 |  | 278 | step interface and stage enum | 4.7 | 4.2 | 0.461 |
| walker |  | 6610 | 227 | Code::CodeKey { rung: Names, file: pkg/model/workflow.go, decl: 0, sub: 2, line: 0 } |  |  | 0.465 |
| walker |  | 6721 | 111 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 30, sub: 0, line: 503 } |  |  | 0.465 |
| walker |  | 6850 | 129 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 33, sub: 0, line: 555 } |  |  | 0.465 |
| ns | 6933 |  | 495 | run_context.go: method roster (container lifecycle) | 4.8 |  | 0.452 |
| walker |  | 7058 | 208 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 34, sub: 0, line: 569 } |  |  | 0.473 |
| walker |  | 7068 | 10 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 35, sub: 0, line: 586 } |  |  | 0.473 |
| ns | 7079 |  | 146 | ActionCache implementations | 4.9 | 4.2 | 0.465 |
| walker |  | 7081 | 13 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 32, sub: 0, line: 530 } |  |  | 0.465 |
| walker |  | 7095 | 14 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 34, sub: 0, line: 569 } |  |  | 0.467 |
| walker |  | 7109 | 14 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 37, sub: 0, line: 603 } |  |  | 0.467 |
| walker |  | 7123 | 14 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 38, sub: 0, line: 615 } |  |  | 0.467 |
| walker |  | 7139 | 16 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 36, sub: 0, line: 598 } |  |  | 0.467 |
| ns | 7239 |  | 160 | pkg/container: complete file list | 5.1 |  | 0.492 |
| walker |  | 7372 | 233 | Code::CodeKey { rung: Names, file: pkg/model/workflow.go, decl: 0, sub: 3, line: 0 } |  |  | 0.496 |
| walker |  | 7403 | 31 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 46, sub: 0, line: 750 } |  |  | 0.496 |
| ns | 7516 |  | 277 | Container interface | 5.2 |  | 0.488 |
| walker |  | 7612 | 209 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 40, sub: 0, line: 647 } |  |  | 0.514 |
| walker |  | 7625 | 13 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 42, sub: 0, line: 691 } |  |  | 0.514 |
| walker |  | 7642 | 17 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 44, sub: 0, line: 726 } |  |  | 0.514 |
| ns | 7658 |  | 142 | ExecutionsEnvironment interface | 5.3 |  | 0.508 |
| walker |  | 7820 | 178 | Code::CodeKey { rung: Names, file: pkg/model/planner.go, decl: 0, sub: 0, line: 0 } |  |  | 0.516 |
| walker |  | 7834 | 14 | Code::CodeKey { rung: Decl, file: pkg/model/planner.go, decl: 2, sub: 0, line: 25 } |  |  | 0.517 |
| walker |  | 7848 | 14 | Code::CodeKey { rung: Decl, file: pkg/model/planner.go, decl: 3, sub: 0, line: 30 } |  |  | 0.518 |
| walker |  | 7870 | 22 | Code::CodeKey { rung: Decl, file: pkg/model/planner.go, decl: 4, sub: 0, line: 35 } |  |  | 0.520 |
| walker |  | 7896 | 26 | Code::CodeKey { rung: Decl, file: pkg/model/planner.go, decl: 7, sub: 0, line: 53 } |  |  | 0.520 |
| ns | 7923 |  | 265 | Docker backend vs docker-less stub: the build-tag split | 5.4 |  | 0.511 |
| walker |  | 7956 | 60 | Code::CodeKey { rung: Decl, file: pkg/model/planner.go, decl: 1, sub: 0, line: 17 } |  |  | 0.517 |
| walker |  | 7969 | 13 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 1, sub: 0, line: 17 } |  |  | 0.520 |
| walker |  | 7982 | 13 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 6, sub: 0, line: 49 } |  |  | 0.522 |
| walker |  | 7996 | 14 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 2, sub: 0, line: 25 } |  |  | 0.525 |
| walker |  | 8010 | 14 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 3, sub: 0, line: 30 } |  |  | 0.528 |
| walker |  | 8026 | 16 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 4, sub: 0, line: 35 } |  |  | 0.532 |
| walker |  | 8044 | 18 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 10, sub: 0, line: 304 } |  |  | 0.532 |
| walker |  | 8062 | 18 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 11, sub: 0, line: 318 } |  |  | 0.532 |
| walker |  | 8080 | 18 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 1, sub: 0, line: 19 } |  |  | 0.535 |
| walker |  | 8098 | 18 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 20, sub: 0, line: 253 } |  |  | 0.535 |
| ns | 8100 |  | 177 | Docker socket discovery | 5.5 |  | 0.529 |
| walker |  | 8116 | 18 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 29, sub: 0, line: 501 } |  |  | 0.529 |
| walker |  | 8134 | 18 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 39, sub: 0, line: 645 } |  |  | 0.531 |
| walker |  | 8152 | 18 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 45, sub: 0, line: 742 } |  |  | 0.531 |
| walker |  | 8171 | 19 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 33, sub: 0, line: 555 } |  |  | 0.531 |
| walker |  | 8190 | 19 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 43, sub: 0, line: 714 } |  |  | 0.543 |
| ns | 8190 |  | 90 | Supporting packages: complete file lists (A) | 6.1 |  | 0.543 |
| ns | 8349 |  | 159 | Supporting packages: complete file lists (B) | 6.2 |  | 0.561 |
| ns | 8493 |  | 144 | Context names the expression interpreter resolves | 6.3 |  | 0.555 |
| walker |  | 8555 | 365 | Code::CodeKey { rung: Names, file: pkg/common/executor.go, decl: 0, sub: 0, line: 0 } |  |  | 0.571 |
| walker |  | 8567 | 12 | Code::CodeKey { rung: Decl, file: pkg/common/executor.go, decl: 1, sub: 0, line: 12 } |  |  | 0.571 |
| walker |  | 8576 | 9 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 2, sub: 0, line: 17 } |  |  | 0.571 |
| walker |  | 8587 | 11 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 3, sub: 0, line: 22 } |  |  | 0.571 |
| walker |  | 8598 | 11 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 5, sub: 0, line: 33 } |  |  | 0.574 |
| walker |  | 8609 | 11 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 20, sub: 0, line: 237 } |  |  | 0.574 |
| ns | 8618 |  | 125 | Expression functions act implements | 6.4 |  | 0.569 |
| walker |  | 8622 | 13 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 6, sub: 0, line: 36 } |  |  | 0.569 |
| walker |  | 8636 | 14 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 13, sub: 0, line: 141 } |  |  | 0.569 |
| walker |  | 8650 | 14 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 14, sub: 0, line: 160 } |  |  | 0.569 |
| walker |  | 8664 | 14 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 15, sub: 0, line: 179 } |  |  | 0.569 |
| walker |  | 8679 | 15 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 4, sub: 0, line: 30 } |  |  | 0.572 |
| walker |  | 8694 | 15 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 7, sub: 0, line: 45 } |  |  | 0.572 |
| walker |  | 8709 | 15 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 16, sub: 0, line: 198 } |  |  | 0.572 |
| walker |  | 8724 | 15 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 19, sub: 0, line: 225 } |  |  | 0.572 |
| walker |  | 8740 | 16 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 1, sub: 0, line: 12 } |  |  | 0.572 |
| walker |  | 8756 | 16 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 9, sub: 0, line: 72 } |  |  | 0.572 |
| walker |  | 8772 | 16 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 17, sub: 0, line: 208 } |  |  | 0.572 |
| walker |  | 8788 | 16 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 18, sub: 0, line: 218 } |  |  | 0.572 |
| walker |  | 8805 | 17 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 10, sub: 0, line: 88 } |  |  | 0.572 |
| walker |  | 8825 | 20 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 8, sub: 0, line: 54 } |  |  | 0.572 |
| ns | 8842 |  | 224 | Artifact cache server: routes and lifecycle | 6.5 |  | 0.567 |
| walker |  | 8845 | 20 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 11, sub: 0, line: 95 } |  |  | 0.567 |
| walker |  | 8980 | 135 | Code::CodeKey { rung: Names, file: pkg/common/job_error.go, decl: 0, sub: 0, line: 0 } |  |  | 0.567 |
| walker |  | 8995 | 15 | Code::CodeKey { rung: Doc, file: pkg/common/job_error.go, decl: 2, sub: 0, line: 16 } |  |  | 0.567 |
| ns | 9007 |  | 165 | Artifact server: Serve + the v4 route base | 6.6 |  | 0.562 |
| walker |  | 9132 | 137 | Code::CodeKey { rung: Names, file: pkg/artifacts/artifacts_v4.go, decl: 0, sub: 0, line: 0 } |  |  | 0.563 |
| walker |  | 9137 | 5 | Code::CodeKey { rung: Decl, file: pkg/artifacts/artifacts_v4.go, decl: 1, sub: 0, line: 105 } |  |  | 0.563 |
| walker |  | 9163 | 26 | Code::CodeKey { rung: Decl, file: pkg/artifacts/artifacts_v4.go, decl: 2, sub: 0, line: 118 } |  |  | 0.563 |
| walker |  | 9289 | 126 | Code::CodeKey { rung: Names, file: pkg/artifacts/server.go, decl: 0, sub: 0, line: 0 } |  |  | 0.564 |
| walker |  | 9300 | 11 | Code::CodeKey { rung: Decl, file: pkg/artifacts/server.go, decl: 7, sub: 0, line: 49 } |  |  | 0.564 |
| ns | 9312 |  | 305 | pkg/common: complete exported-function roster | 6.7 |  | 0.556 |
| walker |  | 9314 | 14 | Code::CodeKey { rung: Decl, file: pkg/artifacts/server.go, decl: 6, sub: 0, line: 45 } |  |  | 0.556 |
| walker |  | 9330 | 16 | Code::CodeKey { rung: Decl, file: pkg/artifacts/server.go, decl: 5, sub: 0, line: 41 } |  |  | 0.556 |
| walker |  | 9350 | 20 | Code::CodeKey { rung: Decl, file: pkg/artifacts/server.go, decl: 1, sub: 0, line: 21 } |  |  | 0.556 |
| walker |  | 9382 | 32 | Code::CodeKey { rung: Decl, file: pkg/artifacts/server.go, decl: 2, sub: 0, line: 25 } |  |  | 0.556 |
| walker |  | 9414 | 32 | Code::CodeKey { rung: Decl, file: pkg/artifacts/server.go, decl: 3, sub: 0, line: 30 } |  |  | 0.556 |
| walker |  | 9452 | 38 | Code::CodeKey { rung: Decl, file: pkg/artifacts/server.go, decl: 8, sub: 0, line: 53 } |  |  | 0.556 |
| ns | 9475 |  | 163 | pkg/common/git: repo detection and cloning | 6.8 |  | 0.551 |
| walker |  | 9494 | 42 | Code::CodeKey { rung: Decl, file: pkg/artifacts/server.go, decl: 4, sub: 0, line: 35 } |  |  | 0.551 |
| walker |  | 9505 | 11 | Code::CodeKey { rung: Body, file: pkg/artifacts/artifacts_v4.go, decl: 3, sub: 0, line: 129 } |  |  | 0.551 |
| walker |  | 9516 | 11 | Code::CodeKey { rung: Body, file: pkg/artifacts/artifacts_v4.go, decl: 4, sub: 0, line: 133 } |  |  | 0.551 |
| ns | 9601 |  | 126 | CI, issue-template and editor directories | 7.1 |  | 0.546 |
| walker |  | 9771 | 255 | Code::CodeKey { rung: Names, file: pkg/filecollector/file_collector.go, decl: 0, sub: 0, line: 0 } |  |  | 0.546 |
| walker |  | 9774 | 3 | Code::CodeKey { rung: Decl, file: pkg/filecollector/file_collector.go, decl: 8, sub: 0, line: 100 } |  |  | 0.546 |
| walker |  | 9786 | 12 | Code::CodeKey { rung: Decl, file: pkg/filecollector/file_collector.go, decl: 4, sub: 0, line: 62 } |  |  | 0.546 |
| walker |  | 9798 | 12 | Code::CodeKey { rung: Decl, file: pkg/filecollector/file_collector.go, decl: 13, sub: 0, line: 127 } |  |  | 0.546 |
| walker |  | 9825 | 27 | Code::CodeKey { rung: Decl, file: pkg/filecollector/file_collector.go, decl: 1, sub: 0, line: 20 } |  |  | 0.546 |
| ns | 9854 |  | 253 | Key direct dependencies | 7.2 |  | 0.554 |
| walker |  | 9867 | 42 | Code::CodeKey { rung: Decl, file: pkg/filecollector/file_collector.go, decl: 2, sub: 0, line: 24 } |  |  | 0.554 |
| walker |  | 9919 | 52 | Code::CodeKey { rung: Decl, file: pkg/filecollector/file_collector.go, decl: 6, sub: 0, line: 85 } |  |  | 0.554 |
| walker |  | 9986 | 67 | Code::CodeKey { rung: Decl, file: pkg/filecollector/file_collector.go, decl: 7, sub: 0, line: 93 } |  |  | 0.554 |
| walker |  | 9998 | 12 | Code::CodeKey { rung: Names, file: pkg/artifactcache/handler.go, decl: 0, sub: 0, line: 0 } |  |  | 0.554 |

Score(3000)=0.606 I=0.876 C=0.419 ns_rows≤3K=18/47 grid(1000/1442/2080/3000/4327/6240/9000)=0.757/0.806/0.724/0.606/0.503/0.477/0.565

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
| walker |  | 2092 | 7 | Code::CodeKey { rung: Decl, file: cmd/input.go, decl: 1, sub: 0, line: 10 } |  |  | 0.729 |
| walker |  | 2104 | 12 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 3, sub: 0, line: 90 } |  |  | 0.729 |
| walker |  | 2117 | 13 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 2, sub: 0, line: 85 } |  |  | 0.729 |
| walker |  | 2130 | 13 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 5, sub: 0, line: 99 } |  |  | 0.729 |
| walker |  | 2144 | 14 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 1, sub: 0, line: 10 } |  |  | 0.736 |
| ns | 2145 |  | 278 | Config-file discovery: .actrc locations | 2.6 |  | 0.698 |
| walker |  | 2158 | 14 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 7, sub: 0, line: 109 } |  |  | 0.698 |
| walker |  | 2173 | 15 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 6, sub: 0, line: 104 } |  |  | 0.698 |
| walker |  | 2188 | 15 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 8, sub: 0, line: 114 } |  |  | 0.698 |
| walker |  | 2197 | 9 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 5, sub: 0, line: 99 } |  |  | 0.698 |
| walker |  | 2231 | 34 | Code::CodeKey { rung: Names, file: cmd/notices.go, decl: 0, sub: 0, line: 0 } |  |  | 0.698 |
| walker |  | 2260 | 29 | Code::CodeKey { rung: Decl, file: cmd/notices.go, decl: 1, sub: 0, line: 17 } |  |  | 0.698 |
| ns | 2461 |  | 316 | cmd package: complete function roster | 2.7 |  | 0.655 |
| walker |  | 2528 | 268 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.655 |
| walker |  | 2539 | 11 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 2, sub: 0, line: 85 } |  |  | 0.655 |
| walker |  | 2550 | 11 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 3, sub: 0, line: 90 } |  |  | 0.655 |
| walker |  | 2561 | 11 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 4, sub: 0, line: 94 } |  |  | 0.655 |
| walker |  | 2581 | 20 | Code::CodeKey { rung: Names, file: cmd/secrets.go, decl: 0, sub: 0, line: 0 } |  |  | 0.655 |
| walker |  | 2588 | 7 | Code::CodeKey { rung: Body, file: cmd/secrets.go, decl: 1, sub: 0, line: 40 } |  |  | 0.655 |
| walker |  | 2599 | 11 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 7, sub: 0, line: 109 } |  |  | 0.655 |
| walker |  | 2610 | 11 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 8, sub: 0, line: 114 } |  |  | 0.655 |
| walker |  | 2631 | 21 | Code::CodeKey { rung: Names, file: cmd/platforms.go, decl: 0, sub: 0, line: 0 } |  |  | 0.656 |
| walker |  | 2661 | 30 | Code::CodeKey { rung: Names, file: cmd/dir.go, decl: 0, sub: 0, line: 0 } |  |  | 0.657 |
| walker |  | 2666 | 5 | Code::CodeKey { rung: Decl, file: cmd/dir.go, decl: 1, sub: 0, line: 10 } |  |  | 0.657 |
| walker |  | 2684 | 18 | Code::CodeKey { rung: Names, file: cmd/list.go, decl: 0, sub: 0, line: 0 } |  |  | 0.658 |
| walker |  | 2702 | 18 | Code::CodeKey { rung: Names, file: cmd/graph.go, decl: 0, sub: 0, line: 0 } |  |  | 0.659 |
| ns | 2808 |  | 347 | pkg/model: complete type roster | 3.1 |  | 0.617 |
| walker |  | 2871 | 169 | Code::CodeKey { rung: Names, file: pkg/exprparser/interpreter.go, decl: 0, sub: 0, line: 0 } |  |  | 0.617 |
| walker |  | 2874 | 3 | Code::CodeKey { rung: Decl, file: pkg/exprparser/interpreter.go, decl: 5, sub: 0, line: 43 } |  |  | 0.617 |
| walker |  | 2900 | 26 | Code::CodeKey { rung: Decl, file: pkg/exprparser/interpreter.go, decl: 7, sub: 0, line: 65 } |  |  | 0.617 |
| ns | 2918 |  | 110 | Workflow struct | 3.2 | 3.1 | 0.606 |
| walker |  | 2930 | 30 | Code::CodeKey { rung: Decl, file: pkg/exprparser/interpreter.go, decl: 2, sub: 0, line: 30 } |  |  | 0.606 |
| walker |  | 2960 | 30 | Code::CodeKey { rung: Decl, file: pkg/exprparser/interpreter.go, decl: 3, sub: 0, line: 35 } |  |  | 0.606 |
| walker |  | 3122 | 162 | Code::CodeKey { rung: Decl, file: pkg/exprparser/interpreter.go, decl: 1, sub: 0, line: 14 } |  |  | 0.606 |
| walker |  | 3134 | 12 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 6, sub: 0, line: 104 } |  |  | 0.606 |
| ns | 3148 |  | 230 | Planner interface + Plan/Stage/Run | 3.3 | 3.1 | 0.577 |
| ns | 3410 |  | 262 | Job struct: every supported job key | 3.4 | 3.1 | 0.558 |
| ns | 3634 |  | 224 | Step struct: every supported step key | 3.5 | 3.1 | 0.544 |
| walker |  | 3943 | 809 | GoMod::File { file: go.mod } |  |  | 0.545 |
| ns | 3966 |  | 332 | StepType constants | 3.6 | 3.1 | 0.525 |
| ns | 4187 |  | 221 | GithubContext: complete field-name roster | 3.7 | 3.1 | 0.503 |
| walker |  | 4200 | 257 | Code::CodeKey { rung: Names, file: pkg/schema/schema.go, decl: 0, sub: 0, line: 0 } |  |  | 0.503 |
| walker |  | 4203 | 3 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 9, sub: 0, line: 92 } |  |  | 0.503 |
| walker |  | 4206 | 3 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 10, sub: 0, line: 95 } |  |  | 0.503 |
| walker |  | 4209 | 3 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 11, sub: 0, line: 98 } |  |  | 0.503 |
| walker |  | 4216 | 7 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 15, sub: 0, line: 119 } |  |  | 0.503 |
| walker |  | 4230 | 14 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 1, sub: 0, line: 25 } |  |  | 0.503 |
| walker |  | 4246 | 16 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 7, sub: 0, line: 83 } |  |  | 0.503 |
| walker |  | 4265 | 19 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 5, sub: 0, line: 70 } |  |  | 0.503 |
| walker |  | 4290 | 25 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 8, sub: 0, line: 87 } |  |  | 0.503 |
| walker |  | 4319 | 29 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 14, sub: 0, line: 113 } |  |  | 0.503 |
| walker |  | 4370 | 51 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 4, sub: 0, line: 64 } |  |  | 0.503 |
| ns | 4470 |  | 283 | Executor type + complete combinator roster | 4.1 |  | 0.488 |
| walker |  | 4476 | 106 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 3, sub: 0, line: 52 } |  |  | 0.488 |
| walker |  | 4506 | 30 | Code::CodeKey { rung: Names, file: pkg/lookpath/env.go, decl: 0, sub: 0, line: 0 } |  |  | 0.488 |
| walker |  | 4522 | 16 | Code::CodeKey { rung: Decl, file: pkg/lookpath/env.go, decl: 1, sub: 0, line: 5 } |  |  | 0.488 |
| walker |  | 4550 | 28 | Code::CodeKey { rung: Names, file: pkg/lookpath/error.go, decl: 0, sub: 0, line: 0 } |  |  | 0.488 |
| walker |  | 4568 | 18 | Code::CodeKey { rung: Decl, file: pkg/lookpath/error.go, decl: 1, sub: 0, line: 3 } |  |  | 0.488 |
| walker |  | 4577 | 9 | Code::CodeKey { rung: Body, file: pkg/lookpath/error.go, decl: 2, sub: 0, line: 8 } |  |  | 0.488 |
| walker |  | 4591 | 14 | Code::CodeKey { rung: Body, file: pkg/lookpath/env.go, decl: 2, sub: 0, line: 16 } |  |  | 0.488 |
| walker |  | 4811 | 220 | Code::CodeKey { rung: Names, file: pkg/model/workflow.go, decl: 0, sub: 0, line: 0 } |  |  | 0.493 |
| walker |  | 4824 | 13 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 13, sub: 0, line: 171 } |  |  | 0.493 |
| walker |  | 4844 | 20 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 8, sub: 0, line: 113 } |  |  | 0.493 |
| walker |  | 4870 | 26 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 11, sub: 0, line: 161 } |  |  | 0.493 |
| walker |  | 4909 | 39 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 12, sub: 0, line: 166 } |  |  | 0.493 |
| walker |  | 4964 | 55 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 10, sub: 0, line: 154 } |  |  | 0.493 |
| ns | 5005 |  | 535 | pkg/runner: complete type roster | 4.2 |  | 0.471 |
| walker |  | 5033 | 69 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 7, sub: 0, line: 105 } |  |  | 0.471 |
| ns | 5115 |  | 110 | Runner interface and New() | 4.3 | 4.2 | 0.465 |
| walker |  | 5120 | 87 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 1, sub: 0, line: 19 } |  |  | 0.476 |
| walker |  | 5129 | 9 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 2, sub: 0, line: 29 } |  |  | 0.476 |
| walker |  | 5331 | 202 | Code::CodeKey { rung: Names, file: pkg/model/workflow.go, decl: 0, sub: 1, line: 0 } |  |  | 0.482 |
| walker |  | 5348 | 17 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 17, sub: 0, line: 225 } |  |  | 0.482 |
| walker |  | 5380 | 32 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 18, sub: 0, line: 230 } |  |  | 0.482 |
| walker |  | 5452 | 72 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 16, sub: 0, line: 216 } |  |  | 0.482 |
| ns | 5503 |  | 388 | runner.Config: fields (first half, rest elided) | 4.4 | 4.2 | 0.469 |
| walker |  | 5698 | 246 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 15, sub: 0, line: 196 } |  |  | 0.499 |
| walker |  | 5706 | 8 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 16, sub: 0, line: 216 } |  |  | 0.499 |
| walker |  | 5716 | 10 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 24, sub: 0, line: 311 } |  |  | 0.499 |
| walker |  | 5727 | 11 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 23, sub: 0, line: 293 } |  |  | 0.499 |
| walker |  | 5738 | 11 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 25, sub: 0, line: 330 } |  |  | 0.499 |
| walker |  | 5751 | 13 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 27, sub: 0, line: 388 } |  |  | 0.499 |
| walker |  | 5765 | 14 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 18, sub: 0, line: 230 } |  |  | 0.499 |
| walker |  | 5781 | 16 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 15, sub: 0, line: 196 } |  |  | 0.502 |
| walker |  | 5797 | 16 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 26, sub: 0, line: 383 } |  |  | 0.502 |
| ns | 5805 |  | 302 | RunContext struct | 4.5 | 4.2 | 0.488 |
| walker |  | 5814 | 17 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 17, sub: 0, line: 225 } |  |  | 0.488 |
| walker |  | 5831 | 17 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 19, sub: 0, line: 236 } |  |  | 0.488 |
| walker |  | 6058 | 227 | Code::CodeKey { rung: Names, file: pkg/model/workflow.go, decl: 0, sub: 2, line: 0 } |  |  | 0.492 |
| ns | 6160 |  | 355 | stepFactory: StepType -> implementation | 4.6 | 4.2 | 0.477 |
| walker |  | 6169 | 111 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 30, sub: 0, line: 503 } |  |  | 0.477 |
| walker |  | 6298 | 129 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 33, sub: 0, line: 555 } |  |  | 0.477 |
| ns | 6438 |  | 278 | step interface and stage enum | 4.7 | 4.2 | 0.463 |
| walker |  | 6506 | 208 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 34, sub: 0, line: 569 } |  |  | 0.484 |
| walker |  | 6516 | 10 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 35, sub: 0, line: 586 } |  |  | 0.484 |
| walker |  | 6529 | 13 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 32, sub: 0, line: 530 } |  |  | 0.484 |
| walker |  | 6543 | 14 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 34, sub: 0, line: 569 } |  |  | 0.487 |
| walker |  | 6557 | 14 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 37, sub: 0, line: 603 } |  |  | 0.487 |
| walker |  | 6571 | 14 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 38, sub: 0, line: 615 } |  |  | 0.487 |
| walker |  | 6587 | 16 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 36, sub: 0, line: 598 } |  |  | 0.487 |
| walker |  | 6820 | 233 | Code::CodeKey { rung: Names, file: pkg/model/workflow.go, decl: 0, sub: 3, line: 0 } |  |  | 0.492 |
| walker |  | 6851 | 31 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 46, sub: 0, line: 750 } |  |  | 0.492 |
| ns | 6933 |  | 495 | run_context.go: method roster (container lifecycle) | 4.8 |  | 0.478 |
| walker |  | 7060 | 209 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 40, sub: 0, line: 647 } |  |  | 0.507 |
| walker |  | 7073 | 13 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 42, sub: 0, line: 691 } |  |  | 0.507 |
| ns | 7079 |  | 146 | ActionCache implementations | 4.9 | 4.2 | 0.498 |
| walker |  | 7090 | 17 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 44, sub: 0, line: 726 } |  |  | 0.498 |
| ns | 7239 |  | 160 | pkg/container: complete file list | 5.1 |  | 0.520 |
| walker |  | 7268 | 178 | Code::CodeKey { rung: Names, file: pkg/model/planner.go, decl: 0, sub: 0, line: 0 } |  |  | 0.528 |
| walker |  | 7275 | 7 | Code::CodeKey { rung: Decl, file: pkg/model/planner.go, decl: 7, sub: 0, line: 53 } |  |  | 0.528 |
| walker |  | 7289 | 14 | Code::CodeKey { rung: Decl, file: pkg/model/planner.go, decl: 2, sub: 0, line: 25 } |  |  | 0.529 |
| walker |  | 7303 | 14 | Code::CodeKey { rung: Decl, file: pkg/model/planner.go, decl: 3, sub: 0, line: 30 } |  |  | 0.530 |
| walker |  | 7325 | 22 | Code::CodeKey { rung: Decl, file: pkg/model/planner.go, decl: 4, sub: 0, line: 35 } |  |  | 0.532 |
| walker |  | 7385 | 60 | Code::CodeKey { rung: Decl, file: pkg/model/planner.go, decl: 1, sub: 0, line: 17 } |  |  | 0.538 |
| walker |  | 7398 | 13 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 1, sub: 0, line: 17 } |  |  | 0.541 |
| walker |  | 7411 | 13 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 6, sub: 0, line: 49 } |  |  | 0.543 |
| walker |  | 7425 | 14 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 2, sub: 0, line: 25 } |  |  | 0.546 |
| walker |  | 7439 | 14 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 3, sub: 0, line: 30 } |  |  | 0.549 |
| walker |  | 7455 | 16 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 4, sub: 0, line: 35 } |  |  | 0.553 |
| walker |  | 7473 | 18 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 10, sub: 0, line: 304 } |  |  | 0.553 |
| walker |  | 7491 | 18 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 11, sub: 0, line: 318 } |  |  | 0.553 |
| walker |  | 7509 | 18 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 1, sub: 0, line: 19 } |  |  | 0.556 |
| ns | 7516 |  | 277 | Container interface | 5.2 |  | 0.548 |
| walker |  | 7527 | 18 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 20, sub: 0, line: 253 } |  |  | 0.548 |
| walker |  | 7545 | 18 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 29, sub: 0, line: 501 } |  |  | 0.548 |
| walker |  | 7563 | 18 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 39, sub: 0, line: 645 } |  |  | 0.551 |
| walker |  | 7581 | 18 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 45, sub: 0, line: 742 } |  |  | 0.551 |
| walker |  | 7600 | 19 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 33, sub: 0, line: 555 } |  |  | 0.551 |
| walker |  | 7619 | 19 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 43, sub: 0, line: 714 } |  |  | 0.551 |
| ns | 7658 |  | 142 | ExecutionsEnvironment interface | 5.3 |  | 0.545 |
| ns | 7923 |  | 265 | Docker backend vs docker-less stub: the build-tag split | 5.4 |  | 0.536 |
| walker |  | 7984 | 365 | Code::CodeKey { rung: Names, file: pkg/common/executor.go, decl: 0, sub: 0, line: 0 } |  |  | 0.554 |
| walker |  | 7996 | 12 | Code::CodeKey { rung: Decl, file: pkg/common/executor.go, decl: 1, sub: 0, line: 12 } |  |  | 0.554 |
| walker |  | 8005 | 9 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 2, sub: 0, line: 17 } |  |  | 0.554 |
| walker |  | 8016 | 11 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 3, sub: 0, line: 22 } |  |  | 0.554 |
| walker |  | 8027 | 11 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 5, sub: 0, line: 33 } |  |  | 0.557 |
| walker |  | 8038 | 11 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 20, sub: 0, line: 237 } |  |  | 0.557 |
| walker |  | 8051 | 13 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 6, sub: 0, line: 36 } |  |  | 0.557 |
| walker |  | 8065 | 14 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 13, sub: 0, line: 141 } |  |  | 0.557 |
| walker |  | 8079 | 14 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 14, sub: 0, line: 160 } |  |  | 0.557 |
| walker |  | 8093 | 14 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 15, sub: 0, line: 179 } |  |  | 0.557 |
| ns | 8100 |  | 177 | Docker socket discovery | 5.5 |  | 0.551 |
| walker |  | 8108 | 15 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 4, sub: 0, line: 30 } |  |  | 0.554 |
| walker |  | 8123 | 15 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 7, sub: 0, line: 45 } |  |  | 0.554 |
| walker |  | 8138 | 15 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 16, sub: 0, line: 198 } |  |  | 0.554 |
| walker |  | 8153 | 15 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 19, sub: 0, line: 225 } |  |  | 0.554 |
| walker |  | 8169 | 16 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 1, sub: 0, line: 12 } |  |  | 0.554 |
| walker |  | 8185 | 16 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 9, sub: 0, line: 72 } |  |  | 0.554 |
| ns | 8190 |  | 90 | Supporting packages: complete file lists (A) | 6.1 |  | 0.565 |
| walker |  | 8201 | 16 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 17, sub: 0, line: 208 } |  |  | 0.565 |
| walker |  | 8217 | 16 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 18, sub: 0, line: 218 } |  |  | 0.565 |
| walker |  | 8234 | 17 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 10, sub: 0, line: 88 } |  |  | 0.565 |
| walker |  | 8254 | 20 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 8, sub: 0, line: 54 } |  |  | 0.565 |
| walker |  | 8274 | 20 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 11, sub: 0, line: 95 } |  |  | 0.565 |
| ns | 8349 |  | 159 | Supporting packages: complete file lists (B) | 6.2 |  | 0.581 |
| walker |  | 8409 | 135 | Code::CodeKey { rung: Names, file: pkg/common/job_error.go, decl: 0, sub: 0, line: 0 } |  |  | 0.581 |
| walker |  | 8424 | 15 | Code::CodeKey { rung: Doc, file: pkg/common/job_error.go, decl: 2, sub: 0, line: 16 } |  |  | 0.581 |
| ns | 8493 |  | 144 | Context names the expression interpreter resolves | 6.3 |  | 0.575 |
| walker |  | 8561 | 137 | Code::CodeKey { rung: Names, file: pkg/artifacts/artifacts_v4.go, decl: 0, sub: 0, line: 0 } |  |  | 0.575 |
| walker |  | 8566 | 5 | Code::CodeKey { rung: Decl, file: pkg/artifacts/artifacts_v4.go, decl: 1, sub: 0, line: 105 } |  |  | 0.575 |
| walker |  | 8592 | 26 | Code::CodeKey { rung: Decl, file: pkg/artifacts/artifacts_v4.go, decl: 2, sub: 0, line: 118 } |  |  | 0.575 |
| ns | 8618 |  | 125 | Expression functions act implements | 6.4 |  | 0.571 |
| walker |  | 8718 | 126 | Code::CodeKey { rung: Names, file: pkg/artifacts/server.go, decl: 0, sub: 0, line: 0 } |  |  | 0.571 |
| walker |  | 8729 | 11 | Code::CodeKey { rung: Decl, file: pkg/artifacts/server.go, decl: 7, sub: 0, line: 49 } |  |  | 0.571 |
| walker |  | 8743 | 14 | Code::CodeKey { rung: Decl, file: pkg/artifacts/server.go, decl: 6, sub: 0, line: 45 } |  |  | 0.571 |
| walker |  | 8759 | 16 | Code::CodeKey { rung: Decl, file: pkg/artifacts/server.go, decl: 5, sub: 0, line: 41 } |  |  | 0.571 |
| walker |  | 8779 | 20 | Code::CodeKey { rung: Decl, file: pkg/artifacts/server.go, decl: 1, sub: 0, line: 21 } |  |  | 0.571 |
| walker |  | 8811 | 32 | Code::CodeKey { rung: Decl, file: pkg/artifacts/server.go, decl: 2, sub: 0, line: 25 } |  |  | 0.571 |
| ns | 8842 |  | 224 | Artifact cache server: routes and lifecycle | 6.5 |  | 0.565 |
| walker |  | 8843 | 32 | Code::CodeKey { rung: Decl, file: pkg/artifacts/server.go, decl: 3, sub: 0, line: 30 } |  |  | 0.565 |
| walker |  | 8881 | 38 | Code::CodeKey { rung: Decl, file: pkg/artifacts/server.go, decl: 8, sub: 0, line: 53 } |  |  | 0.565 |
| walker |  | 8923 | 42 | Code::CodeKey { rung: Decl, file: pkg/artifacts/server.go, decl: 4, sub: 0, line: 35 } |  |  | 0.565 |
| walker |  | 8934 | 11 | Code::CodeKey { rung: Body, file: pkg/artifacts/artifacts_v4.go, decl: 3, sub: 0, line: 129 } |  |  | 0.565 |
| walker |  | 8945 | 11 | Code::CodeKey { rung: Body, file: pkg/artifacts/artifacts_v4.go, decl: 4, sub: 0, line: 133 } |  |  | 0.565 |
| ns | 9007 |  | 165 | Artifact server: Serve + the v4 route base | 6.6 |  | 0.562 |
| walker |  | 9200 | 255 | Code::CodeKey { rung: Names, file: pkg/filecollector/file_collector.go, decl: 0, sub: 0, line: 0 } |  |  | 0.562 |
| walker |  | 9203 | 3 | Code::CodeKey { rung: Decl, file: pkg/filecollector/file_collector.go, decl: 8, sub: 0, line: 100 } |  |  | 0.562 |
| walker |  | 9215 | 12 | Code::CodeKey { rung: Decl, file: pkg/filecollector/file_collector.go, decl: 4, sub: 0, line: 62 } |  |  | 0.562 |
| walker |  | 9227 | 12 | Code::CodeKey { rung: Decl, file: pkg/filecollector/file_collector.go, decl: 13, sub: 0, line: 127 } |  |  | 0.562 |
| walker |  | 9254 | 27 | Code::CodeKey { rung: Decl, file: pkg/filecollector/file_collector.go, decl: 1, sub: 0, line: 20 } |  |  | 0.562 |
| walker |  | 9296 | 42 | Code::CodeKey { rung: Decl, file: pkg/filecollector/file_collector.go, decl: 2, sub: 0, line: 24 } |  |  | 0.562 |
| ns | 9312 |  | 305 | pkg/common: complete exported-function roster | 6.7 |  | 0.555 |
| walker |  | 9348 | 52 | Code::CodeKey { rung: Decl, file: pkg/filecollector/file_collector.go, decl: 6, sub: 0, line: 85 } |  |  | 0.555 |
| walker |  | 9415 | 67 | Code::CodeKey { rung: Decl, file: pkg/filecollector/file_collector.go, decl: 7, sub: 0, line: 93 } |  |  | 0.555 |
| ns | 9475 |  | 163 | pkg/common/git: repo detection and cloning | 6.8 |  | 0.550 |
| walker |  | 9514 | 99 | Code::CodeKey { rung: Names, file: pkg/artifactcache/handler.go, decl: 0, sub: 0, line: 0 } |  |  | 0.550 |
| walker |  | 9519 | 5 | Code::CodeKey { rung: Decl, file: pkg/artifactcache/handler.go, decl: 1, sub: 0, line: 32 } |  |  | 0.550 |
| walker |  | 9541 | 22 | Code::CodeKey { rung: Doc, file: pkg/common/job_error.go, decl: 4, sub: 0, line: 31 } |  |  | 0.550 |
| ns | 9601 |  | 126 | CI, issue-template and editor directories | 7.1 |  | 0.545 |
| walker |  | 9663 | 122 | Code::CodeKey { rung: Names, file: pkg/workflowpattern/workflow_pattern.go, decl: 0, sub: 0, line: 0 } |  |  | 0.545 |
| walker |  | 9675 | 12 | Code::CodeKey { rung: Decl, file: pkg/workflowpattern/workflow_pattern.go, decl: 3, sub: 0, line: 37 } |  |  | 0.545 |
| walker |  | 9707 | 32 | Code::CodeKey { rung: Decl, file: pkg/workflowpattern/workflow_pattern.go, decl: 1, sub: 0, line: 9 } |  |  | 0.545 |
| walker |  | 9724 | 17 | Code::CodeKey { rung: Doc, file: pkg/workflowpattern/workflow_pattern.go, decl: 5, sub: 0, line: 151 } |  |  | 0.545 |
| walker |  | 9809 | 85 | Code::CodeKey { rung: Names, file: pkg/workflowpattern/trace_writer.go, decl: 0, sub: 0, line: 0 } |  |  | 0.545 |
| walker |  | 9812 | 3 | Code::CodeKey { rung: Decl, file: pkg/workflowpattern/trace_writer.go, decl: 3, sub: 0, line: 11 } |  |  | 0.545 |
| walker |  | 9826 | 14 | Code::CodeKey { rung: Decl, file: pkg/workflowpattern/trace_writer.go, decl: 1, sub: 0, line: 5 } |  |  | 0.545 |
| walker |  | 9838 | 12 | Code::CodeKey { rung: Body, file: pkg/workflowpattern/trace_writer.go, decl: 5, sub: 0, line: 16 } |  |  | 0.545 |
| ns | 9854 |  | 253 | Key direct dependencies | 7.2 |  | 0.553 |
| walker |  | 9857 | 19 | Code::CodeKey { rung: Doc, file: pkg/workflowpattern/workflow_pattern.go, decl: 6, sub: 0, line: 177 } |  |  | 0.553 |
| walker |  | 9965 | 108 | Code::CodeKey { rung: Names, file: pkg/container/docker_run.go, decl: 0, sub: 0, line: 0 } |  |  | 0.554 |
| walker |  | 9979 | 14 | Code::CodeKey { rung: Doc, file: pkg/container/docker_run.go, decl: 1, sub: 0, line: 45 } |  |  | 0.555 |
| walker |  | 9992 | 13 | Code::CodeKey { rung: Names, file: pkg/runner/run_context.go, decl: 0, sub: 0, line: 0 } |  |  | 0.555 |

Score(3000)=0.607 I=0.876 C=0.420 ns_rows≤3K=18/47 grid(1000/1442/2080/3000/4327/6240/9000)=0.757/0.806/0.724/0.607/0.504/0.478/0.566

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
| walker |  | 2311 | 51 | Code::CodeKey { rung: Names, file: cmd/secrets.go, decl: 0, sub: 0, line: 0 } |  |  | 0.698 |
| walker |  | 2318 | 7 | Code::CodeKey { rung: Body, file: cmd/secrets.go, decl: 3, sub: 0, line: 40 } |  |  | 0.698 |
| ns | 2461 |  | 316 | cmd package: complete function roster | 2.7 |  | 0.656 |
| walker |  | 2586 | 268 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.656 |
| walker |  | 2597 | 11 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 2, sub: 0, line: 85 } |  |  | 0.656 |
| walker |  | 2608 | 11 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 3, sub: 0, line: 90 } |  |  | 0.656 |
| walker |  | 2619 | 11 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 4, sub: 0, line: 94 } |  |  | 0.656 |
| walker |  | 2630 | 11 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 7, sub: 0, line: 109 } |  |  | 0.656 |
| walker |  | 2641 | 11 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 8, sub: 0, line: 114 } |  |  | 0.656 |
| walker |  | 2662 | 21 | Code::CodeKey { rung: Names, file: cmd/platforms.go, decl: 0, sub: 0, line: 0 } |  |  | 0.657 |
| walker |  | 2692 | 30 | Code::CodeKey { rung: Names, file: cmd/dir.go, decl: 0, sub: 0, line: 0 } |  |  | 0.657 |
| walker |  | 2697 | 5 | Code::CodeKey { rung: Decl, file: cmd/dir.go, decl: 1, sub: 0, line: 10 } |  |  | 0.658 |
| walker |  | 2715 | 18 | Code::CodeKey { rung: Names, file: cmd/list.go, decl: 0, sub: 0, line: 0 } |  |  | 0.659 |
| walker |  | 2733 | 18 | Code::CodeKey { rung: Names, file: cmd/graph.go, decl: 0, sub: 0, line: 0 } |  |  | 0.660 |
| ns | 2808 |  | 347 | pkg/model: complete type roster | 3.1 |  | 0.618 |
| walker |  | 2902 | 169 | Code::CodeKey { rung: Names, file: pkg/exprparser/interpreter.go, decl: 0, sub: 0, line: 0 } |  |  | 0.618 |
| walker |  | 2905 | 3 | Code::CodeKey { rung: Decl, file: pkg/exprparser/interpreter.go, decl: 5, sub: 0, line: 43 } |  |  | 0.618 |
| ns | 2918 |  | 110 | Workflow struct | 3.2 | 3.1 | 0.607 |
| walker |  | 2931 | 26 | Code::CodeKey { rung: Decl, file: pkg/exprparser/interpreter.go, decl: 7, sub: 0, line: 65 } |  |  | 0.607 |
| walker |  | 2961 | 30 | Code::CodeKey { rung: Decl, file: pkg/exprparser/interpreter.go, decl: 2, sub: 0, line: 30 } |  |  | 0.607 |
| walker |  | 2991 | 30 | Code::CodeKey { rung: Decl, file: pkg/exprparser/interpreter.go, decl: 3, sub: 0, line: 35 } |  |  | 0.607 |
| ns | 3148 |  | 230 | Planner interface + Plan/Stage/Run | 3.3 | 3.1 | 0.578 |
| walker |  | 3153 | 162 | Code::CodeKey { rung: Decl, file: pkg/exprparser/interpreter.go, decl: 1, sub: 0, line: 14 } |  |  | 0.578 |
| walker |  | 3165 | 12 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 6, sub: 0, line: 104 } |  |  | 0.578 |
| ns | 3410 |  | 262 | Job struct: every supported job key | 3.4 | 3.1 | 0.559 |
| ns | 3634 |  | 224 | Step struct: every supported step key | 3.5 | 3.1 | 0.545 |
| ns | 3966 |  | 332 | StepType constants | 3.6 | 3.1 | 0.525 |
| walker |  | 3974 | 809 | GoMod::File { file: go.mod } |  |  | 0.526 |
| ns | 4187 |  | 221 | GithubContext: complete field-name roster | 3.7 | 3.1 | 0.504 |
| walker |  | 4231 | 257 | Code::CodeKey { rung: Names, file: pkg/schema/schema.go, decl: 0, sub: 0, line: 0 } |  |  | 0.504 |
| walker |  | 4234 | 3 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 9, sub: 0, line: 92 } |  |  | 0.504 |
| walker |  | 4237 | 3 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 10, sub: 0, line: 95 } |  |  | 0.504 |
| walker |  | 4240 | 3 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 11, sub: 0, line: 98 } |  |  | 0.504 |
| walker |  | 4247 | 7 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 15, sub: 0, line: 119 } |  |  | 0.504 |
| walker |  | 4261 | 14 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 1, sub: 0, line: 25 } |  |  | 0.504 |
| walker |  | 4277 | 16 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 7, sub: 0, line: 83 } |  |  | 0.504 |
| walker |  | 4296 | 19 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 5, sub: 0, line: 70 } |  |  | 0.504 |
| walker |  | 4321 | 25 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 8, sub: 0, line: 87 } |  |  | 0.504 |
| walker |  | 4350 | 29 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 14, sub: 0, line: 113 } |  |  | 0.504 |
| walker |  | 4401 | 51 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 4, sub: 0, line: 64 } |  |  | 0.504 |
| ns | 4470 |  | 283 | Executor type + complete combinator roster | 4.1 |  | 0.489 |
| walker |  | 4507 | 106 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 3, sub: 0, line: 52 } |  |  | 0.489 |
| walker |  | 4537 | 30 | Code::CodeKey { rung: Names, file: pkg/lookpath/env.go, decl: 0, sub: 0, line: 0 } |  |  | 0.489 |
| walker |  | 4553 | 16 | Code::CodeKey { rung: Decl, file: pkg/lookpath/env.go, decl: 1, sub: 0, line: 5 } |  |  | 0.489 |
| walker |  | 4581 | 28 | Code::CodeKey { rung: Names, file: pkg/lookpath/error.go, decl: 0, sub: 0, line: 0 } |  |  | 0.489 |
| walker |  | 4599 | 18 | Code::CodeKey { rung: Decl, file: pkg/lookpath/error.go, decl: 1, sub: 0, line: 3 } |  |  | 0.489 |
| walker |  | 4608 | 9 | Code::CodeKey { rung: Body, file: pkg/lookpath/error.go, decl: 2, sub: 0, line: 8 } |  |  | 0.489 |
| walker |  | 4622 | 14 | Code::CodeKey { rung: Body, file: pkg/lookpath/env.go, decl: 2, sub: 0, line: 16 } |  |  | 0.489 |
| walker |  | 4842 | 220 | Code::CodeKey { rung: Names, file: pkg/model/workflow.go, decl: 0, sub: 0, line: 0 } |  |  | 0.494 |
| walker |  | 4855 | 13 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 13, sub: 0, line: 171 } |  |  | 0.494 |
| walker |  | 4875 | 20 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 8, sub: 0, line: 113 } |  |  | 0.494 |
| walker |  | 4901 | 26 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 11, sub: 0, line: 161 } |  |  | 0.494 |
| walker |  | 4940 | 39 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 12, sub: 0, line: 166 } |  |  | 0.494 |
| walker |  | 4995 | 55 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 10, sub: 0, line: 154 } |  |  | 0.494 |
| ns | 5005 |  | 535 | pkg/runner: complete type roster | 4.2 |  | 0.472 |
| walker |  | 5064 | 69 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 7, sub: 0, line: 105 } |  |  | 0.472 |
| ns | 5115 |  | 110 | Runner interface and New() | 4.3 | 4.2 | 0.465 |
| walker |  | 5151 | 87 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 1, sub: 0, line: 19 } |  |  | 0.477 |
| walker |  | 5160 | 9 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 2, sub: 0, line: 29 } |  |  | 0.477 |
| walker |  | 5362 | 202 | Code::CodeKey { rung: Names, file: pkg/model/workflow.go, decl: 0, sub: 1, line: 0 } |  |  | 0.482 |
| walker |  | 5379 | 17 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 17, sub: 0, line: 225 } |  |  | 0.482 |
| walker |  | 5411 | 32 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 18, sub: 0, line: 230 } |  |  | 0.482 |
| walker |  | 5483 | 72 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 16, sub: 0, line: 216 } |  |  | 0.482 |
| ns | 5503 |  | 388 | runner.Config: fields (first half, rest elided) | 4.4 | 4.2 | 0.469 |
| walker |  | 5729 | 246 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 15, sub: 0, line: 196 } |  |  | 0.499 |
| walker |  | 5737 | 8 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 16, sub: 0, line: 216 } |  |  | 0.499 |
| walker |  | 5747 | 10 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 24, sub: 0, line: 311 } |  |  | 0.499 |
| walker |  | 5758 | 11 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 23, sub: 0, line: 293 } |  |  | 0.499 |
| walker |  | 5769 | 11 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 25, sub: 0, line: 330 } |  |  | 0.499 |
| walker |  | 5782 | 13 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 27, sub: 0, line: 388 } |  |  | 0.499 |
| walker |  | 5796 | 14 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 18, sub: 0, line: 230 } |  |  | 0.499 |
| ns | 5805 |  | 302 | RunContext struct | 4.5 | 4.2 | 0.486 |
| walker |  | 5812 | 16 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 15, sub: 0, line: 196 } |  |  | 0.489 |
| walker |  | 5828 | 16 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 26, sub: 0, line: 383 } |  |  | 0.489 |
| walker |  | 5845 | 17 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 17, sub: 0, line: 225 } |  |  | 0.489 |
| walker |  | 5862 | 17 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 19, sub: 0, line: 236 } |  |  | 0.489 |
| walker |  | 6089 | 227 | Code::CodeKey { rung: Names, file: pkg/model/workflow.go, decl: 0, sub: 2, line: 0 } |  |  | 0.493 |
| ns | 6160 |  | 355 | stepFactory: StepType -> implementation | 4.6 | 4.2 | 0.478 |
| walker |  | 6200 | 111 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 30, sub: 0, line: 503 } |  |  | 0.478 |
| walker |  | 6329 | 129 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 33, sub: 0, line: 555 } |  |  | 0.478 |
| ns | 6438 |  | 278 | step interface and stage enum | 4.7 | 4.2 | 0.464 |
| walker |  | 6537 | 208 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 34, sub: 0, line: 569 } |  |  | 0.485 |
| walker |  | 6547 | 10 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 35, sub: 0, line: 586 } |  |  | 0.485 |
| walker |  | 6560 | 13 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 32, sub: 0, line: 530 } |  |  | 0.485 |
| walker |  | 6574 | 14 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 34, sub: 0, line: 569 } |  |  | 0.488 |
| walker |  | 6588 | 14 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 37, sub: 0, line: 603 } |  |  | 0.488 |
| walker |  | 6602 | 14 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 38, sub: 0, line: 615 } |  |  | 0.488 |
| walker |  | 6618 | 16 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 36, sub: 0, line: 598 } |  |  | 0.488 |
| walker |  | 6851 | 233 | Code::CodeKey { rung: Names, file: pkg/model/workflow.go, decl: 0, sub: 3, line: 0 } |  |  | 0.492 |
| walker |  | 6882 | 31 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 46, sub: 0, line: 750 } |  |  | 0.492 |
| ns | 6933 |  | 495 | run_context.go: method roster (container lifecycle) | 4.8 |  | 0.479 |
| ns | 7079 |  | 146 | ActionCache implementations | 4.9 | 4.2 | 0.470 |
| walker |  | 7091 | 209 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 40, sub: 0, line: 647 } |  |  | 0.498 |
| walker |  | 7104 | 13 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 42, sub: 0, line: 691 } |  |  | 0.498 |
| walker |  | 7121 | 17 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 44, sub: 0, line: 726 } |  |  | 0.498 |
| ns | 7239 |  | 160 | pkg/container: complete file list | 5.1 |  | 0.520 |
| walker |  | 7299 | 178 | Code::CodeKey { rung: Names, file: pkg/model/planner.go, decl: 0, sub: 0, line: 0 } |  |  | 0.529 |
| walker |  | 7306 | 7 | Code::CodeKey { rung: Decl, file: pkg/model/planner.go, decl: 7, sub: 0, line: 53 } |  |  | 0.529 |
| walker |  | 7320 | 14 | Code::CodeKey { rung: Decl, file: pkg/model/planner.go, decl: 2, sub: 0, line: 25 } |  |  | 0.530 |
| walker |  | 7334 | 14 | Code::CodeKey { rung: Decl, file: pkg/model/planner.go, decl: 3, sub: 0, line: 30 } |  |  | 0.530 |
| walker |  | 7356 | 22 | Code::CodeKey { rung: Decl, file: pkg/model/planner.go, decl: 4, sub: 0, line: 35 } |  |  | 0.532 |
| walker |  | 7416 | 60 | Code::CodeKey { rung: Decl, file: pkg/model/planner.go, decl: 1, sub: 0, line: 17 } |  |  | 0.539 |
| walker |  | 7429 | 13 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 1, sub: 0, line: 17 } |  |  | 0.541 |
| walker |  | 7442 | 13 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 6, sub: 0, line: 49 } |  |  | 0.543 |
| walker |  | 7456 | 14 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 2, sub: 0, line: 25 } |  |  | 0.546 |
| walker |  | 7470 | 14 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 3, sub: 0, line: 30 } |  |  | 0.550 |
| walker |  | 7486 | 16 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 4, sub: 0, line: 35 } |  |  | 0.553 |
| walker |  | 7504 | 18 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 10, sub: 0, line: 304 } |  |  | 0.553 |
| ns | 7516 |  | 277 | Container interface | 5.2 |  | 0.545 |
| walker |  | 7522 | 18 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 11, sub: 0, line: 318 } |  |  | 0.545 |
| walker |  | 7540 | 18 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 1, sub: 0, line: 19 } |  |  | 0.548 |
| walker |  | 7558 | 18 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 20, sub: 0, line: 253 } |  |  | 0.548 |
| walker |  | 7576 | 18 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 29, sub: 0, line: 501 } |  |  | 0.548 |
| walker |  | 7594 | 18 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 39, sub: 0, line: 645 } |  |  | 0.551 |
| walker |  | 7612 | 18 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 45, sub: 0, line: 742 } |  |  | 0.551 |
| walker |  | 7631 | 19 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 33, sub: 0, line: 555 } |  |  | 0.551 |
| walker |  | 7650 | 19 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 43, sub: 0, line: 714 } |  |  | 0.551 |
| ns | 7658 |  | 142 | ExecutionsEnvironment interface | 5.3 |  | 0.545 |
| ns | 7923 |  | 265 | Docker backend vs docker-less stub: the build-tag split | 5.4 |  | 0.536 |
| walker |  | 8015 | 365 | Code::CodeKey { rung: Names, file: pkg/common/executor.go, decl: 0, sub: 0, line: 0 } |  |  | 0.555 |
| walker |  | 8027 | 12 | Code::CodeKey { rung: Decl, file: pkg/common/executor.go, decl: 1, sub: 0, line: 12 } |  |  | 0.555 |
| walker |  | 8036 | 9 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 2, sub: 0, line: 17 } |  |  | 0.555 |
| walker |  | 8047 | 11 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 3, sub: 0, line: 22 } |  |  | 0.555 |
| walker |  | 8058 | 11 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 5, sub: 0, line: 33 } |  |  | 0.558 |
| walker |  | 8069 | 11 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 20, sub: 0, line: 237 } |  |  | 0.558 |
| walker |  | 8082 | 13 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 6, sub: 0, line: 36 } |  |  | 0.558 |
| walker |  | 8096 | 14 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 13, sub: 0, line: 141 } |  |  | 0.558 |
| ns | 8100 |  | 177 | Docker socket discovery | 5.5 |  | 0.551 |
| walker |  | 8110 | 14 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 14, sub: 0, line: 160 } |  |  | 0.551 |
| walker |  | 8124 | 14 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 15, sub: 0, line: 179 } |  |  | 0.551 |
| walker |  | 8139 | 15 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 4, sub: 0, line: 30 } |  |  | 0.555 |
| walker |  | 8154 | 15 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 7, sub: 0, line: 45 } |  |  | 0.555 |
| walker |  | 8169 | 15 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 16, sub: 0, line: 198 } |  |  | 0.555 |
| walker |  | 8184 | 15 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 19, sub: 0, line: 225 } |  |  | 0.555 |
| ns | 8190 |  | 90 | Supporting packages: complete file lists (A) | 6.1 |  | 0.565 |
| walker |  | 8200 | 16 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 1, sub: 0, line: 12 } |  |  | 0.565 |
| walker |  | 8216 | 16 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 9, sub: 0, line: 72 } |  |  | 0.565 |
| walker |  | 8232 | 16 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 17, sub: 0, line: 208 } |  |  | 0.565 |
| walker |  | 8248 | 16 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 18, sub: 0, line: 218 } |  |  | 0.565 |
| walker |  | 8265 | 17 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 10, sub: 0, line: 88 } |  |  | 0.565 |
| walker |  | 8285 | 20 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 8, sub: 0, line: 54 } |  |  | 0.565 |
| walker |  | 8305 | 20 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 11, sub: 0, line: 95 } |  |  | 0.565 |
| ns | 8349 |  | 159 | Supporting packages: complete file lists (B) | 6.2 |  | 0.581 |
| walker |  | 8440 | 135 | Code::CodeKey { rung: Names, file: pkg/common/job_error.go, decl: 0, sub: 0, line: 0 } |  |  | 0.581 |
| walker |  | 8455 | 15 | Code::CodeKey { rung: Doc, file: pkg/common/job_error.go, decl: 2, sub: 0, line: 16 } |  |  | 0.581 |
| ns | 8493 |  | 144 | Context names the expression interpreter resolves | 6.3 |  | 0.576 |
| walker |  | 8592 | 137 | Code::CodeKey { rung: Names, file: pkg/artifacts/artifacts_v4.go, decl: 0, sub: 0, line: 0 } |  |  | 0.576 |
| walker |  | 8597 | 5 | Code::CodeKey { rung: Decl, file: pkg/artifacts/artifacts_v4.go, decl: 1, sub: 0, line: 105 } |  |  | 0.576 |
| ns | 8618 |  | 125 | Expression functions act implements | 6.4 |  | 0.571 |
| walker |  | 8623 | 26 | Code::CodeKey { rung: Decl, file: pkg/artifacts/artifacts_v4.go, decl: 2, sub: 0, line: 118 } |  |  | 0.571 |
| walker |  | 8749 | 126 | Code::CodeKey { rung: Names, file: pkg/artifacts/server.go, decl: 0, sub: 0, line: 0 } |  |  | 0.571 |
| walker |  | 8760 | 11 | Code::CodeKey { rung: Decl, file: pkg/artifacts/server.go, decl: 7, sub: 0, line: 49 } |  |  | 0.571 |
| walker |  | 8774 | 14 | Code::CodeKey { rung: Decl, file: pkg/artifacts/server.go, decl: 6, sub: 0, line: 45 } |  |  | 0.571 |
| walker |  | 8790 | 16 | Code::CodeKey { rung: Decl, file: pkg/artifacts/server.go, decl: 5, sub: 0, line: 41 } |  |  | 0.571 |
| walker |  | 8810 | 20 | Code::CodeKey { rung: Decl, file: pkg/artifacts/server.go, decl: 1, sub: 0, line: 21 } |  |  | 0.571 |
| walker |  | 8842 | 32 | Code::CodeKey { rung: Decl, file: pkg/artifacts/server.go, decl: 2, sub: 0, line: 25 } |  |  | 0.566 |
| ns | 8842 |  | 224 | Artifact cache server: routes and lifecycle | 6.5 |  | 0.566 |
| walker |  | 8874 | 32 | Code::CodeKey { rung: Decl, file: pkg/artifacts/server.go, decl: 3, sub: 0, line: 30 } |  |  | 0.566 |
| walker |  | 8912 | 38 | Code::CodeKey { rung: Decl, file: pkg/artifacts/server.go, decl: 8, sub: 0, line: 53 } |  |  | 0.566 |
| walker |  | 8954 | 42 | Code::CodeKey { rung: Decl, file: pkg/artifacts/server.go, decl: 4, sub: 0, line: 35 } |  |  | 0.566 |
| walker |  | 8965 | 11 | Code::CodeKey { rung: Body, file: pkg/artifacts/artifacts_v4.go, decl: 3, sub: 0, line: 129 } |  |  | 0.566 |
| walker |  | 8976 | 11 | Code::CodeKey { rung: Body, file: pkg/artifacts/artifacts_v4.go, decl: 4, sub: 0, line: 133 } |  |  | 0.566 |
| ns | 9007 |  | 165 | Artifact server: Serve + the v4 route base | 6.6 |  | 0.563 |
| walker |  | 9231 | 255 | Code::CodeKey { rung: Names, file: pkg/filecollector/file_collector.go, decl: 0, sub: 0, line: 0 } |  |  | 0.563 |
| walker |  | 9234 | 3 | Code::CodeKey { rung: Decl, file: pkg/filecollector/file_collector.go, decl: 8, sub: 0, line: 100 } |  |  | 0.563 |
| walker |  | 9246 | 12 | Code::CodeKey { rung: Decl, file: pkg/filecollector/file_collector.go, decl: 4, sub: 0, line: 62 } |  |  | 0.563 |
| walker |  | 9258 | 12 | Code::CodeKey { rung: Decl, file: pkg/filecollector/file_collector.go, decl: 13, sub: 0, line: 127 } |  |  | 0.563 |
| walker |  | 9285 | 27 | Code::CodeKey { rung: Decl, file: pkg/filecollector/file_collector.go, decl: 1, sub: 0, line: 20 } |  |  | 0.563 |
| ns | 9312 |  | 305 | pkg/common: complete exported-function roster | 6.7 |  | 0.555 |
| walker |  | 9327 | 42 | Code::CodeKey { rung: Decl, file: pkg/filecollector/file_collector.go, decl: 2, sub: 0, line: 24 } |  |  | 0.555 |
| walker |  | 9379 | 52 | Code::CodeKey { rung: Decl, file: pkg/filecollector/file_collector.go, decl: 6, sub: 0, line: 85 } |  |  | 0.555 |
| walker |  | 9446 | 67 | Code::CodeKey { rung: Decl, file: pkg/filecollector/file_collector.go, decl: 7, sub: 0, line: 93 } |  |  | 0.555 |
| ns | 9475 |  | 163 | pkg/common/git: repo detection and cloning | 6.8 |  | 0.550 |
| walker |  | 9545 | 99 | Code::CodeKey { rung: Names, file: pkg/artifactcache/handler.go, decl: 0, sub: 0, line: 0 } |  |  | 0.551 |
| walker |  | 9550 | 5 | Code::CodeKey { rung: Decl, file: pkg/artifactcache/handler.go, decl: 1, sub: 0, line: 32 } |  |  | 0.551 |
| walker |  | 9572 | 22 | Code::CodeKey { rung: Doc, file: pkg/common/job_error.go, decl: 4, sub: 0, line: 31 } |  |  | 0.551 |
| ns | 9601 |  | 126 | CI, issue-template and editor directories | 7.1 |  | 0.545 |
| walker |  | 9694 | 122 | Code::CodeKey { rung: Names, file: pkg/workflowpattern/workflow_pattern.go, decl: 0, sub: 0, line: 0 } |  |  | 0.545 |
| walker |  | 9706 | 12 | Code::CodeKey { rung: Decl, file: pkg/workflowpattern/workflow_pattern.go, decl: 3, sub: 0, line: 37 } |  |  | 0.545 |
| walker |  | 9738 | 32 | Code::CodeKey { rung: Decl, file: pkg/workflowpattern/workflow_pattern.go, decl: 1, sub: 0, line: 9 } |  |  | 0.545 |
| walker |  | 9755 | 17 | Code::CodeKey { rung: Doc, file: pkg/workflowpattern/workflow_pattern.go, decl: 5, sub: 0, line: 151 } |  |  | 0.545 |
| walker |  | 9840 | 85 | Code::CodeKey { rung: Names, file: pkg/workflowpattern/trace_writer.go, decl: 0, sub: 0, line: 0 } |  |  | 0.545 |
| walker |  | 9843 | 3 | Code::CodeKey { rung: Decl, file: pkg/workflowpattern/trace_writer.go, decl: 3, sub: 0, line: 11 } |  |  | 0.545 |
| ns | 9854 |  | 253 | Key direct dependencies | 7.2 |  | 0.553 |
| walker |  | 9857 | 14 | Code::CodeKey { rung: Decl, file: pkg/workflowpattern/trace_writer.go, decl: 1, sub: 0, line: 5 } |  |  | 0.553 |
| walker |  | 9869 | 12 | Code::CodeKey { rung: Body, file: pkg/workflowpattern/trace_writer.go, decl: 5, sub: 0, line: 16 } |  |  | 0.553 |
| walker |  | 9888 | 19 | Code::CodeKey { rung: Doc, file: pkg/workflowpattern/workflow_pattern.go, decl: 6, sub: 0, line: 177 } |  |  | 0.553 |
| walker |  | 9996 | 108 | Code::CodeKey { rung: Names, file: pkg/container/docker_run.go, decl: 0, sub: 0, line: 0 } |  |  | 0.555 |

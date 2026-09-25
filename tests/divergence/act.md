Score(3000)=0.580 I=0.870 C=0.387 ns_rows≤3K=18/47 grid(1000/1442/2080/3000/4327/6240/9000)=0.757/0.649/0.566/0.580/0.552/0.475/0.498

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
| walker |  | 551 | 5 | Fs::DirListing { dir: pkg/artifactcache/testdata/example } |  |  | 0.581 |
| ns | 591 |  | 186 | pkg/runner/: complete file list | 1.7 |  | 0.467 |
| walker |  | 597 | 46 | Fs::DirListing { dir: cmd } |  |  | 0.552 |
| walker |  | 616 | 19 | Fs::DirListing { dir: cmd/testdata } |  |  | 0.552 |
| walker |  | 695 | 79 | Fs::DirListing { dir: pkg/common } |  |  | 0.555 |
| ns | 701 |  | 110 | README: how act works, end to end | 1.8 |  | 0.552 |
| walker |  | 704 | 9 | Fs::DirListing { dir: pkg/common/git } |  |  | 0.552 |
| ns | 743 |  | 42 | Module path, Go version, released version | 1.9 |  | 0.553 |
| walker |  | 760 | 56 | Fs::DirListing { dir: pkg/model } |  |  | 0.621 |
| walker |  | 768 | 8 | Fs::DirListing { dir: .vscode } |  |  | 0.621 |
| ns | 858 |  | 115 | cmd.Execute: CLI entry | 2.1 |  | 0.590 |
| walker |  | 954 | 186 | Fs::DirListing { dir: pkg/runner } |  |  | 0.757 |
| walker |  | 958 | 4 | Fs::DirListing { dir: pkg/runner/hashfiles } |  |  | 0.757 |
| walker |  | 962 | 4 | Fs::DirListing { dir: pkg/runner/res } |  |  | 0.757 |
| ns | 1070 |  | 212 | Root cobra.Command definition | 2.2 |  | 0.721 |
| walker |  | 1122 | 160 | Fs::DirListing { dir: pkg/container } |  |  | 0.725 |
| walker |  | 1140 | 18 | Fs::DirListing { dir: pkg/artifacts/testdata } |  |  | 0.726 |
| walker |  | 1144 | 4 | Fs::DirListing { dir: pkg/artifacts/testdata/GHSL-2023-004 } |  |  | 0.726 |
| walker |  | 1148 | 4 | Fs::DirListing { dir: pkg/artifacts/testdata/upload-and-download } |  |  | 0.726 |
| walker |  | 1152 | 4 | Fs::DirListing { dir: pkg/artifacts/testdata/v4 } |  |  | 0.726 |
| walker |  | 1173 | 21 | Fs::DirListing { dir: pkg/model/testdata } |  |  | 0.726 |
| walker |  | 1177 | 4 | Fs::DirListing { dir: pkg/model/testdata/container-volumes } |  |  | 0.726 |
| walker |  | 1181 | 4 | Fs::DirListing { dir: pkg/model/testdata/strategy } |  |  | 0.726 |
| walker |  | 1187 | 6 | Fs::DirListing { dir: pkg/model/testdata/empty-workflow } |  |  | 0.726 |
| walker |  | 1194 | 7 | Fs::DirListing { dir: pkg/model/testdata/nested } |  |  | 0.726 |
| walker |  | 1200 | 6 | Fs::DirListing { dir: pkg/model/testdata/nested/workflows } |  |  | 0.726 |
| walker |  | 1218 | 18 | Code::CodeKey { rung: ModuleDoc, file: pkg/container/docker_cli.go, decl: 0, sub: 0, line: 0 } |  |  | 0.726 |
| walker |  | 1241 | 23 | Fs::DirListing { dir: pkg/exprparser/testdata } |  |  | 0.726 |
| walker |  | 1248 | 7 | Fs::DirListing { dir: pkg/exprparser/testdata/for-hashing-3 } |  |  | 0.726 |
| walker |  | 1253 | 5 | Fs::DirListing { dir: pkg/exprparser/testdata/for-hashing-3/nested } |  |  | 0.726 |
| walker |  | 1308 | 55 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.726 |
| walker |  | 1336 | 28 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.726 |
| walker |  | 1356 | 20 | Fs::DirListing { dir: .github } |  |  | 0.726 |
| ns | 1371 |  | 301 | Command-local flags: names and shorthands | 2.3 |  | 0.648 |
| walker |  | 1382 | 26 | Fs::DirListing { dir: .github/workflows } |  |  | 0.649 |
| walker |  | 1419 | 37 | Fs::DirListing { dir: pkg/container/testdata } |  |  | 0.649 |
| walker |  | 1423 | 4 | Fs::DirListing { dir: pkg/container/testdata/docker-pull-options } |  |  | 0.649 |
| walker |  | 1427 | 4 | Fs::DirListing { dir: pkg/container/testdata/scratch } |  |  | 0.649 |
| walker |  | 1451 | 24 | Fs::DirListing { dir: pkg/model/testdata/invalid-job-name } |  |  | 0.649 |
| ns | 1721 |  | 350 | Persistent flags: names and shorthands | 2.4 |  | 0.583 |
| ns | 1867 |  | 146 | Input struct + path accessor roster | 2.5 |  | 0.566 |
| ns | 2145 |  | 278 | Config-file discovery: .actrc locations | 2.6 |  | 0.536 |
| walker |  | 2166 | 715 | Fs::DirListing { dir: pkg/runner/testdata } |  |  | 0.536 |
| walker |  | 2191 | 25 | Code::CodeKey { rung: Names, file: main.go, decl: 0, sub: 0, line: 0 } |  |  | 0.545 |
| walker |  | 2242 | 51 | Code::CodeKey { rung: Decl, file: main.go, decl: 2, sub: 0, line: 13 } |  |  | 0.633 |
| walker |  | 2252 | 10 | Code::CodeKey { rung: Doc, file: main.go, decl: 1, sub: 0, line: 11 } |  |  | 0.667 |
| ns | 2461 |  | 316 | cmd package: complete function roster | 2.7 |  | 0.626 |
| walker |  | 2519 | 267 | Plaintext::Whole { file: Makefile } |  |  | 0.626 |
| walker |  | 2525 | 6 | Plaintext::DeclSurface { file: pkg/exprparser/testdata/for-hashing-1.txt } |  |  | 0.626 |
| walker |  | 2531 | 6 | Plaintext::DeclSurface { file: pkg/exprparser/testdata/for-hashing-2.txt } |  |  | 0.626 |
| walker |  | 2665 | 134 | Code::CodeKey { rung: ModuleDoc, file: pkg/artifactcache/doc.go, decl: 0, sub: 0, line: 0 } |  |  | 0.626 |
| walker |  | 2699 | 34 | Code::CodeKey { rung: Names, file: cmd/dir.go, decl: 0, sub: 0, line: 0 } |  |  | 0.627 |
| walker |  | 2708 | 9 | Code::CodeKey { rung: Decl, file: cmd/dir.go, decl: 1, sub: 0, line: 10 } |  |  | 0.627 |
| ns | 2808 |  | 347 | pkg/model: complete type roster | 3.1 |  | 0.588 |
| walker |  | 2885 | 177 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.591 |
| walker |  | 2892 | 7 | Plaintext::DeclSurface { file: pkg/container/testdata/scratch/test.txt } |  |  | 0.591 |
| ns | 2918 |  | 110 | Workflow struct | 3.2 | 3.1 | 0.580 |
| walker |  | 2959 | 67 | Plaintext::Whole { file: pkg/container/testdata/Dockerfile } |  |  | 0.580 |
| walker |  | 2967 | 8 | Plaintext::DeclSurface { file: pkg/exprparser/testdata/for-hashing-3/data.txt } |  |  | 0.580 |
| walker |  | 3114 | 147 | Code::CodeKey { rung: Names, file: cmd/notices.go, decl: 0, sub: 0, line: 0 } |  |  | 0.585 |
| walker |  | 3141 | 27 | Code::CodeKey { rung: Decl, file: cmd/notices.go, decl: 1, sub: 0, line: 17 } |  |  | 0.585 |
| ns | 3148 |  | 230 | Planner interface + Plan/Stage/Run | 3.3 | 3.1 | 0.557 |
| walker |  | 3291 | 150 | Code::CodeKey { rung: Names, file: cmd/input.go, decl: 0, sub: 0, line: 0 } |  |  | 0.576 |
| walker |  | 3300 | 9 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 6, sub: 0, line: 99 } |  |  | 0.576 |
| walker |  | 3311 | 11 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 3, sub: 0, line: 85 } |  |  | 0.576 |
| walker |  | 3322 | 11 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 4, sub: 0, line: 90 } |  |  | 0.576 |
| walker |  | 3333 | 11 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 5, sub: 0, line: 94 } |  |  | 0.576 |
| walker |  | 3345 | 12 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 4, sub: 0, line: 90 } |  |  | 0.576 |
| walker |  | 3358 | 13 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 3, sub: 0, line: 85 } |  |  | 0.576 |
| walker |  | 3371 | 13 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 6, sub: 0, line: 99 } |  |  | 0.576 |
| walker |  | 3385 | 14 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 8, sub: 0, line: 109 } |  |  | 0.576 |
| walker |  | 3400 | 15 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 7, sub: 0, line: 104 } |  |  | 0.576 |
| ns | 3410 |  | 262 | Job struct: every supported job key | 3.4 | 3.1 | 0.557 |
| walker |  | 3415 | 15 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 9, sub: 0, line: 114 } |  |  | 0.557 |
| walker |  | 3466 | 51 | Code::CodeKey { rung: Names, file: cmd/secrets.go, decl: 0, sub: 0, line: 0 } |  |  | 0.561 |
| walker |  | 3473 | 7 | Code::CodeKey { rung: Body, file: cmd/secrets.go, decl: 3, sub: 0, line: 40 } |  |  | 0.561 |
| ns | 3634 |  | 224 | Step struct: every supported step key | 3.5 | 3.1 | 0.547 |
| walker |  | 3741 | 268 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.547 |
| walker |  | 3759 | 18 | Code::CodeKey { rung: Names, file: cmd/graph.go, decl: 0, sub: 0, line: 0 } |  |  | 0.549 |
| walker |  | 3777 | 18 | Code::CodeKey { rung: Names, file: cmd/list.go, decl: 0, sub: 0, line: 0 } |  |  | 0.550 |
| ns | 3966 |  | 332 | StepType constants | 3.6 | 3.1 | 0.530 |
| walker |  | 4151 | 374 | Code::CodeKey { rung: Names, file: cmd/root.go, decl: 0, sub: 0, line: 0 } |  |  | 0.572 |
| ns | 4187 |  | 221 | GithubContext: complete field-name roster | 3.7 | 3.1 | 0.548 |
| walker |  | 4201 | 50 | Code::CodeKey { rung: Decl, file: cmd/root.go, decl: 1, sub: 0, line: 37 } |  |  | 0.548 |
| walker |  | 4213 | 12 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 18, sub: 0, line: 391 } |  |  | 0.548 |
| walker |  | 4226 | 13 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 3, sub: 0, line: 47 } |  |  | 0.550 |
| walker |  | 4242 | 16 | Code::CodeKey { rung: Body, file: cmd/root.go, decl: 15, sub: 0, line: 345 } |  |  | 0.550 |
| ns | 4470 |  | 283 | Executor type + complete combinator roster | 4.1 |  | 0.534 |
| walker |  | 4823 | 581 | Code::CodeKey { rung: Decl, file: cmd/input.go, decl: 1, sub: 0, line: 10 } |  |  | 0.535 |
| walker |  | 4837 | 14 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 1, sub: 0, line: 10 } |  |  | 0.541 |
| walker |  | 4844 | 7 | Plaintext::DeclSurface { file: pkg/exprparser/testdata/for-hashing-3/nested/nested-data.txt } |  |  | 0.541 |
| walker |  | 4865 | 21 | Code::CodeKey { rung: Names, file: cmd/platforms.go, decl: 0, sub: 0, line: 0 } |  |  | 0.548 |
| walker |  | 4969 | 104 | Code::CodeKey { rung: Names, file: pkg/container/container_types.go, decl: 0, sub: 0, line: 0 } |  |  | 0.548 |
| walker |  | 4976 | 7 | Code::CodeKey { rung: Decl, file: pkg/container/container_types.go, decl: 7, sub: 0, line: 80 } |  |  | 0.548 |
| walker |  | 5005 | 29 | Code::CodeKey { rung: Decl, file: pkg/container/container_types.go, decl: 2, sub: 0, line: 36 } |  |  | 0.523 |
| ns | 5005 |  | 535 | pkg/runner: complete type roster | 4.2 |  | 0.523 |
| walker |  | 5052 | 47 | Code::CodeKey { rung: Decl, file: pkg/container/container_types.go, decl: 5, sub: 0, line: 70 } |  |  | 0.523 |
| walker |  | 5104 | 52 | Code::CodeKey { rung: Decl, file: pkg/container/container_types.go, decl: 4, sub: 0, line: 61 } |  |  | 0.523 |
| ns | 5115 |  | 110 | Runner interface and New() | 4.3 | 4.2 | 0.516 |
| walker |  | 5312 | 208 | Code::CodeKey { rung: Decl, file: pkg/container/container_types.go, decl: 1, sub: 0, line: 12 } |  |  | 0.516 |
| ns | 5503 |  | 388 | runner.Config: fields (first half, rest elided) | 4.4 | 4.2 | 0.502 |
| walker |  | 5560 | 248 | Code::CodeKey { rung: Decl, file: pkg/container/container_types.go, decl: 3, sub: 0, line: 43 } |  |  | 0.503 |
| walker |  | 5570 | 10 | Code::CodeKey { rung: Doc, file: pkg/container/container_types.go, decl: 3, sub: 0, line: 43 } |  |  | 0.503 |
| walker |  | 5584 | 14 | Code::CodeKey { rung: Doc, file: pkg/container/container_types.go, decl: 2, sub: 0, line: 36 } |  |  | 0.503 |
| walker |  | 5599 | 15 | Code::CodeKey { rung: Doc, file: pkg/container/container_types.go, decl: 1, sub: 0, line: 12 } |  |  | 0.503 |
| walker |  | 5628 | 29 | Code::CodeKey { rung: Body, file: cmd/notices.go, decl: 4, sub: 0, line: 57 } |  |  | 0.503 |
| walker |  | 5641 | 13 | Code::CodeKey { rung: Names, file: pkg/model/job_context.go, decl: 0, sub: 0, line: 0 } |  |  | 0.503 |
| walker |  | 5735 | 94 | Code::CodeKey { rung: Decl, file: pkg/model/job_context.go, decl: 1, sub: 0, line: 3 } |  |  | 0.503 |
| walker |  | 5775 | 40 | Code::CodeKey { rung: Names, file: pkg/artifactcache/model.go, decl: 0, sub: 0, line: 0 } |  |  | 0.503 |
| ns | 5805 |  | 302 | RunContext struct | 4.5 | 4.2 | 0.489 |
| walker |  | 5818 | 43 | Code::CodeKey { rung: Decl, file: pkg/artifactcache/model.go, decl: 1, sub: 0, line: 3 } |  |  | 0.489 |
| walker |  | 5965 | 147 | Code::CodeKey { rung: Decl, file: pkg/artifactcache/model.go, decl: 3, sub: 0, line: 26 } |  |  | 0.489 |
| ns | 6160 |  | 355 | stepFactory: StepType -> implementation | 4.6 | 4.2 | 0.474 |
| ns | 6438 |  | 278 | step interface and stage enum | 4.7 | 4.2 | 0.460 |
| walker |  | 6774 | 809 | GoMod::File { file: go.mod } |  |  | 0.461 |
| walker |  | 6788 | 14 | Code::CodeKey { rung: Names, file: pkg/container/executions_environment.go, decl: 0, sub: 0, line: 0 } |  |  | 0.461 |
| walker |  | 6896 | 108 | Code::CodeKey { rung: Decl, file: pkg/container/executions_environment.go, decl: 1, sub: 0, line: 5 } |  |  | 0.461 |
| walker |  | 6924 | 28 | Code::CodeKey { rung: Names, file: pkg/lookpath/error.go, decl: 0, sub: 0, line: 0 } |  |  | 0.461 |
| ns | 6933 |  | 495 | run_context.go: method roster (container lifecycle) | 4.8 |  | 0.448 |
| walker |  | 6942 | 18 | Code::CodeKey { rung: Decl, file: pkg/lookpath/error.go, decl: 1, sub: 0, line: 3 } |  |  | 0.448 |
| walker |  | 6951 | 9 | Code::CodeKey { rung: Body, file: pkg/lookpath/error.go, decl: 2, sub: 0, line: 8 } |  |  | 0.448 |
| walker |  | 6969 | 18 | Code::CodeKey { rung: Doc, file: pkg/container/container_types.go, decl: 4, sub: 0, line: 61 } |  |  | 0.448 |
| walker |  | 6987 | 18 | Code::CodeKey { rung: Doc, file: pkg/container/container_types.go, decl: 5, sub: 0, line: 70 } |  |  | 0.448 |
| walker |  | 7022 | 35 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 5, sub: 0, line: 138 } |  |  | 0.450 |
| ns | 7079 |  | 146 | ActionCache implementations | 4.9 | 4.2 | 0.442 |
| walker |  | 7153 | 131 | Code::CodeKey { rung: Names, file: pkg/model/step_result.go, decl: 0, sub: 0, line: 0 } |  |  | 0.443 |
| walker |  | 7160 | 7 | Code::CodeKey { rung: Decl, file: pkg/model/step_result.go, decl: 2, sub: 0, line: 7 } |  |  | 0.443 |
| walker |  | 7188 | 28 | Code::CodeKey { rung: Decl, file: pkg/model/step_result.go, decl: 3, sub: 0, line: 13 } |  |  | 0.443 |
| walker |  | 7237 | 49 | Code::CodeKey { rung: Decl, file: pkg/model/step_result.go, decl: 7, sub: 0, line: 41 } |  |  | 0.443 |
| ns | 7239 |  | 160 | pkg/container: complete file list | 5.1 |  | 0.470 |
| walker |  | 7249 | 12 | Code::CodeKey { rung: Body, file: pkg/model/step_result.go, decl: 4, sub: 0, line: 19 } |  |  | 0.470 |
| walker |  | 7308 | 59 | Code::CodeKey { rung: Names, file: pkg/lookpath/env.go, decl: 0, sub: 0, line: 0 } |  |  | 0.470 |
| walker |  | 7311 | 3 | Code::CodeKey { rung: Decl, file: pkg/lookpath/env.go, decl: 2, sub: 0, line: 9 } |  |  | 0.470 |
| walker |  | 7325 | 14 | Code::CodeKey { rung: Decl, file: pkg/lookpath/env.go, decl: 1, sub: 0, line: 5 } |  |  | 0.470 |
| walker |  | 7334 | 9 | Code::CodeKey { rung: Body, file: pkg/lookpath/env.go, decl: 3, sub: 0, line: 12 } |  |  | 0.470 |
| walker |  | 7348 | 14 | Code::CodeKey { rung: Body, file: pkg/lookpath/env.go, decl: 4, sub: 0, line: 16 } |  |  | 0.470 |
| ns | 7516 |  | 277 | Container interface | 5.2 |  | 0.483 |
| walker |  | 7615 | 267 | Code::CodeKey { rung: Names, file: pkg/common/draw.go, decl: 0, sub: 0, line: 0 } |  |  | 0.483 |
| walker |  | 7626 | 11 | Code::CodeKey { rung: Decl, file: pkg/common/draw.go, decl: 2, sub: 0, line: 14 } |  |  | 0.483 |
| walker |  | 7646 | 20 | Code::CodeKey { rung: Decl, file: pkg/common/draw.go, decl: 7, sub: 0, line: 59 } |  |  | 0.483 |
| ns | 7658 |  | 142 | ExecutionsEnvironment interface | 5.3 |  | 0.488 |
| walker |  | 7675 | 29 | Code::CodeKey { rung: Decl, file: pkg/common/draw.go, decl: 6, sub: 0, line: 52 } |  |  | 0.488 |
| walker |  | 7732 | 57 | Code::CodeKey { rung: Decl, file: pkg/common/draw.go, decl: 4, sub: 0, line: 35 } |  |  | 0.488 |
| walker |  | 7737 | 5 | Code::CodeKey { rung: Doc, file: pkg/common/draw.go, decl: 2, sub: 0, line: 14 } |  |  | 0.488 |
| walker |  | 7868 | 131 | Code::CodeKey { rung: Decl, file: pkg/common/draw.go, decl: 5, sub: 0, line: 44 } |  |  | 0.488 |
| walker |  | 7874 | 6 | Code::CodeKey { rung: Doc, file: pkg/common/draw.go, decl: 6, sub: 0, line: 52 } |  |  | 0.488 |
| walker |  | 7880 | 6 | Code::CodeKey { rung: Doc, file: pkg/common/draw.go, decl: 7, sub: 0, line: 59 } |  |  | 0.488 |
| walker |  | 7889 | 9 | Code::CodeKey { rung: Doc, file: pkg/common/draw.go, decl: 13, sub: 0, line: 127 } |  |  | 0.488 |
| walker |  | 7899 | 10 | Code::CodeKey { rung: Doc, file: pkg/common/draw.go, decl: 3, sub: 0, line: 22 } |  |  | 0.488 |
| walker |  | 7909 | 10 | Code::CodeKey { rung: Doc, file: pkg/common/draw.go, decl: 11, sub: 0, line: 98 } |  |  | 0.488 |
| walker |  | 7919 | 10 | Code::CodeKey { rung: Doc, file: pkg/common/draw.go, decl: 14, sub: 0, line: 141 } |  |  | 0.488 |
| ns | 7923 |  | 265 | Docker backend vs docker-less stub: the build-tag split | 5.4 |  | 0.480 |
| walker |  | 7930 | 11 | Code::CodeKey { rung: Doc, file: pkg/common/draw.go, decl: 1, sub: 0, line: 11 } |  |  | 0.480 |
| walker |  | 7941 | 11 | Code::CodeKey { rung: Doc, file: pkg/common/draw.go, decl: 12, sub: 0, line: 110 } |  |  | 0.480 |
| walker |  | 7957 | 16 | Code::CodeKey { rung: Names, file: pkg/common/outbound_ip.go, decl: 0, sub: 0, line: 0 } |  |  | 0.480 |
| ns | 8100 |  | 177 | Docker socket discovery | 5.5 |  | 0.475 |
| ns | 8190 |  | 90 | Supporting packages: complete file lists (A) | 6.1 |  | 0.489 |
| walker |  | 8231 | 274 | Code::CodeKey { rung: Names, file: pkg/model/action.go, decl: 0, sub: 0, line: 0 } |  |  | 0.491 |
| walker |  | 8259 | 28 | Code::CodeKey { rung: Decl, file: pkg/model/action.go, decl: 11, sub: 0, line: 127 } |  |  | 0.491 |
| walker |  | 8301 | 42 | Code::CodeKey { rung: Decl, file: pkg/model/action.go, decl: 10, sub: 0, line: 120 } |  |  | 0.491 |
| ns | 8349 |  | 159 | Supporting packages: complete file lists (B) | 6.2 |  | 0.512 |
| walker |  | 8397 | 96 | Code::CodeKey { rung: Decl, file: pkg/model/action.go, decl: 3, sub: 0, line: 39 } |  |  | 0.512 |
| ns | 8493 |  | 144 | Context names the expression interpreter resolves | 6.3 |  | 0.507 |
| walker |  | 8534 | 137 | Code::CodeKey { rung: Decl, file: pkg/model/action.go, decl: 8, sub: 0, line: 89 } |  |  | 0.507 |
| ns | 8618 |  | 125 | Expression functions act implements | 6.4 |  | 0.503 |
| walker |  | 8740 | 206 | Code::CodeKey { rung: Decl, file: pkg/model/action.go, decl: 7, sub: 0, line: 72 } |  |  | 0.503 |
| walker |  | 8752 | 12 | Code::CodeKey { rung: Doc, file: pkg/model/action.go, decl: 12, sub: 0, line: 133 } |  |  | 0.503 |
| walker |  | 8765 | 13 | Code::CodeKey { rung: Doc, file: pkg/model/action.go, decl: 7, sub: 0, line: 72 } |  |  | 0.503 |
| walker |  | 8782 | 17 | Code::CodeKey { rung: Doc, file: pkg/model/action.go, decl: 1, sub: 0, line: 13 } |  |  | 0.503 |
| ns | 8842 |  | 224 | Artifact cache server: routes and lifecycle | 6.5 |  | 0.498 |
| walker |  | 8896 | 114 | Code::CodeKey { rung: Names, file: pkg/common/auth.go, decl: 0, sub: 0, line: 0 } |  |  | 0.498 |
| walker |  | 8903 | 7 | Code::CodeKey { rung: Decl, file: pkg/common/auth.go, decl: 4, sub: 0, line: 33 } |  |  | 0.498 |
| walker |  | 8925 | 22 | Code::CodeKey { rung: Decl, file: pkg/common/auth.go, decl: 2, sub: 0, line: 26 } |  |  | 0.498 |
| walker |  | 8995 | 70 | Code::CodeKey { rung: Decl, file: pkg/common/auth.go, decl: 1, sub: 0, line: 17 } |  |  | 0.498 |
| ns | 9007 |  | 165 | Artifact server: Serve + the v4 route base | 6.6 |  | 0.494 |
| walker |  | 9215 | 220 | Code::CodeKey { rung: Names, file: pkg/model/workflow.go, decl: 0, sub: 0, line: 0 } |  |  | 0.502 |
| walker |  | 9228 | 13 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 13, sub: 0, line: 171 } |  |  | 0.502 |
| walker |  | 9248 | 20 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 8, sub: 0, line: 113 } |  |  | 0.502 |
| walker |  | 9274 | 26 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 11, sub: 0, line: 161 } |  |  | 0.502 |
| ns | 9312 |  | 305 | pkg/common: complete exported-function roster | 6.7 |  | 0.497 |
| walker |  | 9313 | 39 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 12, sub: 0, line: 166 } |  |  | 0.497 |
| walker |  | 9368 | 55 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 10, sub: 0, line: 154 } |  |  | 0.497 |
| walker |  | 9437 | 69 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 7, sub: 0, line: 105 } |  |  | 0.497 |
| ns | 9475 |  | 163 | pkg/common/git: repo detection and cloning | 6.8 |  | 0.492 |
| walker |  | 9524 | 87 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 1, sub: 0, line: 19 } |  |  | 0.498 |
| walker |  | 9533 | 9 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 2, sub: 0, line: 29 } |  |  | 0.498 |
| walker |  | 9551 | 18 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 1, sub: 0, line: 19 } |  |  | 0.501 |
| ns | 9601 |  | 126 | CI, issue-template and editor directories | 7.1 |  | 0.505 |
| walker |  | 9750 | 199 | Code::CodeKey { rung: Names, file: pkg/model/workflow.go, decl: 0, sub: 1, line: 0 } |  |  | 0.509 |
| walker |  | 9767 | 17 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 17, sub: 0, line: 225 } |  |  | 0.509 |
| walker |  | 9799 | 32 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 18, sub: 0, line: 230 } |  |  | 0.509 |
| ns | 9854 |  | 253 | Key direct dependencies | 7.2 |  | 0.518 |
| walker |  | 9871 | 72 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 16, sub: 0, line: 216 } |  |  | 0.518 |
| walker |  | 9881 | 10 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 16, sub: 0, line: 216 } |  |  | 0.518 |
| walker |  | 9891 | 10 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 24, sub: 0, line: 311 } |  |  | 0.518 |
| walker |  | 9998 | 107 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 15, sub: 0, line: 196 } |  |  | 0.522 |

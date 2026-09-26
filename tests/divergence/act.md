Score(3000)=0.580 I=0.870 C=0.387 ns_rows≤3K=18/47 grid(1000/1442/2080/3000/4327/6240/9000)=0.757/0.649/0.566/0.580/0.514/0.475/0.500

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
| ns | 3410 |  | 262 | Job struct: every supported job key | 3.4 | 3.1 | 0.557 |
| ns | 3634 |  | 224 | Step struct: every supported step key | 3.5 | 3.1 | 0.543 |
| walker |  | 3872 | 581 | Code::CodeKey { rung: Decl, file: cmd/input.go, decl: 1, sub: 0, line: 10 } |  |  | 0.545 |
| walker |  | 3884 | 12 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 4, sub: 0, line: 90 } |  |  | 0.545 |
| walker |  | 3897 | 13 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 3, sub: 0, line: 85 } |  |  | 0.545 |
| walker |  | 3910 | 13 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 6, sub: 0, line: 99 } |  |  | 0.545 |
| walker |  | 3924 | 14 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 1, sub: 0, line: 10 } |  |  | 0.552 |
| walker |  | 3938 | 14 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 8, sub: 0, line: 109 } |  |  | 0.552 |
| walker |  | 3953 | 15 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 7, sub: 0, line: 104 } |  |  | 0.552 |
| ns | 3966 |  | 332 | StepType constants | 3.6 | 3.1 | 0.531 |
| walker |  | 3968 | 15 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 9, sub: 0, line: 114 } |  |  | 0.531 |
| walker |  | 4019 | 51 | Code::CodeKey { rung: Names, file: cmd/secrets.go, decl: 0, sub: 0, line: 0 } |  |  | 0.535 |
| walker |  | 4026 | 7 | Code::CodeKey { rung: Body, file: cmd/secrets.go, decl: 3, sub: 0, line: 40 } |  |  | 0.535 |
| ns | 4187 |  | 221 | GithubContext: complete field-name roster | 3.7 | 3.1 | 0.513 |
| walker |  | 4294 | 268 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.513 |
| walker |  | 4312 | 18 | Code::CodeKey { rung: Names, file: cmd/graph.go, decl: 0, sub: 0, line: 0 } |  |  | 0.514 |
| walker |  | 4330 | 18 | Code::CodeKey { rung: Names, file: cmd/list.go, decl: 0, sub: 0, line: 0 } |  |  | 0.516 |
| ns | 4470 |  | 283 | Executor type + complete combinator roster | 4.1 |  | 0.500 |
| walker |  | 4704 | 374 | Code::CodeKey { rung: Names, file: cmd/root.go, decl: 0, sub: 0, line: 0 } |  |  | 0.539 |
| walker |  | 4754 | 50 | Code::CodeKey { rung: Decl, file: cmd/root.go, decl: 1, sub: 0, line: 37 } |  |  | 0.539 |
| walker |  | 4766 | 12 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 18, sub: 0, line: 391 } |  |  | 0.539 |
| walker |  | 4779 | 13 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 3, sub: 0, line: 47 } |  |  | 0.541 |
| walker |  | 4786 | 7 | Plaintext::DeclSurface { file: pkg/exprparser/testdata/for-hashing-3/nested/nested-data.txt } |  |  | 0.541 |
| walker |  | 4807 | 21 | Code::CodeKey { rung: Names, file: cmd/platforms.go, decl: 0, sub: 0, line: 0 } |  |  | 0.548 |
| walker |  | 4911 | 104 | Code::CodeKey { rung: Names, file: pkg/container/container_types.go, decl: 0, sub: 0, line: 0 } |  |  | 0.548 |
| walker |  | 4918 | 7 | Code::CodeKey { rung: Decl, file: pkg/container/container_types.go, decl: 7, sub: 0, line: 80 } |  |  | 0.548 |
| walker |  | 4947 | 29 | Code::CodeKey { rung: Decl, file: pkg/container/container_types.go, decl: 2, sub: 0, line: 36 } |  |  | 0.548 |
| walker |  | 4994 | 47 | Code::CodeKey { rung: Decl, file: pkg/container/container_types.go, decl: 5, sub: 0, line: 70 } |  |  | 0.548 |
| ns | 5005 |  | 535 | pkg/runner: complete type roster | 4.2 |  | 0.523 |
| walker |  | 5046 | 52 | Code::CodeKey { rung: Decl, file: pkg/container/container_types.go, decl: 4, sub: 0, line: 61 } |  |  | 0.523 |
| ns | 5115 |  | 110 | Runner interface and New() | 4.3 | 4.2 | 0.516 |
| walker |  | 5254 | 208 | Code::CodeKey { rung: Decl, file: pkg/container/container_types.go, decl: 1, sub: 0, line: 12 } |  |  | 0.516 |
| walker |  | 5502 | 248 | Code::CodeKey { rung: Decl, file: pkg/container/container_types.go, decl: 3, sub: 0, line: 43 } |  |  | 0.517 |
| ns | 5503 |  | 388 | runner.Config: fields (first half, rest elided) | 4.4 | 4.2 | 0.503 |
| walker |  | 5512 | 10 | Code::CodeKey { rung: Doc, file: pkg/container/container_types.go, decl: 3, sub: 0, line: 43 } |  |  | 0.503 |
| walker |  | 5526 | 14 | Code::CodeKey { rung: Doc, file: pkg/container/container_types.go, decl: 2, sub: 0, line: 36 } |  |  | 0.503 |
| walker |  | 5541 | 15 | Code::CodeKey { rung: Doc, file: pkg/container/container_types.go, decl: 1, sub: 0, line: 12 } |  |  | 0.503 |
| walker |  | 5554 | 13 | Code::CodeKey { rung: Names, file: pkg/model/job_context.go, decl: 0, sub: 0, line: 0 } |  |  | 0.503 |
| walker |  | 5648 | 94 | Code::CodeKey { rung: Decl, file: pkg/model/job_context.go, decl: 1, sub: 0, line: 3 } |  |  | 0.503 |
| walker |  | 5688 | 40 | Code::CodeKey { rung: Names, file: pkg/artifactcache/model.go, decl: 0, sub: 0, line: 0 } |  |  | 0.503 |
| walker |  | 5731 | 43 | Code::CodeKey { rung: Decl, file: pkg/artifactcache/model.go, decl: 1, sub: 0, line: 3 } |  |  | 0.503 |
| ns | 5805 |  | 302 | RunContext struct | 4.5 | 4.2 | 0.489 |
| walker |  | 5878 | 147 | Code::CodeKey { rung: Decl, file: pkg/artifactcache/model.go, decl: 3, sub: 0, line: 26 } |  |  | 0.489 |
| ns | 6160 |  | 355 | stepFactory: StepType -> implementation | 4.6 | 4.2 | 0.474 |
| ns | 6438 |  | 278 | step interface and stage enum | 4.7 | 4.2 | 0.460 |
| walker |  | 6687 | 809 | GoMod::File { file: go.mod } |  |  | 0.461 |
| walker |  | 6701 | 14 | Code::CodeKey { rung: Names, file: pkg/container/executions_environment.go, decl: 0, sub: 0, line: 0 } |  |  | 0.461 |
| walker |  | 6809 | 108 | Code::CodeKey { rung: Decl, file: pkg/container/executions_environment.go, decl: 1, sub: 0, line: 5 } |  |  | 0.461 |
| walker |  | 6837 | 28 | Code::CodeKey { rung: Names, file: pkg/lookpath/error.go, decl: 0, sub: 0, line: 0 } |  |  | 0.461 |
| walker |  | 6855 | 18 | Code::CodeKey { rung: Decl, file: pkg/lookpath/error.go, decl: 1, sub: 0, line: 3 } |  |  | 0.461 |
| walker |  | 6864 | 9 | Code::CodeKey { rung: Body, file: pkg/lookpath/error.go, decl: 2, sub: 0, line: 8 } |  |  | 0.461 |
| walker |  | 6882 | 18 | Code::CodeKey { rung: Doc, file: pkg/container/container_types.go, decl: 4, sub: 0, line: 61 } |  |  | 0.461 |
| walker |  | 6900 | 18 | Code::CodeKey { rung: Doc, file: pkg/container/container_types.go, decl: 5, sub: 0, line: 70 } |  |  | 0.461 |
| ns | 6933 |  | 495 | run_context.go: method roster (container lifecycle) | 4.8 |  | 0.448 |
| walker |  | 6935 | 35 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 5, sub: 0, line: 138 } |  |  | 0.450 |
| walker |  | 7066 | 131 | Code::CodeKey { rung: Names, file: pkg/model/step_result.go, decl: 0, sub: 0, line: 0 } |  |  | 0.451 |
| walker |  | 7073 | 7 | Code::CodeKey { rung: Decl, file: pkg/model/step_result.go, decl: 2, sub: 0, line: 7 } |  |  | 0.451 |
| ns | 7079 |  | 146 | ActionCache implementations | 4.9 | 4.2 | 0.443 |
| walker |  | 7101 | 28 | Code::CodeKey { rung: Decl, file: pkg/model/step_result.go, decl: 3, sub: 0, line: 13 } |  |  | 0.443 |
| walker |  | 7150 | 49 | Code::CodeKey { rung: Decl, file: pkg/model/step_result.go, decl: 7, sub: 0, line: 41 } |  |  | 0.443 |
| walker |  | 7209 | 59 | Code::CodeKey { rung: Names, file: pkg/lookpath/env.go, decl: 0, sub: 0, line: 0 } |  |  | 0.443 |
| walker |  | 7212 | 3 | Code::CodeKey { rung: Decl, file: pkg/lookpath/env.go, decl: 2, sub: 0, line: 9 } |  |  | 0.443 |
| walker |  | 7226 | 14 | Code::CodeKey { rung: Decl, file: pkg/lookpath/env.go, decl: 1, sub: 0, line: 5 } |  |  | 0.443 |
| walker |  | 7235 | 9 | Code::CodeKey { rung: Body, file: pkg/lookpath/env.go, decl: 3, sub: 0, line: 12 } |  |  | 0.443 |
| ns | 7239 |  | 160 | pkg/container: complete file list | 5.1 |  | 0.470 |
| walker |  | 7249 | 14 | Code::CodeKey { rung: Body, file: pkg/lookpath/env.go, decl: 4, sub: 0, line: 16 } |  |  | 0.470 |
| walker |  | 7516 | 267 | Code::CodeKey { rung: Names, file: pkg/common/draw.go, decl: 0, sub: 0, line: 0 } |  |  | 0.483 |
| ns | 7516 |  | 277 | Container interface | 5.2 |  | 0.483 |
| walker |  | 7527 | 11 | Code::CodeKey { rung: Decl, file: pkg/common/draw.go, decl: 2, sub: 0, line: 14 } |  |  | 0.483 |
| walker |  | 7547 | 20 | Code::CodeKey { rung: Decl, file: pkg/common/draw.go, decl: 7, sub: 0, line: 59 } |  |  | 0.483 |
| walker |  | 7576 | 29 | Code::CodeKey { rung: Decl, file: pkg/common/draw.go, decl: 6, sub: 0, line: 52 } |  |  | 0.483 |
| walker |  | 7633 | 57 | Code::CodeKey { rung: Decl, file: pkg/common/draw.go, decl: 4, sub: 0, line: 35 } |  |  | 0.483 |
| walker |  | 7638 | 5 | Code::CodeKey { rung: Doc, file: pkg/common/draw.go, decl: 2, sub: 0, line: 14 } |  |  | 0.483 |
| ns | 7658 |  | 142 | ExecutionsEnvironment interface | 5.3 |  | 0.488 |
| walker |  | 7769 | 131 | Code::CodeKey { rung: Decl, file: pkg/common/draw.go, decl: 5, sub: 0, line: 44 } |  |  | 0.488 |
| walker |  | 7775 | 6 | Code::CodeKey { rung: Doc, file: pkg/common/draw.go, decl: 6, sub: 0, line: 52 } |  |  | 0.488 |
| walker |  | 7781 | 6 | Code::CodeKey { rung: Doc, file: pkg/common/draw.go, decl: 7, sub: 0, line: 59 } |  |  | 0.488 |
| walker |  | 7790 | 9 | Code::CodeKey { rung: Doc, file: pkg/common/draw.go, decl: 13, sub: 0, line: 127 } |  |  | 0.488 |
| walker |  | 7800 | 10 | Code::CodeKey { rung: Doc, file: pkg/common/draw.go, decl: 3, sub: 0, line: 22 } |  |  | 0.488 |
| walker |  | 7810 | 10 | Code::CodeKey { rung: Doc, file: pkg/common/draw.go, decl: 11, sub: 0, line: 98 } |  |  | 0.488 |
| walker |  | 7820 | 10 | Code::CodeKey { rung: Doc, file: pkg/common/draw.go, decl: 14, sub: 0, line: 141 } |  |  | 0.488 |
| walker |  | 7831 | 11 | Code::CodeKey { rung: Doc, file: pkg/common/draw.go, decl: 1, sub: 0, line: 11 } |  |  | 0.488 |
| walker |  | 7842 | 11 | Code::CodeKey { rung: Doc, file: pkg/common/draw.go, decl: 12, sub: 0, line: 110 } |  |  | 0.488 |
| walker |  | 7858 | 16 | Code::CodeKey { rung: Names, file: pkg/common/outbound_ip.go, decl: 0, sub: 0, line: 0 } |  |  | 0.488 |
| ns | 7923 |  | 265 | Docker backend vs docker-less stub: the build-tag split | 5.4 |  | 0.480 |
| ns | 8100 |  | 177 | Docker socket discovery | 5.5 |  | 0.475 |
| walker |  | 8132 | 274 | Code::CodeKey { rung: Names, file: pkg/model/action.go, decl: 0, sub: 0, line: 0 } |  |  | 0.477 |
| walker |  | 8160 | 28 | Code::CodeKey { rung: Decl, file: pkg/model/action.go, decl: 11, sub: 0, line: 127 } |  |  | 0.477 |
| ns | 8190 |  | 90 | Supporting packages: complete file lists (A) | 6.1 |  | 0.491 |
| walker |  | 8202 | 42 | Code::CodeKey { rung: Decl, file: pkg/model/action.go, decl: 10, sub: 0, line: 120 } |  |  | 0.491 |
| walker |  | 8298 | 96 | Code::CodeKey { rung: Decl, file: pkg/model/action.go, decl: 3, sub: 0, line: 39 } |  |  | 0.491 |
| ns | 8349 |  | 159 | Supporting packages: complete file lists (B) | 6.2 |  | 0.512 |
| walker |  | 8435 | 137 | Code::CodeKey { rung: Decl, file: pkg/model/action.go, decl: 8, sub: 0, line: 89 } |  |  | 0.512 |
| ns | 8493 |  | 144 | Context names the expression interpreter resolves | 6.3 |  | 0.507 |
| ns | 8618 |  | 125 | Expression functions act implements | 6.4 |  | 0.503 |
| walker |  | 8641 | 206 | Code::CodeKey { rung: Decl, file: pkg/model/action.go, decl: 7, sub: 0, line: 72 } |  |  | 0.503 |
| walker |  | 8653 | 12 | Code::CodeKey { rung: Doc, file: pkg/model/action.go, decl: 12, sub: 0, line: 133 } |  |  | 0.503 |
| walker |  | 8666 | 13 | Code::CodeKey { rung: Doc, file: pkg/model/action.go, decl: 7, sub: 0, line: 72 } |  |  | 0.503 |
| walker |  | 8683 | 17 | Code::CodeKey { rung: Doc, file: pkg/model/action.go, decl: 1, sub: 0, line: 13 } |  |  | 0.503 |
| walker |  | 8797 | 114 | Code::CodeKey { rung: Names, file: pkg/common/auth.go, decl: 0, sub: 0, line: 0 } |  |  | 0.503 |
| walker |  | 8804 | 7 | Code::CodeKey { rung: Decl, file: pkg/common/auth.go, decl: 4, sub: 0, line: 33 } |  |  | 0.503 |
| walker |  | 8826 | 22 | Code::CodeKey { rung: Decl, file: pkg/common/auth.go, decl: 2, sub: 0, line: 26 } |  |  | 0.503 |
| ns | 8842 |  | 224 | Artifact cache server: routes and lifecycle | 6.5 |  | 0.498 |
| walker |  | 8896 | 70 | Code::CodeKey { rung: Decl, file: pkg/common/auth.go, decl: 1, sub: 0, line: 17 } |  |  | 0.498 |
| ns | 9007 |  | 165 | Artifact server: Serve + the v4 route base | 6.6 |  | 0.494 |
| walker |  | 9116 | 220 | Code::CodeKey { rung: Names, file: pkg/model/workflow.go, decl: 0, sub: 0, line: 0 } |  |  | 0.502 |
| walker |  | 9129 | 13 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 13, sub: 0, line: 171 } |  |  | 0.502 |
| walker |  | 9149 | 20 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 8, sub: 0, line: 113 } |  |  | 0.502 |
| walker |  | 9175 | 26 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 11, sub: 0, line: 161 } |  |  | 0.502 |
| walker |  | 9214 | 39 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 12, sub: 0, line: 166 } |  |  | 0.502 |
| walker |  | 9269 | 55 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 10, sub: 0, line: 154 } |  |  | 0.502 |
| ns | 9312 |  | 305 | pkg/common: complete exported-function roster | 6.7 |  | 0.497 |
| walker |  | 9338 | 69 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 7, sub: 0, line: 105 } |  |  | 0.497 |
| walker |  | 9425 | 87 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 1, sub: 0, line: 19 } |  |  | 0.503 |
| walker |  | 9434 | 9 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 2, sub: 0, line: 29 } |  |  | 0.503 |
| walker |  | 9452 | 18 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 1, sub: 0, line: 19 } |  |  | 0.506 |
| ns | 9475 |  | 163 | pkg/common/git: repo detection and cloning | 6.8 |  | 0.501 |
| ns | 9601 |  | 126 | CI, issue-template and editor directories | 7.1 |  | 0.505 |
| walker |  | 9651 | 199 | Code::CodeKey { rung: Names, file: pkg/model/workflow.go, decl: 0, sub: 1, line: 0 } |  |  | 0.509 |
| walker |  | 9668 | 17 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 17, sub: 0, line: 225 } |  |  | 0.509 |
| walker |  | 9700 | 32 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 18, sub: 0, line: 230 } |  |  | 0.509 |
| walker |  | 9772 | 72 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 16, sub: 0, line: 216 } |  |  | 0.509 |
| ns | 9854 |  | 253 | Key direct dependencies | 7.2 |  | 0.518 |
| walker |  | 9988 | 216 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 15, sub: 0, line: 196 } |  |  | 0.530 |

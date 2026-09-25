Score(3000)=0.583 I=0.870 C=0.391 ns_rows≤3K=18/47 grid(1000/1442/2080/3000/4327/6240/9000)=0.757/0.649/0.566/0.583/0.550/0.474/0.498

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
| walker |  | 704 | 9 | Fs::DirListing { dir: pkg/common/git } |  |  | 0.555 |
| walker |  | 760 | 56 | Fs::DirListing { dir: pkg/model } |  |  | 0.626 |
| ns | 762 |  | 171 | README: how act works, end to end | 1.8 |  | 0.623 |
| walker |  | 768 | 8 | Fs::DirListing { dir: .vscode } |  |  | 0.623 |
| ns | 804 |  | 42 | Module path, Go version, released version | 1.9 |  | 0.621 |
| ns | 919 |  | 115 | cmd.Execute: CLI entry | 2.1 |  | 0.590 |
| walker |  | 954 | 186 | Fs::DirListing { dir: pkg/runner } |  |  | 0.757 |
| walker |  | 958 | 4 | Fs::DirListing { dir: pkg/runner/hashfiles } |  |  | 0.757 |
| walker |  | 962 | 4 | Fs::DirListing { dir: pkg/runner/res } |  |  | 0.757 |
| walker |  | 1122 | 160 | Fs::DirListing { dir: pkg/container } |  |  | 0.761 |
| ns | 1131 |  | 212 | Root cobra.Command definition | 2.2 |  | 0.725 |
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
| walker |  | 1382 | 26 | Fs::DirListing { dir: .github/workflows } |  |  | 0.727 |
| walker |  | 1419 | 37 | Fs::DirListing { dir: pkg/container/testdata } |  |  | 0.727 |
| walker |  | 1423 | 4 | Fs::DirListing { dir: pkg/container/testdata/docker-pull-options } |  |  | 0.727 |
| walker |  | 1427 | 4 | Fs::DirListing { dir: pkg/container/testdata/scratch } |  |  | 0.727 |
| ns | 1432 |  | 301 | Command-local flags: names and shorthands | 2.3 |  | 0.649 |
| walker |  | 1451 | 24 | Fs::DirListing { dir: pkg/model/testdata/invalid-job-name } |  |  | 0.649 |
| ns | 1782 |  | 350 | Persistent flags: names and shorthands | 2.4 |  | 0.583 |
| ns | 1928 |  | 146 | Input struct + path accessor roster | 2.5 |  | 0.566 |
| walker |  | 2166 | 715 | Fs::DirListing { dir: pkg/runner/testdata } |  |  | 0.566 |
| walker |  | 2191 | 25 | Code::CodeKey { rung: Names, file: main.go, decl: 0, sub: 0, line: 0 } |  |  | 0.576 |
| ns | 2206 |  | 278 | Config-file discovery: .actrc locations | 2.6 |  | 0.545 |
| walker |  | 2242 | 51 | Code::CodeKey { rung: Decl, file: main.go, decl: 2, sub: 0, line: 13 } |  |  | 0.633 |
| walker |  | 2252 | 10 | Code::CodeKey { rung: Doc, file: main.go, decl: 1, sub: 0, line: 11 } |  |  | 0.667 |
| walker |  | 2519 | 267 | Plaintext::Whole { file: Makefile } |  |  | 0.667 |
| ns | 2522 |  | 316 | cmd package: complete function roster | 2.7 |  | 0.626 |
| walker |  | 2525 | 6 | Plaintext::DeclSurface { file: pkg/exprparser/testdata/for-hashing-1.txt } |  |  | 0.626 |
| walker |  | 2531 | 6 | Plaintext::DeclSurface { file: pkg/exprparser/testdata/for-hashing-2.txt } |  |  | 0.626 |
| walker |  | 2665 | 134 | Code::CodeKey { rung: ModuleDoc, file: pkg/artifactcache/doc.go, decl: 0, sub: 0, line: 0 } |  |  | 0.626 |
| walker |  | 2699 | 34 | Code::CodeKey { rung: Names, file: cmd/dir.go, decl: 0, sub: 0, line: 0 } |  |  | 0.627 |
| walker |  | 2708 | 9 | Code::CodeKey { rung: Decl, file: cmd/dir.go, decl: 1, sub: 0, line: 10 } |  |  | 0.627 |
| walker |  | 2715 | 7 | Plaintext::DeclSurface { file: pkg/container/testdata/scratch/test.txt } |  |  | 0.627 |
| walker |  | 2782 | 67 | Plaintext::Whole { file: pkg/container/testdata/Dockerfile } |  |  | 0.627 |
| walker |  | 2790 | 8 | Plaintext::DeclSurface { file: pkg/exprparser/testdata/for-hashing-3/data.txt } |  |  | 0.627 |
| ns | 2869 |  | 347 | pkg/model: complete type roster | 3.1 |  | 0.588 |
| walker |  | 2937 | 147 | Code::CodeKey { rung: Names, file: cmd/notices.go, decl: 0, sub: 0, line: 0 } |  |  | 0.594 |
| walker |  | 2964 | 27 | Code::CodeKey { rung: Decl, file: cmd/notices.go, decl: 1, sub: 0, line: 17 } |  |  | 0.594 |
| ns | 2979 |  | 110 | Workflow struct | 3.2 | 3.1 | 0.582 |
| walker |  | 3114 | 150 | Code::CodeKey { rung: Names, file: cmd/input.go, decl: 0, sub: 0, line: 0 } |  |  | 0.602 |
| walker |  | 3123 | 9 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 6, sub: 0, line: 99 } |  |  | 0.602 |
| walker |  | 3134 | 11 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 3, sub: 0, line: 85 } |  |  | 0.602 |
| walker |  | 3145 | 11 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 4, sub: 0, line: 90 } |  |  | 0.602 |
| walker |  | 3156 | 11 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 5, sub: 0, line: 94 } |  |  | 0.602 |
| walker |  | 3168 | 12 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 4, sub: 0, line: 90 } |  |  | 0.602 |
| walker |  | 3181 | 13 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 3, sub: 0, line: 85 } |  |  | 0.602 |
| walker |  | 3194 | 13 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 6, sub: 0, line: 99 } |  |  | 0.602 |
| walker |  | 3208 | 14 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 8, sub: 0, line: 109 } |  |  | 0.602 |
| ns | 3209 |  | 230 | Planner interface + Plan/Stage/Run | 3.3 | 3.1 | 0.573 |
| walker |  | 3223 | 15 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 7, sub: 0, line: 104 } |  |  | 0.573 |
| walker |  | 3238 | 15 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 9, sub: 0, line: 114 } |  |  | 0.573 |
| walker |  | 3289 | 51 | Code::CodeKey { rung: Names, file: cmd/secrets.go, decl: 0, sub: 0, line: 0 } |  |  | 0.577 |
| walker |  | 3296 | 7 | Code::CodeKey { rung: Body, file: cmd/secrets.go, decl: 3, sub: 0, line: 40 } |  |  | 0.577 |
| walker |  | 3314 | 18 | Code::CodeKey { rung: Names, file: cmd/graph.go, decl: 0, sub: 0, line: 0 } |  |  | 0.579 |
| walker |  | 3332 | 18 | Code::CodeKey { rung: Names, file: cmd/list.go, decl: 0, sub: 0, line: 0 } |  |  | 0.580 |
| ns | 3471 |  | 262 | Job struct: every supported job key | 3.4 | 3.1 | 0.562 |
| ns | 3695 |  | 224 | Step struct: every supported step key | 3.5 | 3.1 | 0.548 |
| walker |  | 3706 | 374 | Code::CodeKey { rung: Names, file: cmd/root.go, decl: 0, sub: 0, line: 0 } |  |  | 0.592 |
| walker |  | 3756 | 50 | Code::CodeKey { rung: Decl, file: cmd/root.go, decl: 1, sub: 0, line: 37 } |  |  | 0.592 |
| walker |  | 3768 | 12 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 18, sub: 0, line: 391 } |  |  | 0.592 |
| walker |  | 3781 | 13 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 3, sub: 0, line: 47 } |  |  | 0.594 |
| walker |  | 3797 | 16 | Code::CodeKey { rung: Body, file: cmd/root.go, decl: 15, sub: 0, line: 345 } |  |  | 0.594 |
| ns | 4027 |  | 332 | StepType constants | 3.6 | 3.1 | 0.572 |
| walker |  | 4088 | 291 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.572 |
| ns | 4248 |  | 221 | GithubContext: complete field-name roster | 3.7 | 3.1 | 0.548 |
| walker |  | 4326 | 238 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.550 |
| ns | 4531 |  | 283 | Executor type + complete combinator roster | 4.1 |  | 0.534 |
| walker |  | 4907 | 581 | Code::CodeKey { rung: Decl, file: cmd/input.go, decl: 1, sub: 0, line: 10 } |  |  | 0.535 |
| walker |  | 4921 | 14 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 1, sub: 0, line: 10 } |  |  | 0.541 |
| walker |  | 4928 | 7 | Plaintext::DeclSurface { file: pkg/exprparser/testdata/for-hashing-3/nested/nested-data.txt } |  |  | 0.541 |
| walker |  | 4949 | 21 | Code::CodeKey { rung: Names, file: cmd/platforms.go, decl: 0, sub: 0, line: 0 } |  |  | 0.548 |
| walker |  | 5053 | 104 | Code::CodeKey { rung: Names, file: pkg/container/container_types.go, decl: 0, sub: 0, line: 0 } |  |  | 0.548 |
| walker |  | 5060 | 7 | Code::CodeKey { rung: Decl, file: pkg/container/container_types.go, decl: 7, sub: 0, line: 80 } |  |  | 0.548 |
| ns | 5066 |  | 535 | pkg/runner: complete type roster | 4.2 |  | 0.523 |
| walker |  | 5089 | 29 | Code::CodeKey { rung: Decl, file: pkg/container/container_types.go, decl: 2, sub: 0, line: 36 } |  |  | 0.523 |
| walker |  | 5136 | 47 | Code::CodeKey { rung: Decl, file: pkg/container/container_types.go, decl: 5, sub: 0, line: 70 } |  |  | 0.523 |
| ns | 5176 |  | 110 | Runner interface and New() | 4.3 | 4.2 | 0.516 |
| walker |  | 5188 | 52 | Code::CodeKey { rung: Decl, file: pkg/container/container_types.go, decl: 4, sub: 0, line: 61 } |  |  | 0.516 |
| walker |  | 5396 | 208 | Code::CodeKey { rung: Decl, file: pkg/container/container_types.go, decl: 1, sub: 0, line: 12 } |  |  | 0.516 |
| ns | 5564 |  | 388 | runner.Config: fields (first half, rest elided) | 4.4 | 4.2 | 0.502 |
| walker |  | 5644 | 248 | Code::CodeKey { rung: Decl, file: pkg/container/container_types.go, decl: 3, sub: 0, line: 43 } |  |  | 0.503 |
| walker |  | 5654 | 10 | Code::CodeKey { rung: Doc, file: pkg/container/container_types.go, decl: 3, sub: 0, line: 43 } |  |  | 0.503 |
| walker |  | 5668 | 14 | Code::CodeKey { rung: Doc, file: pkg/container/container_types.go, decl: 2, sub: 0, line: 36 } |  |  | 0.503 |
| walker |  | 5683 | 15 | Code::CodeKey { rung: Doc, file: pkg/container/container_types.go, decl: 1, sub: 0, line: 12 } |  |  | 0.503 |
| walker |  | 5712 | 29 | Code::CodeKey { rung: Body, file: cmd/notices.go, decl: 4, sub: 0, line: 57 } |  |  | 0.503 |
| walker |  | 5725 | 13 | Code::CodeKey { rung: Names, file: pkg/model/job_context.go, decl: 0, sub: 0, line: 0 } |  |  | 0.503 |
| walker |  | 5819 | 94 | Code::CodeKey { rung: Decl, file: pkg/model/job_context.go, decl: 1, sub: 0, line: 3 } |  |  | 0.503 |
| walker |  | 5859 | 40 | Code::CodeKey { rung: Names, file: pkg/artifactcache/model.go, decl: 0, sub: 0, line: 0 } |  |  | 0.503 |
| ns | 5866 |  | 302 | RunContext struct | 4.5 | 4.2 | 0.489 |
| walker |  | 5902 | 43 | Code::CodeKey { rung: Decl, file: pkg/artifactcache/model.go, decl: 1, sub: 0, line: 3 } |  |  | 0.489 |
| walker |  | 6049 | 147 | Code::CodeKey { rung: Decl, file: pkg/artifactcache/model.go, decl: 3, sub: 0, line: 26 } |  |  | 0.489 |
| ns | 6221 |  | 355 | stepFactory: StepType -> implementation | 4.6 | 4.2 | 0.474 |
| ns | 6499 |  | 278 | step interface and stage enum | 4.7 | 4.2 | 0.460 |
| walker |  | 6858 | 809 | GoMod::File { file: go.mod } |  |  | 0.461 |
| walker |  | 6872 | 14 | Code::CodeKey { rung: Names, file: pkg/container/executions_environment.go, decl: 0, sub: 0, line: 0 } |  |  | 0.461 |
| walker |  | 6980 | 108 | Code::CodeKey { rung: Decl, file: pkg/container/executions_environment.go, decl: 1, sub: 0, line: 5 } |  |  | 0.461 |
| ns | 6994 |  | 495 | run_context.go: method roster (container lifecycle) | 4.8 |  | 0.448 |
| walker |  | 7008 | 28 | Code::CodeKey { rung: Names, file: pkg/lookpath/error.go, decl: 0, sub: 0, line: 0 } |  |  | 0.448 |
| walker |  | 7026 | 18 | Code::CodeKey { rung: Decl, file: pkg/lookpath/error.go, decl: 1, sub: 0, line: 3 } |  |  | 0.448 |
| walker |  | 7035 | 9 | Code::CodeKey { rung: Body, file: pkg/lookpath/error.go, decl: 2, sub: 0, line: 8 } |  |  | 0.448 |
| walker |  | 7053 | 18 | Code::CodeKey { rung: Doc, file: pkg/container/container_types.go, decl: 4, sub: 0, line: 61 } |  |  | 0.448 |
| walker |  | 7071 | 18 | Code::CodeKey { rung: Doc, file: pkg/container/container_types.go, decl: 5, sub: 0, line: 70 } |  |  | 0.448 |
| walker |  | 7106 | 35 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 5, sub: 0, line: 138 } |  |  | 0.450 |
| ns | 7140 |  | 146 | ActionCache implementations | 4.9 | 4.2 | 0.442 |
| walker |  | 7237 | 131 | Code::CodeKey { rung: Names, file: pkg/model/step_result.go, decl: 0, sub: 0, line: 0 } |  |  | 0.443 |
| walker |  | 7244 | 7 | Code::CodeKey { rung: Decl, file: pkg/model/step_result.go, decl: 2, sub: 0, line: 7 } |  |  | 0.443 |
| walker |  | 7272 | 28 | Code::CodeKey { rung: Decl, file: pkg/model/step_result.go, decl: 3, sub: 0, line: 13 } |  |  | 0.443 |
| ns | 7300 |  | 160 | pkg/container: complete file list | 5.1 |  | 0.470 |
| walker |  | 7321 | 49 | Code::CodeKey { rung: Decl, file: pkg/model/step_result.go, decl: 7, sub: 0, line: 41 } |  |  | 0.470 |
| walker |  | 7333 | 12 | Code::CodeKey { rung: Body, file: pkg/model/step_result.go, decl: 4, sub: 0, line: 19 } |  |  | 0.470 |
| walker |  | 7392 | 59 | Code::CodeKey { rung: Names, file: pkg/lookpath/env.go, decl: 0, sub: 0, line: 0 } |  |  | 0.470 |
| walker |  | 7395 | 3 | Code::CodeKey { rung: Decl, file: pkg/lookpath/env.go, decl: 2, sub: 0, line: 9 } |  |  | 0.470 |
| walker |  | 7409 | 14 | Code::CodeKey { rung: Decl, file: pkg/lookpath/env.go, decl: 1, sub: 0, line: 5 } |  |  | 0.470 |
| walker |  | 7418 | 9 | Code::CodeKey { rung: Body, file: pkg/lookpath/env.go, decl: 3, sub: 0, line: 12 } |  |  | 0.470 |
| walker |  | 7432 | 14 | Code::CodeKey { rung: Body, file: pkg/lookpath/env.go, decl: 4, sub: 0, line: 16 } |  |  | 0.470 |
| ns | 7577 |  | 277 | Container interface | 5.2 |  | 0.483 |
| walker |  | 7699 | 267 | Code::CodeKey { rung: Names, file: pkg/common/draw.go, decl: 0, sub: 0, line: 0 } |  |  | 0.483 |
| walker |  | 7710 | 11 | Code::CodeKey { rung: Decl, file: pkg/common/draw.go, decl: 2, sub: 0, line: 14 } |  |  | 0.483 |
| ns | 7719 |  | 142 | ExecutionsEnvironment interface | 5.3 |  | 0.488 |
| walker |  | 7730 | 20 | Code::CodeKey { rung: Decl, file: pkg/common/draw.go, decl: 7, sub: 0, line: 59 } |  |  | 0.488 |
| walker |  | 7759 | 29 | Code::CodeKey { rung: Decl, file: pkg/common/draw.go, decl: 6, sub: 0, line: 52 } |  |  | 0.488 |
| walker |  | 7816 | 57 | Code::CodeKey { rung: Decl, file: pkg/common/draw.go, decl: 4, sub: 0, line: 35 } |  |  | 0.488 |
| walker |  | 7821 | 5 | Code::CodeKey { rung: Doc, file: pkg/common/draw.go, decl: 2, sub: 0, line: 14 } |  |  | 0.488 |
| walker |  | 7952 | 131 | Code::CodeKey { rung: Decl, file: pkg/common/draw.go, decl: 5, sub: 0, line: 44 } |  |  | 0.488 |
| walker |  | 7958 | 6 | Code::CodeKey { rung: Doc, file: pkg/common/draw.go, decl: 6, sub: 0, line: 52 } |  |  | 0.488 |
| walker |  | 7964 | 6 | Code::CodeKey { rung: Doc, file: pkg/common/draw.go, decl: 7, sub: 0, line: 59 } |  |  | 0.488 |
| walker |  | 7973 | 9 | Code::CodeKey { rung: Doc, file: pkg/common/draw.go, decl: 13, sub: 0, line: 127 } |  |  | 0.488 |
| walker |  | 7983 | 10 | Code::CodeKey { rung: Doc, file: pkg/common/draw.go, decl: 3, sub: 0, line: 22 } |  |  | 0.488 |
| ns | 7984 |  | 265 | Docker backend vs docker-less stub: the build-tag split | 5.4 |  | 0.480 |
| walker |  | 7993 | 10 | Code::CodeKey { rung: Doc, file: pkg/common/draw.go, decl: 11, sub: 0, line: 98 } |  |  | 0.480 |
| walker |  | 8003 | 10 | Code::CodeKey { rung: Doc, file: pkg/common/draw.go, decl: 14, sub: 0, line: 141 } |  |  | 0.480 |
| walker |  | 8014 | 11 | Code::CodeKey { rung: Doc, file: pkg/common/draw.go, decl: 1, sub: 0, line: 11 } |  |  | 0.480 |
| walker |  | 8025 | 11 | Code::CodeKey { rung: Doc, file: pkg/common/draw.go, decl: 12, sub: 0, line: 110 } |  |  | 0.480 |
| walker |  | 8041 | 16 | Code::CodeKey { rung: Names, file: pkg/common/outbound_ip.go, decl: 0, sub: 0, line: 0 } |  |  | 0.480 |
| ns | 8161 |  | 177 | Docker socket discovery | 5.5 |  | 0.475 |
| ns | 8251 |  | 90 | Supporting packages: complete file lists (A) | 6.1 |  | 0.489 |
| walker |  | 8315 | 274 | Code::CodeKey { rung: Names, file: pkg/model/action.go, decl: 0, sub: 0, line: 0 } |  |  | 0.491 |
| walker |  | 8343 | 28 | Code::CodeKey { rung: Decl, file: pkg/model/action.go, decl: 11, sub: 0, line: 127 } |  |  | 0.491 |
| walker |  | 8385 | 42 | Code::CodeKey { rung: Decl, file: pkg/model/action.go, decl: 10, sub: 0, line: 120 } |  |  | 0.491 |
| ns | 8410 |  | 159 | Supporting packages: complete file lists (B) | 6.2 |  | 0.512 |
| walker |  | 8481 | 96 | Code::CodeKey { rung: Decl, file: pkg/model/action.go, decl: 3, sub: 0, line: 39 } |  |  | 0.512 |
| ns | 8554 |  | 144 | Context names the expression interpreter resolves | 6.3 |  | 0.507 |
| walker |  | 8618 | 137 | Code::CodeKey { rung: Decl, file: pkg/model/action.go, decl: 8, sub: 0, line: 89 } |  |  | 0.507 |
| ns | 8679 |  | 125 | Expression functions act implements | 6.4 |  | 0.503 |
| walker |  | 8824 | 206 | Code::CodeKey { rung: Decl, file: pkg/model/action.go, decl: 7, sub: 0, line: 72 } |  |  | 0.503 |
| walker |  | 8836 | 12 | Code::CodeKey { rung: Doc, file: pkg/model/action.go, decl: 12, sub: 0, line: 133 } |  |  | 0.503 |
| walker |  | 8849 | 13 | Code::CodeKey { rung: Doc, file: pkg/model/action.go, decl: 7, sub: 0, line: 72 } |  |  | 0.503 |
| walker |  | 8866 | 17 | Code::CodeKey { rung: Doc, file: pkg/model/action.go, decl: 1, sub: 0, line: 13 } |  |  | 0.503 |
| ns | 8903 |  | 224 | Artifact cache server: routes and lifecycle | 6.5 |  | 0.498 |
| walker |  | 8980 | 114 | Code::CodeKey { rung: Names, file: pkg/common/auth.go, decl: 0, sub: 0, line: 0 } |  |  | 0.498 |
| walker |  | 8987 | 7 | Code::CodeKey { rung: Decl, file: pkg/common/auth.go, decl: 4, sub: 0, line: 33 } |  |  | 0.498 |
| walker |  | 9009 | 22 | Code::CodeKey { rung: Decl, file: pkg/common/auth.go, decl: 2, sub: 0, line: 26 } |  |  | 0.498 |
| ns | 9068 |  | 165 | Artifact server: Serve + the v4 route base | 6.6 |  | 0.494 |
| walker |  | 9079 | 70 | Code::CodeKey { rung: Decl, file: pkg/common/auth.go, decl: 1, sub: 0, line: 17 } |  |  | 0.494 |
| walker |  | 9299 | 220 | Code::CodeKey { rung: Names, file: pkg/model/workflow.go, decl: 0, sub: 0, line: 0 } |  |  | 0.502 |
| walker |  | 9312 | 13 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 13, sub: 0, line: 171 } |  |  | 0.502 |
| walker |  | 9332 | 20 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 8, sub: 0, line: 113 } |  |  | 0.502 |
| walker |  | 9358 | 26 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 11, sub: 0, line: 161 } |  |  | 0.502 |
| ns | 9373 |  | 305 | pkg/common: complete exported-function roster | 6.7 |  | 0.497 |
| walker |  | 9397 | 39 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 12, sub: 0, line: 166 } |  |  | 0.497 |
| walker |  | 9452 | 55 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 10, sub: 0, line: 154 } |  |  | 0.497 |
| walker |  | 9521 | 69 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 7, sub: 0, line: 105 } |  |  | 0.497 |
| ns | 9536 |  | 163 | pkg/common/git: repo detection and cloning | 6.8 |  | 0.492 |
| walker |  | 9608 | 87 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 1, sub: 0, line: 19 } |  |  | 0.498 |
| walker |  | 9617 | 9 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 2, sub: 0, line: 29 } |  |  | 0.498 |
| walker |  | 9635 | 18 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 1, sub: 0, line: 19 } |  |  | 0.501 |
| ns | 9662 |  | 126 | CI, issue-template and editor directories | 7.1 |  | 0.505 |
| walker |  | 9834 | 199 | Code::CodeKey { rung: Names, file: pkg/model/workflow.go, decl: 0, sub: 1, line: 0 } |  |  | 0.509 |
| walker |  | 9851 | 17 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 17, sub: 0, line: 225 } |  |  | 0.509 |
| walker |  | 9883 | 32 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 18, sub: 0, line: 230 } |  |  | 0.509 |
| ns | 9915 |  | 253 | Key direct dependencies | 7.2 |  | 0.518 |
| walker |  | 9955 | 72 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 16, sub: 0, line: 216 } |  |  | 0.518 |
| walker |  | 9965 | 10 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 16, sub: 0, line: 216 } |  |  | 0.518 |
| walker |  | 9975 | 10 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 24, sub: 0, line: 311 } |  |  | 0.518 |
| walker |  | 9988 | 13 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 15, sub: 0, line: 196 } |  |  | 0.519 |

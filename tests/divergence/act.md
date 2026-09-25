Score(3000)=0.578 I=0.869 C=0.384 ns_rows≤3K=18/47 grid(1000/1442/2080/3000/4327/6240/9000)=0.589/0.649/0.566/0.578/0.552/0.475/0.497

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 49 |  | 49 | README tagline + what act is | 1.1 |  | 0.000 |
| walker |  | 127 | 127 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 135 |  | 86 | main.go: process entry point | 1.2 |  | 0.000 |
| walker |  | 159 | 32 | GoMod::Identity { file: go.mod } |  |  | 0.000 |
| ns | 176 |  | 41 | pkg/: complete package list | 1.3 |  | 0.000 |
| walker |  | 200 | 41 | Fs::DirListing { dir: pkg } |  |  | 0.421 |
| walker |  | 216 | 16 | Fs::DirListing { dir: pkg/workflowpattern } |  |  | 0.421 |
| ns | 222 |  | 46 | cmd/: complete file list | 1.4 |  | 0.333 |
| walker |  | 225 | 9 | Fs::DirListing { dir: pkg/gh } |  |  | 0.333 |
| walker |  | 247 | 22 | Fs::DirListing { dir: pkg/exprparser } |  |  | 0.333 |
| walker |  | 271 | 24 | Fs::DirListing { dir: pkg/artifacts } |  |  | 0.333 |
| walker |  | 296 | 25 | Fs::DirListing { dir: pkg/artifactcache } |  |  | 0.335 |
| walker |  | 299 | 3 | Fs::DirListing { dir: pkg/artifactcache/testdata } |  |  | 0.335 |
| walker |  | 312 | 13 | Fs::DirListing { dir: pkg/filecollector } |  |  | 0.335 |
| walker |  | 316 | 4 | Fs::DirListing { dir: pkg/artifactcache/testdata/example } |  |  | 0.335 |
| walker |  | 349 | 33 | Fs::DirListing { dir: pkg/lookpath } |  |  | 0.432 |
| ns | 349 |  | 127 | Repository root: complete entry list | 1.5 |  | 0.432 |
| walker |  | 368 | 19 | Fs::DirListing { dir: pkg/schema } |  |  | 0.434 |
| ns | 405 |  | 56 | pkg/model/: complete file list | 1.6 |  | 0.393 |
| walker |  | 414 | 46 | Fs::DirListing { dir: cmd } |  |  | 0.495 |
| walker |  | 433 | 19 | Fs::DirListing { dir: cmd/testdata } |  |  | 0.495 |
| walker |  | 512 | 79 | Fs::DirListing { dir: pkg/common } |  |  | 0.499 |
| walker |  | 521 | 9 | Fs::DirListing { dir: pkg/common/git } |  |  | 0.500 |
| walker |  | 577 | 56 | Fs::DirListing { dir: pkg/model } |  |  | 0.576 |
| walker |  | 585 | 8 | Fs::DirListing { dir: .vscode } |  |  | 0.577 |
| ns | 591 |  | 186 | pkg/runner/: complete file list | 1.7 |  | 0.463 |
| ns | 762 |  | 171 | README: how act works, end to end | 1.8 |  | 0.461 |
| walker |  | 771 | 186 | Fs::DirListing { dir: pkg/runner } |  |  | 0.620 |
| walker |  | 775 | 4 | Fs::DirListing { dir: pkg/runner/hashfiles } |  |  | 0.620 |
| walker |  | 779 | 4 | Fs::DirListing { dir: pkg/runner/res } |  |  | 0.620 |
| ns | 804 |  | 42 | Module path, Go version, released version | 1.9 |  | 0.615 |
| ns | 919 |  | 115 | cmd.Execute: CLI entry | 2.1 |  | 0.584 |
| walker |  | 939 | 160 | Fs::DirListing { dir: pkg/container } |  |  | 0.589 |
| walker |  | 957 | 18 | Fs::DirListing { dir: pkg/artifacts/testdata } |  |  | 0.589 |
| walker |  | 961 | 4 | Fs::DirListing { dir: pkg/artifacts/testdata/GHSL-2023-004 } |  |  | 0.589 |
| walker |  | 965 | 4 | Fs::DirListing { dir: pkg/artifacts/testdata/upload-and-download } |  |  | 0.589 |
| walker |  | 969 | 4 | Fs::DirListing { dir: pkg/artifacts/testdata/v4 } |  |  | 0.589 |
| walker |  | 990 | 21 | Fs::DirListing { dir: pkg/model/testdata } |  |  | 0.589 |
| walker |  | 994 | 4 | Fs::DirListing { dir: pkg/model/testdata/container-volumes } |  |  | 0.589 |
| walker |  | 998 | 4 | Fs::DirListing { dir: pkg/model/testdata/strategy } |  |  | 0.589 |
| walker |  | 1004 | 6 | Fs::DirListing { dir: pkg/model/testdata/empty-workflow } |  |  | 0.589 |
| walker |  | 1011 | 7 | Fs::DirListing { dir: pkg/model/testdata/nested } |  |  | 0.589 |
| walker |  | 1017 | 6 | Fs::DirListing { dir: pkg/model/testdata/nested/workflows } |  |  | 0.589 |
| walker |  | 1040 | 23 | Fs::DirListing { dir: pkg/exprparser/testdata } |  |  | 0.589 |
| walker |  | 1047 | 7 | Fs::DirListing { dir: pkg/exprparser/testdata/for-hashing-3 } |  |  | 0.589 |
| walker |  | 1052 | 5 | Fs::DirListing { dir: pkg/exprparser/testdata/for-hashing-3/nested } |  |  | 0.589 |
| walker |  | 1070 | 18 | Code::CodeKey { rung: ModuleDoc, file: pkg/container/docker_cli.go, decl: 0, sub: 0, line: 0 } |  |  | 0.589 |
| walker |  | 1090 | 20 | Fs::DirListing { dir: .github } |  |  | 0.590 |
| walker |  | 1116 | 26 | Fs::DirListing { dir: .github/workflows } |  |  | 0.591 |
| ns | 1131 |  | 212 | Root cobra.Command definition | 2.2 |  | 0.563 |
| walker |  | 1153 | 37 | Fs::DirListing { dir: pkg/container/testdata } |  |  | 0.563 |
| walker |  | 1157 | 4 | Fs::DirListing { dir: pkg/container/testdata/docker-pull-options } |  |  | 0.563 |
| walker |  | 1161 | 4 | Fs::DirListing { dir: pkg/container/testdata/scratch } |  |  | 0.563 |
| walker |  | 1346 | 185 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.727 |
| walker |  | 1401 | 55 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.727 |
| walker |  | 1425 | 24 | Fs::DirListing { dir: pkg/model/testdata/invalid-job-name } |  |  | 0.727 |
| ns | 1432 |  | 301 | Command-local flags: names and shorthands | 2.3 |  | 0.649 |
| ns | 1782 |  | 350 | Persistent flags: names and shorthands | 2.4 |  | 0.583 |
| ns | 1928 |  | 146 | Input struct + path accessor roster | 2.5 |  | 0.566 |
| walker |  | 2140 | 715 | Fs::DirListing { dir: pkg/runner/testdata } |  |  | 0.566 |
| walker |  | 2205 | 65 | Markdown::HeadingsOutline { file: IMAGES.md } |  |  | 0.566 |
| ns | 2206 |  | 278 | Config-file discovery: .actrc locations | 2.6 |  | 0.536 |
| walker |  | 2233 | 28 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.536 |
| walker |  | 2261 | 28 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.536 |
| walker |  | 2310 | 49 | Markdown::Section { file: IMAGES.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.536 |
| walker |  | 2335 | 25 | Code::CodeKey { rung: Names, file: main.go, decl: 0, sub: 0, line: 0 } |  |  | 0.545 |
| walker |  | 2386 | 51 | Code::CodeKey { rung: Decl, file: main.go, decl: 2, sub: 0, line: 13 } |  |  | 0.633 |
| walker |  | 2396 | 10 | Code::CodeKey { rung: Doc, file: main.go, decl: 1, sub: 0, line: 11 } |  |  | 0.667 |
| ns | 2522 |  | 316 | cmd package: complete function roster | 2.7 |  | 0.626 |
| walker |  | 2663 | 267 | Plaintext::Whole { file: Makefile } |  |  | 0.626 |
| walker |  | 2669 | 6 | Plaintext::DeclSurface { file: pkg/exprparser/testdata/for-hashing-1.txt } |  |  | 0.626 |
| walker |  | 2675 | 6 | Plaintext::DeclSurface { file: pkg/exprparser/testdata/for-hashing-2.txt } |  |  | 0.626 |
| walker |  | 2809 | 134 | Code::CodeKey { rung: ModuleDoc, file: pkg/artifactcache/doc.go, decl: 0, sub: 0, line: 0 } |  |  | 0.626 |
| walker |  | 2813 | 4 | Fs::DirListing { dir: .github/actions } |  |  | 0.626 |
| walker |  | 2847 | 34 | Code::CodeKey { rung: Names, file: cmd/dir.go, decl: 0, sub: 0, line: 0 } |  |  | 0.627 |
| walker |  | 2856 | 9 | Code::CodeKey { rung: Decl, file: cmd/dir.go, decl: 1, sub: 0, line: 10 } |  |  | 0.627 |
| walker |  | 2863 | 7 | Plaintext::DeclSurface { file: pkg/container/testdata/scratch/test.txt } |  |  | 0.627 |
| ns | 2869 |  | 347 | pkg/model: complete type roster | 3.1 |  | 0.588 |
| walker |  | 2930 | 67 | Plaintext::Whole { file: pkg/container/testdata/Dockerfile } |  |  | 0.588 |
| walker |  | 2938 | 8 | Plaintext::DeclSurface { file: pkg/exprparser/testdata/for-hashing-3/data.txt } |  |  | 0.588 |
| ns | 2979 |  | 110 | Workflow struct | 3.2 | 3.1 | 0.577 |
| walker |  | 3085 | 147 | Code::CodeKey { rung: Names, file: cmd/notices.go, decl: 0, sub: 0, line: 0 } |  |  | 0.582 |
| walker |  | 3112 | 27 | Code::CodeKey { rung: Decl, file: cmd/notices.go, decl: 1, sub: 0, line: 17 } |  |  | 0.582 |
| walker |  | 3141 | 29 | Code::CodeKey { rung: Body, file: cmd/notices.go, decl: 4, sub: 0, line: 57 } |  |  | 0.582 |
| walker |  | 3208 | 67 | Code::CodeKey { rung: Body, file: cmd/notices.go, decl: 9, sub: 0, line: 138 } |  |  | 0.582 |
| ns | 3209 |  | 230 | Planner interface + Plan/Stage/Run | 3.3 | 3.1 | 0.554 |
| walker |  | 3285 | 77 | Code::CodeKey { rung: Body, file: cmd/notices.go, decl: 7, sub: 0, line: 121 } |  |  | 0.554 |
| walker |  | 3435 | 150 | Code::CodeKey { rung: Names, file: cmd/input.go, decl: 0, sub: 0, line: 0 } |  |  | 0.573 |
| walker |  | 3444 | 9 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 6, sub: 0, line: 99 } |  |  | 0.573 |
| walker |  | 3455 | 11 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 3, sub: 0, line: 85 } |  |  | 0.573 |
| walker |  | 3466 | 11 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 4, sub: 0, line: 90 } |  |  | 0.573 |
| ns | 3471 |  | 262 | Job struct: every supported job key | 3.4 | 3.1 | 0.555 |
| walker |  | 3477 | 11 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 5, sub: 0, line: 94 } |  |  | 0.555 |
| walker |  | 3489 | 12 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 4, sub: 0, line: 90 } |  |  | 0.555 |
| walker |  | 3502 | 13 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 3, sub: 0, line: 85 } |  |  | 0.555 |
| walker |  | 3515 | 13 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 6, sub: 0, line: 99 } |  |  | 0.555 |
| walker |  | 3529 | 14 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 8, sub: 0, line: 109 } |  |  | 0.555 |
| walker |  | 3544 | 15 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 7, sub: 0, line: 104 } |  |  | 0.555 |
| walker |  | 3559 | 15 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 9, sub: 0, line: 114 } |  |  | 0.555 |
| walker |  | 3610 | 51 | Code::CodeKey { rung: Names, file: cmd/secrets.go, decl: 0, sub: 0, line: 0 } |  |  | 0.559 |
| walker |  | 3617 | 7 | Code::CodeKey { rung: Body, file: cmd/secrets.go, decl: 3, sub: 0, line: 40 } |  |  | 0.559 |
| walker |  | 3635 | 18 | Code::CodeKey { rung: Names, file: cmd/graph.go, decl: 0, sub: 0, line: 0 } |  |  | 0.560 |
| walker |  | 3653 | 18 | Code::CodeKey { rung: Names, file: cmd/list.go, decl: 0, sub: 0, line: 0 } |  |  | 0.562 |
| walker |  | 3664 | 11 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 8, sub: 0, line: 109 } |  |  | 0.562 |
| ns | 3695 |  | 224 | Step struct: every supported step key | 3.5 | 3.1 | 0.548 |
| ns | 4027 |  | 332 | StepType constants | 3.6 | 3.1 | 0.527 |
| walker |  | 4038 | 374 | Code::CodeKey { rung: Names, file: cmd/root.go, decl: 0, sub: 0, line: 0 } |  |  | 0.570 |
| walker |  | 4088 | 50 | Code::CodeKey { rung: Decl, file: cmd/root.go, decl: 1, sub: 0, line: 37 } |  |  | 0.570 |
| walker |  | 4100 | 12 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 18, sub: 0, line: 391 } |  |  | 0.570 |
| walker |  | 4113 | 13 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 3, sub: 0, line: 47 } |  |  | 0.572 |
| walker |  | 4129 | 16 | Code::CodeKey { rung: Body, file: cmd/root.go, decl: 15, sub: 0, line: 345 } |  |  | 0.572 |
| walker |  | 4164 | 35 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 5, sub: 0, line: 138 } |  |  | 0.574 |
| walker |  | 4196 | 32 | Code::CodeKey { rung: Body, file: cmd/root.go, decl: 12, sub: 0, line: 314 } |  |  | 0.574 |
| ns | 4248 |  | 221 | GithubContext: complete field-name roster | 3.7 | 3.1 | 0.550 |
| walker |  | 4309 | 113 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.550 |
| ns | 4531 |  | 283 | Executor type + complete combinator roster | 4.1 |  | 0.534 |
| walker |  | 4890 | 581 | Code::CodeKey { rung: Decl, file: cmd/input.go, decl: 1, sub: 0, line: 10 } |  |  | 0.536 |
| walker |  | 4904 | 14 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 1, sub: 0, line: 10 } |  |  | 0.541 |
| walker |  | 4911 | 7 | Plaintext::DeclSurface { file: pkg/exprparser/testdata/for-hashing-3/nested/nested-data.txt } |  |  | 0.541 |
| walker |  | 4932 | 21 | Code::CodeKey { rung: Names, file: cmd/platforms.go, decl: 0, sub: 0, line: 0 } |  |  | 0.548 |
| walker |  | 5036 | 104 | Code::CodeKey { rung: Names, file: pkg/container/container_types.go, decl: 0, sub: 0, line: 0 } |  |  | 0.548 |
| walker |  | 5043 | 7 | Code::CodeKey { rung: Decl, file: pkg/container/container_types.go, decl: 7, sub: 0, line: 80 } |  |  | 0.548 |
| ns | 5066 |  | 535 | pkg/runner: complete type roster | 4.2 |  | 0.523 |
| walker |  | 5072 | 29 | Code::CodeKey { rung: Decl, file: pkg/container/container_types.go, decl: 2, sub: 0, line: 36 } |  |  | 0.523 |
| walker |  | 5119 | 47 | Code::CodeKey { rung: Decl, file: pkg/container/container_types.go, decl: 5, sub: 0, line: 70 } |  |  | 0.523 |
| walker |  | 5171 | 52 | Code::CodeKey { rung: Decl, file: pkg/container/container_types.go, decl: 4, sub: 0, line: 61 } |  |  | 0.523 |
| ns | 5176 |  | 110 | Runner interface and New() | 4.3 | 4.2 | 0.516 |
| walker |  | 5187 | 16 | Code::CodeKey { rung: Doc, file: pkg/container/container_types.go, decl: 2, sub: 0, line: 36 } |  |  | 0.516 |
| walker |  | 5205 | 18 | Code::CodeKey { rung: Doc, file: pkg/container/container_types.go, decl: 5, sub: 0, line: 70 } |  |  | 0.516 |
| walker |  | 5225 | 20 | Code::CodeKey { rung: Doc, file: pkg/container/container_types.go, decl: 4, sub: 0, line: 61 } |  |  | 0.516 |
| walker |  | 5431 | 206 | Code::CodeKey { rung: Decl, file: pkg/container/container_types.go, decl: 1, sub: 0, line: 12 } |  |  | 0.516 |
| walker |  | 5446 | 15 | Code::CodeKey { rung: Doc, file: pkg/container/container_types.go, decl: 1, sub: 0, line: 12 } |  |  | 0.516 |
| ns | 5564 |  | 388 | runner.Config: fields (first half, rest elided) | 4.4 | 4.2 | 0.502 |
| walker |  | 5692 | 246 | Code::CodeKey { rung: Decl, file: pkg/container/container_types.go, decl: 3, sub: 0, line: 43 } |  |  | 0.503 |
| walker |  | 5702 | 10 | Code::CodeKey { rung: Doc, file: pkg/container/container_types.go, decl: 3, sub: 0, line: 43 } |  |  | 0.503 |
| walker |  | 5779 | 77 | Code::CodeKey { rung: Body, file: cmd/notices.go, decl: 8, sub: 0, line: 130 } |  |  | 0.503 |
| ns | 5866 |  | 302 | RunContext struct | 4.5 | 4.2 | 0.489 |
| walker |  | 5896 | 117 | Code::CodeKey { rung: Body, file: cmd/dir.go, decl: 2, sub: 0, line: 15 } |  |  | 0.489 |
| walker |  | 5909 | 13 | Code::CodeKey { rung: Names, file: pkg/model/job_context.go, decl: 0, sub: 0, line: 0 } |  |  | 0.489 |
| walker |  | 6003 | 94 | Code::CodeKey { rung: Decl, file: pkg/model/job_context.go, decl: 1, sub: 0, line: 3 } |  |  | 0.489 |
| walker |  | 6043 | 40 | Code::CodeKey { rung: Names, file: pkg/artifactcache/model.go, decl: 0, sub: 0, line: 0 } |  |  | 0.489 |
| walker |  | 6086 | 43 | Code::CodeKey { rung: Decl, file: pkg/artifactcache/model.go, decl: 1, sub: 0, line: 3 } |  |  | 0.489 |
| ns | 6221 |  | 355 | stepFactory: StepType -> implementation | 4.6 | 4.2 | 0.475 |
| walker |  | 6233 | 147 | Code::CodeKey { rung: Decl, file: pkg/artifactcache/model.go, decl: 3, sub: 0, line: 26 } |  |  | 0.475 |
| ns | 6499 |  | 278 | step interface and stage enum | 4.7 | 4.2 | 0.460 |
| ns | 6994 |  | 495 | run_context.go: method roster (container lifecycle) | 4.8 |  | 0.447 |
| walker |  | 7042 | 809 | GoMod::File { file: go.mod } |  |  | 0.448 |
| walker |  | 7056 | 14 | Code::CodeKey { rung: Names, file: pkg/container/executions_environment.go, decl: 0, sub: 0, line: 0 } |  |  | 0.448 |
| ns | 7140 |  | 146 | ActionCache implementations | 4.9 | 4.2 | 0.440 |
| walker |  | 7164 | 108 | Code::CodeKey { rung: Decl, file: pkg/container/executions_environment.go, decl: 1, sub: 0, line: 5 } |  |  | 0.441 |
| walker |  | 7192 | 28 | Code::CodeKey { rung: Names, file: pkg/lookpath/error.go, decl: 0, sub: 0, line: 0 } |  |  | 0.441 |
| walker |  | 7210 | 18 | Code::CodeKey { rung: Decl, file: pkg/lookpath/error.go, decl: 1, sub: 0, line: 3 } |  |  | 0.441 |
| walker |  | 7219 | 9 | Code::CodeKey { rung: Body, file: pkg/lookpath/error.go, decl: 2, sub: 0, line: 8 } |  |  | 0.441 |
| ns | 7300 |  | 160 | pkg/container: complete file list | 5.1 |  | 0.468 |
| walker |  | 7350 | 131 | Code::CodeKey { rung: Names, file: pkg/model/step_result.go, decl: 0, sub: 0, line: 0 } |  |  | 0.468 |
| walker |  | 7357 | 7 | Code::CodeKey { rung: Decl, file: pkg/model/step_result.go, decl: 2, sub: 0, line: 7 } |  |  | 0.468 |
| walker |  | 7385 | 28 | Code::CodeKey { rung: Decl, file: pkg/model/step_result.go, decl: 3, sub: 0, line: 13 } |  |  | 0.468 |
| walker |  | 7434 | 49 | Code::CodeKey { rung: Decl, file: pkg/model/step_result.go, decl: 7, sub: 0, line: 41 } |  |  | 0.468 |
| walker |  | 7446 | 12 | Code::CodeKey { rung: Body, file: pkg/model/step_result.go, decl: 4, sub: 0, line: 19 } |  |  | 0.468 |
| walker |  | 7484 | 38 | Code::CodeKey { rung: Body, file: pkg/model/step_result.go, decl: 6, sub: 0, line: 34 } |  |  | 0.468 |
| walker |  | 7543 | 59 | Code::CodeKey { rung: Names, file: pkg/lookpath/env.go, decl: 0, sub: 0, line: 0 } |  |  | 0.468 |
| walker |  | 7546 | 3 | Code::CodeKey { rung: Decl, file: pkg/lookpath/env.go, decl: 2, sub: 0, line: 9 } |  |  | 0.468 |
| walker |  | 7560 | 14 | Code::CodeKey { rung: Decl, file: pkg/lookpath/env.go, decl: 1, sub: 0, line: 5 } |  |  | 0.468 |
| walker |  | 7569 | 9 | Code::CodeKey { rung: Body, file: pkg/lookpath/env.go, decl: 3, sub: 0, line: 12 } |  |  | 0.468 |
| ns | 7577 |  | 277 | Container interface | 5.2 |  | 0.482 |
| walker |  | 7583 | 14 | Code::CodeKey { rung: Body, file: pkg/lookpath/env.go, decl: 4, sub: 0, line: 16 } |  |  | 0.482 |
| ns | 7719 |  | 142 | ExecutionsEnvironment interface | 5.3 |  | 0.487 |
| walker |  | 7850 | 267 | Code::CodeKey { rung: Names, file: pkg/common/draw.go, decl: 0, sub: 0, line: 0 } |  |  | 0.487 |
| walker |  | 7861 | 11 | Code::CodeKey { rung: Decl, file: pkg/common/draw.go, decl: 2, sub: 0, line: 14 } |  |  | 0.487 |
| walker |  | 7881 | 20 | Code::CodeKey { rung: Decl, file: pkg/common/draw.go, decl: 7, sub: 0, line: 59 } |  |  | 0.487 |
| walker |  | 7910 | 29 | Code::CodeKey { rung: Decl, file: pkg/common/draw.go, decl: 6, sub: 0, line: 52 } |  |  | 0.487 |
| walker |  | 7915 | 5 | Code::CodeKey { rung: Doc, file: pkg/common/draw.go, decl: 2, sub: 0, line: 14 } |  |  | 0.487 |
| walker |  | 7921 | 6 | Code::CodeKey { rung: Doc, file: pkg/common/draw.go, decl: 7, sub: 0, line: 59 } |  |  | 0.487 |
| walker |  | 7929 | 8 | Code::CodeKey { rung: Doc, file: pkg/common/draw.go, decl: 6, sub: 0, line: 52 } |  |  | 0.487 |
| walker |  | 7938 | 9 | Code::CodeKey { rung: Doc, file: pkg/common/draw.go, decl: 13, sub: 0, line: 127 } |  |  | 0.487 |
| ns | 7984 |  | 265 | Docker backend vs docker-less stub: the build-tag split | 5.4 |  | 0.479 |
| walker |  | 7995 | 57 | Code::CodeKey { rung: Decl, file: pkg/common/draw.go, decl: 4, sub: 0, line: 35 } |  |  | 0.479 |
| walker |  | 8005 | 10 | Code::CodeKey { rung: Doc, file: pkg/common/draw.go, decl: 3, sub: 0, line: 22 } |  |  | 0.479 |
| walker |  | 8015 | 10 | Code::CodeKey { rung: Doc, file: pkg/common/draw.go, decl: 11, sub: 0, line: 98 } |  |  | 0.479 |
| walker |  | 8025 | 10 | Code::CodeKey { rung: Doc, file: pkg/common/draw.go, decl: 14, sub: 0, line: 141 } |  |  | 0.479 |
| walker |  | 8036 | 11 | Code::CodeKey { rung: Doc, file: pkg/common/draw.go, decl: 1, sub: 0, line: 11 } |  |  | 0.479 |
| walker |  | 8047 | 11 | Code::CodeKey { rung: Doc, file: pkg/common/draw.go, decl: 12, sub: 0, line: 110 } |  |  | 0.479 |
| ns | 8161 |  | 177 | Docker socket discovery | 5.5 |  | 0.473 |
| walker |  | 8176 | 129 | Code::CodeKey { rung: Decl, file: pkg/common/draw.go, decl: 5, sub: 0, line: 44 } |  |  | 0.473 |
| walker |  | 8192 | 16 | Code::CodeKey { rung: Names, file: pkg/common/outbound_ip.go, decl: 0, sub: 0, line: 0 } |  |  | 0.473 |
| ns | 8251 |  | 90 | Supporting packages: complete file lists (A) | 6.1 |  | 0.488 |
| ns | 8410 |  | 159 | Supporting packages: complete file lists (B) | 6.2 |  | 0.509 |
| walker |  | 8466 | 274 | Code::CodeKey { rung: Names, file: pkg/model/action.go, decl: 0, sub: 0, line: 0 } |  |  | 0.511 |
| walker |  | 8494 | 28 | Code::CodeKey { rung: Decl, file: pkg/model/action.go, decl: 11, sub: 0, line: 127 } |  |  | 0.511 |
| walker |  | 8536 | 42 | Code::CodeKey { rung: Decl, file: pkg/model/action.go, decl: 10, sub: 0, line: 120 } |  |  | 0.511 |
| walker |  | 8548 | 12 | Code::CodeKey { rung: Doc, file: pkg/model/action.go, decl: 12, sub: 0, line: 133 } |  |  | 0.511 |
| ns | 8554 |  | 144 | Context names the expression interpreter resolves | 6.3 |  | 0.506 |
| walker |  | 8560 | 12 | Code::CodeKey { rung: Body, file: pkg/model/action.go, decl: 5, sub: 0, line: 63 } |  |  | 0.506 |
| walker |  | 8656 | 96 | Code::CodeKey { rung: Decl, file: pkg/model/action.go, decl: 3, sub: 0, line: 39 } |  |  | 0.506 |
| walker |  | 8673 | 17 | Code::CodeKey { rung: Doc, file: pkg/model/action.go, decl: 1, sub: 0, line: 13 } |  |  | 0.506 |
| ns | 8679 |  | 125 | Expression functions act implements | 6.4 |  | 0.502 |
| walker |  | 8810 | 137 | Code::CodeKey { rung: Decl, file: pkg/model/action.go, decl: 8, sub: 0, line: 89 } |  |  | 0.502 |
| ns | 8903 |  | 224 | Artifact cache server: routes and lifecycle | 6.5 |  | 0.497 |
| walker |  | 9016 | 206 | Code::CodeKey { rung: Decl, file: pkg/model/action.go, decl: 7, sub: 0, line: 72 } |  |  | 0.497 |
| walker |  | 9029 | 13 | Code::CodeKey { rung: Doc, file: pkg/model/action.go, decl: 7, sub: 0, line: 72 } |  |  | 0.497 |
| ns | 9068 |  | 165 | Artifact server: Serve + the v4 route base | 6.6 |  | 0.493 |
| walker |  | 9073 | 44 | Code::CodeKey { rung: Doc, file: pkg/model/action.go, decl: 8, sub: 0, line: 89 } |  |  | 0.493 |
| walker |  | 9122 | 49 | Code::CodeKey { rung: Doc, file: pkg/model/action.go, decl: 10, sub: 0, line: 120 } |  |  | 0.493 |
| walker |  | 9195 | 73 | Code::CodeKey { rung: Doc, file: pkg/model/action.go, decl: 11, sub: 0, line: 127 } |  |  | 0.493 |
| walker |  | 9309 | 114 | Code::CodeKey { rung: Names, file: pkg/common/auth.go, decl: 0, sub: 0, line: 0 } |  |  | 0.493 |
| walker |  | 9316 | 7 | Code::CodeKey { rung: Decl, file: pkg/common/auth.go, decl: 4, sub: 0, line: 33 } |  |  | 0.493 |
| walker |  | 9338 | 22 | Code::CodeKey { rung: Decl, file: pkg/common/auth.go, decl: 2, sub: 0, line: 26 } |  |  | 0.493 |
| ns | 9373 |  | 305 | pkg/common: complete exported-function roster | 6.7 |  | 0.488 |
| walker |  | 9408 | 70 | Code::CodeKey { rung: Decl, file: pkg/common/auth.go, decl: 1, sub: 0, line: 17 } |  |  | 0.488 |
| walker |  | 9484 | 76 | Code::CodeKey { rung: Doc, file: pkg/common/outbound_ip.go, decl: 1, sub: 0, line: 13 } |  |  | 0.488 |
| walker |  | 9495 | 11 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 9, sub: 0, line: 114 } |  |  | 0.488 |
| ns | 9536 |  | 163 | pkg/common/git: repo detection and cloning | 6.8 |  | 0.484 |
| ns | 9666 |  | 130 | CI, issue-template and editor directories | 7.1 |  | 0.489 |
| walker |  | 9786 | 291 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.489 |
| walker |  | 9871 | 85 | Code::CodeKey { rung: Names, file: pkg/container/docker_socket.go, decl: 0, sub: 0, line: 0 } |  |  | 0.490 |
| walker |  | 9891 | 20 | Code::CodeKey { rung: Decl, file: pkg/container/docker_socket.go, decl: 4, sub: 0, line: 57 } |  |  | 0.492 |
| walker |  | 9906 | 15 | Code::CodeKey { rung: Doc, file: pkg/container/docker_socket.go, decl: 2, sub: 0, line: 23 } |  |  | 0.492 |
| ns | 9919 |  | 253 | Key direct dependencies | 7.2 |  | 0.501 |
| walker |  | 9989 | 83 | Code::CodeKey { rung: Decl, file: pkg/container/docker_socket.go, decl: 1, sub: 0, line: 12 } |  |  | 0.508 |

Score(3000)=0.578 I=0.869 C=0.384 ns_rows≤3K=18/47 grid(1000/1442/2080/3000/4327/6240/9000)=0.589/0.649/0.566/0.578/0.550/0.476/0.498

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
| walker |  | 1429 | 28 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.727 |
| ns | 1432 |  | 301 | Command-local flags: names and shorthands | 2.3 |  | 0.649 |
| walker |  | 1457 | 28 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.649 |
| walker |  | 1481 | 24 | Fs::DirListing { dir: pkg/model/testdata/invalid-job-name } |  |  | 0.649 |
| ns | 1782 |  | 350 | Persistent flags: names and shorthands | 2.4 |  | 0.583 |
| ns | 1928 |  | 146 | Input struct + path accessor roster | 2.5 |  | 0.566 |
| walker |  | 2196 | 715 | Fs::DirListing { dir: pkg/runner/testdata } |  |  | 0.566 |
| ns | 2206 |  | 278 | Config-file discovery: .actrc locations | 2.6 |  | 0.536 |
| walker |  | 2221 | 25 | Code::CodeKey { rung: Names, file: main.go, decl: 0, sub: 0, line: 0 } |  |  | 0.545 |
| walker |  | 2272 | 51 | Code::CodeKey { rung: Decl, file: main.go, decl: 2, sub: 0, line: 13 } |  |  | 0.633 |
| walker |  | 2282 | 10 | Code::CodeKey { rung: Doc, file: main.go, decl: 1, sub: 0, line: 11 } |  |  | 0.667 |
| ns | 2522 |  | 316 | cmd package: complete function roster | 2.7 |  | 0.626 |
| walker |  | 2549 | 267 | Plaintext::Whole { file: Makefile } |  |  | 0.626 |
| walker |  | 2555 | 6 | Plaintext::DeclSurface { file: pkg/exprparser/testdata/for-hashing-1.txt } |  |  | 0.626 |
| walker |  | 2561 | 6 | Plaintext::DeclSurface { file: pkg/exprparser/testdata/for-hashing-2.txt } |  |  | 0.626 |
| walker |  | 2695 | 134 | Code::CodeKey { rung: ModuleDoc, file: pkg/artifactcache/doc.go, decl: 0, sub: 0, line: 0 } |  |  | 0.626 |
| walker |  | 2699 | 4 | Fs::DirListing { dir: .github/actions } |  |  | 0.626 |
| walker |  | 2733 | 34 | Code::CodeKey { rung: Names, file: cmd/dir.go, decl: 0, sub: 0, line: 0 } |  |  | 0.627 |
| walker |  | 2742 | 9 | Code::CodeKey { rung: Decl, file: cmd/dir.go, decl: 1, sub: 0, line: 10 } |  |  | 0.627 |
| walker |  | 2855 | 113 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.627 |
| walker |  | 2862 | 7 | Plaintext::DeclSurface { file: pkg/container/testdata/scratch/test.txt } |  |  | 0.627 |
| ns | 2869 |  | 347 | pkg/model: complete type roster | 3.1 |  | 0.588 |
| walker |  | 2929 | 67 | Plaintext::Whole { file: pkg/container/testdata/Dockerfile } |  |  | 0.588 |
| walker |  | 2937 | 8 | Plaintext::DeclSurface { file: pkg/exprparser/testdata/for-hashing-3/data.txt } |  |  | 0.588 |
| ns | 2979 |  | 110 | Workflow struct | 3.2 | 3.1 | 0.577 |
| walker |  | 3084 | 147 | Code::CodeKey { rung: Names, file: cmd/notices.go, decl: 0, sub: 0, line: 0 } |  |  | 0.582 |
| walker |  | 3111 | 27 | Code::CodeKey { rung: Decl, file: cmd/notices.go, decl: 1, sub: 0, line: 17 } |  |  | 0.582 |
| walker |  | 3140 | 29 | Code::CodeKey { rung: Body, file: cmd/notices.go, decl: 4, sub: 0, line: 57 } |  |  | 0.582 |
| walker |  | 3207 | 67 | Code::CodeKey { rung: Body, file: cmd/notices.go, decl: 9, sub: 0, line: 138 } |  |  | 0.582 |
| ns | 3209 |  | 230 | Planner interface + Plan/Stage/Run | 3.3 | 3.1 | 0.554 |
| walker |  | 3284 | 77 | Code::CodeKey { rung: Body, file: cmd/notices.go, decl: 7, sub: 0, line: 121 } |  |  | 0.554 |
| walker |  | 3434 | 150 | Code::CodeKey { rung: Names, file: cmd/input.go, decl: 0, sub: 0, line: 0 } |  |  | 0.573 |
| walker |  | 3443 | 9 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 6, sub: 0, line: 99 } |  |  | 0.573 |
| walker |  | 3454 | 11 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 3, sub: 0, line: 85 } |  |  | 0.573 |
| walker |  | 3465 | 11 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 4, sub: 0, line: 90 } |  |  | 0.573 |
| ns | 3471 |  | 262 | Job struct: every supported job key | 3.4 | 3.1 | 0.555 |
| walker |  | 3476 | 11 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 5, sub: 0, line: 94 } |  |  | 0.555 |
| walker |  | 3488 | 12 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 4, sub: 0, line: 90 } |  |  | 0.555 |
| walker |  | 3501 | 13 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 3, sub: 0, line: 85 } |  |  | 0.555 |
| walker |  | 3514 | 13 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 6, sub: 0, line: 99 } |  |  | 0.555 |
| walker |  | 3528 | 14 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 8, sub: 0, line: 109 } |  |  | 0.555 |
| walker |  | 3543 | 15 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 7, sub: 0, line: 104 } |  |  | 0.555 |
| walker |  | 3558 | 15 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 9, sub: 0, line: 114 } |  |  | 0.555 |
| walker |  | 3609 | 51 | Code::CodeKey { rung: Names, file: cmd/secrets.go, decl: 0, sub: 0, line: 0 } |  |  | 0.559 |
| walker |  | 3616 | 7 | Code::CodeKey { rung: Body, file: cmd/secrets.go, decl: 3, sub: 0, line: 40 } |  |  | 0.559 |
| walker |  | 3634 | 18 | Code::CodeKey { rung: Names, file: cmd/graph.go, decl: 0, sub: 0, line: 0 } |  |  | 0.560 |
| walker |  | 3652 | 18 | Code::CodeKey { rung: Names, file: cmd/list.go, decl: 0, sub: 0, line: 0 } |  |  | 0.562 |
| walker |  | 3663 | 11 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 8, sub: 0, line: 109 } |  |  | 0.562 |
| ns | 3695 |  | 224 | Step struct: every supported step key | 3.5 | 3.1 | 0.548 |
| ns | 4027 |  | 332 | StepType constants | 3.6 | 3.1 | 0.527 |
| walker |  | 4037 | 374 | Code::CodeKey { rung: Names, file: cmd/root.go, decl: 0, sub: 0, line: 0 } |  |  | 0.570 |
| walker |  | 4087 | 50 | Code::CodeKey { rung: Decl, file: cmd/root.go, decl: 1, sub: 0, line: 37 } |  |  | 0.570 |
| walker |  | 4099 | 12 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 18, sub: 0, line: 391 } |  |  | 0.570 |
| walker |  | 4112 | 13 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 3, sub: 0, line: 47 } |  |  | 0.572 |
| walker |  | 4128 | 16 | Code::CodeKey { rung: Body, file: cmd/root.go, decl: 15, sub: 0, line: 345 } |  |  | 0.572 |
| walker |  | 4163 | 35 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 5, sub: 0, line: 138 } |  |  | 0.574 |
| walker |  | 4195 | 32 | Code::CodeKey { rung: Body, file: cmd/root.go, decl: 12, sub: 0, line: 314 } |  |  | 0.574 |
| ns | 4248 |  | 221 | GithubContext: complete field-name roster | 3.7 | 3.1 | 0.550 |
| walker |  | 4486 | 291 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.550 |
| ns | 4531 |  | 283 | Executor type + complete combinator roster | 4.1 |  | 0.534 |
| walker |  | 4724 | 238 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.536 |
| ns | 5066 |  | 535 | pkg/runner: complete type roster | 4.2 |  | 0.512 |
| ns | 5176 |  | 110 | Runner interface and New() | 4.3 | 4.2 | 0.505 |
| walker |  | 5305 | 581 | Code::CodeKey { rung: Decl, file: cmd/input.go, decl: 1, sub: 0, line: 10 } |  |  | 0.506 |
| walker |  | 5319 | 14 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 1, sub: 0, line: 10 } |  |  | 0.512 |
| walker |  | 5326 | 7 | Plaintext::DeclSurface { file: pkg/exprparser/testdata/for-hashing-3/nested/nested-data.txt } |  |  | 0.512 |
| walker |  | 5347 | 21 | Code::CodeKey { rung: Names, file: cmd/platforms.go, decl: 0, sub: 0, line: 0 } |  |  | 0.518 |
| walker |  | 5451 | 104 | Code::CodeKey { rung: Names, file: pkg/container/container_types.go, decl: 0, sub: 0, line: 0 } |  |  | 0.518 |
| walker |  | 5458 | 7 | Code::CodeKey { rung: Decl, file: pkg/container/container_types.go, decl: 7, sub: 0, line: 80 } |  |  | 0.518 |
| walker |  | 5487 | 29 | Code::CodeKey { rung: Decl, file: pkg/container/container_types.go, decl: 2, sub: 0, line: 36 } |  |  | 0.518 |
| walker |  | 5534 | 47 | Code::CodeKey { rung: Decl, file: pkg/container/container_types.go, decl: 5, sub: 0, line: 70 } |  |  | 0.518 |
| ns | 5564 |  | 388 | runner.Config: fields (first half, rest elided) | 4.4 | 4.2 | 0.504 |
| walker |  | 5586 | 52 | Code::CodeKey { rung: Decl, file: pkg/container/container_types.go, decl: 4, sub: 0, line: 61 } |  |  | 0.504 |
| walker |  | 5602 | 16 | Code::CodeKey { rung: Doc, file: pkg/container/container_types.go, decl: 2, sub: 0, line: 36 } |  |  | 0.504 |
| walker |  | 5620 | 18 | Code::CodeKey { rung: Doc, file: pkg/container/container_types.go, decl: 5, sub: 0, line: 70 } |  |  | 0.504 |
| walker |  | 5640 | 20 | Code::CodeKey { rung: Doc, file: pkg/container/container_types.go, decl: 4, sub: 0, line: 61 } |  |  | 0.504 |
| walker |  | 5846 | 206 | Code::CodeKey { rung: Decl, file: pkg/container/container_types.go, decl: 1, sub: 0, line: 12 } |  |  | 0.504 |
| walker |  | 5861 | 15 | Code::CodeKey { rung: Doc, file: pkg/container/container_types.go, decl: 1, sub: 0, line: 12 } |  |  | 0.504 |
| ns | 5866 |  | 302 | RunContext struct | 4.5 | 4.2 | 0.490 |
| walker |  | 6107 | 246 | Code::CodeKey { rung: Decl, file: pkg/container/container_types.go, decl: 3, sub: 0, line: 43 } |  |  | 0.491 |
| walker |  | 6117 | 10 | Code::CodeKey { rung: Doc, file: pkg/container/container_types.go, decl: 3, sub: 0, line: 43 } |  |  | 0.491 |
| walker |  | 6194 | 77 | Code::CodeKey { rung: Body, file: cmd/notices.go, decl: 8, sub: 0, line: 130 } |  |  | 0.491 |
| ns | 6221 |  | 355 | stepFactory: StepType -> implementation | 4.6 | 4.2 | 0.476 |
| walker |  | 6311 | 117 | Code::CodeKey { rung: Body, file: cmd/dir.go, decl: 2, sub: 0, line: 15 } |  |  | 0.476 |
| walker |  | 6324 | 13 | Code::CodeKey { rung: Names, file: pkg/model/job_context.go, decl: 0, sub: 0, line: 0 } |  |  | 0.477 |
| walker |  | 6418 | 94 | Code::CodeKey { rung: Decl, file: pkg/model/job_context.go, decl: 1, sub: 0, line: 3 } |  |  | 0.477 |
| walker |  | 6458 | 40 | Code::CodeKey { rung: Names, file: pkg/artifactcache/model.go, decl: 0, sub: 0, line: 0 } |  |  | 0.477 |
| ns | 6499 |  | 278 | step interface and stage enum | 4.7 | 4.2 | 0.462 |
| walker |  | 6501 | 43 | Code::CodeKey { rung: Decl, file: pkg/artifactcache/model.go, decl: 1, sub: 0, line: 3 } |  |  | 0.462 |
| walker |  | 6648 | 147 | Code::CodeKey { rung: Decl, file: pkg/artifactcache/model.go, decl: 3, sub: 0, line: 26 } |  |  | 0.462 |
| ns | 6994 |  | 495 | run_context.go: method roster (container lifecycle) | 4.8 |  | 0.449 |
| ns | 7140 |  | 146 | ActionCache implementations | 4.9 | 4.2 | 0.441 |
| ns | 7300 |  | 160 | pkg/container: complete file list | 5.1 |  | 0.468 |
| walker |  | 7457 | 809 | GoMod::File { file: go.mod } |  |  | 0.469 |
| walker |  | 7471 | 14 | Code::CodeKey { rung: Names, file: pkg/container/executions_environment.go, decl: 0, sub: 0, line: 0 } |  |  | 0.469 |
| ns | 7577 |  | 277 | Container interface | 5.2 |  | 0.482 |
| walker |  | 7579 | 108 | Code::CodeKey { rung: Decl, file: pkg/container/executions_environment.go, decl: 1, sub: 0, line: 5 } |  |  | 0.483 |
| walker |  | 7607 | 28 | Code::CodeKey { rung: Names, file: pkg/lookpath/error.go, decl: 0, sub: 0, line: 0 } |  |  | 0.483 |
| walker |  | 7625 | 18 | Code::CodeKey { rung: Decl, file: pkg/lookpath/error.go, decl: 1, sub: 0, line: 3 } |  |  | 0.483 |
| walker |  | 7634 | 9 | Code::CodeKey { rung: Body, file: pkg/lookpath/error.go, decl: 2, sub: 0, line: 8 } |  |  | 0.483 |
| ns | 7719 |  | 142 | ExecutionsEnvironment interface | 5.3 |  | 0.488 |
| walker |  | 7765 | 131 | Code::CodeKey { rung: Names, file: pkg/model/step_result.go, decl: 0, sub: 0, line: 0 } |  |  | 0.488 |
| walker |  | 7772 | 7 | Code::CodeKey { rung: Decl, file: pkg/model/step_result.go, decl: 2, sub: 0, line: 7 } |  |  | 0.488 |
| walker |  | 7800 | 28 | Code::CodeKey { rung: Decl, file: pkg/model/step_result.go, decl: 3, sub: 0, line: 13 } |  |  | 0.488 |
| walker |  | 7849 | 49 | Code::CodeKey { rung: Decl, file: pkg/model/step_result.go, decl: 7, sub: 0, line: 41 } |  |  | 0.488 |
| walker |  | 7861 | 12 | Code::CodeKey { rung: Body, file: pkg/model/step_result.go, decl: 4, sub: 0, line: 19 } |  |  | 0.488 |
| walker |  | 7899 | 38 | Code::CodeKey { rung: Body, file: pkg/model/step_result.go, decl: 6, sub: 0, line: 34 } |  |  | 0.488 |
| walker |  | 7958 | 59 | Code::CodeKey { rung: Names, file: pkg/lookpath/env.go, decl: 0, sub: 0, line: 0 } |  |  | 0.488 |
| walker |  | 7961 | 3 | Code::CodeKey { rung: Decl, file: pkg/lookpath/env.go, decl: 2, sub: 0, line: 9 } |  |  | 0.488 |
| walker |  | 7975 | 14 | Code::CodeKey { rung: Decl, file: pkg/lookpath/env.go, decl: 1, sub: 0, line: 5 } |  |  | 0.488 |
| walker |  | 7984 | 9 | Code::CodeKey { rung: Body, file: pkg/lookpath/env.go, decl: 3, sub: 0, line: 12 } |  |  | 0.480 |
| ns | 7984 |  | 265 | Docker backend vs docker-less stub: the build-tag split | 5.4 |  | 0.480 |
| walker |  | 7998 | 14 | Code::CodeKey { rung: Body, file: pkg/lookpath/env.go, decl: 4, sub: 0, line: 16 } |  |  | 0.480 |
| ns | 8161 |  | 177 | Docker socket discovery | 5.5 |  | 0.475 |
| ns | 8251 |  | 90 | Supporting packages: complete file lists (A) | 6.1 |  | 0.489 |
| walker |  | 8265 | 267 | Code::CodeKey { rung: Names, file: pkg/common/draw.go, decl: 0, sub: 0, line: 0 } |  |  | 0.489 |
| walker |  | 8276 | 11 | Code::CodeKey { rung: Decl, file: pkg/common/draw.go, decl: 2, sub: 0, line: 14 } |  |  | 0.489 |
| walker |  | 8296 | 20 | Code::CodeKey { rung: Decl, file: pkg/common/draw.go, decl: 7, sub: 0, line: 59 } |  |  | 0.489 |
| walker |  | 8325 | 29 | Code::CodeKey { rung: Decl, file: pkg/common/draw.go, decl: 6, sub: 0, line: 52 } |  |  | 0.489 |
| walker |  | 8330 | 5 | Code::CodeKey { rung: Doc, file: pkg/common/draw.go, decl: 2, sub: 0, line: 14 } |  |  | 0.489 |
| walker |  | 8336 | 6 | Code::CodeKey { rung: Doc, file: pkg/common/draw.go, decl: 7, sub: 0, line: 59 } |  |  | 0.489 |
| walker |  | 8344 | 8 | Code::CodeKey { rung: Doc, file: pkg/common/draw.go, decl: 6, sub: 0, line: 52 } |  |  | 0.489 |
| walker |  | 8353 | 9 | Code::CodeKey { rung: Doc, file: pkg/common/draw.go, decl: 13, sub: 0, line: 127 } |  |  | 0.489 |
| walker |  | 8410 | 57 | Code::CodeKey { rung: Decl, file: pkg/common/draw.go, decl: 4, sub: 0, line: 35 } |  |  | 0.510 |
| ns | 8410 |  | 159 | Supporting packages: complete file lists (B) | 6.2 |  | 0.510 |
| walker |  | 8420 | 10 | Code::CodeKey { rung: Doc, file: pkg/common/draw.go, decl: 3, sub: 0, line: 22 } |  |  | 0.510 |
| walker |  | 8430 | 10 | Code::CodeKey { rung: Doc, file: pkg/common/draw.go, decl: 11, sub: 0, line: 98 } |  |  | 0.510 |
| walker |  | 8440 | 10 | Code::CodeKey { rung: Doc, file: pkg/common/draw.go, decl: 14, sub: 0, line: 141 } |  |  | 0.510 |
| walker |  | 8451 | 11 | Code::CodeKey { rung: Doc, file: pkg/common/draw.go, decl: 1, sub: 0, line: 11 } |  |  | 0.510 |
| walker |  | 8462 | 11 | Code::CodeKey { rung: Doc, file: pkg/common/draw.go, decl: 12, sub: 0, line: 110 } |  |  | 0.510 |
| ns | 8554 |  | 144 | Context names the expression interpreter resolves | 6.3 |  | 0.505 |
| walker |  | 8591 | 129 | Code::CodeKey { rung: Decl, file: pkg/common/draw.go, decl: 5, sub: 0, line: 44 } |  |  | 0.505 |
| walker |  | 8607 | 16 | Code::CodeKey { rung: Names, file: pkg/common/outbound_ip.go, decl: 0, sub: 0, line: 0 } |  |  | 0.505 |
| ns | 8679 |  | 125 | Expression functions act implements | 6.4 |  | 0.501 |
| walker |  | 8881 | 274 | Code::CodeKey { rung: Names, file: pkg/model/action.go, decl: 0, sub: 0, line: 0 } |  |  | 0.503 |
| ns | 8903 |  | 224 | Artifact cache server: routes and lifecycle | 6.5 |  | 0.498 |
| walker |  | 8909 | 28 | Code::CodeKey { rung: Decl, file: pkg/model/action.go, decl: 11, sub: 0, line: 127 } |  |  | 0.498 |
| walker |  | 8951 | 42 | Code::CodeKey { rung: Decl, file: pkg/model/action.go, decl: 10, sub: 0, line: 120 } |  |  | 0.498 |
| walker |  | 8963 | 12 | Code::CodeKey { rung: Doc, file: pkg/model/action.go, decl: 12, sub: 0, line: 133 } |  |  | 0.498 |
| walker |  | 8975 | 12 | Code::CodeKey { rung: Body, file: pkg/model/action.go, decl: 5, sub: 0, line: 63 } |  |  | 0.498 |
| ns | 9068 |  | 165 | Artifact server: Serve + the v4 route base | 6.6 |  | 0.494 |
| walker |  | 9071 | 96 | Code::CodeKey { rung: Decl, file: pkg/model/action.go, decl: 3, sub: 0, line: 39 } |  |  | 0.494 |
| walker |  | 9088 | 17 | Code::CodeKey { rung: Doc, file: pkg/model/action.go, decl: 1, sub: 0, line: 13 } |  |  | 0.494 |
| walker |  | 9225 | 137 | Code::CodeKey { rung: Decl, file: pkg/model/action.go, decl: 8, sub: 0, line: 89 } |  |  | 0.494 |
| ns | 9373 |  | 305 | pkg/common: complete exported-function roster | 6.7 |  | 0.488 |
| walker |  | 9431 | 206 | Code::CodeKey { rung: Decl, file: pkg/model/action.go, decl: 7, sub: 0, line: 72 } |  |  | 0.488 |
| walker |  | 9444 | 13 | Code::CodeKey { rung: Doc, file: pkg/model/action.go, decl: 7, sub: 0, line: 72 } |  |  | 0.488 |
| walker |  | 9488 | 44 | Code::CodeKey { rung: Doc, file: pkg/model/action.go, decl: 8, sub: 0, line: 89 } |  |  | 0.488 |
| ns | 9536 |  | 163 | pkg/common/git: repo detection and cloning | 6.8 |  | 0.483 |
| walker |  | 9537 | 49 | Code::CodeKey { rung: Doc, file: pkg/model/action.go, decl: 10, sub: 0, line: 120 } |  |  | 0.483 |
| walker |  | 9610 | 73 | Code::CodeKey { rung: Doc, file: pkg/model/action.go, decl: 11, sub: 0, line: 127 } |  |  | 0.483 |
| ns | 9666 |  | 130 | CI, issue-template and editor directories | 7.1 |  | 0.489 |
| walker |  | 9724 | 114 | Code::CodeKey { rung: Names, file: pkg/common/auth.go, decl: 0, sub: 0, line: 0 } |  |  | 0.491 |
| walker |  | 9731 | 7 | Code::CodeKey { rung: Decl, file: pkg/common/auth.go, decl: 4, sub: 0, line: 33 } |  |  | 0.491 |
| walker |  | 9753 | 22 | Code::CodeKey { rung: Decl, file: pkg/common/auth.go, decl: 2, sub: 0, line: 26 } |  |  | 0.491 |
| walker |  | 9823 | 70 | Code::CodeKey { rung: Decl, file: pkg/common/auth.go, decl: 1, sub: 0, line: 17 } |  |  | 0.491 |
| walker |  | 9899 | 76 | Code::CodeKey { rung: Doc, file: pkg/common/outbound_ip.go, decl: 1, sub: 0, line: 13 } |  |  | 0.491 |
| walker |  | 9910 | 11 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 9, sub: 0, line: 114 } |  |  | 0.491 |
| ns | 9919 |  | 253 | Key direct dependencies | 7.2 |  | 0.500 |
| walker |  | 9995 | 85 | Code::CodeKey { rung: Names, file: pkg/container/docker_socket.go, decl: 0, sub: 0, line: 0 } |  |  | 0.501 |
| walker |  | 9995 | 0 | Code::CodeKey { rung: Decl, file: pkg/container/docker_socket.go, decl: 4, sub: 0, line: 57 } |  |  | 0.501 |

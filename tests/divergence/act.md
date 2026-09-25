Score(3000)=0.573 I=0.856 C=0.383 ns_rows≤3K=18/47 grid(1000/1442/2080/3000/4327/6240/9000)=0.449/0.654/0.571/0.573/0.479/0.472/0.516

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
| walker |  | 489 | 56 | Fs::DirListing { dir: pkg/model } |  |  | 0.571 |
| walker |  | 497 | 8 | Fs::DirListing { dir: .vscode } |  |  | 0.571 |
| walker |  | 507 | 10 | Plaintext::Whole { file: VERSION } |  |  | 0.573 |
| walker |  | 586 | 79 | Fs::DirListing { dir: pkg/common } |  |  | 0.578 |
| ns | 591 |  | 186 | pkg/runner/: complete file list | 1.7 |  | 0.464 |
| walker |  | 595 | 9 | Fs::DirListing { dir: pkg/common/git } |  |  | 0.465 |
| walker |  | 613 | 18 | Fs::DirListing { dir: pkg/artifacts/testdata } |  |  | 0.465 |
| walker |  | 617 | 4 | Fs::DirListing { dir: pkg/artifacts/testdata/GHSL-2023-004 } |  |  | 0.465 |
| walker |  | 621 | 4 | Fs::DirListing { dir: pkg/artifacts/testdata/upload-and-download } |  |  | 0.465 |
| walker |  | 625 | 4 | Fs::DirListing { dir: pkg/artifacts/testdata/v4 } |  |  | 0.465 |
| walker |  | 646 | 21 | Fs::DirListing { dir: pkg/model/testdata } |  |  | 0.465 |
| walker |  | 650 | 4 | Fs::DirListing { dir: pkg/model/testdata/container-volumes } |  |  | 0.465 |
| walker |  | 654 | 4 | Fs::DirListing { dir: pkg/model/testdata/strategy } |  |  | 0.465 |
| walker |  | 660 | 6 | Fs::DirListing { dir: pkg/model/testdata/empty-workflow } |  |  | 0.465 |
| walker |  | 667 | 7 | Fs::DirListing { dir: pkg/model/testdata/nested } |  |  | 0.465 |
| walker |  | 673 | 6 | Fs::DirListing { dir: pkg/model/testdata/nested/workflows } |  |  | 0.465 |
| walker |  | 696 | 23 | Fs::DirListing { dir: pkg/exprparser/testdata } |  |  | 0.465 |
| walker |  | 703 | 7 | Fs::DirListing { dir: pkg/exprparser/testdata/for-hashing-3 } |  |  | 0.465 |
| walker |  | 708 | 5 | Fs::DirListing { dir: pkg/exprparser/testdata/for-hashing-3/nested } |  |  | 0.465 |
| walker |  | 728 | 20 | Fs::DirListing { dir: .github } |  |  | 0.465 |
| walker |  | 754 | 26 | Fs::DirListing { dir: .github/workflows } |  |  | 0.466 |
| ns | 762 |  | 171 | README: how act works, end to end | 1.8 |  | 0.464 |
| ns | 804 |  | 42 | Module path, Go version, released version | 1.9 |  | 0.468 |
| walker |  | 914 | 160 | Fs::DirListing { dir: pkg/container } |  |  | 0.473 |
| ns | 919 |  | 115 | cmd.Execute: CLI entry | 2.1 |  | 0.449 |
| walker |  | 932 | 18 | Code::CodeKey { rung: ModuleDoc, file: pkg/container/docker_cli.go, decl: 0, sub: 0, line: 0 } |  |  | 0.449 |
| walker |  | 969 | 37 | Fs::DirListing { dir: pkg/container/testdata } |  |  | 0.449 |
| walker |  | 973 | 4 | Fs::DirListing { dir: pkg/container/testdata/docker-pull-options } |  |  | 0.449 |
| walker |  | 977 | 4 | Fs::DirListing { dir: pkg/container/testdata/scratch } |  |  | 0.449 |
| ns | 1131 |  | 212 | Root cobra.Command definition | 2.2 |  | 0.428 |
| walker |  | 1162 | 185 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.573 |
| walker |  | 1217 | 55 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.573 |
| walker |  | 1403 | 186 | Fs::DirListing { dir: pkg/runner } |  |  | 0.733 |
| walker |  | 1407 | 4 | Fs::DirListing { dir: pkg/runner/hashfiles } |  |  | 0.733 |
| walker |  | 1411 | 4 | Fs::DirListing { dir: pkg/runner/res } |  |  | 0.733 |
| ns | 1432 |  | 301 | Command-local flags: names and shorthands | 2.3 |  | 0.654 |
| walker |  | 1435 | 24 | Fs::DirListing { dir: pkg/model/testdata/invalid-job-name } |  |  | 0.654 |
| ns | 1782 |  | 350 | Persistent flags: names and shorthands | 2.4 |  | 0.588 |
| ns | 1928 |  | 146 | Input struct + path accessor roster | 2.5 |  | 0.571 |
| walker |  | 2150 | 715 | Fs::DirListing { dir: pkg/runner/testdata } |  |  | 0.571 |
| ns | 2206 |  | 278 | Config-file discovery: .actrc locations | 2.6 |  | 0.540 |
| walker |  | 2215 | 65 | Markdown::HeadingsOutline { file: IMAGES.md } |  |  | 0.540 |
| walker |  | 2243 | 28 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.540 |
| walker |  | 2271 | 28 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.540 |
| walker |  | 2320 | 49 | Markdown::Section { file: IMAGES.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.540 |
| ns | 2522 |  | 316 | cmd package: complete function roster | 2.7 |  | 0.508 |
| walker |  | 2611 | 291 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.508 |
| walker |  | 2636 | 25 | Code::CodeKey { rung: Names, file: main.go, decl: 0, sub: 0, line: 0 } |  |  | 0.516 |
| walker |  | 2646 | 10 | Code::CodeKey { rung: Doc, file: main.go, decl: 1, sub: 0, line: 11 } |  |  | 0.528 |
| walker |  | 2694 | 48 | Code::CodeKey { rung: Body, file: main.go, decl: 2, sub: 0, line: 13 } |  |  | 0.623 |
| walker |  | 2700 | 6 | Plaintext::DeclSurface { file: pkg/exprparser/testdata/for-hashing-1.txt } |  |  | 0.623 |
| walker |  | 2706 | 6 | Plaintext::DeclSurface { file: pkg/exprparser/testdata/for-hashing-2.txt } |  |  | 0.623 |
| walker |  | 2840 | 134 | Code::CodeKey { rung: ModuleDoc, file: pkg/artifactcache/doc.go, decl: 0, sub: 0, line: 0 } |  |  | 0.623 |
| walker |  | 2844 | 4 | Fs::DirListing { dir: .github/actions } |  |  | 0.623 |
| ns | 2869 |  | 347 | pkg/model: complete type roster | 3.1 |  | 0.584 |
| ns | 2979 |  | 110 | Workflow struct | 3.2 | 3.1 | 0.573 |
| ns | 3209 |  | 230 | Planner interface + Plan/Stage/Run | 3.3 | 3.1 | 0.545 |
| ns | 3471 |  | 262 | Job struct: every supported job key | 3.4 | 3.1 | 0.528 |
| walker |  | 3653 | 809 | GoMod::File { file: go.mod } |  |  | 0.529 |
| walker |  | 3660 | 7 | Plaintext::DeclSurface { file: pkg/container/testdata/scratch/test.txt } |  |  | 0.529 |
| ns | 3695 |  | 224 | Step struct: every supported step key | 3.5 | 3.1 | 0.516 |
| walker |  | 3727 | 67 | Plaintext::Whole { file: pkg/container/testdata/Dockerfile } |  |  | 0.516 |
| walker |  | 3735 | 8 | Plaintext::DeclSurface { file: pkg/exprparser/testdata/for-hashing-3/data.txt } |  |  | 0.516 |
| walker |  | 3848 | 113 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.516 |
| walker |  | 3995 | 147 | Code::CodeKey { rung: Names, file: cmd/notices.go, decl: 0, sub: 0, line: 0 } |  |  | 0.519 |
| walker |  | 4022 | 27 | Code::CodeKey { rung: Decl, file: cmd/notices.go, decl: 1, sub: 0, line: 17 } |  |  | 0.519 |
| ns | 4027 |  | 332 | StepType constants | 3.6 | 3.1 | 0.500 |
| walker |  | 4051 | 29 | Code::CodeKey { rung: Body, file: cmd/notices.go, decl: 4, sub: 0, line: 57 } |  |  | 0.500 |
| walker |  | 4118 | 67 | Code::CodeKey { rung: Body, file: cmd/notices.go, decl: 9, sub: 0, line: 138 } |  |  | 0.500 |
| walker |  | 4195 | 77 | Code::CodeKey { rung: Body, file: cmd/notices.go, decl: 7, sub: 0, line: 121 } |  |  | 0.500 |
| ns | 4248 |  | 221 | GithubContext: complete field-name roster | 3.7 | 3.1 | 0.479 |
| walker |  | 4345 | 150 | Code::CodeKey { rung: Names, file: cmd/input.go, decl: 0, sub: 0, line: 0 } |  |  | 0.495 |
| walker |  | 4354 | 9 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 6, sub: 0, line: 99 } |  |  | 0.495 |
| walker |  | 4365 | 11 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 3, sub: 0, line: 85 } |  |  | 0.495 |
| walker |  | 4376 | 11 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 4, sub: 0, line: 90 } |  |  | 0.495 |
| walker |  | 4387 | 11 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 5, sub: 0, line: 94 } |  |  | 0.495 |
| walker |  | 4399 | 12 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 4, sub: 0, line: 90 } |  |  | 0.495 |
| walker |  | 4412 | 13 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 3, sub: 0, line: 85 } |  |  | 0.495 |
| walker |  | 4425 | 13 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 6, sub: 0, line: 99 } |  |  | 0.495 |
| walker |  | 4439 | 14 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 8, sub: 0, line: 109 } |  |  | 0.495 |
| walker |  | 4454 | 15 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 7, sub: 0, line: 104 } |  |  | 0.495 |
| walker |  | 4469 | 15 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 9, sub: 0, line: 114 } |  |  | 0.495 |
| walker |  | 4480 | 11 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 8, sub: 0, line: 109 } |  |  | 0.495 |
| ns | 4531 |  | 283 | Executor type + complete combinator roster | 4.1 |  | 0.481 |
| walker |  | 5061 | 581 | Code::CodeKey { rung: Decl, file: cmd/input.go, decl: 1, sub: 0, line: 10 } |  |  | 0.482 |
| ns | 5066 |  | 535 | pkg/runner: complete type roster | 4.2 |  | 0.461 |
| walker |  | 5075 | 14 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 1, sub: 0, line: 10 } |  |  | 0.466 |
| walker |  | 5082 | 7 | Plaintext::DeclSurface { file: pkg/exprparser/testdata/for-hashing-3/nested/nested-data.txt } |  |  | 0.466 |
| walker |  | 5116 | 34 | Code::CodeKey { rung: Names, file: cmd/dir.go, decl: 0, sub: 0, line: 0 } |  |  | 0.467 |
| walker |  | 5125 | 9 | Code::CodeKey { rung: Decl, file: cmd/dir.go, decl: 1, sub: 0, line: 10 } |  |  | 0.468 |
| walker |  | 5176 | 51 | Code::CodeKey { rung: Names, file: cmd/secrets.go, decl: 0, sub: 0, line: 0 } |  |  | 0.464 |
| ns | 5176 |  | 110 | Runner interface and New() | 4.3 | 4.2 | 0.464 |
| walker |  | 5183 | 7 | Code::CodeKey { rung: Body, file: cmd/secrets.go, decl: 3, sub: 0, line: 40 } |  |  | 0.464 |
| walker |  | 5201 | 18 | Code::CodeKey { rung: Names, file: cmd/graph.go, decl: 0, sub: 0, line: 0 } |  |  | 0.466 |
| walker |  | 5219 | 18 | Code::CodeKey { rung: Names, file: cmd/list.go, decl: 0, sub: 0, line: 0 } |  |  | 0.467 |
| ns | 5564 |  | 388 | runner.Config: fields (first half, rest elided) | 4.4 | 4.2 | 0.454 |
| walker |  | 5593 | 374 | Code::CodeKey { rung: Names, file: cmd/root.go, decl: 0, sub: 0, line: 0 } |  |  | 0.490 |
| walker |  | 5643 | 50 | Code::CodeKey { rung: Decl, file: cmd/root.go, decl: 1, sub: 0, line: 37 } |  |  | 0.490 |
| walker |  | 5655 | 12 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 18, sub: 0, line: 391 } |  |  | 0.490 |
| walker |  | 5668 | 13 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 3, sub: 0, line: 47 } |  |  | 0.492 |
| walker |  | 5684 | 16 | Code::CodeKey { rung: Body, file: cmd/root.go, decl: 15, sub: 0, line: 345 } |  |  | 0.492 |
| walker |  | 5719 | 35 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 5, sub: 0, line: 138 } |  |  | 0.494 |
| walker |  | 5751 | 32 | Code::CodeKey { rung: Body, file: cmd/root.go, decl: 12, sub: 0, line: 314 } |  |  | 0.494 |
| walker |  | 5828 | 77 | Code::CodeKey { rung: Body, file: cmd/notices.go, decl: 8, sub: 0, line: 130 } |  |  | 0.494 |
| ns | 5866 |  | 302 | RunContext struct | 4.5 | 4.2 | 0.480 |
| walker |  | 5945 | 117 | Code::CodeKey { rung: Body, file: cmd/dir.go, decl: 2, sub: 0, line: 15 } |  |  | 0.480 |
| walker |  | 5966 | 21 | Code::CodeKey { rung: Names, file: cmd/platforms.go, decl: 0, sub: 0, line: 0 } |  |  | 0.486 |
| walker |  | 5979 | 13 | Code::CodeKey { rung: Names, file: pkg/model/job_context.go, decl: 0, sub: 0, line: 0 } |  |  | 0.486 |
| walker |  | 6073 | 94 | Code::CodeKey { rung: Decl, file: pkg/model/job_context.go, decl: 1, sub: 0, line: 3 } |  |  | 0.486 |
| walker |  | 6113 | 40 | Code::CodeKey { rung: Names, file: pkg/artifactcache/model.go, decl: 0, sub: 0, line: 0 } |  |  | 0.486 |
| walker |  | 6156 | 43 | Code::CodeKey { rung: Decl, file: pkg/artifactcache/model.go, decl: 1, sub: 0, line: 3 } |  |  | 0.486 |
| ns | 6221 |  | 355 | stepFactory: StepType -> implementation | 4.6 | 4.2 | 0.472 |
| walker |  | 6303 | 147 | Code::CodeKey { rung: Decl, file: pkg/artifactcache/model.go, decl: 3, sub: 0, line: 26 } |  |  | 0.472 |
| walker |  | 6314 | 11 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 9, sub: 0, line: 114 } |  |  | 0.472 |
| walker |  | 6328 | 14 | Code::CodeKey { rung: Names, file: pkg/container/executions_environment.go, decl: 0, sub: 0, line: 0 } |  |  | 0.472 |
| walker |  | 6436 | 108 | Code::CodeKey { rung: Decl, file: pkg/container/executions_environment.go, decl: 1, sub: 0, line: 5 } |  |  | 0.472 |
| walker |  | 6464 | 28 | Code::CodeKey { rung: Names, file: pkg/lookpath/error.go, decl: 0, sub: 0, line: 0 } |  |  | 0.472 |
| walker |  | 6482 | 18 | Code::CodeKey { rung: Decl, file: pkg/lookpath/error.go, decl: 1, sub: 0, line: 3 } |  |  | 0.472 |
| walker |  | 6491 | 9 | Code::CodeKey { rung: Body, file: pkg/lookpath/error.go, decl: 2, sub: 0, line: 8 } |  |  | 0.472 |
| ns | 6499 |  | 278 | step interface and stage enum | 4.7 | 4.2 | 0.458 |
| walker |  | 6729 | 238 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.460 |
| walker |  | 6788 | 59 | Code::CodeKey { rung: Names, file: pkg/lookpath/env.go, decl: 0, sub: 0, line: 0 } |  |  | 0.460 |
| walker |  | 6791 | 3 | Code::CodeKey { rung: Decl, file: pkg/lookpath/env.go, decl: 2, sub: 0, line: 9 } |  |  | 0.460 |
| walker |  | 6805 | 14 | Code::CodeKey { rung: Decl, file: pkg/lookpath/env.go, decl: 1, sub: 0, line: 5 } |  |  | 0.460 |
| walker |  | 6814 | 9 | Code::CodeKey { rung: Body, file: pkg/lookpath/env.go, decl: 3, sub: 0, line: 12 } |  |  | 0.460 |
| walker |  | 6828 | 14 | Code::CodeKey { rung: Body, file: pkg/lookpath/env.go, decl: 4, sub: 0, line: 16 } |  |  | 0.460 |
| walker |  | 6932 | 104 | Code::CodeKey { rung: Names, file: pkg/container/container_types.go, decl: 0, sub: 0, line: 0 } |  |  | 0.460 |
| walker |  | 6939 | 7 | Code::CodeKey { rung: Decl, file: pkg/container/container_types.go, decl: 7, sub: 0, line: 80 } |  |  | 0.460 |
| walker |  | 6968 | 29 | Code::CodeKey { rung: Decl, file: pkg/container/container_types.go, decl: 2, sub: 0, line: 36 } |  |  | 0.460 |
| ns | 6994 |  | 495 | run_context.go: method roster (container lifecycle) | 4.8 |  | 0.447 |
| walker |  | 7015 | 47 | Code::CodeKey { rung: Decl, file: pkg/container/container_types.go, decl: 5, sub: 0, line: 70 } |  |  | 0.447 |
| walker |  | 7067 | 52 | Code::CodeKey { rung: Decl, file: pkg/container/container_types.go, decl: 4, sub: 0, line: 61 } |  |  | 0.447 |
| walker |  | 7083 | 16 | Code::CodeKey { rung: Doc, file: pkg/container/container_types.go, decl: 2, sub: 0, line: 36 } |  |  | 0.447 |
| walker |  | 7101 | 18 | Code::CodeKey { rung: Doc, file: pkg/container/container_types.go, decl: 5, sub: 0, line: 70 } |  |  | 0.447 |
| walker |  | 7121 | 20 | Code::CodeKey { rung: Doc, file: pkg/container/container_types.go, decl: 4, sub: 0, line: 61 } |  |  | 0.447 |
| ns | 7140 |  | 146 | ActionCache implementations | 4.9 | 4.2 | 0.439 |
| ns | 7300 |  | 160 | pkg/container: complete file list | 5.1 |  | 0.465 |
| walker |  | 7327 | 206 | Code::CodeKey { rung: Decl, file: pkg/container/container_types.go, decl: 1, sub: 0, line: 12 } |  |  | 0.465 |
| walker |  | 7342 | 15 | Code::CodeKey { rung: Doc, file: pkg/container/container_types.go, decl: 1, sub: 0, line: 12 } |  |  | 0.465 |
| ns | 7577 |  | 277 | Container interface | 5.2 |  | 0.459 |
| walker |  | 7588 | 246 | Code::CodeKey { rung: Decl, file: pkg/container/container_types.go, decl: 3, sub: 0, line: 43 } |  |  | 0.478 |
| walker |  | 7598 | 10 | Code::CodeKey { rung: Doc, file: pkg/container/container_types.go, decl: 3, sub: 0, line: 43 } |  |  | 0.480 |
| walker |  | 7659 | 61 | Code::CodeKey { rung: Body, file: cmd/root.go, decl: 3, sub: 0, line: 47 } |  |  | 0.491 |
| walker |  | 7675 | 16 | Code::CodeKey { rung: Names, file: pkg/common/outbound_ip.go, decl: 0, sub: 0, line: 0 } |  |  | 0.491 |
| ns | 7719 |  | 142 | ExecutionsEnvironment interface | 5.3 |  | 0.496 |
| walker |  | 7751 | 76 | Code::CodeKey { rung: Doc, file: pkg/common/outbound_ip.go, decl: 1, sub: 0, line: 13 } |  |  | 0.496 |
| walker |  | 7836 | 85 | Code::CodeKey { rung: Names, file: pkg/container/docker_socket.go, decl: 0, sub: 0, line: 0 } |  |  | 0.496 |
| walker |  | 7856 | 20 | Code::CodeKey { rung: Decl, file: pkg/container/docker_socket.go, decl: 4, sub: 0, line: 57 } |  |  | 0.496 |
| walker |  | 7871 | 15 | Code::CodeKey { rung: Doc, file: pkg/container/docker_socket.go, decl: 2, sub: 0, line: 23 } |  |  | 0.496 |
| walker |  | 7970 | 99 | Code::CodeKey { rung: Decl, file: pkg/container/docker_socket.go, decl: 1, sub: 0, line: 12 } |  |  | 0.497 |
| ns | 7984 |  | 265 | Docker backend vs docker-less stub: the build-tag split | 5.4 |  | 0.489 |
| walker |  | 8055 | 85 | Code::CodeKey { rung: Names, file: pkg/workflowpattern/trace_writer.go, decl: 0, sub: 0, line: 0 } |  |  | 0.489 |
| walker |  | 8058 | 3 | Code::CodeKey { rung: Decl, file: pkg/workflowpattern/trace_writer.go, decl: 3, sub: 0, line: 11 } |  |  | 0.489 |
| walker |  | 8072 | 14 | Code::CodeKey { rung: Decl, file: pkg/workflowpattern/trace_writer.go, decl: 1, sub: 0, line: 5 } |  |  | 0.489 |
| walker |  | 8084 | 12 | Code::CodeKey { rung: Body, file: pkg/workflowpattern/trace_writer.go, decl: 5, sub: 0, line: 16 } |  |  | 0.489 |
| walker |  | 8096 | 12 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 7, sub: 0, line: 104 } |  |  | 0.489 |
| ns | 8161 |  | 177 | Docker socket discovery | 5.5 |  | 0.498 |
| ns | 8251 |  | 90 | Supporting packages: complete file lists (A) | 6.1 |  | 0.511 |
| walker |  | 8294 | 198 | Code::CodeKey { rung: Body, file: cmd/platforms.go, decl: 1, sub: 0, line: 7 } |  |  | 0.511 |
| ns | 8410 |  | 159 | Supporting packages: complete file lists (B) | 6.2 |  | 0.530 |
| ns | 8554 |  | 144 | Context names the expression interpreter resolves | 6.3 |  | 0.525 |
| walker |  | 8601 | 307 | Code::CodeKey { rung: Names, file: pkg/artifacts/server.go, decl: 0, sub: 0, line: 0 } |  |  | 0.525 |
| walker |  | 8604 | 3 | Code::CodeKey { rung: Decl, file: pkg/artifacts/server.go, decl: 9, sub: 0, line: 58 } |  |  | 0.525 |
| walker |  | 8615 | 11 | Code::CodeKey { rung: Decl, file: pkg/artifacts/server.go, decl: 7, sub: 0, line: 49 } |  |  | 0.525 |
| walker |  | 8629 | 14 | Code::CodeKey { rung: Decl, file: pkg/artifacts/server.go, decl: 6, sub: 0, line: 45 } |  |  | 0.525 |
| walker |  | 8645 | 16 | Code::CodeKey { rung: Decl, file: pkg/artifacts/server.go, decl: 5, sub: 0, line: 41 } |  |  | 0.525 |
| walker |  | 8665 | 20 | Code::CodeKey { rung: Decl, file: pkg/artifacts/server.go, decl: 1, sub: 0, line: 21 } |  |  | 0.525 |
| ns | 8679 |  | 125 | Expression functions act implements | 6.4 |  | 0.521 |
| walker |  | 8697 | 32 | Code::CodeKey { rung: Decl, file: pkg/artifacts/server.go, decl: 2, sub: 0, line: 25 } |  |  | 0.521 |
| walker |  | 8729 | 32 | Code::CodeKey { rung: Decl, file: pkg/artifacts/server.go, decl: 3, sub: 0, line: 30 } |  |  | 0.521 |
| walker |  | 8765 | 36 | Code::CodeKey { rung: Decl, file: pkg/artifacts/server.go, decl: 8, sub: 0, line: 53 } |  |  | 0.521 |
| walker |  | 8807 | 42 | Code::CodeKey { rung: Decl, file: pkg/artifacts/server.go, decl: 4, sub: 0, line: 35 } |  |  | 0.521 |
| walker |  | 8816 | 9 | Code::CodeKey { rung: Body, file: pkg/artifacts/server.go, decl: 10, sub: 0, line: 61 } |  |  | 0.521 |
| ns | 8903 |  | 224 | Artifact cache server: routes and lifecycle | 6.5 |  | 0.516 |
| ns | 9068 |  | 165 | Artifact server: Serve + the v4 route base | 6.6 |  | 0.512 |
| walker |  | 9180 | 364 | Code::CodeKey { rung: Names, file: pkg/model/planner.go, decl: 0, sub: 0, line: 0 } |  |  | 0.515 |
| walker |  | 9194 | 14 | Code::CodeKey { rung: Decl, file: pkg/model/planner.go, decl: 2, sub: 0, line: 25 } |  |  | 0.515 |
| walker |  | 9208 | 14 | Code::CodeKey { rung: Decl, file: pkg/model/planner.go, decl: 3, sub: 0, line: 30 } |  |  | 0.516 |
| walker |  | 9222 | 14 | Code::CodeKey { rung: Decl, file: pkg/model/planner.go, decl: 11, sub: 0, line: 196 } |  |  | 0.516 |
| walker |  | 9244 | 22 | Code::CodeKey { rung: Decl, file: pkg/model/planner.go, decl: 4, sub: 0, line: 35 } |  |  | 0.517 |
| walker |  | 9270 | 26 | Code::CodeKey { rung: Decl, file: pkg/model/planner.go, decl: 7, sub: 0, line: 53 } |  |  | 0.517 |
| walker |  | 9330 | 60 | Code::CodeKey { rung: Decl, file: pkg/model/planner.go, decl: 1, sub: 0, line: 17 } |  |  | 0.522 |
| walker |  | 9343 | 13 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 1, sub: 0, line: 17 } |  |  | 0.525 |
| walker |  | 9356 | 13 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 6, sub: 0, line: 49 } |  |  | 0.526 |
| walker |  | 9369 | 13 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 18, sub: 0, line: 327 } |  |  | 0.526 |
| ns | 9373 |  | 305 | pkg/common: complete exported-function roster | 6.7 |  | 0.518 |
| walker |  | 9383 | 14 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 2, sub: 0, line: 25 } |  |  | 0.521 |
| walker |  | 9397 | 14 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 3, sub: 0, line: 30 } |  |  | 0.523 |
| walker |  | 9413 | 16 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 4, sub: 0, line: 35 } |  |  | 0.526 |
| walker |  | 9429 | 16 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 15, sub: 0, line: 274 } |  |  | 0.526 |
| walker |  | 9446 | 17 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 14, sub: 0, line: 252 } |  |  | 0.526 |
| walker |  | 9464 | 18 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 16, sub: 0, line: 304 } |  |  | 0.526 |
| walker |  | 9482 | 18 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 17, sub: 0, line: 318 } |  |  | 0.526 |
| walker |  | 9502 | 20 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 12, sub: 0, line: 201 } |  |  | 0.526 |
| walker |  | 9522 | 20 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 13, sub: 0, line: 232 } |  |  | 0.526 |
| ns | 9536 |  | 163 | pkg/common/git: repo detection and cloning | 6.8 |  | 0.521 |
| walker |  | 9544 | 22 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 20, sub: 0, line: 384 } |  |  | 0.521 |
| walker |  | 9572 | 28 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 8, sub: 0, line: 59 } |  |  | 0.521 |
| ns | 9666 |  | 130 | CI, issue-template and editor directories | 7.1 |  | 0.525 |
| ns | 9919 |  | 253 | Key direct dependencies | 7.2 |  | 0.534 |
| walker |  | 9937 | 365 | Code::CodeKey { rung: Names, file: pkg/common/executor.go, decl: 0, sub: 0, line: 0 } |  |  | 0.548 |
| walker |  | 9949 | 12 | Code::CodeKey { rung: Decl, file: pkg/common/executor.go, decl: 1, sub: 0, line: 12 } |  |  | 0.548 |
| walker |  | 9957 | 8 | Code::CodeKey { rung: Body, file: pkg/common/executor.go, decl: 2, sub: 0, line: 17 } |  |  | 0.548 |
| walker |  | 9966 | 9 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 2, sub: 0, line: 17 } |  |  | 0.548 |
| walker |  | 9977 | 11 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 3, sub: 0, line: 22 } |  |  | 0.548 |
| walker |  | 9988 | 11 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 5, sub: 0, line: 33 } |  |  | 0.550 |
| walker |  | 9999 | 11 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 20, sub: 0, line: 237 } |  |  | 0.550 |

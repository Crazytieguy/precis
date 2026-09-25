Score(3000)=0.573 I=0.856 C=0.383 ns_rows≤3K=18/47 grid(1000/1442/2080/3000/4327/6240/9000)=0.449/0.654/0.571/0.573/0.495/0.486/0.539

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
| walker |  | 2296 | 25 | Code::CodeKey { rung: Names, file: main.go, decl: 0, sub: 0, line: 0 } |  |  | 0.550 |
| walker |  | 2306 | 10 | Code::CodeKey { rung: Doc, file: main.go, decl: 1, sub: 0, line: 11 } |  |  | 0.562 |
| walker |  | 2354 | 48 | Code::CodeKey { rung: Body, file: main.go, decl: 2, sub: 0, line: 13 } |  |  | 0.663 |
| walker |  | 2403 | 49 | Markdown::Section { file: IMAGES.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.663 |
| ns | 2522 |  | 316 | cmd package: complete function roster | 2.7 |  | 0.623 |
| walker |  | 2694 | 291 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.623 |
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
| walker |  | 4172 | 150 | Code::CodeKey { rung: Names, file: cmd/input.go, decl: 0, sub: 0, line: 0 } |  |  | 0.517 |
| walker |  | 4181 | 9 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 6, sub: 0, line: 99 } |  |  | 0.517 |
| walker |  | 4192 | 11 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 3, sub: 0, line: 85 } |  |  | 0.517 |
| walker |  | 4203 | 11 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 4, sub: 0, line: 90 } |  |  | 0.517 |
| walker |  | 4214 | 11 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 5, sub: 0, line: 94 } |  |  | 0.517 |
| walker |  | 4226 | 12 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 4, sub: 0, line: 90 } |  |  | 0.517 |
| walker |  | 4239 | 13 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 3, sub: 0, line: 85 } |  |  | 0.517 |
| ns | 4248 |  | 221 | GithubContext: complete field-name roster | 3.7 | 3.1 | 0.495 |
| walker |  | 4252 | 13 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 6, sub: 0, line: 99 } |  |  | 0.495 |
| walker |  | 4266 | 14 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 8, sub: 0, line: 109 } |  |  | 0.495 |
| walker |  | 4281 | 15 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 7, sub: 0, line: 104 } |  |  | 0.495 |
| walker |  | 4296 | 15 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 9, sub: 0, line: 114 } |  |  | 0.495 |
| walker |  | 4307 | 11 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 8, sub: 0, line: 109 } |  |  | 0.495 |
| ns | 4531 |  | 283 | Executor type + complete combinator roster | 4.1 |  | 0.481 |
| walker |  | 4888 | 581 | Code::CodeKey { rung: Decl, file: cmd/input.go, decl: 1, sub: 0, line: 10 } |  |  | 0.482 |
| walker |  | 4902 | 14 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 1, sub: 0, line: 10 } |  |  | 0.488 |
| walker |  | 4909 | 7 | Plaintext::DeclSurface { file: pkg/exprparser/testdata/for-hashing-3/nested/nested-data.txt } |  |  | 0.488 |
| walker |  | 4943 | 34 | Code::CodeKey { rung: Names, file: cmd/dir.go, decl: 0, sub: 0, line: 0 } |  |  | 0.489 |
| walker |  | 4952 | 9 | Code::CodeKey { rung: Decl, file: cmd/dir.go, decl: 1, sub: 0, line: 10 } |  |  | 0.490 |
| walker |  | 5003 | 51 | Code::CodeKey { rung: Names, file: cmd/secrets.go, decl: 0, sub: 0, line: 0 } |  |  | 0.493 |
| walker |  | 5010 | 7 | Code::CodeKey { rung: Body, file: cmd/secrets.go, decl: 3, sub: 0, line: 40 } |  |  | 0.493 |
| walker |  | 5039 | 29 | Code::CodeKey { rung: Body, file: cmd/notices.go, decl: 4, sub: 0, line: 57 } |  |  | 0.493 |
| walker |  | 5057 | 18 | Code::CodeKey { rung: Names, file: cmd/graph.go, decl: 0, sub: 0, line: 0 } |  |  | 0.494 |
| ns | 5066 |  | 535 | pkg/runner: complete type roster | 4.2 |  | 0.472 |
| walker |  | 5075 | 18 | Code::CodeKey { rung: Names, file: cmd/list.go, decl: 0, sub: 0, line: 0 } |  |  | 0.474 |
| ns | 5176 |  | 110 | Runner interface and New() | 4.3 | 4.2 | 0.467 |
| walker |  | 5449 | 374 | Code::CodeKey { rung: Names, file: cmd/root.go, decl: 0, sub: 0, line: 0 } |  |  | 0.504 |
| walker |  | 5499 | 50 | Code::CodeKey { rung: Decl, file: cmd/root.go, decl: 1, sub: 0, line: 37 } |  |  | 0.504 |
| walker |  | 5512 | 13 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 3, sub: 0, line: 47 } |  |  | 0.505 |
| walker |  | 5524 | 12 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 18, sub: 0, line: 391 } |  |  | 0.505 |
| walker |  | 5540 | 16 | Code::CodeKey { rung: Body, file: cmd/root.go, decl: 15, sub: 0, line: 345 } |  |  | 0.505 |
| ns | 5564 |  | 388 | runner.Config: fields (first half, rest elided) | 4.4 | 4.2 | 0.492 |
| walker |  | 5601 | 61 | Code::CodeKey { rung: Body, file: cmd/root.go, decl: 3, sub: 0, line: 47 } |  |  | 0.506 |
| walker |  | 5636 | 35 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 5, sub: 0, line: 138 } |  |  | 0.508 |
| walker |  | 5657 | 21 | Code::CodeKey { rung: Names, file: cmd/platforms.go, decl: 0, sub: 0, line: 0 } |  |  | 0.514 |
| walker |  | 5670 | 13 | Code::CodeKey { rung: Names, file: pkg/model/job_context.go, decl: 0, sub: 0, line: 0 } |  |  | 0.515 |
| walker |  | 5764 | 94 | Code::CodeKey { rung: Decl, file: pkg/model/job_context.go, decl: 1, sub: 0, line: 3 } |  |  | 0.515 |
| walker |  | 5804 | 40 | Code::CodeKey { rung: Names, file: pkg/artifactcache/model.go, decl: 0, sub: 0, line: 0 } |  |  | 0.515 |
| walker |  | 5847 | 43 | Code::CodeKey { rung: Decl, file: pkg/artifactcache/model.go, decl: 1, sub: 0, line: 3 } |  |  | 0.515 |
| ns | 5866 |  | 302 | RunContext struct | 4.5 | 4.2 | 0.500 |
| walker |  | 5994 | 147 | Code::CodeKey { rung: Decl, file: pkg/artifactcache/model.go, decl: 3, sub: 0, line: 26 } |  |  | 0.500 |
| walker |  | 6005 | 11 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 9, sub: 0, line: 114 } |  |  | 0.500 |
| walker |  | 6019 | 14 | Code::CodeKey { rung: Names, file: pkg/container/executions_environment.go, decl: 0, sub: 0, line: 0 } |  |  | 0.500 |
| walker |  | 6127 | 108 | Code::CodeKey { rung: Decl, file: pkg/container/executions_environment.go, decl: 1, sub: 0, line: 5 } |  |  | 0.501 |
| walker |  | 6155 | 28 | Code::CodeKey { rung: Names, file: pkg/lookpath/error.go, decl: 0, sub: 0, line: 0 } |  |  | 0.501 |
| walker |  | 6173 | 18 | Code::CodeKey { rung: Decl, file: pkg/lookpath/error.go, decl: 1, sub: 0, line: 3 } |  |  | 0.501 |
| walker |  | 6182 | 9 | Code::CodeKey { rung: Body, file: pkg/lookpath/error.go, decl: 2, sub: 0, line: 8 } |  |  | 0.501 |
| ns | 6221 |  | 355 | stepFactory: StepType -> implementation | 4.6 | 4.2 | 0.486 |
| walker |  | 6420 | 238 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.488 |
| walker |  | 6479 | 59 | Code::CodeKey { rung: Names, file: pkg/lookpath/env.go, decl: 0, sub: 0, line: 0 } |  |  | 0.488 |
| walker |  | 6482 | 3 | Code::CodeKey { rung: Decl, file: pkg/lookpath/env.go, decl: 2, sub: 0, line: 9 } |  |  | 0.488 |
| walker |  | 6496 | 14 | Code::CodeKey { rung: Decl, file: pkg/lookpath/env.go, decl: 1, sub: 0, line: 5 } |  |  | 0.488 |
| ns | 6499 |  | 278 | step interface and stage enum | 4.7 | 4.2 | 0.473 |
| walker |  | 6510 | 14 | Code::CodeKey { rung: Body, file: pkg/lookpath/env.go, decl: 4, sub: 0, line: 16 } |  |  | 0.473 |
| walker |  | 6519 | 9 | Code::CodeKey { rung: Body, file: pkg/lookpath/env.go, decl: 3, sub: 0, line: 12 } |  |  | 0.473 |
| walker |  | 6623 | 104 | Code::CodeKey { rung: Names, file: pkg/container/container_types.go, decl: 0, sub: 0, line: 0 } |  |  | 0.473 |
| walker |  | 6630 | 7 | Code::CodeKey { rung: Decl, file: pkg/container/container_types.go, decl: 7, sub: 0, line: 80 } |  |  | 0.473 |
| walker |  | 6659 | 29 | Code::CodeKey { rung: Decl, file: pkg/container/container_types.go, decl: 2, sub: 0, line: 36 } |  |  | 0.473 |
| walker |  | 6706 | 47 | Code::CodeKey { rung: Decl, file: pkg/container/container_types.go, decl: 5, sub: 0, line: 70 } |  |  | 0.473 |
| walker |  | 6758 | 52 | Code::CodeKey { rung: Decl, file: pkg/container/container_types.go, decl: 4, sub: 0, line: 61 } |  |  | 0.473 |
| walker |  | 6774 | 16 | Code::CodeKey { rung: Doc, file: pkg/container/container_types.go, decl: 2, sub: 0, line: 36 } |  |  | 0.473 |
| walker |  | 6792 | 18 | Code::CodeKey { rung: Doc, file: pkg/container/container_types.go, decl: 5, sub: 0, line: 70 } |  |  | 0.473 |
| walker |  | 6812 | 20 | Code::CodeKey { rung: Doc, file: pkg/container/container_types.go, decl: 4, sub: 0, line: 61 } |  |  | 0.473 |
| ns | 6994 |  | 495 | run_context.go: method roster (container lifecycle) | 4.8 |  | 0.459 |
| walker |  | 7018 | 206 | Code::CodeKey { rung: Decl, file: pkg/container/container_types.go, decl: 1, sub: 0, line: 12 } |  |  | 0.459 |
| walker |  | 7033 | 15 | Code::CodeKey { rung: Doc, file: pkg/container/container_types.go, decl: 1, sub: 0, line: 12 } |  |  | 0.459 |
| ns | 7140 |  | 146 | ActionCache implementations | 4.9 | 4.2 | 0.451 |
| walker |  | 7279 | 246 | Code::CodeKey { rung: Decl, file: pkg/container/container_types.go, decl: 3, sub: 0, line: 43 } |  |  | 0.452 |
| walker |  | 7289 | 10 | Code::CodeKey { rung: Doc, file: pkg/container/container_types.go, decl: 3, sub: 0, line: 43 } |  |  | 0.452 |
| ns | 7300 |  | 160 | pkg/container: complete file list | 5.1 |  | 0.478 |
| walker |  | 7305 | 16 | Code::CodeKey { rung: Names, file: pkg/common/outbound_ip.go, decl: 0, sub: 0, line: 0 } |  |  | 0.478 |
| walker |  | 7381 | 76 | Code::CodeKey { rung: Doc, file: pkg/common/outbound_ip.go, decl: 1, sub: 0, line: 13 } |  |  | 0.478 |
| walker |  | 7466 | 85 | Code::CodeKey { rung: Names, file: pkg/container/docker_socket.go, decl: 0, sub: 0, line: 0 } |  |  | 0.478 |
| walker |  | 7486 | 20 | Code::CodeKey { rung: Decl, file: pkg/container/docker_socket.go, decl: 4, sub: 0, line: 57 } |  |  | 0.478 |
| ns | 7577 |  | 277 | Container interface | 5.2 |  | 0.491 |
| walker |  | 7587 | 101 | Code::CodeKey { rung: Decl, file: pkg/container/docker_socket.go, decl: 1, sub: 0, line: 12 } |  |  | 0.492 |
| walker |  | 7600 | 13 | Code::CodeKey { rung: Doc, file: pkg/container/docker_socket.go, decl: 2, sub: 0, line: 23 } |  |  | 0.492 |
| walker |  | 7685 | 85 | Code::CodeKey { rung: Names, file: pkg/workflowpattern/trace_writer.go, decl: 0, sub: 0, line: 0 } |  |  | 0.492 |
| walker |  | 7688 | 3 | Code::CodeKey { rung: Decl, file: pkg/workflowpattern/trace_writer.go, decl: 3, sub: 0, line: 11 } |  |  | 0.492 |
| walker |  | 7702 | 14 | Code::CodeKey { rung: Decl, file: pkg/workflowpattern/trace_writer.go, decl: 1, sub: 0, line: 5 } |  |  | 0.492 |
| walker |  | 7714 | 12 | Code::CodeKey { rung: Body, file: pkg/workflowpattern/trace_writer.go, decl: 5, sub: 0, line: 16 } |  |  | 0.492 |
| ns | 7719 |  | 142 | ExecutionsEnvironment interface | 5.3 |  | 0.497 |
| walker |  | 7726 | 12 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 7, sub: 0, line: 104 } |  |  | 0.497 |
| ns | 7984 |  | 265 | Docker backend vs docker-less stub: the build-tag split | 5.4 |  | 0.489 |
| walker |  | 8033 | 307 | Code::CodeKey { rung: Names, file: pkg/artifacts/server.go, decl: 0, sub: 0, line: 0 } |  |  | 0.489 |
| walker |  | 8036 | 3 | Code::CodeKey { rung: Decl, file: pkg/artifacts/server.go, decl: 9, sub: 0, line: 58 } |  |  | 0.489 |
| walker |  | 8047 | 11 | Code::CodeKey { rung: Decl, file: pkg/artifacts/server.go, decl: 7, sub: 0, line: 49 } |  |  | 0.489 |
| walker |  | 8061 | 14 | Code::CodeKey { rung: Decl, file: pkg/artifacts/server.go, decl: 6, sub: 0, line: 45 } |  |  | 0.489 |
| walker |  | 8077 | 16 | Code::CodeKey { rung: Decl, file: pkg/artifacts/server.go, decl: 5, sub: 0, line: 41 } |  |  | 0.489 |
| walker |  | 8097 | 20 | Code::CodeKey { rung: Decl, file: pkg/artifacts/server.go, decl: 1, sub: 0, line: 21 } |  |  | 0.489 |
| walker |  | 8129 | 32 | Code::CodeKey { rung: Decl, file: pkg/artifacts/server.go, decl: 2, sub: 0, line: 25 } |  |  | 0.489 |
| walker |  | 8161 | 32 | Code::CodeKey { rung: Decl, file: pkg/artifacts/server.go, decl: 3, sub: 0, line: 30 } |  |  | 0.498 |
| ns | 8161 |  | 177 | Docker socket discovery | 5.5 |  | 0.498 |
| walker |  | 8197 | 36 | Code::CodeKey { rung: Decl, file: pkg/artifacts/server.go, decl: 8, sub: 0, line: 53 } |  |  | 0.498 |
| walker |  | 8239 | 42 | Code::CodeKey { rung: Decl, file: pkg/artifacts/server.go, decl: 4, sub: 0, line: 35 } |  |  | 0.498 |
| ns | 8251 |  | 90 | Supporting packages: complete file lists (A) | 6.1 |  | 0.511 |
| ns | 8410 |  | 159 | Supporting packages: complete file lists (B) | 6.2 |  | 0.530 |
| ns | 8554 |  | 144 | Context names the expression interpreter resolves | 6.3 |  | 0.525 |
| walker |  | 8603 | 364 | Code::CodeKey { rung: Names, file: pkg/model/planner.go, decl: 0, sub: 0, line: 0 } |  |  | 0.528 |
| walker |  | 8617 | 14 | Code::CodeKey { rung: Decl, file: pkg/model/planner.go, decl: 2, sub: 0, line: 25 } |  |  | 0.528 |
| walker |  | 8631 | 14 | Code::CodeKey { rung: Decl, file: pkg/model/planner.go, decl: 3, sub: 0, line: 30 } |  |  | 0.529 |
| walker |  | 8653 | 22 | Code::CodeKey { rung: Decl, file: pkg/model/planner.go, decl: 4, sub: 0, line: 35 } |  |  | 0.530 |
| walker |  | 8679 | 26 | Code::CodeKey { rung: Decl, file: pkg/model/planner.go, decl: 7, sub: 0, line: 53 } |  |  | 0.526 |
| ns | 8679 |  | 125 | Expression functions act implements | 6.4 |  | 0.526 |
| walker |  | 8693 | 14 | Code::CodeKey { rung: Decl, file: pkg/model/planner.go, decl: 11, sub: 0, line: 196 } |  |  | 0.526 |
| walker |  | 8753 | 60 | Code::CodeKey { rung: Decl, file: pkg/model/planner.go, decl: 1, sub: 0, line: 17 } |  |  | 0.531 |
| walker |  | 8766 | 13 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 1, sub: 0, line: 17 } |  |  | 0.534 |
| walker |  | 8779 | 13 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 6, sub: 0, line: 49 } |  |  | 0.535 |
| walker |  | 8793 | 14 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 2, sub: 0, line: 25 } |  |  | 0.538 |
| walker |  | 8807 | 14 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 3, sub: 0, line: 30 } |  |  | 0.541 |
| walker |  | 8823 | 16 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 4, sub: 0, line: 35 } |  |  | 0.544 |
| walker |  | 8841 | 18 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 16, sub: 0, line: 304 } |  |  | 0.544 |
| walker |  | 8859 | 18 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 17, sub: 0, line: 318 } |  |  | 0.544 |
| walker |  | 8887 | 28 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 8, sub: 0, line: 59 } |  |  | 0.544 |
| walker |  | 8900 | 13 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 18, sub: 0, line: 327 } |  |  | 0.544 |
| ns | 8903 |  | 224 | Artifact cache server: routes and lifecycle | 6.5 |  | 0.539 |
| walker |  | 8916 | 16 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 15, sub: 0, line: 274 } |  |  | 0.539 |
| walker |  | 8933 | 17 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 14, sub: 0, line: 252 } |  |  | 0.539 |
| walker |  | 8953 | 20 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 12, sub: 0, line: 201 } |  |  | 0.539 |
| walker |  | 8973 | 20 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 13, sub: 0, line: 232 } |  |  | 0.539 |
| walker |  | 8995 | 22 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 20, sub: 0, line: 384 } |  |  | 0.539 |
| ns | 9068 |  | 165 | Artifact server: Serve + the v4 route base | 6.6 |  | 0.534 |
| walker |  | 9360 | 365 | Code::CodeKey { rung: Names, file: pkg/common/executor.go, decl: 0, sub: 0, line: 0 } |  |  | 0.550 |
| walker |  | 9372 | 12 | Code::CodeKey { rung: Decl, file: pkg/common/executor.go, decl: 1, sub: 0, line: 12 } |  |  | 0.550 |
| ns | 9373 |  | 305 | pkg/common: complete exported-function roster | 6.7 |  | 0.542 |
| walker |  | 9380 | 8 | Code::CodeKey { rung: Body, file: pkg/common/executor.go, decl: 2, sub: 0, line: 17 } |  |  | 0.542 |
| walker |  | 9389 | 9 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 2, sub: 0, line: 17 } |  |  | 0.542 |
| walker |  | 9400 | 11 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 3, sub: 0, line: 22 } |  |  | 0.542 |
| walker |  | 9411 | 11 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 5, sub: 0, line: 33 } |  |  | 0.544 |
| walker |  | 9422 | 11 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 20, sub: 0, line: 237 } |  |  | 0.544 |
| walker |  | 9435 | 13 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 6, sub: 0, line: 36 } |  |  | 0.544 |
| walker |  | 9449 | 14 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 13, sub: 0, line: 141 } |  |  | 0.544 |
| walker |  | 9463 | 14 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 14, sub: 0, line: 160 } |  |  | 0.544 |
| walker |  | 9477 | 14 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 15, sub: 0, line: 179 } |  |  | 0.544 |
| walker |  | 9492 | 15 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 4, sub: 0, line: 30 } |  |  | 0.547 |
| walker |  | 9507 | 15 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 7, sub: 0, line: 45 } |  |  | 0.547 |
| walker |  | 9522 | 15 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 16, sub: 0, line: 198 } |  |  | 0.547 |
| ns | 9536 |  | 163 | pkg/common/git: repo detection and cloning | 6.8 |  | 0.542 |
| walker |  | 9537 | 15 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 19, sub: 0, line: 225 } |  |  | 0.542 |
| walker |  | 9553 | 16 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 1, sub: 0, line: 12 } |  |  | 0.542 |
| walker |  | 9569 | 16 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 9, sub: 0, line: 72 } |  |  | 0.542 |
| walker |  | 9585 | 16 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 17, sub: 0, line: 208 } |  |  | 0.542 |
| walker |  | 9601 | 16 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 18, sub: 0, line: 218 } |  |  | 0.542 |
| walker |  | 9618 | 17 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 10, sub: 0, line: 88 } |  |  | 0.542 |
| walker |  | 9638 | 20 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 8, sub: 0, line: 54 } |  |  | 0.542 |
| walker |  | 9658 | 20 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 11, sub: 0, line: 95 } |  |  | 0.542 |
| ns | 9666 |  | 130 | CI, issue-template and editor directories | 7.1 |  | 0.545 |
| walker |  | 9677 | 19 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.551 |
| walker |  | 9808 | 131 | Code::CodeKey { rung: Names, file: pkg/model/step_result.go, decl: 0, sub: 0, line: 0 } |  |  | 0.552 |
| walker |  | 9815 | 7 | Code::CodeKey { rung: Decl, file: pkg/model/step_result.go, decl: 2, sub: 0, line: 7 } |  |  | 0.552 |
| walker |  | 9864 | 49 | Code::CodeKey { rung: Decl, file: pkg/model/step_result.go, decl: 7, sub: 0, line: 41 } |  |  | 0.552 |
| walker |  | 9892 | 28 | Code::CodeKey { rung: Decl, file: pkg/model/step_result.go, decl: 3, sub: 0, line: 13 } |  |  | 0.552 |
| walker |  | 9904 | 12 | Code::CodeKey { rung: Body, file: pkg/model/step_result.go, decl: 4, sub: 0, line: 19 } |  |  | 0.552 |
| ns | 9919 |  | 253 | Key direct dependencies | 7.2 |  | 0.559 |
| walker |  | 9979 | 75 | Code::CodeKey { rung: Names, file: pkg/common/dryrun.go, decl: 0, sub: 0, line: 0 } |  |  | 0.559 |
| walker |  | 9994 | 15 | Code::CodeKey { rung: Doc, file: pkg/common/dryrun.go, decl: 3, sub: 0, line: 12 } |  |  | 0.559 |

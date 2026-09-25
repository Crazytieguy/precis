Score(3000)=0.573 I=0.856 C=0.383 ns_rows≤3K=18/47 grid(1000/1442/2080/3000/4327/6240/9000)=0.595/0.654/0.571/0.573/0.509/0.472/0.512

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
| walker |  | 949 | 10 | Plaintext::Whole { file: VERSION } |  |  | 0.594 |
| walker |  | 967 | 18 | Fs::DirListing { dir: pkg/artifacts/testdata } |  |  | 0.595 |
| walker |  | 971 | 4 | Fs::DirListing { dir: pkg/artifacts/testdata/GHSL-2023-004 } |  |  | 0.595 |
| walker |  | 975 | 4 | Fs::DirListing { dir: pkg/artifacts/testdata/upload-and-download } |  |  | 0.595 |
| walker |  | 979 | 4 | Fs::DirListing { dir: pkg/artifacts/testdata/v4 } |  |  | 0.595 |
| walker |  | 1000 | 21 | Fs::DirListing { dir: pkg/model/testdata } |  |  | 0.595 |
| walker |  | 1004 | 4 | Fs::DirListing { dir: pkg/model/testdata/container-volumes } |  |  | 0.595 |
| walker |  | 1008 | 4 | Fs::DirListing { dir: pkg/model/testdata/strategy } |  |  | 0.595 |
| walker |  | 1014 | 6 | Fs::DirListing { dir: pkg/model/testdata/empty-workflow } |  |  | 0.595 |
| walker |  | 1021 | 7 | Fs::DirListing { dir: pkg/model/testdata/nested } |  |  | 0.595 |
| walker |  | 1027 | 6 | Fs::DirListing { dir: pkg/model/testdata/nested/workflows } |  |  | 0.595 |
| walker |  | 1050 | 23 | Fs::DirListing { dir: pkg/exprparser/testdata } |  |  | 0.595 |
| walker |  | 1057 | 7 | Fs::DirListing { dir: pkg/exprparser/testdata/for-hashing-3 } |  |  | 0.595 |
| walker |  | 1062 | 5 | Fs::DirListing { dir: pkg/exprparser/testdata/for-hashing-3/nested } |  |  | 0.595 |
| walker |  | 1080 | 18 | Code::CodeKey { rung: ModuleDoc, file: pkg/container/docker_cli.go, decl: 0, sub: 0, line: 0 } |  |  | 0.595 |
| walker |  | 1100 | 20 | Fs::DirListing { dir: .github } |  |  | 0.595 |
| walker |  | 1126 | 26 | Fs::DirListing { dir: .github/workflows } |  |  | 0.596 |
| ns | 1131 |  | 212 | Root cobra.Command definition | 2.2 |  | 0.568 |
| walker |  | 1163 | 37 | Fs::DirListing { dir: pkg/container/testdata } |  |  | 0.568 |
| walker |  | 1167 | 4 | Fs::DirListing { dir: pkg/container/testdata/docker-pull-options } |  |  | 0.568 |
| walker |  | 1171 | 4 | Fs::DirListing { dir: pkg/container/testdata/scratch } |  |  | 0.568 |
| walker |  | 1356 | 185 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.733 |
| walker |  | 1411 | 55 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.733 |
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
| walker |  | 2345 | 25 | Code::CodeKey { rung: Names, file: main.go, decl: 0, sub: 0, line: 0 } |  |  | 0.550 |
| walker |  | 2355 | 10 | Code::CodeKey { rung: Doc, file: main.go, decl: 1, sub: 0, line: 11 } |  |  | 0.562 |
| walker |  | 2403 | 48 | Code::CodeKey { rung: Body, file: main.go, decl: 2, sub: 0, line: 13 } |  |  | 0.663 |
| walker |  | 2409 | 6 | Plaintext::DeclSurface { file: pkg/exprparser/testdata/for-hashing-1.txt } |  |  | 0.663 |
| walker |  | 2415 | 6 | Plaintext::DeclSurface { file: pkg/exprparser/testdata/for-hashing-2.txt } |  |  | 0.663 |
| ns | 2522 |  | 316 | cmd package: complete function roster | 2.7 |  | 0.623 |
| walker |  | 2549 | 134 | Code::CodeKey { rung: ModuleDoc, file: pkg/artifactcache/doc.go, decl: 0, sub: 0, line: 0 } |  |  | 0.623 |
| walker |  | 2553 | 4 | Fs::DirListing { dir: .github/actions } |  |  | 0.623 |
| walker |  | 2560 | 7 | Plaintext::DeclSurface { file: pkg/container/testdata/scratch/test.txt } |  |  | 0.623 |
| ns | 2869 |  | 347 | pkg/model: complete type roster | 3.1 |  | 0.584 |
| ns | 2979 |  | 110 | Workflow struct | 3.2 | 3.1 | 0.573 |
| ns | 3209 |  | 230 | Planner interface + Plan/Stage/Run | 3.3 | 3.1 | 0.545 |
| walker |  | 3369 | 809 | GoMod::File { file: go.mod } |  |  | 0.546 |
| walker |  | 3436 | 67 | Plaintext::Whole { file: pkg/container/testdata/Dockerfile } |  |  | 0.546 |
| walker |  | 3444 | 8 | Plaintext::DeclSurface { file: pkg/exprparser/testdata/for-hashing-3/data.txt } |  |  | 0.546 |
| ns | 3471 |  | 262 | Job struct: every supported job key | 3.4 | 3.1 | 0.529 |
| walker |  | 3591 | 147 | Code::CodeKey { rung: Names, file: cmd/notices.go, decl: 0, sub: 0, line: 0 } |  |  | 0.533 |
| walker |  | 3618 | 27 | Code::CodeKey { rung: Decl, file: cmd/notices.go, decl: 1, sub: 0, line: 17 } |  |  | 0.533 |
| walker |  | 3647 | 29 | Code::CodeKey { rung: Body, file: cmd/notices.go, decl: 4, sub: 0, line: 57 } |  |  | 0.533 |
| ns | 3695 |  | 224 | Step struct: every supported step key | 3.5 | 3.1 | 0.519 |
| walker |  | 3714 | 67 | Code::CodeKey { rung: Body, file: cmd/notices.go, decl: 9, sub: 0, line: 138 } |  |  | 0.519 |
| walker |  | 3791 | 77 | Code::CodeKey { rung: Body, file: cmd/notices.go, decl: 7, sub: 0, line: 121 } |  |  | 0.519 |
| walker |  | 3941 | 150 | Code::CodeKey { rung: Names, file: cmd/input.go, decl: 0, sub: 0, line: 0 } |  |  | 0.537 |
| walker |  | 3950 | 9 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 6, sub: 0, line: 99 } |  |  | 0.537 |
| walker |  | 3961 | 11 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 3, sub: 0, line: 85 } |  |  | 0.537 |
| walker |  | 3972 | 11 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 4, sub: 0, line: 90 } |  |  | 0.537 |
| walker |  | 3983 | 11 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 5, sub: 0, line: 94 } |  |  | 0.537 |
| walker |  | 3995 | 12 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 4, sub: 0, line: 90 } |  |  | 0.537 |
| walker |  | 4008 | 13 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 3, sub: 0, line: 85 } |  |  | 0.537 |
| walker |  | 4021 | 13 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 6, sub: 0, line: 99 } |  |  | 0.537 |
| ns | 4027 |  | 332 | StepType constants | 3.6 | 3.1 | 0.517 |
| walker |  | 4035 | 14 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 8, sub: 0, line: 109 } |  |  | 0.517 |
| walker |  | 4050 | 15 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 7, sub: 0, line: 104 } |  |  | 0.517 |
| walker |  | 4065 | 15 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 9, sub: 0, line: 114 } |  |  | 0.517 |
| walker |  | 4099 | 34 | Code::CodeKey { rung: Names, file: cmd/dir.go, decl: 0, sub: 0, line: 0 } |  |  | 0.518 |
| walker |  | 4108 | 9 | Code::CodeKey { rung: Decl, file: cmd/dir.go, decl: 1, sub: 0, line: 10 } |  |  | 0.518 |
| walker |  | 4159 | 51 | Code::CodeKey { rung: Names, file: cmd/secrets.go, decl: 0, sub: 0, line: 0 } |  |  | 0.522 |
| walker |  | 4166 | 7 | Code::CodeKey { rung: Body, file: cmd/secrets.go, decl: 3, sub: 0, line: 40 } |  |  | 0.522 |
| walker |  | 4184 | 18 | Code::CodeKey { rung: Names, file: cmd/graph.go, decl: 0, sub: 0, line: 0 } |  |  | 0.524 |
| walker |  | 4202 | 18 | Code::CodeKey { rung: Names, file: cmd/list.go, decl: 0, sub: 0, line: 0 } |  |  | 0.525 |
| walker |  | 4213 | 11 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 8, sub: 0, line: 109 } |  |  | 0.525 |
| ns | 4248 |  | 221 | GithubContext: complete field-name roster | 3.7 | 3.1 | 0.503 |
| ns | 4531 |  | 283 | Executor type + complete combinator roster | 4.1 |  | 0.488 |
| walker |  | 4587 | 374 | Code::CodeKey { rung: Names, file: cmd/root.go, decl: 0, sub: 0, line: 0 } |  |  | 0.527 |
| walker |  | 4637 | 50 | Code::CodeKey { rung: Decl, file: cmd/root.go, decl: 1, sub: 0, line: 37 } |  |  | 0.527 |
| walker |  | 4649 | 12 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 18, sub: 0, line: 391 } |  |  | 0.527 |
| walker |  | 4662 | 13 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 3, sub: 0, line: 47 } |  |  | 0.529 |
| walker |  | 4678 | 16 | Code::CodeKey { rung: Body, file: cmd/root.go, decl: 15, sub: 0, line: 345 } |  |  | 0.529 |
| walker |  | 4713 | 35 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 5, sub: 0, line: 138 } |  |  | 0.532 |
| walker |  | 4745 | 32 | Code::CodeKey { rung: Body, file: cmd/root.go, decl: 12, sub: 0, line: 314 } |  |  | 0.532 |
| walker |  | 4858 | 113 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.532 |
| ns | 5066 |  | 535 | pkg/runner: complete type roster | 4.2 |  | 0.508 |
| ns | 5176 |  | 110 | Runner interface and New() | 4.3 | 4.2 | 0.501 |
| walker |  | 5439 | 581 | Code::CodeKey { rung: Decl, file: cmd/input.go, decl: 1, sub: 0, line: 10 } |  |  | 0.502 |
| walker |  | 5453 | 14 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 1, sub: 0, line: 10 } |  |  | 0.508 |
| walker |  | 5460 | 7 | Plaintext::DeclSurface { file: pkg/exprparser/testdata/for-hashing-3/nested/nested-data.txt } |  |  | 0.508 |
| walker |  | 5481 | 21 | Code::CodeKey { rung: Names, file: cmd/platforms.go, decl: 0, sub: 0, line: 0 } |  |  | 0.514 |
| walker |  | 5558 | 77 | Code::CodeKey { rung: Body, file: cmd/notices.go, decl: 8, sub: 0, line: 130 } |  |  | 0.514 |
| ns | 5564 |  | 388 | runner.Config: fields (first half, rest elided) | 4.4 | 4.2 | 0.500 |
| walker |  | 5675 | 117 | Code::CodeKey { rung: Body, file: cmd/dir.go, decl: 2, sub: 0, line: 15 } |  |  | 0.500 |
| walker |  | 5688 | 13 | Code::CodeKey { rung: Names, file: pkg/model/job_context.go, decl: 0, sub: 0, line: 0 } |  |  | 0.500 |
| walker |  | 5782 | 94 | Code::CodeKey { rung: Decl, file: pkg/model/job_context.go, decl: 1, sub: 0, line: 3 } |  |  | 0.500 |
| walker |  | 5822 | 40 | Code::CodeKey { rung: Names, file: pkg/artifactcache/model.go, decl: 0, sub: 0, line: 0 } |  |  | 0.500 |
| walker |  | 5865 | 43 | Code::CodeKey { rung: Decl, file: pkg/artifactcache/model.go, decl: 1, sub: 0, line: 3 } |  |  | 0.500 |
| ns | 5866 |  | 302 | RunContext struct | 4.5 | 4.2 | 0.486 |
| walker |  | 6012 | 147 | Code::CodeKey { rung: Decl, file: pkg/artifactcache/model.go, decl: 3, sub: 0, line: 26 } |  |  | 0.486 |
| walker |  | 6026 | 14 | Code::CodeKey { rung: Names, file: pkg/container/executions_environment.go, decl: 0, sub: 0, line: 0 } |  |  | 0.486 |
| walker |  | 6134 | 108 | Code::CodeKey { rung: Decl, file: pkg/container/executions_environment.go, decl: 1, sub: 0, line: 5 } |  |  | 0.487 |
| walker |  | 6162 | 28 | Code::CodeKey { rung: Names, file: pkg/lookpath/error.go, decl: 0, sub: 0, line: 0 } |  |  | 0.487 |
| walker |  | 6180 | 18 | Code::CodeKey { rung: Decl, file: pkg/lookpath/error.go, decl: 1, sub: 0, line: 3 } |  |  | 0.487 |
| walker |  | 6189 | 9 | Code::CodeKey { rung: Body, file: pkg/lookpath/error.go, decl: 2, sub: 0, line: 8 } |  |  | 0.487 |
| ns | 6221 |  | 355 | stepFactory: StepType -> implementation | 4.6 | 4.2 | 0.472 |
| walker |  | 6248 | 59 | Code::CodeKey { rung: Names, file: pkg/lookpath/env.go, decl: 0, sub: 0, line: 0 } |  |  | 0.472 |
| walker |  | 6251 | 3 | Code::CodeKey { rung: Decl, file: pkg/lookpath/env.go, decl: 2, sub: 0, line: 9 } |  |  | 0.472 |
| walker |  | 6265 | 14 | Code::CodeKey { rung: Decl, file: pkg/lookpath/env.go, decl: 1, sub: 0, line: 5 } |  |  | 0.472 |
| walker |  | 6274 | 9 | Code::CodeKey { rung: Body, file: pkg/lookpath/env.go, decl: 3, sub: 0, line: 12 } |  |  | 0.472 |
| walker |  | 6288 | 14 | Code::CodeKey { rung: Body, file: pkg/lookpath/env.go, decl: 4, sub: 0, line: 16 } |  |  | 0.472 |
| walker |  | 6392 | 104 | Code::CodeKey { rung: Names, file: pkg/container/container_types.go, decl: 0, sub: 0, line: 0 } |  |  | 0.472 |
| walker |  | 6399 | 7 | Code::CodeKey { rung: Decl, file: pkg/container/container_types.go, decl: 7, sub: 0, line: 80 } |  |  | 0.472 |
| walker |  | 6428 | 29 | Code::CodeKey { rung: Decl, file: pkg/container/container_types.go, decl: 2, sub: 0, line: 36 } |  |  | 0.472 |
| walker |  | 6475 | 47 | Code::CodeKey { rung: Decl, file: pkg/container/container_types.go, decl: 5, sub: 0, line: 70 } |  |  | 0.472 |
| ns | 6499 |  | 278 | step interface and stage enum | 4.7 | 4.2 | 0.458 |
| walker |  | 6527 | 52 | Code::CodeKey { rung: Decl, file: pkg/container/container_types.go, decl: 4, sub: 0, line: 61 } |  |  | 0.458 |
| walker |  | 6543 | 16 | Code::CodeKey { rung: Doc, file: pkg/container/container_types.go, decl: 2, sub: 0, line: 36 } |  |  | 0.458 |
| walker |  | 6561 | 18 | Code::CodeKey { rung: Doc, file: pkg/container/container_types.go, decl: 5, sub: 0, line: 70 } |  |  | 0.458 |
| walker |  | 6581 | 20 | Code::CodeKey { rung: Doc, file: pkg/container/container_types.go, decl: 4, sub: 0, line: 61 } |  |  | 0.458 |
| walker |  | 6787 | 206 | Code::CodeKey { rung: Decl, file: pkg/container/container_types.go, decl: 1, sub: 0, line: 12 } |  |  | 0.458 |
| walker |  | 6802 | 15 | Code::CodeKey { rung: Doc, file: pkg/container/container_types.go, decl: 1, sub: 0, line: 12 } |  |  | 0.458 |
| ns | 6994 |  | 495 | run_context.go: method roster (container lifecycle) | 4.8 |  | 0.445 |
| walker |  | 7048 | 246 | Code::CodeKey { rung: Decl, file: pkg/container/container_types.go, decl: 3, sub: 0, line: 43 } |  |  | 0.446 |
| walker |  | 7058 | 10 | Code::CodeKey { rung: Doc, file: pkg/container/container_types.go, decl: 3, sub: 0, line: 43 } |  |  | 0.446 |
| walker |  | 7074 | 16 | Code::CodeKey { rung: Names, file: pkg/common/outbound_ip.go, decl: 0, sub: 0, line: 0 } |  |  | 0.446 |
| ns | 7140 |  | 146 | ActionCache implementations | 4.9 | 4.2 | 0.438 |
| walker |  | 7150 | 76 | Code::CodeKey { rung: Doc, file: pkg/common/outbound_ip.go, decl: 1, sub: 0, line: 13 } |  |  | 0.438 |
| walker |  | 7161 | 11 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 9, sub: 0, line: 114 } |  |  | 0.438 |
| ns | 7300 |  | 160 | pkg/container: complete file list | 5.1 |  | 0.465 |
| walker |  | 7452 | 291 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.465 |
| walker |  | 7537 | 85 | Code::CodeKey { rung: Names, file: pkg/container/docker_socket.go, decl: 0, sub: 0, line: 0 } |  |  | 0.465 |
| walker |  | 7557 | 20 | Code::CodeKey { rung: Decl, file: pkg/container/docker_socket.go, decl: 4, sub: 0, line: 57 } |  |  | 0.465 |
| walker |  | 7572 | 15 | Code::CodeKey { rung: Doc, file: pkg/container/docker_socket.go, decl: 2, sub: 0, line: 23 } |  |  | 0.465 |
| ns | 7577 |  | 277 | Container interface | 5.2 |  | 0.479 |
| walker |  | 7671 | 99 | Code::CodeKey { rung: Decl, file: pkg/container/docker_socket.go, decl: 1, sub: 0, line: 12 } |  |  | 0.479 |
| ns | 7719 |  | 142 | ExecutionsEnvironment interface | 5.3 |  | 0.484 |
| walker |  | 7756 | 85 | Code::CodeKey { rung: Names, file: pkg/workflowpattern/trace_writer.go, decl: 0, sub: 0, line: 0 } |  |  | 0.484 |
| walker |  | 7759 | 3 | Code::CodeKey { rung: Decl, file: pkg/workflowpattern/trace_writer.go, decl: 3, sub: 0, line: 11 } |  |  | 0.484 |
| walker |  | 7773 | 14 | Code::CodeKey { rung: Decl, file: pkg/workflowpattern/trace_writer.go, decl: 1, sub: 0, line: 5 } |  |  | 0.484 |
| walker |  | 7785 | 12 | Code::CodeKey { rung: Body, file: pkg/workflowpattern/trace_writer.go, decl: 5, sub: 0, line: 16 } |  |  | 0.484 |
| ns | 7984 |  | 265 | Docker backend vs docker-less stub: the build-tag split | 5.4 |  | 0.476 |
| walker |  | 8023 | 238 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.478 |
| ns | 8161 |  | 177 | Docker socket discovery | 5.5 |  | 0.487 |
| ns | 8251 |  | 90 | Supporting packages: complete file lists (A) | 6.1 |  | 0.501 |
| walker |  | 8330 | 307 | Code::CodeKey { rung: Names, file: pkg/artifacts/server.go, decl: 0, sub: 0, line: 0 } |  |  | 0.501 |
| walker |  | 8333 | 3 | Code::CodeKey { rung: Decl, file: pkg/artifacts/server.go, decl: 9, sub: 0, line: 58 } |  |  | 0.501 |
| walker |  | 8344 | 11 | Code::CodeKey { rung: Decl, file: pkg/artifacts/server.go, decl: 7, sub: 0, line: 49 } |  |  | 0.501 |
| walker |  | 8358 | 14 | Code::CodeKey { rung: Decl, file: pkg/artifacts/server.go, decl: 6, sub: 0, line: 45 } |  |  | 0.501 |
| walker |  | 8374 | 16 | Code::CodeKey { rung: Decl, file: pkg/artifacts/server.go, decl: 5, sub: 0, line: 41 } |  |  | 0.501 |
| walker |  | 8394 | 20 | Code::CodeKey { rung: Decl, file: pkg/artifacts/server.go, decl: 1, sub: 0, line: 21 } |  |  | 0.501 |
| ns | 8410 |  | 159 | Supporting packages: complete file lists (B) | 6.2 |  | 0.521 |
| walker |  | 8426 | 32 | Code::CodeKey { rung: Decl, file: pkg/artifacts/server.go, decl: 2, sub: 0, line: 25 } |  |  | 0.521 |
| walker |  | 8458 | 32 | Code::CodeKey { rung: Decl, file: pkg/artifacts/server.go, decl: 3, sub: 0, line: 30 } |  |  | 0.521 |
| walker |  | 8494 | 36 | Code::CodeKey { rung: Decl, file: pkg/artifacts/server.go, decl: 8, sub: 0, line: 53 } |  |  | 0.521 |
| walker |  | 8536 | 42 | Code::CodeKey { rung: Decl, file: pkg/artifacts/server.go, decl: 4, sub: 0, line: 35 } |  |  | 0.521 |
| walker |  | 8545 | 9 | Code::CodeKey { rung: Body, file: pkg/artifacts/server.go, decl: 10, sub: 0, line: 61 } |  |  | 0.521 |
| ns | 8554 |  | 144 | Context names the expression interpreter resolves | 6.3 |  | 0.516 |
| ns | 8679 |  | 125 | Expression functions act implements | 6.4 |  | 0.511 |
| ns | 8903 |  | 224 | Artifact cache server: routes and lifecycle | 6.5 |  | 0.506 |
| walker |  | 8909 | 364 | Code::CodeKey { rung: Names, file: pkg/model/planner.go, decl: 0, sub: 0, line: 0 } |  |  | 0.509 |
| walker |  | 8923 | 14 | Code::CodeKey { rung: Decl, file: pkg/model/planner.go, decl: 2, sub: 0, line: 25 } |  |  | 0.510 |
| walker |  | 8937 | 14 | Code::CodeKey { rung: Decl, file: pkg/model/planner.go, decl: 3, sub: 0, line: 30 } |  |  | 0.510 |
| walker |  | 8951 | 14 | Code::CodeKey { rung: Decl, file: pkg/model/planner.go, decl: 11, sub: 0, line: 196 } |  |  | 0.510 |
| walker |  | 8973 | 22 | Code::CodeKey { rung: Decl, file: pkg/model/planner.go, decl: 4, sub: 0, line: 35 } |  |  | 0.512 |
| walker |  | 8999 | 26 | Code::CodeKey { rung: Decl, file: pkg/model/planner.go, decl: 7, sub: 0, line: 53 } |  |  | 0.512 |
| walker |  | 9059 | 60 | Code::CodeKey { rung: Decl, file: pkg/model/planner.go, decl: 1, sub: 0, line: 17 } |  |  | 0.517 |
| ns | 9068 |  | 165 | Artifact server: Serve + the v4 route base | 6.6 |  | 0.513 |
| walker |  | 9072 | 13 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 1, sub: 0, line: 17 } |  |  | 0.515 |
| walker |  | 9085 | 13 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 6, sub: 0, line: 49 } |  |  | 0.517 |
| walker |  | 9098 | 13 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 18, sub: 0, line: 327 } |  |  | 0.517 |
| walker |  | 9112 | 14 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 2, sub: 0, line: 25 } |  |  | 0.520 |
| walker |  | 9126 | 14 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 3, sub: 0, line: 30 } |  |  | 0.522 |
| walker |  | 9142 | 16 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 4, sub: 0, line: 35 } |  |  | 0.525 |
| walker |  | 9158 | 16 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 15, sub: 0, line: 274 } |  |  | 0.525 |
| walker |  | 9175 | 17 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 14, sub: 0, line: 252 } |  |  | 0.525 |
| walker |  | 9193 | 18 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 16, sub: 0, line: 304 } |  |  | 0.525 |
| walker |  | 9211 | 18 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 17, sub: 0, line: 318 } |  |  | 0.525 |
| walker |  | 9231 | 20 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 12, sub: 0, line: 201 } |  |  | 0.525 |
| walker |  | 9251 | 20 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 13, sub: 0, line: 232 } |  |  | 0.525 |
| walker |  | 9273 | 22 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 20, sub: 0, line: 384 } |  |  | 0.525 |
| walker |  | 9301 | 28 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 8, sub: 0, line: 59 } |  |  | 0.525 |
| ns | 9373 |  | 305 | pkg/common: complete exported-function roster | 6.7 |  | 0.517 |
| ns | 9536 |  | 163 | pkg/common/git: repo detection and cloning | 6.8 |  | 0.512 |
| walker |  | 9666 | 365 | Code::CodeKey { rung: Names, file: pkg/common/executor.go, decl: 0, sub: 0, line: 0 } |  |  | 0.531 |
| ns | 9666 |  | 130 | CI, issue-template and editor directories | 7.1 |  | 0.531 |
| walker |  | 9678 | 12 | Code::CodeKey { rung: Decl, file: pkg/common/executor.go, decl: 1, sub: 0, line: 12 } |  |  | 0.531 |
| walker |  | 9686 | 8 | Code::CodeKey { rung: Body, file: pkg/common/executor.go, decl: 2, sub: 0, line: 17 } |  |  | 0.531 |
| walker |  | 9695 | 9 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 2, sub: 0, line: 17 } |  |  | 0.531 |
| walker |  | 9706 | 11 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 3, sub: 0, line: 22 } |  |  | 0.531 |
| walker |  | 9717 | 11 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 5, sub: 0, line: 33 } |  |  | 0.534 |
| walker |  | 9728 | 11 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 20, sub: 0, line: 237 } |  |  | 0.534 |
| walker |  | 9741 | 13 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 6, sub: 0, line: 36 } |  |  | 0.534 |
| walker |  | 9755 | 14 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 13, sub: 0, line: 141 } |  |  | 0.534 |
| walker |  | 9769 | 14 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 14, sub: 0, line: 160 } |  |  | 0.534 |
| walker |  | 9783 | 14 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 15, sub: 0, line: 179 } |  |  | 0.534 |
| walker |  | 9798 | 15 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 4, sub: 0, line: 30 } |  |  | 0.537 |
| walker |  | 9813 | 15 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 7, sub: 0, line: 45 } |  |  | 0.537 |
| walker |  | 9828 | 15 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 16, sub: 0, line: 198 } |  |  | 0.537 |
| walker |  | 9843 | 15 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 19, sub: 0, line: 225 } |  |  | 0.537 |
| walker |  | 9859 | 16 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 1, sub: 0, line: 12 } |  |  | 0.537 |
| walker |  | 9875 | 16 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 9, sub: 0, line: 72 } |  |  | 0.537 |
| walker |  | 9891 | 16 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 17, sub: 0, line: 208 } |  |  | 0.537 |
| walker |  | 9907 | 16 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 18, sub: 0, line: 218 } |  |  | 0.537 |
| ns | 9919 |  | 253 | Key direct dependencies | 7.2 |  | 0.544 |
| walker |  | 9924 | 17 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 10, sub: 0, line: 88 } |  |  | 0.544 |
| walker |  | 9944 | 20 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 8, sub: 0, line: 54 } |  |  | 0.544 |
| walker |  | 9964 | 20 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 11, sub: 0, line: 95 } |  |  | 0.544 |
| walker |  | 9993 | 29 | Code::CodeKey { rung: Names, file: pkg/model/step_result.go, decl: 0, sub: 0, line: 0 } |  |  | 0.545 |

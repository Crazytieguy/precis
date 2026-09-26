Score(3000)=0.579 I=0.869 C=0.386 ns_rows≤3K=18/47 grid(1000/1442/2080/3000/4327/6240/9000)=0.757/0.649/0.566/0.579/0.516/0.480/0.561

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
| ns | 2808 |  | 347 | pkg/model: complete type roster | 3.1 |  | 0.587 |
| walker |  | 2842 | 177 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.590 |
| walker |  | 2849 | 7 | Plaintext::DeclSurface { file: pkg/container/testdata/scratch/test.txt } |  |  | 0.590 |
| walker |  | 2916 | 67 | Plaintext::Whole { file: pkg/container/testdata/Dockerfile } |  |  | 0.590 |
| ns | 2918 |  | 110 | Workflow struct | 3.2 | 3.1 | 0.579 |
| walker |  | 2924 | 8 | Plaintext::DeclSurface { file: pkg/exprparser/testdata/for-hashing-3/data.txt } |  |  | 0.579 |
| ns | 3148 |  | 230 | Planner interface + Plan/Stage/Run | 3.3 | 3.1 | 0.551 |
| walker |  | 3192 | 268 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.551 |
| ns | 3410 |  | 262 | Job struct: every supported job key | 3.4 | 3.1 | 0.534 |
| walker |  | 3566 | 374 | Code::CodeKey { rung: Names, file: cmd/root.go, decl: 0, sub: 0, line: 0 } |  |  | 0.552 |
| walker |  | 3616 | 50 | Code::CodeKey { rung: Decl, file: cmd/root.go, decl: 1, sub: 0, line: 37 } |  |  | 0.552 |
| walker |  | 3628 | 12 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 18, sub: 0, line: 391 } |  |  | 0.552 |
| ns | 3634 |  | 224 | Step struct: every supported step key | 3.5 | 3.1 | 0.538 |
| walker |  | 3641 | 13 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 3, sub: 0, line: 47 } |  |  | 0.540 |
| walker |  | 3791 | 150 | Code::CodeKey { rung: Names, file: cmd/input.go, decl: 0, sub: 0, line: 0 } |  |  | 0.557 |
| ns | 3966 |  | 332 | StepType constants | 3.6 | 3.1 | 0.537 |
| ns | 4187 |  | 221 | GithubContext: complete field-name roster | 3.7 | 3.1 | 0.514 |
| walker |  | 4372 | 581 | Code::CodeKey { rung: Decl, file: cmd/input.go, decl: 1, sub: 0, line: 10 } |  |  | 0.516 |
| walker |  | 4384 | 12 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 4, sub: 0, line: 90 } |  |  | 0.516 |
| walker |  | 4397 | 13 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 3, sub: 0, line: 85 } |  |  | 0.516 |
| walker |  | 4410 | 13 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 6, sub: 0, line: 99 } |  |  | 0.516 |
| walker |  | 4424 | 14 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 1, sub: 0, line: 10 } |  |  | 0.522 |
| walker |  | 4438 | 14 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 8, sub: 0, line: 109 } |  |  | 0.522 |
| walker |  | 4453 | 15 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 7, sub: 0, line: 104 } |  |  | 0.522 |
| walker |  | 4468 | 15 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 9, sub: 0, line: 114 } |  |  | 0.522 |
| ns | 4470 |  | 283 | Executor type + complete combinator roster | 4.1 |  | 0.506 |
| walker |  | 4615 | 147 | Code::CodeKey { rung: Names, file: cmd/notices.go, decl: 0, sub: 0, line: 0 } |  |  | 0.523 |
| walker |  | 4642 | 27 | Code::CodeKey { rung: Decl, file: cmd/notices.go, decl: 1, sub: 0, line: 17 } |  |  | 0.523 |
| walker |  | 4693 | 51 | Code::CodeKey { rung: Names, file: cmd/secrets.go, decl: 0, sub: 0, line: 0 } |  |  | 0.531 |
| walker |  | 4700 | 7 | Code::CodeKey { rung: Body, file: cmd/secrets.go, decl: 3, sub: 0, line: 40 } |  |  | 0.531 |
| walker |  | 4707 | 7 | Plaintext::DeclSurface { file: pkg/exprparser/testdata/for-hashing-3/nested/nested-data.txt } |  |  | 0.531 |
| walker |  | 4728 | 21 | Code::CodeKey { rung: Names, file: cmd/platforms.go, decl: 0, sub: 0, line: 0 } |  |  | 0.537 |
| walker |  | 4762 | 34 | Code::CodeKey { rung: Names, file: cmd/dir.go, decl: 0, sub: 0, line: 0 } |  |  | 0.541 |
| walker |  | 4771 | 9 | Code::CodeKey { rung: Decl, file: cmd/dir.go, decl: 1, sub: 0, line: 10 } |  |  | 0.541 |
| walker |  | 4789 | 18 | Code::CodeKey { rung: Names, file: cmd/list.go, decl: 0, sub: 0, line: 0 } |  |  | 0.544 |
| walker |  | 4807 | 18 | Code::CodeKey { rung: Names, file: cmd/graph.go, decl: 0, sub: 0, line: 0 } |  |  | 0.548 |
| ns | 5005 |  | 535 | pkg/runner: complete type roster | 4.2 |  | 0.523 |
| ns | 5115 |  | 110 | Runner interface and New() | 4.3 | 4.2 | 0.516 |
| ns | 5503 |  | 388 | runner.Config: fields (first half, rest elided) | 4.4 | 4.2 | 0.502 |
| walker |  | 5616 | 809 | GoMod::File { file: go.mod } |  |  | 0.503 |
| walker |  | 5651 | 35 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 5, sub: 0, line: 138 } |  |  | 0.505 |
| walker |  | 5710 | 59 | Code::CodeKey { rung: Names, file: pkg/lookpath/env.go, decl: 0, sub: 0, line: 0 } |  |  | 0.505 |
| walker |  | 5713 | 3 | Code::CodeKey { rung: Decl, file: pkg/lookpath/env.go, decl: 2, sub: 0, line: 9 } |  |  | 0.505 |
| walker |  | 5727 | 14 | Code::CodeKey { rung: Decl, file: pkg/lookpath/env.go, decl: 1, sub: 0, line: 5 } |  |  | 0.505 |
| walker |  | 5736 | 9 | Code::CodeKey { rung: Body, file: pkg/lookpath/env.go, decl: 3, sub: 0, line: 12 } |  |  | 0.505 |
| walker |  | 5750 | 14 | Code::CodeKey { rung: Body, file: pkg/lookpath/env.go, decl: 4, sub: 0, line: 16 } |  |  | 0.505 |
| walker |  | 5778 | 28 | Code::CodeKey { rung: Names, file: pkg/lookpath/error.go, decl: 0, sub: 0, line: 0 } |  |  | 0.505 |
| walker |  | 5796 | 18 | Code::CodeKey { rung: Decl, file: pkg/lookpath/error.go, decl: 1, sub: 0, line: 3 } |  |  | 0.505 |
| walker |  | 5805 | 9 | Code::CodeKey { rung: Body, file: pkg/lookpath/error.go, decl: 2, sub: 0, line: 8 } |  |  | 0.491 |
| ns | 5805 |  | 302 | RunContext struct | 4.5 | 4.2 | 0.491 |
| walker |  | 6025 | 220 | Code::CodeKey { rung: Names, file: pkg/model/workflow.go, decl: 0, sub: 0, line: 0 } |  |  | 0.495 |
| walker |  | 6038 | 13 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 13, sub: 0, line: 171 } |  |  | 0.495 |
| walker |  | 6058 | 20 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 8, sub: 0, line: 113 } |  |  | 0.495 |
| walker |  | 6084 | 26 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 11, sub: 0, line: 161 } |  |  | 0.495 |
| walker |  | 6123 | 39 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 12, sub: 0, line: 166 } |  |  | 0.495 |
| ns | 6160 |  | 355 | stepFactory: StepType -> implementation | 4.6 | 4.2 | 0.480 |
| walker |  | 6178 | 55 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 10, sub: 0, line: 154 } |  |  | 0.480 |
| walker |  | 6247 | 69 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 7, sub: 0, line: 105 } |  |  | 0.480 |
| walker |  | 6334 | 87 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 1, sub: 0, line: 19 } |  |  | 0.491 |
| walker |  | 6343 | 9 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 2, sub: 0, line: 29 } |  |  | 0.491 |
| walker |  | 6361 | 18 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 1, sub: 0, line: 19 } |  |  | 0.495 |
| ns | 6438 |  | 278 | step interface and stage enum | 4.7 | 4.2 | 0.480 |
| walker |  | 6560 | 199 | Code::CodeKey { rung: Names, file: pkg/model/workflow.go, decl: 0, sub: 1, line: 0 } |  |  | 0.484 |
| walker |  | 6577 | 17 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 17, sub: 0, line: 225 } |  |  | 0.484 |
| walker |  | 6609 | 32 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 18, sub: 0, line: 230 } |  |  | 0.484 |
| walker |  | 6681 | 72 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 16, sub: 0, line: 216 } |  |  | 0.484 |
| walker |  | 6927 | 246 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 15, sub: 0, line: 196 } |  |  | 0.509 |
| ns | 6933 |  | 495 | run_context.go: method roster (container lifecycle) | 4.8 |  | 0.495 |
| walker |  | 6935 | 8 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 16, sub: 0, line: 216 } |  |  | 0.495 |
| walker |  | 6945 | 10 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 24, sub: 0, line: 311 } |  |  | 0.495 |
| walker |  | 6956 | 11 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 23, sub: 0, line: 293 } |  |  | 0.495 |
| walker |  | 6967 | 11 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 25, sub: 0, line: 330 } |  |  | 0.495 |
| walker |  | 6981 | 14 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 18, sub: 0, line: 230 } |  |  | 0.495 |
| walker |  | 6997 | 16 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 15, sub: 0, line: 196 } |  |  | 0.498 |
| walker |  | 7014 | 17 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 17, sub: 0, line: 225 } |  |  | 0.498 |
| walker |  | 7031 | 17 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 19, sub: 0, line: 236 } |  |  | 0.498 |
| walker |  | 7049 | 18 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 20, sub: 0, line: 253 } |  |  | 0.498 |
| ns | 7079 |  | 146 | ActionCache implementations | 4.9 | 4.2 | 0.489 |
| ns | 7239 |  | 160 | pkg/container: complete file list | 5.1 |  | 0.512 |
| walker |  | 7242 | 193 | Code::CodeKey { rung: Names, file: pkg/model/workflow.go, decl: 0, sub: 2, line: 0 } |  |  | 0.513 |
| walker |  | 7359 | 117 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 34, sub: 0, line: 503 } |  |  | 0.513 |
| walker |  | 7372 | 13 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 29, sub: 0, line: 388 } |  |  | 0.513 |
| walker |  | 7388 | 16 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 28, sub: 0, line: 383 } |  |  | 0.513 |
| walker |  | 7406 | 18 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 33, sub: 0, line: 501 } |  |  | 0.513 |
| ns | 7516 |  | 277 | Container interface | 5.2 |  | 0.505 |
| walker |  | 7637 | 231 | Code::CodeKey { rung: Names, file: pkg/model/workflow.go, decl: 0, sub: 3, line: 0 } |  |  | 0.511 |
| ns | 7658 |  | 142 | ExecutionsEnvironment interface | 5.3 |  | 0.505 |
| walker |  | 7766 | 129 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 37, sub: 0, line: 555 } |  |  | 0.505 |
| ns | 7923 |  | 265 | Docker backend vs docker-less stub: the build-tag split | 5.4 |  | 0.497 |
| walker |  | 7974 | 208 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 38, sub: 0, line: 569 } |  |  | 0.514 |
| ns | 8100 |  | 177 | Docker socket discovery | 5.5 |  | 0.508 |
| walker |  | 8189 | 215 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 44, sub: 0, line: 647 } |  |  | 0.531 |
| ns | 8190 |  | 90 | Supporting packages: complete file lists (A) | 6.1 |  | 0.543 |
| walker |  | 8199 | 10 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 39, sub: 0, line: 586 } |  |  | 0.543 |
| walker |  | 8212 | 13 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 36, sub: 0, line: 530 } |  |  | 0.543 |
| walker |  | 8226 | 14 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 38, sub: 0, line: 569 } |  |  | 0.545 |
| walker |  | 8240 | 14 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 41, sub: 0, line: 603 } |  |  | 0.545 |
| walker |  | 8254 | 14 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 42, sub: 0, line: 615 } |  |  | 0.545 |
| walker |  | 8270 | 16 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 40, sub: 0, line: 598 } |  |  | 0.545 |
| ns | 8349 |  | 159 | Supporting packages: complete file lists (B) | 6.2 |  | 0.562 |
| walker |  | 8391 | 121 | Code::CodeKey { rung: Names, file: pkg/model/workflow.go, decl: 0, sub: 4, line: 0 } |  |  | 0.562 |
| walker |  | 8420 | 29 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 50, sub: 0, line: 750 } |  |  | 0.562 |
| walker |  | 8433 | 13 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 46, sub: 0, line: 691 } |  |  | 0.562 |
| walker |  | 8450 | 17 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 48, sub: 0, line: 726 } |  |  | 0.562 |
| walker |  | 8468 | 18 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 43, sub: 0, line: 645 } |  |  | 0.565 |
| walker |  | 8486 | 18 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 49, sub: 0, line: 742 } |  |  | 0.565 |
| ns | 8493 |  | 144 | Context names the expression interpreter resolves | 6.3 |  | 0.559 |
| walker |  | 8505 | 19 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 37, sub: 0, line: 555 } |  |  | 0.559 |
| walker |  | 8524 | 19 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 47, sub: 0, line: 714 } |  |  | 0.559 |
| ns | 8618 |  | 125 | Expression functions act implements | 6.4 |  | 0.555 |
| ns | 8842 |  | 224 | Artifact cache server: routes and lifecycle | 6.5 |  | 0.550 |
| walker |  | 8888 | 364 | Code::CodeKey { rung: Names, file: pkg/model/planner.go, decl: 0, sub: 0, line: 0 } |  |  | 0.558 |
| walker |  | 8902 | 14 | Code::CodeKey { rung: Decl, file: pkg/model/planner.go, decl: 2, sub: 0, line: 25 } |  |  | 0.559 |
| walker |  | 8916 | 14 | Code::CodeKey { rung: Decl, file: pkg/model/planner.go, decl: 3, sub: 0, line: 30 } |  |  | 0.559 |
| walker |  | 8930 | 14 | Code::CodeKey { rung: Decl, file: pkg/model/planner.go, decl: 11, sub: 0, line: 196 } |  |  | 0.559 |
| walker |  | 8952 | 22 | Code::CodeKey { rung: Decl, file: pkg/model/planner.go, decl: 4, sub: 0, line: 35 } |  |  | 0.561 |
| walker |  | 8978 | 26 | Code::CodeKey { rung: Decl, file: pkg/model/planner.go, decl: 7, sub: 0, line: 53 } |  |  | 0.561 |
| ns | 9007 |  | 165 | Artifact server: Serve + the v4 route base | 6.6 |  | 0.556 |
| walker |  | 9038 | 60 | Code::CodeKey { rung: Decl, file: pkg/model/planner.go, decl: 1, sub: 0, line: 17 } |  |  | 0.561 |
| walker |  | 9051 | 13 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 1, sub: 0, line: 17 } |  |  | 0.564 |
| walker |  | 9064 | 13 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 6, sub: 0, line: 49 } |  |  | 0.565 |
| walker |  | 9077 | 13 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 18, sub: 0, line: 327 } |  |  | 0.565 |
| walker |  | 9091 | 14 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 2, sub: 0, line: 25 } |  |  | 0.568 |
| walker |  | 9105 | 14 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 3, sub: 0, line: 30 } |  |  | 0.570 |
| walker |  | 9121 | 16 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 4, sub: 0, line: 35 } |  |  | 0.573 |
| walker |  | 9137 | 16 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 15, sub: 0, line: 274 } |  |  | 0.573 |
| walker |  | 9154 | 17 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 14, sub: 0, line: 252 } |  |  | 0.573 |
| walker |  | 9172 | 18 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 16, sub: 0, line: 304 } |  |  | 0.573 |
| walker |  | 9190 | 18 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 17, sub: 0, line: 318 } |  |  | 0.573 |
| walker |  | 9210 | 20 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 12, sub: 0, line: 201 } |  |  | 0.573 |
| walker |  | 9230 | 20 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 13, sub: 0, line: 232 } |  |  | 0.573 |
| walker |  | 9252 | 22 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 20, sub: 0, line: 384 } |  |  | 0.573 |
| ns | 9312 |  | 305 | pkg/common: complete exported-function roster | 6.7 |  | 0.564 |
| ns | 9475 |  | 163 | pkg/common/git: repo detection and cloning | 6.8 |  | 0.559 |
| ns | 9601 |  | 126 | CI, issue-template and editor directories | 7.1 |  | 0.561 |
| walker |  | 9617 | 365 | Code::CodeKey { rung: Names, file: pkg/common/executor.go, decl: 0, sub: 0, line: 0 } |  |  | 0.575 |
| walker |  | 9629 | 12 | Code::CodeKey { rung: Decl, file: pkg/common/executor.go, decl: 1, sub: 0, line: 12 } |  |  | 0.575 |
| walker |  | 9638 | 9 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 2, sub: 0, line: 17 } |  |  | 0.575 |
| walker |  | 9649 | 11 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 3, sub: 0, line: 22 } |  |  | 0.575 |
| walker |  | 9660 | 11 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 5, sub: 0, line: 33 } |  |  | 0.578 |
| walker |  | 9671 | 11 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 20, sub: 0, line: 237 } |  |  | 0.578 |
| walker |  | 9684 | 13 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 6, sub: 0, line: 36 } |  |  | 0.578 |
| walker |  | 9698 | 14 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 13, sub: 0, line: 141 } |  |  | 0.578 |
| walker |  | 9712 | 14 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 14, sub: 0, line: 160 } |  |  | 0.578 |
| walker |  | 9726 | 14 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 15, sub: 0, line: 179 } |  |  | 0.578 |
| walker |  | 9741 | 15 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 4, sub: 0, line: 30 } |  |  | 0.580 |
| walker |  | 9756 | 15 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 7, sub: 0, line: 45 } |  |  | 0.580 |
| walker |  | 9771 | 15 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 16, sub: 0, line: 198 } |  |  | 0.580 |
| walker |  | 9786 | 15 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 19, sub: 0, line: 225 } |  |  | 0.580 |
| walker |  | 9802 | 16 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 1, sub: 0, line: 12 } |  |  | 0.580 |
| walker |  | 9818 | 16 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 9, sub: 0, line: 72 } |  |  | 0.580 |
| walker |  | 9834 | 16 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 17, sub: 0, line: 208 } |  |  | 0.580 |
| walker |  | 9850 | 16 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 18, sub: 0, line: 218 } |  |  | 0.580 |
| ns | 9854 |  | 253 | Key direct dependencies | 7.2 |  | 0.587 |
| walker |  | 9867 | 17 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 10, sub: 0, line: 88 } |  |  | 0.587 |
| walker |  | 9887 | 20 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 8, sub: 0, line: 54 } |  |  | 0.587 |
| walker |  | 9907 | 20 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 11, sub: 0, line: 95 } |  |  | 0.587 |
| walker |  | 9936 | 29 | Code::CodeKey { rung: Body, file: cmd/notices.go, decl: 4, sub: 0, line: 57 } |  |  | 0.587 |
| walker |  | 9990 | 54 | Code::CodeKey { rung: Names, file: pkg/common/job_error.go, decl: 0, sub: 0, line: 0 } |  |  | 0.587 |

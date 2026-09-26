Score(3000)=0.581 I=0.870 C=0.388 ns_rows≤3K=18/47 grid(1000/1442/2080/3000/4327/6240/9000)=0.757/0.649/0.566/0.581/0.505/0.425/0.526

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
| walker |  | 2870 | 28 | Code::CodeKey { rung: Names, file: cmd/root.go, decl: 0, sub: 0, line: 0 } |  |  | 0.591 |
| ns | 2918 |  | 110 | Workflow struct | 3.2 | 3.1 | 0.580 |
| walker |  | 2922 | 52 | Code::CodeKey { rung: Decl, file: cmd/root.go, decl: 1, sub: 0, line: 37 } |  |  | 0.580 |
| walker |  | 2937 | 15 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 2, sub: 0, line: 47 } |  |  | 0.581 |
| walker |  | 2944 | 7 | Plaintext::DeclSurface { file: pkg/container/testdata/scratch/test.txt } |  |  | 0.581 |
| walker |  | 3011 | 67 | Plaintext::Whole { file: pkg/container/testdata/Dockerfile } |  |  | 0.581 |
| walker |  | 3019 | 8 | Plaintext::DeclSurface { file: pkg/exprparser/testdata/for-hashing-3/data.txt } |  |  | 0.581 |
| ns | 3148 |  | 230 | Planner interface + Plan/Stage/Run | 3.3 | 3.1 | 0.553 |
| walker |  | 3151 | 132 | Code::CodeKey { rung: Names, file: cmd/input.go, decl: 0, sub: 0, line: 0 } |  |  | 0.568 |
| ns | 3410 |  | 262 | Job struct: every supported job key | 3.4 | 3.1 | 0.550 |
| ns | 3634 |  | 224 | Step struct: every supported step key | 3.5 | 3.1 | 0.536 |
| walker |  | 3734 | 583 | Code::CodeKey { rung: Decl, file: cmd/input.go, decl: 1, sub: 0, line: 10 } |  |  | 0.538 |
| walker |  | 3746 | 12 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 3, sub: 0, line: 90 } |  |  | 0.538 |
| walker |  | 3759 | 13 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 2, sub: 0, line: 85 } |  |  | 0.538 |
| walker |  | 3772 | 13 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 5, sub: 0, line: 99 } |  |  | 0.538 |
| walker |  | 3786 | 14 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 1, sub: 0, line: 10 } |  |  | 0.544 |
| walker |  | 3800 | 14 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 7, sub: 0, line: 109 } |  |  | 0.544 |
| walker |  | 3815 | 15 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 6, sub: 0, line: 104 } |  |  | 0.544 |
| walker |  | 3830 | 15 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 8, sub: 0, line: 114 } |  |  | 0.544 |
| walker |  | 3864 | 34 | Code::CodeKey { rung: Names, file: cmd/notices.go, decl: 0, sub: 0, line: 0 } |  |  | 0.544 |
| walker |  | 3893 | 29 | Code::CodeKey { rung: Decl, file: cmd/notices.go, decl: 1, sub: 0, line: 17 } |  |  | 0.544 |
| walker |  | 3944 | 51 | Code::CodeKey { rung: Names, file: cmd/secrets.go, decl: 0, sub: 0, line: 0 } |  |  | 0.545 |
| walker |  | 3951 | 7 | Code::CodeKey { rung: Body, file: cmd/secrets.go, decl: 3, sub: 0, line: 40 } |  |  | 0.545 |
| ns | 3966 |  | 332 | StepType constants | 3.6 | 3.1 | 0.524 |
| ns | 4187 |  | 221 | GithubContext: complete field-name roster | 3.7 | 3.1 | 0.503 |
| walker |  | 4219 | 268 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.503 |
| walker |  | 4226 | 7 | Plaintext::DeclSurface { file: pkg/exprparser/testdata/for-hashing-3/nested/nested-data.txt } |  |  | 0.503 |
| walker |  | 4247 | 21 | Code::CodeKey { rung: Names, file: cmd/platforms.go, decl: 0, sub: 0, line: 0 } |  |  | 0.503 |
| walker |  | 4277 | 30 | Code::CodeKey { rung: Names, file: cmd/dir.go, decl: 0, sub: 0, line: 0 } |  |  | 0.504 |
| walker |  | 4282 | 5 | Code::CodeKey { rung: Decl, file: cmd/dir.go, decl: 1, sub: 0, line: 10 } |  |  | 0.504 |
| walker |  | 4300 | 18 | Code::CodeKey { rung: Names, file: cmd/list.go, decl: 0, sub: 0, line: 0 } |  |  | 0.505 |
| walker |  | 4318 | 18 | Code::CodeKey { rung: Names, file: cmd/graph.go, decl: 0, sub: 0, line: 0 } |  |  | 0.505 |
| ns | 4470 |  | 283 | Executor type + complete combinator roster | 4.1 |  | 0.490 |
| walker |  | 4487 | 169 | Code::CodeKey { rung: Names, file: pkg/exprparser/interpreter.go, decl: 0, sub: 0, line: 0 } |  |  | 0.490 |
| walker |  | 4490 | 3 | Code::CodeKey { rung: Decl, file: pkg/exprparser/interpreter.go, decl: 5, sub: 0, line: 43 } |  |  | 0.490 |
| walker |  | 4516 | 26 | Code::CodeKey { rung: Decl, file: pkg/exprparser/interpreter.go, decl: 7, sub: 0, line: 65 } |  |  | 0.490 |
| walker |  | 4546 | 30 | Code::CodeKey { rung: Decl, file: pkg/exprparser/interpreter.go, decl: 2, sub: 0, line: 30 } |  |  | 0.490 |
| walker |  | 4576 | 30 | Code::CodeKey { rung: Decl, file: pkg/exprparser/interpreter.go, decl: 3, sub: 0, line: 35 } |  |  | 0.490 |
| walker |  | 4738 | 162 | Code::CodeKey { rung: Decl, file: pkg/exprparser/interpreter.go, decl: 1, sub: 0, line: 14 } |  |  | 0.490 |
| ns | 5005 |  | 535 | pkg/runner: complete type roster | 4.2 |  | 0.468 |
| ns | 5115 |  | 110 | Runner interface and New() | 4.3 | 4.2 | 0.462 |
| ns | 5503 |  | 388 | runner.Config: fields (first half, rest elided) | 4.4 | 4.2 | 0.449 |
| walker |  | 5547 | 809 | GoMod::File { file: go.mod } |  |  | 0.450 |
| walker |  | 5804 | 257 | Code::CodeKey { rung: Names, file: pkg/schema/schema.go, decl: 0, sub: 0, line: 0 } |  |  | 0.450 |
| ns | 5805 |  | 302 | RunContext struct | 4.5 | 4.2 | 0.438 |
| walker |  | 5807 | 3 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 9, sub: 0, line: 92 } |  |  | 0.438 |
| walker |  | 5810 | 3 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 10, sub: 0, line: 95 } |  |  | 0.438 |
| walker |  | 5813 | 3 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 11, sub: 0, line: 98 } |  |  | 0.438 |
| walker |  | 5827 | 14 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 1, sub: 0, line: 25 } |  |  | 0.438 |
| walker |  | 5843 | 16 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 7, sub: 0, line: 83 } |  |  | 0.438 |
| walker |  | 5862 | 19 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 5, sub: 0, line: 70 } |  |  | 0.438 |
| walker |  | 5887 | 25 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 8, sub: 0, line: 87 } |  |  | 0.438 |
| walker |  | 5915 | 28 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 15, sub: 0, line: 119 } |  |  | 0.438 |
| walker |  | 5944 | 29 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 14, sub: 0, line: 113 } |  |  | 0.438 |
| walker |  | 5995 | 51 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 4, sub: 0, line: 64 } |  |  | 0.438 |
| walker |  | 6101 | 106 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 3, sub: 0, line: 52 } |  |  | 0.438 |
| walker |  | 6131 | 30 | Code::CodeKey { rung: Names, file: pkg/lookpath/env.go, decl: 0, sub: 0, line: 0 } |  |  | 0.438 |
| walker |  | 6147 | 16 | Code::CodeKey { rung: Decl, file: pkg/lookpath/env.go, decl: 1, sub: 0, line: 5 } |  |  | 0.438 |
| ns | 6160 |  | 355 | stepFactory: StepType -> implementation | 4.6 | 4.2 | 0.424 |
| walker |  | 6161 | 14 | Code::CodeKey { rung: Body, file: pkg/lookpath/env.go, decl: 2, sub: 0, line: 16 } |  |  | 0.424 |
| walker |  | 6189 | 28 | Code::CodeKey { rung: Names, file: pkg/lookpath/error.go, decl: 0, sub: 0, line: 0 } |  |  | 0.424 |
| walker |  | 6207 | 18 | Code::CodeKey { rung: Decl, file: pkg/lookpath/error.go, decl: 1, sub: 0, line: 3 } |  |  | 0.424 |
| walker |  | 6216 | 9 | Code::CodeKey { rung: Body, file: pkg/lookpath/error.go, decl: 2, sub: 0, line: 8 } |  |  | 0.424 |
| walker |  | 6436 | 220 | Code::CodeKey { rung: Names, file: pkg/model/workflow.go, decl: 0, sub: 0, line: 0 } |  |  | 0.429 |
| ns | 6438 |  | 278 | step interface and stage enum | 4.7 | 4.2 | 0.416 |
| walker |  | 6449 | 13 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 13, sub: 0, line: 171 } |  |  | 0.416 |
| walker |  | 6469 | 20 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 8, sub: 0, line: 113 } |  |  | 0.416 |
| walker |  | 6495 | 26 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 11, sub: 0, line: 161 } |  |  | 0.416 |
| walker |  | 6534 | 39 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 12, sub: 0, line: 166 } |  |  | 0.416 |
| walker |  | 6589 | 55 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 10, sub: 0, line: 154 } |  |  | 0.416 |
| walker |  | 6658 | 69 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 7, sub: 0, line: 105 } |  |  | 0.416 |
| walker |  | 6745 | 87 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 1, sub: 0, line: 19 } |  |  | 0.427 |
| walker |  | 6754 | 9 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 2, sub: 0, line: 29 } |  |  | 0.427 |
| walker |  | 6772 | 18 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 1, sub: 0, line: 19 } |  |  | 0.431 |
| ns | 6933 |  | 495 | run_context.go: method roster (container lifecycle) | 4.8 |  | 0.419 |
| walker |  | 6974 | 202 | Code::CodeKey { rung: Names, file: pkg/model/workflow.go, decl: 0, sub: 1, line: 0 } |  |  | 0.423 |
| walker |  | 6991 | 17 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 17, sub: 0, line: 225 } |  |  | 0.423 |
| walker |  | 7023 | 32 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 18, sub: 0, line: 230 } |  |  | 0.423 |
| ns | 7079 |  | 146 | ActionCache implementations | 4.9 | 4.2 | 0.416 |
| walker |  | 7095 | 72 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 16, sub: 0, line: 216 } |  |  | 0.416 |
| ns | 7239 |  | 160 | pkg/container: complete file list | 5.1 |  | 0.445 |
| walker |  | 7341 | 246 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 15, sub: 0, line: 196 } |  |  | 0.468 |
| walker |  | 7349 | 8 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 16, sub: 0, line: 216 } |  |  | 0.468 |
| walker |  | 7359 | 10 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 24, sub: 0, line: 311 } |  |  | 0.468 |
| walker |  | 7370 | 11 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 23, sub: 0, line: 293 } |  |  | 0.468 |
| walker |  | 7381 | 11 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 25, sub: 0, line: 330 } |  |  | 0.468 |
| walker |  | 7394 | 13 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 27, sub: 0, line: 388 } |  |  | 0.468 |
| walker |  | 7408 | 14 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 18, sub: 0, line: 230 } |  |  | 0.468 |
| walker |  | 7424 | 16 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 15, sub: 0, line: 196 } |  |  | 0.471 |
| walker |  | 7440 | 16 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 26, sub: 0, line: 383 } |  |  | 0.471 |
| walker |  | 7457 | 17 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 17, sub: 0, line: 225 } |  |  | 0.471 |
| walker |  | 7474 | 17 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 19, sub: 0, line: 236 } |  |  | 0.471 |
| walker |  | 7492 | 18 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 20, sub: 0, line: 253 } |  |  | 0.471 |
| ns | 7516 |  | 277 | Container interface | 5.2 |  | 0.464 |
| ns | 7658 |  | 142 | ExecutionsEnvironment interface | 5.3 |  | 0.459 |
| walker |  | 7719 | 227 | Code::CodeKey { rung: Names, file: pkg/model/workflow.go, decl: 0, sub: 2, line: 0 } |  |  | 0.462 |
| walker |  | 7830 | 111 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 30, sub: 0, line: 503 } |  |  | 0.462 |
| ns | 7923 |  | 265 | Docker backend vs docker-less stub: the build-tag split | 5.4 |  | 0.455 |
| walker |  | 7959 | 129 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 33, sub: 0, line: 555 } |  |  | 0.455 |
| ns | 8100 |  | 177 | Docker socket discovery | 5.5 |  | 0.450 |
| walker |  | 8167 | 208 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 34, sub: 0, line: 569 } |  |  | 0.467 |
| walker |  | 8177 | 10 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 35, sub: 0, line: 586 } |  |  | 0.467 |
| walker |  | 8190 | 13 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 32, sub: 0, line: 530 } |  |  | 0.482 |
| ns | 8190 |  | 90 | Supporting packages: complete file lists (A) | 6.1 |  | 0.482 |
| walker |  | 8204 | 14 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 34, sub: 0, line: 569 } |  |  | 0.484 |
| walker |  | 8218 | 14 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 37, sub: 0, line: 603 } |  |  | 0.484 |
| walker |  | 8232 | 14 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 38, sub: 0, line: 615 } |  |  | 0.484 |
| walker |  | 8248 | 16 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 36, sub: 0, line: 598 } |  |  | 0.484 |
| walker |  | 8266 | 18 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 29, sub: 0, line: 501 } |  |  | 0.484 |
| walker |  | 8285 | 19 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 33, sub: 0, line: 555 } |  |  | 0.484 |
| ns | 8349 |  | 159 | Supporting packages: complete file lists (B) | 6.2 |  | 0.506 |
| ns | 8493 |  | 144 | Context names the expression interpreter resolves | 6.3 |  | 0.501 |
| walker |  | 8518 | 233 | Code::CodeKey { rung: Names, file: pkg/model/workflow.go, decl: 0, sub: 3, line: 0 } |  |  | 0.504 |
| walker |  | 8549 | 31 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 46, sub: 0, line: 750 } |  |  | 0.504 |
| ns | 8618 |  | 125 | Expression functions act implements | 6.4 |  | 0.500 |
| walker |  | 8758 | 209 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 40, sub: 0, line: 647 } |  |  | 0.521 |
| walker |  | 8771 | 13 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 42, sub: 0, line: 691 } |  |  | 0.521 |
| walker |  | 8788 | 17 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 44, sub: 0, line: 726 } |  |  | 0.521 |
| walker |  | 8806 | 18 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 39, sub: 0, line: 645 } |  |  | 0.524 |
| walker |  | 8824 | 18 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 45, sub: 0, line: 742 } |  |  | 0.524 |
| ns | 8842 |  | 224 | Artifact cache server: routes and lifecycle | 6.5 |  | 0.519 |
| walker |  | 8843 | 19 | Code::CodeKey { rung: Doc, file: pkg/model/workflow.go, decl: 43, sub: 0, line: 714 } |  |  | 0.519 |
| ns | 9007 |  | 165 | Artifact server: Serve + the v4 route base | 6.6 |  | 0.514 |
| walker |  | 9021 | 178 | Code::CodeKey { rung: Names, file: pkg/model/planner.go, decl: 0, sub: 0, line: 0 } |  |  | 0.522 |
| walker |  | 9035 | 14 | Code::CodeKey { rung: Decl, file: pkg/model/planner.go, decl: 2, sub: 0, line: 25 } |  |  | 0.522 |
| walker |  | 9049 | 14 | Code::CodeKey { rung: Decl, file: pkg/model/planner.go, decl: 3, sub: 0, line: 30 } |  |  | 0.523 |
| walker |  | 9071 | 22 | Code::CodeKey { rung: Decl, file: pkg/model/planner.go, decl: 4, sub: 0, line: 35 } |  |  | 0.524 |
| walker |  | 9097 | 26 | Code::CodeKey { rung: Decl, file: pkg/model/planner.go, decl: 7, sub: 0, line: 53 } |  |  | 0.524 |
| walker |  | 9157 | 60 | Code::CodeKey { rung: Decl, file: pkg/model/planner.go, decl: 1, sub: 0, line: 17 } |  |  | 0.530 |
| walker |  | 9170 | 13 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 1, sub: 0, line: 17 } |  |  | 0.532 |
| walker |  | 9183 | 13 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 6, sub: 0, line: 49 } |  |  | 0.533 |
| walker |  | 9197 | 14 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 2, sub: 0, line: 25 } |  |  | 0.536 |
| walker |  | 9211 | 14 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 3, sub: 0, line: 30 } |  |  | 0.538 |
| walker |  | 9227 | 16 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 4, sub: 0, line: 35 } |  |  | 0.542 |
| walker |  | 9245 | 18 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 10, sub: 0, line: 304 } |  |  | 0.542 |
| walker |  | 9263 | 18 | Code::CodeKey { rung: Doc, file: pkg/model/planner.go, decl: 11, sub: 0, line: 318 } |  |  | 0.542 |
| ns | 9312 |  | 305 | pkg/common: complete exported-function roster | 6.7 |  | 0.533 |
| ns | 9475 |  | 163 | pkg/common/git: repo detection and cloning | 6.8 |  | 0.528 |
| ns | 9601 |  | 126 | CI, issue-template and editor directories | 7.1 |  | 0.531 |
| walker |  | 9628 | 365 | Code::CodeKey { rung: Names, file: pkg/common/executor.go, decl: 0, sub: 0, line: 0 } |  |  | 0.546 |
| walker |  | 9640 | 12 | Code::CodeKey { rung: Decl, file: pkg/common/executor.go, decl: 1, sub: 0, line: 12 } |  |  | 0.546 |
| walker |  | 9649 | 9 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 2, sub: 0, line: 17 } |  |  | 0.546 |
| walker |  | 9660 | 11 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 3, sub: 0, line: 22 } |  |  | 0.546 |
| walker |  | 9671 | 11 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 5, sub: 0, line: 33 } |  |  | 0.548 |
| walker |  | 9682 | 11 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 20, sub: 0, line: 237 } |  |  | 0.548 |
| walker |  | 9695 | 13 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 6, sub: 0, line: 36 } |  |  | 0.548 |
| walker |  | 9709 | 14 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 13, sub: 0, line: 141 } |  |  | 0.548 |
| walker |  | 9723 | 14 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 14, sub: 0, line: 160 } |  |  | 0.548 |
| walker |  | 9737 | 14 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 15, sub: 0, line: 179 } |  |  | 0.548 |
| walker |  | 9752 | 15 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 4, sub: 0, line: 30 } |  |  | 0.551 |
| walker |  | 9767 | 15 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 7, sub: 0, line: 45 } |  |  | 0.551 |
| walker |  | 9782 | 15 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 16, sub: 0, line: 198 } |  |  | 0.551 |
| walker |  | 9797 | 15 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 19, sub: 0, line: 225 } |  |  | 0.551 |
| walker |  | 9813 | 16 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 1, sub: 0, line: 12 } |  |  | 0.551 |
| walker |  | 9829 | 16 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 9, sub: 0, line: 72 } |  |  | 0.551 |
| walker |  | 9845 | 16 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 17, sub: 0, line: 208 } |  |  | 0.551 |
| ns | 9854 |  | 253 | Key direct dependencies | 7.2 |  | 0.558 |
| walker |  | 9861 | 16 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 18, sub: 0, line: 218 } |  |  | 0.558 |
| walker |  | 9878 | 17 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 10, sub: 0, line: 88 } |  |  | 0.558 |
| walker |  | 9898 | 20 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 8, sub: 0, line: 54 } |  |  | 0.558 |
| walker |  | 9918 | 20 | Code::CodeKey { rung: Doc, file: pkg/common/executor.go, decl: 11, sub: 0, line: 95 } |  |  | 0.558 |
| walker |  | 9990 | 72 | Code::CodeKey { rung: Names, file: pkg/common/job_error.go, decl: 0, sub: 0, line: 0 } |  |  | 0.559 |

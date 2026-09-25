Score(3000)=0.578 I=0.857 C=0.390 ns_rows≤3K=18/47 grid(1000/1442/2080/3000/4327/6240/9000)=0.483/0.681/0.594/0.578/0.480/0.412/0.463

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
| walker |  | 522 | 25 | Code::CodeKey { rung: Names, file: main.go, decl: 0, sub: 0, line: 0 } |  |  | 0.591 |
| walker |  | 532 | 10 | Code::CodeKey { rung: Doc, file: main.go, decl: 1, sub: 0, line: 11 } |  |  | 0.616 |
| walker |  | 542 | 10 | Plaintext::Whole { file: VERSION } |  |  | 0.618 |
| ns | 591 |  | 186 | pkg/runner/: complete file list | 1.7 |  | 0.497 |
| walker |  | 621 | 79 | Fs::DirListing { dir: pkg/common } |  |  | 0.500 |
| walker |  | 630 | 9 | Fs::DirListing { dir: pkg/common/git } |  |  | 0.500 |
| walker |  | 648 | 18 | Fs::DirListing { dir: pkg/artifacts/testdata } |  |  | 0.501 |
| walker |  | 652 | 4 | Fs::DirListing { dir: pkg/artifacts/testdata/GHSL-2023-004 } |  |  | 0.501 |
| walker |  | 656 | 4 | Fs::DirListing { dir: pkg/artifacts/testdata/upload-and-download } |  |  | 0.501 |
| walker |  | 660 | 4 | Fs::DirListing { dir: pkg/artifacts/testdata/v4 } |  |  | 0.501 |
| walker |  | 681 | 21 | Fs::DirListing { dir: pkg/model/testdata } |  |  | 0.501 |
| walker |  | 685 | 4 | Fs::DirListing { dir: pkg/model/testdata/container-volumes } |  |  | 0.501 |
| walker |  | 689 | 4 | Fs::DirListing { dir: pkg/model/testdata/strategy } |  |  | 0.501 |
| walker |  | 695 | 6 | Fs::DirListing { dir: pkg/model/testdata/empty-workflow } |  |  | 0.501 |
| walker |  | 702 | 7 | Fs::DirListing { dir: pkg/model/testdata/nested } |  |  | 0.501 |
| walker |  | 708 | 6 | Fs::DirListing { dir: pkg/model/testdata/nested/workflows } |  |  | 0.501 |
| walker |  | 731 | 23 | Fs::DirListing { dir: pkg/exprparser/testdata } |  |  | 0.501 |
| walker |  | 738 | 7 | Fs::DirListing { dir: pkg/exprparser/testdata/for-hashing-3 } |  |  | 0.501 |
| walker |  | 743 | 5 | Fs::DirListing { dir: pkg/exprparser/testdata/for-hashing-3/nested } |  |  | 0.501 |
| ns | 762 |  | 171 | README: how act works, end to end | 1.8 |  | 0.498 |
| walker |  | 763 | 20 | Fs::DirListing { dir: .github } |  |  | 0.499 |
| walker |  | 789 | 26 | Fs::DirListing { dir: .github/workflows } |  |  | 0.500 |
| ns | 804 |  | 42 | Module path, Go version, released version | 1.9 |  | 0.504 |
| ns | 919 |  | 115 | cmd.Execute: CLI entry | 2.1 |  | 0.478 |
| walker |  | 949 | 160 | Fs::DirListing { dir: pkg/container } |  |  | 0.483 |
| walker |  | 967 | 18 | Code::CodeKey { rung: ModuleDoc, file: pkg/container/docker_cli.go, decl: 0, sub: 0, line: 0 } |  |  | 0.483 |
| walker |  | 1004 | 37 | Fs::DirListing { dir: pkg/container/testdata } |  |  | 0.483 |
| walker |  | 1008 | 4 | Fs::DirListing { dir: pkg/container/testdata/docker-pull-options } |  |  | 0.483 |
| walker |  | 1012 | 4 | Fs::DirListing { dir: pkg/container/testdata/scratch } |  |  | 0.483 |
| ns | 1131 |  | 212 | Root cobra.Command definition | 2.2 |  | 0.460 |
| walker |  | 1197 | 185 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.600 |
| walker |  | 1252 | 55 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.600 |
| ns | 1432 |  | 301 | Command-local flags: names and shorthands | 2.3 |  | 0.536 |
| walker |  | 1438 | 186 | Fs::DirListing { dir: pkg/runner } |  |  | 0.681 |
| walker |  | 1442 | 4 | Fs::DirListing { dir: pkg/runner/hashfiles } |  |  | 0.681 |
| walker |  | 1446 | 4 | Fs::DirListing { dir: pkg/runner/res } |  |  | 0.681 |
| walker |  | 1470 | 24 | Fs::DirListing { dir: pkg/model/testdata/invalid-job-name } |  |  | 0.681 |
| ns | 1782 |  | 350 | Persistent flags: names and shorthands | 2.4 |  | 0.612 |
| ns | 1928 |  | 146 | Input struct + path accessor roster | 2.5 |  | 0.594 |
| walker |  | 2185 | 715 | Fs::DirListing { dir: pkg/runner/testdata } |  |  | 0.594 |
| walker |  | 2203 | 18 | Code::CodeKey { rung: Names, file: cmd/graph.go, decl: 0, sub: 0, line: 0 } |  |  | 0.594 |
| ns | 2206 |  | 278 | Config-file discovery: .actrc locations | 2.6 |  | 0.562 |
| walker |  | 2221 | 18 | Code::CodeKey { rung: Names, file: cmd/list.go, decl: 0, sub: 0, line: 0 } |  |  | 0.562 |
| walker |  | 2286 | 65 | Markdown::HeadingsOutline { file: IMAGES.md } |  |  | 0.562 |
| walker |  | 2334 | 48 | Code::CodeKey { rung: Body, file: main.go, decl: 2, sub: 0, line: 13 } |  |  | 0.663 |
| walker |  | 2355 | 21 | Code::CodeKey { rung: Names, file: cmd/platforms.go, decl: 0, sub: 0, line: 0 } |  |  | 0.663 |
| walker |  | 2383 | 28 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.663 |
| walker |  | 2396 | 13 | Code::CodeKey { rung: Names, file: pkg/model/job_context.go, decl: 0, sub: 0, line: 0 } |  |  | 0.663 |
| walker |  | 2424 | 28 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.663 |
| walker |  | 2438 | 14 | Code::CodeKey { rung: Names, file: pkg/container/executions_environment.go, decl: 0, sub: 0, line: 0 } |  |  | 0.663 |
| walker |  | 2454 | 16 | Code::CodeKey { rung: Names, file: pkg/common/outbound_ip.go, decl: 0, sub: 0, line: 0 } |  |  | 0.663 |
| walker |  | 2488 | 34 | Code::CodeKey { rung: Names, file: cmd/dir.go, decl: 0, sub: 0, line: 0 } |  |  | 0.664 |
| walker |  | 2497 | 9 | Code::CodeKey { rung: Decl, file: cmd/dir.go, decl: 1, sub: 0, line: 10 } |  |  | 0.664 |
| ns | 2522 |  | 316 | cmd package: complete function roster | 2.7 |  | 0.625 |
| walker |  | 2546 | 49 | Markdown::Section { file: IMAGES.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.625 |
| walker |  | 2571 | 25 | Code::CodeKey { rung: Names, file: pkg/gh/gh.go, decl: 0, sub: 0, line: 0 } |  |  | 0.625 |
| walker |  | 2622 | 51 | Code::CodeKey { rung: Names, file: cmd/secrets.go, decl: 0, sub: 0, line: 0 } |  |  | 0.628 |
| walker |  | 2629 | 7 | Code::CodeKey { rung: Body, file: cmd/secrets.go, decl: 3, sub: 0, line: 40 } |  |  | 0.628 |
| walker |  | 2657 | 28 | Code::CodeKey { rung: Names, file: pkg/container/parse_env_file.go, decl: 0, sub: 0, line: 0 } |  |  | 0.628 |
| walker |  | 2685 | 28 | Code::CodeKey { rung: Names, file: pkg/lookpath/error.go, decl: 0, sub: 0, line: 0 } |  |  | 0.628 |
| walker |  | 2703 | 18 | Code::CodeKey { rung: Decl, file: pkg/lookpath/error.go, decl: 1, sub: 0, line: 3 } |  |  | 0.628 |
| walker |  | 2712 | 9 | Code::CodeKey { rung: Body, file: pkg/lookpath/error.go, decl: 2, sub: 0, line: 8 } |  |  | 0.628 |
| walker |  | 2806 | 94 | Code::CodeKey { rung: Decl, file: pkg/model/job_context.go, decl: 1, sub: 0, line: 3 } |  |  | 0.628 |
| ns | 2869 |  | 347 | pkg/model: complete type roster | 3.1 |  | 0.589 |
| ns | 2979 |  | 110 | Workflow struct | 3.2 | 3.1 | 0.578 |
| walker |  | 3097 | 291 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.578 |
| walker |  | 3205 | 108 | Code::CodeKey { rung: Decl, file: pkg/container/executions_environment.go, decl: 1, sub: 0, line: 5 } |  |  | 0.579 |
| ns | 3209 |  | 230 | Planner interface + Plan/Stage/Run | 3.3 | 3.1 | 0.551 |
| walker |  | 3211 | 6 | Plaintext::DeclSurface { file: pkg/exprparser/testdata/for-hashing-1.txt } |  |  | 0.551 |
| walker |  | 3217 | 6 | Plaintext::DeclSurface { file: pkg/exprparser/testdata/for-hashing-2.txt } |  |  | 0.551 |
| walker |  | 3351 | 134 | Code::CodeKey { rung: ModuleDoc, file: pkg/artifactcache/doc.go, decl: 0, sub: 0, line: 0 } |  |  | 0.551 |
| walker |  | 3389 | 38 | Code::CodeKey { rung: Names, file: pkg/container/docker_network.go, decl: 0, sub: 0, line: 0 } |  |  | 0.551 |
| walker |  | 3429 | 40 | Code::CodeKey { rung: Names, file: pkg/artifactcache/model.go, decl: 0, sub: 0, line: 0 } |  |  | 0.551 |
| ns | 3471 |  | 262 | Job struct: every supported job key | 3.4 | 3.1 | 0.534 |
| walker |  | 3472 | 43 | Code::CodeKey { rung: Decl, file: pkg/artifactcache/model.go, decl: 1, sub: 0, line: 3 } |  |  | 0.534 |
| walker |  | 3514 | 42 | Code::CodeKey { rung: Names, file: pkg/common/file.go, decl: 0, sub: 0, line: 0 } |  |  | 0.534 |
| walker |  | 3524 | 10 | Code::CodeKey { rung: Doc, file: pkg/common/file.go, decl: 1, sub: 0, line: 10 } |  |  | 0.534 |
| walker |  | 3536 | 12 | Code::CodeKey { rung: Doc, file: pkg/common/file.go, decl: 2, sub: 0, line: 37 } |  |  | 0.534 |
| walker |  | 3578 | 42 | Code::CodeKey { rung: Names, file: pkg/container/docker_volume.go, decl: 0, sub: 0, line: 0 } |  |  | 0.534 |
| walker |  | 3582 | 4 | Fs::DirListing { dir: .github/actions } |  |  | 0.534 |
| walker |  | 3625 | 43 | Code::CodeKey { rung: Names, file: pkg/common/cartesian.go, decl: 0, sub: 0, line: 0 } |  |  | 0.534 |
| walker |  | 3643 | 18 | Code::CodeKey { rung: Doc, file: pkg/common/cartesian.go, decl: 1, sub: 0, line: 4 } |  |  | 0.534 |
| ns | 3695 |  | 224 | Step struct: every supported step key | 3.5 | 3.1 | 0.520 |
| walker |  | 3790 | 147 | Code::CodeKey { rung: Decl, file: pkg/artifactcache/model.go, decl: 3, sub: 0, line: 26 } |  |  | 0.520 |
| walker |  | 3837 | 47 | Code::CodeKey { rung: Names, file: pkg/model/anchors.go, decl: 0, sub: 0, line: 0 } |  |  | 0.520 |
| walker |  | 3885 | 48 | Code::CodeKey { rung: Names, file: pkg/common/context.go, decl: 0, sub: 0, line: 0 } |  |  | 0.520 |
| walker |  | 3933 | 48 | Code::CodeKey { rung: Names, file: pkg/container/util.go, decl: 0, sub: 0, line: 0 } |  |  | 0.520 |
| walker |  | 3981 | 48 | Code::CodeKey { rung: Names, file: pkg/lookpath/lp_js.go, decl: 0, sub: 0, line: 0 } |  |  | 0.520 |
| walker |  | 4004 | 23 | Code::CodeKey { rung: Doc, file: pkg/lookpath/lp_js.go, decl: 1, sub: 0, line: 14 } |  |  | 0.520 |
| ns | 4027 |  | 332 | StepType constants | 3.6 | 3.1 | 0.501 |
| walker |  | 4053 | 49 | Code::CodeKey { rung: Names, file: pkg/container/util_openbsd_mips64.go, decl: 0, sub: 0, line: 0 } |  |  | 0.501 |
| walker |  | 4102 | 49 | Code::CodeKey { rung: Names, file: pkg/container/util_plan9.go, decl: 0, sub: 0, line: 0 } |  |  | 0.501 |
| walker |  | 4151 | 49 | Code::CodeKey { rung: Names, file: pkg/container/util_windows.go, decl: 0, sub: 0, line: 0 } |  |  | 0.501 |
| walker |  | 4177 | 26 | Code::CodeKey { rung: Body, file: pkg/common/context.go, decl: 2, sub: 0, line: 42 } |  |  | 0.501 |
| walker |  | 4227 | 50 | Code::CodeKey { rung: Names, file: pkg/container/docker_auth.go, decl: 0, sub: 0, line: 0 } |  |  | 0.501 |
| ns | 4248 |  | 221 | GithubContext: complete field-name roster | 3.7 | 3.1 | 0.480 |
| walker |  | 4281 | 54 | Code::CodeKey { rung: Names, file: pkg/container/docker_build.go, decl: 0, sub: 0, line: 0 } |  |  | 0.480 |
| walker |  | 4300 | 19 | Code::CodeKey { rung: Doc, file: pkg/container/docker_build.go, decl: 1, sub: 0, line: 23 } |  |  | 0.480 |
| ns | 4531 |  | 283 | Executor type + complete combinator roster | 4.1 |  | 0.466 |
| ns | 5066 |  | 535 | pkg/runner: complete type roster | 4.2 |  | 0.445 |
| walker |  | 5109 | 809 | GoMod::File { file: go.mod } |  |  | 0.445 |
| walker |  | 5118 | 9 | Code::CodeKey { rung: Body, file: pkg/container/util.go, decl: 2, sub: 0, line: 24 } |  |  | 0.445 |
| ns | 5176 |  | 110 | Runner interface and New() | 4.3 | 4.2 | 0.439 |
| walker |  | 5177 | 59 | Code::CodeKey { rung: Names, file: pkg/lookpath/env.go, decl: 0, sub: 0, line: 0 } |  |  | 0.439 |
| walker |  | 5180 | 3 | Code::CodeKey { rung: Decl, file: pkg/lookpath/env.go, decl: 2, sub: 0, line: 9 } |  |  | 0.439 |
| walker |  | 5194 | 14 | Code::CodeKey { rung: Decl, file: pkg/lookpath/env.go, decl: 1, sub: 0, line: 5 } |  |  | 0.439 |
| walker |  | 5208 | 14 | Code::CodeKey { rung: Body, file: pkg/lookpath/env.go, decl: 4, sub: 0, line: 16 } |  |  | 0.439 |
| walker |  | 5217 | 9 | Code::CodeKey { rung: Body, file: pkg/lookpath/env.go, decl: 3, sub: 0, line: 12 } |  |  | 0.439 |
| walker |  | 5277 | 60 | Code::CodeKey { rung: Names, file: pkg/container/docker_images.go, decl: 0, sub: 0, line: 0 } |  |  | 0.439 |
| walker |  | 5307 | 30 | Code::CodeKey { rung: Doc, file: pkg/container/docker_images.go, decl: 2, sub: 0, line: 44 } |  |  | 0.439 |
| walker |  | 5368 | 61 | Code::CodeKey { rung: Names, file: pkg/runner/step_factory.go, decl: 0, sub: 0, line: 0 } |  |  | 0.439 |
| walker |  | 5392 | 24 | Code::CodeKey { rung: Decl, file: pkg/runner/step_factory.go, decl: 1, sub: 0, line: 9 } |  |  | 0.439 |
| walker |  | 5399 | 7 | Plaintext::DeclSurface { file: pkg/container/testdata/scratch/test.txt } |  |  | 0.439 |
| walker |  | 5464 | 65 | Code::CodeKey { rung: Names, file: pkg/lookpath/lp_plan9.go, decl: 0, sub: 0, line: 0 } |  |  | 0.439 |
| walker |  | 5487 | 23 | Code::CodeKey { rung: Doc, file: pkg/lookpath/lp_plan9.go, decl: 1, sub: 0, line: 16 } |  |  | 0.439 |
| walker |  | 5552 | 65 | Code::CodeKey { rung: Names, file: pkg/lookpath/lp_unix.go, decl: 0, sub: 0, line: 0 } |  |  | 0.439 |
| ns | 5564 |  | 388 | runner.Config: fields (first half, rest elided) | 4.4 | 4.2 | 0.428 |
| walker |  | 5575 | 23 | Code::CodeKey { rung: Doc, file: pkg/lookpath/lp_unix.go, decl: 1, sub: 0, line: 18 } |  |  | 0.428 |
| walker |  | 5642 | 67 | Plaintext::Whole { file: pkg/container/testdata/Dockerfile } |  |  | 0.428 |
| walker |  | 5680 | 38 | Code::CodeKey { rung: Doc, file: pkg/container/docker_images.go, decl: 1, sub: 0, line: 16 } |  |  | 0.428 |
| walker |  | 5752 | 72 | Code::CodeKey { rung: Names, file: pkg/container/docker_pull.go, decl: 0, sub: 0, line: 0 } |  |  | 0.428 |
| walker |  | 5771 | 19 | Code::CodeKey { rung: Doc, file: pkg/container/docker_pull.go, decl: 1, sub: 0, line: 21 } |  |  | 0.428 |
| walker |  | 5779 | 8 | Plaintext::DeclSurface { file: pkg/exprparser/testdata/for-hashing-3/data.txt } |  |  | 0.428 |
| walker |  | 5854 | 75 | Code::CodeKey { rung: Names, file: pkg/common/dryrun.go, decl: 0, sub: 0, line: 0 } |  |  | 0.428 |
| ns | 5866 |  | 302 | RunContext struct | 4.5 | 4.2 | 0.416 |
| walker |  | 5869 | 15 | Code::CodeKey { rung: Doc, file: pkg/common/dryrun.go, decl: 3, sub: 0, line: 12 } |  |  | 0.416 |
| walker |  | 5887 | 18 | Code::CodeKey { rung: Doc, file: pkg/common/dryrun.go, decl: 4, sub: 0, line: 23 } |  |  | 0.416 |
| walker |  | 5906 | 19 | Code::CodeKey { rung: Body, file: pkg/common/dryrun.go, decl: 4, sub: 0, line: 23 } |  |  | 0.416 |
| walker |  | 5946 | 40 | Code::CodeKey { rung: Body, file: pkg/lookpath/lp_js.go, decl: 2, sub: 0, line: 20 } |  |  | 0.416 |
| walker |  | 6093 | 147 | Code::CodeKey { rung: Names, file: cmd/notices.go, decl: 0, sub: 0, line: 0 } |  |  | 0.425 |
| walker |  | 6120 | 27 | Code::CodeKey { rung: Decl, file: cmd/notices.go, decl: 1, sub: 0, line: 17 } |  |  | 0.425 |
| walker |  | 6197 | 77 | Code::CodeKey { rung: Names, file: pkg/common/logger.go, decl: 0, sub: 0, line: 0 } |  |  | 0.425 |
| walker |  | 6209 | 12 | Code::CodeKey { rung: Doc, file: pkg/common/logger.go, decl: 3, sub: 0, line: 14 } |  |  | 0.425 |
| ns | 6221 |  | 355 | stepFactory: StepType -> implementation | 4.6 | 4.2 | 0.412 |
| walker |  | 6226 | 17 | Code::CodeKey { rung: Doc, file: pkg/common/logger.go, decl: 4, sub: 0, line: 25 } |  |  | 0.412 |
| walker |  | 6243 | 17 | Code::CodeKey { rung: Body, file: pkg/common/logger.go, decl: 4, sub: 0, line: 25 } |  |  | 0.412 |
| walker |  | 6393 | 150 | Code::CodeKey { rung: Names, file: cmd/input.go, decl: 0, sub: 0, line: 0 } |  |  | 0.426 |
| walker |  | 6402 | 9 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 6, sub: 0, line: 99 } |  |  | 0.426 |
| walker |  | 6413 | 11 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 3, sub: 0, line: 85 } |  |  | 0.426 |
| walker |  | 6424 | 11 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 4, sub: 0, line: 90 } |  |  | 0.426 |
| walker |  | 6435 | 11 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 5, sub: 0, line: 94 } |  |  | 0.426 |
| walker |  | 6446 | 11 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 8, sub: 0, line: 109 } |  |  | 0.426 |
| walker |  | 6457 | 11 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 9, sub: 0, line: 114 } |  |  | 0.426 |
| walker |  | 6469 | 12 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 4, sub: 0, line: 90 } |  |  | 0.426 |
| walker |  | 6481 | 12 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 7, sub: 0, line: 104 } |  |  | 0.426 |
| walker |  | 6494 | 13 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 3, sub: 0, line: 85 } |  |  | 0.426 |
| ns | 6499 |  | 278 | step interface and stage enum | 4.7 | 4.2 | 0.413 |
| walker |  | 6507 | 13 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 6, sub: 0, line: 99 } |  |  | 0.413 |
| walker |  | 6592 | 85 | Code::CodeKey { rung: Names, file: pkg/container/docker_socket.go, decl: 0, sub: 0, line: 0 } |  |  | 0.413 |
| walker |  | 6612 | 20 | Code::CodeKey { rung: Decl, file: pkg/container/docker_socket.go, decl: 4, sub: 0, line: 57 } |  |  | 0.413 |
| walker |  | 6713 | 101 | Code::CodeKey { rung: Decl, file: pkg/container/docker_socket.go, decl: 1, sub: 0, line: 12 } |  |  | 0.414 |
| walker |  | 6726 | 13 | Code::CodeKey { rung: Doc, file: pkg/container/docker_socket.go, decl: 2, sub: 0, line: 23 } |  |  | 0.414 |
| walker |  | 6811 | 85 | Code::CodeKey { rung: Names, file: pkg/workflowpattern/trace_writer.go, decl: 0, sub: 0, line: 0 } |  |  | 0.414 |
| walker |  | 6814 | 3 | Code::CodeKey { rung: Decl, file: pkg/workflowpattern/trace_writer.go, decl: 3, sub: 0, line: 11 } |  |  | 0.414 |
| walker |  | 6828 | 14 | Code::CodeKey { rung: Decl, file: pkg/workflowpattern/trace_writer.go, decl: 1, sub: 0, line: 5 } |  |  | 0.414 |
| walker |  | 6840 | 12 | Code::CodeKey { rung: Body, file: pkg/workflowpattern/trace_writer.go, decl: 5, sub: 0, line: 16 } |  |  | 0.414 |
| walker |  | 6927 | 87 | Code::CodeKey { rung: Names, file: pkg/runner/local_repository_cache.go, decl: 0, sub: 0, line: 0 } |  |  | 0.414 |
| walker |  | 6963 | 36 | Code::CodeKey { rung: Decl, file: pkg/runner/local_repository_cache.go, decl: 1, sub: 0, line: 19 } |  |  | 0.414 |
| ns | 6994 |  | 495 | run_context.go: method roster (container lifecycle) | 4.8 |  | 0.402 |
| walker |  | 7076 | 113 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.402 |
| walker |  | 7090 | 14 | Code::CodeKey { rung: Body, file: pkg/container/util_openbsd_mips64.go, decl: 2, sub: 0, line: 15 } |  |  | 0.402 |
| walker |  | 7104 | 14 | Code::CodeKey { rung: Body, file: pkg/container/util_plan9.go, decl: 2, sub: 0, line: 15 } |  |  | 0.402 |
| walker |  | 7118 | 14 | Code::CodeKey { rung: Body, file: pkg/container/util_windows.go, decl: 2, sub: 0, line: 13 } |  |  | 0.402 |
| ns | 7140 |  | 146 | ActionCache implementations | 4.9 | 4.2 | 0.397 |
| walker |  | 7209 | 91 | Code::CodeKey { rung: Names, file: pkg/container/docker_logger.go, decl: 0, sub: 0, line: 0 } |  |  | 0.397 |
| walker |  | 7295 | 86 | Code::CodeKey { rung: Decl, file: pkg/container/docker_logger.go, decl: 1, sub: 0, line: 14 } |  |  | 0.397 |
| ns | 7300 |  | 160 | pkg/container: complete file list | 5.1 |  | 0.428 |
| ns | 7577 |  | 277 | Container interface | 5.2 |  | 0.422 |
| ns | 7719 |  | 142 | ExecutionsEnvironment interface | 5.3 |  | 0.428 |
| walker |  | 7876 | 581 | Code::CodeKey { rung: Decl, file: cmd/input.go, decl: 1, sub: 0, line: 10 } |  |  | 0.430 |
| walker |  | 7970 | 94 | Code::CodeKey { rung: Names, file: pkg/runner/action_cache_offline_mode.go, decl: 0, sub: 0, line: 0 } |  |  | 0.431 |
| walker |  | 7984 | 14 | Code::CodeKey { rung: Decl, file: pkg/runner/action_cache_offline_mode.go, decl: 1, sub: 0, line: 13 } |  |  | 0.426 |
| ns | 7984 |  | 265 | Docker backend vs docker-less stub: the build-tag split | 5.4 |  | 0.426 |
| walker |  | 8004 | 20 | Code::CodeKey { rung: Body, file: pkg/runner/action_cache_offline_mode.go, decl: 3, sub: 0, line: 45 } |  |  | 0.426 |
| walker |  | 8011 | 7 | Plaintext::DeclSurface { file: pkg/exprparser/testdata/for-hashing-3/nested/nested-data.txt } |  |  | 0.426 |
| walker |  | 8106 | 95 | Code::CodeKey { rung: Names, file: pkg/common/line_writer.go, decl: 0, sub: 0, line: 0 } |  |  | 0.426 |
| walker |  | 8130 | 24 | Code::CodeKey { rung: Decl, file: pkg/common/line_writer.go, decl: 2, sub: 0, line: 11 } |  |  | 0.426 |
| walker |  | 8145 | 15 | Code::CodeKey { rung: Doc, file: pkg/common/line_writer.go, decl: 3, sub: 0, line: 17 } |  |  | 0.426 |
| walker |  | 8161 | 16 | Code::CodeKey { rung: Doc, file: pkg/common/line_writer.go, decl: 1, sub: 0, line: 9 } |  |  | 0.437 |
| ns | 8161 |  | 177 | Docker socket discovery | 5.5 |  | 0.437 |
| walker |  | 8187 | 26 | Code::CodeKey { rung: Body, file: pkg/common/line_writer.go, decl: 3, sub: 0, line: 17 } |  |  | 0.437 |
| walker |  | 8216 | 29 | Code::CodeKey { rung: Body, file: cmd/notices.go, decl: 4, sub: 0, line: 57 } |  |  | 0.437 |
| ns | 8251 |  | 90 | Supporting packages: complete file lists (A) | 6.1 |  | 0.453 |
| walker |  | 8319 | 103 | Code::CodeKey { rung: Names, file: pkg/lookpath/lp_windows.go, decl: 0, sub: 0, line: 0 } |  |  | 0.453 |
| walker |  | 8342 | 23 | Code::CodeKey { rung: Doc, file: pkg/lookpath/lp_windows.go, decl: 1, sub: 0, line: 16 } |  |  | 0.453 |
| ns | 8410 |  | 159 | Supporting packages: complete file lists (B) | 6.2 |  | 0.476 |
| walker |  | 8446 | 104 | Code::CodeKey { rung: Names, file: pkg/container/container_types.go, decl: 0, sub: 0, line: 0 } |  |  | 0.476 |
| walker |  | 8453 | 7 | Code::CodeKey { rung: Decl, file: pkg/container/container_types.go, decl: 7, sub: 0, line: 80 } |  |  | 0.476 |
| walker |  | 8482 | 29 | Code::CodeKey { rung: Decl, file: pkg/container/container_types.go, decl: 2, sub: 0, line: 36 } |  |  | 0.476 |
| walker |  | 8529 | 47 | Code::CodeKey { rung: Decl, file: pkg/container/container_types.go, decl: 5, sub: 0, line: 70 } |  |  | 0.476 |
| ns | 8554 |  | 144 | Context names the expression interpreter resolves | 6.3 |  | 0.472 |
| walker |  | 8581 | 52 | Code::CodeKey { rung: Decl, file: pkg/container/container_types.go, decl: 4, sub: 0, line: 61 } |  |  | 0.472 |
| walker |  | 8597 | 16 | Code::CodeKey { rung: Doc, file: pkg/container/container_types.go, decl: 2, sub: 0, line: 36 } |  |  | 0.472 |
| ns | 8679 |  | 125 | Expression functions act implements | 6.4 |  | 0.468 |
| walker |  | 8803 | 206 | Code::CodeKey { rung: Decl, file: pkg/container/container_types.go, decl: 1, sub: 0, line: 12 } |  |  | 0.468 |
| ns | 8903 |  | 224 | Artifact cache server: routes and lifecycle | 6.5 |  | 0.463 |
| walker |  | 9051 | 248 | Code::CodeKey { rung: Decl, file: pkg/container/container_types.go, decl: 3, sub: 0, line: 43 } |  |  | 0.479 |
| walker |  | 9061 | 10 | Code::CodeKey { rung: Doc, file: pkg/container/container_types.go, decl: 3, sub: 0, line: 43 } |  |  | 0.480 |
| ns | 9068 |  | 165 | Artifact server: Serve + the v4 route base | 6.6 |  | 0.477 |
| ns | 9373 |  | 305 | pkg/common: complete exported-function roster | 6.7 |  | 0.474 |
| walker |  | 9435 | 374 | Code::CodeKey { rung: Names, file: cmd/root.go, decl: 0, sub: 0, line: 0 } |  |  | 0.496 |
| walker |  | 9485 | 50 | Code::CodeKey { rung: Decl, file: cmd/root.go, decl: 1, sub: 0, line: 37 } |  |  | 0.496 |
| walker |  | 9498 | 13 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 3, sub: 0, line: 47 } |  |  | 0.497 |
| walker |  | 9510 | 12 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 18, sub: 0, line: 391 } |  |  | 0.497 |
| walker |  | 9526 | 16 | Code::CodeKey { rung: Body, file: cmd/root.go, decl: 15, sub: 0, line: 345 } |  |  | 0.497 |
| ns | 9536 |  | 163 | pkg/common/git: repo detection and cloning | 6.8 |  | 0.493 |
| walker |  | 9587 | 61 | Code::CodeKey { rung: Body, file: cmd/root.go, decl: 3, sub: 0, line: 47 } |  |  | 0.502 |
| ns | 9666 |  | 130 | CI, issue-template and editor directories | 7.1 |  | 0.507 |
| walker |  | 9869 | 282 | Code::CodeKey { rung: Names, file: pkg/schema/schema.go, decl: 0, sub: 0, line: 0 } |  |  | 0.507 |
| walker |  | 9872 | 3 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 12, sub: 0, line: 92 } |  |  | 0.507 |
| walker |  | 9875 | 3 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 13, sub: 0, line: 95 } |  |  | 0.507 |
| walker |  | 9878 | 3 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 14, sub: 0, line: 98 } |  |  | 0.507 |
| walker |  | 9892 | 14 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 4, sub: 0, line: 25 } |  |  | 0.507 |
| walker |  | 9908 | 16 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 10, sub: 0, line: 83 } |  |  | 0.507 |
| ns | 9919 |  | 253 | Key direct dependencies | 7.2 |  | 0.515 |
| walker |  | 9927 | 19 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 8, sub: 0, line: 70 } |  |  | 0.515 |
| walker |  | 9952 | 25 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 11, sub: 0, line: 87 } |  |  | 0.515 |
| walker |  | 9978 | 26 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 18, sub: 0, line: 119 } |  |  | 0.515 |

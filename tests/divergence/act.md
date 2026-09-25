Score(3000)=0.578 I=0.857 C=0.390 ns_rows≤3K=18/47 grid(1000/1442/2080/3000/4327/6240/9000)=0.479/0.411/0.594/0.578/0.506/0.426/0.494

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 49 |  | 49 | README tagline + what act is | 1.1 |  | 0.000 |
| walker |  | 127 | 127 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 135 |  | 86 | main.go: process entry point | 1.2 |  | 0.000 |
| walker |  | 152 | 25 | Code::CodeKey { rung: Names, file: main.go, decl: 0, sub: 0, line: 0 } |  |  | 0.106 |
| ns | 176 |  | 41 | pkg/: complete package list | 1.3 |  | 0.068 |
| walker |  | 193 | 41 | Fs::DirListing { dir: pkg } |  |  | 0.445 |
| walker |  | 209 | 16 | Fs::DirListing { dir: pkg/workflowpattern } |  |  | 0.445 |
| ns | 222 |  | 46 | cmd/: complete file list | 1.4 |  | 0.352 |
| walker |  | 231 | 22 | Fs::DirListing { dir: pkg/exprparser } |  |  | 0.352 |
| walker |  | 240 | 9 | Fs::DirListing { dir: pkg/gh } |  |  | 0.352 |
| walker |  | 264 | 24 | Fs::DirListing { dir: pkg/artifacts } |  |  | 0.353 |
| walker |  | 289 | 25 | Fs::DirListing { dir: pkg/artifactcache } |  |  | 0.354 |
| walker |  | 292 | 3 | Fs::DirListing { dir: pkg/artifactcache/testdata } |  |  | 0.354 |
| walker |  | 305 | 13 | Fs::DirListing { dir: pkg/filecollector } |  |  | 0.354 |
| walker |  | 309 | 4 | Fs::DirListing { dir: pkg/artifactcache/testdata/example } |  |  | 0.354 |
| walker |  | 342 | 33 | Fs::DirListing { dir: pkg/lookpath } |  |  | 0.355 |
| ns | 349 |  | 127 | Repository root: complete entry list | 1.5 |  | 0.449 |
| walker |  | 374 | 32 | GoMod::Identity { file: go.mod } |  |  | 0.453 |
| walker |  | 393 | 19 | Fs::DirListing { dir: pkg/schema } |  |  | 0.454 |
| walker |  | 403 | 10 | Code::CodeKey { rung: Doc, file: main.go, decl: 1, sub: 0, line: 11 } |  |  | 0.481 |
| ns | 405 |  | 56 | pkg/model/: complete file list | 1.6 |  | 0.436 |
| walker |  | 449 | 46 | Fs::DirListing { dir: cmd } |  |  | 0.538 |
| walker |  | 468 | 19 | Fs::DirListing { dir: cmd/testdata } |  |  | 0.538 |
| walker |  | 476 | 8 | Fs::DirListing { dir: .vscode } |  |  | 0.538 |
| walker |  | 494 | 18 | Code::CodeKey { rung: Names, file: cmd/graph.go, decl: 0, sub: 0, line: 0 } |  |  | 0.538 |
| walker |  | 512 | 18 | Code::CodeKey { rung: Names, file: cmd/list.go, decl: 0, sub: 0, line: 0 } |  |  | 0.538 |
| walker |  | 568 | 56 | Fs::DirListing { dir: pkg/model } |  |  | 0.617 |
| walker |  | 578 | 10 | Plaintext::Whole { file: VERSION } |  |  | 0.618 |
| ns | 591 |  | 186 | pkg/runner/: complete file list | 1.7 |  | 0.497 |
| walker |  | 599 | 21 | Code::CodeKey { rung: Names, file: cmd/platforms.go, decl: 0, sub: 0, line: 0 } |  |  | 0.497 |
| walker |  | 617 | 18 | Fs::DirListing { dir: pkg/artifacts/testdata } |  |  | 0.497 |
| walker |  | 621 | 4 | Fs::DirListing { dir: pkg/artifacts/testdata/GHSL-2023-004 } |  |  | 0.497 |
| walker |  | 625 | 4 | Fs::DirListing { dir: pkg/artifacts/testdata/upload-and-download } |  |  | 0.497 |
| walker |  | 629 | 4 | Fs::DirListing { dir: pkg/artifacts/testdata/v4 } |  |  | 0.497 |
| walker |  | 642 | 13 | Code::CodeKey { rung: Names, file: pkg/model/job_context.go, decl: 0, sub: 0, line: 0 } |  |  | 0.497 |
| walker |  | 721 | 79 | Fs::DirListing { dir: pkg/common } |  |  | 0.500 |
| walker |  | 730 | 9 | Fs::DirListing { dir: pkg/common/git } |  |  | 0.501 |
| walker |  | 751 | 21 | Fs::DirListing { dir: pkg/model/testdata } |  |  | 0.501 |
| walker |  | 755 | 4 | Fs::DirListing { dir: pkg/model/testdata/container-volumes } |  |  | 0.501 |
| walker |  | 759 | 4 | Fs::DirListing { dir: pkg/model/testdata/strategy } |  |  | 0.501 |
| ns | 762 |  | 171 | README: how act works, end to end | 1.8 |  | 0.498 |
| walker |  | 765 | 6 | Fs::DirListing { dir: pkg/model/testdata/empty-workflow } |  |  | 0.498 |
| walker |  | 772 | 7 | Fs::DirListing { dir: pkg/model/testdata/nested } |  |  | 0.498 |
| walker |  | 778 | 6 | Fs::DirListing { dir: pkg/model/testdata/nested/workflows } |  |  | 0.498 |
| walker |  | 794 | 16 | Code::CodeKey { rung: Names, file: pkg/common/outbound_ip.go, decl: 0, sub: 0, line: 0 } |  |  | 0.498 |
| ns | 804 |  | 42 | Module path, Go version, released version | 1.9 |  | 0.502 |
| walker |  | 817 | 23 | Fs::DirListing { dir: pkg/exprparser/testdata } |  |  | 0.502 |
| walker |  | 824 | 7 | Fs::DirListing { dir: pkg/exprparser/testdata/for-hashing-3 } |  |  | 0.502 |
| walker |  | 829 | 5 | Fs::DirListing { dir: pkg/exprparser/testdata/for-hashing-3/nested } |  |  | 0.502 |
| walker |  | 863 | 34 | Code::CodeKey { rung: Names, file: cmd/dir.go, decl: 0, sub: 0, line: 0 } |  |  | 0.503 |
| walker |  | 872 | 9 | Code::CodeKey { rung: Decl, file: cmd/dir.go, decl: 1, sub: 0, line: 10 } |  |  | 0.503 |
| walker |  | 892 | 20 | Fs::DirListing { dir: .github } |  |  | 0.503 |
| walker |  | 918 | 26 | Fs::DirListing { dir: .github/workflows } |  |  | 0.504 |
| ns | 919 |  | 115 | cmd.Execute: CLI entry | 2.1 |  | 0.478 |
| walker |  | 943 | 25 | Code::CodeKey { rung: Names, file: pkg/gh/gh.go, decl: 0, sub: 0, line: 0 } |  |  | 0.478 |
| walker |  | 994 | 51 | Code::CodeKey { rung: Names, file: cmd/secrets.go, decl: 0, sub: 0, line: 0 } |  |  | 0.479 |
| ns | 1131 |  | 212 | Root cobra.Command definition | 2.2 |  | 0.456 |
| walker |  | 1154 | 160 | Fs::DirListing { dir: pkg/container } |  |  | 0.461 |
| walker |  | 1168 | 14 | Code::CodeKey { rung: Names, file: pkg/container/executions_environment.go, decl: 0, sub: 0, line: 0 } |  |  | 0.461 |
| walker |  | 1186 | 18 | Code::CodeKey { rung: ModuleDoc, file: pkg/container/docker_cli.go, decl: 0, sub: 0, line: 0 } |  |  | 0.461 |
| walker |  | 1223 | 37 | Fs::DirListing { dir: pkg/container/testdata } |  |  | 0.461 |
| walker |  | 1227 | 4 | Fs::DirListing { dir: pkg/container/testdata/docker-pull-options } |  |  | 0.461 |
| walker |  | 1231 | 4 | Fs::DirListing { dir: pkg/container/testdata/scratch } |  |  | 0.461 |
| walker |  | 1259 | 28 | Code::CodeKey { rung: Names, file: pkg/container/parse_env_file.go, decl: 0, sub: 0, line: 0 } |  |  | 0.461 |
| walker |  | 1287 | 28 | Code::CodeKey { rung: Names, file: pkg/lookpath/error.go, decl: 0, sub: 0, line: 0 } |  |  | 0.461 |
| walker |  | 1305 | 18 | Code::CodeKey { rung: Decl, file: pkg/lookpath/error.go, decl: 1, sub: 0, line: 3 } |  |  | 0.461 |
| ns | 1432 |  | 301 | Command-local flags: names and shorthands | 2.3 |  | 0.411 |
| walker |  | 1490 | 185 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.536 |
| walker |  | 1545 | 55 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.536 |
| walker |  | 1569 | 24 | Fs::DirListing { dir: pkg/model/testdata/invalid-job-name } |  |  | 0.536 |
| walker |  | 1755 | 186 | Fs::DirListing { dir: pkg/runner } |  |  | 0.681 |
| walker |  | 1759 | 4 | Fs::DirListing { dir: pkg/runner/hashfiles } |  |  | 0.681 |
| walker |  | 1763 | 4 | Fs::DirListing { dir: pkg/runner/res } |  |  | 0.681 |
| ns | 1782 |  | 350 | Persistent flags: names and shorthands | 2.4 |  | 0.613 |
| ns | 1928 |  | 146 | Input struct + path accessor roster | 2.5 |  | 0.594 |
| ns | 2206 |  | 278 | Config-file discovery: .actrc locations | 2.6 |  | 0.563 |
| walker |  | 2478 | 715 | Fs::DirListing { dir: pkg/runner/testdata } |  |  | 0.563 |
| ns | 2522 |  | 316 | cmd package: complete function roster | 2.7 |  | 0.533 |
| walker |  | 2543 | 65 | Markdown::HeadingsOutline { file: IMAGES.md } |  |  | 0.533 |
| walker |  | 2591 | 48 | Code::CodeKey { rung: Body, file: main.go, decl: 2, sub: 0, line: 13 } |  |  | 0.628 |
| walker |  | 2629 | 38 | Code::CodeKey { rung: Names, file: pkg/container/docker_network.go, decl: 0, sub: 0, line: 0 } |  |  | 0.628 |
| walker |  | 2657 | 28 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.628 |
| walker |  | 2697 | 40 | Code::CodeKey { rung: Names, file: pkg/artifactcache/model.go, decl: 0, sub: 0, line: 0 } |  |  | 0.628 |
| walker |  | 2739 | 42 | Code::CodeKey { rung: Names, file: pkg/common/file.go, decl: 0, sub: 0, line: 0 } |  |  | 0.628 |
| walker |  | 2781 | 42 | Code::CodeKey { rung: Names, file: pkg/container/docker_volume.go, decl: 0, sub: 0, line: 0 } |  |  | 0.628 |
| walker |  | 2824 | 43 | Code::CodeKey { rung: Names, file: pkg/common/cartesian.go, decl: 0, sub: 0, line: 0 } |  |  | 0.628 |
| walker |  | 2852 | 28 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.628 |
| ns | 2869 |  | 347 | pkg/model: complete type roster | 3.1 |  | 0.589 |
| walker |  | 2895 | 43 | Code::CodeKey { rung: Decl, file: pkg/artifactcache/model.go, decl: 1, sub: 0, line: 3 } |  |  | 0.589 |
| walker |  | 2942 | 47 | Code::CodeKey { rung: Names, file: pkg/model/anchors.go, decl: 0, sub: 0, line: 0 } |  |  | 0.589 |
| ns | 2979 |  | 110 | Workflow struct | 3.2 | 3.1 | 0.578 |
| walker |  | 2990 | 48 | Code::CodeKey { rung: Names, file: pkg/common/context.go, decl: 0, sub: 0, line: 0 } |  |  | 0.578 |
| walker |  | 3038 | 48 | Code::CodeKey { rung: Names, file: pkg/container/util.go, decl: 0, sub: 0, line: 0 } |  |  | 0.578 |
| walker |  | 3086 | 48 | Code::CodeKey { rung: Names, file: pkg/lookpath/lp_js.go, decl: 0, sub: 0, line: 0 } |  |  | 0.578 |
| walker |  | 3135 | 49 | Code::CodeKey { rung: Names, file: pkg/container/util_openbsd_mips64.go, decl: 0, sub: 0, line: 0 } |  |  | 0.578 |
| walker |  | 3184 | 49 | Code::CodeKey { rung: Names, file: pkg/container/util_plan9.go, decl: 0, sub: 0, line: 0 } |  |  | 0.578 |
| ns | 3209 |  | 230 | Planner interface + Plan/Stage/Run | 3.3 | 3.1 | 0.550 |
| walker |  | 3233 | 49 | Code::CodeKey { rung: Names, file: pkg/container/util_windows.go, decl: 0, sub: 0, line: 0 } |  |  | 0.550 |
| walker |  | 3283 | 50 | Code::CodeKey { rung: Names, file: pkg/container/docker_auth.go, decl: 0, sub: 0, line: 0 } |  |  | 0.550 |
| walker |  | 3337 | 54 | Code::CodeKey { rung: Names, file: pkg/container/docker_build.go, decl: 0, sub: 0, line: 0 } |  |  | 0.550 |
| walker |  | 3346 | 9 | Code::CodeKey { rung: Body, file: pkg/lookpath/error.go, decl: 2, sub: 0, line: 8 } |  |  | 0.550 |
| walker |  | 3405 | 59 | Code::CodeKey { rung: Names, file: pkg/lookpath/env.go, decl: 0, sub: 0, line: 0 } |  |  | 0.550 |
| walker |  | 3408 | 3 | Code::CodeKey { rung: Decl, file: pkg/lookpath/env.go, decl: 2, sub: 0, line: 9 } |  |  | 0.550 |
| walker |  | 3422 | 14 | Code::CodeKey { rung: Decl, file: pkg/lookpath/env.go, decl: 1, sub: 0, line: 5 } |  |  | 0.550 |
| ns | 3471 |  | 262 | Job struct: every supported job key | 3.4 | 3.1 | 0.533 |
| walker |  | 3482 | 60 | Code::CodeKey { rung: Names, file: pkg/container/docker_images.go, decl: 0, sub: 0, line: 0 } |  |  | 0.533 |
| walker |  | 3543 | 61 | Code::CodeKey { rung: Names, file: pkg/runner/step_factory.go, decl: 0, sub: 0, line: 0 } |  |  | 0.533 |
| walker |  | 3553 | 10 | Code::CodeKey { rung: Doc, file: pkg/common/file.go, decl: 1, sub: 0, line: 10 } |  |  | 0.533 |
| walker |  | 3618 | 65 | Code::CodeKey { rung: Names, file: pkg/lookpath/lp_plan9.go, decl: 0, sub: 0, line: 0 } |  |  | 0.533 |
| walker |  | 3683 | 65 | Code::CodeKey { rung: Names, file: pkg/lookpath/lp_unix.go, decl: 0, sub: 0, line: 0 } |  |  | 0.533 |
| ns | 3695 |  | 224 | Step struct: every supported step key | 3.5 | 3.1 | 0.519 |
| walker |  | 3755 | 72 | Code::CodeKey { rung: Names, file: pkg/container/docker_pull.go, decl: 0, sub: 0, line: 0 } |  |  | 0.519 |
| walker |  | 3804 | 49 | Markdown::Section { file: IMAGES.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.519 |
| walker |  | 3816 | 12 | Code::CodeKey { rung: Doc, file: pkg/common/file.go, decl: 2, sub: 0, line: 37 } |  |  | 0.519 |
| walker |  | 3891 | 75 | Code::CodeKey { rung: Names, file: pkg/common/dryrun.go, decl: 0, sub: 0, line: 0 } |  |  | 0.520 |
| ns | 4027 |  | 332 | StepType constants | 3.6 | 3.1 | 0.500 |
| walker |  | 4038 | 147 | Code::CodeKey { rung: Names, file: cmd/notices.go, decl: 0, sub: 0, line: 0 } |  |  | 0.511 |
| walker |  | 4065 | 27 | Code::CodeKey { rung: Decl, file: cmd/notices.go, decl: 1, sub: 0, line: 17 } |  |  | 0.511 |
| walker |  | 4142 | 77 | Code::CodeKey { rung: Names, file: pkg/common/logger.go, decl: 0, sub: 0, line: 0 } |  |  | 0.511 |
| walker |  | 4154 | 12 | Code::CodeKey { rung: Doc, file: pkg/common/logger.go, decl: 3, sub: 0, line: 14 } |  |  | 0.511 |
| ns | 4248 |  | 221 | GithubContext: complete field-name roster | 3.7 | 3.1 | 0.490 |
| walker |  | 4304 | 150 | Code::CodeKey { rung: Names, file: cmd/input.go, decl: 0, sub: 0, line: 0 } |  |  | 0.506 |
| walker |  | 4313 | 9 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 6, sub: 0, line: 99 } |  |  | 0.506 |
| walker |  | 4324 | 11 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 3, sub: 0, line: 85 } |  |  | 0.506 |
| walker |  | 4335 | 11 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 4, sub: 0, line: 90 } |  |  | 0.506 |
| walker |  | 4346 | 11 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 5, sub: 0, line: 94 } |  |  | 0.506 |
| walker |  | 4357 | 11 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 8, sub: 0, line: 109 } |  |  | 0.506 |
| walker |  | 4368 | 11 | Code::CodeKey { rung: Body, file: cmd/input.go, decl: 9, sub: 0, line: 114 } |  |  | 0.506 |
| walker |  | 4375 | 7 | Code::CodeKey { rung: Body, file: cmd/secrets.go, decl: 3, sub: 0, line: 40 } |  |  | 0.506 |
| walker |  | 4460 | 85 | Code::CodeKey { rung: Names, file: pkg/container/docker_socket.go, decl: 0, sub: 0, line: 0 } |  |  | 0.506 |
| walker |  | 4480 | 20 | Code::CodeKey { rung: Decl, file: pkg/container/docker_socket.go, decl: 4, sub: 0, line: 57 } |  |  | 0.506 |
| ns | 4531 |  | 283 | Executor type + complete combinator roster | 4.1 |  | 0.491 |
| walker |  | 4565 | 85 | Code::CodeKey { rung: Names, file: pkg/workflowpattern/trace_writer.go, decl: 0, sub: 0, line: 0 } |  |  | 0.491 |
| walker |  | 4568 | 3 | Code::CodeKey { rung: Decl, file: pkg/workflowpattern/trace_writer.go, decl: 3, sub: 0, line: 11 } |  |  | 0.491 |
| walker |  | 4582 | 14 | Code::CodeKey { rung: Decl, file: pkg/workflowpattern/trace_writer.go, decl: 1, sub: 0, line: 5 } |  |  | 0.491 |
| walker |  | 4594 | 12 | Code::CodeKey { rung: Body, file: pkg/workflowpattern/trace_writer.go, decl: 5, sub: 0, line: 16 } |  |  | 0.491 |
| walker |  | 4618 | 24 | Code::CodeKey { rung: Decl, file: pkg/runner/step_factory.go, decl: 1, sub: 0, line: 9 } |  |  | 0.491 |
| walker |  | 4705 | 87 | Code::CodeKey { rung: Names, file: pkg/runner/local_repository_cache.go, decl: 0, sub: 0, line: 0 } |  |  | 0.491 |
| walker |  | 4741 | 36 | Code::CodeKey { rung: Decl, file: pkg/runner/local_repository_cache.go, decl: 1, sub: 0, line: 19 } |  |  | 0.491 |
| walker |  | 4755 | 14 | Code::CodeKey { rung: Body, file: pkg/lookpath/env.go, decl: 4, sub: 0, line: 16 } |  |  | 0.491 |
| walker |  | 4846 | 91 | Code::CodeKey { rung: Names, file: pkg/container/docker_logger.go, decl: 0, sub: 0, line: 0 } |  |  | 0.491 |
| walker |  | 4861 | 15 | Code::CodeKey { rung: Doc, file: pkg/common/dryrun.go, decl: 3, sub: 0, line: 12 } |  |  | 0.491 |
| walker |  | 4955 | 94 | Code::CodeKey { rung: Names, file: pkg/runner/action_cache_offline_mode.go, decl: 0, sub: 0, line: 0 } |  |  | 0.491 |
| walker |  | 4969 | 14 | Code::CodeKey { rung: Decl, file: pkg/runner/action_cache_offline_mode.go, decl: 1, sub: 0, line: 13 } |  |  | 0.491 |
| walker |  | 5064 | 95 | Code::CodeKey { rung: Names, file: pkg/common/line_writer.go, decl: 0, sub: 0, line: 0 } |  |  | 0.491 |
| ns | 5066 |  | 535 | pkg/runner: complete type roster | 4.2 |  | 0.470 |
| walker |  | 5088 | 24 | Code::CodeKey { rung: Decl, file: pkg/common/line_writer.go, decl: 2, sub: 0, line: 11 } |  |  | 0.470 |
| walker |  | 5103 | 15 | Code::CodeKey { rung: Doc, file: pkg/common/line_writer.go, decl: 3, sub: 0, line: 17 } |  |  | 0.470 |
| walker |  | 5119 | 16 | Code::CodeKey { rung: Doc, file: pkg/common/line_writer.go, decl: 1, sub: 0, line: 9 } |  |  | 0.470 |
| ns | 5176 |  | 110 | Runner interface and New() | 4.3 | 4.2 | 0.463 |
| walker |  | 5213 | 94 | Code::CodeKey { rung: Decl, file: pkg/model/job_context.go, decl: 1, sub: 0, line: 3 } |  |  | 0.463 |
| walker |  | 5504 | 291 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.463 |
| ns | 5564 |  | 388 | runner.Config: fields (first half, rest elided) | 4.4 | 4.2 | 0.451 |
| walker |  | 5607 | 103 | Code::CodeKey { rung: Names, file: pkg/lookpath/lp_windows.go, decl: 0, sub: 0, line: 0 } |  |  | 0.451 |
| walker |  | 5711 | 104 | Code::CodeKey { rung: Names, file: pkg/container/container_types.go, decl: 0, sub: 0, line: 0 } |  |  | 0.451 |
| walker |  | 5718 | 7 | Code::CodeKey { rung: Decl, file: pkg/container/container_types.go, decl: 7, sub: 0, line: 80 } |  |  | 0.451 |
| walker |  | 5747 | 29 | Code::CodeKey { rung: Decl, file: pkg/container/container_types.go, decl: 2, sub: 0, line: 36 } |  |  | 0.451 |
| walker |  | 5794 | 47 | Code::CodeKey { rung: Decl, file: pkg/container/container_types.go, decl: 5, sub: 0, line: 70 } |  |  | 0.451 |
| walker |  | 5846 | 52 | Code::CodeKey { rung: Decl, file: pkg/container/container_types.go, decl: 4, sub: 0, line: 61 } |  |  | 0.451 |
| walker |  | 5863 | 17 | Code::CodeKey { rung: Doc, file: pkg/common/logger.go, decl: 4, sub: 0, line: 25 } |  |  | 0.451 |
| ns | 5866 |  | 302 | RunContext struct | 4.5 | 4.2 | 0.439 |
| walker |  | 5964 | 101 | Code::CodeKey { rung: Decl, file: pkg/container/docker_socket.go, decl: 1, sub: 0, line: 12 } |  |  | 0.439 |
| ns | 6221 |  | 355 | stepFactory: StepType -> implementation | 4.6 | 4.2 | 0.426 |
| walker |  | 6338 | 374 | Code::CodeKey { rung: Names, file: cmd/root.go, decl: 0, sub: 0, line: 0 } |  |  | 0.463 |
| walker |  | 6388 | 50 | Code::CodeKey { rung: Decl, file: cmd/root.go, decl: 1, sub: 0, line: 37 } |  |  | 0.463 |
| walker |  | 6401 | 13 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 3, sub: 0, line: 47 } |  |  | 0.465 |
| ns | 6499 |  | 278 | step interface and stage enum | 4.7 | 4.2 | 0.450 |
| walker |  | 6683 | 282 | Code::CodeKey { rung: Names, file: pkg/schema/schema.go, decl: 0, sub: 0, line: 0 } |  |  | 0.450 |
| walker |  | 6686 | 3 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 12, sub: 0, line: 92 } |  |  | 0.450 |
| walker |  | 6689 | 3 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 13, sub: 0, line: 95 } |  |  | 0.450 |
| walker |  | 6692 | 3 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 14, sub: 0, line: 98 } |  |  | 0.450 |
| walker |  | 6706 | 14 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 4, sub: 0, line: 25 } |  |  | 0.450 |
| walker |  | 6722 | 16 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 10, sub: 0, line: 83 } |  |  | 0.450 |
| walker |  | 6741 | 19 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 8, sub: 0, line: 70 } |  |  | 0.450 |
| walker |  | 6766 | 25 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 11, sub: 0, line: 87 } |  |  | 0.450 |
| walker |  | 6792 | 26 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 18, sub: 0, line: 119 } |  |  | 0.450 |
| walker |  | 6821 | 29 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 17, sub: 0, line: 113 } |  |  | 0.450 |
| walker |  | 6872 | 51 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 7, sub: 0, line: 64 } |  |  | 0.450 |
| walker |  | 6889 | 17 | Code::CodeKey { rung: Body, file: pkg/common/logger.go, decl: 4, sub: 0, line: 25 } |  |  | 0.450 |
| walker |  | 6901 | 12 | Code::CodeKey { rung: Doc, file: cmd/input.go, decl: 4, sub: 0, line: 90 } |  |  | 0.450 |
| walker |  | 6919 | 18 | Code::CodeKey { rung: Doc, file: pkg/common/cartesian.go, decl: 1, sub: 0, line: 4 } |  |  | 0.450 |
| walker |  | 6937 | 18 | Code::CodeKey { rung: Doc, file: pkg/common/dryrun.go, decl: 4, sub: 0, line: 23 } |  |  | 0.450 |
| ns | 6994 |  | 495 | run_context.go: method roster (container lifecycle) | 4.8 |  | 0.438 |
| walker |  | 7043 | 106 | Code::CodeKey { rung: Decl, file: pkg/schema/schema.go, decl: 6, sub: 0, line: 52 } |  |  | 0.438 |
| ns | 7140 |  | 146 | ActionCache implementations | 4.9 | 4.2 | 0.435 |
| walker |  | 7151 | 108 | Code::CodeKey { rung: Decl, file: pkg/container/executions_environment.go, decl: 1, sub: 0, line: 5 } |  |  | 0.436 |
| walker |  | 7265 | 114 | Code::CodeKey { rung: Names, file: pkg/common/auth.go, decl: 0, sub: 0, line: 0 } |  |  | 0.436 |
| walker |  | 7272 | 7 | Code::CodeKey { rung: Decl, file: pkg/common/auth.go, decl: 4, sub: 0, line: 33 } |  |  | 0.436 |
| walker |  | 7294 | 22 | Code::CodeKey { rung: Decl, file: pkg/common/auth.go, decl: 2, sub: 0, line: 26 } |  |  | 0.436 |
| ns | 7300 |  | 160 | pkg/container: complete file list | 5.1 |  | 0.462 |
| walker |  | 7313 | 19 | Code::CodeKey { rung: Doc, file: pkg/container/docker_build.go, decl: 1, sub: 0, line: 23 } |  |  | 0.462 |
| walker |  | 7332 | 19 | Code::CodeKey { rung: Doc, file: pkg/container/docker_pull.go, decl: 1, sub: 0, line: 21 } |  |  | 0.462 |
| walker |  | 7450 | 118 | Code::CodeKey { rung: Names, file: pkg/runner/job_executor.go, decl: 0, sub: 0, line: 0 } |  |  | 0.463 |
| walker |  | 7456 | 6 | Plaintext::DeclSurface { file: pkg/exprparser/testdata/for-hashing-1.txt } |  |  | 0.463 |
| walker |  | 7462 | 6 | Plaintext::DeclSurface { file: pkg/exprparser/testdata/for-hashing-2.txt } |  |  | 0.463 |
| ns | 7577 |  | 277 | Container interface | 5.2 |  | 0.456 |
| walker |  | 7679 | 217 | Code::CodeKey { rung: Names, file: pkg/container/host_environment.go, decl: 0, sub: 0, line: 0 } |  |  | 0.456 |
| ns | 7719 |  | 142 | ExecutionsEnvironment interface | 5.3 |  | 0.462 |
| walker |  | 7750 | 71 | Code::CodeKey { rung: Decl, file: pkg/container/host_environment.go, decl: 1, sub: 0, line: 28 } |  |  | 0.462 |
| walker |  | 7769 | 19 | Code::CodeKey { rung: Body, file: pkg/common/dryrun.go, decl: 4, sub: 0, line: 23 } |  |  | 0.462 |
| walker |  | 7891 | 122 | Code::CodeKey { rung: Names, file: pkg/workflowpattern/workflow_pattern.go, decl: 0, sub: 0, line: 0 } |  |  | 0.462 |
| walker |  | 7923 | 32 | Code::CodeKey { rung: Decl, file: pkg/workflowpattern/workflow_pattern.go, decl: 1, sub: 0, line: 9 } |  |  | 0.462 |
| walker |  | 7935 | 12 | Code::CodeKey { rung: Doc, file: pkg/workflowpattern/workflow_pattern.go, decl: 3, sub: 0, line: 38 } |  |  | 0.462 |
| walker |  | 7952 | 17 | Code::CodeKey { rung: Doc, file: pkg/workflowpattern/workflow_pattern.go, decl: 5, sub: 0, line: 151 } |  |  | 0.462 |
| walker |  | 7971 | 19 | Code::CodeKey { rung: Doc, file: pkg/workflowpattern/workflow_pattern.go, decl: 6, sub: 0, line: 177 } |  |  | 0.462 |
| ns | 7984 |  | 265 | Docker backend vs docker-less stub: the build-tag split | 5.4 |  | 0.454 |
| ns | 8161 |  | 177 | Docker socket discovery | 5.5 |  | 0.465 |
| walker |  | 8184 | 213 | Code::CodeKey { rung: Names, file: pkg/container/docker_cli.go, decl: 0, sub: 0, line: 0 } |  |  | 0.465 |
| ns | 8251 |  | 90 | Supporting packages: complete file lists (A) | 6.1 |  | 0.479 |
| walker |  | 8318 | 134 | Code::CodeKey { rung: ModuleDoc, file: pkg/artifactcache/doc.go, decl: 0, sub: 0, line: 0 } |  |  | 0.479 |
| ns | 8410 |  | 159 | Supporting packages: complete file lists (B) | 6.2 |  | 0.500 |
| walker |  | 8520 | 202 | Code::CodeKey { rung: Names, file: pkg/container/docker_run.go, decl: 0, sub: 0, line: 0 } |  |  | 0.501 |
| walker |  | 8534 | 14 | Code::CodeKey { rung: Doc, file: pkg/container/docker_run.go, decl: 1, sub: 0, line: 45 } |  |  | 0.501 |
| ns | 8554 |  | 144 | Context names the expression interpreter resolves | 6.3 |  | 0.496 |
| ns | 8679 |  | 125 | Expression functions act implements | 6.4 |  | 0.492 |
| walker |  | 8812 | 278 | Code::CodeKey { rung: Names, file: pkg/model/workflow.go, decl: 0, sub: 0, line: 0 } |  |  | 0.498 |
| walker |  | 8825 | 13 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 13, sub: 0, line: 171 } |  |  | 0.498 |
| walker |  | 8842 | 17 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 17, sub: 0, line: 225 } |  |  | 0.498 |
| walker |  | 8862 | 20 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 8, sub: 0, line: 113 } |  |  | 0.498 |
| walker |  | 8888 | 26 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 11, sub: 0, line: 161 } |  |  | 0.498 |
| ns | 8903 |  | 224 | Artifact cache server: routes and lifecycle | 6.5 |  | 0.494 |
| walker |  | 8920 | 32 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 18, sub: 0, line: 230 } |  |  | 0.494 |
| walker |  | 8959 | 39 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 12, sub: 0, line: 166 } |  |  | 0.494 |
| walker |  | 9014 | 55 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 10, sub: 0, line: 154 } |  |  | 0.494 |
| ns | 9068 |  | 165 | Artifact server: Serve + the v4 route base | 6.6 |  | 0.490 |
| walker |  | 9083 | 69 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 7, sub: 0, line: 105 } |  |  | 0.490 |
| walker |  | 9155 | 72 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 16, sub: 0, line: 216 } |  |  | 0.490 |
| walker |  | 9242 | 87 | Code::CodeKey { rung: Decl, file: pkg/model/workflow.go, decl: 1, sub: 0, line: 19 } |  |  | 0.496 |
| walker |  | 9262 | 20 | Code::CodeKey { rung: Body, file: pkg/runner/action_cache_offline_mode.go, decl: 3, sub: 0, line: 45 } |  |  | 0.496 |
| walker |  | 9274 | 12 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 18, sub: 0, line: 391 } |  |  | 0.496 |
| ns | 9373 |  | 305 | pkg/common: complete exported-function roster | 6.7 |  | 0.495 |
| walker |  | 9405 | 131 | Code::CodeKey { rung: Names, file: pkg/model/step_result.go, decl: 0, sub: 0, line: 0 } |  |  | 0.497 |
| walker |  | 9412 | 7 | Code::CodeKey { rung: Decl, file: pkg/model/step_result.go, decl: 2, sub: 0, line: 7 } |  |  | 0.497 |
| walker |  | 9461 | 49 | Code::CodeKey { rung: Decl, file: pkg/model/step_result.go, decl: 7, sub: 0, line: 41 } |  |  | 0.497 |
| walker |  | 9489 | 28 | Code::CodeKey { rung: Decl, file: pkg/model/step_result.go, decl: 3, sub: 0, line: 13 } |  |  | 0.497 |
| ns | 9536 |  | 163 | pkg/common/git: repo detection and cloning | 6.8 |  | 0.492 |
| ns | 9666 |  | 130 | CI, issue-template and editor directories | 7.1 |  | 0.496 |
| walker |  | 9764 | 275 | Code::CodeKey { rung: Names, file: pkg/runner/logger.go, decl: 0, sub: 0, line: 0 } |  |  | 0.497 |
| walker |  | 9781 | 17 | Code::CodeKey { rung: Decl, file: pkg/runner/logger.go, decl: 10, sub: 0, line: 60 } |  |  | 0.497 |
| walker |  | 9793 | 12 | Code::CodeKey { rung: Doc, file: pkg/runner/logger.go, decl: 8, sub: 0, line: 45 } |  |  | 0.497 |
| walker |  | 9815 | 22 | Code::CodeKey { rung: Decl, file: pkg/runner/logger.go, decl: 1, sub: 0, line: 18 } |  |  | 0.497 |
| walker |  | 9832 | 17 | Code::CodeKey { rung: Doc, file: pkg/runner/logger.go, decl: 9, sub: 0, line: 56 } |  |  | 0.497 |
| ns | 9919 |  | 253 | Key direct dependencies | 7.2 |  | 0.491 |

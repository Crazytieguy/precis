Score(3000)=0.578 I=0.857 C=0.390 ns_rows≤3K=18/47 grid(1000/1442/2080/3000/4327/6240/9000)=0.479/0.411/0.594/0.578/0.506/0.426/0.494

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 49 |  | 49 | README tagline + what act is | 1.1 |  | 0.000 |
| walker |  | 127 | 127 | listing of '.' |  |  | 0.000 |
| ns | 135 |  | 86 | main.go: process entry point | 1.2 |  | 0.000 |
| walker |  | 152 | 25 | go names main.go |  |  | 0.106 |
| ns | 176 |  | 41 | pkg/: complete package list | 1.3 |  | 0.068 |
| walker |  | 193 | 41 | listing of 'pkg' |  |  | 0.445 |
| walker |  | 209 | 16 | listing of 'pkg/workflowpattern' |  |  | 0.445 |
| ns | 222 |  | 46 | cmd/: complete file list | 1.4 |  | 0.352 |
| walker |  | 231 | 22 | listing of 'pkg/exprparser' |  |  | 0.352 |
| walker |  | 240 | 9 | listing of 'pkg/gh' |  |  | 0.352 |
| walker |  | 264 | 24 | listing of 'pkg/artifacts' |  |  | 0.353 |
| walker |  | 289 | 25 | listing of 'pkg/artifactcache' |  |  | 0.354 |
| walker |  | 292 | 3 | listing of 'pkg/artifactcache/testdata' |  |  | 0.354 |
| walker |  | 305 | 13 | listing of 'pkg/filecollector' |  |  | 0.354 |
| walker |  | 309 | 4 | listing of 'pkg/artifactcache/testdata/example' |  |  | 0.354 |
| walker |  | 342 | 33 | listing of 'pkg/lookpath' |  |  | 0.355 |
| ns | 349 |  | 127 | Repository root: complete entry list | 1.5 |  | 0.449 |
| walker |  | 374 | 32 | go module identity in go.mod |  |  | 0.453 |
| walker |  | 393 | 19 | listing of 'pkg/schema' |  |  | 0.454 |
| walker |  | 403 | 10 | go doc main.go:11 |  |  | 0.481 |
| ns | 405 |  | 56 | pkg/model/: complete file list | 1.6 |  | 0.436 |
| walker |  | 449 | 46 | listing of 'cmd' |  |  | 0.538 |
| walker |  | 468 | 19 | listing of 'cmd/testdata' |  |  | 0.538 |
| walker |  | 476 | 8 | listing of '.vscode' |  |  | 0.538 |
| walker |  | 494 | 18 | go names cmd/graph.go |  |  | 0.538 |
| walker |  | 512 | 18 | go names cmd/list.go |  |  | 0.538 |
| walker |  | 568 | 56 | listing of 'pkg/model' |  |  | 0.617 |
| walker |  | 578 | 10 | plaintext config VERSION |  |  | 0.618 |
| ns | 591 |  | 186 | pkg/runner/: complete file list | 1.7 |  | 0.497 |
| walker |  | 599 | 21 | go names cmd/platforms.go |  |  | 0.497 |
| walker |  | 617 | 18 | listing of 'pkg/artifacts/testdata' |  |  | 0.497 |
| walker |  | 621 | 4 | listing of 'pkg/artifacts/testdata/GHSL-2023-004' |  |  | 0.497 |
| walker |  | 625 | 4 | listing of 'pkg/artifacts/testdata/upload-and-download' |  |  | 0.497 |
| walker |  | 629 | 4 | listing of 'pkg/artifacts/testdata/v4' |  |  | 0.497 |
| walker |  | 642 | 13 | go names pkg/model/job_context.go |  |  | 0.497 |
| walker |  | 721 | 79 | listing of 'pkg/common' |  |  | 0.500 |
| walker |  | 730 | 9 | listing of 'pkg/common/git' |  |  | 0.501 |
| walker |  | 751 | 21 | listing of 'pkg/model/testdata' |  |  | 0.501 |
| walker |  | 755 | 4 | listing of 'pkg/model/testdata/container-volumes' |  |  | 0.501 |
| walker |  | 759 | 4 | listing of 'pkg/model/testdata/strategy' |  |  | 0.501 |
| ns | 762 |  | 171 | README: how act works, end to end | 1.8 |  | 0.498 |
| walker |  | 765 | 6 | listing of 'pkg/model/testdata/empty-workflow' |  |  | 0.498 |
| walker |  | 772 | 7 | listing of 'pkg/model/testdata/nested' |  |  | 0.498 |
| walker |  | 778 | 6 | listing of 'pkg/model/testdata/nested/workflows' |  |  | 0.498 |
| walker |  | 794 | 16 | go names pkg/common/outbound_ip.go |  |  | 0.498 |
| ns | 804 |  | 42 | Module path, Go version, released version | 1.9 |  | 0.502 |
| walker |  | 817 | 23 | listing of 'pkg/exprparser/testdata' |  |  | 0.502 |
| walker |  | 824 | 7 | listing of 'pkg/exprparser/testdata/for-hashing-3' |  |  | 0.502 |
| walker |  | 829 | 5 | listing of 'pkg/exprparser/testdata/for-hashing-3/nested' |  |  | 0.502 |
| walker |  | 863 | 34 | go names cmd/dir.go |  |  | 0.503 |
| walker |  | 872 | 9 | go decl cmd/dir.go:10 |  |  | 0.503 |
| walker |  | 892 | 20 | listing of '.github' |  |  | 0.503 |
| walker |  | 918 | 26 | listing of '.github/workflows' |  |  | 0.504 |
| ns | 919 |  | 115 | cmd.Execute: CLI entry | 2.1 |  | 0.478 |
| walker |  | 943 | 25 | go names pkg/gh/gh.go |  |  | 0.478 |
| walker |  | 994 | 51 | go names cmd/secrets.go |  |  | 0.479 |
| ns | 1131 |  | 212 | Root cobra.Command definition | 2.2 |  | 0.456 |
| walker |  | 1154 | 160 | listing of 'pkg/container' |  |  | 0.461 |
| walker |  | 1168 | 14 | go names pkg/container/executions_environment.go |  |  | 0.461 |
| walker |  | 1186 | 18 | go module doc pkg/container/docker_cli.go |  |  | 0.461 |
| walker |  | 1223 | 37 | listing of 'pkg/container/testdata' |  |  | 0.461 |
| walker |  | 1227 | 4 | listing of 'pkg/container/testdata/docker-pull-options' |  |  | 0.461 |
| walker |  | 1231 | 4 | listing of 'pkg/container/testdata/scratch' |  |  | 0.461 |
| walker |  | 1259 | 28 | go names pkg/container/parse_env_file.go |  |  | 0.461 |
| walker |  | 1287 | 28 | go names pkg/lookpath/error.go |  |  | 0.461 |
| walker |  | 1305 | 18 | go decl pkg/lookpath/error.go:3 |  |  | 0.461 |
| ns | 1432 |  | 301 | Command-local flags: names and shorthands | 2.3 |  | 0.411 |
| walker |  | 1490 | 185 | README headline in README.md |  |  | 0.536 |
| walker |  | 1545 | 55 | headings outline in README.md |  |  | 0.536 |
| walker |  | 1569 | 24 | listing of 'pkg/model/testdata/invalid-job-name' |  |  | 0.536 |
| walker |  | 1755 | 186 | listing of 'pkg/runner' |  |  | 0.681 |
| walker |  | 1759 | 4 | listing of 'pkg/runner/hashfiles' |  |  | 0.681 |
| walker |  | 1763 | 4 | listing of 'pkg/runner/res' |  |  | 0.681 |
| ns | 1782 |  | 350 | Persistent flags: names and shorthands | 2.4 |  | 0.613 |
| ns | 1928 |  | 146 | Input struct + path accessor roster | 2.5 |  | 0.594 |
| ns | 2206 |  | 278 | Config-file discovery: .actrc locations | 2.6 |  | 0.563 |
| walker |  | 2478 | 715 | listing of 'pkg/runner/testdata' |  |  | 0.563 |
| ns | 2522 |  | 316 | cmd package: complete function roster | 2.7 |  | 0.533 |
| walker |  | 2543 | 65 | headings outline in IMAGES.md |  |  | 0.533 |
| walker |  | 2591 | 48 | go body main.go:13 |  |  | 0.628 |
| walker |  | 2629 | 38 | go names pkg/container/docker_network.go |  |  | 0.628 |
| walker |  | 2657 | 28 | README.md section #2 |  |  | 0.628 |
| walker |  | 2697 | 40 | go names pkg/artifactcache/model.go |  |  | 0.628 |
| walker |  | 2739 | 42 | go names pkg/common/file.go |  |  | 0.628 |
| walker |  | 2781 | 42 | go names pkg/container/docker_volume.go |  |  | 0.628 |
| walker |  | 2824 | 43 | go names pkg/common/cartesian.go |  |  | 0.628 |
| walker |  | 2852 | 28 | README.md section #3 |  |  | 0.628 |
| ns | 2869 |  | 347 | pkg/model: complete type roster | 3.1 |  | 0.589 |
| walker |  | 2895 | 43 | go decl pkg/artifactcache/model.go:3 |  |  | 0.589 |
| walker |  | 2942 | 47 | go names pkg/model/anchors.go |  |  | 0.589 |
| ns | 2979 |  | 110 | Workflow struct | 3.2 | 3.1 | 0.578 |
| walker |  | 2990 | 48 | go names pkg/common/context.go |  |  | 0.578 |
| walker |  | 3038 | 48 | go names pkg/container/util.go |  |  | 0.578 |
| walker |  | 3086 | 48 | go names pkg/lookpath/lp_js.go |  |  | 0.578 |
| walker |  | 3135 | 49 | go names pkg/container/util_openbsd_mips64.go |  |  | 0.578 |
| walker |  | 3184 | 49 | go names pkg/container/util_plan9.go |  |  | 0.578 |
| ns | 3209 |  | 230 | Planner interface + Plan/Stage/Run | 3.3 | 3.1 | 0.550 |
| walker |  | 3233 | 49 | go names pkg/container/util_windows.go |  |  | 0.550 |
| walker |  | 3283 | 50 | go names pkg/container/docker_auth.go |  |  | 0.550 |
| walker |  | 3337 | 54 | go names pkg/container/docker_build.go |  |  | 0.550 |
| walker |  | 3346 | 9 | go body pkg/lookpath/error.go:8 |  |  | 0.550 |
| walker |  | 3405 | 59 | go names pkg/lookpath/env.go |  |  | 0.550 |
| walker |  | 3408 | 3 | go decl pkg/lookpath/env.go:9 |  |  | 0.550 |
| walker |  | 3422 | 14 | go decl pkg/lookpath/env.go:5 |  |  | 0.550 |
| ns | 3471 |  | 262 | Job struct: every supported job key | 3.4 | 3.1 | 0.533 |
| walker |  | 3482 | 60 | go names pkg/container/docker_images.go |  |  | 0.533 |
| walker |  | 3543 | 61 | go names pkg/runner/step_factory.go |  |  | 0.533 |
| walker |  | 3553 | 10 | go doc pkg/common/file.go:10 |  |  | 0.533 |
| walker |  | 3618 | 65 | go names pkg/lookpath/lp_plan9.go |  |  | 0.533 |
| walker |  | 3683 | 65 | go names pkg/lookpath/lp_unix.go |  |  | 0.533 |
| ns | 3695 |  | 224 | Step struct: every supported step key | 3.5 | 3.1 | 0.519 |
| walker |  | 3755 | 72 | go names pkg/container/docker_pull.go |  |  | 0.519 |
| walker |  | 3804 | 49 | IMAGES.md section #0 |  |  | 0.519 |
| walker |  | 3816 | 12 | go doc pkg/common/file.go:37 |  |  | 0.519 |
| walker |  | 3891 | 75 | go names pkg/common/dryrun.go |  |  | 0.520 |
| ns | 4027 |  | 332 | StepType constants | 3.6 | 3.1 | 0.500 |
| walker |  | 4038 | 147 | go names cmd/notices.go |  |  | 0.511 |
| walker |  | 4065 | 27 | go decl cmd/notices.go:17 |  |  | 0.511 |
| walker |  | 4142 | 77 | go names pkg/common/logger.go |  |  | 0.511 |
| walker |  | 4154 | 12 | go doc pkg/common/logger.go:14 |  |  | 0.511 |
| ns | 4248 |  | 221 | GithubContext: complete field-name roster | 3.7 | 3.1 | 0.490 |
| walker |  | 4304 | 150 | go names cmd/input.go |  |  | 0.506 |
| walker |  | 4313 | 9 | go body cmd/input.go:99 |  |  | 0.506 |
| walker |  | 4324 | 11 | go body cmd/input.go:85 |  |  | 0.506 |
| walker |  | 4335 | 11 | go body cmd/input.go:90 |  |  | 0.506 |
| walker |  | 4346 | 11 | go body cmd/input.go:94 |  |  | 0.506 |
| walker |  | 4357 | 11 | go body cmd/input.go:109 |  |  | 0.506 |
| walker |  | 4368 | 11 | go body cmd/input.go:114 |  |  | 0.506 |
| walker |  | 4375 | 7 | go body cmd/secrets.go:40 |  |  | 0.506 |
| walker |  | 4460 | 85 | go names pkg/container/docker_socket.go |  |  | 0.506 |
| walker |  | 4480 | 20 | go decl pkg/container/docker_socket.go:57 |  |  | 0.506 |
| ns | 4531 |  | 283 | Executor type + complete combinator roster | 4.1 |  | 0.491 |
| walker |  | 4565 | 85 | go names pkg/workflowpattern/trace_writer.go |  |  | 0.491 |
| walker |  | 4568 | 3 | go decl pkg/workflowpattern/trace_writer.go:11 |  |  | 0.491 |
| walker |  | 4582 | 14 | go decl pkg/workflowpattern/trace_writer.go:5 |  |  | 0.491 |
| walker |  | 4594 | 12 | go body pkg/workflowpattern/trace_writer.go:16 |  |  | 0.491 |
| walker |  | 4618 | 24 | go decl pkg/runner/step_factory.go:9 |  |  | 0.491 |
| walker |  | 4705 | 87 | go names pkg/runner/local_repository_cache.go |  |  | 0.491 |
| walker |  | 4741 | 36 | go decl pkg/runner/local_repository_cache.go:19 |  |  | 0.491 |
| walker |  | 4755 | 14 | go body pkg/lookpath/env.go:16 |  |  | 0.491 |
| walker |  | 4846 | 91 | go names pkg/container/docker_logger.go |  |  | 0.491 |
| walker |  | 4861 | 15 | go doc pkg/common/dryrun.go:12 |  |  | 0.491 |
| walker |  | 4955 | 94 | go names pkg/runner/action_cache_offline_mode.go |  |  | 0.491 |
| walker |  | 4969 | 14 | go decl pkg/runner/action_cache_offline_mode.go:13 |  |  | 0.491 |
| walker |  | 5064 | 95 | go names pkg/common/line_writer.go |  |  | 0.491 |
| ns | 5066 |  | 535 | pkg/runner: complete type roster | 4.2 |  | 0.470 |
| walker |  | 5088 | 24 | go decl pkg/common/line_writer.go:11 |  |  | 0.470 |
| walker |  | 5103 | 15 | go doc pkg/common/line_writer.go:17 |  |  | 0.470 |
| walker |  | 5119 | 16 | go doc pkg/common/line_writer.go:9 |  |  | 0.470 |
| ns | 5176 |  | 110 | Runner interface and New() | 4.3 | 4.2 | 0.463 |
| walker |  | 5213 | 94 | go decl pkg/model/job_context.go:3 |  |  | 0.463 |
| walker |  | 5504 | 291 | README.md section #0 |  |  | 0.463 |
| ns | 5564 |  | 388 | runner.Config: fields (first half, rest elided) | 4.4 | 4.2 | 0.451 |
| walker |  | 5607 | 103 | go names pkg/lookpath/lp_windows.go |  |  | 0.451 |
| walker |  | 5711 | 104 | go names pkg/container/container_types.go |  |  | 0.451 |
| walker |  | 5718 | 7 | go decl pkg/container/container_types.go:80 |  |  | 0.451 |
| walker |  | 5747 | 29 | go decl pkg/container/container_types.go:36 |  |  | 0.451 |
| walker |  | 5794 | 47 | go decl pkg/container/container_types.go:70 |  |  | 0.451 |
| walker |  | 5846 | 52 | go decl pkg/container/container_types.go:61 |  |  | 0.451 |
| walker |  | 5863 | 17 | go doc pkg/common/logger.go:25 |  |  | 0.451 |
| ns | 5866 |  | 302 | RunContext struct | 4.5 | 4.2 | 0.439 |
| walker |  | 5964 | 101 | go decl pkg/container/docker_socket.go:12 |  |  | 0.439 |
| ns | 6221 |  | 355 | stepFactory: StepType -> implementation | 4.6 | 4.2 | 0.426 |
| walker |  | 6338 | 374 | go names cmd/root.go |  |  | 0.463 |
| walker |  | 6388 | 50 | go decl cmd/root.go:37 |  |  | 0.463 |
| walker |  | 6401 | 13 | go doc cmd/root.go:47 |  |  | 0.465 |
| ns | 6499 |  | 278 | step interface and stage enum | 4.7 | 4.2 | 0.450 |
| walker |  | 6683 | 282 | go names pkg/schema/schema.go |  |  | 0.450 |
| walker |  | 6686 | 3 | go decl pkg/schema/schema.go:92 |  |  | 0.450 |
| walker |  | 6689 | 3 | go decl pkg/schema/schema.go:95 |  |  | 0.450 |
| walker |  | 6692 | 3 | go decl pkg/schema/schema.go:98 |  |  | 0.450 |
| walker |  | 6706 | 14 | go decl pkg/schema/schema.go:25 |  |  | 0.450 |
| walker |  | 6722 | 16 | go decl pkg/schema/schema.go:83 |  |  | 0.450 |
| walker |  | 6741 | 19 | go decl pkg/schema/schema.go:70 |  |  | 0.450 |
| walker |  | 6766 | 25 | go decl pkg/schema/schema.go:87 |  |  | 0.450 |
| walker |  | 6792 | 26 | go decl pkg/schema/schema.go:119 |  |  | 0.450 |
| walker |  | 6821 | 29 | go decl pkg/schema/schema.go:113 |  |  | 0.450 |
| walker |  | 6872 | 51 | go decl pkg/schema/schema.go:64 |  |  | 0.450 |
| walker |  | 6889 | 17 | go body pkg/common/logger.go:25 |  |  | 0.450 |
| walker |  | 6901 | 12 | go doc cmd/input.go:90 |  |  | 0.450 |
| walker |  | 6919 | 18 | go doc pkg/common/cartesian.go:4 |  |  | 0.450 |
| walker |  | 6937 | 18 | go doc pkg/common/dryrun.go:23 |  |  | 0.450 |
| ns | 6994 |  | 495 | run_context.go: method roster (container lifecycle) | 4.8 |  | 0.438 |
| walker |  | 7043 | 106 | go decl pkg/schema/schema.go:52 |  |  | 0.438 |
| ns | 7140 |  | 146 | ActionCache implementations | 4.9 | 4.2 | 0.435 |
| walker |  | 7151 | 108 | go decl pkg/container/executions_environment.go:5 |  |  | 0.436 |
| walker |  | 7265 | 114 | go names pkg/common/auth.go |  |  | 0.436 |
| walker |  | 7272 | 7 | go decl pkg/common/auth.go:33 |  |  | 0.436 |
| walker |  | 7294 | 22 | go decl pkg/common/auth.go:26 |  |  | 0.436 |
| ns | 7300 |  | 160 | pkg/container: complete file list | 5.1 |  | 0.462 |
| walker |  | 7313 | 19 | go doc pkg/container/docker_build.go:23 |  |  | 0.462 |
| walker |  | 7332 | 19 | go doc pkg/container/docker_pull.go:21 |  |  | 0.462 |
| walker |  | 7450 | 118 | go names pkg/runner/job_executor.go |  |  | 0.463 |
| walker |  | 7456 | 6 | declaration surface of pkg/exprparser/testdata/for-hashing-1.txt |  |  | 0.463 |
| walker |  | 7462 | 6 | declaration surface of pkg/exprparser/testdata/for-hashing-2.txt |  |  | 0.463 |
| ns | 7577 |  | 277 | Container interface | 5.2 |  | 0.456 |
| walker |  | 7679 | 217 | go names pkg/container/host_environment.go |  |  | 0.456 |
| ns | 7719 |  | 142 | ExecutionsEnvironment interface | 5.3 |  | 0.462 |
| walker |  | 7750 | 71 | go decl pkg/container/host_environment.go:28 |  |  | 0.462 |
| walker |  | 7769 | 19 | go body pkg/common/dryrun.go:23 |  |  | 0.462 |
| walker |  | 7891 | 122 | go names pkg/workflowpattern/workflow_pattern.go |  |  | 0.462 |
| walker |  | 7923 | 32 | go decl pkg/workflowpattern/workflow_pattern.go:9 |  |  | 0.462 |
| walker |  | 7935 | 12 | go doc pkg/workflowpattern/workflow_pattern.go:38 |  |  | 0.462 |
| walker |  | 7952 | 17 | go doc pkg/workflowpattern/workflow_pattern.go:151 |  |  | 0.462 |
| walker |  | 7971 | 19 | go doc pkg/workflowpattern/workflow_pattern.go:177 |  |  | 0.462 |
| ns | 7984 |  | 265 | Docker backend vs docker-less stub: the build-tag split | 5.4 |  | 0.454 |
| ns | 8161 |  | 177 | Docker socket discovery | 5.5 |  | 0.465 |
| walker |  | 8184 | 213 | go names pkg/container/docker_cli.go |  |  | 0.465 |
| ns | 8251 |  | 90 | Supporting packages: complete file lists (A) | 6.1 |  | 0.479 |
| walker |  | 8318 | 134 | go module doc pkg/artifactcache/doc.go |  |  | 0.479 |
| ns | 8410 |  | 159 | Supporting packages: complete file lists (B) | 6.2 |  | 0.500 |
| walker |  | 8520 | 202 | go names pkg/container/docker_run.go |  |  | 0.501 |
| walker |  | 8534 | 14 | go doc pkg/container/docker_run.go:45 |  |  | 0.501 |
| ns | 8554 |  | 144 | Context names the expression interpreter resolves | 6.3 |  | 0.496 |
| ns | 8679 |  | 125 | Expression functions act implements | 6.4 |  | 0.492 |
| walker |  | 8812 | 278 | go names pkg/model/workflow.go |  |  | 0.498 |
| walker |  | 8825 | 13 | go decl pkg/model/workflow.go:171 |  |  | 0.498 |
| walker |  | 8842 | 17 | go decl pkg/model/workflow.go:225 |  |  | 0.498 |
| walker |  | 8862 | 20 | go decl pkg/model/workflow.go:113 |  |  | 0.498 |
| walker |  | 8888 | 26 | go decl pkg/model/workflow.go:161 |  |  | 0.498 |
| ns | 8903 |  | 224 | Artifact cache server: routes and lifecycle | 6.5 |  | 0.494 |
| walker |  | 8920 | 32 | go decl pkg/model/workflow.go:230 |  |  | 0.494 |
| walker |  | 8959 | 39 | go decl pkg/model/workflow.go:166 |  |  | 0.494 |
| walker |  | 9014 | 55 | go decl pkg/model/workflow.go:154 |  |  | 0.494 |
| ns | 9068 |  | 165 | Artifact server: Serve + the v4 route base | 6.6 |  | 0.490 |
| walker |  | 9083 | 69 | go decl pkg/model/workflow.go:105 |  |  | 0.490 |
| walker |  | 9155 | 72 | go decl pkg/model/workflow.go:216 |  |  | 0.490 |
| walker |  | 9242 | 87 | go decl pkg/model/workflow.go:19 |  |  | 0.496 |
| walker |  | 9262 | 20 | go body pkg/runner/action_cache_offline_mode.go:45 |  |  | 0.496 |
| walker |  | 9274 | 12 | go doc cmd/root.go:391 |  |  | 0.496 |
| ns | 9373 |  | 305 | pkg/common: complete exported-function roster | 6.7 |  | 0.495 |
| walker |  | 9405 | 131 | go names pkg/model/step_result.go |  |  | 0.497 |
| walker |  | 9412 | 7 | go decl pkg/model/step_result.go:7 |  |  | 0.497 |
| walker |  | 9461 | 49 | go decl pkg/model/step_result.go:41 |  |  | 0.497 |
| walker |  | 9489 | 28 | go decl pkg/model/step_result.go:13 |  |  | 0.497 |
| ns | 9536 |  | 163 | pkg/common/git: repo detection and cloning | 6.8 |  | 0.492 |
| ns | 9666 |  | 130 | CI, issue-template and editor directories | 7.1 |  | 0.496 |
| walker |  | 9764 | 275 | go names pkg/runner/logger.go |  |  | 0.497 |
| walker |  | 9781 | 17 | go decl pkg/runner/logger.go:60 |  |  | 0.497 |
| walker |  | 9793 | 12 | go doc pkg/runner/logger.go:45 |  |  | 0.497 |
| walker |  | 9815 | 22 | go decl pkg/runner/logger.go:18 |  |  | 0.497 |
| walker |  | 9832 | 17 | go doc pkg/runner/logger.go:56 |  |  | 0.497 |
| ns | 9919 |  | 253 | Key direct dependencies | 7.2 |  | 0.491 |

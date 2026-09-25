Score(3000)=0.578 I=0.857 C=0.390 ns_rows≤3K=18/47 grid(1000/1442/2080/3000/4327/6240/9000)=0.625/0.536/0.594/0.578/0.506/0.463/0.494

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
| walker |  | 384 | 10 | plaintext config VERSION |  |  | 0.454 |
| walker |  | 403 | 19 | listing of 'pkg/schema' |  |  | 0.456 |
| ns | 405 |  | 56 | pkg/model/: complete file list | 1.6 |  | 0.413 |
| walker |  | 413 | 10 | go doc main.go:11 |  |  | 0.437 |
| walker |  | 459 | 46 | listing of 'cmd' |  |  | 0.540 |
| walker |  | 478 | 19 | listing of 'cmd/testdata' |  |  | 0.540 |
| walker |  | 486 | 8 | listing of '.vscode' |  |  | 0.540 |
| walker |  | 547 | 61 | README headline in README.md |  |  | 0.724 |
| walker |  | 565 | 18 | go names cmd/graph.go |  |  | 0.724 |
| walker |  | 583 | 18 | go names cmd/list.go |  |  | 0.724 |
| ns | 591 |  | 186 | pkg/runner/: complete file list | 1.7 |  | 0.582 |
| walker |  | 639 | 56 | listing of 'pkg/model' |  |  | 0.654 |
| walker |  | 660 | 21 | go names cmd/platforms.go |  |  | 0.654 |
| walker |  | 678 | 18 | listing of 'pkg/artifacts/testdata' |  |  | 0.654 |
| walker |  | 682 | 4 | listing of 'pkg/artifacts/testdata/GHSL-2023-004' |  |  | 0.654 |
| walker |  | 686 | 4 | listing of 'pkg/artifacts/testdata/upload-and-download' |  |  | 0.654 |
| walker |  | 690 | 4 | listing of 'pkg/artifacts/testdata/v4' |  |  | 0.654 |
| walker |  | 703 | 13 | go names pkg/model/job_context.go |  |  | 0.654 |
| ns | 762 |  | 171 | README: how act works, end to end | 1.8 |  | 0.651 |
| walker |  | 782 | 79 | listing of 'pkg/common' |  |  | 0.653 |
| walker |  | 791 | 9 | listing of 'pkg/common/git' |  |  | 0.654 |
| ns | 804 |  | 42 | Module path, Go version, released version | 1.9 |  | 0.659 |
| walker |  | 812 | 21 | listing of 'pkg/model/testdata' |  |  | 0.659 |
| walker |  | 816 | 4 | listing of 'pkg/model/testdata/container-volumes' |  |  | 0.659 |
| walker |  | 820 | 4 | listing of 'pkg/model/testdata/strategy' |  |  | 0.659 |
| walker |  | 826 | 6 | listing of 'pkg/model/testdata/empty-workflow' |  |  | 0.659 |
| walker |  | 833 | 7 | listing of 'pkg/model/testdata/nested' |  |  | 0.659 |
| walker |  | 839 | 6 | listing of 'pkg/model/testdata/nested/workflows' |  |  | 0.659 |
| walker |  | 855 | 16 | go names pkg/common/outbound_ip.go |  |  | 0.659 |
| walker |  | 878 | 23 | listing of 'pkg/exprparser/testdata' |  |  | 0.659 |
| walker |  | 885 | 7 | listing of 'pkg/exprparser/testdata/for-hashing-3' |  |  | 0.659 |
| walker |  | 890 | 5 | listing of 'pkg/exprparser/testdata/for-hashing-3/nested' |  |  | 0.659 |
| ns | 919 |  | 115 | cmd.Execute: CLI entry | 2.1 |  | 0.625 |
| walker |  | 924 | 34 | go names cmd/dir.go |  |  | 0.625 |
| walker |  | 933 | 9 | go decl cmd/dir.go:10 |  |  | 0.625 |
| walker |  | 988 | 55 | headings outline in README.md |  |  | 0.625 |
| walker |  | 1008 | 20 | listing of '.github' |  |  | 0.626 |
| walker |  | 1034 | 26 | listing of '.github/workflows' |  |  | 0.626 |
| walker |  | 1059 | 25 | go names pkg/gh/gh.go |  |  | 0.626 |
| walker |  | 1110 | 51 | go names cmd/secrets.go |  |  | 0.627 |
| ns | 1131 |  | 212 | Root cobra.Command definition | 2.2 |  | 0.597 |
| walker |  | 1270 | 160 | listing of 'pkg/container' |  |  | 0.601 |
| walker |  | 1284 | 14 | go names pkg/container/executions_environment.go |  |  | 0.601 |
| walker |  | 1302 | 18 | go module doc pkg/container/docker_cli.go |  |  | 0.601 |
| walker |  | 1339 | 37 | listing of 'pkg/container/testdata' |  |  | 0.601 |
| walker |  | 1343 | 4 | listing of 'pkg/container/testdata/docker-pull-options' |  |  | 0.601 |
| walker |  | 1347 | 4 | listing of 'pkg/container/testdata/scratch' |  |  | 0.601 |
| walker |  | 1375 | 28 | go names pkg/container/parse_env_file.go |  |  | 0.601 |
| walker |  | 1403 | 28 | go names pkg/lookpath/error.go |  |  | 0.601 |
| walker |  | 1421 | 18 | go decl pkg/lookpath/error.go:3 |  |  | 0.601 |
| ns | 1432 |  | 301 | Command-local flags: names and shorthands | 2.3 |  | 0.536 |
| walker |  | 1445 | 24 | listing of 'pkg/model/testdata/invalid-job-name' |  |  | 0.536 |
| walker |  | 1631 | 186 | listing of 'pkg/runner' |  |  | 0.681 |
| walker |  | 1635 | 4 | listing of 'pkg/runner/hashfiles' |  |  | 0.681 |
| walker |  | 1639 | 4 | listing of 'pkg/runner/res' |  |  | 0.681 |
| ns | 1782 |  | 350 | Persistent flags: names and shorthands | 2.4 |  | 0.613 |
| ns | 1928 |  | 146 | Input struct + path accessor roster | 2.5 |  | 0.594 |
| ns | 2206 |  | 278 | Config-file discovery: .actrc locations | 2.6 |  | 0.563 |
| walker |  | 2354 | 715 | listing of 'pkg/runner/testdata' |  |  | 0.563 |
| walker |  | 2419 | 65 | headings outline in IMAGES.md |  |  | 0.563 |
| walker |  | 2467 | 48 | go body main.go:13 |  |  | 0.664 |
| walker |  | 2505 | 38 | go names pkg/container/docker_network.go |  |  | 0.664 |
| ns | 2522 |  | 316 | cmd package: complete function roster | 2.7 |  | 0.628 |
| walker |  | 2533 | 28 | README.md section #2 |  |  | 0.628 |
| walker |  | 2573 | 40 | go names pkg/artifactcache/model.go |  |  | 0.628 |
| walker |  | 2615 | 42 | go names pkg/common/file.go |  |  | 0.628 |
| walker |  | 2657 | 42 | go names pkg/container/docker_volume.go |  |  | 0.628 |
| walker |  | 2682 | 25 | ts names pkg/runner/res/trampoline.js |  |  | 0.628 |
| walker |  | 2725 | 43 | go names pkg/common/cartesian.go |  |  | 0.628 |
| walker |  | 2753 | 28 | README.md section #3 |  |  | 0.628 |
| walker |  | 2796 | 43 | go decl pkg/artifactcache/model.go:3 |  |  | 0.628 |
| walker |  | 2843 | 47 | go names pkg/model/anchors.go |  |  | 0.628 |
| ns | 2869 |  | 347 | pkg/model: complete type roster | 3.1 |  | 0.589 |
| walker |  | 2891 | 48 | go names pkg/common/context.go |  |  | 0.589 |
| walker |  | 2939 | 48 | go names pkg/container/util.go |  |  | 0.589 |
| ns | 2979 |  | 110 | Workflow struct | 3.2 | 3.1 | 0.578 |
| walker |  | 2987 | 48 | go names pkg/lookpath/lp_js.go |  |  | 0.578 |
| walker |  | 3036 | 49 | go names pkg/container/util_openbsd_mips64.go |  |  | 0.578 |
| walker |  | 3085 | 49 | go names pkg/container/util_plan9.go |  |  | 0.578 |
| walker |  | 3134 | 49 | go names pkg/container/util_windows.go |  |  | 0.578 |
| walker |  | 3184 | 50 | go names pkg/container/docker_auth.go |  |  | 0.578 |
| ns | 3209 |  | 230 | Planner interface + Plan/Stage/Run | 3.3 | 3.1 | 0.550 |
| walker |  | 3238 | 54 | go names pkg/container/docker_build.go |  |  | 0.550 |
| walker |  | 3247 | 9 | go body pkg/lookpath/error.go:8 |  |  | 0.550 |
| walker |  | 3306 | 59 | go names pkg/lookpath/env.go |  |  | 0.550 |
| walker |  | 3309 | 3 | go decl pkg/lookpath/env.go:9 |  |  | 0.550 |
| walker |  | 3323 | 14 | go decl pkg/lookpath/env.go:5 |  |  | 0.550 |
| walker |  | 3383 | 60 | go names pkg/container/docker_images.go |  |  | 0.550 |
| walker |  | 3444 | 61 | go names pkg/runner/step_factory.go |  |  | 0.550 |
| walker |  | 3454 | 10 | go doc pkg/common/file.go:10 |  |  | 0.550 |
| ns | 3471 |  | 262 | Job struct: every supported job key | 3.4 | 3.1 | 0.533 |
| walker |  | 3519 | 65 | go names pkg/lookpath/lp_plan9.go |  |  | 0.533 |
| walker |  | 3584 | 65 | go names pkg/lookpath/lp_unix.go |  |  | 0.533 |
| walker |  | 3656 | 72 | go names pkg/container/docker_pull.go |  |  | 0.533 |
| ns | 3695 |  | 224 | Step struct: every supported step key | 3.5 | 3.1 | 0.519 |
| walker |  | 3705 | 49 | IMAGES.md section #0 |  |  | 0.519 |
| walker |  | 3717 | 12 | go doc pkg/common/file.go:37 |  |  | 0.519 |
| walker |  | 3792 | 75 | go names pkg/common/dryrun.go |  |  | 0.520 |
| walker |  | 3939 | 147 | go names cmd/notices.go |  |  | 0.531 |
| walker |  | 3966 | 27 | go decl cmd/notices.go:17 |  |  | 0.531 |
| ns | 4027 |  | 332 | StepType constants | 3.6 | 3.1 | 0.511 |
| walker |  | 4043 | 77 | go names pkg/common/logger.go |  |  | 0.511 |
| walker |  | 4055 | 12 | go doc pkg/common/logger.go:14 |  |  | 0.511 |
| walker |  | 4205 | 150 | go names cmd/input.go |  |  | 0.528 |
| walker |  | 4214 | 9 | go body cmd/input.go:99 |  |  | 0.528 |
| walker |  | 4225 | 11 | go body cmd/input.go:85 |  |  | 0.528 |
| walker |  | 4236 | 11 | go body cmd/input.go:90 |  |  | 0.528 |
| walker |  | 4247 | 11 | go body cmd/input.go:94 |  |  | 0.528 |
| ns | 4248 |  | 221 | GithubContext: complete field-name roster | 3.7 | 3.1 | 0.506 |
| walker |  | 4258 | 11 | go body cmd/input.go:109 |  |  | 0.506 |
| walker |  | 4269 | 11 | go body cmd/input.go:114 |  |  | 0.506 |
| walker |  | 4276 | 7 | go body cmd/secrets.go:40 |  |  | 0.506 |
| walker |  | 4361 | 85 | go names pkg/container/docker_socket.go |  |  | 0.506 |
| walker |  | 4381 | 20 | go decl pkg/container/docker_socket.go:57 |  |  | 0.506 |
| walker |  | 4466 | 85 | go names pkg/workflowpattern/trace_writer.go |  |  | 0.506 |
| walker |  | 4469 | 3 | go decl pkg/workflowpattern/trace_writer.go:11 |  |  | 0.506 |
| walker |  | 4483 | 14 | go decl pkg/workflowpattern/trace_writer.go:5 |  |  | 0.506 |
| walker |  | 4495 | 12 | go body pkg/workflowpattern/trace_writer.go:16 |  |  | 0.506 |
| walker |  | 4519 | 24 | go decl pkg/runner/step_factory.go:9 |  |  | 0.506 |
| ns | 4531 |  | 283 | Executor type + complete combinator roster | 4.1 |  | 0.491 |
| walker |  | 4606 | 87 | go names pkg/runner/local_repository_cache.go |  |  | 0.491 |
| walker |  | 4642 | 36 | go decl pkg/runner/local_repository_cache.go:19 |  |  | 0.491 |
| walker |  | 4656 | 14 | go body pkg/lookpath/env.go:16 |  |  | 0.491 |
| walker |  | 4747 | 91 | go names pkg/container/docker_logger.go |  |  | 0.491 |
| walker |  | 4762 | 15 | go doc pkg/common/dryrun.go:12 |  |  | 0.491 |
| walker |  | 4856 | 94 | go names pkg/runner/action_cache_offline_mode.go |  |  | 0.491 |
| walker |  | 4870 | 14 | go decl pkg/runner/action_cache_offline_mode.go:13 |  |  | 0.491 |
| walker |  | 4965 | 95 | go names pkg/common/line_writer.go |  |  | 0.491 |
| walker |  | 4989 | 24 | go decl pkg/common/line_writer.go:11 |  |  | 0.491 |
| walker |  | 5004 | 15 | go doc pkg/common/line_writer.go:17 |  |  | 0.491 |
| walker |  | 5020 | 16 | go doc pkg/common/line_writer.go:9 |  |  | 0.491 |
| ns | 5066 |  | 535 | pkg/runner: complete type roster | 4.2 |  | 0.470 |
| walker |  | 5114 | 94 | go decl pkg/model/job_context.go:3 |  |  | 0.470 |
| ns | 5176 |  | 110 | Runner interface and New() | 4.3 | 4.2 | 0.463 |
| walker |  | 5405 | 291 | README.md section #0 |  |  | 0.463 |
| walker |  | 5508 | 103 | go names pkg/lookpath/lp_windows.go |  |  | 0.463 |
| ns | 5564 |  | 388 | runner.Config: fields (first half, rest elided) | 4.4 | 4.2 | 0.451 |
| walker |  | 5612 | 104 | go names pkg/container/container_types.go |  |  | 0.451 |
| walker |  | 5619 | 7 | go decl pkg/container/container_types.go:80 |  |  | 0.451 |
| walker |  | 5648 | 29 | go decl pkg/container/container_types.go:36 |  |  | 0.451 |
| walker |  | 5695 | 47 | go decl pkg/container/container_types.go:70 |  |  | 0.451 |
| walker |  | 5747 | 52 | go decl pkg/container/container_types.go:61 |  |  | 0.451 |
| walker |  | 5764 | 17 | go doc pkg/common/logger.go:25 |  |  | 0.451 |
| walker |  | 5865 | 101 | go decl pkg/container/docker_socket.go:12 |  |  | 0.452 |
| ns | 5866 |  | 302 | RunContext struct | 4.5 | 4.2 | 0.439 |
| ns | 6221 |  | 355 | stepFactory: StepType -> implementation | 4.6 | 4.2 | 0.426 |
| walker |  | 6239 | 374 | go names cmd/root.go |  |  | 0.463 |
| walker |  | 6289 | 50 | go decl cmd/root.go:37 |  |  | 0.463 |
| walker |  | 6302 | 13 | go doc cmd/root.go:47 |  |  | 0.465 |
| ns | 6499 |  | 278 | step interface and stage enum | 4.7 | 4.2 | 0.450 |
| walker |  | 6584 | 282 | go names pkg/schema/schema.go |  |  | 0.450 |
| walker |  | 6587 | 3 | go decl pkg/schema/schema.go:92 |  |  | 0.450 |
| walker |  | 6590 | 3 | go decl pkg/schema/schema.go:95 |  |  | 0.450 |
| walker |  | 6593 | 3 | go decl pkg/schema/schema.go:98 |  |  | 0.450 |
| walker |  | 6607 | 14 | go decl pkg/schema/schema.go:25 |  |  | 0.450 |
| walker |  | 6623 | 16 | go decl pkg/schema/schema.go:83 |  |  | 0.450 |
| walker |  | 6642 | 19 | go decl pkg/schema/schema.go:70 |  |  | 0.450 |
| walker |  | 6667 | 25 | go decl pkg/schema/schema.go:87 |  |  | 0.450 |
| walker |  | 6693 | 26 | go decl pkg/schema/schema.go:119 |  |  | 0.450 |
| walker |  | 6722 | 29 | go decl pkg/schema/schema.go:113 |  |  | 0.450 |
| walker |  | 6773 | 51 | go decl pkg/schema/schema.go:64 |  |  | 0.450 |
| walker |  | 6790 | 17 | go body pkg/common/logger.go:25 |  |  | 0.450 |
| walker |  | 6802 | 12 | go doc cmd/input.go:90 |  |  | 0.450 |
| walker |  | 6820 | 18 | go doc pkg/common/cartesian.go:4 |  |  | 0.450 |
| walker |  | 6838 | 18 | go doc pkg/common/dryrun.go:23 |  |  | 0.450 |
| walker |  | 6944 | 106 | go decl pkg/schema/schema.go:52 |  |  | 0.450 |
| ns | 6994 |  | 495 | run_context.go: method roster (container lifecycle) | 4.8 |  | 0.438 |
| walker |  | 7052 | 108 | go decl pkg/container/executions_environment.go:5 |  |  | 0.438 |
| ns | 7140 |  | 146 | ActionCache implementations | 4.9 | 4.2 | 0.436 |
| walker |  | 7166 | 114 | go names pkg/common/auth.go |  |  | 0.436 |
| walker |  | 7173 | 7 | go decl pkg/common/auth.go:33 |  |  | 0.436 |
| walker |  | 7195 | 22 | go decl pkg/common/auth.go:26 |  |  | 0.436 |
| walker |  | 7214 | 19 | go doc pkg/container/docker_build.go:23 |  |  | 0.436 |
| walker |  | 7233 | 19 | go doc pkg/container/docker_pull.go:21 |  |  | 0.436 |
| ns | 7300 |  | 160 | pkg/container: complete file list | 5.1 |  | 0.462 |
| walker |  | 7351 | 118 | go names pkg/runner/job_executor.go |  |  | 0.463 |
| walker |  | 7568 | 217 | go names pkg/container/host_environment.go |  |  | 0.463 |
| ns | 7577 |  | 277 | Container interface | 5.2 |  | 0.456 |
| walker |  | 7639 | 71 | go decl pkg/container/host_environment.go:28 |  |  | 0.456 |
| walker |  | 7658 | 19 | go body pkg/common/dryrun.go:23 |  |  | 0.456 |
| ns | 7719 |  | 142 | ExecutionsEnvironment interface | 5.3 |  | 0.462 |
| walker |  | 7780 | 122 | go names pkg/workflowpattern/workflow_pattern.go |  |  | 0.462 |
| walker |  | 7812 | 32 | go decl pkg/workflowpattern/workflow_pattern.go:9 |  |  | 0.462 |
| walker |  | 7824 | 12 | go doc pkg/workflowpattern/workflow_pattern.go:38 |  |  | 0.462 |
| walker |  | 7841 | 17 | go doc pkg/workflowpattern/workflow_pattern.go:151 |  |  | 0.462 |
| walker |  | 7860 | 19 | go doc pkg/workflowpattern/workflow_pattern.go:177 |  |  | 0.462 |
| ns | 7984 |  | 265 | Docker backend vs docker-less stub: the build-tag split | 5.4 |  | 0.454 |
| walker |  | 8073 | 213 | go names pkg/container/docker_cli.go |  |  | 0.454 |
| ns | 8161 |  | 177 | Docker socket discovery | 5.5 |  | 0.465 |
| walker |  | 8207 | 134 | go module doc pkg/artifactcache/doc.go |  |  | 0.465 |
| ns | 8251 |  | 90 | Supporting packages: complete file lists (A) | 6.1 |  | 0.479 |
| walker |  | 8409 | 202 | go names pkg/container/docker_run.go |  |  | 0.479 |
| ns | 8410 |  | 159 | Supporting packages: complete file lists (B) | 6.2 |  | 0.501 |
| walker |  | 8423 | 14 | go doc pkg/container/docker_run.go:45 |  |  | 0.501 |
| ns | 8554 |  | 144 | Context names the expression interpreter resolves | 6.3 |  | 0.496 |
| ns | 8679 |  | 125 | Expression functions act implements | 6.4 |  | 0.492 |
| walker |  | 8701 | 278 | go names pkg/model/workflow.go |  |  | 0.498 |
| walker |  | 8714 | 13 | go decl pkg/model/workflow.go:171 |  |  | 0.498 |
| walker |  | 8731 | 17 | go decl pkg/model/workflow.go:225 |  |  | 0.498 |
| walker |  | 8751 | 20 | go decl pkg/model/workflow.go:113 |  |  | 0.498 |
| walker |  | 8777 | 26 | go decl pkg/model/workflow.go:161 |  |  | 0.498 |
| walker |  | 8809 | 32 | go decl pkg/model/workflow.go:230 |  |  | 0.498 |
| walker |  | 8848 | 39 | go decl pkg/model/workflow.go:166 |  |  | 0.498 |
| walker |  | 8903 | 55 | go decl pkg/model/workflow.go:154 |  |  | 0.494 |
| ns | 8903 |  | 224 | Artifact cache server: routes and lifecycle | 6.5 |  | 0.494 |
| walker |  | 8972 | 69 | go decl pkg/model/workflow.go:105 |  |  | 0.494 |
| walker |  | 9044 | 72 | go decl pkg/model/workflow.go:216 |  |  | 0.494 |
| ns | 9068 |  | 165 | Artifact server: Serve + the v4 route base | 6.6 |  | 0.490 |
| walker |  | 9131 | 87 | go decl pkg/model/workflow.go:19 |  |  | 0.496 |
| walker |  | 9151 | 20 | go body pkg/runner/action_cache_offline_mode.go:45 |  |  | 0.496 |
| walker |  | 9163 | 12 | go doc cmd/root.go:391 |  |  | 0.496 |
| walker |  | 9294 | 131 | go names pkg/model/step_result.go |  |  | 0.498 |
| walker |  | 9301 | 7 | go decl pkg/model/step_result.go:7 |  |  | 0.498 |
| walker |  | 9350 | 49 | go decl pkg/model/step_result.go:41 |  |  | 0.498 |
| ns | 9373 |  | 305 | pkg/common: complete exported-function roster | 6.7 |  | 0.497 |
| walker |  | 9378 | 28 | go decl pkg/model/step_result.go:13 |  |  | 0.497 |
| ns | 9536 |  | 163 | pkg/common/git: repo detection and cloning | 6.8 |  | 0.492 |
| walker |  | 9653 | 275 | go names pkg/runner/logger.go |  |  | 0.493 |
| ns | 9666 |  | 130 | CI, issue-template and editor directories | 7.1 |  | 0.497 |
| walker |  | 9670 | 17 | go decl pkg/runner/logger.go:60 |  |  | 0.497 |
| walker |  | 9682 | 12 | go doc pkg/runner/logger.go:45 |  |  | 0.497 |
| walker |  | 9704 | 22 | go decl pkg/runner/logger.go:18 |  |  | 0.497 |
| walker |  | 9721 | 17 | go doc pkg/runner/logger.go:56 |  |  | 0.497 |
| ns | 9919 |  | 253 | Key direct dependencies | 7.2 |  | 0.491 |
| walker |  | 9950 | 229 | go names pkg/artifactcache/handler.go |  |  | 0.493 |
| walker |  | 9959 | 9 | go decl pkg/artifactcache/handler.go:28 |  |  | 0.493 |
| walker |  | 9977 | 18 | go body pkg/artifactcache/handler.go:129 |  |  | 0.493 |

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
| walker |  | 2700 | 43 | go names pkg/common/cartesian.go |  |  | 0.628 |
| walker |  | 2728 | 28 | README.md section #3 |  |  | 0.628 |
| walker |  | 2771 | 43 | go decl pkg/artifactcache/model.go:3 |  |  | 0.628 |
| walker |  | 2818 | 47 | go names pkg/model/anchors.go |  |  | 0.628 |
| walker |  | 2866 | 48 | go names pkg/common/context.go |  |  | 0.629 |
| ns | 2869 |  | 347 | pkg/model: complete type roster | 3.1 |  | 0.589 |
| walker |  | 2914 | 48 | go names pkg/container/util.go |  |  | 0.589 |
| walker |  | 2962 | 48 | go names pkg/lookpath/lp_js.go |  |  | 0.589 |
| ns | 2979 |  | 110 | Workflow struct | 3.2 | 3.1 | 0.578 |
| walker |  | 3011 | 49 | go names pkg/container/util_openbsd_mips64.go |  |  | 0.578 |
| walker |  | 3060 | 49 | go names pkg/container/util_plan9.go |  |  | 0.578 |
| walker |  | 3109 | 49 | go names pkg/container/util_windows.go |  |  | 0.578 |
| walker |  | 3159 | 50 | go names pkg/container/docker_auth.go |  |  | 0.578 |
| ns | 3209 |  | 230 | Planner interface + Plan/Stage/Run | 3.3 | 3.1 | 0.550 |
| walker |  | 3213 | 54 | go names pkg/container/docker_build.go |  |  | 0.550 |
| walker |  | 3222 | 9 | go body pkg/lookpath/error.go:8 |  |  | 0.550 |
| walker |  | 3281 | 59 | go names pkg/lookpath/env.go |  |  | 0.550 |
| walker |  | 3284 | 3 | go decl pkg/lookpath/env.go:9 |  |  | 0.550 |
| walker |  | 3298 | 14 | go decl pkg/lookpath/env.go:5 |  |  | 0.550 |
| walker |  | 3358 | 60 | go names pkg/container/docker_images.go |  |  | 0.550 |
| walker |  | 3419 | 61 | go names pkg/runner/step_factory.go |  |  | 0.550 |
| walker |  | 3429 | 10 | go doc pkg/common/file.go:10 |  |  | 0.550 |
| ns | 3471 |  | 262 | Job struct: every supported job key | 3.4 | 3.1 | 0.533 |
| walker |  | 3494 | 65 | go names pkg/lookpath/lp_plan9.go |  |  | 0.533 |
| walker |  | 3559 | 65 | go names pkg/lookpath/lp_unix.go |  |  | 0.533 |
| walker |  | 3631 | 72 | go names pkg/container/docker_pull.go |  |  | 0.533 |
| walker |  | 3680 | 49 | IMAGES.md section #0 |  |  | 0.533 |
| walker |  | 3692 | 12 | go doc pkg/common/file.go:37 |  |  | 0.533 |
| ns | 3695 |  | 224 | Step struct: every supported step key | 3.5 | 3.1 | 0.519 |
| walker |  | 3767 | 75 | go names pkg/common/dryrun.go |  |  | 0.520 |
| walker |  | 3914 | 147 | go names cmd/notices.go |  |  | 0.531 |
| walker |  | 3941 | 27 | go decl cmd/notices.go:17 |  |  | 0.531 |
| walker |  | 4018 | 77 | go names pkg/common/logger.go |  |  | 0.531 |
| ns | 4027 |  | 332 | StepType constants | 3.6 | 3.1 | 0.511 |
| walker |  | 4030 | 12 | go doc pkg/common/logger.go:14 |  |  | 0.511 |
| walker |  | 4180 | 150 | go names cmd/input.go |  |  | 0.528 |
| walker |  | 4189 | 9 | go body cmd/input.go:99 |  |  | 0.528 |
| walker |  | 4200 | 11 | go body cmd/input.go:85 |  |  | 0.528 |
| walker |  | 4211 | 11 | go body cmd/input.go:90 |  |  | 0.528 |
| walker |  | 4222 | 11 | go body cmd/input.go:94 |  |  | 0.528 |
| walker |  | 4233 | 11 | go body cmd/input.go:109 |  |  | 0.528 |
| walker |  | 4244 | 11 | go body cmd/input.go:114 |  |  | 0.528 |
| ns | 4248 |  | 221 | GithubContext: complete field-name roster | 3.7 | 3.1 | 0.506 |
| walker |  | 4251 | 7 | go body cmd/secrets.go:40 |  |  | 0.506 |
| walker |  | 4336 | 85 | go names pkg/container/docker_socket.go |  |  | 0.506 |
| walker |  | 4356 | 20 | go decl pkg/container/docker_socket.go:57 |  |  | 0.506 |
| walker |  | 4441 | 85 | go names pkg/workflowpattern/trace_writer.go |  |  | 0.506 |
| walker |  | 4444 | 3 | go decl pkg/workflowpattern/trace_writer.go:11 |  |  | 0.506 |
| walker |  | 4458 | 14 | go decl pkg/workflowpattern/trace_writer.go:5 |  |  | 0.506 |
| walker |  | 4470 | 12 | go body pkg/workflowpattern/trace_writer.go:16 |  |  | 0.506 |
| walker |  | 4494 | 24 | go decl pkg/runner/step_factory.go:9 |  |  | 0.506 |
| ns | 4531 |  | 283 | Executor type + complete combinator roster | 4.1 |  | 0.491 |
| walker |  | 4581 | 87 | go names pkg/runner/local_repository_cache.go |  |  | 0.491 |
| walker |  | 4617 | 36 | go decl pkg/runner/local_repository_cache.go:19 |  |  | 0.491 |
| walker |  | 4631 | 14 | go body pkg/lookpath/env.go:16 |  |  | 0.491 |
| walker |  | 4722 | 91 | go names pkg/container/docker_logger.go |  |  | 0.491 |
| walker |  | 4737 | 15 | go doc pkg/common/dryrun.go:12 |  |  | 0.491 |
| walker |  | 4831 | 94 | go names pkg/runner/action_cache_offline_mode.go |  |  | 0.491 |
| walker |  | 4845 | 14 | go decl pkg/runner/action_cache_offline_mode.go:13 |  |  | 0.491 |
| walker |  | 4940 | 95 | go names pkg/common/line_writer.go |  |  | 0.491 |
| walker |  | 4964 | 24 | go decl pkg/common/line_writer.go:11 |  |  | 0.491 |
| walker |  | 4979 | 15 | go doc pkg/common/line_writer.go:17 |  |  | 0.491 |
| walker |  | 4995 | 16 | go doc pkg/common/line_writer.go:9 |  |  | 0.491 |
| ns | 5066 |  | 535 | pkg/runner: complete type roster | 4.2 |  | 0.470 |
| walker |  | 5089 | 94 | go decl pkg/model/job_context.go:3 |  |  | 0.470 |
| ns | 5176 |  | 110 | Runner interface and New() | 4.3 | 4.2 | 0.463 |
| walker |  | 5380 | 291 | README.md section #0 |  |  | 0.463 |
| walker |  | 5483 | 103 | go names pkg/lookpath/lp_windows.go |  |  | 0.463 |
| ns | 5564 |  | 388 | runner.Config: fields (first half, rest elided) | 4.4 | 4.2 | 0.451 |
| walker |  | 5587 | 104 | go names pkg/container/container_types.go |  |  | 0.451 |
| walker |  | 5594 | 7 | go decl pkg/container/container_types.go:80 |  |  | 0.451 |
| walker |  | 5623 | 29 | go decl pkg/container/container_types.go:36 |  |  | 0.451 |
| walker |  | 5670 | 47 | go decl pkg/container/container_types.go:70 |  |  | 0.451 |
| walker |  | 5722 | 52 | go decl pkg/container/container_types.go:61 |  |  | 0.451 |
| walker |  | 5739 | 17 | go doc pkg/common/logger.go:25 |  |  | 0.451 |
| walker |  | 5840 | 101 | go decl pkg/container/docker_socket.go:12 |  |  | 0.452 |
| ns | 5866 |  | 302 | RunContext struct | 4.5 | 4.2 | 0.439 |
| walker |  | 6214 | 374 | go names cmd/root.go |  |  | 0.477 |
| ns | 6221 |  | 355 | stepFactory: StepType -> implementation | 4.6 | 4.2 | 0.463 |
| walker |  | 6264 | 50 | go decl cmd/root.go:37 |  |  | 0.463 |
| walker |  | 6277 | 13 | go doc cmd/root.go:47 |  |  | 0.465 |
| ns | 6499 |  | 278 | step interface and stage enum | 4.7 | 4.2 | 0.450 |
| walker |  | 6559 | 282 | go names pkg/schema/schema.go |  |  | 0.450 |
| walker |  | 6562 | 3 | go decl pkg/schema/schema.go:92 |  |  | 0.450 |
| walker |  | 6565 | 3 | go decl pkg/schema/schema.go:95 |  |  | 0.450 |
| walker |  | 6568 | 3 | go decl pkg/schema/schema.go:98 |  |  | 0.450 |
| walker |  | 6582 | 14 | go decl pkg/schema/schema.go:25 |  |  | 0.450 |
| walker |  | 6598 | 16 | go decl pkg/schema/schema.go:83 |  |  | 0.450 |
| walker |  | 6617 | 19 | go decl pkg/schema/schema.go:70 |  |  | 0.450 |
| walker |  | 6642 | 25 | go decl pkg/schema/schema.go:87 |  |  | 0.450 |
| walker |  | 6668 | 26 | go decl pkg/schema/schema.go:119 |  |  | 0.450 |
| walker |  | 6697 | 29 | go decl pkg/schema/schema.go:113 |  |  | 0.450 |
| walker |  | 6748 | 51 | go decl pkg/schema/schema.go:64 |  |  | 0.450 |
| walker |  | 6765 | 17 | go body pkg/common/logger.go:25 |  |  | 0.450 |
| walker |  | 6777 | 12 | go doc cmd/input.go:90 |  |  | 0.450 |
| walker |  | 6795 | 18 | go doc pkg/common/cartesian.go:4 |  |  | 0.450 |
| walker |  | 6813 | 18 | go doc pkg/common/dryrun.go:23 |  |  | 0.450 |
| walker |  | 6919 | 106 | go decl pkg/schema/schema.go:52 |  |  | 0.450 |
| ns | 6994 |  | 495 | run_context.go: method roster (container lifecycle) | 4.8 |  | 0.438 |
| walker |  | 7027 | 108 | go decl pkg/container/executions_environment.go:5 |  |  | 0.438 |
| ns | 7140 |  | 146 | ActionCache implementations | 4.9 | 4.2 | 0.436 |
| walker |  | 7141 | 114 | go names pkg/common/auth.go |  |  | 0.436 |
| walker |  | 7148 | 7 | go decl pkg/common/auth.go:33 |  |  | 0.436 |
| walker |  | 7170 | 22 | go decl pkg/common/auth.go:26 |  |  | 0.436 |
| walker |  | 7189 | 19 | go doc pkg/container/docker_build.go:23 |  |  | 0.436 |
| walker |  | 7208 | 19 | go doc pkg/container/docker_pull.go:21 |  |  | 0.436 |
| ns | 7300 |  | 160 | pkg/container: complete file list | 5.1 |  | 0.462 |
| walker |  | 7326 | 118 | go names pkg/runner/job_executor.go |  |  | 0.463 |
| walker |  | 7543 | 217 | go names pkg/container/host_environment.go |  |  | 0.463 |
| ns | 7577 |  | 277 | Container interface | 5.2 |  | 0.456 |
| walker |  | 7614 | 71 | go decl pkg/container/host_environment.go:28 |  |  | 0.456 |
| walker |  | 7633 | 19 | go body pkg/common/dryrun.go:23 |  |  | 0.456 |
| ns | 7719 |  | 142 | ExecutionsEnvironment interface | 5.3 |  | 0.462 |
| walker |  | 7755 | 122 | go names pkg/workflowpattern/workflow_pattern.go |  |  | 0.462 |
| walker |  | 7787 | 32 | go decl pkg/workflowpattern/workflow_pattern.go:9 |  |  | 0.462 |
| walker |  | 7799 | 12 | go doc pkg/workflowpattern/workflow_pattern.go:38 |  |  | 0.462 |
| walker |  | 7816 | 17 | go doc pkg/workflowpattern/workflow_pattern.go:151 |  |  | 0.462 |
| walker |  | 7835 | 19 | go doc pkg/workflowpattern/workflow_pattern.go:177 |  |  | 0.462 |
| ns | 7984 |  | 265 | Docker backend vs docker-less stub: the build-tag split | 5.4 |  | 0.454 |
| walker |  | 8048 | 213 | go names pkg/container/docker_cli.go |  |  | 0.454 |
| ns | 8161 |  | 177 | Docker socket discovery | 5.5 |  | 0.465 |
| walker |  | 8182 | 134 | go module doc pkg/artifactcache/doc.go |  |  | 0.465 |
| ns | 8251 |  | 90 | Supporting packages: complete file lists (A) | 6.1 |  | 0.479 |
| walker |  | 8384 | 202 | go names pkg/container/docker_run.go |  |  | 0.479 |
| walker |  | 8398 | 14 | go doc pkg/container/docker_run.go:45 |  |  | 0.480 |
| ns | 8410 |  | 159 | Supporting packages: complete file lists (B) | 6.2 |  | 0.501 |
| ns | 8554 |  | 144 | Context names the expression interpreter resolves | 6.3 |  | 0.496 |
| walker |  | 8676 | 278 | go names pkg/model/workflow.go |  |  | 0.503 |
| ns | 8679 |  | 125 | Expression functions act implements | 6.4 |  | 0.498 |
| walker |  | 8689 | 13 | go decl pkg/model/workflow.go:171 |  |  | 0.498 |
| walker |  | 8706 | 17 | go decl pkg/model/workflow.go:225 |  |  | 0.498 |
| walker |  | 8726 | 20 | go decl pkg/model/workflow.go:113 |  |  | 0.498 |
| walker |  | 8752 | 26 | go decl pkg/model/workflow.go:161 |  |  | 0.498 |
| walker |  | 8784 | 32 | go decl pkg/model/workflow.go:230 |  |  | 0.498 |
| walker |  | 8823 | 39 | go decl pkg/model/workflow.go:166 |  |  | 0.498 |
| walker |  | 8878 | 55 | go decl pkg/model/workflow.go:154 |  |  | 0.498 |
| ns | 8903 |  | 224 | Artifact cache server: routes and lifecycle | 6.5 |  | 0.494 |
| walker |  | 8947 | 69 | go decl pkg/model/workflow.go:105 |  |  | 0.494 |
| walker |  | 9019 | 72 | go decl pkg/model/workflow.go:216 |  |  | 0.494 |
| ns | 9068 |  | 165 | Artifact server: Serve + the v4 route base | 6.6 |  | 0.490 |
| walker |  | 9106 | 87 | go decl pkg/model/workflow.go:19 |  |  | 0.496 |
| walker |  | 9126 | 20 | go body pkg/runner/action_cache_offline_mode.go:45 |  |  | 0.496 |
| walker |  | 9138 | 12 | go doc cmd/root.go:391 |  |  | 0.496 |
| walker |  | 9269 | 131 | go names pkg/model/step_result.go |  |  | 0.498 |
| walker |  | 9276 | 7 | go decl pkg/model/step_result.go:7 |  |  | 0.498 |
| walker |  | 9325 | 49 | go decl pkg/model/step_result.go:41 |  |  | 0.498 |
| walker |  | 9353 | 28 | go decl pkg/model/step_result.go:13 |  |  | 0.498 |
| ns | 9373 |  | 305 | pkg/common: complete exported-function roster | 6.7 |  | 0.497 |
| ns | 9536 |  | 163 | pkg/common/git: repo detection and cloning | 6.8 |  | 0.492 |
| walker |  | 9628 | 275 | go names pkg/runner/logger.go |  |  | 0.493 |
| walker |  | 9645 | 17 | go decl pkg/runner/logger.go:60 |  |  | 0.493 |
| walker |  | 9657 | 12 | go doc pkg/runner/logger.go:45 |  |  | 0.493 |
| ns | 9666 |  | 130 | CI, issue-template and editor directories | 7.1 |  | 0.497 |
| walker |  | 9679 | 22 | go decl pkg/runner/logger.go:18 |  |  | 0.497 |
| walker |  | 9696 | 17 | go doc pkg/runner/logger.go:56 |  |  | 0.497 |
| ns | 9919 |  | 253 | Key direct dependencies | 7.2 |  | 0.491 |
| walker |  | 9925 | 229 | go names pkg/artifactcache/handler.go |  |  | 0.493 |
| walker |  | 9934 | 9 | go decl pkg/artifactcache/handler.go:28 |  |  | 0.493 |
| walker |  | 9952 | 18 | go body pkg/artifactcache/handler.go:129 |  |  | 0.493 |

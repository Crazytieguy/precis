Score(3000)=0.494 I=0.767 C=0.318 ns_rows≤3K=20/41 (reached=5 partial=2 missing=13)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 45 |  | 45 | README tagline + one-line pitch | 1.1 |  | 0.000 |
| ns | 75 |  | 30 | Module name + Go version | 1.2 |  | 0.000 |
| ns | 116 |  | 41 | pkg/ subpackage listing | 1.3 |  | 0.000 |
| walker |  | 125 | 125 | listing of '.' |  |  | 0.000 |
| walker |  | 145 | 20 | go decl names surface in main.go |  |  | 0.000 |
| walker |  | 145 | 0 | go decl at main.go:11 |  |  | 0.000 |
| walker |  | 145 | 0 | go decl at main.go:13 |  |  | 0.000 |
| ns | 162 |  | 46 | cmd/ file listing | 1.4 |  | 0.000 |
| walker |  | 170 | 25 | go module identity in go.mod |  |  | 0.150 |
| walker |  | 180 | 10 | plaintext config VERSION |  |  | 0.150 |
| walker |  | 232 | 52 | README headline in README.md |  |  | 0.316 |
| walker |  | 242 | 10 | go decl doc at main.go:11 |  |  | 0.316 |
| ns | 287 |  | 125 | Repo root file listing | 1.5 |  | 0.597 |
| walker |  | 288 | 46 | listing of 'cmd' |  |  | 0.739 |
| walker |  | 304 | 16 | go decl names surface in cmd/graph.go |  |  | 0.739 |
| walker |  | 304 | 0 | go decl at cmd/graph.go:10 |  |  | 0.739 |
| walker |  | 320 | 16 | go decl names surface in cmd/list.go |  |  | 0.739 |
| walker |  | 320 | 0 | go decl at cmd/list.go:11 |  |  | 0.739 |
| walker |  | 339 | 19 | go decl names surface in cmd/platforms.go |  |  | 0.739 |
| walker |  | 339 | 0 | go decl at cmd/platforms.go:7 |  |  | 0.739 |
| walker |  | 394 | 55 | headings outline in README.md |  |  | 0.739 |
| walker |  | 435 | 41 | listing of 'pkg' |  |  | 0.964 |
| ns | 443 |  | 156 | main.go entry point (full) | 1.6 |  | 0.822 |
| walker |  | 468 | 33 | listing of 'pkg/lookpath' |  |  | 0.822 |
| walker |  | 490 | 22 | listing of 'pkg/exprparser' |  |  | 0.822 |
| walker |  | 514 | 24 | listing of 'pkg/artifacts' |  |  | 0.822 |
| walker |  | 539 | 25 | listing of 'pkg/artifactcache' |  |  | 0.822 |
| walker |  | 595 | 56 | listing of 'pkg/model' |  |  | 0.822 |
| walker |  | 606 | 11 | go decl names surface in pkg/model/job_context.go |  |  | 0.822 |
| ns | 635 |  | 192 | CLAUDE.md architecture: execution flow | 1.7 |  | 0.770 |
| walker |  | 671 | 65 | headings outline in IMAGES.md |  |  | 0.770 |
| walker |  | 687 | 16 | listing of 'pkg/workflowpattern' |  |  | 0.770 |
| ns | 758 |  | 123 | CLAUDE.md: Executor pattern + combinators | 1.8 |  | 0.731 |
| walker |  | 766 | 79 | listing of 'pkg/common' |  |  | 0.731 |
| walker |  | 780 | 14 | go decl names surface in pkg/common/outbound_ip.go |  |  | 0.731 |
| walker |  | 780 | 0 | go decl at pkg/common/outbound_ip.go:13 |  |  | 0.731 |
| walker |  | 812 | 32 | go decl names surface in cmd/dir.go |  |  | 0.731 |
| walker |  | 812 | 0 | go decl at cmd/dir.go:15 |  |  | 0.731 |
| walker |  | 821 | 9 | go decl at cmd/dir.go:10 |  |  | 0.731 |
| walker |  | 862 | 41 | go decl body at main.go:13 |  |  | 0.762 |
| walker |  | 887 | 25 | README.md section #2 |  |  | 0.762 |
| ns | 930 |  | 172 | CLAUDE.md: Key Packages | 1.9 |  | 0.727 |
| walker |  | 944 | 57 | go package + imports in main.go |  |  | 0.785 |
| walker |  | 963 | 19 | listing of 'cmd/testdata' |  |  | 0.785 |
| walker |  | 982 | 19 | listing of 'pkg/schema' |  |  | 0.785 |
| walker |  | 1007 | 25 | README.md section #3 |  |  | 0.785 |
| walker |  | 1053 | 46 | go decl names surface in cmd/secrets.go |  |  | 0.785 |
| walker |  | 1053 | 0 | go decl at cmd/secrets.go:14 |  |  | 0.785 |
| walker |  | 1053 | 0 | go decl at cmd/secrets.go:40 |  |  | 0.785 |
| walker |  | 1058 | 5 | go decl body at cmd/secrets.go:40 |  |  | 0.785 |
| walker |  | 1084 | 26 | go decl names surface in pkg/lookpath/error.go |  |  | 0.785 |
| walker |  | 1084 | 0 | go decl at pkg/lookpath/error.go:8 |  |  | 0.785 |
| walker |  | 1102 | 18 | go decl at pkg/lookpath/error.go:3 |  |  | 0.785 |
| walker |  | 1109 | 7 | go decl body at pkg/lookpath/error.go:8 |  |  | 0.785 |
| ns | 1112 |  | 182 | CLAUDE.md: build/test commands | 1.10 |  | 0.740 |
| walker |  | 1116 | 7 | go package + imports in pkg/common/cartesian.go |  |  | 0.740 |
| walker |  | 1123 | 7 | go package + imports in pkg/model/job_context.go |  |  | 0.740 |
| walker |  | 1131 | 8 | go package + imports in pkg/artifactcache/doc.go |  |  | 0.740 |
| walker |  | 1139 | 8 | go package + imports in pkg/artifactcache/model.go |  |  | 0.740 |
| walker |  | 1147 | 8 | go package + imports in pkg/lookpath/error.go |  |  | 0.740 |
| walker |  | 1307 | 160 | listing of 'pkg/container' |  |  | 0.740 |
| ns | 1314 |  | 202 | README rationale bullets (Fast Feedback / Local Task Runner) | 1.11 |  | 0.732 |
| walker |  | 1319 | 12 | go decl names surface in pkg/container/executions_environment.go |  |  | 0.732 |
| walker |  | 1345 | 26 | go decl names surface in pkg/container/parse_env_file.go |  |  | 0.732 |
| walker |  | 1345 | 0 | go decl at pkg/container/parse_env_file.go:14 |  |  | 0.732 |
| walker |  | 1381 | 36 | go decl names surface in pkg/container/docker_network.go |  |  | 0.732 |
| walker |  | 1381 | 0 | go decl at pkg/container/docker_network.go:12 |  |  | 0.732 |
| walker |  | 1381 | 0 | go decl at pkg/container/docker_network.go:45 |  |  | 0.732 |
| ns | 1563 |  | 249 | CLAUDE.md: linting + testing rules | 1.12 |  | 0.679 |
| walker |  | 1567 | 186 | listing of 'pkg/runner' |  |  | 0.679 |
| walker |  | 1605 | 38 | go decl names surface in pkg/artifactcache/model.go |  |  | 0.679 |
| walker |  | 1605 | 0 | go decl at pkg/artifactcache/model.go:9 |  |  | 0.679 |
| ns | 1622 |  | 59 | common.Executor type + Conditional | 2.1 |  | 0.664 |
| walker |  | 1651 | 46 | IMAGES.md section #0 |  |  | 0.664 |
| walker |  | 1691 | 40 | go decl names surface in pkg/common/file.go |  |  | 0.664 |
| walker |  | 1691 | 0 | go decl at pkg/common/file.go:10 |  |  | 0.664 |
| walker |  | 1691 | 0 | go decl at pkg/common/file.go:37 |  |  | 0.664 |
| walker |  | 1701 | 10 | go decl doc at pkg/common/file.go:10 |  |  | 0.664 |
| walker |  | 1713 | 12 | go decl doc at pkg/common/file.go:37 |  |  | 0.664 |
| walker |  | 1753 | 40 | go decl names surface in pkg/container/docker_volume.go |  |  | 0.664 |
| walker |  | 1753 | 0 | go decl at pkg/container/docker_volume.go:12 |  |  | 0.664 |
| walker |  | 1753 | 0 | go decl at pkg/container/docker_volume.go:36 |  |  | 0.664 |
| ns | 1758 |  | 136 | Plan / Stage / Run types | 2.2 |  | 0.625 |
| walker |  | 1794 | 41 | go decl names surface in pkg/common/cartesian.go |  |  | 0.625 |
| walker |  | 1794 | 0 | go decl at pkg/common/cartesian.go:4 |  |  | 0.625 |
| walker |  | 1794 | 0 | go decl at pkg/common/cartesian.go:25 |  |  | 0.625 |
| walker |  | 1815 | 21 | listing of 'pkg/model/testdata' |  |  | 0.625 |
| walker |  | 1860 | 45 | go decl names surface in pkg/model/anchors.go |  |  | 0.625 |
| walker |  | 1860 | 0 | go decl at pkg/model/anchors.go:9 |  |  | 0.625 |
| walker |  | 1860 | 0 | go decl at pkg/model/anchors.go:36 |  |  | 0.625 |
| ns | 1871 |  | 113 | Workflow struct + YAML tags | 2.3 |  | 0.604 |
| walker |  | 1906 | 46 | go decl names surface in pkg/common/context.go |  |  | 0.604 |
| walker |  | 1906 | 0 | go decl at pkg/common/context.go:10 |  |  | 0.604 |
| walker |  | 1906 | 0 | go decl at pkg/common/context.go:42 |  |  | 0.604 |
| walker |  | 1952 | 46 | go decl names surface in pkg/container/util.go |  |  | 0.604 |
| walker |  | 1952 | 0 | go decl at pkg/container/util.go:12 |  |  | 0.604 |
| walker |  | 1952 | 0 | go decl at pkg/container/util.go:24 |  |  | 0.604 |
| ns | 1953 |  | 82 | WorkflowPlanner interface | 2.4 |  | 0.590 |
| walker |  | 2000 | 48 | go decl names surface in pkg/container/docker_auth.go |  |  | 0.590 |
| walker |  | 2000 | 0 | go decl at pkg/container/docker_auth.go:15 |  |  | 0.590 |
| walker |  | 2000 | 0 | go decl at pkg/container/docker_auth.go:42 |  |  | 0.590 |
| walker |  | 2018 | 18 | go decl doc at pkg/common/cartesian.go:4 |  |  | 0.590 |
| walker |  | 2061 | 43 | go decl at pkg/artifactcache/model.go:3 |  |  | 0.590 |
| walker |  | 2113 | 52 | go decl names surface in pkg/container/docker_build.go |  |  | 0.590 |
| walker |  | 2113 | 0 | go decl at pkg/container/docker_build.go:23 |  |  | 0.590 |
| walker |  | 2113 | 0 | go decl at pkg/container/docker_build.go:78 |  |  | 0.590 |
| walker |  | 2138 | 25 | go package + imports in cmd/platforms.go |  |  | 0.590 |
| walker |  | 2157 | 19 | go decl doc at pkg/container/docker_build.go:23 |  |  | 0.590 |
| walker |  | 2164 | 7 | go decl body at pkg/container/util.go:24 |  |  | 0.590 |
| ns | 2185 |  | 232 | Step struct (YAML fields) | 2.5 |  | 0.559 |
| walker |  | 2220 | 56 | go decl names surface in pkg/runner/step_factory.go |  |  | 0.559 |
| walker |  | 2220 | 0 | go decl at pkg/runner/step_factory.go:15 |  |  | 0.559 |
| ns | 2455 |  | 270 | Job struct (YAML fields) | 2.6 |  | 0.528 |
| walker |  | 2508 | 288 | README.md section #0 |  |  | 0.536 |
| walker |  | 2565 | 57 | go decl names surface in pkg/lookpath/env.go |  |  | 0.536 |
| walker |  | 2565 | 0 | go decl at pkg/lookpath/env.go:12 |  |  | 0.536 |
| walker |  | 2565 | 0 | go decl at pkg/lookpath/env.go:16 |  |  | 0.536 |
| walker |  | 2568 | 3 | go decl at pkg/lookpath/env.go:9 |  |  | 0.536 |
| walker |  | 2582 | 14 | go decl at pkg/lookpath/env.go:5 |  |  | 0.536 |
| walker |  | 2594 | 12 | go decl body at pkg/lookpath/env.go:16 |  |  | 0.536 |
| walker |  | 2609 | 15 | go package + imports in pkg/container/executions_environment.go |  |  | 0.536 |
| walker |  | 2624 | 15 | go package + imports in pkg/model/step_result.go |  |  | 0.536 |
| ns | 2646 |  | 191 | Strategy + Defaults + RunDefaults | 2.7 |  | 0.510 |
| walker |  | 2682 | 58 | go decl names surface in pkg/container/docker_images.go |  |  | 0.510 |
| walker |  | 2682 | 0 | go decl at pkg/container/docker_images.go:16 |  |  | 0.510 |
| walker |  | 2682 | 0 | go decl at pkg/container/docker_images.go:44 |  |  | 0.510 |
| walker |  | 2689 | 7 | go decl body at pkg/lookpath/env.go:12 |  |  | 0.510 |
| walker |  | 2692 | 3 | listing of 'pkg/artifactcache/testdata' |  |  | 0.510 |
| walker |  | 2708 | 16 | go package + imports in pkg/lookpath/env.go |  |  | 0.510 |
| walker |  | 2724 | 16 | go package + imports in pkg/workflowpattern/trace_writer.go |  |  | 0.510 |
| walker |  | 2794 | 70 | go decl names surface in pkg/common/dryrun.go |  |  | 0.510 |
| walker |  | 2794 | 0 | go decl at pkg/common/dryrun.go:12 |  |  | 0.510 |
| walker |  | 2794 | 0 | go decl at pkg/common/dryrun.go:23 |  |  | 0.510 |
| ns | 2803 |  | 157 | ContainerSpec (services / container YAML) | 2.8 |  | 0.494 |
| walker |  | 2811 | 17 | go decl doc at pkg/common/dryrun.go:12 |  |  | 0.494 |
| walker |  | 2829 | 18 | go decl doc at pkg/common/dryrun.go:23 |  |  | 0.494 |
| walker |  | 2846 | 17 | go decl body at pkg/common/dryrun.go:23 |  |  | 0.494 |
| walker |  | 2916 | 70 | go decl names surface in pkg/container/docker_pull.go |  |  | 0.494 |
| walker |  | 2916 | 0 | go decl at pkg/container/docker_pull.go:21 |  |  | 0.494 |
| walker |  | 2916 | 0 | go decl at pkg/container/docker_pull.go:78 |  |  | 0.494 |
| walker |  | 2916 | 0 | go decl at pkg/container/docker_pull.go:123 |  |  | 0.494 |
| walker |  | 2935 | 19 | go decl doc at pkg/container/docker_pull.go:21 |  |  | 0.494 |
| walker |  | 3007 | 72 | go decl names surface in pkg/common/logger.go |  |  | 0.475 |
| walker |  | 3007 | 0 | go decl at pkg/common/logger.go:14 |  |  | 0.475 |
| walker |  | 3007 | 0 | go decl at pkg/common/logger.go:25 |  |  | 0.475 |
| ns | 3007 |  | 204 | JobType enum constants | 2.9 |  | 0.475 |
| walker |  | 3021 | 14 | go decl doc at pkg/common/logger.go:14 |  |  | 0.475 |
| walker |  | 3038 | 17 | go decl doc at pkg/common/logger.go:25 |  |  | 0.475 |
| walker |  | 3053 | 15 | go decl body at pkg/common/logger.go:25 |  |  | 0.475 |
| walker |  | 3192 | 139 | go decl names surface in cmd/notices.go |  |  | 0.475 |
| walker |  | 3192 | 0 | go decl at cmd/notices.go:22 |  |  | 0.475 |
| walker |  | 3192 | 0 | go decl at cmd/notices.go:57 |  |  | 0.475 |
| walker |  | 3192 | 0 | go decl at cmd/notices.go:65 |  |  | 0.475 |
| walker |  | 3192 | 0 | go decl at cmd/notices.go:121 |  |  | 0.475 |
| walker |  | 3192 | 0 | go decl at cmd/notices.go:130 |  |  | 0.475 |
| walker |  | 3192 | 0 | go decl at cmd/notices.go:138 |  |  | 0.475 |
| walker |  | 3219 | 27 | go decl at cmd/notices.go:17 |  |  | 0.475 |
| walker |  | 3256 | 37 | listing of 'pkg/container/testdata' |  |  | 0.475 |
| walker |  | 3333 | 77 | go decl names surface in pkg/workflowpattern/trace_writer.go |  |  | 0.475 |
| walker |  | 3333 | 0 | go decl at pkg/workflowpattern/trace_writer.go:16 |  |  | 0.475 |
| walker |  | 3347 | 14 | go decl at pkg/workflowpattern/trace_writer.go:5 |  |  | 0.475 |
| ns | 3348 |  | 341 | StepType enum constants | 2.10 |  | 0.450 |
| walker |  | 3357 | 10 | go decl body at pkg/workflowpattern/trace_writer.go:16 |  |  | 0.450 |
| walker |  | 3381 | 24 | go decl at pkg/runner/step_factory.go:9 |  |  | 0.450 |
| walker |  | 3385 | 4 | listing of 'pkg/runner/hashfiles' |  |  | 0.450 |
| walker |  | 3389 | 4 | listing of 'pkg/runner/res' |  |  | 0.450 |
| walker |  | 3413 | 24 | listing of 'pkg/model/testdata/invalid-job-name' |  |  | 0.450 |
| walker |  | 3561 | 148 | go decl names surface in cmd/input.go |  |  | 0.450 |
| walker |  | 3561 | 0 | go decl at cmd/input.go:70 |  |  | 0.450 |
| walker |  | 3561 | 0 | go decl at cmd/input.go:85 |  |  | 0.450 |
| walker |  | 3561 | 0 | go decl at cmd/input.go:90 |  |  | 0.450 |
| walker |  | 3561 | 0 | go decl at cmd/input.go:94 |  |  | 0.450 |
| walker |  | 3561 | 0 | go decl at cmd/input.go:99 |  |  | 0.450 |
| walker |  | 3561 | 0 | go decl at cmd/input.go:104 |  |  | 0.450 |
| walker |  | 3561 | 0 | go decl at cmd/input.go:109 |  |  | 0.450 |
| walker |  | 3561 | 0 | go decl at cmd/input.go:114 |  |  | 0.450 |
| walker |  | 3568 | 7 | go decl body at cmd/input.go:99 |  |  | 0.450 |
| walker |  | 3577 | 9 | go decl body at cmd/input.go:85 |  |  | 0.450 |
| walker |  | 3586 | 9 | go decl body at cmd/input.go:90 |  |  | 0.450 |
| walker |  | 3595 | 9 | go decl body at cmd/input.go:94 |  |  | 0.450 |
| walker |  | 3604 | 9 | go decl body at cmd/input.go:109 |  |  | 0.450 |
| walker |  | 3613 | 9 | go decl body at cmd/input.go:114 |  |  | 0.450 |
| walker |  | 3623 | 10 | go decl body at cmd/input.go:104 |  |  | 0.450 |
| walker |  | 3635 | 12 | go decl doc at cmd/input.go:90 |  |  | 0.450 |
| walker |  | 3648 | 13 | go decl doc at cmd/input.go:85 |  |  | 0.450 |
| walker |  | 3661 | 13 | go decl doc at cmd/input.go:99 |  |  | 0.450 |
| ns | 3671 |  | 323 | Step.Type() dispatch body | 2.11 |  | 0.430 |
| walker |  | 3675 | 14 | go decl doc at cmd/input.go:109 |  |  | 0.430 |
| walker |  | 3690 | 15 | go decl doc at cmd/input.go:104 |  |  | 0.430 |
| walker |  | 3705 | 15 | go decl doc at cmd/input.go:114 |  |  | 0.430 |
| walker |  | 3788 | 83 | go decl names surface in pkg/container/docker_socket.go |  |  | 0.430 |
| walker |  | 3788 | 0 | go decl at pkg/container/docker_socket.go:23 |  |  | 0.430 |
| walker |  | 3788 | 0 | go decl at pkg/container/docker_socket.go:45 |  |  | 0.430 |
| walker |  | 3788 | 0 | go decl at pkg/container/docker_socket.go:62 |  |  | 0.430 |
| walker |  | 3808 | 20 | go decl at pkg/container/docker_socket.go:57 |  |  | 0.430 |
| walker |  | 3838 | 30 | go decl doc at pkg/container/docker_images.go:44 |  |  | 0.430 |
| walker |  | 3923 | 85 | go decl names surface in pkg/runner/local_repository_cache.go |  |  | 0.430 |
| walker |  | 3923 | 0 | go decl at pkg/runner/local_repository_cache.go:25 |  |  | 0.430 |
| walker |  | 3923 | 0 | go decl at pkg/runner/local_repository_cache.go:44 |  |  | 0.430 |
| walker |  | 3959 | 36 | go decl at pkg/runner/local_repository_cache.go:19 |  |  | 0.430 |
| ns | 4026 |  | 355 | ActionRunsUsing constants (node12/16/20/24, docker, composite) | 2.12 |  | 0.406 |
| walker |  | 4045 | 86 | go decl names surface in pkg/container/docker_logger.go |  |  | 0.406 |
| walker |  | 4045 | 0 | go decl at pkg/container/docker_logger.go:27 |  |  | 0.406 |
| walker |  | 4045 | 0 | go decl at pkg/container/docker_logger.go:77 |  |  | 0.406 |
| walker |  | 4087 | 42 | go package + imports in cmd/input.go |  |  | 0.406 |
| walker |  | 4177 | 90 | go decl names surface in pkg/common/line_writer.go |  |  | 0.406 |
| walker |  | 4177 | 0 | go decl at pkg/common/line_writer.go:9 |  |  | 0.406 |
| walker |  | 4177 | 0 | go decl at pkg/common/line_writer.go:17 |  |  | 0.406 |
| walker |  | 4177 | 0 | go decl at pkg/common/line_writer.go:23 |  |  | 0.406 |
| walker |  | 4177 | 0 | go decl at pkg/common/line_writer.go:43 |  |  | 0.406 |
| walker |  | 4193 | 16 | go decl doc at pkg/common/line_writer.go:9 |  |  | 0.406 |
| walker |  | 4210 | 17 | go decl doc at pkg/common/line_writer.go:17 |  |  | 0.406 |
| walker |  | 4232 | 22 | go decl at pkg/common/line_writer.go:11 |  |  | 0.406 |
| walker |  | 4324 | 92 | go decl names surface in pkg/runner/action_cache_offline_mode.go |  |  | 0.406 |
| walker |  | 4324 | 0 | go decl at pkg/runner/action_cache_offline_mode.go:17 |  |  | 0.406 |
| walker |  | 4324 | 0 | go decl at pkg/runner/action_cache_offline_mode.go:45 |  |  | 0.406 |
| walker |  | 4338 | 14 | go decl at pkg/runner/action_cache_offline_mode.go:13 |  |  | 0.406 |
| walker |  | 4356 | 18 | go decl body at pkg/runner/action_cache_offline_mode.go:45 |  |  | 0.406 |
| walker |  | 4380 | 24 | go decl body at pkg/common/context.go:42 |  |  | 0.406 |
| walker |  | 4404 | 24 | go decl body at pkg/common/line_writer.go:17 |  |  | 0.406 |
| walker |  | 4429 | 25 | go package + imports in pkg/common/dryrun.go |  |  | 0.406 |
| ns | 4452 |  | 426 | Action + ActionRuns structs | 2.13 |  | 0.386 |
| walker |  | 4454 | 25 | go package + imports in pkg/common/job_error.go |  |  | 0.386 |
| walker |  | 4469 | 15 | go decl doc at pkg/container/docker_socket.go:23 |  |  | 0.386 |
| walker |  | 4478 | 9 | listing of 'pkg/gh' |  |  | 0.386 |
| walker |  | 4501 | 23 | go decl names surface in pkg/gh/gh.go |  |  | 0.386 |
| walker |  | 4501 | 0 | go decl at pkg/gh/gh.go:10 |  |  | 0.386 |
| walker |  | 4603 | 102 | go decl names surface in pkg/container/container_types.go |  |  | 0.386 |
| walker |  | 4612 | 9 | go decl at pkg/container/container_types.go:80 |  |  | 0.386 |
| walker |  | 4639 | 27 | go decl at pkg/container/container_types.go:36 |  |  | 0.386 |
| walker |  | 4655 | 16 | go decl doc at pkg/container/container_types.go:36 |  |  | 0.386 |
| walker |  | 4702 | 47 | go decl at pkg/container/container_types.go:70 |  |  | 0.386 |
| walker |  | 4722 | 20 | go decl doc at pkg/container/container_types.go:70 |  |  | 0.386 |
| walker |  | 4772 | 50 | go decl at pkg/container/container_types.go:61 |  |  | 0.386 |
| walker |  | 4792 | 20 | go decl doc at pkg/container/container_types.go:61 |  |  | 0.386 |
| walker |  | 4841 | 49 | go package + imports in cmd/dir.go |  |  | 0.386 |
| ns | 4852 |  | 400 | Action Input + Output + ReadAction | 2.14 |  | 0.368 |
| walker |  | 4859 | 18 | listing of 'pkg/artifacts/testdata' |  |  | 0.368 |
| walker |  | 4897 | 38 | go decl doc at pkg/container/docker_images.go:16 |  |  | 0.368 |
| walker |  | 5009 | 112 | go decl names surface in pkg/common/auth.go |  |  | 0.368 |
| walker |  | 5009 | 0 | go decl at pkg/common/auth.go:38 |  |  | 0.368 |
| walker |  | 5009 | 0 | go decl at pkg/common/auth.go:72 |  |  | 0.368 |
| walker |  | 5018 | 9 | go decl at pkg/common/auth.go:33 |  |  | 0.368 |
| walker |  | 5040 | 22 | go decl at pkg/common/auth.go:26 |  |  | 0.368 |
| walker |  | 5094 | 54 | go package + imports in cmd/list.go |  |  | 0.368 |
| ns | 5195 |  | 343 | Job.Type() dispatch body | 2.15 |  | 0.356 |
| walker |  | 5210 | 116 | go decl names surface in pkg/runner/job_executor.go |  |  | 0.356 |
| walker |  | 5210 | 0 | go decl at pkg/runner/job_executor.go:23 |  |  | 0.356 |
| walker |  | 5210 | 0 | go decl at pkg/runner/job_executor.go:157 |  |  | 0.356 |
| walker |  | 5210 | 0 | go decl at pkg/runner/job_executor.go:185 |  |  | 0.356 |
| walker |  | 5210 | 0 | go decl at pkg/runner/job_executor.go:200 |  |  | 0.356 |
| walker |  | 5221 | 11 | go decl doc at pkg/runner/job_executor.go:23 |  |  | 0.356 |
| walker |  | 5276 | 55 | go package + imports in cmd/graph.go |  |  | 0.356 |
| ns | 5506 |  | 311 | RunContext struct | 2.16 |  | 0.344 |
| walker |  | 6086 | 810 | go module file go.mod |  |  | 0.344 |
| walker |  | 6206 | 120 | go decl names surface in pkg/workflowpattern/workflow_pattern.go |  |  | 0.344 |
| walker |  | 6206 | 0 | go decl at pkg/workflowpattern/workflow_pattern.go:15 |  |  | 0.344 |
| walker |  | 6206 | 0 | go decl at pkg/workflowpattern/workflow_pattern.go:38 |  |  | 0.344 |
| walker |  | 6206 | 0 | go decl at pkg/workflowpattern/workflow_pattern.go:138 |  |  | 0.344 |
| walker |  | 6206 | 0 | go decl at pkg/workflowpattern/workflow_pattern.go:151 |  |  | 0.344 |
| walker |  | 6206 | 0 | go decl at pkg/workflowpattern/workflow_pattern.go:177 |  |  | 0.344 |
| walker |  | 6218 | 12 | go decl doc at pkg/workflowpattern/workflow_pattern.go:38 |  |  | 0.344 |
| walker |  | 6250 | 32 | go decl at pkg/workflowpattern/workflow_pattern.go:9 |  |  | 0.344 |
| walker |  | 6267 | 17 | go decl doc at pkg/workflowpattern/workflow_pattern.go:151 |  |  | 0.344 |
| walker |  | 6286 | 19 | go decl doc at pkg/workflowpattern/workflow_pattern.go:177 |  |  | 0.344 |
| walker |  | 6318 | 32 | go package + imports in pkg/common/line_writer.go |  |  | 0.344 |
| ns | 6439 |  | 933 | Executor combinators (Then/OnError/Finally/If/IfNot) | 2.17 |  | 0.307 |
| walker |  | 6450 | 132 | go package doc lede in pkg/artifactcache/doc.go |  |  | 0.307 |
| walker |  | 6579 | 129 | go decl names surface in pkg/model/step_result.go |  |  | 0.307 |
| walker |  | 6579 | 0 | go decl at pkg/model/step_result.go:19 |  |  | 0.307 |
| walker |  | 6579 | 0 | go decl at pkg/model/step_result.go:23 |  |  | 0.307 |
| walker |  | 6579 | 0 | go decl at pkg/model/step_result.go:34 |  |  | 0.307 |
| walker |  | 6588 | 9 | go decl at pkg/model/step_result.go:7 |  |  | 0.307 |
| walker |  | 6637 | 49 | go decl at pkg/model/step_result.go:41 |  |  | 0.307 |
| walker |  | 6647 | 10 | go decl body at pkg/model/step_result.go:19 |  |  | 0.307 |
| walker |  | 6670 | 23 | listing of 'pkg/exprparser/testdata' |  |  | 0.307 |
| walker |  | 6674 | 4 | listing of 'pkg/artifacts/testdata/GHSL-2023-004' |  |  | 0.307 |
| walker |  | 6678 | 4 | listing of 'pkg/artifacts/testdata/upload-and-download' |  |  | 0.307 |
| walker |  | 6682 | 4 | listing of 'pkg/artifacts/testdata/v4' |  |  | 0.307 |
| walker |  | 6686 | 4 | listing of 'pkg/container/testdata/docker-pull-options' |  |  | 0.307 |
| walker |  | 6690 | 4 | listing of 'pkg/container/testdata/scratch' |  |  | 0.307 |
| walker |  | 6694 | 4 | listing of 'pkg/model/testdata/container-volumes' |  |  | 0.307 |
| walker |  | 6698 | 4 | listing of 'pkg/model/testdata/empty-workflow' |  |  | 0.307 |
| walker |  | 6702 | 4 | listing of 'pkg/model/testdata/strategy' |  |  | 0.307 |
| walker |  | 6796 | 94 | go decl at pkg/model/job_context.go:3 |  |  | 0.307 |
| walker |  | 6863 | 67 | go package + imports in cmd/secrets.go |  |  | 0.307 |
| walker |  | 6876 | 13 | listing of 'pkg/filecollector' |  |  | 0.307 |
| walker |  | 6978 | 102 | README.md section #4 |  |  | 0.307 |
| walker |  | 7017 | 39 | go package + imports in pkg/common/file.go |  |  | 0.307 |
| walker |  | 7056 | 39 | go package + imports in pkg/common/logger.go |  |  | 0.307 |
| walker |  | 7096 | 40 | go package + imports in pkg/common/outbound_ip.go |  |  | 0.307 |
| walker |  | 7136 | 40 | go package + imports in pkg/model/anchors.go |  |  | 0.307 |
| walker |  | 7176 | 40 | go package + imports in pkg/runner/step_factory.go |  |  | 0.307 |
| walker |  | 7217 | 41 | go package + imports in pkg/workflowpattern/workflow_pattern.go |  |  | 0.307 |
| ns | 7272 |  | 833 | NewPipelineExecutor + NewParallelExecutor | 2.18 |  | 0.284 |
| walker |  | 7376 | 159 | go decl names surface in pkg/container/linux_container_environment_extensions.go |  |  | 0.284 |
| walker |  | 7376 | 0 | go decl at pkg/container/linux_container_environment_extensions.go:19 |  |  | 0.284 |
| walker |  | 7376 | 0 | go decl at pkg/container/linux_container_environment_extensions.go:50 |  |  | 0.284 |
| walker |  | 7376 | 0 | go decl at pkg/container/linux_container_environment_extensions.go:54 |  |  | 0.284 |
| walker |  | 7376 | 0 | go decl at pkg/container/linux_container_environment_extensions.go:58 |  |  | 0.284 |
| walker |  | 7376 | 0 | go decl at pkg/container/linux_container_environment_extensions.go:62 |  |  | 0.284 |
| walker |  | 7376 | 0 | go decl at pkg/container/linux_container_environment_extensions.go:66 |  |  | 0.284 |
| walker |  | 7376 | 0 | go decl at pkg/container/linux_container_environment_extensions.go:75 |  |  | 0.284 |
| walker |  | 7379 | 3 | go decl at pkg/container/linux_container_environment_extensions.go:13 |  |  | 0.284 |
| walker |  | 7384 | 5 | go decl body at pkg/container/linux_container_environment_extensions.go:75 |  |  | 0.284 |
| walker |  | 7390 | 6 | go decl body at pkg/container/linux_container_environment_extensions.go:54 |  |  | 0.284 |
| walker |  | 7399 | 9 | go decl body at pkg/container/linux_container_environment_extensions.go:50 |  |  | 0.284 |
| walker |  | 7408 | 9 | go decl body at pkg/container/linux_container_environment_extensions.go:62 |  |  | 0.284 |
| walker |  | 7433 | 25 | go decl body at pkg/container/linux_container_environment_extensions.go:58 |  |  | 0.284 |
| walker |  | 7596 | 163 | go decl names surface in pkg/runner/action_composite.go |  |  | 0.284 |
| walker |  | 7596 | 0 | go decl at pkg/runner/action_composite.go:13 |  |  | 0.284 |
| walker |  | 7596 | 0 | go decl at pkg/runner/action_composite.go:47 |  |  | 0.284 |
| walker |  | 7596 | 0 | go decl at pkg/runner/action_composite.go:84 |  |  | 0.284 |
| walker |  | 7596 | 0 | go decl at pkg/runner/action_composite.go:133 |  |  | 0.284 |
| walker |  | 7596 | 0 | go decl at pkg/runner/action_composite.go:197 |  |  | 0.284 |
| walker |  | 7596 | 0 | go decl at pkg/runner/action_composite.go:222 |  |  | 0.284 |
| walker |  | 7624 | 28 | go decl at pkg/runner/action_composite.go:126 |  |  | 0.284 |
| walker |  | 7642 | 18 | go decl doc at pkg/runner/action_composite.go:133 |  |  | 0.284 |
| ns | 7717 |  | 445 | stepFactoryImpl.newStep — step-type dispatch | 3.1 |  | 0.276 |
| walker |  | 7750 | 108 | go decl at pkg/container/executions_environment.go:5 |  |  | 0.277 |
| walker |  | 7767 | 17 | go decl body at pkg/model/anchors.go:36 |  |  | 0.277 |
| walker |  | 7794 | 27 | go decl body at cmd/notices.go:57 |  |  | 0.277 |
| walker |  | 7822 | 28 | go decl at pkg/model/step_result.go:13 |  |  | 0.277 |
| walker |  | 7868 | 46 | go package + imports in pkg/common/draw.go |  |  | 0.277 |
| ns | 7928 |  | 211 | step interface (pre/main/post) | 3.2 |  | 0.272 |
| walker |  | 8047 | 179 | go decl names surface in pkg/common/job_error.go |  |  | 0.272 |
| walker |  | 8047 | 0 | go decl at pkg/common/job_error.go:16 |  |  | 0.272 |
| walker |  | 8047 | 0 | go decl at pkg/common/job_error.go:26 |  |  | 0.272 |
| walker |  | 8047 | 0 | go decl at pkg/common/job_error.go:31 |  |  | 0.272 |
| walker |  | 8047 | 0 | go decl at pkg/common/job_error.go:36 |  |  | 0.272 |
| walker |  | 8047 | 0 | go decl at pkg/common/job_error.go:40 |  |  | 0.272 |
| walker |  | 8047 | 0 | go decl at pkg/common/job_error.go:51 |  |  | 0.272 |
| walker |  | 8064 | 17 | go decl doc at pkg/common/job_error.go:16 |  |  | 0.272 |
| walker |  | 8080 | 16 | go decl body at pkg/common/job_error.go:36 |  |  | 0.272 |
| walker |  | 8102 | 22 | go decl doc at pkg/common/job_error.go:31 |  |  | 0.272 |
| walker |  | 8130 | 28 | go decl doc at pkg/common/job_error.go:51 |  |  | 0.272 |
| walker |  | 8151 | 21 | go decl body at pkg/common/job_error.go:26 |  |  | 0.272 |
| walker |  | 8178 | 27 | go decl body at pkg/common/job_error.go:31 |  |  | 0.272 |
| ns | 8215 |  | 287 | RunContext.Executor — JobType dispatch | 3.3 |  | 0.266 |
| walker |  | 8225 | 47 | go package + imports in pkg/container/util.go |  |  | 0.266 |
| walker |  | 8234 | 9 | listing of 'pkg/common/git' |  |  | 0.266 |
| walker |  | 8294 | 60 | go decl doc at pkg/container/linux_container_environment_extensions.go:19 |  |  | 0.266 |
| ns | 8411 |  | 196 | actionStep interface + action loader signatures | 3.4 |  | 0.262 |
| walker |  | 8434 | 140 | go decl names surface in pkg/runner/run_context.go |  |  | 0.262 |
| walker |  | 8434 | 0 | go decl at pkg/runner/run_context.go:58 |  |  | 0.262 |
| walker |  | 8434 | 0 | go decl at pkg/runner/run_context.go:67 |  |  | 0.262 |
| walker |  | 8434 | 0 | go decl at pkg/runner/run_context.go:78 |  |  | 0.262 |
| walker |  | 8434 | 0 | go decl at pkg/runner/run_context.go:92 |  |  | 0.262 |
| walker |  | 8434 | 0 | go decl at pkg/runner/run_context.go:98 |  |  | 0.262 |
| walker |  | 8434 | 0 | go decl at pkg/runner/run_context.go:108 |  |  | 0.262 |
| walker |  | 8455 | 21 | go decl at pkg/runner/run_context.go:62 |  |  | 0.262 |
| walker |  | 8469 | 14 | go decl doc at pkg/runner/run_context.go:78 |  |  | 0.262 |
| walker |  | 8482 | 13 | go decl body at pkg/runner/run_context.go:58 |  |  | 0.262 |
| walker |  | 8494 | 12 | go decl body at pkg/runner/run_context.go:92 |  |  | 0.262 |
| walker |  | 8544 | 50 | go package + imports in pkg/common/context.go |  |  | 0.262 |
| walker |  | 8594 | 50 | go package + imports in pkg/gh/gh.go |  |  | 0.262 |
| walker |  | 8647 | 53 | go package + imports in pkg/container/docker_network.go |  |  | 0.262 |
| walker |  | 8700 | 53 | go package + imports in pkg/container/docker_volume.go |  |  | 0.262 |
| ns | 8943 |  | 532 | runActionImpl — Node/Docker/Composite dispatch (switch body) | 3.5 |  | 0.255 |
| walker |  | 9072 | 372 | go decl names surface in cmd/root.go |  |  | 0.255 |
| walker |  | 9072 | 0 | go decl at cmd/root.go:47 |  |  | 0.255 |
| walker |  | 9072 | 0 | go decl at cmd/root.go:56 |  |  | 0.255 |
| walker |  | 9072 | 0 | go decl at cmd/root.go:138 |  |  | 0.255 |
| walker |  | 9072 | 0 | go decl at cmd/root.go:155 |  |  | 0.255 |
| walker |  | 9072 | 0 | go decl at cmd/root.go:167 |  |  | 0.255 |
| walker |  | 9072 | 0 | go decl at cmd/root.go:256 |  |  | 0.255 |
| walker |  | 9072 | 0 | go decl at cmd/root.go:268 |  |  | 0.255 |
| walker |  | 9072 | 0 | go decl at cmd/root.go:278 |  |  | 0.255 |
| walker |  | 9072 | 0 | go decl at cmd/root.go:304 |  |  | 0.255 |
| walker |  | 9072 | 0 | go decl at cmd/root.go:314 |  |  | 0.255 |
| walker |  | 9072 | 0 | go decl at cmd/root.go:320 |  |  | 0.255 |
| walker |  | 9072 | 0 | go decl at cmd/root.go:333 |  |  | 0.255 |
| walker |  | 9072 | 0 | go decl at cmd/root.go:345 |  |  | 0.255 |
| walker |  | 9072 | 0 | go decl at cmd/root.go:349 |  |  | 0.255 |
| walker |  | 9072 | 0 | go decl at cmd/root.go:373 |  |  | 0.255 |
| walker |  | 9072 | 0 | go decl at cmd/root.go:391 |  |  | 0.255 |
| walker |  | 9072 | 0 | go decl at cmd/root.go:716 |  |  | 0.255 |
| walker |  | 9072 | 0 | go decl at cmd/root.go:759 |  |  | 0.255 |
| walker |  | 9087 | 15 | go decl doc at cmd/root.go:47 |  |  | 0.255 |
| ns | 9088 |  | 145 | ExecutionsEnvironment interface (container abstraction) | 3.6 |  | 0.273 |
| walker |  | 9137 | 50 | go decl at cmd/root.go:37 |  |  | 0.273 |
| walker |  | 9149 | 12 | go decl doc at cmd/root.go:391 |  |  | 0.273 |
| walker |  | 9163 | 14 | go decl body at cmd/root.go:345 |  |  | 0.273 |
| walker |  | 9198 | 35 | go decl doc at cmd/root.go:138 |  |  | 0.273 |
| walker |  | 9252 | 54 | go decl body at cmd/root.go:47 |  |  | 0.273 |
| walker |  | 9282 | 30 | go decl body at cmd/root.go:314 |  |  | 0.273 |
| walker |  | 9336 | 54 | go package + imports in pkg/common/executor.go |  |  | 0.273 |
| ns | 9356 |  | 268 | Container interface (Docker wrapper API) | 3.7 |  | 0.269 |
| walker |  | 9546 | 210 | go decl names surface in pkg/runner/runner.go |  |  | 0.269 |
| walker |  | 9546 | 0 | go decl at pkg/runner/runner.go:67 |  |  | 0.269 |
| walker |  | 9546 | 0 | go decl at pkg/runner/runner.go:91 |  |  | 0.269 |
| walker |  | 9546 | 0 | go decl at pkg/runner/runner.go:99 |  |  | 0.269 |
| walker |  | 9546 | 0 | go decl at pkg/runner/runner.go:122 |  |  | 0.269 |
| walker |  | 9546 | 0 | go decl at pkg/runner/runner.go:221 |  |  | 0.269 |
| walker |  | 9546 | 0 | go decl at pkg/runner/runner.go:234 |  |  | 0.269 |
| walker |  | 9546 | 0 | go decl at pkg/runner/runner.go:253 |  |  | 0.269 |
| walker |  | 9564 | 18 | go decl at pkg/runner/runner.go:17 |  |  | 0.269 |
| walker |  | 9575 | 11 | go decl doc at pkg/runner/runner.go:91 |  |  | 0.269 |
| walker |  | 9589 | 14 | go decl doc at pkg/runner/runner.go:17 |  |  | 0.269 |
| walker |  | 9602 | 13 | go decl at pkg/runner/runner.go:80 |  |  | 0.269 |
| ns | 9603 |  | 247 | runner.Config field names (truncated, with marker) | 4.1 |  | 0.264 |
| walker |  | 9611 | 9 | go decl doc at pkg/runner/runner.go:122 |  |  | 0.264 |
| walker |  | 9644 | 33 | go decl body at pkg/runner/runner.go:91 |  |  | 0.264 |
| walker |  | 9686 | 42 | go decl at pkg/runner/runner.go:84 |  |  | 0.264 |
| walker |  | 9762 | 76 | go decl doc at pkg/common/outbound_ip.go:13 |  |  | 0.264 |
| ns | 9776 |  | 173 | EvaluationEnvironment (expression context bag) | 4.2 |  | 0.261 |
| ns | 9860 |  | 84 | exprparser function names (locations) | 4.3 |  | 0.258 |
| ns | 9959 |  | 99 | command.go workflow-command case lines (locations) | 4.4 |  | 0.257 |
| walker |  | 9976 | 214 | go decl names surface in pkg/container/docker_stub.go |  |  | 0.257 |
| walker |  | 9976 | 0 | go decl at pkg/container/docker_stub.go:16 |  |  | 0.257 |
| walker |  | 9976 | 0 | go decl at pkg/container/docker_stub.go:22 |  |  | 0.257 |
| walker |  | 9976 | 0 | go decl at pkg/container/docker_stub.go:27 |  |  | 0.257 |
| walker |  | 9976 | 0 | go decl at pkg/container/docker_stub.go:34 |  |  | 0.257 |
| walker |  | 9976 | 0 | go decl at pkg/container/docker_stub.go:41 |  |  | 0.257 |
| walker |  | 9976 | 0 | go decl at pkg/container/docker_stub.go:45 |  |  | 0.257 |
| walker |  | 9976 | 0 | go decl at pkg/container/docker_stub.go:49 |  |  | 0.257 |
| walker |  | 9976 | 0 | go decl at pkg/container/docker_stub.go:53 |  |  | 0.257 |
| walker |  | 9976 | 0 | go decl at pkg/container/docker_stub.go:59 |  |  | 0.257 |
| walker |  | 9976 | 0 | go decl at pkg/container/docker_stub.go:65 |  |  | 0.257 |
| walker |  | 9981 | 5 | go decl body at pkg/container/docker_stub.go:41 |  |  | 0.257 |
| walker |  | 9989 | 8 | go decl body at pkg/container/docker_stub.go:45 |  |  | 0.257 |
| walker |  | 9997 | 8 | go decl body at pkg/container/docker_stub.go:49 |  |  | 0.257 |

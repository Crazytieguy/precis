Score(3000)=0.517 I=0.836 C=0.320 ns_rows≤3K=12/42 (reached=3 partial=2 missing=7)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 62 |  | 62 | README headline + tagline + How-it-works heading | 1.1 |  | 0.000 |
| ns | 123 |  | 61 | main.go func main body | 1.2 |  | 0.000 |
| walker |  | 125 | 125 | listing of '.' |  |  | 0.000 |
| walker |  | 145 | 20 | go decl names surface in main.go |  |  | 0.065 |
| walker |  | 145 | 0 | go decl at main.go:11 |  |  | 0.065 |
| walker |  | 145 | 0 | go decl at main.go:13 |  |  | 0.065 |
| walker |  | 170 | 25 | go module identity in go.mod |  |  | 0.065 |
| walker |  | 180 | 10 | plaintext config VERSION |  |  | 0.065 |
| walker |  | 232 | 52 | README headline in README.md |  |  | 0.501 |
| walker |  | 242 | 10 | go decl doc at main.go:11 |  |  | 0.502 |
| ns | 248 |  | 125 | Repository root listing | 1.3 |  | 0.747 |
| walker |  | 288 | 46 | listing of 'cmd' |  |  | 0.770 |
| walker |  | 304 | 16 | go decl names surface in cmd/graph.go |  |  | 0.770 |
| walker |  | 304 | 0 | go decl at cmd/graph.go:10 |  |  | 0.770 |
| walker |  | 320 | 16 | go decl names surface in cmd/list.go |  |  | 0.770 |
| walker |  | 320 | 0 | go decl at cmd/list.go:11 |  |  | 0.770 |
| ns | 326 |  | 78 | main.go imports + go:embed VERSION | 1.4 | 1.2 | 0.692 |
| walker |  | 339 | 19 | go decl names surface in cmd/platforms.go |  |  | 0.692 |
| walker |  | 339 | 0 | go decl at cmd/platforms.go:7 |  |  | 0.692 |
| ns | 372 |  | 46 | cmd/ directory listing | 2.1 |  | 0.705 |
| walker |  | 394 | 55 | headings outline in README.md |  |  | 0.763 |
| walker |  | 435 | 41 | listing of 'pkg' |  |  | 0.768 |
| walker |  | 468 | 33 | listing of 'pkg/lookpath' |  |  | 0.768 |
| ns | 483 |  | 111 | cmd.Execute entry-point function | 2.2 |  | 0.697 |
| walker |  | 490 | 22 | listing of 'pkg/exprparser' |  |  | 0.698 |
| walker |  | 514 | 24 | listing of 'pkg/artifacts' |  |  | 0.698 |
| walker |  | 539 | 25 | listing of 'pkg/artifactcache' |  |  | 0.699 |
| walker |  | 595 | 56 | listing of 'pkg/model' |  |  | 0.701 |
| walker |  | 606 | 11 | go decl names surface in pkg/model/job_context.go |  |  | 0.701 |
| walker |  | 671 | 65 | headings outline in IMAGES.md |  |  | 0.701 |
| walker |  | 687 | 16 | listing of 'pkg/workflowpattern' |  |  | 0.701 |
| ns | 695 |  | 212 | createRootCommand Cobra shape | 2.3 |  | 0.645 |
| walker |  | 766 | 79 | listing of 'pkg/common' |  |  | 0.652 |
| walker |  | 780 | 14 | go decl names surface in pkg/common/outbound_ip.go |  |  | 0.652 |
| walker |  | 780 | 0 | go decl at pkg/common/outbound_ip.go:13 |  |  | 0.652 |
| walker |  | 812 | 32 | go decl names surface in cmd/dir.go |  |  | 0.652 |
| walker |  | 812 | 0 | go decl at cmd/dir.go:15 |  |  | 0.652 |
| walker |  | 821 | 9 | go decl at cmd/dir.go:10 |  |  | 0.652 |
| walker |  | 846 | 25 | README.md section #2 |  |  | 0.652 |
| walker |  | 903 | 57 | go package + imports in main.go |  |  | 0.705 |
| walker |  | 922 | 19 | listing of 'cmd/testdata' |  |  | 0.705 |
| walker |  | 941 | 19 | listing of 'pkg/schema' |  |  | 0.706 |
| walker |  | 966 | 25 | README.md section #3 |  |  | 0.706 |
| ns | 992 |  | 297 | Input struct (first half) — workflow/secrets/env/platform/container fields | 3.1 |  | 0.590 |
| walker |  | 1012 | 46 | go decl names surface in cmd/secrets.go |  |  | 0.590 |
| walker |  | 1012 | 0 | go decl at cmd/secrets.go:14 |  |  | 0.590 |
| walker |  | 1012 | 0 | go decl at cmd/secrets.go:40 |  |  | 0.590 |
| walker |  | 1017 | 5 | go decl body at cmd/secrets.go:40 |  |  | 0.590 |
| walker |  | 1043 | 26 | go decl names surface in pkg/lookpath/error.go |  |  | 0.590 |
| walker |  | 1043 | 0 | go decl at pkg/lookpath/error.go:8 |  |  | 0.590 |
| walker |  | 1061 | 18 | go decl at pkg/lookpath/error.go:3 |  |  | 0.590 |
| walker |  | 1068 | 7 | go decl body at pkg/lookpath/error.go:8 |  |  | 0.590 |
| walker |  | 1075 | 7 | go package + imports in pkg/common/cartesian.go |  |  | 0.590 |
| walker |  | 1082 | 7 | go package + imports in pkg/model/job_context.go |  |  | 0.590 |
| walker |  | 1090 | 8 | go package + imports in pkg/artifactcache/doc.go |  |  | 0.590 |
| walker |  | 1098 | 8 | go package + imports in pkg/artifactcache/model.go |  |  | 0.590 |
| walker |  | 1106 | 8 | go package + imports in pkg/lookpath/error.go |  |  | 0.590 |
| walker |  | 1266 | 160 | listing of 'pkg/container' |  |  | 0.596 |
| walker |  | 1278 | 12 | go decl names surface in pkg/container/executions_environment.go |  |  | 0.596 |
| ns | 1300 |  | 308 | Input struct (second half) — caps/servers/cache/matrix/network fields | 3.2 | 3.1 | 0.520 |
| walker |  | 1304 | 26 | go decl names surface in pkg/container/parse_env_file.go |  |  | 0.520 |
| walker |  | 1304 | 0 | go decl at pkg/container/parse_env_file.go:14 |  |  | 0.520 |
| walker |  | 1340 | 36 | go decl names surface in pkg/container/docker_network.go |  |  | 0.520 |
| walker |  | 1340 | 0 | go decl at pkg/container/docker_network.go:12 |  |  | 0.520 |
| walker |  | 1340 | 0 | go decl at pkg/container/docker_network.go:45 |  |  | 0.520 |
| ns | 1422 |  | 122 | Input path-resolving method names | 3.3 | 3.2 | 0.503 |
| walker |  | 1526 | 186 | listing of 'pkg/runner' |  |  | 0.511 |
| walker |  | 1564 | 38 | go decl names surface in pkg/artifactcache/model.go |  |  | 0.511 |
| walker |  | 1564 | 0 | go decl at pkg/artifactcache/model.go:9 |  |  | 0.511 |
| walker |  | 1610 | 46 | IMAGES.md section #0 |  |  | 0.511 |
| walker |  | 1650 | 40 | go decl names surface in pkg/common/file.go |  |  | 0.511 |
| walker |  | 1650 | 0 | go decl at pkg/common/file.go:10 |  |  | 0.511 |
| walker |  | 1650 | 0 | go decl at pkg/common/file.go:37 |  |  | 0.511 |
| walker |  | 1660 | 10 | go decl doc at pkg/common/file.go:10 |  |  | 0.511 |
| walker |  | 1672 | 12 | go decl doc at pkg/common/file.go:37 |  |  | 0.511 |
| walker |  | 1712 | 40 | go decl names surface in pkg/container/docker_volume.go |  |  | 0.511 |
| walker |  | 1712 | 0 | go decl at pkg/container/docker_volume.go:12 |  |  | 0.511 |
| walker |  | 1712 | 0 | go decl at pkg/container/docker_volume.go:36 |  |  | 0.511 |
| walker |  | 1753 | 41 | go decl names surface in pkg/common/cartesian.go |  |  | 0.511 |
| walker |  | 1753 | 0 | go decl at pkg/common/cartesian.go:4 |  |  | 0.511 |
| walker |  | 1753 | 0 | go decl at pkg/common/cartesian.go:25 |  |  | 0.511 |
| walker |  | 1774 | 21 | listing of 'pkg/model/testdata' |  |  | 0.511 |
| walker |  | 1819 | 45 | go decl names surface in pkg/model/anchors.go |  |  | 0.511 |
| walker |  | 1819 | 0 | go decl at pkg/model/anchors.go:9 |  |  | 0.511 |
| walker |  | 1819 | 0 | go decl at pkg/model/anchors.go:36 |  |  | 0.511 |
| walker |  | 1865 | 46 | go decl names surface in pkg/common/context.go |  |  | 0.511 |
| walker |  | 1865 | 0 | go decl at pkg/common/context.go:10 |  |  | 0.511 |
| walker |  | 1865 | 0 | go decl at pkg/common/context.go:42 |  |  | 0.511 |
| walker |  | 1911 | 46 | go decl names surface in pkg/container/util.go |  |  | 0.511 |
| walker |  | 1911 | 0 | go decl at pkg/container/util.go:12 |  |  | 0.511 |
| walker |  | 1911 | 0 | go decl at pkg/container/util.go:24 |  |  | 0.511 |
| ns | 1913 |  | 491 | Run-mode flags (root.go 68-82) — list/graph/job/secrets/env | 3.4 |  | 0.484 |
| walker |  | 1958 | 47 | go decl names surface in pkg/container/util_openbsd_mips64.go |  |  | 0.484 |
| walker |  | 1958 | 0 | go decl at pkg/container/util_openbsd_mips64.go:9 |  |  | 0.484 |
| walker |  | 1958 | 0 | go decl at pkg/container/util_openbsd_mips64.go:15 |  |  | 0.484 |
| walker |  | 2006 | 48 | go decl names surface in pkg/container/docker_auth.go |  |  | 0.484 |
| walker |  | 2006 | 0 | go decl at pkg/container/docker_auth.go:15 |  |  | 0.484 |
| walker |  | 2006 | 0 | go decl at pkg/container/docker_auth.go:42 |  |  | 0.484 |
| walker |  | 2024 | 18 | go decl doc at pkg/common/cartesian.go:4 |  |  | 0.484 |
| walker |  | 2067 | 43 | go decl at pkg/artifactcache/model.go:3 |  |  | 0.484 |
| walker |  | 2108 | 41 | go decl body at main.go:13 |  |  | 0.545 |
| walker |  | 2160 | 52 | go decl names surface in pkg/container/docker_build.go |  |  | 0.545 |
| walker |  | 2160 | 0 | go decl at pkg/container/docker_build.go:23 |  |  | 0.545 |
| walker |  | 2160 | 0 | go decl at pkg/container/docker_build.go:78 |  |  | 0.545 |
| walker |  | 2185 | 25 | go package + imports in cmd/platforms.go |  |  | 0.545 |
| walker |  | 2204 | 19 | go decl doc at pkg/container/docker_build.go:23 |  |  | 0.545 |
| walker |  | 2211 | 7 | go decl body at pkg/container/util.go:24 |  |  | 0.545 |
| walker |  | 2267 | 56 | go decl names surface in pkg/runner/step_factory.go |  |  | 0.545 |
| walker |  | 2267 | 0 | go decl at pkg/runner/step_factory.go:15 |  |  | 0.545 |
| walker |  | 2555 | 288 | README.md section #0 |  |  | 0.545 |
| ns | 2579 |  | 666 | Run-mode flags (root.go 83-98) — bind/pull/event/privileged/caps | 3.5 | 3.4 | 0.517 |
| walker |  | 2612 | 57 | go decl names surface in pkg/lookpath/env.go |  |  | 0.517 |
| walker |  | 2612 | 0 | go decl at pkg/lookpath/env.go:12 |  |  | 0.517 |
| walker |  | 2612 | 0 | go decl at pkg/lookpath/env.go:16 |  |  | 0.517 |
| walker |  | 2615 | 3 | go decl at pkg/lookpath/env.go:9 |  |  | 0.517 |
| walker |  | 2629 | 14 | go decl at pkg/lookpath/env.go:5 |  |  | 0.517 |
| walker |  | 2641 | 12 | go decl body at pkg/lookpath/env.go:16 |  |  | 0.517 |
| walker |  | 2656 | 15 | go package + imports in pkg/container/executions_environment.go |  |  | 0.517 |
| walker |  | 2671 | 15 | go package + imports in pkg/model/step_result.go |  |  | 0.517 |
| walker |  | 2729 | 58 | go decl names surface in pkg/container/docker_images.go |  |  | 0.517 |
| walker |  | 2729 | 0 | go decl at pkg/container/docker_images.go:16 |  |  | 0.517 |
| walker |  | 2729 | 0 | go decl at pkg/container/docker_images.go:44 |  |  | 0.517 |
| walker |  | 2736 | 7 | go decl body at pkg/lookpath/env.go:12 |  |  | 0.517 |
| walker |  | 2739 | 3 | listing of 'pkg/artifactcache/testdata' |  |  | 0.517 |
| walker |  | 2755 | 16 | go package + imports in pkg/lookpath/env.go |  |  | 0.517 |
| walker |  | 2771 | 16 | go package + imports in pkg/workflowpattern/trace_writer.go |  |  | 0.517 |
| walker |  | 2841 | 70 | go decl names surface in pkg/common/dryrun.go |  |  | 0.517 |
| walker |  | 2841 | 0 | go decl at pkg/common/dryrun.go:12 |  |  | 0.517 |
| walker |  | 2841 | 0 | go decl at pkg/common/dryrun.go:23 |  |  | 0.517 |
| walker |  | 2858 | 17 | go decl doc at pkg/common/dryrun.go:12 |  |  | 0.517 |
| walker |  | 2876 | 18 | go decl doc at pkg/common/dryrun.go:23 |  |  | 0.517 |
| walker |  | 2893 | 17 | go decl body at pkg/common/dryrun.go:23 |  |  | 0.517 |
| walker |  | 2963 | 70 | go decl names surface in pkg/container/docker_pull.go |  |  | 0.517 |
| walker |  | 2963 | 0 | go decl at pkg/container/docker_pull.go:21 |  |  | 0.517 |
| walker |  | 2963 | 0 | go decl at pkg/container/docker_pull.go:78 |  |  | 0.517 |
| walker |  | 2963 | 0 | go decl at pkg/container/docker_pull.go:123 |  |  | 0.517 |
| walker |  | 2982 | 19 | go decl doc at pkg/container/docker_pull.go:21 |  |  | 0.517 |
| walker |  | 3054 | 72 | go decl names surface in pkg/common/logger.go |  |  | 0.517 |
| walker |  | 3054 | 0 | go decl at pkg/common/logger.go:14 |  |  | 0.517 |
| walker |  | 3054 | 0 | go decl at pkg/common/logger.go:25 |  |  | 0.517 |
| walker |  | 3068 | 14 | go decl doc at pkg/common/logger.go:14 |  |  | 0.517 |
| walker |  | 3085 | 17 | go decl doc at pkg/common/logger.go:25 |  |  | 0.517 |
| walker |  | 3100 | 15 | go decl body at pkg/common/logger.go:25 |  |  | 0.517 |
| walker |  | 3239 | 139 | go decl names surface in cmd/notices.go |  |  | 0.517 |
| walker |  | 3239 | 0 | go decl at cmd/notices.go:22 |  |  | 0.517 |
| walker |  | 3239 | 0 | go decl at cmd/notices.go:57 |  |  | 0.517 |
| walker |  | 3239 | 0 | go decl at cmd/notices.go:65 |  |  | 0.517 |
| walker |  | 3239 | 0 | go decl at cmd/notices.go:121 |  |  | 0.517 |
| walker |  | 3239 | 0 | go decl at cmd/notices.go:130 |  |  | 0.517 |
| walker |  | 3239 | 0 | go decl at cmd/notices.go:138 |  |  | 0.517 |
| walker |  | 3266 | 27 | go decl at cmd/notices.go:17 |  |  | 0.517 |
| walker |  | 3303 | 37 | listing of 'pkg/container/testdata' |  |  | 0.517 |
| ns | 3356 |  | 777 | Persistent flags (root.go 99-117) — workflows path / logging / env files / container | 3.6 | 3.5 | 0.488 |
| walker |  | 3380 | 77 | go decl names surface in pkg/workflowpattern/trace_writer.go |  |  | 0.488 |
| walker |  | 3380 | 0 | go decl at pkg/workflowpattern/trace_writer.go:16 |  |  | 0.488 |
| walker |  | 3394 | 14 | go decl at pkg/workflowpattern/trace_writer.go:5 |  |  | 0.488 |
| walker |  | 3404 | 10 | go decl body at pkg/workflowpattern/trace_writer.go:16 |  |  | 0.488 |
| walker |  | 3428 | 24 | go decl at pkg/runner/step_factory.go:9 |  |  | 0.488 |
| walker |  | 3432 | 4 | listing of 'pkg/runner/hashfiles' |  |  | 0.488 |
| walker |  | 3436 | 4 | listing of 'pkg/runner/res' |  |  | 0.488 |
| walker |  | 3460 | 24 | listing of 'pkg/model/testdata/invalid-job-name' |  |  | 0.488 |
| walker |  | 3608 | 148 | go decl names surface in cmd/input.go |  |  | 0.523 |
| walker |  | 3608 | 0 | go decl at cmd/input.go:70 |  |  | 0.523 |
| walker |  | 3608 | 0 | go decl at cmd/input.go:85 |  |  | 0.523 |
| walker |  | 3608 | 0 | go decl at cmd/input.go:90 |  |  | 0.523 |
| walker |  | 3608 | 0 | go decl at cmd/input.go:94 |  |  | 0.523 |
| walker |  | 3608 | 0 | go decl at cmd/input.go:99 |  |  | 0.523 |
| walker |  | 3608 | 0 | go decl at cmd/input.go:104 |  |  | 0.523 |
| walker |  | 3608 | 0 | go decl at cmd/input.go:109 |  |  | 0.523 |
| walker |  | 3608 | 0 | go decl at cmd/input.go:114 |  |  | 0.523 |
| walker |  | 3615 | 7 | go decl body at cmd/input.go:99 |  |  | 0.523 |
| walker |  | 3624 | 9 | go decl body at cmd/input.go:85 |  |  | 0.523 |
| walker |  | 3633 | 9 | go decl body at cmd/input.go:90 |  |  | 0.523 |
| walker |  | 3642 | 9 | go decl body at cmd/input.go:94 |  |  | 0.523 |
| walker |  | 3651 | 9 | go decl body at cmd/input.go:109 |  |  | 0.523 |
| walker |  | 3660 | 9 | go decl body at cmd/input.go:114 |  |  | 0.523 |
| walker |  | 3670 | 10 | go decl body at cmd/input.go:104 |  |  | 0.523 |
| walker |  | 3682 | 12 | go decl doc at cmd/input.go:90 |  |  | 0.523 |
| walker |  | 3695 | 13 | go decl doc at cmd/input.go:85 |  |  | 0.523 |
| walker |  | 3708 | 13 | go decl doc at cmd/input.go:99 |  |  | 0.523 |
| walker |  | 3722 | 14 | go decl doc at cmd/input.go:109 |  |  | 0.523 |
| walker |  | 3737 | 15 | go decl doc at cmd/input.go:104 |  |  | 0.523 |
| walker |  | 3752 | 15 | go decl doc at cmd/input.go:114 |  |  | 0.523 |
| walker |  | 3835 | 83 | go decl names surface in pkg/container/docker_socket.go |  |  | 0.523 |
| walker |  | 3835 | 0 | go decl at pkg/container/docker_socket.go:23 |  |  | 0.523 |
| walker |  | 3835 | 0 | go decl at pkg/container/docker_socket.go:45 |  |  | 0.523 |
| walker |  | 3835 | 0 | go decl at pkg/container/docker_socket.go:62 |  |  | 0.523 |
| walker |  | 3855 | 20 | go decl at pkg/container/docker_socket.go:57 |  |  | 0.523 |
| walker |  | 3885 | 30 | go decl doc at pkg/container/docker_images.go:44 |  |  | 0.523 |
| walker |  | 3970 | 85 | go decl names surface in pkg/runner/local_repository_cache.go |  |  | 0.523 |
| walker |  | 3970 | 0 | go decl at pkg/runner/local_repository_cache.go:25 |  |  | 0.523 |
| walker |  | 3970 | 0 | go decl at pkg/runner/local_repository_cache.go:44 |  |  | 0.523 |
| walker |  | 4006 | 36 | go decl at pkg/runner/local_repository_cache.go:19 |  |  | 0.523 |
| ns | 4057 |  | 701 | Server/cache flags + SetArgs (root.go 118-135) | 3.7 | 3.6 | 0.498 |
| walker |  | 4092 | 86 | go decl names surface in pkg/container/docker_logger.go |  |  | 0.498 |
| walker |  | 4092 | 0 | go decl at pkg/container/docker_logger.go:27 |  |  | 0.498 |
| walker |  | 4092 | 0 | go decl at pkg/container/docker_logger.go:77 |  |  | 0.498 |
| walker |  | 4134 | 42 | go package + imports in cmd/input.go |  |  | 0.498 |
| walker |  | 4224 | 90 | go decl names surface in pkg/common/line_writer.go |  |  | 0.498 |
| walker |  | 4224 | 0 | go decl at pkg/common/line_writer.go:9 |  |  | 0.498 |
| walker |  | 4224 | 0 | go decl at pkg/common/line_writer.go:17 |  |  | 0.498 |
| walker |  | 4224 | 0 | go decl at pkg/common/line_writer.go:23 |  |  | 0.498 |
| walker |  | 4224 | 0 | go decl at pkg/common/line_writer.go:43 |  |  | 0.498 |
| walker |  | 4240 | 16 | go decl doc at pkg/common/line_writer.go:9 |  |  | 0.498 |
| walker |  | 4257 | 17 | go decl doc at pkg/common/line_writer.go:17 |  |  | 0.498 |
| walker |  | 4279 | 22 | go decl at pkg/common/line_writer.go:11 |  |  | 0.498 |
| walker |  | 4371 | 92 | go decl names surface in pkg/runner/action_cache_offline_mode.go |  |  | 0.498 |
| walker |  | 4371 | 0 | go decl at pkg/runner/action_cache_offline_mode.go:17 |  |  | 0.498 |
| walker |  | 4371 | 0 | go decl at pkg/runner/action_cache_offline_mode.go:45 |  |  | 0.498 |
| walker |  | 4385 | 14 | go decl at pkg/runner/action_cache_offline_mode.go:13 |  |  | 0.498 |
| walker |  | 4403 | 18 | go decl body at pkg/runner/action_cache_offline_mode.go:45 |  |  | 0.498 |
| ns | 4409 |  | 352 | configLocations + args (.actrc precedence + merge) | 3.8 |  | 0.463 |
| walker |  | 4427 | 24 | go decl body at pkg/common/context.go:42 |  |  | 0.463 |
| ns | 4450 |  | 41 | pkg/ subdirectory listing | 4.1 |  | 0.490 |
| walker |  | 4451 | 24 | go decl body at pkg/common/line_writer.go:17 |  |  | 0.490 |
| walker |  | 4476 | 25 | go package + imports in pkg/common/dryrun.go |  |  | 0.490 |
| walker |  | 4501 | 25 | go package + imports in pkg/common/job_error.go |  |  | 0.490 |
| walker |  | 4516 | 15 | go decl doc at pkg/container/docker_socket.go:23 |  |  | 0.490 |
| walker |  | 4525 | 9 | listing of 'pkg/gh' |  |  | 0.490 |
| walker |  | 4548 | 23 | go decl names surface in pkg/gh/gh.go |  |  | 0.490 |
| walker |  | 4548 | 0 | go decl at pkg/gh/gh.go:10 |  |  | 0.490 |
| ns | 4636 |  | 186 | pkg/runner file listing | 4.2 |  | 0.547 |
| walker |  | 4650 | 102 | go decl names surface in pkg/container/container_types.go |  |  | 0.547 |
| walker |  | 4659 | 9 | go decl at pkg/container/container_types.go:80 |  |  | 0.547 |
| walker |  | 4686 | 27 | go decl at pkg/container/container_types.go:36 |  |  | 0.547 |
| walker |  | 4702 | 16 | go decl doc at pkg/container/container_types.go:36 |  |  | 0.547 |
| walker |  | 4749 | 47 | go decl at pkg/container/container_types.go:70 |  |  | 0.547 |
| walker |  | 4769 | 20 | go decl doc at pkg/container/container_types.go:70 |  |  | 0.547 |
| ns | 4771 |  | 135 | pkg/model + pkg/common file listings | 4.3 |  | 0.581 |
| walker |  | 4819 | 50 | go decl at pkg/container/container_types.go:61 |  |  | 0.581 |
| walker |  | 4839 | 20 | go decl doc at pkg/container/container_types.go:61 |  |  | 0.581 |
| walker |  | 4888 | 49 | go package + imports in cmd/dir.go |  |  | 0.581 |
| walker |  | 4906 | 18 | listing of 'pkg/artifacts/testdata' |  |  | 0.581 |
| ns | 4931 |  | 160 | pkg/container file listing | 4.4 |  | 0.606 |
| walker |  | 4944 | 38 | go decl doc at pkg/container/docker_images.go:16 |  |  | 0.606 |
| walker |  | 4956 | 12 | go decl body at pkg/container/util_openbsd_mips64.go:15 |  |  | 0.606 |
| walker |  | 5068 | 112 | go decl names surface in pkg/common/auth.go |  |  | 0.606 |
| walker |  | 5068 | 0 | go decl at pkg/common/auth.go:38 |  |  | 0.606 |
| walker |  | 5068 | 0 | go decl at pkg/common/auth.go:72 |  |  | 0.606 |
| walker |  | 5077 | 9 | go decl at pkg/common/auth.go:33 |  |  | 0.606 |
| walker |  | 5099 | 22 | go decl at pkg/common/auth.go:26 |  |  | 0.606 |
| ns | 5145 |  | 214 | Executor + Conditional type defs + Warning helper | 5.1 |  | 0.585 |
| walker |  | 5153 | 54 | go package + imports in cmd/list.go |  |  | 0.585 |
| ns | 5208 |  | 63 | Executor constructor signatures | 5.2 | 5.1 | 0.579 |
| ns | 5256 |  | 48 | Executor chaining method signatures (.Then/.Finally/.If/...) | 5.3 | 5.1 | 0.572 |
| walker |  | 5269 | 116 | go decl names surface in pkg/runner/job_executor.go |  |  | 0.572 |
| walker |  | 5269 | 0 | go decl at pkg/runner/job_executor.go:23 |  |  | 0.572 |
| walker |  | 5269 | 0 | go decl at pkg/runner/job_executor.go:157 |  |  | 0.572 |
| walker |  | 5269 | 0 | go decl at pkg/runner/job_executor.go:185 |  |  | 0.572 |
| walker |  | 5269 | 0 | go decl at pkg/runner/job_executor.go:200 |  |  | 0.572 |
| walker |  | 5280 | 11 | go decl doc at pkg/runner/job_executor.go:23 |  |  | 0.572 |
| walker |  | 5335 | 55 | go package + imports in cmd/graph.go |  |  | 0.572 |
| ns | 5398 |  | 142 | NewPipelineExecutor body (canonical fold-via-.Then example) | 5.4 | 5.2 | 0.559 |
| ns | 5744 |  | 346 | Plan / Stage / Run / WorkflowPlanner types | 6.1 |  | 0.532 |
| ns | 6127 |  | 383 | Workflow struct + Job struct (YAML schema fields) | 6.2 |  | 0.514 |
| walker |  | 6145 | 810 | go module file go.mod |  |  | 0.515 |
| walker |  | 6265 | 120 | go decl names surface in pkg/workflowpattern/workflow_pattern.go |  |  | 0.515 |
| walker |  | 6265 | 0 | go decl at pkg/workflowpattern/workflow_pattern.go:15 |  |  | 0.515 |
| walker |  | 6265 | 0 | go decl at pkg/workflowpattern/workflow_pattern.go:38 |  |  | 0.515 |
| walker |  | 6265 | 0 | go decl at pkg/workflowpattern/workflow_pattern.go:138 |  |  | 0.515 |
| walker |  | 6265 | 0 | go decl at pkg/workflowpattern/workflow_pattern.go:151 |  |  | 0.515 |
| walker |  | 6265 | 0 | go decl at pkg/workflowpattern/workflow_pattern.go:177 |  |  | 0.515 |
| walker |  | 6277 | 12 | go decl doc at pkg/workflowpattern/workflow_pattern.go:38 |  |  | 0.515 |
| walker |  | 6309 | 32 | go decl at pkg/workflowpattern/workflow_pattern.go:9 |  |  | 0.515 |
| walker |  | 6326 | 17 | go decl doc at pkg/workflowpattern/workflow_pattern.go:151 |  |  | 0.515 |
| walker |  | 6345 | 19 | go decl doc at pkg/workflowpattern/workflow_pattern.go:177 |  |  | 0.515 |
| walker |  | 6377 | 32 | go package + imports in pkg/common/line_writer.go |  |  | 0.515 |
| walker |  | 6506 | 129 | go decl names surface in pkg/model/step_result.go |  |  | 0.515 |
| walker |  | 6506 | 0 | go decl at pkg/model/step_result.go:19 |  |  | 0.515 |
| walker |  | 6506 | 0 | go decl at pkg/model/step_result.go:23 |  |  | 0.515 |
| walker |  | 6506 | 0 | go decl at pkg/model/step_result.go:34 |  |  | 0.515 |
| walker |  | 6515 | 9 | go decl at pkg/model/step_result.go:7 |  |  | 0.515 |
| walker |  | 6564 | 49 | go decl at pkg/model/step_result.go:41 |  |  | 0.515 |
| walker |  | 6574 | 10 | go decl body at pkg/model/step_result.go:19 |  |  | 0.515 |
| walker |  | 6597 | 23 | listing of 'pkg/exprparser/testdata' |  |  | 0.515 |
| walker |  | 6601 | 4 | listing of 'pkg/artifacts/testdata/GHSL-2023-004' |  |  | 0.515 |
| walker |  | 6605 | 4 | listing of 'pkg/artifacts/testdata/upload-and-download' |  |  | 0.515 |
| walker |  | 6609 | 4 | listing of 'pkg/artifacts/testdata/v4' |  |  | 0.515 |
| walker |  | 6613 | 4 | listing of 'pkg/container/testdata/docker-pull-options' |  |  | 0.515 |
| walker |  | 6617 | 4 | listing of 'pkg/container/testdata/scratch' |  |  | 0.515 |
| walker |  | 6621 | 4 | listing of 'pkg/model/testdata/container-volumes' |  |  | 0.515 |
| walker |  | 6625 | 4 | listing of 'pkg/model/testdata/empty-workflow' |  |  | 0.515 |
| walker |  | 6629 | 4 | listing of 'pkg/model/testdata/strategy' |  |  | 0.515 |
| ns | 6678 |  | 551 | Action runtime constants + Action struct + Input/Output | 6.3 |  | 0.492 |
| walker |  | 6723 | 94 | go decl at pkg/model/job_context.go:3 |  |  | 0.492 |
| walker |  | 6790 | 67 | go package + imports in cmd/secrets.go |  |  | 0.492 |
| walker |  | 6803 | 13 | listing of 'pkg/filecollector' |  |  | 0.493 |
| walker |  | 6905 | 102 | README.md section #4 |  |  | 0.493 |
| ns | 6906 |  | 228 | ActionRuns struct (action-execution fields) | 6.4 | 6.3 | 0.484 |
| walker |  | 6944 | 39 | go package + imports in pkg/common/file.go |  |  | 0.484 |
| walker |  | 6983 | 39 | go package + imports in pkg/common/logger.go |  |  | 0.484 |
| walker |  | 7023 | 40 | go package + imports in pkg/common/outbound_ip.go |  |  | 0.484 |
| walker |  | 7063 | 40 | go package + imports in pkg/model/anchors.go |  |  | 0.484 |
| walker |  | 7103 | 40 | go package + imports in pkg/runner/step_factory.go |  |  | 0.484 |
| walker |  | 7144 | 41 | go package + imports in pkg/container/util_openbsd_mips64.go |  |  | 0.484 |
| walker |  | 7185 | 41 | go package + imports in pkg/workflowpattern/workflow_pattern.go |  |  | 0.484 |
| ns | 7205 |  | 299 | Workflow.On RawOn dispatcher (yaml.Node polymorphism pattern) | 6.5 | 6.2 | 0.468 |
| walker |  | 7344 | 159 | go decl names surface in pkg/container/linux_container_environment_extensions.go |  |  | 0.468 |
| walker |  | 7344 | 0 | go decl at pkg/container/linux_container_environment_extensions.go:19 |  |  | 0.468 |
| walker |  | 7344 | 0 | go decl at pkg/container/linux_container_environment_extensions.go:50 |  |  | 0.468 |
| walker |  | 7344 | 0 | go decl at pkg/container/linux_container_environment_extensions.go:54 |  |  | 0.468 |
| walker |  | 7344 | 0 | go decl at pkg/container/linux_container_environment_extensions.go:58 |  |  | 0.468 |
| walker |  | 7344 | 0 | go decl at pkg/container/linux_container_environment_extensions.go:62 |  |  | 0.468 |
| walker |  | 7344 | 0 | go decl at pkg/container/linux_container_environment_extensions.go:66 |  |  | 0.468 |
| walker |  | 7344 | 0 | go decl at pkg/container/linux_container_environment_extensions.go:75 |  |  | 0.468 |
| walker |  | 7347 | 3 | go decl at pkg/container/linux_container_environment_extensions.go:13 |  |  | 0.468 |
| walker |  | 7352 | 5 | go decl body at pkg/container/linux_container_environment_extensions.go:75 |  |  | 0.468 |
| walker |  | 7358 | 6 | go decl body at pkg/container/linux_container_environment_extensions.go:54 |  |  | 0.468 |
| walker |  | 7367 | 9 | go decl body at pkg/container/linux_container_environment_extensions.go:50 |  |  | 0.468 |
| walker |  | 7376 | 9 | go decl body at pkg/container/linux_container_environment_extensions.go:62 |  |  | 0.468 |
| walker |  | 7401 | 25 | go decl body at pkg/container/linux_container_environment_extensions.go:58 |  |  | 0.468 |
| ns | 7519 |  | 314 | WorkflowDispatch / WorkflowCall input+output types | 6.6 | 6.2 | 0.454 |
| walker |  | 7564 | 163 | go decl names surface in pkg/runner/action_composite.go |  |  | 0.454 |
| walker |  | 7564 | 0 | go decl at pkg/runner/action_composite.go:13 |  |  | 0.454 |
| walker |  | 7564 | 0 | go decl at pkg/runner/action_composite.go:47 |  |  | 0.454 |
| walker |  | 7564 | 0 | go decl at pkg/runner/action_composite.go:84 |  |  | 0.454 |
| walker |  | 7564 | 0 | go decl at pkg/runner/action_composite.go:133 |  |  | 0.454 |
| walker |  | 7564 | 0 | go decl at pkg/runner/action_composite.go:197 |  |  | 0.454 |
| walker |  | 7564 | 0 | go decl at pkg/runner/action_composite.go:222 |  |  | 0.454 |
| walker |  | 7592 | 28 | go decl at pkg/runner/action_composite.go:126 |  |  | 0.454 |
| walker |  | 7610 | 18 | go decl doc at pkg/runner/action_composite.go:133 |  |  | 0.454 |
| ns | 7715 |  | 196 | Strategy / Defaults / RunDefaults structs | 6.7 | 6.2 | 0.446 |
| walker |  | 7718 | 108 | go decl at pkg/container/executions_environment.go:5 |  |  | 0.446 |
| walker |  | 7735 | 17 | go decl body at pkg/model/anchors.go:36 |  |  | 0.446 |
| walker |  | 7762 | 27 | go decl body at cmd/notices.go:57 |  |  | 0.446 |
| ns | 7771 |  | 56 | WorkflowPlanner constructor signature | 6.8 |  | 0.445 |
| walker |  | 7790 | 28 | go decl at pkg/model/step_result.go:13 |  |  | 0.445 |
| walker |  | 7836 | 46 | go package + imports in pkg/common/draw.go |  |  | 0.445 |
| walker |  | 8015 | 179 | go decl names surface in pkg/common/job_error.go |  |  | 0.445 |
| walker |  | 8015 | 0 | go decl at pkg/common/job_error.go:16 |  |  | 0.445 |
| walker |  | 8015 | 0 | go decl at pkg/common/job_error.go:26 |  |  | 0.445 |
| walker |  | 8015 | 0 | go decl at pkg/common/job_error.go:31 |  |  | 0.445 |
| walker |  | 8015 | 0 | go decl at pkg/common/job_error.go:36 |  |  | 0.445 |
| walker |  | 8015 | 0 | go decl at pkg/common/job_error.go:40 |  |  | 0.445 |
| walker |  | 8015 | 0 | go decl at pkg/common/job_error.go:51 |  |  | 0.445 |
| walker |  | 8032 | 17 | go decl doc at pkg/common/job_error.go:16 |  |  | 0.445 |
| walker |  | 8048 | 16 | go decl body at pkg/common/job_error.go:36 |  |  | 0.445 |
| walker |  | 8070 | 22 | go decl doc at pkg/common/job_error.go:31 |  |  | 0.445 |
| walker |  | 8098 | 28 | go decl doc at pkg/common/job_error.go:51 |  |  | 0.445 |
| walker |  | 8119 | 21 | go decl body at pkg/common/job_error.go:26 |  |  | 0.445 |
| ns | 8123 |  | 352 | Runner interface + step interface | 7.1 |  | 0.430 |
| walker |  | 8146 | 27 | go decl body at pkg/common/job_error.go:31 |  |  | 0.430 |
| walker |  | 8193 | 47 | go package + imports in pkg/container/util.go |  |  | 0.430 |
| walker |  | 8202 | 9 | listing of 'pkg/common/git' |  |  | 0.430 |
| walker |  | 8262 | 60 | go decl doc at pkg/container/linux_container_environment_extensions.go:19 |  |  | 0.430 |
| walker |  | 8402 | 140 | go decl names surface in pkg/runner/run_context.go |  |  | 0.430 |
| walker |  | 8402 | 0 | go decl at pkg/runner/run_context.go:58 |  |  | 0.430 |
| walker |  | 8402 | 0 | go decl at pkg/runner/run_context.go:67 |  |  | 0.430 |
| walker |  | 8402 | 0 | go decl at pkg/runner/run_context.go:78 |  |  | 0.430 |
| walker |  | 8402 | 0 | go decl at pkg/runner/run_context.go:92 |  |  | 0.430 |
| walker |  | 8402 | 0 | go decl at pkg/runner/run_context.go:98 |  |  | 0.430 |
| walker |  | 8402 | 0 | go decl at pkg/runner/run_context.go:108 |  |  | 0.430 |
| walker |  | 8423 | 21 | go decl at pkg/runner/run_context.go:62 |  |  | 0.430 |
| ns | 8434 |  | 311 | RunContext struct (per-job execution state) | 7.2 |  | 0.421 |
| walker |  | 8437 | 14 | go decl doc at pkg/runner/run_context.go:78 |  |  | 0.421 |
| walker |  | 8450 | 13 | go decl body at pkg/runner/run_context.go:58 |  |  | 0.421 |
| walker |  | 8462 | 12 | go decl body at pkg/runner/run_context.go:92 |  |  | 0.421 |
| walker |  | 8512 | 50 | go package + imports in pkg/common/context.go |  |  | 0.421 |
| walker |  | 8562 | 50 | go package + imports in pkg/gh/gh.go |  |  | 0.421 |
| walker |  | 8615 | 53 | go package + imports in pkg/container/docker_network.go |  |  | 0.421 |
| walker |  | 8668 | 53 | go package + imports in pkg/container/docker_volume.go |  |  | 0.421 |
| ns | 8818 |  | 384 | Runner Config struct (first half — basic fields) | 7.3 |  | 0.413 |
| walker |  | 9040 | 372 | go decl names surface in cmd/root.go |  |  | 0.415 |
| walker |  | 9040 | 0 | go decl at cmd/root.go:47 |  |  | 0.415 |
| walker |  | 9040 | 0 | go decl at cmd/root.go:56 |  |  | 0.415 |
| walker |  | 9040 | 0 | go decl at cmd/root.go:138 |  |  | 0.415 |
| walker |  | 9040 | 0 | go decl at cmd/root.go:155 |  |  | 0.415 |
| walker |  | 9040 | 0 | go decl at cmd/root.go:167 |  |  | 0.415 |
| walker |  | 9040 | 0 | go decl at cmd/root.go:256 |  |  | 0.415 |
| walker |  | 9040 | 0 | go decl at cmd/root.go:268 |  |  | 0.415 |
| walker |  | 9040 | 0 | go decl at cmd/root.go:278 |  |  | 0.415 |
| walker |  | 9040 | 0 | go decl at cmd/root.go:304 |  |  | 0.415 |
| walker |  | 9040 | 0 | go decl at cmd/root.go:314 |  |  | 0.415 |
| walker |  | 9040 | 0 | go decl at cmd/root.go:320 |  |  | 0.415 |
| walker |  | 9040 | 0 | go decl at cmd/root.go:333 |  |  | 0.415 |
| walker |  | 9040 | 0 | go decl at cmd/root.go:345 |  |  | 0.415 |
| walker |  | 9040 | 0 | go decl at cmd/root.go:349 |  |  | 0.415 |
| walker |  | 9040 | 0 | go decl at cmd/root.go:373 |  |  | 0.415 |
| walker |  | 9040 | 0 | go decl at cmd/root.go:391 |  |  | 0.415 |
| walker |  | 9040 | 0 | go decl at cmd/root.go:716 |  |  | 0.415 |
| walker |  | 9040 | 0 | go decl at cmd/root.go:759 |  |  | 0.415 |
| walker |  | 9055 | 15 | go decl doc at cmd/root.go:47 |  |  | 0.417 |
| walker |  | 9105 | 50 | go decl at cmd/root.go:37 |  |  | 0.417 |
| walker |  | 9117 | 12 | go decl doc at cmd/root.go:391 |  |  | 0.417 |
| walker |  | 9131 | 14 | go decl body at cmd/root.go:345 |  |  | 0.417 |
| walker |  | 9166 | 35 | go decl doc at cmd/root.go:138 |  |  | 0.418 |
| walker |  | 9220 | 54 | go decl body at cmd/root.go:47 |  |  | 0.431 |
| walker |  | 9250 | 30 | go decl body at cmd/root.go:314 |  |  | 0.431 |
| ns | 9263 |  | 445 | Runner Config struct (second half — container/server/cache fields) | 7.4 | 7.3 | 0.423 |
| walker |  | 9304 | 54 | go package + imports in pkg/common/executor.go |  |  | 0.423 |
| ns | 9383 |  | 120 | ExecutionsEnvironment interface (host vs docker boundary) | 8.1 |  | 0.432 |
| ns | 9495 |  | 112 | Container interface method signatures | 8.2 | 8.1 | 0.427 |
| walker |  | 9514 | 210 | go decl names surface in pkg/runner/runner.go |  |  | 0.427 |
| walker |  | 9514 | 0 | go decl at pkg/runner/runner.go:67 |  |  | 0.427 |
| walker |  | 9514 | 0 | go decl at pkg/runner/runner.go:91 |  |  | 0.427 |
| walker |  | 9514 | 0 | go decl at pkg/runner/runner.go:99 |  |  | 0.427 |
| walker |  | 9514 | 0 | go decl at pkg/runner/runner.go:122 |  |  | 0.427 |
| walker |  | 9514 | 0 | go decl at pkg/runner/runner.go:221 |  |  | 0.427 |
| walker |  | 9514 | 0 | go decl at pkg/runner/runner.go:234 |  |  | 0.427 |
| walker |  | 9514 | 0 | go decl at pkg/runner/runner.go:253 |  |  | 0.427 |
| walker |  | 9532 | 18 | go decl at pkg/runner/runner.go:17 |  |  | 0.427 |
| walker |  | 9543 | 11 | go decl doc at pkg/runner/runner.go:91 |  |  | 0.427 |
| walker |  | 9557 | 14 | go decl doc at pkg/runner/runner.go:17 |  |  | 0.428 |
| walker |  | 9570 | 13 | go decl at pkg/runner/runner.go:80 |  |  | 0.428 |
| walker |  | 9579 | 9 | go decl doc at pkg/runner/runner.go:122 |  |  | 0.428 |
| ns | 9603 |  | 108 | EvaluationEnvironment record (${{ }} context shape) | 9.1 |  | 0.425 |
| walker |  | 9612 | 33 | go decl body at pkg/runner/runner.go:91 |  |  | 0.425 |
| walker |  | 9654 | 42 | go decl at pkg/runner/runner.go:84 |  |  | 0.425 |
| ns | 9693 |  | 90 | pkg/exprparser + pkg/artifacts + pkg/artifactcache + pkg/schema listings | 10.1 |  | 0.440 |
| walker |  | 9730 | 76 | go decl doc at pkg/common/outbound_ip.go:13 |  |  | 0.440 |
| ns | 9764 |  | 71 | pkg/lookpath + pkg/workflowpattern + pkg/gh + pkg/filecollector listings | 10.2 |  | 0.449 |
| walker |  | 9944 | 214 | go decl names surface in pkg/container/docker_stub.go |  |  | 0.449 |
| walker |  | 9944 | 0 | go decl at pkg/container/docker_stub.go:16 |  |  | 0.449 |
| walker |  | 9944 | 0 | go decl at pkg/container/docker_stub.go:22 |  |  | 0.449 |
| walker |  | 9944 | 0 | go decl at pkg/container/docker_stub.go:27 |  |  | 0.449 |
| walker |  | 9944 | 0 | go decl at pkg/container/docker_stub.go:34 |  |  | 0.449 |
| walker |  | 9944 | 0 | go decl at pkg/container/docker_stub.go:41 |  |  | 0.449 |
| walker |  | 9944 | 0 | go decl at pkg/container/docker_stub.go:45 |  |  | 0.449 |
| walker |  | 9944 | 0 | go decl at pkg/container/docker_stub.go:49 |  |  | 0.449 |
| walker |  | 9944 | 0 | go decl at pkg/container/docker_stub.go:53 |  |  | 0.449 |
| walker |  | 9944 | 0 | go decl at pkg/container/docker_stub.go:59 |  |  | 0.449 |
| walker |  | 9944 | 0 | go decl at pkg/container/docker_stub.go:65 |  |  | 0.449 |
| walker |  | 9949 | 5 | go decl body at pkg/container/docker_stub.go:41 |  |  | 0.455 |
| ns | 9949 |  | 185 | go.mod module + direct dependency block header | 11.1 |  | 0.455 |
| walker |  | 9957 | 8 | go decl body at pkg/container/docker_stub.go:45 |  |  | 0.455 |
| walker |  | 9965 | 8 | go decl body at pkg/container/docker_stub.go:49 |  |  | 0.455 |
| walker |  | 9976 | 11 | go decl body at pkg/container/docker_stub.go:16 |  |  | 0.455 |
| walker |  | 9987 | 11 | go decl body at pkg/container/docker_stub.go:22 |  |  | 0.455 |
| ns | 9995 |  | 46 | Makefile pr + test target bodies | 11.2 |  | 0.454 |

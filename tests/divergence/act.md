Score(3000)=0.730 I=0.910 C=0.586 ns_rows≤3K=19/41 (reached=11 partial=1 missing=7)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 45 |  | 45 | Root directory listing — source and docs | 1.1 |  | 0.000 |
| walker |  | 125 | 125 | listing of '.' |  |  | 1.000 |
| ns | 125 |  | 80 | Root directory listing — tooling config and packaging | 1.2 | 1.1 | 1.000 |
| walker |  | 150 | 25 | go decl names surface in main.go |  |  | 1.000 |
| walker |  | 150 | 0 | go decl at main.go:11 |  |  | 1.000 |
| walker |  | 150 | 0 | go decl at main.go:13 |  |  | 1.000 |
| ns | 162 |  | 37 | go.mod — module + Go version | 1.3 |  | 0.931 |
| walker |  | 182 | 32 | go module identity in go.mod |  |  | 0.983 |
| walker |  | 192 | 10 | plaintext config VERSION |  |  | 0.984 |
| walker |  | 202 | 10 | go decl doc at main.go:11 |  |  | 0.985 |
| ns | 258 |  | 96 | Binary entrypoint (main.go) + VERSION | 1.4 |  | 0.856 |
| walker |  | 263 | 61 | README headline in README.md |  |  | 0.856 |
| ns | 280 |  | 22 | pkg/exprparser/ directory listing | 2.1 |  | 0.805 |
| walker |  | 304 | 41 | listing of 'pkg' |  |  | 0.805 |
| walker |  | 320 | 16 | listing of 'pkg/workflowpattern' |  |  | 0.805 |
| walker |  | 329 | 9 | listing of 'pkg/gh' |  |  | 0.717 |
| ns | 329 |  | 49 | pkg/artifacts/ and pkg/artifactcache/ directory listings | 2.2 |  | 0.717 |
| walker |  | 351 | 22 | listing of 'pkg/exprparser' |  |  | 0.774 |
| walker |  | 375 | 24 | listing of 'pkg/artifacts' |  |  | 0.799 |
| ns | 385 |  | 56 | pkg/model/ directory listing | 2.3 |  | 0.719 |
| walker |  | 400 | 25 | listing of 'pkg/artifactcache' |  |  | 0.801 |
| walker |  | 403 | 3 | listing of 'pkg/artifactcache/testdata' |  |  | 0.801 |
| walker |  | 416 | 13 | listing of 'pkg/filecollector' |  |  | 0.802 |
| walker |  | 420 | 4 | listing of 'pkg/artifactcache/testdata/example' |  |  | 0.802 |
| ns | 450 |  | 65 | cmd/ directory listing | 2.4 |  | 0.719 |
| walker |  | 453 | 33 | listing of 'pkg/lookpath' |  |  | 0.724 |
| walker |  | 472 | 19 | listing of 'pkg/schema' |  |  | 0.729 |
| walker |  | 518 | 46 | listing of 'cmd' |  |  | 0.791 |
| walker |  | 537 | 19 | listing of 'cmd/testdata' |  |  | 0.843 |
| ns | 538 |  | 88 | pkg/common/ and pkg/common/git/ directory listing | 2.5 |  | 0.752 |
| walker |  | 545 | 8 | listing of '.vscode' |  |  | 0.752 |
| walker |  | 601 | 56 | listing of 'pkg/model' |  |  | 0.830 |
| walker |  | 619 | 18 | listing of 'pkg/artifacts/testdata' |  |  | 0.830 |
| walker |  | 623 | 4 | listing of 'pkg/artifacts/testdata/GHSL-2023-004' |  |  | 0.830 |
| walker |  | 627 | 4 | listing of 'pkg/artifacts/testdata/upload-and-download' |  |  | 0.830 |
| ns | 628 |  | 90 | gh/, lookpath/, filecollector/, workflowpattern/, schema/ directory listings | 2.6 |  | 0.840 |
| walker |  | 631 | 4 | listing of 'pkg/artifacts/testdata/v4' |  |  | 0.840 |
| walker |  | 649 | 18 | go decl names surface in cmd/graph.go |  |  | 0.840 |
| walker |  | 649 | 0 | go decl at cmd/graph.go:10 |  |  | 0.840 |
| walker |  | 667 | 18 | go decl names surface in cmd/list.go |  |  | 0.840 |
| walker |  | 667 | 0 | go decl at cmd/list.go:11 |  |  | 0.840 |
| walker |  | 746 | 79 | listing of 'pkg/common' |  |  | 0.917 |
| walker |  | 755 | 9 | listing of 'pkg/common/git' |  |  | 0.935 |
| ns | 788 |  | 160 | pkg/container/ directory listing | 2.7 |  | 0.828 |
| walker |  | 810 | 55 | headings outline in README.md |  |  | 0.829 |
| walker |  | 831 | 21 | listing of 'pkg/model/testdata' |  |  | 0.829 |
| walker |  | 835 | 4 | listing of 'pkg/model/testdata/container-volumes' |  |  | 0.829 |
| walker |  | 839 | 4 | listing of 'pkg/model/testdata/empty-workflow' |  |  | 0.829 |
| walker |  | 843 | 4 | listing of 'pkg/model/testdata/strategy' |  |  | 0.829 |
| walker |  | 850 | 7 | listing of 'pkg/model/testdata/nested' |  |  | 0.829 |
| walker |  | 854 | 4 | listing of 'pkg/model/testdata/nested/workflows' |  |  | 0.829 |
| walker |  | 875 | 21 | go decl names surface in cmd/platforms.go |  |  | 0.829 |
| walker |  | 875 | 0 | go decl at cmd/platforms.go:7 |  |  | 0.829 |
| walker |  | 940 | 65 | headings outline in IMAGES.md |  |  | 0.829 |
| walker |  | 963 | 23 | listing of 'pkg/exprparser/testdata' |  |  | 0.829 |
| walker |  | 970 | 7 | listing of 'pkg/exprparser/testdata/for-hashing-3' |  |  | 0.829 |
| walker |  | 975 | 5 | listing of 'pkg/exprparser/testdata/for-hashing-3/nested' |  |  | 0.829 |
| ns | 982 |  | 194 | pkg/runner/ directory listing | 2.8 |  | 0.732 |
| walker |  | 988 | 13 | go decl names surface in pkg/model/job_context.go |  |  | 0.732 |
| walker |  | 1004 | 16 | go decl names surface in pkg/common/outbound_ip.go |  |  | 0.732 |
| walker |  | 1004 | 0 | go decl at pkg/common/outbound_ip.go:13 |  |  | 0.732 |
| walker |  | 1024 | 20 | listing of '.github' |  |  | 0.732 |
| walker |  | 1050 | 26 | listing of '.github/workflows' |  |  | 0.732 |
| walker |  | 1084 | 34 | go decl names surface in cmd/dir.go |  |  | 0.732 |
| walker |  | 1084 | 0 | go decl at cmd/dir.go:15 |  |  | 0.732 |
| walker |  | 1093 | 9 | go decl at cmd/dir.go:10 |  |  | 0.732 |
| ns | 1170 |  | 188 | README — how it works | 3.1 |  | 0.725 |
| walker |  | 1253 | 160 | listing of 'pkg/container' |  |  | 0.826 |
| walker |  | 1267 | 14 | go decl names surface in pkg/container/executions_environment.go |  |  | 0.826 |
| walker |  | 1304 | 37 | listing of 'pkg/container/testdata' |  |  | 0.826 |
| walker |  | 1308 | 4 | listing of 'pkg/container/testdata/docker-pull-options' |  |  | 0.826 |
| walker |  | 1312 | 4 | listing of 'pkg/container/testdata/scratch' |  |  | 0.826 |
| walker |  | 1340 | 28 | README.md section #2 |  |  | 0.827 |
| walker |  | 1364 | 24 | listing of 'pkg/model/testdata/invalid-job-name' |  |  | 0.827 |
| walker |  | 1369 | 5 | go package + imports in pkg/model/job_context.go |  |  | 0.827 |
| ns | 1392 |  | 222 | README — user guide / contributing / manual build | 3.2 | 3.1 | 0.793 |
| walker |  | 1417 | 48 | go decl body at main.go:13 |  |  | 0.829 |
| walker |  | 1445 | 28 | README.md section #3 |  |  | 0.835 |
| ns | 1534 |  | 142 | Makefile — pr/build/format targets | 3.3 |  | 0.802 |
| walker |  | 1631 | 186 | listing of 'pkg/runner' |  |  | 0.897 |
| walker |  | 1635 | 4 | listing of 'pkg/runner/hashfiles' |  |  | 0.902 |
| walker |  | 1639 | 4 | listing of 'pkg/runner/res' |  |  | 0.908 |
| ns | 1789 |  | 255 | Makefile — test/lint targets | 3.4 | 3.3 | 0.851 |
| ns | 2211 |  | 422 | CI checks.yml — lint job | 4.1 |  | 0.793 |
| walker |  | 2354 | 715 | listing of 'pkg/runner/testdata' |  |  | 0.793 |
| walker |  | 2467 | 113 | Makefile target skeleton chunk #1 of Makefile |  |  | 0.798 |
| walker |  | 2532 | 65 | go package + imports in main.go |  |  | 0.798 |
| ns | 2539 |  | 328 | go.mod — primary direct dependencies | 4.2 |  | 0.772 |
| walker |  | 2557 | 25 | go decl names surface in pkg/gh/gh.go |  |  | 0.772 |
| walker |  | 2557 | 0 | go decl at pkg/gh/gh.go:10 |  |  | 0.772 |
| walker |  | 2608 | 51 | go decl names surface in cmd/secrets.go |  |  | 0.772 |
| walker |  | 2608 | 0 | go decl at cmd/secrets.go:14 |  |  | 0.772 |
| walker |  | 2608 | 0 | go decl at cmd/secrets.go:40 |  |  | 0.772 |
| walker |  | 2636 | 28 | go decl names surface in pkg/container/parse_env_file.go |  |  | 0.772 |
| walker |  | 2636 | 0 | go decl at pkg/container/parse_env_file.go:14 |  |  | 0.772 |
| walker |  | 2664 | 28 | go decl names surface in pkg/lookpath/error.go |  |  | 0.772 |
| walker |  | 2664 | 0 | go decl at pkg/lookpath/error.go:8 |  |  | 0.772 |
| walker |  | 2682 | 18 | go decl at pkg/lookpath/error.go:3 |  |  | 0.772 |
| walker |  | 2688 | 6 | go package + imports in pkg/lookpath/error.go |  |  | 0.772 |
| walker |  | 2695 | 7 | go decl body at cmd/secrets.go:40 |  |  | 0.772 |
| walker |  | 2704 | 9 | go decl body at pkg/lookpath/error.go:8 |  |  | 0.772 |
| walker |  | 2753 | 49 | IMAGES.md section #0 |  |  | 0.772 |
| walker |  | 2791 | 38 | go decl names surface in pkg/container/docker_network.go |  |  | 0.772 |
| walker |  | 2791 | 0 | go decl at pkg/container/docker_network.go:12 |  |  | 0.772 |
| walker |  | 2791 | 0 | go decl at pkg/container/docker_network.go:45 |  |  | 0.772 |
| walker |  | 2800 | 9 | go package + imports in pkg/common/cartesian.go |  |  | 0.772 |
| walker |  | 2840 | 40 | go decl names surface in pkg/artifactcache/model.go |  |  | 0.772 |
| walker |  | 2840 | 0 | go decl at pkg/artifactcache/model.go:9 |  |  | 0.772 |
| walker |  | 2846 | 6 | go package + imports in pkg/artifactcache/model.go |  |  | 0.772 |
| ns | 2850 |  | 311 | Input struct — flag-bound fields (part 1: run behavior) | 5.1 |  | 0.730 |
| walker |  | 2887 | 41 | go decl names surface in pkg/common/cartesian.go |  |  | 0.730 |
| walker |  | 2887 | 0 | go decl at pkg/common/cartesian.go:4 |  |  | 0.730 |
| walker |  | 2887 | 0 | go decl at pkg/common/cartesian.go:25 |  |  | 0.730 |
| walker |  | 2929 | 42 | go decl names surface in pkg/common/file.go |  |  | 0.730 |
| walker |  | 2929 | 0 | go decl at pkg/common/file.go:10 |  |  | 0.730 |
| walker |  | 2929 | 0 | go decl at pkg/common/file.go:37 |  |  | 0.730 |
| walker |  | 2939 | 10 | go decl doc at pkg/common/file.go:10 |  |  | 0.730 |
| walker |  | 2951 | 12 | go decl doc at pkg/common/file.go:37 |  |  | 0.730 |
| walker |  | 2993 | 42 | go decl names surface in pkg/container/docker_volume.go |  |  | 0.730 |
| walker |  | 2993 | 0 | go decl at pkg/container/docker_volume.go:12 |  |  | 0.730 |
| walker |  | 2993 | 0 | go decl at pkg/container/docker_volume.go:36 |  |  | 0.730 |
| walker |  | 3003 | 10 | go package + imports in pkg/artifactcache/doc.go |  |  | 0.730 |
| walker |  | 3050 | 47 | go decl names surface in pkg/model/anchors.go |  |  | 0.730 |
| walker |  | 3050 | 0 | go decl at pkg/model/anchors.go:9 |  |  | 0.730 |
| walker |  | 3050 | 0 | go decl at pkg/model/anchors.go:36 |  |  | 0.730 |
| walker |  | 3098 | 48 | go decl names surface in pkg/common/context.go |  |  | 0.730 |
| walker |  | 3098 | 0 | go decl at pkg/common/context.go:10 |  |  | 0.730 |
| walker |  | 3098 | 0 | go decl at pkg/common/context.go:42 |  |  | 0.730 |
| walker |  | 3146 | 48 | go decl names surface in pkg/container/util.go |  |  | 0.730 |
| walker |  | 3146 | 0 | go decl at pkg/container/util.go:12 |  |  | 0.730 |
| walker |  | 3146 | 0 | go decl at pkg/container/util.go:24 |  |  | 0.730 |
| ns | 3148 |  | 298 | Input struct — flag-bound fields (part 2: container/cache/misc) | 5.2 | 5.1 | 0.696 |
| walker |  | 3195 | 49 | go decl names surface in pkg/container/util_openbsd_mips64.go |  |  | 0.696 |
| walker |  | 3195 | 0 | go decl at pkg/container/util_openbsd_mips64.go:9 |  |  | 0.696 |
| walker |  | 3195 | 0 | go decl at pkg/container/util_openbsd_mips64.go:15 |  |  | 0.696 |
| walker |  | 3245 | 50 | go decl names surface in pkg/container/docker_auth.go |  |  | 0.696 |
| walker |  | 3245 | 0 | go decl at pkg/container/docker_auth.go:15 |  |  | 0.696 |
| walker |  | 3245 | 0 | go decl at pkg/container/docker_auth.go:42 |  |  | 0.696 |
| walker |  | 3261 | 16 | go decl doc at pkg/common/cartesian.go:4 |  |  | 0.696 |
| ns | 3545 |  | 397 | cmd/root.go — Flag struct, Execute, createRootCommand header | 5.3 |  | 0.666 |
| walker |  | 3552 | 291 | README.md section #0 |  |  | 0.666 |
| walker |  | 3595 | 43 | go decl at pkg/artifactcache/model.go:3 |  |  | 0.666 |
| walker |  | 3649 | 54 | go decl names surface in pkg/container/docker_build.go |  |  | 0.666 |
| walker |  | 3649 | 0 | go decl at pkg/container/docker_build.go:23 |  |  | 0.666 |
| walker |  | 3649 | 0 | go decl at pkg/container/docker_build.go:78 |  |  | 0.666 |
| walker |  | 3708 | 59 | go decl names surface in pkg/lookpath/env.go |  |  | 0.666 |
| walker |  | 3708 | 0 | go decl at pkg/lookpath/env.go:12 |  |  | 0.666 |
| walker |  | 3708 | 0 | go decl at pkg/lookpath/env.go:16 |  |  | 0.666 |
| walker |  | 3711 | 3 | go decl at pkg/lookpath/env.go:9 |  |  | 0.666 |
| walker |  | 3725 | 14 | go decl at pkg/lookpath/env.go:5 |  |  | 0.666 |
| walker |  | 3739 | 14 | go decl body at pkg/lookpath/env.go:16 |  |  | 0.666 |
| walker |  | 3758 | 19 | go decl doc at pkg/container/docker_build.go:23 |  |  | 0.666 |
| walker |  | 3818 | 60 | go decl names surface in pkg/container/docker_images.go |  |  | 0.666 |
| walker |  | 3818 | 0 | go decl at pkg/container/docker_images.go:16 |  |  | 0.666 |
| walker |  | 3818 | 0 | go decl at pkg/container/docker_images.go:44 |  |  | 0.666 |
| walker |  | 3879 | 61 | go decl names surface in pkg/runner/step_factory.go |  |  | 0.666 |
| walker |  | 3879 | 0 | go decl at pkg/runner/step_factory.go:15 |  |  | 0.666 |
| walker |  | 3907 | 28 | go package + imports in cmd/platforms.go |  |  | 0.666 |
| walker |  | 3979 | 72 | go decl names surface in pkg/container/docker_pull.go |  |  | 0.666 |
| walker |  | 3979 | 0 | go decl at pkg/container/docker_pull.go:21 |  |  | 0.666 |
| walker |  | 3979 | 0 | go decl at pkg/container/docker_pull.go:78 |  |  | 0.666 |
| walker |  | 3979 | 0 | go decl at pkg/container/docker_pull.go:123 |  |  | 0.666 |
| walker |  | 3998 | 19 | go decl doc at pkg/container/docker_pull.go:21 |  |  | 0.666 |
| walker |  | 4073 | 75 | go decl names surface in pkg/common/dryrun.go |  |  | 0.666 |
| walker |  | 4073 | 0 | go decl at pkg/common/dryrun.go:12 |  |  | 0.666 |
| walker |  | 4073 | 0 | go decl at pkg/common/dryrun.go:23 |  |  | 0.666 |
| walker |  | 4088 | 15 | go decl doc at pkg/common/dryrun.go:12 |  |  | 0.666 |
| walker |  | 4106 | 18 | go decl doc at pkg/common/dryrun.go:23 |  |  | 0.666 |
| ns | 4191 |  | 646 | cmd/root.go — most-queried CLI flags | 5.4 | 5.2 | 0.650 |
| walker |  | 4253 | 147 | go decl names surface in cmd/notices.go |  |  | 0.650 |
| walker |  | 4253 | 0 | go decl at cmd/notices.go:22 |  |  | 0.650 |
| walker |  | 4253 | 0 | go decl at cmd/notices.go:57 |  |  | 0.650 |
| walker |  | 4253 | 0 | go decl at cmd/notices.go:65 |  |  | 0.650 |
| walker |  | 4253 | 0 | go decl at cmd/notices.go:121 |  |  | 0.650 |
| walker |  | 4253 | 0 | go decl at cmd/notices.go:130 |  |  | 0.650 |
| walker |  | 4253 | 0 | go decl at cmd/notices.go:138 |  |  | 0.650 |
| walker |  | 4280 | 27 | go decl at cmd/notices.go:17 |  |  | 0.650 |
| walker |  | 4357 | 77 | go decl names surface in pkg/common/logger.go |  |  | 0.650 |
| walker |  | 4357 | 0 | go decl at pkg/common/logger.go:14 |  |  | 0.650 |
| walker |  | 4357 | 0 | go decl at pkg/common/logger.go:25 |  |  | 0.650 |
| walker |  | 4369 | 12 | go decl doc at pkg/common/logger.go:14 |  |  | 0.650 |
| walker |  | 4386 | 17 | go decl doc at pkg/common/logger.go:25 |  |  | 0.650 |
| walker |  | 4403 | 17 | go decl body at pkg/common/logger.go:25 |  |  | 0.650 |
| ns | 4413 |  | 222 | platforms.go — default runs-on image mapping | 5.5 |  | 0.636 |
| walker |  | 4553 | 150 | go decl names surface in cmd/input.go |  |  | 0.636 |
| walker |  | 4553 | 0 | go decl at cmd/input.go:70 |  |  | 0.636 |
| walker |  | 4553 | 0 | go decl at cmd/input.go:85 |  |  | 0.636 |
| walker |  | 4553 | 0 | go decl at cmd/input.go:90 |  |  | 0.636 |
| walker |  | 4553 | 0 | go decl at cmd/input.go:94 |  |  | 0.636 |
| walker |  | 4553 | 0 | go decl at cmd/input.go:99 |  |  | 0.636 |
| walker |  | 4553 | 0 | go decl at cmd/input.go:104 |  |  | 0.636 |
| walker |  | 4553 | 0 | go decl at cmd/input.go:109 |  |  | 0.636 |
| walker |  | 4553 | 0 | go decl at cmd/input.go:114 |  |  | 0.636 |
| walker |  | 4558 | 5 | go decl at cmd/input.go:10 |  |  | 0.636 |
| walker |  | 4567 | 9 | go decl body at cmd/input.go:99 |  |  | 0.636 |
| walker |  | 4581 | 14 | go decl doc at cmd/input.go:10 |  |  | 0.636 |
| walker |  | 4593 | 12 | go decl doc at cmd/input.go:90 |  |  | 0.636 |
| walker |  | 4606 | 13 | go decl doc at cmd/input.go:85 |  |  | 0.636 |
| ns | 4615 |  | 202 | Executor / Conditional / Warning types | 6.1 |  | 0.618 |
| walker |  | 4619 | 13 | go decl doc at cmd/input.go:99 |  |  | 0.618 |
| walker |  | 4633 | 14 | go decl doc at cmd/input.go:109 |  |  | 0.618 |
| walker |  | 4648 | 15 | go decl doc at cmd/input.go:104 |  |  | 0.618 |
| walker |  | 4663 | 15 | go decl doc at cmd/input.go:114 |  |  | 0.618 |
| walker |  | 4681 | 18 | go package + imports in pkg/container/executions_environment.go |  |  | 0.618 |
| walker |  | 4700 | 19 | go decl body at pkg/common/dryrun.go:23 |  |  | 0.618 |
| walker |  | 4709 | 9 | go decl body at pkg/container/util.go:24 |  |  | 0.618 |
| ns | 4732 |  | 117 | Workflow struct — top-level YAML shape | 7.1 |  | 0.611 |
| walker |  | 4733 | 24 | go decl at pkg/runner/step_factory.go:9 |  |  | 0.611 |
| walker |  | 4752 | 19 | go package + imports in pkg/lookpath/env.go |  |  | 0.611 |
| walker |  | 4837 | 85 | go decl names surface in pkg/container/docker_socket.go |  |  | 0.611 |
| walker |  | 4837 | 0 | go decl at pkg/container/docker_socket.go:23 |  |  | 0.611 |
| walker |  | 4837 | 0 | go decl at pkg/container/docker_socket.go:45 |  |  | 0.611 |
| walker |  | 4837 | 0 | go decl at pkg/container/docker_socket.go:62 |  |  | 0.611 |
| walker |  | 4857 | 20 | go decl at pkg/container/docker_socket.go:57 |  |  | 0.611 |
| walker |  | 4942 | 85 | go decl names surface in pkg/workflowpattern/trace_writer.go |  |  | 0.611 |
| walker |  | 4942 | 0 | go decl at pkg/workflowpattern/trace_writer.go:16 |  |  | 0.611 |
| walker |  | 4956 | 14 | go decl at pkg/workflowpattern/trace_writer.go:5 |  |  | 0.611 |
| walker |  | 4968 | 12 | go decl body at pkg/workflowpattern/trace_writer.go:16 |  |  | 0.611 |
| walker |  | 4987 | 19 | go package + imports in pkg/workflowpattern/trace_writer.go |  |  | 0.611 |
| ns | 5004 |  | 272 | Job struct — per-job YAML shape | 7.2 | 7.1 | 0.597 |
| walker |  | 5074 | 87 | go decl names surface in pkg/runner/local_repository_cache.go |  |  | 0.597 |
| walker |  | 5074 | 0 | go decl at pkg/runner/local_repository_cache.go:25 |  |  | 0.597 |
| walker |  | 5074 | 0 | go decl at pkg/runner/local_repository_cache.go:44 |  |  | 0.597 |
| walker |  | 5110 | 36 | go decl at pkg/runner/local_repository_cache.go:19 |  |  | 0.597 |
| walker |  | 5119 | 9 | go decl body at pkg/lookpath/env.go:12 |  |  | 0.597 |
| ns | 5197 |  | 193 | Strategy / Defaults / RunDefaults structs | 7.3 | 7.2 | 0.584 |
| walker |  | 5210 | 91 | go decl names surface in pkg/container/docker_logger.go |  |  | 0.584 |
| walker |  | 5210 | 0 | go decl at pkg/container/docker_logger.go:27 |  |  | 0.584 |
| walker |  | 5210 | 0 | go decl at pkg/container/docker_logger.go:77 |  |  | 0.584 |
| walker |  | 5304 | 94 | go decl names surface in pkg/runner/action_cache_offline_mode.go |  |  | 0.584 |
| walker |  | 5304 | 0 | go decl at pkg/runner/action_cache_offline_mode.go:17 |  |  | 0.584 |
| walker |  | 5304 | 0 | go decl at pkg/runner/action_cache_offline_mode.go:45 |  |  | 0.584 |
| walker |  | 5318 | 14 | go decl at pkg/runner/action_cache_offline_mode.go:13 |  |  | 0.584 |
| walker |  | 5348 | 30 | go decl doc at pkg/container/docker_images.go:44 |  |  | 0.584 |
| ns | 5431 |  | 234 | Step struct — per-step YAML shape | 7.4 | 7.2 | 0.574 |
| walker |  | 5443 | 95 | go decl names surface in pkg/common/line_writer.go |  |  | 0.574 |
| walker |  | 5443 | 0 | go decl at pkg/common/line_writer.go:9 |  |  | 0.574 |
| walker |  | 5443 | 0 | go decl at pkg/common/line_writer.go:17 |  |  | 0.574 |
| walker |  | 5443 | 0 | go decl at pkg/common/line_writer.go:23 |  |  | 0.574 |
| walker |  | 5443 | 0 | go decl at pkg/common/line_writer.go:43 |  |  | 0.574 |
| walker |  | 5459 | 16 | go decl doc at pkg/common/line_writer.go:9 |  |  | 0.574 |
| walker |  | 5476 | 17 | go decl doc at pkg/common/line_writer.go:17 |  |  | 0.574 |
| walker |  | 5498 | 22 | go decl at pkg/common/line_writer.go:11 |  |  | 0.574 |
| walker |  | 5520 | 22 | go package + imports in pkg/model/step_result.go |  |  | 0.574 |
| walker |  | 5540 | 20 | go decl body at pkg/runner/action_cache_offline_mode.go:45 |  |  | 0.574 |
| ns | 5904 |  | 473 | GithubContext struct — every github.* expression field | 7.5 |  | 0.554 |
| ns | 6136 |  | 232 | action.go — ActionRuns struct | 7.6 |  | 0.545 |
| walker |  | 6349 | 809 | go module file go.mod |  |  | 0.574 |
| walker |  | 6453 | 104 | go decl names surface in pkg/container/container_types.go |  |  | 0.561 |
| ns | 6453 |  | 317 | action.go — Action / Input / Output structs | 7.7 | 7.6 | 0.561 |
| walker |  | 6460 | 7 | go decl at pkg/container/container_types.go:80 |  |  | 0.561 |
| walker |  | 6489 | 29 | go decl at pkg/container/container_types.go:36 |  |  | 0.561 |
| walker |  | 6505 | 16 | go decl doc at pkg/container/container_types.go:36 |  |  | 0.561 |
| walker |  | 6552 | 47 | go decl at pkg/container/container_types.go:70 |  |  | 0.561 |
| walker |  | 6572 | 20 | go decl doc at pkg/container/container_types.go:70 |  |  | 0.561 |
| walker |  | 6622 | 50 | go decl at pkg/container/container_types.go:61 |  |  | 0.561 |
| walker |  | 6642 | 20 | go decl doc at pkg/container/container_types.go:61 |  |  | 0.561 |
| walker |  | 6657 | 15 | go decl doc at pkg/container/docker_socket.go:23 |  |  | 0.561 |
| walker |  | 6661 | 4 | listing of '.github/actions' |  |  | 0.561 |
| walker |  | 6711 | 50 | go package + imports in cmd/input.go |  |  | 0.561 |
| walker |  | 6825 | 114 | go decl names surface in pkg/common/auth.go |  |  | 0.561 |
| walker |  | 6825 | 0 | go decl at pkg/common/auth.go:38 |  |  | 0.561 |
| walker |  | 6825 | 0 | go decl at pkg/common/auth.go:72 |  |  | 0.561 |
| walker |  | 6832 | 7 | go decl at pkg/common/auth.go:33 |  |  | 0.561 |
| ns | 6835 |  | 382 | planner.go — Plan/Stage/Run + WorkflowPlanner interface | 7.8 |  | 0.540 |
| walker |  | 6854 | 22 | go decl at pkg/common/auth.go:26 |  |  | 0.540 |
| walker |  | 6972 | 118 | go decl names surface in pkg/runner/job_executor.go |  |  | 0.540 |
| walker |  | 6972 | 0 | go decl at pkg/runner/job_executor.go:23 |  |  | 0.540 |
| walker |  | 6972 | 0 | go decl at pkg/runner/job_executor.go:157 |  |  | 0.540 |
| walker |  | 6972 | 0 | go decl at pkg/runner/job_executor.go:185 |  |  | 0.540 |
| walker |  | 6972 | 0 | go decl at pkg/runner/job_executor.go:200 |  |  | 0.540 |
| walker |  | 6983 | 11 | go decl doc at pkg/runner/job_executor.go:23 |  |  | 0.540 |
| walker |  | 7021 | 38 | go decl doc at pkg/container/docker_images.go:16 |  |  | 0.540 |
| walker |  | 7049 | 28 | go package + imports in pkg/common/dryrun.go |  |  | 0.540 |
| walker |  | 7075 | 26 | go decl body at pkg/common/context.go:42 |  |  | 0.540 |
| walker |  | 7101 | 26 | go decl body at pkg/common/line_writer.go:17 |  |  | 0.540 |
| ns | 7176 |  | 341 | Example workflow file (integration fixture) | 8.1 |  | 0.526 |
| walker |  | 7223 | 122 | go decl names surface in pkg/workflowpattern/workflow_pattern.go |  |  | 0.526 |
| walker |  | 7223 | 0 | go decl at pkg/workflowpattern/workflow_pattern.go:15 |  |  | 0.526 |
| walker |  | 7223 | 0 | go decl at pkg/workflowpattern/workflow_pattern.go:38 |  |  | 0.526 |
| walker |  | 7223 | 0 | go decl at pkg/workflowpattern/workflow_pattern.go:138 |  |  | 0.526 |
| walker |  | 7223 | 0 | go decl at pkg/workflowpattern/workflow_pattern.go:151 |  |  | 0.526 |
| walker |  | 7223 | 0 | go decl at pkg/workflowpattern/workflow_pattern.go:177 |  |  | 0.526 |
| walker |  | 7255 | 32 | go decl at pkg/workflowpattern/workflow_pattern.go:9 |  |  | 0.526 |
| walker |  | 7267 | 12 | go decl doc at pkg/workflowpattern/workflow_pattern.go:38 |  |  | 0.526 |
| walker |  | 7284 | 17 | go decl doc at pkg/workflowpattern/workflow_pattern.go:151 |  |  | 0.526 |
| walker |  | 7303 | 19 | go decl doc at pkg/workflowpattern/workflow_pattern.go:177 |  |  | 0.526 |
| ns | 7359 |  | 183 | Example action.yml (docker action) | 8.2 |  | 0.518 |
| walker |  | 7433 | 130 | go package doc lede in pkg/artifactcache/doc.go |  |  | 0.518 |
| ns | 7474 |  | 115 | Sample .actrc / secrets files | 8.3 |  | 0.514 |
| walker |  | 7560 | 127 | go decl names surface in pkg/model/step_result.go |  |  | 0.514 |
| walker |  | 7560 | 0 | go decl at pkg/model/step_result.go:19 |  |  | 0.514 |
| walker |  | 7560 | 0 | go decl at pkg/model/step_result.go:23 |  |  | 0.514 |
| walker |  | 7560 | 0 | go decl at pkg/model/step_result.go:34 |  |  | 0.514 |
| walker |  | 7567 | 7 | go decl at pkg/model/step_result.go:7 |  |  | 0.514 |
| walker |  | 7616 | 49 | go decl at pkg/model/step_result.go:41 |  |  | 0.514 |
| walker |  | 7673 | 57 | go package + imports in cmd/dir.go |  |  | 0.514 |
| walker |  | 7685 | 12 | go decl body at pkg/model/step_result.go:19 |  |  | 0.514 |
| walker |  | 7773 | 88 | go struct field group at cmd/input.go:10 group 59 |  |  | 0.518 |
| walker |  | 7805 | 32 | go package + imports in pkg/common/job_error.go |  |  | 0.518 |
| ns | 7810 |  | 336 | Runner interface + Config struct (part 1: run behavior) | 9.1 |  | 0.509 |
| walker |  | 7867 | 62 | go package + imports in cmd/list.go |  |  | 0.509 |
| walker |  | 7930 | 63 | go package + imports in cmd/graph.go |  |  | 0.509 |
| walker |  | 7944 | 14 | go decl body at pkg/container/util_openbsd_mips64.go:15 |  |  | 0.509 |
| walker |  | 8038 | 94 | go decl at pkg/model/job_context.go:3 |  |  | 0.509 |
| walker |  | 8073 | 35 | go package + imports in pkg/common/line_writer.go |  |  | 0.509 |
| walker |  | 8186 | 113 | README.md section #4 |  |  | 0.525 |
| walker |  | 8347 | 161 | go decl names surface in pkg/container/linux_container_environment_extensions.go |  |  | 0.525 |
| walker |  | 8347 | 0 | go decl at pkg/container/linux_container_environment_extensions.go:19 |  |  | 0.525 |
| walker |  | 8347 | 0 | go decl at pkg/container/linux_container_environment_extensions.go:50 |  |  | 0.525 |
| walker |  | 8347 | 0 | go decl at pkg/container/linux_container_environment_extensions.go:54 |  |  | 0.525 |
| walker |  | 8347 | 0 | go decl at pkg/container/linux_container_environment_extensions.go:58 |  |  | 0.525 |
| walker |  | 8347 | 0 | go decl at pkg/container/linux_container_environment_extensions.go:62 |  |  | 0.525 |
| walker |  | 8347 | 0 | go decl at pkg/container/linux_container_environment_extensions.go:66 |  |  | 0.525 |
| walker |  | 8347 | 0 | go decl at pkg/container/linux_container_environment_extensions.go:75 |  |  | 0.525 |
| walker |  | 8352 | 5 | go decl at pkg/container/linux_container_environment_extensions.go:13 |  |  | 0.525 |
| walker |  | 8359 | 7 | go decl body at pkg/container/linux_container_environment_extensions.go:75 |  |  | 0.525 |
| walker |  | 8367 | 8 | go decl body at pkg/container/linux_container_environment_extensions.go:54 |  |  | 0.525 |
| ns | 8368 |  | 558 | Config struct (part 2: container/artifact-server/misc) | 9.2 | 9.1 | 0.513 |
| walker |  | 8378 | 11 | go decl body at pkg/container/linux_container_environment_extensions.go:50 |  |  | 0.513 |
| walker |  | 8389 | 11 | go decl body at pkg/container/linux_container_environment_extensions.go:62 |  |  | 0.513 |
| walker |  | 8554 | 165 | go decl names surface in pkg/runner/action_composite.go |  |  | 0.513 |
| walker |  | 8554 | 0 | go decl at pkg/runner/action_composite.go:13 |  |  | 0.513 |
| walker |  | 8554 | 0 | go decl at pkg/runner/action_composite.go:47 |  |  | 0.513 |
| walker |  | 8554 | 0 | go decl at pkg/runner/action_composite.go:84 |  |  | 0.513 |
| walker |  | 8554 | 0 | go decl at pkg/runner/action_composite.go:133 |  |  | 0.513 |
| walker |  | 8554 | 0 | go decl at pkg/runner/action_composite.go:197 |  |  | 0.513 |
| walker |  | 8554 | 0 | go decl at pkg/runner/action_composite.go:222 |  |  | 0.513 |
| walker |  | 8584 | 30 | go decl at pkg/runner/action_composite.go:126 |  |  | 0.513 |
| walker |  | 8600 | 16 | go decl doc at pkg/runner/action_composite.go:133 |  |  | 0.513 |
| walker |  | 8675 | 75 | go package + imports in cmd/secrets.go |  |  | 0.513 |
| walker |  | 8703 | 28 | go decl at pkg/model/step_result.go:13 |  |  | 0.513 |
| walker |  | 8811 | 108 | go decl at pkg/container/executions_environment.go:5 |  |  | 0.513 |
| ns | 8815 |  | 447 | step_factory.go — Step.Type() to step-kind dispatch (full file) | 9.3 |  | 0.500 |
| walker |  | 8822 | 11 | go decl body at cmd/input.go:85 |  |  | 0.500 |
| walker |  | 8864 | 42 | go package + imports in pkg/common/file.go |  |  | 0.500 |
| walker |  | 9050 | 186 | go decl names surface in pkg/common/job_error.go |  |  | 0.500 |
| walker |  | 9050 | 0 | go decl at pkg/common/job_error.go:16 |  |  | 0.500 |
| walker |  | 9050 | 0 | go decl at pkg/common/job_error.go:26 |  |  | 0.500 |
| walker |  | 9050 | 0 | go decl at pkg/common/job_error.go:31 |  |  | 0.500 |
| walker |  | 9050 | 0 | go decl at pkg/common/job_error.go:36 |  |  | 0.500 |
| walker |  | 9050 | 0 | go decl at pkg/common/job_error.go:40 |  |  | 0.500 |
| walker |  | 9050 | 0 | go decl at pkg/common/job_error.go:51 |  |  | 0.500 |
| walker |  | 9065 | 15 | go decl doc at pkg/common/job_error.go:16 |  |  | 0.500 |
| walker |  | 9087 | 22 | go decl doc at pkg/common/job_error.go:31 |  |  | 0.500 |
| walker |  | 9105 | 18 | go decl body at pkg/common/job_error.go:36 |  |  | 0.500 |
| ns | 9130 |  | 315 | RunContext struct — per-job execution state | 9.4 |  | 0.490 |
| walker |  | 9133 | 28 | go decl doc at pkg/common/job_error.go:51 |  |  | 0.490 |
| walker |  | 9156 | 23 | go decl body at pkg/common/job_error.go:26 |  |  | 0.490 |
| walker |  | 9273 | 117 | go struct field group at cmd/input.go:10 group 23 |  |  | 0.498 |
| walker |  | 9302 | 29 | go decl body at cmd/notices.go:57 |  |  | 0.498 |
| walker |  | 9346 | 44 | go package + imports in pkg/container/util_openbsd_mips64.go |  |  | 0.498 |
| walker |  | 9390 | 44 | go package + imports in pkg/workflowpattern/workflow_pattern.go |  |  | 0.498 |
| ns | 9402 |  | 272 | Container interface — the execution-backend contract | 10.1 |  | 0.492 |
| walker |  | 9509 | 119 | go struct field group at cmd/input.go:10 group 11 |  |  | 0.510 |
| walker |  | 9651 | 142 | go decl names surface in pkg/runner/run_context.go |  |  | 0.505 |
| walker |  | 9651 | 0 | go decl at pkg/runner/run_context.go:58 |  |  | 0.505 |
| walker |  | 9651 | 0 | go decl at pkg/runner/run_context.go:67 |  |  | 0.505 |
| walker |  | 9651 | 0 | go decl at pkg/runner/run_context.go:78 |  |  | 0.505 |
| walker |  | 9651 | 0 | go decl at pkg/runner/run_context.go:92 |  |  | 0.505 |
| walker |  | 9651 | 0 | go decl at pkg/runner/run_context.go:98 |  |  | 0.505 |
| walker |  | 9651 | 0 | go decl at pkg/runner/run_context.go:108 |  |  | 0.505 |
| ns | 9651 |  | 249 | functions.go — every builtin GitHub Actions expression function | 11.1 |  | 0.505 |
| walker |  | 9672 | 21 | go decl at pkg/runner/run_context.go:62 |  |  | 0.505 |
| walker |  | 9686 | 14 | go decl doc at pkg/runner/run_context.go:78 |  |  | 0.505 |
| walker |  | 9701 | 15 | go decl body at pkg/runner/run_context.go:58 |  |  | 0.505 |
| walker |  | 9715 | 14 | go decl body at pkg/runner/run_context.go:92 |  |  | 0.505 |

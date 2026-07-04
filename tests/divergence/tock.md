Score(3000)=0.662 I=0.839 C=0.523 ns_rows≤3K=18/43 (reached=6 partial=5 missing=7)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 8 |  | 8 | README title | 1.1 |  | 0.000 |
| walker |  | 52 | 52 | listing of '.' |  |  | 0.000 |
| walker |  | 56 | 4 | listing of 'cmd' |  |  | 0.000 |
| walker |  | 60 | 4 | listing of 'docs' |  |  | 0.821 |
| ns | 60 |  | 52 | Repo top-level listing | 1.2 |  | 0.821 |
| walker |  | 86 | 26 | go module identity in go.mod |  |  | 0.823 |
| ns | 134 |  | 74 | README feature lede (first half) | 1.3 |  | 0.704 |
| walker |  | 173 | 87 | README headline in README.md |  |  | 0.857 |
| ns | 178 |  | 44 | internal/ subpackage listing | 1.4 |  | 0.637 |
| walker |  | 193 | 20 | README.md section #11 |  |  | 0.637 |
| walker |  | 212 | 19 | listing of 'internal' |  |  | 0.690 |
| walker |  | 232 | 20 | listing of 'assets' |  |  | 0.690 |
| walker |  | 244 | 12 | listing of 'internal/core' |  |  | 0.780 |
| ns | 316 |  | 138 | README feature lede (rest) | 1.5 | 1.3 | 0.711 |
| walker |  | 422 | 178 | headings outline in docs/commands.md |  |  | 0.711 |
| walker |  | 427 | 5 | docs/commands.md section #5 |  |  | 0.711 |
| walker |  | 432 | 5 | docs/commands.md section #10 |  |  | 0.711 |
| ns | 435 |  | 119 | go.mod module + Go version + UI deps | 1.6 |  | 0.645 |
| walker |  | 437 | 5 | docs/commands.md section #15 |  |  | 0.645 |
| walker |  | 442 | 5 | docs/commands.md section #20 |  |  | 0.645 |
| walker |  | 447 | 5 | docs/commands.md section #30 |  |  | 0.645 |
| walker |  | 452 | 5 | docs/commands.md section #35 |  |  | 0.645 |
| walker |  | 457 | 5 | docs/commands.md section #40 |  |  | 0.645 |
| ns | 589 |  | 154 | go.mod remaining direct deps | 1.7 | 1.6 | 0.585 |
| ns | 690 |  | 101 | Sentinel errors (whole file) | 2.1 |  | 0.533 |
| walker |  | 705 | 248 | go module file go.mod |  |  | 0.691 |
| walker |  | 709 | 4 | listing of 'cmd/tock' |  |  | 0.691 |
| walker |  | 719 | 10 | go decl names surface in cmd/tock/main.go |  |  | 0.691 |
| walker |  | 719 | 0 | go decl at cmd/tock/main.go:7 |  |  | 0.691 |
| walker |  | 725 | 6 | go decl body at cmd/tock/main.go:7 |  |  | 0.692 |
| walker |  | 731 | 6 | listing of 'internal/adapters' |  |  | 0.727 |
| walker |  | 742 | 11 | listing of 'internal/adapters/repositories' |  |  | 0.727 |
| walker |  | 760 | 18 | listing of 'internal/adapters/repositories/file' |  |  | 0.727 |
| walker |  | 767 | 7 | listing of 'internal/services' |  |  | 0.768 |
| walker |  | 771 | 4 | listing of 'internal/core/errors' |  |  | 0.768 |
| walker |  | 775 | 4 | listing of 'internal/core/models' |  |  | 0.768 |
| walker |  | 779 | 4 | listing of 'internal/services/ics' |  |  | 0.768 |
| walker |  | 794 | 15 | go package + imports in internal/core/errors/errors.go |  |  | 0.770 |
| walker |  | 803 | 9 | listing of 'internal/config' |  |  | 0.770 |
| ns | 810 |  | 120 | models.Activity struct fields | 2.2 |  | 0.716 |
| walker |  | 812 | 9 | listing of 'internal/extra' |  |  | 0.716 |
| walker |  | 848 | 36 | go decl names surface in internal/extra/extra.go |  |  | 0.716 |
| walker |  | 848 | 0 | go decl at internal/extra/extra.go:11 |  |  | 0.716 |
| ns | 1062 |  | 252 | ports.ActivityResolver interface | 2.3 |  | 0.625 |
| walker |  | 1085 | 237 | README.md section #1 |  |  | 0.705 |
| walker |  | 1151 | 66 | go decl names surface in internal/services/ics/generator.go |  |  | 0.706 |
| walker |  | 1151 | 0 | go decl at internal/services/ics/generator.go:12 |  |  | 0.706 |
| walker |  | 1151 | 0 | go decl at internal/services/ics/generator.go:18 |  |  | 0.706 |
| walker |  | 1151 | 0 | go decl at internal/services/ics/generator.go:29 |  |  | 0.706 |
| walker |  | 1151 | 0 | go decl at internal/services/ics/generator.go:71 |  |  | 0.706 |
| walker |  | 1168 | 17 | go decl doc at internal/services/ics/generator.go:12 |  |  | 0.706 |
| walker |  | 1185 | 17 | go decl doc at internal/services/ics/generator.go:29 |  |  | 0.706 |
| walker |  | 1205 | 20 | go decl doc at internal/services/ics/generator.go:18 |  |  | 0.706 |
| ns | 1217 |  | 155 | ports.ActivityRepository + NotesRepository | 2.4 | 2.3 | 0.663 |
| walker |  | 1272 | 67 | go decl names surface in internal/core/errors/errors.go |  |  | 0.709 |
| walker |  | 1281 | 9 | go decl at internal/core/errors/errors.go:5 |  |  | 0.716 |
| walker |  | 1289 | 8 | docs/commands.md section #16 |  |  | 0.716 |
| walker |  | 1368 | 79 | README.md section #6 |  |  | 0.716 |
| walker |  | 1388 | 20 | go decl body at internal/services/ics/generator.go:12 |  |  | 0.716 |
| walker |  | 1394 | 6 | listing of 'internal/core/dto' |  |  | 0.716 |
| walker |  | 1462 | 68 | go decl names surface in internal/core/dto/activity_dto.go |  |  | 0.717 |
| walker |  | 1493 | 31 | go decl at internal/core/dto/activity_dto.go:17 |  |  | 0.718 |
| walker |  | 1526 | 33 | go decl at internal/core/dto/activity_dto.go:46 |  |  | 0.719 |
| walker |  | 1564 | 38 | go decl at internal/core/dto/activity_dto.go:40 |  |  | 0.721 |
| walker |  | 1612 | 48 | go decl at internal/core/dto/activity_dto.go:9 |  |  | 0.725 |
| ns | 1630 |  | 413 | dto request/filter/report types | 2.5 |  | 0.645 |
| walker |  | 1667 | 55 | go decl at internal/core/dto/activity_dto.go:32 |  |  | 0.679 |
| walker |  | 1725 | 58 | go decl at internal/core/dto/activity_dto.go:23 |  |  | 0.726 |
| walker |  | 1736 | 11 | listing of 'internal/timeutil' |  |  | 0.726 |
| walker |  | 1813 | 77 | go decl names surface in internal/core/models/activity.go |  |  | 0.726 |
| walker |  | 1813 | 0 | go decl at internal/core/models/activity.go:19 |  |  | 0.726 |
| walker |  | 1813 | 0 | go decl at internal/core/models/activity.go:23 |  |  | 0.726 |
| walker |  | 1813 | 0 | go decl at internal/core/models/activity.go:31 |  |  | 0.726 |
| walker |  | 1813 | 0 | go decl at internal/core/models/activity.go:41 |  |  | 0.726 |
| walker |  | 1824 | 11 | go decl body at internal/core/models/activity.go:19 |  |  | 0.727 |
| walker |  | 1843 | 19 | go decl doc at internal/core/models/activity.go:31 |  |  | 0.727 |
| walker |  | 1852 | 9 | docs/commands.md section #1 |  |  | 0.727 |
| ns | 1858 |  | 228 | models.Activity helper methods | 2.6 | 2.2 | 0.684 |
| walker |  | 1861 | 9 | docs/commands.md section #6 |  |  | 0.684 |
| walker |  | 1898 | 37 | go package + imports in cmd/tock/main.go |  |  | 0.686 |
| walker |  | 1905 | 7 | listing of 'internal/core/ports' |  |  | 0.686 |
| ns | 1926 |  | 68 | main.go — entry point | 3.1 |  | 0.686 |
| walker |  | 1938 | 33 | go decl names surface in internal/core/ports/ports.go |  |  | 0.687 |
| walker |  | 1993 | 55 | go decl at internal/core/ports/ports.go:29 |  |  | 0.696 |
| walker |  | 2003 | 10 | docs/commands.md section #11 |  |  | 0.696 |
| ns | 2005 |  | 79 | cli/ directory listing | 3.2 |  | 0.658 |
| walker |  | 2013 | 10 | docs/commands.md section #31 |  |  | 0.658 |
| walker |  | 2098 | 85 | README.md section #10 |  |  | 0.658 |
| ns | 2188 |  | 183 | Cobra command factory locations across cli/ | 3.3 |  | 0.634 |
| walker |  | 2259 | 161 | go decl names surface in internal/config/config.go |  |  | 0.634 |
| walker |  | 2259 | 0 | go decl at internal/config/config.go:62 |  |  | 0.634 |
| walker |  | 2259 | 0 | go decl at internal/config/config.go:68 |  |  | 0.634 |
| walker |  | 2259 | 0 | go decl at internal/config/config.go:74 |  |  | 0.634 |
| walker |  | 2259 | 0 | go decl at internal/config/config.go:80 |  |  | 0.634 |
| walker |  | 2274 | 15 | go decl at internal/config/config.go:41 |  |  | 0.634 |
| walker |  | 2291 | 17 | go decl at internal/config/config.go:37 |  |  | 0.634 |
| walker |  | 2308 | 17 | go decl at internal/config/config.go:45 |  |  | 0.634 |
| walker |  | 2327 | 19 | go decl at internal/config/config.go:33 |  |  | 0.634 |
| walker |  | 2356 | 29 | go decl body at internal/config/config.go:62 |  |  | 0.634 |
| walker |  | 2385 | 29 | go decl body at internal/config/config.go:68 |  |  | 0.634 |
| walker |  | 2414 | 29 | go decl body at internal/config/config.go:74 |  |  | 0.634 |
| ns | 2423 |  | 235 | README commands list (Use/Short for every cmd) | 3.4 |  | 0.606 |
| walker |  | 2504 | 90 | go decl at internal/config/config.go:25 |  |  | 0.607 |
| ns | 2510 |  | 87 | Activity service constructor | 4.1 |  | 0.596 |
| walker |  | 2583 | 79 | listing of 'internal/adapters/cli' |  |  | 0.652 |
| walker |  | 2598 | 15 | go decl names surface in internal/adapters/cli/start.go |  |  | 0.653 |
| walker |  | 2598 | 0 | go decl at internal/adapters/cli/start.go:17 |  |  | 0.653 |
| walker |  | 2613 | 15 | go decl names surface in internal/adapters/cli/stop.go |  |  | 0.653 |
| walker |  | 2613 | 0 | go decl at internal/adapters/cli/stop.go:14 |  |  | 0.653 |
| walker |  | 2643 | 30 | go decl names surface in internal/adapters/cli/continue.go |  |  | 0.654 |
| walker |  | 2643 | 0 | go decl at internal/adapters/cli/continue.go:20 |  |  | 0.654 |
| walker |  | 2652 | 9 | go decl at internal/adapters/cli/continue.go:15 |  |  | 0.654 |
| walker |  | 2696 | 44 | go decl names surface in internal/adapters/cli/update.go |  |  | 0.654 |
| walker |  | 2696 | 0 | go decl at internal/adapters/cli/update.go:22 |  |  | 0.654 |
| walker |  | 2696 | 0 | go decl at internal/adapters/cli/update.go:45 |  |  | 0.654 |
| walker |  | 2743 | 47 | go decl names surface in internal/adapters/cli/version.go |  |  | 0.656 |
| walker |  | 2743 | 0 | go decl at internal/adapters/cli/version.go:16 |  |  | 0.656 |
| walker |  | 2752 | 9 | go decl at internal/adapters/cli/version.go:10 |  |  | 0.656 |
| walker |  | 2800 | 48 | go decl names surface in internal/adapters/cli/last.go |  |  | 0.657 |
| walker |  | 2800 | 0 | go decl at internal/adapters/cli/last.go:21 |  |  | 0.657 |
| walker |  | 2800 | 0 | go decl at internal/adapters/cli/last.go:42 |  |  | 0.657 |
| walker |  | 2848 | 48 | go decl names surface in internal/adapters/cli/report.go |  |  | 0.660 |
| walker |  | 2848 | 0 | go decl at internal/adapters/cli/report.go:31 |  |  | 0.660 |
| walker |  | 2848 | 0 | go decl at internal/adapters/cli/report.go:61 |  |  | 0.660 |
| walker |  | 2866 | 18 | go decl doc at internal/adapters/cli/continue.go:20 |  |  | 0.660 |
| walker |  | 2888 | 22 | go decl doc at internal/adapters/cli/start.go:17 |  |  | 0.660 |
| walker |  | 2952 | 64 | go decl names surface in internal/adapters/cli/current.go |  |  | 0.662 |
| walker |  | 2952 | 0 | go decl at internal/adapters/cli/current.go:23 |  |  | 0.662 |
| walker |  | 2952 | 0 | go decl at internal/adapters/cli/current.go:27 |  |  | 0.662 |
| walker |  | 2952 | 0 | go decl at internal/adapters/cli/current.go:35 |  |  | 0.662 |
| walker |  | 2963 | 11 | go decl at internal/adapters/cli/current.go:19 |  |  | 0.662 |
| walker |  | 2984 | 21 | go decl at internal/adapters/cli/last.go:16 |  |  | 0.662 |
| ns | 3040 |  | 530 | File-format ParseActivity | 4.2 |  | 0.596 |
| walker |  | 3054 | 70 | go decl names surface in internal/adapters/cli/analyze.go |  |  | 0.598 |
| walker |  | 3054 | 0 | go decl at internal/adapters/cli/analyze.go:20 |  |  | 0.598 |
| walker |  | 3054 | 0 | go decl at internal/adapters/cli/analyze.go:84 |  |  | 0.598 |
| walker |  | 3054 | 0 | go decl at internal/adapters/cli/analyze.go:206 |  |  | 0.598 |
| walker |  | 3147 | 93 | go decl names surface in internal/adapters/cli/remove.go |  |  | 0.602 |
| walker |  | 3147 | 0 | go decl at internal/adapters/cli/remove.go:22 |  |  | 0.602 |
| walker |  | 3147 | 0 | go decl at internal/adapters/cli/remove.go:95 |  |  | 0.602 |
| walker |  | 3147 | 0 | go decl at internal/adapters/cli/remove.go:116 |  |  | 0.602 |
| walker |  | 3147 | 0 | go decl at internal/adapters/cli/remove.go:127 |  |  | 0.602 |
| ns | 3173 |  | 133 | File-format FormatActivity (writer side) | 4.3 | 4.2 | 0.590 |
| walker |  | 3247 | 100 | go decl names surface in internal/adapters/cli/add.go |  |  | 0.593 |
| walker |  | 3247 | 0 | go decl at internal/adapters/cli/add.go:26 |  |  | 0.593 |
| walker |  | 3247 | 0 | go decl at internal/adapters/cli/add.go:64 |  |  | 0.593 |
| walker |  | 3247 | 0 | go decl at internal/adapters/cli/add.go:124 |  |  | 0.593 |
| walker |  | 3247 | 0 | go decl at internal/adapters/cli/add.go:163 |  |  | 0.593 |
| walker |  | 3320 | 73 | go decl at internal/core/ports/ports.go:22 |  |  | 0.614 |
| walker |  | 3338 | 18 | listing of 'internal/core/ports/mocks' |  |  | 0.614 |
| walker |  | 3349 | 11 | go decl body at internal/adapters/cli/current.go:23 |  |  | 0.614 |
| walker |  | 3394 | 45 | go package + imports in internal/timeutil/timeutil.go |  |  | 0.614 |
| walker |  | 3503 | 109 | go decl names surface in internal/adapters/cli/calendar_sidebar.go |  |  | 0.615 |
| walker |  | 3503 | 0 | go decl at internal/adapters/cli/calendar_sidebar.go:15 |  |  | 0.615 |
| walker |  | 3503 | 0 | go decl at internal/adapters/cli/calendar_sidebar.go:27 |  |  | 0.615 |
| walker |  | 3503 | 0 | go decl at internal/adapters/cli/calendar_sidebar.go:51 |  |  | 0.615 |
| walker |  | 3503 | 0 | go decl at internal/adapters/cli/calendar_sidebar.go:128 |  |  | 0.615 |
| walker |  | 3503 | 0 | go decl at internal/adapters/cli/calendar_sidebar.go:211 |  |  | 0.615 |
| walker |  | 3617 | 114 | go decl at internal/config/config.go:49 |  |  | 0.616 |
| walker |  | 3648 | 31 | go decl at internal/adapters/cli/update.go:17 |  |  | 0.616 |
| walker |  | 3657 | 9 | listing of 'internal/services/activity' |  |  | 0.616 |
| walker |  | 3674 | 17 | go decl doc at internal/adapters/cli/analyze.go:84 |  |  | 0.616 |
| walker |  | 3686 | 12 | docs/commands.md section #41 |  |  | 0.616 |
| ns | 3722 |  | 549 | Config struct + sub-structs | 5.1 |  | 0.604 |
| walker |  | 3873 | 187 | go decl names surface in internal/timeutil/timeutil.go |  |  | 0.604 |
| walker |  | 3873 | 0 | go decl at internal/timeutil/timeutil.go:11 |  |  | 0.604 |
| walker |  | 3873 | 0 | go decl at internal/timeutil/timeutil.go:25 |  |  | 0.604 |
| walker |  | 3873 | 0 | go decl at internal/timeutil/timeutil.go:34 |  |  | 0.604 |
| walker |  | 3873 | 0 | go decl at internal/timeutil/timeutil.go:39 |  |  | 0.604 |
| walker |  | 3873 | 0 | go decl at internal/timeutil/timeutil.go:47 |  |  | 0.604 |
| walker |  | 3873 | 0 | go decl at internal/timeutil/timeutil.go:56 |  |  | 0.604 |
| walker |  | 3873 | 0 | go decl at internal/timeutil/timeutil.go:103 |  |  | 0.604 |
| walker |  | 3873 | 0 | go decl at internal/timeutil/timeutil.go:158 |  |  | 0.604 |
| walker |  | 3884 | 11 | go decl at internal/timeutil/timeutil.go:19 |  |  | 0.604 |
| walker |  | 3893 | 9 | go decl at internal/timeutil/timeutil.go:13 |  |  | 0.604 |
| walker |  | 3899 | 6 | go decl body at internal/timeutil/timeutil.go:34 |  |  | 0.604 |
| walker |  | 3911 | 12 | go decl doc at internal/timeutil/timeutil.go:19 |  |  | 0.604 |
| walker |  | 3925 | 14 | go decl doc at internal/timeutil/timeutil.go:11 |  |  | 0.605 |
| walker |  | 3940 | 15 | go decl doc at internal/timeutil/timeutil.go:34 |  |  | 0.605 |
| walker |  | 3956 | 16 | go decl doc at internal/timeutil/timeutil.go:47 |  |  | 0.605 |
| walker |  | 3973 | 17 | go decl doc at internal/timeutil/timeutil.go:39 |  |  | 0.605 |
| walker |  | 4009 | 36 | go decl doc at internal/timeutil/timeutil.go:56 |  |  | 0.605 |
| ns | 4047 |  | 325 | Env-var bindings (TOCK_*) | 5.2 |  | 0.591 |
| walker |  | 4051 | 42 | go decl doc at internal/timeutil/timeutil.go:25 |  |  | 0.592 |
| walker |  | 4095 | 44 | go decl doc at internal/timeutil/timeutil.go:103 |  |  | 0.592 |
| walker |  | 4205 | 110 | README.md section #7 |  |  | 0.592 |
| walker |  | 4243 | 38 | go decl body at internal/timeutil/timeutil.go:39 |  |  | 0.592 |
| walker |  | 4261 | 18 | go decl doc at internal/adapters/cli/report.go:61 |  |  | 0.592 |
| walker |  | 4380 | 119 | go decl names surface in internal/adapters/cli/theme.go |  |  | 0.592 |
| walker |  | 4380 | 0 | go decl at internal/adapters/cli/theme.go:40 |  |  | 0.592 |
| walker |  | 4380 | 0 | go decl at internal/adapters/cli/theme.go:53 |  |  | 0.592 |
| walker |  | 4380 | 0 | go decl at internal/adapters/cli/theme.go:66 |  |  | 0.592 |
| walker |  | 4380 | 0 | go decl at internal/adapters/cli/theme.go:79 |  |  | 0.592 |
| walker |  | 4380 | 0 | go decl at internal/adapters/cli/theme.go:93 |  |  | 0.592 |
| walker |  | 4380 | 0 | go decl at internal/adapters/cli/theme.go:122 |  |  | 0.592 |
| walker |  | 4380 | 0 | go decl at internal/adapters/cli/theme.go:167 |  |  | 0.592 |
| walker |  | 4396 | 16 | go decl doc at internal/adapters/cli/theme.go:167 |  |  | 0.592 |
| walker |  | 4414 | 18 | go decl doc at internal/adapters/cli/theme.go:40 |  |  | 0.592 |
| walker |  | 4432 | 18 | go decl doc at internal/adapters/cli/theme.go:53 |  |  | 0.592 |
| walker |  | 4451 | 19 | go decl doc at internal/adapters/cli/theme.go:122 |  |  | 0.592 |
| walker |  | 4471 | 20 | go decl doc at internal/adapters/cli/theme.go:66 |  |  | 0.592 |
| walker |  | 4492 | 21 | go decl doc at internal/adapters/cli/theme.go:79 |  |  | 0.592 |
| ns | 4497 |  | 450 | timeutil: Formatter type + display formats | 6.1 |  | 0.590 |
| walker |  | 4521 | 29 | go decl doc at internal/adapters/cli/theme.go:93 |  |  | 0.590 |
| walker |  | 4606 | 85 | go decl at internal/adapters/cli/theme.go:11 |  |  | 0.590 |
| walker |  | 4620 | 14 | go decl doc at internal/adapters/cli/theme.go:11 |  |  | 0.591 |
| walker |  | 4692 | 72 | go decl doc at internal/timeutil/timeutil.go:158 |  |  | 0.591 |
| walker |  | 4705 | 13 | docs/commands.md section #45 |  |  | 0.591 |
| walker |  | 4718 | 13 | docs/commands.md section #49 |  |  | 0.591 |
| walker |  | 4772 | 54 | go package + imports in internal/extra/extra.go |  |  | 0.591 |
| walker |  | 4792 | 20 | go decl doc at internal/adapters/cli/analyze.go:206 |  |  | 0.591 |
| walker |  | 4806 | 14 | docs/commands.md section #21 |  |  | 0.591 |
| walker |  | 4820 | 14 | docs/commands.md section #26 |  |  | 0.591 |
| walker |  | 4841 | 21 | go decl doc at internal/adapters/cli/calendar_sidebar.go:15 |  |  | 0.591 |
| ns | 4861 |  | 364 | TimeWarrior repo: type + filename + toTWInterval | 6.2 |  | 0.564 |
| walker |  | 4939 | 98 | go decl at internal/core/models/activity.go:10 |  |  | 0.580 |
| walker |  | 4951 | 12 | go decl doc at internal/core/models/activity.go:10 |  |  | 0.584 |
| walker |  | 4966 | 15 | docs/commands.md section #36 |  |  | 0.584 |
| walker |  | 5017 | 51 | go decl body at internal/timeutil/timeutil.go:25 |  |  | 0.596 |
| walker |  | 5057 | 40 | go package + imports in internal/core/models/activity.go |  |  | 0.596 |
| ns | 5074 |  | 213 | Notes-repo: paths + frontmatter + signatures | 6.3 |  | 0.581 |
| walker |  | 5123 | 66 | go package + imports in internal/config/config.go |  |  | 0.581 |
| walker |  | 5165 | 42 | go package + imports in internal/core/dto/activity_dto.go |  |  | 0.598 |
| ns | 5311 |  | 237 | Service method signatures | 6.4 | 4.1 | 0.591 |
| walker |  | 5328 | 163 | go decl names surface in internal/adapters/cli/watch.go |  |  | 0.594 |
| walker |  | 5328 | 0 | go decl at internal/adapters/cli/watch.go:19 |  |  | 0.594 |
| walker |  | 5328 | 0 | go decl at internal/adapters/cli/watch.go:95 |  |  | 0.594 |
| walker |  | 5328 | 0 | go decl at internal/adapters/cli/watch.go:115 |  |  | 0.594 |
| walker |  | 5328 | 0 | go decl at internal/adapters/cli/watch.go:121 |  |  | 0.594 |
| walker |  | 5328 | 0 | go decl at internal/adapters/cli/watch.go:175 |  |  | 0.594 |
| walker |  | 5328 | 0 | go decl at internal/adapters/cli/watch.go:262 |  |  | 0.594 |
| walker |  | 5350 | 22 | go decl at internal/adapters/cli/watch.go:90 |  |  | 0.594 |
| walker |  | 5355 | 5 | listing of 'demo' |  |  | 0.594 |
| walker |  | 5523 | 168 | go decl names surface in internal/adapters/cli/ical.go |  |  | 0.596 |
| walker |  | 5523 | 0 | go decl at internal/adapters/cli/ical.go:23 |  |  | 0.596 |
| walker |  | 5523 | 0 | go decl at internal/adapters/cli/ical.go:78 |  |  | 0.596 |
| walker |  | 5523 | 0 | go decl at internal/adapters/cli/ical.go:155 |  |  | 0.596 |
| walker |  | 5523 | 0 | go decl at internal/adapters/cli/ical.go:185 |  |  | 0.596 |
| walker |  | 5523 | 0 | go decl at internal/adapters/cli/ical.go:210 |  |  | 0.596 |
| walker |  | 5523 | 0 | go decl at internal/adapters/cli/ical.go:250 |  |  | 0.596 |
| walker |  | 5523 | 0 | go decl at internal/adapters/cli/ical.go:302 |  |  | 0.596 |
| walker |  | 5636 | 113 | go decl names surface in internal/adapters/repositories/file/parser.go |  |  | 0.598 |
| walker |  | 5636 | 0 | go decl at internal/adapters/repositories/file/parser.go:20 |  |  | 0.598 |
| walker |  | 5636 | 0 | go decl at internal/adapters/repositories/file/parser.go:64 |  |  | 0.598 |
| walker |  | 5636 | 0 | go decl at internal/adapters/repositories/file/parser.go:72 |  |  | 0.598 |
| walker |  | 5645 | 9 | go decl at internal/adapters/repositories/file/parser.go:15 |  |  | 0.598 |
| ns | 5652 |  | 341 | extra.CalculateEndTime | 6.5 |  | 0.579 |
| walker |  | 5697 | 52 | go decl body at internal/timeutil/timeutil.go:47 |  |  | 0.588 |
| walker |  | 5743 | 46 | go package + imports in internal/adapters/cli/version.go |  |  | 0.588 |
| ns | 5880 |  | 228 | File-repo: type + constructor | 6.6 |  | 0.572 |
| walker |  | 5911 | 168 | go decl at internal/config/config.go:12 |  |  | 0.605 |
| walker |  | 5920 | 9 | listing of 'internal/adapters/repositories/notes' |  |  | 0.605 |
| walker |  | 5929 | 9 | listing of 'internal/adapters/repositories/timewarrior' |  |  | 0.605 |
| walker |  | 6137 | 208 | go decl names surface in internal/adapters/cli/list_gui.go |  |  | 0.608 |
| walker |  | 6137 | 0 | go decl at internal/adapters/cli/list_gui.go:22 |  |  | 0.608 |
| walker |  | 6137 | 0 | go decl at internal/adapters/cli/list_gui.go:57 |  |  | 0.608 |
| walker |  | 6137 | 0 | go decl at internal/adapters/cli/list_gui.go:70 |  |  | 0.608 |
| walker |  | 6137 | 0 | go decl at internal/adapters/cli/list_gui.go:101 |  |  | 0.608 |
| walker |  | 6137 | 0 | go decl at internal/adapters/cli/list_gui.go:111 |  |  | 0.608 |
| walker |  | 6137 | 0 | go decl at internal/adapters/cli/list_gui.go:128 |  |  | 0.608 |
| walker |  | 6137 | 0 | go decl at internal/adapters/cli/list_gui.go:154 |  |  | 0.608 |
| walker |  | 6137 | 0 | go decl at internal/adapters/cli/list_gui.go:200 |  |  | 0.608 |
| walker |  | 6137 | 0 | go decl at internal/adapters/cli/list_gui.go:204 |  |  | 0.608 |
| walker |  | 6137 | 0 | go decl at internal/adapters/cli/list_gui.go:230 |  |  | 0.608 |
| walker |  | 6142 | 5 | go decl body at internal/adapters/cli/list_gui.go:200 |  |  | 0.608 |
| walker |  | 6183 | 41 | go decl body at internal/core/models/activity.go:23 |  |  | 0.613 |
| ns | 6277 |  | 397 | File-repo Find body (filter logic) | 6.7 | 6.6 | 0.590 |
| walker |  | 6326 | 143 | go decl names surface in internal/adapters/repositories/notes/repository.go |  |  | 0.599 |
| walker |  | 6326 | 0 | go decl at internal/adapters/repositories/notes/repository.go:28 |  |  | 0.599 |
| walker |  | 6326 | 0 | go decl at internal/adapters/repositories/notes/repository.go:36 |  |  | 0.599 |
| walker |  | 6326 | 0 | go decl at internal/adapters/repositories/notes/repository.go:73 |  |  | 0.599 |
| walker |  | 6337 | 11 | go decl at internal/adapters/repositories/notes/repository.go:24 |  |  | 0.602 |
| walker |  | 6346 | 9 | go decl at internal/adapters/repositories/notes/repository.go:18 |  |  | 0.603 |
| walker |  | 6358 | 12 | go decl body at internal/adapters/repositories/notes/repository.go:28 |  |  | 0.605 |
| walker |  | 6375 | 17 | go decl at internal/adapters/repositories/notes/repository.go:32 |  |  | 0.609 |
| walker |  | 6432 | 57 | go package + imports in internal/services/ics/generator.go |  |  | 0.609 |
| walker |  | 6495 | 63 | go package + imports in internal/adapters/cli/calendar_sidebar.go |  |  | 0.609 |
| walker |  | 6560 | 65 | go package + imports in internal/adapters/cli/theme.go |  |  | 0.609 |
| walker |  | 6728 | 168 | go decl names surface in internal/adapters/repositories/file/repository.go |  |  | 0.610 |
| walker |  | 6728 | 0 | go decl at internal/adapters/repositories/file/repository.go:23 |  |  | 0.610 |
| walker |  | 6728 | 0 | go decl at internal/adapters/repositories/file/repository.go:27 |  |  | 0.610 |
| walker |  | 6728 | 0 | go decl at internal/adapters/repositories/file/repository.go:93 |  |  | 0.610 |
| walker |  | 6728 | 0 | go decl at internal/adapters/repositories/file/repository.go:130 |  |  | 0.610 |
| walker |  | 6728 | 0 | go decl at internal/adapters/repositories/file/repository.go:168 |  |  | 0.610 |
| walker |  | 6728 | 0 | go decl at internal/adapters/repositories/file/repository.go:229 |  |  | 0.610 |
| walker |  | 6728 | 0 | go decl at internal/adapters/repositories/file/repository.go:250 |  |  | 0.610 |
| walker |  | 6739 | 11 | go decl at internal/adapters/repositories/file/repository.go:19 |  |  | 0.610 |
| walker |  | 6751 | 12 | go decl body at internal/adapters/repositories/file/repository.go:23 |  |  | 0.611 |
| ns | 6789 |  | 512 | cli/root.go — context keys + NewRootCmd shell | 7.1 |  | 0.587 |
| walker |  | 6795 | 44 | go package + imports in internal/core/ports/mocks/mock_notesrepository.go |  |  | 0.587 |
| walker |  | 6862 | 67 | go package + imports in internal/core/ports/ports.go |  |  | 0.590 |
| ns | 6956 |  | 167 | cli/root.go — AddCommand block | 7.2 | 7.1 | 0.581 |
| walker |  | 7015 | 153 | go decl at internal/adapters/cli/analyze.go:70 |  |  | 0.581 |
| walker |  | 7038 | 23 | docs/commands.md section #19 |  |  | 0.581 |
| walker |  | 7300 | 262 | go decl names surface in internal/adapters/cli/root.go |  |  | 0.583 |
| walker |  | 7300 | 0 | go decl at internal/adapters/cli/root.go:31 |  |  | 0.583 |
| walker |  | 7300 | 0 | go decl at internal/adapters/cli/root.go:107 |  |  | 0.583 |
| walker |  | 7300 | 0 | go decl at internal/adapters/cli/root.go:115 |  |  | 0.583 |
| walker |  | 7300 | 0 | go decl at internal/adapters/cli/root.go:119 |  |  | 0.583 |
| walker |  | 7300 | 0 | go decl at internal/adapters/cli/root.go:123 |  |  | 0.583 |
| walker |  | 7300 | 0 | go decl at internal/adapters/cli/root.go:127 |  |  | 0.583 |
| walker |  | 7300 | 0 | go decl at internal/adapters/cli/root.go:134 |  |  | 0.583 |
| walker |  | 7300 | 0 | go decl at internal/adapters/cli/root.go:173 |  |  | 0.583 |
| walker |  | 7300 | 0 | go decl at internal/adapters/cli/root.go:195 |  |  | 0.583 |
| walker |  | 7309 | 9 | go decl at internal/adapters/cli/root.go:21 |  |  | 0.584 |
| walker |  | 7363 | 54 | go decl body at internal/adapters/cli/root.go:107 |  |  | 0.584 |
| ns | 7464 |  | 508 | cli/root.go — PersistentPreRunE body | 7.3 | 7.1 | 0.561 |
| walker |  | 7522 | 159 | go decl at internal/core/ports/ports.go:11 |  |  | 0.585 |
| ns | 7530 |  | 66 | cli/root.go — initRepository (file/timewarrior switch) | 7.4 | 7.3 | 0.583 |
| walker |  | 7547 | 25 | go decl body at internal/adapters/cli/root.go:119 |  |  | 0.583 |
| walker |  | 7805 | 258 | docs/commands.md section #0 |  |  | 0.583 |
| walker |  | 7870 | 65 | go decl at internal/adapters/cli/add.go:16 |  |  | 0.583 |
| walker |  | 7945 | 75 | go package + imports in internal/adapters/cli/stop.go |  |  | 0.583 |
| walker |  | 7971 | 26 | go decl body at internal/adapters/cli/root.go:115 |  |  | 0.583 |
| ns | 8014 |  | 484 | Service Start body | 7.5 | 6.4 | 0.563 |
| walker |  | 8261 | 290 | go decl names surface in internal/services/activity/service.go |  |  | 0.574 |
| walker |  | 8261 | 0 | go decl at internal/services/activity/service.go:20 |  |  | 0.574 |
| walker |  | 8261 | 0 | go decl at internal/services/activity/service.go:24 |  |  | 0.574 |
| walker |  | 8261 | 0 | go decl at internal/services/activity/service.go:68 |  |  | 0.574 |
| walker |  | 8261 | 0 | go decl at internal/services/activity/service.go:118 |  |  | 0.574 |
| walker |  | 8261 | 0 | go decl at internal/services/activity/service.go:141 |  |  | 0.574 |
| walker |  | 8261 | 0 | go decl at internal/services/activity/service.go:149 |  |  | 0.574 |
| walker |  | 8261 | 0 | go decl at internal/services/activity/service.go:186 |  |  | 0.574 |
| walker |  | 8261 | 0 | go decl at internal/services/activity/service.go:210 |  |  | 0.574 |
| walker |  | 8261 | 0 | go decl at internal/services/activity/service.go:214 |  |  | 0.574 |
| walker |  | 8261 | 0 | go decl at internal/services/activity/service.go:218 |  |  | 0.574 |
| walker |  | 8277 | 16 | go decl body at internal/services/activity/service.go:20 |  |  | 0.575 |
| walker |  | 8286 | 9 | go decl body at internal/services/activity/service.go:210 |  |  | 0.575 |
| walker |  | 8312 | 26 | go decl at internal/services/activity/service.go:15 |  |  | 0.580 |
| walker |  | 8322 | 10 | go decl body at internal/services/activity/service.go:214 |  |  | 0.580 |
| walker |  | 8347 | 25 | docs/commands.md section #23 |  |  | 0.580 |
| ns | 8456 |  | 442 | start/stop/add flag declarations | 8.1 | 3.3 | 0.570 |
| walker |  | 8639 | 292 | go decl names surface in internal/adapters/cli/calendar.go |  |  | 0.573 |
| walker |  | 8639 | 0 | go decl at internal/adapters/cli/calendar.go:25 |  |  | 0.573 |
| walker |  | 8639 | 0 | go decl at internal/adapters/cli/calendar.go:64 |  |  | 0.573 |
| walker |  | 8639 | 0 | go decl at internal/adapters/cli/calendar.go:79 |  |  | 0.573 |
| walker |  | 8639 | 0 | go decl at internal/adapters/cli/calendar.go:83 |  |  | 0.573 |
| walker |  | 8639 | 0 | go decl at internal/adapters/cli/calendar.go:134 |  |  | 0.573 |
| walker |  | 8639 | 0 | go decl at internal/adapters/cli/calendar.go:156 |  |  | 0.573 |
| walker |  | 8639 | 0 | go decl at internal/adapters/cli/calendar.go:234 |  |  | 0.573 |
| walker |  | 8639 | 0 | go decl at internal/adapters/cli/calendar.go:256 |  |  | 0.573 |
| walker |  | 8639 | 0 | go decl at internal/adapters/cli/calendar.go:402 |  |  | 0.573 |
| walker |  | 8639 | 0 | go decl at internal/adapters/cli/calendar.go:424 |  |  | 0.573 |
| walker |  | 8639 | 0 | go decl at internal/adapters/cli/calendar.go:488 |  |  | 0.573 |
| walker |  | 8639 | 0 | go decl at internal/adapters/cli/calendar.go:520 |  |  | 0.573 |
| walker |  | 8654 | 15 | go decl at internal/adapters/cli/calendar.go:418 |  |  | 0.573 |
| walker |  | 8662 | 8 | go decl body at internal/adapters/cli/calendar.go:79 |  |  | 0.573 |
| walker |  | 8687 | 25 | go decl doc at internal/adapters/cli/calendar.go:256 |  |  | 0.573 |
| walker |  | 8858 | 171 | go decl at internal/adapters/cli/theme.go:22 |  |  | 0.574 |
| ns | 8859 |  | 403 | Theme/Styles fields + theme constructor names | 8.2 |  | 0.587 |
| walker |  | 8876 | 18 | go decl doc at internal/adapters/cli/theme.go:22 |  |  | 0.590 |
| walker |  | 8919 | 43 | go decl doc at internal/adapters/cli/calendar.go:488 |  |  | 0.590 |
| walker |  | 8947 | 28 | go decl body at internal/adapters/cli/root.go:123 |  |  | 0.590 |
| walker |  | 8974 | 27 | docs/commands.md section #29 |  |  | 0.590 |
| walker |  | 9048 | 74 | go decl at internal/adapters/cli/report.go:20 |  |  | 0.590 |
| walker |  | 9137 | 89 | go package + imports in internal/adapters/cli/continue.go |  |  | 0.590 |
| walker |  | 9166 | 29 | docs/commands.md section #32 |  |  | 0.590 |
| ns | 9185 |  | 326 | report flag declarations + reportOptions | 9.1 | 3.3 | 0.583 |
| walker |  | 9195 | 29 | docs/commands.md section #37 |  |  | 0.583 |
| ns | 9203 |  | 18 | Mocks directory listing | 9.2 |  | 0.584 |
| walker |  | 9290 | 95 | go package + imports in internal/adapters/cli/start.go |  |  | 0.584 |
| ns | 9346 |  | 143 | ICS generator — public fn signatures | 9.3 |  | 0.586 |
| ns | 9507 |  | 161 | Interactive helpers — three public fns | 9.4 |  | 0.583 |
| ns | 9600 |  | 93 | calendar.go: handleKeyMsg case lines | 9.5 |  | 0.580 |
| walker |  | 9661 | 371 | go decl names surface in internal/adapters/cli/interactive.go |  |  | 0.586 |
| walker |  | 9661 | 0 | go decl at internal/adapters/cli/interactive.go:17 |  |  | 0.586 |
| walker |  | 9661 | 0 | go decl at internal/adapters/cli/interactive.go:36 |  |  | 0.586 |
| walker |  | 9661 | 0 | go decl at internal/adapters/cli/interactive.go:57 |  |  | 0.586 |
| walker |  | 9661 | 0 | go decl at internal/adapters/cli/interactive.go:80 |  |  | 0.586 |
| walker |  | 9661 | 0 | go decl at internal/adapters/cli/interactive.go:89 |  |  | 0.586 |
| walker |  | 9661 | 0 | go decl at internal/adapters/cli/interactive.go:98 |  |  | 0.586 |
| walker |  | 9661 | 0 | go decl at internal/adapters/cli/interactive.go:133 |  |  | 0.586 |
| walker |  | 9661 | 0 | go decl at internal/adapters/cli/interactive.go:155 |  |  | 0.586 |
| walker |  | 9661 | 0 | go decl at internal/adapters/cli/interactive.go:176 |  |  | 0.586 |
| walker |  | 9661 | 0 | go decl at internal/adapters/cli/interactive.go:214 |  |  | 0.586 |
| walker |  | 9661 | 0 | go decl at internal/adapters/cli/interactive.go:251 |  |  | 0.586 |
| walker |  | 9661 | 0 | go decl at internal/adapters/cli/interactive.go:263 |  |  | 0.586 |
| walker |  | 9661 | 0 | go decl at internal/adapters/cli/interactive.go:301 |  |  | 0.586 |
| ns | 9671 |  | 71 | calendar_sidebar.go: section renderer signatures | 9.6 |  | 0.588 |
| walker |  | 9707 | 46 | go decl at internal/adapters/cli/interactive.go:123 |  |  | 0.588 |
| walker |  | 9776 | 69 | go decl at internal/adapters/cli/interactive.go:201 |  |  | 0.588 |
| walker |  | 9876 | 100 | go package + imports in internal/adapters/cli/last.go |  |  | 0.588 |
| walker |  | 9907 | 31 | docs/commands.md section #7 |  |  | 0.588 |
| walker |  | 9938 | 31 | docs/commands.md section #12 |  |  | 0.588 |
| ns | 9955 |  | 284 | Remaining commands' flag declarations | 9.7 | 3.3 | 0.583 |
| walker |  | 9969 | 31 | docs/commands.md section #27 |  |  | 0.583 |
| walker |  | 10000 | 31 | docs/commands.md section #42 |  |  | 0.583 |

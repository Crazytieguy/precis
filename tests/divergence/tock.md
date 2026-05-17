Score(3000)=0.655 I=0.806 C=0.532 ns_rows≤3K=18/43 (reached=6 partial=5 missing=7)

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
| walker |  | 192 | 19 | listing of 'internal' |  |  | 0.690 |
| walker |  | 212 | 20 | listing of 'assets' |  |  | 0.690 |
| walker |  | 224 | 12 | listing of 'internal/core' |  |  | 0.780 |
| walker |  | 244 | 20 | README.md section #11 |  |  | 0.780 |
| ns | 316 |  | 138 | README feature lede (rest) | 1.5 | 1.3 | 0.711 |
| ns | 435 |  | 119 | go.mod module + Go version + UI deps | 1.6 |  | 0.645 |
| walker |  | 492 | 248 | go module file go.mod |  |  | 0.738 |
| walker |  | 496 | 4 | listing of 'cmd/tock' |  |  | 0.738 |
| walker |  | 506 | 10 | go decl names surface in cmd/tock/main.go |  |  | 0.738 |
| walker |  | 506 | 0 | go decl at cmd/tock/main.go:7 |  |  | 0.738 |
| walker |  | 512 | 6 | go decl body at cmd/tock/main.go:7 |  |  | 0.738 |
| walker |  | 518 | 6 | listing of 'internal/adapters' |  |  | 0.784 |
| walker |  | 529 | 11 | listing of 'internal/adapters/repositories' |  |  | 0.784 |
| walker |  | 547 | 18 | listing of 'internal/adapters/repositories/file' |  |  | 0.784 |
| walker |  | 554 | 7 | listing of 'internal/services' |  |  | 0.837 |
| walker |  | 558 | 4 | listing of 'internal/core/errors' |  |  | 0.837 |
| walker |  | 562 | 4 | listing of 'internal/core/models' |  |  | 0.837 |
| walker |  | 566 | 4 | listing of 'internal/services/ics' |  |  | 0.837 |
| walker |  | 581 | 15 | go package + imports in internal/core/errors/errors.go |  |  | 0.837 |
| ns | 589 |  | 154 | go.mod remaining direct deps | 1.7 | 1.6 | 0.843 |
| walker |  | 590 | 9 | listing of 'internal/config' |  |  | 0.843 |
| walker |  | 599 | 9 | listing of 'internal/extra' |  |  | 0.843 |
| walker |  | 635 | 36 | go decl names surface in internal/extra/extra.go |  |  | 0.843 |
| walker |  | 635 | 0 | go decl at internal/extra/extra.go:11 |  |  | 0.843 |
| ns | 690 |  | 101 | Sentinel errors (whole file) | 2.1 |  | 0.770 |
| walker |  | 701 | 66 | go decl names surface in internal/services/ics/generator.go |  |  | 0.770 |
| walker |  | 701 | 0 | go decl at internal/services/ics/generator.go:12 |  |  | 0.770 |
| walker |  | 701 | 0 | go decl at internal/services/ics/generator.go:18 |  |  | 0.770 |
| walker |  | 701 | 0 | go decl at internal/services/ics/generator.go:29 |  |  | 0.770 |
| walker |  | 701 | 0 | go decl at internal/services/ics/generator.go:71 |  |  | 0.770 |
| walker |  | 718 | 17 | go decl doc at internal/services/ics/generator.go:12 |  |  | 0.770 |
| walker |  | 735 | 17 | go decl doc at internal/services/ics/generator.go:29 |  |  | 0.770 |
| walker |  | 755 | 20 | go decl doc at internal/services/ics/generator.go:18 |  |  | 0.770 |
| ns | 810 |  | 120 | models.Activity struct fields | 2.2 |  | 0.716 |
| walker |  | 822 | 67 | go decl names surface in internal/core/errors/errors.go |  |  | 0.774 |
| walker |  | 831 | 9 | go decl at internal/core/errors/errors.go:5 |  |  | 0.784 |
| walker |  | 910 | 79 | README.md section #6 |  |  | 0.784 |
| walker |  | 930 | 20 | go decl body at internal/services/ics/generator.go:12 |  |  | 0.784 |
| walker |  | 936 | 6 | listing of 'internal/core/dto' |  |  | 0.784 |
| walker |  | 1004 | 68 | go decl names surface in internal/core/dto/activity_dto.go |  |  | 0.785 |
| walker |  | 1035 | 31 | go decl at internal/core/dto/activity_dto.go:17 |  |  | 0.786 |
| ns | 1062 |  | 252 | ports.ActivityResolver interface | 2.3 |  | 0.686 |
| walker |  | 1068 | 33 | go decl at internal/core/dto/activity_dto.go:46 |  |  | 0.688 |
| walker |  | 1106 | 38 | go decl at internal/core/dto/activity_dto.go:40 |  |  | 0.690 |
| walker |  | 1154 | 48 | go decl at internal/core/dto/activity_dto.go:9 |  |  | 0.693 |
| walker |  | 1209 | 55 | go decl at internal/core/dto/activity_dto.go:32 |  |  | 0.697 |
| ns | 1217 |  | 155 | ports.ActivityRepository + NotesRepository | 2.4 | 2.3 | 0.655 |
| walker |  | 1267 | 58 | go decl at internal/core/dto/activity_dto.go:23 |  |  | 0.661 |
| walker |  | 1278 | 11 | listing of 'internal/timeutil' |  |  | 0.661 |
| walker |  | 1355 | 77 | go decl names surface in internal/core/models/activity.go |  |  | 0.661 |
| walker |  | 1355 | 0 | go decl at internal/core/models/activity.go:19 |  |  | 0.661 |
| walker |  | 1355 | 0 | go decl at internal/core/models/activity.go:23 |  |  | 0.661 |
| walker |  | 1355 | 0 | go decl at internal/core/models/activity.go:31 |  |  | 0.661 |
| walker |  | 1355 | 0 | go decl at internal/core/models/activity.go:41 |  |  | 0.661 |
| walker |  | 1366 | 11 | go decl body at internal/core/models/activity.go:19 |  |  | 0.662 |
| walker |  | 1385 | 19 | go decl doc at internal/core/models/activity.go:31 |  |  | 0.662 |
| walker |  | 1422 | 37 | go package + imports in cmd/tock/main.go |  |  | 0.664 |
| walker |  | 1429 | 7 | listing of 'internal/core/ports' |  |  | 0.664 |
| walker |  | 1462 | 33 | go decl names surface in internal/core/ports/ports.go |  |  | 0.666 |
| walker |  | 1517 | 55 | go decl at internal/core/ports/ports.go:29 |  |  | 0.681 |
| walker |  | 1602 | 85 | README.md section #10 |  |  | 0.681 |
| ns | 1630 |  | 413 | dto request/filter/report types | 2.5 |  | 0.685 |
| walker |  | 1765 | 163 | go decl names surface in internal/config/config.go |  |  | 0.685 |
| walker |  | 1765 | 0 | go decl at internal/config/config.go:62 |  |  | 0.685 |
| walker |  | 1765 | 0 | go decl at internal/config/config.go:68 |  |  | 0.685 |
| walker |  | 1765 | 0 | go decl at internal/config/config.go:74 |  |  | 0.685 |
| walker |  | 1765 | 0 | go decl at internal/config/config.go:80 |  |  | 0.685 |
| walker |  | 1780 | 15 | go decl at internal/config/config.go:41 |  |  | 0.685 |
| walker |  | 1797 | 17 | go decl at internal/config/config.go:37 |  |  | 0.686 |
| walker |  | 1814 | 17 | go decl at internal/config/config.go:45 |  |  | 0.686 |
| walker |  | 1833 | 19 | go decl at internal/config/config.go:33 |  |  | 0.686 |
| ns | 1858 |  | 228 | models.Activity helper methods | 2.6 | 2.2 | 0.646 |
| walker |  | 1862 | 29 | go decl body at internal/config/config.go:62 |  |  | 0.646 |
| walker |  | 1891 | 29 | go decl body at internal/config/config.go:68 |  |  | 0.646 |
| walker |  | 1920 | 29 | go decl body at internal/config/config.go:74 |  |  | 0.646 |
| ns | 1926 |  | 68 | main.go — entry point | 3.1 |  | 0.647 |
| ns | 2005 |  | 79 | cli/ directory listing | 3.2 |  | 0.612 |
| walker |  | 2010 | 90 | go decl at internal/config/config.go:25 |  |  | 0.613 |
| walker |  | 2089 | 79 | listing of 'internal/adapters/cli' |  |  | 0.676 |
| walker |  | 2104 | 15 | go decl names surface in internal/adapters/cli/start.go |  |  | 0.676 |
| walker |  | 2104 | 0 | go decl at internal/adapters/cli/start.go:17 |  |  | 0.676 |
| walker |  | 2119 | 15 | go decl names surface in internal/adapters/cli/stop.go |  |  | 0.676 |
| walker |  | 2119 | 0 | go decl at internal/adapters/cli/stop.go:14 |  |  | 0.676 |
| walker |  | 2149 | 30 | go decl names surface in internal/adapters/cli/continue.go |  |  | 0.676 |
| walker |  | 2149 | 0 | go decl at internal/adapters/cli/continue.go:20 |  |  | 0.676 |
| walker |  | 2158 | 9 | go decl at internal/adapters/cli/continue.go:15 |  |  | 0.676 |
| ns | 2188 |  | 183 | Cobra command factory locations across cli/ | 3.3 |  | 0.653 |
| walker |  | 2202 | 44 | go decl names surface in internal/adapters/cli/update.go |  |  | 0.653 |
| walker |  | 2202 | 0 | go decl at internal/adapters/cli/update.go:22 |  |  | 0.653 |
| walker |  | 2202 | 0 | go decl at internal/adapters/cli/update.go:45 |  |  | 0.653 |
| walker |  | 2249 | 47 | go decl names surface in internal/adapters/cli/version.go |  |  | 0.655 |
| walker |  | 2249 | 0 | go decl at internal/adapters/cli/version.go:16 |  |  | 0.655 |
| walker |  | 2258 | 9 | go decl at internal/adapters/cli/version.go:10 |  |  | 0.655 |
| walker |  | 2306 | 48 | go decl names surface in internal/adapters/cli/last.go |  |  | 0.657 |
| walker |  | 2306 | 0 | go decl at internal/adapters/cli/last.go:21 |  |  | 0.657 |
| walker |  | 2306 | 0 | go decl at internal/adapters/cli/last.go:42 |  |  | 0.657 |
| walker |  | 2354 | 48 | go decl names surface in internal/adapters/cli/report.go |  |  | 0.659 |
| walker |  | 2354 | 0 | go decl at internal/adapters/cli/report.go:31 |  |  | 0.659 |
| walker |  | 2354 | 0 | go decl at internal/adapters/cli/report.go:61 |  |  | 0.659 |
| walker |  | 2372 | 18 | go decl doc at internal/adapters/cli/continue.go:20 |  |  | 0.659 |
| walker |  | 2394 | 22 | go decl doc at internal/adapters/cli/start.go:17 |  |  | 0.659 |
| ns | 2423 |  | 235 | README commands list (Use/Short for every cmd) | 3.4 |  | 0.630 |
| walker |  | 2458 | 64 | go decl names surface in internal/adapters/cli/current.go |  |  | 0.633 |
| walker |  | 2458 | 0 | go decl at internal/adapters/cli/current.go:23 |  |  | 0.633 |
| walker |  | 2458 | 0 | go decl at internal/adapters/cli/current.go:27 |  |  | 0.633 |
| walker |  | 2458 | 0 | go decl at internal/adapters/cli/current.go:35 |  |  | 0.633 |
| walker |  | 2469 | 11 | go decl at internal/adapters/cli/current.go:19 |  |  | 0.633 |
| walker |  | 2490 | 21 | go decl at internal/adapters/cli/last.go:16 |  |  | 0.633 |
| ns | 2510 |  | 87 | Activity service constructor | 4.1 |  | 0.621 |
| walker |  | 2560 | 70 | go decl names surface in internal/adapters/cli/analyze.go |  |  | 0.624 |
| walker |  | 2560 | 0 | go decl at internal/adapters/cli/analyze.go:20 |  |  | 0.624 |
| walker |  | 2560 | 0 | go decl at internal/adapters/cli/analyze.go:84 |  |  | 0.624 |
| walker |  | 2560 | 0 | go decl at internal/adapters/cli/analyze.go:206 |  |  | 0.624 |
| walker |  | 2653 | 93 | go decl names surface in internal/adapters/cli/remove.go |  |  | 0.627 |
| walker |  | 2653 | 0 | go decl at internal/adapters/cli/remove.go:22 |  |  | 0.627 |
| walker |  | 2653 | 0 | go decl at internal/adapters/cli/remove.go:95 |  |  | 0.627 |
| walker |  | 2653 | 0 | go decl at internal/adapters/cli/remove.go:116 |  |  | 0.627 |
| walker |  | 2653 | 0 | go decl at internal/adapters/cli/remove.go:127 |  |  | 0.627 |
| walker |  | 2753 | 100 | go decl names surface in internal/adapters/cli/add.go |  |  | 0.631 |
| walker |  | 2753 | 0 | go decl at internal/adapters/cli/add.go:26 |  |  | 0.631 |
| walker |  | 2753 | 0 | go decl at internal/adapters/cli/add.go:64 |  |  | 0.631 |
| walker |  | 2753 | 0 | go decl at internal/adapters/cli/add.go:124 |  |  | 0.631 |
| walker |  | 2753 | 0 | go decl at internal/adapters/cli/add.go:163 |  |  | 0.631 |
| walker |  | 2826 | 73 | go decl at internal/core/ports/ports.go:22 |  |  | 0.655 |
| walker |  | 2844 | 18 | listing of 'internal/core/ports/mocks' |  |  | 0.655 |
| walker |  | 2855 | 11 | go decl body at internal/adapters/cli/current.go:23 |  |  | 0.655 |
| walker |  | 2900 | 45 | go package + imports in internal/timeutil/timeutil.go |  |  | 0.655 |
| walker |  | 3009 | 109 | go decl names surface in internal/adapters/cli/calendar_sidebar.go |  |  | 0.655 |
| walker |  | 3009 | 0 | go decl at internal/adapters/cli/calendar_sidebar.go:15 |  |  | 0.655 |
| walker |  | 3009 | 0 | go decl at internal/adapters/cli/calendar_sidebar.go:27 |  |  | 0.655 |
| walker |  | 3009 | 0 | go decl at internal/adapters/cli/calendar_sidebar.go:51 |  |  | 0.655 |
| walker |  | 3009 | 0 | go decl at internal/adapters/cli/calendar_sidebar.go:128 |  |  | 0.655 |
| walker |  | 3009 | 0 | go decl at internal/adapters/cli/calendar_sidebar.go:211 |  |  | 0.655 |
| ns | 3040 |  | 530 | File-format ParseActivity | 4.2 |  | 0.589 |
| walker |  | 3123 | 114 | go decl at internal/config/config.go:49 |  |  | 0.591 |
| walker |  | 3154 | 31 | go decl at internal/adapters/cli/update.go:17 |  |  | 0.591 |
| walker |  | 3163 | 9 | listing of 'internal/services/activity' |  |  | 0.591 |
| ns | 3173 |  | 133 | File-format FormatActivity (writer side) | 4.3 | 4.2 | 0.580 |
| walker |  | 3180 | 17 | go decl doc at internal/adapters/cli/analyze.go:84 |  |  | 0.580 |
| walker |  | 3367 | 187 | go decl names surface in internal/timeutil/timeutil.go |  |  | 0.580 |
| walker |  | 3367 | 0 | go decl at internal/timeutil/timeutil.go:11 |  |  | 0.580 |
| walker |  | 3367 | 0 | go decl at internal/timeutil/timeutil.go:25 |  |  | 0.580 |
| walker |  | 3367 | 0 | go decl at internal/timeutil/timeutil.go:34 |  |  | 0.580 |
| walker |  | 3367 | 0 | go decl at internal/timeutil/timeutil.go:39 |  |  | 0.580 |
| walker |  | 3367 | 0 | go decl at internal/timeutil/timeutil.go:47 |  |  | 0.580 |
| walker |  | 3367 | 0 | go decl at internal/timeutil/timeutil.go:56 |  |  | 0.580 |
| walker |  | 3367 | 0 | go decl at internal/timeutil/timeutil.go:103 |  |  | 0.580 |
| walker |  | 3367 | 0 | go decl at internal/timeutil/timeutil.go:158 |  |  | 0.580 |
| walker |  | 3378 | 11 | go decl at internal/timeutil/timeutil.go:19 |  |  | 0.580 |
| walker |  | 3387 | 9 | go decl at internal/timeutil/timeutil.go:13 |  |  | 0.580 |
| walker |  | 3393 | 6 | go decl body at internal/timeutil/timeutil.go:34 |  |  | 0.580 |
| walker |  | 3405 | 12 | go decl doc at internal/timeutil/timeutil.go:19 |  |  | 0.580 |
| walker |  | 3419 | 14 | go decl doc at internal/timeutil/timeutil.go:11 |  |  | 0.581 |
| walker |  | 3434 | 15 | go decl doc at internal/timeutil/timeutil.go:34 |  |  | 0.581 |
| walker |  | 3450 | 16 | go decl doc at internal/timeutil/timeutil.go:47 |  |  | 0.581 |
| walker |  | 3467 | 17 | go decl doc at internal/timeutil/timeutil.go:39 |  |  | 0.581 |
| walker |  | 3503 | 36 | go decl doc at internal/timeutil/timeutil.go:56 |  |  | 0.581 |
| walker |  | 3545 | 42 | go decl doc at internal/timeutil/timeutil.go:25 |  |  | 0.582 |
| walker |  | 3589 | 44 | go decl doc at internal/timeutil/timeutil.go:103 |  |  | 0.582 |
| walker |  | 3699 | 110 | README.md section #7 |  |  | 0.582 |
| ns | 3722 |  | 549 | Config struct + sub-structs | 5.1 |  | 0.572 |
| walker |  | 3737 | 38 | go decl body at internal/timeutil/timeutil.go:39 |  |  | 0.573 |
| walker |  | 3755 | 18 | go decl doc at internal/adapters/cli/report.go:61 |  |  | 0.573 |
| walker |  | 3874 | 119 | go decl names surface in internal/adapters/cli/theme.go |  |  | 0.573 |
| walker |  | 3874 | 0 | go decl at internal/adapters/cli/theme.go:40 |  |  | 0.573 |
| walker |  | 3874 | 0 | go decl at internal/adapters/cli/theme.go:53 |  |  | 0.573 |
| walker |  | 3874 | 0 | go decl at internal/adapters/cli/theme.go:66 |  |  | 0.573 |
| walker |  | 3874 | 0 | go decl at internal/adapters/cli/theme.go:79 |  |  | 0.573 |
| walker |  | 3874 | 0 | go decl at internal/adapters/cli/theme.go:93 |  |  | 0.573 |
| walker |  | 3874 | 0 | go decl at internal/adapters/cli/theme.go:122 |  |  | 0.573 |
| walker |  | 3874 | 0 | go decl at internal/adapters/cli/theme.go:167 |  |  | 0.573 |
| walker |  | 3890 | 16 | go decl doc at internal/adapters/cli/theme.go:167 |  |  | 0.573 |
| walker |  | 3908 | 18 | go decl doc at internal/adapters/cli/theme.go:40 |  |  | 0.573 |
| walker |  | 3926 | 18 | go decl doc at internal/adapters/cli/theme.go:53 |  |  | 0.573 |
| walker |  | 3945 | 19 | go decl doc at internal/adapters/cli/theme.go:122 |  |  | 0.573 |
| walker |  | 3965 | 20 | go decl doc at internal/adapters/cli/theme.go:66 |  |  | 0.573 |
| walker |  | 3986 | 21 | go decl doc at internal/adapters/cli/theme.go:79 |  |  | 0.573 |
| walker |  | 4015 | 29 | go decl doc at internal/adapters/cli/theme.go:93 |  |  | 0.573 |
| ns | 4047 |  | 325 | Env-var bindings (TOCK_*) | 5.2 |  | 0.559 |
| walker |  | 4100 | 85 | go decl at internal/adapters/cli/theme.go:11 |  |  | 0.560 |
| walker |  | 4114 | 14 | go decl doc at internal/adapters/cli/theme.go:11 |  |  | 0.560 |
| walker |  | 4186 | 72 | go decl doc at internal/timeutil/timeutil.go:158 |  |  | 0.560 |
| walker |  | 4240 | 54 | go package + imports in internal/extra/extra.go |  |  | 0.560 |
| walker |  | 4260 | 20 | go decl doc at internal/adapters/cli/analyze.go:206 |  |  | 0.560 |
| walker |  | 4497 | 237 | README.md section #1 |  |  | 0.591 |
| ns | 4497 |  | 450 | timeutil: Formatter type + display formats | 6.1 |  | 0.591 |
| walker |  | 4518 | 21 | go decl doc at internal/adapters/cli/calendar_sidebar.go:15 |  |  | 0.591 |
| walker |  | 4616 | 98 | go decl at internal/core/models/activity.go:10 |  |  | 0.607 |
| walker |  | 4628 | 12 | go decl doc at internal/core/models/activity.go:10 |  |  | 0.612 |
| walker |  | 4679 | 51 | go decl body at internal/timeutil/timeutil.go:25 |  |  | 0.625 |
| walker |  | 4719 | 40 | go package + imports in internal/core/models/activity.go |  |  | 0.625 |
| walker |  | 4785 | 66 | go package + imports in internal/config/config.go |  |  | 0.625 |
| walker |  | 4827 | 42 | go package + imports in internal/core/dto/activity_dto.go |  |  | 0.642 |
| walker |  | 4832 | 5 | listing of 'demo' |  |  | 0.642 |
| ns | 4861 |  | 364 | TimeWarrior repo: type + filename + toTWInterval | 6.2 |  | 0.613 |
| walker |  | 4997 | 165 | go decl names surface in internal/adapters/cli/watch.go |  |  | 0.615 |
| walker |  | 4997 | 0 | go decl at internal/adapters/cli/watch.go:19 |  |  | 0.615 |
| walker |  | 4997 | 0 | go decl at internal/adapters/cli/watch.go:95 |  |  | 0.615 |
| walker |  | 4997 | 0 | go decl at internal/adapters/cli/watch.go:115 |  |  | 0.615 |
| walker |  | 4997 | 0 | go decl at internal/adapters/cli/watch.go:121 |  |  | 0.615 |
| walker |  | 4997 | 0 | go decl at internal/adapters/cli/watch.go:175 |  |  | 0.615 |
| walker |  | 4997 | 0 | go decl at internal/adapters/cli/watch.go:262 |  |  | 0.615 |
| walker |  | 5019 | 22 | go decl at internal/adapters/cli/watch.go:90 |  |  | 0.615 |
| ns | 5074 |  | 213 | Notes-repo: paths + frontmatter + signatures | 6.3 |  | 0.600 |
| walker |  | 5187 | 168 | go decl names surface in internal/adapters/cli/ical.go |  |  | 0.603 |
| walker |  | 5187 | 0 | go decl at internal/adapters/cli/ical.go:23 |  |  | 0.603 |
| walker |  | 5187 | 0 | go decl at internal/adapters/cli/ical.go:78 |  |  | 0.603 |
| walker |  | 5187 | 0 | go decl at internal/adapters/cli/ical.go:155 |  |  | 0.603 |
| walker |  | 5187 | 0 | go decl at internal/adapters/cli/ical.go:185 |  |  | 0.603 |
| walker |  | 5187 | 0 | go decl at internal/adapters/cli/ical.go:210 |  |  | 0.603 |
| walker |  | 5187 | 0 | go decl at internal/adapters/cli/ical.go:250 |  |  | 0.603 |
| walker |  | 5187 | 0 | go decl at internal/adapters/cli/ical.go:302 |  |  | 0.603 |
| walker |  | 5300 | 113 | go decl names surface in internal/adapters/repositories/file/parser.go |  |  | 0.605 |
| walker |  | 5300 | 0 | go decl at internal/adapters/repositories/file/parser.go:20 |  |  | 0.605 |
| walker |  | 5300 | 0 | go decl at internal/adapters/repositories/file/parser.go:64 |  |  | 0.605 |
| walker |  | 5300 | 0 | go decl at internal/adapters/repositories/file/parser.go:72 |  |  | 0.605 |
| walker |  | 5309 | 9 | go decl at internal/adapters/repositories/file/parser.go:15 |  |  | 0.605 |
| ns | 5311 |  | 237 | Service method signatures | 6.4 | 4.1 | 0.598 |
| walker |  | 5361 | 52 | go decl body at internal/timeutil/timeutil.go:47 |  |  | 0.608 |
| walker |  | 5407 | 46 | go package + imports in internal/adapters/cli/version.go |  |  | 0.608 |
| walker |  | 5575 | 168 | go decl at internal/config/config.go:12 |  |  | 0.643 |
| walker |  | 5584 | 9 | listing of 'internal/adapters/repositories/notes' |  |  | 0.643 |
| walker |  | 5593 | 9 | listing of 'internal/adapters/repositories/timewarrior' |  |  | 0.643 |
| ns | 5652 |  | 341 | extra.CalculateEndTime | 6.5 |  | 0.622 |
| walker |  | 5801 | 208 | go decl names surface in internal/adapters/cli/list_gui.go |  |  | 0.625 |
| walker |  | 5801 | 0 | go decl at internal/adapters/cli/list_gui.go:22 |  |  | 0.625 |
| walker |  | 5801 | 0 | go decl at internal/adapters/cli/list_gui.go:57 |  |  | 0.625 |
| walker |  | 5801 | 0 | go decl at internal/adapters/cli/list_gui.go:70 |  |  | 0.625 |
| walker |  | 5801 | 0 | go decl at internal/adapters/cli/list_gui.go:101 |  |  | 0.625 |
| walker |  | 5801 | 0 | go decl at internal/adapters/cli/list_gui.go:111 |  |  | 0.625 |
| walker |  | 5801 | 0 | go decl at internal/adapters/cli/list_gui.go:128 |  |  | 0.625 |
| walker |  | 5801 | 0 | go decl at internal/adapters/cli/list_gui.go:154 |  |  | 0.625 |
| walker |  | 5801 | 0 | go decl at internal/adapters/cli/list_gui.go:200 |  |  | 0.625 |
| walker |  | 5801 | 0 | go decl at internal/adapters/cli/list_gui.go:204 |  |  | 0.625 |
| walker |  | 5801 | 0 | go decl at internal/adapters/cli/list_gui.go:230 |  |  | 0.625 |
| walker |  | 5806 | 5 | go decl body at internal/adapters/cli/list_gui.go:200 |  |  | 0.625 |
| walker |  | 5847 | 41 | go decl body at internal/core/models/activity.go:23 |  |  | 0.630 |
| ns | 5880 |  | 228 | File-repo: type + constructor | 6.6 |  | 0.613 |
| walker |  | 5990 | 143 | go decl names surface in internal/adapters/repositories/notes/repository.go |  |  | 0.623 |
| walker |  | 5990 | 0 | go decl at internal/adapters/repositories/notes/repository.go:28 |  |  | 0.623 |
| walker |  | 5990 | 0 | go decl at internal/adapters/repositories/notes/repository.go:36 |  |  | 0.623 |
| walker |  | 5990 | 0 | go decl at internal/adapters/repositories/notes/repository.go:73 |  |  | 0.623 |
| walker |  | 6001 | 11 | go decl at internal/adapters/repositories/notes/repository.go:24 |  |  | 0.626 |
| walker |  | 6010 | 9 | go decl at internal/adapters/repositories/notes/repository.go:18 |  |  | 0.627 |
| walker |  | 6022 | 12 | go decl body at internal/adapters/repositories/notes/repository.go:28 |  |  | 0.629 |
| walker |  | 6039 | 17 | go decl at internal/adapters/repositories/notes/repository.go:32 |  |  | 0.633 |
| walker |  | 6096 | 57 | go package + imports in internal/services/ics/generator.go |  |  | 0.633 |
| walker |  | 6159 | 63 | go package + imports in internal/adapters/cli/calendar_sidebar.go |  |  | 0.633 |
| walker |  | 6224 | 65 | go package + imports in internal/adapters/cli/theme.go |  |  | 0.633 |
| ns | 6277 |  | 397 | File-repo Find body (filter logic) | 6.7 | 6.6 | 0.609 |
| walker |  | 6392 | 168 | go decl names surface in internal/adapters/repositories/file/repository.go |  |  | 0.610 |
| walker |  | 6392 | 0 | go decl at internal/adapters/repositories/file/repository.go:23 |  |  | 0.610 |
| walker |  | 6392 | 0 | go decl at internal/adapters/repositories/file/repository.go:27 |  |  | 0.610 |
| walker |  | 6392 | 0 | go decl at internal/adapters/repositories/file/repository.go:93 |  |  | 0.610 |
| walker |  | 6392 | 0 | go decl at internal/adapters/repositories/file/repository.go:130 |  |  | 0.610 |
| walker |  | 6392 | 0 | go decl at internal/adapters/repositories/file/repository.go:168 |  |  | 0.610 |
| walker |  | 6392 | 0 | go decl at internal/adapters/repositories/file/repository.go:229 |  |  | 0.610 |
| walker |  | 6392 | 0 | go decl at internal/adapters/repositories/file/repository.go:250 |  |  | 0.610 |
| walker |  | 6403 | 11 | go decl at internal/adapters/repositories/file/repository.go:19 |  |  | 0.610 |
| walker |  | 6415 | 12 | go decl body at internal/adapters/repositories/file/repository.go:23 |  |  | 0.611 |
| walker |  | 6459 | 44 | go package + imports in internal/core/ports/mocks/mock_notesrepository.go |  |  | 0.611 |
| walker |  | 6526 | 67 | go package + imports in internal/core/ports/ports.go |  |  | 0.614 |
| walker |  | 6679 | 153 | go decl at internal/adapters/cli/analyze.go:70 |  |  | 0.614 |
| ns | 6789 |  | 512 | cli/root.go — context keys + NewRootCmd shell | 7.1 |  | 0.590 |
| walker |  | 6943 | 264 | go decl names surface in internal/adapters/cli/root.go |  |  | 0.592 |
| walker |  | 6943 | 0 | go decl at internal/adapters/cli/root.go:31 |  |  | 0.592 |
| walker |  | 6943 | 0 | go decl at internal/adapters/cli/root.go:107 |  |  | 0.592 |
| walker |  | 6943 | 0 | go decl at internal/adapters/cli/root.go:115 |  |  | 0.592 |
| walker |  | 6943 | 0 | go decl at internal/adapters/cli/root.go:119 |  |  | 0.592 |
| walker |  | 6943 | 0 | go decl at internal/adapters/cli/root.go:123 |  |  | 0.592 |
| walker |  | 6943 | 0 | go decl at internal/adapters/cli/root.go:127 |  |  | 0.592 |
| walker |  | 6943 | 0 | go decl at internal/adapters/cli/root.go:134 |  |  | 0.592 |
| walker |  | 6943 | 0 | go decl at internal/adapters/cli/root.go:173 |  |  | 0.592 |
| walker |  | 6943 | 0 | go decl at internal/adapters/cli/root.go:195 |  |  | 0.592 |
| walker |  | 6952 | 9 | go decl at internal/adapters/cli/root.go:21 |  |  | 0.592 |
| ns | 6956 |  | 167 | cli/root.go — AddCommand block | 7.2 | 7.1 | 0.584 |
| walker |  | 7006 | 54 | go decl body at internal/adapters/cli/root.go:107 |  |  | 0.584 |
| walker |  | 7165 | 159 | go decl at internal/core/ports/ports.go:11 |  |  | 0.609 |
| walker |  | 7190 | 25 | go decl body at internal/adapters/cli/root.go:119 |  |  | 0.609 |
| walker |  | 7255 | 65 | go decl at internal/adapters/cli/add.go:16 |  |  | 0.609 |
| walker |  | 7330 | 75 | go package + imports in internal/adapters/cli/stop.go |  |  | 0.609 |
| walker |  | 7356 | 26 | go decl body at internal/adapters/cli/root.go:115 |  |  | 0.609 |
| ns | 7464 |  | 508 | cli/root.go — PersistentPreRunE body | 7.3 | 7.1 | 0.585 |
| ns | 7530 |  | 66 | cli/root.go — initRepository (file/timewarrior switch) | 7.4 | 7.3 | 0.583 |
| walker |  | 7646 | 290 | go decl names surface in internal/services/activity/service.go |  |  | 0.594 |
| walker |  | 7646 | 0 | go decl at internal/services/activity/service.go:20 |  |  | 0.594 |
| walker |  | 7646 | 0 | go decl at internal/services/activity/service.go:24 |  |  | 0.594 |
| walker |  | 7646 | 0 | go decl at internal/services/activity/service.go:68 |  |  | 0.594 |
| walker |  | 7646 | 0 | go decl at internal/services/activity/service.go:118 |  |  | 0.594 |
| walker |  | 7646 | 0 | go decl at internal/services/activity/service.go:141 |  |  | 0.594 |
| walker |  | 7646 | 0 | go decl at internal/services/activity/service.go:149 |  |  | 0.594 |
| walker |  | 7646 | 0 | go decl at internal/services/activity/service.go:186 |  |  | 0.594 |
| walker |  | 7646 | 0 | go decl at internal/services/activity/service.go:210 |  |  | 0.594 |
| walker |  | 7646 | 0 | go decl at internal/services/activity/service.go:214 |  |  | 0.594 |
| walker |  | 7646 | 0 | go decl at internal/services/activity/service.go:218 |  |  | 0.594 |
| walker |  | 7662 | 16 | go decl body at internal/services/activity/service.go:20 |  |  | 0.596 |
| walker |  | 7671 | 9 | go decl body at internal/services/activity/service.go:210 |  |  | 0.596 |
| walker |  | 7697 | 26 | go decl at internal/services/activity/service.go:15 |  |  | 0.600 |
| walker |  | 7707 | 10 | go decl body at internal/services/activity/service.go:214 |  |  | 0.600 |
| walker |  | 8001 | 294 | go decl names surface in internal/adapters/cli/calendar.go |  |  | 0.603 |
| walker |  | 8001 | 0 | go decl at internal/adapters/cli/calendar.go:25 |  |  | 0.603 |
| walker |  | 8001 | 0 | go decl at internal/adapters/cli/calendar.go:64 |  |  | 0.603 |
| walker |  | 8001 | 0 | go decl at internal/adapters/cli/calendar.go:79 |  |  | 0.603 |
| walker |  | 8001 | 0 | go decl at internal/adapters/cli/calendar.go:83 |  |  | 0.603 |
| walker |  | 8001 | 0 | go decl at internal/adapters/cli/calendar.go:134 |  |  | 0.603 |
| walker |  | 8001 | 0 | go decl at internal/adapters/cli/calendar.go:156 |  |  | 0.603 |
| walker |  | 8001 | 0 | go decl at internal/adapters/cli/calendar.go:234 |  |  | 0.603 |
| walker |  | 8001 | 0 | go decl at internal/adapters/cli/calendar.go:256 |  |  | 0.603 |
| walker |  | 8001 | 0 | go decl at internal/adapters/cli/calendar.go:402 |  |  | 0.603 |
| walker |  | 8001 | 0 | go decl at internal/adapters/cli/calendar.go:424 |  |  | 0.603 |
| walker |  | 8001 | 0 | go decl at internal/adapters/cli/calendar.go:488 |  |  | 0.603 |
| walker |  | 8001 | 0 | go decl at internal/adapters/cli/calendar.go:520 |  |  | 0.603 |
| ns | 8014 |  | 484 | Service Start body | 7.5 | 6.4 | 0.582 |
| walker |  | 8016 | 15 | go decl at internal/adapters/cli/calendar.go:418 |  |  | 0.582 |
| walker |  | 8024 | 8 | go decl body at internal/adapters/cli/calendar.go:79 |  |  | 0.582 |
| walker |  | 8049 | 25 | go decl doc at internal/adapters/cli/calendar.go:256 |  |  | 0.582 |
| walker |  | 8220 | 171 | go decl at internal/adapters/cli/theme.go:22 |  |  | 0.584 |
| walker |  | 8238 | 18 | go decl doc at internal/adapters/cli/theme.go:22 |  |  | 0.584 |
| walker |  | 8281 | 43 | go decl doc at internal/adapters/cli/calendar.go:488 |  |  | 0.584 |
| walker |  | 8309 | 28 | go decl body at internal/adapters/cli/root.go:123 |  |  | 0.584 |
| walker |  | 8383 | 74 | go decl at internal/adapters/cli/report.go:20 |  |  | 0.584 |
| ns | 8456 |  | 442 | start/stop/add flag declarations | 8.1 | 3.3 | 0.574 |
| walker |  | 8472 | 89 | go package + imports in internal/adapters/cli/continue.go |  |  | 0.574 |
| walker |  | 8567 | 95 | go package + imports in internal/adapters/cli/start.go |  |  | 0.574 |
| ns | 8859 |  | 403 | Theme/Styles fields + theme constructor names | 8.2 |  | 0.590 |
| walker |  | 8942 | 375 | go decl names surface in internal/adapters/cli/interactive.go |  |  | 0.591 |
| walker |  | 8942 | 0 | go decl at internal/adapters/cli/interactive.go:17 |  |  | 0.591 |
| walker |  | 8942 | 0 | go decl at internal/adapters/cli/interactive.go:36 |  |  | 0.591 |
| walker |  | 8942 | 0 | go decl at internal/adapters/cli/interactive.go:57 |  |  | 0.591 |
| walker |  | 8942 | 0 | go decl at internal/adapters/cli/interactive.go:80 |  |  | 0.591 |
| walker |  | 8942 | 0 | go decl at internal/adapters/cli/interactive.go:89 |  |  | 0.591 |
| walker |  | 8942 | 0 | go decl at internal/adapters/cli/interactive.go:98 |  |  | 0.591 |
| walker |  | 8942 | 0 | go decl at internal/adapters/cli/interactive.go:133 |  |  | 0.591 |
| walker |  | 8942 | 0 | go decl at internal/adapters/cli/interactive.go:155 |  |  | 0.591 |
| walker |  | 8942 | 0 | go decl at internal/adapters/cli/interactive.go:176 |  |  | 0.591 |
| walker |  | 8942 | 0 | go decl at internal/adapters/cli/interactive.go:214 |  |  | 0.591 |
| walker |  | 8942 | 0 | go decl at internal/adapters/cli/interactive.go:251 |  |  | 0.591 |
| walker |  | 8942 | 0 | go decl at internal/adapters/cli/interactive.go:263 |  |  | 0.591 |
| walker |  | 8942 | 0 | go decl at internal/adapters/cli/interactive.go:301 |  |  | 0.591 |
| walker |  | 8988 | 46 | go decl at internal/adapters/cli/interactive.go:123 |  |  | 0.591 |
| walker |  | 9057 | 69 | go decl at internal/adapters/cli/interactive.go:201 |  |  | 0.591 |
| walker |  | 9157 | 100 | go package + imports in internal/adapters/cli/last.go |  |  | 0.591 |
| ns | 9185 |  | 326 | report flag declarations + reportOptions | 9.1 | 3.3 | 0.584 |
| ns | 9203 |  | 18 | Mocks directory listing | 9.2 |  | 0.585 |
| walker |  | 9225 | 68 | go package + imports in internal/adapters/repositories/file/parser.go |  |  | 0.585 |
| walker |  | 9257 | 32 | go decl body at internal/adapters/cli/watch.go:115 |  |  | 0.585 |
| ns | 9346 |  | 143 | ICS generator — public fn signatures | 9.3 |  | 0.586 |
| walker |  | 9368 | 111 | go decl body at internal/timeutil/timeutil.go:158 |  |  | 0.586 |
| walker |  | 9477 | 109 | go package + imports in internal/adapters/cli/update.go |  |  | 0.586 |
| ns | 9507 |  | 161 | Interactive helpers — three public fns | 9.4 |  | 0.589 |
| walker |  | 9549 | 72 | go package + imports in internal/core/ports/mocks/mock_activityrepository.go |  |  | 0.589 |
| ns | 9600 |  | 93 | calendar.go: handleKeyMsg case lines | 9.5 |  | 0.586 |
| walker |  | 9621 | 72 | go package + imports in internal/core/ports/mocks/mock_activityresolver.go |  |  | 0.586 |
| ns | 9671 |  | 71 | calendar_sidebar.go: section renderer signatures | 9.6 |  | 0.588 |
| walker |  | 9737 | 116 | go package + imports in internal/services/activity/service.go |  |  | 0.588 |
| walker |  | 9829 | 92 | go decl at internal/adapters/cli/watch.go:77 |  |  | 0.588 |
| walker |  | 9904 | 75 | go decl body at internal/core/models/activity.go:41 |  |  | 0.588 |
| walker |  | 9918 | 14 | listing of '.github' |  |  | 0.588 |
| walker |  | 9926 | 8 | listing of '.github/workflows' |  |  | 0.588 |
| ns | 9955 |  | 284 | Remaining commands' flag declarations | 9.7 | 3.3 | 0.583 |

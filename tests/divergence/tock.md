Score(3000)=0.587 I=0.812 C=0.424 ns_rows≤3K=22/71 (reached=12 partial=2 missing=8)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 58 | 58 | listing of '.' |  |  | 1.000 |
| ns | 58 |  | 58 | Root fixture listing | 1.1 |  | 1.000 |
| walker |  | 61 | 3 | listing of 'docs' |  |  | 1.000 |
| walker |  | 65 | 4 | listing of 'cmd' |  |  | 1.000 |
| walker |  | 68 | 3 | listing of 'cmd/tock' |  |  | 1.000 |
| walker |  | 87 | 19 | listing of 'assets' |  |  | 1.000 |
| walker |  | 111 | 24 | listing of 'internal' |  |  | 0.922 |
| ns | 111 |  | 53 | README one-line pitch | 1.2 |  | 0.922 |
| walker |  | 118 | 7 | listing of 'internal/adapters' |  |  | 0.922 |
| walker |  | 126 | 8 | listing of 'internal/config' |  |  | 0.922 |
| walker |  | 134 | 8 | listing of 'internal/extra' |  |  | 0.922 |
| walker |  | 142 | 8 | listing of 'internal/services' |  |  | 0.922 |
| ns | 144 |  | 33 | go.mod identity (module + Go version) | 1.3 |  | 0.835 |
| walker |  | 145 | 3 | listing of 'internal/services/ics' |  |  | 0.836 |
| walker |  | 155 | 10 | listing of 'internal/timeutil' |  |  | 0.840 |
| walker |  | 188 | 33 | go module identity in go.mod |  |  | 0.911 |
| walker |  | 192 | 4 | listing of 'demo' |  |  | 0.911 |
| walker |  | 207 | 15 | listing of 'internal/core' |  |  | 0.911 |
| walker |  | 210 | 3 | listing of 'internal/core/errors' |  |  | 0.911 |
| walker |  | 213 | 3 | listing of 'internal/core/models' |  |  | 0.911 |
| walker |  | 218 | 5 | listing of 'internal/core/dto' |  |  | 0.911 |
| walker |  | 225 | 7 | listing of 'internal/core/ports' |  |  | 0.911 |
| walker |  | 233 | 8 | listing of 'internal/services/activity' |  |  | 0.911 |
| walker |  | 246 | 13 | listing of 'internal/adapters/repositories' |  |  | 0.911 |
| walker |  | 254 | 8 | listing of 'internal/adapters/repositories/notes' |  |  | 0.911 |
| walker |  | 262 | 8 | listing of 'internal/adapters/repositories/timewarrior' |  |  | 0.911 |
| ns | 273 |  | 129 | Makefile target names + commands (lint + test) | 1.4 |  | 0.844 |
| walker |  | 353 | 91 | README headline in README.md |  |  | 0.844 |
| walker |  | 367 | 14 | listing of '.github' |  |  | 0.844 |
| walker |  | 374 | 7 | listing of '.github/workflows' |  |  | 0.844 |
| ns | 438 |  | 165 | README Project Structure | 1.5 |  | 0.716 |
| ns | 462 |  | 24 | internal/ directory listing | 2.1 |  | 0.747 |
| ns | 473 |  | 11 | cmd/tock/ + internal/adapters/ listings | 2.2 |  | 0.758 |
| ns | 551 |  | 78 | internal/adapters/cli/ listing | 2.3 |  | 0.616 |
| ns | 600 |  | 49 | internal/adapters/repositories/ + backend subdirs | 2.4 |  | 0.603 |
| ns | 655 |  | 55 | internal/core/ + subpackage listings | 2.5 |  | 0.607 |
| ns | 676 |  | 21 | internal/services/ + subpackage listings | 2.6 |  | 0.623 |
| ns | 702 |  | 26 | internal/config/, timeutil/, extra/ listings | 2.7 |  | 0.639 |
| walker |  | 718 | 344 | YAML config at .github/workflows/ci.yml |  |  | 0.639 |
| ns | 724 |  | 22 | .github/ + workflows/ listings | 2.8 |  | 0.651 |
| walker |  | 730 | 12 | go decl names surface in cmd/tock/main.go |  |  | 0.651 |
| walker |  | 730 | 0 | go decl at cmd/tock/main.go:7 |  |  | 0.651 |
| walker |  | 747 | 17 | listing of 'internal/adapters/repositories/file' |  |  | 0.703 |
| ns | 747 |  | 23 | assets/ + demo/ listings | 2.9 |  | 0.703 |
| walker |  | 764 | 17 | listing of 'internal/core/ports/mocks' |  |  | 0.735 |
| walker |  | 842 | 78 | listing of 'internal/adapters/cli' |  |  | 0.856 |
| walker |  | 850 | 8 | go decl body at cmd/tock/main.go:7 |  |  | 0.857 |
| walker |  | 867 | 17 | go decl names surface in internal/adapters/cli/start.go |  |  | 0.857 |
| walker |  | 867 | 0 | go decl at internal/adapters/cli/start.go:17 |  |  | 0.857 |
| walker |  | 884 | 17 | go decl names surface in internal/adapters/cli/stop.go |  |  | 0.857 |
| walker |  | 884 | 0 | go decl at internal/adapters/cli/stop.go:14 |  |  | 0.857 |
| walker |  | 909 | 25 | README.md section #22 |  |  | 0.857 |
| walker |  | 947 | 38 | go decl names surface in internal/extra/extra.go |  |  | 0.857 |
| walker |  | 947 | 0 | go decl at internal/extra/extra.go:11 |  |  | 0.857 |
| ns | 1025 |  | 278 | docs/commands.md table of contents | 2.10 |  | 0.772 |
| ns | 1093 |  | 68 | cmd/tock/main.go | 3.1 |  | 0.745 |
| ns | 1265 |  | 172 | models.Activity (the one domain entity) | 3.2 |  | 0.694 |
| walker |  | 1280 | 333 | plaintext config Makefile |  |  | 0.720 |
| walker |  | 1312 | 32 | go decl names surface in internal/adapters/cli/continue.go |  |  | 0.720 |
| walker |  | 1312 | 0 | go decl at internal/adapters/cli/continue.go:20 |  |  | 0.720 |
| walker |  | 1347 | 35 | go decl names surface in internal/core/ports/ports.go |  |  | 0.720 |
| walker |  | 1358 | 11 | go decl at internal/adapters/cli/continue.go:15 |  |  | 0.720 |
| walker |  | 1404 | 46 | go decl names surface in internal/adapters/cli/update.go |  |  | 0.720 |
| walker |  | 1404 | 0 | go decl at internal/adapters/cli/update.go:22 |  |  | 0.720 |
| walker |  | 1404 | 0 | go decl at internal/adapters/cli/update.go:45 |  |  | 0.720 |
| walker |  | 1592 | 188 | headings outline in docs/commands.md |  |  | 0.720 |
| walker |  | 1597 | 5 | docs/commands.md section #5 |  |  | 0.720 |
| walker |  | 1602 | 5 | docs/commands.md section #10 |  |  | 0.720 |
| walker |  | 1607 | 5 | docs/commands.md section #15 |  |  | 0.720 |
| walker |  | 1612 | 5 | docs/commands.md section #20 |  |  | 0.720 |
| walker |  | 1617 | 5 | docs/commands.md section #30 |  |  | 0.720 |
| walker |  | 1622 | 5 | docs/commands.md section #35 |  |  | 0.720 |
| walker |  | 1627 | 5 | docs/commands.md section #40 |  |  | 0.720 |
| walker |  | 1676 | 49 | go decl names surface in internal/adapters/cli/version.go |  |  | 0.721 |
| walker |  | 1676 | 0 | go decl at internal/adapters/cli/version.go:16 |  |  | 0.721 |
| ns | 1677 |  | 412 | core/ports (the three storage/business interfaces) | 3.3 |  | 0.648 |
| walker |  | 1685 | 9 | go decl at internal/adapters/cli/version.go:10 |  |  | 0.648 |
| walker |  | 1735 | 50 | go decl names surface in internal/adapters/cli/last.go |  |  | 0.648 |
| walker |  | 1735 | 0 | go decl at internal/adapters/cli/last.go:21 |  |  | 0.648 |
| walker |  | 1735 | 0 | go decl at internal/adapters/cli/last.go:42 |  |  | 0.648 |
| walker |  | 1785 | 50 | go decl names surface in internal/adapters/cli/report.go |  |  | 0.648 |
| walker |  | 1785 | 0 | go decl at internal/adapters/cli/report.go:31 |  |  | 0.648 |
| walker |  | 1785 | 0 | go decl at internal/adapters/cli/report.go:61 |  |  | 0.648 |
| walker |  | 1801 | 16 | go decl doc at internal/adapters/cli/continue.go:20 |  |  | 0.648 |
| ns | 1990 |  | 313 | core/dto request/filter shapes | 3.4 |  | 0.581 |
| walker |  | 2063 | 262 | README.md section #1 |  |  | 0.599 |
| walker |  | 2071 | 8 | docs/commands.md section #16 |  |  | 0.599 |
| ns | 2091 |  | 101 | core/errors sentinels | 3.5 |  | 0.585 |
| walker |  | 2137 | 66 | go decl names surface in internal/adapters/cli/current.go |  |  | 0.585 |
| walker |  | 2137 | 0 | go decl at internal/adapters/cli/current.go:23 |  |  | 0.585 |
| walker |  | 2137 | 0 | go decl at internal/adapters/cli/current.go:27 |  |  | 0.585 |
| walker |  | 2137 | 0 | go decl at internal/adapters/cli/current.go:35 |  |  | 0.585 |
| walker |  | 2148 | 11 | go decl at internal/adapters/cli/current.go:19 |  |  | 0.585 |
| walker |  | 2216 | 68 | go decl names surface in internal/services/ics/generator.go |  |  | 0.585 |
| walker |  | 2216 | 0 | go decl at internal/services/ics/generator.go:12 |  |  | 0.585 |
| walker |  | 2216 | 0 | go decl at internal/services/ics/generator.go:18 |  |  | 0.585 |
| walker |  | 2216 | 0 | go decl at internal/services/ics/generator.go:29 |  |  | 0.585 |
| walker |  | 2216 | 0 | go decl at internal/services/ics/generator.go:71 |  |  | 0.585 |
| walker |  | 2233 | 17 | go decl doc at internal/services/ics/generator.go:12 |  |  | 0.585 |
| ns | 2238 |  | 147 | tock.yaml.example (concrete config sample) | 4.1 |  | 0.567 |
| walker |  | 2250 | 17 | go decl doc at internal/services/ics/generator.go:29 |  |  | 0.567 |
| walker |  | 2270 | 20 | go decl doc at internal/services/ics/generator.go:18 |  |  | 0.567 |
| walker |  | 2291 | 21 | go decl at internal/adapters/cli/last.go:16 |  |  | 0.567 |
| walker |  | 2360 | 69 | go decl names surface in internal/core/errors/errors.go |  |  | 0.581 |
| walker |  | 2369 | 9 | go decl at internal/core/errors/errors.go:5 |  |  | 0.584 |
| walker |  | 2391 | 22 | go decl doc at internal/adapters/cli/start.go:17 |  |  | 0.584 |
| walker |  | 2461 | 70 | go decl names surface in internal/core/dto/activity_dto.go |  |  | 0.587 |
| walker |  | 2492 | 31 | go decl at internal/core/dto/activity_dto.go:17 |  |  | 0.592 |
| walker |  | 2525 | 33 | go decl at internal/core/dto/activity_dto.go:46 |  |  | 0.592 |
| walker |  | 2563 | 38 | go decl at internal/core/dto/activity_dto.go:40 |  |  | 0.592 |
| ns | 2605 |  | 367 | config.Config struct + sub-structs | 4.2 |  | 0.554 |
| walker |  | 2611 | 48 | go decl at internal/core/dto/activity_dto.go:9 |  |  | 0.566 |
| walker |  | 2683 | 72 | go decl names surface in internal/adapters/cli/analyze.go |  |  | 0.566 |
| walker |  | 2683 | 0 | go decl at internal/adapters/cli/analyze.go:20 |  |  | 0.566 |
| walker |  | 2683 | 0 | go decl at internal/adapters/cli/analyze.go:84 |  |  | 0.566 |
| walker |  | 2683 | 0 | go decl at internal/adapters/cli/analyze.go:206 |  |  | 0.566 |
| walker |  | 2774 | 91 | README.md section #17 |  |  | 0.566 |
| walker |  | 2829 | 55 | go decl at internal/core/dto/activity_dto.go:32 |  |  | 0.584 |
| walker |  | 2884 | 55 | go decl at internal/core/ports/ports.go:29 |  |  | 0.587 |
| ns | 3049 |  | 444 | config.go: option funcs + defaults + every TOCK_* BindEnv call | 4.3 |  | 0.558 |
| ns | 3098 |  | 49 | config_test.go test names | 4.4 |  | 0.554 |
| ns | 3493 |  | 395 | activity.service: Start | 5.1 |  | 0.521 |
| ns | 3715 |  | 222 | activity.service: Stop | 5.2 |  | 0.504 |
| ns | 3771 |  | 56 | activity.service: Add/List/GetReport/GetRecent/GetLast/Remove signatures + enrichActivities | 5.3 |  | 0.499 |
| walker |  | 3919 | 1035 | go module file go.mod |  |  | 0.503 |
| walker |  | 3928 | 9 | docs/commands.md section #1 |  |  | 0.503 |
| ns | 3930 |  | 159 | services/ics: iCal generation | 5.4 |  | 0.494 |
| walker |  | 3937 | 9 | docs/commands.md section #6 |  |  | 0.494 |
| walker |  | 3955 | 18 | go package + imports in internal/core/errors/errors.go |  |  | 0.502 |
| walker |  | 4013 | 58 | go decl at internal/core/dto/activity_dto.go:23 |  |  | 0.525 |
| walker |  | 4092 | 79 | go decl names surface in internal/core/models/activity.go |  |  | 0.525 |
| walker |  | 4092 | 0 | go decl at internal/core/models/activity.go:19 |  |  | 0.525 |
| walker |  | 4092 | 0 | go decl at internal/core/models/activity.go:23 |  |  | 0.525 |
| walker |  | 4092 | 0 | go decl at internal/core/models/activity.go:31 |  |  | 0.525 |
| walker |  | 4092 | 0 | go decl at internal/core/models/activity.go:41 |  |  | 0.525 |
| walker |  | 4105 | 13 | go decl body at internal/core/models/activity.go:19 |  |  | 0.525 |
| walker |  | 4124 | 19 | go decl doc at internal/core/models/activity.go:31 |  |  | 0.525 |
| ns | 4341 |  | 411 | cli.NewRootCmd (composition root, wiring) | 6.1 |  | 0.499 |
| ns | 4651 |  | 310 | cli.NewStartCmd (cmd shape + time-parse logic) | 6.2 |  | 0.483 |
| walker |  | 4758 | 634 | plaintext config install.sh |  |  | 0.484 |
| walker |  | 4768 | 10 | docs/commands.md section #11 |  |  | 0.484 |
| walker |  | 4778 | 10 | docs/commands.md section #31 |  |  | 0.484 |
| ns | 4785 |  | 134 | cli.NewStopCmd | 6.3 |  | 0.476 |
| walker |  | 4868 | 90 | README.md section #21 |  |  | 0.476 |
| walker |  | 4963 | 95 | go decl names surface in internal/adapters/cli/remove.go |  |  | 0.476 |
| walker |  | 4963 | 0 | go decl at internal/adapters/cli/remove.go:22 |  |  | 0.476 |
| walker |  | 4963 | 0 | go decl at internal/adapters/cli/remove.go:95 |  |  | 0.476 |
| walker |  | 4963 | 0 | go decl at internal/adapters/cli/remove.go:116 |  |  | 0.476 |
| walker |  | 4963 | 0 | go decl at internal/adapters/cli/remove.go:127 |  |  | 0.476 |
| walker |  | 4985 | 22 | go decl body at internal/services/ics/generator.go:12 |  |  | 0.480 |
| ns | 5095 |  | 310 | cli.NewAddCmd (definition + flags only) | 6.4 |  | 0.468 |
| walker |  | 5153 | 168 | go decl names surface in internal/config/config.go |  |  | 0.468 |
| walker |  | 5153 | 0 | go decl at internal/config/config.go:62 |  |  | 0.468 |
| walker |  | 5153 | 0 | go decl at internal/config/config.go:68 |  |  | 0.468 |
| walker |  | 5153 | 0 | go decl at internal/config/config.go:74 |  |  | 0.468 |
| walker |  | 5153 | 0 | go decl at internal/config/config.go:80 |  |  | 0.468 |
| walker |  | 5168 | 15 | go decl at internal/config/config.go:41 |  |  | 0.468 |
| walker |  | 5185 | 17 | go decl at internal/config/config.go:37 |  |  | 0.468 |
| walker |  | 5202 | 17 | go decl at internal/config/config.go:45 |  |  | 0.468 |
| walker |  | 5221 | 19 | go decl at internal/config/config.go:33 |  |  | 0.468 |
| walker |  | 5311 | 90 | go decl at internal/config/config.go:25 |  |  | 0.473 |
| walker |  | 5351 | 40 | go package + imports in cmd/tock/main.go |  |  | 0.486 |
| ns | 5372 |  | 277 | cli.NewContinueCmd (definition + flags) | 6.5 |  | 0.475 |
| walker |  | 5453 | 102 | go decl names surface in internal/adapters/cli/add.go |  |  | 0.475 |
| walker |  | 5453 | 0 | go decl at internal/adapters/cli/add.go:26 |  |  | 0.475 |
| walker |  | 5453 | 0 | go decl at internal/adapters/cli/add.go:64 |  |  | 0.475 |
| walker |  | 5453 | 0 | go decl at internal/adapters/cli/add.go:124 |  |  | 0.475 |
| walker |  | 5453 | 0 | go decl at internal/adapters/cli/add.go:163 |  |  | 0.475 |
| ns | 5456 |  | 84 | cli.NewRemoveCmd (definition) | 6.6 |  | 0.471 |
| walker |  | 5526 | 73 | go decl at internal/core/ports/ports.go:22 |  |  | 0.477 |
| walker |  | 5538 | 12 | docs/commands.md section #41 |  |  | 0.477 |
| ns | 5631 |  | 175 | cli.NewCurrentCmd (definition + flags) | 6.7 |  | 0.471 |
| walker |  | 5652 | 114 | go decl at internal/config/config.go:49 |  |  | 0.471 |
| ns | 5792 |  | 161 | cli.NewLastCmd | 6.8 |  | 0.464 |
| walker |  | 5841 | 189 | go decl names surface in internal/timeutil/timeutil.go |  |  | 0.464 |
| walker |  | 5841 | 0 | go decl at internal/timeutil/timeutil.go:11 |  |  | 0.464 |
| walker |  | 5841 | 0 | go decl at internal/timeutil/timeutil.go:25 |  |  | 0.464 |
| walker |  | 5841 | 0 | go decl at internal/timeutil/timeutil.go:34 |  |  | 0.464 |
| walker |  | 5841 | 0 | go decl at internal/timeutil/timeutil.go:39 |  |  | 0.464 |
| walker |  | 5841 | 0 | go decl at internal/timeutil/timeutil.go:47 |  |  | 0.464 |
| walker |  | 5841 | 0 | go decl at internal/timeutil/timeutil.go:56 |  |  | 0.464 |
| walker |  | 5841 | 0 | go decl at internal/timeutil/timeutil.go:103 |  |  | 0.464 |
| walker |  | 5841 | 0 | go decl at internal/timeutil/timeutil.go:158 |  |  | 0.464 |
| walker |  | 5854 | 13 | go decl at internal/timeutil/timeutil.go:19 |  |  | 0.464 |
| walker |  | 5863 | 9 | go decl at internal/timeutil/timeutil.go:13 |  |  | 0.464 |
| walker |  | 5873 | 10 | go decl doc at internal/timeutil/timeutil.go:19 |  |  | 0.464 |
| walker |  | 5881 | 8 | go decl body at internal/timeutil/timeutil.go:34 |  |  | 0.464 |
| walker |  | 5895 | 14 | go decl doc at internal/timeutil/timeutil.go:11 |  |  | 0.464 |
| walker |  | 5910 | 15 | go decl doc at internal/timeutil/timeutil.go:34 |  |  | 0.464 |
| walker |  | 5926 | 16 | go decl doc at internal/timeutil/timeutil.go:47 |  |  | 0.464 |
| walker |  | 5943 | 17 | go decl doc at internal/timeutil/timeutil.go:39 |  |  | 0.464 |
| walker |  | 5979 | 36 | go decl doc at internal/timeutil/timeutil.go:56 |  |  | 0.464 |
| walker |  | 6019 | 40 | go decl doc at internal/timeutil/timeutil.go:25 |  |  | 0.464 |
| walker |  | 6063 | 44 | go decl doc at internal/timeutil/timeutil.go:103 |  |  | 0.464 |
| ns | 6102 |  | 310 | cli.NewReportCmd (definition + flags) | 6.9 |  | 0.456 |
| ns | 6168 |  | 66 | cli.runUpdateCheck (update-check throttle gate) | 6.10 |  | 0.453 |
| walker |  | 6174 | 111 | go decl names surface in internal/adapters/cli/calendar_sidebar.go |  |  | 0.453 |
| walker |  | 6174 | 0 | go decl at internal/adapters/cli/calendar_sidebar.go:15 |  |  | 0.453 |
| walker |  | 6174 | 0 | go decl at internal/adapters/cli/calendar_sidebar.go:27 |  |  | 0.453 |
| walker |  | 6174 | 0 | go decl at internal/adapters/cli/calendar_sidebar.go:51 |  |  | 0.453 |
| walker |  | 6174 | 0 | go decl at internal/adapters/cli/calendar_sidebar.go:128 |  |  | 0.453 |
| walker |  | 6174 | 0 | go decl at internal/adapters/cli/calendar_sidebar.go:211 |  |  | 0.453 |
| walker |  | 6205 | 31 | go decl at internal/adapters/cli/update.go:17 |  |  | 0.453 |
| ns | 6272 |  | 104 | cli version/commit/date ldflags vars | 6.11 |  | 0.451 |
| walker |  | 6330 | 125 | README.md section #18 |  |  | 0.451 |
| walker |  | 6347 | 17 | go decl doc at internal/adapters/cli/analyze.go:84 |  |  | 0.451 |
| ns | 6417 |  | 145 | cli watchModel + keyMap (state for `tock watch`) | 7.1 |  | 0.443 |
| walker |  | 6468 | 121 | go decl names surface in internal/adapters/cli/theme.go |  |  | 0.443 |
| walker |  | 6468 | 0 | go decl at internal/adapters/cli/theme.go:40 |  |  | 0.443 |
| walker |  | 6468 | 0 | go decl at internal/adapters/cli/theme.go:53 |  |  | 0.443 |
| walker |  | 6468 | 0 | go decl at internal/adapters/cli/theme.go:66 |  |  | 0.443 |
| walker |  | 6468 | 0 | go decl at internal/adapters/cli/theme.go:79 |  |  | 0.443 |
| walker |  | 6468 | 0 | go decl at internal/adapters/cli/theme.go:93 |  |  | 0.443 |
| walker |  | 6468 | 0 | go decl at internal/adapters/cli/theme.go:122 |  |  | 0.443 |
| walker |  | 6468 | 0 | go decl at internal/adapters/cli/theme.go:167 |  |  | 0.443 |
| walker |  | 6484 | 16 | go decl doc at internal/adapters/cli/theme.go:167 |  |  | 0.443 |
| walker |  | 6502 | 18 | go decl doc at internal/adapters/cli/theme.go:40 |  |  | 0.443 |
| ns | 6515 |  | 98 | cli.NewICalCmd (definition + flags) | 7.2 |  | 0.440 |
| walker |  | 6520 | 18 | go decl doc at internal/adapters/cli/theme.go:53 |  |  | 0.440 |
| walker |  | 6539 | 19 | go decl doc at internal/adapters/cli/theme.go:122 |  |  | 0.440 |
| walker |  | 6559 | 20 | go decl doc at internal/adapters/cli/theme.go:66 |  |  | 0.440 |
| walker |  | 6580 | 21 | go decl doc at internal/adapters/cli/theme.go:79 |  |  | 0.440 |
| walker |  | 6609 | 29 | go decl doc at internal/adapters/cli/theme.go:93 |  |  | 0.440 |
| walker |  | 6622 | 13 | docs/commands.md section #45 |  |  | 0.440 |
| walker |  | 6635 | 13 | docs/commands.md section #49 |  |  | 0.440 |
| walker |  | 6653 | 18 | go decl doc at internal/adapters/cli/report.go:61 |  |  | 0.440 |
| ns | 6688 |  | 173 | cli.NewAnalyzeCmd + AnalysisStats | 7.3 |  | 0.434 |
| walker |  | 6740 | 87 | go decl at internal/adapters/cli/theme.go:11 |  |  | 0.435 |
| walker |  | 6754 | 14 | go decl doc at internal/adapters/cli/theme.go:11 |  |  | 0.435 |
| walker |  | 6773 | 19 | go decl doc at internal/adapters/cli/calendar_sidebar.go:15 |  |  | 0.435 |
| walker |  | 6845 | 72 | go decl doc at internal/timeutil/timeutil.go:158 |  |  | 0.435 |
| walker |  | 6859 | 14 | docs/commands.md section #21 |  |  | 0.435 |
| ns | 6867 |  | 179 | cli reportModel (state for `tock calendar`) | 7.4 |  | 0.429 |
| walker |  | 6912 | 53 | go package + imports in internal/timeutil/timeutil.go |  |  | 0.429 |
| walker |  | 6932 | 20 | go decl doc at internal/adapters/cli/analyze.go:206 |  |  | 0.429 |
| walker |  | 6945 | 13 | go decl body at internal/adapters/cli/current.go:23 |  |  | 0.429 |
| walker |  | 6960 | 15 | docs/commands.md section #36 |  |  | 0.429 |
| ns | 7054 |  | 187 | cli calendar_sidebar.go: renderSidebar (fixed-height-budget layout) | 7.5 |  | 0.423 |
| walker |  | 7058 | 98 | go decl at internal/core/models/activity.go:10 |  |  | 0.434 |
| walker |  | 7070 | 12 | go decl doc at internal/core/models/activity.go:10 |  |  | 0.437 |
| walker |  | 7086 | 16 | docs/commands.md section #26 |  |  | 0.437 |
| ns | 7171 |  | 117 | cli list_gui model (state for `tock list`) | 7.6 |  | 0.432 |
| walker |  | 7256 | 170 | go decl names surface in internal/adapters/cli/ical.go |  |  | 0.432 |
| walker |  | 7256 | 0 | go decl at internal/adapters/cli/ical.go:23 |  |  | 0.432 |
| walker |  | 7256 | 0 | go decl at internal/adapters/cli/ical.go:78 |  |  | 0.432 |
| walker |  | 7256 | 0 | go decl at internal/adapters/cli/ical.go:155 |  |  | 0.432 |
| walker |  | 7256 | 0 | go decl at internal/adapters/cli/ical.go:185 |  |  | 0.432 |
| walker |  | 7256 | 0 | go decl at internal/adapters/cli/ical.go:210 |  |  | 0.432 |
| walker |  | 7256 | 0 | go decl at internal/adapters/cli/ical.go:250 |  |  | 0.432 |
| walker |  | 7256 | 0 | go decl at internal/adapters/cli/ical.go:302 |  |  | 0.432 |
| ns | 7362 |  | 191 | cli.SelectActivityMetadata (interactive picker entry point) | 7.7 |  | 0.425 |
| walker |  | 7426 | 170 | go decl names surface in internal/adapters/cli/watch.go |  |  | 0.426 |
| walker |  | 7426 | 0 | go decl at internal/adapters/cli/watch.go:19 |  |  | 0.426 |
| walker |  | 7426 | 0 | go decl at internal/adapters/cli/watch.go:95 |  |  | 0.426 |
| walker |  | 7426 | 0 | go decl at internal/adapters/cli/watch.go:115 |  |  | 0.426 |
| walker |  | 7426 | 0 | go decl at internal/adapters/cli/watch.go:121 |  |  | 0.426 |
| walker |  | 7426 | 0 | go decl at internal/adapters/cli/watch.go:175 |  |  | 0.426 |
| walker |  | 7426 | 0 | go decl at internal/adapters/cli/watch.go:262 |  |  | 0.426 |
| walker |  | 7448 | 22 | go decl at internal/adapters/cli/watch.go:90 |  |  | 0.428 |
| ns | 7475 |  | 113 | cli theme.go: Theme/Styles structs + DarkTheme | 7.8 |  | 0.438 |
| walker |  | 7515 | 67 | go package + imports in internal/extra/extra.go |  |  | 0.438 |
| ns | 7588 |  | 113 | file/parser.go: ParseActivity (the plaintext activity format) | 8.1 |  | 0.434 |
| ns | 7751 |  | 163 | file/repository.go: Find | 8.2 |  | 0.429 |
| ns | 7865 |  | 114 | file package test names | 8.3 |  | 0.426 |
| walker |  | 7915 | 400 | README.md section #19 |  |  | 0.426 |
| walker |  | 8030 | 115 | go decl names surface in internal/adapters/repositories/file/parser.go |  |  | 0.426 |
| walker |  | 8030 | 0 | go decl at internal/adapters/repositories/file/parser.go:20 |  |  | 0.426 |
| walker |  | 8030 | 0 | go decl at internal/adapters/repositories/file/parser.go:64 |  |  | 0.426 |
| walker |  | 8030 | 0 | go decl at internal/adapters/repositories/file/parser.go:72 |  |  | 0.426 |
| walker |  | 8037 | 7 | go decl at internal/adapters/repositories/file/parser.go:15 |  |  | 0.426 |
| ns | 8045 |  | 180 | notes/repository.go: Save (per-activity notes/tags sidecar) | 8.4 |  | 0.422 |
| ns | 8098 |  | 53 | notes package test names | 8.5 |  | 0.421 |
| walker |  | 8205 | 168 | go decl at internal/config/config.go:12 |  |  | 0.440 |
| ns | 8230 |  | 132 | timewarrior/repository.go: Find + filter helpers | 8.6 |  | 0.437 |
| walker |  | 8248 | 43 | go package + imports in internal/core/models/activity.go |  |  | 0.449 |
| walker |  | 8322 | 74 | go package + imports in internal/config/config.go |  |  | 0.464 |
| ns | 8342 |  | 112 | timewarrior/repository.go: parseIncLine (native undo-log format) | 8.7 |  | 0.462 |
| ns | 8448 |  | 106 | timewarrior package test names | 8.8 |  | 0.459 |
| walker |  | 8532 | 210 | go decl names surface in internal/adapters/cli/list_gui.go |  |  | 0.459 |
| walker |  | 8532 | 0 | go decl at internal/adapters/cli/list_gui.go:22 |  |  | 0.459 |
| walker |  | 8532 | 0 | go decl at internal/adapters/cli/list_gui.go:57 |  |  | 0.459 |
| walker |  | 8532 | 0 | go decl at internal/adapters/cli/list_gui.go:70 |  |  | 0.459 |
| walker |  | 8532 | 0 | go decl at internal/adapters/cli/list_gui.go:101 |  |  | 0.459 |
| walker |  | 8532 | 0 | go decl at internal/adapters/cli/list_gui.go:111 |  |  | 0.459 |
| walker |  | 8532 | 0 | go decl at internal/adapters/cli/list_gui.go:128 |  |  | 0.459 |
| walker |  | 8532 | 0 | go decl at internal/adapters/cli/list_gui.go:154 |  |  | 0.459 |
| walker |  | 8532 | 0 | go decl at internal/adapters/cli/list_gui.go:200 |  |  | 0.459 |
| walker |  | 8532 | 0 | go decl at internal/adapters/cli/list_gui.go:204 |  |  | 0.459 |
| walker |  | 8532 | 0 | go decl at internal/adapters/cli/list_gui.go:230 |  |  | 0.459 |
| walker |  | 8539 | 7 | go decl body at internal/adapters/cli/list_gui.go:200 |  |  | 0.459 |
| ns | 8574 |  | 126 | timeutil.Formatter: type + display-format getters | 9.1 |  | 0.471 |
| walker |  | 8589 | 50 | go package + imports in internal/core/dto/activity_dto.go |  |  | 0.484 |
| walker |  | 8734 | 145 | go decl names surface in internal/adapters/repositories/notes/repository.go |  |  | 0.484 |
| walker |  | 8734 | 0 | go decl at internal/adapters/repositories/notes/repository.go:28 |  |  | 0.484 |
| walker |  | 8734 | 0 | go decl at internal/adapters/repositories/notes/repository.go:36 |  |  | 0.484 |
| walker |  | 8734 | 0 | go decl at internal/adapters/repositories/notes/repository.go:73 |  |  | 0.484 |
| walker |  | 8745 | 11 | go decl at internal/adapters/repositories/notes/repository.go:24 |  |  | 0.484 |
| walker |  | 8754 | 9 | go decl at internal/adapters/repositories/notes/repository.go:18 |  |  | 0.484 |
| ns | 8756 |  | 182 | timeutil.ParseTime (24h-first-then-12h fallback chain) | 9.2 |  | 0.481 |
| walker |  | 8771 | 17 | go decl at internal/adapters/repositories/notes/repository.go:32 |  |  | 0.481 |
| walker |  | 8785 | 14 | go decl body at internal/adapters/repositories/notes/repository.go:28 |  |  | 0.481 |
| walker |  | 8839 | 54 | go package + imports in internal/adapters/cli/version.go |  |  | 0.492 |
| ns | 8859 |  | 103 | timeutil_test.go test names | 9.3 |  | 0.490 |
| ns | 8974 |  | 115 | extra.CalculateEndTime + its test name | 9.4 |  | 0.488 |
| walker |  | 9009 | 170 | go decl names surface in internal/adapters/repositories/file/repository.go |  |  | 0.488 |
| walker |  | 9009 | 0 | go decl at internal/adapters/repositories/file/repository.go:23 |  |  | 0.488 |
| walker |  | 9009 | 0 | go decl at internal/adapters/repositories/file/repository.go:27 |  |  | 0.488 |
| walker |  | 9009 | 0 | go decl at internal/adapters/repositories/file/repository.go:93 |  |  | 0.488 |
| walker |  | 9009 | 0 | go decl at internal/adapters/repositories/file/repository.go:130 |  |  | 0.488 |
| walker |  | 9009 | 0 | go decl at internal/adapters/repositories/file/repository.go:168 |  |  | 0.488 |
| walker |  | 9009 | 0 | go decl at internal/adapters/repositories/file/repository.go:229 |  |  | 0.488 |
| walker |  | 9009 | 0 | go decl at internal/adapters/repositories/file/repository.go:250 |  |  | 0.488 |
| walker |  | 9020 | 11 | go decl at internal/adapters/repositories/file/repository.go:19 |  |  | 0.488 |
| ns | 9022 |  | 48 | .github/workflows/ci.yml | 10.1 |  | 0.492 |
| walker |  | 9034 | 14 | go decl body at internal/adapters/repositories/file/repository.go:23 |  |  | 0.492 |
| ns | 9096 |  | 74 | .github/workflows/release.yml | 10.2 |  | 0.488 |
| ns | 9184 |  | 88 | .goreleaser.yaml | 10.3 |  | 0.485 |
| ns | 9281 |  | 97 | .mockery.yaml (mocked interfaces) | 10.4 |  | 0.482 |
| walker |  | 9303 | 269 | go decl names surface in internal/adapters/cli/root.go |  |  | 0.482 |
| walker |  | 9303 | 0 | go decl at internal/adapters/cli/root.go:31 |  |  | 0.482 |
| walker |  | 9303 | 0 | go decl at internal/adapters/cli/root.go:107 |  |  | 0.482 |
| walker |  | 9303 | 0 | go decl at internal/adapters/cli/root.go:115 |  |  | 0.482 |
| walker |  | 9303 | 0 | go decl at internal/adapters/cli/root.go:119 |  |  | 0.482 |
| walker |  | 9303 | 0 | go decl at internal/adapters/cli/root.go:123 |  |  | 0.482 |
| walker |  | 9303 | 0 | go decl at internal/adapters/cli/root.go:127 |  |  | 0.482 |
| walker |  | 9303 | 0 | go decl at internal/adapters/cli/root.go:134 |  |  | 0.482 |
| walker |  | 9303 | 0 | go decl at internal/adapters/cli/root.go:173 |  |  | 0.482 |
| walker |  | 9303 | 0 | go decl at internal/adapters/cli/root.go:195 |  |  | 0.482 |
| walker |  | 9312 | 9 | go decl at internal/adapters/cli/root.go:21 |  |  | 0.482 |
| ns | 9383 |  | 102 | .golangci.yaml: version/issues/formatters | 10.5 |  | 0.479 |
| walker |  | 9465 | 153 | go decl at internal/adapters/cli/analyze.go:70 |  |  | 0.491 |
| ns | 9499 |  | 116 | install.sh (OS/arch detection + fetch-and-install) | 10.6 |  | 0.497 |
| ns | 9573 |  | 74 | README Quick Start / Installation options | 11.1 |  | 0.495 |
| ns | 9682 |  | 109 | README Configuration: priority order + env var list | 11.2 |  | 0.491 |
| walker |  | 9731 | 266 | docs/commands.md section #0 |  |  | 0.517 |
| ns | 9733 |  | 51 | README Theming section | 11.3 |  | 0.515 |
| walker |  | 9796 | 65 | go package + imports in internal/services/ics/generator.go |  |  | 0.526 |
| ns | 9865 |  | 132 | README File Format spec | 11.4 |  | 0.530 |
| ns | 9942 |  | 77 | docs/commands.md: `start` section lede + usage | 11.6 |  | 0.528 |
| walker |  | 9955 | 159 | go decl at internal/core/ports/ports.go:11 |  |  | 0.542 |
| ns | 9984 |  | 42 | core/ports/mocks: the three mock type declarations | 12.1 |  | 0.541 |

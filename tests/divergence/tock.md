Score(3000)=0.552 I=0.826 C=0.368 ns_rows≤3K=25/59 grid(1000/1442/2080/3000/4327/6240/9000)=0.698/0.733/0.628/0.552/0.567/0.537/0.532

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 49 |  | 49 | Project name and one-line description | 1.1 |  | 0.000 |
| walker |  | 52 | 52 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 62 | 10 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.131 |
| walker |  | 66 | 4 | Fs::DirListing { dir: docs } |  |  | 0.131 |
| walker |  | 71 | 5 | Fs::DirListing { dir: demo } |  |  | 0.131 |
| walker |  | 77 | 6 | Fs::DirListing { dir: cmd/tock } |  |  | 0.131 |
| walker |  | 96 | 19 | Fs::DirListing { dir: internal } |  |  | 0.131 |
| ns | 101 |  | 52 | Repository root listing (complete) | 1.2 |  | 0.742 |
| walker |  | 102 | 6 | Fs::DirListing { dir: internal/adapters } |  |  | 0.750 |
| walker |  | 109 | 7 | Fs::DirListing { dir: internal/services } |  |  | 0.759 |
| walker |  | 113 | 4 | Fs::DirListing { dir: internal/services/ics } |  |  | 0.759 |
| walker |  | 133 | 20 | Fs::DirListing { dir: assets } |  |  | 0.760 |
| ns | 134 |  | 33 | Module import path and Go version | 1.3 |  | 0.693 |
| walker |  | 166 | 33 | GoMod::Identity { file: go.mod } |  |  | 0.774 |
| walker |  | 175 | 9 | Fs::DirListing { dir: internal/config } |  |  | 0.775 |
| walker |  | 184 | 9 | Fs::DirListing { dir: internal/extra } |  |  | 0.778 |
| walker |  | 195 | 11 | Fs::DirListing { dir: internal/timeutil } |  |  | 0.781 |
| walker |  | 207 | 12 | Fs::DirListing { dir: internal/core } |  |  | 0.804 |
| walker |  | 211 | 4 | Fs::DirListing { dir: internal/core/errors } |  |  | 0.806 |
| walker |  | 215 | 4 | Fs::DirListing { dir: internal/core/models } |  |  | 0.809 |
| ns | 217 |  | 83 | README feature bullets, part 1 (storage, notes, TUI, footprint) | 1.4 |  | 0.728 |
| walker |  | 221 | 6 | Fs::DirListing { dir: internal/core/dto } |  |  | 0.730 |
| walker |  | 228 | 7 | Fs::DirListing { dir: internal/core/ports } |  |  | 0.736 |
| walker |  | 237 | 9 | Fs::DirListing { dir: internal/services/activity } |  |  | 0.743 |
| walker |  | 248 | 11 | Fs::DirListing { dir: internal/adapters/repositories } |  |  | 0.743 |
| walker |  | 257 | 9 | Fs::DirListing { dir: internal/adapters/repositories/notes } |  |  | 0.744 |
| walker |  | 266 | 9 | Fs::DirListing { dir: internal/adapters/repositories/timewarrior } |  |  | 0.745 |
| walker |  | 280 | 14 | Fs::DirListing { dir: .github } |  |  | 0.746 |
| walker |  | 288 | 8 | Fs::DirListing { dir: .github/workflows } |  |  | 0.746 |
| ns | 294 |  | 77 | README feature bullets, part 2 (Bartib/TimeWarrior compat, themes, iCal) | 1.5 |  | 0.697 |
| ns | 362 |  | 68 | Binary entry point | 1.6 |  | 0.589 |
| ns | 412 |  | 50 | Package tree: cmd/, internal/ and its layer directories | 1.7 |  | 0.637 |
| ns | 493 |  | 81 | Domain, service and support package file listings (complete) | 1.8 |  | 0.621 |
| ns | 619 |  | 126 | Adapter package listings: cli/ and the three repositories | 1.9 |  | 0.511 |
| walker |  | 621 | 333 | Plaintext::Whole { file: Makefile } |  |  | 0.511 |
| walker |  | 639 | 18 | Fs::DirListing { dir: internal/adapters/repositories/file } |  |  | 0.526 |
| ns | 724 |  | 105 | README section headings (all H2) | 1.10 |  | 0.494 |
| walker |  | 744 | 105 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.554 |
| walker |  | 788 | 44 | Markdown::CommandBlock { file: README.md, row: 49 } |  |  | 0.554 |
| ns | 898 |  | 174 | ports.ActivityResolver — the full service contract | 2.1 |  | 0.527 |
| walker |  | 1002 | 214 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.712 |
| walker |  | 1020 | 18 | Fs::DirListing { dir: internal/core/ports/mocks } |  |  | 0.745 |
| ns | 1022 |  | 124 | models.Activity struct with JSON tags | 2.2 |  | 0.715 |
| walker |  | 1099 | 79 | Fs::DirListing { dir: internal/adapters/cli } |  |  | 0.853 |
| ns | 1175 |  | 153 | ports.ActivityRepository and ports.NotesRepository | 2.3 |  | 0.813 |
| walker |  | 1178 | 79 | Markdown::Section { file: README.md, section_index: 19, keeps_default_concavity: false } |  |  | 0.813 |
| ns | 1253 |  | 78 | Domain sentinel errors | 2.4 |  | 0.794 |
| walker |  | 1420 | 242 | GoMod::File { file: go.mod } |  |  | 0.795 |
| ns | 1440 |  | 187 | dto request types for Start / Stop / Add | 2.5 |  | 0.733 |
| walker |  | 1501 | 81 | Markdown::Section { file: README.md, section_index: 25, keeps_default_concavity: false } |  |  | 0.733 |
| ns | 1506 |  | 66 | dto.ActivityFilter — the query type | 2.6 |  | 0.715 |
| ns | 1601 |  | 95 | dto.Report and dto.ProjectReport | 2.7 |  | 0.691 |
| walker |  | 1616 | 115 | Markdown::Section { file: README.md, section_index: 20, keeps_default_concavity: false } |  |  | 0.691 |
| ns | 1685 |  | 84 | models.Activity method roster (bodies elided) | 2.8 |  | 0.672 |
| walker |  | 1728 | 112 | Plaintext::DeclSurface { file: install.sh } |  |  | 0.672 |
| ns | 1832 |  | 147 | models.Activity method bodies | 2.9 | 2.8 | 0.640 |
| walker |  | 1967 | 239 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.640 |
| ns | 2000 |  | 168 | Root command declaration and the three persistent flags | 3.1 |  | 0.628 |
| ns | 2162 |  | 162 | Complete subcommand registration list | 3.2 |  | 0.605 |
| walker |  | 2198 | 231 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.605 |
| walker |  | 2210 | 12 | Code::CodeKey { rung: Names, file: cmd/tock/main.go, decl: 0, sub: 0, line: 0 } |  |  | 0.605 |
| walker |  | 2221 | 11 | Code::CodeKey { rung: Decl, file: cmd/tock/main.go, decl: 1, sub: 0, line: 7 } |  |  | 0.609 |
| ns | 2398 |  | 236 | Command declarations, part 1: add, analyze, calendar, continue, current, ical | 3.3 |  | 0.588 |
| walker |  | 2444 | 223 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.588 |
| walker |  | 2612 | 168 | Code::CodeKey { rung: Names, file: internal/config/config.go, decl: 0, sub: 0, line: 0 } |  |  | 0.589 |
| walker |  | 2627 | 15 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 5, sub: 0, line: 41 } |  |  | 0.589 |
| walker |  | 2644 | 17 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 4, sub: 0, line: 37 } |  |  | 0.589 |
| walker |  | 2661 | 17 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 6, sub: 0, line: 45 } |  |  | 0.589 |
| walker |  | 2680 | 19 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 3, sub: 0, line: 33 } |  |  | 0.589 |
| ns | 2733 |  | 335 | Command declarations, part 2: last, list, remove, report, start, stop, version, watch | 3.4 |  | 0.562 |
| ns | 2764 |  | 31 | docs/commands.md structure | 3.5 |  | 0.558 |
| walker |  | 2770 | 90 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 2, sub: 0, line: 25 } |  |  | 0.559 |
| walker |  | 2884 | 114 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 7, sub: 0, line: 49 } |  |  | 0.560 |
| ns | 2956 |  | 192 | Flags of `start` and `stop` | 3.6 |  | 0.551 |
| walker |  | 3052 | 168 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 1, sub: 0, line: 12 } |  |  | 0.552 |
| ns | 3280 |  | 324 | Flags of `add` and `continue` | 3.7 |  | 0.539 |
| walker |  | 3317 | 265 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.539 |
| walker |  | 3515 | 198 | Code::CodeKey { rung: Names, file: internal/timeutil/timeutil.go, decl: 0, sub: 0, line: 0 } |  |  | 0.540 |
| walker |  | 3520 | 5 | Code::CodeKey { rung: Decl, file: internal/timeutil/timeutil.go, decl: 2, sub: 0, line: 13 } |  |  | 0.540 |
| walker |  | 3533 | 13 | Code::CodeKey { rung: Decl, file: internal/timeutil/timeutil.go, decl: 3, sub: 0, line: 19 } |  |  | 0.540 |
| walker |  | 3543 | 10 | Code::CodeKey { rung: Doc, file: internal/timeutil/timeutil.go, decl: 3, sub: 0, line: 19 } |  |  | 0.540 |
| walker |  | 3557 | 14 | Code::CodeKey { rung: Doc, file: internal/timeutil/timeutil.go, decl: 1, sub: 0, line: 11 } |  |  | 0.540 |
| walker |  | 3565 | 8 | Code::CodeKey { rung: Body, file: internal/timeutil/timeutil.go, decl: 5, sub: 0, line: 34 } |  |  | 0.540 |
| walker |  | 3580 | 15 | Code::CodeKey { rung: Doc, file: internal/timeutil/timeutil.go, decl: 5, sub: 0, line: 34 } |  |  | 0.540 |
| ns | 3583 |  | 303 | Flags of `report`, `last` and `analyze` | 3.8 |  | 0.529 |
| walker |  | 3596 | 16 | Code::CodeKey { rung: Doc, file: internal/timeutil/timeutil.go, decl: 7, sub: 0, line: 47 } |  |  | 0.529 |
| walker |  | 3613 | 17 | Code::CodeKey { rung: Doc, file: internal/timeutil/timeutil.go, decl: 6, sub: 0, line: 39 } |  |  | 0.529 |
| ns | 3775 |  | 192 | Flags of `current`, `watch`, `ical` and `remove` | 3.9 |  | 0.522 |
| walker |  | 3844 | 231 | Markdown::Section { file: README.md, section_index: 22, keeps_default_concavity: false } |  |  | 0.522 |
| walker |  | 3879 | 35 | Code::CodeKey { rung: Names, file: internal/core/ports/ports.go, decl: 0, sub: 0, line: 0 } |  |  | 0.523 |
| walker |  | 3934 | 55 | Code::CodeKey { rung: Decl, file: internal/core/ports/ports.go, decl: 3, sub: 0, line: 29 } |  |  | 0.529 |
| walker |  | 4007 | 73 | Code::CodeKey { rung: Decl, file: internal/core/ports/ports.go, decl: 2, sub: 0, line: 22 } |  |  | 0.550 |
| walker |  | 4166 | 159 | Code::CodeKey { rung: Decl, file: internal/core/ports/ports.go, decl: 1, sub: 0, line: 11 } |  |  | 0.578 |
| ns | 4170 |  | 395 | PersistentPreRunE — dependency injection and backend selection | 3.10 |  | 0.544 |
| walker |  | 4236 | 70 | Code::CodeKey { rung: Names, file: internal/core/dto/activity_dto.go, decl: 0, sub: 0, line: 0 } |  |  | 0.548 |
| walker |  | 4267 | 31 | Code::CodeKey { rung: Decl, file: internal/core/dto/activity_dto.go, decl: 2, sub: 0, line: 17 } |  |  | 0.553 |
| walker |  | 4300 | 33 | Code::CodeKey { rung: Decl, file: internal/core/dto/activity_dto.go, decl: 6, sub: 0, line: 46 } |  |  | 0.560 |
| walker |  | 4338 | 38 | Code::CodeKey { rung: Decl, file: internal/core/dto/activity_dto.go, decl: 5, sub: 0, line: 40 } |  |  | 0.576 |
| walker |  | 4386 | 48 | Code::CodeKey { rung: Decl, file: internal/core/dto/activity_dto.go, decl: 1, sub: 0, line: 9 } |  |  | 0.590 |
| ns | 4404 |  | 234 | Execute, the context keys, and the accessor helpers | 3.11 |  | 0.569 |
| walker |  | 4441 | 55 | Code::CodeKey { rung: Decl, file: internal/core/dto/activity_dto.go, decl: 4, sub: 0, line: 32 } |  |  | 0.584 |
| walker |  | 4499 | 58 | Code::CodeKey { rung: Decl, file: internal/core/dto/activity_dto.go, decl: 3, sub: 0, line: 23 } |  |  | 0.609 |
| ns | 4548 |  | 144 | `current --format` template variables (Long help) | 3.12 |  | 0.600 |
| ns | 4713 |  | 165 | `ical` argument semantics and `remove` Long help | 3.13 |  | 0.593 |
| ns | 4835 |  | 122 | Shell-completion machinery | 3.14 |  | 0.587 |
| walker |  | 4907 | 408 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.587 |
| ns | 4926 |  | 91 | activity.service struct and constructor | 4.1 |  | 0.580 |
| walker |  | 5163 | 256 | Markdown::Section { file: README.md, section_index: 23, keeps_default_concavity: false } |  |  | 0.580 |
| ns | 5193 |  | 267 | service method roster (bodies elided) | 4.2 |  | 0.565 |
| walker |  | 5324 | 161 | Markdown::Section { file: README.md, section_index: 24, keeps_default_concavity: false } |  |  | 0.565 |
| walker |  | 5351 | 27 | Code::CodeKey { rung: Names, file: internal/adapters/cli/root.go, decl: 0, sub: 0, line: 0 } |  |  | 0.565 |
| walker |  | 5426 | 75 | Code::CodeKey { rung: Names, file: internal/core/errors/errors.go, decl: 0, sub: 0, line: 0 } |  |  | 0.575 |
| ns | 5427 |  | 234 | Service.Start — implicit stop of running activities | 4.3 | 4.2 | 0.559 |
| walker |  | 5429 | 3 | Code::CodeKey { rung: Decl, file: internal/core/errors/errors.go, decl: 1, sub: 0, line: 5 } |  |  | 0.561 |
| walker |  | 5508 | 79 | Code::CodeKey { rung: Names, file: internal/core/models/activity.go, decl: 0, sub: 0, line: 0 } |  |  | 0.567 |
| walker |  | 5606 | 98 | Code::CodeKey { rung: Decl, file: internal/core/models/activity.go, decl: 1, sub: 0, line: 10 } |  |  | 0.581 |
| walker |  | 5618 | 12 | Code::CodeKey { rung: Doc, file: internal/core/models/activity.go, decl: 1, sub: 0, line: 10 } |  |  | 0.585 |
| ns | 5646 |  | 219 | Service.Stop — target selection and validation | 4.4 | 4.2 | 0.568 |
| walker |  | 5758 | 140 | Plaintext::DeclSurface { file: demo/demo_script.sh } |  |  | 0.568 |
| walker |  | 5775 | 17 | Code::CodeKey { rung: Names, file: internal/adapters/cli/current.go, decl: 0, sub: 0, line: 0 } |  |  | 0.568 |
| walker |  | 5792 | 17 | Code::CodeKey { rung: Names, file: internal/adapters/cli/calendar.go, decl: 0, sub: 0, line: 0 } |  |  | 0.568 |
| ns | 5885 |  | 239 | GetRecent de-duplication and note enrichment | 4.5 | 4.2 | 0.554 |
| walker |  | 5913 | 121 | Code::CodeKey { rung: Names, file: internal/adapters/cli/theme.go, decl: 0, sub: 0, line: 0 } |  |  | 0.554 |
| walker |  | 6000 | 87 | Code::CodeKey { rung: Decl, file: internal/adapters/cli/theme.go, decl: 1, sub: 0, line: 11 } |  |  | 0.555 |
| ns | 6089 |  | 204 | Bartib log line format: layouts and FormatActivity | 5.1 |  | 0.545 |
| walker |  | 6173 | 173 | Code::CodeKey { rung: Decl, file: internal/adapters/cli/theme.go, decl: 2, sub: 0, line: 22 } |  |  | 0.545 |
| walker |  | 6187 | 14 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/theme.go, decl: 1, sub: 0, line: 11 } |  |  | 0.546 |
| walker |  | 6203 | 16 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/theme.go, decl: 2, sub: 0, line: 22 } |  |  | 0.546 |
| walker |  | 6219 | 16 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/theme.go, decl: 3, sub: 0, line: 40 } |  |  | 0.546 |
| ns | 6227 |  | 138 | ParseActivity — the reading half of the format | 5.2 | 5.1 | 0.537 |
| walker |  | 6235 | 16 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/theme.go, decl: 9, sub: 0, line: 167 } |  |  | 0.537 |
| walker |  | 6281 | 46 | Code::CodeKey { rung: Names, file: internal/adapters/cli/update.go, decl: 0, sub: 0, line: 0 } |  |  | 0.537 |
| walker |  | 6312 | 31 | Code::CodeKey { rung: Decl, file: internal/adapters/cli/update.go, decl: 1, sub: 0, line: 17 } |  |  | 0.537 |
| walker |  | 6330 | 18 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/theme.go, decl: 4, sub: 0, line: 53 } |  |  | 0.537 |
| walker |  | 6347 | 17 | Code::CodeKey { rung: Names, file: internal/adapters/cli/version.go, decl: 0, sub: 0, line: 0 } |  |  | 0.537 |
| walker |  | 6364 | 17 | Code::CodeKey { rung: Names, file: internal/adapters/cli/add.go, decl: 0, sub: 0, line: 0 } |  |  | 0.537 |
| walker |  | 6381 | 17 | Code::CodeKey { rung: Names, file: internal/adapters/cli/list_gui.go, decl: 0, sub: 0, line: 0 } |  |  | 0.537 |
| walker |  | 6398 | 17 | Code::CodeKey { rung: Names, file: internal/adapters/cli/start.go, decl: 0, sub: 0, line: 0 } |  |  | 0.537 |
| walker |  | 6420 | 22 | Code::CodeKey { rung: Decl, file: internal/adapters/cli/start.go, decl: 1, sub: 0, line: 16 } |  |  | 0.537 |
| ns | 6430 |  | 203 | file repository: struct, constructor and complete method roster | 5.3 |  | 0.529 |
| walker |  | 6448 | 28 | Code::CodeKey { rung: Names, file: internal/adapters/cli/analyze.go, decl: 0, sub: 0, line: 0 } |  |  | 0.530 |
| walker |  | 6603 | 155 | Code::CodeKey { rung: Decl, file: internal/adapters/cli/analyze.go, decl: 2, sub: 0, line: 70 } |  |  | 0.530 |
| walker |  | 6620 | 17 | Code::CodeKey { rung: Names, file: internal/adapters/cli/watch.go, decl: 0, sub: 0, line: 0 } |  |  | 0.530 |
| ns | 6635 |  | 205 | file repository Save — activity identity is minute-precision StartTime | 5.4 | 5.3 | 0.523 |
| walker |  | 6637 | 17 | Code::CodeKey { rung: Names, file: internal/adapters/cli/report.go, decl: 0, sub: 0, line: 0 } |  |  | 0.523 |
| walker |  | 6656 | 19 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/theme.go, decl: 8, sub: 0, line: 122 } |  |  | 0.523 |
| walker |  | 6675 | 19 | Code::CodeKey { rung: Doc, file: internal/core/models/activity.go, decl: 4, sub: 0, line: 31 } |  |  | 0.526 |
| walker |  | 6693 | 18 | Code::CodeKey { rung: Names, file: internal/adapters/cli/ical.go, decl: 0, sub: 0, line: 0 } |  |  | 0.526 |
| walker |  | 6747 | 54 | Code::CodeKey { rung: Names, file: internal/services/ics/generator.go, decl: 0, sub: 0, line: 0 } |  |  | 0.526 |
| walker |  | 6764 | 17 | Code::CodeKey { rung: Doc, file: internal/services/ics/generator.go, decl: 1, sub: 0, line: 12 } |  |  | 0.526 |
| walker |  | 6781 | 17 | Code::CodeKey { rung: Doc, file: internal/services/ics/generator.go, decl: 3, sub: 0, line: 29 } |  |  | 0.526 |
| walker |  | 6801 | 20 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/theme.go, decl: 5, sub: 0, line: 66 } |  |  | 0.526 |
| walker |  | 6821 | 20 | Code::CodeKey { rung: Doc, file: internal/services/ics/generator.go, decl: 2, sub: 0, line: 18 } |  |  | 0.527 |
| walker |  | 6842 | 21 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/theme.go, decl: 6, sub: 0, line: 79 } |  |  | 0.527 |
| ns | 6874 |  | 239 | notes repository: sidecar file layout and YAML front matter | 5.5 |  | 0.516 |
| ns | 7045 |  | 171 | timewarrior repository: interval shape and data layout | 5.6 |  | 0.507 |
| ns | 7231 |  | 186 | timewarrior repository function roster (names only) | 5.7 |  | 0.500 |
| walker |  | 7354 | 512 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.500 |
| ns | 7465 |  | 234 | iCalendar generator | 5.8 |  | 0.494 |
| ns | 7647 |  | 182 | config.Config — the top-level settings struct | 6.1 |  | 0.504 |
| walker |  | 7828 | 474 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.504 |
| ns | 8013 |  | 366 | All nested config structs (complete) | 6.2 |  | 0.524 |
| ns | 8127 |  | 114 | Config defaults | 6.3 |  | 0.520 |
| walker |  | 8155 | 327 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.520 |
| walker |  | 8191 | 36 | Code::CodeKey { rung: Doc, file: internal/timeutil/timeutil.go, decl: 8, sub: 0, line: 56 } |  |  | 0.520 |
| ns | 8452 |  | 325 | Complete TOCK_* environment variable bindings | 6.4 |  | 0.514 |
| walker |  | 8627 | 436 | Markdown::Section { file: README.md, section_index: 12, keeps_default_concavity: false } |  |  | 0.514 |
| ns | 8700 |  | 248 | timeutil.Formatter — 12/24-hour formats and method roster | 6.5 |  | 0.522 |
| ns | 8888 |  | 188 | Theme type and the complete theme constructor roster | 6.6 |  | 0.532 |
| ns | 9046 |  | 158 | GetTheme — accepted theme names and auto-detection | 6.7 | 6.6 | 0.525 |
| walker |  | 9063 | 436 | Markdown::Section { file: README.md, section_index: 13, keeps_default_concavity: false } |  |  | 0.525 |
| walker |  | 9076 | 13 | Code::CodeKey { rung: Body, file: internal/core/models/activity.go, decl: 2, sub: 0, line: 19 } |  |  | 0.526 |
| ns | 9216 |  | 170 | Type roster of the cli package (names only) | 7.1 |  | 0.521 |
| ns | 9371 |  | 155 | AnalysisStats — everything `tock analyze` computes | 7.2 | 7.1 | 0.527 |
| walker |  | 9466 | 390 | Markdown::Section { file: README.md, section_index: 21, keeps_default_concavity: false } |  |  | 0.527 |
| ns | 9517 |  | 146 | Calendar TUI key bindings (complete case list) | 7.3 |  | 0.520 |
| ns | 9754 |  | 237 | Direct dependency set | 8.1 |  | 0.527 |
| ns | 9805 |  | 51 | Peripheral directory listings (docs, demo, assets, .github) | 8.2 |  | 0.533 |
| walker |  | 9929 | 463 | Markdown::Section { file: README.md, section_index: 14, keeps_default_concavity: false } |  |  | 0.533 |
| ns | 9961 |  | 156 | Build, release and mock-generation entry points | 8.3 |  | 0.529 |
| walker |  | 9969 | 40 | Code::CodeKey { rung: Doc, file: internal/timeutil/timeutil.go, decl: 4, sub: 0, line: 25 } |  |  | 0.529 |

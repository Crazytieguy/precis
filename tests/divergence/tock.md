Score(3000)=0.528 I=0.806 C=0.346 ns_rows≤3K=25/59 grid(1000/1442/2080/3000/4327/6240/9000)=0.683/0.687/0.591/0.528/0.558/0.528/0.525

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 49 |  | 49 | Project name and one-line description | 1.1 |  | 0.000 |
| walker |  | 52 | 52 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 56 | 4 | Fs::DirListing { dir: cmd } |  |  | 0.000 |
| walker |  | 60 | 4 | Fs::DirListing { dir: docs } |  |  | 0.000 |
| walker |  | 65 | 5 | Fs::DirListing { dir: demo } |  |  | 0.000 |
| walker |  | 69 | 4 | Fs::DirListing { dir: cmd/tock } |  |  | 0.000 |
| walker |  | 79 | 10 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.131 |
| walker |  | 98 | 19 | Fs::DirListing { dir: internal } |  |  | 0.131 |
| ns | 101 |  | 52 | Repository root listing (complete) | 1.2 |  | 0.742 |
| walker |  | 104 | 6 | Fs::DirListing { dir: internal/adapters } |  |  | 0.750 |
| walker |  | 111 | 7 | Fs::DirListing { dir: internal/services } |  |  | 0.759 |
| walker |  | 115 | 4 | Fs::DirListing { dir: internal/services/ics } |  |  | 0.759 |
| ns | 134 |  | 33 | Module import path and Go version | 1.3 |  | 0.692 |
| walker |  | 135 | 20 | Fs::DirListing { dir: assets } |  |  | 0.693 |
| walker |  | 144 | 9 | Fs::DirListing { dir: internal/config } |  |  | 0.694 |
| walker |  | 153 | 9 | Fs::DirListing { dir: internal/extra } |  |  | 0.696 |
| walker |  | 164 | 11 | Fs::DirListing { dir: internal/timeutil } |  |  | 0.699 |
| walker |  | 176 | 12 | Fs::DirListing { dir: internal/core } |  |  | 0.721 |
| walker |  | 180 | 4 | Fs::DirListing { dir: internal/core/errors } |  |  | 0.723 |
| walker |  | 184 | 4 | Fs::DirListing { dir: internal/core/models } |  |  | 0.725 |
| walker |  | 190 | 6 | Fs::DirListing { dir: internal/core/dto } |  |  | 0.728 |
| walker |  | 197 | 7 | Fs::DirListing { dir: internal/core/ports } |  |  | 0.734 |
| ns | 217 |  | 83 | README feature bullets, part 1 (storage, notes, TUI, footprint) | 1.4 |  | 0.661 |
| walker |  | 230 | 33 | GoMod::Identity { file: go.mod } |  |  | 0.736 |
| walker |  | 239 | 9 | Fs::DirListing { dir: internal/services/activity } |  |  | 0.743 |
| walker |  | 250 | 11 | Fs::DirListing { dir: internal/adapters/repositories } |  |  | 0.743 |
| walker |  | 259 | 9 | Fs::DirListing { dir: internal/adapters/repositories/notes } |  |  | 0.744 |
| walker |  | 268 | 9 | Fs::DirListing { dir: internal/adapters/repositories/timewarrior } |  |  | 0.745 |
| walker |  | 282 | 14 | Fs::DirListing { dir: .github } |  |  | 0.746 |
| walker |  | 290 | 8 | Fs::DirListing { dir: .github/workflows } |  |  | 0.746 |
| ns | 294 |  | 77 | README feature bullets, part 2 (Bartib/TimeWarrior compat, themes, iCal) | 1.5 |  | 0.697 |
| ns | 362 |  | 68 | Binary entry point | 1.6 |  | 0.589 |
| ns | 414 |  | 52 | Package tree: cmd/, internal/ and its layer directories | 1.7 |  | 0.637 |
| ns | 495 |  | 81 | Domain, service and support package file listings (complete) | 1.8 |  | 0.621 |
| ns | 621 |  | 126 | Adapter package listings: cli/ and the three repositories | 1.9 |  | 0.511 |
| walker |  | 623 | 333 | Plaintext::Whole { file: Makefile } |  |  | 0.511 |
| walker |  | 641 | 18 | Fs::DirListing { dir: internal/adapters/repositories/file } |  |  | 0.526 |
| walker |  | 659 | 18 | Fs::DirListing { dir: internal/core/ports/mocks } |  |  | 0.561 |
| ns | 726 |  | 105 | README section headings (all H2) | 1.10 |  | 0.528 |
| ns | 900 |  | 174 | ports.ActivityResolver — the full service contract | 2.1 |  | 0.502 |
| walker |  | 923 | 264 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.683 |
| walker |  | 1002 | 79 | Fs::DirListing { dir: internal/adapters/cli } |  |  | 0.832 |
| ns | 1024 |  | 124 | models.Activity struct with JSON tags | 2.2 |  | 0.798 |
| ns | 1177 |  | 153 | ports.ActivityRepository and ports.NotesRepository | 2.3 |  | 0.761 |
| walker |  | 1244 | 242 | GoMod::File { file: go.mod } |  |  | 0.762 |
| ns | 1255 |  | 78 | Domain sentinel errors | 2.4 |  | 0.744 |
| walker |  | 1269 | 25 | Markdown::Section { file: README.md, section_index: 26, keeps_default_concavity: false } |  |  | 0.745 |
| ns | 1442 |  | 187 | dto request types for Start / Stop / Add | 2.5 |  | 0.687 |
| ns | 1508 |  | 66 | dto.ActivityFilter — the query type | 2.6 |  | 0.670 |
| walker |  | 1548 | 279 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.670 |
| ns | 1603 |  | 95 | dto.Report and dto.ProjectReport | 2.7 |  | 0.647 |
| ns | 1687 |  | 84 | models.Activity method roster (bodies elided) | 2.8 |  | 0.630 |
| walker |  | 1736 | 188 | Markdown::HeadingsOutline { file: docs/commands.md } |  |  | 0.630 |
| walker |  | 1827 | 91 | Markdown::Section { file: README.md, section_index: 19, keeps_default_concavity: false } |  |  | 0.632 |
| ns | 1834 |  | 147 | models.Activity method bodies | 2.9 | 2.8 | 0.602 |
| ns | 2002 |  | 168 | Root command declaration and the three persistent flags | 3.1 |  | 0.591 |
| ns | 2164 |  | 162 | Complete subcommand registration list | 3.2 |  | 0.569 |
| ns | 2400 |  | 236 | Command declarations, part 1: add, analyze, calendar, continue, current, ical | 3.3 |  | 0.550 |
| walker |  | 2461 | 634 | Plaintext::Whole { file: install.sh } |  |  | 0.550 |
| walker |  | 2551 | 90 | Markdown::Section { file: README.md, section_index: 25, keeps_default_concavity: false } |  |  | 0.552 |
| walker |  | 2563 | 12 | Code::CodeKey { rung: Names, file: cmd/tock/main.go, decl: 0, sub: 0, line: 0 } |  |  | 0.553 |
| walker |  | 2571 | 8 | Code::CodeKey { rung: Body, file: cmd/tock/main.go, decl: 1, sub: 0, line: 7 } |  |  | 0.555 |
| walker |  | 2696 | 125 | Markdown::Section { file: README.md, section_index: 20, keeps_default_concavity: false } |  |  | 0.557 |
| ns | 2735 |  | 335 | Command declarations, part 2: last, list, remove, report, start, stop, version, watch | 3.4 |  | 0.532 |
| ns | 2766 |  | 31 | docs/commands.md structure | 3.5 |  | 0.537 |
| walker |  | 2864 | 168 | Code::CodeKey { rung: Names, file: internal/config/config.go, decl: 0, sub: 0, line: 0 } |  |  | 0.537 |
| walker |  | 2879 | 15 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 5, sub: 0, line: 41 } |  |  | 0.537 |
| walker |  | 2896 | 17 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 4, sub: 0, line: 37 } |  |  | 0.537 |
| walker |  | 2913 | 17 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 6, sub: 0, line: 45 } |  |  | 0.537 |
| walker |  | 2932 | 19 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 3, sub: 0, line: 33 } |  |  | 0.537 |
| ns | 2958 |  | 192 | Flags of `start` and `stop` | 3.6 |  | 0.528 |
| walker |  | 3022 | 90 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 2, sub: 0, line: 25 } |  |  | 0.529 |
| walker |  | 3136 | 114 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 7, sub: 0, line: 49 } |  |  | 0.530 |
| ns | 3282 |  | 324 | Flags of `add` and `continue` | 3.7 |  | 0.517 |
| walker |  | 3304 | 168 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 1, sub: 0, line: 12 } |  |  | 0.518 |
| walker |  | 3493 | 189 | Code::CodeKey { rung: Names, file: internal/timeutil/timeutil.go, decl: 0, sub: 0, line: 0 } |  |  | 0.519 |
| walker |  | 3502 | 9 | Code::CodeKey { rung: Decl, file: internal/timeutil/timeutil.go, decl: 2, sub: 0, line: 13 } |  |  | 0.519 |
| walker |  | 3515 | 13 | Code::CodeKey { rung: Decl, file: internal/timeutil/timeutil.go, decl: 3, sub: 0, line: 19 } |  |  | 0.519 |
| walker |  | 3523 | 8 | Code::CodeKey { rung: Body, file: internal/timeutil/timeutil.go, decl: 5, sub: 0, line: 34 } |  |  | 0.519 |
| walker |  | 3533 | 10 | Code::CodeKey { rung: Doc, file: internal/timeutil/timeutil.go, decl: 3, sub: 0, line: 19 } |  |  | 0.519 |
| walker |  | 3547 | 14 | Code::CodeKey { rung: Doc, file: internal/timeutil/timeutil.go, decl: 1, sub: 0, line: 11 } |  |  | 0.520 |
| walker |  | 3562 | 15 | Code::CodeKey { rung: Doc, file: internal/timeutil/timeutil.go, decl: 5, sub: 0, line: 34 } |  |  | 0.520 |
| walker |  | 3578 | 16 | Code::CodeKey { rung: Doc, file: internal/timeutil/timeutil.go, decl: 7, sub: 0, line: 47 } |  |  | 0.520 |
| ns | 3585 |  | 303 | Flags of `report`, `last` and `analyze` | 3.8 |  | 0.509 |
| walker |  | 3595 | 17 | Code::CodeKey { rung: Doc, file: internal/timeutil/timeutil.go, decl: 6, sub: 0, line: 39 } |  |  | 0.509 |
| walker |  | 3631 | 36 | Code::CodeKey { rung: Doc, file: internal/timeutil/timeutil.go, decl: 8, sub: 0, line: 56 } |  |  | 0.509 |
| walker |  | 3671 | 40 | Code::CodeKey { rung: Doc, file: internal/timeutil/timeutil.go, decl: 4, sub: 0, line: 25 } |  |  | 0.509 |
| walker |  | 3715 | 44 | Code::CodeKey { rung: Doc, file: internal/timeutil/timeutil.go, decl: 9, sub: 0, line: 103 } |  |  | 0.509 |
| ns | 3777 |  | 192 | Flags of `current`, `watch`, `ical` and `remove` | 3.9 |  | 0.502 |
| walker |  | 3787 | 72 | Code::CodeKey { rung: Doc, file: internal/timeutil/timeutil.go, decl: 10, sub: 0, line: 158 } |  |  | 0.502 |
| walker |  | 3822 | 35 | Code::CodeKey { rung: Names, file: internal/core/ports/ports.go, decl: 0, sub: 0, line: 0 } |  |  | 0.503 |
| walker |  | 3877 | 55 | Code::CodeKey { rung: Decl, file: internal/core/ports/ports.go, decl: 3, sub: 0, line: 29 } |  |  | 0.509 |
| walker |  | 3950 | 73 | Code::CodeKey { rung: Decl, file: internal/core/ports/ports.go, decl: 2, sub: 0, line: 22 } |  |  | 0.531 |
| walker |  | 4109 | 159 | Code::CodeKey { rung: Decl, file: internal/core/ports/ports.go, decl: 1, sub: 0, line: 11 } |  |  | 0.559 |
| ns | 4172 |  | 395 | PersistentPreRunE — dependency injection and backend selection | 3.10 |  | 0.526 |
| walker |  | 4179 | 70 | Code::CodeKey { rung: Names, file: internal/core/dto/activity_dto.go, decl: 0, sub: 0, line: 0 } |  |  | 0.530 |
| walker |  | 4210 | 31 | Code::CodeKey { rung: Decl, file: internal/core/dto/activity_dto.go, decl: 2, sub: 0, line: 17 } |  |  | 0.535 |
| walker |  | 4243 | 33 | Code::CodeKey { rung: Decl, file: internal/core/dto/activity_dto.go, decl: 6, sub: 0, line: 46 } |  |  | 0.542 |
| walker |  | 4281 | 38 | Code::CodeKey { rung: Decl, file: internal/core/dto/activity_dto.go, decl: 5, sub: 0, line: 40 } |  |  | 0.558 |
| walker |  | 4329 | 48 | Code::CodeKey { rung: Decl, file: internal/core/dto/activity_dto.go, decl: 1, sub: 0, line: 9 } |  |  | 0.573 |
| walker |  | 4384 | 55 | Code::CodeKey { rung: Decl, file: internal/core/dto/activity_dto.go, decl: 4, sub: 0, line: 32 } |  |  | 0.589 |
| ns | 4406 |  | 234 | Execute, the context keys, and the accessor helpers | 3.11 |  | 0.567 |
| walker |  | 4442 | 58 | Code::CodeKey { rung: Decl, file: internal/core/dto/activity_dto.go, decl: 3, sub: 0, line: 23 } |  |  | 0.593 |
| ns | 4550 |  | 144 | `current --format` template variables (Long help) | 3.12 |  | 0.584 |
| ns | 4715 |  | 165 | `ical` argument semantics and `remove` Long help | 3.13 |  | 0.577 |
| walker |  | 4733 | 291 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.579 |
| ns | 4837 |  | 122 | Shell-completion machinery | 3.14 |  | 0.573 |
| ns | 4928 |  | 91 | activity.service struct and constructor | 4.1 |  | 0.567 |
| walker |  | 5000 | 267 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.567 |
| ns | 5195 |  | 267 | service method roster (bodies elided) | 4.2 |  | 0.552 |
| walker |  | 5233 | 233 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.554 |
| walker |  | 5354 | 121 | Code::CodeKey { rung: Names, file: internal/adapters/cli/theme.go, decl: 0, sub: 0, line: 0 } |  |  | 0.554 |
| ns | 5429 |  | 234 | Service.Start — implicit stop of running activities | 4.3 | 4.2 | 0.538 |
| walker |  | 5441 | 87 | Code::CodeKey { rung: Decl, file: internal/adapters/cli/theme.go, decl: 1, sub: 0, line: 11 } |  |  | 0.539 |
| walker |  | 5455 | 14 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/theme.go, decl: 1, sub: 0, line: 11 } |  |  | 0.539 |
| walker |  | 5471 | 16 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/theme.go, decl: 9, sub: 0, line: 167 } |  |  | 0.539 |
| walker |  | 5489 | 18 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/theme.go, decl: 3, sub: 0, line: 40 } |  |  | 0.539 |
| walker |  | 5507 | 18 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/theme.go, decl: 4, sub: 0, line: 53 } |  |  | 0.539 |
| walker |  | 5526 | 19 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/theme.go, decl: 8, sub: 0, line: 122 } |  |  | 0.539 |
| walker |  | 5546 | 20 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/theme.go, decl: 5, sub: 0, line: 66 } |  |  | 0.539 |
| walker |  | 5567 | 21 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/theme.go, decl: 6, sub: 0, line: 79 } |  |  | 0.539 |
| ns | 5648 |  | 219 | Service.Stop — target selection and validation | 4.4 | 4.2 | 0.524 |
| walker |  | 5738 | 171 | Code::CodeKey { rung: Decl, file: internal/adapters/cli/theme.go, decl: 2, sub: 0, line: 22 } |  |  | 0.524 |
| walker |  | 5754 | 16 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/theme.go, decl: 2, sub: 0, line: 22 } |  |  | 0.524 |
| walker |  | 5783 | 29 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/theme.go, decl: 7, sub: 0, line: 93 } |  |  | 0.524 |
| walker |  | 5814 | 31 | Code::CodeKey { rung: Body, file: internal/config/config.go, decl: 9, sub: 0, line: 62 } |  |  | 0.524 |
| walker |  | 5860 | 46 | Code::CodeKey { rung: Names, file: internal/adapters/cli/update.go, decl: 0, sub: 0, line: 0 } |  |  | 0.524 |
| ns | 5887 |  | 239 | GetRecent de-duplication and note enrichment | 4.5 | 4.2 | 0.511 |
| walker |  | 5891 | 31 | Code::CodeKey { rung: Decl, file: internal/adapters/cli/update.go, decl: 1, sub: 0, line: 17 } |  |  | 0.511 |
| walker |  | 5970 | 79 | Code::CodeKey { rung: Names, file: internal/core/models/activity.go, decl: 0, sub: 0, line: 0 } |  |  | 0.516 |
| walker |  | 5983 | 13 | Code::CodeKey { rung: Body, file: internal/core/models/activity.go, decl: 2, sub: 0, line: 19 } |  |  | 0.518 |
| walker |  | 6081 | 98 | Code::CodeKey { rung: Decl, file: internal/core/models/activity.go, decl: 1, sub: 0, line: 10 } |  |  | 0.532 |
| ns | 6091 |  | 204 | Bartib log line format: layouts and FormatActivity | 5.1 |  | 0.522 |
| walker |  | 6093 | 12 | Code::CodeKey { rung: Doc, file: internal/core/models/activity.go, decl: 1, sub: 0, line: 10 } |  |  | 0.526 |
| walker |  | 6112 | 19 | Code::CodeKey { rung: Doc, file: internal/core/models/activity.go, decl: 4, sub: 0, line: 31 } |  |  | 0.530 |
| walker |  | 6155 | 43 | Code::CodeKey { rung: Body, file: internal/core/models/activity.go, decl: 3, sub: 0, line: 23 } |  |  | 0.536 |
| ns | 6229 |  | 138 | ParseActivity — the reading half of the format | 5.2 | 5.1 | 0.528 |
| walker |  | 6420 | 265 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.528 |
| ns | 6432 |  | 203 | file repository: struct, constructor and complete method roster | 5.3 |  | 0.520 |
| walker |  | 6452 | 32 | Code::CodeKey { rung: Names, file: internal/adapters/cli/continue.go, decl: 0, sub: 0, line: 0 } |  |  | 0.520 |
| walker |  | 6463 | 11 | Code::CodeKey { rung: Decl, file: internal/adapters/cli/continue.go, decl: 1, sub: 0, line: 15 } |  |  | 0.520 |
| walker |  | 6479 | 16 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/continue.go, decl: 2, sub: 0, line: 20 } |  |  | 0.520 |
| walker |  | 6619 | 140 | Plaintext::DeclSurface { file: demo/demo_script.sh } |  |  | 0.520 |
| ns | 6637 |  | 205 | file repository Save — activity identity is minute-precision StartTime | 5.4 | 5.3 | 0.513 |
| walker |  | 6685 | 66 | Code::CodeKey { rung: Names, file: internal/adapters/cli/current.go, decl: 0, sub: 0, line: 0 } |  |  | 0.513 |
| walker |  | 6696 | 11 | Code::CodeKey { rung: Decl, file: internal/adapters/cli/current.go, decl: 1, sub: 0, line: 19 } |  |  | 0.513 |
| walker |  | 6709 | 13 | Code::CodeKey { rung: Body, file: internal/adapters/cli/current.go, decl: 2, sub: 0, line: 23 } |  |  | 0.513 |
| walker |  | 6759 | 50 | Code::CodeKey { rung: Names, file: internal/adapters/cli/last.go, decl: 0, sub: 0, line: 0 } |  |  | 0.513 |
| walker |  | 6780 | 21 | Code::CodeKey { rung: Decl, file: internal/adapters/cli/last.go, decl: 1, sub: 0, line: 16 } |  |  | 0.513 |
| walker |  | 6830 | 50 | Code::CodeKey { rung: Names, file: internal/adapters/cli/report.go, decl: 0, sub: 0, line: 0 } |  |  | 0.513 |
| ns | 6876 |  | 239 | notes repository: sidecar file layout and YAML front matter | 5.5 |  | 0.503 |
| walker |  | 6904 | 74 | Code::CodeKey { rung: Decl, file: internal/adapters/cli/report.go, decl: 1, sub: 0, line: 20 } |  |  | 0.503 |
| walker |  | 6922 | 18 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/report.go, decl: 3, sub: 0, line: 61 } |  |  | 0.503 |
| walker |  | 6939 | 17 | Code::CodeKey { rung: Names, file: internal/adapters/cli/start.go, decl: 0, sub: 0, line: 0 } |  |  | 0.503 |
| walker |  | 6961 | 22 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/start.go, decl: 1, sub: 0, line: 17 } |  |  | 0.503 |
| walker |  | 6978 | 17 | Code::CodeKey { rung: Names, file: internal/adapters/cli/stop.go, decl: 0, sub: 0, line: 0 } |  |  | 0.503 |
| ns | 7047 |  | 171 | timewarrior repository: interval shape and data layout | 5.6 |  | 0.494 |
| walker |  | 7148 | 170 | Code::CodeKey { rung: Names, file: internal/adapters/cli/watch.go, decl: 0, sub: 0, line: 0 } |  |  | 0.494 |
| walker |  | 7170 | 22 | Code::CodeKey { rung: Decl, file: internal/adapters/cli/watch.go, decl: 4, sub: 0, line: 90 } |  |  | 0.494 |
| ns | 7233 |  | 186 | timewarrior repository function roster (names only) | 5.7 |  | 0.487 |
| walker |  | 7262 | 92 | Code::CodeKey { rung: Decl, file: internal/adapters/cli/watch.go, decl: 3, sub: 0, line: 77 } |  |  | 0.487 |
| walker |  | 7296 | 34 | Code::CodeKey { rung: Body, file: internal/adapters/cli/watch.go, decl: 6, sub: 0, line: 115 } |  |  | 0.487 |
| ns | 7467 |  | 234 | iCalendar generator | 5.8 |  | 0.478 |
| walker |  | 7569 | 273 | Code::CodeKey { rung: Decl, file: internal/adapters/cli/watch.go, decl: 9, sub: 0, line: 248 } |  |  | 0.478 |
| walker |  | 7637 | 68 | Code::CodeKey { rung: Names, file: internal/services/ics/generator.go, decl: 0, sub: 0, line: 0 } |  |  | 0.480 |
| ns | 7649 |  | 182 | config.Config — the top-level settings struct | 6.1 |  | 0.491 |
| walker |  | 7654 | 17 | Code::CodeKey { rung: Doc, file: internal/services/ics/generator.go, decl: 1, sub: 0, line: 12 } |  |  | 0.492 |
| walker |  | 7671 | 17 | Code::CodeKey { rung: Doc, file: internal/services/ics/generator.go, decl: 3, sub: 0, line: 29 } |  |  | 0.492 |
| walker |  | 7691 | 20 | Code::CodeKey { rung: Doc, file: internal/services/ics/generator.go, decl: 2, sub: 0, line: 18 } |  |  | 0.494 |
| walker |  | 7713 | 22 | Code::CodeKey { rung: Body, file: internal/services/ics/generator.go, decl: 1, sub: 0, line: 12 } |  |  | 0.496 |
| walker |  | 7793 | 80 | Code::CodeKey { rung: Body, file: internal/adapters/cli/current.go, decl: 3, sub: 0, line: 27 } |  |  | 0.496 |
| walker |  | 7865 | 72 | Code::CodeKey { rung: Names, file: internal/adapters/cli/analyze.go, decl: 0, sub: 0, line: 0 } |  |  | 0.496 |
| walker |  | 7882 | 17 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/analyze.go, decl: 3, sub: 0, line: 84 } |  |  | 0.496 |
| walker |  | 7902 | 20 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/analyze.go, decl: 4, sub: 0, line: 206 } |  |  | 0.496 |
| ns | 8015 |  | 366 | All nested config structs (complete) | 6.2 |  | 0.516 |
| walker |  | 8055 | 153 | Code::CodeKey { rung: Decl, file: internal/adapters/cli/analyze.go, decl: 2, sub: 0, line: 70 } |  |  | 0.517 |
| ns | 8129 |  | 114 | Config defaults | 6.3 |  | 0.513 |
| walker |  | 8321 | 266 | Markdown::Section { file: docs/commands.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.513 |
| walker |  | 8432 | 111 | Code::CodeKey { rung: Names, file: internal/adapters/cli/calendar_sidebar.go, decl: 0, sub: 0, line: 0 } |  |  | 0.513 |
| walker |  | 8451 | 19 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/calendar_sidebar.go, decl: 2, sub: 0, line: 15 } |  |  | 0.513 |
| ns | 8454 |  | 325 | Complete TOCK_* environment variable bindings | 6.4 |  | 0.507 |
| walker |  | 8661 | 210 | Code::CodeKey { rung: Names, file: internal/adapters/cli/list_gui.go, decl: 0, sub: 0, line: 0 } |  |  | 0.507 |
| walker |  | 8668 | 7 | Code::CodeKey { rung: Body, file: internal/adapters/cli/list_gui.go, decl: 9, sub: 0, line: 200 } |  |  | 0.507 |
| ns | 8702 |  | 248 | timeutil.Formatter — 12/24-hour formats and method roster | 6.5 |  | 0.514 |
| walker |  | 8766 | 98 | Code::CodeKey { rung: Decl, file: internal/adapters/cli/list_gui.go, decl: 2, sub: 0, line: 45 } |  |  | 0.514 |
| walker |  | 8841 | 75 | Code::CodeKey { rung: Body, file: internal/adapters/cli/list_gui.go, decl: 5, sub: 0, line: 101 } |  |  | 0.514 |
| ns | 8890 |  | 188 | Theme type and the complete theme constructor roster | 6.6 |  | 0.525 |
| walker |  | 8929 | 88 | Code::CodeKey { rung: Body, file: internal/adapters/cli/list_gui.go, decl: 3, sub: 0, line: 57 } |  |  | 0.525 |
| ns | 9048 |  | 158 | GetTheme — accepted theme names and auto-detection | 6.7 | 6.6 | 0.517 |
| walker |  | 9198 | 269 | Code::CodeKey { rung: Names, file: internal/adapters/cli/root.go, decl: 0, sub: 0, line: 0 } |  |  | 0.526 |
| walker |  | 9207 | 9 | Code::CodeKey { rung: Decl, file: internal/adapters/cli/root.go, decl: 1, sub: 0, line: 21 } |  |  | 0.526 |
| ns | 9218 |  | 170 | Type roster of the cli package (names only) | 7.1 |  | 0.525 |
| walker |  | 9234 | 27 | Code::CodeKey { rung: Body, file: internal/adapters/cli/root.go, decl: 9, sub: 0, line: 119 } |  |  | 0.526 |
| walker |  | 9262 | 28 | Code::CodeKey { rung: Body, file: internal/adapters/cli/root.go, decl: 8, sub: 0, line: 115 } |  |  | 0.527 |
| walker |  | 9292 | 30 | Code::CodeKey { rung: Body, file: internal/adapters/cli/root.go, decl: 10, sub: 0, line: 123 } |  |  | 0.527 |
| walker |  | 9335 | 43 | Code::CodeKey { rung: Body, file: internal/adapters/cli/root.go, decl: 11, sub: 0, line: 127 } |  |  | 0.534 |
| walker |  | 9366 | 31 | Code::CodeKey { rung: Body, file: internal/config/config.go, decl: 10, sub: 0, line: 68 } |  |  | 0.534 |
| ns | 9373 |  | 155 | AnalysisStats — everything `tock analyze` computes | 7.2 | 7.1 | 0.540 |
| ns | 9519 |  | 146 | Calendar TUI key bindings (complete case list) | 7.3 |  | 0.532 |
| walker |  | 9607 | 241 | Markdown::Section { file: README.md, section_index: 22, keeps_default_concavity: false } |  |  | 0.534 |
| ns | 9756 |  | 237 | Direct dependency set | 8.1 |  | 0.541 |
| ns | 9807 |  | 51 | Peripheral directory listings (docs, demo, assets, .github) | 8.2 |  | 0.546 |
| walker |  | 9906 | 299 | Code::CodeKey { rung: Names, file: internal/adapters/cli/calendar.go, decl: 0, sub: 0, line: 0 } |  |  | 0.551 |
| walker |  | 9921 | 15 | Code::CodeKey { rung: Decl, file: internal/adapters/cli/calendar.go, decl: 11, sub: 0, line: 418 } |  |  | 0.551 |
| walker |  | 9931 | 10 | Code::CodeKey { rung: Body, file: internal/adapters/cli/calendar.go, decl: 4, sub: 0, line: 79 } |  |  | 0.551 |
| walker |  | 9956 | 25 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/calendar.go, decl: 9, sub: 0, line: 256 } |  |  | 0.551 |
| ns | 9963 |  | 156 | Build, release and mock-generation entry points | 8.3 |  | 0.547 |

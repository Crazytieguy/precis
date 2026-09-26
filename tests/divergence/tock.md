Score(3000)=0.552 I=0.826 C=0.368 ns_rows≤3K=25/59 grid(1000/1442/2080/3000/4327/6240/9000)=0.712/0.733/0.628/0.552/0.576/0.537/0.531

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 49 |  | 49 | Project name and one-line description | 1.1 |  | 0.000 |
| walker |  | 52 | 52 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 56 | 4 | Fs::DirListing { dir: docs } |  |  | 0.000 |
| walker |  | 61 | 5 | Fs::DirListing { dir: demo } |  |  | 0.000 |
| ns | 101 |  | 52 | Repository root listing (complete) | 1.2 |  | 0.719 |
| walker |  | 119 | 58 | Markdown::ReadmeHeadline { file: README.md } |  |  | 1.000 |
| walker |  | 125 | 6 | Fs::DirListing { dir: cmd/tock } |  |  | 1.000 |
| ns | 134 |  | 33 | Module import path and Go version | 1.3 |  | 0.912 |
| walker |  | 144 | 19 | Fs::DirListing { dir: internal } |  |  | 0.923 |
| walker |  | 150 | 6 | Fs::DirListing { dir: internal/adapters } |  |  | 0.929 |
| walker |  | 157 | 7 | Fs::DirListing { dir: internal/services } |  |  | 0.929 |
| walker |  | 161 | 4 | Fs::DirListing { dir: internal/services/ics } |  |  | 0.929 |
| walker |  | 181 | 20 | Fs::DirListing { dir: assets } |  |  | 0.929 |
| walker |  | 214 | 33 | GoMod::Identity { file: go.mod } |  |  | 1.000 |
| ns | 217 |  | 83 | README feature bullets, part 1 (storage, notes, TUI, footprint) | 1.4 |  | 0.920 |
| walker |  | 223 | 9 | Fs::DirListing { dir: internal/config } |  |  | 0.920 |
| walker |  | 232 | 9 | Fs::DirListing { dir: internal/extra } |  |  | 0.920 |
| walker |  | 243 | 11 | Fs::DirListing { dir: internal/timeutil } |  |  | 0.920 |
| walker |  | 255 | 12 | Fs::DirListing { dir: internal/core } |  |  | 0.920 |
| walker |  | 259 | 4 | Fs::DirListing { dir: internal/core/errors } |  |  | 0.920 |
| walker |  | 263 | 4 | Fs::DirListing { dir: internal/core/models } |  |  | 0.920 |
| walker |  | 269 | 6 | Fs::DirListing { dir: internal/core/dto } |  |  | 0.920 |
| walker |  | 276 | 7 | Fs::DirListing { dir: internal/core/ports } |  |  | 0.920 |
| walker |  | 285 | 9 | Fs::DirListing { dir: internal/services/activity } |  |  | 0.920 |
| ns | 294 |  | 77 | README feature bullets, part 2 (Bartib/TimeWarrior compat, themes, iCal) | 1.5 |  | 0.871 |
| walker |  | 296 | 11 | Fs::DirListing { dir: internal/adapters/repositories } |  |  | 0.871 |
| walker |  | 305 | 9 | Fs::DirListing { dir: internal/adapters/repositories/notes } |  |  | 0.871 |
| walker |  | 314 | 9 | Fs::DirListing { dir: internal/adapters/repositories/timewarrior } |  |  | 0.871 |
| walker |  | 328 | 14 | Fs::DirListing { dir: .github } |  |  | 0.871 |
| walker |  | 336 | 8 | Fs::DirListing { dir: .github/workflows } |  |  | 0.871 |
| ns | 362 |  | 68 | Binary entry point | 1.6 |  | 0.761 |
| ns | 412 |  | 50 | Package tree: cmd/, internal/ and its layer directories | 1.7 |  | 0.809 |
| ns | 493 |  | 81 | Domain, service and support package file listings (complete) | 1.8 |  | 0.782 |
| ns | 619 |  | 126 | Adapter package listings: cli/ and the three repositories | 1.9 |  | 0.644 |
| walker |  | 669 | 333 | Plaintext::Whole { file: Makefile } |  |  | 0.644 |
| walker |  | 687 | 18 | Fs::DirListing { dir: internal/adapters/repositories/file } |  |  | 0.661 |
| ns | 724 |  | 105 | README section headings (all H2) | 1.10 |  | 0.622 |
| walker |  | 783 | 96 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.690 |
| walker |  | 827 | 44 | Markdown::CommandBlock { file: README.md, row: 49 } |  |  | 0.690 |
| ns | 898 |  | 174 | ports.ActivityResolver — the full service contract | 2.1 |  | 0.656 |
| walker |  | 997 | 170 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.712 |
| walker |  | 1015 | 18 | Fs::DirListing { dir: internal/core/ports/mocks } |  |  | 0.745 |
| ns | 1022 |  | 124 | models.Activity struct with JSON tags | 2.2 |  | 0.715 |
| walker |  | 1094 | 79 | Fs::DirListing { dir: internal/adapters/cli } |  |  | 0.853 |
| walker |  | 1173 | 79 | Markdown::Section { file: README.md, section_index: 19, keeps_default_concavity: false } |  |  | 0.853 |
| ns | 1175 |  | 153 | ports.ActivityRepository and ports.NotesRepository | 2.3 |  | 0.813 |
| ns | 1253 |  | 78 | Domain sentinel errors | 2.4 |  | 0.794 |
| walker |  | 1415 | 242 | GoMod::File { file: go.mod } |  |  | 0.795 |
| ns | 1440 |  | 187 | dto request types for Start / Stop / Add | 2.5 |  | 0.733 |
| walker |  | 1496 | 81 | Markdown::Section { file: README.md, section_index: 25, keeps_default_concavity: false } |  |  | 0.733 |
| ns | 1506 |  | 66 | dto.ActivityFilter — the query type | 2.6 |  | 0.715 |
| ns | 1601 |  | 95 | dto.Report and dto.ProjectReport | 2.7 |  | 0.691 |
| walker |  | 1611 | 115 | Markdown::Section { file: README.md, section_index: 20, keeps_default_concavity: false } |  |  | 0.691 |
| ns | 1685 |  | 84 | models.Activity method roster (bodies elided) | 2.8 |  | 0.672 |
| walker |  | 1723 | 112 | Plaintext::DeclSurface { file: install.sh } |  |  | 0.672 |
| ns | 1832 |  | 147 | models.Activity method bodies | 2.9 | 2.8 | 0.640 |
| walker |  | 1962 | 239 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.640 |
| ns | 2000 |  | 168 | Root command declaration and the three persistent flags | 3.1 |  | 0.628 |
| ns | 2162 |  | 162 | Complete subcommand registration list | 3.2 |  | 0.605 |
| walker |  | 2193 | 231 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.605 |
| walker |  | 2205 | 12 | Code::CodeKey { rung: Names, file: cmd/tock/main.go, decl: 0, sub: 0, line: 0 } |  |  | 0.605 |
| walker |  | 2216 | 11 | Code::CodeKey { rung: Decl, file: cmd/tock/main.go, decl: 1, sub: 0, line: 7 } |  |  | 0.609 |
| ns | 2398 |  | 236 | Command declarations, part 1: add, analyze, calendar, continue, current, ical | 3.3 |  | 0.588 |
| walker |  | 2439 | 223 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.588 |
| walker |  | 2607 | 168 | Code::CodeKey { rung: Names, file: internal/config/config.go, decl: 0, sub: 0, line: 0 } |  |  | 0.589 |
| walker |  | 2622 | 15 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 5, sub: 0, line: 41 } |  |  | 0.589 |
| walker |  | 2639 | 17 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 4, sub: 0, line: 37 } |  |  | 0.589 |
| walker |  | 2656 | 17 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 6, sub: 0, line: 45 } |  |  | 0.589 |
| walker |  | 2675 | 19 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 3, sub: 0, line: 33 } |  |  | 0.589 |
| ns | 2733 |  | 335 | Command declarations, part 2: last, list, remove, report, start, stop, version, watch | 3.4 |  | 0.562 |
| ns | 2764 |  | 31 | docs/commands.md structure | 3.5 |  | 0.558 |
| walker |  | 2765 | 90 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 2, sub: 0, line: 25 } |  |  | 0.559 |
| walker |  | 2879 | 114 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 7, sub: 0, line: 49 } |  |  | 0.560 |
| ns | 2956 |  | 192 | Flags of `start` and `stop` | 3.6 |  | 0.551 |
| walker |  | 3047 | 168 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 1, sub: 0, line: 12 } |  |  | 0.552 |
| ns | 3280 |  | 324 | Flags of `add` and `continue` | 3.7 |  | 0.539 |
| walker |  | 3312 | 265 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.539 |
| walker |  | 3510 | 198 | Code::CodeKey { rung: Names, file: internal/timeutil/timeutil.go, decl: 0, sub: 0, line: 0 } |  |  | 0.540 |
| walker |  | 3515 | 5 | Code::CodeKey { rung: Decl, file: internal/timeutil/timeutil.go, decl: 2, sub: 0, line: 13 } |  |  | 0.540 |
| walker |  | 3522 | 7 | Code::CodeKey { rung: Decl, file: internal/timeutil/timeutil.go, decl: 3, sub: 0, line: 19 } |  |  | 0.540 |
| walker |  | 3532 | 10 | Code::CodeKey { rung: Doc, file: internal/timeutil/timeutil.go, decl: 3, sub: 0, line: 19 } |  |  | 0.540 |
| walker |  | 3546 | 14 | Code::CodeKey { rung: Doc, file: internal/timeutil/timeutil.go, decl: 1, sub: 0, line: 11 } |  |  | 0.540 |
| walker |  | 3554 | 8 | Code::CodeKey { rung: Body, file: internal/timeutil/timeutil.go, decl: 5, sub: 0, line: 34 } |  |  | 0.540 |
| walker |  | 3569 | 15 | Code::CodeKey { rung: Doc, file: internal/timeutil/timeutil.go, decl: 5, sub: 0, line: 34 } |  |  | 0.540 |
| ns | 3583 |  | 303 | Flags of `report`, `last` and `analyze` | 3.8 |  | 0.529 |
| walker |  | 3585 | 16 | Code::CodeKey { rung: Doc, file: internal/timeutil/timeutil.go, decl: 7, sub: 0, line: 47 } |  |  | 0.529 |
| walker |  | 3602 | 17 | Code::CodeKey { rung: Doc, file: internal/timeutil/timeutil.go, decl: 6, sub: 0, line: 39 } |  |  | 0.529 |
| ns | 3775 |  | 192 | Flags of `current`, `watch`, `ical` and `remove` | 3.9 |  | 0.522 |
| walker |  | 3833 | 231 | Markdown::Section { file: README.md, section_index: 22, keeps_default_concavity: false } |  |  | 0.522 |
| walker |  | 3868 | 35 | Code::CodeKey { rung: Names, file: internal/core/ports/ports.go, decl: 0, sub: 0, line: 0 } |  |  | 0.523 |
| walker |  | 3923 | 55 | Code::CodeKey { rung: Decl, file: internal/core/ports/ports.go, decl: 3, sub: 0, line: 29 } |  |  | 0.529 |
| walker |  | 3996 | 73 | Code::CodeKey { rung: Decl, file: internal/core/ports/ports.go, decl: 2, sub: 0, line: 22 } |  |  | 0.550 |
| walker |  | 4155 | 159 | Code::CodeKey { rung: Decl, file: internal/core/ports/ports.go, decl: 1, sub: 0, line: 11 } |  |  | 0.578 |
| ns | 4170 |  | 395 | PersistentPreRunE — dependency injection and backend selection | 3.10 |  | 0.544 |
| walker |  | 4225 | 70 | Code::CodeKey { rung: Names, file: internal/core/dto/activity_dto.go, decl: 0, sub: 0, line: 0 } |  |  | 0.547 |
| walker |  | 4256 | 31 | Code::CodeKey { rung: Decl, file: internal/core/dto/activity_dto.go, decl: 2, sub: 0, line: 17 } |  |  | 0.552 |
| walker |  | 4289 | 33 | Code::CodeKey { rung: Decl, file: internal/core/dto/activity_dto.go, decl: 6, sub: 0, line: 46 } |  |  | 0.559 |
| walker |  | 4327 | 38 | Code::CodeKey { rung: Decl, file: internal/core/dto/activity_dto.go, decl: 5, sub: 0, line: 40 } |  |  | 0.576 |
| walker |  | 4375 | 48 | Code::CodeKey { rung: Decl, file: internal/core/dto/activity_dto.go, decl: 1, sub: 0, line: 9 } |  |  | 0.590 |
| ns | 4404 |  | 234 | Execute, the context keys, and the accessor helpers | 3.11 |  | 0.568 |
| walker |  | 4430 | 55 | Code::CodeKey { rung: Decl, file: internal/core/dto/activity_dto.go, decl: 4, sub: 0, line: 32 } |  |  | 0.584 |
| walker |  | 4488 | 58 | Code::CodeKey { rung: Decl, file: internal/core/dto/activity_dto.go, decl: 3, sub: 0, line: 23 } |  |  | 0.609 |
| ns | 4548 |  | 144 | `current --format` template variables (Long help) | 3.12 |  | 0.600 |
| ns | 4713 |  | 165 | `ical` argument semantics and `remove` Long help | 3.13 |  | 0.593 |
| ns | 4835 |  | 122 | Shell-completion machinery | 3.14 |  | 0.587 |
| walker |  | 4896 | 408 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.587 |
| ns | 4926 |  | 91 | activity.service struct and constructor | 4.1 |  | 0.580 |
| walker |  | 5152 | 256 | Markdown::Section { file: README.md, section_index: 23, keeps_default_concavity: false } |  |  | 0.580 |
| ns | 5193 |  | 267 | service method roster (bodies elided) | 4.2 |  | 0.565 |
| walker |  | 5313 | 161 | Markdown::Section { file: README.md, section_index: 24, keeps_default_concavity: false } |  |  | 0.565 |
| walker |  | 5340 | 27 | Code::CodeKey { rung: Names, file: internal/adapters/cli/root.go, decl: 0, sub: 0, line: 0 } |  |  | 0.565 |
| ns | 5427 |  | 234 | Service.Start — implicit stop of running activities | 4.3 | 4.2 | 0.549 |
| walker |  | 5466 | 126 | Plaintext::DeclSurface { file: demo/demo_script.sh } |  |  | 0.549 |
| walker |  | 5541 | 75 | Code::CodeKey { rung: Names, file: internal/core/errors/errors.go, decl: 0, sub: 0, line: 0 } |  |  | 0.559 |
| walker |  | 5544 | 3 | Code::CodeKey { rung: Decl, file: internal/core/errors/errors.go, decl: 1, sub: 0, line: 5 } |  |  | 0.561 |
| walker |  | 5623 | 79 | Code::CodeKey { rung: Names, file: internal/core/models/activity.go, decl: 0, sub: 0, line: 0 } |  |  | 0.566 |
| ns | 5646 |  | 219 | Service.Stop — target selection and validation | 4.4 | 4.2 | 0.550 |
| walker |  | 5721 | 98 | Code::CodeKey { rung: Decl, file: internal/core/models/activity.go, decl: 1, sub: 0, line: 10 } |  |  | 0.564 |
| walker |  | 5733 | 12 | Code::CodeKey { rung: Doc, file: internal/core/models/activity.go, decl: 1, sub: 0, line: 10 } |  |  | 0.568 |
| walker |  | 5750 | 17 | Code::CodeKey { rung: Names, file: internal/adapters/cli/current.go, decl: 0, sub: 0, line: 0 } |  |  | 0.568 |
| walker |  | 5767 | 17 | Code::CodeKey { rung: Names, file: internal/adapters/cli/calendar.go, decl: 0, sub: 0, line: 0 } |  |  | 0.568 |
| ns | 5885 |  | 239 | GetRecent de-duplication and note enrichment | 4.5 | 4.2 | 0.554 |
| walker |  | 5888 | 121 | Code::CodeKey { rung: Names, file: internal/adapters/cli/theme.go, decl: 0, sub: 0, line: 0 } |  |  | 0.554 |
| walker |  | 5975 | 87 | Code::CodeKey { rung: Decl, file: internal/adapters/cli/theme.go, decl: 1, sub: 0, line: 11 } |  |  | 0.555 |
| ns | 6089 |  | 204 | Bartib log line format: layouts and FormatActivity | 5.1 |  | 0.545 |
| walker |  | 6148 | 173 | Code::CodeKey { rung: Decl, file: internal/adapters/cli/theme.go, decl: 2, sub: 0, line: 22 } |  |  | 0.545 |
| walker |  | 6162 | 14 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/theme.go, decl: 1, sub: 0, line: 11 } |  |  | 0.545 |
| walker |  | 6178 | 16 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/theme.go, decl: 2, sub: 0, line: 22 } |  |  | 0.545 |
| walker |  | 6194 | 16 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/theme.go, decl: 3, sub: 0, line: 40 } |  |  | 0.545 |
| walker |  | 6210 | 16 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/theme.go, decl: 9, sub: 0, line: 167 } |  |  | 0.545 |
| ns | 6227 |  | 138 | ParseActivity — the reading half of the format | 5.2 | 5.1 | 0.537 |
| walker |  | 6256 | 46 | Code::CodeKey { rung: Names, file: internal/adapters/cli/update.go, decl: 0, sub: 0, line: 0 } |  |  | 0.537 |
| walker |  | 6287 | 31 | Code::CodeKey { rung: Decl, file: internal/adapters/cli/update.go, decl: 1, sub: 0, line: 17 } |  |  | 0.537 |
| walker |  | 6305 | 18 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/theme.go, decl: 4, sub: 0, line: 53 } |  |  | 0.537 |
| walker |  | 6322 | 17 | Code::CodeKey { rung: Names, file: internal/adapters/cli/version.go, decl: 0, sub: 0, line: 0 } |  |  | 0.537 |
| walker |  | 6339 | 17 | Code::CodeKey { rung: Names, file: internal/adapters/cli/add.go, decl: 0, sub: 0, line: 0 } |  |  | 0.537 |
| walker |  | 6356 | 17 | Code::CodeKey { rung: Names, file: internal/adapters/cli/list_gui.go, decl: 0, sub: 0, line: 0 } |  |  | 0.537 |
| walker |  | 6373 | 17 | Code::CodeKey { rung: Names, file: internal/adapters/cli/start.go, decl: 0, sub: 0, line: 0 } |  |  | 0.537 |
| walker |  | 6395 | 22 | Code::CodeKey { rung: Decl, file: internal/adapters/cli/start.go, decl: 1, sub: 0, line: 16 } |  |  | 0.537 |
| walker |  | 6423 | 28 | Code::CodeKey { rung: Names, file: internal/adapters/cli/analyze.go, decl: 0, sub: 0, line: 0 } |  |  | 0.537 |
| ns | 6430 |  | 203 | file repository: struct, constructor and complete method roster | 5.3 |  | 0.529 |
| walker |  | 6578 | 155 | Code::CodeKey { rung: Decl, file: internal/adapters/cli/analyze.go, decl: 2, sub: 0, line: 70 } |  |  | 0.530 |
| walker |  | 6595 | 17 | Code::CodeKey { rung: Names, file: internal/adapters/cli/watch.go, decl: 0, sub: 0, line: 0 } |  |  | 0.530 |
| walker |  | 6612 | 17 | Code::CodeKey { rung: Names, file: internal/adapters/cli/report.go, decl: 0, sub: 0, line: 0 } |  |  | 0.530 |
| walker |  | 6631 | 19 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/theme.go, decl: 8, sub: 0, line: 122 } |  |  | 0.530 |
| ns | 6635 |  | 205 | file repository Save — activity identity is minute-precision StartTime | 5.4 | 5.3 | 0.523 |
| walker |  | 6650 | 19 | Code::CodeKey { rung: Doc, file: internal/core/models/activity.go, decl: 4, sub: 0, line: 31 } |  |  | 0.526 |
| walker |  | 6668 | 18 | Code::CodeKey { rung: Names, file: internal/adapters/cli/ical.go, decl: 0, sub: 0, line: 0 } |  |  | 0.526 |
| walker |  | 6722 | 54 | Code::CodeKey { rung: Names, file: internal/services/ics/generator.go, decl: 0, sub: 0, line: 0 } |  |  | 0.526 |
| walker |  | 6739 | 17 | Code::CodeKey { rung: Doc, file: internal/services/ics/generator.go, decl: 1, sub: 0, line: 12 } |  |  | 0.526 |
| walker |  | 6756 | 17 | Code::CodeKey { rung: Doc, file: internal/services/ics/generator.go, decl: 3, sub: 0, line: 29 } |  |  | 0.526 |
| walker |  | 6776 | 20 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/theme.go, decl: 5, sub: 0, line: 66 } |  |  | 0.526 |
| walker |  | 6796 | 20 | Code::CodeKey { rung: Doc, file: internal/services/ics/generator.go, decl: 2, sub: 0, line: 18 } |  |  | 0.526 |
| walker |  | 6817 | 21 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/theme.go, decl: 6, sub: 0, line: 79 } |  |  | 0.526 |
| ns | 6874 |  | 239 | notes repository: sidecar file layout and YAML front matter | 5.5 |  | 0.516 |
| ns | 7045 |  | 171 | timewarrior repository: interval shape and data layout | 5.6 |  | 0.506 |
| ns | 7231 |  | 186 | timewarrior repository function roster (names only) | 5.7 |  | 0.500 |
| walker |  | 7329 | 512 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.500 |
| ns | 7465 |  | 234 | iCalendar generator | 5.8 |  | 0.494 |
| ns | 7647 |  | 182 | config.Config — the top-level settings struct | 6.1 |  | 0.504 |
| walker |  | 7803 | 474 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.504 |
| ns | 8013 |  | 366 | All nested config structs (complete) | 6.2 |  | 0.524 |
| ns | 8127 |  | 114 | Config defaults | 6.3 |  | 0.520 |
| walker |  | 8130 | 327 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.520 |
| walker |  | 8166 | 36 | Code::CodeKey { rung: Doc, file: internal/timeutil/timeutil.go, decl: 8, sub: 0, line: 56 } |  |  | 0.520 |
| ns | 8452 |  | 325 | Complete TOCK_* environment variable bindings | 6.4 |  | 0.514 |
| walker |  | 8602 | 436 | Markdown::Section { file: README.md, section_index: 12, keeps_default_concavity: false } |  |  | 0.514 |
| ns | 8700 |  | 248 | timeutil.Formatter — 12/24-hour formats and method roster | 6.5 |  | 0.520 |
| ns | 8888 |  | 188 | Theme type and the complete theme constructor roster | 6.6 |  | 0.531 |
| walker |  | 9038 | 436 | Markdown::Section { file: README.md, section_index: 13, keeps_default_concavity: false } |  |  | 0.531 |
| ns | 9046 |  | 158 | GetTheme — accepted theme names and auto-detection | 6.7 | 6.6 | 0.523 |
| walker |  | 9051 | 13 | Code::CodeKey { rung: Body, file: internal/core/models/activity.go, decl: 2, sub: 0, line: 19 } |  |  | 0.525 |
| ns | 9216 |  | 170 | Type roster of the cli package (names only) | 7.1 |  | 0.519 |
| ns | 9371 |  | 155 | AnalysisStats — everything `tock analyze` computes | 7.2 | 7.1 | 0.526 |
| walker |  | 9441 | 390 | Markdown::Section { file: README.md, section_index: 21, keeps_default_concavity: false } |  |  | 0.526 |
| ns | 9517 |  | 146 | Calendar TUI key bindings (complete case list) | 7.3 |  | 0.518 |
| ns | 9754 |  | 237 | Direct dependency set | 8.1 |  | 0.525 |
| ns | 9805 |  | 51 | Peripheral directory listings (docs, demo, assets, .github) | 8.2 |  | 0.531 |
| walker |  | 9904 | 463 | Markdown::Section { file: README.md, section_index: 14, keeps_default_concavity: false } |  |  | 0.531 |
| walker |  | 9944 | 40 | Code::CodeKey { rung: Doc, file: internal/timeutil/timeutil.go, decl: 4, sub: 0, line: 25 } |  |  | 0.531 |
| ns | 9961 |  | 156 | Build, release and mock-generation entry points | 8.3 |  | 0.527 |
| walker |  | 9982 | 38 | Code::CodeKey { rung: Names, file: internal/extra/extra.go, decl: 0, sub: 0, line: 0 } |  |  | 0.527 |
| walker |  | 9990 | 8 | Markdown::Section { file: README.md, section_index: 15, keeps_default_concavity: false } |  |  | 0.527 |

Score(3000)=0.549 I=0.819 C=0.368 ns_rows≤3K=25/59 grid(1000/1442/2080/3000/4327/6240/9000)=0.732/0.731/0.632/0.549/0.518/0.540/0.534

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
| walker |  | 657 | 18 | Fs::DirListing { dir: internal/core/ports/mocks } |  |  | 0.561 |
| ns | 724 |  | 105 | README section headings (all H2) | 1.10 |  | 0.528 |
| walker |  | 762 | 105 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.587 |
| ns | 898 |  | 174 | ports.ActivityResolver — the full service contract | 2.1 |  | 0.558 |
| walker |  | 1015 | 253 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.745 |
| ns | 1022 |  | 124 | models.Activity struct with JSON tags | 2.2 |  | 0.715 |
| ns | 1175 |  | 153 | ports.ActivityRepository and ports.NotesRepository | 2.3 |  | 0.681 |
| ns | 1253 |  | 78 | Domain sentinel errors | 2.4 |  | 0.665 |
| walker |  | 1257 | 242 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.665 |
| walker |  | 1336 | 79 | Fs::DirListing { dir: internal/adapters/cli } |  |  | 0.794 |
| walker |  | 1415 | 79 | Markdown::Section { file: README.md, section_index: 19, keeps_default_concavity: false } |  |  | 0.794 |
| ns | 1440 |  | 187 | dto request types for Start / Stop / Add | 2.5 |  | 0.731 |
| ns | 1506 |  | 66 | dto.ActivityFilter — the query type | 2.6 |  | 0.714 |
| ns | 1601 |  | 95 | dto.Report and dto.ProjectReport | 2.7 |  | 0.690 |
| walker |  | 1657 | 242 | GoMod::File { file: go.mod } |  |  | 0.691 |
| ns | 1685 |  | 84 | models.Activity method roster (bodies elided) | 2.8 |  | 0.672 |
| walker |  | 1738 | 81 | Markdown::Section { file: README.md, section_index: 25, keeps_default_concavity: false } |  |  | 0.672 |
| ns | 1832 |  | 147 | models.Activity method bodies | 2.9 | 2.8 | 0.640 |
| walker |  | 1853 | 115 | Markdown::Section { file: README.md, section_index: 20, keeps_default_concavity: false } |  |  | 0.640 |
| walker |  | 1965 | 112 | Plaintext::DeclSurface { file: install.sh } |  |  | 0.640 |
| walker |  | 1977 | 12 | Code::CodeKey { rung: Names, file: cmd/tock/main.go, decl: 0, sub: 0, line: 0 } |  |  | 0.641 |
| walker |  | 1988 | 11 | Code::CodeKey { rung: Decl, file: cmd/tock/main.go, decl: 1, sub: 0, line: 7 } |  |  | 0.644 |
| ns | 2000 |  | 168 | Root command declaration and the three persistent flags | 3.1 |  | 0.632 |
| ns | 2162 |  | 162 | Complete subcommand registration list | 3.2 |  | 0.609 |
| walker |  | 2211 | 223 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.609 |
| ns | 2398 |  | 236 | Command declarations, part 1: add, analyze, calendar, continue, current, ical | 3.3 |  | 0.588 |
| walker |  | 2494 | 283 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.588 |
| ns | 2733 |  | 335 | Command declarations, part 2: last, list, remove, report, start, stop, version, watch | 3.4 |  | 0.561 |
| walker |  | 2761 | 267 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.561 |
| ns | 2764 |  | 31 | docs/commands.md structure | 3.5 |  | 0.558 |
| walker |  | 2929 | 168 | Code::CodeKey { rung: Names, file: internal/config/config.go, decl: 0, sub: 0, line: 0 } |  |  | 0.558 |
| walker |  | 2944 | 15 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 5, sub: 0, line: 41 } |  |  | 0.558 |
| ns | 2956 |  | 192 | Flags of `start` and `stop` | 3.6 |  | 0.549 |
| walker |  | 2961 | 17 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 4, sub: 0, line: 37 } |  |  | 0.549 |
| walker |  | 2978 | 17 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 6, sub: 0, line: 45 } |  |  | 0.549 |
| walker |  | 2997 | 19 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 3, sub: 0, line: 33 } |  |  | 0.549 |
| walker |  | 3087 | 90 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 2, sub: 0, line: 25 } |  |  | 0.550 |
| walker |  | 3201 | 114 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 7, sub: 0, line: 49 } |  |  | 0.551 |
| ns | 3280 |  | 324 | Flags of `add` and `continue` | 3.7 |  | 0.538 |
| walker |  | 3369 | 168 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 1, sub: 0, line: 12 } |  |  | 0.539 |
| ns | 3583 |  | 303 | Flags of `report`, `last` and `analyze` | 3.8 |  | 0.528 |
| walker |  | 3634 | 265 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.528 |
| ns | 3775 |  | 192 | Flags of `current`, `watch`, `ical` and `remove` | 3.9 |  | 0.521 |
| walker |  | 3832 | 198 | Code::CodeKey { rung: Names, file: internal/timeutil/timeutil.go, decl: 0, sub: 0, line: 0 } |  |  | 0.521 |
| walker |  | 3837 | 5 | Code::CodeKey { rung: Decl, file: internal/timeutil/timeutil.go, decl: 2, sub: 0, line: 13 } |  |  | 0.521 |
| walker |  | 3850 | 13 | Code::CodeKey { rung: Decl, file: internal/timeutil/timeutil.go, decl: 3, sub: 0, line: 19 } |  |  | 0.521 |
| walker |  | 3860 | 10 | Code::CodeKey { rung: Doc, file: internal/timeutil/timeutil.go, decl: 3, sub: 0, line: 19 } |  |  | 0.522 |
| walker |  | 3874 | 14 | Code::CodeKey { rung: Doc, file: internal/timeutil/timeutil.go, decl: 1, sub: 0, line: 11 } |  |  | 0.522 |
| walker |  | 3882 | 8 | Code::CodeKey { rung: Body, file: internal/timeutil/timeutil.go, decl: 5, sub: 0, line: 34 } |  |  | 0.522 |
| walker |  | 3897 | 15 | Code::CodeKey { rung: Doc, file: internal/timeutil/timeutil.go, decl: 5, sub: 0, line: 34 } |  |  | 0.522 |
| walker |  | 3913 | 16 | Code::CodeKey { rung: Doc, file: internal/timeutil/timeutil.go, decl: 7, sub: 0, line: 47 } |  |  | 0.522 |
| walker |  | 3930 | 17 | Code::CodeKey { rung: Doc, file: internal/timeutil/timeutil.go, decl: 6, sub: 0, line: 39 } |  |  | 0.522 |
| walker |  | 4161 | 231 | Markdown::Section { file: README.md, section_index: 22, keeps_default_concavity: false } |  |  | 0.522 |
| ns | 4170 |  | 395 | PersistentPreRunE — dependency injection and backend selection | 3.10 |  | 0.491 |
| walker |  | 4196 | 35 | Code::CodeKey { rung: Names, file: internal/core/ports/ports.go, decl: 0, sub: 0, line: 0 } |  |  | 0.492 |
| walker |  | 4251 | 55 | Code::CodeKey { rung: Decl, file: internal/core/ports/ports.go, decl: 3, sub: 0, line: 29 } |  |  | 0.498 |
| walker |  | 4324 | 73 | Code::CodeKey { rung: Decl, file: internal/core/ports/ports.go, decl: 2, sub: 0, line: 22 } |  |  | 0.518 |
| ns | 4404 |  | 234 | Execute, the context keys, and the accessor helpers | 3.11 |  | 0.499 |
| walker |  | 4483 | 159 | Code::CodeKey { rung: Decl, file: internal/core/ports/ports.go, decl: 1, sub: 0, line: 11 } |  |  | 0.524 |
| ns | 4548 |  | 144 | `current --format` template variables (Long help) | 3.12 |  | 0.516 |
| walker |  | 4553 | 70 | Code::CodeKey { rung: Names, file: internal/core/dto/activity_dto.go, decl: 0, sub: 0, line: 0 } |  |  | 0.520 |
| walker |  | 4584 | 31 | Code::CodeKey { rung: Decl, file: internal/core/dto/activity_dto.go, decl: 2, sub: 0, line: 17 } |  |  | 0.524 |
| walker |  | 4617 | 33 | Code::CodeKey { rung: Decl, file: internal/core/dto/activity_dto.go, decl: 6, sub: 0, line: 46 } |  |  | 0.531 |
| walker |  | 4655 | 38 | Code::CodeKey { rung: Decl, file: internal/core/dto/activity_dto.go, decl: 5, sub: 0, line: 40 } |  |  | 0.546 |
| walker |  | 4703 | 48 | Code::CodeKey { rung: Decl, file: internal/core/dto/activity_dto.go, decl: 1, sub: 0, line: 9 } |  |  | 0.560 |
| ns | 4713 |  | 165 | `ical` argument semantics and `remove` Long help | 3.13 |  | 0.553 |
| walker |  | 4758 | 55 | Code::CodeKey { rung: Decl, file: internal/core/dto/activity_dto.go, decl: 4, sub: 0, line: 32 } |  |  | 0.568 |
| walker |  | 4816 | 58 | Code::CodeKey { rung: Decl, file: internal/core/dto/activity_dto.go, decl: 3, sub: 0, line: 23 } |  |  | 0.593 |
| ns | 4835 |  | 122 | Shell-completion machinery | 3.14 |  | 0.587 |
| ns | 4926 |  | 91 | activity.service struct and constructor | 4.1 |  | 0.580 |
| ns | 5193 |  | 267 | service method roster (bodies elided) | 4.2 |  | 0.565 |
| walker |  | 5224 | 408 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.565 |
| ns | 5427 |  | 234 | Service.Start — implicit stop of running activities | 4.3 | 4.2 | 0.549 |
| walker |  | 5480 | 256 | Markdown::Section { file: README.md, section_index: 23, keeps_default_concavity: false } |  |  | 0.549 |
| walker |  | 5641 | 161 | Markdown::Section { file: README.md, section_index: 24, keeps_default_concavity: false } |  |  | 0.549 |
| ns | 5646 |  | 219 | Service.Stop — target selection and validation | 4.4 | 4.2 | 0.533 |
| walker |  | 5668 | 27 | Code::CodeKey { rung: Names, file: internal/adapters/cli/root.go, decl: 0, sub: 0, line: 0 } |  |  | 0.533 |
| walker |  | 5743 | 75 | Code::CodeKey { rung: Names, file: internal/core/errors/errors.go, decl: 0, sub: 0, line: 0 } |  |  | 0.543 |
| walker |  | 5746 | 3 | Code::CodeKey { rung: Decl, file: internal/core/errors/errors.go, decl: 1, sub: 0, line: 5 } |  |  | 0.545 |
| walker |  | 5825 | 79 | Code::CodeKey { rung: Names, file: internal/core/models/activity.go, decl: 0, sub: 0, line: 0 } |  |  | 0.550 |
| ns | 5885 |  | 239 | GetRecent de-duplication and note enrichment | 4.5 | 4.2 | 0.537 |
| walker |  | 5923 | 98 | Code::CodeKey { rung: Decl, file: internal/core/models/activity.go, decl: 1, sub: 0, line: 10 } |  |  | 0.550 |
| walker |  | 5935 | 12 | Code::CodeKey { rung: Doc, file: internal/core/models/activity.go, decl: 1, sub: 0, line: 10 } |  |  | 0.554 |
| walker |  | 5954 | 19 | Code::CodeKey { rung: Doc, file: internal/core/models/activity.go, decl: 4, sub: 0, line: 31 } |  |  | 0.558 |
| ns | 6089 |  | 204 | Bartib log line format: layouts and FormatActivity | 5.1 |  | 0.548 |
| walker |  | 6094 | 140 | Plaintext::DeclSurface { file: demo/demo_script.sh } |  |  | 0.548 |
| walker |  | 6130 | 36 | Code::CodeKey { rung: Doc, file: internal/timeutil/timeutil.go, decl: 8, sub: 0, line: 56 } |  |  | 0.548 |
| walker |  | 6147 | 17 | Code::CodeKey { rung: Names, file: internal/adapters/cli/current.go, decl: 0, sub: 0, line: 0 } |  |  | 0.548 |
| walker |  | 6164 | 17 | Code::CodeKey { rung: Names, file: internal/adapters/cli/calendar.go, decl: 0, sub: 0, line: 0 } |  |  | 0.548 |
| ns | 6227 |  | 138 | ParseActivity — the reading half of the format | 5.2 | 5.1 | 0.540 |
| walker |  | 6285 | 121 | Code::CodeKey { rung: Names, file: internal/adapters/cli/theme.go, decl: 0, sub: 0, line: 0 } |  |  | 0.540 |
| walker |  | 6372 | 87 | Code::CodeKey { rung: Decl, file: internal/adapters/cli/theme.go, decl: 1, sub: 0, line: 11 } |  |  | 0.540 |
| ns | 6430 |  | 203 | file repository: struct, constructor and complete method roster | 5.3 |  | 0.533 |
| walker |  | 6545 | 173 | Code::CodeKey { rung: Decl, file: internal/adapters/cli/theme.go, decl: 2, sub: 0, line: 22 } |  |  | 0.533 |
| walker |  | 6559 | 14 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/theme.go, decl: 1, sub: 0, line: 11 } |  |  | 0.533 |
| walker |  | 6575 | 16 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/theme.go, decl: 2, sub: 0, line: 22 } |  |  | 0.533 |
| walker |  | 6591 | 16 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/theme.go, decl: 3, sub: 0, line: 40 } |  |  | 0.533 |
| walker |  | 6607 | 16 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/theme.go, decl: 9, sub: 0, line: 167 } |  |  | 0.533 |
| walker |  | 6625 | 18 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/theme.go, decl: 4, sub: 0, line: 53 } |  |  | 0.533 |
| ns | 6635 |  | 205 | file repository Save — activity identity is minute-precision StartTime | 5.4 | 5.3 | 0.526 |
| walker |  | 6644 | 19 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/theme.go, decl: 8, sub: 0, line: 122 } |  |  | 0.526 |
| walker |  | 6690 | 46 | Code::CodeKey { rung: Names, file: internal/adapters/cli/update.go, decl: 0, sub: 0, line: 0 } |  |  | 0.526 |
| walker |  | 6721 | 31 | Code::CodeKey { rung: Decl, file: internal/adapters/cli/update.go, decl: 1, sub: 0, line: 17 } |  |  | 0.526 |
| walker |  | 6741 | 20 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/theme.go, decl: 5, sub: 0, line: 66 } |  |  | 0.526 |
| walker |  | 6762 | 21 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/theme.go, decl: 6, sub: 0, line: 79 } |  |  | 0.526 |
| walker |  | 6779 | 17 | Code::CodeKey { rung: Names, file: internal/adapters/cli/version.go, decl: 0, sub: 0, line: 0 } |  |  | 0.526 |
| walker |  | 6796 | 17 | Code::CodeKey { rung: Names, file: internal/adapters/cli/add.go, decl: 0, sub: 0, line: 0 } |  |  | 0.526 |
| walker |  | 6813 | 17 | Code::CodeKey { rung: Names, file: internal/adapters/cli/list_gui.go, decl: 0, sub: 0, line: 0 } |  |  | 0.526 |
| walker |  | 6830 | 17 | Code::CodeKey { rung: Names, file: internal/adapters/cli/start.go, decl: 0, sub: 0, line: 0 } |  |  | 0.526 |
| walker |  | 6852 | 22 | Code::CodeKey { rung: Decl, file: internal/adapters/cli/start.go, decl: 1, sub: 0, line: 16 } |  |  | 0.526 |
| ns | 6874 |  | 239 | notes repository: sidecar file layout and YAML front matter | 5.5 |  | 0.515 |
| walker |  | 6880 | 28 | Code::CodeKey { rung: Names, file: internal/adapters/cli/analyze.go, decl: 0, sub: 0, line: 0 } |  |  | 0.515 |
| walker |  | 7035 | 155 | Code::CodeKey { rung: Decl, file: internal/adapters/cli/analyze.go, decl: 2, sub: 0, line: 70 } |  |  | 0.516 |
| ns | 7045 |  | 171 | timewarrior repository: interval shape and data layout | 5.6 |  | 0.506 |
| walker |  | 7052 | 17 | Code::CodeKey { rung: Names, file: internal/adapters/cli/watch.go, decl: 0, sub: 0, line: 0 } |  |  | 0.506 |
| walker |  | 7069 | 17 | Code::CodeKey { rung: Names, file: internal/adapters/cli/report.go, decl: 0, sub: 0, line: 0 } |  |  | 0.506 |
| walker |  | 7082 | 13 | Code::CodeKey { rung: Body, file: internal/core/models/activity.go, decl: 2, sub: 0, line: 19 } |  |  | 0.508 |
| walker |  | 7100 | 18 | Code::CodeKey { rung: Names, file: internal/adapters/cli/ical.go, decl: 0, sub: 0, line: 0 } |  |  | 0.508 |
| walker |  | 7154 | 54 | Code::CodeKey { rung: Names, file: internal/services/ics/generator.go, decl: 0, sub: 0, line: 0 } |  |  | 0.508 |
| walker |  | 7171 | 17 | Code::CodeKey { rung: Doc, file: internal/services/ics/generator.go, decl: 1, sub: 0, line: 12 } |  |  | 0.508 |
| walker |  | 7188 | 17 | Code::CodeKey { rung: Doc, file: internal/services/ics/generator.go, decl: 3, sub: 0, line: 29 } |  |  | 0.508 |
| walker |  | 7208 | 20 | Code::CodeKey { rung: Doc, file: internal/services/ics/generator.go, decl: 2, sub: 0, line: 18 } |  |  | 0.508 |
| ns | 7231 |  | 186 | timewarrior repository function roster (names only) | 5.7 |  | 0.502 |
| walker |  | 7248 | 40 | Code::CodeKey { rung: Doc, file: internal/timeutil/timeutil.go, decl: 4, sub: 0, line: 25 } |  |  | 0.502 |
| ns | 7465 |  | 234 | iCalendar generator | 5.8 |  | 0.496 |
| ns | 7647 |  | 182 | config.Config — the top-level settings struct | 6.1 |  | 0.506 |
| walker |  | 7760 | 512 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.506 |
| ns | 8013 |  | 366 | All nested config structs (complete) | 6.2 |  | 0.526 |
| ns | 8127 |  | 114 | Config defaults | 6.3 |  | 0.522 |
| walker |  | 8234 | 474 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.522 |
| ns | 8452 |  | 325 | Complete TOCK_* environment variable bindings | 6.4 |  | 0.515 |
| walker |  | 8561 | 327 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.515 |
| ns | 8700 |  | 248 | timeutil.Formatter — 12/24-hour formats and method roster | 6.5 |  | 0.523 |
| ns | 8888 |  | 188 | Theme type and the complete theme constructor roster | 6.6 |  | 0.534 |
| walker |  | 8997 | 436 | Markdown::Section { file: README.md, section_index: 12, keeps_default_concavity: false } |  |  | 0.534 |
| ns | 9046 |  | 158 | GetTheme — accepted theme names and auto-detection | 6.7 | 6.6 | 0.526 |
| ns | 9216 |  | 170 | Type roster of the cli package (names only) | 7.1 |  | 0.521 |
| ns | 9371 |  | 155 | AnalysisStats — everything `tock analyze` computes | 7.2 | 7.1 | 0.527 |
| walker |  | 9433 | 436 | Markdown::Section { file: README.md, section_index: 13, keeps_default_concavity: false } |  |  | 0.527 |
| walker |  | 9477 | 44 | Code::CodeKey { rung: Doc, file: internal/timeutil/timeutil.go, decl: 9, sub: 0, line: 103 } |  |  | 0.527 |
| ns | 9517 |  | 146 | Calendar TUI key bindings (complete case list) | 7.3 |  | 0.520 |
| ns | 9754 |  | 237 | Direct dependency set | 8.1 |  | 0.527 |
| ns | 9805 |  | 51 | Peripheral directory listings (docs, demo, assets, .github) | 8.2 |  | 0.533 |
| walker |  | 9867 | 390 | Markdown::Section { file: README.md, section_index: 21, keeps_default_concavity: false } |  |  | 0.533 |
| ns | 9961 |  | 156 | Build, release and mock-generation entry points | 8.3 |  | 0.529 |
| walker |  | 10000 | 133 | Markdown::Section { file: README.md, section_index: 14, keeps_default_concavity: false } |  |  | 0.529 |

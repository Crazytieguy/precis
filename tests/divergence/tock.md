Score(3000)=0.524 I=0.807 C=0.340 ns_rows≤3K=25/59 grid(1000/1442/2080/3000/4327/6240/9000)=0.822/0.685/0.600/0.524/0.498/0.491/0.548

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
| walker |  | 186 | 33 | GoMod::Identity { file: go.mod } |  |  | 0.778 |
| walker |  | 197 | 11 | Fs::DirListing { dir: internal/timeutil } |  |  | 0.781 |
| walker |  | 209 | 12 | Fs::DirListing { dir: internal/core } |  |  | 0.804 |
| walker |  | 213 | 4 | Fs::DirListing { dir: internal/core/errors } |  |  | 0.806 |
| walker |  | 217 | 4 | Fs::DirListing { dir: internal/core/models } |  |  | 0.728 |
| ns | 217 |  | 83 | README feature bullets, part 1 (storage, notes, TUI, footprint) | 1.4 |  | 0.728 |
| walker |  | 223 | 6 | Fs::DirListing { dir: internal/core/dto } |  |  | 0.730 |
| walker |  | 230 | 7 | Fs::DirListing { dir: internal/core/ports } |  |  | 0.736 |
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
| ns | 1255 |  | 78 | Domain sentinel errors | 2.4 |  | 0.743 |
| walker |  | 1281 | 279 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.743 |
| ns | 1442 |  | 187 | dto request types for Start / Stop / Add | 2.5 |  | 0.685 |
| ns | 1508 |  | 66 | dto.ActivityFilter — the query type | 2.6 |  | 0.669 |
| walker |  | 1523 | 242 | GoMod::File { file: go.mod } |  |  | 0.670 |
| ns | 1603 |  | 95 | dto.Report and dto.ProjectReport | 2.7 |  | 0.646 |
| walker |  | 1614 | 91 | Markdown::Section { file: README.md, section_index: 19, keeps_default_concavity: false } |  |  | 0.647 |
| ns | 1687 |  | 84 | models.Activity method roster (bodies elided) | 2.8 |  | 0.630 |
| walker |  | 1706 | 92 | Markdown::Section { file: README.md, section_index: 25, keeps_default_concavity: false } |  |  | 0.632 |
| walker |  | 1831 | 125 | Markdown::Section { file: README.md, section_index: 20, keeps_default_concavity: false } |  |  | 0.634 |
| ns | 1834 |  | 147 | models.Activity method bodies | 2.9 | 2.8 | 0.604 |
| walker |  | 1943 | 112 | Plaintext::DeclSurface { file: install.sh } |  |  | 0.604 |
| walker |  | 1955 | 12 | Code::CodeKey { rung: Names, file: cmd/tock/main.go, decl: 0, sub: 0, line: 0 } |  |  | 0.605 |
| walker |  | 1966 | 11 | Code::CodeKey { rung: Decl, file: cmd/tock/main.go, decl: 1, sub: 0, line: 7 } |  |  | 0.608 |
| ns | 2002 |  | 168 | Root command declaration and the three persistent flags | 3.1 |  | 0.597 |
| ns | 2164 |  | 162 | Complete subcommand registration list | 3.2 |  | 0.575 |
| walker |  | 2257 | 291 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.577 |
| ns | 2400 |  | 236 | Command declarations, part 1: add, analyze, calendar, continue, current, ical | 3.3 |  | 0.558 |
| walker |  | 2524 | 267 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.558 |
| ns | 2735 |  | 335 | Command declarations, part 2: last, list, remove, report, start, stop, version, watch | 3.4 |  | 0.533 |
| walker |  | 2757 | 233 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.536 |
| ns | 2766 |  | 31 | docs/commands.md structure | 3.5 |  | 0.532 |
| walker |  | 2925 | 168 | Code::CodeKey { rung: Names, file: internal/config/config.go, decl: 0, sub: 0, line: 0 } |  |  | 0.532 |
| walker |  | 2940 | 15 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 5, sub: 0, line: 41 } |  |  | 0.532 |
| walker |  | 2957 | 17 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 4, sub: 0, line: 37 } |  |  | 0.532 |
| ns | 2958 |  | 192 | Flags of `start` and `stop` | 3.6 |  | 0.524 |
| walker |  | 2974 | 17 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 6, sub: 0, line: 45 } |  |  | 0.524 |
| walker |  | 2993 | 19 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 3, sub: 0, line: 33 } |  |  | 0.524 |
| walker |  | 3083 | 90 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 2, sub: 0, line: 25 } |  |  | 0.524 |
| walker |  | 3197 | 114 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 7, sub: 0, line: 49 } |  |  | 0.526 |
| ns | 3282 |  | 324 | Flags of `add` and `continue` | 3.7 |  | 0.513 |
| walker |  | 3365 | 168 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 1, sub: 0, line: 12 } |  |  | 0.514 |
| ns | 3585 |  | 303 | Flags of `report`, `last` and `analyze` | 3.8 |  | 0.503 |
| walker |  | 3630 | 265 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.503 |
| ns | 3777 |  | 192 | Flags of `current`, `watch`, `ical` and `remove` | 3.9 |  | 0.497 |
| walker |  | 3819 | 189 | Code::CodeKey { rung: Names, file: internal/timeutil/timeutil.go, decl: 0, sub: 0, line: 0 } |  |  | 0.497 |
| walker |  | 3828 | 9 | Code::CodeKey { rung: Decl, file: internal/timeutil/timeutil.go, decl: 2, sub: 0, line: 13 } |  |  | 0.497 |
| walker |  | 3841 | 13 | Code::CodeKey { rung: Decl, file: internal/timeutil/timeutil.go, decl: 3, sub: 0, line: 19 } |  |  | 0.497 |
| walker |  | 3849 | 8 | Code::CodeKey { rung: Body, file: internal/timeutil/timeutil.go, decl: 5, sub: 0, line: 34 } |  |  | 0.497 |
| walker |  | 3859 | 10 | Code::CodeKey { rung: Doc, file: internal/timeutil/timeutil.go, decl: 3, sub: 0, line: 19 } |  |  | 0.498 |
| walker |  | 3873 | 14 | Code::CodeKey { rung: Doc, file: internal/timeutil/timeutil.go, decl: 1, sub: 0, line: 11 } |  |  | 0.498 |
| walker |  | 3888 | 15 | Code::CodeKey { rung: Doc, file: internal/timeutil/timeutil.go, decl: 5, sub: 0, line: 34 } |  |  | 0.498 |
| walker |  | 3904 | 16 | Code::CodeKey { rung: Doc, file: internal/timeutil/timeutil.go, decl: 7, sub: 0, line: 47 } |  |  | 0.498 |
| walker |  | 3921 | 17 | Code::CodeKey { rung: Doc, file: internal/timeutil/timeutil.go, decl: 6, sub: 0, line: 39 } |  |  | 0.498 |
| walker |  | 3957 | 36 | Code::CodeKey { rung: Doc, file: internal/timeutil/timeutil.go, decl: 8, sub: 0, line: 56 } |  |  | 0.498 |
| walker |  | 3997 | 40 | Code::CodeKey { rung: Doc, file: internal/timeutil/timeutil.go, decl: 4, sub: 0, line: 25 } |  |  | 0.498 |
| walker |  | 4041 | 44 | Code::CodeKey { rung: Doc, file: internal/timeutil/timeutil.go, decl: 9, sub: 0, line: 103 } |  |  | 0.498 |
| walker |  | 4113 | 72 | Code::CodeKey { rung: Doc, file: internal/timeutil/timeutil.go, decl: 10, sub: 0, line: 158 } |  |  | 0.498 |
| walker |  | 4148 | 35 | Code::CodeKey { rung: Names, file: internal/core/ports/ports.go, decl: 0, sub: 0, line: 0 } |  |  | 0.499 |
| ns | 4172 |  | 395 | PersistentPreRunE — dependency injection and backend selection | 3.10 |  | 0.469 |
| walker |  | 4203 | 55 | Code::CodeKey { rung: Decl, file: internal/core/ports/ports.go, decl: 3, sub: 0, line: 29 } |  |  | 0.475 |
| walker |  | 4276 | 73 | Code::CodeKey { rung: Decl, file: internal/core/ports/ports.go, decl: 2, sub: 0, line: 22 } |  |  | 0.495 |
| ns | 4406 |  | 234 | Execute, the context keys, and the accessor helpers | 3.11 |  | 0.477 |
| walker |  | 4435 | 159 | Code::CodeKey { rung: Decl, file: internal/core/ports/ports.go, decl: 1, sub: 0, line: 11 } |  |  | 0.503 |
| walker |  | 4505 | 70 | Code::CodeKey { rung: Names, file: internal/core/dto/activity_dto.go, decl: 0, sub: 0, line: 0 } |  |  | 0.507 |
| walker |  | 4536 | 31 | Code::CodeKey { rung: Decl, file: internal/core/dto/activity_dto.go, decl: 2, sub: 0, line: 17 } |  |  | 0.512 |
| ns | 4550 |  | 144 | `current --format` template variables (Long help) | 3.12 |  | 0.504 |
| walker |  | 4569 | 33 | Code::CodeKey { rung: Decl, file: internal/core/dto/activity_dto.go, decl: 6, sub: 0, line: 46 } |  |  | 0.511 |
| walker |  | 4607 | 38 | Code::CodeKey { rung: Decl, file: internal/core/dto/activity_dto.go, decl: 5, sub: 0, line: 40 } |  |  | 0.526 |
| walker |  | 4655 | 48 | Code::CodeKey { rung: Decl, file: internal/core/dto/activity_dto.go, decl: 1, sub: 0, line: 9 } |  |  | 0.540 |
| walker |  | 4710 | 55 | Code::CodeKey { rung: Decl, file: internal/core/dto/activity_dto.go, decl: 4, sub: 0, line: 32 } |  |  | 0.556 |
| ns | 4715 |  | 165 | `ical` argument semantics and `remove` Long help | 3.13 |  | 0.549 |
| walker |  | 4768 | 58 | Code::CodeKey { rung: Decl, file: internal/core/dto/activity_dto.go, decl: 3, sub: 0, line: 23 } |  |  | 0.574 |
| ns | 4837 |  | 122 | Shell-completion machinery | 3.14 |  | 0.568 |
| ns | 4928 |  | 91 | activity.service struct and constructor | 4.1 |  | 0.562 |
| walker |  | 5009 | 241 | Markdown::Section { file: README.md, section_index: 22, keeps_default_concavity: false } |  |  | 0.565 |
| walker |  | 5058 | 49 | Code::CodeKey { rung: Names, file: internal/adapters/cli/version.go, decl: 0, sub: 0, line: 0 } |  |  | 0.565 |
| walker |  | 5067 | 9 | Code::CodeKey { rung: Decl, file: internal/adapters/cli/version.go, decl: 1, sub: 0, line: 10 } |  |  | 0.565 |
| ns | 5195 |  | 267 | service method roster (bodies elided) | 4.2 |  | 0.550 |
| ns | 5429 |  | 234 | Service.Start — implicit stop of running activities | 4.3 | 4.2 | 0.534 |
| walker |  | 5477 | 410 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.534 |
| ns | 5648 |  | 219 | Service.Stop — target selection and validation | 4.4 | 4.2 | 0.519 |
| walker |  | 5733 | 256 | Markdown::Section { file: README.md, section_index: 23, keeps_default_concavity: false } |  |  | 0.519 |
| ns | 5887 |  | 239 | GetRecent de-duplication and note enrichment | 4.5 | 4.2 | 0.506 |
| walker |  | 5894 | 161 | Markdown::Section { file: README.md, section_index: 24, keeps_default_concavity: false } |  |  | 0.506 |
| walker |  | 6015 | 121 | Code::CodeKey { rung: Names, file: internal/adapters/cli/theme.go, decl: 0, sub: 0, line: 0 } |  |  | 0.506 |
| ns | 6091 |  | 204 | Bartib log line format: layouts and FormatActivity | 5.1 |  | 0.498 |
| walker |  | 6102 | 87 | Code::CodeKey { rung: Decl, file: internal/adapters/cli/theme.go, decl: 1, sub: 0, line: 11 } |  |  | 0.498 |
| walker |  | 6116 | 14 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/theme.go, decl: 1, sub: 0, line: 11 } |  |  | 0.498 |
| walker |  | 6132 | 16 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/theme.go, decl: 9, sub: 0, line: 167 } |  |  | 0.498 |
| walker |  | 6150 | 18 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/theme.go, decl: 3, sub: 0, line: 40 } |  |  | 0.498 |
| walker |  | 6168 | 18 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/theme.go, decl: 4, sub: 0, line: 53 } |  |  | 0.498 |
| walker |  | 6187 | 19 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/theme.go, decl: 8, sub: 0, line: 122 } |  |  | 0.498 |
| walker |  | 6207 | 20 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/theme.go, decl: 5, sub: 0, line: 66 } |  |  | 0.498 |
| walker |  | 6228 | 21 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/theme.go, decl: 6, sub: 0, line: 79 } |  |  | 0.498 |
| ns | 6229 |  | 138 | ParseActivity — the reading half of the format | 5.2 | 5.1 | 0.491 |
| walker |  | 6399 | 171 | Code::CodeKey { rung: Decl, file: internal/adapters/cli/theme.go, decl: 2, sub: 0, line: 22 } |  |  | 0.491 |
| walker |  | 6415 | 16 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/theme.go, decl: 2, sub: 0, line: 22 } |  |  | 0.491 |
| ns | 6432 |  | 203 | file repository: struct, constructor and complete method roster | 5.3 |  | 0.484 |
| walker |  | 6444 | 29 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/theme.go, decl: 7, sub: 0, line: 93 } |  |  | 0.484 |
| walker |  | 6475 | 31 | Code::CodeKey { rung: Body, file: internal/config/config.go, decl: 9, sub: 0, line: 62 } |  |  | 0.484 |
| walker |  | 6521 | 46 | Code::CodeKey { rung: Names, file: internal/adapters/cli/update.go, decl: 0, sub: 0, line: 0 } |  |  | 0.484 |
| walker |  | 6552 | 31 | Code::CodeKey { rung: Decl, file: internal/adapters/cli/update.go, decl: 1, sub: 0, line: 17 } |  |  | 0.484 |
| walker |  | 6631 | 79 | Code::CodeKey { rung: Names, file: internal/core/models/activity.go, decl: 0, sub: 0, line: 0 } |  |  | 0.489 |
| ns | 6637 |  | 205 | file repository Save — activity identity is minute-precision StartTime | 5.4 | 5.3 | 0.482 |
| walker |  | 6644 | 13 | Code::CodeKey { rung: Body, file: internal/core/models/activity.go, decl: 2, sub: 0, line: 19 } |  |  | 0.484 |
| walker |  | 6742 | 98 | Code::CodeKey { rung: Decl, file: internal/core/models/activity.go, decl: 1, sub: 0, line: 10 } |  |  | 0.497 |
| walker |  | 6754 | 12 | Code::CodeKey { rung: Doc, file: internal/core/models/activity.go, decl: 1, sub: 0, line: 10 } |  |  | 0.500 |
| walker |  | 6773 | 19 | Code::CodeKey { rung: Doc, file: internal/core/models/activity.go, decl: 4, sub: 0, line: 31 } |  |  | 0.504 |
| walker |  | 6816 | 43 | Code::CodeKey { rung: Body, file: internal/core/models/activity.go, decl: 3, sub: 0, line: 23 } |  |  | 0.510 |
| walker |  | 6848 | 32 | Code::CodeKey { rung: Names, file: internal/adapters/cli/continue.go, decl: 0, sub: 0, line: 0 } |  |  | 0.510 |
| walker |  | 6859 | 11 | Code::CodeKey { rung: Decl, file: internal/adapters/cli/continue.go, decl: 1, sub: 0, line: 15 } |  |  | 0.510 |
| walker |  | 6875 | 16 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/continue.go, decl: 2, sub: 0, line: 20 } |  |  | 0.510 |
| ns | 6876 |  | 239 | notes repository: sidecar file layout and YAML front matter | 5.5 |  | 0.499 |
| walker |  | 7015 | 140 | Plaintext::DeclSurface { file: demo/demo_script.sh } |  |  | 0.499 |
| ns | 7047 |  | 171 | timewarrior repository: interval shape and data layout | 5.6 |  | 0.490 |
| walker |  | 7081 | 66 | Code::CodeKey { rung: Names, file: internal/adapters/cli/current.go, decl: 0, sub: 0, line: 0 } |  |  | 0.490 |
| walker |  | 7092 | 11 | Code::CodeKey { rung: Decl, file: internal/adapters/cli/current.go, decl: 1, sub: 0, line: 19 } |  |  | 0.490 |
| walker |  | 7105 | 13 | Code::CodeKey { rung: Body, file: internal/adapters/cli/current.go, decl: 2, sub: 0, line: 23 } |  |  | 0.490 |
| walker |  | 7155 | 50 | Code::CodeKey { rung: Names, file: internal/adapters/cli/last.go, decl: 0, sub: 0, line: 0 } |  |  | 0.490 |
| walker |  | 7176 | 21 | Code::CodeKey { rung: Decl, file: internal/adapters/cli/last.go, decl: 1, sub: 0, line: 16 } |  |  | 0.490 |
| walker |  | 7226 | 50 | Code::CodeKey { rung: Names, file: internal/adapters/cli/report.go, decl: 0, sub: 0, line: 0 } |  |  | 0.490 |
| ns | 7233 |  | 186 | timewarrior repository function roster (names only) | 5.7 |  | 0.484 |
| walker |  | 7300 | 74 | Code::CodeKey { rung: Decl, file: internal/adapters/cli/report.go, decl: 1, sub: 0, line: 20 } |  |  | 0.484 |
| walker |  | 7318 | 18 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/report.go, decl: 3, sub: 0, line: 61 } |  |  | 0.484 |
| walker |  | 7335 | 17 | Code::CodeKey { rung: Names, file: internal/adapters/cli/start.go, decl: 0, sub: 0, line: 0 } |  |  | 0.484 |
| walker |  | 7357 | 22 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/start.go, decl: 1, sub: 0, line: 17 } |  |  | 0.484 |
| walker |  | 7374 | 17 | Code::CodeKey { rung: Names, file: internal/adapters/cli/stop.go, decl: 0, sub: 0, line: 0 } |  |  | 0.484 |
| ns | 7467 |  | 234 | iCalendar generator | 5.8 |  | 0.475 |
| walker |  | 7544 | 170 | Code::CodeKey { rung: Names, file: internal/adapters/cli/watch.go, decl: 0, sub: 0, line: 0 } |  |  | 0.475 |
| walker |  | 7566 | 22 | Code::CodeKey { rung: Decl, file: internal/adapters/cli/watch.go, decl: 4, sub: 0, line: 90 } |  |  | 0.475 |
| ns | 7649 |  | 182 | config.Config — the top-level settings struct | 6.1 |  | 0.486 |
| walker |  | 7658 | 92 | Code::CodeKey { rung: Decl, file: internal/adapters/cli/watch.go, decl: 3, sub: 0, line: 77 } |  |  | 0.486 |
| walker |  | 7692 | 34 | Code::CodeKey { rung: Body, file: internal/adapters/cli/watch.go, decl: 6, sub: 0, line: 115 } |  |  | 0.486 |
| walker |  | 7965 | 273 | Code::CodeKey { rung: Decl, file: internal/adapters/cli/watch.go, decl: 9, sub: 0, line: 248 } |  |  | 0.486 |
| ns | 8015 |  | 366 | All nested config structs (complete) | 6.2 |  | 0.507 |
| walker |  | 8033 | 68 | Code::CodeKey { rung: Names, file: internal/services/ics/generator.go, decl: 0, sub: 0, line: 0 } |  |  | 0.509 |
| walker |  | 8050 | 17 | Code::CodeKey { rung: Doc, file: internal/services/ics/generator.go, decl: 1, sub: 0, line: 12 } |  |  | 0.510 |
| walker |  | 8067 | 17 | Code::CodeKey { rung: Doc, file: internal/services/ics/generator.go, decl: 3, sub: 0, line: 29 } |  |  | 0.510 |
| walker |  | 8087 | 20 | Code::CodeKey { rung: Doc, file: internal/services/ics/generator.go, decl: 2, sub: 0, line: 18 } |  |  | 0.511 |
| walker |  | 8109 | 22 | Code::CodeKey { rung: Body, file: internal/services/ics/generator.go, decl: 1, sub: 0, line: 12 } |  |  | 0.514 |
| ns | 8129 |  | 114 | Config defaults | 6.3 |  | 0.510 |
| walker |  | 8189 | 80 | Code::CodeKey { rung: Body, file: internal/adapters/cli/current.go, decl: 3, sub: 0, line: 27 } |  |  | 0.510 |
| walker |  | 8258 | 69 | Code::CodeKey { rung: Names, file: internal/core/errors/errors.go, decl: 0, sub: 0, line: 0 } |  |  | 0.516 |
| walker |  | 8267 | 9 | Code::CodeKey { rung: Decl, file: internal/core/errors/errors.go, decl: 1, sub: 0, line: 5 } |  |  | 0.519 |
| ns | 8454 |  | 325 | Complete TOCK_* environment variable bindings | 6.4 |  | 0.512 |
| walker |  | 8536 | 269 | Code::CodeKey { rung: Names, file: internal/adapters/cli/root.go, decl: 0, sub: 0, line: 0 } |  |  | 0.521 |
| walker |  | 8545 | 9 | Code::CodeKey { rung: Decl, file: internal/adapters/cli/root.go, decl: 1, sub: 0, line: 21 } |  |  | 0.521 |
| walker |  | 8572 | 27 | Code::CodeKey { rung: Body, file: internal/adapters/cli/root.go, decl: 9, sub: 0, line: 119 } |  |  | 0.522 |
| walker |  | 8600 | 28 | Code::CodeKey { rung: Body, file: internal/adapters/cli/root.go, decl: 8, sub: 0, line: 115 } |  |  | 0.523 |
| walker |  | 8630 | 30 | Code::CodeKey { rung: Body, file: internal/adapters/cli/root.go, decl: 10, sub: 0, line: 123 } |  |  | 0.524 |
| walker |  | 8673 | 43 | Code::CodeKey { rung: Body, file: internal/adapters/cli/root.go, decl: 11, sub: 0, line: 127 } |  |  | 0.531 |
| ns | 8702 |  | 248 | timeutil.Formatter — 12/24-hour formats and method roster | 6.5 |  | 0.537 |
| walker |  | 8745 | 72 | Code::CodeKey { rung: Names, file: internal/adapters/cli/analyze.go, decl: 0, sub: 0, line: 0 } |  |  | 0.537 |
| walker |  | 8762 | 17 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/analyze.go, decl: 3, sub: 0, line: 84 } |  |  | 0.537 |
| walker |  | 8782 | 20 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/analyze.go, decl: 4, sub: 0, line: 206 } |  |  | 0.537 |
| ns | 8890 |  | 188 | Theme type and the complete theme constructor roster | 6.6 |  | 0.547 |
| walker |  | 8935 | 153 | Code::CodeKey { rung: Decl, file: internal/adapters/cli/analyze.go, decl: 2, sub: 0, line: 70 } |  |  | 0.548 |
| walker |  | 9046 | 111 | Code::CodeKey { rung: Names, file: internal/adapters/cli/calendar_sidebar.go, decl: 0, sub: 0, line: 0 } |  |  | 0.548 |
| ns | 9048 |  | 158 | GetTheme — accepted theme names and auto-detection | 6.7 | 6.6 | 0.540 |
| walker |  | 9065 | 19 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/calendar_sidebar.go, decl: 2, sub: 0, line: 15 } |  |  | 0.540 |
| ns | 9218 |  | 170 | Type roster of the cli package (names only) | 7.1 |  | 0.538 |
| walker |  | 9275 | 210 | Code::CodeKey { rung: Names, file: internal/adapters/cli/list_gui.go, decl: 0, sub: 0, line: 0 } |  |  | 0.539 |
| walker |  | 9282 | 7 | Code::CodeKey { rung: Body, file: internal/adapters/cli/list_gui.go, decl: 9, sub: 0, line: 200 } |  |  | 0.539 |
| ns | 9373 |  | 155 | AnalysisStats — everything `tock analyze` computes | 7.2 | 7.1 | 0.545 |
| walker |  | 9380 | 98 | Code::CodeKey { rung: Decl, file: internal/adapters/cli/list_gui.go, decl: 2, sub: 0, line: 45 } |  |  | 0.545 |
| walker |  | 9455 | 75 | Code::CodeKey { rung: Body, file: internal/adapters/cli/list_gui.go, decl: 5, sub: 0, line: 101 } |  |  | 0.545 |
| ns | 9519 |  | 146 | Calendar TUI key bindings (complete case list) | 7.3 |  | 0.537 |
| walker |  | 9543 | 88 | Code::CodeKey { rung: Body, file: internal/adapters/cli/list_gui.go, decl: 3, sub: 0, line: 57 } |  |  | 0.537 |
| walker |  | 9574 | 31 | Code::CodeKey { rung: Body, file: internal/config/config.go, decl: 10, sub: 0, line: 68 } |  |  | 0.537 |
| ns | 9756 |  | 237 | Direct dependency set | 8.1 |  | 0.544 |
| ns | 9807 |  | 51 | Peripheral directory listings (docs, demo, assets, .github) | 8.2 |  | 0.550 |
| ns | 9963 |  | 156 | Build, release and mock-generation entry points | 8.3 |  | 0.545 |
| walker |  | 9997 | 423 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.547 |

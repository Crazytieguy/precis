Score(3000)=0.531 I=0.815 C=0.346 ns_rows≤3K=25/59 grid(1000/1442/2080/3000/4327/6240/9000)=0.822/0.687/0.599/0.531/0.499/0.494/0.525

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
| walker |  | 1027 | 25 | Markdown::Section { file: README.md, section_index: 26, keeps_default_concavity: false } |  |  | 0.799 |
| ns | 1177 |  | 153 | ports.ActivityRepository and ports.NotesRepository | 2.3 |  | 0.762 |
| ns | 1255 |  | 78 | Domain sentinel errors | 2.4 |  | 0.744 |
| walker |  | 1269 | 242 | GoMod::File { file: go.mod } |  |  | 0.745 |
| ns | 1442 |  | 187 | dto request types for Start / Stop / Add | 2.5 |  | 0.687 |
| ns | 1508 |  | 66 | dto.ActivityFilter — the query type | 2.6 |  | 0.670 |
| walker |  | 1548 | 279 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.670 |
| ns | 1603 |  | 95 | dto.Report and dto.ProjectReport | 2.7 |  | 0.647 |
| ns | 1687 |  | 84 | models.Activity method roster (bodies elided) | 2.8 |  | 0.630 |
| walker |  | 1736 | 188 | Markdown::HeadingsOutline { file: docs/commands.md } |  |  | 0.630 |
| walker |  | 1827 | 91 | Markdown::Section { file: README.md, section_index: 19, keeps_default_concavity: false } |  |  | 0.632 |
| ns | 1834 |  | 147 | models.Activity method bodies | 2.9 | 2.8 | 0.602 |
| walker |  | 1917 | 90 | Markdown::Section { file: README.md, section_index: 25, keeps_default_concavity: false } |  |  | 0.605 |
| walker |  | 1929 | 12 | Code::CodeKey { rung: Names, file: cmd/tock/main.go, decl: 0, sub: 0, line: 0 } |  |  | 0.605 |
| walker |  | 1937 | 8 | Code::CodeKey { rung: Body, file: cmd/tock/main.go, decl: 1, sub: 0, line: 7 } |  |  | 0.607 |
| ns | 2002 |  | 168 | Root command declaration and the three persistent flags | 3.1 |  | 0.596 |
| walker |  | 2062 | 125 | Markdown::Section { file: README.md, section_index: 20, keeps_default_concavity: false } |  |  | 0.599 |
| ns | 2164 |  | 162 | Complete subcommand registration list | 3.2 |  | 0.576 |
| walker |  | 2230 | 168 | Code::CodeKey { rung: Names, file: internal/config/config.go, decl: 0, sub: 0, line: 0 } |  |  | 0.576 |
| walker |  | 2245 | 15 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 5, sub: 0, line: 41 } |  |  | 0.576 |
| walker |  | 2262 | 17 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 4, sub: 0, line: 37 } |  |  | 0.576 |
| walker |  | 2279 | 17 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 6, sub: 0, line: 45 } |  |  | 0.577 |
| walker |  | 2298 | 19 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 3, sub: 0, line: 33 } |  |  | 0.577 |
| walker |  | 2388 | 90 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 2, sub: 0, line: 25 } |  |  | 0.577 |
| ns | 2400 |  | 236 | Command declarations, part 1: add, analyze, calendar, continue, current, ical | 3.3 |  | 0.558 |
| walker |  | 2502 | 114 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 7, sub: 0, line: 49 } |  |  | 0.560 |
| walker |  | 2670 | 168 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 1, sub: 0, line: 12 } |  |  | 0.561 |
| ns | 2735 |  | 335 | Command declarations, part 2: last, list, remove, report, start, stop, version, watch | 3.4 |  | 0.535 |
| ns | 2766 |  | 31 | docs/commands.md structure | 3.5 |  | 0.540 |
| ns | 2958 |  | 192 | Flags of `start` and `stop` | 3.6 |  | 0.531 |
| ns | 3282 |  | 324 | Flags of `add` and `continue` | 3.7 |  | 0.518 |
| ns | 3585 |  | 303 | Flags of `report`, `last` and `analyze` | 3.8 |  | 0.508 |
| walker |  | 3672 | 1002 | Plaintext::Whole { file: install.sh } |  |  | 0.508 |
| ns | 3777 |  | 192 | Flags of `current`, `watch`, `ical` and `remove` | 3.9 |  | 0.501 |
| walker |  | 3861 | 189 | Code::CodeKey { rung: Names, file: internal/timeutil/timeutil.go, decl: 0, sub: 0, line: 0 } |  |  | 0.501 |
| walker |  | 3870 | 9 | Code::CodeKey { rung: Decl, file: internal/timeutil/timeutil.go, decl: 2, sub: 0, line: 13 } |  |  | 0.501 |
| walker |  | 3883 | 13 | Code::CodeKey { rung: Decl, file: internal/timeutil/timeutil.go, decl: 3, sub: 0, line: 19 } |  |  | 0.502 |
| walker |  | 3891 | 8 | Code::CodeKey { rung: Body, file: internal/timeutil/timeutil.go, decl: 5, sub: 0, line: 34 } |  |  | 0.502 |
| walker |  | 3901 | 10 | Code::CodeKey { rung: Doc, file: internal/timeutil/timeutil.go, decl: 3, sub: 0, line: 19 } |  |  | 0.502 |
| walker |  | 3915 | 14 | Code::CodeKey { rung: Doc, file: internal/timeutil/timeutil.go, decl: 1, sub: 0, line: 11 } |  |  | 0.502 |
| walker |  | 3930 | 15 | Code::CodeKey { rung: Doc, file: internal/timeutil/timeutil.go, decl: 5, sub: 0, line: 34 } |  |  | 0.502 |
| walker |  | 3946 | 16 | Code::CodeKey { rung: Doc, file: internal/timeutil/timeutil.go, decl: 7, sub: 0, line: 47 } |  |  | 0.502 |
| walker |  | 3963 | 17 | Code::CodeKey { rung: Doc, file: internal/timeutil/timeutil.go, decl: 6, sub: 0, line: 39 } |  |  | 0.502 |
| walker |  | 3999 | 36 | Code::CodeKey { rung: Doc, file: internal/timeutil/timeutil.go, decl: 8, sub: 0, line: 56 } |  |  | 0.502 |
| walker |  | 4039 | 40 | Code::CodeKey { rung: Doc, file: internal/timeutil/timeutil.go, decl: 4, sub: 0, line: 25 } |  |  | 0.502 |
| walker |  | 4083 | 44 | Code::CodeKey { rung: Doc, file: internal/timeutil/timeutil.go, decl: 9, sub: 0, line: 103 } |  |  | 0.502 |
| walker |  | 4155 | 72 | Code::CodeKey { rung: Doc, file: internal/timeutil/timeutil.go, decl: 10, sub: 0, line: 158 } |  |  | 0.502 |
| ns | 4172 |  | 395 | PersistentPreRunE — dependency injection and backend selection | 3.10 |  | 0.472 |
| walker |  | 4190 | 35 | Code::CodeKey { rung: Names, file: internal/core/ports/ports.go, decl: 0, sub: 0, line: 0 } |  |  | 0.473 |
| walker |  | 4245 | 55 | Code::CodeKey { rung: Decl, file: internal/core/ports/ports.go, decl: 3, sub: 0, line: 29 } |  |  | 0.479 |
| walker |  | 4318 | 73 | Code::CodeKey { rung: Decl, file: internal/core/ports/ports.go, decl: 2, sub: 0, line: 22 } |  |  | 0.499 |
| ns | 4406 |  | 234 | Execute, the context keys, and the accessor helpers | 3.11 |  | 0.481 |
| walker |  | 4477 | 159 | Code::CodeKey { rung: Decl, file: internal/core/ports/ports.go, decl: 1, sub: 0, line: 11 } |  |  | 0.507 |
| walker |  | 4547 | 70 | Code::CodeKey { rung: Names, file: internal/core/dto/activity_dto.go, decl: 0, sub: 0, line: 0 } |  |  | 0.510 |
| ns | 4550 |  | 144 | `current --format` template variables (Long help) | 3.12 |  | 0.503 |
| walker |  | 4578 | 31 | Code::CodeKey { rung: Decl, file: internal/core/dto/activity_dto.go, decl: 2, sub: 0, line: 17 } |  |  | 0.507 |
| walker |  | 4611 | 33 | Code::CodeKey { rung: Decl, file: internal/core/dto/activity_dto.go, decl: 6, sub: 0, line: 46 } |  |  | 0.514 |
| walker |  | 4649 | 38 | Code::CodeKey { rung: Decl, file: internal/core/dto/activity_dto.go, decl: 5, sub: 0, line: 40 } |  |  | 0.530 |
| walker |  | 4697 | 48 | Code::CodeKey { rung: Decl, file: internal/core/dto/activity_dto.go, decl: 1, sub: 0, line: 9 } |  |  | 0.543 |
| ns | 4715 |  | 165 | `ical` argument semantics and `remove` Long help | 3.13 |  | 0.537 |
| walker |  | 4752 | 55 | Code::CodeKey { rung: Decl, file: internal/core/dto/activity_dto.go, decl: 4, sub: 0, line: 32 } |  |  | 0.552 |
| walker |  | 4810 | 58 | Code::CodeKey { rung: Decl, file: internal/core/dto/activity_dto.go, decl: 3, sub: 0, line: 23 } |  |  | 0.577 |
| ns | 4837 |  | 122 | Shell-completion machinery | 3.14 |  | 0.571 |
| ns | 4928 |  | 91 | activity.service struct and constructor | 4.1 |  | 0.564 |
| walker |  | 5101 | 291 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.567 |
| ns | 5195 |  | 267 | service method roster (bodies elided) | 4.2 |  | 0.552 |
| walker |  | 5368 | 267 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.552 |
| ns | 5429 |  | 234 | Service.Start — implicit stop of running activities | 4.3 | 4.2 | 0.536 |
| walker |  | 5601 | 233 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.538 |
| ns | 5648 |  | 219 | Service.Stop — target selection and validation | 4.4 | 4.2 | 0.523 |
| walker |  | 5722 | 121 | Code::CodeKey { rung: Names, file: internal/adapters/cli/theme.go, decl: 0, sub: 0, line: 0 } |  |  | 0.523 |
| walker |  | 5809 | 87 | Code::CodeKey { rung: Decl, file: internal/adapters/cli/theme.go, decl: 1, sub: 0, line: 11 } |  |  | 0.524 |
| walker |  | 5823 | 14 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/theme.go, decl: 1, sub: 0, line: 11 } |  |  | 0.524 |
| walker |  | 5839 | 16 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/theme.go, decl: 9, sub: 0, line: 167 } |  |  | 0.524 |
| walker |  | 5857 | 18 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/theme.go, decl: 3, sub: 0, line: 40 } |  |  | 0.524 |
| walker |  | 5875 | 18 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/theme.go, decl: 4, sub: 0, line: 53 } |  |  | 0.524 |
| ns | 5887 |  | 239 | GetRecent de-duplication and note enrichment | 4.5 | 4.2 | 0.511 |
| walker |  | 5894 | 19 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/theme.go, decl: 8, sub: 0, line: 122 } |  |  | 0.511 |
| walker |  | 5914 | 20 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/theme.go, decl: 5, sub: 0, line: 66 } |  |  | 0.511 |
| walker |  | 5935 | 21 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/theme.go, decl: 6, sub: 0, line: 79 } |  |  | 0.511 |
| ns | 6091 |  | 204 | Bartib log line format: layouts and FormatActivity | 5.1 |  | 0.502 |
| walker |  | 6106 | 171 | Code::CodeKey { rung: Decl, file: internal/adapters/cli/theme.go, decl: 2, sub: 0, line: 22 } |  |  | 0.502 |
| walker |  | 6122 | 16 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/theme.go, decl: 2, sub: 0, line: 22 } |  |  | 0.502 |
| walker |  | 6151 | 29 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/theme.go, decl: 7, sub: 0, line: 93 } |  |  | 0.502 |
| walker |  | 6182 | 31 | Code::CodeKey { rung: Body, file: internal/config/config.go, decl: 9, sub: 0, line: 62 } |  |  | 0.502 |
| walker |  | 6228 | 46 | Code::CodeKey { rung: Names, file: internal/adapters/cli/update.go, decl: 0, sub: 0, line: 0 } |  |  | 0.502 |
| ns | 6229 |  | 138 | ParseActivity — the reading half of the format | 5.2 | 5.1 | 0.494 |
| walker |  | 6259 | 31 | Code::CodeKey { rung: Decl, file: internal/adapters/cli/update.go, decl: 1, sub: 0, line: 17 } |  |  | 0.494 |
| walker |  | 6338 | 79 | Code::CodeKey { rung: Names, file: internal/core/models/activity.go, decl: 0, sub: 0, line: 0 } |  |  | 0.499 |
| walker |  | 6351 | 13 | Code::CodeKey { rung: Body, file: internal/core/models/activity.go, decl: 2, sub: 0, line: 19 } |  |  | 0.501 |
| ns | 6432 |  | 203 | file repository: struct, constructor and complete method roster | 5.3 |  | 0.494 |
| walker |  | 6449 | 98 | Code::CodeKey { rung: Decl, file: internal/core/models/activity.go, decl: 1, sub: 0, line: 10 } |  |  | 0.507 |
| walker |  | 6461 | 12 | Code::CodeKey { rung: Doc, file: internal/core/models/activity.go, decl: 1, sub: 0, line: 10 } |  |  | 0.511 |
| walker |  | 6480 | 19 | Code::CodeKey { rung: Doc, file: internal/core/models/activity.go, decl: 4, sub: 0, line: 31 } |  |  | 0.515 |
| walker |  | 6523 | 43 | Code::CodeKey { rung: Body, file: internal/core/models/activity.go, decl: 3, sub: 0, line: 23 } |  |  | 0.520 |
| ns | 6637 |  | 205 | file repository Save — activity identity is minute-precision StartTime | 5.4 | 5.3 | 0.513 |
| walker |  | 6788 | 265 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.513 |
| walker |  | 6820 | 32 | Code::CodeKey { rung: Names, file: internal/adapters/cli/continue.go, decl: 0, sub: 0, line: 0 } |  |  | 0.513 |
| walker |  | 6831 | 11 | Code::CodeKey { rung: Decl, file: internal/adapters/cli/continue.go, decl: 1, sub: 0, line: 15 } |  |  | 0.513 |
| walker |  | 6847 | 16 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/continue.go, decl: 2, sub: 0, line: 20 } |  |  | 0.513 |
| ns | 6876 |  | 239 | notes repository: sidecar file layout and YAML front matter | 5.5 |  | 0.503 |
| walker |  | 6987 | 140 | Plaintext::DeclSurface { file: demo/demo_script.sh } |  |  | 0.503 |
| ns | 7047 |  | 171 | timewarrior repository: interval shape and data layout | 5.6 |  | 0.494 |
| walker |  | 7053 | 66 | Code::CodeKey { rung: Names, file: internal/adapters/cli/current.go, decl: 0, sub: 0, line: 0 } |  |  | 0.494 |
| walker |  | 7064 | 11 | Code::CodeKey { rung: Decl, file: internal/adapters/cli/current.go, decl: 1, sub: 0, line: 19 } |  |  | 0.494 |
| walker |  | 7077 | 13 | Code::CodeKey { rung: Body, file: internal/adapters/cli/current.go, decl: 2, sub: 0, line: 23 } |  |  | 0.494 |
| walker |  | 7127 | 50 | Code::CodeKey { rung: Names, file: internal/adapters/cli/last.go, decl: 0, sub: 0, line: 0 } |  |  | 0.494 |
| walker |  | 7148 | 21 | Code::CodeKey { rung: Decl, file: internal/adapters/cli/last.go, decl: 1, sub: 0, line: 16 } |  |  | 0.494 |
| walker |  | 7198 | 50 | Code::CodeKey { rung: Names, file: internal/adapters/cli/report.go, decl: 0, sub: 0, line: 0 } |  |  | 0.494 |
| ns | 7233 |  | 186 | timewarrior repository function roster (names only) | 5.7 |  | 0.487 |
| walker |  | 7272 | 74 | Code::CodeKey { rung: Decl, file: internal/adapters/cli/report.go, decl: 1, sub: 0, line: 20 } |  |  | 0.487 |
| walker |  | 7290 | 18 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/report.go, decl: 3, sub: 0, line: 61 } |  |  | 0.487 |
| walker |  | 7307 | 17 | Code::CodeKey { rung: Names, file: internal/adapters/cli/start.go, decl: 0, sub: 0, line: 0 } |  |  | 0.487 |
| walker |  | 7329 | 22 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/start.go, decl: 1, sub: 0, line: 17 } |  |  | 0.487 |
| walker |  | 7346 | 17 | Code::CodeKey { rung: Names, file: internal/adapters/cli/stop.go, decl: 0, sub: 0, line: 0 } |  |  | 0.487 |
| ns | 7467 |  | 234 | iCalendar generator | 5.8 |  | 0.478 |
| walker |  | 7516 | 170 | Code::CodeKey { rung: Names, file: internal/adapters/cli/watch.go, decl: 0, sub: 0, line: 0 } |  |  | 0.478 |
| walker |  | 7538 | 22 | Code::CodeKey { rung: Decl, file: internal/adapters/cli/watch.go, decl: 4, sub: 0, line: 90 } |  |  | 0.478 |
| walker |  | 7630 | 92 | Code::CodeKey { rung: Decl, file: internal/adapters/cli/watch.go, decl: 3, sub: 0, line: 77 } |  |  | 0.478 |
| ns | 7649 |  | 182 | config.Config — the top-level settings struct | 6.1 |  | 0.489 |
| walker |  | 7664 | 34 | Code::CodeKey { rung: Body, file: internal/adapters/cli/watch.go, decl: 6, sub: 0, line: 115 } |  |  | 0.489 |
| walker |  | 7937 | 273 | Code::CodeKey { rung: Decl, file: internal/adapters/cli/watch.go, decl: 9, sub: 0, line: 248 } |  |  | 0.489 |
| walker |  | 8005 | 68 | Code::CodeKey { rung: Names, file: internal/services/ics/generator.go, decl: 0, sub: 0, line: 0 } |  |  | 0.491 |
| ns | 8015 |  | 366 | All nested config structs (complete) | 6.2 |  | 0.512 |
| walker |  | 8022 | 17 | Code::CodeKey { rung: Doc, file: internal/services/ics/generator.go, decl: 1, sub: 0, line: 12 } |  |  | 0.513 |
| walker |  | 8039 | 17 | Code::CodeKey { rung: Doc, file: internal/services/ics/generator.go, decl: 3, sub: 0, line: 29 } |  |  | 0.513 |
| walker |  | 8059 | 20 | Code::CodeKey { rung: Doc, file: internal/services/ics/generator.go, decl: 2, sub: 0, line: 18 } |  |  | 0.514 |
| walker |  | 8081 | 22 | Code::CodeKey { rung: Body, file: internal/services/ics/generator.go, decl: 1, sub: 0, line: 12 } |  |  | 0.516 |
| ns | 8129 |  | 114 | Config defaults | 6.3 |  | 0.512 |
| walker |  | 8161 | 80 | Code::CodeKey { rung: Body, file: internal/adapters/cli/current.go, decl: 3, sub: 0, line: 27 } |  |  | 0.512 |
| walker |  | 8233 | 72 | Code::CodeKey { rung: Names, file: internal/adapters/cli/analyze.go, decl: 0, sub: 0, line: 0 } |  |  | 0.512 |
| walker |  | 8250 | 17 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/analyze.go, decl: 3, sub: 0, line: 84 } |  |  | 0.512 |
| walker |  | 8270 | 20 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/analyze.go, decl: 4, sub: 0, line: 206 } |  |  | 0.512 |
| walker |  | 8423 | 153 | Code::CodeKey { rung: Decl, file: internal/adapters/cli/analyze.go, decl: 2, sub: 0, line: 70 } |  |  | 0.513 |
| ns | 8454 |  | 325 | Complete TOCK_* environment variable bindings | 6.4 |  | 0.507 |
| walker |  | 8689 | 266 | Markdown::Section { file: docs/commands.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.507 |
| ns | 8702 |  | 248 | timeutil.Formatter — 12/24-hour formats and method roster | 6.5 |  | 0.514 |
| walker |  | 8800 | 111 | Code::CodeKey { rung: Names, file: internal/adapters/cli/calendar_sidebar.go, decl: 0, sub: 0, line: 0 } |  |  | 0.514 |
| walker |  | 8819 | 19 | Code::CodeKey { rung: Doc, file: internal/adapters/cli/calendar_sidebar.go, decl: 2, sub: 0, line: 15 } |  |  | 0.514 |
| ns | 8890 |  | 188 | Theme type and the complete theme constructor roster | 6.6 |  | 0.525 |
| walker |  | 9029 | 210 | Code::CodeKey { rung: Names, file: internal/adapters/cli/list_gui.go, decl: 0, sub: 0, line: 0 } |  |  | 0.525 |
| walker |  | 9036 | 7 | Code::CodeKey { rung: Body, file: internal/adapters/cli/list_gui.go, decl: 9, sub: 0, line: 200 } |  |  | 0.525 |
| ns | 9048 |  | 158 | GetTheme — accepted theme names and auto-detection | 6.7 | 6.6 | 0.517 |
| walker |  | 9134 | 98 | Code::CodeKey { rung: Decl, file: internal/adapters/cli/list_gui.go, decl: 2, sub: 0, line: 45 } |  |  | 0.517 |
| walker |  | 9209 | 75 | Code::CodeKey { rung: Body, file: internal/adapters/cli/list_gui.go, decl: 5, sub: 0, line: 101 } |  |  | 0.517 |
| ns | 9218 |  | 170 | Type roster of the cli package (names only) | 7.1 |  | 0.517 |
| walker |  | 9297 | 88 | Code::CodeKey { rung: Body, file: internal/adapters/cli/list_gui.go, decl: 3, sub: 0, line: 57 } |  |  | 0.517 |
| ns | 9373 |  | 155 | AnalysisStats — everything `tock analyze` computes | 7.2 | 7.1 | 0.524 |
| ns | 9519 |  | 146 | Calendar TUI key bindings (complete case list) | 7.3 |  | 0.516 |
| walker |  | 9566 | 269 | Code::CodeKey { rung: Names, file: internal/adapters/cli/root.go, decl: 0, sub: 0, line: 0 } |  |  | 0.524 |
| walker |  | 9575 | 9 | Code::CodeKey { rung: Decl, file: internal/adapters/cli/root.go, decl: 1, sub: 0, line: 21 } |  |  | 0.524 |
| walker |  | 9602 | 27 | Code::CodeKey { rung: Body, file: internal/adapters/cli/root.go, decl: 9, sub: 0, line: 119 } |  |  | 0.525 |
| walker |  | 9630 | 28 | Code::CodeKey { rung: Body, file: internal/adapters/cli/root.go, decl: 8, sub: 0, line: 115 } |  |  | 0.525 |
| walker |  | 9660 | 30 | Code::CodeKey { rung: Body, file: internal/adapters/cli/root.go, decl: 10, sub: 0, line: 123 } |  |  | 0.526 |
| walker |  | 9703 | 43 | Code::CodeKey { rung: Body, file: internal/adapters/cli/root.go, decl: 11, sub: 0, line: 127 } |  |  | 0.532 |
| walker |  | 9734 | 31 | Code::CodeKey { rung: Body, file: internal/config/config.go, decl: 10, sub: 0, line: 68 } |  |  | 0.532 |
| ns | 9756 |  | 237 | Direct dependency set | 8.1 |  | 0.539 |
| ns | 9807 |  | 51 | Peripheral directory listings (docs, demo, assets, .github) | 8.2 |  | 0.544 |
| ns | 9963 |  | 156 | Build, release and mock-generation entry points | 8.3 |  | 0.540 |
| walker |  | 9975 | 241 | Markdown::Section { file: README.md, section_index: 22, keeps_default_concavity: false } |  |  | 0.542 |
| walker |  | 9992 | 17 | Code::CodeKey { rung: Names, file: internal/adapters/cli/calendar.go, decl: 0, sub: 0, line: 0 } |  |  | 0.542 |

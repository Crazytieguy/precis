scores: Sim=0.567 Reached=18/43 Early=2 Late=11 Partial=7 Missing=18 Used=9905/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 7 | 5 | 2 | 0 | 0.91 |
| 2 | 6 | 5 | 0 | 1 | 0.80 |
| 3 | 4 | 2 | 1 | 1 | 0.67 |
| 4 | 3 | 0 | 1 | 2 | 0.33 |
| 5 | 2 | 1 | 0 | 1 | 0.44 |
| 6 | 7 | 1 | 2 | 4 | 0.43 |
| 7 | 5 | 0 | 0 | 5 | 0.08 |
| 8 | 2 | 1 | 0 | 1 | 0.47 |
| 9 | 7 | 3 | 1 | 3 | 0.61 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.1 | 8 | 165 | +157 | 1.00 | late | README title | README headline in README.md (t=165, 1 atoms) |
| 1.3 | 134 | — | — | 0.60 | partial | README feature lede (first half) | README.md section #1 (t=4805, 3 atoms) |
| 1.4 | 178 | 300 | +122 | 1.00 | late | internal/ subpackage listing |  |
| 1.5 | 316 | 4805 | +4489 | 1.00 | late | README feature lede (rest) | README.md section #1 (t=4805, 6 atoms) |
| 1.6 | 435 | — | — | 0.75 | partial | go.mod module + Go version + UI deps | go module file go.mod (t=653, 6 atoms) |
| 2.1 | 690 | 1181 | +491 | 0.80 | late | Sentinel errors (whole file) | go decl at internal/core/errors/errors.go:5 (t=1181, 6 atoms) |
| 2.2 | 810 | 4924 | +4114 | 1.00 | late | models.Activity struct fields | go decl at internal/core/models/activity.go:10 (t=4924, 8 atoms) |
| 2.3 | 1062 | 7555 | +6493 | 0.85 | late | ports.ActivityResolver interface | go decl at internal/core/ports/ports.go:11 (t=7555, 10 atoms) |
| 2.4 | 1217 | 3119 | +1902 | 0.91 | late | ports.ActivityRepository + NotesRepository | go decl at internal/core/ports/ports.go:22 (t=3119, 6 atoms) |
| 2.5 | 1630 | 5135 | +3505 | 0.84 | late | dto request/filter/report types | go decl names surface in internal/core/dto/activity_dto.go (t=1249, 12 atoms) |
| 2.6 | 1858 | — | — | 0.43 | missing | models.Activity helper methods | go decl names surface in internal/core/models/activity.go (t=1743, 6 atoms) |
| 3.1 | 1926 | — | — | 0.67 | partial | main.go — entry point | go package + imports in cmd/tock/main.go (t=1810, 4 atoms) |
| 3.3 | 2188 | 5495 | +3307 | 1.00 | late | Cobra command factory locations across cli/ | go decl names surface in internal/adapters/cli/start.go (t=2397, 1 atoms) |
| 3.4 | 2423 | — | — | 0.00 | missing | README commands list (Use/Short for every cmd) |  |
| 4.1 | 2510 | — | — | 0.75 | partial | Activity service constructor | go decl names surface in internal/services/activity/service.go (t=8294, 4 atoms) |
| 4.2 | 3040 | — | — | 0.14 | missing | File-format ParseActivity | go decl names surface in internal/adapters/repositories/file/parser.go (t=5608, 7 atoms) |
| 4.3 | 3173 | — | — | 0.10 | missing | File-format FormatActivity (writer side) | go decl names surface in internal/adapters/repositories/file/parser.go (t=5608, 2 atoms) |
| 5.1 | 3722 | 5883 | +2161 | 0.87 | late | Config struct + sub-structs | go decl names surface in internal/config/config.go (t=2058, 14 atoms) |
| 5.2 | 4047 | — | — | 0.00 | missing | Env-var bindings (TOCK_*) |  |
| 6.1 | 4497 | — | — | 0.79 | partial | timeutil: Formatter type + display formats | go decl names surface in internal/timeutil/timeutil.go (t=3633, 15 atoms) |
| 6.2 | 4861 | — | — | 0.00 | missing | TimeWarrior repo: type + filename + toTWInterval |  |
| 6.3 | 5074 | — | — | 0.75 | partial | Notes-repo: paths + frontmatter + signatures | go decl names surface in internal/adapters/repositories/notes/repository.go (t=6380, 13 atoms) |
| 6.4 | 5311 | 8294 | +2983 | 1.00 | late | Service method signatures | go decl names surface in internal/services/activity/service.go (t=8294, 17 atoms) |
| 6.5 | 5652 | — | — | 0.21 | missing | extra.CalculateEndTime | go package + imports in internal/extra/extra.go (t=4506, 6 atoms) |
| 6.6 | 5880 | — | — | 0.20 | missing | File-repo: type + constructor | go decl names surface in internal/adapters/repositories/file/repository.go (t=6782, 4 atoms) |
| 6.7 | 6277 | — | — | 0.03 | missing | File-repo Find body (filter logic) | go decl names surface in internal/adapters/repositories/file/repository.go (t=6782, 2 atoms) |
| 7.1 | 6789 | — | — | 0.24 | missing | cli/root.go — context keys + NewRootCmd shell | go decl names surface in internal/adapters/cli/root.go (t=7333, 10 atoms) |
| 7.2 | 6956 | — | — | 0.00 | missing | cli/root.go — AddCommand block |  |
| 7.3 | 7464 | — | — | 0.00 | missing | cli/root.go — PersistentPreRunE body |  |
| 7.4 | 7530 | — | — | 0.17 | missing | cli/root.go — initRepository (file/timewarrior switch) | go decl names surface in internal/adapters/cli/root.go (t=7333, 2 atoms) |
| 7.5 | 8014 | — | — | 0.00 | missing | Service Start body | go decl names surface in internal/services/activity/service.go (t=8294, 1 atoms) |
| 8.1 | 8456 | — | — | 0.00 | missing | start/stop/add flag declarations |  |
| 8.2 | 8859 | 8868 | +9 | 0.94 | aligned | Theme/Styles fields + theme constructor names | go decl names surface in internal/adapters/cli/theme.go (t=4140, 17 atoms) |
| 9.1 | 9185 | — | — | 0.48 | missing | report flag declarations + reportOptions | go decl at internal/adapters/cli/report.go:20 (t=9031, 10 atoms) |
| 9.2 | 9203 | 788 | -8415 | 1.00 | early | Mocks directory listing |  |
| 9.3 | 9346 | — | — | 0.78 | partial | ICS generator — public fn signatures | go decl names surface in internal/services/ics/generator.go (t=1051, 6 atoms) |
| 9.5 | 9600 | — | — | 0.00 | missing | calendar.go: handleKeyMsg case lines |  |
| 9.6 | 9671 | 3284 | -6387 | 1.00 | early | calendar_sidebar.go: section renderer signatures | go decl names surface in internal/adapters/cli/calendar_sidebar.go (t=3284, 7 atoms) |
| 9.7 | 9955 | — | — | 0.00 | missing | Remaining commands' flag declarations |  |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 3 | 274 | README.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 281 | 0.96 | 294 | 8649 | go decl names surface in internal/adapters/cli/calendar.go |
| 258 | 1.00 | 258 | 7838 | docs/commands.md section #0 |
| 214 | 0.57 | 375 | 9590 | go decl names surface in internal/adapters/cli/interactive.go |
| 195 | 0.94 | 208 | 6091 | go decl names surface in internal/adapters/cli/list_gui.go |
| 182 | 1.00 | 182 | 970 | headings outline in docs/commands.md |
| 157 | 0.59 | 264 | 7333 | go decl names surface in internal/adapters/cli/root.go |
| 154 | 0.92 | 168 | 5495 | go decl names surface in internal/adapters/cli/ical.go |
| 153 | 1.00 | 153 | 7069 | go decl at internal/adapters/cli/analyze.go:70 |
| 152 | 0.92 | 165 | 5305 | go decl names surface in internal/adapters/cli/watch.go |
| 114 | 0.68 | 168 | 6782 | go decl names surface in internal/adapters/repositories/file/repository.go |
| 110 | 1.00 | 110 | 3965 | README.md section #7 |
| 100 | 1.00 | 100 | 9805 | go package + imports in internal/adapters/cli/last.go |
| 100 | 1.00 | 100 | 6237 | plaintext config .gitignore |
| 95 | 1.00 | 95 | 9215 | go package + imports in internal/adapters/cli/start.go |
| 89 | 1.00 | 89 | 9120 | go package + imports in internal/adapters/cli/continue.go |
| 87 | 0.87 | 100 | 3046 | go decl names surface in internal/adapters/cli/add.go |
| 85 | 1.00 | 85 | 1895 | README.md section #10 |
| 84 | 0.52 | 163 | 2058 | go decl names surface in internal/config/config.go |
| 80 | 0.86 | 93 | 2946 | go decl names surface in internal/adapters/cli/remove.go |
| 79 | 0.91 | 87 | 165 | README headline in README.md |
| 79 | 1.00 | 79 | 1588 | README.md section #6 |
| 75 | 1.00 | 75 | 7978 | go package + imports in internal/adapters/cli/stop.go |
| 72 | 1.00 | 72 | 4452 | go decl doc at internal/timeutil/timeutil.go:158 |
| 69 | 1.00 | 69 | 9705 | go decl at internal/adapters/cli/interactive.go:201 |
| 68 | 0.36 | 187 | 3633 | go decl names surface in internal/timeutil/timeutil.go |
| 68 | 1.00 | 68 | 9873 | go package + imports in internal/adapters/repositories/file/parser.go |
| 66 | 1.00 | 66 | 5093 | go package + imports in internal/config/config.go |
| 65 | 1.00 | 65 | 7903 | go decl at internal/adapters/cli/add.go:16 |
| 65 | 1.00 | 65 | 6614 | go package + imports in internal/adapters/cli/theme.go |
| 63 | 1.00 | 63 | 6549 | go package + imports in internal/adapters/cli/calendar_sidebar.go |
| 57 | 0.81 | 70 | 2853 | go decl names surface in internal/adapters/cli/analyze.go |
| 57 | 1.00 | 57 | 6486 | go package + imports in internal/services/ics/generator.go |
| 54 | 1.00 | 54 | 7396 | go decl body at internal/adapters/cli/root.go:107 |
| 51 | 0.80 | 64 | 2751 | go decl names surface in internal/adapters/cli/current.go |

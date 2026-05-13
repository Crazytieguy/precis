scores: Score(3000)=0.624 ns_rows≤3K=18/43 (reached=5 partial=4 missing=9)

## Per-budget scores

| B | A_B | I(B) | C(B) | compl(B) | Score(B) | walker_used |
|--:|----:|-----:|-----:|---------:|---------:|------------:|
| 1000 | 79 | 0.860 | 0.594 | 0.998 | 0.715 | 1000 |
| 1442 | 110 | 0.851 | 0.502 | 0.771 | 0.653 | 1442 |
| 2080 | 209 | 0.792 | 0.473 | 0.742 | 0.612 | 1976 |
| 3000 | 249 | 0.792 | 0.492 | 0.749 | 0.624 | 2934 |
| 4327 | 371 | 0.770 | 0.406 | 0.782 | 0.559 | 4324 |
| 6240 | 536 | 0.819 | 0.447 | 0.804 | 0.605 | 6045 |
| 9000 | 783 | 0.818 | 0.392 | 0.755 | 0.566 | 8907 |

## Top opportunities

### Additive (close partial / missing rows)

| intervention | rows | gap@1k | gap@3k | gap@9k | evidence | top row ids |
|:-------------|-----:|-------:|-------:|-------:|:---------|:------------|
| split wrong-slice walker batches | 25 | 2.31 | 1.84 | 1.03 | nearby candidates have low exact atom overlap | 1.3, 2.3, 4.2, 2.5, 5.1, ... |
| tune ranking for high-overlap unscheduled candidates | 7 | 0.35 | 0.35 | 0.35 | high-overlap candidates not in the schedule by T_max, exact total=160/184 | 3.4, 7.3, 6.7, 7.5, 5.2, ... |

### Subtractive (suppress consistently off-NS batches)

| pattern | batches | freed@1k | freed@3k | freed@9k | evidence | top batch ids |
|:--------|--------:|---------:|---------:|---------:|:---------|:--------------|
| headings outline in docs/commands.md | 1 | 182 | 182 | 182 | off_3k=182 | headings outline in docs/commands.md |
| README.md section #<n> | 2 | 0 | 164 | 274 | off_3k=164 | README.md section #10, README.md section #6 |
| go decl names surface in internal/config/config.go | 1 | 0 | 84 | 84 | off_3k=163 | go decl names surface in internal/config/config.go |
| go decl names surface in internal/adapters/cli/remove.go | 1 | 0 | 80 | 80 | off_3k=80 | go decl names surface in internal/adapters/cli/remove.go |
| README headline in README.md | 1 | 79 | 79 | 79 | off_3k=79 | README headline in README.md |

Top missed paths (NS rows ≤ 3K): internal/core/dto/activity_dto.go (1 row, 50 atoms), internal/core/ports/ports.go (2 rows, 31 atoms), internal/core/models/activity.go (2 rows, 30 atoms), README.md (3 rows, 29 atoms), internal/adapters/cli/add.go (1 row, 14 atoms), internal/core/errors/errors.go (1 row, 10 atoms), cmd/tock/main.go (1 row, 9 atoms), go.mod (1 row, 8 atoms), +1 more

## Arrival ledger by diagnosis

### ranking-recoverable

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 3.4 | 2423 | 0.00 | missing | README commands list (Use/Short for every cmd) | [unscheduled bbox exact=17/18] README.md section #5 (17 atoms, too expensive at final margin) |
| 5.2 | 4047 | 0.00 | missing | Env-var bindings (TOCK_*) | [unscheduled bbox exact=15/15] go decl body at internal/config/config.go:80 (15 atoms, too expensive at final margin) |
| 6.7 | 6277 | 0.00 | missing | File-repo Find body (filter logic) | [scheduled bbox exact=1/38] go decl names surface in internal/adapters/repositories/file/repository.go (t=6944, 2 atoms); better unscheduled exact=32/38: go decl body at internal/adapters/repositories/file/repository.go:27 (54 atoms, too expensive at final margin) |
| 7.2 | 6956 | 0.00 | missing | cli/root.go — AddCommand block | [unscheduled bbox exact=15/16] go decl body at internal/adapters/cli/root.go:31 (15 atoms, too expensive at final margin) |
| 7.3 | 7464 | 0.00 | missing | cli/root.go — PersistentPreRunE body | [unscheduled bbox exact=37/45] go decl body at internal/adapters/cli/root.go:31 (37 atoms, too expensive at final margin) |
| 7.5 | 8014 | 0.00 | missing | Service Start body | [scheduled bbox exact=1/43] go decl names surface in internal/services/activity/service.go (t=8479, 1 atoms); better unscheduled exact=35/43: go decl body at internal/services/activity/service.go:24 (35 atoms, too expensive at final margin) |
| 9.5 | 9600 | 0.00 | missing | calendar.go: handleKeyMsg case lines | [unscheduled bbox exact=9/9] go decl body at internal/adapters/cli/calendar.go:520 (45 atoms, too expensive at final margin) |

### wrong-slice / granularity

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 1.3 | 134 | 0.00 | missing | README feature lede (first half) | [scheduled bbox exact=3/5] README.md section #1 (t=4924, 3 atoms) |
| 1.6 | 435 | 0.75 | partial | go.mod module + Go version + UI deps | [scheduled bbox exact=6/8] go module file go.mod (t=653, 6 atoms) |
| 2.1 | 690 | 0.80 | partial | Sentinel errors (whole file) | [scheduled bbox exact=6/10] go decl at internal/core/errors/errors.go:5 (t=1216, 6 atoms) |
| 2.3 | 1062 | 0.05 | missing | ports.ActivityResolver interface | [scheduled bbox exact=10/20] go decl at internal/core/ports/ports.go:11 (t=7740, 10 atoms) |
| 2.4 | 1217 | 0.46 | missing | ports.ActivityRepository + NotesRepository | [scheduled bbox exact=6/11] go decl at internal/core/ports/ports.go:22 (t=3200, 6 atoms) |
| 2.5 | 1630 | 0.74 | partial | dto request/filter/report types | [scheduled bbox exact=12/50] go decl names surface in internal/core/dto/activity_dto.go (t=1292, 12 atoms) |
| 2.6 | 1858 | 0.24 | missing | models.Activity helper methods | [scheduled bbox exact=6/21] go decl names surface in internal/core/models/activity.go (t=1786, 6 atoms); better unscheduled exact=7/21: go decl body at internal/core/models/activity.go:31 (7 atoms, too expensive at final margin) |
| 3.1 | 1926 | 0.67 | partial | main.go — entry point | [scheduled bbox exact=4/9] go package + imports in cmd/tock/main.go (t=1871, 4 atoms) |
| 3.3 | 2188 | 0.57 | missing | Cobra command factory locations across cli/ | [scheduled bbox exact=1/14] go decl at internal/adapters/cli/calendar.go:25 (t=8859, 1 atoms) |
| 4.1 | 2510 | 0.00 | missing | Activity service constructor | [scheduled bbox exact=4/8] go decl at internal/services/activity/service.go:15 (t=8530, 4 atoms) |
| 4.2 | 3040 | 0.00 | missing | File-format ParseActivity | [scheduled bbox exact=7/50] go decl names surface in internal/adapters/repositories/file/parser.go (t=5770, 7 atoms); better unscheduled exact=36/50: go decl body at internal/adapters/repositories/file/parser.go:20 (36 atoms, too expensive at final margin) |
| 4.3 | 3173 | 0.00 | missing | File-format FormatActivity (writer side) | [scheduled bbox exact=2/10] go decl names surface in internal/adapters/repositories/file/parser.go (t=5770, 2 atoms); better unscheduled exact=6/10: go decl body at internal/adapters/repositories/file/parser.go:72 (6 atoms, too expensive at final margin) |
| 5.1 | 3722 | 0.45 | missing | Config struct + sub-structs | [scheduled bbox exact=14/47] go decl names surface in internal/config/config.go (t=2139, 14 atoms) |
| 6.1 | 4497 | 0.00 | missing | timeutil: Formatter type + display formats | [scheduled bbox exact=15/43] go decl names surface in internal/timeutil/timeutil.go (t=3726, 15 atoms) |
| 6.2 | 4861 | 0.00 | missing | TimeWarrior repo: type + filename + toTWInterval | [unscheduled bbox exact=12/35] go decl names surface in internal/adapters/repositories/timewarrior/repository.go (30 atoms, too expensive at final margin) |
| 6.3 | 5074 | 0.00 | missing | Notes-repo: paths + frontmatter + signatures | [scheduled bbox exact=12/20] go decl names surface in internal/adapters/repositories/notes/repository.go (t=6542, 13 atoms) |
| 6.5 | 5652 | 0.03 | missing | extra.CalculateEndTime | [scheduled bbox exact=6/33] go package + imports in internal/extra/extra.go (t=4625, 6 atoms); better unscheduled exact=18/33: go decl body at internal/extra/extra.go:11 (18 atoms, too expensive at final margin) |
| 6.6 | 5880 | 0.00 | missing | File-repo: type + constructor | [scheduled bbox exact=4/25] go decl names surface in internal/adapters/repositories/file/repository.go (t=6944, 4 atoms); better unscheduled exact=14/25: go package + imports in internal/adapters/repositories/file/repository.go (14 atoms, too expensive at final margin) |
| 7.1 | 6789 | 0.00 | missing | cli/root.go — context keys + NewRootCmd shell | [scheduled bbox exact=10/42] go decl names surface in internal/adapters/cli/root.go (t=7518, 10 atoms); better unscheduled exact=15/42: go package + imports in internal/adapters/cli/root.go (15 atoms, too expensive at final margin) |
| 7.4 | 7530 | 0.00 | missing | cli/root.go — initRepository (file/timewarrior switch) | [scheduled bbox exact=2/6] go decl names surface in internal/adapters/cli/root.go (t=7518, 2 atoms); better unscheduled exact=4/6: go decl body at internal/adapters/cli/root.go:127 (4 atoms, too expensive at final margin) |
| 8.1 | 8456 | 0.00 | missing | start/stop/add flag declarations | [unscheduled bbox exact=9/21] go decl body at internal/adapters/cli/stop.go:14 (9 atoms, too expensive at final margin) |
| 8.2 | 8859 | 0.00 | missing | Theme/Styles fields + theme constructor names | [scheduled bbox exact=11/36] go decl names surface in internal/adapters/cli/theme.go (t=4233, 17 atoms) |
| 9.1 | 9185 | 0.05 | missing | report flag declarations + reportOptions | [scheduled bbox exact=10/21] go decl at internal/adapters/cli/report.go:20 (t=9268, 10 atoms) |
| 9.3 | 9346 | 0.78 | partial | ICS generator — public fn signatures | [scheduled bbox exact=6/9] go decl names surface in internal/services/ics/generator.go (t=1086, 6 atoms) |
| 9.7 | 9955 | 0.00 | missing | Remaining commands' flag declarations | [unscheduled bbox exact=3/12] go decl body at internal/adapters/cli/last.go:21 (3 atoms, too expensive at final margin) |

### mixed/unknown

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 1.5 | 316 | 0.00 | missing | README feature lede (rest) | [scheduled bbox exact=6/6] README.md section #1 (t=4924, 6 atoms) |
| 2.2 | 810 | 0.11 | missing | models.Activity struct fields | [scheduled bbox exact=8/9] go decl at internal/core/models/activity.go:10 (t=5071, 8 atoms) |
| 6.4 | 5311 | 0.00 | missing | Service method signatures | [scheduled bbox exact=9/9] go decl names surface in internal/services/activity/service.go (t=8479, 17 atoms) |
| 9.4 | 9507 | 0.00 | missing | Interactive helpers — three public fns | [scheduled bbox exact=7/7] go decl names surface in internal/adapters/cli/interactive.go (t=9885, 21 atoms) |
| 9.6 | 9671 | 0.00 | missing | calendar_sidebar.go: section renderer signatures | [scheduled bbox exact=4/4] go decl names surface in internal/adapters/cli/calendar_sidebar.go (t=3365, 7 atoms) |

Top wasted paths (off-NS at 3K): internal/config/config.go (253t, 2 batches), README.md (243t, 3 batches), docs/commands.md (182t, 1 batch), internal/adapters/cli/remove.go (80t, 1 batch), internal/services/ics/generator.go (66t, 1 batch), internal/adapters/cli/analyze.go (57t, 1 batch), internal/adapters/cli/current.go (51t, 1 batch)

## Walker waste rollup (by descriptor pattern)

| n | off_3k_total | pattern |
|--:|-------------:|:--------|
| 2 | 164 | README.md section #<n> |

## Walker waste (walker_t ≤ 3000, off_3k ≥ 50)

| off_3k | off_3k_ratio | off_any | cost | walker_t | batch |
|-------:|-------------:|--------:|-----:|---------:|:------|
| 182 | 1.00 | 182 | 182 | 788 | headings outline in docs/commands.md |
| 163 | 1.00 | 84 | 163 | 1976 | go decl names surface in internal/config/config.go |
| 90 | 1.00 | 0 | 90 | 2294 | go decl at internal/config/config.go:25 |
| 85 | 1.00 | 85 | 85 | 1891 | README.md section #10 |
| 80 | 0.86 | 80 | 93 | 2934 | go decl names surface in internal/adapters/cli/remove.go |
| 79 | 0.91 | 79 | 87 | 78 | README headline in README.md |
| 79 | 1.00 | 79 | 79 | 1552 | README.md section #6 |
| 66 | 1.00 | 14 | 66 | 1020 | go decl names surface in internal/services/ics/generator.go |
| 57 | 0.81 | 57 | 70 | 2864 | go decl names surface in internal/adapters/cli/analyze.go |
| 51 | 0.80 | 51 | 64 | 2768 | go decl names surface in internal/adapters/cli/current.go |

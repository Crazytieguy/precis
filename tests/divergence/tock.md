scores: Score(3000)=0.624 ns_rows≤3K=18/43 (reached=5 partial=4 missing=9)

## Per-budget scores

| B | A_B | I(B) | C(B) | Score(B) | walker_used |
|--:|----:|-----:|-----:|---------:|------------:|
| 1000 | 79 | 0.860 | 0.594 | 0.715 | 1000 |
| 1442 | 110 | 0.851 | 0.502 | 0.653 | 1442 |
| 2080 | 209 | 0.792 | 0.473 | 0.612 | 1976 |
| 3000 | 249 | 0.792 | 0.492 | 0.624 | 2934 |
| 4327 | 371 | 0.770 | 0.406 | 0.559 | 4324 |
| 6240 | 536 | 0.819 | 0.447 | 0.605 | 6045 |
| 9000 | 783 | 0.818 | 0.392 | 0.566 | 8907 |

## Verdict

Verdict: wrong-slice bound
Likely primary lever: split walker batches to match NS semantic slices
Evidence: 7 ranking-recoverable (gap@3k=0.35), 25 wrong-slice/granularity (gap@3k=1.84), 0 no-discovered (gap@3k=0.00)
Secondary intervention: free T_max budget for 7 too-expensive candidates
Top rows: 1.3, 2.3, 4.2, 2.5, 5.1, ...

## Top opportunities

_`gap@B` is a non-additive priority score: `Σ over atoms in row: (1 − damped_credit(a)) / rank(a)` evaluated at budget B's walker state. Approximates how much closing the row would lift `Score(B)` (via the Importance numerator); not an exact delta. `gap@3k` is the primary sort key. `gap@1k` and `gap@9k` show how the same intervention scales across the budget vector — gap is monotone non-increasing in B (walker has more budget at higher B). Rows can overlap between opportunities; sums are upper bounds on Score(B) impact, not additive estimates._

| intervention | rows | gap@1k | gap@3k | gap@9k | evidence | top row ids |
|:-------------|-----:|-------:|-------:|-------:|:---------|:------------|
| split wrong-slice walker batches | 25 | 2.31 | 1.84 | 1.03 | nearby candidates have low exact atom overlap | 1.3, 2.3, 4.2, 2.5, 5.1, ... |
| free T_max budget / demote late waste | 7 | 0.35 | 0.35 | 0.35 | high-overlap candidates exceed remaining budget at T_max (caveat: not 3K-budget — see below), exact total=160/184 | 3.4, 7.3, 6.7, 7.5, 5.2, ... |
| finish partially-delivered NS batches | 4 | 0.43 | 0.35 | 0.12 | avg batch completion=0.48 | 5.1, 2.6, 2.4, 3.3 |

## Diagnosis rollup

| diagnosis | rows | missing | partial | likely lever |
|:----------|-----:|--------:|--------:|:-------------|
| ranking-recoverable | 7 | 7 | 0 | value/ranking |
| wrong-slice / granularity | 25 | 20 | 5 | walker granularity / wrong slice |
| mixed/unknown | 5 | 5 | 0 | inspect row |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | gap@3k | likely lever |
|:------------|-----:|-------:|:-------------|
| too expensive at final margin | 7 | 0.35 | free T_max budget |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=29, unscheduled bbox=8

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | missing | low | 19 |
| scheduled bbox | missing | high | 1 |
| scheduled bbox | missing | full | 4 |
| scheduled bbox | partial | low | 5 |
| unscheduled bbox | missing | low | 3 |
| unscheduled bbox | missing | high | 3 |
| unscheduled bbox | missing | full | 2 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 3.4 | 2423 | 0.00 | 0.00 | missing | README commands list (Use/Short for every cmd) | [unscheduled bbox exact=17/18] README.md section #5 (17 atoms, too expensive at final margin) |
| 5.2 | 4047 | 0.00 | 0.00 | missing | Env-var bindings (TOCK_*) | [unscheduled bbox exact=15/15] go decl body at internal/config/config.go:80 (15 atoms, too expensive at final margin) |
| 6.7 | 6277 | 0.00 | 0.00 | missing | File-repo Find body (filter logic) | [scheduled bbox exact=1/38] go decl names surface in internal/adapters/repositories/file/repository.go (t=6944, 2 atoms); better unscheduled exact=32/38: go decl body at internal/adapters/repositories/file/repository.go:27 (54 atoms, too expensive at final margin) |
| 7.2 | 6956 | 0.00 | 0.00 | missing | cli/root.go — AddCommand block | [unscheduled bbox exact=15/16] go decl body at internal/adapters/cli/root.go:31 (15 atoms, too expensive at final margin) |
| 7.3 | 7464 | 0.00 | 0.00 | missing | cli/root.go — PersistentPreRunE body | [unscheduled bbox exact=37/45] go decl body at internal/adapters/cli/root.go:31 (37 atoms, too expensive at final margin) |
| 7.5 | 8014 | 0.00 | 0.00 | missing | Service Start body | [scheduled bbox exact=1/43] go decl names surface in internal/services/activity/service.go (t=8479, 1 atoms); better unscheduled exact=35/43: go decl body at internal/services/activity/service.go:24 (35 atoms, too expensive at final margin) |
| 9.5 | 9600 | 0.00 | 0.00 | missing | calendar.go: handleKeyMsg case lines | [unscheduled bbox exact=9/9] go decl body at internal/adapters/cli/calendar.go:520 (45 atoms, too expensive at final margin) |

### wrong-slice / granularity

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 1.3 | 134 | 0.00 | 0.00 | missing | README feature lede (first half) | [scheduled bbox exact=3/5] README.md section #1 (t=4924, 3 atoms) |
| 1.6 | 435 | 0.75 | 0.99 | partial | go.mod module + Go version + UI deps | [scheduled bbox exact=6/8] go module file go.mod (t=653, 6 atoms) |
| 2.1 | 690 | 0.80 | 0.99 | partial | Sentinel errors (whole file) | [scheduled bbox exact=6/10] go decl at internal/core/errors/errors.go:5 (t=1216, 6 atoms) |
| 2.3 | 1062 | 0.05 | 0.04 | missing | ports.ActivityResolver interface | [scheduled bbox exact=10/20] go decl at internal/core/ports/ports.go:11 (t=7740, 10 atoms) |
| 2.4 | 1217 | 0.46 | 0.50 | missing | ports.ActivityRepository + NotesRepository | [scheduled bbox exact=6/11] go decl at internal/core/ports/ports.go:22 (t=3200, 6 atoms) |
| 2.5 | 1630 | 0.74 | 0.90 | partial | dto request/filter/report types | [scheduled bbox exact=12/50] go decl names surface in internal/core/dto/activity_dto.go (t=1292, 12 atoms) |
| 2.6 | 1858 | 0.24 | 0.45 | missing | models.Activity helper methods | [scheduled bbox exact=6/21] go decl names surface in internal/core/models/activity.go (t=1786, 6 atoms); better unscheduled exact=7/21: go decl body at internal/core/models/activity.go:31 (7 atoms, too expensive at final margin) |
| 3.1 | 1926 | 0.67 | 0.97 | partial | main.go — entry point | [scheduled bbox exact=4/9] go package + imports in cmd/tock/main.go (t=1871, 4 atoms) |
| 3.3 | 2188 | 0.57 | 0.58 | missing | Cobra command factory locations across cli/ | [scheduled bbox exact=1/14] go decl at internal/adapters/cli/calendar.go:25 (t=8859, 1 atoms) |
| 4.1 | 2510 | 0.00 | 0.00 | missing | Activity service constructor | [scheduled bbox exact=4/8] go decl at internal/services/activity/service.go:15 (t=8530, 4 atoms) |
| 4.2 | 3040 | 0.00 | 0.00 | missing | File-format ParseActivity | [scheduled bbox exact=7/50] go decl names surface in internal/adapters/repositories/file/parser.go (t=5770, 7 atoms); better unscheduled exact=36/50: go decl body at internal/adapters/repositories/file/parser.go:20 (36 atoms, too expensive at final margin) |
| 4.3 | 3173 | 0.00 | 0.00 | missing | File-format FormatActivity (writer side) | [scheduled bbox exact=2/10] go decl names surface in internal/adapters/repositories/file/parser.go (t=5770, 2 atoms); better unscheduled exact=6/10: go decl body at internal/adapters/repositories/file/parser.go:72 (6 atoms, too expensive at final margin) |
| 5.1 | 3722 | 0.45 | 0.40 | missing | Config struct + sub-structs | [scheduled bbox exact=14/47] go decl names surface in internal/config/config.go (t=2139, 14 atoms) |
| 6.1 | 4497 | 0.00 | 0.00 | missing | timeutil: Formatter type + display formats | [scheduled bbox exact=15/43] go decl names surface in internal/timeutil/timeutil.go (t=3726, 15 atoms) |
| 6.2 | 4861 | 0.00 | 0.00 | missing | TimeWarrior repo: type + filename + toTWInterval | [unscheduled bbox exact=12/35] go decl names surface in internal/adapters/repositories/timewarrior/repository.go (30 atoms, too expensive at final margin) |
| 6.3 | 5074 | 0.00 | 0.00 | missing | Notes-repo: paths + frontmatter + signatures | [scheduled bbox exact=12/20] go decl names surface in internal/adapters/repositories/notes/repository.go (t=6542, 13 atoms) |
| 6.5 | 5652 | 0.03 | 0.15 | missing | extra.CalculateEndTime | [scheduled bbox exact=6/33] go package + imports in internal/extra/extra.go (t=4625, 6 atoms); better unscheduled exact=18/33: go decl body at internal/extra/extra.go:11 (18 atoms, too expensive at final margin) |
| 6.6 | 5880 | 0.00 | 0.00 | missing | File-repo: type + constructor | [scheduled bbox exact=4/25] go decl names surface in internal/adapters/repositories/file/repository.go (t=6944, 4 atoms); better unscheduled exact=14/25: go package + imports in internal/adapters/repositories/file/repository.go (14 atoms, too expensive at final margin) |
| 7.1 | 6789 | 0.00 | 0.00 | missing | cli/root.go — context keys + NewRootCmd shell | [scheduled bbox exact=10/42] go decl names surface in internal/adapters/cli/root.go (t=7518, 10 atoms); better unscheduled exact=15/42: go package + imports in internal/adapters/cli/root.go (15 atoms, too expensive at final margin) |
| 7.4 | 7530 | 0.00 | 0.00 | missing | cli/root.go — initRepository (file/timewarrior switch) | [scheduled bbox exact=2/6] go decl names surface in internal/adapters/cli/root.go (t=7518, 2 atoms); better unscheduled exact=4/6: go decl body at internal/adapters/cli/root.go:127 (4 atoms, too expensive at final margin) |
| 8.1 | 8456 | 0.00 | 0.00 | missing | start/stop/add flag declarations | [unscheduled bbox exact=9/21] go decl body at internal/adapters/cli/stop.go:14 (9 atoms, too expensive at final margin) |
| 8.2 | 8859 | 0.00 | 0.00 | missing | Theme/Styles fields + theme constructor names | [scheduled bbox exact=11/36] go decl names surface in internal/adapters/cli/theme.go (t=4233, 17 atoms) |
| 9.1 | 9185 | 0.05 | 0.03 | missing | report flag declarations + reportOptions | [scheduled bbox exact=10/21] go decl at internal/adapters/cli/report.go:20 (t=9268, 10 atoms) |
| 9.3 | 9346 | 0.78 | 0.85 | partial | ICS generator — public fn signatures | [scheduled bbox exact=6/9] go decl names surface in internal/services/ics/generator.go (t=1086, 6 atoms) |
| 9.7 | 9955 | 0.00 | 0.00 | missing | Remaining commands' flag declarations | [unscheduled bbox exact=3/12] go decl body at internal/adapters/cli/last.go:21 (3 atoms, too expensive at final margin) |

### mixed/unknown

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 1.5 | 316 | 0.00 | 0.00 | missing | README feature lede (rest) | [scheduled bbox exact=6/6] README.md section #1 (t=4924, 6 atoms) |
| 2.2 | 810 | 0.11 | 0.06 | missing | models.Activity struct fields | [scheduled bbox exact=8/9] go decl at internal/core/models/activity.go:10 (t=5071, 8 atoms) |
| 6.4 | 5311 | 0.00 | 0.00 | missing | Service method signatures | [scheduled bbox exact=9/9] go decl names surface in internal/services/activity/service.go (t=8479, 17 atoms) |
| 9.4 | 9507 | 0.00 | 0.00 | missing | Interactive helpers — three public fns | [scheduled bbox exact=7/7] go decl names surface in internal/adapters/cli/interactive.go (t=9885, 21 atoms) |
| 9.6 | 9671 | 0.00 | 0.00 | missing | calendar_sidebar.go: section renderer signatures | [scheduled bbox exact=4/4] go decl names surface in internal/adapters/cli/calendar_sidebar.go (t=3365, 7 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 3 | 274 | README.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 281 | 0.96 | 294 | 8859 | go decl names surface in internal/adapters/cli/calendar.go |
| 258 | 1.00 | 258 | 8023 | docs/commands.md section #0 |
| 214 | 0.57 | 375 | 9885 | go decl names surface in internal/adapters/cli/interactive.go |
| 195 | 0.94 | 208 | 6253 | go decl names surface in internal/adapters/cli/list_gui.go |
| 182 | 1.00 | 182 | 970 | headings outline in docs/commands.md |
| 157 | 0.59 | 264 | 7518 | go decl names surface in internal/adapters/cli/root.go |
| 154 | 0.92 | 168 | 5657 | go decl names surface in internal/adapters/cli/ical.go |
| 153 | 1.00 | 153 | 7231 | go decl at internal/adapters/cli/analyze.go:70 |
| 152 | 0.92 | 165 | 5467 | go decl names surface in internal/adapters/cli/watch.go |
| 114 | 0.68 | 168 | 6944 | go decl names surface in internal/adapters/repositories/file/repository.go |
| 1650 | — | — | — | +22 more rows |

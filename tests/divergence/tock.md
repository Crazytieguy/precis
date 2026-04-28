scores: Sim=0.563 Reached=18/43 Early=2 Late=11 Partial=7 Missing=18 Used=10000/10000

## Verdict

Verdict: wrong-slice bound
Likely primary lever: split walker batches to match NS semantic slices
Evidence: 7 ranking-recoverable (w×gap=0.55), 18 wrong-slice/granularity (w×gap=1.66), 0 no-discovered (w×gap=0.00)
Secondary intervention: free final budget for 7 too-expensive candidates
Loss reasons: 0 predecessor-gated, 7 too-expensive, 0 discovered-unscheduled
Top rows: 1.3, 2.6, 1.6, 4.2, 4.3, ...
Note: likely lever is heuristic; verify `Sim` moves, not just bucket counts.

## Top opportunities

_`w(t)×gap` is a non-additive priority score: Σ exp(-exp_t/τ) × (1 - credit) per row, τ=2000. Same time weighting and credit gap as Sim, but rows can overlap between opportunities — sums across rows are an upper bound on Sim impact, not an additive estimate._

| intervention | rows | w(t)×gap | bands ≤3k/≤6k/total | evidence | top row ids |
|:-------------|-----:|---------:|:----------------------|:---------|:------------|
| split wrong-slice walker batches | 18 | 1.66 | 5/12/18 | nearby candidates have low exact atom overlap | 1.3, 2.6, 1.6, 4.2, 4.3, ... |
| free final budget / demote late waste | 7 | 0.55 | 1/2/7 | high-overlap candidates exceed final remaining budget, exact total=160/184 | 3.4, 5.2, 6.7, 7.2, 7.3, ... |

Tiers: 1=5/7 reached, 2 partial, 0 missing, avg=0.91; 2=5/6 reached, 0 partial, 1 missing, avg=0.80; 3=2/4 reached, 1 partial, 1 missing, avg=0.67; 4=0/3 reached, 1 partial, 2 missing, avg=0.33; 5=1/2 reached, 0 partial, 1 missing, avg=0.44; 6=1/7 reached, 2 partial, 4 missing, avg=0.43; 7=0/5 reached, 0 partial, 5 missing, avg=0.08; 8=1/2 reached, 0 partial, 1 missing, avg=0.47; 9=3/7 reached, 1 partial, 3 missing, avg=0.61

## Diagnosis rollup

| diagnosis | rows | missing | partial | timing | likely lever |
|:----------|-----:|--------:|--------:|-------:|:-------------|
| ranking-recoverable | 7 | 7 | 0 | 0 | value/ranking |
| wrong-slice / granularity | 18 | 11 | 7 | 0 | walker granularity / wrong slice |
| timing-only | 14 | 0 | 0 | 14 | usually no code change |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | w(t)×gap | likely lever |
|:------------|-----:|---------:|:-------------|
| too expensive at final margin | 7 | 0.55 | free final budget |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=29, unscheduled bbox=8, fs-only=2

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | aligned | low | 1 |
| scheduled bbox | early | full | 1 |
| scheduled bbox | late | low | 6 |
| scheduled bbox | late | high | 1 |
| scheduled bbox | late | full | 3 |
| scheduled bbox | missing | low | 10 |
| scheduled bbox | partial | low | 7 |
| unscheduled bbox | missing | low | 3 |
| unscheduled bbox | missing | high | 3 |
| unscheduled bbox | missing | full | 2 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 3.4 | 2423 | — | — | 0.00 | missing | README commands list (Use/Short for every cmd) | [unscheduled bbox exact=17/18] README.md section #5 (17 atoms, too expensive at final margin) |
| 5.2 | 4047 | — | — | 0.00 | missing | Env-var bindings (TOCK_*) | [unscheduled bbox exact=15/15] go decl body at internal/config/config.go:80 (15 atoms, too expensive at final margin) |
| 6.7 | 6277 | — | — | 0.03 | missing | File-repo Find body (filter logic) | [scheduled bbox exact=1/38] go decl names surface in internal/adapters/repositories/file/repository.go (t=6944, 2 atoms); better unscheduled exact=32/38: go decl body at internal/adapters/repositories/file/repository.go:27 (54 atoms, too expensive at final margin) |
| 7.2 | 6956 | — | — | 0.00 | missing | cli/root.go — AddCommand block | [unscheduled bbox exact=15/16] go decl body at internal/adapters/cli/root.go:31 (15 atoms, too expensive at final margin) |
| 7.3 | 7464 | — | — | 0.00 | missing | cli/root.go — PersistentPreRunE body | [unscheduled bbox exact=37/45] go decl body at internal/adapters/cli/root.go:31 (37 atoms, too expensive at final margin) |
| 7.5 | 8014 | — | — | 0.00 | missing | Service Start body | [scheduled bbox exact=1/43] go decl names surface in internal/services/activity/service.go (t=8479, 1 atoms); better unscheduled exact=35/43: go decl body at internal/services/activity/service.go:24 (35 atoms, too expensive at final margin) |
| 9.5 | 9600 | — | — | 0.00 | missing | calendar.go: handleKeyMsg case lines | [unscheduled bbox exact=9/9] go decl body at internal/adapters/cli/calendar.go:520 (45 atoms, too expensive at final margin) |

### wrong-slice / granularity

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.3 | 134 | — | — | 0.60 | partial | README feature lede (first half) | [scheduled bbox exact=3/5] README.md section #1 (t=4924, 3 atoms) |
| 1.6 | 435 | — | — | 0.75 | partial | go.mod module + Go version + UI deps | [scheduled bbox exact=6/8] go module file go.mod (t=653, 6 atoms) |
| 2.6 | 1858 | — | — | 0.43 | missing | models.Activity helper methods | [scheduled bbox exact=6/21] go decl names surface in internal/core/models/activity.go (t=1786, 6 atoms); better unscheduled exact=7/21: go decl body at internal/core/models/activity.go:31 (7 atoms, too expensive at final margin) |
| 3.1 | 1926 | — | — | 0.67 | partial | main.go — entry point | [scheduled bbox exact=4/9] go package + imports in cmd/tock/main.go (t=1871, 4 atoms) |
| 4.1 | 2510 | — | — | 0.75 | partial | Activity service constructor | [scheduled bbox exact=4/8] go decl at internal/services/activity/service.go:15 (t=8530, 4 atoms) |
| 4.2 | 3040 | — | — | 0.14 | missing | File-format ParseActivity | [scheduled bbox exact=7/50] go decl names surface in internal/adapters/repositories/file/parser.go (t=5770, 7 atoms); better unscheduled exact=36/50: go decl body at internal/adapters/repositories/file/parser.go:20 (36 atoms, too expensive at final margin) |
| 4.3 | 3173 | — | — | 0.10 | missing | File-format FormatActivity (writer side) | [scheduled bbox exact=2/10] go decl names surface in internal/adapters/repositories/file/parser.go (t=5770, 2 atoms); better unscheduled exact=6/10: go decl body at internal/adapters/repositories/file/parser.go:72 (6 atoms, too expensive at final margin) |
| 6.1 | 4497 | — | — | 0.79 | partial | timeutil: Formatter type + display formats | [scheduled bbox exact=15/43] go decl names surface in internal/timeutil/timeutil.go (t=3726, 15 atoms) |
| 6.2 | 4861 | — | — | 0.00 | missing | TimeWarrior repo: type + filename + toTWInterval | [unscheduled bbox exact=12/35] go decl names surface in internal/adapters/repositories/timewarrior/repository.go (30 atoms, too expensive at final margin) |
| 6.3 | 5074 | — | — | 0.75 | partial | Notes-repo: paths + frontmatter + signatures | [scheduled bbox exact=12/20] go decl names surface in internal/adapters/repositories/notes/repository.go (t=6542, 13 atoms) |
| 6.5 | 5652 | — | — | 0.21 | missing | extra.CalculateEndTime | [scheduled bbox exact=6/33] go package + imports in internal/extra/extra.go (t=4625, 6 atoms); better unscheduled exact=18/33: go decl body at internal/extra/extra.go:11 (18 atoms, too expensive at final margin) |
| 6.6 | 5880 | — | — | 0.20 | missing | File-repo: type + constructor | [scheduled bbox exact=4/25] go decl names surface in internal/adapters/repositories/file/repository.go (t=6944, 4 atoms); better unscheduled exact=14/25: go package + imports in internal/adapters/repositories/file/repository.go (14 atoms, too expensive at final margin) |
| 7.1 | 6789 | — | — | 0.24 | missing | cli/root.go — context keys + NewRootCmd shell | [scheduled bbox exact=10/42] go decl names surface in internal/adapters/cli/root.go (t=7518, 10 atoms); better unscheduled exact=15/42: go package + imports in internal/adapters/cli/root.go (15 atoms, too expensive at final margin) |
| 7.4 | 7530 | — | — | 0.17 | missing | cli/root.go — initRepository (file/timewarrior switch) | [scheduled bbox exact=2/6] go decl names surface in internal/adapters/cli/root.go (t=7518, 2 atoms); better unscheduled exact=4/6: go decl body at internal/adapters/cli/root.go:127 (4 atoms, too expensive at final margin) |
| 8.1 | 8456 | — | — | 0.00 | missing | start/stop/add flag declarations | [unscheduled bbox exact=9/21] go decl body at internal/adapters/cli/stop.go:14 (9 atoms, too expensive at final margin) |
| 9.1 | 9185 | — | — | 0.48 | missing | report flag declarations + reportOptions | [scheduled bbox exact=10/21] go decl at internal/adapters/cli/report.go:20 (t=9268, 10 atoms) |
| 9.3 | 9346 | — | — | 0.78 | partial | ICS generator — public fn signatures | [scheduled bbox exact=6/9] go decl names surface in internal/services/ics/generator.go (t=1086, 6 atoms) |
| 9.7 | 9955 | — | — | 0.00 | missing | Remaining commands' flag declarations | [unscheduled bbox exact=3/12] go decl body at internal/adapters/cli/last.go:21 (3 atoms, too expensive at final margin) |

### timing-only

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.1 | 8 | 165 | +157 | 1.00 | late | README title | [scheduled bbox exact=1/1] README headline in README.md (t=165, 1 atoms) |
| 1.4 | 178 | 300 | +122 | 1.00 | late | internal/ subpackage listing | fs-only |
| 1.5 | 316 | 4924 | +4608 | 1.00 | late | README feature lede (rest) | [scheduled bbox exact=6/6] README.md section #1 (t=4924, 6 atoms) |
| 2.1 | 690 | 1216 | +526 | 0.80 | late | Sentinel errors (whole file) | [scheduled bbox exact=6/10] go decl at internal/core/errors/errors.go:5 (t=1216, 6 atoms) |
| 2.2 | 810 | 5071 | +4261 | 1.00 | late | models.Activity struct fields | [scheduled bbox exact=8/9] go decl at internal/core/models/activity.go:10 (t=5071, 8 atoms) |
| 2.3 | 1062 | 7740 | +6678 | 0.85 | late | ports.ActivityResolver interface | [scheduled bbox exact=10/20] go decl at internal/core/ports/ports.go:11 (t=7740, 10 atoms) |
| 2.4 | 1217 | 3200 | +1983 | 0.91 | late | ports.ActivityRepository + NotesRepository | [scheduled bbox exact=6/11] go decl at internal/core/ports/ports.go:22 (t=3200, 6 atoms) |
| 2.5 | 1630 | 5297 | +3667 | 0.84 | late | dto request/filter/report types | [scheduled bbox exact=12/50] go decl names surface in internal/core/dto/activity_dto.go (t=1292, 12 atoms) |
| 3.3 | 2188 | 5657 | +3469 | 1.00 | late | Cobra command factory locations across cli/ | [scheduled bbox exact=1/14] go decl at internal/adapters/cli/calendar.go:25 (t=8859, 1 atoms) |
| 5.1 | 3722 | 6045 | +2323 | 0.87 | late | Config struct + sub-structs | [scheduled bbox exact=14/47] go decl names surface in internal/config/config.go (t=2139, 14 atoms) |
| 6.4 | 5311 | 8479 | +3168 | 1.00 | late | Service method signatures | [scheduled bbox exact=9/9] go decl names surface in internal/services/activity/service.go (t=8479, 17 atoms) |
| 8.2 | 8859 | 9078 | +219 | 0.94 | aligned | Theme/Styles fields + theme constructor names | [scheduled bbox exact=11/36] go decl names surface in internal/adapters/cli/theme.go (t=4233, 17 atoms) |
| 9.2 | 9203 | 788 | -8415 | 1.00 | early | Mocks directory listing | fs-only |
| 9.6 | 9671 | 3365 | -6306 | 1.00 | early | calendar_sidebar.go: section renderer signatures | [scheduled bbox exact=4/4] go decl names surface in internal/adapters/cli/calendar_sidebar.go (t=3365, 7 atoms) |

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

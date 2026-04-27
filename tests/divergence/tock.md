scores: Sim=0.379 Reached=7/43 Early=2 Late=4 Partial=1 Missing=35 Used=9848/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 7 | 4 | 1 | 2 | 0.66 |
| 2 | 6 | 0 | 0 | 6 | 0.00 |
| 3 | 4 | 2 | 0 | 2 | 0.49 |
| 4 | 3 | 0 | 0 | 3 | 0.00 |
| 5 | 2 | 0 | 0 | 2 | 0.00 |
| 6 | 7 | 0 | 0 | 7 | 0.00 |
| 7 | 5 | 0 | 0 | 5 | 0.00 |
| 8 | 2 | 0 | 0 | 2 | 0.00 |
| 9 | 7 | 1 | 0 | 6 | 0.14 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.1 | 8 | 165 | +157 | 1.00 | late | README title | README headline in README.md (t=165, 1 atoms) |
| 1.3 | 134 | — | — | 0.60 | partial | README feature lede (first half) | README.md section #1 (t=1243, 3 atoms) |
| 1.4 | 178 | 290 | +112 | 1.00 | late | internal/ subpackage listing |  |
| 1.5 | 316 | 1243 | +927 | 1.00 | late | README feature lede (rest) | README.md section #1 (t=1243, 6 atoms) |
| 1.6 | 435 | — | — | 0.00 | missing | go.mod module + Go version + UI deps |  |
| 1.7 | 589 | — | — | 0.00 | missing | go.mod remaining direct deps |  |
| 2.1 | 690 | — | — | 0.00 | missing | Sentinel errors (whole file) |  |
| 2.2 | 810 | — | — | 0.00 | missing | models.Activity struct fields |  |
| 2.3 | 1062 | — | — | 0.00 | missing | ports.ActivityResolver interface |  |
| 2.4 | 1217 | — | — | 0.00 | missing | ports.ActivityRepository + NotesRepository |  |
| 2.5 | 1630 | — | — | 0.00 | missing | dto request/filter/report types |  |
| 2.6 | 1858 | — | — | 0.00 | missing | models.Activity helper methods |  |
| 3.1 | 1926 | — | — | 0.00 | missing | main.go — entry point |  |
| 3.2 | 2005 | 854 | -1151 | 1.00 | early | cli/ directory listing |  |
| 3.3 | 2188 | — | — | 0.00 | missing | Cobra command factory locations across cli/ |  |
| 3.4 | 2423 | 8855 | +6432 | 0.94 | late | README commands list (Use/Short for every cmd) | README.md section #5 (t=8855, 17 atoms) |
| 4.1 | 2510 | — | — | 0.00 | missing | Activity service constructor |  |
| 4.2 | 3040 | — | — | 0.00 | missing | File-format ParseActivity |  |
| 4.3 | 3173 | — | — | 0.00 | missing | File-format FormatActivity (writer side) |  |
| 5.1 | 3722 | — | — | 0.00 | missing | Config struct + sub-structs |  |
| 5.2 | 4047 | — | — | 0.00 | missing | Env-var bindings (TOCK_*) |  |
| 6.1 | 4497 | — | — | 0.00 | missing | timeutil: Formatter type + display formats |  |
| 6.2 | 4861 | — | — | 0.00 | missing | TimeWarrior repo: type + filename + toTWInterval |  |
| 6.3 | 5074 | — | — | 0.00 | missing | Notes-repo: paths + frontmatter + signatures |  |
| 6.4 | 5311 | — | — | 0.00 | missing | Service method signatures |  |
| 6.5 | 5652 | — | — | 0.00 | missing | extra.CalculateEndTime |  |
| 6.6 | 5880 | — | — | 0.00 | missing | File-repo: type + constructor |  |
| 6.7 | 6277 | — | — | 0.00 | missing | File-repo Find body (filter logic) |  |
| 7.1 | 6789 | — | — | 0.00 | missing | cli/root.go — context keys + NewRootCmd shell |  |
| 7.2 | 6956 | — | — | 0.00 | missing | cli/root.go — AddCommand block |  |
| 7.3 | 7464 | — | — | 0.00 | missing | cli/root.go — PersistentPreRunE body |  |
| 7.4 | 7530 | — | — | 0.00 | missing | cli/root.go — initRepository (file/timewarrior switch) |  |
| 7.5 | 8014 | — | — | 0.00 | missing | Service Start body |  |
| 8.1 | 8456 | — | — | 0.00 | missing | start/stop/add flag declarations |  |
| 8.2 | 8859 | — | — | 0.00 | missing | Theme/Styles fields + theme constructor names |  |
| 9.1 | 9185 | — | — | 0.00 | missing | report flag declarations + reportOptions |  |
| 9.2 | 9203 | 429 | -8774 | 1.00 | early | Mocks directory listing |  |
| 9.3 | 9346 | — | — | 0.00 | missing | ICS generator — public fn signatures |  |
| 9.4 | 9507 | — | — | 0.00 | missing | Interactive helpers — three public fns |  |
| 9.5 | 9600 | — | — | 0.00 | missing | calendar.go: handleKeyMsg case lines |  |
| 9.6 | 9671 | — | — | 0.00 | missing | calendar_sidebar.go: section renderer signatures |  |
| 9.7 | 9955 | — | — | 0.00 | missing | Remaining commands' flag declarations |  |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 9 | 7020 | README.md section #<n> |
| 8 | 1524 | docs/commands.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 2623 | 0.92 | 2853 | 8855 | README.md section #5 |
| 1177 | 1.00 | 1177 | 4574 | README.md section #3 |
| 1155 | 1.00 | 1155 | 5729 | README.md section #4 |
| 853 | 1.00 | 853 | 2799 | README.md section #2 |
| 598 | 1.00 | 598 | 3397 | README.md section #9 |
| 340 | 1.00 | 340 | 1946 | README.md section #8 |
| 258 | 1.00 | 258 | 1606 | docs/commands.md section #0 |
| 241 | 1.00 | 241 | 9848 | docs/commands.md section #12 |
| 239 | 1.00 | 239 | 9607 | docs/commands.md section #6 |
| 182 | 1.00 | 182 | 611 | headings outline in docs/commands.md |
| 177 | 1.00 | 177 | 9368 | docs/commands.md section #4 |
| 171 | 1.00 | 171 | 9191 | docs/commands.md section #7 |
| 165 | 1.00 | 165 | 9020 | docs/commands.md section #8 |
| 141 | 1.00 | 141 | 6002 | docs/commands.md section #11 |
| 132 | 1.00 | 132 | 5861 | docs/commands.md section #9 |
| 110 | 1.00 | 110 | 964 | README.md section #7 |
| 100 | 1.00 | 100 | 1348 | plaintext config .gitignore |
| 85 | 1.00 | 85 | 775 | README.md section #10 |
| 79 | 0.91 | 87 | 165 | README headline in README.md |
| 79 | 1.00 | 79 | 690 | README.md section #6 |

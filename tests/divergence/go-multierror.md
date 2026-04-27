scores: Sim=0.383 Reached=6/39 Early=3 Late=2 Partial=4 Missing=29 Used=2608/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 3 | 2 | 0 | 1 | 0.67 |
| 2 | 4 | 1 | 0 | 3 | 0.25 |
| 3 | 8 | 0 | 0 | 8 | 0.00 |
| 4 | 6 | 3 | 3 | 0 | 0.81 |
| 5 | 8 | 0 | 0 | 8 | 0.00 |
| 6 | 10 | 0 | 1 | 9 | 0.06 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 181 | 297 | +116 | 1.00 | late | README lede + deprecation note | README headline in README.md (t=297, 4 atoms) |
| 1.3 | 252 | — | — | 0.00 | missing | Error struct definition |  |
| 2.1 | 337 | — | — | 0.00 | missing | Package + import declarations across all .go files |  |
| 2.2 | 400 | — | — | 0.00 | missing | All public function/method first-lines (multierror.go) |  |
| 2.3 | 564 | — | — | 0.00 | missing | Public function names across single-fn files |  |
| 2.4 | 592 | 348 | -244 | 1.00 | early | README H2 headings | README.md section #1 (t=1629, 77 atoms) |
| 3.1 | 725 | — | — | 0.00 | missing | Append doc comment + signature |  |
| 3.2 | 909 | — | — | 0.00 | missing | Error.Unwrap doc + signature |  |
| 3.3 | 1011 | — | — | 0.00 | missing | Group struct + Go/Wait signatures with doc comments |  |
| 3.4 | 1140 | — | — | 0.00 | missing | Flatten + Prefix doc + signatures |  |
| 3.5 | 1186 | — | — | 0.00 | missing | ErrorFormatFunc + ListFormatFunc docs |  |
| 3.6 | 1331 | — | — | 0.00 | missing | Error.Error + WrappedErrors + GoString docs |  |
| 3.7 | 1419 | — | — | 0.00 | missing | sort.Interface methods on Error |  |
| 3.8 | 1596 | — | — | 0.00 | missing | chain type explainer comment + decl |  |
| 4.1 | 1718 | 2608 | +890 | 0.80 | late | README usage section overview lines | README.md section #3 (t=2608, 66 atoms) |
| 4.2 | 1908 | — | — | 0.79 | partial | README Append usage example | README.md section #3 (t=2608, 15 atoms) |
| 4.3 | 2416 | — | — | 0.80 | partial | README errors.Is / errors.As / Unwrap stdlib-compat snippets | README.md section #3 (t=2608, 35 atoms) |
| 4.4 | 2690 | — | — | 0.73 | partial | README ErrorFormat + ErrorOrNil examples | README.md section #3 (t=2608, 58 atoms) |
| 4.5 | 3075 | 1629 | -1446 | 0.86 | early | README intro paragraph (unwrap + Go-version notes) | README.md section #2 (t=731, 11 atoms) |
| 4.6 | 3611 | 1629 | -1982 | 0.91 | early | README migration to errors.Join — basic + Group sections | README.md section #1 (t=1629, 64 atoms) |
| 5.1 | 3936 | — | — | 0.00 | missing | Append body |  |
| 5.2 | 4319 | — | — | 0.00 | missing | Error.Unwrap body + chain methods |  |
| 5.3 | 4466 | — | — | 0.00 | missing | Group.Go and Group.Wait bodies |  |
| 5.4 | 4669 | — | — | 0.00 | missing | Flatten body (Flatten + flatten recursion) |  |
| 5.5 | 4872 | — | — | 0.00 | missing | Prefix body |  |
| 5.6 | 5019 | — | — | 0.00 | missing | ListFormatFunc body |  |
| 5.7 | 5197 | — | — | 0.00 | missing | Error.Error / ErrorOrNil / WrappedErrors / GoString bodies |  |
| 5.8 | 5271 | — | — | 0.00 | missing | sort.Interface bodies |  |
| 6.1 | 5514 | — | — | 0.00 | missing | Test function name inventory across all _test.go files |  |
| 6.2 | 5622 | — | — | 0.00 | missing | Format expected-output strings from tests |  |
| 6.3 | 5988 | — | — | 0.00 | missing | TestErrorUnwrap — chain semantics in action |  |
| 6.4 | 6733 | — | — | 0.00 | missing | TestAppend bodies — Append edge cases |  |
| 6.5 | 7458 | — | — | 0.00 | missing | TestFlatten + TestGroup bodies |  |
| 6.6 | 8394 | — | — | 0.00 | missing | TestErrorIs + TestErrorAs bodies |  |
| 6.7 | 8988 | — | — | 0.00 | missing | Remaining test bodies (sort + prefix) |  |
| 6.8 | 9527 | — | — | 0.00 | missing | .github/ listing + workflow file structure |  |
| 6.9 | 9759 | — | — | 0.00 | missing | Makefile targets |  |
| 6.10 | 9864 | — | — | 0.62 | partial | Boilerplate metadata | headings outline in CHANGELOG.md (t=136, 8 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 2 | 313 | README.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 166 | 0.18 | 898 | 1629 | README.md section #1 |
| 147 | 0.53 | 276 | 731 | README.md section #2 |

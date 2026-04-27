scores: Sim=0.644 Reached=22/39 Early=3 Late=11 Partial=9 Missing=8 Used=6038/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 3 | 3 | 0 | 0 | 1.00 |
| 2 | 4 | 4 | 0 | 0 | 0.97 |
| 3 | 8 | 8 | 0 | 0 | 1.00 |
| 4 | 6 | 3 | 3 | 0 | 0.81 |
| 5 | 8 | 3 | 5 | 0 | 0.76 |
| 6 | 10 | 1 | 1 | 8 | 0.18 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 181 | 612 | +431 | 1.00 | late | README lede + deprecation note | README headline in README.md (t=612, 4 atoms) |
| 1.3 | 252 | 1400 | +1148 | 1.00 | late | Error struct definition | go decl at multierror.go:13 (t=1329, 4 atoms) |
| 2.1 | 337 | 963 | +626 | 0.90 | late | Package + import declarations across all .go files | go module file go.mod (t=102, 2 atoms) |
| 2.2 | 400 | 1306 | +906 | 1.00 | late | All public function/method first-lines (multierror.go) | go decl names surface in multierror.go (t=1306, 19 atoms) |
| 2.3 | 564 | 422 | -142 | 1.00 | aligned+over | Public function names across single-fn files | go decl body at group.go:20 (t=2428, 9 atoms) |
| 3.1 | 725 | 1921 | +1196 | 1.00 | late | Append doc comment + signature | go decl doc at append.go:14 (t=1921, 8 atoms) |
| 3.2 | 909 | 2339 | +1430 | 1.00 | late | Error.Unwrap doc + signature | go decl doc at multierror.go:71 (t=2339, 11 atoms) |
| 3.4 | 1140 | 1722 | +582 | 1.00 | late | Flatten + Prefix doc + signatures | go decl doc at prefix.go:16 (t=1722, 6 atoms) |
| 3.6 | 1331 | 2039 | +708 | 1.00 | late | Error.Error + WrappedErrors + GoString docs | go decl names surface in multierror.go (t=1306, 7 atoms) |
| 3.7 | 1419 | 463 | -956 | 1.00 | early | sort.Interface methods on Error | go decl names surface in sort.go (t=422, 6 atoms) |
| 3.8 | 1596 | 2769 | +1173 | 1.00 | late | chain type explainer comment + decl | go decl doc at multierror.go:99 (t=2769, 11 atoms) |
| 4.1 | 1718 | 5627 | +3909 | 0.80 | late | README usage section overview lines | README.md section #3 (t=5627, 66 atoms) |
| 4.2 | 1908 | — | — | 0.79 | partial | README Append usage example | README.md section #3 (t=5627, 15 atoms) |
| 4.3 | 2416 | — | — | 0.80 | partial | README errors.Is / errors.As / Unwrap stdlib-compat snippets | README.md section #3 (t=5627, 35 atoms) |
| 4.4 | 2690 | — | — | 0.73 | partial | README ErrorFormat + ErrorOrNil examples | README.md section #3 (t=5627, 58 atoms) |
| 4.5 | 3075 | 4648 | +1573 | 0.86 | late | README intro paragraph (unwrap + Go-version notes) | README.md section #2 (t=3447, 11 atoms) |
| 4.6 | 3611 | 4648 | +1037 | 0.91 | aligned | README migration to errors.Join — basic + Group sections | README.md section #1 (t=4648, 64 atoms) |
| 5.1 | 3936 | 3750 | -186 | 0.88 | aligned | Append body | go decl body at append.go:14 (t=3750, 28 atoms) |
| 5.2 | 4319 | — | — | 0.72 | partial | Error.Unwrap body + chain methods | go decl body at multierror.go:71 (t=2911, 12 atoms) |
| 5.3 | 4466 | — | — | 0.76 | partial | Group.Go and Group.Wait bodies | go decl body at group.go:20 (t=2428, 9 atoms) |
| 5.4 | 4669 | 2990 | -1679 | 0.81 | early | Flatten body (Flatten + flatten recursion) | go decl body at flatten.go:8 (t=2164, 8 atoms) |
| 5.5 | 4872 | 3171 | -1701 | 0.81 | early | Prefix body | go decl body at prefix.go:16 (t=3171, 17 atoms) |
| 5.6 | 5019 | — | — | 0.77 | partial | ListFormatFunc body | go decl body at format.go:17 (t=2592, 10 atoms) |
| 5.7 | 5197 | — | — | 0.74 | partial | Error.Error / ErrorOrNil / WrappedErrors / GoString bodies | go decl names surface in multierror.go (t=1306, 7 atoms) |
| 5.8 | 5271 | — | — | 0.56 | partial | sort.Interface bodies | go decl names surface in sort.go (t=422, 5 atoms) |
| 6.1 | 5514 | 6038 | +524 | 1.00 | aligned+over | Test function name inventory across all _test.go files | go test names surface in multierror_test.go (t=6038, 15 atoms) |
| 6.2 | 5622 | — | — | 0.01 | missing | Format expected-output strings from tests | go test names surface in format_test.go (t=5742, 3 atoms) |
| 6.3 | 5988 | — | — | 0.03 | missing | TestErrorUnwrap — chain semantics in action | go test names surface in multierror_test.go (t=6038, 2 atoms) |
| 6.4 | 6733 | — | — | 0.07 | missing | TestAppend bodies — Append edge cases | go test names surface in append_test.go (t=5904, 11 atoms) |
| 6.5 | 7458 | — | — | 0.00 | missing | TestFlatten + TestGroup bodies | go test names surface in flatten_test.go (t=5674, 3 atoms) |
| 6.6 | 8394 | — | — | 0.02 | missing | TestErrorIs + TestErrorAs bodies | go test names surface in multierror_test.go (t=6038, 4 atoms) |
| 6.7 | 8988 | — | — | 0.00 | missing | Remaining test bodies (sort + prefix) | go test names surface in prefix_test.go (t=5794, 5 atoms) |
| 6.8 | 9527 | — | — | 0.00 | missing | .github/ listing + workflow file structure |  |
| 6.9 | 9759 | — | — | 0.00 | missing | Makefile targets |  |
| 6.10 | 9864 | — | — | 0.62 | partial | Boilerplate metadata | headings outline in CHANGELOG.md (t=280, 8 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 2 | 313 | README.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 166 | 0.18 | 898 | 4648 | README.md section #1 |
| 147 | 0.53 | 276 | 3447 | README.md section #2 |
| 72 | 1.00 | 72 | 1557 | go decl doc at multierror.go:31 |

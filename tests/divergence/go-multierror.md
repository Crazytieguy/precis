scores: Sim=0.644 Reached=22/39 Early=3 Late=11 Partial=9 Missing=8 Used=6038/10000

## Verdict

Verdict: wrong-slice bound
Likely primary lever: split walker batches to match NS semantic slices
Evidence: 0 ranking-recoverable (w×gap=0.00), 15 wrong-slice/granularity (w×gap=0.53), 2 no-discovered (w×gap=0.02)
Secondary intervention: split wrong-slice batches for 15 rows
Loss reasons: 0 predecessor-gated, 0 too-expensive, 0 discovered-unscheduled
Top rows: 4.2, 4.4, 4.3, 6.2, 6.3, ...
Note: likely lever is heuristic; verify `Sim` moves, not just bucket counts.

## Top opportunities

_`w(t)×gap` is a non-additive priority score: Σ exp(-exp_t/τ) × (1 - credit) per row, τ=2000. Same time weighting and credit gap as Sim, but rows can overlap between opportunities — sums across rows are an upper bound on Sim impact, not an additive estimate._

| intervention | rows | w(t)×gap | bands ≤3k/≤6k/total | evidence | top row ids |
|:-------------|-----:|---------:|:----------------------|:---------|:------------|
| split wrong-slice walker batches | 15 | 0.53 | 3/10/15 | nearby candidates have low exact atom overlap | 4.2, 4.4, 4.3, 6.2, 6.3, ... |
| add walker candidates for no-discovered rows | 2 | 0.02 | 0/0/2 | NS rows have no discovered line candidate | 6.8, 6.9 |

Tiers: 1=3/3 reached, 0 partial, 0 missing, avg=1.00; 2=4/4 reached, 0 partial, 0 missing, avg=0.97; 3=8/8 reached, 0 partial, 0 missing, avg=1.00; 4=3/6 reached, 3 partial, 0 missing, avg=0.81; 5=3/8 reached, 5 partial, 0 missing, avg=0.76; 6=1/10 reached, 1 partial, 8 missing, avg=0.18

## Diagnosis rollup

| diagnosis | rows | missing | partial | timing | likely lever |
|:----------|-----:|--------:|--------:|-------:|:-------------|
| wrong-slice / granularity | 15 | 6 | 9 | 0 | walker granularity / wrong slice |
| no discovered candidate | 2 | 2 | 0 | 0 | walker coverage or predecessor-gated emit |
| timing-only | 18 | 0 | 0 | 18 | usually no code change |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=33, no discovered candidate=2

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | aligned | none | 1 |
| scheduled bbox | aligned | low | 1 |
| scheduled bbox | aligned | high | 2 |
| scheduled bbox | early | low | 2 |
| scheduled bbox | early | high | 1 |
| scheduled bbox | late | low | 5 |
| scheduled bbox | late | high | 4 |
| scheduled bbox | late | full | 2 |
| scheduled bbox | missing | low | 6 |
| scheduled bbox | partial | none | 1 |
| scheduled bbox | partial | low | 8 |

## Arrival ledger by diagnosis

### wrong-slice / granularity

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 4.2 | 1908 | — | — | 0.79 | partial | README Append usage example | [scheduled bbox exact=15/19] README.md section #3 (t=5627, 15 atoms) |
| 4.3 | 2416 | — | — | 0.80 | partial | README errors.Is / errors.As / Unwrap stdlib-compat snippets | [scheduled bbox exact=35/44] README.md section #3 (t=5627, 35 atoms) |
| 4.4 | 2690 | — | — | 0.73 | partial | README ErrorFormat + ErrorOrNil examples | [scheduled bbox exact=22/30] README.md section #3 (t=5627, 58 atoms) |
| 5.2 | 4319 | — | — | 0.72 | partial | Error.Unwrap body + chain methods | [scheduled bbox exact=12/39] go decl body at multierror.go:71 (t=2911, 12 atoms) |
| 5.3 | 4466 | — | — | 0.76 | partial | Group.Go and Group.Wait bodies | [scheduled bbox exact=9/17] go decl body at group.go:20 (t=2428, 9 atoms) |
| 5.6 | 5019 | — | — | 0.77 | partial | ListFormatFunc body | [scheduled bbox exact=10/13] go decl body at format.go:17 (t=2592, 10 atoms) |
| 5.7 | 5197 | — | — | 0.74 | partial | Error.Error / ErrorOrNil / WrappedErrors / GoString bodies | [scheduled bbox exact=0/23] go decl doc at multierror.go:53 (t=2039, 7 atoms) |
| 5.8 | 5271 | — | — | 0.56 | partial | sort.Interface bodies | [scheduled bbox exact=2/9] go decl names surface in sort.go (t=422, 5 atoms) |
| 6.2 | 5622 | — | — | 0.01 | missing | Format expected-output strings from tests | [scheduled bbox exact=2/15] go test names surface in format_test.go (t=5742, 3 atoms) |
| 6.3 | 5988 | — | — | 0.03 | missing | TestErrorUnwrap — chain semantics in action | [scheduled bbox exact=2/35] go test names surface in multierror_test.go (t=6038, 2 atoms) |
| 6.4 | 6733 | — | — | 0.07 | missing | TestAppend bodies — Append edge cases | [scheduled bbox exact=11/74] go test names surface in append_test.go (t=5904, 11 atoms) |
| 6.5 | 7458 | — | — | 0.00 | missing | TestFlatten + TestGroup bodies | [scheduled bbox exact=2/70] go test names surface in flatten_test.go (t=5674, 3 atoms) |
| 6.6 | 8394 | — | — | 0.02 | missing | TestErrorIs + TestErrorAs bodies | [scheduled bbox exact=4/94] go test names surface in multierror_test.go (t=6038, 4 atoms) |
| 6.7 | 8988 | — | — | 0.00 | missing | Remaining test bodies (sort + prefix) | [scheduled bbox exact=3/67] go test names surface in prefix_test.go (t=5794, 5 atoms) |
| 6.10 | 9864 | — | — | 0.62 | partial | Boilerplate metadata | [scheduled bbox exact=8/13] headings outline in CHANGELOG.md (t=280, 8 atoms) |

### no discovered candidate

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 6.8 | 9527 | — | — | 0.00 | missing | .github/ listing + workflow file structure | no discovered line candidate |
| 6.9 | 9759 | — | — | 0.00 | missing | Makefile targets | no discovered line candidate |

### timing-only

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 181 | 612 | +431 | 1.00 | late | README lede + deprecation note | [scheduled bbox exact=2/2] README headline in README.md (t=612, 4 atoms) |
| 1.3 | 252 | 1400 | +1148 | 1.00 | late | Error struct definition | [scheduled bbox exact=4/6] go decl at multierror.go:13 (t=1329, 4 atoms) |
| 2.1 | 337 | 963 | +626 | 0.90 | late | Package + import declarations across all .go files | [scheduled bbox exact=2/10] go module file go.mod (t=102, 2 atoms) |
| 2.2 | 400 | 1306 | +906 | 1.00 | late | All public function/method first-lines (multierror.go) | [scheduled bbox exact=10/10] go decl names surface in multierror.go (t=1306, 19 atoms) |
| 2.3 | 564 | 422 | -142 | 1.00 | aligned+over | Public function names across single-fn files | [scheduled bbox exact=0/19] go decl body at group.go:20 (t=2428, 9 atoms) |
| 3.1 | 725 | 1921 | +1196 | 1.00 | late | Append doc comment + signature | [scheduled bbox exact=8/9] go decl doc at append.go:14 (t=1921, 8 atoms) |
| 3.2 | 909 | 2339 | +1430 | 1.00 | late | Error.Unwrap doc + signature | [scheduled bbox exact=11/12] go decl doc at multierror.go:71 (t=2339, 11 atoms) |
| 3.4 | 1140 | 1722 | +582 | 1.00 | late | Flatten + Prefix doc + signatures | [scheduled bbox exact=6/10] go decl doc at prefix.go:16 (t=1722, 6 atoms) |
| 3.6 | 1331 | 2039 | +708 | 1.00 | late | Error.Error + WrappedErrors + GoString docs | [scheduled bbox exact=7/10] go decl doc at multierror.go:53 (t=2039, 7 atoms) |
| 3.7 | 1419 | 463 | -956 | 1.00 | early | sort.Interface methods on Error | [scheduled bbox exact=4/7] go decl names surface in sort.go (t=422, 6 atoms) |
| 3.8 | 1596 | 2769 | +1173 | 1.00 | late | chain type explainer comment + decl | [scheduled bbox exact=11/12] go decl doc at multierror.go:99 (t=2769, 11 atoms) |
| 4.1 | 1718 | 5627 | +3909 | 0.80 | late | README usage section overview lines | [scheduled bbox exact=8/10] README.md section #3 (t=5627, 66 atoms) |
| 4.5 | 3075 | 4648 | +1573 | 0.86 | late | README intro paragraph (unwrap + Go-version notes) | [scheduled bbox exact=11/22] README.md section #1 (t=4648, 11 atoms) |
| 4.6 | 3611 | 4648 | +1037 | 0.91 | aligned | README migration to errors.Join — basic + Group sections | [scheduled bbox exact=49/54] README.md section #1 (t=4648, 64 atoms) |
| 5.1 | 3936 | 3750 | -186 | 0.88 | aligned | Append body | [scheduled bbox exact=28/32] go decl body at append.go:14 (t=3750, 28 atoms) |
| 5.4 | 4669 | 2990 | -1679 | 0.81 | early | Flatten body (Flatten + flatten recursion) | [scheduled bbox exact=8/21] go decl body at flatten.go:20 (t=2990, 8 atoms) |
| 5.5 | 4872 | 3171 | -1701 | 0.81 | early | Prefix body | [scheduled bbox exact=17/21] go decl body at prefix.go:16 (t=3171, 17 atoms) |
| 6.1 | 5514 | 6038 | +524 | 1.00 | aligned+over | Test function name inventory across all _test.go files | [scheduled bbox exact=8/24] go test names surface in multierror_test.go (t=6038, 15 atoms) |

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

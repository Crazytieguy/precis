scores: Score(3000)=0.743 ns_rows≤3K=19/39 (reached=15 partial=0 missing=4)

## Per-budget scores

| B | A_B | I(B) | C(B) | Score(B) | walker_used |
|--:|----:|-----:|-----:|---------:|------------:|
| 1000 | 94 | 0.892 | 0.596 | 0.729 | 996 |
| 1442 | 136 | 0.926 | 0.708 | 0.809 | 1438 |
| 2080 | 177 | 0.939 | 0.695 | 0.808 | 2072 |
| 3000 | 251 | 0.944 | 0.585 | 0.743 | 2990 |
| 4327 | 398 | 0.897 | 0.516 | 0.680 | 3750 |
| 6240 | 576 | 0.947 | 0.783 | 0.862 | 6038 |
| 9000 | 881 | 0.893 | 0.513 | 0.677 | 6038 |

## Verdict

Verdict: wrong-slice bound
Likely primary lever: split walker batches to match NS semantic slices
Evidence: 0 ranking-recoverable (gap@3k=0.00), 17 wrong-slice/granularity (gap@3k=1.19), 2 no-discovered (gap@3k=0.08)
Secondary intervention: investigate 2 no-discovered rows
Top rows: 4.3, 4.4, 6.6, 6.4, 4.2, ...

## Top opportunities

_`gap@B` is a non-additive priority score: `Σ over atoms in row: (1 − damped_credit(a)) / rank(a)` evaluated at budget B's walker state. Approximates how much closing the row would lift `Score(B)` (via the Importance numerator); not an exact delta. `gap@3k` is the primary sort key. `gap@1k` and `gap@9k` show how the same intervention scales across the budget vector — gap is monotone non-increasing in B (walker has more budget at higher B). Rows can overlap between opportunities; sums are upper bounds on Score(B) impact, not additive estimates._

| intervention | rows | gap@1k | gap@3k | gap@9k | evidence | top row ids |
|:-------------|-----:|-------:|-------:|-------:|:---------|:------------|
| split wrong-slice walker batches | 17 | 1.36 | 1.19 | 0.71 | nearby candidates have low exact atom overlap | 4.3, 4.4, 6.6, 6.4, 4.2, ... |
| add walker candidates for no-discovered rows | 2 | 0.08 | 0.08 | 0.08 | NS rows have no discovered line candidate | 6.8, 6.9 |
| finish partially-delivered NS batches | 1 | 0.01 | 0.01 | 0.01 | avg batch completion=0.34 | 6.10 |

## Diagnosis rollup

| diagnosis | rows | missing | partial | likely lever |
|:----------|-----:|--------:|--------:|:-------------|
| wrong-slice / granularity | 17 | 12 | 5 | walker granularity / wrong slice |
| no discovered candidate | 2 | 2 | 0 | walker coverage or predecessor-gated emit |
| mixed/unknown | 4 | 4 | 0 | inspect row |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=21, no discovered candidate=2

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | missing | low | 12 |
| scheduled bbox | missing | high | 4 |
| scheduled bbox | partial | none | 1 |
| scheduled bbox | partial | low | 4 |

## Arrival ledger by diagnosis

### wrong-slice / granularity

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 4.2 | 1908 | 0.00 | 0.00 | missing | README Append usage example | [scheduled bbox exact=15/19] README.md section #3 (t=5627, 15 atoms) |
| 4.3 | 2416 | 0.00 | 0.00 | missing | README errors.Is / errors.As / Unwrap stdlib-compat snippets | [scheduled bbox exact=35/44] README.md section #3 (t=5627, 35 atoms) |
| 4.4 | 2690 | 0.00 | 0.00 | missing | README ErrorFormat + ErrorOrNil examples | [scheduled bbox exact=22/30] README.md section #3 (t=5627, 58 atoms) |
| 4.5 | 3075 | 0.09 | 0.04 | missing | README intro paragraph (unwrap + Go-version notes) | [scheduled bbox exact=11/22] README.md section #1 (t=4648, 11 atoms) |
| 5.2 | 4319 | 0.72 | 0.99 | partial | Error.Unwrap body + chain methods | [scheduled bbox exact=12/39] go decl body at multierror.go:71 (t=2911, 12 atoms) |
| 5.3 | 4466 | 0.76 | 0.98 | partial | Group.Go and Group.Wait bodies | [scheduled bbox exact=9/17] go decl body at group.go:20 (t=2428, 9 atoms) |
| 5.6 | 5019 | 0.77 | 0.99 | partial | ListFormatFunc body | [scheduled bbox exact=10/13] go decl body at format.go:17 (t=2592, 10 atoms) |
| 5.7 | 5197 | 0.74 | 0.97 | partial | Error.Error / ErrorOrNil / WrappedErrors / GoString bodies | [scheduled bbox exact=0/23] go decl doc at multierror.go:53 (t=2039, 7 atoms) |
| 5.8 | 5271 | 0.56 | 0.97 | partial | sort.Interface bodies | [scheduled bbox exact=2/9] go decl names surface in sort.go (t=422, 5 atoms) |
| 6.1 | 5514 | 0.00 | 0.00 | missing | Test function name inventory across all _test.go files | [scheduled bbox exact=8/24] go test names surface in multierror_test.go (t=6038, 15 atoms) |
| 6.2 | 5622 | 0.00 | 0.00 | missing | Format expected-output strings from tests | [scheduled bbox exact=2/15] go test names surface in format_test.go (t=5742, 3 atoms) |
| 6.3 | 5988 | 0.00 | 0.00 | missing | TestErrorUnwrap — chain semantics in action | [scheduled bbox exact=2/35] go test names surface in multierror_test.go (t=6038, 2 atoms) |
| 6.4 | 6733 | 0.00 | 0.00 | missing | TestAppend bodies — Append edge cases | [scheduled bbox exact=11/74] go test names surface in append_test.go (t=5904, 11 atoms) |
| 6.5 | 7458 | 0.00 | 0.00 | missing | TestFlatten + TestGroup bodies | [scheduled bbox exact=2/70] go test names surface in flatten_test.go (t=5674, 3 atoms) |
| 6.6 | 8394 | 0.00 | 0.00 | missing | TestErrorIs + TestErrorAs bodies | [scheduled bbox exact=4/94] go test names surface in multierror_test.go (t=6038, 4 atoms) |
| 6.7 | 8988 | 0.00 | 0.00 | missing | Remaining test bodies (sort + prefix) | [scheduled bbox exact=3/67] go test names surface in prefix_test.go (t=5794, 5 atoms) |
| 6.10 | 9864 | 0.62 | 0.34 | missing | Boilerplate metadata | [scheduled bbox exact=8/13] headings outline in CHANGELOG.md (t=280, 8 atoms) |

### no discovered candidate

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 6.8 | 9527 | 0.00 | 0.00 | missing | .github/ listing + workflow file structure | no discovered line candidate |
| 6.9 | 9759 | 0.00 | 0.00 | missing | Makefile targets | no discovered line candidate |

### mixed/unknown

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 4.1 | 1718 | 0.00 | 0.00 | missing | README usage section overview lines | [scheduled bbox exact=8/10] README.md section #3 (t=5627, 66 atoms) |
| 4.6 | 3611 | 0.00 | 0.00 | missing | README migration to errors.Join — basic + Group sections | [scheduled bbox exact=49/54] README.md section #1 (t=4648, 64 atoms) |
| 5.1 | 3936 | 0.00 | 0.00 | missing | Append body | [scheduled bbox exact=28/32] go decl body at append.go:14 (t=3750, 28 atoms) |
| 5.5 | 4872 | 0.00 | 0.00 | missing | Prefix body | [scheduled bbox exact=17/21] go decl body at prefix.go:16 (t=3171, 17 atoms) |

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

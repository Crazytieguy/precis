scores: Score(3000)=0.469 ns_rows≤3K=14/39 (reached=5 partial=0 missing=9)

## Per-budget scores

| B | A_B | I(B) | C(B) | Score(B) | walker_used |
|--:|----:|-----:|-----:|---------:|------------:|
| 1000 | 96 | 0.842 | 0.489 | 0.642 | 988 |
| 1442 | 126 | 0.812 | 0.463 | 0.613 | 1400 |
| 2080 | 154 | 0.791 | 0.461 | 0.603 | 2071 |
| 3000 | 252 | 0.736 | 0.298 | 0.469 | 2992 |
| 4327 | 365 | 0.722 | 0.266 | 0.438 | 4319 |
| 6240 | 538 | 0.762 | 0.400 | 0.552 | 6135 |
| 9000 | 797 | 0.750 | 0.387 | 0.539 | 8729 |

## Verdict

Verdict: wrong-slice bound
Likely primary lever: split walker batches to match NS semantic slices
Evidence: 1 ranking-recoverable (gap@3k=0.08), 22 wrong-slice/granularity (gap@3k=1.62), 9 no-discovered (gap@3k=0.76)
Secondary intervention: promote predecessors for 1 gated candidate
Top rows: 2.2, 2.3, 1.10, 2.6, 2.5, ...

## Top opportunities

_`gap@B` is a non-additive priority score: `Σ over atoms in row: (1 − damped_credit(a)) / rank(a)` evaluated at budget B's walker state. Approximates how much closing the row would lift `Score(B)` (via the Importance numerator); not an exact delta. `gap@3k` is the primary sort key. `gap@1k` and `gap@9k` show how the same intervention scales across the budget vector — gap is monotone non-increasing in B (walker has more budget at higher B). Rows can overlap between opportunities; sums are upper bounds on Score(B) impact, not additive estimates._

| intervention | rows | gap@1k | gap@3k | gap@9k | evidence | top row ids |
|:-------------|-----:|-------:|-------:|-------:|:---------|:------------|
| split wrong-slice walker batches | 22 | 1.78 | 1.62 | 1.10 | nearby candidates have low exact atom overlap | 2.2, 2.3, 1.10, 2.6, 2.5, ... |
| add walker candidates for no-discovered rows | 9 | 0.76 | 0.76 | 0.76 | NS rows have no discovered line candidate | 1.6, 1.7, 1.5, 3.8, 4.2, ... |
| promote python decl names surface #2 in src/typeguard/_checkers.py | 1 | 0.08 | 0.08 | 0.08 | 0 files, exact total=41/45 | 3.2 |

## Diagnosis rollup

| diagnosis | rows | missing | partial | likely lever |
|:----------|-----:|--------:|--------:|:-------------|
| ranking-recoverable | 1 | 1 | 0 | value/ranking |
| wrong-slice / granularity | 22 | 20 | 2 | walker granularity / wrong slice |
| no discovered candidate | 9 | 9 | 0 | walker coverage or predecessor-gated emit |
| fs/listing | 1 | 1 | 0 | filesystem/listing value |
| mixed/unknown | 1 | 1 | 0 | inspect row |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | gap@3k | likely lever |
|:------------|-----:|-------:|:-------------|
| predecessor not scheduled | 1 | 0.08 | promote predecessor |

Candidate hint kinds: scheduled bbox=19, unscheduled bbox=5, fs-only=1, no discovered candidate=9 _(candidates are walker batches discovered this run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` isn't proof that no emit path exists)._

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | missing | low | 16 |
| scheduled bbox | missing | high | 1 |
| scheduled bbox | partial | low | 2 |
| unscheduled bbox | missing | none | 1 |
| unscheduled bbox | missing | low | 3 |
| unscheduled bbox | missing | high | 1 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 3.2 | 6639 | 0.00 | 0.00 | missing | _checkers.py — origin_type_checkers dispatch table | [unscheduled bbox exact=41/45] python decl at src/typeguard/_checkers.py:1005 (41 atoms, predecessor not scheduled: python decl names surface #2 in src/typeguard/_checkers.py) |

### wrong-slice / granularity

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 1.10 | 1351 | 0.29 | 0.39 | missing | typeguard/__init__.py — module rewrite, lazy `config`, autoload | [scheduled bbox exact=4/21] python decl body at src/typeguard/__init__.py:37 body 38 (t=1177, 4 atoms) |
| 2.1 | 1663 | 0.61 | 0.74 | missing | _exceptions.py — class signatures + summary docstrings | [scheduled bbox exact=5/28] python method sigs in src/typeguard/_exceptions.py (t=1659, 10 atoms) |
| 2.2 | 2249 | 0.03 | 0.01 | missing | check_type — primary entry-point signature | [scheduled bbox exact=12/42] python decl at src/typeguard/_functions.py:50 (t=4556, 12 atoms); better unscheduled exact=24/42: python decl doc at src/typeguard/_functions.py:50 (24 atoms, too expensive at final margin) |
| 2.3 | 2766 | 0.29 | 0.28 | missing | @typechecked — overloaded signatures | [scheduled bbox exact=8/38] python decl at src/typeguard/_decorators.py:150 (t=3532, 8 atoms); better unscheduled exact=12/38: python decl doc at src/typeguard/_decorators.py:150 (12 atoms, too expensive at final margin) |
| 2.4 | 2985 | 0.28 | 0.24 | missing | install_import_hook — signature + docstring | [scheduled bbox exact=9/18] python decl doc at src/typeguard/_importhook.py:183 (t=6282, 9 atoms) |
| 2.5 | 3337 | 0.22 | 0.27 | missing | suppress_type_checks — overloaded signatures + docstring | [scheduled bbox exact=14/32] python decl doc at src/typeguard/_suppression.py:30 (t=8445, 14 atoms) |
| 2.6 | 3817 | 0.20 | 0.27 | missing | TypeCheckConfiguration — dataclass + attribute docstrings | [scheduled bbox exact=6/45] python class body at src/typeguard/_config.py:62 (t=2199, 6 atoms); better unscheduled exact=21/45: python decl doc at src/typeguard/_config.py:62 (21 atoms, too expensive at final margin) |
| 2.7 | 4180 | 0.23 | 0.24 | missing | ForwardRefPolicy + CollectionCheckStrategy enums | [scheduled bbox exact=6/36] python decl doc at src/typeguard/_config.py:30 (t=5833, 13 atoms); better unscheduled exact=7/36: python method body at src/typeguard/_config.py:52 body 53 (7 atoms, discovered unscheduled) |
| 2.8 | 4402 | 0.27 | 0.41 | missing | TypeCheckMemo — class skeleton + __init__ | [scheduled bbox exact=1/23] python decl doc at src/typeguard/_memo.py:8 (t=7877, 17 atoms) |
| 2.9 | 4573 | 0.00 | 0.00 | missing | Checker plug-in API — TypeCheckerCallable / TypeCheckLookupCallback / checker_lookup_functions | [scheduled bbox exact=5/16] python decl at src/typeguard/_checkers.py:89 (t=5270, 5 atoms) |
| 2.10 | 4926 | 0.17 | 0.16 | missing | TypeguardFinder + ImportHookManager — class + key methods | [scheduled bbox exact=6/31] python method sigs in src/typeguard/_importhook.py (t=3775, 14 atoms) |
| 2.11 | 5177 | 0.06 | 0.09 | missing | warn_on_error + load_plugins — signatures | [scheduled bbox exact=5/19] python decl doc at src/typeguard/_functions.py:291 (t=3430, 5 atoms); better unscheduled exact=7/19: python decl doc at src/typeguard/_checkers.py:1099 (7 atoms, predecessor not scheduled: python decl at src/typeguard/_checkers.py:1099) |
| 2.12 | 5358 | 0.00 | 0.00 | missing | check_type_internal — signature + docstring | [unscheduled bbox exact=9/16] python decl doc at src/typeguard/_checkers.py:924 (9 atoms, predecessor not scheduled: python decl at src/typeguard/_checkers.py:924) |
| 2.13 | 5642 | 0.33 | 0.39 | missing | Internal check_argument_types / check_return_type / check_yield_type / check_send_type / check_variable_assignment — signatures | [scheduled bbox exact=10/28] python decl names surface in src/typeguard/_functions.py (t=2948, 10 atoms) |
| 2.14 | 5813 | 0.80 | 0.99 | partial | Unset sentinel + small _utils helpers | [scheduled bbox exact=9/15] python decl names surface in src/typeguard/_utils.py (t=2434, 15 atoms) |
| 3.1 | 6041 | 0.00 | 0.00 | missing | _checkers.py — every check_* function name (locations) | [scheduled bbox exact=11/25] python decl names surface #1 in src/typeguard/_checkers.py (t=6657, 24 atoms) |
| 3.3 | 7082 | 0.00 | 0.00 | missing | _checkers.py — builtin_checker_lookup dispatch fallbacks | [unscheduled bbox exact=27/36] python decl body at src/typeguard/_checkers.py:1061 body 1065 (27 atoms, predecessor not scheduled: python decl at src/typeguard/_checkers.py:1061) |
| 3.5 | 7354 | 0.00 | 0.00 | missing | _transformer.py — TypeguardTransformer visit_* methods (locations) | [scheduled bbox exact=14/20] python method sigs #1 in src/typeguard/_transformer.py (t=9120, 41 atoms) |
| 3.6 | 7639 | 0.00 | 0.00 | missing | _transformer.py — generator_names + annotated_names + ignore_decorators tables | [scheduled bbox exact=14/30] python decl at src/typeguard/_transformer.py:70 (t=6135, 14 atoms) |
| 3.7 | 7673 | 0.75 | 0.68 | partial | _decorators.py — function locations | [scheduled bbox exact=3/4] python decl names surface in src/typeguard/_decorators.py (t=1791, 5 atoms) |
| 4.4 | 9827 | 0.00 | 0.00 | missing | tests/test_checkers.py — TestX class locations | [unscheduled bbox exact=0/28] python test names surface in tests/test_checkers.py (298 atoms, too expensive at final margin) |
| 4.5 | 9976 | 0.00 | 0.00 | missing | tests/test_typechecked.py — Test class + module test locations | [unscheduled bbox exact=9/14] python test names surface in tests/test_typechecked.py (103 atoms, too expensive at final margin) |

### no discovered candidate

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 1.5 | 349 | 0.00 | 0.00 | missing | README — one-paragraph lede | no discovered line candidate |
| 1.6 | 550 | 0.00 | 0.00 | missing | README — check_type vs. code instrumentation modes | no discovered line candidate |
| 1.7 | 700 | 0.00 | 0.00 | missing | README — instrumentation options (@typechecked vs import hook) | no discovered line candidate |
| 3.8 | 8036 | 0.00 | 0.00 | missing | docs/api.rst — public API by topic group (head) | no discovered line candidate |
| 3.9 | 8213 | 0.00 | 0.00 | missing | docs/userguide.rst — H2 section locations | no discovered line candidate |
| 3.10 | 8392 | 0.00 | 0.00 | missing | docs/features.rst — H2 section locations | no discovered line candidate |
| 4.1 | 8692 | 0.00 | 0.00 | missing | docs/api.rst — Custom checkers / Suppression / Exceptions sections | no discovered line candidate |
| 4.2 | 9266 | 0.00 | 0.00 | missing | docs/extending.rst — writing a lookup + checker function (head) | no discovered line candidate |
| 4.3 | 9578 | 0.00 | 0.00 | missing | docs/extending.rst — MySpecialType worked example | no discovered line candidate |

### fs/listing

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 1.4 | 247 | 0.00 | 0.00 | missing | tests/ listing | fs-only |

### mixed/unknown

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 3.4 | 7148 | 0.17 | 0.19 | missing | _transformer.py — class locations | [scheduled bbox exact=5/6] python decl names surface in src/typeguard/_transformer.py (t=4778, 8 atoms) |

## Walker waste, primary-actionable (first_t ≤ 3000, off-NS spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 126 | 1.00 | 126 | 1526 | python decl names surface in docs/conf.py |
| 101 | 1.00 | 101 | 2635 | python decl names surface #1 in docs/conf.py |
| 76 | 0.67 | 113 | 1977 | python decl names surface in src/typeguard/_importhook.py |
| 69 | 1.00 | 69 | 2802 | python decl doc at src/typeguard/_utils.py:104 |
| 61 | 1.00 | 61 | 2534 | python decl doc at src/typeguard/_utils.py:127 |
| 60 | 1.00 | 60 | 2695 | python decl at docs/conf.py:28 |
| 55 | 0.38 | 146 | 2948 | python decl names surface in src/typeguard/_functions.py |

## Walker waste, late (first_t > 3000, off-NS spend ≥ 50) — higher-budget calibration only

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 215 | 0.98 | 219 | 7877 | python decl doc at src/typeguard/_memo.py:8 |
| 201 | 1.00 | 201 | 9852 | python imports in src/typeguard/_decorators.py |
| 185 | 1.00 | 185 | 8630 | python imports in src/typeguard/_importhook.py |
| 176 | 1.00 | 176 | 8144 | python imports in src/typeguard/_functions.py |
| 160 | 1.00 | 160 | 7500 | python imports in src/typeguard/_utils.py |
| 158 | 1.00 | 158 | 7658 | python decl at src/typeguard/_transformer.py:100 |
| 137 | 0.35 | 391 | 9120 | python method sigs #1 in src/typeguard/_transformer.py |
| 127 | 1.00 | 127 | 5960 | python imports in src/typeguard/_pytest_plugin.py |
| 122 | 1.00 | 122 | 6480 | plaintext config .gitignore |
| 112 | 1.00 | 112 | 5043 | python class body at src/typeguard/_transformer.py:338 |
| 1360 | — | — | — | +19 more rows |

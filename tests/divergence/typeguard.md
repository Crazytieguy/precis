scores: Score(3000)=0.461 ns_rows≤3K=14/39 (reached=5 partial=1 missing=8)

## Per-budget scores

| B | A_B | I(B) | C(B) | Score(B) | walker_used |
|--:|----:|-----:|-----:|---------:|------------:|
| 1000 | 96 | 0.788 | 0.323 | 0.504 | 676 |
| 1442 | 126 | 0.806 | 0.463 | 0.611 | 1321 |
| 2080 | 154 | 0.785 | 0.432 | 0.582 | 1957 |
| 3000 | 252 | 0.741 | 0.287 | 0.461 | 2926 |
| 4327 | 365 | 0.711 | 0.223 | 0.399 | 4238 |
| 6240 | 538 | 0.697 | 0.373 | 0.510 | 6201 |
| 9000 | 797 | 0.698 | 0.389 | 0.521 | 8968 |

## Verdict

Verdict: wrong-slice bound
Likely primary lever: split walker batches to match NS semantic slices
Evidence: 1 ranking-recoverable (gap@3k=0.00), 18 wrong-slice/granularity (gap@3k=0.75), 11 no-discovered (gap@3k=0.52)
Secondary intervention: free T_max budget for 1 too-expensive candidate
Top rows: 2.2, 2.3, 1.10, 2.1, 2.4, ...

## Top opportunities

_`gap@B` is a non-additive priority score: `Σ over atoms with rank ≤ |A_B|: (1 − damped_credit(a)) / rank(a)`. `gap@3k` is the primary sort key — direct proxy for `Score(3000)` headroom. `gap@1k` and `gap@9k` show how the same intervention scales across the budget vector. Rows can overlap between opportunities; sums are upper bounds on Score(B) impact, not additive estimates._

| intervention | rows | gap@1k | gap@3k | gap@9k | evidence | top row ids |
|:-------------|-----:|-------:|-------:|-------:|:---------|:------------|
| split wrong-slice walker batches | 18 | 0.00 | 0.75 | 1.01 | nearby candidates have low exact atom overlap | 2.2, 2.3, 1.10, 2.1, 2.4, ... |
| add walker candidates for no-discovered rows | 11 | 0.52 | 0.52 | 0.68 | NS rows have no discovered line candidate | 1.6, 1.7, 1.5, 3.8, 3.9, ... |
| finish partially-delivered NS batches | 7 | 0.00 | 0.34 | 0.33 | avg batch completion=0.43 | 1.10, 2.1, 2.4, 2.5, 2.7, ... |

## Diagnosis rollup

| diagnosis | rows | missing | partial | likely lever |
|:----------|-----:|--------:|--------:|:-------------|
| ranking-recoverable | 1 | 1 | 0 | value/ranking |
| wrong-slice / granularity | 18 | 16 | 2 | walker granularity / wrong slice |
| no discovered candidate | 11 | 11 | 0 | walker coverage or predecessor-gated emit |
| fs/listing | 1 | 1 | 0 | filesystem/listing value |
| mixed/unknown | 1 | 1 | 0 | inspect row |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | gap@3k | likely lever |
|:------------|-----:|-------:|:-------------|
| too expensive at final margin | 1 | 0.00 | free T_max budget |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=20, fs-only=1, no discovered candidate=11

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | missing | low | 17 |
| scheduled bbox | missing | high | 1 |
| scheduled bbox | partial | low | 2 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 3.2 | 6639 | 0.00 | 0.00 | missing | _checkers.py — origin_type_checkers dispatch table | [scheduled bbox exact=1/45] python decl names surface #2 in src/typeguard/_checkers.py (t=8753, 1 atoms); better unscheduled exact=41/45: python decl at src/typeguard/_checkers.py:1005 (41 atoms, too expensive at final margin) |

### wrong-slice / granularity

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 1.10 | 1351 | 0.29 | 0.39 | missing | typeguard/__init__.py — module rewrite, lazy `config`, autoload | [scheduled bbox exact=4/21] python decl body at src/typeguard/__init__.py:37 body 38 (t=1177, 4 atoms) |
| 2.1 | 1663 | 0.61 | 0.74 | partial | _exceptions.py — class signatures + summary docstrings | [scheduled bbox exact=5/28] python method sigs in src/typeguard/_exceptions.py (t=2212, 10 atoms) |
| 2.2 | 2249 | 0.03 | 0.01 | missing | check_type — primary entry-point signature | [scheduled bbox exact=12/42] python decl at src/typeguard/_functions.py:50 (t=5362, 12 atoms); better unscheduled exact=24/42: python decl doc at src/typeguard/_functions.py:50 (24 atoms, too expensive at final margin) |
| 2.3 | 2766 | 0.11 | 0.07 | missing | @typechecked — overloaded signatures | [scheduled bbox exact=8/38] python decl at src/typeguard/_decorators.py:150 (t=4340, 8 atoms); better unscheduled exact=12/38: python decl doc at src/typeguard/_decorators.py:150 (12 atoms, too expensive at final margin) |
| 2.4 | 2985 | 0.28 | 0.24 | missing | install_import_hook — signature + docstring | [scheduled bbox exact=9/18] python decl doc at src/typeguard/_importhook.py:183 (t=6985, 9 atoms) |
| 2.5 | 3337 | 0.22 | 0.27 | missing | suppress_type_checks — overloaded signatures + docstring | [scheduled bbox exact=14/32] python decl doc at src/typeguard/_suppression.py:30 (t=8331, 14 atoms) |
| 2.6 | 3817 | 0.07 | 0.06 | missing | TypeCheckConfiguration — dataclass + attribute docstrings | [scheduled bbox exact=6/45] python class body at src/typeguard/_config.py:62 (t=3100, 6 atoms); better unscheduled exact=21/45: python decl doc at src/typeguard/_config.py:62 (21 atoms, too expensive at final margin) |
| 2.7 | 4180 | 0.23 | 0.24 | missing | ForwardRefPolicy + CollectionCheckStrategy enums | [scheduled bbox exact=6/36] python decl doc at src/typeguard/_config.py:30 (t=6536, 13 atoms); better unscheduled exact=7/36: python method body at src/typeguard/_config.py:52 body 53 (7 atoms, too expensive at final margin) |
| 2.8 | 4402 | 0.27 | 0.41 | missing | TypeCheckMemo — class skeleton + __init__ | [scheduled bbox exact=1/23] python decl doc at src/typeguard/_memo.py:8 (t=7763, 17 atoms) |
| 2.9 | 4573 | 0.00 | 0.00 | missing | Checker plug-in API — TypeCheckerCallable / TypeCheckLookupCallback / checker_lookup_functions | [scheduled bbox exact=5/16] python decl at src/typeguard/_checkers.py:89 (t=3881, 5 atoms) |
| 2.10 | 4926 | 0.17 | 0.16 | missing | TypeguardFinder + ImportHookManager — class + key methods | [scheduled bbox exact=6/31] python method sigs in src/typeguard/_importhook.py (t=4695, 14 atoms) |
| 2.11 | 5177 | 0.11 | 0.20 | missing | warn_on_error + load_plugins — signatures | [scheduled bbox exact=7/19] python decl doc at src/typeguard/_checkers.py:1099 (t=9274, 7 atoms) |
| 2.12 | 5358 | 0.00 | 0.00 | missing | check_type_internal — signature + docstring | [scheduled bbox exact=9/16] python decl doc at src/typeguard/_checkers.py:924 (t=9399, 9 atoms) |
| 3.1 | 6041 | 0.00 | 0.00 | missing | _checkers.py — every check_* function name (locations) | [scheduled bbox exact=11/25] python decl names surface #1 in src/typeguard/_checkers.py (t=5698, 24 atoms) |
| 3.3 | 7082 | 0.00 | 0.00 | missing | _checkers.py — builtin_checker_lookup dispatch fallbacks | [scheduled bbox exact=3/36] python decl at src/typeguard/_checkers.py:1061 (t=8824, 3 atoms); better unscheduled exact=27/36: python decl body at src/typeguard/_checkers.py:1061 body 1065 (27 atoms, too expensive at final margin) |
| 3.5 | 7354 | 0.00 | 0.00 | missing | _transformer.py — TypeguardTransformer visit_* methods (locations) | [scheduled bbox exact=14/20] python method sigs #1 in src/typeguard/_transformer.py (t=9882, 41 atoms) |
| 3.6 | 7639 | 0.00 | 0.00 | missing | _transformer.py — generator_names + annotated_names + ignore_decorators tables | [scheduled bbox exact=14/30] python decl at src/typeguard/_transformer.py:70 (t=6838, 14 atoms) |
| 3.7 | 7673 | 0.75 | 0.68 | partial | _decorators.py — function locations | [scheduled bbox exact=3/4] python decl names surface in src/typeguard/_decorators.py (t=1640, 5 atoms) |

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
| 4.4 | 9827 | 0.00 | 0.00 | missing | tests/test_checkers.py — TestX class locations | no discovered line candidate |
| 4.5 | 9976 | 0.00 | 0.00 | missing | tests/test_typechecked.py — Test class + module test locations | no discovered line candidate |

### fs/listing

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 1.4 | 247 | 0.00 | 0.00 | missing | tests/ listing | fs-only |

### mixed/unknown

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 3.4 | 7148 | 0.17 | 0.19 | missing | _transformer.py — class locations | [scheduled bbox exact=5/6] python decl names surface in src/typeguard/_transformer.py (t=3501, 8 atoms) |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 215 | 0.98 | 219 | 7763 | python decl doc at src/typeguard/_memo.py:8 |
| 185 | 1.00 | 185 | 8516 | python imports in src/typeguard/_importhook.py |
| 176 | 1.00 | 176 | 8030 | python imports in src/typeguard/_functions.py |
| 160 | 1.00 | 160 | 7386 | python imports in src/typeguard/_utils.py |
| 158 | 1.00 | 158 | 7544 | python decl at src/typeguard/_transformer.py:100 |
| 137 | 0.35 | 391 | 9882 | python method sigs #1 in src/typeguard/_transformer.py |
| 127 | 1.00 | 127 | 6663 | python imports in src/typeguard/_pytest_plugin.py |
| 126 | 1.00 | 126 | 1447 | python decl names surface in docs/conf.py |
| 122 | 1.00 | 122 | 7183 | plaintext config .gitignore |
| 112 | 1.00 | 112 | 4452 | python class body at src/typeguard/_transformer.py:338 |
| 1686 | — | — | — | +23 more rows |

scores: Score(3000)=0.469 ns_rows≤3K=14/39 (reached=5 partial=0 missing=9)

## Per-budget scores

| B | A_B | I(B) | C(B) | compl(B) | Score(B) | walker_used |
|--:|----:|-----:|-----:|---------:|---------:|------------:|
| 1000 | 96 | 0.842 | 0.489 | 1.000 | 0.642 | 988 |
| 1442 | 126 | 0.812 | 0.463 | 0.898 | 0.613 | 1400 |
| 2080 | 154 | 0.791 | 0.461 | 0.875 | 0.603 | 2071 |
| 3000 | 252 | 0.736 | 0.298 | 0.665 | 0.469 | 2992 |
| 4327 | 365 | 0.722 | 0.266 | 0.620 | 0.438 | 4319 |
| 6240 | 538 | 0.762 | 0.400 | 0.704 | 0.552 | 6135 |
| 9000 | 797 | 0.750 | 0.387 | 0.822 | 0.539 | 8729 |

## Top opportunities

### Additive (close partial / missing rows)

| intervention | rows | gap@1k | gap@3k | gap@9k | evidence | top row ids |
|:-------------|-----:|-------:|-------:|-------:|:---------|:------------|
| split wrong-slice walker batches | 22 | 1.78 | 1.62 | 1.10 | nearby candidates have low exact atom overlap | 2.2, 2.3, 1.10, 2.6, 2.5, ... |
| add walker candidates for no-discovered rows | 9 | 0.76 | 0.76 | 0.76 | NS rows have no discovered line candidate | 1.6, 1.7, 1.5, 3.8, 4.2, ... |
| promote python decl names surface #<n> in src/typeguard/_checkers.py | 1 | 0.08 | 0.08 | 0.08 | 0 files, exact total=41/45 | 3.2 |

### Subtractive (suppress consistently off-NS batches)

| pattern | batches | freed@1k | freed@3k | freed@9k | evidence | top batch ids |
|:--------|--------:|---------:|---------:|---------:|:---------|:--------------|
| python decl names surface in docs/conf.py | 1 | 0 | 126 | 126 | off_3k=126 | python decl names surface in docs/conf.py |
| python decl names surface #<n> in docs/conf.py | 1 | 0 | 101 | 101 | off_3k=101 | python decl names surface #1 in docs/conf.py |
| python decl names surface in src/typeguard/_importhook.py | 1 | 0 | 76 | 76 | off_3k=102 | python decl names surface in src/typeguard/_importhook.py |
| python decl doc at src/typeguard/_utils.py:104 | 1 | 0 | 69 | 69 | off_3k=69 | python decl doc at src/typeguard/_utils.py:104 |
| python decl doc at src/typeguard/_utils.py:127 | 1 | 0 | 61 | 61 | off_3k=61 | python decl doc at src/typeguard/_utils.py:127 |

Top missed paths (NS rows ≤ 3K): src/typeguard/_functions.py (1 row, 42 atoms), src/typeguard/_decorators.py (1 row, 38 atoms), README.rst (3 rows, 32 atoms), src/typeguard/_exceptions.py (1 row, 28 atoms), src/typeguard/__init__.py (1 row, 21 atoms), src/typeguard/_importhook.py (1 row, 18 atoms), tests (1 row, 16 atoms)

## Arrival ledger by diagnosis

### ranking-recoverable

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 3.2 | 6639 | 0.00 | missing | _checkers.py — origin_type_checkers dispatch table | [unscheduled bbox exact=41/45] python decl at src/typeguard/_checkers.py:1005 (41 atoms, predecessor not scheduled: python decl names surface #2 in src/typeguard/_checkers.py) |

### wrong-slice / granularity

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 1.10 | 1351 | 0.29 | missing | typeguard/__init__.py — module rewrite, lazy `config`, autoload | [scheduled bbox exact=4/21] python decl body at src/typeguard/__init__.py:37 body 38 (t=1177, 4 atoms) |
| 2.1 | 1663 | 0.61 | missing | _exceptions.py — class signatures + summary docstrings | [scheduled bbox exact=5/28] python method sigs in src/typeguard/_exceptions.py (t=1659, 10 atoms) |
| 2.2 | 2249 | 0.03 | missing | check_type — primary entry-point signature | [scheduled bbox exact=12/42] python decl at src/typeguard/_functions.py:50 (t=4556, 12 atoms); better unscheduled exact=24/42: python decl doc at src/typeguard/_functions.py:50 (24 atoms, too expensive at final margin) |
| 2.3 | 2766 | 0.29 | missing | @typechecked — overloaded signatures | [scheduled bbox exact=8/38] python decl at src/typeguard/_decorators.py:150 (t=3532, 8 atoms); better unscheduled exact=12/38: python decl doc at src/typeguard/_decorators.py:150 (12 atoms, too expensive at final margin) |
| 2.4 | 2985 | 0.28 | missing | install_import_hook — signature + docstring | [scheduled bbox exact=9/18] python decl doc at src/typeguard/_importhook.py:183 (t=6282, 9 atoms) |
| 2.5 | 3337 | 0.22 | missing | suppress_type_checks — overloaded signatures + docstring | [scheduled bbox exact=14/32] python decl doc at src/typeguard/_suppression.py:30 (t=8445, 14 atoms) |
| 2.6 | 3817 | 0.20 | missing | TypeCheckConfiguration — dataclass + attribute docstrings | [scheduled bbox exact=6/45] python class body at src/typeguard/_config.py:62 (t=2199, 6 atoms); better unscheduled exact=21/45: python decl doc at src/typeguard/_config.py:62 (21 atoms, too expensive at final margin) |
| 2.7 | 4180 | 0.23 | missing | ForwardRefPolicy + CollectionCheckStrategy enums | [scheduled bbox exact=6/36] python decl doc at src/typeguard/_config.py:30 (t=5833, 13 atoms); better unscheduled exact=7/36: python method body at src/typeguard/_config.py:52 body 53 (7 atoms, discovered unscheduled) |
| 2.8 | 4402 | 0.27 | missing | TypeCheckMemo — class skeleton + __init__ | [scheduled bbox exact=1/23] python decl doc at src/typeguard/_memo.py:8 (t=7877, 17 atoms) |
| 2.9 | 4573 | 0.00 | missing | Checker plug-in API — TypeCheckerCallable / TypeCheckLookupCallback / checker_lookup_functions | [scheduled bbox exact=5/16] python decl at src/typeguard/_checkers.py:89 (t=5270, 5 atoms) |
| 2.10 | 4926 | 0.17 | missing | TypeguardFinder + ImportHookManager — class + key methods | [scheduled bbox exact=6/31] python method sigs in src/typeguard/_importhook.py (t=3775, 14 atoms) |
| 2.11 | 5177 | 0.06 | missing | warn_on_error + load_plugins — signatures | [scheduled bbox exact=5/19] python decl doc at src/typeguard/_functions.py:291 (t=3430, 5 atoms); better unscheduled exact=7/19: python decl doc at src/typeguard/_checkers.py:1099 (7 atoms, predecessor not scheduled: python decl at src/typeguard/_checkers.py:1099) |
| 2.12 | 5358 | 0.00 | missing | check_type_internal — signature + docstring | [unscheduled bbox exact=9/16] python decl doc at src/typeguard/_checkers.py:924 (9 atoms, predecessor not scheduled: python decl at src/typeguard/_checkers.py:924) |
| 2.13 | 5642 | 0.33 | missing | Internal check_argument_types / check_return_type / check_yield_type / check_send_type / check_variable_assignment — signatures | [scheduled bbox exact=10/28] python decl names surface in src/typeguard/_functions.py (t=2948, 10 atoms) |
| 2.14 | 5813 | 0.80 | partial | Unset sentinel + small _utils helpers | [scheduled bbox exact=9/15] python decl names surface in src/typeguard/_utils.py (t=2434, 15 atoms) |
| 3.1 | 6041 | 0.00 | missing | _checkers.py — every check_* function name (locations) | [scheduled bbox exact=11/25] python decl names surface #1 in src/typeguard/_checkers.py (t=6657, 24 atoms) |
| 3.3 | 7082 | 0.00 | missing | _checkers.py — builtin_checker_lookup dispatch fallbacks | [unscheduled bbox exact=27/36] python decl body at src/typeguard/_checkers.py:1061 body 1065 (27 atoms, predecessor not scheduled: python decl at src/typeguard/_checkers.py:1061) |
| 3.5 | 7354 | 0.00 | missing | _transformer.py — TypeguardTransformer visit_* methods (locations) | [scheduled bbox exact=14/20] python method sigs #1 in src/typeguard/_transformer.py (t=9120, 41 atoms) |
| 3.6 | 7639 | 0.00 | missing | _transformer.py — generator_names + annotated_names + ignore_decorators tables | [scheduled bbox exact=14/30] python decl at src/typeguard/_transformer.py:70 (t=6135, 14 atoms) |
| 3.7 | 7673 | 0.75 | partial | _decorators.py — function locations | [scheduled bbox exact=3/4] python decl names surface in src/typeguard/_decorators.py (t=1791, 5 atoms) |
| 4.4 | 9827 | 0.00 | missing | tests/test_checkers.py — TestX class locations | [unscheduled bbox exact=0/28] python test names surface in tests/test_checkers.py (298 atoms, too expensive at final margin) |
| 4.5 | 9976 | 0.00 | missing | tests/test_typechecked.py — Test class + module test locations | [unscheduled bbox exact=9/14] python test names surface in tests/test_typechecked.py (103 atoms, too expensive at final margin) |

### no discovered candidate

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 1.5 | 349 | 0.00 | missing | README — one-paragraph lede | no discovered line candidate |
| 1.6 | 550 | 0.00 | missing | README — check_type vs. code instrumentation modes | no discovered line candidate |
| 1.7 | 700 | 0.00 | missing | README — instrumentation options (@typechecked vs import hook) | no discovered line candidate |
| 3.8 | 8036 | 0.00 | missing | docs/api.rst — public API by topic group (head) | no discovered line candidate |
| 3.9 | 8213 | 0.00 | missing | docs/userguide.rst — H2 section locations | no discovered line candidate |
| 3.10 | 8392 | 0.00 | missing | docs/features.rst — H2 section locations | no discovered line candidate |
| 4.1 | 8692 | 0.00 | missing | docs/api.rst — Custom checkers / Suppression / Exceptions sections | no discovered line candidate |
| 4.2 | 9266 | 0.00 | missing | docs/extending.rst — writing a lookup + checker function (head) | no discovered line candidate |
| 4.3 | 9578 | 0.00 | missing | docs/extending.rst — MySpecialType worked example | no discovered line candidate |

### fs/listing

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 1.4 | 247 | 0.00 | missing | tests/ listing | fs-only |

### mixed/unknown

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 3.4 | 7148 | 0.17 | missing | _transformer.py — class locations | [scheduled bbox exact=5/6] python decl names surface in src/typeguard/_transformer.py (t=4778, 8 atoms) |

Top wasted paths (off-NS at 3K): docs/conf.py (287t, 3 batches), src/typeguard/_utils.py (259t, 3 batches), src/typeguard/_config.py (140t, 2 batches), src/typeguard/_functions.py (136t, 1 batch), src/typeguard/_importhook.py (102t, 1 batch), src/typeguard/_suppression.py (78t, 1 batch), src/typeguard/_decorators.py (75t, 1 batch)

## Walker waste (walker_t ≤ 3000, off_3k ≥ 50)

| off_3k | off_3k_ratio | off_any | cost | walker_t | batch |
|-------:|-------------:|--------:|-----:|---------:|:------|
| 136 | 0.93 | 55 | 146 | 2802 | python decl names surface in src/typeguard/_functions.py |
| 129 | 1.00 | 12 | 129 | 2305 | python decl names surface in src/typeguard/_utils.py |
| 126 | 1.00 | 126 | 126 | 1400 | python decl names surface in docs/conf.py |
| 102 | 0.90 | 76 | 113 | 1864 | python decl names surface in src/typeguard/_importhook.py |
| 101 | 1.00 | 101 | 101 | 2534 | python decl names surface #1 in docs/conf.py |
| 84 | 1.00 | 0 | 84 | 2115 | python class body at src/typeguard/_config.py:62 |
| 78 | 1.00 | 47 | 78 | 1219 | python decl names surface in src/typeguard/_suppression.py |
| 75 | 0.71 | 31 | 105 | 1686 | python decl names surface in src/typeguard/_decorators.py |
| 69 | 1.00 | 69 | 69 | 2733 | python decl doc at src/typeguard/_utils.py:104 |
| 61 | 1.00 | 61 | 61 | 2473 | python decl doc at src/typeguard/_utils.py:127 |
| 116 | — | — | — | — | +2 more rows |

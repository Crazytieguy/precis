scores: Sim=0.459 Reached=14/39 Early=3 Late=5 Partial=6 Missing=19 Used=9757/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 10 | 5 | 0 | 5 | 0.52 |
| 2 | 14 | 6 | 5 | 3 | 0.69 |
| 3 | 10 | 3 | 1 | 6 | 0.39 |
| 4 | 5 | 0 | 0 | 5 | 0.00 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 75 | 190 | +115 | 1.00 | late | docs/ listing |  |
| 1.3 | 147 | 316 | +169 | 1.00 | late | src/typeguard/ module listing |  |
| 1.4 | 247 | — | — | 0.00 | missing | tests/ listing |  |
| 1.5 | 349 | — | — | 0.00 | missing | README — one-paragraph lede |  |
| 1.6 | 550 | — | — | 0.00 | missing | README — check_type vs. code instrumentation modes |  |
| 1.7 | 700 | — | — | 0.00 | missing | README — instrumentation options (@typechecked vs import hook) |  |
| 1.8 | 964 | 1077 | +113 | 0.94 | aligned | Public re-exports — head of typeguard/__init__.py | python imports in src/typeguard/__init__.py (t=1077, 16 atoms) |
| 1.10 | 1351 | — | — | 0.29 | missing | typeguard/__init__.py — module rewrite, lazy `config`, autoload | python decl body at src/typeguard/__init__.py:37 (t=1164, 4 atoms) |
| 2.1 | 1663 | 9072 | +7409 | 0.82 | late | _exceptions.py — class signatures + summary docstrings | python method sigs in src/typeguard/_exceptions.py (t=2360, 10 atoms) |
| 2.2 | 2249 | — | — | 0.29 | missing | check_type — primary entry-point signature | python decl at src/typeguard/_functions.py:50 (t=4847, 12 atoms) |
| 2.3 | 2766 | — | — | 0.47 | missing | @typechecked — overloaded signatures | python decl at src/typeguard/_decorators.py:150 (t=3825, 8 atoms) |
| 2.4 | 2985 | — | — | 0.78 | partial | install_import_hook — signature + docstring | python decl doc at src/typeguard/_importhook.py:183 (t=5653, 9 atoms) |
| 2.5 | 3337 | — | — | 0.66 | partial | suppress_type_checks — overloaded signatures + docstring | python decl doc at src/typeguard/_suppression.py:30 (t=9272, 14 atoms) |
| 2.6 | 3817 | — | — | 0.20 | missing | TypeCheckConfiguration — dataclass + attribute docstrings | python class body at src/typeguard/_config.py:62 (t=3087, 6 atoms) |
| 2.7 | 4180 | — | — | 0.58 | partial | ForwardRefPolicy + CollectionCheckStrategy enums | python decl doc at src/typeguard/_config.py:30 (t=5204, 13 atoms) |
| 2.8 | 4402 | — | — | 0.78 | partial | TypeCheckMemo — class skeleton + __init__ | python decl doc at src/typeguard/_memo.py:8 (t=8712, 17 atoms) |
| 2.9 | 4573 | 6223 | +1650 | 0.81 | late | Checker plug-in API — TypeCheckerCallable / TypeCheckLookupCallback / checker_lookup_functions | python decl names surface in src/typeguard/_checkers.py (t=6066, 5 atoms) |
| 2.10 | 4926 | 5049 | +123 | 0.84 | aligned | TypeguardFinder + ImportHookManager — class + key methods | python method sigs in src/typeguard/_importhook.py (t=4180, 14 atoms) |
| 2.11 | 5177 | — | — | 0.79 | partial | warn_on_error + load_plugins — signatures | python decl doc at src/typeguard/_checkers.py:1099 (t=7644, 7 atoms) |
| 2.12 | 5358 | 7769 | +2411 | 0.88 | late | check_type_internal — signature + docstring | python decl doc at src/typeguard/_checkers.py:924 (t=7769, 9 atoms) |
| 2.13 | 5642 | 2777 | -2865 | 1.00 | early | Internal check_argument_types / check_return_type / check_yield_type / check_send_type / check_variable_assignment — signatures | python decl names surface in src/typeguard/_functions.py (t=2577, 10 atoms) |
| 2.14 | 5813 | 2273 | -3540 | 0.80 | early | Unset sentinel + small _utils helpers | python decl names surface in src/typeguard/_utils.py (t=2234, 15 atoms) |
| 3.2 | 6639 | — | — | 0.02 | missing | _checkers.py — origin_type_checkers dispatch table | python decl names surface in src/typeguard/_checkers.py (t=6066, 1 atoms) |
| 3.3 | 7082 | — | — | 0.08 | missing | _checkers.py — builtin_checker_lookup dispatch fallbacks | python decl at src/typeguard/_checkers.py:1061 (t=6163, 3 atoms) |
| 3.4 | 7148 | 3240 | -3908 | 1.00 | early | _transformer.py — class locations | python decl names surface in src/typeguard/_transformer.py (t=3240, 9 atoms) |
| 3.5 | 7354 | — | — | 0.00 | missing | _transformer.py — TypeguardTransformer visit_* methods (locations) | python decl names surface in src/typeguard/_transformer.py (t=3240, 2 atoms) |
| 3.7 | 7673 | — | — | 0.75 | partial | _decorators.py — function locations | python decl names surface in src/typeguard/_decorators.py (t=1455, 5 atoms) |
| 3.8 | 8036 | — | — | 0.00 | missing | docs/api.rst — public API by topic group (head) |  |
| 3.9 | 8213 | — | — | 0.00 | missing | docs/userguide.rst — H2 section locations |  |
| 3.10 | 8392 | — | — | 0.00 | missing | docs/features.rst — H2 section locations |  |
| 4.1 | 8692 | — | — | 0.00 | missing | docs/api.rst — Custom checkers / Suppression / Exceptions sections |  |
| 4.2 | 9266 | — | — | 0.00 | missing | docs/extending.rst — writing a lookup + checker function (head) |  |
| 4.3 | 9578 | — | — | 0.00 | missing | docs/extending.rst — MySpecialType worked example |  |
| 4.4 | 9827 | — | — | 0.00 | missing | tests/test_checkers.py — TestX class locations |  |
| 4.5 | 9976 | — | — | 0.00 | missing | tests/test_typechecked.py — Test class + module test locations |  |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 227 | 1.00 | 227 | 1999 | python decl names surface in docs/conf.py |
| 215 | 0.98 | 219 | 8712 | python decl doc at src/typeguard/_memo.py:8 |
| 201 | 1.00 | 201 | 9757 | python imports in src/typeguard/_decorators.py |
| 185 | 1.00 | 185 | 9457 | python imports in src/typeguard/_importhook.py |
| 176 | 1.00 | 176 | 8971 | python imports in src/typeguard/_functions.py |
| 160 | 1.00 | 160 | 8246 | python imports in src/typeguard/_utils.py |
| 158 | 1.00 | 158 | 8404 | python decl at src/typeguard/_transformer.py:100 |
| 127 | 1.00 | 127 | 5331 | python imports in src/typeguard/_pytest_plugin.py |
| 122 | 1.00 | 122 | 8043 | plaintext config .gitignore |
| 112 | 1.00 | 112 | 3937 | python class body at src/typeguard/_transformer.py:338 |
| 103 | 0.69 | 149 | 4180 | python method sigs in src/typeguard/_importhook.py |
| 99 | 1.00 | 99 | 9556 | python decl body at src/typeguard/_utils.py:142 |
| 90 | 0.58 | 155 | 5204 | python decl doc at src/typeguard/_config.py:30 |
| 89 | 1.00 | 89 | 8493 | python decl body at src/typeguard/_utils.py:127 |
| 83 | 1.00 | 83 | 8795 | .github/SECURITY.md section #1 |
| 82 | 1.00 | 82 | 4929 | python imports in src/typeguard/_config.py |
| 81 | 1.00 | 81 | 2994 | python decl at src/typeguard/_functions.py:28 |
| 80 | 1.00 | 80 | 2904 | python decl at src/typeguard/_functions.py:39 |
| 78 | 0.19 | 413 | 6066 | python decl names surface in src/typeguard/_checkers.py |
| 78 | 1.00 | 78 | 4724 | python method at src/typeguard/_importhook.py:56 |
| 76 | 1.00 | 76 | 7845 | python decl body at src/typeguard/_checkers.py:651 |
| 76 | 1.00 | 76 | 7921 | python decl body at src/typeguard/_utils.py:162 |
| 76 | 0.67 | 113 | 1678 | python decl names surface in src/typeguard/_importhook.py |
| 76 | 1.00 | 76 | 4543 | python imports in src/typeguard/_suppression.py |
| 69 | 1.00 | 69 | 3723 | python decl doc at src/typeguard/_utils.py:104 |
| 67 | 1.00 | 67 | 7443 | python decl at src/typeguard/_checkers.py:555 |
| 61 | 1.00 | 61 | 7376 | python decl at src/typeguard/_checkers.py:365 |
| 61 | 1.00 | 61 | 3616 | python decl doc at src/typeguard/_utils.py:127 |
| 60 | 1.00 | 60 | 2105 | python decl at docs/conf.py:28 |
| 58 | 0.38 | 153 | 3240 | python decl names surface in src/typeguard/_transformer.py |
| 56 | 1.00 | 56 | 4985 | python decl body at src/typeguard/_utils.py:154 |
| 55 | 0.38 | 146 | 2577 | python decl names surface in src/typeguard/_functions.py |
| 50 | 1.00 | 50 | 4281 | python method at src/typeguard/_importhook.py:167 |
| 50 | 1.00 | 50 | 9021 | python method body at src/typeguard/_importhook.py:99 |

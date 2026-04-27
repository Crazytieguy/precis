scores: Sim=0.360 Reached=4/39 Early=0 Late=3 Partial=0 Missing=35 Used=1220/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 10 | 4 | 0 | 6 | 0.40 |
| 2 | 14 | 0 | 0 | 14 | 0.00 |
| 3 | 10 | 0 | 0 | 10 | 0.00 |
| 4 | 5 | 0 | 0 | 5 | 0.00 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 75 | 190 | +115 | 1.00 | late | docs/ listing |  |
| 1.3 | 147 | 292 | +145 | 1.00 | late | src/typeguard/ module listing |  |
| 1.4 | 247 | 915 | +668 | 1.00 | late | tests/ listing |  |
| 1.5 | 349 | — | — | 0.00 | missing | README — one-paragraph lede |  |
| 1.6 | 550 | — | — | 0.00 | missing | README — check_type vs. code instrumentation modes |  |
| 1.7 | 700 | — | — | 0.00 | missing | README — instrumentation options (@typechecked vs import hook) |  |
| 1.8 | 964 | — | — | 0.00 | missing | Public re-exports — head of typeguard/__init__.py |  |
| 1.9 | 1119 | — | — | 0.00 | missing | Public re-exports — tail of typeguard/__init__.py |  |
| 1.10 | 1351 | — | — | 0.00 | missing | typeguard/__init__.py — module rewrite, lazy `config`, autoload |  |
| 2.1 | 1663 | — | — | 0.00 | missing | _exceptions.py — class signatures + summary docstrings |  |
| 2.2 | 2249 | — | — | 0.00 | missing | check_type — primary entry-point signature |  |
| 2.3 | 2766 | — | — | 0.00 | missing | @typechecked — overloaded signatures |  |
| 2.4 | 2985 | — | — | 0.00 | missing | install_import_hook — signature + docstring |  |
| 2.5 | 3337 | — | — | 0.00 | missing | suppress_type_checks — overloaded signatures + docstring |  |
| 2.6 | 3817 | — | — | 0.00 | missing | TypeCheckConfiguration — dataclass + attribute docstrings |  |
| 2.7 | 4180 | — | — | 0.00 | missing | ForwardRefPolicy + CollectionCheckStrategy enums |  |
| 2.8 | 4402 | — | — | 0.00 | missing | TypeCheckMemo — class skeleton + __init__ |  |
| 2.9 | 4573 | — | — | 0.00 | missing | Checker plug-in API — TypeCheckerCallable / TypeCheckLookupCallback / checker_lookup_functions |  |
| 2.10 | 4926 | — | — | 0.00 | missing | TypeguardFinder + ImportHookManager — class + key methods |  |
| 2.11 | 5177 | — | — | 0.00 | missing | warn_on_error + load_plugins — signatures |  |
| 2.12 | 5358 | — | — | 0.00 | missing | check_type_internal — signature + docstring |  |
| 2.13 | 5642 | — | — | 0.00 | missing | Internal check_argument_types / check_return_type / check_yield_type / check_send_type / check_variable_assignment — signatures |  |
| 2.14 | 5813 | — | — | 0.00 | missing | Unset sentinel + small _utils helpers |  |
| 3.1 | 6041 | — | — | 0.00 | missing | _checkers.py — every check_* function name (locations) |  |
| 3.2 | 6639 | — | — | 0.00 | missing | _checkers.py — origin_type_checkers dispatch table |  |
| 3.3 | 7082 | — | — | 0.00 | missing | _checkers.py — builtin_checker_lookup dispatch fallbacks |  |
| 3.4 | 7148 | — | — | 0.00 | missing | _transformer.py — class locations |  |
| 3.5 | 7354 | — | — | 0.00 | missing | _transformer.py — TypeguardTransformer visit_* methods (locations) |  |
| 3.6 | 7639 | — | — | 0.00 | missing | _transformer.py — generator_names + annotated_names + ignore_decorators tables |  |
| 3.7 | 7673 | — | — | 0.00 | missing | _decorators.py — function locations |  |
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
| 291 | 1.00 | 291 | 1220 | plaintext config LICENSE |
| 280 | 1.00 | 280 | 815 | .github/pull_request_template.md section #1 |
| 122 | 1.00 | 122 | 452 | plaintext config .gitignore |
| 83 | 1.00 | 83 | 535 | .github/SECURITY.md section #1 |

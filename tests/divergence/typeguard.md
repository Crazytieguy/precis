scores: Sim=0.457 Reached=14/39 Early=3 Late=5 Partial=7 Missing=18 Used=9974/10000

## Verdict

Verdict: coverage-gap bound
Likely primary lever: add walker candidates for no-discovered NS rows
Evidence: 2 ranking-recoverable (w×gap=0.06), 11 wrong-slice/granularity (w×gap=1.07), 11 no-discovered (w×gap=2.40)
Secondary intervention: free final budget for 2 too-expensive candidates
Loss reasons: 0 predecessor-gated, 2 too-expensive, 0 discovered-unscheduled
Top rows: 1.5, 1.6, 1.7, 3.8, 3.9, ...
Note: likely lever is heuristic; verify `Sim` moves, not just bucket counts.

## Top opportunities

_`w(t)×gap` is a non-additive priority score: Σ exp(-exp_t/τ) × (1 - credit) per row, τ=2000. Same time weighting and credit gap as Sim, but rows can overlap between opportunities — sums across rows are an upper bound on Sim impact, not an additive estimate._

| intervention | rows | w(t)×gap | bands ≤3k/≤6k/total | evidence | top row ids |
|:-------------|-----:|---------:|:----------------------|:---------|:------------|
| add walker candidates for no-discovered rows | 11 | 2.40 | 3/3/11 | NS rows have no discovered line candidate | 1.5, 1.6, 1.7, 3.8, 3.9, ... |
| split wrong-slice walker batches | 11 | 1.07 | 4/9/11 | nearby candidates have low exact atom overlap | 1.10, 2.2, 2.3, 2.6, 2.5, ... |
| free final budget / demote late waste | 2 | 0.06 | 0/0/2 | high-overlap candidates exceed final remaining budget, exact total=70/81 | 3.2, 3.3 |

Tiers: 1=5/10 reached, 0 partial, 5 missing, avg=0.52; 2=6/14 reached, 5 partial, 3 missing, avg=0.69; 3=3/10 reached, 2 partial, 5 missing, avg=0.46; 4=0/5 reached, 0 partial, 5 missing, avg=0.00

## Diagnosis rollup

| diagnosis | rows | missing | partial | timing | likely lever |
|:----------|-----:|--------:|--------:|-------:|:-------------|
| ranking-recoverable | 2 | 2 | 0 | 0 | value/ranking |
| wrong-slice / granularity | 11 | 4 | 7 | 0 | walker granularity / wrong slice |
| no discovered candidate | 11 | 11 | 0 | 0 | walker coverage or predecessor-gated emit |
| fs/listing | 1 | 1 | 0 | 0 | filesystem/listing value |
| timing-only | 11 | 0 | 0 | 11 | usually no code change |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | w(t)×gap | likely lever |
|:------------|-----:|---------:|:-------------|
| too expensive at final margin | 2 | 0.06 | free final budget |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=22, fs-only=3, no discovered candidate=11

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | aligned | low | 2 |
| scheduled bbox | aligned | high | 1 |
| scheduled bbox | early | low | 2 |
| scheduled bbox | early | high | 1 |
| scheduled bbox | late | low | 3 |
| scheduled bbox | missing | low | 6 |
| scheduled bbox | partial | low | 7 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 3.2 | 6639 | — | — | 0.02 | missing | _checkers.py — origin_type_checkers dispatch table | [scheduled bbox exact=1/45] python decl names surface #2 in src/typeguard/_checkers.py (t=8834, 1 atoms); better unscheduled exact=41/45: python decl at src/typeguard/_checkers.py:1005 (41 atoms, too expensive at final margin) |
| 3.3 | 7082 | — | — | 0.08 | missing | _checkers.py — builtin_checker_lookup dispatch fallbacks | [scheduled bbox exact=3/36] python decl at src/typeguard/_checkers.py:1061 (t=8905, 3 atoms); better unscheduled exact=29/36: python decl body at src/typeguard/_checkers.py:1061 (29 atoms, too expensive at final margin) |

### wrong-slice / granularity

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.10 | 1351 | — | — | 0.29 | missing | typeguard/__init__.py — module rewrite, lazy `config`, autoload | [scheduled bbox exact=4/21] python decl body at src/typeguard/__init__.py:37 (t=1177, 4 atoms) |
| 2.2 | 2249 | — | — | 0.29 | missing | check_type — primary entry-point signature | [scheduled bbox exact=12/42] python decl at src/typeguard/_functions.py:50 (t=5362, 12 atoms); better unscheduled exact=24/42: python decl doc at src/typeguard/_functions.py:50 (24 atoms, too expensive at final margin) |
| 2.3 | 2766 | — | — | 0.47 | missing | @typechecked — overloaded signatures | [scheduled bbox exact=8/38] python decl at src/typeguard/_decorators.py:150 (t=4340, 8 atoms); better unscheduled exact=12/38: python decl doc at src/typeguard/_decorators.py:150 (12 atoms, too expensive at final margin) |
| 2.4 | 2985 | — | — | 0.78 | partial | install_import_hook — signature + docstring | [scheduled bbox exact=9/18] python decl doc at src/typeguard/_importhook.py:183 (t=6985, 9 atoms) |
| 2.5 | 3337 | — | — | 0.66 | partial | suppress_type_checks — overloaded signatures + docstring | [scheduled bbox exact=14/32] python decl doc at src/typeguard/_suppression.py:30 (t=8412, 14 atoms) |
| 2.6 | 3817 | — | — | 0.20 | missing | TypeCheckConfiguration — dataclass + attribute docstrings | [scheduled bbox exact=6/45] python class body at src/typeguard/_config.py:62 (t=3100, 6 atoms); better unscheduled exact=21/45: python decl doc at src/typeguard/_config.py:62 (21 atoms, too expensive at final margin) |
| 2.7 | 4180 | — | — | 0.58 | partial | ForwardRefPolicy + CollectionCheckStrategy enums | [scheduled bbox exact=6/36] python decl doc at src/typeguard/_config.py:30 (t=6536, 13 atoms); better unscheduled exact=7/36: python method body at src/typeguard/_config.py:52 (7 atoms, too expensive at final margin) |
| 2.8 | 4402 | — | — | 0.78 | partial | TypeCheckMemo — class skeleton + __init__ | [scheduled bbox exact=1/23] python decl doc at src/typeguard/_memo.py:8 (t=7852, 17 atoms) |
| 2.11 | 5177 | — | — | 0.79 | partial | warn_on_error + load_plugins — signatures | [scheduled bbox exact=7/19] python decl doc at src/typeguard/_checkers.py:1099 (t=9355, 7 atoms) |
| 3.5 | 7354 | — | — | 0.70 | partial | _transformer.py — TypeguardTransformer visit_* methods (locations) | [scheduled bbox exact=14/20] python method sigs #1 in src/typeguard/_transformer.py (t=9947, 41 atoms) |
| 3.7 | 7673 | — | — | 0.75 | partial | _decorators.py — function locations | [scheduled bbox exact=3/4] python decl names surface in src/typeguard/_decorators.py (t=1640, 5 atoms) |

### no discovered candidate

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.5 | 349 | — | — | 0.00 | missing | README — one-paragraph lede | no discovered line candidate |
| 1.6 | 550 | — | — | 0.00 | missing | README — check_type vs. code instrumentation modes | no discovered line candidate |
| 1.7 | 700 | — | — | 0.00 | missing | README — instrumentation options (@typechecked vs import hook) | no discovered line candidate |
| 3.8 | 8036 | — | — | 0.00 | missing | docs/api.rst — public API by topic group (head) | no discovered line candidate |
| 3.9 | 8213 | — | — | 0.00 | missing | docs/userguide.rst — H2 section locations | no discovered line candidate |
| 3.10 | 8392 | — | — | 0.00 | missing | docs/features.rst — H2 section locations | no discovered line candidate |
| 4.1 | 8692 | — | — | 0.00 | missing | docs/api.rst — Custom checkers / Suppression / Exceptions sections | no discovered line candidate |
| 4.2 | 9266 | — | — | 0.00 | missing | docs/extending.rst — writing a lookup + checker function (head) | no discovered line candidate |
| 4.3 | 9578 | — | — | 0.00 | missing | docs/extending.rst — MySpecialType worked example | no discovered line candidate |
| 4.4 | 9827 | — | — | 0.00 | missing | tests/test_checkers.py — TestX class locations | no discovered line candidate |
| 4.5 | 9976 | — | — | 0.00 | missing | tests/test_typechecked.py — Test class + module test locations | no discovered line candidate |

### fs/listing

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.4 | 247 | — | — | 0.00 | missing | tests/ listing | fs-only |

### timing-only

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 75 | 190 | +115 | 1.00 | late | docs/ listing | fs-only |
| 1.3 | 147 | 316 | +169 | 1.00 | late | src/typeguard/ module listing | fs-only |
| 1.8 | 964 | 1090 | +126 | 0.94 | aligned | Public re-exports — head of typeguard/__init__.py | [scheduled bbox exact=16/17] python imports in src/typeguard/__init__.py (t=1090, 16 atoms) |
| 2.1 | 1663 | 8212 | +6549 | 0.82 | late | _exceptions.py — class signatures + summary docstrings | [scheduled bbox exact=5/28] python method sigs in src/typeguard/_exceptions.py (t=2212, 10 atoms) |
| 2.9 | 4573 | 3881 | -692 | 0.81 | aligned | Checker plug-in API — TypeCheckerCallable / TypeCheckLookupCallback / checker_lookup_functions | [scheduled bbox exact=5/16] python decl at src/typeguard/_checkers.py:89 (t=3881, 5 atoms) |
| 2.10 | 4926 | 5564 | +638 | 0.84 | aligned | TypeguardFinder + ImportHookManager — class + key methods | [scheduled bbox exact=6/31] python method sigs in src/typeguard/_importhook.py (t=4695, 14 atoms) |
| 2.12 | 5358 | 9480 | +4122 | 0.88 | late | check_type_internal — signature + docstring | [scheduled bbox exact=9/16] python decl doc at src/typeguard/_checkers.py:924 (t=9480, 9 atoms) |
| 2.13 | 5642 | 2790 | -2852 | 1.00 | early | Internal check_argument_types / check_return_type / check_yield_type / check_send_type / check_variable_assignment — signatures | [scheduled bbox exact=10/28] python decl names surface in src/typeguard/_functions.py (t=2590, 10 atoms) |
| 2.14 | 5813 | 2125 | -3688 | 0.80 | early | Unset sentinel + small _utils helpers | [scheduled bbox exact=9/15] python decl names surface in src/typeguard/_utils.py (t=2086, 15 atoms) |
| 3.1 | 6041 | 8834 | +2793 | 1.00 | late | _checkers.py — every check_* function name (locations) | [scheduled bbox exact=11/25] python decl names surface #1 in src/typeguard/_checkers.py (t=5698, 24 atoms) |
| 3.4 | 7148 | 3501 | -3647 | 1.00 | early | _transformer.py — class locations | [scheduled bbox exact=5/6] python decl names surface in src/typeguard/_transformer.py (t=3501, 8 atoms) |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 215 | 0.98 | 219 | 7852 | python decl doc at src/typeguard/_memo.py:8 |
| 185 | 1.00 | 185 | 8597 | python imports in src/typeguard/_importhook.py |
| 176 | 1.00 | 176 | 8111 | python imports in src/typeguard/_functions.py |
| 160 | 1.00 | 160 | 7386 | python imports in src/typeguard/_utils.py |
| 158 | 1.00 | 158 | 7544 | python decl at src/typeguard/_transformer.py:100 |
| 137 | 0.35 | 391 | 9947 | python method sigs #1 in src/typeguard/_transformer.py |
| 127 | 1.00 | 127 | 6663 | python imports in src/typeguard/_pytest_plugin.py |
| 126 | 1.00 | 126 | 1447 | python decl names surface in docs/conf.py |
| 122 | 1.00 | 122 | 7183 | plaintext config .gitignore |
| 112 | 1.00 | 112 | 4452 | python class body at src/typeguard/_transformer.py:338 |
| 1775 | — | — | — | +24 more rows |

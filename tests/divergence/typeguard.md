scores: Sim=0.459 Reached=14/39 Early=3 Late=5 Partial=6 Missing=19 Used=9757/10000

## Verdict

Verdict: coverage-gap bound
Likely primary lever: add walker candidates for no-discovered NS rows
Evidence: 3 ranking-recoverable (w×gap=0.09), 10 wrong-slice/granularity (w×gap=1.06), 11 no-discovered (w×gap=2.40)
Secondary intervention: free final budget for 3 too-expensive candidates
Loss reasons: 0 predecessor-gated, 3 too-expensive, 0 discovered-unscheduled
Top rows: 1.5, 1.6, 1.7, 3.8, 3.9, ...
Note: likely lever is heuristic; verify `Sim` moves, not just bucket counts.

## Top opportunities

_`w(t)×gap` is a non-additive priority score: Σ exp(-exp_t/τ) × (1 - credit) per row, τ=2000. Same time weighting and credit gap as Sim, but rows can overlap between opportunities — sums across rows are an upper bound on Sim impact, not an additive estimate._

| intervention | rows | w(t)×gap | bands ≤3k/≤6k/total | evidence | top row ids |
|:-------------|-----:|---------:|:----------------------|:---------|:------------|
| add walker candidates for no-discovered rows | 11 | 2.40 | 3/3/11 | NS rows have no discovered line candidate | 1.5, 1.6, 1.7, 3.8, 3.9, ... |
| split wrong-slice walker batches | 10 | 1.06 | 4/9/10 | nearby candidates have low exact atom overlap | 1.10, 2.2, 2.3, 2.6, 2.5, ... |
| free final budget / demote late waste | 3 | 0.09 | 0/0/3 | high-overlap candidates exceed final remaining budget, exact total=90/101 | 3.2, 3.3, 3.5 |

Tiers: 1=5/10 reached, 0 partial, 5 missing, avg=0.52; 2=6/14 reached, 5 partial, 3 missing, avg=0.69; 3=3/10 reached, 1 partial, 6 missing, avg=0.39; 4=0/5 reached, 0 partial, 5 missing, avg=0.00

## Diagnosis rollup

| diagnosis | rows | missing | partial | timing | likely lever |
|:----------|-----:|--------:|--------:|-------:|:-------------|
| ranking-recoverable | 3 | 3 | 0 | 0 | value/ranking |
| wrong-slice / granularity | 10 | 4 | 6 | 0 | walker granularity / wrong slice |
| no discovered candidate | 11 | 11 | 0 | 0 | walker coverage or predecessor-gated emit |
| fs/listing | 1 | 1 | 0 | 0 | filesystem/listing value |
| timing-only | 10 | 0 | 0 | 10 | usually no code change |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | w(t)×gap | likely lever |
|:------------|-----:|---------:|:-------------|
| too expensive at final margin | 3 | 0.09 | free final budget |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=21, fs-only=3, no discovered candidate=11

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | aligned | low | 1 |
| scheduled bbox | aligned | high | 1 |
| scheduled bbox | early | low | 2 |
| scheduled bbox | early | full | 1 |
| scheduled bbox | late | low | 3 |
| scheduled bbox | missing | none | 1 |
| scheduled bbox | missing | low | 6 |
| scheduled bbox | partial | low | 6 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 3.2 | 6639 | — | — | 0.02 | missing | _checkers.py — origin_type_checkers dispatch table | [scheduled bbox exact=1/45] python decl names surface in src/typeguard/_checkers.py (t=6066, 1 atoms); better unscheduled exact=41/45: python decl at src/typeguard/_checkers.py:1005 (41 atoms, too expensive at final margin) |
| 3.3 | 7082 | — | — | 0.08 | missing | _checkers.py — builtin_checker_lookup dispatch fallbacks | [scheduled bbox exact=3/36] python decl at src/typeguard/_checkers.py:1061 (t=6163, 3 atoms); better unscheduled exact=29/36: python decl body at src/typeguard/_checkers.py:1061 (29 atoms, too expensive at final margin) |
| 3.5 | 7354 | — | — | 0.00 | missing | _transformer.py — TypeguardTransformer visit_* methods (locations) | [scheduled bbox exact=0/20] python decl names surface in src/typeguard/_transformer.py (t=3240, 2 atoms); better unscheduled exact=20/20: python method sigs in src/typeguard/_transformer.py (53 atoms, too expensive at final margin) |

### wrong-slice / granularity

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.10 | 1351 | — | — | 0.29 | missing | typeguard/__init__.py — module rewrite, lazy `config`, autoload | [scheduled bbox exact=4/21] python decl body at src/typeguard/__init__.py:37 (t=1164, 4 atoms) |
| 2.2 | 2249 | — | — | 0.29 | missing | check_type — primary entry-point signature | [scheduled bbox exact=12/42] python decl at src/typeguard/_functions.py:50 (t=4847, 12 atoms); better unscheduled exact=24/42: python decl doc at src/typeguard/_functions.py:50 (24 atoms, too expensive at final margin) |
| 2.3 | 2766 | — | — | 0.47 | missing | @typechecked — overloaded signatures | [scheduled bbox exact=8/38] python decl at src/typeguard/_decorators.py:150 (t=3825, 8 atoms); better unscheduled exact=12/38: python decl doc at src/typeguard/_decorators.py:150 (12 atoms, too expensive at final margin) |
| 2.4 | 2985 | — | — | 0.78 | partial | install_import_hook — signature + docstring | [scheduled bbox exact=9/18] python decl doc at src/typeguard/_importhook.py:183 (t=5653, 9 atoms) |
| 2.5 | 3337 | — | — | 0.66 | partial | suppress_type_checks — overloaded signatures + docstring | [scheduled bbox exact=14/32] python decl doc at src/typeguard/_suppression.py:30 (t=9272, 14 atoms) |
| 2.6 | 3817 | — | — | 0.20 | missing | TypeCheckConfiguration — dataclass + attribute docstrings | [scheduled bbox exact=6/45] python class body at src/typeguard/_config.py:62 (t=3087, 6 atoms); better unscheduled exact=21/45: python decl doc at src/typeguard/_config.py:62 (21 atoms, too expensive at final margin) |
| 2.7 | 4180 | — | — | 0.58 | partial | ForwardRefPolicy + CollectionCheckStrategy enums | [scheduled bbox exact=6/36] python decl doc at src/typeguard/_config.py:30 (t=5204, 13 atoms); better unscheduled exact=7/36: python method body at src/typeguard/_config.py:52 (7 atoms, discovered unscheduled) |
| 2.8 | 4402 | — | — | 0.78 | partial | TypeCheckMemo — class skeleton + __init__ | [scheduled bbox exact=1/23] python decl doc at src/typeguard/_memo.py:8 (t=8712, 17 atoms) |
| 2.11 | 5177 | — | — | 0.79 | partial | warn_on_error + load_plugins — signatures | [scheduled bbox exact=7/19] python decl doc at src/typeguard/_checkers.py:1099 (t=7644, 7 atoms) |
| 3.7 | 7673 | — | — | 0.75 | partial | _decorators.py — function locations | [scheduled bbox exact=3/4] python decl names surface in src/typeguard/_decorators.py (t=1455, 5 atoms) |

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
| 1.8 | 964 | 1077 | +113 | 0.94 | aligned | Public re-exports — head of typeguard/__init__.py | [scheduled bbox exact=16/17] python imports in src/typeguard/__init__.py (t=1077, 16 atoms) |
| 2.1 | 1663 | 9072 | +7409 | 0.82 | late | _exceptions.py — class signatures + summary docstrings | [scheduled bbox exact=5/28] python method sigs in src/typeguard/_exceptions.py (t=2360, 10 atoms) |
| 2.9 | 4573 | 6223 | +1650 | 0.81 | late | Checker plug-in API — TypeCheckerCallable / TypeCheckLookupCallback / checker_lookup_functions | [scheduled bbox exact=5/16] python decl at src/typeguard/_checkers.py:89 (t=6223, 5 atoms) |
| 2.10 | 4926 | 5049 | +123 | 0.84 | aligned | TypeguardFinder + ImportHookManager — class + key methods | [scheduled bbox exact=6/31] python method sigs in src/typeguard/_importhook.py (t=4180, 14 atoms) |
| 2.12 | 5358 | 7769 | +2411 | 0.88 | late | check_type_internal — signature + docstring | [scheduled bbox exact=9/16] python decl doc at src/typeguard/_checkers.py:924 (t=7769, 9 atoms) |
| 2.13 | 5642 | 2777 | -2865 | 1.00 | early | Internal check_argument_types / check_return_type / check_yield_type / check_send_type / check_variable_assignment — signatures | [scheduled bbox exact=10/28] python decl names surface in src/typeguard/_functions.py (t=2577, 10 atoms) |
| 2.14 | 5813 | 2273 | -3540 | 0.80 | early | Unset sentinel + small _utils helpers | [scheduled bbox exact=9/15] python decl names surface in src/typeguard/_utils.py (t=2234, 15 atoms) |
| 3.4 | 7148 | 3240 | -3908 | 1.00 | early | _transformer.py — class locations | [scheduled bbox exact=6/6] python decl names surface in src/typeguard/_transformer.py (t=3240, 9 atoms) |

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
| 1754 | — | — | — | +24 more rows |

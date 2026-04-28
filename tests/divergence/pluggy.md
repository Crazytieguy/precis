scores: Sim=0.393 Reached=14/40 Early=4 Late=7 Partial=5 Missing=21 Used=9991/10000

## Verdict

Verdict: coverage-gap bound
Likely primary lever: add walker candidates for no-discovered NS rows
Evidence: 4 ranking-recoverable (w×gap=0.57), 8 wrong-slice/granularity (w×gap=0.34), 13 no-discovered (w×gap=3.20)
Secondary intervention: free final budget for 4 too-expensive candidates
Loss reasons: 0 predecessor-gated, 4 too-expensive, 0 discovered-unscheduled
Top rows: 1.2, 2.1, 2.2, 3.2, 5.7, ...
Note: likely lever is heuristic; verify `Sim` moves, not just bucket counts.

## Top opportunities

_`w(t)×gap` is a non-additive priority score: Σ exp(-exp_t/τ) × (1 - credit) per row, τ=2000. Same time weighting and credit gap as Sim, but rows can overlap between opportunities — sums across rows are an upper bound on Sim impact, not an additive estimate._

| intervention | rows | w(t)×gap | bands ≤3k/≤6k/total | evidence | top row ids |
|:-------------|-----:|---------:|:----------------------|:---------|:------------|
| add walker candidates for no-discovered rows | 13 | 3.20 | 4/7/13 | NS rows have no discovered line candidate | 1.2, 2.1, 2.2, 3.2, 5.7, ... |
| free final budget / demote late waste | 4 | 0.57 | 1/2/4 | high-overlap candidates exceed final remaining budget, exact total=116/131 | 4.1, 5.4, 7.3, 7.11 |
| split wrong-slice walker batches | 8 | 0.34 | 2/4/8 | nearby candidates have low exact atom overlap | 4.5, 5.3, 5.1, 5.6, 7.4, ... |

Tiers: 1=3/4 reached, 0 partial, 1 missing, avg=0.75; 2=0/2 reached, 0 partial, 2 missing, avg=0.00; 3=1/2 reached, 0 partial, 1 missing, avg=0.50; 4=4/6 reached, 1 partial, 1 missing, avg=0.80; 5=2/7 reached, 3 partial, 2 missing, avg=0.53; 6=0/5 reached, 0 partial, 5 missing, avg=0.00; 7=4/14 reached, 1 partial, 9 missing, avg=0.37

## Diagnosis rollup

| diagnosis | rows | missing | partial | timing | likely lever |
|:----------|-----:|--------:|--------:|-------:|:-------------|
| ranking-recoverable | 4 | 4 | 0 | 0 | value/ranking |
| wrong-slice / granularity | 8 | 3 | 5 | 0 | walker granularity / wrong slice |
| no discovered candidate | 13 | 13 | 0 | 0 | walker coverage or predecessor-gated emit |
| fs/listing | 1 | 1 | 0 | 0 | filesystem/listing value |
| timing-only | 12 | 0 | 0 | 12 | usually no code change |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | w(t)×gap | likely lever |
|:------------|-----:|---------:|:-------------|
| too expensive at final margin | 4 | 0.57 | free final budget |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=16, unscheduled bbox=3, fs-only=6, no discovered candidate=13

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | aligned | high | 1 |
| scheduled bbox | late | none | 1 |
| scheduled bbox | late | low | 3 |
| scheduled bbox | late | high | 1 |
| scheduled bbox | late | full | 1 |
| scheduled bbox | missing | none | 1 |
| scheduled bbox | missing | low | 3 |
| scheduled bbox | partial | low | 5 |
| unscheduled bbox | missing | low | 2 |
| unscheduled bbox | missing | full | 1 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 4.1 | 1595 | — | — | 0.15 | missing | _manager.py — every class + def name | [scheduled bbox exact=0/34] python decl doc at src/pluggy/_manager.py:83 (t=6582, 11 atoms); better unscheduled exact=29/34: python method sigs in src/pluggy/_manager.py (60 atoms, too expensive at final margin) |
| 5.4 | 3651 | — | — | 0.05 | missing | HookCaller._add_hookimpl body — the actual ordering algorithm | [scheduled bbox exact=1/22] python method doc at src/pluggy/_hooks.py:453 (t=9057, 1 atoms); better unscheduled exact=20/22: python method body at src/pluggy/_hooks.py:453 (20 atoms, too expensive at final margin) |
| 7.3 | 7193 | — | — | 0.00 | missing | test_pluginmanager.py — every test fn name | [unscheduled bbox exact=32/32] python test names surface in testing/test_pluginmanager.py (63 atoms, too expensive at final margin) |
| 7.11 | 9819 | — | — | 0.09 | missing | _callers._multicall body — the actual call loop | [scheduled bbox exact=4/43] python decl doc at src/pluggy/_callers.py:82 (t=6413, 4 atoms); better unscheduled exact=35/43: python decl body at src/pluggy/_callers.py:82 (35 atoms, too expensive at final margin) |

### wrong-slice / granularity

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 4.5 | 2581 | — | — | 0.63 | partial | _warnings.py — full file (Pluggy*Warning classes) | [scheduled bbox exact=10/27] python decl doc at src/pluggy/_warnings.py:10 (t=6870, 10 atoms) |
| 5.1 | 2873 | — | — | 0.73 | partial | PluginManager class docstring | [scheduled bbox exact=11/15] python decl doc at src/pluggy/_manager.py:83 (t=6582, 11 atoms) |
| 5.3 | 3369 | — | — | 0.55 | partial | HookCaller __slots__ + the 6-bucket call-order comment | [scheduled bbox exact=7/29] python method at src/pluggy/_hooks.py:393 (t=9289, 7 atoms); better unscheduled exact=11/29: python method body at src/pluggy/_hooks.py:393 (11 atoms, too expensive at final margin) |
| 5.6 | 4494 | — | — | 0.62 | partial | Result API — force_result, force_exception, get_result bodies | [scheduled bbox exact=6/37] python method doc at src/pluggy/_result.py:67 (t=7258, 6 atoms); better unscheduled exact=7/37: python method body at src/pluggy/_result.py:91 (7 atoms, too expensive at final margin) |
| 7.2 | 6827 | — | — | 0.72 | partial | testing/conftest.py — pm + he_pm fixtures | [scheduled bbox exact=7/25] python decl body at testing/conftest.py:7 (t=4492, 7 atoms) |
| 7.4 | 7623 | — | — | 0.00 | missing | test_hookcaller.py + test_multicall.py — every test fn name | [unscheduled bbox exact=21/36] python test names surface in testing/test_multicall.py (41 atoms, too expensive at final margin) |
| 7.5 | 8023 | — | — | 0.00 | missing | Smaller test files — every test fn name | [unscheduled bbox exact=13/35] python test names surface in testing/test_invocations.py (25 atoms, too expensive at final margin) |
| 7.12 | 9946 | — | — | 0.31 | missing | Top-level meta — AGENTS.md, SECURITY.md, MANIFEST.in | [scheduled bbox exact=2/13] SECURITY.md section #0 (t=423, 2 atoms) |

### no discovered candidate

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 213 | — | — | 0.00 | missing | README lede + tagline | no discovered line candidate |
| 2.1 | 624 | — | — | 0.00 | missing | Toy example — first half (spec + plugin classes) | no discovered line candidate |
| 2.2 | 829 | — | — | 0.00 | missing | Toy example — second half (PluginManager wiring + call) | no discovered line candidate |
| 3.2 | 1261 | — | — | 0.00 | missing | docs/index.rst lede — "what is pluggy" | no discovered line candidate |
| 5.7 | 4801 | — | — | 0.00 | missing | docs/index.rst H2 heading map — sections at exact line numbers | no discovered line candidate |
| 6.2 | 5075 | — | — | 0.00 | missing | Eggsample hookspecs.py — host-side hook specifications | no discovered line candidate |
| 6.3 | 5479 | — | — | 0.00 | missing | Eggsample lib.py + eggsample-spam (host impls + plugin impls) | no discovered line candidate |
| 6.4 | 6020 | — | — | 0.00 | missing | docs/index.rst — "Call time order" section (tryfirst / trylast) | no discovered line candidate |
| 6.5 | 6517 | — | — | 0.00 | missing | docs/index.rst — Wrappers (new-style) wrapper-protocol summary | no discovered line candidate |
| 7.6 | 8324 | — | — | 0.00 | missing | pyproject.toml — [project] essentials (skip classifier list) | no discovered line candidate |
| 7.7 | 8576 | — | — | 0.00 | missing | pyproject.toml — [tool.ruff.lint] config | no discovered line candidate |
| 7.8 | 8859 | — | — | 0.00 | missing | tox.ini — [tox] + [testenv] + [pytest] (skip release/docs envs) | no discovered line candidate |
| 7.9 | 9238 | — | — | 0.00 | missing | CHANGELOG.rst — pluggy 1.6.0 entry only | no discovered line candidate |

### fs/listing

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 6.1 | 4855 | — | — | 0.00 | missing | docs/examples/ FS listing | fs-only |

### timing-only

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.3 | 256 | 766 | +510 | 1.00 | late | Source package layout (src/pluggy/) | fs-only |
| 1.4 | 406 | 1185 | +779 | 1.00 | late | __all__ — full public-name list | [scheduled bbox exact=16/16] python imports in src/pluggy/__init__.py (t=1185, 16 atoms) |
| 4.2 | 2037 | 8799 | +6762 | 1.00 | late | _hooks.py — every class + def name (markers, HookCaller, HookImpl, HookSpec) | [scheduled bbox exact=30/42] python method sigs in src/pluggy/_hooks.py (t=8799, 59 atoms) |
| 4.3 | 2138 | 5803 | +3665 | 1.00 | late | _callers.py — every def + _multicall signature | [scheduled bbox exact=5/9] python decl names surface in src/pluggy/_callers.py (t=1378, 8 atoms) |
| 4.4 | 2275 | 2443 | +168 | 1.00 | aligned+over | _result.py — Result class + every method (in full) | [scheduled bbox exact=10/12] python method sigs in src/pluggy/_result.py (t=2430, 13 atoms) |
| 4.6 | 2682 | 6213 | +3531 | 1.00 | late | _tracing.py — class + def names | [scheduled bbox exact=9/11] python method sigs in src/pluggy/_tracing.py (t=6213, 17 atoms) |
| 5.2 | 3038 | 5362 | +2324 | 0.83 | late | HookspecMarker + HookimplMarker class docstrings | [scheduled bbox exact=0/12] python method at src/pluggy/_hooks.py:111 (t=9697, 8 atoms) |
| 5.5 | 4080 | 7096 | +3016 | 0.91 | late | HookspecOpts + HookimplOpts — TypedDict bodies | [scheduled bbox exact=16/32] python class body at src/pluggy/_hooks.py:56 (t=7096, 16 atoms) |
| 7.1 | 6579 | 501 | -6078 | 1.00 | early | testing/ FS listing | fs-only |
| 7.10 | 9300 | 394 | -8906 | 1.00 | early | changelog/ + downstream/ FS listings | fs-only |
| 7.13 | 9964 | 135 | -9829 | 1.00 | early | scripts/ FS + .github/workflows/ FS | fs-only |
| 7.14 | 9996 | 198 | -9798 | 1.00 | early | docs/ root FS + docs/requirements.txt | fs-only |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 3 | 258 | CLAUDE.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 284 | 1.00 | 284 | 2876 | python decl names surface in docs/conf.py |
| 208 | 1.00 | 208 | 8217 | python imports in src/pluggy/_hooks.py |
| 201 | 1.00 | 201 | 4102 | python decl at testing/benchmark.py:54 |
| 191 | 0.65 | 294 | 4786 | python decl names surface in src/pluggy/_hooks.py |
| 161 | 1.00 | 161 | 7464 | python imports in src/pluggy/_callers.py |
| 158 | 1.00 | 158 | 4403 | python decl at docs/conf.py:66 |
| 152 | 1.00 | 152 | 7901 | python decl body at testing/benchmark.py:40 |
| 146 | 1.00 | 146 | 3901 | python decl at docs/conf.py:41 |
| 139 | 1.00 | 139 | 7603 | python decl body at scripts/release.py:15 |
| 136 | 1.00 | 136 | 3549 | python decl at docs/conf.py:96 |
| 2925 | — | — | — | +36 more rows |

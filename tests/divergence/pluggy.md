scores: Sim=0.425 Reached=14/40 Early=4 Late=6 Partial=4 Missing=22 Used=9938/10000

## Verdict

Verdict: coverage-gap bound
Likely primary lever: add walker candidates for no-discovered NS rows
Evidence: 2 ranking-recoverable (w×gap=0.41), 12 wrong-slice/granularity (w×gap=0.55), 11 no-discovered (w×gap=3.17)
Secondary intervention: free final budget for 2 too-expensive candidates
Loss reasons: 0 predecessor-gated, 2 too-expensive, 0 discovered-unscheduled
Top rows: 1.2, 2.1, 2.2, 3.2, 5.7, ...
Note: likely lever is heuristic; verify `Sim` moves, not just bucket counts.

## Top opportunities

_`w(t)×gap` is a non-additive priority score: Σ exp(-exp_t/τ) × (1 - credit) per row, τ=2000. Same time weighting and credit gap as Sim, but rows can overlap between opportunities — sums across rows are an upper bound on Sim impact, not an additive estimate._

| intervention | rows | w(t)×gap | bands ≤3k/≤6k/total | evidence | top row ids |
|:-------------|-----:|---------:|:----------------------|:---------|:------------|
| add walker candidates for no-discovered rows | 11 | 3.17 | 4/7/11 | NS rows have no discovered line candidate | 1.2, 2.1, 2.2, 3.2, 5.7, ... |
| split wrong-slice walker batches | 12 | 0.55 | 2/5/12 | nearby candidates have low exact atom overlap | 5.4, 4.5, 5.3, 5.1, 5.6, ... |
| free final budget / demote late waste | 2 | 0.41 | 1/1/2 | high-overlap candidates exceed final remaining budget, exact total=61/66 | 4.1, 7.3 |

Tiers: 1=3/4 reached, 0 partial, 1 missing, avg=0.75; 2=0/2 reached, 0 partial, 2 missing, avg=0.00; 3=1/2 reached, 0 partial, 1 missing, avg=0.50; 4=4/6 reached, 1 partial, 1 missing, avg=0.80; 5=2/7 reached, 2 partial, 3 missing, avg=0.50; 6=0/5 reached, 0 partial, 5 missing, avg=0.00; 7=4/14 reached, 1 partial, 9 missing, avg=0.37

## Diagnosis rollup

| diagnosis | rows | missing | partial | timing | likely lever |
|:----------|-----:|--------:|--------:|-------:|:-------------|
| ranking-recoverable | 2 | 2 | 0 | 0 | value/ranking |
| wrong-slice / granularity | 12 | 8 | 4 | 0 | walker granularity / wrong slice |
| no discovered candidate | 11 | 11 | 0 | 0 | walker coverage or predecessor-gated emit |
| fs/listing | 1 | 1 | 0 | 0 | filesystem/listing value |
| timing-only | 11 | 0 | 0 | 11 | usually no code change |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | w(t)×gap | likely lever |
|:------------|-----:|---------:|:-------------|
| too expensive at final margin | 2 | 0.41 | free final budget |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=16, unscheduled bbox=4, unscheduled same-file=1, fs-only=5, no discovered candidate=11

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | aligned | high | 1 |
| scheduled bbox | late | none | 1 |
| scheduled bbox | late | low | 3 |
| scheduled bbox | late | high | 1 |
| scheduled bbox | late | full | 1 |
| scheduled bbox | missing | none | 1 |
| scheduled bbox | missing | low | 4 |
| scheduled bbox | partial | low | 4 |
| unscheduled bbox | missing | low | 3 |
| unscheduled bbox | missing | full | 1 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 4.1 | 1595 | — | — | 0.15 | missing | _manager.py — every class + def name | [scheduled bbox exact=0/34] python decl doc at src/pluggy/_manager.py:83 (t=7345, 11 atoms); better unscheduled exact=29/34: python method sigs in src/pluggy/_manager.py (60 atoms, too expensive at final margin) |
| 7.3 | 7193 | — | — | 0.00 | missing | test_pluginmanager.py — every test fn name | [unscheduled bbox exact=32/32] python test names surface in testing/test_pluginmanager.py (63 atoms, too expensive at final margin) |

### wrong-slice / granularity

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 4.5 | 2581 | — | — | 0.63 | partial | _warnings.py — full file (Pluggy*Warning classes) | [scheduled bbox exact=10/27] python decl doc at src/pluggy/_warnings.py:10 (t=7633, 10 atoms) |
| 5.1 | 2873 | — | — | 0.73 | partial | PluginManager class docstring | [scheduled bbox exact=11/15] python decl doc at src/pluggy/_manager.py:83 (t=7345, 11 atoms) |
| 5.3 | 3369 | — | — | 0.55 | partial | HookCaller __slots__ + the 6-bucket call-order comment | [scheduled bbox exact=7/29] python method at src/pluggy/_hooks.py:393 (t=9341, 7 atoms) |
| 5.4 | 3651 | — | — | 0.05 | missing | HookCaller._add_hookimpl body — the actual ordering algorithm | [scheduled bbox exact=1/22] python method doc at src/pluggy/_hooks.py:453 (t=9109, 1 atoms); better unscheduled exact=10/22: python method body at src/pluggy/_hooks.py:453 body 466 (10 atoms, too expensive at final margin) |
| 5.6 | 4494 | — | — | 0.46 | missing | Result API — force_result, force_exception, get_result bodies | [scheduled bbox exact=6/37] python method doc at src/pluggy/_result.py:67 (t=8021, 6 atoms) |
| 7.2 | 6827 | — | — | 0.72 | partial | testing/conftest.py — pm + he_pm fixtures | [scheduled bbox exact=7/25] python decl body at testing/conftest.py:7 body 12 (t=5153, 7 atoms) |
| 7.4 | 7623 | — | — | 0.00 | missing | test_hookcaller.py + test_multicall.py — every test fn name | [unscheduled bbox exact=21/36] python test names surface in testing/test_multicall.py (41 atoms, too expensive at final margin) |
| 7.5 | 8023 | — | — | 0.00 | missing | Smaller test files — every test fn name | [unscheduled bbox exact=13/35] python test names surface in testing/test_invocations.py (25 atoms, too expensive at final margin) |
| 7.6 | 8324 | — | — | 0.00 | missing | pyproject.toml — [project] essentials (skip classifier list) | [unscheduled bbox exact=3/26] [package] in pyproject.toml (3 atoms, discovered unscheduled) |
| 7.7 | 8576 | — | — | 0.00 | missing | pyproject.toml — [tool.ruff.lint] config | [unscheduled same-file] [package] in pyproject.toml (3 atoms, discovered unscheduled) |
| 7.11 | 9819 | — | — | 0.09 | missing | _callers._multicall body — the actual call loop | [scheduled bbox exact=4/43] python decl doc at src/pluggy/_callers.py:82 (t=6945, 4 atoms); better unscheduled exact=31/43: python decl body at src/pluggy/_callers.py:82 body 97 (31 atoms, too expensive at final margin) |
| 7.12 | 9946 | — | — | 0.31 | missing | Top-level meta — AGENTS.md, SECURITY.md, MANIFEST.in | [scheduled bbox exact=2/13] SECURITY.md section #0 (t=546, 2 atoms) |

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
| 7.8 | 8859 | — | — | 0.00 | missing | tox.ini — [tox] + [testenv] + [pytest] (skip release/docs envs) | no discovered line candidate |
| 7.9 | 9238 | — | — | 0.00 | missing | CHANGELOG.rst — pluggy 1.6.0 entry only | no discovered line candidate |

### fs/listing

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 6.1 | 4855 | — | — | 0.00 | missing | docs/examples/ FS listing | fs-only |

### timing-only

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.4 | 406 | 1131 | +725 | 1.00 | late | __all__ — full public-name list | [scheduled bbox exact=16/16] python imports in src/pluggy/__init__.py (t=1131, 16 atoms) |
| 4.2 | 2037 | 8875 | +6838 | 1.00 | late | _hooks.py — every class + def name (markers, HookCaller, HookImpl, HookSpec) | [scheduled bbox exact=22/42] python method sigs #1 in src/pluggy/_hooks.py (t=8875, 43 atoms) |
| 4.3 | 2138 | 6166 | +4028 | 1.00 | late | _callers.py — every def + _multicall signature | [scheduled bbox exact=5/9] python decl names surface in src/pluggy/_callers.py (t=1378, 8 atoms) |
| 4.4 | 2275 | 2646 | +371 | 1.00 | aligned+over | _result.py — Result class + every method (in full) | [scheduled bbox exact=10/12] python method sigs in src/pluggy/_result.py (t=2633, 13 atoms) |
| 4.6 | 2682 | 6779 | +4097 | 1.00 | late | _tracing.py — class + def names | [scheduled bbox exact=9/11] python method sigs in src/pluggy/_tracing.py (t=6779, 17 atoms) |
| 5.2 | 3038 | 4026 | +988 | 0.83 | late | HookspecMarker + HookimplMarker class docstrings | [scheduled bbox exact=0/12] python method at src/pluggy/_hooks.py:111 (t=6461, 8 atoms) |
| 5.5 | 4080 | 7859 | +3779 | 0.91 | late | HookspecOpts + HookimplOpts — TypedDict bodies | [scheduled bbox exact=16/32] python class body at src/pluggy/_hooks.py:56 (t=7859, 16 atoms) |
| 7.1 | 6579 | 647 | -5932 | 1.00 | early | testing/ FS listing | fs-only |
| 7.10 | 9300 | 517 | -8783 | 1.00 | early | changelog/ + downstream/ FS listings | fs-only |
| 7.13 | 9964 | 135 | -9829 | 1.00 | early | scripts/ FS + .github/workflows/ FS | fs-only |
| 7.14 | 9996 | 259 | -9737 | 1.00 | early | docs/ root FS + docs/requirements.txt | fs-only |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 3 | 258 | CLAUDE.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 208 | 1.00 | 208 | 9876 | python imports in src/pluggy/_hooks.py |
| 201 | 1.00 | 201 | 4594 | python decl at testing/benchmark.py:54 |
| 161 | 1.00 | 161 | 8182 | python imports in src/pluggy/_callers.py |
| 158 | 1.00 | 158 | 5045 | python decl at docs/conf.py:66 |
| 152 | 1.00 | 152 | 8480 | python decl body at testing/benchmark.py:40 body 41 |
| 149 | 1.00 | 149 | 3203 | python decl names surface #1 in docs/conf.py |
| 146 | 1.00 | 146 | 4393 | python decl at docs/conf.py:41 |
| 136 | 1.00 | 136 | 3475 | python decl at docs/conf.py:96 |
| 135 | 1.00 | 135 | 2069 | python decl names surface in docs/conf.py |
| 131 | 0.72 | 181 | 3809 | python decl names surface in src/pluggy/_hooks.py |
| 2980 | — | — | — | +37 more rows |

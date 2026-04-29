scores: Score(3000)=0.455 ns_rows≤3K=15/40 (reached=5 partial=0 missing=10)

## Per-budget scores

| B | A_B | I(B) | C(B) | Score(B) | walker_used |
|--:|----:|-----:|-----:|---------:|------------:|
| 1000 | 113 | 0.768 | 0.292 | 0.474 | 687 |
| 1442 | 132 | 0.830 | 0.470 | 0.624 | 1389 |
| 2080 | 208 | 0.770 | 0.298 | 0.479 | 2057 |
| 3000 | 282 | 0.742 | 0.279 | 0.455 | 2974 |
| 4327 | 377 | 0.710 | 0.209 | 0.385 | 4272 |
| 6240 | 576 | 0.691 | 0.223 | 0.392 | 6232 |
| 9000 | 820 | 0.709 | 0.304 | 0.464 | 8988 |

## Verdict

Verdict: wrong-slice bound
Likely primary lever: split walker batches to match NS semantic slices
Evidence: 2 ranking-recoverable (gap@3k=0.27), 20 wrong-slice/granularity (gap@3k=1.71), 7 no-discovered (gap@3k=0.77)
Secondary intervention: investigate 7 no-discovered rows
Top rows: 2.1, 4.2, 2.2, 4.5, 5.3, ...

## Top opportunities

_`gap@B` is a non-additive priority score: `Σ over atoms in row: (1 − damped_credit(a)) / rank(a)` evaluated at budget B's walker state. Approximates how much closing the row would lift `Score(B)` (via the Importance numerator); not an exact delta. `gap@3k` is the primary sort key. `gap@1k` and `gap@9k` show how the same intervention scales across the budget vector — gap is monotone non-increasing in B (walker has more budget at higher B). Rows can overlap between opportunities; sums are upper bounds on Score(B) impact, not additive estimates._

| intervention | rows | gap@1k | gap@3k | gap@9k | evidence | top row ids |
|:-------------|-----:|-------:|-------:|-------:|:---------|:------------|
| split wrong-slice walker batches | 20 | 1.72 | 1.71 | 1.20 | nearby candidates have low exact atom overlap | 2.1, 4.2, 2.2, 4.5, 5.3, ... |
| add walker candidates for no-discovered rows | 7 | 0.77 | 0.77 | 0.77 | NS rows have no discovered line candidate | 1.2, 3.2, 6.4, 5.7, 6.5, ... |
| tune ranking for high-overlap unscheduled candidates | 2 | 0.28 | 0.27 | 0.27 | high-overlap candidates not in the schedule by T_max, exact total=61/66 | 4.1, 7.3 |

## Diagnosis rollup

| diagnosis | rows | missing | partial | likely lever |
|:----------|-----:|--------:|--------:|:-------------|
| ranking-recoverable | 2 | 2 | 0 | value/ranking |
| wrong-slice / granularity | 20 | 20 | 0 | walker granularity / wrong slice |
| no discovered candidate | 7 | 7 | 0 | walker coverage or predecessor-gated emit |
| fs/listing | 1 | 0 | 1 | filesystem/listing value |
| mixed/unknown | 1 | 1 | 0 | inspect row |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | gap@3k | likely lever |
|:------------|-----:|-------:|:-------------|
| too expensive at final margin | 2 | 0.27 | tune ranking |

Candidate hint kinds: scheduled bbox=14, unscheduled bbox=8, unscheduled same-file=1, fs-only=1, no discovered candidate=7 _(candidates are walker batches discovered this run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` isn't proof that no emit path exists)._

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | missing | none | 2 |
| scheduled bbox | missing | low | 11 |
| scheduled bbox | missing | high | 1 |
| unscheduled bbox | missing | low | 7 |
| unscheduled bbox | missing | full | 1 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 4.1 | 1595 | 0.15 | 0.15 | missing | _manager.py — every class + def name | [scheduled bbox exact=0/34] python decl doc at src/pluggy/_manager.py:83 (t=7396, 11 atoms); better unscheduled exact=29/34: python method sigs in src/pluggy/_manager.py (60 atoms, too expensive at final margin) |
| 7.3 | 7193 | 0.00 | 0.00 | missing | test_pluginmanager.py — every test fn name | [unscheduled bbox exact=32/32] python test names surface in testing/test_pluginmanager.py (63 atoms, too expensive at final margin) |

### wrong-slice / granularity

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 2.1 | 624 | 0.00 | 0.00 | missing | Toy example — first half (spec + plugin classes) | [unscheduled bbox exact=6/23] python decl names surface in docs/examples/toy-example.py (6 atoms, too expensive at final margin) |
| 2.2 | 829 | 0.00 | 0.00 | missing | Toy example — second half (PluginManager wiring + call) | [unscheduled bbox exact=4/19] python decl names surface in docs/examples/toy-example.py (4 atoms, too expensive at final margin) |
| 4.2 | 2037 | 0.00 | 0.00 | missing | _hooks.py — every class + def name (markers, HookCaller, HookImpl, HookSpec) | [scheduled bbox exact=22/42] python method sigs #1 in src/pluggy/_hooks.py (t=8944, 43 atoms) |
| 4.3 | 2138 | 0.45 | 0.40 | missing | _callers.py — every def + _multicall signature | [scheduled bbox exact=5/9] python decl names surface in src/pluggy/_callers.py (t=1457, 8 atoms) |
| 4.5 | 2581 | 0.26 | 0.27 | missing | _warnings.py — full file (Pluggy*Warning classes) | [scheduled bbox exact=10/27] python decl doc at src/pluggy/_warnings.py:10 (t=7684, 10 atoms) |
| 5.1 | 2873 | 0.00 | 0.00 | missing | PluginManager class docstring | [scheduled bbox exact=11/15] python decl doc at src/pluggy/_manager.py:83 (t=7396, 11 atoms) |
| 5.2 | 3038 | 0.00 | 0.00 | missing | HookspecMarker + HookimplMarker class docstrings | [scheduled bbox exact=0/12] python method at src/pluggy/_hooks.py:111 (t=6135, 8 atoms) |
| 5.3 | 3369 | 0.00 | 0.00 | missing | HookCaller __slots__ + the 6-bucket call-order comment | [scheduled bbox exact=7/29] python method at src/pluggy/_hooks.py:393 (t=9410, 7 atoms) |
| 5.4 | 3651 | 0.00 | 0.00 | missing | HookCaller._add_hookimpl body — the actual ordering algorithm | [scheduled bbox exact=1/22] python method doc at src/pluggy/_hooks.py:453 (t=9178, 1 atoms); better unscheduled exact=10/22: python method body at src/pluggy/_hooks.py:453 body 466 (10 atoms, too expensive at final margin) |
| 5.5 | 4080 | 0.00 | 0.00 | missing | HookspecOpts + HookimplOpts — TypedDict bodies | [scheduled bbox exact=16/32] python class body at src/pluggy/_hooks.py:56 (t=7910, 16 atoms) |
| 5.6 | 4494 | 0.08 | 0.13 | missing | Result API — force_result, force_exception, get_result bodies | [scheduled bbox exact=6/37] python method doc at src/pluggy/_result.py:67 (t=8072, 6 atoms) |
| 6.2 | 5075 | 0.00 | 0.00 | missing | Eggsample hookspecs.py — host-side hook specifications | [unscheduled bbox exact=5/22] python decl names surface in docs/examples/eggsample/eggsample/hookspecs.py (5 atoms, discovered unscheduled) |
| 6.3 | 5479 | 0.00 | 0.00 | missing | Eggsample lib.py + eggsample-spam (host impls + plugin impls) | [unscheduled bbox exact=4/36] python decl body at docs/examples/eggsample/eggsample/lib.py:4 body 6 (4 atoms, predecessor not scheduled: python decl at docs/examples/eggsample/eggsample/lib.py:4) |
| 7.2 | 6827 | 0.44 | 0.59 | missing | testing/conftest.py — pm + he_pm fixtures | [scheduled bbox exact=7/25] python decl body at testing/conftest.py:7 body 12 (t=4519, 7 atoms) |
| 7.4 | 7623 | 0.00 | 0.00 | missing | test_hookcaller.py + test_multicall.py — every test fn name | [unscheduled bbox exact=21/36] python test names surface in testing/test_multicall.py (41 atoms, too expensive at final margin) |
| 7.5 | 8023 | 0.00 | 0.00 | missing | Smaller test files — every test fn name | [unscheduled bbox exact=13/35] python test names surface in testing/test_invocations.py (25 atoms, too expensive at final margin) |
| 7.6 | 8324 | 0.00 | 0.00 | missing | pyproject.toml — [project] essentials (skip classifier list) | [unscheduled bbox exact=3/26] [package] in pyproject.toml (3 atoms, discovered unscheduled) |
| 7.7 | 8576 | 0.00 | 0.00 | missing | pyproject.toml — [tool.ruff.lint] config | [unscheduled same-file] [package] in pyproject.toml (3 atoms, discovered unscheduled) |
| 7.11 | 9819 | 0.00 | 0.00 | missing | _callers._multicall body — the actual call loop | [scheduled bbox exact=4/43] python decl doc at src/pluggy/_callers.py:82 (t=6340, 4 atoms); better unscheduled exact=31/43: python decl body at src/pluggy/_callers.py:82 body 97 (31 atoms, too expensive at final margin) |
| 7.12 | 9946 | 0.31 | 0.56 | missing | Top-level meta — AGENTS.md, SECURITY.md, MANIFEST.in | [scheduled bbox exact=2/13] SECURITY.md section #0 (t=484, 2 atoms) |

### no discovered candidate

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 1.2 | 213 | 0.00 | 0.00 | missing | README lede + tagline | no discovered line candidate |
| 3.2 | 1261 | 0.00 | 0.00 | missing | docs/index.rst lede — "what is pluggy" | no discovered line candidate |
| 5.7 | 4801 | 0.00 | 0.00 | missing | docs/index.rst H2 heading map — sections at exact line numbers | no discovered line candidate |
| 6.4 | 6020 | 0.00 | 0.00 | missing | docs/index.rst — "Call time order" section (tryfirst / trylast) | no discovered line candidate |
| 6.5 | 6517 | 0.00 | 0.00 | missing | docs/index.rst — Wrappers (new-style) wrapper-protocol summary | no discovered line candidate |
| 7.8 | 8859 | 0.00 | 0.00 | missing | tox.ini — [tox] + [testenv] + [pytest] (skip release/docs envs) | no discovered line candidate |
| 7.9 | 9238 | 0.00 | 0.00 | missing | CHANGELOG.rst — pluggy 1.6.0 entry only | no discovered line candidate |

### fs/listing

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 6.1 | 4855 | 0.82 | 0.82 | partial | docs/examples/ FS listing | fs-only |

### mixed/unknown

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 4.6 | 2682 | 0.19 | 0.20 | missing | _tracing.py — class + def names | [scheduled bbox exact=9/11] python method sigs in src/pluggy/_tracing.py (t=5016, 17 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 3 | 258 | CLAUDE.md section #<n> |

## Walker waste, primary-actionable (first_t ≤ 3000, off-NS spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 135 | 1.00 | 135 | 2319 | python decl names surface in docs/conf.py |
| 129 | 1.00 | 129 | 1787 | python decl names surface in testing/benchmark.py |
| 101 | 1.00 | 101 | 392 | headings outline in CLAUDE.md |
| 100 | 1.00 | 100 | 2946 | python imports in testing/benchmark.py |
| 84 | 1.00 | 84 | 1171 | python decl names surface in scripts/release.py |
| 79 | 1.00 | 79 | 2184 | python imports in scripts/release.py |
| 68 | 1.00 | 68 | 2387 | python decl at docs/conf.py:9 |
| 64 | 1.00 | 64 | 1957 | python decl doc at scripts/towncrier-draft-to-file.py:5 |
| 59 | 1.00 | 59 | 3033 | python imports in src/pluggy/_tracing.py |
| 57 | 0.83 | 69 | 1586 | python decl names surface in src/pluggy/_result.py |
| 56 | — | — | — | +1 more rows |

## Walker waste, late (first_t > 3000, off-NS spend ≥ 50) — higher-budget calibration only

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 208 | 1.00 | 208 | 9945 | python imports in src/pluggy/_hooks.py |
| 201 | 1.00 | 201 | 3486 | python decl at testing/benchmark.py:54 |
| 161 | 1.00 | 161 | 8233 | python imports in src/pluggy/_callers.py |
| 158 | 1.00 | 158 | 4430 | python decl at docs/conf.py:66 |
| 152 | 1.00 | 152 | 8549 | python decl body at testing/benchmark.py:40 body 41 |
| 149 | 1.00 | 149 | 3854 | python decl names surface #1 in docs/conf.py |
| 146 | 1.00 | 146 | 4272 | python decl at docs/conf.py:41 |
| 136 | 1.00 | 136 | 4126 | python decl at docs/conf.py:96 |
| 131 | 0.72 | 181 | 5266 | python decl names surface in src/pluggy/_hooks.py |
| 117 | 1.00 | 117 | 7513 | python decl body at scripts/release.py:59 body 60 |
| 2004 | — | — | — | +25 more rows |

scores: Score(3000)=0.455 ns_rows≤3K=15/40 (reached=5 partial=0 missing=10)

## Per-budget scores

| B | A_B | I(B) | C(B) | compl(B) | Score(B) | walker_used |
|--:|----:|-----:|-----:|---------:|---------:|------------:|
| 1000 | 113 | 0.768 | 0.292 | 1.000 | 0.474 | 687 |
| 1442 | 132 | 0.830 | 0.470 | 1.000 | 0.624 | 1389 |
| 2080 | 208 | 0.770 | 0.298 | 1.000 | 0.479 | 2057 |
| 3000 | 282 | 0.742 | 0.279 | 0.602 | 0.455 | 2974 |
| 4327 | 377 | 0.710 | 0.209 | 0.602 | 0.385 | 4272 |
| 6240 | 576 | 0.691 | 0.223 | 0.694 | 0.392 | 6232 |
| 9000 | 820 | 0.709 | 0.304 | 0.831 | 0.464 | 8988 |

## Top opportunities

### Additive (close partial / missing rows)

| intervention | rows | gap@1k | gap@3k | gap@9k | evidence | top row ids |
|:-------------|-----:|-------:|-------:|-------:|:---------|:------------|
| split wrong-slice walker batches | 20 | 1.72 | 1.71 | 1.20 | nearby candidates have low exact atom overlap | 2.1, 4.2, 2.2, 4.5, 5.3, ... |
| add walker candidates for no-discovered rows | 7 | 0.77 | 0.77 | 0.77 | NS rows have no discovered line candidate | 1.2, 3.2, 6.4, 5.7, 6.5, ... |
| tune ranking for high-overlap unscheduled candidates | 2 | 0.28 | 0.27 | 0.27 | high-overlap candidates not in the schedule by T_max, exact total=61/66 | 4.1, 7.3 |

### Subtractive (suppress consistently off-NS batches)

| pattern | batches | freed@1k | freed@3k | freed@9k | evidence | top batch ids |
|:--------|--------:|---------:|---------:|---------:|:---------|:--------------|
| python decl names surface in docs/conf.py | 1 | 0 | 135 | 135 | off_3k=135 | python decl names surface in docs/conf.py |
| python decl names surface in testing/benchmark.py | 1 | 0 | 129 | 129 | off_3k=129 | python decl names surface in testing/benchmark.py |
| headings outline in CLAUDE.md | 1 | 101 | 101 | 101 | off_3k=101 | headings outline in CLAUDE.md |
| python imports in testing/benchmark.py | 1 | 0 | 100 | 100 | off_3k=100 | python imports in testing/benchmark.py |
| python decl names surface in scripts/release.py | 1 | 0 | 84 | 84 | off_3k=84 | python decl names surface in scripts/release.py |

Top missed paths (NS rows ≤ 3K): src/pluggy/_manager.py (2 rows, 49 atoms), docs/examples/toy-example.py (2 rows, 42 atoms), src/pluggy/_hooks.py (1 row, 42 atoms), src/pluggy/_warnings.py (1 row, 27 atoms), docs/index.rst (1 row, 19 atoms), src/pluggy/_tracing.py (1 row, 11 atoms), README.rst (1 row, 9 atoms), src/pluggy/_callers.py (1 row, 9 atoms)

## Arrival ledger by diagnosis

### ranking-recoverable

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 4.1 | 1595 | 0.15 | missing | _manager.py — every class + def name | [scheduled bbox exact=0/34] python decl doc at src/pluggy/_manager.py:83 (t=7396, 11 atoms); better unscheduled exact=29/34: python method sigs in src/pluggy/_manager.py (60 atoms, too expensive at final margin) |
| 7.3 | 7193 | 0.00 | missing | test_pluginmanager.py — every test fn name | [unscheduled bbox exact=32/32] python test names surface in testing/test_pluginmanager.py (63 atoms, too expensive at final margin) |

### wrong-slice / granularity

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 2.1 | 624 | 0.00 | missing | Toy example — first half (spec + plugin classes) | [unscheduled bbox exact=6/23] python decl names surface in docs/examples/toy-example.py (6 atoms, too expensive at final margin) |
| 2.2 | 829 | 0.00 | missing | Toy example — second half (PluginManager wiring + call) | [unscheduled bbox exact=4/19] python decl names surface in docs/examples/toy-example.py (4 atoms, too expensive at final margin) |
| 4.2 | 2037 | 0.00 | missing | _hooks.py — every class + def name (markers, HookCaller, HookImpl, HookSpec) | [scheduled bbox exact=22/42] python method sigs #1 in src/pluggy/_hooks.py (t=8944, 43 atoms) |
| 4.3 | 2138 | 0.45 | missing | _callers.py — every def + _multicall signature | [scheduled bbox exact=5/9] python decl names surface in src/pluggy/_callers.py (t=1457, 8 atoms) |
| 4.5 | 2581 | 0.26 | missing | _warnings.py — full file (Pluggy*Warning classes) | [scheduled bbox exact=10/27] python decl doc at src/pluggy/_warnings.py:10 (t=7684, 10 atoms) |
| 5.1 | 2873 | 0.00 | missing | PluginManager class docstring | [scheduled bbox exact=11/15] python decl doc at src/pluggy/_manager.py:83 (t=7396, 11 atoms) |
| 5.2 | 3038 | 0.00 | missing | HookspecMarker + HookimplMarker class docstrings | [scheduled bbox exact=0/12] python method at src/pluggy/_hooks.py:111 (t=6135, 8 atoms) |
| 5.3 | 3369 | 0.00 | missing | HookCaller __slots__ + the 6-bucket call-order comment | [scheduled bbox exact=7/29] python method at src/pluggy/_hooks.py:393 (t=9410, 7 atoms) |
| 5.4 | 3651 | 0.00 | missing | HookCaller._add_hookimpl body — the actual ordering algorithm | [scheduled bbox exact=1/22] python method doc at src/pluggy/_hooks.py:453 (t=9178, 1 atoms); better unscheduled exact=10/22: python method body at src/pluggy/_hooks.py:453 body 466 (10 atoms, too expensive at final margin) |
| 5.5 | 4080 | 0.00 | missing | HookspecOpts + HookimplOpts — TypedDict bodies | [scheduled bbox exact=16/32] python class body at src/pluggy/_hooks.py:56 (t=7910, 16 atoms) |
| 5.6 | 4494 | 0.08 | missing | Result API — force_result, force_exception, get_result bodies | [scheduled bbox exact=6/37] python method doc at src/pluggy/_result.py:67 (t=8072, 6 atoms) |
| 6.2 | 5075 | 0.00 | missing | Eggsample hookspecs.py — host-side hook specifications | [unscheduled bbox exact=5/22] python decl names surface in docs/examples/eggsample/eggsample/hookspecs.py (5 atoms, discovered unscheduled) |
| 6.3 | 5479 | 0.00 | missing | Eggsample lib.py + eggsample-spam (host impls + plugin impls) | [unscheduled bbox exact=4/36] python decl body at docs/examples/eggsample/eggsample/lib.py:4 body 6 (4 atoms, predecessor not scheduled: python decl at docs/examples/eggsample/eggsample/lib.py:4) |
| 7.2 | 6827 | 0.44 | missing | testing/conftest.py — pm + he_pm fixtures | [scheduled bbox exact=7/25] python decl body at testing/conftest.py:7 body 12 (t=4519, 7 atoms) |
| 7.4 | 7623 | 0.00 | missing | test_hookcaller.py + test_multicall.py — every test fn name | [unscheduled bbox exact=21/36] python test names surface in testing/test_multicall.py (41 atoms, too expensive at final margin) |
| 7.5 | 8023 | 0.00 | missing | Smaller test files — every test fn name | [unscheduled bbox exact=13/35] python test names surface in testing/test_invocations.py (25 atoms, too expensive at final margin) |
| 7.6 | 8324 | 0.00 | missing | pyproject.toml — [project] essentials (skip classifier list) | [unscheduled bbox exact=3/26] [package] in pyproject.toml (3 atoms, discovered unscheduled) |
| 7.7 | 8576 | 0.00 | missing | pyproject.toml — [tool.ruff.lint] config | [unscheduled same-file] [package] in pyproject.toml (3 atoms, discovered unscheduled) |
| 7.11 | 9819 | 0.00 | missing | _callers._multicall body — the actual call loop | [scheduled bbox exact=4/43] python decl doc at src/pluggy/_callers.py:82 (t=6340, 4 atoms); better unscheduled exact=31/43: python decl body at src/pluggy/_callers.py:82 body 97 (31 atoms, too expensive at final margin) |
| 7.12 | 9946 | 0.31 | missing | Top-level meta — AGENTS.md, SECURITY.md, MANIFEST.in | [scheduled bbox exact=2/13] SECURITY.md section #0 (t=484, 2 atoms) |

### no discovered candidate

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 1.2 | 213 | 0.00 | missing | README lede + tagline | no discovered line candidate |
| 3.2 | 1261 | 0.00 | missing | docs/index.rst lede — "what is pluggy" | no discovered line candidate |
| 5.7 | 4801 | 0.00 | missing | docs/index.rst H2 heading map — sections at exact line numbers | no discovered line candidate |
| 6.4 | 6020 | 0.00 | missing | docs/index.rst — "Call time order" section (tryfirst / trylast) | no discovered line candidate |
| 6.5 | 6517 | 0.00 | missing | docs/index.rst — Wrappers (new-style) wrapper-protocol summary | no discovered line candidate |
| 7.8 | 8859 | 0.00 | missing | tox.ini — [tox] + [testenv] + [pytest] (skip release/docs envs) | no discovered line candidate |
| 7.9 | 9238 | 0.00 | missing | CHANGELOG.rst — pluggy 1.6.0 entry only | no discovered line candidate |

### fs/listing

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 6.1 | 4855 | 0.82 | partial | docs/examples/ FS listing | fs-only |

### mixed/unknown

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 4.6 | 2682 | 0.19 | missing | _tracing.py — class + def names | [scheduled bbox exact=9/11] python method sigs in src/pluggy/_tracing.py (t=5016, 17 atoms) |

Top wasted paths (off-NS at 3K): testing/benchmark.py (229t, 2 batches), docs/conf.py (203t, 2 batches), scripts/release.py (163t, 2 batches), CLAUDE.md (101t, 1 batch), scripts/towncrier-draft-to-file.py (64t, 1 batch), testing (62t, 1 batch), src/pluggy/_tracing.py (59t, 1 batch), src/pluggy/_result.py (57t, 1 batch), +2 more

## Walker waste (walker_t ≤ 3000, off_3k ≥ 50)

| off_3k | off_3k_ratio | off_any | cost | walker_t | batch |
|-------:|-------------:|--------:|-----:|---------:|:------|
| 135 | 1.00 | 135 | 135 | 2184 | python decl names surface in docs/conf.py |
| 129 | 1.00 | 129 | 129 | 1658 | python decl names surface in testing/benchmark.py |
| 101 | 1.00 | 101 | 101 | 291 | headings outline in CLAUDE.md |
| 100 | 1.00 | 100 | 100 | 2846 | python imports in testing/benchmark.py |
| 84 | 1.00 | 84 | 84 | 1087 | python decl names surface in scripts/release.py |
| 79 | 1.00 | 79 | 79 | 2105 | python imports in scripts/release.py |
| 68 | 1.00 | 68 | 68 | 2319 | python decl at docs/conf.py:9 |
| 64 | 1.00 | 64 | 64 | 1893 | python decl doc at scripts/towncrier-draft-to-file.py:5 |
| 62 | 1.00 | 0 | 62 | 585 | listing of 'testing' |
| 59 | 1.00 | 59 | 59 | 2974 | python imports in src/pluggy/_tracing.py |
| 167 | — | — | — | — | +3 more rows |

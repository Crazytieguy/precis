scores: Sim=0.332 Reached=14/40 Early=4 Late=8 Partial=5 Missing=21 Used=9991/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 4 | 3 | 0 | 1 | 0.75 |
| 2 | 2 | 0 | 0 | 2 | 0.00 |
| 3 | 2 | 1 | 0 | 1 | 0.50 |
| 4 | 6 | 4 | 1 | 1 | 0.80 |
| 5 | 7 | 2 | 3 | 2 | 0.53 |
| 6 | 5 | 0 | 0 | 5 | 0.00 |
| 7 | 14 | 4 | 1 | 9 | 0.37 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 213 | — | — | 0.00 | missing | README lede + tagline |  |
| 1.3 | 256 | 766 | +510 | 1.00 | late | Source package layout (src/pluggy/) |  |
| 1.4 | 406 | 3901 | +3495 | 1.00 | late | __all__ — full public-name list | python imports in src/pluggy/__init__.py (t=3901, 16 atoms) |
| 2.1 | 624 | — | — | 0.00 | missing | Toy example — first half (spec + plugin classes) |  |
| 2.2 | 829 | — | — | 0.00 | missing | Toy example — second half (PluginManager wiring + call) |  |
| 3.1 | 995 | 3901 | +2906 | 1.00 | late | __init__.py re-exports — name → owning module | python imports in src/pluggy/__init__.py (t=3901, 13 atoms) |
| 3.2 | 1261 | — | — | 0.00 | missing | docs/index.rst lede — "what is pluggy" |  |
| 4.1 | 1595 | — | — | 0.15 | missing | _manager.py — every class + def name | python decl doc at src/pluggy/_manager.py:83 (t=6582, 11 atoms) |
| 4.2 | 2037 | 8799 | +6762 | 1.00 | late | _hooks.py — every class + def name (markers, HookCaller, HookImpl, HookSpec) | python method sigs in src/pluggy/_hooks.py (t=8799, 59 atoms) |
| 4.3 | 2138 | 5803 | +3665 | 1.00 | late | _callers.py — every def + _multicall signature | python decl names surface in src/pluggy/_callers.py (t=1062, 8 atoms) |
| 4.4 | 2275 | 2127 | -148 | 1.00 | aligned+over | _result.py — Result class + every method (in full) | python method sigs in src/pluggy/_result.py (t=2114, 13 atoms) |
| 4.5 | 2581 | — | — | 0.63 | partial | _warnings.py — full file (Pluggy*Warning classes) | python decl doc at src/pluggy/_warnings.py:10 (t=6870, 10 atoms) |
| 4.6 | 2682 | 6213 | +3531 | 1.00 | late | _tracing.py — class + def names | python method sigs in src/pluggy/_tracing.py (t=6213, 17 atoms) |
| 5.1 | 2873 | — | — | 0.73 | partial | PluginManager class docstring | python decl doc at src/pluggy/_manager.py:83 (t=6582, 11 atoms) |
| 5.2 | 3038 | 5362 | +2324 | 0.83 | late | HookspecMarker + HookimplMarker class docstrings | python method sigs in src/pluggy/_hooks.py (t=8799, 8 atoms) |
| 5.3 | 3369 | — | — | 0.55 | partial | HookCaller __slots__ + the 6-bucket call-order comment | python class body at src/pluggy/_hooks.py:382 (t=5089, 7 atoms) |
| 5.4 | 3651 | — | — | 0.05 | missing | HookCaller._add_hookimpl body — the actual ordering algorithm | python method sigs in src/pluggy/_hooks.py (t=8799, 1 atoms) |
| 5.5 | 4080 | 7096 | +3016 | 0.91 | late | HookspecOpts + HookimplOpts — TypedDict bodies | python class body at src/pluggy/_hooks.py:56 (t=7096, 16 atoms) |
| 5.6 | 4494 | — | — | 0.62 | partial | Result API — force_result, force_exception, get_result bodies | python method sigs in src/pluggy/_result.py (t=2114, 6 atoms) |
| 5.7 | 4801 | — | — | 0.00 | missing | docs/index.rst H2 heading map — sections at exact line numbers |  |
| 6.1 | 4855 | — | — | 0.00 | missing | docs/examples/ FS listing |  |
| 6.2 | 5075 | — | — | 0.00 | missing | Eggsample hookspecs.py — host-side hook specifications |  |
| 6.3 | 5479 | — | — | 0.00 | missing | Eggsample lib.py + eggsample-spam (host impls + plugin impls) |  |
| 6.4 | 6020 | — | — | 0.00 | missing | docs/index.rst — "Call time order" section (tryfirst / trylast) |  |
| 6.5 | 6517 | — | — | 0.00 | missing | docs/index.rst — Wrappers (new-style) wrapper-protocol summary |  |
| 7.1 | 6579 | 501 | -6078 | 1.00 | early | testing/ FS listing |  |
| 7.2 | 6827 | — | — | 0.72 | partial | testing/conftest.py — pm + he_pm fixtures | python decl body at testing/conftest.py:7 (t=4492, 7 atoms) |
| 7.3 | 7193 | — | — | 0.00 | missing | test_pluginmanager.py — every test fn name |  |
| 7.4 | 7623 | — | — | 0.00 | missing | test_hookcaller.py + test_multicall.py — every test fn name |  |
| 7.5 | 8023 | — | — | 0.00 | missing | Smaller test files — every test fn name |  |
| 7.6 | 8324 | — | — | 0.00 | missing | pyproject.toml — [project] essentials (skip classifier list) |  |
| 7.7 | 8576 | — | — | 0.00 | missing | pyproject.toml — [tool.ruff.lint] config |  |
| 7.8 | 8859 | — | — | 0.00 | missing | tox.ini — [tox] + [testenv] + [pytest] (skip release/docs envs) |  |
| 7.9 | 9238 | — | — | 0.00 | missing | CHANGELOG.rst — pluggy 1.6.0 entry only |  |
| 7.10 | 9300 | 394 | -8906 | 1.00 | early | changelog/ + downstream/ FS listings |  |
| 7.11 | 9819 | — | — | 0.09 | missing | _callers._multicall body — the actual call loop | python decl doc at src/pluggy/_callers.py:82 (t=6413, 4 atoms) |
| 7.12 | 9946 | — | — | 0.31 | missing | Top-level meta — AGENTS.md, SECURITY.md, MANIFEST.in | AGENTS.md section #0 (t=230, 2 atoms) |
| 7.13 | 9964 | 135 | -9829 | 1.00 | early | scripts/ FS + .github/workflows/ FS |  |
| 7.14 | 9996 | 198 | -9798 | 1.00 | early | docs/ root FS + docs/requirements.txt |  |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 3 | 258 | CLAUDE.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 284 | 1.00 | 284 | 2560 | python decl names surface in docs/conf.py |
| 208 | 1.00 | 208 | 8217 | python imports in src/pluggy/_hooks.py |
| 201 | 1.00 | 201 | 4102 | python decl at testing/benchmark.py:54 |
| 191 | 0.65 | 294 | 4786 | python decl names surface in src/pluggy/_hooks.py |
| 161 | 1.00 | 161 | 7464 | python imports in src/pluggy/_callers.py |
| 158 | 1.00 | 158 | 4403 | python decl at docs/conf.py:66 |
| 152 | 1.00 | 152 | 7901 | python decl body at testing/benchmark.py:40 |
| 146 | 1.00 | 146 | 3585 | python decl at docs/conf.py:41 |
| 139 | 1.00 | 139 | 7603 | python decl body at scripts/release.py:15 |
| 136 | 1.00 | 136 | 3233 | python decl at docs/conf.py:96 |
| 129 | 1.00 | 129 | 1327 | python decl names surface in testing/benchmark.py |
| 121 | 0.23 | 518 | 8799 | python method sigs in src/pluggy/_hooks.py |
| 117 | 1.00 | 117 | 6699 | python decl body at scripts/release.py:59 |
| 112 | 1.00 | 112 | 5474 | python class body at src/pluggy/_hooks.py:638 |
| 101 | 1.00 | 101 | 331 | headings outline in CLAUDE.md |
| 101 | 1.00 | 101 | 6003 | python imports in src/pluggy/_result.py |
| 100 | 1.00 | 100 | 2864 | python imports in testing/benchmark.py |
| 99 | 1.00 | 99 | 5902 | CLAUDE.md section #2 |
| 97 | 1.00 | 97 | 7700 | json config .claude/settings.json |
| 95 | 1.00 | 95 | 9697 | python method at src/pluggy/_hooks.py:111 |
| 91 | 0.87 | 105 | 9975 | python method at src/pluggy/_hooks.py:190 |
| 90 | 0.93 | 97 | 9859 | python method at src/pluggy/_hooks.py:178 |
| 88 | 1.00 | 88 | 5177 | python class body at src/pluggy/_hooks.py:696 |
| 87 | 1.00 | 87 | 4189 | CLAUDE.md section #3 |
| 84 | 1.00 | 84 | 625 | python decl names surface in scripts/release.py |
| 81 | 1.00 | 81 | 2764 | python decl at docs/conf.py:83 |
| 79 | 1.00 | 79 | 3439 | python decl body at scripts/release.py:39 |
| 79 | 1.00 | 79 | 1985 | python imports in scripts/release.py |
| 78 | 1.00 | 78 | 3311 | python decl body at scripts/release.py:50 |
| 76 | 0.84 | 90 | 9586 | python method at src/pluggy/_hooks.py:101 |
| 75 | 1.00 | 75 | 7171 | python decl body at src/pluggy/_hooks.py:281 |
| 75 | 0.91 | 82 | 9472 | python method at src/pluggy/_hooks.py:91 |
| 74 | 1.00 | 74 | 3097 | python decl body at scripts/release.py:30 |
| 72 | 1.00 | 72 | 3023 | CLAUDE.md section #6 |
| 72 | 1.00 | 72 | 5546 | python decl doc at src/pluggy/_hooks.py:293 |
| 68 | 1.00 | 68 | 2683 | python decl at docs/conf.py:9 |
| 65 | 1.00 | 65 | 9762 | python method doc at src/pluggy/_hooks.py:499 |
| 64 | 1.00 | 64 | 1618 | python decl doc at scripts/towncrier-draft-to-file.py:5 |
| 64 | 1.00 | 64 | 8281 | python method body at src/pluggy/_result.py:31 |
| 61 | 1.00 | 61 | 9209 | python method at src/pluggy/_hooks.py:656 |
| 59 | 1.00 | 59 | 2951 | python imports in src/pluggy/_tracing.py |
| 57 | 0.83 | 69 | 1160 | python decl names surface in src/pluggy/_result.py |
| 56 | 1.00 | 56 | 1489 | python decl body at src/pluggy/__init__.py:32 |
| 55 | 1.00 | 55 | 8009 | python method body at src/pluggy/_tracing.py:51 |
| 53 | 1.00 | 53 | 7954 | python method body at src/pluggy/_result.py:42 |
| 50 | 1.00 | 50 | 9124 | python method at src/pluggy/_hooks.py:516 |

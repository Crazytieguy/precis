scores: Sim=0.296 Reached=7/40 Early=5 Late=1 Partial=0 Missing=33 Used=1524/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 4 | 2 | 0 | 2 | 0.50 |
| 2 | 2 | 0 | 0 | 2 | 0.00 |
| 3 | 2 | 0 | 0 | 2 | 0.00 |
| 4 | 6 | 0 | 0 | 6 | 0.00 |
| 5 | 7 | 0 | 0 | 7 | 0.00 |
| 6 | 5 | 1 | 0 | 4 | 0.20 |
| 7 | 14 | 4 | 0 | 10 | 0.31 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 213 | — | — | 0.00 | missing | README lede + tagline |  |
| 1.3 | 256 | 516 | +260 | 1.00 | late | Source package layout (src/pluggy/) |  |
| 1.4 | 406 | — | — | 0.00 | missing | __all__ — full public-name list |  |
| 2.1 | 624 | — | — | 0.00 | missing | Toy example — first half (spec + plugin classes) |  |
| 2.2 | 829 | — | — | 0.00 | missing | Toy example — second half (PluginManager wiring + call) |  |
| 3.1 | 995 | — | — | 0.00 | missing | __init__.py re-exports — name → owning module |  |
| 3.2 | 1261 | — | — | 0.00 | missing | docs/index.rst lede — "what is pluggy" |  |
| 4.1 | 1595 | — | — | 0.00 | missing | _manager.py — every class + def name |  |
| 4.2 | 2037 | — | — | 0.00 | missing | _hooks.py — every class + def name (markers, HookCaller, HookImpl, HookSpec) |  |
| 4.3 | 2138 | — | — | 0.00 | missing | _callers.py — every def + _multicall signature |  |
| 4.4 | 2275 | — | — | 0.00 | missing | _result.py — Result class + every method (in full) |  |
| 4.5 | 2581 | — | — | 0.00 | missing | _warnings.py — full file (Pluggy*Warning classes) |  |
| 4.6 | 2682 | — | — | 0.00 | missing | _tracing.py — class + def names |  |
| 5.1 | 2873 | — | — | 0.00 | missing | PluginManager class docstring |  |
| 5.2 | 3038 | — | — | 0.00 | missing | HookspecMarker + HookimplMarker class docstrings |  |
| 5.3 | 3369 | — | — | 0.00 | missing | HookCaller __slots__ + the 6-bucket call-order comment |  |
| 5.4 | 3651 | — | — | 0.00 | missing | HookCaller._add_hookimpl body — the actual ordering algorithm |  |
| 5.5 | 4080 | — | — | 0.00 | missing | HookspecOpts + HookimplOpts — TypedDict bodies |  |
| 5.6 | 4494 | — | — | 0.00 | missing | Result API — force_result, force_exception, get_result bodies |  |
| 5.7 | 4801 | — | — | 0.00 | missing | docs/index.rst H2 heading map — sections at exact line numbers |  |
| 6.1 | 4855 | 1222 | -3633 | 1.00 | early | docs/examples/ FS listing |  |
| 6.2 | 5075 | — | — | 0.00 | missing | Eggsample hookspecs.py — host-side hook specifications |  |
| 6.3 | 5479 | — | — | 0.00 | missing | Eggsample lib.py + eggsample-spam (host impls + plugin impls) |  |
| 6.4 | 6020 | — | — | 0.00 | missing | docs/index.rst — "Call time order" section (tryfirst / trylast) |  |
| 6.5 | 6517 | — | — | 0.00 | missing | docs/index.rst — Wrappers (new-style) wrapper-protocol summary |  |
| 7.1 | 6579 | 473 | -6106 | 1.00 | early | testing/ FS listing |  |
| 7.2 | 6827 | — | — | 0.00 | missing | testing/conftest.py — pm + he_pm fixtures |  |
| 7.3 | 7193 | — | — | 0.00 | missing | test_pluginmanager.py — every test fn name |  |
| 7.4 | 7623 | — | — | 0.00 | missing | test_hookcaller.py + test_multicall.py — every test fn name |  |
| 7.5 | 8023 | — | — | 0.00 | missing | Smaller test files — every test fn name |  |
| 7.6 | 8324 | — | — | 0.00 | missing | pyproject.toml — [project] essentials (skip classifier list) |  |
| 7.7 | 8576 | — | — | 0.00 | missing | pyproject.toml — [tool.ruff.lint] config |  |
| 7.8 | 8859 | — | — | 0.00 | missing | tox.ini — [tox] + [testenv] + [pytest] (skip release/docs envs) |  |
| 7.9 | 9238 | — | — | 0.00 | missing | CHANGELOG.rst — pluggy 1.6.0 entry only |  |
| 7.10 | 9300 | 375 | -8925 | 1.00 | early | changelog/ + downstream/ FS listings |  |
| 7.11 | 9819 | — | — | 0.00 | missing | _callers._multicall body — the actual call loop |  |
| 7.12 | 9946 | — | — | 0.31 | missing | Top-level meta — AGENTS.md, SECURITY.md, MANIFEST.in | AGENTS.md section #0 (t=211, 2 atoms) |
| 7.13 | 9964 | 135 | -9829 | 1.00 | early | scripts/ FS + .github/workflows/ FS |  |
| 7.14 | 9996 | 186 | -9810 | 1.00 | early | docs/ root FS + docs/requirements.txt |  |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 5 | 464 | CLAUDE.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 302 | 1.00 | 302 | 1524 | plaintext config LICENSE |
| 140 | 1.00 | 140 | 1202 | CLAUDE.md section #4 |
| 101 | 1.00 | 101 | 312 | headings outline in CLAUDE.md |
| 99 | 1.00 | 99 | 819 | CLAUDE.md section #2 |
| 97 | 1.00 | 97 | 962 | json config .claude/settings.json |
| 87 | 1.00 | 87 | 720 | CLAUDE.md section #3 |
| 72 | 1.00 | 72 | 633 | CLAUDE.md section #6 |
| 66 | 1.00 | 66 | 1051 | CLAUDE.md section #5 |

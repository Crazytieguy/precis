Score(3000)=0.455 I=0.742 C=0.279 ns_rows≤3K=15/40 (reached=5 partial=0 missing=10)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 96 | 96 | listing of '.' |  |  | 1.000 |
| ns | 96 |  | 96 | Top-level repo listing | 1.1 |  | 1.000 |
| walker |  | 100 | 4 | listing of 'src' |  |  | 1.000 |
| walker |  | 104 | 4 | listing of '.claude' |  |  | 1.000 |
| walker |  | 117 | 13 | listing of '.github' |  |  | 1.000 |
| walker |  | 121 | 4 | listing of '.github/workflows' |  |  | 1.000 |
| walker |  | 135 | 14 | listing of 'scripts' |  |  | 1.000 |
| walker |  | 147 | 12 | python decl names surface in scripts/towncrier-draft-to-file.py |  |  | 1.000 |
| walker |  | 147 | 0 | python decl at scripts/towncrier-draft-to-file.py:5 |  |  | 1.000 |
| walker |  | 166 | 19 | listing of 'changelog' |  |  | 1.000 |
| walker |  | 209 | 43 | listing of 'src/pluggy' |  |  | 1.000 |
| ns | 213 |  | 117 | README lede + tagline | 1.2 |  | 0.848 |
| walker |  | 227 | 18 | python decl names surface in src/pluggy/__init__.py |  |  | 0.848 |
| walker |  | 227 | 0 | python decl at src/pluggy/__init__.py:32 |  |  | 0.848 |
| ns | 256 |  | 43 | Source package layout (src/pluggy/) | 1.3 |  | 0.855 |
| walker |  | 259 | 32 | listing of 'docs' |  |  | 0.856 |
| walker |  | 262 | 3 | listing of 'docs/_static' |  |  | 0.856 |
| walker |  | 266 | 4 | listing of 'docs/_static/img' |  |  | 0.856 |
| walker |  | 291 | 25 | AGENTS.md section #0 |  |  | 0.856 |
| walker |  | 392 | 101 | headings outline in CLAUDE.md |  |  | 0.856 |
| ns | 406 |  | 150 | __all__ — full public-name list | 1.4 |  | 0.703 |
| walker |  | 412 | 20 | CLAUDE.md section #0 |  |  | 0.703 |
| walker |  | 455 | 43 | listing of 'downstream' |  |  | 0.704 |
| walker |  | 484 | 29 | SECURITY.md section #0 |  |  | 0.704 |
| walker |  | 506 | 22 | python decl names surface in src/pluggy/_warnings.py |  |  | 0.704 |
| walker |  | 506 | 0 | python decl at src/pluggy/_warnings.py:4 |  |  | 0.704 |
| walker |  | 520 | 14 | python decl at src/pluggy/_warnings.py:10 |  |  | 0.705 |
| walker |  | 533 | 13 | python class body at src/pluggy/_warnings.py:4 |  |  | 0.705 |
| walker |  | 546 | 13 | python class body at src/pluggy/_warnings.py:10 |  |  | 0.705 |
| walker |  | 562 | 16 | python imports in scripts/towncrier-draft-to-file.py |  |  | 0.705 |
| walker |  | 571 | 9 | python imports in src/pluggy/_warnings.py |  |  | 0.705 |
| walker |  | 585 | 14 | python decl doc at src/pluggy/_warnings.py:4 |  |  | 0.705 |
| ns | 624 |  | 218 | Toy example — first half (spec + plugin classes) | 2.1 |  | 0.576 |
| walker |  | 647 | 62 | listing of 'testing' |  |  | 0.578 |
| walker |  | 667 | 20 | python decl names surface in testing/conftest.py |  |  | 0.578 |
| walker |  | 676 | 9 | python decl at testing/conftest.py:23 |  |  | 0.578 |
| walker |  | 687 | 11 | python decl body at testing/conftest.py:23 body 25 |  |  | 0.578 |
| ns | 829 |  | 205 | Toy example — second half (PluginManager wiring + call) | 2.2 | 2.1 | 0.509 |
| ns | 995 |  | 166 | __init__.py re-exports — name → owning module | 3.1 | 1.4 | 0.474 |
| walker |  | 1003 | 316 | python imports in src/pluggy/__init__.py |  |  | 0.683 |
| walker |  | 1057 | 54 | python decl at testing/conftest.py:7 |  |  | 0.684 |
| walker |  | 1087 | 30 | python imports in testing/conftest.py |  |  | 0.684 |
| walker |  | 1171 | 84 | python decl names surface in scripts/release.py |  |  | 0.684 |
| walker |  | 1171 | 0 | python decl at scripts/release.py:15 |  |  | 0.684 |
| walker |  | 1171 | 0 | python decl at scripts/release.py:30 |  |  | 0.684 |
| walker |  | 1171 | 0 | python decl at scripts/release.py:39 |  |  | 0.684 |
| walker |  | 1171 | 0 | python decl at scripts/release.py:50 |  |  | 0.684 |
| walker |  | 1171 | 0 | python decl at scripts/release.py:59 |  |  | 0.684 |
| walker |  | 1183 | 12 | python decl doc at scripts/release.py:15 |  |  | 0.684 |
| walker |  | 1198 | 15 | python decl doc at scripts/release.py:30 |  |  | 0.684 |
| walker |  | 1215 | 17 | python decl doc at scripts/release.py:39 |  |  | 0.684 |
| walker |  | 1255 | 40 | python imports in docs/conf.py |  |  | 0.684 |
| ns | 1261 |  | 266 | docs/index.rst lede — "what is pluggy" | 3.2 |  | 0.624 |
| walker |  | 1310 | 55 | python decl names surface in src/pluggy/_tracing.py |  |  | 0.624 |
| walker |  | 1310 | 0 | python decl at src/pluggy/_tracing.py:12 |  |  | 0.624 |
| walker |  | 1310 | 0 | python decl at src/pluggy/_tracing.py:13 |  |  | 0.624 |
| walker |  | 1310 | 0 | python decl at src/pluggy/_tracing.py:16 |  |  | 0.624 |
| walker |  | 1310 | 0 | python decl at src/pluggy/_tracing.py:59 |  |  | 0.624 |
| walker |  | 1366 | 56 | python decl body at src/pluggy/__init__.py:32 body 33 |  |  | 0.624 |
| walker |  | 1381 | 15 | listing of 'docs/examples' |  |  | 0.624 |
| walker |  | 1389 | 8 | listing of 'docs/examples/eggsample' |  |  | 0.624 |
| walker |  | 1457 | 68 | python decl names surface in src/pluggy/_callers.py |  |  | 0.625 |
| walker |  | 1457 | 0 | python decl at src/pluggy/_callers.py:24 |  |  | 0.625 |
| walker |  | 1486 | 29 | python decl at src/pluggy/_callers.py:27 |  |  | 0.625 |
| walker |  | 1517 | 31 | python decl doc at src/pluggy/_callers.py:27 |  |  | 0.625 |
| walker |  | 1586 | 69 | python decl names surface in src/pluggy/_result.py |  |  | 0.625 |
| walker |  | 1586 | 0 | python decl at src/pluggy/_result.py:16 |  |  | 0.625 |
| walker |  | 1586 | 0 | python decl at src/pluggy/_result.py:17 |  |  | 0.625 |
| walker |  | 1586 | 0 | python decl at src/pluggy/_result.py:20 |  |  | 0.625 |
| walker |  | 1586 | 0 | python decl at src/pluggy/_result.py:107 |  |  | 0.625 |
| walker |  | 1595 | 9 | python decl at src/pluggy/_result.py:24 |  |  | 0.546 |
| ns | 1595 |  | 334 | _manager.py — every class + def name | 4.1 |  | 0.546 |
| walker |  | 1604 | 9 | python decl doc at src/pluggy/_result.py:20 |  |  | 0.546 |
| walker |  | 1624 | 20 | python class body at src/pluggy/_result.py:24 |  |  | 0.546 |
| walker |  | 1658 | 34 | python decl doc at src/pluggy/_result.py:24 |  |  | 0.546 |
| walker |  | 1787 | 129 | python decl names surface in testing/benchmark.py |  |  | 0.546 |
| walker |  | 1787 | 0 | python decl at testing/benchmark.py:16 |  |  | 0.546 |
| walker |  | 1787 | 0 | python decl at testing/benchmark.py:17 |  |  | 0.546 |
| walker |  | 1787 | 0 | python decl at testing/benchmark.py:40 |  |  | 0.546 |
| walker |  | 1800 | 13 | python decl at testing/benchmark.py:20 |  |  | 0.546 |
| walker |  | 1813 | 13 | python decl at testing/benchmark.py:25 |  |  | 0.546 |
| walker |  | 1826 | 13 | python decl at testing/benchmark.py:30 |  |  | 0.546 |
| walker |  | 1839 | 13 | python decl at testing/benchmark.py:35 |  |  | 0.546 |
| walker |  | 1848 | 9 | python decl body at testing/benchmark.py:25 body 27 |  |  | 0.546 |
| walker |  | 1863 | 15 | python decl body at testing/benchmark.py:20 body 22 |  |  | 0.546 |
| walker |  | 1878 | 15 | python decl body at testing/benchmark.py:30 body 32 |  |  | 0.546 |
| walker |  | 1893 | 15 | python decl body at testing/benchmark.py:35 body 37 |  |  | 0.546 |
| walker |  | 1957 | 64 | python decl doc at scripts/towncrier-draft-to-file.py:5 |  |  | 0.546 |
| walker |  | 1977 | 20 | listing of 'docs/examples/eggsample/eggsample' |  |  | 0.547 |
| walker |  | 2022 | 45 | CLAUDE.md section #1 |  |  | 0.547 |
| ns | 2037 |  | 442 | _hooks.py — every class + def name (markers, HookCaller, HookImpl, HookSpec) | 4.2 |  | 0.479 |
| walker |  | 2049 | 27 | python decl at src/pluggy/_callers.py:70 |  |  | 0.479 |
| walker |  | 2057 | 8 | python imports in docs/examples/eggsample/eggsample/__init__.py |  |  | 0.479 |
| walker |  | 2105 | 48 | python decl body at scripts/towncrier-draft-to-file.py:5 body 11 |  |  | 0.479 |
| ns | 2138 |  | 101 | _callers.py — every def + _multicall signature | 4.3 |  | 0.473 |
| walker |  | 2184 | 79 | python imports in scripts/release.py |  |  | 0.473 |
| ns | 2275 |  | 137 | _result.py — Result class + every method (in full) | 4.4 |  | 0.460 |
| walker |  | 2319 | 135 | python decl names surface in docs/conf.py |  |  | 0.460 |
| walker |  | 2319 | 0 | python decl at docs/conf.py:18 |  |  | 0.460 |
| walker |  | 2319 | 0 | python decl at docs/conf.py:20 |  |  | 0.460 |
| walker |  | 2319 | 0 | python decl at docs/conf.py:23 |  |  | 0.460 |
| walker |  | 2319 | 0 | python decl at docs/conf.py:27 |  |  | 0.460 |
| walker |  | 2319 | 0 | python decl at docs/conf.py:28 |  |  | 0.460 |
| walker |  | 2319 | 0 | python decl at docs/conf.py:29 |  |  | 0.460 |
| walker |  | 2319 | 0 | python decl at docs/conf.py:31 |  |  | 0.460 |
| walker |  | 2319 | 0 | python decl at docs/conf.py:33 |  |  | 0.460 |
| walker |  | 2319 | 0 | python decl at docs/conf.py:36 |  |  | 0.460 |
| walker |  | 2319 | 0 | python decl at docs/conf.py:38 |  |  | 0.460 |
| walker |  | 2319 | 0 | python decl at docs/conf.py:40 |  |  | 0.460 |
| walker |  | 2387 | 68 | python decl at docs/conf.py:9 |  |  | 0.460 |
| walker |  | 2419 | 32 | python decl at src/pluggy/_callers.py:60 |  |  | 0.460 |
| walker |  | 2516 | 97 | python method sigs in src/pluggy/_result.py |  |  | 0.477 |
| walker |  | 2516 | 0 | python method at src/pluggy/_result.py:67 |  |  | 0.477 |
| walker |  | 2516 | 0 | python method at src/pluggy/_result.py:80 |  |  | 0.477 |
| walker |  | 2516 | 0 | python method at src/pluggy/_result.py:91 |  |  | 0.477 |
| walker |  | 2529 | 13 | python method at src/pluggy/_result.py:51 |  |  | 0.484 |
| walker |  | 2544 | 15 | python method at src/pluggy/_result.py:42 |  |  | 0.491 |
| walker |  | 2566 | 22 | python method at src/pluggy/_result.py:56 |  |  | 0.503 |
| walker |  | 2578 | 12 | python method doc at src/pluggy/_result.py:42 |  |  | 0.503 |
| ns | 2581 |  | 306 | _warnings.py — full file (Pluggy*Warning classes) | 4.5 |  | 0.477 |
| walker |  | 2590 | 12 | python method doc at src/pluggy/_result.py:51 |  |  | 0.477 |
| walker |  | 2602 | 12 | python method doc at src/pluggy/_result.py:56 |  |  | 0.477 |
| walker |  | 2612 | 10 | python method body at src/pluggy/_result.py:51 body 54 |  |  | 0.477 |
| walker |  | 2650 | 38 | python method at src/pluggy/_result.py:31 |  |  | 0.477 |
| walker |  | 2662 | 12 | python method doc at src/pluggy/_result.py:31 |  |  | 0.477 |
| ns | 2682 |  | 101 | _tracing.py — class + def names | 4.6 |  | 0.467 |
| walker |  | 2778 | 116 | python decl names surface in src/pluggy/_manager.py |  |  | 0.470 |
| walker |  | 2778 | 0 | python decl at src/pluggy/_manager.py:36 |  |  | 0.470 |
| walker |  | 2778 | 0 | python decl at src/pluggy/_manager.py:42 |  |  | 0.470 |
| walker |  | 2778 | 0 | python decl at src/pluggy/_manager.py:52 |  |  | 0.470 |
| walker |  | 2778 | 0 | python decl at src/pluggy/_manager.py:65 |  |  | 0.470 |
| walker |  | 2778 | 0 | python decl at src/pluggy/_manager.py:83 |  |  | 0.470 |
| walker |  | 2778 | 0 | python decl at src/pluggy/_manager.py:525 |  |  | 0.470 |
| walker |  | 2789 | 11 | python decl doc at src/pluggy/_manager.py:65 |  |  | 0.470 |
| walker |  | 2830 | 41 | python decl doc at src/pluggy/_manager.py:52 |  |  | 0.470 |
| walker |  | 2846 | 16 | python decl body at src/pluggy/_manager.py:525 body 526 |  |  | 0.470 |
| ns | 2873 |  | 191 | PluginManager class docstring | 5.1 | 4.1 | 0.455 |
| walker |  | 2946 | 100 | python imports in testing/benchmark.py |  |  | 0.455 |
| walker |  | 2974 | 28 | python decl at src/pluggy/_manager.py:37 |  |  | 0.455 |
| walker |  | 3033 | 59 | python imports in src/pluggy/_tracing.py |  |  | 0.455 |
| ns | 3038 |  | 165 | HookspecMarker + HookimplMarker class docstrings | 5.2 | 4.2 | 0.444 |
| walker |  | 3105 | 72 | CLAUDE.md section #10 |  |  | 0.444 |
| walker |  | 3131 | 26 | CLAUDE.md section #7 |  |  | 0.444 |
| walker |  | 3209 | 78 | python decl body at scripts/release.py:50 body 51 |  |  | 0.444 |
| walker |  | 3258 | 49 | python method doc at src/pluggy/_result.py:80 |  |  | 0.444 |
| walker |  | 3285 | 27 | CLAUDE.md section #5 |  |  | 0.444 |
| ns | 3369 |  | 331 | HookCaller __slots__ + the 6-bucket call-order comment | 5.3 | 4.2 | 0.421 |
| walker |  | 3486 | 201 | python decl at testing/benchmark.py:54 |  |  | 0.421 |
| walker |  | 3573 | 87 | CLAUDE.md section #3 |  |  | 0.421 |
| walker |  | 3601 | 28 | CLAUDE.md section #6 |  |  | 0.421 |
| walker |  | 3630 | 29 | CLAUDE.md section #4 |  |  | 0.421 |
| ns | 3651 |  | 282 | HookCaller._add_hookimpl body — the actual ordering algorithm | 5.4 | 4.2 | 0.405 |
| walker |  | 3686 | 56 | python method doc at src/pluggy/_result.py:91 |  |  | 0.405 |
| walker |  | 3705 | 19 | python decl body at scripts/release.py:30 body 36 |  |  | 0.405 |
| walker |  | 3854 | 149 | python decl names surface #1 in docs/conf.py |  |  | 0.405 |
| walker |  | 3854 | 0 | python decl at docs/conf.py:57 |  |  | 0.405 |
| walker |  | 3854 | 0 | python decl at docs/conf.py:61 |  |  | 0.405 |
| walker |  | 3854 | 0 | python decl at docs/conf.py:63 |  |  | 0.405 |
| walker |  | 3854 | 0 | python decl at docs/conf.py:65 |  |  | 0.405 |
| walker |  | 3854 | 0 | python decl at docs/conf.py:106 |  |  | 0.405 |
| walker |  | 3854 | 0 | python decl at docs/conf.py:130 |  |  | 0.405 |
| walker |  | 3861 | 7 | python decl body at docs/conf.py:130 body 131 |  |  | 0.405 |
| walker |  | 3879 | 18 | python decl doc at docs/conf.py:106 |  |  | 0.405 |
| walker |  | 3909 | 30 | python decl at docs/conf.py:54 |  |  | 0.405 |
| walker |  | 3990 | 81 | python decl at docs/conf.py:83 |  |  | 0.405 |
| ns | 4080 |  | 429 | HookspecOpts + HookimplOpts — TypedDict bodies | 5.5 | 4.2 | 0.385 |
| walker |  | 4126 | 136 | python decl at docs/conf.py:96 |  |  | 0.385 |
| walker |  | 4272 | 146 | python decl at docs/conf.py:41 |  |  | 0.385 |
| walker |  | 4430 | 158 | python decl at docs/conf.py:66 |  |  | 0.385 |
| ns | 4494 |  | 414 | Result API — force_result, force_exception, get_result bodies | 5.6 | 4.4 | 0.375 |
| walker |  | 4519 | 89 | python decl body at testing/conftest.py:7 body 12 |  |  | 0.375 |
| walker |  | 4549 | 30 | CLAUDE.md section #8 |  |  | 0.375 |
| walker |  | 4606 | 57 | python decl at src/pluggy/_callers.py:82 |  |  | 0.393 |
| walker |  | 4705 | 99 | CLAUDE.md section #2 |  |  | 0.393 |
| ns | 4801 |  | 307 | docs/index.rst H2 heading map — sections at exact line numbers | 5.7 | 3.2 | 0.373 |
| walker |  | 4806 | 101 | python imports in src/pluggy/_result.py |  |  | 0.373 |
| ns | 4855 |  | 54 | docs/examples/ FS listing | 6.1 |  | 0.383 |
| walker |  | 5016 | 210 | python method sigs in src/pluggy/_tracing.py |  |  | 0.404 |
| walker |  | 5016 | 0 | python method at src/pluggy/_tracing.py:17 |  |  | 0.404 |
| walker |  | 5016 | 0 | python method at src/pluggy/_tracing.py:22 |  |  | 0.404 |
| walker |  | 5016 | 0 | python method at src/pluggy/_tracing.py:25 |  |  | 0.404 |
| walker |  | 5016 | 0 | python method at src/pluggy/_tracing.py:42 |  |  | 0.404 |
| walker |  | 5016 | 0 | python method at src/pluggy/_tracing.py:48 |  |  | 0.404 |
| walker |  | 5016 | 0 | python method at src/pluggy/_tracing.py:51 |  |  | 0.404 |
| walker |  | 5016 | 0 | python method at src/pluggy/_tracing.py:60 |  |  | 0.404 |
| walker |  | 5016 | 0 | python method at src/pluggy/_tracing.py:64 |  |  | 0.404 |
| walker |  | 5016 | 0 | python method at src/pluggy/_tracing.py:67 |  |  | 0.404 |
| walker |  | 5025 | 9 | python method body at src/pluggy/_tracing.py:48 body 49 |  |  | 0.404 |
| walker |  | 5037 | 12 | python method body at src/pluggy/_tracing.py:22 body 23 |  |  | 0.404 |
| walker |  | 5050 | 13 | python method body at src/pluggy/_tracing.py:64 body 65 |  |  | 0.404 |
| walker |  | 5067 | 17 | python method body at src/pluggy/_tracing.py:67 body 68 |  |  | 0.404 |
| ns | 5075 |  | 220 | Eggsample hookspecs.py — host-side hook specifications | 6.2 |  | 0.394 |
| walker |  | 5085 | 18 | python method body at src/pluggy/_tracing.py:60 body 61 |  |  | 0.394 |
| walker |  | 5266 | 181 | python decl names surface in src/pluggy/_hooks.py |  |  | 0.394 |
| walker |  | 5266 | 0 | python decl at src/pluggy/_hooks.py:28 |  |  | 0.394 |
| walker |  | 5266 | 0 | python decl at src/pluggy/_hooks.py:29 |  |  | 0.394 |
| walker |  | 5266 | 0 | python decl at src/pluggy/_hooks.py:31 |  |  | 0.394 |
| walker |  | 5266 | 0 | python decl at src/pluggy/_hooks.py:32 |  |  | 0.394 |
| walker |  | 5266 | 0 | python decl at src/pluggy/_hooks.py:37 |  |  | 0.394 |
| walker |  | 5266 | 0 | python decl at src/pluggy/_hooks.py:40 |  |  | 0.394 |
| walker |  | 5266 | 0 | python decl at src/pluggy/_hooks.py:56 |  |  | 0.394 |
| walker |  | 5266 | 0 | python decl at src/pluggy/_hooks.py:281 |  |  | 0.394 |
| walker |  | 5266 | 0 | python decl at src/pluggy/_hooks.py:290 |  |  | 0.394 |
| walker |  | 5273 | 7 | python decl at src/pluggy/_hooks.py:77 |  |  | 0.395 |
| walker |  | 5280 | 7 | python decl at src/pluggy/_hooks.py:164 |  |  | 0.395 |
| walker |  | 5290 | 10 | python decl doc at src/pluggy/_hooks.py:40 |  |  | 0.395 |
| walker |  | 5300 | 10 | python decl doc at src/pluggy/_hooks.py:56 |  |  | 0.396 |
| walker |  | 5314 | 14 | python class body at src/pluggy/_hooks.py:77 |  |  | 0.396 |
| walker |  | 5328 | 14 | python class body at src/pluggy/_hooks.py:164 |  |  | 0.396 |
| walker |  | 5405 | 77 | python decl doc at src/pluggy/_hooks.py:164 |  |  | 0.400 |
| ns | 5479 |  | 404 | Eggsample lib.py + eggsample-spam (host impls + plugin impls) | 6.3 |  | 0.384 |
| walker |  | 5483 | 78 | python decl doc at src/pluggy/_hooks.py:77 |  |  | 0.398 |
| walker |  | 5617 | 134 | python method sigs in src/pluggy/_hooks.py |  |  | 0.401 |
| walker |  | 5617 | 0 | python method at src/pluggy/_hooks.py:88 |  |  | 0.401 |
| walker |  | 5617 | 0 | python method at src/pluggy/_hooks.py:175 |  |  | 0.401 |
| walker |  | 5629 | 12 | python method body at src/pluggy/_hooks.py:88 body 89 |  |  | 0.401 |
| walker |  | 5641 | 12 | python method body at src/pluggy/_hooks.py:175 body 176 |  |  | 0.401 |
| walker |  | 5677 | 36 | python decl at src/pluggy/_hooks.py:33 |  |  | 0.401 |
| walker |  | 5759 | 82 | python method at src/pluggy/_hooks.py:91 |  |  | 0.402 |
| walker |  | 5770 | 11 | python method body at src/pluggy/_hooks.py:91 body 99 |  |  | 0.402 |
| walker |  | 5860 | 90 | python method at src/pluggy/_hooks.py:101 |  |  | 0.403 |
| walker |  | 5876 | 16 | python method body at src/pluggy/_hooks.py:101 body 109 |  |  | 0.403 |
| ns | 6020 |  | 541 | docs/index.rst — "Call time order" section (tryfirst / trylast) | 6.4 | 5.7 | 0.382 |
| walker |  | 6040 | 164 | python class body at src/pluggy/_hooks.py:40 |  |  | 0.391 |
| walker |  | 6135 | 95 | python method at src/pluggy/_hooks.py:111 |  |  | 0.391 |
| walker |  | 6232 | 97 | python method at src/pluggy/_hooks.py:178 |  |  | 0.392 |
| walker |  | 6243 | 11 | python method body at src/pluggy/_hooks.py:178 body 188 |  |  | 0.392 |
| walker |  | 6289 | 46 | plaintext config downstream/.gitignore |  |  | 0.392 |
| walker |  | 6340 | 51 | python decl doc at src/pluggy/_callers.py:82 |  |  | 0.392 |
| walker |  | 6445 | 105 | python method at src/pluggy/_hooks.py:190 |  |  | 0.393 |
| walker |  | 6461 | 16 | python method body at src/pluggy/_hooks.py:190 body 200 |  |  | 0.393 |
| ns | 6517 |  | 497 | docs/index.rst — Wrappers (new-style) wrapper-protocol summary | 6.5 | 5.7 | 0.380 |
| walker |  | 6571 | 110 | python method at src/pluggy/_hooks.py:202 |  |  | 0.380 |
| ns | 6579 |  | 62 | testing/ FS listing | 7.1 |  | 0.392 |
| walker |  | 6684 | 113 | python decl names surface #1 in src/pluggy/_hooks.py |  |  | 0.396 |
| walker |  | 6684 | 0 | python decl at src/pluggy/_hooks.py:293 |  |  | 0.396 |
| walker |  | 6684 | 0 | python decl at src/pluggy/_hooks.py:374 |  |  | 0.396 |
| walker |  | 6684 | 0 | python decl at src/pluggy/_hooks.py:382 |  |  | 0.396 |
| walker |  | 6684 | 0 | python decl at src/pluggy/_hooks.py:590 |  |  | 0.396 |
| walker |  | 6684 | 0 | python decl at src/pluggy/_hooks.py:593 |  |  | 0.396 |
| walker |  | 6690 | 6 | python decl at src/pluggy/_hooks.py:358 |  |  | 0.397 |
| walker |  | 6696 | 6 | python decl at src/pluggy/_hooks.py:638 |  |  | 0.398 |
| walker |  | 6702 | 6 | python decl at src/pluggy/_hooks.py:696 |  |  | 0.400 |
| walker |  | 6717 | 15 | python decl doc at src/pluggy/_hooks.py:382 |  |  | 0.400 |
| walker |  | 6735 | 18 | python decl doc at src/pluggy/_hooks.py:638 |  |  | 0.400 |
| walker |  | 6766 | 31 | python decl doc at src/pluggy/_hooks.py:358 |  |  | 0.400 |
| walker |  | 6811 | 45 | python class body at src/pluggy/_hooks.py:358 |  |  | 0.401 |
| ns | 6827 |  | 248 | testing/conftest.py — pm + he_pm fixtures | 7.2 |  | 0.416 |
| walker |  | 6837 | 26 | python decl at src/pluggy/_hooks.py:377 |  |  | 0.416 |
| walker |  | 6925 | 88 | python class body at src/pluggy/_hooks.py:382 |  |  | 0.418 |
| walker |  | 7013 | 88 | python class body at src/pluggy/_hooks.py:696 |  |  | 0.418 |
| walker |  | 7043 | 30 | python decl doc at src/pluggy/_hooks.py:593 |  |  | 0.418 |
| walker |  | 7155 | 112 | python class body at src/pluggy/_hooks.py:638 |  |  | 0.418 |
| ns | 7193 |  | 366 | test_pluginmanager.py — every test fn name | 7.3 |  | 0.407 |
| walker |  | 7227 | 72 | python decl doc at src/pluggy/_hooks.py:293 |  |  | 0.407 |
| walker |  | 7396 | 169 | python decl doc at src/pluggy/_manager.py:83 |  |  | 0.422 |
| walker |  | 7513 | 117 | python decl body at scripts/release.py:59 body 60 |  |  | 0.422 |
| ns | 7623 |  | 430 | test_hookcaller.py + test_multicall.py — every test fn name | 7.4 |  | 0.410 |
| walker |  | 7684 | 171 | python decl doc at src/pluggy/_warnings.py:10 |  |  | 0.429 |
| walker |  | 7910 | 226 | python class body at src/pluggy/_hooks.py:56 |  |  | 0.457 |
| walker |  | 7985 | 75 | python decl body at src/pluggy/_hooks.py:281 body 282 |  |  | 0.457 |
| ns | 8023 |  | 400 | Smaller test files — every test fn name | 7.5 |  | 0.445 |
| walker |  | 8072 | 87 | python method doc at src/pluggy/_result.py:67 |  |  | 0.453 |
| walker |  | 8233 | 161 | python imports in src/pluggy/_callers.py |  |  | 0.453 |
| ns | 8324 |  | 301 | pyproject.toml — [project] essentials (skip classifier list) | 7.6 |  | 0.444 |
| walker |  | 8330 | 97 | json config .claude/settings.json |  |  | 0.444 |
| walker |  | 8379 | 49 | python method body at src/pluggy/_tracing.py:17 body 18 |  |  | 0.444 |
| walker |  | 8397 | 18 | python decl names surface in docs/examples/eggsample/eggsample/__init__.py |  |  | 0.444 |
| walker |  | 8397 | 0 | python decl at docs/examples/eggsample/eggsample/__init__.py:4 |  |  | 0.444 |
| walker |  | 8549 | 152 | python decl body at testing/benchmark.py:40 body 41 |  |  | 0.444 |
| walker |  | 8560 | 11 | python method body at src/pluggy/_result.py:42 body 45 |  |  | 0.444 |
| ns | 8576 |  | 252 | pyproject.toml — [tool.ruff.lint] config | 7.7 | 7.6 | 0.438 |
| ns | 8859 |  | 283 | tox.ini — [tox] + [testenv] + [pytest] (skip release/docs envs) | 7.8 |  | 0.430 |
| walker |  | 8944 | 384 | python method sigs #1 in src/pluggy/_hooks.py |  |  | 0.462 |
| walker |  | 8944 | 0 | python method at src/pluggy/_hooks.py:365 |  |  | 0.462 |
| walker |  | 8944 | 0 | python method at src/pluggy/_hooks.py:420 |  |  | 0.462 |
| walker |  | 8944 | 0 | python method at src/pluggy/_hooks.py:438 |  |  | 0.462 |
| walker |  | 8944 | 0 | python method at src/pluggy/_hooks.py:442 |  |  | 0.462 |
| walker |  | 8944 | 0 | python method at src/pluggy/_hooks.py:449 |  |  | 0.462 |
| walker |  | 8944 | 0 | python method at src/pluggy/_hooks.py:453 |  |  | 0.462 |
| walker |  | 8944 | 0 | python method at src/pluggy/_hooks.py:477 |  |  | 0.462 |
| walker |  | 8944 | 0 | python method at src/pluggy/_hooks.py:499 |  |  | 0.462 |
| walker |  | 8944 | 0 | python method at src/pluggy/_hooks.py:577 |  |  | 0.462 |
| walker |  | 8944 | 0 | python method at src/pluggy/_hooks.py:612 |  |  | 0.462 |
| walker |  | 8944 | 0 | python method at src/pluggy/_hooks.py:634 |  |  | 0.462 |
| walker |  | 8944 | 0 | python method at src/pluggy/_hooks.py:692 |  |  | 0.462 |
| walker |  | 8944 | 0 | python method at src/pluggy/_hooks.py:709 |  |  | 0.462 |
| walker |  | 8965 | 21 | python method at src/pluggy/_hooks.py:626 |  |  | 0.464 |
| walker |  | 8975 | 10 | python method doc at src/pluggy/_hooks.py:365 |  |  | 0.464 |
| walker |  | 8988 | 13 | python method doc at src/pluggy/_hooks.py:449 |  |  | 0.464 |
| walker |  | 9017 | 29 | python method at src/pluggy/_hooks.py:543 |  |  | 0.464 |
| walker |  | 9027 | 10 | python method body at src/pluggy/_hooks.py:420 body 421 |  |  | 0.464 |
| walker |  | 9038 | 11 | python method body at src/pluggy/_hooks.py:626 body 628 |  |  | 0.464 |
| walker |  | 9054 | 16 | python method doc at src/pluggy/_hooks.py:438 |  |  | 0.464 |
| walker |  | 9069 | 15 | python method at src/pluggy/_hooks.py:480 |  |  | 0.464 |
| walker |  | 9082 | 13 | python method body at src/pluggy/_hooks.py:449 body 451 |  |  | 0.464 |
| walker |  | 9121 | 39 | python method at src/pluggy/_hooks.py:424 |  |  | 0.464 |
| walker |  | 9137 | 16 | python method at src/pluggy/_hooks.py:618 |  |  | 0.466 |
| walker |  | 9151 | 14 | python method body at src/pluggy/_hooks.py:438 body 440 |  |  | 0.466 |
| walker |  | 9166 | 15 | python method body at src/pluggy/_hooks.py:477 body 478 |  |  | 0.466 |
| walker |  | 9178 | 12 | python method doc at src/pluggy/_hooks.py:453 |  |  | 0.466 |
| walker |  | 9195 | 17 | python method body at src/pluggy/_hooks.py:634 body 635 |  |  | 0.466 |
| ns | 9238 |  | 379 | CHANGELOG.rst — pluggy 1.6.0 entry only | 7.9 |  | 0.457 |
| walker |  | 9245 | 50 | python method at src/pluggy/_hooks.py:516 |  |  | 0.457 |
| walker |  | 9269 | 24 | python method at src/pluggy/_hooks.py:630 |  |  | 0.459 |
| ns | 9300 |  | 62 | changelog/ + downstream/ FS listings | 7.10 |  | 0.466 |
| walker |  | 9330 | 61 | python method at src/pluggy/_hooks.py:656 |  |  | 0.466 |
| walker |  | 9342 | 12 | python method doc at src/pluggy/_hooks.py:656 |  |  | 0.466 |
| walker |  | 9410 | 68 | python method at src/pluggy/_hooks.py:393 |  |  | 0.471 |
| walker |  | 9422 | 12 | python method doc at src/pluggy/_hooks.py:393 |  |  | 0.472 |
| walker |  | 9441 | 19 | python method doc at src/pluggy/_hooks.py:577 |  |  | 0.472 |
| walker |  | 9486 | 45 | python method doc at src/pluggy/_hooks.py:543 |  |  | 0.472 |
| walker |  | 9511 | 25 | python method body at src/pluggy/_hooks.py:692 body 693 |  |  | 0.472 |
| walker |  | 9524 | 13 | python method body at src/pluggy/_hooks.py:630 body 632 |  |  | 0.472 |
| walker |  | 9589 | 65 | python method doc at src/pluggy/_hooks.py:499 |  |  | 0.472 |
| walker |  | 9682 | 93 | python method doc at src/pluggy/_hooks.py:516 |  |  | 0.472 |
| walker |  | 9737 | 55 | python method body at src/pluggy/_tracing.py:51 body 52 |  |  | 0.472 |
| ns | 9819 |  | 519 | _callers._multicall body — the actual call loop | 7.11 | 4.3 | 0.459 |
| walker |  | 9945 | 208 | python imports in src/pluggy/_hooks.py |  |  | 0.459 |
| ns | 9946 |  | 127 | Top-level meta — AGENTS.md, SECURITY.md, MANIFEST.in | 7.12 |  | 0.457 |
| ns | 9964 |  | 18 | scripts/ FS + .github/workflows/ FS | 7.13 |  | 0.459 |
| ns | 9996 |  | 32 | docs/ root FS + docs/requirements.txt | 7.14 |  | 0.463 |

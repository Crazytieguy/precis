Score(3000)=0.605 I=0.805 C=0.455 ns_rows≤3K=15/40 (reached=7 partial=3 missing=5)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 96 | 96 | listing of '.' |  |  | 1.000 |
| ns | 96 |  | 96 | Top-level repo listing | 1.1 |  | 1.000 |
| walker |  | 100 | 4 | listing of 'src' |  |  | 1.000 |
| walker |  | 202 | 102 | README headline in README.rst |  |  | 1.000 |
| ns | 213 |  | 117 | README lede + tagline | 1.2 |  | 0.943 |
| walker |  | 245 | 43 | listing of 'src/pluggy' |  |  | 0.954 |
| ns | 256 |  | 43 | Source package layout (src/pluggy/) | 1.3 |  | 0.952 |
| ns | 406 |  | 150 | __all__ — full public-name list | 1.4 |  | 0.782 |
| walker |  | 561 | 316 | python imports in src/pluggy/__init__.py |  |  | 0.973 |
| walker |  | 579 | 18 | python decl names surface in src/pluggy/__init__.py |  |  | 0.973 |
| walker |  | 579 | 0 | python decl at src/pluggy/__init__.py:32 |  |  | 0.973 |
| walker |  | 598 | 19 | listing of 'changelog' |  |  | 0.973 |
| ns | 624 |  | 218 | Toy example — first half (spec + plugin classes) | 2.1 |  | 0.798 |
| walker |  | 630 | 32 | listing of 'docs' |  |  | 0.798 |
| walker |  | 675 | 45 | [package] in pyproject.toml |  |  | 0.798 |
| walker |  | 718 | 43 | listing of 'downstream' |  |  | 0.800 |
| walker |  | 721 | 3 | listing of 'docs/_static' |  |  | 0.800 |
| walker |  | 730 | 9 | python imports in src/pluggy/_warnings.py |  |  | 0.800 |
| walker |  | 752 | 22 | python decl names surface in src/pluggy/_warnings.py |  |  | 0.800 |
| walker |  | 752 | 0 | python decl at src/pluggy/_warnings.py:4 |  |  | 0.800 |
| walker |  | 765 | 13 | python class body at src/pluggy/_warnings.py:4 |  |  | 0.800 |
| walker |  | 779 | 14 | python decl doc at src/pluggy/_warnings.py:4 |  |  | 0.800 |
| walker |  | 828 | 49 | python decl at src/pluggy/_warnings.py:10 |  |  | 0.801 |
| ns | 829 |  | 205 | Toy example — second half (PluginManager wiring + call) | 2.2 | 2.1 | 0.706 |
| walker |  | 841 | 13 | python class body at src/pluggy/_warnings.py:10 |  |  | 0.706 |
| walker |  | 897 | 56 | python decl body at src/pluggy/__init__.py:32 body 33 |  |  | 0.706 |
| walker |  | 901 | 4 | listing of 'docs/_static/img' |  |  | 0.706 |
| walker |  | 963 | 62 | listing of 'testing' |  |  | 0.707 |
| ns | 995 |  | 166 | __init__.py re-exports — name → owning module | 3.1 | 1.4 | 0.732 |
| walker |  | 1018 | 55 | python decl names surface in src/pluggy/_tracing.py |  |  | 0.732 |
| walker |  | 1018 | 0 | python decl at src/pluggy/_tracing.py:16 |  |  | 0.732 |
| walker |  | 1018 | 0 | python decl at src/pluggy/_tracing.py:59 |  |  | 0.732 |
| walker |  | 1086 | 68 | python decl names surface in src/pluggy/_callers.py |  |  | 0.732 |
| walker |  | 1115 | 29 | python decl at src/pluggy/_callers.py:27 |  |  | 0.732 |
| walker |  | 1146 | 31 | python decl doc at src/pluggy/_callers.py:27 |  |  | 0.732 |
| walker |  | 1173 | 27 | python decl at src/pluggy/_callers.py:70 |  |  | 0.732 |
| walker |  | 1205 | 32 | python decl at src/pluggy/_callers.py:60 |  |  | 0.732 |
| ns | 1261 |  | 266 | docs/index.rst lede — "what is pluggy" | 3.2 |  | 0.668 |
| walker |  | 1274 | 69 | python decl names surface in src/pluggy/_result.py |  |  | 0.668 |
| walker |  | 1274 | 0 | python decl at src/pluggy/_result.py:20 |  |  | 0.668 |
| walker |  | 1283 | 9 | python decl at src/pluggy/_result.py:24 |  |  | 0.668 |
| walker |  | 1292 | 9 | python decl doc at src/pluggy/_result.py:20 |  |  | 0.668 |
| walker |  | 1312 | 20 | python class body at src/pluggy/_result.py:24 |  |  | 0.668 |
| walker |  | 1346 | 34 | python decl doc at src/pluggy/_result.py:24 |  |  | 0.668 |
| walker |  | 1443 | 97 | python method sigs in src/pluggy/_result.py |  |  | 0.669 |
| walker |  | 1443 | 0 | python method at src/pluggy/_result.py:67 |  |  | 0.669 |
| walker |  | 1443 | 0 | python method at src/pluggy/_result.py:80 |  |  | 0.669 |
| walker |  | 1443 | 0 | python method at src/pluggy/_result.py:91 |  |  | 0.669 |
| walker |  | 1456 | 13 | python method at src/pluggy/_result.py:51 |  |  | 0.670 |
| walker |  | 1471 | 15 | python method at src/pluggy/_result.py:42 |  |  | 0.671 |
| walker |  | 1493 | 22 | python method at src/pluggy/_result.py:56 |  |  | 0.672 |
| walker |  | 1505 | 12 | python method doc at src/pluggy/_result.py:42 |  |  | 0.672 |
| walker |  | 1517 | 12 | python method doc at src/pluggy/_result.py:51 |  |  | 0.672 |
| walker |  | 1529 | 12 | python method doc at src/pluggy/_result.py:56 |  |  | 0.672 |
| walker |  | 1539 | 10 | python method body at src/pluggy/_result.py:51 body 54 |  |  | 0.672 |
| walker |  | 1577 | 38 | python method at src/pluggy/_result.py:31 |  |  | 0.672 |
| walker |  | 1589 | 12 | python method doc at src/pluggy/_result.py:31 |  |  | 0.672 |
| walker |  | 1593 | 4 | listing of '.claude' |  |  | 0.672 |
| ns | 1595 |  | 334 | _manager.py — every class + def name | 4.1 |  | 0.587 |
| walker |  | 1608 | 15 | listing of 'docs/examples' |  |  | 0.587 |
| walker |  | 1657 | 49 | python method doc at src/pluggy/_result.py:80 |  |  | 0.587 |
| walker |  | 1713 | 56 | python method doc at src/pluggy/_result.py:91 |  |  | 0.588 |
| walker |  | 1770 | 57 | python decl at src/pluggy/_callers.py:82 |  |  | 0.590 |
| walker |  | 1906 | 136 | python decl doc at src/pluggy/_warnings.py:10 |  |  | 0.593 |
| ns | 2037 |  | 442 | _hooks.py — every class + def name (markers, HookCaller, HookImpl, HookSpec) | 4.2 |  | 0.520 |
| walker |  | 2116 | 210 | python method sigs in src/pluggy/_tracing.py |  |  | 0.522 |
| walker |  | 2116 | 0 | python method at src/pluggy/_tracing.py:17 |  |  | 0.522 |
| walker |  | 2116 | 0 | python method at src/pluggy/_tracing.py:22 |  |  | 0.522 |
| walker |  | 2116 | 0 | python method at src/pluggy/_tracing.py:25 |  |  | 0.522 |
| walker |  | 2116 | 0 | python method at src/pluggy/_tracing.py:42 |  |  | 0.522 |
| walker |  | 2116 | 0 | python method at src/pluggy/_tracing.py:48 |  |  | 0.522 |
| walker |  | 2116 | 0 | python method at src/pluggy/_tracing.py:51 |  |  | 0.522 |
| walker |  | 2116 | 0 | python method at src/pluggy/_tracing.py:60 |  |  | 0.522 |
| walker |  | 2116 | 0 | python method at src/pluggy/_tracing.py:64 |  |  | 0.522 |
| walker |  | 2116 | 0 | python method at src/pluggy/_tracing.py:67 |  |  | 0.522 |
| walker |  | 2125 | 9 | python method body at src/pluggy/_tracing.py:48 body 49 |  |  | 0.522 |
| walker |  | 2137 | 12 | python method body at src/pluggy/_tracing.py:22 body 23 |  |  | 0.522 |
| ns | 2138 |  | 101 | _callers.py — every def + _multicall signature | 4.3 |  | 0.542 |
| walker |  | 2150 | 13 | python method body at src/pluggy/_tracing.py:64 body 65 |  |  | 0.542 |
| walker |  | 2167 | 17 | python method body at src/pluggy/_tracing.py:67 body 68 |  |  | 0.542 |
| walker |  | 2185 | 18 | python method body at src/pluggy/_tracing.py:60 body 61 |  |  | 0.542 |
| walker |  | 2236 | 51 | python decl doc at src/pluggy/_callers.py:82 |  |  | 0.542 |
| ns | 2275 |  | 137 | _result.py — Result class + every method (in full) | 4.4 |  | 0.564 |
| walker |  | 2352 | 116 | python decl names surface in src/pluggy/_manager.py |  |  | 0.567 |
| walker |  | 2352 | 0 | python decl at src/pluggy/_manager.py:42 |  |  | 0.567 |
| walker |  | 2352 | 0 | python decl at src/pluggy/_manager.py:52 |  |  | 0.567 |
| walker |  | 2352 | 0 | python decl at src/pluggy/_manager.py:65 |  |  | 0.567 |
| walker |  | 2352 | 0 | python decl at src/pluggy/_manager.py:525 |  |  | 0.567 |
| walker |  | 2377 | 25 | python decl at src/pluggy/_manager.py:83 |  |  | 0.567 |
| walker |  | 2388 | 11 | python decl doc at src/pluggy/_manager.py:65 |  |  | 0.567 |
| walker |  | 2429 | 41 | python decl doc at src/pluggy/_manager.py:52 |  |  | 0.567 |
| walker |  | 2445 | 16 | python decl body at src/pluggy/_manager.py:525 body 526 |  |  | 0.567 |
| walker |  | 2473 | 28 | python decl at src/pluggy/_manager.py:37 |  |  | 0.567 |
| ns | 2581 |  | 306 | _warnings.py — full file (Pluggy*Warning classes) | 4.5 |  | 0.579 |
| walker |  | 2617 | 144 | python decl doc at src/pluggy/_manager.py:83 |  |  | 0.581 |
| walker |  | 2676 | 59 | python imports in src/pluggy/_tracing.py |  |  | 0.581 |
| ns | 2682 |  | 101 | _tracing.py — class + def names | 4.6 |  | 0.596 |
| walker |  | 2684 | 8 | listing of 'docs/examples/eggsample' |  |  | 0.596 |
| walker |  | 2704 | 20 | listing of 'docs/examples/eggsample/eggsample' |  |  | 0.597 |
| walker |  | 2712 | 8 | python imports in docs/examples/eggsample/eggsample/__init__.py |  |  | 0.597 |
| walker |  | 2799 | 87 | python method doc at src/pluggy/_result.py:67 |  |  | 0.598 |
| walker |  | 2848 | 49 | python method body at src/pluggy/_tracing.py:17 body 18 |  |  | 0.598 |
| walker |  | 2866 | 18 | python decl names surface in docs/examples/eggsample/eggsample/__init__.py |  |  | 0.598 |
| ns | 2873 |  | 191 | PluginManager class docstring | 5.1 | 4.1 | 0.605 |
| walker |  | 2877 | 11 | python method body at src/pluggy/_result.py:42 body 45 |  |  | 0.605 |
| walker |  | 2978 | 101 | python imports in src/pluggy/_result.py |  |  | 0.605 |
| walker |  | 3033 | 55 | python method body at src/pluggy/_tracing.py:51 body 52 |  |  | 0.605 |
| ns | 3038 |  | 165 | HookspecMarker + HookimplMarker class docstrings | 5.2 | 4.2 | 0.591 |
| walker |  | 3046 | 13 | listing of '.github' |  |  | 0.591 |
| walker |  | 3050 | 4 | listing of '.github/workflows' |  |  | 0.591 |
| walker |  | 3064 | 14 | listing of 'scripts' |  |  | 0.591 |
| walker |  | 3245 | 181 | python decl names surface in src/pluggy/_hooks.py |  |  | 0.592 |
| walker |  | 3245 | 0 | python decl at src/pluggy/_hooks.py:40 |  |  | 0.592 |
| walker |  | 3245 | 0 | python decl at src/pluggy/_hooks.py:56 |  |  | 0.592 |
| walker |  | 3245 | 0 | python decl at src/pluggy/_hooks.py:281 |  |  | 0.592 |
| walker |  | 3252 | 7 | python decl at src/pluggy/_hooks.py:77 |  |  | 0.592 |
| walker |  | 3259 | 7 | python decl at src/pluggy/_hooks.py:164 |  |  | 0.593 |
| walker |  | 3269 | 10 | python decl doc at src/pluggy/_hooks.py:40 |  |  | 0.593 |
| walker |  | 3279 | 10 | python decl doc at src/pluggy/_hooks.py:56 |  |  | 0.593 |
| walker |  | 3293 | 14 | python class body at src/pluggy/_hooks.py:77 |  |  | 0.593 |
| walker |  | 3307 | 14 | python class body at src/pluggy/_hooks.py:164 |  |  | 0.593 |
| ns | 3369 |  | 331 | HookCaller __slots__ + the 6-bucket call-order comment | 5.3 | 4.2 | 0.562 |
| walker |  | 3384 | 77 | python decl doc at src/pluggy/_hooks.py:164 |  |  | 0.567 |
| walker |  | 3462 | 78 | python decl doc at src/pluggy/_hooks.py:77 |  |  | 0.585 |
| walker |  | 3596 | 134 | python method sigs in src/pluggy/_hooks.py |  |  | 0.589 |
| walker |  | 3596 | 0 | python method at src/pluggy/_hooks.py:88 |  |  | 0.589 |
| walker |  | 3596 | 0 | python method at src/pluggy/_hooks.py:175 |  |  | 0.589 |
| walker |  | 3608 | 12 | python method body at src/pluggy/_hooks.py:88 body 89 |  |  | 0.589 |
| walker |  | 3620 | 12 | python method body at src/pluggy/_hooks.py:175 body 176 |  |  | 0.589 |
| ns | 3651 |  | 282 | HookCaller._add_hookimpl body — the actual ordering algorithm | 5.4 | 4.2 | 0.567 |
| walker |  | 3656 | 36 | python decl at src/pluggy/_hooks.py:33 |  |  | 0.567 |
| walker |  | 3738 | 82 | python method at src/pluggy/_hooks.py:91 |  |  | 0.568 |
| walker |  | 3749 | 11 | python method body at src/pluggy/_hooks.py:91 body 99 |  |  | 0.568 |
| walker |  | 3839 | 90 | python method at src/pluggy/_hooks.py:101 |  |  | 0.569 |
| walker |  | 3855 | 16 | python method body at src/pluggy/_hooks.py:101 body 109 |  |  | 0.569 |
| walker |  | 4019 | 164 | python class body at src/pluggy/_hooks.py:40 |  |  | 0.570 |
| ns | 4080 |  | 429 | HookspecOpts + HookimplOpts — TypedDict bodies | 5.5 | 4.2 | 0.553 |
| walker |  | 4114 | 95 | python method at src/pluggy/_hooks.py:111 |  |  | 0.553 |
| walker |  | 4211 | 97 | python method at src/pluggy/_hooks.py:178 |  |  | 0.554 |
| walker |  | 4222 | 11 | python method body at src/pluggy/_hooks.py:178 body 188 |  |  | 0.554 |
| walker |  | 4327 | 105 | python method at src/pluggy/_hooks.py:190 |  |  | 0.556 |
| walker |  | 4343 | 16 | python method body at src/pluggy/_hooks.py:190 body 200 |  |  | 0.556 |
| walker |  | 4453 | 110 | python method at src/pluggy/_hooks.py:202 |  |  | 0.556 |
| ns | 4494 |  | 414 | Result API — force_result, force_exception, get_result bodies | 5.6 | 4.4 | 0.545 |
| walker |  | 4679 | 226 | python class body at src/pluggy/_hooks.py:56 |  |  | 0.587 |
| walker |  | 4754 | 75 | python decl body at src/pluggy/_hooks.py:281 body 282 |  |  | 0.587 |
| ns | 4801 |  | 307 | docs/index.rst H2 heading map — sections at exact line numbers | 5.7 | 3.2 | 0.557 |
| ns | 4855 |  | 54 | docs/examples/ FS listing | 6.1 |  | 0.560 |
| walker |  | 4867 | 113 | python decl names surface #1 in src/pluggy/_hooks.py |  |  | 0.564 |
| walker |  | 4867 | 0 | python decl at src/pluggy/_hooks.py:293 |  |  | 0.564 |
| walker |  | 4867 | 0 | python decl at src/pluggy/_hooks.py:382 |  |  | 0.564 |
| walker |  | 4867 | 0 | python decl at src/pluggy/_hooks.py:593 |  |  | 0.564 |
| walker |  | 4873 | 6 | python decl at src/pluggy/_hooks.py:358 |  |  | 0.565 |
| walker |  | 4879 | 6 | python decl at src/pluggy/_hooks.py:638 |  |  | 0.567 |
| walker |  | 4885 | 6 | python decl at src/pluggy/_hooks.py:696 |  |  | 0.568 |
| walker |  | 4900 | 15 | python decl doc at src/pluggy/_hooks.py:382 |  |  | 0.568 |
| walker |  | 4918 | 18 | python decl doc at src/pluggy/_hooks.py:638 |  |  | 0.568 |
| walker |  | 4949 | 31 | python decl doc at src/pluggy/_hooks.py:358 |  |  | 0.568 |
| walker |  | 4994 | 45 | python class body at src/pluggy/_hooks.py:358 |  |  | 0.570 |
| walker |  | 5020 | 26 | python decl at src/pluggy/_hooks.py:377 |  |  | 0.570 |
| ns | 5075 |  | 220 | Eggsample hookspecs.py — host-side hook specifications | 6.2 |  | 0.555 |
| walker |  | 5108 | 88 | python class body at src/pluggy/_hooks.py:382 |  |  | 0.558 |
| walker |  | 5196 | 88 | python class body at src/pluggy/_hooks.py:696 |  |  | 0.558 |
| walker |  | 5226 | 30 | python decl doc at src/pluggy/_hooks.py:593 |  |  | 0.558 |
| walker |  | 5338 | 112 | python class body at src/pluggy/_hooks.py:638 |  |  | 0.558 |
| walker |  | 5410 | 72 | python decl doc at src/pluggy/_hooks.py:293 |  |  | 0.558 |
| ns | 5479 |  | 404 | Eggsample lib.py + eggsample-spam (host impls + plugin impls) | 6.3 |  | 0.535 |
| walker |  | 5794 | 384 | python method sigs #1 in src/pluggy/_hooks.py |  |  | 0.579 |
| walker |  | 5794 | 0 | python method at src/pluggy/_hooks.py:365 |  |  | 0.579 |
| walker |  | 5794 | 0 | python method at src/pluggy/_hooks.py:420 |  |  | 0.579 |
| walker |  | 5794 | 0 | python method at src/pluggy/_hooks.py:438 |  |  | 0.579 |
| walker |  | 5794 | 0 | python method at src/pluggy/_hooks.py:442 |  |  | 0.579 |
| walker |  | 5794 | 0 | python method at src/pluggy/_hooks.py:449 |  |  | 0.579 |
| walker |  | 5794 | 0 | python method at src/pluggy/_hooks.py:453 |  |  | 0.579 |
| walker |  | 5794 | 0 | python method at src/pluggy/_hooks.py:477 |  |  | 0.579 |
| walker |  | 5794 | 0 | python method at src/pluggy/_hooks.py:499 |  |  | 0.579 |
| walker |  | 5794 | 0 | python method at src/pluggy/_hooks.py:577 |  |  | 0.579 |
| walker |  | 5794 | 0 | python method at src/pluggy/_hooks.py:612 |  |  | 0.579 |
| walker |  | 5794 | 0 | python method at src/pluggy/_hooks.py:634 |  |  | 0.579 |
| walker |  | 5794 | 0 | python method at src/pluggy/_hooks.py:692 |  |  | 0.579 |
| walker |  | 5794 | 0 | python method at src/pluggy/_hooks.py:709 |  |  | 0.579 |
| walker |  | 5815 | 21 | python method at src/pluggy/_hooks.py:626 |  |  | 0.581 |
| walker |  | 5825 | 10 | python method doc at src/pluggy/_hooks.py:365 |  |  | 0.581 |
| walker |  | 5838 | 13 | python method doc at src/pluggy/_hooks.py:449 |  |  | 0.581 |
| walker |  | 5867 | 29 | python method at src/pluggy/_hooks.py:543 |  |  | 0.581 |
| walker |  | 5877 | 10 | python method body at src/pluggy/_hooks.py:420 body 421 |  |  | 0.581 |
| walker |  | 5888 | 11 | python method body at src/pluggy/_hooks.py:626 body 628 |  |  | 0.581 |
| walker |  | 5904 | 16 | python method doc at src/pluggy/_hooks.py:438 |  |  | 0.581 |
| walker |  | 5919 | 15 | python method at src/pluggy/_hooks.py:480 |  |  | 0.581 |
| walker |  | 5932 | 13 | python method body at src/pluggy/_hooks.py:449 body 451 |  |  | 0.581 |
| walker |  | 5971 | 39 | python method at src/pluggy/_hooks.py:424 |  |  | 0.581 |
| walker |  | 5987 | 16 | python method at src/pluggy/_hooks.py:618 |  |  | 0.584 |
| walker |  | 6001 | 14 | python method body at src/pluggy/_hooks.py:438 body 440 |  |  | 0.584 |
| walker |  | 6016 | 15 | python method body at src/pluggy/_hooks.py:477 body 478 |  |  | 0.584 |
| ns | 6020 |  | 541 | docs/index.rst — "Call time order" section (tryfirst / trylast) | 6.4 | 5.7 | 0.552 |
| walker |  | 6028 | 12 | python method doc at src/pluggy/_hooks.py:453 |  |  | 0.552 |
| walker |  | 6045 | 17 | python method body at src/pluggy/_hooks.py:634 body 635 |  |  | 0.552 |
| walker |  | 6095 | 50 | python method at src/pluggy/_hooks.py:516 |  |  | 0.552 |
| walker |  | 6119 | 24 | python method at src/pluggy/_hooks.py:630 |  |  | 0.555 |
| walker |  | 6180 | 61 | python method at src/pluggy/_hooks.py:656 |  |  | 0.555 |
| walker |  | 6192 | 12 | python method doc at src/pluggy/_hooks.py:656 |  |  | 0.555 |
| walker |  | 6260 | 68 | python method at src/pluggy/_hooks.py:393 |  |  | 0.562 |
| walker |  | 6272 | 12 | python method doc at src/pluggy/_hooks.py:393 |  |  | 0.563 |
| walker |  | 6291 | 19 | python method doc at src/pluggy/_hooks.py:577 |  |  | 0.563 |
| walker |  | 6336 | 45 | python method doc at src/pluggy/_hooks.py:543 |  |  | 0.563 |
| walker |  | 6361 | 25 | python method body at src/pluggy/_hooks.py:692 body 693 |  |  | 0.563 |
| walker |  | 6374 | 13 | python method body at src/pluggy/_hooks.py:630 body 632 |  |  | 0.563 |
| walker |  | 6439 | 65 | python method doc at src/pluggy/_hooks.py:499 |  |  | 0.563 |
| ns | 6517 |  | 497 | docs/index.rst — Wrappers (new-style) wrapper-protocol summary | 6.5 | 5.7 | 0.544 |
| walker |  | 6532 | 93 | python method doc at src/pluggy/_hooks.py:516 |  |  | 0.544 |
| ns | 6579 |  | 62 | testing/ FS listing | 7.1 |  | 0.551 |
| walker |  | 6594 | 62 | python method body at src/pluggy/_hooks.py:612 body 613 |  |  | 0.551 |
| ns | 6827 |  | 248 | testing/conftest.py — pm + he_pm fixtures | 7.2 |  | 0.539 |
| ns | 7193 |  | 366 | test_pluginmanager.py — every test fn name | 7.3 |  | 0.524 |
| walker |  | 7208 | 614 | python method sigs in src/pluggy/_manager.py |  |  | 0.568 |
| walker |  | 7208 | 0 | python method at src/pluggy/_manager.py:59 |  |  | 0.568 |
| walker |  | 7208 | 0 | python method at src/pluggy/_manager.py:68 |  |  | 0.568 |
| walker |  | 7208 | 0 | python method at src/pluggy/_manager.py:76 |  |  | 0.568 |
| walker |  | 7208 | 0 | python method at src/pluggy/_manager.py:79 |  |  | 0.568 |
| walker |  | 7208 | 0 | python method at src/pluggy/_manager.py:125 |  |  | 0.568 |
| walker |  | 7208 | 0 | python method at src/pluggy/_manager.py:176 |  |  | 0.568 |
| walker |  | 7208 | 0 | python method at src/pluggy/_manager.py:233 |  |  | 0.568 |
| walker |  | 7208 | 0 | python method at src/pluggy/_manager.py:238 |  |  | 0.568 |
| walker |  | 7208 | 0 | python method at src/pluggy/_manager.py:242 |  |  | 0.568 |
| walker |  | 7208 | 0 | python method at src/pluggy/_manager.py:252 |  |  | 0.568 |
| walker |  | 7208 | 0 | python method at src/pluggy/_manager.py:296 |  |  | 0.568 |
| walker |  | 7208 | 0 | python method at src/pluggy/_manager.py:300 |  |  | 0.568 |
| walker |  | 7208 | 0 | python method at src/pluggy/_manager.py:304 |  |  | 0.568 |
| walker |  | 7208 | 0 | python method at src/pluggy/_manager.py:315 |  |  | 0.568 |
| walker |  | 7208 | 0 | python method at src/pluggy/_manager.py:319 |  |  | 0.568 |
| walker |  | 7208 | 0 | python method at src/pluggy/_manager.py:323 |  |  | 0.568 |
| walker |  | 7208 | 0 | python method at src/pluggy/_manager.py:331 |  |  | 0.568 |
| walker |  | 7208 | 0 | python method at src/pluggy/_manager.py:379 |  |  | 0.568 |
| walker |  | 7208 | 0 | python method at src/pluggy/_manager.py:395 |  |  | 0.568 |
| walker |  | 7208 | 0 | python method at src/pluggy/_manager.py:425 |  |  | 0.568 |
| walker |  | 7208 | 0 | python method at src/pluggy/_manager.py:430 |  |  | 0.568 |
| walker |  | 7208 | 0 | python method at src/pluggy/_manager.py:434 |  |  | 0.568 |
| walker |  | 7208 | 0 | python method at src/pluggy/_manager.py:487 |  |  | 0.568 |
| walker |  | 7216 | 8 | python method at src/pluggy/_manager.py:100 |  |  | 0.568 |
| walker |  | 7227 | 11 | python method at src/pluggy/_manager.py:71 |  |  | 0.570 |
| walker |  | 7239 | 12 | python method doc at src/pluggy/_manager.py:300 |  |  | 0.570 |
| walker |  | 7265 | 26 | python method at src/pluggy/_manager.py:512 |  |  | 0.570 |
| walker |  | 7274 | 9 | python method body at src/pluggy/_manager.py:68 body 69 |  |  | 0.570 |
| walker |  | 7287 | 13 | python method doc at src/pluggy/_manager.py:238 |  |  | 0.570 |
| walker |  | 7300 | 13 | python method doc at src/pluggy/_manager.py:296 |  |  | 0.570 |
| walker |  | 7329 | 29 | python method at src/pluggy/_manager.py:278 |  |  | 0.570 |
| walker |  | 7358 | 29 | python method at src/pluggy/_manager.py:450 |  |  | 0.570 |
| walker |  | 7373 | 15 | python method doc at src/pluggy/_manager.py:319 |  |  | 0.570 |
| walker |  | 7389 | 16 | python method doc at src/pluggy/_manager.py:233 |  |  | 0.570 |
| walker |  | 7405 | 16 | python method doc at src/pluggy/_manager.py:315 |  |  | 0.570 |
| walker |  | 7438 | 33 | python method at src/pluggy/_manager.py:201 |  |  | 0.570 |
| walker |  | 7457 | 19 | python method doc at src/pluggy/_manager.py:430 |  |  | 0.570 |
| walker |  | 7470 | 13 | python method body at src/pluggy/_manager.py:76 body 77 |  |  | 0.570 |
| walker |  | 7483 | 13 | python method body at src/pluggy/_manager.py:425 body 428 |  |  | 0.570 |
| walker |  | 7497 | 14 | python method body at src/pluggy/_manager.py:315 body 317 |  |  | 0.570 |
| walker |  | 7511 | 14 | python method body at src/pluggy/_manager.py:430 body 432 |  |  | 0.570 |
| walker |  | 7526 | 15 | python method body at src/pluggy/_manager.py:319 body 321 |  |  | 0.570 |
| walker |  | 7553 | 27 | python method doc at src/pluggy/_manager.py:323 |  |  | 0.570 |
| walker |  | 7581 | 28 | python method doc at src/pluggy/_manager.py:242 |  |  | 0.570 |
| walker |  | 7609 | 28 | python method doc at src/pluggy/_manager.py:425 |  |  | 0.570 |
| ns | 7623 |  | 430 | test_hookcaller.py + test_multicall.py — every test fn name | 7.4 |  | 0.554 |
| walker |  | 7628 | 19 | python method body at src/pluggy/_manager.py:79 body 80 |  |  | 0.554 |
| walker |  | 7648 | 20 | python method body at src/pluggy/_manager.py:300 body 302 |  |  | 0.554 |
| walker |  | 7683 | 35 | python method doc at src/pluggy/_manager.py:487 |  |  | 0.554 |
| walker |  | 7705 | 22 | python method body at src/pluggy/_manager.py:71 body 73 |  |  | 0.554 |
| walker |  | 7747 | 42 | python method doc at src/pluggy/_manager.py:379 |  |  | 0.554 |
| walker |  | 7771 | 24 | python method body at src/pluggy/_manager.py:238 body 240 |  |  | 0.554 |
| walker |  | 7795 | 24 | python method body at src/pluggy/_manager.py:296 body 298 |  |  | 0.554 |
| walker |  | 7845 | 50 | python method doc at src/pluggy/_manager.py:512 |  |  | 0.554 |
| walker |  | 7901 | 56 | python method doc at src/pluggy/_manager.py:252 |  |  | 0.554 |
| walker |  | 7959 | 58 | python method doc at src/pluggy/_manager.py:434 |  |  | 0.554 |
| walker |  | 7990 | 31 | python method body at src/pluggy/_manager.py:59 body 60 |  |  | 0.554 |
| walker |  | 7998 | 8 | python method body at src/pluggy/_manager.py:242 body 250 |  |  | 0.554 |
| walker |  | 8006 | 8 | python method body at src/pluggy/_manager.py:323 body 329 |  |  | 0.554 |
| ns | 8023 |  | 400 | Smaller test files — every test fn name | 7.5 |  | 0.539 |
| walker |  | 8083 | 77 | python method doc at src/pluggy/_manager.py:201 |  |  | 0.539 |
| walker |  | 8173 | 90 | python method doc at src/pluggy/_manager.py:304 |  |  | 0.539 |
| walker |  | 8264 | 91 | python method doc at src/pluggy/_manager.py:395 |  |  | 0.539 |
| walker |  | 8274 | 10 | python method body at src/pluggy/_manager.py:233 body 235 |  |  | 0.539 |
| ns | 8324 |  | 301 | pyproject.toml — [project] essentials (skip classifier list) | 7.6 |  | 0.529 |
| walker |  | 8394 | 120 | python method doc at src/pluggy/_manager.py:125 |  |  | 0.529 |
| walker |  | 8517 | 123 | python method doc at src/pluggy/_manager.py:176 |  |  | 0.529 |
| walker |  | 8529 | 12 | python method body at src/pluggy/_manager.py:304 body 313 |  |  | 0.529 |
| ns | 8576 |  | 252 | pyproject.toml — [tool.ruff.lint] config | 7.7 | 7.6 | 0.522 |
| walker |  | 8665 | 136 | python method doc at src/pluggy/_manager.py:278 |  |  | 0.522 |
| walker |  | 8829 | 164 | python method doc at src/pluggy/_manager.py:450 |  |  | 0.522 |
| walker |  | 8844 | 15 | python method body at src/pluggy/_manager.py:233 body 236 |  |  | 0.522 |
| ns | 8859 |  | 283 | tox.ini — [tox] + [testenv] + [pytest] (skip release/docs envs) | 7.8 |  | 0.513 |
| walker |  | 8910 | 66 | python decl body at src/pluggy/_callers.py:60 body 64 |  |  | 0.513 |
| walker |  | 9071 | 161 | python imports in src/pluggy/_callers.py |  |  | 0.513 |
| walker |  | 9168 | 97 | python method at src/pluggy/_manager.py:114 |  |  | 0.513 |
| walker |  | 9190 | 22 | python method body at src/pluggy/_manager.py:114 body 123 |  |  | 0.513 |
| ns | 9238 |  | 379 | CHANGELOG.rst — pluggy 1.6.0 entry only | 7.9 |  | 0.502 |
| walker |  | 9265 | 75 | python decl body at src/pluggy/_manager.py:42 body 43 |  |  | 0.502 |
| ns | 9300 |  | 62 | changelog/ + downstream/ FS listings | 7.10 |  | 0.509 |
| walker |  | 9478 | 213 | python class body at src/pluggy/_hooks.py:593 |  |  | 0.509 |
| walker |  | 9497 | 19 | python method body at src/pluggy/_manager.py:304 body 312 |  |  | 0.509 |
| walker |  | 9505 | 8 | python method body at src/pluggy/_manager.py:252 body 258 |  |  | 0.509 |
| walker |  | 9513 | 8 | python method body at src/pluggy/_manager.py:278 body 294 |  |  | 0.509 |
| walker |  | 9525 | 12 | python decl names surface in scripts/towncrier-draft-to-file.py |  |  | 0.509 |
| walker |  | 9525 | 0 | python decl at scripts/towncrier-draft-to-file.py:5 |  |  | 0.509 |
| walker |  | 9733 | 208 | python imports in src/pluggy/_hooks.py |  |  | 0.509 |
| ns | 9819 |  | 519 | _callers._multicall body — the actual call loop | 7.11 | 4.3 | 0.495 |
| walker |  | 9834 | 101 | python method body at src/pluggy/_hooks.py:709 body 710 |  |  | 0.495 |
| walker |  | 9938 | 104 | python method body at src/pluggy/_hooks.py:424 body 429 |  |  | 0.495 |
| ns | 9946 |  | 127 | Top-level meta — AGENTS.md, SECURITY.md, MANIFEST.in | 7.12 |  | 0.491 |
| ns | 9964 |  | 18 | scripts/ FS + .github/workflows/ FS | 7.13 |  | 0.493 |
| walker |  | 9989 | 51 | python method body at src/pluggy/_hooks.py:618 body 620 |  |  | 0.493 |
| ns | 9996 |  | 32 | docs/ root FS + docs/requirements.txt | 7.14 |  | 0.497 |

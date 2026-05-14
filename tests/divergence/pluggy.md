Score(3000)=0.504 I=0.779 C=0.326 ns_rows≤3K=15/40 (reached=6 partial=1 missing=8)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 96 | 96 | listing of '.' |  |  | 1.000 |
| ns | 96 |  | 96 | Top-level repo listing | 1.1 |  | 1.000 |
| walker |  | 100 | 4 | listing of 'src' |  |  | 1.000 |
| walker |  | 104 | 4 | listing of '.claude' |  |  | 1.000 |
| walker |  | 206 | 102 | README headline in README.rst |  |  | 1.000 |
| ns | 213 |  | 117 | README lede + tagline | 1.2 |  | 0.943 |
| walker |  | 225 | 19 | listing of 'changelog' |  |  | 0.943 |
| ns | 256 |  | 43 | Source package layout (src/pluggy/) | 1.3 |  | 0.827 |
| walker |  | 268 | 43 | listing of 'src/pluggy' |  |  | 0.952 |
| walker |  | 286 | 18 | python decl names surface in src/pluggy/__init__.py |  |  | 0.952 |
| walker |  | 286 | 0 | python decl at src/pluggy/__init__.py:32 |  |  | 0.952 |
| walker |  | 318 | 32 | listing of 'docs' |  |  | 0.953 |
| walker |  | 321 | 3 | listing of 'docs/_static' |  |  | 0.953 |
| walker |  | 325 | 4 | listing of 'docs/_static/img' |  |  | 0.953 |
| walker |  | 368 | 43 | listing of 'downstream' |  |  | 0.954 |
| walker |  | 390 | 22 | python decl names surface in src/pluggy/_warnings.py |  |  | 0.954 |
| walker |  | 390 | 0 | python decl at src/pluggy/_warnings.py:4 |  |  | 0.954 |
| walker |  | 404 | 14 | python decl at src/pluggy/_warnings.py:10 |  |  | 0.955 |
| ns | 406 |  | 150 | __all__ — full public-name list | 1.4 |  | 0.784 |
| walker |  | 417 | 13 | python class body at src/pluggy/_warnings.py:4 |  |  | 0.784 |
| walker |  | 430 | 13 | python class body at src/pluggy/_warnings.py:10 |  |  | 0.784 |
| walker |  | 439 | 9 | python imports in src/pluggy/_warnings.py |  |  | 0.784 |
| walker |  | 453 | 14 | python decl doc at src/pluggy/_warnings.py:4 |  |  | 0.784 |
| ns | 624 |  | 218 | Toy example — first half (spec + plugin classes) | 2.1 |  | 0.641 |
| walker |  | 769 | 316 | python imports in src/pluggy/__init__.py |  |  | 0.800 |
| walker |  | 809 | 40 | python imports in docs/conf.py |  |  | 0.800 |
| ns | 829 |  | 205 | Toy example — second half (PluginManager wiring + call) | 2.2 | 2.1 | 0.705 |
| walker |  | 864 | 55 | python decl names surface in src/pluggy/_tracing.py |  |  | 0.706 |
| walker |  | 864 | 0 | python decl at src/pluggy/_tracing.py:16 |  |  | 0.706 |
| walker |  | 864 | 0 | python decl at src/pluggy/_tracing.py:59 |  |  | 0.706 |
| walker |  | 920 | 56 | python decl body at src/pluggy/__init__.py:32 body 33 |  |  | 0.706 |
| walker |  | 935 | 15 | listing of 'docs/examples' |  |  | 0.706 |
| walker |  | 943 | 8 | listing of 'docs/examples/eggsample' |  |  | 0.706 |
| ns | 995 |  | 166 | __init__.py re-exports — name → owning module | 3.1 | 1.4 | 0.730 |
| walker |  | 1011 | 68 | python decl names surface in src/pluggy/_callers.py |  |  | 0.731 |
| walker |  | 1040 | 29 | python decl at src/pluggy/_callers.py:27 |  |  | 0.731 |
| walker |  | 1071 | 31 | python decl doc at src/pluggy/_callers.py:27 |  |  | 0.731 |
| walker |  | 1140 | 69 | python decl names surface in src/pluggy/_result.py |  |  | 0.731 |
| walker |  | 1140 | 0 | python decl at src/pluggy/_result.py:20 |  |  | 0.731 |
| walker |  | 1149 | 9 | python decl at src/pluggy/_result.py:24 |  |  | 0.731 |
| walker |  | 1158 | 9 | python decl doc at src/pluggy/_result.py:20 |  |  | 0.731 |
| walker |  | 1178 | 20 | python class body at src/pluggy/_result.py:24 |  |  | 0.731 |
| walker |  | 1212 | 34 | python decl doc at src/pluggy/_result.py:24 |  |  | 0.731 |
| walker |  | 1232 | 20 | listing of 'docs/examples/eggsample/eggsample' |  |  | 0.732 |
| walker |  | 1259 | 27 | python decl at src/pluggy/_callers.py:70 |  |  | 0.732 |
| ns | 1261 |  | 266 | docs/index.rst lede — "what is pluggy" | 3.2 |  | 0.667 |
| walker |  | 1267 | 8 | python imports in docs/examples/eggsample/eggsample/__init__.py |  |  | 0.667 |
| walker |  | 1402 | 135 | python decl names surface in docs/conf.py |  |  | 0.667 |
| walker |  | 1470 | 68 | python decl at docs/conf.py:9 |  |  | 0.667 |
| walker |  | 1502 | 32 | python decl at src/pluggy/_callers.py:60 |  |  | 0.667 |
| ns | 1595 |  | 334 | _manager.py — every class + def name | 4.1 |  | 0.583 |
| walker |  | 1599 | 97 | python method sigs in src/pluggy/_result.py |  |  | 0.584 |
| walker |  | 1599 | 0 | python method at src/pluggy/_result.py:67 |  |  | 0.584 |
| walker |  | 1599 | 0 | python method at src/pluggy/_result.py:80 |  |  | 0.584 |
| walker |  | 1599 | 0 | python method at src/pluggy/_result.py:91 |  |  | 0.584 |
| walker |  | 1612 | 13 | python method at src/pluggy/_result.py:51 |  |  | 0.585 |
| walker |  | 1627 | 15 | python method at src/pluggy/_result.py:42 |  |  | 0.585 |
| walker |  | 1649 | 22 | python method at src/pluggy/_result.py:56 |  |  | 0.586 |
| walker |  | 1661 | 12 | python method doc at src/pluggy/_result.py:42 |  |  | 0.586 |
| walker |  | 1673 | 12 | python method doc at src/pluggy/_result.py:51 |  |  | 0.586 |
| walker |  | 1685 | 12 | python method doc at src/pluggy/_result.py:56 |  |  | 0.586 |
| walker |  | 1695 | 10 | python method body at src/pluggy/_result.py:51 body 54 |  |  | 0.586 |
| walker |  | 1733 | 38 | python method at src/pluggy/_result.py:31 |  |  | 0.586 |
| walker |  | 1745 | 12 | python method doc at src/pluggy/_result.py:31 |  |  | 0.586 |
| walker |  | 1861 | 116 | python decl names surface in src/pluggy/_manager.py |  |  | 0.590 |
| walker |  | 1861 | 0 | python decl at src/pluggy/_manager.py:42 |  |  | 0.590 |
| walker |  | 1861 | 0 | python decl at src/pluggy/_manager.py:52 |  |  | 0.590 |
| walker |  | 1861 | 0 | python decl at src/pluggy/_manager.py:65 |  |  | 0.590 |
| walker |  | 1861 | 0 | python decl at src/pluggy/_manager.py:83 |  |  | 0.590 |
| walker |  | 1861 | 0 | python decl at src/pluggy/_manager.py:525 |  |  | 0.590 |
| walker |  | 1872 | 11 | python decl doc at src/pluggy/_manager.py:65 |  |  | 0.590 |
| walker |  | 1913 | 41 | python decl doc at src/pluggy/_manager.py:52 |  |  | 0.590 |
| walker |  | 1975 | 62 | listing of 'testing' |  |  | 0.591 |
| walker |  | 1991 | 16 | python decl body at src/pluggy/_manager.py:525 body 526 |  |  | 0.591 |
| walker |  | 2019 | 28 | python decl at src/pluggy/_manager.py:37 |  |  | 0.591 |
| ns | 2037 |  | 442 | _hooks.py — every class + def name (markers, HookCaller, HookImpl, HookSpec) | 4.2 |  | 0.518 |
| walker |  | 2078 | 59 | python imports in src/pluggy/_tracing.py |  |  | 0.518 |
| walker |  | 2127 | 49 | python method doc at src/pluggy/_result.py:80 |  |  | 0.518 |
| ns | 2138 |  | 101 | _callers.py — every def + _multicall signature | 4.3 |  | 0.511 |
| walker |  | 2183 | 56 | python method doc at src/pluggy/_result.py:91 |  |  | 0.512 |
| ns | 2275 |  | 137 | _result.py — Result class + every method (in full) | 4.4 |  | 0.536 |
| walker |  | 2332 | 149 | python decl names surface #1 in docs/conf.py |  |  | 0.536 |
| walker |  | 2332 | 0 | python decl at docs/conf.py:106 |  |  | 0.536 |
| walker |  | 2332 | 0 | python decl at docs/conf.py:130 |  |  | 0.536 |
| walker |  | 2339 | 7 | python decl body at docs/conf.py:130 body 131 |  |  | 0.536 |
| walker |  | 2357 | 18 | python decl doc at docs/conf.py:106 |  |  | 0.536 |
| walker |  | 2387 | 30 | python decl at docs/conf.py:54 |  |  | 0.536 |
| walker |  | 2468 | 81 | python decl at docs/conf.py:83 |  |  | 0.536 |
| ns | 2581 |  | 306 | _warnings.py — full file (Pluggy*Warning classes) | 4.5 |  | 0.508 |
| walker |  | 2604 | 136 | python decl at docs/conf.py:96 |  |  | 0.508 |
| ns | 2682 |  | 101 | _tracing.py — class + def names | 4.6 |  | 0.497 |
| walker |  | 2750 | 146 | python decl at docs/conf.py:41 |  |  | 0.497 |
| ns | 2873 |  | 191 | PluginManager class docstring | 5.1 | 4.1 | 0.482 |
| walker |  | 2908 | 158 | python decl at docs/conf.py:66 |  |  | 0.482 |
| walker |  | 2965 | 57 | python decl at src/pluggy/_callers.py:82 |  |  | 0.504 |
| ns | 3038 |  | 165 | HookspecMarker + HookimplMarker class docstrings | 5.2 | 4.2 | 0.492 |
| walker |  | 3066 | 101 | python imports in src/pluggy/_result.py |  |  | 0.492 |
| walker |  | 3276 | 210 | python method sigs in src/pluggy/_tracing.py |  |  | 0.522 |
| walker |  | 3276 | 0 | python method at src/pluggy/_tracing.py:17 |  |  | 0.522 |
| walker |  | 3276 | 0 | python method at src/pluggy/_tracing.py:22 |  |  | 0.522 |
| walker |  | 3276 | 0 | python method at src/pluggy/_tracing.py:25 |  |  | 0.522 |
| walker |  | 3276 | 0 | python method at src/pluggy/_tracing.py:42 |  |  | 0.522 |
| walker |  | 3276 | 0 | python method at src/pluggy/_tracing.py:48 |  |  | 0.522 |
| walker |  | 3276 | 0 | python method at src/pluggy/_tracing.py:51 |  |  | 0.522 |
| walker |  | 3276 | 0 | python method at src/pluggy/_tracing.py:60 |  |  | 0.522 |
| walker |  | 3276 | 0 | python method at src/pluggy/_tracing.py:64 |  |  | 0.522 |
| walker |  | 3276 | 0 | python method at src/pluggy/_tracing.py:67 |  |  | 0.522 |
| walker |  | 3285 | 9 | python method body at src/pluggy/_tracing.py:48 body 49 |  |  | 0.522 |
| walker |  | 3297 | 12 | python method body at src/pluggy/_tracing.py:22 body 23 |  |  | 0.522 |
| walker |  | 3310 | 13 | python method body at src/pluggy/_tracing.py:64 body 65 |  |  | 0.522 |
| walker |  | 3327 | 17 | python method body at src/pluggy/_tracing.py:67 body 68 |  |  | 0.522 |
| walker |  | 3345 | 18 | python method body at src/pluggy/_tracing.py:60 body 61 |  |  | 0.522 |
| ns | 3369 |  | 331 | HookCaller __slots__ + the 6-bucket call-order comment | 5.3 | 4.2 | 0.494 |
| walker |  | 3526 | 181 | python decl names surface in src/pluggy/_hooks.py |  |  | 0.495 |
| walker |  | 3526 | 0 | python decl at src/pluggy/_hooks.py:40 |  |  | 0.495 |
| walker |  | 3526 | 0 | python decl at src/pluggy/_hooks.py:56 |  |  | 0.495 |
| walker |  | 3526 | 0 | python decl at src/pluggy/_hooks.py:281 |  |  | 0.495 |
| walker |  | 3533 | 7 | python decl at src/pluggy/_hooks.py:77 |  |  | 0.496 |
| walker |  | 3540 | 7 | python decl at src/pluggy/_hooks.py:164 |  |  | 0.496 |
| walker |  | 3550 | 10 | python decl doc at src/pluggy/_hooks.py:40 |  |  | 0.496 |
| walker |  | 3560 | 10 | python decl doc at src/pluggy/_hooks.py:56 |  |  | 0.496 |
| walker |  | 3574 | 14 | python class body at src/pluggy/_hooks.py:77 |  |  | 0.496 |
| walker |  | 3588 | 14 | python class body at src/pluggy/_hooks.py:164 |  |  | 0.496 |
| ns | 3651 |  | 282 | HookCaller._add_hookimpl body — the actual ordering algorithm | 5.4 | 4.2 | 0.478 |
| walker |  | 3665 | 77 | python decl doc at src/pluggy/_hooks.py:164 |  |  | 0.484 |
| walker |  | 3743 | 78 | python decl doc at src/pluggy/_hooks.py:77 |  |  | 0.502 |
| walker |  | 3877 | 134 | python method sigs in src/pluggy/_hooks.py |  |  | 0.506 |
| walker |  | 3877 | 0 | python method at src/pluggy/_hooks.py:88 |  |  | 0.506 |
| walker |  | 3877 | 0 | python method at src/pluggy/_hooks.py:175 |  |  | 0.506 |
| walker |  | 3889 | 12 | python method body at src/pluggy/_hooks.py:88 body 89 |  |  | 0.506 |
| walker |  | 3901 | 12 | python method body at src/pluggy/_hooks.py:175 body 176 |  |  | 0.506 |
| walker |  | 3937 | 36 | python decl at src/pluggy/_hooks.py:33 |  |  | 0.506 |
| walker |  | 4019 | 82 | python method at src/pluggy/_hooks.py:91 |  |  | 0.507 |
| walker |  | 4030 | 11 | python method body at src/pluggy/_hooks.py:91 body 99 |  |  | 0.507 |
| ns | 4080 |  | 429 | HookspecOpts + HookimplOpts — TypedDict bodies | 5.5 | 4.2 | 0.482 |
| walker |  | 4120 | 90 | python method at src/pluggy/_hooks.py:101 |  |  | 0.484 |
| walker |  | 4136 | 16 | python method body at src/pluggy/_hooks.py:101 body 109 |  |  | 0.484 |
| walker |  | 4300 | 164 | python class body at src/pluggy/_hooks.py:40 |  |  | 0.496 |
| walker |  | 4395 | 95 | python method at src/pluggy/_hooks.py:111 |  |  | 0.496 |
| walker |  | 4492 | 97 | python method at src/pluggy/_hooks.py:178 |  |  | 0.497 |
| ns | 4494 |  | 414 | Result API — force_result, force_exception, get_result bodies | 5.6 | 4.4 | 0.480 |
| walker |  | 4503 | 11 | python method body at src/pluggy/_hooks.py:178 body 188 |  |  | 0.480 |
| walker |  | 4549 | 46 | plaintext config downstream/.gitignore |  |  | 0.480 |
| walker |  | 4600 | 51 | python decl doc at src/pluggy/_callers.py:82 |  |  | 0.480 |
| walker |  | 4705 | 105 | python method at src/pluggy/_hooks.py:190 |  |  | 0.481 |
| walker |  | 4721 | 16 | python method body at src/pluggy/_hooks.py:190 body 200 |  |  | 0.481 |
| ns | 4801 |  | 307 | docs/index.rst H2 heading map — sections at exact line numbers | 5.7 | 3.2 | 0.456 |
| walker |  | 4831 | 110 | python method at src/pluggy/_hooks.py:202 |  |  | 0.456 |
| ns | 4855 |  | 54 | docs/examples/ FS listing | 6.1 |  | 0.463 |
| walker |  | 4944 | 113 | python decl names surface #1 in src/pluggy/_hooks.py |  |  | 0.468 |
| walker |  | 4944 | 0 | python decl at src/pluggy/_hooks.py:293 |  |  | 0.468 |
| walker |  | 4944 | 0 | python decl at src/pluggy/_hooks.py:382 |  |  | 0.468 |
| walker |  | 4944 | 0 | python decl at src/pluggy/_hooks.py:593 |  |  | 0.468 |
| walker |  | 4950 | 6 | python decl at src/pluggy/_hooks.py:358 |  |  | 0.469 |
| walker |  | 4956 | 6 | python decl at src/pluggy/_hooks.py:638 |  |  | 0.470 |
| walker |  | 4962 | 6 | python decl at src/pluggy/_hooks.py:696 |  |  | 0.472 |
| walker |  | 4977 | 15 | python decl doc at src/pluggy/_hooks.py:382 |  |  | 0.472 |
| walker |  | 4995 | 18 | python decl doc at src/pluggy/_hooks.py:638 |  |  | 0.472 |
| walker |  | 5026 | 31 | python decl doc at src/pluggy/_hooks.py:358 |  |  | 0.472 |
| walker |  | 5071 | 45 | python class body at src/pluggy/_hooks.py:358 |  |  | 0.474 |
| ns | 5075 |  | 220 | Eggsample hookspecs.py — host-side hook specifications | 6.2 |  | 0.462 |
| walker |  | 5097 | 26 | python decl at src/pluggy/_hooks.py:377 |  |  | 0.462 |
| walker |  | 5185 | 88 | python class body at src/pluggy/_hooks.py:382 |  |  | 0.465 |
| walker |  | 5273 | 88 | python class body at src/pluggy/_hooks.py:696 |  |  | 0.465 |
| walker |  | 5303 | 30 | python decl doc at src/pluggy/_hooks.py:593 |  |  | 0.465 |
| walker |  | 5415 | 112 | python class body at src/pluggy/_hooks.py:638 |  |  | 0.465 |
| ns | 5479 |  | 404 | Eggsample lib.py + eggsample-spam (host impls + plugin impls) | 6.3 |  | 0.446 |
| walker |  | 5487 | 72 | python decl doc at src/pluggy/_hooks.py:293 |  |  | 0.446 |
| walker |  | 5656 | 169 | python decl doc at src/pluggy/_manager.py:83 |  |  | 0.465 |
| walker |  | 5827 | 171 | python decl doc at src/pluggy/_warnings.py:10 |  |  | 0.490 |
| ns | 6020 |  | 541 | docs/index.rst — "Call time order" section (tryfirst / trylast) | 6.4 | 5.7 | 0.464 |
| walker |  | 6053 | 226 | python class body at src/pluggy/_hooks.py:56 |  |  | 0.497 |
| walker |  | 6128 | 75 | python decl body at src/pluggy/_hooks.py:281 body 282 |  |  | 0.497 |
| walker |  | 6215 | 87 | python method doc at src/pluggy/_result.py:67 |  |  | 0.506 |
| walker |  | 6376 | 161 | python imports in src/pluggy/_callers.py |  |  | 0.506 |
| walker |  | 6473 | 97 | json config .claude/settings.json |  |  | 0.506 |
| ns | 6517 |  | 497 | docs/index.rst — Wrappers (new-style) wrapper-protocol summary | 6.5 | 5.7 | 0.490 |
| walker |  | 6522 | 49 | python method body at src/pluggy/_tracing.py:17 body 18 |  |  | 0.490 |
| walker |  | 6540 | 18 | python decl names surface in docs/examples/eggsample/eggsample/__init__.py |  |  | 0.490 |
| walker |  | 6551 | 11 | python method body at src/pluggy/_result.py:42 body 45 |  |  | 0.490 |
| ns | 6579 |  | 62 | testing/ FS listing | 7.1 |  | 0.498 |
| ns | 6827 |  | 248 | testing/conftest.py — pm + he_pm fixtures | 7.2 |  | 0.487 |
| walker |  | 6935 | 384 | python method sigs #1 in src/pluggy/_hooks.py |  |  | 0.525 |
| walker |  | 6935 | 0 | python method at src/pluggy/_hooks.py:365 |  |  | 0.525 |
| walker |  | 6935 | 0 | python method at src/pluggy/_hooks.py:420 |  |  | 0.525 |
| walker |  | 6935 | 0 | python method at src/pluggy/_hooks.py:438 |  |  | 0.525 |
| walker |  | 6935 | 0 | python method at src/pluggy/_hooks.py:442 |  |  | 0.525 |
| walker |  | 6935 | 0 | python method at src/pluggy/_hooks.py:449 |  |  | 0.525 |
| walker |  | 6935 | 0 | python method at src/pluggy/_hooks.py:453 |  |  | 0.525 |
| walker |  | 6935 | 0 | python method at src/pluggy/_hooks.py:477 |  |  | 0.525 |
| walker |  | 6935 | 0 | python method at src/pluggy/_hooks.py:499 |  |  | 0.525 |
| walker |  | 6935 | 0 | python method at src/pluggy/_hooks.py:577 |  |  | 0.525 |
| walker |  | 6935 | 0 | python method at src/pluggy/_hooks.py:612 |  |  | 0.525 |
| walker |  | 6935 | 0 | python method at src/pluggy/_hooks.py:634 |  |  | 0.525 |
| walker |  | 6935 | 0 | python method at src/pluggy/_hooks.py:692 |  |  | 0.525 |
| walker |  | 6935 | 0 | python method at src/pluggy/_hooks.py:709 |  |  | 0.525 |
| walker |  | 6956 | 21 | python method at src/pluggy/_hooks.py:626 |  |  | 0.527 |
| walker |  | 6966 | 10 | python method doc at src/pluggy/_hooks.py:365 |  |  | 0.527 |
| walker |  | 6979 | 13 | python method doc at src/pluggy/_hooks.py:449 |  |  | 0.527 |
| walker |  | 7008 | 29 | python method at src/pluggy/_hooks.py:543 |  |  | 0.527 |
| walker |  | 7018 | 10 | python method body at src/pluggy/_hooks.py:420 body 421 |  |  | 0.527 |
| walker |  | 7029 | 11 | python method body at src/pluggy/_hooks.py:626 body 628 |  |  | 0.527 |
| walker |  | 7045 | 16 | python method doc at src/pluggy/_hooks.py:438 |  |  | 0.527 |
| walker |  | 7060 | 15 | python method at src/pluggy/_hooks.py:480 |  |  | 0.527 |
| walker |  | 7073 | 13 | python method body at src/pluggy/_hooks.py:449 body 451 |  |  | 0.527 |
| walker |  | 7112 | 39 | python method at src/pluggy/_hooks.py:424 |  |  | 0.527 |
| walker |  | 7128 | 16 | python method at src/pluggy/_hooks.py:618 |  |  | 0.529 |
| walker |  | 7142 | 14 | python method body at src/pluggy/_hooks.py:438 body 440 |  |  | 0.529 |
| walker |  | 7157 | 15 | python method body at src/pluggy/_hooks.py:477 body 478 |  |  | 0.529 |
| walker |  | 7169 | 12 | python method doc at src/pluggy/_hooks.py:453 |  |  | 0.529 |
| walker |  | 7186 | 17 | python method body at src/pluggy/_hooks.py:634 body 635 |  |  | 0.529 |
| ns | 7193 |  | 366 | test_pluginmanager.py — every test fn name | 7.3 |  | 0.515 |
| walker |  | 7236 | 50 | python method at src/pluggy/_hooks.py:516 |  |  | 0.515 |
| walker |  | 7260 | 24 | python method at src/pluggy/_hooks.py:630 |  |  | 0.517 |
| walker |  | 7321 | 61 | python method at src/pluggy/_hooks.py:656 |  |  | 0.517 |
| walker |  | 7333 | 12 | python method doc at src/pluggy/_hooks.py:656 |  |  | 0.517 |
| walker |  | 7401 | 68 | python method at src/pluggy/_hooks.py:393 |  |  | 0.523 |
| walker |  | 7413 | 12 | python method doc at src/pluggy/_hooks.py:393 |  |  | 0.524 |
| walker |  | 7432 | 19 | python method doc at src/pluggy/_hooks.py:577 |  |  | 0.524 |
| walker |  | 7477 | 45 | python method doc at src/pluggy/_hooks.py:543 |  |  | 0.524 |
| walker |  | 7502 | 25 | python method body at src/pluggy/_hooks.py:692 body 693 |  |  | 0.524 |
| walker |  | 7515 | 13 | python method body at src/pluggy/_hooks.py:630 body 632 |  |  | 0.524 |
| walker |  | 7580 | 65 | python method doc at src/pluggy/_hooks.py:499 |  |  | 0.524 |
| ns | 7623 |  | 430 | test_hookcaller.py + test_multicall.py — every test fn name | 7.4 |  | 0.509 |
| walker |  | 7673 | 93 | python method doc at src/pluggy/_hooks.py:516 |  |  | 0.509 |
| walker |  | 7728 | 55 | python method body at src/pluggy/_tracing.py:51 body 52 |  |  | 0.509 |
| walker |  | 7936 | 208 | python imports in src/pluggy/_hooks.py |  |  | 0.509 |
| walker |  | 7949 | 13 | listing of '.github' |  |  | 0.509 |
| walker |  | 7953 | 4 | listing of '.github/workflows' |  |  | 0.509 |
| walker |  | 8015 | 62 | python method body at src/pluggy/_hooks.py:612 body 613 |  |  | 0.509 |
| ns | 8023 |  | 400 | Smaller test files — every test fn name | 7.5 |  | 0.495 |
| walker |  | 8029 | 14 | listing of 'scripts' |  |  | 0.495 |
| walker |  | 8041 | 12 | python decl names surface in scripts/towncrier-draft-to-file.py |  |  | 0.495 |
| walker |  | 8041 | 0 | python decl at scripts/towncrier-draft-to-file.py:5 |  |  | 0.495 |
| ns | 8324 |  | 301 | pyproject.toml — [project] essentials (skip classifier list) | 7.6 |  | 0.486 |
| ns | 8576 |  | 252 | pyproject.toml — [tool.ruff.lint] config | 7.7 | 7.6 | 0.479 |
| walker |  | 8655 | 614 | python method sigs in src/pluggy/_manager.py |  |  | 0.519 |
| walker |  | 8655 | 0 | python method at src/pluggy/_manager.py:59 |  |  | 0.519 |
| walker |  | 8655 | 0 | python method at src/pluggy/_manager.py:68 |  |  | 0.519 |
| walker |  | 8655 | 0 | python method at src/pluggy/_manager.py:76 |  |  | 0.519 |
| walker |  | 8655 | 0 | python method at src/pluggy/_manager.py:79 |  |  | 0.519 |
| walker |  | 8655 | 0 | python method at src/pluggy/_manager.py:125 |  |  | 0.519 |
| walker |  | 8655 | 0 | python method at src/pluggy/_manager.py:176 |  |  | 0.519 |
| walker |  | 8655 | 0 | python method at src/pluggy/_manager.py:233 |  |  | 0.519 |
| walker |  | 8655 | 0 | python method at src/pluggy/_manager.py:238 |  |  | 0.519 |
| walker |  | 8655 | 0 | python method at src/pluggy/_manager.py:242 |  |  | 0.519 |
| walker |  | 8655 | 0 | python method at src/pluggy/_manager.py:252 |  |  | 0.519 |
| walker |  | 8655 | 0 | python method at src/pluggy/_manager.py:296 |  |  | 0.519 |
| walker |  | 8655 | 0 | python method at src/pluggy/_manager.py:300 |  |  | 0.519 |
| walker |  | 8655 | 0 | python method at src/pluggy/_manager.py:304 |  |  | 0.519 |
| walker |  | 8655 | 0 | python method at src/pluggy/_manager.py:315 |  |  | 0.519 |
| walker |  | 8655 | 0 | python method at src/pluggy/_manager.py:319 |  |  | 0.519 |
| walker |  | 8655 | 0 | python method at src/pluggy/_manager.py:323 |  |  | 0.519 |
| walker |  | 8655 | 0 | python method at src/pluggy/_manager.py:331 |  |  | 0.519 |
| walker |  | 8655 | 0 | python method at src/pluggy/_manager.py:379 |  |  | 0.519 |
| walker |  | 8655 | 0 | python method at src/pluggy/_manager.py:395 |  |  | 0.519 |
| walker |  | 8655 | 0 | python method at src/pluggy/_manager.py:425 |  |  | 0.519 |
| walker |  | 8655 | 0 | python method at src/pluggy/_manager.py:430 |  |  | 0.519 |
| walker |  | 8655 | 0 | python method at src/pluggy/_manager.py:434 |  |  | 0.519 |
| walker |  | 8655 | 0 | python method at src/pluggy/_manager.py:487 |  |  | 0.519 |
| walker |  | 8663 | 8 | python method at src/pluggy/_manager.py:100 |  |  | 0.519 |
| walker |  | 8674 | 11 | python method at src/pluggy/_manager.py:71 |  |  | 0.521 |
| walker |  | 8686 | 12 | python method doc at src/pluggy/_manager.py:300 |  |  | 0.521 |
| walker |  | 8712 | 26 | python method at src/pluggy/_manager.py:512 |  |  | 0.521 |
| walker |  | 8721 | 9 | python method body at src/pluggy/_manager.py:68 body 69 |  |  | 0.521 |
| walker |  | 8734 | 13 | python method doc at src/pluggy/_manager.py:238 |  |  | 0.521 |
| walker |  | 8747 | 13 | python method doc at src/pluggy/_manager.py:296 |  |  | 0.521 |
| walker |  | 8776 | 29 | python method at src/pluggy/_manager.py:278 |  |  | 0.521 |
| walker |  | 8805 | 29 | python method at src/pluggy/_manager.py:450 |  |  | 0.521 |
| walker |  | 8820 | 15 | python method doc at src/pluggy/_manager.py:319 |  |  | 0.521 |
| walker |  | 8836 | 16 | python method doc at src/pluggy/_manager.py:233 |  |  | 0.521 |
| walker |  | 8852 | 16 | python method doc at src/pluggy/_manager.py:315 |  |  | 0.521 |
| ns | 8859 |  | 283 | tox.ini — [tox] + [testenv] + [pytest] (skip release/docs envs) | 7.8 |  | 0.512 |
| walker |  | 8885 | 33 | python method at src/pluggy/_manager.py:201 |  |  | 0.512 |
| walker |  | 8904 | 19 | python method doc at src/pluggy/_manager.py:430 |  |  | 0.512 |
| walker |  | 8917 | 13 | python method body at src/pluggy/_manager.py:76 body 77 |  |  | 0.512 |
| walker |  | 8930 | 13 | python method body at src/pluggy/_manager.py:425 body 428 |  |  | 0.512 |
| walker |  | 8944 | 14 | python method body at src/pluggy/_manager.py:315 body 317 |  |  | 0.512 |
| walker |  | 8958 | 14 | python method body at src/pluggy/_manager.py:430 body 432 |  |  | 0.512 |
| walker |  | 8973 | 15 | python method body at src/pluggy/_manager.py:319 body 321 |  |  | 0.512 |
| walker |  | 9000 | 27 | python method doc at src/pluggy/_manager.py:323 |  |  | 0.512 |
| walker |  | 9028 | 28 | python method doc at src/pluggy/_manager.py:242 |  |  | 0.512 |
| walker |  | 9056 | 28 | python method doc at src/pluggy/_manager.py:425 |  |  | 0.512 |
| walker |  | 9075 | 19 | python method body at src/pluggy/_manager.py:79 body 80 |  |  | 0.512 |
| walker |  | 9095 | 20 | python method body at src/pluggy/_manager.py:300 body 302 |  |  | 0.512 |
| walker |  | 9130 | 35 | python method doc at src/pluggy/_manager.py:487 |  |  | 0.512 |
| walker |  | 9152 | 22 | python method body at src/pluggy/_manager.py:71 body 73 |  |  | 0.512 |
| walker |  | 9194 | 42 | python method doc at src/pluggy/_manager.py:379 |  |  | 0.512 |
| walker |  | 9218 | 24 | python method body at src/pluggy/_manager.py:238 body 240 |  |  | 0.512 |
| ns | 9238 |  | 379 | CHANGELOG.rst — pluggy 1.6.0 entry only | 7.9 |  | 0.502 |
| walker |  | 9242 | 24 | python method body at src/pluggy/_manager.py:296 body 298 |  |  | 0.502 |
| walker |  | 9292 | 50 | python method doc at src/pluggy/_manager.py:512 |  |  | 0.502 |
| ns | 9300 |  | 62 | changelog/ + downstream/ FS listings | 7.10 |  | 0.508 |
| walker |  | 9348 | 56 | python method doc at src/pluggy/_manager.py:252 |  |  | 0.508 |
| walker |  | 9406 | 58 | python method doc at src/pluggy/_manager.py:434 |  |  | 0.508 |
| walker |  | 9437 | 31 | python method body at src/pluggy/_manager.py:59 body 60 |  |  | 0.508 |
| walker |  | 9445 | 8 | python method body at src/pluggy/_manager.py:242 body 250 |  |  | 0.508 |
| walker |  | 9453 | 8 | python method body at src/pluggy/_manager.py:323 body 329 |  |  | 0.508 |
| walker |  | 9530 | 77 | python method doc at src/pluggy/_manager.py:201 |  |  | 0.508 |
| walker |  | 9620 | 90 | python method doc at src/pluggy/_manager.py:304 |  |  | 0.508 |
| walker |  | 9711 | 91 | python method doc at src/pluggy/_manager.py:395 |  |  | 0.508 |
| walker |  | 9721 | 10 | python method body at src/pluggy/_manager.py:233 body 235 |  |  | 0.508 |
| ns | 9819 |  | 519 | _callers._multicall body — the actual call loop | 7.11 | 4.3 | 0.495 |
| walker |  | 9841 | 120 | python method doc at src/pluggy/_manager.py:125 |  |  | 0.495 |
| ns | 9946 |  | 127 | Top-level meta — AGENTS.md, SECURITY.md, MANIFEST.in | 7.12 |  | 0.491 |
| walker |  | 9964 | 123 | python method doc at src/pluggy/_manager.py:176 |  |  | 0.492 |
| ns | 9964 |  | 18 | scripts/ FS + .github/workflows/ FS | 7.13 |  | 0.492 |
| walker |  | 9976 | 12 | python method body at src/pluggy/_manager.py:304 body 313 |  |  | 0.492 |
| ns | 9996 |  | 32 | docs/ root FS + docs/requirements.txt | 7.14 |  | 0.496 |

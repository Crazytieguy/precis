Score(3000)=0.605 I=0.804 C=0.455 ns_rows≤3K=15/40 (reached=7 partial=3 missing=5)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 96 | 96 | listing of '.' |  |  | 1.000 |
| ns | 96 |  | 96 | Top-level repo listing | 1.1 |  | 1.000 |
| walker |  | 100 | 4 | listing of 'src' |  |  | 1.000 |
| walker |  | 202 | 102 | README headline in README.rst |  |  | 1.000 |
| ns | 213 |  | 117 | README lede + tagline | 1.2 |  | 0.943 |
| walker |  | 221 | 19 | listing of 'changelog' |  |  | 0.943 |
| ns | 256 |  | 43 | Source package layout (src/pluggy/) | 1.3 |  | 0.827 |
| walker |  | 264 | 43 | listing of 'src/pluggy' |  |  | 0.952 |
| walker |  | 282 | 18 | python decl names surface in src/pluggy/__init__.py |  |  | 0.952 |
| walker |  | 282 | 0 | python decl at src/pluggy/__init__.py:32 |  |  | 0.952 |
| walker |  | 314 | 32 | listing of 'docs' |  |  | 0.953 |
| walker |  | 357 | 43 | listing of 'downstream' |  |  | 0.954 |
| walker |  | 360 | 3 | listing of 'docs/_static' |  |  | 0.954 |
| ns | 406 |  | 150 | __all__ — full public-name list | 1.4 |  | 0.784 |
| ns | 624 |  | 218 | Toy example — first half (spec + plugin classes) | 2.1 |  | 0.641 |
| walker |  | 676 | 316 | python imports in src/pluggy/__init__.py |  |  | 0.800 |
| walker |  | 685 | 9 | python imports in src/pluggy/_warnings.py |  |  | 0.800 |
| walker |  | 707 | 22 | python decl names surface in src/pluggy/_warnings.py |  |  | 0.800 |
| walker |  | 707 | 0 | python decl at src/pluggy/_warnings.py:4 |  |  | 0.800 |
| walker |  | 721 | 14 | python decl at src/pluggy/_warnings.py:10 |  |  | 0.800 |
| walker |  | 734 | 13 | python class body at src/pluggy/_warnings.py:4 |  |  | 0.800 |
| walker |  | 747 | 13 | python class body at src/pluggy/_warnings.py:10 |  |  | 0.800 |
| walker |  | 761 | 14 | python decl doc at src/pluggy/_warnings.py:4 |  |  | 0.800 |
| walker |  | 817 | 56 | python decl body at src/pluggy/__init__.py:32 body 33 |  |  | 0.800 |
| walker |  | 821 | 4 | listing of 'docs/_static/img' |  |  | 0.800 |
| ns | 829 |  | 205 | Toy example — second half (PluginManager wiring + call) | 2.2 | 2.1 | 0.705 |
| walker |  | 861 | 40 | python imports in docs/conf.py |  |  | 0.705 |
| walker |  | 923 | 62 | listing of 'testing' |  |  | 0.707 |
| walker |  | 978 | 55 | python decl names surface in src/pluggy/_tracing.py |  |  | 0.707 |
| walker |  | 978 | 0 | python decl at src/pluggy/_tracing.py:16 |  |  | 0.707 |
| walker |  | 978 | 0 | python decl at src/pluggy/_tracing.py:59 |  |  | 0.707 |
| ns | 995 |  | 166 | __init__.py re-exports — name → owning module | 3.1 | 1.4 | 0.731 |
| walker |  | 1046 | 68 | python decl names surface in src/pluggy/_callers.py |  |  | 0.732 |
| walker |  | 1075 | 29 | python decl at src/pluggy/_callers.py:27 |  |  | 0.732 |
| walker |  | 1106 | 31 | python decl doc at src/pluggy/_callers.py:27 |  |  | 0.732 |
| walker |  | 1133 | 27 | python decl at src/pluggy/_callers.py:70 |  |  | 0.732 |
| walker |  | 1165 | 32 | python decl at src/pluggy/_callers.py:60 |  |  | 0.732 |
| walker |  | 1234 | 69 | python decl names surface in src/pluggy/_result.py |  |  | 0.732 |
| walker |  | 1234 | 0 | python decl at src/pluggy/_result.py:20 |  |  | 0.732 |
| walker |  | 1243 | 9 | python decl at src/pluggy/_result.py:24 |  |  | 0.732 |
| walker |  | 1252 | 9 | python decl doc at src/pluggy/_result.py:20 |  |  | 0.732 |
| ns | 1261 |  | 266 | docs/index.rst lede — "what is pluggy" | 3.2 |  | 0.667 |
| walker |  | 1272 | 20 | python class body at src/pluggy/_result.py:24 |  |  | 0.667 |
| walker |  | 1306 | 34 | python decl doc at src/pluggy/_result.py:24 |  |  | 0.667 |
| walker |  | 1403 | 97 | python method sigs in src/pluggy/_result.py |  |  | 0.669 |
| walker |  | 1403 | 0 | python method at src/pluggy/_result.py:67 |  |  | 0.669 |
| walker |  | 1403 | 0 | python method at src/pluggy/_result.py:80 |  |  | 0.669 |
| walker |  | 1403 | 0 | python method at src/pluggy/_result.py:91 |  |  | 0.669 |
| walker |  | 1416 | 13 | python method at src/pluggy/_result.py:51 |  |  | 0.669 |
| walker |  | 1431 | 15 | python method at src/pluggy/_result.py:42 |  |  | 0.670 |
| walker |  | 1453 | 22 | python method at src/pluggy/_result.py:56 |  |  | 0.671 |
| walker |  | 1465 | 12 | python method doc at src/pluggy/_result.py:42 |  |  | 0.671 |
| walker |  | 1477 | 12 | python method doc at src/pluggy/_result.py:51 |  |  | 0.671 |
| walker |  | 1489 | 12 | python method doc at src/pluggy/_result.py:56 |  |  | 0.671 |
| walker |  | 1499 | 10 | python method body at src/pluggy/_result.py:51 body 54 |  |  | 0.671 |
| walker |  | 1537 | 38 | python method at src/pluggy/_result.py:31 |  |  | 0.671 |
| walker |  | 1549 | 12 | python method doc at src/pluggy/_result.py:31 |  |  | 0.671 |
| ns | 1595 |  | 334 | _manager.py — every class + def name | 4.1 |  | 0.586 |
| walker |  | 1684 | 135 | python decl names surface in docs/conf.py |  |  | 0.586 |
| walker |  | 1752 | 68 | python decl at docs/conf.py:9 |  |  | 0.586 |
| walker |  | 1756 | 4 | listing of '.claude' |  |  | 0.586 |
| walker |  | 1771 | 15 | listing of 'docs/examples' |  |  | 0.586 |
| walker |  | 1820 | 49 | python method doc at src/pluggy/_result.py:80 |  |  | 0.587 |
| walker |  | 1876 | 56 | python method doc at src/pluggy/_result.py:91 |  |  | 0.587 |
| walker |  | 1933 | 57 | python decl at src/pluggy/_callers.py:82 |  |  | 0.589 |
| ns | 2037 |  | 442 | _hooks.py — every class + def name (markers, HookCaller, HookImpl, HookSpec) | 4.2 |  | 0.516 |
| ns | 2138 |  | 101 | _callers.py — every def + _multicall signature | 4.3 |  | 0.536 |
| walker |  | 2143 | 210 | python method sigs in src/pluggy/_tracing.py |  |  | 0.538 |
| walker |  | 2143 | 0 | python method at src/pluggy/_tracing.py:17 |  |  | 0.538 |
| walker |  | 2143 | 0 | python method at src/pluggy/_tracing.py:22 |  |  | 0.538 |
| walker |  | 2143 | 0 | python method at src/pluggy/_tracing.py:25 |  |  | 0.538 |
| walker |  | 2143 | 0 | python method at src/pluggy/_tracing.py:42 |  |  | 0.538 |
| walker |  | 2143 | 0 | python method at src/pluggy/_tracing.py:48 |  |  | 0.538 |
| walker |  | 2143 | 0 | python method at src/pluggy/_tracing.py:51 |  |  | 0.538 |
| walker |  | 2143 | 0 | python method at src/pluggy/_tracing.py:60 |  |  | 0.538 |
| walker |  | 2143 | 0 | python method at src/pluggy/_tracing.py:64 |  |  | 0.538 |
| walker |  | 2143 | 0 | python method at src/pluggy/_tracing.py:67 |  |  | 0.538 |
| walker |  | 2152 | 9 | python method body at src/pluggy/_tracing.py:48 body 49 |  |  | 0.538 |
| walker |  | 2164 | 12 | python method body at src/pluggy/_tracing.py:22 body 23 |  |  | 0.538 |
| walker |  | 2177 | 13 | python method body at src/pluggy/_tracing.py:64 body 65 |  |  | 0.538 |
| walker |  | 2194 | 17 | python method body at src/pluggy/_tracing.py:67 body 68 |  |  | 0.538 |
| walker |  | 2212 | 18 | python method body at src/pluggy/_tracing.py:60 body 61 |  |  | 0.538 |
| walker |  | 2263 | 51 | python decl doc at src/pluggy/_callers.py:82 |  |  | 0.538 |
| ns | 2275 |  | 137 | _result.py — Result class + every method (in full) | 4.4 |  | 0.561 |
| walker |  | 2379 | 116 | python decl names surface in src/pluggy/_manager.py |  |  | 0.563 |
| walker |  | 2379 | 0 | python decl at src/pluggy/_manager.py:42 |  |  | 0.563 |
| walker |  | 2379 | 0 | python decl at src/pluggy/_manager.py:52 |  |  | 0.563 |
| walker |  | 2379 | 0 | python decl at src/pluggy/_manager.py:65 |  |  | 0.563 |
| walker |  | 2379 | 0 | python decl at src/pluggy/_manager.py:83 |  |  | 0.563 |
| walker |  | 2379 | 0 | python decl at src/pluggy/_manager.py:525 |  |  | 0.563 |
| walker |  | 2390 | 11 | python decl doc at src/pluggy/_manager.py:65 |  |  | 0.563 |
| walker |  | 2431 | 41 | python decl doc at src/pluggy/_manager.py:52 |  |  | 0.563 |
| walker |  | 2447 | 16 | python decl body at src/pluggy/_manager.py:525 body 526 |  |  | 0.563 |
| walker |  | 2475 | 28 | python decl at src/pluggy/_manager.py:37 |  |  | 0.563 |
| walker |  | 2534 | 59 | python imports in src/pluggy/_tracing.py |  |  | 0.563 |
| ns | 2581 |  | 306 | _warnings.py — full file (Pluggy*Warning classes) | 4.5 |  | 0.534 |
| ns | 2682 |  | 101 | _tracing.py — class + def names | 4.6 |  | 0.551 |
| walker |  | 2703 | 169 | python decl doc at src/pluggy/_manager.py:83 |  |  | 0.553 |
| ns | 2873 |  | 191 | PluginManager class docstring | 5.1 | 4.1 | 0.564 |
| walker |  | 2874 | 171 | python decl doc at src/pluggy/_warnings.py:10 |  |  | 0.603 |
| walker |  | 2882 | 8 | listing of 'docs/examples/eggsample' |  |  | 0.604 |
| walker |  | 2902 | 20 | listing of 'docs/examples/eggsample/eggsample' |  |  | 0.604 |
| walker |  | 2910 | 8 | python imports in docs/examples/eggsample/eggsample/__init__.py |  |  | 0.604 |
| walker |  | 2997 | 87 | python method doc at src/pluggy/_result.py:67 |  |  | 0.605 |
| ns | 3038 |  | 165 | HookspecMarker + HookimplMarker class docstrings | 5.2 | 4.2 | 0.591 |
| walker |  | 3146 | 149 | python decl names surface #1 in docs/conf.py |  |  | 0.591 |
| walker |  | 3146 | 0 | python decl at docs/conf.py:106 |  |  | 0.591 |
| walker |  | 3146 | 0 | python decl at docs/conf.py:130 |  |  | 0.591 |
| walker |  | 3153 | 7 | python decl body at docs/conf.py:130 body 131 |  |  | 0.591 |
| walker |  | 3171 | 18 | python decl doc at docs/conf.py:106 |  |  | 0.591 |
| walker |  | 3201 | 30 | python decl at docs/conf.py:54 |  |  | 0.591 |
| walker |  | 3282 | 81 | python decl at docs/conf.py:83 |  |  | 0.591 |
| ns | 3369 |  | 331 | HookCaller __slots__ + the 6-bucket call-order comment | 5.3 | 4.2 | 0.560 |
| walker |  | 3418 | 136 | python decl at docs/conf.py:96 |  |  | 0.560 |
| walker |  | 3564 | 146 | python decl at docs/conf.py:41 |  |  | 0.560 |
| ns | 3651 |  | 282 | HookCaller._add_hookimpl body — the actual ordering algorithm | 5.4 | 4.2 | 0.539 |
| walker |  | 3722 | 158 | python decl at docs/conf.py:66 |  |  | 0.539 |
| walker |  | 3771 | 49 | python method body at src/pluggy/_tracing.py:17 body 18 |  |  | 0.539 |
| walker |  | 3789 | 18 | python decl names surface in docs/examples/eggsample/eggsample/__init__.py |  |  | 0.539 |
| walker |  | 3800 | 11 | python method body at src/pluggy/_result.py:42 body 45 |  |  | 0.539 |
| walker |  | 3901 | 101 | python imports in src/pluggy/_result.py |  |  | 0.539 |
| walker |  | 3956 | 55 | python method body at src/pluggy/_tracing.py:51 body 52 |  |  | 0.539 |
| walker |  | 3969 | 13 | listing of '.github' |  |  | 0.539 |
| walker |  | 3973 | 4 | listing of '.github/workflows' |  |  | 0.539 |
| walker |  | 3987 | 14 | listing of 'scripts' |  |  | 0.539 |
| ns | 4080 |  | 429 | HookspecOpts + HookimplOpts — TypedDict bodies | 5.5 | 4.2 | 0.512 |
| walker |  | 4168 | 181 | python decl names surface in src/pluggy/_hooks.py |  |  | 0.513 |
| walker |  | 4168 | 0 | python decl at src/pluggy/_hooks.py:40 |  |  | 0.513 |
| walker |  | 4168 | 0 | python decl at src/pluggy/_hooks.py:56 |  |  | 0.513 |
| walker |  | 4168 | 0 | python decl at src/pluggy/_hooks.py:281 |  |  | 0.513 |
| walker |  | 4175 | 7 | python decl at src/pluggy/_hooks.py:77 |  |  | 0.513 |
| walker |  | 4182 | 7 | python decl at src/pluggy/_hooks.py:164 |  |  | 0.514 |
| walker |  | 4192 | 10 | python decl doc at src/pluggy/_hooks.py:40 |  |  | 0.514 |
| walker |  | 4202 | 10 | python decl doc at src/pluggy/_hooks.py:56 |  |  | 0.514 |
| walker |  | 4216 | 14 | python class body at src/pluggy/_hooks.py:77 |  |  | 0.514 |
| walker |  | 4230 | 14 | python class body at src/pluggy/_hooks.py:164 |  |  | 0.514 |
| walker |  | 4307 | 77 | python decl doc at src/pluggy/_hooks.py:164 |  |  | 0.519 |
| walker |  | 4385 | 78 | python decl doc at src/pluggy/_hooks.py:77 |  |  | 0.535 |
| ns | 4494 |  | 414 | Result API — force_result, force_exception, get_result bodies | 5.6 | 4.4 | 0.526 |
| walker |  | 4519 | 134 | python method sigs in src/pluggy/_hooks.py |  |  | 0.530 |
| walker |  | 4519 | 0 | python method at src/pluggy/_hooks.py:88 |  |  | 0.530 |
| walker |  | 4519 | 0 | python method at src/pluggy/_hooks.py:175 |  |  | 0.530 |
| walker |  | 4531 | 12 | python method body at src/pluggy/_hooks.py:88 body 89 |  |  | 0.530 |
| walker |  | 4543 | 12 | python method body at src/pluggy/_hooks.py:175 body 176 |  |  | 0.530 |
| walker |  | 4579 | 36 | python decl at src/pluggy/_hooks.py:33 |  |  | 0.530 |
| walker |  | 4661 | 82 | python method at src/pluggy/_hooks.py:91 |  |  | 0.531 |
| walker |  | 4672 | 11 | python method body at src/pluggy/_hooks.py:91 body 99 |  |  | 0.531 |
| walker |  | 4762 | 90 | python method at src/pluggy/_hooks.py:101 |  |  | 0.532 |
| walker |  | 4778 | 16 | python method body at src/pluggy/_hooks.py:101 body 109 |  |  | 0.532 |
| ns | 4801 |  | 307 | docs/index.rst H2 heading map — sections at exact line numbers | 5.7 | 3.2 | 0.504 |
| ns | 4855 |  | 54 | docs/examples/ FS listing | 6.1 |  | 0.509 |
| walker |  | 4942 | 164 | python class body at src/pluggy/_hooks.py:40 |  |  | 0.519 |
| walker |  | 5037 | 95 | python method at src/pluggy/_hooks.py:111 |  |  | 0.519 |
| ns | 5075 |  | 220 | Eggsample hookspecs.py — host-side hook specifications | 6.2 |  | 0.506 |
| walker |  | 5134 | 97 | python method at src/pluggy/_hooks.py:178 |  |  | 0.507 |
| walker |  | 5145 | 11 | python method body at src/pluggy/_hooks.py:178 body 188 |  |  | 0.507 |
| walker |  | 5250 | 105 | python method at src/pluggy/_hooks.py:190 |  |  | 0.508 |
| walker |  | 5266 | 16 | python method body at src/pluggy/_hooks.py:190 body 200 |  |  | 0.508 |
| walker |  | 5376 | 110 | python method at src/pluggy/_hooks.py:202 |  |  | 0.508 |
| ns | 5479 |  | 404 | Eggsample lib.py + eggsample-spam (host impls + plugin impls) | 6.3 |  | 0.488 |
| walker |  | 5602 | 226 | python class body at src/pluggy/_hooks.py:56 |  |  | 0.523 |
| walker |  | 5677 | 75 | python decl body at src/pluggy/_hooks.py:281 body 282 |  |  | 0.523 |
| walker |  | 5790 | 113 | python decl names surface #1 in src/pluggy/_hooks.py |  |  | 0.527 |
| walker |  | 5790 | 0 | python decl at src/pluggy/_hooks.py:293 |  |  | 0.527 |
| walker |  | 5790 | 0 | python decl at src/pluggy/_hooks.py:382 |  |  | 0.527 |
| walker |  | 5790 | 0 | python decl at src/pluggy/_hooks.py:593 |  |  | 0.527 |
| walker |  | 5796 | 6 | python decl at src/pluggy/_hooks.py:358 |  |  | 0.529 |
| walker |  | 5802 | 6 | python decl at src/pluggy/_hooks.py:638 |  |  | 0.530 |
| walker |  | 5808 | 6 | python decl at src/pluggy/_hooks.py:696 |  |  | 0.531 |
| walker |  | 5823 | 15 | python decl doc at src/pluggy/_hooks.py:382 |  |  | 0.531 |
| walker |  | 5841 | 18 | python decl doc at src/pluggy/_hooks.py:638 |  |  | 0.531 |
| walker |  | 5872 | 31 | python decl doc at src/pluggy/_hooks.py:358 |  |  | 0.531 |
| walker |  | 5917 | 45 | python class body at src/pluggy/_hooks.py:358 |  |  | 0.533 |
| walker |  | 5943 | 26 | python decl at src/pluggy/_hooks.py:377 |  |  | 0.533 |
| ns | 6020 |  | 541 | docs/index.rst — "Call time order" section (tryfirst / trylast) | 6.4 | 5.7 | 0.504 |
| walker |  | 6031 | 88 | python class body at src/pluggy/_hooks.py:382 |  |  | 0.507 |
| walker |  | 6119 | 88 | python class body at src/pluggy/_hooks.py:696 |  |  | 0.507 |
| walker |  | 6149 | 30 | python decl doc at src/pluggy/_hooks.py:593 |  |  | 0.507 |
| walker |  | 6261 | 112 | python class body at src/pluggy/_hooks.py:638 |  |  | 0.507 |
| walker |  | 6333 | 72 | python decl doc at src/pluggy/_hooks.py:293 |  |  | 0.507 |
| ns | 6517 |  | 497 | docs/index.rst — Wrappers (new-style) wrapper-protocol summary | 6.5 | 5.7 | 0.490 |
| ns | 6579 |  | 62 | testing/ FS listing | 7.1 |  | 0.498 |
| walker |  | 6717 | 384 | python method sigs #1 in src/pluggy/_hooks.py |  |  | 0.537 |
| walker |  | 6717 | 0 | python method at src/pluggy/_hooks.py:365 |  |  | 0.537 |
| walker |  | 6717 | 0 | python method at src/pluggy/_hooks.py:420 |  |  | 0.537 |
| walker |  | 6717 | 0 | python method at src/pluggy/_hooks.py:438 |  |  | 0.537 |
| walker |  | 6717 | 0 | python method at src/pluggy/_hooks.py:442 |  |  | 0.537 |
| walker |  | 6717 | 0 | python method at src/pluggy/_hooks.py:449 |  |  | 0.537 |
| walker |  | 6717 | 0 | python method at src/pluggy/_hooks.py:453 |  |  | 0.537 |
| walker |  | 6717 | 0 | python method at src/pluggy/_hooks.py:477 |  |  | 0.537 |
| walker |  | 6717 | 0 | python method at src/pluggy/_hooks.py:499 |  |  | 0.537 |
| walker |  | 6717 | 0 | python method at src/pluggy/_hooks.py:577 |  |  | 0.537 |
| walker |  | 6717 | 0 | python method at src/pluggy/_hooks.py:612 |  |  | 0.537 |
| walker |  | 6717 | 0 | python method at src/pluggy/_hooks.py:634 |  |  | 0.537 |
| walker |  | 6717 | 0 | python method at src/pluggy/_hooks.py:692 |  |  | 0.537 |
| walker |  | 6717 | 0 | python method at src/pluggy/_hooks.py:709 |  |  | 0.537 |
| walker |  | 6738 | 21 | python method at src/pluggy/_hooks.py:626 |  |  | 0.539 |
| walker |  | 6748 | 10 | python method doc at src/pluggy/_hooks.py:365 |  |  | 0.539 |
| walker |  | 6761 | 13 | python method doc at src/pluggy/_hooks.py:449 |  |  | 0.539 |
| walker |  | 6790 | 29 | python method at src/pluggy/_hooks.py:543 |  |  | 0.539 |
| walker |  | 6800 | 10 | python method body at src/pluggy/_hooks.py:420 body 421 |  |  | 0.539 |
| walker |  | 6811 | 11 | python method body at src/pluggy/_hooks.py:626 body 628 |  |  | 0.539 |
| walker |  | 6827 | 16 | python method doc at src/pluggy/_hooks.py:438 |  |  | 0.527 |
| ns | 6827 |  | 248 | testing/conftest.py — pm + he_pm fixtures | 7.2 |  | 0.527 |
| walker |  | 6842 | 15 | python method at src/pluggy/_hooks.py:480 |  |  | 0.527 |
| walker |  | 6855 | 13 | python method body at src/pluggy/_hooks.py:449 body 451 |  |  | 0.527 |
| walker |  | 6894 | 39 | python method at src/pluggy/_hooks.py:424 |  |  | 0.527 |
| walker |  | 6910 | 16 | python method at src/pluggy/_hooks.py:618 |  |  | 0.529 |
| walker |  | 6924 | 14 | python method body at src/pluggy/_hooks.py:438 body 440 |  |  | 0.529 |
| walker |  | 6939 | 15 | python method body at src/pluggy/_hooks.py:477 body 478 |  |  | 0.529 |
| walker |  | 6951 | 12 | python method doc at src/pluggy/_hooks.py:453 |  |  | 0.529 |
| walker |  | 6968 | 17 | python method body at src/pluggy/_hooks.py:634 body 635 |  |  | 0.529 |
| walker |  | 7018 | 50 | python method at src/pluggy/_hooks.py:516 |  |  | 0.529 |
| walker |  | 7042 | 24 | python method at src/pluggy/_hooks.py:630 |  |  | 0.532 |
| walker |  | 7103 | 61 | python method at src/pluggy/_hooks.py:656 |  |  | 0.532 |
| walker |  | 7115 | 12 | python method doc at src/pluggy/_hooks.py:656 |  |  | 0.532 |
| walker |  | 7183 | 68 | python method at src/pluggy/_hooks.py:393 |  |  | 0.538 |
| ns | 7193 |  | 366 | test_pluginmanager.py — every test fn name | 7.3 |  | 0.523 |
| walker |  | 7195 | 12 | python method doc at src/pluggy/_hooks.py:393 |  |  | 0.524 |
| walker |  | 7214 | 19 | python method doc at src/pluggy/_hooks.py:577 |  |  | 0.524 |
| walker |  | 7259 | 45 | python method doc at src/pluggy/_hooks.py:543 |  |  | 0.524 |
| walker |  | 7284 | 25 | python method body at src/pluggy/_hooks.py:692 body 693 |  |  | 0.524 |
| walker |  | 7297 | 13 | python method body at src/pluggy/_hooks.py:630 body 632 |  |  | 0.524 |
| walker |  | 7362 | 65 | python method doc at src/pluggy/_hooks.py:499 |  |  | 0.524 |
| walker |  | 7455 | 93 | python method doc at src/pluggy/_hooks.py:516 |  |  | 0.524 |
| walker |  | 7517 | 62 | python method body at src/pluggy/_hooks.py:612 body 613 |  |  | 0.524 |
| ns | 7623 |  | 430 | test_hookcaller.py + test_multicall.py — every test fn name | 7.4 |  | 0.509 |
| ns | 8023 |  | 400 | Smaller test files — every test fn name | 7.5 |  | 0.495 |
| walker |  | 8131 | 614 | python method sigs in src/pluggy/_manager.py |  |  | 0.536 |
| walker |  | 8131 | 0 | python method at src/pluggy/_manager.py:59 |  |  | 0.536 |
| walker |  | 8131 | 0 | python method at src/pluggy/_manager.py:68 |  |  | 0.536 |
| walker |  | 8131 | 0 | python method at src/pluggy/_manager.py:76 |  |  | 0.536 |
| walker |  | 8131 | 0 | python method at src/pluggy/_manager.py:79 |  |  | 0.536 |
| walker |  | 8131 | 0 | python method at src/pluggy/_manager.py:125 |  |  | 0.536 |
| walker |  | 8131 | 0 | python method at src/pluggy/_manager.py:176 |  |  | 0.536 |
| walker |  | 8131 | 0 | python method at src/pluggy/_manager.py:233 |  |  | 0.536 |
| walker |  | 8131 | 0 | python method at src/pluggy/_manager.py:238 |  |  | 0.536 |
| walker |  | 8131 | 0 | python method at src/pluggy/_manager.py:242 |  |  | 0.536 |
| walker |  | 8131 | 0 | python method at src/pluggy/_manager.py:252 |  |  | 0.536 |
| walker |  | 8131 | 0 | python method at src/pluggy/_manager.py:296 |  |  | 0.536 |
| walker |  | 8131 | 0 | python method at src/pluggy/_manager.py:300 |  |  | 0.536 |
| walker |  | 8131 | 0 | python method at src/pluggy/_manager.py:304 |  |  | 0.536 |
| walker |  | 8131 | 0 | python method at src/pluggy/_manager.py:315 |  |  | 0.536 |
| walker |  | 8131 | 0 | python method at src/pluggy/_manager.py:319 |  |  | 0.536 |
| walker |  | 8131 | 0 | python method at src/pluggy/_manager.py:323 |  |  | 0.536 |
| walker |  | 8131 | 0 | python method at src/pluggy/_manager.py:331 |  |  | 0.536 |
| walker |  | 8131 | 0 | python method at src/pluggy/_manager.py:379 |  |  | 0.536 |
| walker |  | 8131 | 0 | python method at src/pluggy/_manager.py:395 |  |  | 0.536 |
| walker |  | 8131 | 0 | python method at src/pluggy/_manager.py:425 |  |  | 0.536 |
| walker |  | 8131 | 0 | python method at src/pluggy/_manager.py:430 |  |  | 0.536 |
| walker |  | 8131 | 0 | python method at src/pluggy/_manager.py:434 |  |  | 0.536 |
| walker |  | 8131 | 0 | python method at src/pluggy/_manager.py:487 |  |  | 0.536 |
| walker |  | 8139 | 8 | python method at src/pluggy/_manager.py:100 |  |  | 0.536 |
| walker |  | 8150 | 11 | python method at src/pluggy/_manager.py:71 |  |  | 0.539 |
| walker |  | 8162 | 12 | python method doc at src/pluggy/_manager.py:300 |  |  | 0.539 |
| walker |  | 8188 | 26 | python method at src/pluggy/_manager.py:512 |  |  | 0.539 |
| walker |  | 8197 | 9 | python method body at src/pluggy/_manager.py:68 body 69 |  |  | 0.539 |
| walker |  | 8210 | 13 | python method doc at src/pluggy/_manager.py:238 |  |  | 0.539 |
| walker |  | 8223 | 13 | python method doc at src/pluggy/_manager.py:296 |  |  | 0.539 |
| walker |  | 8252 | 29 | python method at src/pluggy/_manager.py:278 |  |  | 0.539 |
| walker |  | 8281 | 29 | python method at src/pluggy/_manager.py:450 |  |  | 0.539 |
| walker |  | 8296 | 15 | python method doc at src/pluggy/_manager.py:319 |  |  | 0.539 |
| walker |  | 8312 | 16 | python method doc at src/pluggy/_manager.py:233 |  |  | 0.539 |
| ns | 8324 |  | 301 | pyproject.toml — [project] essentials (skip classifier list) | 7.6 |  | 0.528 |
| walker |  | 8328 | 16 | python method doc at src/pluggy/_manager.py:315 |  |  | 0.528 |
| walker |  | 8361 | 33 | python method at src/pluggy/_manager.py:201 |  |  | 0.528 |
| walker |  | 8380 | 19 | python method doc at src/pluggy/_manager.py:430 |  |  | 0.528 |
| walker |  | 8393 | 13 | python method body at src/pluggy/_manager.py:76 body 77 |  |  | 0.528 |
| walker |  | 8406 | 13 | python method body at src/pluggy/_manager.py:425 body 428 |  |  | 0.528 |
| walker |  | 8420 | 14 | python method body at src/pluggy/_manager.py:315 body 317 |  |  | 0.528 |
| walker |  | 8434 | 14 | python method body at src/pluggy/_manager.py:430 body 432 |  |  | 0.528 |
| walker |  | 8449 | 15 | python method body at src/pluggy/_manager.py:319 body 321 |  |  | 0.528 |
| walker |  | 8476 | 27 | python method doc at src/pluggy/_manager.py:323 |  |  | 0.528 |
| walker |  | 8504 | 28 | python method doc at src/pluggy/_manager.py:242 |  |  | 0.528 |
| walker |  | 8532 | 28 | python method doc at src/pluggy/_manager.py:425 |  |  | 0.528 |
| walker |  | 8551 | 19 | python method body at src/pluggy/_manager.py:79 body 80 |  |  | 0.528 |
| walker |  | 8571 | 20 | python method body at src/pluggy/_manager.py:300 body 302 |  |  | 0.528 |
| ns | 8576 |  | 252 | pyproject.toml — [tool.ruff.lint] config | 7.7 | 7.6 | 0.521 |
| walker |  | 8606 | 35 | python method doc at src/pluggy/_manager.py:487 |  |  | 0.521 |
| walker |  | 8628 | 22 | python method body at src/pluggy/_manager.py:71 body 73 |  |  | 0.521 |
| walker |  | 8670 | 42 | python method doc at src/pluggy/_manager.py:379 |  |  | 0.521 |
| walker |  | 8694 | 24 | python method body at src/pluggy/_manager.py:238 body 240 |  |  | 0.521 |
| walker |  | 8718 | 24 | python method body at src/pluggy/_manager.py:296 body 298 |  |  | 0.521 |
| walker |  | 8768 | 50 | python method doc at src/pluggy/_manager.py:512 |  |  | 0.521 |
| walker |  | 8824 | 56 | python method doc at src/pluggy/_manager.py:252 |  |  | 0.521 |
| ns | 8859 |  | 283 | tox.ini — [tox] + [testenv] + [pytest] (skip release/docs envs) | 7.8 |  | 0.512 |
| walker |  | 8882 | 58 | python method doc at src/pluggy/_manager.py:434 |  |  | 0.512 |
| walker |  | 8913 | 31 | python method body at src/pluggy/_manager.py:59 body 60 |  |  | 0.512 |
| walker |  | 8921 | 8 | python method body at src/pluggy/_manager.py:242 body 250 |  |  | 0.512 |
| walker |  | 8929 | 8 | python method body at src/pluggy/_manager.py:323 body 329 |  |  | 0.512 |
| walker |  | 9006 | 77 | python method doc at src/pluggy/_manager.py:201 |  |  | 0.512 |
| walker |  | 9096 | 90 | python method doc at src/pluggy/_manager.py:304 |  |  | 0.512 |
| walker |  | 9187 | 91 | python method doc at src/pluggy/_manager.py:395 |  |  | 0.512 |
| walker |  | 9197 | 10 | python method body at src/pluggy/_manager.py:233 body 235 |  |  | 0.512 |
| ns | 9238 |  | 379 | CHANGELOG.rst — pluggy 1.6.0 entry only | 7.9 |  | 0.502 |
| ns | 9300 |  | 62 | changelog/ + downstream/ FS listings | 7.10 |  | 0.508 |
| walker |  | 9317 | 120 | python method doc at src/pluggy/_manager.py:125 |  |  | 0.508 |
| walker |  | 9440 | 123 | python method doc at src/pluggy/_manager.py:176 |  |  | 0.508 |
| walker |  | 9452 | 12 | python method body at src/pluggy/_manager.py:304 body 313 |  |  | 0.508 |
| walker |  | 9588 | 136 | python method doc at src/pluggy/_manager.py:278 |  |  | 0.508 |
| walker |  | 9752 | 164 | python method doc at src/pluggy/_manager.py:450 |  |  | 0.508 |
| walker |  | 9767 | 15 | python method body at src/pluggy/_manager.py:233 body 236 |  |  | 0.508 |
| ns | 9819 |  | 519 | _callers._multicall body — the actual call loop | 7.11 | 4.3 | 0.495 |
| walker |  | 9833 | 66 | python decl body at src/pluggy/_callers.py:60 body 64 |  |  | 0.495 |
| ns | 9946 |  | 127 | Top-level meta — AGENTS.md, SECURITY.md, MANIFEST.in | 7.12 |  | 0.491 |
| ns | 9964 |  | 18 | scripts/ FS + .github/workflows/ FS | 7.13 |  | 0.492 |
| walker |  | 9994 | 161 | python imports in src/pluggy/_callers.py |  |  | 0.492 |
| ns | 9996 |  | 32 | docs/ root FS + docs/requirements.txt | 7.14 |  | 0.496 |

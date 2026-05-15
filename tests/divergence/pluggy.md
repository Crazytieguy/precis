Score(3000)=0.604 I=0.802 C=0.455 ns_rows≤3K=15/40 (reached=7 partial=3 missing=5)

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
| walker |  | 361 | 43 | listing of 'downstream' |  |  | 0.954 |
| walker |  | 364 | 3 | listing of 'docs/_static' |  |  | 0.954 |
| ns | 406 |  | 150 | __all__ — full public-name list | 1.4 |  | 0.784 |
| ns | 624 |  | 218 | Toy example — first half (spec + plugin classes) | 2.1 |  | 0.641 |
| walker |  | 680 | 316 | python imports in src/pluggy/__init__.py |  |  | 0.800 |
| walker |  | 689 | 9 | python imports in src/pluggy/_warnings.py |  |  | 0.800 |
| walker |  | 711 | 22 | python decl names surface in src/pluggy/_warnings.py |  |  | 0.800 |
| walker |  | 711 | 0 | python decl at src/pluggy/_warnings.py:4 |  |  | 0.800 |
| walker |  | 725 | 14 | python decl at src/pluggy/_warnings.py:10 |  |  | 0.800 |
| walker |  | 738 | 13 | python class body at src/pluggy/_warnings.py:4 |  |  | 0.800 |
| walker |  | 751 | 13 | python class body at src/pluggy/_warnings.py:10 |  |  | 0.800 |
| walker |  | 765 | 14 | python decl doc at src/pluggy/_warnings.py:4 |  |  | 0.800 |
| walker |  | 821 | 56 | python decl body at src/pluggy/__init__.py:32 body 33 |  |  | 0.800 |
| walker |  | 825 | 4 | listing of 'docs/_static/img' |  |  | 0.800 |
| ns | 829 |  | 205 | Toy example — second half (PluginManager wiring + call) | 2.2 | 2.1 | 0.705 |
| walker |  | 865 | 40 | python imports in docs/conf.py |  |  | 0.705 |
| walker |  | 927 | 62 | listing of 'testing' |  |  | 0.707 |
| walker |  | 982 | 55 | python decl names surface in src/pluggy/_tracing.py |  |  | 0.707 |
| walker |  | 982 | 0 | python decl at src/pluggy/_tracing.py:16 |  |  | 0.707 |
| walker |  | 982 | 0 | python decl at src/pluggy/_tracing.py:59 |  |  | 0.707 |
| ns | 995 |  | 166 | __init__.py re-exports — name → owning module | 3.1 | 1.4 | 0.731 |
| walker |  | 1050 | 68 | python decl names surface in src/pluggy/_callers.py |  |  | 0.732 |
| walker |  | 1079 | 29 | python decl at src/pluggy/_callers.py:27 |  |  | 0.732 |
| walker |  | 1110 | 31 | python decl doc at src/pluggy/_callers.py:27 |  |  | 0.732 |
| walker |  | 1137 | 27 | python decl at src/pluggy/_callers.py:70 |  |  | 0.732 |
| walker |  | 1169 | 32 | python decl at src/pluggy/_callers.py:60 |  |  | 0.732 |
| walker |  | 1238 | 69 | python decl names surface in src/pluggy/_result.py |  |  | 0.732 |
| walker |  | 1238 | 0 | python decl at src/pluggy/_result.py:20 |  |  | 0.732 |
| walker |  | 1247 | 9 | python decl at src/pluggy/_result.py:24 |  |  | 0.732 |
| walker |  | 1256 | 9 | python decl doc at src/pluggy/_result.py:20 |  |  | 0.732 |
| ns | 1261 |  | 266 | docs/index.rst lede — "what is pluggy" | 3.2 |  | 0.667 |
| walker |  | 1276 | 20 | python class body at src/pluggy/_result.py:24 |  |  | 0.667 |
| walker |  | 1310 | 34 | python decl doc at src/pluggy/_result.py:24 |  |  | 0.667 |
| walker |  | 1407 | 97 | python method sigs in src/pluggy/_result.py |  |  | 0.669 |
| walker |  | 1407 | 0 | python method at src/pluggy/_result.py:67 |  |  | 0.669 |
| walker |  | 1407 | 0 | python method at src/pluggy/_result.py:80 |  |  | 0.669 |
| walker |  | 1407 | 0 | python method at src/pluggy/_result.py:91 |  |  | 0.669 |
| walker |  | 1420 | 13 | python method at src/pluggy/_result.py:51 |  |  | 0.669 |
| walker |  | 1435 | 15 | python method at src/pluggy/_result.py:42 |  |  | 0.670 |
| walker |  | 1457 | 22 | python method at src/pluggy/_result.py:56 |  |  | 0.671 |
| walker |  | 1469 | 12 | python method doc at src/pluggy/_result.py:42 |  |  | 0.671 |
| walker |  | 1481 | 12 | python method doc at src/pluggy/_result.py:51 |  |  | 0.671 |
| walker |  | 1493 | 12 | python method doc at src/pluggy/_result.py:56 |  |  | 0.671 |
| walker |  | 1503 | 10 | python method body at src/pluggy/_result.py:51 body 54 |  |  | 0.671 |
| walker |  | 1541 | 38 | python method at src/pluggy/_result.py:31 |  |  | 0.671 |
| walker |  | 1553 | 12 | python method doc at src/pluggy/_result.py:31 |  |  | 0.671 |
| ns | 1595 |  | 334 | _manager.py — every class + def name | 4.1 |  | 0.586 |
| walker |  | 1688 | 135 | python decl names surface in docs/conf.py |  |  | 0.586 |
| walker |  | 1756 | 68 | python decl at docs/conf.py:9 |  |  | 0.586 |
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
| walker |  | 2258 | 46 | plaintext config downstream/.gitignore |  |  | 0.538 |
| ns | 2275 |  | 137 | _result.py — Result class + every method (in full) | 4.4 |  | 0.561 |
| walker |  | 2309 | 51 | python decl doc at src/pluggy/_callers.py:82 |  |  | 0.561 |
| walker |  | 2425 | 116 | python decl names surface in src/pluggy/_manager.py |  |  | 0.563 |
| walker |  | 2425 | 0 | python decl at src/pluggy/_manager.py:42 |  |  | 0.563 |
| walker |  | 2425 | 0 | python decl at src/pluggy/_manager.py:52 |  |  | 0.563 |
| walker |  | 2425 | 0 | python decl at src/pluggy/_manager.py:65 |  |  | 0.563 |
| walker |  | 2425 | 0 | python decl at src/pluggy/_manager.py:83 |  |  | 0.563 |
| walker |  | 2425 | 0 | python decl at src/pluggy/_manager.py:525 |  |  | 0.563 |
| walker |  | 2436 | 11 | python decl doc at src/pluggy/_manager.py:65 |  |  | 0.563 |
| walker |  | 2477 | 41 | python decl doc at src/pluggy/_manager.py:52 |  |  | 0.563 |
| walker |  | 2493 | 16 | python decl body at src/pluggy/_manager.py:525 body 526 |  |  | 0.563 |
| walker |  | 2521 | 28 | python decl at src/pluggy/_manager.py:37 |  |  | 0.563 |
| walker |  | 2580 | 59 | python imports in src/pluggy/_tracing.py |  |  | 0.563 |
| ns | 2581 |  | 306 | _warnings.py — full file (Pluggy*Warning classes) | 4.5 |  | 0.534 |
| ns | 2682 |  | 101 | _tracing.py — class + def names | 4.6 |  | 0.551 |
| walker |  | 2749 | 169 | python decl doc at src/pluggy/_manager.py:83 |  |  | 0.553 |
| ns | 2873 |  | 191 | PluginManager class docstring | 5.1 | 4.1 | 0.564 |
| walker |  | 2920 | 171 | python decl doc at src/pluggy/_warnings.py:10 |  |  | 0.603 |
| walker |  | 2928 | 8 | listing of 'docs/examples/eggsample' |  |  | 0.604 |
| walker |  | 2948 | 20 | listing of 'docs/examples/eggsample/eggsample' |  |  | 0.604 |
| walker |  | 2956 | 8 | python imports in docs/examples/eggsample/eggsample/__init__.py |  |  | 0.604 |
| ns | 3038 |  | 165 | HookspecMarker + HookimplMarker class docstrings | 5.2 | 4.2 | 0.590 |
| walker |  | 3043 | 87 | python method doc at src/pluggy/_result.py:67 |  |  | 0.591 |
| walker |  | 3192 | 149 | python decl names surface #1 in docs/conf.py |  |  | 0.591 |
| walker |  | 3192 | 0 | python decl at docs/conf.py:106 |  |  | 0.591 |
| walker |  | 3192 | 0 | python decl at docs/conf.py:130 |  |  | 0.591 |
| walker |  | 3199 | 7 | python decl body at docs/conf.py:130 body 131 |  |  | 0.591 |
| walker |  | 3217 | 18 | python decl doc at docs/conf.py:106 |  |  | 0.591 |
| walker |  | 3247 | 30 | python decl at docs/conf.py:54 |  |  | 0.591 |
| walker |  | 3328 | 81 | python decl at docs/conf.py:83 |  |  | 0.591 |
| ns | 3369 |  | 331 | HookCaller __slots__ + the 6-bucket call-order comment | 5.3 | 4.2 | 0.560 |
| walker |  | 3464 | 136 | python decl at docs/conf.py:96 |  |  | 0.560 |
| walker |  | 3610 | 146 | python decl at docs/conf.py:41 |  |  | 0.560 |
| ns | 3651 |  | 282 | HookCaller._add_hookimpl body — the actual ordering algorithm | 5.4 | 4.2 | 0.539 |
| walker |  | 3768 | 158 | python decl at docs/conf.py:66 |  |  | 0.539 |
| walker |  | 3817 | 49 | python method body at src/pluggy/_tracing.py:17 body 18 |  |  | 0.539 |
| walker |  | 3835 | 18 | python decl names surface in docs/examples/eggsample/eggsample/__init__.py |  |  | 0.539 |
| walker |  | 3846 | 11 | python method body at src/pluggy/_result.py:42 body 45 |  |  | 0.539 |
| walker |  | 3947 | 101 | python imports in src/pluggy/_result.py |  |  | 0.539 |
| walker |  | 4002 | 55 | python method body at src/pluggy/_tracing.py:51 body 52 |  |  | 0.539 |
| ns | 4080 |  | 429 | HookspecOpts + HookimplOpts — TypedDict bodies | 5.5 | 4.2 | 0.512 |
| walker |  | 4099 | 97 | json config .claude/settings.json |  |  | 0.512 |
| walker |  | 4112 | 13 | listing of '.github' |  |  | 0.512 |
| walker |  | 4116 | 4 | listing of '.github/workflows' |  |  | 0.512 |
| walker |  | 4130 | 14 | listing of 'scripts' |  |  | 0.512 |
| walker |  | 4311 | 181 | python decl names surface in src/pluggy/_hooks.py |  |  | 0.513 |
| walker |  | 4311 | 0 | python decl at src/pluggy/_hooks.py:40 |  |  | 0.513 |
| walker |  | 4311 | 0 | python decl at src/pluggy/_hooks.py:56 |  |  | 0.513 |
| walker |  | 4311 | 0 | python decl at src/pluggy/_hooks.py:281 |  |  | 0.513 |
| walker |  | 4318 | 7 | python decl at src/pluggy/_hooks.py:77 |  |  | 0.513 |
| walker |  | 4325 | 7 | python decl at src/pluggy/_hooks.py:164 |  |  | 0.514 |
| walker |  | 4335 | 10 | python decl doc at src/pluggy/_hooks.py:40 |  |  | 0.514 |
| walker |  | 4345 | 10 | python decl doc at src/pluggy/_hooks.py:56 |  |  | 0.514 |
| walker |  | 4359 | 14 | python class body at src/pluggy/_hooks.py:77 |  |  | 0.514 |
| walker |  | 4373 | 14 | python class body at src/pluggy/_hooks.py:164 |  |  | 0.514 |
| walker |  | 4450 | 77 | python decl doc at src/pluggy/_hooks.py:164 |  |  | 0.519 |
| ns | 4494 |  | 414 | Result API — force_result, force_exception, get_result bodies | 5.6 | 4.4 | 0.512 |
| walker |  | 4528 | 78 | python decl doc at src/pluggy/_hooks.py:77 |  |  | 0.526 |
| walker |  | 4662 | 134 | python method sigs in src/pluggy/_hooks.py |  |  | 0.530 |
| walker |  | 4662 | 0 | python method at src/pluggy/_hooks.py:88 |  |  | 0.530 |
| walker |  | 4662 | 0 | python method at src/pluggy/_hooks.py:175 |  |  | 0.530 |
| walker |  | 4674 | 12 | python method body at src/pluggy/_hooks.py:88 body 89 |  |  | 0.530 |
| walker |  | 4686 | 12 | python method body at src/pluggy/_hooks.py:175 body 176 |  |  | 0.530 |
| walker |  | 4722 | 36 | python decl at src/pluggy/_hooks.py:33 |  |  | 0.530 |
| ns | 4801 |  | 307 | docs/index.rst H2 heading map — sections at exact line numbers | 5.7 | 3.2 | 0.502 |
| walker |  | 4804 | 82 | python method at src/pluggy/_hooks.py:91 |  |  | 0.503 |
| walker |  | 4815 | 11 | python method body at src/pluggy/_hooks.py:91 body 99 |  |  | 0.503 |
| ns | 4855 |  | 54 | docs/examples/ FS listing | 6.1 |  | 0.508 |
| walker |  | 4905 | 90 | python method at src/pluggy/_hooks.py:101 |  |  | 0.509 |
| walker |  | 4921 | 16 | python method body at src/pluggy/_hooks.py:101 body 109 |  |  | 0.509 |
| ns | 5075 |  | 220 | Eggsample hookspecs.py — host-side hook specifications | 6.2 |  | 0.496 |
| walker |  | 5085 | 164 | python class body at src/pluggy/_hooks.py:40 |  |  | 0.506 |
| walker |  | 5180 | 95 | python method at src/pluggy/_hooks.py:111 |  |  | 0.506 |
| walker |  | 5277 | 97 | python method at src/pluggy/_hooks.py:178 |  |  | 0.507 |
| walker |  | 5288 | 11 | python method body at src/pluggy/_hooks.py:178 body 188 |  |  | 0.507 |
| walker |  | 5393 | 105 | python method at src/pluggy/_hooks.py:190 |  |  | 0.508 |
| walker |  | 5409 | 16 | python method body at src/pluggy/_hooks.py:190 body 200 |  |  | 0.508 |
| ns | 5479 |  | 404 | Eggsample lib.py + eggsample-spam (host impls + plugin impls) | 6.3 |  | 0.488 |
| walker |  | 5519 | 110 | python method at src/pluggy/_hooks.py:202 |  |  | 0.488 |
| walker |  | 5745 | 226 | python class body at src/pluggy/_hooks.py:56 |  |  | 0.523 |
| walker |  | 5820 | 75 | python decl body at src/pluggy/_hooks.py:281 body 282 |  |  | 0.523 |
| walker |  | 5933 | 113 | python decl names surface #1 in src/pluggy/_hooks.py |  |  | 0.527 |
| walker |  | 5933 | 0 | python decl at src/pluggy/_hooks.py:293 |  |  | 0.527 |
| walker |  | 5933 | 0 | python decl at src/pluggy/_hooks.py:382 |  |  | 0.527 |
| walker |  | 5933 | 0 | python decl at src/pluggy/_hooks.py:593 |  |  | 0.527 |
| walker |  | 5939 | 6 | python decl at src/pluggy/_hooks.py:358 |  |  | 0.529 |
| walker |  | 5945 | 6 | python decl at src/pluggy/_hooks.py:638 |  |  | 0.530 |
| walker |  | 5951 | 6 | python decl at src/pluggy/_hooks.py:696 |  |  | 0.531 |
| walker |  | 5966 | 15 | python decl doc at src/pluggy/_hooks.py:382 |  |  | 0.531 |
| walker |  | 5984 | 18 | python decl doc at src/pluggy/_hooks.py:638 |  |  | 0.531 |
| walker |  | 6015 | 31 | python decl doc at src/pluggy/_hooks.py:358 |  |  | 0.531 |
| ns | 6020 |  | 541 | docs/index.rst — "Call time order" section (tryfirst / trylast) | 6.4 | 5.7 | 0.502 |
| walker |  | 6060 | 45 | python class body at src/pluggy/_hooks.py:358 |  |  | 0.504 |
| walker |  | 6086 | 26 | python decl at src/pluggy/_hooks.py:377 |  |  | 0.504 |
| walker |  | 6174 | 88 | python class body at src/pluggy/_hooks.py:382 |  |  | 0.507 |
| walker |  | 6262 | 88 | python class body at src/pluggy/_hooks.py:696 |  |  | 0.507 |
| walker |  | 6292 | 30 | python decl doc at src/pluggy/_hooks.py:593 |  |  | 0.507 |
| walker |  | 6404 | 112 | python class body at src/pluggy/_hooks.py:638 |  |  | 0.507 |
| walker |  | 6476 | 72 | python decl doc at src/pluggy/_hooks.py:293 |  |  | 0.507 |
| ns | 6517 |  | 497 | docs/index.rst — Wrappers (new-style) wrapper-protocol summary | 6.5 | 5.7 | 0.490 |
| ns | 6579 |  | 62 | testing/ FS listing | 7.1 |  | 0.498 |
| ns | 6827 |  | 248 | testing/conftest.py — pm + he_pm fixtures | 7.2 |  | 0.487 |
| walker |  | 6860 | 384 | python method sigs #1 in src/pluggy/_hooks.py |  |  | 0.525 |
| walker |  | 6860 | 0 | python method at src/pluggy/_hooks.py:365 |  |  | 0.525 |
| walker |  | 6860 | 0 | python method at src/pluggy/_hooks.py:420 |  |  | 0.525 |
| walker |  | 6860 | 0 | python method at src/pluggy/_hooks.py:438 |  |  | 0.525 |
| walker |  | 6860 | 0 | python method at src/pluggy/_hooks.py:442 |  |  | 0.525 |
| walker |  | 6860 | 0 | python method at src/pluggy/_hooks.py:449 |  |  | 0.525 |
| walker |  | 6860 | 0 | python method at src/pluggy/_hooks.py:453 |  |  | 0.525 |
| walker |  | 6860 | 0 | python method at src/pluggy/_hooks.py:477 |  |  | 0.525 |
| walker |  | 6860 | 0 | python method at src/pluggy/_hooks.py:499 |  |  | 0.525 |
| walker |  | 6860 | 0 | python method at src/pluggy/_hooks.py:577 |  |  | 0.525 |
| walker |  | 6860 | 0 | python method at src/pluggy/_hooks.py:612 |  |  | 0.525 |
| walker |  | 6860 | 0 | python method at src/pluggy/_hooks.py:634 |  |  | 0.525 |
| walker |  | 6860 | 0 | python method at src/pluggy/_hooks.py:692 |  |  | 0.525 |
| walker |  | 6860 | 0 | python method at src/pluggy/_hooks.py:709 |  |  | 0.525 |
| walker |  | 6881 | 21 | python method at src/pluggy/_hooks.py:626 |  |  | 0.527 |
| walker |  | 6891 | 10 | python method doc at src/pluggy/_hooks.py:365 |  |  | 0.527 |
| walker |  | 6904 | 13 | python method doc at src/pluggy/_hooks.py:449 |  |  | 0.527 |
| walker |  | 6933 | 29 | python method at src/pluggy/_hooks.py:543 |  |  | 0.527 |
| walker |  | 6943 | 10 | python method body at src/pluggy/_hooks.py:420 body 421 |  |  | 0.527 |
| walker |  | 6954 | 11 | python method body at src/pluggy/_hooks.py:626 body 628 |  |  | 0.527 |
| walker |  | 6970 | 16 | python method doc at src/pluggy/_hooks.py:438 |  |  | 0.527 |
| walker |  | 6985 | 15 | python method at src/pluggy/_hooks.py:480 |  |  | 0.527 |
| walker |  | 6998 | 13 | python method body at src/pluggy/_hooks.py:449 body 451 |  |  | 0.527 |
| walker |  | 7037 | 39 | python method at src/pluggy/_hooks.py:424 |  |  | 0.527 |
| walker |  | 7053 | 16 | python method at src/pluggy/_hooks.py:618 |  |  | 0.529 |
| walker |  | 7067 | 14 | python method body at src/pluggy/_hooks.py:438 body 440 |  |  | 0.529 |
| walker |  | 7082 | 15 | python method body at src/pluggy/_hooks.py:477 body 478 |  |  | 0.529 |
| walker |  | 7094 | 12 | python method doc at src/pluggy/_hooks.py:453 |  |  | 0.529 |
| walker |  | 7111 | 17 | python method body at src/pluggy/_hooks.py:634 body 635 |  |  | 0.529 |
| walker |  | 7161 | 50 | python method at src/pluggy/_hooks.py:516 |  |  | 0.529 |
| walker |  | 7185 | 24 | python method at src/pluggy/_hooks.py:630 |  |  | 0.532 |
| ns | 7193 |  | 366 | test_pluginmanager.py — every test fn name | 7.3 |  | 0.517 |
| walker |  | 7246 | 61 | python method at src/pluggy/_hooks.py:656 |  |  | 0.517 |
| walker |  | 7258 | 12 | python method doc at src/pluggy/_hooks.py:656 |  |  | 0.517 |
| walker |  | 7326 | 68 | python method at src/pluggy/_hooks.py:393 |  |  | 0.523 |
| walker |  | 7338 | 12 | python method doc at src/pluggy/_hooks.py:393 |  |  | 0.524 |
| walker |  | 7357 | 19 | python method doc at src/pluggy/_hooks.py:577 |  |  | 0.524 |
| walker |  | 7402 | 45 | python method doc at src/pluggy/_hooks.py:543 |  |  | 0.524 |
| walker |  | 7427 | 25 | python method body at src/pluggy/_hooks.py:692 body 693 |  |  | 0.524 |
| walker |  | 7440 | 13 | python method body at src/pluggy/_hooks.py:630 body 632 |  |  | 0.524 |
| walker |  | 7505 | 65 | python method doc at src/pluggy/_hooks.py:499 |  |  | 0.524 |
| walker |  | 7598 | 93 | python method doc at src/pluggy/_hooks.py:516 |  |  | 0.524 |
| ns | 7623 |  | 430 | test_hookcaller.py + test_multicall.py — every test fn name | 7.4 |  | 0.509 |
| walker |  | 7660 | 62 | python method body at src/pluggy/_hooks.py:612 body 613 |  |  | 0.509 |
| ns | 8023 |  | 400 | Smaller test files — every test fn name | 7.5 |  | 0.495 |
| walker |  | 8274 | 614 | python method sigs in src/pluggy/_manager.py |  |  | 0.536 |
| walker |  | 8274 | 0 | python method at src/pluggy/_manager.py:59 |  |  | 0.536 |
| walker |  | 8274 | 0 | python method at src/pluggy/_manager.py:68 |  |  | 0.536 |
| walker |  | 8274 | 0 | python method at src/pluggy/_manager.py:76 |  |  | 0.536 |
| walker |  | 8274 | 0 | python method at src/pluggy/_manager.py:79 |  |  | 0.536 |
| walker |  | 8274 | 0 | python method at src/pluggy/_manager.py:125 |  |  | 0.536 |
| walker |  | 8274 | 0 | python method at src/pluggy/_manager.py:176 |  |  | 0.536 |
| walker |  | 8274 | 0 | python method at src/pluggy/_manager.py:233 |  |  | 0.536 |
| walker |  | 8274 | 0 | python method at src/pluggy/_manager.py:238 |  |  | 0.536 |
| walker |  | 8274 | 0 | python method at src/pluggy/_manager.py:242 |  |  | 0.536 |
| walker |  | 8274 | 0 | python method at src/pluggy/_manager.py:252 |  |  | 0.536 |
| walker |  | 8274 | 0 | python method at src/pluggy/_manager.py:296 |  |  | 0.536 |
| walker |  | 8274 | 0 | python method at src/pluggy/_manager.py:300 |  |  | 0.536 |
| walker |  | 8274 | 0 | python method at src/pluggy/_manager.py:304 |  |  | 0.536 |
| walker |  | 8274 | 0 | python method at src/pluggy/_manager.py:315 |  |  | 0.536 |
| walker |  | 8274 | 0 | python method at src/pluggy/_manager.py:319 |  |  | 0.536 |
| walker |  | 8274 | 0 | python method at src/pluggy/_manager.py:323 |  |  | 0.536 |
| walker |  | 8274 | 0 | python method at src/pluggy/_manager.py:331 |  |  | 0.536 |
| walker |  | 8274 | 0 | python method at src/pluggy/_manager.py:379 |  |  | 0.536 |
| walker |  | 8274 | 0 | python method at src/pluggy/_manager.py:395 |  |  | 0.536 |
| walker |  | 8274 | 0 | python method at src/pluggy/_manager.py:425 |  |  | 0.536 |
| walker |  | 8274 | 0 | python method at src/pluggy/_manager.py:430 |  |  | 0.536 |
| walker |  | 8274 | 0 | python method at src/pluggy/_manager.py:434 |  |  | 0.536 |
| walker |  | 8274 | 0 | python method at src/pluggy/_manager.py:487 |  |  | 0.536 |
| walker |  | 8282 | 8 | python method at src/pluggy/_manager.py:100 |  |  | 0.536 |
| walker |  | 8293 | 11 | python method at src/pluggy/_manager.py:71 |  |  | 0.539 |
| walker |  | 8305 | 12 | python method doc at src/pluggy/_manager.py:300 |  |  | 0.539 |
| ns | 8324 |  | 301 | pyproject.toml — [project] essentials (skip classifier list) | 7.6 |  | 0.528 |
| walker |  | 8331 | 26 | python method at src/pluggy/_manager.py:512 |  |  | 0.528 |
| walker |  | 8340 | 9 | python method body at src/pluggy/_manager.py:68 body 69 |  |  | 0.528 |
| walker |  | 8353 | 13 | python method doc at src/pluggy/_manager.py:238 |  |  | 0.528 |
| walker |  | 8366 | 13 | python method doc at src/pluggy/_manager.py:296 |  |  | 0.528 |
| walker |  | 8395 | 29 | python method at src/pluggy/_manager.py:278 |  |  | 0.528 |
| walker |  | 8424 | 29 | python method at src/pluggy/_manager.py:450 |  |  | 0.528 |
| walker |  | 8439 | 15 | python method doc at src/pluggy/_manager.py:319 |  |  | 0.528 |
| walker |  | 8455 | 16 | python method doc at src/pluggy/_manager.py:233 |  |  | 0.528 |
| walker |  | 8471 | 16 | python method doc at src/pluggy/_manager.py:315 |  |  | 0.528 |
| walker |  | 8504 | 33 | python method at src/pluggy/_manager.py:201 |  |  | 0.528 |
| walker |  | 8523 | 19 | python method doc at src/pluggy/_manager.py:430 |  |  | 0.528 |
| walker |  | 8536 | 13 | python method body at src/pluggy/_manager.py:76 body 77 |  |  | 0.528 |
| walker |  | 8549 | 13 | python method body at src/pluggy/_manager.py:425 body 428 |  |  | 0.528 |
| walker |  | 8563 | 14 | python method body at src/pluggy/_manager.py:315 body 317 |  |  | 0.528 |
| ns | 8576 |  | 252 | pyproject.toml — [tool.ruff.lint] config | 7.7 | 7.6 | 0.521 |
| walker |  | 8577 | 14 | python method body at src/pluggy/_manager.py:430 body 432 |  |  | 0.521 |
| walker |  | 8592 | 15 | python method body at src/pluggy/_manager.py:319 body 321 |  |  | 0.521 |
| walker |  | 8619 | 27 | python method doc at src/pluggy/_manager.py:323 |  |  | 0.521 |
| walker |  | 8647 | 28 | python method doc at src/pluggy/_manager.py:242 |  |  | 0.521 |
| walker |  | 8675 | 28 | python method doc at src/pluggy/_manager.py:425 |  |  | 0.521 |
| walker |  | 8694 | 19 | python method body at src/pluggy/_manager.py:79 body 80 |  |  | 0.521 |
| walker |  | 8714 | 20 | python method body at src/pluggy/_manager.py:300 body 302 |  |  | 0.521 |
| walker |  | 8749 | 35 | python method doc at src/pluggy/_manager.py:487 |  |  | 0.521 |
| walker |  | 8771 | 22 | python method body at src/pluggy/_manager.py:71 body 73 |  |  | 0.521 |
| walker |  | 8813 | 42 | python method doc at src/pluggy/_manager.py:379 |  |  | 0.521 |
| walker |  | 8837 | 24 | python method body at src/pluggy/_manager.py:238 body 240 |  |  | 0.521 |
| ns | 8859 |  | 283 | tox.ini — [tox] + [testenv] + [pytest] (skip release/docs envs) | 7.8 |  | 0.512 |
| walker |  | 8861 | 24 | python method body at src/pluggy/_manager.py:296 body 298 |  |  | 0.512 |
| walker |  | 8911 | 50 | python method doc at src/pluggy/_manager.py:512 |  |  | 0.512 |
| walker |  | 8967 | 56 | python method doc at src/pluggy/_manager.py:252 |  |  | 0.512 |
| walker |  | 9025 | 58 | python method doc at src/pluggy/_manager.py:434 |  |  | 0.512 |
| walker |  | 9056 | 31 | python method body at src/pluggy/_manager.py:59 body 60 |  |  | 0.512 |
| walker |  | 9064 | 8 | python method body at src/pluggy/_manager.py:242 body 250 |  |  | 0.512 |
| walker |  | 9072 | 8 | python method body at src/pluggy/_manager.py:323 body 329 |  |  | 0.512 |
| walker |  | 9149 | 77 | python method doc at src/pluggy/_manager.py:201 |  |  | 0.512 |
| ns | 9238 |  | 379 | CHANGELOG.rst — pluggy 1.6.0 entry only | 7.9 |  | 0.502 |
| walker |  | 9239 | 90 | python method doc at src/pluggy/_manager.py:304 |  |  | 0.502 |
| ns | 9300 |  | 62 | changelog/ + downstream/ FS listings | 7.10 |  | 0.508 |
| walker |  | 9330 | 91 | python method doc at src/pluggy/_manager.py:395 |  |  | 0.508 |
| walker |  | 9340 | 10 | python method body at src/pluggy/_manager.py:233 body 235 |  |  | 0.508 |
| walker |  | 9460 | 120 | python method doc at src/pluggy/_manager.py:125 |  |  | 0.508 |
| walker |  | 9583 | 123 | python method doc at src/pluggy/_manager.py:176 |  |  | 0.508 |
| walker |  | 9595 | 12 | python method body at src/pluggy/_manager.py:304 body 313 |  |  | 0.508 |
| walker |  | 9731 | 136 | python method doc at src/pluggy/_manager.py:278 |  |  | 0.508 |
| ns | 9819 |  | 519 | _callers._multicall body — the actual call loop | 7.11 | 4.3 | 0.495 |
| walker |  | 9895 | 164 | python method doc at src/pluggy/_manager.py:450 |  |  | 0.495 |
| walker |  | 9910 | 15 | python method body at src/pluggy/_manager.py:233 body 236 |  |  | 0.495 |
| ns | 9946 |  | 127 | Top-level meta — AGENTS.md, SECURITY.md, MANIFEST.in | 7.12 |  | 0.491 |
| ns | 9964 |  | 18 | scripts/ FS + .github/workflows/ FS | 7.13 |  | 0.492 |
| walker |  | 9976 | 66 | python decl body at src/pluggy/_callers.py:60 body 64 |  |  | 0.492 |
| ns | 9996 |  | 32 | docs/ root FS + docs/requirements.txt | 7.14 |  | 0.496 |

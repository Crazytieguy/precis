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
| walker |  | 883 | 62 | listing of 'testing' |  |  | 0.707 |
| walker |  | 938 | 55 | python decl names surface in src/pluggy/_tracing.py |  |  | 0.707 |
| walker |  | 938 | 0 | python decl at src/pluggy/_tracing.py:16 |  |  | 0.707 |
| walker |  | 938 | 0 | python decl at src/pluggy/_tracing.py:59 |  |  | 0.707 |
| ns | 995 |  | 166 | __init__.py re-exports — name → owning module | 3.1 | 1.4 | 0.731 |
| walker |  | 1006 | 68 | python decl names surface in src/pluggy/_callers.py |  |  | 0.732 |
| walker |  | 1035 | 29 | python decl at src/pluggy/_callers.py:27 |  |  | 0.732 |
| walker |  | 1066 | 31 | python decl doc at src/pluggy/_callers.py:27 |  |  | 0.732 |
| walker |  | 1093 | 27 | python decl at src/pluggy/_callers.py:70 |  |  | 0.732 |
| walker |  | 1125 | 32 | python decl at src/pluggy/_callers.py:60 |  |  | 0.732 |
| walker |  | 1194 | 69 | python decl names surface in src/pluggy/_result.py |  |  | 0.732 |
| walker |  | 1194 | 0 | python decl at src/pluggy/_result.py:20 |  |  | 0.732 |
| walker |  | 1203 | 9 | python decl at src/pluggy/_result.py:24 |  |  | 0.732 |
| walker |  | 1212 | 9 | python decl doc at src/pluggy/_result.py:20 |  |  | 0.732 |
| walker |  | 1232 | 20 | python class body at src/pluggy/_result.py:24 |  |  | 0.732 |
| ns | 1261 |  | 266 | docs/index.rst lede — "what is pluggy" | 3.2 |  | 0.667 |
| walker |  | 1266 | 34 | python decl doc at src/pluggy/_result.py:24 |  |  | 0.667 |
| walker |  | 1363 | 97 | python method sigs in src/pluggy/_result.py |  |  | 0.669 |
| walker |  | 1363 | 0 | python method at src/pluggy/_result.py:67 |  |  | 0.669 |
| walker |  | 1363 | 0 | python method at src/pluggy/_result.py:80 |  |  | 0.669 |
| walker |  | 1363 | 0 | python method at src/pluggy/_result.py:91 |  |  | 0.669 |
| walker |  | 1376 | 13 | python method at src/pluggy/_result.py:51 |  |  | 0.669 |
| walker |  | 1391 | 15 | python method at src/pluggy/_result.py:42 |  |  | 0.670 |
| walker |  | 1413 | 22 | python method at src/pluggy/_result.py:56 |  |  | 0.671 |
| walker |  | 1425 | 12 | python method doc at src/pluggy/_result.py:42 |  |  | 0.671 |
| walker |  | 1437 | 12 | python method doc at src/pluggy/_result.py:51 |  |  | 0.671 |
| walker |  | 1449 | 12 | python method doc at src/pluggy/_result.py:56 |  |  | 0.671 |
| walker |  | 1459 | 10 | python method body at src/pluggy/_result.py:51 body 54 |  |  | 0.671 |
| walker |  | 1497 | 38 | python method at src/pluggy/_result.py:31 |  |  | 0.671 |
| walker |  | 1509 | 12 | python method doc at src/pluggy/_result.py:31 |  |  | 0.671 |
| walker |  | 1513 | 4 | listing of '.claude' |  |  | 0.671 |
| walker |  | 1528 | 15 | listing of 'docs/examples' |  |  | 0.671 |
| walker |  | 1577 | 49 | python method doc at src/pluggy/_result.py:80 |  |  | 0.671 |
| ns | 1595 |  | 334 | _manager.py — every class + def name | 4.1 |  | 0.587 |
| walker |  | 1633 | 56 | python method doc at src/pluggy/_result.py:91 |  |  | 0.587 |
| walker |  | 1690 | 57 | python decl at src/pluggy/_callers.py:82 |  |  | 0.589 |
| walker |  | 1900 | 210 | python method sigs in src/pluggy/_tracing.py |  |  | 0.592 |
| walker |  | 1900 | 0 | python method at src/pluggy/_tracing.py:17 |  |  | 0.592 |
| walker |  | 1900 | 0 | python method at src/pluggy/_tracing.py:22 |  |  | 0.592 |
| walker |  | 1900 | 0 | python method at src/pluggy/_tracing.py:25 |  |  | 0.592 |
| walker |  | 1900 | 0 | python method at src/pluggy/_tracing.py:42 |  |  | 0.592 |
| walker |  | 1900 | 0 | python method at src/pluggy/_tracing.py:48 |  |  | 0.592 |
| walker |  | 1900 | 0 | python method at src/pluggy/_tracing.py:51 |  |  | 0.592 |
| walker |  | 1900 | 0 | python method at src/pluggy/_tracing.py:60 |  |  | 0.592 |
| walker |  | 1900 | 0 | python method at src/pluggy/_tracing.py:64 |  |  | 0.592 |
| walker |  | 1900 | 0 | python method at src/pluggy/_tracing.py:67 |  |  | 0.592 |
| walker |  | 1909 | 9 | python method body at src/pluggy/_tracing.py:48 body 49 |  |  | 0.592 |
| walker |  | 1921 | 12 | python method body at src/pluggy/_tracing.py:22 body 23 |  |  | 0.592 |
| walker |  | 1934 | 13 | python method body at src/pluggy/_tracing.py:64 body 65 |  |  | 0.592 |
| walker |  | 1951 | 17 | python method body at src/pluggy/_tracing.py:67 body 68 |  |  | 0.592 |
| walker |  | 1969 | 18 | python method body at src/pluggy/_tracing.py:60 body 61 |  |  | 0.592 |
| walker |  | 2020 | 51 | python decl doc at src/pluggy/_callers.py:82 |  |  | 0.592 |
| ns | 2037 |  | 442 | _hooks.py — every class + def name (markers, HookCaller, HookImpl, HookSpec) | 4.2 |  | 0.518 |
| walker |  | 2136 | 116 | python decl names surface in src/pluggy/_manager.py |  |  | 0.522 |
| walker |  | 2136 | 0 | python decl at src/pluggy/_manager.py:42 |  |  | 0.522 |
| walker |  | 2136 | 0 | python decl at src/pluggy/_manager.py:52 |  |  | 0.522 |
| walker |  | 2136 | 0 | python decl at src/pluggy/_manager.py:65 |  |  | 0.522 |
| walker |  | 2136 | 0 | python decl at src/pluggy/_manager.py:83 |  |  | 0.522 |
| walker |  | 2136 | 0 | python decl at src/pluggy/_manager.py:525 |  |  | 0.522 |
| ns | 2138 |  | 101 | _callers.py — every def + _multicall signature | 4.3 |  | 0.541 |
| walker |  | 2147 | 11 | python decl doc at src/pluggy/_manager.py:65 |  |  | 0.541 |
| walker |  | 2188 | 41 | python decl doc at src/pluggy/_manager.py:52 |  |  | 0.541 |
| walker |  | 2204 | 16 | python decl body at src/pluggy/_manager.py:525 body 526 |  |  | 0.541 |
| walker |  | 2232 | 28 | python decl at src/pluggy/_manager.py:37 |  |  | 0.541 |
| ns | 2275 |  | 137 | _result.py — Result class + every method (in full) | 4.4 |  | 0.563 |
| walker |  | 2291 | 59 | python imports in src/pluggy/_tracing.py |  |  | 0.563 |
| walker |  | 2460 | 169 | python decl doc at src/pluggy/_manager.py:83 |  |  | 0.566 |
| ns | 2581 |  | 306 | _warnings.py — full file (Pluggy*Warning classes) | 4.5 |  | 0.536 |
| walker |  | 2631 | 171 | python decl doc at src/pluggy/_warnings.py:10 |  |  | 0.581 |
| walker |  | 2639 | 8 | listing of 'docs/examples/eggsample' |  |  | 0.581 |
| walker |  | 2659 | 20 | listing of 'docs/examples/eggsample/eggsample' |  |  | 0.582 |
| walker |  | 2667 | 8 | python imports in docs/examples/eggsample/eggsample/__init__.py |  |  | 0.582 |
| ns | 2682 |  | 101 | _tracing.py — class + def names | 4.6 |  | 0.597 |
| walker |  | 2754 | 87 | python method doc at src/pluggy/_result.py:67 |  |  | 0.598 |
| walker |  | 2803 | 49 | python method body at src/pluggy/_tracing.py:17 body 18 |  |  | 0.598 |
| walker |  | 2821 | 18 | python decl names surface in docs/examples/eggsample/eggsample/__init__.py |  |  | 0.598 |
| walker |  | 2832 | 11 | python method body at src/pluggy/_result.py:42 body 45 |  |  | 0.598 |
| ns | 2873 |  | 191 | PluginManager class docstring | 5.1 | 4.1 | 0.605 |
| walker |  | 2933 | 101 | python imports in src/pluggy/_result.py |  |  | 0.605 |
| walker |  | 2988 | 55 | python method body at src/pluggy/_tracing.py:51 body 52 |  |  | 0.605 |
| walker |  | 3001 | 13 | listing of '.github' |  |  | 0.605 |
| walker |  | 3005 | 4 | listing of '.github/workflows' |  |  | 0.605 |
| walker |  | 3019 | 14 | listing of 'scripts' |  |  | 0.606 |
| ns | 3038 |  | 165 | HookspecMarker + HookimplMarker class docstrings | 5.2 | 4.2 | 0.591 |
| walker |  | 3200 | 181 | python decl names surface in src/pluggy/_hooks.py |  |  | 0.592 |
| walker |  | 3200 | 0 | python decl at src/pluggy/_hooks.py:40 |  |  | 0.592 |
| walker |  | 3200 | 0 | python decl at src/pluggy/_hooks.py:56 |  |  | 0.592 |
| walker |  | 3200 | 0 | python decl at src/pluggy/_hooks.py:281 |  |  | 0.592 |
| walker |  | 3207 | 7 | python decl at src/pluggy/_hooks.py:77 |  |  | 0.592 |
| walker |  | 3214 | 7 | python decl at src/pluggy/_hooks.py:164 |  |  | 0.593 |
| walker |  | 3224 | 10 | python decl doc at src/pluggy/_hooks.py:40 |  |  | 0.593 |
| walker |  | 3234 | 10 | python decl doc at src/pluggy/_hooks.py:56 |  |  | 0.593 |
| walker |  | 3248 | 14 | python class body at src/pluggy/_hooks.py:77 |  |  | 0.593 |
| walker |  | 3262 | 14 | python class body at src/pluggy/_hooks.py:164 |  |  | 0.593 |
| walker |  | 3339 | 77 | python decl doc at src/pluggy/_hooks.py:164 |  |  | 0.599 |
| ns | 3369 |  | 331 | HookCaller __slots__ + the 6-bucket call-order comment | 5.3 | 4.2 | 0.567 |
| walker |  | 3417 | 78 | python decl doc at src/pluggy/_hooks.py:77 |  |  | 0.585 |
| walker |  | 3551 | 134 | python method sigs in src/pluggy/_hooks.py |  |  | 0.589 |
| walker |  | 3551 | 0 | python method at src/pluggy/_hooks.py:88 |  |  | 0.589 |
| walker |  | 3551 | 0 | python method at src/pluggy/_hooks.py:175 |  |  | 0.589 |
| walker |  | 3563 | 12 | python method body at src/pluggy/_hooks.py:88 body 89 |  |  | 0.589 |
| walker |  | 3575 | 12 | python method body at src/pluggy/_hooks.py:175 body 176 |  |  | 0.589 |
| walker |  | 3611 | 36 | python decl at src/pluggy/_hooks.py:33 |  |  | 0.589 |
| ns | 3651 |  | 282 | HookCaller._add_hookimpl body — the actual ordering algorithm | 5.4 | 4.2 | 0.567 |
| walker |  | 3693 | 82 | python method at src/pluggy/_hooks.py:91 |  |  | 0.568 |
| walker |  | 3704 | 11 | python method body at src/pluggy/_hooks.py:91 body 99 |  |  | 0.568 |
| walker |  | 3794 | 90 | python method at src/pluggy/_hooks.py:101 |  |  | 0.569 |
| walker |  | 3810 | 16 | python method body at src/pluggy/_hooks.py:101 body 109 |  |  | 0.569 |
| walker |  | 3974 | 164 | python class body at src/pluggy/_hooks.py:40 |  |  | 0.570 |
| walker |  | 4069 | 95 | python method at src/pluggy/_hooks.py:111 |  |  | 0.570 |
| ns | 4080 |  | 429 | HookspecOpts + HookimplOpts — TypedDict bodies | 5.5 | 4.2 | 0.553 |
| walker |  | 4166 | 97 | python method at src/pluggy/_hooks.py:178 |  |  | 0.554 |
| walker |  | 4177 | 11 | python method body at src/pluggy/_hooks.py:178 body 188 |  |  | 0.554 |
| walker |  | 4282 | 105 | python method at src/pluggy/_hooks.py:190 |  |  | 0.556 |
| walker |  | 4298 | 16 | python method body at src/pluggy/_hooks.py:190 body 200 |  |  | 0.556 |
| walker |  | 4408 | 110 | python method at src/pluggy/_hooks.py:202 |  |  | 0.556 |
| ns | 4494 |  | 414 | Result API — force_result, force_exception, get_result bodies | 5.6 | 4.4 | 0.545 |
| walker |  | 4634 | 226 | python class body at src/pluggy/_hooks.py:56 |  |  | 0.587 |
| walker |  | 4709 | 75 | python decl body at src/pluggy/_hooks.py:281 body 282 |  |  | 0.587 |
| ns | 4801 |  | 307 | docs/index.rst H2 heading map — sections at exact line numbers | 5.7 | 3.2 | 0.557 |
| walker |  | 4822 | 113 | python decl names surface #1 in src/pluggy/_hooks.py |  |  | 0.561 |
| walker |  | 4822 | 0 | python decl at src/pluggy/_hooks.py:293 |  |  | 0.561 |
| walker |  | 4822 | 0 | python decl at src/pluggy/_hooks.py:382 |  |  | 0.561 |
| walker |  | 4822 | 0 | python decl at src/pluggy/_hooks.py:593 |  |  | 0.561 |
| walker |  | 4828 | 6 | python decl at src/pluggy/_hooks.py:358 |  |  | 0.562 |
| walker |  | 4834 | 6 | python decl at src/pluggy/_hooks.py:638 |  |  | 0.563 |
| walker |  | 4840 | 6 | python decl at src/pluggy/_hooks.py:696 |  |  | 0.565 |
| walker |  | 4855 | 15 | python decl doc at src/pluggy/_hooks.py:382 |  |  | 0.568 |
| ns | 4855 |  | 54 | docs/examples/ FS listing | 6.1 |  | 0.568 |
| walker |  | 4873 | 18 | python decl doc at src/pluggy/_hooks.py:638 |  |  | 0.568 |
| walker |  | 4904 | 31 | python decl doc at src/pluggy/_hooks.py:358 |  |  | 0.568 |
| walker |  | 4949 | 45 | python class body at src/pluggy/_hooks.py:358 |  |  | 0.570 |
| walker |  | 4975 | 26 | python decl at src/pluggy/_hooks.py:377 |  |  | 0.570 |
| walker |  | 5063 | 88 | python class body at src/pluggy/_hooks.py:382 |  |  | 0.573 |
| ns | 5075 |  | 220 | Eggsample hookspecs.py — host-side hook specifications | 6.2 |  | 0.558 |
| walker |  | 5151 | 88 | python class body at src/pluggy/_hooks.py:696 |  |  | 0.558 |
| walker |  | 5181 | 30 | python decl doc at src/pluggy/_hooks.py:593 |  |  | 0.558 |
| walker |  | 5293 | 112 | python class body at src/pluggy/_hooks.py:638 |  |  | 0.558 |
| walker |  | 5365 | 72 | python decl doc at src/pluggy/_hooks.py:293 |  |  | 0.558 |
| ns | 5479 |  | 404 | Eggsample lib.py + eggsample-spam (host impls + plugin impls) | 6.3 |  | 0.535 |
| walker |  | 5749 | 384 | python method sigs #1 in src/pluggy/_hooks.py |  |  | 0.579 |
| walker |  | 5749 | 0 | python method at src/pluggy/_hooks.py:365 |  |  | 0.579 |
| walker |  | 5749 | 0 | python method at src/pluggy/_hooks.py:420 |  |  | 0.579 |
| walker |  | 5749 | 0 | python method at src/pluggy/_hooks.py:438 |  |  | 0.579 |
| walker |  | 5749 | 0 | python method at src/pluggy/_hooks.py:442 |  |  | 0.579 |
| walker |  | 5749 | 0 | python method at src/pluggy/_hooks.py:449 |  |  | 0.579 |
| walker |  | 5749 | 0 | python method at src/pluggy/_hooks.py:453 |  |  | 0.579 |
| walker |  | 5749 | 0 | python method at src/pluggy/_hooks.py:477 |  |  | 0.579 |
| walker |  | 5749 | 0 | python method at src/pluggy/_hooks.py:499 |  |  | 0.579 |
| walker |  | 5749 | 0 | python method at src/pluggy/_hooks.py:577 |  |  | 0.579 |
| walker |  | 5749 | 0 | python method at src/pluggy/_hooks.py:612 |  |  | 0.579 |
| walker |  | 5749 | 0 | python method at src/pluggy/_hooks.py:634 |  |  | 0.579 |
| walker |  | 5749 | 0 | python method at src/pluggy/_hooks.py:692 |  |  | 0.579 |
| walker |  | 5749 | 0 | python method at src/pluggy/_hooks.py:709 |  |  | 0.579 |
| walker |  | 5770 | 21 | python method at src/pluggy/_hooks.py:626 |  |  | 0.581 |
| walker |  | 5780 | 10 | python method doc at src/pluggy/_hooks.py:365 |  |  | 0.581 |
| walker |  | 5793 | 13 | python method doc at src/pluggy/_hooks.py:449 |  |  | 0.581 |
| walker |  | 5822 | 29 | python method at src/pluggy/_hooks.py:543 |  |  | 0.581 |
| walker |  | 5832 | 10 | python method body at src/pluggy/_hooks.py:420 body 421 |  |  | 0.581 |
| walker |  | 5843 | 11 | python method body at src/pluggy/_hooks.py:626 body 628 |  |  | 0.581 |
| walker |  | 5859 | 16 | python method doc at src/pluggy/_hooks.py:438 |  |  | 0.581 |
| walker |  | 5874 | 15 | python method at src/pluggy/_hooks.py:480 |  |  | 0.581 |
| walker |  | 5887 | 13 | python method body at src/pluggy/_hooks.py:449 body 451 |  |  | 0.581 |
| walker |  | 5926 | 39 | python method at src/pluggy/_hooks.py:424 |  |  | 0.581 |
| walker |  | 5942 | 16 | python method at src/pluggy/_hooks.py:618 |  |  | 0.584 |
| walker |  | 5956 | 14 | python method body at src/pluggy/_hooks.py:438 body 440 |  |  | 0.584 |
| walker |  | 5971 | 15 | python method body at src/pluggy/_hooks.py:477 body 478 |  |  | 0.584 |
| walker |  | 5983 | 12 | python method doc at src/pluggy/_hooks.py:453 |  |  | 0.584 |
| walker |  | 6000 | 17 | python method body at src/pluggy/_hooks.py:634 body 635 |  |  | 0.584 |
| ns | 6020 |  | 541 | docs/index.rst — "Call time order" section (tryfirst / trylast) | 6.4 | 5.7 | 0.552 |
| walker |  | 6050 | 50 | python method at src/pluggy/_hooks.py:516 |  |  | 0.552 |
| walker |  | 6074 | 24 | python method at src/pluggy/_hooks.py:630 |  |  | 0.555 |
| walker |  | 6135 | 61 | python method at src/pluggy/_hooks.py:656 |  |  | 0.555 |
| walker |  | 6147 | 12 | python method doc at src/pluggy/_hooks.py:656 |  |  | 0.555 |
| walker |  | 6215 | 68 | python method at src/pluggy/_hooks.py:393 |  |  | 0.562 |
| walker |  | 6227 | 12 | python method doc at src/pluggy/_hooks.py:393 |  |  | 0.563 |
| walker |  | 6246 | 19 | python method doc at src/pluggy/_hooks.py:577 |  |  | 0.563 |
| walker |  | 6291 | 45 | python method doc at src/pluggy/_hooks.py:543 |  |  | 0.563 |
| walker |  | 6316 | 25 | python method body at src/pluggy/_hooks.py:692 body 693 |  |  | 0.563 |
| walker |  | 6329 | 13 | python method body at src/pluggy/_hooks.py:630 body 632 |  |  | 0.563 |
| walker |  | 6394 | 65 | python method doc at src/pluggy/_hooks.py:499 |  |  | 0.563 |
| walker |  | 6487 | 93 | python method doc at src/pluggy/_hooks.py:516 |  |  | 0.563 |
| ns | 6517 |  | 497 | docs/index.rst — Wrappers (new-style) wrapper-protocol summary | 6.5 | 5.7 | 0.544 |
| walker |  | 6549 | 62 | python method body at src/pluggy/_hooks.py:612 body 613 |  |  | 0.544 |
| ns | 6579 |  | 62 | testing/ FS listing | 7.1 |  | 0.551 |
| ns | 6827 |  | 248 | testing/conftest.py — pm + he_pm fixtures | 7.2 |  | 0.539 |
| walker |  | 7163 | 614 | python method sigs in src/pluggy/_manager.py |  |  | 0.583 |
| walker |  | 7163 | 0 | python method at src/pluggy/_manager.py:59 |  |  | 0.583 |
| walker |  | 7163 | 0 | python method at src/pluggy/_manager.py:68 |  |  | 0.583 |
| walker |  | 7163 | 0 | python method at src/pluggy/_manager.py:76 |  |  | 0.583 |
| walker |  | 7163 | 0 | python method at src/pluggy/_manager.py:79 |  |  | 0.583 |
| walker |  | 7163 | 0 | python method at src/pluggy/_manager.py:125 |  |  | 0.583 |
| walker |  | 7163 | 0 | python method at src/pluggy/_manager.py:176 |  |  | 0.583 |
| walker |  | 7163 | 0 | python method at src/pluggy/_manager.py:233 |  |  | 0.583 |
| walker |  | 7163 | 0 | python method at src/pluggy/_manager.py:238 |  |  | 0.583 |
| walker |  | 7163 | 0 | python method at src/pluggy/_manager.py:242 |  |  | 0.583 |
| walker |  | 7163 | 0 | python method at src/pluggy/_manager.py:252 |  |  | 0.583 |
| walker |  | 7163 | 0 | python method at src/pluggy/_manager.py:296 |  |  | 0.583 |
| walker |  | 7163 | 0 | python method at src/pluggy/_manager.py:300 |  |  | 0.583 |
| walker |  | 7163 | 0 | python method at src/pluggy/_manager.py:304 |  |  | 0.583 |
| walker |  | 7163 | 0 | python method at src/pluggy/_manager.py:315 |  |  | 0.583 |
| walker |  | 7163 | 0 | python method at src/pluggy/_manager.py:319 |  |  | 0.583 |
| walker |  | 7163 | 0 | python method at src/pluggy/_manager.py:323 |  |  | 0.583 |
| walker |  | 7163 | 0 | python method at src/pluggy/_manager.py:331 |  |  | 0.583 |
| walker |  | 7163 | 0 | python method at src/pluggy/_manager.py:379 |  |  | 0.583 |
| walker |  | 7163 | 0 | python method at src/pluggy/_manager.py:395 |  |  | 0.583 |
| walker |  | 7163 | 0 | python method at src/pluggy/_manager.py:425 |  |  | 0.583 |
| walker |  | 7163 | 0 | python method at src/pluggy/_manager.py:430 |  |  | 0.583 |
| walker |  | 7163 | 0 | python method at src/pluggy/_manager.py:434 |  |  | 0.583 |
| walker |  | 7163 | 0 | python method at src/pluggy/_manager.py:487 |  |  | 0.583 |
| walker |  | 7171 | 8 | python method at src/pluggy/_manager.py:100 |  |  | 0.583 |
| walker |  | 7182 | 11 | python method at src/pluggy/_manager.py:71 |  |  | 0.586 |
| ns | 7193 |  | 366 | test_pluginmanager.py — every test fn name | 7.3 |  | 0.570 |
| walker |  | 7194 | 12 | python method doc at src/pluggy/_manager.py:300 |  |  | 0.570 |
| walker |  | 7220 | 26 | python method at src/pluggy/_manager.py:512 |  |  | 0.570 |
| walker |  | 7229 | 9 | python method body at src/pluggy/_manager.py:68 body 69 |  |  | 0.570 |
| walker |  | 7242 | 13 | python method doc at src/pluggy/_manager.py:238 |  |  | 0.570 |
| walker |  | 7255 | 13 | python method doc at src/pluggy/_manager.py:296 |  |  | 0.570 |
| walker |  | 7284 | 29 | python method at src/pluggy/_manager.py:278 |  |  | 0.570 |
| walker |  | 7313 | 29 | python method at src/pluggy/_manager.py:450 |  |  | 0.570 |
| walker |  | 7328 | 15 | python method doc at src/pluggy/_manager.py:319 |  |  | 0.570 |
| walker |  | 7344 | 16 | python method doc at src/pluggy/_manager.py:233 |  |  | 0.570 |
| walker |  | 7360 | 16 | python method doc at src/pluggy/_manager.py:315 |  |  | 0.570 |
| walker |  | 7393 | 33 | python method at src/pluggy/_manager.py:201 |  |  | 0.570 |
| walker |  | 7412 | 19 | python method doc at src/pluggy/_manager.py:430 |  |  | 0.570 |
| walker |  | 7425 | 13 | python method body at src/pluggy/_manager.py:76 body 77 |  |  | 0.570 |
| walker |  | 7438 | 13 | python method body at src/pluggy/_manager.py:425 body 428 |  |  | 0.570 |
| walker |  | 7452 | 14 | python method body at src/pluggy/_manager.py:315 body 317 |  |  | 0.570 |
| walker |  | 7466 | 14 | python method body at src/pluggy/_manager.py:430 body 432 |  |  | 0.570 |
| walker |  | 7481 | 15 | python method body at src/pluggy/_manager.py:319 body 321 |  |  | 0.570 |
| walker |  | 7508 | 27 | python method doc at src/pluggy/_manager.py:323 |  |  | 0.570 |
| walker |  | 7536 | 28 | python method doc at src/pluggy/_manager.py:242 |  |  | 0.570 |
| walker |  | 7564 | 28 | python method doc at src/pluggy/_manager.py:425 |  |  | 0.570 |
| walker |  | 7583 | 19 | python method body at src/pluggy/_manager.py:79 body 80 |  |  | 0.570 |
| walker |  | 7603 | 20 | python method body at src/pluggy/_manager.py:300 body 302 |  |  | 0.570 |
| ns | 7623 |  | 430 | test_hookcaller.py + test_multicall.py — every test fn name | 7.4 |  | 0.554 |
| walker |  | 7638 | 35 | python method doc at src/pluggy/_manager.py:487 |  |  | 0.554 |
| walker |  | 7660 | 22 | python method body at src/pluggy/_manager.py:71 body 73 |  |  | 0.554 |
| walker |  | 7702 | 42 | python method doc at src/pluggy/_manager.py:379 |  |  | 0.554 |
| walker |  | 7726 | 24 | python method body at src/pluggy/_manager.py:238 body 240 |  |  | 0.554 |
| walker |  | 7750 | 24 | python method body at src/pluggy/_manager.py:296 body 298 |  |  | 0.554 |
| walker |  | 7800 | 50 | python method doc at src/pluggy/_manager.py:512 |  |  | 0.554 |
| walker |  | 7856 | 56 | python method doc at src/pluggy/_manager.py:252 |  |  | 0.554 |
| walker |  | 7914 | 58 | python method doc at src/pluggy/_manager.py:434 |  |  | 0.554 |
| walker |  | 7945 | 31 | python method body at src/pluggy/_manager.py:59 body 60 |  |  | 0.554 |
| walker |  | 7953 | 8 | python method body at src/pluggy/_manager.py:242 body 250 |  |  | 0.554 |
| walker |  | 7961 | 8 | python method body at src/pluggy/_manager.py:323 body 329 |  |  | 0.554 |
| ns | 8023 |  | 400 | Smaller test files — every test fn name | 7.5 |  | 0.539 |
| walker |  | 8038 | 77 | python method doc at src/pluggy/_manager.py:201 |  |  | 0.539 |
| walker |  | 8128 | 90 | python method doc at src/pluggy/_manager.py:304 |  |  | 0.539 |
| walker |  | 8219 | 91 | python method doc at src/pluggy/_manager.py:395 |  |  | 0.539 |
| walker |  | 8229 | 10 | python method body at src/pluggy/_manager.py:233 body 235 |  |  | 0.539 |
| ns | 8324 |  | 301 | pyproject.toml — [project] essentials (skip classifier list) | 7.6 |  | 0.528 |
| walker |  | 8349 | 120 | python method doc at src/pluggy/_manager.py:125 |  |  | 0.528 |
| walker |  | 8472 | 123 | python method doc at src/pluggy/_manager.py:176 |  |  | 0.528 |
| walker |  | 8484 | 12 | python method body at src/pluggy/_manager.py:304 body 313 |  |  | 0.528 |
| ns | 8576 |  | 252 | pyproject.toml — [tool.ruff.lint] config | 7.7 | 7.6 | 0.521 |
| walker |  | 8620 | 136 | python method doc at src/pluggy/_manager.py:278 |  |  | 0.521 |
| walker |  | 8784 | 164 | python method doc at src/pluggy/_manager.py:450 |  |  | 0.521 |
| walker |  | 8799 | 15 | python method body at src/pluggy/_manager.py:233 body 236 |  |  | 0.521 |
| ns | 8859 |  | 283 | tox.ini — [tox] + [testenv] + [pytest] (skip release/docs envs) | 7.8 |  | 0.512 |
| walker |  | 8865 | 66 | python decl body at src/pluggy/_callers.py:60 body 64 |  |  | 0.512 |
| walker |  | 9026 | 161 | python imports in src/pluggy/_callers.py |  |  | 0.512 |
| walker |  | 9123 | 97 | python method at src/pluggy/_manager.py:114 |  |  | 0.512 |
| walker |  | 9145 | 22 | python method body at src/pluggy/_manager.py:114 body 123 |  |  | 0.512 |
| walker |  | 9220 | 75 | python decl body at src/pluggy/_manager.py:42 body 43 |  |  | 0.512 |
| ns | 9238 |  | 379 | CHANGELOG.rst — pluggy 1.6.0 entry only | 7.9 |  | 0.502 |
| ns | 9300 |  | 62 | changelog/ + downstream/ FS listings | 7.10 |  | 0.508 |
| walker |  | 9433 | 213 | python class body at src/pluggy/_hooks.py:593 |  |  | 0.508 |
| walker |  | 9452 | 19 | python method body at src/pluggy/_manager.py:304 body 312 |  |  | 0.508 |
| walker |  | 9460 | 8 | python method body at src/pluggy/_manager.py:252 body 258 |  |  | 0.508 |
| walker |  | 9468 | 8 | python method body at src/pluggy/_manager.py:278 body 294 |  |  | 0.508 |
| walker |  | 9480 | 12 | python decl names surface in scripts/towncrier-draft-to-file.py |  |  | 0.508 |
| walker |  | 9480 | 0 | python decl at scripts/towncrier-draft-to-file.py:5 |  |  | 0.508 |
| walker |  | 9688 | 208 | python imports in src/pluggy/_hooks.py |  |  | 0.508 |
| walker |  | 9789 | 101 | python method body at src/pluggy/_hooks.py:709 body 710 |  |  | 0.508 |
| ns | 9819 |  | 519 | _callers._multicall body — the actual call loop | 7.11 | 4.3 | 0.495 |
| walker |  | 9893 | 104 | python method body at src/pluggy/_hooks.py:424 body 429 |  |  | 0.495 |
| walker |  | 9944 | 51 | python method body at src/pluggy/_hooks.py:618 body 620 |  |  | 0.495 |
| ns | 9946 |  | 127 | Top-level meta — AGENTS.md, SECURITY.md, MANIFEST.in | 7.12 |  | 0.491 |
| ns | 9964 |  | 18 | scripts/ FS + .github/workflows/ FS | 7.13 |  | 0.492 |
| ns | 9996 |  | 32 | docs/ root FS + docs/requirements.txt | 7.14 |  | 0.496 |

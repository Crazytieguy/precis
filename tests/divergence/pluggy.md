Score(3000)=0.840 I=0.921 C=0.766 ns_rows≤3K=16/40 (reached=10 partial=1 missing=5)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 28 |  | 28 | README title + tagline | 1.1 |  | 0.000 |
| walker |  | 96 | 96 | listing of '.' |  |  | 0.000 |
| walker |  | 100 | 4 | listing of 'src' |  |  | 0.000 |
| ns | 114 |  | 86 | README badges line + who depends on pluggy + doc pointer | 1.2 |  | 0.000 |
| walker |  | 119 | 19 | listing of 'changelog' |  |  | 0.000 |
| ns | 200 |  | 86 | Directory listing: docs/ and its worked-example subtree | 1.3 |  | 0.000 |
| walker |  | 238 | 119 | README headline in README.rst |  |  | 0.484 |
| ns | 293 |  | 93 | Directory listing: changelog, CI, downstream smoke-tests, scripts | 1.4 |  | 0.360 |
| ns | 445 |  | 152 | pluggy/__init__.py: __all__ (the literal public export list) | 1.5 |  | 0.298 |
| ns | 646 |  | 201 | Directory listing: repo root, src/pluggy, testing | 1.6 |  | 0.350 |
| walker |  | 784 | 546 | README.rst section #0 |  |  | 0.371 |
| walker |  | 788 | 4 | listing of '.claude' |  |  | 0.371 |
| walker |  | 820 | 32 | listing of 'docs' |  |  | 0.406 |
| walker |  | 823 | 3 | listing of 'docs/_static' |  |  | 0.406 |
| walker |  | 827 | 4 | listing of 'docs/_static/img' |  |  | 0.406 |
| walker |  | 870 | 43 | listing of 'src/pluggy' |  |  | 0.492 |
| ns | 904 |  | 258 | pluggy/__init__.py: re-export imports + dynamic __version__ | 1.7 | 1.5 | 0.439 |
| walker |  | 1188 | 318 | python imports in src/pluggy/__init__.py |  |  | 0.599 |
| walker |  | 1204 | 16 | python decl names surface in src/pluggy/__init__.py |  |  | 0.607 |
| walker |  | 1204 | 0 | python decl at src/pluggy/__init__.py:32 |  |  | 0.607 |
| ns | 1237 |  | 333 | README definitive example, part 1: markers + spec/impl namespaces | 1.8 |  | 0.656 |
| walker |  | 1247 | 43 | listing of 'downstream' |  |  | 0.699 |
| walker |  | 1260 | 13 | listing of '.github' |  |  | 0.725 |
| walker |  | 1264 | 4 | listing of '.github/workflows' |  |  | 0.735 |
| walker |  | 1278 | 14 | listing of 'scripts' |  |  | 0.756 |
| walker |  | 1331 | 53 | [package] in pyproject.toml |  |  | 0.756 |
| walker |  | 1355 | 24 | python decl names surface in src/pluggy/_warnings.py |  |  | 0.756 |
| walker |  | 1355 | 0 | python decl at src/pluggy/_warnings.py:4 |  |  | 0.756 |
| walker |  | 1371 | 16 | python decl at src/pluggy/_warnings.py:10 |  |  | 0.756 |
| walker |  | 1384 | 13 | python class body at src/pluggy/_warnings.py:4 |  |  | 0.756 |
| walker |  | 1397 | 13 | python class body at src/pluggy/_warnings.py:10 |  |  | 0.756 |
| walker |  | 1411 | 14 | python decl doc at src/pluggy/_warnings.py:4 |  |  | 0.756 |
| walker |  | 1418 | 7 | python imports in src/pluggy/_warnings.py |  |  | 0.756 |
| walker |  | 1433 | 15 | listing of 'docs/examples' |  |  | 0.782 |
| walker |  | 1441 | 8 | listing of 'docs/examples/eggsample' |  |  | 0.803 |
| ns | 1445 |  | 208 | README definitive example, part 2: manager wiring + call + real output | 1.9 | 1.8 | 0.810 |
| ns | 1497 |  | 52 | _callers.py: every def location (signatures only) | 2.1 |  | 0.800 |
| walker |  | 1503 | 62 | listing of 'testing' |  |  | 0.872 |
| walker |  | 1523 | 20 | listing of 'docs/examples/eggsample/eggsample' |  |  | 0.919 |
| walker |  | 1570 | 47 | plaintext config docs/requirements.txt |  |  | 0.919 |
| walker |  | 1636 | 66 | python decl body at src/pluggy/__init__.py:32 body 33 |  |  | 0.953 |
| walker |  | 1717 | 81 | python decl names surface in src/pluggy/_result.py |  |  | 0.954 |
| walker |  | 1717 | 0 | python decl at src/pluggy/_result.py:20 |  |  | 0.954 |
| walker |  | 1728 | 11 | python decl at src/pluggy/_result.py:24 |  |  | 0.954 |
| walker |  | 1737 | 9 | python decl doc at src/pluggy/_result.py:20 |  |  | 0.954 |
| walker |  | 1759 | 22 | python class body at src/pluggy/_result.py:24 |  |  | 0.954 |
| walker |  | 1791 | 32 | python decl doc at src/pluggy/_result.py:24 |  |  | 0.954 |
| ns | 1815 |  | 318 | _result.py / _tracing.py / _warnings.py: every class/def location | 2.2 |  | 0.886 |
| walker |  | 1918 | 127 | python method sigs in src/pluggy/_result.py |  |  | 0.905 |
| walker |  | 1918 | 0 | python method at src/pluggy/_result.py:67 |  |  | 0.905 |
| walker |  | 1918 | 0 | python method at src/pluggy/_result.py:80 |  |  | 0.905 |
| walker |  | 1918 | 0 | python method at src/pluggy/_result.py:91 |  |  | 0.905 |
| walker |  | 1926 | 8 | python method at src/pluggy/_result.py:42 |  |  | 0.907 |
| walker |  | 1934 | 8 | python method at src/pluggy/_result.py:51 |  |  | 0.910 |
| walker |  | 1942 | 8 | python method at src/pluggy/_result.py:56 |  |  | 0.913 |
| ns | 1951 |  | 136 | _hooks.py: every class/def location, part 1 (markers + helpers) | 2.3 |  | 0.884 |
| walker |  | 1954 | 12 | python method doc at src/pluggy/_result.py:42 |  |  | 0.884 |
| walker |  | 1966 | 12 | python method doc at src/pluggy/_result.py:51 |  |  | 0.884 |
| walker |  | 1978 | 12 | python method doc at src/pluggy/_result.py:56 |  |  | 0.884 |
| walker |  | 2018 | 40 | python method at src/pluggy/_result.py:31 |  |  | 0.884 |
| walker |  | 2030 | 12 | python method doc at src/pluggy/_result.py:31 |  |  | 0.884 |
| ns | 2144 |  | 193 | _hooks.py: every class/def location, part 2 (HookRelay + the full HookCaller method list) | 2.4 |  | 0.849 |
| walker |  | 2150 | 120 | python decl names surface in src/pluggy/_manager.py |  |  | 0.849 |
| walker |  | 2150 | 0 | python decl at src/pluggy/_manager.py:42 |  |  | 0.849 |
| walker |  | 2150 | 0 | python decl at src/pluggy/_manager.py:52 |  |  | 0.849 |
| walker |  | 2150 | 0 | python decl at src/pluggy/_manager.py:65 |  |  | 0.849 |
| walker |  | 2150 | 0 | python decl at src/pluggy/_manager.py:83 |  |  | 0.849 |
| walker |  | 2150 | 0 | python decl at src/pluggy/_manager.py:525 |  |  | 0.849 |
| walker |  | 2163 | 13 | python decl doc at src/pluggy/_manager.py:65 |  |  | 0.849 |
| walker |  | 2211 | 48 | python decl doc at src/pluggy/_manager.py:52 |  |  | 0.849 |
| walker |  | 2237 | 26 | python decl at src/pluggy/_manager.py:37 |  |  | 0.849 |
| walker |  | 2253 | 16 | python decl body at src/pluggy/_manager.py:525 body 526 |  |  | 0.849 |
| ns | 2306 |  | 162 | _hooks.py: every class/def location, part 3 (_SubsetHookCaller/HookImpl/HookSpec) | 2.5 |  | 0.821 |
| ns | 2609 |  | 303 | _manager.py: every class/def location, part 1 (registration + introspection) | 2.6 |  | 0.777 |
| ns | 2734 |  | 125 | _manager.py: every class/def location, part 2 (validation, pending, plugins, tracing) | 2.7 |  | 0.761 |
| walker |  | 2868 | 615 | python method sigs in src/pluggy/_manager.py |  |  | 0.837 |
| walker |  | 2868 | 0 | python method at src/pluggy/_manager.py:59 |  |  | 0.837 |
| walker |  | 2868 | 0 | python method at src/pluggy/_manager.py:68 |  |  | 0.837 |
| walker |  | 2868 | 0 | python method at src/pluggy/_manager.py:76 |  |  | 0.837 |
| walker |  | 2868 | 0 | python method at src/pluggy/_manager.py:79 |  |  | 0.837 |
| walker |  | 2868 | 0 | python method at src/pluggy/_manager.py:125 |  |  | 0.837 |
| walker |  | 2868 | 0 | python method at src/pluggy/_manager.py:176 |  |  | 0.837 |
| walker |  | 2868 | 0 | python method at src/pluggy/_manager.py:233 |  |  | 0.837 |
| walker |  | 2868 | 0 | python method at src/pluggy/_manager.py:238 |  |  | 0.837 |
| walker |  | 2868 | 0 | python method at src/pluggy/_manager.py:242 |  |  | 0.837 |
| walker |  | 2868 | 0 | python method at src/pluggy/_manager.py:252 |  |  | 0.837 |
| walker |  | 2868 | 0 | python method at src/pluggy/_manager.py:296 |  |  | 0.837 |
| walker |  | 2868 | 0 | python method at src/pluggy/_manager.py:300 |  |  | 0.837 |
| walker |  | 2868 | 0 | python method at src/pluggy/_manager.py:304 |  |  | 0.837 |
| walker |  | 2868 | 0 | python method at src/pluggy/_manager.py:315 |  |  | 0.837 |
| walker |  | 2868 | 0 | python method at src/pluggy/_manager.py:319 |  |  | 0.837 |
| walker |  | 2868 | 0 | python method at src/pluggy/_manager.py:323 |  |  | 0.837 |
| walker |  | 2868 | 0 | python method at src/pluggy/_manager.py:331 |  |  | 0.837 |
| walker |  | 2868 | 0 | python method at src/pluggy/_manager.py:379 |  |  | 0.837 |
| walker |  | 2868 | 0 | python method at src/pluggy/_manager.py:395 |  |  | 0.837 |
| walker |  | 2868 | 0 | python method at src/pluggy/_manager.py:425 |  |  | 0.837 |
| walker |  | 2868 | 0 | python method at src/pluggy/_manager.py:430 |  |  | 0.837 |
| walker |  | 2868 | 0 | python method at src/pluggy/_manager.py:434 |  |  | 0.837 |
| walker |  | 2868 | 0 | python method at src/pluggy/_manager.py:487 |  |  | 0.837 |
| walker |  | 2876 | 8 | python method at src/pluggy/_manager.py:71 |  |  | 0.840 |
| walker |  | 2886 | 10 | python method at src/pluggy/_manager.py:100 |  |  | 0.840 |
| walker |  | 2914 | 28 | python method at src/pluggy/_manager.py:512 |  |  | 0.840 |
| walker |  | 2928 | 14 | python method doc at src/pluggy/_manager.py:300 |  |  | 0.840 |
| walker |  | 2959 | 31 | python method at src/pluggy/_manager.py:278 |  |  | 0.840 |
| walker |  | 2990 | 31 | python method at src/pluggy/_manager.py:450 |  |  | 0.840 |
| walker |  | 3005 | 15 | python method doc at src/pluggy/_manager.py:238 |  |  | 0.840 |
| walker |  | 3020 | 15 | python method doc at src/pluggy/_manager.py:296 |  |  | 0.840 |
| walker |  | 3055 | 35 | python method at src/pluggy/_manager.py:201 |  |  | 0.840 |
| walker |  | 3072 | 17 | python method doc at src/pluggy/_manager.py:319 |  |  | 0.840 |
| walker |  | 3090 | 18 | python method doc at src/pluggy/_manager.py:233 |  |  | 0.840 |
| walker |  | 3108 | 18 | python method doc at src/pluggy/_manager.py:315 |  |  | 0.840 |
| walker |  | 3129 | 21 | python method doc at src/pluggy/_manager.py:430 |  |  | 0.840 |
| walker |  | 3158 | 29 | python method doc at src/pluggy/_manager.py:323 |  |  | 0.840 |
| walker |  | 3188 | 30 | python method doc at src/pluggy/_manager.py:425 |  |  | 0.840 |
| walker |  | 3223 | 35 | python method doc at src/pluggy/_manager.py:242 |  |  | 0.840 |
| ns | 3382 |  | 648 | HookspecMarker.__call__: full option docs (firstresult/historic/warn_on_impl) | 3.1 | 2.3 | 0.764 |
| walker |  | 3568 | 345 | python decl names surface in src/pluggy/_hooks.py |  |  | 0.772 |
| walker |  | 3568 | 0 | python decl at src/pluggy/_hooks.py:40 |  |  | 0.772 |
| walker |  | 3568 | 0 | python decl at src/pluggy/_hooks.py:56 |  |  | 0.772 |
| walker |  | 3568 | 0 | python decl at src/pluggy/_hooks.py:281 |  |  | 0.772 |
| walker |  | 3568 | 0 | python decl at src/pluggy/_hooks.py:293 |  |  | 0.772 |
| walker |  | 3568 | 0 | python decl at src/pluggy/_hooks.py:382 |  |  | 0.772 |
| walker |  | 3568 | 0 | python decl at src/pluggy/_hooks.py:593 |  |  | 0.772 |
| walker |  | 3576 | 8 | python decl at src/pluggy/_hooks.py:358 |  |  | 0.772 |
| walker |  | 3584 | 8 | python decl at src/pluggy/_hooks.py:638 |  |  | 0.773 |
| walker |  | 3592 | 8 | python decl at src/pluggy/_hooks.py:696 |  |  | 0.774 |
| walker |  | 3601 | 9 | python decl at src/pluggy/_hooks.py:77 |  |  | 0.777 |
| walker |  | 3610 | 9 | python decl at src/pluggy/_hooks.py:164 |  |  | 0.779 |
| walker |  | 3622 | 12 | python decl doc at src/pluggy/_hooks.py:40 |  |  | 0.779 |
| walker |  | 3634 | 12 | python decl doc at src/pluggy/_hooks.py:56 |  |  | 0.779 |
| walker |  | 3650 | 16 | python class body at src/pluggy/_hooks.py:77 |  |  | 0.779 |
| walker |  | 3666 | 16 | python class body at src/pluggy/_hooks.py:164 |  |  | 0.779 |
| walker |  | 3683 | 17 | python decl doc at src/pluggy/_hooks.py:382 |  |  | 0.781 |
| walker |  | 3701 | 18 | python decl doc at src/pluggy/_hooks.py:638 |  |  | 0.781 |
| walker |  | 3732 | 31 | python decl doc at src/pluggy/_hooks.py:358 |  |  | 0.781 |
| ns | 3795 |  | 413 | HookCaller: hookimpl call-order algorithm (_add_hookimpl + its ordering comment) | 3.2 | 2.4 | 0.743 |
| walker |  | 3844 | 112 | python class body at src/pluggy/_hooks.py:638 |  |  | 0.743 |
| walker |  | 3896 | 52 | python class body at src/pluggy/_hooks.py:358 |  |  | 0.743 |
| walker |  | 3984 | 88 | python class body at src/pluggy/_hooks.py:696 |  |  | 0.743 |
| walker |  | 4076 | 92 | python class body at src/pluggy/_hooks.py:382 |  |  | 0.743 |
| walker |  | 4100 | 24 | python decl at src/pluggy/_hooks.py:377 |  |  | 0.743 |
| walker |  | 4640 | 540 | python method sigs in src/pluggy/_hooks.py |  |  | 0.798 |
| walker |  | 4640 | 0 | python method at src/pluggy/_hooks.py:88 |  |  | 0.798 |
| walker |  | 4640 | 0 | python method at src/pluggy/_hooks.py:175 |  |  | 0.798 |
| walker |  | 4640 | 0 | python method at src/pluggy/_hooks.py:365 |  |  | 0.798 |
| walker |  | 4640 | 0 | python method at src/pluggy/_hooks.py:420 |  |  | 0.798 |
| walker |  | 4640 | 0 | python method at src/pluggy/_hooks.py:438 |  |  | 0.798 |
| walker |  | 4640 | 0 | python method at src/pluggy/_hooks.py:442 |  |  | 0.798 |
| walker |  | 4640 | 0 | python method at src/pluggy/_hooks.py:449 |  |  | 0.798 |
| walker |  | 4640 | 0 | python method at src/pluggy/_hooks.py:453 |  |  | 0.798 |
| walker |  | 4640 | 0 | python method at src/pluggy/_hooks.py:477 |  |  | 0.798 |
| walker |  | 4640 | 0 | python method at src/pluggy/_hooks.py:499 |  |  | 0.798 |
| walker |  | 4640 | 0 | python method at src/pluggy/_hooks.py:577 |  |  | 0.798 |
| walker |  | 4640 | 0 | python method at src/pluggy/_hooks.py:612 |  |  | 0.798 |
| walker |  | 4640 | 0 | python method at src/pluggy/_hooks.py:634 |  |  | 0.798 |
| walker |  | 4640 | 0 | python method at src/pluggy/_hooks.py:692 |  |  | 0.798 |
| walker |  | 4640 | 0 | python method at src/pluggy/_hooks.py:709 |  |  | 0.798 |
| walker |  | 4648 | 8 | python method at src/pluggy/_hooks.py:626 |  |  | 0.798 |
| walker |  | 4656 | 8 | python method at src/pluggy/_hooks.py:630 |  |  | 0.798 |
| walker |  | 4666 | 10 | python method doc at src/pluggy/_hooks.py:365 |  |  | 0.798 |
| walker |  | 4697 | 31 | python method at src/pluggy/_hooks.py:543 |  |  | 0.798 |
| walker |  | 4712 | 15 | python method doc at src/pluggy/_hooks.py:449 |  |  | 0.798 |
| walker |  | 4727 | 15 | python method at src/pluggy/_hooks.py:618 |  |  | 0.798 |
| walker |  | 4745 | 18 | python method doc at src/pluggy/_hooks.py:438 |  |  | 0.798 |
| walker |  | 4786 | 41 | python method at src/pluggy/_hooks.py:424 |  |  | 0.798 |
| walker |  | 4803 | 17 | python method at src/pluggy/_hooks.py:480 |  |  | 0.798 |
| ns | 4819 |  | 1024 | HookimplMarker.__call__: full option docs (wrapper/hookwrapper/optionalhook/tryfirst/trylast/specname) | 3.3 | 2.3 | 0.715 |
| walker |  | 4855 | 52 | python method at src/pluggy/_hooks.py:516 |  |  | 0.715 |
| walker |  | 4869 | 14 | python method doc at src/pluggy/_hooks.py:453 |  |  | 0.716 |
| walker |  | 5093 | 224 | python class body at src/pluggy/_hooks.py:56 |  |  | 0.716 |
| walker |  | 5255 | 162 | python class body at src/pluggy/_hooks.py:40 |  |  | 0.716 |
| walker |  | 5287 | 32 | python decl doc at src/pluggy/_hooks.py:593 |  |  | 0.723 |
| walker |  | 5350 | 63 | python method at src/pluggy/_hooks.py:656 |  |  | 0.723 |
| walker |  | 5362 | 12 | python method doc at src/pluggy/_hooks.py:656 |  |  | 0.723 |
| walker |  | 5442 | 80 | python decl doc at src/pluggy/_hooks.py:164 |  |  | 0.723 |
| walker |  | 5523 | 81 | python decl doc at src/pluggy/_hooks.py:77 |  |  | 0.723 |
| walker |  | 5557 | 34 | python decl at src/pluggy/_hooks.py:33 |  |  | 0.723 |
| walker |  | 5627 | 70 | python method at src/pluggy/_hooks.py:393 |  |  | 0.723 |
| walker |  | 5639 | 12 | python method doc at src/pluggy/_hooks.py:393 |  |  | 0.723 |
| walker |  | 5681 | 42 | python method doc at src/pluggy/_manager.py:487 |  |  | 0.723 |
| walker |  | 5725 | 44 | python method doc at src/pluggy/_manager.py:379 |  |  | 0.723 |
| walker |  | 5746 | 21 | python method doc at src/pluggy/_hooks.py:577 |  |  | 0.723 |
| walker |  | 5791 | 45 | python method doc at src/pluggy/_hooks.py:543 |  |  | 0.723 |
| ns | 5840 |  | 1021 | _multicall(): full body -- the actual call loop | 3.4 | 2.1 | 0.648 |
| walker |  | 5870 | 79 | python decl doc at src/pluggy/_hooks.py:293 |  |  | 0.648 |
| walker |  | 5880 | 10 | python imports in docs/examples/eggsample/eggsample/__init__.py |  |  | 0.648 |
| walker |  | 5930 | 50 | python method doc at src/pluggy/_manager.py:512 |  |  | 0.648 |
| walker |  | 6014 | 84 | python method at src/pluggy/_hooks.py:91 |  |  | 0.648 |
| walker |  | 6105 | 91 | python method at src/pluggy/_hooks.py:101 |  |  | 0.648 |
| ns | 6113 |  | 273 | PluggyWarning / PluggyTeardownRaisedWarning: full body | 3.5 | 2.2 | 0.633 |
| walker |  | 6166 | 61 | python method doc at src/pluggy/_result.py:80 |  |  | 0.633 |
| walker |  | 6263 | 97 | python method at src/pluggy/_hooks.py:111 |  |  | 0.634 |
| walker |  | 6326 | 63 | python method doc at src/pluggy/_manager.py:252 |  |  | 0.634 |
| walker |  | 6389 | 63 | python method doc at src/pluggy/_result.py:91 |  |  | 0.634 |
| walker |  | 6488 | 99 | python method at src/pluggy/_hooks.py:178 |  |  | 0.634 |
| ns | 6545 |  | 432 | Test roster: test_pluginmanager.py (32 tests) | 4.1 |  | 0.615 |
| walker |  | 6553 | 65 | python method doc at src/pluggy/_manager.py:434 |  |  | 0.615 |
| walker |  | 6659 | 106 | python method at src/pluggy/_hooks.py:190 |  |  | 0.615 |
| ns | 6694 |  | 149 | Test roster: test_details.py (10 tests) | 4.2 |  | 0.609 |
| walker |  | 6771 | 112 | python method at src/pluggy/_hooks.py:202 |  |  | 0.610 |
| walker |  | 6838 | 67 | python decl names surface in src/pluggy/_tracing.py |  |  | 0.613 |
| walker |  | 6838 | 0 | python decl at src/pluggy/_tracing.py:16 |  |  | 0.613 |
| walker |  | 6838 | 0 | python decl at src/pluggy/_tracing.py:59 |  |  | 0.613 |
| ns | 6913 |  | 219 | Test roster: test_hookcaller.py (15 tests) | 4.3 |  | 0.605 |
| walker |  | 7048 | 210 | python method sigs in src/pluggy/_tracing.py |  |  | 0.623 |
| walker |  | 7048 | 0 | python method at src/pluggy/_tracing.py:17 |  |  | 0.623 |
| walker |  | 7048 | 0 | python method at src/pluggy/_tracing.py:22 |  |  | 0.623 |
| walker |  | 7048 | 0 | python method at src/pluggy/_tracing.py:25 |  |  | 0.623 |
| walker |  | 7048 | 0 | python method at src/pluggy/_tracing.py:42 |  |  | 0.623 |
| walker |  | 7048 | 0 | python method at src/pluggy/_tracing.py:48 |  |  | 0.623 |
| walker |  | 7048 | 0 | python method at src/pluggy/_tracing.py:51 |  |  | 0.623 |
| walker |  | 7048 | 0 | python method at src/pluggy/_tracing.py:60 |  |  | 0.623 |
| walker |  | 7048 | 0 | python method at src/pluggy/_tracing.py:64 |  |  | 0.623 |
| walker |  | 7048 | 0 | python method at src/pluggy/_tracing.py:67 |  |  | 0.623 |
| walker |  | 7057 | 9 | python method body at src/pluggy/_tracing.py:48 body 49 |  |  | 0.623 |
| walker |  | 7069 | 12 | python method body at src/pluggy/_tracing.py:22 body 23 |  |  | 0.623 |
| walker |  | 7082 | 13 | python method body at src/pluggy/_tracing.py:64 body 65 |  |  | 0.623 |
| ns | 7085 |  | 172 | Test roster: test_invocations.py (13 tests) | 4.4 |  | 0.616 |
| walker |  | 7159 | 77 | python method doc at src/pluggy/_hooks.py:499 |  |  | 0.616 |
| walker |  | 7176 | 17 | python method body at src/pluggy/_tracing.py:67 body 68 |  |  | 0.616 |
| walker |  | 7263 | 87 | python method doc at src/pluggy/_manager.py:201 |  |  | 0.616 |
| walker |  | 7343 | 80 | python decl names surface in src/pluggy/_callers.py |  |  | 0.621 |
| ns | 7372 |  | 287 | Test roster: test_multicall.py (21 tests) | 4.5 |  | 0.610 |
| walker |  | 7374 | 31 | python decl at src/pluggy/_callers.py:27 |  |  | 0.610 |
| walker |  | 7405 | 31 | python decl doc at src/pluggy/_callers.py:27 |  |  | 0.610 |
| walker |  | 7434 | 29 | python decl at src/pluggy/_callers.py:70 |  |  | 0.610 |
| walker |  | 7468 | 34 | python decl at src/pluggy/_callers.py:60 |  |  | 0.610 |
| ns | 7620 |  | 248 | testing/conftest.py: full body (shared pm / he_pm fixtures) | 4.6 |  | 0.597 |
| walker |  | 7657 | 189 | python decl doc at src/pluggy/_manager.py:83 |  |  | 0.597 |
| walker |  | 7846 | 189 | python decl doc at src/pluggy/_warnings.py:10 |  |  | 0.618 |
| ns | 7848 |  | 228 | docs/api_reference.rst: autodoc roster (which classes are documented, in order) | 5.1 |  | 0.610 |
| walker |  | 7943 | 97 | python method doc at src/pluggy/_manager.py:304 |  |  | 0.610 |
| walker |  | 8041 | 98 | python method doc at src/pluggy/_hooks.py:516 |  |  | 0.610 |
| walker |  | 8140 | 99 | python method doc at src/pluggy/_result.py:67 |  |  | 0.610 |
| ns | 8153 |  | 305 | docs/index.rst: 'How does it work?' overview | 5.2 |  | 0.601 |
| walker |  | 8243 | 103 | python method doc at src/pluggy/_manager.py:395 |  |  | 0.601 |
| ns | 8302 |  | 149 | pyproject.toml: every distinct [section] header (config-key roster) | 6.1 |  | 0.595 |
| walker |  | 8459 | 216 | python class body at src/pluggy/_hooks.py:593 |  |  | 0.595 |
| ns | 8496 |  | 194 | tox.ini: [tox] envlist + [testenv] | 6.2 |  | 0.588 |
| walker |  | 8518 | 59 | python decl at src/pluggy/_callers.py:82 |  |  | 0.589 |
| walker |  | 8536 | 18 | python method body at src/pluggy/_tracing.py:60 body 61 |  |  | 0.589 |
| walker |  | 8645 | 109 | python imports in src/pluggy/_result.py |  |  | 0.589 |
| ns | 8674 |  | 178 | tox.ini: [testenv:docs] + [pytest] | 6.3 | 6.2 | 0.582 |
| walker |  | 8780 | 135 | python method doc at src/pluggy/_manager.py:176 |  |  | 0.582 |
| ns | 8786 |  | 112 | tox.ini: [testenv:release] | 6.4 | 6.2 | 0.578 |
| walker |  | 8836 | 56 | python decl doc at src/pluggy/_callers.py:82 |  |  | 0.578 |
| walker |  | 8973 | 137 | python method doc at src/pluggy/_manager.py:125 |  |  | 0.578 |
| ns | 8988 |  | 202 | .pre-commit-config.yaml: repo + hook-id roster | 6.5 |  | 0.573 |
| ns | 9039 |  | 51 | .coveragerc: [run] section (what coverage.py tracks) | 6.6 |  | 0.570 |
| ns | 9102 |  | 63 | MANIFEST.in: full (sdist inclusion rules) | 6.7 |  | 0.567 |
| ns | 9212 |  | 110 | CHANGELOG.rst: pluggy 1.6.0 -- version header + deprecations | 7.1 |  | 0.563 |
| ns | 9443 |  | 231 | CHANGELOG.rst: pluggy 1.6.0 -- bug fixes | 7.2 | 7.1 | 0.558 |
| ns | 9646 |  | 203 | changelog/README.rst: newsfragment type taxonomy + naming convention | 7.3 |  | 0.554 |
| ns | 9971 |  | 325 | RELEASING.rst: full release procedure | 7.4 |  | 0.544 |

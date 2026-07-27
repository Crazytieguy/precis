Score(3000)=0.762 I=0.902 C=0.644 ns_rows≤3K=16/40 (reached=8 partial=1 missing=7)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 28 |  | 28 | README title + tagline | 1.1 |  | 0.000 |
| walker |  | 104 | 104 | listing of '.' |  |  | 0.000 |
| walker |  | 108 | 4 | listing of 'src' |  |  | 0.000 |
| ns | 114 |  | 86 | README badges line + who depends on pluggy + doc pointer | 1.2 |  | 0.000 |
| walker |  | 126 | 18 | listing of 'changelog' |  |  | 0.000 |
| walker |  | 129 | 3 | listing of '.claude' |  |  | 0.000 |
| walker |  | 162 | 33 | listing of 'docs' |  |  | 0.000 |
| walker |  | 165 | 3 | listing of 'docs/_static' |  |  | 0.000 |
| walker |  | 168 | 3 | listing of 'docs/_static/img' |  |  | 0.000 |
| ns | 205 |  | 91 | Directory listing: docs/ and its worked-example subtree | 1.3 |  | 0.110 |
| walker |  | 210 | 42 | listing of 'downstream' |  |  | 0.134 |
| walker |  | 252 | 42 | listing of 'src/pluggy' |  |  | 0.146 |
| ns | 299 |  | 94 | Directory listing: changelog, CI, downstream smoke-tests, scripts | 1.4 |  | 0.210 |
| ns | 451 |  | 152 | pluggy/__init__.py: __all__ (the literal public export list) | 1.5 |  | 0.174 |
| walker |  | 570 | 318 | python imports in src/pluggy/__init__.py |  |  | 0.335 |
| walker |  | 586 | 16 | python decl names surface in src/pluggy/__init__.py |  |  | 0.336 |
| walker |  | 586 | 0 | python decl at src/pluggy/__init__.py:32 |  |  | 0.336 |
| ns | 655 |  | 204 | Directory listing: repo root, src/pluggy, testing | 1.6 |  | 0.335 |
| walker |  | 705 | 119 | README headline in README.rst |  |  | 0.654 |
| ns | 913 |  | 258 | pluggy/__init__.py: re-export imports + dynamic __version__ | 1.7 | 1.5 | 0.631 |
| ns | 1246 |  | 333 | README definitive example, part 1: markers + spec/impl namespaces | 1.8 |  | 0.548 |
| walker |  | 1251 | 546 | README.rst section #0 |  |  | 0.699 |
| walker |  | 1264 | 13 | listing of '.github' |  |  | 0.725 |
| walker |  | 1267 | 3 | listing of '.github/workflows' |  |  | 0.735 |
| walker |  | 1280 | 13 | listing of 'scripts' |  |  | 0.756 |
| walker |  | 1306 | 26 | downstream/README.md section #0 |  |  | 0.756 |
| walker |  | 1349 | 43 | tool.setuptools_scm+uv config in pyproject.toml |  |  | 0.756 |
| walker |  | 1400 | 51 | tool.setuptools config in pyproject.toml |  |  | 0.756 |
| walker |  | 1451 | 51 | [package] in pyproject.toml |  |  | 0.756 |
| ns | 1454 |  | 208 | README definitive example, part 2: manager wiring + call + real output | 1.9 | 1.8 | 0.765 |
| ns | 1506 |  | 52 | _callers.py: every def location (signatures only) | 2.1 |  | 0.755 |
| walker |  | 1514 | 63 | manifest config in pyproject.toml |  |  | 0.755 |
| walker |  | 1530 | 16 | listing of 'docs/examples' |  |  | 0.780 |
| walker |  | 1538 | 8 | listing of 'docs/examples/eggsample' |  |  | 0.799 |
| walker |  | 1549 | 11 | python imports in src/pluggy/_warnings.py |  |  | 0.799 |
| walker |  | 1568 | 19 | listing of 'docs/examples/eggsample/eggsample' |  |  | 0.846 |
| walker |  | 1629 | 61 | listing of 'testing' |  |  | 0.919 |
| walker |  | 1649 | 20 | python decl names surface in src/pluggy/_warnings.py |  |  | 0.919 |
| walker |  | 1649 | 0 | python decl at src/pluggy/_warnings.py:4 |  |  | 0.919 |
| walker |  | 1665 | 16 | python decl at src/pluggy/_warnings.py:10 |  |  | 0.919 |
| walker |  | 1681 | 16 | python decl doc at src/pluggy/_warnings.py:4 |  |  | 0.919 |
| walker |  | 1692 | 11 | python class body at src/pluggy/_warnings.py:4 |  |  | 0.919 |
| walker |  | 1705 | 13 | python class body at src/pluggy/_warnings.py:10 |  |  | 0.920 |
| walker |  | 1752 | 47 | plaintext config docs/requirements.txt |  |  | 0.920 |
| walker |  | 1818 | 66 | python decl body at src/pluggy/__init__.py:32 body 33 |  |  | 0.954 |
| ns | 1824 |  | 318 | _result.py / _tracing.py / _warnings.py: every class/def location | 2.2 |  | 0.883 |
| ns | 1960 |  | 136 | _hooks.py: every class/def location, part 1 (markers + helpers) | 2.3 |  | 0.855 |
| walker |  | 2054 | 236 | tool.mypy config in pyproject.toml |  |  | 0.855 |
| ns | 2153 |  | 193 | _hooks.py: every class/def location, part 2 (HookRelay + the full HookCaller method list) | 2.4 |  | 0.821 |
| walker |  | 2304 | 250 | tool.ruff config in pyproject.toml |  |  | 0.822 |
| ns | 2315 |  | 162 | _hooks.py: every class/def location, part 3 (_SubsetHookCaller/HookImpl/HookSpec) | 2.5 |  | 0.795 |
| walker |  | 2385 | 81 | python decl names surface in src/pluggy/_result.py |  |  | 0.796 |
| walker |  | 2385 | 0 | python decl at src/pluggy/_result.py:20 |  |  | 0.796 |
| walker |  | 2396 | 11 | python decl at src/pluggy/_result.py:24 |  |  | 0.797 |
| walker |  | 2405 | 9 | python decl doc at src/pluggy/_result.py:20 |  |  | 0.797 |
| walker |  | 2439 | 34 | python decl doc at src/pluggy/_result.py:24 |  |  | 0.797 |
| walker |  | 2568 | 129 | python method sigs in src/pluggy/_result.py |  |  | 0.814 |
| walker |  | 2568 | 0 | python method at src/pluggy/_result.py:67 |  |  | 0.814 |
| walker |  | 2568 | 0 | python method at src/pluggy/_result.py:80 |  |  | 0.814 |
| walker |  | 2568 | 0 | python method at src/pluggy/_result.py:91 |  |  | 0.814 |
| walker |  | 2576 | 8 | python method at src/pluggy/_result.py:42 |  |  | 0.816 |
| walker |  | 2584 | 8 | python method at src/pluggy/_result.py:51 |  |  | 0.819 |
| walker |  | 2592 | 8 | python method at src/pluggy/_result.py:56 |  |  | 0.822 |
| walker |  | 2604 | 12 | python method doc at src/pluggy/_result.py:42 |  |  | 0.822 |
| walker |  | 2616 | 12 | python method doc at src/pluggy/_result.py:51 |  |  | 0.822 |
| ns | 2618 |  | 303 | _manager.py: every class/def location, part 1 (registration + introspection) | 2.6 |  | 0.776 |
| walker |  | 2628 | 12 | python method doc at src/pluggy/_result.py:56 |  |  | 0.776 |
| walker |  | 2668 | 40 | python method at src/pluggy/_result.py:31 |  |  | 0.776 |
| walker |  | 2680 | 12 | python method doc at src/pluggy/_result.py:31 |  |  | 0.776 |
| walker |  | 2698 | 18 | python class body at src/pluggy/_result.py:24 |  |  | 0.776 |
| ns | 2743 |  | 125 | _manager.py: every class/def location, part 2 (validation, pending, plugins, tracing) | 2.7 |  | 0.760 |
| walker |  | 2818 | 120 | python decl names surface in src/pluggy/_manager.py |  |  | 0.762 |
| walker |  | 2818 | 0 | python decl at src/pluggy/_manager.py:42 |  |  | 0.762 |
| walker |  | 2818 | 0 | python decl at src/pluggy/_manager.py:52 |  |  | 0.762 |
| walker |  | 2818 | 0 | python decl at src/pluggy/_manager.py:65 |  |  | 0.762 |
| walker |  | 2818 | 0 | python decl at src/pluggy/_manager.py:83 |  |  | 0.762 |
| walker |  | 2818 | 0 | python decl at src/pluggy/_manager.py:525 |  |  | 0.762 |
| walker |  | 2831 | 13 | python decl doc at src/pluggy/_manager.py:65 |  |  | 0.762 |
| walker |  | 2879 | 48 | python decl doc at src/pluggy/_manager.py:52 |  |  | 0.762 |
| walker |  | 2905 | 26 | python decl at src/pluggy/_manager.py:37 |  |  | 0.762 |
| walker |  | 2921 | 16 | python decl body at src/pluggy/_manager.py:525 body 526 |  |  | 0.762 |
| walker |  | 3266 | 345 | python decl names surface in src/pluggy/_hooks.py |  |  | 0.771 |
| walker |  | 3266 | 0 | python decl at src/pluggy/_hooks.py:40 |  |  | 0.771 |
| walker |  | 3266 | 0 | python decl at src/pluggy/_hooks.py:56 |  |  | 0.771 |
| walker |  | 3266 | 0 | python decl at src/pluggy/_hooks.py:281 |  |  | 0.771 |
| walker |  | 3266 | 0 | python decl at src/pluggy/_hooks.py:293 |  |  | 0.771 |
| walker |  | 3266 | 0 | python decl at src/pluggy/_hooks.py:382 |  |  | 0.771 |
| walker |  | 3266 | 0 | python decl at src/pluggy/_hooks.py:593 |  |  | 0.771 |
| walker |  | 3274 | 8 | python decl at src/pluggy/_hooks.py:358 |  |  | 0.771 |
| walker |  | 3282 | 8 | python decl at src/pluggy/_hooks.py:638 |  |  | 0.772 |
| walker |  | 3290 | 8 | python decl at src/pluggy/_hooks.py:696 |  |  | 0.773 |
| walker |  | 3299 | 9 | python decl at src/pluggy/_hooks.py:77 |  |  | 0.776 |
| walker |  | 3308 | 9 | python decl at src/pluggy/_hooks.py:164 |  |  | 0.779 |
| walker |  | 3320 | 12 | python decl doc at src/pluggy/_hooks.py:40 |  |  | 0.779 |
| walker |  | 3332 | 12 | python decl doc at src/pluggy/_hooks.py:56 |  |  | 0.779 |
| walker |  | 3349 | 17 | python decl doc at src/pluggy/_hooks.py:382 |  |  | 0.781 |
| walker |  | 3367 | 18 | python decl doc at src/pluggy/_hooks.py:638 |  |  | 0.781 |
| ns | 3391 |  | 648 | HookspecMarker.__call__: full option docs (firstresult/historic/warn_on_impl) | 3.1 | 2.3 | 0.711 |
| walker |  | 3398 | 31 | python decl doc at src/pluggy/_hooks.py:358 |  |  | 0.711 |
| walker |  | 3510 | 112 | python class body at src/pluggy/_hooks.py:638 |  |  | 0.711 |
| walker |  | 3598 | 88 | python class body at src/pluggy/_hooks.py:696 |  |  | 0.711 |
| walker |  | 3690 | 92 | python class body at src/pluggy/_hooks.py:382 |  |  | 0.711 |
| walker |  | 3714 | 24 | python decl at src/pluggy/_hooks.py:377 |  |  | 0.711 |
| ns | 3804 |  | 413 | HookCaller: hookimpl call-order algorithm (_add_hookimpl + its ordering comment) | 3.2 | 2.4 | 0.676 |
| walker |  | 3938 | 224 | python class body at src/pluggy/_hooks.py:56 |  |  | 0.676 |
| walker |  | 4100 | 162 | python class body at src/pluggy/_hooks.py:40 |  |  | 0.676 |
| walker |  | 4116 | 16 | python class body at src/pluggy/_hooks.py:77 |  |  | 0.676 |
| walker |  | 4132 | 16 | python class body at src/pluggy/_hooks.py:164 |  |  | 0.676 |
| walker |  | 4164 | 32 | python decl doc at src/pluggy/_hooks.py:593 |  |  | 0.680 |
| walker |  | 4216 | 52 | python class body at src/pluggy/_hooks.py:358 |  |  | 0.680 |
| walker |  | 4734 | 518 | python method sigs in src/pluggy/_hooks.py |  |  | 0.742 |
| walker |  | 4734 | 0 | python method at src/pluggy/_hooks.py:88 |  |  | 0.742 |
| walker |  | 4734 | 0 | python method at src/pluggy/_hooks.py:175 |  |  | 0.742 |
| walker |  | 4734 | 0 | python method at src/pluggy/_hooks.py:365 |  |  | 0.742 |
| walker |  | 4734 | 0 | python method at src/pluggy/_hooks.py:420 |  |  | 0.742 |
| walker |  | 4734 | 0 | python method at src/pluggy/_hooks.py:438 |  |  | 0.742 |
| walker |  | 4734 | 0 | python method at src/pluggy/_hooks.py:442 |  |  | 0.742 |
| walker |  | 4734 | 0 | python method at src/pluggy/_hooks.py:449 |  |  | 0.742 |
| walker |  | 4734 | 0 | python method at src/pluggy/_hooks.py:453 |  |  | 0.742 |
| walker |  | 4734 | 0 | python method at src/pluggy/_hooks.py:477 |  |  | 0.742 |
| walker |  | 4734 | 0 | python method at src/pluggy/_hooks.py:499 |  |  | 0.742 |
| walker |  | 4734 | 0 | python method at src/pluggy/_hooks.py:577 |  |  | 0.742 |
| walker |  | 4734 | 0 | python method at src/pluggy/_hooks.py:612 |  |  | 0.742 |
| walker |  | 4734 | 0 | python method at src/pluggy/_hooks.py:634 |  |  | 0.742 |
| walker |  | 4734 | 0 | python method at src/pluggy/_hooks.py:692 |  |  | 0.742 |
| walker |  | 4734 | 0 | python method at src/pluggy/_hooks.py:709 |  |  | 0.742 |
| walker |  | 4742 | 8 | python method at src/pluggy/_hooks.py:626 |  |  | 0.742 |
| walker |  | 4750 | 8 | python method at src/pluggy/_hooks.py:630 |  |  | 0.742 |
| walker |  | 4760 | 10 | python method doc at src/pluggy/_hooks.py:365 |  |  | 0.742 |
| walker |  | 4791 | 31 | python method at src/pluggy/_hooks.py:543 |  |  | 0.742 |
| walker |  | 4806 | 15 | python method doc at src/pluggy/_hooks.py:449 |  |  | 0.742 |
| walker |  | 4821 | 15 | python method at src/pluggy/_hooks.py:618 |  |  | 0.742 |
| ns | 4828 |  | 1024 | HookimplMarker.__call__: full option docs (wrapper/hookwrapper/optionalhook/tryfirst/trylast/specname) | 3.3 | 2.3 | 0.665 |
| walker |  | 4839 | 18 | python method doc at src/pluggy/_hooks.py:438 |  |  | 0.665 |
| walker |  | 4880 | 41 | python method at src/pluggy/_hooks.py:424 |  |  | 0.665 |
| walker |  | 4897 | 17 | python method at src/pluggy/_hooks.py:480 |  |  | 0.665 |
| walker |  | 4949 | 52 | python method at src/pluggy/_hooks.py:516 |  |  | 0.665 |
| walker |  | 4963 | 14 | python method doc at src/pluggy/_hooks.py:453 |  |  | 0.666 |
| walker |  | 5026 | 63 | python method at src/pluggy/_hooks.py:656 |  |  | 0.666 |
| walker |  | 5038 | 12 | python method doc at src/pluggy/_hooks.py:656 |  |  | 0.666 |
| walker |  | 5118 | 80 | python decl doc at src/pluggy/_hooks.py:164 |  |  | 0.666 |
| walker |  | 5199 | 81 | python decl doc at src/pluggy/_hooks.py:77 |  |  | 0.666 |
| walker |  | 5233 | 34 | python decl at src/pluggy/_hooks.py:33 |  |  | 0.666 |
| walker |  | 5303 | 70 | python method at src/pluggy/_hooks.py:393 |  |  | 0.666 |
| walker |  | 5315 | 12 | python method doc at src/pluggy/_hooks.py:393 |  |  | 0.666 |
| walker |  | 5336 | 21 | python method doc at src/pluggy/_hooks.py:577 |  |  | 0.666 |
| walker |  | 5381 | 45 | python method doc at src/pluggy/_hooks.py:543 |  |  | 0.666 |
| ns | 5849 |  | 1021 | _multicall(): full body -- the actual call loop | 3.4 | 2.1 | 0.597 |
| walker |  | 5996 | 615 | python method sigs in src/pluggy/_manager.py |  |  | 0.647 |
| walker |  | 5996 | 0 | python method at src/pluggy/_manager.py:59 |  |  | 0.647 |
| walker |  | 5996 | 0 | python method at src/pluggy/_manager.py:68 |  |  | 0.647 |
| walker |  | 5996 | 0 | python method at src/pluggy/_manager.py:76 |  |  | 0.647 |
| walker |  | 5996 | 0 | python method at src/pluggy/_manager.py:79 |  |  | 0.647 |
| walker |  | 5996 | 0 | python method at src/pluggy/_manager.py:125 |  |  | 0.647 |
| walker |  | 5996 | 0 | python method at src/pluggy/_manager.py:176 |  |  | 0.647 |
| walker |  | 5996 | 0 | python method at src/pluggy/_manager.py:233 |  |  | 0.647 |
| walker |  | 5996 | 0 | python method at src/pluggy/_manager.py:238 |  |  | 0.647 |
| walker |  | 5996 | 0 | python method at src/pluggy/_manager.py:242 |  |  | 0.647 |
| walker |  | 5996 | 0 | python method at src/pluggy/_manager.py:252 |  |  | 0.647 |
| walker |  | 5996 | 0 | python method at src/pluggy/_manager.py:296 |  |  | 0.647 |
| walker |  | 5996 | 0 | python method at src/pluggy/_manager.py:300 |  |  | 0.647 |
| walker |  | 5996 | 0 | python method at src/pluggy/_manager.py:304 |  |  | 0.647 |
| walker |  | 5996 | 0 | python method at src/pluggy/_manager.py:315 |  |  | 0.647 |
| walker |  | 5996 | 0 | python method at src/pluggy/_manager.py:319 |  |  | 0.647 |
| walker |  | 5996 | 0 | python method at src/pluggy/_manager.py:323 |  |  | 0.647 |
| walker |  | 5996 | 0 | python method at src/pluggy/_manager.py:331 |  |  | 0.647 |
| walker |  | 5996 | 0 | python method at src/pluggy/_manager.py:379 |  |  | 0.647 |
| walker |  | 5996 | 0 | python method at src/pluggy/_manager.py:395 |  |  | 0.647 |
| walker |  | 5996 | 0 | python method at src/pluggy/_manager.py:425 |  |  | 0.647 |
| walker |  | 5996 | 0 | python method at src/pluggy/_manager.py:430 |  |  | 0.647 |
| walker |  | 5996 | 0 | python method at src/pluggy/_manager.py:434 |  |  | 0.647 |
| walker |  | 5996 | 0 | python method at src/pluggy/_manager.py:487 |  |  | 0.647 |
| walker |  | 6004 | 8 | python method at src/pluggy/_manager.py:71 |  |  | 0.649 |
| walker |  | 6014 | 10 | python method at src/pluggy/_manager.py:100 |  |  | 0.649 |
| walker |  | 6042 | 28 | python method at src/pluggy/_manager.py:512 |  |  | 0.649 |
| walker |  | 6056 | 14 | python method doc at src/pluggy/_manager.py:300 |  |  | 0.649 |
| walker |  | 6087 | 31 | python method at src/pluggy/_manager.py:278 |  |  | 0.649 |
| walker |  | 6118 | 31 | python method at src/pluggy/_manager.py:450 |  |  | 0.649 |
| ns | 6122 |  | 273 | PluggyWarning / PluggyTeardownRaisedWarning: full body | 3.5 | 2.2 | 0.633 |
| walker |  | 6133 | 15 | python method doc at src/pluggy/_manager.py:238 |  |  | 0.633 |
| walker |  | 6148 | 15 | python method doc at src/pluggy/_manager.py:296 |  |  | 0.633 |
| walker |  | 6183 | 35 | python method at src/pluggy/_manager.py:201 |  |  | 0.633 |
| walker |  | 6200 | 17 | python method doc at src/pluggy/_manager.py:319 |  |  | 0.633 |
| walker |  | 6218 | 18 | python method doc at src/pluggy/_manager.py:233 |  |  | 0.633 |
| walker |  | 6236 | 18 | python method doc at src/pluggy/_manager.py:315 |  |  | 0.633 |
| walker |  | 6257 | 21 | python method doc at src/pluggy/_manager.py:430 |  |  | 0.633 |
| walker |  | 6286 | 29 | python method doc at src/pluggy/_manager.py:323 |  |  | 0.633 |
| walker |  | 6316 | 30 | python method doc at src/pluggy/_manager.py:425 |  |  | 0.633 |
| walker |  | 6351 | 35 | python method doc at src/pluggy/_manager.py:242 |  |  | 0.633 |
| walker |  | 6393 | 42 | python method doc at src/pluggy/_manager.py:487 |  |  | 0.633 |
| walker |  | 6437 | 44 | python method doc at src/pluggy/_manager.py:379 |  |  | 0.633 |
| walker |  | 6516 | 79 | python decl doc at src/pluggy/_hooks.py:293 |  |  | 0.633 |
| walker |  | 6526 | 10 | python imports in docs/examples/eggsample/eggsample/__init__.py |  |  | 0.633 |
| ns | 6554 |  | 432 | Test roster: test_pluginmanager.py (32 tests) | 4.1 |  | 0.614 |
| walker |  | 6576 | 50 | python method doc at src/pluggy/_manager.py:512 |  |  | 0.614 |
| ns | 6703 |  | 149 | Test roster: test_details.py (10 tests) | 4.2 |  | 0.608 |
| ns | 6922 |  | 219 | Test roster: test_hookcaller.py (15 tests) | 4.3 |  | 0.600 |
| walker |  | 6968 | 392 | tool.towncrier config in pyproject.toml |  |  | 0.600 |
| walker |  | 7029 | 61 | python method doc at src/pluggy/_result.py:80 |  |  | 0.600 |
| ns | 7094 |  | 172 | Test roster: test_invocations.py (13 tests) | 4.4 |  | 0.593 |
| walker |  | 7126 | 97 | python method at src/pluggy/_hooks.py:111 |  |  | 0.594 |
| walker |  | 7189 | 63 | python method doc at src/pluggy/_manager.py:252 |  |  | 0.594 |
| walker |  | 7252 | 63 | python method doc at src/pluggy/_result.py:91 |  |  | 0.594 |
| walker |  | 7317 | 65 | python method doc at src/pluggy/_manager.py:434 |  |  | 0.594 |
| ns | 7381 |  | 287 | Test roster: test_multicall.py (21 tests) | 4.5 |  | 0.583 |
| walker |  | 7422 | 105 | python method at src/pluggy/_hooks.py:101 |  |  | 0.583 |
| walker |  | 7422 | 0 | python method body at src/pluggy/_hooks.py:101 body 109 |  |  | 0.583 |
| walker |  | 7526 | 104 | python method at src/pluggy/_hooks.py:91 |  |  | 0.583 |
| walker |  | 7526 | 0 | python method body at src/pluggy/_hooks.py:91 body 99 |  |  | 0.583 |
| ns | 7629 |  | 248 | testing/conftest.py: full body (shared pm / he_pm fixtures) | 4.6 |  | 0.571 |
| walker |  | 7638 | 112 | python method at src/pluggy/_hooks.py:202 |  |  | 0.572 |
| walker |  | 7715 | 77 | python method doc at src/pluggy/_hooks.py:499 |  |  | 0.572 |
| walker |  | 7835 | 120 | python method at src/pluggy/_hooks.py:190 |  |  | 0.572 |
| walker |  | 7835 | 0 | python method body at src/pluggy/_hooks.py:190 body 200 |  |  | 0.572 |
| ns | 7857 |  | 228 | docs/api_reference.rst: autodoc roster (which classes are documented, in order) | 5.1 |  | 0.565 |
| walker |  | 7954 | 119 | python method at src/pluggy/_hooks.py:178 |  |  | 0.565 |
| walker |  | 7954 | 0 | python method body at src/pluggy/_hooks.py:178 body 188 |  |  | 0.565 |
| walker |  | 8041 | 87 | python method doc at src/pluggy/_manager.py:201 |  |  | 0.565 |
| walker |  | 8081 | 40 | declaration surface of changelog/590.trivial.rst |  |  | 0.565 |
| ns | 8162 |  | 305 | docs/index.rst: 'How does it work?' overview | 5.2 |  | 0.556 |
| walker |  | 8178 | 97 | declaration surface of tox.ini |  |  | 0.556 |
| ns | 8311 |  | 149 | pyproject.toml: every distinct [section] header (config-key roster) | 6.1 |  | 0.561 |
| walker |  | 8367 | 189 | python decl doc at src/pluggy/_manager.py:83 |  |  | 0.561 |
| ns | 8505 |  | 194 | tox.ini: [tox] envlist + [testenv] | 6.2 |  | 0.557 |
| walker |  | 8556 | 189 | python decl doc at src/pluggy/_warnings.py:10 |  |  | 0.578 |
| walker |  | 8653 | 97 | python method doc at src/pluggy/_manager.py:304 |  |  | 0.578 |
| ns | 8683 |  | 178 | tox.ini: [testenv:docs] + [pytest] | 6.3 | 6.2 | 0.571 |
| walker |  | 8751 | 98 | python method doc at src/pluggy/_hooks.py:516 |  |  | 0.571 |
| ns | 8795 |  | 112 | tox.ini: [testenv:release] | 6.4 | 6.2 | 0.567 |
| walker |  | 8850 | 99 | python method doc at src/pluggy/_result.py:67 |  |  | 0.567 |
| walker |  | 8953 | 103 | python method doc at src/pluggy/_manager.py:395 |  |  | 0.567 |
| ns | 8997 |  | 202 | .pre-commit-config.yaml: repo + hook-id roster | 6.5 |  | 0.561 |
| ns | 9048 |  | 51 | .coveragerc: [run] section (what coverage.py tracks) | 6.6 |  | 0.559 |
| ns | 9111 |  | 63 | MANIFEST.in: full (sdist inclusion rules) | 6.7 |  | 0.556 |
| walker |  | 9169 | 216 | python class body at src/pluggy/_hooks.py:593 |  |  | 0.556 |
| ns | 9221 |  | 110 | CHANGELOG.rst: pluggy 1.6.0 -- version header + deprecations | 7.1 |  | 0.552 |
| walker |  | 9249 | 80 | python decl names surface in src/pluggy/_callers.py |  |  | 0.557 |
| walker |  | 9280 | 31 | python decl at src/pluggy/_callers.py:27 |  |  | 0.557 |
| walker |  | 9311 | 31 | python decl doc at src/pluggy/_callers.py:27 |  |  | 0.557 |
| walker |  | 9340 | 29 | python decl at src/pluggy/_callers.py:70 |  |  | 0.557 |
| walker |  | 9374 | 34 | python decl at src/pluggy/_callers.py:60 |  |  | 0.557 |
| walker |  | 9433 | 59 | python decl at src/pluggy/_callers.py:82 |  |  | 0.557 |
| ns | 9452 |  | 231 | CHANGELOG.rst: pluggy 1.6.0 -- bug fixes | 7.2 | 7.1 | 0.552 |
| walker |  | 9500 | 67 | python decl names surface in src/pluggy/_tracing.py |  |  | 0.555 |
| walker |  | 9500 | 0 | python decl at src/pluggy/_tracing.py:16 |  |  | 0.555 |
| walker |  | 9500 | 0 | python decl at src/pluggy/_tracing.py:59 |  |  | 0.555 |
| ns | 9655 |  | 203 | changelog/README.rst: newsfragment type taxonomy + naming convention | 7.3 |  | 0.551 |
| walker |  | 9710 | 210 | python method sigs in src/pluggy/_tracing.py |  |  | 0.565 |
| walker |  | 9710 | 0 | python method at src/pluggy/_tracing.py:17 |  |  | 0.565 |
| walker |  | 9710 | 0 | python method at src/pluggy/_tracing.py:22 |  |  | 0.565 |
| walker |  | 9710 | 0 | python method at src/pluggy/_tracing.py:25 |  |  | 0.565 |
| walker |  | 9710 | 0 | python method at src/pluggy/_tracing.py:42 |  |  | 0.565 |
| walker |  | 9710 | 0 | python method at src/pluggy/_tracing.py:48 |  |  | 0.565 |
| walker |  | 9710 | 0 | python method at src/pluggy/_tracing.py:51 |  |  | 0.565 |
| walker |  | 9710 | 0 | python method at src/pluggy/_tracing.py:60 |  |  | 0.565 |
| walker |  | 9710 | 0 | python method at src/pluggy/_tracing.py:64 |  |  | 0.565 |
| walker |  | 9710 | 0 | python method at src/pluggy/_tracing.py:67 |  |  | 0.565 |
| walker |  | 9719 | 9 | python method body at src/pluggy/_tracing.py:48 body 49 |  |  | 0.565 |
| walker |  | 9731 | 12 | python method body at src/pluggy/_tracing.py:22 body 23 |  |  | 0.565 |
| walker |  | 9744 | 13 | python method body at src/pluggy/_tracing.py:64 body 65 |  |  | 0.565 |
| walker |  | 9761 | 17 | python method body at src/pluggy/_tracing.py:67 body 68 |  |  | 0.565 |
| walker |  | 9895 | 134 | declaration surface of TIDELIFT.rst |  |  | 0.565 |
| walker |  | 9913 | 18 | python method body at src/pluggy/_tracing.py:60 body 61 |  |  | 0.565 |
| ns | 9980 |  | 325 | RELEASING.rst: full release procedure | 7.4 |  | 0.555 |

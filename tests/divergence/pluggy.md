Score(3000)=0.762 I=0.902 C=0.644 ns_rows≤3K=16/40 (reached=8 partial=1 missing=7)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 28 |  | 28 | README title + tagline | 1.1 |  | 0.000 |
| walker |  | 96 | 96 | listing of '.' |  |  | 0.000 |
| walker |  | 100 | 4 | listing of 'src' |  |  | 0.000 |
| ns | 114 |  | 86 | README badges line + who depends on pluggy + doc pointer | 1.2 |  | 0.000 |
| walker |  | 119 | 19 | listing of 'changelog' |  |  | 0.000 |
| walker |  | 123 | 4 | listing of '.claude' |  |  | 0.000 |
| walker |  | 155 | 32 | listing of 'docs' |  |  | 0.000 |
| walker |  | 158 | 3 | listing of 'docs/_static' |  |  | 0.000 |
| walker |  | 162 | 4 | listing of 'docs/_static/img' |  |  | 0.000 |
| ns | 200 |  | 86 | Directory listing: docs/ and its worked-example subtree | 1.3 |  | 0.110 |
| walker |  | 205 | 43 | listing of 'downstream' |  |  | 0.134 |
| walker |  | 248 | 43 | listing of 'src/pluggy' |  |  | 0.146 |
| ns | 293 |  | 93 | Directory listing: changelog, CI, downstream smoke-tests, scripts | 1.4 |  | 0.210 |
| ns | 445 |  | 152 | pluggy/__init__.py: __all__ (the literal public export list) | 1.5 |  | 0.174 |
| walker |  | 566 | 318 | python imports in src/pluggy/__init__.py |  |  | 0.335 |
| walker |  | 582 | 16 | python decl names surface in src/pluggy/__init__.py |  |  | 0.336 |
| walker |  | 582 | 0 | python decl at src/pluggy/__init__.py:32 |  |  | 0.336 |
| ns | 646 |  | 201 | Directory listing: repo root, src/pluggy, testing | 1.6 |  | 0.335 |
| walker |  | 701 | 119 | README headline in README.rst |  |  | 0.654 |
| ns | 904 |  | 258 | pluggy/__init__.py: re-export imports + dynamic __version__ | 1.7 | 1.5 | 0.631 |
| ns | 1237 |  | 333 | README definitive example, part 1: markers + spec/impl namespaces | 1.8 |  | 0.548 |
| walker |  | 1247 | 546 | README.rst section #0 |  |  | 0.699 |
| walker |  | 1260 | 13 | listing of '.github' |  |  | 0.725 |
| walker |  | 1264 | 4 | listing of '.github/workflows' |  |  | 0.735 |
| walker |  | 1290 | 26 | downstream/README.md section #0 |  |  | 0.735 |
| walker |  | 1304 | 14 | listing of 'scripts' |  |  | 0.756 |
| walker |  | 1347 | 43 | tool.setuptools_scm+uv config in pyproject.toml |  |  | 0.756 |
| walker |  | 1398 | 51 | tool.setuptools config in pyproject.toml |  |  | 0.756 |
| ns | 1445 |  | 208 | README definitive example, part 2: manager wiring + call + real output | 1.9 | 1.8 | 0.765 |
| walker |  | 1449 | 51 | [package] in pyproject.toml |  |  | 0.765 |
| ns | 1497 |  | 52 | _callers.py: every def location (signatures only) | 2.1 |  | 0.755 |
| walker |  | 1512 | 63 | manifest config in pyproject.toml |  |  | 0.755 |
| walker |  | 1536 | 24 | python decl names surface in src/pluggy/_warnings.py |  |  | 0.755 |
| walker |  | 1536 | 0 | python decl at src/pluggy/_warnings.py:4 |  |  | 0.755 |
| walker |  | 1552 | 16 | python decl at src/pluggy/_warnings.py:10 |  |  | 0.756 |
| walker |  | 1565 | 13 | python class body at src/pluggy/_warnings.py:4 |  |  | 0.756 |
| walker |  | 1578 | 13 | python class body at src/pluggy/_warnings.py:10 |  |  | 0.756 |
| walker |  | 1592 | 14 | python decl doc at src/pluggy/_warnings.py:4 |  |  | 0.756 |
| walker |  | 1599 | 7 | python imports in src/pluggy/_warnings.py |  |  | 0.756 |
| walker |  | 1614 | 15 | listing of 'docs/examples' |  |  | 0.780 |
| walker |  | 1622 | 8 | listing of 'docs/examples/eggsample' |  |  | 0.800 |
| walker |  | 1642 | 20 | listing of 'docs/examples/eggsample/eggsample' |  |  | 0.846 |
| walker |  | 1704 | 62 | listing of 'testing' |  |  | 0.920 |
| walker |  | 1751 | 47 | plaintext config docs/requirements.txt |  |  | 0.920 |
| ns | 1815 |  | 318 | _result.py / _tracing.py / _warnings.py: every class/def location | 2.2 |  | 0.852 |
| walker |  | 1817 | 66 | python decl body at src/pluggy/__init__.py:32 body 33 |  |  | 0.883 |
| ns | 1951 |  | 136 | _hooks.py: every class/def location, part 1 (markers + helpers) | 2.3 |  | 0.855 |
| walker |  | 2053 | 236 | tool.mypy config in pyproject.toml |  |  | 0.855 |
| walker |  | 2134 | 81 | python decl names surface in src/pluggy/_result.py |  |  | 0.857 |
| walker |  | 2134 | 0 | python decl at src/pluggy/_result.py:20 |  |  | 0.857 |
| ns | 2144 |  | 193 | _hooks.py: every class/def location, part 2 (HookRelay + the full HookCaller method list) | 2.4 |  | 0.822 |
| walker |  | 2145 | 11 | python decl at src/pluggy/_result.py:24 |  |  | 0.824 |
| walker |  | 2154 | 9 | python decl doc at src/pluggy/_result.py:20 |  |  | 0.824 |
| walker |  | 2176 | 22 | python class body at src/pluggy/_result.py:24 |  |  | 0.824 |
| walker |  | 2208 | 32 | python decl doc at src/pluggy/_result.py:24 |  |  | 0.824 |
| ns | 2306 |  | 162 | _hooks.py: every class/def location, part 3 (_SubsetHookCaller/HookImpl/HookSpec) | 2.5 |  | 0.797 |
| walker |  | 2335 | 127 | python method sigs in src/pluggy/_result.py |  |  | 0.814 |
| walker |  | 2335 | 0 | python method at src/pluggy/_result.py:67 |  |  | 0.814 |
| walker |  | 2335 | 0 | python method at src/pluggy/_result.py:80 |  |  | 0.814 |
| walker |  | 2335 | 0 | python method at src/pluggy/_result.py:91 |  |  | 0.814 |
| walker |  | 2343 | 8 | python method at src/pluggy/_result.py:42 |  |  | 0.816 |
| walker |  | 2351 | 8 | python method at src/pluggy/_result.py:51 |  |  | 0.818 |
| walker |  | 2359 | 8 | python method at src/pluggy/_result.py:56 |  |  | 0.821 |
| walker |  | 2371 | 12 | python method doc at src/pluggy/_result.py:42 |  |  | 0.821 |
| walker |  | 2383 | 12 | python method doc at src/pluggy/_result.py:51 |  |  | 0.821 |
| walker |  | 2395 | 12 | python method doc at src/pluggy/_result.py:56 |  |  | 0.821 |
| walker |  | 2435 | 40 | python method at src/pluggy/_result.py:31 |  |  | 0.821 |
| walker |  | 2447 | 12 | python method doc at src/pluggy/_result.py:31 |  |  | 0.821 |
| ns | 2609 |  | 303 | _manager.py: every class/def location, part 1 (registration + introspection) | 2.6 |  | 0.776 |
| walker |  | 2697 | 250 | tool.ruff config in pyproject.toml |  |  | 0.776 |
| ns | 2734 |  | 125 | _manager.py: every class/def location, part 2 (validation, pending, plugins, tracing) | 2.7 |  | 0.760 |
| walker |  | 2817 | 120 | python decl names surface in src/pluggy/_manager.py |  |  | 0.762 |
| walker |  | 2817 | 0 | python decl at src/pluggy/_manager.py:42 |  |  | 0.762 |
| walker |  | 2817 | 0 | python decl at src/pluggy/_manager.py:52 |  |  | 0.762 |
| walker |  | 2817 | 0 | python decl at src/pluggy/_manager.py:65 |  |  | 0.762 |
| walker |  | 2817 | 0 | python decl at src/pluggy/_manager.py:83 |  |  | 0.762 |
| walker |  | 2817 | 0 | python decl at src/pluggy/_manager.py:525 |  |  | 0.762 |
| walker |  | 2830 | 13 | python decl doc at src/pluggy/_manager.py:65 |  |  | 0.762 |
| walker |  | 2878 | 48 | python decl doc at src/pluggy/_manager.py:52 |  |  | 0.762 |
| walker |  | 2904 | 26 | python decl at src/pluggy/_manager.py:37 |  |  | 0.762 |
| walker |  | 2920 | 16 | python decl body at src/pluggy/_manager.py:525 body 526 |  |  | 0.762 |
| walker |  | 3265 | 345 | python decl names surface in src/pluggy/_hooks.py |  |  | 0.771 |
| walker |  | 3265 | 0 | python decl at src/pluggy/_hooks.py:40 |  |  | 0.771 |
| walker |  | 3265 | 0 | python decl at src/pluggy/_hooks.py:56 |  |  | 0.771 |
| walker |  | 3265 | 0 | python decl at src/pluggy/_hooks.py:281 |  |  | 0.771 |
| walker |  | 3265 | 0 | python decl at src/pluggy/_hooks.py:293 |  |  | 0.771 |
| walker |  | 3265 | 0 | python decl at src/pluggy/_hooks.py:382 |  |  | 0.771 |
| walker |  | 3265 | 0 | python decl at src/pluggy/_hooks.py:593 |  |  | 0.771 |
| walker |  | 3273 | 8 | python decl at src/pluggy/_hooks.py:358 |  |  | 0.771 |
| walker |  | 3281 | 8 | python decl at src/pluggy/_hooks.py:638 |  |  | 0.772 |
| walker |  | 3289 | 8 | python decl at src/pluggy/_hooks.py:696 |  |  | 0.773 |
| walker |  | 3298 | 9 | python decl at src/pluggy/_hooks.py:77 |  |  | 0.776 |
| walker |  | 3307 | 9 | python decl at src/pluggy/_hooks.py:164 |  |  | 0.779 |
| walker |  | 3319 | 12 | python decl doc at src/pluggy/_hooks.py:40 |  |  | 0.779 |
| walker |  | 3331 | 12 | python decl doc at src/pluggy/_hooks.py:56 |  |  | 0.779 |
| walker |  | 3347 | 16 | python class body at src/pluggy/_hooks.py:77 |  |  | 0.779 |
| walker |  | 3363 | 16 | python class body at src/pluggy/_hooks.py:164 |  |  | 0.779 |
| walker |  | 3380 | 17 | python decl doc at src/pluggy/_hooks.py:382 |  |  | 0.781 |
| ns | 3382 |  | 648 | HookspecMarker.__call__: full option docs (firstresult/historic/warn_on_impl) | 3.1 | 2.3 | 0.711 |
| walker |  | 3398 | 18 | python decl doc at src/pluggy/_hooks.py:638 |  |  | 0.711 |
| walker |  | 3429 | 31 | python decl doc at src/pluggy/_hooks.py:358 |  |  | 0.711 |
| walker |  | 3541 | 112 | python class body at src/pluggy/_hooks.py:638 |  |  | 0.711 |
| walker |  | 3593 | 52 | python class body at src/pluggy/_hooks.py:358 |  |  | 0.711 |
| walker |  | 3681 | 88 | python class body at src/pluggy/_hooks.py:696 |  |  | 0.711 |
| walker |  | 3773 | 92 | python class body at src/pluggy/_hooks.py:382 |  |  | 0.711 |
| ns | 3795 |  | 413 | HookCaller: hookimpl call-order algorithm (_add_hookimpl + its ordering comment) | 3.2 | 2.4 | 0.676 |
| walker |  | 3797 | 24 | python decl at src/pluggy/_hooks.py:377 |  |  | 0.676 |
| walker |  | 4021 | 224 | python class body at src/pluggy/_hooks.py:56 |  |  | 0.676 |
| walker |  | 4183 | 162 | python class body at src/pluggy/_hooks.py:40 |  |  | 0.676 |
| walker |  | 4215 | 32 | python decl doc at src/pluggy/_hooks.py:593 |  |  | 0.680 |
| walker |  | 4295 | 80 | python decl doc at src/pluggy/_hooks.py:164 |  |  | 0.680 |
| walker |  | 4376 | 81 | python decl doc at src/pluggy/_hooks.py:77 |  |  | 0.680 |
| ns | 4819 |  | 1024 | HookimplMarker.__call__: full option docs (wrapper/hookwrapper/optionalhook/tryfirst/trylast/specname) | 3.3 | 2.3 | 0.610 |
| walker |  | 4916 | 540 | python method sigs in src/pluggy/_hooks.py |  |  | 0.665 |
| walker |  | 4916 | 0 | python method at src/pluggy/_hooks.py:88 |  |  | 0.665 |
| walker |  | 4916 | 0 | python method at src/pluggy/_hooks.py:175 |  |  | 0.665 |
| walker |  | 4916 | 0 | python method at src/pluggy/_hooks.py:365 |  |  | 0.665 |
| walker |  | 4916 | 0 | python method at src/pluggy/_hooks.py:420 |  |  | 0.665 |
| walker |  | 4916 | 0 | python method at src/pluggy/_hooks.py:438 |  |  | 0.665 |
| walker |  | 4916 | 0 | python method at src/pluggy/_hooks.py:442 |  |  | 0.665 |
| walker |  | 4916 | 0 | python method at src/pluggy/_hooks.py:449 |  |  | 0.665 |
| walker |  | 4916 | 0 | python method at src/pluggy/_hooks.py:453 |  |  | 0.665 |
| walker |  | 4916 | 0 | python method at src/pluggy/_hooks.py:477 |  |  | 0.665 |
| walker |  | 4916 | 0 | python method at src/pluggy/_hooks.py:499 |  |  | 0.665 |
| walker |  | 4916 | 0 | python method at src/pluggy/_hooks.py:577 |  |  | 0.665 |
| walker |  | 4916 | 0 | python method at src/pluggy/_hooks.py:612 |  |  | 0.665 |
| walker |  | 4916 | 0 | python method at src/pluggy/_hooks.py:634 |  |  | 0.665 |
| walker |  | 4916 | 0 | python method at src/pluggy/_hooks.py:692 |  |  | 0.665 |
| walker |  | 4916 | 0 | python method at src/pluggy/_hooks.py:709 |  |  | 0.665 |
| walker |  | 4924 | 8 | python method at src/pluggy/_hooks.py:626 |  |  | 0.665 |
| walker |  | 4932 | 8 | python method at src/pluggy/_hooks.py:630 |  |  | 0.665 |
| walker |  | 4942 | 10 | python method doc at src/pluggy/_hooks.py:365 |  |  | 0.665 |
| walker |  | 4973 | 31 | python method at src/pluggy/_hooks.py:543 |  |  | 0.665 |
| walker |  | 4988 | 15 | python method doc at src/pluggy/_hooks.py:449 |  |  | 0.665 |
| walker |  | 5003 | 15 | python method at src/pluggy/_hooks.py:618 |  |  | 0.665 |
| walker |  | 5021 | 18 | python method doc at src/pluggy/_hooks.py:438 |  |  | 0.665 |
| walker |  | 5062 | 41 | python method at src/pluggy/_hooks.py:424 |  |  | 0.665 |
| walker |  | 5079 | 17 | python method at src/pluggy/_hooks.py:480 |  |  | 0.665 |
| walker |  | 5131 | 52 | python method at src/pluggy/_hooks.py:516 |  |  | 0.665 |
| walker |  | 5145 | 14 | python method doc at src/pluggy/_hooks.py:453 |  |  | 0.666 |
| walker |  | 5208 | 63 | python method at src/pluggy/_hooks.py:656 |  |  | 0.666 |
| walker |  | 5220 | 12 | python method doc at src/pluggy/_hooks.py:656 |  |  | 0.666 |
| walker |  | 5254 | 34 | python decl at src/pluggy/_hooks.py:33 |  |  | 0.666 |
| walker |  | 5324 | 70 | python method at src/pluggy/_hooks.py:393 |  |  | 0.666 |
| walker |  | 5336 | 12 | python method doc at src/pluggy/_hooks.py:393 |  |  | 0.666 |
| walker |  | 5357 | 21 | python method doc at src/pluggy/_hooks.py:577 |  |  | 0.666 |
| walker |  | 5402 | 45 | python method doc at src/pluggy/_hooks.py:543 |  |  | 0.666 |
| ns | 5840 |  | 1021 | _multicall(): full body -- the actual call loop | 3.4 | 2.1 | 0.597 |
| walker |  | 6017 | 615 | python method sigs in src/pluggy/_manager.py |  |  | 0.647 |
| walker |  | 6017 | 0 | python method at src/pluggy/_manager.py:59 |  |  | 0.647 |
| walker |  | 6017 | 0 | python method at src/pluggy/_manager.py:68 |  |  | 0.647 |
| walker |  | 6017 | 0 | python method at src/pluggy/_manager.py:76 |  |  | 0.647 |
| walker |  | 6017 | 0 | python method at src/pluggy/_manager.py:79 |  |  | 0.647 |
| walker |  | 6017 | 0 | python method at src/pluggy/_manager.py:125 |  |  | 0.647 |
| walker |  | 6017 | 0 | python method at src/pluggy/_manager.py:176 |  |  | 0.647 |
| walker |  | 6017 | 0 | python method at src/pluggy/_manager.py:233 |  |  | 0.647 |
| walker |  | 6017 | 0 | python method at src/pluggy/_manager.py:238 |  |  | 0.647 |
| walker |  | 6017 | 0 | python method at src/pluggy/_manager.py:242 |  |  | 0.647 |
| walker |  | 6017 | 0 | python method at src/pluggy/_manager.py:252 |  |  | 0.647 |
| walker |  | 6017 | 0 | python method at src/pluggy/_manager.py:296 |  |  | 0.647 |
| walker |  | 6017 | 0 | python method at src/pluggy/_manager.py:300 |  |  | 0.647 |
| walker |  | 6017 | 0 | python method at src/pluggy/_manager.py:304 |  |  | 0.647 |
| walker |  | 6017 | 0 | python method at src/pluggy/_manager.py:315 |  |  | 0.647 |
| walker |  | 6017 | 0 | python method at src/pluggy/_manager.py:319 |  |  | 0.647 |
| walker |  | 6017 | 0 | python method at src/pluggy/_manager.py:323 |  |  | 0.647 |
| walker |  | 6017 | 0 | python method at src/pluggy/_manager.py:331 |  |  | 0.647 |
| walker |  | 6017 | 0 | python method at src/pluggy/_manager.py:379 |  |  | 0.647 |
| walker |  | 6017 | 0 | python method at src/pluggy/_manager.py:395 |  |  | 0.647 |
| walker |  | 6017 | 0 | python method at src/pluggy/_manager.py:425 |  |  | 0.647 |
| walker |  | 6017 | 0 | python method at src/pluggy/_manager.py:430 |  |  | 0.647 |
| walker |  | 6017 | 0 | python method at src/pluggy/_manager.py:434 |  |  | 0.647 |
| walker |  | 6017 | 0 | python method at src/pluggy/_manager.py:487 |  |  | 0.647 |
| walker |  | 6025 | 8 | python method at src/pluggy/_manager.py:71 |  |  | 0.649 |
| walker |  | 6035 | 10 | python method at src/pluggy/_manager.py:100 |  |  | 0.649 |
| walker |  | 6063 | 28 | python method at src/pluggy/_manager.py:512 |  |  | 0.649 |
| walker |  | 6077 | 14 | python method doc at src/pluggy/_manager.py:300 |  |  | 0.649 |
| walker |  | 6108 | 31 | python method at src/pluggy/_manager.py:278 |  |  | 0.649 |
| ns | 6113 |  | 273 | PluggyWarning / PluggyTeardownRaisedWarning: full body | 3.5 | 2.2 | 0.633 |
| walker |  | 6139 | 31 | python method at src/pluggy/_manager.py:450 |  |  | 0.633 |
| walker |  | 6154 | 15 | python method doc at src/pluggy/_manager.py:238 |  |  | 0.633 |
| walker |  | 6169 | 15 | python method doc at src/pluggy/_manager.py:296 |  |  | 0.633 |
| walker |  | 6204 | 35 | python method at src/pluggy/_manager.py:201 |  |  | 0.633 |
| walker |  | 6221 | 17 | python method doc at src/pluggy/_manager.py:319 |  |  | 0.633 |
| walker |  | 6239 | 18 | python method doc at src/pluggy/_manager.py:233 |  |  | 0.633 |
| walker |  | 6257 | 18 | python method doc at src/pluggy/_manager.py:315 |  |  | 0.633 |
| walker |  | 6278 | 21 | python method doc at src/pluggy/_manager.py:430 |  |  | 0.633 |
| walker |  | 6307 | 29 | python method doc at src/pluggy/_manager.py:323 |  |  | 0.633 |
| walker |  | 6337 | 30 | python method doc at src/pluggy/_manager.py:425 |  |  | 0.633 |
| walker |  | 6372 | 35 | python method doc at src/pluggy/_manager.py:242 |  |  | 0.633 |
| walker |  | 6414 | 42 | python method doc at src/pluggy/_manager.py:487 |  |  | 0.633 |
| walker |  | 6458 | 44 | python method doc at src/pluggy/_manager.py:379 |  |  | 0.633 |
| walker |  | 6537 | 79 | python decl doc at src/pluggy/_hooks.py:293 |  |  | 0.633 |
| ns | 6545 |  | 432 | Test roster: test_pluginmanager.py (32 tests) | 4.1 |  | 0.614 |
| walker |  | 6547 | 10 | python imports in docs/examples/eggsample/eggsample/__init__.py |  |  | 0.614 |
| walker |  | 6597 | 50 | python method doc at src/pluggy/_manager.py:512 |  |  | 0.614 |
| walker |  | 6681 | 84 | python method at src/pluggy/_hooks.py:91 |  |  | 0.614 |
| ns | 6694 |  | 149 | Test roster: test_details.py (10 tests) | 4.2 |  | 0.608 |
| ns | 6913 |  | 219 | Test roster: test_hookcaller.py (15 tests) | 4.3 |  | 0.600 |
| walker |  | 7073 | 392 | tool.towncrier config in pyproject.toml |  |  | 0.600 |
| ns | 7085 |  | 172 | Test roster: test_invocations.py (13 tests) | 4.4 |  | 0.593 |
| walker |  | 7164 | 91 | python method at src/pluggy/_hooks.py:101 |  |  | 0.593 |
| walker |  | 7225 | 61 | python method doc at src/pluggy/_result.py:80 |  |  | 0.593 |
| walker |  | 7322 | 97 | python method at src/pluggy/_hooks.py:111 |  |  | 0.594 |
| ns | 7372 |  | 287 | Test roster: test_multicall.py (21 tests) | 4.5 |  | 0.583 |
| walker |  | 7385 | 63 | python method doc at src/pluggy/_manager.py:252 |  |  | 0.583 |
| walker |  | 7448 | 63 | python method doc at src/pluggy/_result.py:91 |  |  | 0.583 |
| walker |  | 7547 | 99 | python method at src/pluggy/_hooks.py:178 |  |  | 0.583 |
| walker |  | 7612 | 65 | python method doc at src/pluggy/_manager.py:434 |  |  | 0.583 |
| ns | 7620 |  | 248 | testing/conftest.py: full body (shared pm / he_pm fixtures) | 4.6 |  | 0.571 |
| walker |  | 7718 | 106 | python method at src/pluggy/_hooks.py:190 |  |  | 0.571 |
| walker |  | 7830 | 112 | python method at src/pluggy/_hooks.py:202 |  |  | 0.572 |
| ns | 7848 |  | 228 | docs/api_reference.rst: autodoc roster (which classes are documented, in order) | 5.1 |  | 0.565 |
| walker |  | 7897 | 67 | python decl names surface in src/pluggy/_tracing.py |  |  | 0.568 |
| walker |  | 7897 | 0 | python decl at src/pluggy/_tracing.py:16 |  |  | 0.568 |
| walker |  | 7897 | 0 | python decl at src/pluggy/_tracing.py:59 |  |  | 0.568 |
| walker |  | 8107 | 210 | python method sigs in src/pluggy/_tracing.py |  |  | 0.585 |
| walker |  | 8107 | 0 | python method at src/pluggy/_tracing.py:17 |  |  | 0.585 |
| walker |  | 8107 | 0 | python method at src/pluggy/_tracing.py:22 |  |  | 0.585 |
| walker |  | 8107 | 0 | python method at src/pluggy/_tracing.py:25 |  |  | 0.585 |
| walker |  | 8107 | 0 | python method at src/pluggy/_tracing.py:42 |  |  | 0.585 |
| walker |  | 8107 | 0 | python method at src/pluggy/_tracing.py:48 |  |  | 0.585 |
| walker |  | 8107 | 0 | python method at src/pluggy/_tracing.py:51 |  |  | 0.585 |
| walker |  | 8107 | 0 | python method at src/pluggy/_tracing.py:60 |  |  | 0.585 |
| walker |  | 8107 | 0 | python method at src/pluggy/_tracing.py:64 |  |  | 0.585 |
| walker |  | 8107 | 0 | python method at src/pluggy/_tracing.py:67 |  |  | 0.585 |
| walker |  | 8116 | 9 | python method body at src/pluggy/_tracing.py:48 body 49 |  |  | 0.585 |
| walker |  | 8128 | 12 | python method body at src/pluggy/_tracing.py:22 body 23 |  |  | 0.585 |
| walker |  | 8141 | 13 | python method body at src/pluggy/_tracing.py:64 body 65 |  |  | 0.585 |
| ns | 8153 |  | 305 | docs/index.rst: 'How does it work?' overview | 5.2 |  | 0.576 |
| walker |  | 8218 | 77 | python method doc at src/pluggy/_hooks.py:499 |  |  | 0.576 |
| walker |  | 8235 | 17 | python method body at src/pluggy/_tracing.py:67 body 68 |  |  | 0.576 |
| ns | 8302 |  | 149 | pyproject.toml: every distinct [section] header (config-key roster) | 6.1 |  | 0.580 |
| walker |  | 8322 | 87 | python method doc at src/pluggy/_manager.py:201 |  |  | 0.580 |
| walker |  | 8362 | 40 | declaration surface of changelog/590.trivial.rst |  |  | 0.580 |
| walker |  | 8442 | 80 | python decl names surface in src/pluggy/_callers.py |  |  | 0.585 |
| walker |  | 8473 | 31 | python decl at src/pluggy/_callers.py:27 |  |  | 0.585 |
| ns | 8496 |  | 194 | tox.ini: [tox] envlist + [testenv] | 6.2 |  | 0.579 |
| walker |  | 8504 | 31 | python decl doc at src/pluggy/_callers.py:27 |  |  | 0.579 |
| walker |  | 8533 | 29 | python decl at src/pluggy/_callers.py:70 |  |  | 0.579 |
| walker |  | 8567 | 34 | python decl at src/pluggy/_callers.py:60 |  |  | 0.579 |
| walker |  | 8664 | 97 | declaration surface of tox.ini |  |  | 0.581 |
| ns | 8674 |  | 178 | tox.ini: [testenv:docs] + [pytest] | 6.3 | 6.2 | 0.575 |
| ns | 8786 |  | 112 | tox.ini: [testenv:release] | 6.4 | 6.2 | 0.570 |
| walker |  | 8853 | 189 | python decl doc at src/pluggy/_manager.py:83 |  |  | 0.570 |
| ns | 8988 |  | 202 | .pre-commit-config.yaml: repo + hook-id roster | 6.5 |  | 0.565 |
| ns | 9039 |  | 51 | .coveragerc: [run] section (what coverage.py tracks) | 6.6 |  | 0.562 |
| walker |  | 9042 | 189 | python decl doc at src/pluggy/_warnings.py:10 |  |  | 0.581 |
| ns | 9102 |  | 63 | MANIFEST.in: full (sdist inclusion rules) | 6.7 |  | 0.578 |
| walker |  | 9139 | 97 | python method doc at src/pluggy/_manager.py:304 |  |  | 0.578 |
| ns | 9212 |  | 110 | CHANGELOG.rst: pluggy 1.6.0 -- version header + deprecations | 7.1 |  | 0.575 |
| walker |  | 9237 | 98 | python method doc at src/pluggy/_hooks.py:516 |  |  | 0.575 |
| walker |  | 9336 | 99 | python method doc at src/pluggy/_result.py:67 |  |  | 0.575 |
| walker |  | 9439 | 103 | python method doc at src/pluggy/_manager.py:395 |  |  | 0.575 |
| ns | 9443 |  | 231 | CHANGELOG.rst: pluggy 1.6.0 -- bug fixes | 7.2 | 7.1 | 0.570 |
| ns | 9646 |  | 203 | changelog/README.rst: newsfragment type taxonomy + naming convention | 7.3 |  | 0.565 |
| walker |  | 9655 | 216 | python class body at src/pluggy/_hooks.py:593 |  |  | 0.565 |
| walker |  | 9714 | 59 | python decl at src/pluggy/_callers.py:82 |  |  | 0.565 |
| walker |  | 9848 | 134 | declaration surface of TIDELIFT.rst |  |  | 0.565 |
| walker |  | 9866 | 18 | python method body at src/pluggy/_tracing.py:60 body 61 |  |  | 0.565 |
| ns | 9971 |  | 325 | RELEASING.rst: full release procedure | 7.4 |  | 0.555 |
| walker |  | 9975 | 109 | python imports in src/pluggy/_result.py |  |  | 0.555 |

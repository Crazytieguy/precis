Score(3000)=0.590 I=0.760 C=0.458 ns_rows≤3K=16/43 (reached=6 partial=3 missing=7)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 62 |  | 62 | Repository identity: README title, dependents, and the docs tagline | 1.1 |  | 0.000 |
| walker |  | 104 | 104 | listing of '.' |  |  | 0.000 |
| ns | 105 |  | 43 | Complete listing of the shipped package directory `src/pluggy/` | 1.2 |  | 0.000 |
| walker |  | 108 | 4 | listing of 'src' |  |  | 0.000 |
| walker |  | 126 | 18 | listing of 'changelog' |  |  | 0.000 |
| walker |  | 129 | 3 | listing of '.claude' |  |  | 0.000 |
| walker |  | 162 | 33 | listing of 'docs' |  |  | 0.000 |
| walker |  | 165 | 3 | listing of 'docs/_static' |  |  | 0.000 |
| walker |  | 168 | 3 | listing of 'docs/_static/img' |  |  | 0.000 |
| ns | 209 |  | 104 | Complete repository root listing | 1.3 |  | 0.445 |
| walker |  | 210 | 42 | listing of 'downstream' |  |  | 0.445 |
| walker |  | 252 | 42 | listing of 'src/pluggy' |  |  | 0.723 |
| ns | 361 |  | 152 | The complete public API name list (`__all__` of src/pluggy/__init__.py) | 1.4 |  | 0.577 |
| ns | 527 |  | 166 | Public name to private module map (the re-export block) | 1.5 | 1.4 | 0.504 |
| walker |  | 570 | 318 | python imports in src/pluggy/__init__.py |  |  | 0.770 |
| walker |  | 586 | 16 | python decl names surface in src/pluggy/__init__.py |  |  | 0.771 |
| walker |  | 586 | 0 | python decl at src/pluggy/__init__.py:32 |  |  | 0.771 |
| walker |  | 599 | 13 | listing of '.github' |  |  | 0.771 |
| walker |  | 602 | 3 | listing of '.github/workflows' |  |  | 0.771 |
| walker |  | 615 | 13 | listing of 'scripts' |  |  | 0.771 |
| ns | 621 |  | 94 | Complete listings of `testing/` and `docs/` | 1.6 |  | 0.680 |
| walker |  | 734 | 119 | README headline in README.rst |  |  | 0.820 |
| ns | 845 |  | 224 | What pluggy is and what problem it solves (docs/index.rst lede) | 1.7 |  | 0.752 |
| ns | 901 |  | 56 | Complete listing of the two worked example packages under docs/examples/ | 1.8 |  | 0.705 |
| ns | 993 |  | 92 | Lazy `__version__` resolution in `__init__.py` | 1.9 | 1.5 | 0.672 |
| walker |  | 1280 | 546 | README.rst section #0 |  |  | 0.672 |
| ns | 1304 |  | 311 | Every method name on `PluginManager` (complete roster, names only) | 2.1 |  | 0.597 |
| walker |  | 1306 | 26 | downstream/README.md section #0 |  |  | 0.597 |
| walker |  | 1322 | 16 | listing of 'docs/examples' |  |  | 0.601 |
| walker |  | 1330 | 8 | listing of 'docs/examples/eggsample' |  |  | 0.608 |
| walker |  | 1383 | 53 | [package] in pyproject.toml |  |  | 0.608 |
| walker |  | 1424 | 41 | tool.setuptools_scm+uv config in pyproject.toml |  |  | 0.608 |
| walker |  | 1475 | 51 | tool.setuptools config in pyproject.toml |  |  | 0.609 |
| ns | 1536 |  | 232 | The rest of `_manager.py`: PluginValidationError, DistFacade, and the two module helpers | 2.2 |  | 0.565 |
| walker |  | 1538 | 63 | manifest config in pyproject.toml |  |  | 0.565 |
| walker |  | 1618 | 80 | package metadata in pyproject.toml |  |  | 0.565 |
| walker |  | 1679 | 61 | listing of 'testing' |  |  | 0.640 |
| walker |  | 1690 | 11 | python imports in src/pluggy/_warnings.py |  |  | 0.640 |
| walker |  | 1709 | 19 | listing of 'docs/examples/eggsample/eggsample' |  |  | 0.663 |
| walker |  | 1729 | 20 | python decl names surface in src/pluggy/_warnings.py |  |  | 0.663 |
| walker |  | 1729 | 0 | python decl at src/pluggy/_warnings.py:4 |  |  | 0.663 |
| walker |  | 1745 | 16 | python decl at src/pluggy/_warnings.py:10 |  |  | 0.663 |
| walker |  | 1761 | 16 | python decl doc at src/pluggy/_warnings.py:4 |  |  | 0.663 |
| walker |  | 1772 | 11 | python class body at src/pluggy/_warnings.py:4 |  |  | 0.663 |
| walker |  | 1848 | 76 | python decl names surface in src/pluggy/_hooks.py |  |  | 0.663 |
| walker |  | 1848 | 0 | python decl at src/pluggy/_hooks.py:40 |  |  | 0.663 |
| walker |  | 1848 | 0 | python decl at src/pluggy/_hooks.py:56 |  |  | 0.663 |
| walker |  | 1848 | 0 | python decl at src/pluggy/_hooks.py:382 |  |  | 0.663 |
| walker |  | 1856 | 8 | python decl at src/pluggy/_hooks.py:358 |  |  | 0.663 |
| walker |  | 1864 | 8 | python decl at src/pluggy/_hooks.py:638 |  |  | 0.664 |
| ns | 1865 |  | 329 | Every top-level symbol in `_hooks.py` (complete roster) plus the two backward-compat aliases | 2.3 |  | 0.616 |
| walker |  | 1873 | 9 | python decl at src/pluggy/_hooks.py:77 |  |  | 0.617 |
| walker |  | 1882 | 9 | python decl at src/pluggy/_hooks.py:164 |  |  | 0.619 |
| walker |  | 1894 | 12 | python decl doc at src/pluggy/_hooks.py:40 |  |  | 0.619 |
| walker |  | 1906 | 12 | python decl doc at src/pluggy/_hooks.py:56 |  |  | 0.619 |
| walker |  | 1923 | 17 | python decl doc at src/pluggy/_hooks.py:382 |  |  | 0.622 |
| walker |  | 1941 | 18 | python decl doc at src/pluggy/_hooks.py:638 |  |  | 0.626 |
| walker |  | 1972 | 31 | python decl doc at src/pluggy/_hooks.py:358 |  |  | 0.634 |
| ns | 2027 |  | 162 | Every method name on `HookCaller` (complete roster, names only) | 2.4 |  | 0.609 |
| walker |  | 2231 | 259 | python decl names surface #1 in src/pluggy/_hooks.py |  |  | 0.634 |
| walker |  | 2231 | 0 | python decl at src/pluggy/_hooks.py:281 |  |  | 0.634 |
| walker |  | 2231 | 0 | python decl at src/pluggy/_hooks.py:293 |  |  | 0.634 |
| walker |  | 2231 | 0 | python decl at src/pluggy/_hooks.py:593 |  |  | 0.634 |
| walker |  | 2239 | 8 | python decl at src/pluggy/_hooks.py:696 |  |  | 0.638 |
| walker |  | 2263 | 24 | python decl at src/pluggy/_hooks.py:377 |  |  | 0.638 |
| ns | 2309 |  | 282 | Complete `Result` API with signatures (`_result.py`) | 2.5 |  | 0.603 |
| ns | 2571 |  | 262 | Complete top-level roster of `_callers.py` (the call loop module) | 2.6 |  | 0.574 |
| walker |  | 2797 | 534 | python method sigs in src/pluggy/_hooks.py |  |  | 0.614 |
| walker |  | 2797 | 0 | python method at src/pluggy/_hooks.py:88 |  |  | 0.614 |
| walker |  | 2797 | 0 | python method at src/pluggy/_hooks.py:175 |  |  | 0.614 |
| walker |  | 2797 | 0 | python method at src/pluggy/_hooks.py:365 |  |  | 0.614 |
| walker |  | 2797 | 0 | python method at src/pluggy/_hooks.py:420 |  |  | 0.614 |
| walker |  | 2797 | 0 | python method at src/pluggy/_hooks.py:438 |  |  | 0.614 |
| walker |  | 2797 | 0 | python method at src/pluggy/_hooks.py:442 |  |  | 0.614 |
| walker |  | 2797 | 0 | python method at src/pluggy/_hooks.py:449 |  |  | 0.614 |
| walker |  | 2797 | 0 | python method at src/pluggy/_hooks.py:453 |  |  | 0.614 |
| walker |  | 2797 | 0 | python method at src/pluggy/_hooks.py:477 |  |  | 0.614 |
| walker |  | 2797 | 0 | python method at src/pluggy/_hooks.py:499 |  |  | 0.614 |
| walker |  | 2797 | 0 | python method at src/pluggy/_hooks.py:577 |  |  | 0.614 |
| walker |  | 2797 | 0 | python method at src/pluggy/_hooks.py:612 |  |  | 0.614 |
| walker |  | 2797 | 0 | python method at src/pluggy/_hooks.py:634 |  |  | 0.614 |
| walker |  | 2797 | 0 | python method at src/pluggy/_hooks.py:692 |  |  | 0.614 |
| walker |  | 2797 | 0 | python method at src/pluggy/_hooks.py:709 |  |  | 0.614 |
| walker |  | 2805 | 8 | python method at src/pluggy/_hooks.py:626 |  |  | 0.614 |
| walker |  | 2813 | 8 | python method at src/pluggy/_hooks.py:630 |  |  | 0.614 |
| walker |  | 2844 | 31 | python method at src/pluggy/_hooks.py:543 |  |  | 0.614 |
| walker |  | 2859 | 15 | python method at src/pluggy/_hooks.py:618 |  |  | 0.614 |
| ns | 2895 |  | 324 | Complete symbol rosters for the two remaining modules: `_tracing.py` and `_warnings.py` | 2.7 |  | 0.590 |
| walker |  | 2900 | 41 | python method at src/pluggy/_hooks.py:424 |  |  | 0.590 |
| walker |  | 2917 | 17 | python method at src/pluggy/_hooks.py:480 |  |  | 0.590 |
| walker |  | 2969 | 52 | python method at src/pluggy/_hooks.py:516 |  |  | 0.590 |
| walker |  | 2981 | 12 | python method doc at src/pluggy/_hooks.py:365 |  |  | 0.590 |
| walker |  | 2996 | 15 | python method doc at src/pluggy/_hooks.py:449 |  |  | 0.590 |
| walker |  | 3059 | 63 | python method at src/pluggy/_hooks.py:656 |  |  | 0.590 |
| walker |  | 3071 | 12 | python method doc at src/pluggy/_hooks.py:656 |  |  | 0.590 |
| walker |  | 3105 | 34 | python decl at src/pluggy/_hooks.py:33 |  |  | 0.590 |
| walker |  | 3175 | 70 | python method at src/pluggy/_hooks.py:393 |  |  | 0.590 |
| walker |  | 3187 | 12 | python method doc at src/pluggy/_hooks.py:393 |  |  | 0.590 |
| walker |  | 3205 | 18 | python method doc at src/pluggy/_hooks.py:438 |  |  | 0.590 |
| ns | 3306 |  | 411 | Complete section map of the 1082-line manual `docs/index.rst` | 2.8 |  | 0.540 |
| walker |  | 3315 | 110 | python class body at src/pluggy/_hooks.py:638 |  |  | 0.540 |
| walker |  | 3401 | 86 | python class body at src/pluggy/_hooks.py:696 |  |  | 0.542 |
| walker |  | 3487 | 86 | python class body at src/pluggy/_hooks.py:382 |  |  | 0.542 |
| ns | 3551 |  | 245 | The real call signatures of `@hookspec` and `@hookimpl` — every accepted option with its default | 3.1 |  | 0.522 |
| walker |  | 3584 | 97 | python method at src/pluggy/_hooks.py:111 |  |  | 0.533 |
| walker |  | 3597 | 13 | python class body at src/pluggy/_warnings.py:10 |  |  | 0.533 |
| walker |  | 3702 | 105 | python method at src/pluggy/_hooks.py:101 |  |  | 0.533 |
| walker |  | 3702 | 0 | python method body at src/pluggy/_hooks.py:101 body 109 |  |  | 0.533 |
| ns | 3799 |  | 248 | `HookimplOpts` in full: every hook-implementation option and what it means | 3.2 | 2.3 | 0.516 |
| walker |  | 3806 | 104 | python method at src/pluggy/_hooks.py:91 |  |  | 0.517 |
| walker |  | 3806 | 0 | python method body at src/pluggy/_hooks.py:91 body 99 |  |  | 0.517 |
| walker |  | 3820 | 14 | python method doc at src/pluggy/_hooks.py:453 |  |  | 0.517 |
| walker |  | 3867 | 47 | plaintext config docs/requirements.txt |  |  | 0.517 |
| ns | 3983 |  | 184 | `HookspecOpts` in full: every hook-specification option | 3.3 | 2.3 | 0.505 |
| walker |  | 4091 | 224 | python class body at src/pluggy/_hooks.py:56 |  |  | 0.542 |
| walker |  | 4105 | 14 | python class body at src/pluggy/_hooks.py:77 |  |  | 0.542 |
| walker |  | 4119 | 14 | python class body at src/pluggy/_hooks.py:164 |  |  | 0.542 |
| walker |  | 4185 | 66 | python decl body at src/pluggy/__init__.py:32 body 33 |  |  | 0.559 |
| walker |  | 4297 | 112 | python method at src/pluggy/_hooks.py:202 |  |  | 0.587 |
| ns | 4415 |  | 432 | Hook implementation ordering: the `_hookimpls` layout comment and `_add_hookimpl` in full | 3.4 | 2.4 | 0.558 |
| walker |  | 4459 | 162 | python class body at src/pluggy/_hooks.py:40 |  |  | 0.581 |
| walker |  | 4579 | 120 | python method at src/pluggy/_hooks.py:190 |  |  | 0.581 |
| walker |  | 4579 | 0 | python method body at src/pluggy/_hooks.py:190 body 200 |  |  | 0.581 |
| ns | 4621 |  | 206 | `HookCaller.__call__` in full: what `pm.hook.myhook(...)` actually does | 3.5 | 2.4 | 0.567 |
| walker |  | 4698 | 119 | python method at src/pluggy/_hooks.py:178 |  |  | 0.569 |
| walker |  | 4698 | 0 | python method body at src/pluggy/_hooks.py:178 body 188 |  |  | 0.569 |
| walker |  | 4746 | 48 | python class body at src/pluggy/_hooks.py:358 |  |  | 0.569 |
| walker |  | 4778 | 32 | python decl doc at src/pluggy/_hooks.py:593 |  |  | 0.578 |
| walker |  | 4858 | 80 | python decl doc at src/pluggy/_hooks.py:164 |  |  | 0.578 |
| walker |  | 4939 | 81 | python decl doc at src/pluggy/_hooks.py:77 |  |  | 0.578 |
| walker |  | 5020 | 81 | python decl names surface in src/pluggy/_result.py |  |  | 0.578 |
| walker |  | 5020 | 0 | python decl at src/pluggy/_result.py:20 |  |  | 0.578 |
| walker |  | 5031 | 11 | python decl at src/pluggy/_result.py:24 |  |  | 0.579 |
| walker |  | 5040 | 9 | python decl doc at src/pluggy/_result.py:20 |  |  | 0.579 |
| walker |  | 5074 | 34 | python decl doc at src/pluggy/_result.py:24 |  |  | 0.582 |
| ns | 5086 |  | 465 | `_multicall` part 1: the setup / non-wrapper call loop | 3.6 |  | 0.551 |
| walker |  | 5203 | 129 | python method sigs in src/pluggy/_result.py |  |  | 0.566 |
| walker |  | 5203 | 0 | python method at src/pluggy/_result.py:67 |  |  | 0.566 |
| walker |  | 5203 | 0 | python method at src/pluggy/_result.py:80 |  |  | 0.566 |
| walker |  | 5203 | 0 | python method at src/pluggy/_result.py:91 |  |  | 0.566 |
| walker |  | 5211 | 8 | python method at src/pluggy/_result.py:42 |  |  | 0.568 |
| walker |  | 5219 | 8 | python method at src/pluggy/_result.py:51 |  |  | 0.570 |
| walker |  | 5227 | 8 | python method at src/pluggy/_result.py:56 |  |  | 0.572 |
| walker |  | 5239 | 12 | python method doc at src/pluggy/_result.py:42 |  |  | 0.572 |
| walker |  | 5251 | 12 | python method doc at src/pluggy/_result.py:51 |  |  | 0.572 |
| walker |  | 5263 | 12 | python method doc at src/pluggy/_result.py:56 |  |  | 0.572 |
| walker |  | 5303 | 40 | python method at src/pluggy/_result.py:31 |  |  | 0.572 |
| walker |  | 5315 | 12 | python method doc at src/pluggy/_result.py:31 |  |  | 0.572 |
| walker |  | 5333 | 18 | python class body at src/pluggy/_result.py:24 |  |  | 0.576 |
| walker |  | 5354 | 21 | python method doc at src/pluggy/_hooks.py:577 |  |  | 0.576 |
| walker |  | 5399 | 45 | python method doc at src/pluggy/_hooks.py:543 |  |  | 0.576 |
| walker |  | 5478 | 79 | python decl doc at src/pluggy/_hooks.py:293 |  |  | 0.576 |
| ns | 5529 |  | 443 | `_multicall` part 2: the teardown loop, exception routing and `firstresult` collapse | 3.7 | 3.6 | 0.545 |
| walker |  | 5598 | 120 | python decl names surface in src/pluggy/_manager.py |  |  | 0.547 |
| walker |  | 5598 | 0 | python decl at src/pluggy/_manager.py:42 |  |  | 0.547 |
| walker |  | 5598 | 0 | python decl at src/pluggy/_manager.py:52 |  |  | 0.547 |
| walker |  | 5598 | 0 | python decl at src/pluggy/_manager.py:65 |  |  | 0.547 |
| walker |  | 5598 | 0 | python decl at src/pluggy/_manager.py:83 |  |  | 0.547 |
| walker |  | 5598 | 0 | python decl at src/pluggy/_manager.py:525 |  |  | 0.547 |
| walker |  | 5611 | 13 | python decl doc at src/pluggy/_manager.py:65 |  |  | 0.548 |
| walker |  | 5659 | 48 | python decl doc at src/pluggy/_manager.py:52 |  |  | 0.554 |
| walker |  | 5685 | 26 | python decl at src/pluggy/_manager.py:37 |  |  | 0.546 |
| ns | 5685 |  | 156 | `PluginManager.register` docstring: naming, blocking and the duplicate-registration contract | 3.8 | 2.1 | 0.546 |
| walker |  | 5701 | 16 | python decl body at src/pluggy/_manager.py:525 body 526 |  |  | 0.548 |
| ns | 5981 |  | 296 | `PluginManager.register` body: how hook implementations are discovered and attached | 3.9 | 3.8 | 0.535 |
| walker |  | 6316 | 615 | python method sigs in src/pluggy/_manager.py |  |  | 0.589 |
| walker |  | 6316 | 0 | python method at src/pluggy/_manager.py:59 |  |  | 0.589 |
| walker |  | 6316 | 0 | python method at src/pluggy/_manager.py:68 |  |  | 0.589 |
| walker |  | 6316 | 0 | python method at src/pluggy/_manager.py:76 |  |  | 0.589 |
| walker |  | 6316 | 0 | python method at src/pluggy/_manager.py:79 |  |  | 0.589 |
| walker |  | 6316 | 0 | python method at src/pluggy/_manager.py:125 |  |  | 0.589 |
| walker |  | 6316 | 0 | python method at src/pluggy/_manager.py:176 |  |  | 0.589 |
| walker |  | 6316 | 0 | python method at src/pluggy/_manager.py:233 |  |  | 0.589 |
| walker |  | 6316 | 0 | python method at src/pluggy/_manager.py:238 |  |  | 0.589 |
| walker |  | 6316 | 0 | python method at src/pluggy/_manager.py:242 |  |  | 0.589 |
| walker |  | 6316 | 0 | python method at src/pluggy/_manager.py:252 |  |  | 0.589 |
| walker |  | 6316 | 0 | python method at src/pluggy/_manager.py:296 |  |  | 0.589 |
| walker |  | 6316 | 0 | python method at src/pluggy/_manager.py:300 |  |  | 0.589 |
| walker |  | 6316 | 0 | python method at src/pluggy/_manager.py:304 |  |  | 0.589 |
| walker |  | 6316 | 0 | python method at src/pluggy/_manager.py:315 |  |  | 0.589 |
| walker |  | 6316 | 0 | python method at src/pluggy/_manager.py:319 |  |  | 0.589 |
| walker |  | 6316 | 0 | python method at src/pluggy/_manager.py:323 |  |  | 0.589 |
| walker |  | 6316 | 0 | python method at src/pluggy/_manager.py:331 |  |  | 0.589 |
| walker |  | 6316 | 0 | python method at src/pluggy/_manager.py:379 |  |  | 0.589 |
| walker |  | 6316 | 0 | python method at src/pluggy/_manager.py:395 |  |  | 0.589 |
| walker |  | 6316 | 0 | python method at src/pluggy/_manager.py:425 |  |  | 0.589 |
| walker |  | 6316 | 0 | python method at src/pluggy/_manager.py:430 |  |  | 0.589 |
| walker |  | 6316 | 0 | python method at src/pluggy/_manager.py:434 |  |  | 0.589 |
| walker |  | 6316 | 0 | python method at src/pluggy/_manager.py:487 |  |  | 0.589 |
| walker |  | 6324 | 8 | python method at src/pluggy/_manager.py:71 |  |  | 0.591 |
| walker |  | 6334 | 10 | python method at src/pluggy/_manager.py:100 |  |  | 0.591 |
| walker |  | 6362 | 28 | python method at src/pluggy/_manager.py:512 |  |  | 0.591 |
| walker |  | 6376 | 14 | python method doc at src/pluggy/_manager.py:300 |  |  | 0.591 |
| walker |  | 6407 | 31 | python method at src/pluggy/_manager.py:278 |  |  | 0.591 |
| walker |  | 6438 | 31 | python method at src/pluggy/_manager.py:450 |  |  | 0.591 |
| ns | 6439 |  | 458 | `PluginManager._verify_hook`: every validation error message pluggy can raise | 3.10 | 2.1 | 0.570 |
| walker |  | 6453 | 15 | python method doc at src/pluggy/_manager.py:238 |  |  | 0.570 |
| walker |  | 6468 | 15 | python method doc at src/pluggy/_manager.py:296 |  |  | 0.570 |
| walker |  | 6503 | 35 | python method at src/pluggy/_manager.py:201 |  |  | 0.570 |
| walker |  | 6520 | 17 | python method doc at src/pluggy/_manager.py:319 |  |  | 0.570 |
| walker |  | 6538 | 18 | python method doc at src/pluggy/_manager.py:233 |  |  | 0.570 |
| walker |  | 6556 | 18 | python method doc at src/pluggy/_manager.py:315 |  |  | 0.570 |
| walker |  | 6577 | 21 | python method doc at src/pluggy/_manager.py:430 |  |  | 0.570 |
| walker |  | 6606 | 29 | python method doc at src/pluggy/_manager.py:323 |  |  | 0.570 |
| ns | 6635 |  | 196 | `PluginManager.__init__`: the complete state of a plugin manager | 3.11 | 2.1 | 0.563 |
| walker |  | 6636 | 30 | python method doc at src/pluggy/_manager.py:425 |  |  | 0.563 |
| walker |  | 6671 | 35 | python method doc at src/pluggy/_manager.py:242 |  |  | 0.563 |
| walker |  | 6713 | 42 | python method doc at src/pluggy/_manager.py:487 |  |  | 0.563 |
| walker |  | 6757 | 44 | python method doc at src/pluggy/_manager.py:379 |  |  | 0.563 |
| walker |  | 6767 | 10 | python imports in docs/examples/eggsample/eggsample/__init__.py |  |  | 0.563 |
| walker |  | 6817 | 50 | python method doc at src/pluggy/_manager.py:512 |  |  | 0.563 |
| walker |  | 6878 | 61 | python method doc at src/pluggy/_result.py:80 |  |  | 0.563 |
| walker |  | 6941 | 63 | python method doc at src/pluggy/_manager.py:252 |  |  | 0.563 |
| ns | 7001 |  | 366 | Historic hooks end to end: `set_specification`, `call_historic`, `_maybe_apply_history` | 3.12 | 2.4 | 0.549 |
| walker |  | 7004 | 63 | python method doc at src/pluggy/_result.py:91 |  |  | 0.549 |
| walker |  | 7069 | 65 | python method doc at src/pluggy/_manager.py:434 |  |  | 0.549 |
| walker |  | 7146 | 77 | python method doc at src/pluggy/_hooks.py:499 |  |  | 0.553 |
| ns | 7201 |  | 200 | Complete attribute sets of `HookImpl` and `HookSpec` (`__slots__`) | 3.13 |  | 0.567 |
| walker |  | 7233 | 87 | python method doc at src/pluggy/_manager.py:201 |  |  | 0.567 |
| walker |  | 7273 | 40 | declaration surface of changelog/590.trivial.rst |  |  | 0.567 |
| walker |  | 7370 | 97 | declaration surface of tox.ini |  |  | 0.567 |
| ns | 7511 |  | 310 | `load_setuptools_entrypoints`: how third-party plugins are discovered | 3.14 | 2.1 | 0.553 |
| walker |  | 7559 | 189 | python decl doc at src/pluggy/_manager.py:83 |  |  | 0.553 |
| ns | 7705 |  | 194 | Blocking semantics: `set_blocked`, `is_blocked`, `unblock` | 3.15 | 2.1 | 0.549 |
| walker |  | 7748 | 189 | python decl doc at src/pluggy/_warnings.py:10 |  |  | 0.551 |
| walker |  | 7845 | 97 | python method doc at src/pluggy/_manager.py:304 |  |  | 0.551 |
| ns | 7937 |  | 232 | `docs/api_reference.rst`: exactly which types are publicly documented | 4.1 |  | 0.544 |
| walker |  | 7943 | 98 | python method doc at src/pluggy/_hooks.py:516 |  |  | 0.544 |
| walker |  | 8042 | 99 | python method doc at src/pluggy/_result.py:67 |  |  | 0.544 |
| walker |  | 8145 | 103 | python method doc at src/pluggy/_manager.py:395 |  |  | 0.549 |
| ns | 8267 |  | 330 | `docs/examples/toy-example.py`: the canonical end-to-end usage | 4.2 |  | 0.534 |
| walker |  | 8361 | 216 | python class body at src/pluggy/_hooks.py:593 |  |  | 0.534 |
| ns | 8548 |  | 281 | The eggsample host program: wiring a PluginManager and calling a hook | 4.3 |  | 0.524 |
| walker |  | 8582 | 221 | python imports in src/pluggy/_hooks.py |  |  | 0.524 |
| walker |  | 8662 | 80 | python decl names surface in src/pluggy/_callers.py |  |  | 0.525 |
| walker |  | 8693 | 31 | python decl at src/pluggy/_callers.py:27 |  |  | 0.527 |
| walker |  | 8724 | 31 | python decl doc at src/pluggy/_callers.py:27 |  |  | 0.527 |
| walker |  | 8753 | 29 | python decl at src/pluggy/_callers.py:70 |  |  | 0.529 |
| ns | 8768 |  | 220 | `eggsample/hookspecs.py` in full: what a real hookspec module looks like | 4.4 |  | 0.520 |
| walker |  | 8787 | 34 | python decl at src/pluggy/_callers.py:60 |  |  | 0.523 |
| walker |  | 8846 | 59 | python decl at src/pluggy/_callers.py:82 |  |  | 0.531 |
| walker |  | 8980 | 134 | declaration surface of TIDELIFT.rst |  |  | 0.531 |
| walker |  | 9047 | 67 | python decl names surface in src/pluggy/_tracing.py |  |  | 0.532 |
| walker |  | 9047 | 0 | python decl at src/pluggy/_tracing.py:16 |  |  | 0.532 |
| walker |  | 9047 | 0 | python decl at src/pluggy/_tracing.py:59 |  |  | 0.532 |
| ns | 9104 |  | 336 | Both sides of hook implementation: the host's own `lib.py` and the external plugin `eggsample_spam.py` | 4.5 |  | 0.520 |
| ns | 9247 |  | 143 | Entry-point wiring in both example `setup.py` files | 4.6 |  | 0.516 |
| walker |  | 9257 | 210 | python method sigs in src/pluggy/_tracing.py |  |  | 0.532 |
| walker |  | 9257 | 0 | python method at src/pluggy/_tracing.py:17 |  |  | 0.532 |
| walker |  | 9257 | 0 | python method at src/pluggy/_tracing.py:22 |  |  | 0.532 |
| walker |  | 9257 | 0 | python method at src/pluggy/_tracing.py:25 |  |  | 0.532 |
| walker |  | 9257 | 0 | python method at src/pluggy/_tracing.py:42 |  |  | 0.532 |
| walker |  | 9257 | 0 | python method at src/pluggy/_tracing.py:48 |  |  | 0.532 |
| walker |  | 9257 | 0 | python method at src/pluggy/_tracing.py:51 |  |  | 0.532 |
| walker |  | 9257 | 0 | python method at src/pluggy/_tracing.py:60 |  |  | 0.532 |
| walker |  | 9257 | 0 | python method at src/pluggy/_tracing.py:64 |  |  | 0.532 |
| walker |  | 9257 | 0 | python method at src/pluggy/_tracing.py:67 |  |  | 0.532 |
| walker |  | 9266 | 9 | python method body at src/pluggy/_tracing.py:48 body 49 |  |  | 0.532 |
| walker |  | 9278 | 12 | python method body at src/pluggy/_tracing.py:22 body 23 |  |  | 0.532 |
| walker |  | 9291 | 13 | python method body at src/pluggy/_tracing.py:64 body 65 |  |  | 0.532 |
| walker |  | 9308 | 17 | python method body at src/pluggy/_tracing.py:67 body 68 |  |  | 0.532 |
| walker |  | 9326 | 18 | python method body at src/pluggy/_tracing.py:60 body 61 |  |  | 0.532 |
| walker |  | 9435 | 109 | python imports in src/pluggy/_result.py |  |  | 0.532 |
| ns | 9495 |  | 248 | `testing/conftest.py` in full: the two fixtures every test in the suite uses | 5.1 |  | 0.523 |
| walker |  | 9570 | 135 | python method doc at src/pluggy/_manager.py:176 |  |  | 0.523 |
| walker |  | 9626 | 56 | python decl doc at src/pluggy/_callers.py:82 |  |  | 0.523 |
| ns | 9654 |  | 159 | `pyproject.toml`: package identity, Python floor, dependency groups and the src layout | 5.2 |  | 0.523 |
| walker |  | 9763 | 137 | python method doc at src/pluggy/_manager.py:125 |  |  | 0.535 |
| ns | 9833 |  | 179 | `tox.ini`: the environment list and the embedded pytest configuration | 5.3 |  | 0.530 |
| walker |  | 9909 | 146 | python method doc at src/pluggy/_manager.py:278 |  |  | 0.530 |
| ns | 9923 |  | 90 | `[tool.towncrier]` config: how CHANGELOG.rst is produced | 5.4 |  | 0.528 |
| ns | 9954 |  | 31 | Listings of `changelog/` and `scripts/` | 5.5 |  | 0.530 |
| walker |  | 9976 | 67 | python imports in src/pluggy/_tracing.py |  |  | 0.530 |
| walker |  | 9994 | 18 | python decl names surface in docs/examples/eggsample/eggsample/__init__.py |  |  | 0.530 |

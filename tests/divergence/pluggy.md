Score(3000)=0.596 I=0.764 C=0.465 ns_rows≤3K=16/43 (reached=6 partial=3 missing=7)

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
| ns | 621 |  | 94 | Complete listings of `testing/` and `docs/` | 1.6 |  | 0.680 |
| walker |  | 705 | 119 | README headline in README.rst |  |  | 0.820 |
| ns | 845 |  | 224 | What pluggy is and what problem it solves (docs/index.rst lede) | 1.7 |  | 0.751 |
| ns | 901 |  | 56 | Complete listing of the two worked example packages under docs/examples/ | 1.8 |  | 0.704 |
| ns | 993 |  | 92 | Lazy `__version__` resolution in `__init__.py` | 1.9 | 1.5 | 0.672 |
| walker |  | 1251 | 546 | README.rst section #0 |  |  | 0.672 |
| walker |  | 1264 | 13 | listing of '.github' |  |  | 0.672 |
| walker |  | 1267 | 3 | listing of '.github/workflows' |  |  | 0.672 |
| walker |  | 1280 | 13 | listing of 'scripts' |  |  | 0.672 |
| ns | 1304 |  | 311 | Every method name on `PluginManager` (complete roster, names only) | 2.1 |  | 0.597 |
| walker |  | 1306 | 26 | downstream/README.md section #0 |  |  | 0.597 |
| walker |  | 1350 | 44 | dev/build/target dependencies in pyproject.toml |  |  | 0.597 |
| walker |  | 1401 | 51 | [package] in pyproject.toml |  |  | 0.597 |
| walker |  | 1442 | 41 | tool.setuptools_scm+uv config in pyproject.toml |  |  | 0.597 |
| walker |  | 1491 | 49 | tool.setuptools config in pyproject.toml |  |  | 0.597 |
| ns | 1536 |  | 232 | The rest of `_manager.py`: PluginValidationError, DistFacade, and the two module helpers | 2.2 |  | 0.555 |
| walker |  | 1554 | 63 | manifest config in pyproject.toml |  |  | 0.555 |
| walker |  | 1570 | 16 | listing of 'docs/examples' |  |  | 0.559 |
| walker |  | 1578 | 8 | listing of 'docs/examples/eggsample' |  |  | 0.565 |
| walker |  | 1656 | 78 | package metadata in pyproject.toml |  |  | 0.566 |
| walker |  | 1667 | 11 | python imports in src/pluggy/_warnings.py |  |  | 0.566 |
| walker |  | 1686 | 19 | listing of 'docs/examples/eggsample/eggsample' |  |  | 0.590 |
| walker |  | 1747 | 61 | listing of 'testing' |  |  | 0.663 |
| walker |  | 1767 | 20 | python decl names surface in src/pluggy/_warnings.py |  |  | 0.664 |
| walker |  | 1767 | 0 | python decl at src/pluggy/_warnings.py:4 |  |  | 0.664 |
| walker |  | 1783 | 16 | python decl at src/pluggy/_warnings.py:10 |  |  | 0.664 |
| walker |  | 1799 | 16 | python decl doc at src/pluggy/_warnings.py:4 |  |  | 0.664 |
| walker |  | 1810 | 11 | python class body at src/pluggy/_warnings.py:4 |  |  | 0.664 |
| walker |  | 1823 | 13 | python class body at src/pluggy/_warnings.py:10 |  |  | 0.664 |
| ns | 1865 |  | 329 | Every top-level symbol in `_hooks.py` (complete roster) plus the two backward-compat aliases | 2.3 |  | 0.614 |
| walker |  | 1870 | 47 | plaintext config docs/requirements.txt |  |  | 0.614 |
| walker |  | 1936 | 66 | python decl body at src/pluggy/__init__.py:32 body 33 |  |  | 0.641 |
| ns | 2027 |  | 162 | Every method name on `HookCaller` (complete roster, names only) | 2.4 |  | 0.616 |
| walker |  | 2172 | 236 | tool.mypy config in pyproject.toml |  |  | 0.616 |
| ns | 2309 |  | 282 | Complete `Result` API with signatures (`_result.py`) | 2.5 |  | 0.582 |
| walker |  | 2422 | 250 | tool.ruff config in pyproject.toml |  |  | 0.582 |
| walker |  | 2503 | 81 | python decl names surface in src/pluggy/_result.py |  |  | 0.583 |
| walker |  | 2503 | 0 | python decl at src/pluggy/_result.py:20 |  |  | 0.583 |
| walker |  | 2514 | 11 | python decl at src/pluggy/_result.py:24 |  |  | 0.583 |
| walker |  | 2523 | 9 | python decl doc at src/pluggy/_result.py:20 |  |  | 0.585 |
| walker |  | 2557 | 34 | python decl doc at src/pluggy/_result.py:24 |  |  | 0.589 |
| ns | 2571 |  | 262 | Complete top-level roster of `_callers.py` (the call loop module) | 2.6 |  | 0.561 |
| walker |  | 2686 | 129 | python method sigs in src/pluggy/_result.py |  |  | 0.588 |
| walker |  | 2686 | 0 | python method at src/pluggy/_result.py:67 |  |  | 0.588 |
| walker |  | 2686 | 0 | python method at src/pluggy/_result.py:80 |  |  | 0.588 |
| walker |  | 2686 | 0 | python method at src/pluggy/_result.py:91 |  |  | 0.588 |
| walker |  | 2694 | 8 | python method at src/pluggy/_result.py:42 |  |  | 0.591 |
| walker |  | 2702 | 8 | python method at src/pluggy/_result.py:51 |  |  | 0.594 |
| walker |  | 2710 | 8 | python method at src/pluggy/_result.py:56 |  |  | 0.598 |
| walker |  | 2722 | 12 | python method doc at src/pluggy/_result.py:42 |  |  | 0.598 |
| walker |  | 2734 | 12 | python method doc at src/pluggy/_result.py:51 |  |  | 0.598 |
| walker |  | 2746 | 12 | python method doc at src/pluggy/_result.py:56 |  |  | 0.598 |
| walker |  | 2786 | 40 | python method at src/pluggy/_result.py:31 |  |  | 0.598 |
| walker |  | 2798 | 12 | python method doc at src/pluggy/_result.py:31 |  |  | 0.598 |
| walker |  | 2816 | 18 | python class body at src/pluggy/_result.py:24 |  |  | 0.605 |
| ns | 2895 |  | 324 | Complete symbol rosters for the two remaining modules: `_tracing.py` and `_warnings.py` | 2.7 |  | 0.581 |
| walker |  | 2936 | 120 | python decl names surface in src/pluggy/_manager.py |  |  | 0.584 |
| walker |  | 2936 | 0 | python decl at src/pluggy/_manager.py:42 |  |  | 0.584 |
| walker |  | 2936 | 0 | python decl at src/pluggy/_manager.py:52 |  |  | 0.584 |
| walker |  | 2936 | 0 | python decl at src/pluggy/_manager.py:65 |  |  | 0.584 |
| walker |  | 2936 | 0 | python decl at src/pluggy/_manager.py:83 |  |  | 0.584 |
| walker |  | 2936 | 0 | python decl at src/pluggy/_manager.py:525 |  |  | 0.584 |
| walker |  | 2949 | 13 | python decl doc at src/pluggy/_manager.py:65 |  |  | 0.585 |
| walker |  | 2997 | 48 | python decl doc at src/pluggy/_manager.py:52 |  |  | 0.596 |
| walker |  | 3023 | 26 | python decl at src/pluggy/_manager.py:37 |  |  | 0.596 |
| walker |  | 3039 | 16 | python decl body at src/pluggy/_manager.py:525 body 526 |  |  | 0.600 |
| ns | 3306 |  | 411 | Complete section map of the 1082-line manual `docs/index.rst` | 2.8 |  | 0.549 |
| walker |  | 3384 | 345 | python decl names surface in src/pluggy/_hooks.py |  |  | 0.558 |
| walker |  | 3384 | 0 | python decl at src/pluggy/_hooks.py:40 |  |  | 0.558 |
| walker |  | 3384 | 0 | python decl at src/pluggy/_hooks.py:56 |  |  | 0.558 |
| walker |  | 3384 | 0 | python decl at src/pluggy/_hooks.py:281 |  |  | 0.558 |
| walker |  | 3384 | 0 | python decl at src/pluggy/_hooks.py:293 |  |  | 0.558 |
| walker |  | 3384 | 0 | python decl at src/pluggy/_hooks.py:382 |  |  | 0.558 |
| walker |  | 3384 | 0 | python decl at src/pluggy/_hooks.py:593 |  |  | 0.558 |
| walker |  | 3392 | 8 | python decl at src/pluggy/_hooks.py:358 |  |  | 0.560 |
| walker |  | 3400 | 8 | python decl at src/pluggy/_hooks.py:638 |  |  | 0.561 |
| walker |  | 3408 | 8 | python decl at src/pluggy/_hooks.py:696 |  |  | 0.563 |
| walker |  | 3417 | 9 | python decl at src/pluggy/_hooks.py:77 |  |  | 0.564 |
| walker |  | 3426 | 9 | python decl at src/pluggy/_hooks.py:164 |  |  | 0.566 |
| walker |  | 3438 | 12 | python decl doc at src/pluggy/_hooks.py:40 |  |  | 0.566 |
| walker |  | 3450 | 12 | python decl doc at src/pluggy/_hooks.py:56 |  |  | 0.566 |
| walker |  | 3467 | 17 | python decl doc at src/pluggy/_hooks.py:382 |  |  | 0.571 |
| walker |  | 3485 | 18 | python decl doc at src/pluggy/_hooks.py:638 |  |  | 0.575 |
| walker |  | 3516 | 31 | python decl doc at src/pluggy/_hooks.py:358 |  |  | 0.584 |
| ns | 3551 |  | 245 | The real call signatures of `@hookspec` and `@hookimpl` — every accepted option with its default | 3.1 |  | 0.563 |
| walker |  | 3628 | 112 | python class body at src/pluggy/_hooks.py:638 |  |  | 0.563 |
| walker |  | 3716 | 88 | python class body at src/pluggy/_hooks.py:696 |  |  | 0.565 |
| ns | 3799 |  | 248 | `HookimplOpts` in full: every hook-implementation option and what it means | 3.2 | 2.3 | 0.546 |
| walker |  | 3808 | 92 | python class body at src/pluggy/_hooks.py:382 |  |  | 0.546 |
| walker |  | 3832 | 24 | python decl at src/pluggy/_hooks.py:377 |  |  | 0.546 |
| ns | 3983 |  | 184 | `HookspecOpts` in full: every hook-specification option | 3.3 | 2.3 | 0.534 |
| walker |  | 4056 | 224 | python class body at src/pluggy/_hooks.py:56 |  |  | 0.569 |
| walker |  | 4218 | 162 | python class body at src/pluggy/_hooks.py:40 |  |  | 0.594 |
| walker |  | 4234 | 16 | python class body at src/pluggy/_hooks.py:77 |  |  | 0.594 |
| walker |  | 4250 | 16 | python class body at src/pluggy/_hooks.py:164 |  |  | 0.594 |
| walker |  | 4282 | 32 | python decl doc at src/pluggy/_hooks.py:593 |  |  | 0.603 |
| walker |  | 4334 | 52 | python class body at src/pluggy/_hooks.py:358 |  |  | 0.603 |
| ns | 4415 |  | 432 | Hook implementation ordering: the `_hookimpls` layout comment and `_add_hookimpl` in full | 3.4 | 2.4 | 0.573 |
| ns | 4621 |  | 206 | `HookCaller.__call__` in full: what `pm.hook.myhook(...)` actually does | 3.5 | 2.4 | 0.560 |
| walker |  | 4852 | 518 | python method sigs in src/pluggy/_hooks.py |  |  | 0.586 |
| walker |  | 4852 | 0 | python method at src/pluggy/_hooks.py:88 |  |  | 0.586 |
| walker |  | 4852 | 0 | python method at src/pluggy/_hooks.py:175 |  |  | 0.586 |
| walker |  | 4852 | 0 | python method at src/pluggy/_hooks.py:365 |  |  | 0.586 |
| walker |  | 4852 | 0 | python method at src/pluggy/_hooks.py:420 |  |  | 0.586 |
| walker |  | 4852 | 0 | python method at src/pluggy/_hooks.py:438 |  |  | 0.586 |
| walker |  | 4852 | 0 | python method at src/pluggy/_hooks.py:442 |  |  | 0.586 |
| walker |  | 4852 | 0 | python method at src/pluggy/_hooks.py:449 |  |  | 0.586 |
| walker |  | 4852 | 0 | python method at src/pluggy/_hooks.py:453 |  |  | 0.586 |
| walker |  | 4852 | 0 | python method at src/pluggy/_hooks.py:477 |  |  | 0.586 |
| walker |  | 4852 | 0 | python method at src/pluggy/_hooks.py:499 |  |  | 0.586 |
| walker |  | 4852 | 0 | python method at src/pluggy/_hooks.py:577 |  |  | 0.586 |
| walker |  | 4852 | 0 | python method at src/pluggy/_hooks.py:612 |  |  | 0.586 |
| walker |  | 4852 | 0 | python method at src/pluggy/_hooks.py:634 |  |  | 0.586 |
| walker |  | 4852 | 0 | python method at src/pluggy/_hooks.py:692 |  |  | 0.586 |
| walker |  | 4852 | 0 | python method at src/pluggy/_hooks.py:709 |  |  | 0.586 |
| walker |  | 4860 | 8 | python method at src/pluggy/_hooks.py:626 |  |  | 0.586 |
| walker |  | 4868 | 8 | python method at src/pluggy/_hooks.py:630 |  |  | 0.586 |
| walker |  | 4878 | 10 | python method doc at src/pluggy/_hooks.py:365 |  |  | 0.586 |
| walker |  | 4909 | 31 | python method at src/pluggy/_hooks.py:543 |  |  | 0.586 |
| walker |  | 4924 | 15 | python method doc at src/pluggy/_hooks.py:449 |  |  | 0.586 |
| walker |  | 4939 | 15 | python method at src/pluggy/_hooks.py:618 |  |  | 0.586 |
| walker |  | 4957 | 18 | python method doc at src/pluggy/_hooks.py:438 |  |  | 0.586 |
| walker |  | 4998 | 41 | python method at src/pluggy/_hooks.py:424 |  |  | 0.586 |
| walker |  | 5015 | 17 | python method at src/pluggy/_hooks.py:480 |  |  | 0.586 |
| walker |  | 5067 | 52 | python method at src/pluggy/_hooks.py:516 |  |  | 0.586 |
| walker |  | 5081 | 14 | python method doc at src/pluggy/_hooks.py:453 |  |  | 0.587 |
| ns | 5086 |  | 465 | `_multicall` part 1: the setup / non-wrapper call loop | 3.6 |  | 0.555 |
| walker |  | 5144 | 63 | python method at src/pluggy/_hooks.py:656 |  |  | 0.555 |
| walker |  | 5156 | 12 | python method doc at src/pluggy/_hooks.py:656 |  |  | 0.555 |
| walker |  | 5236 | 80 | python decl doc at src/pluggy/_hooks.py:164 |  |  | 0.555 |
| walker |  | 5317 | 81 | python decl doc at src/pluggy/_hooks.py:77 |  |  | 0.555 |
| walker |  | 5351 | 34 | python decl at src/pluggy/_hooks.py:33 |  |  | 0.555 |
| walker |  | 5421 | 70 | python method at src/pluggy/_hooks.py:393 |  |  | 0.555 |
| walker |  | 5433 | 12 | python method doc at src/pluggy/_hooks.py:393 |  |  | 0.555 |
| walker |  | 5454 | 21 | python method doc at src/pluggy/_hooks.py:577 |  |  | 0.555 |
| walker |  | 5499 | 45 | python method doc at src/pluggy/_hooks.py:543 |  |  | 0.555 |
| ns | 5529 |  | 443 | `_multicall` part 2: the teardown loop, exception routing and `firstresult` collapse | 3.7 | 3.6 | 0.525 |
| ns | 5685 |  | 156 | `PluginManager.register` docstring: naming, blocking and the duplicate-registration contract | 3.8 | 2.1 | 0.517 |
| ns | 5981 |  | 296 | `PluginManager.register` body: how hook implementations are discovered and attached | 3.9 | 3.8 | 0.505 |
| walker |  | 6114 | 615 | python method sigs in src/pluggy/_manager.py |  |  | 0.560 |
| walker |  | 6114 | 0 | python method at src/pluggy/_manager.py:59 |  |  | 0.560 |
| walker |  | 6114 | 0 | python method at src/pluggy/_manager.py:68 |  |  | 0.560 |
| walker |  | 6114 | 0 | python method at src/pluggy/_manager.py:76 |  |  | 0.560 |
| walker |  | 6114 | 0 | python method at src/pluggy/_manager.py:79 |  |  | 0.560 |
| walker |  | 6114 | 0 | python method at src/pluggy/_manager.py:125 |  |  | 0.560 |
| walker |  | 6114 | 0 | python method at src/pluggy/_manager.py:176 |  |  | 0.560 |
| walker |  | 6114 | 0 | python method at src/pluggy/_manager.py:233 |  |  | 0.560 |
| walker |  | 6114 | 0 | python method at src/pluggy/_manager.py:238 |  |  | 0.560 |
| walker |  | 6114 | 0 | python method at src/pluggy/_manager.py:242 |  |  | 0.560 |
| walker |  | 6114 | 0 | python method at src/pluggy/_manager.py:252 |  |  | 0.560 |
| walker |  | 6114 | 0 | python method at src/pluggy/_manager.py:296 |  |  | 0.560 |
| walker |  | 6114 | 0 | python method at src/pluggy/_manager.py:300 |  |  | 0.560 |
| walker |  | 6114 | 0 | python method at src/pluggy/_manager.py:304 |  |  | 0.560 |
| walker |  | 6114 | 0 | python method at src/pluggy/_manager.py:315 |  |  | 0.560 |
| walker |  | 6114 | 0 | python method at src/pluggy/_manager.py:319 |  |  | 0.560 |
| walker |  | 6114 | 0 | python method at src/pluggy/_manager.py:323 |  |  | 0.560 |
| walker |  | 6114 | 0 | python method at src/pluggy/_manager.py:331 |  |  | 0.560 |
| walker |  | 6114 | 0 | python method at src/pluggy/_manager.py:379 |  |  | 0.560 |
| walker |  | 6114 | 0 | python method at src/pluggy/_manager.py:395 |  |  | 0.560 |
| walker |  | 6114 | 0 | python method at src/pluggy/_manager.py:425 |  |  | 0.560 |
| walker |  | 6114 | 0 | python method at src/pluggy/_manager.py:430 |  |  | 0.560 |
| walker |  | 6114 | 0 | python method at src/pluggy/_manager.py:434 |  |  | 0.560 |
| walker |  | 6114 | 0 | python method at src/pluggy/_manager.py:487 |  |  | 0.560 |
| walker |  | 6122 | 8 | python method at src/pluggy/_manager.py:71 |  |  | 0.562 |
| walker |  | 6132 | 10 | python method at src/pluggy/_manager.py:100 |  |  | 0.562 |
| walker |  | 6160 | 28 | python method at src/pluggy/_manager.py:512 |  |  | 0.562 |
| walker |  | 6174 | 14 | python method doc at src/pluggy/_manager.py:300 |  |  | 0.562 |
| walker |  | 6205 | 31 | python method at src/pluggy/_manager.py:278 |  |  | 0.562 |
| walker |  | 6236 | 31 | python method at src/pluggy/_manager.py:450 |  |  | 0.562 |
| walker |  | 6251 | 15 | python method doc at src/pluggy/_manager.py:238 |  |  | 0.562 |
| walker |  | 6266 | 15 | python method doc at src/pluggy/_manager.py:296 |  |  | 0.562 |
| walker |  | 6301 | 35 | python method at src/pluggy/_manager.py:201 |  |  | 0.562 |
| walker |  | 6318 | 17 | python method doc at src/pluggy/_manager.py:319 |  |  | 0.562 |
| walker |  | 6336 | 18 | python method doc at src/pluggy/_manager.py:233 |  |  | 0.562 |
| walker |  | 6354 | 18 | python method doc at src/pluggy/_manager.py:315 |  |  | 0.562 |
| walker |  | 6375 | 21 | python method doc at src/pluggy/_manager.py:430 |  |  | 0.562 |
| walker |  | 6404 | 29 | python method doc at src/pluggy/_manager.py:323 |  |  | 0.562 |
| walker |  | 6434 | 30 | python method doc at src/pluggy/_manager.py:425 |  |  | 0.562 |
| ns | 6439 |  | 458 | `PluginManager._verify_hook`: every validation error message pluggy can raise | 3.10 | 2.1 | 0.542 |
| walker |  | 6469 | 35 | python method doc at src/pluggy/_manager.py:242 |  |  | 0.542 |
| walker |  | 6511 | 42 | python method doc at src/pluggy/_manager.py:487 |  |  | 0.542 |
| walker |  | 6555 | 44 | python method doc at src/pluggy/_manager.py:379 |  |  | 0.542 |
| walker |  | 6634 | 79 | python decl doc at src/pluggy/_hooks.py:293 |  |  | 0.542 |
| ns | 6635 |  | 196 | `PluginManager.__init__`: the complete state of a plugin manager | 3.11 | 2.1 | 0.536 |
| walker |  | 6644 | 10 | python imports in docs/examples/eggsample/eggsample/__init__.py |  |  | 0.536 |
| walker |  | 6694 | 50 | python method doc at src/pluggy/_manager.py:512 |  |  | 0.536 |
| ns | 7001 |  | 366 | Historic hooks end to end: `set_specification`, `call_historic`, `_maybe_apply_history` | 3.12 | 2.4 | 0.523 |
| walker |  | 7086 | 392 | tool.towncrier config in pyproject.toml |  |  | 0.523 |
| walker |  | 7147 | 61 | python method doc at src/pluggy/_result.py:80 |  |  | 0.523 |
| ns | 7201 |  | 200 | Complete attribute sets of `HookImpl` and `HookSpec` (`__slots__`) | 3.13 |  | 0.538 |
| walker |  | 7244 | 97 | python method at src/pluggy/_hooks.py:111 |  |  | 0.544 |
| walker |  | 7307 | 63 | python method doc at src/pluggy/_manager.py:252 |  |  | 0.544 |
| walker |  | 7370 | 63 | python method doc at src/pluggy/_result.py:91 |  |  | 0.544 |
| walker |  | 7435 | 65 | python method doc at src/pluggy/_manager.py:434 |  |  | 0.544 |
| ns | 7511 |  | 310 | `load_setuptools_entrypoints`: how third-party plugins are discovered | 3.14 | 2.1 | 0.530 |
| walker |  | 7540 | 105 | python method at src/pluggy/_hooks.py:101 |  |  | 0.530 |
| walker |  | 7540 | 0 | python method body at src/pluggy/_hooks.py:101 body 109 |  |  | 0.530 |
| walker |  | 7644 | 104 | python method at src/pluggy/_hooks.py:91 |  |  | 0.531 |
| walker |  | 7644 | 0 | python method body at src/pluggy/_hooks.py:91 body 99 |  |  | 0.531 |
| ns | 7705 |  | 194 | Blocking semantics: `set_blocked`, `is_blocked`, `unblock` | 3.15 | 2.1 | 0.528 |
| walker |  | 7756 | 112 | python method at src/pluggy/_hooks.py:202 |  |  | 0.545 |
| walker |  | 7833 | 77 | python method doc at src/pluggy/_hooks.py:499 |  |  | 0.549 |
| ns | 7937 |  | 232 | `docs/api_reference.rst`: exactly which types are publicly documented | 4.1 |  | 0.542 |
| walker |  | 7953 | 120 | python method at src/pluggy/_hooks.py:190 |  |  | 0.542 |
| walker |  | 7953 | 0 | python method body at src/pluggy/_hooks.py:190 body 200 |  |  | 0.542 |
| walker |  | 8072 | 119 | python method at src/pluggy/_hooks.py:178 |  |  | 0.543 |
| walker |  | 8072 | 0 | python method body at src/pluggy/_hooks.py:178 body 188 |  |  | 0.543 |
| walker |  | 8159 | 87 | python method doc at src/pluggy/_manager.py:201 |  |  | 0.543 |
| walker |  | 8199 | 40 | declaration surface of changelog/590.trivial.rst |  |  | 0.543 |
| ns | 8267 |  | 330 | `docs/examples/toy-example.py`: the canonical end-to-end usage | 4.2 |  | 0.528 |
| walker |  | 8296 | 97 | declaration surface of tox.ini |  |  | 0.528 |
| walker |  | 8485 | 189 | python decl doc at src/pluggy/_manager.py:83 |  |  | 0.528 |
| ns | 8548 |  | 281 | The eggsample host program: wiring a PluginManager and calling a hook | 4.3 |  | 0.518 |
| walker |  | 8674 | 189 | python decl doc at src/pluggy/_warnings.py:10 |  |  | 0.520 |
| ns | 8768 |  | 220 | `eggsample/hookspecs.py` in full: what a real hookspec module looks like | 4.4 |  | 0.511 |
| walker |  | 8771 | 97 | python method doc at src/pluggy/_manager.py:304 |  |  | 0.511 |
| walker |  | 8869 | 98 | python method doc at src/pluggy/_hooks.py:516 |  |  | 0.511 |
| walker |  | 8968 | 99 | python method doc at src/pluggy/_result.py:67 |  |  | 0.511 |
| walker |  | 9071 | 103 | python method doc at src/pluggy/_manager.py:395 |  |  | 0.516 |
| ns | 9104 |  | 336 | Both sides of hook implementation: the host's own `lib.py` and the external plugin `eggsample_spam.py` | 4.5 |  | 0.504 |
| ns | 9247 |  | 143 | Entry-point wiring in both example `setup.py` files | 4.6 |  | 0.500 |
| walker |  | 9287 | 216 | python class body at src/pluggy/_hooks.py:593 |  |  | 0.500 |
| walker |  | 9367 | 80 | python decl names surface in src/pluggy/_callers.py |  |  | 0.502 |
| walker |  | 9398 | 31 | python decl at src/pluggy/_callers.py:27 |  |  | 0.503 |
| walker |  | 9429 | 31 | python decl doc at src/pluggy/_callers.py:27 |  |  | 0.503 |
| walker |  | 9458 | 29 | python decl at src/pluggy/_callers.py:70 |  |  | 0.505 |
| walker |  | 9492 | 34 | python decl at src/pluggy/_callers.py:60 |  |  | 0.508 |
| ns | 9495 |  | 248 | `testing/conftest.py` in full: the two fixtures every test in the suite uses | 5.1 |  | 0.499 |
| walker |  | 9551 | 59 | python decl at src/pluggy/_callers.py:82 |  |  | 0.507 |
| walker |  | 9618 | 67 | python decl names surface in src/pluggy/_tracing.py |  |  | 0.508 |
| walker |  | 9618 | 0 | python decl at src/pluggy/_tracing.py:16 |  |  | 0.508 |
| walker |  | 9618 | 0 | python decl at src/pluggy/_tracing.py:59 |  |  | 0.508 |
| ns | 9654 |  | 159 | `pyproject.toml`: package identity, Python floor, dependency groups and the src layout | 5.2 |  | 0.512 |
| walker |  | 9828 | 210 | python method sigs in src/pluggy/_tracing.py |  |  | 0.528 |
| walker |  | 9828 | 0 | python method at src/pluggy/_tracing.py:17 |  |  | 0.528 |
| walker |  | 9828 | 0 | python method at src/pluggy/_tracing.py:22 |  |  | 0.528 |
| walker |  | 9828 | 0 | python method at src/pluggy/_tracing.py:25 |  |  | 0.528 |
| walker |  | 9828 | 0 | python method at src/pluggy/_tracing.py:42 |  |  | 0.528 |
| walker |  | 9828 | 0 | python method at src/pluggy/_tracing.py:48 |  |  | 0.528 |
| walker |  | 9828 | 0 | python method at src/pluggy/_tracing.py:51 |  |  | 0.528 |
| walker |  | 9828 | 0 | python method at src/pluggy/_tracing.py:60 |  |  | 0.528 |
| walker |  | 9828 | 0 | python method at src/pluggy/_tracing.py:64 |  |  | 0.528 |
| walker |  | 9828 | 0 | python method at src/pluggy/_tracing.py:67 |  |  | 0.528 |
| ns | 9833 |  | 179 | `tox.ini`: the environment list and the embedded pytest configuration | 5.3 |  | 0.523 |
| walker |  | 9837 | 9 | python method body at src/pluggy/_tracing.py:48 body 49 |  |  | 0.523 |
| walker |  | 9849 | 12 | python method body at src/pluggy/_tracing.py:22 body 23 |  |  | 0.523 |
| walker |  | 9862 | 13 | python method body at src/pluggy/_tracing.py:64 body 65 |  |  | 0.523 |
| walker |  | 9879 | 17 | python method body at src/pluggy/_tracing.py:67 body 68 |  |  | 0.523 |
| ns | 9923 |  | 90 | `[tool.towncrier]` config: how CHANGELOG.rst is produced | 5.4 |  | 0.526 |
| ns | 9954 |  | 31 | Listings of `changelog/` and `scripts/` | 5.5 |  | 0.529 |

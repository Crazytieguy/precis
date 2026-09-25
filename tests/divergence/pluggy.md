Score(3000)=0.590 I=0.760 C=0.458 ns_rows≤3K=16/43 grid(1000/1442/2080/3000/4327/6240/9000)=0.672/0.608/0.609/0.590/0.587/0.535/0.531

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 62 |  | 62 | Repository identity: README title, dependents, and the docs tagline | 1.1 |  | 0.000 |
| walker |  | 96 | 96 | listing of '.' |  |  | 0.000 |
| walker |  | 100 | 4 | listing of 'src' |  |  | 0.000 |
| ns | 107 |  | 45 | Complete listing of the shipped package directory `src/pluggy/` | 1.2 |  | 0.000 |
| walker |  | 119 | 19 | listing of 'changelog' |  |  | 0.000 |
| walker |  | 123 | 4 | listing of '.claude' |  |  | 0.000 |
| walker |  | 155 | 32 | listing of 'docs' |  |  | 0.000 |
| walker |  | 158 | 3 | listing of 'docs/_static' |  |  | 0.000 |
| walker |  | 162 | 4 | listing of 'docs/_static/img' |  |  | 0.000 |
| ns | 203 |  | 96 | Complete repository root listing | 1.3 |  | 0.445 |
| walker |  | 205 | 43 | listing of 'downstream' |  |  | 0.445 |
| walker |  | 250 | 45 | listing of 'src/pluggy' |  |  | 0.723 |
| ns | 355 |  | 152 | The complete public API name list (`__all__` of src/pluggy/__init__.py) | 1.4 |  | 0.577 |
| ns | 521 |  | 166 | Public name to private module map (the re-export block) | 1.5 | 1.4 | 0.504 |
| walker |  | 568 | 318 | python imports in src/pluggy/__init__.py |  |  | 0.770 |
| walker |  | 584 | 16 | python decl names surface in src/pluggy/__init__.py |  |  | 0.771 |
| walker |  | 584 | 0 | python decl at src/pluggy/__init__.py:32 |  |  | 0.771 |
| walker |  | 597 | 13 | listing of '.github' |  |  | 0.771 |
| walker |  | 601 | 4 | listing of '.github/workflows' |  |  | 0.771 |
| walker |  | 615 | 14 | listing of 'scripts' |  |  | 0.680 |
| ns | 615 |  | 94 | Complete listings of `testing/` and `docs/` | 1.6 |  | 0.680 |
| walker |  | 734 | 119 | README headline in README.rst |  |  | 0.820 |
| ns | 839 |  | 224 | What pluggy is and what problem it solves (docs/index.rst lede) | 1.7 |  | 0.752 |
| ns | 893 |  | 54 | Complete listing of the two worked example packages under docs/examples/ | 1.8 |  | 0.705 |
| ns | 985 |  | 92 | Lazy `__version__` resolution in `__init__.py` | 1.9 | 1.5 | 0.672 |
| walker |  | 1280 | 546 | README.rst section #0 |  |  | 0.672 |
| ns | 1296 |  | 311 | Every method name on `PluginManager` (complete roster, names only) | 2.1 |  | 0.597 |
| walker |  | 1306 | 26 | downstream/README.md section #0 |  |  | 0.597 |
| walker |  | 1321 | 15 | listing of 'docs/examples' |  |  | 0.601 |
| walker |  | 1329 | 8 | listing of 'docs/examples/eggsample' |  |  | 0.608 |
| walker |  | 1382 | 53 | [package] in pyproject.toml |  |  | 0.608 |
| walker |  | 1423 | 41 | tool.setuptools_scm+uv config in pyproject.toml |  |  | 0.608 |
| walker |  | 1474 | 51 | tool.setuptools config in pyproject.toml |  |  | 0.609 |
| ns | 1528 |  | 232 | The rest of `_manager.py`: PluginValidationError, DistFacade, and the two module helpers | 2.2 |  | 0.565 |
| walker |  | 1537 | 63 | manifest config in pyproject.toml |  |  | 0.565 |
| walker |  | 1617 | 80 | package metadata in pyproject.toml |  |  | 0.565 |
| walker |  | 1679 | 62 | listing of 'testing' |  |  | 0.640 |
| walker |  | 1690 | 11 | python imports in src/pluggy/_warnings.py |  |  | 0.640 |
| walker |  | 1710 | 20 | listing of 'docs/examples/eggsample/eggsample' |  |  | 0.663 |
| walker |  | 1730 | 20 | python decl names surface in src/pluggy/_warnings.py |  |  | 0.663 |
| walker |  | 1730 | 0 | python decl at src/pluggy/_warnings.py:4 |  |  | 0.663 |
| walker |  | 1746 | 16 | python decl at src/pluggy/_warnings.py:10 |  |  | 0.663 |
| walker |  | 1762 | 16 | python decl doc at src/pluggy/_warnings.py:4 |  |  | 0.663 |
| walker |  | 1773 | 11 | python class body at src/pluggy/_warnings.py:4 |  |  | 0.663 |
| walker |  | 1849 | 76 | python decl names surface in src/pluggy/_hooks.py |  |  | 0.663 |
| walker |  | 1849 | 0 | python decl at src/pluggy/_hooks.py:40 |  |  | 0.663 |
| walker |  | 1849 | 0 | python decl at src/pluggy/_hooks.py:56 |  |  | 0.663 |
| walker |  | 1849 | 0 | python decl at src/pluggy/_hooks.py:382 |  |  | 0.663 |
| walker |  | 1857 | 8 | python decl at src/pluggy/_hooks.py:358 |  |  | 0.615 |
| ns | 1857 |  | 329 | Every top-level symbol in `_hooks.py` (complete roster) plus the two backward-compat aliases | 2.3 |  | 0.615 |
| walker |  | 1865 | 8 | python decl at src/pluggy/_hooks.py:638 |  |  | 0.616 |
| walker |  | 1874 | 9 | python decl at src/pluggy/_hooks.py:77 |  |  | 0.617 |
| walker |  | 1883 | 9 | python decl at src/pluggy/_hooks.py:164 |  |  | 0.619 |
| walker |  | 1895 | 12 | python decl doc at src/pluggy/_hooks.py:40 |  |  | 0.619 |
| walker |  | 1907 | 12 | python decl doc at src/pluggy/_hooks.py:56 |  |  | 0.619 |
| walker |  | 1924 | 17 | python decl doc at src/pluggy/_hooks.py:382 |  |  | 0.622 |
| walker |  | 1942 | 18 | python decl doc at src/pluggy/_hooks.py:638 |  |  | 0.626 |
| walker |  | 1973 | 31 | python decl doc at src/pluggy/_hooks.py:358 |  |  | 0.634 |
| ns | 2019 |  | 162 | Every method name on `HookCaller` (complete roster, names only) | 2.4 |  | 0.609 |
| walker |  | 2232 | 259 | python decl names surface #1 in src/pluggy/_hooks.py |  |  | 0.634 |
| walker |  | 2232 | 0 | python decl at src/pluggy/_hooks.py:281 |  |  | 0.634 |
| walker |  | 2232 | 0 | python decl at src/pluggy/_hooks.py:293 |  |  | 0.634 |
| walker |  | 2232 | 0 | python decl at src/pluggy/_hooks.py:593 |  |  | 0.634 |
| walker |  | 2240 | 8 | python decl at src/pluggy/_hooks.py:696 |  |  | 0.638 |
| walker |  | 2264 | 24 | python decl at src/pluggy/_hooks.py:377 |  |  | 0.638 |
| ns | 2301 |  | 282 | Complete `Result` API with signatures (`_result.py`) | 2.5 |  | 0.603 |
| ns | 2563 |  | 262 | Complete top-level roster of `_callers.py` (the call loop module) | 2.6 |  | 0.574 |
| walker |  | 2798 | 534 | python method sigs in src/pluggy/_hooks.py |  |  | 0.614 |
| walker |  | 2798 | 0 | python method at src/pluggy/_hooks.py:88 |  |  | 0.614 |
| walker |  | 2798 | 0 | python method at src/pluggy/_hooks.py:175 |  |  | 0.614 |
| walker |  | 2798 | 0 | python method at src/pluggy/_hooks.py:365 |  |  | 0.614 |
| walker |  | 2798 | 0 | python method at src/pluggy/_hooks.py:420 |  |  | 0.614 |
| walker |  | 2798 | 0 | python method at src/pluggy/_hooks.py:438 |  |  | 0.614 |
| walker |  | 2798 | 0 | python method at src/pluggy/_hooks.py:442 |  |  | 0.614 |
| walker |  | 2798 | 0 | python method at src/pluggy/_hooks.py:449 |  |  | 0.614 |
| walker |  | 2798 | 0 | python method at src/pluggy/_hooks.py:453 |  |  | 0.614 |
| walker |  | 2798 | 0 | python method at src/pluggy/_hooks.py:477 |  |  | 0.614 |
| walker |  | 2798 | 0 | python method at src/pluggy/_hooks.py:499 |  |  | 0.614 |
| walker |  | 2798 | 0 | python method at src/pluggy/_hooks.py:577 |  |  | 0.614 |
| walker |  | 2798 | 0 | python method at src/pluggy/_hooks.py:612 |  |  | 0.614 |
| walker |  | 2798 | 0 | python method at src/pluggy/_hooks.py:634 |  |  | 0.614 |
| walker |  | 2798 | 0 | python method at src/pluggy/_hooks.py:692 |  |  | 0.614 |
| walker |  | 2798 | 0 | python method at src/pluggy/_hooks.py:709 |  |  | 0.614 |
| walker |  | 2806 | 8 | python method at src/pluggy/_hooks.py:626 |  |  | 0.614 |
| walker |  | 2814 | 8 | python method at src/pluggy/_hooks.py:630 |  |  | 0.614 |
| walker |  | 2845 | 31 | python method at src/pluggy/_hooks.py:543 |  |  | 0.614 |
| walker |  | 2860 | 15 | python method at src/pluggy/_hooks.py:618 |  |  | 0.614 |
| ns | 2887 |  | 324 | Complete symbol rosters for the two remaining modules: `_tracing.py` and `_warnings.py` | 2.7 |  | 0.590 |
| walker |  | 2901 | 41 | python method at src/pluggy/_hooks.py:424 |  |  | 0.590 |
| walker |  | 2918 | 17 | python method at src/pluggy/_hooks.py:480 |  |  | 0.590 |
| walker |  | 2970 | 52 | python method at src/pluggy/_hooks.py:516 |  |  | 0.590 |
| walker |  | 2982 | 12 | python method doc at src/pluggy/_hooks.py:365 |  |  | 0.590 |
| walker |  | 2997 | 15 | python method doc at src/pluggy/_hooks.py:449 |  |  | 0.590 |
| walker |  | 3060 | 63 | python method at src/pluggy/_hooks.py:656 |  |  | 0.590 |
| walker |  | 3072 | 12 | python method doc at src/pluggy/_hooks.py:656 |  |  | 0.590 |
| walker |  | 3106 | 34 | python decl at src/pluggy/_hooks.py:33 |  |  | 0.590 |
| walker |  | 3176 | 70 | python method at src/pluggy/_hooks.py:393 |  |  | 0.590 |
| walker |  | 3188 | 12 | python method doc at src/pluggy/_hooks.py:393 |  |  | 0.590 |
| walker |  | 3206 | 18 | python method doc at src/pluggy/_hooks.py:438 |  |  | 0.590 |
| ns | 3298 |  | 411 | Complete section map of the 1082-line manual `docs/index.rst` | 2.8 |  | 0.540 |
| walker |  | 3316 | 110 | python class body at src/pluggy/_hooks.py:638 |  |  | 0.540 |
| walker |  | 3402 | 86 | python class body at src/pluggy/_hooks.py:696 |  |  | 0.542 |
| walker |  | 3488 | 86 | python class body at src/pluggy/_hooks.py:382 |  |  | 0.542 |
| ns | 3543 |  | 245 | The real call signatures of `@hookspec` and `@hookimpl` — every accepted option with its default | 3.1 |  | 0.522 |
| walker |  | 3585 | 97 | python method at src/pluggy/_hooks.py:111 |  |  | 0.533 |
| walker |  | 3598 | 13 | python class body at src/pluggy/_warnings.py:10 |  |  | 0.533 |
| walker |  | 3703 | 105 | python method at src/pluggy/_hooks.py:101 |  |  | 0.533 |
| walker |  | 3703 | 0 | python method body at src/pluggy/_hooks.py:101 body 109 |  |  | 0.533 |
| ns | 3791 |  | 248 | `HookimplOpts` in full: every hook-implementation option and what it means | 3.2 | 2.3 | 0.516 |
| walker |  | 3807 | 104 | python method at src/pluggy/_hooks.py:91 |  |  | 0.517 |
| walker |  | 3807 | 0 | python method body at src/pluggy/_hooks.py:91 body 99 |  |  | 0.517 |
| walker |  | 3821 | 14 | python method doc at src/pluggy/_hooks.py:453 |  |  | 0.517 |
| ns | 3975 |  | 184 | `HookspecOpts` in full: every hook-specification option | 3.3 | 2.3 | 0.505 |
| walker |  | 4045 | 224 | python class body at src/pluggy/_hooks.py:56 |  |  | 0.542 |
| walker |  | 4059 | 14 | python class body at src/pluggy/_hooks.py:77 |  |  | 0.542 |
| walker |  | 4073 | 14 | python class body at src/pluggy/_hooks.py:164 |  |  | 0.542 |
| walker |  | 4139 | 66 | python decl body at src/pluggy/__init__.py:32 body 33 |  |  | 0.559 |
| walker |  | 4251 | 112 | python method at src/pluggy/_hooks.py:202 |  |  | 0.587 |
| ns | 4407 |  | 432 | Hook implementation ordering: the `_hookimpls` layout comment and `_add_hookimpl` in full | 3.4 | 2.4 | 0.558 |
| walker |  | 4413 | 162 | python class body at src/pluggy/_hooks.py:40 |  |  | 0.581 |
| walker |  | 4533 | 120 | python method at src/pluggy/_hooks.py:190 |  |  | 0.581 |
| walker |  | 4533 | 0 | python method body at src/pluggy/_hooks.py:190 body 200 |  |  | 0.581 |
| ns | 4613 |  | 206 | `HookCaller.__call__` in full: what `pm.hook.myhook(...)` actually does | 3.5 | 2.4 | 0.567 |
| walker |  | 4652 | 119 | python method at src/pluggy/_hooks.py:178 |  |  | 0.569 |
| walker |  | 4652 | 0 | python method body at src/pluggy/_hooks.py:178 body 188 |  |  | 0.569 |
| walker |  | 4700 | 48 | python class body at src/pluggy/_hooks.py:358 |  |  | 0.569 |
| walker |  | 4732 | 32 | python decl doc at src/pluggy/_hooks.py:593 |  |  | 0.578 |
| walker |  | 4812 | 80 | python decl doc at src/pluggy/_hooks.py:164 |  |  | 0.578 |
| walker |  | 4893 | 81 | python decl doc at src/pluggy/_hooks.py:77 |  |  | 0.578 |
| walker |  | 4974 | 81 | python decl names surface in src/pluggy/_result.py |  |  | 0.578 |
| walker |  | 4974 | 0 | python decl at src/pluggy/_result.py:20 |  |  | 0.578 |
| walker |  | 4985 | 11 | python decl at src/pluggy/_result.py:24 |  |  | 0.579 |
| walker |  | 4994 | 9 | python decl doc at src/pluggy/_result.py:20 |  |  | 0.579 |
| walker |  | 5028 | 34 | python decl doc at src/pluggy/_result.py:24 |  |  | 0.582 |
| ns | 5078 |  | 465 | `_multicall` part 1: the setup / non-wrapper call loop | 3.6 |  | 0.551 |
| walker |  | 5157 | 129 | python method sigs in src/pluggy/_result.py |  |  | 0.566 |
| walker |  | 5157 | 0 | python method at src/pluggy/_result.py:67 |  |  | 0.566 |
| walker |  | 5157 | 0 | python method at src/pluggy/_result.py:80 |  |  | 0.566 |
| walker |  | 5157 | 0 | python method at src/pluggy/_result.py:91 |  |  | 0.566 |
| walker |  | 5165 | 8 | python method at src/pluggy/_result.py:42 |  |  | 0.568 |
| walker |  | 5173 | 8 | python method at src/pluggy/_result.py:51 |  |  | 0.570 |
| walker |  | 5181 | 8 | python method at src/pluggy/_result.py:56 |  |  | 0.572 |
| walker |  | 5193 | 12 | python method doc at src/pluggy/_result.py:42 |  |  | 0.572 |
| walker |  | 5205 | 12 | python method doc at src/pluggy/_result.py:51 |  |  | 0.572 |
| walker |  | 5217 | 12 | python method doc at src/pluggy/_result.py:56 |  |  | 0.572 |
| walker |  | 5257 | 40 | python method at src/pluggy/_result.py:31 |  |  | 0.572 |
| walker |  | 5269 | 12 | python method doc at src/pluggy/_result.py:31 |  |  | 0.572 |
| walker |  | 5287 | 18 | python class body at src/pluggy/_result.py:24 |  |  | 0.576 |
| walker |  | 5308 | 21 | python method doc at src/pluggy/_hooks.py:577 |  |  | 0.576 |
| walker |  | 5353 | 45 | python method doc at src/pluggy/_hooks.py:543 |  |  | 0.576 |
| walker |  | 5432 | 79 | python decl doc at src/pluggy/_hooks.py:293 |  |  | 0.576 |
| ns | 5521 |  | 443 | `_multicall` part 2: the teardown loop, exception routing and `firstresult` collapse | 3.7 | 3.6 | 0.545 |
| walker |  | 5552 | 120 | python decl names surface in src/pluggy/_manager.py |  |  | 0.547 |
| walker |  | 5552 | 0 | python decl at src/pluggy/_manager.py:42 |  |  | 0.547 |
| walker |  | 5552 | 0 | python decl at src/pluggy/_manager.py:52 |  |  | 0.547 |
| walker |  | 5552 | 0 | python decl at src/pluggy/_manager.py:65 |  |  | 0.547 |
| walker |  | 5552 | 0 | python decl at src/pluggy/_manager.py:83 |  |  | 0.547 |
| walker |  | 5552 | 0 | python decl at src/pluggy/_manager.py:525 |  |  | 0.547 |
| walker |  | 5565 | 13 | python decl doc at src/pluggy/_manager.py:65 |  |  | 0.548 |
| walker |  | 5613 | 48 | python decl doc at src/pluggy/_manager.py:52 |  |  | 0.554 |
| walker |  | 5639 | 26 | python decl at src/pluggy/_manager.py:37 |  |  | 0.554 |
| walker |  | 5655 | 16 | python decl body at src/pluggy/_manager.py:525 body 526 |  |  | 0.557 |
| ns | 5677 |  | 156 | `PluginManager.register` docstring: naming, blocking and the duplicate-registration contract | 3.8 | 2.1 | 0.548 |
| ns | 5973 |  | 296 | `PluginManager.register` body: how hook implementations are discovered and attached | 3.9 | 3.8 | 0.535 |
| walker |  | 6270 | 615 | python method sigs in src/pluggy/_manager.py |  |  | 0.589 |
| walker |  | 6270 | 0 | python method at src/pluggy/_manager.py:59 |  |  | 0.589 |
| walker |  | 6270 | 0 | python method at src/pluggy/_manager.py:68 |  |  | 0.589 |
| walker |  | 6270 | 0 | python method at src/pluggy/_manager.py:76 |  |  | 0.589 |
| walker |  | 6270 | 0 | python method at src/pluggy/_manager.py:79 |  |  | 0.589 |
| walker |  | 6270 | 0 | python method at src/pluggy/_manager.py:125 |  |  | 0.589 |
| walker |  | 6270 | 0 | python method at src/pluggy/_manager.py:176 |  |  | 0.589 |
| walker |  | 6270 | 0 | python method at src/pluggy/_manager.py:233 |  |  | 0.589 |
| walker |  | 6270 | 0 | python method at src/pluggy/_manager.py:238 |  |  | 0.589 |
| walker |  | 6270 | 0 | python method at src/pluggy/_manager.py:242 |  |  | 0.589 |
| walker |  | 6270 | 0 | python method at src/pluggy/_manager.py:252 |  |  | 0.589 |
| walker |  | 6270 | 0 | python method at src/pluggy/_manager.py:296 |  |  | 0.589 |
| walker |  | 6270 | 0 | python method at src/pluggy/_manager.py:300 |  |  | 0.589 |
| walker |  | 6270 | 0 | python method at src/pluggy/_manager.py:304 |  |  | 0.589 |
| walker |  | 6270 | 0 | python method at src/pluggy/_manager.py:315 |  |  | 0.589 |
| walker |  | 6270 | 0 | python method at src/pluggy/_manager.py:319 |  |  | 0.589 |
| walker |  | 6270 | 0 | python method at src/pluggy/_manager.py:323 |  |  | 0.589 |
| walker |  | 6270 | 0 | python method at src/pluggy/_manager.py:331 |  |  | 0.589 |
| walker |  | 6270 | 0 | python method at src/pluggy/_manager.py:379 |  |  | 0.589 |
| walker |  | 6270 | 0 | python method at src/pluggy/_manager.py:395 |  |  | 0.589 |
| walker |  | 6270 | 0 | python method at src/pluggy/_manager.py:425 |  |  | 0.589 |
| walker |  | 6270 | 0 | python method at src/pluggy/_manager.py:430 |  |  | 0.589 |
| walker |  | 6270 | 0 | python method at src/pluggy/_manager.py:434 |  |  | 0.589 |
| walker |  | 6270 | 0 | python method at src/pluggy/_manager.py:487 |  |  | 0.589 |
| walker |  | 6278 | 8 | python method at src/pluggy/_manager.py:71 |  |  | 0.591 |
| walker |  | 6288 | 10 | python method at src/pluggy/_manager.py:100 |  |  | 0.591 |
| walker |  | 6316 | 28 | python method at src/pluggy/_manager.py:512 |  |  | 0.591 |
| walker |  | 6330 | 14 | python method doc at src/pluggy/_manager.py:300 |  |  | 0.591 |
| walker |  | 6361 | 31 | python method at src/pluggy/_manager.py:278 |  |  | 0.591 |
| walker |  | 6392 | 31 | python method at src/pluggy/_manager.py:450 |  |  | 0.591 |
| walker |  | 6407 | 15 | python method doc at src/pluggy/_manager.py:238 |  |  | 0.591 |
| walker |  | 6422 | 15 | python method doc at src/pluggy/_manager.py:296 |  |  | 0.591 |
| ns | 6431 |  | 458 | `PluginManager._verify_hook`: every validation error message pluggy can raise | 3.10 | 2.1 | 0.570 |
| walker |  | 6457 | 35 | python method at src/pluggy/_manager.py:201 |  |  | 0.570 |
| walker |  | 6474 | 17 | python method doc at src/pluggy/_manager.py:319 |  |  | 0.570 |
| walker |  | 6492 | 18 | python method doc at src/pluggy/_manager.py:233 |  |  | 0.570 |
| walker |  | 6510 | 18 | python method doc at src/pluggy/_manager.py:315 |  |  | 0.570 |
| walker |  | 6531 | 21 | python method doc at src/pluggy/_manager.py:430 |  |  | 0.570 |
| walker |  | 6560 | 29 | python method doc at src/pluggy/_manager.py:323 |  |  | 0.570 |
| walker |  | 6590 | 30 | python method doc at src/pluggy/_manager.py:425 |  |  | 0.570 |
| walker |  | 6625 | 35 | python method doc at src/pluggy/_manager.py:242 |  |  | 0.570 |
| ns | 6627 |  | 196 | `PluginManager.__init__`: the complete state of a plugin manager | 3.11 | 2.1 | 0.563 |
| walker |  | 6667 | 42 | python method doc at src/pluggy/_manager.py:487 |  |  | 0.563 |
| walker |  | 6711 | 44 | python method doc at src/pluggy/_manager.py:379 |  |  | 0.563 |
| walker |  | 6721 | 10 | python imports in docs/examples/eggsample/eggsample/__init__.py |  |  | 0.563 |
| walker |  | 6771 | 50 | python method doc at src/pluggy/_manager.py:512 |  |  | 0.563 |
| walker |  | 6832 | 61 | python method doc at src/pluggy/_result.py:80 |  |  | 0.563 |
| walker |  | 6895 | 63 | python method doc at src/pluggy/_manager.py:252 |  |  | 0.563 |
| walker |  | 6958 | 63 | python method doc at src/pluggy/_result.py:91 |  |  | 0.563 |
| ns | 6993 |  | 366 | Historic hooks end to end: `set_specification`, `call_historic`, `_maybe_apply_history` | 3.12 | 2.4 | 0.549 |
| walker |  | 7023 | 65 | python method doc at src/pluggy/_manager.py:434 |  |  | 0.549 |
| walker |  | 7100 | 77 | python method doc at src/pluggy/_hooks.py:499 |  |  | 0.553 |
| walker |  | 7187 | 87 | python method doc at src/pluggy/_manager.py:201 |  |  | 0.553 |
| ns | 7193 |  | 200 | Complete attribute sets of `HookImpl` and `HookSpec` (`__slots__`) | 3.13 |  | 0.567 |
| walker |  | 7227 | 40 | declaration surface of changelog/590.trivial.rst |  |  | 0.567 |
| walker |  | 7324 | 97 | declaration surface of tox.ini |  |  | 0.567 |
| ns | 7503 |  | 310 | `load_setuptools_entrypoints`: how third-party plugins are discovered | 3.14 | 2.1 | 0.553 |
| walker |  | 7513 | 189 | python decl doc at src/pluggy/_manager.py:83 |  |  | 0.553 |
| ns | 7697 |  | 194 | Blocking semantics: `set_blocked`, `is_blocked`, `unblock` | 3.15 | 2.1 | 0.549 |
| walker |  | 7702 | 189 | python decl doc at src/pluggy/_warnings.py:10 |  |  | 0.551 |
| walker |  | 7799 | 97 | python method doc at src/pluggy/_manager.py:304 |  |  | 0.551 |
| walker |  | 7897 | 98 | python method doc at src/pluggy/_hooks.py:516 |  |  | 0.551 |
| ns | 7929 |  | 232 | `docs/api_reference.rst`: exactly which types are publicly documented | 4.1 |  | 0.544 |
| walker |  | 7996 | 99 | python method doc at src/pluggy/_result.py:67 |  |  | 0.544 |
| walker |  | 8099 | 103 | python method doc at src/pluggy/_manager.py:395 |  |  | 0.549 |
| ns | 8259 |  | 330 | `docs/examples/toy-example.py`: the canonical end-to-end usage | 4.2 |  | 0.534 |
| walker |  | 8315 | 216 | python class body at src/pluggy/_hooks.py:593 |  |  | 0.534 |
| walker |  | 8536 | 221 | python imports in src/pluggy/_hooks.py |  |  | 0.534 |
| ns | 8540 |  | 281 | The eggsample host program: wiring a PluginManager and calling a hook | 4.3 |  | 0.524 |
| walker |  | 8616 | 80 | python decl names surface in src/pluggy/_callers.py |  |  | 0.525 |
| walker |  | 8647 | 31 | python decl at src/pluggy/_callers.py:27 |  |  | 0.527 |
| walker |  | 8678 | 31 | python decl doc at src/pluggy/_callers.py:27 |  |  | 0.527 |
| walker |  | 8707 | 29 | python decl at src/pluggy/_callers.py:70 |  |  | 0.529 |
| walker |  | 8741 | 34 | python decl at src/pluggy/_callers.py:60 |  |  | 0.532 |
| ns | 8760 |  | 220 | `eggsample/hookspecs.py` in full: what a real hookspec module looks like | 4.4 |  | 0.523 |
| walker |  | 8800 | 59 | python decl at src/pluggy/_callers.py:82 |  |  | 0.531 |
| walker |  | 8934 | 134 | declaration surface of TIDELIFT.rst |  |  | 0.531 |
| walker |  | 9001 | 67 | python decl names surface in src/pluggy/_tracing.py |  |  | 0.532 |
| walker |  | 9001 | 0 | python decl at src/pluggy/_tracing.py:16 |  |  | 0.532 |
| walker |  | 9001 | 0 | python decl at src/pluggy/_tracing.py:59 |  |  | 0.532 |
| ns | 9096 |  | 336 | Both sides of hook implementation: the host's own `lib.py` and the external plugin `eggsample_spam.py` | 4.5 |  | 0.520 |
| walker |  | 9211 | 210 | python method sigs in src/pluggy/_tracing.py |  |  | 0.536 |
| walker |  | 9211 | 0 | python method at src/pluggy/_tracing.py:17 |  |  | 0.536 |
| walker |  | 9211 | 0 | python method at src/pluggy/_tracing.py:22 |  |  | 0.536 |
| walker |  | 9211 | 0 | python method at src/pluggy/_tracing.py:25 |  |  | 0.536 |
| walker |  | 9211 | 0 | python method at src/pluggy/_tracing.py:42 |  |  | 0.536 |
| walker |  | 9211 | 0 | python method at src/pluggy/_tracing.py:48 |  |  | 0.536 |
| walker |  | 9211 | 0 | python method at src/pluggy/_tracing.py:51 |  |  | 0.536 |
| walker |  | 9211 | 0 | python method at src/pluggy/_tracing.py:60 |  |  | 0.536 |
| walker |  | 9211 | 0 | python method at src/pluggy/_tracing.py:64 |  |  | 0.536 |
| walker |  | 9211 | 0 | python method at src/pluggy/_tracing.py:67 |  |  | 0.536 |
| walker |  | 9220 | 9 | python method body at src/pluggy/_tracing.py:48 body 49 |  |  | 0.536 |
| walker |  | 9232 | 12 | python method body at src/pluggy/_tracing.py:22 body 23 |  |  | 0.536 |
| ns | 9239 |  | 143 | Entry-point wiring in both example `setup.py` files | 4.6 |  | 0.532 |
| walker |  | 9245 | 13 | python method body at src/pluggy/_tracing.py:64 body 65 |  |  | 0.532 |
| walker |  | 9262 | 17 | python method body at src/pluggy/_tracing.py:67 body 68 |  |  | 0.532 |
| walker |  | 9280 | 18 | python method body at src/pluggy/_tracing.py:60 body 61 |  |  | 0.532 |
| walker |  | 9389 | 109 | python imports in src/pluggy/_result.py |  |  | 0.532 |
| ns | 9487 |  | 248 | `testing/conftest.py` in full: the two fixtures every test in the suite uses | 5.1 |  | 0.523 |
| walker |  | 9524 | 135 | python method doc at src/pluggy/_manager.py:176 |  |  | 0.523 |
| walker |  | 9580 | 56 | python decl doc at src/pluggy/_callers.py:82 |  |  | 0.523 |
| ns | 9646 |  | 159 | `pyproject.toml`: package identity, Python floor, dependency groups and the src layout | 5.2 |  | 0.523 |
| walker |  | 9717 | 137 | python method doc at src/pluggy/_manager.py:125 |  |  | 0.535 |
| ns | 9825 |  | 179 | `tox.ini`: the environment list and the embedded pytest configuration | 5.3 |  | 0.530 |
| walker |  | 9863 | 146 | python method doc at src/pluggy/_manager.py:278 |  |  | 0.530 |
| ns | 9915 |  | 90 | `[tool.towncrier]` config: how CHANGELOG.rst is produced | 5.4 |  | 0.528 |
| walker |  | 9930 | 67 | python imports in src/pluggy/_tracing.py |  |  | 0.528 |
| walker |  | 9948 | 18 | python decl names surface in docs/examples/eggsample/eggsample/__init__.py |  |  | 0.530 |
| ns | 9948 |  | 33 | Listings of `changelog/` and `scripts/` | 5.5 |  | 0.530 |

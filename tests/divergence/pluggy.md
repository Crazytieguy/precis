Score(3000)=0.654 I=0.789 C=0.543 ns_rows≤3K=16/43 grid(1000/1442/2080/3000/4327/6240/9000)=0.825/0.732/0.624/0.654/0.583/0.567/0.544

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 62 |  | 62 | Repository identity: README title, dependents, and the docs tagline | 1.1 |  | 0.000 |
| walker |  | 96 | 96 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 107 |  | 45 | Complete listing of the shipped package directory `src/pluggy/` | 1.2 |  | 0.000 |
| walker |  | 144 | 48 | Fs::DirListing { dir: src/pluggy } |  |  | 0.751 |
| walker |  | 202 | 58 | Markdown::ReadmeHeadline { file: README.rst } |  |  | 0.937 |
| ns | 203 |  | 96 | Complete repository root listing | 1.3 |  | 0.908 |
| walker |  | 217 | 15 | Markdown::Section { file: README.rst, section_index: 0, keeps_default_concavity: false } |  |  | 0.908 |
| walker |  | 236 | 19 | Fs::DirListing { dir: changelog } |  |  | 0.909 |
| walker |  | 240 | 4 | Fs::DirListing { dir: .claude } |  |  | 0.909 |
| walker |  | 272 | 32 | Fs::DirListing { dir: docs } |  |  | 0.913 |
| walker |  | 277 | 5 | Fs::DirListing { dir: docs/_static/img } |  |  | 0.913 |
| walker |  | 330 | 53 | Toml::Identity { file: pyproject.toml } |  |  | 0.913 |
| ns | 355 |  | 152 | The complete public API name list (`__all__` of src/pluggy/__init__.py) | 1.4 |  | 0.729 |
| walker |  | 373 | 43 | Fs::DirListing { dir: downstream } |  |  | 0.729 |
| walker |  | 386 | 13 | Fs::DirListing { dir: .github } |  |  | 0.729 |
| walker |  | 390 | 4 | Fs::DirListing { dir: .github/workflows } |  |  | 0.729 |
| walker |  | 404 | 14 | Fs::DirListing { dir: scripts } |  |  | 0.729 |
| walker |  | 419 | 15 | Fs::DirListing { dir: docs/examples } |  |  | 0.730 |
| walker |  | 427 | 8 | Fs::DirListing { dir: docs/examples/eggsample } |  |  | 0.731 |
| walker |  | 438 | 11 | Fs::DirListing { dir: docs/examples/eggsample-spam } |  |  | 0.733 |
| walker |  | 458 | 20 | Fs::DirListing { dir: docs/examples/eggsample/eggsample } |  |  | 0.740 |
| ns | 521 |  | 166 | Public name to private module map (the re-export block) | 1.5 | 1.4 | 0.646 |
| ns | 615 |  | 94 | Complete listings of `testing/` and `docs/` | 1.6 |  | 0.579 |
| walker |  | 802 | 344 | Code::CodeKey { rung: Names, file: src/pluggy/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.831 |
| ns | 839 |  | 224 | What pluggy is and what problem it solves (docs/index.rst lede) | 1.7 |  | 0.761 |
| walker |  | 864 | 62 | Fs::DirListing { dir: testing } |  |  | 0.865 |
| ns | 893 |  | 54 | Complete listing of the two worked example packages under docs/examples/ | 1.8 |  | 0.864 |
| ns | 985 |  | 92 | Lazy `__version__` resolution in `__init__.py` | 1.9 | 1.5 | 0.825 |
| walker |  | 994 | 130 | Markdown::Section { file: README.rst, section_index: 4, keeps_default_concavity: false } |  |  | 0.825 |
| walker |  | 1236 | 242 | Markdown::Section { file: README.rst, section_index: 1, keeps_default_concavity: false } |  |  | 0.825 |
| ns | 1296 |  | 311 | Every method name on `PluginManager` (complete roster, names only) | 2.1 |  | 0.732 |
| walker |  | 1333 | 97 | Plaintext::DeclSurface { file: tox.ini } |  |  | 0.732 |
| walker |  | 1380 | 47 | Plaintext::DeclSurface { file: docs/requirements.txt } |  |  | 0.732 |
| ns | 1528 |  | 232 | The rest of `_manager.py`: PluginValidationError, DistFacade, and the two module helpers | 2.2 |  | 0.680 |
| walker |  | 1674 | 294 | Markdown::Section { file: README.rst, section_index: 2, keeps_default_concavity: false } |  |  | 0.680 |
| walker |  | 1794 | 120 | Code::CodeKey { rung: Names, file: src/pluggy/_manager.py, decl: 0, sub: 0, line: 0 } |  |  | 0.684 |
| walker |  | 1819 | 25 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 4, sub: 0, line: 52 } |  |  | 0.687 |
| walker |  | 1845 | 26 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 2, sub: 0, line: 37 } |  |  | 0.687 |
| ns | 1857 |  | 329 | Every top-level symbol in `_hooks.py` (complete roster) plus the two backward-compat aliases | 2.3 |  | 0.635 |
| walker |  | 1938 | 93 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 6, sub: 0, line: 65 } |  |  | 0.644 |
| ns | 2019 |  | 162 | Every method name on `HookCaller` (complete roster, names only) | 2.4 |  | 0.619 |
| walker |  | 2108 | 170 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 11, sub: 0, line: 83 } |  |  | 0.627 |
| walker |  | 2130 | 22 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 16, sub: 0, line: 201 } |  |  | 0.627 |
| walker |  | 2179 | 49 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 13, sub: 0, line: 114 } |  |  | 0.627 |
| ns | 2301 |  | 282 | Complete `Result` API with signatures (`_result.py`) | 2.5 |  | 0.593 |
| walker |  | 2369 | 190 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 11, sub: 1, line: 83 } |  |  | 0.620 |
| walker |  | 2385 | 16 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 21, sub: 0, line: 278 } |  |  | 0.620 |
| ns | 2563 |  | 262 | Complete top-level roster of `_callers.py` (the call loop module) | 2.6 |  | 0.590 |
| walker |  | 2603 | 218 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 11, sub: 2, line: 83 } |  |  | 0.641 |
| walker |  | 2619 | 16 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 36, sub: 0, line: 512 } |  |  | 0.641 |
| walker |  | 2636 | 17 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 34, sub: 0, line: 450 } |  |  | 0.641 |
| walker |  | 2647 | 11 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 6, sub: 0, line: 65 } |  |  | 0.645 |
| walker |  | 2661 | 14 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 23, sub: 0, line: 300 } |  |  | 0.645 |
| walker |  | 2676 | 15 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 18, sub: 0, line: 238 } |  |  | 0.645 |
| walker |  | 2691 | 15 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 22, sub: 0, line: 296 } |  |  | 0.645 |
| walker |  | 2708 | 17 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 26, sub: 0, line: 319 } |  |  | 0.645 |
| walker |  | 2726 | 18 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 17, sub: 0, line: 233 } |  |  | 0.645 |
| walker |  | 2744 | 18 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 25, sub: 0, line: 315 } |  |  | 0.645 |
| walker |  | 2765 | 21 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 32, sub: 0, line: 430 } |  |  | 0.645 |
| ns | 2887 |  | 324 | Complete symbol rosters for the two remaining modules: `_tracing.py` and `_warnings.py` | 2.7 |  | 0.618 |
| walker |  | 2891 | 126 | Code::CodeKey { rung: Names, file: src/pluggy/_callers.py, decl: 0, sub: 0, line: 0 } |  |  | 0.626 |
| walker |  | 2910 | 19 | Code::CodeKey { rung: Decl, file: src/pluggy/_callers.py, decl: 4, sub: 0, line: 70 } |  |  | 0.629 |
| walker |  | 2930 | 20 | Code::CodeKey { rung: Decl, file: src/pluggy/_callers.py, decl: 2, sub: 0, line: 27 } |  |  | 0.633 |
| walker |  | 2953 | 23 | Code::CodeKey { rung: Decl, file: src/pluggy/_callers.py, decl: 3, sub: 0, line: 60 } |  |  | 0.640 |
| walker |  | 2998 | 45 | Code::CodeKey { rung: Decl, file: src/pluggy/_callers.py, decl: 5, sub: 0, line: 82 } |  |  | 0.654 |
| ns | 3298 |  | 411 | Complete section map of the 1082-line manual `docs/index.rst` | 2.8 |  | 0.599 |
| walker |  | 3350 | 352 | Code::CodeKey { rung: Names, file: src/pluggy/_hooks.py, decl: 0, sub: 0, line: 0 } |  |  | 0.613 |
| walker |  | 3374 | 24 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 26, sub: 0, line: 377 } |  |  | 0.613 |
| walker |  | 3408 | 34 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 5, sub: 0, line: 33 } |  |  | 0.613 |
| walker |  | 3488 | 80 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 22, sub: 0, line: 358 } |  |  | 0.613 |
| ns | 3543 |  | 245 | The real call signatures of `@hookspec` and `@hookimpl` — every accepted option with its default | 3.1 |  | 0.590 |
| walker |  | 3617 | 129 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 51, sub: 0, line: 696 } |  |  | 0.592 |
| walker |  | 3779 | 162 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 48, sub: 0, line: 638 } |  |  | 0.595 |
| ns | 3791 |  | 248 | `HookimplOpts` in full: every hook-implementation option and what it means | 3.2 | 2.3 | 0.575 |
| walker |  | 3831 | 52 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 49, sub: 0, line: 656 } |  |  | 0.575 |
| ns | 3975 |  | 184 | `HookspecOpts` in full: every hook-specification option | 3.3 | 2.3 | 0.562 |
| walker |  | 3995 | 164 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 7, sub: 0, line: 40 } |  |  | 0.582 |
| walker |  | 4170 | 175 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 9, sub: 0, line: 77 } |  |  | 0.583 |
| walker |  | 4243 | 73 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 11, sub: 0, line: 91 } |  |  | 0.583 |
| walker |  | 4317 | 74 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 12, sub: 0, line: 101 } |  |  | 0.583 |
| walker |  | 4394 | 77 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 13, sub: 0, line: 111 } |  |  | 0.591 |
| ns | 4407 |  | 432 | Hook implementation ordering: the `_hookimpls` layout comment and `_add_hookimpl` in full | 3.4 | 2.4 | 0.562 |
| walker |  | 4569 | 175 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 14, sub: 0, line: 164 } |  |  | 0.568 |
| ns | 4613 |  | 206 | `HookCaller.__call__` in full: what `pm.hook.myhook(...)` actually does | 3.5 | 2.4 | 0.554 |
| walker |  | 4657 | 88 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 16, sub: 0, line: 178 } |  |  | 0.554 |
| walker |  | 4746 | 89 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 17, sub: 0, line: 190 } |  |  | 0.554 |
| walker |  | 4838 | 92 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 18, sub: 0, line: 202 } |  |  | 0.578 |
| walker |  | 5064 | 226 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 8, sub: 0, line: 56 } |  |  | 0.605 |
| walker |  | 5074 | 10 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 7, sub: 0, line: 40 } |  |  | 0.609 |
| ns | 5078 |  | 465 | `_multicall` part 1: the setup / non-wrapper call loop | 3.6 |  | 0.576 |
| walker |  | 5084 | 10 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 8, sub: 0, line: 56 } |  |  | 0.580 |
| walker |  | 5094 | 10 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 23, sub: 0, line: 365 } |  |  | 0.580 |
| walker |  | 5432 | 338 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 27, sub: 0, line: 382 } |  |  | 0.603 |
| walker |  | 5452 | 20 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 39, sub: 0, line: 543 } |  |  | 0.603 |
| walker |  | 5482 | 30 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 30, sub: 0, line: 424 } |  |  | 0.603 |
| ns | 5521 |  | 443 | `_multicall` part 2: the teardown loop, exception routing and `firstresult` collapse | 3.7 | 3.6 | 0.571 |
| walker |  | 5523 | 41 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 38, sub: 0, line: 516 } |  |  | 0.571 |
| walker |  | 5582 | 59 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 28, sub: 0, line: 393 } |  |  | 0.571 |
| walker |  | 5594 | 12 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 28, sub: 0, line: 393 } |  |  | 0.571 |
| walker |  | 5606 | 12 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 49, sub: 0, line: 656 } |  |  | 0.571 |
| walker |  | 5620 | 14 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 29, sub: 0, line: 420 } |  |  | 0.571 |
| walker |  | 5634 | 14 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 30, sub: 0, line: 424 } |  |  | 0.571 |
| walker |  | 5648 | 14 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 34, sub: 0, line: 453 } |  |  | 0.571 |
| ns | 5677 |  | 156 | `PluginManager.register` docstring: naming, blocking and the duplicate-registration contract | 3.8 | 2.1 | 0.563 |
| ns | 5973 |  | 296 | `PluginManager.register` body: how hook implementations are discovered and attached | 3.9 | 3.8 | 0.549 |
| walker |  | 6019 | 371 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 42, sub: 0, line: 593 } |  |  | 0.549 |
| walker |  | 6034 | 15 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 27, sub: 0, line: 382 } |  |  | 0.552 |
| walker |  | 6049 | 15 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 33, sub: 0, line: 449 } |  |  | 0.552 |
| walker |  | 6065 | 16 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 48, sub: 0, line: 638 } |  |  | 0.554 |
| walker |  | 6083 | 18 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 31, sub: 0, line: 438 } |  |  | 0.554 |
| walker |  | 6104 | 21 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 40, sub: 0, line: 577 } |  |  | 0.554 |
| walker |  | 6133 | 29 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 22, sub: 0, line: 358 } |  |  | 0.560 |
| walker |  | 6163 | 30 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 42, sub: 0, line: 593 } |  |  | 0.567 |
| walker |  | 6248 | 85 | Code::CodeKey { rung: Names, file: src/pluggy/_result.py, decl: 0, sub: 0, line: 0 } |  |  | 0.568 |
| ns | 6431 |  | 458 | `PluginManager._verify_hook`: every validation error message pluggy can raise | 3.10 | 2.1 | 0.547 |
| walker |  | 6444 | 196 | Code::CodeKey { rung: Decl, file: src/pluggy/_result.py, decl: 4, sub: 0, line: 24 } |  |  | 0.563 |
| walker |  | 6473 | 29 | Code::CodeKey { rung: Decl, file: src/pluggy/_result.py, decl: 5, sub: 0, line: 31 } |  |  | 0.563 |
| walker |  | 6482 | 9 | Code::CodeKey { rung: Doc, file: src/pluggy/_result.py, decl: 3, sub: 0, line: 20 } |  |  | 0.565 |
| walker |  | 6494 | 12 | Code::CodeKey { rung: Doc, file: src/pluggy/_result.py, decl: 5, sub: 0, line: 31 } |  |  | 0.565 |
| walker |  | 6506 | 12 | Code::CodeKey { rung: Doc, file: src/pluggy/_result.py, decl: 6, sub: 0, line: 42 } |  |  | 0.565 |
| walker |  | 6518 | 12 | Code::CodeKey { rung: Doc, file: src/pluggy/_result.py, decl: 7, sub: 0, line: 51 } |  |  | 0.565 |
| walker |  | 6530 | 12 | Code::CodeKey { rung: Doc, file: src/pluggy/_result.py, decl: 8, sub: 0, line: 56 } |  |  | 0.565 |
| walker |  | 6575 | 45 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 39, sub: 0, line: 543 } |  |  | 0.565 |
| walker |  | 6583 | 8 | Code::CodeKey { rung: Body, file: src/pluggy/_result.py, decl: 7, sub: 0, line: 51 } |  |  | 0.565 |
| walker |  | 6612 | 29 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 27, sub: 0, line: 323 } |  |  | 0.565 |
| ns | 6627 |  | 196 | `PluginManager.__init__`: the complete state of a plugin manager | 3.11 | 2.1 | 0.558 |
| walker |  | 6642 | 30 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 31, sub: 0, line: 425 } |  |  | 0.558 |
| walker |  | 6709 | 67 | Code::CodeKey { rung: Names, file: src/pluggy/_tracing.py, decl: 0, sub: 0, line: 0 } |  |  | 0.558 |
| walker |  | 6776 | 67 | Code::CodeKey { rung: Decl, file: src/pluggy/_tracing.py, decl: 10, sub: 0, line: 59 } |  |  | 0.560 |
| walker |  | 6919 | 143 | Code::CodeKey { rung: Decl, file: src/pluggy/_tracing.py, decl: 3, sub: 0, line: 16 } |  |  | 0.569 |
| walker |  | 6952 | 33 | Code::CodeKey { rung: Names, file: src/pluggy/_warnings.py, decl: 0, sub: 0, line: 0 } |  |  | 0.573 |
| walker |  | 6967 | 15 | Code::CodeKey { rung: Decl, file: src/pluggy/_warnings.py, decl: 1, sub: 0, line: 4 } |  |  | 0.573 |
| walker |  | 6985 | 18 | Code::CodeKey { rung: Decl, file: src/pluggy/_warnings.py, decl: 2, sub: 0, line: 10 } |  |  | 0.574 |
| ns | 6993 |  | 366 | Historic hooks end to end: `set_specification`, `call_historic`, `_maybe_apply_history` | 3.12 | 2.4 | 0.560 |
| walker |  | 6999 | 14 | Code::CodeKey { rung: Doc, file: src/pluggy/_warnings.py, decl: 1, sub: 0, line: 4 } |  |  | 0.562 |
| walker |  | 7008 | 9 | Code::CodeKey { rung: Body, file: src/pluggy/_tracing.py, decl: 8, sub: 0, line: 48 } |  |  | 0.562 |
| walker |  | 7095 | 87 | Plaintext::DeclSurface { file: downstream/conda.sh } |  |  | 0.562 |
| walker |  | 7137 | 42 | Plaintext::Whole { file: downstream/conda.sh } |  |  | 0.562 |
| walker |  | 7169 | 32 | Code::CodeKey { rung: Doc, file: src/pluggy/_result.py, decl: 4, sub: 0, line: 24 } |  |  | 0.568 |
| ns | 7193 |  | 200 | Complete attribute sets of `HookImpl` and `HookSpec` (`__slots__`) | 3.13 |  | 0.580 |
| walker |  | 7246 | 77 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 37, sub: 0, line: 499 } |  |  | 0.584 |
| walker |  | 7325 | 79 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 21, sub: 0, line: 293 } |  |  | 0.584 |
| walker |  | 7405 | 80 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 14, sub: 0, line: 164 } |  |  | 0.584 |
| walker |  | 7486 | 81 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 9, sub: 0, line: 77 } |  |  | 0.584 |
| ns | 7503 |  | 310 | `load_setuptools_entrypoints`: how third-party plugins are discovered | 3.14 | 2.1 | 0.569 |
| walker |  | 7521 | 35 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 19, sub: 0, line: 242 } |  |  | 0.570 |
| walker |  | 7533 | 12 | Code::CodeKey { rung: Body, file: src/pluggy/_tracing.py, decl: 5, sub: 0, line: 22 } |  |  | 0.570 |
| walker |  | 7631 | 98 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 38, sub: 0, line: 516 } |  |  | 0.570 |
| walker |  | 7673 | 42 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 35, sub: 0, line: 487 } |  |  | 0.570 |
| ns | 7697 |  | 194 | Blocking semantics: `set_blocked`, `is_blocked`, `unblock` | 3.15 | 2.1 | 0.566 |
| walker |  | 7717 | 44 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 29, sub: 0, line: 379 } |  |  | 0.566 |
| walker |  | 7763 | 46 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 4, sub: 0, line: 52 } |  |  | 0.574 |
| walker |  | 7776 | 13 | Code::CodeKey { rung: Body, file: src/pluggy/_tracing.py, decl: 12, sub: 0, line: 64 } |  |  | 0.574 |
| walker |  | 7826 | 50 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 36, sub: 0, line: 512 } |  |  | 0.574 |
| walker |  | 7857 | 31 | Code::CodeKey { rung: Doc, file: src/pluggy/_callers.py, decl: 2, sub: 0, line: 27 } |  |  | 0.574 |
| walker |  | 7923 | 66 | Code::CodeKey { rung: Body, file: src/pluggy/__init__.py, decl: 1, sub: 0, line: 32 } |  |  | 0.586 |
| ns | 7929 |  | 232 | `docs/api_reference.rst`: exactly which types are publicly documented | 4.1 |  | 0.579 |
| walker |  | 7984 | 61 | Code::CodeKey { rung: Doc, file: src/pluggy/_result.py, decl: 10, sub: 0, line: 80 } |  |  | 0.579 |
| walker |  | 8001 | 17 | Code::CodeKey { rung: Body, file: src/pluggy/_tracing.py, decl: 13, sub: 0, line: 67 } |  |  | 0.579 |
| walker |  | 8064 | 63 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 20, sub: 0, line: 252 } |  |  | 0.579 |
| walker |  | 8127 | 63 | Code::CodeKey { rung: Doc, file: src/pluggy/_result.py, decl: 11, sub: 0, line: 91 } |  |  | 0.579 |
| walker |  | 8192 | 65 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 33, sub: 0, line: 434 } |  |  | 0.579 |
| walker |  | 8210 | 18 | Code::CodeKey { rung: Body, file: src/pluggy/_tracing.py, decl: 11, sub: 0, line: 60 } |  |  | 0.579 |
| ns | 8259 |  | 330 | `docs/examples/toy-example.py`: the canonical end-to-end usage | 4.2 |  | 0.564 |
| walker |  | 8286 | 76 | Plaintext::DeclSurface { file: downstream/datasette.sh } |  |  | 0.564 |
| walker |  | 8308 | 22 | Plaintext::Whole { file: downstream/datasette.sh } |  |  | 0.564 |
| ns | 8540 |  | 281 | The eggsample host program: wiring a PluginManager and calling a hook | 4.3 |  | 0.553 |
| ns | 8760 |  | 220 | `eggsample/hookspecs.py` in full: what a real hookspec module looks like | 4.4 |  | 0.543 |
| walker |  | 8866 | 558 | Plaintext::Whole { file: tox.ini } |  |  | 0.544 |
| walker |  | 8953 | 87 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 16, sub: 0, line: 201 } |  |  | 0.544 |
| walker |  | 9009 | 56 | Code::CodeKey { rung: Doc, file: src/pluggy/_callers.py, decl: 5, sub: 0, line: 82 } |  |  | 0.544 |
| ns | 9096 |  | 336 | Both sides of hook implementation: the host's own `lib.py` and the external plugin `eggsample_spam.py` | 4.5 |  | 0.532 |
| walker |  | 9106 | 97 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 24, sub: 0, line: 304 } |  |  | 0.532 |
| walker |  | 9205 | 99 | Code::CodeKey { rung: Doc, file: src/pluggy/_result.py, decl: 9, sub: 0, line: 67 } |  |  | 0.532 |
| ns | 9239 |  | 143 | Entry-point wiring in both example `setup.py` files | 4.6 |  | 0.528 |
| walker |  | 9304 | 99 | Plaintext::DeclSurface { file: downstream/devpi.sh } |  |  | 0.528 |
| walker |  | 9324 | 20 | Plaintext::Whole { file: downstream/devpi.sh } |  |  | 0.528 |
| walker |  | 9427 | 103 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 30, sub: 0, line: 395 } |  |  | 0.532 |
| ns | 9487 |  | 248 | `testing/conftest.py` in full: the two fixtures every test in the suite uses | 5.1 |  | 0.523 |
| walker |  | 9493 | 66 | Plaintext::DeclSurface { file: downstream/hatch.sh } |  |  | 0.523 |
| walker |  | 9514 | 21 | Plaintext::Whole { file: downstream/hatch.sh } |  |  | 0.523 |
| ns | 9646 |  | 159 | `pyproject.toml`: package identity, Python floor, dependency groups and the src layout | 5.2 |  | 0.518 |
| walker |  | 9673 | 159 | Plaintext::DeclSurface { file: downstream/python-lsp-server.sh } |  |  | 0.518 |
| walker |  | 9815 | 142 | Plaintext::Whole { file: downstream/python-lsp-server.sh } |  |  | 0.518 |
| ns | 9825 |  | 179 | `tox.ini`: the environment list and the embedded pytest configuration | 5.3 |  | 0.527 |
| walker |  | 9847 | 32 | Code::CodeKey { rung: Body, file: src/pluggy/_result.py, decl: 9, sub: 0, line: 67 } |  |  | 0.527 |
| ns | 9915 |  | 90 | `[tool.towncrier]` config: how CHANGELOG.rst is produced | 5.4 |  | 0.525 |
| ns | 9948 |  | 33 | Listings of `changelog/` and `scripts/` | 5.5 |  | 0.527 |
| walker |  | 9982 | 135 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 15, sub: 0, line: 176 } |  |  | 0.527 |
| walker |  | 9996 | 14 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 14, sub: 0, line: 125 } |  |  | 0.528 |

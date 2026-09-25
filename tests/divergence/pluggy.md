Score(3000)=0.548 I=0.760 C=0.395 ns_rows≤3K=16/43 grid(1000/1442/2080/3000/4327/6240/9000)=0.825/0.732/0.624/0.548/0.595/0.547/0.543

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 62 |  | 62 | Repository identity: README title, dependents, and the docs tagline | 1.1 |  | 0.000 |
| walker |  | 96 | 96 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 100 | 4 | Fs::DirListing { dir: src } |  |  | 0.000 |
| ns | 107 |  | 45 | Complete listing of the shipped package directory `src/pluggy/` | 1.2 |  | 0.000 |
| walker |  | 158 | 58 | Markdown::ReadmeHeadline { file: README.rst } |  |  | 0.345 |
| walker |  | 173 | 15 | Markdown::Section { file: README.rst, section_index: 0, keeps_default_concavity: false } |  |  | 0.345 |
| walker |  | 192 | 19 | Fs::DirListing { dir: changelog } |  |  | 0.345 |
| walker |  | 196 | 4 | Fs::DirListing { dir: .claude } |  |  | 0.345 |
| ns | 203 |  | 96 | Complete repository root listing | 1.3 |  | 0.652 |
| walker |  | 228 | 32 | Fs::DirListing { dir: docs } |  |  | 0.656 |
| walker |  | 231 | 3 | Fs::DirListing { dir: docs/_static } |  |  | 0.656 |
| walker |  | 235 | 4 | Fs::DirListing { dir: docs/_static/img } |  |  | 0.656 |
| walker |  | 288 | 53 | Toml::Identity { file: pyproject.toml } |  |  | 0.656 |
| walker |  | 331 | 43 | Fs::DirListing { dir: downstream } |  |  | 0.656 |
| ns | 355 |  | 152 | The complete public API name list (`__all__` of src/pluggy/__init__.py) | 1.4 |  | 0.524 |
| walker |  | 376 | 45 | Fs::DirListing { dir: src/pluggy } |  |  | 0.729 |
| walker |  | 389 | 13 | Fs::DirListing { dir: .github } |  |  | 0.729 |
| walker |  | 393 | 4 | Fs::DirListing { dir: .github/workflows } |  |  | 0.729 |
| walker |  | 407 | 14 | Fs::DirListing { dir: scripts } |  |  | 0.729 |
| walker |  | 422 | 15 | Fs::DirListing { dir: docs/examples } |  |  | 0.730 |
| walker |  | 430 | 8 | Fs::DirListing { dir: docs/examples/eggsample } |  |  | 0.731 |
| walker |  | 441 | 11 | Fs::DirListing { dir: docs/examples/eggsample-spam } |  |  | 0.733 |
| walker |  | 461 | 20 | Fs::DirListing { dir: docs/examples/eggsample/eggsample } |  |  | 0.740 |
| ns | 521 |  | 166 | Public name to private module map (the re-export block) | 1.5 | 1.4 | 0.646 |
| ns | 615 |  | 94 | Complete listings of `testing/` and `docs/` | 1.6 |  | 0.579 |
| walker |  | 805 | 344 | Code::CodeKey { rung: Names, file: src/pluggy/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.831 |
| ns | 839 |  | 224 | What pluggy is and what problem it solves (docs/index.rst lede) | 1.7 |  | 0.761 |
| walker |  | 867 | 62 | Fs::DirListing { dir: testing } |  |  | 0.865 |
| ns | 893 |  | 54 | Complete listing of the two worked example packages under docs/examples/ | 1.8 |  | 0.864 |
| ns | 985 |  | 92 | Lazy `__version__` resolution in `__init__.py` | 1.9 | 1.5 | 0.825 |
| walker |  | 1038 | 171 | Markdown::Section { file: README.rst, section_index: 4, keeps_default_concavity: false } |  |  | 0.825 |
| walker |  | 1280 | 242 | Markdown::Section { file: README.rst, section_index: 1, keeps_default_concavity: false } |  |  | 0.825 |
| ns | 1296 |  | 311 | Every method name on `PluginManager` (complete roster, names only) | 2.1 |  | 0.732 |
| walker |  | 1377 | 97 | Plaintext::DeclSurface { file: tox.ini } |  |  | 0.732 |
| walker |  | 1424 | 47 | Plaintext::DeclSurface { file: docs/requirements.txt } |  |  | 0.732 |
| walker |  | 1457 | 33 | Code::CodeKey { rung: Names, file: src/pluggy/_warnings.py, decl: 0, sub: 0, line: 0 } |  |  | 0.732 |
| walker |  | 1472 | 15 | Code::CodeKey { rung: Decl, file: src/pluggy/_warnings.py, decl: 1, sub: 0, line: 4 } |  |  | 0.732 |
| walker |  | 1490 | 18 | Code::CodeKey { rung: Decl, file: src/pluggy/_warnings.py, decl: 2, sub: 0, line: 10 } |  |  | 0.732 |
| walker |  | 1504 | 14 | Code::CodeKey { rung: Doc, file: src/pluggy/_warnings.py, decl: 1, sub: 0, line: 4 } |  |  | 0.733 |
| ns | 1528 |  | 232 | The rest of `_manager.py`: PluginValidationError, DistFacade, and the two module helpers | 2.2 |  | 0.680 |
| walker |  | 1856 | 352 | Code::CodeKey { rung: Names, file: src/pluggy/_hooks.py, decl: 0, sub: 0, line: 0 } |  |  | 0.683 |
| ns | 1857 |  | 329 | Every top-level symbol in `_hooks.py` (complete roster) plus the two backward-compat aliases | 2.3 |  | 0.650 |
| walker |  | 1880 | 24 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 25, sub: 0, line: 377 } |  |  | 0.650 |
| walker |  | 1914 | 34 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 5, sub: 0, line: 33 } |  |  | 0.650 |
| walker |  | 1994 | 80 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 22, sub: 0, line: 358 } |  |  | 0.650 |
| ns | 2019 |  | 162 | Every method name on `HookCaller` (complete roster, names only) | 2.4 |  | 0.624 |
| walker |  | 2088 | 94 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 9, sub: 0, line: 77 } |  |  | 0.624 |
| walker |  | 2182 | 94 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 14, sub: 0, line: 164 } |  |  | 0.624 |
| walker |  | 2277 | 95 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 11, sub: 0, line: 91 } |  |  | 0.624 |
| ns | 2301 |  | 282 | Complete `Result` API with signatures (`_result.py`) | 2.5 |  | 0.590 |
| walker |  | 2374 | 97 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 13, sub: 0, line: 111 } |  |  | 0.591 |
| walker |  | 2477 | 103 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 12, sub: 0, line: 101 } |  |  | 0.591 |
| ns | 2563 |  | 262 | Complete top-level roster of `_callers.py` (the call loop module) | 2.6 |  | 0.563 |
| walker |  | 2587 | 110 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 16, sub: 0, line: 178 } |  |  | 0.563 |
| walker |  | 2699 | 112 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 18, sub: 0, line: 202 } |  |  | 0.566 |
| walker |  | 2817 | 118 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 17, sub: 0, line: 190 } |  |  | 0.566 |
| ns | 2887 |  | 324 | Complete symbol rosters for the two remaining modules: `_tracing.py` and `_warnings.py` | 2.7 |  | 0.544 |
| walker |  | 2946 | 129 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 50, sub: 0, line: 696 } |  |  | 0.546 |
| walker |  | 3097 | 151 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 47, sub: 0, line: 638 } |  |  | 0.549 |
| walker |  | 3160 | 63 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 48, sub: 0, line: 656 } |  |  | 0.549 |
| ns | 3298 |  | 411 | Complete section map of the 1082-line manual `docs/index.rst` | 2.8 |  | 0.503 |
| walker |  | 3324 | 164 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 7, sub: 0, line: 40 } |  |  | 0.504 |
| walker |  | 3334 | 10 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 7, sub: 0, line: 40 } |  |  | 0.505 |
| walker |  | 3344 | 10 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 23, sub: 0, line: 365 } |  |  | 0.505 |
| ns | 3543 |  | 245 | The real call signatures of `@hookspec` and `@hookimpl` — every accepted option with its default | 3.1 |  | 0.532 |
| walker |  | 3570 | 226 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 8, sub: 0, line: 56 } |  |  | 0.535 |
| walker |  | 3580 | 10 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 8, sub: 0, line: 56 } |  |  | 0.535 |
| walker |  | 3592 | 12 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 48, sub: 0, line: 656 } |  |  | 0.535 |
| ns | 3791 |  | 248 | `HookimplOpts` in full: every hook-implementation option and what it means | 3.2 | 2.3 | 0.552 |
| walker |  | 3886 | 294 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 26, sub: 0, line: 382 } |  |  | 0.582 |
| walker |  | 3917 | 31 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 38, sub: 0, line: 543 } |  |  | 0.582 |
| walker |  | 3958 | 41 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 29, sub: 0, line: 424 } |  |  | 0.582 |
| ns | 3975 |  | 184 | `HookspecOpts` in full: every hook-specification option | 3.3 | 2.3 | 0.591 |
| walker |  | 4010 | 52 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 37, sub: 0, line: 516 } |  |  | 0.591 |
| walker |  | 4080 | 70 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 27, sub: 0, line: 393 } |  |  | 0.591 |
| walker |  | 4092 | 12 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 27, sub: 0, line: 393 } |  |  | 0.591 |
| walker |  | 4106 | 14 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 33, sub: 0, line: 453 } |  |  | 0.591 |
| walker |  | 4121 | 15 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 26, sub: 0, line: 382 } |  |  | 0.595 |
| walker |  | 4136 | 15 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 32, sub: 0, line: 449 } |  |  | 0.595 |
| ns | 4407 |  | 432 | Hook implementation ordering: the `_hookimpls` layout comment and `_add_hookimpl` in full | 3.4 | 2.4 | 0.566 |
| walker |  | 4476 | 340 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 41, sub: 0, line: 593 } |  |  | 0.566 |
| walker |  | 4484 | 8 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 44, sub: 0, line: 626 } |  |  | 0.566 |
| walker |  | 4492 | 8 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 45, sub: 0, line: 630 } |  |  | 0.566 |
| walker |  | 4507 | 15 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 43, sub: 0, line: 618 } |  |  | 0.566 |
| walker |  | 4523 | 16 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 47, sub: 0, line: 638 } |  |  | 0.569 |
| walker |  | 4541 | 18 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 30, sub: 0, line: 438 } |  |  | 0.569 |
| walker |  | 4562 | 21 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 39, sub: 0, line: 577 } |  |  | 0.569 |
| walker |  | 4591 | 29 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 22, sub: 0, line: 358 } |  |  | 0.576 |
| ns | 4613 |  | 206 | `HookCaller.__call__` in full: what `pm.hook.myhook(...)` actually does | 3.5 | 2.4 | 0.563 |
| walker |  | 4621 | 30 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 41, sub: 0, line: 593 } |  |  | 0.572 |
| walker |  | 4666 | 45 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 38, sub: 0, line: 543 } |  |  | 0.572 |
| walker |  | 4751 | 85 | Code::CodeKey { rung: Names, file: src/pluggy/_result.py, decl: 0, sub: 0, line: 0 } |  |  | 0.572 |
| walker |  | 4912 | 161 | Code::CodeKey { rung: Decl, file: src/pluggy/_result.py, decl: 4, sub: 0, line: 24 } |  |  | 0.587 |
| walker |  | 4920 | 8 | Code::CodeKey { rung: Decl, file: src/pluggy/_result.py, decl: 6, sub: 0, line: 42 } |  |  | 0.589 |
| walker |  | 4928 | 8 | Code::CodeKey { rung: Decl, file: src/pluggy/_result.py, decl: 7, sub: 0, line: 51 } |  |  | 0.590 |
| walker |  | 4936 | 8 | Code::CodeKey { rung: Decl, file: src/pluggy/_result.py, decl: 8, sub: 0, line: 56 } |  |  | 0.592 |
| walker |  | 4976 | 40 | Code::CodeKey { rung: Decl, file: src/pluggy/_result.py, decl: 5, sub: 0, line: 31 } |  |  | 0.592 |
| walker |  | 4985 | 9 | Code::CodeKey { rung: Doc, file: src/pluggy/_result.py, decl: 3, sub: 0, line: 20 } |  |  | 0.595 |
| walker |  | 4997 | 12 | Code::CodeKey { rung: Doc, file: src/pluggy/_result.py, decl: 5, sub: 0, line: 31 } |  |  | 0.595 |
| walker |  | 5009 | 12 | Code::CodeKey { rung: Doc, file: src/pluggy/_result.py, decl: 6, sub: 0, line: 42 } |  |  | 0.595 |
| walker |  | 5021 | 12 | Code::CodeKey { rung: Doc, file: src/pluggy/_result.py, decl: 7, sub: 0, line: 51 } |  |  | 0.595 |
| walker |  | 5033 | 12 | Code::CodeKey { rung: Doc, file: src/pluggy/_result.py, decl: 8, sub: 0, line: 56 } |  |  | 0.595 |
| ns | 5078 |  | 465 | `_multicall` part 1: the setup / non-wrapper call loop | 3.6 |  | 0.563 |
| walker |  | 5327 | 294 | Markdown::Section { file: README.rst, section_index: 2, keeps_default_concavity: false } |  |  | 0.563 |
| walker |  | 5447 | 120 | Code::CodeKey { rung: Names, file: src/pluggy/_manager.py, decl: 0, sub: 0, line: 0 } |  |  | 0.565 |
| walker |  | 5472 | 25 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 4, sub: 0, line: 52 } |  |  | 0.566 |
| walker |  | 5498 | 26 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 2, sub: 0, line: 37 } |  |  | 0.566 |
| ns | 5521 |  | 443 | `_multicall` part 2: the teardown loop, exception routing and `firstresult` collapse | 3.7 | 3.6 | 0.536 |
| walker |  | 5583 | 85 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 6, sub: 0, line: 65 } |  |  | 0.539 |
| walker |  | 5591 | 8 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 8, sub: 0, line: 71 } |  |  | 0.540 |
| walker |  | 5602 | 11 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 6, sub: 0, line: 65 } |  |  | 0.542 |
| walker |  | 5611 | 9 | Code::CodeKey { rung: Body, file: src/pluggy/_manager.py, decl: 7, sub: 0, line: 68 } |  |  | 0.542 |
| ns | 5677 |  | 156 | `PluginManager.register` docstring: naming, blocking and the duplicate-registration contract | 3.8 | 2.1 | 0.534 |
| walker |  | 5795 | 184 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 11, sub: 0, line: 83 } |  |  | 0.541 |
| walker |  | 5830 | 35 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 16, sub: 0, line: 201 } |  |  | 0.541 |
| walker |  | 5894 | 64 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 13, sub: 0, line: 114 } |  |  | 0.541 |
| walker |  | 5909 | 15 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 18, sub: 0, line: 238 } |  |  | 0.541 |
| walker |  | 5927 | 18 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 17, sub: 0, line: 233 } |  |  | 0.541 |
| ns | 5973 |  | 296 | `PluginManager.register` body: how hook implementations are discovered and attached | 3.9 | 3.8 | 0.527 |
| walker |  | 6102 | 175 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 11, sub: 1, line: 83 } |  |  | 0.544 |
| walker |  | 6133 | 31 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 21, sub: 0, line: 278 } |  |  | 0.544 |
| walker |  | 6147 | 14 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 23, sub: 0, line: 300 } |  |  | 0.544 |
| walker |  | 6162 | 15 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 22, sub: 0, line: 296 } |  |  | 0.544 |
| walker |  | 6179 | 17 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 26, sub: 0, line: 319 } |  |  | 0.544 |
| walker |  | 6197 | 18 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 25, sub: 0, line: 315 } |  |  | 0.544 |
| walker |  | 6347 | 150 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 11, sub: 2, line: 83 } |  |  | 0.567 |
| walker |  | 6375 | 28 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 36, sub: 0, line: 512 } |  |  | 0.567 |
| walker |  | 6406 | 31 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 34, sub: 0, line: 450 } |  |  | 0.567 |
| walker |  | 6427 | 21 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 32, sub: 0, line: 430 } |  |  | 0.567 |
| ns | 6431 |  | 458 | `PluginManager._verify_hook`: every validation error message pluggy can raise | 3.10 | 2.1 | 0.546 |
| walker |  | 6493 | 66 | Code::CodeKey { rung: Body, file: src/pluggy/__init__.py, decl: 1, sub: 0, line: 32 } |  |  | 0.561 |
| walker |  | 6522 | 29 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 27, sub: 0, line: 323 } |  |  | 0.561 |
| walker |  | 6552 | 30 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 31, sub: 0, line: 425 } |  |  | 0.561 |
| walker |  | 6584 | 32 | Code::CodeKey { rung: Doc, file: src/pluggy/_result.py, decl: 4, sub: 0, line: 24 } |  |  | 0.567 |
| ns | 6627 |  | 196 | `PluginManager.__init__`: the complete state of a plugin manager | 3.11 | 2.1 | 0.560 |
| walker |  | 6661 | 77 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 36, sub: 0, line: 499 } |  |  | 0.564 |
| walker |  | 6740 | 79 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 21, sub: 0, line: 293 } |  |  | 0.564 |
| walker |  | 6820 | 80 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 14, sub: 0, line: 164 } |  |  | 0.564 |
| walker |  | 6900 | 80 | Code::CodeKey { rung: Names, file: src/pluggy/_callers.py, decl: 0, sub: 0, line: 0 } |  |  | 0.566 |
| walker |  | 6929 | 29 | Code::CodeKey { rung: Decl, file: src/pluggy/_callers.py, decl: 4, sub: 0, line: 70 } |  |  | 0.567 |
| walker |  | 6960 | 31 | Code::CodeKey { rung: Decl, file: src/pluggy/_callers.py, decl: 2, sub: 0, line: 27 } |  |  | 0.570 |
| ns | 6993 |  | 366 | Historic hooks end to end: `set_specification`, `call_historic`, `_maybe_apply_history` | 3.12 | 2.4 | 0.556 |
| walker |  | 6994 | 34 | Code::CodeKey { rung: Decl, file: src/pluggy/_callers.py, decl: 3, sub: 0, line: 60 } |  |  | 0.560 |
| walker |  | 7053 | 59 | Code::CodeKey { rung: Decl, file: src/pluggy/_callers.py, decl: 5, sub: 0, line: 82 } |  |  | 0.569 |
| walker |  | 7134 | 81 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 9, sub: 0, line: 77 } |  |  | 0.569 |
| walker |  | 7169 | 35 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 19, sub: 0, line: 242 } |  |  | 0.569 |
| ns | 7193 |  | 200 | Complete attribute sets of `HookImpl` and `HookSpec` (`__slots__`) | 3.13 |  | 0.582 |
| walker |  | 7236 | 67 | Code::CodeKey { rung: Names, file: src/pluggy/_tracing.py, decl: 0, sub: 0, line: 0 } |  |  | 0.583 |
| walker |  | 7303 | 67 | Code::CodeKey { rung: Decl, file: src/pluggy/_tracing.py, decl: 10, sub: 0, line: 59 } |  |  | 0.586 |
| walker |  | 7446 | 143 | Code::CodeKey { rung: Decl, file: src/pluggy/_tracing.py, decl: 3, sub: 0, line: 16 } |  |  | 0.598 |
| walker |  | 7455 | 9 | Code::CodeKey { rung: Body, file: src/pluggy/_tracing.py, decl: 8, sub: 0, line: 48 } |  |  | 0.598 |
| walker |  | 7467 | 12 | Code::CodeKey { rung: Body, file: src/pluggy/_tracing.py, decl: 5, sub: 0, line: 22 } |  |  | 0.598 |
| walker |  | 7480 | 13 | Code::CodeKey { rung: Body, file: src/pluggy/_tracing.py, decl: 12, sub: 0, line: 64 } |  |  | 0.598 |
| ns | 7503 |  | 310 | `load_setuptools_entrypoints`: how third-party plugins are discovered | 3.14 | 2.1 | 0.582 |
| walker |  | 7564 | 84 | Plaintext::DeclSurface { file: downstream/tox.sh } |  |  | 0.582 |
| walker |  | 7593 | 29 | Plaintext::Whole { file: downstream/tox.sh } |  |  | 0.582 |
| walker |  | 7680 | 87 | Plaintext::DeclSurface { file: downstream/conda.sh } |  |  | 0.582 |
| ns | 7697 |  | 194 | Blocking semantics: `set_blocked`, `is_blocked`, `unblock` | 3.15 | 2.1 | 0.578 |
| walker |  | 7722 | 42 | Plaintext::Whole { file: downstream/conda.sh } |  |  | 0.578 |
| walker |  | 7813 | 91 | Plaintext::DeclSurface { file: downstream/pytest.sh } |  |  | 0.578 |
| walker |  | 7842 | 29 | Plaintext::Whole { file: downstream/pytest.sh } |  |  | 0.578 |
| ns | 7929 |  | 232 | `docs/api_reference.rst`: exactly which types are publicly documented | 4.1 |  | 0.571 |
| walker |  | 7935 | 93 | Plaintext::DeclSurface { file: downstream/datasette.sh } |  |  | 0.571 |
| walker |  | 7965 | 30 | Plaintext::Whole { file: downstream/datasette.sh } |  |  | 0.571 |
| walker |  | 8060 | 95 | Plaintext::DeclSurface { file: downstream/hatch.sh } |  |  | 0.571 |
| walker |  | 8089 | 29 | Plaintext::Whole { file: downstream/hatch.sh } |  |  | 0.571 |
| walker |  | 8187 | 98 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 37, sub: 0, line: 516 } |  |  | 0.571 |
| walker |  | 8229 | 42 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 35, sub: 0, line: 487 } |  |  | 0.571 |
| ns | 8259 |  | 330 | `docs/examples/toy-example.py`: the canonical end-to-end usage | 4.2 |  | 0.556 |
| walker |  | 8273 | 44 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 29, sub: 0, line: 379 } |  |  | 0.556 |
| walker |  | 8319 | 46 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 4, sub: 0, line: 52 } |  |  | 0.564 |
| walker |  | 8427 | 108 | Plaintext::DeclSurface { file: downstream/devpi.sh } |  |  | 0.564 |
| walker |  | 8475 | 48 | Plaintext::Whole { file: downstream/devpi.sh } |  |  | 0.564 |
| ns | 8540 |  | 281 | The eggsample host program: wiring a PluginManager and calling a hook | 4.3 |  | 0.553 |
| ns | 8760 |  | 220 | `eggsample/hookspecs.py` in full: what a real hookspec module looks like | 4.4 |  | 0.543 |
| walker |  | 9016 | 541 | Markdown::Section { file: README.rst, section_index: 3, keeps_default_concavity: false } |  |  | 0.543 |
| walker |  | 9066 | 50 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 36, sub: 0, line: 512 } |  |  | 0.543 |
| ns | 9096 |  | 336 | Both sides of hook implementation: the host's own `lib.py` and the external plugin `eggsample_spam.py` | 4.5 |  | 0.531 |
| walker |  | 9097 | 31 | Code::CodeKey { rung: Doc, file: src/pluggy/_callers.py, decl: 2, sub: 0, line: 27 } |  |  | 0.531 |
| walker |  | 9235 | 138 | Plaintext::DeclSurface { file: downstream/python-lsp-server.sh } |  |  | 0.531 |
| ns | 9239 |  | 143 | Entry-point wiring in both example `setup.py` files | 4.6 |  | 0.527 |
| walker |  | 9252 | 17 | Code::CodeKey { rung: Body, file: src/pluggy/_tracing.py, decl: 13, sub: 0, line: 67 } |  |  | 0.527 |
| walker |  | 9313 | 61 | Code::CodeKey { rung: Doc, file: src/pluggy/_result.py, decl: 10, sub: 0, line: 80 } |  |  | 0.527 |
| walker |  | 9376 | 63 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 20, sub: 0, line: 252 } |  |  | 0.527 |
| walker |  | 9439 | 63 | Code::CodeKey { rung: Doc, file: src/pluggy/_result.py, decl: 11, sub: 0, line: 91 } |  |  | 0.527 |
| ns | 9487 |  | 248 | `testing/conftest.py` in full: the two fixtures every test in the suite uses | 5.1 |  | 0.518 |
| walker |  | 9504 | 65 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 33, sub: 0, line: 434 } |  |  | 0.518 |
| walker |  | 9591 | 87 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 16, sub: 0, line: 201 } |  |  | 0.518 |
| ns | 9646 |  | 159 | `pyproject.toml`: package identity, Python floor, dependency groups and the src layout | 5.2 |  | 0.514 |
| walker |  | 9796 | 205 | Plaintext::Whole { file: downstream/python-lsp-server.sh } |  |  | 0.514 |
| walker |  | 9814 | 18 | Code::CodeKey { rung: Body, file: src/pluggy/_tracing.py, decl: 11, sub: 0, line: 60 } |  |  | 0.514 |
| ns | 9825 |  | 179 | `tox.ini`: the environment list and the embedded pytest configuration | 5.3 |  | 0.509 |
| walker |  | 9870 | 56 | Code::CodeKey { rung: Doc, file: src/pluggy/_callers.py, decl: 5, sub: 0, line: 82 } |  |  | 0.509 |
| ns | 9915 |  | 90 | `[tool.towncrier]` config: how CHANGELOG.rst is produced | 5.4 |  | 0.507 |
| ns | 9948 |  | 33 | Listings of `changelog/` and `scripts/` | 5.5 |  | 0.509 |
| walker |  | 9967 | 97 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 24, sub: 0, line: 304 } |  |  | 0.509 |
| walker |  | 9988 | 21 | Code::CodeKey { rung: Doc, file: src/pluggy/_result.py, decl: 9, sub: 0, line: 67 } |  |  | 0.509 |

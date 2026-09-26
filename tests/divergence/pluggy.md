Score(3000)=0.640 I=0.785 C=0.521 ns_rows≤3K=16/43 grid(1000/1442/2080/3000/4327/6240/9000)=0.825/0.732/0.622/0.640/0.605/0.583/0.541

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 62 |  | 62 | Repository identity: README title, dependents, and the docs tagline | 1.1 |  | 0.000 |
| walker |  | 96 | 96 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 107 |  | 45 | Complete listing of the shipped package directory `src/pluggy/` | 1.2 |  | 0.000 |
| walker |  | 154 | 58 | Markdown::ReadmeHeadline { file: README.rst } |  |  | 0.345 |
| walker |  | 169 | 15 | Markdown::Section { file: README.rst, section_index: 0, keeps_default_concavity: false } |  |  | 0.345 |
| walker |  | 188 | 19 | Fs::DirListing { dir: changelog } |  |  | 0.345 |
| walker |  | 192 | 4 | Fs::DirListing { dir: .claude } |  |  | 0.345 |
| ns | 203 |  | 96 | Complete repository root listing | 1.3 |  | 0.652 |
| walker |  | 224 | 32 | Fs::DirListing { dir: docs } |  |  | 0.656 |
| walker |  | 229 | 5 | Fs::DirListing { dir: docs/_static/img } |  |  | 0.656 |
| walker |  | 282 | 53 | Toml::Identity { file: pyproject.toml } |  |  | 0.656 |
| walker |  | 325 | 43 | Fs::DirListing { dir: downstream } |  |  | 0.656 |
| ns | 355 |  | 152 | The complete public API name list (`__all__` of src/pluggy/__init__.py) | 1.4 |  | 0.524 |
| walker |  | 373 | 48 | Fs::DirListing { dir: src/pluggy } |  |  | 0.729 |
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
| walker |  | 1035 | 171 | Markdown::Section { file: README.rst, section_index: 4, keeps_default_concavity: false } |  |  | 0.825 |
| walker |  | 1124 | 89 | Plaintext::DeclSurface { file: tox.ini } |  |  | 0.825 |
| ns | 1296 |  | 311 | Every method name on `PluginManager` (complete roster, names only) | 2.1 |  | 0.732 |
| walker |  | 1366 | 242 | Markdown::Section { file: README.rst, section_index: 1, keeps_default_concavity: false } |  |  | 0.732 |
| walker |  | 1413 | 47 | Plaintext::DeclSurface { file: docs/requirements.txt } |  |  | 0.732 |
| ns | 1528 |  | 232 | The rest of `_manager.py`: PluginValidationError, DistFacade, and the two module helpers | 2.2 |  | 0.680 |
| walker |  | 1707 | 294 | Markdown::Section { file: README.rst, section_index: 2, keeps_default_concavity: false } |  |  | 0.680 |
| walker |  | 1827 | 120 | Code::CodeKey { rung: Names, file: src/pluggy/_manager.py, decl: 0, sub: 0, line: 0 } |  |  | 0.684 |
| walker |  | 1852 | 25 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 4, sub: 0, line: 52 } |  |  | 0.687 |
| ns | 1857 |  | 329 | Every top-level symbol in `_hooks.py` (complete roster) plus the two backward-compat aliases | 2.3 |  | 0.635 |
| walker |  | 1878 | 26 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 2, sub: 0, line: 37 } |  |  | 0.635 |
| walker |  | 1971 | 93 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 6, sub: 0, line: 65 } |  |  | 0.644 |
| ns | 2019 |  | 162 | Every method name on `HookCaller` (complete roster, names only) | 2.4 |  | 0.619 |
| walker |  | 2141 | 170 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 11, sub: 0, line: 83 } |  |  | 0.627 |
| walker |  | 2163 | 22 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 16, sub: 0, line: 201 } |  |  | 0.627 |
| walker |  | 2212 | 49 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 13, sub: 0, line: 114 } |  |  | 0.627 |
| ns | 2301 |  | 282 | Complete `Result` API with signatures (`_result.py`) | 2.5 |  | 0.593 |
| walker |  | 2402 | 190 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 11, sub: 1, line: 83 } |  |  | 0.620 |
| walker |  | 2418 | 16 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 21, sub: 0, line: 278 } |  |  | 0.620 |
| ns | 2563 |  | 262 | Complete top-level roster of `_callers.py` (the call loop module) | 2.6 |  | 0.590 |
| walker |  | 2636 | 218 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 11, sub: 2, line: 83 } |  |  | 0.641 |
| walker |  | 2652 | 16 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 36, sub: 0, line: 512 } |  |  | 0.641 |
| walker |  | 2669 | 17 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 34, sub: 0, line: 450 } |  |  | 0.641 |
| walker |  | 2680 | 11 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 6, sub: 0, line: 65 } |  |  | 0.645 |
| walker |  | 2694 | 14 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 23, sub: 0, line: 300 } |  |  | 0.645 |
| walker |  | 2709 | 15 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 18, sub: 0, line: 238 } |  |  | 0.645 |
| walker |  | 2724 | 15 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 22, sub: 0, line: 296 } |  |  | 0.645 |
| walker |  | 2741 | 17 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 26, sub: 0, line: 319 } |  |  | 0.645 |
| walker |  | 2759 | 18 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 17, sub: 0, line: 233 } |  |  | 0.645 |
| walker |  | 2777 | 18 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 25, sub: 0, line: 315 } |  |  | 0.645 |
| walker |  | 2798 | 21 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 32, sub: 0, line: 430 } |  |  | 0.645 |
| ns | 2887 |  | 324 | Complete symbol rosters for the two remaining modules: `_tracing.py` and `_warnings.py` | 2.7 |  | 0.618 |
| walker |  | 2924 | 126 | Code::CodeKey { rung: Names, file: src/pluggy/_callers.py, decl: 0, sub: 0, line: 0 } |  |  | 0.626 |
| walker |  | 2943 | 19 | Code::CodeKey { rung: Decl, file: src/pluggy/_callers.py, decl: 4, sub: 0, line: 70 } |  |  | 0.629 |
| walker |  | 2963 | 20 | Code::CodeKey { rung: Decl, file: src/pluggy/_callers.py, decl: 2, sub: 0, line: 27 } |  |  | 0.633 |
| walker |  | 2986 | 23 | Code::CodeKey { rung: Decl, file: src/pluggy/_callers.py, decl: 3, sub: 0, line: 60 } |  |  | 0.640 |
| walker |  | 3031 | 45 | Code::CodeKey { rung: Decl, file: src/pluggy/_callers.py, decl: 5, sub: 0, line: 82 } |  |  | 0.654 |
| ns | 3298 |  | 411 | Complete section map of the 1082-line manual `docs/index.rst` | 2.8 |  | 0.599 |
| walker |  | 3418 | 387 | Code::CodeKey { rung: Names, file: src/pluggy/_hooks.py, decl: 0, sub: 0, line: 0 } |  |  | 0.615 |
| walker |  | 3442 | 24 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 21, sub: 0, line: 377 } |  |  | 0.615 |
| walker |  | 3476 | 34 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 5, sub: 0, line: 33 } |  |  | 0.615 |
| ns | 3543 |  | 245 | The real call signatures of `@hookspec` and `@hookimpl` — every accepted option with its default | 3.1 |  | 0.592 |
| walker |  | 3549 | 73 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 18, sub: 0, line: 358 } |  |  | 0.592 |
| walker |  | 3627 | 78 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 9, sub: 0, line: 77 } |  |  | 0.593 |
| walker |  | 3704 | 77 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 11, sub: 0, line: 111 } |  |  | 0.601 |
| walker |  | 3782 | 78 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 12, sub: 0, line: 164 } |  |  | 0.606 |
| ns | 3791 |  | 248 | `HookimplOpts` in full: every hook-implementation option and what it means | 3.2 | 2.3 | 0.585 |
| walker |  | 3874 | 92 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 14, sub: 0, line: 202 } |  |  | 0.611 |
| ns | 3975 |  | 184 | `HookspecOpts` in full: every hook-specification option | 3.3 | 2.3 | 0.597 |
| walker |  | 3996 | 122 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 46, sub: 0, line: 696 } |  |  | 0.597 |
| walker |  | 4151 | 155 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 43, sub: 0, line: 638 } |  |  | 0.599 |
| walker |  | 4203 | 52 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 44, sub: 0, line: 656 } |  |  | 0.599 |
| walker |  | 4367 | 164 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 7, sub: 0, line: 40 } |  |  | 0.619 |
| ns | 4407 |  | 432 | Hook implementation ordering: the `_hookimpls` layout comment and `_add_hookimpl` in full | 3.4 | 2.4 | 0.588 |
| walker |  | 4593 | 226 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 8, sub: 0, line: 56 } |  |  | 0.616 |
| walker |  | 4603 | 10 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 7, sub: 0, line: 40 } |  |  | 0.620 |
| walker |  | 4613 | 10 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 8, sub: 0, line: 56 } |  |  | 0.609 |
| ns | 4613 |  | 206 | `HookCaller.__call__` in full: what `pm.hook.myhook(...)` actually does | 3.5 | 2.4 | 0.609 |
| walker |  | 4623 | 10 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 19, sub: 0, line: 365 } |  |  | 0.609 |
| walker |  | 4961 | 338 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 22, sub: 0, line: 382 } |  |  | 0.634 |
| walker |  | 4981 | 20 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 34, sub: 0, line: 543 } |  |  | 0.634 |
| walker |  | 5011 | 30 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 25, sub: 0, line: 424 } |  |  | 0.634 |
| walker |  | 5052 | 41 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 33, sub: 0, line: 516 } |  |  | 0.634 |
| ns | 5078 |  | 465 | `_multicall` part 1: the setup / non-wrapper call loop | 3.6 |  | 0.600 |
| walker |  | 5111 | 59 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 23, sub: 0, line: 393 } |  |  | 0.600 |
| walker |  | 5123 | 12 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 23, sub: 0, line: 393 } |  |  | 0.600 |
| walker |  | 5135 | 12 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 44, sub: 0, line: 656 } |  |  | 0.600 |
| walker |  | 5149 | 14 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 24, sub: 0, line: 420 } |  |  | 0.600 |
| walker |  | 5163 | 14 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 25, sub: 0, line: 424 } |  |  | 0.600 |
| walker |  | 5177 | 14 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 29, sub: 0, line: 453 } |  |  | 0.600 |
| ns | 5521 |  | 443 | `_multicall` part 2: the teardown loop, exception routing and `firstresult` collapse | 3.7 | 3.6 | 0.568 |
| walker |  | 5548 | 371 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 37, sub: 0, line: 593 } |  |  | 0.568 |
| walker |  | 5563 | 15 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 22, sub: 0, line: 382 } |  |  | 0.571 |
| walker |  | 5578 | 15 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 28, sub: 0, line: 449 } |  |  | 0.571 |
| walker |  | 5594 | 16 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 43, sub: 0, line: 638 } |  |  | 0.574 |
| walker |  | 5612 | 18 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 26, sub: 0, line: 438 } |  |  | 0.574 |
| walker |  | 5633 | 21 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 35, sub: 0, line: 577 } |  |  | 0.574 |
| walker |  | 5662 | 29 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 18, sub: 0, line: 358 } |  |  | 0.580 |
| ns | 5677 |  | 156 | `PluginManager.register` docstring: naming, blocking and the duplicate-registration contract | 3.8 | 2.1 | 0.571 |
| walker |  | 5692 | 30 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 37, sub: 0, line: 593 } |  |  | 0.578 |
| walker |  | 5784 | 92 | Code::CodeKey { rung: Names, file: src/pluggy/_result.py, decl: 0, sub: 0, line: 0 } |  |  | 0.579 |
| walker |  | 5973 | 189 | Code::CodeKey { rung: Decl, file: src/pluggy/_result.py, decl: 4, sub: 0, line: 24 } |  |  | 0.581 |
| ns | 5973 |  | 296 | `PluginManager.register` body: how hook implementations are discovered and attached | 3.9 | 3.8 | 0.581 |
| walker |  | 6002 | 29 | Code::CodeKey { rung: Decl, file: src/pluggy/_result.py, decl: 5, sub: 0, line: 31 } |  |  | 0.581 |
| walker |  | 6011 | 9 | Code::CodeKey { rung: Doc, file: src/pluggy/_result.py, decl: 3, sub: 0, line: 20 } |  |  | 0.583 |
| walker |  | 6023 | 12 | Code::CodeKey { rung: Doc, file: src/pluggy/_result.py, decl: 5, sub: 0, line: 31 } |  |  | 0.583 |
| walker |  | 6035 | 12 | Code::CodeKey { rung: Doc, file: src/pluggy/_result.py, decl: 6, sub: 0, line: 42 } |  |  | 0.583 |
| walker |  | 6047 | 12 | Code::CodeKey { rung: Doc, file: src/pluggy/_result.py, decl: 7, sub: 0, line: 51 } |  |  | 0.583 |
| walker |  | 6059 | 12 | Code::CodeKey { rung: Doc, file: src/pluggy/_result.py, decl: 8, sub: 0, line: 56 } |  |  | 0.583 |
| walker |  | 6104 | 45 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 34, sub: 0, line: 543 } |  |  | 0.583 |
| walker |  | 6112 | 8 | Code::CodeKey { rung: Body, file: src/pluggy/_result.py, decl: 7, sub: 0, line: 51 } |  |  | 0.583 |
| walker |  | 6141 | 29 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 27, sub: 0, line: 323 } |  |  | 0.583 |
| walker |  | 6171 | 30 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 31, sub: 0, line: 425 } |  |  | 0.583 |
| walker |  | 6238 | 67 | Code::CodeKey { rung: Names, file: src/pluggy/_tracing.py, decl: 0, sub: 0, line: 0 } |  |  | 0.583 |
| walker |  | 6305 | 67 | Code::CodeKey { rung: Decl, file: src/pluggy/_tracing.py, decl: 10, sub: 0, line: 59 } |  |  | 0.585 |
| ns | 6431 |  | 458 | `PluginManager._verify_hook`: every validation error message pluggy can raise | 3.10 | 2.1 | 0.564 |
| walker |  | 6448 | 143 | Code::CodeKey { rung: Decl, file: src/pluggy/_tracing.py, decl: 3, sub: 0, line: 16 } |  |  | 0.573 |
| walker |  | 6488 | 40 | Code::CodeKey { rung: Names, file: src/pluggy/_warnings.py, decl: 0, sub: 0, line: 0 } |  |  | 0.579 |
| walker |  | 6501 | 13 | Code::CodeKey { rung: Decl, file: src/pluggy/_warnings.py, decl: 1, sub: 0, line: 4 } |  |  | 0.579 |
| walker |  | 6514 | 13 | Code::CodeKey { rung: Decl, file: src/pluggy/_warnings.py, decl: 2, sub: 0, line: 10 } |  |  | 0.579 |
| walker |  | 6528 | 14 | Code::CodeKey { rung: Doc, file: src/pluggy/_warnings.py, decl: 1, sub: 0, line: 4 } |  |  | 0.581 |
| walker |  | 6537 | 9 | Code::CodeKey { rung: Body, file: src/pluggy/_tracing.py, decl: 8, sub: 0, line: 48 } |  |  | 0.581 |
| walker |  | 6621 | 84 | Plaintext::DeclSurface { file: downstream/tox.sh } |  |  | 0.581 |
| ns | 6627 |  | 196 | `PluginManager.__init__`: the complete state of a plugin manager | 3.11 | 2.1 | 0.573 |
| walker |  | 6650 | 29 | Plaintext::Whole { file: downstream/tox.sh } |  |  | 0.573 |
| walker |  | 6735 | 85 | Plaintext::DeclSurface { file: downstream/conda.sh } |  |  | 0.573 |
| walker |  | 6779 | 44 | Plaintext::Whole { file: downstream/conda.sh } |  |  | 0.573 |
| walker |  | 6811 | 32 | Code::CodeKey { rung: Doc, file: src/pluggy/_result.py, decl: 4, sub: 0, line: 24 } |  |  | 0.579 |
| walker |  | 6888 | 77 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 32, sub: 0, line: 499 } |  |  | 0.584 |
| walker |  | 6979 | 91 | Plaintext::DeclSurface { file: downstream/pytest.sh } |  |  | 0.584 |
| ns | 6993 |  | 366 | Historic hooks end to end: `set_specification`, `call_historic`, `_maybe_apply_history` | 3.12 | 2.4 | 0.569 |
| walker |  | 7008 | 29 | Plaintext::Whole { file: downstream/pytest.sh } |  |  | 0.569 |
| walker |  | 7087 | 79 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 17, sub: 0, line: 293 } |  |  | 0.569 |
| walker |  | 7167 | 80 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 12, sub: 0, line: 164 } |  |  | 0.569 |
| ns | 7193 |  | 200 | Complete attribute sets of `HookImpl` and `HookSpec` (`__slots__`) | 3.13 |  | 0.582 |
| walker |  | 7260 | 93 | Plaintext::DeclSurface { file: downstream/datasette.sh } |  |  | 0.582 |
| walker |  | 7290 | 30 | Plaintext::Whole { file: downstream/datasette.sh } |  |  | 0.582 |
| walker |  | 7371 | 81 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 9, sub: 0, line: 77 } |  |  | 0.582 |
| walker |  | 7466 | 95 | Plaintext::DeclSurface { file: downstream/hatch.sh } |  |  | 0.582 |
| walker |  | 7495 | 29 | Plaintext::Whole { file: downstream/hatch.sh } |  |  | 0.582 |
| ns | 7503 |  | 310 | `load_setuptools_entrypoints`: how third-party plugins are discovered | 3.14 | 2.1 | 0.567 |
| walker |  | 7530 | 35 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 19, sub: 0, line: 242 } |  |  | 0.567 |
| walker |  | 7635 | 105 | Plaintext::DeclSurface { file: downstream/devpi.sh } |  |  | 0.567 |
| walker |  | 7686 | 51 | Plaintext::Whole { file: downstream/devpi.sh } |  |  | 0.567 |
| ns | 7697 |  | 194 | Blocking semantics: `set_blocked`, `is_blocked`, `unblock` | 3.15 | 2.1 | 0.563 |
| ns | 7929 |  | 232 | `docs/api_reference.rst`: exactly which types are publicly documented | 4.1 |  | 0.556 |
| walker |  | 8227 | 541 | Markdown::Section { file: README.rst, section_index: 3, keeps_default_concavity: false } |  |  | 0.556 |
| walker |  | 8239 | 12 | Code::CodeKey { rung: Body, file: src/pluggy/_tracing.py, decl: 5, sub: 0, line: 22 } |  |  | 0.556 |
| ns | 8259 |  | 330 | `docs/examples/toy-example.py`: the canonical end-to-end usage | 4.2 |  | 0.541 |
| walker |  | 8337 | 98 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 33, sub: 0, line: 516 } |  |  | 0.542 |
| walker |  | 8379 | 42 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 35, sub: 0, line: 487 } |  |  | 0.542 |
| walker |  | 8423 | 44 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 29, sub: 0, line: 379 } |  |  | 0.542 |
| walker |  | 8469 | 46 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 4, sub: 0, line: 52 } |  |  | 0.549 |
| walker |  | 8482 | 13 | Code::CodeKey { rung: Body, file: src/pluggy/_tracing.py, decl: 12, sub: 0, line: 64 } |  |  | 0.549 |
| walker |  | 8532 | 50 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 36, sub: 0, line: 512 } |  |  | 0.549 |
| ns | 8540 |  | 281 | The eggsample host program: wiring a PluginManager and calling a hook | 4.3 |  | 0.538 |
| walker |  | 8563 | 31 | Code::CodeKey { rung: Doc, file: src/pluggy/_callers.py, decl: 2, sub: 0, line: 27 } |  |  | 0.538 |
| walker |  | 8717 | 154 | Plaintext::DeclSurface { file: downstream/python-lsp-server.sh } |  |  | 0.538 |
| ns | 8760 |  | 220 | `eggsample/hookspecs.py` in full: what a real hookspec module looks like | 4.4 |  | 0.529 |
| walker |  | 8783 | 66 | Code::CodeKey { rung: Body, file: src/pluggy/__init__.py, decl: 1, sub: 0, line: 32 } |  |  | 0.541 |
| walker |  | 8844 | 61 | Code::CodeKey { rung: Doc, file: src/pluggy/_result.py, decl: 10, sub: 0, line: 80 } |  |  | 0.541 |
| walker |  | 8861 | 17 | Code::CodeKey { rung: Body, file: src/pluggy/_tracing.py, decl: 13, sub: 0, line: 67 } |  |  | 0.541 |
| walker |  | 8924 | 63 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 20, sub: 0, line: 252 } |  |  | 0.541 |
| walker |  | 8987 | 63 | Code::CodeKey { rung: Doc, file: src/pluggy/_result.py, decl: 11, sub: 0, line: 91 } |  |  | 0.541 |
| walker |  | 9052 | 65 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 33, sub: 0, line: 434 } |  |  | 0.541 |
| walker |  | 9070 | 18 | Code::CodeKey { rung: Body, file: src/pluggy/_tracing.py, decl: 11, sub: 0, line: 60 } |  |  | 0.541 |
| ns | 9096 |  | 336 | Both sides of hook implementation: the host's own `lib.py` and the external plugin `eggsample_spam.py` | 4.5 |  | 0.529 |
| ns | 9239 |  | 143 | Entry-point wiring in both example `setup.py` files | 4.6 |  | 0.525 |
| walker |  | 9259 | 189 | Plaintext::Whole { file: downstream/python-lsp-server.sh } |  |  | 0.525 |
| walker |  | 9346 | 87 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 16, sub: 0, line: 201 } |  |  | 0.525 |
| ns | 9487 |  | 248 | `testing/conftest.py` in full: the two fixtures every test in the suite uses | 5.1 |  | 0.516 |
| ns | 9646 |  | 159 | `pyproject.toml`: package identity, Python floor, dependency groups and the src layout | 5.2 |  | 0.511 |
| ns | 9825 |  | 179 | `tox.ini`: the environment list and the embedded pytest configuration | 5.3 |  | 0.506 |
| walker |  | 9912 | 566 | Plaintext::Whole { file: tox.ini } |  |  | 0.522 |
| ns | 9915 |  | 90 | `[tool.towncrier]` config: how CHANGELOG.rst is produced | 5.4 |  | 0.519 |
| ns | 9948 |  | 33 | Listings of `changelog/` and `scripts/` | 5.5 |  | 0.522 |
| walker |  | 9968 | 56 | Code::CodeKey { rung: Doc, file: src/pluggy/_callers.py, decl: 5, sub: 0, line: 82 } |  |  | 0.522 |
| walker |  | 9983 | 15 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 24, sub: 0, line: 304 } |  |  | 0.522 |

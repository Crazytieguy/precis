Score(3000)=0.628 I=0.771 C=0.512 ns_rows≤3K=16/43 grid(1000/1442/2080/3000/4327/6240/9000)=0.803/0.755/0.627/0.628/0.585/0.546/0.541

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 62 |  | 62 | Repository identity: README title, dependents, and the docs tagline | 1.1 |  | 0.000 |
| walker |  | 96 | 96 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 100 | 4 | Fs::DirListing { dir: src } |  |  | 0.000 |
| ns | 107 |  | 45 | Complete listing of the shipped package directory `src/pluggy/` | 1.2 |  | 0.000 |
| walker |  | 119 | 19 | Fs::DirListing { dir: changelog } |  |  | 0.000 |
| walker |  | 123 | 4 | Fs::DirListing { dir: .claude } |  |  | 0.000 |
| walker |  | 155 | 32 | Fs::DirListing { dir: docs } |  |  | 0.000 |
| walker |  | 158 | 3 | Fs::DirListing { dir: docs/_static } |  |  | 0.000 |
| walker |  | 162 | 4 | Fs::DirListing { dir: docs/_static/img } |  |  | 0.000 |
| ns | 203 |  | 96 | Complete repository root listing | 1.3 |  | 0.445 |
| walker |  | 215 | 53 | Toml::Identity { file: pyproject.toml } |  |  | 0.445 |
| walker |  | 258 | 43 | Fs::DirListing { dir: downstream } |  |  | 0.445 |
| walker |  | 303 | 45 | Fs::DirListing { dir: src/pluggy } |  |  | 0.723 |
| ns | 355 |  | 152 | The complete public API name list (`__all__` of src/pluggy/__init__.py) | 1.4 |  | 0.577 |
| walker |  | 361 | 58 | Markdown::ReadmeHeadline { file: README.rst } |  |  | 0.729 |
| walker |  | 376 | 15 | Markdown::Section { file: README.rst, section_index: 0, keeps_default_concavity: false } |  |  | 0.729 |
| walker |  | 389 | 13 | Fs::DirListing { dir: .github } |  |  | 0.729 |
| walker |  | 393 | 4 | Fs::DirListing { dir: .github/workflows } |  |  | 0.729 |
| walker |  | 407 | 14 | Fs::DirListing { dir: scripts } |  |  | 0.729 |
| walker |  | 433 | 26 | Markdown::Section { file: downstream/README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.729 |
| walker |  | 448 | 15 | Fs::DirListing { dir: docs/examples } |  |  | 0.730 |
| walker |  | 456 | 8 | Fs::DirListing { dir: docs/examples/eggsample } |  |  | 0.731 |
| walker |  | 476 | 20 | Fs::DirListing { dir: docs/examples/eggsample/eggsample } |  |  | 0.736 |
| ns | 521 |  | 166 | Public name to private module map (the re-export block) | 1.5 | 1.4 | 0.643 |
| walker |  | 556 | 80 | Toml::PackageMetadata { file: pyproject.toml } |  |  | 0.643 |
| ns | 615 |  | 94 | Complete listings of `testing/` and `docs/` | 1.6 |  | 0.577 |
| walker |  | 618 | 62 | Fs::DirListing { dir: testing } |  |  | 0.703 |
| walker |  | 665 | 47 | Plaintext::Whole { file: docs/requirements.txt } |  |  | 0.703 |
| ns | 839 |  | 224 | What pluggy is and what problem it solves (docs/index.rst lede) | 1.7 |  | 0.645 |
| ns | 893 |  | 54 | Complete listing of the two worked example packages under docs/examples/ | 1.8 |  | 0.645 |
| ns | 985 |  | 92 | Lazy `__version__` resolution in `__init__.py` | 1.9 | 1.5 | 0.615 |
| walker |  | 1009 | 344 | Code::CodeKey { rung: Names, file: src/pluggy/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.806 |
| walker |  | 1075 | 66 | Code::CodeKey { rung: Body, file: src/pluggy/__init__.py, decl: 1, sub: 0, line: 32 } |  |  | 0.851 |
| walker |  | 1115 | 40 | Plaintext::DeclSurface { file: changelog/590.trivial.rst } |  |  | 0.851 |
| walker |  | 1212 | 97 | Plaintext::DeclSurface { file: tox.ini } |  |  | 0.851 |
| ns | 1296 |  | 311 | Every method name on `PluginManager` (complete roster, names only) | 2.1 |  | 0.755 |
| walker |  | 1346 | 134 | Plaintext::DeclSurface { file: TIDELIFT.rst } |  |  | 0.755 |
| walker |  | 1517 | 171 | Markdown::Section { file: README.rst, section_index: 4, keeps_default_concavity: false } |  |  | 0.755 |
| ns | 1528 |  | 232 | The rest of `_manager.py`: PluginValidationError, DistFacade, and the two module helpers | 2.2 |  | 0.701 |
| walker |  | 1759 | 242 | Markdown::Section { file: README.rst, section_index: 1, keeps_default_concavity: false } |  |  | 0.701 |
| walker |  | 1839 | 80 | Code::CodeKey { rung: Names, file: src/pluggy/_callers.py, decl: 0, sub: 0, line: 0 } |  |  | 0.701 |
| ns | 1857 |  | 329 | Every top-level symbol in `_hooks.py` (complete roster) plus the two backward-compat aliases | 2.3 |  | 0.648 |
| walker |  | 1868 | 29 | Code::CodeKey { rung: Decl, file: src/pluggy/_callers.py, decl: 4, sub: 0, line: 70 } |  |  | 0.649 |
| walker |  | 1899 | 31 | Code::CodeKey { rung: Decl, file: src/pluggy/_callers.py, decl: 2, sub: 0, line: 27 } |  |  | 0.649 |
| walker |  | 1933 | 34 | Code::CodeKey { rung: Decl, file: src/pluggy/_callers.py, decl: 3, sub: 0, line: 60 } |  |  | 0.650 |
| walker |  | 1992 | 59 | Code::CodeKey { rung: Decl, file: src/pluggy/_callers.py, decl: 5, sub: 0, line: 82 } |  |  | 0.652 |
| ns | 2019 |  | 162 | Every method name on `HookCaller` (complete roster, names only) | 2.4 |  | 0.627 |
| walker |  | 2023 | 31 | Code::CodeKey { rung: Doc, file: src/pluggy/_callers.py, decl: 2, sub: 0, line: 27 } |  |  | 0.627 |
| walker |  | 2079 | 56 | Code::CodeKey { rung: Doc, file: src/pluggy/_callers.py, decl: 5, sub: 0, line: 82 } |  |  | 0.627 |
| walker |  | 2112 | 33 | Code::CodeKey { rung: Names, file: src/pluggy/_warnings.py, decl: 0, sub: 0, line: 0 } |  |  | 0.627 |
| walker |  | 2127 | 15 | Code::CodeKey { rung: Decl, file: src/pluggy/_warnings.py, decl: 1, sub: 0, line: 4 } |  |  | 0.627 |
| walker |  | 2145 | 18 | Code::CodeKey { rung: Decl, file: src/pluggy/_warnings.py, decl: 2, sub: 0, line: 10 } |  |  | 0.627 |
| walker |  | 2159 | 14 | Code::CodeKey { rung: Doc, file: src/pluggy/_warnings.py, decl: 1, sub: 0, line: 4 } |  |  | 0.627 |
| walker |  | 2226 | 67 | Code::CodeKey { rung: Names, file: src/pluggy/_tracing.py, decl: 0, sub: 0, line: 0 } |  |  | 0.627 |
| walker |  | 2293 | 67 | Code::CodeKey { rung: Decl, file: src/pluggy/_tracing.py, decl: 10, sub: 0, line: 59 } |  |  | 0.628 |
| ns | 2301 |  | 282 | Complete `Result` API with signatures (`_result.py`) | 2.5 |  | 0.593 |
| walker |  | 2306 | 13 | Code::CodeKey { rung: Body, file: src/pluggy/_tracing.py, decl: 12, sub: 0, line: 64 } |  |  | 0.593 |
| walker |  | 2323 | 17 | Code::CodeKey { rung: Body, file: src/pluggy/_tracing.py, decl: 13, sub: 0, line: 67 } |  |  | 0.593 |
| walker |  | 2341 | 18 | Code::CodeKey { rung: Body, file: src/pluggy/_tracing.py, decl: 11, sub: 0, line: 60 } |  |  | 0.593 |
| walker |  | 2484 | 143 | Code::CodeKey { rung: Decl, file: src/pluggy/_tracing.py, decl: 3, sub: 0, line: 16 } |  |  | 0.596 |
| walker |  | 2493 | 9 | Code::CodeKey { rung: Body, file: src/pluggy/_tracing.py, decl: 8, sub: 0, line: 48 } |  |  | 0.596 |
| walker |  | 2505 | 12 | Code::CodeKey { rung: Body, file: src/pluggy/_tracing.py, decl: 5, sub: 0, line: 22 } |  |  | 0.596 |
| ns | 2563 |  | 262 | Complete top-level roster of `_callers.py` (the call loop module) | 2.6 |  | 0.605 |
| walker |  | 2857 | 352 | Code::CodeKey { rung: Names, file: src/pluggy/_hooks.py, decl: 0, sub: 0, line: 0 } |  |  | 0.622 |
| walker |  | 2881 | 24 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 25, sub: 0, line: 377 } |  |  | 0.622 |
| ns | 2887 |  | 324 | Complete symbol rosters for the two remaining modules: `_tracing.py` and `_warnings.py` | 2.7 |  | 0.628 |
| walker |  | 2915 | 34 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 5, sub: 0, line: 33 } |  |  | 0.628 |
| walker |  | 2995 | 80 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 22, sub: 0, line: 358 } |  |  | 0.628 |
| walker |  | 3005 | 10 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 23, sub: 0, line: 365 } |  |  | 0.628 |
| walker |  | 3099 | 94 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 9, sub: 0, line: 77 } |  |  | 0.628 |
| walker |  | 3193 | 94 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 14, sub: 0, line: 164 } |  |  | 0.628 |
| walker |  | 3288 | 95 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 11, sub: 0, line: 91 } |  |  | 0.628 |
| ns | 3298 |  | 411 | Complete section map of the 1082-line manual `docs/index.rst` | 2.8 |  | 0.575 |
| walker |  | 3385 | 97 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 13, sub: 0, line: 111 } |  |  | 0.576 |
| walker |  | 3488 | 103 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 12, sub: 0, line: 101 } |  |  | 0.576 |
| ns | 3543 |  | 245 | The real call signatures of `@hookspec` and `@hookimpl` — every accepted option with its default | 3.1 |  | 0.566 |
| walker |  | 3598 | 110 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 16, sub: 0, line: 178 } |  |  | 0.567 |
| walker |  | 3710 | 112 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 18, sub: 0, line: 202 } |  |  | 0.599 |
| ns | 3791 |  | 248 | `HookimplOpts` in full: every hook-implementation option and what it means | 3.2 | 2.3 | 0.579 |
| walker |  | 3828 | 118 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 17, sub: 0, line: 190 } |  |  | 0.579 |
| walker |  | 3957 | 129 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 50, sub: 0, line: 696 } |  |  | 0.580 |
| ns | 3975 |  | 184 | `HookspecOpts` in full: every hook-specification option | 3.3 | 2.3 | 0.567 |
| walker |  | 4108 | 151 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 47, sub: 0, line: 638 } |  |  | 0.570 |
| walker |  | 4171 | 63 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 48, sub: 0, line: 656 } |  |  | 0.570 |
| walker |  | 4183 | 12 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 48, sub: 0, line: 656 } |  |  | 0.570 |
| walker |  | 4199 | 16 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 47, sub: 0, line: 638 } |  |  | 0.573 |
| walker |  | 4363 | 164 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 7, sub: 0, line: 40 } |  |  | 0.593 |
| walker |  | 4373 | 10 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 7, sub: 0, line: 40 } |  |  | 0.597 |
| walker |  | 4402 | 29 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 22, sub: 0, line: 358 } |  |  | 0.604 |
| ns | 4407 |  | 432 | Hook implementation ordering: the `_hookimpls` layout comment and `_add_hookimpl` in full | 3.4 | 2.4 | 0.574 |
| ns | 4613 |  | 206 | `HookCaller.__call__` in full: what `pm.hook.myhook(...)` actually does | 3.5 | 2.4 | 0.561 |
| walker |  | 4628 | 226 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 8, sub: 0, line: 56 } |  |  | 0.588 |
| walker |  | 4638 | 10 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 8, sub: 0, line: 56 } |  |  | 0.592 |
| walker |  | 4932 | 294 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 26, sub: 0, line: 382 } |  |  | 0.616 |
| walker |  | 4963 | 31 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 38, sub: 0, line: 543 } |  |  | 0.616 |
| walker |  | 5004 | 41 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 29, sub: 0, line: 424 } |  |  | 0.616 |
| walker |  | 5056 | 52 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 37, sub: 0, line: 516 } |  |  | 0.617 |
| ns | 5078 |  | 465 | `_multicall` part 1: the setup / non-wrapper call loop | 3.6 |  | 0.583 |
| walker |  | 5126 | 70 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 27, sub: 0, line: 393 } |  |  | 0.583 |
| walker |  | 5138 | 12 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 27, sub: 0, line: 393 } |  |  | 0.583 |
| walker |  | 5152 | 14 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 33, sub: 0, line: 453 } |  |  | 0.584 |
| walker |  | 5167 | 15 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 26, sub: 0, line: 382 } |  |  | 0.588 |
| walker |  | 5182 | 15 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 32, sub: 0, line: 449 } |  |  | 0.588 |
| walker |  | 5200 | 18 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 30, sub: 0, line: 438 } |  |  | 0.588 |
| walker |  | 5221 | 21 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 39, sub: 0, line: 577 } |  |  | 0.588 |
| walker |  | 5266 | 45 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 38, sub: 0, line: 543 } |  |  | 0.588 |
| ns | 5521 |  | 443 | `_multicall` part 2: the teardown loop, exception routing and `firstresult` collapse | 3.7 | 3.6 | 0.556 |
| walker |  | 5606 | 340 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 41, sub: 0, line: 593 } |  |  | 0.556 |
| walker |  | 5614 | 8 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 44, sub: 0, line: 626 } |  |  | 0.556 |
| walker |  | 5622 | 8 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 45, sub: 0, line: 630 } |  |  | 0.556 |
| walker |  | 5637 | 15 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 43, sub: 0, line: 618 } |  |  | 0.556 |
| walker |  | 5667 | 30 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 41, sub: 0, line: 593 } |  |  | 0.564 |
| ns | 5677 |  | 156 | `PluginManager.register` docstring: naming, blocking and the duplicate-registration contract | 3.8 | 2.1 | 0.555 |
| walker |  | 5744 | 77 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 36, sub: 0, line: 499 } |  |  | 0.560 |
| walker |  | 5823 | 79 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 21, sub: 0, line: 293 } |  |  | 0.560 |
| walker |  | 5903 | 80 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 14, sub: 0, line: 164 } |  |  | 0.560 |
| ns | 5973 |  | 296 | `PluginManager.register` body: how hook implementations are discovered and attached | 3.9 | 3.8 | 0.546 |
| walker |  | 5984 | 81 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 9, sub: 0, line: 77 } |  |  | 0.546 |
| walker |  | 6082 | 98 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 37, sub: 0, line: 516 } |  |  | 0.546 |
| walker |  | 6166 | 84 | Plaintext::DeclSurface { file: downstream/tox.sh } |  |  | 0.546 |
| walker |  | 6195 | 29 | Plaintext::Whole { file: downstream/tox.sh } |  |  | 0.546 |
| walker |  | 6280 | 85 | Code::CodeKey { rung: Names, file: src/pluggy/_result.py, decl: 0, sub: 0, line: 0 } |  |  | 0.547 |
| walker |  | 6291 | 11 | Code::CodeKey { rung: Doc, file: src/pluggy/_result.py, decl: 3, sub: 0, line: 20 } |  |  | 0.548 |
| ns | 6431 |  | 458 | `PluginManager._verify_hook`: every validation error message pluggy can raise | 3.10 | 2.1 | 0.528 |
| walker |  | 6450 | 159 | Code::CodeKey { rung: Decl, file: src/pluggy/_result.py, decl: 4, sub: 0, line: 24 } |  |  | 0.541 |
| walker |  | 6458 | 8 | Code::CodeKey { rung: Decl, file: src/pluggy/_result.py, decl: 6, sub: 0, line: 42 } |  |  | 0.542 |
| walker |  | 6466 | 8 | Code::CodeKey { rung: Decl, file: src/pluggy/_result.py, decl: 7, sub: 0, line: 51 } |  |  | 0.544 |
| walker |  | 6474 | 8 | Code::CodeKey { rung: Decl, file: src/pluggy/_result.py, decl: 8, sub: 0, line: 56 } |  |  | 0.546 |
| walker |  | 6514 | 40 | Code::CodeKey { rung: Decl, file: src/pluggy/_result.py, decl: 5, sub: 0, line: 31 } |  |  | 0.546 |
| walker |  | 6526 | 12 | Code::CodeKey { rung: Doc, file: src/pluggy/_result.py, decl: 5, sub: 0, line: 31 } |  |  | 0.546 |
| walker |  | 6538 | 12 | Code::CodeKey { rung: Doc, file: src/pluggy/_result.py, decl: 6, sub: 0, line: 42 } |  |  | 0.546 |
| walker |  | 6550 | 12 | Code::CodeKey { rung: Doc, file: src/pluggy/_result.py, decl: 7, sub: 0, line: 51 } |  |  | 0.546 |
| walker |  | 6562 | 12 | Code::CodeKey { rung: Doc, file: src/pluggy/_result.py, decl: 8, sub: 0, line: 56 } |  |  | 0.546 |
| walker |  | 6594 | 32 | Code::CodeKey { rung: Doc, file: src/pluggy/_result.py, decl: 4, sub: 0, line: 24 } |  |  | 0.551 |
| ns | 6627 |  | 196 | `PluginManager.__init__`: the complete state of a plugin manager | 3.11 | 2.1 | 0.544 |
| walker |  | 6655 | 61 | Code::CodeKey { rung: Doc, file: src/pluggy/_result.py, decl: 10, sub: 0, line: 80 } |  |  | 0.544 |
| walker |  | 6718 | 63 | Code::CodeKey { rung: Doc, file: src/pluggy/_result.py, decl: 11, sub: 0, line: 91 } |  |  | 0.544 |
| ns | 6993 |  | 366 | Historic hooks end to end: `set_specification`, `call_historic`, `_maybe_apply_history` | 3.12 | 2.4 | 0.531 |
| walker |  | 7012 | 294 | Markdown::Section { file: README.rst, section_index: 2, keeps_default_concavity: false } |  |  | 0.531 |
| walker |  | 7132 | 120 | Code::CodeKey { rung: Names, file: src/pluggy/_manager.py, decl: 0, sub: 0, line: 0 } |  |  | 0.532 |
| walker |  | 7157 | 25 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 4, sub: 0, line: 52 } |  |  | 0.534 |
| walker |  | 7183 | 26 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 2, sub: 0, line: 37 } |  |  | 0.534 |
| ns | 7193 |  | 200 | Complete attribute sets of `HookImpl` and `HookSpec` (`__slots__`) | 3.13 |  | 0.548 |
| walker |  | 7268 | 85 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 6, sub: 0, line: 65 } |  |  | 0.550 |
| walker |  | 7276 | 8 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 8, sub: 0, line: 71 } |  |  | 0.551 |
| walker |  | 7287 | 11 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 6, sub: 0, line: 65 } |  |  | 0.553 |
| walker |  | 7296 | 9 | Code::CodeKey { rung: Body, file: src/pluggy/_manager.py, decl: 7, sub: 0, line: 68 } |  |  | 0.553 |
| walker |  | 7309 | 13 | Code::CodeKey { rung: Body, file: src/pluggy/_manager.py, decl: 9, sub: 0, line: 76 } |  |  | 0.553 |
| walker |  | 7355 | 46 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 4, sub: 0, line: 52 } |  |  | 0.561 |
| walker |  | 7371 | 16 | Code::CodeKey { rung: Body, file: src/pluggy/_manager.py, decl: 37, sub: 0, line: 525 } |  |  | 0.564 |
| walker |  | 7458 | 87 | Plaintext::DeclSurface { file: downstream/conda.sh } |  |  | 0.564 |
| walker |  | 7500 | 42 | Plaintext::Whole { file: downstream/conda.sh } |  |  | 0.564 |
| ns | 7503 |  | 310 | `load_setuptools_entrypoints`: how third-party plugins are discovered | 3.14 | 2.1 | 0.550 |
| walker |  | 7590 | 90 | Plaintext::DeclSurface { file: docs/api_reference.rst } |  |  | 0.550 |
| ns | 7697 |  | 194 | Blocking semantics: `set_blocked`, `is_blocked`, `unblock` | 3.15 | 2.1 | 0.541 |
| walker |  | 7803 | 213 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 11, sub: 0, line: 83 } |  |  | 0.549 |
| walker |  | 7834 | 31 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 21, sub: 0, line: 278 } |  |  | 0.549 |
| walker |  | 7869 | 35 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 16, sub: 0, line: 201 } |  |  | 0.549 |
| ns | 7929 |  | 232 | `docs/api_reference.rst`: exactly which types are publicly documented | 4.1 |  | 0.543 |
| walker |  | 7933 | 64 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 13, sub: 0, line: 114 } |  |  | 0.543 |
| walker |  | 7948 | 15 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 18, sub: 0, line: 238 } |  |  | 0.543 |
| walker |  | 7963 | 15 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 22, sub: 0, line: 296 } |  |  | 0.543 |
| walker |  | 7981 | 18 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 17, sub: 0, line: 233 } |  |  | 0.544 |
| walker |  | 8016 | 35 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 19, sub: 0, line: 242 } |  |  | 0.548 |
| walker |  | 8079 | 63 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 20, sub: 0, line: 252 } |  |  | 0.548 |
| ns | 8259 |  | 330 | `docs/examples/toy-example.py`: the canonical end-to-end usage | 4.2 |  | 0.533 |
| walker |  | 8375 | 296 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 11, sub: 1, line: 83 } |  |  | 0.561 |
| walker |  | 8403 | 28 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 36, sub: 0, line: 512 } |  |  | 0.561 |
| walker |  | 8434 | 31 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 34, sub: 0, line: 450 } |  |  | 0.561 |
| walker |  | 8448 | 14 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 23, sub: 0, line: 300 } |  |  | 0.561 |
| walker |  | 8465 | 17 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 26, sub: 0, line: 319 } |  |  | 0.561 |
| walker |  | 8483 | 18 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 25, sub: 0, line: 315 } |  |  | 0.561 |
| walker |  | 8504 | 21 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 32, sub: 0, line: 430 } |  |  | 0.561 |
| walker |  | 8533 | 29 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 27, sub: 0, line: 323 } |  |  | 0.561 |
| ns | 8540 |  | 281 | The eggsample host program: wiring a PluginManager and calling a hook | 4.3 |  | 0.551 |
| walker |  | 8563 | 30 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 31, sub: 0, line: 425 } |  |  | 0.551 |
| walker |  | 8605 | 42 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 35, sub: 0, line: 487 } |  |  | 0.551 |
| walker |  | 8649 | 44 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 29, sub: 0, line: 379 } |  |  | 0.551 |
| walker |  | 8699 | 50 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 36, sub: 0, line: 512 } |  |  | 0.551 |
| ns | 8760 |  | 220 | `eggsample/hookspecs.py` in full: what a real hookspec module looks like | 4.4 |  | 0.541 |
| walker |  | 8764 | 65 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 33, sub: 0, line: 434 } |  |  | 0.541 |
| walker |  | 8855 | 91 | Plaintext::DeclSurface { file: downstream/pytest.sh } |  |  | 0.541 |
| walker |  | 8884 | 29 | Plaintext::Whole { file: downstream/pytest.sh } |  |  | 0.541 |
| walker |  | 8977 | 93 | Plaintext::DeclSurface { file: downstream/datasette.sh } |  |  | 0.541 |
| walker |  | 9007 | 30 | Plaintext::Whole { file: downstream/datasette.sh } |  |  | 0.541 |
| walker |  | 9094 | 87 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 16, sub: 0, line: 201 } |  |  | 0.541 |
| ns | 9096 |  | 336 | Both sides of hook implementation: the host's own `lib.py` and the external plugin `eggsample_spam.py` | 4.5 |  | 0.529 |
| walker |  | 9189 | 95 | Plaintext::DeclSurface { file: downstream/hatch.sh } |  |  | 0.529 |
| walker |  | 9218 | 29 | Plaintext::Whole { file: downstream/hatch.sh } |  |  | 0.529 |
| ns | 9239 |  | 143 | Entry-point wiring in both example `setup.py` files | 4.6 |  | 0.525 |
| walker |  | 9317 | 99 | Plaintext::DeclSurface { file: changelog/_template.rst } |  |  | 0.525 |
| walker |  | 9414 | 97 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 24, sub: 0, line: 304 } |  |  | 0.525 |
| ns | 9487 |  | 248 | `testing/conftest.py` in full: the two fixtures every test in the suite uses | 5.1 |  | 0.516 |
| walker |  | 9513 | 99 | Code::CodeKey { rung: Doc, file: src/pluggy/_result.py, decl: 9, sub: 0, line: 67 } |  |  | 0.516 |
| walker |  | 9621 | 108 | Plaintext::DeclSurface { file: downstream/devpi.sh } |  |  | 0.516 |
| ns | 9646 |  | 159 | `pyproject.toml`: package identity, Python floor, dependency groups and the src layout | 5.2 |  | 0.513 |
| walker |  | 9669 | 48 | Plaintext::Whole { file: downstream/devpi.sh } |  |  | 0.513 |
| walker |  | 9772 | 103 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 30, sub: 0, line: 395 } |  |  | 0.517 |
| walker |  | 9783 | 11 | Fs::DirListing { dir: docs/examples/eggsample-spam } |  |  | 0.521 |
| ns | 9825 |  | 179 | `tox.ini`: the environment list and the embedded pytest configuration | 5.3 |  | 0.517 |
| ns | 9915 |  | 90 | `[tool.towncrier]` config: how CHANGELOG.rst is produced | 5.4 |  | 0.514 |
| ns | 9948 |  | 33 | Listings of `changelog/` and `scripts/` | 5.5 |  | 0.517 |
| walker |  | 9984 | 201 | Toml::Config { file: pyproject.toml } |  |  | 0.521 |

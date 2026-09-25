Score(3000)=0.657 I=0.778 C=0.555 ns_rows≤3K=16/43 grid(1000/1442/2080/3000/4327/6240/9000)=0.806/0.756/0.631/0.657/0.584/0.514/0.533

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
| ns | 839 |  | 224 | What pluggy is and what problem it solves (docs/index.rst lede) | 1.7 |  | 0.645 |
| ns | 893 |  | 54 | Complete listing of the two worked example packages under docs/examples/ | 1.8 |  | 0.645 |
| walker |  | 962 | 344 | Code::CodeKey { rung: Names, file: src/pluggy/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.843 |
| ns | 985 |  | 92 | Lazy `__version__` resolution in `__init__.py` | 1.9 | 1.5 | 0.806 |
| walker |  | 1028 | 66 | Code::CodeKey { rung: Body, file: src/pluggy/__init__.py, decl: 1, sub: 0, line: 32 } |  |  | 0.851 |
| walker |  | 1061 | 33 | Code::CodeKey { rung: Names, file: src/pluggy/_warnings.py, decl: 0, sub: 0, line: 0 } |  |  | 0.851 |
| walker |  | 1076 | 15 | Code::CodeKey { rung: Decl, file: src/pluggy/_warnings.py, decl: 1, sub: 0, line: 4 } |  |  | 0.851 |
| walker |  | 1094 | 18 | Code::CodeKey { rung: Decl, file: src/pluggy/_warnings.py, decl: 2, sub: 0, line: 10 } |  |  | 0.851 |
| walker |  | 1108 | 14 | Code::CodeKey { rung: Doc, file: src/pluggy/_warnings.py, decl: 1, sub: 0, line: 4 } |  |  | 0.851 |
| walker |  | 1155 | 47 | Plaintext::Whole { file: docs/requirements.txt } |  |  | 0.851 |
| walker |  | 1222 | 67 | Code::CodeKey { rung: Names, file: src/pluggy/_tracing.py, decl: 0, sub: 0, line: 0 } |  |  | 0.851 |
| walker |  | 1289 | 67 | Code::CodeKey { rung: Decl, file: src/pluggy/_tracing.py, decl: 10, sub: 0, line: 59 } |  |  | 0.852 |
| ns | 1296 |  | 311 | Every method name on `PluginManager` (complete roster, names only) | 2.1 |  | 0.756 |
| walker |  | 1302 | 13 | Code::CodeKey { rung: Body, file: src/pluggy/_tracing.py, decl: 12, sub: 0, line: 64 } |  |  | 0.756 |
| walker |  | 1319 | 17 | Code::CodeKey { rung: Body, file: src/pluggy/_tracing.py, decl: 13, sub: 0, line: 67 } |  |  | 0.756 |
| walker |  | 1337 | 18 | Code::CodeKey { rung: Body, file: src/pluggy/_tracing.py, decl: 11, sub: 0, line: 60 } |  |  | 0.756 |
| walker |  | 1480 | 143 | Code::CodeKey { rung: Decl, file: src/pluggy/_tracing.py, decl: 3, sub: 0, line: 16 } |  |  | 0.759 |
| walker |  | 1489 | 9 | Code::CodeKey { rung: Body, file: src/pluggy/_tracing.py, decl: 8, sub: 0, line: 48 } |  |  | 0.759 |
| walker |  | 1501 | 12 | Code::CodeKey { rung: Body, file: src/pluggy/_tracing.py, decl: 5, sub: 0, line: 22 } |  |  | 0.759 |
| ns | 1528 |  | 232 | The rest of `_manager.py`: PluginValidationError, DistFacade, and the two module helpers | 2.2 |  | 0.705 |
| walker |  | 1541 | 40 | Plaintext::DeclSurface { file: changelog/590.trivial.rst } |  |  | 0.705 |
| walker |  | 1638 | 97 | Plaintext::DeclSurface { file: tox.ini } |  |  | 0.705 |
| walker |  | 1718 | 80 | Code::CodeKey { rung: Names, file: src/pluggy/_callers.py, decl: 0, sub: 0, line: 0 } |  |  | 0.705 |
| walker |  | 1749 | 31 | Code::CodeKey { rung: Decl, file: src/pluggy/_callers.py, decl: 2, sub: 0, line: 27 } |  |  | 0.706 |
| walker |  | 1778 | 29 | Code::CodeKey { rung: Decl, file: src/pluggy/_callers.py, decl: 4, sub: 0, line: 70 } |  |  | 0.706 |
| walker |  | 1812 | 34 | Code::CodeKey { rung: Decl, file: src/pluggy/_callers.py, decl: 3, sub: 0, line: 60 } |  |  | 0.707 |
| walker |  | 1843 | 31 | Code::CodeKey { rung: Doc, file: src/pluggy/_callers.py, decl: 2, sub: 0, line: 27 } |  |  | 0.707 |
| ns | 1857 |  | 329 | Every top-level symbol in `_hooks.py` (complete roster) plus the two backward-compat aliases | 2.3 |  | 0.654 |
| walker |  | 1902 | 59 | Code::CodeKey { rung: Decl, file: src/pluggy/_callers.py, decl: 5, sub: 0, line: 82 } |  |  | 0.656 |
| walker |  | 1987 | 85 | Code::CodeKey { rung: Names, file: src/pluggy/_result.py, decl: 0, sub: 0, line: 0 } |  |  | 0.656 |
| walker |  | 1998 | 11 | Code::CodeKey { rung: Doc, file: src/pluggy/_result.py, decl: 3, sub: 0, line: 20 } |  |  | 0.656 |
| ns | 2019 |  | 162 | Every method name on `HookCaller` (complete roster, names only) | 2.4 |  | 0.631 |
| walker |  | 2157 | 159 | Code::CodeKey { rung: Decl, file: src/pluggy/_result.py, decl: 4, sub: 0, line: 24 } |  |  | 0.633 |
| walker |  | 2165 | 8 | Code::CodeKey { rung: Decl, file: src/pluggy/_result.py, decl: 6, sub: 0, line: 42 } |  |  | 0.634 |
| walker |  | 2173 | 8 | Code::CodeKey { rung: Decl, file: src/pluggy/_result.py, decl: 7, sub: 0, line: 51 } |  |  | 0.634 |
| walker |  | 2181 | 8 | Code::CodeKey { rung: Decl, file: src/pluggy/_result.py, decl: 8, sub: 0, line: 56 } |  |  | 0.634 |
| walker |  | 2221 | 40 | Code::CodeKey { rung: Decl, file: src/pluggy/_result.py, decl: 5, sub: 0, line: 31 } |  |  | 0.634 |
| walker |  | 2233 | 12 | Code::CodeKey { rung: Doc, file: src/pluggy/_result.py, decl: 5, sub: 0, line: 31 } |  |  | 0.634 |
| walker |  | 2245 | 12 | Code::CodeKey { rung: Doc, file: src/pluggy/_result.py, decl: 6, sub: 0, line: 42 } |  |  | 0.634 |
| walker |  | 2257 | 12 | Code::CodeKey { rung: Doc, file: src/pluggy/_result.py, decl: 7, sub: 0, line: 51 } |  |  | 0.634 |
| walker |  | 2269 | 12 | Code::CodeKey { rung: Doc, file: src/pluggy/_result.py, decl: 8, sub: 0, line: 56 } |  |  | 0.634 |
| walker |  | 2301 | 32 | Code::CodeKey { rung: Doc, file: src/pluggy/_result.py, decl: 4, sub: 0, line: 24 } |  |  | 0.649 |
| ns | 2301 |  | 282 | Complete `Result` API with signatures (`_result.py`) | 2.5 |  | 0.649 |
| walker |  | 2435 | 134 | Plaintext::DeclSurface { file: TIDELIFT.rst } |  |  | 0.649 |
| ns | 2563 |  | 262 | Complete top-level roster of `_callers.py` (the call loop module) | 2.6 |  | 0.653 |
| walker |  | 2606 | 171 | Markdown::Section { file: README.rst, section_index: 4, keeps_default_concavity: false } |  |  | 0.653 |
| walker |  | 2667 | 61 | Code::CodeKey { rung: Doc, file: src/pluggy/_result.py, decl: 10, sub: 0, line: 80 } |  |  | 0.653 |
| walker |  | 2730 | 63 | Code::CodeKey { rung: Doc, file: src/pluggy/_result.py, decl: 11, sub: 0, line: 91 } |  |  | 0.653 |
| ns | 2887 |  | 324 | Complete symbol rosters for the two remaining modules: `_tracing.py` and `_warnings.py` | 2.7 |  | 0.657 |
| walker |  | 2972 | 242 | Markdown::Section { file: README.rst, section_index: 1, keeps_default_concavity: false } |  |  | 0.657 |
| walker |  | 3266 | 294 | Markdown::Section { file: README.rst, section_index: 2, keeps_default_concavity: true } |  |  | 0.657 |
| ns | 3298 |  | 411 | Complete section map of the 1082-line manual `docs/index.rst` | 2.8 |  | 0.601 |
| walker |  | 3386 | 120 | Code::CodeKey { rung: Names, file: src/pluggy/_manager.py, decl: 0, sub: 0, line: 0 } |  |  | 0.603 |
| walker |  | 3411 | 25 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 4, sub: 0, line: 52 } |  |  | 0.605 |
| walker |  | 3496 | 85 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 6, sub: 0, line: 65 } |  |  | 0.610 |
| walker |  | 3504 | 8 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 8, sub: 0, line: 71 } |  |  | 0.612 |
| walker |  | 3513 | 9 | Code::CodeKey { rung: Body, file: src/pluggy/_manager.py, decl: 7, sub: 0, line: 68 } |  |  | 0.612 |
| walker |  | 3524 | 11 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 6, sub: 0, line: 65 } |  |  | 0.615 |
| ns | 3543 |  | 245 | The real call signatures of `@hookspec` and `@hookimpl` — every accepted option with its default | 3.1 |  | 0.592 |
| walker |  | 3550 | 26 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 2, sub: 0, line: 37 } |  |  | 0.592 |
| walker |  | 3563 | 13 | Code::CodeKey { rung: Body, file: src/pluggy/_manager.py, decl: 9, sub: 0, line: 76 } |  |  | 0.592 |
| walker |  | 3609 | 46 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 4, sub: 0, line: 52 } |  |  | 0.606 |
| walker |  | 3693 | 84 | Plaintext::DeclSurface { file: downstream/tox.sh } |  |  | 0.606 |
| walker |  | 3780 | 87 | Plaintext::DeclSurface { file: downstream/conda.sh } |  |  | 0.606 |
| ns | 3791 |  | 248 | `HookimplOpts` in full: every hook-implementation option and what it means | 3.2 | 2.3 | 0.586 |
| walker |  | 3870 | 90 | Plaintext::DeclSurface { file: docs/api_reference.rst } |  |  | 0.586 |
| ns | 3975 |  | 184 | `HookspecOpts` in full: every hook-specification option | 3.3 | 2.3 | 0.572 |
| walker |  | 4083 | 213 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 11, sub: 0, line: 83 } |  |  | 0.584 |
| walker |  | 4114 | 31 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 21, sub: 0, line: 278 } |  |  | 0.584 |
| walker |  | 4149 | 35 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 16, sub: 0, line: 201 } |  |  | 0.584 |
| walker |  | 4164 | 15 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 18, sub: 0, line: 238 } |  |  | 0.584 |
| walker |  | 4179 | 15 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 22, sub: 0, line: 296 } |  |  | 0.584 |
| walker |  | 4197 | 18 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 17, sub: 0, line: 233 } |  |  | 0.584 |
| walker |  | 4232 | 35 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 19, sub: 0, line: 242 } |  |  | 0.584 |
| walker |  | 4296 | 64 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 13, sub: 0, line: 114 } |  |  | 0.584 |
| walker |  | 4359 | 63 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 20, sub: 0, line: 252 } |  |  | 0.584 |
| ns | 4407 |  | 432 | Hook implementation ordering: the `_hookimpls` layout comment and `_add_hookimpl` in full | 3.4 | 2.4 | 0.555 |
| ns | 4613 |  | 206 | `HookCaller.__call__` in full: what `pm.hook.myhook(...)` actually does | 3.5 | 2.4 | 0.542 |
| walker |  | 4655 | 296 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 11, sub: 1, line: 83 } |  |  | 0.587 |
| walker |  | 4683 | 28 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 36, sub: 0, line: 512 } |  |  | 0.587 |
| walker |  | 4714 | 31 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 34, sub: 0, line: 450 } |  |  | 0.587 |
| walker |  | 4728 | 14 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 23, sub: 0, line: 300 } |  |  | 0.587 |
| walker |  | 4745 | 17 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 26, sub: 0, line: 319 } |  |  | 0.587 |
| walker |  | 4763 | 18 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 25, sub: 0, line: 315 } |  |  | 0.587 |
| walker |  | 4784 | 21 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 32, sub: 0, line: 430 } |  |  | 0.587 |
| walker |  | 4813 | 29 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 27, sub: 0, line: 323 } |  |  | 0.587 |
| walker |  | 4843 | 30 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 31, sub: 0, line: 425 } |  |  | 0.587 |
| walker |  | 4885 | 42 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 35, sub: 0, line: 487 } |  |  | 0.587 |
| walker |  | 4929 | 44 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 29, sub: 0, line: 379 } |  |  | 0.587 |
| walker |  | 4979 | 50 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 36, sub: 0, line: 512 } |  |  | 0.587 |
| walker |  | 5044 | 65 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 33, sub: 0, line: 434 } |  |  | 0.587 |
| ns | 5078 |  | 465 | `_multicall` part 1: the setup / non-wrapper call loop | 3.6 |  | 0.555 |
| walker |  | 5131 | 87 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 16, sub: 0, line: 201 } |  |  | 0.555 |
| walker |  | 5222 | 91 | Plaintext::DeclSurface { file: downstream/pytest.sh } |  |  | 0.555 |
| walker |  | 5315 | 93 | Plaintext::DeclSurface { file: downstream/datasette.sh } |  |  | 0.555 |
| walker |  | 5410 | 95 | Plaintext::DeclSurface { file: downstream/hatch.sh } |  |  | 0.555 |
| walker |  | 5509 | 99 | Plaintext::DeclSurface { file: changelog/_template.rst } |  |  | 0.555 |
| ns | 5521 |  | 443 | `_multicall` part 2: the teardown loop, exception routing and `firstresult` collapse | 3.7 | 3.6 | 0.526 |
| walker |  | 5606 | 97 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 24, sub: 0, line: 304 } |  |  | 0.526 |
| ns | 5677 |  | 156 | `PluginManager.register` docstring: naming, blocking and the duplicate-registration contract | 3.8 | 2.1 | 0.518 |
| walker |  | 5705 | 99 | Code::CodeKey { rung: Doc, file: src/pluggy/_result.py, decl: 9, sub: 0, line: 67 } |  |  | 0.518 |
| ns | 5973 |  | 296 | `PluginManager.register` body: how hook implementations are discovered and attached | 3.9 | 3.8 | 0.505 |
| walker |  | 6057 | 352 | Code::CodeKey { rung: Names, file: src/pluggy/_hooks.py, decl: 0, sub: 0, line: 0 } |  |  | 0.514 |
| walker |  | 6137 | 80 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 22, sub: 0, line: 358 } |  |  | 0.514 |
| walker |  | 6147 | 10 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 23, sub: 0, line: 365 } |  |  | 0.514 |
| walker |  | 6171 | 24 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 25, sub: 0, line: 377 } |  |  | 0.514 |
| walker |  | 6265 | 94 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 9, sub: 0, line: 77 } |  |  | 0.515 |
| walker |  | 6359 | 94 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 14, sub: 0, line: 164 } |  |  | 0.515 |
| ns | 6431 |  | 458 | `PluginManager._verify_hook`: every validation error message pluggy can raise | 3.10 | 2.1 | 0.496 |
| walker |  | 6454 | 95 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 11, sub: 0, line: 91 } |  |  | 0.497 |
| walker |  | 6551 | 97 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 13, sub: 0, line: 111 } |  |  | 0.504 |
| ns | 6627 |  | 196 | `PluginManager.__init__`: the complete state of a plugin manager | 3.11 | 2.1 | 0.497 |
| walker |  | 6654 | 103 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 12, sub: 0, line: 101 } |  |  | 0.497 |
| walker |  | 6764 | 110 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 16, sub: 0, line: 178 } |  |  | 0.498 |
| walker |  | 6876 | 112 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 18, sub: 0, line: 202 } |  |  | 0.519 |
| walker |  | 6910 | 34 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 5, sub: 0, line: 33 } |  |  | 0.519 |
| ns | 6993 |  | 366 | Historic hooks end to end: `set_specification`, `call_historic`, `_maybe_apply_history` | 3.12 | 2.4 | 0.504 |
| walker |  | 7028 | 118 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 17, sub: 0, line: 190 } |  |  | 0.504 |
| walker |  | 7157 | 129 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 50, sub: 0, line: 696 } |  |  | 0.505 |
| ns | 7193 |  | 200 | Complete attribute sets of `HookImpl` and `HookSpec` (`__slots__`) | 3.13 |  | 0.499 |
| walker |  | 7308 | 151 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 47, sub: 0, line: 638 } |  |  | 0.523 |
| walker |  | 7371 | 63 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 48, sub: 0, line: 656 } |  |  | 0.523 |
| walker |  | 7383 | 12 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 48, sub: 0, line: 656 } |  |  | 0.523 |
| walker |  | 7399 | 16 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 47, sub: 0, line: 638 } |  |  | 0.525 |
| ns | 7503 |  | 310 | `load_setuptools_entrypoints`: how third-party plugins are discovered | 3.14 | 2.1 | 0.512 |
| walker |  | 7563 | 164 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 7, sub: 0, line: 40 } |  |  | 0.524 |
| walker |  | 7573 | 10 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 7, sub: 0, line: 40 } |  |  | 0.527 |
| walker |  | 7602 | 29 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 22, sub: 0, line: 358 } |  |  | 0.531 |
| ns | 7697 |  | 194 | Blocking semantics: `set_blocked`, `is_blocked`, `unblock` | 3.15 | 2.1 | 0.528 |
| walker |  | 7828 | 226 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 8, sub: 0, line: 56 } |  |  | 0.546 |
| walker |  | 7838 | 10 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 8, sub: 0, line: 56 } |  |  | 0.549 |
| ns | 7929 |  | 232 | `docs/api_reference.rst`: exactly which types are publicly documented | 4.1 |  | 0.542 |
| walker |  | 8132 | 294 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 26, sub: 0, line: 382 } |  |  | 0.559 |
| walker |  | 8163 | 31 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 38, sub: 0, line: 543 } |  |  | 0.559 |
| walker |  | 8204 | 41 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 29, sub: 0, line: 424 } |  |  | 0.559 |
| walker |  | 8256 | 52 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 37, sub: 0, line: 516 } |  |  | 0.560 |
| ns | 8259 |  | 330 | `docs/examples/toy-example.py`: the canonical end-to-end usage | 4.2 |  | 0.545 |
| walker |  | 8326 | 70 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 27, sub: 0, line: 393 } |  |  | 0.545 |
| walker |  | 8338 | 12 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 27, sub: 0, line: 393 } |  |  | 0.545 |
| walker |  | 8353 | 15 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 26, sub: 0, line: 382 } |  |  | 0.548 |
| walker |  | 8368 | 15 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 32, sub: 0, line: 449 } |  |  | 0.548 |
| walker |  | 8386 | 18 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 30, sub: 0, line: 438 } |  |  | 0.548 |
| walker |  | 8431 | 45 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 38, sub: 0, line: 543 } |  |  | 0.548 |
| walker |  | 8445 | 14 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 33, sub: 0, line: 453 } |  |  | 0.548 |
| walker |  | 8466 | 21 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 39, sub: 0, line: 577 } |  |  | 0.549 |
| ns | 8540 |  | 281 | The eggsample host program: wiring a PluginManager and calling a hook | 4.3 |  | 0.539 |
| walker |  | 8543 | 77 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 36, sub: 0, line: 499 } |  |  | 0.542 |
| walker |  | 8622 | 79 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 21, sub: 0, line: 293 } |  |  | 0.542 |
| walker |  | 8702 | 80 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 14, sub: 0, line: 164 } |  |  | 0.542 |
| ns | 8760 |  | 220 | `eggsample/hookspecs.py` in full: what a real hookspec module looks like | 4.4 |  | 0.533 |
| walker |  | 8783 | 81 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 9, sub: 0, line: 77 } |  |  | 0.533 |
| walker |  | 8881 | 98 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 37, sub: 0, line: 516 } |  |  | 0.533 |
| ns | 9096 |  | 336 | Both sides of hook implementation: the host's own `lib.py` and the external plugin `eggsample_spam.py` | 4.5 |  | 0.521 |
| walker |  | 9221 | 340 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 41, sub: 0, line: 593 } |  |  | 0.521 |
| walker |  | 9229 | 8 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 44, sub: 0, line: 626 } |  |  | 0.521 |
| walker |  | 9237 | 8 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 45, sub: 0, line: 630 } |  |  | 0.521 |
| ns | 9239 |  | 143 | Entry-point wiring in both example `setup.py` files | 4.6 |  | 0.518 |
| walker |  | 9252 | 15 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 43, sub: 0, line: 618 } |  |  | 0.518 |
| walker |  | 9282 | 30 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 41, sub: 0, line: 593 } |  |  | 0.523 |
| walker |  | 9385 | 103 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 30, sub: 0, line: 395 } |  |  | 0.527 |
| ns | 9487 |  | 248 | `testing/conftest.py` in full: the two fixtures every test in the suite uses | 5.1 |  | 0.518 |
| walker |  | 9493 | 108 | Plaintext::DeclSurface { file: downstream/devpi.sh } |  |  | 0.518 |
| walker |  | 9504 | 11 | Fs::DirListing { dir: docs/examples/eggsample-spam } |  |  | 0.522 |
| ns | 9646 |  | 159 | `pyproject.toml`: package identity, Python floor, dependency groups and the src layout | 5.2 |  | 0.519 |
| ns | 9825 |  | 179 | `tox.ini`: the environment list and the embedded pytest configuration | 5.3 |  | 0.514 |
| ns | 9915 |  | 90 | `[tool.towncrier]` config: how CHANGELOG.rst is produced | 5.4 |  | 0.512 |
| ns | 9948 |  | 33 | Listings of `changelog/` and `scripts/` | 5.5 |  | 0.514 |

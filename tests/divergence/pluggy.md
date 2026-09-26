Score(3000)=0.640 I=0.785 C=0.521 ns_rows≤3K=16/43 grid(1000/1442/2080/3000/4327/6240/9000)=0.825/0.732/0.621/0.640/0.605/0.583/0.541

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
| walker |  | 1035 | 171 | Markdown::Section { file: README.rst, section_index: 4, keeps_default_concavity: false } |  |  | 0.825 |
| walker |  | 1277 | 242 | Markdown::Section { file: README.rst, section_index: 1, keeps_default_concavity: false } |  |  | 0.825 |
| ns | 1296 |  | 311 | Every method name on `PluginManager` (complete roster, names only) | 2.1 |  | 0.732 |
| walker |  | 1374 | 97 | Plaintext::DeclSurface { file: tox.ini } |  |  | 0.732 |
| walker |  | 1421 | 47 | Plaintext::DeclSurface { file: docs/requirements.txt } |  |  | 0.732 |
| ns | 1528 |  | 232 | The rest of `_manager.py`: PluginValidationError, DistFacade, and the two module helpers | 2.2 |  | 0.680 |
| walker |  | 1715 | 294 | Markdown::Section { file: README.rst, section_index: 2, keeps_default_concavity: false } |  |  | 0.680 |
| walker |  | 1835 | 120 | Code::CodeKey { rung: Names, file: src/pluggy/_manager.py, decl: 0, sub: 0, line: 0 } |  |  | 0.684 |
| ns | 1857 |  | 329 | Every top-level symbol in `_hooks.py` (complete roster) plus the two backward-compat aliases | 2.3 |  | 0.632 |
| walker |  | 1860 | 25 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 4, sub: 0, line: 52 } |  |  | 0.635 |
| walker |  | 1886 | 26 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 2, sub: 0, line: 37 } |  |  | 0.635 |
| walker |  | 1979 | 93 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 6, sub: 0, line: 65 } |  |  | 0.644 |
| ns | 2019 |  | 162 | Every method name on `HookCaller` (complete roster, names only) | 2.4 |  | 0.619 |
| walker |  | 2149 | 170 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 11, sub: 0, line: 83 } |  |  | 0.627 |
| walker |  | 2171 | 22 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 16, sub: 0, line: 201 } |  |  | 0.627 |
| walker |  | 2220 | 49 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 13, sub: 0, line: 114 } |  |  | 0.627 |
| ns | 2301 |  | 282 | Complete `Result` API with signatures (`_result.py`) | 2.5 |  | 0.593 |
| walker |  | 2410 | 190 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 11, sub: 1, line: 83 } |  |  | 0.620 |
| walker |  | 2426 | 16 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 21, sub: 0, line: 278 } |  |  | 0.620 |
| ns | 2563 |  | 262 | Complete top-level roster of `_callers.py` (the call loop module) | 2.6 |  | 0.590 |
| walker |  | 2644 | 218 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 11, sub: 2, line: 83 } |  |  | 0.641 |
| walker |  | 2660 | 16 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 36, sub: 0, line: 512 } |  |  | 0.641 |
| walker |  | 2677 | 17 | Code::CodeKey { rung: Decl, file: src/pluggy/_manager.py, decl: 34, sub: 0, line: 450 } |  |  | 0.641 |
| walker |  | 2688 | 11 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 6, sub: 0, line: 65 } |  |  | 0.645 |
| walker |  | 2702 | 14 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 23, sub: 0, line: 300 } |  |  | 0.645 |
| walker |  | 2717 | 15 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 18, sub: 0, line: 238 } |  |  | 0.645 |
| walker |  | 2732 | 15 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 22, sub: 0, line: 296 } |  |  | 0.645 |
| walker |  | 2749 | 17 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 26, sub: 0, line: 319 } |  |  | 0.645 |
| walker |  | 2767 | 18 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 17, sub: 0, line: 233 } |  |  | 0.645 |
| walker |  | 2785 | 18 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 25, sub: 0, line: 315 } |  |  | 0.645 |
| walker |  | 2806 | 21 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 32, sub: 0, line: 430 } |  |  | 0.645 |
| ns | 2887 |  | 324 | Complete symbol rosters for the two remaining modules: `_tracing.py` and `_warnings.py` | 2.7 |  | 0.618 |
| walker |  | 2932 | 126 | Code::CodeKey { rung: Names, file: src/pluggy/_callers.py, decl: 0, sub: 0, line: 0 } |  |  | 0.626 |
| walker |  | 2951 | 19 | Code::CodeKey { rung: Decl, file: src/pluggy/_callers.py, decl: 4, sub: 0, line: 70 } |  |  | 0.629 |
| walker |  | 2971 | 20 | Code::CodeKey { rung: Decl, file: src/pluggy/_callers.py, decl: 2, sub: 0, line: 27 } |  |  | 0.633 |
| walker |  | 2994 | 23 | Code::CodeKey { rung: Decl, file: src/pluggy/_callers.py, decl: 3, sub: 0, line: 60 } |  |  | 0.640 |
| walker |  | 3039 | 45 | Code::CodeKey { rung: Decl, file: src/pluggy/_callers.py, decl: 5, sub: 0, line: 82 } |  |  | 0.654 |
| ns | 3298 |  | 411 | Complete section map of the 1082-line manual `docs/index.rst` | 2.8 |  | 0.599 |
| walker |  | 3391 | 352 | Code::CodeKey { rung: Names, file: src/pluggy/_hooks.py, decl: 0, sub: 0, line: 0 } |  |  | 0.613 |
| walker |  | 3415 | 24 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 22, sub: 0, line: 377 } |  |  | 0.613 |
| walker |  | 3449 | 34 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 5, sub: 0, line: 33 } |  |  | 0.613 |
| walker |  | 3529 | 80 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 18, sub: 0, line: 358 } |  |  | 0.613 |
| ns | 3543 |  | 245 | The real call signatures of `@hookspec` and `@hookimpl` — every accepted option with its default | 3.1 |  | 0.590 |
| walker |  | 3614 | 85 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 9, sub: 0, line: 77 } |  |  | 0.590 |
| walker |  | 3691 | 77 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 11, sub: 0, line: 111 } |  |  | 0.599 |
| walker |  | 3776 | 85 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 12, sub: 0, line: 164 } |  |  | 0.603 |
| ns | 3791 |  | 248 | `HookimplOpts` in full: every hook-implementation option and what it means | 3.2 | 2.3 | 0.583 |
| walker |  | 3868 | 92 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 14, sub: 0, line: 202 } |  |  | 0.609 |
| ns | 3975 |  | 184 | `HookspecOpts` in full: every hook-specification option | 3.3 | 2.3 | 0.594 |
| walker |  | 3997 | 129 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 47, sub: 0, line: 696 } |  |  | 0.596 |
| walker |  | 4159 | 162 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 44, sub: 0, line: 638 } |  |  | 0.599 |
| walker |  | 4211 | 52 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 45, sub: 0, line: 656 } |  |  | 0.599 |
| walker |  | 4375 | 164 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 7, sub: 0, line: 40 } |  |  | 0.619 |
| ns | 4407 |  | 432 | Hook implementation ordering: the `_hookimpls` layout comment and `_add_hookimpl` in full | 3.4 | 2.4 | 0.588 |
| walker |  | 4601 | 226 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 8, sub: 0, line: 56 } |  |  | 0.616 |
| walker |  | 4611 | 10 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 7, sub: 0, line: 40 } |  |  | 0.620 |
| ns | 4613 |  | 206 | `HookCaller.__call__` in full: what `pm.hook.myhook(...)` actually does | 3.5 | 2.4 | 0.605 |
| walker |  | 4621 | 10 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 8, sub: 0, line: 56 } |  |  | 0.609 |
| walker |  | 4631 | 10 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 19, sub: 0, line: 365 } |  |  | 0.609 |
| walker |  | 4969 | 338 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 23, sub: 0, line: 382 } |  |  | 0.634 |
| walker |  | 4989 | 20 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 35, sub: 0, line: 543 } |  |  | 0.634 |
| walker |  | 5019 | 30 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 26, sub: 0, line: 424 } |  |  | 0.634 |
| walker |  | 5060 | 41 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 34, sub: 0, line: 516 } |  |  | 0.634 |
| ns | 5078 |  | 465 | `_multicall` part 1: the setup / non-wrapper call loop | 3.6 |  | 0.600 |
| walker |  | 5119 | 59 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 24, sub: 0, line: 393 } |  |  | 0.600 |
| walker |  | 5131 | 12 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 24, sub: 0, line: 393 } |  |  | 0.600 |
| walker |  | 5143 | 12 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 45, sub: 0, line: 656 } |  |  | 0.600 |
| walker |  | 5157 | 14 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 25, sub: 0, line: 420 } |  |  | 0.600 |
| walker |  | 5171 | 14 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 26, sub: 0, line: 424 } |  |  | 0.600 |
| walker |  | 5185 | 14 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 30, sub: 0, line: 453 } |  |  | 0.600 |
| ns | 5521 |  | 443 | `_multicall` part 2: the teardown loop, exception routing and `firstresult` collapse | 3.7 | 3.6 | 0.568 |
| walker |  | 5556 | 371 | Code::CodeKey { rung: Decl, file: src/pluggy/_hooks.py, decl: 38, sub: 0, line: 593 } |  |  | 0.568 |
| walker |  | 5571 | 15 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 23, sub: 0, line: 382 } |  |  | 0.571 |
| walker |  | 5586 | 15 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 29, sub: 0, line: 449 } |  |  | 0.571 |
| walker |  | 5602 | 16 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 44, sub: 0, line: 638 } |  |  | 0.574 |
| walker |  | 5620 | 18 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 27, sub: 0, line: 438 } |  |  | 0.574 |
| walker |  | 5641 | 21 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 36, sub: 0, line: 577 } |  |  | 0.574 |
| walker |  | 5670 | 29 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 18, sub: 0, line: 358 } |  |  | 0.580 |
| ns | 5677 |  | 156 | `PluginManager.register` docstring: naming, blocking and the duplicate-registration contract | 3.8 | 2.1 | 0.571 |
| walker |  | 5700 | 30 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 38, sub: 0, line: 593 } |  |  | 0.578 |
| walker |  | 5785 | 85 | Code::CodeKey { rung: Names, file: src/pluggy/_result.py, decl: 0, sub: 0, line: 0 } |  |  | 0.579 |
| ns | 5973 |  | 296 | `PluginManager.register` body: how hook implementations are discovered and attached | 3.9 | 3.8 | 0.565 |
| walker |  | 5981 | 196 | Code::CodeKey { rung: Decl, file: src/pluggy/_result.py, decl: 4, sub: 0, line: 24 } |  |  | 0.581 |
| walker |  | 6010 | 29 | Code::CodeKey { rung: Decl, file: src/pluggy/_result.py, decl: 5, sub: 0, line: 31 } |  |  | 0.581 |
| walker |  | 6019 | 9 | Code::CodeKey { rung: Doc, file: src/pluggy/_result.py, decl: 3, sub: 0, line: 20 } |  |  | 0.583 |
| walker |  | 6031 | 12 | Code::CodeKey { rung: Doc, file: src/pluggy/_result.py, decl: 5, sub: 0, line: 31 } |  |  | 0.583 |
| walker |  | 6043 | 12 | Code::CodeKey { rung: Doc, file: src/pluggy/_result.py, decl: 6, sub: 0, line: 42 } |  |  | 0.583 |
| walker |  | 6055 | 12 | Code::CodeKey { rung: Doc, file: src/pluggy/_result.py, decl: 7, sub: 0, line: 51 } |  |  | 0.583 |
| walker |  | 6067 | 12 | Code::CodeKey { rung: Doc, file: src/pluggy/_result.py, decl: 8, sub: 0, line: 56 } |  |  | 0.583 |
| walker |  | 6112 | 45 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 35, sub: 0, line: 543 } |  |  | 0.583 |
| walker |  | 6120 | 8 | Code::CodeKey { rung: Body, file: src/pluggy/_result.py, decl: 7, sub: 0, line: 51 } |  |  | 0.583 |
| walker |  | 6149 | 29 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 27, sub: 0, line: 323 } |  |  | 0.583 |
| walker |  | 6179 | 30 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 31, sub: 0, line: 425 } |  |  | 0.583 |
| walker |  | 6246 | 67 | Code::CodeKey { rung: Names, file: src/pluggy/_tracing.py, decl: 0, sub: 0, line: 0 } |  |  | 0.583 |
| walker |  | 6313 | 67 | Code::CodeKey { rung: Decl, file: src/pluggy/_tracing.py, decl: 10, sub: 0, line: 59 } |  |  | 0.585 |
| ns | 6431 |  | 458 | `PluginManager._verify_hook`: every validation error message pluggy can raise | 3.10 | 2.1 | 0.564 |
| walker |  | 6456 | 143 | Code::CodeKey { rung: Decl, file: src/pluggy/_tracing.py, decl: 3, sub: 0, line: 16 } |  |  | 0.573 |
| walker |  | 6489 | 33 | Code::CodeKey { rung: Names, file: src/pluggy/_warnings.py, decl: 0, sub: 0, line: 0 } |  |  | 0.577 |
| walker |  | 6504 | 15 | Code::CodeKey { rung: Decl, file: src/pluggy/_warnings.py, decl: 1, sub: 0, line: 4 } |  |  | 0.577 |
| walker |  | 6522 | 18 | Code::CodeKey { rung: Decl, file: src/pluggy/_warnings.py, decl: 2, sub: 0, line: 10 } |  |  | 0.579 |
| walker |  | 6536 | 14 | Code::CodeKey { rung: Doc, file: src/pluggy/_warnings.py, decl: 1, sub: 0, line: 4 } |  |  | 0.581 |
| walker |  | 6545 | 9 | Code::CodeKey { rung: Body, file: src/pluggy/_tracing.py, decl: 8, sub: 0, line: 48 } |  |  | 0.581 |
| ns | 6627 |  | 196 | `PluginManager.__init__`: the complete state of a plugin manager | 3.11 | 2.1 | 0.574 |
| walker |  | 6629 | 84 | Plaintext::DeclSurface { file: downstream/tox.sh } |  |  | 0.574 |
| walker |  | 6658 | 29 | Plaintext::Whole { file: downstream/tox.sh } |  |  | 0.574 |
| walker |  | 6745 | 87 | Plaintext::DeclSurface { file: downstream/conda.sh } |  |  | 0.574 |
| walker |  | 6787 | 42 | Plaintext::Whole { file: downstream/conda.sh } |  |  | 0.574 |
| walker |  | 6819 | 32 | Code::CodeKey { rung: Doc, file: src/pluggy/_result.py, decl: 4, sub: 0, line: 24 } |  |  | 0.579 |
| walker |  | 6896 | 77 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 33, sub: 0, line: 499 } |  |  | 0.584 |
| walker |  | 6987 | 91 | Plaintext::DeclSurface { file: downstream/pytest.sh } |  |  | 0.584 |
| ns | 6993 |  | 366 | Historic hooks end to end: `set_specification`, `call_historic`, `_maybe_apply_history` | 3.12 | 2.4 | 0.569 |
| walker |  | 7016 | 29 | Plaintext::Whole { file: downstream/pytest.sh } |  |  | 0.569 |
| walker |  | 7095 | 79 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 17, sub: 0, line: 293 } |  |  | 0.569 |
| walker |  | 7175 | 80 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 12, sub: 0, line: 164 } |  |  | 0.569 |
| ns | 7193 |  | 200 | Complete attribute sets of `HookImpl` and `HookSpec` (`__slots__`) | 3.13 |  | 0.582 |
| walker |  | 7268 | 93 | Plaintext::DeclSurface { file: downstream/datasette.sh } |  |  | 0.582 |
| walker |  | 7298 | 30 | Plaintext::Whole { file: downstream/datasette.sh } |  |  | 0.582 |
| walker |  | 7379 | 81 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 9, sub: 0, line: 77 } |  |  | 0.582 |
| walker |  | 7474 | 95 | Plaintext::DeclSurface { file: downstream/hatch.sh } |  |  | 0.582 |
| walker |  | 7503 | 29 | Plaintext::Whole { file: downstream/hatch.sh } |  |  | 0.567 |
| ns | 7503 |  | 310 | `load_setuptools_entrypoints`: how third-party plugins are discovered | 3.14 | 2.1 | 0.567 |
| walker |  | 7538 | 35 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 19, sub: 0, line: 242 } |  |  | 0.567 |
| walker |  | 7646 | 108 | Plaintext::DeclSurface { file: downstream/devpi.sh } |  |  | 0.567 |
| walker |  | 7694 | 48 | Plaintext::Whole { file: downstream/devpi.sh } |  |  | 0.567 |
| ns | 7697 |  | 194 | Blocking semantics: `set_blocked`, `is_blocked`, `unblock` | 3.15 | 2.1 | 0.563 |
| ns | 7929 |  | 232 | `docs/api_reference.rst`: exactly which types are publicly documented | 4.1 |  | 0.556 |
| walker |  | 8235 | 541 | Markdown::Section { file: README.rst, section_index: 3, keeps_default_concavity: false } |  |  | 0.556 |
| walker |  | 8247 | 12 | Code::CodeKey { rung: Body, file: src/pluggy/_tracing.py, decl: 5, sub: 0, line: 22 } |  |  | 0.556 |
| ns | 8259 |  | 330 | `docs/examples/toy-example.py`: the canonical end-to-end usage | 4.2 |  | 0.541 |
| walker |  | 8345 | 98 | Code::CodeKey { rung: Doc, file: src/pluggy/_hooks.py, decl: 34, sub: 0, line: 516 } |  |  | 0.542 |
| walker |  | 8387 | 42 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 35, sub: 0, line: 487 } |  |  | 0.542 |
| walker |  | 8431 | 44 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 29, sub: 0, line: 379 } |  |  | 0.542 |
| walker |  | 8477 | 46 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 4, sub: 0, line: 52 } |  |  | 0.549 |
| walker |  | 8490 | 13 | Code::CodeKey { rung: Body, file: src/pluggy/_tracing.py, decl: 12, sub: 0, line: 64 } |  |  | 0.549 |
| walker |  | 8540 | 50 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 36, sub: 0, line: 512 } |  |  | 0.538 |
| ns | 8540 |  | 281 | The eggsample host program: wiring a PluginManager and calling a hook | 4.3 |  | 0.538 |
| walker |  | 8678 | 138 | Plaintext::DeclSurface { file: downstream/python-lsp-server.sh } |  |  | 0.538 |
| walker |  | 8709 | 31 | Code::CodeKey { rung: Doc, file: src/pluggy/_callers.py, decl: 2, sub: 0, line: 27 } |  |  | 0.538 |
| ns | 8760 |  | 220 | `eggsample/hookspecs.py` in full: what a real hookspec module looks like | 4.4 |  | 0.530 |
| walker |  | 8775 | 66 | Code::CodeKey { rung: Body, file: src/pluggy/__init__.py, decl: 1, sub: 0, line: 32 } |  |  | 0.541 |
| walker |  | 8836 | 61 | Code::CodeKey { rung: Doc, file: src/pluggy/_result.py, decl: 10, sub: 0, line: 80 } |  |  | 0.541 |
| walker |  | 8853 | 17 | Code::CodeKey { rung: Body, file: src/pluggy/_tracing.py, decl: 13, sub: 0, line: 67 } |  |  | 0.541 |
| walker |  | 8916 | 63 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 20, sub: 0, line: 252 } |  |  | 0.541 |
| walker |  | 8979 | 63 | Code::CodeKey { rung: Doc, file: src/pluggy/_result.py, decl: 11, sub: 0, line: 91 } |  |  | 0.541 |
| walker |  | 9044 | 65 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 33, sub: 0, line: 434 } |  |  | 0.541 |
| walker |  | 9062 | 18 | Code::CodeKey { rung: Body, file: src/pluggy/_tracing.py, decl: 11, sub: 0, line: 60 } |  |  | 0.541 |
| ns | 9096 |  | 336 | Both sides of hook implementation: the host's own `lib.py` and the external plugin `eggsample_spam.py` | 4.5 |  | 0.529 |
| ns | 9239 |  | 143 | Entry-point wiring in both example `setup.py` files | 4.6 |  | 0.525 |
| walker |  | 9267 | 205 | Plaintext::Whole { file: downstream/python-lsp-server.sh } |  |  | 0.525 |
| ns | 9487 |  | 248 | `testing/conftest.py` in full: the two fixtures every test in the suite uses | 5.1 |  | 0.516 |
| ns | 9646 |  | 159 | `pyproject.toml`: package identity, Python floor, dependency groups and the src layout | 5.2 |  | 0.512 |
| walker |  | 9825 | 558 | Plaintext::Whole { file: tox.ini } |  |  | 0.522 |
| ns | 9825 |  | 179 | `tox.ini`: the environment list and the embedded pytest configuration | 5.3 |  | 0.522 |
| walker |  | 9912 | 87 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 16, sub: 0, line: 201 } |  |  | 0.522 |
| ns | 9915 |  | 90 | `[tool.towncrier]` config: how CHANGELOG.rst is produced | 5.4 |  | 0.519 |
| ns | 9948 |  | 33 | Listings of `changelog/` and `scripts/` | 5.5 |  | 0.522 |
| walker |  | 9968 | 56 | Code::CodeKey { rung: Doc, file: src/pluggy/_callers.py, decl: 5, sub: 0, line: 82 } |  |  | 0.522 |
| walker |  | 9983 | 15 | Code::CodeKey { rung: Doc, file: src/pluggy/_manager.py, decl: 24, sub: 0, line: 304 } |  |  | 0.522 |

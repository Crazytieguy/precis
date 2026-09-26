Score(3000)=0.742 I=0.917 C=0.601 ns_rows≤3K=19/49 grid(1000/1442/2080/3000/4327/6240/9000)=0.754/0.724/0.867/0.742/0.628/0.568/0.585

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 33 | 33 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 72 |  | 72 | README identity: name, one-line description, pitch | 1.1 |  | 0.000 |
| ns | 105 |  | 33 | Repository root listing (complete) | 1.2 |  | 0.472 |
| walker |  | 182 | 149 | Markdown::ReadmeHeadline { file: README.md } |  |  | 1.000 |
| walker |  | 202 | 20 | Fs::DirListing { dir: docs } |  |  | 1.000 |
| ns | 208 |  | 103 | The `htmy/` package and its two subpackages (complete) | 1.3 |  | 0.571 |
| walker |  | 275 | 73 | Toml::Identity { file: pyproject.toml } |  |  | 0.572 |
| walker |  | 282 | 7 | Fs::DirListing { dir: .github } |  |  | 0.572 |
| walker |  | 301 | 19 | Fs::DirListing { dir: .github/workflows } |  |  | 0.573 |
| ns | 349 |  | 141 | README key features, first half | 1.4 |  | 0.511 |
| walker |  | 368 | 67 | Fs::DirListing { dir: htmy } |  |  | 0.687 |
| walker |  | 382 | 14 | Fs::DirListing { dir: htmy/md } |  |  | 0.756 |
| walker |  | 399 | 17 | Code::CodeKey { rung: ModuleDoc, file: htmy/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.757 |
| walker |  | 421 | 22 | Fs::DirListing { dir: htmy/renderer } |  |  | 0.894 |
| walker |  | 463 | 42 | Fs::DirListing { dir: docs/api } |  |  | 0.894 |
| walker |  | 475 | 12 | Fs::DirListing { dir: docs/api/renderer } |  |  | 0.894 |
| walker |  | 497 | 22 | Fs::DirListing { dir: examples } |  |  | 0.894 |
| ns | 504 |  | 155 | README key features, second half | 1.5 | 1.4 | 0.812 |
| walker |  | 510 | 13 | Fs::DirListing { dir: examples/internationalization } |  |  | 0.812 |
| walker |  | 524 | 14 | Fs::DirListing { dir: examples/markdown_customization } |  |  | 0.812 |
| walker |  | 538 | 14 | Fs::DirListing { dir: examples/markdown_essentials } |  |  | 0.812 |
| walker |  | 631 | 93 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.817 |
| walker |  | 647 | 16 | Fs::DirListing { dir: examples/snippet-slots-fastapi } |  |  | 0.817 |
| ns | 648 |  | 144 | `htmy/__init__.py` exports, part 1: version + everything from `core` | 1.6 |  | 0.732 |
| ns | 786 |  | 138 | `htmy/__init__.py` exports, part 2: `ErrorBoundary`, `component`, renderers, `Snippet`/`Slots`, tags | 1.7 | 1.6 | 0.672 |
| walker |  | 859 | 212 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.673 |
| ns | 1014 |  | 228 | `htmy/__init__.py` exports, part 3: the sixteen re-exported type names | 1.8 | 1.7 | 0.600 |
| walker |  | 1145 | 286 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.740 |
| ns | 1160 |  | 146 | `htmy/__init__.py` exports, part 4: `utils` helpers and the two aliases | 1.9 | 1.8 | 0.687 |
| walker |  | 1191 | 46 | Markdown::Section { file: README.md, section_index: 16, keeps_default_concavity: false } |  |  | 0.687 |
| walker |  | 1250 | 59 | Fs::DirListing { dir: tests } |  |  | 0.692 |
| walker |  | 1265 | 15 | Fs::DirListing { dir: tests/data } |  |  | 0.692 |
| walker |  | 1293 | 28 | Fs::DirListing { dir: tests/renderer } |  |  | 0.694 |
| ns | 1424 |  | 264 | README "Built-in components": one line of semantics per built-in | 2.1 |  | 0.654 |
| walker |  | 1485 | 192 | Code::CodeKey { rung: Names, file: htmy/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.742 |
| ns | 1612 |  | 188 | README: the definition of a component (duck-typed `htmy()` method) | 2.2 |  | 0.724 |
| walker |  | 1677 | 192 | Code::CodeKey { rung: Names, file: htmy/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.797 |
| ns | 1758 |  | 146 | README "Rendering": how to actually invoke the renderer | 2.3 |  | 0.771 |
| ns | 1900 |  | 142 | README "Context": the prop-drilling escape hatch and `htmy_context()` | 2.4 |  | 0.754 |
| walker |  | 1917 | 240 | Code::CodeKey { rung: Names, file: htmy/__init__.py, decl: 0, sub: 2, line: 0 } |  |  | 0.867 |
| walker |  | 2005 | 88 | Code::CodeKey { rung: Names, file: htmy/md/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.867 |
| walker |  | 2108 | 103 | Code::CodeKey { rung: Names, file: htmy/renderer/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.868 |
| walker |  | 2142 | 34 | Fs::DirListing { dir: docs/examples } |  |  | 0.842 |
| ns | 2142 |  | 242 | README "Formatter": the default attribute name/value conversion rules | 2.5 |  | 0.842 |
| walker |  | 2199 | 57 | Markdown::Section { file: README.md, section_index: 20, keeps_default_concavity: false } |  |  | 0.842 |
| walker |  | 2269 | 70 | Markdown::Section { file: README.md, section_index: 19, keeps_default_concavity: false } |  |  | 0.842 |
| ns | 2302 |  | 160 | README "XSS prevention": escaping by default, and the two exceptions | 2.6 |  | 0.815 |
| ns | 2506 |  | 204 | `pyproject.toml` project block: runtime deps, Python floor, optional `lxml` | 3.1 |  | 0.799 |
| walker |  | 2605 | 336 | Code::CodeKey { rung: Names, file: htmy/typing.py, decl: 0, sub: 0, line: 0 } |  |  | 0.802 |
| walker |  | 2621 | 16 | Code::CodeKey { rung: Decl, file: htmy/typing.py, decl: 18, sub: 0, line: 73 } |  |  | 0.802 |
| walker |  | 2638 | 17 | Code::CodeKey { rung: Decl, file: htmy/typing.py, decl: 20, sub: 0, line: 81 } |  |  | 0.802 |
| walker |  | 2657 | 19 | Code::CodeKey { rung: Decl, file: htmy/typing.py, decl: 24, sub: 0, line: 103 } |  |  | 0.802 |
| ns | 2660 |  | 154 | `[tool.poe.tasks]`: the project's canonical commands | 3.2 |  | 0.762 |
| walker |  | 2679 | 22 | Code::CodeKey { rung: Decl, file: htmy/typing.py, decl: 9, sub: 0, line: 37 } |  |  | 0.762 |
| walker |  | 2702 | 23 | Code::CodeKey { rung: Decl, file: htmy/typing.py, decl: 11, sub: 0, line: 45 } |  |  | 0.763 |
| walker |  | 2713 | 11 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 10, sub: 0, line: 40 } |  |  | 0.763 |
| walker |  | 2724 | 11 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 12, sub: 0, line: 48 } |  |  | 0.763 |
| walker |  | 2735 | 11 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 18, sub: 0, line: 73 } |  |  | 0.763 |
| walker |  | 2746 | 11 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 20, sub: 0, line: 81 } |  |  | 0.764 |
| walker |  | 2750 | 4 | Code::CodeKey { rung: Body, file: htmy/typing.py, decl: 10, sub: 0, line: 40 } |  |  | 0.764 |
| walker |  | 2754 | 4 | Code::CodeKey { rung: Body, file: htmy/typing.py, decl: 12, sub: 0, line: 48 } |  |  | 0.764 |
| ns | 2766 |  | 106 | Test suite and CI workflow listings (complete) | 3.3 |  | 0.779 |
| walker |  | 2767 | 13 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 19, sub: 0, line: 76 } |  |  | 0.779 |
| walker |  | 2771 | 4 | Code::CodeKey { rung: Body, file: htmy/typing.py, decl: 19, sub: 0, line: 76 } |  |  | 0.780 |
| walker |  | 2784 | 13 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 21, sub: 0, line: 84 } |  |  | 0.780 |
| walker |  | 2788 | 4 | Code::CodeKey { rung: Body, file: htmy/typing.py, decl: 21, sub: 0, line: 84 } |  |  | 0.780 |
| walker |  | 2802 | 14 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 9, sub: 0, line: 37 } |  |  | 0.780 |
| walker |  | 2816 | 14 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 11, sub: 0, line: 45 } |  |  | 0.780 |
| walker |  | 2920 | 104 | Code::CodeKey { rung: Names, file: htmy/core.py, decl: 0, sub: 0, line: 0 } |  |  | 0.781 |
| ns | 2922 |  | 156 | `tests/conftest.py` in full: the three session-scoped renderer fixtures | 3.4 |  | 0.742 |
| walker |  | 2926 | 6 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 14, sub: 0, line: 157 } |  |  | 0.742 |
| walker |  | 2932 | 6 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 15, sub: 0, line: 163 } |  |  | 0.742 |
| walker |  | 2969 | 37 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 12, sub: 0, line: 146 } |  |  | 0.742 |
| walker |  | 3006 | 37 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 16, sub: 0, line: 175 } |  |  | 0.742 |
| walker |  | 3066 | 60 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 1, sub: 0, line: 19 } |  |  | 0.743 |
| ns | 3090 |  | 168 | `pyproject.toml` build backend, version source and type/test settings | 3.5 |  | 0.707 |
| walker |  | 3127 | 61 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 4, sub: 0, line: 38 } |  |  | 0.708 |
| walker |  | 3260 | 133 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 7, sub: 0, line: 64 } |  |  | 0.708 |
| ns | 3307 |  | 217 | `htmy/typing.py` lines 4-27: `T`/`U`, property types, the context types | 4.1 |  | 0.682 |
| walker |  | 3443 | 183 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 19, sub: 0, line: 201 } |  |  | 0.683 |
| walker |  | 3490 | 47 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 20, sub: 0, line: 222 } |  |  | 0.683 |
| walker |  | 3501 | 11 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 3, sub: 0, line: 33 } |  |  | 0.683 |
| walker |  | 3513 | 12 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 25, sub: 0, line: 281 } |  |  | 0.683 |
| walker |  | 3526 | 13 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 6, sub: 0, line: 56 } |  |  | 0.683 |
| walker |  | 3540 | 14 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 1, sub: 0, line: 19 } |  |  | 0.683 |
| walker |  | 3555 | 15 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 14, sub: 0, line: 157 } |  |  | 0.684 |
| walker |  | 3570 | 15 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 26, sub: 0, line: 286 } |  |  | 0.684 |
| ns | 3621 |  | 314 | `htmy/typing.py` lines 34-67: the component protocols and the `Component` union | 4.2 | 4.1 | 0.665 |
| walker |  | 3657 | 87 | Code::CodeKey { rung: Names, file: htmy/tag.py, decl: 0, sub: 0, line: 0 } |  |  | 0.665 |
| walker |  | 3703 | 46 | Code::CodeKey { rung: Decl, file: htmy/tag.py, decl: 14, sub: 0, line: 84 } |  |  | 0.665 |
| walker |  | 3764 | 61 | Code::CodeKey { rung: Decl, file: htmy/tag.py, decl: 8, sub: 0, line: 56 } |  |  | 0.665 |
| walker |  | 3826 | 62 | Code::CodeKey { rung: Decl, file: htmy/tag.py, decl: 1, sub: 0, line: 12 } |  |  | 0.665 |
| walker |  | 3892 | 66 | Code::CodeKey { rung: Decl, file: htmy/tag.py, decl: 4, sub: 0, line: 24 } |  |  | 0.665 |
| ns | 3922 |  | 301 | `htmy/typing.py` lines 70-109: context providers, `TextProcessor`, `TextResolver` | 4.3 | 4.1 | 0.649 |
| walker |  | 3936 | 44 | Code::CodeKey { rung: Decl, file: htmy/tag.py, decl: 5, sub: 0, line: 27 } |  |  | 0.649 |
| walker |  | 4013 | 77 | Code::CodeKey { rung: Decl, file: htmy/tag.py, decl: 11, sub: 0, line: 73 } |  |  | 0.649 |
| walker |  | 4029 | 16 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 21, sub: 0, line: 241 } |  |  | 0.649 |
| walker |  | 4046 | 17 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 12, sub: 0, line: 146 } |  |  | 0.650 |
| walker |  | 4052 | 6 | Code::CodeKey { rung: Body, file: htmy/typing.py, decl: 25, sub: 0, line: 108 } |  |  | 0.650 |
| ns | 4057 |  | 135 | `htmy/renderer/__init__.py` in full: which class each renderer name resolves to | 5.1 |  | 0.647 |
| walker |  | 4074 | 22 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 13, sub: 0, line: 151 } |  |  | 0.647 |
| walker |  | 4082 | 8 | Code::CodeKey { rung: Body, file: htmy/tag.py, decl: 9, sub: 0, line: 66 } |  |  | 0.647 |
| walker |  | 4105 | 23 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 4, sub: 0, line: 38 } |  |  | 0.648 |
| walker |  | 4206 | 101 | Markdown::Section { file: README.md, section_index: 13, keeps_default_concavity: false } |  |  | 0.648 |
| ns | 4299 |  | 242 | `htmy/renderer/typing.py`: all four symbols of the renderer protocol module | 5.2 |  | 0.628 |
| walker |  | 4362 | 156 | Code::CodeKey { rung: Names, file: htmy/utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.629 |
| walker |  | 4391 | 29 | Code::CodeKey { rung: Decl, file: htmy/utils.py, decl: 1, sub: 0, line: 12 } |  |  | 0.629 |
| walker |  | 4408 | 17 | Code::CodeKey { rung: Doc, file: htmy/utils.py, decl: 6, sub: 0, line: 67 } |  |  | 0.629 |
| walker |  | 4426 | 18 | Code::CodeKey { rung: Doc, file: htmy/utils.py, decl: 4, sub: 0, line: 55 } |  |  | 0.629 |
| walker |  | 4444 | 18 | Code::CodeKey { rung: Doc, file: htmy/utils.py, decl: 5, sub: 0, line: 62 } |  |  | 0.629 |
| walker |  | 4465 | 21 | Code::CodeKey { rung: Doc, file: htmy/utils.py, decl: 7, sub: 0, line: 73 } |  |  | 0.629 |
| walker |  | 4477 | 12 | Code::CodeKey { rung: Body, file: htmy/utils.py, decl: 5, sub: 0, line: 62 } |  |  | 0.629 |
| ns | 4480 |  | 181 | `Renderer` in `htmy/renderer/default.py`: strategy docstring and `__init__` | 5.3 |  | 0.613 |
| walker |  | 4574 | 97 | Markdown::Section { file: README.md, section_index: 15, keeps_default_concavity: false } |  |  | 0.613 |
| walker |  | 4602 | 28 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 24, sub: 0, line: 103 } |  |  | 0.621 |
| ns | 4667 |  | 187 | `Renderer.render()` in `default.py`: context layering and renderer self-registration | 5.4 | 5.3 | 0.611 |
| walker |  | 4738 | 136 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.611 |
| walker |  | 4767 | 29 | Code::CodeKey { rung: Doc, file: htmy/utils.py, decl: 3, sub: 0, line: 49 } |  |  | 0.611 |
| ns | 4905 |  | 238 | `Renderer` in `htmy/renderer/baseline.py`: why it exists, plus its full method roster | 5.5 |  | 0.593 |
| walker |  | 4968 | 201 | Code::CodeKey { rung: Names, file: htmy/html.py, decl: 0, sub: 0, line: 0 } |  |  | 0.594 |
| walker |  | 4992 | 24 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 1, sub: 0, line: 13 } |  |  | 0.594 |
| walker |  | 5018 | 26 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 8, sub: 0, line: 63 } |  |  | 0.594 |
| ns | 5045 |  | 140 | `htmy/renderer/context.py`: `RendererContext.from_context()` | 5.6 |  | 0.583 |
| walker |  | 5161 | 143 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 11, sub: 0, line: 79 } |  |  | 0.583 |
| walker |  | 5182 | 21 | Code::CodeKey { rung: Names, file: htmy/snippet.py, decl: 0, sub: 0, line: 0 } |  |  | 0.583 |
| ns | 5270 |  | 225 | `core.py`: `Fragment` and `WithContext` | 6.1 |  | 0.585 |
| walker |  | 5335 | 153 | Code::CodeKey { rung: Decl, file: htmy/snippet.py, decl: 5, sub: 0, line: 158 } |  |  | 0.586 |
| walker |  | 5392 | 57 | Code::CodeKey { rung: Decl, file: htmy/snippet.py, decl: 6, sub: 0, line: 218 } |  |  | 0.586 |
| walker |  | 5403 | 11 | Code::CodeKey { rung: Doc, file: htmy/snippet.py, decl: 7, sub: 0, line: 241 } |  |  | 0.586 |
| walker |  | 5411 | 8 | Code::CodeKey { rung: Doc, file: htmy/html.py, decl: 1, sub: 0, line: 13 } |  |  | 0.586 |
| walker |  | 5426 | 15 | Code::CodeKey { rung: Doc, file: htmy/snippet.py, decl: 10, sub: 0, line: 272 } |  |  | 0.586 |
| walker |  | 5442 | 16 | Code::CodeKey { rung: Doc, file: htmy/snippet.py, decl: 8, sub: 0, line: 256 } |  |  | 0.586 |
| walker |  | 5452 | 10 | Code::CodeKey { rung: Doc, file: htmy/html.py, decl: 8, sub: 0, line: 63 } |  |  | 0.586 |
| walker |  | 5462 | 10 | Code::CodeKey { rung: Doc, file: htmy/html.py, decl: 11, sub: 0, line: 79 } |  |  | 0.586 |
| ns | 5522 |  | 252 | `core.py`: `ContextAware`, the typed context registration base class | 6.2 |  | 0.578 |
| walker |  | 5695 | 233 | Code::CodeKey { rung: Decl, file: htmy/snippet.py, decl: 1, sub: 0, line: 27 } |  |  | 0.578 |
| ns | 5795 |  | 273 | `core.py`: `SkipProperty`, `Text`, `SafeStr`, `XBool`, `xml_format_string` | 6.3 |  | 0.572 |
| walker |  | 5978 | 283 | Code::CodeKey { rung: Decl, file: htmy/snippet.py, decl: 1, sub: 1, line: 27 } |  |  | 0.573 |
| walker |  | 6033 | 55 | Code::CodeKey { rung: Decl, file: htmy/snippet.py, decl: 2, sub: 0, line: 83 } |  |  | 0.573 |
| walker |  | 6046 | 13 | Code::CodeKey { rung: Names, file: htmy/etree.py, decl: 0, sub: 0, line: 0 } |  |  | 0.573 |
| ns | 6085 |  | 290 | `core.py`: `Formatter` -- construction and the complete method surface | 6.4 |  | 0.568 |
| ns | 6259 |  | 174 | `core.py`: the default value-formatter table | 6.5 | 6.4 | 0.560 |
| walker |  | 6279 | 233 | Code::CodeKey { rung: Decl, file: htmy/etree.py, decl: 1, sub: 0, line: 23 } |  |  | 0.560 |
| walker |  | 6294 | 15 | Code::CodeKey { rung: Doc, file: htmy/etree.py, decl: 7, sub: 0, line: 103 } |  |  | 0.560 |
| walker |  | 6311 | 17 | Code::CodeKey { rung: Doc, file: htmy/etree.py, decl: 4, sub: 0, line: 63 } |  |  | 0.560 |
| walker |  | 6342 | 31 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 18, sub: 0, line: 196 } |  |  | 0.564 |
| walker |  | 6368 | 26 | Code::CodeKey { rung: Names, file: htmy/md/core.py, decl: 0, sub: 0, line: 0 } |  |  | 0.564 |
| walker |  | 6451 | 83 | Code::CodeKey { rung: Decl, file: htmy/md/core.py, decl: 6, sub: 0, line: 81 } |  |  | 0.564 |
| ns | 6452 |  | 193 | `htmy/snippet.py`: `Slots` and the slot placeholder syntax | 7.1 |  | 0.568 |
| walker |  | 6541 | 90 | Code::CodeKey { rung: Decl, file: htmy/md/core.py, decl: 7, sub: 0, line: 103 } |  |  | 0.568 |
| walker |  | 6690 | 149 | Code::CodeKey { rung: Decl, file: htmy/md/core.py, decl: 1, sub: 0, line: 20 } |  |  | 0.569 |
| walker |  | 6723 | 33 | Code::CodeKey { rung: Doc, file: htmy/utils.py, decl: 2, sub: 0, line: 42 } |  |  | 0.569 |
| walker |  | 6737 | 14 | Code::CodeKey { rung: Body, file: htmy/utils.py, decl: 2, sub: 0, line: 42 } |  |  | 0.569 |
| ns | 6744 |  | 292 | `htmy/snippet.py`: `Snippet` and its four-step text pipeline | 7.2 |  | 0.562 |
| walker |  | 6965 | 228 | Code::CodeKey { rung: Names, file: htmy/html.py, decl: 0, sub: 1, line: 0 } |  |  | 0.563 |
| ns | 7031 |  | 287 | `htmy/function_component.py`: the complete `@component` decorator family | 7.3 |  | 0.552 |
| walker |  | 7194 | 229 | Code::CodeKey { rung: Names, file: htmy/html.py, decl: 0, sub: 2, line: 0 } |  |  | 0.553 |
| ns | 7205 |  | 174 | `htmy/md/core.py`: `MarkdownParser` | 7.4 |  | 0.554 |
| walker |  | 7399 | 205 | Code::CodeKey { rung: Names, file: htmy/html.py, decl: 0, sub: 3, line: 0 } |  |  | 0.554 |
| ns | 7514 |  | 309 | `htmy/md/core.py`: the `MD` component, plus the `md/typing.py` definitions | 7.5 |  | 0.546 |
| walker |  | 7601 | 202 | Code::CodeKey { rung: Names, file: htmy/html.py, decl: 0, sub: 4, line: 0 } |  |  | 0.547 |
| ns | 7773 |  | 259 | `htmy/i18n.py`: `I18n`, its error hierarchy, and the resource loader | 7.6 |  | 0.539 |
| walker |  | 7805 | 204 | Code::CodeKey { rung: Names, file: htmy/html.py, decl: 0, sub: 5, line: 0 } |  |  | 0.540 |
| ns | 8005 |  | 232 | `htmy/tag.py`: how every `html.*` tag is actually built | 7.7 |  | 0.544 |
| walker |  | 8015 | 210 | Code::CodeKey { rung: Names, file: htmy/html.py, decl: 0, sub: 6, line: 0 } |  |  | 0.544 |
| walker |  | 8162 | 147 | Code::CodeKey { rung: Names, file: htmy/html.py, decl: 0, sub: 7, line: 0 } |  |  | 0.545 |
| ns | 8206 |  | 201 | `htmy/error_boundary.py` in outline: `ErrorBoundary` | 7.8 |  | 0.538 |
| walker |  | 8359 | 197 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 120, sub: 0, line: 831 } |  |  | 0.538 |
| walker |  | 8380 | 21 | Code::CodeKey { rung: Doc, file: htmy/etree.py, decl: 3, sub: 0, line: 55 } |  |  | 0.538 |
| ns | 8453 |  | 247 | `htmy/etree.py`: `ETreeConverter` and the optional-`lxml` import switch | 7.9 |  | 0.538 |
| walker |  | 8527 | 147 | Markdown::Section { file: README.md, section_index: 12, keeps_default_concavity: false } |  |  | 0.550 |
| walker |  | 8564 | 37 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 17, sub: 0, line: 186 } |  |  | 0.554 |
| walker |  | 8575 | 11 | Code::CodeKey { rung: Body, file: htmy/etree.py, decl: 2, sub: 0, line: 46 } |  |  | 0.554 |
| walker |  | 8615 | 40 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 9, sub: 0, line: 108 } |  |  | 0.554 |
| walker |  | 8629 | 14 | Code::CodeKey { rung: Body, file: htmy/tag.py, decl: 10, sub: 0, line: 69 } |  |  | 0.557 |
| ns | 8676 |  | 223 | `htmy/utils.py` and `htmy/io.py`: every helper, by signature | 7.10 |  | 0.561 |
| walker |  | 8783 | 154 | Code::CodeKey { rung: Names, file: htmy/i18n.py, decl: 0, sub: 0, line: 0 } |  |  | 0.566 |
| ns | 8818 |  | 142 | `html.py` tag roster 1/5: document skeleton, `Link` and `Meta` factories | 8.1 |  | 0.573 |
| walker |  | 8963 | 180 | Code::CodeKey { rung: Decl, file: htmy/i18n.py, decl: 5, sub: 0, line: 24 } |  |  | 0.585 |
| walker |  | 8975 | 12 | Code::CodeKey { rung: Names, file: htmy/error_boundary.py, decl: 0, sub: 0, line: 0 } |  |  | 0.585 |
| ns | 9055 |  | 237 | `html.py` tag roster 2/5: sectioning and container tags | 8.2 | 8.1 | 0.597 |
| walker |  | 9062 | 87 | Code::CodeKey { rung: Decl, file: htmy/error_boundary.py, decl: 1, sub: 0, line: 15 } |  |  | 0.599 |
| walker |  | 9106 | 44 | Code::CodeKey { rung: Decl, file: htmy/error_boundary.py, decl: 2, sub: 0, line: 25 } |  |  | 0.599 |
| walker |  | 9128 | 22 | Code::CodeKey { rung: Doc, file: htmy/i18n.py, decl: 5, sub: 0, line: 24 } |  |  | 0.599 |
| walker |  | 9150 | 22 | Code::CodeKey { rung: Doc, file: htmy/md/core.py, decl: 2, sub: 0, line: 33 } |  |  | 0.599 |
| walker |  | 9193 | 43 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 2, sub: 0, line: 24 } |  |  | 0.600 |
| ns | 9220 |  | 165 | `html.py` tag roster 3/5: form and interactive tags | 8.3 | 8.1 | 0.608 |
| walker |  | 9237 | 44 | Code::CodeKey { rung: Doc, file: htmy/snippet.py, decl: 9, sub: 0, line: 265 } |  |  | 0.608 |
| walker |  | 9259 | 22 | Code::CodeKey { rung: Names, file: htmy/io.py, decl: 0, sub: 0, line: 0 } |  |  | 0.609 |
| walker |  | 9274 | 15 | Code::CodeKey { rung: Doc, file: htmy/io.py, decl: 1, sub: 0, line: 11 } |  |  | 0.609 |
| walker |  | 9498 | 224 | Code::CodeKey { rung: Names, file: htmy/function_component.py, decl: 0, sub: 0, line: 0 } |  |  | 0.610 |
| walker |  | 9514 | 16 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 4, sub: 0, line: 31 } |  |  | 0.610 |
| walker |  | 9532 | 18 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 7, sub: 0, line: 46 } |  |  | 0.610 |
| ns | 9544 |  | 324 | `html.py` tag roster 4/5: text-level tags and embedded media | 8.4 | 8.1 | 0.623 |
| walker |  | 9735 | 203 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 11, sub: 0, line: 71 } |  |  | 0.623 |
| walker |  | 9761 | 26 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 14, sub: 0, line: 194 } |  |  | 0.623 |
| walker |  | 9788 | 27 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 12, sub: 0, line: 86 } |  |  | 0.623 |
| walker |  | 9815 | 27 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 13, sub: 0, line: 147 } |  |  | 0.623 |
| walker |  | 9844 | 29 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 16, sub: 0, line: 315 } |  |  | 0.623 |
| walker |  | 9879 | 35 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 15, sub: 0, line: 244 } |  |  | 0.623 |
| ns | 9889 |  | 345 | `html.py` tag roster 5/5: lists, tables, headings, media, `entity` | 8.5 | 8.1 | 0.636 |
| walker |  | 9898 | 19 | Code::CodeKey { rung: Doc, file: htmy/function_component.py, decl: 11, sub: 0, line: 71 } |  |  | 0.636 |
| walker |  | 9937 | 39 | Code::CodeKey { rung: Doc, file: htmy/function_component.py, decl: 4, sub: 0, line: 31 } |  |  | 0.636 |
| walker |  | 9976 | 39 | Code::CodeKey { rung: Doc, file: htmy/function_component.py, decl: 7, sub: 0, line: 46 } |  |  | 0.636 |
| walker |  | 9990 | 14 | Code::CodeKey { rung: Names, file: htmy/renderer/typing.py, decl: 0, sub: 0, line: 0 } |  |  | 0.636 |

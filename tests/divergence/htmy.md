Score(3000)=0.741 I=0.914 C=0.601 ns_rows≤3K=19/49 grid(1000/1442/2080/3000/4327/6240/9000)=0.754/0.724/0.867/0.741/0.628/0.562/0.581

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
| walker |  | 2858 | 104 | Code::CodeKey { rung: Names, file: htmy/core.py, decl: 0, sub: 0, line: 0 } |  |  | 0.779 |
| walker |  | 2864 | 6 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 14, sub: 0, line: 157 } |  |  | 0.779 |
| walker |  | 2870 | 6 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 15, sub: 0, line: 163 } |  |  | 0.779 |
| walker |  | 2907 | 37 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 12, sub: 0, line: 146 } |  |  | 0.779 |
| ns | 2922 |  | 156 | `tests/conftest.py` in full: the three session-scoped renderer fixtures | 3.4 |  | 0.741 |
| walker |  | 2944 | 37 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 16, sub: 0, line: 175 } |  |  | 0.741 |
| walker |  | 3004 | 60 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 1, sub: 0, line: 19 } |  |  | 0.741 |
| walker |  | 3065 | 61 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 4, sub: 0, line: 38 } |  |  | 0.742 |
| ns | 3090 |  | 168 | `pyproject.toml` build backend, version source and type/test settings | 3.5 |  | 0.706 |
| walker |  | 3198 | 133 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 7, sub: 0, line: 64 } |  |  | 0.707 |
| ns | 3307 |  | 217 | `htmy/typing.py` lines 4-27: `T`/`U`, property types, the context types | 4.1 |  | 0.681 |
| walker |  | 3381 | 183 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 19, sub: 0, line: 201 } |  |  | 0.682 |
| walker |  | 3428 | 47 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 20, sub: 0, line: 222 } |  |  | 0.682 |
| walker |  | 3439 | 11 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 3, sub: 0, line: 33 } |  |  | 0.682 |
| walker |  | 3451 | 12 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 25, sub: 0, line: 281 } |  |  | 0.682 |
| walker |  | 3464 | 13 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 6, sub: 0, line: 56 } |  |  | 0.682 |
| walker |  | 3477 | 13 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 19, sub: 0, line: 76 } |  |  | 0.682 |
| walker |  | 3481 | 4 | Code::CodeKey { rung: Body, file: htmy/typing.py, decl: 19, sub: 0, line: 76 } |  |  | 0.682 |
| walker |  | 3494 | 13 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 21, sub: 0, line: 84 } |  |  | 0.683 |
| walker |  | 3498 | 4 | Code::CodeKey { rung: Body, file: htmy/typing.py, decl: 21, sub: 0, line: 84 } |  |  | 0.683 |
| walker |  | 3585 | 87 | Code::CodeKey { rung: Names, file: htmy/tag.py, decl: 0, sub: 0, line: 0 } |  |  | 0.683 |
| ns | 3621 |  | 314 | `htmy/typing.py` lines 34-67: the component protocols and the `Component` union | 4.2 | 4.1 | 0.661 |
| walker |  | 3631 | 46 | Code::CodeKey { rung: Decl, file: htmy/tag.py, decl: 14, sub: 0, line: 84 } |  |  | 0.661 |
| walker |  | 3692 | 61 | Code::CodeKey { rung: Decl, file: htmy/tag.py, decl: 8, sub: 0, line: 56 } |  |  | 0.661 |
| walker |  | 3754 | 62 | Code::CodeKey { rung: Decl, file: htmy/tag.py, decl: 1, sub: 0, line: 12 } |  |  | 0.661 |
| walker |  | 3820 | 66 | Code::CodeKey { rung: Decl, file: htmy/tag.py, decl: 4, sub: 0, line: 24 } |  |  | 0.661 |
| walker |  | 3864 | 44 | Code::CodeKey { rung: Decl, file: htmy/tag.py, decl: 5, sub: 0, line: 27 } |  |  | 0.661 |
| ns | 3922 |  | 301 | `htmy/typing.py` lines 70-109: context providers, `TextProcessor`, `TextResolver` | 4.3 | 4.1 | 0.645 |
| walker |  | 3941 | 77 | Code::CodeKey { rung: Decl, file: htmy/tag.py, decl: 11, sub: 0, line: 73 } |  |  | 0.646 |
| walker |  | 3955 | 14 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 1, sub: 0, line: 19 } |  |  | 0.646 |
| walker |  | 3969 | 14 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 9, sub: 0, line: 37 } |  |  | 0.649 |
| walker |  | 3983 | 14 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 11, sub: 0, line: 45 } |  |  | 0.649 |
| walker |  | 3998 | 15 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 14, sub: 0, line: 157 } |  |  | 0.649 |
| walker |  | 4013 | 15 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 26, sub: 0, line: 286 } |  |  | 0.649 |
| walker |  | 4029 | 16 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 21, sub: 0, line: 241 } |  |  | 0.649 |
| walker |  | 4046 | 17 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 12, sub: 0, line: 146 } |  |  | 0.650 |
| ns | 4057 |  | 135 | `htmy/renderer/__init__.py` in full: which class each renderer name resolves to | 5.1 |  | 0.647 |
| walker |  | 4147 | 101 | Markdown::Section { file: README.md, section_index: 13, keeps_default_concavity: false } |  |  | 0.647 |
| walker |  | 4153 | 6 | Code::CodeKey { rung: Body, file: htmy/typing.py, decl: 25, sub: 0, line: 108 } |  |  | 0.647 |
| ns | 4299 |  | 242 | `htmy/renderer/typing.py`: all four symbols of the renderer protocol module | 5.2 |  | 0.628 |
| walker |  | 4309 | 156 | Code::CodeKey { rung: Names, file: htmy/utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.628 |
| walker |  | 4338 | 29 | Code::CodeKey { rung: Decl, file: htmy/utils.py, decl: 1, sub: 0, line: 12 } |  |  | 0.629 |
| walker |  | 4355 | 17 | Code::CodeKey { rung: Doc, file: htmy/utils.py, decl: 6, sub: 0, line: 67 } |  |  | 0.629 |
| walker |  | 4373 | 18 | Code::CodeKey { rung: Doc, file: htmy/utils.py, decl: 4, sub: 0, line: 55 } |  |  | 0.629 |
| walker |  | 4391 | 18 | Code::CodeKey { rung: Doc, file: htmy/utils.py, decl: 5, sub: 0, line: 62 } |  |  | 0.629 |
| walker |  | 4412 | 21 | Code::CodeKey { rung: Doc, file: htmy/utils.py, decl: 7, sub: 0, line: 73 } |  |  | 0.629 |
| walker |  | 4434 | 22 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 13, sub: 0, line: 151 } |  |  | 0.629 |
| ns | 4480 |  | 181 | `Renderer` in `htmy/renderer/default.py`: strategy docstring and `__init__` | 5.3 |  | 0.612 |
| walker |  | 4531 | 97 | Markdown::Section { file: README.md, section_index: 15, keeps_default_concavity: false } |  |  | 0.612 |
| walker |  | 4539 | 8 | Code::CodeKey { rung: Body, file: htmy/tag.py, decl: 9, sub: 0, line: 66 } |  |  | 0.612 |
| walker |  | 4562 | 23 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 4, sub: 0, line: 38 } |  |  | 0.613 |
| walker |  | 4574 | 12 | Code::CodeKey { rung: Body, file: htmy/utils.py, decl: 5, sub: 0, line: 62 } |  |  | 0.613 |
| ns | 4667 |  | 187 | `Renderer.render()` in `default.py`: context layering and renderer self-registration | 5.4 | 5.3 | 0.602 |
| walker |  | 4710 | 136 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.602 |
| ns | 4905 |  | 238 | `Renderer` in `htmy/renderer/baseline.py`: why it exists, plus its full method roster | 5.5 |  | 0.586 |
| walker |  | 4911 | 201 | Code::CodeKey { rung: Names, file: htmy/html.py, decl: 0, sub: 0, line: 0 } |  |  | 0.586 |
| walker |  | 4935 | 24 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 1, sub: 0, line: 13 } |  |  | 0.587 |
| walker |  | 4961 | 26 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 8, sub: 0, line: 63 } |  |  | 0.587 |
| ns | 5045 |  | 140 | `htmy/renderer/context.py`: `RendererContext.from_context()` | 5.6 |  | 0.575 |
| walker |  | 5104 | 143 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 11, sub: 0, line: 79 } |  |  | 0.575 |
| walker |  | 5125 | 21 | Code::CodeKey { rung: Names, file: htmy/snippet.py, decl: 0, sub: 0, line: 0 } |  |  | 0.575 |
| ns | 5270 |  | 225 | `core.py`: `Fragment` and `WithContext` | 6.1 |  | 0.578 |
| walker |  | 5278 | 153 | Code::CodeKey { rung: Decl, file: htmy/snippet.py, decl: 5, sub: 0, line: 158 } |  |  | 0.578 |
| walker |  | 5335 | 57 | Code::CodeKey { rung: Decl, file: htmy/snippet.py, decl: 6, sub: 0, line: 218 } |  |  | 0.578 |
| walker |  | 5346 | 11 | Code::CodeKey { rung: Doc, file: htmy/snippet.py, decl: 7, sub: 0, line: 241 } |  |  | 0.578 |
| walker |  | 5354 | 8 | Code::CodeKey { rung: Doc, file: htmy/html.py, decl: 1, sub: 0, line: 13 } |  |  | 0.578 |
| walker |  | 5369 | 15 | Code::CodeKey { rung: Doc, file: htmy/snippet.py, decl: 10, sub: 0, line: 272 } |  |  | 0.578 |
| ns | 5522 |  | 252 | `core.py`: `ContextAware`, the typed context registration base class | 6.2 |  | 0.571 |
| walker |  | 5602 | 233 | Code::CodeKey { rung: Decl, file: htmy/snippet.py, decl: 1, sub: 0, line: 27 } |  |  | 0.571 |
| ns | 5795 |  | 273 | `core.py`: `SkipProperty`, `Text`, `SafeStr`, `XBool`, `xml_format_string` | 6.3 |  | 0.565 |
| walker |  | 5885 | 283 | Code::CodeKey { rung: Decl, file: htmy/snippet.py, decl: 1, sub: 1, line: 27 } |  |  | 0.566 |
| walker |  | 5940 | 55 | Code::CodeKey { rung: Decl, file: htmy/snippet.py, decl: 2, sub: 0, line: 83 } |  |  | 0.566 |
| walker |  | 5956 | 16 | Code::CodeKey { rung: Doc, file: htmy/snippet.py, decl: 8, sub: 0, line: 256 } |  |  | 0.566 |
| walker |  | 5966 | 10 | Code::CodeKey { rung: Doc, file: htmy/html.py, decl: 8, sub: 0, line: 63 } |  |  | 0.566 |
| walker |  | 5976 | 10 | Code::CodeKey { rung: Doc, file: htmy/html.py, decl: 11, sub: 0, line: 79 } |  |  | 0.566 |
| walker |  | 5989 | 13 | Code::CodeKey { rung: Names, file: htmy/etree.py, decl: 0, sub: 0, line: 0 } |  |  | 0.566 |
| ns | 6085 |  | 290 | `core.py`: `Formatter` -- construction and the complete method surface | 6.4 |  | 0.562 |
| walker |  | 6222 | 233 | Code::CodeKey { rung: Decl, file: htmy/etree.py, decl: 1, sub: 0, line: 23 } |  |  | 0.562 |
| walker |  | 6237 | 15 | Code::CodeKey { rung: Doc, file: htmy/etree.py, decl: 7, sub: 0, line: 103 } |  |  | 0.562 |
| ns | 6259 |  | 174 | `core.py`: the default value-formatter table | 6.5 | 6.4 | 0.554 |
| walker |  | 6263 | 26 | Code::CodeKey { rung: Names, file: htmy/md/core.py, decl: 0, sub: 0, line: 0 } |  |  | 0.554 |
| walker |  | 6346 | 83 | Code::CodeKey { rung: Decl, file: htmy/md/core.py, decl: 6, sub: 0, line: 81 } |  |  | 0.554 |
| walker |  | 6436 | 90 | Code::CodeKey { rung: Decl, file: htmy/md/core.py, decl: 7, sub: 0, line: 103 } |  |  | 0.554 |
| ns | 6452 |  | 193 | `htmy/snippet.py`: `Slots` and the slot placeholder syntax | 7.1 |  | 0.559 |
| walker |  | 6585 | 149 | Code::CodeKey { rung: Decl, file: htmy/md/core.py, decl: 1, sub: 0, line: 20 } |  |  | 0.559 |
| walker |  | 6613 | 28 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 24, sub: 0, line: 103 } |  |  | 0.566 |
| ns | 6744 |  | 292 | `htmy/snippet.py`: `Snippet` and its four-step text pipeline | 7.2 |  | 0.559 |
| walker |  | 6841 | 228 | Code::CodeKey { rung: Names, file: htmy/html.py, decl: 0, sub: 1, line: 0 } |  |  | 0.560 |
| ns | 7031 |  | 287 | `htmy/function_component.py`: the complete `@component` decorator family | 7.3 |  | 0.549 |
| walker |  | 7070 | 229 | Code::CodeKey { rung: Names, file: htmy/html.py, decl: 0, sub: 2, line: 0 } |  |  | 0.550 |
| ns | 7205 |  | 174 | `htmy/md/core.py`: `MarkdownParser` | 7.4 |  | 0.551 |
| walker |  | 7275 | 205 | Code::CodeKey { rung: Names, file: htmy/html.py, decl: 0, sub: 3, line: 0 } |  |  | 0.551 |
| walker |  | 7477 | 202 | Code::CodeKey { rung: Names, file: htmy/html.py, decl: 0, sub: 4, line: 0 } |  |  | 0.552 |
| ns | 7514 |  | 309 | `htmy/md/core.py`: the `MD` component, plus the `md/typing.py` definitions | 7.5 |  | 0.544 |
| walker |  | 7681 | 204 | Code::CodeKey { rung: Names, file: htmy/html.py, decl: 0, sub: 5, line: 0 } |  |  | 0.544 |
| ns | 7773 |  | 259 | `htmy/i18n.py`: `I18n`, its error hierarchy, and the resource loader | 7.6 |  | 0.537 |
| walker |  | 7891 | 210 | Code::CodeKey { rung: Names, file: htmy/html.py, decl: 0, sub: 6, line: 0 } |  |  | 0.537 |
| ns | 8005 |  | 232 | `htmy/tag.py`: how every `html.*` tag is actually built | 7.7 |  | 0.541 |
| walker |  | 8038 | 147 | Code::CodeKey { rung: Names, file: htmy/html.py, decl: 0, sub: 7, line: 0 } |  |  | 0.542 |
| ns | 8206 |  | 201 | `htmy/error_boundary.py` in outline: `ErrorBoundary` | 7.8 |  | 0.535 |
| walker |  | 8235 | 197 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 120, sub: 0, line: 831 } |  |  | 0.535 |
| walker |  | 8252 | 17 | Code::CodeKey { rung: Doc, file: htmy/etree.py, decl: 4, sub: 0, line: 63 } |  |  | 0.535 |
| walker |  | 8281 | 29 | Code::CodeKey { rung: Doc, file: htmy/utils.py, decl: 3, sub: 0, line: 49 } |  |  | 0.535 |
| walker |  | 8312 | 31 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 18, sub: 0, line: 196 } |  |  | 0.538 |
| ns | 8453 |  | 247 | `htmy/etree.py`: `ETreeConverter` and the optional-`lxml` import switch | 7.9 |  | 0.538 |
| walker |  | 8459 | 147 | Markdown::Section { file: README.md, section_index: 12, keeps_default_concavity: false } |  |  | 0.550 |
| walker |  | 8492 | 33 | Code::CodeKey { rung: Doc, file: htmy/utils.py, decl: 2, sub: 0, line: 42 } |  |  | 0.550 |
| walker |  | 8506 | 14 | Code::CodeKey { rung: Body, file: htmy/utils.py, decl: 2, sub: 0, line: 42 } |  |  | 0.550 |
| walker |  | 8660 | 154 | Code::CodeKey { rung: Names, file: htmy/i18n.py, decl: 0, sub: 0, line: 0 } |  |  | 0.555 |
| ns | 8676 |  | 223 | `htmy/utils.py` and `htmy/io.py`: every helper, by signature | 7.10 |  | 0.559 |
| ns | 8818 |  | 142 | `html.py` tag roster 1/5: document skeleton, `Link` and `Meta` factories | 8.1 |  | 0.567 |
| walker |  | 8840 | 180 | Code::CodeKey { rung: Decl, file: htmy/i18n.py, decl: 5, sub: 0, line: 24 } |  |  | 0.579 |
| walker |  | 8852 | 12 | Code::CodeKey { rung: Names, file: htmy/error_boundary.py, decl: 0, sub: 0, line: 0 } |  |  | 0.579 |
| walker |  | 8939 | 87 | Code::CodeKey { rung: Decl, file: htmy/error_boundary.py, decl: 1, sub: 0, line: 15 } |  |  | 0.581 |
| walker |  | 8983 | 44 | Code::CodeKey { rung: Decl, file: htmy/error_boundary.py, decl: 2, sub: 0, line: 25 } |  |  | 0.581 |
| walker |  | 9004 | 21 | Code::CodeKey { rung: Doc, file: htmy/etree.py, decl: 3, sub: 0, line: 55 } |  |  | 0.581 |
| walker |  | 9041 | 37 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 17, sub: 0, line: 186 } |  |  | 0.585 |
| ns | 9055 |  | 237 | `html.py` tag roster 2/5: sectioning and container tags | 8.2 | 8.1 | 0.597 |
| walker |  | 9063 | 22 | Code::CodeKey { rung: Doc, file: htmy/i18n.py, decl: 5, sub: 0, line: 24 } |  |  | 0.597 |
| walker |  | 9074 | 11 | Code::CodeKey { rung: Body, file: htmy/etree.py, decl: 2, sub: 0, line: 46 } |  |  | 0.597 |
| walker |  | 9114 | 40 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 9, sub: 0, line: 108 } |  |  | 0.597 |
| walker |  | 9136 | 22 | Code::CodeKey { rung: Names, file: htmy/io.py, decl: 0, sub: 0, line: 0 } |  |  | 0.599 |
| ns | 9220 |  | 165 | `html.py` tag roster 3/5: form and interactive tags | 8.3 | 8.1 | 0.607 |
| walker |  | 9360 | 224 | Code::CodeKey { rung: Names, file: htmy/function_component.py, decl: 0, sub: 0, line: 0 } |  |  | 0.607 |
| walker |  | 9376 | 16 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 4, sub: 0, line: 31 } |  |  | 0.607 |
| walker |  | 9394 | 18 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 7, sub: 0, line: 46 } |  |  | 0.607 |
| ns | 9544 |  | 324 | `html.py` tag roster 4/5: text-level tags and embedded media | 8.4 | 8.1 | 0.620 |
| walker |  | 9597 | 203 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 11, sub: 0, line: 71 } |  |  | 0.620 |
| walker |  | 9623 | 26 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 14, sub: 0, line: 194 } |  |  | 0.620 |
| walker |  | 9650 | 27 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 12, sub: 0, line: 86 } |  |  | 0.620 |
| walker |  | 9677 | 27 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 13, sub: 0, line: 147 } |  |  | 0.620 |
| walker |  | 9706 | 29 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 16, sub: 0, line: 315 } |  |  | 0.620 |
| walker |  | 9741 | 35 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 15, sub: 0, line: 244 } |  |  | 0.620 |
| walker |  | 9760 | 19 | Code::CodeKey { rung: Doc, file: htmy/function_component.py, decl: 11, sub: 0, line: 71 } |  |  | 0.620 |
| walker |  | 9775 | 15 | Code::CodeKey { rung: Doc, file: htmy/io.py, decl: 1, sub: 0, line: 11 } |  |  | 0.620 |
| walker |  | 9814 | 39 | Code::CodeKey { rung: Doc, file: htmy/function_component.py, decl: 4, sub: 0, line: 31 } |  |  | 0.620 |
| walker |  | 9853 | 39 | Code::CodeKey { rung: Doc, file: htmy/function_component.py, decl: 7, sub: 0, line: 46 } |  |  | 0.620 |
| ns | 9889 |  | 345 | `html.py` tag roster 5/5: lists, tables, headings, media, `entity` | 8.5 | 8.1 | 0.633 |
| walker |  | 9930 | 77 | Code::CodeKey { rung: Names, file: htmy/renderer/typing.py, decl: 0, sub: 0, line: 0 } |  |  | 0.634 |
| walker |  | 9957 | 27 | Code::CodeKey { rung: Decl, file: htmy/renderer/typing.py, decl: 1, sub: 0, line: 12 } |  |  | 0.635 |
| walker |  | 9985 | 28 | Code::CodeKey { rung: Decl, file: htmy/renderer/typing.py, decl: 4, sub: 0, line: 36 } |  |  | 0.636 |
| walker |  | 9991 | 6 | Code::CodeKey { rung: Body, file: htmy/renderer/typing.py, decl: 2, sub: 0, line: 15 } |  |  | 0.636 |
| walker |  | 9997 | 6 | Code::CodeKey { rung: Body, file: htmy/renderer/typing.py, decl: 5, sub: 0, line: 39 } |  |  | 0.636 |

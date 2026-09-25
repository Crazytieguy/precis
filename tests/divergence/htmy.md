Score(3000)=0.664 I=0.853 C=0.517 ns_rows≤3K=19/49 grid(1000/1442/2080/3000/4327/6240/9000)=0.567/0.659/0.713/0.664/0.601/0.559/0.590

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 33 | 33 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 53 | 20 | Fs::DirListing { dir: docs } |  |  | 0.000 |
| walker |  | 60 | 7 | Fs::DirListing { dir: .github } |  |  | 0.000 |
| ns | 72 |  | 72 | README identity: name, one-line description, pitch | 1.1 |  | 0.000 |
| walker |  | 79 | 19 | Fs::DirListing { dir: .github/workflows } |  |  | 0.000 |
| ns | 105 |  | 33 | Repository root listing (complete) | 1.2 |  | 0.473 |
| walker |  | 152 | 73 | Toml::Identity { file: pyproject.toml } |  |  | 0.476 |
| ns | 208 |  | 103 | The `htmy/` package and its two subpackages (complete) | 1.3 |  | 0.272 |
| walker |  | 219 | 67 | Fs::DirListing { dir: htmy } |  |  | 0.439 |
| walker |  | 236 | 17 | Code::CodeKey { rung: ModuleDoc, file: htmy/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.439 |
| walker |  | 250 | 14 | Fs::DirListing { dir: htmy/md } |  |  | 0.505 |
| walker |  | 272 | 22 | Fs::DirListing { dir: htmy/renderer } |  |  | 0.639 |
| walker |  | 284 | 12 | Code::CodeKey { rung: Names, file: htmy/error_boundary.py, decl: 0, sub: 0, line: 0 } |  |  | 0.639 |
| walker |  | 297 | 13 | Code::CodeKey { rung: Names, file: htmy/etree.py, decl: 0, sub: 0, line: 0 } |  |  | 0.639 |
| ns | 349 |  | 141 | README key features, first half | 1.4 |  | 0.570 |
| ns | 504 |  | 155 | README key features, second half | 1.5 | 1.4 | 0.518 |
| walker |  | 543 | 246 | Code::CodeKey { rung: Names, file: htmy/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.552 |
| walker |  | 631 | 88 | Code::CodeKey { rung: Names, file: htmy/md/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.552 |
| ns | 648 |  | 144 | `htmy/__init__.py` exports, part 1: version + everything from `core` | 1.6 |  | 0.562 |
| walker |  | 652 | 21 | Code::CodeKey { rung: Names, file: htmy/snippet.py, decl: 0, sub: 0, line: 0 } |  |  | 0.562 |
| walker |  | 663 | 11 | Code::CodeKey { rung: Names, file: htmy/renderer/baseline.py, decl: 0, sub: 0, line: 0 } |  |  | 0.562 |
| walker |  | 685 | 22 | Code::CodeKey { rung: Names, file: htmy/io.py, decl: 0, sub: 0, line: 0 } |  |  | 0.562 |
| ns | 786 |  | 138 | `htmy/__init__.py` exports, part 2: `ErrorBoundary`, `component`, renderers, `Snippet`/`Slots`, tags | 1.7 | 1.6 | 0.562 |
| walker |  | 788 | 103 | Code::CodeKey { rung: Names, file: htmy/renderer/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.565 |
| walker |  | 800 | 12 | Code::CodeKey { rung: Names, file: htmy/renderer/context.py, decl: 0, sub: 0, line: 0 } |  |  | 0.565 |
| walker |  | 842 | 42 | Fs::DirListing { dir: docs/api } |  |  | 0.565 |
| walker |  | 854 | 12 | Fs::DirListing { dir: docs/api/renderer } |  |  | 0.565 |
| walker |  | 874 | 20 | Code::CodeKey { rung: Decl, file: htmy/renderer/context.py, decl: 1, sub: 0, line: 6 } |  |  | 0.565 |
| walker |  | 882 | 8 | Code::CodeKey { rung: Decl, file: htmy/renderer/context.py, decl: 2, sub: 0, line: 11 } |  |  | 0.565 |
| walker |  | 911 | 29 | Toml::PackageMetadata { file: pyproject.toml } |  |  | 0.567 |
| ns | 1014 |  | 228 | `htmy/__init__.py` exports, part 3: the sixteen re-exported type names | 1.8 | 1.7 | 0.506 |
| walker |  | 1060 | 149 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.747 |
| walker |  | 1082 | 22 | Fs::DirListing { dir: examples } |  |  | 0.747 |
| ns | 1160 |  | 146 | `htmy/__init__.py` exports, part 4: `utils` helpers and the two aliases | 1.9 | 1.8 | 0.694 |
| walker |  | 1175 | 93 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.699 |
| walker |  | 1201 | 26 | Code::CodeKey { rung: Names, file: htmy/md/core.py, decl: 0, sub: 0, line: 0 } |  |  | 0.699 |
| ns | 1424 |  | 264 | README "Built-in components": one line of semantics per built-in | 2.1 |  | 0.659 |
| walker |  | 1443 | 242 | Code::CodeKey { rung: Names, file: htmy/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.772 |
| walker |  | 1456 | 13 | Fs::DirListing { dir: examples/internationalization } |  |  | 0.772 |
| walker |  | 1470 | 14 | Fs::DirListing { dir: examples/markdown_customization } |  |  | 0.772 |
| walker |  | 1484 | 14 | Fs::DirListing { dir: examples/markdown_essentials } |  |  | 0.772 |
| walker |  | 1560 | 76 | Code::CodeKey { rung: Names, file: htmy/tag.py, decl: 0, sub: 0, line: 0 } |  |  | 0.772 |
| ns | 1612 |  | 188 | README: the definition of a component (duck-typed `htmy()` method) | 2.2 |  | 0.753 |
| walker |  | 1617 | 57 | Code::CodeKey { rung: Decl, file: htmy/tag.py, decl: 14, sub: 0, line: 84 } |  |  | 0.753 |
| walker |  | 1678 | 61 | Code::CodeKey { rung: Decl, file: htmy/tag.py, decl: 8, sub: 0, line: 56 } |  |  | 0.753 |
| walker |  | 1754 | 76 | Code::CodeKey { rung: Decl, file: htmy/error_boundary.py, decl: 1, sub: 0, line: 15 } |  |  | 0.754 |
| ns | 1758 |  | 146 | README "Rendering": how to actually invoke the renderer | 2.3 |  | 0.729 |
| walker |  | 1831 | 77 | Code::CodeKey { rung: Decl, file: htmy/tag.py, decl: 11, sub: 0, line: 73 } |  |  | 0.729 |
| walker |  | 1887 | 56 | Markdown::HeadingsOutline { file: docs/components-guide.md } |  |  | 0.729 |
| ns | 1900 |  | 142 | README "Context": the prop-drilling escape hatch and `htmy_context()` | 2.4 |  | 0.713 |
| walker |  | 1902 | 15 | Code::CodeKey { rung: Doc, file: htmy/io.py, decl: 1, sub: 0, line: 11 } |  |  | 0.713 |
| walker |  | 1961 | 59 | Markdown::HeadingsOutline { file: docs/function-components.md } |  |  | 0.713 |
| walker |  | 1969 | 8 | Code::CodeKey { rung: Body, file: htmy/tag.py, decl: 9, sub: 0, line: 66 } |  |  | 0.713 |
| walker |  | 2105 | 136 | Code::CodeKey { rung: Names, file: htmy/__init__.py, decl: 0, sub: 2, line: 0 } |  |  | 0.767 |
| ns | 2170 |  | 270 | README "Formatter": the default attribute name/value conversion rules | 2.5 |  | 0.744 |
| walker |  | 2209 | 104 | Code::CodeKey { rung: Names, file: htmy/core.py, decl: 0, sub: 0, line: 0 } |  |  | 0.744 |
| walker |  | 2215 | 6 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 14, sub: 0, line: 157 } |  |  | 0.744 |
| walker |  | 2221 | 6 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 15, sub: 0, line: 163 } |  |  | 0.744 |
| walker |  | 2247 | 26 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 12, sub: 0, line: 146 } |  |  | 0.745 |
| walker |  | 2253 | 6 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 13, sub: 0, line: 151 } |  |  | 0.745 |
| walker |  | 2290 | 37 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 16, sub: 0, line: 175 } |  |  | 0.745 |
| ns | 2330 |  | 160 | README "XSS prevention": escaping by default, and the two exceptions | 2.6 |  | 0.721 |
| walker |  | 2350 | 60 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 1, sub: 0, line: 19 } |  |  | 0.721 |
| walker |  | 2411 | 61 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 4, sub: 0, line: 38 } |  |  | 0.721 |
| walker |  | 2466 | 55 | Code::CodeKey { rung: Decl, file: htmy/error_boundary.py, decl: 2, sub: 0, line: 25 } |  |  | 0.721 |
| walker |  | 2525 | 59 | Fs::DirListing { dir: tests } |  |  | 0.726 |
| ns | 2534 |  | 204 | `pyproject.toml` project block: runtime deps, Python floor, optional `lxml` | 3.1 |  | 0.736 |
| walker |  | 2649 | 124 | Code::CodeKey { rung: Decl, file: htmy/snippet.py, decl: 5, sub: 0, line: 158 } |  |  | 0.736 |
| walker |  | 2667 | 18 | Code::CodeKey { rung: Decl, file: htmy/snippet.py, decl: 10, sub: 0, line: 272 } |  |  | 0.736 |
| walker |  | 2678 | 11 | Code::CodeKey { rung: Doc, file: htmy/snippet.py, decl: 7, sub: 0, line: 241 } |  |  | 0.736 |
| ns | 2688 |  | 154 | `[tool.poe.tasks]`: the project's canonical commands | 3.2 |  | 0.700 |
| ns | 2794 |  | 106 | Test suite and CI workflow listings (complete) | 3.3 |  | 0.698 |
| walker |  | 2803 | 125 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 7, sub: 0, line: 64 } |  |  | 0.698 |
| walker |  | 2811 | 8 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 11, sub: 0, line: 127 } |  |  | 0.698 |
| walker |  | 2879 | 68 | Code::CodeKey { rung: Decl, file: htmy/snippet.py, decl: 6, sub: 0, line: 218 } |  |  | 0.698 |
| ns | 2950 |  | 156 | `tests/conftest.py` in full: the three session-scoped renderer fixtures | 3.4 |  | 0.664 |
| walker |  | 3018 | 139 | Code::CodeKey { rung: Names, file: htmy/utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.664 |
| walker |  | 3064 | 46 | Code::CodeKey { rung: Decl, file: htmy/utils.py, decl: 1, sub: 0, line: 12 } |  |  | 0.665 |
| walker |  | 3078 | 14 | Code::CodeKey { rung: Body, file: htmy/utils.py, decl: 5, sub: 0, line: 62 } |  |  | 0.665 |
| walker |  | 3094 | 16 | Code::CodeKey { rung: Doc, file: htmy/utils.py, decl: 5, sub: 0, line: 62 } |  |  | 0.665 |
| walker |  | 3110 | 16 | Code::CodeKey { rung: Body, file: htmy/utils.py, decl: 2, sub: 0, line: 42 } |  |  | 0.665 |
| ns | 3118 |  | 168 | `pyproject.toml` build backend, version source and type/test settings | 3.5 |  | 0.633 |
| walker |  | 3183 | 73 | Code::CodeKey { rung: Names, file: htmy/renderer/default.py, decl: 0, sub: 0, line: 0 } |  |  | 0.633 |
| walker |  | 3244 | 61 | Code::CodeKey { rung: Decl, file: htmy/renderer/default.py, decl: 13, sub: 0, line: 228 } |  |  | 0.633 |
| walker |  | 3272 | 28 | Fs::DirListing { dir: tests/renderer } |  |  | 0.656 |
| ns | 3335 |  | 217 | `htmy/typing.py` lines 4-27: `T`/`U`, property types, the context types | 4.1 |  | 0.620 |
| walker |  | 3484 | 212 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.621 |
| ns | 3649 |  | 314 | `htmy/typing.py` lines 34-67: the component protocols and the `Component` union | 4.2 | 4.1 | 0.587 |
| walker |  | 3770 | 286 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.645 |
| walker |  | 3915 | 145 | Code::CodeKey { rung: Names, file: htmy/i18n.py, decl: 0, sub: 0, line: 0 } |  |  | 0.646 |
| walker |  | 3924 | 9 | Code::CodeKey { rung: Decl, file: htmy/i18n.py, decl: 11, sub: 0, line: 115 } |  |  | 0.646 |
| ns | 3950 |  | 301 | `htmy/typing.py` lines 70-109: context providers, `TextProcessor`, `TextResolver` | 4.3 | 4.1 | 0.608 |
| walker |  | 3996 | 72 | Code::CodeKey { rung: Decl, file: htmy/md/core.py, decl: 6, sub: 0, line: 81 } |  |  | 0.608 |
| walker |  | 4073 | 77 | Code::CodeKey { rung: Names, file: htmy/renderer/typing.py, decl: 0, sub: 0, line: 0 } |  |  | 0.609 |
| ns | 4085 |  | 135 | `htmy/renderer/__init__.py` in full: which class each renderer name resolves to | 5.1 |  | 0.608 |
| walker |  | 4100 | 27 | Code::CodeKey { rung: Decl, file: htmy/renderer/typing.py, decl: 1, sub: 0, line: 12 } |  |  | 0.608 |
| walker |  | 4128 | 28 | Code::CodeKey { rung: Decl, file: htmy/renderer/typing.py, decl: 4, sub: 0, line: 36 } |  |  | 0.608 |
| walker |  | 4138 | 10 | Code::CodeKey { rung: Doc, file: htmy/renderer/typing.py, decl: 1, sub: 0, line: 12 } |  |  | 0.609 |
| walker |  | 4149 | 11 | Code::CodeKey { rung: Doc, file: htmy/renderer/typing.py, decl: 4, sub: 0, line: 36 } |  |  | 0.609 |
| walker |  | 4166 | 17 | Code::CodeKey { rung: Doc, file: htmy/utils.py, decl: 6, sub: 0, line: 67 } |  |  | 0.609 |
| walker |  | 4199 | 33 | Markdown::Section { file: README.md, section_index: 23, keeps_default_concavity: false } |  |  | 0.609 |
| walker |  | 4285 | 86 | Code::CodeKey { rung: Names, file: htmy/md/typing.py, decl: 0, sub: 0, line: 0 } |  |  | 0.609 |
| walker |  | 4310 | 25 | Code::CodeKey { rung: Decl, file: htmy/md/typing.py, decl: 2, sub: 0, line: 14 } |  |  | 0.609 |
| walker |  | 4321 | 11 | Code::CodeKey { rung: Doc, file: htmy/md/typing.py, decl: 2, sub: 0, line: 14 } |  |  | 0.609 |
| ns | 4327 |  | 242 | `htmy/renderer/typing.py`: all four symbols of the renderer protocol module | 5.2 |  | 0.601 |
| walker |  | 4347 | 26 | Code::CodeKey { rung: Body, file: htmy/io.py, decl: 1, sub: 0, line: 11 } |  |  | 0.601 |
| walker |  | 4381 | 34 | Fs::DirListing { dir: docs/examples } |  |  | 0.601 |
| ns | 4508 |  | 181 | `Renderer` in `htmy/renderer/default.py`: strategy docstring and `__init__` | 5.3 |  | 0.587 |
| walker |  | 4553 | 172 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 19, sub: 0, line: 201 } |  |  | 0.587 |
| walker |  | 4611 | 58 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 20, sub: 0, line: 222 } |  |  | 0.588 |
| ns | 4695 |  | 187 | `Renderer.render()` in `default.py`: context layering and renderer self-registration | 5.4 | 5.3 | 0.578 |
| ns | 4933 |  | 238 | `Renderer` in `htmy/renderer/baseline.py`: why it exists, plus its full method roster | 5.5 |  | 0.562 |
| walker |  | 4947 | 336 | Code::CodeKey { rung: Names, file: htmy/typing.py, decl: 0, sub: 0, line: 0 } |  |  | 0.581 |
| walker |  | 4963 | 16 | Code::CodeKey { rung: Decl, file: htmy/typing.py, decl: 18, sub: 0, line: 73 } |  |  | 0.582 |
| walker |  | 4980 | 17 | Code::CodeKey { rung: Decl, file: htmy/typing.py, decl: 20, sub: 0, line: 81 } |  |  | 0.584 |
| walker |  | 4999 | 19 | Code::CodeKey { rung: Decl, file: htmy/typing.py, decl: 24, sub: 0, line: 103 } |  |  | 0.585 |
| walker |  | 5021 | 22 | Code::CodeKey { rung: Decl, file: htmy/typing.py, decl: 9, sub: 0, line: 37 } |  |  | 0.587 |
| walker |  | 5044 | 23 | Code::CodeKey { rung: Decl, file: htmy/typing.py, decl: 11, sub: 0, line: 45 } |  |  | 0.589 |
| walker |  | 5055 | 11 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 18, sub: 0, line: 73 } |  |  | 0.591 |
| walker |  | 5066 | 11 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 20, sub: 0, line: 81 } |  |  | 0.594 |
| ns | 5073 |  | 140 | `htmy/renderer/context.py`: `RendererContext.from_context()` | 5.6 |  | 0.583 |
| walker |  | 5121 | 55 | Code::CodeKey { rung: Decl, file: htmy/tag.py, decl: 4, sub: 0, line: 24 } |  |  | 0.583 |
| ns | 5298 |  | 225 | `core.py`: `Fragment` and `WithContext` | 6.1 |  | 0.575 |
| walker |  | 5382 | 261 | Code::CodeKey { rung: Names, file: htmy/html.py, decl: 0, sub: 0, line: 0 } |  |  | 0.576 |
| walker |  | 5400 | 18 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 8, sub: 0, line: 63 } |  |  | 0.576 |
| walker |  | 5408 | 8 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 9, sub: 0, line: 66 } |  |  | 0.576 |
| walker |  | 5432 | 24 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 1, sub: 0, line: 13 } |  |  | 0.576 |
| walker |  | 5440 | 8 | Code::CodeKey { rung: Doc, file: htmy/html.py, decl: 1, sub: 0, line: 13 } |  |  | 0.576 |
| walker |  | 5450 | 10 | Code::CodeKey { rung: Doc, file: htmy/html.py, decl: 8, sub: 0, line: 63 } |  |  | 0.576 |
| ns | 5550 |  | 252 | `core.py`: `ContextAware`, the typed context registration base class | 6.2 |  | 0.569 |
| walker |  | 5553 | 103 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 11, sub: 0, line: 79 } |  |  | 0.569 |
| walker |  | 5561 | 8 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 12, sub: 0, line: 82 } |  |  | 0.569 |
| walker |  | 5569 | 8 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 13, sub: 0, line: 86 } |  |  | 0.569 |
| walker |  | 5577 | 8 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 14, sub: 0, line: 90 } |  |  | 0.569 |
| walker |  | 5585 | 8 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 15, sub: 0, line: 94 } |  |  | 0.569 |
| walker |  | 5593 | 8 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 16, sub: 0, line: 98 } |  |  | 0.569 |
| walker |  | 5599 | 6 | Code::CodeKey { rung: Body, file: htmy/renderer/typing.py, decl: 2, sub: 0, line: 15 } |  |  | 0.569 |
| walker |  | 5654 | 55 | Code::CodeKey { rung: Decl, file: htmy/renderer/default.py, decl: 14, sub: 0, line: 238 } |  |  | 0.573 |
| walker |  | 5716 | 62 | Code::CodeKey { rung: Decl, file: htmy/tag.py, decl: 1, sub: 0, line: 12 } |  |  | 0.573 |
| ns | 5823 |  | 273 | `core.py`: `SkipProperty`, `Text`, `SafeStr`, `XBool`, `xml_format_string` | 6.3 |  | 0.562 |
| walker |  | 5940 | 224 | Code::CodeKey { rung: Names, file: htmy/function_component.py, decl: 0, sub: 0, line: 0 } |  |  | 0.562 |
| walker |  | 5956 | 16 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 4, sub: 0, line: 31 } |  |  | 0.562 |
| walker |  | 5974 | 18 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 7, sub: 0, line: 46 } |  |  | 0.562 |
| walker |  | 5992 | 18 | Code::CodeKey { rung: Doc, file: htmy/utils.py, decl: 4, sub: 0, line: 55 } |  |  | 0.562 |
| ns | 6113 |  | 290 | `core.py`: `Formatter` -- construction and the complete method surface | 6.4 |  | 0.558 |
| walker |  | 6222 | 230 | Code::CodeKey { rung: Decl, file: htmy/i18n.py, decl: 5, sub: 0, line: 24 } |  |  | 0.559 |
| walker |  | 6229 | 7 | Code::CodeKey { rung: Decl, file: htmy/i18n.py, decl: 8, sub: 0, line: 48 } |  |  | 0.559 |
| walker |  | 6238 | 9 | Code::CodeKey { rung: Decl, file: htmy/i18n.py, decl: 7, sub: 0, line: 45 } |  |  | 0.559 |
| walker |  | 6246 | 8 | Code::CodeKey { rung: Decl, file: htmy/i18n.py, decl: 10, sub: 0, line: 79 } |  |  | 0.559 |
| ns | 6287 |  | 174 | `core.py`: the default value-formatter table | 6.5 | 6.4 | 0.551 |
| walker |  | 6292 | 46 | Markdown::Section { file: README.md, section_index: 17, keeps_default_concavity: false } |  |  | 0.551 |
| ns | 6480 |  | 193 | `htmy/snippet.py`: `Slots` and the slot placeholder syntax | 7.1 |  | 0.543 |
| walker |  | 6525 | 233 | Code::CodeKey { rung: Decl, file: htmy/etree.py, decl: 1, sub: 0, line: 23 } |  |  | 0.543 |
| walker |  | 6536 | 11 | Code::CodeKey { rung: Body, file: htmy/etree.py, decl: 2, sub: 0, line: 46 } |  |  | 0.543 |
| walker |  | 6553 | 17 | Code::CodeKey { rung: Doc, file: htmy/etree.py, decl: 4, sub: 0, line: 63 } |  |  | 0.543 |
| walker |  | 6574 | 21 | Code::CodeKey { rung: Doc, file: htmy/etree.py, decl: 3, sub: 0, line: 55 } |  |  | 0.543 |
| walker |  | 6646 | 72 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.543 |
| ns | 6772 |  | 292 | `htmy/snippet.py`: `Snippet` and its four-step text pipeline | 7.2 |  | 0.537 |
| walker |  | 6908 | 262 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 11, sub: 0, line: 71 } |  |  | 0.538 |
| walker |  | 6915 | 7 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 13, sub: 0, line: 83 } |  |  | 0.538 |
| walker |  | 6922 | 7 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 16, sub: 0, line: 144 } |  |  | 0.538 |
| walker |  | 6931 | 9 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 12, sub: 0, line: 80 } |  |  | 0.538 |
| walker |  | 6940 | 9 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 15, sub: 0, line: 141 } |  |  | 0.538 |
| walker |  | 6980 | 40 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 19, sub: 0, line: 189 } |  |  | 0.538 |
| walker |  | 7020 | 40 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 18, sub: 0, line: 184 } |  |  | 0.538 |
| ns | 7059 |  | 287 | `htmy/function_component.py`: the complete `@component` decorator family | 7.3 |  | 0.539 |
| walker |  | 7060 | 40 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 25, sub: 0, line: 310 } |  |  | 0.539 |
| walker |  | 7100 | 40 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 24, sub: 0, line: 305 } |  |  | 0.546 |
| walker |  | 7146 | 46 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 22, sub: 0, line: 239 } |  |  | 0.546 |
| walker |  | 7192 | 46 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 21, sub: 0, line: 234 } |  |  | 0.554 |
| ns | 7233 |  | 174 | `htmy/md/core.py`: `MarkdownParser` | 7.4 |  | 0.547 |
| walker |  | 7245 | 53 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 14, sub: 0, line: 86 } |  |  | 0.547 |
| walker |  | 7298 | 53 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 17, sub: 0, line: 147 } |  |  | 0.547 |
| walker |  | 7351 | 53 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 20, sub: 0, line: 194 } |  |  | 0.547 |
| walker |  | 7406 | 55 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 26, sub: 0, line: 315 } |  |  | 0.547 |
| walker |  | 7473 | 67 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 23, sub: 0, line: 244 } |  |  | 0.547 |
| walker |  | 7479 | 6 | Code::CodeKey { rung: Body, file: htmy/typing.py, decl: 10, sub: 0, line: 40 } |  |  | 0.547 |
| ns | 7542 |  | 309 | `htmy/md/core.py`: the `MD` component, plus the `md/typing.py` definitions | 7.5 |  | 0.540 |
| walker |  | 7617 | 138 | Code::CodeKey { rung: Decl, file: htmy/md/core.py, decl: 1, sub: 0, line: 20 } |  |  | 0.547 |
| walker |  | 7623 | 6 | Code::CodeKey { rung: Decl, file: htmy/md/core.py, decl: 2, sub: 0, line: 33 } |  |  | 0.549 |
| walker |  | 7645 | 22 | Code::CodeKey { rung: Doc, file: htmy/i18n.py, decl: 5, sub: 0, line: 24 } |  |  | 0.549 |
| walker |  | 7789 | 144 | Code::CodeKey { rung: Decl, file: htmy/renderer/baseline.py, decl: 1, sub: 0, line: 18 } |  |  | 0.554 |
| ns | 7801 |  | 259 | `htmy/i18n.py`: `I18n`, its error hierarchy, and the resource loader | 7.6 |  | 0.563 |
| walker |  | 7844 | 55 | Code::CodeKey { rung: Decl, file: htmy/renderer/baseline.py, decl: 2, sub: 0, line: 30 } |  |  | 0.564 |
| walker |  | 7850 | 6 | Code::CodeKey { rung: Body, file: htmy/renderer/typing.py, decl: 5, sub: 0, line: 39 } |  |  | 0.564 |
| walker |  | 7896 | 46 | Code::CodeKey { rung: Decl, file: htmy/renderer/default.py, decl: 12, sub: 0, line: 205 } |  |  | 0.564 |
| walker |  | 7953 | 57 | Markdown::Section { file: README.md, section_index: 21, keeps_default_concavity: false } |  |  | 0.564 |
| ns | 8033 |  | 232 | `htmy/tag.py`: how every `html.*` tag is actually built | 7.7 |  | 0.567 |
| walker |  | 8170 | 217 | Markdown::HeadingsOutline { file: docs/index.md } |  |  | 0.567 |
| walker |  | 8176 | 6 | Code::CodeKey { rung: Body, file: htmy/typing.py, decl: 12, sub: 0, line: 48 } |  |  | 0.567 |
| ns | 8234 |  | 201 | `htmy/error_boundary.py` in outline: `ErrorBoundary` | 7.8 |  | 0.562 |
| walker |  | 8236 | 60 | Code::CodeKey { rung: Doc, file: htmy/error_boundary.py, decl: 1, sub: 0, line: 15 } |  |  | 0.569 |
| walker |  | 8257 | 21 | Code::CodeKey { rung: Doc, file: htmy/utils.py, decl: 7, sub: 0, line: 73 } |  |  | 0.569 |
| walker |  | 8312 | 55 | Code::CodeKey { rung: Decl, file: htmy/tag.py, decl: 5, sub: 0, line: 27 } |  |  | 0.569 |
| walker |  | 8344 | 32 | Code::CodeKey { rung: Doc, file: htmy/renderer/context.py, decl: 1, sub: 0, line: 6 } |  |  | 0.572 |
| walker |  | 8401 | 57 | Markdown::Section { file: docs/index.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.572 |
| ns | 8481 |  | 247 | `htmy/etree.py`: `ETreeConverter` and the optional-`lxml` import switch | 7.9 |  | 0.571 |
| walker |  | 8502 | 101 | Code::CodeKey { rung: Decl, file: htmy/md/core.py, decl: 7, sub: 0, line: 103 } |  |  | 0.577 |
| walker |  | 8529 | 27 | Markdown::HeadingsOutline { file: docs/api/md.md } |  |  | 0.577 |
| walker |  | 8550 | 21 | Markdown::Section { file: docs/api/md.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.577 |
| walker |  | 8553 | 3 | Fs::DirListing { dir: examples/internationalization/locale } |  |  | 0.577 |
| walker |  | 8588 | 35 | Code::CodeKey { rung: Body, file: htmy/error_boundary.py, decl: 2, sub: 0, line: 25 } |  |  | 0.577 |
| walker |  | 8621 | 33 | Markdown::Section { file: docs/api/core.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.577 |
| walker |  | 8654 | 33 | Markdown::Section { file: docs/api/html.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.577 |
| walker |  | 8687 | 33 | Markdown::Section { file: docs/api/utils.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.577 |
| ns | 8704 |  | 223 | `htmy/utils.py` and `htmy/io.py`: every helper, by signature | 7.10 |  | 0.582 |
| walker |  | 8757 | 70 | Markdown::Section { file: README.md, section_index: 20, keeps_default_concavity: false } |  |  | 0.582 |
| walker |  | 8791 | 34 | Markdown::Section { file: docs/api/etree.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.582 |
| walker |  | 8825 | 34 | Markdown::Section { file: docs/api/function_component.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.582 |
| ns | 8846 |  | 142 | `html.py` tag roster 1/5: document skeleton, `Link` and `Meta` factories | 8.1 |  | 0.589 |
| walker |  | 8859 | 34 | Markdown::Section { file: docs/api/snippet.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.589 |
| walker |  | 8893 | 34 | Markdown::Section { file: docs/api/typing.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.589 |
| walker |  | 8928 | 35 | Markdown::Section { file: docs/api/i18n.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.589 |
| walker |  | 8934 | 6 | Code::CodeKey { rung: Body, file: htmy/typing.py, decl: 19, sub: 0, line: 76 } |  |  | 0.590 |
| walker |  | 9009 | 75 | Code::CodeKey { rung: Decl, file: htmy/renderer/default.py, decl: 1, sub: 0, line: 20 } |  |  | 0.590 |
| ns | 9083 |  | 237 | `html.py` tag roster 2/5: sectioning and container tags | 8.2 | 8.1 | 0.580 |
| ns | 9248 |  | 165 | `html.py` tag roster 3/5: form and interactive tags | 8.3 | 8.1 | 0.573 |
| walker |  | 9514 | 505 | Code::CodeKey { rung: Decl, file: htmy/snippet.py, decl: 1, sub: 0, line: 27 } |  |  | 0.582 |
| ns | 9572 |  | 324 | `html.py` tag roster 4/5: text-level tags and embedded media | 8.4 | 8.1 | 0.568 |
| walker |  | 9580 | 66 | Code::CodeKey { rung: Decl, file: htmy/snippet.py, decl: 2, sub: 0, line: 83 } |  |  | 0.569 |
| walker |  | 9593 | 13 | Code::CodeKey { rung: Body, file: htmy/snippet.py, decl: 3, sub: 0, line: 110 } |  |  | 0.569 |
| walker |  | 9615 | 22 | Code::CodeKey { rung: Body, file: htmy/tag.py, decl: 14, sub: 0, line: 84 } |  |  | 0.569 |
| walker |  | 9621 | 6 | Code::CodeKey { rung: Body, file: htmy/typing.py, decl: 21, sub: 0, line: 84 } |  |  | 0.569 |
| walker |  | 9722 | 101 | Markdown::Section { file: README.md, section_index: 14, keeps_default_concavity: false } |  |  | 0.569 |
| walker |  | 9732 | 10 | Code::CodeKey { rung: Doc, file: htmy/html.py, decl: 11, sub: 0, line: 79 } |  |  | 0.569 |
| walker |  | 9765 | 33 | Markdown::Section { file: docs/index.md, section_index: 22, keeps_default_concavity: false } |  |  | 0.569 |
| walker |  | 9862 | 97 | Markdown::Section { file: README.md, section_index: 16, keeps_default_concavity: false } |  |  | 0.569 |
| walker |  | 9880 | 18 | Code::CodeKey { rung: Doc, file: htmy/renderer/typing.py, decl: 3, sub: 0, line: 29 } |  |  | 0.571 |
| walker |  | 9901 | 21 | Markdown::Section { file: docs/api/md.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.571 |
| ns | 9917 |  | 345 | `html.py` tag roster 5/5: lists, tables, headings, media, `entity` | 8.5 | 8.1 | 0.557 |
| walker |  | 9928 | 27 | Code::CodeKey { rung: Body, file: htmy/utils.py, decl: 3, sub: 0, line: 49 } |  |  | 0.557 |

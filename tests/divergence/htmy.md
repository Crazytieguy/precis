Score(3000)=0.739 I=0.908 C=0.601 ns_rows≤3K=19/49 grid(1000/1442/2080/3000/4327/6240/9000)=0.754/0.724/0.867/0.739/0.603/0.571/0.574

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
| walker |  | 2026 | 21 | Code::CodeKey { rung: Names, file: htmy/snippet.py, decl: 0, sub: 0, line: 0 } |  |  | 0.867 |
| ns | 2142 |  | 242 | README "Formatter": the default attribute name/value conversion rules | 2.5 |  | 0.841 |
| walker |  | 2150 | 124 | Code::CodeKey { rung: Decl, file: htmy/snippet.py, decl: 5, sub: 0, line: 158 } |  |  | 0.841 |
| walker |  | 2168 | 18 | Code::CodeKey { rung: Decl, file: htmy/snippet.py, decl: 10, sub: 0, line: 272 } |  |  | 0.841 |
| walker |  | 2236 | 68 | Code::CodeKey { rung: Decl, file: htmy/snippet.py, decl: 6, sub: 0, line: 218 } |  |  | 0.841 |
| walker |  | 2246 | 10 | Code::CodeKey { rung: Body, file: htmy/snippet.py, decl: 9, sub: 0, line: 265 } |  |  | 0.841 |
| walker |  | 2257 | 11 | Code::CodeKey { rung: Doc, file: htmy/snippet.py, decl: 7, sub: 0, line: 241 } |  |  | 0.841 |
| ns | 2302 |  | 160 | README "XSS prevention": escaping by default, and the two exceptions | 2.6 |  | 0.814 |
| walker |  | 2360 | 103 | Code::CodeKey { rung: Names, file: htmy/renderer/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.815 |
| walker |  | 2417 | 57 | Markdown::Section { file: README.md, section_index: 20, keeps_default_concavity: false } |  |  | 0.815 |
| walker |  | 2451 | 34 | Fs::DirListing { dir: docs/examples } |  |  | 0.815 |
| ns | 2506 |  | 204 | `pyproject.toml` project block: runtime deps, Python floor, optional `lxml` | 3.1 |  | 0.800 |
| walker |  | 2555 | 104 | Code::CodeKey { rung: Names, file: htmy/core.py, decl: 0, sub: 0, line: 0 } |  |  | 0.800 |
| walker |  | 2561 | 6 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 14, sub: 0, line: 157 } |  |  | 0.800 |
| walker |  | 2567 | 6 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 15, sub: 0, line: 163 } |  |  | 0.800 |
| walker |  | 2593 | 26 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 12, sub: 0, line: 146 } |  |  | 0.800 |
| walker |  | 2599 | 6 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 13, sub: 0, line: 151 } |  |  | 0.800 |
| walker |  | 2636 | 37 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 16, sub: 0, line: 175 } |  |  | 0.800 |
| ns | 2660 |  | 154 | `[tool.poe.tasks]`: the project's canonical commands | 3.2 |  | 0.760 |
| walker |  | 2696 | 60 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 1, sub: 0, line: 19 } |  |  | 0.760 |
| walker |  | 2757 | 61 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 4, sub: 0, line: 38 } |  |  | 0.761 |
| ns | 2766 |  | 106 | Test suite and CI workflow listings (complete) | 3.3 |  | 0.776 |
| walker |  | 2882 | 125 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 7, sub: 0, line: 64 } |  |  | 0.777 |
| walker |  | 2890 | 8 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 11, sub: 0, line: 127 } |  |  | 0.777 |
| ns | 2922 |  | 156 | `tests/conftest.py` in full: the three session-scoped renderer fixtures | 3.4 |  | 0.739 |
| walker |  | 3062 | 172 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 19, sub: 0, line: 201 } |  |  | 0.739 |
| ns | 3090 |  | 168 | `pyproject.toml` build backend, version source and type/test settings | 3.5 |  | 0.704 |
| walker |  | 3120 | 58 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 20, sub: 0, line: 222 } |  |  | 0.704 |
| walker |  | 3131 | 11 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 3, sub: 0, line: 33 } |  |  | 0.704 |
| walker |  | 3143 | 12 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 25, sub: 0, line: 281 } |  |  | 0.704 |
| walker |  | 3156 | 13 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 6, sub: 0, line: 56 } |  |  | 0.704 |
| walker |  | 3170 | 14 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 1, sub: 0, line: 19 } |  |  | 0.704 |
| walker |  | 3185 | 15 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 14, sub: 0, line: 157 } |  |  | 0.705 |
| walker |  | 3200 | 15 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 26, sub: 0, line: 286 } |  |  | 0.705 |
| walker |  | 3215 | 15 | Code::CodeKey { rung: Doc, file: htmy/snippet.py, decl: 10, sub: 0, line: 272 } |  |  | 0.705 |
| walker |  | 3227 | 12 | Code::CodeKey { rung: Names, file: htmy/error_boundary.py, decl: 0, sub: 0, line: 0 } |  |  | 0.705 |
| walker |  | 3303 | 76 | Code::CodeKey { rung: Decl, file: htmy/error_boundary.py, decl: 1, sub: 0, line: 15 } |  |  | 0.705 |
| ns | 3307 |  | 217 | `htmy/typing.py` lines 4-27: `T`/`U`, property types, the context types | 4.1 |  | 0.666 |
| walker |  | 3358 | 55 | Code::CodeKey { rung: Decl, file: htmy/error_boundary.py, decl: 2, sub: 0, line: 25 } |  |  | 0.666 |
| walker |  | 3374 | 16 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 21, sub: 0, line: 241 } |  |  | 0.666 |
| walker |  | 3390 | 16 | Code::CodeKey { rung: Doc, file: htmy/snippet.py, decl: 8, sub: 0, line: 256 } |  |  | 0.666 |
| walker |  | 3466 | 76 | Code::CodeKey { rung: Names, file: htmy/tag.py, decl: 0, sub: 0, line: 0 } |  |  | 0.666 |
| walker |  | 3521 | 55 | Code::CodeKey { rung: Decl, file: htmy/tag.py, decl: 4, sub: 0, line: 24 } |  |  | 0.666 |
| walker |  | 3576 | 55 | Code::CodeKey { rung: Decl, file: htmy/tag.py, decl: 5, sub: 0, line: 27 } |  |  | 0.666 |
| ns | 3621 |  | 314 | `htmy/typing.py` lines 34-67: the component protocols and the `Component` union | 4.2 | 4.1 | 0.631 |
| walker |  | 3633 | 57 | Code::CodeKey { rung: Decl, file: htmy/tag.py, decl: 14, sub: 0, line: 84 } |  |  | 0.631 |
| walker |  | 3694 | 61 | Code::CodeKey { rung: Decl, file: htmy/tag.py, decl: 8, sub: 0, line: 56 } |  |  | 0.631 |
| walker |  | 3756 | 62 | Code::CodeKey { rung: Decl, file: htmy/tag.py, decl: 1, sub: 0, line: 12 } |  |  | 0.631 |
| walker |  | 3833 | 77 | Code::CodeKey { rung: Decl, file: htmy/tag.py, decl: 11, sub: 0, line: 73 } |  |  | 0.631 |
| walker |  | 3850 | 17 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 12, sub: 0, line: 146 } |  |  | 0.632 |
| walker |  | 3920 | 70 | Markdown::Section { file: README.md, section_index: 19, keeps_default_concavity: false } |  |  | 0.632 |
| ns | 3922 |  | 301 | `htmy/typing.py` lines 70-109: context providers, `TextProcessor`, `TextResolver` | 4.3 | 4.1 | 0.595 |
| ns | 4057 |  | 135 | `htmy/renderer/__init__.py` in full: which class each renderer name resolves to | 5.1 |  | 0.595 |
| walker |  | 4256 | 336 | Code::CodeKey { rung: Names, file: htmy/typing.py, decl: 0, sub: 0, line: 0 } |  |  | 0.617 |
| walker |  | 4272 | 16 | Code::CodeKey { rung: Decl, file: htmy/typing.py, decl: 18, sub: 0, line: 73 } |  |  | 0.618 |
| walker |  | 4289 | 17 | Code::CodeKey { rung: Decl, file: htmy/typing.py, decl: 20, sub: 0, line: 81 } |  |  | 0.620 |
| ns | 4299 |  | 242 | `htmy/renderer/typing.py`: all four symbols of the renderer protocol module | 5.2 |  | 0.601 |
| walker |  | 4308 | 19 | Code::CodeKey { rung: Decl, file: htmy/typing.py, decl: 24, sub: 0, line: 103 } |  |  | 0.603 |
| walker |  | 4330 | 22 | Code::CodeKey { rung: Decl, file: htmy/typing.py, decl: 9, sub: 0, line: 37 } |  |  | 0.605 |
| walker |  | 4353 | 23 | Code::CodeKey { rung: Decl, file: htmy/typing.py, decl: 11, sub: 0, line: 45 } |  |  | 0.608 |
| walker |  | 4364 | 11 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 10, sub: 0, line: 40 } |  |  | 0.610 |
| walker |  | 4375 | 11 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 12, sub: 0, line: 48 } |  |  | 0.610 |
| walker |  | 4386 | 11 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 18, sub: 0, line: 73 } |  |  | 0.612 |
| walker |  | 4397 | 11 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 20, sub: 0, line: 81 } |  |  | 0.615 |
| walker |  | 4410 | 13 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 19, sub: 0, line: 76 } |  |  | 0.617 |
| walker |  | 4423 | 13 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 21, sub: 0, line: 84 } |  |  | 0.620 |
| walker |  | 4437 | 14 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 9, sub: 0, line: 37 } |  |  | 0.623 |
| walker |  | 4451 | 14 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 11, sub: 0, line: 45 } |  |  | 0.623 |
| walker |  | 4473 | 22 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 13, sub: 0, line: 151 } |  |  | 0.623 |
| ns | 4480 |  | 181 | `Renderer` in `htmy/renderer/default.py`: strategy docstring and `__init__` | 5.3 |  | 0.607 |
| ns | 4667 |  | 187 | `Renderer.render()` in `default.py`: context layering and renderer self-registration | 5.4 | 5.3 | 0.597 |
| ns | 4905 |  | 238 | `Renderer` in `htmy/renderer/baseline.py`: why it exists, plus its full method roster | 5.5 |  | 0.580 |
| walker |  | 4978 | 505 | Code::CodeKey { rung: Decl, file: htmy/snippet.py, decl: 1, sub: 0, line: 27 } |  |  | 0.581 |
| walker |  | 5044 | 66 | Code::CodeKey { rung: Decl, file: htmy/snippet.py, decl: 2, sub: 0, line: 83 } |  |  | 0.581 |
| ns | 5045 |  | 140 | `htmy/renderer/context.py`: `RendererContext.from_context()` | 5.6 |  | 0.569 |
| walker |  | 5067 | 23 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 4, sub: 0, line: 38 } |  |  | 0.570 |
| walker |  | 5075 | 8 | Code::CodeKey { rung: Body, file: htmy/tag.py, decl: 9, sub: 0, line: 66 } |  |  | 0.570 |
| walker |  | 5176 | 101 | Markdown::Section { file: README.md, section_index: 13, keeps_default_concavity: false } |  |  | 0.570 |
| ns | 5270 |  | 225 | `core.py`: `Fragment` and `WithContext` | 6.1 |  | 0.573 |
| walker |  | 5315 | 139 | Code::CodeKey { rung: Names, file: htmy/utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.573 |
| walker |  | 5361 | 46 | Code::CodeKey { rung: Decl, file: htmy/utils.py, decl: 1, sub: 0, line: 12 } |  |  | 0.574 |
| walker |  | 5375 | 14 | Code::CodeKey { rung: Body, file: htmy/utils.py, decl: 5, sub: 0, line: 62 } |  |  | 0.574 |
| walker |  | 5391 | 16 | Code::CodeKey { rung: Doc, file: htmy/utils.py, decl: 5, sub: 0, line: 62 } |  |  | 0.574 |
| walker |  | 5407 | 16 | Code::CodeKey { rung: Body, file: htmy/utils.py, decl: 2, sub: 0, line: 42 } |  |  | 0.574 |
| walker |  | 5424 | 17 | Code::CodeKey { rung: Doc, file: htmy/utils.py, decl: 6, sub: 0, line: 67 } |  |  | 0.574 |
| walker |  | 5442 | 18 | Code::CodeKey { rung: Doc, file: htmy/utils.py, decl: 4, sub: 0, line: 55 } |  |  | 0.574 |
| walker |  | 5463 | 21 | Code::CodeKey { rung: Doc, file: htmy/utils.py, decl: 7, sub: 0, line: 73 } |  |  | 0.574 |
| ns | 5522 |  | 252 | `core.py`: `ContextAware`, the typed context registration base class | 6.2 |  | 0.567 |
| walker |  | 5560 | 97 | Markdown::Section { file: README.md, section_index: 15, keeps_default_concavity: false } |  |  | 0.567 |
| walker |  | 5571 | 11 | Code::CodeKey { rung: Names, file: htmy/renderer/baseline.py, decl: 0, sub: 0, line: 0 } |  |  | 0.567 |
| walker |  | 5715 | 144 | Code::CodeKey { rung: Decl, file: htmy/renderer/baseline.py, decl: 1, sub: 0, line: 18 } |  |  | 0.573 |
| walker |  | 5770 | 55 | Code::CodeKey { rung: Decl, file: htmy/renderer/baseline.py, decl: 2, sub: 0, line: 30 } |  |  | 0.574 |
| walker |  | 5783 | 13 | Code::CodeKey { rung: Names, file: htmy/etree.py, decl: 0, sub: 0, line: 0 } |  |  | 0.574 |
| ns | 5795 |  | 273 | `core.py`: `SkipProperty`, `Text`, `SafeStr`, `XBool`, `xml_format_string` | 6.3 |  | 0.568 |
| walker |  | 6016 | 233 | Code::CodeKey { rung: Decl, file: htmy/etree.py, decl: 1, sub: 0, line: 23 } |  |  | 0.569 |
| walker |  | 6027 | 11 | Code::CodeKey { rung: Body, file: htmy/etree.py, decl: 2, sub: 0, line: 46 } |  |  | 0.569 |
| walker |  | 6042 | 15 | Code::CodeKey { rung: Doc, file: htmy/etree.py, decl: 7, sub: 0, line: 103 } |  |  | 0.569 |
| walker |  | 6058 | 16 | Code::CodeKey { rung: Body, file: htmy/etree.py, decl: 7, sub: 0, line: 103 } |  |  | 0.569 |
| ns | 6085 |  | 290 | `core.py`: `Formatter` -- construction and the complete method surface | 6.4 |  | 0.564 |
| walker |  | 6086 | 28 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 24, sub: 0, line: 103 } |  |  | 0.571 |
| walker |  | 6222 | 136 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.571 |
| ns | 6259 |  | 174 | `core.py`: the default value-formatter table | 6.5 | 6.4 | 0.562 |
| walker |  | 6446 | 224 | Code::CodeKey { rung: Names, file: htmy/function_component.py, decl: 0, sub: 0, line: 0 } |  |  | 0.562 |
| ns | 6452 |  | 193 | `htmy/snippet.py`: `Slots` and the slot placeholder syntax | 7.1 |  | 0.567 |
| walker |  | 6462 | 16 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 4, sub: 0, line: 31 } |  |  | 0.567 |
| walker |  | 6480 | 18 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 7, sub: 0, line: 46 } |  |  | 0.567 |
| walker |  | 6742 | 262 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 11, sub: 0, line: 71 } |  |  | 0.567 |
| ns | 6744 |  | 292 | `htmy/snippet.py`: `Snippet` and its four-step text pipeline | 7.2 |  | 0.560 |
| walker |  | 6749 | 7 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 13, sub: 0, line: 83 } |  |  | 0.560 |
| walker |  | 6756 | 7 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 16, sub: 0, line: 144 } |  |  | 0.560 |
| walker |  | 6765 | 9 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 12, sub: 0, line: 80 } |  |  | 0.560 |
| walker |  | 6774 | 9 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 15, sub: 0, line: 141 } |  |  | 0.560 |
| walker |  | 6814 | 40 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 19, sub: 0, line: 189 } |  |  | 0.560 |
| walker |  | 6854 | 40 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 18, sub: 0, line: 184 } |  |  | 0.561 |
| walker |  | 6894 | 40 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 25, sub: 0, line: 310 } |  |  | 0.561 |
| walker |  | 6934 | 40 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 24, sub: 0, line: 305 } |  |  | 0.561 |
| walker |  | 6980 | 46 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 22, sub: 0, line: 239 } |  |  | 0.561 |
| walker |  | 7026 | 46 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 21, sub: 0, line: 234 } |  |  | 0.562 |
| ns | 7031 |  | 287 | `htmy/function_component.py`: the complete `@component` decorator family | 7.3 |  | 0.575 |
| walker |  | 7079 | 53 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 14, sub: 0, line: 86 } |  |  | 0.575 |
| walker |  | 7132 | 53 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 17, sub: 0, line: 147 } |  |  | 0.575 |
| walker |  | 7185 | 53 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 20, sub: 0, line: 194 } |  |  | 0.575 |
| ns | 7205 |  | 174 | `htmy/md/core.py`: `MarkdownParser` | 7.4 |  | 0.567 |
| walker |  | 7240 | 55 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 26, sub: 0, line: 315 } |  |  | 0.567 |
| walker |  | 7307 | 67 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 23, sub: 0, line: 244 } |  |  | 0.567 |
| walker |  | 7326 | 19 | Code::CodeKey { rung: Doc, file: htmy/function_component.py, decl: 11, sub: 0, line: 71 } |  |  | 0.567 |
| walker |  | 7343 | 17 | Code::CodeKey { rung: Doc, file: htmy/etree.py, decl: 4, sub: 0, line: 63 } |  |  | 0.567 |
| walker |  | 7372 | 29 | Code::CodeKey { rung: Doc, file: htmy/utils.py, decl: 3, sub: 0, line: 49 } |  |  | 0.567 |
| ns | 7514 |  | 309 | `htmy/md/core.py`: the `MD` component, plus the `md/typing.py` definitions | 7.5 |  | 0.555 |
| walker |  | 7573 | 201 | Code::CodeKey { rung: Names, file: htmy/html.py, decl: 0, sub: 0, line: 0 } |  |  | 0.556 |
| walker |  | 7591 | 18 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 8, sub: 0, line: 63 } |  |  | 0.556 |
| walker |  | 7599 | 8 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 9, sub: 0, line: 66 } |  |  | 0.556 |
| walker |  | 7623 | 24 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 1, sub: 0, line: 13 } |  |  | 0.556 |
| walker |  | 7726 | 103 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 11, sub: 0, line: 79 } |  |  | 0.556 |
| walker |  | 7734 | 8 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 12, sub: 0, line: 82 } |  |  | 0.556 |
| walker |  | 7742 | 8 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 13, sub: 0, line: 86 } |  |  | 0.556 |
| walker |  | 7750 | 8 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 14, sub: 0, line: 90 } |  |  | 0.556 |
| walker |  | 7758 | 8 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 15, sub: 0, line: 94 } |  |  | 0.556 |
| walker |  | 7766 | 8 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 16, sub: 0, line: 98 } |  |  | 0.556 |
| ns | 7773 |  | 259 | `htmy/i18n.py`: `I18n`, its error hierarchy, and the resource loader | 7.6 |  | 0.548 |
| walker |  | 7774 | 8 | Code::CodeKey { rung: Doc, file: htmy/html.py, decl: 1, sub: 0, line: 13 } |  |  | 0.548 |
| walker |  | 7784 | 10 | Code::CodeKey { rung: Doc, file: htmy/html.py, decl: 8, sub: 0, line: 63 } |  |  | 0.548 |
| walker |  | 7794 | 10 | Code::CodeKey { rung: Doc, file: htmy/html.py, decl: 11, sub: 0, line: 79 } |  |  | 0.548 |
| walker |  | 7825 | 31 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 18, sub: 0, line: 196 } |  |  | 0.551 |
| walker |  | 7856 | 31 | Code::CodeKey { rung: Doc, file: htmy/utils.py, decl: 2, sub: 0, line: 42 } |  |  | 0.551 |
| walker |  | 7882 | 26 | Code::CodeKey { rung: Names, file: htmy/md/core.py, decl: 0, sub: 0, line: 0 } |  |  | 0.551 |
| walker |  | 7954 | 72 | Code::CodeKey { rung: Decl, file: htmy/md/core.py, decl: 6, sub: 0, line: 81 } |  |  | 0.551 |
| ns | 8005 |  | 232 | `htmy/tag.py`: how every `html.*` tag is actually built | 7.7 |  | 0.555 |
| walker |  | 8055 | 101 | Code::CodeKey { rung: Decl, file: htmy/md/core.py, decl: 7, sub: 0, line: 103 } |  |  | 0.558 |
| walker |  | 8193 | 138 | Code::CodeKey { rung: Decl, file: htmy/md/core.py, decl: 1, sub: 0, line: 20 } |  |  | 0.564 |
| walker |  | 8199 | 6 | Code::CodeKey { rung: Decl, file: htmy/md/core.py, decl: 2, sub: 0, line: 33 } |  |  | 0.565 |
| ns | 8206 |  | 201 | `htmy/error_boundary.py` in outline: `ErrorBoundary` | 7.8 |  | 0.560 |
| walker |  | 8427 | 228 | Code::CodeKey { rung: Names, file: htmy/html.py, decl: 0, sub: 1, line: 0 } |  |  | 0.561 |
| ns | 8453 |  | 247 | `htmy/etree.py`: `ETreeConverter` and the optional-`lxml` import switch | 7.9 |  | 0.560 |
| walker |  | 8656 | 229 | Code::CodeKey { rung: Names, file: htmy/html.py, decl: 0, sub: 2, line: 0 } |  |  | 0.562 |
| ns | 8676 |  | 223 | `htmy/utils.py` and `htmy/io.py`: every helper, by signature | 7.10 |  | 0.566 |
| ns | 8818 |  | 142 | `html.py` tag roster 1/5: document skeleton, `Link` and `Meta` factories | 8.1 |  | 0.573 |
| walker |  | 8861 | 205 | Code::CodeKey { rung: Names, file: htmy/html.py, decl: 0, sub: 3, line: 0 } |  |  | 0.574 |
| ns | 9055 |  | 237 | `html.py` tag roster 2/5: sectioning and container tags | 8.2 | 8.1 | 0.586 |
| walker |  | 9063 | 202 | Code::CodeKey { rung: Names, file: htmy/html.py, decl: 0, sub: 4, line: 0 } |  |  | 0.587 |
| ns | 9220 |  | 165 | `html.py` tag roster 3/5: form and interactive tags | 8.3 | 8.1 | 0.595 |
| walker |  | 9267 | 204 | Code::CodeKey { rung: Names, file: htmy/html.py, decl: 0, sub: 5, line: 0 } |  |  | 0.596 |
| walker |  | 9477 | 210 | Code::CodeKey { rung: Names, file: htmy/html.py, decl: 0, sub: 6, line: 0 } |  |  | 0.596 |
| ns | 9544 |  | 324 | `html.py` tag roster 4/5: text-level tags and embedded media | 8.4 | 8.1 | 0.610 |
| walker |  | 9624 | 147 | Code::CodeKey { rung: Names, file: htmy/html.py, decl: 0, sub: 7, line: 0 } |  |  | 0.611 |
| walker |  | 9821 | 197 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 120, sub: 0, line: 831 } |  |  | 0.612 |
| walker |  | 9856 | 35 | Code::CodeKey { rung: Body, file: htmy/error_boundary.py, decl: 2, sub: 0, line: 25 } |  |  | 0.612 |
| walker |  | 9877 | 21 | Code::CodeKey { rung: Doc, file: htmy/etree.py, decl: 3, sub: 0, line: 55 } |  |  | 0.612 |
| ns | 9889 |  | 345 | `html.py` tag roster 5/5: lists, tables, headings, media, `entity` | 8.5 | 8.1 | 0.625 |
| walker |  | 9950 | 73 | Code::CodeKey { rung: Names, file: htmy/renderer/default.py, decl: 0, sub: 0, line: 0 } |  |  | 0.625 |
| walker |  | 9996 | 46 | Code::CodeKey { rung: Decl, file: htmy/renderer/default.py, decl: 12, sub: 0, line: 205 } |  |  | 0.625 |
| walker |  | 9996 | 0 | Code::CodeKey { rung: Decl, file: htmy/renderer/default.py, decl: 13, sub: 0, line: 228 } |  |  | 0.625 |

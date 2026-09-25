Score(3000)=0.719 I=0.894 C=0.578 ns_rows≤3K=19/49 grid(1000/1442/2080/3000/4327/6240/9000)=0.842/0.659/0.616/0.719/0.588/0.534/0.582

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
| walker |  | 314 | 42 | Fs::DirListing { dir: docs/api } |  |  | 0.639 |
| walker |  | 326 | 12 | Fs::DirListing { dir: docs/api/renderer } |  |  | 0.639 |
| walker |  | 338 | 12 | Code::CodeKey { rung: Names, file: htmy/error_boundary.py, decl: 0, sub: 0, line: 0 } |  |  | 0.639 |
| ns | 349 |  | 141 | README key features, first half | 1.4 |  | 0.570 |
| walker |  | 367 | 29 | Toml::PackageMetadata { file: pyproject.toml } |  |  | 0.572 |
| ns | 504 |  | 155 | README key features, second half | 1.5 | 1.4 | 0.520 |
| walker |  | 516 | 149 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.814 |
| walker |  | 529 | 13 | Code::CodeKey { rung: Names, file: htmy/etree.py, decl: 0, sub: 0, line: 0 } |  |  | 0.814 |
| walker |  | 551 | 22 | Fs::DirListing { dir: examples } |  |  | 0.814 |
| walker |  | 644 | 93 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.820 |
| ns | 648 |  | 144 | `htmy/__init__.py` exports, part 1: version + everything from `core` | 1.6 |  | 0.735 |
| ns | 786 |  | 138 | `htmy/__init__.py` exports, part 2: `ErrorBoundary`, `component`, renderers, `Snippet`/`Slots`, tags | 1.7 | 1.6 | 0.675 |
| walker |  | 890 | 246 | Code::CodeKey { rung: Names, file: htmy/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.842 |
| walker |  | 903 | 13 | Fs::DirListing { dir: examples/internationalization } |  |  | 0.842 |
| walker |  | 991 | 88 | Code::CodeKey { rung: Names, file: htmy/md/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.842 |
| walker |  | 1012 | 21 | Code::CodeKey { rung: Names, file: htmy/snippet.py, decl: 0, sub: 0, line: 0 } |  |  | 0.842 |
| ns | 1014 |  | 228 | `htmy/__init__.py` exports, part 3: the sixteen re-exported type names | 1.8 | 1.7 | 0.751 |
| walker |  | 1026 | 14 | Fs::DirListing { dir: examples/markdown_customization } |  |  | 0.751 |
| walker |  | 1040 | 14 | Fs::DirListing { dir: examples/markdown_essentials } |  |  | 0.751 |
| walker |  | 1051 | 11 | Code::CodeKey { rung: Names, file: htmy/renderer/baseline.py, decl: 0, sub: 0, line: 0 } |  |  | 0.751 |
| walker |  | 1073 | 22 | Code::CodeKey { rung: Names, file: htmy/io.py, decl: 0, sub: 0, line: 0 } |  |  | 0.751 |
| ns | 1160 |  | 146 | `htmy/__init__.py` exports, part 4: `utils` helpers and the two aliases | 1.9 | 1.8 | 0.698 |
| walker |  | 1176 | 103 | Code::CodeKey { rung: Names, file: htmy/renderer/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.699 |
| walker |  | 1188 | 12 | Code::CodeKey { rung: Names, file: htmy/renderer/context.py, decl: 0, sub: 0, line: 0 } |  |  | 0.699 |
| walker |  | 1208 | 20 | Code::CodeKey { rung: Decl, file: htmy/renderer/context.py, decl: 1, sub: 0, line: 6 } |  |  | 0.699 |
| walker |  | 1216 | 8 | Code::CodeKey { rung: Decl, file: htmy/renderer/context.py, decl: 2, sub: 0, line: 11 } |  |  | 0.699 |
| walker |  | 1292 | 76 | Code::CodeKey { rung: Decl, file: htmy/error_boundary.py, decl: 1, sub: 0, line: 15 } |  |  | 0.699 |
| walker |  | 1348 | 56 | Markdown::HeadingsOutline { file: docs/components-guide.md } |  |  | 0.699 |
| walker |  | 1363 | 15 | Code::CodeKey { rung: Doc, file: htmy/io.py, decl: 1, sub: 0, line: 11 } |  |  | 0.699 |
| walker |  | 1422 | 59 | Markdown::HeadingsOutline { file: docs/function-components.md } |  |  | 0.699 |
| ns | 1424 |  | 264 | README "Built-in components": one line of semantics per built-in | 2.1 |  | 0.659 |
| walker |  | 1477 | 55 | Code::CodeKey { rung: Decl, file: htmy/error_boundary.py, decl: 2, sub: 0, line: 25 } |  |  | 0.659 |
| walker |  | 1536 | 59 | Fs::DirListing { dir: tests } |  |  | 0.663 |
| ns | 1612 |  | 188 | README: the definition of a component (duck-typed `htmy()` method) | 2.2 |  | 0.646 |
| walker |  | 1660 | 124 | Code::CodeKey { rung: Decl, file: htmy/snippet.py, decl: 5, sub: 0, line: 158 } |  |  | 0.647 |
| walker |  | 1678 | 18 | Code::CodeKey { rung: Decl, file: htmy/snippet.py, decl: 10, sub: 0, line: 272 } |  |  | 0.647 |
| walker |  | 1689 | 11 | Code::CodeKey { rung: Doc, file: htmy/snippet.py, decl: 7, sub: 0, line: 241 } |  |  | 0.647 |
| walker |  | 1757 | 68 | Code::CodeKey { rung: Decl, file: htmy/snippet.py, decl: 6, sub: 0, line: 218 } |  |  | 0.647 |
| ns | 1758 |  | 146 | README "Rendering": how to actually invoke the renderer | 2.3 |  | 0.626 |
| walker |  | 1785 | 28 | Fs::DirListing { dir: tests/renderer } |  |  | 0.628 |
| ns | 1900 |  | 142 | README "Context": the prop-drilling escape hatch and `htmy_context()` | 2.4 |  | 0.614 |
| walker |  | 1997 | 212 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.616 |
| ns | 2170 |  | 270 | README "Formatter": the default attribute name/value conversion rules | 2.5 |  | 0.597 |
| walker |  | 2283 | 286 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.698 |
| walker |  | 2316 | 33 | Markdown::Section { file: README.md, section_index: 23, keeps_default_concavity: false } |  |  | 0.698 |
| ns | 2330 |  | 160 | README "XSS prevention": escaping by default, and the two exceptions | 2.6 |  | 0.676 |
| walker |  | 2342 | 26 | Code::CodeKey { rung: Body, file: htmy/io.py, decl: 1, sub: 0, line: 11 } |  |  | 0.676 |
| walker |  | 2368 | 26 | Code::CodeKey { rung: Names, file: htmy/md/core.py, decl: 0, sub: 0, line: 0 } |  |  | 0.676 |
| walker |  | 2440 | 72 | Code::CodeKey { rung: Decl, file: htmy/md/core.py, decl: 6, sub: 0, line: 81 } |  |  | 0.676 |
| walker |  | 2474 | 34 | Fs::DirListing { dir: docs/examples } |  |  | 0.676 |
| ns | 2534 |  | 204 | `pyproject.toml` project block: runtime deps, Python floor, optional `lxml` | 3.1 |  | 0.694 |
| ns | 2688 |  | 154 | `[tool.poe.tasks]`: the project's canonical commands | 3.2 |  | 0.659 |
| walker |  | 2716 | 242 | Code::CodeKey { rung: Names, file: htmy/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.738 |
| walker |  | 2762 | 46 | Markdown::Section { file: README.md, section_index: 17, keeps_default_concavity: false } |  |  | 0.738 |
| ns | 2794 |  | 106 | Test suite and CI workflow listings (complete) | 3.3 |  | 0.755 |
| ns | 2950 |  | 156 | `tests/conftest.py` in full: the three session-scoped renderer fixtures | 3.4 |  | 0.718 |
| walker |  | 2995 | 233 | Code::CodeKey { rung: Decl, file: htmy/etree.py, decl: 1, sub: 0, line: 23 } |  |  | 0.719 |
| walker |  | 3006 | 11 | Code::CodeKey { rung: Body, file: htmy/etree.py, decl: 2, sub: 0, line: 46 } |  |  | 0.719 |
| walker |  | 3023 | 17 | Code::CodeKey { rung: Doc, file: htmy/etree.py, decl: 4, sub: 0, line: 63 } |  |  | 0.719 |
| walker |  | 3044 | 21 | Code::CodeKey { rung: Doc, file: htmy/etree.py, decl: 3, sub: 0, line: 55 } |  |  | 0.719 |
| ns | 3118 |  | 168 | `pyproject.toml` build backend, version source and type/test settings | 3.5 |  | 0.685 |
| walker |  | 3120 | 76 | Code::CodeKey { rung: Names, file: htmy/tag.py, decl: 0, sub: 0, line: 0 } |  |  | 0.685 |
| walker |  | 3177 | 57 | Code::CodeKey { rung: Decl, file: htmy/tag.py, decl: 14, sub: 0, line: 84 } |  |  | 0.685 |
| walker |  | 3238 | 61 | Code::CodeKey { rung: Decl, file: htmy/tag.py, decl: 8, sub: 0, line: 56 } |  |  | 0.685 |
| walker |  | 3315 | 77 | Code::CodeKey { rung: Decl, file: htmy/tag.py, decl: 11, sub: 0, line: 73 } |  |  | 0.686 |
| walker |  | 3323 | 8 | Code::CodeKey { rung: Body, file: htmy/tag.py, decl: 9, sub: 0, line: 66 } |  |  | 0.686 |
| ns | 3335 |  | 217 | `htmy/typing.py` lines 4-27: `T`/`U`, property types, the context types | 4.1 |  | 0.648 |
| walker |  | 3378 | 55 | Code::CodeKey { rung: Decl, file: htmy/tag.py, decl: 4, sub: 0, line: 24 } |  |  | 0.648 |
| walker |  | 3440 | 62 | Code::CodeKey { rung: Decl, file: htmy/tag.py, decl: 1, sub: 0, line: 12 } |  |  | 0.648 |
| walker |  | 3512 | 72 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.648 |
| ns | 3649 |  | 314 | `htmy/typing.py` lines 34-67: the component protocols and the `Component` union | 4.2 | 4.1 | 0.613 |
| walker |  | 3650 | 138 | Code::CodeKey { rung: Decl, file: htmy/md/core.py, decl: 1, sub: 0, line: 20 } |  |  | 0.614 |
| walker |  | 3656 | 6 | Code::CodeKey { rung: Decl, file: htmy/md/core.py, decl: 2, sub: 0, line: 33 } |  |  | 0.614 |
| walker |  | 3800 | 144 | Code::CodeKey { rung: Decl, file: htmy/renderer/baseline.py, decl: 1, sub: 0, line: 18 } |  |  | 0.614 |
| walker |  | 3855 | 55 | Code::CodeKey { rung: Decl, file: htmy/renderer/baseline.py, decl: 2, sub: 0, line: 30 } |  |  | 0.614 |
| walker |  | 3912 | 57 | Markdown::Section { file: README.md, section_index: 21, keeps_default_concavity: false } |  |  | 0.614 |
| ns | 3950 |  | 301 | `htmy/typing.py` lines 70-109: context providers, `TextProcessor`, `TextResolver` | 4.3 | 4.1 | 0.579 |
| walker |  | 4048 | 136 | Code::CodeKey { rung: Names, file: htmy/__init__.py, decl: 0, sub: 2, line: 0 } |  |  | 0.607 |
| ns | 4085 |  | 135 | `htmy/renderer/__init__.py` in full: which class each renderer name resolves to | 5.1 |  | 0.607 |
| walker |  | 4265 | 217 | Markdown::HeadingsOutline { file: docs/index.md } |  |  | 0.607 |
| ns | 4327 |  | 242 | `htmy/renderer/typing.py`: all four symbols of the renderer protocol module | 5.2 |  | 0.588 |
| walker |  | 4369 | 104 | Code::CodeKey { rung: Names, file: htmy/core.py, decl: 0, sub: 0, line: 0 } |  |  | 0.589 |
| walker |  | 4375 | 6 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 14, sub: 0, line: 157 } |  |  | 0.589 |
| walker |  | 4381 | 6 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 15, sub: 0, line: 163 } |  |  | 0.589 |
| walker |  | 4407 | 26 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 12, sub: 0, line: 146 } |  |  | 0.589 |
| walker |  | 4413 | 6 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 13, sub: 0, line: 151 } |  |  | 0.589 |
| walker |  | 4450 | 37 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 16, sub: 0, line: 175 } |  |  | 0.589 |
| ns | 4508 |  | 181 | `Renderer` in `htmy/renderer/default.py`: strategy docstring and `__init__` | 5.3 |  | 0.574 |
| walker |  | 4510 | 60 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 1, sub: 0, line: 19 } |  |  | 0.574 |
| walker |  | 4571 | 61 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 4, sub: 0, line: 38 } |  |  | 0.574 |
| ns | 4695 |  | 187 | `Renderer.render()` in `default.py`: context layering and renderer self-registration | 5.4 | 5.3 | 0.565 |
| walker |  | 4696 | 125 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 7, sub: 0, line: 64 } |  |  | 0.565 |
| walker |  | 4704 | 8 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 11, sub: 0, line: 127 } |  |  | 0.565 |
| walker |  | 4876 | 172 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 19, sub: 0, line: 201 } |  |  | 0.566 |
| ns | 4933 |  | 238 | `Renderer` in `htmy/renderer/baseline.py`: why it exists, plus its full method roster | 5.5 |  | 0.558 |
| walker |  | 4934 | 58 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 20, sub: 0, line: 222 } |  |  | 0.558 |
| walker |  | 4994 | 60 | Code::CodeKey { rung: Doc, file: htmy/error_boundary.py, decl: 1, sub: 0, line: 15 } |  |  | 0.559 |
| walker |  | 5049 | 55 | Code::CodeKey { rung: Decl, file: htmy/tag.py, decl: 5, sub: 0, line: 27 } |  |  | 0.559 |
| ns | 5073 |  | 140 | `htmy/renderer/context.py`: `RendererContext.from_context()` | 5.6 |  | 0.549 |
| walker |  | 5081 | 32 | Code::CodeKey { rung: Doc, file: htmy/renderer/context.py, decl: 1, sub: 0, line: 6 } |  |  | 0.554 |
| walker |  | 5138 | 57 | Markdown::Section { file: docs/index.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.554 |
| walker |  | 5239 | 101 | Code::CodeKey { rung: Decl, file: htmy/md/core.py, decl: 7, sub: 0, line: 103 } |  |  | 0.554 |
| walker |  | 5266 | 27 | Markdown::HeadingsOutline { file: docs/api/md.md } |  |  | 0.554 |
| walker |  | 5287 | 21 | Markdown::Section { file: docs/api/md.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.554 |
| walker |  | 5290 | 3 | Fs::DirListing { dir: examples/internationalization/locale } |  |  | 0.554 |
| ns | 5298 |  | 225 | `core.py`: `Fragment` and `WithContext` | 6.1 |  | 0.547 |
| walker |  | 5325 | 35 | Code::CodeKey { rung: Body, file: htmy/error_boundary.py, decl: 2, sub: 0, line: 25 } |  |  | 0.547 |
| walker |  | 5358 | 33 | Markdown::Section { file: docs/api/core.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.547 |
| walker |  | 5391 | 33 | Markdown::Section { file: docs/api/html.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.547 |
| walker |  | 5424 | 33 | Markdown::Section { file: docs/api/utils.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.547 |
| walker |  | 5494 | 70 | Markdown::Section { file: README.md, section_index: 20, keeps_default_concavity: false } |  |  | 0.547 |
| walker |  | 5528 | 34 | Markdown::Section { file: docs/api/etree.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.547 |
| ns | 5550 |  | 252 | `core.py`: `ContextAware`, the typed context registration base class | 6.2 |  | 0.541 |
| walker |  | 5562 | 34 | Markdown::Section { file: docs/api/function_component.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.541 |
| walker |  | 5596 | 34 | Markdown::Section { file: docs/api/snippet.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.541 |
| walker |  | 5630 | 34 | Markdown::Section { file: docs/api/typing.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.541 |
| walker |  | 5665 | 35 | Markdown::Section { file: docs/api/i18n.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.541 |
| walker |  | 5804 | 139 | Code::CodeKey { rung: Names, file: htmy/utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.541 |
| ns | 5823 |  | 273 | `core.py`: `SkipProperty`, `Text`, `SafeStr`, `XBool`, `xml_format_string` | 6.3 |  | 0.531 |
| walker |  | 5850 | 46 | Code::CodeKey { rung: Decl, file: htmy/utils.py, decl: 1, sub: 0, line: 12 } |  |  | 0.532 |
| walker |  | 5864 | 14 | Code::CodeKey { rung: Body, file: htmy/utils.py, decl: 5, sub: 0, line: 62 } |  |  | 0.532 |
| walker |  | 5880 | 16 | Code::CodeKey { rung: Doc, file: htmy/utils.py, decl: 5, sub: 0, line: 62 } |  |  | 0.532 |
| walker |  | 5896 | 16 | Code::CodeKey { rung: Body, file: htmy/utils.py, decl: 2, sub: 0, line: 42 } |  |  | 0.532 |
| walker |  | 5913 | 17 | Code::CodeKey { rung: Doc, file: htmy/utils.py, decl: 6, sub: 0, line: 67 } |  |  | 0.532 |
| walker |  | 5931 | 18 | Code::CodeKey { rung: Doc, file: htmy/utils.py, decl: 4, sub: 0, line: 55 } |  |  | 0.532 |
| walker |  | 5952 | 21 | Code::CodeKey { rung: Doc, file: htmy/utils.py, decl: 7, sub: 0, line: 73 } |  |  | 0.532 |
| walker |  | 6025 | 73 | Code::CodeKey { rung: Names, file: htmy/renderer/default.py, decl: 0, sub: 0, line: 0 } |  |  | 0.532 |
| walker |  | 6086 | 61 | Code::CodeKey { rung: Decl, file: htmy/renderer/default.py, decl: 13, sub: 0, line: 228 } |  |  | 0.533 |
| ns | 6113 |  | 290 | `core.py`: `Formatter` -- construction and the complete method surface | 6.4 |  | 0.530 |
| walker |  | 6141 | 55 | Code::CodeKey { rung: Decl, file: htmy/renderer/default.py, decl: 14, sub: 0, line: 238 } |  |  | 0.534 |
| walker |  | 6187 | 46 | Code::CodeKey { rung: Decl, file: htmy/renderer/default.py, decl: 12, sub: 0, line: 205 } |  |  | 0.534 |
| ns | 6287 |  | 174 | `core.py`: the default value-formatter table | 6.5 | 6.4 | 0.526 |
| walker |  | 6332 | 145 | Code::CodeKey { rung: Names, file: htmy/i18n.py, decl: 0, sub: 0, line: 0 } |  |  | 0.526 |
| walker |  | 6341 | 9 | Code::CodeKey { rung: Decl, file: htmy/i18n.py, decl: 11, sub: 0, line: 115 } |  |  | 0.526 |
| ns | 6480 |  | 193 | `htmy/snippet.py`: `Slots` and the slot placeholder syntax | 7.1 |  | 0.518 |
| walker |  | 6571 | 230 | Code::CodeKey { rung: Decl, file: htmy/i18n.py, decl: 5, sub: 0, line: 24 } |  |  | 0.519 |
| walker |  | 6578 | 7 | Code::CodeKey { rung: Decl, file: htmy/i18n.py, decl: 8, sub: 0, line: 48 } |  |  | 0.519 |
| walker |  | 6587 | 9 | Code::CodeKey { rung: Decl, file: htmy/i18n.py, decl: 7, sub: 0, line: 45 } |  |  | 0.519 |
| walker |  | 6595 | 8 | Code::CodeKey { rung: Decl, file: htmy/i18n.py, decl: 10, sub: 0, line: 79 } |  |  | 0.519 |
| walker |  | 6617 | 22 | Code::CodeKey { rung: Doc, file: htmy/i18n.py, decl: 5, sub: 0, line: 24 } |  |  | 0.519 |
| walker |  | 6694 | 77 | Code::CodeKey { rung: Names, file: htmy/renderer/typing.py, decl: 0, sub: 0, line: 0 } |  |  | 0.521 |
| walker |  | 6721 | 27 | Code::CodeKey { rung: Decl, file: htmy/renderer/typing.py, decl: 1, sub: 0, line: 12 } |  |  | 0.522 |
| walker |  | 6749 | 28 | Code::CodeKey { rung: Decl, file: htmy/renderer/typing.py, decl: 4, sub: 0, line: 36 } |  |  | 0.524 |
| walker |  | 6759 | 10 | Code::CodeKey { rung: Doc, file: htmy/renderer/typing.py, decl: 1, sub: 0, line: 12 } |  |  | 0.526 |
| walker |  | 6770 | 11 | Code::CodeKey { rung: Doc, file: htmy/renderer/typing.py, decl: 4, sub: 0, line: 36 } |  |  | 0.528 |
| ns | 6772 |  | 292 | `htmy/snippet.py`: `Snippet` and its four-step text pipeline | 7.2 |  | 0.522 |
| walker |  | 6776 | 6 | Code::CodeKey { rung: Body, file: htmy/renderer/typing.py, decl: 2, sub: 0, line: 15 } |  |  | 0.522 |
| walker |  | 6782 | 6 | Code::CodeKey { rung: Body, file: htmy/renderer/typing.py, decl: 5, sub: 0, line: 39 } |  |  | 0.522 |
| walker |  | 6857 | 75 | Code::CodeKey { rung: Decl, file: htmy/renderer/default.py, decl: 1, sub: 0, line: 20 } |  |  | 0.522 |
| ns | 7059 |  | 287 | `htmy/function_component.py`: the complete `@component` decorator family | 7.3 |  | 0.511 |
| ns | 7233 |  | 174 | `htmy/md/core.py`: `MarkdownParser` | 7.4 |  | 0.513 |
| walker |  | 7362 | 505 | Code::CodeKey { rung: Decl, file: htmy/snippet.py, decl: 1, sub: 0, line: 27 } |  |  | 0.525 |
| walker |  | 7428 | 66 | Code::CodeKey { rung: Decl, file: htmy/snippet.py, decl: 2, sub: 0, line: 83 } |  |  | 0.526 |
| walker |  | 7441 | 13 | Code::CodeKey { rung: Body, file: htmy/snippet.py, decl: 3, sub: 0, line: 110 } |  |  | 0.526 |
| walker |  | 7463 | 22 | Code::CodeKey { rung: Body, file: htmy/tag.py, decl: 14, sub: 0, line: 84 } |  |  | 0.526 |
| ns | 7542 |  | 309 | `htmy/md/core.py`: the `MD` component, plus the `md/typing.py` definitions | 7.5 |  | 0.518 |
| walker |  | 7549 | 86 | Code::CodeKey { rung: Names, file: htmy/md/typing.py, decl: 0, sub: 0, line: 0 } |  |  | 0.524 |
| walker |  | 7574 | 25 | Code::CodeKey { rung: Decl, file: htmy/md/typing.py, decl: 2, sub: 0, line: 14 } |  |  | 0.525 |
| walker |  | 7585 | 11 | Code::CodeKey { rung: Doc, file: htmy/md/typing.py, decl: 2, sub: 0, line: 14 } |  |  | 0.527 |
| walker |  | 7686 | 101 | Markdown::Section { file: README.md, section_index: 14, keeps_default_concavity: false } |  |  | 0.527 |
| walker |  | 7719 | 33 | Markdown::Section { file: docs/index.md, section_index: 22, keeps_default_concavity: false } |  |  | 0.527 |
| ns | 7801 |  | 259 | `htmy/i18n.py`: `I18n`, its error hierarchy, and the resource loader | 7.6 |  | 0.537 |
| walker |  | 7816 | 97 | Markdown::Section { file: README.md, section_index: 16, keeps_default_concavity: false } |  |  | 0.537 |
| walker |  | 7834 | 18 | Code::CodeKey { rung: Doc, file: htmy/renderer/typing.py, decl: 3, sub: 0, line: 29 } |  |  | 0.539 |
| ns | 8033 |  | 232 | `htmy/tag.py`: how every `html.*` tag is actually built | 7.7 |  | 0.543 |
| walker |  | 8170 | 336 | Code::CodeKey { rung: Names, file: htmy/typing.py, decl: 0, sub: 0, line: 0 } |  |  | 0.556 |
| walker |  | 8186 | 16 | Code::CodeKey { rung: Decl, file: htmy/typing.py, decl: 18, sub: 0, line: 73 } |  |  | 0.557 |
| walker |  | 8203 | 17 | Code::CodeKey { rung: Decl, file: htmy/typing.py, decl: 20, sub: 0, line: 81 } |  |  | 0.558 |
| walker |  | 8222 | 19 | Code::CodeKey { rung: Decl, file: htmy/typing.py, decl: 24, sub: 0, line: 103 } |  |  | 0.559 |
| ns | 8234 |  | 201 | `htmy/error_boundary.py` in outline: `ErrorBoundary` | 7.8 |  | 0.560 |
| walker |  | 8244 | 22 | Code::CodeKey { rung: Decl, file: htmy/typing.py, decl: 9, sub: 0, line: 37 } |  |  | 0.562 |
| walker |  | 8267 | 23 | Code::CodeKey { rung: Decl, file: htmy/typing.py, decl: 11, sub: 0, line: 45 } |  |  | 0.563 |
| walker |  | 8278 | 11 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 18, sub: 0, line: 73 } |  |  | 0.564 |
| walker |  | 8289 | 11 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 20, sub: 0, line: 81 } |  |  | 0.566 |
| walker |  | 8295 | 6 | Code::CodeKey { rung: Body, file: htmy/typing.py, decl: 10, sub: 0, line: 40 } |  |  | 0.567 |
| walker |  | 8301 | 6 | Code::CodeKey { rung: Body, file: htmy/typing.py, decl: 12, sub: 0, line: 48 } |  |  | 0.567 |
| walker |  | 8307 | 6 | Code::CodeKey { rung: Body, file: htmy/typing.py, decl: 19, sub: 0, line: 76 } |  |  | 0.568 |
| walker |  | 8313 | 6 | Code::CodeKey { rung: Body, file: htmy/typing.py, decl: 21, sub: 0, line: 84 } |  |  | 0.569 |
| walker |  | 8334 | 21 | Markdown::Section { file: docs/api/md.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.569 |
| walker |  | 8361 | 27 | Code::CodeKey { rung: Body, file: htmy/utils.py, decl: 3, sub: 0, line: 49 } |  |  | 0.569 |
| ns | 8481 |  | 247 | `htmy/etree.py`: `ETreeConverter` and the optional-`lxml` import switch | 7.9 |  | 0.568 |
| walker |  | 8497 | 136 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.568 |
| ns | 8704 |  | 223 | `htmy/utils.py` and `htmy/io.py`: every helper, by signature | 7.10 |  | 0.573 |
| walker |  | 8758 | 261 | Code::CodeKey { rung: Names, file: htmy/html.py, decl: 0, sub: 0, line: 0 } |  |  | 0.574 |
| walker |  | 8776 | 18 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 8, sub: 0, line: 63 } |  |  | 0.574 |
| walker |  | 8784 | 8 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 9, sub: 0, line: 66 } |  |  | 0.574 |
| walker |  | 8808 | 24 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 1, sub: 0, line: 13 } |  |  | 0.574 |
| walker |  | 8816 | 8 | Code::CodeKey { rung: Doc, file: htmy/html.py, decl: 1, sub: 0, line: 13 } |  |  | 0.574 |
| walker |  | 8826 | 10 | Code::CodeKey { rung: Doc, file: htmy/html.py, decl: 8, sub: 0, line: 63 } |  |  | 0.574 |
| ns | 8846 |  | 142 | `html.py` tag roster 1/5: document skeleton, `Link` and `Meta` factories | 8.1 |  | 0.582 |
| walker |  | 8929 | 103 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 11, sub: 0, line: 79 } |  |  | 0.582 |
| walker |  | 8937 | 8 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 12, sub: 0, line: 82 } |  |  | 0.582 |
| walker |  | 8945 | 8 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 13, sub: 0, line: 86 } |  |  | 0.582 |
| walker |  | 8953 | 8 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 14, sub: 0, line: 90 } |  |  | 0.582 |
| walker |  | 8961 | 8 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 15, sub: 0, line: 94 } |  |  | 0.582 |
| walker |  | 8969 | 8 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 16, sub: 0, line: 98 } |  |  | 0.582 |
| walker |  | 8979 | 10 | Code::CodeKey { rung: Doc, file: htmy/html.py, decl: 11, sub: 0, line: 79 } |  |  | 0.582 |
| walker |  | 8982 | 3 | Fs::DirListing { dir: examples/internationalization/locale/en } |  |  | 0.582 |
| walker |  | 8988 | 6 | Code::CodeKey { rung: Body, file: htmy/typing.py, decl: 25, sub: 0, line: 108 } |  |  | 0.582 |
| walker |  | 9008 | 20 | Code::CodeKey { rung: Body, file: htmy/md/core.py, decl: 3, sub: 0, line: 43 } |  |  | 0.582 |
| walker |  | 9051 | 43 | Code::CodeKey { rung: Doc, file: htmy/etree.py, decl: 2, sub: 0, line: 46 } |  |  | 0.582 |
| ns | 9083 |  | 237 | `html.py` tag roster 2/5: sectioning and container tags | 8.2 | 8.1 | 0.572 |
| walker |  | 9085 | 34 | Markdown::Section { file: docs/api/renderer/default.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.572 |
| ns | 9248 |  | 165 | `html.py` tag roster 3/5: form and interactive tags | 8.3 | 8.1 | 0.565 |
| walker |  | 9309 | 224 | Code::CodeKey { rung: Names, file: htmy/function_component.py, decl: 0, sub: 0, line: 0 } |  |  | 0.565 |
| walker |  | 9325 | 16 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 4, sub: 0, line: 31 } |  |  | 0.565 |
| walker |  | 9343 | 18 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 7, sub: 0, line: 46 } |  |  | 0.565 |
| ns | 9572 |  | 324 | `html.py` tag roster 4/5: text-level tags and embedded media | 8.4 | 8.1 | 0.551 |
| walker |  | 9605 | 262 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 11, sub: 0, line: 71 } |  |  | 0.555 |
| walker |  | 9612 | 7 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 13, sub: 0, line: 83 } |  |  | 0.555 |
| walker |  | 9619 | 7 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 16, sub: 0, line: 144 } |  |  | 0.555 |
| walker |  | 9628 | 9 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 12, sub: 0, line: 80 } |  |  | 0.556 |
| walker |  | 9637 | 9 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 15, sub: 0, line: 141 } |  |  | 0.557 |
| walker |  | 9677 | 40 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 19, sub: 0, line: 189 } |  |  | 0.557 |
| walker |  | 9717 | 40 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 18, sub: 0, line: 184 } |  |  | 0.561 |
| walker |  | 9757 | 40 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 25, sub: 0, line: 310 } |  |  | 0.561 |
| walker |  | 9797 | 40 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 24, sub: 0, line: 305 } |  |  | 0.565 |
| walker |  | 9843 | 46 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 22, sub: 0, line: 239 } |  |  | 0.565 |
| walker |  | 9889 | 46 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 21, sub: 0, line: 234 } |  |  | 0.571 |
| ns | 9917 |  | 345 | `html.py` tag roster 5/5: lists, tables, headings, media, `entity` | 8.5 | 8.1 | 0.557 |
| walker |  | 9942 | 53 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 14, sub: 0, line: 86 } |  |  | 0.557 |
| walker |  | 9995 | 53 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 17, sub: 0, line: 147 } |  |  | 0.557 |

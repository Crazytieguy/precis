Score(3000)=0.753 I=0.909 C=0.624 ns_rows≤3K=19/49 grid(1000/1442/2080/3000/4327/6240/9000)=0.675/0.770/0.774/0.753/0.590/0.546/0.527

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
| ns | 349 |  | 141 | README key features, first half | 1.4 |  | 0.570 |
| walker |  | 355 | 29 | Toml::PackageMetadata { file: pyproject.toml } |  |  | 0.572 |
| walker |  | 504 | 149 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.814 |
| ns | 504 |  | 155 | README key features, second half | 1.5 | 1.4 | 0.814 |
| walker |  | 526 | 22 | Fs::DirListing { dir: examples } |  |  | 0.814 |
| walker |  | 619 | 93 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.820 |
| walker |  | 632 | 13 | Fs::DirListing { dir: examples/internationalization } |  |  | 0.820 |
| walker |  | 646 | 14 | Fs::DirListing { dir: examples/markdown_customization } |  |  | 0.820 |
| ns | 648 |  | 144 | `htmy/__init__.py` exports, part 1: version + everything from `core` | 1.6 |  | 0.735 |
| walker |  | 660 | 14 | Fs::DirListing { dir: examples/markdown_essentials } |  |  | 0.735 |
| walker |  | 716 | 56 | Markdown::HeadingsOutline { file: docs/components-guide.md } |  |  | 0.735 |
| walker |  | 775 | 59 | Markdown::HeadingsOutline { file: docs/function-components.md } |  |  | 0.735 |
| ns | 786 |  | 138 | `htmy/__init__.py` exports, part 2: `ErrorBoundary`, `component`, renderers, `Snippet`/`Slots`, tags | 1.7 | 1.6 | 0.675 |
| ns | 1014 |  | 228 | `htmy/__init__.py` exports, part 3: the sixteen re-exported type names | 1.8 | 1.7 | 0.602 |
| walker |  | 1021 | 246 | Code::CodeKey { rung: Names, file: htmy/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.751 |
| ns | 1160 |  | 146 | `htmy/__init__.py` exports, part 4: `utils` helpers and the two aliases | 1.9 | 1.8 | 0.698 |
| walker |  | 1263 | 242 | Code::CodeKey { rung: Names, file: htmy/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.818 |
| walker |  | 1351 | 88 | Code::CodeKey { rung: Names, file: htmy/md/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.818 |
| ns | 1424 |  | 264 | README "Built-in components": one line of semantics per built-in | 2.1 |  | 0.770 |
| walker |  | 1454 | 103 | Code::CodeKey { rung: Names, file: htmy/renderer/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.772 |
| walker |  | 1590 | 136 | Code::CodeKey { rung: Names, file: htmy/__init__.py, decl: 0, sub: 2, line: 0 } |  |  | 0.831 |
| ns | 1612 |  | 188 | README: the definition of a component (duck-typed `htmy()` method) | 2.2 |  | 0.810 |
| walker |  | 1649 | 59 | Fs::DirListing { dir: tests } |  |  | 0.815 |
| walker |  | 1677 | 28 | Fs::DirListing { dir: tests/renderer } |  |  | 0.818 |
| ns | 1758 |  | 146 | README "Rendering": how to actually invoke the renderer | 2.3 |  | 0.791 |
| walker |  | 1889 | 212 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.792 |
| ns | 1900 |  | 142 | README "Context": the prop-drilling escape hatch and `htmy_context()` | 2.4 |  | 0.774 |
| ns | 2170 |  | 270 | README "Formatter": the default attribute name/value conversion rules | 2.5 |  | 0.751 |
| walker |  | 2175 | 286 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.845 |
| walker |  | 2208 | 33 | Markdown::Section { file: README.md, section_index: 23, keeps_default_concavity: false } |  |  | 0.845 |
| walker |  | 2242 | 34 | Fs::DirListing { dir: docs/examples } |  |  | 0.845 |
| walker |  | 2288 | 46 | Markdown::Section { file: README.md, section_index: 17, keeps_default_concavity: false } |  |  | 0.845 |
| ns | 2330 |  | 160 | README "XSS prevention": escaping by default, and the two exceptions | 2.6 |  | 0.818 |
| walker |  | 2360 | 72 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.818 |
| walker |  | 2417 | 57 | Markdown::Section { file: README.md, section_index: 21, keeps_default_concavity: false } |  |  | 0.818 |
| ns | 2534 |  | 204 | `pyproject.toml` project block: runtime deps, Python floor, optional `lxml` | 3.1 |  | 0.820 |
| walker |  | 2634 | 217 | Markdown::HeadingsOutline { file: docs/index.md } |  |  | 0.820 |
| ns | 2688 |  | 154 | `[tool.poe.tasks]`: the project's canonical commands | 3.2 |  | 0.779 |
| walker |  | 2691 | 57 | Markdown::Section { file: docs/index.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.779 |
| walker |  | 2712 | 21 | Code::CodeKey { rung: Names, file: htmy/snippet.py, decl: 0, sub: 0, line: 0 } |  |  | 0.779 |
| ns | 2794 |  | 106 | Test suite and CI workflow listings (complete) | 3.3 |  | 0.792 |
| walker |  | 2836 | 124 | Code::CodeKey { rung: Decl, file: htmy/snippet.py, decl: 5, sub: 0, line: 158 } |  |  | 0.792 |
| walker |  | 2854 | 18 | Code::CodeKey { rung: Decl, file: htmy/snippet.py, decl: 10, sub: 0, line: 272 } |  |  | 0.792 |
| walker |  | 2865 | 11 | Code::CodeKey { rung: Doc, file: htmy/snippet.py, decl: 7, sub: 0, line: 241 } |  |  | 0.792 |
| walker |  | 2933 | 68 | Code::CodeKey { rung: Decl, file: htmy/snippet.py, decl: 6, sub: 0, line: 218 } |  |  | 0.792 |
| walker |  | 2948 | 15 | Code::CodeKey { rung: Doc, file: htmy/snippet.py, decl: 10, sub: 0, line: 272 } |  |  | 0.792 |
| ns | 2950 |  | 156 | `tests/conftest.py` in full: the three session-scoped renderer fixtures | 3.4 |  | 0.753 |
| walker |  | 2964 | 16 | Code::CodeKey { rung: Doc, file: htmy/snippet.py, decl: 8, sub: 0, line: 256 } |  |  | 0.753 |
| walker |  | 2991 | 27 | Markdown::HeadingsOutline { file: docs/api/md.md } |  |  | 0.753 |
| walker |  | 3012 | 21 | Markdown::Section { file: docs/api/md.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.753 |
| walker |  | 3015 | 3 | Fs::DirListing { dir: examples/internationalization/locale } |  |  | 0.753 |
| ns | 3118 |  | 168 | `pyproject.toml` build backend, version source and type/test settings | 3.5 |  | 0.717 |
| walker |  | 3119 | 104 | Code::CodeKey { rung: Names, file: htmy/core.py, decl: 0, sub: 0, line: 0 } |  |  | 0.717 |
| walker |  | 3125 | 6 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 14, sub: 0, line: 157 } |  |  | 0.717 |
| walker |  | 3131 | 6 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 15, sub: 0, line: 163 } |  |  | 0.717 |
| walker |  | 3157 | 26 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 12, sub: 0, line: 146 } |  |  | 0.718 |
| walker |  | 3163 | 6 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 13, sub: 0, line: 151 } |  |  | 0.718 |
| walker |  | 3200 | 37 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 16, sub: 0, line: 175 } |  |  | 0.718 |
| walker |  | 3260 | 60 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 1, sub: 0, line: 19 } |  |  | 0.718 |
| walker |  | 3321 | 61 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 4, sub: 0, line: 38 } |  |  | 0.718 |
| walker |  | 3332 | 11 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 3, sub: 0, line: 33 } |  |  | 0.719 |
| ns | 3335 |  | 217 | `htmy/typing.py` lines 4-27: `T`/`U`, property types, the context types | 4.1 |  | 0.679 |
| walker |  | 3345 | 13 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 6, sub: 0, line: 56 } |  |  | 0.679 |
| walker |  | 3359 | 14 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 1, sub: 0, line: 19 } |  |  | 0.680 |
| walker |  | 3374 | 15 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 14, sub: 0, line: 157 } |  |  | 0.680 |
| walker |  | 3391 | 17 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 12, sub: 0, line: 146 } |  |  | 0.680 |
| walker |  | 3516 | 125 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 7, sub: 0, line: 64 } |  |  | 0.680 |
| walker |  | 3524 | 8 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 11, sub: 0, line: 127 } |  |  | 0.681 |
| walker |  | 3546 | 22 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 13, sub: 0, line: 151 } |  |  | 0.681 |
| walker |  | 3569 | 23 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 4, sub: 0, line: 38 } |  |  | 0.681 |
| ns | 3649 |  | 314 | `htmy/typing.py` lines 34-67: the component protocols and the `Component` union | 4.2 | 4.1 | 0.644 |
| walker |  | 3741 | 172 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 19, sub: 0, line: 201 } |  |  | 0.645 |
| walker |  | 3799 | 58 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 20, sub: 0, line: 222 } |  |  | 0.645 |
| walker |  | 3815 | 16 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 21, sub: 0, line: 241 } |  |  | 0.645 |
| walker |  | 3846 | 31 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 18, sub: 0, line: 196 } |  |  | 0.645 |
| walker |  | 3883 | 37 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 17, sub: 0, line: 186 } |  |  | 0.646 |
| walker |  | 3923 | 40 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 9, sub: 0, line: 108 } |  |  | 0.646 |
| walker |  | 3935 | 12 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 25, sub: 0, line: 281 } |  |  | 0.646 |
| ns | 3950 |  | 301 | `htmy/typing.py` lines 70-109: context providers, `TextProcessor`, `TextResolver` | 4.3 | 4.1 | 0.608 |
| walker |  | 3978 | 43 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 2, sub: 0, line: 24 } |  |  | 0.608 |
| walker |  | 4027 | 49 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 10, sub: 0, line: 115 } |  |  | 0.608 |
| walker |  | 4076 | 49 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 23, sub: 0, line: 259 } |  |  | 0.608 |
| ns | 4085 |  | 135 | `htmy/renderer/__init__.py` in full: which class each renderer name resolves to | 5.1 |  | 0.608 |
| walker |  | 4091 | 15 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 26, sub: 0, line: 286 } |  |  | 0.608 |
| walker |  | 4156 | 65 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 5, sub: 0, line: 45 } |  |  | 0.608 |
| walker |  | 4223 | 67 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 22, sub: 0, line: 246 } |  |  | 0.608 |
| walker |  | 4291 | 68 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 16, sub: 0, line: 175 } |  |  | 0.608 |
| walker |  | 4324 | 33 | Markdown::Section { file: docs/api/core.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.608 |
| ns | 4327 |  | 242 | `htmy/renderer/typing.py`: all four symbols of the renderer protocol module | 5.2 |  | 0.590 |
| walker |  | 4357 | 33 | Markdown::Section { file: docs/api/html.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.590 |
| walker |  | 4390 | 33 | Markdown::Section { file: docs/api/utils.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.590 |
| walker |  | 4460 | 70 | Markdown::Section { file: README.md, section_index: 20, keeps_default_concavity: false } |  |  | 0.590 |
| walker |  | 4494 | 34 | Markdown::Section { file: docs/api/etree.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.590 |
| ns | 4508 |  | 181 | `Renderer` in `htmy/renderer/default.py`: strategy docstring and `__init__` | 5.3 |  | 0.575 |
| walker |  | 4528 | 34 | Markdown::Section { file: docs/api/function_component.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.575 |
| walker |  | 4562 | 34 | Markdown::Section { file: docs/api/snippet.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.575 |
| walker |  | 4596 | 34 | Markdown::Section { file: docs/api/typing.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.575 |
| walker |  | 4608 | 12 | Code::CodeKey { rung: Names, file: htmy/error_boundary.py, decl: 0, sub: 0, line: 0 } |  |  | 0.575 |
| walker |  | 4684 | 76 | Code::CodeKey { rung: Decl, file: htmy/error_boundary.py, decl: 1, sub: 0, line: 15 } |  |  | 0.575 |
| ns | 4695 |  | 187 | `Renderer.render()` in `default.py`: context layering and renderer self-registration | 5.4 | 5.3 | 0.565 |
| walker |  | 4739 | 55 | Code::CodeKey { rung: Decl, file: htmy/error_boundary.py, decl: 2, sub: 0, line: 25 } |  |  | 0.565 |
| walker |  | 4774 | 35 | Code::CodeKey { rung: Body, file: htmy/error_boundary.py, decl: 2, sub: 0, line: 25 } |  |  | 0.565 |
| walker |  | 4834 | 60 | Code::CodeKey { rung: Doc, file: htmy/error_boundary.py, decl: 1, sub: 0, line: 15 } |  |  | 0.566 |
| walker |  | 4908 | 74 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 24, sub: 0, line: 268 } |  |  | 0.566 |
| ns | 4933 |  | 238 | `Renderer` in `htmy/renderer/baseline.py`: why it exists, plus its full method roster | 5.5 |  | 0.550 |
| walker |  | 4943 | 35 | Markdown::Section { file: docs/api/i18n.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.550 |
| walker |  | 5019 | 76 | Code::CodeKey { rung: Names, file: htmy/tag.py, decl: 0, sub: 0, line: 0 } |  |  | 0.550 |
| ns | 5073 |  | 140 | `htmy/renderer/context.py`: `RendererContext.from_context()` | 5.6 |  | 0.539 |
| walker |  | 5076 | 57 | Code::CodeKey { rung: Decl, file: htmy/tag.py, decl: 14, sub: 0, line: 84 } |  |  | 0.539 |
| walker |  | 5137 | 61 | Code::CodeKey { rung: Decl, file: htmy/tag.py, decl: 8, sub: 0, line: 56 } |  |  | 0.539 |
| walker |  | 5145 | 8 | Code::CodeKey { rung: Body, file: htmy/tag.py, decl: 9, sub: 0, line: 66 } |  |  | 0.539 |
| walker |  | 5222 | 77 | Code::CodeKey { rung: Decl, file: htmy/tag.py, decl: 11, sub: 0, line: 73 } |  |  | 0.540 |
| walker |  | 5236 | 14 | Code::CodeKey { rung: Body, file: htmy/tag.py, decl: 10, sub: 0, line: 69 } |  |  | 0.540 |
| walker |  | 5291 | 55 | Code::CodeKey { rung: Decl, file: htmy/tag.py, decl: 4, sub: 0, line: 24 } |  |  | 0.540 |
| ns | 5298 |  | 225 | `core.py`: `Fragment` and `WithContext` | 6.1 |  | 0.548 |
| walker |  | 5346 | 55 | Code::CodeKey { rung: Decl, file: htmy/tag.py, decl: 5, sub: 0, line: 27 } |  |  | 0.548 |
| walker |  | 5408 | 62 | Code::CodeKey { rung: Decl, file: htmy/tag.py, decl: 1, sub: 0, line: 12 } |  |  | 0.548 |
| walker |  | 5459 | 51 | Code::CodeKey { rung: Doc, file: htmy/tag.py, decl: 8, sub: 0, line: 56 } |  |  | 0.548 |
| walker |  | 5536 | 77 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 11, sub: 0, line: 127 } |  |  | 0.548 |
| walker |  | 5549 | 13 | Code::CodeKey { rung: Names, file: htmy/etree.py, decl: 0, sub: 0, line: 0 } |  |  | 0.548 |
| ns | 5550 |  | 252 | `core.py`: `ContextAware`, the typed context registration base class | 6.2 |  | 0.541 |
| walker |  | 5782 | 233 | Code::CodeKey { rung: Decl, file: htmy/etree.py, decl: 1, sub: 0, line: 23 } |  |  | 0.542 |
| walker |  | 5793 | 11 | Code::CodeKey { rung: Body, file: htmy/etree.py, decl: 2, sub: 0, line: 46 } |  |  | 0.542 |
| walker |  | 5810 | 17 | Code::CodeKey { rung: Doc, file: htmy/etree.py, decl: 4, sub: 0, line: 63 } |  |  | 0.542 |
| ns | 5823 |  | 273 | `core.py`: `SkipProperty`, `Text`, `SafeStr`, `XBool`, `xml_format_string` | 6.3 |  | 0.549 |
| walker |  | 5831 | 21 | Code::CodeKey { rung: Doc, file: htmy/etree.py, decl: 3, sub: 0, line: 55 } |  |  | 0.549 |
| walker |  | 5874 | 43 | Code::CodeKey { rung: Doc, file: htmy/etree.py, decl: 2, sub: 0, line: 46 } |  |  | 0.549 |
| walker |  | 5889 | 15 | Code::CodeKey { rung: Doc, file: htmy/etree.py, decl: 7, sub: 0, line: 103 } |  |  | 0.549 |
| walker |  | 5972 | 83 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 20, sub: 0, line: 222 } |  |  | 0.549 |
| ns | 6113 |  | 290 | `core.py`: `Formatter` -- construction and the complete method surface | 6.4 |  | 0.545 |
| walker |  | 6233 | 261 | Code::CodeKey { rung: Names, file: htmy/html.py, decl: 0, sub: 0, line: 0 } |  |  | 0.546 |
| walker |  | 6251 | 18 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 8, sub: 0, line: 63 } |  |  | 0.546 |
| walker |  | 6259 | 8 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 9, sub: 0, line: 66 } |  |  | 0.546 |
| walker |  | 6283 | 24 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 1, sub: 0, line: 13 } |  |  | 0.546 |
| ns | 6287 |  | 174 | `core.py`: the default value-formatter table | 6.5 | 6.4 | 0.538 |
| walker |  | 6291 | 8 | Code::CodeKey { rung: Doc, file: htmy/html.py, decl: 1, sub: 0, line: 13 } |  |  | 0.538 |
| walker |  | 6301 | 10 | Code::CodeKey { rung: Doc, file: htmy/html.py, decl: 8, sub: 0, line: 63 } |  |  | 0.538 |
| walker |  | 6404 | 103 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 11, sub: 0, line: 79 } |  |  | 0.538 |
| walker |  | 6412 | 8 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 12, sub: 0, line: 82 } |  |  | 0.538 |
| walker |  | 6420 | 8 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 13, sub: 0, line: 86 } |  |  | 0.538 |
| walker |  | 6428 | 8 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 14, sub: 0, line: 90 } |  |  | 0.538 |
| walker |  | 6436 | 8 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 15, sub: 0, line: 94 } |  |  | 0.538 |
| walker |  | 6444 | 8 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 16, sub: 0, line: 98 } |  |  | 0.538 |
| walker |  | 6454 | 10 | Code::CodeKey { rung: Doc, file: htmy/html.py, decl: 11, sub: 0, line: 79 } |  |  | 0.538 |
| ns | 6480 |  | 193 | `htmy/snippet.py`: `Slots` and the slot placeholder syntax | 7.1 |  | 0.530 |
| ns | 6772 |  | 292 | `htmy/snippet.py`: `Snippet` and its four-step text pipeline | 7.2 |  | 0.525 |
| walker |  | 6959 | 505 | Code::CodeKey { rung: Decl, file: htmy/snippet.py, decl: 1, sub: 0, line: 27 } |  |  | 0.537 |
| walker |  | 7025 | 66 | Code::CodeKey { rung: Decl, file: htmy/snippet.py, decl: 2, sub: 0, line: 83 } |  |  | 0.538 |
| walker |  | 7038 | 13 | Code::CodeKey { rung: Body, file: htmy/snippet.py, decl: 3, sub: 0, line: 110 } |  |  | 0.538 |
| ns | 7059 |  | 287 | `htmy/function_component.py`: the complete `@component` decorator family | 7.3 |  | 0.527 |
| ns | 7233 |  | 174 | `htmy/md/core.py`: `MarkdownParser` | 7.4 |  | 0.520 |
| walker |  | 7322 | 284 | Code::CodeKey { rung: Names, file: htmy/html.py, decl: 0, sub: 1, line: 0 } |  |  | 0.522 |
| ns | 7542 |  | 309 | `htmy/md/core.py`: the `MD` component, plus the `md/typing.py` definitions | 7.5 |  | 0.510 |
| walker |  | 7589 | 267 | Code::CodeKey { rung: Names, file: htmy/html.py, decl: 0, sub: 2, line: 0 } |  |  | 0.511 |
| ns | 7801 |  | 259 | `htmy/i18n.py`: `I18n`, its error hierarchy, and the resource loader | 7.6 |  | 0.504 |
| walker |  | 7842 | 253 | Code::CodeKey { rung: Names, file: htmy/html.py, decl: 0, sub: 3, line: 0 } |  |  | 0.505 |
| ns | 8033 |  | 232 | `htmy/tag.py`: how every `html.*` tag is actually built | 7.7 |  | 0.515 |
| walker |  | 8106 | 264 | Code::CodeKey { rung: Names, file: htmy/html.py, decl: 0, sub: 4, line: 0 } |  |  | 0.516 |
| ns | 8234 |  | 201 | `htmy/error_boundary.py` in outline: `ErrorBoundary` | 7.8 |  | 0.519 |
| walker |  | 8403 | 297 | Code::CodeKey { rung: Names, file: htmy/html.py, decl: 0, sub: 5, line: 0 } |  |  | 0.521 |
| ns | 8481 |  | 247 | `htmy/etree.py`: `ETreeConverter` and the optional-`lxml` import switch | 7.9 |  | 0.522 |
| walker |  | 8600 | 197 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 120, sub: 0, line: 831 } |  |  | 0.522 |
| walker |  | 8701 | 101 | Markdown::Section { file: README.md, section_index: 14, keeps_default_concavity: false } |  |  | 0.522 |
| ns | 8704 |  | 223 | `htmy/utils.py` and `htmy/io.py`: every helper, by signature | 7.10 |  | 0.516 |
| walker |  | 8798 | 97 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 15, sub: 0, line: 163 } |  |  | 0.518 |
| walker |  | 8831 | 33 | Markdown::Section { file: docs/index.md, section_index: 22, keeps_default_concavity: false } |  |  | 0.518 |
| ns | 8846 |  | 142 | `html.py` tag roster 1/5: document skeleton, `Link` and `Meta` factories | 8.1 |  | 0.527 |
| walker |  | 8928 | 97 | Markdown::Section { file: README.md, section_index: 16, keeps_default_concavity: false } |  |  | 0.527 |
| ns | 9083 |  | 237 | `html.py` tag roster 2/5: sectioning and container tags | 8.2 | 8.1 | 0.542 |
| ns | 9248 |  | 165 | `html.py` tag roster 3/5: form and interactive tags | 8.3 | 8.1 | 0.552 |
| walker |  | 9264 | 336 | Code::CodeKey { rung: Names, file: htmy/typing.py, decl: 0, sub: 0, line: 0 } |  |  | 0.563 |
| walker |  | 9280 | 16 | Code::CodeKey { rung: Decl, file: htmy/typing.py, decl: 18, sub: 0, line: 73 } |  |  | 0.564 |
| walker |  | 9297 | 17 | Code::CodeKey { rung: Decl, file: htmy/typing.py, decl: 20, sub: 0, line: 81 } |  |  | 0.565 |
| walker |  | 9316 | 19 | Code::CodeKey { rung: Decl, file: htmy/typing.py, decl: 24, sub: 0, line: 103 } |  |  | 0.566 |
| walker |  | 9338 | 22 | Code::CodeKey { rung: Decl, file: htmy/typing.py, decl: 9, sub: 0, line: 37 } |  |  | 0.567 |
| walker |  | 9361 | 23 | Code::CodeKey { rung: Decl, file: htmy/typing.py, decl: 11, sub: 0, line: 45 } |  |  | 0.568 |
| walker |  | 9372 | 11 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 10, sub: 0, line: 40 } |  |  | 0.569 |
| walker |  | 9383 | 11 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 12, sub: 0, line: 48 } |  |  | 0.569 |
| walker |  | 9394 | 11 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 18, sub: 0, line: 73 } |  |  | 0.570 |
| walker |  | 9405 | 11 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 20, sub: 0, line: 81 } |  |  | 0.572 |
| walker |  | 9418 | 13 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 19, sub: 0, line: 76 } |  |  | 0.573 |
| walker |  | 9431 | 13 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 21, sub: 0, line: 84 } |  |  | 0.575 |
| walker |  | 9445 | 14 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 9, sub: 0, line: 37 } |  |  | 0.576 |
| walker |  | 9459 | 14 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 11, sub: 0, line: 45 } |  |  | 0.576 |
| walker |  | 9487 | 28 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 24, sub: 0, line: 103 } |  |  | 0.580 |
| walker |  | 9491 | 4 | Code::CodeKey { rung: Body, file: htmy/typing.py, decl: 10, sub: 0, line: 40 } |  |  | 0.581 |
| walker |  | 9495 | 4 | Code::CodeKey { rung: Body, file: htmy/typing.py, decl: 12, sub: 0, line: 48 } |  |  | 0.581 |
| ns | 9572 |  | 324 | `html.py` tag roster 4/5: text-level tags and embedded media | 8.4 | 8.1 | 0.596 |
| walker |  | 9575 | 80 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 25, sub: 0, line: 108 } |  |  | 0.597 |
| walker |  | 9579 | 4 | Code::CodeKey { rung: Body, file: htmy/typing.py, decl: 19, sub: 0, line: 76 } |  |  | 0.598 |
| walker |  | 9600 | 21 | Markdown::Section { file: docs/api/md.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.598 |
| walker |  | 9736 | 136 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.598 |
| walker |  | 9739 | 3 | Fs::DirListing { dir: examples/internationalization/locale/en } |  |  | 0.598 |
| walker |  | 9854 | 115 | Code::CodeKey { rung: Doc, file: htmy/tag.py, decl: 14, sub: 0, line: 84 } |  |  | 0.598 |
| ns | 9917 |  | 345 | `html.py` tag roster 5/5: lists, tables, headings, media, `entity` | 8.5 | 8.1 | 0.612 |
| walker |  | 9993 | 139 | Code::CodeKey { rung: Names, file: htmy/utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.616 |

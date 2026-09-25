Score(3000)=0.753 I=0.909 C=0.624 ns_rows≤3K=19/49 grid(1000/1442/2080/3000/4327/6240/9000)=0.682/0.657/0.817/0.753/0.590/0.546/0.531

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
| walker |  | 233 | 14 | Fs::DirListing { dir: htmy/md } |  |  | 0.505 |
| walker |  | 255 | 22 | Fs::DirListing { dir: htmy/renderer } |  |  | 0.639 |
| walker |  | 272 | 17 | Code::CodeKey { rung: ModuleDoc, file: htmy/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.639 |
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
| walker |  | 834 | 59 | Fs::DirListing { dir: tests } |  |  | 0.679 |
| walker |  | 862 | 28 | Fs::DirListing { dir: tests/renderer } |  |  | 0.682 |
| ns | 1014 |  | 228 | `htmy/__init__.py` exports, part 3: the sixteen re-exported type names | 1.8 | 1.7 | 0.609 |
| walker |  | 1074 | 212 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.609 |
| ns | 1160 |  | 146 | `htmy/__init__.py` exports, part 4: `utils` helpers and the two aliases | 1.9 | 1.8 | 0.566 |
| walker |  | 1360 | 286 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.697 |
| walker |  | 1393 | 33 | Markdown::Section { file: README.md, section_index: 23, keeps_default_concavity: false } |  |  | 0.697 |
| ns | 1424 |  | 264 | README "Built-in components": one line of semantics per built-in | 2.1 |  | 0.657 |
| walker |  | 1427 | 34 | Fs::DirListing { dir: docs/examples } |  |  | 0.657 |
| ns | 1612 |  | 188 | README: the definition of a component (duck-typed `htmy()` method) | 2.2 |  | 0.641 |
| walker |  | 1673 | 246 | Code::CodeKey { rung: Names, file: htmy/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.759 |
| ns | 1758 |  | 146 | README "Rendering": how to actually invoke the renderer | 2.3 |  | 0.734 |
| ns | 1900 |  | 142 | README "Context": the prop-drilling escape hatch and `htmy_context()` | 2.4 |  | 0.718 |
| walker |  | 1915 | 242 | Code::CodeKey { rung: Names, file: htmy/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.817 |
| walker |  | 2003 | 88 | Code::CodeKey { rung: Names, file: htmy/md/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.817 |
| walker |  | 2106 | 103 | Code::CodeKey { rung: Names, file: htmy/renderer/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.819 |
| ns | 2170 |  | 270 | README "Formatter": the default attribute name/value conversion rules | 2.5 |  | 0.794 |
| walker |  | 2242 | 136 | Code::CodeKey { rung: Names, file: htmy/__init__.py, decl: 0, sub: 2, line: 0 } |  |  | 0.845 |
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
| walker |  | 2864 | 10 | Code::CodeKey { rung: Body, file: htmy/snippet.py, decl: 9, sub: 0, line: 265 } |  |  | 0.792 |
| walker |  | 2875 | 11 | Code::CodeKey { rung: Doc, file: htmy/snippet.py, decl: 7, sub: 0, line: 241 } |  |  | 0.792 |
| walker |  | 2943 | 68 | Code::CodeKey { rung: Decl, file: htmy/snippet.py, decl: 6, sub: 0, line: 218 } |  |  | 0.792 |
| ns | 2950 |  | 156 | `tests/conftest.py` in full: the three session-scoped renderer fixtures | 3.4 |  | 0.753 |
| walker |  | 2958 | 15 | Code::CodeKey { rung: Doc, file: htmy/snippet.py, decl: 10, sub: 0, line: 272 } |  |  | 0.753 |
| walker |  | 2974 | 16 | Code::CodeKey { rung: Doc, file: htmy/snippet.py, decl: 8, sub: 0, line: 256 } |  |  | 0.753 |
| walker |  | 2984 | 10 | Code::CodeKey { rung: Body, file: htmy/snippet.py, decl: 10, sub: 0, line: 272 } |  |  | 0.753 |
| walker |  | 3026 | 42 | Code::CodeKey { rung: Doc, file: htmy/snippet.py, decl: 9, sub: 0, line: 265 } |  |  | 0.753 |
| walker |  | 3053 | 27 | Markdown::HeadingsOutline { file: docs/api/md.md } |  |  | 0.753 |
| walker |  | 3074 | 21 | Markdown::Section { file: docs/api/md.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.753 |
| walker |  | 3077 | 3 | Fs::DirListing { dir: examples/internationalization/locale } |  |  | 0.753 |
| ns | 3118 |  | 168 | `pyproject.toml` build backend, version source and type/test settings | 3.5 |  | 0.717 |
| walker |  | 3181 | 104 | Code::CodeKey { rung: Names, file: htmy/core.py, decl: 0, sub: 0, line: 0 } |  |  | 0.717 |
| walker |  | 3187 | 6 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 14, sub: 0, line: 157 } |  |  | 0.717 |
| walker |  | 3193 | 6 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 15, sub: 0, line: 163 } |  |  | 0.717 |
| walker |  | 3219 | 26 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 12, sub: 0, line: 146 } |  |  | 0.718 |
| walker |  | 3225 | 6 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 13, sub: 0, line: 151 } |  |  | 0.718 |
| walker |  | 3262 | 37 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 16, sub: 0, line: 175 } |  |  | 0.718 |
| walker |  | 3322 | 60 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 1, sub: 0, line: 19 } |  |  | 0.718 |
| ns | 3335 |  | 217 | `htmy/typing.py` lines 4-27: `T`/`U`, property types, the context types | 4.1 |  | 0.679 |
| walker |  | 3383 | 61 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 4, sub: 0, line: 38 } |  |  | 0.679 |
| walker |  | 3394 | 11 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 3, sub: 0, line: 33 } |  |  | 0.679 |
| walker |  | 3407 | 13 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 6, sub: 0, line: 56 } |  |  | 0.679 |
| walker |  | 3421 | 14 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 1, sub: 0, line: 19 } |  |  | 0.680 |
| walker |  | 3436 | 15 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 14, sub: 0, line: 157 } |  |  | 0.680 |
| walker |  | 3453 | 17 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 12, sub: 0, line: 146 } |  |  | 0.680 |
| walker |  | 3578 | 125 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 7, sub: 0, line: 64 } |  |  | 0.680 |
| walker |  | 3586 | 8 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 11, sub: 0, line: 127 } |  |  | 0.681 |
| walker |  | 3608 | 22 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 13, sub: 0, line: 151 } |  |  | 0.681 |
| walker |  | 3631 | 23 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 4, sub: 0, line: 38 } |  |  | 0.681 |
| ns | 3649 |  | 314 | `htmy/typing.py` lines 34-67: the component protocols and the `Component` union | 4.2 | 4.1 | 0.644 |
| walker |  | 3803 | 172 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 19, sub: 0, line: 201 } |  |  | 0.645 |
| walker |  | 3861 | 58 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 20, sub: 0, line: 222 } |  |  | 0.645 |
| walker |  | 3873 | 12 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 25, sub: 0, line: 281 } |  |  | 0.645 |
| walker |  | 3888 | 15 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 26, sub: 0, line: 286 } |  |  | 0.645 |
| walker |  | 3904 | 16 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 21, sub: 0, line: 241 } |  |  | 0.645 |
| walker |  | 3935 | 31 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 18, sub: 0, line: 196 } |  |  | 0.645 |
| ns | 3950 |  | 301 | `htmy/typing.py` lines 70-109: context providers, `TextProcessor`, `TextResolver` | 4.3 | 4.1 | 0.608 |
| walker |  | 3972 | 37 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 17, sub: 0, line: 186 } |  |  | 0.608 |
| walker |  | 4012 | 40 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 9, sub: 0, line: 108 } |  |  | 0.608 |
| walker |  | 4055 | 43 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 2, sub: 0, line: 24 } |  |  | 0.608 |
| ns | 4085 |  | 135 | `htmy/renderer/__init__.py` in full: which class each renderer name resolves to | 5.1 |  | 0.608 |
| walker |  | 4104 | 49 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 10, sub: 0, line: 115 } |  |  | 0.608 |
| walker |  | 4153 | 49 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 23, sub: 0, line: 259 } |  |  | 0.608 |
| walker |  | 4218 | 65 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 5, sub: 0, line: 45 } |  |  | 0.608 |
| walker |  | 4285 | 67 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 22, sub: 0, line: 246 } |  |  | 0.608 |
| ns | 4327 |  | 242 | `htmy/renderer/typing.py`: all four symbols of the renderer protocol module | 5.2 |  | 0.590 |
| walker |  | 4353 | 68 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 16, sub: 0, line: 175 } |  |  | 0.590 |
| walker |  | 4386 | 33 | Markdown::Section { file: docs/api/core.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.590 |
| walker |  | 4419 | 33 | Markdown::Section { file: docs/api/html.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.590 |
| walker |  | 4452 | 33 | Markdown::Section { file: docs/api/utils.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.590 |
| ns | 4508 |  | 181 | `Renderer` in `htmy/renderer/default.py`: strategy docstring and `__init__` | 5.3 |  | 0.575 |
| walker |  | 4522 | 70 | Markdown::Section { file: README.md, section_index: 20, keeps_default_concavity: false } |  |  | 0.575 |
| walker |  | 4556 | 34 | Markdown::Section { file: docs/api/etree.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.575 |
| walker |  | 4590 | 34 | Markdown::Section { file: docs/api/function_component.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.575 |
| walker |  | 4624 | 34 | Markdown::Section { file: docs/api/snippet.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.575 |
| walker |  | 4658 | 34 | Markdown::Section { file: docs/api/typing.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.575 |
| walker |  | 4670 | 12 | Code::CodeKey { rung: Names, file: htmy/error_boundary.py, decl: 0, sub: 0, line: 0 } |  |  | 0.575 |
| ns | 4695 |  | 187 | `Renderer.render()` in `default.py`: context layering and renderer self-registration | 5.4 | 5.3 | 0.565 |
| walker |  | 4746 | 76 | Code::CodeKey { rung: Decl, file: htmy/error_boundary.py, decl: 1, sub: 0, line: 15 } |  |  | 0.565 |
| walker |  | 4801 | 55 | Code::CodeKey { rung: Decl, file: htmy/error_boundary.py, decl: 2, sub: 0, line: 25 } |  |  | 0.565 |
| walker |  | 4836 | 35 | Code::CodeKey { rung: Body, file: htmy/error_boundary.py, decl: 2, sub: 0, line: 25 } |  |  | 0.565 |
| walker |  | 4892 | 56 | Code::CodeKey { rung: Body, file: htmy/error_boundary.py, decl: 4, sub: 0, line: 56 } |  |  | 0.565 |
| ns | 4933 |  | 238 | `Renderer` in `htmy/renderer/baseline.py`: why it exists, plus its full method roster | 5.5 |  | 0.549 |
| walker |  | 4952 | 60 | Code::CodeKey { rung: Doc, file: htmy/error_boundary.py, decl: 1, sub: 0, line: 15 } |  |  | 0.550 |
| walker |  | 5026 | 74 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 24, sub: 0, line: 268 } |  |  | 0.550 |
| walker |  | 5061 | 35 | Markdown::Section { file: docs/api/i18n.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.550 |
| ns | 5073 |  | 140 | `htmy/renderer/context.py`: `RendererContext.from_context()` | 5.6 |  | 0.539 |
| walker |  | 5137 | 76 | Code::CodeKey { rung: Names, file: htmy/tag.py, decl: 0, sub: 0, line: 0 } |  |  | 0.539 |
| walker |  | 5192 | 55 | Code::CodeKey { rung: Decl, file: htmy/tag.py, decl: 4, sub: 0, line: 24 } |  |  | 0.539 |
| walker |  | 5247 | 55 | Code::CodeKey { rung: Decl, file: htmy/tag.py, decl: 5, sub: 0, line: 27 } |  |  | 0.539 |
| ns | 5298 |  | 225 | `core.py`: `Fragment` and `WithContext` | 6.1 |  | 0.547 |
| walker |  | 5304 | 57 | Code::CodeKey { rung: Decl, file: htmy/tag.py, decl: 14, sub: 0, line: 84 } |  |  | 0.547 |
| walker |  | 5365 | 61 | Code::CodeKey { rung: Decl, file: htmy/tag.py, decl: 8, sub: 0, line: 56 } |  |  | 0.547 |
| walker |  | 5427 | 62 | Code::CodeKey { rung: Decl, file: htmy/tag.py, decl: 1, sub: 0, line: 12 } |  |  | 0.547 |
| walker |  | 5504 | 77 | Code::CodeKey { rung: Decl, file: htmy/tag.py, decl: 11, sub: 0, line: 73 } |  |  | 0.548 |
| walker |  | 5512 | 8 | Code::CodeKey { rung: Body, file: htmy/tag.py, decl: 9, sub: 0, line: 66 } |  |  | 0.548 |
| ns | 5550 |  | 252 | `core.py`: `ContextAware`, the typed context registration base class | 6.2 |  | 0.541 |
| walker |  | 5563 | 51 | Code::CodeKey { rung: Doc, file: htmy/tag.py, decl: 8, sub: 0, line: 56 } |  |  | 0.542 |
| walker |  | 5577 | 14 | Code::CodeKey { rung: Body, file: htmy/tag.py, decl: 10, sub: 0, line: 69 } |  |  | 0.542 |
| walker |  | 5654 | 77 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 11, sub: 0, line: 127 } |  |  | 0.542 |
| walker |  | 5667 | 13 | Code::CodeKey { rung: Names, file: htmy/etree.py, decl: 0, sub: 0, line: 0 } |  |  | 0.542 |
| ns | 5823 |  | 273 | `core.py`: `SkipProperty`, `Text`, `SafeStr`, `XBool`, `xml_format_string` | 6.3 |  | 0.549 |
| walker |  | 5900 | 233 | Code::CodeKey { rung: Decl, file: htmy/etree.py, decl: 1, sub: 0, line: 23 } |  |  | 0.549 |
| walker |  | 5911 | 11 | Code::CodeKey { rung: Body, file: htmy/etree.py, decl: 2, sub: 0, line: 46 } |  |  | 0.549 |
| walker |  | 5926 | 15 | Code::CodeKey { rung: Doc, file: htmy/etree.py, decl: 7, sub: 0, line: 103 } |  |  | 0.549 |
| walker |  | 5942 | 16 | Code::CodeKey { rung: Body, file: htmy/etree.py, decl: 7, sub: 0, line: 103 } |  |  | 0.549 |
| walker |  | 5959 | 17 | Code::CodeKey { rung: Doc, file: htmy/etree.py, decl: 4, sub: 0, line: 63 } |  |  | 0.549 |
| walker |  | 5980 | 21 | Code::CodeKey { rung: Doc, file: htmy/etree.py, decl: 3, sub: 0, line: 55 } |  |  | 0.549 |
| walker |  | 6018 | 38 | Code::CodeKey { rung: Doc, file: htmy/etree.py, decl: 6, sub: 0, line: 91 } |  |  | 0.549 |
| walker |  | 6061 | 43 | Code::CodeKey { rung: Doc, file: htmy/etree.py, decl: 2, sub: 0, line: 46 } |  |  | 0.549 |
| ns | 6113 |  | 290 | `core.py`: `Formatter` -- construction and the complete method surface | 6.4 |  | 0.546 |
| walker |  | 6115 | 54 | Code::CodeKey { rung: Doc, file: htmy/etree.py, decl: 5, sub: 0, line: 83 } |  |  | 0.546 |
| walker |  | 6198 | 83 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 20, sub: 0, line: 222 } |  |  | 0.546 |
| walker |  | 6282 | 84 | Code::CodeKey { rung: Doc, file: htmy/error_boundary.py, decl: 4, sub: 0, line: 56 } |  |  | 0.546 |
| ns | 6287 |  | 174 | `core.py`: the default value-formatter table | 6.5 | 6.4 | 0.537 |
| ns | 6480 |  | 193 | `htmy/snippet.py`: `Slots` and the slot placeholder syntax | 7.1 |  | 0.530 |
| walker |  | 6543 | 261 | Code::CodeKey { rung: Names, file: htmy/html.py, decl: 0, sub: 0, line: 0 } |  |  | 0.530 |
| walker |  | 6561 | 18 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 8, sub: 0, line: 63 } |  |  | 0.530 |
| walker |  | 6569 | 8 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 9, sub: 0, line: 66 } |  |  | 0.530 |
| walker |  | 6593 | 24 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 1, sub: 0, line: 13 } |  |  | 0.531 |
| walker |  | 6601 | 8 | Code::CodeKey { rung: Doc, file: htmy/html.py, decl: 1, sub: 0, line: 13 } |  |  | 0.531 |
| walker |  | 6611 | 10 | Code::CodeKey { rung: Doc, file: htmy/html.py, decl: 8, sub: 0, line: 63 } |  |  | 0.531 |
| walker |  | 6714 | 103 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 11, sub: 0, line: 79 } |  |  | 0.531 |
| walker |  | 6722 | 8 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 12, sub: 0, line: 82 } |  |  | 0.531 |
| walker |  | 6730 | 8 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 13, sub: 0, line: 86 } |  |  | 0.531 |
| walker |  | 6738 | 8 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 14, sub: 0, line: 90 } |  |  | 0.531 |
| walker |  | 6746 | 8 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 15, sub: 0, line: 94 } |  |  | 0.531 |
| walker |  | 6754 | 8 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 16, sub: 0, line: 98 } |  |  | 0.531 |
| walker |  | 6764 | 10 | Code::CodeKey { rung: Doc, file: htmy/html.py, decl: 11, sub: 0, line: 79 } |  |  | 0.531 |
| ns | 6772 |  | 292 | `htmy/snippet.py`: `Snippet` and its four-step text pipeline | 7.2 |  | 0.525 |
| ns | 7059 |  | 287 | `htmy/function_component.py`: the complete `@component` decorator family | 7.3 |  | 0.514 |
| ns | 7233 |  | 174 | `htmy/md/core.py`: `MarkdownParser` | 7.4 |  | 0.507 |
| walker |  | 7269 | 505 | Code::CodeKey { rung: Decl, file: htmy/snippet.py, decl: 1, sub: 0, line: 27 } |  |  | 0.519 |
| walker |  | 7335 | 66 | Code::CodeKey { rung: Decl, file: htmy/snippet.py, decl: 2, sub: 0, line: 83 } |  |  | 0.520 |
| ns | 7542 |  | 309 | `htmy/md/core.py`: the `MD` component, plus the `md/typing.py` definitions | 7.5 |  | 0.509 |
| walker |  | 7619 | 284 | Code::CodeKey { rung: Names, file: htmy/html.py, decl: 0, sub: 1, line: 0 } |  |  | 0.511 |
| ns | 7801 |  | 259 | `htmy/i18n.py`: `I18n`, its error hierarchy, and the resource loader | 7.6 |  | 0.503 |
| walker |  | 7886 | 267 | Code::CodeKey { rung: Names, file: htmy/html.py, decl: 0, sub: 2, line: 0 } |  |  | 0.504 |
| ns | 8033 |  | 232 | `htmy/tag.py`: how every `html.*` tag is actually built | 7.7 |  | 0.515 |
| walker |  | 8139 | 253 | Code::CodeKey { rung: Names, file: htmy/html.py, decl: 0, sub: 3, line: 0 } |  |  | 0.516 |
| ns | 8234 |  | 201 | `htmy/error_boundary.py` in outline: `ErrorBoundary` | 7.8 |  | 0.524 |
| walker |  | 8403 | 264 | Code::CodeKey { rung: Names, file: htmy/html.py, decl: 0, sub: 4, line: 0 } |  |  | 0.525 |
| ns | 8481 |  | 247 | `htmy/etree.py`: `ETreeConverter` and the optional-`lxml` import switch | 7.9 |  | 0.526 |
| walker |  | 8700 | 297 | Code::CodeKey { rung: Names, file: htmy/html.py, decl: 0, sub: 5, line: 0 } |  |  | 0.527 |
| ns | 8704 |  | 223 | `htmy/utils.py` and `htmy/io.py`: every helper, by signature | 7.10 |  | 0.521 |
| ns | 8846 |  | 142 | `html.py` tag roster 1/5: document skeleton, `Link` and `Meta` factories | 8.1 |  | 0.530 |
| walker |  | 8897 | 197 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 120, sub: 0, line: 831 } |  |  | 0.531 |
| walker |  | 8998 | 101 | Markdown::Section { file: README.md, section_index: 14, keeps_default_concavity: false } |  |  | 0.531 |
| ns | 9083 |  | 237 | `html.py` tag roster 2/5: sectioning and container tags | 8.2 | 8.1 | 0.545 |
| walker |  | 9095 | 97 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 15, sub: 0, line: 163 } |  |  | 0.548 |
| walker |  | 9128 | 33 | Markdown::Section { file: docs/index.md, section_index: 22, keeps_default_concavity: false } |  |  | 0.548 |
| walker |  | 9225 | 97 | Markdown::Section { file: README.md, section_index: 16, keeps_default_concavity: false } |  |  | 0.548 |
| ns | 9248 |  | 165 | `html.py` tag roster 3/5: form and interactive tags | 8.3 | 8.1 | 0.557 |
| walker |  | 9561 | 336 | Code::CodeKey { rung: Names, file: htmy/typing.py, decl: 0, sub: 0, line: 0 } |  |  | 0.568 |
| ns | 9572 |  | 324 | `html.py` tag roster 4/5: text-level tags and embedded media | 8.4 | 8.1 | 0.584 |
| walker |  | 9577 | 16 | Code::CodeKey { rung: Decl, file: htmy/typing.py, decl: 18, sub: 0, line: 73 } |  |  | 0.584 |
| walker |  | 9594 | 17 | Code::CodeKey { rung: Decl, file: htmy/typing.py, decl: 20, sub: 0, line: 81 } |  |  | 0.585 |
| walker |  | 9613 | 19 | Code::CodeKey { rung: Decl, file: htmy/typing.py, decl: 24, sub: 0, line: 103 } |  |  | 0.586 |
| walker |  | 9635 | 22 | Code::CodeKey { rung: Decl, file: htmy/typing.py, decl: 9, sub: 0, line: 37 } |  |  | 0.587 |
| walker |  | 9658 | 23 | Code::CodeKey { rung: Decl, file: htmy/typing.py, decl: 11, sub: 0, line: 45 } |  |  | 0.588 |
| walker |  | 9669 | 11 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 10, sub: 0, line: 40 } |  |  | 0.589 |
| walker |  | 9680 | 11 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 12, sub: 0, line: 48 } |  |  | 0.589 |
| walker |  | 9691 | 11 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 18, sub: 0, line: 73 } |  |  | 0.590 |
| walker |  | 9702 | 11 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 20, sub: 0, line: 81 } |  |  | 0.592 |
| walker |  | 9715 | 13 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 19, sub: 0, line: 76 } |  |  | 0.593 |
| walker |  | 9728 | 13 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 21, sub: 0, line: 84 } |  |  | 0.594 |
| walker |  | 9742 | 14 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 9, sub: 0, line: 37 } |  |  | 0.596 |
| walker |  | 9756 | 14 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 11, sub: 0, line: 45 } |  |  | 0.596 |
| walker |  | 9784 | 28 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 24, sub: 0, line: 103 } |  |  | 0.600 |
| walker |  | 9788 | 4 | Code::CodeKey { rung: Body, file: htmy/typing.py, decl: 10, sub: 0, line: 40 } |  |  | 0.600 |
| walker |  | 9792 | 4 | Code::CodeKey { rung: Body, file: htmy/typing.py, decl: 12, sub: 0, line: 48 } |  |  | 0.600 |
| walker |  | 9872 | 80 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 25, sub: 0, line: 108 } |  |  | 0.601 |
| walker |  | 9876 | 4 | Code::CodeKey { rung: Body, file: htmy/typing.py, decl: 19, sub: 0, line: 76 } |  |  | 0.602 |
| walker |  | 9897 | 21 | Markdown::Section { file: docs/api/md.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.602 |
| ns | 9917 |  | 345 | `html.py` tag roster 5/5: lists, tables, headings, media, `entity` | 8.5 | 8.1 | 0.617 |

Score(3000)=0.754 I=0.910 C=0.624 ns_rows≤3K=19/49 grid(1000/1442/2080/3000/4327/6240/9000)=0.842/0.659/0.771/0.754/0.589/0.556/0.572

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
| ns | 1612 |  | 188 | README: the definition of a component (duck-typed `htmy()` method) | 2.2 |  | 0.642 |
| walker |  | 1719 | 242 | Code::CodeKey { rung: Names, file: htmy/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.753 |
| ns | 1758 |  | 146 | README "Rendering": how to actually invoke the renderer | 2.3 |  | 0.728 |
| walker |  | 1778 | 59 | Fs::DirListing { dir: tests } |  |  | 0.733 |
| ns | 1900 |  | 142 | README "Context": the prop-drilling escape hatch and `htmy_context()` | 2.4 |  | 0.716 |
| walker |  | 1902 | 124 | Code::CodeKey { rung: Decl, file: htmy/snippet.py, decl: 5, sub: 0, line: 158 } |  |  | 0.716 |
| walker |  | 1920 | 18 | Code::CodeKey { rung: Decl, file: htmy/snippet.py, decl: 10, sub: 0, line: 272 } |  |  | 0.716 |
| walker |  | 1931 | 11 | Code::CodeKey { rung: Doc, file: htmy/snippet.py, decl: 7, sub: 0, line: 241 } |  |  | 0.716 |
| walker |  | 2067 | 136 | Code::CodeKey { rung: Names, file: htmy/__init__.py, decl: 0, sub: 2, line: 0 } |  |  | 0.771 |
| walker |  | 2135 | 68 | Code::CodeKey { rung: Decl, file: htmy/snippet.py, decl: 6, sub: 0, line: 218 } |  |  | 0.771 |
| walker |  | 2163 | 28 | Fs::DirListing { dir: tests/renderer } |  |  | 0.774 |
| ns | 2170 |  | 270 | README "Formatter": the default attribute name/value conversion rules | 2.5 |  | 0.751 |
| ns | 2330 |  | 160 | README "XSS prevention": escaping by default, and the two exceptions | 2.6 |  | 0.726 |
| walker |  | 2375 | 212 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.728 |
| ns | 2534 |  | 204 | `pyproject.toml` project block: runtime deps, Python floor, optional `lxml` | 3.1 |  | 0.738 |
| walker |  | 2661 | 286 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.820 |
| ns | 2688 |  | 154 | `[tool.poe.tasks]`: the project's canonical commands | 3.2 |  | 0.780 |
| walker |  | 2694 | 33 | Markdown::Section { file: README.md, section_index: 23, keeps_default_concavity: false } |  |  | 0.780 |
| walker |  | 2720 | 26 | Code::CodeKey { rung: Body, file: htmy/io.py, decl: 1, sub: 0, line: 11 } |  |  | 0.780 |
| walker |  | 2746 | 26 | Code::CodeKey { rung: Names, file: htmy/md/core.py, decl: 0, sub: 0, line: 0 } |  |  | 0.780 |
| ns | 2794 |  | 106 | Test suite and CI workflow listings (complete) | 3.3 |  | 0.793 |
| walker |  | 2818 | 72 | Code::CodeKey { rung: Decl, file: htmy/md/core.py, decl: 6, sub: 0, line: 81 } |  |  | 0.793 |
| walker |  | 2852 | 34 | Fs::DirListing { dir: docs/examples } |  |  | 0.793 |
| walker |  | 2898 | 46 | Markdown::Section { file: README.md, section_index: 17, keeps_default_concavity: false } |  |  | 0.793 |
| ns | 2950 |  | 156 | `tests/conftest.py` in full: the three session-scoped renderer fixtures | 3.4 |  | 0.754 |
| ns | 3118 |  | 168 | `pyproject.toml` build backend, version source and type/test settings | 3.5 |  | 0.718 |
| walker |  | 3131 | 233 | Code::CodeKey { rung: Decl, file: htmy/etree.py, decl: 1, sub: 0, line: 23 } |  |  | 0.718 |
| walker |  | 3142 | 11 | Code::CodeKey { rung: Body, file: htmy/etree.py, decl: 2, sub: 0, line: 46 } |  |  | 0.718 |
| walker |  | 3159 | 17 | Code::CodeKey { rung: Doc, file: htmy/etree.py, decl: 4, sub: 0, line: 63 } |  |  | 0.718 |
| walker |  | 3180 | 21 | Code::CodeKey { rung: Doc, file: htmy/etree.py, decl: 3, sub: 0, line: 55 } |  |  | 0.718 |
| walker |  | 3256 | 76 | Code::CodeKey { rung: Names, file: htmy/tag.py, decl: 0, sub: 0, line: 0 } |  |  | 0.719 |
| walker |  | 3313 | 57 | Code::CodeKey { rung: Decl, file: htmy/tag.py, decl: 14, sub: 0, line: 84 } |  |  | 0.719 |
| ns | 3335 |  | 217 | `htmy/typing.py` lines 4-27: `T`/`U`, property types, the context types | 4.1 |  | 0.679 |
| walker |  | 3374 | 61 | Code::CodeKey { rung: Decl, file: htmy/tag.py, decl: 8, sub: 0, line: 56 } |  |  | 0.679 |
| walker |  | 3451 | 77 | Code::CodeKey { rung: Decl, file: htmy/tag.py, decl: 11, sub: 0, line: 73 } |  |  | 0.680 |
| walker |  | 3459 | 8 | Code::CodeKey { rung: Body, file: htmy/tag.py, decl: 9, sub: 0, line: 66 } |  |  | 0.680 |
| walker |  | 3514 | 55 | Code::CodeKey { rung: Decl, file: htmy/tag.py, decl: 4, sub: 0, line: 24 } |  |  | 0.680 |
| walker |  | 3576 | 62 | Code::CodeKey { rung: Decl, file: htmy/tag.py, decl: 1, sub: 0, line: 12 } |  |  | 0.680 |
| walker |  | 3648 | 72 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.680 |
| ns | 3649 |  | 314 | `htmy/typing.py` lines 34-67: the component protocols and the `Component` union | 4.2 | 4.1 | 0.643 |
| walker |  | 3786 | 138 | Code::CodeKey { rung: Decl, file: htmy/md/core.py, decl: 1, sub: 0, line: 20 } |  |  | 0.644 |
| walker |  | 3792 | 6 | Code::CodeKey { rung: Decl, file: htmy/md/core.py, decl: 2, sub: 0, line: 33 } |  |  | 0.644 |
| walker |  | 3936 | 144 | Code::CodeKey { rung: Decl, file: htmy/renderer/baseline.py, decl: 1, sub: 0, line: 18 } |  |  | 0.645 |
| ns | 3950 |  | 301 | `htmy/typing.py` lines 70-109: context providers, `TextProcessor`, `TextResolver` | 4.3 | 4.1 | 0.607 |
| walker |  | 3991 | 55 | Code::CodeKey { rung: Decl, file: htmy/renderer/baseline.py, decl: 2, sub: 0, line: 30 } |  |  | 0.607 |
| walker |  | 4042 | 51 | Code::CodeKey { rung: Doc, file: htmy/tag.py, decl: 8, sub: 0, line: 56 } |  |  | 0.608 |
| ns | 4085 |  | 135 | `htmy/renderer/__init__.py` in full: which class each renderer name resolves to | 5.1 |  | 0.607 |
| walker |  | 4099 | 57 | Markdown::Section { file: README.md, section_index: 21, keeps_default_concavity: false } |  |  | 0.607 |
| walker |  | 4316 | 217 | Markdown::HeadingsOutline { file: docs/index.md } |  |  | 0.607 |
| ns | 4327 |  | 242 | `htmy/renderer/typing.py`: all four symbols of the renderer protocol module | 5.2 |  | 0.589 |
| walker |  | 4420 | 104 | Code::CodeKey { rung: Names, file: htmy/core.py, decl: 0, sub: 0, line: 0 } |  |  | 0.589 |
| walker |  | 4426 | 6 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 14, sub: 0, line: 157 } |  |  | 0.589 |
| walker |  | 4432 | 6 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 15, sub: 0, line: 163 } |  |  | 0.589 |
| walker |  | 4458 | 26 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 12, sub: 0, line: 146 } |  |  | 0.589 |
| walker |  | 4464 | 6 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 13, sub: 0, line: 151 } |  |  | 0.589 |
| walker |  | 4501 | 37 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 16, sub: 0, line: 175 } |  |  | 0.589 |
| ns | 4508 |  | 181 | `Renderer` in `htmy/renderer/default.py`: strategy docstring and `__init__` | 5.3 |  | 0.574 |
| walker |  | 4561 | 60 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 1, sub: 0, line: 19 } |  |  | 0.574 |
| walker |  | 4622 | 61 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 4, sub: 0, line: 38 } |  |  | 0.574 |
| walker |  | 4636 | 14 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 1, sub: 0, line: 19 } |  |  | 0.575 |
| walker |  | 4651 | 15 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 14, sub: 0, line: 157 } |  |  | 0.575 |
| walker |  | 4668 | 17 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 12, sub: 0, line: 146 } |  |  | 0.575 |
| walker |  | 4679 | 11 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 3, sub: 0, line: 33 } |  |  | 0.575 |
| ns | 4695 |  | 187 | `Renderer.render()` in `default.py`: context layering and renderer self-registration | 5.4 | 5.3 | 0.565 |
| walker |  | 4804 | 125 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 7, sub: 0, line: 64 } |  |  | 0.566 |
| walker |  | 4812 | 8 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 11, sub: 0, line: 127 } |  |  | 0.566 |
| walker |  | 4835 | 23 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 4, sub: 0, line: 38 } |  |  | 0.566 |
| walker |  | 4848 | 13 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 6, sub: 0, line: 56 } |  |  | 0.566 |
| ns | 4933 |  | 238 | `Renderer` in `htmy/renderer/baseline.py`: why it exists, plus its full method roster | 5.5 |  | 0.559 |
| walker |  | 5020 | 172 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 19, sub: 0, line: 201 } |  |  | 0.559 |
| ns | 5073 |  | 140 | `htmy/renderer/context.py`: `RendererContext.from_context()` | 5.6 |  | 0.550 |
| walker |  | 5078 | 58 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 20, sub: 0, line: 222 } |  |  | 0.550 |
| walker |  | 5094 | 16 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 21, sub: 0, line: 241 } |  |  | 0.550 |
| walker |  | 5125 | 31 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 18, sub: 0, line: 196 } |  |  | 0.550 |
| walker |  | 5147 | 22 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 13, sub: 0, line: 151 } |  |  | 0.550 |
| walker |  | 5207 | 60 | Code::CodeKey { rung: Doc, file: htmy/error_boundary.py, decl: 1, sub: 0, line: 15 } |  |  | 0.550 |
| walker |  | 5262 | 55 | Code::CodeKey { rung: Decl, file: htmy/tag.py, decl: 5, sub: 0, line: 27 } |  |  | 0.550 |
| walker |  | 5294 | 32 | Code::CodeKey { rung: Doc, file: htmy/renderer/context.py, decl: 1, sub: 0, line: 6 } |  |  | 0.555 |
| ns | 5298 |  | 225 | `core.py`: `Fragment` and `WithContext` | 6.1 |  | 0.559 |
| walker |  | 5351 | 57 | Markdown::Section { file: docs/index.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.559 |
| walker |  | 5452 | 101 | Code::CodeKey { rung: Decl, file: htmy/md/core.py, decl: 7, sub: 0, line: 103 } |  |  | 0.559 |
| walker |  | 5479 | 27 | Markdown::HeadingsOutline { file: docs/api/md.md } |  |  | 0.559 |
| walker |  | 5500 | 21 | Markdown::Section { file: docs/api/md.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.559 |
| walker |  | 5503 | 3 | Fs::DirListing { dir: examples/internationalization/locale } |  |  | 0.559 |
| ns | 5550 |  | 252 | `core.py`: `ContextAware`, the typed context registration base class | 6.2 |  | 0.553 |
| walker |  | 5571 | 68 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 16, sub: 0, line: 175 } |  |  | 0.553 |
| walker |  | 5606 | 35 | Code::CodeKey { rung: Body, file: htmy/error_boundary.py, decl: 2, sub: 0, line: 25 } |  |  | 0.553 |
| walker |  | 5643 | 37 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 17, sub: 0, line: 186 } |  |  | 0.553 |
| walker |  | 5676 | 33 | Markdown::Section { file: docs/api/core.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.553 |
| walker |  | 5709 | 33 | Markdown::Section { file: docs/api/html.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.553 |
| walker |  | 5742 | 33 | Markdown::Section { file: docs/api/utils.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.553 |
| walker |  | 5812 | 70 | Markdown::Section { file: README.md, section_index: 20, keeps_default_concavity: false } |  |  | 0.553 |
| ns | 5823 |  | 273 | `core.py`: `SkipProperty`, `Text`, `SafeStr`, `XBool`, `xml_format_string` | 6.3 |  | 0.560 |
| walker |  | 5846 | 34 | Markdown::Section { file: docs/api/etree.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.560 |
| walker |  | 5880 | 34 | Markdown::Section { file: docs/api/function_component.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.560 |
| walker |  | 5914 | 34 | Markdown::Section { file: docs/api/snippet.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.560 |
| walker |  | 5948 | 34 | Markdown::Section { file: docs/api/typing.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.560 |
| walker |  | 5983 | 35 | Markdown::Section { file: docs/api/i18n.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.560 |
| walker |  | 6023 | 40 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 9, sub: 0, line: 108 } |  |  | 0.560 |
| ns | 6113 |  | 290 | `core.py`: `Formatter` -- construction and the complete method surface | 6.4 |  | 0.556 |
| walker |  | 6162 | 139 | Code::CodeKey { rung: Names, file: htmy/utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.556 |
| walker |  | 6208 | 46 | Code::CodeKey { rung: Decl, file: htmy/utils.py, decl: 1, sub: 0, line: 12 } |  |  | 0.556 |
| walker |  | 6222 | 14 | Code::CodeKey { rung: Body, file: htmy/utils.py, decl: 5, sub: 0, line: 62 } |  |  | 0.556 |
| walker |  | 6238 | 16 | Code::CodeKey { rung: Doc, file: htmy/utils.py, decl: 5, sub: 0, line: 62 } |  |  | 0.556 |
| walker |  | 6254 | 16 | Code::CodeKey { rung: Body, file: htmy/utils.py, decl: 2, sub: 0, line: 42 } |  |  | 0.556 |
| walker |  | 6271 | 17 | Code::CodeKey { rung: Doc, file: htmy/utils.py, decl: 6, sub: 0, line: 67 } |  |  | 0.556 |
| ns | 6287 |  | 174 | `core.py`: the default value-formatter table | 6.5 | 6.4 | 0.548 |
| walker |  | 6289 | 18 | Code::CodeKey { rung: Doc, file: htmy/utils.py, decl: 4, sub: 0, line: 55 } |  |  | 0.548 |
| walker |  | 6310 | 21 | Code::CodeKey { rung: Doc, file: htmy/utils.py, decl: 7, sub: 0, line: 73 } |  |  | 0.548 |
| walker |  | 6339 | 29 | Code::CodeKey { rung: Doc, file: htmy/utils.py, decl: 3, sub: 0, line: 49 } |  |  | 0.548 |
| walker |  | 6370 | 31 | Code::CodeKey { rung: Doc, file: htmy/utils.py, decl: 2, sub: 0, line: 42 } |  |  | 0.548 |
| walker |  | 6382 | 12 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 25, sub: 0, line: 281 } |  |  | 0.548 |
| walker |  | 6455 | 73 | Code::CodeKey { rung: Names, file: htmy/renderer/default.py, decl: 0, sub: 0, line: 0 } |  |  | 0.548 |
| ns | 6480 |  | 193 | `htmy/snippet.py`: `Slots` and the slot placeholder syntax | 7.1 |  | 0.540 |
| walker |  | 6516 | 61 | Code::CodeKey { rung: Decl, file: htmy/renderer/default.py, decl: 13, sub: 0, line: 228 } |  |  | 0.541 |
| walker |  | 6571 | 55 | Code::CodeKey { rung: Decl, file: htmy/renderer/default.py, decl: 14, sub: 0, line: 238 } |  |  | 0.545 |
| walker |  | 6617 | 46 | Code::CodeKey { rung: Decl, file: htmy/renderer/default.py, decl: 12, sub: 0, line: 205 } |  |  | 0.545 |
| walker |  | 6762 | 145 | Code::CodeKey { rung: Names, file: htmy/i18n.py, decl: 0, sub: 0, line: 0 } |  |  | 0.545 |
| walker |  | 6771 | 9 | Code::CodeKey { rung: Decl, file: htmy/i18n.py, decl: 11, sub: 0, line: 115 } |  |  | 0.545 |
| ns | 6772 |  | 292 | `htmy/snippet.py`: `Snippet` and its four-step text pipeline | 7.2 |  | 0.539 |
| walker |  | 7001 | 230 | Code::CodeKey { rung: Decl, file: htmy/i18n.py, decl: 5, sub: 0, line: 24 } |  |  | 0.540 |
| walker |  | 7008 | 7 | Code::CodeKey { rung: Decl, file: htmy/i18n.py, decl: 8, sub: 0, line: 48 } |  |  | 0.540 |
| walker |  | 7017 | 9 | Code::CodeKey { rung: Decl, file: htmy/i18n.py, decl: 7, sub: 0, line: 45 } |  |  | 0.540 |
| walker |  | 7025 | 8 | Code::CodeKey { rung: Decl, file: htmy/i18n.py, decl: 10, sub: 0, line: 79 } |  |  | 0.540 |
| walker |  | 7047 | 22 | Code::CodeKey { rung: Doc, file: htmy/i18n.py, decl: 5, sub: 0, line: 24 } |  |  | 0.540 |
| ns | 7059 |  | 287 | `htmy/function_component.py`: the complete `@component` decorator family | 7.3 |  | 0.529 |
| walker |  | 7069 | 22 | Code::CodeKey { rung: Doc, file: htmy/md/core.py, decl: 2, sub: 0, line: 33 } |  |  | 0.529 |
| walker |  | 7112 | 43 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 2, sub: 0, line: 24 } |  |  | 0.530 |
| walker |  | 7155 | 43 | Code::CodeKey { rung: Doc, file: htmy/etree.py, decl: 2, sub: 0, line: 46 } |  |  | 0.530 |
| walker |  | 7232 | 77 | Code::CodeKey { rung: Names, file: htmy/renderer/typing.py, decl: 0, sub: 0, line: 0 } |  |  | 0.532 |
| ns | 7233 |  | 174 | `htmy/md/core.py`: `MarkdownParser` | 7.4 |  | 0.533 |
| walker |  | 7259 | 27 | Code::CodeKey { rung: Decl, file: htmy/renderer/typing.py, decl: 1, sub: 0, line: 12 } |  |  | 0.534 |
| walker |  | 7287 | 28 | Code::CodeKey { rung: Decl, file: htmy/renderer/typing.py, decl: 4, sub: 0, line: 36 } |  |  | 0.536 |
| walker |  | 7297 | 10 | Code::CodeKey { rung: Doc, file: htmy/renderer/typing.py, decl: 1, sub: 0, line: 12 } |  |  | 0.537 |
| walker |  | 7308 | 11 | Code::CodeKey { rung: Doc, file: htmy/renderer/typing.py, decl: 4, sub: 0, line: 36 } |  |  | 0.539 |
| walker |  | 7314 | 6 | Code::CodeKey { rung: Body, file: htmy/renderer/typing.py, decl: 2, sub: 0, line: 15 } |  |  | 0.539 |
| walker |  | 7332 | 18 | Code::CodeKey { rung: Doc, file: htmy/renderer/typing.py, decl: 3, sub: 0, line: 29 } |  |  | 0.541 |
| walker |  | 7351 | 19 | Code::CodeKey { rung: Doc, file: htmy/renderer/typing.py, decl: 6, sub: 0, line: 53 } |  |  | 0.544 |
| walker |  | 7435 | 84 | Code::CodeKey { rung: Doc, file: htmy/utils.py, decl: 1, sub: 0, line: 12 } |  |  | 0.544 |
| walker |  | 7510 | 75 | Code::CodeKey { rung: Decl, file: htmy/renderer/default.py, decl: 1, sub: 0, line: 20 } |  |  | 0.544 |
| ns | 7542 |  | 309 | `htmy/md/core.py`: the `MD` component, plus the `md/typing.py` definitions | 7.5 |  | 0.535 |
| ns | 7801 |  | 259 | `htmy/i18n.py`: `I18n`, its error hierarchy, and the resource loader | 7.6 |  | 0.545 |
| walker |  | 8015 | 505 | Code::CodeKey { rung: Decl, file: htmy/snippet.py, decl: 1, sub: 0, line: 27 } |  |  | 0.556 |
| ns | 8033 |  | 232 | `htmy/tag.py`: how every `html.*` tag is actually built | 7.7 |  | 0.561 |
| walker |  | 8081 | 66 | Code::CodeKey { rung: Decl, file: htmy/snippet.py, decl: 2, sub: 0, line: 83 } |  |  | 0.562 |
| walker |  | 8094 | 13 | Code::CodeKey { rung: Body, file: htmy/snippet.py, decl: 3, sub: 0, line: 110 } |  |  | 0.562 |
| walker |  | 8100 | 6 | Code::CodeKey { rung: Body, file: htmy/renderer/typing.py, decl: 5, sub: 0, line: 39 } |  |  | 0.562 |
| walker |  | 8186 | 86 | Code::CodeKey { rung: Names, file: htmy/md/typing.py, decl: 0, sub: 0, line: 0 } |  |  | 0.567 |
| walker |  | 8211 | 25 | Code::CodeKey { rung: Decl, file: htmy/md/typing.py, decl: 2, sub: 0, line: 14 } |  |  | 0.568 |
| walker |  | 8222 | 11 | Code::CodeKey { rung: Doc, file: htmy/md/typing.py, decl: 2, sub: 0, line: 14 } |  |  | 0.570 |
| ns | 8234 |  | 201 | `htmy/error_boundary.py` in outline: `ErrorBoundary` | 7.8 |  | 0.571 |
| walker |  | 8271 | 49 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 10, sub: 0, line: 115 } |  |  | 0.571 |
| walker |  | 8320 | 49 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 23, sub: 0, line: 259 } |  |  | 0.571 |
| walker |  | 8421 | 101 | Markdown::Section { file: README.md, section_index: 14, keeps_default_concavity: false } |  |  | 0.571 |
| ns | 8481 |  | 247 | `htmy/etree.py`: `ETreeConverter` and the optional-`lxml` import switch | 7.9 |  | 0.570 |
| walker |  | 8518 | 97 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 15, sub: 0, line: 163 } |  |  | 0.573 |
| walker |  | 8533 | 15 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 26, sub: 0, line: 286 } |  |  | 0.573 |
| walker |  | 8548 | 15 | Code::CodeKey { rung: Doc, file: htmy/etree.py, decl: 7, sub: 0, line: 103 } |  |  | 0.573 |
| walker |  | 8563 | 15 | Code::CodeKey { rung: Doc, file: htmy/snippet.py, decl: 10, sub: 0, line: 272 } |  |  | 0.573 |
| walker |  | 8596 | 33 | Markdown::Section { file: docs/index.md, section_index: 22, keeps_default_concavity: false } |  |  | 0.573 |
| walker |  | 8623 | 27 | Code::CodeKey { rung: Doc, file: htmy/md/core.py, decl: 4, sub: 0, line: 53 } |  |  | 0.573 |
| ns | 8704 |  | 223 | `htmy/utils.py` and `htmy/io.py`: every helper, by signature | 7.10 |  | 0.578 |
| walker |  | 8720 | 97 | Markdown::Section { file: README.md, section_index: 16, keeps_default_concavity: false } |  |  | 0.578 |
| ns | 8846 |  | 142 | `html.py` tag roster 1/5: document skeleton, `Link` and `Meta` factories | 8.1 |  | 0.572 |
| walker |  | 9056 | 336 | Code::CodeKey { rung: Names, file: htmy/typing.py, decl: 0, sub: 0, line: 0 } |  |  | 0.583 |
| walker |  | 9072 | 16 | Code::CodeKey { rung: Decl, file: htmy/typing.py, decl: 18, sub: 0, line: 73 } |  |  | 0.584 |
| ns | 9083 |  | 237 | `html.py` tag roster 2/5: sectioning and container tags | 8.2 | 8.1 | 0.573 |
| walker |  | 9089 | 17 | Code::CodeKey { rung: Decl, file: htmy/typing.py, decl: 20, sub: 0, line: 81 } |  |  | 0.574 |
| walker |  | 9108 | 19 | Code::CodeKey { rung: Decl, file: htmy/typing.py, decl: 24, sub: 0, line: 103 } |  |  | 0.575 |
| walker |  | 9130 | 22 | Code::CodeKey { rung: Decl, file: htmy/typing.py, decl: 9, sub: 0, line: 37 } |  |  | 0.576 |
| walker |  | 9153 | 23 | Code::CodeKey { rung: Decl, file: htmy/typing.py, decl: 11, sub: 0, line: 45 } |  |  | 0.577 |
| walker |  | 9164 | 11 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 18, sub: 0, line: 73 } |  |  | 0.578 |
| walker |  | 9175 | 11 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 20, sub: 0, line: 81 } |  |  | 0.579 |
| walker |  | 9189 | 14 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 9, sub: 0, line: 37 } |  |  | 0.581 |
| walker |  | 9203 | 14 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 11, sub: 0, line: 45 } |  |  | 0.581 |
| walker |  | 9214 | 11 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 10, sub: 0, line: 40 } |  |  | 0.582 |
| walker |  | 9225 | 11 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 12, sub: 0, line: 48 } |  |  | 0.582 |
| walker |  | 9238 | 13 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 19, sub: 0, line: 76 } |  |  | 0.584 |
| ns | 9248 |  | 165 | `html.py` tag roster 3/5: form and interactive tags | 8.3 | 8.1 | 0.576 |
| walker |  | 9251 | 13 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 21, sub: 0, line: 84 } |  |  | 0.578 |
| walker |  | 9279 | 28 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 24, sub: 0, line: 103 } |  |  | 0.582 |
| walker |  | 9295 | 16 | Code::CodeKey { rung: Doc, file: htmy/snippet.py, decl: 8, sub: 0, line: 256 } |  |  | 0.582 |
| walker |  | 9311 | 16 | Code::CodeKey { rung: Doc, file: htmy/renderer/default.py, decl: 12, sub: 0, line: 205 } |  |  | 0.582 |
| walker |  | 9332 | 21 | Markdown::Section { file: docs/api/md.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.582 |
| walker |  | 9468 | 136 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.582 |
| ns | 9572 |  | 324 | `html.py` tag roster 4/5: text-level tags and embedded media | 8.4 | 8.1 | 0.568 |
| walker |  | 9577 | 109 | Code::CodeKey { rung: Doc, file: htmy/i18n.py, decl: 11, sub: 0, line: 115 } |  |  | 0.568 |
| walker |  | 9838 | 261 | Code::CodeKey { rung: Names, file: htmy/html.py, decl: 0, sub: 0, line: 0 } |  |  | 0.579 |
| walker |  | 9856 | 18 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 8, sub: 0, line: 63 } |  |  | 0.579 |
| walker |  | 9864 | 8 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 9, sub: 0, line: 66 } |  |  | 0.579 |
| walker |  | 9888 | 24 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 1, sub: 0, line: 13 } |  |  | 0.583 |
| walker |  | 9896 | 8 | Code::CodeKey { rung: Doc, file: htmy/html.py, decl: 1, sub: 0, line: 13 } |  |  | 0.583 |
| walker |  | 9906 | 10 | Code::CodeKey { rung: Doc, file: htmy/html.py, decl: 8, sub: 0, line: 63 } |  |  | 0.583 |
| ns | 9917 |  | 345 | `html.py` tag roster 5/5: lists, tables, headings, media, `entity` | 8.5 | 8.1 | 0.568 |

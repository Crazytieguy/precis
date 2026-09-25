Score(3000)=0.754 I=0.911 C=0.624 ns_rows≤3K=19/49 grid(1000/1442/2080/3000/4327/6240/9000)=0.682/0.657/0.817/0.754/0.590/0.544/0.553

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
| walker |  | 2381 | 21 | Code::CodeKey { rung: Names, file: htmy/snippet.py, decl: 0, sub: 0, line: 0 } |  |  | 0.818 |
| walker |  | 2505 | 124 | Code::CodeKey { rung: Decl, file: htmy/snippet.py, decl: 5, sub: 0, line: 158 } |  |  | 0.818 |
| walker |  | 2523 | 18 | Code::CodeKey { rung: Decl, file: htmy/snippet.py, decl: 10, sub: 0, line: 272 } |  |  | 0.818 |
| walker |  | 2533 | 10 | Code::CodeKey { rung: Body, file: htmy/snippet.py, decl: 9, sub: 0, line: 265 } |  |  | 0.818 |
| ns | 2534 |  | 204 | `pyproject.toml` project block: runtime deps, Python floor, optional `lxml` | 3.1 |  | 0.820 |
| walker |  | 2544 | 11 | Code::CodeKey { rung: Doc, file: htmy/snippet.py, decl: 7, sub: 0, line: 241 } |  |  | 0.820 |
| walker |  | 2612 | 68 | Code::CodeKey { rung: Decl, file: htmy/snippet.py, decl: 6, sub: 0, line: 218 } |  |  | 0.820 |
| walker |  | 2627 | 15 | Code::CodeKey { rung: Doc, file: htmy/snippet.py, decl: 10, sub: 0, line: 272 } |  |  | 0.820 |
| walker |  | 2643 | 16 | Code::CodeKey { rung: Doc, file: htmy/snippet.py, decl: 8, sub: 0, line: 256 } |  |  | 0.820 |
| walker |  | 2653 | 10 | Code::CodeKey { rung: Body, file: htmy/snippet.py, decl: 10, sub: 0, line: 272 } |  |  | 0.820 |
| ns | 2688 |  | 154 | `[tool.poe.tasks]`: the project's canonical commands | 3.2 |  | 0.779 |
| walker |  | 2695 | 42 | Code::CodeKey { rung: Doc, file: htmy/snippet.py, decl: 9, sub: 0, line: 265 } |  |  | 0.779 |
| walker |  | 2752 | 57 | Markdown::Section { file: README.md, section_index: 21, keeps_default_concavity: false } |  |  | 0.779 |
| ns | 2794 |  | 106 | Test suite and CI workflow listings (complete) | 3.3 |  | 0.792 |
| walker |  | 2856 | 104 | Code::CodeKey { rung: Names, file: htmy/core.py, decl: 0, sub: 0, line: 0 } |  |  | 0.793 |
| walker |  | 2862 | 6 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 14, sub: 0, line: 157 } |  |  | 0.793 |
| walker |  | 2868 | 6 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 15, sub: 0, line: 163 } |  |  | 0.793 |
| walker |  | 2894 | 26 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 12, sub: 0, line: 146 } |  |  | 0.793 |
| walker |  | 2900 | 6 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 13, sub: 0, line: 151 } |  |  | 0.793 |
| walker |  | 2937 | 37 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 16, sub: 0, line: 175 } |  |  | 0.793 |
| ns | 2950 |  | 156 | `tests/conftest.py` in full: the three session-scoped renderer fixtures | 3.4 |  | 0.754 |
| walker |  | 2997 | 60 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 1, sub: 0, line: 19 } |  |  | 0.754 |
| walker |  | 3058 | 61 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 4, sub: 0, line: 38 } |  |  | 0.755 |
| walker |  | 3069 | 11 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 3, sub: 0, line: 33 } |  |  | 0.755 |
| walker |  | 3082 | 13 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 6, sub: 0, line: 56 } |  |  | 0.755 |
| walker |  | 3096 | 14 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 1, sub: 0, line: 19 } |  |  | 0.755 |
| walker |  | 3111 | 15 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 14, sub: 0, line: 157 } |  |  | 0.755 |
| ns | 3118 |  | 168 | `pyproject.toml` build backend, version source and type/test settings | 3.5 |  | 0.719 |
| walker |  | 3128 | 17 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 12, sub: 0, line: 146 } |  |  | 0.719 |
| walker |  | 3253 | 125 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 7, sub: 0, line: 64 } |  |  | 0.720 |
| walker |  | 3261 | 8 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 11, sub: 0, line: 127 } |  |  | 0.720 |
| walker |  | 3283 | 22 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 13, sub: 0, line: 151 } |  |  | 0.720 |
| walker |  | 3306 | 23 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 4, sub: 0, line: 38 } |  |  | 0.721 |
| ns | 3335 |  | 217 | `htmy/typing.py` lines 4-27: `T`/`U`, property types, the context types | 4.1 |  | 0.681 |
| walker |  | 3478 | 172 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 19, sub: 0, line: 201 } |  |  | 0.682 |
| walker |  | 3536 | 58 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 20, sub: 0, line: 222 } |  |  | 0.682 |
| walker |  | 3548 | 12 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 25, sub: 0, line: 281 } |  |  | 0.682 |
| walker |  | 3563 | 15 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 26, sub: 0, line: 286 } |  |  | 0.682 |
| walker |  | 3579 | 16 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 21, sub: 0, line: 241 } |  |  | 0.682 |
| walker |  | 3610 | 31 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 18, sub: 0, line: 196 } |  |  | 0.682 |
| walker |  | 3647 | 37 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 17, sub: 0, line: 186 } |  |  | 0.682 |
| ns | 3649 |  | 314 | `htmy/typing.py` lines 34-67: the component protocols and the `Component` union | 4.2 | 4.1 | 0.646 |
| walker |  | 3687 | 40 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 9, sub: 0, line: 108 } |  |  | 0.646 |
| walker |  | 3730 | 43 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 2, sub: 0, line: 24 } |  |  | 0.646 |
| walker |  | 3779 | 49 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 10, sub: 0, line: 115 } |  |  | 0.646 |
| walker |  | 3828 | 49 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 23, sub: 0, line: 259 } |  |  | 0.646 |
| ns | 3950 |  | 301 | `htmy/typing.py` lines 70-109: context providers, `TextProcessor`, `TextResolver` | 4.3 | 4.1 | 0.608 |
| walker |  | 4045 | 217 | Markdown::HeadingsOutline { file: docs/index.md } |  |  | 0.608 |
| walker |  | 4057 | 12 | Code::CodeKey { rung: Names, file: htmy/error_boundary.py, decl: 0, sub: 0, line: 0 } |  |  | 0.608 |
| ns | 4085 |  | 135 | `htmy/renderer/__init__.py` in full: which class each renderer name resolves to | 5.1 |  | 0.608 |
| walker |  | 4133 | 76 | Code::CodeKey { rung: Decl, file: htmy/error_boundary.py, decl: 1, sub: 0, line: 15 } |  |  | 0.608 |
| walker |  | 4188 | 55 | Code::CodeKey { rung: Decl, file: htmy/error_boundary.py, decl: 2, sub: 0, line: 25 } |  |  | 0.608 |
| walker |  | 4223 | 35 | Code::CodeKey { rung: Body, file: htmy/error_boundary.py, decl: 2, sub: 0, line: 25 } |  |  | 0.608 |
| walker |  | 4279 | 56 | Code::CodeKey { rung: Body, file: htmy/error_boundary.py, decl: 4, sub: 0, line: 56 } |  |  | 0.608 |
| ns | 4327 |  | 242 | `htmy/renderer/typing.py`: all four symbols of the renderer protocol module | 5.2 |  | 0.590 |
| walker |  | 4355 | 76 | Code::CodeKey { rung: Names, file: htmy/tag.py, decl: 0, sub: 0, line: 0 } |  |  | 0.590 |
| walker |  | 4410 | 55 | Code::CodeKey { rung: Decl, file: htmy/tag.py, decl: 4, sub: 0, line: 24 } |  |  | 0.590 |
| walker |  | 4465 | 55 | Code::CodeKey { rung: Decl, file: htmy/tag.py, decl: 5, sub: 0, line: 27 } |  |  | 0.590 |
| ns | 4508 |  | 181 | `Renderer` in `htmy/renderer/default.py`: strategy docstring and `__init__` | 5.3 |  | 0.575 |
| walker |  | 4522 | 57 | Code::CodeKey { rung: Decl, file: htmy/tag.py, decl: 14, sub: 0, line: 84 } |  |  | 0.575 |
| walker |  | 4583 | 61 | Code::CodeKey { rung: Decl, file: htmy/tag.py, decl: 8, sub: 0, line: 56 } |  |  | 0.575 |
| walker |  | 4645 | 62 | Code::CodeKey { rung: Decl, file: htmy/tag.py, decl: 1, sub: 0, line: 12 } |  |  | 0.575 |
| ns | 4695 |  | 187 | `Renderer.render()` in `default.py`: context layering and renderer self-registration | 5.4 | 5.3 | 0.565 |
| walker |  | 4722 | 77 | Code::CodeKey { rung: Decl, file: htmy/tag.py, decl: 11, sub: 0, line: 73 } |  |  | 0.566 |
| walker |  | 4730 | 8 | Code::CodeKey { rung: Body, file: htmy/tag.py, decl: 9, sub: 0, line: 66 } |  |  | 0.566 |
| walker |  | 4781 | 51 | Code::CodeKey { rung: Doc, file: htmy/tag.py, decl: 8, sub: 0, line: 56 } |  |  | 0.566 |
| walker |  | 4838 | 57 | Markdown::Section { file: docs/index.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.566 |
| walker |  | 4898 | 60 | Code::CodeKey { rung: Doc, file: htmy/error_boundary.py, decl: 1, sub: 0, line: 15 } |  |  | 0.567 |
| walker |  | 4911 | 13 | Code::CodeKey { rung: Names, file: htmy/etree.py, decl: 0, sub: 0, line: 0 } |  |  | 0.567 |
| ns | 4933 |  | 238 | `Renderer` in `htmy/renderer/baseline.py`: why it exists, plus its full method roster | 5.5 |  | 0.551 |
| ns | 5073 |  | 140 | `htmy/renderer/context.py`: `RendererContext.from_context()` | 5.6 |  | 0.540 |
| walker |  | 5144 | 233 | Code::CodeKey { rung: Decl, file: htmy/etree.py, decl: 1, sub: 0, line: 23 } |  |  | 0.540 |
| walker |  | 5155 | 11 | Code::CodeKey { rung: Body, file: htmy/etree.py, decl: 2, sub: 0, line: 46 } |  |  | 0.540 |
| walker |  | 5170 | 15 | Code::CodeKey { rung: Doc, file: htmy/etree.py, decl: 7, sub: 0, line: 103 } |  |  | 0.540 |
| walker |  | 5186 | 16 | Code::CodeKey { rung: Body, file: htmy/etree.py, decl: 7, sub: 0, line: 103 } |  |  | 0.540 |
| walker |  | 5203 | 17 | Code::CodeKey { rung: Doc, file: htmy/etree.py, decl: 4, sub: 0, line: 63 } |  |  | 0.540 |
| walker |  | 5224 | 21 | Code::CodeKey { rung: Doc, file: htmy/etree.py, decl: 3, sub: 0, line: 55 } |  |  | 0.540 |
| walker |  | 5262 | 38 | Code::CodeKey { rung: Doc, file: htmy/etree.py, decl: 6, sub: 0, line: 91 } |  |  | 0.540 |
| ns | 5298 |  | 225 | `core.py`: `Fragment` and `WithContext` | 6.1 |  | 0.547 |
| walker |  | 5305 | 43 | Code::CodeKey { rung: Doc, file: htmy/etree.py, decl: 2, sub: 0, line: 46 } |  |  | 0.547 |
| walker |  | 5359 | 54 | Code::CodeKey { rung: Doc, file: htmy/etree.py, decl: 5, sub: 0, line: 83 } |  |  | 0.547 |
| walker |  | 5386 | 27 | Markdown::HeadingsOutline { file: docs/api/md.md } |  |  | 0.547 |
| walker |  | 5407 | 21 | Markdown::Section { file: docs/api/md.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.547 |
| walker |  | 5410 | 3 | Fs::DirListing { dir: examples/internationalization/locale } |  |  | 0.547 |
| walker |  | 5475 | 65 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 5, sub: 0, line: 45 } |  |  | 0.548 |
| ns | 5550 |  | 252 | `core.py`: `ContextAware`, the typed context registration base class | 6.2 |  | 0.542 |
| walker |  | 5736 | 261 | Code::CodeKey { rung: Names, file: htmy/html.py, decl: 0, sub: 0, line: 0 } |  |  | 0.543 |
| walker |  | 5754 | 18 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 8, sub: 0, line: 63 } |  |  | 0.543 |
| walker |  | 5762 | 8 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 9, sub: 0, line: 66 } |  |  | 0.543 |
| walker |  | 5786 | 24 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 1, sub: 0, line: 13 } |  |  | 0.543 |
| walker |  | 5794 | 8 | Code::CodeKey { rung: Doc, file: htmy/html.py, decl: 1, sub: 0, line: 13 } |  |  | 0.543 |
| walker |  | 5804 | 10 | Code::CodeKey { rung: Doc, file: htmy/html.py, decl: 8, sub: 0, line: 63 } |  |  | 0.543 |
| ns | 5823 |  | 273 | `core.py`: `SkipProperty`, `Text`, `SafeStr`, `XBool`, `xml_format_string` | 6.3 |  | 0.547 |
| walker |  | 5907 | 103 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 11, sub: 0, line: 79 } |  |  | 0.547 |
| walker |  | 5915 | 8 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 12, sub: 0, line: 82 } |  |  | 0.547 |
| walker |  | 5923 | 8 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 13, sub: 0, line: 86 } |  |  | 0.547 |
| walker |  | 5931 | 8 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 14, sub: 0, line: 90 } |  |  | 0.547 |
| walker |  | 5939 | 8 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 15, sub: 0, line: 94 } |  |  | 0.547 |
| walker |  | 5947 | 8 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 16, sub: 0, line: 98 } |  |  | 0.547 |
| walker |  | 5957 | 10 | Code::CodeKey { rung: Doc, file: htmy/html.py, decl: 11, sub: 0, line: 79 } |  |  | 0.547 |
| walker |  | 5990 | 33 | Markdown::Section { file: docs/api/core.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.547 |
| walker |  | 6023 | 33 | Markdown::Section { file: docs/api/html.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.547 |
| walker |  | 6056 | 33 | Markdown::Section { file: docs/api/utils.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.547 |
| ns | 6113 |  | 290 | `core.py`: `Formatter` -- construction and the complete method surface | 6.4 |  | 0.544 |
| walker |  | 6123 | 67 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 22, sub: 0, line: 246 } |  |  | 0.544 |
| walker |  | 6193 | 70 | Markdown::Section { file: README.md, section_index: 20, keeps_default_concavity: false } |  |  | 0.544 |
| walker |  | 6261 | 68 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 16, sub: 0, line: 175 } |  |  | 0.546 |
| ns | 6287 |  | 174 | `core.py`: the default value-formatter table | 6.5 | 6.4 | 0.538 |
| walker |  | 6295 | 34 | Markdown::Section { file: docs/api/etree.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.538 |
| walker |  | 6329 | 34 | Markdown::Section { file: docs/api/function_component.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.538 |
| walker |  | 6363 | 34 | Markdown::Section { file: docs/api/snippet.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.538 |
| walker |  | 6397 | 34 | Markdown::Section { file: docs/api/typing.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.538 |
| walker |  | 6411 | 14 | Code::CodeKey { rung: Body, file: htmy/tag.py, decl: 10, sub: 0, line: 69 } |  |  | 0.538 |
| walker |  | 6446 | 35 | Markdown::Section { file: docs/api/i18n.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.538 |
| ns | 6480 |  | 193 | `htmy/snippet.py`: `Slots` and the slot placeholder syntax | 7.1 |  | 0.531 |
| walker |  | 6730 | 284 | Code::CodeKey { rung: Names, file: htmy/html.py, decl: 0, sub: 1, line: 0 } |  |  | 0.532 |
| ns | 6772 |  | 292 | `htmy/snippet.py`: `Snippet` and its four-step text pipeline | 7.2 |  | 0.527 |
| walker |  | 6997 | 267 | Code::CodeKey { rung: Names, file: htmy/html.py, decl: 0, sub: 2, line: 0 } |  |  | 0.528 |
| ns | 7059 |  | 287 | `htmy/function_component.py`: the complete `@component` decorator family | 7.3 |  | 0.517 |
| ns | 7233 |  | 174 | `htmy/md/core.py`: `MarkdownParser` | 7.4 |  | 0.510 |
| walker |  | 7250 | 253 | Code::CodeKey { rung: Names, file: htmy/html.py, decl: 0, sub: 3, line: 0 } |  |  | 0.511 |
| walker |  | 7514 | 264 | Code::CodeKey { rung: Names, file: htmy/html.py, decl: 0, sub: 4, line: 0 } |  |  | 0.512 |
| ns | 7542 |  | 309 | `htmy/md/core.py`: the `MD` component, plus the `md/typing.py` definitions | 7.5 |  | 0.501 |
| ns | 7801 |  | 259 | `htmy/i18n.py`: `I18n`, its error hierarchy, and the resource loader | 7.6 |  | 0.494 |
| walker |  | 7811 | 297 | Code::CodeKey { rung: Names, file: htmy/html.py, decl: 0, sub: 5, line: 0 } |  |  | 0.495 |
| walker |  | 8008 | 197 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 120, sub: 0, line: 831 } |  |  | 0.495 |
| ns | 8033 |  | 232 | `htmy/tag.py`: how every `html.*` tag is actually built | 7.7 |  | 0.506 |
| walker |  | 8082 | 74 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 24, sub: 0, line: 268 } |  |  | 0.506 |
| walker |  | 8159 | 77 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 11, sub: 0, line: 127 } |  |  | 0.506 |
| ns | 8234 |  | 201 | `htmy/error_boundary.py` in outline: `ErrorBoundary` | 7.8 |  | 0.515 |
| ns | 8481 |  | 247 | `htmy/etree.py`: `ETreeConverter` and the optional-`lxml` import switch | 7.9 |  | 0.516 |
| walker |  | 8495 | 336 | Code::CodeKey { rung: Names, file: htmy/typing.py, decl: 0, sub: 0, line: 0 } |  |  | 0.529 |
| walker |  | 8511 | 16 | Code::CodeKey { rung: Decl, file: htmy/typing.py, decl: 18, sub: 0, line: 73 } |  |  | 0.530 |
| walker |  | 8528 | 17 | Code::CodeKey { rung: Decl, file: htmy/typing.py, decl: 20, sub: 0, line: 81 } |  |  | 0.531 |
| walker |  | 8547 | 19 | Code::CodeKey { rung: Decl, file: htmy/typing.py, decl: 24, sub: 0, line: 103 } |  |  | 0.532 |
| walker |  | 8569 | 22 | Code::CodeKey { rung: Decl, file: htmy/typing.py, decl: 9, sub: 0, line: 37 } |  |  | 0.533 |
| walker |  | 8592 | 23 | Code::CodeKey { rung: Decl, file: htmy/typing.py, decl: 11, sub: 0, line: 45 } |  |  | 0.534 |
| walker |  | 8603 | 11 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 10, sub: 0, line: 40 } |  |  | 0.536 |
| walker |  | 8614 | 11 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 12, sub: 0, line: 48 } |  |  | 0.536 |
| walker |  | 8625 | 11 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 18, sub: 0, line: 73 } |  |  | 0.537 |
| walker |  | 8636 | 11 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 20, sub: 0, line: 81 } |  |  | 0.539 |
| walker |  | 8649 | 13 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 19, sub: 0, line: 76 } |  |  | 0.540 |
| walker |  | 8662 | 13 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 21, sub: 0, line: 84 } |  |  | 0.542 |
| walker |  | 8676 | 14 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 9, sub: 0, line: 37 } |  |  | 0.544 |
| walker |  | 8690 | 14 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 11, sub: 0, line: 45 } |  |  | 0.544 |
| ns | 8704 |  | 223 | `htmy/utils.py` and `htmy/io.py`: every helper, by signature | 7.10 |  | 0.538 |
| walker |  | 8718 | 28 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 24, sub: 0, line: 103 } |  |  | 0.542 |
| walker |  | 8722 | 4 | Code::CodeKey { rung: Body, file: htmy/typing.py, decl: 10, sub: 0, line: 40 } |  |  | 0.543 |
| walker |  | 8726 | 4 | Code::CodeKey { rung: Body, file: htmy/typing.py, decl: 12, sub: 0, line: 48 } |  |  | 0.543 |
| walker |  | 8806 | 80 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 25, sub: 0, line: 108 } |  |  | 0.544 |
| ns | 8846 |  | 142 | `html.py` tag roster 1/5: document skeleton, `Link` and `Meta` factories | 8.1 |  | 0.553 |
| ns | 9083 |  | 237 | `html.py` tag roster 2/5: sectioning and container tags | 8.2 | 8.1 | 0.567 |
| ns | 9248 |  | 165 | `html.py` tag roster 3/5: form and interactive tags | 8.3 | 8.1 | 0.575 |
| walker |  | 9311 | 505 | Code::CodeKey { rung: Decl, file: htmy/snippet.py, decl: 1, sub: 0, line: 27 } |  |  | 0.584 |
| walker |  | 9377 | 66 | Code::CodeKey { rung: Decl, file: htmy/snippet.py, decl: 2, sub: 0, line: 83 } |  |  | 0.585 |
| walker |  | 9460 | 83 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 20, sub: 0, line: 222 } |  |  | 0.585 |
| walker |  | 9544 | 84 | Code::CodeKey { rung: Doc, file: htmy/error_boundary.py, decl: 4, sub: 0, line: 56 } |  |  | 0.585 |
| ns | 9572 |  | 324 | `html.py` tag roster 4/5: text-level tags and embedded media | 8.4 | 8.1 | 0.600 |
| walker |  | 9645 | 101 | Markdown::Section { file: README.md, section_index: 14, keeps_default_concavity: false } |  |  | 0.600 |
| walker |  | 9784 | 139 | Code::CodeKey { rung: Names, file: htmy/utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.604 |
| walker |  | 9830 | 46 | Code::CodeKey { rung: Decl, file: htmy/utils.py, decl: 1, sub: 0, line: 12 } |  |  | 0.609 |
| walker |  | 9844 | 14 | Code::CodeKey { rung: Body, file: htmy/utils.py, decl: 5, sub: 0, line: 62 } |  |  | 0.609 |
| walker |  | 9860 | 16 | Code::CodeKey { rung: Doc, file: htmy/utils.py, decl: 5, sub: 0, line: 62 } |  |  | 0.609 |
| walker |  | 9876 | 16 | Code::CodeKey { rung: Body, file: htmy/utils.py, decl: 2, sub: 0, line: 42 } |  |  | 0.609 |
| walker |  | 9893 | 17 | Code::CodeKey { rung: Doc, file: htmy/utils.py, decl: 6, sub: 0, line: 67 } |  |  | 0.609 |
| walker |  | 9911 | 18 | Code::CodeKey { rung: Doc, file: htmy/utils.py, decl: 4, sub: 0, line: 55 } |  |  | 0.609 |
| ns | 9917 |  | 345 | `html.py` tag roster 5/5: lists, tables, headings, media, `entity` | 8.5 | 8.1 | 0.622 |
| walker |  | 9932 | 21 | Code::CodeKey { rung: Doc, file: htmy/utils.py, decl: 7, sub: 0, line: 73 } |  |  | 0.622 |
| walker |  | 9961 | 29 | Code::CodeKey { rung: Doc, file: htmy/utils.py, decl: 3, sub: 0, line: 49 } |  |  | 0.622 |
| walker |  | 9992 | 31 | Code::CodeKey { rung: Doc, file: htmy/utils.py, decl: 2, sub: 0, line: 42 } |  |  | 0.622 |

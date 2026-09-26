Score(3000)=0.788 I=0.926 C=0.670 ns_rows≤3K=19/49 grid(1000/1442/2080/3000/4327/6240/9000)=0.754/0.660/0.874/0.788/0.650/0.579/0.594

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 33 | 33 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 72 |  | 72 | README identity: name, one-line description, pitch | 1.1 |  | 0.000 |
| walker |  | 100 | 67 | Fs::DirListing { dir: htmy } |  |  | 0.000 |
| ns | 105 |  | 33 | Repository root listing (complete) | 1.2 |  | 0.556 |
| walker |  | 114 | 14 | Fs::DirListing { dir: htmy/md } |  |  | 0.582 |
| ns | 208 |  | 103 | The `htmy/` package and its two subpackages (complete) | 1.3 |  | 0.502 |
| walker |  | 263 | 149 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.846 |
| walker |  | 283 | 20 | Fs::DirListing { dir: docs } |  |  | 0.846 |
| walker |  | 300 | 17 | Code::CodeKey { rung: ModuleDoc, file: htmy/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.846 |
| walker |  | 322 | 22 | Fs::DirListing { dir: htmy/renderer } |  |  | 1.000 |
| ns | 349 |  | 141 | README key features, first half | 1.4 |  | 0.892 |
| walker |  | 395 | 73 | Toml::Identity { file: pyproject.toml } |  |  | 0.893 |
| walker |  | 402 | 7 | Fs::DirListing { dir: .github } |  |  | 0.893 |
| walker |  | 421 | 19 | Fs::DirListing { dir: .github/workflows } |  |  | 0.894 |
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
| walker |  | 1419 | 154 | Toml::Operational { file: pyproject.toml } |  |  | 0.698 |
| ns | 1424 |  | 264 | README "Built-in components": one line of semantics per built-in | 2.1 |  | 0.658 |
| walker |  | 1447 | 28 | Fs::DirListing { dir: tests/renderer } |  |  | 0.660 |
| ns | 1612 |  | 188 | README: the definition of a component (duck-typed `htmy()` method) | 2.2 |  | 0.644 |
| walker |  | 1639 | 192 | Code::CodeKey { rung: Names, file: htmy/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.730 |
| ns | 1758 |  | 146 | README "Rendering": how to actually invoke the renderer | 2.3 |  | 0.706 |
| walker |  | 1831 | 192 | Code::CodeKey { rung: Names, file: htmy/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.777 |
| ns | 1900 |  | 142 | README "Context": the prop-drilling escape hatch and `htmy_context()` | 2.4 |  | 0.760 |
| walker |  | 2071 | 240 | Code::CodeKey { rung: Names, file: htmy/__init__.py, decl: 0, sub: 2, line: 0 } |  |  | 0.874 |
| walker |  | 2105 | 34 | Fs::DirListing { dir: docs/examples } |  |  | 0.874 |
| ns | 2142 |  | 242 | README "Formatter": the default attribute name/value conversion rules | 2.5 |  | 0.847 |
| walker |  | 2162 | 57 | Markdown::Section { file: README.md, section_index: 20, keeps_default_concavity: false } |  |  | 0.847 |
| walker |  | 2232 | 70 | Markdown::Section { file: README.md, section_index: 19, keeps_default_concavity: false } |  |  | 0.847 |
| ns | 2302 |  | 160 | README "XSS prevention": escaping by default, and the two exceptions | 2.6 |  | 0.820 |
| ns | 2506 |  | 204 | `pyproject.toml` project block: runtime deps, Python floor, optional `lxml` | 3.1 |  | 0.804 |
| walker |  | 2568 | 336 | Code::CodeKey { rung: Names, file: htmy/typing.py, decl: 0, sub: 0, line: 0 } |  |  | 0.807 |
| walker |  | 2584 | 16 | Code::CodeKey { rung: Decl, file: htmy/typing.py, decl: 18, sub: 0, line: 73 } |  |  | 0.807 |
| walker |  | 2601 | 17 | Code::CodeKey { rung: Decl, file: htmy/typing.py, decl: 20, sub: 0, line: 81 } |  |  | 0.807 |
| walker |  | 2620 | 19 | Code::CodeKey { rung: Decl, file: htmy/typing.py, decl: 24, sub: 0, line: 103 } |  |  | 0.807 |
| walker |  | 2642 | 22 | Code::CodeKey { rung: Decl, file: htmy/typing.py, decl: 9, sub: 0, line: 37 } |  |  | 0.807 |
| ns | 2660 |  | 154 | `[tool.poe.tasks]`: the project's canonical commands | 3.2 |  | 0.817 |
| walker |  | 2665 | 23 | Code::CodeKey { rung: Decl, file: htmy/typing.py, decl: 11, sub: 0, line: 45 } |  |  | 0.818 |
| walker |  | 2676 | 11 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 10, sub: 0, line: 40 } |  |  | 0.818 |
| walker |  | 2687 | 11 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 12, sub: 0, line: 48 } |  |  | 0.818 |
| walker |  | 2698 | 11 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 18, sub: 0, line: 73 } |  |  | 0.818 |
| walker |  | 2709 | 11 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 20, sub: 0, line: 81 } |  |  | 0.818 |
| walker |  | 2713 | 4 | Code::CodeKey { rung: Body, file: htmy/typing.py, decl: 10, sub: 0, line: 40 } |  |  | 0.818 |
| walker |  | 2717 | 4 | Code::CodeKey { rung: Body, file: htmy/typing.py, decl: 12, sub: 0, line: 48 } |  |  | 0.818 |
| ns | 2766 |  | 106 | Test suite and CI workflow listings (complete) | 3.3 |  | 0.828 |
| walker |  | 2821 | 104 | Code::CodeKey { rung: Names, file: htmy/core.py, decl: 0, sub: 0, line: 0 } |  |  | 0.828 |
| walker |  | 2827 | 6 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 14, sub: 0, line: 157 } |  |  | 0.828 |
| walker |  | 2833 | 6 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 15, sub: 0, line: 163 } |  |  | 0.828 |
| walker |  | 2870 | 37 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 12, sub: 0, line: 146 } |  |  | 0.828 |
| walker |  | 2907 | 37 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 16, sub: 0, line: 175 } |  |  | 0.828 |
| ns | 2922 |  | 156 | `tests/conftest.py` in full: the three session-scoped renderer fixtures | 3.4 |  | 0.787 |
| walker |  | 2967 | 60 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 1, sub: 0, line: 19 } |  |  | 0.788 |
| walker |  | 3028 | 61 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 4, sub: 0, line: 38 } |  |  | 0.788 |
| ns | 3090 |  | 168 | `pyproject.toml` build backend, version source and type/test settings | 3.5 |  | 0.750 |
| walker |  | 3161 | 133 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 7, sub: 0, line: 64 } |  |  | 0.751 |
| ns | 3307 |  | 217 | `htmy/typing.py` lines 4-27: `T`/`U`, property types, the context types | 4.1 |  | 0.722 |
| walker |  | 3344 | 183 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 19, sub: 0, line: 201 } |  |  | 0.723 |
| walker |  | 3391 | 47 | Code::CodeKey { rung: Decl, file: htmy/core.py, decl: 20, sub: 0, line: 222 } |  |  | 0.723 |
| walker |  | 3402 | 11 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 3, sub: 0, line: 33 } |  |  | 0.723 |
| walker |  | 3414 | 12 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 25, sub: 0, line: 281 } |  |  | 0.723 |
| walker |  | 3427 | 13 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 6, sub: 0, line: 56 } |  |  | 0.723 |
| walker |  | 3440 | 13 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 19, sub: 0, line: 76 } |  |  | 0.723 |
| walker |  | 3444 | 4 | Code::CodeKey { rung: Body, file: htmy/typing.py, decl: 19, sub: 0, line: 76 } |  |  | 0.723 |
| walker |  | 3457 | 13 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 21, sub: 0, line: 84 } |  |  | 0.724 |
| walker |  | 3461 | 4 | Code::CodeKey { rung: Body, file: htmy/typing.py, decl: 21, sub: 0, line: 84 } |  |  | 0.724 |
| walker |  | 3548 | 87 | Code::CodeKey { rung: Names, file: htmy/tag.py, decl: 0, sub: 0, line: 0 } |  |  | 0.724 |
| walker |  | 3594 | 46 | Code::CodeKey { rung: Decl, file: htmy/tag.py, decl: 14, sub: 0, line: 84 } |  |  | 0.724 |
| ns | 3621 |  | 314 | `htmy/typing.py` lines 34-67: the component protocols and the `Component` union | 4.2 | 4.1 | 0.699 |
| walker |  | 3655 | 61 | Code::CodeKey { rung: Decl, file: htmy/tag.py, decl: 8, sub: 0, line: 56 } |  |  | 0.700 |
| walker |  | 3717 | 62 | Code::CodeKey { rung: Decl, file: htmy/tag.py, decl: 1, sub: 0, line: 12 } |  |  | 0.700 |
| walker |  | 3783 | 66 | Code::CodeKey { rung: Decl, file: htmy/tag.py, decl: 4, sub: 0, line: 24 } |  |  | 0.700 |
| walker |  | 3827 | 44 | Code::CodeKey { rung: Decl, file: htmy/tag.py, decl: 5, sub: 0, line: 27 } |  |  | 0.700 |
| walker |  | 3904 | 77 | Code::CodeKey { rung: Decl, file: htmy/tag.py, decl: 11, sub: 0, line: 73 } |  |  | 0.700 |
| walker |  | 3918 | 14 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 1, sub: 0, line: 19 } |  |  | 0.701 |
| ns | 3922 |  | 301 | `htmy/typing.py` lines 70-109: context providers, `TextProcessor`, `TextResolver` | 4.3 | 4.1 | 0.681 |
| walker |  | 3932 | 14 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 9, sub: 0, line: 37 } |  |  | 0.684 |
| walker |  | 3946 | 14 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 11, sub: 0, line: 45 } |  |  | 0.684 |
| walker |  | 3961 | 15 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 14, sub: 0, line: 157 } |  |  | 0.685 |
| walker |  | 3976 | 15 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 26, sub: 0, line: 286 } |  |  | 0.685 |
| walker |  | 3992 | 16 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 21, sub: 0, line: 241 } |  |  | 0.685 |
| walker |  | 4009 | 17 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 12, sub: 0, line: 146 } |  |  | 0.685 |
| ns | 4057 |  | 135 | `htmy/renderer/__init__.py` in full: which class each renderer name resolves to | 5.1 |  | 0.669 |
| walker |  | 4110 | 101 | Markdown::Section { file: README.md, section_index: 13, keeps_default_concavity: false } |  |  | 0.669 |
| walker |  | 4116 | 6 | Code::CodeKey { rung: Body, file: htmy/typing.py, decl: 25, sub: 0, line: 108 } |  |  | 0.669 |
| walker |  | 4272 | 156 | Code::CodeKey { rung: Names, file: htmy/utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.670 |
| ns | 4299 |  | 242 | `htmy/renderer/typing.py`: all four symbols of the renderer protocol module | 5.2 |  | 0.649 |
| walker |  | 4301 | 29 | Code::CodeKey { rung: Decl, file: htmy/utils.py, decl: 1, sub: 0, line: 12 } |  |  | 0.650 |
| walker |  | 4318 | 17 | Code::CodeKey { rung: Doc, file: htmy/utils.py, decl: 6, sub: 0, line: 67 } |  |  | 0.650 |
| walker |  | 4336 | 18 | Code::CodeKey { rung: Doc, file: htmy/utils.py, decl: 4, sub: 0, line: 55 } |  |  | 0.650 |
| walker |  | 4354 | 18 | Code::CodeKey { rung: Doc, file: htmy/utils.py, decl: 5, sub: 0, line: 62 } |  |  | 0.650 |
| walker |  | 4375 | 21 | Code::CodeKey { rung: Doc, file: htmy/utils.py, decl: 7, sub: 0, line: 73 } |  |  | 0.650 |
| walker |  | 4397 | 22 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 13, sub: 0, line: 151 } |  |  | 0.650 |
| ns | 4480 |  | 181 | `Renderer` in `htmy/renderer/default.py`: strategy docstring and `__init__` | 5.3 |  | 0.633 |
| walker |  | 4494 | 97 | Markdown::Section { file: README.md, section_index: 15, keeps_default_concavity: false } |  |  | 0.633 |
| walker |  | 4502 | 8 | Code::CodeKey { rung: Body, file: htmy/tag.py, decl: 9, sub: 0, line: 66 } |  |  | 0.633 |
| walker |  | 4525 | 23 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 4, sub: 0, line: 38 } |  |  | 0.633 |
| walker |  | 4537 | 12 | Code::CodeKey { rung: Body, file: htmy/utils.py, decl: 5, sub: 0, line: 62 } |  |  | 0.633 |
| ns | 4667 |  | 187 | `Renderer.render()` in `default.py`: context layering and renderer self-registration | 5.4 | 5.3 | 0.623 |
| walker |  | 4673 | 136 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.623 |
| walker |  | 4874 | 201 | Code::CodeKey { rung: Names, file: htmy/html.py, decl: 0, sub: 0, line: 0 } |  |  | 0.623 |
| walker |  | 4898 | 24 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 1, sub: 0, line: 13 } |  |  | 0.624 |
| ns | 4905 |  | 238 | `Renderer` in `htmy/renderer/baseline.py`: why it exists, plus its full method roster | 5.5 |  | 0.606 |
| walker |  | 4924 | 26 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 8, sub: 0, line: 63 } |  |  | 0.606 |
| ns | 5045 |  | 140 | `htmy/renderer/context.py`: `RendererContext.from_context()` | 5.6 |  | 0.594 |
| walker |  | 5067 | 143 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 11, sub: 0, line: 79 } |  |  | 0.594 |
| walker |  | 5088 | 21 | Code::CodeKey { rung: Names, file: htmy/snippet.py, decl: 0, sub: 0, line: 0 } |  |  | 0.594 |
| walker |  | 5233 | 145 | Code::CodeKey { rung: Decl, file: htmy/snippet.py, decl: 5, sub: 0, line: 158 } |  |  | 0.594 |
| walker |  | 5241 | 8 | Code::CodeKey { rung: Decl, file: htmy/snippet.py, decl: 10, sub: 0, line: 272 } |  |  | 0.595 |
| ns | 5270 |  | 225 | `core.py`: `Fragment` and `WithContext` | 6.1 |  | 0.597 |
| walker |  | 5298 | 57 | Code::CodeKey { rung: Decl, file: htmy/snippet.py, decl: 6, sub: 0, line: 218 } |  |  | 0.597 |
| walker |  | 5309 | 11 | Code::CodeKey { rung: Doc, file: htmy/snippet.py, decl: 7, sub: 0, line: 241 } |  |  | 0.597 |
| walker |  | 5317 | 8 | Code::CodeKey { rung: Doc, file: htmy/html.py, decl: 1, sub: 0, line: 13 } |  |  | 0.597 |
| walker |  | 5332 | 15 | Code::CodeKey { rung: Doc, file: htmy/snippet.py, decl: 10, sub: 0, line: 272 } |  |  | 0.597 |
| ns | 5522 |  | 252 | `core.py`: `ContextAware`, the typed context registration base class | 6.2 |  | 0.589 |
| walker |  | 5565 | 233 | Code::CodeKey { rung: Decl, file: htmy/snippet.py, decl: 1, sub: 0, line: 27 } |  |  | 0.589 |
| ns | 5795 |  | 273 | `core.py`: `SkipProperty`, `Text`, `SafeStr`, `XBool`, `xml_format_string` | 6.3 |  | 0.582 |
| walker |  | 5848 | 283 | Code::CodeKey { rung: Decl, file: htmy/snippet.py, decl: 1, sub: 1, line: 27 } |  |  | 0.583 |
| walker |  | 5903 | 55 | Code::CodeKey { rung: Decl, file: htmy/snippet.py, decl: 2, sub: 0, line: 83 } |  |  | 0.583 |
| walker |  | 5919 | 16 | Code::CodeKey { rung: Doc, file: htmy/snippet.py, decl: 8, sub: 0, line: 256 } |  |  | 0.583 |
| walker |  | 5929 | 10 | Code::CodeKey { rung: Doc, file: htmy/html.py, decl: 8, sub: 0, line: 63 } |  |  | 0.583 |
| walker |  | 5939 | 10 | Code::CodeKey { rung: Doc, file: htmy/html.py, decl: 11, sub: 0, line: 79 } |  |  | 0.583 |
| walker |  | 5952 | 13 | Code::CodeKey { rung: Names, file: htmy/etree.py, decl: 0, sub: 0, line: 0 } |  |  | 0.583 |
| ns | 6085 |  | 290 | `core.py`: `Formatter` -- construction and the complete method surface | 6.4 |  | 0.578 |
| walker |  | 6185 | 233 | Code::CodeKey { rung: Decl, file: htmy/etree.py, decl: 1, sub: 0, line: 23 } |  |  | 0.579 |
| walker |  | 6200 | 15 | Code::CodeKey { rung: Doc, file: htmy/etree.py, decl: 7, sub: 0, line: 103 } |  |  | 0.579 |
| walker |  | 6226 | 26 | Code::CodeKey { rung: Names, file: htmy/md/core.py, decl: 0, sub: 0, line: 0 } |  |  | 0.579 |
| ns | 6259 |  | 174 | `core.py`: the default value-formatter table | 6.5 | 6.4 | 0.570 |
| walker |  | 6309 | 83 | Code::CodeKey { rung: Decl, file: htmy/md/core.py, decl: 6, sub: 0, line: 81 } |  |  | 0.570 |
| walker |  | 6399 | 90 | Code::CodeKey { rung: Decl, file: htmy/md/core.py, decl: 7, sub: 0, line: 103 } |  |  | 0.570 |
| ns | 6452 |  | 193 | `htmy/snippet.py`: `Slots` and the slot placeholder syntax | 7.1 |  | 0.575 |
| walker |  | 6548 | 149 | Code::CodeKey { rung: Decl, file: htmy/md/core.py, decl: 1, sub: 0, line: 20 } |  |  | 0.575 |
| walker |  | 6576 | 28 | Code::CodeKey { rung: Doc, file: htmy/typing.py, decl: 24, sub: 0, line: 103 } |  |  | 0.581 |
| ns | 6744 |  | 292 | `htmy/snippet.py`: `Snippet` and its four-step text pipeline | 7.2 |  | 0.575 |
| walker |  | 6804 | 228 | Code::CodeKey { rung: Names, file: htmy/html.py, decl: 0, sub: 1, line: 0 } |  |  | 0.576 |
| ns | 7031 |  | 287 | `htmy/function_component.py`: the complete `@component` decorator family | 7.3 |  | 0.564 |
| walker |  | 7033 | 229 | Code::CodeKey { rung: Names, file: htmy/html.py, decl: 0, sub: 2, line: 0 } |  |  | 0.565 |
| ns | 7205 |  | 174 | `htmy/md/core.py`: `MarkdownParser` | 7.4 |  | 0.566 |
| walker |  | 7238 | 205 | Code::CodeKey { rung: Names, file: htmy/html.py, decl: 0, sub: 3, line: 0 } |  |  | 0.566 |
| walker |  | 7440 | 202 | Code::CodeKey { rung: Names, file: htmy/html.py, decl: 0, sub: 4, line: 0 } |  |  | 0.567 |
| ns | 7514 |  | 309 | `htmy/md/core.py`: the `MD` component, plus the `md/typing.py` definitions | 7.5 |  | 0.558 |
| walker |  | 7644 | 204 | Code::CodeKey { rung: Names, file: htmy/html.py, decl: 0, sub: 5, line: 0 } |  |  | 0.559 |
| ns | 7773 |  | 259 | `htmy/i18n.py`: `I18n`, its error hierarchy, and the resource loader | 7.6 |  | 0.551 |
| walker |  | 7854 | 210 | Code::CodeKey { rung: Names, file: htmy/html.py, decl: 0, sub: 6, line: 0 } |  |  | 0.552 |
| walker |  | 8001 | 147 | Code::CodeKey { rung: Names, file: htmy/html.py, decl: 0, sub: 7, line: 0 } |  |  | 0.553 |
| ns | 8005 |  | 232 | `htmy/tag.py`: how every `html.*` tag is actually built | 7.7 |  | 0.556 |
| walker |  | 8198 | 197 | Code::CodeKey { rung: Decl, file: htmy/html.py, decl: 120, sub: 0, line: 831 } |  |  | 0.557 |
| ns | 8206 |  | 201 | `htmy/error_boundary.py` in outline: `ErrorBoundary` | 7.8 |  | 0.549 |
| walker |  | 8215 | 17 | Code::CodeKey { rung: Doc, file: htmy/etree.py, decl: 4, sub: 0, line: 63 } |  |  | 0.549 |
| walker |  | 8244 | 29 | Code::CodeKey { rung: Doc, file: htmy/utils.py, decl: 3, sub: 0, line: 49 } |  |  | 0.549 |
| walker |  | 8275 | 31 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 18, sub: 0, line: 196 } |  |  | 0.552 |
| walker |  | 8422 | 147 | Markdown::Section { file: README.md, section_index: 12, keeps_default_concavity: false } |  |  | 0.564 |
| ns | 8453 |  | 247 | `htmy/etree.py`: `ETreeConverter` and the optional-`lxml` import switch | 7.9 |  | 0.563 |
| walker |  | 8455 | 33 | Code::CodeKey { rung: Doc, file: htmy/utils.py, decl: 2, sub: 0, line: 42 } |  |  | 0.563 |
| walker |  | 8469 | 14 | Code::CodeKey { rung: Body, file: htmy/utils.py, decl: 2, sub: 0, line: 42 } |  |  | 0.563 |
| walker |  | 8623 | 154 | Code::CodeKey { rung: Names, file: htmy/i18n.py, decl: 0, sub: 0, line: 0 } |  |  | 0.568 |
| ns | 8676 |  | 223 | `htmy/utils.py` and `htmy/io.py`: every helper, by signature | 7.10 |  | 0.572 |
| walker |  | 8803 | 180 | Code::CodeKey { rung: Decl, file: htmy/i18n.py, decl: 5, sub: 0, line: 24 } |  |  | 0.584 |
| walker |  | 8815 | 12 | Code::CodeKey { rung: Names, file: htmy/error_boundary.py, decl: 0, sub: 0, line: 0 } |  |  | 0.584 |
| ns | 8818 |  | 142 | `html.py` tag roster 1/5: document skeleton, `Link` and `Meta` factories | 8.1 |  | 0.591 |
| walker |  | 8902 | 87 | Code::CodeKey { rung: Decl, file: htmy/error_boundary.py, decl: 1, sub: 0, line: 15 } |  |  | 0.593 |
| walker |  | 8946 | 44 | Code::CodeKey { rung: Decl, file: htmy/error_boundary.py, decl: 2, sub: 0, line: 25 } |  |  | 0.594 |
| walker |  | 8967 | 21 | Code::CodeKey { rung: Doc, file: htmy/etree.py, decl: 3, sub: 0, line: 55 } |  |  | 0.594 |
| walker |  | 9004 | 37 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 17, sub: 0, line: 186 } |  |  | 0.598 |
| walker |  | 9026 | 22 | Code::CodeKey { rung: Doc, file: htmy/i18n.py, decl: 5, sub: 0, line: 24 } |  |  | 0.598 |
| walker |  | 9037 | 11 | Code::CodeKey { rung: Body, file: htmy/etree.py, decl: 2, sub: 0, line: 46 } |  |  | 0.598 |
| ns | 9055 |  | 237 | `html.py` tag roster 2/5: sectioning and container tags | 8.2 | 8.1 | 0.609 |
| walker |  | 9077 | 40 | Code::CodeKey { rung: Doc, file: htmy/core.py, decl: 9, sub: 0, line: 108 } |  |  | 0.609 |
| walker |  | 9099 | 22 | Code::CodeKey { rung: Names, file: htmy/io.py, decl: 0, sub: 0, line: 0 } |  |  | 0.611 |
| ns | 9220 |  | 165 | `html.py` tag roster 3/5: form and interactive tags | 8.3 | 8.1 | 0.618 |
| walker |  | 9323 | 224 | Code::CodeKey { rung: Names, file: htmy/function_component.py, decl: 0, sub: 0, line: 0 } |  |  | 0.619 |
| walker |  | 9339 | 16 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 4, sub: 0, line: 31 } |  |  | 0.619 |
| walker |  | 9357 | 18 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 7, sub: 0, line: 46 } |  |  | 0.619 |
| ns | 9544 |  | 324 | `html.py` tag roster 4/5: text-level tags and embedded media | 8.4 | 8.1 | 0.632 |
| walker |  | 9560 | 203 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 11, sub: 0, line: 71 } |  |  | 0.632 |
| walker |  | 9586 | 26 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 14, sub: 0, line: 194 } |  |  | 0.632 |
| walker |  | 9613 | 27 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 12, sub: 0, line: 86 } |  |  | 0.632 |
| walker |  | 9640 | 27 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 13, sub: 0, line: 147 } |  |  | 0.632 |
| walker |  | 9669 | 29 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 16, sub: 0, line: 315 } |  |  | 0.632 |
| walker |  | 9704 | 35 | Code::CodeKey { rung: Decl, file: htmy/function_component.py, decl: 15, sub: 0, line: 244 } |  |  | 0.632 |
| walker |  | 9723 | 19 | Code::CodeKey { rung: Doc, file: htmy/function_component.py, decl: 11, sub: 0, line: 71 } |  |  | 0.632 |
| walker |  | 9738 | 15 | Code::CodeKey { rung: Doc, file: htmy/io.py, decl: 1, sub: 0, line: 11 } |  |  | 0.632 |
| walker |  | 9777 | 39 | Code::CodeKey { rung: Doc, file: htmy/function_component.py, decl: 4, sub: 0, line: 31 } |  |  | 0.632 |
| walker |  | 9816 | 39 | Code::CodeKey { rung: Doc, file: htmy/function_component.py, decl: 7, sub: 0, line: 46 } |  |  | 0.632 |
| ns | 9889 |  | 345 | `html.py` tag roster 5/5: lists, tables, headings, media, `entity` | 8.5 | 8.1 | 0.644 |
| walker |  | 9893 | 77 | Code::CodeKey { rung: Names, file: htmy/renderer/typing.py, decl: 0, sub: 0, line: 0 } |  |  | 0.645 |
| walker |  | 9920 | 27 | Code::CodeKey { rung: Decl, file: htmy/renderer/typing.py, decl: 1, sub: 0, line: 12 } |  |  | 0.646 |
| walker |  | 9948 | 28 | Code::CodeKey { rung: Decl, file: htmy/renderer/typing.py, decl: 4, sub: 0, line: 36 } |  |  | 0.647 |
| walker |  | 9954 | 6 | Code::CodeKey { rung: Body, file: htmy/renderer/typing.py, decl: 2, sub: 0, line: 15 } |  |  | 0.647 |
| walker |  | 9960 | 6 | Code::CodeKey { rung: Body, file: htmy/renderer/typing.py, decl: 5, sub: 0, line: 39 } |  |  | 0.647 |
| walker |  | 9970 | 10 | Code::CodeKey { rung: Doc, file: htmy/renderer/typing.py, decl: 1, sub: 0, line: 12 } |  |  | 0.648 |
| walker |  | 9981 | 11 | Code::CodeKey { rung: Doc, file: htmy/renderer/typing.py, decl: 4, sub: 0, line: 36 } |  |  | 0.649 |
| walker |  | 9993 | 12 | Code::CodeKey { rung: Names, file: htmy/renderer/default.py, decl: 0, sub: 0, line: 0 } |  |  | 0.649 |

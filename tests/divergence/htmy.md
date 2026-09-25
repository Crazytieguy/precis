Score(3000)=0.627 I=0.845 C=0.466 ns_rows≤3K=19/49 grid(1000/1442/2080/3000/4327/6240/9000)=0.567/0.659/0.767/0.627/0.585/0.559/0.590

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 33 | 33 | listing of '.' |  |  | 0.000 |
| walker |  | 53 | 20 | listing of 'docs' |  |  | 0.000 |
| walker |  | 60 | 7 | listing of '.github' |  |  | 0.000 |
| ns | 72 |  | 72 | README identity: name, one-line description, pitch | 1.1 |  | 0.000 |
| walker |  | 79 | 19 | listing of '.github/workflows' |  |  | 0.000 |
| ns | 105 |  | 33 | Repository root listing (complete) | 1.2 |  | 0.473 |
| walker |  | 152 | 73 | [package] in pyproject.toml |  |  | 0.476 |
| ns | 208 |  | 103 | The `htmy/` package and its two subpackages (complete) | 1.3 |  | 0.272 |
| walker |  | 219 | 67 | listing of 'htmy' |  |  | 0.439 |
| walker |  | 236 | 17 | python module doc htmy/__init__.py |  |  | 0.439 |
| walker |  | 250 | 14 | listing of 'htmy/md' |  |  | 0.505 |
| walker |  | 272 | 22 | listing of 'htmy/renderer' |  |  | 0.639 |
| walker |  | 284 | 12 | python names htmy/error_boundary.py |  |  | 0.639 |
| walker |  | 297 | 13 | python names htmy/etree.py |  |  | 0.639 |
| ns | 349 |  | 141 | README key features, first half | 1.4 |  | 0.570 |
| ns | 504 |  | 155 | README key features, second half | 1.5 | 1.4 | 0.518 |
| walker |  | 543 | 246 | python names htmy/__init__.py |  |  | 0.552 |
| walker |  | 631 | 88 | python names htmy/md/__init__.py |  |  | 0.552 |
| ns | 648 |  | 144 | `htmy/__init__.py` exports, part 1: version + everything from `core` | 1.6 |  | 0.562 |
| walker |  | 652 | 21 | python names htmy/snippet.py |  |  | 0.562 |
| walker |  | 663 | 11 | python names htmy/renderer/baseline.py |  |  | 0.562 |
| walker |  | 685 | 22 | python names htmy/io.py |  |  | 0.562 |
| ns | 786 |  | 138 | `htmy/__init__.py` exports, part 2: `ErrorBoundary`, `component`, renderers, `Snippet`/`Slots`, tags | 1.7 | 1.6 | 0.562 |
| walker |  | 788 | 103 | python names htmy/renderer/__init__.py |  |  | 0.565 |
| walker |  | 800 | 12 | python names htmy/renderer/context.py |  |  | 0.565 |
| walker |  | 842 | 42 | listing of 'docs/api' |  |  | 0.565 |
| walker |  | 854 | 12 | listing of 'docs/api/renderer' |  |  | 0.565 |
| walker |  | 874 | 20 | python decl htmy/renderer/context.py:6 |  |  | 0.565 |
| walker |  | 882 | 8 | python decl htmy/renderer/context.py:11 |  |  | 0.565 |
| walker |  | 911 | 29 | package metadata in pyproject.toml |  |  | 0.567 |
| ns | 1014 |  | 228 | `htmy/__init__.py` exports, part 3: the sixteen re-exported type names | 1.8 | 1.7 | 0.506 |
| walker |  | 1060 | 149 | README headline in README.md |  |  | 0.747 |
| walker |  | 1082 | 22 | listing of 'examples' |  |  | 0.747 |
| ns | 1160 |  | 146 | `htmy/__init__.py` exports, part 4: `utils` helpers and the two aliases | 1.9 | 1.8 | 0.694 |
| walker |  | 1175 | 93 | [dependencies] in pyproject.toml |  |  | 0.699 |
| walker |  | 1201 | 26 | python names htmy/md/core.py |  |  | 0.699 |
| ns | 1424 |  | 264 | README "Built-in components": one line of semantics per built-in | 2.1 |  | 0.659 |
| walker |  | 1443 | 242 | python names htmy/__init__.py #1 |  |  | 0.772 |
| walker |  | 1519 | 76 | python names htmy/tag.py |  |  | 0.772 |
| walker |  | 1576 | 57 | python decl htmy/tag.py:84 |  |  | 0.772 |
| ns | 1612 |  | 188 | README: the definition of a component (duck-typed `htmy()` method) | 2.2 |  | 0.753 |
| walker |  | 1637 | 61 | python decl htmy/tag.py:56 |  |  | 0.753 |
| walker |  | 1713 | 76 | python decl htmy/error_boundary.py:15 |  |  | 0.754 |
| ns | 1758 |  | 146 | README "Rendering": how to actually invoke the renderer | 2.3 |  | 0.729 |
| walker |  | 1790 | 77 | python decl htmy/tag.py:73 |  |  | 0.729 |
| walker |  | 1846 | 56 | headings outline in docs/components-guide.md |  |  | 0.729 |
| walker |  | 1861 | 15 | python doc htmy/io.py:11 |  |  | 0.729 |
| ns | 1900 |  | 142 | README "Context": the prop-drilling escape hatch and `htmy_context()` | 2.4 |  | 0.713 |
| walker |  | 1920 | 59 | headings outline in docs/function-components.md |  |  | 0.713 |
| walker |  | 1928 | 8 | python body htmy/tag.py:66 |  |  | 0.713 |
| walker |  | 2064 | 136 | python names htmy/__init__.py #2 |  |  | 0.767 |
| walker |  | 2168 | 104 | python names htmy/core.py |  |  | 0.768 |
| ns | 2170 |  | 270 | README "Formatter": the default attribute name/value conversion rules | 2.5 |  | 0.744 |
| walker |  | 2174 | 6 | python decl htmy/core.py:157 |  |  | 0.744 |
| walker |  | 2180 | 6 | python decl htmy/core.py:163 |  |  | 0.744 |
| walker |  | 2206 | 26 | python decl htmy/core.py:146 |  |  | 0.745 |
| walker |  | 2212 | 6 | python decl htmy/core.py:151 |  |  | 0.745 |
| walker |  | 2249 | 37 | python decl htmy/core.py:175 |  |  | 0.745 |
| walker |  | 2309 | 60 | python decl htmy/core.py:19 |  |  | 0.745 |
| ns | 2330 |  | 160 | README "XSS prevention": escaping by default, and the two exceptions | 2.6 |  | 0.721 |
| walker |  | 2370 | 61 | python decl htmy/core.py:38 |  |  | 0.721 |
| walker |  | 2425 | 55 | python decl htmy/error_boundary.py:25 |  |  | 0.721 |
| walker |  | 2438 | 13 | listing of 'examples/internationalization' |  |  | 0.721 |
| walker |  | 2452 | 14 | listing of 'examples/markdown_customization' |  |  | 0.721 |
| walker |  | 2466 | 14 | listing of 'examples/markdown_essentials' |  |  | 0.721 |
| ns | 2534 |  | 204 | `pyproject.toml` project block: runtime deps, Python floor, optional `lxml` | 3.1 |  | 0.732 |
| walker |  | 2590 | 124 | python decl htmy/snippet.py:158 |  |  | 0.732 |
| walker |  | 2608 | 18 | python decl htmy/snippet.py:272 |  |  | 0.732 |
| walker |  | 2619 | 11 | python doc htmy/snippet.py:241 |  |  | 0.732 |
| ns | 2688 |  | 154 | `[tool.poe.tasks]`: the project's canonical commands | 3.2 |  | 0.696 |
| walker |  | 2744 | 125 | python decl htmy/core.py:64 |  |  | 0.696 |
| walker |  | 2752 | 8 | python decl htmy/core.py:127 |  |  | 0.696 |
| ns | 2794 |  | 106 | Test suite and CI workflow listings (complete) | 3.3 |  | 0.659 |
| walker |  | 2820 | 68 | python decl htmy/snippet.py:218 |  |  | 0.659 |
| ns | 2950 |  | 156 | `tests/conftest.py` in full: the three session-scoped renderer fixtures | 3.4 |  | 0.627 |
| walker |  | 2959 | 139 | python names htmy/utils.py |  |  | 0.627 |
| walker |  | 3005 | 46 | python decl htmy/utils.py:12 |  |  | 0.628 |
| walker |  | 3019 | 14 | python body htmy/utils.py:62 |  |  | 0.628 |
| walker |  | 3035 | 16 | python doc htmy/utils.py:62 |  |  | 0.628 |
| walker |  | 3051 | 16 | python body htmy/utils.py:42 |  |  | 0.628 |
| ns | 3118 |  | 168 | `pyproject.toml` build backend, version source and type/test settings | 3.5 |  | 0.598 |
| walker |  | 3124 | 73 | python names htmy/renderer/default.py |  |  | 0.598 |
| walker |  | 3185 | 61 | python decl htmy/renderer/default.py:228 |  |  | 0.598 |
| walker |  | 3244 | 59 | listing of 'tests' |  |  | 0.633 |
| ns | 3335 |  | 217 | `htmy/typing.py` lines 4-27: `T`/`U`, property types, the context types | 4.1 |  | 0.599 |
| walker |  | 3456 | 212 | headings outline in README.md |  |  | 0.600 |
| ns | 3649 |  | 314 | `htmy/typing.py` lines 34-67: the component protocols and the `Component` union | 4.2 | 4.1 | 0.567 |
| walker |  | 3742 | 286 | README.md section #1 |  |  | 0.626 |
| walker |  | 3887 | 145 | python names htmy/i18n.py |  |  | 0.626 |
| walker |  | 3896 | 9 | python decl htmy/i18n.py:115 |  |  | 0.626 |
| ns | 3950 |  | 301 | `htmy/typing.py` lines 70-109: context providers, `TextProcessor`, `TextResolver` | 4.3 | 4.1 | 0.590 |
| walker |  | 3968 | 72 | python decl htmy/md/core.py:81 |  |  | 0.590 |
| walker |  | 4045 | 77 | python names htmy/renderer/typing.py |  |  | 0.590 |
| walker |  | 4072 | 27 | python decl htmy/renderer/typing.py:12 |  |  | 0.591 |
| ns | 4085 |  | 135 | `htmy/renderer/__init__.py` in full: which class each renderer name resolves to | 5.1 |  | 0.591 |
| walker |  | 4100 | 28 | python decl htmy/renderer/typing.py:36 |  |  | 0.591 |
| walker |  | 4110 | 10 | python doc htmy/renderer/typing.py:12 |  |  | 0.591 |
| walker |  | 4121 | 11 | python doc htmy/renderer/typing.py:36 |  |  | 0.591 |
| walker |  | 4138 | 17 | python doc htmy/utils.py:67 |  |  | 0.591 |
| walker |  | 4172 | 34 | listing of 'docs/examples' |  |  | 0.591 |
| walker |  | 4205 | 33 | README.md section #39 |  |  | 0.591 |
| walker |  | 4291 | 86 | python names htmy/md/typing.py |  |  | 0.591 |
| walker |  | 4316 | 25 | python decl htmy/md/typing.py:14 |  |  | 0.591 |
| walker |  | 4327 | 11 | python doc htmy/md/typing.py:14 |  |  | 0.585 |
| ns | 4327 |  | 242 | `htmy/renderer/typing.py`: all four symbols of the renderer protocol module | 5.2 |  | 0.585 |
| walker |  | 4353 | 26 | python body htmy/io.py:11 |  |  | 0.585 |
| ns | 4508 |  | 181 | `Renderer` in `htmy/renderer/default.py`: strategy docstring and `__init__` | 5.3 |  | 0.571 |
| walker |  | 4525 | 172 | python decl htmy/core.py:201 |  |  | 0.571 |
| walker |  | 4583 | 58 | python decl htmy/core.py:222 |  |  | 0.571 |
| ns | 4695 |  | 187 | `Renderer.render()` in `default.py`: context layering and renderer self-registration | 5.4 | 5.3 | 0.562 |
| walker |  | 4919 | 336 | python names htmy/typing.py |  |  | 0.582 |
| ns | 4933 |  | 238 | `Renderer` in `htmy/renderer/baseline.py`: why it exists, plus its full method roster | 5.5 |  | 0.566 |
| walker |  | 4935 | 16 | python decl htmy/typing.py:73 |  |  | 0.567 |
| walker |  | 4952 | 17 | python decl htmy/typing.py:81 |  |  | 0.568 |
| walker |  | 4971 | 19 | python decl htmy/typing.py:103 |  |  | 0.570 |
| walker |  | 4993 | 22 | python decl htmy/typing.py:37 |  |  | 0.572 |
| walker |  | 5016 | 23 | python decl htmy/typing.py:45 |  |  | 0.574 |
| walker |  | 5027 | 11 | python doc htmy/typing.py:73 |  |  | 0.576 |
| walker |  | 5038 | 11 | python doc htmy/typing.py:81 |  |  | 0.579 |
| ns | 5073 |  | 140 | `htmy/renderer/context.py`: `RendererContext.from_context()` | 5.6 |  | 0.568 |
| walker |  | 5093 | 55 | python decl htmy/tag.py:24 |  |  | 0.568 |
| ns | 5298 |  | 225 | `core.py`: `Fragment` and `WithContext` | 6.1 |  | 0.561 |
| walker |  | 5354 | 261 | python names htmy/html.py |  |  | 0.562 |
| walker |  | 5372 | 18 | python decl htmy/html.py:63 |  |  | 0.562 |
| walker |  | 5380 | 8 | python decl htmy/html.py:66 |  |  | 0.562 |
| walker |  | 5404 | 24 | python decl htmy/html.py:13 |  |  | 0.562 |
| walker |  | 5412 | 8 | python doc htmy/html.py:13 |  |  | 0.562 |
| walker |  | 5422 | 10 | python doc htmy/html.py:63 |  |  | 0.562 |
| walker |  | 5525 | 103 | python decl htmy/html.py:79 |  |  | 0.562 |
| walker |  | 5533 | 8 | python decl htmy/html.py:82 |  |  | 0.562 |
| walker |  | 5541 | 8 | python decl htmy/html.py:86 |  |  | 0.562 |
| walker |  | 5549 | 8 | python decl htmy/html.py:90 |  |  | 0.562 |
| ns | 5550 |  | 252 | `core.py`: `ContextAware`, the typed context registration base class | 6.2 |  | 0.555 |
| walker |  | 5557 | 8 | python decl htmy/html.py:94 |  |  | 0.555 |
| walker |  | 5565 | 8 | python decl htmy/html.py:98 |  |  | 0.555 |
| walker |  | 5581 | 16 | README.md section #28 |  |  | 0.555 |
| walker |  | 5587 | 6 | python body htmy/renderer/typing.py:15 |  |  | 0.555 |
| walker |  | 5642 | 55 | python decl htmy/renderer/default.py:238 |  |  | 0.560 |
| walker |  | 5670 | 28 | listing of 'tests/renderer' |  |  | 0.573 |
| walker |  | 5732 | 62 | python decl htmy/tag.py:12 |  |  | 0.573 |
| ns | 5823 |  | 273 | `core.py`: `SkipProperty`, `Text`, `SafeStr`, `XBool`, `xml_format_string` | 6.3 |  | 0.562 |
| walker |  | 5956 | 224 | python names htmy/function_component.py |  |  | 0.562 |
| walker |  | 5972 | 16 | python decl htmy/function_component.py:31 |  |  | 0.562 |
| walker |  | 5990 | 18 | python decl htmy/function_component.py:46 |  |  | 0.562 |
| walker |  | 6008 | 18 | python doc htmy/utils.py:55 |  |  | 0.562 |
| ns | 6113 |  | 290 | `core.py`: `Formatter` -- construction and the complete method surface | 6.4 |  | 0.558 |
| walker |  | 6238 | 230 | python decl htmy/i18n.py:24 |  |  | 0.559 |
| walker |  | 6245 | 7 | python decl htmy/i18n.py:48 |  |  | 0.559 |
| walker |  | 6254 | 9 | python decl htmy/i18n.py:45 |  |  | 0.559 |
| walker |  | 6262 | 8 | python decl htmy/i18n.py:79 |  |  | 0.559 |
| ns | 6287 |  | 174 | `core.py`: the default value-formatter table | 6.5 | 6.4 | 0.551 |
| walker |  | 6308 | 46 | README.md section #33 |  |  | 0.551 |
| ns | 6480 |  | 193 | `htmy/snippet.py`: `Slots` and the slot placeholder syntax | 7.1 |  | 0.543 |
| walker |  | 6541 | 233 | python decl htmy/etree.py:23 |  |  | 0.543 |
| walker |  | 6552 | 11 | python body htmy/etree.py:46 |  |  | 0.543 |
| walker |  | 6569 | 17 | python doc htmy/etree.py:63 |  |  | 0.543 |
| walker |  | 6590 | 21 | python doc htmy/etree.py:55 |  |  | 0.543 |
| walker |  | 6662 | 72 | README.md section #3 |  |  | 0.543 |
| ns | 6772 |  | 292 | `htmy/snippet.py`: `Snippet` and its four-step text pipeline | 7.2 |  | 0.537 |
| walker |  | 6924 | 262 | python decl htmy/function_component.py:71 |  |  | 0.538 |
| walker |  | 6931 | 7 | python decl htmy/function_component.py:83 |  |  | 0.538 |
| walker |  | 6938 | 7 | python decl htmy/function_component.py:144 |  |  | 0.538 |
| walker |  | 6947 | 9 | python decl htmy/function_component.py:80 |  |  | 0.538 |
| walker |  | 6956 | 9 | python decl htmy/function_component.py:141 |  |  | 0.538 |
| walker |  | 6996 | 40 | python decl htmy/function_component.py:189 |  |  | 0.538 |
| walker |  | 7036 | 40 | python decl htmy/function_component.py:184 |  |  | 0.538 |
| ns | 7059 |  | 287 | `htmy/function_component.py`: the complete `@component` decorator family | 7.3 |  | 0.539 |
| walker |  | 7076 | 40 | python decl htmy/function_component.py:310 |  |  | 0.539 |
| walker |  | 7116 | 40 | python decl htmy/function_component.py:305 |  |  | 0.546 |
| walker |  | 7162 | 46 | python decl htmy/function_component.py:239 |  |  | 0.546 |
| walker |  | 7208 | 46 | python decl htmy/function_component.py:234 |  |  | 0.554 |
| ns | 7233 |  | 174 | `htmy/md/core.py`: `MarkdownParser` | 7.4 |  | 0.547 |
| walker |  | 7261 | 53 | python decl htmy/function_component.py:86 |  |  | 0.547 |
| walker |  | 7314 | 53 | python decl htmy/function_component.py:147 |  |  | 0.547 |
| walker |  | 7367 | 53 | python decl htmy/function_component.py:194 |  |  | 0.547 |
| walker |  | 7422 | 55 | python decl htmy/function_component.py:315 |  |  | 0.547 |
| walker |  | 7489 | 67 | python decl htmy/function_component.py:244 |  |  | 0.547 |
| walker |  | 7495 | 6 | python body htmy/typing.py:40 |  |  | 0.547 |
| ns | 7542 |  | 309 | `htmy/md/core.py`: the `MD` component, plus the `md/typing.py` definitions | 7.5 |  | 0.540 |
| walker |  | 7633 | 138 | python decl htmy/md/core.py:20 |  |  | 0.547 |
| walker |  | 7639 | 6 | python decl htmy/md/core.py:33 |  |  | 0.549 |
| walker |  | 7660 | 21 | README.md section #13 |  |  | 0.549 |
| walker |  | 7681 | 21 | README.md section #26 |  |  | 0.549 |
| walker |  | 7703 | 22 | python doc htmy/i18n.py:24 |  |  | 0.549 |
| ns | 7801 |  | 259 | `htmy/i18n.py`: `I18n`, its error hierarchy, and the resource loader | 7.6 |  | 0.559 |
| walker |  | 7847 | 144 | python decl htmy/renderer/baseline.py:18 |  |  | 0.564 |
| walker |  | 7902 | 55 | python decl htmy/renderer/baseline.py:30 |  |  | 0.565 |
| walker |  | 7908 | 6 | python body htmy/renderer/typing.py:39 |  |  | 0.565 |
| walker |  | 7954 | 46 | python decl htmy/renderer/default.py:205 |  |  | 0.565 |
| walker |  | 8011 | 57 | README.md section #37 |  |  | 0.565 |
| ns | 8033 |  | 232 | `htmy/tag.py`: how every `html.*` tag is actually built | 7.7 |  | 0.567 |
| walker |  | 8228 | 217 | headings outline in docs/index.md |  |  | 0.567 |
| ns | 8234 |  | 201 | `htmy/error_boundary.py` in outline: `ErrorBoundary` | 7.8 |  | 0.562 |
| walker |  | 8235 | 7 | docs/function-components.md section #14 |  |  | 0.562 |
| walker |  | 8241 | 6 | python body htmy/typing.py:48 |  |  | 0.562 |
| walker |  | 8301 | 60 | python doc htmy/error_boundary.py:15 |  |  | 0.570 |
| walker |  | 8322 | 21 | python doc htmy/utils.py:73 |  |  | 0.570 |
| walker |  | 8377 | 55 | python decl htmy/tag.py:27 |  |  | 0.570 |
| walker |  | 8403 | 26 | README.md section #25 |  |  | 0.570 |
| walker |  | 8435 | 32 | python doc htmy/renderer/context.py:6 |  |  | 0.573 |
| ns | 8481 |  | 247 | `htmy/etree.py`: `ETreeConverter` and the optional-`lxml` import switch | 7.9 |  | 0.572 |
| walker |  | 8492 | 57 | docs/index.md section #0 |  |  | 0.572 |
| walker |  | 8593 | 101 | python decl htmy/md/core.py:103 |  |  | 0.578 |
| walker |  | 8620 | 27 | headings outline in docs/api/md.md |  |  | 0.578 |
| walker |  | 8641 | 21 | docs/api/md.md section #0 |  |  | 0.578 |
| walker |  | 8676 | 35 | python body htmy/error_boundary.py:25 |  |  | 0.578 |
| ns | 8704 |  | 223 | `htmy/utils.py` and `htmy/io.py`: every helper, by signature | 7.10 |  | 0.583 |
| walker |  | 8709 | 33 | docs/api/core.md section #0 |  |  | 0.583 |
| walker |  | 8742 | 33 | docs/api/html.md section #0 |  |  | 0.583 |
| walker |  | 8775 | 33 | docs/api/utils.md section #0 |  |  | 0.583 |
| walker |  | 8845 | 70 | README.md section #36 |  |  | 0.583 |
| ns | 8846 |  | 142 | `html.py` tag roster 1/5: document skeleton, `Link` and `Meta` factories | 8.1 |  | 0.590 |
| walker |  | 8879 | 34 | docs/api/etree.md section #0 |  |  | 0.590 |
| walker |  | 8913 | 34 | docs/api/function_component.md section #0 |  |  | 0.590 |
| walker |  | 8947 | 34 | docs/api/snippet.md section #0 |  |  | 0.590 |
| walker |  | 8981 | 34 | docs/api/typing.md section #0 |  |  | 0.590 |
| walker |  | 9016 | 35 | docs/api/i18n.md section #0 |  |  | 0.590 |
| walker |  | 9022 | 6 | python body htmy/typing.py:76 |  |  | 0.591 |
| ns | 9083 |  | 237 | `html.py` tag roster 2/5: sectioning and container tags | 8.2 | 8.1 | 0.581 |
| walker |  | 9097 | 75 | python decl htmy/renderer/default.py:20 |  |  | 0.581 |
| ns | 9248 |  | 165 | `html.py` tag roster 3/5: form and interactive tags | 8.3 | 8.1 | 0.574 |
| ns | 9572 |  | 324 | `html.py` tag roster 4/5: text-level tags and embedded media | 8.4 | 8.1 | 0.560 |
| walker |  | 9602 | 505 | python decl htmy/snippet.py:27 |  |  | 0.568 |
| walker |  | 9668 | 66 | python decl htmy/snippet.py:83 |  |  | 0.569 |
| walker |  | 9681 | 13 | python body htmy/snippet.py:110 |  |  | 0.569 |
| walker |  | 9703 | 22 | python body htmy/tag.py:84 |  |  | 0.569 |
| walker |  | 9709 | 6 | python body htmy/typing.py:84 |  |  | 0.570 |
| walker |  | 9745 | 36 | README.md section #23 |  |  | 0.570 |
| walker |  | 9846 | 101 | README.md section #30 |  |  | 0.570 |
| walker |  | 9856 | 10 | python doc htmy/html.py:79 |  |  | 0.570 |
| walker |  | 9889 | 33 | docs/index.md section #38 |  |  | 0.570 |
| ns | 9917 |  | 345 | `html.py` tag roster 5/5: lists, tables, headings, media, `entity` | 8.5 | 8.1 | 0.556 |
| walker |  | 9986 | 97 | README.md section #32 |  |  | 0.556 |

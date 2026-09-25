Score(3000)=0.628 I=0.845 C=0.466 ns_rows≤3K=19/49 grid(1000/1442/2080/3000/4327/6240/9000)=0.835/0.765/0.761/0.628/0.570/0.606/0.629

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 33 | 33 | listing of '.' |  |  | 0.000 |
| walker |  | 53 | 20 | listing of 'docs' |  |  | 0.000 |
| walker |  | 60 | 7 | listing of '.github' |  |  | 0.000 |
| ns | 72 |  | 72 | README identity: name, one-line description, pitch | 1.1 |  | 0.000 |
| walker |  | 79 | 19 | listing of '.github/workflows' |  |  | 0.000 |
| ns | 105 |  | 33 | Repository root listing (complete) | 1.2 |  | 0.473 |
| walker |  | 146 | 67 | listing of 'htmy' |  |  | 0.557 |
| walker |  | 163 | 17 | python module doc htmy/__init__.py |  |  | 0.557 |
| walker |  | 177 | 14 | listing of 'htmy/md' |  |  | 0.583 |
| walker |  | 199 | 22 | listing of 'htmy/renderer' |  |  | 0.634 |
| ns | 208 |  | 103 | The `htmy/` package and its two subpackages (complete) | 1.3 |  | 0.636 |
| walker |  | 211 | 12 | python names htmy/error_boundary.py |  |  | 0.636 |
| walker |  | 224 | 13 | python names htmy/etree.py |  |  | 0.636 |
| ns | 349 |  | 141 | README key features, first half | 1.4 |  | 0.567 |
| walker |  | 470 | 246 | python names htmy/__init__.py |  |  | 0.606 |
| ns | 504 |  | 155 | README key features, second half | 1.5 | 1.4 | 0.551 |
| walker |  | 558 | 88 | python names htmy/md/__init__.py |  |  | 0.551 |
| walker |  | 579 | 21 | python names htmy/snippet.py |  |  | 0.551 |
| walker |  | 590 | 11 | python names htmy/renderer/baseline.py |  |  | 0.551 |
| walker |  | 612 | 22 | python names htmy/io.py |  |  | 0.551 |
| ns | 648 |  | 144 | `htmy/__init__.py` exports, part 1: version + everything from `core` | 1.6 |  | 0.561 |
| walker |  | 715 | 103 | python names htmy/renderer/__init__.py |  |  | 0.563 |
| walker |  | 727 | 12 | python names htmy/renderer/context.py |  |  | 0.563 |
| walker |  | 769 | 42 | listing of 'docs/api' |  |  | 0.563 |
| walker |  | 781 | 12 | listing of 'docs/api/renderer' |  |  | 0.563 |
| ns | 786 |  | 138 | `htmy/__init__.py` exports, part 2: `ErrorBoundary`, `component`, renderers, `Snippet`/`Slots`, tags | 1.7 | 1.6 | 0.563 |
| walker |  | 801 | 20 | python decl htmy/renderer/context.py:6 |  |  | 0.563 |
| walker |  | 809 | 8 | python decl htmy/renderer/context.py:11 |  |  | 0.563 |
| walker |  | 958 | 149 | README headline in README.md |  |  | 0.835 |
| walker |  | 980 | 22 | listing of 'examples' |  |  | 0.835 |
| walker |  | 1006 | 26 | python names htmy/md/core.py |  |  | 0.835 |
| ns | 1014 |  | 228 | `htmy/__init__.py` exports, part 3: the sixteen re-exported type names | 1.8 | 1.7 | 0.744 |
| ns | 1160 |  | 146 | `htmy/__init__.py` exports, part 4: `utils` helpers and the two aliases | 1.9 | 1.8 | 0.692 |
| walker |  | 1248 | 242 | python names htmy/__init__.py #1 |  |  | 0.811 |
| walker |  | 1324 | 76 | python names htmy/tag.py |  |  | 0.811 |
| walker |  | 1381 | 57 | python decl htmy/tag.py:84 |  |  | 0.812 |
| ns | 1424 |  | 264 | README "Built-in components": one line of semantics per built-in | 2.1 |  | 0.764 |
| walker |  | 1442 | 61 | python decl htmy/tag.py:56 |  |  | 0.765 |
| walker |  | 1518 | 76 | python decl htmy/error_boundary.py:15 |  |  | 0.765 |
| walker |  | 1595 | 77 | python decl htmy/tag.py:73 |  |  | 0.766 |
| ns | 1612 |  | 188 | README: the definition of a component (duck-typed `htmy()` method) | 2.2 |  | 0.747 |
| walker |  | 1651 | 56 | headings outline in docs/components-guide.md |  |  | 0.747 |
| walker |  | 1651 | 0 | docs/components-guide.md section #0 |  |  | 0.747 |
| walker |  | 1666 | 15 | python doc htmy/io.py:11 |  |  | 0.747 |
| ns | 1758 |  | 146 | README "Rendering": how to actually invoke the renderer | 2.3 |  | 0.722 |
| walker |  | 1763 | 97 | [dependencies] in pyproject.toml |  |  | 0.723 |
| walker |  | 1822 | 59 | headings outline in docs/function-components.md |  |  | 0.723 |
| walker |  | 1830 | 8 | python body htmy/tag.py:66 |  |  | 0.723 |
| ns | 1900 |  | 142 | README "Context": the prop-drilling escape hatch and `htmy_context()` | 2.4 |  | 0.707 |
| walker |  | 1966 | 136 | python names htmy/__init__.py #2 |  |  | 0.761 |
| walker |  | 2070 | 104 | python names htmy/core.py |  |  | 0.761 |
| walker |  | 2076 | 6 | python decl htmy/core.py:157 |  |  | 0.761 |
| walker |  | 2082 | 6 | python decl htmy/core.py:163 |  |  | 0.761 |
| walker |  | 2108 | 26 | python decl htmy/core.py:146 |  |  | 0.761 |
| walker |  | 2114 | 6 | python decl htmy/core.py:151 |  |  | 0.762 |
| walker |  | 2151 | 37 | python decl htmy/core.py:175 |  |  | 0.762 |
| ns | 2170 |  | 270 | README "Formatter": the default attribute name/value conversion rules | 2.5 |  | 0.739 |
| walker |  | 2211 | 60 | python decl htmy/core.py:19 |  |  | 0.739 |
| walker |  | 2272 | 61 | python decl htmy/core.py:38 |  |  | 0.740 |
| ns | 2330 |  | 160 | README "XSS prevention": escaping by default, and the two exceptions | 2.6 |  | 0.716 |
| walker |  | 2341 | 69 | [package] in pyproject.toml |  |  | 0.719 |
| walker |  | 2370 | 29 | package metadata in pyproject.toml |  |  | 0.721 |
| walker |  | 2405 | 35 | manifest config in pyproject.toml |  |  | 0.722 |
| walker |  | 2460 | 55 | python decl htmy/error_boundary.py:25 |  |  | 0.722 |
| walker |  | 2473 | 13 | listing of 'examples/internationalization' |  |  | 0.722 |
| walker |  | 2487 | 14 | listing of 'examples/markdown_customization' |  |  | 0.722 |
| walker |  | 2501 | 14 | listing of 'examples/markdown_essentials' |  |  | 0.722 |
| ns | 2534 |  | 204 | `pyproject.toml` project block: runtime deps, Python floor, optional `lxml` | 3.1 |  | 0.732 |
| walker |  | 2625 | 124 | python decl htmy/snippet.py:158 |  |  | 0.732 |
| walker |  | 2643 | 18 | python decl htmy/snippet.py:272 |  |  | 0.732 |
| walker |  | 2654 | 11 | python doc htmy/snippet.py:241 |  |  | 0.732 |
| ns | 2688 |  | 154 | `[tool.poe.tasks]`: the project's canonical commands | 3.2 |  | 0.696 |
| walker |  | 2779 | 125 | python decl htmy/core.py:64 |  |  | 0.696 |
| walker |  | 2787 | 8 | python decl htmy/core.py:127 |  |  | 0.697 |
| ns | 2794 |  | 106 | Test suite and CI workflow listings (complete) | 3.3 |  | 0.660 |
| walker |  | 2855 | 68 | python decl htmy/snippet.py:218 |  |  | 0.660 |
| ns | 2950 |  | 156 | `tests/conftest.py` in full: the three session-scoped renderer fixtures | 3.4 |  | 0.627 |
| walker |  | 2994 | 139 | python names htmy/utils.py |  |  | 0.628 |
| walker |  | 3040 | 46 | python decl htmy/utils.py:12 |  |  | 0.628 |
| walker |  | 3054 | 14 | python body htmy/utils.py:62 |  |  | 0.628 |
| walker |  | 3070 | 16 | python doc htmy/utils.py:62 |  |  | 0.628 |
| walker |  | 3086 | 16 | python body htmy/utils.py:42 |  |  | 0.628 |
| ns | 3118 |  | 168 | `pyproject.toml` build backend, version source and type/test settings | 3.5 |  | 0.600 |
| walker |  | 3159 | 73 | python names htmy/renderer/default.py |  |  | 0.600 |
| walker |  | 3220 | 61 | python decl htmy/renderer/default.py:228 |  |  | 0.600 |
| walker |  | 3279 | 59 | listing of 'tests' |  |  | 0.636 |
| ns | 3335 |  | 217 | `htmy/typing.py` lines 4-27: `T`/`U`, property types, the context types | 4.1 |  | 0.601 |
| walker |  | 3491 | 212 | headings outline in README.md |  |  | 0.602 |
| walker |  | 3502 | 11 | README.md section #14 |  |  | 0.602 |
| walker |  | 3518 | 16 | README.md section #4 |  |  | 0.603 |
| walker |  | 3534 | 16 | README.md section #7 |  |  | 0.604 |
| walker |  | 3548 | 14 | README.md section #8 |  |  | 0.605 |
| walker |  | 3565 | 17 | README.md section #13 |  |  | 0.608 |
| walker |  | 3584 | 19 | README.md section #3 |  |  | 0.611 |
| walker |  | 3605 | 21 | README.md section #9 |  |  | 0.615 |
| walker |  | 3626 | 21 | README.md section #12 |  |  | 0.620 |
| ns | 3649 |  | 314 | `htmy/typing.py` lines 34-67: the component protocols and the `Component` union | 4.2 | 4.1 | 0.586 |
| walker |  | 3771 | 145 | python names htmy/i18n.py |  |  | 0.587 |
| walker |  | 3780 | 9 | python decl htmy/i18n.py:115 |  |  | 0.587 |
| walker |  | 3852 | 72 | python decl htmy/md/core.py:81 |  |  | 0.587 |
| walker |  | 3929 | 77 | python names htmy/renderer/typing.py |  |  | 0.587 |
| ns | 3950 |  | 301 | `htmy/typing.py` lines 70-109: context providers, `TextProcessor`, `TextResolver` | 4.3 | 4.1 | 0.553 |
| walker |  | 3956 | 27 | python decl htmy/renderer/typing.py:12 |  |  | 0.553 |
| walker |  | 3984 | 28 | python decl htmy/renderer/typing.py:36 |  |  | 0.553 |
| walker |  | 3994 | 10 | python doc htmy/renderer/typing.py:12 |  |  | 0.553 |
| walker |  | 4005 | 11 | python doc htmy/renderer/typing.py:36 |  |  | 0.554 |
| walker |  | 4027 | 22 | README.md section #5 |  |  | 0.557 |
| walker |  | 4044 | 17 | python doc htmy/utils.py:67 |  |  | 0.557 |
| walker |  | 4067 | 23 | README.md section #1 |  |  | 0.562 |
| ns | 4085 |  | 135 | `htmy/renderer/__init__.py` in full: which class each renderer name resolves to | 5.1 |  | 0.563 |
| walker |  | 4088 | 21 | README.md section #2 |  |  | 0.569 |
| walker |  | 4122 | 34 | listing of 'docs/examples' |  |  | 0.569 |
| walker |  | 4155 | 33 | README.md section #52 |  |  | 0.569 |
| walker |  | 4241 | 86 | python names htmy/md/typing.py |  |  | 0.569 |
| walker |  | 4266 | 25 | python decl htmy/md/typing.py:14 |  |  | 0.569 |
| walker |  | 4277 | 11 | python doc htmy/md/typing.py:14 |  |  | 0.570 |
| walker |  | 4303 | 26 | python body htmy/io.py:11 |  |  | 0.570 |
| walker |  | 4327 | 24 | README.md section #11 |  |  | 0.570 |
| ns | 4327 |  | 242 | `htmy/renderer/typing.py`: all four symbols of the renderer protocol module | 5.2 |  | 0.570 |
| walker |  | 4352 | 25 | README.md section #6 |  |  | 0.579 |
| ns | 4508 |  | 181 | `Renderer` in `htmy/renderer/default.py`: strategy docstring and `__init__` | 5.3 |  | 0.565 |
| walker |  | 4524 | 172 | python decl htmy/core.py:201 |  |  | 0.566 |
| walker |  | 4582 | 58 | python decl htmy/core.py:222 |  |  | 0.566 |
| ns | 4695 |  | 187 | `Renderer.render()` in `default.py`: context layering and renderer self-registration | 5.4 | 5.3 | 0.556 |
| walker |  | 4918 | 336 | python names htmy/typing.py |  |  | 0.577 |
| ns | 4933 |  | 238 | `Renderer` in `htmy/renderer/baseline.py`: why it exists, plus its full method roster | 5.5 |  | 0.561 |
| walker |  | 4934 | 16 | python decl htmy/typing.py:73 |  |  | 0.562 |
| walker |  | 4951 | 17 | python decl htmy/typing.py:81 |  |  | 0.563 |
| walker |  | 4970 | 19 | python decl htmy/typing.py:103 |  |  | 0.565 |
| walker |  | 4992 | 22 | python decl htmy/typing.py:37 |  |  | 0.567 |
| walker |  | 5015 | 23 | python decl htmy/typing.py:45 |  |  | 0.569 |
| walker |  | 5026 | 11 | python doc htmy/typing.py:73 |  |  | 0.571 |
| walker |  | 5037 | 11 | python doc htmy/typing.py:81 |  |  | 0.574 |
| ns | 5073 |  | 140 | `htmy/renderer/context.py`: `RendererContext.from_context()` | 5.6 |  | 0.563 |
| walker |  | 5092 | 55 | python decl htmy/tag.py:24 |  |  | 0.563 |
| ns | 5298 |  | 225 | `core.py`: `Fragment` and `WithContext` | 6.1 |  | 0.556 |
| walker |  | 5353 | 261 | python names htmy/html.py |  |  | 0.557 |
| walker |  | 5371 | 18 | python decl htmy/html.py:63 |  |  | 0.557 |
| walker |  | 5379 | 8 | python decl htmy/html.py:66 |  |  | 0.557 |
| walker |  | 5403 | 24 | python decl htmy/html.py:13 |  |  | 0.557 |
| walker |  | 5411 | 8 | python doc htmy/html.py:13 |  |  | 0.557 |
| walker |  | 5421 | 10 | python doc htmy/html.py:63 |  |  | 0.557 |
| walker |  | 5524 | 103 | python decl htmy/html.py:79 |  |  | 0.557 |
| walker |  | 5532 | 8 | python decl htmy/html.py:82 |  |  | 0.557 |
| walker |  | 5540 | 8 | python decl htmy/html.py:86 |  |  | 0.557 |
| walker |  | 5548 | 8 | python decl htmy/html.py:90 |  |  | 0.557 |
| ns | 5550 |  | 252 | `core.py`: `ContextAware`, the typed context registration base class | 6.2 |  | 0.551 |
| walker |  | 5556 | 8 | python decl htmy/html.py:94 |  |  | 0.551 |
| walker |  | 5564 | 8 | python decl htmy/html.py:98 |  |  | 0.551 |
| walker |  | 5580 | 16 | README.md section #41 |  |  | 0.551 |
| walker |  | 5586 | 6 | python body htmy/renderer/typing.py:15 |  |  | 0.551 |
| walker |  | 5641 | 55 | python decl htmy/renderer/default.py:238 |  |  | 0.555 |
| walker |  | 5669 | 28 | listing of 'tests/renderer' |  |  | 0.569 |
| walker |  | 5823 | 154 | tool.poe config in pyproject.toml |  |  | 0.586 |
| ns | 5823 |  | 273 | `core.py`: `SkipProperty`, `Text`, `SafeStr`, `XBool`, `xml_format_string` | 6.3 |  | 0.586 |
| walker |  | 5979 | 156 | tool.mypy+pdm+pyright+pytest config in pyproject.toml |  |  | 0.612 |
| walker |  | 6041 | 62 | python decl htmy/tag.py:12 |  |  | 0.612 |
| ns | 6113 |  | 290 | `core.py`: `Formatter` -- construction and the complete method surface | 6.4 |  | 0.606 |
| walker |  | 6265 | 224 | python names htmy/function_component.py |  |  | 0.606 |
| walker |  | 6281 | 16 | python decl htmy/function_component.py:31 |  |  | 0.606 |
| ns | 6287 |  | 174 | `core.py`: the default value-formatter table | 6.5 | 6.4 | 0.597 |
| walker |  | 6299 | 18 | python decl htmy/function_component.py:46 |  |  | 0.597 |
| walker |  | 6317 | 18 | python doc htmy/utils.py:55 |  |  | 0.597 |
| walker |  | 6348 | 31 | README.md section #10 |  |  | 0.603 |
| ns | 6480 |  | 193 | `htmy/snippet.py`: `Slots` and the slot placeholder syntax | 7.1 |  | 0.594 |
| walker |  | 6578 | 230 | python decl htmy/i18n.py:24 |  |  | 0.595 |
| walker |  | 6585 | 7 | python decl htmy/i18n.py:48 |  |  | 0.595 |
| walker |  | 6594 | 9 | python decl htmy/i18n.py:45 |  |  | 0.595 |
| walker |  | 6602 | 8 | python decl htmy/i18n.py:79 |  |  | 0.595 |
| walker |  | 6648 | 46 | README.md section #46 |  |  | 0.595 |
| ns | 6772 |  | 292 | `htmy/snippet.py`: `Snippet` and its four-step text pipeline | 7.2 |  | 0.588 |
| walker |  | 6881 | 233 | python decl htmy/etree.py:23 |  |  | 0.588 |
| walker |  | 6892 | 11 | python body htmy/etree.py:46 |  |  | 0.588 |
| walker |  | 6909 | 17 | python doc htmy/etree.py:63 |  |  | 0.588 |
| walker |  | 6930 | 21 | python doc htmy/etree.py:55 |  |  | 0.588 |
| walker |  | 7002 | 72 | README.md section #16 |  |  | 0.588 |
| ns | 7059 |  | 287 | `htmy/function_component.py`: the complete `@component` decorator family | 7.3 |  | 0.576 |
| ns | 7233 |  | 174 | `htmy/md/core.py`: `MarkdownParser` | 7.4 |  | 0.568 |
| walker |  | 7264 | 262 | python decl htmy/function_component.py:71 |  |  | 0.574 |
| walker |  | 7271 | 7 | python decl htmy/function_component.py:83 |  |  | 0.574 |
| walker |  | 7278 | 7 | python decl htmy/function_component.py:144 |  |  | 0.574 |
| walker |  | 7287 | 9 | python decl htmy/function_component.py:80 |  |  | 0.574 |
| walker |  | 7296 | 9 | python decl htmy/function_component.py:141 |  |  | 0.575 |
| walker |  | 7336 | 40 | python decl htmy/function_component.py:189 |  |  | 0.575 |
| walker |  | 7376 | 40 | python decl htmy/function_component.py:184 |  |  | 0.580 |
| walker |  | 7416 | 40 | python decl htmy/function_component.py:310 |  |  | 0.580 |
| walker |  | 7456 | 40 | python decl htmy/function_component.py:305 |  |  | 0.586 |
| walker |  | 7502 | 46 | python decl htmy/function_component.py:239 |  |  | 0.586 |
| ns | 7542 |  | 309 | `htmy/md/core.py`: the `MD` component, plus the `md/typing.py` definitions | 7.5 |  | 0.578 |
| walker |  | 7548 | 46 | python decl htmy/function_component.py:234 |  |  | 0.586 |
| walker |  | 7601 | 53 | python decl htmy/function_component.py:86 |  |  | 0.586 |
| walker |  | 7654 | 53 | python decl htmy/function_component.py:147 |  |  | 0.586 |
| walker |  | 7707 | 53 | python decl htmy/function_component.py:194 |  |  | 0.586 |
| walker |  | 7762 | 55 | python decl htmy/function_component.py:315 |  |  | 0.586 |
| ns | 7801 |  | 259 | `htmy/i18n.py`: `I18n`, its error hierarchy, and the resource loader | 7.6 |  | 0.594 |
| walker |  | 7829 | 67 | python decl htmy/function_component.py:244 |  |  | 0.594 |
| walker |  | 7835 | 6 | python body htmy/typing.py:40 |  |  | 0.595 |
| walker |  | 7973 | 138 | python decl htmy/md/core.py:20 |  |  | 0.601 |
| walker |  | 7979 | 6 | python decl htmy/md/core.py:33 |  |  | 0.602 |
| walker |  | 8000 | 21 | README.md section #26 |  |  | 0.603 |
| walker |  | 8021 | 21 | README.md section #39 |  |  | 0.603 |
| ns | 8033 |  | 232 | `htmy/tag.py`: how every `html.*` tag is actually built | 7.7 |  | 0.605 |
| walker |  | 8043 | 22 | python doc htmy/i18n.py:24 |  |  | 0.605 |
| walker |  | 8187 | 144 | python decl htmy/renderer/baseline.py:18 |  |  | 0.610 |
| ns | 8234 |  | 201 | `htmy/error_boundary.py` in outline: `ErrorBoundary` | 7.8 |  | 0.604 |
| walker |  | 8242 | 55 | python decl htmy/renderer/baseline.py:30 |  |  | 0.604 |
| walker |  | 8248 | 6 | python body htmy/renderer/typing.py:39 |  |  | 0.604 |
| walker |  | 8294 | 46 | python decl htmy/renderer/default.py:205 |  |  | 0.604 |
| walker |  | 8351 | 57 | README.md section #50 |  |  | 0.604 |
| ns | 8481 |  | 247 | `htmy/etree.py`: `ETreeConverter` and the optional-`lxml` import switch | 7.9 |  | 0.603 |
| walker |  | 8568 | 217 | headings outline in docs/index.md |  |  | 0.603 |
| walker |  | 8575 | 7 | docs/function-components.md section #14 |  |  | 0.603 |
| walker |  | 8581 | 6 | python body htmy/typing.py:48 |  |  | 0.603 |
| walker |  | 8641 | 60 | python doc htmy/error_boundary.py:15 |  |  | 0.610 |
| walker |  | 8662 | 21 | python doc htmy/utils.py:73 |  |  | 0.610 |
| ns | 8704 |  | 223 | `htmy/utils.py` and `htmy/io.py`: every helper, by signature | 7.10 |  | 0.614 |
| walker |  | 8717 | 55 | python decl htmy/tag.py:27 |  |  | 0.614 |
| walker |  | 8743 | 26 | README.md section #38 |  |  | 0.614 |
| walker |  | 8775 | 32 | python doc htmy/renderer/context.py:6 |  |  | 0.617 |
| walker |  | 8832 | 57 | docs/index.md section #0 |  |  | 0.617 |
| ns | 8846 |  | 142 | `html.py` tag roster 1/5: document skeleton, `Link` and `Meta` factories | 8.1 |  | 0.623 |
| walker |  | 8933 | 101 | python decl htmy/md/core.py:103 |  |  | 0.629 |
| walker |  | 8960 | 27 | headings outline in docs/api/md.md |  |  | 0.629 |
| walker |  | 8981 | 21 | docs/api/md.md section #0 |  |  | 0.629 |
| walker |  | 9016 | 35 | python body htmy/error_boundary.py:25 |  |  | 0.629 |
| walker |  | 9049 | 33 | docs/api/core.md section #0 |  |  | 0.629 |
| walker |  | 9082 | 33 | docs/api/html.md section #0 |  |  | 0.629 |
| ns | 9083 |  | 237 | `html.py` tag roster 2/5: sectioning and container tags | 8.2 | 8.1 | 0.618 |
| walker |  | 9115 | 33 | docs/api/utils.md section #0 |  |  | 0.618 |
| walker |  | 9185 | 70 | README.md section #49 |  |  | 0.618 |
| walker |  | 9219 | 34 | docs/api/etree.md section #0 |  |  | 0.618 |
| ns | 9248 |  | 165 | `html.py` tag roster 3/5: form and interactive tags | 8.3 | 8.1 | 0.610 |
| walker |  | 9253 | 34 | docs/api/function_component.md section #0 |  |  | 0.610 |
| walker |  | 9287 | 34 | docs/api/snippet.md section #0 |  |  | 0.610 |
| walker |  | 9321 | 34 | docs/api/typing.md section #0 |  |  | 0.610 |
| walker |  | 9356 | 35 | docs/api/i18n.md section #0 |  |  | 0.610 |
| walker |  | 9362 | 6 | python body htmy/typing.py:76 |  |  | 0.611 |
| walker |  | 9437 | 75 | python decl htmy/renderer/default.py:20 |  |  | 0.611 |
| ns | 9572 |  | 324 | `html.py` tag roster 4/5: text-level tags and embedded media | 8.4 | 8.1 | 0.597 |
| ns | 9917 |  | 345 | `html.py` tag roster 5/5: lists, tables, headings, media, `entity` | 8.5 | 8.1 | 0.582 |
| walker |  | 9942 | 505 | python decl htmy/snippet.py:27 |  |  | 0.589 |

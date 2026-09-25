Score(3000)=0.627 I=0.845 C=0.466 ns_rows≤3K=19/49 grid(1000/1442/2080/3000/4327/6240/9000)=0.835/0.766/0.761/0.627/0.577/0.559/0.590

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
| walker |  | 1103 | 97 | [dependencies] in pyproject.toml |  |  | 0.746 |
| ns | 1160 |  | 146 | `htmy/__init__.py` exports, part 4: `utils` helpers and the two aliases | 1.9 | 1.8 | 0.693 |
| walker |  | 1345 | 242 | python names htmy/__init__.py #1 |  |  | 0.813 |
| walker |  | 1421 | 76 | python names htmy/tag.py |  |  | 0.813 |
| ns | 1424 |  | 264 | README "Built-in components": one line of semantics per built-in | 2.1 |  | 0.766 |
| walker |  | 1478 | 57 | python decl htmy/tag.py:84 |  |  | 0.766 |
| walker |  | 1539 | 61 | python decl htmy/tag.py:56 |  |  | 0.766 |
| ns | 1612 |  | 188 | README: the definition of a component (duck-typed `htmy()` method) | 2.2 |  | 0.747 |
| walker |  | 1615 | 76 | python decl htmy/error_boundary.py:15 |  |  | 0.747 |
| walker |  | 1692 | 77 | python decl htmy/tag.py:73 |  |  | 0.748 |
| walker |  | 1748 | 56 | headings outline in docs/components-guide.md |  |  | 0.748 |
| walker |  | 1748 | 0 | docs/components-guide.md section #0 |  |  | 0.748 |
| ns | 1758 |  | 146 | README "Rendering": how to actually invoke the renderer | 2.3 |  | 0.723 |
| walker |  | 1763 | 15 | python doc htmy/io.py:11 |  |  | 0.723 |
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
| walker |  | 3467 | 11 | README.md section #14 |  |  | 0.600 |
| walker |  | 3483 | 16 | README.md section #4 |  |  | 0.601 |
| walker |  | 3499 | 16 | README.md section #7 |  |  | 0.602 |
| walker |  | 3513 | 14 | README.md section #8 |  |  | 0.603 |
| walker |  | 3530 | 17 | README.md section #13 |  |  | 0.606 |
| walker |  | 3549 | 19 | README.md section #3 |  |  | 0.609 |
| walker |  | 3570 | 21 | README.md section #9 |  |  | 0.612 |
| walker |  | 3591 | 21 | README.md section #12 |  |  | 0.618 |
| ns | 3649 |  | 314 | `htmy/typing.py` lines 34-67: the component protocols and the `Component` union | 4.2 | 4.1 | 0.584 |
| walker |  | 3736 | 145 | python names htmy/i18n.py |  |  | 0.585 |
| walker |  | 3745 | 9 | python decl htmy/i18n.py:115 |  |  | 0.585 |
| walker |  | 3817 | 72 | python decl htmy/md/core.py:81 |  |  | 0.585 |
| walker |  | 3894 | 77 | python names htmy/renderer/typing.py |  |  | 0.585 |
| walker |  | 3921 | 27 | python decl htmy/renderer/typing.py:12 |  |  | 0.585 |
| walker |  | 3949 | 28 | python decl htmy/renderer/typing.py:36 |  |  | 0.585 |
| ns | 3950 |  | 301 | `htmy/typing.py` lines 70-109: context providers, `TextProcessor`, `TextResolver` | 4.3 | 4.1 | 0.551 |
| walker |  | 3959 | 10 | python doc htmy/renderer/typing.py:12 |  |  | 0.552 |
| walker |  | 3970 | 11 | python doc htmy/renderer/typing.py:36 |  |  | 0.552 |
| walker |  | 3992 | 22 | README.md section #5 |  |  | 0.555 |
| walker |  | 4009 | 17 | python doc htmy/utils.py:67 |  |  | 0.555 |
| walker |  | 4032 | 23 | README.md section #1 |  |  | 0.560 |
| walker |  | 4053 | 21 | README.md section #2 |  |  | 0.567 |
| ns | 4085 |  | 135 | `htmy/renderer/__init__.py` in full: which class each renderer name resolves to | 5.1 |  | 0.567 |
| walker |  | 4087 | 34 | listing of 'docs/examples' |  |  | 0.567 |
| walker |  | 4120 | 33 | README.md section #52 |  |  | 0.567 |
| walker |  | 4206 | 86 | python names htmy/md/typing.py |  |  | 0.568 |
| walker |  | 4231 | 25 | python decl htmy/md/typing.py:14 |  |  | 0.568 |
| walker |  | 4242 | 11 | python doc htmy/md/typing.py:14 |  |  | 0.568 |
| walker |  | 4268 | 26 | python body htmy/io.py:11 |  |  | 0.568 |
| walker |  | 4292 | 24 | README.md section #11 |  |  | 0.574 |
| walker |  | 4317 | 25 | README.md section #6 |  |  | 0.584 |
| ns | 4327 |  | 242 | `htmy/renderer/typing.py`: all four symbols of the renderer protocol module | 5.2 |  | 0.577 |
| walker |  | 4489 | 172 | python decl htmy/core.py:201 |  |  | 0.578 |
| ns | 4508 |  | 181 | `Renderer` in `htmy/renderer/default.py`: strategy docstring and `__init__` | 5.3 |  | 0.564 |
| walker |  | 4547 | 58 | python decl htmy/core.py:222 |  |  | 0.564 |
| ns | 4695 |  | 187 | `Renderer.render()` in `default.py`: context layering and renderer self-registration | 5.4 | 5.3 | 0.555 |
| walker |  | 4883 | 336 | python names htmy/typing.py |  |  | 0.575 |
| walker |  | 4899 | 16 | python decl htmy/typing.py:73 |  |  | 0.576 |
| walker |  | 4916 | 17 | python decl htmy/typing.py:81 |  |  | 0.578 |
| ns | 4933 |  | 238 | `Renderer` in `htmy/renderer/baseline.py`: why it exists, plus its full method roster | 5.5 |  | 0.562 |
| walker |  | 4935 | 19 | python decl htmy/typing.py:103 |  |  | 0.563 |
| walker |  | 4957 | 22 | python decl htmy/typing.py:37 |  |  | 0.565 |
| walker |  | 4980 | 23 | python decl htmy/typing.py:45 |  |  | 0.568 |
| walker |  | 4991 | 11 | python doc htmy/typing.py:73 |  |  | 0.570 |
| walker |  | 5002 | 11 | python doc htmy/typing.py:81 |  |  | 0.572 |
| walker |  | 5057 | 55 | python decl htmy/tag.py:24 |  |  | 0.572 |
| ns | 5073 |  | 140 | `htmy/renderer/context.py`: `RendererContext.from_context()` | 5.6 |  | 0.562 |
| ns | 5298 |  | 225 | `core.py`: `Fragment` and `WithContext` | 6.1 |  | 0.555 |
| walker |  | 5318 | 261 | python names htmy/html.py |  |  | 0.556 |
| walker |  | 5336 | 18 | python decl htmy/html.py:63 |  |  | 0.556 |
| walker |  | 5344 | 8 | python decl htmy/html.py:66 |  |  | 0.556 |
| walker |  | 5368 | 24 | python decl htmy/html.py:13 |  |  | 0.556 |
| walker |  | 5376 | 8 | python doc htmy/html.py:13 |  |  | 0.556 |
| walker |  | 5386 | 10 | python doc htmy/html.py:63 |  |  | 0.556 |
| walker |  | 5489 | 103 | python decl htmy/html.py:79 |  |  | 0.556 |
| walker |  | 5497 | 8 | python decl htmy/html.py:82 |  |  | 0.556 |
| walker |  | 5505 | 8 | python decl htmy/html.py:86 |  |  | 0.556 |
| walker |  | 5513 | 8 | python decl htmy/html.py:90 |  |  | 0.556 |
| walker |  | 5521 | 8 | python decl htmy/html.py:94 |  |  | 0.556 |
| walker |  | 5529 | 8 | python decl htmy/html.py:98 |  |  | 0.556 |
| walker |  | 5545 | 16 | README.md section #41 |  |  | 0.556 |
| ns | 5550 |  | 252 | `core.py`: `ContextAware`, the typed context registration base class | 6.2 |  | 0.549 |
| walker |  | 5551 | 6 | python body htmy/renderer/typing.py:15 |  |  | 0.549 |
| walker |  | 5606 | 55 | python decl htmy/renderer/default.py:238 |  |  | 0.554 |
| walker |  | 5634 | 28 | listing of 'tests/renderer' |  |  | 0.567 |
| walker |  | 5696 | 62 | python decl htmy/tag.py:12 |  |  | 0.567 |
| ns | 5823 |  | 273 | `core.py`: `SkipProperty`, `Text`, `SafeStr`, `XBool`, `xml_format_string` | 6.3 |  | 0.557 |
| walker |  | 5920 | 224 | python names htmy/function_component.py |  |  | 0.557 |
| walker |  | 5936 | 16 | python decl htmy/function_component.py:31 |  |  | 0.557 |
| walker |  | 5954 | 18 | python decl htmy/function_component.py:46 |  |  | 0.557 |
| walker |  | 5972 | 18 | python doc htmy/utils.py:55 |  |  | 0.557 |
| walker |  | 6003 | 31 | README.md section #10 |  |  | 0.562 |
| ns | 6113 |  | 290 | `core.py`: `Formatter` -- construction and the complete method surface | 6.4 |  | 0.558 |
| walker |  | 6233 | 230 | python decl htmy/i18n.py:24 |  |  | 0.559 |
| walker |  | 6240 | 7 | python decl htmy/i18n.py:48 |  |  | 0.559 |
| walker |  | 6249 | 9 | python decl htmy/i18n.py:45 |  |  | 0.559 |
| walker |  | 6257 | 8 | python decl htmy/i18n.py:79 |  |  | 0.559 |
| ns | 6287 |  | 174 | `core.py`: the default value-formatter table | 6.5 | 6.4 | 0.551 |
| walker |  | 6303 | 46 | README.md section #46 |  |  | 0.551 |
| ns | 6480 |  | 193 | `htmy/snippet.py`: `Slots` and the slot placeholder syntax | 7.1 |  | 0.543 |
| walker |  | 6536 | 233 | python decl htmy/etree.py:23 |  |  | 0.543 |
| walker |  | 6547 | 11 | python body htmy/etree.py:46 |  |  | 0.543 |
| walker |  | 6564 | 17 | python doc htmy/etree.py:63 |  |  | 0.543 |
| walker |  | 6585 | 21 | python doc htmy/etree.py:55 |  |  | 0.543 |
| walker |  | 6657 | 72 | README.md section #16 |  |  | 0.543 |
| ns | 6772 |  | 292 | `htmy/snippet.py`: `Snippet` and its four-step text pipeline | 7.2 |  | 0.537 |
| walker |  | 6919 | 262 | python decl htmy/function_component.py:71 |  |  | 0.538 |
| walker |  | 6926 | 7 | python decl htmy/function_component.py:83 |  |  | 0.538 |
| walker |  | 6933 | 7 | python decl htmy/function_component.py:144 |  |  | 0.538 |
| walker |  | 6942 | 9 | python decl htmy/function_component.py:80 |  |  | 0.538 |
| walker |  | 6951 | 9 | python decl htmy/function_component.py:141 |  |  | 0.538 |
| walker |  | 6991 | 40 | python decl htmy/function_component.py:189 |  |  | 0.538 |
| walker |  | 7031 | 40 | python decl htmy/function_component.py:184 |  |  | 0.538 |
| ns | 7059 |  | 287 | `htmy/function_component.py`: the complete `@component` decorator family | 7.3 |  | 0.539 |
| walker |  | 7071 | 40 | python decl htmy/function_component.py:310 |  |  | 0.539 |
| walker |  | 7111 | 40 | python decl htmy/function_component.py:305 |  |  | 0.546 |
| walker |  | 7157 | 46 | python decl htmy/function_component.py:239 |  |  | 0.546 |
| walker |  | 7203 | 46 | python decl htmy/function_component.py:234 |  |  | 0.554 |
| ns | 7233 |  | 174 | `htmy/md/core.py`: `MarkdownParser` | 7.4 |  | 0.547 |
| walker |  | 7256 | 53 | python decl htmy/function_component.py:86 |  |  | 0.547 |
| walker |  | 7309 | 53 | python decl htmy/function_component.py:147 |  |  | 0.547 |
| walker |  | 7362 | 53 | python decl htmy/function_component.py:194 |  |  | 0.547 |
| walker |  | 7417 | 55 | python decl htmy/function_component.py:315 |  |  | 0.547 |
| walker |  | 7484 | 67 | python decl htmy/function_component.py:244 |  |  | 0.547 |
| walker |  | 7490 | 6 | python body htmy/typing.py:40 |  |  | 0.547 |
| ns | 7542 |  | 309 | `htmy/md/core.py`: the `MD` component, plus the `md/typing.py` definitions | 7.5 |  | 0.540 |
| walker |  | 7628 | 138 | python decl htmy/md/core.py:20 |  |  | 0.547 |
| walker |  | 7634 | 6 | python decl htmy/md/core.py:33 |  |  | 0.549 |
| walker |  | 7655 | 21 | README.md section #26 |  |  | 0.549 |
| walker |  | 7676 | 21 | README.md section #39 |  |  | 0.549 |
| walker |  | 7698 | 22 | python doc htmy/i18n.py:24 |  |  | 0.549 |
| ns | 7801 |  | 259 | `htmy/i18n.py`: `I18n`, its error hierarchy, and the resource loader | 7.6 |  | 0.559 |
| walker |  | 7842 | 144 | python decl htmy/renderer/baseline.py:18 |  |  | 0.564 |
| walker |  | 7897 | 55 | python decl htmy/renderer/baseline.py:30 |  |  | 0.565 |
| walker |  | 7903 | 6 | python body htmy/renderer/typing.py:39 |  |  | 0.565 |
| walker |  | 7949 | 46 | python decl htmy/renderer/default.py:205 |  |  | 0.565 |
| walker |  | 8006 | 57 | README.md section #50 |  |  | 0.565 |
| ns | 8033 |  | 232 | `htmy/tag.py`: how every `html.*` tag is actually built | 7.7 |  | 0.567 |
| walker |  | 8223 | 217 | headings outline in docs/index.md |  |  | 0.567 |
| walker |  | 8230 | 7 | docs/function-components.md section #14 |  |  | 0.567 |
| ns | 8234 |  | 201 | `htmy/error_boundary.py` in outline: `ErrorBoundary` | 7.8 |  | 0.562 |
| walker |  | 8236 | 6 | python body htmy/typing.py:48 |  |  | 0.562 |
| walker |  | 8296 | 60 | python doc htmy/error_boundary.py:15 |  |  | 0.570 |
| walker |  | 8317 | 21 | python doc htmy/utils.py:73 |  |  | 0.570 |
| walker |  | 8372 | 55 | python decl htmy/tag.py:27 |  |  | 0.570 |
| walker |  | 8398 | 26 | README.md section #38 |  |  | 0.570 |
| walker |  | 8430 | 32 | python doc htmy/renderer/context.py:6 |  |  | 0.573 |
| ns | 8481 |  | 247 | `htmy/etree.py`: `ETreeConverter` and the optional-`lxml` import switch | 7.9 |  | 0.572 |
| walker |  | 8487 | 57 | docs/index.md section #0 |  |  | 0.572 |
| walker |  | 8588 | 101 | python decl htmy/md/core.py:103 |  |  | 0.578 |
| walker |  | 8615 | 27 | headings outline in docs/api/md.md |  |  | 0.578 |
| walker |  | 8636 | 21 | docs/api/md.md section #0 |  |  | 0.578 |
| walker |  | 8671 | 35 | python body htmy/error_boundary.py:25 |  |  | 0.578 |
| walker |  | 8704 | 33 | docs/api/core.md section #0 |  |  | 0.583 |
| ns | 8704 |  | 223 | `htmy/utils.py` and `htmy/io.py`: every helper, by signature | 7.10 |  | 0.583 |
| walker |  | 8737 | 33 | docs/api/html.md section #0 |  |  | 0.583 |
| walker |  | 8770 | 33 | docs/api/utils.md section #0 |  |  | 0.583 |
| walker |  | 8840 | 70 | README.md section #49 |  |  | 0.583 |
| ns | 8846 |  | 142 | `html.py` tag roster 1/5: document skeleton, `Link` and `Meta` factories | 8.1 |  | 0.590 |
| walker |  | 8874 | 34 | docs/api/etree.md section #0 |  |  | 0.590 |
| walker |  | 8908 | 34 | docs/api/function_component.md section #0 |  |  | 0.590 |
| walker |  | 8942 | 34 | docs/api/snippet.md section #0 |  |  | 0.590 |
| walker |  | 8976 | 34 | docs/api/typing.md section #0 |  |  | 0.590 |
| walker |  | 9011 | 35 | docs/api/i18n.md section #0 |  |  | 0.590 |
| walker |  | 9017 | 6 | python body htmy/typing.py:76 |  |  | 0.591 |
| ns | 9083 |  | 237 | `html.py` tag roster 2/5: sectioning and container tags | 8.2 | 8.1 | 0.581 |
| walker |  | 9092 | 75 | python decl htmy/renderer/default.py:20 |  |  | 0.581 |
| ns | 9248 |  | 165 | `html.py` tag roster 3/5: form and interactive tags | 8.3 | 8.1 | 0.574 |
| ns | 9572 |  | 324 | `html.py` tag roster 4/5: text-level tags and embedded media | 8.4 | 8.1 | 0.560 |
| walker |  | 9597 | 505 | python decl htmy/snippet.py:27 |  |  | 0.568 |
| walker |  | 9663 | 66 | python decl htmy/snippet.py:83 |  |  | 0.569 |
| walker |  | 9676 | 13 | python body htmy/snippet.py:110 |  |  | 0.569 |
| walker |  | 9698 | 22 | python body htmy/tag.py:84 |  |  | 0.569 |
| walker |  | 9704 | 6 | python body htmy/typing.py:84 |  |  | 0.570 |
| walker |  | 9740 | 36 | README.md section #36 |  |  | 0.570 |
| walker |  | 9841 | 101 | README.md section #43 |  |  | 0.570 |
| walker |  | 9851 | 10 | python doc htmy/html.py:79 |  |  | 0.570 |
| walker |  | 9884 | 33 | docs/index.md section #50 |  |  | 0.570 |
| ns | 9917 |  | 345 | `html.py` tag roster 5/5: lists, tables, headings, media, `entity` | 8.5 | 8.1 | 0.556 |
| walker |  | 9981 | 97 | README.md section #45 |  |  | 0.556 |
| walker |  | 9999 | 18 | python doc htmy/renderer/typing.py:29 |  |  | 0.557 |

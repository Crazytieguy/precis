Score(3000)=0.523 I=0.810 C=0.337 ns_rows≤3K=20/52 (reached=8 partial=1 missing=11)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 50 | 50 | listing of '.' |  |  | 1.000 |
| ns | 50 |  | 50 | Top-level directory listing | 1.1 |  | 1.000 |
| walker |  | 53 | 3 | listing of 'src' |  |  | 1.000 |
| ns | 92 |  | 42 | Package name + version | 1.2 |  | 0.859 |
| walker |  | 125 | 72 | README headline in README.md |  |  | 0.887 |
| walker |  | 155 | 30 | headings outline in README.md |  |  | 0.888 |
| ns | 169 |  | 77 | README headline + tagline | 1.3 |  | 0.861 |
| walker |  | 182 | 27 | [dependencies] in pyproject.toml |  |  | 0.870 |
| ns | 209 |  | 40 | Python version floor + runtime dep | 1.4 | 1.2 | 0.834 |
| walker |  | 268 | 86 | listing of 'src/click' |  |  | 0.834 |
| walker |  | 279 | 11 | python imports #1 in src/click/__init__.py |  |  | 0.834 |
| walker |  | 297 | 18 | python decl names surface in src/click/__init__.py |  |  | 0.834 |
| walker |  | 297 | 0 | python decl at src/click/__init__.py:77 |  |  | 0.834 |
| ns | 310 |  | 101 | README differentiators (three-bullet pitch) | 1.5 | 1.3 | 0.705 |
| walker |  | 370 | 73 | python imports in src/click/__init__.py |  |  | 0.705 |
| ns | 410 |  | 100 | __init__.py: core-class re-exports | 1.6 |  | 0.627 |
| walker |  | 461 | 91 | README.md section #0 |  |  | 0.739 |
| walker |  | 477 | 16 | python imports #6 in src/click/__init__.py |  |  | 0.739 |
| walker |  | 490 | 13 | listing of '.github' |  |  | 0.739 |
| walker |  | 513 | 23 | listing of '.github/workflows' |  |  | 0.739 |
| walker |  | 527 | 14 | python decl names surface in src/click/_textwrap.py |  |  | 0.739 |
| walker |  | 527 | 0 | python decl at src/click/_textwrap.py:8 |  |  | 0.739 |
| walker |  | 557 | 30 | python imports #5 in src/click/__init__.py |  |  | 0.740 |
| ns | 578 |  | 168 | README hello-world example | 1.7 | 1.3 | 0.618 |
| walker |  | 635 | 78 | [package] in pyproject.toml |  |  | 0.704 |
| walker |  | 735 | 100 | python imports #2 in src/click/__init__.py |  |  | 0.798 |
| ns | 739 |  | 161 | __init__.py: decorator re-exports | 1.8 | 1.6 | 0.723 |
| walker |  | 776 | 41 | python method sigs in src/click/_textwrap.py |  |  | 0.723 |
| walker |  | 776 | 0 | python method at src/click/_textwrap.py:40 |  |  | 0.723 |
| walker |  | 795 | 19 | python method at src/click/_textwrap.py:27 |  |  | 0.723 |
| walker |  | 862 | 67 | README.md section #3 |  |  | 0.723 |
| ns | 931 |  | 192 | __init__.py: exception + formatting + globals re-exports | 1.9 | 1.8 | 0.660 |
| walker |  | 944 | 82 | README.md section #2 |  |  | 0.660 |
| walker |  | 980 | 36 | listing of 'examples' |  |  | 0.662 |
| ns | 1120 |  | 189 | __init__.py: termui re-exports | 1.10 | 1.9 | 0.608 |
| walker |  | 1172 | 192 | listing of 'docs' |  |  | 0.608 |
| walker |  | 1194 | 22 | docs/setuptools.md section #0 |  |  | 0.608 |
| walker |  | 1209 | 15 | listing of 'docs/_static' |  |  | 0.608 |
| walker |  | 1257 | 48 | docs/license.md section #0 |  |  | 0.608 |
| walker |  | 1278 | 21 | headings outline in docs/click-concepts.md |  |  | 0.608 |
| ns | 1302 |  | 182 | __init__.py: built-in type re-exports | 1.11 | 1.10 | 0.562 |
| walker |  | 1343 | 65 | docs/click-concepts.md section #0 |  |  | 0.562 |
| walker |  | 1364 | 21 | headings outline in docs/unicode-support.md |  |  | 0.562 |
| ns | 1395 |  | 93 | __init__.py: utils re-exports | 1.12 | 1.11 | 0.543 |
| walker |  | 1525 | 161 | python imports #3 in src/click/__init__.py |  |  | 0.618 |
| walker |  | 1547 | 22 | headings outline in docs/command-line-reference.md |  |  | 0.618 |
| walker |  | 1615 | 68 | docs/command-line-reference.md section #0 |  |  | 0.618 |
| walker |  | 1637 | 22 | headings outline in docs/contrib.md |  |  | 0.618 |
| ns | 1647 |  | 252 | Deprecation shim: BaseCommand → Command, MultiCommand → Group | 1.13 | 1.12 | 0.555 |
| walker |  | 1659 | 22 | headings outline in docs/design-opinions.md |  |  | 0.555 |
| walker |  | 1719 | 60 | docs/design-opinions.md section #0 |  |  | 0.555 |
| walker |  | 1865 | 146 | python imports #4 in src/click/__init__.py |  |  | 0.625 |
| walker |  | 1887 | 22 | python decl names surface #1 in src/click/exceptions.py |  |  | 0.625 |
| walker |  | 1887 | 0 | python decl at src/click/exceptions.py:330 |  |  | 0.625 |
| walker |  | 1887 | 0 | python decl at src/click/exceptions.py:334 |  |  | 0.625 |
| walker |  | 1901 | 14 | python class body at src/click/exceptions.py:334 |  |  | 0.625 |
| walker |  | 1915 | 14 | python decl doc at src/click/exceptions.py:330 |  |  | 0.625 |
| ns | 1916 |  | 269 | Deprecation shim: OptionParser, __version__ | 1.14 | 1.13 | 0.569 |
| walker |  | 1938 | 23 | python method sigs #1 in src/click/exceptions.py |  |  | 0.569 |
| walker |  | 1938 | 0 | python method at src/click/exceptions.py:343 |  |  | 0.569 |
| walker |  | 1949 | 11 | python method body at src/click/exceptions.py:343 body 344 |  |  | 0.569 |
| walker |  | 1995 | 46 | python decl doc at src/click/exceptions.py:334 |  |  | 0.569 |
| walker |  | 2022 | 27 | python imports in src/click/_utils.py |  |  | 0.569 |
| walker |  | 2052 | 30 | headings outline in docs/entry-points.md |  |  | 0.569 |
| walker |  | 2064 | 12 | listing of 'examples/complex' |  |  | 0.569 |
| walker |  | 2077 | 13 | listing of 'examples/complex/complex' |  |  | 0.569 |
| walker |  | 2093 | 16 | listing of 'examples/complex/complex/commands' |  |  | 0.569 |
| ns | 2121 |  | 205 | decorators.py: location of every public decorator | 2.1 |  | 0.546 |
| ns | 2209 |  | 88 | @command: signature + 1-liner | 2.2 | 2.1 | 0.536 |
| walker |  | 2295 | 202 | README.md section #1 |  |  | 0.605 |
| walker |  | 2327 | 32 | headings outline in docs/extending-click.md |  |  | 0.605 |
| ns | 2367 |  | 158 | @option: signature + 1-liner | 2.3 | 2.1 | 0.586 |
| walker |  | 2433 | 106 | docs/extending-click.md section #0 |  |  | 0.586 |
| walker |  | 2466 | 33 | headings outline in docs/faqs.md |  |  | 0.586 |
| walker |  | 2499 | 33 | docs/faqs.md section #0 |  |  | 0.586 |
| ns | 2525 |  | 158 | @argument: signature + 1-liner | 2.4 | 2.1 | 0.570 |
| walker |  | 2532 | 33 | headings outline in docs/wincmd.md |  |  | 0.570 |
| walker |  | 2606 | 74 | docs/wincmd.md section #0 |  |  | 0.570 |
| walker |  | 2641 | 35 | headings outline in docs/virtualenv.md |  |  | 0.570 |
| walker |  | 2641 | 0 | docs/virtualenv.md section #0 |  |  | 0.570 |
| walker |  | 2678 | 37 | headings outline in docs/parameters.md |  |  | 0.570 |
| ns | 2688 |  | 163 | @group: signature + 1-liner | 2.5 | 2.1 | 0.551 |
| walker |  | 2717 | 39 | docs/parameters.md section #0 |  |  | 0.551 |
| walker |  | 2759 | 42 | headings outline in docs/option-decorators.md |  |  | 0.551 |
| walker |  | 2787 | 28 | docs/option-decorators.md section #3 |  |  | 0.551 |
| walker |  | 2864 | 77 | docs/option-decorators.md section #0 |  |  | 0.551 |
| walker |  | 2907 | 43 | headings outline in docs/handling-files.md |  |  | 0.551 |
| ns | 2923 |  | 235 | @pass_context / @pass_obj signatures + docstrings | 2.6 | 2.1 | 0.523 |
| walker |  | 2971 | 64 | docs/handling-files.md section #0 |  |  | 0.523 |
| walker |  | 3013 | 42 | python imports in src/click/_textwrap.py |  |  | 0.523 |
| walker |  | 3106 | 93 | python decl names surface in src/click/formatting.py |  |  | 0.523 |
| walker |  | 3106 | 0 | python decl at src/click/formatting.py:14 |  |  | 0.523 |
| walker |  | 3106 | 0 | python decl at src/click/formatting.py:104 |  |  | 0.523 |
| walker |  | 3106 | 0 | python decl at src/click/formatting.py:283 |  |  | 0.523 |
| walker |  | 3141 | 35 | python decl at src/click/formatting.py:24 |  |  | 0.523 |
| walker |  | 3204 | 63 | python decl at src/click/formatting.py:31 |  |  | 0.523 |
| walker |  | 3233 | 29 | python decl body at src/click/formatting.py:24 body 27 |  |  | 0.523 |
| ns | 3240 |  | 317 | @make_pass_decorator: signature + docstring + how-it-works | 2.7 | 2.1 | 0.493 |
| walker |  | 3314 | 81 | python decl doc at src/click/formatting.py:283 |  |  | 0.493 |
| walker |  | 3409 | 95 | python decl names surface in src/click/testing.py |  |  | 0.494 |
| walker |  | 3409 | 0 | python decl at src/click/testing.py:26 |  |  | 0.494 |
| walker |  | 3409 | 0 | python decl at src/click/testing.py:70 |  |  | 0.494 |
| walker |  | 3409 | 0 | python decl at src/click/testing.py:89 |  |  | 0.494 |
| walker |  | 3409 | 0 | python decl at src/click/testing.py:103 |  |  | 0.494 |
| walker |  | 3409 | 0 | python decl at src/click/testing.py:183 |  |  | 0.494 |
| walker |  | 3409 | 0 | python decl at src/click/testing.py:261 |  |  | 0.494 |
| walker |  | 3452 | 43 | python decl at src/click/testing.py:163 |  |  | 0.494 |
| walker |  | 3474 | 22 | python decl at src/click/testing.py:60 |  |  | 0.494 |
| ns | 3498 |  | 258 | @confirmation_option / @password_option full bodies | 2.8 | 2.1 | 0.469 |
| walker |  | 3514 | 40 | python decl doc at src/click/testing.py:70 |  |  | 0.469 |
| walker |  | 3565 | 51 | python decl doc at src/click/testing.py:89 |  |  | 0.469 |
| walker |  | 3616 | 51 | headings outline in docs/arguments.md |  |  | 0.469 |
| ns | 3738 |  | 240 | @version_option signature + key kwargs | 2.9 | 2.1 | 0.453 |
| walker |  | 3750 | 134 | docs/arguments.md section #0 |  |  | 0.453 |
| walker |  | 3875 | 125 | python decl doc at src/click/formatting.py:104 |  |  | 0.453 |
| walker |  | 4067 | 192 | python method sigs in src/click/formatting.py |  |  | 0.453 |
| walker |  | 4067 | 0 | python method at src/click/formatting.py:135 |  |  | 0.453 |
| walker |  | 4067 | 0 | python method at src/click/formatting.py:139 |  |  | 0.453 |
| walker |  | 4067 | 0 | python method at src/click/formatting.py:143 |  |  | 0.453 |
| walker |  | 4067 | 0 | python method at src/click/formatting.py:147 |  |  | 0.453 |
| walker |  | 4067 | 0 | python method at src/click/formatting.py:185 |  |  | 0.453 |
| walker |  | 4067 | 0 | python method at src/click/formatting.py:189 |  |  | 0.453 |
| walker |  | 4067 | 0 | python method at src/click/formatting.py:194 |  |  | 0.453 |
| walker |  | 4067 | 0 | python method at src/click/formatting.py:278 |  |  | 0.453 |
| walker |  | 4081 | 14 | python method at src/click/formatting.py:269 |  |  | 0.453 |
| ns | 4085 |  | 347 | quickstart.md: 'Basic Concepts - Creating a Command' section | 2.10 |  | 0.427 |
| walker |  | 4099 | 18 | python method at src/click/formatting.py:254 |  |  | 0.427 |
| walker |  | 4108 | 9 | python method doc at src/click/formatting.py:139 |  |  | 0.427 |
| walker |  | 4117 | 9 | python method doc at src/click/formatting.py:143 |  |  | 0.427 |
| walker |  | 4126 | 9 | python method doc at src/click/formatting.py:278 |  |  | 0.427 |
| walker |  | 4137 | 11 | python method doc at src/click/formatting.py:185 |  |  | 0.427 |
| walker |  | 4148 | 11 | python method doc at src/click/formatting.py:189 |  |  | 0.427 |
| walker |  | 4161 | 13 | python method doc at src/click/formatting.py:135 |  |  | 0.427 |
| walker |  | 4175 | 14 | python method doc at src/click/formatting.py:269 |  |  | 0.427 |
| walker |  | 4185 | 10 | python method body at src/click/formatting.py:135 body 137 |  |  | 0.427 |
| walker |  | 4196 | 11 | python method body at src/click/formatting.py:278 body 280 |  |  | 0.427 |
| ns | 4205 |  | 120 | core.py: every class header (top-level inventory) | 3.1 |  | 0.420 |
| walker |  | 4210 | 14 | python method body at src/click/formatting.py:139 body 141 |  |  | 0.420 |
| walker |  | 4224 | 14 | python method body at src/click/formatting.py:143 body 145 |  |  | 0.420 |
| walker |  | 4278 | 54 | python method at src/click/formatting.py:116 |  |  | 0.420 |
| walker |  | 4297 | 19 | python method body at src/click/formatting.py:189 body 191 |  |  | 0.420 |
| ns | 4349 |  | 144 | Context: one-paragraph lede | 3.2 | 3.1 | 0.411 |
| walker |  | 4353 | 56 | python method at src/click/formatting.py:210 |  |  | 0.411 |
| walker |  | 4387 | 34 | python method doc at src/click/formatting.py:194 |  |  | 0.411 |
| walker |  | 4409 | 22 | python method body at src/click/formatting.py:185 body 187 |  |  | 0.411 |
| walker |  | 4460 | 51 | python method doc at src/click/formatting.py:254 |  |  | 0.411 |
| walker |  | 4512 | 52 | headings outline in docs/support-multiple-versions.md |  |  | 0.411 |
| ns | 4618 |  | 269 | Context.__init__ signature | 3.3 | 3.2 | 0.400 |
| walker |  | 4629 | 117 | docs/support-multiple-versions.md section #0 |  |  | 0.400 |
| walker |  | 4679 | 50 | python imports in src/click/globals.py |  |  | 0.400 |
| walker |  | 4806 | 127 | listing of 'tests' |  |  | 0.400 |
| walker |  | 4862 | 56 | listing of 'tests/typing' |  |  | 0.400 |
| walker |  | 4950 | 88 | python imports #9 in src/click/__init__.py |  |  | 0.416 |
| ns | 5031 |  | 413 | Context: location of every method | 3.4 | 3.2 | 0.402 |
| walker |  | 5055 | 105 | python decl names surface in src/click/_utils.py |  |  | 0.402 |
| walker |  | 5055 | 0 | python decl at src/click/_utils.py:7 |  |  | 0.402 |
| walker |  | 5071 | 16 | python method sigs in src/click/_utils.py |  |  | 0.402 |
| walker |  | 5071 | 0 | python method at src/click/_utils.py:18 |  |  | 0.402 |
| walker |  | 5093 | 22 | python class body at src/click/_utils.py:7 |  |  | 0.402 |
| walker |  | 5109 | 16 | python method body at src/click/_utils.py:18 body 19 |  |  | 0.402 |
| ns | 5156 |  | 125 | Command: one-paragraph lede + headline kwargs | 3.5 | 3.1 | 0.397 |
| walker |  | 5164 | 55 | python decl doc at src/click/_utils.py:7 |  |  | 0.397 |
| walker |  | 5269 | 105 | python decl names surface in src/click/globals.py |  |  | 0.397 |
| walker |  | 5269 | 0 | python decl at src/click/globals.py:12 |  |  | 0.397 |
| walker |  | 5269 | 0 | python decl at src/click/globals.py:16 |  |  | 0.397 |
| walker |  | 5269 | 0 | python decl at src/click/globals.py:20 |  |  | 0.397 |
| walker |  | 5269 | 0 | python decl at src/click/globals.py:44 |  |  | 0.397 |
| walker |  | 5269 | 0 | python decl at src/click/globals.py:49 |  |  | 0.397 |
| walker |  | 5269 | 0 | python decl at src/click/globals.py:54 |  |  | 0.397 |
| walker |  | 5279 | 10 | python decl body at src/click/globals.py:49 body 51 |  |  | 0.397 |
| walker |  | 5292 | 13 | python decl doc at src/click/globals.py:49 |  |  | 0.397 |
| walker |  | 5306 | 14 | python decl doc at src/click/globals.py:44 |  |  | 0.397 |
| walker |  | 5324 | 18 | python decl body at src/click/globals.py:16 body 17 |  |  | 0.397 |
| walker |  | 5343 | 19 | python decl body at src/click/globals.py:44 body 46 |  |  | 0.397 |
| ns | 5357 |  | 201 | Command.__init__ signature | 3.6 | 3.5 | 0.389 |
| walker |  | 5365 | 22 | python decl body at src/click/globals.py:12 body 13 |  |  | 0.389 |
| walker |  | 5418 | 53 | python decl doc at src/click/globals.py:54 |  |  | 0.389 |
| walker |  | 5426 | 8 | python decl body at src/click/globals.py:20 body 41 |  |  | 0.389 |
| walker |  | 5484 | 58 | headings outline in docs/shell-completion.md |  |  | 0.389 |
| walker |  | 5677 | 193 | docs/shell-completion.md section #0 |  |  | 0.389 |
| ns | 5745 |  | 388 | Command: location of every method | 3.7 | 3.5 | 0.380 |
| walker |  | 5874 | 197 | docs/contrib.md section #0 |  |  | 0.380 |
| walker |  | 5935 | 61 | headings outline in docs/why.md |  |  | 0.380 |
| walker |  | 5997 | 62 | headings outline in docs/standalone-apps.md |  |  | 0.380 |
| walker |  | 6031 | 34 | docs/standalone-apps.md section #1 |  |  | 0.380 |
| walker |  | 6095 | 64 | headings outline in docs/exceptions.md |  |  | 0.380 |
| walker |  | 6194 | 99 | docs/exceptions.md section #0 |  |  | 0.380 |
| ns | 6201 |  | 456 | Group: lede + __init__ signature | 3.8 | 3.1 | 0.366 |
| walker |  | 6403 | 209 | docs/entry-points.md section #0 |  |  | 0.366 |
| ns | 6415 |  | 214 | Group: method-locations + decorator-style command/group registrars | 3.9 | 3.8 | 0.362 |
| walker |  | 6480 | 77 | python method doc at src/click/formatting.py:147 |  |  | 0.362 |
| ns | 6567 |  | 152 | Parameter (ABC): lede + key kwargs paragraph | 3.10 | 3.1 | 0.357 |
| walker |  | 6700 | 220 | docs/standalone-apps.md section #0 |  |  | 0.357 |
| walker |  | 6827 | 127 | python decl names surface in src/click/parser.py |  |  | 0.357 |
| walker |  | 6827 | 0 | python decl at src/click/parser.py:111 |  |  | 0.357 |
| walker |  | 6827 | 0 | python decl at src/click/parser.py:120 |  |  | 0.357 |
| walker |  | 6827 | 0 | python decl at src/click/parser.py:127 |  |  | 0.357 |
| walker |  | 6827 | 0 | python decl at src/click/parser.py:185 |  |  | 0.357 |
| walker |  | 6827 | 0 | python decl at src/click/parser.py:216 |  |  | 0.357 |
| walker |  | 6827 | 0 | python decl at src/click/parser.py:224 |  |  | 0.357 |
| walker |  | 6827 | 0 | python decl at src/click/parser.py:503 |  |  | 0.357 |
| ns | 6856 |  | 289 | Parameter.__init__ signature | 3.11 | 3.10 | 0.349 |
| walker |  | 6874 | 47 | python decl at src/click/parser.py:51 |  |  | 0.349 |
| walker |  | 6943 | 69 | headings outline in docs/parameter-types.md |  |  | 0.349 |
| walker |  | 6953 | 10 | docs/parameter-types.md section #1 |  |  | 0.349 |
| walker |  | 6961 | 8 | docs/parameter-types.md section #8 |  |  | 0.349 |
| walker |  | 7074 | 113 | docs/parameter-types.md section #0 |  |  | 0.349 |
| walker |  | 7143 | 69 | headings outline in docs/prompts.md |  |  | 0.349 |
| ns | 7202 |  | 346 | Option: lede + __init__ signature | 3.12 | 3.10 | 0.340 |
| walker |  | 7303 | 160 | docs/prompts.md section #0 |  |  | 0.340 |
| ns | 7355 |  | 153 | Argument: lede + __init__ signature + required-by-default logic | 3.13 | 3.10 | 0.335 |
| walker |  | 7378 | 75 | python decl body at src/click/formatting.py:14 body 15 |  |  | 0.335 |
| walker |  | 7446 | 68 | python imports in src/click/formatting.py |  |  | 0.335 |
| walker |  | 7455 | 9 | python method body at src/click/formatting.py:269 body 272 |  |  | 0.335 |
| walker |  | 7533 | 78 | headings outline in docs/testing.md |  |  | 0.335 |
| walker |  | 7682 | 149 | docs/testing.md section #0 |  |  | 0.335 |
| ns | 7722 |  | 367 | Parameter/Option/Argument: method locations | 3.14 | 3.11 | 0.329 |
| walker |  | 7871 | 189 | python imports #7 in src/click/__init__.py |  |  | 0.358 |
| walker |  | 7947 | 76 | docs/virtualenv.md section #1 |  |  | 0.358 |
| ns | 8019 |  | 297 | types.py: every ParamType subclass header | 4.1 |  | 0.352 |
| ns | 8250 |  | 231 | ParamType: the subclassing recipe | 4.2 | 4.1 | 0.346 |
| walker |  | 8257 | 310 | python method sigs in src/click/parser.py |  |  | 0.346 |
| walker |  | 8257 | 0 | python method at src/click/parser.py:169 |  |  | 0.346 |
| walker |  | 8257 | 0 | python method at src/click/parser.py:186 |  |  | 0.346 |
| walker |  | 8257 | 0 | python method at src/click/parser.py:217 |  |  | 0.346 |
| walker |  | 8257 | 0 | python method at src/click/parser.py:290 |  |  | 0.346 |
| walker |  | 8257 | 0 | python method at src/click/parser.py:316 |  |  | 0.346 |
| walker |  | 8257 | 0 | python method at src/click/parser.py:327 |  |  | 0.346 |
| walker |  | 8257 | 0 | python method at src/click/parser.py:390 |  |  | 0.346 |
| walker |  | 8257 | 0 | python method at src/click/parser.py:470 |  |  | 0.346 |
| walker |  | 8268 | 11 | python method at src/click/parser.py:165 |  |  | 0.346 |
| walker |  | 8301 | 33 | python method at src/click/parser.py:298 |  |  | 0.346 |
| walker |  | 8337 | 36 | python method at src/click/parser.py:241 |  |  | 0.346 |
| walker |  | 8352 | 15 | python method body at src/click/parser.py:165 body 167 |  |  | 0.346 |
| walker |  | 8400 | 48 | python method at src/click/parser.py:191 |  |  | 0.346 |
| walker |  | 8421 | 21 | python method body at src/click/parser.py:290 body 296 |  |  | 0.346 |
| ns | 8431 |  | 181 | ParamType.convert + ParamType.shell_complete signatures | 4.3 | 4.2 | 0.342 |
| walker |  | 8452 | 31 | python method at src/click/parser.py:363 |  |  | 0.342 |
| walker |  | 8535 | 83 | python method at src/click/parser.py:128 |  |  | 0.342 |
| ns | 8564 |  | 133 | exceptions.py: every class header | 4.4 |  | 0.339 |
| walker |  | 8590 | 55 | python method doc at src/click/parser.py:290 |  |  | 0.339 |
| walker |  | 8619 | 29 | python method body at src/click/parser.py:186 body 187 |  |  | 0.339 |
| walker |  | 8705 | 86 | python method at src/click/parser.py:265 |  |  | 0.339 |
| walker |  | 8751 | 46 | python method at src/click/parser.py:430 |  |  | 0.339 |
| ns | 8790 |  | 226 | ClickException + UsageError ledes + exit_code | 4.5 | 4.4 | 0.333 |
| ns | 8844 |  | 54 | testing.py: CliRunner, Result, helper class locations | 5.1 |  | 0.341 |
| walker |  | 8850 | 99 | python method doc at src/click/parser.py:298 |  |  | 0.341 |
| ns | 8975 |  | 131 | CliRunner.invoke signature | 5.2 | 5.1 | 0.338 |
| walker |  | 9006 | 156 | python decl names surface in src/click/_termui_impl.py |  |  | 0.338 |
| walker |  | 9006 | 0 | python decl at src/click/_termui_impl.py:43 |  |  | 0.338 |
| walker |  | 9006 | 0 | python decl at src/click/_termui_impl.py:375 |  |  | 0.338 |
| walker |  | 9006 | 0 | python decl at src/click/_termui_impl.py:608 |  |  | 0.338 |
| walker |  | 9006 | 0 | python decl at src/click/_termui_impl.py:724 |  |  | 0.338 |
| walker |  | 9006 | 0 | python decl at src/click/_termui_impl.py:794 |  |  | 0.338 |
| walker |  | 9033 | 27 | python decl at src/click/_termui_impl.py:417 |  |  | 0.338 |
| walker |  | 9076 | 43 | python class body at src/click/_termui_impl.py:608 |  |  | 0.338 |
| walker |  | 9110 | 34 | python decl at src/click/_termui_impl.py:386 |  |  | 0.338 |
| walker |  | 9127 | 17 | python decl doc at src/click/_termui_impl.py:386 |  |  | 0.338 |
| ns | 9163 |  | 188 | Result: attribute documentation | 5.3 | 5.1 | 0.335 |
| walker |  | 9171 | 44 | python decl at src/click/_termui_impl.py:597 |  |  | 0.335 |
| walker |  | 9190 | 19 | python decl doc at src/click/_termui_impl.py:597 |  |  | 0.335 |
| ns | 9224 |  | 61 | tests/conftest.py: the canonical 'runner' fixture | 5.4 | 5.1 | 0.332 |
| walker |  | 9272 | 82 | python decl doc at src/click/_termui_impl.py:417 |  |  | 0.332 |
| walker |  | 9320 | 48 | python decl at src/click/_termui_impl.py:442 |  |  | 0.332 |
| walker |  | 9369 | 49 | python decl at src/click/_termui_impl.py:546 |  |  | 0.332 |
| ns | 9391 |  | 167 | termui.py: every public function location | 5.5 |  | 0.329 |
| walker |  | 9424 | 55 | python method at src/click/_textwrap.py:9 |  |  | 0.329 |
| ns | 9472 |  | 81 | utils.py: echo() signature + key kwargs | 5.6 |  | 0.327 |
| ns | 9508 |  | 36 | examples/ subdirectory listing | 6.1 |  | 0.340 |
| walker |  | 9527 | 103 | python method doc at src/click/formatting.py:210 |  |  | 0.340 |
| walker |  | 9594 | 67 | python decl names surface #1 in src/click/utils.py |  |  | 0.340 |
| walker |  | 9594 | 0 | python decl at src/click/utils.py:453 |  |  | 0.340 |
| walker |  | 9594 | 0 | python decl at src/click/utils.py:502 |  |  | 0.340 |
| walker |  | 9653 | 59 | python method sigs #1 in src/click/utils.py |  |  | 0.340 |
| walker |  | 9653 | 0 | python method at src/click/utils.py:511 |  |  | 0.340 |
| walker |  | 9653 | 0 | python method at src/click/utils.py:514 |  |  | 0.340 |
| walker |  | 9653 | 0 | python method at src/click/utils.py:523 |  |  | 0.340 |
| walker |  | 9662 | 9 | python method body at src/click/utils.py:511 body 512 |  |  | 0.340 |
| walker |  | 9673 | 11 | python method body at src/click/utils.py:523 body 524 |  |  | 0.340 |
| walker |  | 9702 | 29 | python decl at src/click/utils.py:527 |  |  | 0.340 |
| ns | 9715 |  | 207 | examples/naval/naval.py: canonical nested-group example | 6.2 |  | 0.334 |
| walker |  | 9831 | 129 | python decl doc at src/click/utils.py:502 |  |  | 0.334 |
| walker |  | 9891 | 60 | python decl at src/click/utils.py:582 |  |  | 0.334 |
| ns | 9915 |  | 200 | docs/index.rst: General Reference toctree | 6.3 |  | 0.328 |
| walker |  | 9983 | 92 | headings outline in docs/advanced.md |  |  | 0.328 |

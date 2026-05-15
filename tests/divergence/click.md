Score(3000)=0.492 I=0.772 C=0.314 ns_rows≤3K=20/52 (reached=6 partial=2 missing=12)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 50 | 50 | listing of '.' |  |  | 1.000 |
| ns | 50 |  | 50 | Top-level directory listing | 1.1 |  | 1.000 |
| walker |  | 53 | 3 | listing of 'src' |  |  | 1.000 |
| ns | 92 |  | 42 | Package name + version | 1.2 |  | 0.859 |
| walker |  | 125 | 72 | README headline in README.md |  |  | 0.887 |
| walker |  | 155 | 30 | headings outline in README.md |  |  | 0.888 |
| ns | 169 |  | 77 | README headline + tagline | 1.3 |  | 0.861 |
| walker |  | 184 | 29 | [package] in pyproject.toml |  |  | 0.931 |
| ns | 209 |  | 40 | Python version floor + runtime dep | 1.4 | 1.2 | 0.848 |
| walker |  | 275 | 91 | README.md section #0 |  |  | 0.872 |
| ns | 310 |  | 101 | README differentiators (three-bullet pitch) | 1.5 | 1.3 | 0.844 |
| walker |  | 361 | 86 | listing of 'src/click' |  |  | 0.844 |
| walker |  | 372 | 11 | python imports #1 in src/click/__init__.py |  |  | 0.844 |
| walker |  | 390 | 18 | python decl names surface in src/click/__init__.py |  |  | 0.844 |
| walker |  | 390 | 0 | python decl at src/click/__init__.py:77 |  |  | 0.844 |
| ns | 410 |  | 100 | __init__.py: core-class re-exports | 1.6 |  | 0.750 |
| walker |  | 463 | 73 | python imports in src/click/__init__.py |  |  | 0.750 |
| walker |  | 479 | 16 | python imports #6 in src/click/__init__.py |  |  | 0.751 |
| walker |  | 492 | 13 | listing of '.github' |  |  | 0.751 |
| walker |  | 515 | 23 | listing of '.github/workflows' |  |  | 0.751 |
| walker |  | 529 | 14 | python decl names surface in src/click/_textwrap.py |  |  | 0.751 |
| walker |  | 529 | 0 | python decl at src/click/_textwrap.py:8 |  |  | 0.751 |
| walker |  | 559 | 30 | python imports #5 in src/click/__init__.py |  |  | 0.751 |
| ns | 578 |  | 168 | README hello-world example | 1.7 | 1.3 | 0.628 |
| walker |  | 659 | 100 | python imports #2 in src/click/__init__.py |  |  | 0.724 |
| walker |  | 700 | 41 | python method sigs in src/click/_textwrap.py |  |  | 0.724 |
| walker |  | 700 | 0 | python method at src/click/_textwrap.py:40 |  |  | 0.724 |
| walker |  | 719 | 19 | python method at src/click/_textwrap.py:27 |  |  | 0.724 |
| ns | 739 |  | 161 | __init__.py: decorator re-exports | 1.8 | 1.6 | 0.656 |
| walker |  | 786 | 67 | README.md section #3 |  |  | 0.656 |
| walker |  | 868 | 82 | README.md section #2 |  |  | 0.656 |
| walker |  | 904 | 36 | listing of 'examples' |  |  | 0.658 |
| ns | 931 |  | 192 | __init__.py: exception + formatting + globals re-exports | 1.9 | 1.8 | 0.602 |
| walker |  | 1096 | 192 | listing of 'docs' |  |  | 0.602 |
| walker |  | 1118 | 22 | docs/setuptools.md section #0 |  |  | 0.602 |
| ns | 1120 |  | 189 | __init__.py: termui re-exports | 1.10 | 1.9 | 0.553 |
| walker |  | 1133 | 15 | listing of 'docs/_static' |  |  | 0.553 |
| walker |  | 1181 | 48 | docs/license.md section #0 |  |  | 0.553 |
| walker |  | 1202 | 21 | headings outline in docs/click-concepts.md |  |  | 0.553 |
| walker |  | 1267 | 65 | docs/click-concepts.md section #0 |  |  | 0.553 |
| walker |  | 1288 | 21 | headings outline in docs/unicode-support.md |  |  | 0.553 |
| ns | 1302 |  | 182 | __init__.py: built-in type re-exports | 1.11 | 1.10 | 0.511 |
| ns | 1395 |  | 93 | __init__.py: utils re-exports | 1.12 | 1.11 | 0.493 |
| walker |  | 1449 | 161 | python imports #3 in src/click/__init__.py |  |  | 0.571 |
| walker |  | 1471 | 22 | headings outline in docs/command-line-reference.md |  |  | 0.571 |
| walker |  | 1539 | 68 | docs/command-line-reference.md section #0 |  |  | 0.571 |
| walker |  | 1561 | 22 | headings outline in docs/contrib.md |  |  | 0.571 |
| walker |  | 1583 | 22 | headings outline in docs/design-opinions.md |  |  | 0.571 |
| walker |  | 1643 | 60 | docs/design-opinions.md section #0 |  |  | 0.571 |
| ns | 1647 |  | 252 | Deprecation shim: BaseCommand → Command, MultiCommand → Group | 1.13 | 1.12 | 0.513 |
| walker |  | 1789 | 146 | python imports #4 in src/click/__init__.py |  |  | 0.583 |
| walker |  | 1811 | 22 | python decl names surface #1 in src/click/exceptions.py |  |  | 0.584 |
| walker |  | 1811 | 0 | python decl at src/click/exceptions.py:330 |  |  | 0.584 |
| walker |  | 1811 | 0 | python decl at src/click/exceptions.py:334 |  |  | 0.584 |
| walker |  | 1825 | 14 | python class body at src/click/exceptions.py:334 |  |  | 0.584 |
| walker |  | 1839 | 14 | python decl doc at src/click/exceptions.py:330 |  |  | 0.584 |
| walker |  | 1862 | 23 | python method sigs #1 in src/click/exceptions.py |  |  | 0.584 |
| walker |  | 1862 | 0 | python method at src/click/exceptions.py:343 |  |  | 0.584 |
| walker |  | 1873 | 11 | python method body at src/click/exceptions.py:343 body 344 |  |  | 0.584 |
| ns | 1916 |  | 269 | Deprecation shim: OptionParser, __version__ | 1.14 | 1.13 | 0.531 |
| walker |  | 1919 | 46 | python decl doc at src/click/exceptions.py:334 |  |  | 0.531 |
| walker |  | 1946 | 27 | python imports in src/click/_utils.py |  |  | 0.531 |
| walker |  | 1976 | 30 | headings outline in docs/entry-points.md |  |  | 0.531 |
| walker |  | 1988 | 12 | listing of 'examples/complex' |  |  | 0.531 |
| walker |  | 2001 | 13 | listing of 'examples/complex/complex' |  |  | 0.531 |
| walker |  | 2017 | 16 | listing of 'examples/complex/complex/commands' |  |  | 0.531 |
| ns | 2121 |  | 205 | decorators.py: location of every public decorator | 2.1 |  | 0.510 |
| ns | 2209 |  | 88 | @command: signature + 1-liner | 2.2 | 2.1 | 0.500 |
| walker |  | 2219 | 202 | README.md section #1 |  |  | 0.570 |
| walker |  | 2251 | 32 | headings outline in docs/extending-click.md |  |  | 0.570 |
| walker |  | 2357 | 106 | docs/extending-click.md section #0 |  |  | 0.570 |
| ns | 2367 |  | 158 | @option: signature + 1-liner | 2.3 | 2.1 | 0.552 |
| walker |  | 2390 | 33 | headings outline in docs/faqs.md |  |  | 0.552 |
| walker |  | 2423 | 33 | docs/faqs.md section #0 |  |  | 0.552 |
| walker |  | 2456 | 33 | headings outline in docs/wincmd.md |  |  | 0.552 |
| ns | 2525 |  | 158 | @argument: signature + 1-liner | 2.4 | 2.1 | 0.537 |
| walker |  | 2530 | 74 | docs/wincmd.md section #0 |  |  | 0.537 |
| walker |  | 2565 | 35 | headings outline in docs/virtualenv.md |  |  | 0.537 |
| walker |  | 2565 | 0 | docs/virtualenv.md section #0 |  |  | 0.537 |
| walker |  | 2602 | 37 | headings outline in docs/parameters.md |  |  | 0.537 |
| walker |  | 2641 | 39 | docs/parameters.md section #0 |  |  | 0.537 |
| walker |  | 2683 | 42 | headings outline in docs/option-decorators.md |  |  | 0.537 |
| ns | 2688 |  | 163 | @group: signature + 1-liner | 2.5 | 2.1 | 0.519 |
| walker |  | 2711 | 28 | docs/option-decorators.md section #3 |  |  | 0.519 |
| walker |  | 2788 | 77 | docs/option-decorators.md section #0 |  |  | 0.519 |
| walker |  | 2831 | 43 | headings outline in docs/handling-files.md |  |  | 0.519 |
| walker |  | 2895 | 64 | docs/handling-files.md section #0 |  |  | 0.519 |
| ns | 2923 |  | 235 | @pass_context / @pass_obj signatures + docstrings | 2.6 | 2.1 | 0.492 |
| walker |  | 2937 | 42 | python imports in src/click/_textwrap.py |  |  | 0.492 |
| walker |  | 3030 | 93 | python decl names surface in src/click/formatting.py |  |  | 0.492 |
| walker |  | 3030 | 0 | python decl at src/click/formatting.py:14 |  |  | 0.492 |
| walker |  | 3030 | 0 | python decl at src/click/formatting.py:104 |  |  | 0.492 |
| walker |  | 3030 | 0 | python decl at src/click/formatting.py:283 |  |  | 0.492 |
| walker |  | 3065 | 35 | python decl at src/click/formatting.py:24 |  |  | 0.492 |
| walker |  | 3128 | 63 | python decl at src/click/formatting.py:31 |  |  | 0.492 |
| walker |  | 3157 | 29 | python decl body at src/click/formatting.py:24 body 27 |  |  | 0.492 |
| walker |  | 3238 | 81 | python decl doc at src/click/formatting.py:283 |  |  | 0.492 |
| ns | 3240 |  | 317 | @make_pass_decorator: signature + docstring + how-it-works | 2.7 | 2.1 | 0.465 |
| walker |  | 3333 | 95 | python decl names surface in src/click/testing.py |  |  | 0.465 |
| walker |  | 3333 | 0 | python decl at src/click/testing.py:26 |  |  | 0.465 |
| walker |  | 3333 | 0 | python decl at src/click/testing.py:70 |  |  | 0.465 |
| walker |  | 3333 | 0 | python decl at src/click/testing.py:89 |  |  | 0.465 |
| walker |  | 3333 | 0 | python decl at src/click/testing.py:103 |  |  | 0.465 |
| walker |  | 3333 | 0 | python decl at src/click/testing.py:183 |  |  | 0.465 |
| walker |  | 3333 | 0 | python decl at src/click/testing.py:261 |  |  | 0.465 |
| walker |  | 3376 | 43 | python decl at src/click/testing.py:163 |  |  | 0.465 |
| walker |  | 3398 | 22 | python decl at src/click/testing.py:60 |  |  | 0.465 |
| walker |  | 3438 | 40 | python decl doc at src/click/testing.py:70 |  |  | 0.465 |
| walker |  | 3489 | 51 | python decl doc at src/click/testing.py:89 |  |  | 0.465 |
| ns | 3498 |  | 258 | @confirmation_option / @password_option full bodies | 2.8 | 2.1 | 0.442 |
| walker |  | 3540 | 51 | headings outline in docs/arguments.md |  |  | 0.442 |
| walker |  | 3674 | 134 | docs/arguments.md section #0 |  |  | 0.442 |
| ns | 3738 |  | 240 | @version_option signature + key kwargs | 2.9 | 2.1 | 0.427 |
| walker |  | 3799 | 125 | python decl doc at src/click/formatting.py:104 |  |  | 0.427 |
| walker |  | 3991 | 192 | python method sigs in src/click/formatting.py |  |  | 0.427 |
| walker |  | 3991 | 0 | python method at src/click/formatting.py:135 |  |  | 0.427 |
| walker |  | 3991 | 0 | python method at src/click/formatting.py:139 |  |  | 0.427 |
| walker |  | 3991 | 0 | python method at src/click/formatting.py:143 |  |  | 0.427 |
| walker |  | 3991 | 0 | python method at src/click/formatting.py:147 |  |  | 0.427 |
| walker |  | 3991 | 0 | python method at src/click/formatting.py:185 |  |  | 0.427 |
| walker |  | 3991 | 0 | python method at src/click/formatting.py:189 |  |  | 0.427 |
| walker |  | 3991 | 0 | python method at src/click/formatting.py:194 |  |  | 0.427 |
| walker |  | 3991 | 0 | python method at src/click/formatting.py:278 |  |  | 0.427 |
| walker |  | 4005 | 14 | python method at src/click/formatting.py:269 |  |  | 0.427 |
| walker |  | 4023 | 18 | python method at src/click/formatting.py:254 |  |  | 0.427 |
| walker |  | 4032 | 9 | python method doc at src/click/formatting.py:139 |  |  | 0.427 |
| walker |  | 4041 | 9 | python method doc at src/click/formatting.py:143 |  |  | 0.427 |
| walker |  | 4050 | 9 | python method doc at src/click/formatting.py:278 |  |  | 0.427 |
| walker |  | 4061 | 11 | python method doc at src/click/formatting.py:185 |  |  | 0.427 |
| walker |  | 4072 | 11 | python method doc at src/click/formatting.py:189 |  |  | 0.427 |
| walker |  | 4085 | 13 | python method doc at src/click/formatting.py:135 |  |  | 0.403 |
| ns | 4085 |  | 347 | quickstart.md: 'Basic Concepts - Creating a Command' section | 2.10 |  | 0.403 |
| walker |  | 4099 | 14 | python method doc at src/click/formatting.py:269 |  |  | 0.403 |
| walker |  | 4109 | 10 | python method body at src/click/formatting.py:135 body 137 |  |  | 0.403 |
| walker |  | 4120 | 11 | python method body at src/click/formatting.py:278 body 280 |  |  | 0.403 |
| walker |  | 4134 | 14 | python method body at src/click/formatting.py:139 body 141 |  |  | 0.403 |
| walker |  | 4148 | 14 | python method body at src/click/formatting.py:143 body 145 |  |  | 0.403 |
| walker |  | 4202 | 54 | python method at src/click/formatting.py:116 |  |  | 0.403 |
| ns | 4205 |  | 120 | core.py: every class header (top-level inventory) | 3.1 |  | 0.395 |
| walker |  | 4221 | 19 | python method body at src/click/formatting.py:189 body 191 |  |  | 0.395 |
| walker |  | 4277 | 56 | python method at src/click/formatting.py:210 |  |  | 0.395 |
| walker |  | 4311 | 34 | python method doc at src/click/formatting.py:194 |  |  | 0.395 |
| walker |  | 4333 | 22 | python method body at src/click/formatting.py:185 body 187 |  |  | 0.395 |
| ns | 4349 |  | 144 | Context: one-paragraph lede | 3.2 | 3.1 | 0.388 |
| walker |  | 4384 | 51 | python method doc at src/click/formatting.py:254 |  |  | 0.388 |
| walker |  | 4436 | 52 | headings outline in docs/support-multiple-versions.md |  |  | 0.388 |
| walker |  | 4553 | 117 | docs/support-multiple-versions.md section #0 |  |  | 0.388 |
| walker |  | 4603 | 50 | python imports in src/click/globals.py |  |  | 0.388 |
| ns | 4618 |  | 269 | Context.__init__ signature | 3.3 | 3.2 | 0.376 |
| walker |  | 4730 | 127 | listing of 'tests' |  |  | 0.376 |
| walker |  | 4786 | 56 | listing of 'tests/typing' |  |  | 0.376 |
| walker |  | 4874 | 88 | python imports #9 in src/click/__init__.py |  |  | 0.393 |
| walker |  | 4979 | 105 | python decl names surface in src/click/_utils.py |  |  | 0.393 |
| walker |  | 4979 | 0 | python decl at src/click/_utils.py:7 |  |  | 0.393 |
| walker |  | 4995 | 16 | python method sigs in src/click/_utils.py |  |  | 0.393 |
| walker |  | 4995 | 0 | python method at src/click/_utils.py:18 |  |  | 0.393 |
| walker |  | 5017 | 22 | python class body at src/click/_utils.py:7 |  |  | 0.393 |
| ns | 5031 |  | 413 | Context: location of every method | 3.4 | 3.2 | 0.380 |
| walker |  | 5033 | 16 | python method body at src/click/_utils.py:18 body 19 |  |  | 0.380 |
| walker |  | 5088 | 55 | python decl doc at src/click/_utils.py:7 |  |  | 0.380 |
| ns | 5156 |  | 125 | Command: one-paragraph lede + headline kwargs | 3.5 | 3.1 | 0.375 |
| walker |  | 5193 | 105 | python decl names surface in src/click/globals.py |  |  | 0.375 |
| walker |  | 5193 | 0 | python decl at src/click/globals.py:12 |  |  | 0.375 |
| walker |  | 5193 | 0 | python decl at src/click/globals.py:16 |  |  | 0.375 |
| walker |  | 5193 | 0 | python decl at src/click/globals.py:20 |  |  | 0.375 |
| walker |  | 5193 | 0 | python decl at src/click/globals.py:44 |  |  | 0.375 |
| walker |  | 5193 | 0 | python decl at src/click/globals.py:49 |  |  | 0.375 |
| walker |  | 5193 | 0 | python decl at src/click/globals.py:54 |  |  | 0.375 |
| walker |  | 5203 | 10 | python decl body at src/click/globals.py:49 body 51 |  |  | 0.375 |
| walker |  | 5216 | 13 | python decl doc at src/click/globals.py:49 |  |  | 0.375 |
| walker |  | 5230 | 14 | python decl doc at src/click/globals.py:44 |  |  | 0.375 |
| walker |  | 5248 | 18 | python decl body at src/click/globals.py:16 body 17 |  |  | 0.375 |
| walker |  | 5267 | 19 | python decl body at src/click/globals.py:44 body 46 |  |  | 0.375 |
| walker |  | 5289 | 22 | python decl body at src/click/globals.py:12 body 13 |  |  | 0.375 |
| walker |  | 5342 | 53 | python decl doc at src/click/globals.py:54 |  |  | 0.375 |
| walker |  | 5350 | 8 | python decl body at src/click/globals.py:20 body 41 |  |  | 0.375 |
| ns | 5357 |  | 201 | Command.__init__ signature | 3.6 | 3.5 | 0.368 |
| walker |  | 5408 | 58 | headings outline in docs/shell-completion.md |  |  | 0.368 |
| walker |  | 5601 | 193 | docs/shell-completion.md section #0 |  |  | 0.368 |
| ns | 5745 |  | 388 | Command: location of every method | 3.7 | 3.5 | 0.359 |
| walker |  | 5798 | 197 | docs/contrib.md section #0 |  |  | 0.359 |
| walker |  | 5859 | 61 | headings outline in docs/why.md |  |  | 0.359 |
| walker |  | 5921 | 62 | headings outline in docs/standalone-apps.md |  |  | 0.359 |
| walker |  | 5955 | 34 | docs/standalone-apps.md section #1 |  |  | 0.359 |
| walker |  | 6019 | 64 | headings outline in docs/exceptions.md |  |  | 0.359 |
| walker |  | 6118 | 99 | docs/exceptions.md section #0 |  |  | 0.359 |
| ns | 6201 |  | 456 | Group: lede + __init__ signature | 3.8 | 3.1 | 0.346 |
| walker |  | 6327 | 209 | docs/entry-points.md section #0 |  |  | 0.346 |
| walker |  | 6404 | 77 | python method doc at src/click/formatting.py:147 |  |  | 0.346 |
| ns | 6415 |  | 214 | Group: method-locations + decorator-style command/group registrars | 3.9 | 3.8 | 0.342 |
| ns | 6567 |  | 152 | Parameter (ABC): lede + key kwargs paragraph | 3.10 | 3.1 | 0.337 |
| walker |  | 6624 | 220 | docs/standalone-apps.md section #0 |  |  | 0.337 |
| walker |  | 6751 | 127 | python decl names surface in src/click/parser.py |  |  | 0.337 |
| walker |  | 6751 | 0 | python decl at src/click/parser.py:111 |  |  | 0.337 |
| walker |  | 6751 | 0 | python decl at src/click/parser.py:120 |  |  | 0.337 |
| walker |  | 6751 | 0 | python decl at src/click/parser.py:127 |  |  | 0.337 |
| walker |  | 6751 | 0 | python decl at src/click/parser.py:185 |  |  | 0.337 |
| walker |  | 6751 | 0 | python decl at src/click/parser.py:216 |  |  | 0.337 |
| walker |  | 6751 | 0 | python decl at src/click/parser.py:224 |  |  | 0.337 |
| walker |  | 6751 | 0 | python decl at src/click/parser.py:503 |  |  | 0.337 |
| walker |  | 6798 | 47 | python decl at src/click/parser.py:51 |  |  | 0.337 |
| ns | 6856 |  | 289 | Parameter.__init__ signature | 3.11 | 3.10 | 0.330 |
| walker |  | 6867 | 69 | headings outline in docs/parameter-types.md |  |  | 0.330 |
| walker |  | 6877 | 10 | docs/parameter-types.md section #1 |  |  | 0.330 |
| walker |  | 6885 | 8 | docs/parameter-types.md section #8 |  |  | 0.330 |
| walker |  | 6998 | 113 | docs/parameter-types.md section #0 |  |  | 0.330 |
| walker |  | 7067 | 69 | headings outline in docs/prompts.md |  |  | 0.330 |
| ns | 7202 |  | 346 | Option: lede + __init__ signature | 3.12 | 3.10 | 0.321 |
| walker |  | 7227 | 160 | docs/prompts.md section #0 |  |  | 0.321 |
| walker |  | 7302 | 75 | python decl body at src/click/formatting.py:14 body 15 |  |  | 0.321 |
| ns | 7355 |  | 153 | Argument: lede + __init__ signature + required-by-default logic | 3.13 | 3.10 | 0.317 |
| walker |  | 7370 | 68 | python imports in src/click/formatting.py |  |  | 0.317 |
| walker |  | 7379 | 9 | python method body at src/click/formatting.py:269 body 272 |  |  | 0.317 |
| walker |  | 7457 | 78 | headings outline in docs/testing.md |  |  | 0.317 |
| walker |  | 7606 | 149 | docs/testing.md section #0 |  |  | 0.317 |
| ns | 7722 |  | 367 | Parameter/Option/Argument: method locations | 3.14 | 3.11 | 0.311 |
| walker |  | 7795 | 189 | python imports #7 in src/click/__init__.py |  |  | 0.340 |
| walker |  | 7871 | 76 | docs/virtualenv.md section #1 |  |  | 0.340 |
| ns | 8019 |  | 297 | types.py: every ParamType subclass header | 4.1 |  | 0.334 |
| walker |  | 8181 | 310 | python method sigs in src/click/parser.py |  |  | 0.334 |
| walker |  | 8181 | 0 | python method at src/click/parser.py:169 |  |  | 0.334 |
| walker |  | 8181 | 0 | python method at src/click/parser.py:186 |  |  | 0.334 |
| walker |  | 8181 | 0 | python method at src/click/parser.py:217 |  |  | 0.334 |
| walker |  | 8181 | 0 | python method at src/click/parser.py:290 |  |  | 0.334 |
| walker |  | 8181 | 0 | python method at src/click/parser.py:316 |  |  | 0.334 |
| walker |  | 8181 | 0 | python method at src/click/parser.py:327 |  |  | 0.334 |
| walker |  | 8181 | 0 | python method at src/click/parser.py:390 |  |  | 0.334 |
| walker |  | 8181 | 0 | python method at src/click/parser.py:470 |  |  | 0.334 |
| walker |  | 8192 | 11 | python method at src/click/parser.py:165 |  |  | 0.334 |
| walker |  | 8225 | 33 | python method at src/click/parser.py:298 |  |  | 0.334 |
| ns | 8250 |  | 231 | ParamType: the subclassing recipe | 4.2 | 4.1 | 0.328 |
| walker |  | 8261 | 36 | python method at src/click/parser.py:241 |  |  | 0.328 |
| walker |  | 8276 | 15 | python method body at src/click/parser.py:165 body 167 |  |  | 0.328 |
| walker |  | 8324 | 48 | python method at src/click/parser.py:191 |  |  | 0.328 |
| walker |  | 8345 | 21 | python method body at src/click/parser.py:290 body 296 |  |  | 0.328 |
| walker |  | 8376 | 31 | python method at src/click/parser.py:363 |  |  | 0.328 |
| ns | 8431 |  | 181 | ParamType.convert + ParamType.shell_complete signatures | 4.3 | 4.2 | 0.325 |
| walker |  | 8459 | 83 | python method at src/click/parser.py:128 |  |  | 0.325 |
| walker |  | 8514 | 55 | python method doc at src/click/parser.py:290 |  |  | 0.325 |
| walker |  | 8543 | 29 | python method body at src/click/parser.py:186 body 187 |  |  | 0.325 |
| ns | 8564 |  | 133 | exceptions.py: every class header | 4.4 |  | 0.321 |
| walker |  | 8629 | 86 | python method at src/click/parser.py:265 |  |  | 0.321 |
| walker |  | 8675 | 46 | python method at src/click/parser.py:430 |  |  | 0.321 |
| walker |  | 8774 | 99 | python method doc at src/click/parser.py:298 |  |  | 0.321 |
| ns | 8790 |  | 226 | ClickException + UsageError ledes + exit_code | 4.5 | 4.4 | 0.316 |
| ns | 8844 |  | 54 | testing.py: CliRunner, Result, helper class locations | 5.1 |  | 0.325 |
| walker |  | 8930 | 156 | python decl names surface in src/click/_termui_impl.py |  |  | 0.325 |
| walker |  | 8930 | 0 | python decl at src/click/_termui_impl.py:43 |  |  | 0.325 |
| walker |  | 8930 | 0 | python decl at src/click/_termui_impl.py:375 |  |  | 0.325 |
| walker |  | 8930 | 0 | python decl at src/click/_termui_impl.py:608 |  |  | 0.325 |
| walker |  | 8930 | 0 | python decl at src/click/_termui_impl.py:724 |  |  | 0.325 |
| walker |  | 8930 | 0 | python decl at src/click/_termui_impl.py:794 |  |  | 0.325 |
| walker |  | 8957 | 27 | python decl at src/click/_termui_impl.py:417 |  |  | 0.325 |
| ns | 8975 |  | 131 | CliRunner.invoke signature | 5.2 | 5.1 | 0.322 |
| walker |  | 9000 | 43 | python class body at src/click/_termui_impl.py:608 |  |  | 0.322 |
| walker |  | 9034 | 34 | python decl at src/click/_termui_impl.py:386 |  |  | 0.322 |
| walker |  | 9051 | 17 | python decl doc at src/click/_termui_impl.py:386 |  |  | 0.322 |
| walker |  | 9095 | 44 | python decl at src/click/_termui_impl.py:597 |  |  | 0.322 |
| walker |  | 9114 | 19 | python decl doc at src/click/_termui_impl.py:597 |  |  | 0.322 |
| ns | 9163 |  | 188 | Result: attribute documentation | 5.3 | 5.1 | 0.318 |
| walker |  | 9196 | 82 | python decl doc at src/click/_termui_impl.py:417 |  |  | 0.318 |
| ns | 9224 |  | 61 | tests/conftest.py: the canonical 'runner' fixture | 5.4 | 5.1 | 0.316 |
| walker |  | 9244 | 48 | python decl at src/click/_termui_impl.py:442 |  |  | 0.316 |
| walker |  | 9293 | 49 | python decl at src/click/_termui_impl.py:546 |  |  | 0.316 |
| walker |  | 9348 | 55 | python method at src/click/_textwrap.py:9 |  |  | 0.316 |
| ns | 9391 |  | 167 | termui.py: every public function location | 5.5 |  | 0.313 |
| walker |  | 9451 | 103 | python method doc at src/click/formatting.py:210 |  |  | 0.313 |
| ns | 9472 |  | 81 | utils.py: echo() signature + key kwargs | 5.6 |  | 0.311 |
| ns | 9508 |  | 36 | examples/ subdirectory listing | 6.1 |  | 0.325 |
| walker |  | 9518 | 67 | python decl names surface #1 in src/click/utils.py |  |  | 0.325 |
| walker |  | 9518 | 0 | python decl at src/click/utils.py:453 |  |  | 0.325 |
| walker |  | 9518 | 0 | python decl at src/click/utils.py:502 |  |  | 0.325 |
| walker |  | 9577 | 59 | python method sigs #1 in src/click/utils.py |  |  | 0.325 |
| walker |  | 9577 | 0 | python method at src/click/utils.py:511 |  |  | 0.325 |
| walker |  | 9577 | 0 | python method at src/click/utils.py:514 |  |  | 0.325 |
| walker |  | 9577 | 0 | python method at src/click/utils.py:523 |  |  | 0.325 |
| walker |  | 9586 | 9 | python method body at src/click/utils.py:511 body 512 |  |  | 0.325 |
| walker |  | 9597 | 11 | python method body at src/click/utils.py:523 body 524 |  |  | 0.325 |
| walker |  | 9626 | 29 | python decl at src/click/utils.py:527 |  |  | 0.325 |
| ns | 9715 |  | 207 | examples/naval/naval.py: canonical nested-group example | 6.2 |  | 0.319 |
| walker |  | 9755 | 129 | python decl doc at src/click/utils.py:502 |  |  | 0.319 |
| walker |  | 9815 | 60 | python decl at src/click/utils.py:582 |  |  | 0.319 |
| walker |  | 9907 | 92 | headings outline in docs/advanced.md |  |  | 0.319 |
| ns | 9915 |  | 200 | docs/index.rst: General Reference toctree | 6.3 |  | 0.312 |
| walker |  | 9973 | 66 | docs/advanced.md section #0 |  |  | 0.312 |

Score(3000)=0.641 I=0.865 C=0.475 ns_rows≤3K=20/52 (reached=11 partial=1 missing=8)

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
| ns | 310 |  | 101 | README differentiators (three-bullet pitch) | 1.5 | 1.3 | 0.705 |
| walker |  | 352 | 73 | python imports in src/click/__init__.py |  |  | 0.705 |
| walker |  | 368 | 16 | python imports #6 in src/click/__init__.py |  |  | 0.705 |
| walker |  | 398 | 30 | python imports #5 in src/click/__init__.py |  |  | 0.706 |
| ns | 410 |  | 100 | __init__.py: core-class re-exports | 1.6 |  | 0.628 |
| walker |  | 498 | 100 | python imports #2 in src/click/__init__.py |  |  | 0.752 |
| ns | 578 |  | 168 | README hello-world example | 1.7 | 1.3 | 0.628 |
| walker |  | 659 | 161 | python imports #3 in src/click/__init__.py |  |  | 0.641 |
| walker |  | 677 | 18 | python decl names surface in src/click/__init__.py |  |  | 0.641 |
| walker |  | 677 | 0 | python decl at src/click/__init__.py:77 |  |  | 0.641 |
| ns | 739 |  | 161 | __init__.py: decorator re-exports | 1.8 | 1.6 | 0.678 |
| walker |  | 823 | 146 | python imports #4 in src/click/__init__.py |  |  | 0.691 |
| walker |  | 911 | 88 | python imports #9 in src/click/__init__.py |  |  | 0.696 |
| ns | 931 |  | 192 | __init__.py: exception + formatting + globals re-exports | 1.9 | 1.8 | 0.723 |
| walker |  | 1002 | 91 | README.md section #0 |  |  | 0.787 |
| ns | 1120 |  | 189 | __init__.py: termui re-exports | 1.10 | 1.9 | 0.723 |
| walker |  | 1191 | 189 | python imports #7 in src/click/__init__.py |  |  | 0.809 |
| ns | 1302 |  | 182 | __init__.py: built-in type re-exports | 1.11 | 1.10 | 0.748 |
| walker |  | 1373 | 182 | python imports #8 in src/click/__init__.py |  |  | 0.827 |
| walker |  | 1386 | 13 | listing of '.github' |  |  | 0.827 |
| ns | 1395 |  | 93 | __init__.py: utils re-exports | 1.12 | 1.11 | 0.825 |
| walker |  | 1409 | 23 | listing of '.github/workflows' |  |  | 0.825 |
| walker |  | 1423 | 14 | python decl names surface in src/click/_textwrap.py |  |  | 0.825 |
| walker |  | 1423 | 0 | python decl at src/click/_textwrap.py:8 |  |  | 0.825 |
| walker |  | 1501 | 78 | [package] in pyproject.toml |  |  | 0.877 |
| walker |  | 1542 | 41 | python method sigs in src/click/_textwrap.py |  |  | 0.877 |
| walker |  | 1542 | 0 | python method at src/click/_textwrap.py:40 |  |  | 0.877 |
| walker |  | 1561 | 19 | python method at src/click/_textwrap.py:27 |  |  | 0.877 |
| walker |  | 1628 | 67 | README.md section #3 |  |  | 0.877 |
| ns | 1647 |  | 252 | Deprecation shim: BaseCommand → Command, MultiCommand → Group | 1.13 | 1.12 | 0.788 |
| walker |  | 1710 | 82 | README.md section #2 |  |  | 0.788 |
| walker |  | 1746 | 36 | listing of 'examples' |  |  | 0.789 |
| ns | 1916 |  | 269 | Deprecation shim: OptionParser, __version__ | 1.14 | 1.13 | 0.718 |
| walker |  | 1938 | 192 | listing of 'docs' |  |  | 0.718 |
| walker |  | 1960 | 22 | docs/setuptools.md section #0 |  |  | 0.718 |
| walker |  | 1975 | 15 | listing of 'docs/_static' |  |  | 0.718 |
| walker |  | 2023 | 48 | docs/license.md section #0 |  |  | 0.718 |
| walker |  | 2045 | 22 | python decl names surface #1 in src/click/exceptions.py |  |  | 0.718 |
| walker |  | 2045 | 0 | python decl at src/click/exceptions.py:330 |  |  | 0.718 |
| walker |  | 2045 | 0 | python decl at src/click/exceptions.py:334 |  |  | 0.718 |
| walker |  | 2059 | 14 | python class body at src/click/exceptions.py:334 |  |  | 0.718 |
| walker |  | 2073 | 14 | python decl doc at src/click/exceptions.py:330 |  |  | 0.718 |
| walker |  | 2096 | 23 | python method sigs #1 in src/click/exceptions.py |  |  | 0.718 |
| walker |  | 2096 | 0 | python method at src/click/exceptions.py:343 |  |  | 0.718 |
| walker |  | 2107 | 11 | python method body at src/click/exceptions.py:343 body 344 |  |  | 0.718 |
| ns | 2121 |  | 205 | decorators.py: location of every public decorator | 2.1 |  | 0.690 |
| walker |  | 2153 | 46 | python decl doc at src/click/exceptions.py:334 |  |  | 0.690 |
| walker |  | 2180 | 27 | python imports in src/click/_utils.py |  |  | 0.690 |
| walker |  | 2192 | 12 | listing of 'examples/complex' |  |  | 0.690 |
| walker |  | 2205 | 13 | listing of 'examples/complex/complex' |  |  | 0.690 |
| ns | 2209 |  | 88 | @command: signature + 1-liner | 2.2 | 2.1 | 0.677 |
| walker |  | 2221 | 16 | listing of 'examples/complex/complex/commands' |  |  | 0.677 |
| ns | 2367 |  | 158 | @option: signature + 1-liner | 2.3 | 2.1 | 0.657 |
| walker |  | 2423 | 202 | README.md section #1 |  |  | 0.718 |
| walker |  | 2465 | 42 | python imports in src/click/_textwrap.py |  |  | 0.718 |
| ns | 2525 |  | 158 | @argument: signature + 1-liner | 2.4 | 2.1 | 0.698 |
| walker |  | 2558 | 93 | python decl names surface in src/click/formatting.py |  |  | 0.698 |
| walker |  | 2558 | 0 | python decl at src/click/formatting.py:14 |  |  | 0.698 |
| walker |  | 2558 | 0 | python decl at src/click/formatting.py:104 |  |  | 0.698 |
| walker |  | 2558 | 0 | python decl at src/click/formatting.py:283 |  |  | 0.698 |
| walker |  | 2593 | 35 | python decl at src/click/formatting.py:24 |  |  | 0.698 |
| walker |  | 2622 | 29 | python decl body at src/click/formatting.py:24 body 27 |  |  | 0.698 |
| ns | 2688 |  | 163 | @group: signature + 1-liner | 2.5 | 2.1 | 0.675 |
| walker |  | 2724 | 102 | python decl at src/click/formatting.py:31 |  |  | 0.675 |
| walker |  | 2805 | 81 | python decl doc at src/click/formatting.py:283 |  |  | 0.675 |
| walker |  | 2900 | 95 | python decl names surface in src/click/testing.py |  |  | 0.676 |
| walker |  | 2900 | 0 | python decl at src/click/testing.py:26 |  |  | 0.676 |
| walker |  | 2900 | 0 | python decl at src/click/testing.py:70 |  |  | 0.676 |
| walker |  | 2900 | 0 | python decl at src/click/testing.py:89 |  |  | 0.676 |
| walker |  | 2900 | 0 | python decl at src/click/testing.py:103 |  |  | 0.676 |
| walker |  | 2915 | 15 | python decl at src/click/testing.py:183 |  |  | 0.676 |
| ns | 2923 |  | 235 | @pass_context / @pass_obj signatures + docstrings | 2.6 | 2.1 | 0.641 |
| walker |  | 2950 | 35 | python decl at src/click/testing.py:261 |  |  | 0.641 |
| walker |  | 2993 | 43 | python decl at src/click/testing.py:163 |  |  | 0.641 |
| walker |  | 3015 | 22 | python decl at src/click/testing.py:60 |  |  | 0.641 |
| walker |  | 3055 | 40 | python decl doc at src/click/testing.py:70 |  |  | 0.641 |
| walker |  | 3106 | 51 | python decl doc at src/click/testing.py:89 |  |  | 0.641 |
| walker |  | 3231 | 125 | python decl doc at src/click/formatting.py:104 |  |  | 0.641 |
| ns | 3240 |  | 317 | @make_pass_decorator: signature + docstring + how-it-works | 2.7 | 2.1 | 0.605 |
| walker |  | 3423 | 192 | python method sigs in src/click/formatting.py |  |  | 0.605 |
| walker |  | 3423 | 0 | python method at src/click/formatting.py:135 |  |  | 0.605 |
| walker |  | 3423 | 0 | python method at src/click/formatting.py:139 |  |  | 0.605 |
| walker |  | 3423 | 0 | python method at src/click/formatting.py:143 |  |  | 0.605 |
| walker |  | 3423 | 0 | python method at src/click/formatting.py:147 |  |  | 0.605 |
| walker |  | 3423 | 0 | python method at src/click/formatting.py:185 |  |  | 0.605 |
| walker |  | 3423 | 0 | python method at src/click/formatting.py:189 |  |  | 0.605 |
| walker |  | 3423 | 0 | python method at src/click/formatting.py:194 |  |  | 0.605 |
| walker |  | 3423 | 0 | python method at src/click/formatting.py:278 |  |  | 0.605 |
| walker |  | 3437 | 14 | python method at src/click/formatting.py:269 |  |  | 0.605 |
| walker |  | 3455 | 18 | python method at src/click/formatting.py:254 |  |  | 0.605 |
| walker |  | 3464 | 9 | python method doc at src/click/formatting.py:139 |  |  | 0.605 |
| walker |  | 3473 | 9 | python method doc at src/click/formatting.py:143 |  |  | 0.605 |
| walker |  | 3482 | 9 | python method doc at src/click/formatting.py:278 |  |  | 0.605 |
| walker |  | 3493 | 11 | python method doc at src/click/formatting.py:185 |  |  | 0.605 |
| ns | 3498 |  | 258 | @confirmation_option / @password_option full bodies | 2.8 | 2.1 | 0.575 |
| walker |  | 3504 | 11 | python method doc at src/click/formatting.py:189 |  |  | 0.575 |
| walker |  | 3517 | 13 | python method doc at src/click/formatting.py:135 |  |  | 0.575 |
| walker |  | 3531 | 14 | python method doc at src/click/formatting.py:269 |  |  | 0.575 |
| walker |  | 3541 | 10 | python method body at src/click/formatting.py:135 body 137 |  |  | 0.575 |
| walker |  | 3552 | 11 | python method body at src/click/formatting.py:278 body 280 |  |  | 0.575 |
| walker |  | 3566 | 14 | python method body at src/click/formatting.py:139 body 141 |  |  | 0.575 |
| walker |  | 3580 | 14 | python method body at src/click/formatting.py:143 body 145 |  |  | 0.575 |
| walker |  | 3634 | 54 | python method at src/click/formatting.py:116 |  |  | 0.575 |
| walker |  | 3653 | 19 | python method body at src/click/formatting.py:189 body 191 |  |  | 0.575 |
| walker |  | 3709 | 56 | python method at src/click/formatting.py:210 |  |  | 0.575 |
| ns | 3738 |  | 240 | @version_option signature + key kwargs | 2.9 | 2.1 | 0.555 |
| walker |  | 3743 | 34 | python method doc at src/click/formatting.py:194 |  |  | 0.555 |
| walker |  | 3765 | 22 | python method body at src/click/formatting.py:185 body 187 |  |  | 0.555 |
| walker |  | 3816 | 51 | python method doc at src/click/formatting.py:254 |  |  | 0.555 |
| walker |  | 3866 | 50 | python imports in src/click/globals.py |  |  | 0.555 |
| walker |  | 3993 | 127 | listing of 'tests' |  |  | 0.555 |
| walker |  | 4049 | 56 | listing of 'tests/typing' |  |  | 0.555 |
| ns | 4085 |  | 347 | quickstart.md: 'Basic Concepts - Creating a Command' section | 2.10 |  | 0.523 |
| walker |  | 4154 | 105 | python decl names surface in src/click/_utils.py |  |  | 0.523 |
| walker |  | 4154 | 0 | python decl at src/click/_utils.py:7 |  |  | 0.523 |
| walker |  | 4170 | 16 | python method sigs in src/click/_utils.py |  |  | 0.523 |
| walker |  | 4170 | 0 | python method at src/click/_utils.py:18 |  |  | 0.523 |
| walker |  | 4192 | 22 | python class body at src/click/_utils.py:7 |  |  | 0.523 |
| ns | 4205 |  | 120 | core.py: every class header (top-level inventory) | 3.1 |  | 0.514 |
| walker |  | 4208 | 16 | python method body at src/click/_utils.py:18 body 19 |  |  | 0.514 |
| walker |  | 4263 | 55 | python decl doc at src/click/_utils.py:7 |  |  | 0.514 |
| ns | 4349 |  | 144 | Context: one-paragraph lede | 3.2 | 3.1 | 0.504 |
| walker |  | 4368 | 105 | python decl names surface in src/click/globals.py |  |  | 0.504 |
| walker |  | 4368 | 0 | python decl at src/click/globals.py:12 |  |  | 0.504 |
| walker |  | 4368 | 0 | python decl at src/click/globals.py:16 |  |  | 0.504 |
| walker |  | 4368 | 0 | python decl at src/click/globals.py:44 |  |  | 0.504 |
| walker |  | 4368 | 0 | python decl at src/click/globals.py:49 |  |  | 0.504 |
| walker |  | 4368 | 0 | python decl at src/click/globals.py:54 |  |  | 0.504 |
| walker |  | 4378 | 10 | python decl body at src/click/globals.py:49 body 51 |  |  | 0.504 |
| walker |  | 4391 | 13 | python decl doc at src/click/globals.py:49 |  |  | 0.504 |
| walker |  | 4405 | 14 | python decl doc at src/click/globals.py:44 |  |  | 0.504 |
| walker |  | 4445 | 40 | python decl at src/click/globals.py:20 |  |  | 0.504 |
| walker |  | 4463 | 18 | python decl body at src/click/globals.py:16 body 17 |  |  | 0.504 |
| walker |  | 4482 | 19 | python decl body at src/click/globals.py:44 body 46 |  |  | 0.504 |
| walker |  | 4504 | 22 | python decl body at src/click/globals.py:12 body 13 |  |  | 0.504 |
| walker |  | 4557 | 53 | python decl doc at src/click/globals.py:54 |  |  | 0.504 |
| walker |  | 4565 | 8 | python decl body at src/click/globals.py:20 body 41 |  |  | 0.504 |
| ns | 4618 |  | 269 | Context.__init__ signature | 3.3 | 3.2 | 0.489 |
| walker |  | 4642 | 77 | python method doc at src/click/formatting.py:147 |  |  | 0.489 |
| walker |  | 4769 | 127 | python decl names surface in src/click/parser.py |  |  | 0.489 |
| walker |  | 4769 | 0 | python decl at src/click/parser.py:111 |  |  | 0.489 |
| walker |  | 4769 | 0 | python decl at src/click/parser.py:120 |  |  | 0.489 |
| walker |  | 4769 | 0 | python decl at src/click/parser.py:127 |  |  | 0.489 |
| walker |  | 4769 | 0 | python decl at src/click/parser.py:185 |  |  | 0.489 |
| walker |  | 4769 | 0 | python decl at src/click/parser.py:216 |  |  | 0.489 |
| walker |  | 4769 | 0 | python decl at src/click/parser.py:503 |  |  | 0.489 |
| walker |  | 4806 | 37 | python decl at src/click/parser.py:224 |  |  | 0.489 |
| walker |  | 4853 | 47 | python decl at src/click/parser.py:51 |  |  | 0.489 |
| walker |  | 4928 | 75 | python decl body at src/click/formatting.py:14 body 15 |  |  | 0.489 |
| walker |  | 4996 | 68 | python imports in src/click/formatting.py |  |  | 0.489 |
| ns | 5031 |  | 413 | Context: location of every method | 3.4 | 3.2 | 0.473 |
| walker |  | 5148 | 152 | python decl doc at src/click/globals.py:20 |  |  | 0.473 |
| ns | 5156 |  | 125 | Command: one-paragraph lede + headline kwargs | 3.5 | 3.1 | 0.467 |
| walker |  | 5157 | 9 | python method body at src/click/formatting.py:269 body 272 |  |  | 0.467 |
| ns | 5357 |  | 201 | Command.__init__ signature | 3.6 | 3.5 | 0.458 |
| walker |  | 5467 | 310 | python method sigs in src/click/parser.py |  |  | 0.458 |
| walker |  | 5467 | 0 | python method at src/click/parser.py:169 |  |  | 0.458 |
| walker |  | 5467 | 0 | python method at src/click/parser.py:186 |  |  | 0.458 |
| walker |  | 5467 | 0 | python method at src/click/parser.py:217 |  |  | 0.458 |
| walker |  | 5467 | 0 | python method at src/click/parser.py:290 |  |  | 0.458 |
| walker |  | 5467 | 0 | python method at src/click/parser.py:316 |  |  | 0.458 |
| walker |  | 5467 | 0 | python method at src/click/parser.py:327 |  |  | 0.458 |
| walker |  | 5467 | 0 | python method at src/click/parser.py:390 |  |  | 0.458 |
| walker |  | 5467 | 0 | python method at src/click/parser.py:470 |  |  | 0.458 |
| walker |  | 5478 | 11 | python method at src/click/parser.py:165 |  |  | 0.458 |
| walker |  | 5511 | 33 | python method at src/click/parser.py:298 |  |  | 0.458 |
| walker |  | 5547 | 36 | python method at src/click/parser.py:241 |  |  | 0.458 |
| walker |  | 5562 | 15 | python method body at src/click/parser.py:165 body 167 |  |  | 0.458 |
| walker |  | 5610 | 48 | python method at src/click/parser.py:191 |  |  | 0.458 |
| walker |  | 5631 | 21 | python method body at src/click/parser.py:290 body 296 |  |  | 0.458 |
| walker |  | 5662 | 31 | python method at src/click/parser.py:363 |  |  | 0.458 |
| walker |  | 5745 | 83 | python method at src/click/parser.py:128 |  |  | 0.446 |
| ns | 5745 |  | 388 | Command: location of every method | 3.7 | 3.5 | 0.446 |
| walker |  | 5800 | 55 | python method doc at src/click/parser.py:290 |  |  | 0.446 |
| walker |  | 5829 | 29 | python method body at src/click/parser.py:186 body 187 |  |  | 0.446 |
| walker |  | 5915 | 86 | python method at src/click/parser.py:265 |  |  | 0.446 |
| walker |  | 5961 | 46 | python method at src/click/parser.py:430 |  |  | 0.446 |
| walker |  | 6060 | 99 | python method doc at src/click/parser.py:298 |  |  | 0.446 |
| ns | 6201 |  | 456 | Group: lede + __init__ signature | 3.8 | 3.1 | 0.431 |
| walker |  | 6216 | 156 | python decl names surface in src/click/_termui_impl.py |  |  | 0.431 |
| walker |  | 6216 | 0 | python decl at src/click/_termui_impl.py:43 |  |  | 0.431 |
| walker |  | 6216 | 0 | python decl at src/click/_termui_impl.py:375 |  |  | 0.431 |
| walker |  | 6216 | 0 | python decl at src/click/_termui_impl.py:608 |  |  | 0.431 |
| walker |  | 6216 | 0 | python decl at src/click/_termui_impl.py:724 |  |  | 0.431 |
| walker |  | 6216 | 0 | python decl at src/click/_termui_impl.py:794 |  |  | 0.431 |
| walker |  | 6243 | 27 | python decl at src/click/_termui_impl.py:417 |  |  | 0.431 |
| walker |  | 6286 | 43 | python class body at src/click/_termui_impl.py:608 |  |  | 0.431 |
| walker |  | 6320 | 34 | python decl at src/click/_termui_impl.py:386 |  |  | 0.431 |
| walker |  | 6337 | 17 | python decl doc at src/click/_termui_impl.py:386 |  |  | 0.431 |
| walker |  | 6381 | 44 | python decl at src/click/_termui_impl.py:597 |  |  | 0.431 |
| walker |  | 6400 | 19 | python decl doc at src/click/_termui_impl.py:597 |  |  | 0.431 |
| ns | 6415 |  | 214 | Group: method-locations + decorator-style command/group registrars | 3.9 | 3.8 | 0.425 |
| walker |  | 6482 | 82 | python decl doc at src/click/_termui_impl.py:417 |  |  | 0.425 |
| walker |  | 6530 | 48 | python decl at src/click/_termui_impl.py:442 |  |  | 0.425 |
| ns | 6567 |  | 152 | Parameter (ABC): lede + key kwargs paragraph | 3.10 | 3.1 | 0.420 |
| walker |  | 6579 | 49 | python decl at src/click/_termui_impl.py:546 |  |  | 0.420 |
| walker |  | 6634 | 55 | python method at src/click/_textwrap.py:9 |  |  | 0.420 |
| walker |  | 6737 | 103 | python method doc at src/click/formatting.py:210 |  |  | 0.420 |
| walker |  | 6804 | 67 | python decl names surface #1 in src/click/utils.py |  |  | 0.420 |
| walker |  | 6804 | 0 | python decl at src/click/utils.py:502 |  |  | 0.420 |
| walker |  | 6839 | 35 | python decl at src/click/utils.py:453 |  |  | 0.420 |
| ns | 6856 |  | 289 | Parameter.__init__ signature | 3.11 | 3.10 | 0.411 |
| walker |  | 6898 | 59 | python method sigs #1 in src/click/utils.py |  |  | 0.411 |
| walker |  | 6898 | 0 | python method at src/click/utils.py:511 |  |  | 0.411 |
| walker |  | 6898 | 0 | python method at src/click/utils.py:514 |  |  | 0.411 |
| walker |  | 6898 | 0 | python method at src/click/utils.py:523 |  |  | 0.411 |
| walker |  | 6907 | 9 | python method body at src/click/utils.py:511 body 512 |  |  | 0.411 |
| walker |  | 6918 | 11 | python method body at src/click/utils.py:523 body 524 |  |  | 0.411 |
| walker |  | 7047 | 129 | python decl doc at src/click/utils.py:502 |  |  | 0.411 |
| walker |  | 7117 | 70 | python decl at src/click/utils.py:527 |  |  | 0.411 |
| walker |  | 7192 | 75 | python decl at src/click/utils.py:582 |  |  | 0.411 |
| ns | 7202 |  | 346 | Option: lede + __init__ signature | 3.12 | 3.10 | 0.399 |
| walker |  | 7282 | 90 | python imports in src/click/_compat.py |  |  | 0.399 |
| ns | 7355 |  | 153 | Argument: lede + __init__ signature + required-by-default logic | 3.13 | 3.10 | 0.394 |
| walker |  | 7518 | 236 | python decl doc at src/click/testing.py:261 |  |  | 0.394 |
| walker |  | 7529 | 11 | python method body at src/click/parser.py:169 body 182 |  |  | 0.394 |
| ns | 7722 |  | 367 | Parameter/Option/Argument: method locations | 3.14 | 3.11 | 0.387 |
| walker |  | 7774 | 245 | python decl doc at src/click/testing.py:183 |  |  | 0.388 |
| walker |  | 7785 | 11 | listing of '.devcontainer' |  |  | 0.388 |
| walker |  | 7907 | 122 | python method doc at src/click/parser.py:265 |  |  | 0.388 |
| walker |  | 7962 | 55 | python method body at src/click/parser.py:217 body 218 |  |  | 0.388 |
| walker |  | 8017 | 55 | python method body at src/click/utils.py:514 body 515 |  |  | 0.388 |
| ns | 8019 |  | 297 | types.py: every ParamType subclass header | 4.1 |  | 0.381 |
| walker |  | 8238 | 221 | python decl doc at src/click/formatting.py:31 |  |  | 0.381 |
| ns | 8250 |  | 231 | ParamType: the subclassing recipe | 4.2 | 4.1 | 0.375 |
| walker |  | 8334 | 96 | python decl doc at src/click/_termui_impl.py:546 |  |  | 0.375 |
| ns | 8431 |  | 181 | ParamType.convert + ParamType.shell_complete signatures | 4.3 | 4.2 | 0.370 |
| walker |  | 8490 | 156 | python decl names surface in src/click/shell_completion.py |  |  | 0.370 |
| walker |  | 8490 | 0 | python decl at src/click/shell_completion.py:204 |  |  | 0.370 |
| walker |  | 8490 | 0 | python decl at src/click/shell_completion.py:308 |  |  | 0.370 |
| walker |  | 8490 | 0 | python decl at src/click/shell_completion.py:367 |  |  | 0.370 |
| walker |  | 8490 | 0 | python decl at src/click/shell_completion.py:403 |  |  | 0.370 |
| walker |  | 8499 | 9 | python decl doc at src/click/shell_completion.py:308 |  |  | 0.370 |
| walker |  | 8508 | 9 | python decl doc at src/click/shell_completion.py:403 |  |  | 0.370 |
| walker |  | 8518 | 10 | python decl doc at src/click/shell_completion.py:367 |  |  | 0.370 |
| walker |  | 8545 | 27 | python decl at src/click/shell_completion.py:449 |  |  | 0.370 |
| ns | 8564 |  | 133 | exceptions.py: every class header | 4.4 |  | 0.367 |
| walker |  | 8579 | 34 | python decl at src/click/shell_completion.py:57 |  |  | 0.367 |
| walker |  | 8600 | 21 | python class body at src/click/shell_completion.py:57 |  |  | 0.367 |
| walker |  | 8623 | 23 | python class body at src/click/shell_completion.py:308 |  |  | 0.367 |
| walker |  | 8646 | 23 | python class body at src/click/shell_completion.py:403 |  |  | 0.367 |
| walker |  | 8670 | 24 | python class body at src/click/shell_completion.py:367 |  |  | 0.367 |
| walker |  | 8731 | 61 | python decl at src/click/shell_completion.py:19 |  |  | 0.367 |
| ns | 8790 |  | 226 | ClickException + UsageError ledes + exit_code | 4.5 | 4.4 | 0.361 |
| ns | 8844 |  | 54 | testing.py: CliRunner, Result, helper class locations | 5.1 |  | 0.368 |
| walker |  | 8849 | 118 | python class body at src/click/shell_completion.py:204 |  |  | 0.368 |
| walker |  | 8889 | 40 | python decl at src/click/shell_completion.py:442 |  |  | 0.368 |
| ns | 8975 |  | 131 | CliRunner.invoke signature | 5.2 | 5.1 | 0.365 |
| walker |  | 9016 | 127 | python decl doc at src/click/shell_completion.py:204 |  |  | 0.365 |
| walker |  | 9125 | 109 | python decl doc at src/click/shell_completion.py:449 |  |  | 0.365 |
| ns | 9163 |  | 188 | Result: attribute documentation | 5.3 | 5.1 | 0.379 |
| ns | 9224 |  | 61 | tests/conftest.py: the canonical 'runner' fixture | 5.4 | 5.1 | 0.376 |
| walker |  | 9281 | 156 | python decl doc at src/click/shell_completion.py:19 |  |  | 0.376 |
| ns | 9391 |  | 167 | termui.py: every public function location | 5.5 |  | 0.372 |
| ns | 9472 |  | 81 | utils.py: echo() signature + key kwargs | 5.6 |  | 0.370 |
| walker |  | 9484 | 203 | python decl doc at src/click/shell_completion.py:57 |  |  | 0.370 |
| ns | 9508 |  | 36 | examples/ subdirectory listing | 6.1 |  | 0.382 |
| ns | 9715 |  | 207 | examples/naval/naval.py: canonical nested-group example | 6.2 |  | 0.374 |
| walker |  | 9798 | 314 | python method sigs in src/click/shell_completion.py |  |  | 0.374 |
| walker |  | 9798 | 0 | python method at src/click/shell_completion.py:90 |  |  | 0.374 |
| walker |  | 9798 | 0 | python method at src/click/shell_completion.py:248 |  |  | 0.374 |
| walker |  | 9798 | 0 | python method at src/click/shell_completion.py:260 |  |  | 0.374 |
| walker |  | 9798 | 0 | python method at src/click/shell_completion.py:268 |  |  | 0.374 |
| walker |  | 9798 | 0 | python method at src/click/shell_completion.py:275 |  |  | 0.374 |
| walker |  | 9798 | 0 | python method at src/click/shell_completion.py:287 |  |  | 0.374 |
| walker |  | 9798 | 0 | python method at src/click/shell_completion.py:295 |  |  | 0.374 |
| walker |  | 9798 | 0 | python method at src/click/shell_completion.py:347 |  |  | 0.374 |
| walker |  | 9798 | 0 | python method at src/click/shell_completion.py:351 |  |  | 0.374 |
| walker |  | 9798 | 0 | python method at src/click/shell_completion.py:363 |  |  | 0.374 |
| walker |  | 9798 | 0 | python method at src/click/shell_completion.py:373 |  |  | 0.374 |
| walker |  | 9798 | 0 | python method at src/click/shell_completion.py:385 |  |  | 0.374 |
| walker |  | 9798 | 0 | python method at src/click/shell_completion.py:409 |  |  | 0.374 |
| walker |  | 9798 | 0 | python method at src/click/shell_completion.py:423 |  |  | 0.374 |
| walker |  | 9809 | 11 | python method at src/click/shell_completion.py:240 |  |  | 0.374 |
| walker |  | 9820 | 11 | python method at src/click/shell_completion.py:314 |  |  | 0.374 |
| walker |  | 9830 | 10 | python method body at src/click/shell_completion.py:90 body 91 |  |  | 0.374 |
| walker |  | 9840 | 10 | python method body at src/click/shell_completion.py:268 body 273 |  |  | 0.374 |
| walker |  | 9850 | 10 | python method body at src/click/shell_completion.py:287 body 293 |  |  | 0.374 |
| walker |  | 9862 | 12 | python method body at src/click/shell_completion.py:363 body 364 |  |  | 0.374 |
| walker |  | 9876 | 14 | python method body at src/click/shell_completion.py:260 body 266 |  |  | 0.374 |
| walker |  | 9906 | 30 | python method doc at src/click/shell_completion.py:240 |  |  | 0.374 |
| ns | 9915 |  | 200 | docs/index.rst: General Reference toctree | 6.3 |  | 0.367 |
| walker |  | 9924 | 18 | python method body at src/click/shell_completion.py:347 body 348 |  |  | 0.367 |
| walker |  | 9984 | 60 | python method at src/click/shell_completion.py:78 |  |  | 0.367 |

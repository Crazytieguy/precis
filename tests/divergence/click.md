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
| walker |  | 2656 | 63 | python decl at src/click/formatting.py:31 |  |  | 0.698 |
| walker |  | 2685 | 29 | python decl body at src/click/formatting.py:24 body 27 |  |  | 0.698 |
| ns | 2688 |  | 163 | @group: signature + 1-liner | 2.5 | 2.1 | 0.675 |
| walker |  | 2766 | 81 | python decl doc at src/click/formatting.py:283 |  |  | 0.675 |
| walker |  | 2861 | 95 | python decl names surface in src/click/testing.py |  |  | 0.676 |
| walker |  | 2861 | 0 | python decl at src/click/testing.py:26 |  |  | 0.676 |
| walker |  | 2861 | 0 | python decl at src/click/testing.py:70 |  |  | 0.676 |
| walker |  | 2861 | 0 | python decl at src/click/testing.py:89 |  |  | 0.676 |
| walker |  | 2861 | 0 | python decl at src/click/testing.py:103 |  |  | 0.676 |
| walker |  | 2861 | 0 | python decl at src/click/testing.py:183 |  |  | 0.676 |
| walker |  | 2861 | 0 | python decl at src/click/testing.py:261 |  |  | 0.676 |
| walker |  | 2904 | 43 | python decl at src/click/testing.py:163 |  |  | 0.676 |
| ns | 2923 |  | 235 | @pass_context / @pass_obj signatures + docstrings | 2.6 | 2.1 | 0.641 |
| walker |  | 2926 | 22 | python decl at src/click/testing.py:60 |  |  | 0.641 |
| walker |  | 2966 | 40 | python decl doc at src/click/testing.py:70 |  |  | 0.641 |
| walker |  | 3017 | 51 | python decl doc at src/click/testing.py:89 |  |  | 0.641 |
| walker |  | 3142 | 125 | python decl doc at src/click/formatting.py:104 |  |  | 0.641 |
| ns | 3240 |  | 317 | @make_pass_decorator: signature + docstring + how-it-works | 2.7 | 2.1 | 0.605 |
| walker |  | 3334 | 192 | python method sigs in src/click/formatting.py |  |  | 0.605 |
| walker |  | 3334 | 0 | python method at src/click/formatting.py:135 |  |  | 0.605 |
| walker |  | 3334 | 0 | python method at src/click/formatting.py:139 |  |  | 0.605 |
| walker |  | 3334 | 0 | python method at src/click/formatting.py:143 |  |  | 0.605 |
| walker |  | 3334 | 0 | python method at src/click/formatting.py:147 |  |  | 0.605 |
| walker |  | 3334 | 0 | python method at src/click/formatting.py:185 |  |  | 0.605 |
| walker |  | 3334 | 0 | python method at src/click/formatting.py:189 |  |  | 0.605 |
| walker |  | 3334 | 0 | python method at src/click/formatting.py:194 |  |  | 0.605 |
| walker |  | 3334 | 0 | python method at src/click/formatting.py:278 |  |  | 0.605 |
| walker |  | 3348 | 14 | python method at src/click/formatting.py:269 |  |  | 0.605 |
| walker |  | 3366 | 18 | python method at src/click/formatting.py:254 |  |  | 0.605 |
| walker |  | 3375 | 9 | python method doc at src/click/formatting.py:139 |  |  | 0.605 |
| walker |  | 3384 | 9 | python method doc at src/click/formatting.py:143 |  |  | 0.605 |
| walker |  | 3393 | 9 | python method doc at src/click/formatting.py:278 |  |  | 0.605 |
| walker |  | 3404 | 11 | python method doc at src/click/formatting.py:185 |  |  | 0.605 |
| walker |  | 3415 | 11 | python method doc at src/click/formatting.py:189 |  |  | 0.605 |
| walker |  | 3428 | 13 | python method doc at src/click/formatting.py:135 |  |  | 0.605 |
| walker |  | 3442 | 14 | python method doc at src/click/formatting.py:269 |  |  | 0.605 |
| walker |  | 3452 | 10 | python method body at src/click/formatting.py:135 body 137 |  |  | 0.605 |
| walker |  | 3463 | 11 | python method body at src/click/formatting.py:278 body 280 |  |  | 0.605 |
| walker |  | 3477 | 14 | python method body at src/click/formatting.py:139 body 141 |  |  | 0.605 |
| walker |  | 3491 | 14 | python method body at src/click/formatting.py:143 body 145 |  |  | 0.605 |
| ns | 3498 |  | 258 | @confirmation_option / @password_option full bodies | 2.8 | 2.1 | 0.575 |
| walker |  | 3545 | 54 | python method at src/click/formatting.py:116 |  |  | 0.575 |
| walker |  | 3564 | 19 | python method body at src/click/formatting.py:189 body 191 |  |  | 0.575 |
| walker |  | 3620 | 56 | python method at src/click/formatting.py:210 |  |  | 0.575 |
| walker |  | 3654 | 34 | python method doc at src/click/formatting.py:194 |  |  | 0.575 |
| walker |  | 3676 | 22 | python method body at src/click/formatting.py:185 body 187 |  |  | 0.575 |
| walker |  | 3727 | 51 | python method doc at src/click/formatting.py:254 |  |  | 0.575 |
| ns | 3738 |  | 240 | @version_option signature + key kwargs | 2.9 | 2.1 | 0.555 |
| walker |  | 3777 | 50 | python imports in src/click/globals.py |  |  | 0.555 |
| walker |  | 3904 | 127 | listing of 'tests' |  |  | 0.555 |
| walker |  | 3960 | 56 | listing of 'tests/typing' |  |  | 0.555 |
| walker |  | 4065 | 105 | python decl names surface in src/click/_utils.py |  |  | 0.555 |
| walker |  | 4065 | 0 | python decl at src/click/_utils.py:7 |  |  | 0.555 |
| walker |  | 4081 | 16 | python method sigs in src/click/_utils.py |  |  | 0.555 |
| walker |  | 4081 | 0 | python method at src/click/_utils.py:18 |  |  | 0.555 |
| ns | 4085 |  | 347 | quickstart.md: 'Basic Concepts - Creating a Command' section | 2.10 |  | 0.523 |
| walker |  | 4103 | 22 | python class body at src/click/_utils.py:7 |  |  | 0.523 |
| walker |  | 4119 | 16 | python method body at src/click/_utils.py:18 body 19 |  |  | 0.523 |
| walker |  | 4174 | 55 | python decl doc at src/click/_utils.py:7 |  |  | 0.523 |
| ns | 4205 |  | 120 | core.py: every class header (top-level inventory) | 3.1 |  | 0.514 |
| walker |  | 4279 | 105 | python decl names surface in src/click/globals.py |  |  | 0.514 |
| walker |  | 4279 | 0 | python decl at src/click/globals.py:12 |  |  | 0.514 |
| walker |  | 4279 | 0 | python decl at src/click/globals.py:16 |  |  | 0.514 |
| walker |  | 4279 | 0 | python decl at src/click/globals.py:20 |  |  | 0.514 |
| walker |  | 4279 | 0 | python decl at src/click/globals.py:44 |  |  | 0.514 |
| walker |  | 4279 | 0 | python decl at src/click/globals.py:49 |  |  | 0.514 |
| walker |  | 4279 | 0 | python decl at src/click/globals.py:54 |  |  | 0.514 |
| walker |  | 4289 | 10 | python decl body at src/click/globals.py:49 body 51 |  |  | 0.514 |
| walker |  | 4302 | 13 | python decl doc at src/click/globals.py:49 |  |  | 0.514 |
| walker |  | 4316 | 14 | python decl doc at src/click/globals.py:44 |  |  | 0.514 |
| walker |  | 4334 | 18 | python decl body at src/click/globals.py:16 body 17 |  |  | 0.514 |
| ns | 4349 |  | 144 | Context: one-paragraph lede | 3.2 | 3.1 | 0.504 |
| walker |  | 4353 | 19 | python decl body at src/click/globals.py:44 body 46 |  |  | 0.504 |
| walker |  | 4375 | 22 | python decl body at src/click/globals.py:12 body 13 |  |  | 0.504 |
| walker |  | 4428 | 53 | python decl doc at src/click/globals.py:54 |  |  | 0.504 |
| walker |  | 4436 | 8 | python decl body at src/click/globals.py:20 body 41 |  |  | 0.504 |
| walker |  | 4513 | 77 | python method doc at src/click/formatting.py:147 |  |  | 0.504 |
| ns | 4618 |  | 269 | Context.__init__ signature | 3.3 | 3.2 | 0.489 |
| walker |  | 4640 | 127 | python decl names surface in src/click/parser.py |  |  | 0.489 |
| walker |  | 4640 | 0 | python decl at src/click/parser.py:111 |  |  | 0.489 |
| walker |  | 4640 | 0 | python decl at src/click/parser.py:120 |  |  | 0.489 |
| walker |  | 4640 | 0 | python decl at src/click/parser.py:127 |  |  | 0.489 |
| walker |  | 4640 | 0 | python decl at src/click/parser.py:185 |  |  | 0.489 |
| walker |  | 4640 | 0 | python decl at src/click/parser.py:216 |  |  | 0.489 |
| walker |  | 4640 | 0 | python decl at src/click/parser.py:224 |  |  | 0.489 |
| walker |  | 4640 | 0 | python decl at src/click/parser.py:503 |  |  | 0.489 |
| walker |  | 4687 | 47 | python decl at src/click/parser.py:51 |  |  | 0.489 |
| walker |  | 4762 | 75 | python decl body at src/click/formatting.py:14 body 15 |  |  | 0.489 |
| walker |  | 4830 | 68 | python imports in src/click/formatting.py |  |  | 0.489 |
| walker |  | 4839 | 9 | python method body at src/click/formatting.py:269 body 272 |  |  | 0.489 |
| ns | 5031 |  | 413 | Context: location of every method | 3.4 | 3.2 | 0.473 |
| walker |  | 5149 | 310 | python method sigs in src/click/parser.py |  |  | 0.473 |
| walker |  | 5149 | 0 | python method at src/click/parser.py:169 |  |  | 0.473 |
| walker |  | 5149 | 0 | python method at src/click/parser.py:186 |  |  | 0.473 |
| walker |  | 5149 | 0 | python method at src/click/parser.py:217 |  |  | 0.473 |
| walker |  | 5149 | 0 | python method at src/click/parser.py:290 |  |  | 0.473 |
| walker |  | 5149 | 0 | python method at src/click/parser.py:316 |  |  | 0.473 |
| walker |  | 5149 | 0 | python method at src/click/parser.py:327 |  |  | 0.473 |
| walker |  | 5149 | 0 | python method at src/click/parser.py:390 |  |  | 0.473 |
| walker |  | 5149 | 0 | python method at src/click/parser.py:470 |  |  | 0.473 |
| ns | 5156 |  | 125 | Command: one-paragraph lede + headline kwargs | 3.5 | 3.1 | 0.467 |
| walker |  | 5160 | 11 | python method at src/click/parser.py:165 |  |  | 0.467 |
| walker |  | 5193 | 33 | python method at src/click/parser.py:298 |  |  | 0.467 |
| walker |  | 5229 | 36 | python method at src/click/parser.py:241 |  |  | 0.467 |
| walker |  | 5244 | 15 | python method body at src/click/parser.py:165 body 167 |  |  | 0.467 |
| walker |  | 5292 | 48 | python method at src/click/parser.py:191 |  |  | 0.467 |
| walker |  | 5313 | 21 | python method body at src/click/parser.py:290 body 296 |  |  | 0.467 |
| walker |  | 5344 | 31 | python method at src/click/parser.py:363 |  |  | 0.467 |
| ns | 5357 |  | 201 | Command.__init__ signature | 3.6 | 3.5 | 0.458 |
| walker |  | 5427 | 83 | python method at src/click/parser.py:128 |  |  | 0.458 |
| walker |  | 5482 | 55 | python method doc at src/click/parser.py:290 |  |  | 0.458 |
| walker |  | 5511 | 29 | python method body at src/click/parser.py:186 body 187 |  |  | 0.458 |
| walker |  | 5597 | 86 | python method at src/click/parser.py:265 |  |  | 0.458 |
| walker |  | 5643 | 46 | python method at src/click/parser.py:430 |  |  | 0.458 |
| walker |  | 5742 | 99 | python method doc at src/click/parser.py:298 |  |  | 0.458 |
| ns | 5745 |  | 388 | Command: location of every method | 3.7 | 3.5 | 0.446 |
| walker |  | 5898 | 156 | python decl names surface in src/click/_termui_impl.py |  |  | 0.446 |
| walker |  | 5898 | 0 | python decl at src/click/_termui_impl.py:43 |  |  | 0.446 |
| walker |  | 5898 | 0 | python decl at src/click/_termui_impl.py:375 |  |  | 0.446 |
| walker |  | 5898 | 0 | python decl at src/click/_termui_impl.py:608 |  |  | 0.446 |
| walker |  | 5898 | 0 | python decl at src/click/_termui_impl.py:724 |  |  | 0.446 |
| walker |  | 5898 | 0 | python decl at src/click/_termui_impl.py:794 |  |  | 0.446 |
| walker |  | 5925 | 27 | python decl at src/click/_termui_impl.py:417 |  |  | 0.446 |
| walker |  | 5968 | 43 | python class body at src/click/_termui_impl.py:608 |  |  | 0.446 |
| walker |  | 6002 | 34 | python decl at src/click/_termui_impl.py:386 |  |  | 0.446 |
| walker |  | 6019 | 17 | python decl doc at src/click/_termui_impl.py:386 |  |  | 0.446 |
| walker |  | 6063 | 44 | python decl at src/click/_termui_impl.py:597 |  |  | 0.446 |
| walker |  | 6082 | 19 | python decl doc at src/click/_termui_impl.py:597 |  |  | 0.446 |
| walker |  | 6164 | 82 | python decl doc at src/click/_termui_impl.py:417 |  |  | 0.446 |
| ns | 6201 |  | 456 | Group: lede + __init__ signature | 3.8 | 3.1 | 0.431 |
| walker |  | 6212 | 48 | python decl at src/click/_termui_impl.py:442 |  |  | 0.431 |
| walker |  | 6261 | 49 | python decl at src/click/_termui_impl.py:546 |  |  | 0.431 |
| walker |  | 6316 | 55 | python method at src/click/_textwrap.py:9 |  |  | 0.431 |
| ns | 6415 |  | 214 | Group: method-locations + decorator-style command/group registrars | 3.9 | 3.8 | 0.425 |
| walker |  | 6419 | 103 | python method doc at src/click/formatting.py:210 |  |  | 0.425 |
| walker |  | 6486 | 67 | python decl names surface #1 in src/click/utils.py |  |  | 0.425 |
| walker |  | 6486 | 0 | python decl at src/click/utils.py:453 |  |  | 0.425 |
| walker |  | 6486 | 0 | python decl at src/click/utils.py:502 |  |  | 0.425 |
| walker |  | 6545 | 59 | python method sigs #1 in src/click/utils.py |  |  | 0.425 |
| walker |  | 6545 | 0 | python method at src/click/utils.py:511 |  |  | 0.425 |
| walker |  | 6545 | 0 | python method at src/click/utils.py:514 |  |  | 0.425 |
| walker |  | 6545 | 0 | python method at src/click/utils.py:523 |  |  | 0.425 |
| walker |  | 6554 | 9 | python method body at src/click/utils.py:511 body 512 |  |  | 0.425 |
| walker |  | 6565 | 11 | python method body at src/click/utils.py:523 body 524 |  |  | 0.425 |
| ns | 6567 |  | 152 | Parameter (ABC): lede + key kwargs paragraph | 3.10 | 3.1 | 0.420 |
| walker |  | 6594 | 29 | python decl at src/click/utils.py:527 |  |  | 0.420 |
| walker |  | 6723 | 129 | python decl doc at src/click/utils.py:502 |  |  | 0.420 |
| walker |  | 6783 | 60 | python decl at src/click/utils.py:582 |  |  | 0.420 |
| ns | 6856 |  | 289 | Parameter.__init__ signature | 3.11 | 3.10 | 0.411 |
| walker |  | 6975 | 192 | python decl doc at src/click/globals.py:20 |  |  | 0.411 |
| walker |  | 7065 | 90 | python imports in src/click/_compat.py |  |  | 0.411 |
| walker |  | 7076 | 11 | python method body at src/click/parser.py:169 body 182 |  |  | 0.411 |
| walker |  | 7087 | 11 | listing of '.devcontainer' |  |  | 0.411 |
| ns | 7202 |  | 346 | Option: lede + __init__ signature | 3.12 | 3.10 | 0.399 |
| walker |  | 7209 | 122 | python method doc at src/click/parser.py:265 |  |  | 0.399 |
| ns | 7355 |  | 153 | Argument: lede + __init__ signature + required-by-default logic | 3.13 | 3.10 | 0.394 |
| walker |  | 7469 | 260 | python decl doc at src/click/testing.py:183 |  |  | 0.395 |
| walker |  | 7524 | 55 | python method body at src/click/parser.py:217 body 218 |  |  | 0.395 |
| walker |  | 7579 | 55 | python method body at src/click/utils.py:514 body 515 |  |  | 0.395 |
| walker |  | 7675 | 96 | python decl doc at src/click/_termui_impl.py:546 |  |  | 0.395 |
| ns | 7722 |  | 367 | Parameter/Option/Argument: method locations | 3.14 | 3.11 | 0.388 |
| walker |  | 7946 | 271 | python decl doc at src/click/testing.py:261 |  |  | 0.388 |
| ns | 8019 |  | 297 | types.py: every ParamType subclass header | 4.1 |  | 0.381 |
| walker |  | 8102 | 156 | python decl names surface in src/click/shell_completion.py |  |  | 0.381 |
| walker |  | 8102 | 0 | python decl at src/click/shell_completion.py:57 |  |  | 0.381 |
| walker |  | 8102 | 0 | python decl at src/click/shell_completion.py:204 |  |  | 0.381 |
| walker |  | 8102 | 0 | python decl at src/click/shell_completion.py:308 |  |  | 0.381 |
| walker |  | 8102 | 0 | python decl at src/click/shell_completion.py:367 |  |  | 0.381 |
| walker |  | 8102 | 0 | python decl at src/click/shell_completion.py:403 |  |  | 0.381 |
| walker |  | 8111 | 9 | python decl doc at src/click/shell_completion.py:308 |  |  | 0.381 |
| walker |  | 8120 | 9 | python decl doc at src/click/shell_completion.py:403 |  |  | 0.381 |
| walker |  | 8130 | 10 | python decl doc at src/click/shell_completion.py:367 |  |  | 0.381 |
| walker |  | 8157 | 27 | python decl at src/click/shell_completion.py:449 |  |  | 0.381 |
| walker |  | 8178 | 21 | python class body at src/click/shell_completion.py:57 |  |  | 0.381 |
| walker |  | 8201 | 23 | python class body at src/click/shell_completion.py:308 |  |  | 0.381 |
| walker |  | 8224 | 23 | python class body at src/click/shell_completion.py:403 |  |  | 0.381 |
| walker |  | 8248 | 24 | python class body at src/click/shell_completion.py:367 |  |  | 0.381 |
| ns | 8250 |  | 231 | ParamType: the subclassing recipe | 4.2 | 4.1 | 0.375 |
| walker |  | 8309 | 61 | python decl at src/click/shell_completion.py:19 |  |  | 0.375 |
| walker |  | 8427 | 118 | python class body at src/click/shell_completion.py:204 |  |  | 0.375 |
| ns | 8431 |  | 181 | ParamType.convert + ParamType.shell_complete signatures | 4.3 | 4.2 | 0.370 |
| walker |  | 8467 | 40 | python decl at src/click/shell_completion.py:442 |  |  | 0.370 |
| ns | 8564 |  | 133 | exceptions.py: every class header | 4.4 |  | 0.367 |
| walker |  | 8594 | 127 | python decl doc at src/click/shell_completion.py:204 |  |  | 0.367 |
| walker |  | 8703 | 109 | python decl doc at src/click/shell_completion.py:449 |  |  | 0.367 |
| ns | 8790 |  | 226 | ClickException + UsageError ledes + exit_code | 4.5 | 4.4 | 0.361 |
| ns | 8844 |  | 54 | testing.py: CliRunner, Result, helper class locations | 5.1 |  | 0.368 |
| walker |  | 8859 | 156 | python decl doc at src/click/shell_completion.py:19 |  |  | 0.368 |
| ns | 8975 |  | 131 | CliRunner.invoke signature | 5.2 | 5.1 | 0.365 |
| ns | 9163 |  | 188 | Result: attribute documentation | 5.3 | 5.1 | 0.379 |
| walker |  | 9173 | 314 | python method sigs in src/click/shell_completion.py |  |  | 0.379 |
| walker |  | 9173 | 0 | python method at src/click/shell_completion.py:90 |  |  | 0.379 |
| walker |  | 9173 | 0 | python method at src/click/shell_completion.py:248 |  |  | 0.379 |
| walker |  | 9173 | 0 | python method at src/click/shell_completion.py:260 |  |  | 0.379 |
| walker |  | 9173 | 0 | python method at src/click/shell_completion.py:268 |  |  | 0.379 |
| walker |  | 9173 | 0 | python method at src/click/shell_completion.py:275 |  |  | 0.379 |
| walker |  | 9173 | 0 | python method at src/click/shell_completion.py:287 |  |  | 0.379 |
| walker |  | 9173 | 0 | python method at src/click/shell_completion.py:295 |  |  | 0.379 |
| walker |  | 9173 | 0 | python method at src/click/shell_completion.py:347 |  |  | 0.379 |
| walker |  | 9173 | 0 | python method at src/click/shell_completion.py:351 |  |  | 0.379 |
| walker |  | 9173 | 0 | python method at src/click/shell_completion.py:363 |  |  | 0.379 |
| walker |  | 9173 | 0 | python method at src/click/shell_completion.py:373 |  |  | 0.379 |
| walker |  | 9173 | 0 | python method at src/click/shell_completion.py:385 |  |  | 0.379 |
| walker |  | 9173 | 0 | python method at src/click/shell_completion.py:409 |  |  | 0.379 |
| walker |  | 9173 | 0 | python method at src/click/shell_completion.py:423 |  |  | 0.379 |
| walker |  | 9184 | 11 | python method at src/click/shell_completion.py:240 |  |  | 0.379 |
| walker |  | 9195 | 11 | python method at src/click/shell_completion.py:314 |  |  | 0.379 |
| walker |  | 9205 | 10 | python method body at src/click/shell_completion.py:90 body 91 |  |  | 0.379 |
| walker |  | 9215 | 10 | python method body at src/click/shell_completion.py:268 body 273 |  |  | 0.379 |
| ns | 9224 |  | 61 | tests/conftest.py: the canonical 'runner' fixture | 5.4 | 5.1 | 0.376 |
| walker |  | 9225 | 10 | python method body at src/click/shell_completion.py:287 body 293 |  |  | 0.376 |
| walker |  | 9237 | 12 | python method body at src/click/shell_completion.py:363 body 364 |  |  | 0.376 |
| walker |  | 9251 | 14 | python method body at src/click/shell_completion.py:260 body 266 |  |  | 0.376 |
| walker |  | 9281 | 30 | python method doc at src/click/shell_completion.py:240 |  |  | 0.376 |
| walker |  | 9299 | 18 | python method body at src/click/shell_completion.py:347 body 348 |  |  | 0.376 |
| walker |  | 9359 | 60 | python method at src/click/shell_completion.py:78 |  |  | 0.376 |
| ns | 9391 |  | 167 | termui.py: every public function location | 5.5 |  | 0.372 |
| walker |  | 9419 | 60 | python method at src/click/shell_completion.py:228 |  |  | 0.372 |
| walker |  | 9468 | 49 | python method doc at src/click/shell_completion.py:248 |  |  | 0.372 |
| ns | 9472 |  | 81 | utils.py: echo() signature + key kwargs | 5.6 |  | 0.370 |
| ns | 9508 |  | 36 | examples/ subdirectory listing | 6.1 |  | 0.382 |
| walker |  | 9517 | 49 | python method doc at src/click/shell_completion.py:268 |  |  | 0.382 |
| walker |  | 9567 | 50 | python method doc at src/click/shell_completion.py:287 |  |  | 0.382 |
| walker |  | 9631 | 64 | python method doc at src/click/shell_completion.py:260 |  |  | 0.382 |
| walker |  | 9697 | 66 | python method doc at src/click/shell_completion.py:295 |  |  | 0.382 |
| ns | 9715 |  | 207 | examples/naval/naval.py: canonical nested-group example | 6.2 |  | 0.374 |
| walker |  | 9783 | 86 | python method doc at src/click/shell_completion.py:423 |  |  | 0.374 |
| walker |  | 9872 | 89 | python method doc at src/click/shell_completion.py:275 |  |  | 0.374 |
| ns | 9915 |  | 200 | docs/index.rst: General Reference toctree | 6.3 |  | 0.367 |
| walker |  | 9919 | 47 | python method body at src/click/shell_completion.py:228 body 235 |  |  | 0.367 |
| walker |  | 9927 | 8 | python decl body at src/click/shell_completion.py:449 body 466 |  |  | 0.367 |
| walker |  | 9977 | 50 | python method body at src/click/shell_completion.py:78 body 85 |  |  | 0.367 |

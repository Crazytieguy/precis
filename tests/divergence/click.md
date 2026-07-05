Score(3000)=0.600 I=0.839 C=0.430 ns_rows≤3K=20/52 (reached=11 partial=0 missing=9)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 50 | 50 | listing of '.' |  |  | 1.000 |
| ns | 50 |  | 50 | Top-level directory listing | 1.1 |  | 1.000 |
| walker |  | 53 | 3 | listing of 'src' |  |  | 1.000 |
| ns | 94 |  | 44 | Package name + version | 1.2 |  | 0.859 |
| walker |  | 134 | 81 | README headline in README.md |  |  | 0.889 |
| walker |  | 164 | 30 | headings outline in README.md |  |  | 0.889 |
| ns | 175 |  | 81 | README headline + tagline | 1.3 |  | 0.889 |
| walker |  | 195 | 31 | [dependencies] in pyproject.toml |  |  | 0.898 |
| ns | 217 |  | 42 | Python version floor + runtime dep | 1.4 | 1.2 | 0.859 |
| walker |  | 294 | 99 | README.md section #0 |  |  | 0.889 |
| ns | 318 |  | 101 | README differentiators (three-bullet pitch) | 1.5 | 1.3 | 0.886 |
| walker |  | 372 | 78 | [package] in pyproject.toml |  |  | 1.000 |
| ns | 422 |  | 104 | __init__.py: core-class re-exports | 1.6 |  | 0.889 |
| walker |  | 458 | 86 | listing of 'src/click' |  |  | 0.889 |
| walker |  | 473 | 15 | python imports #1 in src/click/__init__.py |  |  | 0.889 |
| walker |  | 544 | 71 | python imports in src/click/__init__.py |  |  | 0.889 |
| walker |  | 562 | 18 | python imports #6 in src/click/__init__.py |  |  | 0.890 |
| ns | 590 |  | 168 | README hello-world example | 1.7 | 1.3 | 0.743 |
| walker |  | 592 | 30 | python imports #5 in src/click/__init__.py |  |  | 0.743 |
| walker |  | 692 | 100 | python imports #2 in src/click/__init__.py |  |  | 0.836 |
| ns | 751 |  | 161 | __init__.py: decorator re-exports | 1.8 | 1.6 | 0.757 |
| walker |  | 853 | 161 | python imports #3 in src/click/__init__.py |  |  | 0.858 |
| walker |  | 871 | 18 | python decl names surface in src/click/__init__.py |  |  | 0.858 |
| walker |  | 871 | 0 | python decl at src/click/__init__.py:77 |  |  | 0.858 |
| ns | 943 |  | 192 | __init__.py: exception + formatting + globals re-exports | 1.9 | 1.8 | 0.783 |
| walker |  | 1015 | 144 | python imports #4 in src/click/__init__.py |  |  | 0.876 |
| walker |  | 1103 | 88 | python imports #9 in src/click/__init__.py |  |  | 0.881 |
| ns | 1132 |  | 189 | __init__.py: termui re-exports | 1.10 | 1.9 | 0.810 |
| walker |  | 1292 | 189 | python imports #7 in src/click/__init__.py |  |  | 0.895 |
| ns | 1314 |  | 182 | __init__.py: built-in type re-exports | 1.11 | 1.10 | 0.827 |
| ns | 1407 |  | 93 | __init__.py: utils re-exports | 1.12 | 1.11 | 0.826 |
| walker |  | 1472 | 180 | python imports #8 in src/click/__init__.py |  |  | 0.900 |
| walker |  | 1485 | 13 | listing of '.github' |  |  | 0.900 |
| walker |  | 1508 | 23 | listing of '.github/workflows' |  |  | 0.900 |
| walker |  | 1524 | 16 | python decl names surface in src/click/_textwrap.py |  |  | 0.900 |
| walker |  | 1524 | 0 | python decl at src/click/_textwrap.py:8 |  |  | 0.900 |
| walker |  | 1565 | 41 | python method sigs in src/click/_textwrap.py |  |  | 0.900 |
| walker |  | 1565 | 0 | python method at src/click/_textwrap.py:40 |  |  | 0.900 |
| walker |  | 1586 | 21 | python method at src/click/_textwrap.py:27 |  |  | 0.900 |
| ns | 1659 |  | 252 | Deprecation shim: BaseCommand → Command, MultiCommand → Group | 1.13 | 1.12 | 0.809 |
| walker |  | 1661 | 75 | README.md section #3 |  |  | 0.809 |
| walker |  | 1751 | 90 | README.md section #2 |  |  | 0.809 |
| walker |  | 1787 | 36 | listing of 'examples' |  |  | 0.810 |
| ns | 1926 |  | 267 | Deprecation shim: OptionParser, __version__ | 1.14 | 1.13 | 0.737 |
| walker |  | 1979 | 192 | listing of 'docs' |  |  | 0.737 |
| walker |  | 1994 | 15 | listing of 'docs/_static' |  |  | 0.737 |
| walker |  | 2023 | 29 | docs/setuptools.md section #0 |  |  | 0.737 |
| walker |  | 2076 | 53 | docs/license.md section #0 |  |  | 0.737 |
| walker |  | 2097 | 21 | headings outline in docs/click-concepts.md |  |  | 0.737 |
| walker |  | 2118 | 21 | headings outline in docs/unicode-support.md |  |  | 0.737 |
| walker |  | 2140 | 22 | headings outline in docs/command-line-reference.md |  |  | 0.737 |
| ns | 2157 |  | 231 | decorators.py: location of every public decorator | 2.1 |  | 0.708 |
| walker |  | 2162 | 22 | headings outline in docs/design-opinions.md |  |  | 0.708 |
| walker |  | 2230 | 68 | docs/design-opinions.md section #0 |  |  | 0.708 |
| ns | 2245 |  | 88 | @command: signature + 1-liner | 2.2 | 2.1 | 0.695 |
| walker |  | 2254 | 24 | headings outline in docs/contrib.md |  |  | 0.695 |
| walker |  | 2332 | 78 | docs/click-concepts.md section #0 |  |  | 0.695 |
| ns | 2403 |  | 158 | @option: signature + 1-liner | 2.3 | 2.1 | 0.674 |
| walker |  | 2413 | 81 | docs/command-line-reference.md section #0 |  |  | 0.674 |
| walker |  | 2540 | 127 | listing of 'tests' |  |  | 0.674 |
| ns | 2561 |  | 158 | @argument: signature + 1-liner | 2.4 | 2.1 | 0.654 |
| walker |  | 2570 | 30 | headings outline in docs/entry-points.md |  |  | 0.654 |
| walker |  | 2594 | 24 | python decl names surface #1 in src/click/exceptions.py |  |  | 0.655 |
| walker |  | 2594 | 0 | python decl at src/click/exceptions.py:330 |  |  | 0.655 |
| walker |  | 2594 | 0 | python decl at src/click/exceptions.py:334 |  |  | 0.655 |
| walker |  | 2610 | 16 | python class body at src/click/exceptions.py:334 |  |  | 0.655 |
| walker |  | 2624 | 14 | python decl doc at src/click/exceptions.py:330 |  |  | 0.655 |
| walker |  | 2675 | 51 | python decl doc at src/click/exceptions.py:334 |  |  | 0.655 |
| walker |  | 2707 | 32 | headings outline in docs/extending-click.md |  |  | 0.655 |
| ns | 2724 |  | 163 | @group: signature + 1-liner | 2.5 | 2.1 | 0.633 |
| walker |  | 2740 | 33 | headings outline in docs/wincmd.md |  |  | 0.633 |
| walker |  | 2827 | 87 | docs/wincmd.md section #0 |  |  | 0.633 |
| ns | 2955 |  | 231 | @pass_context / @pass_obj signatures + docstrings | 2.6 | 2.1 | 0.600 |
| walker |  | 3047 | 220 | README.md section #1 |  |  | 0.663 |
| walker |  | 3081 | 34 | python imports in src/click/_utils.py |  |  | 0.663 |
| walker |  | 3117 | 36 | headings outline in docs/faqs.md |  |  | 0.663 |
| walker |  | 3153 | 36 | docs/faqs.md section #0 |  |  | 0.663 |
| walker |  | 3189 | 36 | python imports in src/click/globals.py |  |  | 0.663 |
| ns | 3272 |  | 317 | @make_pass_decorator: signature + docstring + how-it-works | 2.7 | 2.1 | 0.626 |
| walker |  | 3313 | 124 | docs/extending-click.md section #0 |  |  | 0.626 |
| walker |  | 3410 | 97 | python decl names surface in src/click/testing.py |  |  | 0.627 |
| walker |  | 3410 | 0 | python decl at src/click/testing.py:26 |  |  | 0.627 |
| walker |  | 3410 | 0 | python decl at src/click/testing.py:70 |  |  | 0.627 |
| walker |  | 3410 | 0 | python decl at src/click/testing.py:89 |  |  | 0.627 |
| walker |  | 3410 | 0 | python decl at src/click/testing.py:103 |  |  | 0.627 |
| walker |  | 3410 | 0 | python decl at src/click/testing.py:183 |  |  | 0.627 |
| walker |  | 3410 | 0 | python decl at src/click/testing.py:261 |  |  | 0.627 |
| walker |  | 3455 | 45 | python decl at src/click/testing.py:163 |  |  | 0.627 |
| walker |  | 3479 | 24 | python decl at src/click/testing.py:60 |  |  | 0.627 |
| walker |  | 3526 | 47 | python decl doc at src/click/testing.py:70 |  |  | 0.627 |
| ns | 3530 |  | 258 | @confirmation_option / @password_option full bodies | 2.8 | 2.1 | 0.596 |
| walker |  | 3589 | 63 | python decl doc at src/click/testing.py:89 |  |  | 0.596 |
| walker |  | 3628 | 39 | headings outline in docs/parameters.md |  |  | 0.596 |
| walker |  | 3675 | 47 | docs/parameters.md section #0 |  |  | 0.596 |
| walker |  | 3715 | 40 | headings outline in docs/virtualenv.md |  |  | 0.596 |
| walker |  | 3715 | 0 | docs/virtualenv.md section #0 |  |  | 0.596 |
| walker |  | 3757 | 42 | headings outline in docs/option-decorators.md |  |  | 0.596 |
| ns | 3770 |  | 240 | @version_option signature + key kwargs | 2.9 | 2.1 | 0.575 |
| walker |  | 3788 | 31 | docs/option-decorators.md section #3 |  |  | 0.575 |
| walker |  | 3878 | 90 | docs/option-decorators.md section #0 |  |  | 0.575 |
| walker |  | 4117 | 239 | python method sigs in src/click/testing.py |  |  | 0.575 |
| walker |  | 4117 | 0 | python method at src/click/testing.py:27 |  |  | 0.575 |
| walker |  | 4117 | 0 | python method at src/click/testing.py:32 |  |  | 0.575 |
| walker |  | 4117 | 0 | python method at src/click/testing.py:35 |  |  | 0.575 |
| walker |  | 4117 | 0 | python method at src/click/testing.py:41 |  |  | 0.575 |
| walker |  | 4117 | 0 | python method at src/click/testing.py:44 |  |  | 0.575 |
| walker |  | 4117 | 0 | python method at src/click/testing.py:47 |  |  | 0.575 |
| walker |  | 4117 | 0 | python method at src/click/testing.py:50 |  |  | 0.575 |
| walker |  | 4117 | 0 | python method at src/click/testing.py:53 |  |  | 0.575 |
| walker |  | 4117 | 0 | python method at src/click/testing.py:56 |  |  | 0.575 |
| walker |  | 4117 | 0 | python method at src/click/testing.py:76 |  |  | 0.575 |
| walker |  | 4117 | 0 | python method at src/click/testing.py:80 |  |  | 0.575 |
| walker |  | 4117 | 0 | python method at src/click/testing.py:84 |  |  | 0.575 |
| ns | 4121 |  | 351 | quickstart.md: 'Basic Concepts - Creating a Command' section | 2.10 |  | 0.542 |
| walker |  | 4126 | 9 | python method body at src/click/testing.py:56 body 57 |  |  | 0.542 |
| walker |  | 4137 | 11 | python method body at src/click/testing.py:32 body 33 |  |  | 0.542 |
| walker |  | 4150 | 13 | python method body at src/click/testing.py:41 body 42 |  |  | 0.542 |
| walker |  | 4163 | 13 | python method body at src/click/testing.py:47 body 48 |  |  | 0.542 |
| walker |  | 4180 | 17 | python method body at src/click/testing.py:53 body 54 |  |  | 0.542 |
| walker |  | 4197 | 17 | python method body at src/click/testing.py:80 body 81 |  |  | 0.542 |
| walker |  | 4215 | 18 | python method body at src/click/testing.py:50 body 51 |  |  | 0.542 |
| walker |  | 4234 | 19 | python method body at src/click/testing.py:76 body 77 |  |  | 0.542 |
| walker |  | 4254 | 20 | python method body at src/click/testing.py:44 body 45 |  |  | 0.542 |
| ns | 4265 |  | 144 | core.py: every class header (top-level inventory) | 3.1 |  | 0.533 |
| walker |  | 4274 | 20 | python method body at src/click/testing.py:84 body 85 |  |  | 0.533 |
| walker |  | 4359 | 85 | python decl names surface in src/click/globals.py |  |  | 0.533 |
| walker |  | 4359 | 0 | python decl at src/click/globals.py:20 |  |  | 0.533 |
| walker |  | 4359 | 0 | python decl at src/click/globals.py:44 |  |  | 0.533 |
| walker |  | 4359 | 0 | python decl at src/click/globals.py:49 |  |  | 0.533 |
| walker |  | 4359 | 0 | python decl at src/click/globals.py:54 |  |  | 0.533 |
| walker |  | 4368 | 9 | python decl at src/click/globals.py:12 |  |  | 0.533 |
| walker |  | 4379 | 11 | python decl at src/click/globals.py:16 |  |  | 0.533 |
| walker |  | 4389 | 10 | python decl body at src/click/globals.py:49 body 51 |  |  | 0.533 |
| walker |  | 4402 | 13 | python decl doc at src/click/globals.py:49 |  |  | 0.533 |
| ns | 4409 |  | 144 | Context: one-paragraph lede | 3.2 | 3.1 | 0.522 |
| walker |  | 4418 | 16 | python decl doc at src/click/globals.py:44 |  |  | 0.522 |
| walker |  | 4435 | 17 | python decl body at src/click/globals.py:44 body 46 |  |  | 0.522 |
| walker |  | 4453 | 18 | python decl body at src/click/globals.py:16 body 17 |  |  | 0.522 |
| walker |  | 4475 | 22 | python decl body at src/click/globals.py:12 body 13 |  |  | 0.522 |
| walker |  | 4530 | 55 | python decl doc at src/click/globals.py:54 |  |  | 0.522 |
| walker |  | 4538 | 8 | python decl body at src/click/globals.py:20 body 41 |  |  | 0.522 |
| walker |  | 4583 | 45 | headings outline in docs/handling-files.md |  |  | 0.522 |
| walker |  | 4660 | 77 | docs/handling-files.md section #0 |  |  | 0.522 |
| ns | 4680 |  | 271 | Context.__init__ signature | 3.3 | 3.2 | 0.507 |
| walker |  | 4705 | 45 | python imports in src/click/_textwrap.py |  |  | 0.507 |
| walker |  | 4865 | 160 | python decl names surface in src/click/_termui_impl.py |  |  | 0.507 |
| walker |  | 4865 | 0 | python decl at src/click/_termui_impl.py:43 |  |  | 0.507 |
| walker |  | 4865 | 0 | python decl at src/click/_termui_impl.py:375 |  |  | 0.507 |
| walker |  | 4865 | 0 | python decl at src/click/_termui_impl.py:608 |  |  | 0.507 |
| walker |  | 4865 | 0 | python decl at src/click/_termui_impl.py:724 |  |  | 0.507 |
| walker |  | 4865 | 0 | python decl at src/click/_termui_impl.py:794 |  |  | 0.507 |
| walker |  | 4894 | 29 | python decl at src/click/_termui_impl.py:417 |  |  | 0.507 |
| walker |  | 4939 | 45 | python class body at src/click/_termui_impl.py:608 |  |  | 0.507 |
| walker |  | 4975 | 36 | python decl at src/click/_termui_impl.py:386 |  |  | 0.507 |
| walker |  | 4992 | 17 | python decl doc at src/click/_termui_impl.py:386 |  |  | 0.507 |
| ns | 5139 |  | 459 | Context: location of every method | 3.4 | 3.2 | 0.490 |
| walker |  | 5155 | 163 | python method sigs in src/click/_termui_impl.py |  |  | 0.490 |
| walker |  | 5155 | 0 | python method at src/click/_termui_impl.py:115 |  |  | 0.490 |
| walker |  | 5155 | 0 | python method at src/click/_termui_impl.py:128 |  |  | 0.490 |
| walker |  | 5155 | 0 | python method at src/click/_termui_impl.py:142 |  |  | 0.490 |
| walker |  | 5155 | 0 | python method at src/click/_termui_impl.py:166 |  |  | 0.490 |
| walker |  | 5155 | 0 | python method at src/click/_termui_impl.py:187 |  |  | 0.490 |
| walker |  | 5155 | 0 | python method at src/click/_termui_impl.py:193 |  |  | 0.490 |
| walker |  | 5167 | 12 | python method at src/click/_termui_impl.py:148 |  |  | 0.490 |
| walker |  | 5179 | 12 | python method at src/click/_termui_impl.py:160 |  |  | 0.490 |
| walker |  | 5193 | 14 | python method at src/click/_termui_impl.py:154 |  |  | 0.490 |
| walker |  | 5251 | 58 | python method at src/click/_termui_impl.py:120 |  |  | 0.490 |
| walker |  | 5258 | 7 | python method body at src/click/_termui_impl.py:120 body 126 |  |  | 0.490 |
| ns | 5264 |  | 125 | Command: one-paragraph lede + headline kwargs | 3.5 | 3.1 | 0.484 |
| walker |  | 5281 | 23 | python method body at src/click/_termui_impl.py:193 body 194 |  |  | 0.484 |
| walker |  | 5327 | 46 | python decl at src/click/_termui_impl.py:597 |  |  | 0.484 |
| walker |  | 5346 | 19 | python decl doc at src/click/_termui_impl.py:597 |  |  | 0.484 |
| walker |  | 5428 | 82 | python decl doc at src/click/_termui_impl.py:417 |  |  | 0.484 |
| walker |  | 5454 | 26 | python method body at src/click/_termui_impl.py:115 body 116 |  |  | 0.484 |
| ns | 5467 |  | 203 | Command.__init__ signature | 3.6 | 3.5 | 0.474 |
| walker |  | 5504 | 50 | python decl at src/click/_termui_impl.py:442 |  |  | 0.474 |
| walker |  | 5555 | 51 | python decl at src/click/_termui_impl.py:546 |  |  | 0.474 |
| walker |  | 5570 | 15 | python decl names surface #3 in src/click/types.py |  |  | 0.474 |
| walker |  | 5712 | 142 | python method sigs #1 in src/click/testing.py |  |  | 0.474 |
| walker |  | 5712 | 0 | python method at src/click/testing.py:97 |  |  | 0.474 |
| walker |  | 5712 | 0 | python method at src/click/testing.py:131 |  |  | 0.474 |
| walker |  | 5712 | 0 | python method at src/click/testing.py:139 |  |  | 0.474 |
| walker |  | 5712 | 0 | python method at src/click/testing.py:256 |  |  | 0.474 |
| walker |  | 5724 | 12 | python method at src/click/testing.py:154 |  |  | 0.474 |
| walker |  | 5736 | 12 | python method at src/click/testing.py:158 |  |  | 0.474 |
| walker |  | 5748 | 12 | python method at src/click/testing.py:226 |  |  | 0.474 |
| walker |  | 5760 | 12 | python method at src/click/testing.py:238 |  |  | 0.474 |
| walker |  | 5772 | 12 | python method at src/click/testing.py:245 |  |  | 0.474 |
| walker |  | 5780 | 8 | python method body at src/click/testing.py:154 body 156 |  |  | 0.474 |
| walker |  | 5788 | 8 | python method body at src/click/testing.py:158 body 160 |  |  | 0.474 |
| walker |  | 5801 | 13 | python method doc at src/click/testing.py:238 |  |  | 0.474 |
| walker |  | 5876 | 75 | python method at src/click/testing.py:117 |  |  | 0.474 |
| ns | 5893 |  | 426 | Command: location of every method | 3.7 | 3.5 | 0.463 |
| walker |  | 5953 | 77 | python method at src/click/testing.py:283 |  |  | 0.463 |
| walker |  | 6010 | 57 | python method doc at src/click/testing.py:245 |  |  | 0.463 |
| walker |  | 6149 | 139 | python decl names surface in src/click/parser.py |  |  | 0.463 |
| walker |  | 6149 | 0 | python decl at src/click/parser.py:111 |  |  | 0.463 |
| walker |  | 6149 | 0 | python decl at src/click/parser.py:120 |  |  | 0.463 |
| walker |  | 6149 | 0 | python decl at src/click/parser.py:127 |  |  | 0.463 |
| walker |  | 6149 | 0 | python decl at src/click/parser.py:185 |  |  | 0.463 |
| walker |  | 6149 | 0 | python decl at src/click/parser.py:216 |  |  | 0.463 |
| walker |  | 6149 | 0 | python decl at src/click/parser.py:224 |  |  | 0.463 |
| walker |  | 6149 | 0 | python decl at src/click/parser.py:503 |  |  | 0.463 |
| ns | 6351 |  | 458 | Group: lede + __init__ signature | 3.8 | 3.1 | 0.446 |
| walker |  | 6459 | 310 | python method sigs in src/click/parser.py |  |  | 0.446 |
| walker |  | 6459 | 0 | python method at src/click/parser.py:169 |  |  | 0.446 |
| walker |  | 6459 | 0 | python method at src/click/parser.py:186 |  |  | 0.446 |
| walker |  | 6459 | 0 | python method at src/click/parser.py:217 |  |  | 0.446 |
| walker |  | 6459 | 0 | python method at src/click/parser.py:290 |  |  | 0.446 |
| walker |  | 6459 | 0 | python method at src/click/parser.py:316 |  |  | 0.446 |
| walker |  | 6459 | 0 | python method at src/click/parser.py:327 |  |  | 0.446 |
| walker |  | 6459 | 0 | python method at src/click/parser.py:390 |  |  | 0.446 |
| walker |  | 6459 | 0 | python method at src/click/parser.py:470 |  |  | 0.446 |
| walker |  | 6472 | 13 | python method at src/click/parser.py:165 |  |  | 0.446 |
| walker |  | 6507 | 35 | python method at src/click/parser.py:298 |  |  | 0.446 |
| walker |  | 6545 | 38 | python method at src/click/parser.py:241 |  |  | 0.446 |
| walker |  | 6558 | 13 | python method body at src/click/parser.py:165 body 167 |  |  | 0.446 |
| ns | 6587 |  | 236 | Group: method-locations + decorator-style command/group registrars | 3.9 | 3.8 | 0.441 |
| walker |  | 6608 | 50 | python method at src/click/parser.py:191 |  |  | 0.441 |
| walker |  | 6629 | 21 | python method body at src/click/parser.py:290 body 296 |  |  | 0.441 |
| walker |  | 6678 | 49 | python decl at src/click/parser.py:51 |  |  | 0.441 |
| walker |  | 6711 | 33 | python method at src/click/parser.py:363 |  |  | 0.441 |
| ns | 6739 |  | 152 | Parameter (ABC): lede + key kwargs paragraph | 3.10 | 3.1 | 0.435 |
| walker |  | 6796 | 85 | python method at src/click/parser.py:128 |  |  | 0.435 |
| walker |  | 6825 | 29 | python method body at src/click/parser.py:186 body 187 |  |  | 0.435 |
| walker |  | 6913 | 88 | python method at src/click/parser.py:265 |  |  | 0.435 |
| walker |  | 6969 | 56 | listing of 'tests/typing' |  |  | 0.435 |
| walker |  | 7029 | 60 | python method doc at src/click/parser.py:290 |  |  | 0.435 |
| ns | 7032 |  | 293 | Parameter.__init__ signature | 3.11 | 3.10 | 0.426 |
| walker |  | 7120 | 91 | python method at src/click/_termui_impl.py:134 |  |  | 0.426 |
| walker |  | 7128 | 8 | python method body at src/click/_termui_impl.py:134 body 140 |  |  | 0.426 |
| walker |  | 7159 | 31 | python method body at src/click/testing.py:27 body 28 |  |  | 0.426 |
| walker |  | 7212 | 53 | headings outline in docs/arguments.md |  |  | 0.426 |
| walker |  | 7369 | 157 | docs/arguments.md section #0 |  |  | 0.426 |
| ns | 7380 |  | 348 | Option: lede + __init__ signature | 3.12 | 3.10 | 0.414 |
| walker |  | 7424 | 55 | headings outline in docs/support-multiple-versions.md |  |  | 0.414 |
| ns | 7535 |  | 155 | Argument: lede + __init__ signature + required-by-default logic | 3.13 | 3.10 | 0.408 |
| walker |  | 7554 | 130 | docs/support-multiple-versions.md section #0 |  |  | 0.408 |
| walker |  | 7561 | 7 | python method body at src/click/_termui_impl.py:166 body 185 |  |  | 0.408 |
| walker |  | 7666 | 105 | python decl names surface in src/click/formatting.py |  |  | 0.408 |
| walker |  | 7666 | 0 | python decl at src/click/formatting.py:14 |  |  | 0.408 |
| walker |  | 7666 | 0 | python decl at src/click/formatting.py:104 |  |  | 0.408 |
| walker |  | 7666 | 0 | python decl at src/click/formatting.py:283 |  |  | 0.408 |
| walker |  | 7703 | 37 | python decl at src/click/formatting.py:24 |  |  | 0.408 |
| walker |  | 7768 | 65 | python decl at src/click/formatting.py:31 |  |  | 0.408 |
| walker |  | 7795 | 27 | python decl body at src/click/formatting.py:24 body 27 |  |  | 0.408 |
| ns | 7936 |  | 401 | Parameter/Option/Argument: method locations | 3.14 | 3.11 | 0.401 |
| walker |  | 7987 | 192 | python method sigs in src/click/formatting.py |  |  | 0.401 |
| walker |  | 7987 | 0 | python method at src/click/formatting.py:135 |  |  | 0.401 |
| walker |  | 7987 | 0 | python method at src/click/formatting.py:139 |  |  | 0.401 |
| walker |  | 7987 | 0 | python method at src/click/formatting.py:143 |  |  | 0.401 |
| walker |  | 7987 | 0 | python method at src/click/formatting.py:147 |  |  | 0.401 |
| walker |  | 7987 | 0 | python method at src/click/formatting.py:185 |  |  | 0.401 |
| walker |  | 7987 | 0 | python method at src/click/formatting.py:189 |  |  | 0.401 |
| walker |  | 7987 | 0 | python method at src/click/formatting.py:194 |  |  | 0.401 |
| walker |  | 7987 | 0 | python method at src/click/formatting.py:278 |  |  | 0.401 |
| walker |  | 8003 | 16 | python method at src/click/formatting.py:269 |  |  | 0.401 |
| walker |  | 8023 | 20 | python method at src/click/formatting.py:254 |  |  | 0.401 |
| walker |  | 8034 | 11 | python method doc at src/click/formatting.py:139 |  |  | 0.401 |
| walker |  | 8045 | 11 | python method doc at src/click/formatting.py:143 |  |  | 0.401 |
| walker |  | 8056 | 11 | python method doc at src/click/formatting.py:278 |  |  | 0.401 |
| walker |  | 8065 | 9 | python method body at src/click/formatting.py:278 body 280 |  |  | 0.401 |
| walker |  | 8078 | 13 | python method doc at src/click/formatting.py:185 |  |  | 0.401 |
| walker |  | 8091 | 13 | python method doc at src/click/formatting.py:189 |  |  | 0.401 |
| walker |  | 8105 | 14 | python method doc at src/click/formatting.py:269 |  |  | 0.401 |
| walker |  | 8115 | 10 | python method body at src/click/formatting.py:135 body 137 |  |  | 0.401 |
| walker |  | 8128 | 13 | python method doc at src/click/formatting.py:135 |  |  | 0.401 |
| walker |  | 8140 | 12 | python method body at src/click/formatting.py:139 body 141 |  |  | 0.401 |
| walker |  | 8152 | 12 | python method body at src/click/formatting.py:143 body 145 |  |  | 0.401 |
| walker |  | 8169 | 17 | python method body at src/click/formatting.py:189 body 191 |  |  | 0.401 |
| walker |  | 8225 | 56 | python method at src/click/formatting.py:116 |  |  | 0.401 |
| ns | 8271 |  | 335 | types.py: every ParamType subclass header | 4.1 |  | 0.394 |
| walker |  | 8283 | 58 | python method at src/click/formatting.py:210 |  |  | 0.394 |
| walker |  | 8303 | 20 | python method body at src/click/formatting.py:185 body 187 |  |  | 0.394 |
| walker |  | 8339 | 36 | python method doc at src/click/formatting.py:194 |  |  | 0.394 |
| walker |  | 8422 | 83 | python decl doc at src/click/formatting.py:283 |  |  | 0.394 |
| walker |  | 8478 | 56 | python method doc at src/click/formatting.py:254 |  |  | 0.394 |
| ns | 8502 |  | 231 | ParamType: the subclassing recipe | 4.2 | 4.1 | 0.388 |
| walker |  | 8613 | 135 | python decl doc at src/click/formatting.py:104 |  |  | 0.388 |
| walker |  | 8681 | 68 | python method doc at src/click/testing.py:131 |  |  | 0.388 |
| ns | 8687 |  | 185 | ParamType.convert + ParamType.shell_complete signatures | 4.3 | 4.2 | 0.383 |
| walker |  | 8792 | 111 | python decl names surface in src/click/_utils.py |  |  | 0.383 |
| walker |  | 8792 | 0 | python decl at src/click/_utils.py:7 |  |  | 0.383 |
| walker |  | 8808 | 16 | python method sigs in src/click/_utils.py |  |  | 0.383 |
| walker |  | 8808 | 0 | python method at src/click/_utils.py:18 |  |  | 0.383 |
| walker |  | 8830 | 22 | python class body at src/click/_utils.py:7 |  |  | 0.383 |
| walker |  | 8846 | 16 | python method body at src/click/_utils.py:18 body 19 |  |  | 0.379 |
| ns | 8846 |  | 159 | exceptions.py: every class header | 4.4 |  | 0.379 |
| walker |  | 8911 | 65 | python decl doc at src/click/_utils.py:7 |  |  | 0.379 |
| walker |  | 8971 | 60 | headings outline in docs/shell-completion.md |  |  | 0.379 |
| walker |  | 9006 | 35 | python method body at src/click/_termui_impl.py:154 body 156 |  |  | 0.379 |
| walker |  | 9041 | 35 | python method body at src/click/testing.py:238 body 241 |  |  | 0.379 |
| walker |  | 9076 | 35 | python method body at src/click/testing.py:245 body 252 |  |  | 0.373 |
| ns | 9076 |  | 230 | ClickException + UsageError ledes + exit_code | 4.5 | 4.4 | 0.373 |
| walker |  | 9137 | 61 | headings outline in docs/why.md |  |  | 0.373 |
| ns | 9144 |  | 68 | testing.py: CliRunner, Result, helper class locations | 5.1 |  | 0.381 |
| walker |  | 9199 | 62 | headings outline in docs/standalone-apps.md |  |  | 0.381 |
| walker |  | 9241 | 42 | docs/standalone-apps.md section #1 |  |  | 0.381 |
| ns | 9277 |  | 133 | CliRunner.invoke signature | 5.2 | 5.1 | 0.377 |
| walker |  | 9415 | 174 | python decl names surface in src/click/core.py |  |  | 0.385 |
| walker |  | 9415 | 0 | python decl at src/click/core.py:96 |  |  | 0.385 |
| walker |  | 9415 | 0 | python decl at src/click/core.py:146 |  |  | 0.385 |
| walker |  | 9415 | 0 | python decl at src/click/core.py:185 |  |  | 0.385 |
| walker |  | 9415 | 0 | python decl at src/click/core.py:903 |  |  | 0.385 |
| walker |  | 9415 | 0 | python decl at src/click/core.py:1516 |  |  | 0.385 |
| walker |  | 9415 | 0 | python decl at src/click/core.py:1524 |  |  | 0.385 |
| walker |  | 9415 | 0 | python decl at src/click/core.py:1531 |  |  | 0.385 |
| walker |  | 9415 | 0 | python decl at src/click/core.py:1981 |  |  | 0.385 |
| walker |  | 9436 | 21 | python decl doc at src/click/core.py:1531 |  |  | 0.385 |
| ns | 9465 |  | 188 | Result: attribute documentation | 5.3 | 5.1 | 0.381 |
| walker |  | 9474 | 38 | python decl at src/click/core.py:100 |  |  | 0.381 |
| walker |  | 9489 | 15 | python decl doc at src/click/core.py:100 |  |  | 0.381 |
| ns | 9526 |  | 61 | tests/conftest.py: the canonical 'runner' fixture | 5.4 | 5.1 | 0.379 |
| walker |  | 9527 | 38 | python decl at src/click/core.py:119 |  |  | 0.379 |
| walker |  | 9545 | 18 | python decl body at src/click/core.py:96 body 97 |  |  | 0.379 |
| walker |  | 9578 | 33 | python decl doc at src/click/core.py:146 |  |  | 0.379 |
| walker |  | 9637 | 59 | python class body at src/click/core.py:185 |  |  | 0.379 |
| walker |  | 9687 | 50 | python decl doc at src/click/core.py:903 |  |  | 0.381 |
| walker |  | 9715 | 28 | python decl at src/click/core.py:57 |  |  | 0.381 |
| ns | 9723 |  | 197 | termui.py: every public function location | 5.5 |  | 0.377 |
| walker |  | 9769 | 54 | python decl doc at src/click/core.py:185 |  |  | 0.380 |
| ns | 9808 |  | 85 | utils.py: echo() signature + key kwargs | 5.6 |  | 0.378 |
| ns | 9844 |  | 36 | examples/ subdirectory listing | 6.1 |  | 0.389 |
| walker |  | 9905 | 136 | python class body at src/click/core.py:146 |  |  | 0.389 |
| walker |  | 9938 | 33 | python decl at src/click/core.py:76 |  |  | 0.389 |
| ns | 10053 |  | 209 | examples/naval/naval.py: canonical nested-group example | 6.2 |  | 0.382 |
| ns | 10257 |  | 204 | docs/index.rst: General Reference toctree | 6.3 |  | 0.375 |

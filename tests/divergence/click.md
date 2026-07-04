Score(3000)=0.600 I=0.839 C=0.430 ns_rows≤3K=20/52 (reached=11 partial=0 missing=9)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 50 | 50 | listing of '.' |  |  | 1.000 |
| ns | 50 |  | 50 | Top-level directory listing | 1.1 |  | 1.000 |
| walker |  | 53 | 3 | listing of 'src' |  |  | 1.000 |
| ns | 92 |  | 42 | Package name + version | 1.2 |  | 0.859 |
| walker |  | 130 | 77 | README headline in README.md |  |  | 0.889 |
| walker |  | 160 | 30 | headings outline in README.md |  |  | 0.889 |
| ns | 169 |  | 77 | README headline + tagline | 1.3 |  | 0.889 |
| walker |  | 187 | 27 | [dependencies] in pyproject.toml |  |  | 0.898 |
| ns | 209 |  | 40 | Python version floor + runtime dep | 1.4 | 1.2 | 0.859 |
| walker |  | 288 | 101 | README.md section #0 |  |  | 0.889 |
| ns | 310 |  | 101 | README differentiators (three-bullet pitch) | 1.5 | 1.3 | 0.886 |
| walker |  | 366 | 78 | [package] in pyproject.toml |  |  | 1.000 |
| ns | 410 |  | 100 | __init__.py: core-class re-exports | 1.6 |  | 0.889 |
| walker |  | 452 | 86 | listing of 'src/click' |  |  | 0.889 |
| walker |  | 463 | 11 | python imports #1 in src/click/__init__.py |  |  | 0.889 |
| walker |  | 536 | 73 | python imports in src/click/__init__.py |  |  | 0.889 |
| walker |  | 552 | 16 | python imports #6 in src/click/__init__.py |  |  | 0.890 |
| ns | 578 |  | 168 | README hello-world example | 1.7 | 1.3 | 0.743 |
| walker |  | 582 | 30 | python imports #5 in src/click/__init__.py |  |  | 0.743 |
| walker |  | 682 | 100 | python imports #2 in src/click/__init__.py |  |  | 0.836 |
| ns | 739 |  | 161 | __init__.py: decorator re-exports | 1.8 | 1.6 | 0.757 |
| walker |  | 843 | 161 | python imports #3 in src/click/__init__.py |  |  | 0.858 |
| walker |  | 861 | 18 | python decl names surface in src/click/__init__.py |  |  | 0.858 |
| walker |  | 861 | 0 | python decl at src/click/__init__.py:77 |  |  | 0.858 |
| ns | 931 |  | 192 | __init__.py: exception + formatting + globals re-exports | 1.9 | 1.8 | 0.783 |
| walker |  | 1007 | 146 | python imports #4 in src/click/__init__.py |  |  | 0.876 |
| walker |  | 1095 | 88 | python imports #9 in src/click/__init__.py |  |  | 0.881 |
| ns | 1120 |  | 189 | __init__.py: termui re-exports | 1.10 | 1.9 | 0.810 |
| walker |  | 1284 | 189 | python imports #7 in src/click/__init__.py |  |  | 0.895 |
| ns | 1302 |  | 182 | __init__.py: built-in type re-exports | 1.11 | 1.10 | 0.827 |
| ns | 1395 |  | 93 | __init__.py: utils re-exports | 1.12 | 1.11 | 0.826 |
| walker |  | 1466 | 182 | python imports #8 in src/click/__init__.py |  |  | 0.900 |
| walker |  | 1479 | 13 | listing of '.github' |  |  | 0.900 |
| walker |  | 1502 | 23 | listing of '.github/workflows' |  |  | 0.900 |
| walker |  | 1516 | 14 | python decl names surface in src/click/_textwrap.py |  |  | 0.900 |
| walker |  | 1516 | 0 | python decl at src/click/_textwrap.py:8 |  |  | 0.900 |
| walker |  | 1557 | 41 | python method sigs in src/click/_textwrap.py |  |  | 0.900 |
| walker |  | 1557 | 0 | python method at src/click/_textwrap.py:40 |  |  | 0.900 |
| walker |  | 1576 | 19 | python method at src/click/_textwrap.py:27 |  |  | 0.900 |
| ns | 1647 |  | 252 | Deprecation shim: BaseCommand → Command, MultiCommand → Group | 1.13 | 1.12 | 0.809 |
| walker |  | 1651 | 75 | README.md section #3 |  |  | 0.809 |
| walker |  | 1741 | 90 | README.md section #2 |  |  | 0.809 |
| walker |  | 1777 | 36 | listing of 'examples' |  |  | 0.810 |
| ns | 1916 |  | 269 | Deprecation shim: OptionParser, __version__ | 1.14 | 1.13 | 0.737 |
| walker |  | 1969 | 192 | listing of 'docs' |  |  | 0.737 |
| walker |  | 1996 | 27 | docs/setuptools.md section #0 |  |  | 0.737 |
| walker |  | 2011 | 15 | listing of 'docs/_static' |  |  | 0.737 |
| walker |  | 2064 | 53 | docs/license.md section #0 |  |  | 0.737 |
| walker |  | 2085 | 21 | headings outline in docs/click-concepts.md |  |  | 0.737 |
| walker |  | 2106 | 21 | headings outline in docs/unicode-support.md |  |  | 0.737 |
| ns | 2121 |  | 205 | decorators.py: location of every public decorator | 2.1 |  | 0.708 |
| walker |  | 2128 | 22 | headings outline in docs/command-line-reference.md |  |  | 0.708 |
| walker |  | 2150 | 22 | headings outline in docs/contrib.md |  |  | 0.708 |
| walker |  | 2172 | 22 | headings outline in docs/design-opinions.md |  |  | 0.708 |
| ns | 2209 |  | 88 | @command: signature + 1-liner | 2.2 | 2.1 | 0.695 |
| walker |  | 2240 | 68 | docs/design-opinions.md section #0 |  |  | 0.695 |
| walker |  | 2318 | 78 | docs/click-concepts.md section #0 |  |  | 0.695 |
| ns | 2367 |  | 158 | @option: signature + 1-liner | 2.3 | 2.1 | 0.674 |
| walker |  | 2399 | 81 | docs/command-line-reference.md section #0 |  |  | 0.674 |
| walker |  | 2421 | 22 | python decl names surface #1 in src/click/exceptions.py |  |  | 0.674 |
| walker |  | 2421 | 0 | python decl at src/click/exceptions.py:330 |  |  | 0.674 |
| walker |  | 2421 | 0 | python decl at src/click/exceptions.py:334 |  |  | 0.674 |
| walker |  | 2435 | 14 | python class body at src/click/exceptions.py:334 |  |  | 0.674 |
| walker |  | 2449 | 14 | python decl doc at src/click/exceptions.py:330 |  |  | 0.674 |
| walker |  | 2500 | 51 | python decl doc at src/click/exceptions.py:334 |  |  | 0.674 |
| ns | 2525 |  | 158 | @argument: signature + 1-liner | 2.4 | 2.1 | 0.655 |
| walker |  | 2627 | 127 | listing of 'tests' |  |  | 0.655 |
| walker |  | 2657 | 30 | headings outline in docs/entry-points.md |  |  | 0.655 |
| walker |  | 2669 | 12 | listing of 'examples/complex' |  |  | 0.655 |
| walker |  | 2682 | 13 | listing of 'examples/complex/complex' |  |  | 0.655 |
| ns | 2688 |  | 163 | @group: signature + 1-liner | 2.5 | 2.1 | 0.633 |
| walker |  | 2698 | 16 | listing of 'examples/complex/complex/commands' |  |  | 0.633 |
| walker |  | 2730 | 32 | headings outline in docs/extending-click.md |  |  | 0.633 |
| walker |  | 2763 | 33 | headings outline in docs/wincmd.md |  |  | 0.633 |
| walker |  | 2850 | 87 | docs/wincmd.md section #0 |  |  | 0.633 |
| walker |  | 2882 | 32 | python imports in src/click/_utils.py |  |  | 0.633 |
| walker |  | 2893 | 11 | python decl names surface #3 in src/click/types.py |  |  | 0.633 |
| ns | 2923 |  | 235 | @pass_context / @pass_obj signatures + docstrings | 2.6 | 2.1 | 0.600 |
| walker |  | 3113 | 220 | README.md section #1 |  |  | 0.663 |
| walker |  | 3147 | 34 | python imports in src/click/globals.py |  |  | 0.663 |
| walker |  | 3183 | 36 | headings outline in docs/faqs.md |  |  | 0.663 |
| walker |  | 3219 | 36 | docs/faqs.md section #0 |  |  | 0.663 |
| ns | 3240 |  | 317 | @make_pass_decorator: signature + docstring + how-it-works | 2.7 | 2.1 | 0.626 |
| walker |  | 3256 | 37 | headings outline in docs/parameters.md |  |  | 0.626 |
| walker |  | 3303 | 47 | docs/parameters.md section #0 |  |  | 0.626 |
| walker |  | 3398 | 95 | python decl names surface in src/click/testing.py |  |  | 0.627 |
| walker |  | 3398 | 0 | python decl at src/click/testing.py:26 |  |  | 0.627 |
| walker |  | 3398 | 0 | python decl at src/click/testing.py:70 |  |  | 0.627 |
| walker |  | 3398 | 0 | python decl at src/click/testing.py:89 |  |  | 0.627 |
| walker |  | 3398 | 0 | python decl at src/click/testing.py:103 |  |  | 0.627 |
| walker |  | 3398 | 0 | python decl at src/click/testing.py:183 |  |  | 0.627 |
| walker |  | 3398 | 0 | python decl at src/click/testing.py:261 |  |  | 0.627 |
| walker |  | 3441 | 43 | python decl at src/click/testing.py:163 |  |  | 0.627 |
| walker |  | 3463 | 22 | python decl at src/click/testing.py:60 |  |  | 0.627 |
| ns | 3498 |  | 258 | @confirmation_option / @password_option full bodies | 2.8 | 2.1 | 0.596 |
| walker |  | 3508 | 45 | python decl doc at src/click/testing.py:70 |  |  | 0.596 |
| walker |  | 3569 | 61 | python decl doc at src/click/testing.py:89 |  |  | 0.596 |
| walker |  | 3607 | 38 | headings outline in docs/virtualenv.md |  |  | 0.596 |
| walker |  | 3607 | 0 | docs/virtualenv.md section #0 |  |  | 0.596 |
| walker |  | 3731 | 124 | docs/extending-click.md section #0 |  |  | 0.596 |
| ns | 3738 |  | 240 | @version_option signature + key kwargs | 2.9 | 2.1 | 0.575 |
| walker |  | 3773 | 42 | headings outline in docs/option-decorators.md |  |  | 0.575 |
| walker |  | 3804 | 31 | docs/option-decorators.md section #3 |  |  | 0.575 |
| walker |  | 3894 | 90 | docs/option-decorators.md section #0 |  |  | 0.575 |
| walker |  | 3937 | 43 | headings outline in docs/handling-files.md |  |  | 0.575 |
| walker |  | 4014 | 77 | docs/handling-files.md section #0 |  |  | 0.575 |
| ns | 4085 |  | 347 | quickstart.md: 'Basic Concepts - Creating a Command' section | 2.10 |  | 0.542 |
| walker |  | 4097 | 83 | python decl names surface in src/click/globals.py |  |  | 0.542 |
| walker |  | 4097 | 0 | python decl at src/click/globals.py:20 |  |  | 0.542 |
| walker |  | 4097 | 0 | python decl at src/click/globals.py:44 |  |  | 0.542 |
| walker |  | 4097 | 0 | python decl at src/click/globals.py:49 |  |  | 0.542 |
| walker |  | 4097 | 0 | python decl at src/click/globals.py:54 |  |  | 0.542 |
| walker |  | 4106 | 9 | python decl at src/click/globals.py:12 |  |  | 0.542 |
| walker |  | 4115 | 9 | python decl at src/click/globals.py:16 |  |  | 0.542 |
| walker |  | 4125 | 10 | python decl body at src/click/globals.py:49 body 51 |  |  | 0.542 |
| walker |  | 4138 | 13 | python decl doc at src/click/globals.py:49 |  |  | 0.542 |
| walker |  | 4152 | 14 | python decl doc at src/click/globals.py:44 |  |  | 0.542 |
| walker |  | 4171 | 19 | python decl body at src/click/globals.py:44 body 46 |  |  | 0.542 |
| walker |  | 4191 | 20 | python decl body at src/click/globals.py:16 body 17 |  |  | 0.542 |
| ns | 4205 |  | 120 | core.py: every class header (top-level inventory) | 3.1 |  | 0.533 |
| walker |  | 4215 | 24 | python decl body at src/click/globals.py:12 body 13 |  |  | 0.533 |
| walker |  | 4268 | 53 | python decl doc at src/click/globals.py:54 |  |  | 0.533 |
| walker |  | 4276 | 8 | python decl body at src/click/globals.py:20 body 41 |  |  | 0.533 |
| ns | 4349 |  | 144 | Context: one-paragraph lede | 3.2 | 3.1 | 0.522 |
| walker |  | 4517 | 241 | python method sigs in src/click/testing.py |  |  | 0.522 |
| walker |  | 4517 | 0 | python method at src/click/testing.py:27 |  |  | 0.522 |
| walker |  | 4517 | 0 | python method at src/click/testing.py:32 |  |  | 0.522 |
| walker |  | 4517 | 0 | python method at src/click/testing.py:35 |  |  | 0.522 |
| walker |  | 4517 | 0 | python method at src/click/testing.py:41 |  |  | 0.522 |
| walker |  | 4517 | 0 | python method at src/click/testing.py:44 |  |  | 0.522 |
| walker |  | 4517 | 0 | python method at src/click/testing.py:47 |  |  | 0.522 |
| walker |  | 4517 | 0 | python method at src/click/testing.py:50 |  |  | 0.522 |
| walker |  | 4517 | 0 | python method at src/click/testing.py:53 |  |  | 0.522 |
| walker |  | 4517 | 0 | python method at src/click/testing.py:56 |  |  | 0.522 |
| walker |  | 4517 | 0 | python method at src/click/testing.py:76 |  |  | 0.522 |
| walker |  | 4517 | 0 | python method at src/click/testing.py:80 |  |  | 0.522 |
| walker |  | 4517 | 0 | python method at src/click/testing.py:84 |  |  | 0.522 |
| walker |  | 4526 | 9 | python method body at src/click/testing.py:56 body 57 |  |  | 0.522 |
| walker |  | 4537 | 11 | python method body at src/click/testing.py:32 body 33 |  |  | 0.522 |
| walker |  | 4550 | 13 | python method body at src/click/testing.py:41 body 42 |  |  | 0.522 |
| walker |  | 4563 | 13 | python method body at src/click/testing.py:47 body 48 |  |  | 0.522 |
| walker |  | 4580 | 17 | python method body at src/click/testing.py:53 body 54 |  |  | 0.522 |
| walker |  | 4597 | 17 | python method body at src/click/testing.py:80 body 81 |  |  | 0.522 |
| walker |  | 4615 | 18 | python method body at src/click/testing.py:50 body 51 |  |  | 0.522 |
| ns | 4618 |  | 269 | Context.__init__ signature | 3.3 | 3.2 | 0.507 |
| walker |  | 4634 | 19 | python method body at src/click/testing.py:76 body 77 |  |  | 0.507 |
| walker |  | 4654 | 20 | python method body at src/click/testing.py:44 body 45 |  |  | 0.507 |
| walker |  | 4674 | 20 | python method body at src/click/testing.py:84 body 85 |  |  | 0.507 |
| walker |  | 4830 | 156 | python decl names surface in src/click/_termui_impl.py |  |  | 0.507 |
| walker |  | 4830 | 0 | python decl at src/click/_termui_impl.py:43 |  |  | 0.507 |
| walker |  | 4830 | 0 | python decl at src/click/_termui_impl.py:375 |  |  | 0.507 |
| walker |  | 4830 | 0 | python decl at src/click/_termui_impl.py:608 |  |  | 0.507 |
| walker |  | 4830 | 0 | python decl at src/click/_termui_impl.py:724 |  |  | 0.507 |
| walker |  | 4830 | 0 | python decl at src/click/_termui_impl.py:794 |  |  | 0.507 |
| walker |  | 4857 | 27 | python decl at src/click/_termui_impl.py:417 |  |  | 0.507 |
| walker |  | 4900 | 43 | python class body at src/click/_termui_impl.py:608 |  |  | 0.507 |
| walker |  | 4934 | 34 | python decl at src/click/_termui_impl.py:386 |  |  | 0.507 |
| walker |  | 4951 | 17 | python decl doc at src/click/_termui_impl.py:386 |  |  | 0.507 |
| ns | 5031 |  | 413 | Context: location of every method | 3.4 | 3.2 | 0.490 |
| walker |  | 5114 | 163 | python method sigs in src/click/_termui_impl.py |  |  | 0.490 |
| walker |  | 5114 | 0 | python method at src/click/_termui_impl.py:115 |  |  | 0.490 |
| walker |  | 5114 | 0 | python method at src/click/_termui_impl.py:128 |  |  | 0.490 |
| walker |  | 5114 | 0 | python method at src/click/_termui_impl.py:142 |  |  | 0.490 |
| walker |  | 5114 | 0 | python method at src/click/_termui_impl.py:166 |  |  | 0.490 |
| walker |  | 5114 | 0 | python method at src/click/_termui_impl.py:187 |  |  | 0.490 |
| walker |  | 5114 | 0 | python method at src/click/_termui_impl.py:193 |  |  | 0.490 |
| walker |  | 5124 | 10 | python method at src/click/_termui_impl.py:148 |  |  | 0.490 |
| walker |  | 5134 | 10 | python method at src/click/_termui_impl.py:160 |  |  | 0.490 |
| walker |  | 5146 | 12 | python method at src/click/_termui_impl.py:154 |  |  | 0.490 |
| ns | 5156 |  | 125 | Command: one-paragraph lede + headline kwargs | 3.5 | 3.1 | 0.484 |
| walker |  | 5202 | 56 | python method at src/click/_termui_impl.py:120 |  |  | 0.484 |
| walker |  | 5211 | 9 | python method body at src/click/_termui_impl.py:120 body 126 |  |  | 0.484 |
| walker |  | 5232 | 21 | python method body at src/click/_termui_impl.py:193 body 194 |  |  | 0.484 |
| walker |  | 5276 | 44 | python decl at src/click/_termui_impl.py:597 |  |  | 0.484 |
| walker |  | 5295 | 19 | python decl doc at src/click/_termui_impl.py:597 |  |  | 0.484 |
| ns | 5357 |  | 201 | Command.__init__ signature | 3.6 | 3.5 | 0.474 |
| walker |  | 5377 | 82 | python decl doc at src/click/_termui_impl.py:417 |  |  | 0.474 |
| walker |  | 5403 | 26 | python method body at src/click/_termui_impl.py:115 body 116 |  |  | 0.474 |
| walker |  | 5451 | 48 | python decl at src/click/_termui_impl.py:442 |  |  | 0.474 |
| walker |  | 5500 | 49 | python decl at src/click/_termui_impl.py:546 |  |  | 0.474 |
| walker |  | 5644 | 144 | python method sigs #1 in src/click/testing.py |  |  | 0.474 |
| walker |  | 5644 | 0 | python method at src/click/testing.py:97 |  |  | 0.474 |
| walker |  | 5644 | 0 | python method at src/click/testing.py:131 |  |  | 0.474 |
| walker |  | 5644 | 0 | python method at src/click/testing.py:139 |  |  | 0.474 |
| walker |  | 5644 | 0 | python method at src/click/testing.py:256 |  |  | 0.474 |
| walker |  | 5654 | 10 | python method at src/click/testing.py:154 |  |  | 0.474 |
| walker |  | 5664 | 10 | python method at src/click/testing.py:158 |  |  | 0.474 |
| walker |  | 5674 | 10 | python method at src/click/testing.py:226 |  |  | 0.474 |
| walker |  | 5684 | 10 | python method at src/click/testing.py:238 |  |  | 0.474 |
| walker |  | 5694 | 10 | python method at src/click/testing.py:245 |  |  | 0.474 |
| walker |  | 5707 | 13 | python method doc at src/click/testing.py:238 |  |  | 0.474 |
| walker |  | 5717 | 10 | python method body at src/click/testing.py:154 body 156 |  |  | 0.474 |
| walker |  | 5727 | 10 | python method body at src/click/testing.py:158 body 160 |  |  | 0.474 |
| ns | 5745 |  | 388 | Command: location of every method | 3.7 | 3.5 | 0.463 |
| walker |  | 5800 | 73 | python method at src/click/testing.py:117 |  |  | 0.463 |
| walker |  | 5875 | 75 | python method at src/click/testing.py:283 |  |  | 0.463 |
| walker |  | 5932 | 57 | python method doc at src/click/testing.py:245 |  |  | 0.463 |
| walker |  | 6069 | 137 | python decl names surface in src/click/parser.py |  |  | 0.463 |
| walker |  | 6069 | 0 | python decl at src/click/parser.py:111 |  |  | 0.463 |
| walker |  | 6069 | 0 | python decl at src/click/parser.py:120 |  |  | 0.463 |
| walker |  | 6069 | 0 | python decl at src/click/parser.py:127 |  |  | 0.463 |
| walker |  | 6069 | 0 | python decl at src/click/parser.py:185 |  |  | 0.463 |
| walker |  | 6069 | 0 | python decl at src/click/parser.py:216 |  |  | 0.463 |
| walker |  | 6069 | 0 | python decl at src/click/parser.py:224 |  |  | 0.463 |
| walker |  | 6069 | 0 | python decl at src/click/parser.py:503 |  |  | 0.463 |
| ns | 6201 |  | 456 | Group: lede + __init__ signature | 3.8 | 3.1 | 0.446 |
| walker |  | 6379 | 310 | python method sigs in src/click/parser.py |  |  | 0.446 |
| walker |  | 6379 | 0 | python method at src/click/parser.py:169 |  |  | 0.446 |
| walker |  | 6379 | 0 | python method at src/click/parser.py:186 |  |  | 0.446 |
| walker |  | 6379 | 0 | python method at src/click/parser.py:217 |  |  | 0.446 |
| walker |  | 6379 | 0 | python method at src/click/parser.py:290 |  |  | 0.446 |
| walker |  | 6379 | 0 | python method at src/click/parser.py:316 |  |  | 0.446 |
| walker |  | 6379 | 0 | python method at src/click/parser.py:327 |  |  | 0.446 |
| walker |  | 6379 | 0 | python method at src/click/parser.py:390 |  |  | 0.446 |
| walker |  | 6379 | 0 | python method at src/click/parser.py:470 |  |  | 0.446 |
| walker |  | 6390 | 11 | python method at src/click/parser.py:165 |  |  | 0.446 |
| ns | 6415 |  | 214 | Group: method-locations + decorator-style command/group registrars | 3.9 | 3.8 | 0.441 |
| walker |  | 6423 | 33 | python method at src/click/parser.py:298 |  |  | 0.441 |
| walker |  | 6459 | 36 | python method at src/click/parser.py:241 |  |  | 0.441 |
| walker |  | 6474 | 15 | python method body at src/click/parser.py:165 body 167 |  |  | 0.441 |
| walker |  | 6522 | 48 | python method at src/click/parser.py:191 |  |  | 0.441 |
| walker |  | 6543 | 21 | python method body at src/click/parser.py:290 body 296 |  |  | 0.441 |
| ns | 6567 |  | 152 | Parameter (ABC): lede + key kwargs paragraph | 3.10 | 3.1 | 0.435 |
| walker |  | 6590 | 47 | python decl at src/click/parser.py:51 |  |  | 0.435 |
| walker |  | 6621 | 31 | python method at src/click/parser.py:363 |  |  | 0.435 |
| walker |  | 6704 | 83 | python method at src/click/parser.py:128 |  |  | 0.435 |
| walker |  | 6733 | 29 | python method body at src/click/parser.py:186 body 187 |  |  | 0.435 |
| walker |  | 6819 | 86 | python method at src/click/parser.py:265 |  |  | 0.435 |
| ns | 6856 |  | 289 | Parameter.__init__ signature | 3.11 | 3.10 | 0.426 |
| walker |  | 6866 | 47 | python imports in src/click/_textwrap.py |  |  | 0.426 |
| walker |  | 6955 | 89 | python method at src/click/_termui_impl.py:134 |  |  | 0.426 |
| walker |  | 6965 | 10 | python method body at src/click/_termui_impl.py:134 body 140 |  |  | 0.426 |
| walker |  | 7021 | 56 | listing of 'tests/typing' |  |  | 0.426 |
| walker |  | 7072 | 51 | headings outline in docs/arguments.md |  |  | 0.426 |
| ns | 7202 |  | 346 | Option: lede + __init__ signature | 3.12 | 3.10 | 0.414 |
| walker |  | 7229 | 157 | docs/arguments.md section #0 |  |  | 0.414 |
| walker |  | 7289 | 60 | python method doc at src/click/parser.py:290 |  |  | 0.414 |
| walker |  | 7320 | 31 | python method body at src/click/testing.py:27 body 28 |  |  | 0.414 |
| ns | 7355 |  | 153 | Argument: lede + __init__ signature + required-by-default logic | 3.13 | 3.10 | 0.408 |
| walker |  | 7423 | 103 | python decl names surface in src/click/formatting.py |  |  | 0.408 |
| walker |  | 7423 | 0 | python decl at src/click/formatting.py:14 |  |  | 0.408 |
| walker |  | 7423 | 0 | python decl at src/click/formatting.py:104 |  |  | 0.408 |
| walker |  | 7423 | 0 | python decl at src/click/formatting.py:283 |  |  | 0.408 |
| walker |  | 7458 | 35 | python decl at src/click/formatting.py:24 |  |  | 0.408 |
| walker |  | 7521 | 63 | python decl at src/click/formatting.py:31 |  |  | 0.408 |
| walker |  | 7550 | 29 | python decl body at src/click/formatting.py:24 body 27 |  |  | 0.408 |
| ns | 7722 |  | 367 | Parameter/Option/Argument: method locations | 3.14 | 3.11 | 0.401 |
| walker |  | 7742 | 192 | python method sigs in src/click/formatting.py |  |  | 0.401 |
| walker |  | 7742 | 0 | python method at src/click/formatting.py:135 |  |  | 0.401 |
| walker |  | 7742 | 0 | python method at src/click/formatting.py:139 |  |  | 0.401 |
| walker |  | 7742 | 0 | python method at src/click/formatting.py:143 |  |  | 0.401 |
| walker |  | 7742 | 0 | python method at src/click/formatting.py:147 |  |  | 0.401 |
| walker |  | 7742 | 0 | python method at src/click/formatting.py:185 |  |  | 0.401 |
| walker |  | 7742 | 0 | python method at src/click/formatting.py:189 |  |  | 0.401 |
| walker |  | 7742 | 0 | python method at src/click/formatting.py:194 |  |  | 0.401 |
| walker |  | 7742 | 0 | python method at src/click/formatting.py:278 |  |  | 0.401 |
| walker |  | 7756 | 14 | python method at src/click/formatting.py:269 |  |  | 0.401 |
| walker |  | 7774 | 18 | python method at src/click/formatting.py:254 |  |  | 0.401 |
| walker |  | 7783 | 9 | python method doc at src/click/formatting.py:139 |  |  | 0.401 |
| walker |  | 7792 | 9 | python method doc at src/click/formatting.py:143 |  |  | 0.401 |
| walker |  | 7801 | 9 | python method doc at src/click/formatting.py:278 |  |  | 0.401 |
| walker |  | 7812 | 11 | python method doc at src/click/formatting.py:185 |  |  | 0.401 |
| walker |  | 7823 | 11 | python method doc at src/click/formatting.py:189 |  |  | 0.401 |
| walker |  | 7836 | 13 | python method doc at src/click/formatting.py:135 |  |  | 0.401 |
| walker |  | 7850 | 14 | python method doc at src/click/formatting.py:269 |  |  | 0.401 |
| walker |  | 7860 | 10 | python method body at src/click/formatting.py:135 body 137 |  |  | 0.401 |
| walker |  | 7871 | 11 | python method body at src/click/formatting.py:278 body 280 |  |  | 0.401 |
| walker |  | 7885 | 14 | python method body at src/click/formatting.py:139 body 141 |  |  | 0.401 |
| walker |  | 7899 | 14 | python method body at src/click/formatting.py:143 body 145 |  |  | 0.401 |
| walker |  | 7953 | 54 | python method at src/click/formatting.py:116 |  |  | 0.401 |
| walker |  | 7972 | 19 | python method body at src/click/formatting.py:189 body 191 |  |  | 0.401 |
| ns | 8019 |  | 297 | types.py: every ParamType subclass header | 4.1 |  | 0.394 |
| walker |  | 8028 | 56 | python method at src/click/formatting.py:210 |  |  | 0.394 |
| walker |  | 8062 | 34 | python method doc at src/click/formatting.py:194 |  |  | 0.394 |
| walker |  | 8084 | 22 | python method body at src/click/formatting.py:185 body 187 |  |  | 0.394 |
| walker |  | 8165 | 81 | python decl doc at src/click/formatting.py:283 |  |  | 0.394 |
| walker |  | 8221 | 56 | python method doc at src/click/formatting.py:254 |  |  | 0.394 |
| ns | 8250 |  | 231 | ParamType: the subclassing recipe | 4.2 | 4.1 | 0.388 |
| walker |  | 8276 | 55 | headings outline in docs/support-multiple-versions.md |  |  | 0.388 |
| walker |  | 8406 | 130 | docs/support-multiple-versions.md section #0 |  |  | 0.388 |
| walker |  | 8413 | 7 | python method body at src/click/_termui_impl.py:166 body 185 |  |  | 0.388 |
| ns | 8431 |  | 181 | ParamType.convert + ParamType.shell_complete signatures | 4.3 | 4.2 | 0.383 |
| walker |  | 8548 | 135 | python decl doc at src/click/formatting.py:104 |  |  | 0.383 |
| ns | 8564 |  | 133 | exceptions.py: every class header | 4.4 |  | 0.379 |
| walker |  | 8653 | 105 | python decl names surface in src/click/_utils.py |  |  | 0.379 |
| walker |  | 8653 | 0 | python decl at src/click/_utils.py:7 |  |  | 0.379 |
| walker |  | 8669 | 16 | python method sigs in src/click/_utils.py |  |  | 0.379 |
| walker |  | 8669 | 0 | python method at src/click/_utils.py:18 |  |  | 0.379 |
| walker |  | 8691 | 22 | python class body at src/click/_utils.py:7 |  |  | 0.379 |
| walker |  | 8707 | 16 | python method body at src/click/_utils.py:18 body 19 |  |  | 0.379 |
| walker |  | 8772 | 65 | python decl doc at src/click/_utils.py:7 |  |  | 0.379 |
| ns | 8790 |  | 226 | ClickException + UsageError ledes + exit_code | 4.5 | 4.4 | 0.373 |
| walker |  | 8830 | 58 | headings outline in docs/shell-completion.md |  |  | 0.373 |
| ns | 8844 |  | 54 | testing.py: CliRunner, Result, helper class locations | 5.1 |  | 0.381 |
| walker |  | 8898 | 68 | python method doc at src/click/testing.py:131 |  |  | 0.381 |
| walker |  | 8959 | 61 | headings outline in docs/why.md |  |  | 0.381 |
| ns | 8975 |  | 131 | CliRunner.invoke signature | 5.2 | 5.1 | 0.377 |
| walker |  | 9006 | 47 | python decl names surface #1 in src/click/utils.py |  |  | 0.377 |
| walker |  | 9006 | 0 | python decl at src/click/utils.py:502 |  |  | 0.377 |
| walker |  | 9035 | 29 | python decl at src/click/utils.py:527 |  |  | 0.377 |
| ns | 9163 |  | 188 | Result: attribute documentation | 5.3 | 5.1 | 0.373 |
| walker |  | 9164 | 129 | python decl doc at src/click/utils.py:502 |  |  | 0.373 |
| walker |  | 9224 | 60 | python decl at src/click/utils.py:582 |  |  | 0.371 |
| ns | 9224 |  | 61 | tests/conftest.py: the canonical 'runner' fixture | 5.4 | 5.1 | 0.371 |
| walker |  | 9286 | 62 | headings outline in docs/standalone-apps.md |  |  | 0.371 |
| walker |  | 9328 | 42 | docs/standalone-apps.md section #1 |  |  | 0.371 |
| ns | 9391 |  | 167 | termui.py: every public function location | 5.5 |  | 0.367 |
| ns | 9472 |  | 81 | utils.py: echo() signature + key kwargs | 5.6 |  | 0.365 |
| walker |  | 9500 | 172 | python decl names surface in src/click/core.py |  |  | 0.372 |
| walker |  | 9500 | 0 | python decl at src/click/core.py:96 |  |  | 0.372 |
| walker |  | 9500 | 0 | python decl at src/click/core.py:146 |  |  | 0.372 |
| walker |  | 9500 | 0 | python decl at src/click/core.py:185 |  |  | 0.372 |
| walker |  | 9500 | 0 | python decl at src/click/core.py:903 |  |  | 0.372 |
| walker |  | 9500 | 0 | python decl at src/click/core.py:1516 |  |  | 0.372 |
| walker |  | 9500 | 0 | python decl at src/click/core.py:1524 |  |  | 0.372 |
| walker |  | 9500 | 0 | python decl at src/click/core.py:1531 |  |  | 0.372 |
| walker |  | 9500 | 0 | python decl at src/click/core.py:1981 |  |  | 0.372 |
| ns | 9508 |  | 36 | examples/ subdirectory listing | 6.1 |  | 0.384 |
| walker |  | 9519 | 19 | python decl doc at src/click/core.py:1531 |  |  | 0.384 |
| walker |  | 9555 | 36 | python decl at src/click/core.py:100 |  |  | 0.384 |
| walker |  | 9570 | 15 | python decl doc at src/click/core.py:100 |  |  | 0.384 |
| walker |  | 9606 | 36 | python decl at src/click/core.py:119 |  |  | 0.384 |
| walker |  | 9624 | 18 | python decl body at src/click/core.py:96 body 97 |  |  | 0.384 |
| walker |  | 9655 | 31 | python decl doc at src/click/core.py:146 |  |  | 0.384 |
| walker |  | 9712 | 57 | python class body at src/click/core.py:185 |  |  | 0.384 |
| ns | 9715 |  | 207 | examples/naval/naval.py: canonical nested-group example | 6.2 |  | 0.377 |
| walker |  | 9738 | 26 | python decl at src/click/core.py:57 |  |  | 0.377 |
| walker |  | 9786 | 48 | python decl doc at src/click/core.py:903 |  |  | 0.380 |
| walker |  | 9838 | 52 | python decl doc at src/click/core.py:185 |  |  | 0.382 |
| walker |  | 9869 | 31 | python decl at src/click/core.py:76 |  |  | 0.382 |
| ns | 9915 |  | 200 | docs/index.rst: General Reference toctree | 6.3 |  | 0.375 |

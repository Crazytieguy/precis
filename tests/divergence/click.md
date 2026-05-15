Score(3000)=0.282 I=0.645 C=0.123 ns_rows≤3K=20/52 (reached=2 partial=2 missing=16)

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
| walker |  | 379 | 18 | python decl names surface in src/click/__init__.py |  |  | 0.844 |
| walker |  | 379 | 0 | python decl at src/click/__init__.py:77 |  |  | 0.844 |
| walker |  | 392 | 13 | listing of '.github' |  |  | 0.844 |
| ns | 410 |  | 100 | __init__.py: core-class re-exports | 1.6 |  | 0.750 |
| walker |  | 415 | 23 | listing of '.github/workflows' |  |  | 0.750 |
| walker |  | 429 | 14 | python decl names surface in src/click/_textwrap.py |  |  | 0.750 |
| walker |  | 429 | 0 | python decl at src/click/_textwrap.py:8 |  |  | 0.750 |
| walker |  | 470 | 41 | python method sigs in src/click/_textwrap.py |  |  | 0.750 |
| walker |  | 470 | 0 | python method at src/click/_textwrap.py:40 |  |  | 0.750 |
| walker |  | 489 | 19 | python method at src/click/_textwrap.py:27 |  |  | 0.750 |
| walker |  | 556 | 67 | README.md section #3 |  |  | 0.750 |
| ns | 578 |  | 168 | README hello-world example | 1.7 | 1.3 | 0.627 |
| walker |  | 638 | 82 | README.md section #2 |  |  | 0.627 |
| walker |  | 674 | 36 | listing of 'examples' |  |  | 0.628 |
| ns | 739 |  | 161 | __init__.py: decorator re-exports | 1.8 | 1.6 | 0.569 |
| walker |  | 866 | 192 | listing of 'docs' |  |  | 0.569 |
| walker |  | 887 | 21 | headings outline in docs/click-concepts.md |  |  | 0.569 |
| walker |  | 908 | 21 | headings outline in docs/unicode-support.md |  |  | 0.569 |
| walker |  | 930 | 22 | headings outline in docs/command-line-reference.md |  |  | 0.569 |
| ns | 931 |  | 192 | __init__.py: exception + formatting + globals re-exports | 1.9 | 1.8 | 0.516 |
| walker |  | 952 | 22 | headings outline in docs/contrib.md |  |  | 0.516 |
| walker |  | 974 | 22 | headings outline in docs/design-opinions.md |  |  | 0.516 |
| walker |  | 1004 | 30 | headings outline in docs/entry-points.md |  |  | 0.516 |
| walker |  | 1036 | 32 | headings outline in docs/extending-click.md |  |  | 0.516 |
| walker |  | 1069 | 33 | headings outline in docs/faqs.md |  |  | 0.516 |
| walker |  | 1102 | 33 | headings outline in docs/wincmd.md |  |  | 0.516 |
| ns | 1120 |  | 189 | __init__.py: termui re-exports | 1.10 | 1.9 | 0.474 |
| walker |  | 1137 | 35 | headings outline in docs/virtualenv.md |  |  | 0.474 |
| walker |  | 1137 | 0 | docs/virtualenv.md section #0 |  |  | 0.474 |
| walker |  | 1174 | 37 | headings outline in docs/parameters.md |  |  | 0.474 |
| walker |  | 1216 | 42 | headings outline in docs/option-decorators.md |  |  | 0.474 |
| walker |  | 1259 | 43 | headings outline in docs/handling-files.md |  |  | 0.474 |
| ns | 1302 |  | 182 | __init__.py: built-in type re-exports | 1.11 | 1.10 | 0.438 |
| walker |  | 1310 | 51 | headings outline in docs/arguments.md |  |  | 0.438 |
| walker |  | 1362 | 52 | headings outline in docs/support-multiple-versions.md |  |  | 0.438 |
| ns | 1395 |  | 93 | __init__.py: utils re-exports | 1.12 | 1.11 | 0.423 |
| walker |  | 1420 | 58 | headings outline in docs/shell-completion.md |  |  | 0.423 |
| walker |  | 1481 | 61 | headings outline in docs/why.md |  |  | 0.423 |
| walker |  | 1543 | 62 | headings outline in docs/standalone-apps.md |  |  | 0.423 |
| walker |  | 1607 | 64 | headings outline in docs/exceptions.md |  |  | 0.423 |
| ns | 1647 |  | 252 | Deprecation shim: BaseCommand → Command, MultiCommand → Group | 1.13 | 1.12 | 0.380 |
| walker |  | 1676 | 69 | headings outline in docs/parameter-types.md |  |  | 0.380 |
| walker |  | 1745 | 69 | headings outline in docs/prompts.md |  |  | 0.380 |
| walker |  | 1755 | 10 | docs/parameter-types.md section #1 |  |  | 0.380 |
| walker |  | 1833 | 78 | headings outline in docs/testing.md |  |  | 0.380 |
| walker |  | 1855 | 22 | docs/setuptools.md section #0 |  |  | 0.380 |
| ns | 1916 |  | 269 | Deprecation shim: OptionParser, __version__ | 1.14 | 1.13 | 0.346 |
| walker |  | 1947 | 92 | headings outline in docs/advanced.md |  |  | 0.346 |
| walker |  | 2044 | 97 | headings outline in docs/quickstart.md |  |  | 0.346 |
| walker |  | 2061 | 17 | docs/quickstart.md section #0 |  |  | 0.346 |
| walker |  | 2076 | 15 | listing of 'docs/_static' |  |  | 0.346 |
| ns | 2121 |  | 205 | decorators.py: location of every public decorator | 2.1 |  | 0.332 |
| walker |  | 2186 | 110 | headings outline in docs/api.md |  |  | 0.332 |
| ns | 2209 |  | 88 | @command: signature + 1-liner | 2.2 | 2.1 | 0.326 |
| walker |  | 2297 | 111 | headings outline in docs/commands.md |  |  | 0.326 |
| walker |  | 2330 | 33 | docs/faqs.md section #0 |  |  | 0.326 |
| ns | 2367 |  | 158 | @option: signature + 1-liner | 2.3 | 2.1 | 0.316 |
| walker |  | 2471 | 141 | headings outline in docs/documentation.md |  |  | 0.316 |
| walker |  | 2510 | 39 | docs/parameters.md section #0 |  |  | 0.316 |
| ns | 2525 |  | 158 | @argument: signature + 1-liner | 2.4 | 2.1 | 0.307 |
| walker |  | 2662 | 152 | headings outline in docs/utils.md |  |  | 0.307 |
| ns | 2688 |  | 163 | @group: signature + 1-liner | 2.5 | 2.1 | 0.297 |
| walker |  | 2826 | 164 | headings outline in docs/complex.md |  |  | 0.297 |
| ns | 2923 |  | 235 | @pass_context / @pass_obj signatures + docstrings | 2.6 | 2.1 | 0.282 |
| walker |  | 2994 | 168 | headings outline in docs/options.md |  |  | 0.282 |
| walker |  | 3042 | 48 | docs/license.md section #0 |  |  | 0.282 |
| walker |  | 3236 | 194 | headings outline in docs/commands-and-groups.md |  |  | 0.282 |
| ns | 3240 |  | 317 | @make_pass_decorator: signature + docstring + how-it-works | 2.7 | 2.1 | 0.266 |
| walker |  | 3252 | 16 | docs/commands-and-groups.md section #13 |  |  | 0.266 |
| walker |  | 3280 | 28 | python imports in docs/conf.py |  |  | 0.266 |
| walker |  | 3337 | 57 | docs/documentation.md section #0 |  |  | 0.266 |
| walker |  | 3394 | 57 | docs/utils.md section #0 |  |  | 0.266 |
| walker |  | 3454 | 60 | docs/design-opinions.md section #0 |  |  | 0.266 |
| walker |  | 3462 | 8 | docs/parameter-types.md section #8 |  |  | 0.266 |
| ns | 3498 |  | 258 | @confirmation_option / @password_option full bodies | 2.8 | 2.1 | 0.253 |
| walker |  | 3526 | 64 | docs/handling-files.md section #0 |  |  | 0.253 |
| walker |  | 3591 | 65 | docs/click-concepts.md section #0 |  |  | 0.253 |
| walker |  | 3657 | 66 | docs/advanced.md section #0 |  |  | 0.253 |
| walker |  | 3725 | 68 | docs/command-line-reference.md section #0 |  |  | 0.253 |
| ns | 3738 |  | 240 | @version_option signature + key kwargs | 2.9 | 2.1 | 0.244 |
| walker |  | 3753 | 28 | docs/complex.md section #1 |  |  | 0.244 |
| walker |  | 3781 | 28 | docs/option-decorators.md section #3 |  |  | 0.244 |
| walker |  | 3855 | 74 | docs/wincmd.md section #0 |  |  | 0.244 |
| walker |  | 3931 | 76 | docs/api.md section #0 |  |  | 0.244 |
| walker |  | 4008 | 77 | docs/option-decorators.md section #0 |  |  | 0.244 |
| ns | 4085 |  | 347 | quickstart.md: 'Basic Concepts - Creating a Command' section | 2.10 |  | 0.231 |
| ns | 4205 |  | 120 | core.py: every class header (top-level inventory) | 3.1 |  | 0.226 |
| ns | 4349 |  | 144 | Context: one-paragraph lede | 3.2 | 3.1 | 0.222 |
| ns | 4618 |  | 269 | Context.__init__ signature | 3.3 | 3.2 | 0.216 |
| walker |  | 5004 | 996 | python imports in src/click/__init__.py |  |  | 0.424 |
| ns | 5031 |  | 413 | Context: location of every method | 3.4 | 3.2 | 0.410 |
| walker |  | 5092 | 88 | docs/commands.md section #0 |  |  | 0.410 |
| walker |  | 5114 | 22 | python decl names surface #1 in src/click/exceptions.py |  |  | 0.410 |
| walker |  | 5114 | 0 | python decl at src/click/exceptions.py:330 |  |  | 0.410 |
| walker |  | 5114 | 0 | python decl at src/click/exceptions.py:334 |  |  | 0.410 |
| walker |  | 5128 | 14 | python class body at src/click/exceptions.py:334 |  |  | 0.410 |
| walker |  | 5142 | 14 | python decl doc at src/click/exceptions.py:330 |  |  | 0.410 |
| ns | 5156 |  | 125 | Command: one-paragraph lede + headline kwargs | 3.5 | 3.1 | 0.405 |
| walker |  | 5165 | 23 | python method sigs #1 in src/click/exceptions.py |  |  | 0.405 |
| walker |  | 5165 | 0 | python method at src/click/exceptions.py:343 |  |  | 0.405 |
| walker |  | 5176 | 11 | python method body at src/click/exceptions.py:343 body 344 |  |  | 0.405 |
| walker |  | 5222 | 46 | python decl doc at src/click/exceptions.py:334 |  |  | 0.405 |
| walker |  | 5249 | 27 | python imports in src/click/_utils.py |  |  | 0.405 |
| walker |  | 5283 | 34 | docs/standalone-apps.md section #1 |  |  | 0.405 |
| walker |  | 5318 | 35 | docs/api.md section #9 |  |  | 0.405 |
| walker |  | 5330 | 12 | listing of 'examples/complex' |  |  | 0.405 |
| walker |  | 5343 | 13 | listing of 'examples/complex/complex' |  |  | 0.405 |
| ns | 5357 |  | 201 | Command.__init__ signature | 3.6 | 3.5 | 0.397 |
| walker |  | 5359 | 16 | listing of 'examples/complex/complex/commands' |  |  | 0.397 |
| walker |  | 5458 | 99 | docs/exceptions.md section #0 |  |  | 0.397 |
| walker |  | 5660 | 202 | README.md section #1 |  |  | 0.436 |
| ns | 5745 |  | 388 | Command: location of every method | 3.7 | 3.5 | 0.426 |
| walker |  | 5766 | 106 | docs/extending-click.md section #0 |  |  | 0.426 |
| walker |  | 5879 | 113 | docs/parameter-types.md section #0 |  |  | 0.426 |
| walker |  | 5996 | 117 | docs/support-multiple-versions.md section #0 |  |  | 0.426 |
| walker |  | 6127 | 131 | python decl names surface in docs/conf.py |  |  | 0.426 |
| walker |  | 6151 | 24 | python decl at docs/conf.py:31 |  |  | 0.426 |
| ns | 6201 |  | 456 | Group: lede + __init__ signature | 3.8 | 3.1 | 0.410 |
| walker |  | 6209 | 58 | python decl at docs/conf.py:27 |  |  | 0.410 |
| walker |  | 6302 | 93 | python decl at docs/conf.py:15 |  |  | 0.410 |
| ns | 6415 |  | 214 | Group: method-locations + decorator-style command/group registrars | 3.9 | 3.8 | 0.405 |
| walker |  | 6432 | 130 | docs/commands-and-groups.md section #0 |  |  | 0.405 |
| walker |  | 6566 | 134 | docs/arguments.md section #0 |  |  | 0.405 |
| ns | 6567 |  | 152 | Parameter (ABC): lede + key kwargs paragraph | 3.10 | 3.1 | 0.400 |
| walker |  | 6608 | 42 | python imports in src/click/_textwrap.py |  |  | 0.400 |
| walker |  | 6757 | 149 | docs/testing.md section #0 |  |  | 0.400 |
| walker |  | 6850 | 93 | python decl names surface in src/click/formatting.py |  |  | 0.400 |
| walker |  | 6850 | 0 | python decl at src/click/formatting.py:14 |  |  | 0.400 |
| walker |  | 6850 | 0 | python decl at src/click/formatting.py:104 |  |  | 0.400 |
| walker |  | 6850 | 0 | python decl at src/click/formatting.py:283 |  |  | 0.400 |
| ns | 6856 |  | 289 | Parameter.__init__ signature | 3.11 | 3.10 | 0.392 |
| walker |  | 6885 | 35 | python decl at src/click/formatting.py:24 |  |  | 0.392 |
| walker |  | 6948 | 63 | python decl at src/click/formatting.py:31 |  |  | 0.392 |
| walker |  | 6977 | 29 | python decl body at src/click/formatting.py:24 body 27 |  |  | 0.392 |
| walker |  | 7058 | 81 | python decl doc at src/click/formatting.py:283 |  |  | 0.392 |
| ns | 7202 |  | 346 | Option: lede + __init__ signature | 3.12 | 3.10 | 0.381 |
| walker |  | 7218 | 160 | docs/prompts.md section #0 |  |  | 0.381 |
| walker |  | 7270 | 52 | docs/quickstart.md section #1 |  |  | 0.381 |
| ns | 7355 |  | 153 | Argument: lede + __init__ signature + required-by-default logic | 3.13 | 3.10 | 0.376 |
| walker |  | 7365 | 95 | python decl names surface in src/click/testing.py |  |  | 0.376 |
| walker |  | 7365 | 0 | python decl at src/click/testing.py:26 |  |  | 0.376 |
| walker |  | 7365 | 0 | python decl at src/click/testing.py:70 |  |  | 0.376 |
| walker |  | 7365 | 0 | python decl at src/click/testing.py:89 |  |  | 0.376 |
| walker |  | 7365 | 0 | python decl at src/click/testing.py:103 |  |  | 0.376 |
| walker |  | 7365 | 0 | python decl at src/click/testing.py:183 |  |  | 0.376 |
| walker |  | 7365 | 0 | python decl at src/click/testing.py:261 |  |  | 0.376 |
| walker |  | 7408 | 43 | python decl at src/click/testing.py:163 |  |  | 0.376 |
| walker |  | 7430 | 22 | python decl at src/click/testing.py:60 |  |  | 0.376 |
| walker |  | 7470 | 40 | python decl doc at src/click/testing.py:70 |  |  | 0.376 |
| walker |  | 7521 | 51 | python decl doc at src/click/testing.py:89 |  |  | 0.376 |
| walker |  | 7646 | 125 | python decl doc at src/click/formatting.py:104 |  |  | 0.376 |
| ns | 7722 |  | 367 | Parameter/Option/Argument: method locations | 3.14 | 3.11 | 0.369 |
| walker |  | 7838 | 192 | python method sigs in src/click/formatting.py |  |  | 0.369 |
| walker |  | 7838 | 0 | python method at src/click/formatting.py:135 |  |  | 0.369 |
| walker |  | 7838 | 0 | python method at src/click/formatting.py:139 |  |  | 0.369 |
| walker |  | 7838 | 0 | python method at src/click/formatting.py:143 |  |  | 0.369 |
| walker |  | 7838 | 0 | python method at src/click/formatting.py:147 |  |  | 0.369 |
| walker |  | 7838 | 0 | python method at src/click/formatting.py:185 |  |  | 0.369 |
| walker |  | 7838 | 0 | python method at src/click/formatting.py:189 |  |  | 0.369 |
| walker |  | 7838 | 0 | python method at src/click/formatting.py:194 |  |  | 0.369 |
| walker |  | 7838 | 0 | python method at src/click/formatting.py:278 |  |  | 0.369 |
| walker |  | 7852 | 14 | python method at src/click/formatting.py:269 |  |  | 0.369 |
| walker |  | 7870 | 18 | python method at src/click/formatting.py:254 |  |  | 0.369 |
| walker |  | 7879 | 9 | python method doc at src/click/formatting.py:139 |  |  | 0.369 |
| walker |  | 7888 | 9 | python method doc at src/click/formatting.py:143 |  |  | 0.369 |
| walker |  | 7897 | 9 | python method doc at src/click/formatting.py:278 |  |  | 0.369 |
| walker |  | 7908 | 11 | python method doc at src/click/formatting.py:185 |  |  | 0.369 |
| walker |  | 7919 | 11 | python method doc at src/click/formatting.py:189 |  |  | 0.369 |
| walker |  | 7932 | 13 | python method doc at src/click/formatting.py:135 |  |  | 0.369 |
| walker |  | 7946 | 14 | python method doc at src/click/formatting.py:269 |  |  | 0.369 |
| walker |  | 7956 | 10 | python method body at src/click/formatting.py:135 body 137 |  |  | 0.369 |
| walker |  | 7967 | 11 | python method body at src/click/formatting.py:278 body 280 |  |  | 0.369 |
| walker |  | 7981 | 14 | python method body at src/click/formatting.py:139 body 141 |  |  | 0.369 |
| walker |  | 7995 | 14 | python method body at src/click/formatting.py:143 body 145 |  |  | 0.369 |
| ns | 8019 |  | 297 | types.py: every ParamType subclass header | 4.1 |  | 0.363 |
| walker |  | 8049 | 54 | python method at src/click/formatting.py:116 |  |  | 0.363 |
| walker |  | 8068 | 19 | python method body at src/click/formatting.py:189 body 191 |  |  | 0.363 |
| walker |  | 8124 | 56 | python method at src/click/formatting.py:210 |  |  | 0.363 |
| walker |  | 8158 | 34 | python method doc at src/click/formatting.py:194 |  |  | 0.363 |
| walker |  | 8180 | 22 | python method body at src/click/formatting.py:185 body 187 |  |  | 0.363 |
| walker |  | 8231 | 51 | python method doc at src/click/formatting.py:254 |  |  | 0.363 |
| ns | 8250 |  | 231 | ParamType: the subclassing recipe | 4.2 | 4.1 | 0.357 |
| walker |  | 8281 | 50 | python imports in src/click/globals.py |  |  | 0.357 |
| walker |  | 8408 | 127 | listing of 'tests' |  |  | 0.357 |
| ns | 8431 |  | 181 | ParamType.convert + ParamType.shell_complete signatures | 4.3 | 4.2 | 0.353 |
| walker |  | 8464 | 56 | listing of 'tests/typing' |  |  | 0.353 |
| ns | 8564 |  | 133 | exceptions.py: every class header | 4.4 |  | 0.349 |
| walker |  | 8569 | 105 | python decl names surface in src/click/_utils.py |  |  | 0.349 |
| walker |  | 8569 | 0 | python decl at src/click/_utils.py:7 |  |  | 0.349 |
| walker |  | 8585 | 16 | python method sigs in src/click/_utils.py |  |  | 0.349 |
| walker |  | 8585 | 0 | python method at src/click/_utils.py:18 |  |  | 0.349 |
| walker |  | 8607 | 22 | python class body at src/click/_utils.py:7 |  |  | 0.349 |
| walker |  | 8623 | 16 | python method body at src/click/_utils.py:18 body 19 |  |  | 0.349 |
| walker |  | 8678 | 55 | python decl doc at src/click/_utils.py:7 |  |  | 0.349 |
| walker |  | 8783 | 105 | python decl names surface in src/click/globals.py |  |  | 0.349 |
| walker |  | 8783 | 0 | python decl at src/click/globals.py:12 |  |  | 0.349 |
| walker |  | 8783 | 0 | python decl at src/click/globals.py:16 |  |  | 0.349 |
| walker |  | 8783 | 0 | python decl at src/click/globals.py:20 |  |  | 0.349 |
| walker |  | 8783 | 0 | python decl at src/click/globals.py:44 |  |  | 0.349 |
| walker |  | 8783 | 0 | python decl at src/click/globals.py:49 |  |  | 0.349 |
| walker |  | 8783 | 0 | python decl at src/click/globals.py:54 |  |  | 0.349 |
| ns | 8790 |  | 226 | ClickException + UsageError ledes + exit_code | 4.5 | 4.4 | 0.344 |
| walker |  | 8793 | 10 | python decl body at src/click/globals.py:49 body 51 |  |  | 0.344 |
| walker |  | 8806 | 13 | python decl doc at src/click/globals.py:49 |  |  | 0.344 |
| walker |  | 8820 | 14 | python decl doc at src/click/globals.py:44 |  |  | 0.344 |
| walker |  | 8838 | 18 | python decl body at src/click/globals.py:16 body 17 |  |  | 0.344 |
| ns | 8844 |  | 54 | testing.py: CliRunner, Result, helper class locations | 5.1 |  | 0.351 |
| walker |  | 8857 | 19 | python decl body at src/click/globals.py:44 body 46 |  |  | 0.351 |
| walker |  | 8879 | 22 | python decl body at src/click/globals.py:12 body 13 |  |  | 0.351 |
| walker |  | 8932 | 53 | python decl doc at src/click/globals.py:54 |  |  | 0.351 |
| walker |  | 8940 | 8 | python decl body at src/click/globals.py:20 body 41 |  |  | 0.351 |
| ns | 8975 |  | 131 | CliRunner.invoke signature | 5.2 | 5.1 | 0.348 |
| walker |  | 9133 | 193 | docs/shell-completion.md section #0 |  |  | 0.348 |
| ns | 9163 |  | 188 | Result: attribute documentation | 5.3 | 5.1 | 0.344 |
| ns | 9224 |  | 61 | tests/conftest.py: the canonical 'runner' fixture | 5.4 | 5.1 | 0.342 |
| walker |  | 9330 | 197 | docs/contrib.md section #0 |  |  | 0.342 |
| ns | 9391 |  | 167 | termui.py: every public function location | 5.5 |  | 0.338 |
| ns | 9472 |  | 81 | utils.py: echo() signature + key kwargs | 5.6 |  | 0.336 |
| ns | 9508 |  | 36 | examples/ subdirectory listing | 6.1 |  | 0.349 |
| walker |  | 9529 | 199 | docs/options.md section #0 |  |  | 0.349 |
| walker |  | 9591 | 62 | docs/api.md section #8 |  |  | 0.349 |
| walker |  | 9611 | 20 | docs/commands-and-groups.md section #17 |  |  | 0.349 |
| ns | 9715 |  | 207 | examples/naval/naval.py: canonical nested-group example | 6.2 |  | 0.343 |
| walker |  | 9820 | 209 | docs/entry-points.md section #0 |  |  | 0.343 |
| walker |  | 9897 | 77 | python method doc at src/click/formatting.py:147 |  |  | 0.343 |
| ns | 9915 |  | 200 | docs/index.rst: General Reference toctree | 6.3 |  | 0.336 |

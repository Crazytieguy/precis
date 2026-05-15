Score(3000)=0.282 I=0.645 C=0.123 ns_rows≤3K=20/52 (reached=2 partial=2 missing=16)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 50 | 50 | listing of '.' |  |  | 1.000 |
| ns | 50 |  | 50 | Top-level directory listing | 1.1 |  | 1.000 |
| walker |  | 53 | 3 | listing of 'src' |  |  | 1.000 |
| walker |  | 64 | 11 | listing of '.devcontainer' |  |  | 1.000 |
| ns | 92 |  | 42 | Package name + version | 1.2 |  | 0.859 |
| walker |  | 136 | 72 | README headline in README.md |  |  | 0.887 |
| walker |  | 166 | 30 | headings outline in README.md |  |  | 0.888 |
| ns | 169 |  | 77 | README headline + tagline | 1.3 |  | 0.861 |
| walker |  | 195 | 29 | [package] in pyproject.toml |  |  | 0.931 |
| ns | 209 |  | 40 | Python version floor + runtime dep | 1.4 | 1.2 | 0.848 |
| walker |  | 286 | 91 | README.md section #0 |  |  | 0.872 |
| ns | 310 |  | 101 | README differentiators (three-bullet pitch) | 1.5 | 1.3 | 0.844 |
| walker |  | 372 | 86 | listing of 'src/click' |  |  | 0.844 |
| walker |  | 390 | 18 | python decl names surface in src/click/__init__.py |  |  | 0.844 |
| walker |  | 390 | 0 | python decl at src/click/__init__.py:77 |  |  | 0.844 |
| walker |  | 403 | 13 | listing of '.github' |  |  | 0.844 |
| ns | 410 |  | 100 | __init__.py: core-class re-exports | 1.6 |  | 0.750 |
| walker |  | 426 | 23 | listing of '.github/workflows' |  |  | 0.750 |
| walker |  | 440 | 14 | python decl names surface in src/click/_textwrap.py |  |  | 0.750 |
| walker |  | 440 | 0 | python decl at src/click/_textwrap.py:8 |  |  | 0.750 |
| walker |  | 481 | 41 | python method sigs in src/click/_textwrap.py |  |  | 0.750 |
| walker |  | 481 | 0 | python method at src/click/_textwrap.py:40 |  |  | 0.750 |
| walker |  | 500 | 19 | python method at src/click/_textwrap.py:27 |  |  | 0.750 |
| walker |  | 567 | 67 | README.md section #3 |  |  | 0.750 |
| ns | 578 |  | 168 | README hello-world example | 1.7 | 1.3 | 0.627 |
| walker |  | 649 | 82 | README.md section #2 |  |  | 0.627 |
| walker |  | 685 | 36 | listing of 'examples' |  |  | 0.628 |
| ns | 739 |  | 161 | __init__.py: decorator re-exports | 1.8 | 1.6 | 0.569 |
| walker |  | 877 | 192 | listing of 'docs' |  |  | 0.569 |
| walker |  | 898 | 21 | headings outline in docs/click-concepts.md |  |  | 0.569 |
| walker |  | 919 | 21 | headings outline in docs/unicode-support.md |  |  | 0.569 |
| ns | 931 |  | 192 | __init__.py: exception + formatting + globals re-exports | 1.9 | 1.8 | 0.516 |
| walker |  | 941 | 22 | headings outline in docs/command-line-reference.md |  |  | 0.516 |
| walker |  | 963 | 22 | headings outline in docs/contrib.md |  |  | 0.516 |
| walker |  | 985 | 22 | headings outline in docs/design-opinions.md |  |  | 0.516 |
| walker |  | 1015 | 30 | headings outline in docs/entry-points.md |  |  | 0.516 |
| walker |  | 1047 | 32 | headings outline in docs/extending-click.md |  |  | 0.516 |
| walker |  | 1080 | 33 | headings outline in docs/faqs.md |  |  | 0.516 |
| walker |  | 1113 | 33 | headings outline in docs/wincmd.md |  |  | 0.516 |
| ns | 1120 |  | 189 | __init__.py: termui re-exports | 1.10 | 1.9 | 0.474 |
| walker |  | 1148 | 35 | headings outline in docs/virtualenv.md |  |  | 0.474 |
| walker |  | 1148 | 0 | docs/virtualenv.md section #0 |  |  | 0.474 |
| walker |  | 1185 | 37 | headings outline in docs/parameters.md |  |  | 0.474 |
| walker |  | 1227 | 42 | headings outline in docs/option-decorators.md |  |  | 0.474 |
| walker |  | 1270 | 43 | headings outline in docs/handling-files.md |  |  | 0.474 |
| ns | 1302 |  | 182 | __init__.py: built-in type re-exports | 1.11 | 1.10 | 0.438 |
| walker |  | 1321 | 51 | headings outline in docs/arguments.md |  |  | 0.438 |
| walker |  | 1373 | 52 | headings outline in docs/support-multiple-versions.md |  |  | 0.438 |
| ns | 1395 |  | 93 | __init__.py: utils re-exports | 1.12 | 1.11 | 0.423 |
| walker |  | 1431 | 58 | headings outline in docs/shell-completion.md |  |  | 0.423 |
| walker |  | 1492 | 61 | headings outline in docs/why.md |  |  | 0.423 |
| walker |  | 1554 | 62 | headings outline in docs/standalone-apps.md |  |  | 0.423 |
| walker |  | 1618 | 64 | headings outline in docs/exceptions.md |  |  | 0.423 |
| ns | 1647 |  | 252 | Deprecation shim: BaseCommand → Command, MultiCommand → Group | 1.13 | 1.12 | 0.380 |
| walker |  | 1687 | 69 | headings outline in docs/parameter-types.md |  |  | 0.380 |
| walker |  | 1756 | 69 | headings outline in docs/prompts.md |  |  | 0.380 |
| walker |  | 1766 | 10 | docs/parameter-types.md section #1 |  |  | 0.380 |
| walker |  | 1844 | 78 | headings outline in docs/testing.md |  |  | 0.380 |
| walker |  | 1866 | 22 | docs/setuptools.md section #0 |  |  | 0.380 |
| ns | 1916 |  | 269 | Deprecation shim: OptionParser, __version__ | 1.14 | 1.13 | 0.346 |
| walker |  | 1958 | 92 | headings outline in docs/advanced.md |  |  | 0.346 |
| walker |  | 2055 | 97 | headings outline in docs/quickstart.md |  |  | 0.346 |
| walker |  | 2072 | 17 | docs/quickstart.md section #0 |  |  | 0.346 |
| walker |  | 2087 | 15 | listing of 'docs/_static' |  |  | 0.346 |
| ns | 2121 |  | 205 | decorators.py: location of every public decorator | 2.1 |  | 0.332 |
| walker |  | 2197 | 110 | headings outline in docs/api.md |  |  | 0.332 |
| ns | 2209 |  | 88 | @command: signature + 1-liner | 2.2 | 2.1 | 0.326 |
| walker |  | 2308 | 111 | headings outline in docs/commands.md |  |  | 0.326 |
| walker |  | 2341 | 33 | docs/faqs.md section #0 |  |  | 0.326 |
| ns | 2367 |  | 158 | @option: signature + 1-liner | 2.3 | 2.1 | 0.316 |
| walker |  | 2482 | 141 | headings outline in docs/documentation.md |  |  | 0.316 |
| walker |  | 2521 | 39 | docs/parameters.md section #0 |  |  | 0.316 |
| ns | 2525 |  | 158 | @argument: signature + 1-liner | 2.4 | 2.1 | 0.307 |
| walker |  | 2673 | 152 | headings outline in docs/utils.md |  |  | 0.307 |
| ns | 2688 |  | 163 | @group: signature + 1-liner | 2.5 | 2.1 | 0.297 |
| walker |  | 2837 | 164 | headings outline in docs/complex.md |  |  | 0.297 |
| ns | 2923 |  | 235 | @pass_context / @pass_obj signatures + docstrings | 2.6 | 2.1 | 0.282 |
| walker |  | 3005 | 168 | headings outline in docs/options.md |  |  | 0.282 |
| walker |  | 3053 | 48 | docs/license.md section #0 |  |  | 0.282 |
| ns | 3240 |  | 317 | @make_pass_decorator: signature + docstring + how-it-works | 2.7 | 2.1 | 0.266 |
| walker |  | 3247 | 194 | headings outline in docs/commands-and-groups.md |  |  | 0.266 |
| walker |  | 3263 | 16 | docs/commands-and-groups.md section #13 |  |  | 0.266 |
| walker |  | 3291 | 28 | python imports in docs/conf.py |  |  | 0.266 |
| walker |  | 3348 | 57 | docs/documentation.md section #0 |  |  | 0.266 |
| walker |  | 3405 | 57 | docs/utils.md section #0 |  |  | 0.266 |
| walker |  | 3465 | 60 | docs/design-opinions.md section #0 |  |  | 0.266 |
| walker |  | 3473 | 8 | docs/parameter-types.md section #8 |  |  | 0.266 |
| ns | 3498 |  | 258 | @confirmation_option / @password_option full bodies | 2.8 | 2.1 | 0.253 |
| walker |  | 3537 | 64 | docs/handling-files.md section #0 |  |  | 0.253 |
| walker |  | 3602 | 65 | docs/click-concepts.md section #0 |  |  | 0.253 |
| walker |  | 3668 | 66 | docs/advanced.md section #0 |  |  | 0.253 |
| walker |  | 3736 | 68 | docs/command-line-reference.md section #0 |  |  | 0.253 |
| ns | 3738 |  | 240 | @version_option signature + key kwargs | 2.9 | 2.1 | 0.244 |
| walker |  | 3764 | 28 | docs/complex.md section #1 |  |  | 0.244 |
| walker |  | 3792 | 28 | docs/option-decorators.md section #3 |  |  | 0.244 |
| walker |  | 3866 | 74 | docs/wincmd.md section #0 |  |  | 0.244 |
| walker |  | 3942 | 76 | docs/api.md section #0 |  |  | 0.244 |
| walker |  | 4019 | 77 | docs/option-decorators.md section #0 |  |  | 0.244 |
| ns | 4085 |  | 347 | quickstart.md: 'Basic Concepts - Creating a Command' section | 2.10 |  | 0.231 |
| ns | 4205 |  | 120 | core.py: every class header (top-level inventory) | 3.1 |  | 0.226 |
| ns | 4349 |  | 144 | Context: one-paragraph lede | 3.2 | 3.1 | 0.222 |
| ns | 4618 |  | 269 | Context.__init__ signature | 3.3 | 3.2 | 0.216 |
| walker |  | 5015 | 996 | python imports in src/click/__init__.py |  |  | 0.424 |
| ns | 5031 |  | 413 | Context: location of every method | 3.4 | 3.2 | 0.410 |
| walker |  | 5103 | 88 | docs/commands.md section #0 |  |  | 0.410 |
| walker |  | 5125 | 22 | python decl names surface #1 in src/click/exceptions.py |  |  | 0.410 |
| walker |  | 5125 | 0 | python decl at src/click/exceptions.py:330 |  |  | 0.410 |
| walker |  | 5125 | 0 | python decl at src/click/exceptions.py:334 |  |  | 0.410 |
| walker |  | 5139 | 14 | python class body at src/click/exceptions.py:334 |  |  | 0.410 |
| walker |  | 5153 | 14 | python decl doc at src/click/exceptions.py:330 |  |  | 0.410 |
| ns | 5156 |  | 125 | Command: one-paragraph lede + headline kwargs | 3.5 | 3.1 | 0.405 |
| walker |  | 5176 | 23 | python method sigs #1 in src/click/exceptions.py |  |  | 0.405 |
| walker |  | 5176 | 0 | python method at src/click/exceptions.py:343 |  |  | 0.405 |
| walker |  | 5187 | 11 | python method body at src/click/exceptions.py:343 body 344 |  |  | 0.405 |
| walker |  | 5233 | 46 | python decl doc at src/click/exceptions.py:334 |  |  | 0.405 |
| walker |  | 5260 | 27 | python imports in src/click/_utils.py |  |  | 0.405 |
| walker |  | 5294 | 34 | docs/standalone-apps.md section #1 |  |  | 0.405 |
| walker |  | 5329 | 35 | docs/api.md section #9 |  |  | 0.405 |
| walker |  | 5341 | 12 | listing of 'examples/complex' |  |  | 0.405 |
| walker |  | 5354 | 13 | listing of 'examples/complex/complex' |  |  | 0.405 |
| ns | 5357 |  | 201 | Command.__init__ signature | 3.6 | 3.5 | 0.397 |
| walker |  | 5370 | 16 | listing of 'examples/complex/complex/commands' |  |  | 0.397 |
| walker |  | 5469 | 99 | docs/exceptions.md section #0 |  |  | 0.397 |
| walker |  | 5671 | 202 | README.md section #1 |  |  | 0.436 |
| ns | 5745 |  | 388 | Command: location of every method | 3.7 | 3.5 | 0.426 |
| walker |  | 5777 | 106 | docs/extending-click.md section #0 |  |  | 0.426 |
| walker |  | 5890 | 113 | docs/parameter-types.md section #0 |  |  | 0.426 |
| walker |  | 6007 | 117 | docs/support-multiple-versions.md section #0 |  |  | 0.426 |
| walker |  | 6138 | 131 | python decl names surface in docs/conf.py |  |  | 0.426 |
| walker |  | 6162 | 24 | python decl at docs/conf.py:31 |  |  | 0.426 |
| ns | 6201 |  | 456 | Group: lede + __init__ signature | 3.8 | 3.1 | 0.410 |
| walker |  | 6220 | 58 | python decl at docs/conf.py:27 |  |  | 0.410 |
| walker |  | 6313 | 93 | python decl at docs/conf.py:15 |  |  | 0.410 |
| ns | 6415 |  | 214 | Group: method-locations + decorator-style command/group registrars | 3.9 | 3.8 | 0.405 |
| walker |  | 6443 | 130 | docs/commands-and-groups.md section #0 |  |  | 0.405 |
| walker |  | 6510 | 67 | plaintext config .gitignore |  |  | 0.405 |
| ns | 6567 |  | 152 | Parameter (ABC): lede + key kwargs paragraph | 3.10 | 3.1 | 0.400 |
| walker |  | 6644 | 134 | docs/arguments.md section #0 |  |  | 0.400 |
| walker |  | 6767 | 123 | plaintext config .editorconfig |  |  | 0.400 |
| walker |  | 6809 | 42 | python imports in src/click/_textwrap.py |  |  | 0.400 |
| ns | 6856 |  | 289 | Parameter.__init__ signature | 3.11 | 3.10 | 0.392 |
| walker |  | 6958 | 149 | docs/testing.md section #0 |  |  | 0.392 |
| walker |  | 7051 | 93 | python decl names surface in src/click/formatting.py |  |  | 0.392 |
| walker |  | 7051 | 0 | python decl at src/click/formatting.py:14 |  |  | 0.392 |
| walker |  | 7051 | 0 | python decl at src/click/formatting.py:104 |  |  | 0.392 |
| walker |  | 7051 | 0 | python decl at src/click/formatting.py:283 |  |  | 0.392 |
| walker |  | 7086 | 35 | python decl at src/click/formatting.py:24 |  |  | 0.392 |
| walker |  | 7149 | 63 | python decl at src/click/formatting.py:31 |  |  | 0.392 |
| walker |  | 7178 | 29 | python decl body at src/click/formatting.py:24 body 27 |  |  | 0.392 |
| ns | 7202 |  | 346 | Option: lede + __init__ signature | 3.12 | 3.10 | 0.381 |
| walker |  | 7259 | 81 | python decl doc at src/click/formatting.py:283 |  |  | 0.381 |
| ns | 7355 |  | 153 | Argument: lede + __init__ signature + required-by-default logic | 3.13 | 3.10 | 0.376 |
| walker |  | 7419 | 160 | docs/prompts.md section #0 |  |  | 0.376 |
| walker |  | 7471 | 52 | docs/quickstart.md section #1 |  |  | 0.376 |
| walker |  | 7566 | 95 | python decl names surface in src/click/testing.py |  |  | 0.376 |
| walker |  | 7566 | 0 | python decl at src/click/testing.py:26 |  |  | 0.376 |
| walker |  | 7566 | 0 | python decl at src/click/testing.py:70 |  |  | 0.376 |
| walker |  | 7566 | 0 | python decl at src/click/testing.py:89 |  |  | 0.376 |
| walker |  | 7566 | 0 | python decl at src/click/testing.py:103 |  |  | 0.376 |
| walker |  | 7566 | 0 | python decl at src/click/testing.py:183 |  |  | 0.376 |
| walker |  | 7566 | 0 | python decl at src/click/testing.py:261 |  |  | 0.376 |
| walker |  | 7609 | 43 | python decl at src/click/testing.py:163 |  |  | 0.376 |
| walker |  | 7631 | 22 | python decl at src/click/testing.py:60 |  |  | 0.376 |
| walker |  | 7671 | 40 | python decl doc at src/click/testing.py:70 |  |  | 0.376 |
| walker |  | 7722 | 51 | python decl doc at src/click/testing.py:89 |  |  | 0.369 |
| ns | 7722 |  | 367 | Parameter/Option/Argument: method locations | 3.14 | 3.11 | 0.369 |
| walker |  | 7847 | 125 | python decl doc at src/click/formatting.py:104 |  |  | 0.369 |
| ns | 8019 |  | 297 | types.py: every ParamType subclass header | 4.1 |  | 0.363 |
| walker |  | 8039 | 192 | python method sigs in src/click/formatting.py |  |  | 0.363 |
| walker |  | 8039 | 0 | python method at src/click/formatting.py:135 |  |  | 0.363 |
| walker |  | 8039 | 0 | python method at src/click/formatting.py:139 |  |  | 0.363 |
| walker |  | 8039 | 0 | python method at src/click/formatting.py:143 |  |  | 0.363 |
| walker |  | 8039 | 0 | python method at src/click/formatting.py:147 |  |  | 0.363 |
| walker |  | 8039 | 0 | python method at src/click/formatting.py:185 |  |  | 0.363 |
| walker |  | 8039 | 0 | python method at src/click/formatting.py:189 |  |  | 0.363 |
| walker |  | 8039 | 0 | python method at src/click/formatting.py:194 |  |  | 0.363 |
| walker |  | 8039 | 0 | python method at src/click/formatting.py:278 |  |  | 0.363 |
| walker |  | 8053 | 14 | python method at src/click/formatting.py:269 |  |  | 0.363 |
| walker |  | 8071 | 18 | python method at src/click/formatting.py:254 |  |  | 0.363 |
| walker |  | 8080 | 9 | python method doc at src/click/formatting.py:139 |  |  | 0.363 |
| walker |  | 8089 | 9 | python method doc at src/click/formatting.py:143 |  |  | 0.363 |
| walker |  | 8098 | 9 | python method doc at src/click/formatting.py:278 |  |  | 0.363 |
| walker |  | 8109 | 11 | python method doc at src/click/formatting.py:185 |  |  | 0.363 |
| walker |  | 8120 | 11 | python method doc at src/click/formatting.py:189 |  |  | 0.363 |
| walker |  | 8133 | 13 | python method doc at src/click/formatting.py:135 |  |  | 0.363 |
| walker |  | 8147 | 14 | python method doc at src/click/formatting.py:269 |  |  | 0.363 |
| walker |  | 8157 | 10 | python method body at src/click/formatting.py:135 body 137 |  |  | 0.363 |
| walker |  | 8168 | 11 | python method body at src/click/formatting.py:278 body 280 |  |  | 0.363 |
| walker |  | 8182 | 14 | python method body at src/click/formatting.py:139 body 141 |  |  | 0.363 |
| walker |  | 8196 | 14 | python method body at src/click/formatting.py:143 body 145 |  |  | 0.363 |
| walker |  | 8250 | 54 | python method at src/click/formatting.py:116 |  |  | 0.357 |
| ns | 8250 |  | 231 | ParamType: the subclassing recipe | 4.2 | 4.1 | 0.357 |
| walker |  | 8269 | 19 | python method body at src/click/formatting.py:189 body 191 |  |  | 0.357 |
| walker |  | 8325 | 56 | python method at src/click/formatting.py:210 |  |  | 0.357 |
| walker |  | 8359 | 34 | python method doc at src/click/formatting.py:194 |  |  | 0.357 |
| walker |  | 8381 | 22 | python method body at src/click/formatting.py:185 body 187 |  |  | 0.357 |
| ns | 8431 |  | 181 | ParamType.convert + ParamType.shell_complete signatures | 4.3 | 4.2 | 0.353 |
| walker |  | 8432 | 51 | python method doc at src/click/formatting.py:254 |  |  | 0.353 |
| walker |  | 8482 | 50 | python imports in src/click/globals.py |  |  | 0.353 |
| ns | 8564 |  | 133 | exceptions.py: every class header | 4.4 |  | 0.349 |
| walker |  | 8609 | 127 | listing of 'tests' |  |  | 0.349 |
| walker |  | 8665 | 56 | listing of 'tests/typing' |  |  | 0.349 |
| walker |  | 8770 | 105 | python decl names surface in src/click/_utils.py |  |  | 0.349 |
| walker |  | 8770 | 0 | python decl at src/click/_utils.py:7 |  |  | 0.349 |
| walker |  | 8786 | 16 | python method sigs in src/click/_utils.py |  |  | 0.349 |
| walker |  | 8786 | 0 | python method at src/click/_utils.py:18 |  |  | 0.349 |
| ns | 8790 |  | 226 | ClickException + UsageError ledes + exit_code | 4.5 | 4.4 | 0.344 |
| walker |  | 8808 | 22 | python class body at src/click/_utils.py:7 |  |  | 0.344 |
| walker |  | 8824 | 16 | python method body at src/click/_utils.py:18 body 19 |  |  | 0.344 |
| ns | 8844 |  | 54 | testing.py: CliRunner, Result, helper class locations | 5.1 |  | 0.351 |
| walker |  | 8879 | 55 | python decl doc at src/click/_utils.py:7 |  |  | 0.351 |
| ns | 8975 |  | 131 | CliRunner.invoke signature | 5.2 | 5.1 | 0.348 |
| walker |  | 8984 | 105 | python decl names surface in src/click/globals.py |  |  | 0.348 |
| walker |  | 8984 | 0 | python decl at src/click/globals.py:12 |  |  | 0.348 |
| walker |  | 8984 | 0 | python decl at src/click/globals.py:16 |  |  | 0.348 |
| walker |  | 8984 | 0 | python decl at src/click/globals.py:20 |  |  | 0.348 |
| walker |  | 8984 | 0 | python decl at src/click/globals.py:44 |  |  | 0.348 |
| walker |  | 8984 | 0 | python decl at src/click/globals.py:49 |  |  | 0.348 |
| walker |  | 8984 | 0 | python decl at src/click/globals.py:54 |  |  | 0.348 |
| walker |  | 8994 | 10 | python decl body at src/click/globals.py:49 body 51 |  |  | 0.348 |
| walker |  | 9007 | 13 | python decl doc at src/click/globals.py:49 |  |  | 0.348 |
| walker |  | 9021 | 14 | python decl doc at src/click/globals.py:44 |  |  | 0.348 |
| walker |  | 9039 | 18 | python decl body at src/click/globals.py:16 body 17 |  |  | 0.348 |
| walker |  | 9058 | 19 | python decl body at src/click/globals.py:44 body 46 |  |  | 0.348 |
| walker |  | 9080 | 22 | python decl body at src/click/globals.py:12 body 13 |  |  | 0.348 |
| walker |  | 9133 | 53 | python decl doc at src/click/globals.py:54 |  |  | 0.348 |
| walker |  | 9141 | 8 | python decl body at src/click/globals.py:20 body 41 |  |  | 0.348 |
| ns | 9163 |  | 188 | Result: attribute documentation | 5.3 | 5.1 | 0.344 |
| ns | 9224 |  | 61 | tests/conftest.py: the canonical 'runner' fixture | 5.4 | 5.1 | 0.342 |
| walker |  | 9334 | 193 | docs/shell-completion.md section #0 |  |  | 0.342 |
| ns | 9391 |  | 167 | termui.py: every public function location | 5.5 |  | 0.338 |
| ns | 9472 |  | 81 | utils.py: echo() signature + key kwargs | 5.6 |  | 0.336 |
| ns | 9508 |  | 36 | examples/ subdirectory listing | 6.1 |  | 0.349 |
| walker |  | 9531 | 197 | docs/contrib.md section #0 |  |  | 0.349 |
| ns | 9715 |  | 207 | examples/naval/naval.py: canonical nested-group example | 6.2 |  | 0.343 |
| walker |  | 9730 | 199 | docs/options.md section #0 |  |  | 0.343 |
| walker |  | 9792 | 62 | docs/api.md section #8 |  |  | 0.343 |
| walker |  | 9812 | 20 | docs/commands-and-groups.md section #17 |  |  | 0.343 |
| ns | 9915 |  | 200 | docs/index.rst: General Reference toctree | 6.3 |  | 0.336 |

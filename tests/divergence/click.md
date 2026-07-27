Score(3000)=0.709 I=0.822 C=0.611 ns_rows≤3K=18/46 (reached=11 partial=1 missing=6)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 56 | 56 | listing of '.' |  |  | 1.000 |
| ns | 56 |  | 56 | Root directory listing | 1.1 |  | 1.000 |
| walker |  | 59 | 3 | listing of 'src' |  |  | 1.000 |
| ns | 137 |  | 81 | README title + lede | 1.2 |  | 0.806 |
| walker |  | 140 | 81 | README headline in README.md |  |  | 1.000 |
| walker |  | 170 | 30 | headings outline in README.md |  |  | 1.000 |
| walker |  | 201 | 31 | [dependencies] in pyproject.toml |  |  | 1.000 |
| ns | 242 |  | 105 | pyproject.toml package identity | 1.3 |  | 0.816 |
| ns | 271 |  | 29 | CHANGES.rst range markers | 1.4 |  | 0.783 |
| ns | 389 |  | 118 | docs/ sitemap (index.rst toctree, Documentation + Tutorials) | 1.5 |  | 0.606 |
| walker |  | 393 | 192 | listing of 'docs' |  |  | 0.637 |
| walker |  | 407 | 14 | listing of 'docs/_static' |  |  | 0.642 |
| walker |  | 417 | 10 | listing of '.devcontainer' |  |  | 0.642 |
| walker |  | 431 | 14 | listing of '.github' |  |  | 0.642 |
| walker |  | 453 | 22 | listing of '.github/workflows' |  |  | 0.644 |
| ns | 473 |  | 84 | docs/ sitemap (index.rst toctree, How to Guides) | 1.6 | 1.5 | 0.573 |
| walker |  | 538 | 85 | listing of 'src/click' |  |  | 0.592 |
| walker |  | 553 | 15 | python imports #1 in src/click/__init__.py |  |  | 0.592 |
| ns | 559 |  | 86 | src/click/ module listing | 1.7 |  | 0.683 |
| walker |  | 624 | 71 | python imports in src/click/__init__.py |  |  | 0.684 |
| walker |  | 642 | 18 | python imports #6 in src/click/__init__.py |  |  | 0.684 |
| walker |  | 672 | 30 | python imports #5 in src/click/__init__.py |  |  | 0.684 |
| ns | 766 |  | 207 | docs/ + docs/_static/ listing | 1.8 |  | 0.756 |
| walker |  | 772 | 100 | python imports #2 in src/click/__init__.py |  |  | 0.761 |
| walker |  | 790 | 18 | python decl names surface in src/click/__init__.py |  |  | 0.761 |
| walker |  | 790 | 0 | python decl at src/click/__init__.py:77 |  |  | 0.761 |
| ns | 949 |  | 183 | tests/ + tests/typing/ listing | 1.9 |  | 0.659 |
| walker |  | 951 | 161 | python imports #3 in src/click/__init__.py |  |  | 0.661 |
| walker |  | 1095 | 144 | python imports #4 in src/click/__init__.py |  |  | 0.665 |
| ns | 1176 |  | 227 | examples/ tree listing | 1.10 |  | 0.560 |
| walker |  | 1183 | 88 | python imports #9 in src/click/__init__.py |  |  | 0.561 |
| ns | 1237 |  | 61 | .github/ + .devcontainer/ listing | 1.11 |  | 0.566 |
| walker |  | 1282 | 99 | README.md section #0 |  |  | 0.566 |
| ns | 1334 |  | 97 | pyproject.toml: pytest invocation config | 1.12 |  | 0.552 |
| walker |  | 1471 | 189 | python imports #7 in src/click/__init__.py |  |  | 0.556 |
| ns | 1530 |  | 196 | click/__init__.py: docstring + core exports | 2.1 |  | 0.576 |
| walker |  | 1651 | 180 | python imports #8 in src/click/__init__.py |  |  | 0.579 |
| walker |  | 1729 | 78 | [package] in pyproject.toml |  |  | 0.597 |
| walker |  | 1767 | 38 | package metadata in pyproject.toml |  |  | 0.621 |
| walker |  | 1812 | 45 | listing of 'examples' |  |  | 0.628 |
| ns | 1837 |  | 307 | click/__init__.py: decorators + exceptions exports | 2.2 |  | 0.651 |
| walker |  | 1939 | 127 | listing of 'tests' |  |  | 0.698 |
| ns | 2072 |  | 235 | click/__init__.py: formatting/globals/termui exports | 2.3 |  | 0.709 |
| walker |  | 2073 | 134 | manifest config in pyproject.toml |  |  | 0.709 |
| walker |  | 2102 | 29 | docs/setuptools.md section #0 |  |  | 0.709 |
| ns | 2254 |  | 182 | click/__init__.py: parameter-type exports | 2.4 |  | 0.717 |
| walker |  | 2265 | 163 | tool.mypy+pyright config in pyproject.toml |  |  | 0.717 |
| walker |  | 2340 | 75 | README.md section #3 |  |  | 0.717 |
| ns | 2342 |  | 88 | click/__init__.py: utils exports | 2.5 |  | 0.720 |
| walker |  | 2430 | 90 | README.md section #2 |  |  | 0.720 |
| walker |  | 2640 | 210 | tool.ruff config in pyproject.toml |  |  | 0.720 |
| walker |  | 2693 | 53 | docs/license.md section #0 |  |  | 0.720 |
| walker |  | 2748 | 55 | listing of 'tests/typing' |  |  | 0.758 |
| ns | 2761 |  | 419 | _utils.py: UNSET / FLAG_NEEDS_VALUE sentinels | 2.6 |  | 0.709 |
| walker |  | 2769 | 21 | headings outline in docs/click-concepts.md |  |  | 0.709 |
| walker |  | 2790 | 21 | headings outline in docs/unicode-support.md |  |  | 0.709 |
| walker |  | 2812 | 22 | headings outline in docs/command-line-reference.md |  |  | 0.709 |
| walker |  | 2834 | 22 | headings outline in docs/design-opinions.md |  |  | 0.709 |
| walker |  | 2902 | 68 | docs/design-opinions.md section #0 |  |  | 0.709 |
| walker |  | 2926 | 24 | headings outline in docs/contrib.md |  |  | 0.709 |
| ns | 3020 |  | 259 | globals.py: context-stack push/pop | 2.7 |  | 0.681 |
| ns | 3160 |  | 140 | ParameterSource enum members | 3.1 |  | 0.670 |
| walker |  | 3234 | 308 | dev/build/target dependencies in pyproject.toml |  |  | 0.670 |
| ns | 3431 |  | 271 | Context.__init__ signature | 3.2 |  | 0.651 |
| walker |  | 3601 | 367 | tool.flit+uv+pytest+coverage config in pyproject.toml |  |  | 0.668 |
| walker |  | 3679 | 78 | docs/click-concepts.md section #0 |  |  | 0.668 |
| walker |  | 3760 | 81 | docs/command-line-reference.md section #0 |  |  | 0.668 |
| walker |  | 3790 | 30 | headings outline in docs/entry-points.md |  |  | 0.668 |
| walker |  | 3822 | 32 | headings outline in docs/extending-click.md |  |  | 0.668 |
| walker |  | 3855 | 33 | headings outline in docs/wincmd.md |  |  | 0.668 |
| ns | 3901 |  | 470 | Context — public method locations | 3.3 |  | 0.645 |
| walker |  | 3942 | 87 | docs/wincmd.md section #0 |  |  | 0.645 |
| walker |  | 4162 | 220 | README.md section #1 |  |  | 0.645 |
| walker |  | 4198 | 36 | headings outline in docs/faqs.md |  |  | 0.645 |
| walker |  | 4234 | 36 | docs/faqs.md section #0 |  |  | 0.645 |
| walker |  | 4273 | 39 | headings outline in docs/parameters.md |  |  | 0.645 |
| walker |  | 4320 | 47 | docs/parameters.md section #0 |  |  | 0.645 |
| ns | 4345 |  | 444 | Context.invoke — Command branch | 3.4 | 3.3 | 0.615 |
| walker |  | 4360 | 40 | headings outline in docs/virtualenv.md |  |  | 0.615 |
| walker |  | 4360 | 0 | docs/virtualenv.md section #0 |  |  | 0.615 |
| walker |  | 4484 | 124 | docs/extending-click.md section #0 |  |  | 0.615 |
| walker |  | 4526 | 42 | headings outline in docs/option-decorators.md |  |  | 0.615 |
| ns | 4548 |  | 203 | Command.__init__ signature | 4.1 |  | 0.604 |
| walker |  | 4557 | 31 | docs/option-decorators.md section #3 |  |  | 0.604 |
| walker |  | 4647 | 90 | docs/option-decorators.md section #0 |  |  | 0.604 |
| walker |  | 4681 | 34 | python imports in src/click/_utils.py |  |  | 0.604 |
| walker |  | 4778 | 97 | python decl names surface in src/click/testing.py |  |  | 0.604 |
| walker |  | 4778 | 0 | python decl at src/click/testing.py:26 |  |  | 0.604 |
| walker |  | 4778 | 0 | python decl at src/click/testing.py:70 |  |  | 0.604 |
| walker |  | 4778 | 0 | python decl at src/click/testing.py:89 |  |  | 0.604 |
| walker |  | 4778 | 0 | python decl at src/click/testing.py:103 |  |  | 0.604 |
| walker |  | 4778 | 0 | python decl at src/click/testing.py:183 |  |  | 0.604 |
| walker |  | 4778 | 0 | python decl at src/click/testing.py:261 |  |  | 0.604 |
| walker |  | 4823 | 45 | python decl at src/click/testing.py:163 |  |  | 0.604 |
| walker |  | 4847 | 24 | python decl at src/click/testing.py:60 |  |  | 0.604 |
| walker |  | 4894 | 47 | python decl doc at src/click/testing.py:70 |  |  | 0.604 |
| walker |  | 4957 | 63 | python decl doc at src/click/testing.py:89 |  |  | 0.604 |
| ns | 5021 |  | 473 | Command — public method locations | 4.2 |  | 0.589 |
| ns | 5377 |  | 356 | Group.__init__ signature + defaulting rules | 5.1 |  | 0.570 |
| walker |  | 5422 | 465 | python method sigs in src/click/testing.py |  |  | 0.571 |
| walker |  | 5422 | 0 | python method at src/click/testing.py:27 |  |  | 0.571 |
| walker |  | 5422 | 0 | python method at src/click/testing.py:32 |  |  | 0.571 |
| walker |  | 5422 | 0 | python method at src/click/testing.py:35 |  |  | 0.571 |
| walker |  | 5422 | 0 | python method at src/click/testing.py:41 |  |  | 0.571 |
| walker |  | 5422 | 0 | python method at src/click/testing.py:44 |  |  | 0.571 |
| walker |  | 5422 | 0 | python method at src/click/testing.py:47 |  |  | 0.571 |
| walker |  | 5422 | 0 | python method at src/click/testing.py:50 |  |  | 0.571 |
| walker |  | 5422 | 0 | python method at src/click/testing.py:53 |  |  | 0.571 |
| walker |  | 5422 | 0 | python method at src/click/testing.py:56 |  |  | 0.571 |
| walker |  | 5422 | 0 | python method at src/click/testing.py:76 |  |  | 0.571 |
| walker |  | 5422 | 0 | python method at src/click/testing.py:80 |  |  | 0.571 |
| walker |  | 5422 | 0 | python method at src/click/testing.py:84 |  |  | 0.571 |
| walker |  | 5422 | 0 | python method at src/click/testing.py:97 |  |  | 0.571 |
| walker |  | 5422 | 0 | python method at src/click/testing.py:131 |  |  | 0.571 |
| walker |  | 5422 | 0 | python method at src/click/testing.py:139 |  |  | 0.571 |
| walker |  | 5422 | 0 | python method at src/click/testing.py:256 |  |  | 0.571 |
| walker |  | 5422 | 0 | python method at src/click/testing.py:295 |  |  | 0.571 |
| walker |  | 5430 | 8 | python method at src/click/testing.py:154 |  |  | 0.571 |
| walker |  | 5438 | 8 | python method at src/click/testing.py:158 |  |  | 0.571 |
| walker |  | 5446 | 8 | python method at src/click/testing.py:226 |  |  | 0.571 |
| walker |  | 5454 | 8 | python method at src/click/testing.py:238 |  |  | 0.571 |
| walker |  | 5462 | 8 | python method at src/click/testing.py:245 |  |  | 0.571 |
| walker |  | 5475 | 13 | python method doc at src/click/testing.py:238 |  |  | 0.571 |
| walker |  | 5514 | 39 | python method at src/click/testing.py:302 |  |  | 0.571 |
| walker |  | 5529 | 15 | python method doc at src/click/testing.py:302 |  |  | 0.571 |
| walker |  | 5574 | 45 | python method at src/click/testing.py:638 |  |  | 0.571 |
| walker |  | 5619 | 45 | headings outline in docs/handling-files.md |  |  | 0.571 |
| walker |  | 5696 | 77 | docs/handling-files.md section #0 |  |  | 0.571 |
| walker |  | 5732 | 36 | python imports in src/click/globals.py |  |  | 0.571 |
| ns | 5778 |  | 401 | Group + CommandCollection — public method locations | 5.2 |  | 0.559 |
| walker |  | 5807 | 75 | python method at src/click/testing.py:117 |  |  | 0.559 |
| walker |  | 5884 | 77 | python method at src/click/testing.py:283 |  |  | 0.559 |
| walker |  | 5937 | 53 | headings outline in docs/arguments.md |  |  | 0.559 |
| ns | 5941 |  | 163 | Group.invoke — non-chain dispatch | 5.3 | 5.2 | 0.552 |
| walker |  | 6094 | 157 | docs/arguments.md section #0 |  |  | 0.552 |
| walker |  | 6110 | 16 | python decl names surface in src/click/_textwrap.py |  |  | 0.552 |
| walker |  | 6110 | 0 | python decl at src/click/_textwrap.py:8 |  |  | 0.552 |
| walker |  | 6163 | 53 | python method sigs in src/click/_textwrap.py |  |  | 0.552 |
| walker |  | 6163 | 0 | python method at src/click/_textwrap.py:40 |  |  | 0.552 |
| walker |  | 6172 | 9 | python method at src/click/_textwrap.py:27 |  |  | 0.552 |
| walker |  | 6227 | 55 | headings outline in docs/support-multiple-versions.md |  |  | 0.552 |
| ns | 6234 |  | 293 | Parameter.__init__ signature | 6.1 |  | 0.541 |
| walker |  | 6357 | 130 | docs/support-multiple-versions.md section #0 |  |  | 0.541 |
| walker |  | 6517 | 160 | python decl names surface in src/click/_termui_impl.py |  |  | 0.541 |
| walker |  | 6517 | 0 | python decl at src/click/_termui_impl.py:43 |  |  | 0.541 |
| walker |  | 6517 | 0 | python decl at src/click/_termui_impl.py:375 |  |  | 0.541 |
| walker |  | 6517 | 0 | python decl at src/click/_termui_impl.py:608 |  |  | 0.541 |
| walker |  | 6517 | 0 | python decl at src/click/_termui_impl.py:724 |  |  | 0.541 |
| walker |  | 6517 | 0 | python decl at src/click/_termui_impl.py:794 |  |  | 0.541 |
| walker |  | 6546 | 29 | python decl at src/click/_termui_impl.py:417 |  |  | 0.541 |
| walker |  | 6582 | 36 | python decl at src/click/_termui_impl.py:386 |  |  | 0.541 |
| walker |  | 6599 | 17 | python decl doc at src/click/_termui_impl.py:386 |  |  | 0.541 |
| ns | 6617 |  | 383 | Parameter — public method locations | 6.2 |  | 0.531 |
| ns | 6982 |  | 365 | Option + Argument — public method locations | 6.3 |  | 0.523 |
| walker |  | 7024 | 425 | python method sigs in src/click/_termui_impl.py |  |  | 0.523 |
| walker |  | 7024 | 0 | python method at src/click/_termui_impl.py:115 |  |  | 0.523 |
| walker |  | 7024 | 0 | python method at src/click/_termui_impl.py:128 |  |  | 0.523 |
| walker |  | 7024 | 0 | python method at src/click/_termui_impl.py:142 |  |  | 0.523 |
| walker |  | 7024 | 0 | python method at src/click/_termui_impl.py:166 |  |  | 0.523 |
| walker |  | 7024 | 0 | python method at src/click/_termui_impl.py:187 |  |  | 0.523 |
| walker |  | 7024 | 0 | python method at src/click/_termui_impl.py:193 |  |  | 0.523 |
| walker |  | 7024 | 0 | python method at src/click/_termui_impl.py:196 |  |  | 0.523 |
| walker |  | 7024 | 0 | python method at src/click/_termui_impl.py:215 |  |  | 0.523 |
| walker |  | 7024 | 0 | python method at src/click/_termui_impl.py:242 |  |  | 0.523 |
| walker |  | 7024 | 0 | python method at src/click/_termui_impl.py:288 |  |  | 0.523 |
| walker |  | 7024 | 0 | python method at src/click/_termui_impl.py:310 |  |  | 0.523 |
| walker |  | 7024 | 0 | python method at src/click/_termui_impl.py:336 |  |  | 0.523 |
| walker |  | 7024 | 0 | python method at src/click/_termui_impl.py:341 |  |  | 0.523 |
| walker |  | 7024 | 0 | python method at src/click/_termui_impl.py:376 |  |  | 0.523 |
| walker |  | 7024 | 0 | python method at src/click/_termui_impl.py:380 |  |  | 0.523 |
| walker |  | 7024 | 0 | python method at src/click/_termui_impl.py:621 |  |  | 0.523 |
| walker |  | 7024 | 0 | python method at src/click/_termui_impl.py:638 |  |  | 0.523 |
| walker |  | 7024 | 0 | python method at src/click/_termui_impl.py:676 |  |  | 0.523 |
| walker |  | 7032 | 8 | python method at src/click/_termui_impl.py:148 |  |  | 0.523 |
| walker |  | 7040 | 8 | python method at src/click/_termui_impl.py:154 |  |  | 0.523 |
| walker |  | 7048 | 8 | python method at src/click/_termui_impl.py:160 |  |  | 0.523 |
| walker |  | 7061 | 13 | python method doc at src/click/_termui_impl.py:638 |  |  | 0.523 |
| walker |  | 7092 | 31 | python method at src/click/_termui_impl.py:673 |  |  | 0.523 |
| walker |  | 7092 | 0 | python method body at src/click/_termui_impl.py:673 body 674 |  |  | 0.523 |
| walker |  | 7126 | 34 | python method at src/click/_termui_impl.py:668 |  |  | 0.523 |
| walker |  | 7126 | 0 | python method body at src/click/_termui_impl.py:668 body 669 |  |  | 0.523 |
| walker |  | 7184 | 58 | python method at src/click/_termui_impl.py:120 |  |  | 0.523 |
| ns | 7213 |  | 231 | decorators.py — public function locations | 7.1 |  | 0.517 |
| walker |  | 7230 | 46 | python decl at src/click/_termui_impl.py:597 |  |  | 0.517 |
| walker |  | 7249 | 19 | python decl doc at src/click/_termui_impl.py:597 |  |  | 0.517 |
| walker |  | 7321 | 72 | python method at src/click/_termui_impl.py:609 |  |  | 0.517 |
| walker |  | 7362 | 41 | python class body at src/click/_termui_impl.py:608 |  |  | 0.517 |
| walker |  | 7412 | 50 | python decl at src/click/_termui_impl.py:442 |  |  | 0.517 |
| walker |  | 7494 | 82 | python decl doc at src/click/_termui_impl.py:417 |  |  | 0.517 |
| ns | 7507 |  | 294 | types.py — class locations | 8.1 |  | 0.509 |
| walker |  | 7545 | 51 | python decl at src/click/_termui_impl.py:546 |  |  | 0.509 |
| walker |  | 7596 | 51 | python method doc at src/click/_termui_impl.py:341 |  |  | 0.509 |
| walker |  | 7641 | 45 | python imports in src/click/_textwrap.py |  |  | 0.509 |
| ns | 7714 |  | 207 | ParamType.convert() contract | 8.2 | 8.1 | 0.502 |
| ns | 7818 |  | 104 | convert_type() dispatch (tail) | 8.3 | 8.1 | 0.497 |
| walker |  | 7852 | 211 | python decl names surface in src/click/exceptions.py |  |  | 0.498 |
| walker |  | 7852 | 0 | python decl at src/click/exceptions.py:19 |  |  | 0.498 |
| walker |  | 7852 | 0 | python decl at src/click/exceptions.py:26 |  |  | 0.498 |
| walker |  | 7852 | 0 | python decl at src/click/exceptions.py:35 |  |  | 0.498 |
| walker |  | 7852 | 0 | python decl at src/click/exceptions.py:65 |  |  | 0.498 |
| walker |  | 7852 | 0 | python decl at src/click/exceptions.py:108 |  |  | 0.498 |
| walker |  | 7852 | 0 | python decl at src/click/exceptions.py:150 |  |  | 0.498 |
| walker |  | 7852 | 0 | python decl at src/click/exceptions.py:221 |  |  | 0.498 |
| walker |  | 7852 | 0 | python decl at src/click/exceptions.py:251 |  |  | 0.498 |
| walker |  | 7852 | 0 | python decl at src/click/exceptions.py:278 |  |  | 0.498 |
| walker |  | 7852 | 0 | python decl at src/click/exceptions.py:295 |  |  | 0.498 |
| walker |  | 7852 | 0 | python decl at src/click/exceptions.py:304 |  |  | 0.498 |
| walker |  | 7852 | 0 | python decl at src/click/exceptions.py:313 |  |  | 0.498 |
| walker |  | 7852 | 0 | python decl at src/click/exceptions.py:330 |  |  | 0.498 |
| walker |  | 7852 | 0 | python decl at src/click/exceptions.py:334 |  |  | 0.498 |
| ns | 7862 |  | 44 | parser.py — internal class locations | 9.1 |  | 0.496 |
| walker |  | 7866 | 14 | python decl doc at src/click/exceptions.py:313 |  |  | 0.496 |
| walker |  | 7880 | 14 | python decl doc at src/click/exceptions.py:330 |  |  | 0.496 |
| walker |  | 7898 | 18 | python decl doc at src/click/exceptions.py:35 |  |  | 0.496 |
| walker |  | 7917 | 19 | python decl doc at src/click/exceptions.py:251 |  |  | 0.496 |
| walker |  | 7961 | 44 | python decl doc at src/click/exceptions.py:221 |  |  | 0.496 |
| walker |  | 8014 | 53 | python decl doc at src/click/exceptions.py:334 |  |  | 0.496 |
| ns | 8021 |  | 159 | exceptions.py — class locations | 9.2 |  | 0.504 |
| walker |  | 8038 | 24 | python class body at src/click/exceptions.py:35 |  |  | 0.504 |
| walker |  | 8051 | 13 | python class body at src/click/exceptions.py:65 |  |  | 0.504 |
| ns | 8312 |  | 291 | formatting.py — function/class locations | 10.1 |  | 0.497 |
| walker |  | 8413 | 362 | python method sigs in src/click/exceptions.py |  |  | 0.497 |
| walker |  | 8413 | 0 | python method at src/click/exceptions.py:41 |  |  | 0.497 |
| walker |  | 8413 | 0 | python method at src/click/exceptions.py:48 |  |  | 0.497 |
| walker |  | 8413 | 0 | python method at src/click/exceptions.py:51 |  |  | 0.497 |
| walker |  | 8413 | 0 | python method at src/click/exceptions.py:54 |  |  | 0.497 |
| walker |  | 8413 | 0 | python method at src/click/exceptions.py:76 |  |  | 0.497 |
| walker |  | 8413 | 0 | python method at src/click/exceptions.py:81 |  |  | 0.497 |
| walker |  | 8413 | 0 | python method at src/click/exceptions.py:137 |  |  | 0.497 |
| walker |  | 8413 | 0 | python method at src/click/exceptions.py:173 |  |  | 0.497 |
| walker |  | 8413 | 0 | python method at src/click/exceptions.py:213 |  |  | 0.497 |
| walker |  | 8413 | 0 | python method at src/click/exceptions.py:245 |  |  | 0.497 |
| walker |  | 8413 | 0 | python method at src/click/exceptions.py:272 |  |  | 0.497 |
| walker |  | 8413 | 0 | python method at src/click/exceptions.py:305 |  |  | 0.497 |
| walker |  | 8413 | 0 | python method at src/click/exceptions.py:309 |  |  | 0.497 |
| walker |  | 8413 | 0 | python method at src/click/exceptions.py:316 |  |  | 0.497 |
| walker |  | 8413 | 0 | python method at src/click/exceptions.py:324 |  |  | 0.497 |
| walker |  | 8413 | 0 | python method at src/click/exceptions.py:343 |  |  | 0.497 |
| walker |  | 8446 | 33 | python method at src/click/exceptions.py:288 |  |  | 0.497 |
| walker |  | 8458 | 12 | python class body at src/click/exceptions.py:334 |  |  | 0.497 |
| ns | 8526 |  | 214 | termui.py — public function locations | 10.2 |  | 0.491 |
| walker |  | 8533 | 75 | python decl doc at src/click/exceptions.py:295 |  |  | 0.491 |
| walker |  | 8603 | 70 | python method at src/click/exceptions.py:227 |  |  | 0.491 |
| ns | 8624 |  | 98 | _termui_impl.py — top-level locations | 10.3 |  | 0.495 |
| walker |  | 8673 | 70 | python method at src/click/exceptions.py:254 |  |  | 0.495 |
| walker |  | 8744 | 71 | python method at src/click/exceptions.py:126 |  |  | 0.495 |
| ns | 8832 |  | 208 | utils.py — public function/class locations | 11.1 |  | 0.490 |
| walker |  | 8833 | 89 | python decl doc at src/click/exceptions.py:65 |  |  | 0.490 |
| walker |  | 8932 | 99 | python decl doc at src/click/exceptions.py:278 |  |  | 0.490 |
| walker |  | 9021 | 89 | python method at src/click/exceptions.py:162 |  |  | 0.490 |
| ns | 9104 |  | 272 | _compat.py: platform flags + '-' stream handling | 11.2 |  | 0.483 |
| walker |  | 9160 | 139 | python decl names surface in src/click/parser.py |  |  | 0.488 |
| walker |  | 9160 | 0 | python decl at src/click/parser.py:111 |  |  | 0.488 |
| walker |  | 9160 | 0 | python decl at src/click/parser.py:120 |  |  | 0.488 |
| walker |  | 9160 | 0 | python decl at src/click/parser.py:127 |  |  | 0.488 |
| walker |  | 9160 | 0 | python decl at src/click/parser.py:185 |  |  | 0.488 |
| walker |  | 9160 | 0 | python decl at src/click/parser.py:216 |  |  | 0.488 |
| walker |  | 9160 | 0 | python decl at src/click/parser.py:224 |  |  | 0.488 |
| walker |  | 9160 | 0 | python decl at src/click/parser.py:503 |  |  | 0.488 |
| ns | 9253 |  | 149 | testing.py — class/method locations | 12.1 |  | 0.495 |
| ns | 9376 |  | 123 | CliRunner.invoke() signature | 12.2 | 12.1 | 0.492 |
| walker |  | 9475 | 315 | python method sigs in src/click/parser.py |  |  | 0.492 |
| walker |  | 9475 | 0 | python method at src/click/parser.py:169 |  |  | 0.492 |
| walker |  | 9475 | 0 | python method at src/click/parser.py:186 |  |  | 0.492 |
| walker |  | 9475 | 0 | python method at src/click/parser.py:217 |  |  | 0.492 |
| walker |  | 9475 | 0 | python method at src/click/parser.py:290 |  |  | 0.492 |
| walker |  | 9475 | 0 | python method at src/click/parser.py:316 |  |  | 0.492 |
| walker |  | 9475 | 0 | python method at src/click/parser.py:327 |  |  | 0.492 |
| walker |  | 9475 | 0 | python method at src/click/parser.py:390 |  |  | 0.492 |
| walker |  | 9475 | 0 | python method at src/click/parser.py:470 |  |  | 0.492 |
| walker |  | 9483 | 8 | python method at src/click/parser.py:165 |  |  | 0.492 |
| ns | 9500 |  | 124 | shell_completion.py — public locations | 12.3 |  | 0.488 |
| walker |  | 9518 | 35 | python method at src/click/parser.py:298 |  |  | 0.488 |
| walker |  | 9556 | 38 | python method at src/click/parser.py:241 |  |  | 0.488 |
| walker |  | 9606 | 50 | python method at src/click/parser.py:191 |  |  | 0.488 |
| walker |  | 9619 | 13 | python method body at src/click/parser.py:165 body 167 |  |  | 0.488 |
| walker |  | 9668 | 49 | python decl at src/click/parser.py:51 |  |  | 0.488 |
| walker |  | 9701 | 33 | python method at src/click/parser.py:363 |  |  | 0.488 |
| walker |  | 9786 | 85 | python method at src/click/parser.py:128 |  |  | 0.488 |
| walker |  | 9874 | 88 | python method at src/click/parser.py:265 |  |  | 0.488 |
| walker |  | 9965 | 91 | python method at src/click/_termui_impl.py:134 |  |  | 0.488 |
| ns | 9985 |  | 485 | docs/options.md — default/flag_value interaction table | 13.1 |  | 0.478 |

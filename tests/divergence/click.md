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
| walker |  | 211 | 10 | listing of '.devcontainer' |  |  | 1.000 |
| walker |  | 225 | 14 | listing of '.github' |  |  | 1.000 |
| ns | 242 |  | 105 | pyproject.toml package identity | 1.3 |  | 0.817 |
| walker |  | 247 | 22 | listing of '.github/workflows' |  |  | 0.820 |
| ns | 271 |  | 29 | CHANGES.rst range markers | 1.4 |  | 0.787 |
| walker |  | 332 | 85 | listing of 'src/click' |  |  | 0.816 |
| walker |  | 347 | 15 | python imports #1 in src/click/__init__.py |  |  | 0.816 |
| ns | 389 |  | 118 | docs/ sitemap (index.rst toctree, Documentation + Tutorials) | 1.5 |  | 0.631 |
| walker |  | 418 | 71 | python imports in src/click/__init__.py |  |  | 0.632 |
| walker |  | 436 | 18 | python imports #6 in src/click/__init__.py |  |  | 0.632 |
| walker |  | 466 | 30 | python imports #5 in src/click/__init__.py |  |  | 0.632 |
| ns | 473 |  | 84 | docs/ sitemap (index.rst toctree, How to Guides) | 1.6 | 1.5 | 0.562 |
| ns | 559 |  | 86 | src/click/ module listing | 1.7 |  | 0.649 |
| walker |  | 566 | 100 | python imports #2 in src/click/__init__.py |  |  | 0.653 |
| walker |  | 584 | 18 | python decl names surface in src/click/__init__.py |  |  | 0.653 |
| walker |  | 584 | 0 | python decl at src/click/__init__.py:77 |  |  | 0.653 |
| walker |  | 745 | 161 | python imports #3 in src/click/__init__.py |  |  | 0.655 |
| ns | 766 |  | 207 | docs/ + docs/_static/ listing | 1.8 |  | 0.502 |
| walker |  | 889 | 144 | python imports #4 in src/click/__init__.py |  |  | 0.506 |
| ns | 949 |  | 183 | tests/ + tests/typing/ listing | 1.9 |  | 0.439 |
| walker |  | 977 | 88 | python imports #9 in src/click/__init__.py |  |  | 0.440 |
| walker |  | 1076 | 99 | README.md section #0 |  |  | 0.440 |
| ns | 1176 |  | 227 | examples/ tree listing | 1.10 |  | 0.370 |
| ns | 1237 |  | 61 | .github/ + .devcontainer/ listing | 1.11 |  | 0.390 |
| walker |  | 1265 | 189 | python imports #7 in src/click/__init__.py |  |  | 0.393 |
| ns | 1334 |  | 97 | pyproject.toml: pytest invocation config | 1.12 |  | 0.384 |
| walker |  | 1457 | 192 | listing of 'docs' |  |  | 0.534 |
| walker |  | 1471 | 14 | listing of 'docs/_static' |  |  | 0.556 |
| ns | 1530 |  | 196 | click/__init__.py: docstring + core exports | 2.1 |  | 0.576 |
| walker |  | 1651 | 180 | python imports #8 in src/click/__init__.py |  |  | 0.579 |
| walker |  | 1729 | 78 | [package] in pyproject.toml |  |  | 0.597 |
| walker |  | 1767 | 38 | package metadata in pyproject.toml |  |  | 0.621 |
| walker |  | 1812 | 45 | listing of 'examples' |  |  | 0.628 |
| ns | 1837 |  | 307 | click/__init__.py: decorators + exceptions exports | 2.2 |  | 0.651 |
| walker |  | 1939 | 127 | listing of 'tests' |  |  | 0.698 |
| walker |  | 1955 | 16 | python decl names surface in src/click/_textwrap.py |  |  | 0.698 |
| walker |  | 1955 | 0 | python decl at src/click/_textwrap.py:8 |  |  | 0.698 |
| ns | 2072 |  | 235 | click/__init__.py: formatting/globals/termui exports | 2.3 |  | 0.709 |
| walker |  | 2089 | 134 | manifest config in pyproject.toml |  |  | 0.709 |
| walker |  | 2118 | 29 | docs/setuptools.md section #0 |  |  | 0.709 |
| ns | 2254 |  | 182 | click/__init__.py: parameter-type exports | 2.4 |  | 0.717 |
| walker |  | 2281 | 163 | tool.mypy+pyright config in pyproject.toml |  |  | 0.717 |
| ns | 2342 |  | 88 | click/__init__.py: utils exports | 2.5 |  | 0.720 |
| walker |  | 2356 | 75 | README.md section #3 |  |  | 0.720 |
| walker |  | 2446 | 90 | README.md section #2 |  |  | 0.720 |
| walker |  | 2656 | 210 | tool.ruff config in pyproject.toml |  |  | 0.720 |
| walker |  | 2709 | 53 | python method sigs in src/click/_textwrap.py |  |  | 0.720 |
| walker |  | 2709 | 0 | python method at src/click/_textwrap.py:40 |  |  | 0.720 |
| walker |  | 2718 | 9 | python method at src/click/_textwrap.py:27 |  |  | 0.720 |
| ns | 2761 |  | 419 | _utils.py: UNSET / FLAG_NEEDS_VALUE sentinels | 2.6 |  | 0.674 |
| walker |  | 2771 | 53 | docs/license.md section #0 |  |  | 0.674 |
| walker |  | 2826 | 55 | listing of 'tests/typing' |  |  | 0.709 |
| walker |  | 2847 | 21 | headings outline in docs/click-concepts.md |  |  | 0.709 |
| walker |  | 2868 | 21 | headings outline in docs/unicode-support.md |  |  | 0.709 |
| walker |  | 2890 | 22 | headings outline in docs/command-line-reference.md |  |  | 0.709 |
| walker |  | 2912 | 22 | headings outline in docs/design-opinions.md |  |  | 0.709 |
| walker |  | 2980 | 68 | docs/design-opinions.md section #0 |  |  | 0.709 |
| walker |  | 3004 | 24 | headings outline in docs/contrib.md |  |  | 0.709 |
| ns | 3020 |  | 259 | globals.py: context-stack push/pop | 2.7 |  | 0.681 |
| ns | 3160 |  | 140 | ParameterSource enum members | 3.1 |  | 0.670 |
| walker |  | 3312 | 308 | dev/build/target dependencies in pyproject.toml |  |  | 0.670 |
| ns | 3431 |  | 271 | Context.__init__ signature | 3.2 |  | 0.651 |
| walker |  | 3679 | 367 | tool.flit+uv+pytest+coverage config in pyproject.toml |  |  | 0.668 |
| walker |  | 3757 | 78 | docs/click-concepts.md section #0 |  |  | 0.668 |
| walker |  | 3838 | 81 | docs/command-line-reference.md section #0 |  |  | 0.668 |
| walker |  | 3868 | 30 | headings outline in docs/entry-points.md |  |  | 0.668 |
| walker |  | 3900 | 32 | headings outline in docs/extending-click.md |  |  | 0.668 |
| ns | 3901 |  | 470 | Context — public method locations | 3.3 |  | 0.645 |
| walker |  | 3933 | 33 | headings outline in docs/wincmd.md |  |  | 0.645 |
| walker |  | 4020 | 87 | docs/wincmd.md section #0 |  |  | 0.645 |
| walker |  | 4240 | 220 | README.md section #1 |  |  | 0.645 |
| walker |  | 4276 | 36 | headings outline in docs/faqs.md |  |  | 0.645 |
| walker |  | 4312 | 36 | docs/faqs.md section #0 |  |  | 0.645 |
| ns | 4345 |  | 444 | Context.invoke — Command branch | 3.4 | 3.3 | 0.615 |
| walker |  | 4351 | 39 | headings outline in docs/parameters.md |  |  | 0.615 |
| walker |  | 4398 | 47 | docs/parameters.md section #0 |  |  | 0.615 |
| walker |  | 4438 | 40 | headings outline in docs/virtualenv.md |  |  | 0.615 |
| walker |  | 4438 | 0 | docs/virtualenv.md section #0 |  |  | 0.615 |
| ns | 4548 |  | 203 | Command.__init__ signature | 4.1 |  | 0.604 |
| walker |  | 4562 | 124 | docs/extending-click.md section #0 |  |  | 0.604 |
| walker |  | 4604 | 42 | headings outline in docs/option-decorators.md |  |  | 0.604 |
| walker |  | 4635 | 31 | docs/option-decorators.md section #3 |  |  | 0.604 |
| walker |  | 4725 | 90 | docs/option-decorators.md section #0 |  |  | 0.604 |
| walker |  | 4759 | 34 | python imports in src/click/_utils.py |  |  | 0.604 |
| walker |  | 4856 | 97 | python decl names surface in src/click/testing.py |  |  | 0.604 |
| walker |  | 4856 | 0 | python decl at src/click/testing.py:26 |  |  | 0.604 |
| walker |  | 4856 | 0 | python decl at src/click/testing.py:70 |  |  | 0.604 |
| walker |  | 4856 | 0 | python decl at src/click/testing.py:89 |  |  | 0.604 |
| walker |  | 4856 | 0 | python decl at src/click/testing.py:103 |  |  | 0.604 |
| walker |  | 4856 | 0 | python decl at src/click/testing.py:183 |  |  | 0.604 |
| walker |  | 4856 | 0 | python decl at src/click/testing.py:261 |  |  | 0.604 |
| walker |  | 4901 | 45 | python decl at src/click/testing.py:163 |  |  | 0.604 |
| walker |  | 4925 | 24 | python decl at src/click/testing.py:60 |  |  | 0.604 |
| walker |  | 4972 | 47 | python decl doc at src/click/testing.py:70 |  |  | 0.604 |
| ns | 5021 |  | 473 | Command — public method locations | 4.2 |  | 0.589 |
| walker |  | 5035 | 63 | python decl doc at src/click/testing.py:89 |  |  | 0.589 |
| ns | 5377 |  | 356 | Group.__init__ signature + defaulting rules | 5.1 |  | 0.570 |
| walker |  | 5500 | 465 | python method sigs in src/click/testing.py |  |  | 0.571 |
| walker |  | 5500 | 0 | python method at src/click/testing.py:27 |  |  | 0.571 |
| walker |  | 5500 | 0 | python method at src/click/testing.py:32 |  |  | 0.571 |
| walker |  | 5500 | 0 | python method at src/click/testing.py:35 |  |  | 0.571 |
| walker |  | 5500 | 0 | python method at src/click/testing.py:41 |  |  | 0.571 |
| walker |  | 5500 | 0 | python method at src/click/testing.py:44 |  |  | 0.571 |
| walker |  | 5500 | 0 | python method at src/click/testing.py:47 |  |  | 0.571 |
| walker |  | 5500 | 0 | python method at src/click/testing.py:50 |  |  | 0.571 |
| walker |  | 5500 | 0 | python method at src/click/testing.py:53 |  |  | 0.571 |
| walker |  | 5500 | 0 | python method at src/click/testing.py:56 |  |  | 0.571 |
| walker |  | 5500 | 0 | python method at src/click/testing.py:76 |  |  | 0.571 |
| walker |  | 5500 | 0 | python method at src/click/testing.py:80 |  |  | 0.571 |
| walker |  | 5500 | 0 | python method at src/click/testing.py:84 |  |  | 0.571 |
| walker |  | 5500 | 0 | python method at src/click/testing.py:97 |  |  | 0.571 |
| walker |  | 5500 | 0 | python method at src/click/testing.py:131 |  |  | 0.571 |
| walker |  | 5500 | 0 | python method at src/click/testing.py:139 |  |  | 0.571 |
| walker |  | 5500 | 0 | python method at src/click/testing.py:256 |  |  | 0.571 |
| walker |  | 5500 | 0 | python method at src/click/testing.py:295 |  |  | 0.571 |
| walker |  | 5508 | 8 | python method at src/click/testing.py:154 |  |  | 0.571 |
| walker |  | 5516 | 8 | python method at src/click/testing.py:158 |  |  | 0.571 |
| walker |  | 5524 | 8 | python method at src/click/testing.py:226 |  |  | 0.571 |
| walker |  | 5532 | 8 | python method at src/click/testing.py:238 |  |  | 0.571 |
| walker |  | 5540 | 8 | python method at src/click/testing.py:245 |  |  | 0.571 |
| walker |  | 5553 | 13 | python method doc at src/click/testing.py:238 |  |  | 0.571 |
| walker |  | 5592 | 39 | python method at src/click/testing.py:302 |  |  | 0.571 |
| walker |  | 5607 | 15 | python method doc at src/click/testing.py:302 |  |  | 0.571 |
| walker |  | 5652 | 45 | python method at src/click/testing.py:638 |  |  | 0.571 |
| walker |  | 5697 | 45 | headings outline in docs/handling-files.md |  |  | 0.571 |
| walker |  | 5774 | 77 | docs/handling-files.md section #0 |  |  | 0.571 |
| ns | 5778 |  | 401 | Group + CommandCollection — public method locations | 5.2 |  | 0.559 |
| walker |  | 5810 | 36 | python imports in src/click/globals.py |  |  | 0.559 |
| walker |  | 5885 | 75 | python method at src/click/testing.py:117 |  |  | 0.559 |
| ns | 5941 |  | 163 | Group.invoke — non-chain dispatch | 5.3 | 5.2 | 0.552 |
| walker |  | 5962 | 77 | python method at src/click/testing.py:283 |  |  | 0.552 |
| walker |  | 6047 | 85 | python decl names surface in src/click/globals.py |  |  | 0.553 |
| walker |  | 6047 | 0 | python decl at src/click/globals.py:20 |  |  | 0.553 |
| walker |  | 6047 | 0 | python decl at src/click/globals.py:44 |  |  | 0.553 |
| walker |  | 6047 | 0 | python decl at src/click/globals.py:49 |  |  | 0.553 |
| walker |  | 6047 | 0 | python decl at src/click/globals.py:54 |  |  | 0.553 |
| walker |  | 6056 | 9 | python decl at src/click/globals.py:12 |  |  | 0.553 |
| walker |  | 6067 | 11 | python decl at src/click/globals.py:16 |  |  | 0.553 |
| walker |  | 6077 | 10 | python decl body at src/click/globals.py:49 body 51 |  |  | 0.553 |
| walker |  | 6090 | 13 | python decl doc at src/click/globals.py:49 |  |  | 0.554 |
| walker |  | 6106 | 16 | python decl doc at src/click/globals.py:44 |  |  | 0.555 |
| walker |  | 6123 | 17 | python decl body at src/click/globals.py:44 body 46 |  |  | 0.557 |
| walker |  | 6178 | 55 | python decl doc at src/click/globals.py:54 |  |  | 0.564 |
| walker |  | 6231 | 53 | headings outline in docs/arguments.md |  |  | 0.564 |
| ns | 6234 |  | 293 | Parameter.__init__ signature | 6.1 |  | 0.553 |
| walker |  | 6388 | 157 | docs/arguments.md section #0 |  |  | 0.553 |
| walker |  | 6406 | 18 | python decl body at src/click/globals.py:16 body 17 |  |  | 0.553 |
| walker |  | 6461 | 55 | headings outline in docs/support-multiple-versions.md |  |  | 0.553 |
| walker |  | 6591 | 130 | docs/support-multiple-versions.md section #0 |  |  | 0.553 |
| ns | 6617 |  | 383 | Parameter — public method locations | 6.2 |  | 0.543 |
| walker |  | 6751 | 160 | python decl names surface in src/click/_termui_impl.py |  |  | 0.543 |
| walker |  | 6751 | 0 | python decl at src/click/_termui_impl.py:43 |  |  | 0.543 |
| walker |  | 6751 | 0 | python decl at src/click/_termui_impl.py:375 |  |  | 0.543 |
| walker |  | 6751 | 0 | python decl at src/click/_termui_impl.py:608 |  |  | 0.543 |
| walker |  | 6751 | 0 | python decl at src/click/_termui_impl.py:724 |  |  | 0.543 |
| walker |  | 6751 | 0 | python decl at src/click/_termui_impl.py:794 |  |  | 0.543 |
| walker |  | 6780 | 29 | python decl at src/click/_termui_impl.py:417 |  |  | 0.543 |
| walker |  | 6825 | 45 | python class body at src/click/_termui_impl.py:608 |  |  | 0.543 |
| walker |  | 6861 | 36 | python decl at src/click/_termui_impl.py:386 |  |  | 0.543 |
| walker |  | 6878 | 17 | python decl doc at src/click/_termui_impl.py:386 |  |  | 0.543 |
| ns | 6982 |  | 365 | Option + Argument — public method locations | 6.3 |  | 0.534 |
| ns | 7213 |  | 231 | decorators.py — public function locations | 7.1 |  | 0.528 |
| walker |  | 7351 | 473 | python method sigs in src/click/_termui_impl.py |  |  | 0.528 |
| walker |  | 7351 | 0 | python method at src/click/_termui_impl.py:115 |  |  | 0.528 |
| walker |  | 7351 | 0 | python method at src/click/_termui_impl.py:128 |  |  | 0.528 |
| walker |  | 7351 | 0 | python method at src/click/_termui_impl.py:142 |  |  | 0.528 |
| walker |  | 7351 | 0 | python method at src/click/_termui_impl.py:166 |  |  | 0.528 |
| walker |  | 7351 | 0 | python method at src/click/_termui_impl.py:187 |  |  | 0.528 |
| walker |  | 7351 | 0 | python method at src/click/_termui_impl.py:193 |  |  | 0.528 |
| walker |  | 7351 | 0 | python method at src/click/_termui_impl.py:196 |  |  | 0.528 |
| walker |  | 7351 | 0 | python method at src/click/_termui_impl.py:215 |  |  | 0.528 |
| walker |  | 7351 | 0 | python method at src/click/_termui_impl.py:242 |  |  | 0.528 |
| walker |  | 7351 | 0 | python method at src/click/_termui_impl.py:288 |  |  | 0.528 |
| walker |  | 7351 | 0 | python method at src/click/_termui_impl.py:310 |  |  | 0.528 |
| walker |  | 7351 | 0 | python method at src/click/_termui_impl.py:336 |  |  | 0.528 |
| walker |  | 7351 | 0 | python method at src/click/_termui_impl.py:341 |  |  | 0.528 |
| walker |  | 7351 | 0 | python method at src/click/_termui_impl.py:376 |  |  | 0.528 |
| walker |  | 7351 | 0 | python method at src/click/_termui_impl.py:380 |  |  | 0.528 |
| walker |  | 7351 | 0 | python method at src/click/_termui_impl.py:621 |  |  | 0.528 |
| walker |  | 7351 | 0 | python method at src/click/_termui_impl.py:638 |  |  | 0.528 |
| walker |  | 7351 | 0 | python method at src/click/_termui_impl.py:676 |  |  | 0.528 |
| walker |  | 7359 | 8 | python method at src/click/_termui_impl.py:148 |  |  | 0.528 |
| walker |  | 7367 | 8 | python method at src/click/_termui_impl.py:154 |  |  | 0.528 |
| walker |  | 7375 | 8 | python method at src/click/_termui_impl.py:160 |  |  | 0.528 |
| walker |  | 7383 | 8 | python method at src/click/_termui_impl.py:673 |  |  | 0.528 |
| walker |  | 7383 | 0 | python method body at src/click/_termui_impl.py:673 body 674 |  |  | 0.528 |
| walker |  | 7393 | 10 | python method at src/click/_termui_impl.py:668 |  |  | 0.528 |
| walker |  | 7393 | 0 | python method body at src/click/_termui_impl.py:668 body 669 |  |  | 0.528 |
| walker |  | 7406 | 13 | python method doc at src/click/_termui_impl.py:638 |  |  | 0.528 |
| walker |  | 7464 | 58 | python method at src/click/_termui_impl.py:120 |  |  | 0.528 |
| ns | 7507 |  | 294 | types.py — class locations | 8.1 |  | 0.520 |
| walker |  | 7510 | 46 | python decl at src/click/_termui_impl.py:597 |  |  | 0.520 |
| walker |  | 7529 | 19 | python decl doc at src/click/_termui_impl.py:597 |  |  | 0.520 |
| walker |  | 7601 | 72 | python method at src/click/_termui_impl.py:609 |  |  | 0.520 |
| walker |  | 7651 | 50 | python decl at src/click/_termui_impl.py:442 |  |  | 0.520 |
| ns | 7714 |  | 207 | ParamType.convert() contract | 8.2 | 8.1 | 0.513 |
| walker |  | 7733 | 82 | python decl doc at src/click/_termui_impl.py:417 |  |  | 0.513 |
| walker |  | 7784 | 51 | python decl at src/click/_termui_impl.py:546 |  |  | 0.513 |
| ns | 7818 |  | 104 | convert_type() dispatch (tail) | 8.3 | 8.1 | 0.508 |
| walker |  | 7835 | 51 | python method doc at src/click/_termui_impl.py:341 |  |  | 0.508 |
| ns | 7862 |  | 44 | parser.py — internal class locations | 9.1 |  | 0.506 |
| walker |  | 7880 | 45 | python imports in src/click/_textwrap.py |  |  | 0.506 |
| ns | 8021 |  | 159 | exceptions.py — class locations | 9.2 |  | 0.501 |
| walker |  | 8091 | 211 | python decl names surface in src/click/exceptions.py |  |  | 0.515 |
| walker |  | 8091 | 0 | python decl at src/click/exceptions.py:19 |  |  | 0.515 |
| walker |  | 8091 | 0 | python decl at src/click/exceptions.py:26 |  |  | 0.515 |
| walker |  | 8091 | 0 | python decl at src/click/exceptions.py:35 |  |  | 0.515 |
| walker |  | 8091 | 0 | python decl at src/click/exceptions.py:65 |  |  | 0.515 |
| walker |  | 8091 | 0 | python decl at src/click/exceptions.py:108 |  |  | 0.515 |
| walker |  | 8091 | 0 | python decl at src/click/exceptions.py:150 |  |  | 0.515 |
| walker |  | 8091 | 0 | python decl at src/click/exceptions.py:221 |  |  | 0.515 |
| walker |  | 8091 | 0 | python decl at src/click/exceptions.py:251 |  |  | 0.515 |
| walker |  | 8091 | 0 | python decl at src/click/exceptions.py:278 |  |  | 0.515 |
| walker |  | 8091 | 0 | python decl at src/click/exceptions.py:295 |  |  | 0.515 |
| walker |  | 8091 | 0 | python decl at src/click/exceptions.py:304 |  |  | 0.515 |
| walker |  | 8091 | 0 | python decl at src/click/exceptions.py:313 |  |  | 0.515 |
| walker |  | 8091 | 0 | python decl at src/click/exceptions.py:330 |  |  | 0.515 |
| walker |  | 8091 | 0 | python decl at src/click/exceptions.py:334 |  |  | 0.515 |
| walker |  | 8104 | 13 | python class body at src/click/exceptions.py:65 |  |  | 0.515 |
| walker |  | 8120 | 16 | python class body at src/click/exceptions.py:334 |  |  | 0.515 |
| walker |  | 8134 | 14 | python decl doc at src/click/exceptions.py:313 |  |  | 0.515 |
| walker |  | 8148 | 14 | python decl doc at src/click/exceptions.py:330 |  |  | 0.515 |
| walker |  | 8166 | 18 | python decl doc at src/click/exceptions.py:35 |  |  | 0.515 |
| walker |  | 8190 | 24 | python class body at src/click/exceptions.py:35 |  |  | 0.515 |
| walker |  | 8209 | 19 | python decl doc at src/click/exceptions.py:251 |  |  | 0.515 |
| walker |  | 8253 | 44 | python decl doc at src/click/exceptions.py:221 |  |  | 0.515 |
| walker |  | 8304 | 51 | python decl doc at src/click/exceptions.py:334 |  |  | 0.515 |
| ns | 8312 |  | 291 | formatting.py — function/class locations | 10.1 |  | 0.508 |
| ns | 8526 |  | 214 | termui.py — public function locations | 10.2 |  | 0.501 |
| ns | 8624 |  | 98 | _termui_impl.py — top-level locations | 10.3 |  | 0.505 |
| walker |  | 8664 | 360 | python method sigs in src/click/exceptions.py |  |  | 0.505 |
| walker |  | 8664 | 0 | python method at src/click/exceptions.py:41 |  |  | 0.505 |
| walker |  | 8664 | 0 | python method at src/click/exceptions.py:48 |  |  | 0.505 |
| walker |  | 8664 | 0 | python method at src/click/exceptions.py:51 |  |  | 0.505 |
| walker |  | 8664 | 0 | python method at src/click/exceptions.py:54 |  |  | 0.505 |
| walker |  | 8664 | 0 | python method at src/click/exceptions.py:76 |  |  | 0.505 |
| walker |  | 8664 | 0 | python method at src/click/exceptions.py:81 |  |  | 0.505 |
| walker |  | 8664 | 0 | python method at src/click/exceptions.py:137 |  |  | 0.505 |
| walker |  | 8664 | 0 | python method at src/click/exceptions.py:173 |  |  | 0.505 |
| walker |  | 8664 | 0 | python method at src/click/exceptions.py:213 |  |  | 0.505 |
| walker |  | 8664 | 0 | python method at src/click/exceptions.py:245 |  |  | 0.505 |
| walker |  | 8664 | 0 | python method at src/click/exceptions.py:272 |  |  | 0.505 |
| walker |  | 8664 | 0 | python method at src/click/exceptions.py:305 |  |  | 0.505 |
| walker |  | 8664 | 0 | python method at src/click/exceptions.py:309 |  |  | 0.505 |
| walker |  | 8664 | 0 | python method at src/click/exceptions.py:316 |  |  | 0.505 |
| walker |  | 8664 | 0 | python method at src/click/exceptions.py:324 |  |  | 0.505 |
| walker |  | 8664 | 0 | python method at src/click/exceptions.py:343 |  |  | 0.505 |
| walker |  | 8697 | 33 | python method at src/click/exceptions.py:288 |  |  | 0.505 |
| walker |  | 8772 | 75 | python decl doc at src/click/exceptions.py:295 |  |  | 0.505 |
| ns | 8832 |  | 208 | utils.py — public function/class locations | 11.1 |  | 0.500 |
| walker |  | 8842 | 70 | python method at src/click/exceptions.py:227 |  |  | 0.500 |
| walker |  | 8912 | 70 | python method at src/click/exceptions.py:254 |  |  | 0.500 |
| walker |  | 8983 | 71 | python method at src/click/exceptions.py:126 |  |  | 0.500 |
| walker |  | 9072 | 89 | python decl doc at src/click/exceptions.py:65 |  |  | 0.500 |
| ns | 9104 |  | 272 | _compat.py: platform flags + '-' stream handling | 11.2 |  | 0.493 |
| walker |  | 9171 | 99 | python decl doc at src/click/exceptions.py:278 |  |  | 0.493 |
| ns | 9253 |  | 149 | testing.py — class/method locations | 12.1 |  | 0.501 |
| walker |  | 9260 | 89 | python method at src/click/exceptions.py:162 |  |  | 0.501 |
| ns | 9376 |  | 123 | CliRunner.invoke() signature | 12.2 | 12.1 | 0.497 |
| walker |  | 9399 | 139 | python decl names surface in src/click/parser.py |  |  | 0.501 |
| walker |  | 9399 | 0 | python decl at src/click/parser.py:111 |  |  | 0.501 |
| walker |  | 9399 | 0 | python decl at src/click/parser.py:120 |  |  | 0.501 |
| walker |  | 9399 | 0 | python decl at src/click/parser.py:127 |  |  | 0.501 |
| walker |  | 9399 | 0 | python decl at src/click/parser.py:185 |  |  | 0.501 |
| walker |  | 9399 | 0 | python decl at src/click/parser.py:216 |  |  | 0.501 |
| walker |  | 9399 | 0 | python decl at src/click/parser.py:224 |  |  | 0.501 |
| walker |  | 9399 | 0 | python decl at src/click/parser.py:503 |  |  | 0.501 |
| ns | 9500 |  | 124 | shell_completion.py — public locations | 12.3 |  | 0.498 |
| walker |  | 9714 | 315 | python method sigs in src/click/parser.py |  |  | 0.498 |
| walker |  | 9714 | 0 | python method at src/click/parser.py:169 |  |  | 0.498 |
| walker |  | 9714 | 0 | python method at src/click/parser.py:186 |  |  | 0.498 |
| walker |  | 9714 | 0 | python method at src/click/parser.py:217 |  |  | 0.498 |
| walker |  | 9714 | 0 | python method at src/click/parser.py:290 |  |  | 0.498 |
| walker |  | 9714 | 0 | python method at src/click/parser.py:316 |  |  | 0.498 |
| walker |  | 9714 | 0 | python method at src/click/parser.py:327 |  |  | 0.498 |
| walker |  | 9714 | 0 | python method at src/click/parser.py:390 |  |  | 0.498 |
| walker |  | 9714 | 0 | python method at src/click/parser.py:470 |  |  | 0.498 |
| walker |  | 9722 | 8 | python method at src/click/parser.py:165 |  |  | 0.498 |
| walker |  | 9757 | 35 | python method at src/click/parser.py:298 |  |  | 0.498 |
| walker |  | 9795 | 38 | python method at src/click/parser.py:241 |  |  | 0.498 |
| walker |  | 9845 | 50 | python method at src/click/parser.py:191 |  |  | 0.498 |
| walker |  | 9858 | 13 | python method body at src/click/parser.py:165 body 167 |  |  | 0.498 |
| walker |  | 9907 | 49 | python decl at src/click/parser.py:51 |  |  | 0.498 |
| walker |  | 9940 | 33 | python method at src/click/parser.py:363 |  |  | 0.498 |
| ns | 9985 |  | 485 | docs/options.md — default/flag_value interaction table | 13.1 |  | 0.487 |

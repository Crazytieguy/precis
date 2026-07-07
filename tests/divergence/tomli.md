Score(3000)=0.428 I=0.712 C=0.257 ns_rows≤3K=29/43 (reached=10 partial=1 missing=18)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 8 |  | 8 | fuzzer/ listing | 1.1 |  | 0.000 |
| ns | 17 |  | 9 | profiler/ listing | 1.2 |  | 0.000 |
| ns | 34 |  | 17 | scripts/ listing | 1.3 |  | 0.000 |
| ns | 60 |  | 26 | src/tomli/ listing | 1.4 |  | 0.000 |
| walker |  | 62 | 62 | listing of '.' |  |  | 0.000 |
| walker |  | 66 | 4 | listing of 'src' |  |  | 0.000 |
| ns | 80 |  | 20 | benchmark/ listing | 1.5 |  | 0.000 |
| ns | 110 |  | 30 | tests/ top-level listing | 1.6 |  | 0.000 |
| walker |  | 124 | 58 | headings outline in tomllib.md |  |  | 0.000 |
| ns | 143 |  | 33 | tests/data/invalid/dotted-keys/ listing | 1.7 |  | 0.000 |
| walker |  | 150 | 26 | listing of 'src/tomli' |  |  | 0.205 |
| ns | 199 |  | 56 | External toml-test invalid/ category listing | 1.8 |  | 0.156 |
| ns | 261 |  | 62 | Repo root listing | 1.9 |  | 0.272 |
| ns | 269 |  | 8 | MANIFEST.in (full, 1 line) | 1.10 |  | 0.269 |
| walker |  | 280 | 130 | python imports in src/tomli/__init__.py |  |  | 0.282 |
| walker |  | 283 | 3 | listing of '.github' |  |  | 0.282 |
| walker |  | 287 | 4 | listing of '.github/workflows' |  |  | 0.282 |
| ns | 323 |  | 54 | _parser.py roster: entry points + error class | 1.11 |  | 0.269 |
| ns | 401 |  | 78 | CI workflow job roster | 1.12 |  | 0.254 |
| ns | 448 |  | 47 | _re.py function roster | 1.13 |  | 0.246 |
| ns | 566 |  | 118 | pyproject.toml tox environment roster | 1.14 |  | 0.232 |
| walker |  | 680 | 393 | README headline in README.md |  |  | 0.232 |
| walker |  | 688 | 8 | listing of 'fuzzer' |  |  | 0.385 |
| ns | 696 |  | 130 | tomli/__init__.py (full public API) | 2.1 |  | 0.418 |
| walker |  | 697 | 9 | listing of 'profiler' |  |  | 0.478 |
| walker |  | 727 | 30 | listing of 'tests' |  |  | 0.540 |
| ns | 818 |  | 122 | _parser.py roster: table/array/key rules | 2.2 |  | 0.506 |
| walker |  | 832 | 105 | [package] in pyproject.toml |  |  | 0.507 |
| walker |  | 864 | 32 | python imports in setup.py |  |  | 0.507 |
| walker |  | 881 | 17 | listing of 'scripts' |  |  | 0.563 |
| walker |  | 901 | 20 | listing of 'benchmark' |  |  | 0.618 |
| ns | 945 |  | 127 | pyproject.toml mypy strict config | 2.3 |  | 0.580 |
| ns | 1083 |  | 138 | _parser.py roster: value parsing | 2.4 |  | 0.547 |
| ns | 1225 |  | 142 | _parser.py roster: namespace state machinery | 2.5 |  | 0.517 |
| walker |  | 1286 | 385 | headings outline in README.md |  |  | 0.517 |
| walker |  | 1295 | 9 | README.md section #0 |  |  | 0.517 |
| walker |  | 1301 | 6 | README.md section #19 |  |  | 0.517 |
| walker |  | 1326 | 25 | README.md section #2 |  |  | 0.517 |
| walker |  | 1334 | 8 | README.md section #13 |  |  | 0.517 |
| walker |  | 1345 | 11 | README.md section #14 |  |  | 0.517 |
| walker |  | 1359 | 14 | README.md section #18 |  |  | 0.517 |
| walker |  | 1393 | 34 | python decl names surface in src/tomli/_types.py |  |  | 0.518 |
| ns | 1396 |  | 171 | .pre-commit-config.yaml hook-id roster | 2.6 |  | 0.493 |
| walker |  | 1421 | 28 | README.md section #16 |  |  | 0.493 |
| ns | 1497 |  | 101 | tests/data/valid/ listing | 2.7 |  | 0.464 |
| walker |  | 1504 | 83 | README.md section #25 |  |  | 0.464 |
| walker |  | 1535 | 31 | README.md section #23 |  |  | 0.464 |
| walker |  | 1568 | 33 | README.md section #21 |  |  | 0.464 |
| ns | 1651 |  | 154 | tests/data/invalid/ listing | 2.8 |  | 0.431 |
| ns | 1676 |  | 25 | Sample invalid TOML fixture (dotted-keys/extend-defined-aot) | 2.9 |  | 0.427 |
| walker |  | 1743 | 175 | tomllib.md section #0 |  |  | 0.429 |
| walker |  | 1783 | 40 | README.md section #9 |  |  | 0.429 |
| ns | 1915 |  | 239 | Test method roster (test_data/test_error/test_misc) | 2.10 |  | 0.406 |
| ns | 2024 |  | 109 | _types.py (full) | 2.11 |  | 0.399 |
| ns | 2179 |  | 155 | Sample valid TOML/JSON fixture pair (array-subtables) | 3.1 |  | 0.381 |
| ns | 2287 |  | 108 | tests/__init__.py (tomllib aliasing) | 3.2 |  | 0.373 |
| ns | 2517 |  | 230 | External toml-test valid/ category listing | 3.3 |  | 0.343 |
| walker |  | 2689 | 906 | python decl names surface in src/tomli/_parser.py |  |  | 0.431 |
| walker |  | 2689 | 0 | python decl at src/tomli/_parser.py:71 |  |  | 0.431 |
| walker |  | 2689 | 0 | python decl at src/tomli/_parser.py:76 |  |  | 0.431 |
| walker |  | 2689 | 0 | python decl at src/tomli/_parser.py:137 |  |  | 0.431 |
| walker |  | 2689 | 0 | python decl at src/tomli/_parser.py:149 |  |  | 0.431 |
| walker |  | 2689 | 0 | python decl at src/tomli/_parser.py:220 |  |  | 0.431 |
| walker |  | 2689 | 0 | python decl at src/tomli/_parser.py:278 |  |  | 0.431 |
| walker |  | 2689 | 0 | python decl at src/tomli/_parser.py:312 |  |  | 0.431 |
| walker |  | 2689 | 0 | python decl at src/tomli/_parser.py:318 |  |  | 0.431 |
| walker |  | 2689 | 0 | python decl at src/tomli/_parser.py:349 |  |  | 0.431 |
| walker |  | 2689 | 0 | python decl at src/tomli/_parser.py:361 |  |  | 0.431 |
| walker |  | 2689 | 0 | python decl at src/tomli/_parser.py:370 |  |  | 0.431 |
| walker |  | 2689 | 0 | python decl at src/tomli/_parser.py:390 |  |  | 0.431 |
| walker |  | 2689 | 0 | python decl at src/tomli/_parser.py:463 |  |  | 0.431 |
| walker |  | 2689 | 0 | python decl at src/tomli/_parser.py:481 |  |  | 0.431 |
| walker |  | 2689 | 0 | python decl at src/tomli/_parser.py:497 |  |  | 0.431 |
| walker |  | 2689 | 0 | python decl at src/tomli/_parser.py:595 |  |  | 0.431 |
| walker |  | 2689 | 0 | python decl at src/tomli/_parser.py:599 |  |  | 0.431 |
| walker |  | 2689 | 0 | python decl at src/tomli/_parser.py:612 |  |  | 0.431 |
| walker |  | 2689 | 0 | python decl at src/tomli/_parser.py:621 |  |  | 0.431 |
| walker |  | 2689 | 0 | python decl at src/tomli/_parser.py:652 |  |  | 0.431 |
| walker |  | 2689 | 0 | python decl at src/tomli/_parser.py:760 |  |  | 0.431 |
| walker |  | 2689 | 0 | python decl at src/tomli/_parser.py:764 |  |  | 0.431 |
| walker |  | 2702 | 13 | python decl doc at src/tomli/_parser.py:149 |  |  | 0.431 |
| ns | 2711 |  | 194 | pyproject.toml build-system + core [project] identity | 3.4 |  | 0.428 |
| walker |  | 2718 | 16 | python decl doc at src/tomli/_parser.py:220 |  |  | 0.428 |
| walker |  | 2741 | 23 | python decl at src/tomli/_parser.py:51 |  |  | 0.428 |
| walker |  | 2773 | 32 | python decl at src/tomli/_parser.py:564 |  |  | 0.428 |
| walker |  | 2788 | 15 | python decl doc at src/tomli/_parser.py:137 |  |  | 0.428 |
| walker |  | 2823 | 35 | python decl at src/tomli/_parser.py:413 |  |  | 0.428 |
| walker |  | 2859 | 36 | python decl at src/tomli/_parser.py:684 |  |  | 0.428 |
| walker |  | 2896 | 37 | python decl at src/tomli/_parser.py:502 |  |  | 0.428 |
| walker |  | 2934 | 38 | python decl at src/tomli/_parser.py:447 |  |  | 0.428 |
| walker |  | 2973 | 39 | python decl at src/tomli/_parser.py:528 |  |  | 0.428 |
| walker |  | 2988 | 15 | python decl body at src/tomli/_parser.py:595 body 596 |  |  | 0.428 |
| walker |  | 3020 | 32 | python decl doc at src/tomli/_parser.py:71 |  |  | 0.428 |
| ns | 3036 |  | 325 | README intro | 3.5 |  | 0.414 |
| walker |  | 3044 | 24 | python decl body at src/tomli/_parser.py:497 body 498 |  |  | 0.414 |
| walker |  | 3110 | 66 | python decl at src/tomli/_parser.py:327 |  |  | 0.414 |
| walker |  | 3137 | 27 | python decl body at src/tomli/_parser.py:760 body 761 |  |  | 0.414 |
| walker |  | 3217 | 80 | python class body at src/tomli/_parser.py:220 |  |  | 0.414 |
| ns | 3257 |  | 221 | README usage: parse a TOML string | 4.1 |  | 0.401 |
| walker |  | 3420 | 203 | python method sigs in src/tomli/_parser.py |  |  | 0.441 |
| walker |  | 3420 | 0 | python method at src/tomli/_parser.py:229 |  |  | 0.441 |
| walker |  | 3420 | 0 | python method at src/tomli/_parser.py:233 |  |  | 0.441 |
| walker |  | 3420 | 0 | python method at src/tomli/_parser.py:236 |  |  | 0.441 |
| walker |  | 3420 | 0 | python method at src/tomli/_parser.py:241 |  |  | 0.441 |
| walker |  | 3420 | 0 | python method at src/tomli/_parser.py:249 |  |  | 0.441 |
| walker |  | 3420 | 0 | python method at src/tomli/_parser.py:260 |  |  | 0.441 |
| walker |  | 3420 | 0 | python method at src/tomli/_parser.py:300 |  |  | 0.441 |
| walker |  | 3420 | 0 | python method at src/tomli/_parser.py:313 |  |  | 0.441 |
| walker |  | 3435 | 15 | python method at src/tomli/_parser.py:279 |  |  | 0.441 |
| walker |  | 3448 | 13 | python method body at src/tomli/_parser.py:233 body 234 |  |  | 0.441 |
| walker |  | 3461 | 13 | python method body at src/tomli/_parser.py:279 body 281 |  |  | 0.441 |
| walker |  | 3507 | 46 | python method at src/tomli/_parser.py:283 |  |  | 0.441 |
| walker |  | 3526 | 19 | python method body at src/tomli/_parser.py:313 body 314 |  |  | 0.441 |
| ns | 3588 |  | 331 | tomllib.md - intro + sync status | 4.2 |  | 0.452 |
| walker |  | 3609 | 83 | python method at src/tomli/_parser.py:87 |  |  | 0.452 |
| walker |  | 3660 | 51 | python decl body at src/tomli/_parser.py:318 body 319 |  |  | 0.452 |
| walker |  | 3770 | 110 | python decl doc at src/tomli/_parser.py:76 |  |  | 0.453 |
| walker |  | 3865 | 95 | python decl doc at src/tomli/_parser.py:764 |  |  | 0.453 |
| walker |  | 3874 | 9 | README headline in benchmark/README.md |  |  | 0.453 |
| ns | 3885 |  | 297 | README: TOML->Python type mapping table | 4.3 |  | 0.441 |
| walker |  | 3913 | 39 | README.md section #22 |  |  | 0.441 |
| walker |  | 3983 | 70 | python decl body at src/tomli/_parser.py:361 body 362 |  |  | 0.441 |
| ns | 4024 |  | 139 | CHANGELOG.md - latest two releases | 4.4 |  | 0.432 |
| walker |  | 4032 | 49 | README.md section #17 |  |  | 0.432 |
| walker |  | 4070 | 38 | python method body at src/tomli/_parser.py:229 body 230 |  |  | 0.432 |
| walker |  | 4108 | 38 | python method body at src/tomli/_parser.py:236 body 237 |  |  | 0.432 |
| walker |  | 4164 | 56 | README.md section #5 |  |  | 0.432 |
| walker |  | 4220 | 56 | README.md section #8 |  |  | 0.432 |
| ns | 4256 |  | 232 | burntsushi.py: convert() (partial) | 4.5 |  | 0.420 |
| walker |  | 4278 | 58 | README.md section #4 |  |  | 0.420 |
| walker |  | 4434 | 156 | python decl at src/tomli/_parser.py:57 |  |  | 0.420 |
| walker |  | 4464 | 30 | tomllib.md section #3 |  |  | 0.420 |
| ns | 4555 |  | 299 | test_misc.py: test_parse_float | 4.6 | 2.10 | 0.405 |
| walker |  | 4765 | 301 | README.md section #1 |  |  | 0.439 |
| walker |  | 4855 | 90 | python decl body at src/tomli/_parser.py:612 body 613 |  |  | 0.439 |
| walker |  | 4887 | 32 | tomllib.md section #2 |  |  | 0.439 |
| walker |  | 4958 | 71 | README.md section #10 |  |  | 0.439 |
| ns | 5094 |  | 539 | test_data.py (full, data-driven glob mechanism) | 4.7 | 2.10 | 0.415 |
| walker |  | 5119 | 161 | python decl names surface in src/tomli/_re.py |  |  | 0.420 |
| walker |  | 5119 | 0 | python decl at src/tomli/_re.py:59 |  |  | 0.420 |
| walker |  | 5119 | 0 | python decl at src/tomli/_re.py:109 |  |  | 0.420 |
| walker |  | 5119 | 0 | python decl at src/tomli/_re.py:116 |  |  | 0.420 |
| walker |  | 5145 | 26 | python decl at src/tomli/_re.py:98 |  |  | 0.424 |
| walker |  | 5179 | 34 | python decl body at src/tomli/_re.py:116 body 117 |  |  | 0.424 |
| walker |  | 5241 | 62 | python decl doc at src/tomli/_re.py:59 |  |  | 0.424 |
| walker |  | 5341 | 100 | python decl body at src/tomli/_parser.py:349 body 350 |  |  | 0.424 |
| walker |  | 5351 | 10 | CHANGELOG.md section #0 |  |  | 0.424 |
| walker |  | 5422 | 71 | README.md section #20 |  |  | 0.424 |
| walker |  | 5495 | 73 | README.md section #15 |  |  | 0.424 |
| walker |  | 5559 | 64 | python imports in src/tomli/_types.py |  |  | 0.436 |
| walker |  | 5629 | 70 | python decl body at src/tomli/_re.py:98 body 100 |  |  | 0.436 |
| walker |  | 5693 | 64 | python method body at src/tomli/_parser.py:241 body 242 |  |  | 0.436 |
| walker |  | 5704 | 11 | python decl body at src/tomli/_parser.py:137 body 139 |  |  | 0.436 |
| ns | 5772 |  | 678 | Flags class (namespace mutability tracking) | 5.1 | 2.5 | 0.429 |
| walker |  | 5801 | 97 | README.md section #11 |  |  | 0.429 |
| walker |  | 5960 | 159 | python decl at src/tomli/_re.py:46 |  |  | 0.430 |
| walker |  | 6043 | 83 | python decl body at src/tomli/_re.py:109 body 110 |  |  | 0.430 |
| walker |  | 6144 | 101 | README.md section #7 |  |  | 0.430 |
| walker |  | 6301 | 157 | python imports in src/tomli/_parser.py |  |  | 0.430 |
| walker |  | 6352 | 51 | tomllib.md section #7 |  |  | 0.430 |
| ns | 6492 |  | 720 | TOMLDecodeError (+ deprecation sentinel) full body | 5.2 | 1.11 | 0.413 |
| walker |  | 6500 | 148 | python decl body at src/tomli/_parser.py:447 body 450 |  |  | 0.413 |
| walker |  | 6648 | 148 | python decl body at src/tomli/_parser.py:599 body 600 |  |  | 0.413 |
| walker |  | 6764 | 116 | README.md section #12 |  |  | 0.413 |
| walker |  | 6920 | 156 | python decl body at src/tomli/_parser.py:327 body 335 |  |  | 0.413 |
| walker |  | 6926 | 6 | listing of 'tests/data' |  |  | 0.413 |
| walker |  | 7029 | 103 | python imports in src/tomli/_re.py |  |  | 0.414 |
| walker |  | 7043 | 14 | python decl body at src/tomli/_parser.py:137 body 146 |  |  | 0.414 |
| ns | 7226 |  | 734 | _re.py regex definitions (RE_NUMBER/RE_LOCALTIME/RE_DATETIME) | 5.3 |  | 0.409 |
| ns | 8144 |  | 918 | parse_value() type dispatch | 5.4 | 2.4 | 0.385 |
| walker |  | 8855 | 1812 | manifest config in pyproject.toml |  |  | 0.421 |
| walker |  | 8972 | 117 | README.md section #6 |  |  | 0.421 |
| ns | 9105 |  | 961 | loads()/load() entry point + parse loop | 5.5 | 1.11 | 0.398 |
| walker |  | 9209 | 237 | python decl at src/tomli/_re.py:26 |  |  | 0.420 |
| walker |  | 9313 | 104 | python decl at src/tomli/_re.py:17 |  |  | 0.434 |
| walker |  | 9422 | 109 | python method body at src/tomli/_parser.py:300 body 301 |  |  | 0.434 |
| walker |  | 9498 | 76 | tomllib.md section #4 |  |  | 0.434 |
| walker |  | 9508 | 10 | python decl body at src/tomli/_parser.py:652 body 659 |  |  | 0.434 |
| walker |  | 9518 | 10 | python decl body at src/tomli/_parser.py:652 body 660 |  |  | 0.434 |
| walker |  | 9528 | 10 | python decl body at src/tomli/_parser.py:764 body 782 |  |  | 0.434 |
| walker |  | 9611 | 83 | tomllib.md section #5 |  |  | 0.434 |
| walker |  | 9733 | 122 | python method body at src/tomli/_parser.py:283 body 289 |  |  | 0.434 |
| walker |  | 9810 | 77 | python imports in tests/__init__.py |  |  | 0.439 |
| walker |  | 9982 | 172 | README.md section #3 |  |  | 0.461 |
| ns | 9987 |  | 882 | key_value_rule / parse_key_value_pair / parse_key (full bodies) | 5.6 | 2.2 | 0.445 |

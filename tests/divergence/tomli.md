Score(3000)=0.232 I=0.247 C=0.218 ns_rows≤3K=29/43 (reached=6 partial=1 missing=22)

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
| ns | 323 |  | 54 | _parser.py roster: entry points + error class | 1.11 |  | 0.269 |
| ns | 401 |  | 78 | CI workflow job roster | 1.12 |  | 0.254 |
| ns | 448 |  | 47 | _re.py function roster | 1.13 |  | 0.246 |
| ns | 566 |  | 118 | pyproject.toml tox environment roster | 1.14 |  | 0.232 |
| walker |  | 673 | 393 | README headline in README.md |  |  | 0.232 |
| ns | 696 |  | 130 | tomli/__init__.py (full public API) | 2.1 |  | 0.254 |
| walker |  | 703 | 30 | listing of 'tests' |  |  | 0.316 |
| walker |  | 808 | 105 | [package] in pyproject.toml |  |  | 0.317 |
| ns | 818 |  | 122 | _parser.py roster: table/array/key rules | 2.2 |  | 0.297 |
| walker |  | 840 | 32 | python imports in setup.py |  |  | 0.297 |
| ns | 945 |  | 127 | pyproject.toml mypy strict config | 2.3 |  | 0.279 |
| ns | 1083 |  | 138 | _parser.py roster: value parsing | 2.4 |  | 0.263 |
| walker |  | 1225 | 385 | headings outline in README.md |  |  | 0.249 |
| ns | 1225 |  | 142 | _parser.py roster: namespace state machinery | 2.5 |  | 0.249 |
| walker |  | 1234 | 9 | README.md section #0 |  |  | 0.249 |
| walker |  | 1240 | 6 | README.md section #19 |  |  | 0.249 |
| walker |  | 1265 | 25 | README.md section #2 |  |  | 0.249 |
| walker |  | 1273 | 8 | README.md section #13 |  |  | 0.249 |
| walker |  | 1284 | 11 | README.md section #14 |  |  | 0.249 |
| walker |  | 1298 | 14 | README.md section #18 |  |  | 0.249 |
| walker |  | 1332 | 34 | python decl names surface in src/tomli/_types.py |  |  | 0.249 |
| walker |  | 1360 | 28 | README.md section #16 |  |  | 0.249 |
| ns | 1396 |  | 171 | .pre-commit-config.yaml hook-id roster | 2.6 |  | 0.237 |
| walker |  | 1443 | 83 | README.md section #25 |  |  | 0.237 |
| walker |  | 1474 | 31 | README.md section #23 |  |  | 0.237 |
| ns | 1497 |  | 101 | tests/data/valid/ listing | 2.7 |  | 0.223 |
| walker |  | 1507 | 33 | README.md section #21 |  |  | 0.223 |
| ns | 1651 |  | 154 | tests/data/invalid/ listing | 2.8 |  | 0.207 |
| ns | 1676 |  | 25 | Sample invalid TOML fixture (dotted-keys/extend-defined-aot) | 2.9 |  | 0.206 |
| walker |  | 1682 | 175 | tomllib.md section #0 |  |  | 0.208 |
| walker |  | 1685 | 3 | listing of '.github' |  |  | 0.208 |
| walker |  | 1689 | 4 | listing of '.github/workflows' |  |  | 0.208 |
| walker |  | 1729 | 40 | README.md section #9 |  |  | 0.208 |
| ns | 1915 |  | 239 | Test method roster (test_data/test_error/test_misc) | 2.10 |  | 0.197 |
| ns | 2024 |  | 109 | _types.py (full) | 2.11 |  | 0.194 |
| ns | 2179 |  | 155 | Sample valid TOML/JSON fixture pair (array-subtables) | 3.1 |  | 0.185 |
| ns | 2287 |  | 108 | tests/__init__.py (tomllib aliasing) | 3.2 |  | 0.181 |
| ns | 2517 |  | 230 | External toml-test valid/ category listing | 3.3 |  | 0.167 |
| walker |  | 2635 | 906 | python decl names surface in src/tomli/_parser.py |  |  | 0.233 |
| walker |  | 2635 | 0 | python decl at src/tomli/_parser.py:71 |  |  | 0.233 |
| walker |  | 2635 | 0 | python decl at src/tomli/_parser.py:76 |  |  | 0.233 |
| walker |  | 2635 | 0 | python decl at src/tomli/_parser.py:137 |  |  | 0.233 |
| walker |  | 2635 | 0 | python decl at src/tomli/_parser.py:149 |  |  | 0.233 |
| walker |  | 2635 | 0 | python decl at src/tomli/_parser.py:220 |  |  | 0.233 |
| walker |  | 2635 | 0 | python decl at src/tomli/_parser.py:278 |  |  | 0.233 |
| walker |  | 2635 | 0 | python decl at src/tomli/_parser.py:312 |  |  | 0.233 |
| walker |  | 2635 | 0 | python decl at src/tomli/_parser.py:318 |  |  | 0.233 |
| walker |  | 2635 | 0 | python decl at src/tomli/_parser.py:349 |  |  | 0.233 |
| walker |  | 2635 | 0 | python decl at src/tomli/_parser.py:361 |  |  | 0.233 |
| walker |  | 2635 | 0 | python decl at src/tomli/_parser.py:370 |  |  | 0.233 |
| walker |  | 2635 | 0 | python decl at src/tomli/_parser.py:390 |  |  | 0.233 |
| walker |  | 2635 | 0 | python decl at src/tomli/_parser.py:463 |  |  | 0.233 |
| walker |  | 2635 | 0 | python decl at src/tomli/_parser.py:481 |  |  | 0.233 |
| walker |  | 2635 | 0 | python decl at src/tomli/_parser.py:497 |  |  | 0.233 |
| walker |  | 2635 | 0 | python decl at src/tomli/_parser.py:595 |  |  | 0.233 |
| walker |  | 2635 | 0 | python decl at src/tomli/_parser.py:599 |  |  | 0.233 |
| walker |  | 2635 | 0 | python decl at src/tomli/_parser.py:612 |  |  | 0.233 |
| walker |  | 2635 | 0 | python decl at src/tomli/_parser.py:621 |  |  | 0.233 |
| walker |  | 2635 | 0 | python decl at src/tomli/_parser.py:652 |  |  | 0.233 |
| walker |  | 2635 | 0 | python decl at src/tomli/_parser.py:760 |  |  | 0.233 |
| walker |  | 2635 | 0 | python decl at src/tomli/_parser.py:764 |  |  | 0.233 |
| walker |  | 2648 | 13 | python decl doc at src/tomli/_parser.py:149 |  |  | 0.233 |
| walker |  | 2664 | 16 | python decl doc at src/tomli/_parser.py:220 |  |  | 0.233 |
| walker |  | 2687 | 23 | python decl at src/tomli/_parser.py:51 |  |  | 0.233 |
| ns | 2711 |  | 194 | pyproject.toml build-system + core [project] identity | 3.4 |  | 0.232 |
| walker |  | 2719 | 32 | python decl at src/tomli/_parser.py:564 |  |  | 0.232 |
| walker |  | 2734 | 15 | python decl doc at src/tomli/_parser.py:137 |  |  | 0.232 |
| walker |  | 2769 | 35 | python decl at src/tomli/_parser.py:413 |  |  | 0.232 |
| walker |  | 2805 | 36 | python decl at src/tomli/_parser.py:684 |  |  | 0.232 |
| walker |  | 2842 | 37 | python decl at src/tomli/_parser.py:502 |  |  | 0.232 |
| walker |  | 2880 | 38 | python decl at src/tomli/_parser.py:447 |  |  | 0.232 |
| walker |  | 2919 | 39 | python decl at src/tomli/_parser.py:528 |  |  | 0.232 |
| walker |  | 2934 | 15 | python decl body at src/tomli/_parser.py:595 body 596 |  |  | 0.232 |
| walker |  | 2966 | 32 | python decl doc at src/tomli/_parser.py:71 |  |  | 0.232 |
| walker |  | 2990 | 24 | python decl body at src/tomli/_parser.py:497 body 498 |  |  | 0.232 |
| ns | 3036 |  | 325 | README intro | 3.5 |  | 0.225 |
| walker |  | 3056 | 66 | python decl at src/tomli/_parser.py:327 |  |  | 0.225 |
| walker |  | 3083 | 27 | python decl body at src/tomli/_parser.py:760 body 761 |  |  | 0.225 |
| walker |  | 3163 | 80 | python class body at src/tomli/_parser.py:220 |  |  | 0.225 |
| ns | 3257 |  | 221 | README usage: parse a TOML string | 4.1 |  | 0.218 |
| walker |  | 3366 | 203 | python method sigs in src/tomli/_parser.py |  |  | 0.249 |
| walker |  | 3366 | 0 | python method at src/tomli/_parser.py:229 |  |  | 0.249 |
| walker |  | 3366 | 0 | python method at src/tomli/_parser.py:233 |  |  | 0.249 |
| walker |  | 3366 | 0 | python method at src/tomli/_parser.py:236 |  |  | 0.249 |
| walker |  | 3366 | 0 | python method at src/tomli/_parser.py:241 |  |  | 0.249 |
| walker |  | 3366 | 0 | python method at src/tomli/_parser.py:249 |  |  | 0.249 |
| walker |  | 3366 | 0 | python method at src/tomli/_parser.py:260 |  |  | 0.249 |
| walker |  | 3366 | 0 | python method at src/tomli/_parser.py:300 |  |  | 0.249 |
| walker |  | 3366 | 0 | python method at src/tomli/_parser.py:313 |  |  | 0.249 |
| walker |  | 3381 | 15 | python method at src/tomli/_parser.py:279 |  |  | 0.249 |
| walker |  | 3394 | 13 | python method body at src/tomli/_parser.py:233 body 234 |  |  | 0.249 |
| walker |  | 3407 | 13 | python method body at src/tomli/_parser.py:279 body 281 |  |  | 0.249 |
| walker |  | 3453 | 46 | python method at src/tomli/_parser.py:283 |  |  | 0.249 |
| walker |  | 3472 | 19 | python method body at src/tomli/_parser.py:313 body 314 |  |  | 0.249 |
| walker |  | 3555 | 83 | python method at src/tomli/_parser.py:87 |  |  | 0.250 |
| ns | 3588 |  | 331 | tomllib.md - intro + sync status | 4.2 |  | 0.258 |
| walker |  | 3606 | 51 | python decl body at src/tomli/_parser.py:318 body 319 |  |  | 0.258 |
| walker |  | 3716 | 110 | python decl doc at src/tomli/_parser.py:76 |  |  | 0.258 |
| walker |  | 3811 | 95 | python decl doc at src/tomli/_parser.py:764 |  |  | 0.258 |
| walker |  | 3850 | 39 | README.md section #22 |  |  | 0.258 |
| ns | 3885 |  | 297 | README: TOML->Python type mapping table | 4.3 |  | 0.252 |
| walker |  | 3920 | 70 | python decl body at src/tomli/_parser.py:361 body 362 |  |  | 0.252 |
| walker |  | 3969 | 49 | README.md section #17 |  |  | 0.252 |
| walker |  | 4007 | 38 | python method body at src/tomli/_parser.py:229 body 230 |  |  | 0.252 |
| ns | 4024 |  | 139 | CHANGELOG.md - latest two releases | 4.4 |  | 0.247 |
| walker |  | 4045 | 38 | python method body at src/tomli/_parser.py:236 body 237 |  |  | 0.247 |
| walker |  | 4101 | 56 | README.md section #5 |  |  | 0.247 |
| walker |  | 4157 | 56 | README.md section #8 |  |  | 0.247 |
| walker |  | 4215 | 58 | README.md section #4 |  |  | 0.247 |
| ns | 4256 |  | 232 | burntsushi.py: convert() (partial) | 4.5 |  | 0.240 |
| walker |  | 4371 | 156 | python decl at src/tomli/_parser.py:57 |  |  | 0.240 |
| walker |  | 4401 | 30 | tomllib.md section #3 |  |  | 0.240 |
| ns | 4555 |  | 299 | test_misc.py: test_parse_float | 4.6 | 2.10 | 0.232 |
| walker |  | 4702 | 301 | README.md section #1 |  |  | 0.256 |
| walker |  | 4792 | 90 | python decl body at src/tomli/_parser.py:612 body 613 |  |  | 0.256 |
| walker |  | 4824 | 32 | tomllib.md section #2 |  |  | 0.256 |
| walker |  | 4895 | 71 | README.md section #10 |  |  | 0.256 |
| walker |  | 5056 | 161 | python decl names surface in src/tomli/_re.py |  |  | 0.261 |
| walker |  | 5056 | 0 | python decl at src/tomli/_re.py:59 |  |  | 0.261 |
| walker |  | 5056 | 0 | python decl at src/tomli/_re.py:109 |  |  | 0.261 |
| walker |  | 5056 | 0 | python decl at src/tomli/_re.py:116 |  |  | 0.261 |
| walker |  | 5082 | 26 | python decl at src/tomli/_re.py:98 |  |  | 0.264 |
| ns | 5094 |  | 539 | test_data.py (full, data-driven glob mechanism) | 4.7 | 2.10 | 0.249 |
| walker |  | 5116 | 34 | python decl body at src/tomli/_re.py:116 body 117 |  |  | 0.249 |
| walker |  | 5178 | 62 | python decl doc at src/tomli/_re.py:59 |  |  | 0.249 |
| walker |  | 5278 | 100 | python decl body at src/tomli/_parser.py:349 body 350 |  |  | 0.249 |
| walker |  | 5288 | 10 | CHANGELOG.md section #0 |  |  | 0.249 |
| walker |  | 5359 | 71 | README.md section #20 |  |  | 0.249 |
| walker |  | 5432 | 73 | README.md section #15 |  |  | 0.249 |
| walker |  | 5496 | 64 | python imports in src/tomli/_types.py |  |  | 0.259 |
| walker |  | 5566 | 70 | python decl body at src/tomli/_re.py:98 body 100 |  |  | 0.259 |
| walker |  | 5574 | 8 | listing of 'fuzzer' |  |  | 0.351 |
| walker |  | 5583 | 9 | listing of 'profiler' |  |  | 0.383 |
| walker |  | 5647 | 64 | python method body at src/tomli/_parser.py:241 body 242 |  |  | 0.384 |
| walker |  | 5658 | 11 | python decl body at src/tomli/_parser.py:137 body 139 |  |  | 0.384 |
| walker |  | 5755 | 97 | README.md section #11 |  |  | 0.384 |
| ns | 5772 |  | 678 | Flags class (namespace mutability tracking) | 5.1 | 2.5 | 0.379 |
| walker |  | 5914 | 159 | python decl at src/tomli/_re.py:46 |  |  | 0.379 |
| walker |  | 5997 | 83 | python decl body at src/tomli/_re.py:109 body 110 |  |  | 0.379 |
| walker |  | 6098 | 101 | README.md section #7 |  |  | 0.379 |
| walker |  | 6255 | 157 | python imports in src/tomli/_parser.py |  |  | 0.379 |
| walker |  | 6306 | 51 | tomllib.md section #7 |  |  | 0.379 |
| walker |  | 6454 | 148 | python decl body at src/tomli/_parser.py:447 body 450 |  |  | 0.379 |
| ns | 6492 |  | 720 | TOMLDecodeError (+ deprecation sentinel) full body | 5.2 | 1.11 | 0.365 |
| walker |  | 6602 | 148 | python decl body at src/tomli/_parser.py:599 body 600 |  |  | 0.365 |
| walker |  | 6718 | 116 | README.md section #12 |  |  | 0.365 |
| walker |  | 6874 | 156 | python decl body at src/tomli/_parser.py:327 body 335 |  |  | 0.365 |
| walker |  | 6977 | 103 | python imports in src/tomli/_re.py |  |  | 0.366 |
| walker |  | 6991 | 14 | python decl body at src/tomli/_parser.py:137 body 146 |  |  | 0.366 |
| ns | 7226 |  | 734 | _re.py regex definitions (RE_NUMBER/RE_LOCALTIME/RE_DATETIME) | 5.3 |  | 0.362 |
| ns | 8144 |  | 918 | parse_value() type dispatch | 5.4 | 2.4 | 0.341 |
| walker |  | 8803 | 1812 | manifest config in pyproject.toml |  |  | 0.376 |
| walker |  | 8920 | 117 | README.md section #6 |  |  | 0.376 |
| ns | 9105 |  | 961 | loads()/load() entry point + parse loop | 5.5 | 1.11 | 0.355 |
| walker |  | 9157 | 237 | python decl at src/tomli/_re.py:26 |  |  | 0.376 |
| walker |  | 9261 | 104 | python decl at src/tomli/_re.py:17 |  |  | 0.389 |
| walker |  | 9278 | 17 | listing of 'scripts' |  |  | 0.415 |
| walker |  | 9387 | 109 | python method body at src/tomli/_parser.py:300 body 301 |  |  | 0.415 |
| walker |  | 9463 | 76 | tomllib.md section #4 |  |  | 0.415 |
| walker |  | 9473 | 10 | python decl body at src/tomli/_parser.py:652 body 659 |  |  | 0.415 |
| walker |  | 9483 | 10 | python decl body at src/tomli/_parser.py:652 body 660 |  |  | 0.415 |
| walker |  | 9493 | 10 | python decl body at src/tomli/_parser.py:764 body 782 |  |  | 0.415 |
| walker |  | 9513 | 20 | listing of 'benchmark' |  |  | 0.434 |
| walker |  | 9522 | 9 | README headline in benchmark/README.md |  |  | 0.434 |
| walker |  | 9605 | 83 | tomllib.md section #5 |  |  | 0.434 |
| walker |  | 9727 | 122 | python method body at src/tomli/_parser.py:283 body 289 |  |  | 0.434 |
| walker |  | 9804 | 77 | python imports in tests/__init__.py |  |  | 0.439 |
| walker |  | 9976 | 172 | README.md section #3 |  |  | 0.461 |
| ns | 9987 |  | 882 | key_value_rule / parse_key_value_pair / parse_key (full bodies) | 5.6 | 2.2 | 0.445 |

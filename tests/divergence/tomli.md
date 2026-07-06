Score(3000)=0.266 I=0.268 C=0.265 ns_rows≤3K=29/43 (reached=8 partial=0 missing=21)

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
| ns | 1915 |  | 239 | Test method roster (test_data/test_error/test_misc) | 2.10 |  | 0.197 |
| ns | 2024 |  | 109 | _types.py (full) | 2.11 |  | 0.194 |
| ns | 2179 |  | 155 | Sample valid TOML/JSON fixture pair (array-subtables) | 3.1 |  | 0.185 |
| walker |  | 2257 | 575 | python decl sigs roster in src/tomli/_parser.py |  |  | 0.258 |
| walker |  | 2257 | 0 | python decl names surface in src/tomli/_parser.py |  |  | 0.258 |
| walker |  | 2257 | 0 | python decl names surface #1 in src/tomli/_parser.py |  |  | 0.258 |
| walker |  | 2257 | 0 | python decl at src/tomli/_parser.py:71 |  |  | 0.258 |
| walker |  | 2257 | 0 | python decl at src/tomli/_parser.py:76 |  |  | 0.258 |
| walker |  | 2257 | 0 | python decl at src/tomli/_parser.py:137 |  |  | 0.258 |
| walker |  | 2257 | 0 | python decl at src/tomli/_parser.py:149 |  |  | 0.258 |
| walker |  | 2257 | 0 | python decl at src/tomli/_parser.py:220 |  |  | 0.258 |
| walker |  | 2257 | 0 | python decl at src/tomli/_parser.py:278 |  |  | 0.258 |
| walker |  | 2257 | 0 | python decl at src/tomli/_parser.py:312 |  |  | 0.258 |
| walker |  | 2257 | 0 | python decl at src/tomli/_parser.py:318 |  |  | 0.258 |
| walker |  | 2257 | 0 | python decl at src/tomli/_parser.py:349 |  |  | 0.258 |
| walker |  | 2257 | 0 | python decl at src/tomli/_parser.py:361 |  |  | 0.258 |
| walker |  | 2257 | 0 | python decl at src/tomli/_parser.py:370 |  |  | 0.258 |
| walker |  | 2257 | 0 | python decl at src/tomli/_parser.py:390 |  |  | 0.258 |
| walker |  | 2257 | 0 | python decl at src/tomli/_parser.py:463 |  |  | 0.258 |
| walker |  | 2257 | 0 | python decl at src/tomli/_parser.py:481 |  |  | 0.258 |
| walker |  | 2257 | 0 | python decl at src/tomli/_parser.py:497 |  |  | 0.258 |
| walker |  | 2257 | 0 | python decl at src/tomli/_parser.py:595 |  |  | 0.258 |
| walker |  | 2257 | 0 | python decl at src/tomli/_parser.py:599 |  |  | 0.258 |
| walker |  | 2257 | 0 | python decl at src/tomli/_parser.py:612 |  |  | 0.258 |
| walker |  | 2270 | 13 | python decl doc at src/tomli/_parser.py:149 |  |  | 0.258 |
| walker |  | 2286 | 16 | python decl doc at src/tomli/_parser.py:220 |  |  | 0.258 |
| ns | 2287 |  | 108 | tests/__init__.py (tomllib aliasing) | 3.2 |  | 0.253 |
| walker |  | 2318 | 32 | python decl at src/tomli/_parser.py:564 |  |  | 0.253 |
| walker |  | 2333 | 15 | python decl doc at src/tomli/_parser.py:137 |  |  | 0.253 |
| walker |  | 2368 | 35 | python decl at src/tomli/_parser.py:413 |  |  | 0.253 |
| walker |  | 2405 | 37 | python decl at src/tomli/_parser.py:502 |  |  | 0.253 |
| walker |  | 2443 | 38 | python decl at src/tomli/_parser.py:447 |  |  | 0.253 |
| walker |  | 2482 | 39 | python decl at src/tomli/_parser.py:528 |  |  | 0.253 |
| walker |  | 2497 | 15 | python decl body at src/tomli/_parser.py:595 body 596 |  |  | 0.253 |
| ns | 2517 |  | 230 | External toml-test valid/ category listing | 3.3 |  | 0.233 |
| walker |  | 2529 | 32 | python decl doc at src/tomli/_parser.py:71 |  |  | 0.233 |
| walker |  | 2553 | 24 | python decl body at src/tomli/_parser.py:497 body 498 |  |  | 0.233 |
| walker |  | 2619 | 66 | python decl at src/tomli/_parser.py:327 |  |  | 0.233 |
| walker |  | 2699 | 80 | python class body at src/tomli/_parser.py:220 |  |  | 0.233 |
| ns | 2711 |  | 194 | pyproject.toml build-system + core [project] identity | 3.4 |  | 0.232 |
| walker |  | 2902 | 203 | python method sigs in src/tomli/_parser.py |  |  | 0.266 |
| walker |  | 2902 | 0 | python method at src/tomli/_parser.py:229 |  |  | 0.266 |
| walker |  | 2902 | 0 | python method at src/tomli/_parser.py:233 |  |  | 0.266 |
| walker |  | 2902 | 0 | python method at src/tomli/_parser.py:236 |  |  | 0.266 |
| walker |  | 2902 | 0 | python method at src/tomli/_parser.py:241 |  |  | 0.266 |
| walker |  | 2902 | 0 | python method at src/tomli/_parser.py:249 |  |  | 0.266 |
| walker |  | 2902 | 0 | python method at src/tomli/_parser.py:260 |  |  | 0.266 |
| walker |  | 2902 | 0 | python method at src/tomli/_parser.py:300 |  |  | 0.266 |
| walker |  | 2902 | 0 | python method at src/tomli/_parser.py:313 |  |  | 0.266 |
| walker |  | 2917 | 15 | python method at src/tomli/_parser.py:279 |  |  | 0.266 |
| walker |  | 2930 | 13 | python method body at src/tomli/_parser.py:233 body 234 |  |  | 0.266 |
| walker |  | 2943 | 13 | python method body at src/tomli/_parser.py:279 body 281 |  |  | 0.266 |
| walker |  | 2989 | 46 | python method at src/tomli/_parser.py:283 |  |  | 0.266 |
| walker |  | 3008 | 19 | python method body at src/tomli/_parser.py:313 body 314 |  |  | 0.266 |
| ns | 3036 |  | 325 | README intro | 3.5 |  | 0.258 |
| walker |  | 3091 | 83 | python method at src/tomli/_parser.py:87 |  |  | 0.258 |
| walker |  | 3142 | 51 | python decl body at src/tomli/_parser.py:318 body 319 |  |  | 0.258 |
| walker |  | 3252 | 110 | python decl doc at src/tomli/_parser.py:76 |  |  | 0.259 |
| walker |  | 3255 | 3 | listing of '.github' |  |  | 0.259 |
| ns | 3257 |  | 221 | README usage: parse a TOML string | 4.1 |  | 0.250 |
| walker |  | 3259 | 4 | listing of '.github/workflows' |  |  | 0.250 |
| walker |  | 3299 | 40 | README.md section #9 |  |  | 0.250 |
| walker |  | 3338 | 39 | README.md section #22 |  |  | 0.250 |
| walker |  | 3408 | 70 | python decl body at src/tomli/_parser.py:361 body 362 |  |  | 0.250 |
| walker |  | 3457 | 49 | README.md section #17 |  |  | 0.250 |
| walker |  | 3495 | 38 | python method body at src/tomli/_parser.py:229 body 230 |  |  | 0.250 |
| walker |  | 3533 | 38 | python method body at src/tomli/_parser.py:236 body 237 |  |  | 0.251 |
| ns | 3588 |  | 331 | tomllib.md - intro + sync status | 4.2 |  | 0.259 |
| walker |  | 3589 | 56 | README.md section #5 |  |  | 0.259 |
| walker |  | 3645 | 56 | README.md section #8 |  |  | 0.259 |
| walker |  | 3703 | 58 | README.md section #4 |  |  | 0.259 |
| walker |  | 3733 | 30 | tomllib.md section #3 |  |  | 0.259 |
| ns | 3885 |  | 297 | README: TOML->Python type mapping table | 4.3 |  | 0.252 |
| ns | 4024 |  | 139 | CHANGELOG.md - latest two releases | 4.4 |  | 0.247 |
| walker |  | 4034 | 301 | README.md section #1 |  |  | 0.272 |
| walker |  | 4124 | 90 | python decl body at src/tomli/_parser.py:612 body 613 |  |  | 0.272 |
| walker |  | 4156 | 32 | tomllib.md section #2 |  |  | 0.272 |
| walker |  | 4227 | 71 | README.md section #10 |  |  | 0.272 |
| ns | 4256 |  | 232 | burntsushi.py: convert() (partial) | 4.5 |  | 0.265 |
| walker |  | 4388 | 161 | python decl names surface in src/tomli/_re.py |  |  | 0.271 |
| walker |  | 4388 | 0 | python decl at src/tomli/_re.py:59 |  |  | 0.271 |
| walker |  | 4388 | 0 | python decl at src/tomli/_re.py:109 |  |  | 0.271 |
| walker |  | 4388 | 0 | python decl at src/tomli/_re.py:116 |  |  | 0.271 |
| walker |  | 4414 | 26 | python decl at src/tomli/_re.py:98 |  |  | 0.274 |
| walker |  | 4448 | 34 | python decl body at src/tomli/_re.py:116 body 117 |  |  | 0.274 |
| walker |  | 4510 | 62 | python decl doc at src/tomli/_re.py:59 |  |  | 0.274 |
| ns | 4555 |  | 299 | test_misc.py: test_parse_float | 4.6 | 2.10 | 0.264 |
| walker |  | 4610 | 100 | python decl body at src/tomli/_parser.py:349 body 350 |  |  | 0.264 |
| walker |  | 4620 | 10 | CHANGELOG.md section #0 |  |  | 0.264 |
| walker |  | 4691 | 71 | README.md section #20 |  |  | 0.264 |
| walker |  | 4764 | 73 | README.md section #15 |  |  | 0.264 |
| walker |  | 4828 | 64 | python imports in src/tomli/_types.py |  |  | 0.274 |
| walker |  | 4898 | 70 | python decl body at src/tomli/_re.py:98 body 100 |  |  | 0.274 |
| walker |  | 4906 | 8 | listing of 'fuzzer' |  |  | 0.372 |
| walker |  | 5074 | 168 | python decl names surface #2 in src/tomli/_parser.py |  |  | 0.372 |
| walker |  | 5074 | 0 | python decl at src/tomli/_parser.py:621 |  |  | 0.372 |
| walker |  | 5074 | 0 | python decl at src/tomli/_parser.py:652 |  |  | 0.372 |
| walker |  | 5074 | 0 | python decl at src/tomli/_parser.py:760 |  |  | 0.372 |
| walker |  | 5074 | 0 | python decl at src/tomli/_parser.py:764 |  |  | 0.372 |
| ns | 5094 |  | 539 | test_data.py (full, data-driven glob mechanism) | 4.7 | 2.10 | 0.351 |
| walker |  | 5110 | 36 | python decl at src/tomli/_parser.py:684 |  |  | 0.351 |
| walker |  | 5137 | 27 | python decl body at src/tomli/_parser.py:760 body 761 |  |  | 0.351 |
| walker |  | 5232 | 95 | python decl doc at src/tomli/_parser.py:764 |  |  | 0.351 |
| walker |  | 5241 | 9 | listing of 'profiler' |  |  | 0.383 |
| walker |  | 5305 | 64 | python method body at src/tomli/_parser.py:241 body 242 |  |  | 0.384 |
| walker |  | 5316 | 11 | python decl body at src/tomli/_parser.py:137 body 139 |  |  | 0.384 |
| walker |  | 5413 | 97 | README.md section #11 |  |  | 0.384 |
| walker |  | 5572 | 159 | python decl at src/tomli/_re.py:46 |  |  | 0.384 |
| walker |  | 5655 | 83 | python decl body at src/tomli/_re.py:109 body 110 |  |  | 0.384 |
| walker |  | 5756 | 101 | README.md section #7 |  |  | 0.384 |
| ns | 5772 |  | 678 | Flags class (namespace mutability tracking) | 5.1 | 2.5 | 0.379 |
| walker |  | 5913 | 157 | python imports in src/tomli/_parser.py |  |  | 0.379 |
| walker |  | 5964 | 51 | tomllib.md section #7 |  |  | 0.379 |
| walker |  | 6112 | 148 | python decl body at src/tomli/_parser.py:447 body 450 |  |  | 0.379 |
| walker |  | 6260 | 148 | python decl body at src/tomli/_parser.py:599 body 600 |  |  | 0.379 |
| walker |  | 6376 | 116 | README.md section #12 |  |  | 0.379 |
| ns | 6492 |  | 720 | TOMLDecodeError (+ deprecation sentinel) full body | 5.2 | 1.11 | 0.365 |
| walker |  | 6532 | 156 | python decl body at src/tomli/_parser.py:327 body 335 |  |  | 0.365 |
| walker |  | 6635 | 103 | python imports in src/tomli/_re.py |  |  | 0.366 |
| walker |  | 6649 | 14 | python decl body at src/tomli/_parser.py:137 body 146 |  |  | 0.366 |
| ns | 7226 |  | 734 | _re.py regex definitions (RE_NUMBER/RE_LOCALTIME/RE_DATETIME) | 5.3 |  | 0.362 |
| ns | 8144 |  | 918 | parse_value() type dispatch | 5.4 | 2.4 | 0.341 |
| walker |  | 8461 | 1812 | manifest config in pyproject.toml |  |  | 0.376 |
| walker |  | 8578 | 117 | README.md section #6 |  |  | 0.376 |
| walker |  | 8815 | 237 | python decl at src/tomli/_re.py:26 |  |  | 0.398 |
| walker |  | 8919 | 104 | python decl at src/tomli/_re.py:17 |  |  | 0.412 |
| walker |  | 8936 | 17 | listing of 'scripts' |  |  | 0.439 |
| walker |  | 9045 | 109 | python method body at src/tomli/_parser.py:300 body 301 |  |  | 0.439 |
| ns | 9105 |  | 961 | loads()/load() entry point + parse loop | 5.5 | 1.11 | 0.415 |
| walker |  | 9121 | 76 | tomllib.md section #4 |  |  | 0.415 |
| walker |  | 9131 | 10 | python decl body at src/tomli/_parser.py:652 body 659 |  |  | 0.415 |
| walker |  | 9141 | 10 | python decl body at src/tomli/_parser.py:652 body 660 |  |  | 0.415 |
| walker |  | 9151 | 10 | python decl body at src/tomli/_parser.py:764 body 782 |  |  | 0.415 |
| walker |  | 9171 | 20 | listing of 'benchmark' |  |  | 0.434 |
| walker |  | 9180 | 9 | README headline in benchmark/README.md |  |  | 0.434 |
| walker |  | 9338 | 158 | python decl names surface #3 in src/tomli/_parser.py |  |  | 0.434 |
| walker |  | 9361 | 23 | python decl at src/tomli/_parser.py:51 |  |  | 0.434 |
| walker |  | 9517 | 156 | python decl at src/tomli/_parser.py:57 |  |  | 0.434 |
| walker |  | 9600 | 83 | tomllib.md section #5 |  |  | 0.434 |
| walker |  | 9722 | 122 | python method body at src/tomli/_parser.py:283 body 289 |  |  | 0.434 |
| walker |  | 9799 | 77 | python imports in tests/__init__.py |  |  | 0.439 |
| walker |  | 9971 | 172 | README.md section #3 |  |  | 0.461 |
| ns | 9987 |  | 882 | key_value_rule / parse_key_value_pair / parse_key (full bodies) | 5.6 | 2.2 | 0.445 |

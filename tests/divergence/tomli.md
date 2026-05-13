Score(3000)=0.504 I=0.752 C=0.337 ns_rows≤3K=18/40 (reached=6 partial=3 missing=9)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 62 | 62 | listing of '.' |  |  | 1.000 |
| ns | 62 |  | 62 | Repo root listing | 1.1 |  | 1.000 |
| walker |  | 66 | 4 | listing of 'src' |  |  | 1.000 |
| walker |  | 74 | 8 | listing of 'fuzzer' |  |  | 1.000 |
| walker |  | 83 | 9 | listing of 'profiler' |  |  | 1.000 |
| ns | 87 |  | 25 | README title + tagline | 1.2 |  | 0.911 |
| walker |  | 134 | 51 | README headline in README.md |  |  | 0.969 |
| walker |  | 144 | 10 | python decl names surface in profiler/profiler_script.py |  |  | 0.969 |
| ns | 166 |  | 79 | Public API: __init__ __all__ + version | 1.3 |  | 0.872 |
| walker |  | 170 | 26 | listing of 'src/tomli' |  |  | 0.892 |
| walker |  | 187 | 17 | listing of 'scripts' |  |  | 0.893 |
| ns | 192 |  | 26 | src/tomli/ module listing | 1.4 |  | 0.890 |
| walker |  | 201 | 14 | python decl names surface in scripts/use_setuptools.py |  |  | 0.890 |
| walker |  | 201 | 0 | python decl at scripts/use_setuptools.py:12 |  |  | 0.890 |
| walker |  | 275 | 74 | python imports in src/tomli/__init__.py |  |  | 0.955 |
| ns | 279 |  | 87 | loads / load signatures + docstrings | 1.5 |  | 0.871 |
| walker |  | 330 | 55 | headings outline in tomllib.md |  |  | 0.871 |
| walker |  | 339 | 9 | python imports in fuzzer/fuzz.py |  |  | 0.871 |
| walker |  | 364 | 25 | python imports in setup.py |  |  | 0.871 |
| walker |  | 396 | 32 | python decl at profiler/profiler_script.py:12 |  |  | 0.871 |
| ns | 403 |  | 124 | TOMLDecodeError class + docstring | 1.6 |  | 0.749 |
| walker |  | 428 | 32 | python decl names surface in src/tomli/_types.py |  |  | 0.751 |
| ns | 433 |  | 30 | tests/ directory listing | 2.1 |  | 0.697 |
| walker |  | 441 | 13 | python imports in src/tomli/_types.py |  |  | 0.700 |
| ns | 491 |  | 58 | _types.py — full | 2.2 |  | 0.692 |
| walker |  | 501 | 60 | python decl names surface in fuzzer/fuzz.py |  |  | 0.692 |
| walker |  | 501 | 0 | python decl at fuzzer/fuzz.py:53 |  |  | 0.692 |
| walker |  | 501 | 0 | python decl at fuzzer/fuzz.py:59 |  |  | 0.692 |
| walker |  | 514 | 13 | python decl at fuzzer/fuzz.py:20 |  |  | 0.692 |
| walker |  | 536 | 22 | python decl at fuzzer/fuzz.py:71 |  |  | 0.692 |
| walker |  | 552 | 16 | python decl doc at fuzzer/fuzz.py:59 |  |  | 0.692 |
| walker |  | 582 | 30 | listing of 'tests' |  |  | 0.765 |
| ns | 697 |  | 206 | _parser.py: state-class headers + Flags constants | 2.3 |  | 0.655 |
| ns | 844 |  | 147 | _re.py: regex constants + match-helper locations | 2.4 |  | 0.617 |
| walker |  | 961 | 379 | headings outline in README.md |  |  | 0.617 |
| walker |  | 983 | 22 | README.md section #2 |  |  | 0.617 |
| walker |  | 989 | 6 | README.md section #19 |  |  | 0.617 |
| walker |  | 997 | 8 | README.md section #13 |  |  | 0.617 |
| walker |  | 1008 | 11 | README.md section #14 |  |  | 0.617 |
| walker |  | 1022 | 14 | README.md section #18 |  |  | 0.617 |
| ns | 1160 |  | 316 | README intro paragraph | 2.5 |  | 0.553 |
| walker |  | 1384 | 362 | README.md section #0 |  |  | 0.562 |
| walker |  | 1459 | 75 | README.md section #25 |  |  | 0.562 |
| walker |  | 1485 | 26 | README.md section #16 |  |  | 0.562 |
| ns | 1486 |  | 326 | README table of contents | 2.6 |  | 0.613 |
| walker |  | 1525 | 40 | python decl body at fuzzer/fuzz.py:71 body 73 |  |  | 0.613 |
| walker |  | 1595 | 70 | python imports in profiler/profiler_script.py |  |  | 0.613 |
| walker |  | 1626 | 31 | README.md section #23 |  |  | 0.613 |
| walker |  | 1701 | 75 | python imports in scripts/use_setuptools.py |  |  | 0.613 |
| walker |  | 1863 | 162 | tomllib.md section #0 |  |  | 0.613 |
| walker |  | 1896 | 33 | README.md section #21 |  |  | 0.613 |
| ns | 1904 |  | 418 | _parser.py: parse_* and skip_* function locations | 2.7 |  | 0.553 |
| walker |  | 1936 | 40 | README.md section #9 |  |  | 0.553 |
| walker |  | 1993 | 57 | python decl body at fuzzer/fuzz.py:53 body 54 |  |  | 0.553 |
| walker |  | 1996 | 3 | listing of '.github' |  |  | 0.553 |
| walker |  | 2000 | 4 | listing of '.github/workflows' |  |  | 0.553 |
| walker |  | 2013 | 13 | python decl body at fuzzer/fuzz.py:59 body 61 |  |  | 0.553 |
| walker |  | 2054 | 41 | README.md section #22 |  |  | 0.553 |
| ns | 2100 |  | 196 | README usage: parse a TOML string | 2.8 |  | 0.513 |
| walker |  | 2205 | 151 | python decl names surface in src/tomli/_re.py |  |  | 0.544 |
| walker |  | 2205 | 0 | python decl at src/tomli/_re.py:59 |  |  | 0.544 |
| walker |  | 2205 | 0 | python decl at src/tomli/_re.py:109 |  |  | 0.544 |
| walker |  | 2205 | 0 | python decl at src/tomli/_re.py:116 |  |  | 0.544 |
| walker |  | 2229 | 24 | python decl at src/tomli/_re.py:98 |  |  | 0.555 |
| walker |  | 2263 | 34 | python decl body at src/tomli/_re.py:116 body 117 |  |  | 0.555 |
| walker |  | 2318 | 55 | python decl doc at src/tomli/_re.py:59 |  |  | 0.555 |
| walker |  | 2373 | 55 | README.md section #4 |  |  | 0.556 |
| ns | 2392 |  | 292 | README usage: parse a file + handle errors | 2.9 |  | 0.519 |
| walker |  | 2429 | 56 | README.md section #5 |  |  | 0.538 |
| walker |  | 2485 | 56 | README.md section #8 |  |  | 0.538 |
| ns | 2491 |  | 99 | load body | 3.1 | 1.5 | 0.525 |
| walker |  | 2536 | 51 | README.md section #17 |  |  | 0.525 |
| walker |  | 2619 | 83 | python imports in src/tomli/_re.py |  |  | 0.525 |
| walker |  | 2647 | 28 | tomllib.md section #3 |  |  | 0.525 |
| ns | 2746 |  | 255 | tests/data/{valid,invalid}/ top listing | 3.2 |  | 0.470 |
| walker |  | 2935 | 288 | README.md section #1 |  |  | 0.527 |
| walker |  | 2943 | 8 | CHANGELOG.md section #0 |  |  | 0.527 |
| ns | 2972 |  | 226 | tests/* test method names | 3.3 |  | 0.504 |
| walker |  | 3014 | 71 | README.md section #10 |  |  | 0.504 |
| walker |  | 3048 | 34 | tomllib.md section #2 |  |  | 0.504 |
| walker |  | 3072 | 24 | python imports in tests/__init__.py |  |  | 0.504 |
| walker |  | 3145 | 73 | README.md section #20 |  |  | 0.504 |
| walker |  | 3217 | 72 | python decl body at src/tomli/_re.py:98 body 100 |  |  | 0.504 |
| walker |  | 3292 | 75 | README.md section #15 |  |  | 0.504 |
| ns | 3320 |  | 348 | loads body — prelude + skip / dispatch comments | 3.4 | 1.5 | 0.475 |
| walker |  | 3412 | 120 | python decl body at scripts/use_setuptools.py:12 body 13 |  |  | 0.475 |
| walker |  | 3504 | 92 | README.md section #11 |  |  | 0.475 |
| walker |  | 3654 | 150 | python imports in src/tomli/_parser.py |  |  | 0.475 |
| walker |  | 3737 | 83 | python decl body at src/tomli/_re.py:109 body 110 |  |  | 0.475 |
| ns | 3755 |  | 435 | loads body — rule dispatch + statement terminator | 3.5 | 3.4 | 0.441 |
| ns | 3977 |  | 222 | TOMLDecodeError.__init__ — pos→line/col body | 3.6 | 1.6 | 0.427 |
| walker |  | 3988 | 251 | python decl names surface in src/tomli/_parser.py |  |  | 0.427 |
| walker |  | 4013 | 25 | python decl at src/tomli/_parser.py:51 |  |  | 0.427 |
| walker |  | 4111 | 98 | README.md section #7 |  |  | 0.428 |
| walker |  | 4272 | 161 | python decl at src/tomli/_re.py:46 |  |  | 0.428 |
| walker |  | 4375 | 103 | README.md section #12 |  |  | 0.428 |
| walker |  | 4426 | 51 | tomllib.md section #7 |  |  | 0.428 |
| walker |  | 4432 | 6 | listing of 'tests/data' |  |  | 0.428 |
| walker |  | 4495 | 63 | tomllib.md section #5 |  |  | 0.428 |
| ns | 4502 |  | 525 | parse_value — dispatch head (strings/bools/array/inline-table) | 3.7 | 2.7 | 0.399 |
| walker |  | 4708 | 213 | python decl names surface #1 in src/tomli/_parser.py |  |  | 0.407 |
| walker |  | 4708 | 0 | python decl at src/tomli/_parser.py:71 |  |  | 0.407 |
| walker |  | 4708 | 0 | python decl at src/tomli/_parser.py:76 |  |  | 0.407 |
| walker |  | 4708 | 0 | python decl at src/tomli/_parser.py:137 |  |  | 0.407 |
| walker |  | 4708 | 0 | python decl at src/tomli/_parser.py:149 |  |  | 0.407 |
| walker |  | 4708 | 0 | python decl at src/tomli/_parser.py:220 |  |  | 0.407 |
| walker |  | 4708 | 0 | python decl at src/tomli/_parser.py:278 |  |  | 0.407 |
| walker |  | 4708 | 0 | python decl at src/tomli/_parser.py:312 |  |  | 0.407 |
| walker |  | 4708 | 0 | python decl at src/tomli/_parser.py:318 |  |  | 0.407 |
| walker |  | 4708 | 0 | python decl at src/tomli/_parser.py:349 |  |  | 0.407 |
| walker |  | 4719 | 11 | python decl doc at src/tomli/_parser.py:149 |  |  | 0.411 |
| walker |  | 4733 | 14 | python decl doc at src/tomli/_parser.py:220 |  |  | 0.412 |
| walker |  | 4746 | 13 | python decl doc at src/tomli/_parser.py:137 |  |  | 0.418 |
| walker |  | 4778 | 32 | python decl doc at src/tomli/_parser.py:71 |  |  | 0.424 |
| walker |  | 4842 | 64 | python decl at src/tomli/_parser.py:327 |  |  | 0.424 |
| ns | 4897 |  | 395 | parse_value — datetime/number/special-float tail | 3.8 | 3.7 | 0.406 |
| walker |  | 4922 | 80 | python class body at src/tomli/_parser.py:220 |  |  | 0.426 |
| walker |  | 5027 | 105 | python decl doc at src/tomli/_parser.py:76 |  |  | 0.454 |
| walker |  | 5078 | 51 | python decl body at src/tomli/_parser.py:318 body 319 |  |  | 0.454 |
| ns | 5138 |  | 241 | Flags — __init__, add_pending, finalize_pending, unset_all | 3.9 | 2.3 | 0.443 |
| walker |  | 5283 | 205 | python method sigs #1 in src/tomli/_parser.py |  |  | 0.449 |
| walker |  | 5283 | 0 | python method at src/tomli/_parser.py:229 |  |  | 0.449 |
| walker |  | 5283 | 0 | python method at src/tomli/_parser.py:233 |  |  | 0.449 |
| walker |  | 5283 | 0 | python method at src/tomli/_parser.py:236 |  |  | 0.449 |
| walker |  | 5283 | 0 | python method at src/tomli/_parser.py:241 |  |  | 0.449 |
| walker |  | 5283 | 0 | python method at src/tomli/_parser.py:249 |  |  | 0.449 |
| walker |  | 5283 | 0 | python method at src/tomli/_parser.py:260 |  |  | 0.449 |
| walker |  | 5283 | 0 | python method at src/tomli/_parser.py:300 |  |  | 0.449 |
| walker |  | 5283 | 0 | python method at src/tomli/_parser.py:313 |  |  | 0.449 |
| walker |  | 5296 | 13 | python method at src/tomli/_parser.py:279 |  |  | 0.449 |
| walker |  | 5309 | 13 | python method body at src/tomli/_parser.py:233 body 234 |  |  | 0.451 |
| walker |  | 5353 | 44 | python method at src/tomli/_parser.py:283 |  |  | 0.451 |
| walker |  | 5368 | 15 | python method body at src/tomli/_parser.py:279 body 281 |  |  | 0.451 |
| walker |  | 5387 | 19 | python method body at src/tomli/_parser.py:313 body 314 |  |  | 0.460 |
| walker |  | 5468 | 81 | python method at src/tomli/_parser.py:87 |  |  | 0.460 |
| walker |  | 5506 | 38 | python method body at src/tomli/_parser.py:229 body 230 |  |  | 0.464 |
| ns | 5527 |  | 389 | Flags — set + is_ (the actual lookup) | 3.10 | 3.9 | 0.448 |
| walker |  | 5544 | 38 | python method body at src/tomli/_parser.py:236 body 237 |  |  | 0.455 |
| walker |  | 5702 | 158 | python decl at src/tomli/_parser.py:57 |  |  | 0.455 |
| walker |  | 5802 | 100 | python decl body at src/tomli/_parser.py:349 body 350 |  |  | 0.456 |
| walker |  | 5866 | 64 | python method body at src/tomli/_parser.py:241 body 242 |  |  | 0.473 |
| walker |  | 5877 | 11 | python decl body at src/tomli/_parser.py:137 body 139 |  |  | 0.473 |
| ns | 5894 |  | 367 | NestedDict body — table-tree builder | 3.11 | 2.3 | 0.460 |
| walker |  | 6116 | 239 | python decl at src/tomli/_re.py:26 |  |  | 0.460 |
| ns | 6145 |  | 251 | README usage: Decimal floats | 3.12 |  | 0.473 |
| walker |  | 6187 | 71 | tomllib.md section #4 |  |  | 0.473 |
| walker |  | 6293 | 106 | python decl at src/tomli/_re.py:17 |  |  | 0.473 |
| ns | 6395 |  | 250 | create_dict_rule body — [table] header | 4.1 | 2.7 | 0.464 |
| walker |  | 6446 | 153 | python decl body at src/tomli/_parser.py:327 body 335 |  |  | 0.465 |
| walker |  | 6466 | 20 | listing of 'benchmark' |  |  | 0.465 |
| walker |  | 6473 | 7 | README headline in benchmark/README.md |  |  | 0.465 |
| walker |  | 6487 | 14 | python decl body at src/tomli/_parser.py:137 body 146 |  |  | 0.466 |
| walker |  | 6591 | 104 | README.md section #6 |  |  | 0.491 |
| ns | 6679 |  | 284 | create_list_rule body — [[arr]] header | 4.2 | 2.7 | 0.480 |
| walker |  | 6832 | 241 | python decl names surface #2 in src/tomli/_parser.py |  |  | 0.497 |
| walker |  | 6832 | 0 | python decl at src/tomli/_parser.py:361 |  |  | 0.497 |
| walker |  | 6832 | 0 | python decl at src/tomli/_parser.py:370 |  |  | 0.497 |
| walker |  | 6832 | 0 | python decl at src/tomli/_parser.py:390 |  |  | 0.497 |
| walker |  | 6832 | 0 | python decl at src/tomli/_parser.py:463 |  |  | 0.497 |
| walker |  | 6832 | 0 | python decl at src/tomli/_parser.py:481 |  |  | 0.497 |
| walker |  | 6832 | 0 | python decl at src/tomli/_parser.py:497 |  |  | 0.497 |
| walker |  | 6832 | 0 | python decl at src/tomli/_parser.py:595 |  |  | 0.497 |
| walker |  | 6862 | 30 | python decl at src/tomli/_parser.py:564 |  |  | 0.497 |
| walker |  | 6895 | 33 | python decl at src/tomli/_parser.py:413 |  |  | 0.497 |
| walker |  | 6930 | 35 | python decl at src/tomli/_parser.py:502 |  |  | 0.497 |
| walker |  | 6966 | 36 | python decl at src/tomli/_parser.py:447 |  |  | 0.497 |
| walker |  | 6981 | 15 | python decl body at src/tomli/_parser.py:595 body 596 |  |  | 0.497 |
| walker |  | 7018 | 37 | python decl at src/tomli/_parser.py:528 |  |  | 0.497 |
| walker |  | 7042 | 24 | python decl body at src/tomli/_parser.py:497 body 498 |  |  | 0.497 |
| walker |  | 7112 | 70 | python decl body at src/tomli/_parser.py:361 body 362 |  |  | 0.498 |
| ns | 7159 |  | 480 | key_value_rule body | 4.3 | 2.7 | 0.482 |
| walker |  | 7262 | 150 | python decl body at src/tomli/_parser.py:447 body 450 |  |  | 0.482 |
| walker |  | 7371 | 109 | python method body at src/tomli/_parser.py:300 body 301 |  |  | 0.496 |
| ns | 7480 |  | 321 | parse_inline_table — head + first key/value insert | 4.4 | 2.7 | 0.486 |
| walker |  | 7537 | 166 | python decl names surface #3 in src/tomli/_parser.py |  |  | 0.507 |
| walker |  | 7537 | 0 | python decl at src/tomli/_parser.py:599 |  |  | 0.507 |
| walker |  | 7537 | 0 | python decl at src/tomli/_parser.py:612 |  |  | 0.507 |
| walker |  | 7537 | 0 | python decl at src/tomli/_parser.py:621 |  |  | 0.507 |
| walker |  | 7537 | 0 | python decl at src/tomli/_parser.py:652 |  |  | 0.507 |
| walker |  | 7537 | 0 | python decl at src/tomli/_parser.py:760 |  |  | 0.507 |
| walker |  | 7537 | 0 | python decl at src/tomli/_parser.py:764 |  |  | 0.507 |
| walker |  | 7571 | 34 | python decl at src/tomli/_parser.py:684 |  |  | 0.508 |
| walker |  | 7598 | 27 | python decl body at src/tomli/_parser.py:760 body 761 |  |  | 0.508 |
| ns | 7650 |  | 170 | parse_inline_table — comma/close loop tail | 4.5 | 4.4 | 0.502 |
| walker |  | 7686 | 88 | python decl doc at src/tomli/_parser.py:764 |  |  | 0.502 |
| walker |  | 7776 | 90 | python decl body at src/tomli/_parser.py:612 body 613 |  |  | 0.502 |
| walker |  | 7924 | 148 | python decl body at src/tomli/_parser.py:599 body 600 |  |  | 0.502 |
| walker |  | 7932 | 8 | python decl body at src/tomli/_parser.py:652 body 659 |  |  | 0.502 |
| ns | 7937 |  | 287 | parse_array body | 4.6 | 2.7 | 0.491 |
| walker |  | 8042 | 110 | tomllib.md section #6 |  |  | 0.491 |
| walker |  | 8073 | 31 | [package] in pyproject.toml |  |  | 0.492 |
| walker |  | 8083 | 10 | python decl body at src/tomli/_parser.py:652 body 660 |  |  | 0.492 |
| walker |  | 8093 | 10 | python decl body at src/tomli/_parser.py:764 body 782 |  |  | 0.492 |
| walker |  | 8118 | 25 | python decl names surface in benchmark/run.py |  |  | 0.492 |
| walker |  | 8118 | 0 | python decl at benchmark/run.py:37 |  |  | 0.492 |
| walker |  | 8242 | 124 | python method body at src/tomli/_parser.py:283 body 289 |  |  | 0.515 |
| ns | 8290 |  | 353 | parse_basic_str body | 4.7 | 2.7 | 0.502 |
| walker |  | 8396 | 154 | README.md section #3 |  |  | 0.526 |
| walker |  | 8491 | 95 | python decl body at fuzzer/fuzz.py:59 body 62 |  |  | 0.526 |
| ns | 8626 |  | 336 | README usage: tomllib compat shim | 4.8 |  | 0.538 |
| walker |  | 8641 | 150 | python method body at src/tomli/_parser.py:249 body 250 |  |  | 0.545 |
| walker |  | 8654 | 13 | python decl body at src/tomli/_parser.py:463 body 465 |  |  | 0.545 |
| walker |  | 8704 | 50 | headings outline in benchmark/README.md |  |  | 0.545 |
| walker |  | 8719 | 15 | python decl body at src/tomli/_parser.py:463 body 464 |  |  | 0.545 |
| walker |  | 8813 | 94 | tomllib.md section #1 |  |  | 0.545 |
| walker |  | 8823 | 10 | python decl body at src/tomli/_parser.py:502 body 505 |  |  | 0.545 |
| ns | 8921 |  | 295 | README FAQ: type mapping table | 5.1 |  | 0.538 |
| walker |  | 9004 | 181 | README.md section #27 |  |  | 0.538 |
| walker |  | 9021 | 17 | python decl body at src/tomli/_parser.py:463 body 466 |  |  | 0.538 |
| walker |  | 9040 | 19 | python decl body at src/tomli/_parser.py:764 body 773 |  |  | 0.538 |
| walker |  | 9052 | 12 | python decl body at src/tomli/_parser.py:502 body 506 |  |  | 0.538 |
| walker |  | 9072 | 20 | python decl body at src/tomli/_parser.py:764 body 772 |  |  | 0.538 |
| walker |  | 9330 | 258 | README.md section #24 |  |  | 0.557 |
| walker |  | 9339 | 9 | python decl body at src/tomli/_parser.py:528 body 533 |  |  | 0.557 |
| walker |  | 9349 | 10 | python decl body at src/tomli/_parser.py:528 body 531 |  |  | 0.557 |
| ns | 9402 |  | 481 | skip_chars / skip_until / skip_comment / skip_comments_and_array_ws | 5.2 | 2.7 | 0.576 |
| walker |  | 9407 | 58 | python decl at benchmark/run.py:15 |  |  | 0.576 |
| walker |  | 9423 | 16 | python decl body at src/tomli/_parser.py:502 body 508 |  |  | 0.577 |
| walker |  | 9434 | 11 | python decl body at src/tomli/_parser.py:528 body 532 |  |  | 0.577 |
| ns | 9565 |  | 163 | CHANGELOG: 2.4 + 2.1 entries | 5.3 |  | 0.572 |
| ns | 9623 |  | 58 | benchmark/, fuzzer/, profiler/, scripts/, .github/ listings | 5.4 |  | 0.578 |
| walker |  | 9735 | 301 | README.md section #26 |  |  | 0.578 |
| walker |  | 9770 | 35 | python imports in tests/burntsushi.py |  |  | 0.578 |
| walker |  | 9854 | 84 | python decl names surface in tests/burntsushi.py |  |  | 0.578 |
| walker |  | 9854 | 0 | python decl at tests/burntsushi.py:11 |  |  | 0.578 |
| walker |  | 9854 | 0 | python decl at tests/burntsushi.py:42 |  |  | 0.578 |
| walker |  | 9854 | 0 | python decl at tests/burntsushi.py:68 |  |  | 0.578 |
| walker |  | 9854 | 0 | python decl at tests/burntsushi.py:92 |  |  | 0.578 |
| walker |  | 9854 | 0 | python decl at tests/burntsushi.py:96 |  |  | 0.578 |
| walker |  | 9879 | 25 | python decl doc at tests/burntsushi.py:42 |  |  | 0.578 |
| walker |  | 9926 | 47 | benchmark/README.md section #2 |  |  | 0.578 |
| walker |  | 9936 | 10 | python decl body at src/tomli/_parser.py:564 body 568 |  |  | 0.578 |
| walker |  | 9944 | 8 | python method body at src/tomli/_parser.py:260 body 275 |  |  | 0.579 |
| walker |  | 9967 | 23 | python decl body at src/tomli/_parser.py:481 body 490 |  |  | 0.579 |
| ns | 9973 |  | 350 | pyproject.toml [project] block | 5.5 |  | 0.568 |
| walker |  | 9990 | 23 | python decl body at src/tomli/_parser.py:481 body 494 |  |  | 0.568 |

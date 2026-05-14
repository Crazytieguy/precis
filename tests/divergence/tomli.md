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
| ns | 192 |  | 26 | src/tomli/ module listing | 1.4 |  | 0.890 |
| walker |  | 244 | 74 | python imports in src/tomli/__init__.py |  |  | 0.955 |
| ns | 279 |  | 87 | loads / load signatures + docstrings | 1.5 |  | 0.870 |
| walker |  | 299 | 55 | headings outline in tomllib.md |  |  | 0.870 |
| walker |  | 308 | 9 | python imports in fuzzer/fuzz.py |  |  | 0.870 |
| walker |  | 333 | 25 | python imports in setup.py |  |  | 0.870 |
| walker |  | 365 | 32 | python decl at profiler/profiler_script.py:12 |  |  | 0.870 |
| walker |  | 397 | 32 | python decl names surface in src/tomli/_types.py |  |  | 0.873 |
| ns | 403 |  | 124 | TOMLDecodeError class + docstring | 1.6 |  | 0.751 |
| walker |  | 410 | 13 | python imports in src/tomli/_types.py |  |  | 0.754 |
| ns | 433 |  | 30 | tests/ directory listing | 2.1 |  | 0.700 |
| walker |  | 470 | 60 | python decl names surface in fuzzer/fuzz.py |  |  | 0.700 |
| walker |  | 470 | 0 | python decl at fuzzer/fuzz.py:53 |  |  | 0.700 |
| walker |  | 470 | 0 | python decl at fuzzer/fuzz.py:59 |  |  | 0.700 |
| walker |  | 483 | 13 | python decl at fuzzer/fuzz.py:20 |  |  | 0.700 |
| ns | 491 |  | 58 | _types.py — full | 2.2 |  | 0.692 |
| walker |  | 505 | 22 | python decl at fuzzer/fuzz.py:71 |  |  | 0.692 |
| walker |  | 521 | 16 | python decl doc at fuzzer/fuzz.py:59 |  |  | 0.692 |
| walker |  | 551 | 30 | listing of 'tests' |  |  | 0.765 |
| ns | 697 |  | 206 | _parser.py: state-class headers + Flags constants | 2.3 |  | 0.655 |
| ns | 844 |  | 147 | _re.py: regex constants + match-helper locations | 2.4 |  | 0.616 |
| walker |  | 930 | 379 | headings outline in README.md |  |  | 0.617 |
| walker |  | 952 | 22 | README.md section #2 |  |  | 0.617 |
| walker |  | 958 | 6 | README.md section #19 |  |  | 0.617 |
| walker |  | 966 | 8 | README.md section #13 |  |  | 0.617 |
| walker |  | 977 | 11 | README.md section #14 |  |  | 0.617 |
| walker |  | 991 | 14 | README.md section #18 |  |  | 0.617 |
| ns | 1160 |  | 316 | README intro paragraph | 2.5 |  | 0.552 |
| walker |  | 1353 | 362 | README.md section #0 |  |  | 0.562 |
| walker |  | 1428 | 75 | README.md section #25 |  |  | 0.562 |
| walker |  | 1454 | 26 | README.md section #16 |  |  | 0.562 |
| ns | 1486 |  | 326 | README table of contents | 2.6 |  | 0.613 |
| walker |  | 1494 | 40 | python decl body at fuzzer/fuzz.py:71 body 73 |  |  | 0.613 |
| walker |  | 1564 | 70 | python imports in profiler/profiler_script.py |  |  | 0.613 |
| walker |  | 1595 | 31 | README.md section #23 |  |  | 0.613 |
| walker |  | 1757 | 162 | tomllib.md section #0 |  |  | 0.613 |
| walker |  | 1790 | 33 | README.md section #21 |  |  | 0.613 |
| walker |  | 1830 | 40 | README.md section #9 |  |  | 0.613 |
| walker |  | 1887 | 57 | python decl body at fuzzer/fuzz.py:53 body 54 |  |  | 0.613 |
| walker |  | 1890 | 3 | listing of '.github' |  |  | 0.613 |
| walker |  | 1894 | 4 | listing of '.github/workflows' |  |  | 0.613 |
| ns | 1904 |  | 418 | _parser.py: parse_* and skip_* function locations | 2.7 |  | 0.553 |
| walker |  | 1907 | 13 | python decl body at fuzzer/fuzz.py:59 body 61 |  |  | 0.553 |
| walker |  | 1948 | 41 | README.md section #22 |  |  | 0.553 |
| walker |  | 2099 | 151 | python decl names surface in src/tomli/_re.py |  |  | 0.587 |
| walker |  | 2099 | 0 | python decl at src/tomli/_re.py:59 |  |  | 0.587 |
| walker |  | 2099 | 0 | python decl at src/tomli/_re.py:109 |  |  | 0.587 |
| walker |  | 2099 | 0 | python decl at src/tomli/_re.py:116 |  |  | 0.587 |
| ns | 2100 |  | 196 | README usage: parse a TOML string | 2.8 |  | 0.543 |
| walker |  | 2123 | 24 | python decl at src/tomli/_re.py:98 |  |  | 0.555 |
| walker |  | 2157 | 34 | python decl body at src/tomli/_re.py:116 body 117 |  |  | 0.555 |
| walker |  | 2212 | 55 | python decl doc at src/tomli/_re.py:59 |  |  | 0.555 |
| walker |  | 2267 | 55 | README.md section #4 |  |  | 0.556 |
| walker |  | 2323 | 56 | README.md section #5 |  |  | 0.557 |
| walker |  | 2379 | 56 | README.md section #8 |  |  | 0.558 |
| ns | 2392 |  | 292 | README usage: parse a file + handle errors | 2.9 |  | 0.538 |
| walker |  | 2430 | 51 | README.md section #17 |  |  | 0.538 |
| ns | 2491 |  | 99 | load body | 3.1 | 1.5 | 0.525 |
| walker |  | 2513 | 83 | python imports in src/tomli/_re.py |  |  | 0.525 |
| walker |  | 2541 | 28 | tomllib.md section #3 |  |  | 0.525 |
| ns | 2746 |  | 255 | tests/data/{valid,invalid}/ top listing | 3.2 |  | 0.470 |
| walker |  | 2829 | 288 | README.md section #1 |  |  | 0.527 |
| walker |  | 2837 | 8 | CHANGELOG.md section #0 |  |  | 0.527 |
| walker |  | 2908 | 71 | README.md section #10 |  |  | 0.527 |
| walker |  | 2942 | 34 | tomllib.md section #2 |  |  | 0.527 |
| walker |  | 2966 | 24 | python imports in tests/__init__.py |  |  | 0.527 |
| ns | 2972 |  | 226 | tests/* test method names | 3.3 |  | 0.504 |
| walker |  | 3039 | 73 | README.md section #20 |  |  | 0.504 |
| walker |  | 3111 | 72 | python decl body at src/tomli/_re.py:98 body 100 |  |  | 0.504 |
| walker |  | 3186 | 75 | README.md section #15 |  |  | 0.504 |
| walker |  | 3278 | 92 | README.md section #11 |  |  | 0.504 |
| ns | 3320 |  | 348 | loads body — prelude + skip / dispatch comments | 3.4 | 1.5 | 0.475 |
| walker |  | 3428 | 150 | python imports in src/tomli/_parser.py |  |  | 0.475 |
| walker |  | 3511 | 83 | python decl body at src/tomli/_re.py:109 body 110 |  |  | 0.475 |
| ns | 3755 |  | 435 | loads body — rule dispatch + statement terminator | 3.5 | 3.4 | 0.440 |
| walker |  | 3762 | 251 | python decl names surface in src/tomli/_parser.py |  |  | 0.440 |
| walker |  | 3787 | 25 | python decl at src/tomli/_parser.py:51 |  |  | 0.440 |
| walker |  | 3885 | 98 | README.md section #7 |  |  | 0.441 |
| ns | 3977 |  | 222 | TOMLDecodeError.__init__ — pos→line/col body | 3.6 | 1.6 | 0.427 |
| walker |  | 4046 | 161 | python decl at src/tomli/_re.py:46 |  |  | 0.427 |
| walker |  | 4149 | 103 | README.md section #12 |  |  | 0.428 |
| walker |  | 4200 | 51 | tomllib.md section #7 |  |  | 0.428 |
| walker |  | 4263 | 63 | tomllib.md section #5 |  |  | 0.428 |
| walker |  | 4476 | 213 | python decl names surface #1 in src/tomli/_parser.py |  |  | 0.437 |
| walker |  | 4476 | 0 | python decl at src/tomli/_parser.py:71 |  |  | 0.437 |
| walker |  | 4476 | 0 | python decl at src/tomli/_parser.py:76 |  |  | 0.437 |
| walker |  | 4476 | 0 | python decl at src/tomli/_parser.py:137 |  |  | 0.437 |
| walker |  | 4476 | 0 | python decl at src/tomli/_parser.py:149 |  |  | 0.437 |
| walker |  | 4476 | 0 | python decl at src/tomli/_parser.py:220 |  |  | 0.437 |
| walker |  | 4476 | 0 | python decl at src/tomli/_parser.py:278 |  |  | 0.437 |
| walker |  | 4476 | 0 | python decl at src/tomli/_parser.py:312 |  |  | 0.437 |
| walker |  | 4476 | 0 | python decl at src/tomli/_parser.py:318 |  |  | 0.437 |
| walker |  | 4476 | 0 | python decl at src/tomli/_parser.py:349 |  |  | 0.437 |
| walker |  | 4487 | 11 | python decl doc at src/tomli/_parser.py:149 |  |  | 0.441 |
| walker |  | 4501 | 14 | python decl doc at src/tomli/_parser.py:220 |  |  | 0.443 |
| ns | 4502 |  | 525 | parse_value — dispatch head (strings/bools/array/inline-table) | 3.7 | 2.7 | 0.412 |
| walker |  | 4514 | 13 | python decl doc at src/tomli/_parser.py:137 |  |  | 0.418 |
| walker |  | 4546 | 32 | python decl doc at src/tomli/_parser.py:71 |  |  | 0.423 |
| walker |  | 4610 | 64 | python decl at src/tomli/_parser.py:327 |  |  | 0.423 |
| walker |  | 4690 | 80 | python class body at src/tomli/_parser.py:220 |  |  | 0.444 |
| walker |  | 4795 | 105 | python decl doc at src/tomli/_parser.py:76 |  |  | 0.474 |
| walker |  | 4846 | 51 | python decl body at src/tomli/_parser.py:318 body 319 |  |  | 0.474 |
| ns | 4897 |  | 395 | parse_value — datetime/number/special-float tail | 3.8 | 3.7 | 0.454 |
| walker |  | 5051 | 205 | python method sigs #1 in src/tomli/_parser.py |  |  | 0.459 |
| walker |  | 5051 | 0 | python method at src/tomli/_parser.py:229 |  |  | 0.459 |
| walker |  | 5051 | 0 | python method at src/tomli/_parser.py:233 |  |  | 0.459 |
| walker |  | 5051 | 0 | python method at src/tomli/_parser.py:236 |  |  | 0.459 |
| walker |  | 5051 | 0 | python method at src/tomli/_parser.py:241 |  |  | 0.459 |
| walker |  | 5051 | 0 | python method at src/tomli/_parser.py:249 |  |  | 0.459 |
| walker |  | 5051 | 0 | python method at src/tomli/_parser.py:260 |  |  | 0.459 |
| walker |  | 5051 | 0 | python method at src/tomli/_parser.py:300 |  |  | 0.459 |
| walker |  | 5051 | 0 | python method at src/tomli/_parser.py:313 |  |  | 0.459 |
| walker |  | 5064 | 13 | python method at src/tomli/_parser.py:279 |  |  | 0.459 |
| walker |  | 5077 | 13 | python method body at src/tomli/_parser.py:233 body 234 |  |  | 0.459 |
| walker |  | 5121 | 44 | python method at src/tomli/_parser.py:283 |  |  | 0.459 |
| walker |  | 5136 | 15 | python method body at src/tomli/_parser.py:279 body 281 |  |  | 0.459 |
| ns | 5138 |  | 241 | Flags — __init__, add_pending, finalize_pending, unset_all | 3.9 | 2.3 | 0.451 |
| walker |  | 5155 | 19 | python method body at src/tomli/_parser.py:313 body 314 |  |  | 0.459 |
| walker |  | 5236 | 81 | python method at src/tomli/_parser.py:87 |  |  | 0.459 |
| walker |  | 5274 | 38 | python method body at src/tomli/_parser.py:229 body 230 |  |  | 0.463 |
| walker |  | 5312 | 38 | python method body at src/tomli/_parser.py:236 body 237 |  |  | 0.471 |
| walker |  | 5470 | 158 | python decl at src/tomli/_parser.py:57 |  |  | 0.471 |
| ns | 5527 |  | 389 | Flags — set + is_ (the actual lookup) | 3.10 | 3.9 | 0.455 |
| walker |  | 5570 | 100 | python decl body at src/tomli/_parser.py:349 body 350 |  |  | 0.455 |
| walker |  | 5634 | 64 | python method body at src/tomli/_parser.py:241 body 242 |  |  | 0.473 |
| walker |  | 5645 | 11 | python decl body at src/tomli/_parser.py:137 body 139 |  |  | 0.473 |
| walker |  | 5884 | 239 | python decl at src/tomli/_re.py:26 |  |  | 0.473 |
| ns | 5894 |  | 367 | NestedDict body — table-tree builder | 3.11 | 2.3 | 0.460 |
| walker |  | 5955 | 71 | tomllib.md section #4 |  |  | 0.460 |
| walker |  | 5972 | 17 | listing of 'scripts' |  |  | 0.460 |
| walker |  | 5986 | 14 | python decl names surface in scripts/use_setuptools.py |  |  | 0.460 |
| walker |  | 5986 | 0 | python decl at scripts/use_setuptools.py:12 |  |  | 0.460 |
| walker |  | 6092 | 106 | python decl at src/tomli/_re.py:17 |  |  | 0.460 |
| ns | 6145 |  | 251 | README usage: Decimal floats | 3.12 |  | 0.473 |
| walker |  | 6245 | 153 | python decl body at src/tomli/_parser.py:327 body 335 |  |  | 0.474 |
| walker |  | 6265 | 20 | listing of 'benchmark' |  |  | 0.474 |
| walker |  | 6272 | 7 | README headline in benchmark/README.md |  |  | 0.474 |
| walker |  | 6286 | 14 | python decl body at src/tomli/_parser.py:137 body 146 |  |  | 0.475 |
| walker |  | 6390 | 104 | README.md section #6 |  |  | 0.501 |
| ns | 6395 |  | 250 | create_dict_rule body — [table] header | 4.1 | 2.7 | 0.491 |
| walker |  | 6631 | 241 | python decl names surface #2 in src/tomli/_parser.py |  |  | 0.508 |
| walker |  | 6631 | 0 | python decl at src/tomli/_parser.py:361 |  |  | 0.508 |
| walker |  | 6631 | 0 | python decl at src/tomli/_parser.py:370 |  |  | 0.508 |
| walker |  | 6631 | 0 | python decl at src/tomli/_parser.py:390 |  |  | 0.508 |
| walker |  | 6631 | 0 | python decl at src/tomli/_parser.py:463 |  |  | 0.508 |
| walker |  | 6631 | 0 | python decl at src/tomli/_parser.py:481 |  |  | 0.508 |
| walker |  | 6631 | 0 | python decl at src/tomli/_parser.py:497 |  |  | 0.508 |
| walker |  | 6631 | 0 | python decl at src/tomli/_parser.py:595 |  |  | 0.508 |
| walker |  | 6661 | 30 | python decl at src/tomli/_parser.py:564 |  |  | 0.508 |
| ns | 6679 |  | 284 | create_list_rule body — [[arr]] header | 4.2 | 2.7 | 0.497 |
| walker |  | 6694 | 33 | python decl at src/tomli/_parser.py:413 |  |  | 0.497 |
| walker |  | 6729 | 35 | python decl at src/tomli/_parser.py:502 |  |  | 0.497 |
| walker |  | 6765 | 36 | python decl at src/tomli/_parser.py:447 |  |  | 0.497 |
| walker |  | 6780 | 15 | python decl body at src/tomli/_parser.py:595 body 596 |  |  | 0.497 |
| walker |  | 6817 | 37 | python decl at src/tomli/_parser.py:528 |  |  | 0.497 |
| walker |  | 6841 | 24 | python decl body at src/tomli/_parser.py:497 body 498 |  |  | 0.497 |
| walker |  | 6911 | 70 | python decl body at src/tomli/_parser.py:361 body 362 |  |  | 0.498 |
| walker |  | 7061 | 150 | python decl body at src/tomli/_parser.py:447 body 450 |  |  | 0.498 |
| ns | 7159 |  | 480 | key_value_rule body | 4.3 | 2.7 | 0.482 |
| walker |  | 7170 | 109 | python method body at src/tomli/_parser.py:300 body 301 |  |  | 0.496 |
| walker |  | 7336 | 166 | python decl names surface #3 in src/tomli/_parser.py |  |  | 0.518 |
| walker |  | 7336 | 0 | python decl at src/tomli/_parser.py:599 |  |  | 0.518 |
| walker |  | 7336 | 0 | python decl at src/tomli/_parser.py:612 |  |  | 0.518 |
| walker |  | 7336 | 0 | python decl at src/tomli/_parser.py:621 |  |  | 0.518 |
| walker |  | 7336 | 0 | python decl at src/tomli/_parser.py:652 |  |  | 0.518 |
| walker |  | 7336 | 0 | python decl at src/tomli/_parser.py:760 |  |  | 0.518 |
| walker |  | 7336 | 0 | python decl at src/tomli/_parser.py:764 |  |  | 0.518 |
| walker |  | 7370 | 34 | python decl at src/tomli/_parser.py:684 |  |  | 0.518 |
| walker |  | 7397 | 27 | python decl body at src/tomli/_parser.py:760 body 761 |  |  | 0.518 |
| ns | 7480 |  | 321 | parse_inline_table — head + first key/value insert | 4.4 | 2.7 | 0.508 |
| walker |  | 7485 | 88 | python decl doc at src/tomli/_parser.py:764 |  |  | 0.508 |
| walker |  | 7575 | 90 | python decl body at src/tomli/_parser.py:612 body 613 |  |  | 0.508 |
| ns | 7650 |  | 170 | parse_inline_table — comma/close loop tail | 4.5 | 4.4 | 0.502 |
| walker |  | 7723 | 148 | python decl body at src/tomli/_parser.py:599 body 600 |  |  | 0.502 |
| walker |  | 7731 | 8 | python decl body at src/tomli/_parser.py:652 body 659 |  |  | 0.502 |
| walker |  | 7841 | 110 | tomllib.md section #6 |  |  | 0.502 |
| walker |  | 7872 | 31 | [package] in pyproject.toml |  |  | 0.502 |
| walker |  | 7882 | 10 | python decl body at src/tomli/_parser.py:652 body 660 |  |  | 0.502 |
| walker |  | 7892 | 10 | python decl body at src/tomli/_parser.py:764 body 782 |  |  | 0.502 |
| walker |  | 7917 | 25 | python decl names surface in benchmark/run.py |  |  | 0.502 |
| walker |  | 7917 | 0 | python decl at benchmark/run.py:37 |  |  | 0.502 |
| ns | 7937 |  | 287 | parse_array body | 4.6 | 2.7 | 0.492 |
| walker |  | 8041 | 124 | python method body at src/tomli/_parser.py:283 body 289 |  |  | 0.515 |
| walker |  | 8195 | 154 | README.md section #3 |  |  | 0.540 |
| walker |  | 8290 | 95 | python decl body at fuzzer/fuzz.py:59 body 62 |  |  | 0.526 |
| ns | 8290 |  | 353 | parse_basic_str body | 4.7 | 2.7 | 0.526 |
| walker |  | 8440 | 150 | python method body at src/tomli/_parser.py:249 body 250 |  |  | 0.533 |
| walker |  | 8453 | 13 | python decl body at src/tomli/_parser.py:463 body 465 |  |  | 0.533 |
| walker |  | 8503 | 50 | headings outline in benchmark/README.md |  |  | 0.533 |
| walker |  | 8518 | 15 | python decl body at src/tomli/_parser.py:463 body 464 |  |  | 0.533 |
| walker |  | 8612 | 94 | tomllib.md section #1 |  |  | 0.533 |
| walker |  | 8622 | 10 | python decl body at src/tomli/_parser.py:502 body 505 |  |  | 0.534 |
| ns | 8626 |  | 336 | README usage: tomllib compat shim | 4.8 |  | 0.545 |
| walker |  | 8803 | 181 | README.md section #27 |  |  | 0.545 |
| walker |  | 8820 | 17 | python decl body at src/tomli/_parser.py:463 body 466 |  |  | 0.545 |
| walker |  | 8839 | 19 | python decl body at src/tomli/_parser.py:764 body 773 |  |  | 0.545 |
| walker |  | 8851 | 12 | python decl body at src/tomli/_parser.py:502 body 506 |  |  | 0.546 |
| walker |  | 8871 | 20 | python decl body at src/tomli/_parser.py:764 body 772 |  |  | 0.546 |
| ns | 8921 |  | 295 | README FAQ: type mapping table | 5.1 |  | 0.538 |
| walker |  | 9129 | 258 | README.md section #24 |  |  | 0.557 |
| walker |  | 9138 | 9 | python decl body at src/tomli/_parser.py:528 body 533 |  |  | 0.557 |
| walker |  | 9148 | 10 | python decl body at src/tomli/_parser.py:528 body 531 |  |  | 0.557 |
| walker |  | 9206 | 58 | python decl at benchmark/run.py:15 |  |  | 0.557 |
| walker |  | 9222 | 16 | python decl body at src/tomli/_parser.py:502 body 508 |  |  | 0.558 |
| walker |  | 9233 | 11 | python decl body at src/tomli/_parser.py:528 body 532 |  |  | 0.558 |
| ns | 9402 |  | 481 | skip_chars / skip_until / skip_comment / skip_comments_and_array_ws | 5.2 | 2.7 | 0.577 |
| walker |  | 9534 | 301 | README.md section #26 |  |  | 0.577 |
| ns | 9565 |  | 163 | CHANGELOG: 2.4 + 2.1 entries | 5.3 |  | 0.572 |
| walker |  | 9569 | 35 | python imports in tests/burntsushi.py |  |  | 0.572 |
| ns | 9623 |  | 58 | benchmark/, fuzzer/, profiler/, scripts/, .github/ listings | 5.4 |  | 0.578 |
| walker |  | 9653 | 84 | python decl names surface in tests/burntsushi.py |  |  | 0.578 |
| walker |  | 9653 | 0 | python decl at tests/burntsushi.py:11 |  |  | 0.578 |
| walker |  | 9653 | 0 | python decl at tests/burntsushi.py:42 |  |  | 0.578 |
| walker |  | 9653 | 0 | python decl at tests/burntsushi.py:68 |  |  | 0.578 |
| walker |  | 9653 | 0 | python decl at tests/burntsushi.py:92 |  |  | 0.578 |
| walker |  | 9653 | 0 | python decl at tests/burntsushi.py:96 |  |  | 0.578 |
| walker |  | 9678 | 25 | python decl doc at tests/burntsushi.py:42 |  |  | 0.578 |
| walker |  | 9684 | 6 | listing of 'tests/data' |  |  | 0.578 |
| walker |  | 9731 | 47 | benchmark/README.md section #2 |  |  | 0.578 |
| walker |  | 9741 | 10 | python decl body at src/tomli/_parser.py:564 body 568 |  |  | 0.578 |
| walker |  | 9749 | 8 | python method body at src/tomli/_parser.py:260 body 275 |  |  | 0.579 |
| walker |  | 9772 | 23 | python decl body at src/tomli/_parser.py:481 body 490 |  |  | 0.579 |
| walker |  | 9795 | 23 | python decl body at src/tomli/_parser.py:481 body 494 |  |  | 0.579 |
| walker |  | 9869 | 74 | python decl body at src/tomli/_parser.py:137 body 140 |  |  | 0.587 |
| walker |  | 9885 | 16 | python decl body at src/tomli/_parser.py:528 body 535 |  |  | 0.588 |
| walker |  | 9909 | 24 | python test names surface in tests/test_data.py |  |  | 0.588 |
| walker |  | 9934 | 25 | python decl body at src/tomli/_parser.py:481 body 492 |  |  | 0.588 |
| walker |  | 9959 | 25 | python decl body at src/tomli/_parser.py:502 body 509 |  |  | 0.589 |
| ns | 9973 |  | 350 | pyproject.toml [project] block | 5.5 |  | 0.578 |

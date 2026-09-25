Score(3000)=0.416 I=0.754 C=0.230 ns_rows≤3K=18/54 grid(1000/1442/2080/3000/4327/6240/9000)=0.575/0.468/0.515/0.416/0.495/0.503/0.482

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 29 |  | 29 | README title and tagline | 1.1 |  | 0.000 |
| walker |  | 62 | 62 | listing of '.' |  |  | 0.000 |
| walker |  | 66 | 4 | listing of 'src' |  |  | 0.000 |
| walker |  | 69 | 3 | listing of '.github' |  |  | 0.000 |
| walker |  | 73 | 4 | listing of '.github/workflows' |  |  | 0.000 |
| ns | 91 |  | 62 | Root directory listing (complete) | 1.2 |  | 0.660 |
| walker |  | 99 | 26 | listing of 'src/tomli' |  |  | 0.691 |
| walker |  | 138 | 39 | python module doc src/tomli/__init__.py |  |  | 0.695 |
| ns | 172 |  | 81 | The complete public API: src/tomli/__init__.py | 1.3 |  | 0.633 |
| walker |  | 175 | 37 | python names src/tomli/__init__.py |  |  | 0.689 |
| walker |  | 183 | 8 | listing of 'fuzzer' |  |  | 0.690 |
| walker |  | 192 | 9 | listing of 'profiler' |  |  | 0.690 |
| ns | 270 |  | 98 | README intro lede: a TOML parser, TOML v1.1.0 as of 2.4.0 | 1.4 |  | 0.644 |
| walker |  | 297 | 105 | [package] in pyproject.toml |  |  | 0.646 |
| ns | 300 |  | 30 | Source package listing: src/ and src/tomli/ | 1.5 |  | 0.648 |
| walker |  | 314 | 17 | listing of 'scripts' |  |  | 0.649 |
| walker |  | 372 | 58 | headings outline in tomllib.md |  |  | 0.650 |
| walker |  | 392 | 20 | listing of 'benchmark' |  |  | 0.653 |
| ns | 408 |  | 108 | README: the tomllib stdlib relationship | 1.6 |  | 0.592 |
| walker |  | 426 | 34 | python names src/tomli/_types.py |  |  | 0.596 |
| walker |  | 456 | 30 | listing of 'tests' |  |  | 0.598 |
| ns | 496 |  | 88 | Entry points: load() and loads() signatures with docstrings | 1.7 |  | 0.564 |
| walker |  | 516 | 60 | package metadata in pyproject.toml |  |  | 0.566 |
| ns | 622 |  | 126 | TOMLDecodeError class docstring and its attributes | 1.8 |  | 0.501 |
| ns | 682 |  | 60 | src/tomli/_types.py type aliases (complete file body) | 1.9 |  | 0.483 |
| ns | 766 |  | 84 | README mypyc/pure-Python distribution note | 1.10 |  | 0.465 |
| walker |  | 909 | 393 | README headline in README.md |  |  | 0.657 |
| ns | 972 |  | 206 | README section map: every H2 and H3 heading | 1.11 |  | 0.575 |
| walker |  | 1150 | 241 | python names src/tomli/_parser.py |  |  | 0.577 |
| ns | 1220 |  | 248 | Complete roster of module-level functions in _parser.py (names only) | 2.1 |  | 0.500 |
| ns | 1340 |  | 120 | Complete roster of classes in _parser.py, with Output's fields | 2.2 |  | 0.468 |
| walker |  | 1535 | 385 | headings outline in README.md |  |  | 0.587 |
| walker |  | 1544 | 9 | README.md section #0 |  |  | 0.587 |
| walker |  | 1569 | 25 | README.md section #2 |  |  | 0.587 |
| ns | 1576 |  | 236 | loads(): parse state setup and the statement-loop rule enumeration | 2.3 | 1.7 | 0.536 |
| ns | 1719 |  | 143 | parse_value() signature and the inline-nesting recursion guard | 2.4 | 2.1 | 0.513 |
| walker |  | 1797 | 228 | python names src/tomli/_parser.py #1 |  |  | 0.529 |
| walker |  | 1808 | 11 | python decl src/tomli/_parser.py:76 |  |  | 0.529 |
| walker |  | 1822 | 14 | python decl src/tomli/_parser.py:312 |  |  | 0.533 |
| walker |  | 1845 | 23 | python decl src/tomli/_parser.py:51 |  |  | 0.533 |
| walker |  | 1895 | 50 | python decl src/tomli/_parser.py:278 |  |  | 0.535 |
| ns | 1924 |  | 205 | MAX_INLINE_NESTING and the mypyc stack-overflow rationale | 2.5 | 2.4 | 0.515 |
| walker |  | 1941 | 46 | python decl src/tomli/_parser.py:283 |  |  | 0.515 |
| walker |  | 2097 | 156 | python decl src/tomli/_parser.py:57 |  |  | 0.518 |
| walker |  | 2180 | 83 | python decl src/tomli/_parser.py:87 |  |  | 0.518 |
| ns | 2357 |  | 433 | loads(): the statement dispatch body and its two top-level errors | 2.6 | 2.3 | 0.454 |
| walker |  | 2395 | 215 | python decl src/tomli/_parser.py:220 |  |  | 0.457 |
| walker |  | 2570 | 175 | python names src/tomli/_re.py |  |  | 0.457 |
| walker |  | 2582 | 12 | python decl src/tomli/_re.py:98 |  |  | 0.458 |
| ns | 2734 |  | 377 | parse_value(): string, boolean, array and inline-table dispatch | 2.7 | 2.4 | 0.416 |
| walker |  | 2741 | 159 | python decl src/tomli/_re.py:46 |  |  | 0.416 |
| walker |  | 2758 | 17 | python names tests/__init__.py |  |  | 0.416 |
| walker |  | 2792 | 34 | python body src/tomli/_re.py:116 |  |  | 0.416 |
| walker |  | 3029 | 237 | python decl src/tomli/_re.py:26 |  |  | 0.418 |
| ns | 3122 |  | 388 | parse_value(): datetime, number and special-float dispatch | 2.8 | 2.4 | 0.390 |
| walker |  | 3239 | 210 | python names src/tomli/_parser.py #2 |  |  | 0.411 |
| walker |  | 3274 | 35 | python decl src/tomli/_parser.py:413 |  |  | 0.412 |
| walker |  | 3312 | 38 | python decl src/tomli/_parser.py:447 |  |  | 0.412 |
| walker |  | 3378 | 66 | python decl src/tomli/_parser.py:327 |  |  | 0.412 |
| ns | 3453 |  | 331 | Character-class constants: the complete set | 2.9 |  | 0.439 |
| ns | 3550 |  | 97 | load() body: the binary-mode requirement | 2.10 | 1.7 | 0.432 |
| walker |  | 3553 | 175 | tomllib.md section #0 |  |  | 0.432 |
| walker |  | 3562 | 9 | README headline in benchmark/README.md |  |  | 0.432 |
| walker |  | 3568 | 6 | listing of 'tests/data' |  |  | 0.432 |
| ns | 3662 |  | 112 | loads() prologue: CRLF normalisation and the str type check | 2.11 | 1.7 | 0.425 |
| walker |  | 3672 | 104 | python decl src/tomli/_re.py:17 |  |  | 0.425 |
| ns | 3836 |  | 174 | BASIC_STR_ESCAPE_REPLACEMENTS: the full escape table | 2.12 |  | 0.449 |
| ns | 3872 |  | 36 | tests/ and tests/data/ listings (complete) | 3.1 |  | 0.463 |
| walker |  | 3899 | 227 | python names src/tomli/_parser.py #3 |  |  | 0.514 |
| walker |  | 3931 | 32 | python decl src/tomli/_parser.py:564 |  |  | 0.510 |
| ns | 3931 |  | 59 | tests/__init__.py: the tomli-as-tomllib alias | 3.2 |  | 0.510 |
| walker |  | 3967 | 36 | python decl src/tomli/_parser.py:684 |  |  | 0.512 |
| walker |  | 4004 | 37 | python decl src/tomli/_parser.py:502 |  |  | 0.512 |
| walker |  | 4043 | 39 | python decl src/tomli/_parser.py:528 |  |  | 0.512 |
| ns | 4195 |  | 264 | Complete roster of test classes and test methods | 3.3 |  | 0.495 |
| walker |  | 4231 | 188 | README.md section #6 |  |  | 0.495 |
| walker |  | 4361 | 130 | README.md section #7 |  |  | 0.495 |
| ns | 4371 |  | 176 | tox core configuration: the interpreter matrix and default command | 3.4 |  | 0.486 |
| ns | 4523 |  | 152 | setup.py: the mypyc build path | 3.5 |  | 0.474 |
| walker |  | 4662 | 301 | README.md section #1 |  |  | 0.518 |
| ns | 4717 |  | 194 | pyproject.toml: build backend and project metadata | 3.6 |  | 0.524 |
| ns | 4775 |  | 58 | Listings of the helper trees: benchmark, fuzzer, profiler, scripts, workflows | 3.7 |  | 0.539 |
| walker |  | 4904 | 242 | README.md section #3 |  |  | 0.539 |
| ns | 4958 |  | 183 | test_data.py: how the TOML corpus is discovered and compared | 3.8 | 3.3 | 0.528 |
| walker |  | 5080 | 176 | README.md section #4 |  |  | 0.528 |
| walker |  | 5094 | 14 | python names profiler/profiler_script.py |  |  | 0.528 |
| ns | 5097 |  | 139 | tests/burntsushi.py: purpose and complete function roster | 3.9 |  | 0.521 |
| walker |  | 5109 | 15 | python names tests/test_error.py |  |  | 0.521 |
| walker |  | 5171 | 62 | python doc src/tomli/_re.py:59 |  |  | 0.522 |
| walker |  | 5187 | 16 | python names scripts/use_setuptools.py |  |  | 0.522 |
| walker |  | 5203 | 16 | python names tests/test_misc.py |  |  | 0.522 |
| ns | 5348 |  | 251 | Flags: the two flag constants, the state fields, and the complete method roster | 4.1 | 2.2 | 0.526 |
| ns | 5472 |  | 124 | NestedDict: the parsed-document container and its two methods | 4.2 | 2.2 | 0.527 |
| walker |  | 5544 | 341 | README.md section #8 |  |  | 0.527 |
| walker |  | 5718 | 174 | tomllib.md section #1 |  |  | 0.527 |
| ns | 5733 |  | 261 | create_dict_rule(): the [table] statement | 4.3 | 2.1 | 0.515 |
| ns | 6033 |  | 300 | create_list_rule(): the [[array of tables]] statement | 4.4 | 2.1 | 0.502 |
| walker |  | 6071 | 353 | tomllib.md section #2 |  |  | 0.503 |
| walker |  | 6141 | 70 | python body src/tomli/_re.py:98 |  |  | 0.503 |
| ns | 6511 |  | 478 | key_value_rule(): dotted keys, pending flags and immutability | 4.5 | 2.1 | 0.485 |
| ns | 6690 |  | 179 | parse_key_part(): bare, literal and basic key forms | 4.6 | 2.1 | 0.478 |
| ns | 6975 |  | 285 | parse_array(): array literals and trailing commas | 4.7 | 2.1 | 0.466 |
| ns | 7464 |  | 489 | parse_inline_table(): inline tables and their local flag scope | 4.8 | 2.1 | 0.451 |
| ns | 7665 |  | 201 | parse_hex_char() and is_unicode_scalar_value(): escape validation | 4.9 | 2.1 | 0.446 |
| ns | 7807 |  | 142 | parse_literal_str() and parse_one_line_basic_str(): the two short string entry points | 4.10 | 2.1 | 0.443 |
| walker |  | 7951 | 1810 | manifest config in pyproject.toml |  |  | 0.467 |
| walker |  | 7978 | 27 | python names benchmark/run.py |  |  | 0.467 |
| ns | 8041 |  | 234 | make_safe_parse_float(): the parse_float contract | 4.11 | 2.1 | 0.459 |
| ns | 8133 |  | 92 | TOMLDecodeError.__init__ signature and the deprecated free-form form | 5.1 | 1.8 | 0.465 |
| ns | 8360 |  | 227 | TOMLDecodeError: line/column computation and message formatting | 5.2 | 5.1 | 0.457 |
| walker |  | 8370 | 392 | README.md section #9 |  |  | 0.457 |
| ns | 8535 |  | 175 | Complete roster of src/tomli/_re.py: four regexes and four functions | 5.3 |  | 0.469 |
| walker |  | 8554 | 184 | README.md section #10 |  |  | 0.469 |
| ns | 8772 |  | 237 | RE_NUMBER: the integer and float grammar | 5.4 | 5.3 | 0.482 |
| ns | 8901 |  | 129 | match_to_datetime docstring and the cached_tz cache-size note | 5.5 | 5.3 | 0.482 |
| walker |  | 9070 | 516 | README.md section #5 |  |  | 0.482 |
| walker |  | 9102 | 32 | python decl profiler/profiler_script.py:12 |  |  | 0.482 |
| walker |  | 9110 | 8 | plaintext config scripts/requirements.txt |  |  | 0.482 |
| ns | 9133 |  | 232 | Complete roster of tox environments with their descriptions | 6.1 | 3.4 | 0.490 |
| walker |  | 9193 | 83 | python body src/tomli/_re.py:109 |  |  | 0.491 |
| walker |  | 9202 | 9 | plaintext config profiler/requirements.txt |  |  | 0.491 |
| walker |  | 9215 | 13 | python doc src/tomli/_parser.py:149 |  |  | 0.493 |
| walker |  | 9229 | 14 | python doc src/tomli/_parser.py:220 |  |  | 0.496 |
| walker |  | 9291 | 62 | python names fuzzer/fuzz.py |  |  | 0.496 |
| ns | 9339 |  | 206 | tomllib.md: section map and the CPython sync procedure | 6.2 |  | 0.503 |
| walker |  | 9351 | 60 | python decl benchmark/run.py:15 |  |  | 0.503 |
| walker |  | 9366 | 15 | python doc src/tomli/_parser.py:137 |  |  | 0.506 |
| walker |  | 9381 | 15 | python body src/tomli/_parser.py:595 |  |  | 0.506 |
| walker |  | 9432 | 51 | headings outline in benchmark/README.md |  |  | 0.506 |
| ns | 9473 |  | 134 | CHANGELOG.md: the two most recent releases | 6.3 |  | 0.500 |
| walker |  | 9518 | 86 | python names tests/burntsushi.py |  |  | 0.503 |
| walker |  | 9542 | 24 | python body src/tomli/_parser.py:497 |  |  | 0.505 |
| walker |  | 9555 | 13 | python body src/tomli/_parser.py:233 |  |  | 0.506 |
| walker |  | 9582 | 27 | python body src/tomli/_parser.py:760 |  |  | 0.506 |
| walker |  | 9614 | 32 | python doc src/tomli/_parser.py:71 |  |  | 0.513 |
| walker |  | 9633 | 19 | python body src/tomli/_parser.py:313 |  |  | 0.518 |
| ns | 9638 |  | 165 | CI workflow: the complete job list and test matrix | 6.4 |  | 0.514 |
| walker |  | 9684 | 51 | python body src/tomli/_parser.py:318 |  |  | 0.514 |
| walker |  | 9712 | 28 | python body src/tomli/_parser.py:279 |  |  | 0.519 |
| walker |  | 9782 | 70 | python body src/tomli/_parser.py:361 |  |  | 0.519 |
| ns | 9809 |  | 171 | pre-commit: the complete list of hook ids | 6.5 |  | 0.514 |
| walker |  | 9820 | 38 | python body src/tomli/_parser.py:229 |  |  | 0.519 |
| walker |  | 9858 | 38 | python body src/tomli/_parser.py:236 |  |  | 0.520 |
| walker |  | 9948 | 90 | python body src/tomli/_parser.py:612 |  |  | 0.528 |
| ns | 9994 |  | 185 | Lint and version-bump configuration: .flake8 and .bumpversion.cfg | 6.6 |  | 0.522 |

Score(3000)=0.454 I=0.722 C=0.285 ns_rows≤3K=29/43 (reached=10 partial=1 missing=18)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 8 |  | 8 | fuzzer/ listing | 1.1 |  | 0.000 |
| ns | 17 |  | 9 | profiler/ listing | 1.2 |  | 0.000 |
| ns | 34 |  | 17 | scripts/ listing | 1.3 |  | 0.000 |
| ns | 60 |  | 26 | src/tomli/ listing | 1.4 |  | 0.000 |
| walker |  | 69 | 69 | listing of '.' |  |  | 0.000 |
| walker |  | 73 | 4 | listing of 'src' |  |  | 0.000 |
| walker |  | 76 | 3 | listing of '.github' |  |  | 0.000 |
| walker |  | 79 | 3 | listing of '.github/workflows' |  |  | 0.000 |
| ns | 80 |  | 20 | benchmark/ listing | 1.5 |  | 0.000 |
| walker |  | 104 | 25 | listing of 'src/tomli' |  |  | 0.274 |
| ns | 111 |  | 31 | tests/ top-level listing | 1.6 |  | 0.226 |
| ns | 144 |  | 33 | tests/data/invalid/dotted-keys/ listing | 1.7 |  | 0.205 |
| ns | 215 |  | 71 | External toml-test invalid/ category listing | 1.8 |  | 0.156 |
| walker |  | 234 | 130 | python imports in src/tomli/__init__.py |  |  | 0.163 |
| walker |  | 241 | 7 | listing of 'fuzzer' |  |  | 0.308 |
| walker |  | 249 | 8 | listing of 'profiler' |  |  | 0.389 |
| ns | 279 |  | 64 | Repo root listing | 1.9 |  | 0.546 |
| ns | 287 |  | 8 | MANIFEST.in (full, 1 line) | 1.10 |  | 0.541 |
| walker |  | 307 | 58 | headings outline in tomllib.md |  |  | 0.541 |
| walker |  | 323 | 16 | listing of 'scripts' |  |  | 0.615 |
| ns | 341 |  | 54 | _parser.py roster: entry points + error class | 1.11 |  | 0.587 |
| walker |  | 342 | 19 | listing of 'benchmark' |  |  | 0.661 |
| walker |  | 372 | 30 | listing of 'tests' |  |  | 0.741 |
| ns | 419 |  | 78 | CI workflow job roster | 1.12 |  | 0.698 |
| ns | 466 |  | 47 | _re.py function roster | 1.13 |  | 0.676 |
| ns | 584 |  | 118 | pyproject.toml tox environment roster | 1.14 |  | 0.638 |
| ns | 714 |  | 130 | tomli/__init__.py (full public API) | 2.1 |  | 0.660 |
| walker |  | 765 | 393 | README headline in README.md |  |  | 0.660 |
| ns | 836 |  | 122 | _parser.py roster: table/array/key rules | 2.2 |  | 0.618 |
| walker |  | 864 | 99 | manifest config in pyproject.toml |  |  | 0.618 |
| ns | 963 |  | 127 | pyproject.toml mypy strict config | 2.3 |  | 0.579 |
| walker |  | 965 | 101 | [package] in pyproject.toml |  |  | 0.581 |
| walker |  | 997 | 32 | python imports in setup.py |  |  | 0.581 |
| ns | 1101 |  | 138 | _parser.py roster: value parsing | 2.4 |  | 0.548 |
| walker |  | 1124 | 127 | tool.isort config in pyproject.toml |  |  | 0.548 |
| ns | 1243 |  | 142 | _parser.py roster: namespace state machinery | 2.5 |  | 0.518 |
| walker |  | 1274 | 150 | tool.coverage config in pyproject.toml |  |  | 0.518 |
| ns | 1414 |  | 171 | .pre-commit-config.yaml hook-id roster | 2.6 |  | 0.494 |
| ns | 1520 |  | 106 | tests/data/valid/ listing | 2.7 |  | 0.464 |
| walker |  | 1659 | 385 | headings outline in README.md |  |  | 0.464 |
| walker |  | 1668 | 9 | README.md section #0 |  |  | 0.464 |
| walker |  | 1674 | 6 | README.md section #19 |  |  | 0.464 |
| ns | 1685 |  | 165 | tests/data/invalid/ listing | 2.8 |  | 0.431 |
| walker |  | 1699 | 25 | README.md section #2 |  |  | 0.431 |
| walker |  | 1707 | 8 | README.md section #13 |  |  | 0.431 |
| ns | 1710 |  | 25 | Sample invalid TOML fixture (dotted-keys/extend-defined-aot) | 2.9 |  | 0.427 |
| walker |  | 1718 | 11 | README.md section #14 |  |  | 0.427 |
| walker |  | 1732 | 14 | README.md section #18 |  |  | 0.427 |
| walker |  | 1760 | 28 | README.md section #16 |  |  | 0.427 |
| ns | 1949 |  | 239 | Test method roster (test_data/test_error/test_misc) | 2.10 |  | 0.405 |
| walker |  | 2032 | 272 | tool.mypy config in pyproject.toml |  |  | 0.457 |
| ns | 2058 |  | 109 | _types.py (full) | 2.11 |  | 0.445 |
| walker |  | 2115 | 83 | README.md section #25 |  |  | 0.445 |
| walker |  | 2146 | 31 | README.md section #23 |  |  | 0.445 |
| walker |  | 2179 | 33 | README.md section #21 |  |  | 0.445 |
| ns | 2213 |  | 155 | Sample valid TOML/JSON fixture pair (array-subtables) | 3.1 |  | 0.425 |
| ns | 2321 |  | 108 | tests/__init__.py (tomllib aliasing) | 3.2 |  | 0.417 |
| walker |  | 2354 | 175 | tomllib.md section #0 |  |  | 0.418 |
| walker |  | 2394 | 40 | README.md section #9 |  |  | 0.418 |
| walker |  | 2433 | 39 | README.md section #22 |  |  | 0.418 |
| walker |  | 2442 | 9 | README headline in benchmark/README.md |  |  | 0.418 |
| walker |  | 2449 | 7 | listing of 'tests/data' |  |  | 0.418 |
| ns | 2562 |  | 241 | External toml-test valid/ category listing | 3.3 |  | 0.385 |
| ns | 2756 |  | 194 | pyproject.toml build-system + core [project] identity | 3.4 |  | 0.394 |
| walker |  | 2913 | 464 | python decl names surface in src/tomli/_parser.py |  |  | 0.452 |
| walker |  | 2962 | 49 | README.md section #17 |  |  | 0.452 |
| walker |  | 2996 | 34 | python decl names surface in src/tomli/_types.py |  |  | 0.454 |
| walker |  | 3052 | 56 | README.md section #5 |  |  | 0.454 |
| ns | 3081 |  | 325 | README intro | 3.5 |  | 0.439 |
| walker |  | 3108 | 56 | README.md section #8 |  |  | 0.439 |
| walker |  | 3166 | 58 | README.md section #4 |  |  | 0.439 |
| walker |  | 3196 | 30 | tomllib.md section #3 |  |  | 0.439 |
| ns | 3302 |  | 221 | README usage: parse a TOML string | 4.1 |  | 0.425 |
| walker |  | 3497 | 301 | README.md section #1 |  |  | 0.466 |
| walker |  | 3529 | 32 | tomllib.md section #2 |  |  | 0.466 |
| walker |  | 3600 | 71 | README.md section #10 |  |  | 0.466 |
| walker |  | 3610 | 10 | CHANGELOG.md section #0 |  |  | 0.466 |
| ns | 3633 |  | 331 | tomllib.md - intro + sync status | 4.2 |  | 0.475 |
| walker |  | 3681 | 71 | README.md section #20 |  |  | 0.475 |
| walker |  | 3754 | 73 | README.md section #15 |  |  | 0.475 |
| walker |  | 3915 | 161 | python decl names surface in src/tomli/_re.py |  |  | 0.482 |
| walker |  | 3915 | 0 | python decl at src/tomli/_re.py:59 |  |  | 0.482 |
| walker |  | 3915 | 0 | python decl at src/tomli/_re.py:109 |  |  | 0.482 |
| walker |  | 3915 | 0 | python decl at src/tomli/_re.py:116 |  |  | 0.482 |
| ns | 3930 |  | 297 | README: TOML->Python type mapping table | 4.3 |  | 0.470 |
| walker |  | 3941 | 26 | python decl at src/tomli/_re.py:98 |  |  | 0.474 |
| walker |  | 3975 | 34 | python decl body at src/tomli/_re.py:116 body 117 |  |  | 0.474 |
| walker |  | 4037 | 62 | python decl doc at src/tomli/_re.py:59 |  |  | 0.474 |
| ns | 4069 |  | 139 | CHANGELOG.md - latest two releases | 4.4 |  | 0.463 |
| ns | 4301 |  | 232 | burntsushi.py: convert() (partial) | 4.5 |  | 0.451 |
| ns | 4600 |  | 299 | test_misc.py: test_parse_float | 4.6 | 2.10 | 0.435 |
| ns | 5139 |  | 539 | test_data.py (full, data-driven glob mechanism) | 4.7 | 2.10 | 0.410 |
| walker |  | 5165 | 1128 | tool.tox config in pyproject.toml |  |  | 0.428 |
| walker |  | 5229 | 64 | python imports in src/tomli/_types.py |  |  | 0.440 |
| walker |  | 5326 | 97 | README.md section #11 |  |  | 0.440 |
| walker |  | 5396 | 70 | python decl body at src/tomli/_re.py:98 body 100 |  |  | 0.440 |
| walker |  | 5497 | 101 | README.md section #7 |  |  | 0.440 |
| walker |  | 5548 | 51 | tomllib.md section #7 |  |  | 0.440 |
| walker |  | 5707 | 159 | python decl at src/tomli/_re.py:46 |  |  | 0.441 |
| ns | 5817 |  | 678 | Flags class (namespace mutability tracking) | 5.1 | 2.5 | 0.414 |
| walker |  | 5823 | 116 | README.md section #12 |  |  | 0.414 |
| walker |  | 5982 | 159 | python imports in src/tomli/_parser.py |  |  | 0.414 |
| walker |  | 6085 | 103 | python imports in src/tomli/_re.py |  |  | 0.414 |
| walker |  | 6202 | 117 | README.md section #6 |  |  | 0.414 |
| walker |  | 6278 | 76 | tomllib.md section #4 |  |  | 0.414 |
| walker |  | 6515 | 237 | python decl at src/tomli/_re.py:26 |  |  | 0.416 |
| ns | 6537 |  | 720 | TOMLDecodeError (+ deprecation sentinel) full body | 5.2 | 1.11 | 0.390 |
| walker |  | 6619 | 104 | python decl at src/tomli/_re.py:17 |  |  | 0.391 |
| walker |  | 6702 | 83 | tomllib.md section #5 |  |  | 0.391 |
| walker |  | 6779 | 77 | python imports in tests/__init__.py |  |  | 0.397 |
| walker |  | 6951 | 172 | README.md section #3 |  |  | 0.429 |
| walker |  | 6959 | 8 | plaintext config scripts/requirements.txt |  |  | 0.429 |
| walker |  | 7143 | 184 | README.md section #27 |  |  | 0.429 |
| walker |  | 7152 | 9 | plaintext config profiler/requirements.txt |  |  | 0.429 |
| ns | 7271 |  | 734 | _re.py regex definitions (RE_NUMBER/RE_LOCALTIME/RE_DATETIME) | 5.3 |  | 0.462 |
| walker |  | 7280 | 128 | tomllib.md section #6 |  |  | 0.462 |
| walker |  | 7363 | 83 | python decl body at src/tomli/_re.py:109 body 110 |  |  | 0.462 |
| walker |  | 7414 | 51 | headings outline in benchmark/README.md |  |  | 0.462 |
| walker |  | 7675 | 261 | README.md section #24 |  |  | 0.483 |
| walker |  | 7772 | 97 | tomllib.md section #1 |  |  | 0.490 |
| walker |  | 8076 | 304 | README.md section #26 |  |  | 0.490 |
| ns | 8189 |  | 918 | parse_value() type dispatch | 5.4 | 2.4 | 0.462 |
| walker |  | 9133 | 1057 | YAML config at .pre-commit-config.yaml |  |  | 0.478 |
| walker |  | 9144 | 11 | python imports in fuzzer/fuzz.py |  |  | 0.478 |
| ns | 9150 |  | 961 | loads()/load() entry point + parse loop | 5.5 | 1.11 | 0.451 |
| walker |  | 9249 | 105 | listing of 'tests/data/valid' |  |  | 0.471 |
| walker |  | 9254 | 5 | listing of 'tests/data/valid/_external' |  |  | 0.471 |
| walker |  | 9260 | 6 | listing of 'tests/data/valid/_external/toml-test' |  |  | 0.471 |
| walker |  | 9281 | 21 | listing of 'tests/data/valid/dates-and-times' |  |  | 0.471 |
| walker |  | 9306 | 25 | listing of 'tests/data/valid/array' |  |  | 0.471 |
| walker |  | 9331 | 25 | listing of 'tests/data/valid/inline-table' |  |  | 0.471 |
| walker |  | 9358 | 27 | listing of 'tests/data/valid/multiline-basic-str' |  |  | 0.471 |
| walker |  | 9363 | 5 | json config tests/data/valid/no-newlines.json |  |  | 0.471 |
| walker |  | 9389 | 26 | python test names surface in tests/test_data.py |  |  | 0.471 |
| walker |  | 9433 | 44 | plaintext config fuzzer/requirements.txt |  |  | 0.471 |
| walker |  | 9598 | 165 | listing of 'tests/data/invalid' |  |  | 0.498 |
| walker |  | 9603 | 5 | listing of 'tests/data/invalid/_external' |  |  | 0.498 |
| walker |  | 9608 | 5 | listing of 'tests/data/invalid/dates-and-times' |  |  | 0.498 |
| walker |  | 9613 | 5 | listing of 'tests/data/invalid/literal-str' |  |  | 0.498 |
| walker |  | 9619 | 6 | listing of 'tests/data/invalid/_external/toml-test' |  |  | 0.498 |
| walker |  | 9633 | 14 | listing of 'tests/data/invalid/multiline-literal-str' |  |  | 0.498 |
| walker |  | 9650 | 17 | listing of 'tests/data/invalid/array-of-tables' |  |  | 0.498 |
| walker |  | 9667 | 17 | listing of 'tests/data/invalid/boolean' |  |  | 0.498 |
| walker |  | 9687 | 20 | listing of 'tests/data/invalid/table' |  |  | 0.498 |
| walker |  | 9709 | 22 | listing of 'tests/data/invalid/array' |  |  | 0.498 |
| walker |  | 9741 | 32 | listing of 'tests/data/invalid/dotted-keys' |  |  | 0.509 |
| walker |  | 9780 | 39 | listing of 'tests/data/invalid/keys-and-vals' |  |  | 0.509 |
| walker |  | 9821 | 41 | listing of 'tests/data/invalid/multiline-basic-str' |  |  | 0.509 |
| walker |  | 9909 | 88 | listing of 'tests/data/invalid/inline-table' |  |  | 0.509 |
| walker |  | 9923 | 14 | python decl names surface in profiler/profiler_script.py |  |  | 0.509 |
| walker |  | 9955 | 32 | python decl at profiler/profiler_script.py:12 |  |  | 0.509 |
| walker |  | 9982 | 27 | python decl names surface in benchmark/run.py |  |  | 0.509 |
| walker |  | 9982 | 0 | python decl at benchmark/run.py:37 |  |  | 0.509 |
| ns | 10032 |  | 882 | key_value_rule / parse_key_value_pair / parse_key (full bodies) | 5.6 | 2.2 | 0.487 |

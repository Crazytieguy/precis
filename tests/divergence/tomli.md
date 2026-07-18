Score(3000)=0.454 I=0.722 C=0.285 ns_rows≤3K=29/43 (reached=10 partial=1 missing=18)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 8 |  | 8 | fuzzer/ listing | 1.1 |  | 0.000 |
| ns | 17 |  | 9 | profiler/ listing | 1.2 |  | 0.000 |
| ns | 34 |  | 17 | scripts/ listing | 1.3 |  | 0.000 |
| ns | 60 |  | 26 | src/tomli/ listing | 1.4 |  | 0.000 |
| walker |  | 62 | 62 | listing of '.' |  |  | 0.000 |
| walker |  | 66 | 4 | listing of 'src' |  |  | 0.000 |
| walker |  | 69 | 3 | listing of '.github' |  |  | 0.000 |
| walker |  | 73 | 4 | listing of '.github/workflows' |  |  | 0.000 |
| ns | 80 |  | 20 | benchmark/ listing | 1.5 |  | 0.000 |
| walker |  | 99 | 26 | listing of 'src/tomli' |  |  | 0.274 |
| ns | 110 |  | 30 | tests/ top-level listing | 1.6 |  | 0.226 |
| ns | 143 |  | 33 | tests/data/invalid/dotted-keys/ listing | 1.7 |  | 0.205 |
| ns | 199 |  | 56 | External toml-test invalid/ category listing | 1.8 |  | 0.156 |
| walker |  | 229 | 130 | python imports in src/tomli/__init__.py |  |  | 0.163 |
| ns | 261 |  | 62 | Repo root listing | 1.9 |  | 0.285 |
| ns | 269 |  | 8 | MANIFEST.in (full, 1 line) | 1.10 |  | 0.282 |
| walker |  | 287 | 58 | headings outline in tomllib.md |  |  | 0.282 |
| walker |  | 295 | 8 | listing of 'fuzzer' |  |  | 0.469 |
| walker |  | 304 | 9 | listing of 'profiler' |  |  | 0.541 |
| ns | 323 |  | 54 | _parser.py roster: entry points + error class | 1.11 |  | 0.516 |
| ns | 401 |  | 78 | CI workflow job roster | 1.12 |  | 0.486 |
| ns | 448 |  | 47 | _re.py function roster | 1.13 |  | 0.471 |
| ns | 566 |  | 118 | pyproject.toml tox environment roster | 1.14 |  | 0.444 |
| ns | 696 |  | 130 | tomli/__init__.py (full public API) | 2.1 |  | 0.478 |
| walker |  | 697 | 393 | README headline in README.md |  |  | 0.478 |
| walker |  | 714 | 17 | listing of 'scripts' |  |  | 0.537 |
| walker |  | 734 | 20 | listing of 'benchmark' |  |  | 0.597 |
| walker |  | 764 | 30 | listing of 'tests' |  |  | 0.660 |
| ns | 818 |  | 122 | _parser.py roster: table/array/key rules | 2.2 |  | 0.618 |
| walker |  | 863 | 99 | manifest config in pyproject.toml |  |  | 0.618 |
| ns | 945 |  | 127 | pyproject.toml mypy strict config | 2.3 |  | 0.579 |
| walker |  | 964 | 101 | [package] in pyproject.toml |  |  | 0.581 |
| walker |  | 996 | 32 | python imports in setup.py |  |  | 0.581 |
| ns | 1083 |  | 138 | _parser.py roster: value parsing | 2.4 |  | 0.548 |
| walker |  | 1123 | 127 | tool.isort config in pyproject.toml |  |  | 0.548 |
| ns | 1225 |  | 142 | _parser.py roster: namespace state machinery | 2.5 |  | 0.518 |
| walker |  | 1273 | 150 | tool.coverage config in pyproject.toml |  |  | 0.518 |
| ns | 1396 |  | 171 | .pre-commit-config.yaml hook-id roster | 2.6 |  | 0.494 |
| ns | 1497 |  | 101 | tests/data/valid/ listing | 2.7 |  | 0.464 |
| ns | 1651 |  | 154 | tests/data/invalid/ listing | 2.8 |  | 0.431 |
| walker |  | 1658 | 385 | headings outline in README.md |  |  | 0.431 |
| walker |  | 1667 | 9 | README.md section #0 |  |  | 0.431 |
| walker |  | 1673 | 6 | README.md section #19 |  |  | 0.431 |
| ns | 1676 |  | 25 | Sample invalid TOML fixture (dotted-keys/extend-defined-aot) | 2.9 |  | 0.427 |
| walker |  | 1698 | 25 | README.md section #2 |  |  | 0.427 |
| walker |  | 1706 | 8 | README.md section #13 |  |  | 0.427 |
| walker |  | 1717 | 11 | README.md section #14 |  |  | 0.427 |
| walker |  | 1731 | 14 | README.md section #18 |  |  | 0.427 |
| walker |  | 1759 | 28 | README.md section #16 |  |  | 0.427 |
| ns | 1915 |  | 239 | Test method roster (test_data/test_error/test_misc) | 2.10 |  | 0.405 |
| ns | 2024 |  | 109 | _types.py (full) | 2.11 |  | 0.395 |
| walker |  | 2031 | 272 | tool.mypy config in pyproject.toml |  |  | 0.445 |
| walker |  | 2114 | 83 | README.md section #25 |  |  | 0.445 |
| walker |  | 2148 | 34 | python decl names surface in src/tomli/_types.py |  |  | 0.448 |
| walker |  | 2179 | 31 | README.md section #23 |  |  | 0.428 |
| ns | 2179 |  | 155 | Sample valid TOML/JSON fixture pair (array-subtables) | 3.1 |  | 0.428 |
| walker |  | 2212 | 33 | README.md section #21 |  |  | 0.428 |
| ns | 2287 |  | 108 | tests/__init__.py (tomllib aliasing) | 3.2 |  | 0.419 |
| walker |  | 2387 | 175 | tomllib.md section #0 |  |  | 0.421 |
| walker |  | 2427 | 40 | README.md section #9 |  |  | 0.421 |
| walker |  | 2466 | 39 | README.md section #22 |  |  | 0.421 |
| walker |  | 2472 | 6 | listing of 'tests/data' |  |  | 0.421 |
| walker |  | 2481 | 9 | README headline in benchmark/README.md |  |  | 0.421 |
| ns | 2517 |  | 230 | External toml-test valid/ category listing | 3.3 |  | 0.387 |
| ns | 2711 |  | 194 | pyproject.toml build-system + core [project] identity | 3.4 |  | 0.397 |
| walker |  | 2945 | 464 | python decl names surface in src/tomli/_parser.py |  |  | 0.454 |
| walker |  | 2994 | 49 | README.md section #17 |  |  | 0.454 |
| ns | 3036 |  | 325 | README intro | 3.5 |  | 0.439 |
| walker |  | 3050 | 56 | README.md section #5 |  |  | 0.439 |
| walker |  | 3106 | 56 | README.md section #8 |  |  | 0.439 |
| walker |  | 3164 | 58 | README.md section #4 |  |  | 0.439 |
| walker |  | 3194 | 30 | tomllib.md section #3 |  |  | 0.439 |
| ns | 3257 |  | 221 | README usage: parse a TOML string | 4.1 |  | 0.425 |
| walker |  | 3495 | 301 | README.md section #1 |  |  | 0.466 |
| walker |  | 3527 | 32 | tomllib.md section #2 |  |  | 0.466 |
| ns | 3588 |  | 331 | tomllib.md - intro + sync status | 4.2 |  | 0.475 |
| walker |  | 3598 | 71 | README.md section #10 |  |  | 0.475 |
| walker |  | 3608 | 10 | CHANGELOG.md section #0 |  |  | 0.475 |
| walker |  | 3679 | 71 | README.md section #20 |  |  | 0.475 |
| walker |  | 3752 | 73 | README.md section #15 |  |  | 0.475 |
| ns | 3885 |  | 297 | README: TOML->Python type mapping table | 4.3 |  | 0.463 |
| walker |  | 3913 | 161 | python decl names surface in src/tomli/_re.py |  |  | 0.470 |
| walker |  | 3913 | 0 | python decl at src/tomli/_re.py:59 |  |  | 0.470 |
| walker |  | 3913 | 0 | python decl at src/tomli/_re.py:109 |  |  | 0.470 |
| walker |  | 3913 | 0 | python decl at src/tomli/_re.py:116 |  |  | 0.470 |
| walker |  | 3939 | 26 | python decl at src/tomli/_re.py:98 |  |  | 0.474 |
| walker |  | 3973 | 34 | python decl body at src/tomli/_re.py:116 body 117 |  |  | 0.474 |
| ns | 4024 |  | 139 | CHANGELOG.md - latest two releases | 4.4 |  | 0.463 |
| walker |  | 4035 | 62 | python decl doc at src/tomli/_re.py:59 |  |  | 0.463 |
| ns | 4256 |  | 232 | burntsushi.py: convert() (partial) | 4.5 |  | 0.451 |
| ns | 4555 |  | 299 | test_misc.py: test_parse_float | 4.6 | 2.10 | 0.435 |
| ns | 5094 |  | 539 | test_data.py (full, data-driven glob mechanism) | 4.7 | 2.10 | 0.410 |
| walker |  | 5163 | 1128 | tool.tox config in pyproject.toml |  |  | 0.428 |
| walker |  | 5227 | 64 | python imports in src/tomli/_types.py |  |  | 0.440 |
| walker |  | 5324 | 97 | README.md section #11 |  |  | 0.440 |
| walker |  | 5394 | 70 | python decl body at src/tomli/_re.py:98 body 100 |  |  | 0.440 |
| walker |  | 5495 | 101 | README.md section #7 |  |  | 0.440 |
| walker |  | 5546 | 51 | tomllib.md section #7 |  |  | 0.440 |
| walker |  | 5705 | 159 | python decl at src/tomli/_re.py:46 |  |  | 0.441 |
| ns | 5772 |  | 678 | Flags class (namespace mutability tracking) | 5.1 | 2.5 | 0.414 |
| walker |  | 5821 | 116 | README.md section #12 |  |  | 0.414 |
| walker |  | 5980 | 159 | python imports in src/tomli/_parser.py |  |  | 0.414 |
| walker |  | 6083 | 103 | python imports in src/tomli/_re.py |  |  | 0.414 |
| walker |  | 6200 | 117 | README.md section #6 |  |  | 0.414 |
| walker |  | 6276 | 76 | tomllib.md section #4 |  |  | 0.414 |
| ns | 6492 |  | 720 | TOMLDecodeError (+ deprecation sentinel) full body | 5.2 | 1.11 | 0.389 |
| walker |  | 6513 | 237 | python decl at src/tomli/_re.py:26 |  |  | 0.390 |
| walker |  | 6617 | 104 | python decl at src/tomli/_re.py:17 |  |  | 0.391 |
| walker |  | 6700 | 83 | tomllib.md section #5 |  |  | 0.391 |
| walker |  | 6777 | 77 | python imports in tests/__init__.py |  |  | 0.397 |
| walker |  | 6949 | 172 | README.md section #3 |  |  | 0.429 |
| walker |  | 6963 | 14 | python decl names surface in profiler/profiler_script.py |  |  | 0.429 |
| walker |  | 6971 | 8 | plaintext config scripts/requirements.txt |  |  | 0.429 |
| walker |  | 7155 | 184 | README.md section #27 |  |  | 0.429 |
| walker |  | 7171 | 16 | python decl names surface in scripts/use_setuptools.py |  |  | 0.429 |
| walker |  | 7171 | 0 | python decl at scripts/use_setuptools.py:12 |  |  | 0.429 |
| walker |  | 7180 | 9 | plaintext config profiler/requirements.txt |  |  | 0.429 |
| ns | 7226 |  | 734 | _re.py regex definitions (RE_NUMBER/RE_LOCALTIME/RE_DATETIME) | 5.3 |  | 0.462 |
| walker |  | 7308 | 128 | tomllib.md section #6 |  |  | 0.462 |
| walker |  | 7391 | 83 | python decl body at src/tomli/_re.py:109 body 110 |  |  | 0.462 |
| walker |  | 7442 | 51 | headings outline in benchmark/README.md |  |  | 0.462 |
| walker |  | 7703 | 261 | README.md section #24 |  |  | 0.483 |
| walker |  | 7800 | 97 | tomllib.md section #1 |  |  | 0.490 |
| walker |  | 7827 | 27 | python decl names surface in benchmark/run.py |  |  | 0.490 |
| walker |  | 7827 | 0 | python decl at benchmark/run.py:37 |  |  | 0.490 |
| walker |  | 8131 | 304 | README.md section #26 |  |  | 0.490 |
| ns | 8144 |  | 918 | parse_value() type dispatch | 5.4 | 2.4 | 0.462 |
| ns | 9105 |  | 961 | loads()/load() entry point + parse loop | 5.5 | 1.11 | 0.436 |
| walker |  | 9188 | 1057 | YAML config at .pre-commit-config.yaml |  |  | 0.451 |
| walker |  | 9220 | 32 | python decl at profiler/profiler_script.py:12 |  |  | 0.451 |
| walker |  | 9231 | 11 | python imports in fuzzer/fuzz.py |  |  | 0.451 |
| walker |  | 9332 | 101 | listing of 'tests/data/valid' |  |  | 0.471 |
| walker |  | 9337 | 5 | listing of 'tests/data/valid/_external' |  |  | 0.471 |
| walker |  | 9343 | 6 | listing of 'tests/data/valid/_external/toml-test' |  |  | 0.471 |
| walker |  | 9365 | 22 | listing of 'tests/data/valid/dates-and-times' |  |  | 0.471 |
| walker |  | 9391 | 26 | listing of 'tests/data/valid/array' |  |  | 0.471 |
| walker |  | 9417 | 26 | listing of 'tests/data/valid/inline-table' |  |  | 0.471 |
| walker |  | 9445 | 28 | listing of 'tests/data/valid/multiline-basic-str' |  |  | 0.471 |
| walker |  | 9450 | 5 | json config tests/data/valid/no-newlines.json |  |  | 0.471 |
| walker |  | 9510 | 60 | python decl at benchmark/run.py:15 |  |  | 0.471 |
| walker |  | 9570 | 60 | python decl names surface in fuzzer/fuzz.py |  |  | 0.471 |
| walker |  | 9570 | 0 | python decl at fuzzer/fuzz.py:53 |  |  | 0.471 |
| walker |  | 9570 | 0 | python decl at fuzzer/fuzz.py:59 |  |  | 0.471 |
| walker |  | 9585 | 15 | python decl at fuzzer/fuzz.py:20 |  |  | 0.471 |
| walker |  | 9609 | 24 | python decl at fuzzer/fuzz.py:71 |  |  | 0.471 |
| walker |  | 9627 | 18 | python decl doc at fuzzer/fuzz.py:59 |  |  | 0.471 |
| walker |  | 9781 | 154 | listing of 'tests/data/invalid' |  |  | 0.498 |
| walker |  | 9786 | 5 | listing of 'tests/data/invalid/_external' |  |  | 0.498 |
| walker |  | 9792 | 6 | listing of 'tests/data/invalid/dates-and-times' |  |  | 0.498 |
| walker |  | 9798 | 6 | listing of 'tests/data/invalid/literal-str' |  |  | 0.498 |
| walker |  | 9804 | 6 | listing of 'tests/data/invalid/_external/toml-test' |  |  | 0.498 |
| walker |  | 9819 | 15 | listing of 'tests/data/invalid/multiline-literal-str' |  |  | 0.498 |
| walker |  | 9837 | 18 | listing of 'tests/data/invalid/array-of-tables' |  |  | 0.498 |
| walker |  | 9855 | 18 | listing of 'tests/data/invalid/boolean' |  |  | 0.498 |
| walker |  | 9876 | 21 | listing of 'tests/data/invalid/table' |  |  | 0.498 |
| walker |  | 9899 | 23 | listing of 'tests/data/invalid/array' |  |  | 0.498 |
| walker |  | 9932 | 33 | listing of 'tests/data/invalid/dotted-keys' |  |  | 0.508 |
| walker |  | 9972 | 40 | listing of 'tests/data/invalid/keys-and-vals' |  |  | 0.508 |
| ns | 9987 |  | 882 | key_value_rule / parse_key_value_pair / parse_key (full bodies) | 5.6 | 2.2 | 0.486 |

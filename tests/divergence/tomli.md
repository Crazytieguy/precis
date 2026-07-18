Score(3000)=0.428 I=0.712 C=0.257 ns_rows≤3K=29/43 (reached=10 partial=1 missing=18)

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
| ns | 110 |  | 30 | tests/ top-level listing | 1.6 |  | 0.000 |
| walker |  | 131 | 58 | headings outline in tomllib.md |  |  | 0.000 |
| ns | 143 |  | 33 | tests/data/invalid/dotted-keys/ listing | 1.7 |  | 0.000 |
| walker |  | 157 | 26 | listing of 'src/tomli' |  |  | 0.205 |
| ns | 199 |  | 56 | External toml-test invalid/ category listing | 1.8 |  | 0.156 |
| ns | 261 |  | 62 | Repo root listing | 1.9 |  | 0.272 |
| ns | 269 |  | 8 | MANIFEST.in (full, 1 line) | 1.10 |  | 0.269 |
| walker |  | 287 | 130 | python imports in src/tomli/__init__.py |  |  | 0.282 |
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
| ns | 945 |  | 127 | pyproject.toml mypy strict config | 2.3 |  | 0.579 |
| walker |  | 1024 | 260 | python decl names surface in src/tomli/_parser.py |  |  | 0.628 |
| ns | 1083 |  | 138 | _parser.py roster: value parsing | 2.4 |  | 0.593 |
| walker |  | 1129 | 105 | [package] in pyproject.toml |  |  | 0.593 |
| walker |  | 1161 | 32 | python imports in setup.py |  |  | 0.593 |
| ns | 1225 |  | 142 | _parser.py roster: namespace state machinery | 2.5 |  | 0.563 |
| ns | 1396 |  | 171 | .pre-commit-config.yaml hook-id roster | 2.6 |  | 0.537 |
| ns | 1497 |  | 101 | tests/data/valid/ listing | 2.7 |  | 0.505 |
| walker |  | 1546 | 385 | headings outline in README.md |  |  | 0.505 |
| walker |  | 1555 | 9 | README.md section #0 |  |  | 0.505 |
| walker |  | 1561 | 6 | README.md section #19 |  |  | 0.505 |
| walker |  | 1586 | 25 | README.md section #2 |  |  | 0.505 |
| walker |  | 1594 | 8 | README.md section #13 |  |  | 0.505 |
| walker |  | 1605 | 11 | README.md section #14 |  |  | 0.505 |
| walker |  | 1619 | 14 | README.md section #18 |  |  | 0.505 |
| ns | 1651 |  | 154 | tests/data/invalid/ listing | 2.8 |  | 0.469 |
| ns | 1676 |  | 25 | Sample invalid TOML fixture (dotted-keys/extend-defined-aot) | 2.9 |  | 0.465 |
| walker |  | 1884 | 265 | python decl names surface #1 in src/tomli/_parser.py |  |  | 0.517 |
| walker |  | 1912 | 28 | README.md section #16 |  |  | 0.517 |
| ns | 1915 |  | 239 | Test method roster (test_data/test_error/test_misc) | 2.10 |  | 0.490 |
| walker |  | 1995 | 83 | README.md section #25 |  |  | 0.490 |
| ns | 2024 |  | 109 | _types.py (full) | 2.11 |  | 0.478 |
| walker |  | 2029 | 34 | python decl names surface in src/tomli/_types.py |  |  | 0.480 |
| walker |  | 2060 | 31 | README.md section #23 |  |  | 0.480 |
| walker |  | 2093 | 33 | README.md section #21 |  |  | 0.480 |
| ns | 2179 |  | 155 | Sample valid TOML/JSON fixture pair (array-subtables) | 3.1 |  | 0.459 |
| walker |  | 2268 | 175 | tomllib.md section #0 |  |  | 0.461 |
| ns | 2287 |  | 108 | tests/__init__.py (tomllib aliasing) | 3.2 |  | 0.451 |
| walker |  | 2308 | 40 | README.md section #9 |  |  | 0.451 |
| walker |  | 2347 | 39 | README.md section #22 |  |  | 0.451 |
| walker |  | 2356 | 9 | README headline in benchmark/README.md |  |  | 0.451 |
| walker |  | 2405 | 49 | README.md section #17 |  |  | 0.451 |
| ns | 2517 |  | 230 | External toml-test valid/ category listing | 3.3 |  | 0.415 |
| ns | 2711 |  | 194 | pyproject.toml build-system + core [project] identity | 3.4 |  | 0.413 |
| walker |  | 2786 | 381 | python decl names surface #2 in src/tomli/_parser.py |  |  | 0.428 |
| walker |  | 2786 | 0 | python decl at src/tomli/_parser.py:71 |  |  | 0.428 |
| walker |  | 2786 | 0 | python decl at src/tomli/_parser.py:76 |  |  | 0.428 |
| walker |  | 2786 | 0 | python decl at src/tomli/_parser.py:137 |  |  | 0.428 |
| walker |  | 2786 | 0 | python decl at src/tomli/_parser.py:149 |  |  | 0.428 |
| walker |  | 2786 | 0 | python decl at src/tomli/_parser.py:220 |  |  | 0.428 |
| walker |  | 2786 | 0 | python decl at src/tomli/_parser.py:278 |  |  | 0.428 |
| walker |  | 2786 | 0 | python decl at src/tomli/_parser.py:312 |  |  | 0.428 |
| walker |  | 2786 | 0 | python decl at src/tomli/_parser.py:318 |  |  | 0.428 |
| walker |  | 2786 | 0 | python decl at src/tomli/_parser.py:349 |  |  | 0.428 |
| walker |  | 2786 | 0 | python decl at src/tomli/_parser.py:361 |  |  | 0.428 |
| walker |  | 2786 | 0 | python decl at src/tomli/_parser.py:370 |  |  | 0.428 |
| walker |  | 2786 | 0 | python decl at src/tomli/_parser.py:390 |  |  | 0.428 |
| walker |  | 2786 | 0 | python decl at src/tomli/_parser.py:463 |  |  | 0.428 |
| walker |  | 2786 | 0 | python decl at src/tomli/_parser.py:481 |  |  | 0.428 |
| walker |  | 2786 | 0 | python decl at src/tomli/_parser.py:497 |  |  | 0.428 |
| walker |  | 2786 | 0 | python decl at src/tomli/_parser.py:595 |  |  | 0.428 |
| walker |  | 2786 | 0 | python decl at src/tomli/_parser.py:599 |  |  | 0.428 |
| walker |  | 2786 | 0 | python decl at src/tomli/_parser.py:612 |  |  | 0.428 |
| walker |  | 2786 | 0 | python decl at src/tomli/_parser.py:621 |  |  | 0.428 |
| walker |  | 2786 | 0 | python decl at src/tomli/_parser.py:652 |  |  | 0.428 |
| walker |  | 2786 | 0 | python decl at src/tomli/_parser.py:760 |  |  | 0.428 |
| walker |  | 2786 | 0 | python decl at src/tomli/_parser.py:764 |  |  | 0.428 |
| walker |  | 2799 | 13 | python decl doc at src/tomli/_parser.py:149 |  |  | 0.428 |
| walker |  | 2815 | 16 | python decl doc at src/tomli/_parser.py:220 |  |  | 0.428 |
| walker |  | 2838 | 23 | python decl at src/tomli/_parser.py:51 |  |  | 0.428 |
| walker |  | 2870 | 32 | python decl at src/tomli/_parser.py:564 |  |  | 0.428 |
| walker |  | 2885 | 15 | python decl doc at src/tomli/_parser.py:137 |  |  | 0.428 |
| walker |  | 2920 | 35 | python decl at src/tomli/_parser.py:413 |  |  | 0.428 |
| walker |  | 2956 | 36 | python decl at src/tomli/_parser.py:684 |  |  | 0.428 |
| walker |  | 2993 | 37 | python decl at src/tomli/_parser.py:502 |  |  | 0.428 |
| walker |  | 3031 | 38 | python decl at src/tomli/_parser.py:447 |  |  | 0.428 |
| ns | 3036 |  | 325 | README intro | 3.5 |  | 0.414 |
| walker |  | 3070 | 39 | python decl at src/tomli/_parser.py:528 |  |  | 0.414 |
| walker |  | 3102 | 32 | python decl doc at src/tomli/_parser.py:71 |  |  | 0.414 |
| walker |  | 3168 | 66 | python decl at src/tomli/_parser.py:327 |  |  | 0.414 |
| walker |  | 3248 | 80 | python class body at src/tomli/_parser.py:220 |  |  | 0.414 |
| ns | 3257 |  | 221 | README usage: parse a TOML string | 4.1 |  | 0.401 |
| walker |  | 3451 | 203 | python method sigs in src/tomli/_parser.py |  |  | 0.441 |
| walker |  | 3451 | 0 | python method at src/tomli/_parser.py:229 |  |  | 0.441 |
| walker |  | 3451 | 0 | python method at src/tomli/_parser.py:233 |  |  | 0.441 |
| walker |  | 3451 | 0 | python method at src/tomli/_parser.py:236 |  |  | 0.441 |
| walker |  | 3451 | 0 | python method at src/tomli/_parser.py:241 |  |  | 0.441 |
| walker |  | 3451 | 0 | python method at src/tomli/_parser.py:249 |  |  | 0.441 |
| walker |  | 3451 | 0 | python method at src/tomli/_parser.py:260 |  |  | 0.441 |
| walker |  | 3451 | 0 | python method at src/tomli/_parser.py:300 |  |  | 0.441 |
| walker |  | 3451 | 0 | python method at src/tomli/_parser.py:313 |  |  | 0.441 |
| walker |  | 3466 | 15 | python method at src/tomli/_parser.py:279 |  |  | 0.441 |
| walker |  | 3512 | 46 | python method at src/tomli/_parser.py:283 |  |  | 0.441 |
| ns | 3588 |  | 331 | tomllib.md - intro + sync status | 4.2 |  | 0.452 |
| walker |  | 3595 | 83 | python method at src/tomli/_parser.py:87 |  |  | 0.452 |
| walker |  | 3705 | 110 | python decl doc at src/tomli/_parser.py:76 |  |  | 0.453 |
| walker |  | 3800 | 95 | python decl doc at src/tomli/_parser.py:764 |  |  | 0.453 |
| walker |  | 3856 | 56 | README.md section #5 |  |  | 0.453 |
| ns | 3885 |  | 297 | README: TOML->Python type mapping table | 4.3 |  | 0.441 |
| walker |  | 3912 | 56 | README.md section #8 |  |  | 0.441 |
| walker |  | 3970 | 58 | README.md section #4 |  |  | 0.441 |
| walker |  | 3976 | 6 | listing of 'tests/data' |  |  | 0.441 |
| walker |  | 4006 | 30 | tomllib.md section #3 |  |  | 0.441 |
| ns | 4024 |  | 139 | CHANGELOG.md - latest two releases | 4.4 |  | 0.432 |
| walker |  | 4162 | 156 | python decl at src/tomli/_parser.py:57 |  |  | 0.432 |
| ns | 4256 |  | 232 | burntsushi.py: convert() (partial) | 4.5 |  | 0.420 |
| walker |  | 4463 | 301 | README.md section #1 |  |  | 0.455 |
| walker |  | 4495 | 32 | tomllib.md section #2 |  |  | 0.455 |
| ns | 4555 |  | 299 | test_misc.py: test_parse_float | 4.6 | 2.10 | 0.439 |
| walker |  | 4566 | 71 | README.md section #10 |  |  | 0.439 |
| walker |  | 4576 | 10 | CHANGELOG.md section #0 |  |  | 0.439 |
| walker |  | 4647 | 71 | README.md section #20 |  |  | 0.439 |
| walker |  | 4720 | 73 | README.md section #15 |  |  | 0.439 |
| walker |  | 4881 | 161 | python decl names surface in src/tomli/_re.py |  |  | 0.445 |
| walker |  | 4881 | 0 | python decl at src/tomli/_re.py:59 |  |  | 0.445 |
| walker |  | 4881 | 0 | python decl at src/tomli/_re.py:109 |  |  | 0.445 |
| walker |  | 4881 | 0 | python decl at src/tomli/_re.py:116 |  |  | 0.445 |
| walker |  | 4907 | 26 | python decl at src/tomli/_re.py:98 |  |  | 0.448 |
| walker |  | 4941 | 34 | python decl body at src/tomli/_re.py:116 body 117 |  |  | 0.448 |
| walker |  | 5003 | 62 | python decl doc at src/tomli/_re.py:59 |  |  | 0.448 |
| walker |  | 5067 | 64 | python imports in src/tomli/_types.py |  |  | 0.461 |
| ns | 5094 |  | 539 | test_data.py (full, data-driven glob mechanism) | 4.7 | 2.10 | 0.436 |
| walker |  | 5164 | 97 | README.md section #11 |  |  | 0.436 |
| walker |  | 5234 | 70 | python decl body at src/tomli/_re.py:98 body 100 |  |  | 0.436 |
| walker |  | 5335 | 101 | README.md section #7 |  |  | 0.436 |
| walker |  | 5386 | 51 | tomllib.md section #7 |  |  | 0.436 |
| walker |  | 5545 | 159 | python decl at src/tomli/_re.py:46 |  |  | 0.436 |
| walker |  | 5661 | 116 | README.md section #12 |  |  | 0.436 |
| ns | 5772 |  | 678 | Flags class (namespace mutability tracking) | 5.1 | 2.5 | 0.415 |
| walker |  | 5818 | 157 | python imports in src/tomli/_parser.py |  |  | 0.415 |
| ns | 6492 |  | 720 | TOMLDecodeError (+ deprecation sentinel) full body | 5.2 | 1.11 | 0.400 |
| ns | 7226 |  | 734 | _re.py regex definitions (RE_NUMBER/RE_LOCALTIME/RE_DATETIME) | 5.3 |  | 0.387 |
| walker |  | 7630 | 1812 | manifest config in pyproject.toml |  |  | 0.425 |
| walker |  | 7733 | 103 | python imports in src/tomli/_re.py |  |  | 0.435 |
| walker |  | 7850 | 117 | README.md section #6 |  |  | 0.435 |
| walker |  | 7926 | 76 | tomllib.md section #4 |  |  | 0.435 |
| ns | 8144 |  | 918 | parse_value() type dispatch | 5.4 | 2.4 | 0.410 |
| walker |  | 8163 | 237 | python decl at src/tomli/_re.py:26 |  |  | 0.435 |
| walker |  | 8267 | 104 | python decl at src/tomli/_re.py:17 |  |  | 0.449 |
| walker |  | 8350 | 83 | tomllib.md section #5 |  |  | 0.449 |
| walker |  | 8427 | 77 | python imports in tests/__init__.py |  |  | 0.454 |
| walker |  | 8599 | 172 | README.md section #3 |  |  | 0.478 |
| walker |  | 8613 | 14 | python decl names surface in profiler/profiler_script.py |  |  | 0.478 |
| walker |  | 8628 | 15 | python decl body at src/tomli/_parser.py:595 body 596 |  |  | 0.478 |
| walker |  | 8636 | 8 | plaintext config scripts/requirements.txt |  |  | 0.478 |
| walker |  | 8820 | 184 | README.md section #27 |  |  | 0.478 |
| walker |  | 8836 | 16 | python decl names surface in scripts/use_setuptools.py |  |  | 0.478 |
| walker |  | 8836 | 0 | python decl at scripts/use_setuptools.py:12 |  |  | 0.478 |
| walker |  | 8845 | 9 | plaintext config profiler/requirements.txt |  |  | 0.478 |
| walker |  | 8973 | 128 | tomllib.md section #6 |  |  | 0.478 |
| walker |  | 9056 | 83 | python decl body at src/tomli/_re.py:109 body 110 |  |  | 0.478 |
| ns | 9105 |  | 961 | loads()/load() entry point + parse loop | 5.5 | 1.11 | 0.451 |
| walker |  | 9107 | 51 | headings outline in benchmark/README.md |  |  | 0.451 |
| walker |  | 9368 | 261 | README.md section #24 |  |  | 0.469 |
| walker |  | 9465 | 97 | tomllib.md section #1 |  |  | 0.475 |
| walker |  | 9492 | 27 | python decl names surface in benchmark/run.py |  |  | 0.475 |
| walker |  | 9492 | 0 | python decl at benchmark/run.py:37 |  |  | 0.475 |
| walker |  | 9516 | 24 | python decl body at src/tomli/_parser.py:497 body 498 |  |  | 0.475 |
| walker |  | 9820 | 304 | README.md section #26 |  |  | 0.475 |
| ns | 9987 |  | 882 | key_value_rule / parse_key_value_pair / parse_key (full bodies) | 5.6 | 2.2 | 0.455 |

Score(3000)=0.417 I=0.695 C=0.250 ns_rows≤3K=29/43 (reached=10 partial=0 missing=19)

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
| walker |  | 1023 | 58 | package metadata in pyproject.toml |  |  | 0.582 |
| walker |  | 1055 | 32 | python imports in setup.py |  |  | 0.582 |
| ns | 1101 |  | 138 | _parser.py roster: value parsing | 2.4 |  | 0.550 |
| walker |  | 1182 | 127 | tool.isort config in pyproject.toml |  |  | 0.550 |
| ns | 1243 |  | 142 | _parser.py roster: namespace state machinery | 2.5 |  | 0.519 |
| walker |  | 1332 | 150 | tool.coverage config in pyproject.toml |  |  | 0.519 |
| ns | 1414 |  | 171 | .pre-commit-config.yaml hook-id roster | 2.6 |  | 0.495 |
| ns | 1520 |  | 106 | tests/data/valid/ listing | 2.7 |  | 0.466 |
| ns | 1685 |  | 165 | tests/data/invalid/ listing | 2.8 |  | 0.432 |
| ns | 1710 |  | 25 | Sample invalid TOML fixture (dotted-keys/extend-defined-aot) | 2.9 |  | 0.428 |
| walker |  | 1717 | 385 | headings outline in README.md |  |  | 0.429 |
| walker |  | 1726 | 9 | README.md section #0 |  |  | 0.429 |
| walker |  | 1732 | 6 | README.md section #19 |  |  | 0.429 |
| walker |  | 1757 | 25 | README.md section #2 |  |  | 0.429 |
| walker |  | 1765 | 8 | README.md section #13 |  |  | 0.429 |
| walker |  | 1776 | 11 | README.md section #14 |  |  | 0.429 |
| walker |  | 1790 | 14 | README.md section #18 |  |  | 0.429 |
| walker |  | 1818 | 28 | README.md section #16 |  |  | 0.429 |
| ns | 1949 |  | 239 | Test method roster (test_data/test_error/test_misc) | 2.10 |  | 0.406 |
| ns | 2058 |  | 109 | _types.py (full) | 2.11 |  | 0.396 |
| walker |  | 2090 | 272 | tool.mypy config in pyproject.toml |  |  | 0.446 |
| walker |  | 2173 | 83 | README.md section #25 |  |  | 0.446 |
| walker |  | 2207 | 34 | python decl names surface in src/tomli/_types.py |  |  | 0.449 |
| ns | 2213 |  | 155 | Sample valid TOML/JSON fixture pair (array-subtables) | 3.1 |  | 0.429 |
| walker |  | 2238 | 31 | README.md section #23 |  |  | 0.429 |
| walker |  | 2271 | 33 | README.md section #21 |  |  | 0.429 |
| ns | 2321 |  | 108 | tests/__init__.py (tomllib aliasing) | 3.2 |  | 0.420 |
| walker |  | 2446 | 175 | tomllib.md section #0 |  |  | 0.422 |
| walker |  | 2486 | 40 | README.md section #9 |  |  | 0.422 |
| walker |  | 2525 | 39 | README.md section #22 |  |  | 0.422 |
| walker |  | 2534 | 9 | README headline in benchmark/README.md |  |  | 0.422 |
| walker |  | 2541 | 7 | listing of 'tests/data' |  |  | 0.422 |
| ns | 2562 |  | 241 | External toml-test valid/ category listing | 3.3 |  | 0.388 |
| ns | 2756 |  | 194 | pyproject.toml build-system + core [project] identity | 3.4 |  | 0.417 |
| walker |  | 3005 | 464 | python decl names surface in src/tomli/_parser.py |  |  | 0.473 |
| walker |  | 3054 | 49 | README.md section #17 |  |  | 0.473 |
| ns | 3081 |  | 325 | README intro | 3.5 |  | 0.457 |
| walker |  | 3110 | 56 | README.md section #5 |  |  | 0.457 |
| walker |  | 3166 | 56 | README.md section #8 |  |  | 0.457 |
| walker |  | 3224 | 58 | README.md section #4 |  |  | 0.457 |
| walker |  | 3254 | 30 | tomllib.md section #3 |  |  | 0.457 |
| ns | 3302 |  | 221 | README usage: parse a TOML string | 4.1 |  | 0.442 |
| walker |  | 3555 | 301 | README.md section #1 |  |  | 0.482 |
| walker |  | 3587 | 32 | tomllib.md section #2 |  |  | 0.482 |
| ns | 3633 |  | 331 | tomllib.md - intro + sync status | 4.2 |  | 0.490 |
| walker |  | 3658 | 71 | README.md section #10 |  |  | 0.490 |
| walker |  | 3668 | 10 | CHANGELOG.md section #0 |  |  | 0.490 |
| walker |  | 3739 | 71 | README.md section #20 |  |  | 0.490 |
| walker |  | 3812 | 73 | README.md section #15 |  |  | 0.490 |
| ns | 3930 |  | 297 | README: TOML->Python type mapping table | 4.3 |  | 0.478 |
| walker |  | 3973 | 161 | python decl names surface in src/tomli/_re.py |  |  | 0.484 |
| walker |  | 3973 | 0 | python decl at src/tomli/_re.py:59 |  |  | 0.484 |
| walker |  | 3973 | 0 | python decl at src/tomli/_re.py:109 |  |  | 0.484 |
| walker |  | 3973 | 0 | python decl at src/tomli/_re.py:116 |  |  | 0.484 |
| walker |  | 3999 | 26 | python decl at src/tomli/_re.py:98 |  |  | 0.488 |
| walker |  | 4033 | 34 | python decl body at src/tomli/_re.py:116 body 117 |  |  | 0.488 |
| ns | 4069 |  | 139 | CHANGELOG.md - latest two releases | 4.4 |  | 0.478 |
| walker |  | 4095 | 62 | python decl doc at src/tomli/_re.py:59 |  |  | 0.478 |
| ns | 4301 |  | 232 | burntsushi.py: convert() (partial) | 4.5 |  | 0.464 |
| ns | 4600 |  | 299 | test_misc.py: test_parse_float | 4.6 | 2.10 | 0.448 |
| ns | 5139 |  | 539 | test_data.py (full, data-driven glob mechanism) | 4.7 | 2.10 | 0.423 |
| walker |  | 5223 | 1128 | tool.tox config in pyproject.toml |  |  | 0.440 |
| walker |  | 5287 | 64 | python imports in src/tomli/_types.py |  |  | 0.452 |
| walker |  | 5384 | 97 | README.md section #11 |  |  | 0.452 |
| walker |  | 5454 | 70 | python decl body at src/tomli/_re.py:98 body 100 |  |  | 0.452 |
| walker |  | 5555 | 101 | README.md section #7 |  |  | 0.452 |
| walker |  | 5606 | 51 | tomllib.md section #7 |  |  | 0.452 |
| walker |  | 5765 | 159 | python decl at src/tomli/_re.py:46 |  |  | 0.453 |
| ns | 5817 |  | 678 | Flags class (namespace mutability tracking) | 5.1 | 2.5 | 0.425 |
| walker |  | 5881 | 116 | README.md section #12 |  |  | 0.425 |
| walker |  | 6040 | 159 | python imports in src/tomli/_parser.py |  |  | 0.425 |
| walker |  | 6143 | 103 | python imports in src/tomli/_re.py |  |  | 0.426 |
| walker |  | 6260 | 117 | README.md section #6 |  |  | 0.426 |
| walker |  | 6336 | 76 | tomllib.md section #4 |  |  | 0.426 |
| ns | 6537 |  | 720 | TOMLDecodeError (+ deprecation sentinel) full body | 5.2 | 1.11 | 0.399 |
| walker |  | 6573 | 237 | python decl at src/tomli/_re.py:26 |  |  | 0.401 |
| walker |  | 6677 | 104 | python decl at src/tomli/_re.py:17 |  |  | 0.402 |
| walker |  | 6760 | 83 | tomllib.md section #5 |  |  | 0.402 |
| walker |  | 6837 | 77 | python imports in tests/__init__.py |  |  | 0.408 |
| walker |  | 7009 | 172 | README.md section #3 |  |  | 0.439 |
| walker |  | 7023 | 14 | python decl names surface in profiler/profiler_script.py |  |  | 0.439 |
| walker |  | 7031 | 8 | plaintext config scripts/requirements.txt |  |  | 0.439 |
| walker |  | 7215 | 184 | README.md section #27 |  |  | 0.439 |
| walker |  | 7231 | 16 | python decl names surface in scripts/use_setuptools.py |  |  | 0.439 |
| walker |  | 7231 | 0 | python decl at scripts/use_setuptools.py:12 |  |  | 0.439 |
| walker |  | 7240 | 9 | plaintext config profiler/requirements.txt |  |  | 0.439 |
| ns | 7271 |  | 734 | _re.py regex definitions (RE_NUMBER/RE_LOCALTIME/RE_DATETIME) | 5.3 |  | 0.471 |
| walker |  | 7368 | 128 | tomllib.md section #6 |  |  | 0.471 |
| walker |  | 7451 | 83 | python decl body at src/tomli/_re.py:109 body 110 |  |  | 0.471 |
| walker |  | 7502 | 51 | headings outline in benchmark/README.md |  |  | 0.471 |
| walker |  | 7763 | 261 | README.md section #24 |  |  | 0.492 |
| walker |  | 7860 | 97 | tomllib.md section #1 |  |  | 0.499 |
| walker |  | 7887 | 27 | python decl names surface in benchmark/run.py |  |  | 0.499 |
| walker |  | 7887 | 0 | python decl at benchmark/run.py:37 |  |  | 0.499 |
| ns | 8189 |  | 918 | parse_value() type dispatch | 5.4 | 2.4 | 0.470 |
| walker |  | 8191 | 304 | README.md section #26 |  |  | 0.470 |
| ns | 9150 |  | 961 | loads()/load() entry point + parse loop | 5.5 | 1.11 | 0.443 |
| walker |  | 9248 | 1057 | YAML config at .pre-commit-config.yaml |  |  | 0.458 |
| walker |  | 9280 | 32 | python decl at profiler/profiler_script.py:12 |  |  | 0.458 |
| walker |  | 9291 | 11 | python imports in fuzzer/fuzz.py |  |  | 0.458 |
| walker |  | 9351 | 60 | python decl at benchmark/run.py:15 |  |  | 0.458 |
| walker |  | 9456 | 105 | listing of 'tests/data/valid' |  |  | 0.478 |
| walker |  | 9461 | 5 | listing of 'tests/data/valid/_external' |  |  | 0.478 |
| walker |  | 9467 | 6 | listing of 'tests/data/valid/_external/toml-test' |  |  | 0.478 |
| walker |  | 9488 | 21 | listing of 'tests/data/valid/dates-and-times' |  |  | 0.478 |
| walker |  | 9513 | 25 | listing of 'tests/data/valid/array' |  |  | 0.478 |
| walker |  | 9538 | 25 | listing of 'tests/data/valid/inline-table' |  |  | 0.478 |
| walker |  | 9565 | 27 | listing of 'tests/data/valid/multiline-basic-str' |  |  | 0.478 |
| walker |  | 9570 | 5 | json config tests/data/valid/no-newlines.json |  |  | 0.478 |
| walker |  | 9630 | 60 | python decl names surface in fuzzer/fuzz.py |  |  | 0.478 |
| walker |  | 9630 | 0 | python decl at fuzzer/fuzz.py:53 |  |  | 0.478 |
| walker |  | 9630 | 0 | python decl at fuzzer/fuzz.py:59 |  |  | 0.478 |
| walker |  | 9645 | 15 | python decl at fuzzer/fuzz.py:20 |  |  | 0.478 |
| walker |  | 9669 | 24 | python decl at fuzzer/fuzz.py:71 |  |  | 0.478 |
| walker |  | 9687 | 18 | python decl doc at fuzzer/fuzz.py:59 |  |  | 0.478 |
| walker |  | 9713 | 26 | python test names surface in tests/test_data.py |  |  | 0.479 |
| walker |  | 9757 | 44 | plaintext config fuzzer/requirements.txt |  |  | 0.479 |
| walker |  | 9922 | 165 | listing of 'tests/data/invalid' |  |  | 0.505 |
| walker |  | 9927 | 5 | listing of 'tests/data/invalid/_external' |  |  | 0.505 |
| walker |  | 9932 | 5 | listing of 'tests/data/invalid/dates-and-times' |  |  | 0.505 |
| walker |  | 9937 | 5 | listing of 'tests/data/invalid/literal-str' |  |  | 0.505 |
| walker |  | 9943 | 6 | listing of 'tests/data/invalid/_external/toml-test' |  |  | 0.505 |
| walker |  | 9957 | 14 | listing of 'tests/data/invalid/multiline-literal-str' |  |  | 0.505 |
| walker |  | 9974 | 17 | listing of 'tests/data/invalid/array-of-tables' |  |  | 0.505 |
| walker |  | 9991 | 17 | listing of 'tests/data/invalid/boolean' |  |  | 0.505 |
| ns | 10032 |  | 882 | key_value_rule / parse_key_value_pair / parse_key (full bodies) | 5.6 | 2.2 | 0.483 |

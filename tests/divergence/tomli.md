scores: Sim=0.428 Reached=17/40 Early=0 Late=13 Partial=4 Missing=19 Used=9964/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 6 | 4 | 2 | 0 | 0.85 |
| 2 | 9 | 7 | 1 | 1 | 0.82 |
| 3 | 12 | 3 | 0 | 9 | 0.25 |
| 4 | 8 | 1 | 0 | 7 | 0.15 |
| 5 | 5 | 2 | 1 | 2 | 0.55 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 87 | — | — | 0.67 | partial | README title + tagline | README headline in README.md (t=149, 2 atoms) |
| 1.3 | 166 | — | — | 0.75 | partial | Public API: __init__ __all__ + version | python imports in src/tomli/__init__.py (t=567, 3 atoms) |
| 1.4 | 192 | 305 | +113 | 1.00 | late | src/tomli/ module listing |  |
| 1.5 | 279 | 5134 | +4855 | 0.80 | late | loads / load signatures + docstrings | python decl body at src/tomli/_parser.py:137 (t=6733, 8 atoms) |
| 1.6 | 403 | 5711 | +5308 | 0.90 | late | TOMLDecodeError class + docstring | python decl doc at src/tomli/_parser.py:76 (t=5711, 8 atoms) |
| 2.1 | 433 | 9307 | +8874 | 1.00 | late | tests/ directory listing |  |
| 2.2 | 491 | — | — | 0.67 | partial | _types.py — full | python decl names surface in src/tomli/_types.py (t=337, 3 atoms) |
| 2.3 | 697 | 6159 | +5462 | 0.88 | late | _parser.py: state-class headers + Flags constants | python method sigs in src/tomli/_parser.py (t=6055, 22 atoms) |
| 2.4 | 844 | 1986 | +1142 | 1.00 | late | _re.py: regex constants + match-helper locations | python decl at src/tomli/_re.py:26 (t=4225, 19 atoms) |
| 2.5 | 1160 | 2642 | +1482 | 0.88 | late | README intro paragraph | README.md section #1 (t=2642, 14 atoms) |
| 2.7 | 1904 | 5096 | +3192 | 1.00 | late | _parser.py: parse_* and skip_* function locations | python decl names surface in src/tomli/_parser.py (t=5096, 43 atoms) |
| 2.8 | 2100 | — | — | 0.10 | missing | README usage: parse a TOML string | headings outline in README.md (t=946, 2 atoms) |
| 2.9 | 2392 | 8531 | +6139 | 0.81 | late | README usage: parse a file + handle errors | README.md section #5 (t=8180, 10 atoms) |
| 3.1 | 2491 | 6733 | +4242 | 1.00 | late | load body | python decl body at src/tomli/_parser.py:137 (t=6733, 8 atoms) |
| 3.2 | 2746 | — | — | 0.00 | missing | tests/data/{valid,invalid}/ top listing |  |
| 3.3 | 2972 | — | — | 0.00 | missing | tests/* test method names |  |
| 3.4 | 3320 | — | — | 0.00 | missing | loads body — prelude + skip / dispatch comments |  |
| 3.5 | 3755 | — | — | 0.00 | missing | loads body — rule dispatch + statement terminator |  |
| 3.6 | 3977 | — | — | 0.00 | missing | TOMLDecodeError.__init__ — pos→line/col body |  |
| 3.7 | 4502 | — | — | 0.05 | missing | parse_value — dispatch head (strings/bools/array/inline-table) | python decl at src/tomli/_parser.py:684 (t=5256, 2 atoms) |
| 3.8 | 4897 | — | — | 0.00 | missing | parse_value — datetime/number/special-float tail |  |
| 3.9 | 5138 | 6897 | +1759 | 0.84 | late | Flags — __init__, add_pending, finalize_pending, unset_all | python method sigs in src/tomli/_parser.py (t=6055, 8 atoms) |
| 3.10 | 5527 | — | — | 0.08 | missing | Flags — set + is_ (the actual lookup) | python method sigs in src/tomli/_parser.py (t=6055, 4 atoms) |
| 3.11 | 5894 | 9731 | +3837 | 0.94 | late | NestedDict body — table-tree builder | python method body at src/tomli/_parser.py:283 (t=9731, 10 atoms) |
| 3.12 | 6145 | — | — | 0.12 | missing | README usage: Decimal floats | headings outline in README.md (t=946, 2 atoms) |
| 4.1 | 6395 | 9964 | +3569 | 0.83 | late | create_dict_rule body — [table] header | python decl body at src/tomli/_parser.py:370 (t=9964, 15 atoms) |
| 4.2 | 6679 | — | — | 0.05 | missing | create_list_rule body — [[arr]] header | python decl names surface in src/tomli/_parser.py (t=5096, 2 atoms) |
| 4.3 | 7159 | — | — | 0.06 | missing | key_value_rule body | python decl at src/tomli/_parser.py:413 (t=5197, 2 atoms) |
| 4.4 | 7480 | — | — | 0.10 | missing | parse_inline_table — head + first key/value insert | python decl at src/tomli/_parser.py:528 (t=5379, 2 atoms) |
| 4.5 | 7650 | — | — | 0.00 | missing | parse_inline_table — comma/close loop tail |  |
| 4.6 | 7937 | — | — | 0.09 | missing | parse_array body | python decl at src/tomli/_parser.py:502 (t=5291, 2 atoms) |
| 4.7 | 8290 | — | — | 0.00 | missing | parse_basic_str body | python decl names surface in src/tomli/_parser.py (t=5096, 1 atoms) |
| 4.8 | 8626 | — | — | 0.07 | missing | README usage: tomllib compat shim | headings outline in README.md (t=946, 2 atoms) |
| 5.1 | 8921 | — | — | 0.12 | missing | README FAQ: type mapping table | headings outline in README.md (t=946, 2 atoms) |
| 5.2 | 9402 | 7701 | -1701 | 0.86 | aligned | skip_chars / skip_until / skip_comment / skip_comments_and_array_ws | python decl body at src/tomli/_parser.py:327 (t=7701, 11 atoms) |
| 5.3 | 9565 | — | — | 0.77 | partial | CHANGELOG: 2.4 + 2.1 entries | CHANGELOG.md section #5 (t=3666, 6 atoms) |
| 5.5 | 9973 | — | — | 0.00 | missing | pyproject.toml [project] block |  |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 11 | 977 | CHANGELOG.md section #<n> |
| 3 | 259 | README.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 320 | 0.37 | 871 | 5096 | python decl names surface in src/tomli/_parser.py |
| 315 | 1.00 | 315 | 8929 | python decl body at fuzzer/fuzz.py:20 |
| 239 | 1.00 | 239 | 4225 | python decl at src/tomli/_re.py:26 |
| 189 | 0.50 | 379 | 946 | headings outline in README.md |
| 184 | 1.00 | 184 | 8420 | python decl body at src/tomli/_parser.py:463 |
| 165 | 1.00 | 165 | 7996 | python decl body at src/tomli/_parser.py:481 |
| 164 | 1.00 | 164 | 9477 | CHANGELOG.md section #7 |
| 162 | 1.00 | 162 | 1800 | tomllib.md section #0 |
| 161 | 1.00 | 161 | 3566 | python decl at src/tomli/_re.py:46 |
| 158 | 1.00 | 158 | 6544 | python decl at src/tomli/_parser.py:57 |
| 150 | 1.00 | 150 | 7423 | python decl body at src/tomli/_parser.py:447 |
| 150 | 1.00 | 150 | 3259 | python imports in src/tomli/_parser.py |
| 148 | 1.00 | 148 | 7273 | python decl body at src/tomli/_parser.py:599 |
| 130 | 1.00 | 130 | 9607 | CHANGELOG.md section #11 |
| 127 | 1.00 | 127 | 9277 | CHANGELOG.md section #10 |
| 125 | 1.00 | 125 | 7548 | CHANGELOG.md section #6 |
| 122 | 1.00 | 122 | 7019 | python decl body at src/tomli/_parser.py:764 |
| 120 | 1.00 | 120 | 3109 | python decl body at scripts/use_setuptools.py:12 |
| 112 | 1.00 | 112 | 9150 | README.md section #9 |
| 108 | 1.00 | 108 | 2881 | python decl body at fuzzer/fuzz.py:59 |
| 106 | 1.00 | 106 | 7125 | python decl at src/tomli/_re.py:17 |
| 90 | 1.00 | 90 | 6634 | python decl body at src/tomli/_parser.py:612 |
| 88 | 1.00 | 88 | 5799 | python decl doc at src/tomli/_parser.py:764 |
| 83 | 1.00 | 83 | 3342 | python decl body at src/tomli/_re.py:109 |
| 83 | 1.00 | 83 | 2354 | python imports in src/tomli/_re.py |
| 81 | 1.00 | 81 | 6240 | python method at src/tomli/_parser.py:87 |
| 80 | 1.00 | 80 | 8076 | CHANGELOG.md section #15 |
| 75 | 1.00 | 75 | 1405 | README.md section #12 |
| 75 | 1.00 | 75 | 1638 | python imports in scripts/use_setuptools.py |
| 72 | 1.00 | 72 | 3937 | README.md section #10 |
| 72 | 1.00 | 72 | 2989 | python decl body at src/tomli/_re.py:98 |
| 70 | 1.00 | 70 | 1563 | python imports in profiler/profiler_script.py |
| 67 | 1.00 | 67 | 3819 | CHANGELOG.md section #12 |
| 60 | 1.00 | 60 | 397 | python decl names surface in fuzzer/fuzz.py |
| 59 | 1.00 | 59 | 2271 | CHANGELOG.md section #3 |
| 58 | 1.00 | 58 | 8589 | CHANGELOG.md section #30 |
| 57 | 1.00 | 57 | 2156 | python decl body at fuzzer/fuzz.py:53 |
| 56 | 1.00 | 56 | 2212 | CHANGELOG.md section #2 |
| 56 | 1.00 | 56 | 8236 | CHANGELOG.md section #28 |
| 55 | 1.00 | 55 | 7831 | CHANGELOG.md section #25 |
| 55 | 1.00 | 55 | 245 | headings outline in tomllib.md |
| 55 | 1.00 | 55 | 2099 | python decl doc at src/tomli/_re.py:59 |

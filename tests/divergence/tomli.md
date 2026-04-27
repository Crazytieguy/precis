scores: Sim=0.403 Reached=10/40 Early=3 Late=4 Partial=4 Missing=26 Used=9899/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 6 | 2 | 1 | 3 | 0.44 |
| 2 | 9 | 5 | 0 | 4 | 0.50 |
| 3 | 12 | 1 | 1 | 10 | 0.15 |
| 4 | 8 | 0 | 1 | 7 | 0.10 |
| 5 | 5 | 2 | 1 | 2 | 0.55 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 87 | — | — | 0.67 | partial | README title + tagline | README headline in README.md (t=149, 2 atoms) |
| 1.3 | 166 | — | — | 0.00 | missing | Public API: __init__ __all__ + version |  |
| 1.5 | 279 | — | — | 0.00 | missing | loads / load signatures + docstrings |  |
| 1.6 | 403 | — | — | 0.00 | missing | TOMLDecodeError class + docstring |  |
| 2.1 | 433 | 3316 | +2883 | 1.00 | late | tests/ directory listing |  |
| 2.2 | 491 | — | — | 0.00 | missing | _types.py — full |  |
| 2.3 | 697 | — | — | 0.00 | missing | _parser.py: state-class headers + Flags constants |  |
| 2.4 | 844 | — | — | 0.00 | missing | _re.py: regex constants + match-helper locations |  |
| 2.5 | 1160 | 1733 | +573 | 0.88 | late | README intro paragraph | README.md section #1 (t=1733, 14 atoms) |
| 2.6 | 1486 | 1010 | -476 | 1.00 | early | README table of contents | README.md section #0 (t=1010, 16 atoms) |
| 2.7 | 1904 | — | — | 0.00 | missing | _parser.py: parse_* and skip_* function locations |  |
| 2.8 | 2100 | 4038 | +1938 | 0.85 | late | README usage: parse a TOML string | README.md section #3 (t=4038, 16 atoms) |
| 2.9 | 2392 | 2989 | +597 | 0.81 | aligned | README usage: parse a file + handle errors | README.md section #5 (t=2822, 10 atoms) |
| 3.1 | 2491 | — | — | 0.00 | missing | load body |  |
| 3.2 | 2746 | 6937 | +4191 | 1.00 | late | tests/data/{valid,invalid}/ top listing |  |
| 3.3 | 2972 | — | — | 0.00 | missing | tests/* test method names |  |
| 3.4 | 3320 | — | — | 0.00 | missing | loads body — prelude + skip / dispatch comments |  |
| 3.5 | 3755 | — | — | 0.00 | missing | loads body — rule dispatch + statement terminator |  |
| 3.6 | 3977 | — | — | 0.00 | missing | TOMLDecodeError.__init__ — pos→line/col body |  |
| 3.7 | 4502 | — | — | 0.00 | missing | parse_value — dispatch head (strings/bools/array/inline-table) |  |
| 3.8 | 4897 | — | — | 0.00 | missing | parse_value — datetime/number/special-float tail |  |
| 3.9 | 5138 | — | — | 0.00 | missing | Flags — __init__, add_pending, finalize_pending, unset_all |  |
| 3.10 | 5527 | — | — | 0.00 | missing | Flags — set + is_ (the actual lookup) |  |
| 3.11 | 5894 | — | — | 0.00 | missing | NestedDict body — table-tree builder |  |
| 3.12 | 6145 | — | — | 0.76 | partial | README usage: Decimal floats | README.md section #6 (t=4485, 12 atoms) |
| 4.1 | 6395 | — | — | 0.00 | missing | create_dict_rule body — [table] header |  |
| 4.2 | 6679 | — | — | 0.00 | missing | create_list_rule body — [[arr]] header |  |
| 4.3 | 7159 | — | — | 0.00 | missing | key_value_rule body |  |
| 4.4 | 7480 | — | — | 0.00 | missing | parse_inline_table — head + first key/value insert |  |
| 4.5 | 7650 | — | — | 0.00 | missing | parse_inline_table — comma/close loop tail |  |
| 4.6 | 7937 | — | — | 0.00 | missing | parse_array body |  |
| 4.7 | 8290 | — | — | 0.00 | missing | parse_basic_str body |  |
| 4.8 | 8626 | — | — | 0.79 | partial | README usage: tomllib compat shim | README.md section #7 (t=5211, 21 atoms) |
| 5.1 | 8921 | 5469 | -3452 | 1.00 | early | README FAQ: type mapping table | README.md section #11 (t=5469, 16 atoms) |
| 5.2 | 9402 | — | — | 0.00 | missing | skip_chars / skip_until / skip_comment / skip_comments_and_array_ws |  |
| 5.3 | 9565 | — | — | 0.77 | partial | CHANGELOG: 2.4 + 2.1 entries | CHANGELOG.md section #5 (t=2063, 6 atoms) |
| 5.4 | 9623 | 2576 | -7047 | 1.00 | early | benchmark/, fuzzer/, profiler/, scripts/, .github/ listings |  |
| 5.5 | 9973 | — | — | 0.00 | missing | pyproject.toml [project] block |  |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 15 | 1448 | CHANGELOG.md section #<n> |
| 6 | 926 | README.md section #<n> |
| 3 | 613 | tomllib.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 357 | 1.00 | 357 | 6261 | tomllib.md section #2 |
| 301 | 1.00 | 301 | 5770 | README.md section #13 |
| 293 | 1.00 | 293 | 6783 | plaintext config LICENSE |
| 230 | 1.00 | 230 | 8495 | listing of 'tests/data/valid/_external/toml-test/valid' |
| 225 | 1.00 | 225 | 9797 | listing of 'tests/data/invalid/_external/toml-test/invalid/control' |
| 216 | 1.00 | 216 | 9572 | listing of 'tests/data/invalid/_external/toml-test/invalid/array' |
| 203 | 1.00 | 203 | 4291 | CHANGELOG.md section #9 |
| 189 | 0.50 | 379 | 626 | headings outline in README.md |
| 185 | 1.00 | 185 | 4764 | README.md section #8 |
| 181 | 1.00 | 181 | 4945 | README.md section #14 |
| 172 | 1.00 | 172 | 9129 | listing of 'tests/data/invalid/_external/toml-test/invalid/inline-table' |
| 164 | 1.00 | 164 | 3486 | CHANGELOG.md section #7 |
| 162 | 1.00 | 162 | 1295 | tomllib.md section #0 |
| 134 | 1.00 | 134 | 8265 | listing of 'tests/data/invalid/_external/toml-test/invalid/encoding' |
| 130 | 1.00 | 130 | 3616 | CHANGELOG.md section #11 |
| 127 | 1.00 | 127 | 3286 | CHANGELOG.md section #10 |
| 125 | 1.00 | 125 | 2508 | CHANGELOG.md section #6 |
| 116 | 1.00 | 116 | 8069 | listing of 'tests/data/invalid/_external/toml-test/invalid/bool' |
| 112 | 1.00 | 112 | 3159 | README.md section #9 |
| 106 | 1.00 | 106 | 8861 | listing of 'tests/data/valid/_external/toml-test/valid/datetime' |
| 102 | 1.00 | 102 | 9899 | json config tests/data/valid/array/array-subtables.json |
| 99 | 1.00 | 99 | 3884 | CHANGELOG.md section #19 |
| 96 | 1.00 | 96 | 7896 | listing of 'tests/data/invalid/_external/toml-test/invalid/local-datetime' |
| 94 | 1.00 | 94 | 4579 | tomllib.md section #1 |
| 92 | 1.00 | 92 | 3708 | CHANGELOG.md section #20 |
| 89 | 1.00 | 89 | 7259 | listing of 'tests/data/invalid/inline-table' |
| 87 | 1.00 | 87 | 5904 | benchmark/README.md section #1 |
| 87 | 1.00 | 87 | 8745 | listing of 'tests/data/valid/_external/toml-test/valid/comment' |
| 86 | 1.00 | 86 | 9253 | json config tests/data/valid/multiline-basic-str/replacements.json |
| 86 | 1.00 | 86 | 7800 | listing of 'tests/data/invalid/_external/toml-test/invalid/local-date' |
| 80 | 1.00 | 80 | 2718 | CHANGELOG.md section #15 |
| 77 | 1.00 | 77 | 3785 | CHANGELOG.md section #27 |
| 75 | 1.00 | 75 | 1085 | README.md section #12 |
| 72 | 1.00 | 72 | 2334 | README.md section #10 |
| 71 | 1.00 | 71 | 8658 | listing of 'tests/data/valid/_external/toml-test/valid/float' |
| 68 | 1.00 | 68 | 7538 | listing of 'tests/data/invalid/_external/toml-test/invalid/spec-1.1.0' |
| 67 | 1.00 | 67 | 2216 | CHANGELOG.md section #12 |
| 62 | 1.00 | 62 | 8131 | json config tests/data/valid/array/open-parent-table.json |
| 61 | 1.00 | 61 | 7714 | json config tests/data/valid/hex-char.json |
| 59 | 1.00 | 59 | 1445 | CHANGELOG.md section #3 |
| 59 | 1.00 | 59 | 7653 | json config tests/data/valid/five-quotes.json |
| 58 | 1.00 | 58 | 3047 | CHANGELOG.md section #30 |
| 58 | 1.00 | 58 | 8587 | listing of 'tests/data/valid/_external/toml-test/valid/integer' |
| 57 | 1.00 | 57 | 7953 | json config tests/data/valid/dates-and-times/localtime.json |
| 56 | 1.00 | 56 | 1386 | CHANGELOG.md section #2 |
| 56 | 1.00 | 56 | 2878 | CHANGELOG.md section #28 |
| 56 | 1.00 | 56 | 7594 | json config tests/data/valid/trailing-comma.json |
| 56 | 1.00 | 56 | 7375 | listing of 'tests/data/invalid/_external/toml-test/invalid' |
| 55 | 1.00 | 55 | 2638 | CHANGELOG.md section #25 |
| 55 | 1.00 | 55 | 221 | headings outline in tomllib.md |
| 52 | 1.00 | 52 | 9356 | json config tests/data/valid/_external/toml-test/valid/newline-crlf.json |
| 51 | 1.00 | 51 | 9304 | json config tests/data/valid/_external/toml-test/valid/newline-lf.json |
| 51 | 1.00 | 51 | 7426 | listing of 'tests/data/invalid/_external/toml-test/invalid/local-time' |
| 50 | 1.00 | 50 | 4088 | headings outline in benchmark/README.md |

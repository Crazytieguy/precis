scores: Sim=0.434 Reached=14/40 Early=0 Late=9 Partial=6 Missing=20 Used=9933/10000

## Verdict

Verdict: budget-pressure bound
Likely primary lever: free final budget / demote late low-value spend
Evidence: 12 ranking-recoverable (w×gap=1.05), 10 wrong-slice/granularity (w×gap=1.04), 2 no-discovered (w×gap=0.23)
Secondary intervention: investigate 2 no-discovered rows
Loss reasons: 0 predecessor-gated, 12 too-expensive, 0 discovered-unscheduled
Top rows: 2.8, 3.4, 3.5, 3.6, 3.8, ...
Note: likely lever is heuristic; verify `Sim` moves, not just bucket counts.

## Top opportunities

_`w(t)×gap` is a non-additive priority score: Σ exp(-exp_t/τ) × (1 - credit) per row, τ=2000. Same time weighting and credit gap as Sim, but rows can overlap between opportunities — sums across rows are an upper bound on Sim impact, not an additive estimate._

| intervention | rows | w(t)×gap | bands ≤3k/≤6k/total | evidence | top row ids |
|:-------------|-----:|---------:|:----------------------|:---------|:------------|
| free final budget / demote late waste | 12 | 1.05 | 1/5/12 | high-overlap candidates exceed final remaining budget, exact total=252/282 | 2.8, 3.4, 3.5, 3.6, 3.8, ... |
| split wrong-slice walker batches | 10 | 1.04 | 3/6/10 | nearby candidates have low exact atom overlap | 1.2, 2.2, 1.3, 3.7, 3.10, ... |
| add walker candidates for no-discovered rows | 2 | 0.23 | 1/1/2 | NS rows have no discovered line candidate | 3.3, 5.5 |

Tiers: 1=4/6 reached, 2 partial, 0 missing, avg=0.85; 2=6/9 reached, 1 partial, 2 missing, avg=0.70; 3=2/12 reached, 1 partial, 9 missing, avg=0.25; 4=0/8 reached, 1 partial, 7 missing, avg=0.14; 5=2/5 reached, 1 partial, 2 missing, avg=0.55

## Diagnosis rollup

| diagnosis | rows | missing | partial | timing | likely lever |
|:----------|-----:|--------:|--------:|-------:|:-------------|
| ranking-recoverable | 12 | 12 | 0 | 0 | value/ranking |
| wrong-slice / granularity | 10 | 4 | 6 | 0 | walker granularity / wrong slice |
| no discovered candidate | 2 | 2 | 0 | 0 | walker coverage or predecessor-gated emit |
| fs/listing | 2 | 2 | 0 | 0 | filesystem/listing value |
| timing-only | 11 | 0 | 0 | 11 | usually no code change |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | w(t)×gap | likely lever |
|:------------|-----:|---------:|:-------------|
| too expensive at final margin | 12 | 1.05 | free final budget |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=27, unscheduled bbox=5, fs-only=3, no discovered candidate=2

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | aligned | low | 2 |
| scheduled bbox | late | none | 1 |
| scheduled bbox | late | low | 4 |
| scheduled bbox | late | high | 2 |
| scheduled bbox | late | full | 1 |
| scheduled bbox | missing | low | 11 |
| scheduled bbox | partial | low | 6 |
| unscheduled bbox | missing | high | 4 |
| unscheduled bbox | missing | full | 1 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 2.8 | 2100 | — | — | 0.10 | missing | README usage: parse a TOML string | [scheduled bbox exact=2/20] headings outline in README.md (t=946, 2 atoms); better unscheduled exact=16/20: README.md section #3 (16 atoms, too expensive at final margin) |
| 3.4 | 3320 | — | — | 0.00 | missing | loads body — prelude + skip / dispatch comments | [unscheduled bbox exact=25/27] python decl body at src/tomli/_parser.py:149 (25 atoms, too expensive at final margin) |
| 3.5 | 3755 | — | — | 0.00 | missing | loads body — rule dispatch + statement terminator | [unscheduled bbox exact=36/39] python decl body at src/tomli/_parser.py:149 (36 atoms, too expensive at final margin) |
| 3.6 | 3977 | — | — | 0.00 | missing | TOMLDecodeError.__init__ — pos→line/col body | [unscheduled bbox exact=16/18] python method body at src/tomli/_parser.py:87 (16 atoms, too expensive at final margin) |
| 3.8 | 4897 | — | — | 0.00 | missing | parse_value — datetime/number/special-float tail | [unscheduled bbox exact=25/29] python decl body at src/tomli/_parser.py:684 (25 atoms, too expensive at final margin) |
| 4.1 | 6395 | — | — | 0.00 | missing | create_dict_rule body — [table] header | [scheduled bbox exact=1/18] python decl names surface #2 in src/tomli/_parser.py (t=7759, 1 atoms); better unscheduled exact=15/18: python decl body at src/tomli/_parser.py:370 (15 atoms, too expensive at final margin) |
| 4.2 | 6679 | — | — | 0.05 | missing | create_list_rule body — [[arr]] header | [scheduled bbox exact=2/21] python decl names surface #2 in src/tomli/_parser.py (t=7759, 2 atoms); better unscheduled exact=18/21: python decl body at src/tomli/_parser.py:390 (18 atoms, too expensive at final margin) |
| 4.3 | 7159 | — | — | 0.06 | missing | key_value_rule body | [scheduled bbox exact=2/31] python decl at src/tomli/_parser.py:413 (t=7822, 2 atoms); better unscheduled exact=26/31: python decl body at src/tomli/_parser.py:413 (26 atoms, too expensive at final margin) |
| 4.4 | 7480 | — | — | 0.10 | missing | parse_inline_table — head + first key/value insert | [scheduled bbox exact=2/21] python decl at src/tomli/_parser.py:528 (t=7945, 2 atoms); better unscheduled exact=18/21: python decl body at src/tomli/_parser.py:528 (18 atoms, too expensive at final margin) |
| 4.5 | 7650 | — | — | 0.00 | missing | parse_inline_table — comma/close loop tail | [unscheduled bbox exact=12/12] python decl body at src/tomli/_parser.py:528 (12 atoms, too expensive at final margin) |
| 4.7 | 8290 | — | — | 0.00 | missing | parse_basic_str body | [scheduled bbox exact=1/29] python decl names surface #3 in src/tomli/_parser.py (t=8437, 1 atoms); better unscheduled exact=29/29: python decl body at src/tomli/_parser.py:652 (29 atoms, too expensive at final margin) |
| 5.1 | 8921 | — | — | 0.12 | missing | README FAQ: type mapping table | [scheduled bbox exact=2/17] headings outline in README.md (t=946, 2 atoms); better unscheduled exact=16/17: README.md section #24 (16 atoms, too expensive at final margin) |

### wrong-slice / granularity

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 87 | — | — | 0.67 | partial | README title + tagline | [scheduled bbox exact=2/3] README headline in README.md (t=149, 2 atoms) |
| 1.3 | 166 | — | — | 0.75 | partial | Public API: __init__ __all__ + version | [scheduled bbox exact=3/4] python imports in src/tomli/__init__.py (t=379, 3 atoms) |
| 2.2 | 491 | — | — | 0.67 | partial | _types.py — full | [scheduled bbox exact=3/6] python decl names surface in src/tomli/_types.py (t=411, 3 atoms) |
| 3.7 | 4502 | — | — | 0.05 | missing | parse_value — dispatch head (strings/bools/array/inline-table) | [scheduled bbox exact=2/44] python decl at src/tomli/_parser.py:684 (t=8471, 2 atoms); better unscheduled exact=35/44: python decl body at src/tomli/_parser.py:684 (35 atoms, too expensive at final margin) |
| 3.10 | 5527 | — | — | 0.08 | missing | Flags — set + is_ (the actual lookup) | [scheduled bbox exact=4/27] python method sigs #1 in src/tomli/_parser.py (t=5632, 4 atoms); better unscheduled exact=15/27: python method body at src/tomli/_parser.py:260 (15 atoms, too expensive at final margin) |
| 3.11 | 5894 | — | — | 0.32 | missing | NestedDict body — table-tree builder | [scheduled bbox exact=6/31] python method at src/tomli/_parser.py:283 (t=5702, 6 atoms); better unscheduled exact=10/31: python method body at src/tomli/_parser.py:283 (10 atoms, too expensive at final margin) |
| 3.12 | 6145 | — | — | 0.76 | partial | README usage: Decimal floats | [scheduled bbox exact=7/17] README.md section #7 (t=4580, 7 atoms) |
| 4.6 | 7937 | — | — | 0.09 | missing | parse_array body | [scheduled bbox exact=2/23] python decl at src/tomli/_parser.py:502 (t=7857, 2 atoms); better unscheduled exact=18/23: python decl body at src/tomli/_parser.py:502 (18 atoms, too expensive at final margin) |
| 4.8 | 8626 | — | — | 0.79 | partial | README usage: tomllib compat shim | [scheduled bbox exact=9/28] README.md section #12 (t=4844, 9 atoms) |
| 5.3 | 9565 | — | — | 0.77 | partial | CHANGELOG: 2.4 + 2.1 entries | [scheduled bbox exact=6/13] CHANGELOG.md section #5 (t=6401, 6 atoms) |

### no discovered candidate

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 3.3 | 2972 | — | — | 0.00 | missing | tests/* test method names | no discovered line candidate |
| 5.5 | 9973 | — | — | 0.00 | missing | pyproject.toml [project] block | no discovered line candidate |

### fs/listing

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 2.1 | 433 | — | — | 0.00 | missing | tests/ directory listing | fs-only |
| 3.2 | 2746 | — | — | 0.00 | missing | tests/data/{valid,invalid}/ top listing | fs-only |

### timing-only

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.4 | 192 | 305 | +113 | 1.00 | late | src/tomli/ module listing | fs-only |
| 1.5 | 279 | 5095 | +4816 | 0.80 | late | loads / load signatures + docstrings | [scheduled bbox exact=0/5] python decl body at src/tomli/_parser.py:137 (t=6201, 8 atoms) |
| 1.6 | 403 | 5376 | +4973 | 0.90 | late | TOMLDecodeError class + docstring | [scheduled bbox exact=8/10] python decl doc at src/tomli/_parser.py:76 (t=5376, 8 atoms) |
| 2.3 | 697 | 5736 | +5039 | 0.88 | late | _parser.py: state-class headers + Flags constants | [scheduled bbox exact=2/17] python method sigs #1 in src/tomli/_parser.py (t=5632, 22 atoms) |
| 2.4 | 844 | 2115 | +1271 | 1.00 | late | _re.py: regex constants + match-helper locations | [scheduled bbox exact=1/8] python decl at src/tomli/_re.py:26 (t=7015, 19 atoms) |
| 2.5 | 1160 | 3098 | +1938 | 0.88 | late | README intro paragraph | [scheduled bbox exact=14/17] README.md section #1 (t=3098, 14 atoms) |
| 2.7 | 1904 | 8437 | +6533 | 1.00 | late | _parser.py: parse_* and skip_* function locations | [scheduled bbox exact=12/22] python decl names surface #2 in src/tomli/_parser.py (t=7759, 24 atoms) |
| 2.9 | 2392 | 9295 | +6903 | 0.81 | late | README usage: parse a file + handle errors | [scheduled bbox exact=10/26] README.md section #6 (t=9295, 10 atoms) |
| 3.1 | 2491 | 6201 | +3710 | 1.00 | late | load body | [scheduled bbox exact=8/8] python decl body at src/tomli/_parser.py:137 (t=6201, 8 atoms) |
| 3.9 | 5138 | 6614 | +1476 | 0.84 | aligned | Flags — __init__, add_pending, finalize_pending, unset_all | [scheduled bbox exact=8/19] python method sigs #1 in src/tomli/_parser.py (t=5632, 8 atoms) |
| 5.2 | 9402 | 8039 | -1363 | 0.86 | aligned | skip_chars / skip_until / skip_comment / skip_comments_and_array_ws | [scheduled bbox exact=11/49] python decl body at src/tomli/_parser.py:327 (t=7470, 11 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 8 | 556 | CHANGELOG.md section #<n> |
| 4 | 347 | tomllib.md section #<n> |
| 4 | 274 | README.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 315 | 1.00 | 315 | 9933 | python decl body at fuzzer/fuzz.py:20 |
| 251 | 1.00 | 251 | 3526 | python decl names surface in src/tomli/_parser.py |
| 239 | 1.00 | 239 | 7015 | python decl at src/tomli/_re.py:26 |
| 189 | 0.50 | 379 | 946 | headings outline in README.md |
| 184 | 1.00 | 184 | 9535 | python decl body at src/tomli/_parser.py:463 |
| 165 | 1.00 | 165 | 9111 | python decl body at src/tomli/_parser.py:481 |
| 162 | 1.00 | 162 | 1896 | tomllib.md section #0 |
| 161 | 1.00 | 161 | 4741 | python decl at src/tomli/_re.py:46 |
| 158 | 1.00 | 158 | 6051 | python decl at src/tomli/_parser.py:57 |
| 150 | 1.00 | 150 | 8189 | python decl body at src/tomli/_parser.py:447 |
| 2638 | — | — | — | +33 more rows |

scores: Sim=0.431 Reached=17/40 Early=0 Late=13 Partial=4 Missing=19 Used=9988/10000

## Verdict

Verdict: wrong-slice bound
Likely primary lever: split walker batches to match NS semantic slices
Evidence: 11 ranking-recoverable (w×gap=1.01), 10 wrong-slice/granularity (w×gap=1.27), 1 no-discovered (w×gap=0.01)
Secondary intervention: free final budget for 11 too-expensive candidates
Loss reasons: 0 predecessor-gated, 11 too-expensive, 0 discovered-unscheduled
Top rows: 1.2, 2.2, 1.3, 3.3, 3.7, ...
Note: likely lever is heuristic; verify `Sim` moves, not just bucket counts.

## Top opportunities

_`w(t)×gap` is a non-additive priority score: Σ exp(-exp_t/τ) × (1 - credit) per row, τ=2000. Same time weighting and credit gap as Sim, but rows can overlap between opportunities — sums across rows are an upper bound on Sim impact, not an additive estimate._

| intervention | rows | w(t)×gap | bands ≤3k/≤6k/total | evidence | top row ids |
|:-------------|-----:|---------:|:----------------------|:---------|:------------|
| split wrong-slice walker batches | 10 | 1.27 | 4/6/10 | nearby candidates have low exact atom overlap | 1.2, 2.2, 1.3, 3.3, 3.7, ... |
| free final budget / demote late waste | 11 | 1.01 | 1/5/11 | high-overlap candidates exceed final remaining budget, exact total=237/264 | 2.8, 3.4, 3.5, 3.6, 3.8, ... |
| add walker candidates for no-discovered rows | 1 | 0.01 | 0/0/1 | NS rows have no discovered line candidate | 5.5 |

Tiers: 1=4/6 reached, 2 partial, 0 missing, avg=0.85; 2=7/9 reached, 1 partial, 1 missing, avg=0.82; 3=3/12 reached, 0 partial, 9 missing, avg=0.25; 4=1/8 reached, 0 partial, 7 missing, avg=0.15; 5=2/5 reached, 1 partial, 2 missing, avg=0.55

## Diagnosis rollup

| diagnosis | rows | missing | partial | timing | likely lever |
|:----------|-----:|--------:|--------:|-------:|:-------------|
| ranking-recoverable | 11 | 11 | 0 | 0 | value/ranking |
| wrong-slice / granularity | 10 | 6 | 4 | 0 | walker granularity / wrong slice |
| no discovered candidate | 1 | 1 | 0 | 0 | walker coverage or predecessor-gated emit |
| fs/listing | 1 | 1 | 0 | 0 | filesystem/listing value |
| timing-only | 14 | 0 | 0 | 14 | usually no code change |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | w(t)×gap | likely lever |
|:------------|-----:|---------:|:-------------|
| too expensive at final margin | 11 | 1.01 | free final budget |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=27, unscheduled bbox=6, fs-only=3, no discovered candidate=1

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | aligned | low | 1 |
| scheduled bbox | late | none | 1 |
| scheduled bbox | late | low | 5 |
| scheduled bbox | late | high | 3 |
| scheduled bbox | late | full | 2 |
| scheduled bbox | missing | low | 11 |
| scheduled bbox | partial | low | 4 |
| unscheduled bbox | missing | low | 1 |
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
| 4.2 | 6679 | — | — | 0.05 | missing | create_list_rule body — [[arr]] header | [scheduled bbox exact=2/21] python decl names surface in src/tomli/_parser.py (t=5096, 2 atoms); better unscheduled exact=18/21: python decl body at src/tomli/_parser.py:390 (18 atoms, too expensive at final margin) |
| 4.3 | 7159 | — | — | 0.06 | missing | key_value_rule body | [scheduled bbox exact=2/31] python decl at src/tomli/_parser.py:413 (t=5197, 2 atoms); better unscheduled exact=26/31: python decl body at src/tomli/_parser.py:413 (26 atoms, too expensive at final margin) |
| 4.4 | 7480 | — | — | 0.10 | missing | parse_inline_table — head + first key/value insert | [scheduled bbox exact=2/21] python decl at src/tomli/_parser.py:528 (t=5379, 2 atoms); better unscheduled exact=18/21: python decl body at src/tomli/_parser.py:528 (18 atoms, too expensive at final margin) |
| 4.5 | 7650 | — | — | 0.00 | missing | parse_inline_table — comma/close loop tail | [unscheduled bbox exact=12/12] python decl body at src/tomli/_parser.py:528 (12 atoms, too expensive at final margin) |
| 4.7 | 8290 | — | — | 0.00 | missing | parse_basic_str body | [scheduled bbox exact=1/29] python decl names surface in src/tomli/_parser.py (t=5096, 1 atoms); better unscheduled exact=29/29: python decl body at src/tomli/_parser.py:652 (29 atoms, too expensive at final margin) |
| 5.1 | 8921 | — | — | 0.12 | missing | README FAQ: type mapping table | [scheduled bbox exact=2/17] headings outline in README.md (t=946, 2 atoms); better unscheduled exact=16/17: README.md section #11 (16 atoms, too expensive at final margin) |

### wrong-slice / granularity

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 87 | — | — | 0.67 | partial | README title + tagline | [scheduled bbox exact=2/3] README headline in README.md (t=149, 2 atoms) |
| 1.3 | 166 | — | — | 0.75 | partial | Public API: __init__ __all__ + version | [scheduled bbox exact=3/4] python imports in src/tomli/__init__.py (t=379, 3 atoms) |
| 2.2 | 491 | — | — | 0.67 | partial | _types.py — full | [scheduled bbox exact=3/6] python decl names surface in src/tomli/_types.py (t=411, 3 atoms) |
| 3.3 | 2972 | — | — | 0.00 | missing | tests/* test method names | [unscheduled bbox exact=7/19] python test names surface in tests/test_misc.py (13 atoms, too expensive at final margin) |
| 3.7 | 4502 | — | — | 0.05 | missing | parse_value — dispatch head (strings/bools/array/inline-table) | [scheduled bbox exact=2/44] python decl at src/tomli/_parser.py:684 (t=5256, 2 atoms); better unscheduled exact=35/44: python decl body at src/tomli/_parser.py:684 (35 atoms, too expensive at final margin) |
| 3.10 | 5527 | — | — | 0.08 | missing | Flags — set + is_ (the actual lookup) | [scheduled bbox exact=4/27] python method sigs in src/tomli/_parser.py (t=6055, 4 atoms); better unscheduled exact=15/27: python method body at src/tomli/_parser.py:260 (15 atoms, too expensive at final margin) |
| 3.12 | 6145 | — | — | 0.12 | missing | README usage: Decimal floats | [scheduled bbox exact=2/17] headings outline in README.md (t=946, 2 atoms); better unscheduled exact=12/17: README.md section #6 (12 atoms, too expensive at final margin) |
| 4.6 | 7937 | — | — | 0.09 | missing | parse_array body | [scheduled bbox exact=2/23] python decl at src/tomli/_parser.py:502 (t=5291, 2 atoms); better unscheduled exact=18/23: python decl body at src/tomli/_parser.py:502 (18 atoms, too expensive at final margin) |
| 4.8 | 8626 | — | — | 0.07 | missing | README usage: tomllib compat shim | [scheduled bbox exact=2/28] headings outline in README.md (t=946, 2 atoms); better unscheduled exact=21/28: README.md section #7 (21 atoms, too expensive at final margin) |
| 5.3 | 9565 | — | — | 0.77 | partial | CHANGELOG: 2.4 + 2.1 entries | [scheduled bbox exact=6/13] CHANGELOG.md section #5 (t=3666, 6 atoms) |

### no discovered candidate

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 5.5 | 9973 | — | — | 0.00 | missing | pyproject.toml [project] block | no discovered line candidate |

### fs/listing

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 3.2 | 2746 | — | — | 0.00 | missing | tests/data/{valid,invalid}/ top listing | fs-only |

### timing-only

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.4 | 192 | 305 | +113 | 1.00 | late | src/tomli/ module listing | fs-only |
| 1.5 | 279 | 5134 | +4855 | 0.80 | late | loads / load signatures + docstrings | [scheduled bbox exact=0/5] python decl body at src/tomli/_parser.py:137 (t=6733, 8 atoms) |
| 1.6 | 403 | 5711 | +5308 | 0.90 | late | TOMLDecodeError class + docstring | [scheduled bbox exact=8/10] python decl doc at src/tomli/_parser.py:76 (t=5711, 8 atoms) |
| 2.1 | 433 | 9307 | +8874 | 1.00 | late | tests/ directory listing | fs-only |
| 2.3 | 697 | 6159 | +5462 | 0.88 | late | _parser.py: state-class headers + Flags constants | [scheduled bbox exact=2/17] python method sigs in src/tomli/_parser.py (t=6055, 22 atoms) |
| 2.4 | 844 | 1986 | +1142 | 1.00 | late | _re.py: regex constants + match-helper locations | [scheduled bbox exact=1/8] python decl at src/tomli/_re.py:26 (t=4225, 19 atoms) |
| 2.5 | 1160 | 2642 | +1482 | 0.88 | late | README intro paragraph | [scheduled bbox exact=14/17] README.md section #1 (t=2642, 14 atoms) |
| 2.7 | 1904 | 5096 | +3192 | 1.00 | late | _parser.py: parse_* and skip_* function locations | [scheduled bbox exact=22/22] python decl names surface in src/tomli/_parser.py (t=5096, 43 atoms) |
| 2.9 | 2392 | 8531 | +6139 | 0.81 | late | README usage: parse a file + handle errors | [scheduled bbox exact=10/26] README.md section #5 (t=8180, 10 atoms) |
| 3.1 | 2491 | 6733 | +4242 | 1.00 | late | load body | [scheduled bbox exact=8/8] python decl body at src/tomli/_parser.py:137 (t=6733, 8 atoms) |
| 3.9 | 5138 | 6897 | +1759 | 0.84 | late | Flags — __init__, add_pending, finalize_pending, unset_all | [scheduled bbox exact=8/19] python method sigs in src/tomli/_parser.py (t=6055, 8 atoms) |
| 3.11 | 5894 | 9755 | +3861 | 0.94 | late | NestedDict body — table-tree builder | [scheduled bbox exact=10/31] python method body at src/tomli/_parser.py:283 (t=9755, 10 atoms) |
| 4.1 | 6395 | 9988 | +3593 | 0.83 | late | create_dict_rule body — [table] header | [scheduled bbox exact=15/18] python decl body at src/tomli/_parser.py:370 (t=9988, 15 atoms) |
| 5.2 | 9402 | 7701 | -1701 | 0.86 | aligned | skip_chars / skip_until / skip_comment / skip_comments_and_array_ws | [scheduled bbox exact=11/49] python decl body at src/tomli/_parser.py:327 (t=7701, 11 atoms) |

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
| 164 | 1.00 | 164 | 9501 | CHANGELOG.md section #7 |
| 162 | 1.00 | 162 | 1800 | tomllib.md section #0 |
| 161 | 1.00 | 161 | 3566 | python decl at src/tomli/_re.py:46 |
| 158 | 1.00 | 158 | 6544 | python decl at src/tomli/_parser.py:57 |
| 2845 | — | — | — | +32 more rows |

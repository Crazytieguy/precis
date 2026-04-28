scores: Sim=0.434 Reached=15/40 Early=0 Late=10 Partial=6 Missing=19 Used=9953/10000

## Verdict

Verdict: wrong-slice bound
Likely primary lever: split walker batches to match NS semantic slices
Evidence: 4 ranking-recoverable (w×gap=0.50), 20 wrong-slice/granularity (w×gap=2.00), 0 no-discovered (w×gap=0.00)
Secondary intervention: free final budget for 4 too-expensive candidates
Loss reasons: 0 predecessor-gated, 4 too-expensive, 0 discovered-unscheduled
Top rows: 1.2, 2.2, 1.3, 3.3, 3.1, ...
Note: likely lever is heuristic; verify `Sim` moves, not just bucket counts.

## Top opportunities

_`w(t)×gap` is a non-additive priority score: Σ exp(-exp_t/τ) × (1 - credit) per row, τ=2000. Same time weighting and credit gap as Sim, but rows can overlap between opportunities — sums across rows are an upper bound on Sim impact, not an additive estimate._

| intervention | rows | w(t)×gap | bands ≤3k/≤6k/total | evidence | top row ids |
|:-------------|-----:|---------:|:----------------------|:---------|:------------|
| split wrong-slice walker batches | 20 | 2.00 | 5/10/20 | nearby candidates have low exact atom overlap | 1.2, 2.2, 1.3, 3.3, 3.1, ... |
| free final budget / demote late waste | 4 | 0.50 | 1/2/4 | high-overlap candidates exceed final remaining budget, exact total=79/88 | 2.8, 3.5, 4.5, 5.1 |

Tiers: 1=4/6 reached, 2 partial, 0 missing, avg=0.85; 2=7/9 reached, 1 partial, 1 missing, avg=0.82; 3=2/12 reached, 1 partial, 9 missing, avg=0.24; 4=0/8 reached, 1 partial, 7 missing, avg=0.14; 5=2/5 reached, 1 partial, 2 missing, avg=0.57

## Diagnosis rollup

| diagnosis | rows | missing | partial | timing | likely lever |
|:----------|-----:|--------:|--------:|-------:|:-------------|
| ranking-recoverable | 4 | 4 | 0 | 0 | value/ranking |
| wrong-slice / granularity | 20 | 14 | 6 | 0 | walker granularity / wrong slice |
| fs/listing | 1 | 1 | 0 | 0 | filesystem/listing value |
| timing-only | 12 | 0 | 0 | 12 | usually no code change |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | w(t)×gap | likely lever |
|:------------|-----:|---------:|:-------------|
| too expensive at final margin | 4 | 0.50 | free final budget |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=28, unscheduled bbox=6, fs-only=3

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | aligned | low | 2 |
| scheduled bbox | late | low | 5 |
| scheduled bbox | late | high | 3 |
| scheduled bbox | missing | low | 12 |
| scheduled bbox | partial | low | 6 |
| unscheduled bbox | missing | low | 4 |
| unscheduled bbox | missing | high | 1 |
| unscheduled bbox | missing | full | 1 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 2.8 | 2100 | — | — | 0.10 | missing | README usage: parse a TOML string | [scheduled bbox exact=2/20] headings outline in README.md (t=946, 2 atoms); better unscheduled exact=16/20: README.md section #3 (16 atoms, too expensive at final margin) |
| 3.5 | 3755 | — | — | 0.00 | missing | loads body — rule dispatch + statement terminator | [unscheduled bbox exact=35/39] python decl body at src/tomli/_parser.py:149 body 167 (35 atoms, too expensive at final margin) |
| 4.5 | 7650 | — | — | 0.00 | missing | parse_inline_table — comma/close loop tail | [unscheduled bbox exact=12/12] python decl body at src/tomli/_parser.py:528 body 538 (12 atoms, too expensive at final margin) |
| 5.1 | 8921 | — | — | 0.12 | missing | README FAQ: type mapping table | [scheduled bbox exact=2/17] headings outline in README.md (t=946, 2 atoms); better unscheduled exact=16/17: README.md section #24 (16 atoms, too expensive at final margin) |

### wrong-slice / granularity

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 87 | — | — | 0.67 | partial | README title + tagline | [scheduled bbox exact=2/3] README headline in README.md (t=149, 2 atoms) |
| 1.3 | 166 | — | — | 0.75 | partial | Public API: __init__ __all__ + version | [scheduled bbox exact=3/4] python imports in src/tomli/__init__.py (t=379, 3 atoms) |
| 2.2 | 491 | — | — | 0.67 | partial | _types.py — full | [scheduled bbox exact=3/6] python decl names surface in src/tomli/_types.py (t=411, 3 atoms) |
| 3.1 | 2491 | — | — | 0.25 | missing | load body | [scheduled bbox exact=1/8] python decl body at src/tomli/_parser.py:137 body 146 (t=8047, 1 atoms); better unscheduled exact=6/8: python decl body at src/tomli/_parser.py:137 body 140 (6 atoms, too expensive at final margin) |
| 3.3 | 2972 | — | — | 0.00 | missing | tests/* test method names | [unscheduled bbox exact=7/19] python test names surface in tests/test_misc.py (13 atoms, too expensive at final margin) |
| 3.4 | 3320 | — | — | 0.00 | missing | loads body — prelude + skip / dispatch comments | [unscheduled bbox exact=11/27] python decl body at src/tomli/_parser.py:149 body 167 (11 atoms, too expensive at final margin) |
| 3.6 | 3977 | — | — | 0.00 | missing | TOMLDecodeError.__init__ — pos→line/col body | [unscheduled bbox exact=4/18] python method body at src/tomli/_parser.py:87 body 123 (4 atoms, too expensive at final margin) |
| 3.7 | 4502 | — | — | 0.05 | missing | parse_value — dispatch head (strings/bools/array/inline-table) | [scheduled bbox exact=2/44] python decl at src/tomli/_parser.py:684 (t=8302, 2 atoms); better unscheduled exact=7/44: python decl body at src/tomli/_parser.py:684 body 687 (7 atoms, too expensive at final margin) |
| 3.8 | 4897 | — | — | 0.00 | missing | parse_value — datetime/number/special-float tail | [unscheduled bbox exact=6/29] python decl body at src/tomli/_parser.py:684 body 732 (6 atoms, too expensive at final margin) |
| 3.10 | 5527 | — | — | 0.08 | missing | Flags — set + is_ (the actual lookup) | [scheduled bbox exact=4/27] python method sigs #1 in src/tomli/_parser.py (t=5537, 4 atoms); better unscheduled exact=9/27: python method body at src/tomli/_parser.py:249 body 250 (9 atoms, too expensive at final margin) |
| 3.12 | 6145 | — | — | 0.76 | partial | README usage: Decimal floats | [scheduled bbox exact=7/17] README.md section #7 (t=4485, 7 atoms) |
| 4.1 | 6395 | — | — | 0.00 | missing | create_dict_rule body — [table] header | [scheduled bbox exact=1/18] python decl names surface #2 in src/tomli/_parser.py (t=7576, 1 atoms); better unscheduled exact=4/18: python decl body at src/tomli/_parser.py:370 body 383 (4 atoms, too expensive at final margin) |
| 4.2 | 6679 | — | — | 0.05 | missing | create_list_rule body — [[arr]] header | [scheduled bbox exact=2/21] python decl names surface #2 in src/tomli/_parser.py (t=7576, 2 atoms); better unscheduled exact=4/21: python decl body at src/tomli/_parser.py:390 body 406 (4 atoms, too expensive at final margin) |
| 4.3 | 7159 | — | — | 0.06 | missing | key_value_rule body | [scheduled bbox exact=2/31] python decl at src/tomli/_parser.py:413 (t=7639, 2 atoms); better unscheduled exact=7/31: python decl body at src/tomli/_parser.py:413 body 421 (7 atoms, too expensive at final margin) |
| 4.4 | 7480 | — | — | 0.10 | missing | parse_inline_table — head + first key/value insert | [scheduled bbox exact=2/21] python decl at src/tomli/_parser.py:528 (t=7762, 2 atoms); better unscheduled exact=12/21: python decl body at src/tomli/_parser.py:528 body 538 (12 atoms, too expensive at final margin) |
| 4.6 | 7937 | — | — | 0.09 | missing | parse_array body | [scheduled bbox exact=2/23] python decl at src/tomli/_parser.py:502 (t=7674, 2 atoms); better unscheduled exact=13/23: python decl body at src/tomli/_parser.py:502 body 511 (13 atoms, too expensive at final margin) |
| 4.7 | 8290 | — | — | 0.07 | missing | parse_basic_str body | [scheduled bbox exact=1/29] python decl body at src/tomli/_parser.py:652 body 660 (t=9373, 1 atoms); better unscheduled exact=21/29: python decl body at src/tomli/_parser.py:652 body 661 (21 atoms, too expensive at final margin) |
| 4.8 | 8626 | — | — | 0.79 | partial | README usage: tomllib compat shim | [scheduled bbox exact=9/28] README.md section #12 (t=4749, 9 atoms) |
| 5.3 | 9565 | — | — | 0.77 | partial | CHANGELOG: 2.4 + 2.1 entries | [scheduled bbox exact=6/13] CHANGELOG.md section #5 (t=6207, 6 atoms) |
| 5.5 | 9973 | — | — | 0.11 | missing | pyproject.toml [project] block | [scheduled bbox exact=3/27] [package] in pyproject.toml (t=9363, 3 atoms) |

### fs/listing

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 3.2 | 2746 | — | — | 0.00 | missing | tests/data/{valid,invalid}/ top listing | fs-only |

### timing-only

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.4 | 192 | 305 | +113 | 1.00 | late | src/tomli/ module listing | fs-only |
| 1.5 | 279 | 5000 | +4721 | 0.80 | late | loads / load signatures + docstrings | [scheduled bbox exact=4/5] python decl names surface #1 in src/tomli/_parser.py (t=4962, 4 atoms) |
| 1.6 | 403 | 5281 | +4878 | 0.90 | late | TOMLDecodeError class + docstring | [scheduled bbox exact=8/10] python decl doc at src/tomli/_parser.py:76 (t=5281, 8 atoms) |
| 2.1 | 433 | 9413 | +8980 | 1.00 | late | tests/ directory listing | fs-only |
| 2.3 | 697 | 5641 | +4944 | 0.88 | late | _parser.py: state-class headers + Flags constants | [scheduled bbox exact=2/17] python method sigs #1 in src/tomli/_parser.py (t=5537, 22 atoms) |
| 2.4 | 844 | 2115 | +1271 | 1.00 | late | _re.py: regex constants + match-helper locations | [scheduled bbox exact=1/8] python decl at src/tomli/_re.py:26 (t=6832, 19 atoms) |
| 2.5 | 1160 | 3111 | +1951 | 0.88 | late | README intro paragraph | [scheduled bbox exact=14/17] README.md section #1 (t=3111, 14 atoms) |
| 2.7 | 1904 | 8268 | +6364 | 1.00 | late | _parser.py: parse_* and skip_* function locations | [scheduled bbox exact=12/22] python decl names surface #2 in src/tomli/_parser.py (t=7576, 24 atoms) |
| 2.9 | 2392 | 8839 | +6447 | 0.81 | late | README usage: parse a file + handle errors | [scheduled bbox exact=10/26] README.md section #6 (t=8839, 10 atoms) |
| 3.9 | 5138 | 6420 | +1282 | 0.84 | aligned | Flags — __init__, add_pending, finalize_pending, unset_all | [scheduled bbox exact=8/19] python method sigs #1 in src/tomli/_parser.py (t=5537, 8 atoms) |
| 3.11 | 5894 | 9861 | +3967 | 0.94 | late | NestedDict body — table-tree builder | [scheduled bbox exact=10/31] python method body at src/tomli/_parser.py:283 body 289 (t=9861, 10 atoms) |
| 5.2 | 9402 | 7856 | -1546 | 0.86 | aligned | skip_chars / skip_until / skip_comment / skip_comments_and_array_ws | [scheduled bbox exact=11/49] python decl body at src/tomli/_parser.py:327 body 335 (t=7287, 11 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 12 | 1069 | CHANGELOG.md section #<n> |
| 5 | 457 | tomllib.md section #<n> |
| 4 | 274 | README.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 251 | 1.00 | 251 | 3539 | python decl names surface in src/tomli/_parser.py |
| 239 | 1.00 | 239 | 6832 | python decl at src/tomli/_re.py:26 |
| 189 | 0.50 | 379 | 946 | headings outline in README.md |
| 164 | 1.00 | 164 | 9607 | CHANGELOG.md section #7 |
| 162 | 1.00 | 162 | 1896 | tomllib.md section #0 |
| 161 | 1.00 | 161 | 4646 | python decl at src/tomli/_re.py:46 |
| 158 | 1.00 | 158 | 5956 | python decl at src/tomli/_parser.py:57 |
| 150 | 1.00 | 150 | 8006 | python decl body at src/tomli/_parser.py:447 body 450 |
| 150 | 1.00 | 150 | 4241 | python imports in src/tomli/_parser.py |
| 148 | 1.00 | 148 | 8655 | python decl body at src/tomli/_parser.py:599 body 600 |
| 2569 | — | — | — | +33 more rows |

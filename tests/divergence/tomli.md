scores: Score(3000)=0.424 ns_rows≤3K=18/40 (reached=4 partial=3 missing=11)

## Per-budget scores

| B | A_B | I(B) | C(B) | Score(B) | walker_used |
|--:|----:|-----:|-----:|---------:|------------:|
| 1000 | 83 | 0.802 | 0.389 | 0.558 | 993 |
| 1442 | 100 | 0.802 | 0.324 | 0.509 | 1369 |
| 2080 | 138 | 0.755 | 0.350 | 0.514 | 1964 |
| 3000 | 251 | 0.705 | 0.254 | 0.424 | 2823 |
| 4327 | 335 | 0.701 | 0.235 | 0.406 | 4324 |
| 6240 | 502 | 0.755 | 0.260 | 0.443 | 6207 |
| 9000 | 702 | 0.783 | 0.290 | 0.477 | 8963 |

## Verdict

Verdict: wrong-slice bound
Likely primary lever: split walker batches to match NS semantic slices
Evidence: 4 ranking-recoverable (gap@3k=0.13), 26 wrong-slice/granularity (gap@3k=0.80), 0 no-discovered (gap@3k=0.00)
Secondary intervention: free T_max budget for 4 too-expensive candidates
Top rows: 2.3, 2.7, 2.9, 3.3, 1.2, ...

## Top opportunities

_`gap@B` is a non-additive priority score: `Σ over atoms with rank ≤ |A_B|: (1 − damped_credit(a)) / rank(a)`. `gap@3k` is the primary sort key — direct proxy for `Score(3000)` headroom. `gap@1k` and `gap@9k` show how the same intervention scales across the budget vector. Rows can overlap between opportunities; sums are upper bounds on Score(B) impact, not additive estimates._

| intervention | rows | gap@1k | gap@3k | gap@9k | evidence | top row ids |
|:-------------|-----:|-------:|-------:|-------:|:---------|:------------|
| split wrong-slice walker batches | 26 | 0.39 | 0.80 | 1.06 | nearby candidates have low exact atom overlap | 2.3, 2.7, 2.9, 3.3, 1.2, ... |
| free T_max budget / demote late waste | 4 | 0.00 | 0.13 | 0.31 | high-overlap candidates exceed remaining budget at T_max (caveat: not 3K-budget — see below), exact total=79/88 | 2.8, 3.5, 4.5, 5.1 |
| finish partially-delivered NS batches | 4 | 0.00 | 0.11 | 0.04 | avg batch completion=0.53 | 2.9, 3.12, 5.3, 5.4 |

## Diagnosis rollup

| diagnosis | rows | missing | partial | likely lever |
|:----------|-----:|--------:|--------:|:-------------|
| ranking-recoverable | 4 | 4 | 0 | value/ranking |
| wrong-slice / granularity | 26 | 23 | 3 | walker granularity / wrong slice |
| fs/listing | 3 | 2 | 1 | filesystem/listing value |
| mixed/unknown | 3 | 3 | 0 | inspect row |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | gap@3k | likely lever |
|:------------|-----:|-------:|:-------------|
| too expensive at final margin | 4 | 0.13 | free T_max budget |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=27, unscheduled bbox=6, fs-only=3

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | missing | low | 21 |
| scheduled bbox | missing | high | 3 |
| scheduled bbox | partial | low | 3 |
| unscheduled bbox | missing | low | 4 |
| unscheduled bbox | missing | high | 1 |
| unscheduled bbox | missing | full | 1 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 2.8 | 2100 | 0.10 | 0.18 | missing | README usage: parse a TOML string | [scheduled bbox exact=2/20] headings outline in README.md (t=946, 2 atoms); better unscheduled exact=16/20: README.md section #3 (16 atoms, too expensive at final margin) |
| 3.5 | 3755 | 0.00 | 0.00 | missing | loads body — rule dispatch + statement terminator | [unscheduled bbox exact=35/39] python decl body at src/tomli/_parser.py:149 body 167 (35 atoms, too expensive at final margin) |
| 4.5 | 7650 | 0.00 | 0.00 | missing | parse_inline_table — comma/close loop tail | [unscheduled bbox exact=12/12] python decl body at src/tomli/_parser.py:528 body 538 (12 atoms, too expensive at final margin) |
| 5.1 | 8921 | 0.12 | 0.06 | missing | README FAQ: type mapping table | [scheduled bbox exact=2/17] headings outline in README.md (t=946, 2 atoms); better unscheduled exact=16/17: README.md section #24 (16 atoms, too expensive at final margin) |

### wrong-slice / granularity

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 1.2 | 87 | 0.67 | 0.96 | partial | README title + tagline | [scheduled bbox exact=2/3] README headline in README.md (t=149, 2 atoms) |
| 1.3 | 166 | 0.75 | 0.99 | partial | Public API: __init__ __all__ + version | [scheduled bbox exact=3/4] python imports in src/tomli/__init__.py (t=290, 3 atoms) |
| 2.2 | 491 | 0.67 | 0.84 | partial | _types.py — full | [scheduled bbox exact=3/6] python decl names surface in src/tomli/_types.py (t=411, 3 atoms) |
| 2.3 | 697 | 0.00 | 0.00 | missing | _parser.py: state-class headers + Flags constants | [scheduled bbox exact=2/17] python method sigs #1 in src/tomli/_parser.py (t=5537, 22 atoms) |
| 2.7 | 1904 | 0.00 | 0.00 | missing | _parser.py: parse_* and skip_* function locations | [scheduled bbox exact=12/22] python decl names surface #2 in src/tomli/_parser.py (t=7636, 24 atoms) |
| 2.9 | 2392 | 0.46 | 0.59 | missing | README usage: parse a file + handle errors | [scheduled bbox exact=10/26] README.md section #6 (t=8899, 10 atoms) |
| 3.1 | 2491 | 0.00 | 0.00 | missing | load body | [scheduled bbox exact=1/8] python decl body at src/tomli/_parser.py:137 body 146 (t=8107, 1 atoms); better unscheduled exact=6/8: python decl body at src/tomli/_parser.py:137 body 140 (6 atoms, too expensive at final margin) |
| 3.3 | 2972 | 0.00 | 0.00 | missing | tests/* test method names | [unscheduled bbox exact=7/19] python test names surface in tests/test_misc.py (13 atoms, too expensive at final margin) |
| 3.4 | 3320 | 0.00 | 0.00 | missing | loads body — prelude + skip / dispatch comments | [unscheduled bbox exact=11/27] python decl body at src/tomli/_parser.py:149 body 167 (11 atoms, too expensive at final margin) |
| 3.6 | 3977 | 0.00 | 0.00 | missing | TOMLDecodeError.__init__ — pos→line/col body | [unscheduled bbox exact=4/18] python method body at src/tomli/_parser.py:87 body 123 (4 atoms, too expensive at final margin) |
| 3.7 | 4502 | 0.00 | 0.00 | missing | parse_value — dispatch head (strings/bools/array/inline-table) | [scheduled bbox exact=2/44] python decl at src/tomli/_parser.py:684 (t=8362, 2 atoms); better unscheduled exact=7/44: python decl body at src/tomli/_parser.py:684 body 687 (7 atoms, too expensive at final margin) |
| 3.8 | 4897 | 0.00 | 0.00 | missing | parse_value — datetime/number/special-float tail | [unscheduled bbox exact=6/29] python decl body at src/tomli/_parser.py:684 body 732 (6 atoms, too expensive at final margin) |
| 3.9 | 5138 | 0.00 | 0.00 | missing | Flags — __init__, add_pending, finalize_pending, unset_all | [scheduled bbox exact=8/19] python method sigs #1 in src/tomli/_parser.py (t=5537, 8 atoms) |
| 3.10 | 5527 | 0.00 | 0.00 | missing | Flags — set + is_ (the actual lookup) | [scheduled bbox exact=4/27] python method sigs #1 in src/tomli/_parser.py (t=5537, 4 atoms); better unscheduled exact=9/27: python method body at src/tomli/_parser.py:249 body 250 (9 atoms, too expensive at final margin) |
| 3.11 | 5894 | 0.00 | 0.00 | missing | NestedDict body — table-tree builder | [scheduled bbox exact=10/31] python method body at src/tomli/_parser.py:283 body 289 (t=9861, 10 atoms) |
| 3.12 | 6145 | 0.35 | 0.65 | missing | README usage: Decimal floats | [scheduled bbox exact=7/17] README.md section #7 (t=4485, 7 atoms) |
| 4.1 | 6395 | 0.00 | 0.00 | missing | create_dict_rule body — [table] header | [scheduled bbox exact=1/18] python decl names surface #2 in src/tomli/_parser.py (t=7636, 1 atoms); better unscheduled exact=4/18: python decl body at src/tomli/_parser.py:370 body 383 (4 atoms, too expensive at final margin) |
| 4.2 | 6679 | 0.00 | 0.00 | missing | create_list_rule body — [[arr]] header | [scheduled bbox exact=2/21] python decl names surface #2 in src/tomli/_parser.py (t=7636, 2 atoms); better unscheduled exact=4/21: python decl body at src/tomli/_parser.py:390 body 406 (4 atoms, too expensive at final margin) |
| 4.3 | 7159 | 0.00 | 0.00 | missing | key_value_rule body | [scheduled bbox exact=2/31] python decl at src/tomli/_parser.py:413 (t=7699, 2 atoms); better unscheduled exact=7/31: python decl body at src/tomli/_parser.py:413 body 421 (7 atoms, too expensive at final margin) |
| 4.4 | 7480 | 0.00 | 0.00 | missing | parse_inline_table — head + first key/value insert | [scheduled bbox exact=2/21] python decl at src/tomli/_parser.py:528 (t=7822, 2 atoms); better unscheduled exact=12/21: python decl body at src/tomli/_parser.py:528 body 538 (12 atoms, too expensive at final margin) |
| 4.6 | 7937 | 0.00 | 0.00 | missing | parse_array body | [scheduled bbox exact=2/23] python decl at src/tomli/_parser.py:502 (t=7734, 2 atoms); better unscheduled exact=13/23: python decl body at src/tomli/_parser.py:502 body 511 (13 atoms, too expensive at final margin) |
| 4.7 | 8290 | 0.00 | 0.00 | missing | parse_basic_str body | [scheduled bbox exact=1/29] python decl body at src/tomli/_parser.py:652 body 660 (t=9433, 1 atoms); better unscheduled exact=21/29: python decl body at src/tomli/_parser.py:652 body 661 (21 atoms, too expensive at final margin) |
| 4.8 | 8626 | 0.07 | 0.13 | missing | README usage: tomllib compat shim | [scheduled bbox exact=9/28] README.md section #12 (t=4749, 9 atoms) |
| 5.2 | 9402 | 0.00 | 0.00 | missing | skip_chars / skip_until / skip_comment / skip_comments_and_array_ws | [scheduled bbox exact=11/49] python decl body at src/tomli/_parser.py:327 body 335 (t=7347, 11 atoms) |
| 5.3 | 9565 | 0.31 | 0.26 | missing | CHANGELOG: 2.4 + 2.1 entries | [scheduled bbox exact=6/13] CHANGELOG.md section #5 (t=6207, 6 atoms) |
| 5.5 | 9973 | 0.00 | 0.00 | missing | pyproject.toml [project] block | [scheduled bbox exact=3/27] [package] in pyproject.toml (t=9423, 3 atoms) |

### fs/listing

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 2.1 | 433 | 0.00 | 0.00 | missing | tests/ directory listing | fs-only |
| 3.2 | 2746 | 0.00 | 0.00 | missing | tests/data/{valid,invalid}/ top listing | fs-only |
| 5.4 | 9623 | 0.62 | 0.62 | partial | benchmark/, fuzzer/, profiler/, scripts/, .github/ listings | fs-only |

### mixed/unknown

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 1.5 | 279 | 0.00 | 0.00 | missing | loads / load signatures + docstrings | [scheduled bbox exact=4/5] python decl names surface #1 in src/tomli/_parser.py (t=4962, 4 atoms) |
| 1.6 | 403 | 0.00 | 0.00 | missing | TOMLDecodeError class + docstring | [scheduled bbox exact=8/10] python decl doc at src/tomli/_parser.py:76 (t=5281, 8 atoms) |
| 2.5 | 1160 | 0.12 | 0.03 | missing | README intro paragraph | [scheduled bbox exact=14/17] README.md section #1 (t=3111, 14 atoms) |

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
| 150 | 1.00 | 150 | 8066 | python decl body at src/tomli/_parser.py:447 body 450 |
| 150 | 1.00 | 150 | 4241 | python imports in src/tomli/_parser.py |
| 148 | 1.00 | 148 | 8715 | python decl body at src/tomli/_parser.py:599 body 600 |
| 2569 | — | — | — | +33 more rows |

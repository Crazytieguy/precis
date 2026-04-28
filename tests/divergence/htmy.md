scores: Sim=0.342 Reached=11/40 Early=3 Late=7 Partial=9 Missing=20 Used=9938/10000

## Verdict

Verdict: wrong-slice bound
Likely primary lever: split walker batches to match NS semantic slices
Evidence: 4 ranking-recoverable (w×gap=1.36), 22 wrong-slice/granularity (w×gap=2.17), 2 no-discovered (w×gap=0.96)
Secondary intervention: free final budget for 4 too-expensive candidates
Loss reasons: 0 predecessor-gated, 4 too-expensive, 0 discovered-unscheduled
Top rows: 1.3, 2.2, 2.3, 2.5, 2.4, ...
Note: likely lever is heuristic; verify `Sim` moves, not just bucket counts.

## Top opportunities

_`w(t)×gap` is a non-additive priority score: Σ exp(-exp_t/τ) × (1 - credit) per row, τ=2000. Same time weighting and credit gap as Sim, but rows can overlap between opportunities — sums across rows are an upper bound on Sim impact, not an additive estimate._

| intervention | rows | w(t)×gap | bands ≤3k/≤6k/total | evidence | top row ids |
|:-------------|-----:|---------:|:----------------------|:---------|:------------|
| split wrong-slice walker batches | 22 | 2.17 | 6/15/22 | nearby candidates have low exact atom overlap | 1.3, 2.2, 2.3, 2.5, 2.4, ... |
| free final budget / demote late waste | 4 | 1.36 | 2/2/4 | high-overlap candidates exceed final remaining budget, exact total=120/135 | 1.8, 1.6, 3.2, 3.3 |
| add walker candidates for no-discovered rows | 2 | 0.96 | 1/1/2 | NS rows have no discovered line candidate | 1.2, 4.3 |

Tiers: 1=7/13 reached, 2 partial, 4 missing, avg=0.68; 2=4/21 reached, 7 partial, 10 missing, avg=0.51; 3=0/3 reached, 0 partial, 3 missing, avg=0.00; 4=0/3 reached, 0 partial, 3 missing, avg=0.00

## Diagnosis rollup

| diagnosis | rows | missing | partial | timing | likely lever |
|:----------|-----:|--------:|--------:|-------:|:-------------|
| ranking-recoverable | 4 | 4 | 0 | 0 | value/ranking |
| wrong-slice / granularity | 22 | 13 | 9 | 0 | walker granularity / wrong slice |
| no discovered candidate | 2 | 2 | 0 | 0 | walker coverage or predecessor-gated emit |
| fs/listing | 1 | 1 | 0 | 0 | filesystem/listing value |
| timing-only | 10 | 0 | 0 | 10 | usually no code change |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | w(t)×gap | likely lever |
|:------------|-----:|---------:|:-------------|
| too expensive at final margin | 4 | 1.36 | free final budget |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=30, unscheduled bbox=5, fs-only=2, no discovered candidate=2

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | early | low | 2 |
| scheduled bbox | late | low | 2 |
| scheduled bbox | late | full | 5 |
| scheduled bbox | missing | none | 2 |
| scheduled bbox | missing | low | 10 |
| scheduled bbox | partial | low | 9 |
| unscheduled bbox | missing | low | 2 |
| unscheduled bbox | missing | high | 1 |
| unscheduled bbox | missing | full | 2 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.6 | 394 | — | — | 0.25 | missing | README key features (top half) | [scheduled bbox exact=2/8] headings outline in README.md (t=1089, 2 atoms); better unscheduled exact=7/8: README.md section #1 (7 atoms, too expensive at final margin) |
| 1.8 | 671 | — | — | 0.00 | missing | README key features (bottom half) | [unscheduled bbox exact=8/8] README.md section #1 (8 atoms, too expensive at final margin) |
| 3.2 | 8131 | — | — | 0.00 | missing | html.py — block + form tag inventory (locations) | [unscheduled bbox exact=41/41] python decl names surface in htmy/html.py (41 atoms, too expensive at final margin) |
| 3.3 | 8784 | — | — | 0.00 | missing | html.py — inline + table + heading tag inventory (locations) | [unscheduled bbox exact=64/78] python decl names surface in htmy/html.py (64 atoms, too expensive at final margin) |

### wrong-slice / granularity

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.3 | 168 | — | — | 0.60 | partial | README H1 + tagline | [scheduled bbox exact=2/5] README headline in README.md (t=59, 2 atoms) |
| 1.11 | 1115 | — | — | 0.75 | partial | Public exports — utility helpers + HTMY alias | [scheduled bbox exact=7/12] python imports in htmy/__init__.py (t=4061, 7 atoms) |
| 2.1 | 1694 | — | — | 0.65 | partial | Component protocol heart (SyncComponent / AsyncComponent) | [scheduled bbox exact=5/17] python decl names surface in htmy/typing.py (t=8245, 5 atoms) |
| 2.2 | 1871 | — | — | 0.29 | missing | Component / ComponentType / ComponentSequence type aliases | [scheduled bbox exact=4/14] python decl names surface in htmy/typing.py (t=8245, 4 atoms) |
| 2.3 | 2190 | — | — | 0.30 | missing | Context / Properties / PropertyValue type aliases | [scheduled bbox exact=8/33] python decl names surface in htmy/typing.py (t=8245, 8 atoms) |
| 2.4 | 2429 | — | — | 0.42 | missing | Context provider protocols | [scheduled bbox exact=5/26] python decl names surface in htmy/typing.py (t=8245, 5 atoms) |
| 2.5 | 3034 | — | — | 0.20 | missing | RendererType + StreamingRendererType protocols | [scheduled bbox exact=8/60] python decl names surface in htmy/renderer/typing.py (t=2498, 8 atoms) |
| 2.6 | 3169 | — | — | 0.54 | partial | Default renderer wiring (renderer/__init__.py) | [scheduled bbox exact=4/13] python imports in htmy/renderer/__init__.py (t=656, 4 atoms) |
| 2.7 | 3325 | — | — | 0.43 | missing | Renderer.__init__ + render signatures | [scheduled bbox exact=5/14] python decl doc at htmy/renderer/default.py:228 (t=5970, 5 atoms); better unscheduled exact=6/14: python method at htmy/renderer/default.py:238 (6 atoms, predecessor not scheduled: python method sigs in htmy/renderer/default.py) |
| 2.8 | 3558 | — | — | 0.54 | partial | BaselineRenderer (streaming) signatures | [scheduled bbox exact=6/17] python method at htmy/renderer/baseline.py:30 (t=9903, 6 atoms); better unscheduled exact=7/17: python decl doc at htmy/renderer/baseline.py:18 (7 atoms, too expensive at final margin) |
| 2.10 | 4419 | — | — | 0.21 | missing | Fragment / WithContext signatures | [scheduled bbox exact=4/38] python decl names surface in htmy/core.py (t=1411, 4 atoms); better unscheduled exact=8/38: python method sigs in htmy/core.py (8 atoms, too expensive at final margin) |
| 2.11 | 4603 | — | — | 0.07 | missing | ContextAware base class — public methods | [scheduled bbox exact=0/16] python class body at htmy/core.py:64 (t=1600, 2 atoms); better unscheduled exact=6/16: python method sigs in htmy/core.py (8 atoms, too expensive at final margin) |
| 2.12 | 5046 | — | — | 0.72 | partial | @component decorator — props+context (function/method) | [scheduled bbox exact=12/36] python method sigs in htmy/function_component.py (t=8690, 30 atoms) |
| 2.14 | 5669 | — | — | 0.70 | partial | Snippet / Slots class signatures | [scheduled bbox exact=6/26] python method sigs in htmy/snippet.py (t=5171, 9 atoms) |
| 2.15 | 5899 | — | — | 0.49 | missing | MD / MarkdownParser class signatures | [scheduled bbox exact=6/19] python method sigs in htmy/md/core.py (t=7702, 10 atoms); better unscheduled exact=9/19: python method at htmy/md/core.py:103 (9 atoms, too expensive at final margin) |
| 2.16 | 6141 | — | — | 0.70 | partial | I18n class signature | [scheduled bbox exact=8/20] python decl names surface in htmy/i18n.py (t=1908, 9 atoms) |
| 2.18 | 6542 | — | — | 0.29 | missing | ETreeConverter class signature | [scheduled bbox exact=0/18] python class body at htmy/etree.py:23 (t=3366, 6 atoms); better unscheduled exact=11/18: python decl doc at htmy/etree.py:23 (11 atoms, too expensive at final margin) |
| 2.19 | 6703 | — | — | 0.10 | missing | Formatter class signature | [scheduled bbox exact=2/12] python decl names surface in htmy/core.py (t=1411, 2 atoms); better unscheduled exact=6/12: python method sigs in htmy/core.py (9 atoms, too expensive at final margin) |
| 2.20 | 7214 | — | — | 0.56 | partial | SafeStr / Text / XBool / SkipProperty + xml_format_string | [scheduled bbox exact=10/45] python decl names surface in htmy/core.py (t=1411, 10 atoms) |
| 3.1 | 7838 | — | — | 0.00 | missing | html.py — top of file (DOCTYPE + first tags + Link) | [unscheduled bbox exact=13/40] python decl names surface in htmy/html.py (13 atoms, too expensive at final margin) |
| 4.1 | 9299 | — | — | 0.00 | missing | Snippet docstring — the 4-step pipeline + warning | [scheduled bbox exact=1/29] python decl names surface in htmy/snippet.py (t=340, 1 atoms); better unscheduled exact=23/29: python decl doc at htmy/snippet.py:158 (23 atoms, too expensive at final margin) |
| 4.2 | 9652 | — | — | 0.00 | missing | README — default attribute-formatting rules | [unscheduled bbox exact=7/9] README.md section #10 (7 atoms, too expensive at final margin) |

### no discovered candidate

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 100 | — | — | 0.00 | missing | Project name + description (pyproject lede) | no discovered line candidate |
| 4.3 | 9808 | — | — | 0.00 | missing | tests/conftest.py — renderer fixtures | no discovered line candidate |

### fs/listing

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.13 | 1519 | — | — | 0.29 | missing | docs/ + tests/ + examples/ directory listings | fs-only |

### timing-only

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.4 | 238 | 4061 | +3823 | 1.00 | late | Public exports — renderer + decorator | [scheduled bbox exact=5/5] python imports in htmy/__init__.py (t=4061, 5 atoms) |
| 1.5 | 253 | 4061 | +3808 | 1.00 | late | Project version | [scheduled bbox exact=1/1] python imports in htmy/__init__.py (t=4061, 1 atoms) |
| 1.7 | 516 | 4061 | +3545 | 1.00 | late | Public exports — core utility classes | [scheduled bbox exact=9/9] python imports in htmy/__init__.py (t=4061, 9 atoms) |
| 1.9 | 739 | 4061 | +3322 | 1.00 | late | Public exports — Snippet/Tag | [scheduled bbox exact=5/5] python imports in htmy/__init__.py (t=4061, 5 atoms) |
| 1.10 | 967 | 4061 | +3094 | 1.00 | late | Public exports — typing aliases | [scheduled bbox exact=16/16] python imports in htmy/__init__.py (t=4061, 16 atoms) |
| 1.12 | 1216 | 564 | -652 | 1.00 | early | htmy/ package directory listing | fs-only |
| 2.9 | 4044 | 9440 | +5396 | 0.83 | late | Tag / TagWithProps / wildcard_tag signatures | [scheduled bbox exact=9/41] python decl doc at htmy/tag.py:84 (t=9440, 9 atoms) |
| 2.13 | 5399 | 9119 | +3720 | 0.86 | late | @component decorator — context-only (function/method) | [scheduled bbox exact=12/28] python method sigs in htmy/function_component.py (t=8690, 18 atoms) |
| 2.17 | 6275 | 3150 | -3125 | 0.92 | early | ErrorBoundary class signature | [scheduled bbox exact=6/13] python method at htmy/error_boundary.py:25 (t=3150, 6 atoms) |
| 2.21 | 7383 | 2829 | -4554 | 1.00 | early | Utility helpers — public function signatures | [scheduled bbox exact=8/11] python decl names surface in htmy/utils.py (t=2689, 13 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 6 | 546 | README.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 215 | 1.00 | 215 | 4551 | headings outline in docs/index.md |
| 204 | 0.95 | 214 | 1089 | headings outline in README.md |
| 187 | 0.90 | 208 | 4958 | python decl names surface in htmy/function_component.py |
| 120 | 1.00 | 120 | 9661 | README.md section #5 |
| 118 | 1.00 | 118 | 7488 | README.md section #4 |
| 106 | 1.00 | 106 | 7594 | python decl doc at htmy/i18n.py:147 |
| 100 | 1.00 | 100 | 7939 | python imports in htmy/etree.py |
| 96 | 1.00 | 96 | 7312 | python imports in htmy/function_component.py |
| 94 | 1.00 | 94 | 6575 | python decl doc at htmy/i18n.py:115 |
| 94 | 1.00 | 94 | 7127 | python imports in htmy/error_boundary.py |
| 1982 | — | — | — | +29 more rows |

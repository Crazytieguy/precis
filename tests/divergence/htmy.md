scores: Sim=0.369 Reached=13/40 Early=3 Late=8 Partial=9 Missing=18 Used=9955/10000

## Verdict

Verdict: wrong-slice bound
Likely primary lever: split walker batches to match NS semantic slices
Evidence: 0 ranking-recoverable (w×gap=0.00), 24 wrong-slice/granularity (w×gap=2.52), 2 no-discovered (w×gap=0.96)
Secondary intervention: split wrong-slice batches for 24 rows
Loss reasons: 0 predecessor-gated, 0 too-expensive, 0 discovered-unscheduled
Top rows: 1.3, 2.2, 2.4, 2.3, 2.5, ...
Note: likely lever is heuristic; verify `Sim` moves, not just bucket counts.

## Top opportunities

_`w(t)×gap` is a non-additive priority score: Σ exp(-exp_t/τ) × (1 - credit) per row, τ=2000. Same time weighting and credit gap as Sim, but rows can overlap between opportunities — sums across rows are an upper bound on Sim impact, not an additive estimate._

| intervention | rows | w(t)×gap | bands ≤3k/≤6k/total | evidence | top row ids |
|:-------------|-----:|---------:|:----------------------|:---------|:------------|
| split wrong-slice walker batches | 24 | 2.52 | 6/16/24 | nearby candidates have low exact atom overlap | 1.3, 2.2, 2.4, 2.3, 2.5, ... |
| add walker candidates for no-discovered rows | 2 | 0.96 | 1/1/2 | NS rows have no discovered line candidate | 1.2, 4.3 |

Tiers: 1=9/13 reached, 2 partial, 2 missing, avg=0.82; 2=3/21 reached, 7 partial, 11 missing, avg=0.45; 3=1/3 reached, 0 partial, 2 missing, avg=0.29; 4=0/3 reached, 0 partial, 3 missing, avg=0.00

## Diagnosis rollup

| diagnosis | rows | missing | partial | timing | likely lever |
|:----------|-----:|--------:|--------:|-------:|:-------------|
| wrong-slice / granularity | 24 | 15 | 9 | 0 | walker granularity / wrong slice |
| no discovered candidate | 2 | 2 | 0 | 0 | walker coverage or predecessor-gated emit |
| fs/listing | 1 | 1 | 0 | 0 | filesystem/listing value |
| timing-only | 12 | 0 | 0 | 12 | usually no code change |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=32, unscheduled bbox=3, fs-only=2, no discovered candidate=2

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | aligned | low | 1 |
| scheduled bbox | early | low | 2 |
| scheduled bbox | late | low | 3 |
| scheduled bbox | late | full | 5 |
| scheduled bbox | missing | none | 2 |
| scheduled bbox | missing | low | 10 |
| scheduled bbox | partial | low | 9 |
| unscheduled bbox | missing | low | 3 |

## Arrival ledger by diagnosis

### wrong-slice / granularity

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.3 | 168 | — | — | 0.60 | partial | README H1 + tagline | [scheduled bbox exact=2/5] README headline in README.md (t=59, 2 atoms) |
| 1.11 | 1115 | — | — | 0.75 | partial | Public exports — utility helpers + HTMY alias | [scheduled bbox exact=7/12] python imports in htmy/__init__.py (t=4402, 7 atoms) |
| 2.1 | 1694 | — | — | 0.65 | partial | Component protocol heart (SyncComponent / AsyncComponent) | [scheduled bbox exact=5/17] python decl names surface in htmy/typing.py (t=6985, 5 atoms) |
| 2.2 | 1871 | — | — | 0.07 | missing | Component / ComponentType / ComponentSequence type aliases | [scheduled bbox exact=1/14] python decl at htmy/typing.py:56 (t=6985, 1 atoms); better unscheduled exact=3/14: python decl names surface #1 in htmy/typing.py (3 atoms, too expensive at final margin) |
| 2.3 | 2190 | — | — | 0.30 | missing | Context / Properties / PropertyValue type aliases | [scheduled bbox exact=8/33] python decl names surface in htmy/typing.py (t=6985, 8 atoms) |
| 2.4 | 2429 | — | — | 0.00 | missing | Context provider protocols | [unscheduled bbox exact=5/26] python decl names surface #1 in htmy/typing.py (5 atoms, too expensive at final margin) |
| 2.5 | 3034 | — | — | 0.20 | missing | RendererType + StreamingRendererType protocols | [scheduled bbox exact=8/60] python decl names surface in htmy/renderer/typing.py (t=2764, 8 atoms) |
| 2.6 | 3169 | — | — | 0.54 | partial | Default renderer wiring (renderer/__init__.py) | [scheduled bbox exact=4/13] python imports in htmy/renderer/__init__.py (t=656, 4 atoms) |
| 2.7 | 3325 | — | — | 0.43 | missing | Renderer.__init__ + render signatures | [scheduled bbox exact=5/14] python decl doc at htmy/renderer/default.py:228 (t=6344, 5 atoms); better unscheduled exact=6/14: python method at htmy/renderer/default.py:238 (6 atoms, predecessor not scheduled: python method sigs in htmy/renderer/default.py) |
| 2.8 | 3558 | — | — | 0.07 | missing | BaselineRenderer (streaming) signatures | [scheduled bbox exact=2/17] python decl names surface in htmy/renderer/baseline.py (t=665, 2 atoms); better unscheduled exact=7/17: python decl doc at htmy/renderer/baseline.py:18 (7 atoms, too expensive at final margin) |
| 2.9 | 4044 | — | — | 0.61 | partial | Tag / TagWithProps / wildcard_tag signatures | [scheduled bbox exact=8/41] python method sigs in htmy/tag.py (t=7931, 8 atoms); better unscheduled exact=9/41: python decl doc at htmy/tag.py:84 (9 atoms, too expensive at final margin) |
| 2.10 | 4419 | — | — | 0.21 | missing | Fragment / WithContext signatures | [scheduled bbox exact=4/38] python decl names surface in htmy/core.py (t=1634, 4 atoms); better unscheduled exact=8/38: python method sigs in htmy/core.py (8 atoms, too expensive at final margin) |
| 2.11 | 4603 | — | — | 0.07 | missing | ContextAware base class — public methods | [scheduled bbox exact=0/16] python class body at htmy/core.py:64 (t=1823, 2 atoms); better unscheduled exact=6/16: python method sigs in htmy/core.py (8 atoms, too expensive at final margin) |
| 2.12 | 5046 | — | — | 0.72 | partial | @component decorator — props+context (function/method) | [scheduled bbox exact=12/36] python method sigs in htmy/function_component.py (t=9315, 30 atoms) |
| 2.14 | 5669 | — | — | 0.70 | partial | Snippet / Slots class signatures | [scheduled bbox exact=6/26] python method sigs in htmy/snippet.py (t=5545, 9 atoms) |
| 2.15 | 5899 | — | — | 0.49 | missing | MD / MarkdownParser class signatures | [scheduled bbox exact=6/19] python method sigs in htmy/md/core.py (t=8842, 10 atoms); better unscheduled exact=9/19: python method at htmy/md/core.py:103 (9 atoms, too expensive at final margin) |
| 2.16 | 6141 | — | — | 0.70 | partial | I18n class signature | [scheduled bbox exact=8/20] python decl names surface in htmy/i18n.py (t=2174, 9 atoms) |
| 2.18 | 6542 | — | — | 0.29 | missing | ETreeConverter class signature | [scheduled bbox exact=0/18] python class body at htmy/etree.py:23 (t=3707, 6 atoms); better unscheduled exact=11/18: python decl doc at htmy/etree.py:23 (11 atoms, too expensive at final margin) |
| 2.19 | 6703 | — | — | 0.10 | missing | Formatter class signature | [scheduled bbox exact=2/12] python decl names surface in htmy/core.py (t=1634, 2 atoms); better unscheduled exact=6/12: python method sigs in htmy/core.py (9 atoms, too expensive at final margin) |
| 2.20 | 7214 | — | — | 0.56 | partial | SafeStr / Text / XBool / SkipProperty + xml_format_string | [scheduled bbox exact=10/45] python decl names surface in htmy/core.py (t=1634, 10 atoms) |
| 3.2 | 8131 | — | — | 0.05 | missing | html.py — block + form tag inventory (locations) | [scheduled bbox exact=2/41] python decl names surface in htmy/html.py (t=6513, 2 atoms); better unscheduled exact=12/41: python decl names surface #3 in htmy/html.py (12 atoms, too expensive at final margin) |
| 3.3 | 8784 | — | — | 0.00 | missing | html.py — inline + table + heading tag inventory (locations) | [unscheduled bbox exact=15/78] python class body at htmy/html.py:831 (15 atoms, predecessor not scheduled: python decl at htmy/html.py:831) |
| 4.1 | 9299 | — | — | 0.00 | missing | Snippet docstring — the 4-step pipeline + warning | [scheduled bbox exact=1/29] python decl names surface in htmy/snippet.py (t=340, 1 atoms); better unscheduled exact=23/29: python decl doc at htmy/snippet.py:158 (23 atoms, too expensive at final margin) |
| 4.2 | 9652 | — | — | 0.00 | missing | README — default attribute-formatting rules | [unscheduled bbox exact=6/9] README.md section #35 (6 atoms, too expensive at final margin) |

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
| 1.4 | 238 | 4402 | +4164 | 1.00 | late | Public exports — renderer + decorator | [scheduled bbox exact=5/5] python imports in htmy/__init__.py (t=4402, 5 atoms) |
| 1.5 | 253 | 4402 | +4149 | 1.00 | late | Project version | [scheduled bbox exact=1/1] python imports in htmy/__init__.py (t=4402, 1 atoms) |
| 1.6 | 394 | 1350 | +956 | 1.00 | late | README key features (top half) | [scheduled bbox exact=2/8] headings outline in README.md (t=1089, 2 atoms) |
| 1.7 | 516 | 4402 | +3886 | 1.00 | late | Public exports — core utility classes | [scheduled bbox exact=9/9] python imports in htmy/__init__.py (t=4402, 9 atoms) |
| 1.8 | 671 | 1532 | +861 | 1.00 | late | README key features (bottom half) | [scheduled bbox exact=1/8] README.md section #10 (t=2851, 1 atoms) |
| 1.9 | 739 | 4402 | +3663 | 1.00 | late | Public exports — Snippet/Tag | [scheduled bbox exact=5/5] python imports in htmy/__init__.py (t=4402, 5 atoms) |
| 1.10 | 967 | 4402 | +3435 | 1.00 | late | Public exports — typing aliases | [scheduled bbox exact=16/16] python imports in htmy/__init__.py (t=4402, 16 atoms) |
| 1.12 | 1216 | 564 | -652 | 1.00 | early | htmy/ package directory listing | fs-only |
| 2.13 | 5399 | 9744 | +4345 | 0.86 | late | @component decorator — context-only (function/method) | [scheduled bbox exact=12/28] python method sigs in htmy/function_component.py (t=9315, 18 atoms) |
| 2.17 | 6275 | 3449 | -2826 | 0.92 | early | ErrorBoundary class signature | [scheduled bbox exact=6/13] python method at htmy/error_boundary.py:25 (t=3449, 6 atoms) |
| 2.21 | 7383 | 3128 | -4255 | 1.00 | early | Utility helpers — public function signatures | [scheduled bbox exact=8/11] python decl names surface in htmy/utils.py (t=2988, 13 atoms) |
| 3.1 | 7838 | 6794 | -1044 | 0.82 | aligned | html.py — top of file (DOCTYPE + first tags + Link) | [scheduled bbox exact=13/40] python decl names surface in htmy/html.py (t=6513, 13 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 5 | 426 | README.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 215 | 1.00 | 215 | 4899 | headings outline in docs/index.md |
| 204 | 0.95 | 214 | 1089 | headings outline in README.md |
| 187 | 0.90 | 208 | 5332 | python decl names surface in htmy/function_component.py |
| 118 | 1.00 | 118 | 8606 | README.md section #17 |
| 106 | 1.00 | 106 | 8712 | python decl doc at htmy/i18n.py:147 |
| 100 | 1.00 | 100 | 9115 | python imports in htmy/etree.py |
| 96 | 1.00 | 96 | 8430 | python imports in htmy/function_component.py |
| 94 | 1.00 | 94 | 7693 | python decl doc at htmy/i18n.py:115 |
| 94 | 1.00 | 94 | 8245 | python imports in htmy/error_boundary.py |
| 89 | 1.00 | 89 | 8334 | README.md section #45 |
| 1703 | — | — | — | +25 more rows |

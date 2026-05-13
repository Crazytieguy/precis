scores: Score(3000)=0.475 ns_rows≤3K=17/40 (reached=6 partial=1 missing=10)

## Per-budget scores

| B | A_B | I(B) | C(B) | compl(B) | Score(B) | walker_used |
|--:|----:|-----:|-----:|---------:|---------:|------------:|
| 1000 | 73 | 0.719 | 0.205 | 0.643 | 0.383 | 952 |
| 1442 | 108 | 0.735 | 0.457 | 0.704 | 0.580 | 1427 |
| 2080 | 201 | 0.703 | 0.346 | 0.729 | 0.493 | 2061 |
| 3000 | 260 | 0.722 | 0.313 | 0.720 | 0.475 | 2997 |
| 4327 | 405 | 0.686 | 0.242 | 0.613 | 0.408 | 4253 |
| 6240 | 588 | 0.692 | 0.248 | 0.610 | 0.414 | 6170 |
| 9000 | 846 | 0.682 | 0.318 | 0.632 | 0.466 | 8986 |

## Top opportunities

### Additive (close partial / missing rows)

| intervention | rows | gap@1k | gap@3k | gap@9k | evidence | top row ids |
|:-------------|-----:|-------:|-------:|-------:|:---------|:------------|
| split wrong-slice walker batches | 32 | 2.54 | 2.37 | 1.95 | nearby candidates have low exact atom overlap | 1.2, 2.5, 1.4, 2.3, 1.11, ... |
| tune ranking for high-overlap unscheduled candidates | 1 | 0.25 | 0.25 | 0.25 | high-overlap candidates not in the schedule by T_max, exact total=16/16 | 1.10 |

### Subtractive (suppress consistently off-NS batches)

| pattern | batches | freed@1k | freed@3k | freed@9k | evidence | top batch ids |
|:--------|--------:|---------:|---------:|---------:|:---------|:--------------|
| headings outline in README.md | 1 | 204 | 204 | 204 | off_3k=204 | headings outline in README.md |
| python imports in htmy/md/__init__.py | 1 | 88 | 88 | 88 | off_3k=88 | python imports in htmy/md/__init__.py |
| README.md section #<n> | 1 | 0 | 69 | 426 | off_3k=69 | README.md section #16 |
| python class body at htmy/i18n.py:24 | 1 | 0 | 66 | 66 | off_3k=66 | python class body at htmy/i18n.py:24 |
| headings outline in docs/function-components.md | 1 | 59 | 59 | 59 | off_3k=59 | headings outline in docs/function-components.md |

Top missed paths (NS rows ≤ 3K): htmy/typing.py (4 rows, 90 atoms), tests (1 row, 62 atoms), htmy/__init__.py (4 rows, 38 atoms), README.md (1 row, 5 atoms), pyproject.toml (1 row, 5 atoms)

## Arrival ledger by diagnosis

### ranking-recoverable

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 1.10 | 967 | 0.00 | missing | Public exports — typing aliases | [unscheduled bbox exact=16/16] python imports #8 in htmy/__init__.py (16 atoms, too expensive at final margin) |

### wrong-slice / granularity

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 1.2 | 100 | 0.60 | missing | Project name + description (pyproject lede) | [scheduled bbox exact=3/5] [package] in pyproject.toml (t=120, 3 atoms) |
| 1.3 | 168 | 0.60 | partial | README H1 + tagline | [scheduled bbox exact=2/5] README headline in README.md (t=59, 2 atoms) |
| 1.4 | 238 | 0.40 | missing | Public exports — renderer + decorator | [scheduled bbox exact=2/5] python imports #4 in htmy/__init__.py (t=4614, 2 atoms) |
| 1.9 | 739 | 0.00 | missing | Public exports — Snippet/Tag | [unscheduled bbox exact=3/5] python imports #7 in htmy/__init__.py (3 atoms, discovered unscheduled) |
| 1.11 | 1115 | 0.17 | missing | Public exports — utility helpers + HTMY alias | [scheduled bbox exact=2/12] python decl names surface in htmy/__init__.py (t=251, 2 atoms); better unscheduled exact=7/12: python imports #9 in htmy/__init__.py (7 atoms, too expensive at final margin) |
| 2.1 | 1694 | 0.00 | missing | Component protocol heart (SyncComponent / AsyncComponent) | [scheduled bbox exact=5/17] python decl names surface in htmy/typing.py (t=9531, 5 atoms) |
| 2.2 | 1871 | 0.00 | missing | Component / ComponentType / ComponentSequence type aliases | [scheduled bbox exact=1/14] python decl at htmy/typing.py:56 (t=9531, 1 atoms); better unscheduled exact=3/14: python decl names surface #1 in htmy/typing.py (3 atoms, too expensive at final margin) |
| 2.3 | 2190 | 0.06 | missing | Context / Properties / PropertyValue type aliases | [scheduled bbox exact=8/33] python decl names surface in htmy/typing.py (t=9531, 8 atoms) |
| 2.4 | 2429 | 0.00 | missing | Context provider protocols | [unscheduled bbox exact=5/26] python decl names surface #1 in htmy/typing.py (5 atoms, too expensive at final margin) |
| 2.5 | 3034 | 0.00 | missing | RendererType + StreamingRendererType protocols | [scheduled bbox exact=8/60] python decl names surface in htmy/renderer/typing.py (t=3831, 8 atoms) |
| 2.6 | 3169 | 0.54 | missing | Default renderer wiring (renderer/__init__.py) | [scheduled bbox exact=4/13] python imports in htmy/renderer/__init__.py (t=354, 4 atoms) |
| 2.7 | 3325 | 0.08 | missing | Renderer.__init__ + render signatures | [scheduled bbox exact=5/14] python decl doc at htmy/renderer/default.py:228 (t=5602, 5 atoms); better unscheduled exact=6/14: python method at htmy/renderer/default.py:238 (6 atoms, predecessor not scheduled: python method sigs in htmy/renderer/default.py) |
| 2.8 | 3558 | 0.07 | missing | BaselineRenderer (streaming) signatures | [scheduled bbox exact=2/17] python decl names surface in htmy/renderer/baseline.py (t=395, 2 atoms); better unscheduled exact=7/17: python decl doc at htmy/renderer/baseline.py:18 (7 atoms, too expensive at final margin) |
| 2.9 | 4044 | 0.39 | missing | Tag / TagWithProps / wildcard_tag signatures | [scheduled bbox exact=9/41] python decl doc at htmy/tag.py:84 (t=9360, 9 atoms) |
| 2.10 | 4419 | 0.21 | missing | Fragment / WithContext signatures | [scheduled bbox exact=4/38] python decl names surface in htmy/core.py (t=2163, 4 atoms); better unscheduled exact=8/38: python method sigs in htmy/core.py (8 atoms, too expensive at final margin) |
| 2.11 | 4603 | 0.07 | missing | ContextAware base class — public methods | [scheduled bbox exact=0/16] python class body at htmy/core.py:64 (t=2352, 2 atoms); better unscheduled exact=6/16: python method sigs in htmy/core.py (8 atoms, too expensive at final margin) |
| 2.12 | 5046 | 0.00 | missing | @component decorator — props+context (function/method) | [scheduled bbox exact=12/36] python method sigs in htmy/function_component.py (t=8610, 30 atoms) |
| 2.13 | 5399 | 0.00 | missing | @component decorator — context-only (function/method) | [scheduled bbox exact=12/28] python method sigs in htmy/function_component.py (t=8610, 18 atoms) |
| 2.14 | 5669 | 0.08 | missing | Snippet / Slots class signatures | [scheduled bbox exact=6/26] python method sigs in htmy/snippet.py (t=4926, 9 atoms) |
| 2.15 | 5899 | 0.12 | missing | MD / MarkdownParser class signatures | [scheduled bbox exact=6/19] python method sigs in htmy/md/core.py (t=7746, 10 atoms); better unscheduled exact=9/19: python method at htmy/md/core.py:103 (9 atoms, too expensive at final margin) |
| 2.16 | 6141 | 0.60 | missing | I18n class signature | [scheduled bbox exact=8/20] python decl names surface in htmy/i18n.py (t=2775, 9 atoms) |
| 2.17 | 6275 | 0.54 | missing | ErrorBoundary class signature | [scheduled bbox exact=6/13] python method at htmy/error_boundary.py:25 (t=3114, 6 atoms) |
| 2.18 | 6542 | 0.06 | missing | ETreeConverter class signature | [scheduled bbox exact=0/18] python class body at htmy/etree.py:23 (t=3305, 6 atoms); better unscheduled exact=11/18: python decl doc at htmy/etree.py:23 (11 atoms, too expensive at final margin) |
| 2.19 | 6703 | 0.10 | missing | Formatter class signature | [scheduled bbox exact=2/12] python decl names surface in htmy/core.py (t=2163, 2 atoms); better unscheduled exact=6/12: python method sigs in htmy/core.py (9 atoms, too expensive at final margin) |
| 2.20 | 7214 | 0.32 | missing | SafeStr / Text / XBool / SkipProperty + xml_format_string | [scheduled bbox exact=10/45] python decl names surface in htmy/core.py (t=2163, 10 atoms) |
| 2.21 | 7383 | 0.00 | missing | Utility helpers — public function signatures | [scheduled bbox exact=8/11] python decl names surface in htmy/utils.py (t=4390, 13 atoms) |
| 3.1 | 7838 | 0.00 | missing | html.py — top of file (DOCTYPE + first tags + Link) | [scheduled bbox exact=13/40] python decl names surface in htmy/html.py (t=8109, 13 atoms) |
| 3.2 | 8131 | 0.00 | missing | html.py — block + form tag inventory (locations) | [scheduled bbox exact=2/41] python decl names surface in htmy/html.py (t=8109, 2 atoms); better unscheduled exact=12/41: python decl names surface #3 in htmy/html.py (12 atoms, too expensive at final margin) |
| 3.3 | 8784 | 0.00 | missing | html.py — inline + table + heading tag inventory (locations) | [unscheduled bbox exact=15/78] python class body at htmy/html.py:831 (15 atoms, predecessor not scheduled: python decl at htmy/html.py:831) |
| 4.1 | 9299 | 0.00 | missing | Snippet docstring — the 4-step pipeline + warning | [scheduled bbox exact=1/29] python decl names surface in htmy/snippet.py (t=424, 1 atoms); better unscheduled exact=23/29: python decl doc at htmy/snippet.py:158 (23 atoms, too expensive at final margin) |
| 4.2 | 9652 | 0.00 | missing | README — default attribute-formatting rules | [unscheduled bbox exact=6/9] README.md section #35 (6 atoms, too expensive at final margin) |
| 4.3 | 9808 | 0.00 | missing | tests/conftest.py — renderer fixtures | [unscheduled bbox exact=6/18] python decl names surface in tests/conftest.py (6 atoms, discovered unscheduled) |

### fs/listing

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 1.13 | 1519 | 0.50 | missing | docs/ + tests/ + examples/ directory listings | fs-only |

Top wasted paths (off-NS at 3K): README.md (273t, 2 batches), htmy/i18n.py (268t, 3 batches), htmy/tag.py (119t, 2 batches), htmy/error_boundary.py (107t, 2 batches), htmy/core.py (102t, 1 batch), htmy/md/__init__.py (88t, 1 batch), htmy/renderer/default.py (61t, 1 batch), htmy/renderer/__init__.py (60t, 1 batch), +2 more

## Walker waste (walker_t ≤ 3000, off_3k ≥ 50)

| off_3k | off_3k_ratio | off_any | cost | walker_t | batch |
|-------:|-------------:|--------:|-----:|---------:|:------|
| 204 | 0.95 | 204 | 214 | 952 | headings outline in README.md |
| 109 | 1.00 | 36 | 109 | 2666 | python decl names surface in htmy/i18n.py |
| 102 | 1.00 | 0 | 102 | 2061 | python decl names surface in htmy/core.py |
| 93 | 1.00 | 14 | 93 | 2878 | python method sigs in htmy/i18n.py |
| 88 | 1.00 | 88 | 88 | 424 | python imports in htmy/md/__init__.py |
| 69 | 1.00 | 69 | 69 | 1964 | README.md section #16 |
| 66 | 1.00 | 66 | 66 | 2812 | python class body at htmy/i18n.py:24 |
| 64 | 1.00 | 34 | 64 | 848 | python decl names surface in htmy/tag.py |
| 61 | 1.00 | 52 | 61 | 2587 | python decl names surface in htmy/renderer/default.py |
| 60 | 1.00 | 0 | 60 | 294 | python imports in htmy/renderer/__init__.py |
| 274 | — | — | — | — | +5 more rows |

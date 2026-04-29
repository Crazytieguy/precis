scores: Score(3000)=0.587 ns_rows≤3K=19/42 (reached=11 partial=0 missing=8)

## Per-budget scores

| B | A_B | I(B) | C(B) | Score(B) | walker_used |
|--:|----:|-----:|-----:|---------:|------------:|
| 1000 | 112 | 0.709 | 0.196 | 0.373 | 960 |
| 1442 | 144 | 0.701 | 0.153 | 0.327 | 1308 |
| 2080 | 185 | 0.690 | 0.335 | 0.481 | 1931 |
| 3000 | 254 | 0.731 | 0.472 | 0.587 | 2864 |
| 4327 | 345 | 0.724 | 0.387 | 0.529 | 3828 |
| 6240 | 521 | 0.821 | 0.393 | 0.568 | 6227 |
| 9000 | 743 | 0.836 | 0.455 | 0.617 | 8990 |

## Verdict

Verdict: wrong-slice bound
Likely primary lever: split walker batches to match NS semantic slices
Evidence: 3 ranking-recoverable (gap@3k=0.31), 16 wrong-slice/granularity (gap@3k=0.72), 0 no-discovered (gap@3k=0.00)
Top rows: 5.1, 5.5, 8.3, 7.3, 9.4, ...

## Top opportunities

_`gap@B` is a non-additive priority score: `Σ over atoms in row: (1 − damped_credit(a)) / rank(a)` evaluated at budget B's walker state. Approximates how much closing the row would lift `Score(B)` (via the Importance numerator); not an exact delta. `gap@3k` is the primary sort key. `gap@1k` and `gap@9k` show how the same intervention scales across the budget vector — gap is monotone non-increasing in B (walker has more budget at higher B). Rows can overlap between opportunities; sums are upper bounds on Score(B) impact, not additive estimates._

| intervention | rows | gap@1k | gap@3k | gap@9k | evidence | top row ids |
|:-------------|-----:|-------:|-------:|-------:|:---------|:------------|
| split wrong-slice walker batches | 16 | 0.73 | 0.72 | 0.46 | nearby candidates have low exact atom overlap | 5.1, 5.5, 8.3, 7.3, 9.4, ... |
| tune ranking for high-overlap unscheduled candidates | 3 | 0.31 | 0.31 | 0.31 | high-overlap candidates not in the schedule by T_max, exact total=111/122 | 6.1, 6.2, 3.1 |

## Diagnosis rollup

| diagnosis | rows | missing | partial | likely lever |
|:----------|-----:|--------:|--------:|:-------------|
| ranking-recoverable | 3 | 3 | 0 | value/ranking |
| wrong-slice / granularity | 16 | 16 | 0 | walker granularity / wrong slice |
| fs/listing | 2 | 2 | 0 | filesystem/listing value |
| mixed/unknown | 7 | 7 | 0 | inspect row |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | gap@3k | likely lever |
|:------------|-----:|-------:|:-------------|
| too expensive at final margin | 3 | 0.31 | tune ranking |

Candidate hint kinds: scheduled bbox=19, unscheduled bbox=3, scheduled same-file=3, unscheduled same-file=1, fs-only=2 _(candidates are walker batches discovered this run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` isn't proof that no emit path exists)._

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | missing | low | 12 |
| scheduled bbox | missing | high | 5 |
| scheduled bbox | missing | full | 2 |
| unscheduled bbox | missing | high | 2 |
| unscheduled bbox | missing | full | 1 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 3.1 | 1232 | 0.00 | 0.00 | missing | Architecture — Approach paragraph (DOM-as-truth) | [unscheduled bbox exact=1/1] ARCHITECTURE.md section #1 (1 atoms, too expensive at final margin) |
| 6.1 | 4490 | 0.00 | 0.00 | missing | index.tsx — onKeyDown switch (next/prev/Home/End/Enter) | [unscheduled bbox exact=58/61] export body at cmdk/src/index.tsx:169 body 170 (58 atoms, too expensive at final margin) |
| 6.2 | 5181 | 0.00 | 0.00 | missing | index.tsx — sort() body (DOM-as-truth, in code) | [unscheduled bbox exact=52/60] export body at cmdk/src/index.tsx:169 body 170 (52 atoms, too expensive at final margin) |

### wrong-slice / granularity

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 5.1 | 2516 | 0.00 | 0.00 | missing | README — Item subsection (the most-asked-about part) | [scheduled bbox exact=9/27] README.md section #16 (t=7597, 9 atoms) |
| 5.2 | 2755 | 0.17 | 0.03 | missing | README — Command (root) value/filter/keywords/loop prose | [scheduled bbox exact=1/12] README.md section #9 (t=7894, 14 atoms) |
| 5.4 | 3397 | 0.00 | 0.00 | missing | FAQ — accessibility, virtualization, RSC, etc. | [scheduled bbox exact=6/11] README.md section #35 (t=8506, 6 atoms) |
| 5.5 | 3705 | 0.25 | 0.13 | missing | README — Group / Separator / Empty / Loading subsections | [scheduled bbox exact=8/32] headings outline in README.md (t=2760, 8 atoms) |
| 5.6 | 3856 | 0.20 | 0.09 | missing | useCommandState — state slice subscription hook | [scheduled bbox exact=5/10] README.md section #27 (t=5762, 5 atoms) |
| 7.1 | 5336 | 0.14 | 0.19 | missing | command-score — exported scoring function signature | [scheduled bbox exact=6/8] export body at cmdk/src/command-score.ts:155 body 156 (t=3828, 6 atoms) |
| 7.2 | 5647 | 0.00 | 0.00 | missing | command-score — SCORE_* constants (positive weights) | [scheduled same-file] export body at cmdk/src/command-score.ts:155 body 156 (t=3828, 6 atoms) |
| 7.3 | 6081 | 0.00 | 0.00 | missing | command-score — PENALTY_* constants (decay weights) | [scheduled same-file] export body at cmdk/src/command-score.ts:155 body 156 (t=3828, 6 atoms) |
| 8.1 | 6245 | 0.00 | 0.00 | missing | Test file → describe-block names | [unscheduled same-file] imports in test/props.test.ts (1 atoms, discovered unscheduled) |
| 8.2 | 6516 | 0.00 | 0.00 | missing | Playwright config — test dir + dev-server hookup | [scheduled same-file] imports in playwright.config.ts (t=895, 1 atoms) |
| 8.3 | 6992 | 0.00 | 0.00 | missing | test/pages/keybinds — fixture for every keybind spec | [scheduled bbox exact=1/47] export at test/pages/keybinds.tsx:47 (t=8107, 1 atoms); better unscheduled exact=3/47: imports in test/pages/keybinds.tsx (3 atoms, discovered unscheduled) |
| 9.4 | 8625 | 0.17 | 0.22 | missing | index.tsx — useCmdk + useValue helpers | [scheduled bbox exact=7/41] module item at cmdk/src/index.tsx:1010 (t=1120, 7 atoms); better unscheduled exact=19/41: module item body at cmdk/src/index.tsx:1010 body 1019 (19 atoms, discovered unscheduled) |
| 10.1 | 9027 | 0.20 | 0.01 | missing | Architecture — Discarded approaches (rejected alternatives) | [scheduled bbox exact=2/10] headings outline in ARCHITECTURE.md (t=736, 2 atoms); better unscheduled exact=5/10: ARCHITECTURE.md section #1 (5 atoms, too expensive at final margin) |
| 10.2 | 9480 | 0.00 | 0.00 | missing | test/pages/group — fixture for the group.test specs | [scheduled bbox exact=1/42] export at test/pages/group.tsx:42 (t=8075, 1 atoms); better unscheduled exact=2/42: imports in test/pages/group.tsx (2 atoms, discovered unscheduled) |
| 10.3 | 9603 | 0.71 | 0.36 | missing | Architecture — Performance + Groups bodies | [scheduled bbox exact=4/7] headings outline in ARCHITECTURE.md (t=736, 4 atoms) |
| 10.7 | 9996 | 0.45 | 0.48 | missing | index.tsx — Empty body | [scheduled bbox exact=3/9] export doc at cmdk/src/index.tsx:899 (t=1726, 3 atoms) |

### fs/listing

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 1.6 | 409 | 0.00 | 0.00 | missing | Test app layout — pages + test files | fs-only |
| 1.7 | 549 | 0.00 | 0.00 | missing | Website (showcase) layout — themes + components | fs-only |

### mixed/unknown

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 1.4 | 257 | 0.00 | 0.00 | missing | Library runtime dependencies | [scheduled bbox exact=6/6] package dependencies in cmdk/package.json (t=7184, 6 atoms) |
| 1.8 | 751 | 0.00 | 0.00 | missing | Root scripts — how to build/run/test | [scheduled bbox exact=10/10] package scripts in package.json (t=3066, 10 atoms) |
| 2.3 | 1166 | 0.00 | 0.00 | missing | README — basic Use snippet | [scheduled bbox exact=20/23] README.md section #2 (t=9573, 20 atoms) |
| 5.3 | 3157 | 0.00 | 0.00 | missing | README — Dialog usage with ⌘K keybind | [scheduled bbox exact=32/38] README.md section #2 (t=9573, 32 atoms) |
| 9.1 | 7365 | 0.04 | 0.08 | missing | index.tsx — Item header (registration + state subscriptions) | [scheduled bbox exact=23/28] export body at cmdk/src/index.tsx:664 body 665 (t=4464, 23 atoms) |
| 9.2 | 7686 | 0.00 | 0.00 | missing | index.tsx — Item render output (the cmdk-item div) | [scheduled bbox exact=26/32] export body at cmdk/src/index.tsx:664 body 665 (t=4464, 26 atoms) |
| 9.3 | 8154 | 0.03 | 0.07 | missing | index.tsx — Group component body | [scheduled bbox exact=34/40] export body at cmdk/src/index.tsx:729 body 730 (t=3634, 34 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 13 | 1452 | README.md section #<n> |
| 2 | 118 | export doc at cmdk/src/index.tsx:<n> |

## Walker waste, primary-actionable (first_t ≤ 3000, off-NS spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 135 | 1.00 | 135 | 2341 | module item at cmdk/src/index.tsx:1081 |
| 80 | 0.28 | 290 | 2760 | headings outline in README.md |
| 66 | 1.00 | 66 | 2172 | export doc at cmdk/src/index.tsx:664 |
| 65 | 1.00 | 65 | 960 | module item at cmdk/src/index.tsx:1071 |
| 52 | 1.00 | 52 | 1931 | export doc at cmdk/src/index.tsx:833 |

## Walker waste, late (first_t > 3000, off-NS spend ≥ 50) — higher-budget calibration only

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 386 | 1.00 | 386 | 6203 | ARCHITECTURE.md section #0 |
| 242 | 1.00 | 242 | 5480 | json config tsconfig.json |
| 210 | 1.00 | 210 | 6772 | package identity in cmdk/package.json |
| 160 | 0.43 | 375 | 8506 | README.md section #35 |
| 148 | 1.00 | 148 | 8876 | README.md section #11 |
| 146 | 1.00 | 146 | 8728 | README.md section #12 |
| 145 | 1.00 | 145 | 6917 | package entrypoints in cmdk/package.json |
| 143 | 1.00 | 143 | 5180 | README.md section #36 |
| 133 | 1.00 | 133 | 3199 | package dependencies in package.json |
| 123 | 1.00 | 123 | 7745 | README.md section #6 |
| 1028 | — | — | — | +11 more rows |

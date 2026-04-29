scores: Score(3000)=0.582 ns_rows≤3K=14/33 (reached=7 partial=1 missing=6)

## Per-budget scores

| B | A_B | I(B) | C(B) | Score(B) | walker_used |
|--:|----:|-----:|-----:|---------:|------------:|
| 1000 | 77 | 0.684 | 0.676 | 0.680 | 943 |
| 1442 | 127 | 0.718 | 0.668 | 0.693 | 1337 |
| 2080 | 178 | 0.715 | 0.651 | 0.682 | 2078 |
| 3000 | 237 | 0.692 | 0.489 | 0.582 | 2550 |
| 4327 | 332 | 0.666 | 0.417 | 0.527 | 4299 |
| 6240 | 468 | 0.718 | 0.549 | 0.628 | 6236 |
| 9000 | 662 | 0.705 | 0.425 | 0.547 | 8916 |

## Verdict

Verdict: wrong-slice bound
Likely primary lever: split walker batches to match NS semantic slices
Evidence: 3 ranking-recoverable (gap@3k=0.26), 9 wrong-slice/granularity (gap@3k=1.67), 3 no-discovered (gap@3k=0.23)
Secondary intervention: free T_max budget for 3 too-expensive candidates
Top rows: 1.1, 2.9, 2.6, 3.11, 3.5, ...

## Top opportunities

_`gap@B` is a non-additive priority score: `Σ over atoms in row: (1 − damped_credit(a)) / rank(a)` evaluated at budget B's walker state. Approximates how much closing the row would lift `Score(B)` (via the Importance numerator); not an exact delta. `gap@3k` is the primary sort key. `gap@1k` and `gap@9k` show how the same intervention scales across the budget vector — gap is monotone non-increasing in B (walker has more budget at higher B). Rows can overlap between opportunities; sums are upper bounds on Score(B) impact, not additive estimates._

| intervention | rows | gap@1k | gap@3k | gap@9k | evidence | top row ids |
|:-------------|-----:|-------:|-------:|-------:|:---------|:------------|
| split wrong-slice walker batches | 9 | 1.69 | 1.67 | 1.63 | nearby candidates have low exact atom overlap | 1.1, 2.9, 2.6, 3.11, 3.5, ... |
| free T_max budget / demote late waste | 3 | 0.26 | 0.26 | 0.26 | high-overlap candidates exceed remaining budget at T_max (caveat: not 3K-budget — see below), exact total=102/122 | 3.9, 3.7, 3.4 |
| add walker candidates for no-discovered rows | 3 | 0.23 | 0.23 | 0.23 | NS rows have no discovered line candidate | 2.10, 1.3, 2.11 |
| finish partially-delivered NS batches | 1 | 0.06 | 0.04 | 0.04 | avg batch completion=0.68 | 3.10 |

## Diagnosis rollup

| diagnosis | rows | missing | partial | likely lever |
|:----------|-----:|--------:|--------:|:-------------|
| ranking-recoverable | 3 | 3 | 0 | value/ranking |
| wrong-slice / granularity | 9 | 8 | 1 | walker granularity / wrong slice |
| no discovered candidate | 3 | 3 | 0 | walker coverage or predecessor-gated emit |
| fs/listing | 3 | 3 | 0 | filesystem/listing value |
| mixed/unknown | 6 | 6 | 0 | inspect row |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | gap@3k | likely lever |
|:------------|-----:|-------:|:-------------|
| too expensive at final margin | 3 | 0.26 | free T_max budget |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=15, unscheduled bbox=3, fs-only=3, no discovered candidate=3

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | missing | none | 1 |
| scheduled bbox | missing | low | 7 |
| scheduled bbox | missing | high | 1 |
| scheduled bbox | missing | full | 5 |
| scheduled bbox | partial | low | 1 |
| unscheduled bbox | missing | high | 2 |
| unscheduled bbox | missing | full | 1 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 3.4 | 4688 | 0.00 | 0.00 | missing | Root inner-function locations (onPress, onDrag, onRelease, …) | [unscheduled bbox exact=11/11] export body at src/index.tsx:139 body 170 (384 atoms, too expensive at final margin) |
| 3.7 | 6032 | 0.00 | 0.00 | missing | useSnapPoints — return shape + snapPointsOffset memo | [unscheduled bbox exact=39/47] export body at src/use-snap-points.ts:7 body 30 (178 atoms, too expensive at final margin) |
| 3.9 | 7179 | 0.00 | 0.00 | missing | Root onRelease — close-threshold + velocity decision | [unscheduled bbox exact=52/64] export body at src/index.tsx:139 body 170 (52 atoms, too expensive at final margin) |

### wrong-slice / granularity

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 1.1 | 46 | 0.75 | 0.99 | partial | package.json — name, version, description | [scheduled bbox exact=3/4] package identity in package.json (t=314, 3 atoms) |
| 2.6 | 2011 | 0.00 | 0.00 | missing | Overlay + Content render — what data-vaul-* attributes appear on DOM | [scheduled bbox exact=11/20] export body at src/index.tsx:803 body 805 (t=3748, 11 atoms) |
| 2.9 | 3228 | 0.05 | 0.15 | missing | NestedRoot + Portal — nested-drawer wiring + container override | [scheduled bbox exact=4/38] export names surface in src/index.tsx (t=854, 4 atoms); better unscheduled exact=25/38: export body at src/index.tsx:1098 body 1099 (25 atoms, discovered unscheduled) |
| 3.5 | 5171 | 0.00 | 0.00 | missing | context.ts — DrawerContextValue interface (parent/child contract) | [scheduled bbox exact=2/35] imports in src/context.ts (t=3111, 2 atoms) |
| 3.8 | 6394 | 0.00 | 0.00 | missing | helpers.ts — dampenValue, getTranslate, isVertical bodies | [scheduled bbox exact=10/34] export body at src/helpers.ts:59 body 60 (t=4135, 10 atoms); better unscheduled exact=15/34: export body at src/helpers.ts:72 body 73 (15 atoms, discovered unscheduled) |
| 3.10 | 7585 | 0.45 | 0.68 | missing | browser.ts — every UA-detection function (full file) | [scheduled bbox exact=14/36] export names surface in src/browser.ts (t=2057, 14 atoms) |
| 3.11 | 8357 | 0.02 | 0.02 | missing | useScaleBackground — wrapper-scale effect (full hook) | [scheduled bbox exact=45/60] export body at src/use-scale-background.ts:8 body 9 (t=9549, 45 atoms) |
| 3.12 | 9050 | 0.15 | 0.11 | missing | use-prevent-scroll — exports + the six mobile-Safari quirks | [scheduled bbox exact=0/33] export body at src/use-prevent-scroll.ts:68 body 69 (t=4870, 18 atoms) |
| 4.4 | 9693 | 0.00 | 0.00 | missing | Landing page + Playwright device profiles | [scheduled bbox exact=11/33] export at playwright.config.ts:12 (t=3062, 11 atoms) |

### no discovered candidate

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 1.3 | 171 | 0.00 | 0.00 | missing | README — unmaintained notice | no discovered line candidate |
| 2.10 | 4024 | 0.00 | 0.00 | missing | style.css — every selector head (locations only) | no discovered line candidate |
| 2.11 | 4132 | 0.00 | 0.00 | missing | style.css — keyframe names (locations only) | no discovered line candidate |

### fs/listing

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 4.1 | 9102 | 0.00 | 0.00 | missing | test/ workspace listing | fs-only |
| 4.2 | 9201 | 0.00 | 0.00 | missing | test/src/app/ — every demo page directory | fs-only |
| 4.3 | 9267 | 0.00 | 0.00 | missing | test/tests/ — Playwright spec catalog | fs-only |

### mixed/unknown

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 1.6 | 503 | 0.00 | 0.00 | missing | package.json — runtime + peer deps (Radix dialog, React 16.8–19) | [scheduled bbox exact=8/8] package dependencies in package.json (t=7308, 8 atoms) |
| 2.4 | 1393 | 0.00 | 0.00 | missing | DialogProps — every prop name (signatures only, no JSDoc) | [scheduled bbox exact=28/28] export at src/index.tsx:50 (t=6040, 87 atoms) |
| 2.7 | 2405 | 0.00 | 0.00 | missing | DialogProps — JSDoc for the high-traffic props | [scheduled bbox exact=30/30] export at src/index.tsx:50 (t=6040, 50 atoms) |
| 2.8 | 2810 | 0.00 | 0.00 | missing | DialogProps — JSDoc for the lower-traffic props | [scheduled bbox exact=29/29] export at src/index.tsx:50 (t=6040, 73 atoms) |
| 3.3 | 4502 | 0.00 | 0.00 | missing | helpers.ts — every exported function (signature heads only) | [scheduled bbox exact=8/8] export names surface in src/helpers.ts (t=3919, 15 atoms) |
| 3.6 | 5503 | 0.04 | 0.04 | missing | useSnapPoints — full parameter shape | [scheduled bbox exact=24/28] export at src/use-snap-points.ts:7 (t=4700, 24 atoms) |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 1039 | 1.00 | 1039 | 8658 | export body at src/index.tsx:996 body 1000 |
| 407 | 0.79 | 512 | 3062 | export at playwright.config.ts:12 |
| 327 | 1.00 | 327 | 6808 | export at src/context.ts:37 |
| 311 | 1.00 | 311 | 7619 | imports in src/index.tsx |
| 245 | 1.00 | 245 | 6481 | export body at src/use-controllable-state.ts:39 body 40 |
| 209 | 0.59 | 357 | 7308 | package dependencies in package.json |
| 189 | 0.82 | 230 | 314 | package identity in package.json |
| 174 | 1.00 | 174 | 3308 | package entrypoints in package.json |
| 170 | 1.00 | 170 | 4870 | export body at src/use-prevent-scroll.ts:68 body 69 |
| 156 | 0.51 | 305 | 3748 | export body at src/index.tsx:803 body 805 |
| 1347 | — | — | — | +13 more rows |

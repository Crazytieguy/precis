scores: Score(3000)=0.577 ns_rows≤3K=14/33 (reached=7 partial=1 missing=6)

## Per-budget scores

| B | A_B | I(B) | C(B) | compl(B) | Score(B) | walker_used |
|--:|----:|-----:|-----:|---------:|---------:|------------:|
| 1000 | 77 | 0.673 | 0.623 | 0.818 | 0.648 | 988 |
| 1442 | 127 | 0.718 | 0.668 | 0.873 | 0.693 | 1433 |
| 2080 | 178 | 0.714 | 0.651 | 0.887 | 0.682 | 1875 |
| 3000 | 237 | 0.681 | 0.489 | 0.887 | 0.577 | 2907 |
| 4327 | 332 | 0.668 | 0.417 | 0.800 | 0.528 | 4197 |
| 6240 | 468 | 0.718 | 0.549 | 0.856 | 0.628 | 6236 |
| 9000 | 662 | 0.707 | 0.425 | 0.799 | 0.548 | 8971 |

## Top opportunities

### Additive (close partial / missing rows)

| intervention | rows | gap@1k | gap@3k | gap@9k | evidence | top row ids |
|:-------------|-----:|-------:|-------:|-------:|:---------|:------------|
| split wrong-slice walker batches | 9 | 1.69 | 1.69 | 1.63 | nearby candidates have low exact atom overlap | 1.1, 2.9, 2.6, 3.11, 3.5, ... |
| tune ranking for high-overlap unscheduled candidates | 3 | 0.26 | 0.26 | 0.26 | high-overlap candidates not in the schedule by T_max, exact total=102/122 | 3.9, 3.7, 3.4 |
| add walker candidates for no-discovered rows | 3 | 0.23 | 0.23 | 0.23 | NS rows have no discovered line candidate | 2.10, 1.3, 2.11 |

### Subtractive (suppress consistently off-NS batches)

| pattern | batches | freed@1k | freed@3k | freed@9k | evidence | top batch ids |
|:--------|--------:|---------:|---------:|---------:|:---------|:--------------|
| export at playwright.config.ts:<n> | 1 | 0 | 407 | 407 | off_3k=512 | export at playwright.config.ts:12 |
| package identity in package.json | 1 | 189 | 189 | 189 | off_3k=189 | package identity in package.json |
| package entrypoints in package.json | 1 | 0 | 174 | 174 | off_3k=174 | package entrypoints in package.json |
| package scripts in package.json | 1 | 0 | 135 | 135 | off_3k=135 | package scripts in package.json |
| export at src/use-position-fixed.ts:<n> | 1 | 0 | 124 | 124 | off_3k=124 | export at src/use-position-fixed.ts:15 |

Top missed paths (NS rows ≤ 3K): src/index.tsx (4 rows, 107 atoms), package.json (2 rows, 12 atoms), README.md (1 row, 2 atoms)

## Arrival ledger by diagnosis

### ranking-recoverable

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 3.4 | 4688 | 0.00 | missing | Root inner-function locations (onPress, onDrag, onRelease, …) | [unscheduled bbox exact=11/11] export body at src/index.tsx:139 body 170 (384 atoms, too expensive at final margin) |
| 3.7 | 6032 | 0.00 | missing | useSnapPoints — return shape + snapPointsOffset memo | [unscheduled bbox exact=39/47] export body at src/use-snap-points.ts:7 body 30 (178 atoms, too expensive at final margin) |
| 3.9 | 7179 | 0.00 | missing | Root onRelease — close-threshold + velocity decision | [unscheduled bbox exact=52/64] export body at src/index.tsx:139 body 170 (52 atoms, too expensive at final margin) |

### wrong-slice / granularity

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 1.1 | 46 | 0.75 | partial | package.json — name, version, description | [scheduled bbox exact=3/4] package identity in package.json (t=314, 3 atoms) |
| 2.6 | 2011 | 0.00 | missing | Overlay + Content render — what data-vaul-* attributes appear on DOM | [scheduled bbox exact=11/20] export body at src/index.tsx:803 body 805 (t=3582, 11 atoms) |
| 2.9 | 3228 | 0.05 | missing | NestedRoot + Portal — nested-drawer wiring + container override | [scheduled bbox exact=4/38] export names surface in src/index.tsx (t=962, 4 atoms); better unscheduled exact=25/38: export body at src/index.tsx:1098 body 1099 (25 atoms, discovered unscheduled) |
| 3.5 | 5171 | 0.06 | missing | context.ts — DrawerContextValue interface (parent/child contract) | [scheduled bbox exact=2/35] imports in src/context.ts (t=2436, 2 atoms) |
| 3.8 | 6394 | 0.00 | missing | helpers.ts — dampenValue, getTranslate, isVertical bodies | [scheduled bbox exact=10/34] export body at src/helpers.ts:59 body 60 (t=5754, 10 atoms); better unscheduled exact=15/34: export body at src/helpers.ts:72 body 73 (15 atoms, discovered unscheduled) |
| 3.10 | 7585 | 0.00 | missing | browser.ts — every UA-detection function (full file) | [scheduled bbox exact=14/36] export names surface in src/browser.ts (t=3154, 14 atoms) |
| 3.11 | 8357 | 0.02 | missing | useScaleBackground — wrapper-scale effect (full hook) | [scheduled bbox exact=45/60] export body at src/use-scale-background.ts:8 body 9 (t=9604, 45 atoms) |
| 3.12 | 9050 | 0.15 | missing | use-prevent-scroll — exports + the six mobile-Safari quirks | [scheduled bbox exact=0/33] export body at src/use-prevent-scroll.ts:68 body 69 (t=4197, 18 atoms) |
| 4.4 | 9693 | 0.33 | missing | Landing page + Playwright device profiles | [scheduled bbox exact=11/33] export at playwright.config.ts:12 (t=2387, 11 atoms) |

### no discovered candidate

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 1.3 | 171 | 0.00 | missing | README — unmaintained notice | no discovered line candidate |
| 2.10 | 4024 | 0.00 | missing | style.css — every selector head (locations only) | no discovered line candidate |
| 2.11 | 4132 | 0.00 | missing | style.css — keyframe names (locations only) | no discovered line candidate |

### fs/listing

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 4.1 | 9102 | 0.00 | missing | test/ workspace listing | fs-only |
| 4.2 | 9201 | 0.00 | missing | test/src/app/ — every demo page directory | fs-only |
| 4.3 | 9267 | 0.00 | missing | test/tests/ — Playwright spec catalog | fs-only |

### mixed/unknown

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 1.6 | 503 | 0.00 | missing | package.json — runtime + peer deps (Radix dialog, React 16.8–19) | [scheduled bbox exact=8/8] package dependencies in package.json (t=7363, 8 atoms) |
| 2.4 | 1393 | 0.00 | missing | DialogProps — every prop name (signatures only, no JSDoc) | [scheduled bbox exact=28/28] export at src/index.tsx:50 (t=5367, 87 atoms) |
| 2.7 | 2405 | 0.00 | missing | DialogProps — JSDoc for the high-traffic props | [scheduled bbox exact=30/30] export at src/index.tsx:50 (t=5367, 50 atoms) |
| 2.8 | 2810 | 0.00 | missing | DialogProps — JSDoc for the lower-traffic props | [scheduled bbox exact=29/29] export at src/index.tsx:50 (t=5367, 73 atoms) |
| 3.1 | 4321 | 0.00 | missing | constants.ts — every tunable threshold and timing | [scheduled bbox exact=15/18] export names surface in src/constants.ts (t=3738, 15 atoms) |
| 3.3 | 4502 | 0.00 | missing | helpers.ts — every exported function (signature heads only) | [scheduled bbox exact=8/8] export names surface in src/helpers.ts (t=5538, 15 atoms) |
| 3.6 | 5503 | 0.04 | missing | useSnapPoints — full parameter shape | [scheduled bbox exact=24/28] export at src/use-snap-points.ts:7 (t=4027, 24 atoms) |

Top wasted paths (off-NS at 3K): playwright.config.ts (512t, 1 batch), package.json (498t, 3 batches), src/use-prevent-scroll.ts (235t, 3 batches), src/use-position-fixed.ts (124t, 1 batch), src/context.ts (57t, 1 batch), src/types.ts (52t, 1 batch)

## Walker waste (walker_t ≤ 3000, off_3k ≥ 50)

| off_3k | off_3k_ratio | off_any | cost | walker_t | batch |
|-------:|-------------:|--------:|-----:|---------:|:------|
| 512 | 1.00 | 407 | 512 | 1875 | export at playwright.config.ts:12 |
| 189 | 0.82 | 189 | 230 | 84 | package identity in package.json |
| 174 | 1.00 | 174 | 174 | 2733 | package entrypoints in package.json |
| 135 | 1.00 | 135 | 135 | 2907 | package scripts in package.json |
| 124 | 1.00 | 124 | 124 | 1751 | export at src/use-position-fixed.ts:15 |
| 95 | 1.00 | 10 | 95 | 2459 | export names surface in src/use-prevent-scroll.ts |
| 80 | 1.00 | 80 | 80 | 2653 | export body at src/use-prevent-scroll.ts:34 body 35 |
| 60 | 1.00 | 60 | 60 | 2593 | export body at src/use-prevent-scroll.ts:294 body 295 |
| 57 | 1.00 | 57 | 57 | 682 | export body at src/context.ts:69 body 70 |
| 52 | 1.00 | 0 | 52 | 570 | export names surface in src/types.ts |

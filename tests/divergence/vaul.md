scores: Sim=0.556 Reached=16/33 Early=3 Late=7 Partial=2 Missing=15 Used=9549/10000

## Verdict

Verdict: coverage-gap bound
Likely primary lever: add walker candidates for no-discovered NS rows
Evidence: 3 ranking-recoverable (w×gap=0.17), 8 wrong-slice/granularity (w×gap=0.72), 3 no-discovered (w×gap=1.18)
Secondary intervention: free final budget for 3 too-expensive candidates
Loss reasons: 0 predecessor-gated, 3 too-expensive, 0 discovered-unscheduled
Top rows: 1.3, 2.10, 2.11
Note: likely lever is heuristic; verify `Sim` moves, not just bucket counts.

## Top opportunities

_`w(t)×gap` is a non-additive priority score: Σ exp(-exp_t/τ) × (1 - credit) per row, τ=2000. Same time weighting and credit gap as Sim, but rows can overlap between opportunities — sums across rows are an upper bound on Sim impact, not an additive estimate._

| intervention | rows | w(t)×gap | bands ≤3k/≤6k/total | evidence | top row ids |
|:-------------|-----:|---------:|:----------------------|:---------|:------------|
| add walker candidates for no-discovered rows | 3 | 1.18 | 1/3/3 | NS rows have no discovered line candidate | 1.3, 2.10, 2.11 |
| split wrong-slice walker batches | 8 | 0.72 | 2/4/8 | nearby candidates have low exact atom overlap | 1.1, 2.9, 2.6, 3.5, 3.8, ... |
| free final budget / demote late waste | 3 | 0.17 | 0/1/3 | high-overlap candidates exceed final remaining budget, exact total=102/122 | 3.4, 3.7, 3.9 |

Tiers: 1=4/6 reached, 1 partial, 1 missing, avg=0.79; 2=7/11 reached, 1 partial, 3 missing, avg=0.67; 3=5/12 reached, 0 partial, 7 missing, avg=0.46; 4=0/4 reached, 0 partial, 4 missing, avg=0.08

## Diagnosis rollup

| diagnosis | rows | missing | partial | timing | likely lever |
|:----------|-----:|--------:|--------:|-------:|:-------------|
| ranking-recoverable | 3 | 3 | 0 | 0 | value/ranking |
| wrong-slice / granularity | 8 | 6 | 2 | 0 | walker granularity / wrong slice |
| no discovered candidate | 3 | 3 | 0 | 0 | walker coverage or predecessor-gated emit |
| fs/listing | 3 | 3 | 0 | 0 | filesystem/listing value |
| timing-only | 15 | 0 | 0 | 15 | usually no code change |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | w(t)×gap | likely lever |
|:------------|-----:|---------:|:-------------|
| too expensive at final margin | 3 | 0.17 | free final budget |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=21, unscheduled bbox=3, fs-only=5, no discovered candidate=3

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | aligned | low | 3 |
| scheduled bbox | aligned | high | 1 |
| scheduled bbox | aligned | full | 1 |
| scheduled bbox | early | low | 1 |
| scheduled bbox | early | high | 1 |
| scheduled bbox | late | low | 1 |
| scheduled bbox | late | full | 5 |
| scheduled bbox | missing | none | 1 |
| scheduled bbox | missing | low | 5 |
| scheduled bbox | partial | low | 2 |
| unscheduled bbox | missing | high | 2 |
| unscheduled bbox | missing | full | 1 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 3.4 | 4688 | — | — | 0.00 | missing | Root inner-function locations (onPress, onDrag, onRelease, …) | [unscheduled bbox exact=11/11] export body at src/index.tsx:139 body 170 (384 atoms, too expensive at final margin) |
| 3.7 | 6032 | — | — | 0.00 | missing | useSnapPoints — return shape + snapPointsOffset memo | [unscheduled bbox exact=39/47] export body at src/use-snap-points.ts:7 body 30 (178 atoms, too expensive at final margin) |
| 3.9 | 7179 | — | — | 0.00 | missing | Root onRelease — close-threshold + velocity decision | [unscheduled bbox exact=52/64] export body at src/index.tsx:139 body 170 (52 atoms, too expensive at final margin) |

### wrong-slice / granularity

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.1 | 46 | — | — | 0.75 | partial | package.json — name, version, description | [scheduled bbox exact=3/4] package identity in package.json (t=314, 3 atoms) |
| 2.6 | 2011 | — | — | 0.55 | partial | Overlay + Content render — what data-vaul-* attributes appear on DOM | [scheduled bbox exact=11/20] export body at src/index.tsx:803 body 805 (t=3748, 11 atoms) |
| 2.9 | 3228 | — | — | 0.05 | missing | NestedRoot + Portal — nested-drawer wiring + container override | [scheduled bbox exact=4/38] export names surface in src/index.tsx (t=854, 4 atoms); better unscheduled exact=25/38: export body at src/index.tsx:1098 body 1099 (25 atoms, discovered unscheduled) |
| 3.5 | 5171 | — | — | 0.06 | missing | context.ts — DrawerContextValue interface (parent/child contract) | [scheduled bbox exact=2/35] imports in src/context.ts (t=3111, 2 atoms) |
| 3.8 | 6394 | — | — | 0.38 | missing | helpers.ts — dampenValue, getTranslate, isVertical bodies | [scheduled bbox exact=10/34] export body at src/helpers.ts:59 body 60 (t=4135, 10 atoms); better unscheduled exact=15/34: export body at src/helpers.ts:72 body 73 (15 atoms, discovered unscheduled) |
| 3.10 | 7585 | — | — | 0.45 | missing | browser.ts — every UA-detection function (full file) | [scheduled bbox exact=14/36] export names surface in src/browser.ts (t=2057, 14 atoms) |
| 3.12 | 9050 | — | — | 0.15 | missing | use-prevent-scroll — exports + the six mobile-Safari quirks | [scheduled bbox exact=0/33] export body at src/use-prevent-scroll.ts:68 body 69 (t=4870, 18 atoms) |
| 4.4 | 9693 | — | — | 0.33 | missing | Landing page + Playwright device profiles | [scheduled bbox exact=11/33] export at playwright.config.ts:12 (t=3062, 11 atoms) |

### no discovered candidate

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.3 | 171 | — | — | 0.00 | missing | README — unmaintained notice | no discovered line candidate |
| 2.10 | 4024 | — | — | 0.00 | missing | style.css — every selector head (locations only) | no discovered line candidate |
| 2.11 | 4132 | — | — | 0.00 | missing | style.css — keyframe names (locations only) | no discovered line candidate |

### fs/listing

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 4.1 | 9102 | — | — | 0.00 | missing | test/ workspace listing | fs-only |
| 4.2 | 9201 | — | — | 0.00 | missing | test/src/app/ — every demo page directory | fs-only |
| 4.3 | 9267 | — | — | 0.00 | missing | test/tests/ — Playwright spec catalog | fs-only |

### timing-only

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 108 | 62 | -46 | 1.00 | early | Top-level repo listing | fs-only |
| 1.4 | 242 | 402 | +160 | 1.00 | late | src/ listing — every source file | fs-only |
| 1.5 | 355 | 1325 | +970 | 1.00 | late | Drawer namespace export — the public component map | [scheduled bbox exact=12/12] export at src/index.tsx:1137 (t=1325, 12 atoms) |
| 1.6 | 503 | 7308 | +6805 | 1.00 | late | package.json — runtime + peer deps (Radix dialog, React 16.8–19) | [scheduled bbox exact=8/8] package dependencies in package.json (t=7308, 8 atoms) |
| 2.1 | 642 | 854 | +212 | 1.00 | late | All top-level export locations in src/index.tsx | [scheduled bbox exact=1/11] export at src/index.tsx:50 (t=6040, 88 atoms) |
| 2.2 | 730 | 909 | +179 | 0.82 | aligned | Handle component — snap-point cycle + double-tap timing constants | [scheduled bbox exact=4/11] export at src/index.tsx:996 (t=909, 4 atoms) |
| 2.3 | 1020 | 1224 | +204 | 0.95 | aligned | WithFadeFromProps / WithoutFadeFromProps — snap-point fade contract | [scheduled bbox exact=12/22] export at src/index.tsx:27 (t=1224, 12 atoms) |
| 2.4 | 1393 | 6040 | +4647 | 1.00 | late | DialogProps — every prop name (signatures only, no JSDoc) | [scheduled bbox exact=28/28] export at src/index.tsx:50 (t=6040, 87 atoms) |
| 2.7 | 2405 | 6040 | +3635 | 1.00 | late | DialogProps — JSDoc for the high-traffic props | [scheduled bbox exact=30/30] export at src/index.tsx:50 (t=6040, 50 atoms) |
| 2.8 | 2810 | 6040 | +3230 | 1.00 | late | DialogProps — JSDoc for the lower-traffic props | [scheduled bbox exact=29/29] export at src/index.tsx:50 (t=6040, 73 atoms) |
| 3.1 | 4321 | 2550 | -1771 | 0.94 | early | constants.ts — every tunable threshold and timing | [scheduled bbox exact=15/18] export names surface in src/constants.ts (t=2510, 15 atoms) |
| 3.2 | 4399 | 621 | -3778 | 0.86 | early | types.ts — DrawerDirection, SnapPoint, AnyFunction | [scheduled bbox exact=4/7] export at src/types.ts:2 (t=621, 4 atoms) |
| 3.3 | 4502 | 3919 | -583 | 1.00 | aligned+over | helpers.ts — every exported function (signature heads only) | [scheduled bbox exact=8/8] export names surface in src/helpers.ts (t=3919, 15 atoms) |
| 3.6 | 5503 | 4700 | -803 | 0.82 | aligned | useSnapPoints — full parameter shape | [scheduled bbox exact=24/28] export at src/use-snap-points.ts:7 (t=4700, 24 atoms) |
| 3.11 | 8357 | 9549 | +1192 | 0.83 | aligned | useScaleBackground — wrapper-scale effect (full hook) | [scheduled bbox exact=45/60] export body at src/use-scale-background.ts:8 body 9 (t=9549, 45 atoms) |

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

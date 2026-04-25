scores: Sim=0.424 Reached=17/33 Early=4 Late=11 Partial=2 Missing=14 Used=9863/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 6 | 4 | 1 | 1 | 0.79 |
| 2 | 11 | 7 | 1 | 3 | 0.69 |
| 3 | 12 | 4 | 0 | 8 | 0.38 |
| 4 | 4 | 2 | 0 | 2 | 0.58 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.1 | 46 | — | — | 0.75 | partial | package.json — name, version, description | package identity in package.json (t=303, 3 atoms) |
| 1.2 | 108 | 62 | -46 | 1.00 | early | Top-level repo listing |  |
| 1.3 | 171 | — | — | 0.00 | missing | README — unmaintained notice |  |
| 1.4 | 242 | 828 | +586 | 1.00 | late | src/ listing — every source file |  |
| 1.5 | 355 | 1639 | +1284 | 1.00 | late | Drawer namespace export — the public component map | export at src/index.tsx:1137 (t=1639, 12 atoms) |
| 1.6 | 503 | 6604 | +6101 | 1.00 | late | package.json — runtime + peer deps (Radix dialog, React 16.8–19) | package dependencies in package.json (t=6604, 8 atoms) |
| 2.1 | 642 | 1244 | +602 | 1.00 | late | All top-level export locations in src/index.tsx | export at src/index.tsx:833 (t=8334, 141 atoms) |
| 2.2 | 730 | — | — | 0.64 | partial | Handle component — snap-point cycle + double-tap timing constants | export names surface in src/index.tsx (t=1244, 4 atoms) |
| 2.3 | 1020 | 1538 | +518 | 0.95 | late | WithFadeFromProps / WithoutFadeFromProps — snap-point fade contract | export at src/index.tsx:27 (t=1538, 12 atoms) |
| 2.4 | 1393 | 4502 | +3109 | 1.00 | late | DialogProps — every prop name (signatures only, no JSDoc) | export at src/index.tsx:50 (t=4502, 87 atoms) |
| 2.5 | 1711 | 8923 | +7212 | 1.00 | late | Root signature — full destructuring with every default value | export at src/index.tsx:139 (t=8923, 31 atoms) |
| 2.6 | 2011 | 8334 | +6323 | 1.00 | late | Overlay + Content render — what data-vaul-* attributes appear on DOM | export at src/index.tsx:833 (t=8334, 71 atoms) |
| 2.7 | 2405 | 4502 | +2097 | 1.00 | late | DialogProps — JSDoc for the high-traffic props | export at src/index.tsx:50 (t=4502, 50 atoms) |
| 2.8 | 2810 | 4502 | +1692 | 1.00 | late | DialogProps — JSDoc for the lower-traffic props | export at src/index.tsx:50 (t=4502, 73 atoms) |
| 2.9 | 3228 | — | — | 0.05 | missing | NestedRoot + Portal — nested-drawer wiring + container override | export names surface in src/index.tsx (t=1244, 4 atoms) |
| 2.10 | 4024 | — | — | 0.00 | missing | style.css — every selector head (locations only) |  |
| 2.11 | 4132 | — | — | 0.00 | missing | style.css — keyframe names (locations only) |  |
| 3.1 | 4321 | 2403 | -1918 | 0.94 | early | constants.ts — every tunable threshold and timing | export names surface in src/constants.ts (t=2363, 15 atoms) |
| 3.2 | 4399 | 1021 | -3378 | 0.86 | early | types.ts — DrawerDirection, SnapPoint, AnyFunction | export names surface in src/types.ts (t=1000, 4 atoms) |
| 3.3 | 4502 | 3116 | -1386 | 1.00 | early | helpers.ts — every exported function (signature heads only) | export names surface in src/helpers.ts (t=3116, 15 atoms) |
| 3.4 | 4688 | — | — | 0.00 | missing | Root inner-function locations (onPress, onDrag, onRelease, …) |  |
| 3.5 | 5171 | — | — | 0.06 | missing | context.ts — DrawerContextValue interface (parent/child contract) | imports in src/context.ts (t=5686, 2 atoms) |
| 3.6 | 5503 | 9172 | +3669 | 0.82 | late | useSnapPoints — full parameter shape | export at src/use-snap-points.ts:7 (t=9172, 24 atoms) |
| 3.7 | 6032 | — | — | 0.00 | missing | useSnapPoints — return shape + snapPointsOffset memo |  |
| 3.8 | 6394 | — | — | 0.41 | missing | helpers.ts — dampenValue, getTranslate, isVertical bodies | export at src/helpers.ts:59 (t=3204, 12 atoms) |
| 3.9 | 7179 | — | — | 0.00 | missing | Root onRelease — close-threshold + velocity decision |  |
| 3.10 | 7585 | — | — | 0.20 | missing | browser.ts — every UA-detection function (full file) | export names surface in src/browser.ts (t=2237, 14 atoms) |
| 3.11 | 8357 | — | — | 0.08 | missing | useScaleBackground — wrapper-scale effect (full hook) | imports in src/use-scale-background.ts (t=6173, 4 atoms) |
| 3.12 | 9050 | — | — | 0.15 | missing | use-prevent-scroll — exports + the six mobile-Safari quirks | export names surface in src/use-prevent-scroll.ts (t=1796, 9 atoms) |
| 4.2 | 9201 | — | — | 0.00 | missing | test/src/app/ — every demo page directory |  |
| 4.4 | 9693 | — | — | 0.33 | missing | Landing page + Playwright device profiles | export at playwright.config.ts:12 (t=2915, 11 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 3 | 2789 | export at src/index.tsx:<n> |
| 2 | 389 | export at src/context.ts:<n> |
| 3 | 334 | test/README.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 1607 | 0.93 | 1730 | 8334 | export at src/index.tsx:833 |
| 1025 | 0.96 | 1074 | 5663 | export at src/index.tsx:996 |
| 384 | 0.75 | 512 | 2915 | export at playwright.config.ts:12 |
| 327 | 1.00 | 327 | 6104 | export at src/context.ts:37 |
| 227 | 0.64 | 357 | 6604 | package dependencies in package.json |
| 193 | 0.84 | 230 | 303 | package identity in package.json |
| 174 | 1.00 | 174 | 477 | package entrypoints in package.json |
| 157 | 0.48 | 329 | 2125 | export at src/index.tsx:803 |
| 147 | 1.00 | 147 | 8605 | export doc at src/use-position-fixed.ts:15 |
| 144 | 1.00 | 144 | 9681 | test/README.md section #0 |
| 135 | 1.00 | 135 | 619 | package scripts in package.json |
| 124 | 1.00 | 124 | 8458 | export at src/use-position-fixed.ts:15 |
| 111 | 1.00 | 111 | 730 | json config tsconfig.json |
| 111 | 1.00 | 111 | 3332 | json config turbo.json |
| 104 | 1.00 | 104 | 9525 | test/README.md section #1 |
| 86 | 1.00 | 86 | 9421 | test/README.md section #2 |
| 74 | 1.00 | 74 | 6247 | imports in src/use-snap-points.ts |
| 62 | 1.00 | 62 | 1701 | export at src/context.ts:69 |
| 59 | 1.00 | 59 | 9848 | export at test/tests/helpers.ts:11 |
| 58 | 1.00 | 58 | 4589 | export doc at src/use-prevent-scroll.ts:68 |
| 53 | 0.31 | 171 | 3116 | export names surface in src/helpers.ts |

scores: Sim=0.554 Reached=15/33 Early=3 Late=7 Partial=5 Missing=13 Used=9416/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 6 | 4 | 1 | 1 | 0.79 |
| 2 | 11 | 6 | 3 | 2 | 0.72 |
| 3 | 12 | 5 | 1 | 6 | 0.45 |
| 4 | 4 | 0 | 0 | 4 | 0.08 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.1 | 46 | — | — | 0.75 | partial | package.json — name, version, description | package identity in package.json (t=314, 3 atoms) |
| 1.2 | 108 | 62 | -46 | 1.00 | early | Top-level repo listing |  |
| 1.3 | 171 | — | — | 0.00 | missing | README — unmaintained notice |  |
| 1.4 | 242 | 425 | +183 | 1.00 | late | src/ listing — every source file |  |
| 1.5 | 355 | 1374 | +1019 | 1.00 | late | Drawer namespace export — the public component map | export at src/index.tsx:1137 (t=1374, 12 atoms) |
| 1.6 | 503 | 8119 | +7616 | 1.00 | late | package.json — runtime + peer deps (Radix dialog, React 16.8–19) | package dependencies in package.json (t=8119, 8 atoms) |
| 2.1 | 642 | 851 | +209 | 1.00 | late | All top-level export locations in src/index.tsx | export at src/index.tsx:50 (t=6622, 88 atoms) |
| 2.2 | 730 | — | — | 0.64 | partial | Handle component — snap-point cycle + double-tap timing constants | export names surface in src/index.tsx (t=851, 4 atoms) |
| 2.3 | 1020 | 1273 | +253 | 0.95 | aligned | WithFadeFromProps / WithoutFadeFromProps — snap-point fade contract | export at src/index.tsx:27 (t=1273, 12 atoms) |
| 2.4 | 1393 | 6622 | +5229 | 1.00 | late | DialogProps — every prop name (signatures only, no JSDoc) | export at src/index.tsx:50 (t=6622, 87 atoms) |
| 2.6 | 2011 | — | — | 0.55 | partial | Overlay + Content render — what data-vaul-* attributes appear on DOM | export body at src/index.tsx:803 (t=4161, 11 atoms) |
| 2.7 | 2405 | 6622 | +4217 | 1.00 | late | DialogProps — JSDoc for the high-traffic props | export at src/index.tsx:50 (t=6622, 50 atoms) |
| 2.8 | 2810 | 6622 | +3812 | 1.00 | late | DialogProps — JSDoc for the lower-traffic props | export at src/index.tsx:50 (t=6622, 73 atoms) |
| 2.9 | 3228 | — | — | 0.79 | partial | NestedRoot + Portal — nested-drawer wiring + container override | export body at src/index.tsx:1098 (t=2797, 25 atoms) |
| 2.10 | 4024 | — | — | 0.00 | missing | style.css — every selector head (locations only) |  |
| 2.11 | 4132 | — | — | 0.00 | missing | style.css — keyframe names (locations only) |  |
| 3.1 | 4321 | 2963 | -1358 | 0.94 | early | constants.ts — every tunable threshold and timing | export names surface in src/constants.ts (t=2923, 15 atoms) |
| 3.2 | 4399 | 618 | -3781 | 0.86 | early | types.ts — DrawerDirection, SnapPoint, AnyFunction | export names surface in src/types.ts (t=597, 4 atoms) |
| 3.3 | 4502 | 4332 | -170 | 1.00 | aligned+over | helpers.ts — every exported function (signature heads only) | export names surface in src/helpers.ts (t=4332, 15 atoms) |
| 3.4 | 4688 | — | — | 0.00 | missing | Root inner-function locations (onPress, onDrag, onRelease, …) |  |
| 3.5 | 5171 | — | — | 0.06 | missing | context.ts — DrawerContextValue interface (parent/child contract) | imports in src/context.ts (t=3524, 2 atoms) |
| 3.6 | 5503 | 5282 | -221 | 0.82 | aligned | useSnapPoints — full parameter shape | export at src/use-snap-points.ts:7 (t=5282, 24 atoms) |
| 3.7 | 6032 | — | — | 0.00 | missing | useSnapPoints — return shape + snapPointsOffset memo |  |
| 3.8 | 6394 | 7047 | +653 | 0.85 | aligned | helpers.ts — dampenValue, getTranslate, isVertical bodies | export body at src/helpers.ts:72 (t=7047, 15 atoms) |
| 3.9 | 7179 | — | — | 0.00 | missing | Root onRelease — close-threshold + velocity decision |  |
| 3.10 | 7585 | — | — | 0.64 | partial | browser.ts — every UA-detection function (full file) | export names surface in src/browser.ts (t=2083, 14 atoms) |
| 3.11 | 8357 | — | — | 0.08 | missing | useScaleBackground — wrapper-scale effect (full hook) | imports in src/use-scale-background.ts (t=7688, 4 atoms) |
| 3.12 | 9050 | — | — | 0.15 | missing | use-prevent-scroll — exports + the six mobile-Safari quirks | export body at src/use-prevent-scroll.ts:68 (t=5452, 18 atoms) |
| 4.1 | 9102 | — | — | 0.00 | missing | test/ workspace listing |  |
| 4.2 | 9201 | — | — | 0.00 | missing | test/src/app/ — every demo page directory |  |
| 4.3 | 9267 | — | — | 0.00 | missing | test/tests/ — Playwright spec catalog |  |
| 4.4 | 9693 | — | — | 0.33 | missing | Landing page + Playwright device profiles | export at playwright.config.ts:12 (t=3475, 11 atoms) |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 1039 | 1.00 | 1039 | 9158 | export body at src/index.tsx:996 |
| 407 | 0.79 | 512 | 3475 | export at playwright.config.ts:12 |
| 327 | 1.00 | 327 | 7619 | export at src/context.ts:37 |
| 245 | 1.00 | 245 | 7292 | export body at src/use-controllable-state.ts:39 |
| 209 | 0.59 | 357 | 8119 | package dependencies in package.json |
| 189 | 0.82 | 230 | 314 | package identity in package.json |
| 174 | 1.00 | 174 | 3721 | package entrypoints in package.json |
| 170 | 1.00 | 170 | 5452 | export body at src/use-prevent-scroll.ts:68 |
| 156 | 0.51 | 305 | 4161 | export body at src/index.tsx:803 |
| 152 | 1.00 | 152 | 5033 | export body at src/helpers.ts:23 |
| 147 | 1.00 | 147 | 9305 | export doc at src/use-position-fixed.ts:15 |
| 135 | 1.00 | 135 | 3856 | package scripts in package.json |
| 134 | 1.00 | 134 | 4881 | export body at src/helpers.ts:42 |
| 124 | 1.00 | 124 | 2355 | export at src/use-position-fixed.ts:15 |
| 111 | 1.00 | 111 | 6733 | json config tsconfig.json |
| 111 | 1.00 | 111 | 9416 | json config turbo.json |
| 104 | 1.00 | 104 | 4634 | export body at src/helpers.ts:9 |
| 85 | 1.00 | 85 | 4501 | export body at src/helpers.ts:108 |
| 80 | 1.00 | 80 | 2435 | export body at src/use-prevent-scroll.ts:34 |
| 74 | 1.00 | 74 | 7762 | imports in src/use-snap-points.ts |
| 65 | 1.00 | 65 | 4416 | export body at src/helpers.ts:94 |
| 60 | 1.00 | 60 | 1898 | export body at src/use-prevent-scroll.ts:294 |
| 58 | 1.00 | 58 | 6818 | export doc at src/use-prevent-scroll.ts:68 |
| 57 | 1.00 | 57 | 1971 | export body at src/context.ts:69 |

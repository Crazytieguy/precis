scores: Sim=0.424 Reached=13/33 Early=2 Late=8 Partial=5 Missing=15 Used=8548/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 6 | 4 | 1 | 1 | 0.79 |
| 2 | 11 | 5 | 3 | 3 | 0.63 |
| 3 | 12 | 4 | 1 | 7 | 0.39 |
| 4 | 4 | 0 | 0 | 4 | 0.08 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.1 | 46 | — | — | 0.75 | partial | package.json — name, version, description | package identity in package.json (t=303, 3 atoms) |
| 1.2 | 108 | 62 | -46 | 1.00 | early | Top-level repo listing |  |
| 1.3 | 171 | — | — | 0.00 | missing | README — unmaintained notice |  |
| 1.4 | 242 | 828 | +586 | 1.00 | late | src/ listing — every source file |  |
| 1.5 | 355 | 1691 | +1336 | 1.00 | late | Drawer namespace export — the public component map | export at src/index.tsx:1137 (t=1691, 12 atoms) |
| 1.6 | 503 | 8548 | +8045 | 1.00 | late | package.json — runtime + peer deps (Radix dialog, React 16.8–19) | package dependencies in package.json (t=8548, 8 atoms) |
| 2.1 | 642 | 1244 | +602 | 1.00 | late | All top-level export locations in src/index.tsx | export at src/index.tsx:996 (t=7362, 89 atoms) |
| 2.2 | 730 | — | — | 0.64 | partial | Handle component — snap-point cycle + double-tap timing constants | export names surface in src/index.tsx (t=1244, 4 atoms) |
| 2.3 | 1020 | 1590 | +570 | 0.95 | late | WithFadeFromProps / WithoutFadeFromProps — snap-point fade contract | export at src/index.tsx:27 (t=1590, 12 atoms) |
| 2.4 | 1393 | 5972 | +4579 | 1.00 | late | DialogProps — every prop name (signatures only, no JSDoc) | export at src/index.tsx:50 (t=5972, 87 atoms) |
| 2.5 | 1711 | — | — | 0.03 | missing | Root signature — full destructuring with every default value | export names surface in src/index.tsx (t=1244, 2 atoms) |
| 2.6 | 2011 | — | — | 0.55 | partial | Overlay + Content render — what data-vaul-* attributes appear on DOM | export at src/index.tsx:803 (t=2276, 13 atoms) |
| 2.7 | 2405 | 5972 | +3567 | 1.00 | late | DialogProps — JSDoc for the high-traffic props | export at src/index.tsx:50 (t=5972, 50 atoms) |
| 2.8 | 2810 | 5972 | +3162 | 1.00 | late | DialogProps — JSDoc for the lower-traffic props | export at src/index.tsx:50 (t=5972, 73 atoms) |
| 2.9 | 3228 | — | — | 0.79 | partial | NestedRoot + Portal — nested-drawer wiring + container override | export body at src/index.tsx:1098 (t=2978, 25 atoms) |
| 2.10 | 4024 | — | — | 0.00 | missing | style.css — every selector head (locations only) |  |
| 2.11 | 4132 | — | — | 0.00 | missing | style.css — keyframe names (locations only) |  |
| 3.1 | 4321 | 3144 | -1177 | 0.94 | aligned | constants.ts — every tunable threshold and timing | export names surface in src/constants.ts (t=3104, 15 atoms) |
| 3.2 | 4399 | 1021 | -3378 | 0.86 | early | types.ts — DrawerDirection, SnapPoint, AnyFunction | export names surface in src/types.ts (t=1000, 4 atoms) |
| 3.3 | 4502 | 3857 | -645 | 1.00 | aligned+over | helpers.ts — every exported function (signature heads only) | export names surface in src/helpers.ts (t=3857, 15 atoms) |
| 3.4 | 4688 | — | — | 0.00 | missing | Root inner-function locations (onPress, onDrag, onRelease, …) |  |
| 3.5 | 5171 | — | — | 0.06 | missing | context.ts — DrawerContextValue interface (parent/child contract) | imports in src/context.ts (t=7630, 2 atoms) |
| 3.6 | 5503 | — | — | 0.04 | missing | useSnapPoints — full parameter shape | export names surface in src/use-snap-points.ts (t=852, 2 atoms) |
| 3.7 | 6032 | — | — | 0.00 | missing | useSnapPoints — return shape + snapPointsOffset memo |  |
| 3.8 | 6394 | 6288 | -106 | 0.88 | aligned | helpers.ts — dampenValue, getTranslate, isVertical bodies | export body at src/helpers.ts:72 (t=6288, 15 atoms) |
| 3.9 | 7179 | — | — | 0.00 | missing | Root onRelease — close-threshold + velocity decision |  |
| 3.10 | 7585 | — | — | 0.64 | partial | browser.ts — every UA-detection function (full file) | export names surface in src/browser.ts (t=2388, 14 atoms) |
| 3.11 | 8357 | — | — | 0.08 | missing | useScaleBackground — wrapper-scale effect (full hook) | imports in src/use-scale-background.ts (t=8117, 4 atoms) |
| 3.12 | 9050 | — | — | 0.15 | missing | use-prevent-scroll — exports + the six mobile-Safari quirks | export body at src/use-prevent-scroll.ts:68 (t=4802, 18 atoms) |
| 4.1 | 9102 | — | — | 0.00 | missing | test/ workspace listing |  |
| 4.2 | 9201 | — | — | 0.00 | missing | test/src/app/ — every demo page directory |  |
| 4.3 | 9267 | — | — | 0.00 | missing | test/tests/ — Playwright spec catalog |  |
| 4.4 | 9693 | — | — | 0.33 | missing | Landing page + Playwright device profiles | export at playwright.config.ts:12 (t=3656, 11 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 2 | 1182 | export at src/index.tsx:<n> |
| 2 | 389 | export at src/context.ts:<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 1025 | 0.96 | 1074 | 7362 | export at src/index.tsx:996 |
| 384 | 0.75 | 512 | 3656 | export at playwright.config.ts:12 |
| 327 | 1.00 | 327 | 8048 | export at src/context.ts:37 |
| 245 | 1.00 | 245 | 7607 | export body at src/use-controllable-state.ts:39 |
| 227 | 0.64 | 357 | 8548 | package dependencies in package.json |
| 193 | 0.84 | 230 | 303 | package identity in package.json |
| 174 | 1.00 | 174 | 477 | package entrypoints in package.json |
| 170 | 1.00 | 170 | 4802 | export body at src/use-prevent-scroll.ts:68 |
| 157 | 0.48 | 329 | 2276 | export at src/index.tsx:803 |
| 152 | 1.00 | 152 | 4632 | export body at src/helpers.ts:23 |
| 135 | 1.00 | 135 | 619 | package scripts in package.json |
| 134 | 1.00 | 134 | 4369 | export body at src/helpers.ts:42 |
| 111 | 1.00 | 111 | 730 | json config tsconfig.json |
| 111 | 1.00 | 111 | 4480 | json config turbo.json |
| 104 | 1.00 | 104 | 4218 | export body at src/helpers.ts:9 |
| 85 | 1.00 | 85 | 4114 | export body at src/helpers.ts:108 |
| 80 | 1.00 | 80 | 2616 | export body at src/use-prevent-scroll.ts:34 |
| 74 | 1.00 | 74 | 8191 | imports in src/use-snap-points.ts |
| 65 | 1.00 | 65 | 3941 | export body at src/helpers.ts:94 |
| 62 | 1.00 | 62 | 1753 | export at src/context.ts:69 |
| 60 | 1.00 | 60 | 1947 | export body at src/use-prevent-scroll.ts:294 |
| 58 | 1.00 | 58 | 6059 | export doc at src/use-prevent-scroll.ts:68 |
| 53 | 0.31 | 171 | 3857 | export names surface in src/helpers.ts |

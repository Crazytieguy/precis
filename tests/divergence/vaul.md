Score(3000)=0.591 I=0.683 C=0.512 ns_rows≤3K=14/33 (reached=7 partial=1 missing=6)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 46 |  | 46 | package.json — name, version, description | 1.1 |  | 0.000 |
| walker |  | 62 | 62 | listing of '.' |  |  | 0.000 |
| walker |  | 66 | 4 | listing of '.vscode' |  |  | 0.000 |
| ns | 108 |  | 62 | Top-level repo listing | 1.2 |  | 0.580 |
| walker |  | 118 | 52 | package identity in package.json |  |  | 0.825 |
| walker |  | 129 | 11 | export names surface in playwright.config.ts |  |  | 0.825 |
| ns | 171 |  | 63 | README — unmaintained notice | 1.3 |  | 0.777 |
| walker |  | 200 | 71 | listing of 'src' |  |  | 0.843 |
| walker |  | 213 | 13 | module item at src/index.tsx:993 |  |  | 0.843 |
| walker |  | 226 | 13 | module item at src/index.tsx:994 |  |  | 0.844 |
| ns | 242 |  | 71 | src/ listing — every source file | 1.4 |  | 0.819 |
| walker |  | 249 | 23 | plaintext config pnpm-workspace.yaml |  |  | 0.819 |
| ns | 355 |  | 113 | Drawer namespace export — the public component map | 1.5 |  | 0.683 |
| walker |  | 472 | 223 | export names surface in src/index.tsx |  |  | 0.709 |
| walker |  | 472 | 0 | export at src/index.tsx:1098 |  |  | 0.709 |
| walker |  | 472 | 0 | export at src/index.tsx:1130 |  |  | 0.709 |
| walker |  | 485 | 13 | export at src/index.tsx:989 |  |  | 0.712 |
| walker |  | 498 | 13 | export at src/index.tsx:803 |  |  | 0.712 |
| ns | 503 |  | 148 | package.json — runtime + peer deps (Radix dialog, React 16.8–19) | 1.6 |  | 0.647 |
| walker |  | 527 | 29 | export at src/index.tsx:996 |  |  | 0.653 |
| walker |  | 561 | 34 | export at src/index.tsx:833 |  |  | 0.653 |
| ns | 642 |  | 139 | All top-level export locations in src/index.tsx | 2.1 |  | 0.677 |
| walker |  | 681 | 120 | export at src/index.tsx:40 |  |  | 0.682 |
| ns | 730 |  | 88 | Handle component — snap-point cycle + double-tap timing constants | 2.2 | 2.1 | 0.683 |
| walker |  | 842 | 161 | export at src/index.tsx:27 |  |  | 0.702 |
| walker |  | 943 | 101 | export at src/index.tsx:1137 |  |  | 0.808 |
| ns | 1020 |  | 290 | WithFadeFromProps / WithoutFadeFromProps — snap-point fade contract | 2.3 | 2.1 | 0.801 |
| walker |  | 1261 | 318 | export at src/index.tsx:139 |  |  | 0.823 |
| walker |  | 1273 | 12 | export names surface in src/use-position-fixed.ts |  |  | 0.823 |
| walker |  | 1285 | 12 | export names surface in src/use-snap-points.ts |  |  | 0.823 |
| walker |  | 1298 | 13 | export names surface in src/use-scale-background.ts |  |  | 0.823 |
| walker |  | 1298 | 0 | export at src/use-scale-background.ts:8 |  |  | 0.823 |
| walker |  | 1312 | 14 | export names surface in src/use-composed-refs.ts |  |  | 0.823 |
| ns | 1393 |  | 373 | DialogProps — every prop name (signatures only, no JSDoc) | 2.4 | 2.1 | 0.710 |
| walker |  | 1490 | 178 | package identity metadata in package.json |  |  | 0.710 |
| walker |  | 1664 | 174 | package entrypoints in package.json |  |  | 0.710 |
| walker |  | 1683 | 19 | package runtime metadata in package.json |  |  | 0.711 |
| ns | 1711 |  | 318 | Root signature — full destructuring with every default value | 2.5 | 2.4 | 0.730 |
| walker |  | 1818 | 135 | package scripts in package.json |  |  | 0.730 |
| ns | 2011 |  | 300 | Overlay + Content render — what data-vaul-* attributes appear on DOM | 2.6 |  | 0.681 |
| walker |  | 2123 | 305 | export body at src/index.tsx:803 body 805 |  |  | 0.699 |
| ns | 2405 |  | 394 | DialogProps — JSDoc for the high-traffic props | 2.7 | 2.4 | 0.638 |
| ns | 2810 |  | 405 | DialogProps — JSDoc for the lower-traffic props | 2.8 | 2.4 | 0.591 |
| ns | 3228 |  | 418 | NestedRoot + Portal — nested-drawer wiring + container override | 2.9 | 2.1 | 0.543 |
| walker |  | 3293 | 1170 | export at src/index.tsx:50 |  |  | 0.756 |
| walker |  | 3404 | 111 | json config tsconfig.json |  |  | 0.756 |
| walker |  | 3431 | 27 | json config .vscode/settings.json |  |  | 0.756 |
| walker |  | 3465 | 34 | export names surface in src/context.ts |  |  | 0.756 |
| walker |  | 3465 | 0 | export at src/context.ts:69 |  |  | 0.756 |
| walker |  | 3500 | 35 | export names surface in src/use-controllable-state.ts |  |  | 0.756 |
| walker |  | 3500 | 0 | export at src/use-controllable-state.ts:39 |  |  | 0.756 |
| walker |  | 3517 | 17 | imports in playwright.config.ts |  |  | 0.756 |
| walker |  | 3569 | 52 | listing of 'test' |  |  | 0.757 |
| walker |  | 3572 | 3 | listing of 'test/src' |  |  | 0.757 |
| walker |  | 3582 | 10 | imports in src/use-controllable-state.ts |  |  | 0.757 |
| walker |  | 3920 | 338 | package dependencies in package.json |  |  | 0.783 |
| walker |  | 3923 | 3 | listing of '.github' |  |  | 0.783 |
| walker |  | 3927 | 4 | listing of '.github/workflows' |  |  | 0.783 |
| walker |  | 3939 | 12 | imports in src/use-composed-refs.ts |  |  | 0.783 |
| walker |  | 3991 | 52 | export names surface in src/types.ts |  |  | 0.784 |
| walker |  | 4012 | 21 | export at src/types.ts:2 |  |  | 0.785 |
| ns | 4024 |  | 796 | style.css — every selector head (locations only) | 2.10 |  | 0.741 |
| ns | 4132 |  | 108 | style.css — keyframe names (locations only) | 2.11 |  | 0.727 |
| ns | 4321 |  | 189 | constants.ts — every tunable threshold and timing | 3.1 |  | 0.704 |
| walker |  | 4323 | 311 | imports in src/index.tsx |  |  | 0.704 |
| ns | 4399 |  | 78 | types.ts — DrawerDirection, SnapPoint, AnyFunction | 3.2 |  | 0.705 |
| ns | 4502 |  | 103 | helpers.ts — every exported function (signature heads only) | 3.3 |  | 0.695 |
| ns | 4688 |  | 186 | Root inner-function locations (onPress, onDrag, onRelease, …) | 3.4 |  | 0.683 |
| ns | 5171 |  | 483 | context.ts — DrawerContextValue interface (parent/child contract) | 3.5 |  | 0.647 |
| walker |  | 5362 | 1039 | export body at src/index.tsx:996 body 1000 |  |  | 0.647 |
| walker |  | 5378 | 16 | imports in src/helpers.ts |  |  | 0.647 |
| walker |  | 5435 | 57 | export body at src/context.ts:69 body 70 |  |  | 0.647 |
| ns | 5503 |  | 332 | useSnapPoints — full parameter shape | 3.6 |  | 0.622 |
| walker |  | 5546 | 111 | json config turbo.json |  |  | 0.622 |
| walker |  | 5670 | 124 | export at src/use-position-fixed.ts:15 |  |  | 0.622 |
| ns | 6032 |  | 529 | useSnapPoints — return shape + snapPointsOffset memo | 3.7 |  | 0.585 |
| walker |  | 6182 | 512 | export at playwright.config.ts:12 |  |  | 0.586 |
| walker |  | 6208 | 26 | export doc at playwright.config.ts:12 |  |  | 0.586 |
| ns | 6394 |  | 362 | helpers.ts — dampenValue, getTranslate, isVertical bodies | 3.8 | 3.3 | 0.562 |
| ns | 7179 |  | 785 | Root onRelease — close-threshold + velocity decision | 3.9 | 3.4 | 0.525 |
| ns | 7585 |  | 406 | browser.ts — every UA-detection function (full file) | 3.10 |  | 0.507 |
| walker |  | 7899 | 1691 | export body at src/index.tsx:833 body 837 |  |  | 0.528 |
| walker |  | 7965 | 66 | listing of 'test/tests' |  |  | 0.529 |
| walker |  | 7988 | 23 | imports in src/context.ts |  |  | 0.529 |
| walker |  | 8011 | 23 | imports in src/use-position-fixed.ts |  |  | 0.529 |
| walker |  | 8106 | 95 | export names surface in src/use-prevent-scroll.ts |  |  | 0.529 |
| walker |  | 8106 | 0 | export at src/use-prevent-scroll.ts:29 |  |  | 0.529 |
| walker |  | 8106 | 0 | export at src/use-prevent-scroll.ts:34 |  |  | 0.529 |
| walker |  | 8106 | 0 | export at src/use-prevent-scroll.ts:68 |  |  | 0.529 |
| walker |  | 8106 | 0 | export at src/use-prevent-scroll.ts:294 |  |  | 0.529 |
| walker |  | 8145 | 39 | export body at src/use-prevent-scroll.ts:29 body 30 |  |  | 0.529 |
| walker |  | 8205 | 60 | export body at src/use-prevent-scroll.ts:294 body 295 |  |  | 0.529 |
| walker |  | 8285 | 80 | export body at src/use-prevent-scroll.ts:34 body 35 |  |  | 0.529 |
| ns | 8357 |  | 772 | useScaleBackground — wrapper-scale effect (full hook) | 3.11 |  | 0.501 |
| walker |  | 8397 | 112 | export names surface in src/browser.ts |  |  | 0.504 |
| walker |  | 8397 | 0 | export at src/browser.ts:1 |  |  | 0.504 |
| walker |  | 8397 | 0 | export at src/browser.ts:10 |  |  | 0.504 |
| walker |  | 8397 | 0 | export at src/browser.ts:14 |  |  | 0.504 |
| walker |  | 8397 | 0 | export at src/browser.ts:18 |  |  | 0.504 |
| walker |  | 8397 | 0 | export at src/browser.ts:22 |  |  | 0.504 |
| walker |  | 8397 | 0 | export at src/browser.ts:30 |  |  | 0.504 |
| walker |  | 8397 | 0 | export at src/browser.ts:34 |  |  | 0.504 |
| walker |  | 8407 | 10 | export body at src/browser.ts:10 body 11 |  |  | 0.505 |
| walker |  | 8418 | 11 | export body at src/browser.ts:14 body 15 |  |  | 0.505 |
| walker |  | 8439 | 21 | export body at src/browser.ts:18 body 19 |  |  | 0.507 |
| walker |  | 8520 | 81 | export body at src/browser.ts:1 body 2 |  |  | 0.513 |
| walker |  | 8550 | 30 | imports in src/use-prevent-scroll.ts |  |  | 0.513 |
| walker |  | 8676 | 126 | export names surface in src/constants.ts |  |  | 0.528 |
| walker |  | 8716 | 40 | export at src/constants.ts:1 |  |  | 0.534 |
| walker |  | 8815 | 99 | listing of 'test/src/app' |  |  | 0.535 |
| ns | 9050 |  | 693 | use-prevent-scroll — exports + the six mobile-Safari quirks | 3.12 |  | 0.521 |
| walker |  | 9064 | 249 | export at src/use-snap-points.ts:7 |  |  | 0.540 |
| ns | 9102 |  | 52 | test/ workspace listing | 4.1 |  | 0.546 |
| ns | 9201 |  | 99 | test/src/app/ — every demo page directory | 4.2 |  | 0.554 |
| walker |  | 9234 | 170 | export body at src/use-prevent-scroll.ts:68 body 69 |  |  | 0.554 |
| ns | 9267 |  | 66 | test/tests/ — Playwright spec catalog | 4.3 |  | 0.559 |
| walker |  | 9405 | 171 | export names surface in src/helpers.ts |  |  | 0.568 |
| walker |  | 9405 | 0 | export at src/helpers.ts:9 |  |  | 0.568 |
| walker |  | 9405 | 0 | export at src/helpers.ts:23 |  |  | 0.568 |
| walker |  | 9405 | 0 | export at src/helpers.ts:42 |  |  | 0.568 |
| walker |  | 9405 | 0 | export at src/helpers.ts:59 |  |  | 0.568 |
| walker |  | 9405 | 0 | export at src/helpers.ts:72 |  |  | 0.568 |
| walker |  | 9405 | 0 | export at src/helpers.ts:90 |  |  | 0.568 |
| walker |  | 9405 | 0 | export at src/helpers.ts:94 |  |  | 0.568 |
| walker |  | 9405 | 0 | export at src/helpers.ts:108 |  |  | 0.568 |
| walker |  | 9434 | 29 | export doc at src/helpers.ts:108 |  |  | 0.568 |
| walker |  | 9538 | 104 | export body at src/helpers.ts:9 body 10 |  |  | 0.568 |
| walker |  | 9621 | 83 | export body at src/helpers.ts:59 body 60 |  |  | 0.572 |
| ns | 9693 |  | 426 | Landing page + Playwright device profiles | 4.4 |  | 0.560 |
| walker |  | 9755 | 134 | export body at src/helpers.ts:42 body 43 |  |  | 0.560 |
| walker |  | 9907 | 152 | export body at src/helpers.ts:23 body 24 |  |  | 0.560 |
| walker |  | 9965 | 58 | export doc at src/use-prevent-scroll.ts:68 |  |  | 0.560 |

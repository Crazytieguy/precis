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
| walker |  | 238 | 12 | export names surface in src/use-position-fixed.ts |  |  | 0.844 |
| ns | 242 |  | 71 | src/ listing — every source file | 1.4 |  | 0.819 |
| walker |  | 250 | 12 | export names surface in src/use-snap-points.ts |  |  | 0.819 |
| walker |  | 263 | 13 | export names surface in src/use-scale-background.ts |  |  | 0.819 |
| walker |  | 263 | 0 | export at src/use-scale-background.ts:8 |  |  | 0.819 |
| walker |  | 277 | 14 | export names surface in src/use-composed-refs.ts |  |  | 0.819 |
| walker |  | 300 | 23 | plaintext config pnpm-workspace.yaml |  |  | 0.819 |
| ns | 355 |  | 113 | Drawer namespace export — the public component map | 1.5 |  | 0.683 |
| ns | 503 |  | 148 | package.json — runtime + peer deps (Radix dialog, React 16.8–19) | 1.6 |  | 0.621 |
| walker |  | 523 | 223 | export names surface in src/index.tsx |  |  | 0.645 |
| walker |  | 523 | 0 | export at src/index.tsx:1098 |  |  | 0.645 |
| walker |  | 523 | 0 | export at src/index.tsx:1130 |  |  | 0.645 |
| walker |  | 536 | 13 | export at src/index.tsx:989 |  |  | 0.647 |
| walker |  | 549 | 13 | export at src/index.tsx:803 |  |  | 0.647 |
| walker |  | 578 | 29 | export at src/index.tsx:996 |  |  | 0.653 |
| walker |  | 612 | 34 | export at src/index.tsx:833 |  |  | 0.653 |
| ns | 642 |  | 139 | All top-level export locations in src/index.tsx | 2.1 |  | 0.677 |
| ns | 730 |  | 88 | Handle component — snap-point cycle + double-tap timing constants | 2.2 | 2.1 | 0.678 |
| walker |  | 732 | 120 | export at src/index.tsx:40 |  |  | 0.683 |
| walker |  | 893 | 161 | export at src/index.tsx:27 |  |  | 0.702 |
| walker |  | 994 | 101 | export at src/index.tsx:1137 |  |  | 0.808 |
| ns | 1020 |  | 290 | WithFadeFromProps / WithoutFadeFromProps — snap-point fade contract | 2.3 | 2.1 | 0.801 |
| walker |  | 1312 | 318 | export at src/index.tsx:139 |  |  | 0.823 |
| ns | 1393 |  | 373 | DialogProps — every prop name (signatures only, no JSDoc) | 2.4 | 2.1 | 0.710 |
| walker |  | 1490 | 178 | package identity metadata in package.json |  |  | 0.710 |
| walker |  | 1524 | 34 | export names surface in src/context.ts |  |  | 0.710 |
| walker |  | 1524 | 0 | export at src/context.ts:69 |  |  | 0.710 |
| walker |  | 1698 | 174 | package entrypoints in package.json |  |  | 0.710 |
| ns | 1711 |  | 318 | Root signature — full destructuring with every default value | 2.5 | 2.4 | 0.730 |
| walker |  | 1717 | 19 | package runtime metadata in package.json |  |  | 0.730 |
| walker |  | 1852 | 135 | package scripts in package.json |  |  | 0.730 |
| walker |  | 1887 | 35 | export names surface in src/use-controllable-state.ts |  |  | 0.730 |
| walker |  | 1887 | 0 | export at src/use-controllable-state.ts:39 |  |  | 0.730 |
| walker |  | 1904 | 17 | imports in playwright.config.ts |  |  | 0.730 |
| ns | 2011 |  | 300 | Overlay + Content render — what data-vaul-* attributes appear on DOM | 2.6 |  | 0.681 |
| walker |  | 2209 | 305 | export body at src/index.tsx:803 body 805 |  |  | 0.699 |
| walker |  | 2219 | 10 | imports in src/use-controllable-state.ts |  |  | 0.699 |
| walker |  | 2231 | 12 | imports in src/use-composed-refs.ts |  |  | 0.699 |
| ns | 2405 |  | 394 | DialogProps — JSDoc for the high-traffic props | 2.7 | 2.4 | 0.638 |
| ns | 2810 |  | 405 | DialogProps — JSDoc for the lower-traffic props | 2.8 | 2.4 | 0.591 |
| ns | 3228 |  | 418 | NestedRoot + Portal — nested-drawer wiring + container override | 2.9 | 2.1 | 0.543 |
| walker |  | 3401 | 1170 | export at src/index.tsx:50 |  |  | 0.756 |
| walker |  | 3453 | 52 | export names surface in src/types.ts |  |  | 0.756 |
| walker |  | 3474 | 21 | export at src/types.ts:2 |  |  | 0.757 |
| walker |  | 3585 | 111 | json config tsconfig.json |  |  | 0.757 |
| walker |  | 3612 | 27 | json config .vscode/settings.json |  |  | 0.757 |
| walker |  | 3628 | 16 | imports in src/helpers.ts |  |  | 0.757 |
| walker |  | 3685 | 57 | export body at src/context.ts:69 body 70 |  |  | 0.757 |
| walker |  | 3737 | 52 | listing of 'test' |  |  | 0.758 |
| walker |  | 3740 | 3 | listing of 'test/src' |  |  | 0.758 |
| walker |  | 3864 | 124 | export at src/use-position-fixed.ts:15 |  |  | 0.758 |
| ns | 4024 |  | 796 | style.css — every selector head (locations only) | 2.10 |  | 0.715 |
| ns | 4132 |  | 108 | style.css — keyframe names (locations only) | 2.11 |  | 0.702 |
| ns | 4321 |  | 189 | constants.ts — every tunable threshold and timing | 3.1 |  | 0.680 |
| walker |  | 4376 | 512 | export at playwright.config.ts:12 |  |  | 0.680 |
| ns | 4399 |  | 78 | types.ts — DrawerDirection, SnapPoint, AnyFunction | 3.2 |  | 0.681 |
| walker |  | 4402 | 26 | export doc at playwright.config.ts:12 |  |  | 0.681 |
| ns | 4502 |  | 103 | helpers.ts — every exported function (signature heads only) | 3.3 |  | 0.672 |
| ns | 4688 |  | 186 | Root inner-function locations (onPress, onDrag, onRelease, …) | 3.4 |  | 0.660 |
| walker |  | 4740 | 338 | package dependencies in package.json |  |  | 0.683 |
| walker |  | 4743 | 3 | listing of '.github' |  |  | 0.683 |
| walker |  | 4747 | 4 | listing of '.github/workflows' |  |  | 0.683 |
| walker |  | 4770 | 23 | imports in src/context.ts |  |  | 0.683 |
| walker |  | 4793 | 23 | imports in src/use-position-fixed.ts |  |  | 0.683 |
| walker |  | 4888 | 95 | export names surface in src/use-prevent-scroll.ts |  |  | 0.683 |
| walker |  | 4888 | 0 | export at src/use-prevent-scroll.ts:29 |  |  | 0.683 |
| walker |  | 4888 | 0 | export at src/use-prevent-scroll.ts:34 |  |  | 0.683 |
| walker |  | 4888 | 0 | export at src/use-prevent-scroll.ts:68 |  |  | 0.683 |
| walker |  | 4888 | 0 | export at src/use-prevent-scroll.ts:294 |  |  | 0.683 |
| walker |  | 4927 | 39 | export body at src/use-prevent-scroll.ts:29 body 30 |  |  | 0.683 |
| walker |  | 4987 | 60 | export body at src/use-prevent-scroll.ts:294 body 295 |  |  | 0.683 |
| walker |  | 5067 | 80 | export body at src/use-prevent-scroll.ts:34 body 35 |  |  | 0.683 |
| ns | 5171 |  | 483 | context.ts — DrawerContextValue interface (parent/child contract) | 3.5 |  | 0.648 |
| walker |  | 5378 | 311 | imports in src/index.tsx |  |  | 0.648 |
| walker |  | 5490 | 112 | export names surface in src/browser.ts |  |  | 0.648 |
| walker |  | 5490 | 0 | export at src/browser.ts:1 |  |  | 0.648 |
| walker |  | 5490 | 0 | export at src/browser.ts:10 |  |  | 0.648 |
| walker |  | 5490 | 0 | export at src/browser.ts:14 |  |  | 0.648 |
| walker |  | 5490 | 0 | export at src/browser.ts:18 |  |  | 0.648 |
| walker |  | 5490 | 0 | export at src/browser.ts:22 |  |  | 0.648 |
| walker |  | 5490 | 0 | export at src/browser.ts:30 |  |  | 0.648 |
| walker |  | 5490 | 0 | export at src/browser.ts:34 |  |  | 0.648 |
| walker |  | 5500 | 10 | export body at src/browser.ts:10 body 11 |  |  | 0.648 |
| ns | 5503 |  | 332 | useSnapPoints — full parameter shape | 3.6 |  | 0.623 |
| walker |  | 5511 | 11 | export body at src/browser.ts:14 body 15 |  |  | 0.623 |
| walker |  | 5532 | 21 | export body at src/browser.ts:18 body 19 |  |  | 0.623 |
| walker |  | 5613 | 81 | export body at src/browser.ts:1 body 2 |  |  | 0.624 |
| ns | 6032 |  | 529 | useSnapPoints — return shape + snapPointsOffset memo | 3.7 |  | 0.587 |
| ns | 6394 |  | 362 | helpers.ts — dampenValue, getTranslate, isVertical bodies | 3.8 | 3.3 | 0.564 |
| walker |  | 6652 | 1039 | export body at src/index.tsx:996 body 1000 |  |  | 0.564 |
| walker |  | 6682 | 30 | imports in src/use-prevent-scroll.ts |  |  | 0.564 |
| walker |  | 6808 | 126 | export names surface in src/constants.ts |  |  | 0.581 |
| walker |  | 6848 | 40 | export at src/constants.ts:1 |  |  | 0.588 |
| walker |  | 7097 | 249 | export at src/use-snap-points.ts:7 |  |  | 0.612 |
| ns | 7179 |  | 785 | Root onRelease — close-threshold + velocity decision | 3.9 | 3.4 | 0.571 |
| walker |  | 7208 | 111 | json config turbo.json |  |  | 0.571 |
| walker |  | 7378 | 170 | export body at src/use-prevent-scroll.ts:68 body 69 |  |  | 0.571 |
| walker |  | 7549 | 171 | export names surface in src/helpers.ts |  |  | 0.582 |
| walker |  | 7549 | 0 | export at src/helpers.ts:9 |  |  | 0.582 |
| walker |  | 7549 | 0 | export at src/helpers.ts:23 |  |  | 0.582 |
| walker |  | 7549 | 0 | export at src/helpers.ts:42 |  |  | 0.582 |
| walker |  | 7549 | 0 | export at src/helpers.ts:59 |  |  | 0.582 |
| walker |  | 7549 | 0 | export at src/helpers.ts:72 |  |  | 0.582 |
| walker |  | 7549 | 0 | export at src/helpers.ts:90 |  |  | 0.582 |
| walker |  | 7549 | 0 | export at src/helpers.ts:94 |  |  | 0.582 |
| walker |  | 7549 | 0 | export at src/helpers.ts:108 |  |  | 0.582 |
| walker |  | 7578 | 29 | export doc at src/helpers.ts:108 |  |  | 0.582 |
| ns | 7585 |  | 406 | browser.ts — every UA-detection function (full file) | 3.10 |  | 0.573 |
| walker |  | 7682 | 104 | export body at src/helpers.ts:9 body 10 |  |  | 0.573 |
| walker |  | 7765 | 83 | export body at src/helpers.ts:59 body 60 |  |  | 0.578 |
| walker |  | 7899 | 134 | export body at src/helpers.ts:42 body 43 |  |  | 0.578 |
| walker |  | 8051 | 152 | export body at src/helpers.ts:23 body 24 |  |  | 0.578 |
| ns | 8357 |  | 772 | useScaleBackground — wrapper-scale effect (full hook) | 3.11 |  | 0.548 |
| ns | 9050 |  | 693 | use-prevent-scroll — exports + the six mobile-Safari quirks | 3.12 |  | 0.533 |
| ns | 9102 |  | 52 | test/ workspace listing | 4.1 |  | 0.539 |
| ns | 9201 |  | 99 | test/src/app/ — every demo page directory | 4.2 |  | 0.531 |
| ns | 9267 |  | 66 | test/tests/ — Playwright spec catalog | 4.3 |  | 0.526 |
| ns | 9693 |  | 426 | Landing page + Playwright device profiles | 4.4 |  | 0.515 |
| walker |  | 9742 | 1691 | export body at src/index.tsx:833 body 837 |  |  | 0.532 |
| walker |  | 9800 | 58 | export doc at src/use-prevent-scroll.ts:68 |  |  | 0.532 |
| walker |  | 9866 | 66 | listing of 'test/tests' |  |  | 0.542 |

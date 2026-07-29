Score(3000)=0.875 I=0.930 C=0.824 ns_rows≤3K=20/61 (reached=15 partial=2 missing=3)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 66 | 66 | listing of '.' |  |  | 0.000 |
| walker |  | 69 | 3 | listing of '.github' |  |  | 0.000 |
| walker |  | 72 | 3 | listing of '.github/workflows' |  |  | 0.000 |
| walker |  | 75 | 3 | listing of '.vscode' |  |  | 0.000 |
| ns | 86 |  | 86 | Package identity: name, version, description, entry points | 1.1 |  | 0.000 |
| walker |  | 133 | 58 | package identity in package.json |  |  | 0.460 |
| ns | 152 |  | 66 | Complete repository root listing | 1.2 |  | 0.677 |
| walker |  | 203 | 70 | listing of 'src' |  |  | 0.746 |
| ns | 222 |  | 70 | Complete src/ listing — the shipped library | 1.3 |  | 0.737 |
| walker |  | 239 | 36 | package runtime dependencies in package.json |  |  | 0.737 |
| walker |  | 258 | 19 | package runtime metadata in package.json |  |  | 0.737 |
| walker |  | 275 | 17 | module item at src/index.tsx:993 |  |  | 0.737 |
| walker |  | 288 | 13 | module item at src/index.tsx:994 |  |  | 0.737 |
| ns | 337 |  | 115 | The `Drawer` namespace object — the entire public component set | 1.4 |  | 0.615 |
| walker |  | 351 | 63 | README.md section #0 |  |  | 0.620 |
| walker |  | 364 | 13 | export names surface in playwright.config.ts |  |  | 0.620 |
| ns | 405 |  | 68 | README: the unmaintained notice (whole file) | 1.5 |  | 0.616 |
| walker |  | 418 | 54 | listing of 'test' |  |  | 0.618 |
| walker |  | 421 | 3 | listing of 'test/src' |  |  | 0.618 |
| walker |  | 444 | 23 | plaintext config pnpm-workspace.yaml |  |  | 0.618 |
| ns | 591 |  | 186 | Every top-level export declaration in src/index.tsx (names only) | 1.6 |  | 0.542 |
| walker |  | 668 | 224 | export names surface in src/index.tsx |  |  | 0.637 |
| walker |  | 668 | 0 | export at src/index.tsx:1098 |  |  | 0.637 |
| walker |  | 668 | 0 | export at src/index.tsx:1130 |  |  | 0.637 |
| walker |  | 681 | 13 | export at src/index.tsx:989 |  |  | 0.637 |
| walker |  | 694 | 13 | export at src/index.tsx:803 |  |  | 0.637 |
| walker |  | 723 | 29 | export at src/index.tsx:996 |  |  | 0.637 |
| ns | 738 |  | 147 | src/index.tsx imports: Radix dialog, style.css, and the first hook modules | 1.7 |  | 0.582 |
| walker |  | 757 | 34 | export at src/index.tsx:833 |  |  | 0.582 |
| walker |  | 877 | 120 | export at src/index.tsx:40 |  |  | 0.585 |
| ns | 905 |  | 167 | src/index.tsx imports: the constants set and the remaining hook modules | 1.8 | 1.7 | 0.522 |
| walker |  | 1038 | 161 | export at src/index.tsx:27 |  |  | 0.532 |
| ns | 1042 |  | 137 | package.json scripts — build, dev, test, format | 1.9 |  | 0.502 |
| ns | 1098 |  | 56 | Snap-point prop union: the fields of WithFadeFromProps / WithoutFadeFromProps | 2.1 | 1.6 | 0.520 |
| walker |  | 1139 | 101 | export at src/index.tsx:1137 |  |  | 0.607 |
| ns | 1260 |  | 162 | DialogProps declarations, first half (lines 51-85), doc comments elided | 2.2 | 1.6 | 0.551 |
| walker |  | 1457 | 318 | export at src/index.tsx:139 |  |  | 0.567 |
| walker |  | 1471 | 14 | export names surface in src/use-position-fixed.ts |  |  | 0.567 |
| walker |  | 1485 | 14 | export names surface in src/use-snap-points.ts |  |  | 0.567 |
| ns | 1497 |  | 237 | DialogProps declarations, second half (lines 86-137) plus the union tail | 2.3 | 2.2 | 0.510 |
| walker |  | 1622 | 137 | package scripts in package.json |  |  | 0.554 |
| walker |  | 1637 | 15 | export names surface in src/use-scale-background.ts |  |  | 0.554 |
| walker |  | 1637 | 0 | export at src/use-scale-background.ts:8 |  |  | 0.554 |
| walker |  | 1653 | 16 | export names surface in src/use-composed-refs.ts |  |  | 0.554 |
| ns | 1815 |  | 318 | Root's destructured parameter list — every prop's default value | 2.4 | 1.6 | 0.593 |
| walker |  | 1825 | 172 | package entrypoints in package.json |  |  | 0.729 |
| ns | 2034 |  | 219 | Snap-point prop documentation (fills 2.1's ellipses) | 2.5 | 2.1 | 0.741 |
| ns | 2201 |  | 167 | Docs for dismissible, modal and direction (fills 2.3's ellipses) | 2.6 | 2.3 | 0.713 |
| ns | 2401 |  | 200 | Docs for closeThreshold, noBodyStyles, setBackgroundColorOnScale, scrollLockTimeout | 2.7 | 2.2 | 0.681 |
| ns | 2493 |  | 92 | Docs for fixed and handleOnly | 2.8 | 2.2 | 0.669 |
| ns | 2664 |  | 171 | Docs for defaultOpen, disablePreventScroll and repositionInputs | 2.9 | 2.3 | 0.647 |
| ns | 2805 |  | 141 | Docs for snapToSequentialPoint and onAnimationEnd — last of the prop docs | 2.10 | 2.3 | 0.632 |
| ns | 2994 |  | 189 | src/constants.ts in full — every tuning constant and its value | 3.1 |  | 0.607 |
| walker |  | 2995 | 1170 | export at src/index.tsx:50 |  |  | 0.875 |
| walker |  | 3060 | 65 | listing of 'test/tests' |  |  | 0.877 |
| ns | 3072 |  | 78 | src/types.ts in full — DrawerDirection, SnapPoint, AnyFunction | 3.2 |  | 0.863 |
| walker |  | 3174 | 114 | listing of 'test/src/app' |  |  | 0.866 |
| ns | 3245 |  | 173 | src/helpers.ts — all eight exported helper signatures | 3.3 |  | 0.851 |
| walker |  | 3350 | 176 | package identity metadata in package.json |  |  | 0.851 |
| ns | 3357 |  | 112 | src/browser.ts — the complete platform-detection roster | 3.4 |  | 0.839 |
| walker |  | 3461 | 111 | json config tsconfig.json |  |  | 0.840 |
| walker |  | 3497 | 36 | export names surface in src/context.ts |  |  | 0.840 |
| walker |  | 3497 | 0 | export at src/context.ts:69 |  |  | 0.840 |
| walker |  | 3514 | 17 | imports in playwright.config.ts |  |  | 0.840 |
| walker |  | 3551 | 37 | export names surface in src/use-controllable-state.ts |  |  | 0.840 |
| walker |  | 3551 | 0 | export at src/use-controllable-state.ts:39 |  |  | 0.840 |
| walker |  | 3559 | 8 | listing of 'test/public' |  |  | 0.840 |
| ns | 3636 |  | 279 | DrawerContextValue, first half — refs and pointer callbacks | 3.5 |  | 0.813 |
| ns | 3816 |  | 180 | DrawerContextValue, second half — snap points, direction, container | 3.6 | 3.5 | 0.790 |
| walker |  | 3873 | 314 | imports in src/index.tsx |  |  | 0.858 |
| walker |  | 3885 | 12 | imports in src/use-controllable-state.ts |  |  | 0.858 |
| walker |  | 3899 | 14 | imports in src/use-composed-refs.ts |  |  | 0.858 |
| ns | 3915 |  | 99 | DrawerContext creation and the useDrawerContext accessor | 3.7 | 3.6 | 0.845 |
| walker |  | 4023 | 124 | export at src/use-position-fixed.ts:15 |  |  | 0.845 |
| ns | 4025 |  | 110 | useSnapPoints: entry point and its complete return surface | 3.8 |  | 0.827 |
| walker |  | 4082 | 59 | export body at src/context.ts:69 body 70 |  |  | 0.840 |
| ns | 4274 |  | 249 | useSnapPoints parameter object (fills 3.8's ellipsis) | 3.9 | 3.8 | 0.810 |
| ns | 4568 |  | 294 | usePositionFixed: the iOS rationale comment, its signature and return | 3.10 |  | 0.787 |
| walker |  | 4594 | 512 | export at playwright.config.ts:12 |  |  | 0.789 |
| walker |  | 4620 | 26 | export doc at playwright.config.ts:12 |  |  | 0.789 |
| walker |  | 4638 | 18 | imports in src/helpers.ts |  |  | 0.789 |
| walker |  | 4749 | 111 | json config turbo.json |  |  | 0.790 |
| ns | 4761 |  | 193 | use-prevent-scroll.ts exported surface and its provenance | 3.11 |  | 0.776 |
| walker |  | 4846 | 97 | export names surface in src/use-prevent-scroll.ts |  |  | 0.780 |
| walker |  | 4846 | 0 | export at src/use-prevent-scroll.ts:29 |  |  | 0.780 |
| walker |  | 4846 | 0 | export at src/use-prevent-scroll.ts:34 |  |  | 0.780 |
| walker |  | 4846 | 0 | export at src/use-prevent-scroll.ts:68 |  |  | 0.780 |
| walker |  | 4846 | 0 | export at src/use-prevent-scroll.ts:294 |  |  | 0.780 |
| walker |  | 4887 | 41 | export body at src/use-prevent-scroll.ts:29 body 30 |  |  | 0.780 |
| ns | 4910 |  | 149 | use-prevent-scroll.ts module-private declarations | 3.12 | 3.11 | 0.769 |
| walker |  | 4949 | 62 | export body at src/use-prevent-scroll.ts:294 body 295 |  |  | 0.769 |
| ns | 5128 |  | 218 | use-controllable-state.ts — the controlled/uncontrolled prop machinery | 3.13 |  | 0.758 |
| walker |  | 5266 | 317 | export body at src/index.tsx:803 body 805 |  |  | 0.758 |
| ns | 5322 |  | 194 | useScaleBackground and use-composed-refs — the two smallest modules | 3.14 |  | 0.749 |
| walker |  | 5358 | 92 | export body at src/use-prevent-scroll.ts:34 body 35 |  |  | 0.749 |
| walker |  | 5381 | 23 | imports in src/context.ts |  |  | 0.749 |
| walker |  | 5404 | 23 | imports in src/use-position-fixed.ts |  |  | 0.749 |
| walker |  | 5516 | 112 | export names surface in src/browser.ts |  |  | 0.761 |
| walker |  | 5516 | 0 | export at src/browser.ts:1 |  |  | 0.761 |
| walker |  | 5516 | 0 | export at src/browser.ts:10 |  |  | 0.761 |
| walker |  | 5516 | 0 | export at src/browser.ts:14 |  |  | 0.761 |
| walker |  | 5516 | 0 | export at src/browser.ts:18 |  |  | 0.761 |
| walker |  | 5516 | 0 | export at src/browser.ts:22 |  |  | 0.761 |
| walker |  | 5516 | 0 | export at src/browser.ts:30 |  |  | 0.761 |
| walker |  | 5516 | 0 | export at src/browser.ts:34 |  |  | 0.761 |
| walker |  | 5528 | 12 | export body at src/browser.ts:10 body 11 |  |  | 0.761 |
| walker |  | 5541 | 13 | export body at src/browser.ts:14 body 15 |  |  | 0.761 |
| ns | 5545 |  | 223 | Every member declared inside Root (names + effect locations) | 4.1 |  | 0.744 |
| walker |  | 5564 | 23 | export body at src/browser.ts:18 body 19 |  |  | 0.744 |
| walker |  | 5647 | 83 | export body at src/browser.ts:1 body 2 |  |  | 0.744 |
| ns | 5763 |  | 218 | Root's state and refs — the whole drag bookkeeping set | 4.2 |  | 0.727 |
| walker |  | 5791 | 144 | export names surface in src/constants.ts |  |  | 0.747 |
| walker |  | 5831 | 40 | export at src/constants.ts:1 |  |  | 0.754 |
| walker |  | 5863 | 32 | imports in src/use-prevent-scroll.ts |  |  | 0.754 |
| ns | 5870 |  | 107 | Root's useSnapPoints wiring — what it destructures and what it passes | 4.3 |  | 0.744 |
| ns | 6007 |  | 137 | Scroll-lock and body-position wiring, including the isDisabled predicate | 4.4 |  | 0.733 |
| walker |  | 6112 | 249 | export at src/use-snap-points.ts:7 |  |  | 0.765 |
| ns | 6217 |  | 210 | shouldDrag: the early-out guards | 4.5 | 4.1 | 0.749 |
| walker |  | 6283 | 171 | export names surface in src/helpers.ts |  |  | 0.760 |
| walker |  | 6283 | 0 | export at src/helpers.ts:9 |  |  | 0.760 |
| walker |  | 6283 | 0 | export at src/helpers.ts:23 |  |  | 0.760 |
| walker |  | 6283 | 0 | export at src/helpers.ts:42 |  |  | 0.760 |
| walker |  | 6283 | 0 | export at src/helpers.ts:59 |  |  | 0.760 |
| walker |  | 6283 | 0 | export at src/helpers.ts:72 |  |  | 0.760 |
| walker |  | 6283 | 0 | export at src/helpers.ts:90 |  |  | 0.760 |
| walker |  | 6283 | 0 | export at src/helpers.ts:94 |  |  | 0.760 |
| walker |  | 6283 | 0 | export at src/helpers.ts:108 |  |  | 0.760 |
| walker |  | 6312 | 29 | export doc at src/helpers.ts:108 |  |  | 0.760 |
| walker |  | 6397 | 85 | export body at src/helpers.ts:59 body 60 |  |  | 0.760 |
| walker |  | 6513 | 116 | export body at src/helpers.ts:9 body 10 |  |  | 0.760 |
| ns | 6567 |  | 350 | shouldDrag: direction, open-animation window and scroll-lock timeout | 4.6 | 4.5 | 0.734 |
| walker |  | 6659 | 146 | export body at src/helpers.ts:42 body 43 |  |  | 0.734 |
| ns | 6816 |  | 249 | shouldDrag: the scrollable-ancestor climb | 4.7 | 4.6 | 0.716 |
| walker |  | 6846 | 187 | export body at src/use-prevent-scroll.ts:68 body 69 |  |  | 0.716 |
| walker |  | 6904 | 58 | export doc at src/use-prevent-scroll.ts:68 |  |  | 0.716 |
| ns | 7045 |  | 229 | onRelease: teardown and velocity computation | 4.8 | 4.1 | 0.704 |
| walker |  | 7151 | 247 | LICENSE.md section #0 |  |  | 0.704 |
| ns | 7358 |  | 313 | onRelease: the close-vs-snap-back decision ladder | 4.9 | 4.8 | 0.687 |
| walker |  | 7478 | 327 | export at src/context.ts:37 |  |  | 0.689 |
| ns | 7630 |  | 272 | The data-vaul-* attributes emitted by Overlay and Content | 4.10 |  | 0.683 |
| walker |  | 7735 | 257 | export body at src/use-controllable-state.ts:39 body 40 |  |  | 0.683 |
| ns | 7799 |  | 169 | Handle: click-to-cycle snap points | 4.11 |  | 0.674 |
| walker |  | 7909 | 174 | export body at src/helpers.ts:23 body 24 |  |  | 0.674 |
| ns | 7945 |  | 146 | NestedRoot: how a nested drawer is wired to its parent | 4.12 |  | 0.665 |
| walker |  | 7978 | 69 | imports in src/use-scale-background.ts |  |  | 0.665 |
| walker |  | 8050 | 72 | imports in src/use-snap-points.ts |  |  | 0.665 |
| ns | 8095 |  | 150 | The base [data-vaul-drawer] rule and the shape of the variant rules | 5.1 |  | 0.660 |
| walker |  | 8126 | 76 | README headline in test/README.md |  |  | 0.660 |
| walker |  | 8149 | 23 | headings outline in test/README.md |  |  | 0.660 |
| ns | 8280 |  | 185 | style.css selector inventory — which attribute combinations are styled | 5.2 | 5.1 | 0.655 |
| ns | 8408 |  | 128 | Every @keyframes name in style.css | 5.3 |  | 0.649 |
| ns | 8523 |  | 115 | Complete listing of test/src/app — one demo route per feature | 6.1 |  | 0.658 |
| ns | 8589 |  | 66 | Complete listing of test/tests — the Playwright spec set | 6.2 |  | 0.663 |
| ns | 8642 |  | 53 | Listing of the test/ package root | 6.3 |  | 0.667 |
| ns | 8935 |  | 293 | Spec-suite to demo-route map for every Playwright file | 6.4 |  | 0.658 |
| ns | 9137 |  | 202 | Playwright runner configuration: server, devices, testDir | 6.5 |  | 0.665 |
| ns | 9243 |  | 106 | Shared e2e helpers: openDrawer and ANIMATION_DURATION | 6.6 |  | 0.662 |
| walker |  | 9250 | 1101 | export body at src/index.tsx:996 body 1000 |  |  | 0.677 |
| ns | 9374 |  | 131 | package.json publishing surface: files and the exports map | 7.1 |  | 0.682 |
| walker |  | 9530 | 280 | declaration surface of src/style.css |  |  | 0.682 |
| ns | 9536 |  | 162 | Runtime and peer dependencies, and the pinned package manager | 7.2 |  | 0.678 |
| ns | 9664 |  | 128 | Workspace and task-runner config: pnpm-workspace.yaml and turbo.json | 7.3 |  | 0.683 |
| walker |  | 9677 | 147 | export doc at src/use-position-fixed.ts:15 |  |  | 0.692 |
| ns | 9769 |  | 105 | Root tsconfig.json — compiler settings for the shipped library | 7.4 |  | 0.695 |
| ns | 9773 |  | 4 | Listing of .github/workflows | 7.5 |  | 0.695 |
| ns | 9933 |  | 160 | CI: the Playwright workflow steps | 7.6 |  | 0.690 |
| ns | 9998 |  | 65 | Prettier configuration — the formatting any new code must match | 7.7 |  | 0.686 |

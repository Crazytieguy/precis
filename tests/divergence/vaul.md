Score(3000)=0.847 I=0.910 C=0.788 ns_rows≤3K=17/42 (reached=12 partial=1 missing=4)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 66 | 66 | listing of '.' |  |  | 1.000 |
| ns | 66 |  | 66 | Fixture root listing | 1.1 |  | 1.000 |
| walker |  | 69 | 3 | listing of '.github' |  |  | 1.000 |
| walker |  | 72 | 3 | listing of '.github/workflows' |  |  | 1.000 |
| walker |  | 75 | 3 | listing of '.vscode' |  |  | 1.000 |
| ns | 111 |  | 45 | package.json — name/version/description | 1.2 |  | 0.896 |
| walker |  | 133 | 58 | package identity in package.json |  |  | 1.000 |
| walker |  | 169 | 36 | package runtime dependencies in package.json |  |  | 1.000 |
| ns | 179 |  | 68 | README.md (full) | 1.3 |  | 0.912 |
| walker |  | 188 | 19 | package runtime metadata in package.json |  |  | 0.913 |
| ns | 249 |  | 70 | src/ directory listing | 1.4 |  | 0.683 |
| walker |  | 258 | 70 | listing of 'src' |  |  | 0.941 |
| walker |  | 275 | 17 | module item at src/index.tsx:993 |  |  | 0.941 |
| walker |  | 288 | 13 | module item at src/index.tsx:994 |  |  | 0.941 |
| walker |  | 351 | 63 | README.md section #0 |  |  | 0.982 |
| walker |  | 364 | 13 | export names surface in playwright.config.ts |  |  | 0.982 |
| ns | 386 |  | 137 | package.json — scripts | 1.5 |  | 0.852 |
| walker |  | 387 | 23 | plaintext config pnpm-workspace.yaml |  |  | 0.852 |
| ns | 440 |  | 54 | test/ workspace top-level listing | 1.6 |  | 0.744 |
| ns | 555 |  | 115 | test/src/app/ demo route listing | 1.7 |  | 0.622 |
| walker |  | 611 | 224 | export names surface in src/index.tsx |  |  | 0.628 |
| walker |  | 611 | 0 | export at src/index.tsx:1098 |  |  | 0.628 |
| walker |  | 611 | 0 | export at src/index.tsx:1130 |  |  | 0.628 |
| ns | 620 |  | 65 | test/tests/ Playwright spec listing | 1.8 |  | 0.578 |
| walker |  | 624 | 13 | export at src/index.tsx:989 |  |  | 0.578 |
| walker |  | 637 | 13 | export at src/index.tsx:803 |  |  | 0.578 |
| walker |  | 666 | 29 | export at src/index.tsx:996 |  |  | 0.578 |
| walker |  | 700 | 34 | export at src/index.tsx:833 |  |  | 0.579 |
| ns | 792 |  | 172 | package.json — entry points + files | 1.9 |  | 0.519 |
| walker |  | 820 | 120 | export at src/index.tsx:40 |  |  | 0.521 |
| walker |  | 981 | 161 | export at src/index.tsx:27 |  |  | 0.527 |
| ns | 1003 |  | 211 | package.json — devDependencies | 1.10 |  | 0.488 |
| walker |  | 1082 | 101 | export at src/index.tsx:1137 |  |  | 0.493 |
| ns | 1154 |  | 151 | package.json — peerDependencies + runtime dependency | 1.11 |  | 0.482 |
| ns | 1269 |  | 115 | Drawer export object (full public API surface) | 2.1 |  | 0.528 |
| walker |  | 1400 | 318 | export at src/index.tsx:139 |  |  | 0.535 |
| ns | 1435 |  | 166 | index.tsx top-level export locations | 2.2 |  | 0.563 |
| walker |  | 1454 | 54 | listing of 'test' |  |  | 0.629 |
| walker |  | 1457 | 3 | listing of 'test/src' |  |  | 0.629 |
| walker |  | 1471 | 14 | export names surface in src/use-position-fixed.ts |  |  | 0.629 |
| walker |  | 1485 | 14 | export names surface in src/use-snap-points.ts |  |  | 0.629 |
| walker |  | 1622 | 137 | package scripts in package.json |  |  | 0.684 |
| walker |  | 1637 | 15 | export names surface in src/use-scale-background.ts |  |  | 0.684 |
| walker |  | 1637 | 0 | export at src/use-scale-background.ts:8 |  |  | 0.684 |
| walker |  | 1653 | 16 | export names surface in src/use-composed-refs.ts |  |  | 0.684 |
| ns | 1721 |  | 286 | WithFadeFromProps / WithoutFadeFromProps (full) | 2.3 | 2.2 | 0.711 |
| walker |  | 1825 | 172 | package entrypoints in package.json |  |  | 0.783 |
| ns | 2263 |  | 542 | DialogProps — core open/behavior props (open .. dismissible) | 2.4 | 2.2 | 0.689 |
| ns | 2461 |  | 198 | DialogProps — drag/modal/direction props (onDrag .. direction) | 2.5 | 2.2 | 0.664 |
| ns | 2891 |  | 430 | DialogProps — snap/animation/misc props (defaultOpen .. autoFocus) | 2.6 | 2.2 | 0.615 |
| walker |  | 2995 | 1170 | export at src/index.tsx:50 |  |  | 0.847 |
| walker |  | 3171 | 176 | package identity metadata in package.json |  |  | 0.847 |
| ns | 3209 |  | 318 | Root() destructured props + defaults | 2.7 | 2.2 | 0.851 |
| walker |  | 3282 | 111 | json config tsconfig.json |  |  | 0.851 |
| walker |  | 3318 | 36 | export names surface in src/context.ts |  |  | 0.851 |
| walker |  | 3318 | 0 | export at src/context.ts:69 |  |  | 0.851 |
| walker |  | 3383 | 65 | listing of 'test/tests' |  |  | 0.881 |
| walker |  | 3400 | 17 | imports in playwright.config.ts |  |  | 0.881 |
| walker |  | 3514 | 114 | listing of 'test/src/app' |  |  | 0.938 |
| walker |  | 3551 | 37 | export names surface in src/use-controllable-state.ts |  |  | 0.938 |
| walker |  | 3551 | 0 | export at src/use-controllable-state.ts:39 |  |  | 0.938 |
| ns | 3578 |  | 369 | Overlay component (full) | 2.8 | 2.2 | 0.891 |
| walker |  | 3865 | 314 | imports in src/index.tsx |  |  | 0.891 |
| walker |  | 3877 | 12 | imports in src/use-controllable-state.ts |  |  | 0.891 |
| walker |  | 3885 | 8 | listing of 'test/public' |  |  | 0.891 |
| walker |  | 3899 | 14 | imports in src/use-composed-refs.ts |  |  | 0.891 |
| ns | 3922 |  | 344 | ContentProps type + Content()'s destructured context fields | 2.9 | 2.2 | 0.847 |
| walker |  | 4023 | 124 | export at src/use-position-fixed.ts:15 |  |  | 0.847 |
| walker |  | 4082 | 59 | export body at src/context.ts:69 body 70 |  |  | 0.847 |
| ns | 4167 |  | 245 | HandleProps type + Handle()'s destructured context fields | 2.10 | 2.2 | 0.820 |
| ns | 4486 |  | 319 | NestedRoot() (full) | 2.11 | 2.2 | 0.785 |
| ns | 4567 |  | 81 | Portal component (full) | 2.12 | 2.2 | 0.777 |
| walker |  | 4594 | 512 | export at playwright.config.ts:12 |  |  | 0.778 |
| walker |  | 4620 | 26 | export doc at playwright.config.ts:12 |  |  | 0.778 |
| walker |  | 4638 | 18 | imports in src/helpers.ts |  |  | 0.778 |
| ns | 4690 |  | 123 | Content()'s rendered JSX — pointer/focus handler + isDeltaInDirection locations | 2.13 | 2.9 | 0.767 |
| walker |  | 4749 | 111 | json config turbo.json |  |  | 0.767 |
| walker |  | 4846 | 97 | export names surface in src/use-prevent-scroll.ts |  |  | 0.768 |
| walker |  | 4846 | 0 | export at src/use-prevent-scroll.ts:29 |  |  | 0.768 |
| walker |  | 4846 | 0 | export at src/use-prevent-scroll.ts:34 |  |  | 0.768 |
| walker |  | 4846 | 0 | export at src/use-prevent-scroll.ts:68 |  |  | 0.768 |
| walker |  | 4846 | 0 | export at src/use-prevent-scroll.ts:294 |  |  | 0.768 |
| ns | 4858 |  | 168 | Root() internal handler locations | 3.1 | 2.7 | 0.754 |
| walker |  | 4887 | 41 | export body at src/use-prevent-scroll.ts:29 body 30 |  |  | 0.754 |
| walker |  | 4949 | 62 | export body at src/use-prevent-scroll.ts:294 body 295 |  |  | 0.754 |
| ns | 5130 |  | 272 | onPress() (full) | 3.2 | 3.1 | 0.737 |
| walker |  | 5266 | 317 | export body at src/index.tsx:803 body 805 |  |  | 0.771 |
| walker |  | 5358 | 92 | export body at src/use-prevent-scroll.ts:34 body 35 |  |  | 0.771 |
| walker |  | 5381 | 23 | imports in src/context.ts |  |  | 0.771 |
| walker |  | 5404 | 23 | imports in src/use-position-fixed.ts |  |  | 0.771 |
| walker |  | 5516 | 112 | export names surface in src/browser.ts |  |  | 0.771 |
| walker |  | 5516 | 0 | export at src/browser.ts:1 |  |  | 0.771 |
| walker |  | 5516 | 0 | export at src/browser.ts:10 |  |  | 0.771 |
| walker |  | 5516 | 0 | export at src/browser.ts:14 |  |  | 0.771 |
| walker |  | 5516 | 0 | export at src/browser.ts:18 |  |  | 0.771 |
| walker |  | 5516 | 0 | export at src/browser.ts:22 |  |  | 0.771 |
| walker |  | 5516 | 0 | export at src/browser.ts:30 |  |  | 0.771 |
| walker |  | 5516 | 0 | export at src/browser.ts:34 |  |  | 0.771 |
| walker |  | 5528 | 12 | export body at src/browser.ts:10 body 11 |  |  | 0.771 |
| walker |  | 5541 | 13 | export body at src/browser.ts:14 body 15 |  |  | 0.771 |
| walker |  | 5564 | 23 | export body at src/browser.ts:18 body 19 |  |  | 0.771 |
| walker |  | 5647 | 83 | export body at src/browser.ts:1 body 2 |  |  | 0.771 |
| walker |  | 5949 | 302 | package dev/peer dependencies in package.json |  |  | 0.809 |
| ns | 5961 |  | 831 | shouldDrag() (full) — the gesture-permission gate | 3.3 | 3.1 | 0.741 |
| walker |  | 6093 | 144 | export names surface in src/constants.ts |  |  | 0.742 |
| walker |  | 6133 | 40 | export at src/constants.ts:1 |  |  | 0.743 |
| walker |  | 6165 | 32 | imports in src/use-prevent-scroll.ts |  |  | 0.743 |
| ns | 6352 |  | 391 | closeDrawer() / cancelDrag() (full) + resetDrawer() signature and transform reset | 3.4 | 3.1 | 0.713 |
| walker |  | 6414 | 249 | export at src/use-snap-points.ts:7 |  |  | 0.713 |
| walker |  | 6585 | 171 | export names surface in src/helpers.ts |  |  | 0.714 |
| walker |  | 6585 | 0 | export at src/helpers.ts:9 |  |  | 0.714 |
| walker |  | 6585 | 0 | export at src/helpers.ts:23 |  |  | 0.714 |
| walker |  | 6585 | 0 | export at src/helpers.ts:42 |  |  | 0.714 |
| walker |  | 6585 | 0 | export at src/helpers.ts:59 |  |  | 0.714 |
| walker |  | 6585 | 0 | export at src/helpers.ts:72 |  |  | 0.714 |
| walker |  | 6585 | 0 | export at src/helpers.ts:90 |  |  | 0.714 |
| walker |  | 6585 | 0 | export at src/helpers.ts:94 |  |  | 0.714 |
| walker |  | 6585 | 0 | export at src/helpers.ts:108 |  |  | 0.714 |
| ns | 6600 |  | 248 | onRelease() — velocity/closeThreshold close-or-reset decision | 3.5 | 3.1 | 0.700 |
| walker |  | 6614 | 29 | export doc at src/helpers.ts:108 |  |  | 0.700 |
| walker |  | 6699 | 85 | export body at src/helpers.ts:59 body 60 |  |  | 0.700 |
| ns | 6732 |  | 132 | Root() small effects — enter-animation flag + non-modal pointer-events | 3.6 |  | 0.691 |
| walker |  | 6815 | 116 | export body at src/helpers.ts:9 body 10 |  |  | 0.691 |
| ns | 6921 |  | 189 | constants.ts (full) | 4.1 |  | 0.698 |
| walker |  | 6961 | 146 | export body at src/helpers.ts:42 body 43 |  |  | 0.699 |
| ns | 6999 |  | 78 | types.ts (full) | 4.2 |  | 0.694 |
| walker |  | 7148 | 187 | export body at src/use-prevent-scroll.ts:68 body 69 |  |  | 0.694 |
| walker |  | 7206 | 58 | export doc at src/use-prevent-scroll.ts:68 |  |  | 0.694 |
| walker |  | 7453 | 247 | LICENSE.md section #0 |  |  | 0.694 |
| ns | 7571 |  | 572 | Hook/helper function locations across src/ | 4.3 |  | 0.682 |
| walker |  | 7780 | 327 | export at src/context.ts:37 |  |  | 0.682 |
| ns | 7838 |  | 267 | DrawerContextValue — internal-only fields (refs, callbacks, imperative state) | 4.4 | 4.3 | 0.673 |
| walker |  | 8037 | 257 | export body at src/use-controllable-state.ts:39 body 40 |  |  | 0.673 |
| walker |  | 8211 | 174 | export body at src/helpers.ts:23 body 24 |  |  | 0.675 |
| walker |  | 8280 | 69 | imports in src/use-scale-background.ts |  |  | 0.675 |
| ns | 8300 |  | 462 | helpers.ts — set()/reset()/isVertical() (full) | 4.5 | 4.3 | 0.689 |
| walker |  | 8352 | 72 | imports in src/use-snap-points.ts |  |  | 0.689 |
| walker |  | 8428 | 76 | README headline in test/README.md |  |  | 0.689 |
| walker |  | 8451 | 23 | headings outline in test/README.md |  |  | 0.689 |
| ns | 8613 |  | 313 | use-prevent-scroll.ts — Mobile Safari workaround rationale (comment) + isInput() | 4.6 | 4.3 | 0.679 |
| ns | 8939 |  | 326 | use-position-fixed.ts — rationale comment + signature | 4.7 | 4.3 | 0.670 |
| ns | 9053 |  | 114 | style.css — shared [data-vaul-drawer] base rule | 5.1 |  | 0.667 |
| ns | 9374 |  | 321 | test/src/app/page.tsx — demo index (full) | 6.1 |  | 0.657 |
| walker |  | 9552 | 1101 | export body at src/index.tsx:996 body 1000 |  |  | 0.675 |
| walker |  | 9581 | 29 | test names surface in test/tests/with-redirect.spec.ts |  |  | 0.675 |
| walker |  | 9610 | 29 | test names surface in test/tests/without-scaled-background.spec.ts |  |  | 0.675 |
| walker |  | 9640 | 30 | test names surface in test/tests/nested.spec.ts |  |  | 0.675 |
| walker |  | 9672 | 32 | test names surface in test/tests/with-handle.spec.ts |  |  | 0.675 |
| walker |  | 9705 | 33 | test names surface in test/tests/initial-snap.spec.ts |  |  | 0.675 |
| ns | 9747 |  | 373 | Per-demo distinguishing Drawer.Root/Handle line, one per test/src/app/*/page.tsx route | 6.2 |  | 0.668 |
| walker |  | 9852 | 147 | export doc at src/use-position-fixed.ts:15 |  |  | 0.682 |
| walker |  | 9896 | 44 | test names surface in test/tests/with-scaled-background.spec.ts |  |  | 0.682 |
| ns | 9926 |  | 179 | playwright.config.ts — webServer + device projects | 7.1 |  | 0.688 |
| ns | 10004 |  | 78 | CI workflow — install/build/test steps | 7.2 |  | 0.685 |

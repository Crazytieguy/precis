Score(3000)=0.847 I=0.910 C=0.788 ns_rows≤3K=17/42 (reached=12 partial=1 missing=4)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 62 | 62 | listing of '.' |  |  | 1.000 |
| ns | 62 |  | 62 | Fixture root listing | 1.1 |  | 1.000 |
| walker |  | 65 | 3 | listing of '.github' |  |  | 1.000 |
| walker |  | 69 | 4 | listing of '.github/workflows' |  |  | 1.000 |
| walker |  | 73 | 4 | listing of '.vscode' |  |  | 1.000 |
| ns | 107 |  | 45 | package.json — name/version/description | 1.2 |  | 0.896 |
| walker |  | 131 | 58 | package identity in package.json |  |  | 1.000 |
| walker |  | 152 | 21 | package runtime metadata in package.json |  |  | 1.000 |
| ns | 175 |  | 68 | README.md (full) | 1.3 |  | 0.911 |
| walker |  | 223 | 71 | listing of 'src' |  |  | 0.929 |
| walker |  | 240 | 17 | module item at src/index.tsx:993 |  |  | 0.929 |
| ns | 246 |  | 71 | src/ directory listing | 1.4 |  | 0.940 |
| walker |  | 253 | 13 | module item at src/index.tsx:994 |  |  | 0.940 |
| walker |  | 316 | 63 | README.md section #0 |  |  | 0.980 |
| walker |  | 329 | 13 | export names surface in playwright.config.ts |  |  | 0.980 |
| walker |  | 352 | 23 | plaintext config pnpm-workspace.yaml |  |  | 0.980 |
| ns | 383 |  | 137 | package.json — scripts | 1.5 |  | 0.851 |
| ns | 435 |  | 52 | test/ workspace top-level listing | 1.6 |  | 0.743 |
| ns | 534 |  | 99 | test/src/app/ demo route listing | 1.7 |  | 0.621 |
| walker |  | 576 | 224 | export names surface in src/index.tsx |  |  | 0.627 |
| walker |  | 576 | 0 | export at src/index.tsx:1098 |  |  | 0.627 |
| walker |  | 576 | 0 | export at src/index.tsx:1130 |  |  | 0.627 |
| walker |  | 589 | 13 | export at src/index.tsx:989 |  |  | 0.627 |
| ns | 600 |  | 66 | test/tests/ Playwright spec listing | 1.8 |  | 0.577 |
| walker |  | 602 | 13 | export at src/index.tsx:803 |  |  | 0.577 |
| walker |  | 631 | 29 | export at src/index.tsx:996 |  |  | 0.578 |
| walker |  | 665 | 34 | export at src/index.tsx:833 |  |  | 0.578 |
| ns | 772 |  | 172 | package.json — entry points + files | 1.9 |  | 0.518 |
| walker |  | 785 | 120 | export at src/index.tsx:40 |  |  | 0.520 |
| walker |  | 946 | 161 | export at src/index.tsx:27 |  |  | 0.526 |
| ns | 983 |  | 211 | package.json — devDependencies | 1.10 |  | 0.488 |
| walker |  | 1047 | 101 | export at src/index.tsx:1137 |  |  | 0.493 |
| ns | 1134 |  | 151 | package.json — peerDependencies + runtime dependency | 1.11 |  | 0.472 |
| ns | 1249 |  | 115 | Drawer export object (full public API surface) | 2.1 |  | 0.520 |
| walker |  | 1365 | 318 | export at src/index.tsx:139 |  |  | 0.527 |
| walker |  | 1379 | 14 | export names surface in src/use-position-fixed.ts |  |  | 0.527 |
| walker |  | 1393 | 14 | export names surface in src/use-snap-points.ts |  |  | 0.527 |
| ns | 1415 |  | 166 | index.tsx top-level export locations | 2.2 |  | 0.556 |
| walker |  | 1530 | 137 | package scripts in package.json |  |  | 0.613 |
| walker |  | 1545 | 15 | export names surface in src/use-scale-background.ts |  |  | 0.613 |
| walker |  | 1545 | 0 | export at src/use-scale-background.ts:8 |  |  | 0.613 |
| walker |  | 1597 | 52 | listing of 'test' |  |  | 0.678 |
| walker |  | 1600 | 3 | listing of 'test/src' |  |  | 0.678 |
| walker |  | 1616 | 16 | export names surface in src/use-composed-refs.ts |  |  | 0.678 |
| ns | 1701 |  | 286 | WithFadeFromProps / WithoutFadeFromProps (full) | 2.3 | 2.2 | 0.705 |
| walker |  | 1788 | 172 | package entrypoints in package.json |  |  | 0.778 |
| walker |  | 1822 | 34 | package runtime dependencies in package.json |  |  | 0.783 |
| ns | 2243 |  | 542 | DialogProps — core open/behavior props (open .. dismissible) | 2.4 | 2.2 | 0.689 |
| ns | 2441 |  | 198 | DialogProps — drag/modal/direction props (onDrag .. direction) | 2.5 | 2.2 | 0.664 |
| ns | 2871 |  | 430 | DialogProps — snap/animation/misc props (defaultOpen .. autoFocus) | 2.6 | 2.2 | 0.615 |
| walker |  | 2992 | 1170 | export at src/index.tsx:50 |  |  | 0.847 |
| walker |  | 3168 | 176 | package identity metadata in package.json |  |  | 0.847 |
| ns | 3189 |  | 318 | Root() destructured props + defaults | 2.7 | 2.2 | 0.851 |
| walker |  | 3279 | 111 | json config tsconfig.json |  |  | 0.851 |
| walker |  | 3315 | 36 | export names surface in src/context.ts |  |  | 0.851 |
| walker |  | 3315 | 0 | export at src/context.ts:69 |  |  | 0.851 |
| walker |  | 3414 | 99 | listing of 'test/src/app' |  |  | 0.908 |
| walker |  | 3431 | 17 | imports in playwright.config.ts |  |  | 0.908 |
| walker |  | 3468 | 37 | export names surface in src/use-controllable-state.ts |  |  | 0.908 |
| walker |  | 3468 | 0 | export at src/use-controllable-state.ts:39 |  |  | 0.908 |
| walker |  | 3534 | 66 | listing of 'test/tests' |  |  | 0.938 |
| ns | 3558 |  | 369 | Overlay component (full) | 2.8 | 2.2 | 0.891 |
| walker |  | 3848 | 314 | imports in src/index.tsx |  |  | 0.891 |
| walker |  | 3860 | 12 | imports in src/use-controllable-state.ts |  |  | 0.891 |
| walker |  | 3874 | 14 | imports in src/use-composed-refs.ts |  |  | 0.891 |
| walker |  | 3883 | 9 | listing of 'test/public' |  |  | 0.891 |
| ns | 3902 |  | 344 | ContentProps type + Content()'s destructured context fields | 2.9 | 2.2 | 0.847 |
| walker |  | 4007 | 124 | export at src/use-position-fixed.ts:15 |  |  | 0.847 |
| walker |  | 4066 | 59 | export body at src/context.ts:69 body 70 |  |  | 0.847 |
| ns | 4147 |  | 245 | HandleProps type + Handle()'s destructured context fields | 2.10 | 2.2 | 0.820 |
| ns | 4466 |  | 319 | NestedRoot() (full) | 2.11 | 2.2 | 0.785 |
| ns | 4547 |  | 81 | Portal component (full) | 2.12 | 2.2 | 0.777 |
| walker |  | 4578 | 512 | export at playwright.config.ts:12 |  |  | 0.778 |
| walker |  | 4604 | 26 | export doc at playwright.config.ts:12 |  |  | 0.778 |
| walker |  | 4622 | 18 | imports in src/helpers.ts |  |  | 0.778 |
| ns | 4670 |  | 123 | Content()'s rendered JSX — pointer/focus handler + isDeltaInDirection locations | 2.13 | 2.9 | 0.767 |
| walker |  | 4733 | 111 | json config turbo.json |  |  | 0.767 |
| walker |  | 4830 | 97 | export names surface in src/use-prevent-scroll.ts |  |  | 0.768 |
| walker |  | 4830 | 0 | export at src/use-prevent-scroll.ts:29 |  |  | 0.768 |
| walker |  | 4830 | 0 | export at src/use-prevent-scroll.ts:34 |  |  | 0.768 |
| walker |  | 4830 | 0 | export at src/use-prevent-scroll.ts:68 |  |  | 0.768 |
| walker |  | 4830 | 0 | export at src/use-prevent-scroll.ts:294 |  |  | 0.768 |
| ns | 4838 |  | 168 | Root() internal handler locations | 3.1 | 2.7 | 0.754 |
| walker |  | 4871 | 41 | export body at src/use-prevent-scroll.ts:29 body 30 |  |  | 0.754 |
| walker |  | 4933 | 62 | export body at src/use-prevent-scroll.ts:294 body 295 |  |  | 0.754 |
| ns | 5110 |  | 272 | onPress() (full) | 3.2 | 3.1 | 0.737 |
| walker |  | 5250 | 317 | export body at src/index.tsx:803 body 805 |  |  | 0.771 |
| walker |  | 5342 | 92 | export body at src/use-prevent-scroll.ts:34 body 35 |  |  | 0.771 |
| walker |  | 5365 | 23 | imports in src/context.ts |  |  | 0.771 |
| walker |  | 5388 | 23 | imports in src/use-position-fixed.ts |  |  | 0.771 |
| walker |  | 5500 | 112 | export names surface in src/browser.ts |  |  | 0.771 |
| walker |  | 5500 | 0 | export at src/browser.ts:1 |  |  | 0.771 |
| walker |  | 5500 | 0 | export at src/browser.ts:10 |  |  | 0.771 |
| walker |  | 5500 | 0 | export at src/browser.ts:14 |  |  | 0.771 |
| walker |  | 5500 | 0 | export at src/browser.ts:18 |  |  | 0.771 |
| walker |  | 5500 | 0 | export at src/browser.ts:22 |  |  | 0.771 |
| walker |  | 5500 | 0 | export at src/browser.ts:30 |  |  | 0.771 |
| walker |  | 5500 | 0 | export at src/browser.ts:34 |  |  | 0.771 |
| walker |  | 5512 | 12 | export body at src/browser.ts:10 body 11 |  |  | 0.771 |
| walker |  | 5525 | 13 | export body at src/browser.ts:14 body 15 |  |  | 0.771 |
| walker |  | 5548 | 23 | export body at src/browser.ts:18 body 19 |  |  | 0.771 |
| walker |  | 5631 | 83 | export body at src/browser.ts:1 body 2 |  |  | 0.771 |
| walker |  | 5933 | 302 | package dev/peer dependencies in package.json |  |  | 0.809 |
| ns | 5941 |  | 831 | shouldDrag() (full) — the gesture-permission gate | 3.3 | 3.1 | 0.741 |
| walker |  | 6077 | 144 | export names surface in src/constants.ts |  |  | 0.742 |
| walker |  | 6117 | 40 | export at src/constants.ts:1 |  |  | 0.743 |
| walker |  | 6149 | 32 | imports in src/use-prevent-scroll.ts |  |  | 0.743 |
| ns | 6332 |  | 391 | closeDrawer() / cancelDrag() (full) + resetDrawer() signature and transform reset | 3.4 | 3.1 | 0.713 |
| walker |  | 6398 | 249 | export at src/use-snap-points.ts:7 |  |  | 0.713 |
| walker |  | 6569 | 171 | export names surface in src/helpers.ts |  |  | 0.714 |
| walker |  | 6569 | 0 | export at src/helpers.ts:9 |  |  | 0.714 |
| walker |  | 6569 | 0 | export at src/helpers.ts:23 |  |  | 0.714 |
| walker |  | 6569 | 0 | export at src/helpers.ts:42 |  |  | 0.714 |
| walker |  | 6569 | 0 | export at src/helpers.ts:59 |  |  | 0.714 |
| walker |  | 6569 | 0 | export at src/helpers.ts:72 |  |  | 0.714 |
| walker |  | 6569 | 0 | export at src/helpers.ts:90 |  |  | 0.714 |
| walker |  | 6569 | 0 | export at src/helpers.ts:94 |  |  | 0.714 |
| walker |  | 6569 | 0 | export at src/helpers.ts:108 |  |  | 0.714 |
| ns | 6580 |  | 248 | onRelease() — velocity/closeThreshold close-or-reset decision | 3.5 | 3.1 | 0.700 |
| walker |  | 6598 | 29 | export doc at src/helpers.ts:108 |  |  | 0.700 |
| walker |  | 6683 | 85 | export body at src/helpers.ts:59 body 60 |  |  | 0.700 |
| ns | 6712 |  | 132 | Root() small effects — enter-animation flag + non-modal pointer-events | 3.6 |  | 0.691 |
| walker |  | 6799 | 116 | export body at src/helpers.ts:9 body 10 |  |  | 0.691 |
| ns | 6901 |  | 189 | constants.ts (full) | 4.1 |  | 0.698 |
| walker |  | 6945 | 146 | export body at src/helpers.ts:42 body 43 |  |  | 0.699 |
| ns | 6979 |  | 78 | types.ts (full) | 4.2 |  | 0.694 |
| walker |  | 7132 | 187 | export body at src/use-prevent-scroll.ts:68 body 69 |  |  | 0.694 |
| walker |  | 7190 | 58 | export doc at src/use-prevent-scroll.ts:68 |  |  | 0.694 |
| walker |  | 7437 | 247 | LICENSE.md section #0 |  |  | 0.694 |
| ns | 7551 |  | 572 | Hook/helper function locations across src/ | 4.3 |  | 0.682 |
| walker |  | 7764 | 327 | export at src/context.ts:37 |  |  | 0.682 |
| ns | 7818 |  | 267 | DrawerContextValue — internal-only fields (refs, callbacks, imperative state) | 4.4 | 4.3 | 0.673 |
| walker |  | 8021 | 257 | export body at src/use-controllable-state.ts:39 body 40 |  |  | 0.673 |
| walker |  | 8195 | 174 | export body at src/helpers.ts:23 body 24 |  |  | 0.675 |
| walker |  | 8264 | 69 | imports in src/use-scale-background.ts |  |  | 0.675 |
| ns | 8280 |  | 462 | helpers.ts — set()/reset()/isVertical() (full) | 4.5 | 4.3 | 0.689 |
| walker |  | 8336 | 72 | imports in src/use-snap-points.ts |  |  | 0.689 |
| walker |  | 8412 | 76 | README headline in test/README.md |  |  | 0.689 |
| walker |  | 8435 | 23 | headings outline in test/README.md |  |  | 0.689 |
| ns | 8593 |  | 313 | use-prevent-scroll.ts — Mobile Safari workaround rationale (comment) + isInput() | 4.6 | 4.3 | 0.679 |
| ns | 8919 |  | 326 | use-position-fixed.ts — rationale comment + signature | 4.7 | 4.3 | 0.670 |
| ns | 9033 |  | 114 | style.css — shared [data-vaul-drawer] base rule | 5.1 |  | 0.667 |
| ns | 9354 |  | 321 | test/src/app/page.tsx — demo index (full) | 6.1 |  | 0.657 |
| walker |  | 9536 | 1101 | export body at src/index.tsx:996 body 1000 |  |  | 0.675 |
| walker |  | 9565 | 29 | test names surface in test/tests/with-redirect.spec.ts |  |  | 0.675 |
| walker |  | 9594 | 29 | test names surface in test/tests/without-scaled-background.spec.ts |  |  | 0.675 |
| walker |  | 9624 | 30 | test names surface in test/tests/nested.spec.ts |  |  | 0.675 |
| walker |  | 9656 | 32 | test names surface in test/tests/with-handle.spec.ts |  |  | 0.675 |
| walker |  | 9689 | 33 | test names surface in test/tests/initial-snap.spec.ts |  |  | 0.675 |
| ns | 9727 |  | 373 | Per-demo distinguishing Drawer.Root/Handle line, one per test/src/app/*/page.tsx route | 6.2 |  | 0.668 |
| ns | 9906 |  | 179 | playwright.config.ts — webServer + device projects | 7.1 |  | 0.674 |
| walker |  | 9969 | 280 | declaration surface of src/style.css |  |  | 0.674 |
| ns | 9984 |  | 78 | CI workflow — install/build/test steps | 7.2 |  | 0.672 |

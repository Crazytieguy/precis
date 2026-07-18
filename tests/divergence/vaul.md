Score(3000)=0.835 I=0.894 C=0.780 ns_rows≤3K=17/42 (reached=12 partial=0 missing=5)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 62 | 62 | listing of '.' |  |  | 1.000 |
| ns | 62 |  | 62 | Fixture root listing | 1.1 |  | 1.000 |
| ns | 107 |  | 45 | package.json — name/version/description | 1.2 |  | 0.896 |
| walker |  | 120 | 58 | package identity in package.json |  |  | 1.000 |
| walker |  | 141 | 21 | package runtime metadata in package.json |  |  | 1.000 |
| walker |  | 144 | 3 | listing of '.github' |  |  | 1.000 |
| walker |  | 148 | 4 | listing of '.github/workflows' |  |  | 1.000 |
| walker |  | 152 | 4 | listing of '.vscode' |  |  | 1.000 |
| ns | 175 |  | 68 | README.md (full) | 1.3 |  | 0.911 |
| walker |  | 223 | 71 | listing of 'src' |  |  | 0.929 |
| walker |  | 240 | 17 | module item at src/index.tsx:993 |  |  | 0.929 |
| ns | 246 |  | 71 | src/ directory listing | 1.4 |  | 0.940 |
| walker |  | 253 | 13 | module item at src/index.tsx:994 |  |  | 0.940 |
| walker |  | 266 | 13 | export names surface in playwright.config.ts |  |  | 0.940 |
| ns | 383 |  | 137 | package.json — scripts | 1.5 |  | 0.816 |
| walker |  | 403 | 137 | package scripts in package.json |  |  | 0.950 |
| walker |  | 426 | 23 | plaintext config pnpm-workspace.yaml |  |  | 0.950 |
| ns | 435 |  | 52 | test/ workspace top-level listing | 1.6 |  | 0.829 |
| ns | 534 |  | 99 | test/src/app/ demo route listing | 1.7 |  | 0.693 |
| ns | 600 |  | 66 | test/tests/ Playwright spec listing | 1.8 |  | 0.637 |
| walker |  | 650 | 224 | export names surface in src/index.tsx |  |  | 0.644 |
| walker |  | 650 | 0 | export at src/index.tsx:1098 |  |  | 0.644 |
| walker |  | 650 | 0 | export at src/index.tsx:1130 |  |  | 0.644 |
| walker |  | 663 | 13 | export at src/index.tsx:989 |  |  | 0.644 |
| walker |  | 676 | 13 | export at src/index.tsx:803 |  |  | 0.644 |
| walker |  | 705 | 29 | export at src/index.tsx:996 |  |  | 0.644 |
| walker |  | 739 | 34 | export at src/index.tsx:833 |  |  | 0.644 |
| ns | 772 |  | 172 | package.json — entry points + files | 1.9 |  | 0.578 |
| walker |  | 859 | 120 | export at src/index.tsx:40 |  |  | 0.580 |
| ns | 983 |  | 211 | package.json — devDependencies | 1.10 |  | 0.537 |
| walker |  | 1020 | 161 | export at src/index.tsx:27 |  |  | 0.543 |
| walker |  | 1121 | 101 | export at src/index.tsx:1137 |  |  | 0.549 |
| ns | 1134 |  | 151 | package.json — peerDependencies + runtime dependency | 1.11 |  | 0.526 |
| ns | 1249 |  | 115 | Drawer export object (full public API surface) | 2.1 |  | 0.566 |
| ns | 1415 |  | 166 | index.tsx top-level export locations | 2.2 |  | 0.590 |
| walker |  | 1439 | 318 | export at src/index.tsx:139 |  |  | 0.597 |
| walker |  | 1453 | 14 | export names surface in src/use-position-fixed.ts |  |  | 0.597 |
| walker |  | 1467 | 14 | export names surface in src/use-snap-points.ts |  |  | 0.598 |
| walker |  | 1482 | 15 | export names surface in src/use-scale-background.ts |  |  | 0.598 |
| walker |  | 1482 | 0 | export at src/use-scale-background.ts:8 |  |  | 0.598 |
| walker |  | 1534 | 52 | listing of 'test' |  |  | 0.662 |
| walker |  | 1537 | 3 | listing of 'test/src' |  |  | 0.662 |
| walker |  | 1553 | 16 | export names surface in src/use-composed-refs.ts |  |  | 0.662 |
| ns | 1701 |  | 286 | WithFadeFromProps / WithoutFadeFromProps (full) | 2.3 | 2.2 | 0.691 |
| walker |  | 1725 | 172 | package entrypoints in package.json |  |  | 0.764 |
| walker |  | 1759 | 34 | package runtime dependencies in package.json |  |  | 0.769 |
| ns | 2243 |  | 542 | DialogProps — core open/behavior props (open .. dismissible) | 2.4 | 2.2 | 0.676 |
| ns | 2441 |  | 198 | DialogProps — drag/modal/direction props (onDrag .. direction) | 2.5 | 2.2 | 0.652 |
| ns | 2871 |  | 430 | DialogProps — snap/animation/misc props (defaultOpen .. autoFocus) | 2.6 | 2.2 | 0.603 |
| walker |  | 2929 | 1170 | export at src/index.tsx:50 |  |  | 0.835 |
| walker |  | 3105 | 176 | package identity metadata in package.json |  |  | 0.835 |
| ns | 3189 |  | 318 | Root() destructured props + defaults | 2.7 | 2.2 | 0.840 |
| walker |  | 3216 | 111 | json config tsconfig.json |  |  | 0.840 |
| walker |  | 3252 | 36 | export names surface in src/context.ts |  |  | 0.840 |
| walker |  | 3252 | 0 | export at src/context.ts:69 |  |  | 0.840 |
| walker |  | 3351 | 99 | listing of 'test/src/app' |  |  | 0.897 |
| walker |  | 3368 | 17 | imports in playwright.config.ts |  |  | 0.897 |
| walker |  | 3405 | 37 | export names surface in src/use-controllable-state.ts |  |  | 0.897 |
| walker |  | 3405 | 0 | export at src/use-controllable-state.ts:39 |  |  | 0.897 |
| walker |  | 3471 | 66 | listing of 'test/tests' |  |  | 0.927 |
| ns | 3558 |  | 369 | Overlay component (full) | 2.8 | 2.2 | 0.881 |
| walker |  | 3785 | 314 | imports in src/index.tsx |  |  | 0.881 |
| walker |  | 3797 | 12 | imports in src/use-controllable-state.ts |  |  | 0.881 |
| walker |  | 3811 | 14 | imports in src/use-composed-refs.ts |  |  | 0.881 |
| walker |  | 3820 | 9 | listing of 'test/public' |  |  | 0.881 |
| ns | 3902 |  | 344 | ContentProps type + Content()'s destructured context fields | 2.9 | 2.2 | 0.837 |
| walker |  | 3944 | 124 | export at src/use-position-fixed.ts:15 |  |  | 0.837 |
| walker |  | 4003 | 59 | export body at src/context.ts:69 body 70 |  |  | 0.837 |
| ns | 4147 |  | 245 | HandleProps type + Handle()'s destructured context fields | 2.10 | 2.2 | 0.810 |
| ns | 4466 |  | 319 | NestedRoot() (full) | 2.11 | 2.2 | 0.776 |
| walker |  | 4515 | 512 | export at playwright.config.ts:12 |  |  | 0.778 |
| walker |  | 4541 | 26 | export doc at playwright.config.ts:12 |  |  | 0.778 |
| ns | 4547 |  | 81 | Portal component (full) | 2.12 | 2.2 | 0.769 |
| walker |  | 4559 | 18 | imports in src/helpers.ts |  |  | 0.769 |
| walker |  | 4670 | 111 | json config turbo.json |  |  | 0.758 |
| ns | 4670 |  | 123 | Content()'s rendered JSX — pointer/focus handler + isDeltaInDirection locations | 2.13 | 2.9 | 0.758 |
| walker |  | 4767 | 97 | export names surface in src/use-prevent-scroll.ts |  |  | 0.759 |
| walker |  | 4767 | 0 | export at src/use-prevent-scroll.ts:29 |  |  | 0.759 |
| walker |  | 4767 | 0 | export at src/use-prevent-scroll.ts:34 |  |  | 0.759 |
| walker |  | 4767 | 0 | export at src/use-prevent-scroll.ts:68 |  |  | 0.759 |
| walker |  | 4767 | 0 | export at src/use-prevent-scroll.ts:294 |  |  | 0.759 |
| walker |  | 4808 | 41 | export body at src/use-prevent-scroll.ts:29 body 30 |  |  | 0.759 |
| ns | 4838 |  | 168 | Root() internal handler locations | 3.1 | 2.7 | 0.746 |
| walker |  | 4870 | 62 | export body at src/use-prevent-scroll.ts:294 body 295 |  |  | 0.746 |
| ns | 5110 |  | 272 | onPress() (full) | 3.2 | 3.1 | 0.728 |
| walker |  | 5187 | 317 | export body at src/index.tsx:803 body 805 |  |  | 0.762 |
| walker |  | 5279 | 92 | export body at src/use-prevent-scroll.ts:34 body 35 |  |  | 0.762 |
| walker |  | 5302 | 23 | imports in src/context.ts |  |  | 0.762 |
| walker |  | 5325 | 23 | imports in src/use-position-fixed.ts |  |  | 0.762 |
| walker |  | 5437 | 112 | export names surface in src/browser.ts |  |  | 0.762 |
| walker |  | 5437 | 0 | export at src/browser.ts:1 |  |  | 0.762 |
| walker |  | 5437 | 0 | export at src/browser.ts:10 |  |  | 0.762 |
| walker |  | 5437 | 0 | export at src/browser.ts:14 |  |  | 0.762 |
| walker |  | 5437 | 0 | export at src/browser.ts:18 |  |  | 0.762 |
| walker |  | 5437 | 0 | export at src/browser.ts:22 |  |  | 0.762 |
| walker |  | 5437 | 0 | export at src/browser.ts:30 |  |  | 0.762 |
| walker |  | 5437 | 0 | export at src/browser.ts:34 |  |  | 0.762 |
| walker |  | 5449 | 12 | export body at src/browser.ts:10 body 11 |  |  | 0.762 |
| walker |  | 5462 | 13 | export body at src/browser.ts:14 body 15 |  |  | 0.762 |
| walker |  | 5485 | 23 | export body at src/browser.ts:18 body 19 |  |  | 0.762 |
| walker |  | 5568 | 83 | export body at src/browser.ts:1 body 2 |  |  | 0.762 |
| walker |  | 5870 | 302 | package dev/peer dependencies in package.json |  |  | 0.801 |
| ns | 5941 |  | 831 | shouldDrag() (full) — the gesture-permission gate | 3.3 | 3.1 | 0.733 |
| walker |  | 6014 | 144 | export names surface in src/constants.ts |  |  | 0.734 |
| walker |  | 6054 | 40 | export at src/constants.ts:1 |  |  | 0.735 |
| walker |  | 6086 | 32 | imports in src/use-prevent-scroll.ts |  |  | 0.735 |
| ns | 6332 |  | 391 | closeDrawer() / cancelDrag() (full) + resetDrawer() signature and transform reset | 3.4 | 3.1 | 0.705 |
| walker |  | 6335 | 249 | export at src/use-snap-points.ts:7 |  |  | 0.705 |
| walker |  | 6506 | 171 | export names surface in src/helpers.ts |  |  | 0.706 |
| walker |  | 6506 | 0 | export at src/helpers.ts:9 |  |  | 0.706 |
| walker |  | 6506 | 0 | export at src/helpers.ts:23 |  |  | 0.706 |
| walker |  | 6506 | 0 | export at src/helpers.ts:42 |  |  | 0.706 |
| walker |  | 6506 | 0 | export at src/helpers.ts:59 |  |  | 0.706 |
| walker |  | 6506 | 0 | export at src/helpers.ts:72 |  |  | 0.706 |
| walker |  | 6506 | 0 | export at src/helpers.ts:90 |  |  | 0.706 |
| walker |  | 6506 | 0 | export at src/helpers.ts:94 |  |  | 0.706 |
| walker |  | 6506 | 0 | export at src/helpers.ts:108 |  |  | 0.706 |
| walker |  | 6535 | 29 | export doc at src/helpers.ts:108 |  |  | 0.706 |
| ns | 6580 |  | 248 | onRelease() — velocity/closeThreshold close-or-reset decision | 3.5 | 3.1 | 0.692 |
| walker |  | 6620 | 85 | export body at src/helpers.ts:59 body 60 |  |  | 0.692 |
| ns | 6712 |  | 132 | Root() small effects — enter-animation flag + non-modal pointer-events | 3.6 |  | 0.684 |
| walker |  | 6736 | 116 | export body at src/helpers.ts:9 body 10 |  |  | 0.684 |
| walker |  | 6882 | 146 | export body at src/helpers.ts:42 body 43 |  |  | 0.685 |
| ns | 6901 |  | 189 | constants.ts (full) | 4.1 |  | 0.691 |
| ns | 6979 |  | 78 | types.ts (full) | 4.2 |  | 0.687 |
| walker |  | 7069 | 187 | export body at src/use-prevent-scroll.ts:68 body 69 |  |  | 0.687 |
| walker |  | 7127 | 58 | export doc at src/use-prevent-scroll.ts:68 |  |  | 0.687 |
| walker |  | 7454 | 327 | export at src/context.ts:37 |  |  | 0.687 |
| ns | 7551 |  | 572 | Hook/helper function locations across src/ | 4.3 |  | 0.675 |
| walker |  | 7711 | 257 | export body at src/use-controllable-state.ts:39 body 40 |  |  | 0.675 |
| ns | 7818 |  | 267 | DrawerContextValue — internal-only fields (refs, callbacks, imperative state) | 4.4 | 4.3 | 0.666 |
| walker |  | 7885 | 174 | export body at src/helpers.ts:23 body 24 |  |  | 0.668 |
| walker |  | 7954 | 69 | imports in src/use-scale-background.ts |  |  | 0.668 |
| walker |  | 8026 | 72 | imports in src/use-snap-points.ts |  |  | 0.668 |
| walker |  | 8102 | 76 | README headline in test/README.md |  |  | 0.668 |
| walker |  | 8125 | 23 | headings outline in test/README.md |  |  | 0.668 |
| ns | 8280 |  | 462 | helpers.ts — set()/reset()/isVertical() (full) | 4.5 | 4.3 | 0.682 |
| ns | 8593 |  | 313 | use-prevent-scroll.ts — Mobile Safari workaround rationale (comment) + isInput() | 4.6 | 4.3 | 0.672 |
| ns | 8919 |  | 326 | use-position-fixed.ts — rationale comment + signature | 4.7 | 4.3 | 0.664 |
| ns | 9033 |  | 114 | style.css — shared [data-vaul-drawer] base rule | 5.1 |  | 0.660 |
| walker |  | 9226 | 1101 | export body at src/index.tsx:996 body 1000 |  |  | 0.679 |
| ns | 9354 |  | 321 | test/src/app/page.tsx — demo index (full) | 6.1 |  | 0.669 |
| walker |  | 9373 | 147 | export doc at src/use-position-fixed.ts:15 |  |  | 0.683 |
| ns | 9727 |  | 373 | Per-demo distinguishing Drawer.Root/Handle line, one per test/src/app/*/page.tsx route | 6.2 |  | 0.675 |
| ns | 9906 |  | 179 | playwright.config.ts — webServer + device projects | 7.1 |  | 0.681 |
| ns | 9984 |  | 78 | CI workflow — install/build/test steps | 7.2 |  | 0.679 |

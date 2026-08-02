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
| ns | 405 |  | 68 | README: the unmaintained notice (whole file) | 1.5 |  | 0.616 |
| walker |  | 575 | 224 | export names surface in src/index.tsx |  |  | 0.637 |
| walker |  | 575 | 0 | export at src/index.tsx:1098 |  |  | 0.637 |
| walker |  | 575 | 0 | export at src/index.tsx:1130 |  |  | 0.637 |
| walker |  | 588 | 13 | export at src/index.tsx:989 |  |  | 0.637 |
| ns | 591 |  | 186 | Every top-level export declaration in src/index.tsx (names only) | 1.6 |  | 0.635 |
| walker |  | 601 | 13 | export at src/index.tsx:803 |  |  | 0.635 |
| walker |  | 630 | 29 | export at src/index.tsx:996 |  |  | 0.635 |
| walker |  | 664 | 34 | export at src/index.tsx:833 |  |  | 0.635 |
| ns | 738 |  | 147 | src/index.tsx imports: Radix dialog, style.css, and the first hook modules | 1.7 |  | 0.580 |
| walker |  | 784 | 120 | export at src/index.tsx:40 |  |  | 0.583 |
| ns | 905 |  | 167 | src/index.tsx imports: the constants set and the remaining hook modules | 1.8 | 1.7 | 0.521 |
| walker |  | 945 | 161 | export at src/index.tsx:27 |  |  | 0.531 |
| ns | 1042 |  | 137 | package.json scripts — build, dev, test, format | 1.9 |  | 0.500 |
| walker |  | 1046 | 101 | export at src/index.tsx:1137 |  |  | 0.594 |
| ns | 1098 |  | 56 | Snap-point prop union: the fields of WithFadeFromProps / WithoutFadeFromProps | 2.1 | 1.6 | 0.605 |
| ns | 1260 |  | 162 | DialogProps declarations, first half (lines 51-85), doc comments elided | 2.2 | 1.6 | 0.550 |
| walker |  | 1364 | 318 | export at src/index.tsx:139 |  |  | 0.566 |
| walker |  | 1377 | 13 | export names surface in playwright.config.ts |  |  | 0.566 |
| walker |  | 1431 | 54 | listing of 'test' |  |  | 0.567 |
| walker |  | 1434 | 3 | listing of 'test/src' |  |  | 0.567 |
| walker |  | 1457 | 23 | plaintext config pnpm-workspace.yaml |  |  | 0.567 |
| ns | 1497 |  | 237 | DialogProps declarations, second half (lines 86-137) plus the union tail | 2.3 | 2.2 | 0.509 |
| ns | 1815 |  | 318 | Root's destructured parameter list — every prop's default value | 2.4 | 1.6 | 0.557 |
| ns | 2034 |  | 219 | Snap-point prop documentation (fills 2.1's ellipses) | 2.5 | 2.1 | 0.571 |
| ns | 2201 |  | 167 | Docs for dismissible, modal and direction (fills 2.3's ellipses) | 2.6 | 2.3 | 0.549 |
| ns | 2401 |  | 200 | Docs for closeThreshold, noBodyStyles, setBackgroundColorOnScale, scrollLockTimeout | 2.7 | 2.2 | 0.525 |
| ns | 2493 |  | 92 | Docs for fixed and handleOnly | 2.8 | 2.2 | 0.516 |
| walker |  | 2627 | 1170 | export at src/index.tsx:50 |  |  | 0.741 |
| walker |  | 2641 | 14 | export names surface in src/use-position-fixed.ts |  |  | 0.741 |
| walker |  | 2655 | 14 | export names surface in src/use-snap-points.ts |  |  | 0.741 |
| ns | 2664 |  | 171 | Docs for defaultOpen, disablePreventScroll and repositionInputs | 2.9 | 2.3 | 0.742 |
| walker |  | 2792 | 137 | package scripts in package.json |  |  | 0.768 |
| ns | 2805 |  | 141 | Docs for snapToSequentialPoint and onAnimationEnd — last of the prop docs | 2.10 | 2.3 | 0.768 |
| walker |  | 2807 | 15 | export names surface in src/use-scale-background.ts |  |  | 0.768 |
| walker |  | 2807 | 0 | export at src/use-scale-background.ts:8 |  |  | 0.768 |
| walker |  | 2979 | 172 | package entrypoints in package.json |  |  | 0.912 |
| ns | 2994 |  | 189 | src/constants.ts in full — every tuning constant and its value | 3.1 |  | 0.875 |
| ns | 3072 |  | 78 | src/types.ts in full — DrawerDirection, SnapPoint, AnyFunction | 3.2 |  | 0.862 |
| ns | 3245 |  | 173 | src/helpers.ts — all eight exported helper signatures | 3.3 |  | 0.848 |
| walker |  | 3293 | 314 | imports in src/index.tsx |  |  | 0.921 |
| ns | 3357 |  | 112 | src/browser.ts — the complete platform-detection roster | 3.4 |  | 0.908 |
| walker |  | 3469 | 176 | package identity metadata in package.json |  |  | 0.908 |
| walker |  | 3580 | 111 | json config tsconfig.json |  |  | 0.909 |
| walker |  | 3616 | 36 | export names surface in src/context.ts |  |  | 0.909 |
| walker |  | 3616 | 0 | export at src/context.ts:69 |  |  | 0.909 |
| walker |  | 3633 | 17 | imports in playwright.config.ts |  |  | 0.909 |
| ns | 3636 |  | 279 | DrawerContextValue, first half — refs and pointer callbacks | 3.5 |  | 0.879 |
| walker |  | 3670 | 37 | export names surface in src/use-controllable-state.ts |  |  | 0.879 |
| walker |  | 3670 | 0 | export at src/use-controllable-state.ts:39 |  |  | 0.879 |
| walker |  | 3735 | 65 | listing of 'test/tests' |  |  | 0.880 |
| ns | 3816 |  | 180 | DrawerContextValue, second half — snap points, direction, container | 3.6 | 3.5 | 0.856 |
| walker |  | 3849 | 114 | listing of 'test/src/app' |  |  | 0.858 |
| walker |  | 3907 | 58 | export names surface in src/use-composed-refs.ts |  |  | 0.858 |
| walker |  | 3907 | 0 | export at src/use-composed-refs.ts:23 |  |  | 0.858 |
| walker |  | 3907 | 0 | export at src/use-composed-refs.ts:31 |  |  | 0.858 |
| ns | 3915 |  | 99 | DrawerContext creation and the useDrawerContext accessor | 3.7 | 3.6 | 0.845 |
| walker |  | 3932 | 25 | export body at src/use-composed-refs.ts:23 body 24 |  |  | 0.845 |
| walker |  | 3967 | 35 | export body at src/use-composed-refs.ts:31 body 32 |  |  | 0.845 |
| walker |  | 3979 | 12 | imports in src/use-controllable-state.ts |  |  | 0.845 |
| walker |  | 3993 | 14 | imports in src/use-composed-refs.ts |  |  | 0.845 |
| walker |  | 4001 | 8 | listing of 'test/public' |  |  | 0.845 |
| ns | 4025 |  | 110 | useSnapPoints: entry point and its complete return surface | 3.8 |  | 0.827 |
| walker |  | 4125 | 124 | export at src/use-position-fixed.ts:15 |  |  | 0.827 |
| walker |  | 4184 | 59 | export body at src/context.ts:69 body 70 |  |  | 0.840 |
| ns | 4274 |  | 249 | useSnapPoints parameter object (fills 3.8's ellipsis) | 3.9 | 3.8 | 0.810 |
| ns | 4568 |  | 294 | usePositionFixed: the iOS rationale comment, its signature and return | 3.10 |  | 0.787 |
| walker |  | 4696 | 512 | export at playwright.config.ts:12 |  |  | 0.789 |
| walker |  | 4722 | 26 | export doc at playwright.config.ts:12 |  |  | 0.789 |
| walker |  | 4740 | 18 | imports in src/helpers.ts |  |  | 0.789 |
| ns | 4761 |  | 193 | use-prevent-scroll.ts exported surface and its provenance | 3.11 |  | 0.775 |
| walker |  | 4851 | 111 | json config turbo.json |  |  | 0.776 |
| ns | 4910 |  | 149 | use-prevent-scroll.ts module-private declarations | 3.12 | 3.11 | 0.765 |
| walker |  | 4948 | 97 | export names surface in src/use-prevent-scroll.ts |  |  | 0.769 |
| walker |  | 4948 | 0 | export at src/use-prevent-scroll.ts:29 |  |  | 0.769 |
| walker |  | 4948 | 0 | export at src/use-prevent-scroll.ts:34 |  |  | 0.769 |
| walker |  | 4948 | 0 | export at src/use-prevent-scroll.ts:68 |  |  | 0.769 |
| walker |  | 4948 | 0 | export at src/use-prevent-scroll.ts:294 |  |  | 0.769 |
| walker |  | 4989 | 41 | export body at src/use-prevent-scroll.ts:29 body 30 |  |  | 0.769 |
| walker |  | 5051 | 62 | export body at src/use-prevent-scroll.ts:294 body 295 |  |  | 0.769 |
| ns | 5128 |  | 218 | use-controllable-state.ts — the controlled/uncontrolled prop machinery | 3.13 |  | 0.758 |
| ns | 5322 |  | 194 | useScaleBackground and use-composed-refs — the two smallest modules | 3.14 |  | 0.750 |
| walker |  | 5368 | 317 | export body at src/index.tsx:803 body 805 |  |  | 0.751 |
| walker |  | 5460 | 92 | export body at src/use-prevent-scroll.ts:34 body 35 |  |  | 0.751 |
| walker |  | 5483 | 23 | imports in src/context.ts |  |  | 0.751 |
| walker |  | 5506 | 23 | imports in src/use-position-fixed.ts |  |  | 0.751 |
| ns | 5545 |  | 223 | Every member declared inside Root (names + effect locations) | 4.1 |  | 0.734 |
| walker |  | 5618 | 112 | export names surface in src/browser.ts |  |  | 0.745 |
| walker |  | 5618 | 0 | export at src/browser.ts:1 |  |  | 0.745 |
| walker |  | 5618 | 0 | export at src/browser.ts:10 |  |  | 0.745 |
| walker |  | 5618 | 0 | export at src/browser.ts:14 |  |  | 0.745 |
| walker |  | 5618 | 0 | export at src/browser.ts:18 |  |  | 0.745 |
| walker |  | 5618 | 0 | export at src/browser.ts:22 |  |  | 0.745 |
| walker |  | 5618 | 0 | export at src/browser.ts:30 |  |  | 0.745 |
| walker |  | 5618 | 0 | export at src/browser.ts:34 |  |  | 0.745 |
| walker |  | 5630 | 12 | export body at src/browser.ts:10 body 11 |  |  | 0.745 |
| walker |  | 5643 | 13 | export body at src/browser.ts:14 body 15 |  |  | 0.745 |
| walker |  | 5666 | 23 | export body at src/browser.ts:18 body 19 |  |  | 0.745 |
| walker |  | 5749 | 83 | export body at src/browser.ts:1 body 2 |  |  | 0.745 |
| ns | 5763 |  | 218 | Root's state and refs — the whole drag bookkeeping set | 4.2 |  | 0.729 |
| walker |  | 5786 | 37 | export doc at src/use-composed-refs.ts:23 |  |  | 0.729 |
| walker |  | 5824 | 38 | export doc at src/use-composed-refs.ts:31 |  |  | 0.729 |
| ns | 5870 |  | 107 | Root's useSnapPoints wiring — what it destructures and what it passes | 4.3 |  | 0.719 |
| ns | 6007 |  | 137 | Scroll-lock and body-position wiring, including the isDisabled predicate | 4.4 |  | 0.708 |
| walker |  | 6126 | 302 | package dev/peer dependencies in package.json |  |  | 0.708 |
| ns | 6217 |  | 210 | shouldDrag: the early-out guards | 4.5 | 4.1 | 0.694 |
| walker |  | 6270 | 144 | export names surface in src/constants.ts |  |  | 0.712 |
| walker |  | 6310 | 40 | export at src/constants.ts:1 |  |  | 0.719 |
| walker |  | 6342 | 32 | imports in src/use-prevent-scroll.ts |  |  | 0.719 |
| ns | 6567 |  | 350 | shouldDrag: direction, open-animation window and scroll-lock timeout | 4.6 | 4.5 | 0.694 |
| walker |  | 6591 | 249 | export at src/use-snap-points.ts:7 |  |  | 0.725 |
| walker |  | 6762 | 171 | export names surface in src/helpers.ts |  |  | 0.736 |
| walker |  | 6762 | 0 | export at src/helpers.ts:9 |  |  | 0.736 |
| walker |  | 6762 | 0 | export at src/helpers.ts:23 |  |  | 0.736 |
| walker |  | 6762 | 0 | export at src/helpers.ts:42 |  |  | 0.736 |
| walker |  | 6762 | 0 | export at src/helpers.ts:59 |  |  | 0.736 |
| walker |  | 6762 | 0 | export at src/helpers.ts:72 |  |  | 0.736 |
| walker |  | 6762 | 0 | export at src/helpers.ts:90 |  |  | 0.736 |
| walker |  | 6762 | 0 | export at src/helpers.ts:94 |  |  | 0.736 |
| walker |  | 6762 | 0 | export at src/helpers.ts:108 |  |  | 0.736 |
| walker |  | 6791 | 29 | export doc at src/helpers.ts:108 |  |  | 0.736 |
| ns | 6816 |  | 249 | shouldDrag: the scrollable-ancestor climb | 4.7 | 4.6 | 0.718 |
| walker |  | 6876 | 85 | export body at src/helpers.ts:59 body 60 |  |  | 0.718 |
| walker |  | 6992 | 116 | export body at src/helpers.ts:9 body 10 |  |  | 0.718 |
| ns | 7045 |  | 229 | onRelease: teardown and velocity computation | 4.8 | 4.1 | 0.706 |
| walker |  | 7138 | 146 | export body at src/helpers.ts:42 body 43 |  |  | 0.706 |
| walker |  | 7325 | 187 | export body at src/use-prevent-scroll.ts:68 body 69 |  |  | 0.706 |
| ns | 7358 |  | 313 | onRelease: the close-vs-snap-back decision ladder | 4.9 | 4.8 | 0.689 |
| walker |  | 7383 | 58 | export doc at src/use-prevent-scroll.ts:68 |  |  | 0.689 |
| walker |  | 7630 | 247 | LICENSE.md section #0 |  |  | 0.684 |
| ns | 7630 |  | 272 | The data-vaul-* attributes emitted by Overlay and Content | 4.10 |  | 0.684 |
| ns | 7799 |  | 169 | Handle: click-to-cycle snap points | 4.11 |  | 0.674 |
| ns | 7945 |  | 146 | NestedRoot: how a nested drawer is wired to its parent | 4.12 |  | 0.666 |
| walker |  | 7957 | 327 | export at src/context.ts:37 |  |  | 0.667 |
| ns | 8095 |  | 150 | The base [data-vaul-drawer] rule and the shape of the variant rules | 5.1 |  | 0.662 |
| walker |  | 8214 | 257 | export body at src/use-controllable-state.ts:39 body 40 |  |  | 0.662 |
| ns | 8280 |  | 185 | style.css selector inventory — which attribute combinations are styled | 5.2 | 5.1 | 0.657 |
| walker |  | 8388 | 174 | export body at src/helpers.ts:23 body 24 |  |  | 0.657 |
| ns | 8408 |  | 128 | Every @keyframes name in style.css | 5.3 |  | 0.651 |
| walker |  | 8457 | 69 | imports in src/use-scale-background.ts |  |  | 0.651 |
| ns | 8523 |  | 115 | Complete listing of test/src/app — one demo route per feature | 6.1 |  | 0.660 |
| walker |  | 8529 | 72 | imports in src/use-snap-points.ts |  |  | 0.660 |
| ns | 8589 |  | 66 | Complete listing of test/tests — the Playwright spec set | 6.2 |  | 0.665 |
| walker |  | 8605 | 76 | README headline in test/README.md |  |  | 0.665 |
| walker |  | 8628 | 23 | headings outline in test/README.md |  |  | 0.665 |
| ns | 8642 |  | 53 | Listing of the test/ package root | 6.3 |  | 0.669 |
| ns | 8935 |  | 293 | Spec-suite to demo-route map for every Playwright file | 6.4 |  | 0.660 |
| ns | 9137 |  | 202 | Playwright runner configuration: server, devices, testDir | 6.5 |  | 0.667 |
| ns | 9243 |  | 106 | Shared e2e helpers: openDrawer and ANIMATION_DURATION | 6.6 |  | 0.663 |
| ns | 9374 |  | 131 | package.json publishing surface: files and the exports map | 7.1 |  | 0.668 |
| ns | 9536 |  | 162 | Runtime and peer dependencies, and the pinned package manager | 7.2 |  | 0.672 |
| ns | 9664 |  | 128 | Workspace and task-runner config: pnpm-workspace.yaml and turbo.json | 7.3 |  | 0.677 |
| walker |  | 9729 | 1101 | export body at src/index.tsx:996 body 1000 |  |  | 0.691 |
| ns | 9769 |  | 105 | Root tsconfig.json — compiler settings for the shipped library | 7.4 |  | 0.693 |
| ns | 9773 |  | 4 | Listing of .github/workflows | 7.5 |  | 0.694 |
| ns | 9933 |  | 160 | CI: the Playwright workflow steps | 7.6 |  | 0.688 |
| ns | 9998 |  | 65 | Prettier configuration — the formatting any new code must match | 7.7 |  | 0.684 |

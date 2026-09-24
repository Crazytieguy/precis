Score(3000)=0.877 I=0.934 C=0.824 ns_rows≤3K=20/61 (reached=16 partial=1 missing=3) grid(1000/1442/2080/3000/4327/6240/9000)=0.542/0.567/0.687/0.877/0.811/0.762/0.668

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
| walker |  | 321 | 63 | README.md section #0 |  |  | 0.743 |
| walker |  | 334 | 13 | export names surface in playwright.config.ts |  |  | 0.743 |
| ns | 337 |  | 115 | The `Drawer` namespace object — the entire public component set | 1.4 |  | 0.620 |
| walker |  | 388 | 54 | listing of 'test' |  |  | 0.622 |
| walker |  | 391 | 3 | listing of 'test/src' |  |  | 0.622 |
| ns | 405 |  | 68 | README: the unmaintained notice (whole file) | 1.5 |  | 0.618 |
| walker |  | 414 | 23 | plaintext config pnpm-workspace.yaml |  |  | 0.618 |
| ns | 591 |  | 186 | Every top-level export declaration in src/index.tsx (names only) | 1.6 |  | 0.542 |
| walker |  | 642 | 228 | export names surface in src/index.tsx |  |  | 0.636 |
| walker |  | 642 | 0 | export at src/index.tsx:50 |  |  | 0.636 |
| walker |  | 642 | 0 | export at src/index.tsx:1098 |  |  | 0.636 |
| walker |  | 642 | 0 | export at src/index.tsx:1130 |  |  | 0.636 |
| walker |  | 657 | 15 | export at src/index.tsx:989 |  |  | 0.636 |
| walker |  | 670 | 13 | export at src/index.tsx:803 |  |  | 0.636 |
| walker |  | 699 | 29 | export at src/index.tsx:996 |  |  | 0.636 |
| walker |  | 733 | 34 | export at src/index.tsx:833 |  |  | 0.636 |
| ns | 738 |  | 147 | src/index.tsx imports: Radix dialog, style.css, and the first hook modules | 1.7 |  | 0.582 |
| walker |  | 746 | 13 | module item at src/index.tsx:993 |  |  | 0.582 |
| walker |  | 757 | 11 | module item at src/index.tsx:994 |  |  | 0.582 |
| walker |  | 777 | 20 | module item at src/index.tsx:1128 |  |  | 0.604 |
| walker |  | 897 | 120 | export at src/index.tsx:40 |  |  | 0.607 |
| ns | 905 |  | 167 | src/index.tsx imports: the constants set and the remaining hook modules | 1.8 | 1.7 | 0.542 |
| ns | 1042 |  | 137 | package.json scripts — build, dev, test, format | 1.9 |  | 0.511 |
| walker |  | 1058 | 161 | export at src/index.tsx:27 |  |  | 0.521 |
| ns | 1098 |  | 56 | Snap-point prop union: the fields of WithFadeFromProps / WithoutFadeFromProps | 2.1 | 1.6 | 0.537 |
| walker |  | 1159 | 101 | export at src/index.tsx:1137 |  |  | 0.624 |
| ns | 1260 |  | 162 | DialogProps declarations, first half (lines 51-85), doc comments elided | 2.2 | 1.6 | 0.567 |
| ns | 1497 |  | 237 | DialogProps declarations, second half (lines 86-137) plus the union tail | 2.3 | 2.2 | 0.509 |
| walker |  | 1543 | 384 | export member names at src/index.tsx:50 |  |  | 0.622 |
| ns | 1815 |  | 318 | Root's destructured parameter list — every prop's default value | 2.4 | 1.6 | 0.557 |
| walker |  | 1861 | 318 | export at src/index.tsx:139 |  |  | 0.661 |
| walker |  | 1894 | 33 | export member doc at src/index.tsx:50 member 64 |  |  | 0.665 |
| walker |  | 1936 | 42 | export member doc at src/index.tsx:50 member 71 |  |  | 0.669 |
| walker |  | 1980 | 44 | export member doc at src/index.tsx:50 member 76 |  |  | 0.674 |
| walker |  | 2024 | 44 | export member doc at src/index.tsx:50 member 98 |  |  | 0.678 |
| ns | 2034 |  | 219 | Snap-point prop documentation (fills 2.1's ellipses) | 2.5 | 2.1 | 0.684 |
| walker |  | 2069 | 45 | export member doc at src/index.tsx:50 member 110 |  |  | 0.687 |
| walker |  | 2115 | 46 | export member doc at src/index.tsx:50 member 80 |  |  | 0.691 |
| walker |  | 2161 | 46 | export member doc at src/index.tsx:50 member 85 |  |  | 0.697 |
| ns | 2201 |  | 167 | Docs for dismissible, modal and direction (fills 2.3's ellipses) | 2.6 | 2.3 | 0.673 |
| walker |  | 2208 | 47 | export member doc at src/index.tsx:50 member 105 |  |  | 0.684 |
| walker |  | 2256 | 48 | export member doc at src/index.tsx:50 member 115 |  |  | 0.688 |
| walker |  | 2321 | 65 | export member doc at src/index.tsx:50 member 134 |  |  | 0.692 |
| walker |  | 2397 | 76 | export member doc at src/index.tsx:50 member 91 |  |  | 0.720 |
| ns | 2401 |  | 200 | Docs for closeThreshold, noBodyStyles, setBackgroundColorOnScale, scrollLockTimeout | 2.7 | 2.2 | 0.702 |
| walker |  | 2473 | 76 | export member doc at src/index.tsx:50 member 128 |  |  | 0.708 |
| ns | 2493 |  | 92 | Docs for fixed and handleOnly | 2.8 | 2.2 | 0.710 |
| walker |  | 2551 | 78 | export member doc at src/index.tsx:50 member 121 |  |  | 0.716 |
| walker |  | 2632 | 81 | export member doc at src/index.tsx:50 member 60 |  |  | 0.743 |
| walker |  | 2646 | 14 | export names surface in src/use-position-fixed.ts |  |  | 0.743 |
| walker |  | 2660 | 14 | export names surface in src/use-snap-points.ts |  |  | 0.743 |
| ns | 2664 |  | 171 | Docs for defaultOpen, disablePreventScroll and repositionInputs | 2.9 | 2.3 | 0.744 |
| walker |  | 2797 | 137 | package scripts in package.json |  |  | 0.770 |
| ns | 2805 |  | 141 | Docs for snapToSequentialPoint and onAnimationEnd — last of the prop docs | 2.10 | 2.3 | 0.770 |
| walker |  | 2812 | 15 | export names surface in src/use-scale-background.ts |  |  | 0.770 |
| walker |  | 2812 | 0 | export at src/use-scale-background.ts:8 |  |  | 0.770 |
| walker |  | 2984 | 172 | package entrypoints in package.json |  |  | 0.914 |
| ns | 2994 |  | 189 | src/constants.ts in full — every tuning constant and its value | 3.1 |  | 0.877 |
| ns | 3072 |  | 78 | src/types.ts in full — DrawerDirection, SnapPoint, AnyFunction | 3.2 |  | 0.864 |
| ns | 3245 |  | 173 | src/helpers.ts — all eight exported helper signatures | 3.3 |  | 0.849 |
| walker |  | 3298 | 314 | imports in src/index.tsx |  |  | 0.923 |
| ns | 3357 |  | 112 | src/browser.ts — the complete platform-detection roster | 3.4 |  | 0.909 |
| walker |  | 3474 | 176 | package identity metadata in package.json |  |  | 0.909 |
| walker |  | 3585 | 111 | json config tsconfig.json |  |  | 0.910 |
| walker |  | 3621 | 36 | export names surface in src/context.ts |  |  | 0.911 |
| walker |  | 3621 | 0 | export at src/context.ts:69 |  |  | 0.911 |
| ns | 3636 |  | 279 | DrawerContextValue, first half — refs and pointer callbacks | 3.5 |  | 0.881 |
| walker |  | 3638 | 17 | imports in playwright.config.ts |  |  | 0.881 |
| walker |  | 3675 | 37 | export names surface in src/use-controllable-state.ts |  |  | 0.881 |
| walker |  | 3675 | 0 | export at src/use-controllable-state.ts:39 |  |  | 0.881 |
| walker |  | 3740 | 65 | listing of 'test/tests' |  |  | 0.882 |
| ns | 3816 |  | 180 | DrawerContextValue, second half — snap points, direction, container | 3.6 | 3.5 | 0.858 |
| walker |  | 3854 | 114 | listing of 'test/src/app' |  |  | 0.859 |
| walker |  | 3912 | 58 | export names surface in src/use-composed-refs.ts |  |  | 0.860 |
| walker |  | 3912 | 0 | export at src/use-composed-refs.ts:23 |  |  | 0.860 |
| walker |  | 3912 | 0 | export at src/use-composed-refs.ts:31 |  |  | 0.860 |
| ns | 3915 |  | 99 | DrawerContext creation and the useDrawerContext accessor | 3.7 | 3.6 | 0.847 |
| walker |  | 3937 | 25 | export body at src/use-composed-refs.ts:23 body 24 |  |  | 0.847 |
| walker |  | 3972 | 35 | export body at src/use-composed-refs.ts:31 body 32 |  |  | 0.847 |
| walker |  | 3984 | 12 | imports in src/use-controllable-state.ts |  |  | 0.847 |
| walker |  | 3998 | 14 | imports in src/use-composed-refs.ts |  |  | 0.847 |
| walker |  | 4006 | 8 | listing of 'test/public' |  |  | 0.847 |
| ns | 4025 |  | 110 | useSnapPoints: entry point and its complete return surface | 3.8 |  | 0.828 |
| walker |  | 4130 | 124 | export at src/use-position-fixed.ts:15 |  |  | 0.829 |
| walker |  | 4189 | 59 | export body at src/context.ts:69 body 70 |  |  | 0.841 |
| ns | 4274 |  | 249 | useSnapPoints parameter object (fills 3.8's ellipsis) | 3.9 | 3.8 | 0.811 |
| ns | 4568 |  | 294 | usePositionFixed: the iOS rationale comment, its signature and return | 3.10 |  | 0.788 |
| walker |  | 4701 | 512 | export at playwright.config.ts:12 |  |  | 0.790 |
| walker |  | 4727 | 26 | export doc at playwright.config.ts:12 |  |  | 0.790 |
| walker |  | 4745 | 18 | imports in src/helpers.ts |  |  | 0.790 |
| ns | 4761 |  | 193 | use-prevent-scroll.ts exported surface and its provenance | 3.11 |  | 0.776 |
| walker |  | 4856 | 111 | json config turbo.json |  |  | 0.777 |
| ns | 4910 |  | 149 | use-prevent-scroll.ts module-private declarations | 3.12 | 3.11 | 0.766 |
| walker |  | 4953 | 97 | export names surface in src/use-prevent-scroll.ts |  |  | 0.770 |
| walker |  | 4953 | 0 | export at src/use-prevent-scroll.ts:29 |  |  | 0.770 |
| walker |  | 4953 | 0 | export at src/use-prevent-scroll.ts:34 |  |  | 0.770 |
| walker |  | 4953 | 0 | export at src/use-prevent-scroll.ts:68 |  |  | 0.770 |
| walker |  | 4953 | 0 | export at src/use-prevent-scroll.ts:294 |  |  | 0.770 |
| walker |  | 4994 | 41 | export body at src/use-prevent-scroll.ts:29 body 30 |  |  | 0.770 |
| walker |  | 5056 | 62 | export body at src/use-prevent-scroll.ts:294 body 295 |  |  | 0.770 |
| ns | 5128 |  | 218 | use-controllable-state.ts — the controlled/uncontrolled prop machinery | 3.13 |  | 0.759 |
| walker |  | 5148 | 92 | export body at src/use-prevent-scroll.ts:34 body 35 |  |  | 0.759 |
| walker |  | 5171 | 23 | imports in src/context.ts |  |  | 0.759 |
| walker |  | 5194 | 23 | imports in src/use-position-fixed.ts |  |  | 0.759 |
| walker |  | 5306 | 112 | export names surface in src/browser.ts |  |  | 0.771 |
| walker |  | 5306 | 0 | export at src/browser.ts:1 |  |  | 0.771 |
| walker |  | 5306 | 0 | export at src/browser.ts:10 |  |  | 0.771 |
| walker |  | 5306 | 0 | export at src/browser.ts:14 |  |  | 0.771 |
| walker |  | 5306 | 0 | export at src/browser.ts:18 |  |  | 0.771 |
| walker |  | 5306 | 0 | export at src/browser.ts:22 |  |  | 0.771 |
| walker |  | 5306 | 0 | export at src/browser.ts:30 |  |  | 0.771 |
| walker |  | 5306 | 0 | export at src/browser.ts:34 |  |  | 0.771 |
| walker |  | 5318 | 12 | export body at src/browser.ts:10 body 11 |  |  | 0.771 |
| ns | 5322 |  | 194 | useScaleBackground and use-composed-refs — the two smallest modules | 3.14 |  | 0.763 |
| walker |  | 5331 | 13 | export body at src/browser.ts:14 body 15 |  |  | 0.763 |
| walker |  | 5354 | 23 | export body at src/browser.ts:18 body 19 |  |  | 0.763 |
| walker |  | 5437 | 83 | export body at src/browser.ts:1 body 2 |  |  | 0.763 |
| walker |  | 5474 | 37 | export doc at src/use-composed-refs.ts:23 |  |  | 0.763 |
| walker |  | 5512 | 38 | export doc at src/use-composed-refs.ts:31 |  |  | 0.763 |
| ns | 5545 |  | 223 | Every member declared inside Root (names + effect locations) | 4.1 |  | 0.746 |
| walker |  | 5656 | 144 | export names surface in src/constants.ts |  |  | 0.766 |
| walker |  | 5696 | 40 | export at src/constants.ts:1 |  |  | 0.774 |
| walker |  | 5728 | 32 | imports in src/use-prevent-scroll.ts |  |  | 0.774 |
| ns | 5763 |  | 218 | Root's state and refs — the whole drag bookkeeping set | 4.2 |  | 0.756 |
| ns | 5870 |  | 107 | Root's useSnapPoints wiring — what it destructures and what it passes | 4.3 |  | 0.746 |
| walker |  | 5977 | 249 | export at src/use-snap-points.ts:7 |  |  | 0.779 |
| ns | 6007 |  | 137 | Scroll-lock and body-position wiring, including the isDisabled predicate | 4.4 |  | 0.767 |
| walker |  | 6148 | 171 | export names surface in src/helpers.ts |  |  | 0.779 |
| walker |  | 6148 | 0 | export at src/helpers.ts:9 |  |  | 0.779 |
| walker |  | 6148 | 0 | export at src/helpers.ts:23 |  |  | 0.779 |
| walker |  | 6148 | 0 | export at src/helpers.ts:42 |  |  | 0.779 |
| walker |  | 6148 | 0 | export at src/helpers.ts:59 |  |  | 0.779 |
| walker |  | 6148 | 0 | export at src/helpers.ts:72 |  |  | 0.779 |
| walker |  | 6148 | 0 | export at src/helpers.ts:90 |  |  | 0.779 |
| walker |  | 6148 | 0 | export at src/helpers.ts:94 |  |  | 0.779 |
| walker |  | 6148 | 0 | export at src/helpers.ts:108 |  |  | 0.779 |
| walker |  | 6177 | 29 | export doc at src/helpers.ts:108 |  |  | 0.779 |
| ns | 6217 |  | 210 | shouldDrag: the early-out guards | 4.5 | 4.1 | 0.762 |
| walker |  | 6262 | 85 | export body at src/helpers.ts:59 body 60 |  |  | 0.762 |
| walker |  | 6378 | 116 | export body at src/helpers.ts:9 body 10 |  |  | 0.762 |
| walker |  | 6524 | 146 | export body at src/helpers.ts:42 body 43 |  |  | 0.762 |
| ns | 6567 |  | 350 | shouldDrag: direction, open-animation window and scroll-lock timeout | 4.6 | 4.5 | 0.736 |
| ns | 6816 |  | 249 | shouldDrag: the scrollable-ancestor climb | 4.7 | 4.6 | 0.718 |
| walker |  | 6826 | 302 | package dev/peer dependencies in package.json |  |  | 0.719 |
| walker |  | 7013 | 187 | export body at src/use-prevent-scroll.ts:68 body 69 |  |  | 0.719 |
| ns | 7045 |  | 229 | onRelease: teardown and velocity computation | 4.8 | 4.1 | 0.707 |
| walker |  | 7071 | 58 | export doc at src/use-prevent-scroll.ts:68 |  |  | 0.707 |
| walker |  | 7318 | 247 | LICENSE.md section #0 |  |  | 0.707 |
| ns | 7358 |  | 313 | onRelease: the close-vs-snap-back decision ladder | 4.9 | 4.8 | 0.690 |
| ns | 7630 |  | 272 | The data-vaul-* attributes emitted by Overlay and Content | 4.10 |  | 0.680 |
| walker |  | 7645 | 327 | export at src/context.ts:37 |  |  | 0.681 |
| ns | 7799 |  | 169 | Handle: click-to-cycle snap points | 4.11 |  | 0.671 |
| walker |  | 7902 | 257 | export body at src/use-controllable-state.ts:39 body 40 |  |  | 0.671 |
| ns | 7945 |  | 146 | NestedRoot: how a nested drawer is wired to its parent | 4.12 |  | 0.663 |
| walker |  | 8076 | 174 | export body at src/helpers.ts:23 body 24 |  |  | 0.663 |
| ns | 8095 |  | 150 | The base [data-vaul-drawer] rule and the shape of the variant rules | 5.1 |  | 0.658 |
| walker |  | 8145 | 69 | imports in src/use-scale-background.ts |  |  | 0.658 |
| walker |  | 8217 | 72 | imports in src/use-snap-points.ts |  |  | 0.658 |
| ns | 8280 |  | 185 | style.css selector inventory — which attribute combinations are styled | 5.2 | 5.1 | 0.653 |
| walker |  | 8293 | 76 | README headline in test/README.md |  |  | 0.653 |
| walker |  | 8316 | 23 | headings outline in test/README.md |  |  | 0.653 |
| ns | 8408 |  | 128 | Every @keyframes name in style.css | 5.3 |  | 0.647 |
| ns | 8523 |  | 115 | Complete listing of test/src/app — one demo route per feature | 6.1 |  | 0.656 |
| ns | 8589 |  | 66 | Complete listing of test/tests — the Playwright spec set | 6.2 |  | 0.661 |
| walker |  | 8596 | 280 | declaration surface of src/style.css |  |  | 0.662 |
| ns | 8642 |  | 53 | Listing of the test/ package root | 6.3 |  | 0.666 |
| walker |  | 8743 | 147 | export doc at src/use-position-fixed.ts:15 |  |  | 0.677 |
| ns | 8935 |  | 293 | Spec-suite to demo-route map for every Playwright file | 6.4 |  | 0.668 |
| ns | 9137 |  | 202 | Playwright runner configuration: server, devices, testDir | 6.5 |  | 0.675 |
| ns | 9243 |  | 106 | Shared e2e helpers: openDrawer and ANIMATION_DURATION | 6.6 |  | 0.671 |
| ns | 9374 |  | 131 | package.json publishing surface: files and the exports map | 7.1 |  | 0.676 |
| walker |  | 9408 | 665 | export body at src/use-scale-background.ts:8 body 9 |  |  | 0.680 |
| walker |  | 9460 | 52 | export names surface in src/types.ts |  |  | 0.683 |
| walker |  | 9481 | 21 | export at src/types.ts:2 |  |  | 0.686 |
| ns | 9536 |  | 162 | Runtime and peer dependencies, and the pinned package manager | 7.2 |  | 0.690 |
| ns | 9664 |  | 128 | Workspace and task-runner config: pnpm-workspace.yaml and turbo.json | 7.3 |  | 0.694 |
| ns | 9769 |  | 105 | Root tsconfig.json — compiler settings for the shipped library | 7.4 |  | 0.697 |
| ns | 9773 |  | 4 | Listing of .github/workflows | 7.5 |  | 0.697 |
| walker |  | 9798 | 317 | export body at src/index.tsx:803 body 805 |  |  | 0.701 |
| ns | 9933 |  | 160 | CI: the Playwright workflow steps | 7.6 |  | 0.695 |
| ns | 9998 |  | 65 | Prettier configuration — the formatting any new code must match | 7.7 |  | 0.692 |

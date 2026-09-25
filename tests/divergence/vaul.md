Score(3000)=0.877 I=0.934 C=0.824 ns_rows≤3K=20/61 grid(1000/1442/2080/3000/4327/6240/9000)=0.542/0.567/0.687/0.877/0.811/0.762/0.668

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 62 | 62 | listing of '.' |  |  | 0.000 |
| walker |  | 65 | 3 | listing of '.github' |  |  | 0.000 |
| walker |  | 69 | 4 | listing of '.github/workflows' |  |  | 0.000 |
| walker |  | 73 | 4 | listing of '.vscode' |  |  | 0.000 |
| ns | 86 |  | 86 | Package identity: name, version, description, entry points | 1.1 |  | 0.000 |
| walker |  | 131 | 58 | package identity in package.json |  |  | 0.460 |
| ns | 148 |  | 62 | Complete repository root listing | 1.2 |  | 0.677 |
| walker |  | 202 | 71 | listing of 'src' |  |  | 0.746 |
| ns | 219 |  | 71 | Complete src/ listing — the shipped library | 1.3 |  | 0.737 |
| walker |  | 238 | 36 | package runtime dependencies in package.json |  |  | 0.737 |
| walker |  | 257 | 19 | package runtime metadata in package.json |  |  | 0.737 |
| walker |  | 320 | 63 | README.md section #0 |  |  | 0.743 |
| walker |  | 333 | 13 | export names surface in playwright.config.ts |  |  | 0.743 |
| ns | 334 |  | 115 | The `Drawer` namespace object — the entire public component set | 1.4 |  | 0.620 |
| walker |  | 385 | 52 | listing of 'test' |  |  | 0.622 |
| walker |  | 388 | 3 | listing of 'test/src' |  |  | 0.622 |
| ns | 402 |  | 68 | README: the unmaintained notice (whole file) | 1.5 |  | 0.618 |
| walker |  | 411 | 23 | plaintext config pnpm-workspace.yaml |  |  | 0.618 |
| ns | 588 |  | 186 | Every top-level export declaration in src/index.tsx (names only) | 1.6 |  | 0.542 |
| walker |  | 639 | 228 | export names surface in src/index.tsx |  |  | 0.636 |
| walker |  | 639 | 0 | export at src/index.tsx:50 |  |  | 0.636 |
| walker |  | 639 | 0 | export at src/index.tsx:1098 |  |  | 0.636 |
| walker |  | 639 | 0 | export at src/index.tsx:1130 |  |  | 0.636 |
| walker |  | 654 | 15 | export at src/index.tsx:989 |  |  | 0.636 |
| walker |  | 667 | 13 | export at src/index.tsx:803 |  |  | 0.636 |
| walker |  | 696 | 29 | export at src/index.tsx:996 |  |  | 0.636 |
| walker |  | 730 | 34 | export at src/index.tsx:833 |  |  | 0.636 |
| ns | 735 |  | 147 | src/index.tsx imports: Radix dialog, style.css, and the first hook modules | 1.7 |  | 0.582 |
| walker |  | 743 | 13 | module item at src/index.tsx:993 |  |  | 0.582 |
| walker |  | 754 | 11 | module item at src/index.tsx:994 |  |  | 0.582 |
| walker |  | 774 | 20 | module item at src/index.tsx:1128 |  |  | 0.604 |
| walker |  | 894 | 120 | export at src/index.tsx:40 |  |  | 0.607 |
| ns | 902 |  | 167 | src/index.tsx imports: the constants set and the remaining hook modules | 1.8 | 1.7 | 0.542 |
| ns | 1039 |  | 137 | package.json scripts — build, dev, test, format | 1.9 |  | 0.511 |
| walker |  | 1055 | 161 | export at src/index.tsx:27 |  |  | 0.521 |
| ns | 1095 |  | 56 | Snap-point prop union: the fields of WithFadeFromProps / WithoutFadeFromProps | 2.1 | 1.6 | 0.537 |
| walker |  | 1156 | 101 | export at src/index.tsx:1137 |  |  | 0.624 |
| ns | 1257 |  | 162 | DialogProps declarations, first half (lines 51-85), doc comments elided | 2.2 | 1.6 | 0.567 |
| ns | 1494 |  | 237 | DialogProps declarations, second half (lines 86-137) plus the union tail | 2.3 | 2.2 | 0.509 |
| walker |  | 1540 | 384 | export member names at src/index.tsx:50 |  |  | 0.622 |
| ns | 1812 |  | 318 | Root's destructured parameter list — every prop's default value | 2.4 | 1.6 | 0.557 |
| walker |  | 1858 | 318 | export at src/index.tsx:139 |  |  | 0.661 |
| walker |  | 1891 | 33 | export member doc at src/index.tsx:50 member 64 |  |  | 0.665 |
| walker |  | 1933 | 42 | export member doc at src/index.tsx:50 member 71 |  |  | 0.669 |
| walker |  | 1977 | 44 | export member doc at src/index.tsx:50 member 76 |  |  | 0.674 |
| walker |  | 2021 | 44 | export member doc at src/index.tsx:50 member 98 |  |  | 0.678 |
| ns | 2031 |  | 219 | Snap-point prop documentation (fills 2.1's ellipses) | 2.5 | 2.1 | 0.684 |
| walker |  | 2066 | 45 | export member doc at src/index.tsx:50 member 110 |  |  | 0.687 |
| walker |  | 2112 | 46 | export member doc at src/index.tsx:50 member 80 |  |  | 0.691 |
| walker |  | 2158 | 46 | export member doc at src/index.tsx:50 member 85 |  |  | 0.697 |
| ns | 2198 |  | 167 | Docs for dismissible, modal and direction (fills 2.3's ellipses) | 2.6 | 2.3 | 0.673 |
| walker |  | 2205 | 47 | export member doc at src/index.tsx:50 member 105 |  |  | 0.684 |
| walker |  | 2253 | 48 | export member doc at src/index.tsx:50 member 115 |  |  | 0.688 |
| walker |  | 2318 | 65 | export member doc at src/index.tsx:50 member 134 |  |  | 0.692 |
| walker |  | 2394 | 76 | export member doc at src/index.tsx:50 member 91 |  |  | 0.720 |
| ns | 2398 |  | 200 | Docs for closeThreshold, noBodyStyles, setBackgroundColorOnScale, scrollLockTimeout | 2.7 | 2.2 | 0.702 |
| walker |  | 2470 | 76 | export member doc at src/index.tsx:50 member 128 |  |  | 0.708 |
| ns | 2490 |  | 92 | Docs for fixed and handleOnly | 2.8 | 2.2 | 0.710 |
| walker |  | 2548 | 78 | export member doc at src/index.tsx:50 member 121 |  |  | 0.716 |
| walker |  | 2629 | 81 | export member doc at src/index.tsx:50 member 60 |  |  | 0.743 |
| walker |  | 2643 | 14 | export names surface in src/use-position-fixed.ts |  |  | 0.743 |
| walker |  | 2657 | 14 | export names surface in src/use-snap-points.ts |  |  | 0.743 |
| ns | 2661 |  | 171 | Docs for defaultOpen, disablePreventScroll and repositionInputs | 2.9 | 2.3 | 0.744 |
| walker |  | 2794 | 137 | package scripts in package.json |  |  | 0.770 |
| ns | 2802 |  | 141 | Docs for snapToSequentialPoint and onAnimationEnd — last of the prop docs | 2.10 | 2.3 | 0.770 |
| walker |  | 2809 | 15 | export names surface in src/use-scale-background.ts |  |  | 0.770 |
| walker |  | 2809 | 0 | export at src/use-scale-background.ts:8 |  |  | 0.770 |
| walker |  | 2981 | 172 | package entrypoints in package.json |  |  | 0.914 |
| ns | 2991 |  | 189 | src/constants.ts in full — every tuning constant and its value | 3.1 |  | 0.877 |
| ns | 3069 |  | 78 | src/types.ts in full — DrawerDirection, SnapPoint, AnyFunction | 3.2 |  | 0.864 |
| ns | 3242 |  | 173 | src/helpers.ts — all eight exported helper signatures | 3.3 |  | 0.849 |
| walker |  | 3295 | 314 | imports in src/index.tsx |  |  | 0.923 |
| ns | 3354 |  | 112 | src/browser.ts — the complete platform-detection roster | 3.4 |  | 0.909 |
| walker |  | 3471 | 176 | package identity metadata in package.json |  |  | 0.909 |
| walker |  | 3582 | 111 | json config tsconfig.json |  |  | 0.910 |
| walker |  | 3618 | 36 | export names surface in src/context.ts |  |  | 0.911 |
| walker |  | 3618 | 0 | export at src/context.ts:69 |  |  | 0.911 |
| ns | 3633 |  | 279 | DrawerContextValue, first half — refs and pointer callbacks | 3.5 |  | 0.881 |
| walker |  | 3717 | 99 | listing of 'test/src/app' |  |  | 0.883 |
| walker |  | 3734 | 17 | imports in playwright.config.ts |  |  | 0.883 |
| walker |  | 3771 | 37 | export names surface in src/use-controllable-state.ts |  |  | 0.883 |
| walker |  | 3771 | 0 | export at src/use-controllable-state.ts:39 |  |  | 0.883 |
| ns | 3813 |  | 180 | DrawerContextValue, second half — snap points, direction, container | 3.6 | 3.5 | 0.858 |
| walker |  | 3837 | 66 | listing of 'test/tests' |  |  | 0.859 |
| walker |  | 3895 | 58 | export names surface in src/use-composed-refs.ts |  |  | 0.860 |
| walker |  | 3895 | 0 | export at src/use-composed-refs.ts:23 |  |  | 0.860 |
| walker |  | 3895 | 0 | export at src/use-composed-refs.ts:31 |  |  | 0.860 |
| ns | 3912 |  | 99 | DrawerContext creation and the useDrawerContext accessor | 3.7 | 3.6 | 0.847 |
| walker |  | 3920 | 25 | export body at src/use-composed-refs.ts:23 body 24 |  |  | 0.847 |
| walker |  | 3955 | 35 | export body at src/use-composed-refs.ts:31 body 32 |  |  | 0.847 |
| walker |  | 3967 | 12 | imports in src/use-controllable-state.ts |  |  | 0.847 |
| walker |  | 3981 | 14 | imports in src/use-composed-refs.ts |  |  | 0.847 |
| walker |  | 3990 | 9 | listing of 'test/public' |  |  | 0.847 |
| ns | 4022 |  | 110 | useSnapPoints: entry point and its complete return surface | 3.8 |  | 0.828 |
| walker |  | 4114 | 124 | export at src/use-position-fixed.ts:15 |  |  | 0.829 |
| walker |  | 4173 | 59 | export body at src/context.ts:69 body 70 |  |  | 0.841 |
| ns | 4271 |  | 249 | useSnapPoints parameter object (fills 3.8's ellipsis) | 3.9 | 3.8 | 0.811 |
| ns | 4565 |  | 294 | usePositionFixed: the iOS rationale comment, its signature and return | 3.10 |  | 0.788 |
| walker |  | 4685 | 512 | export at playwright.config.ts:12 |  |  | 0.790 |
| walker |  | 4711 | 26 | export doc at playwright.config.ts:12 |  |  | 0.790 |
| walker |  | 4729 | 18 | imports in src/helpers.ts |  |  | 0.790 |
| ns | 4758 |  | 193 | use-prevent-scroll.ts exported surface and its provenance | 3.11 |  | 0.776 |
| walker |  | 4840 | 111 | json config turbo.json |  |  | 0.777 |
| ns | 4907 |  | 149 | use-prevent-scroll.ts module-private declarations | 3.12 | 3.11 | 0.766 |
| walker |  | 4937 | 97 | export names surface in src/use-prevent-scroll.ts |  |  | 0.770 |
| walker |  | 4937 | 0 | export at src/use-prevent-scroll.ts:29 |  |  | 0.770 |
| walker |  | 4937 | 0 | export at src/use-prevent-scroll.ts:34 |  |  | 0.770 |
| walker |  | 4937 | 0 | export at src/use-prevent-scroll.ts:68 |  |  | 0.770 |
| walker |  | 4937 | 0 | export at src/use-prevent-scroll.ts:294 |  |  | 0.770 |
| walker |  | 4978 | 41 | export body at src/use-prevent-scroll.ts:29 body 30 |  |  | 0.770 |
| walker |  | 5040 | 62 | export body at src/use-prevent-scroll.ts:294 body 295 |  |  | 0.770 |
| ns | 5125 |  | 218 | use-controllable-state.ts — the controlled/uncontrolled prop machinery | 3.13 |  | 0.759 |
| walker |  | 5132 | 92 | export body at src/use-prevent-scroll.ts:34 body 35 |  |  | 0.759 |
| walker |  | 5155 | 23 | imports in src/context.ts |  |  | 0.759 |
| walker |  | 5178 | 23 | imports in src/use-position-fixed.ts |  |  | 0.759 |
| walker |  | 5290 | 112 | export names surface in src/browser.ts |  |  | 0.771 |
| walker |  | 5290 | 0 | export at src/browser.ts:1 |  |  | 0.771 |
| walker |  | 5290 | 0 | export at src/browser.ts:10 |  |  | 0.771 |
| walker |  | 5290 | 0 | export at src/browser.ts:14 |  |  | 0.771 |
| walker |  | 5290 | 0 | export at src/browser.ts:18 |  |  | 0.771 |
| walker |  | 5290 | 0 | export at src/browser.ts:22 |  |  | 0.771 |
| walker |  | 5290 | 0 | export at src/browser.ts:30 |  |  | 0.771 |
| walker |  | 5290 | 0 | export at src/browser.ts:34 |  |  | 0.771 |
| walker |  | 5302 | 12 | export body at src/browser.ts:10 body 11 |  |  | 0.771 |
| walker |  | 5315 | 13 | export body at src/browser.ts:14 body 15 |  |  | 0.771 |
| ns | 5319 |  | 194 | useScaleBackground and use-composed-refs — the two smallest modules | 3.14 |  | 0.763 |
| walker |  | 5338 | 23 | export body at src/browser.ts:18 body 19 |  |  | 0.763 |
| walker |  | 5421 | 83 | export body at src/browser.ts:1 body 2 |  |  | 0.763 |
| walker |  | 5458 | 37 | export doc at src/use-composed-refs.ts:23 |  |  | 0.763 |
| walker |  | 5496 | 38 | export doc at src/use-composed-refs.ts:31 |  |  | 0.763 |
| ns | 5542 |  | 223 | Every member declared inside Root (names + effect locations) | 4.1 |  | 0.746 |
| walker |  | 5640 | 144 | export names surface in src/constants.ts |  |  | 0.766 |
| walker |  | 5680 | 40 | export at src/constants.ts:1 |  |  | 0.774 |
| walker |  | 5712 | 32 | imports in src/use-prevent-scroll.ts |  |  | 0.774 |
| ns | 5760 |  | 218 | Root's state and refs — the whole drag bookkeeping set | 4.2 |  | 0.756 |
| ns | 5867 |  | 107 | Root's useSnapPoints wiring — what it destructures and what it passes | 4.3 |  | 0.746 |
| walker |  | 5961 | 249 | export at src/use-snap-points.ts:7 |  |  | 0.779 |
| ns | 6004 |  | 137 | Scroll-lock and body-position wiring, including the isDisabled predicate | 4.4 |  | 0.767 |
| walker |  | 6132 | 171 | export names surface in src/helpers.ts |  |  | 0.779 |
| walker |  | 6132 | 0 | export at src/helpers.ts:9 |  |  | 0.779 |
| walker |  | 6132 | 0 | export at src/helpers.ts:23 |  |  | 0.779 |
| walker |  | 6132 | 0 | export at src/helpers.ts:42 |  |  | 0.779 |
| walker |  | 6132 | 0 | export at src/helpers.ts:59 |  |  | 0.779 |
| walker |  | 6132 | 0 | export at src/helpers.ts:72 |  |  | 0.779 |
| walker |  | 6132 | 0 | export at src/helpers.ts:90 |  |  | 0.779 |
| walker |  | 6132 | 0 | export at src/helpers.ts:94 |  |  | 0.779 |
| walker |  | 6132 | 0 | export at src/helpers.ts:108 |  |  | 0.779 |
| walker |  | 6161 | 29 | export doc at src/helpers.ts:108 |  |  | 0.779 |
| ns | 6214 |  | 210 | shouldDrag: the early-out guards | 4.5 | 4.1 | 0.762 |
| walker |  | 6246 | 85 | export body at src/helpers.ts:59 body 60 |  |  | 0.762 |
| walker |  | 6362 | 116 | export body at src/helpers.ts:9 body 10 |  |  | 0.762 |
| walker |  | 6508 | 146 | export body at src/helpers.ts:42 body 43 |  |  | 0.762 |
| ns | 6564 |  | 350 | shouldDrag: direction, open-animation window and scroll-lock timeout | 4.6 | 4.5 | 0.736 |
| walker |  | 6810 | 302 | package dev/peer dependencies in package.json |  |  | 0.737 |
| ns | 6813 |  | 249 | shouldDrag: the scrollable-ancestor climb | 4.7 | 4.6 | 0.719 |
| walker |  | 6997 | 187 | export body at src/use-prevent-scroll.ts:68 body 69 |  |  | 0.719 |
| ns | 7042 |  | 229 | onRelease: teardown and velocity computation | 4.8 | 4.1 | 0.707 |
| walker |  | 7055 | 58 | export doc at src/use-prevent-scroll.ts:68 |  |  | 0.707 |
| walker |  | 7302 | 247 | LICENSE.md section #0 |  |  | 0.707 |
| ns | 7355 |  | 313 | onRelease: the close-vs-snap-back decision ladder | 4.9 | 4.8 | 0.690 |
| ns | 7627 |  | 272 | The data-vaul-* attributes emitted by Overlay and Content | 4.10 |  | 0.680 |
| walker |  | 7629 | 327 | export at src/context.ts:37 |  |  | 0.681 |
| ns | 7796 |  | 169 | Handle: click-to-cycle snap points | 4.11 |  | 0.671 |
| walker |  | 7886 | 257 | export body at src/use-controllable-state.ts:39 body 40 |  |  | 0.671 |
| ns | 7942 |  | 146 | NestedRoot: how a nested drawer is wired to its parent | 4.12 |  | 0.663 |
| walker |  | 8060 | 174 | export body at src/helpers.ts:23 body 24 |  |  | 0.663 |
| ns | 8092 |  | 150 | The base [data-vaul-drawer] rule and the shape of the variant rules | 5.1 |  | 0.658 |
| walker |  | 8129 | 69 | imports in src/use-scale-background.ts |  |  | 0.658 |
| walker |  | 8201 | 72 | imports in src/use-snap-points.ts |  |  | 0.658 |
| walker |  | 8277 | 76 | README headline in test/README.md |  |  | 0.653 |
| ns | 8277 |  | 185 | style.css selector inventory — which attribute combinations are styled | 5.2 | 5.1 | 0.653 |
| walker |  | 8300 | 23 | headings outline in test/README.md |  |  | 0.653 |
| ns | 8405 |  | 128 | Every @keyframes name in style.css | 5.3 |  | 0.647 |
| ns | 8504 |  | 99 | Complete listing of test/src/app — one demo route per feature | 6.1 |  | 0.656 |
| ns | 8570 |  | 66 | Complete listing of test/tests — the Playwright spec set | 6.2 |  | 0.661 |
| walker |  | 8580 | 280 | declaration surface of src/style.css |  |  | 0.662 |
| ns | 8622 |  | 52 | Listing of the test/ package root | 6.3 |  | 0.666 |
| walker |  | 8727 | 147 | export doc at src/use-position-fixed.ts:15 |  |  | 0.677 |
| ns | 8915 |  | 293 | Spec-suite to demo-route map for every Playwright file | 6.4 |  | 0.668 |
| ns | 9117 |  | 202 | Playwright runner configuration: server, devices, testDir | 6.5 |  | 0.675 |
| ns | 9223 |  | 106 | Shared e2e helpers: openDrawer and ANIMATION_DURATION | 6.6 |  | 0.671 |
| ns | 9354 |  | 131 | package.json publishing surface: files and the exports map | 7.1 |  | 0.676 |
| walker |  | 9392 | 665 | export body at src/use-scale-background.ts:8 body 9 |  |  | 0.680 |
| walker |  | 9444 | 52 | export names surface in src/types.ts |  |  | 0.683 |
| walker |  | 9465 | 21 | export at src/types.ts:2 |  |  | 0.686 |
| ns | 9516 |  | 162 | Runtime and peer dependencies, and the pinned package manager | 7.2 |  | 0.690 |
| ns | 9644 |  | 128 | Workspace and task-runner config: pnpm-workspace.yaml and turbo.json | 7.3 |  | 0.694 |
| ns | 9749 |  | 105 | Root tsconfig.json — compiler settings for the shipped library | 7.4 |  | 0.697 |
| ns | 9753 |  | 4 | Listing of .github/workflows | 7.5 |  | 0.697 |
| walker |  | 9782 | 317 | export body at src/index.tsx:803 body 805 |  |  | 0.701 |
| ns | 9913 |  | 160 | CI: the Playwright workflow steps | 7.6 |  | 0.695 |
| ns | 9978 |  | 65 | Prettier configuration — the formatting any new code must match | 7.7 |  | 0.692 |

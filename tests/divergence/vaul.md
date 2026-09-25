Score(3000)=0.764 I=0.898 C=0.650 ns_rows≤3K=20/61 grid(1000/1442/2080/3000/4327/6240/9000)=0.637/0.581/0.770/0.764/0.798/0.725/0.655

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 62 | 62 | listing of '.' |  |  | 0.000 |
| walker |  | 65 | 3 | listing of '.github' |  |  | 0.000 |
| walker |  | 69 | 4 | listing of '.github/workflows' |  |  | 0.000 |
| walker |  | 73 | 4 | listing of '.vscode' |  |  | 0.000 |
| ns | 86 |  | 86 | Package identity: name, version, description, entry points | 1.1 |  | 0.000 |
| walker |  | 131 | 58 | package identity in package.json |  |  | 0.460 |
| ns | 148 |  | 62 | Complete repository root listing | 1.2 |  | 0.677 |
| walker |  | 167 | 36 | package runtime dependencies in package.json |  |  | 0.677 |
| ns | 219 |  | 71 | Complete src/ listing — the shipped library | 1.3 |  | 0.507 |
| walker |  | 238 | 71 | listing of 'src' |  |  | 0.737 |
| walker |  | 257 | 19 | package runtime metadata in package.json |  |  | 0.737 |
| walker |  | 320 | 63 | README.md section #0 |  |  | 0.743 |
| ns | 334 |  | 115 | The `Drawer` namespace object — the entire public component set | 1.4 |  | 0.620 |
| ns | 402 |  | 68 | README: the unmaintained notice (whole file) | 1.5 |  | 0.616 |
| ns | 588 |  | 186 | Every top-level export declaration in src/index.tsx (names only) | 1.6 |  | 0.541 |
| walker |  | 605 | 285 | ts names src/index.tsx |  |  | 0.659 |
| walker |  | 618 | 13 | ts decl src/index.tsx:803 |  |  | 0.659 |
| walker |  | 631 | 13 | ts decl src/index.tsx:989 |  |  | 0.659 |
| walker |  | 660 | 29 | ts decl src/index.tsx:996 |  |  | 0.659 |
| walker |  | 694 | 34 | ts decl src/index.tsx:833 |  |  | 0.659 |
| ns | 735 |  | 147 | src/index.tsx imports: Radix dialog, style.css, and the first hook modules | 1.7 |  | 0.602 |
| walker |  | 795 | 101 | ts decl src/index.tsx:1137 |  |  | 0.711 |
| ns | 902 |  | 167 | src/index.tsx imports: the constants set and the remaining hook modules | 1.8 | 1.7 | 0.635 |
| walker |  | 915 | 120 | ts decl src/index.tsx:40 |  |  | 0.637 |
| ns | 1039 |  | 137 | package.json scripts — build, dev, test, format | 1.9 |  | 0.601 |
| walker |  | 1076 | 161 | ts decl src/index.tsx:27 |  |  | 0.612 |
| ns | 1095 |  | 56 | Snap-point prop union: the fields of WithFadeFromProps / WithoutFadeFromProps | 2.1 | 1.6 | 0.622 |
| ns | 1257 |  | 162 | DialogProps declarations, first half (lines 51-85), doc comments elided | 2.2 | 1.6 | 0.565 |
| walker |  | 1394 | 318 | ts decl src/index.tsx:139 |  |  | 0.581 |
| walker |  | 1408 | 14 | ts names src/use-position-fixed.ts |  |  | 0.581 |
| walker |  | 1422 | 14 | ts names src/use-snap-points.ts |  |  | 0.581 |
| walker |  | 1474 | 52 | listing of 'test' |  |  | 0.583 |
| walker |  | 1477 | 3 | listing of 'test/src' |  |  | 0.583 |
| walker |  | 1492 | 15 | ts names src/use-scale-background.ts |  |  | 0.583 |
| ns | 1494 |  | 237 | DialogProps declarations, second half (lines 86-137) plus the union tail | 2.3 | 2.2 | 0.524 |
| walker |  | 1706 | 214 | ts decl src/index.tsx:50 |  |  | 0.546 |
| walker |  | 1729 | 23 | plaintext config pnpm-workspace.yaml |  |  | 0.546 |
| ns | 1812 |  | 318 | Root's destructured parameter list — every prop's default value | 2.4 | 1.6 | 0.587 |
| walker |  | 1866 | 137 | package scripts in package.json |  |  | 0.622 |
| ns | 2031 |  | 219 | Snap-point prop documentation (fills 2.1's ellipses) | 2.5 | 2.1 | 0.631 |
| walker |  | 2038 | 172 | package entrypoints in package.json |  |  | 0.770 |
| ns | 2198 |  | 167 | Docs for dismissible, modal and direction (fills 2.3's ellipses) | 2.6 | 2.3 | 0.741 |
| walker |  | 2295 | 257 | ts decl src/index.tsx:50 #1 |  |  | 0.796 |
| walker |  | 2331 | 36 | ts names src/context.ts |  |  | 0.796 |
| walker |  | 2368 | 37 | ts names src/use-controllable-state.ts |  |  | 0.796 |
| ns | 2398 |  | 200 | Docs for closeThreshold, noBodyStyles, setBackgroundColorOnScale, scrollLockTimeout | 2.7 | 2.2 | 0.804 |
| ns | 2490 |  | 92 | Docs for fixed and handleOnly | 2.8 | 2.2 | 0.807 |
| walker |  | 2583 | 215 | ts decl src/index.tsx:50 #2 |  |  | 0.840 |
| ns | 2661 |  | 171 | Docs for defaultOpen, disablePreventScroll and repositionInputs | 2.9 | 2.3 | 0.813 |
| walker |  | 2759 | 176 | package identity metadata in package.json |  |  | 0.813 |
| ns | 2802 |  | 141 | Docs for snapToSequentialPoint and onAnimationEnd — last of the prop docs | 2.10 | 2.3 | 0.794 |
| walker |  | 2870 | 111 | json config tsconfig.json |  |  | 0.795 |
| walker |  | 2922 | 52 | ts names src/types.ts |  |  | 0.795 |
| walker |  | 2943 | 21 | ts decl src/types.ts:2 |  |  | 0.796 |
| ns | 2991 |  | 189 | src/constants.ts in full — every tuning constant and its value | 3.1 |  | 0.764 |
| walker |  | 3001 | 58 | ts names src/use-composed-refs.ts |  |  | 0.765 |
| ns | 3069 |  | 78 | src/types.ts in full — DrawerDirection, SnapPoint, AnyFunction | 3.2 |  | 0.766 |
| walker |  | 3100 | 99 | listing of 'test/src/app' |  |  | 0.768 |
| ns | 3242 |  | 173 | src/helpers.ts — all eight exported helper signatures | 3.3 |  | 0.755 |
| ns | 3354 |  | 112 | src/browser.ts — the complete platform-detection roster | 3.4 |  | 0.744 |
| walker |  | 3379 | 279 | ts decl src/index.tsx:50 #3 |  |  | 0.809 |
| walker |  | 3445 | 66 | listing of 'test/tests' |  |  | 0.810 |
| walker |  | 3542 | 97 | ts names src/use-prevent-scroll.ts |  |  | 0.811 |
| ns | 3633 |  | 279 | DrawerContextValue, first half — refs and pointer callbacks | 3.5 |  | 0.784 |
| walker |  | 3747 | 205 | ts decl src/index.tsx:50 #4 |  |  | 0.834 |
| ns | 3813 |  | 180 | DrawerContextValue, second half — snap points, direction, container | 3.6 | 3.5 | 0.811 |
| walker |  | 3859 | 112 | ts names src/browser.ts |  |  | 0.825 |
| walker |  | 3871 | 12 | ts body src/browser.ts:10 |  |  | 0.825 |
| walker |  | 3884 | 13 | ts body src/browser.ts:14 |  |  | 0.825 |
| walker |  | 3899 | 15 | ts body src/browser.ts:30 |  |  | 0.825 |
| ns | 3912 |  | 99 | DrawerContext creation and the useDrawerContext accessor | 3.7 | 3.6 | 0.812 |
| ns | 4022 |  | 110 | useSnapPoints: entry point and its complete return surface | 3.8 |  | 0.794 |
| walker |  | 4023 | 124 | ts decl src/use-position-fixed.ts:15 |  |  | 0.794 |
| walker |  | 4032 | 9 | listing of 'test/public' |  |  | 0.794 |
| walker |  | 4176 | 144 | ts names src/constants.ts |  |  | 0.817 |
| walker |  | 4216 | 40 | ts decl src/constants.ts:1 |  |  | 0.827 |
| walker |  | 4239 | 23 | ts body src/browser.ts:18 |  |  | 0.827 |
| walker |  | 4264 | 25 | ts body src/use-composed-refs.ts:23 |  |  | 0.827 |
| ns | 4271 |  | 249 | useSnapPoints parameter object (fills 3.8's ellipsis) | 3.9 | 3.8 | 0.798 |
| walker |  | 4324 | 60 | ts body src/index.tsx:1130 |  |  | 0.798 |
| walker |  | 4435 | 111 | json config turbo.json |  |  | 0.799 |
| ns | 4565 |  | 294 | usePositionFixed: the iOS rationale comment, its signature and return | 3.10 |  | 0.776 |
| walker |  | 4608 | 173 | ts names src/helpers.ts |  |  | 0.790 |
| walker |  | 4629 | 21 | ts body src/helpers.ts:90 |  |  | 0.790 |
| walker |  | 4658 | 29 | ts doc src/helpers.ts:108 |  |  | 0.790 |
| walker |  | 4693 | 35 | ts body src/use-composed-refs.ts:31 |  |  | 0.790 |
| walker |  | 4730 | 37 | ts doc src/use-composed-refs.ts:23 |  |  | 0.790 |
| ns | 4758 |  | 193 | use-prevent-scroll.ts exported surface and its provenance | 3.11 |  | 0.780 |
| walker |  | 4768 | 38 | ts doc src/use-composed-refs.ts:31 |  |  | 0.780 |
| walker |  | 4795 | 27 | ts body src/browser.ts:34 |  |  | 0.780 |
| ns | 4907 |  | 149 | use-prevent-scroll.ts module-private declarations | 3.12 | 3.11 | 0.769 |
| walker |  | 5044 | 249 | ts decl src/use-snap-points.ts:7 |  |  | 0.805 |
| walker |  | 5085 | 41 | ts body src/use-prevent-scroll.ts:29 |  |  | 0.805 |
| ns | 5125 |  | 218 | use-controllable-state.ts — the controlled/uncontrolled prop machinery | 3.13 |  | 0.793 |
| ns | 5319 |  | 194 | useScaleBackground and use-composed-refs — the two smallest modules | 3.14 |  | 0.785 |
| walker |  | 5412 | 327 | ts decl src/context.ts:37 |  |  | 0.786 |
| walker |  | 5425 | 13 | ts names test/tests/constants.ts |  |  | 0.786 |
| walker |  | 5483 | 58 | ts doc src/use-prevent-scroll.ts:68 |  |  | 0.786 |
| walker |  | 5542 | 59 | ts body src/context.ts:69 |  |  | 0.780 |
| ns | 5542 |  | 223 | Every member declared inside Root (names + effect locations) | 4.1 |  | 0.780 |
| walker |  | 5604 | 62 | ts body src/use-prevent-scroll.ts:294 |  |  | 0.780 |
| ns | 5760 |  | 218 | Root's state and refs — the whole drag bookkeeping set | 4.2 |  | 0.762 |
| walker |  | 5851 | 247 | LICENSE.md section #0 |  |  | 0.762 |
| ns | 5867 |  | 107 | Root's useSnapPoints wiring — what it destructures and what it passes | 4.3 |  | 0.752 |
| walker |  | 5928 | 77 | ts body src/helpers.ts:94 |  |  | 0.752 |
| ns | 6004 |  | 137 | Scroll-lock and body-position wiring, including the isDisabled predicate | 4.4 |  | 0.740 |
| walker |  | 6013 | 85 | ts body src/helpers.ts:59 |  |  | 0.740 |
| walker |  | 6105 | 92 | ts body src/use-prevent-scroll.ts:34 |  |  | 0.740 |
| walker |  | 6119 | 14 | ts names test/src/app/page.tsx |  |  | 0.740 |
| walker |  | 6195 | 76 | README headline in test/README.md |  |  | 0.740 |
| ns | 6214 |  | 210 | shouldDrag: the early-out guards | 4.5 | 4.1 | 0.725 |
| walker |  | 6218 | 23 | headings outline in test/README.md |  |  | 0.725 |
| walker |  | 6249 | 31 | ts names test/tests/helpers.ts |  |  | 0.725 |
| walker |  | 6336 | 87 | ts body src/helpers.ts:108 |  |  | 0.725 |
| walker |  | 6483 | 147 | ts doc src/use-position-fixed.ts:15 |  |  | 0.739 |
| ns | 6564 |  | 350 | shouldDrag: direction, open-animation window and scroll-lock timeout | 4.6 | 4.5 | 0.713 |
| walker |  | 6763 | 280 | declaration surface of src/style.css |  |  | 0.713 |
| ns | 6813 |  | 249 | shouldDrag: the scrollable-ancestor climb | 4.7 | 4.6 | 0.696 |
| ns | 7042 |  | 229 | onRelease: teardown and velocity computation | 4.8 | 4.1 | 0.684 |
| walker |  | 7058 | 295 | ts body src/index.tsx:1098 |  |  | 0.686 |
| walker |  | 7128 | 70 | ts body src/browser.ts:22 |  |  | 0.686 |
| ns | 7355 |  | 313 | onRelease: the close-vs-snap-back decision ladder | 4.9 | 4.8 | 0.669 |
| walker |  | 7445 | 317 | ts body src/index.tsx:803 |  |  | 0.670 |
| walker |  | 7528 | 83 | ts body src/browser.ts:1 |  |  | 0.670 |
| ns | 7627 |  | 272 | The data-vaul-* attributes emitted by Overlay and Content | 4.10 |  | 0.664 |
| walker |  | 7644 | 116 | ts body src/helpers.ts:9 |  |  | 0.664 |
| walker |  | 7703 | 59 | ts decl test/tests/helpers.ts:11 |  |  | 0.664 |
| ns | 7796 |  | 169 | Handle: click-to-cycle snap points | 4.11 |  | 0.655 |
| ns | 7942 |  | 146 | NestedRoot: how a nested drawer is wired to its parent | 4.12 |  | 0.661 |
| walker |  | 7960 | 257 | ts body src/use-controllable-state.ts:39 |  |  | 0.661 |
| walker |  | 7997 | 37 | ts names test/src/app/layout.tsx |  |  | 0.661 |
| walker |  | 8026 | 29 | ts decl test/src/app/layout.tsx:7 |  |  | 0.661 |
| ns | 8092 |  | 150 | The base [data-vaul-drawer] rule and the shape of the variant rules | 5.1 |  | 0.657 |
| walker |  | 8213 | 187 | ts body src/use-prevent-scroll.ts:68 |  |  | 0.657 |
| ns | 8277 |  | 185 | style.css selector inventory — which attribute combinations are styled | 5.2 | 5.1 | 0.652 |
| ns | 8405 |  | 128 | Every @keyframes name in style.css | 5.3 |  | 0.646 |
| ns | 8504 |  | 99 | Complete listing of test/src/app — one demo route per feature | 6.1 |  | 0.655 |
| walker |  | 8524 | 311 | YAML config at .github/workflows/playwright.yml |  |  | 0.655 |
| ns | 8570 |  | 66 | Complete listing of test/tests — the Playwright spec set | 6.2 |  | 0.660 |
| ns | 8622 |  | 52 | Listing of the test/ package root | 6.3 |  | 0.664 |
| walker |  | 8670 | 146 | ts body src/helpers.ts:42 |  |  | 0.664 |
| walker |  | 8827 | 157 | test/README.md section #0 |  |  | 0.664 |
| walker |  | 8867 | 40 | package identity in test/package.json |  |  | 0.664 |
| ns | 8915 |  | 293 | Spec-suite to demo-route map for every Playwright file | 6.4 |  | 0.655 |
| walker |  | 8930 | 63 | package scripts in test/package.json |  |  | 0.655 |
| ns | 9117 |  | 202 | Playwright runner configuration: server, devices, testDir | 6.5 |  | 0.646 |
| ns | 9223 |  | 106 | Shared e2e helpers: openDrawer and ANIMATION_DURATION | 6.6 |  | 0.643 |
| ns | 9354 |  | 131 | package.json publishing surface: files and the exports map | 7.1 |  | 0.648 |
| ns | 9516 |  | 162 | Runtime and peer dependencies, and the pinned package manager | 7.2 |  | 0.645 |
| walker |  | 9595 | 665 | ts body src/use-scale-background.ts:8 |  |  | 0.648 |
| ns | 9644 |  | 128 | Workspace and task-runner config: pnpm-workspace.yaml and turbo.json | 7.3 |  | 0.653 |
| ns | 9749 |  | 105 | Root tsconfig.json — compiler settings for the shipped library | 7.4 |  | 0.656 |
| ns | 9753 |  | 4 | Listing of .github/workflows | 7.5 |  | 0.657 |
| walker |  | 9769 | 174 | ts body src/helpers.ts:23 |  |  | 0.657 |
| ns | 9913 |  | 160 | CI: the Playwright workflow steps | 7.6 |  | 0.661 |
| ns | 9978 |  | 65 | Prettier configuration — the formatting any new code must match | 7.7 |  | 0.657 |

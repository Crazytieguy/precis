Score(3000)=0.877 I=0.934 C=0.824 ns_rows≤3K=20/61 grid(1000/1442/2080/3000/4327/6240/9000)=0.690/0.750/0.797/0.877/0.820/0.733/0.658

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 62 | 62 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 65 | 3 | Fs::DirListing { dir: .github } |  |  | 0.000 |
| walker |  | 69 | 4 | Fs::DirListing { dir: .github/workflows } |  |  | 0.000 |
| walker |  | 73 | 4 | Fs::DirListing { dir: .vscode } |  |  | 0.000 |
| ns | 86 |  | 86 | Package identity: name, version, description, entry points | 1.1 |  | 0.000 |
| walker |  | 131 | 58 | Json::Identity { file: package.json } |  |  | 0.460 |
| ns | 148 |  | 62 | Complete repository root listing | 1.2 |  | 0.677 |
| walker |  | 202 | 71 | Fs::DirListing { dir: src } |  |  | 0.746 |
| ns | 219 |  | 71 | Complete src/ listing — the shipped library | 1.3 |  | 0.737 |
| walker |  | 223 | 21 | Json::Runtime { file: package.json } |  |  | 0.737 |
| walker |  | 286 | 63 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.743 |
| walker |  | 320 | 34 | Json::Dependencies { file: package.json } |  |  | 0.743 |
| ns | 334 |  | 115 | The `Drawer` namespace object — the entire public component set | 1.4 |  | 0.620 |
| walker |  | 372 | 52 | Fs::DirListing { dir: test } |  |  | 0.622 |
| walker |  | 375 | 3 | Fs::DirListing { dir: test/src } |  |  | 0.622 |
| walker |  | 398 | 23 | Plaintext::Whole { file: pnpm-workspace.yaml } |  |  | 0.622 |
| ns | 402 |  | 68 | README: the unmaintained notice (whole file) | 1.5 |  | 0.618 |
| walker |  | 535 | 137 | Json::Scripts { file: package.json } |  |  | 0.629 |
| ns | 588 |  | 186 | Every top-level export declaration in src/index.tsx (names only) | 1.6 |  | 0.552 |
| walker |  | 707 | 172 | Json::Entry { file: package.json } |  |  | 0.740 |
| ns | 735 |  | 147 | src/index.tsx imports: Radix dialog, style.css, and the first hook modules | 1.7 |  | 0.676 |
| ns | 902 |  | 167 | src/index.tsx imports: the constants set and the remaining hook modules | 1.8 | 1.7 | 0.603 |
| walker |  | 935 | 228 | Code::CodeKey { rung: Names, file: src/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.690 |
| walker |  | 948 | 13 | Code::CodeKey { rung: Decl, file: src/index.tsx, decl: 5, sub: 0, line: 803 } |  |  | 0.690 |
| walker |  | 963 | 15 | Code::CodeKey { rung: Decl, file: src/index.tsx, decl: 8, sub: 0, line: 989 } |  |  | 0.690 |
| walker |  | 992 | 29 | Code::CodeKey { rung: Decl, file: src/index.tsx, decl: 9, sub: 0, line: 996 } |  |  | 0.690 |
| walker |  | 1026 | 34 | Code::CodeKey { rung: Decl, file: src/index.tsx, decl: 7, sub: 0, line: 833 } |  |  | 0.690 |
| ns | 1039 |  | 137 | package.json scripts — build, dev, test, format | 1.9 |  | 0.711 |
| ns | 1095 |  | 56 | Snap-point prop union: the fields of WithFadeFromProps / WithoutFadeFromProps | 2.1 | 1.6 | 0.681 |
| walker |  | 1127 | 101 | Code::CodeKey { rung: Decl, file: src/index.tsx, decl: 12, sub: 0, line: 1137 } |  |  | 0.775 |
| walker |  | 1247 | 120 | Code::CodeKey { rung: Decl, file: src/index.tsx, decl: 2, sub: 0, line: 40 } |  |  | 0.785 |
| ns | 1257 |  | 162 | DialogProps declarations, first half (lines 51-85), doc comments elided | 2.2 | 1.6 | 0.714 |
| walker |  | 1408 | 161 | Code::CodeKey { rung: Decl, file: src/index.tsx, decl: 1, sub: 0, line: 27 } |  |  | 0.750 |
| ns | 1494 |  | 237 | DialogProps declarations, second half (lines 86-137) plus the union tail | 2.3 | 2.2 | 0.674 |
| walker |  | 1726 | 318 | Code::CodeKey { rung: Decl, file: src/index.tsx, decl: 4, sub: 0, line: 139 } |  |  | 0.687 |
| ns | 1812 |  | 318 | Root's destructured parameter list — every prop's default value | 2.4 | 1.6 | 0.729 |
| ns | 2031 |  | 219 | Snap-point prop documentation (fills 2.1's ellipses) | 2.5 | 2.1 | 0.741 |
| ns | 2198 |  | 167 | Docs for dismissible, modal and direction (fills 2.3's ellipses) | 2.6 | 2.3 | 0.712 |
| ns | 2398 |  | 200 | Docs for closeThreshold, noBodyStyles, setBackgroundColorOnScale, scrollLockTimeout | 2.7 | 2.2 | 0.681 |
| ns | 2490 |  | 92 | Docs for fixed and handleOnly | 2.8 | 2.2 | 0.669 |
| ns | 2661 |  | 171 | Docs for defaultOpen, disablePreventScroll and repositionInputs | 2.9 | 2.3 | 0.647 |
| ns | 2802 |  | 141 | Docs for snapToSequentialPoint and onAnimationEnd — last of the prop docs | 2.10 | 2.3 | 0.632 |
| walker |  | 2896 | 1170 | Code::CodeKey { rung: Decl, file: src/index.tsx, decl: 3, sub: 0, line: 50 } |  |  | 0.912 |
| ns | 2991 |  | 189 | src/constants.ts in full — every tuning constant and its value | 3.1 |  | 0.875 |
| walker |  | 2995 | 99 | Fs::DirListing { dir: test/src/app } |  |  | 0.877 |
| walker |  | 3061 | 66 | Fs::DirListing { dir: test/tests } |  |  | 0.879 |
| ns | 3069 |  | 78 | src/types.ts in full — DrawerDirection, SnapPoint, AnyFunction | 3.2 |  | 0.865 |
| walker |  | 3075 | 14 | Code::CodeKey { rung: Names, file: src/use-position-fixed.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.865 |
| walker |  | 3199 | 124 | Code::CodeKey { rung: Decl, file: src/use-position-fixed.ts, decl: 1, sub: 0, line: 15 } |  |  | 0.866 |
| walker |  | 3213 | 14 | Code::CodeKey { rung: Names, file: src/use-snap-points.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.866 |
| ns | 3242 |  | 173 | src/helpers.ts — all eight exported helper signatures | 3.3 |  | 0.851 |
| ns | 3354 |  | 112 | src/browser.ts — the complete platform-detection roster | 3.4 |  | 0.839 |
| walker |  | 3462 | 249 | Code::CodeKey { rung: Decl, file: src/use-snap-points.ts, decl: 1, sub: 0, line: 7 } |  |  | 0.844 |
| walker |  | 3522 | 60 | Code::CodeKey { rung: Body, file: src/index.tsx, decl: 11, sub: 0, line: 1130 } |  |  | 0.844 |
| walker |  | 3537 | 15 | Code::CodeKey { rung: Names, file: src/use-scale-background.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.844 |
| ns | 3633 |  | 279 | DrawerContextValue, first half — refs and pointer callbacks | 3.5 |  | 0.816 |
| walker |  | 3649 | 112 | Code::CodeKey { rung: Names, file: src/browser.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.831 |
| walker |  | 3661 | 12 | Code::CodeKey { rung: Body, file: src/browser.ts, decl: 2, sub: 0, line: 10 } |  |  | 0.831 |
| walker |  | 3674 | 13 | Code::CodeKey { rung: Body, file: src/browser.ts, decl: 3, sub: 0, line: 14 } |  |  | 0.831 |
| walker |  | 3689 | 15 | Code::CodeKey { rung: Body, file: src/browser.ts, decl: 6, sub: 0, line: 30 } |  |  | 0.831 |
| walker |  | 3712 | 23 | Code::CodeKey { rung: Body, file: src/browser.ts, decl: 4, sub: 0, line: 18 } |  |  | 0.831 |
| walker |  | 3739 | 27 | Code::CodeKey { rung: Body, file: src/browser.ts, decl: 7, sub: 0, line: 34 } |  |  | 0.831 |
| walker |  | 3791 | 52 | Code::CodeKey { rung: Names, file: src/types.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.836 |
| walker |  | 3812 | 21 | Code::CodeKey { rung: Decl, file: src/types.ts, decl: 2, sub: 0, line: 2 } |  |  | 0.843 |
| ns | 3813 |  | 180 | DrawerContextValue, second half — snap points, direction, container | 3.6 | 3.5 | 0.820 |
| walker |  | 3848 | 36 | Code::CodeKey { rung: Names, file: src/context.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.820 |
| ns | 3912 |  | 99 | DrawerContext creation and the useDrawerContext accessor | 3.7 | 3.6 | 0.807 |
| ns | 4022 |  | 110 | useSnapPoints: entry point and its complete return surface | 3.8 |  | 0.790 |
| walker |  | 4175 | 327 | Code::CodeKey { rung: Decl, file: src/context.ts, decl: 1, sub: 0, line: 37 } |  |  | 0.791 |
| walker |  | 4234 | 59 | Code::CodeKey { rung: Body, file: src/context.ts, decl: 2, sub: 0, line: 69 } |  |  | 0.805 |
| ns | 4271 |  | 249 | useSnapPoints parameter object (fills 3.8's ellipsis) | 3.9 | 3.8 | 0.811 |
| walker |  | 4378 | 144 | Code::CodeKey { rung: Names, file: src/constants.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.833 |
| walker |  | 4418 | 40 | Code::CodeKey { rung: Decl, file: src/constants.ts, decl: 1, sub: 0, line: 1 } |  |  | 0.842 |
| walker |  | 4476 | 58 | Code::CodeKey { rung: Names, file: src/use-composed-refs.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.842 |
| walker |  | 4501 | 25 | Code::CodeKey { rung: Body, file: src/use-composed-refs.ts, decl: 1, sub: 0, line: 23 } |  |  | 0.842 |
| walker |  | 4536 | 35 | Code::CodeKey { rung: Body, file: src/use-composed-refs.ts, decl: 2, sub: 0, line: 31 } |  |  | 0.842 |
| ns | 4565 |  | 294 | usePositionFixed: the iOS rationale comment, its signature and return | 3.10 |  | 0.818 |
| walker |  | 4573 | 37 | Code::CodeKey { rung: Doc, file: src/use-composed-refs.ts, decl: 1, sub: 0, line: 23 } |  |  | 0.818 |
| walker |  | 4611 | 38 | Code::CodeKey { rung: Doc, file: src/use-composed-refs.ts, decl: 2, sub: 0, line: 31 } |  |  | 0.818 |
| walker |  | 4708 | 97 | Code::CodeKey { rung: Names, file: src/use-prevent-scroll.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.819 |
| walker |  | 4749 | 41 | Code::CodeKey { rung: Body, file: src/use-prevent-scroll.ts, decl: 2, sub: 0, line: 29 } |  |  | 0.819 |
| ns | 4758 |  | 193 | use-prevent-scroll.ts exported surface and its provenance | 3.11 |  | 0.808 |
| walker |  | 4807 | 58 | Code::CodeKey { rung: Doc, file: src/use-prevent-scroll.ts, decl: 4, sub: 0, line: 68 } |  |  | 0.808 |
| walker |  | 4869 | 62 | Code::CodeKey { rung: Body, file: src/use-prevent-scroll.ts, decl: 5, sub: 0, line: 294 } |  |  | 0.808 |
| ns | 4907 |  | 149 | use-prevent-scroll.ts module-private declarations | 3.12 | 3.11 | 0.796 |
| walker |  | 4961 | 92 | Code::CodeKey { rung: Body, file: src/use-prevent-scroll.ts, decl: 3, sub: 0, line: 34 } |  |  | 0.796 |
| walker |  | 4970 | 9 | Fs::DirListing { dir: test/public } |  |  | 0.796 |
| ns | 5125 |  | 218 | use-controllable-state.ts — the controlled/uncontrolled prop machinery | 3.13 |  | 0.784 |
| walker |  | 5143 | 173 | Code::CodeKey { rung: Names, file: src/helpers.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.797 |
| walker |  | 5164 | 21 | Code::CodeKey { rung: Body, file: src/helpers.ts, decl: 6, sub: 0, line: 90 } |  |  | 0.797 |
| walker |  | 5193 | 29 | Code::CodeKey { rung: Doc, file: src/helpers.ts, decl: 8, sub: 0, line: 108 } |  |  | 0.797 |
| walker |  | 5270 | 77 | Code::CodeKey { rung: Body, file: src/helpers.ts, decl: 7, sub: 0, line: 94 } |  |  | 0.797 |
| ns | 5319 |  | 194 | useScaleBackground and use-composed-refs — the two smallest modules | 3.14 |  | 0.789 |
| walker |  | 5355 | 85 | Code::CodeKey { rung: Body, file: src/helpers.ts, decl: 4, sub: 0, line: 59 } |  |  | 0.789 |
| walker |  | 5442 | 87 | Code::CodeKey { rung: Body, file: src/helpers.ts, decl: 8, sub: 0, line: 108 } |  |  | 0.789 |
| ns | 5542 |  | 223 | Every member declared inside Root (names + effect locations) | 4.1 |  | 0.772 |
| walker |  | 5589 | 147 | Code::CodeKey { rung: Doc, file: src/use-position-fixed.ts, decl: 1, sub: 0, line: 15 } |  |  | 0.786 |
| walker |  | 5659 | 70 | Code::CodeKey { rung: Body, file: src/browser.ts, decl: 5, sub: 0, line: 22 } |  |  | 0.786 |
| walker |  | 5696 | 37 | Code::CodeKey { rung: Names, file: src/use-controllable-state.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.787 |
| ns | 5760 |  | 218 | Root's state and refs — the whole drag bookkeeping set | 4.2 |  | 0.769 |
| walker |  | 5807 | 111 | Json::Whole { file: tsconfig.json } |  |  | 0.770 |
| ns | 5867 |  | 107 | Root's useSnapPoints wiring — what it destructures and what it passes | 4.3 |  | 0.759 |
| walker |  | 5918 | 111 | Json::Whole { file: turbo.json } |  |  | 0.760 |
| walker |  | 6001 | 83 | Code::CodeKey { rung: Body, file: src/browser.ts, decl: 1, sub: 0, line: 1 } |  |  | 0.760 |
| ns | 6004 |  | 137 | Scroll-lock and body-position wiring, including the isDisabled predicate | 4.4 |  | 0.748 |
| walker |  | 6117 | 116 | Code::CodeKey { rung: Body, file: src/helpers.ts, decl: 1, sub: 0, line: 9 } |  |  | 0.748 |
| ns | 6214 |  | 210 | shouldDrag: the early-out guards | 4.5 | 4.1 | 0.733 |
| walker |  | 6374 | 257 | Code::CodeKey { rung: Body, file: src/use-controllable-state.ts, decl: 1, sub: 0, line: 39 } |  |  | 0.733 |
| walker |  | 6561 | 187 | Code::CodeKey { rung: Body, file: src/use-prevent-scroll.ts, decl: 4, sub: 0, line: 68 } |  |  | 0.733 |
| ns | 6564 |  | 350 | shouldDrag: direction, open-animation window and scroll-lock timeout | 4.6 | 4.5 | 0.708 |
| ns | 6813 |  | 249 | shouldDrag: the scrollable-ancestor climb | 4.7 | 4.6 | 0.691 |
| walker |  | 6856 | 295 | Code::CodeKey { rung: Body, file: src/index.tsx, decl: 10, sub: 0, line: 1098 } |  |  | 0.692 |
| walker |  | 7002 | 146 | Code::CodeKey { rung: Body, file: src/helpers.ts, decl: 3, sub: 0, line: 42 } |  |  | 0.692 |
| ns | 7042 |  | 229 | onRelease: teardown and velocity computation | 4.8 | 4.1 | 0.680 |
| walker |  | 7282 | 280 | Plaintext::DeclSurface { file: src/style.css } |  |  | 0.680 |
| ns | 7355 |  | 313 | onRelease: the close-vs-snap-back decision ladder | 4.9 | 4.8 | 0.664 |
| walker |  | 7599 | 317 | Code::CodeKey { rung: Body, file: src/index.tsx, decl: 5, sub: 0, line: 803 } |  |  | 0.664 |
| ns | 7627 |  | 272 | The data-vaul-* attributes emitted by Overlay and Content | 4.10 |  | 0.659 |
| ns | 7796 |  | 169 | Handle: click-to-cycle snap points | 4.11 |  | 0.649 |
| ns | 7942 |  | 146 | NestedRoot: how a nested drawer is wired to its parent | 4.12 |  | 0.656 |
| ns | 8092 |  | 150 | The base [data-vaul-drawer] rule and the shape of the variant rules | 5.1 |  | 0.652 |
| walker |  | 8264 | 665 | Code::CodeKey { rung: Body, file: src/use-scale-background.ts, decl: 1, sub: 0, line: 8 } |  |  | 0.656 |
| ns | 8277 |  | 185 | style.css selector inventory — which attribute combinations are styled | 5.2 | 5.1 | 0.651 |
| ns | 8405 |  | 128 | Every @keyframes name in style.css | 5.3 |  | 0.645 |
| walker |  | 8438 | 174 | Code::CodeKey { rung: Body, file: src/helpers.ts, decl: 2, sub: 0, line: 23 } |  |  | 0.645 |
| ns | 8504 |  | 99 | Complete listing of test/src/app — one demo route per feature | 6.1 |  | 0.654 |
| ns | 8570 |  | 66 | Complete listing of test/tests — the Playwright spec set | 6.2 |  | 0.658 |
| ns | 8622 |  | 52 | Listing of the test/ package root | 6.3 |  | 0.662 |
| walker |  | 8669 | 231 | Code::CodeKey { rung: Body, file: src/helpers.ts, decl: 5, sub: 0, line: 72 } |  |  | 0.662 |
| ns | 8915 |  | 293 | Spec-suite to demo-route map for every Playwright file | 6.4 |  | 0.653 |
| ns | 9117 |  | 202 | Playwright runner configuration: server, devices, testDir | 6.5 |  | 0.644 |
| walker |  | 9185 | 516 | Code::CodeKey { rung: Body, file: src/use-position-fixed.ts, decl: 1, sub: 0, line: 15 } |  |  | 0.649 |
| ns | 9223 |  | 106 | Shared e2e helpers: openDrawer and ANIMATION_DURATION | 6.6 |  | 0.645 |
| ns | 9354 |  | 131 | package.json publishing surface: files and the exports map | 7.1 |  | 0.650 |
| walker |  | 9453 | 268 | Code::CodeKey { rung: Body, file: src/use-position-fixed.ts, decl: 1, sub: 1, line: 15 } |  |  | 0.650 |
| ns | 9516 |  | 162 | Runtime and peer dependencies, and the pinned package manager | 7.2 |  | 0.647 |
| ns | 9644 |  | 128 | Workspace and task-runner config: pnpm-workspace.yaml and turbo.json | 7.3 |  | 0.651 |
| ns | 9749 |  | 105 | Root tsconfig.json — compiler settings for the shipped library | 7.4 |  | 0.655 |
| ns | 9753 |  | 4 | Listing of .github/workflows | 7.5 |  | 0.655 |
| ns | 9913 |  | 160 | CI: the Playwright workflow steps | 7.6 |  | 0.649 |
| walker |  | 9953 | 500 | Code::CodeKey { rung: Body, file: src/use-position-fixed.ts, decl: 1, sub: 2, line: 15 } |  |  | 0.651 |
| ns | 9978 |  | 65 | Prettier configuration — the formatting any new code must match | 7.7 |  | 0.648 |
| walker |  | 9998 | 45 | Code::CodeKey { rung: Body, file: src/index.tsx, decl: 9, sub: 0, line: 996 } |  |  | 0.648 |

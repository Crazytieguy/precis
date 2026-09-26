Score(3000)=0.884 I=0.937 C=0.834 ns_rows≤3K=20/61 grid(1000/1442/2080/3000/4327/6240/9000)=0.711/0.747/0.752/0.884/0.798/0.748/0.660

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 62 | 62 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 86 |  | 86 | Package identity: name, version, description, entry points | 1.1 |  | 0.000 |
| walker |  | 125 | 63 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.000 |
| walker |  | 129 | 4 | Fs::DirListing { dir: .vscode } |  |  | 0.000 |
| ns | 148 |  | 62 | Complete repository root listing | 1.2 |  | 0.503 |
| walker |  | 187 | 58 | Json::Identity { file: package.json } |  |  | 0.683 |
| walker |  | 193 | 6 | Fs::DirListing { dir: .github/workflows } |  |  | 0.684 |
| walker |  | 214 | 21 | Json::Runtime { file: package.json } |  |  | 0.684 |
| ns | 219 |  | 71 | Complete src/ listing — the shipped library | 1.3 |  | 0.511 |
| walker |  | 285 | 71 | Fs::DirListing { dir: src } |  |  | 0.743 |
| walker |  | 319 | 34 | Json::Dependencies { file: package.json } |  |  | 0.743 |
| ns | 334 |  | 115 | The `Drawer` namespace object — the entire public component set | 1.4 |  | 0.620 |
| walker |  | 342 | 23 | Plaintext::Whole { file: pnpm-workspace.yaml } |  |  | 0.620 |
| walker |  | 394 | 52 | Fs::DirListing { dir: test } |  |  | 0.622 |
| ns | 402 |  | 68 | README: the unmaintained notice (whole file) | 1.5 |  | 0.618 |
| walker |  | 531 | 137 | Json::Scripts { file: package.json } |  |  | 0.629 |
| ns | 588 |  | 186 | Every top-level export declaration in src/index.tsx (names only) | 1.6 |  | 0.552 |
| walker |  | 703 | 172 | Json::Entry { file: package.json } |  |  | 0.740 |
| ns | 735 |  | 147 | src/index.tsx imports: Radix dialog, style.css, and the first hook modules | 1.7 |  | 0.676 |
| ns | 902 |  | 167 | src/index.tsx imports: the constants set and the remaining hook modules | 1.8 | 1.7 | 0.603 |
| walker |  | 1011 | 308 | Code::CodeKey { rung: Names, file: src/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.712 |
| walker |  | 1024 | 13 | Code::CodeKey { rung: Decl, file: src/index.tsx, decl: 5, sub: 0, line: 803 } |  |  | 0.712 |
| walker |  | 1039 | 15 | Code::CodeKey { rung: Decl, file: src/index.tsx, decl: 10, sub: 0, line: 989 } |  |  | 0.731 |
| ns | 1039 |  | 137 | package.json scripts — build, dev, test, format | 1.9 |  | 0.731 |
| walker |  | 1068 | 29 | Code::CodeKey { rung: Decl, file: src/index.tsx, decl: 11, sub: 0, line: 996 } |  |  | 0.731 |
| ns | 1095 |  | 56 | Snap-point prop union: the fields of WithFadeFromProps / WithoutFadeFromProps | 2.1 | 1.6 | 0.701 |
| walker |  | 1102 | 34 | Code::CodeKey { rung: Decl, file: src/index.tsx, decl: 8, sub: 0, line: 833 } |  |  | 0.701 |
| walker |  | 1203 | 101 | Code::CodeKey { rung: Decl, file: src/index.tsx, decl: 16, sub: 0, line: 1137 } |  |  | 0.793 |
| ns | 1257 |  | 162 | DialogProps declarations, first half (lines 51-85), doc comments elided | 2.2 | 1.6 | 0.721 |
| walker |  | 1323 | 120 | Code::CodeKey { rung: Decl, file: src/index.tsx, decl: 2, sub: 0, line: 40 } |  |  | 0.730 |
| walker |  | 1484 | 161 | Code::CodeKey { rung: Decl, file: src/index.tsx, decl: 1, sub: 0, line: 27 } |  |  | 0.767 |
| ns | 1494 |  | 237 | DialogProps declarations, second half (lines 86-137) plus the union tail | 2.3 | 2.2 | 0.689 |
| walker |  | 1802 | 318 | Code::CodeKey { rung: Decl, file: src/index.tsx, decl: 4, sub: 0, line: 139 } |  |  | 0.702 |
| ns | 1812 |  | 318 | Root's destructured parameter list — every prop's default value | 2.4 | 1.6 | 0.741 |
| ns | 2031 |  | 219 | Snap-point prop documentation (fills 2.1's ellipses) | 2.5 | 2.1 | 0.752 |
| ns | 2198 |  | 167 | Docs for dismissible, modal and direction (fills 2.3's ellipses) | 2.6 | 2.3 | 0.723 |
| ns | 2398 |  | 200 | Docs for closeThreshold, noBodyStyles, setBackgroundColorOnScale, scrollLockTimeout | 2.7 | 2.2 | 0.692 |
| ns | 2490 |  | 92 | Docs for fixed and handleOnly | 2.8 | 2.2 | 0.679 |
| ns | 2661 |  | 171 | Docs for defaultOpen, disablePreventScroll and repositionInputs | 2.9 | 2.3 | 0.657 |
| ns | 2802 |  | 141 | Docs for snapToSequentialPoint and onAnimationEnd — last of the prop docs | 2.10 | 2.3 | 0.641 |
| walker |  | 2972 | 1170 | Code::CodeKey { rung: Decl, file: src/index.tsx, decl: 3, sub: 0, line: 50 } |  |  | 0.921 |
| ns | 2991 |  | 189 | src/constants.ts in full — every tuning constant and its value | 3.1 |  | 0.884 |
| walker |  | 3038 | 66 | Fs::DirListing { dir: test/tests } |  |  | 0.885 |
| ns | 3069 |  | 78 | src/types.ts in full — DrawerDirection, SnapPoint, AnyFunction | 3.2 |  | 0.872 |
| walker |  | 3149 | 111 | Code::CodeKey { rung: Names, file: src/use-prevent-scroll.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.872 |
| walker |  | 3190 | 41 | Code::CodeKey { rung: Decl, file: src/use-prevent-scroll.ts, decl: 2, sub: 0, line: 10 } |  |  | 0.873 |
| walker |  | 3204 | 14 | Code::CodeKey { rung: Names, file: src/use-snap-points.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.873 |
| ns | 3242 |  | 173 | src/helpers.ts — all eight exported helper signatures | 3.3 |  | 0.859 |
| ns | 3354 |  | 112 | src/browser.ts — the complete platform-detection roster | 3.4 |  | 0.846 |
| walker |  | 3453 | 249 | Code::CodeKey { rung: Decl, file: src/use-snap-points.ts, decl: 1, sub: 0, line: 7 } |  |  | 0.851 |
| walker |  | 3467 | 14 | Code::CodeKey { rung: Names, file: src/use-position-fixed.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.851 |
| walker |  | 3591 | 124 | Code::CodeKey { rung: Decl, file: src/use-position-fixed.ts, decl: 1, sub: 0, line: 15 } |  |  | 0.851 |
| ns | 3633 |  | 279 | DrawerContextValue, first half — refs and pointer callbacks | 3.5 |  | 0.823 |
| walker |  | 3691 | 100 | Fs::DirListing { dir: test/src/app } |  |  | 0.825 |
| ns | 3813 |  | 180 | DrawerContextValue, second half — snap points, direction, container | 3.6 | 3.5 | 0.803 |
| walker |  | 3873 | 182 | Code::CodeKey { rung: Names, file: src/helpers.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.819 |
| walker |  | 3890 | 17 | Code::CodeKey { rung: Decl, file: src/helpers.ts, decl: 1, sub: 0, line: 3 } |  |  | 0.819 |
| ns | 3912 |  | 99 | DrawerContext creation and the useDrawerContext accessor | 3.7 | 3.6 | 0.804 |
| walker |  | 3926 | 36 | Code::CodeKey { rung: Names, file: src/context.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.806 |
| ns | 4022 |  | 110 | useSnapPoints: entry point and its complete return surface | 3.8 |  | 0.789 |
| walker |  | 4253 | 327 | Code::CodeKey { rung: Decl, file: src/context.ts, decl: 1, sub: 0, line: 37 } |  |  | 0.790 |
| walker |  | 4268 | 15 | Code::CodeKey { rung: Names, file: src/use-scale-background.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.790 |
| ns | 4271 |  | 249 | useSnapPoints parameter object (fills 3.8's ellipsis) | 3.9 | 3.8 | 0.798 |
| walker |  | 4277 | 9 | Fs::DirListing { dir: test/public } |  |  | 0.798 |
| walker |  | 4330 | 53 | Code::CodeKey { rung: Names, file: src/use-controllable-state.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.798 |
| walker |  | 4374 | 44 | Code::CodeKey { rung: Decl, file: src/use-controllable-state.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.798 |
| walker |  | 4486 | 112 | Code::CodeKey { rung: Names, file: src/browser.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.811 |
| walker |  | 4498 | 12 | Code::CodeKey { rung: Body, file: src/browser.ts, decl: 2, sub: 0, line: 10 } |  |  | 0.811 |
| walker |  | 4511 | 13 | Code::CodeKey { rung: Body, file: src/browser.ts, decl: 3, sub: 0, line: 14 } |  |  | 0.811 |
| ns | 4565 |  | 294 | usePositionFixed: the iOS rationale comment, its signature and return | 3.10 |  | 0.788 |
| walker |  | 4588 | 77 | Code::CodeKey { rung: Names, file: src/use-composed-refs.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.788 |
| walker |  | 4732 | 144 | Code::CodeKey { rung: Names, file: src/constants.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.809 |
| ns | 4758 |  | 193 | use-prevent-scroll.ts exported surface and its provenance | 3.11 |  | 0.806 |
| walker |  | 4772 | 40 | Code::CodeKey { rung: Decl, file: src/constants.ts, decl: 1, sub: 0, line: 1 } |  |  | 0.814 |
| walker |  | 4824 | 52 | Code::CodeKey { rung: Names, file: src/types.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.818 |
| walker |  | 4845 | 21 | Code::CodeKey { rung: Decl, file: src/types.ts, decl: 2, sub: 0, line: 2 } |  |  | 0.824 |
| walker |  | 4860 | 15 | Code::CodeKey { rung: Body, file: src/browser.ts, decl: 6, sub: 0, line: 30 } |  |  | 0.824 |
| walker |  | 4889 | 29 | Code::CodeKey { rung: Doc, file: src/helpers.ts, decl: 9, sub: 0, line: 108 } |  |  | 0.824 |
| ns | 4907 |  | 149 | use-prevent-scroll.ts module-private declarations | 3.12 | 3.11 | 0.812 |
| walker |  | 4914 | 25 | Code::CodeKey { rung: Body, file: src/use-composed-refs.ts, decl: 2, sub: 0, line: 23 } |  |  | 0.812 |
| walker |  | 4937 | 23 | Code::CodeKey { rung: Body, file: src/browser.ts, decl: 4, sub: 0, line: 18 } |  |  | 0.812 |
| walker |  | 4974 | 37 | Code::CodeKey { rung: Doc, file: src/use-composed-refs.ts, decl: 2, sub: 0, line: 23 } |  |  | 0.812 |
| walker |  | 4995 | 21 | Code::CodeKey { rung: Body, file: src/helpers.ts, decl: 7, sub: 0, line: 90 } |  |  | 0.812 |
| walker |  | 5033 | 38 | Code::CodeKey { rung: Doc, file: src/use-composed-refs.ts, decl: 3, sub: 0, line: 31 } |  |  | 0.812 |
| walker |  | 5060 | 27 | Code::CodeKey { rung: Body, file: src/browser.ts, decl: 7, sub: 0, line: 34 } |  |  | 0.812 |
| ns | 5125 |  | 218 | use-controllable-state.ts — the controlled/uncontrolled prop machinery | 3.13 |  | 0.803 |
| walker |  | 5171 | 111 | Json::Whole { file: tsconfig.json } |  |  | 0.804 |
| walker |  | 5282 | 111 | Json::Whole { file: turbo.json } |  |  | 0.805 |
| walker |  | 5317 | 35 | Code::CodeKey { rung: Body, file: src/use-composed-refs.ts, decl: 3, sub: 0, line: 31 } |  |  | 0.805 |
| ns | 5319 |  | 194 | useScaleBackground and use-composed-refs — the two smallest modules | 3.14 |  | 0.797 |
| walker |  | 5375 | 58 | Code::CodeKey { rung: Doc, file: src/use-prevent-scroll.ts, decl: 5, sub: 0, line: 68 } |  |  | 0.797 |
| walker |  | 5416 | 41 | Code::CodeKey { rung: Body, file: src/use-prevent-scroll.ts, decl: 3, sub: 0, line: 29 } |  |  | 0.797 |
| ns | 5542 |  | 223 | Every member declared inside Root (names + effect locations) | 4.1 |  | 0.779 |
| walker |  | 5696 | 280 | Plaintext::DeclSurface { file: src/style.css } |  |  | 0.779 |
| walker |  | 5758 | 62 | Code::CodeKey { rung: Body, file: src/use-prevent-scroll.ts, decl: 6, sub: 0, line: 294 } |  |  | 0.779 |
| ns | 5760 |  | 218 | Root's state and refs — the whole drag bookkeeping set | 4.2 |  | 0.762 |
| walker |  | 5828 | 70 | Code::CodeKey { rung: Body, file: src/browser.ts, decl: 5, sub: 0, line: 22 } |  |  | 0.762 |
| ns | 5867 |  | 107 | Root's useSnapPoints wiring — what it destructures and what it passes | 4.3 |  | 0.751 |
| walker |  | 5887 | 59 | Code::CodeKey { rung: Body, file: src/context.ts, decl: 11, sub: 0, line: 69 } |  |  | 0.762 |
| walker |  | 5964 | 77 | Code::CodeKey { rung: Body, file: src/helpers.ts, decl: 8, sub: 0, line: 94 } |  |  | 0.762 |
| ns | 6004 |  | 137 | Scroll-lock and body-position wiring, including the isDisabled predicate | 4.4 |  | 0.750 |
| walker |  | 6111 | 147 | Code::CodeKey { rung: Doc, file: src/use-position-fixed.ts, decl: 1, sub: 0, line: 15 } |  |  | 0.764 |
| walker |  | 6194 | 83 | Code::CodeKey { rung: Body, file: src/browser.ts, decl: 1, sub: 0, line: 1 } |  |  | 0.764 |
| ns | 6214 |  | 210 | shouldDrag: the early-out guards | 4.5 | 4.1 | 0.748 |
| walker |  | 6279 | 85 | Code::CodeKey { rung: Body, file: src/helpers.ts, decl: 5, sub: 0, line: 59 } |  |  | 0.748 |
| walker |  | 6371 | 92 | Code::CodeKey { rung: Body, file: src/use-prevent-scroll.ts, decl: 4, sub: 0, line: 34 } |  |  | 0.748 |
| walker |  | 6458 | 87 | Code::CodeKey { rung: Body, file: src/helpers.ts, decl: 9, sub: 0, line: 108 } |  |  | 0.748 |
| walker |  | 6518 | 60 | Code::CodeKey { rung: Body, file: src/index.tsx, decl: 15, sub: 0, line: 1130 } |  |  | 0.748 |
| ns | 6564 |  | 350 | shouldDrag: direction, open-animation window and scroll-lock timeout | 4.6 | 4.5 | 0.722 |
| walker |  | 6775 | 257 | Code::CodeKey { rung: Body, file: src/use-controllable-state.ts, decl: 2, sub: 0, line: 39 } |  |  | 0.722 |
| ns | 6813 |  | 249 | shouldDrag: the scrollable-ancestor climb | 4.7 | 4.6 | 0.705 |
| walker |  | 6891 | 116 | Code::CodeKey { rung: Body, file: src/helpers.ts, decl: 2, sub: 0, line: 9 } |  |  | 0.705 |
| ns | 7042 |  | 229 | onRelease: teardown and velocity computation | 4.8 | 4.1 | 0.693 |
| walker |  | 7078 | 187 | Code::CodeKey { rung: Body, file: src/use-prevent-scroll.ts, decl: 5, sub: 0, line: 68 } |  |  | 0.693 |
| walker |  | 7224 | 146 | Code::CodeKey { rung: Body, file: src/helpers.ts, decl: 4, sub: 0, line: 42 } |  |  | 0.693 |
| ns | 7355 |  | 313 | onRelease: the close-vs-snap-back decision ladder | 4.9 | 4.8 | 0.676 |
| ns | 7627 |  | 272 | The data-vaul-* attributes emitted by Overlay and Content | 4.10 |  | 0.666 |
| ns | 7796 |  | 169 | Handle: click-to-cycle snap points | 4.11 |  | 0.657 |
| walker |  | 7889 | 665 | Code::CodeKey { rung: Body, file: src/use-scale-background.ts, decl: 1, sub: 0, line: 8 } |  |  | 0.661 |
| ns | 7942 |  | 146 | NestedRoot: how a nested drawer is wired to its parent | 4.12 |  | 0.653 |
| walker |  | 8063 | 174 | Code::CodeKey { rung: Body, file: src/helpers.ts, decl: 3, sub: 0, line: 23 } |  |  | 0.653 |
| ns | 8092 |  | 150 | The base [data-vaul-drawer] rule and the shape of the variant rules | 5.1 |  | 0.649 |
| walker |  | 8103 | 40 | Json::Identity { file: test/package.json } |  |  | 0.649 |
| walker |  | 8166 | 63 | Json::Scripts { file: test/package.json } |  |  | 0.649 |
| walker |  | 8275 | 109 | Json::Dependencies { file: test/package.json } |  |  | 0.649 |
| ns | 8277 |  | 185 | style.css selector inventory — which attribute combinations are styled | 5.2 | 5.1 | 0.644 |
| ns | 8405 |  | 128 | Every @keyframes name in style.css | 5.3 |  | 0.638 |
| ns | 8504 |  | 99 | Complete listing of test/src/app — one demo route per feature | 6.1 |  | 0.647 |
| walker |  | 8506 | 231 | Code::CodeKey { rung: Body, file: src/helpers.ts, decl: 6, sub: 0, line: 72 } |  |  | 0.647 |
| ns | 8570 |  | 66 | Complete listing of test/tests — the Playwright spec set | 6.2 |  | 0.651 |
| ns | 8622 |  | 52 | Listing of the test/ package root | 6.3 |  | 0.656 |
| walker |  | 8801 | 295 | Code::CodeKey { rung: Body, file: src/index.tsx, decl: 13, sub: 0, line: 1098 } |  |  | 0.670 |
| ns | 8915 |  | 293 | Spec-suite to demo-route map for every Playwright file | 6.4 |  | 0.660 |
| ns | 9117 |  | 202 | Playwright runner configuration: server, devices, testDir | 6.5 |  | 0.651 |
| walker |  | 9118 | 317 | Code::CodeKey { rung: Body, file: src/index.tsx, decl: 5, sub: 0, line: 803 } |  |  | 0.655 |
| ns | 9223 |  | 106 | Shared e2e helpers: openDrawer and ANIMATION_DURATION | 6.6 |  | 0.652 |
| ns | 9354 |  | 131 | package.json publishing surface: files and the exports map | 7.1 |  | 0.657 |
| ns | 9516 |  | 162 | Runtime and peer dependencies, and the pinned package manager | 7.2 |  | 0.653 |
| walker |  | 9634 | 516 | Code::CodeKey { rung: Body, file: src/use-position-fixed.ts, decl: 1, sub: 0, line: 15 } |  |  | 0.658 |
| ns | 9644 |  | 128 | Workspace and task-runner config: pnpm-workspace.yaml and turbo.json | 7.3 |  | 0.663 |
| walker |  | 9665 | 31 | Code::CodeKey { rung: Names, file: test/tests/helpers.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.663 |
| walker |  | 9724 | 59 | Code::CodeKey { rung: Decl, file: test/tests/helpers.ts, decl: 2, sub: 0, line: 11 } |  |  | 0.663 |
| walker |  | 9737 | 13 | Code::CodeKey { rung: Names, file: test/tests/constants.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.663 |
| ns | 9749 |  | 105 | Root tsconfig.json — compiler settings for the shipped library | 7.4 |  | 0.666 |
| walker |  | 9751 | 14 | Code::CodeKey { rung: Names, file: test/src/app/page.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.666 |
| ns | 9753 |  | 4 | Listing of .github/workflows | 7.5 |  | 0.666 |
| walker |  | 9756 | 5 | Fs::DirListing { dir: test/src/app/controlled } |  |  | 0.666 |
| walker |  | 9761 | 5 | Fs::DirListing { dir: test/src/app/default-open } |  |  | 0.666 |
| walker |  | 9766 | 5 | Fs::DirListing { dir: test/src/app/different-directions } |  |  | 0.666 |
| walker |  | 9771 | 5 | Fs::DirListing { dir: test/src/app/initial-snap } |  |  | 0.666 |
| walker |  | 9776 | 5 | Fs::DirListing { dir: test/src/app/nested-drawers } |  |  | 0.666 |
| walker |  | 9781 | 5 | Fs::DirListing { dir: test/src/app/non-dismissible } |  |  | 0.666 |
| walker |  | 9786 | 5 | Fs::DirListing { dir: test/src/app/open-another-drawer } |  |  | 0.666 |
| walker |  | 9791 | 5 | Fs::DirListing { dir: test/src/app/parent-container } |  |  | 0.666 |
| walker |  | 9796 | 5 | Fs::DirListing { dir: test/src/app/scrollable-page } |  |  | 0.666 |
| walker |  | 9801 | 5 | Fs::DirListing { dir: test/src/app/scrollable-with-inputs } |  |  | 0.666 |
| walker |  | 9806 | 5 | Fs::DirListing { dir: test/src/app/with-handle } |  |  | 0.666 |
| walker |  | 9811 | 5 | Fs::DirListing { dir: test/src/app/with-modal-false } |  |  | 0.666 |
| walker |  | 9816 | 5 | Fs::DirListing { dir: test/src/app/with-scaled-background } |  |  | 0.666 |
| walker |  | 9821 | 5 | Fs::DirListing { dir: test/src/app/with-snap-points } |  |  | 0.666 |
| walker |  | 9826 | 5 | Fs::DirListing { dir: test/src/app/without-scaled-background } |  |  | 0.666 |
| walker |  | 9863 | 37 | Code::CodeKey { rung: Names, file: test/src/app/layout.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.666 |
| walker |  | 9892 | 29 | Code::CodeKey { rung: Decl, file: test/src/app/layout.tsx, decl: 1, sub: 0, line: 7 } |  |  | 0.666 |
| ns | 9913 |  | 160 | CI: the Playwright workflow steps | 7.6 |  | 0.661 |
| ns | 9978 |  | 65 | Prettier configuration — the formatting any new code must match | 7.7 |  | 0.658 |

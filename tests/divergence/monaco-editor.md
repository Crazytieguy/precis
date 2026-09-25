Score(3000)=0.715 I=0.808 C=0.632 ns_rows≤3K=23/56 grid(1000/1442/2080/3000/4327/6240/9000)=0.659/0.695/0.782/0.715/0.639/0.546/0.496

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 67 |  | 67 | README title + one-sentence identity | 1.1 |  | 0.000 |
| ns | 92 |  | 25 | src/ listing — every child of the package source directory | 1.2 |  | 0.000 |
| walker |  | 116 | 116 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 122 | 6 | Fs::DirListing { dir: test-results } |  |  | 0.000 |
| walker |  | 147 | 25 | Fs::DirListing { dir: src } |  |  | 0.801 |
| walker |  | 150 | 3 | Fs::DirListing { dir: src/internal } |  |  | 0.801 |
| walker |  | 161 | 11 | Fs::DirListing { dir: src/deprecated } |  |  | 0.802 |
| walker |  | 172 | 11 | Fs::DirListing { dir: src/languages } |  |  | 0.805 |
| walker |  | 185 | 13 | Fs::DirListing { dir: src/deprecated/language } |  |  | 0.811 |
| walker |  | 206 | 21 | Fs::DirListing { dir: src/languages/features } |  |  | 0.826 |
| ns | 216 |  | 124 | src/index.ts — the whole package entry point (9 lines) | 1.3 |  | 0.525 |
| walker |  | 230 | 24 | Fs::DirListing { dir: src/languages/features/css } |  |  | 0.525 |
| walker |  | 254 | 24 | Fs::DirListing { dir: src/languages/features/html } |  |  | 0.526 |
| walker |  | 281 | 27 | Fs::DirListing { dir: monaco-lsp-client } |  |  | 0.526 |
| walker |  | 285 | 4 | Fs::DirListing { dir: monaco-lsp-client/generator } |  |  | 0.526 |
| walker |  | 300 | 15 | Fs::DirListing { dir: monaco-lsp-client/src } |  |  | 0.527 |
| walker |  | 329 | 29 | Fs::DirListing { dir: src/languages/features/json } |  |  | 0.528 |
| ns | 332 |  | 116 | Complete repository root listing | 1.4 |  | 0.659 |
| walker |  | 358 | 29 | Fs::DirListing { dir: webpack-plugin } |  |  | 0.660 |
| walker |  | 378 | 20 | Fs::DirListing { dir: webpack-plugin/src } |  |  | 0.661 |
| walker |  | 382 | 4 | Fs::DirListing { dir: webpack-plugin/src/loaders } |  |  | 0.661 |
| walker |  | 389 | 7 | Fs::DirListing { dir: src/deprecated/basic-languages } |  |  | 0.661 |
| walker |  | 421 | 32 | Fs::DirListing { dir: src/languages/features/typescript } |  |  | 0.664 |
| walker |  | 426 | 5 | Fs::DirListing { dir: .devcontainer } |  |  | 0.664 |
| walker |  | 431 | 5 | Fs::DirListing { dir: .husky } |  |  | 0.664 |
| walker |  | 439 | 8 | Fs::DirListing { dir: src/internal/common } |  |  | 0.669 |
| walker |  | 447 | 8 | Fs::DirListing { dir: webpack-plugin/src/plugins } |  |  | 0.669 |
| ns | 471 |  | 139 | package.json identity header (name, version, vscodeRef, license) | 1.5 |  | 0.605 |
| walker |  | 485 | 38 | Fs::DirListing { dir: docs } |  |  | 0.605 |
| walker |  | 491 | 6 | Fs::DirListing { dir: scripts } |  |  | 0.605 |
| walker |  | 501 | 10 | Fs::DirListing { dir: src/deprecated/editor } |  |  | 0.605 |
| walker |  | 571 | 70 | Json::Identity { file: package.json } |  |  | 0.628 |
| walker |  | 578 | 7 | Fs::DirListing { dir: src/languages/features/common } |  |  | 0.628 |
| ns | 588 |  | 117 | The three one-to-six-line re-export files src/index.ts depends on | 1.6 | 1.2 | 0.577 |
| walker |  | 645 | 67 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.740 |
| walker |  | 649 | 4 | Fs::DirListing { dir: scripts/lib } |  |  | 0.740 |
| ns | 734 |  | 146 | All README section headings (locations) | 1.7 |  | 0.656 |
| walker |  | 921 | 272 | Fs::DirListing { dir: src/features } |  |  | 0.690 |
| ns | 927 |  | 193 | CONTRIBUTING 'Source Code Structure' — the monaco-editor-core boundary | 1.8 |  | 0.657 |
| ns | 1097 |  | 170 | README Concepts: Models and URIs | 1.9 | 1.7 | 0.649 |
| walker |  | 1225 | 304 | Fs::DirListing { dir: src/languages/definitions } |  |  | 0.674 |
| ns | 1259 |  | 162 | README Concepts: Editors, Providers, Disposables | 1.10 | 1.7 | 0.661 |
| walker |  | 1273 | 48 | Json::Identity { file: monaco-lsp-client/package.json } |  |  | 0.661 |
| walker |  | 1285 | 12 | Fs::DirListing { dir: .azure-pipelines } |  |  | 0.661 |
| walker |  | 1297 | 12 | Fs::DirListing { dir: .vscode } |  |  | 0.661 |
| walker |  | 1309 | 12 | Fs::DirListing { dir: src/deprecated/language/css } |  |  | 0.661 |
| walker |  | 1321 | 12 | Fs::DirListing { dir: src/deprecated/language/html } |  |  | 0.661 |
| ns | 1326 |  | 67 | Listings of every src/ subtree except the two huge uniform ones | 2.1 |  | 0.712 |
| walker |  | 1333 | 12 | Fs::DirListing { dir: src/deprecated/language/json } |  |  | 0.712 |
| walker |  | 1345 | 12 | Fs::DirListing { dir: src/deprecated/language/typescript } |  |  | 0.712 |
| walker |  | 1358 | 13 | Fs::DirListing { dir: test } |  |  | 0.712 |
| ns | 1432 |  | 106 | Uniform shape of a feature shim (hover, gpu) and the one that is not uniform (find) | 2.2 |  | 0.694 |
| walker |  | 1462 | 104 | Fs::DirListing { dir: samples } |  |  | 0.695 |
| walker |  | 1466 | 4 | Fs::DirListing { dir: samples/nwjs-amd } |  |  | 0.695 |
| walker |  | 1470 | 4 | Fs::DirListing { dir: samples/nwjs-amd-v2 } |  |  | 0.695 |
| walker |  | 1494 | 24 | Fs::DirListing { dir: samples/browser-esm-esbuild } |  |  | 0.695 |
| walker |  | 1519 | 25 | Fs::DirListing { dir: samples/browser-esm-webpack } |  |  | 0.695 |
| walker |  | 1544 | 25 | Fs::DirListing { dir: samples/browser-esm-webpack-monaco-plugin } |  |  | 0.695 |
| walker |  | 1569 | 25 | Fs::DirListing { dir: samples/electron-esm-webpack } |  |  | 0.695 |
| walker |  | 1600 | 31 | Fs::DirListing { dir: samples/browser-esm-webpack-small } |  |  | 0.695 |
| walker |  | 1621 | 21 | Fs::DirListing { dir: samples/browser-esm-parcel } |  |  | 0.695 |
| walker |  | 1629 | 8 | Fs::DirListing { dir: samples/browser-esm-parcel/src } |  |  | 0.695 |
| walker |  | 1644 | 15 | Json::Identity { file: samples/browser-esm-esbuild/package.json } |  |  | 0.695 |
| walker |  | 1659 | 15 | Json::Identity { file: samples/browser-esm-parcel/package.json } |  |  | 0.695 |
| walker |  | 1681 | 22 | Fs::DirListing { dir: samples/browser-esm-webpack-typescript } |  |  | 0.695 |
| walker |  | 1689 | 8 | Fs::DirListing { dir: samples/browser-esm-webpack-typescript/src } |  |  | 0.695 |
| ns | 1704 |  | 272 | Complete listing of src/features/ — all 65 editor-feature shim directories | 2.3 |  | 0.769 |
| walker |  | 1706 | 17 | Json::Identity { file: samples/browser-esm-webpack/package.json } |  |  | 0.769 |
| walker |  | 1723 | 17 | Json::Identity { file: samples/electron-esm-webpack/package.json } |  |  | 0.769 |
| walker |  | 1741 | 18 | Json::Identity { file: samples/browser-esm-webpack-small/package.json } |  |  | 0.769 |
| ns | 1748 |  | 44 | src/features/register.all.ts head, elided middle, tail | 2.4 |  | 0.757 |
| walker |  | 1768 | 27 | Fs::DirListing { dir: samples/browser-esm-webpack-typescript-react } |  |  | 0.757 |
| walker |  | 1784 | 16 | Fs::DirListing { dir: samples/browser-esm-webpack-typescript-react/src } |  |  | 0.757 |
| walker |  | 1789 | 5 | Fs::DirListing { dir: samples/browser-esm-webpack-typescript-react/src/components } |  |  | 0.757 |
| walker |  | 1809 | 20 | Json::Identity { file: samples/browser-esm-webpack-monaco-plugin/package.json } |  |  | 0.757 |
| walker |  | 1829 | 20 | Json::Identity { file: samples/browser-esm-webpack-typescript/package.json } |  |  | 0.757 |
| walker |  | 1849 | 20 | Json::Identity { file: samples/browser-esm-webpack-typescript-react/package.json } |  |  | 0.757 |
| walker |  | 1880 | 31 | Fs::DirListing { dir: samples/browser-esm-vite-react } |  |  | 0.757 |
| walker |  | 1899 | 19 | Json::Identity { file: samples/browser-esm-vite-react/package.json } |  |  | 0.757 |
| walker |  | 1935 | 36 | Fs::DirListing { dir: samples/browser-esm-vite } |  |  | 0.757 |
| walker |  | 1953 | 18 | Json::Identity { file: samples/browser-esm-vite/package.json } |  |  | 0.757 |
| walker |  | 1972 | 19 | Fs::DirListing { dir: samples/browser-esm-vite-react/src } |  |  | 0.757 |
| walker |  | 1982 | 10 | Fs::DirListing { dir: samples/browser-esm-vite-react/src/components } |  |  | 0.757 |
| walker |  | 2044 | 62 | Json::Identity { file: webpack-plugin/package.json } |  |  | 0.757 |
| ns | 2052 |  | 304 | Complete listing of src/languages/definitions/ — all 84 Monarch languages plus _.contribution.ts, register.all.ts and test/ | 2.5 |  | 0.787 |
| ns | 2065 |  | 13 | One language definition directory, listed in full (rust) | 2.6 |  | 0.782 |
| walker |  | 2115 | 71 | Json::Identity { file: samples/package.json } |  |  | 0.782 |
| walker |  | 2127 | 12 | Json::Entry { file: samples/package.json } |  |  | 0.782 |
| walker |  | 2147 | 20 | Fs::DirListing { dir: .github } |  |  | 0.782 |
| ns | 2149 |  | 84 | rust/register.ts registerLanguage call | 2.7 | 2.6 | 0.769 |
| walker |  | 2155 | 8 | Fs::DirListing { dir: .github/workflows } |  |  | 0.769 |
| walker |  | 2192 | 37 | Fs::DirListing { dir: monaco-lsp-client/src/adapters } |  |  | 0.770 |
| walker |  | 2203 | 11 | Json::Entry { file: samples/electron-esm-webpack/package.json } |  |  | 0.770 |
| walker |  | 2285 | 82 | Fs::DirListing { dir: samples/legacy } |  |  | 0.770 |
| walker |  | 2289 | 4 | Fs::DirListing { dir: samples/legacy/browser-amd-editor } |  |  | 0.770 |
| walker |  | 2293 | 4 | Fs::DirListing { dir: samples/legacy/browser-amd-localized } |  |  | 0.770 |
| walker |  | 2297 | 4 | Fs::DirListing { dir: samples/legacy/browser-amd-monarch } |  |  | 0.770 |
| walker |  | 2301 | 4 | Fs::DirListing { dir: samples/legacy/browser-amd-requirejs } |  |  | 0.770 |
| walker |  | 2305 | 4 | Fs::DirListing { dir: samples/legacy/browser-amd-shadow-dom } |  |  | 0.770 |
| walker |  | 2309 | 4 | Fs::DirListing { dir: samples/legacy/browser-amd-shared-model } |  |  | 0.770 |
| walker |  | 2313 | 4 | Fs::DirListing { dir: samples/legacy/browser-amd-trusted-types } |  |  | 0.770 |
| walker |  | 2321 | 8 | Fs::DirListing { dir: samples/legacy/browser-amd-iframe } |  |  | 0.770 |
| walker |  | 2333 | 12 | Fs::DirListing { dir: samples/legacy/browser-amd-diff-editor } |  |  | 0.770 |
| walker |  | 2350 | 17 | Fs::DirListing { dir: samples/legacy/electron-amd } |  |  | 0.770 |
| walker |  | 2367 | 17 | Fs::DirListing { dir: samples/legacy/electron-amd-nodeIntegration } |  |  | 0.770 |
| walker |  | 2383 | 16 | Json::Identity { file: samples/legacy/electron-amd/package.json } |  |  | 0.770 |
| walker |  | 2402 | 19 | Json::Identity { file: samples/legacy/electron-amd-nodeIntegration/package.json } |  |  | 0.770 |
| walker |  | 2429 | 27 | Json::Entry { file: monaco-lsp-client/package.json } |  |  | 0.770 |
| ns | 2431 |  | 282 | _.contribution.ts: ILang/ILangImpl and registerLanguage() | 2.8 |  | 0.734 |
| walker |  | 2457 | 28 | Json::Entry { file: webpack-plugin/package.json } |  |  | 0.734 |
| walker |  | 2496 | 39 | Json::Scripts { file: samples/package.json } |  |  | 0.734 |
| ns | 2570 |  | 139 | _.contribution.ts: LazyLanguageLoader and loadLanguage() | 2.9 |  | 0.718 |
| walker |  | 2642 | 146 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.757 |
| ns | 2659 |  | 89 | Grammar file shape: rust.ts conf + language exports | 2.10 |  | 0.747 |
| walker |  | 2686 | 44 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.747 |
| walker |  | 2699 | 13 | Json::Entry { file: samples/legacy/electron-amd/package.json } |  |  | 0.747 |
| walker |  | 2712 | 13 | Json::Entry { file: samples/legacy/electron-amd-nodeIntegration/package.json } |  |  | 0.747 |
| walker |  | 2755 | 43 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.747 |
| ns | 2799 |  | 140 | Tokenization test harness: testRunner.ts types + testTokenization(), and a test file head | 2.11 |  | 0.730 |
| walker |  | 2815 | 60 | Fs::DirListing { dir: website } |  |  | 0.731 |
| ns | 2841 |  | 42 | src/languages/definitions/register.all.ts head, elided middle, tail | 2.12 |  | 0.725 |
| walker |  | 2895 | 80 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.725 |
| walker |  | 2918 | 23 | Fs::DirListing { dir: scripts/ci } |  |  | 0.725 |
| walker |  | 2953 | 35 | Fs::DirListing { dir: src/languages/features/typescript/lib } |  |  | 0.727 |
| ns | 2976 |  | 135 | src/deprecated/: the legacy import paths, all one-line re-exports | 2.13 |  | 0.714 |
| walker |  | 2979 | 26 | Fs::DirListing { dir: website/src } |  |  | 0.715 |
| walker |  | 2991 | 12 | Fs::DirListing { dir: website/src/runner } |  |  | 0.715 |
| walker |  | 3285 | 294 | Code::CodeKey { rung: Names, file: monaco-lsp-client/generator/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.715 |
| walker |  | 3296 | 11 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 9, sub: 0, line: 82 } |  |  | 0.715 |
| walker |  | 3314 | 18 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 32, sub: 0, line: 259 } |  |  | 0.715 |
| walker |  | 3335 | 21 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 21, sub: 0, line: 197 } |  |  | 0.715 |
| ns | 3345 |  | 369 | package.json scripts — build, test and dev commands (packaging variants elided) | 3.1 |  | 0.694 |
| walker |  | 3356 | 21 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 22, sub: 0, line: 202 } |  |  | 0.694 |
| walker |  | 3377 | 21 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 24, sub: 0, line: 213 } |  |  | 0.694 |
| walker |  | 3398 | 21 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 25, sub: 0, line: 218 } |  |  | 0.694 |
| walker |  | 3419 | 21 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 26, sub: 0, line: 223 } |  |  | 0.694 |
| walker |  | 3441 | 22 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 27, sub: 0, line: 228 } |  |  | 0.694 |
| walker |  | 3463 | 22 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 29, sub: 0, line: 241 } |  |  | 0.694 |
| ns | 3477 |  | 132 | CONTRIBUTING: the ordered editor-test command sequence | 3.2 |  | 0.682 |
| walker |  | 3485 | 22 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 30, sub: 0, line: 246 } |  |  | 0.682 |
| walker |  | 3509 | 24 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 31, sub: 0, line: 251 } |  |  | 0.682 |
| walker |  | 3533 | 24 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 34, sub: 0, line: 684 } |  |  | 0.682 |
| ns | 3541 |  | 64 | test/ listing plus the full smoke-test directory | 3.3 |  | 0.668 |
| walker |  | 3562 | 29 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 23, sub: 0, line: 207 } |  |  | 0.668 |
| walker |  | 3593 | 31 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 16, sub: 0, line: 154 } |  |  | 0.668 |
| walker |  | 3640 | 47 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 28, sub: 0, line: 233 } |  |  | 0.668 |
| ns | 3692 |  | 151 | Listings of the five src/languages/features dirs — the uniform rich-service file set | 4.1 |  | 0.682 |
| walker |  | 3695 | 55 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 17, sub: 0, line: 159 } |  |  | 0.682 |
| walker |  | 3752 | 57 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 15, sub: 0, line: 145 } |  |  | 0.682 |
| walker |  | 3812 | 60 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 8, sub: 0, line: 73 } |  |  | 0.682 |
| walker |  | 3876 | 64 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 13, sub: 0, line: 124 } |  |  | 0.682 |
| walker |  | 3950 | 74 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 12, sub: 0, line: 113 } |  |  | 0.682 |
| ns | 3952 |  | 260 | css/register.ts — every top-level export (names only) | 4.2 |  | 0.666 |
| walker |  | 4027 | 77 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 14, sub: 0, line: 134 } |  |  | 0.666 |
| walker |  | 4106 | 79 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 1, sub: 0, line: 7 } |  |  | 0.666 |
| walker |  | 4193 | 87 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 11, sub: 0, line: 101 } |  |  | 0.666 |
| ns | 4237 |  | 285 | css: the default ModeConfiguration toggles, the three defaults objects, and the lazy onLanguage hookup | 4.3 | 4.2 | 0.639 |
| walker |  | 4294 | 101 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 19, sub: 0, line: 170 } |  |  | 0.639 |
| walker |  | 4406 | 112 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 20, sub: 0, line: 183 } |  |  | 0.639 |
| ns | 4496 |  | 259 | json/register.ts — every top-level export (names only) | 4.4 |  | 0.624 |
| walker |  | 4520 | 114 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 10, sub: 0, line: 86 } |  |  | 0.624 |
| walker |  | 4550 | 30 | Json::Scripts { file: samples/browser-esm-esbuild/package.json } |  |  | 0.624 |
| walker |  | 4581 | 31 | Json::Scripts { file: samples/browser-esm-parcel/package.json } |  |  | 0.624 |
| walker |  | 4593 | 12 | Code::CodeKey { rung: Names, file: samples/browser-esm-vite-react/src/main.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.624 |
| walker |  | 4637 | 44 | Code::CodeKey { rung: Decl, file: samples/browser-esm-vite-react/src/main.tsx, decl: 1, sub: 0, line: 6 } |  |  | 0.624 |
| walker |  | 4649 | 12 | Code::CodeKey { rung: Names, file: samples/browser-esm-webpack-typescript-react/src/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.624 |
| walker |  | 4693 | 44 | Code::CodeKey { rung: Decl, file: samples/browser-esm-webpack-typescript-react/src/index.tsx, decl: 1, sub: 0, line: 7 } |  |  | 0.624 |
| walker |  | 4704 | 11 | Json::Dependencies { file: samples/browser-esm-vite/package.json } |  |  | 0.624 |
| walker |  | 4715 | 11 | Json::Dependencies { file: samples/browser-esm-vite-react/package.json } |  |  | 0.624 |
| walker |  | 4726 | 11 | Json::Dependencies { file: samples/browser-esm-webpack-typescript-react/package.json } |  |  | 0.624 |
| ns | 4765 |  | 269 | json defaults: diagnostic option values, mode toggles, jsonDefaults, the worker interface | 4.5 | 4.4 | 0.604 |
| walker |  | 4795 | 69 | Json::Scripts { file: webpack-plugin/package.json } |  |  | 0.604 |
| walker |  | 4963 | 168 | Json::Entry { file: package.json } |  |  | 0.605 |
| walker |  | 4991 | 28 | Code::CodeKey { rung: Names, file: samples/browser-esm-webpack-typescript/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.605 |
| ns | 5027 |  | 262 | html/register.ts — export names plus the registerHTMLLanguageService factory | 4.6 |  | 0.591 |
| walker |  | 5032 | 41 | Code::CodeKey { rung: Decl, file: samples/browser-esm-webpack-typescript/src/index.ts, decl: 2, sub: 0, line: 23 } |  |  | 0.591 |
| walker |  | 5229 | 197 | Code::CodeKey { rung: Decl, file: samples/browser-esm-webpack-typescript/src/index.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.591 |
| walker |  | 5267 | 38 | Json::Scripts { file: samples/browser-esm-webpack/package.json } |  |  | 0.591 |
| walker |  | 5305 | 38 | Json::Scripts { file: samples/browser-esm-webpack-monaco-plugin/package.json } |  |  | 0.591 |
| ns | 5311 |  | 284 | html: the three pre-registered services and their per-language toggles | 4.7 | 4.6 | 0.577 |
| walker |  | 5407 | 102 | Code::CodeKey { rung: Names, file: samples/electron-esm-webpack/main.js, decl: 0, sub: 0, line: 0 } |  |  | 0.577 |
| walker |  | 5436 | 29 | Code::CodeKey { rung: Decl, file: samples/electron-esm-webpack/main.js, decl: 7, sub: 0, line: 28 } |  |  | 0.577 |
| walker |  | 5467 | 31 | Code::CodeKey { rung: Decl, file: samples/electron-esm-webpack/main.js, decl: 6, sub: 0, line: 22 } |  |  | 0.577 |
| ns | 5556 |  | 245 | typescript/register.ts — every top-level export (names only) | 4.8 |  | 0.566 |
| walker |  | 5569 | 102 | Code::CodeKey { rung: Names, file: samples/legacy/electron-amd/main.js, decl: 0, sub: 0, line: 0 } |  |  | 0.566 |
| walker |  | 5598 | 29 | Code::CodeKey { rung: Decl, file: samples/legacy/electron-amd/main.js, decl: 7, sub: 0, line: 28 } |  |  | 0.566 |
| walker |  | 5629 | 31 | Code::CodeKey { rung: Decl, file: samples/legacy/electron-amd/main.js, decl: 6, sub: 0, line: 22 } |  |  | 0.566 |
| walker |  | 5731 | 102 | Code::CodeKey { rung: Names, file: samples/legacy/electron-amd-nodeIntegration/main.js, decl: 0, sub: 0, line: 0 } |  |  | 0.566 |
| walker |  | 5760 | 29 | Code::CodeKey { rung: Decl, file: samples/legacy/electron-amd-nodeIntegration/main.js, decl: 7, sub: 0, line: 33 } |  |  | 0.566 |
| walker |  | 5791 | 31 | Code::CodeKey { rung: Decl, file: samples/legacy/electron-amd-nodeIntegration/main.js, decl: 6, sub: 0, line: 27 } |  |  | 0.555 |
| ns | 5791 |  | 235 | LanguageServiceDefaults (typescript) — all members | 4.9 |  | 0.555 |
| walker |  | 5835 | 44 | Code::CodeKey { rung: Names, file: webpack-plugin/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.555 |
| walker |  | 5881 | 46 | Code::CodeKey { rung: Decl, file: webpack-plugin/src/index.ts, decl: 2, sub: 0, line: 159 } |  |  | 0.555 |
| walker |  | 5911 | 30 | Code::CodeKey { rung: Names, file: src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.557 |
| walker |  | 5942 | 31 | Code::CodeKey { rung: Names, file: samples/browser-esm-webpack-monaco-plugin/index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.557 |
| walker |  | 5982 | 40 | Code::CodeKey { rung: Decl, file: samples/browser-esm-webpack-monaco-plugin/index.js, decl: 2, sub: 0, line: 12 } |  |  | 0.557 |
| ns | 6027 |  | 236 | TypeScriptWorker — all 21 proxy methods (names only) | 4.10 | 4.8 | 0.546 |
| walker |  | 6066 | 84 | Code::CodeKey { rung: Decl, file: samples/browser-esm-webpack-monaco-plugin/index.js, decl: 1, sub: 0, line: 3 } |  |  | 0.546 |
| walker |  | 6148 | 82 | Json::Scripts { file: monaco-lsp-client/package.json } |  |  | 0.546 |
| walker |  | 6181 | 33 | Code::CodeKey { rung: Names, file: samples/browser-esm-esbuild/index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.546 |
| walker |  | 6222 | 41 | Code::CodeKey { rung: Decl, file: samples/browser-esm-esbuild/index.js, decl: 2, sub: 0, line: 21 } |  |  | 0.546 |
| ns | 6318 |  | 291 | typescript/javascript defaults and their divergence | 4.11 | 4.8 | 0.533 |
| walker |  | 6434 | 212 | Code::CodeKey { rung: Decl, file: samples/browser-esm-esbuild/index.js, decl: 1, sub: 0, line: 3 } |  |  | 0.533 |
| walker |  | 6467 | 33 | Code::CodeKey { rung: Names, file: samples/browser-esm-parcel/src/index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.533 |
| walker |  | 6508 | 41 | Code::CodeKey { rung: Decl, file: samples/browser-esm-parcel/src/index.js, decl: 2, sub: 0, line: 26 } |  |  | 0.533 |
| ns | 6547 |  | 229 | The four *Mode.ts entry points and the tsMode re-export tail | 5.1 |  | 0.528 |
| walker |  | 6690 | 182 | Code::CodeKey { rung: Decl, file: samples/browser-esm-parcel/src/index.js, decl: 1, sub: 0, line: 8 } |  |  | 0.528 |
| walker |  | 6723 | 33 | Code::CodeKey { rung: Names, file: samples/browser-esm-webpack/index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.528 |
| ns | 6761 |  | 214 | cssMode.setupMode — WorkerManager, the worker accessor, and ModeConfiguration-gated provider registration | 5.2 | 5.1 | 0.519 |
| walker |  | 6764 | 41 | Code::CodeKey { rung: Decl, file: samples/browser-esm-webpack/index.js, decl: 2, sub: 0, line: 21 } |  |  | 0.519 |
| walker |  | 6961 | 197 | Code::CodeKey { rung: Decl, file: samples/browser-esm-webpack/index.js, decl: 1, sub: 0, line: 3 } |  |  | 0.519 |
| walker |  | 6994 | 33 | Code::CodeKey { rung: Names, file: samples/browser-esm-webpack-small/index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.519 |
| ns | 7114 |  | 353 | common/lspLanguageFeatures.ts — the shared provider adapters and LSP conversion helpers | 5.3 |  | 0.511 |
| walker |  | 7188 | 194 | Code::CodeKey { rung: Decl, file: samples/browser-esm-webpack-small/index.js, decl: 2, sub: 0, line: 152 } |  |  | 0.511 |
| walker |  | 7409 | 221 | Code::CodeKey { rung: Decl, file: samples/browser-esm-webpack-small/index.js, decl: 1, sub: 0, line: 134 } |  |  | 0.511 |
| ns | 7421 |  | 307 | Web-worker plumbing: MonacoEnvironment hooks, createWebWorker, IWebWorkerOptions fields, worker entry points | 5.4 |  | 0.501 |
| walker |  | 7442 | 33 | Code::CodeKey { rung: Names, file: samples/electron-esm-webpack/index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.501 |
| walker |  | 7483 | 41 | Code::CodeKey { rung: Decl, file: samples/electron-esm-webpack/index.js, decl: 2, sub: 0, line: 21 } |  |  | 0.501 |
| ns | 7576 |  | 155 | WorkerManager lifecycle and the CSSWorker entry points | 5.5 |  | 0.497 |
| walker |  | 7680 | 197 | Code::CodeKey { rung: Decl, file: samples/electron-esm-webpack/index.js, decl: 1, sub: 0, line: 3 } |  |  | 0.497 |
| walker |  | 7683 | 3 | Fs::DirListing { dir: website/index } |  |  | 0.497 |
| walker |  | 7705 | 22 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 4, sub: 0, line: 41 } |  |  | 0.497 |
| walker |  | 7727 | 22 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 8, sub: 0, line: 73 } |  |  | 0.497 |
| walker |  | 7749 | 22 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 32, sub: 0, line: 259 } |  |  | 0.497 |
| walker |  | 7772 | 23 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 5, sub: 0, line: 48 } |  |  | 0.497 |
| walker |  | 7818 | 46 | Fs::DirListing { dir: website/src/website } |  |  | 0.497 |
| ns | 7830 |  | 254 | typescript/languageFeatures.ts — the TypeScript-specific adapter roster | 5.6 |  | 0.491 |
| walker |  | 7833 | 15 | Fs::DirListing { dir: website/src/website/data } |  |  | 0.491 |
| walker |  | 7860 | 27 | Fs::DirListing { dir: website/src/website/pages } |  |  | 0.491 |
| walker |  | 7894 | 34 | Fs::DirListing { dir: website/src/website/data/playground-samples } |  |  | 0.491 |
| ns | 7909 |  | 79 | monaco-lsp-client package: complete tree listing | 6.1 |  | 0.500 |
| walker |  | 7913 | 19 | Fs::DirListing { dir: website/src/website/data/playground-samples/customizing-the-appearence } |  |  | 0.500 |
| walker |  | 7934 | 21 | Fs::DirListing { dir: website/src/website/data/playground-samples/creating-the-diffeditor } |  |  | 0.500 |
| walker |  | 7965 | 31 | Fs::DirListing { dir: website/src/website/data/playground-samples/creating-the-editor } |  |  | 0.500 |
| walker |  | 8005 | 40 | Fs::DirListing { dir: website/src/website/components } |  |  | 0.500 |
| walker |  | 8047 | 42 | Fs::DirListing { dir: website/src/website/utils } |  |  | 0.500 |
| walker |  | 8098 | 51 | Fs::DirListing { dir: test/smoke } |  |  | 0.515 |
| walker |  | 8106 | 8 | Fs::DirListing { dir: test/smoke/esbuild } |  |  | 0.515 |
| walker |  | 8114 | 8 | Fs::DirListing { dir: test/smoke/vite } |  |  | 0.515 |
| ns | 8116 |  | 207 | MonacoLspClient wiring + index exports | 6.2 | 6.1 | 0.509 |
| walker |  | 8122 | 8 | Fs::DirListing { dir: test/smoke/webpack } |  |  | 0.509 |
| walker |  | 8134 | 12 | Fs::DirListing { dir: test/smoke/parcel } |  |  | 0.509 |
| ns | 8165 |  | 49 | webpack-plugin package: complete tree listing | 6.3 |  | 0.516 |
| walker |  | 8238 | 104 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.516 |
| walker |  | 8272 | 34 | Json::Dependencies { file: webpack-plugin/package.json } |  |  | 0.516 |
| walker |  | 8297 | 25 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 1, sub: 0, line: 7 } |  |  | 0.516 |
| walker |  | 8322 | 25 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 2, sub: 0, line: 15 } |  |  | 0.516 |
| walker |  | 8347 | 25 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 7, sub: 0, line: 64 } |  |  | 0.516 |
| walker |  | 8372 | 25 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 33, sub: 0, line: 628 } |  |  | 0.516 |
| walker |  | 8426 | 54 | Json::Scripts { file: samples/browser-esm-vite/package.json } |  |  | 0.516 |
| ns | 8449 |  | 284 | webpack-plugin option fields and peer-dependency contract | 6.4 | 6.3 | 0.511 |
| walker |  | 8452 | 26 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 3, sub: 0, line: 26 } |  |  | 0.511 |
| walker |  | 8478 | 26 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 6, sub: 0, line: 57 } |  |  | 0.511 |
| walker |  | 8514 | 36 | Json::Dependencies { file: samples/package.json } |  |  | 0.511 |
| walker |  | 8570 | 56 | Json::Scripts { file: samples/browser-esm-webpack-small/package.json } |  |  | 0.511 |
| ns | 8613 |  | 164 | build/ and scripts/ tree listings | 7.1 |  | 0.502 |
| walker |  | 8655 | 85 | Code::CodeKey { rung: Names, file: samples/browser-esm-vite/main.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.502 |
| walker |  | 8668 | 13 | Code::CodeKey { rung: Decl, file: samples/browser-esm-vite/main.ts, decl: 3, sub: 0, line: 18 } |  |  | 0.502 |
| walker |  | 8700 | 32 | Code::CodeKey { rung: Decl, file: samples/browser-esm-vite/main.ts, decl: 4, sub: 0, line: 21 } |  |  | 0.502 |
| walker |  | 8734 | 34 | Json::Scripts { file: samples/legacy/electron-amd/package.json } |  |  | 0.502 |
| ns | 8762 |  | 149 | build-monaco-editor.ts run() — what the published package is made of | 7.2 |  | 0.499 |
| walker |  | 8768 | 34 | Json::Scripts { file: samples/legacy/electron-amd-nodeIntegration/package.json } |  |  | 0.499 |
| ns | 9000 |  | 238 | check-samples.ts — the invariant every new language must satisfy | 7.3 |  | 0.492 |
| ns | 9166 |  | 166 | package.json distribution fields: typings, main, module, exports | 8.1 |  | 0.497 |
| walker |  | 9389 | 621 | Code::CodeKey { rung: Decl, file: webpack-plugin/src/index.ts, decl: 1, sub: 0, line: 102 } |  |  | 0.506 |
| ns | 9420 |  | 254 | README Installing + CHANGELOG head (0.55.x breaking changes) | 8.2 |  | 0.501 |
| walker |  | 9448 | 59 | Json::Scripts { file: samples/electron-esm-webpack/package.json } |  |  | 0.501 |
| walker |  | 9511 | 63 | Json::Scripts { file: samples/browser-esm-webpack-typescript/package.json } |  |  | 0.501 |
| ns | 9524 |  | 104 | samples/ listing — every integration sample directory | 8.3 |  | 0.509 |
| ns | 9562 |  | 38 | docs/ listing | 8.4 |  | 0.511 |
| walker |  | 9578 | 67 | Fs::DirListing { dir: website/src/website/pages/playground } |  |  | 0.511 |
| walker |  | 9644 | 66 | Json::Scripts { file: samples/browser-esm-webpack-typescript-react/package.json } |  |  | 0.511 |
| ns | 9645 |  | 83 | integrate-esm.md section headings | 8.5 | 8.4 | 0.509 |
| walker |  | 9719 | 75 | Fs::DirListing { dir: website/src/website/data/playground-samples/extending-language-services } |  |  | 0.509 |
| ns | 9771 |  | 126 | website/ and CI/publishing config listings | 8.6 |  | 0.521 |
| walker |  | 9871 | 152 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.522 |
| walker |  | 9899 | 28 | Code::CodeKey { rung: Names, file: monaco-lsp-client/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.522 |
| walker |  | 9973 | 74 | Fs::DirListing { dir: website/src/website/data/playground-samples/interacting-with-the-editor } |  |  | 0.522 |
| walker |  | 9998 | 25 | Fs::DirListing { dir: monaco-lsp-client/src/adapters/languageFeatures } |  |  | 0.522 |

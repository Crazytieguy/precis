Score(3000)=0.713 I=0.805 C=0.632 ns_rows≤3K=23/56 grid(1000/1442/2080/3000/4327/6240/9000)=0.659/0.695/0.782/0.713/0.640/0.564/0.501

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
| walker |  | 314 | 29 | Fs::DirListing { dir: src/languages/features/json } |  |  | 0.528 |
| ns | 332 |  | 116 | Complete repository root listing | 1.4 |  | 0.659 |
| walker |  | 343 | 29 | Fs::DirListing { dir: webpack-plugin } |  |  | 0.659 |
| walker |  | 350 | 7 | Fs::DirListing { dir: src/deprecated/basic-languages } |  |  | 0.659 |
| walker |  | 382 | 32 | Fs::DirListing { dir: src/languages/features/typescript } |  |  | 0.662 |
| walker |  | 397 | 15 | Fs::DirListing { dir: monaco-lsp-client/src } |  |  | 0.663 |
| walker |  | 402 | 5 | Fs::DirListing { dir: .devcontainer } |  |  | 0.663 |
| walker |  | 407 | 5 | Fs::DirListing { dir: .husky } |  |  | 0.663 |
| walker |  | 415 | 8 | Fs::DirListing { dir: src/internal/common } |  |  | 0.667 |
| walker |  | 453 | 38 | Fs::DirListing { dir: docs } |  |  | 0.668 |
| walker |  | 459 | 6 | Fs::DirListing { dir: scripts } |  |  | 0.668 |
| walker |  | 469 | 10 | Fs::DirListing { dir: src/deprecated/editor } |  |  | 0.668 |
| ns | 471 |  | 139 | package.json identity header (name, version, vscodeRef, license) | 1.5 |  | 0.604 |
| walker |  | 489 | 20 | Fs::DirListing { dir: webpack-plugin/src } |  |  | 0.605 |
| walker |  | 493 | 4 | Fs::DirListing { dir: webpack-plugin/src/loaders } |  |  | 0.605 |
| walker |  | 501 | 8 | Fs::DirListing { dir: webpack-plugin/src/plugins } |  |  | 0.605 |
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
| walker |  | 1491 | 21 | Fs::DirListing { dir: samples/browser-esm-parcel } |  |  | 0.695 |
| walker |  | 1499 | 8 | Fs::DirListing { dir: samples/browser-esm-parcel/src } |  |  | 0.695 |
| walker |  | 1514 | 15 | Json::Identity { file: samples/browser-esm-parcel/package.json } |  |  | 0.695 |
| walker |  | 1536 | 22 | Fs::DirListing { dir: samples/browser-esm-webpack-typescript } |  |  | 0.695 |
| walker |  | 1544 | 8 | Fs::DirListing { dir: samples/browser-esm-webpack-typescript/src } |  |  | 0.695 |
| walker |  | 1568 | 24 | Fs::DirListing { dir: samples/browser-esm-esbuild } |  |  | 0.695 |
| walker |  | 1583 | 15 | Json::Identity { file: samples/browser-esm-esbuild/package.json } |  |  | 0.695 |
| walker |  | 1608 | 25 | Fs::DirListing { dir: samples/browser-esm-webpack } |  |  | 0.695 |
| walker |  | 1625 | 17 | Json::Identity { file: samples/browser-esm-webpack/package.json } |  |  | 0.695 |
| walker |  | 1650 | 25 | Fs::DirListing { dir: samples/browser-esm-webpack-monaco-plugin } |  |  | 0.695 |
| walker |  | 1675 | 25 | Fs::DirListing { dir: samples/electron-esm-webpack } |  |  | 0.695 |
| walker |  | 1692 | 17 | Json::Identity { file: samples/electron-esm-webpack/package.json } |  |  | 0.695 |
| ns | 1704 |  | 272 | Complete listing of src/features/ — all 65 editor-feature shim directories | 2.3 |  | 0.769 |
| walker |  | 1719 | 27 | Fs::DirListing { dir: samples/browser-esm-webpack-typescript-react } |  |  | 0.769 |
| walker |  | 1739 | 20 | Json::Identity { file: samples/browser-esm-webpack-monaco-plugin/package.json } |  |  | 0.769 |
| ns | 1748 |  | 44 | src/features/register.all.ts head, elided middle, tail | 2.4 |  | 0.757 |
| walker |  | 1759 | 20 | Json::Identity { file: samples/browser-esm-webpack-typescript/package.json } |  |  | 0.757 |
| walker |  | 1779 | 20 | Json::Identity { file: samples/browser-esm-webpack-typescript-react/package.json } |  |  | 0.757 |
| walker |  | 1795 | 16 | Fs::DirListing { dir: samples/browser-esm-webpack-typescript-react/src } |  |  | 0.757 |
| walker |  | 1800 | 5 | Fs::DirListing { dir: samples/browser-esm-webpack-typescript-react/src/components } |  |  | 0.757 |
| walker |  | 1831 | 31 | Fs::DirListing { dir: samples/browser-esm-vite-react } |  |  | 0.757 |
| walker |  | 1850 | 19 | Json::Identity { file: samples/browser-esm-vite-react/package.json } |  |  | 0.757 |
| walker |  | 1881 | 31 | Fs::DirListing { dir: samples/browser-esm-webpack-small } |  |  | 0.757 |
| walker |  | 1899 | 18 | Json::Identity { file: samples/browser-esm-webpack-small/package.json } |  |  | 0.757 |
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
| walker |  | 2228 | 36 | Markdown::HeadingsOutline { file: MAINTAINING.md } |  |  | 0.770 |
| walker |  | 2239 | 11 | Json::Entry { file: samples/electron-esm-webpack/package.json } |  |  | 0.770 |
| walker |  | 2321 | 82 | Fs::DirListing { dir: samples/legacy } |  |  | 0.770 |
| walker |  | 2325 | 4 | Fs::DirListing { dir: samples/legacy/browser-amd-editor } |  |  | 0.770 |
| walker |  | 2329 | 4 | Fs::DirListing { dir: samples/legacy/browser-amd-localized } |  |  | 0.770 |
| walker |  | 2333 | 4 | Fs::DirListing { dir: samples/legacy/browser-amd-monarch } |  |  | 0.770 |
| walker |  | 2337 | 4 | Fs::DirListing { dir: samples/legacy/browser-amd-requirejs } |  |  | 0.770 |
| walker |  | 2341 | 4 | Fs::DirListing { dir: samples/legacy/browser-amd-shadow-dom } |  |  | 0.770 |
| walker |  | 2345 | 4 | Fs::DirListing { dir: samples/legacy/browser-amd-shared-model } |  |  | 0.770 |
| walker |  | 2349 | 4 | Fs::DirListing { dir: samples/legacy/browser-amd-trusted-types } |  |  | 0.770 |
| walker |  | 2357 | 8 | Fs::DirListing { dir: samples/legacy/browser-amd-iframe } |  |  | 0.770 |
| walker |  | 2369 | 12 | Fs::DirListing { dir: samples/legacy/browser-amd-diff-editor } |  |  | 0.770 |
| walker |  | 2386 | 17 | Fs::DirListing { dir: samples/legacy/electron-amd } |  |  | 0.770 |
| walker |  | 2403 | 17 | Fs::DirListing { dir: samples/legacy/electron-amd-nodeIntegration } |  |  | 0.770 |
| walker |  | 2419 | 16 | Json::Identity { file: samples/legacy/electron-amd/package.json } |  |  | 0.770 |
| ns | 2431 |  | 282 | _.contribution.ts: ILang/ILangImpl and registerLanguage() | 2.8 |  | 0.734 |
| walker |  | 2438 | 19 | Json::Identity { file: samples/legacy/electron-amd-nodeIntegration/package.json } |  |  | 0.734 |
| walker |  | 2465 | 27 | Json::Entry { file: monaco-lsp-client/package.json } |  |  | 0.734 |
| walker |  | 2493 | 28 | Json::Entry { file: webpack-plugin/package.json } |  |  | 0.734 |
| walker |  | 2544 | 51 | Markdown::HeadingsOutline { file: samples/README.md } |  |  | 0.734 |
| ns | 2570 |  | 139 | _.contribution.ts: LazyLanguageLoader and loadLanguage() | 2.9 |  | 0.718 |
| walker |  | 2583 | 39 | Json::Scripts { file: samples/package.json } |  |  | 0.718 |
| walker |  | 2644 | 61 | Markdown::HeadingsOutline { file: webpack-plugin/README.md } |  |  | 0.718 |
| ns | 2659 |  | 89 | Grammar file shape: rust.ts conf + language exports | 2.10 |  | 0.707 |
| walker |  | 2790 | 146 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.747 |
| ns | 2799 |  | 140 | Tokenization test harness: testRunner.ts types + testTokenization(), and a test file head | 2.11 |  | 0.730 |
| walker |  | 2834 | 44 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.730 |
| ns | 2841 |  | 42 | src/languages/definitions/register.all.ts head, elided middle, tail | 2.12 |  | 0.724 |
| walker |  | 2847 | 13 | Json::Entry { file: samples/legacy/electron-amd/package.json } |  |  | 0.724 |
| walker |  | 2860 | 13 | Json::Entry { file: samples/legacy/electron-amd-nodeIntegration/package.json } |  |  | 0.724 |
| walker |  | 2920 | 60 | Fs::DirListing { dir: website } |  |  | 0.725 |
| walker |  | 2929 | 9 | Markdown::Section { file: samples/README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.725 |
| walker |  | 2952 | 23 | Fs::DirListing { dir: scripts/ci } |  |  | 0.725 |
| walker |  | 2970 | 18 | Markdown::Section { file: samples/README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.725 |
| ns | 2976 |  | 135 | src/deprecated/: the legacy import paths, all one-line re-exports | 2.13 |  | 0.712 |
| walker |  | 3005 | 35 | Fs::DirListing { dir: src/languages/features/typescript/lib } |  |  | 0.714 |
| walker |  | 3031 | 26 | Fs::DirListing { dir: website/src } |  |  | 0.715 |
| walker |  | 3043 | 12 | Fs::DirListing { dir: website/src/runner } |  |  | 0.715 |
| walker |  | 3126 | 83 | Markdown::HeadingsOutline { file: docs/integrate-esm.md } |  |  | 0.715 |
| ns | 3345 |  | 369 | package.json scripts — build, test and dev commands (packaging variants elided) | 3.1 |  | 0.694 |
| walker |  | 3406 | 280 | Code::CodeKey { rung: Names, file: monaco-lsp-client/generator/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.694 |
| walker |  | 3417 | 11 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 9, sub: 0, line: 82 } |  |  | 0.694 |
| walker |  | 3435 | 18 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 32, sub: 0, line: 259 } |  |  | 0.694 |
| walker |  | 3456 | 21 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 21, sub: 0, line: 197 } |  |  | 0.694 |
| walker |  | 3477 | 21 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 22, sub: 0, line: 202 } |  |  | 0.683 |
| ns | 3477 |  | 132 | CONTRIBUTING: the ordered editor-test command sequence | 3.2 |  | 0.683 |
| walker |  | 3498 | 21 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 24, sub: 0, line: 213 } |  |  | 0.683 |
| walker |  | 3519 | 21 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 25, sub: 0, line: 218 } |  |  | 0.683 |
| walker |  | 3540 | 21 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 26, sub: 0, line: 223 } |  |  | 0.683 |
| ns | 3541 |  | 64 | test/ listing plus the full smoke-test directory | 3.3 |  | 0.669 |
| walker |  | 3562 | 22 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 27, sub: 0, line: 228 } |  |  | 0.669 |
| walker |  | 3584 | 22 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 29, sub: 0, line: 241 } |  |  | 0.669 |
| walker |  | 3606 | 22 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 30, sub: 0, line: 246 } |  |  | 0.669 |
| walker |  | 3630 | 24 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 31, sub: 0, line: 251 } |  |  | 0.669 |
| walker |  | 3659 | 29 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 23, sub: 0, line: 207 } |  |  | 0.669 |
| walker |  | 3690 | 31 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 16, sub: 0, line: 154 } |  |  | 0.669 |
| ns | 3692 |  | 151 | Listings of the five src/languages/features dirs — the uniform rich-service file set | 4.1 |  | 0.683 |
| walker |  | 3737 | 47 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 28, sub: 0, line: 233 } |  |  | 0.683 |
| walker |  | 3792 | 55 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 17, sub: 0, line: 159 } |  |  | 0.683 |
| walker |  | 3849 | 57 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 15, sub: 0, line: 145 } |  |  | 0.683 |
| walker |  | 3909 | 60 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 8, sub: 0, line: 73 } |  |  | 0.683 |
| ns | 3952 |  | 260 | css/register.ts — every top-level export (names only) | 4.2 |  | 0.666 |
| walker |  | 3973 | 64 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 13, sub: 0, line: 124 } |  |  | 0.666 |
| walker |  | 4047 | 74 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 12, sub: 0, line: 113 } |  |  | 0.666 |
| walker |  | 4124 | 77 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 14, sub: 0, line: 134 } |  |  | 0.666 |
| walker |  | 4203 | 79 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 1, sub: 0, line: 7 } |  |  | 0.666 |
| ns | 4237 |  | 285 | css: the default ModeConfiguration toggles, the three defaults objects, and the lazy onLanguage hookup | 4.3 | 4.2 | 0.640 |
| walker |  | 4290 | 87 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 11, sub: 0, line: 101 } |  |  | 0.640 |
| walker |  | 4391 | 101 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 19, sub: 0, line: 170 } |  |  | 0.640 |
| ns | 4496 |  | 259 | json/register.ts — every top-level export (names only) | 4.4 |  | 0.624 |
| walker |  | 4503 | 112 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 20, sub: 0, line: 183 } |  |  | 0.624 |
| walker |  | 4617 | 114 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 10, sub: 0, line: 86 } |  |  | 0.624 |
| walker |  | 4639 | 22 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 4, sub: 0, line: 41 } |  |  | 0.624 |
| walker |  | 4661 | 22 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 8, sub: 0, line: 73 } |  |  | 0.624 |
| walker |  | 4683 | 22 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 32, sub: 0, line: 259 } |  |  | 0.624 |
| walker |  | 4706 | 23 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 5, sub: 0, line: 48 } |  |  | 0.624 |
| walker |  | 4731 | 25 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 1, sub: 0, line: 7 } |  |  | 0.624 |
| walker |  | 4756 | 25 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 2, sub: 0, line: 15 } |  |  | 0.624 |
| ns | 4765 |  | 269 | json defaults: diagnostic option values, mode toggles, jsonDefaults, the worker interface | 4.5 | 4.4 | 0.605 |
| walker |  | 4781 | 25 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 7, sub: 0, line: 64 } |  |  | 0.605 |
| walker |  | 4806 | 25 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 33, sub: 0, line: 628 } |  |  | 0.605 |
| walker |  | 4832 | 26 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 3, sub: 0, line: 26 } |  |  | 0.605 |
| walker |  | 4858 | 26 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 6, sub: 0, line: 57 } |  |  | 0.605 |
| walker |  | 4888 | 30 | Markdown::Section { file: README.md, section_index: 13, keeps_default_concavity: false } |  |  | 0.605 |
| walker |  | 4918 | 30 | Json::Scripts { file: samples/browser-esm-esbuild/package.json } |  |  | 0.605 |
| walker |  | 4949 | 31 | Json::Scripts { file: samples/browser-esm-parcel/package.json } |  |  | 0.605 |
| walker |  | 4960 | 11 | Json::Dependencies { file: samples/browser-esm-vite/package.json } |  |  | 0.605 |
| walker |  | 4971 | 11 | Json::Dependencies { file: samples/browser-esm-vite-react/package.json } |  |  | 0.605 |
| walker |  | 4982 | 11 | Json::Dependencies { file: samples/browser-esm-webpack-typescript-react/package.json } |  |  | 0.605 |
| ns | 5027 |  | 262 | html/register.ts — export names plus the registerHTMLLanguageService factory | 4.6 |  | 0.591 |
| walker |  | 5051 | 69 | Json::Scripts { file: webpack-plugin/package.json } |  |  | 0.591 |
| walker |  | 5105 | 54 | Code::CodeKey { rung: Names, file: samples/electron-esm-webpack/main.js, decl: 0, sub: 0, line: 0 } |  |  | 0.591 |
| walker |  | 5159 | 54 | Code::CodeKey { rung: Names, file: samples/legacy/electron-amd/main.js, decl: 0, sub: 0, line: 0 } |  |  | 0.591 |
| walker |  | 5213 | 54 | Code::CodeKey { rung: Names, file: samples/legacy/electron-amd-nodeIntegration/main.js, decl: 0, sub: 0, line: 0 } |  |  | 0.591 |
| ns | 5311 |  | 284 | html: the three pre-registered services and their per-language toggles | 4.7 | 4.6 | 0.576 |
| walker |  | 5381 | 168 | Json::Entry { file: package.json } |  |  | 0.577 |
| walker |  | 5424 | 43 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.577 |
| walker |  | 5462 | 38 | Json::Scripts { file: samples/browser-esm-webpack/package.json } |  |  | 0.577 |
| walker |  | 5500 | 38 | Json::Scripts { file: samples/browser-esm-webpack-monaco-plugin/package.json } |  |  | 0.577 |
| ns | 5556 |  | 245 | typescript/register.ts — every top-level export (names only) | 4.8 |  | 0.566 |
| walker |  | 5580 | 80 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.566 |
| walker |  | 5624 | 44 | Code::CodeKey { rung: Names, file: webpack-plugin/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.566 |
| walker |  | 5670 | 46 | Code::CodeKey { rung: Decl, file: webpack-plugin/src/index.ts, decl: 2, sub: 0, line: 159 } |  |  | 0.566 |
| walker |  | 5700 | 30 | Code::CodeKey { rung: Names, file: src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.568 |
| walker |  | 5782 | 82 | Json::Scripts { file: monaco-lsp-client/package.json } |  |  | 0.568 |
| walker |  | 5785 | 3 | Fs::DirListing { dir: website/index } |  |  | 0.568 |
| ns | 5791 |  | 235 | LanguageServiceDefaults (typescript) — all members | 4.9 |  | 0.557 |
| walker |  | 5831 | 46 | Fs::DirListing { dir: website/src/website } |  |  | 0.557 |
| walker |  | 5846 | 15 | Fs::DirListing { dir: website/src/website/data } |  |  | 0.557 |
| walker |  | 5873 | 27 | Fs::DirListing { dir: website/src/website/pages } |  |  | 0.557 |
| walker |  | 5907 | 34 | Fs::DirListing { dir: website/src/website/data/playground-samples } |  |  | 0.557 |
| walker |  | 5926 | 19 | Fs::DirListing { dir: website/src/website/data/playground-samples/customizing-the-appearence } |  |  | 0.557 |
| walker |  | 5947 | 21 | Fs::DirListing { dir: website/src/website/data/playground-samples/creating-the-diffeditor } |  |  | 0.557 |
| walker |  | 5978 | 31 | Fs::DirListing { dir: website/src/website/data/playground-samples/creating-the-editor } |  |  | 0.557 |
| walker |  | 6018 | 40 | Fs::DirListing { dir: website/src/website/components } |  |  | 0.557 |
| ns | 6027 |  | 236 | TypeScriptWorker — all 21 proxy methods (names only) | 4.10 | 4.8 | 0.547 |
| walker |  | 6060 | 42 | Fs::DirListing { dir: website/src/website/utils } |  |  | 0.547 |
| walker |  | 6111 | 51 | Fs::DirListing { dir: test/smoke } |  |  | 0.564 |
| walker |  | 6145 | 34 | Json::Dependencies { file: webpack-plugin/package.json } |  |  | 0.564 |
| walker |  | 6199 | 54 | Json::Scripts { file: samples/browser-esm-vite/package.json } |  |  | 0.564 |
| walker |  | 6235 | 36 | Json::Dependencies { file: samples/package.json } |  |  | 0.564 |
| walker |  | 6315 | 80 | Fs::DirListing { dir: test/manual } |  |  | 0.564 |
| ns | 6318 |  | 291 | typescript/javascript defaults and their divergence | 4.11 | 4.8 | 0.550 |
| walker |  | 6371 | 56 | Json::Scripts { file: samples/browser-esm-webpack-small/package.json } |  |  | 0.550 |
| walker |  | 6411 | 40 | Markdown::Section { file: webpack-plugin/README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.550 |
| walker |  | 6445 | 34 | Json::Scripts { file: samples/legacy/electron-amd/package.json } |  |  | 0.550 |
| walker |  | 6479 | 34 | Json::Scripts { file: samples/legacy/electron-amd-nodeIntegration/package.json } |  |  | 0.550 |
| ns | 6547 |  | 229 | The four *Mode.ts entry points and the tsMode re-export tail | 5.1 |  | 0.545 |
| ns | 6761 |  | 214 | cssMode.setupMode — WorkerManager, the worker accessor, and ModeConfiguration-gated provider registration | 5.2 | 5.1 | 0.535 |
| walker |  | 7100 | 621 | Code::CodeKey { rung: Decl, file: webpack-plugin/src/index.ts, decl: 1, sub: 0, line: 102 } |  |  | 0.536 |
| ns | 7114 |  | 353 | common/lspLanguageFeatures.ts — the shared provider adapters and LSP conversion helpers | 5.3 |  | 0.527 |
| walker |  | 7159 | 59 | Json::Scripts { file: samples/electron-esm-webpack/package.json } |  |  | 0.527 |
| walker |  | 7206 | 47 | Code::CodeKey { rung: Names, file: samples/browser-esm-vite/main.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.527 |
| walker |  | 7219 | 13 | Code::CodeKey { rung: Decl, file: samples/browser-esm-vite/main.ts, decl: 1, sub: 0, line: 18 } |  |  | 0.527 |
| walker |  | 7251 | 32 | Code::CodeKey { rung: Decl, file: samples/browser-esm-vite/main.ts, decl: 2, sub: 0, line: 21 } |  |  | 0.527 |
| walker |  | 7314 | 63 | Json::Scripts { file: samples/browser-esm-webpack-typescript/package.json } |  |  | 0.527 |
| walker |  | 7373 | 59 | Markdown::Section { file: README.md, section_index: 11, keeps_default_concavity: false } |  |  | 0.527 |
| ns | 7421 |  | 307 | Web-worker plumbing: MonacoEnvironment hooks, createWebWorker, IWebWorkerOptions fields, worker entry points | 5.4 |  | 0.517 |
| walker |  | 7486 | 113 | Code::CodeKey { rung: Body, file: samples/legacy/electron-amd/main.js, decl: 4, sub: 0, line: 7 } |  |  | 0.517 |
| ns | 7576 |  | 155 | WorkerManager lifecycle and the CSSWorker entry points | 5.5 |  | 0.513 |
| walker |  | 7601 | 115 | Code::CodeKey { rung: Body, file: samples/electron-esm-webpack/main.js, decl: 4, sub: 0, line: 7 } |  |  | 0.513 |
| walker |  | 7668 | 67 | Fs::DirListing { dir: website/src/website/pages/playground } |  |  | 0.513 |
| walker |  | 7734 | 66 | Json::Scripts { file: samples/browser-esm-webpack-typescript-react/package.json } |  |  | 0.513 |
| ns | 7830 |  | 254 | typescript/languageFeatures.ts — the TypeScript-specific adapter roster | 5.6 |  | 0.507 |
| walker |  | 7844 | 110 | Markdown::Section { file: MAINTAINING.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.507 |
| ns | 7909 |  | 79 | monaco-lsp-client package: complete tree listing | 6.1 |  | 0.516 |
| walker |  | 7919 | 75 | Fs::DirListing { dir: website/src/website/data/playground-samples/extending-language-services } |  |  | 0.516 |
| walker |  | 7947 | 28 | Code::CodeKey { rung: Names, file: monaco-lsp-client/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.516 |
| walker |  | 8002 | 55 | Markdown::Section { file: monaco-lsp-client/README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.516 |
| walker |  | 8076 | 74 | Fs::DirListing { dir: website/src/website/data/playground-samples/interacting-with-the-editor } |  |  | 0.516 |
| ns | 8116 |  | 207 | MonacoLspClient wiring + index exports | 6.2 | 6.1 | 0.510 |
| ns | 8165 |  | 49 | webpack-plugin package: complete tree listing | 6.3 |  | 0.517 |
| walker |  | 8250 | 174 | Fs::DirListing { dir: monaco-lsp-client/src/adapters/languageFeatures } |  |  | 0.517 |
| walker |  | 8337 | 87 | Json::Scripts { file: samples/browser-esm-vite-react/package.json } |  |  | 0.517 |
| walker |  | 8441 | 104 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.517 |
| ns | 8449 |  | 284 | webpack-plugin option fields and peer-dependency contract | 6.4 | 6.3 | 0.521 |
| walker |  | 8596 | 155 | Code::CodeKey { rung: Body, file: samples/legacy/electron-amd-nodeIntegration/main.js, decl: 4, sub: 0, line: 7 } |  |  | 0.521 |
| ns | 8613 |  | 164 | build/ and scripts/ tree listings | 7.1 |  | 0.512 |
| walker |  | 8751 | 155 | Code::CodeKey { rung: Body, file: webpack-plugin/src/index.ts, decl: 3, sub: 0, line: 162 } |  |  | 0.512 |
| ns | 8762 |  | 149 | build-monaco-editor.ts run() — what the published package is made of | 7.2 |  | 0.508 |
| walker |  | 8784 | 33 | Markdown::Section { file: samples/legacy/README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.508 |
| walker |  | 8860 | 76 | Markdown::Section { file: README.md, section_index: 12, keeps_default_concavity: false } |  |  | 0.508 |
| walker |  | 8911 | 51 | Markdown::Section { file: MAINTAINING.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.508 |
| walker |  | 8939 | 28 | Markdown::Section { file: webpack-plugin/README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.508 |
| walker |  | 8943 | 4 | Fs::DirListing { dir: test/smoke/amd } |  |  | 0.508 |
| walker |  | 8951 | 8 | Fs::DirListing { dir: website/static } |  |  | 0.508 |
| ns | 9000 |  | 238 | check-samples.ts — the invariant every new language must satisfy | 7.3 |  | 0.501 |
| walker |  | 9103 | 152 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.501 |
| ns | 9166 |  | 166 | package.json distribution fields: typings, main, module, exports | 8.1 |  | 0.507 |
| walker |  | 9184 | 81 | Json::Dependencies { file: monaco-lsp-client/package.json } |  |  | 0.507 |
| walker |  | 9204 | 20 | Code::CodeKey { rung: Names, file: src/editor.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.507 |
| walker |  | 9213 | 9 | Fs::DirListing { dir: website/scripts } |  |  | 0.507 |
| walker |  | 9248 | 35 | Code::CodeKey { rung: Names, file: monaco-lsp-client/src/utils.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.507 |
| walker |  | 9260 | 12 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/src/utils.ts, decl: 1, sub: 0, line: 1 } |  |  | 0.507 |
| walker |  | 9311 | 51 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/src/utils.ts, decl: 2, sub: 0, line: 5 } |  |  | 0.507 |
| walker |  | 9321 | 10 | Code::CodeKey { rung: Body, file: monaco-lsp-client/src/utils.ts, decl: 4, sub: 0, line: 12 } |  |  | 0.507 |
| walker |  | 9384 | 63 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/src/utils.ts, decl: 5, sub: 0, line: 24 } |  |  | 0.507 |
| ns | 9420 |  | 254 | README Installing + CHANGELOG head (0.55.x breaking changes) | 8.2 |  | 0.504 |
| ns | 9524 |  | 104 | samples/ listing — every integration sample directory | 8.3 |  | 0.511 |
| ns | 9562 |  | 38 | docs/ listing | 8.4 |  | 0.514 |
| ns | 9645 |  | 83 | integrate-esm.md section headings | 8.5 | 8.4 | 0.516 |
| ns | 9771 |  | 126 | website/ and CI/publishing config listings | 8.6 |  | 0.527 |
| walker |  | 9979 | 595 | Json::Scripts { file: package.json } |  |  | 0.538 |

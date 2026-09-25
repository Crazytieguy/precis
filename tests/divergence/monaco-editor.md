Score(3000)=0.675 I=0.771 C=0.592 ns_rows≤3K=23/56 grid(1000/1442/2080/3000/4327/6240/9000)=0.663/0.699/0.784/0.675/0.594/0.552/0.510

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
| walker |  | 284 | 30 | Code::CodeKey { rung: Names, file: src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.543 |
| walker |  | 311 | 27 | Fs::DirListing { dir: monaco-lsp-client } |  |  | 0.543 |
| walker |  | 315 | 4 | Fs::DirListing { dir: monaco-lsp-client/generator } |  |  | 0.543 |
| ns | 332 |  | 116 | Complete repository root listing | 1.4 |  | 0.663 |
| walker |  | 344 | 29 | Fs::DirListing { dir: src/languages/features/json } |  |  | 0.665 |
| walker |  | 373 | 29 | Fs::DirListing { dir: webpack-plugin } |  |  | 0.666 |
| walker |  | 405 | 32 | Fs::DirListing { dir: src/languages/features/typescript } |  |  | 0.668 |
| walker |  | 412 | 7 | Fs::DirListing { dir: src/deprecated/basic-languages } |  |  | 0.668 |
| walker |  | 427 | 15 | Fs::DirListing { dir: monaco-lsp-client/src } |  |  | 0.669 |
| walker |  | 455 | 28 | Code::CodeKey { rung: Names, file: monaco-lsp-client/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.669 |
| walker |  | 460 | 5 | Fs::DirListing { dir: .devcontainer } |  |  | 0.669 |
| walker |  | 465 | 5 | Fs::DirListing { dir: .husky } |  |  | 0.669 |
| ns | 471 |  | 139 | package.json identity header (name, version, vscodeRef, license) | 1.5 |  | 0.605 |
| walker |  | 473 | 8 | Fs::DirListing { dir: src/internal/common } |  |  | 0.609 |
| walker |  | 511 | 38 | Fs::DirListing { dir: docs } |  |  | 0.610 |
| walker |  | 517 | 6 | Fs::DirListing { dir: scripts } |  |  | 0.610 |
| walker |  | 537 | 20 | Fs::DirListing { dir: webpack-plugin/src } |  |  | 0.611 |
| walker |  | 541 | 4 | Fs::DirListing { dir: webpack-plugin/src/loaders } |  |  | 0.611 |
| walker |  | 549 | 8 | Fs::DirListing { dir: webpack-plugin/src/plugins } |  |  | 0.611 |
| walker |  | 559 | 10 | Fs::DirListing { dir: src/deprecated/editor } |  |  | 0.611 |
| ns | 588 |  | 117 | The three one-to-six-line re-export files src/index.ts depends on | 1.6 | 1.2 | 0.561 |
| walker |  | 629 | 70 | Json::Identity { file: package.json } |  |  | 0.582 |
| walker |  | 636 | 7 | Fs::DirListing { dir: src/languages/features/common } |  |  | 0.582 |
| walker |  | 703 | 67 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.745 |
| ns | 734 |  | 146 | All README section headings (locations) | 1.7 |  | 0.661 |
| ns | 927 |  | 193 | CONTRIBUTING 'Source Code Structure' — the monaco-editor-core boundary | 1.8 |  | 0.630 |
| walker |  | 975 | 272 | Fs::DirListing { dir: src/features } |  |  | 0.662 |
| walker |  | 995 | 20 | Code::CodeKey { rung: Names, file: src/editor.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.663 |
| ns | 1097 |  | 170 | README Concepts: Models and URIs | 1.9 | 1.7 | 0.655 |
| ns | 1259 |  | 162 | README Concepts: Editors, Providers, Disposables | 1.10 | 1.7 | 0.642 |
| walker |  | 1299 | 304 | Fs::DirListing { dir: src/languages/definitions } |  |  | 0.668 |
| ns | 1326 |  | 67 | Listings of every src/ subtree except the two huge uniform ones | 2.1 |  | 0.717 |
| walker |  | 1347 | 48 | Json::Identity { file: monaco-lsp-client/package.json } |  |  | 0.717 |
| walker |  | 1359 | 12 | Fs::DirListing { dir: .azure-pipelines } |  |  | 0.717 |
| walker |  | 1371 | 12 | Fs::DirListing { dir: .vscode } |  |  | 0.717 |
| walker |  | 1384 | 13 | Fs::DirListing { dir: test } |  |  | 0.717 |
| ns | 1432 |  | 106 | Uniform shape of a feature shim (hover, gpu) and the one that is not uniform (find) | 2.2 |  | 0.699 |
| walker |  | 1664 | 280 | Code::CodeKey { rung: Names, file: monaco-lsp-client/generator/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.699 |
| walker |  | 1675 | 11 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 9, sub: 0, line: 82 } |  |  | 0.699 |
| walker |  | 1693 | 18 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 32, sub: 0, line: 259 } |  |  | 0.699 |
| ns | 1704 |  | 272 | Complete listing of src/features/ — all 65 editor-feature shim directories | 2.3 |  | 0.771 |
| walker |  | 1714 | 21 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 21, sub: 0, line: 197 } |  |  | 0.771 |
| walker |  | 1735 | 21 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 22, sub: 0, line: 202 } |  |  | 0.771 |
| ns | 1748 |  | 44 | src/features/register.all.ts head, elided middle, tail | 2.4 |  | 0.759 |
| walker |  | 1756 | 21 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 24, sub: 0, line: 213 } |  |  | 0.759 |
| walker |  | 1777 | 21 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 25, sub: 0, line: 218 } |  |  | 0.759 |
| walker |  | 1798 | 21 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 26, sub: 0, line: 223 } |  |  | 0.759 |
| walker |  | 1820 | 22 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 27, sub: 0, line: 228 } |  |  | 0.759 |
| walker |  | 1842 | 22 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 29, sub: 0, line: 241 } |  |  | 0.759 |
| walker |  | 1864 | 22 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 30, sub: 0, line: 246 } |  |  | 0.759 |
| walker |  | 1888 | 24 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 31, sub: 0, line: 251 } |  |  | 0.759 |
| walker |  | 1917 | 29 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 23, sub: 0, line: 207 } |  |  | 0.759 |
| walker |  | 1948 | 31 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 16, sub: 0, line: 154 } |  |  | 0.759 |
| walker |  | 1960 | 12 | Fs::DirListing { dir: src/deprecated/language/css } |  |  | 0.759 |
| walker |  | 1972 | 12 | Fs::DirListing { dir: src/deprecated/language/html } |  |  | 0.759 |
| walker |  | 1984 | 12 | Fs::DirListing { dir: src/deprecated/language/json } |  |  | 0.759 |
| walker |  | 1996 | 12 | Fs::DirListing { dir: src/deprecated/language/typescript } |  |  | 0.759 |
| walker |  | 2000 | 4 | Fs::DirListing { dir: scripts/lib } |  |  | 0.759 |
| ns | 2052 |  | 304 | Complete listing of src/languages/definitions/ — all 84 Monarch languages plus _.contribution.ts, register.all.ts and test/ | 2.5 |  | 0.789 |
| ns | 2065 |  | 13 | One language definition directory, listed in full (rust) | 2.6 |  | 0.784 |
| walker |  | 2104 | 104 | Fs::DirListing { dir: samples } |  |  | 0.785 |
| walker |  | 2108 | 4 | Fs::DirListing { dir: samples/nwjs-amd } |  |  | 0.785 |
| walker |  | 2112 | 4 | Fs::DirListing { dir: samples/nwjs-amd-v2 } |  |  | 0.785 |
| walker |  | 2133 | 21 | Fs::DirListing { dir: samples/browser-esm-parcel } |  |  | 0.785 |
| walker |  | 2141 | 8 | Fs::DirListing { dir: samples/browser-esm-parcel/src } |  |  | 0.785 |
| ns | 2149 |  | 84 | rust/register.ts registerLanguage call | 2.7 | 2.6 | 0.772 |
| walker |  | 2156 | 15 | Json::Identity { file: samples/browser-esm-parcel/package.json } |  |  | 0.772 |
| walker |  | 2178 | 22 | Fs::DirListing { dir: samples/browser-esm-webpack-typescript } |  |  | 0.772 |
| walker |  | 2186 | 8 | Fs::DirListing { dir: samples/browser-esm-webpack-typescript/src } |  |  | 0.772 |
| walker |  | 2210 | 24 | Fs::DirListing { dir: samples/browser-esm-esbuild } |  |  | 0.772 |
| walker |  | 2225 | 15 | Json::Identity { file: samples/browser-esm-esbuild/package.json } |  |  | 0.772 |
| walker |  | 2250 | 25 | Fs::DirListing { dir: samples/browser-esm-webpack } |  |  | 0.772 |
| walker |  | 2267 | 17 | Json::Identity { file: samples/browser-esm-webpack/package.json } |  |  | 0.772 |
| walker |  | 2292 | 25 | Fs::DirListing { dir: samples/browser-esm-webpack-monaco-plugin } |  |  | 0.772 |
| walker |  | 2317 | 25 | Fs::DirListing { dir: samples/electron-esm-webpack } |  |  | 0.772 |
| walker |  | 2334 | 17 | Json::Identity { file: samples/electron-esm-webpack/package.json } |  |  | 0.772 |
| walker |  | 2361 | 27 | Fs::DirListing { dir: samples/browser-esm-webpack-typescript-react } |  |  | 0.772 |
| walker |  | 2381 | 20 | Json::Identity { file: samples/browser-esm-webpack-monaco-plugin/package.json } |  |  | 0.772 |
| walker |  | 2401 | 20 | Json::Identity { file: samples/browser-esm-webpack-typescript/package.json } |  |  | 0.772 |
| walker |  | 2421 | 20 | Json::Identity { file: samples/browser-esm-webpack-typescript-react/package.json } |  |  | 0.772 |
| ns | 2431 |  | 282 | _.contribution.ts: ILang/ILangImpl and registerLanguage() | 2.8 |  | 0.735 |
| walker |  | 2437 | 16 | Fs::DirListing { dir: samples/browser-esm-webpack-typescript-react/src } |  |  | 0.735 |
| walker |  | 2442 | 5 | Fs::DirListing { dir: samples/browser-esm-webpack-typescript-react/src/components } |  |  | 0.735 |
| walker |  | 2473 | 31 | Markdown::ReadmeHeadline { file: samples/README.md } |  |  | 0.735 |
| walker |  | 2504 | 31 | Fs::DirListing { dir: samples/browser-esm-vite-react } |  |  | 0.735 |
| walker |  | 2523 | 19 | Json::Identity { file: samples/browser-esm-vite-react/package.json } |  |  | 0.735 |
| walker |  | 2554 | 31 | Fs::DirListing { dir: samples/browser-esm-webpack-small } |  |  | 0.735 |
| ns | 2570 |  | 139 | _.contribution.ts: LazyLanguageLoader and loadLanguage() | 2.9 |  | 0.719 |
| walker |  | 2572 | 18 | Json::Identity { file: samples/browser-esm-webpack-small/package.json } |  |  | 0.719 |
| walker |  | 2591 | 19 | Fs::DirListing { dir: samples/browser-esm-vite-react/src } |  |  | 0.719 |
| walker |  | 2601 | 10 | Fs::DirListing { dir: samples/browser-esm-vite-react/src/components } |  |  | 0.719 |
| walker |  | 2637 | 36 | Fs::DirListing { dir: samples/browser-esm-vite } |  |  | 0.719 |
| ns | 2659 |  | 89 | Grammar file shape: rust.ts conf + language exports | 2.10 |  | 0.709 |
| walker |  | 2684 | 47 | Code::CodeKey { rung: Names, file: samples/browser-esm-vite/main.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.709 |
| walker |  | 2697 | 13 | Code::CodeKey { rung: Decl, file: samples/browser-esm-vite/main.ts, decl: 1, sub: 0, line: 18 } |  |  | 0.709 |
| walker |  | 2715 | 18 | Json::Identity { file: samples/browser-esm-vite/package.json } |  |  | 0.709 |
| walker |  | 2747 | 32 | Code::CodeKey { rung: Decl, file: samples/browser-esm-vite/main.ts, decl: 2, sub: 0, line: 21 } |  |  | 0.709 |
| ns | 2799 |  | 140 | Tokenization test harness: testRunner.ts types + testTokenization(), and a test file head | 2.11 |  | 0.693 |
| walker |  | 2809 | 62 | Json::Identity { file: webpack-plugin/package.json } |  |  | 0.693 |
| walker |  | 2819 | 10 | Json::IdentityMeta { file: monaco-lsp-client/package.json } |  |  | 0.693 |
| ns | 2841 |  | 42 | src/languages/definitions/register.all.ts head, elided middle, tail | 2.12 |  | 0.688 |
| walker |  | 2890 | 71 | Json::Identity { file: samples/package.json } |  |  | 0.688 |
| walker |  | 2902 | 12 | Json::Entry { file: samples/package.json } |  |  | 0.688 |
| walker |  | 2949 | 47 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 28, sub: 0, line: 233 } |  |  | 0.688 |
| ns | 2976 |  | 135 | src/deprecated/: the legacy import paths, all one-line re-exports | 2.13 |  | 0.675 |
| walker |  | 3004 | 55 | Markdown::ReadmeHeadline { file: monaco-lsp-client/README.md } |  |  | 0.675 |
| walker |  | 3060 | 56 | Markdown::ReadmeHeadline { file: webpack-plugin/README.md } |  |  | 0.675 |
| walker |  | 3072 | 12 | Json::IdentityMeta { file: samples/package.json } |  |  | 0.675 |
| ns | 3345 |  | 369 | package.json scripts — build, test and dev commands (packaging variants elided) | 3.1 |  | 0.655 |
| walker |  | 3439 | 367 | Code::CodeKey { rung: Names, file: webpack-plugin/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.656 |
| ns | 3477 |  | 132 | CONTRIBUTING: the ordered editor-test command sequence | 3.2 |  | 0.645 |
| walker |  | 3483 | 44 | Code::CodeKey { rung: Decl, file: webpack-plugin/src/index.ts, decl: 11, sub: 0, line: 159 } |  |  | 0.645 |
| walker |  | 3508 | 25 | Code::CodeKey { rung: Decl, file: webpack-plugin/src/index.ts, decl: 5, sub: 0, line: 53 } |  |  | 0.645 |
| walker |  | 3535 | 27 | Code::CodeKey { rung: Decl, file: webpack-plugin/src/index.ts, decl: 14, sub: 0, line: 207 } |  |  | 0.645 |
| ns | 3541 |  | 64 | test/ listing plus the full smoke-test directory | 3.3 |  | 0.632 |
| walker |  | 3572 | 37 | Code::CodeKey { rung: Decl, file: webpack-plugin/src/index.ts, decl: 4, sub: 0, line: 43 } |  |  | 0.632 |
| walker |  | 3612 | 40 | Code::CodeKey { rung: Decl, file: webpack-plugin/src/index.ts, decl: 7, sub: 0, line: 63 } |  |  | 0.632 |
| walker |  | 3660 | 48 | Code::CodeKey { rung: Decl, file: webpack-plugin/src/index.ts, decl: 8, sub: 0, line: 90 } |  |  | 0.632 |
| walker |  | 3680 | 20 | Fs::DirListing { dir: .github } |  |  | 0.632 |
| walker |  | 3688 | 8 | Fs::DirListing { dir: .github/workflows } |  |  | 0.632 |
| ns | 3692 |  | 151 | Listings of the five src/languages/features dirs — the uniform rich-service file set | 4.1 |  | 0.633 |
| walker |  | 3743 | 55 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 17, sub: 0, line: 159 } |  |  | 0.633 |
| walker |  | 3799 | 56 | Code::CodeKey { rung: Decl, file: webpack-plugin/src/index.ts, decl: 19, sub: 0, line: 334 } |  |  | 0.633 |
| walker |  | 3856 | 57 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 15, sub: 0, line: 145 } |  |  | 0.633 |
| walker |  | 3916 | 60 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 8, sub: 0, line: 73 } |  |  | 0.633 |
| ns | 3952 |  | 260 | css/register.ts — every top-level export (names only) | 4.2 |  | 0.618 |
| walker |  | 3953 | 37 | Fs::DirListing { dir: monaco-lsp-client/src/adapters } |  |  | 0.618 |
| walker |  | 4127 | 174 | Fs::DirListing { dir: monaco-lsp-client/src/adapters/languageFeatures } |  |  | 0.618 |
| walker |  | 4190 | 63 | Code::CodeKey { rung: Decl, file: webpack-plugin/src/index.ts, decl: 2, sub: 0, line: 12 } |  |  | 0.618 |
| walker |  | 4226 | 36 | Markdown::HeadingsOutline { file: MAINTAINING.md } |  |  | 0.618 |
| ns | 4237 |  | 285 | css: the default ModeConfiguration toggles, the three defaults objects, and the lazy onLanguage hookup | 4.3 | 4.2 | 0.594 |
| walker |  | 4290 | 64 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 13, sub: 0, line: 124 } |  |  | 0.594 |
| walker |  | 4301 | 11 | Json::Entry { file: samples/electron-esm-webpack/package.json } |  |  | 0.594 |
| walker |  | 4366 | 65 | Code::CodeKey { rung: Decl, file: webpack-plugin/src/index.ts, decl: 10, sub: 0, line: 150 } |  |  | 0.594 |
| walker |  | 4448 | 82 | Fs::DirListing { dir: samples/legacy } |  |  | 0.594 |
| walker |  | 4452 | 4 | Fs::DirListing { dir: samples/legacy/browser-amd-editor } |  |  | 0.594 |
| walker |  | 4456 | 4 | Fs::DirListing { dir: samples/legacy/browser-amd-localized } |  |  | 0.594 |
| walker |  | 4460 | 4 | Fs::DirListing { dir: samples/legacy/browser-amd-monarch } |  |  | 0.594 |
| walker |  | 4464 | 4 | Fs::DirListing { dir: samples/legacy/browser-amd-requirejs } |  |  | 0.594 |
| walker |  | 4468 | 4 | Fs::DirListing { dir: samples/legacy/browser-amd-shadow-dom } |  |  | 0.594 |
| walker |  | 4472 | 4 | Fs::DirListing { dir: samples/legacy/browser-amd-shared-model } |  |  | 0.594 |
| walker |  | 4476 | 4 | Fs::DirListing { dir: samples/legacy/browser-amd-trusted-types } |  |  | 0.594 |
| walker |  | 4484 | 8 | Fs::DirListing { dir: samples/legacy/browser-amd-iframe } |  |  | 0.594 |
| walker |  | 4496 | 12 | Fs::DirListing { dir: samples/legacy/browser-amd-diff-editor } |  |  | 0.580 |
| ns | 4496 |  | 259 | json/register.ts — every top-level export (names only) | 4.4 |  | 0.580 |
| walker |  | 4513 | 17 | Fs::DirListing { dir: samples/legacy/electron-amd } |  |  | 0.580 |
| walker |  | 4530 | 17 | Fs::DirListing { dir: samples/legacy/electron-amd-nodeIntegration } |  |  | 0.580 |
| walker |  | 4546 | 16 | Json::Identity { file: samples/legacy/electron-amd/package.json } |  |  | 0.580 |
| walker |  | 4565 | 19 | Json::Identity { file: samples/legacy/electron-amd-nodeIntegration/package.json } |  |  | 0.580 |
| walker |  | 4639 | 74 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 12, sub: 0, line: 113 } |  |  | 0.580 |
| walker |  | 4677 | 38 | Markdown::HeadingsOutline { file: samples/README.md } |  |  | 0.580 |
| walker |  | 4686 | 9 | Markdown::Section { file: samples/README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.580 |
| walker |  | 4763 | 77 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 14, sub: 0, line: 134 } |  |  | 0.580 |
| ns | 4765 |  | 269 | json defaults: diagnostic option values, mode toggles, jsonDefaults, the worker interface | 4.5 | 4.4 | 0.561 |
| walker |  | 4842 | 79 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 1, sub: 0, line: 7 } |  |  | 0.561 |
| walker |  | 4869 | 27 | Json::Entry { file: monaco-lsp-client/package.json } |  |  | 0.561 |
| walker |  | 4897 | 28 | Json::Entry { file: webpack-plugin/package.json } |  |  | 0.562 |
| walker |  | 4984 | 87 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 11, sub: 0, line: 101 } |  |  | 0.562 |
| ns | 5027 |  | 262 | html/register.ts — export names plus the registerHTMLLanguageService factory | 4.6 |  | 0.549 |
| walker |  | 5029 | 45 | Markdown::HeadingsOutline { file: webpack-plugin/README.md } |  |  | 0.549 |
| walker |  | 5127 | 98 | Code::CodeKey { rung: Decl, file: webpack-plugin/src/index.ts, decl: 18, sub: 0, line: 238 } |  |  | 0.549 |
| walker |  | 5228 | 101 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 19, sub: 0, line: 170 } |  |  | 0.549 |
| walker |  | 5239 | 11 | Json::Dependencies { file: samples/browser-esm-vite/package.json } |  |  | 0.549 |
| walker |  | 5250 | 11 | Json::Dependencies { file: samples/browser-esm-vite-react/package.json } |  |  | 0.549 |
| walker |  | 5261 | 11 | Json::Dependencies { file: samples/browser-esm-webpack-typescript-react/package.json } |  |  | 0.549 |
| ns | 5311 |  | 284 | html: the three pre-registered services and their per-language toggles | 4.7 | 4.6 | 0.535 |
| walker |  | 5373 | 112 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 20, sub: 0, line: 183 } |  |  | 0.535 |
| walker |  | 5410 | 37 | Json::Scripts { file: samples/package.json } |  |  | 0.535 |
| walker |  | 5524 | 114 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 10, sub: 0, line: 86 } |  |  | 0.535 |
| ns | 5556 |  | 245 | typescript/register.ts — every top-level export (names only) | 4.8 |  | 0.525 |
| walker |  | 5557 | 33 | Markdown::Section { file: samples/legacy/README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.525 |
| walker |  | 5703 | 146 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.552 |
| walker |  | 5747 | 44 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.552 |
| ns | 5791 |  | 235 | LanguageServiceDefaults (typescript) — all members | 4.9 |  | 0.542 |
| walker |  | 5807 | 60 | Fs::DirListing { dir: website } |  |  | 0.543 |
| walker |  | 5820 | 13 | Json::Entry { file: samples/legacy/electron-amd/package.json } |  |  | 0.543 |
| walker |  | 5833 | 13 | Json::Entry { file: samples/legacy/electron-amd-nodeIntegration/package.json } |  |  | 0.543 |
| walker |  | 5911 | 78 | Json::IdentityMeta { file: package.json } |  |  | 0.549 |
| walker |  | 5939 | 28 | Markdown::Section { file: webpack-plugin/README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.549 |
| walker |  | 5974 | 35 | Fs::DirListing { dir: src/languages/features/typescript/lib } |  |  | 0.562 |
| walker |  | 6008 | 34 | Json::Dependencies { file: webpack-plugin/package.json } |  |  | 0.562 |
| ns | 6027 |  | 236 | TypeScriptWorker — all 21 proxy methods (names only) | 4.10 | 4.8 | 0.552 |
| walker |  | 6091 | 83 | Markdown::HeadingsOutline { file: docs/integrate-esm.md } |  |  | 0.552 |
| walker |  | 6127 | 36 | Json::Dependencies { file: samples/package.json } |  |  | 0.552 |
| walker |  | 6157 | 30 | Markdown::Section { file: README.md, section_index: 13, keeps_default_concavity: false } |  |  | 0.552 |
| walker |  | 6187 | 30 | Json::Scripts { file: samples/browser-esm-esbuild/package.json } |  |  | 0.552 |
| ns | 6318 |  | 291 | typescript/javascript defaults and their divergence | 4.11 | 4.8 | 0.539 |
| ns | 6547 |  | 229 | The four *Mode.ts entry points and the tsMode re-export tail | 5.1 |  | 0.534 |
| ns | 6761 |  | 214 | cssMode.setupMode — WorkerManager, the worker accessor, and ModeConfiguration-gated provider registration | 5.2 | 5.1 | 0.524 |
| walker |  | 6806 | 619 | Code::CodeKey { rung: Decl, file: webpack-plugin/src/index.ts, decl: 9, sub: 0, line: 102 } |  |  | 0.525 |
| walker |  | 6837 | 31 | Json::Scripts { file: samples/browser-esm-parcel/package.json } |  |  | 0.525 |
| walker |  | 6906 | 69 | Json::Scripts { file: webpack-plugin/package.json } |  |  | 0.525 |
| walker |  | 7072 | 166 | Json::Entry { file: package.json } |  |  | 0.526 |
| walker |  | 7095 | 23 | Fs::DirListing { dir: scripts/ci } |  |  | 0.526 |
| ns | 7114 |  | 353 | common/lspLanguageFeatures.ts — the shared provider adapters and LSP conversion helpers | 5.3 |  | 0.517 |
| walker |  | 7138 | 43 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.517 |
| walker |  | 7176 | 38 | Json::Scripts { file: samples/browser-esm-webpack/package.json } |  |  | 0.517 |
| walker |  | 7214 | 38 | Json::Scripts { file: samples/browser-esm-webpack-monaco-plugin/package.json } |  |  | 0.517 |
| walker |  | 7227 | 13 | Code::CodeKey { rung: Names, file: webpack-plugin/src/loader-utils.d.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.517 |
| walker |  | 7307 | 80 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.517 |
| walker |  | 7333 | 26 | Fs::DirListing { dir: website/src } |  |  | 0.518 |
| walker |  | 7345 | 12 | Fs::DirListing { dir: website/src/runner } |  |  | 0.518 |
| ns | 7421 |  | 307 | Web-worker plumbing: MonacoEnvironment hooks, createWebWorker, IWebWorkerOptions fields, worker entry points | 5.4 |  | 0.508 |
| walker |  | 7427 | 82 | Json::Scripts { file: monaco-lsp-client/package.json } |  |  | 0.508 |
| walker |  | 7437 | 10 | Code::CodeKey { rung: Names, file: src/deprecated/editor/editor.main.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.508 |
| walker |  | 7447 | 10 | Code::CodeKey { rung: Names, file: src/deprecated/editor/editor.worker.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.508 |
| walker |  | 7501 | 54 | Json::Scripts { file: samples/browser-esm-vite/package.json } |  |  | 0.508 |
| walker |  | 7557 | 56 | Json::Scripts { file: samples/browser-esm-webpack-small/package.json } |  |  | 0.508 |
| ns | 7576 |  | 155 | WorkerManager lifecycle and the CSSWorker entry points | 5.5 |  | 0.504 |
| walker |  | 7591 | 34 | Json::Scripts { file: samples/legacy/electron-amd/package.json } |  |  | 0.504 |
| walker |  | 7625 | 34 | Json::Scripts { file: samples/legacy/electron-amd-nodeIntegration/package.json } |  |  | 0.504 |
| walker |  | 7684 | 59 | Json::Scripts { file: samples/electron-esm-webpack/package.json } |  |  | 0.504 |
| walker |  | 7747 | 63 | Json::Scripts { file: samples/browser-esm-webpack-typescript/package.json } |  |  | 0.504 |
| walker |  | 7806 | 59 | Markdown::Section { file: README.md, section_index: 11, keeps_default_concavity: false } |  |  | 0.504 |
| ns | 7830 |  | 254 | typescript/languageFeatures.ts — the TypeScript-specific adapter roster | 5.6 |  | 0.498 |
| walker |  | 7887 | 81 | Json::Dependencies { file: monaco-lsp-client/package.json } |  |  | 0.498 |
| ns | 7909 |  | 79 | monaco-lsp-client package: complete tree listing | 6.1 |  | 0.507 |
| walker |  | 7953 | 66 | Json::Scripts { file: samples/browser-esm-webpack-typescript-react/package.json } |  |  | 0.507 |
| walker |  | 8063 | 110 | Markdown::Section { file: MAINTAINING.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.507 |
| walker |  | 8066 | 3 | Fs::DirListing { dir: website/index } |  |  | 0.507 |
| walker |  | 8080 | 14 | Code::CodeKey { rung: Names, file: monaco-lsp-client/src/adapters/LspConnection.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.507 |
| walker |  | 8092 | 12 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/src/adapters/LspConnection.ts, decl: 1, sub: 0, line: 6 } |  |  | 0.507 |
| walker |  | 8106 | 14 | Code::CodeKey { rung: Names, file: src/deprecated/basic-languages/monaco.contribution.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.507 |
| ns | 8116 |  | 207 | MonacoLspClient wiring + index exports | 6.2 | 6.1 | 0.502 |
| walker |  | 8130 | 24 | Code::CodeKey { rung: Names, file: src/languages/register.all.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.504 |
| walker |  | 8154 | 24 | Code::CodeKey { rung: Names, file: webpack-plugin/src/types.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.504 |
| ns | 8165 |  | 49 | webpack-plugin package: complete tree listing | 6.3 |  | 0.510 |
| walker |  | 8173 | 19 | Code::CodeKey { rung: Decl, file: webpack-plugin/src/types.ts, decl: 1, sub: 0, line: 1 } |  |  | 0.510 |
| walker |  | 8224 | 51 | Fs::DirListing { dir: test/smoke } |  |  | 0.525 |
| walker |  | 8239 | 15 | Code::CodeKey { rung: Names, file: monaco-lsp-client/src/adapters/LspClient.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.525 |
| walker |  | 8319 | 80 | Fs::DirListing { dir: test/manual } |  |  | 0.525 |
| walker |  | 8336 | 17 | Code::CodeKey { rung: Names, file: src/languages/features/register.all.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.526 |
| walker |  | 8423 | 87 | Json::Scripts { file: samples/browser-esm-vite-react/package.json } |  |  | 0.526 |
| ns | 8449 |  | 284 | webpack-plugin option fields and peer-dependency contract | 6.4 | 6.3 | 0.531 |
| walker |  | 8469 | 46 | Fs::DirListing { dir: website/src/website } |  |  | 0.531 |
| walker |  | 8484 | 15 | Fs::DirListing { dir: website/src/website/data } |  |  | 0.531 |
| walker |  | 8511 | 27 | Fs::DirListing { dir: website/src/website/pages } |  |  | 0.531 |
| walker |  | 8545 | 34 | Fs::DirListing { dir: website/src/website/data/playground-samples } |  |  | 0.531 |
| walker |  | 8564 | 19 | Fs::DirListing { dir: website/src/website/data/playground-samples/customizing-the-appearence } |  |  | 0.531 |
| walker |  | 8585 | 21 | Fs::DirListing { dir: website/src/website/data/playground-samples/creating-the-diffeditor } |  |  | 0.531 |
| ns | 8613 |  | 164 | build/ and scripts/ tree listings | 7.1 |  | 0.521 |
| walker |  | 8616 | 31 | Fs::DirListing { dir: website/src/website/data/playground-samples/creating-the-editor } |  |  | 0.521 |
| walker |  | 8656 | 40 | Fs::DirListing { dir: website/src/website/components } |  |  | 0.521 |
| walker |  | 8698 | 42 | Fs::DirListing { dir: website/src/website/utils } |  |  | 0.521 |
| ns | 8762 |  | 149 | build-monaco-editor.ts run() — what the published package is made of | 7.2 |  | 0.517 |
| walker |  | 8802 | 104 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.517 |
| walker |  | 8878 | 76 | Markdown::Section { file: README.md, section_index: 12, keeps_default_concavity: false } |  |  | 0.517 |
| walker |  | 8929 | 51 | Markdown::Section { file: MAINTAINING.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.517 |
| walker |  | 8996 | 67 | Fs::DirListing { dir: website/src/website/pages/playground } |  |  | 0.517 |
| ns | 9000 |  | 238 | check-samples.ts — the invariant every new language must satisfy | 7.3 |  | 0.510 |
| walker |  | 9071 | 75 | Fs::DirListing { dir: website/src/website/data/playground-samples/extending-language-services } |  |  | 0.510 |
| walker |  | 9084 | 13 | Code::CodeKey { rung: Names, file: src/deprecated/language/css/monaco.contribution.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.511 |
| walker |  | 9097 | 13 | Code::CodeKey { rung: Names, file: src/deprecated/language/html/monaco.contribution.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.511 |
| walker |  | 9110 | 13 | Code::CodeKey { rung: Names, file: src/deprecated/language/json/monaco.contribution.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.511 |
| walker |  | 9123 | 13 | Code::CodeKey { rung: Names, file: src/languages/features/css/workerManager.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.511 |
| walker |  | 9136 | 13 | Code::CodeKey { rung: Names, file: src/languages/features/html/workerManager.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.511 |
| walker |  | 9149 | 13 | Code::CodeKey { rung: Names, file: src/languages/features/json/workerManager.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.511 |
| walker |  | 9162 | 13 | Code::CodeKey { rung: Names, file: src/languages/features/typescript/workerManager.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.511 |
| ns | 9166 |  | 166 | package.json distribution fields: typings, main, module, exports | 8.1 |  | 0.516 |
| walker |  | 9197 | 35 | Code::CodeKey { rung: Names, file: monaco-lsp-client/src/utils.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.516 |
| walker |  | 9209 | 12 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/src/utils.ts, decl: 1, sub: 0, line: 1 } |  |  | 0.516 |
| walker |  | 9229 | 20 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/src/adapters/LspClient.ts, decl: 1, sub: 0, line: 30 } |  |  | 0.516 |
| walker |  | 9264 | 35 | Code::CodeKey { rung: Decl, file: webpack-plugin/src/types.ts, decl: 2, sub: 0, line: 6 } |  |  | 0.516 |
| walker |  | 9278 | 14 | Code::CodeKey { rung: Names, file: src/deprecated/language/css/css.worker.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.516 |
| walker |  | 9292 | 14 | Code::CodeKey { rung: Names, file: src/deprecated/language/html/html.worker.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.516 |
| walker |  | 9306 | 14 | Code::CodeKey { rung: Names, file: src/deprecated/language/json/json.worker.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.516 |
| walker |  | 9320 | 14 | Code::CodeKey { rung: Names, file: src/deprecated/language/typescript/monaco.contribution.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.516 |
| walker |  | 9342 | 22 | Code::CodeKey { rung: Names, file: monaco-lsp-client/src/adapters/TextDocumentSynchronizer.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.516 |
| walker |  | 9416 | 74 | Fs::DirListing { dir: website/src/website/data/playground-samples/interacting-with-the-editor } |  |  | 0.516 |
| ns | 9420 |  | 254 | README Installing + CHANGELOG head (0.55.x breaking changes) | 8.2 |  | 0.511 |
| walker |  | 9428 | 12 | Code::CodeKey { rung: Body, file: webpack-plugin/src/index.ts, decl: 22, sub: 0, line: 371 } |  |  | 0.511 |
| ns | 9524 |  | 104 | samples/ listing — every integration sample directory | 8.3 |  | 0.518 |
| ns | 9562 |  | 38 | docs/ listing | 8.4 |  | 0.521 |
| walker |  | 9580 | 152 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.523 |
| ns | 9645 |  | 83 | integrate-esm.md section headings | 8.5 | 8.4 | 0.525 |
| walker |  | 9750 | 170 | Json::IdentityMeta { file: webpack-plugin/package.json } |  |  | 0.525 |
| ns | 9771 |  | 126 | website/ and CI/publishing config listings | 8.6 |  | 0.536 |
| walker |  | 9850 | 100 | Markdown::Section { file: samples/README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.536 |
| walker |  | 9866 | 16 | Code::CodeKey { rung: Names, file: src/deprecated/language/typescript/ts.worker.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.536 |
| walker |  | 9883 | 17 | Code::CodeKey { rung: Names, file: monaco-lsp-client/src/adapters/languageFeatures/LspCompletionFeature.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.536 |
| walker |  | 9897 | 14 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/src/adapters/languageFeatures/LspCompletionFeature.ts, decl: 1, sub: 0, line: 17 } |  |  | 0.536 |
| walker |  | 9914 | 17 | Code::CodeKey { rung: Names, file: monaco-lsp-client/src/adapters/languageFeatures/LspDeclarationFeature.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.536 |
| walker |  | 9928 | 14 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/src/adapters/languageFeatures/LspDeclarationFeature.ts, decl: 1, sub: 0, line: 8 } |  |  | 0.536 |
| walker |  | 9945 | 17 | Code::CodeKey { rung: Names, file: monaco-lsp-client/src/adapters/languageFeatures/LspDefinitionFeature.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.536 |
| walker |  | 9959 | 14 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/src/adapters/languageFeatures/LspDefinitionFeature.ts, decl: 1, sub: 0, line: 8 } |  |  | 0.536 |
| walker |  | 9976 | 17 | Code::CodeKey { rung: Names, file: monaco-lsp-client/src/adapters/languageFeatures/LspDiagnosticsFeature.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.536 |
| walker |  | 9991 | 15 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/src/adapters/languageFeatures/LspDiagnosticsFeature.ts, decl: 1, sub: 0, line: 7 } |  |  | 0.536 |

Score(3000)=0.681 I=0.777 C=0.596 ns_rows≤3K=23/56 grid(1000/1442/2080/3000/4327/6240/9000)=0.641/0.709/0.790/0.681/0.680/0.621/0.542

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 67 |  | 67 | README title + one-sentence identity | 1.1 |  | 0.000 |
| ns | 92 |  | 25 | src/ listing — every child of the package source directory | 1.2 |  | 0.000 |
| walker |  | 116 | 116 | listing of '.' |  |  | 0.000 |
| walker |  | 122 | 6 | listing of 'test-results' |  |  | 0.000 |
| walker |  | 147 | 25 | listing of 'src' |  |  | 0.801 |
| walker |  | 150 | 3 | listing of 'src/internal' |  |  | 0.801 |
| walker |  | 161 | 11 | listing of 'src/deprecated' |  |  | 0.802 |
| walker |  | 172 | 11 | listing of 'src/languages' |  |  | 0.805 |
| walker |  | 185 | 13 | listing of 'src/deprecated/language' |  |  | 0.811 |
| walker |  | 206 | 21 | listing of 'src/languages/features' |  |  | 0.826 |
| ns | 216 |  | 124 | src/index.ts — the whole package entry point (9 lines) | 1.3 |  | 0.525 |
| walker |  | 230 | 24 | listing of 'src/languages/features/css' |  |  | 0.525 |
| walker |  | 254 | 24 | listing of 'src/languages/features/html' |  |  | 0.526 |
| walker |  | 281 | 27 | listing of 'monaco-lsp-client' |  |  | 0.526 |
| walker |  | 285 | 4 | listing of 'monaco-lsp-client/generator' |  |  | 0.526 |
| walker |  | 314 | 29 | listing of 'src/languages/features/json' |  |  | 0.528 |
| ns | 332 |  | 116 | Complete repository root listing | 1.4 |  | 0.659 |
| walker |  | 343 | 29 | listing of 'webpack-plugin' |  |  | 0.659 |
| walker |  | 375 | 32 | listing of 'src/languages/features/typescript' |  |  | 0.662 |
| walker |  | 382 | 7 | listing of 'src/deprecated/basic-languages' |  |  | 0.662 |
| walker |  | 397 | 15 | listing of 'monaco-lsp-client/src' |  |  | 0.663 |
| walker |  | 402 | 5 | listing of '.devcontainer' |  |  | 0.663 |
| walker |  | 407 | 5 | listing of '.husky' |  |  | 0.663 |
| walker |  | 415 | 8 | listing of 'src/internal/common' |  |  | 0.667 |
| walker |  | 453 | 38 | listing of 'docs' |  |  | 0.668 |
| walker |  | 459 | 6 | listing of 'scripts' |  |  | 0.668 |
| ns | 471 |  | 139 | package.json identity header (name, version, vscodeRef, license) | 1.5 |  | 0.604 |
| walker |  | 489 | 30 | export names surface in src/index.ts |  |  | 0.610 |
| walker |  | 509 | 20 | listing of 'webpack-plugin/src' |  |  | 0.611 |
| walker |  | 513 | 4 | listing of 'webpack-plugin/src/loaders' |  |  | 0.611 |
| walker |  | 521 | 8 | listing of 'webpack-plugin/src/plugins' |  |  | 0.611 |
| walker |  | 531 | 10 | listing of 'src/deprecated/editor' |  |  | 0.611 |
| walker |  | 538 | 7 | listing of 'src/languages/features/common' |  |  | 0.612 |
| ns | 588 |  | 117 | The three one-to-six-line re-export files src/index.ts depends on | 1.6 | 1.2 | 0.561 |
| walker |  | 605 | 67 | README headline in README.md |  |  | 0.722 |
| ns | 734 |  | 146 | All README section headings (locations) | 1.7 |  | 0.640 |
| walker |  | 877 | 272 | listing of 'src/features' |  |  | 0.673 |
| ns | 927 |  | 193 | CONTRIBUTING 'Source Code Structure' — the monaco-editor-core boundary | 1.8 |  | 0.641 |
| ns | 1097 |  | 170 | README Concepts: Models and URIs | 1.9 | 1.7 | 0.633 |
| walker |  | 1181 | 304 | listing of 'src/languages/definitions' |  |  | 0.658 |
| ns | 1259 |  | 162 | README Concepts: Editors, Providers, Disposables | 1.10 | 1.7 | 0.646 |
| walker |  | 1287 | 106 | package identity in package.json |  |  | 0.681 |
| ns | 1326 |  | 67 | Listings of every src/ subtree except the two huge uniform ones | 2.1 |  | 0.728 |
| walker |  | 1335 | 48 | package identity in monaco-lsp-client/package.json |  |  | 0.728 |
| walker |  | 1347 | 12 | listing of '.azure-pipelines' |  |  | 0.728 |
| walker |  | 1359 | 12 | listing of '.vscode' |  |  | 0.728 |
| walker |  | 1372 | 13 | listing of 'test' |  |  | 0.728 |
| walker |  | 1384 | 12 | listing of 'src/deprecated/language/css' |  |  | 0.728 |
| walker |  | 1396 | 12 | listing of 'src/deprecated/language/html' |  |  | 0.728 |
| walker |  | 1408 | 12 | listing of 'src/deprecated/language/json' |  |  | 0.728 |
| walker |  | 1420 | 12 | listing of 'src/deprecated/language/typescript' |  |  | 0.728 |
| walker |  | 1424 | 4 | listing of 'scripts/lib' |  |  | 0.728 |
| ns | 1432 |  | 106 | Uniform shape of a feature shim (hover, gpu) and the one that is not uniform (find) | 2.2 |  | 0.709 |
| walker |  | 1528 | 104 | listing of 'samples' |  |  | 0.711 |
| walker |  | 1532 | 4 | listing of 'samples/nwjs-amd' |  |  | 0.711 |
| walker |  | 1536 | 4 | listing of 'samples/nwjs-amd-v2' |  |  | 0.711 |
| walker |  | 1557 | 21 | listing of 'samples/browser-esm-parcel' |  |  | 0.711 |
| walker |  | 1565 | 8 | listing of 'samples/browser-esm-parcel/src' |  |  | 0.711 |
| walker |  | 1580 | 15 | package identity in samples/browser-esm-parcel/package.json |  |  | 0.711 |
| walker |  | 1602 | 22 | listing of 'samples/browser-esm-webpack-typescript' |  |  | 0.711 |
| walker |  | 1610 | 8 | listing of 'samples/browser-esm-webpack-typescript/src' |  |  | 0.711 |
| walker |  | 1634 | 24 | listing of 'samples/browser-esm-esbuild' |  |  | 0.711 |
| walker |  | 1649 | 15 | package identity in samples/browser-esm-esbuild/package.json |  |  | 0.711 |
| walker |  | 1674 | 25 | listing of 'samples/browser-esm-webpack' |  |  | 0.711 |
| walker |  | 1691 | 17 | package identity in samples/browser-esm-webpack/package.json |  |  | 0.711 |
| ns | 1704 |  | 272 | Complete listing of src/features/ — all 65 editor-feature shim directories | 2.3 |  | 0.779 |
| walker |  | 1716 | 25 | listing of 'samples/browser-esm-webpack-monaco-plugin' |  |  | 0.779 |
| walker |  | 1741 | 25 | listing of 'samples/electron-esm-webpack' |  |  | 0.779 |
| ns | 1748 |  | 44 | src/features/register.all.ts head, elided middle, tail | 2.4 |  | 0.767 |
| walker |  | 1758 | 17 | package identity in samples/electron-esm-webpack/package.json |  |  | 0.767 |
| walker |  | 1785 | 27 | listing of 'samples/browser-esm-webpack-typescript-react' |  |  | 0.767 |
| walker |  | 1805 | 20 | package identity in samples/browser-esm-webpack-monaco-plugin/package.json |  |  | 0.767 |
| walker |  | 1825 | 20 | package identity in samples/browser-esm-webpack-typescript/package.json |  |  | 0.767 |
| walker |  | 1845 | 20 | package identity in samples/browser-esm-webpack-typescript-react/package.json |  |  | 0.767 |
| walker |  | 1861 | 16 | listing of 'samples/browser-esm-webpack-typescript-react/src' |  |  | 0.767 |
| walker |  | 1866 | 5 | listing of 'samples/browser-esm-webpack-typescript-react/src/components' |  |  | 0.767 |
| walker |  | 1897 | 31 | README headline in samples/README.md |  |  | 0.767 |
| walker |  | 1928 | 31 | listing of 'samples/browser-esm-vite-react' |  |  | 0.767 |
| walker |  | 1947 | 19 | package identity in samples/browser-esm-vite-react/package.json |  |  | 0.767 |
| walker |  | 1978 | 31 | listing of 'samples/browser-esm-webpack-small' |  |  | 0.767 |
| walker |  | 1996 | 18 | package identity in samples/browser-esm-webpack-small/package.json |  |  | 0.767 |
| walker |  | 2015 | 19 | listing of 'samples/browser-esm-vite-react/src' |  |  | 0.767 |
| walker |  | 2025 | 10 | listing of 'samples/browser-esm-vite-react/src/components' |  |  | 0.767 |
| ns | 2052 |  | 304 | Complete listing of src/languages/definitions/ — all 84 Monarch languages plus _.contribution.ts, register.all.ts and test/ | 2.5 |  | 0.795 |
| walker |  | 2061 | 36 | listing of 'samples/browser-esm-vite' |  |  | 0.795 |
| ns | 2065 |  | 13 | One language definition directory, listed in full (rust) | 2.6 |  | 0.790 |
| walker |  | 2079 | 18 | package identity in samples/browser-esm-vite/package.json |  |  | 0.790 |
| walker |  | 2141 | 62 | package identity in webpack-plugin/package.json |  |  | 0.790 |
| ns | 2149 |  | 84 | rust/register.ts registerLanguage call | 2.7 | 2.6 | 0.777 |
| walker |  | 2212 | 71 | package identity in samples/package.json |  |  | 0.777 |
| walker |  | 2224 | 12 | package entrypoints in samples/package.json |  |  | 0.777 |
| walker |  | 2279 | 55 | README headline in monaco-lsp-client/README.md |  |  | 0.777 |
| walker |  | 2335 | 56 | README headline in webpack-plugin/README.md |  |  | 0.777 |
| walker |  | 2345 | 10 | package identity metadata in monaco-lsp-client/package.json |  |  | 0.777 |
| walker |  | 2365 | 20 | listing of '.github' |  |  | 0.777 |
| walker |  | 2373 | 8 | listing of '.github/workflows' |  |  | 0.777 |
| walker |  | 2410 | 37 | listing of 'monaco-lsp-client/src/adapters' |  |  | 0.778 |
| ns | 2431 |  | 282 | _.contribution.ts: ILang/ILangImpl and registerLanguage() | 2.8 |  | 0.741 |
| ns | 2570 |  | 139 | _.contribution.ts: LazyLanguageLoader and loadLanguage() | 2.9 |  | 0.725 |
| walker |  | 2584 | 174 | listing of 'monaco-lsp-client/src/adapters/languageFeatures' |  |  | 0.725 |
| walker |  | 2620 | 36 | headings outline in MAINTAINING.md |  |  | 0.725 |
| walker |  | 2659 | 39 | package scripts in samples/package.json |  |  | 0.715 |
| ns | 2659 |  | 89 | Grammar file shape: rust.ts conf + language exports | 2.10 |  | 0.715 |
| walker |  | 2669 | 10 | package identity metadata in samples/package.json |  |  | 0.715 |
| walker |  | 2680 | 11 | package entrypoints in samples/electron-esm-webpack/package.json |  |  | 0.715 |
| walker |  | 2762 | 82 | listing of 'samples/legacy' |  |  | 0.715 |
| walker |  | 2766 | 4 | listing of 'samples/legacy/browser-amd-editor' |  |  | 0.715 |
| walker |  | 2770 | 4 | listing of 'samples/legacy/browser-amd-localized' |  |  | 0.715 |
| walker |  | 2774 | 4 | listing of 'samples/legacy/browser-amd-monarch' |  |  | 0.715 |
| walker |  | 2778 | 4 | listing of 'samples/legacy/browser-amd-requirejs' |  |  | 0.715 |
| walker |  | 2782 | 4 | listing of 'samples/legacy/browser-amd-shadow-dom' |  |  | 0.715 |
| walker |  | 2786 | 4 | listing of 'samples/legacy/browser-amd-shared-model' |  |  | 0.715 |
| walker |  | 2790 | 4 | listing of 'samples/legacy/browser-amd-trusted-types' |  |  | 0.715 |
| walker |  | 2798 | 8 | listing of 'samples/legacy/browser-amd-iframe' |  |  | 0.715 |
| ns | 2799 |  | 140 | Tokenization test harness: testRunner.ts types + testTokenization(), and a test file head | 2.11 |  | 0.699 |
| walker |  | 2810 | 12 | listing of 'samples/legacy/browser-amd-diff-editor' |  |  | 0.699 |
| walker |  | 2827 | 17 | listing of 'samples/legacy/electron-amd' |  |  | 0.699 |
| ns | 2841 |  | 42 | src/languages/definitions/register.all.ts head, elided middle, tail | 2.12 |  | 0.693 |
| walker |  | 2844 | 17 | listing of 'samples/legacy/electron-amd-nodeIntegration' |  |  | 0.693 |
| walker |  | 2860 | 16 | package identity in samples/legacy/electron-amd/package.json |  |  | 0.693 |
| walker |  | 2879 | 19 | package identity in samples/legacy/electron-amd-nodeIntegration/package.json |  |  | 0.693 |
| walker |  | 2917 | 38 | headings outline in samples/README.md |  |  | 0.693 |
| walker |  | 2926 | 9 | samples/README.md section #4 |  |  | 0.693 |
| walker |  | 2953 | 27 | package entrypoints in monaco-lsp-client/package.json |  |  | 0.693 |
| ns | 2976 |  | 135 | src/deprecated/: the legacy import paths, all one-line re-exports | 2.13 |  | 0.681 |
| walker |  | 2981 | 28 | package entrypoints in webpack-plugin/package.json |  |  | 0.681 |
| walker |  | 3026 | 45 | headings outline in webpack-plugin/README.md |  |  | 0.681 |
| walker |  | 3129 | 103 | json config editor.code-workspace |  |  | 0.681 |
| walker |  | 3157 | 28 | export names surface in monaco-lsp-client/src/index.ts |  |  | 0.681 |
| walker |  | 3174 | 17 | export names surface in src/languages/features/register.all.ts |  |  | 0.681 |
| walker |  | 3207 | 33 | samples/legacy/README.md section #0 |  |  | 0.681 |
| ns | 3345 |  | 369 | package.json scripts — build, test and dev commands (packaging variants elided) | 3.1 |  | 0.661 |
| walker |  | 3353 | 146 | headings outline in README.md |  |  | 0.697 |
| walker |  | 3397 | 44 | README.md section #0 |  |  | 0.697 |
| walker |  | 3457 | 60 | listing of 'website' |  |  | 0.698 |
| walker |  | 3470 | 13 | package entrypoints in samples/legacy/electron-amd/package.json |  |  | 0.698 |
| ns | 3477 |  | 132 | CONTRIBUTING: the ordered editor-test command sequence | 3.2 |  | 0.687 |
| walker |  | 3483 | 13 | package entrypoints in samples/legacy/electron-amd-nodeIntegration/package.json |  |  | 0.687 |
| walker |  | 3511 | 28 | webpack-plugin/README.md section #1 |  |  | 0.687 |
| ns | 3541 |  | 64 | test/ listing plus the full smoke-test directory | 3.3 |  | 0.673 |
| walker |  | 3546 | 35 | listing of 'src/languages/features/typescript/lib' |  |  | 0.674 |
| walker |  | 3608 | 62 | imports in src/index.ts |  |  | 0.699 |
| walker |  | 3691 | 83 | headings outline in docs/integrate-esm.md |  |  | 0.700 |
| ns | 3692 |  | 151 | Listings of the five src/languages/features dirs — the uniform rich-service file set | 4.1 |  | 0.713 |
| walker |  | 3721 | 30 | README.md section #21 |  |  | 0.713 |
| walker |  | 3799 | 78 | package identity metadata in package.json |  |  | 0.723 |
| walker |  | 3829 | 30 | package scripts in samples/browser-esm-esbuild/package.json |  |  | 0.723 |
| walker |  | 3875 | 46 | README.md section #4 |  |  | 0.723 |
| walker |  | 3906 | 31 | package scripts in samples/browser-esm-parcel/package.json |  |  | 0.723 |
| walker |  | 3926 | 20 | imports in src/editor.ts |  |  | 0.724 |
| ns | 3952 |  | 260 | css/register.ts — every top-level export (names only) | 4.2 |  | 0.707 |
| walker |  | 4092 | 166 | package entrypoints in package.json |  |  | 0.708 |
| walker |  | 4115 | 23 | listing of 'scripts/ci' |  |  | 0.708 |
| walker |  | 4158 | 43 | README.md section #17 |  |  | 0.708 |
| ns | 4237 |  | 285 | css: the default ModeConfiguration toggles, the three defaults objects, and the lazy onLanguage hookup | 4.3 | 4.2 | 0.680 |
| walker |  | 4238 | 80 | README.md section #1 |  |  | 0.680 |
| walker |  | 4307 | 69 | package scripts in webpack-plugin/package.json |  |  | 0.680 |
| walker |  | 4333 | 26 | listing of 'website/src' |  |  | 0.680 |
| walker |  | 4345 | 12 | listing of 'website/src/runner' |  |  | 0.680 |
| walker |  | 4383 | 38 | package scripts in samples/browser-esm-webpack/package.json |  |  | 0.680 |
| walker |  | 4421 | 38 | package scripts in samples/browser-esm-webpack-monaco-plugin/package.json |  |  | 0.680 |
| walker |  | 4457 | 36 | package runtime dependencies in samples/package.json |  |  | 0.680 |
| ns | 4496 |  | 259 | json/register.ts — every top-level export (names only) | 4.4 |  | 0.664 |
| walker |  | 4539 | 82 | package scripts in monaco-lsp-client/package.json |  |  | 0.664 |
| walker |  | 4544 | 5 | docs/integrate-esm.md section #6 |  |  | 0.664 |
| walker |  | 4549 | 5 | docs/integrate-esm.md section #11 |  |  | 0.664 |
| walker |  | 4554 | 5 | docs/integrate-esm.md section #19 |  |  | 0.664 |
| walker |  | 4565 | 11 | package runtime dependencies in samples/browser-esm-vite/package.json |  |  | 0.664 |
| walker |  | 4576 | 11 | package runtime dependencies in samples/browser-esm-vite-react/package.json |  |  | 0.664 |
| walker |  | 4587 | 11 | package runtime dependencies in samples/browser-esm-webpack-typescript-react/package.json |  |  | 0.664 |
| walker |  | 4641 | 54 | package scripts in samples/browser-esm-vite/package.json |  |  | 0.664 |
| walker |  | 4697 | 56 | package scripts in samples/browser-esm-webpack-small/package.json |  |  | 0.664 |
| walker |  | 4756 | 59 | README.md section #19 |  |  | 0.664 |
| ns | 4765 |  | 269 | json defaults: diagnostic option values, mode toggles, jsonDefaults, the worker interface | 4.5 | 4.4 | 0.643 |
| walker |  | 4790 | 34 | package scripts in samples/legacy/electron-amd/package.json |  |  | 0.643 |
| walker |  | 4824 | 34 | package scripts in samples/legacy/electron-amd-nodeIntegration/package.json |  |  | 0.643 |
| walker |  | 4934 | 110 | MAINTAINING.md section #0 |  |  | 0.643 |
| walker |  | 4993 | 59 | package scripts in samples/electron-esm-webpack/package.json |  |  | 0.643 |
| walker |  | 4996 | 3 | listing of 'website/index' |  |  | 0.643 |
| ns | 5027 |  | 262 | html/register.ts — export names plus the registerHTMLLanguageService factory | 4.6 |  | 0.628 |
| walker |  | 5059 | 63 | package scripts in samples/browser-esm-webpack-typescript/package.json |  |  | 0.628 |
| walker |  | 5110 | 51 | listing of 'test/smoke' |  |  | 0.647 |
| walker |  | 5120 | 10 | export names surface in src/deprecated/editor/editor.worker.ts |  |  | 0.647 |
| walker |  | 5186 | 66 | package scripts in samples/browser-esm-webpack-typescript-react/package.json |  |  | 0.647 |
| ns | 5311 |  | 284 | html: the three pre-registered services and their per-language toggles | 4.7 | 4.6 | 0.631 |
| ns | 5556 |  | 245 | typescript/register.ts — every top-level export (names only) | 4.8 |  | 0.619 |
| ns | 5791 |  | 235 | LanguageServiceDefaults (typescript) — all members | 4.9 |  | 0.608 |
| walker |  | 5833 | 647 | package scripts in package.json |  |  | 0.631 |
| walker |  | 5858 | 25 | imports in samples/browser-esm-vite/main.ts |  |  | 0.631 |
| walker |  | 5884 | 26 | imports in monaco-lsp-client/generator/index.ts |  |  | 0.631 |
| ns | 6027 |  | 236 | TypeScriptWorker — all 21 proxy methods (names only) | 4.10 | 4.8 | 0.619 |
| walker |  | 6031 | 147 | module item at monaco-lsp-client/generator/index.ts:7 |  |  | 0.619 |
| walker |  | 6041 | 10 | module item body at monaco-lsp-client/generator/index.ts:7 body 42 |  |  | 0.619 |
| walker |  | 6053 | 12 | module item body at monaco-lsp-client/generator/index.ts:7 body 58 |  |  | 0.619 |
| walker |  | 6075 | 22 | module item body at monaco-lsp-client/generator/index.ts:7 body 65 |  |  | 0.619 |
| walker |  | 6108 | 33 | module item body at monaco-lsp-client/generator/index.ts:7 body 49 |  |  | 0.619 |
| walker |  | 6134 | 26 | imports in samples/browser-esm-webpack-typescript/src/index.ts |  |  | 0.619 |
| walker |  | 6214 | 80 | listing of 'test/manual' |  |  | 0.619 |
| walker |  | 6238 | 24 | imports in src/languages/register.all.ts |  |  | 0.621 |
| walker |  | 6297 | 59 | module item body at monaco-lsp-client/generator/index.ts:7 body 16 |  |  | 0.621 |
| ns | 6318 |  | 291 | typescript/javascript defaults and their divergence | 4.11 | 4.8 | 0.607 |
| walker |  | 6343 | 46 | listing of 'website/src/website' |  |  | 0.607 |
| walker |  | 6358 | 15 | listing of 'website/src/website/data' |  |  | 0.607 |
| walker |  | 6385 | 27 | listing of 'website/src/website/pages' |  |  | 0.607 |
| walker |  | 6419 | 34 | listing of 'website/src/website/data/playground-samples' |  |  | 0.607 |
| walker |  | 6438 | 19 | listing of 'website/src/website/data/playground-samples/customizing-the-appearence' |  |  | 0.607 |
| walker |  | 6459 | 21 | listing of 'website/src/website/data/playground-samples/creating-the-diffeditor' |  |  | 0.607 |
| walker |  | 6490 | 31 | listing of 'website/src/website/data/playground-samples/creating-the-editor' |  |  | 0.607 |
| walker |  | 6530 | 40 | listing of 'website/src/website/components' |  |  | 0.607 |
| ns | 6547 |  | 229 | The four *Mode.ts entry points and the tsMode re-export tail | 5.1 |  | 0.601 |
| walker |  | 6572 | 42 | listing of 'website/src/website/utils' |  |  | 0.601 |
| walker |  | 6676 | 104 | README.md section #3 |  |  | 0.601 |
| walker |  | 6752 | 76 | README.md section #20 |  |  | 0.601 |
| ns | 6761 |  | 214 | cssMode.setupMode — WorkerManager, the worker accessor, and ModeConfiguration-gated provider registration | 5.2 | 5.1 | 0.590 |
| walker |  | 6803 | 51 | MAINTAINING.md section #2 |  |  | 0.590 |
| walker |  | 6837 | 34 | package runtime dependencies in webpack-plugin/package.json |  |  | 0.590 |
| walker |  | 6904 | 67 | listing of 'website/src/website/pages/playground' |  |  | 0.590 |
| walker |  | 6991 | 87 | package scripts in samples/browser-esm-vite-react/package.json |  |  | 0.590 |
| walker |  | 7066 | 75 | listing of 'website/src/website/data/playground-samples/extending-language-services' |  |  | 0.590 |
| ns | 7114 |  | 353 | common/lspLanguageFeatures.ts — the shared provider adapters and LSP conversion helpers | 5.3 |  | 0.581 |
| walker |  | 7309 | 243 | module item at monaco-lsp-client/generator/index.ts:259 |  |  | 0.581 |
| walker |  | 7320 | 11 | module item body at monaco-lsp-client/generator/index.ts:259 body 631 |  |  | 0.581 |
| walker |  | 7332 | 12 | module item body at monaco-lsp-client/generator/index.ts:259 body 357 |  |  | 0.581 |
| walker |  | 7342 | 10 | module item body at monaco-lsp-client/generator/index.ts:259 body 359 |  |  | 0.581 |
| walker |  | 7354 | 12 | module item body at monaco-lsp-client/generator/index.ts:259 body 372 |  |  | 0.581 |
| walker |  | 7365 | 11 | module item body at monaco-lsp-client/generator/index.ts:259 body 373 |  |  | 0.581 |
| walker |  | 7375 | 10 | module item body at monaco-lsp-client/generator/index.ts:259 body 374 |  |  | 0.581 |
| walker |  | 7387 | 12 | module item body at monaco-lsp-client/generator/index.ts:259 body 388 |  |  | 0.581 |
| walker |  | 7402 | 15 | export names surface in monaco-lsp-client/src/adapters/LspClient.ts |  |  | 0.581 |
| walker |  | 7417 | 15 | export at monaco-lsp-client/src/adapters/LspClient.ts:30 |  |  | 0.581 |
| ns | 7421 |  | 307 | Web-worker plumbing: MonacoEnvironment hooks, createWebWorker, IWebWorkerOptions fields, worker entry points | 5.4 |  | 0.570 |
| walker |  | 7491 | 74 | listing of 'website/src/website/data/playground-samples/interacting-with-the-editor' |  |  | 0.570 |
| ns | 7576 |  | 155 | WorkerManager lifecycle and the CSSWorker entry points | 5.5 |  | 0.565 |
| walker |  | 7643 | 152 | README.md section #2 |  |  | 0.565 |
| walker |  | 7743 | 100 | samples/README.md section #1 |  |  | 0.565 |
| walker |  | 7780 | 37 | README.md section #15 |  |  | 0.565 |
| walker |  | 7817 | 37 | README.md section #16 |  |  | 0.565 |
| walker |  | 7829 | 12 | module item body at monaco-lsp-client/generator/index.ts:259 body 406 |  |  | 0.565 |
| ns | 7830 |  | 254 | typescript/languageFeatures.ts — the TypeScript-specific adapter roster | 5.6 |  | 0.558 |
| walker |  | 7840 | 11 | module item body at monaco-lsp-client/generator/index.ts:259 body 407 |  |  | 0.558 |
| ns | 7909 |  | 79 | monaco-lsp-client package: complete tree listing | 6.1 |  | 0.566 |
| walker |  | 7939 | 99 | docs/integrate-esm.md section #0 |  |  | 0.566 |
| walker |  | 7980 | 41 | README.md section #12 |  |  | 0.566 |
| walker |  | 8021 | 41 | README.md section #14 |  |  | 0.566 |
| walker |  | 8068 | 47 | imports in samples/browser-esm-vite-react/src/main.tsx |  |  | 0.566 |
| walker |  | 8110 | 42 | README.md section #11 |  |  | 0.566 |
| ns | 8116 |  | 207 | MonacoLspClient wiring + index exports | 6.2 | 6.1 | 0.561 |
| walker |  | 8120 | 10 | module item body at monaco-lsp-client/generator/index.ts:259 body 408 |  |  | 0.561 |
| walker |  | 8132 | 12 | docs/integrate-esm.md section #4 |  |  | 0.561 |
| walker |  | 8144 | 12 | docs/integrate-esm.md section #15 |  |  | 0.561 |
| ns | 8165 |  | 49 | webpack-plugin package: complete tree listing | 6.3 |  | 0.566 |
| walker |  | 8314 | 170 | package identity metadata in webpack-plugin/package.json |  |  | 0.566 |
| walker |  | 8327 | 13 | export names surface in src/languages/features/css/workerManager.ts |  |  | 0.567 |
| walker |  | 8340 | 13 | export names surface in src/languages/features/html/workerManager.ts |  |  | 0.567 |
| walker |  | 8353 | 13 | export names surface in src/languages/features/json/workerManager.ts |  |  | 0.567 |
| walker |  | 8366 | 13 | export names surface in src/languages/features/typescript/workerManager.ts |  |  | 0.567 |
| walker |  | 8418 | 52 | imports in samples/browser-esm-webpack-typescript-react/src/index.tsx |  |  | 0.567 |
| walker |  | 8431 | 13 | docs/integrate-esm.md section #9 |  |  | 0.567 |
| ns | 8449 |  | 284 | webpack-plugin option fields and peer-dependency contract | 6.4 | 6.3 | 0.560 |
| walker |  | 8481 | 50 | README.md section #8 |  |  | 0.560 |
| walker |  | 8592 | 111 | json config webpack-plugin/tsconfig.json |  |  | 0.560 |
| ns | 8613 |  | 164 | build/ and scripts/ tree listings | 7.1 |  | 0.550 |
| walker |  | 8644 | 52 | README.md section #9 |  |  | 0.550 |
| walker |  | 8648 | 4 | listing of 'test/smoke/amd' |  |  | 0.550 |
| walker |  | 8656 | 8 | listing of 'website/static' |  |  | 0.550 |
| walker |  | 8719 | 63 | imports in monaco-lsp-client/src/index.ts |  |  | 0.552 |
| walker |  | 8731 | 12 | module item body at monaco-lsp-client/generator/index.ts:259 body 448 |  |  | 0.552 |
| walker |  | 8740 | 9 | listing of 'website/scripts' |  |  | 0.552 |
| ns | 8762 |  | 149 | build-monaco-editor.ts run() — what the published package is made of | 7.2 |  | 0.548 |
| walker |  | 8847 | 107 | samples/README.md section #3 |  |  | 0.548 |
| walker |  | 8865 | 18 | export names surface in samples/browser-esm-vite-react/src/components/Editor.tsx |  |  | 0.548 |
| walker |  | 8865 | 0 | export at samples/browser-esm-vite-react/src/components/Editor.tsx:5 |  |  | 0.548 |
| walker |  | 8883 | 18 | export names surface in samples/browser-esm-webpack-typescript-react/src/components/Editor.tsx |  |  | 0.548 |
| walker |  | 8883 | 0 | export at samples/browser-esm-webpack-typescript-react/src/components/Editor.tsx:23 |  |  | 0.548 |
| walker |  | 8935 | 52 | README.md section #7 |  |  | 0.549 |
| walker |  | 8994 | 59 | README.md section #13 |  |  | 0.549 |
| ns | 9000 |  | 238 | check-samples.ts — the invariant every new language must satisfy | 7.3 |  | 0.542 |
| walker |  | 9075 | 81 | package runtime dependencies in monaco-lsp-client/package.json |  |  | 0.542 |
| walker |  | 9087 | 12 | module item body at monaco-lsp-client/generator/index.ts:259 body 452 |  |  | 0.542 |
| walker |  | 9097 | 10 | listing of 'website/typedoc' |  |  | 0.542 |
| walker |  | 9126 | 29 | export names surface in src/internal/common/workers.ts |  |  | 0.542 |
| walker |  | 9149 | 23 | export at src/internal/common/workers.ts:92 |  |  | 0.542 |
| walker |  | 9161 | 12 | module item body at monaco-lsp-client/generator/index.ts:259 body 487 |  |  | 0.542 |
| ns | 9166 |  | 166 | package.json distribution fields: typings, main, module, exports | 8.1 |  | 0.548 |
| walker |  | 9167 | 6 | listing of 'website/index/samples' |  |  | 0.548 |
| walker |  | 9315 | 148 | samples/README.md section #2 |  |  | 0.548 |
| walker |  | 9337 | 22 | export names surface in src/languages/features/typescript/ts.worker.ts |  |  | 0.548 |
| walker |  | 9349 | 12 | module item body at monaco-lsp-client/generator/index.ts:259 body 488 |  |  | 0.548 |
| walker |  | 9359 | 10 | module item body at monaco-lsp-client/generator/index.ts:259 body 489 |  |  | 0.548 |
| walker |  | 9410 | 51 | imports in src/languages/features/register.all.ts |  |  | 0.557 |
| ns | 9420 |  | 254 | README Installing + CHANGELOG head (0.55.x breaking changes) | 8.2 |  | 0.553 |
| walker |  | 9422 | 12 | CHANGELOG.md section #0 |  |  | 0.554 |
| walker |  | 9457 | 35 | imports in src/languages/features/css/register.ts |  |  | 0.554 |
| walker |  | 9496 | 39 | export names surface in src/languages/definitions/_.contribution.ts |  |  | 0.554 |
| walker |  | 9496 | 0 | export at src/languages/definitions/_.contribution.ts:55 |  |  | 0.554 |
| walker |  | 9496 | 0 | export at src/languages/definitions/_.contribution.ts:63 |  |  | 0.554 |
| ns | 9524 |  | 104 | samples/ listing — every integration sample directory | 8.3 |  | 0.561 |
| ns | 9562 |  | 38 | docs/ listing | 8.4 |  | 0.563 |
| walker |  | 9566 | 70 | README.md section #10 |  |  | 0.565 |
| ns | 9645 |  | 83 | integrate-esm.md section headings | 8.5 | 8.4 | 0.568 |
| walker |  | 9671 | 105 | package dev/peer dependencies in monaco-lsp-client/package.json |  |  | 0.568 |
| walker |  | 9702 | 31 | package identity in website/package.json |  |  | 0.568 |
| walker |  | 9748 | 46 | export at src/languages/features/html/workerManager.ts:13 |  |  | 0.568 |
| ns | 9771 |  | 126 | website/ and CI/publishing config listings | 8.6 |  | 0.578 |
| walker |  | 9785 | 37 | imports in src/languages/features/html/register.ts |  |  | 0.578 |
| walker |  | 9822 | 37 | imports in src/languages/features/json/register.ts |  |  | 0.578 |
| walker |  | 9869 | 47 | export at src/languages/features/css/workerManager.ts:13 |  |  | 0.579 |
| walker |  | 9916 | 47 | export at src/languages/features/json/workerManager.ts:13 |  |  | 0.579 |
| walker |  | 9935 | 19 | export names surface in src/languages/features/typescript/lib/typescriptServicesMetadata.ts |  |  | 0.579 |

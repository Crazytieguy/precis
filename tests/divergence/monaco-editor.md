Score(3000)=0.675 I=0.771 C=0.592 ns_rows≤3K=23/56 grid(1000/1442/2080/3000/4327/6240/9000)=0.663/0.699/0.784/0.675/0.594/0.552/0.510

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
| walker |  | 284 | 30 | ts names src/index.ts |  |  | 0.543 |
| walker |  | 311 | 27 | listing of 'monaco-lsp-client' |  |  | 0.543 |
| walker |  | 315 | 4 | listing of 'monaco-lsp-client/generator' |  |  | 0.543 |
| ns | 332 |  | 116 | Complete repository root listing | 1.4 |  | 0.663 |
| walker |  | 344 | 29 | listing of 'src/languages/features/json' |  |  | 0.665 |
| walker |  | 373 | 29 | listing of 'webpack-plugin' |  |  | 0.666 |
| walker |  | 405 | 32 | listing of 'src/languages/features/typescript' |  |  | 0.668 |
| walker |  | 412 | 7 | listing of 'src/deprecated/basic-languages' |  |  | 0.668 |
| walker |  | 427 | 15 | listing of 'monaco-lsp-client/src' |  |  | 0.669 |
| walker |  | 455 | 28 | ts names monaco-lsp-client/src/index.ts |  |  | 0.669 |
| walker |  | 460 | 5 | listing of '.devcontainer' |  |  | 0.669 |
| walker |  | 465 | 5 | listing of '.husky' |  |  | 0.669 |
| ns | 471 |  | 139 | package.json identity header (name, version, vscodeRef, license) | 1.5 |  | 0.605 |
| walker |  | 473 | 8 | listing of 'src/internal/common' |  |  | 0.609 |
| walker |  | 511 | 38 | listing of 'docs' |  |  | 0.610 |
| walker |  | 517 | 6 | listing of 'scripts' |  |  | 0.610 |
| walker |  | 537 | 20 | listing of 'webpack-plugin/src' |  |  | 0.611 |
| walker |  | 541 | 4 | listing of 'webpack-plugin/src/loaders' |  |  | 0.611 |
| walker |  | 549 | 8 | listing of 'webpack-plugin/src/plugins' |  |  | 0.611 |
| walker |  | 559 | 10 | listing of 'src/deprecated/editor' |  |  | 0.611 |
| ns | 588 |  | 117 | The three one-to-six-line re-export files src/index.ts depends on | 1.6 | 1.2 | 0.561 |
| walker |  | 629 | 70 | package identity in package.json |  |  | 0.582 |
| walker |  | 636 | 7 | listing of 'src/languages/features/common' |  |  | 0.582 |
| walker |  | 703 | 67 | README headline in README.md |  |  | 0.745 |
| ns | 734 |  | 146 | All README section headings (locations) | 1.7 |  | 0.661 |
| ns | 927 |  | 193 | CONTRIBUTING 'Source Code Structure' — the monaco-editor-core boundary | 1.8 |  | 0.630 |
| walker |  | 975 | 272 | listing of 'src/features' |  |  | 0.662 |
| walker |  | 995 | 20 | ts names src/editor.ts |  |  | 0.663 |
| ns | 1097 |  | 170 | README Concepts: Models and URIs | 1.9 | 1.7 | 0.655 |
| ns | 1259 |  | 162 | README Concepts: Editors, Providers, Disposables | 1.10 | 1.7 | 0.642 |
| walker |  | 1299 | 304 | listing of 'src/languages/definitions' |  |  | 0.668 |
| ns | 1326 |  | 67 | Listings of every src/ subtree except the two huge uniform ones | 2.1 |  | 0.717 |
| walker |  | 1347 | 48 | package identity in monaco-lsp-client/package.json |  |  | 0.717 |
| walker |  | 1359 | 12 | listing of '.azure-pipelines' |  |  | 0.717 |
| walker |  | 1371 | 12 | listing of '.vscode' |  |  | 0.717 |
| walker |  | 1384 | 13 | listing of 'test' |  |  | 0.717 |
| ns | 1432 |  | 106 | Uniform shape of a feature shim (hover, gpu) and the one that is not uniform (find) | 2.2 |  | 0.699 |
| walker |  | 1664 | 280 | ts names monaco-lsp-client/generator/index.ts |  |  | 0.699 |
| walker |  | 1675 | 11 | ts decl monaco-lsp-client/generator/index.ts:82 |  |  | 0.699 |
| walker |  | 1693 | 18 | ts decl monaco-lsp-client/generator/index.ts:259 |  |  | 0.699 |
| ns | 1704 |  | 272 | Complete listing of src/features/ — all 65 editor-feature shim directories | 2.3 |  | 0.771 |
| walker |  | 1714 | 21 | ts decl monaco-lsp-client/generator/index.ts:197 |  |  | 0.771 |
| walker |  | 1735 | 21 | ts decl monaco-lsp-client/generator/index.ts:202 |  |  | 0.771 |
| ns | 1748 |  | 44 | src/features/register.all.ts head, elided middle, tail | 2.4 |  | 0.759 |
| walker |  | 1756 | 21 | ts decl monaco-lsp-client/generator/index.ts:213 |  |  | 0.759 |
| walker |  | 1777 | 21 | ts decl monaco-lsp-client/generator/index.ts:218 |  |  | 0.759 |
| walker |  | 1798 | 21 | ts decl monaco-lsp-client/generator/index.ts:223 |  |  | 0.759 |
| walker |  | 1820 | 22 | ts decl monaco-lsp-client/generator/index.ts:228 |  |  | 0.759 |
| walker |  | 1842 | 22 | ts decl monaco-lsp-client/generator/index.ts:241 |  |  | 0.759 |
| walker |  | 1864 | 22 | ts decl monaco-lsp-client/generator/index.ts:246 |  |  | 0.759 |
| walker |  | 1888 | 24 | ts decl monaco-lsp-client/generator/index.ts:251 |  |  | 0.759 |
| walker |  | 1917 | 29 | ts decl monaco-lsp-client/generator/index.ts:207 |  |  | 0.759 |
| walker |  | 1948 | 31 | ts decl monaco-lsp-client/generator/index.ts:154 |  |  | 0.759 |
| walker |  | 1960 | 12 | listing of 'src/deprecated/language/css' |  |  | 0.759 |
| walker |  | 1972 | 12 | listing of 'src/deprecated/language/html' |  |  | 0.759 |
| walker |  | 1984 | 12 | listing of 'src/deprecated/language/json' |  |  | 0.759 |
| walker |  | 1996 | 12 | listing of 'src/deprecated/language/typescript' |  |  | 0.759 |
| walker |  | 2000 | 4 | listing of 'scripts/lib' |  |  | 0.759 |
| ns | 2052 |  | 304 | Complete listing of src/languages/definitions/ — all 84 Monarch languages plus _.contribution.ts, register.all.ts and test/ | 2.5 |  | 0.789 |
| ns | 2065 |  | 13 | One language definition directory, listed in full (rust) | 2.6 |  | 0.784 |
| walker |  | 2104 | 104 | listing of 'samples' |  |  | 0.785 |
| walker |  | 2108 | 4 | listing of 'samples/nwjs-amd' |  |  | 0.785 |
| walker |  | 2112 | 4 | listing of 'samples/nwjs-amd-v2' |  |  | 0.785 |
| walker |  | 2133 | 21 | listing of 'samples/browser-esm-parcel' |  |  | 0.785 |
| walker |  | 2141 | 8 | listing of 'samples/browser-esm-parcel/src' |  |  | 0.785 |
| ns | 2149 |  | 84 | rust/register.ts registerLanguage call | 2.7 | 2.6 | 0.772 |
| walker |  | 2156 | 15 | package identity in samples/browser-esm-parcel/package.json |  |  | 0.772 |
| walker |  | 2178 | 22 | listing of 'samples/browser-esm-webpack-typescript' |  |  | 0.772 |
| walker |  | 2186 | 8 | listing of 'samples/browser-esm-webpack-typescript/src' |  |  | 0.772 |
| walker |  | 2210 | 24 | listing of 'samples/browser-esm-esbuild' |  |  | 0.772 |
| walker |  | 2225 | 15 | package identity in samples/browser-esm-esbuild/package.json |  |  | 0.772 |
| walker |  | 2250 | 25 | listing of 'samples/browser-esm-webpack' |  |  | 0.772 |
| walker |  | 2267 | 17 | package identity in samples/browser-esm-webpack/package.json |  |  | 0.772 |
| walker |  | 2292 | 25 | listing of 'samples/browser-esm-webpack-monaco-plugin' |  |  | 0.772 |
| walker |  | 2317 | 25 | listing of 'samples/electron-esm-webpack' |  |  | 0.772 |
| walker |  | 2334 | 17 | package identity in samples/electron-esm-webpack/package.json |  |  | 0.772 |
| walker |  | 2361 | 27 | listing of 'samples/browser-esm-webpack-typescript-react' |  |  | 0.772 |
| walker |  | 2381 | 20 | package identity in samples/browser-esm-webpack-monaco-plugin/package.json |  |  | 0.772 |
| walker |  | 2401 | 20 | package identity in samples/browser-esm-webpack-typescript/package.json |  |  | 0.772 |
| walker |  | 2421 | 20 | package identity in samples/browser-esm-webpack-typescript-react/package.json |  |  | 0.772 |
| ns | 2431 |  | 282 | _.contribution.ts: ILang/ILangImpl and registerLanguage() | 2.8 |  | 0.735 |
| walker |  | 2437 | 16 | listing of 'samples/browser-esm-webpack-typescript-react/src' |  |  | 0.735 |
| walker |  | 2442 | 5 | listing of 'samples/browser-esm-webpack-typescript-react/src/components' |  |  | 0.735 |
| walker |  | 2473 | 31 | README headline in samples/README.md |  |  | 0.735 |
| walker |  | 2504 | 31 | listing of 'samples/browser-esm-vite-react' |  |  | 0.735 |
| walker |  | 2523 | 19 | package identity in samples/browser-esm-vite-react/package.json |  |  | 0.735 |
| walker |  | 2554 | 31 | listing of 'samples/browser-esm-webpack-small' |  |  | 0.735 |
| ns | 2570 |  | 139 | _.contribution.ts: LazyLanguageLoader and loadLanguage() | 2.9 |  | 0.719 |
| walker |  | 2572 | 18 | package identity in samples/browser-esm-webpack-small/package.json |  |  | 0.719 |
| walker |  | 2591 | 19 | listing of 'samples/browser-esm-vite-react/src' |  |  | 0.719 |
| walker |  | 2601 | 10 | listing of 'samples/browser-esm-vite-react/src/components' |  |  | 0.719 |
| walker |  | 2637 | 36 | listing of 'samples/browser-esm-vite' |  |  | 0.719 |
| ns | 2659 |  | 89 | Grammar file shape: rust.ts conf + language exports | 2.10 |  | 0.709 |
| walker |  | 2684 | 47 | ts names samples/browser-esm-vite/main.ts |  |  | 0.709 |
| walker |  | 2697 | 13 | ts decl samples/browser-esm-vite/main.ts:18 |  |  | 0.709 |
| walker |  | 2715 | 18 | package identity in samples/browser-esm-vite/package.json |  |  | 0.709 |
| walker |  | 2747 | 32 | ts decl samples/browser-esm-vite/main.ts:21 |  |  | 0.709 |
| ns | 2799 |  | 140 | Tokenization test harness: testRunner.ts types + testTokenization(), and a test file head | 2.11 |  | 0.693 |
| walker |  | 2809 | 62 | package identity in webpack-plugin/package.json |  |  | 0.693 |
| ns | 2841 |  | 42 | src/languages/definitions/register.all.ts head, elided middle, tail | 2.12 |  | 0.688 |
| walker |  | 2880 | 71 | package identity in samples/package.json |  |  | 0.688 |
| walker |  | 2892 | 12 | package entrypoints in samples/package.json |  |  | 0.688 |
| walker |  | 2939 | 47 | ts decl monaco-lsp-client/generator/index.ts:233 |  |  | 0.688 |
| ns | 2976 |  | 135 | src/deprecated/: the legacy import paths, all one-line re-exports | 2.13 |  | 0.675 |
| walker |  | 2994 | 55 | README headline in monaco-lsp-client/README.md |  |  | 0.675 |
| walker |  | 3050 | 56 | README headline in webpack-plugin/README.md |  |  | 0.675 |
| walker |  | 3060 | 10 | package identity metadata in monaco-lsp-client/package.json |  |  | 0.675 |
| ns | 3345 |  | 369 | package.json scripts — build, test and dev commands (packaging variants elided) | 3.1 |  | 0.655 |
| walker |  | 3427 | 367 | ts names webpack-plugin/src/index.ts |  |  | 0.656 |
| walker |  | 3471 | 44 | ts decl webpack-plugin/src/index.ts:159 |  |  | 0.656 |
| ns | 3477 |  | 132 | CONTRIBUTING: the ordered editor-test command sequence | 3.2 |  | 0.645 |
| walker |  | 3496 | 25 | ts decl webpack-plugin/src/index.ts:53 |  |  | 0.645 |
| walker |  | 3523 | 27 | ts decl webpack-plugin/src/index.ts:207 |  |  | 0.645 |
| ns | 3541 |  | 64 | test/ listing plus the full smoke-test directory | 3.3 |  | 0.632 |
| walker |  | 3560 | 37 | ts decl webpack-plugin/src/index.ts:43 |  |  | 0.632 |
| walker |  | 3600 | 40 | ts decl webpack-plugin/src/index.ts:63 |  |  | 0.632 |
| walker |  | 3648 | 48 | ts decl webpack-plugin/src/index.ts:90 |  |  | 0.632 |
| walker |  | 3668 | 20 | listing of '.github' |  |  | 0.632 |
| walker |  | 3676 | 8 | listing of '.github/workflows' |  |  | 0.632 |
| ns | 3692 |  | 151 | Listings of the five src/languages/features dirs — the uniform rich-service file set | 4.1 |  | 0.633 |
| walker |  | 3731 | 55 | ts decl monaco-lsp-client/generator/index.ts:159 |  |  | 0.633 |
| walker |  | 3787 | 56 | ts decl webpack-plugin/src/index.ts:334 |  |  | 0.633 |
| walker |  | 3844 | 57 | ts decl monaco-lsp-client/generator/index.ts:145 |  |  | 0.633 |
| walker |  | 3904 | 60 | ts decl monaco-lsp-client/generator/index.ts:73 |  |  | 0.633 |
| walker |  | 3941 | 37 | listing of 'monaco-lsp-client/src/adapters' |  |  | 0.634 |
| ns | 3952 |  | 260 | css/register.ts — every top-level export (names only) | 4.2 |  | 0.618 |
| walker |  | 4115 | 174 | listing of 'monaco-lsp-client/src/adapters/languageFeatures' |  |  | 0.618 |
| walker |  | 4178 | 63 | ts decl webpack-plugin/src/index.ts:12 |  |  | 0.618 |
| walker |  | 4214 | 36 | headings outline in MAINTAINING.md |  |  | 0.618 |
| walker |  | 4226 | 12 | package identity metadata in samples/package.json |  |  | 0.618 |
| ns | 4237 |  | 285 | css: the default ModeConfiguration toggles, the three defaults objects, and the lazy onLanguage hookup | 4.3 | 4.2 | 0.594 |
| walker |  | 4290 | 64 | ts decl monaco-lsp-client/generator/index.ts:124 |  |  | 0.594 |
| walker |  | 4301 | 11 | package entrypoints in samples/electron-esm-webpack/package.json |  |  | 0.594 |
| walker |  | 4366 | 65 | ts decl webpack-plugin/src/index.ts:150 |  |  | 0.594 |
| walker |  | 4448 | 82 | listing of 'samples/legacy' |  |  | 0.594 |
| walker |  | 4452 | 4 | listing of 'samples/legacy/browser-amd-editor' |  |  | 0.594 |
| walker |  | 4456 | 4 | listing of 'samples/legacy/browser-amd-localized' |  |  | 0.594 |
| walker |  | 4460 | 4 | listing of 'samples/legacy/browser-amd-monarch' |  |  | 0.594 |
| walker |  | 4464 | 4 | listing of 'samples/legacy/browser-amd-requirejs' |  |  | 0.594 |
| walker |  | 4468 | 4 | listing of 'samples/legacy/browser-amd-shadow-dom' |  |  | 0.594 |
| walker |  | 4472 | 4 | listing of 'samples/legacy/browser-amd-shared-model' |  |  | 0.594 |
| walker |  | 4476 | 4 | listing of 'samples/legacy/browser-amd-trusted-types' |  |  | 0.594 |
| walker |  | 4484 | 8 | listing of 'samples/legacy/browser-amd-iframe' |  |  | 0.594 |
| walker |  | 4496 | 12 | listing of 'samples/legacy/browser-amd-diff-editor' |  |  | 0.580 |
| ns | 4496 |  | 259 | json/register.ts — every top-level export (names only) | 4.4 |  | 0.580 |
| walker |  | 4513 | 17 | listing of 'samples/legacy/electron-amd' |  |  | 0.580 |
| walker |  | 4530 | 17 | listing of 'samples/legacy/electron-amd-nodeIntegration' |  |  | 0.580 |
| walker |  | 4546 | 16 | package identity in samples/legacy/electron-amd/package.json |  |  | 0.580 |
| walker |  | 4565 | 19 | package identity in samples/legacy/electron-amd-nodeIntegration/package.json |  |  | 0.580 |
| walker |  | 4639 | 74 | ts decl monaco-lsp-client/generator/index.ts:113 |  |  | 0.580 |
| walker |  | 4677 | 38 | headings outline in samples/README.md |  |  | 0.580 |
| walker |  | 4686 | 9 | samples/README.md section #4 |  |  | 0.580 |
| walker |  | 4763 | 77 | ts decl monaco-lsp-client/generator/index.ts:134 |  |  | 0.580 |
| ns | 4765 |  | 269 | json defaults: diagnostic option values, mode toggles, jsonDefaults, the worker interface | 4.5 | 4.4 | 0.561 |
| walker |  | 4842 | 79 | ts decl monaco-lsp-client/generator/index.ts:7 |  |  | 0.561 |
| walker |  | 4869 | 27 | package entrypoints in monaco-lsp-client/package.json |  |  | 0.561 |
| walker |  | 4897 | 28 | package entrypoints in webpack-plugin/package.json |  |  | 0.562 |
| walker |  | 4984 | 87 | ts decl monaco-lsp-client/generator/index.ts:101 |  |  | 0.562 |
| ns | 5027 |  | 262 | html/register.ts — export names plus the registerHTMLLanguageService factory | 4.6 |  | 0.549 |
| walker |  | 5029 | 45 | headings outline in webpack-plugin/README.md |  |  | 0.549 |
| walker |  | 5127 | 98 | ts decl webpack-plugin/src/index.ts:238 |  |  | 0.549 |
| walker |  | 5228 | 101 | ts decl monaco-lsp-client/generator/index.ts:170 |  |  | 0.549 |
| ns | 5311 |  | 284 | html: the three pre-registered services and their per-language toggles | 4.7 | 4.6 | 0.535 |
| walker |  | 5331 | 103 | json config editor.code-workspace |  |  | 0.535 |
| walker |  | 5443 | 112 | ts decl monaco-lsp-client/generator/index.ts:183 |  |  | 0.535 |
| ns | 5556 |  | 245 | typescript/register.ts — every top-level export (names only) | 4.8 |  | 0.525 |
| walker |  | 5557 | 114 | ts decl monaco-lsp-client/generator/index.ts:86 |  |  | 0.525 |
| walker |  | 5590 | 33 | samples/legacy/README.md section #0 |  |  | 0.525 |
| walker |  | 5736 | 146 | headings outline in README.md |  |  | 0.552 |
| walker |  | 5780 | 44 | README.md section #0 |  |  | 0.552 |
| ns | 5791 |  | 235 | LanguageServiceDefaults (typescript) — all members | 4.9 |  | 0.542 |
| walker |  | 5840 | 60 | listing of 'website' |  |  | 0.543 |
| walker |  | 5877 | 37 | package scripts in samples/package.json |  |  | 0.543 |
| walker |  | 5890 | 13 | package entrypoints in samples/legacy/electron-amd/package.json |  |  | 0.543 |
| walker |  | 5903 | 13 | package entrypoints in samples/legacy/electron-amd-nodeIntegration/package.json |  |  | 0.543 |
| walker |  | 5931 | 28 | webpack-plugin/README.md section #1 |  |  | 0.543 |
| walker |  | 5966 | 35 | listing of 'src/languages/features/typescript/lib' |  |  | 0.556 |
| ns | 6027 |  | 236 | TypeScriptWorker — all 21 proxy methods (names only) | 4.10 | 4.8 | 0.545 |
| walker |  | 6049 | 83 | headings outline in docs/integrate-esm.md |  |  | 0.546 |
| walker |  | 6079 | 30 | README.md section #21 |  |  | 0.546 |
| walker |  | 6157 | 78 | package identity metadata in package.json |  |  | 0.552 |
| ns | 6318 |  | 291 | typescript/javascript defaults and their divergence | 4.11 | 4.8 | 0.539 |
| ns | 6547 |  | 229 | The four *Mode.ts entry points and the tsMode re-export tail | 5.1 |  | 0.534 |
| ns | 6761 |  | 214 | cssMode.setupMode — WorkerManager, the worker accessor, and ModeConfiguration-gated provider registration | 5.2 | 5.1 | 0.524 |
| walker |  | 6776 | 619 | ts decl webpack-plugin/src/index.ts:102 |  |  | 0.525 |
| walker |  | 6806 | 30 | package scripts in samples/browser-esm-esbuild/package.json |  |  | 0.525 |
| walker |  | 6852 | 46 | README.md section #4 |  |  | 0.525 |
| walker |  | 6883 | 31 | package scripts in samples/browser-esm-parcel/package.json |  |  | 0.525 |
| walker |  | 7049 | 166 | package entrypoints in package.json |  |  | 0.526 |
| walker |  | 7072 | 23 | listing of 'scripts/ci' |  |  | 0.526 |
| ns | 7114 |  | 353 | common/lspLanguageFeatures.ts — the shared provider adapters and LSP conversion helpers | 5.3 |  | 0.517 |
| walker |  | 7115 | 43 | README.md section #17 |  |  | 0.517 |
| walker |  | 7128 | 13 | ts names webpack-plugin/src/loader-utils.d.ts |  |  | 0.517 |
| walker |  | 7208 | 80 | README.md section #1 |  |  | 0.517 |
| walker |  | 7277 | 69 | package scripts in webpack-plugin/package.json |  |  | 0.517 |
| walker |  | 7303 | 26 | listing of 'website/src' |  |  | 0.518 |
| walker |  | 7315 | 12 | listing of 'website/src/runner' |  |  | 0.518 |
| walker |  | 7353 | 38 | package scripts in samples/browser-esm-webpack/package.json |  |  | 0.518 |
| walker |  | 7391 | 38 | package scripts in samples/browser-esm-webpack-monaco-plugin/package.json |  |  | 0.518 |
| ns | 7421 |  | 307 | Web-worker plumbing: MonacoEnvironment hooks, createWebWorker, IWebWorkerOptions fields, worker entry points | 5.4 |  | 0.508 |
| walker |  | 7473 | 82 | package scripts in monaco-lsp-client/package.json |  |  | 0.508 |
| walker |  | 7478 | 5 | docs/integrate-esm.md section #6 |  |  | 0.508 |
| walker |  | 7483 | 5 | docs/integrate-esm.md section #11 |  |  | 0.508 |
| walker |  | 7488 | 5 | docs/integrate-esm.md section #19 |  |  | 0.508 |
| walker |  | 7498 | 10 | ts names src/deprecated/editor/editor.main.ts |  |  | 0.508 |
| walker |  | 7508 | 10 | ts names src/deprecated/editor/editor.worker.ts |  |  | 0.508 |
| walker |  | 7519 | 11 | package runtime dependencies in samples/browser-esm-vite/package.json |  |  | 0.508 |
| walker |  | 7530 | 11 | package runtime dependencies in samples/browser-esm-vite-react/package.json |  |  | 0.508 |
| walker |  | 7541 | 11 | package runtime dependencies in samples/browser-esm-webpack-typescript-react/package.json |  |  | 0.508 |
| ns | 7576 |  | 155 | WorkerManager lifecycle and the CSSWorker entry points | 5.5 |  | 0.504 |
| walker |  | 7595 | 54 | package scripts in samples/browser-esm-vite/package.json |  |  | 0.504 |
| walker |  | 7651 | 56 | package scripts in samples/browser-esm-webpack-small/package.json |  |  | 0.504 |
| walker |  | 7710 | 59 | README.md section #19 |  |  | 0.504 |
| walker |  | 7744 | 34 | package scripts in samples/legacy/electron-amd/package.json |  |  | 0.504 |
| walker |  | 7778 | 34 | package scripts in samples/legacy/electron-amd-nodeIntegration/package.json |  |  | 0.504 |
| ns | 7830 |  | 254 | typescript/languageFeatures.ts — the TypeScript-specific adapter roster | 5.6 |  | 0.498 |
| walker |  | 7888 | 110 | MAINTAINING.md section #0 |  |  | 0.498 |
| ns | 7909 |  | 79 | monaco-lsp-client package: complete tree listing | 6.1 |  | 0.507 |
| walker |  | 7947 | 59 | package scripts in samples/electron-esm-webpack/package.json |  |  | 0.507 |
| walker |  | 7950 | 3 | listing of 'website/index' |  |  | 0.507 |
| walker |  | 7964 | 14 | ts names monaco-lsp-client/src/adapters/LspConnection.ts |  |  | 0.507 |
| walker |  | 7976 | 12 | ts decl monaco-lsp-client/src/adapters/LspConnection.ts:6 |  |  | 0.507 |
| walker |  | 7990 | 14 | ts names src/deprecated/basic-languages/monaco.contribution.ts |  |  | 0.507 |
| walker |  | 8014 | 24 | ts names src/languages/register.all.ts |  |  | 0.509 |
| walker |  | 8038 | 24 | ts names webpack-plugin/src/types.ts |  |  | 0.509 |
| walker |  | 8057 | 19 | ts decl webpack-plugin/src/types.ts:1 |  |  | 0.509 |
| ns | 8116 |  | 207 | MonacoLspClient wiring + index exports | 6.2 | 6.1 | 0.504 |
| walker |  | 8120 | 63 | package scripts in samples/browser-esm-webpack-typescript/package.json |  |  | 0.504 |
| ns | 8165 |  | 49 | webpack-plugin package: complete tree listing | 6.3 |  | 0.510 |
| walker |  | 8171 | 51 | listing of 'test/smoke' |  |  | 0.525 |
| walker |  | 8237 | 66 | package scripts in samples/browser-esm-webpack-typescript-react/package.json |  |  | 0.525 |
| walker |  | 8252 | 15 | ts names monaco-lsp-client/src/adapters/LspClient.ts |  |  | 0.525 |
| walker |  | 8332 | 80 | listing of 'test/manual' |  |  | 0.525 |
| walker |  | 8349 | 17 | ts names src/languages/features/register.all.ts |  |  | 0.526 |
| walker |  | 8395 | 46 | listing of 'website/src/website' |  |  | 0.526 |
| walker |  | 8410 | 15 | listing of 'website/src/website/data' |  |  | 0.526 |
| walker |  | 8437 | 27 | listing of 'website/src/website/pages' |  |  | 0.526 |
| ns | 8449 |  | 284 | webpack-plugin option fields and peer-dependency contract | 6.4 | 6.3 | 0.531 |
| walker |  | 8471 | 34 | listing of 'website/src/website/data/playground-samples' |  |  | 0.531 |
| walker |  | 8490 | 19 | listing of 'website/src/website/data/playground-samples/customizing-the-appearence' |  |  | 0.531 |
| walker |  | 8511 | 21 | listing of 'website/src/website/data/playground-samples/creating-the-diffeditor' |  |  | 0.531 |
| walker |  | 8542 | 31 | listing of 'website/src/website/data/playground-samples/creating-the-editor' |  |  | 0.531 |
| walker |  | 8582 | 40 | listing of 'website/src/website/components' |  |  | 0.531 |
| ns | 8613 |  | 164 | build/ and scripts/ tree listings | 7.1 |  | 0.521 |
| walker |  | 8624 | 42 | listing of 'website/src/website/utils' |  |  | 0.521 |
| walker |  | 8728 | 104 | README.md section #3 |  |  | 0.521 |
| ns | 8762 |  | 149 | build-monaco-editor.ts run() — what the published package is made of | 7.2 |  | 0.517 |
| walker |  | 8804 | 76 | README.md section #20 |  |  | 0.517 |
| walker |  | 8855 | 51 | MAINTAINING.md section #2 |  |  | 0.517 |
| walker |  | 8889 | 34 | package runtime dependencies in webpack-plugin/package.json |  |  | 0.517 |
| walker |  | 8956 | 67 | listing of 'website/src/website/pages/playground' |  |  | 0.517 |
| ns | 9000 |  | 238 | check-samples.ts — the invariant every new language must satisfy | 7.3 |  | 0.510 |
| walker |  | 9043 | 87 | package scripts in samples/browser-esm-vite-react/package.json |  |  | 0.510 |
| walker |  | 9079 | 36 | package runtime dependencies in samples/package.json |  |  | 0.510 |
| walker |  | 9154 | 75 | listing of 'website/src/website/data/playground-samples/extending-language-services' |  |  | 0.510 |
| ns | 9166 |  | 166 | package.json distribution fields: typings, main, module, exports | 8.1 |  | 0.516 |
| walker |  | 9167 | 13 | ts names src/deprecated/language/css/monaco.contribution.ts |  |  | 0.516 |
| walker |  | 9180 | 13 | ts names src/deprecated/language/html/monaco.contribution.ts |  |  | 0.516 |
| walker |  | 9193 | 13 | ts names src/deprecated/language/json/monaco.contribution.ts |  |  | 0.516 |
| walker |  | 9206 | 13 | ts names src/languages/features/css/workerManager.ts |  |  | 0.516 |
| walker |  | 9219 | 13 | ts names src/languages/features/html/workerManager.ts |  |  | 0.516 |
| walker |  | 9232 | 13 | ts names src/languages/features/json/workerManager.ts |  |  | 0.516 |
| walker |  | 9245 | 13 | ts names src/languages/features/typescript/workerManager.ts |  |  | 0.516 |
| walker |  | 9280 | 35 | ts names monaco-lsp-client/src/utils.ts |  |  | 0.516 |
| walker |  | 9292 | 12 | ts decl monaco-lsp-client/src/utils.ts:1 |  |  | 0.516 |
| walker |  | 9312 | 20 | ts decl monaco-lsp-client/src/adapters/LspClient.ts:30 |  |  | 0.516 |
| walker |  | 9347 | 35 | ts decl webpack-plugin/src/types.ts:6 |  |  | 0.516 |
| walker |  | 9361 | 14 | ts names src/deprecated/language/css/css.worker.ts |  |  | 0.516 |
| walker |  | 9375 | 14 | ts names src/deprecated/language/html/html.worker.ts |  |  | 0.516 |
| walker |  | 9389 | 14 | ts names src/deprecated/language/json/json.worker.ts |  |  | 0.516 |
| walker |  | 9403 | 14 | ts names src/deprecated/language/typescript/monaco.contribution.ts |  |  | 0.516 |
| ns | 9420 |  | 254 | README Installing + CHANGELOG head (0.55.x breaking changes) | 8.2 |  | 0.511 |
| walker |  | 9425 | 22 | ts names monaco-lsp-client/src/adapters/TextDocumentSynchronizer.ts |  |  | 0.511 |
| walker |  | 9499 | 74 | listing of 'website/src/website/data/playground-samples/interacting-with-the-editor' |  |  | 0.511 |
| walker |  | 9511 | 12 | ts body webpack-plugin/src/index.ts:371 |  |  | 0.511 |
| ns | 9524 |  | 104 | samples/ listing — every integration sample directory | 8.3 |  | 0.518 |
| ns | 9562 |  | 38 | docs/ listing | 8.4 |  | 0.521 |
| ns | 9645 |  | 83 | integrate-esm.md section headings | 8.5 | 8.4 | 0.523 |
| walker |  | 9663 | 152 | README.md section #2 |  |  | 0.525 |
| walker |  | 9763 | 100 | samples/README.md section #1 |  |  | 0.525 |
| ns | 9771 |  | 126 | website/ and CI/publishing config listings | 8.6 |  | 0.536 |
| walker |  | 9800 | 37 | README.md section #15 |  |  | 0.536 |
| walker |  | 9837 | 37 | README.md section #16 |  |  | 0.536 |
| walker |  | 9853 | 16 | ts names src/deprecated/language/typescript/ts.worker.ts |  |  | 0.536 |
| walker |  | 9870 | 17 | ts names monaco-lsp-client/src/adapters/languageFeatures/LspCompletionFeature.ts |  |  | 0.536 |
| walker |  | 9884 | 14 | ts decl monaco-lsp-client/src/adapters/languageFeatures/LspCompletionFeature.ts:17 |  |  | 0.536 |
| walker |  | 9901 | 17 | ts names monaco-lsp-client/src/adapters/languageFeatures/LspDeclarationFeature.ts |  |  | 0.536 |
| walker |  | 9915 | 14 | ts decl monaco-lsp-client/src/adapters/languageFeatures/LspDeclarationFeature.ts:8 |  |  | 0.536 |
| walker |  | 9932 | 17 | ts names monaco-lsp-client/src/adapters/languageFeatures/LspDefinitionFeature.ts |  |  | 0.536 |
| walker |  | 9946 | 14 | ts decl monaco-lsp-client/src/adapters/languageFeatures/LspDefinitionFeature.ts:8 |  |  | 0.536 |
| walker |  | 9963 | 17 | ts names monaco-lsp-client/src/adapters/languageFeatures/LspDiagnosticsFeature.ts |  |  | 0.536 |
| walker |  | 9978 | 15 | ts decl monaco-lsp-client/src/adapters/languageFeatures/LspDiagnosticsFeature.ts:7 |  |  | 0.536 |
| walker |  | 9995 | 17 | ts names monaco-lsp-client/src/adapters/languageFeatures/LspFormattingFeature.ts |  |  | 0.536 |

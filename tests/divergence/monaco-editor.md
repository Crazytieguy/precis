Score(3000)=0.680 I=0.775 C=0.597 ns_rows≤3K=23/56 grid(1000/1442/2080/3000/4327/6240/9000)=0.643/0.711/0.789/0.680/0.601/0.515/0.504

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
| walker |  | 566 | 7 | listing of 'src/languages/features/common' |  |  | 0.612 |
| ns | 588 |  | 117 | The three one-to-six-line re-export files src/index.ts depends on | 1.6 | 1.2 | 0.561 |
| walker |  | 633 | 67 | README headline in README.md |  |  | 0.722 |
| ns | 734 |  | 146 | All README section headings (locations) | 1.7 |  | 0.640 |
| walker |  | 905 | 272 | listing of 'src/features' |  |  | 0.673 |
| walker |  | 925 | 20 | ts names src/editor.ts |  |  | 0.675 |
| ns | 927 |  | 193 | CONTRIBUTING 'Source Code Structure' — the monaco-editor-core boundary | 1.8 |  | 0.643 |
| ns | 1097 |  | 170 | README Concepts: Models and URIs | 1.9 | 1.7 | 0.635 |
| walker |  | 1229 | 304 | listing of 'src/languages/definitions' |  |  | 0.660 |
| ns | 1259 |  | 162 | README Concepts: Editors, Providers, Disposables | 1.10 | 1.7 | 0.647 |
| ns | 1326 |  | 67 | Listings of every src/ subtree except the two huge uniform ones | 2.1 |  | 0.701 |
| walker |  | 1335 | 106 | package identity in package.json |  |  | 0.729 |
| walker |  | 1383 | 48 | package identity in monaco-lsp-client/package.json |  |  | 0.729 |
| walker |  | 1396 | 13 | ts names webpack-plugin/src/loader-utils.d.ts |  |  | 0.729 |
| walker |  | 1408 | 12 | listing of '.azure-pipelines' |  |  | 0.729 |
| walker |  | 1420 | 12 | listing of '.vscode' |  |  | 0.729 |
| ns | 1432 |  | 106 | Uniform shape of a feature shim (hover, gpu) and the one that is not uniform (find) | 2.2 |  | 0.710 |
| walker |  | 1433 | 13 | listing of 'test' |  |  | 0.711 |
| ns | 1704 |  | 272 | Complete listing of src/features/ — all 65 editor-feature shim directories | 2.3 |  | 0.779 |
| walker |  | 1713 | 280 | ts names monaco-lsp-client/generator/index.ts |  |  | 0.779 |
| walker |  | 1724 | 11 | ts decl monaco-lsp-client/generator/index.ts:82 |  |  | 0.779 |
| walker |  | 1742 | 18 | ts decl monaco-lsp-client/generator/index.ts:259 |  |  | 0.779 |
| ns | 1748 |  | 44 | src/features/register.all.ts head, elided middle, tail | 2.4 |  | 0.767 |
| walker |  | 1763 | 21 | ts decl monaco-lsp-client/generator/index.ts:197 |  |  | 0.767 |
| walker |  | 1784 | 21 | ts decl monaco-lsp-client/generator/index.ts:202 |  |  | 0.767 |
| walker |  | 1805 | 21 | ts decl monaco-lsp-client/generator/index.ts:213 |  |  | 0.767 |
| walker |  | 1826 | 21 | ts decl monaco-lsp-client/generator/index.ts:218 |  |  | 0.767 |
| walker |  | 1847 | 21 | ts decl monaco-lsp-client/generator/index.ts:223 |  |  | 0.767 |
| walker |  | 1869 | 22 | ts decl monaco-lsp-client/generator/index.ts:228 |  |  | 0.767 |
| walker |  | 1891 | 22 | ts decl monaco-lsp-client/generator/index.ts:241 |  |  | 0.767 |
| walker |  | 1913 | 22 | ts decl monaco-lsp-client/generator/index.ts:246 |  |  | 0.767 |
| walker |  | 1937 | 24 | ts decl monaco-lsp-client/generator/index.ts:251 |  |  | 0.767 |
| walker |  | 1966 | 29 | ts decl monaco-lsp-client/generator/index.ts:207 |  |  | 0.767 |
| walker |  | 1997 | 31 | ts decl monaco-lsp-client/generator/index.ts:154 |  |  | 0.767 |
| walker |  | 2009 | 12 | listing of 'src/deprecated/language/css' |  |  | 0.767 |
| walker |  | 2021 | 12 | listing of 'src/deprecated/language/html' |  |  | 0.767 |
| walker |  | 2033 | 12 | listing of 'src/deprecated/language/json' |  |  | 0.767 |
| walker |  | 2045 | 12 | listing of 'src/deprecated/language/typescript' |  |  | 0.767 |
| walker |  | 2049 | 4 | listing of 'scripts/lib' |  |  | 0.767 |
| ns | 2052 |  | 304 | Complete listing of src/languages/definitions/ — all 84 Monarch languages plus _.contribution.ts, register.all.ts and test/ | 2.5 |  | 0.794 |
| ns | 2065 |  | 13 | One language definition directory, listed in full (rust) | 2.6 |  | 0.789 |
| ns | 2149 |  | 84 | rust/register.ts registerLanguage call | 2.7 | 2.6 | 0.776 |
| walker |  | 2153 | 104 | listing of 'samples' |  |  | 0.778 |
| walker |  | 2157 | 4 | listing of 'samples/nwjs-amd' |  |  | 0.778 |
| walker |  | 2161 | 4 | listing of 'samples/nwjs-amd-v2' |  |  | 0.778 |
| walker |  | 2182 | 21 | listing of 'samples/browser-esm-parcel' |  |  | 0.778 |
| walker |  | 2190 | 8 | listing of 'samples/browser-esm-parcel/src' |  |  | 0.778 |
| walker |  | 2205 | 15 | package identity in samples/browser-esm-parcel/package.json |  |  | 0.778 |
| walker |  | 2227 | 22 | listing of 'samples/browser-esm-webpack-typescript' |  |  | 0.778 |
| walker |  | 2235 | 8 | listing of 'samples/browser-esm-webpack-typescript/src' |  |  | 0.778 |
| walker |  | 2259 | 24 | listing of 'samples/browser-esm-esbuild' |  |  | 0.778 |
| walker |  | 2274 | 15 | package identity in samples/browser-esm-esbuild/package.json |  |  | 0.778 |
| walker |  | 2299 | 25 | listing of 'samples/browser-esm-webpack' |  |  | 0.778 |
| walker |  | 2316 | 17 | package identity in samples/browser-esm-webpack/package.json |  |  | 0.778 |
| walker |  | 2341 | 25 | listing of 'samples/browser-esm-webpack-monaco-plugin' |  |  | 0.778 |
| walker |  | 2366 | 25 | listing of 'samples/electron-esm-webpack' |  |  | 0.778 |
| walker |  | 2383 | 17 | package identity in samples/electron-esm-webpack/package.json |  |  | 0.778 |
| walker |  | 2410 | 27 | listing of 'samples/browser-esm-webpack-typescript-react' |  |  | 0.778 |
| walker |  | 2430 | 20 | package identity in samples/browser-esm-webpack-monaco-plugin/package.json |  |  | 0.778 |
| ns | 2431 |  | 282 | _.contribution.ts: ILang/ILangImpl and registerLanguage() | 2.8 |  | 0.740 |
| walker |  | 2450 | 20 | package identity in samples/browser-esm-webpack-typescript/package.json |  |  | 0.740 |
| walker |  | 2470 | 20 | package identity in samples/browser-esm-webpack-typescript-react/package.json |  |  | 0.740 |
| walker |  | 2486 | 16 | listing of 'samples/browser-esm-webpack-typescript-react/src' |  |  | 0.740 |
| walker |  | 2491 | 5 | listing of 'samples/browser-esm-webpack-typescript-react/src/components' |  |  | 0.740 |
| walker |  | 2522 | 31 | README headline in samples/README.md |  |  | 0.740 |
| walker |  | 2553 | 31 | listing of 'samples/browser-esm-vite-react' |  |  | 0.740 |
| ns | 2570 |  | 139 | _.contribution.ts: LazyLanguageLoader and loadLanguage() | 2.9 |  | 0.724 |
| walker |  | 2572 | 19 | package identity in samples/browser-esm-vite-react/package.json |  |  | 0.724 |
| walker |  | 2603 | 31 | listing of 'samples/browser-esm-webpack-small' |  |  | 0.724 |
| walker |  | 2621 | 18 | package identity in samples/browser-esm-webpack-small/package.json |  |  | 0.724 |
| walker |  | 2640 | 19 | listing of 'samples/browser-esm-vite-react/src' |  |  | 0.724 |
| walker |  | 2650 | 10 | listing of 'samples/browser-esm-vite-react/src/components' |  |  | 0.724 |
| ns | 2659 |  | 89 | Grammar file shape: rust.ts conf + language exports | 2.10 |  | 0.714 |
| walker |  | 2686 | 36 | listing of 'samples/browser-esm-vite' |  |  | 0.714 |
| walker |  | 2733 | 47 | ts names samples/browser-esm-vite/main.ts |  |  | 0.714 |
| walker |  | 2746 | 13 | ts decl samples/browser-esm-vite/main.ts:18 |  |  | 0.714 |
| walker |  | 2764 | 18 | package identity in samples/browser-esm-vite/package.json |  |  | 0.714 |
| walker |  | 2796 | 32 | ts decl samples/browser-esm-vite/main.ts:21 |  |  | 0.714 |
| ns | 2799 |  | 140 | Tokenization test harness: testRunner.ts types + testTokenization(), and a test file head | 2.11 |  | 0.698 |
| ns | 2841 |  | 42 | src/languages/definitions/register.all.ts head, elided middle, tail | 2.12 |  | 0.692 |
| walker |  | 2858 | 62 | package identity in webpack-plugin/package.json |  |  | 0.692 |
| walker |  | 2868 | 10 | ts names src/deprecated/editor/editor.main.ts |  |  | 0.692 |
| walker |  | 2878 | 10 | ts names src/deprecated/editor/editor.worker.ts |  |  | 0.692 |
| walker |  | 2949 | 71 | package identity in samples/package.json |  |  | 0.692 |
| walker |  | 2961 | 12 | package entrypoints in samples/package.json |  |  | 0.692 |
| ns | 2976 |  | 135 | src/deprecated/: the legacy import paths, all one-line re-exports | 2.13 |  | 0.680 |
| walker |  | 3008 | 47 | ts decl monaco-lsp-client/generator/index.ts:233 |  |  | 0.680 |
| walker |  | 3063 | 55 | README headline in monaco-lsp-client/README.md |  |  | 0.680 |
| walker |  | 3119 | 56 | README headline in webpack-plugin/README.md |  |  | 0.680 |
| walker |  | 3129 | 10 | package identity metadata in monaco-lsp-client/package.json |  |  | 0.680 |
| ns | 3345 |  | 369 | package.json scripts — build, test and dev commands (packaging variants elided) | 3.1 |  | 0.660 |
| ns | 3477 |  | 132 | CONTRIBUTING: the ordered editor-test command sequence | 3.2 |  | 0.649 |
| walker |  | 3496 | 367 | ts names webpack-plugin/src/index.ts |  |  | 0.649 |
| walker |  | 3540 | 44 | ts decl webpack-plugin/src/index.ts:159 |  |  | 0.649 |
| ns | 3541 |  | 64 | test/ listing plus the full smoke-test directory | 3.3 |  | 0.636 |
| walker |  | 3565 | 25 | ts decl webpack-plugin/src/index.ts:53 |  |  | 0.636 |
| walker |  | 3592 | 27 | ts decl webpack-plugin/src/index.ts:207 |  |  | 0.636 |
| walker |  | 3629 | 37 | ts decl webpack-plugin/src/index.ts:43 |  |  | 0.636 |
| walker |  | 3669 | 40 | ts decl webpack-plugin/src/index.ts:63 |  |  | 0.636 |
| ns | 3692 |  | 151 | Listings of the five src/languages/features dirs — the uniform rich-service file set | 4.1 |  | 0.637 |
| walker |  | 3717 | 48 | ts decl webpack-plugin/src/index.ts:90 |  |  | 0.637 |
| walker |  | 3737 | 20 | listing of '.github' |  |  | 0.637 |
| walker |  | 3745 | 8 | listing of '.github/workflows' |  |  | 0.637 |
| walker |  | 3800 | 55 | ts decl monaco-lsp-client/generator/index.ts:159 |  |  | 0.637 |
| walker |  | 3856 | 56 | ts decl webpack-plugin/src/index.ts:334 |  |  | 0.637 |
| walker |  | 3913 | 57 | ts decl monaco-lsp-client/generator/index.ts:145 |  |  | 0.637 |
| walker |  | 3927 | 14 | ts names src/deprecated/basic-languages/monaco.contribution.ts |  |  | 0.638 |
| walker |  | 3951 | 24 | ts names src/languages/register.all.ts |  |  | 0.640 |
| ns | 3952 |  | 260 | css/register.ts — every top-level export (names only) | 4.2 |  | 0.625 |
| walker |  | 3975 | 24 | ts names webpack-plugin/src/types.ts |  |  | 0.625 |
| walker |  | 3994 | 19 | ts decl webpack-plugin/src/types.ts:1 |  |  | 0.625 |
| walker |  | 4054 | 60 | ts decl monaco-lsp-client/generator/index.ts:73 |  |  | 0.625 |
| walker |  | 4091 | 37 | listing of 'monaco-lsp-client/src/adapters' |  |  | 0.626 |
| ns | 4237 |  | 285 | css: the default ModeConfiguration toggles, the three defaults objects, and the lazy onLanguage hookup | 4.3 | 4.2 | 0.601 |
| walker |  | 4265 | 174 | listing of 'monaco-lsp-client/src/adapters/languageFeatures' |  |  | 0.601 |
| walker |  | 4279 | 14 | ts names monaco-lsp-client/src/adapters/LspConnection.ts |  |  | 0.601 |
| walker |  | 4291 | 12 | ts decl monaco-lsp-client/src/adapters/LspConnection.ts:6 |  |  | 0.601 |
| walker |  | 4306 | 15 | ts names monaco-lsp-client/src/adapters/LspClient.ts |  |  | 0.601 |
| walker |  | 4369 | 63 | ts decl webpack-plugin/src/index.ts:12 |  |  | 0.601 |
| walker |  | 4405 | 36 | headings outline in MAINTAINING.md |  |  | 0.601 |
| walker |  | 4444 | 39 | package scripts in samples/package.json |  |  | 0.601 |
| walker |  | 4454 | 10 | package identity metadata in samples/package.json |  |  | 0.601 |
| ns | 4496 |  | 259 | json/register.ts — every top-level export (names only) | 4.4 |  | 0.586 |
| walker |  | 4518 | 64 | ts decl monaco-lsp-client/generator/index.ts:124 |  |  | 0.586 |
| walker |  | 4529 | 11 | package entrypoints in samples/electron-esm-webpack/package.json |  |  | 0.586 |
| walker |  | 4594 | 65 | ts decl webpack-plugin/src/index.ts:150 |  |  | 0.586 |
| walker |  | 4611 | 17 | ts names src/languages/features/register.all.ts |  |  | 0.588 |
| walker |  | 4693 | 82 | listing of 'samples/legacy' |  |  | 0.588 |
| walker |  | 4697 | 4 | listing of 'samples/legacy/browser-amd-editor' |  |  | 0.588 |
| walker |  | 4701 | 4 | listing of 'samples/legacy/browser-amd-localized' |  |  | 0.588 |
| walker |  | 4705 | 4 | listing of 'samples/legacy/browser-amd-monarch' |  |  | 0.588 |
| walker |  | 4709 | 4 | listing of 'samples/legacy/browser-amd-requirejs' |  |  | 0.588 |
| walker |  | 4713 | 4 | listing of 'samples/legacy/browser-amd-shadow-dom' |  |  | 0.588 |
| walker |  | 4717 | 4 | listing of 'samples/legacy/browser-amd-shared-model' |  |  | 0.588 |
| walker |  | 4721 | 4 | listing of 'samples/legacy/browser-amd-trusted-types' |  |  | 0.588 |
| walker |  | 4729 | 8 | listing of 'samples/legacy/browser-amd-iframe' |  |  | 0.588 |
| walker |  | 4741 | 12 | listing of 'samples/legacy/browser-amd-diff-editor' |  |  | 0.588 |
| walker |  | 4758 | 17 | listing of 'samples/legacy/electron-amd' |  |  | 0.588 |
| ns | 4765 |  | 269 | json defaults: diagnostic option values, mode toggles, jsonDefaults, the worker interface | 4.5 | 4.4 | 0.570 |
| walker |  | 4775 | 17 | listing of 'samples/legacy/electron-amd-nodeIntegration' |  |  | 0.570 |
| walker |  | 4791 | 16 | package identity in samples/legacy/electron-amd/package.json |  |  | 0.570 |
| walker |  | 4810 | 19 | package identity in samples/legacy/electron-amd-nodeIntegration/package.json |  |  | 0.570 |
| walker |  | 4884 | 74 | ts decl monaco-lsp-client/generator/index.ts:113 |  |  | 0.570 |
| walker |  | 4922 | 38 | headings outline in samples/README.md |  |  | 0.570 |
| walker |  | 4931 | 9 | samples/README.md section #4 |  |  | 0.570 |
| walker |  | 5008 | 77 | ts decl monaco-lsp-client/generator/index.ts:134 |  |  | 0.570 |
| ns | 5027 |  | 262 | html/register.ts — export names plus the registerHTMLLanguageService factory | 4.6 |  | 0.557 |
| walker |  | 5087 | 79 | ts decl monaco-lsp-client/generator/index.ts:7 |  |  | 0.557 |
| walker |  | 5114 | 27 | package entrypoints in monaco-lsp-client/package.json |  |  | 0.557 |
| walker |  | 5127 | 13 | ts names src/deprecated/language/css/monaco.contribution.ts |  |  | 0.557 |
| walker |  | 5140 | 13 | ts names src/deprecated/language/html/monaco.contribution.ts |  |  | 0.557 |
| walker |  | 5153 | 13 | ts names src/deprecated/language/json/monaco.contribution.ts |  |  | 0.557 |
| walker |  | 5166 | 13 | ts names src/languages/features/css/workerManager.ts |  |  | 0.557 |
| walker |  | 5179 | 13 | ts names src/languages/features/html/workerManager.ts |  |  | 0.557 |
| walker |  | 5192 | 13 | ts names src/languages/features/json/workerManager.ts |  |  | 0.557 |
| walker |  | 5205 | 13 | ts names src/languages/features/typescript/workerManager.ts |  |  | 0.557 |
| walker |  | 5240 | 35 | ts names monaco-lsp-client/src/utils.ts |  |  | 0.557 |
| walker |  | 5252 | 12 | ts decl monaco-lsp-client/src/utils.ts:1 |  |  | 0.557 |
| walker |  | 5280 | 28 | package entrypoints in webpack-plugin/package.json |  |  | 0.558 |
| walker |  | 5300 | 20 | ts decl monaco-lsp-client/src/adapters/LspClient.ts:30 |  |  | 0.558 |
| ns | 5311 |  | 284 | html: the three pre-registered services and their per-language toggles | 4.7 | 4.6 | 0.544 |
| walker |  | 5387 | 87 | ts decl monaco-lsp-client/generator/index.ts:101 |  |  | 0.544 |
| walker |  | 5422 | 35 | ts decl webpack-plugin/src/types.ts:6 |  |  | 0.544 |
| walker |  | 5467 | 45 | headings outline in webpack-plugin/README.md |  |  | 0.544 |
| walker |  | 5481 | 14 | ts names src/deprecated/language/css/css.worker.ts |  |  | 0.544 |
| walker |  | 5495 | 14 | ts names src/deprecated/language/html/html.worker.ts |  |  | 0.544 |
| walker |  | 5509 | 14 | ts names src/deprecated/language/json/json.worker.ts |  |  | 0.544 |
| walker |  | 5523 | 14 | ts names src/deprecated/language/typescript/monaco.contribution.ts |  |  | 0.544 |
| walker |  | 5545 | 22 | ts names monaco-lsp-client/src/adapters/TextDocumentSynchronizer.ts |  |  | 0.544 |
| ns | 5556 |  | 245 | typescript/register.ts — every top-level export (names only) | 4.8 |  | 0.533 |
| walker |  | 5643 | 98 | ts decl webpack-plugin/src/index.ts:238 |  |  | 0.533 |
| walker |  | 5744 | 101 | ts decl monaco-lsp-client/generator/index.ts:170 |  |  | 0.533 |
| walker |  | 5760 | 16 | ts names src/deprecated/language/typescript/ts.worker.ts |  |  | 0.534 |
| ns | 5791 |  | 235 | LanguageServiceDefaults (typescript) — all members | 4.9 |  | 0.525 |
| walker |  | 5863 | 103 | json config editor.code-workspace |  |  | 0.525 |
| walker |  | 5880 | 17 | ts names monaco-lsp-client/src/adapters/languageFeatures/LspCompletionFeature.ts |  |  | 0.525 |
| walker |  | 5894 | 14 | ts decl monaco-lsp-client/src/adapters/languageFeatures/LspCompletionFeature.ts:17 |  |  | 0.525 |
| walker |  | 5911 | 17 | ts names monaco-lsp-client/src/adapters/languageFeatures/LspDeclarationFeature.ts |  |  | 0.525 |
| walker |  | 5925 | 14 | ts decl monaco-lsp-client/src/adapters/languageFeatures/LspDeclarationFeature.ts:8 |  |  | 0.525 |
| walker |  | 5942 | 17 | ts names monaco-lsp-client/src/adapters/languageFeatures/LspDefinitionFeature.ts |  |  | 0.525 |
| walker |  | 5956 | 14 | ts decl monaco-lsp-client/src/adapters/languageFeatures/LspDefinitionFeature.ts:8 |  |  | 0.525 |
| walker |  | 5973 | 17 | ts names monaco-lsp-client/src/adapters/languageFeatures/LspDiagnosticsFeature.ts |  |  | 0.525 |
| walker |  | 5988 | 15 | ts decl monaco-lsp-client/src/adapters/languageFeatures/LspDiagnosticsFeature.ts:7 |  |  | 0.525 |
| walker |  | 6005 | 17 | ts names monaco-lsp-client/src/adapters/languageFeatures/LspFormattingFeature.ts |  |  | 0.525 |
| walker |  | 6019 | 14 | ts decl monaco-lsp-client/src/adapters/languageFeatures/LspFormattingFeature.ts:7 |  |  | 0.525 |
| ns | 6027 |  | 236 | TypeScriptWorker — all 21 proxy methods (names only) | 4.10 | 4.8 | 0.515 |
| walker |  | 6036 | 17 | ts names monaco-lsp-client/src/adapters/languageFeatures/LspHoverFeature.ts |  |  | 0.515 |
| walker |  | 6050 | 14 | ts decl monaco-lsp-client/src/adapters/languageFeatures/LspHoverFeature.ts:7 |  |  | 0.515 |
| walker |  | 6067 | 17 | ts names monaco-lsp-client/src/adapters/languageFeatures/LspImplementationFeature.ts |  |  | 0.515 |
| walker |  | 6081 | 14 | ts decl monaco-lsp-client/src/adapters/languageFeatures/LspImplementationFeature.ts:8 |  |  | 0.515 |
| walker |  | 6098 | 17 | ts names monaco-lsp-client/src/adapters/languageFeatures/LspReferencesFeature.ts |  |  | 0.515 |
| walker |  | 6112 | 14 | ts decl monaco-lsp-client/src/adapters/languageFeatures/LspReferencesFeature.ts:7 |  |  | 0.515 |
| walker |  | 6129 | 17 | ts names monaco-lsp-client/src/adapters/languageFeatures/LspRenameFeature.ts |  |  | 0.515 |
| walker |  | 6143 | 14 | ts decl monaco-lsp-client/src/adapters/languageFeatures/LspRenameFeature.ts:7 |  |  | 0.515 |
| walker |  | 6255 | 112 | ts decl monaco-lsp-client/generator/index.ts:183 |  |  | 0.515 |
| ns | 6318 |  | 291 | typescript/javascript defaults and their divergence | 4.11 | 4.8 | 0.502 |
| walker |  | 6369 | 114 | ts decl monaco-lsp-client/generator/index.ts:86 |  |  | 0.502 |
| walker |  | 6387 | 18 | ts names monaco-lsp-client/src/adapters/languageFeatures/LspCodeActionFeature.ts |  |  | 0.502 |
| walker |  | 6401 | 14 | ts decl monaco-lsp-client/src/adapters/languageFeatures/LspCodeActionFeature.ts:8 |  |  | 0.502 |
| walker |  | 6419 | 18 | ts names monaco-lsp-client/src/adapters/languageFeatures/LspCodeLensFeature.ts |  |  | 0.502 |
| walker |  | 6433 | 14 | ts decl monaco-lsp-client/src/adapters/languageFeatures/LspCodeLensFeature.ts:9 |  |  | 0.502 |
| walker |  | 6451 | 18 | ts names monaco-lsp-client/src/adapters/languageFeatures/LspDocumentHighlightFeature.ts |  |  | 0.502 |
| walker |  | 6465 | 14 | ts decl monaco-lsp-client/src/adapters/languageFeatures/LspDocumentHighlightFeature.ts:8 |  |  | 0.502 |
| walker |  | 6483 | 18 | ts names monaco-lsp-client/src/adapters/languageFeatures/LspDocumentLinkFeature.ts |  |  | 0.502 |
| walker |  | 6497 | 14 | ts decl monaco-lsp-client/src/adapters/languageFeatures/LspDocumentLinkFeature.ts:7 |  |  | 0.502 |
| walker |  | 6515 | 18 | ts names monaco-lsp-client/src/adapters/languageFeatures/LspDocumentSymbolFeature.ts |  |  | 0.502 |
| walker |  | 6529 | 14 | ts decl monaco-lsp-client/src/adapters/languageFeatures/LspDocumentSymbolFeature.ts:8 |  |  | 0.502 |
| walker |  | 6547 | 18 | ts names monaco-lsp-client/src/adapters/languageFeatures/LspRangeFormattingFeature.ts |  |  | 0.498 |
| ns | 6547 |  | 229 | The four *Mode.ts entry points and the tsMode re-export tail | 5.1 |  | 0.498 |
| walker |  | 6561 | 14 | ts decl monaco-lsp-client/src/adapters/languageFeatures/LspRangeFormattingFeature.ts:7 |  |  | 0.498 |
| walker |  | 6579 | 18 | ts names monaco-lsp-client/src/adapters/languageFeatures/LspSelectionRangeFeature.ts |  |  | 0.498 |
| walker |  | 6593 | 14 | ts decl monaco-lsp-client/src/adapters/languageFeatures/LspSelectionRangeFeature.ts:7 |  |  | 0.498 |
| walker |  | 6611 | 18 | ts names monaco-lsp-client/src/adapters/languageFeatures/LspSemanticTokensFeature.ts |  |  | 0.498 |
| walker |  | 6625 | 14 | ts decl monaco-lsp-client/src/adapters/languageFeatures/LspSemanticTokensFeature.ts:7 |  |  | 0.498 |
| walker |  | 6643 | 18 | ts names monaco-lsp-client/src/adapters/languageFeatures/LspSignatureHelpFeature.ts |  |  | 0.498 |
| walker |  | 6657 | 14 | ts decl monaco-lsp-client/src/adapters/languageFeatures/LspSignatureHelpFeature.ts:8 |  |  | 0.498 |
| walker |  | 6675 | 18 | ts names monaco-lsp-client/src/adapters/languageFeatures/LspTypeDefinitionFeature.ts |  |  | 0.498 |
| walker |  | 6689 | 14 | ts decl monaco-lsp-client/src/adapters/languageFeatures/LspTypeDefinitionFeature.ts:8 |  |  | 0.498 |
| walker |  | 6707 | 18 | ts names samples/browser-esm-vite-react/src/components/Editor.tsx |  |  | 0.498 |
| walker |  | 6725 | 18 | ts names samples/browser-esm-webpack-typescript-react/src/components/Editor.tsx |  |  | 0.498 |
| walker |  | 6754 | 29 | ts names src/internal/common/workers.ts |  |  | 0.498 |
| ns | 6761 |  | 214 | cssMode.setupMode — WorkerManager, the worker accessor, and ModeConfiguration-gated provider registration | 5.2 | 5.1 | 0.489 |
| walker |  | 6777 | 23 | ts decl src/internal/common/workers.ts:92 |  |  | 0.489 |
| walker |  | 6796 | 19 | ts names monaco-lsp-client/src/adapters/languageFeatures/LspFoldingRangeFeature.ts |  |  | 0.489 |
| walker |  | 6810 | 14 | ts decl monaco-lsp-client/src/adapters/languageFeatures/LspFoldingRangeFeature.ts:8 |  |  | 0.489 |
| walker |  | 6829 | 19 | ts names monaco-lsp-client/src/adapters/languageFeatures/LspInlayHintsFeature.ts |  |  | 0.489 |
| walker |  | 6845 | 16 | ts decl monaco-lsp-client/src/adapters/languageFeatures/LspInlayHintsFeature.ts:9 |  |  | 0.489 |
| walker |  | 6864 | 19 | ts names monaco-lsp-client/src/adapters/languageFeatures/LspOnTypeFormattingFeature.ts |  |  | 0.489 |
| walker |  | 6878 | 14 | ts decl monaco-lsp-client/src/adapters/languageFeatures/LspOnTypeFormattingFeature.ts:7 |  |  | 0.489 |
| walker |  | 6911 | 33 | samples/legacy/README.md section #0 |  |  | 0.489 |
| walker |  | 7057 | 146 | headings outline in README.md |  |  | 0.514 |
| walker |  | 7101 | 44 | README.md section #0 |  |  | 0.514 |
| ns | 7114 |  | 353 | common/lspLanguageFeatures.ts — the shared provider adapters and LSP conversion helpers | 5.3 |  | 0.506 |
| walker |  | 7151 | 50 | ts decl webpack-plugin/src/loader-utils.d.ts:1 |  |  | 0.506 |
| walker |  | 7211 | 60 | listing of 'website' |  |  | 0.506 |
| walker |  | 7262 | 51 | ts decl monaco-lsp-client/src/utils.ts:5 |  |  | 0.506 |
| walker |  | 7275 | 13 | package entrypoints in samples/legacy/electron-amd/package.json |  |  | 0.506 |
| walker |  | 7288 | 13 | package entrypoints in samples/legacy/electron-amd-nodeIntegration/package.json |  |  | 0.506 |
| walker |  | 7316 | 28 | webpack-plugin/README.md section #1 |  |  | 0.506 |
| walker |  | 7338 | 22 | ts names src/languages/features/typescript/ts.worker.ts |  |  | 0.506 |
| walker |  | 7373 | 35 | ts names monaco-lsp-client/src/adapters/LspCapabilitiesRegistry.ts |  |  | 0.506 |
| walker |  | 7408 | 35 | listing of 'src/languages/features/typescript/lib' |  |  | 0.518 |
| ns | 7421 |  | 307 | Web-worker plumbing: MonacoEnvironment hooks, createWebWorker, IWebWorkerOptions fields, worker entry points | 5.4 |  | 0.509 |
| walker |  | 7445 | 37 | ts names webpack-plugin/src/loaders/include.ts |  |  | 0.509 |
| walker |  | 7459 | 14 | ts decl webpack-plugin/src/loaders/include.ts:10 |  |  | 0.509 |
| walker |  | 7493 | 34 | ts decl webpack-plugin/src/loaders/include.ts:4 |  |  | 0.509 |
| walker |  | 7531 | 38 | ts names webpack-plugin/src/plugins/AddWorkerEntryPointPlugin.ts |  |  | 0.509 |
| walker |  | 7559 | 28 | ts decl webpack-plugin/src/plugins/AddWorkerEntryPointPlugin.ts:40 |  |  | 0.509 |
| ns | 7576 |  | 155 | WorkerManager lifecycle and the CSSWorker entry points | 5.5 |  | 0.505 |
| walker |  | 7622 | 63 | ts decl monaco-lsp-client/src/utils.ts:24 |  |  | 0.505 |
| walker |  | 7661 | 39 | ts names src/languages/definitions/_.contribution.ts |  |  | 0.505 |
| walker |  | 7744 | 83 | headings outline in docs/integrate-esm.md |  |  | 0.506 |
| walker |  | 7785 | 41 | ts names monaco-lsp-client/src/adapters/ITextModelBridge.ts |  |  | 0.506 |
| walker |  | 7815 | 30 | ts decl monaco-lsp-client/src/adapters/ITextModelBridge.ts:32 |  |  | 0.506 |
| ns | 7830 |  | 254 | typescript/languageFeatures.ts — the TypeScript-specific adapter roster | 5.6 |  | 0.500 |
| walker |  | 7857 | 42 | ts names src/internal/common/initialize.ts |  |  | 0.501 |
| walker |  | 7887 | 30 | README.md section #21 |  |  | 0.501 |
| ns | 7909 |  | 79 | monaco-lsp-client package: complete tree listing | 6.1 |  | 0.510 |
| walker |  | 7965 | 78 | package identity metadata in package.json |  |  | 0.517 |
| walker |  | 7984 | 19 | ts names src/languages/features/typescript/lib/typescriptServicesMetadata.ts |  |  | 0.517 |
| ns | 8116 |  | 207 | MonacoLspClient wiring + index exports | 6.2 | 6.1 | 0.512 |
| ns | 8165 |  | 49 | webpack-plugin package: complete tree listing | 6.3 |  | 0.518 |
| ns | 8449 |  | 284 | webpack-plugin option fields and peer-dependency contract | 6.4 | 6.3 | 0.514 |
| walker |  | 8603 | 619 | ts decl webpack-plugin/src/index.ts:102 |  |  | 0.523 |
| walker |  | 8610 | 7 | ts body src/internal/common/initialize.ts:5 |  |  | 0.524 |
| ns | 8613 |  | 164 | build/ and scripts/ tree listings | 7.1 |  | 0.514 |
| walker |  | 8631 | 21 | ts names src/languages/features/typescript/lib/lib.index.ts |  |  | 0.514 |
| walker |  | 8652 | 21 | ts names src/languages/features/typescript/lib/lib.ts |  |  | 0.514 |
| walker |  | 8673 | 21 | ts names src/languages/features/typescript/lib/typescriptServices.d.ts |  |  | 0.514 |
| walker |  | 8703 | 30 | package scripts in samples/browser-esm-esbuild/package.json |  |  | 0.514 |
| walker |  | 8749 | 46 | README.md section #4 |  |  | 0.514 |
| ns | 8762 |  | 149 | build-monaco-editor.ts run() — what the published package is made of | 7.2 |  | 0.510 |
| walker |  | 8780 | 31 | package scripts in samples/browser-esm-parcel/package.json |  |  | 0.510 |
| walker |  | 8946 | 166 | package entrypoints in package.json |  |  | 0.511 |
| walker |  | 8969 | 23 | listing of 'scripts/ci' |  |  | 0.512 |
| ns | 9000 |  | 238 | check-samples.ts — the invariant every new language must satisfy | 7.3 |  | 0.504 |
| walker |  | 9012 | 43 | README.md section #17 |  |  | 0.504 |
| walker |  | 9064 | 52 | ts decl webpack-plugin/src/plugins/AddWorkerEntryPointPlugin.ts:3 |  |  | 0.504 |
| walker |  | 9144 | 80 | README.md section #1 |  |  | 0.504 |
| ns | 9166 |  | 166 | package.json distribution fields: typings, main, module, exports | 8.1 |  | 0.510 |
| walker |  | 9199 | 55 | ts decl monaco-lsp-client/src/adapters/LspCapabilitiesRegistry.ts:5 |  |  | 0.510 |
| walker |  | 9224 | 25 | ts names src/languages/features/typescript/lib/editor.worker.d.ts |  |  | 0.510 |
| walker |  | 9293 | 69 | package scripts in webpack-plugin/package.json |  |  | 0.510 |
| walker |  | 9319 | 26 | listing of 'website/src' |  |  | 0.511 |
| walker |  | 9331 | 12 | listing of 'website/src/runner' |  |  | 0.511 |
| walker |  | 9369 | 38 | package scripts in samples/browser-esm-webpack/package.json |  |  | 0.511 |
| walker |  | 9407 | 38 | package scripts in samples/browser-esm-webpack-monaco-plugin/package.json |  |  | 0.511 |
| ns | 9420 |  | 254 | README Installing + CHANGELOG head (0.55.x breaking changes) | 8.2 |  | 0.506 |
| walker |  | 9428 | 21 | ts decl monaco-lsp-client/src/adapters/languageFeatures/LspCodeActionFeature.ts:9 |  |  | 0.506 |
| walker |  | 9449 | 21 | ts decl monaco-lsp-client/src/adapters/languageFeatures/LspCodeLensFeature.ts:10 |  |  | 0.506 |
| walker |  | 9470 | 21 | ts decl monaco-lsp-client/src/adapters/languageFeatures/LspCompletionFeature.ts:18 |  |  | 0.506 |
| walker |  | 9491 | 21 | ts decl monaco-lsp-client/src/adapters/languageFeatures/LspDeclarationFeature.ts:9 |  |  | 0.506 |
| walker |  | 9512 | 21 | ts decl monaco-lsp-client/src/adapters/languageFeatures/LspDefinitionFeature.ts:9 |  |  | 0.506 |
| ns | 9524 |  | 104 | samples/ listing — every integration sample directory | 8.3 |  | 0.513 |
| walker |  | 9533 | 21 | ts decl monaco-lsp-client/src/adapters/languageFeatures/LspDiagnosticsFeature.ts:11 |  |  | 0.513 |
| walker |  | 9554 | 21 | ts decl monaco-lsp-client/src/adapters/languageFeatures/LspDocumentHighlightFeature.ts:9 |  |  | 0.513 |
| ns | 9562 |  | 38 | docs/ listing | 8.4 |  | 0.516 |
| walker |  | 9575 | 21 | ts decl monaco-lsp-client/src/adapters/languageFeatures/LspDocumentLinkFeature.ts:8 |  |  | 0.516 |
| walker |  | 9596 | 21 | ts decl monaco-lsp-client/src/adapters/languageFeatures/LspDocumentSymbolFeature.ts:9 |  |  | 0.516 |
| walker |  | 9617 | 21 | ts decl monaco-lsp-client/src/adapters/languageFeatures/LspFoldingRangeFeature.ts:9 |  |  | 0.516 |
| walker |  | 9638 | 21 | ts decl monaco-lsp-client/src/adapters/languageFeatures/LspFormattingFeature.ts:8 |  |  | 0.516 |
| ns | 9645 |  | 83 | integrate-esm.md section headings | 8.5 | 8.4 | 0.518 |
| walker |  | 9659 | 21 | ts decl monaco-lsp-client/src/adapters/languageFeatures/LspHoverFeature.ts:8 |  |  | 0.518 |
| walker |  | 9680 | 21 | ts decl monaco-lsp-client/src/adapters/languageFeatures/LspImplementationFeature.ts:9 |  |  | 0.518 |
| walker |  | 9701 | 21 | ts decl monaco-lsp-client/src/adapters/languageFeatures/LspInlayHintsFeature.ts:12 |  |  | 0.518 |
| walker |  | 9722 | 21 | ts decl monaco-lsp-client/src/adapters/languageFeatures/LspOnTypeFormattingFeature.ts:8 |  |  | 0.518 |
| walker |  | 9743 | 21 | ts decl monaco-lsp-client/src/adapters/languageFeatures/LspRangeFormattingFeature.ts:8 |  |  | 0.518 |
| walker |  | 9764 | 21 | ts decl monaco-lsp-client/src/adapters/languageFeatures/LspReferencesFeature.ts:8 |  |  | 0.518 |
| ns | 9771 |  | 126 | website/ and CI/publishing config listings | 8.6 |  | 0.529 |
| walker |  | 9785 | 21 | ts decl monaco-lsp-client/src/adapters/languageFeatures/LspRenameFeature.ts:8 |  |  | 0.529 |
| walker |  | 9806 | 21 | ts decl monaco-lsp-client/src/adapters/languageFeatures/LspSelectionRangeFeature.ts:8 |  |  | 0.529 |
| walker |  | 9827 | 21 | ts decl monaco-lsp-client/src/adapters/languageFeatures/LspSemanticTokensFeature.ts:8 |  |  | 0.529 |
| walker |  | 9848 | 21 | ts decl monaco-lsp-client/src/adapters/languageFeatures/LspSignatureHelpFeature.ts:9 |  |  | 0.529 |
| walker |  | 9869 | 21 | ts decl monaco-lsp-client/src/adapters/languageFeatures/LspTypeDefinitionFeature.ts:9 |  |  | 0.529 |
| walker |  | 9905 | 36 | package runtime dependencies in samples/package.json |  |  | 0.529 |
| walker |  | 9987 | 82 | package scripts in monaco-lsp-client/package.json |  |  | 0.529 |
| walker |  | 9992 | 5 | docs/integrate-esm.md section #6 |  |  | 0.529 |
| walker |  | 9997 | 5 | docs/integrate-esm.md section #11 |  |  | 0.529 |

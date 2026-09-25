Score(3000)=0.676 I=0.773 C=0.591 ns_rows≤3K=23/56 grid(1000/1442/2080/3000/4327/6240/9000)=0.662/0.698/0.784/0.676/0.594/0.553/0.507

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
| walker |  | 949 | 28 | Code::CodeKey { rung: Names, file: monaco-lsp-client/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.657 |
| walker |  | 979 | 30 | Code::CodeKey { rung: Names, file: src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.662 |
| ns | 1097 |  | 170 | README Concepts: Models and URIs | 1.9 | 1.7 | 0.653 |
| ns | 1259 |  | 162 | README Concepts: Editors, Providers, Disposables | 1.10 | 1.7 | 0.641 |
| walker |  | 1283 | 304 | Fs::DirListing { dir: src/languages/definitions } |  |  | 0.666 |
| ns | 1326 |  | 67 | Listings of every src/ subtree except the two huge uniform ones | 2.1 |  | 0.716 |
| walker |  | 1331 | 48 | Json::Identity { file: monaco-lsp-client/package.json } |  |  | 0.716 |
| walker |  | 1343 | 12 | Fs::DirListing { dir: .azure-pipelines } |  |  | 0.716 |
| walker |  | 1355 | 12 | Fs::DirListing { dir: .vscode } |  |  | 0.716 |
| walker |  | 1367 | 12 | Fs::DirListing { dir: src/deprecated/language/css } |  |  | 0.716 |
| walker |  | 1379 | 12 | Fs::DirListing { dir: src/deprecated/language/html } |  |  | 0.716 |
| walker |  | 1391 | 12 | Fs::DirListing { dir: src/deprecated/language/json } |  |  | 0.716 |
| walker |  | 1403 | 12 | Fs::DirListing { dir: src/deprecated/language/typescript } |  |  | 0.716 |
| walker |  | 1416 | 13 | Fs::DirListing { dir: test } |  |  | 0.716 |
| ns | 1432 |  | 106 | Uniform shape of a feature shim (hover, gpu) and the one that is not uniform (find) | 2.2 |  | 0.698 |
| walker |  | 1520 | 104 | Fs::DirListing { dir: samples } |  |  | 0.699 |
| walker |  | 1524 | 4 | Fs::DirListing { dir: samples/nwjs-amd } |  |  | 0.699 |
| walker |  | 1528 | 4 | Fs::DirListing { dir: samples/nwjs-amd-v2 } |  |  | 0.699 |
| walker |  | 1549 | 21 | Fs::DirListing { dir: samples/browser-esm-parcel } |  |  | 0.699 |
| walker |  | 1557 | 8 | Fs::DirListing { dir: samples/browser-esm-parcel/src } |  |  | 0.699 |
| walker |  | 1572 | 15 | Json::Identity { file: samples/browser-esm-parcel/package.json } |  |  | 0.699 |
| walker |  | 1594 | 22 | Fs::DirListing { dir: samples/browser-esm-webpack-typescript } |  |  | 0.699 |
| walker |  | 1602 | 8 | Fs::DirListing { dir: samples/browser-esm-webpack-typescript/src } |  |  | 0.699 |
| walker |  | 1626 | 24 | Fs::DirListing { dir: samples/browser-esm-esbuild } |  |  | 0.699 |
| walker |  | 1641 | 15 | Json::Identity { file: samples/browser-esm-esbuild/package.json } |  |  | 0.699 |
| walker |  | 1666 | 25 | Fs::DirListing { dir: samples/browser-esm-webpack } |  |  | 0.699 |
| walker |  | 1683 | 17 | Json::Identity { file: samples/browser-esm-webpack/package.json } |  |  | 0.699 |
| ns | 1704 |  | 272 | Complete listing of src/features/ — all 65 editor-feature shim directories | 2.3 |  | 0.772 |
| walker |  | 1708 | 25 | Fs::DirListing { dir: samples/browser-esm-webpack-monaco-plugin } |  |  | 0.772 |
| walker |  | 1733 | 25 | Fs::DirListing { dir: samples/electron-esm-webpack } |  |  | 0.772 |
| ns | 1748 |  | 44 | src/features/register.all.ts head, elided middle, tail | 2.4 |  | 0.760 |
| walker |  | 1750 | 17 | Json::Identity { file: samples/electron-esm-webpack/package.json } |  |  | 0.760 |
| walker |  | 1777 | 27 | Fs::DirListing { dir: samples/browser-esm-webpack-typescript-react } |  |  | 0.760 |
| walker |  | 1797 | 20 | Json::Identity { file: samples/browser-esm-webpack-monaco-plugin/package.json } |  |  | 0.760 |
| walker |  | 1817 | 20 | Json::Identity { file: samples/browser-esm-webpack-typescript/package.json } |  |  | 0.760 |
| walker |  | 1837 | 20 | Json::Identity { file: samples/browser-esm-webpack-typescript-react/package.json } |  |  | 0.760 |
| walker |  | 1868 | 31 | Markdown::ReadmeHeadline { file: samples/README.md } |  |  | 0.760 |
| walker |  | 1884 | 16 | Fs::DirListing { dir: samples/browser-esm-webpack-typescript-react/src } |  |  | 0.760 |
| walker |  | 1889 | 5 | Fs::DirListing { dir: samples/browser-esm-webpack-typescript-react/src/components } |  |  | 0.760 |
| walker |  | 1920 | 31 | Fs::DirListing { dir: samples/browser-esm-vite-react } |  |  | 0.760 |
| walker |  | 1939 | 19 | Json::Identity { file: samples/browser-esm-vite-react/package.json } |  |  | 0.760 |
| walker |  | 1970 | 31 | Fs::DirListing { dir: samples/browser-esm-webpack-small } |  |  | 0.760 |
| walker |  | 1988 | 18 | Json::Identity { file: samples/browser-esm-webpack-small/package.json } |  |  | 0.760 |
| walker |  | 2024 | 36 | Fs::DirListing { dir: samples/browser-esm-vite } |  |  | 0.760 |
| walker |  | 2042 | 18 | Json::Identity { file: samples/browser-esm-vite/package.json } |  |  | 0.760 |
| ns | 2052 |  | 304 | Complete listing of src/languages/definitions/ — all 84 Monarch languages plus _.contribution.ts, register.all.ts and test/ | 2.5 |  | 0.789 |
| walker |  | 2061 | 19 | Fs::DirListing { dir: samples/browser-esm-vite-react/src } |  |  | 0.789 |
| ns | 2065 |  | 13 | One language definition directory, listed in full (rust) | 2.6 |  | 0.784 |
| walker |  | 2071 | 10 | Fs::DirListing { dir: samples/browser-esm-vite-react/src/components } |  |  | 0.784 |
| walker |  | 2133 | 62 | Json::Identity { file: webpack-plugin/package.json } |  |  | 0.785 |
| walker |  | 2143 | 10 | Json::IdentityMeta { file: monaco-lsp-client/package.json } |  |  | 0.785 |
| ns | 2149 |  | 84 | rust/register.ts registerLanguage call | 2.7 | 2.6 | 0.772 |
| walker |  | 2190 | 47 | Code::CodeKey { rung: Names, file: samples/browser-esm-vite/main.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.772 |
| walker |  | 2203 | 13 | Code::CodeKey { rung: Decl, file: samples/browser-esm-vite/main.ts, decl: 1, sub: 0, line: 18 } |  |  | 0.772 |
| walker |  | 2235 | 32 | Code::CodeKey { rung: Decl, file: samples/browser-esm-vite/main.ts, decl: 2, sub: 0, line: 21 } |  |  | 0.772 |
| walker |  | 2306 | 71 | Json::Identity { file: samples/package.json } |  |  | 0.772 |
| walker |  | 2318 | 12 | Json::Entry { file: samples/package.json } |  |  | 0.772 |
| walker |  | 2373 | 55 | Markdown::ReadmeHeadline { file: monaco-lsp-client/README.md } |  |  | 0.772 |
| walker |  | 2429 | 56 | Markdown::ReadmeHeadline { file: webpack-plugin/README.md } |  |  | 0.772 |
| ns | 2431 |  | 282 | _.contribution.ts: ILang/ILangImpl and registerLanguage() | 2.8 |  | 0.735 |
| walker |  | 2441 | 12 | Json::IdentityMeta { file: samples/package.json } |  |  | 0.735 |
| walker |  | 2461 | 20 | Fs::DirListing { dir: .github } |  |  | 0.735 |
| walker |  | 2469 | 8 | Fs::DirListing { dir: .github/workflows } |  |  | 0.735 |
| walker |  | 2506 | 37 | Fs::DirListing { dir: monaco-lsp-client/src/adapters } |  |  | 0.736 |
| ns | 2570 |  | 139 | _.contribution.ts: LazyLanguageLoader and loadLanguage() | 2.9 |  | 0.720 |
| ns | 2659 |  | 89 | Grammar file shape: rust.ts conf + language exports | 2.10 |  | 0.710 |
| walker |  | 2680 | 174 | Fs::DirListing { dir: monaco-lsp-client/src/adapters/languageFeatures } |  |  | 0.710 |
| walker |  | 2716 | 36 | Markdown::HeadingsOutline { file: MAINTAINING.md } |  |  | 0.710 |
| walker |  | 2727 | 11 | Json::Entry { file: samples/electron-esm-webpack/package.json } |  |  | 0.710 |
| ns | 2799 |  | 140 | Tokenization test harness: testRunner.ts types + testTokenization(), and a test file head | 2.11 |  | 0.694 |
| walker |  | 2809 | 82 | Fs::DirListing { dir: samples/legacy } |  |  | 0.694 |
| walker |  | 2813 | 4 | Fs::DirListing { dir: samples/legacy/browser-amd-editor } |  |  | 0.694 |
| walker |  | 2817 | 4 | Fs::DirListing { dir: samples/legacy/browser-amd-localized } |  |  | 0.694 |
| walker |  | 2821 | 4 | Fs::DirListing { dir: samples/legacy/browser-amd-monarch } |  |  | 0.694 |
| walker |  | 2825 | 4 | Fs::DirListing { dir: samples/legacy/browser-amd-requirejs } |  |  | 0.694 |
| walker |  | 2829 | 4 | Fs::DirListing { dir: samples/legacy/browser-amd-shadow-dom } |  |  | 0.694 |
| walker |  | 2833 | 4 | Fs::DirListing { dir: samples/legacy/browser-amd-shared-model } |  |  | 0.694 |
| walker |  | 2837 | 4 | Fs::DirListing { dir: samples/legacy/browser-amd-trusted-types } |  |  | 0.694 |
| ns | 2841 |  | 42 | src/languages/definitions/register.all.ts head, elided middle, tail | 2.12 |  | 0.688 |
| walker |  | 2845 | 8 | Fs::DirListing { dir: samples/legacy/browser-amd-iframe } |  |  | 0.688 |
| walker |  | 2857 | 12 | Fs::DirListing { dir: samples/legacy/browser-amd-diff-editor } |  |  | 0.688 |
| walker |  | 2874 | 17 | Fs::DirListing { dir: samples/legacy/electron-amd } |  |  | 0.688 |
| walker |  | 2891 | 17 | Fs::DirListing { dir: samples/legacy/electron-amd-nodeIntegration } |  |  | 0.688 |
| walker |  | 2907 | 16 | Json::Identity { file: samples/legacy/electron-amd/package.json } |  |  | 0.688 |
| walker |  | 2926 | 19 | Json::Identity { file: samples/legacy/electron-amd-nodeIntegration/package.json } |  |  | 0.688 |
| walker |  | 2964 | 38 | Markdown::HeadingsOutline { file: samples/README.md } |  |  | 0.688 |
| walker |  | 2973 | 9 | Markdown::Section { file: samples/README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.688 |
| ns | 2976 |  | 135 | src/deprecated/: the legacy import paths, all one-line re-exports | 2.13 |  | 0.676 |
| walker |  | 3000 | 27 | Json::Entry { file: monaco-lsp-client/package.json } |  |  | 0.676 |
| walker |  | 3020 | 20 | Code::CodeKey { rung: Names, file: src/editor.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.676 |
| walker |  | 3048 | 28 | Json::Entry { file: webpack-plugin/package.json } |  |  | 0.676 |
| walker |  | 3093 | 45 | Markdown::HeadingsOutline { file: webpack-plugin/README.md } |  |  | 0.676 |
| walker |  | 3104 | 11 | Json::Dependencies { file: samples/browser-esm-vite/package.json } |  |  | 0.676 |
| walker |  | 3115 | 11 | Json::Dependencies { file: samples/browser-esm-vite-react/package.json } |  |  | 0.676 |
| walker |  | 3126 | 11 | Json::Dependencies { file: samples/browser-esm-webpack-typescript-react/package.json } |  |  | 0.676 |
| ns | 3345 |  | 369 | package.json scripts — build, test and dev commands (packaging variants elided) | 3.1 |  | 0.656 |
| walker |  | 3406 | 280 | Code::CodeKey { rung: Names, file: monaco-lsp-client/generator/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.656 |
| walker |  | 3417 | 11 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 9, sub: 0, line: 82 } |  |  | 0.656 |
| walker |  | 3435 | 18 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 32, sub: 0, line: 259 } |  |  | 0.656 |
| walker |  | 3456 | 21 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 21, sub: 0, line: 197 } |  |  | 0.656 |
| walker |  | 3477 | 21 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 22, sub: 0, line: 202 } |  |  | 0.646 |
| ns | 3477 |  | 132 | CONTRIBUTING: the ordered editor-test command sequence | 3.2 |  | 0.646 |
| walker |  | 3498 | 21 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 24, sub: 0, line: 213 } |  |  | 0.646 |
| walker |  | 3519 | 21 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 25, sub: 0, line: 218 } |  |  | 0.646 |
| walker |  | 3540 | 21 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 26, sub: 0, line: 223 } |  |  | 0.646 |
| ns | 3541 |  | 64 | test/ listing plus the full smoke-test directory | 3.3 |  | 0.633 |
| walker |  | 3562 | 22 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 27, sub: 0, line: 228 } |  |  | 0.633 |
| walker |  | 3584 | 22 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 29, sub: 0, line: 241 } |  |  | 0.633 |
| walker |  | 3606 | 22 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 30, sub: 0, line: 246 } |  |  | 0.633 |
| walker |  | 3630 | 24 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 31, sub: 0, line: 251 } |  |  | 0.633 |
| walker |  | 3659 | 29 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 23, sub: 0, line: 207 } |  |  | 0.633 |
| walker |  | 3690 | 31 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 16, sub: 0, line: 154 } |  |  | 0.633 |
| ns | 3692 |  | 151 | Listings of the five src/languages/features dirs — the uniform rich-service file set | 4.1 |  | 0.634 |
| walker |  | 3737 | 47 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 28, sub: 0, line: 233 } |  |  | 0.634 |
| walker |  | 3792 | 55 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 17, sub: 0, line: 159 } |  |  | 0.634 |
| walker |  | 3849 | 57 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 15, sub: 0, line: 145 } |  |  | 0.634 |
| walker |  | 3909 | 60 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 8, sub: 0, line: 73 } |  |  | 0.634 |
| ns | 3952 |  | 260 | css/register.ts — every top-level export (names only) | 4.2 |  | 0.618 |
| walker |  | 3973 | 64 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 13, sub: 0, line: 124 } |  |  | 0.618 |
| walker |  | 4047 | 74 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 12, sub: 0, line: 113 } |  |  | 0.618 |
| walker |  | 4124 | 77 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 14, sub: 0, line: 134 } |  |  | 0.618 |
| walker |  | 4203 | 79 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 1, sub: 0, line: 7 } |  |  | 0.618 |
| ns | 4237 |  | 285 | css: the default ModeConfiguration toggles, the three defaults objects, and the lazy onLanguage hookup | 4.3 | 4.2 | 0.594 |
| walker |  | 4290 | 87 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 11, sub: 0, line: 101 } |  |  | 0.594 |
| walker |  | 4391 | 101 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 19, sub: 0, line: 170 } |  |  | 0.594 |
| ns | 4496 |  | 259 | json/register.ts — every top-level export (names only) | 4.4 |  | 0.580 |
| walker |  | 4503 | 112 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 20, sub: 0, line: 183 } |  |  | 0.580 |
| walker |  | 4540 | 37 | Json::Scripts { file: samples/package.json } |  |  | 0.580 |
| walker |  | 4654 | 114 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 10, sub: 0, line: 86 } |  |  | 0.580 |
| walker |  | 4687 | 33 | Markdown::Section { file: samples/legacy/README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.580 |
| ns | 4765 |  | 269 | json defaults: diagnostic option values, mode toggles, jsonDefaults, the worker interface | 4.5 | 4.4 | 0.561 |
| walker |  | 4833 | 146 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.591 |
| walker |  | 4877 | 44 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.591 |
| walker |  | 4899 | 22 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 4, sub: 0, line: 41 } |  |  | 0.591 |
| walker |  | 4921 | 22 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 8, sub: 0, line: 73 } |  |  | 0.591 |
| walker |  | 4943 | 22 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 32, sub: 0, line: 259 } |  |  | 0.591 |
| walker |  | 4956 | 13 | Json::Entry { file: samples/legacy/electron-amd/package.json } |  |  | 0.591 |
| walker |  | 4969 | 13 | Json::Entry { file: samples/legacy/electron-amd-nodeIntegration/package.json } |  |  | 0.591 |
| walker |  | 4992 | 23 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 5, sub: 0, line: 48 } |  |  | 0.591 |
| ns | 5027 |  | 262 | html/register.ts — export names plus the registerHTMLLanguageService factory | 4.6 |  | 0.577 |
| walker |  | 5052 | 60 | Fs::DirListing { dir: website } |  |  | 0.578 |
| walker |  | 5130 | 78 | Json::IdentityMeta { file: package.json } |  |  | 0.585 |
| walker |  | 5158 | 28 | Markdown::Section { file: webpack-plugin/README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.585 |
| walker |  | 5181 | 23 | Fs::DirListing { dir: scripts/ci } |  |  | 0.585 |
| walker |  | 5206 | 25 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 1, sub: 0, line: 7 } |  |  | 0.585 |
| walker |  | 5231 | 25 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 2, sub: 0, line: 15 } |  |  | 0.585 |
| walker |  | 5256 | 25 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 7, sub: 0, line: 64 } |  |  | 0.585 |
| walker |  | 5281 | 25 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 33, sub: 0, line: 628 } |  |  | 0.585 |
| walker |  | 5307 | 26 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 3, sub: 0, line: 26 } |  |  | 0.585 |
| ns | 5311 |  | 284 | html: the three pre-registered services and their per-language toggles | 4.7 | 4.6 | 0.571 |
| walker |  | 5333 | 26 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 6, sub: 0, line: 57 } |  |  | 0.571 |
| walker |  | 5368 | 35 | Fs::DirListing { dir: src/languages/features/typescript/lib } |  |  | 0.584 |
| walker |  | 5402 | 34 | Json::Dependencies { file: webpack-plugin/package.json } |  |  | 0.584 |
| walker |  | 5428 | 26 | Fs::DirListing { dir: website/src } |  |  | 0.585 |
| walker |  | 5440 | 12 | Fs::DirListing { dir: website/src/runner } |  |  | 0.585 |
| walker |  | 5523 | 83 | Markdown::HeadingsOutline { file: docs/integrate-esm.md } |  |  | 0.585 |
| ns | 5556 |  | 245 | typescript/register.ts — every top-level export (names only) | 4.8 |  | 0.574 |
| walker |  | 5559 | 36 | Json::Dependencies { file: samples/package.json } |  |  | 0.574 |
| walker |  | 5589 | 30 | Markdown::Section { file: README.md, section_index: 13, keeps_default_concavity: false } |  |  | 0.574 |
| walker |  | 5619 | 30 | Json::Scripts { file: samples/browser-esm-esbuild/package.json } |  |  | 0.574 |
| ns | 5791 |  | 235 | LanguageServiceDefaults (typescript) — all members | 4.9 |  | 0.563 |
| walker |  | 5986 | 367 | Code::CodeKey { rung: Names, file: webpack-plugin/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.563 |
| ns | 6027 |  | 236 | TypeScriptWorker — all 21 proxy methods (names only) | 4.10 | 4.8 | 0.553 |
| walker |  | 6030 | 44 | Code::CodeKey { rung: Decl, file: webpack-plugin/src/index.ts, decl: 11, sub: 0, line: 159 } |  |  | 0.553 |
| walker |  | 6055 | 25 | Code::CodeKey { rung: Decl, file: webpack-plugin/src/index.ts, decl: 5, sub: 0, line: 53 } |  |  | 0.553 |
| walker |  | 6082 | 27 | Code::CodeKey { rung: Decl, file: webpack-plugin/src/index.ts, decl: 14, sub: 0, line: 207 } |  |  | 0.553 |
| walker |  | 6119 | 37 | Code::CodeKey { rung: Decl, file: webpack-plugin/src/index.ts, decl: 4, sub: 0, line: 43 } |  |  | 0.553 |
| walker |  | 6159 | 40 | Code::CodeKey { rung: Decl, file: webpack-plugin/src/index.ts, decl: 7, sub: 0, line: 63 } |  |  | 0.553 |
| walker |  | 6207 | 48 | Code::CodeKey { rung: Decl, file: webpack-plugin/src/index.ts, decl: 8, sub: 0, line: 90 } |  |  | 0.553 |
| walker |  | 6263 | 56 | Code::CodeKey { rung: Decl, file: webpack-plugin/src/index.ts, decl: 19, sub: 0, line: 334 } |  |  | 0.553 |
| ns | 6318 |  | 291 | typescript/javascript defaults and their divergence | 4.11 | 4.8 | 0.540 |
| walker |  | 6326 | 63 | Code::CodeKey { rung: Decl, file: webpack-plugin/src/index.ts, decl: 2, sub: 0, line: 12 } |  |  | 0.540 |
| walker |  | 6391 | 65 | Code::CodeKey { rung: Decl, file: webpack-plugin/src/index.ts, decl: 10, sub: 0, line: 150 } |  |  | 0.540 |
| walker |  | 6489 | 98 | Code::CodeKey { rung: Decl, file: webpack-plugin/src/index.ts, decl: 18, sub: 0, line: 238 } |  |  | 0.540 |
| walker |  | 6512 | 23 | Code::CodeKey { rung: Doc, file: webpack-plugin/src/index.ts, decl: 3, sub: 0, line: 24 } |  |  | 0.540 |
| walker |  | 6543 | 31 | Code::CodeKey { rung: Doc, file: webpack-plugin/src/index.ts, decl: 4, sub: 0, line: 43 } |  |  | 0.540 |
| ns | 6547 |  | 229 | The four *Mode.ts entry points and the tsMode re-export tail | 5.1 |  | 0.535 |
| ns | 6761 |  | 214 | cssMode.setupMode — WorkerManager, the worker accessor, and ModeConfiguration-gated provider registration | 5.2 | 5.1 | 0.525 |
| ns | 7114 |  | 353 | common/lspLanguageFeatures.ts — the shared provider adapters and LSP conversion helpers | 5.3 |  | 0.517 |
| walker |  | 7162 | 619 | Code::CodeKey { rung: Decl, file: webpack-plugin/src/index.ts, decl: 9, sub: 0, line: 102 } |  |  | 0.517 |
| walker |  | 7193 | 31 | Json::Scripts { file: samples/browser-esm-parcel/package.json } |  |  | 0.517 |
| walker |  | 7262 | 69 | Json::Scripts { file: webpack-plugin/package.json } |  |  | 0.517 |
| ns | 7421 |  | 307 | Web-worker plumbing: MonacoEnvironment hooks, createWebWorker, IWebWorkerOptions fields, worker entry points | 5.4 |  | 0.507 |
| walker |  | 7428 | 166 | Json::Entry { file: package.json } |  |  | 0.508 |
| walker |  | 7471 | 43 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.508 |
| walker |  | 7509 | 38 | Json::Scripts { file: samples/browser-esm-webpack/package.json } |  |  | 0.508 |
| walker |  | 7547 | 38 | Json::Scripts { file: samples/browser-esm-webpack-monaco-plugin/package.json } |  |  | 0.508 |
| ns | 7576 |  | 155 | WorkerManager lifecycle and the CSSWorker entry points | 5.5 |  | 0.504 |
| walker |  | 7627 | 80 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.504 |
| walker |  | 7709 | 82 | Json::Scripts { file: monaco-lsp-client/package.json } |  |  | 0.504 |
| walker |  | 7712 | 3 | Fs::DirListing { dir: website/index } |  |  | 0.504 |
| walker |  | 7758 | 46 | Fs::DirListing { dir: website/src/website } |  |  | 0.504 |
| walker |  | 7773 | 15 | Fs::DirListing { dir: website/src/website/data } |  |  | 0.504 |
| walker |  | 7800 | 27 | Fs::DirListing { dir: website/src/website/pages } |  |  | 0.504 |
| ns | 7830 |  | 254 | typescript/languageFeatures.ts — the TypeScript-specific adapter roster | 5.6 |  | 0.498 |
| walker |  | 7834 | 34 | Fs::DirListing { dir: website/src/website/data/playground-samples } |  |  | 0.498 |
| walker |  | 7853 | 19 | Fs::DirListing { dir: website/src/website/data/playground-samples/customizing-the-appearence } |  |  | 0.498 |
| walker |  | 7874 | 21 | Fs::DirListing { dir: website/src/website/data/playground-samples/creating-the-diffeditor } |  |  | 0.498 |
| walker |  | 7905 | 31 | Fs::DirListing { dir: website/src/website/data/playground-samples/creating-the-editor } |  |  | 0.498 |
| ns | 7909 |  | 79 | monaco-lsp-client package: complete tree listing | 6.1 |  | 0.507 |
| walker |  | 7945 | 40 | Fs::DirListing { dir: website/src/website/components } |  |  | 0.507 |
| walker |  | 7987 | 42 | Fs::DirListing { dir: website/src/website/utils } |  |  | 0.507 |
| walker |  | 8038 | 51 | Fs::DirListing { dir: test/smoke } |  |  | 0.522 |
| walker |  | 8092 | 54 | Json::Scripts { file: samples/browser-esm-vite/package.json } |  |  | 0.522 |
| ns | 8116 |  | 207 | MonacoLspClient wiring + index exports | 6.2 | 6.1 | 0.516 |
| ns | 8165 |  | 49 | webpack-plugin package: complete tree listing | 6.3 |  | 0.523 |
| walker |  | 8172 | 80 | Fs::DirListing { dir: test/manual } |  |  | 0.523 |
| walker |  | 8228 | 56 | Json::Scripts { file: samples/browser-esm-webpack-small/package.json } |  |  | 0.523 |
| walker |  | 8262 | 34 | Json::Scripts { file: samples/legacy/electron-amd/package.json } |  |  | 0.523 |
| walker |  | 8296 | 34 | Json::Scripts { file: samples/legacy/electron-amd-nodeIntegration/package.json } |  |  | 0.523 |
| walker |  | 8355 | 59 | Json::Scripts { file: samples/electron-esm-webpack/package.json } |  |  | 0.523 |
| walker |  | 8418 | 63 | Json::Scripts { file: samples/browser-esm-webpack-typescript/package.json } |  |  | 0.523 |
| ns | 8449 |  | 284 | webpack-plugin option fields and peer-dependency contract | 6.4 | 6.3 | 0.527 |
| walker |  | 8477 | 59 | Markdown::Section { file: README.md, section_index: 11, keeps_default_concavity: false } |  |  | 0.527 |
| walker |  | 8544 | 67 | Fs::DirListing { dir: website/src/website/pages/playground } |  |  | 0.527 |
| ns | 8613 |  | 164 | build/ and scripts/ tree listings | 7.1 |  | 0.517 |
| walker |  | 8625 | 81 | Json::Dependencies { file: monaco-lsp-client/package.json } |  |  | 0.517 |
| walker |  | 8691 | 66 | Json::Scripts { file: samples/browser-esm-webpack-typescript-react/package.json } |  |  | 0.517 |
| ns | 8762 |  | 149 | build-monaco-editor.ts run() — what the published package is made of | 7.2 |  | 0.514 |
| walker |  | 8801 | 110 | Markdown::Section { file: MAINTAINING.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.514 |
| walker |  | 8876 | 75 | Fs::DirListing { dir: website/src/website/data/playground-samples/extending-language-services } |  |  | 0.514 |
| walker |  | 8950 | 74 | Fs::DirListing { dir: website/src/website/data/playground-samples/interacting-with-the-editor } |  |  | 0.514 |
| ns | 9000 |  | 238 | check-samples.ts — the invariant every new language must satisfy | 7.3 |  | 0.507 |
| walker |  | 9037 | 87 | Json::Scripts { file: samples/browser-esm-vite-react/package.json } |  |  | 0.507 |
| walker |  | 9141 | 104 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.507 |
| ns | 9166 |  | 166 | package.json distribution fields: typings, main, module, exports | 8.1 |  | 0.512 |
| walker |  | 9217 | 76 | Markdown::Section { file: README.md, section_index: 12, keeps_default_concavity: false } |  |  | 0.512 |
| walker |  | 9268 | 51 | Markdown::Section { file: MAINTAINING.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.512 |
| walker |  | 9272 | 4 | Fs::DirListing { dir: test/smoke/amd } |  |  | 0.512 |
| walker |  | 9280 | 8 | Fs::DirListing { dir: website/static } |  |  | 0.512 |
| ns | 9420 |  | 254 | README Installing + CHANGELOG head (0.55.x breaking changes) | 8.2 |  | 0.507 |
| walker |  | 9432 | 152 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.509 |
| ns | 9524 |  | 104 | samples/ listing — every integration sample directory | 8.3 |  | 0.516 |
| ns | 9562 |  | 38 | docs/ listing | 8.4 |  | 0.519 |
| walker |  | 9602 | 170 | Json::IdentityMeta { file: webpack-plugin/package.json } |  |  | 0.519 |
| ns | 9645 |  | 83 | integrate-esm.md section headings | 8.5 | 8.4 | 0.521 |
| walker |  | 9702 | 100 | Markdown::Section { file: samples/README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.521 |
| walker |  | 9715 | 13 | Code::CodeKey { rung: Names, file: webpack-plugin/src/loader-utils.d.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.521 |
| walker |  | 9724 | 9 | Fs::DirListing { dir: website/scripts } |  |  | 0.521 |
| ns | 9771 |  | 126 | website/ and CI/publishing config listings | 8.6 |  | 0.532 |

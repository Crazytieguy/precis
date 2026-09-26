Score(3000)=0.712 I=0.802 C=0.632 ns_rows≤3K=23/56 grid(1000/1442/2080/3000/4327/6240/9000)=0.654/0.694/0.782/0.712/0.623/0.558/0.496

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 67 |  | 67 | README title + one-sentence identity | 1.1 |  | 0.000 |
| ns | 92 |  | 25 | src/ listing — every child of the package source directory | 1.2 |  | 0.000 |
| walker |  | 116 | 116 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 122 | 6 | Fs::DirListing { dir: test-results } |  |  | 0.000 |
| walker |  | 189 | 67 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.444 |
| walker |  | 214 | 25 | Fs::DirListing { dir: src } |  |  | 1.000 |
| ns | 216 |  | 124 | src/index.ts — the whole package entry point (9 lines) | 1.3 |  | 0.707 |
| walker |  | 223 | 9 | Fs::DirListing { dir: src/internal/common } |  |  | 0.707 |
| walker |  | 234 | 11 | Fs::DirListing { dir: src/deprecated } |  |  | 0.707 |
| walker |  | 241 | 7 | Fs::DirListing { dir: src/deprecated/basic-languages } |  |  | 0.707 |
| walker |  | 251 | 10 | Fs::DirListing { dir: src/deprecated/editor } |  |  | 0.707 |
| walker |  | 262 | 11 | Fs::DirListing { dir: src/languages } |  |  | 0.707 |
| walker |  | 275 | 13 | Fs::DirListing { dir: src/deprecated/language } |  |  | 0.707 |
| walker |  | 287 | 12 | Fs::DirListing { dir: src/deprecated/language/css } |  |  | 0.707 |
| walker |  | 299 | 12 | Fs::DirListing { dir: src/deprecated/language/html } |  |  | 0.707 |
| walker |  | 311 | 12 | Fs::DirListing { dir: src/deprecated/language/json } |  |  | 0.707 |
| walker |  | 323 | 12 | Fs::DirListing { dir: src/deprecated/language/typescript } |  |  | 0.707 |
| ns | 332 |  | 116 | Complete repository root listing | 1.4 |  | 0.845 |
| walker |  | 344 | 21 | Fs::DirListing { dir: src/languages/features } |  |  | 0.855 |
| walker |  | 351 | 7 | Fs::DirListing { dir: src/languages/features/common } |  |  | 0.855 |
| walker |  | 375 | 24 | Fs::DirListing { dir: src/languages/features/css } |  |  | 0.856 |
| walker |  | 399 | 24 | Fs::DirListing { dir: src/languages/features/html } |  |  | 0.856 |
| walker |  | 426 | 27 | Fs::DirListing { dir: monaco-lsp-client } |  |  | 0.857 |
| walker |  | 430 | 4 | Fs::DirListing { dir: monaco-lsp-client/generator } |  |  | 0.857 |
| walker |  | 445 | 15 | Fs::DirListing { dir: monaco-lsp-client/src } |  |  | 0.857 |
| ns | 471 |  | 139 | package.json identity header (name, version, vscodeRef, license) | 1.5 |  | 0.775 |
| walker |  | 474 | 29 | Fs::DirListing { dir: src/languages/features/json } |  |  | 0.776 |
| walker |  | 503 | 29 | Fs::DirListing { dir: webpack-plugin } |  |  | 0.777 |
| walker |  | 523 | 20 | Fs::DirListing { dir: webpack-plugin/src } |  |  | 0.778 |
| walker |  | 527 | 4 | Fs::DirListing { dir: webpack-plugin/src/loaders } |  |  | 0.778 |
| walker |  | 559 | 32 | Fs::DirListing { dir: src/languages/features/typescript } |  |  | 0.780 |
| walker |  | 564 | 5 | Fs::DirListing { dir: .devcontainer } |  |  | 0.780 |
| walker |  | 569 | 5 | Fs::DirListing { dir: .husky } |  |  | 0.780 |
| walker |  | 577 | 8 | Fs::DirListing { dir: webpack-plugin/src/plugins } |  |  | 0.780 |
| ns | 588 |  | 117 | The three one-to-six-line re-export files src/index.ts depends on | 1.6 | 1.2 | 0.716 |
| walker |  | 615 | 38 | Fs::DirListing { dir: docs } |  |  | 0.716 |
| walker |  | 621 | 6 | Fs::DirListing { dir: scripts } |  |  | 0.716 |
| walker |  | 691 | 70 | Json::Identity { file: package.json } |  |  | 0.740 |
| walker |  | 695 | 4 | Fs::DirListing { dir: scripts/lib } |  |  | 0.740 |
| ns | 734 |  | 146 | All README section headings (locations) | 1.7 |  | 0.656 |
| walker |  | 743 | 48 | Json::Identity { file: monaco-lsp-client/package.json } |  |  | 0.656 |
| ns | 927 |  | 193 | CONTRIBUTING 'Source Code Structure' — the monaco-editor-core boundary | 1.8 |  | 0.625 |
| walker |  | 1015 | 272 | Fs::DirListing { dir: src/features } |  |  | 0.657 |
| ns | 1097 |  | 170 | README Concepts: Models and URIs | 1.9 | 1.7 | 0.649 |
| ns | 1259 |  | 162 | README Concepts: Editors, Providers, Disposables | 1.10 | 1.7 | 0.636 |
| walker |  | 1319 | 304 | Fs::DirListing { dir: src/languages/definitions } |  |  | 0.661 |
| ns | 1324 |  | 65 | Listings of every src/ subtree except the two huge uniform ones | 2.1 |  | 0.712 |
| walker |  | 1331 | 12 | Fs::DirListing { dir: .azure-pipelines } |  |  | 0.712 |
| walker |  | 1343 | 12 | Fs::DirListing { dir: .vscode } |  |  | 0.712 |
| walker |  | 1405 | 62 | Json::Identity { file: webpack-plugin/package.json } |  |  | 0.712 |
| walker |  | 1418 | 13 | Fs::DirListing { dir: test } |  |  | 0.712 |
| ns | 1430 |  | 106 | Uniform shape of a feature shim (hover, gpu) and the one that is not uniform (find) | 2.2 |  | 0.694 |
| walker |  | 1522 | 104 | Fs::DirListing { dir: samples } |  |  | 0.695 |
| walker |  | 1526 | 4 | Fs::DirListing { dir: samples/nwjs-amd } |  |  | 0.695 |
| walker |  | 1530 | 4 | Fs::DirListing { dir: samples/nwjs-amd-v2 } |  |  | 0.695 |
| walker |  | 1554 | 24 | Fs::DirListing { dir: samples/browser-esm-esbuild } |  |  | 0.695 |
| walker |  | 1579 | 25 | Fs::DirListing { dir: samples/browser-esm-webpack } |  |  | 0.695 |
| walker |  | 1604 | 25 | Fs::DirListing { dir: samples/browser-esm-webpack-monaco-plugin } |  |  | 0.695 |
| walker |  | 1629 | 25 | Fs::DirListing { dir: samples/electron-esm-webpack } |  |  | 0.695 |
| walker |  | 1660 | 31 | Fs::DirListing { dir: samples/browser-esm-webpack-small } |  |  | 0.695 |
| walker |  | 1675 | 15 | Json::Identity { file: samples/browser-esm-esbuild/package.json } |  |  | 0.695 |
| walker |  | 1692 | 17 | Json::Identity { file: samples/browser-esm-webpack/package.json } |  |  | 0.695 |
| ns | 1702 |  | 272 | Complete listing of src/features/ — all 65 editor-feature shim directories | 2.3 |  | 0.769 |
| walker |  | 1709 | 17 | Json::Identity { file: samples/electron-esm-webpack/package.json } |  |  | 0.769 |
| walker |  | 1730 | 21 | Fs::DirListing { dir: samples/browser-esm-parcel } |  |  | 0.769 |
| walker |  | 1738 | 8 | Fs::DirListing { dir: samples/browser-esm-parcel/src } |  |  | 0.769 |
| ns | 1746 |  | 44 | src/features/register.all.ts head, elided middle, tail | 2.4 |  | 0.757 |
| walker |  | 1753 | 15 | Json::Identity { file: samples/browser-esm-parcel/package.json } |  |  | 0.757 |
| walker |  | 1771 | 18 | Json::Identity { file: samples/browser-esm-webpack-small/package.json } |  |  | 0.757 |
| walker |  | 1793 | 22 | Fs::DirListing { dir: samples/browser-esm-webpack-typescript } |  |  | 0.757 |
| walker |  | 1801 | 8 | Fs::DirListing { dir: samples/browser-esm-webpack-typescript/src } |  |  | 0.757 |
| walker |  | 1821 | 20 | Json::Identity { file: samples/browser-esm-webpack-monaco-plugin/package.json } |  |  | 0.757 |
| walker |  | 1841 | 20 | Json::Identity { file: samples/browser-esm-webpack-typescript/package.json } |  |  | 0.757 |
| walker |  | 1868 | 27 | Fs::DirListing { dir: samples/browser-esm-webpack-typescript-react } |  |  | 0.757 |
| walker |  | 1884 | 16 | Fs::DirListing { dir: samples/browser-esm-webpack-typescript-react/src } |  |  | 0.757 |
| walker |  | 1889 | 5 | Fs::DirListing { dir: samples/browser-esm-webpack-typescript-react/src/components } |  |  | 0.757 |
| walker |  | 1909 | 20 | Json::Identity { file: samples/browser-esm-webpack-typescript-react/package.json } |  |  | 0.757 |
| walker |  | 1940 | 31 | Fs::DirListing { dir: samples/browser-esm-vite-react } |  |  | 0.757 |
| walker |  | 1959 | 19 | Json::Identity { file: samples/browser-esm-vite-react/package.json } |  |  | 0.757 |
| walker |  | 1995 | 36 | Fs::DirListing { dir: samples/browser-esm-vite } |  |  | 0.757 |
| walker |  | 2013 | 18 | Json::Identity { file: samples/browser-esm-vite/package.json } |  |  | 0.757 |
| walker |  | 2032 | 19 | Fs::DirListing { dir: samples/browser-esm-vite-react/src } |  |  | 0.757 |
| walker |  | 2042 | 10 | Fs::DirListing { dir: samples/browser-esm-vite-react/src/components } |  |  | 0.757 |
| ns | 2050 |  | 304 | Complete listing of src/languages/definitions/ — all 84 Monarch languages plus _.contribution.ts, register.all.ts and test/ | 2.5 |  | 0.787 |
| ns | 2063 |  | 13 | One language definition directory, listed in full (rust) | 2.6 |  | 0.782 |
| walker |  | 2113 | 71 | Json::Identity { file: samples/package.json } |  |  | 0.782 |
| walker |  | 2125 | 12 | Json::Entry { file: samples/package.json } |  |  | 0.782 |
| walker |  | 2136 | 11 | Json::Dependencies { file: samples/browser-esm-vite/package.json } |  |  | 0.782 |
| walker |  | 2147 | 11 | Json::Dependencies { file: samples/browser-esm-vite-react/package.json } |  |  | 0.769 |
| ns | 2147 |  | 84 | rust/register.ts registerLanguage call | 2.7 | 2.6 | 0.769 |
| walker |  | 2158 | 11 | Json::Dependencies { file: samples/browser-esm-webpack-typescript-react/package.json } |  |  | 0.769 |
| walker |  | 2178 | 20 | Fs::DirListing { dir: .github } |  |  | 0.769 |
| walker |  | 2186 | 8 | Fs::DirListing { dir: .github/workflows } |  |  | 0.769 |
| walker |  | 2197 | 11 | Json::Entry { file: samples/electron-esm-webpack/package.json } |  |  | 0.769 |
| walker |  | 2234 | 37 | Fs::DirListing { dir: monaco-lsp-client/src/adapters } |  |  | 0.770 |
| walker |  | 2261 | 27 | Json::Entry { file: monaco-lsp-client/package.json } |  |  | 0.770 |
| walker |  | 2343 | 82 | Fs::DirListing { dir: samples/legacy } |  |  | 0.770 |
| walker |  | 2347 | 4 | Fs::DirListing { dir: samples/legacy/browser-amd-editor } |  |  | 0.770 |
| walker |  | 2351 | 4 | Fs::DirListing { dir: samples/legacy/browser-amd-localized } |  |  | 0.770 |
| walker |  | 2355 | 4 | Fs::DirListing { dir: samples/legacy/browser-amd-monarch } |  |  | 0.770 |
| walker |  | 2359 | 4 | Fs::DirListing { dir: samples/legacy/browser-amd-requirejs } |  |  | 0.770 |
| walker |  | 2363 | 4 | Fs::DirListing { dir: samples/legacy/browser-amd-shadow-dom } |  |  | 0.770 |
| walker |  | 2367 | 4 | Fs::DirListing { dir: samples/legacy/browser-amd-shared-model } |  |  | 0.770 |
| walker |  | 2371 | 4 | Fs::DirListing { dir: samples/legacy/browser-amd-trusted-types } |  |  | 0.770 |
| walker |  | 2379 | 8 | Fs::DirListing { dir: samples/legacy/browser-amd-iframe } |  |  | 0.770 |
| walker |  | 2391 | 12 | Fs::DirListing { dir: samples/legacy/browser-amd-diff-editor } |  |  | 0.770 |
| walker |  | 2408 | 17 | Fs::DirListing { dir: samples/legacy/electron-amd } |  |  | 0.770 |
| walker |  | 2425 | 17 | Fs::DirListing { dir: samples/legacy/electron-amd-nodeIntegration } |  |  | 0.770 |
| ns | 2429 |  | 282 | _.contribution.ts: ILang/ILangImpl and registerLanguage() | 2.8 |  | 0.734 |
| walker |  | 2441 | 16 | Json::Identity { file: samples/legacy/electron-amd/package.json } |  |  | 0.734 |
| walker |  | 2460 | 19 | Json::Identity { file: samples/legacy/electron-amd-nodeIntegration/package.json } |  |  | 0.734 |
| walker |  | 2488 | 28 | Json::Entry { file: webpack-plugin/package.json } |  |  | 0.734 |
| walker |  | 2522 | 34 | Json::Dependencies { file: webpack-plugin/package.json } |  |  | 0.734 |
| walker |  | 2558 | 36 | Json::Dependencies { file: samples/package.json } |  |  | 0.734 |
| ns | 2568 |  | 139 | _.contribution.ts: LazyLanguageLoader and loadLanguage() | 2.9 |  | 0.718 |
| walker |  | 2597 | 39 | Json::Scripts { file: samples/package.json } |  |  | 0.718 |
| ns | 2657 |  | 89 | Grammar file shape: rust.ts conf + language exports | 2.10 |  | 0.707 |
| walker |  | 2743 | 146 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.747 |
| walker |  | 2787 | 44 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.747 |
| ns | 2797 |  | 140 | Tokenization test harness: testRunner.ts types + testTokenization(), and a test file head | 2.11 |  | 0.730 |
| walker |  | 2800 | 13 | Json::Entry { file: samples/legacy/electron-amd/package.json } |  |  | 0.730 |
| walker |  | 2813 | 13 | Json::Entry { file: samples/legacy/electron-amd-nodeIntegration/package.json } |  |  | 0.730 |
| ns | 2839 |  | 42 | src/languages/definitions/register.all.ts head, elided middle, tail | 2.12 |  | 0.724 |
| walker |  | 2856 | 43 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.724 |
| walker |  | 2936 | 80 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.724 |
| ns | 2974 |  | 135 | src/deprecated/: the legacy import paths, all one-line re-exports | 2.13 |  | 0.711 |
| walker |  | 2996 | 60 | Fs::DirListing { dir: website } |  |  | 0.712 |
| walker |  | 3004 | 8 | Fs::DirListing { dir: website/static } |  |  | 0.712 |
| walker |  | 3013 | 9 | Fs::DirListing { dir: website/scripts } |  |  | 0.712 |
| walker |  | 3028 | 15 | Fs::DirListing { dir: website/static/monarch } |  |  | 0.712 |
| walker |  | 3051 | 23 | Fs::DirListing { dir: scripts/ci } |  |  | 0.712 |
| ns | 3343 |  | 369 | package.json scripts — build, test and dev commands (packaging variants elided) | 3.1 |  | 0.691 |
| walker |  | 3345 | 294 | Code::CodeKey { rung: Names, file: monaco-lsp-client/generator/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.691 |
| walker |  | 3356 | 11 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 9, sub: 0, line: 82 } |  |  | 0.691 |
| walker |  | 3374 | 18 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 32, sub: 0, line: 259 } |  |  | 0.691 |
| walker |  | 3395 | 21 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 21, sub: 0, line: 197 } |  |  | 0.691 |
| walker |  | 3416 | 21 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 22, sub: 0, line: 202 } |  |  | 0.691 |
| walker |  | 3437 | 21 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 24, sub: 0, line: 213 } |  |  | 0.691 |
| walker |  | 3458 | 21 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 25, sub: 0, line: 218 } |  |  | 0.691 |
| ns | 3475 |  | 132 | CONTRIBUTING: the ordered editor-test command sequence | 3.2 |  | 0.680 |
| walker |  | 3479 | 21 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 26, sub: 0, line: 223 } |  |  | 0.680 |
| walker |  | 3501 | 22 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 27, sub: 0, line: 228 } |  |  | 0.680 |
| walker |  | 3523 | 22 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 29, sub: 0, line: 241 } |  |  | 0.680 |
| ns | 3539 |  | 64 | test/ listing plus the full smoke-test directory | 3.3 |  | 0.666 |
| walker |  | 3545 | 22 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 30, sub: 0, line: 246 } |  |  | 0.666 |
| walker |  | 3569 | 24 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 31, sub: 0, line: 251 } |  |  | 0.666 |
| walker |  | 3593 | 24 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 34, sub: 0, line: 684 } |  |  | 0.666 |
| walker |  | 3622 | 29 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 23, sub: 0, line: 207 } |  |  | 0.666 |
| walker |  | 3653 | 31 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 16, sub: 0, line: 154 } |  |  | 0.666 |
| ns | 3690 |  | 151 | Listings of the five src/languages/features dirs — the uniform rich-service file set | 4.1 |  | 0.665 |
| walker |  | 3700 | 47 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 28, sub: 0, line: 233 } |  |  | 0.665 |
| walker |  | 3755 | 55 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 17, sub: 0, line: 159 } |  |  | 0.665 |
| walker |  | 3812 | 57 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 15, sub: 0, line: 145 } |  |  | 0.665 |
| walker |  | 3872 | 60 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 8, sub: 0, line: 73 } |  |  | 0.665 |
| walker |  | 3936 | 64 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 13, sub: 0, line: 124 } |  |  | 0.665 |
| ns | 3950 |  | 260 | css/register.ts — every top-level export (names only) | 4.2 |  | 0.649 |
| walker |  | 4010 | 74 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 12, sub: 0, line: 113 } |  |  | 0.649 |
| walker |  | 4087 | 77 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 14, sub: 0, line: 134 } |  |  | 0.649 |
| walker |  | 4166 | 79 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 1, sub: 0, line: 7 } |  |  | 0.649 |
| ns | 4235 |  | 285 | css: the default ModeConfiguration toggles, the three defaults objects, and the lazy onLanguage hookup | 4.3 | 4.2 | 0.623 |
| walker |  | 4253 | 87 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 11, sub: 0, line: 101 } |  |  | 0.623 |
| walker |  | 4354 | 101 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 19, sub: 0, line: 170 } |  |  | 0.623 |
| walker |  | 4468 | 114 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 10, sub: 0, line: 86 } |  |  | 0.623 |
| ns | 4494 |  | 259 | json/register.ts — every top-level export (names only) | 4.4 |  | 0.609 |
| walker |  | 4580 | 112 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 20, sub: 0, line: 183 } |  |  | 0.609 |
| walker |  | 4587 | 7 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 34, sub: 0, line: 684 } |  |  | 0.609 |
| walker |  | 4617 | 30 | Json::Scripts { file: samples/browser-esm-esbuild/package.json } |  |  | 0.609 |
| walker |  | 4641 | 24 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.609 |
| walker |  | 4672 | 31 | Json::Scripts { file: samples/browser-esm-parcel/package.json } |  |  | 0.609 |
| walker |  | 4707 | 35 | Fs::DirListing { dir: src/languages/features/typescript/lib } |  |  | 0.623 |
| walker |  | 4719 | 12 | Code::CodeKey { rung: Names, file: samples/browser-esm-vite-react/src/main.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.623 |
| walker |  | 4763 | 44 | Code::CodeKey { rung: Decl, file: samples/browser-esm-vite-react/src/main.tsx, decl: 1, sub: 0, line: 6 } |  |  | 0.604 |
| ns | 4763 |  | 269 | json defaults: diagnostic option values, mode toggles, jsonDefaults, the worker interface | 4.5 | 4.4 | 0.604 |
| walker |  | 4775 | 12 | Code::CodeKey { rung: Names, file: samples/browser-esm-webpack-typescript-react/src/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.604 |
| walker |  | 4819 | 44 | Code::CodeKey { rung: Decl, file: samples/browser-esm-webpack-typescript-react/src/index.tsx, decl: 1, sub: 0, line: 7 } |  |  | 0.604 |
| walker |  | 4845 | 26 | Fs::DirListing { dir: website/src } |  |  | 0.604 |
| walker |  | 4857 | 12 | Fs::DirListing { dir: website/src/runner } |  |  | 0.604 |
| walker |  | 4926 | 69 | Json::Scripts { file: webpack-plugin/package.json } |  |  | 0.604 |
| ns | 5025 |  | 262 | html/register.ts — export names plus the registerHTMLLanguageService factory | 4.6 |  | 0.591 |
| walker |  | 5094 | 168 | Json::Entry { file: package.json } |  |  | 0.591 |
| walker |  | 5132 | 38 | Json::Scripts { file: samples/browser-esm-webpack/package.json } |  |  | 0.591 |
| walker |  | 5170 | 38 | Json::Scripts { file: samples/browser-esm-webpack-monaco-plugin/package.json } |  |  | 0.591 |
| walker |  | 5272 | 102 | Code::CodeKey { rung: Names, file: samples/electron-esm-webpack/main.js, decl: 0, sub: 0, line: 0 } |  |  | 0.591 |
| walker |  | 5301 | 29 | Code::CodeKey { rung: Decl, file: samples/electron-esm-webpack/main.js, decl: 7, sub: 0, line: 28 } |  |  | 0.591 |
| ns | 5309 |  | 284 | html: the three pre-registered services and their per-language toggles | 4.7 | 4.6 | 0.577 |
| walker |  | 5332 | 31 | Code::CodeKey { rung: Decl, file: samples/electron-esm-webpack/main.js, decl: 6, sub: 0, line: 22 } |  |  | 0.577 |
| walker |  | 5434 | 102 | Code::CodeKey { rung: Names, file: samples/legacy/electron-amd/main.js, decl: 0, sub: 0, line: 0 } |  |  | 0.577 |
| walker |  | 5463 | 29 | Code::CodeKey { rung: Decl, file: samples/legacy/electron-amd/main.js, decl: 7, sub: 0, line: 28 } |  |  | 0.577 |
| walker |  | 5494 | 31 | Code::CodeKey { rung: Decl, file: samples/legacy/electron-amd/main.js, decl: 6, sub: 0, line: 22 } |  |  | 0.577 |
| ns | 5554 |  | 245 | typescript/register.ts — every top-level export (names only) | 4.8 |  | 0.566 |
| walker |  | 5596 | 102 | Code::CodeKey { rung: Names, file: samples/legacy/electron-amd-nodeIntegration/main.js, decl: 0, sub: 0, line: 0 } |  |  | 0.566 |
| walker |  | 5625 | 29 | Code::CodeKey { rung: Decl, file: samples/legacy/electron-amd-nodeIntegration/main.js, decl: 7, sub: 0, line: 33 } |  |  | 0.566 |
| walker |  | 5656 | 31 | Code::CodeKey { rung: Decl, file: samples/legacy/electron-amd-nodeIntegration/main.js, decl: 6, sub: 0, line: 27 } |  |  | 0.566 |
| ns | 5789 |  | 235 | LanguageServiceDefaults (typescript) — all members | 4.9 |  | 0.555 |
| walker |  | 5834 | 178 | Json::Scripts { file: package.json } |  |  | 0.559 |
| walker |  | 5986 | 152 | Json::ScriptsTail { file: package.json, chunk: 1 } |  |  | 0.560 |
| ns | 6025 |  | 236 | TypeScriptWorker — all 21 proxy methods (names only) | 4.10 | 4.8 | 0.549 |
| walker |  | 6138 | 152 | Json::ScriptsTail { file: package.json, chunk: 2 } |  |  | 0.552 |
| walker |  | 6303 | 165 | Json::ScriptsTail { file: package.json, chunk: 3 } |  |  | 0.567 |
| ns | 6316 |  | 291 | typescript/javascript defaults and their divergence | 4.11 | 4.8 | 0.554 |
| walker |  | 6347 | 44 | Code::CodeKey { rung: Names, file: webpack-plugin/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.554 |
| walker |  | 6393 | 46 | Code::CodeKey { rung: Decl, file: webpack-plugin/src/index.ts, decl: 2, sub: 0, line: 159 } |  |  | 0.554 |
| walker |  | 6474 | 81 | Json::Dependencies { file: monaco-lsp-client/package.json } |  |  | 0.554 |
| walker |  | 6504 | 30 | Code::CodeKey { rung: Names, file: src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.555 |
| walker |  | 6535 | 31 | Code::CodeKey { rung: Names, file: samples/browser-esm-webpack-monaco-plugin/index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.555 |
| ns | 6545 |  | 229 | The four *Mode.ts entry points and the tsMode re-export tail | 5.1 |  | 0.550 |
| walker |  | 6575 | 40 | Code::CodeKey { rung: Decl, file: samples/browser-esm-webpack-monaco-plugin/index.js, decl: 2, sub: 0, line: 12 } |  |  | 0.550 |
| walker |  | 6659 | 84 | Code::CodeKey { rung: Decl, file: samples/browser-esm-webpack-monaco-plugin/index.js, decl: 1, sub: 0, line: 3 } |  |  | 0.550 |
| walker |  | 6741 | 82 | Json::Scripts { file: monaco-lsp-client/package.json } |  |  | 0.550 |
| walker |  | 6757 | 16 | Code::CodeKey { rung: Names, file: samples/browser-esm-webpack-typescript/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.550 |
| ns | 6759 |  | 214 | cssMode.setupMode — WorkerManager, the worker accessor, and ModeConfiguration-gated provider registration | 5.2 | 5.1 | 0.540 |
| walker |  | 6798 | 41 | Code::CodeKey { rung: Decl, file: samples/browser-esm-webpack-typescript/src/index.ts, decl: 1, sub: 0, line: 23 } |  |  | 0.540 |
| walker |  | 6820 | 22 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 4, sub: 0, line: 41 } |  |  | 0.540 |
| walker |  | 6842 | 22 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 8, sub: 0, line: 73 } |  |  | 0.540 |
| walker |  | 6864 | 22 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 32, sub: 0, line: 259 } |  |  | 0.540 |
| walker |  | 6887 | 23 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 5, sub: 0, line: 48 } |  |  | 0.540 |
| walker |  | 6991 | 104 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.540 |
| walker |  | 7016 | 25 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 1, sub: 0, line: 7 } |  |  | 0.540 |
| walker |  | 7041 | 25 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 2, sub: 0, line: 15 } |  |  | 0.540 |
| walker |  | 7066 | 25 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 7, sub: 0, line: 64 } |  |  | 0.540 |
| walker |  | 7091 | 25 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 33, sub: 0, line: 628 } |  |  | 0.540 |
| ns | 7112 |  | 353 | common/lspLanguageFeatures.ts — the shared provider adapters and LSP conversion helpers | 5.3 |  | 0.532 |
| walker |  | 7145 | 54 | Json::Scripts { file: samples/browser-esm-vite/package.json } |  |  | 0.532 |
| walker |  | 7171 | 26 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 3, sub: 0, line: 26 } |  |  | 0.532 |
| walker |  | 7197 | 26 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 6, sub: 0, line: 57 } |  |  | 0.532 |
| walker |  | 7253 | 56 | Json::Scripts { file: samples/browser-esm-webpack-small/package.json } |  |  | 0.532 |
| walker |  | 7274 | 21 | Code::CodeKey { rung: Names, file: samples/browser-esm-esbuild/index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.532 |
| walker |  | 7315 | 41 | Code::CodeKey { rung: Decl, file: samples/browser-esm-esbuild/index.js, decl: 1, sub: 0, line: 21 } |  |  | 0.532 |
| walker |  | 7336 | 21 | Code::CodeKey { rung: Names, file: samples/browser-esm-parcel/src/index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.532 |
| walker |  | 7377 | 41 | Code::CodeKey { rung: Decl, file: samples/browser-esm-parcel/src/index.js, decl: 1, sub: 0, line: 26 } |  |  | 0.532 |
| walker |  | 7398 | 21 | Code::CodeKey { rung: Names, file: samples/browser-esm-webpack/index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.532 |
| ns | 7419 |  | 307 | Web-worker plumbing: MonacoEnvironment hooks, createWebWorker, IWebWorkerOptions fields, worker entry points | 5.4 |  | 0.522 |
| walker |  | 7439 | 41 | Code::CodeKey { rung: Decl, file: samples/browser-esm-webpack/index.js, decl: 1, sub: 0, line: 21 } |  |  | 0.522 |
| walker |  | 7460 | 21 | Code::CodeKey { rung: Names, file: samples/browser-esm-webpack-small/index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.522 |
| ns | 7574 |  | 155 | WorkerManager lifecycle and the CSSWorker entry points | 5.5 |  | 0.517 |
| walker |  | 7654 | 194 | Code::CodeKey { rung: Decl, file: samples/browser-esm-webpack-small/index.js, decl: 1, sub: 0, line: 152 } |  |  | 0.517 |
| walker |  | 7675 | 21 | Code::CodeKey { rung: Names, file: samples/electron-esm-webpack/index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.517 |
| walker |  | 7716 | 41 | Code::CodeKey { rung: Decl, file: samples/electron-esm-webpack/index.js, decl: 1, sub: 0, line: 21 } |  |  | 0.517 |
| walker |  | 7801 | 85 | Code::CodeKey { rung: Names, file: samples/browser-esm-vite/main.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.517 |
| walker |  | 7814 | 13 | Code::CodeKey { rung: Decl, file: samples/browser-esm-vite/main.ts, decl: 3, sub: 0, line: 18 } |  |  | 0.517 |
| ns | 7828 |  | 254 | typescript/languageFeatures.ts — the TypeScript-specific adapter roster | 5.6 |  | 0.511 |
| walker |  | 7846 | 32 | Code::CodeKey { rung: Decl, file: samples/browser-esm-vite/main.ts, decl: 4, sub: 0, line: 21 } |  |  | 0.511 |
| walker |  | 7892 | 46 | Fs::DirListing { dir: website/src/website } |  |  | 0.511 |
| walker |  | 7907 | 15 | Fs::DirListing { dir: website/src/website/data } |  |  | 0.520 |
| ns | 7907 |  | 79 | monaco-lsp-client package: complete tree listing | 6.1 |  | 0.520 |
| walker |  | 7934 | 27 | Fs::DirListing { dir: website/src/website/pages } |  |  | 0.520 |
| walker |  | 7939 | 5 | Fs::DirListing { dir: website/src/website/pages/home } |  |  | 0.520 |
| walker |  | 7973 | 34 | Fs::DirListing { dir: website/src/website/data/playground-samples } |  |  | 0.520 |
| walker |  | 7992 | 19 | Fs::DirListing { dir: website/src/website/data/playground-samples/customizing-the-appearence } |  |  | 0.520 |
| walker |  | 8008 | 16 | Fs::DirListing { dir: website/src/website/data/playground-samples/customizing-the-appearence/scrollbars } |  |  | 0.520 |
| walker |  | 8026 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/customizing-the-appearence/exposed-colors } |  |  | 0.520 |
| walker |  | 8044 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/customizing-the-appearence/tokens-and-colors } |  |  | 0.520 |
| walker |  | 8065 | 21 | Fs::DirListing { dir: website/src/website/data/playground-samples/creating-the-diffeditor } |  |  | 0.520 |
| walker |  | 8083 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/creating-the-diffeditor/hello-diff-world } |  |  | 0.520 |
| walker |  | 8101 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/creating-the-diffeditor/inline-diff-example } |  |  | 0.520 |
| ns | 8114 |  | 207 | MonacoLspClient wiring + index exports | 6.2 | 6.1 | 0.514 |
| walker |  | 8119 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/creating-the-diffeditor/multi-line-example } |  |  | 0.514 |
| walker |  | 8150 | 31 | Fs::DirListing { dir: website/src/website/data/playground-samples/creating-the-editor } |  |  | 0.514 |
| ns | 8163 |  | 49 | webpack-plugin package: complete tree listing | 6.3 |  | 0.520 |
| walker |  | 8168 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/creating-the-editor/editor-basic-options } |  |  | 0.520 |
| walker |  | 8186 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/creating-the-editor/hard-wrapping } |  |  | 0.520 |
| walker |  | 8204 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/creating-the-editor/hello-world } |  |  | 0.520 |
| walker |  | 8222 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/creating-the-editor/syntax-highlighting-for-html-elements } |  |  | 0.520 |
| walker |  | 8240 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/creating-the-editor/web-component } |  |  | 0.520 |
| walker |  | 8280 | 40 | Fs::DirListing { dir: website/src/website/components } |  |  | 0.520 |
| walker |  | 8292 | 12 | Fs::DirListing { dir: website/src/website/components/monaco } |  |  | 0.520 |
| walker |  | 8334 | 42 | Fs::DirListing { dir: website/src/website/utils } |  |  | 0.520 |
| walker |  | 8368 | 34 | Json::Scripts { file: samples/legacy/electron-amd/package.json } |  |  | 0.520 |
| walker |  | 8402 | 34 | Json::Scripts { file: samples/legacy/electron-amd-nodeIntegration/package.json } |  |  | 0.520 |
| ns | 8447 |  | 284 | webpack-plugin option fields and peer-dependency contract | 6.4 | 6.3 | 0.516 |
| ns | 8611 |  | 164 | build/ and scripts/ tree listings | 7.1 |  | 0.506 |
| ns | 8760 |  | 149 | build-monaco-editor.ts run() — what the published package is made of | 7.2 |  | 0.503 |
| ns | 8998 |  | 238 | check-samples.ts — the invariant every new language must satisfy | 7.3 |  | 0.496 |
| walker |  | 9023 | 621 | Code::CodeKey { rung: Decl, file: webpack-plugin/src/index.ts, decl: 1, sub: 0, line: 102 } |  |  | 0.505 |
| walker |  | 9074 | 51 | Fs::DirListing { dir: test/smoke } |  |  | 0.518 |
| walker |  | 9082 | 8 | Fs::DirListing { dir: test/smoke/esbuild } |  |  | 0.518 |
| walker |  | 9090 | 8 | Fs::DirListing { dir: test/smoke/vite } |  |  | 0.518 |
| walker |  | 9098 | 8 | Fs::DirListing { dir: test/smoke/webpack } |  |  | 0.518 |
| walker |  | 9110 | 12 | Fs::DirListing { dir: test/smoke/parcel } |  |  | 0.518 |
| ns | 9164 |  | 166 | package.json distribution fields: typings, main, module, exports | 8.1 |  | 0.524 |
| walker |  | 9169 | 59 | Json::Scripts { file: samples/electron-esm-webpack/package.json } |  |  | 0.524 |
| walker |  | 9232 | 63 | Json::Scripts { file: samples/browser-esm-webpack-typescript/package.json } |  |  | 0.524 |
| walker |  | 9298 | 66 | Json::Scripts { file: samples/browser-esm-webpack-typescript-react/package.json } |  |  | 0.524 |
| ns | 9418 |  | 254 | README Installing + CHANGELOG head (0.55.x breaking changes) | 8.2 |  | 0.518 |
| walker |  | 9450 | 152 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.520 |
| walker |  | 9478 | 28 | Code::CodeKey { rung: Names, file: monaco-lsp-client/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.520 |
| ns | 9522 |  | 104 | samples/ listing — every integration sample directory | 8.3 |  | 0.527 |
| walker |  | 9545 | 67 | Fs::DirListing { dir: website/src/website/pages/playground } |  |  | 0.527 |
| ns | 9560 |  | 38 | docs/ listing | 8.4 |  | 0.529 |
| walker |  | 9620 | 75 | Fs::DirListing { dir: website/src/website/data/playground-samples/extending-language-services } |  |  | 0.529 |
| ns | 9643 |  | 83 | integrate-esm.md section headings | 8.5 | 8.4 | 0.527 |
| walker |  | 9707 | 87 | Json::Scripts { file: samples/browser-esm-vite-react/package.json } |  |  | 0.527 |
| ns | 9769 |  | 126 | website/ and CI/publishing config listings | 8.6 |  | 0.538 |
| walker |  | 9781 | 74 | Fs::DirListing { dir: website/src/website/data/playground-samples/interacting-with-the-editor } |  |  | 0.538 |
| walker |  | 9797 | 16 | Fs::DirListing { dir: website/src/website/data/playground-samples/interacting-with-the-editor/line-and-inline-decorations } |  |  | 0.538 |
| walker |  | 9813 | 16 | Fs::DirListing { dir: website/src/website/data/playground-samples/interacting-with-the-editor/listening-to-mouse-events } |  |  | 0.538 |
| walker |  | 9829 | 16 | Fs::DirListing { dir: website/src/website/data/playground-samples/interacting-with-the-editor/rendering-glyphs-in-the-margin } |  |  | 0.538 |
| walker |  | 9847 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/interacting-with-the-editor/adding-a-command-to-an-editor-instance } |  |  | 0.538 |
| walker |  | 9865 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/interacting-with-the-editor/adding-a-keybinding-to-an-existing-command } |  |  | 0.538 |
| walker |  | 9883 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/interacting-with-the-editor/adding-an-action-to-an-editor-instance } |  |  | 0.538 |
| walker |  | 9901 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/interacting-with-the-editor/customizing-the-line-numbers } |  |  | 0.538 |
| walker |  | 9919 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/interacting-with-the-editor/listening-to-key-events } |  |  | 0.538 |
| walker |  | 9937 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/interacting-with-the-editor/revealing-a-position } |  |  | 0.538 |
| walker |  | 9999 | 62 | Fs::DirListing { dir: monaco-lsp-client/src/adapters/languageFeatures } |  |  | 0.538 |

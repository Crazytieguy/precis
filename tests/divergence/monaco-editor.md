Score(3000)=0.712 I=0.803 C=0.632 ns_rows≤3K=23/56 grid(1000/1442/2080/3000/4327/6240/9000)=0.654/0.694/0.782/0.712/0.639/0.580/0.528

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
| walker |  | 2424 | 16 | Json::Identity { file: samples/legacy/electron-amd/package.json } |  |  | 0.770 |
| ns | 2429 |  | 282 | _.contribution.ts: ILang/ILangImpl and registerLanguage() | 2.8 |  | 0.734 |
| walker |  | 2452 | 28 | Json::Entry { file: webpack-plugin/package.json } |  |  | 0.734 |
| walker |  | 2486 | 34 | Json::Dependencies { file: webpack-plugin/package.json } |  |  | 0.734 |
| walker |  | 2522 | 36 | Json::Dependencies { file: samples/package.json } |  |  | 0.734 |
| walker |  | 2561 | 39 | Json::Scripts { file: samples/package.json } |  |  | 0.734 |
| ns | 2568 |  | 139 | _.contribution.ts: LazyLanguageLoader and loadLanguage() | 2.9 |  | 0.718 |
| ns | 2657 |  | 89 | Grammar file shape: rust.ts conf + language exports | 2.10 |  | 0.707 |
| walker |  | 2707 | 146 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.747 |
| walker |  | 2720 | 13 | Json::Entry { file: samples/legacy/electron-amd/package.json } |  |  | 0.747 |
| walker |  | 2763 | 43 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.747 |
| ns | 2797 |  | 140 | Tokenization test harness: testRunner.ts types + testTokenization(), and a test file head | 2.11 |  | 0.730 |
| ns | 2839 |  | 42 | src/languages/definitions/register.all.ts head, elided middle, tail | 2.12 |  | 0.724 |
| walker |  | 2843 | 80 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.724 |
| walker |  | 2903 | 60 | Fs::DirListing { dir: website } |  |  | 0.725 |
| walker |  | 2911 | 8 | Fs::DirListing { dir: website/static } |  |  | 0.725 |
| walker |  | 2920 | 9 | Fs::DirListing { dir: website/scripts } |  |  | 0.725 |
| walker |  | 2935 | 15 | Fs::DirListing { dir: website/static/monarch } |  |  | 0.725 |
| ns | 2974 |  | 135 | src/deprecated/: the legacy import paths, all one-line re-exports | 2.13 |  | 0.712 |
| walker |  | 3084 | 149 | Markdown::CommandBlock { file: CONTRIBUTING.md, row: 91 } |  |  | 0.714 |
| walker |  | 3107 | 23 | Fs::DirListing { dir: scripts/ci } |  |  | 0.714 |
| ns | 3343 |  | 369 | package.json scripts — build, test and dev commands (packaging variants elided) | 3.1 |  | 0.693 |
| walker |  | 3401 | 294 | Code::CodeKey { rung: Names, file: monaco-lsp-client/generator/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.693 |
| walker |  | 3412 | 11 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 9, sub: 0, line: 82 } |  |  | 0.693 |
| walker |  | 3430 | 18 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 32, sub: 0, line: 259 } |  |  | 0.693 |
| walker |  | 3451 | 21 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 21, sub: 0, line: 197 } |  |  | 0.693 |
| walker |  | 3472 | 21 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 22, sub: 0, line: 202 } |  |  | 0.693 |
| ns | 3475 |  | 132 | CONTRIBUTING: the ordered editor-test command sequence | 3.2 |  | 0.698 |
| walker |  | 3493 | 21 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 24, sub: 0, line: 213 } |  |  | 0.698 |
| walker |  | 3514 | 21 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 25, sub: 0, line: 218 } |  |  | 0.698 |
| walker |  | 3535 | 21 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 26, sub: 0, line: 223 } |  |  | 0.698 |
| ns | 3539 |  | 64 | test/ listing plus the full smoke-test directory | 3.3 |  | 0.684 |
| walker |  | 3557 | 22 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 27, sub: 0, line: 228 } |  |  | 0.684 |
| walker |  | 3579 | 22 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 29, sub: 0, line: 241 } |  |  | 0.684 |
| walker |  | 3601 | 22 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 30, sub: 0, line: 246 } |  |  | 0.684 |
| walker |  | 3625 | 24 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 31, sub: 0, line: 251 } |  |  | 0.684 |
| walker |  | 3649 | 24 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 34, sub: 0, line: 684 } |  |  | 0.684 |
| walker |  | 3678 | 29 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 23, sub: 0, line: 207 } |  |  | 0.684 |
| ns | 3690 |  | 151 | Listings of the five src/languages/features dirs — the uniform rich-service file set | 4.1 |  | 0.682 |
| walker |  | 3709 | 31 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 16, sub: 0, line: 154 } |  |  | 0.682 |
| walker |  | 3756 | 47 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 28, sub: 0, line: 233 } |  |  | 0.682 |
| walker |  | 3811 | 55 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 17, sub: 0, line: 159 } |  |  | 0.682 |
| walker |  | 3868 | 57 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 15, sub: 0, line: 145 } |  |  | 0.682 |
| walker |  | 3928 | 60 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 8, sub: 0, line: 73 } |  |  | 0.682 |
| ns | 3950 |  | 260 | css/register.ts — every top-level export (names only) | 4.2 |  | 0.665 |
| walker |  | 3992 | 64 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 13, sub: 0, line: 124 } |  |  | 0.665 |
| walker |  | 4066 | 74 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 12, sub: 0, line: 113 } |  |  | 0.665 |
| walker |  | 4143 | 77 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 14, sub: 0, line: 134 } |  |  | 0.665 |
| walker |  | 4222 | 79 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 1, sub: 0, line: 7 } |  |  | 0.665 |
| ns | 4235 |  | 285 | css: the default ModeConfiguration toggles, the three defaults objects, and the lazy onLanguage hookup | 4.3 | 4.2 | 0.639 |
| walker |  | 4309 | 87 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 11, sub: 0, line: 101 } |  |  | 0.639 |
| walker |  | 4410 | 101 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 19, sub: 0, line: 170 } |  |  | 0.639 |
| ns | 4494 |  | 259 | json/register.ts — every top-level export (names only) | 4.4 |  | 0.624 |
| walker |  | 4524 | 114 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 10, sub: 0, line: 86 } |  |  | 0.624 |
| walker |  | 4636 | 112 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 20, sub: 0, line: 183 } |  |  | 0.624 |
| walker |  | 4643 | 7 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 34, sub: 0, line: 684 } |  |  | 0.624 |
| walker |  | 4673 | 30 | Json::Scripts { file: samples/browser-esm-esbuild/package.json } |  |  | 0.624 |
| walker |  | 4697 | 24 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.624 |
| walker |  | 4728 | 31 | Json::Scripts { file: samples/browser-esm-parcel/package.json } |  |  | 0.624 |
| walker |  | 4763 | 35 | Fs::DirListing { dir: src/languages/features/typescript/lib } |  |  | 0.618 |
| ns | 4763 |  | 269 | json defaults: diagnostic option values, mode toggles, jsonDefaults, the worker interface | 4.5 | 4.4 | 0.618 |
| walker |  | 4775 | 12 | Code::CodeKey { rung: Names, file: samples/browser-esm-vite-react/src/main.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.618 |
| walker |  | 4819 | 44 | Code::CodeKey { rung: Decl, file: samples/browser-esm-vite-react/src/main.tsx, decl: 1, sub: 0, line: 6 } |  |  | 0.618 |
| walker |  | 4831 | 12 | Code::CodeKey { rung: Names, file: samples/browser-esm-webpack-typescript-react/src/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.618 |
| walker |  | 4875 | 44 | Code::CodeKey { rung: Decl, file: samples/browser-esm-webpack-typescript-react/src/index.tsx, decl: 1, sub: 0, line: 7 } |  |  | 0.618 |
| walker |  | 4901 | 26 | Fs::DirListing { dir: website/src } |  |  | 0.619 |
| walker |  | 4913 | 12 | Fs::DirListing { dir: website/src/runner } |  |  | 0.619 |
| walker |  | 4982 | 69 | Json::Scripts { file: webpack-plugin/package.json } |  |  | 0.619 |
| ns | 5025 |  | 262 | html/register.ts — export names plus the registerHTMLLanguageService factory | 4.6 |  | 0.605 |
| walker |  | 5150 | 168 | Json::Entry { file: package.json } |  |  | 0.605 |
| walker |  | 5188 | 38 | Json::Scripts { file: samples/browser-esm-webpack/package.json } |  |  | 0.605 |
| walker |  | 5226 | 38 | Json::Scripts { file: samples/browser-esm-webpack-monaco-plugin/package.json } |  |  | 0.605 |
| ns | 5309 |  | 284 | html: the three pre-registered services and their per-language toggles | 4.7 | 4.6 | 0.590 |
| walker |  | 5328 | 102 | Code::CodeKey { rung: Names, file: samples/electron-esm-webpack/main.js, decl: 0, sub: 0, line: 0 } |  |  | 0.590 |
| walker |  | 5357 | 29 | Code::CodeKey { rung: Decl, file: samples/electron-esm-webpack/main.js, decl: 7, sub: 0, line: 28 } |  |  | 0.590 |
| walker |  | 5388 | 31 | Code::CodeKey { rung: Decl, file: samples/electron-esm-webpack/main.js, decl: 6, sub: 0, line: 22 } |  |  | 0.590 |
| walker |  | 5490 | 102 | Code::CodeKey { rung: Names, file: samples/legacy/electron-amd/main.js, decl: 0, sub: 0, line: 0 } |  |  | 0.590 |
| walker |  | 5519 | 29 | Code::CodeKey { rung: Decl, file: samples/legacy/electron-amd/main.js, decl: 7, sub: 0, line: 28 } |  |  | 0.590 |
| walker |  | 5550 | 31 | Code::CodeKey { rung: Decl, file: samples/legacy/electron-amd/main.js, decl: 6, sub: 0, line: 22 } |  |  | 0.590 |
| ns | 5554 |  | 245 | typescript/register.ts — every top-level export (names only) | 4.8 |  | 0.579 |
| walker |  | 5728 | 178 | Json::Scripts { file: package.json } |  |  | 0.584 |
| ns | 5789 |  | 235 | LanguageServiceDefaults (typescript) — all members | 4.9 |  | 0.573 |
| walker |  | 5880 | 152 | Json::ScriptsTail { file: package.json, chunk: 1 } |  |  | 0.573 |
| ns | 6025 |  | 236 | TypeScriptWorker — all 21 proxy methods (names only) | 4.10 | 4.8 | 0.562 |
| walker |  | 6032 | 152 | Json::ScriptsTail { file: package.json, chunk: 2 } |  |  | 0.565 |
| walker |  | 6197 | 165 | Json::ScriptsTail { file: package.json, chunk: 3 } |  |  | 0.580 |
| walker |  | 6241 | 44 | Code::CodeKey { rung: Names, file: webpack-plugin/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.580 |
| walker |  | 6287 | 46 | Code::CodeKey { rung: Decl, file: webpack-plugin/src/index.ts, decl: 2, sub: 0, line: 159 } |  |  | 0.580 |
| ns | 6316 |  | 291 | typescript/javascript defaults and their divergence | 4.11 | 4.8 | 0.566 |
| walker |  | 6368 | 81 | Json::Dependencies { file: monaco-lsp-client/package.json } |  |  | 0.566 |
| walker |  | 6398 | 30 | Code::CodeKey { rung: Names, file: src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.568 |
| walker |  | 6429 | 31 | Code::CodeKey { rung: Names, file: samples/browser-esm-webpack-monaco-plugin/index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.568 |
| walker |  | 6469 | 40 | Code::CodeKey { rung: Decl, file: samples/browser-esm-webpack-monaco-plugin/index.js, decl: 2, sub: 0, line: 12 } |  |  | 0.568 |
| ns | 6545 |  | 229 | The four *Mode.ts entry points and the tsMode re-export tail | 5.1 |  | 0.563 |
| walker |  | 6553 | 84 | Code::CodeKey { rung: Decl, file: samples/browser-esm-webpack-monaco-plugin/index.js, decl: 1, sub: 0, line: 3 } |  |  | 0.563 |
| walker |  | 6635 | 82 | Json::Scripts { file: monaco-lsp-client/package.json } |  |  | 0.563 |
| walker |  | 6651 | 16 | Code::CodeKey { rung: Names, file: samples/browser-esm-webpack-typescript/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.563 |
| walker |  | 6692 | 41 | Code::CodeKey { rung: Decl, file: samples/browser-esm-webpack-typescript/src/index.ts, decl: 1, sub: 0, line: 23 } |  |  | 0.563 |
| ns | 6759 |  | 214 | cssMode.setupMode — WorkerManager, the worker accessor, and ModeConfiguration-gated provider registration | 5.2 | 5.1 | 0.552 |
| walker |  | 6796 | 104 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.552 |
| walker |  | 6850 | 54 | Json::Scripts { file: samples/browser-esm-vite/package.json } |  |  | 0.552 |
| walker |  | 6872 | 22 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 4, sub: 0, line: 41 } |  |  | 0.552 |
| walker |  | 6894 | 22 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 8, sub: 0, line: 73 } |  |  | 0.552 |
| walker |  | 6916 | 22 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 32, sub: 0, line: 259 } |  |  | 0.552 |
| walker |  | 6972 | 56 | Json::Scripts { file: samples/browser-esm-webpack-small/package.json } |  |  | 0.552 |
| walker |  | 6993 | 21 | Code::CodeKey { rung: Names, file: samples/browser-esm-esbuild/index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.552 |
| walker |  | 7034 | 41 | Code::CodeKey { rung: Decl, file: samples/browser-esm-esbuild/index.js, decl: 1, sub: 0, line: 21 } |  |  | 0.552 |
| walker |  | 7055 | 21 | Code::CodeKey { rung: Names, file: samples/browser-esm-parcel/src/index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.552 |
| walker |  | 7096 | 41 | Code::CodeKey { rung: Decl, file: samples/browser-esm-parcel/src/index.js, decl: 1, sub: 0, line: 26 } |  |  | 0.552 |
| ns | 7112 |  | 353 | common/lspLanguageFeatures.ts — the shared provider adapters and LSP conversion helpers | 5.3 |  | 0.544 |
| walker |  | 7117 | 21 | Code::CodeKey { rung: Names, file: samples/browser-esm-webpack/index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.544 |
| walker |  | 7158 | 41 | Code::CodeKey { rung: Decl, file: samples/browser-esm-webpack/index.js, decl: 1, sub: 0, line: 21 } |  |  | 0.544 |
| walker |  | 7179 | 21 | Code::CodeKey { rung: Names, file: samples/browser-esm-webpack-small/index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.544 |
| walker |  | 7373 | 194 | Code::CodeKey { rung: Decl, file: samples/browser-esm-webpack-small/index.js, decl: 1, sub: 0, line: 152 } |  |  | 0.544 |
| walker |  | 7394 | 21 | Code::CodeKey { rung: Names, file: samples/electron-esm-webpack/index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.544 |
| ns | 7419 |  | 307 | Web-worker plumbing: MonacoEnvironment hooks, createWebWorker, IWebWorkerOptions fields, worker entry points | 5.4 |  | 0.533 |
| walker |  | 7435 | 41 | Code::CodeKey { rung: Decl, file: samples/electron-esm-webpack/index.js, decl: 1, sub: 0, line: 21 } |  |  | 0.533 |
| walker |  | 7458 | 23 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 5, sub: 0, line: 48 } |  |  | 0.533 |
| walker |  | 7543 | 85 | Code::CodeKey { rung: Names, file: samples/browser-esm-vite/main.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.533 |
| walker |  | 7556 | 13 | Code::CodeKey { rung: Decl, file: samples/browser-esm-vite/main.ts, decl: 3, sub: 0, line: 18 } |  |  | 0.533 |
| ns | 7574 |  | 155 | WorkerManager lifecycle and the CSSWorker entry points | 5.5 |  | 0.529 |
| walker |  | 7588 | 32 | Code::CodeKey { rung: Decl, file: samples/browser-esm-vite/main.ts, decl: 4, sub: 0, line: 21 } |  |  | 0.529 |
| walker |  | 7634 | 46 | Fs::DirListing { dir: website/src/website } |  |  | 0.529 |
| walker |  | 7649 | 15 | Fs::DirListing { dir: website/src/website/data } |  |  | 0.529 |
| walker |  | 7676 | 27 | Fs::DirListing { dir: website/src/website/pages } |  |  | 0.529 |
| walker |  | 7681 | 5 | Fs::DirListing { dir: website/src/website/pages/home } |  |  | 0.529 |
| walker |  | 7715 | 34 | Fs::DirListing { dir: website/src/website/data/playground-samples } |  |  | 0.529 |
| walker |  | 7734 | 19 | Fs::DirListing { dir: website/src/website/data/playground-samples/customizing-the-appearence } |  |  | 0.529 |
| walker |  | 7750 | 16 | Fs::DirListing { dir: website/src/website/data/playground-samples/customizing-the-appearence/scrollbars } |  |  | 0.529 |
| walker |  | 7768 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/customizing-the-appearence/exposed-colors } |  |  | 0.529 |
| walker |  | 7786 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/customizing-the-appearence/tokens-and-colors } |  |  | 0.529 |
| walker |  | 7807 | 21 | Fs::DirListing { dir: website/src/website/data/playground-samples/creating-the-diffeditor } |  |  | 0.529 |
| walker |  | 7825 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/creating-the-diffeditor/hello-diff-world } |  |  | 0.529 |
| ns | 7828 |  | 254 | typescript/languageFeatures.ts — the TypeScript-specific adapter roster | 5.6 |  | 0.523 |
| walker |  | 7843 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/creating-the-diffeditor/inline-diff-example } |  |  | 0.523 |
| walker |  | 7861 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/creating-the-diffeditor/multi-line-example } |  |  | 0.523 |
| walker |  | 7892 | 31 | Fs::DirListing { dir: website/src/website/data/playground-samples/creating-the-editor } |  |  | 0.523 |
| ns | 7907 |  | 79 | monaco-lsp-client package: complete tree listing | 6.1 |  | 0.531 |
| walker |  | 7910 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/creating-the-editor/editor-basic-options } |  |  | 0.531 |
| walker |  | 7928 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/creating-the-editor/hard-wrapping } |  |  | 0.531 |
| walker |  | 7946 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/creating-the-editor/hello-world } |  |  | 0.531 |
| walker |  | 7964 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/creating-the-editor/syntax-highlighting-for-html-elements } |  |  | 0.531 |
| walker |  | 7982 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/creating-the-editor/web-component } |  |  | 0.531 |
| walker |  | 8022 | 40 | Fs::DirListing { dir: website/src/website/components } |  |  | 0.531 |
| walker |  | 8034 | 12 | Fs::DirListing { dir: website/src/website/components/monaco } |  |  | 0.531 |
| walker |  | 8076 | 42 | Fs::DirListing { dir: website/src/website/utils } |  |  | 0.531 |
| walker |  | 8110 | 34 | Json::Scripts { file: samples/legacy/electron-amd/package.json } |  |  | 0.531 |
| ns | 8114 |  | 207 | MonacoLspClient wiring + index exports | 6.2 | 6.1 | 0.525 |
| ns | 8163 |  | 49 | webpack-plugin package: complete tree listing | 6.3 |  | 0.531 |
| ns | 8447 |  | 284 | webpack-plugin option fields and peer-dependency contract | 6.4 | 6.3 | 0.526 |
| ns | 8611 |  | 164 | build/ and scripts/ tree listings | 7.1 |  | 0.517 |
| walker |  | 8731 | 621 | Code::CodeKey { rung: Decl, file: webpack-plugin/src/index.ts, decl: 1, sub: 0, line: 102 } |  |  | 0.526 |
| ns | 8760 |  | 149 | build-monaco-editor.ts run() — what the published package is made of | 7.2 |  | 0.522 |
| walker |  | 8782 | 51 | Fs::DirListing { dir: test/smoke } |  |  | 0.536 |
| walker |  | 8790 | 8 | Fs::DirListing { dir: test/smoke/esbuild } |  |  | 0.536 |
| walker |  | 8798 | 8 | Fs::DirListing { dir: test/smoke/vite } |  |  | 0.536 |
| walker |  | 8806 | 8 | Fs::DirListing { dir: test/smoke/webpack } |  |  | 0.536 |
| walker |  | 8818 | 12 | Fs::DirListing { dir: test/smoke/parcel } |  |  | 0.536 |
| walker |  | 8877 | 59 | Json::Scripts { file: samples/electron-esm-webpack/package.json } |  |  | 0.536 |
| walker |  | 8902 | 25 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 1, sub: 0, line: 7 } |  |  | 0.536 |
| walker |  | 8927 | 25 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 2, sub: 0, line: 15 } |  |  | 0.536 |
| walker |  | 8952 | 25 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 7, sub: 0, line: 64 } |  |  | 0.536 |
| walker |  | 8977 | 25 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 33, sub: 0, line: 628 } |  |  | 0.536 |
| ns | 8998 |  | 238 | check-samples.ts — the invariant every new language must satisfy | 7.3 |  | 0.528 |
| walker |  | 9040 | 63 | Json::Scripts { file: samples/browser-esm-webpack-typescript/package.json } |  |  | 0.528 |
| walker |  | 9066 | 26 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 3, sub: 0, line: 26 } |  |  | 0.528 |
| walker |  | 9092 | 26 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 6, sub: 0, line: 57 } |  |  | 0.528 |
| walker |  | 9158 | 66 | Json::Scripts { file: samples/browser-esm-webpack-typescript-react/package.json } |  |  | 0.528 |
| ns | 9164 |  | 166 | package.json distribution fields: typings, main, module, exports | 8.1 |  | 0.533 |
| walker |  | 9310 | 152 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.533 |
| walker |  | 9338 | 28 | Code::CodeKey { rung: Names, file: monaco-lsp-client/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.533 |
| walker |  | 9405 | 67 | Fs::DirListing { dir: website/src/website/pages/playground } |  |  | 0.533 |
| ns | 9418 |  | 254 | README Installing + CHANGELOG head (0.55.x breaking changes) | 8.2 |  | 0.530 |
| walker |  | 9480 | 75 | Fs::DirListing { dir: website/src/website/data/playground-samples/extending-language-services } |  |  | 0.530 |
| ns | 9522 |  | 104 | samples/ listing — every integration sample directory | 8.3 |  | 0.536 |
| ns | 9560 |  | 38 | docs/ listing | 8.4 |  | 0.539 |
| walker |  | 9567 | 87 | Json::Scripts { file: samples/browser-esm-vite-react/package.json } |  |  | 0.539 |
| walker |  | 9641 | 74 | Fs::DirListing { dir: website/src/website/data/playground-samples/interacting-with-the-editor } |  |  | 0.539 |
| ns | 9643 |  | 83 | integrate-esm.md section headings | 8.5 | 8.4 | 0.537 |
| walker |  | 9657 | 16 | Fs::DirListing { dir: website/src/website/data/playground-samples/interacting-with-the-editor/line-and-inline-decorations } |  |  | 0.537 |
| walker |  | 9673 | 16 | Fs::DirListing { dir: website/src/website/data/playground-samples/interacting-with-the-editor/listening-to-mouse-events } |  |  | 0.537 |
| walker |  | 9689 | 16 | Fs::DirListing { dir: website/src/website/data/playground-samples/interacting-with-the-editor/rendering-glyphs-in-the-margin } |  |  | 0.537 |
| walker |  | 9707 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/interacting-with-the-editor/adding-a-command-to-an-editor-instance } |  |  | 0.537 |
| walker |  | 9725 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/interacting-with-the-editor/adding-a-keybinding-to-an-existing-command } |  |  | 0.537 |
| walker |  | 9743 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/interacting-with-the-editor/adding-an-action-to-an-editor-instance } |  |  | 0.537 |
| walker |  | 9761 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/interacting-with-the-editor/customizing-the-line-numbers } |  |  | 0.537 |
| ns | 9769 |  | 126 | website/ and CI/publishing config listings | 8.6 |  | 0.547 |
| walker |  | 9779 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/interacting-with-the-editor/listening-to-key-events } |  |  | 0.547 |
| walker |  | 9797 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/interacting-with-the-editor/revealing-a-position } |  |  | 0.547 |
| walker |  | 9971 | 174 | Fs::DirListing { dir: monaco-lsp-client/src/adapters/languageFeatures } |  |  | 0.547 |
| walker |  | 9997 | 26 | Fs::DirListing { dir: test/manual } |  |  | 0.547 |

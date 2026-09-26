Score(3000)=0.715 I=0.808 C=0.632 ns_rows≤3K=23/56 grid(1000/1442/2080/3000/4327/6240/9000)=0.639/0.696/0.784/0.715/0.655/0.560/0.512

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 67 |  | 67 | README title + one-sentence identity | 1.1 |  | 0.000 |
| ns | 92 |  | 25 | src/ listing — every child of the package source directory | 1.2 |  | 0.000 |
| walker |  | 116 | 116 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 141 | 25 | Fs::DirListing { dir: src } |  |  | 0.801 |
| walker |  | 152 | 11 | Fs::DirListing { dir: src/languages } |  |  | 0.802 |
| walker |  | 173 | 21 | Fs::DirListing { dir: src/languages/features } |  |  | 0.808 |
| walker |  | 205 | 32 | Fs::DirListing { dir: src/languages/features/typescript } |  |  | 0.809 |
| walker |  | 211 | 6 | Fs::DirListing { dir: test-results } |  |  | 0.809 |
| ns | 216 |  | 124 | src/index.ts — the whole package entry point (9 lines) | 1.3 |  | 0.515 |
| walker |  | 218 | 7 | Fs::DirListing { dir: src/languages/features/common } |  |  | 0.515 |
| walker |  | 285 | 67 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.707 |
| walker |  | 294 | 9 | Fs::DirListing { dir: src/internal/common } |  |  | 0.707 |
| walker |  | 305 | 11 | Fs::DirListing { dir: src/deprecated } |  |  | 0.707 |
| walker |  | 312 | 7 | Fs::DirListing { dir: src/deprecated/basic-languages } |  |  | 0.707 |
| walker |  | 322 | 10 | Fs::DirListing { dir: src/deprecated/editor } |  |  | 0.707 |
| ns | 332 |  | 116 | Complete repository root listing | 1.4 |  | 0.849 |
| walker |  | 335 | 13 | Fs::DirListing { dir: src/deprecated/language } |  |  | 0.856 |
| walker |  | 347 | 12 | Fs::DirListing { dir: src/deprecated/language/css } |  |  | 0.856 |
| walker |  | 359 | 12 | Fs::DirListing { dir: src/deprecated/language/html } |  |  | 0.856 |
| walker |  | 371 | 12 | Fs::DirListing { dir: src/deprecated/language/json } |  |  | 0.856 |
| walker |  | 383 | 12 | Fs::DirListing { dir: src/deprecated/language/typescript } |  |  | 0.856 |
| walker |  | 407 | 24 | Fs::DirListing { dir: src/languages/features/css } |  |  | 0.857 |
| walker |  | 431 | 24 | Fs::DirListing { dir: src/languages/features/html } |  |  | 0.858 |
| walker |  | 458 | 27 | Fs::DirListing { dir: monaco-lsp-client } |  |  | 0.858 |
| walker |  | 462 | 4 | Fs::DirListing { dir: monaco-lsp-client/generator } |  |  | 0.858 |
| ns | 471 |  | 139 | package.json identity header (name, version, vscodeRef, license) | 1.5 |  | 0.776 |
| walker |  | 477 | 15 | Fs::DirListing { dir: monaco-lsp-client/src } |  |  | 0.777 |
| walker |  | 506 | 29 | Fs::DirListing { dir: src/languages/features/json } |  |  | 0.779 |
| walker |  | 535 | 29 | Fs::DirListing { dir: webpack-plugin } |  |  | 0.779 |
| walker |  | 555 | 20 | Fs::DirListing { dir: webpack-plugin/src } |  |  | 0.780 |
| walker |  | 559 | 4 | Fs::DirListing { dir: webpack-plugin/src/loaders } |  |  | 0.780 |
| walker |  | 564 | 5 | Fs::DirListing { dir: .devcontainer } |  |  | 0.780 |
| walker |  | 569 | 5 | Fs::DirListing { dir: .husky } |  |  | 0.780 |
| walker |  | 577 | 8 | Fs::DirListing { dir: webpack-plugin/src/plugins } |  |  | 0.780 |
| ns | 588 |  | 117 | The three one-to-six-line re-export files src/index.ts depends on | 1.6 | 1.2 | 0.716 |
| walker |  | 615 | 38 | Fs::DirListing { dir: docs } |  |  | 0.716 |
| walker |  | 621 | 6 | Fs::DirListing { dir: scripts } |  |  | 0.716 |
| walker |  | 691 | 70 | Json::Identity { file: package.json } |  |  | 0.740 |
| ns | 734 |  | 146 | All README section headings (locations) | 1.7 |  | 0.656 |
| walker |  | 752 | 61 | Json::Scripts { file: package.json } |  |  | 0.656 |
| walker |  | 787 | 35 | Fs::DirListing { dir: src/languages/features/typescript/lib } |  |  | 0.658 |
| walker |  | 791 | 4 | Fs::DirListing { dir: scripts/lib } |  |  | 0.658 |
| walker |  | 839 | 48 | Json::Identity { file: monaco-lsp-client/package.json } |  |  | 0.658 |
| ns | 927 |  | 193 | CONTRIBUTING 'Source Code Structure' — the monaco-editor-core boundary | 1.8 |  | 0.627 |
| ns | 1097 |  | 170 | README Concepts: Models and URIs | 1.9 | 1.7 | 0.619 |
| walker |  | 1111 | 272 | Fs::DirListing { dir: src/features } |  |  | 0.651 |
| ns | 1259 |  | 162 | README Concepts: Editors, Providers, Disposables | 1.10 | 1.7 | 0.638 |
| ns | 1324 |  | 65 | Listings of every src/ subtree except the two huge uniform ones | 2.1 |  | 0.687 |
| walker |  | 1415 | 304 | Fs::DirListing { dir: src/languages/definitions } |  |  | 0.714 |
| walker |  | 1427 | 12 | Fs::DirListing { dir: .azure-pipelines } |  |  | 0.714 |
| ns | 1430 |  | 106 | Uniform shape of a feature shim (hover, gpu) and the one that is not uniform (find) | 2.2 |  | 0.696 |
| walker |  | 1439 | 12 | Fs::DirListing { dir: .vscode } |  |  | 0.696 |
| walker |  | 1501 | 62 | Json::Identity { file: webpack-plugin/package.json } |  |  | 0.696 |
| walker |  | 1514 | 13 | Fs::DirListing { dir: test } |  |  | 0.696 |
| walker |  | 1618 | 104 | Fs::DirListing { dir: samples } |  |  | 0.697 |
| walker |  | 1622 | 4 | Fs::DirListing { dir: samples/nwjs-amd } |  |  | 0.697 |
| walker |  | 1626 | 4 | Fs::DirListing { dir: samples/nwjs-amd-v2 } |  |  | 0.697 |
| walker |  | 1650 | 24 | Fs::DirListing { dir: samples/browser-esm-esbuild } |  |  | 0.697 |
| walker |  | 1675 | 25 | Fs::DirListing { dir: samples/browser-esm-webpack } |  |  | 0.697 |
| walker |  | 1700 | 25 | Fs::DirListing { dir: samples/browser-esm-webpack-monaco-plugin } |  |  | 0.697 |
| ns | 1702 |  | 272 | Complete listing of src/features/ — all 65 editor-feature shim directories | 2.3 |  | 0.771 |
| walker |  | 1725 | 25 | Fs::DirListing { dir: samples/electron-esm-webpack } |  |  | 0.771 |
| ns | 1746 |  | 44 | src/features/register.all.ts head, elided middle, tail | 2.4 |  | 0.759 |
| walker |  | 1756 | 31 | Fs::DirListing { dir: samples/browser-esm-webpack-small } |  |  | 0.759 |
| walker |  | 1771 | 15 | Json::Identity { file: samples/browser-esm-esbuild/package.json } |  |  | 0.759 |
| walker |  | 1788 | 17 | Json::Identity { file: samples/browser-esm-webpack/package.json } |  |  | 0.759 |
| walker |  | 1805 | 17 | Json::Identity { file: samples/electron-esm-webpack/package.json } |  |  | 0.759 |
| walker |  | 1826 | 21 | Fs::DirListing { dir: samples/browser-esm-parcel } |  |  | 0.759 |
| walker |  | 1834 | 8 | Fs::DirListing { dir: samples/browser-esm-parcel/src } |  |  | 0.759 |
| walker |  | 1849 | 15 | Json::Identity { file: samples/browser-esm-parcel/package.json } |  |  | 0.759 |
| walker |  | 1867 | 18 | Json::Identity { file: samples/browser-esm-webpack-small/package.json } |  |  | 0.759 |
| walker |  | 1889 | 22 | Fs::DirListing { dir: samples/browser-esm-webpack-typescript } |  |  | 0.759 |
| walker |  | 1897 | 8 | Fs::DirListing { dir: samples/browser-esm-webpack-typescript/src } |  |  | 0.759 |
| walker |  | 1917 | 20 | Json::Identity { file: samples/browser-esm-webpack-monaco-plugin/package.json } |  |  | 0.759 |
| walker |  | 1937 | 20 | Json::Identity { file: samples/browser-esm-webpack-typescript/package.json } |  |  | 0.759 |
| walker |  | 1964 | 27 | Fs::DirListing { dir: samples/browser-esm-webpack-typescript-react } |  |  | 0.759 |
| walker |  | 1980 | 16 | Fs::DirListing { dir: samples/browser-esm-webpack-typescript-react/src } |  |  | 0.759 |
| walker |  | 1985 | 5 | Fs::DirListing { dir: samples/browser-esm-webpack-typescript-react/src/components } |  |  | 0.759 |
| walker |  | 2005 | 20 | Json::Identity { file: samples/browser-esm-webpack-typescript-react/package.json } |  |  | 0.759 |
| walker |  | 2036 | 31 | Fs::DirListing { dir: samples/browser-esm-vite-react } |  |  | 0.759 |
| ns | 2050 |  | 304 | Complete listing of src/languages/definitions/ — all 84 Monarch languages plus _.contribution.ts, register.all.ts and test/ | 2.5 |  | 0.789 |
| walker |  | 2055 | 19 | Json::Identity { file: samples/browser-esm-vite-react/package.json } |  |  | 0.789 |
| ns | 2063 |  | 13 | One language definition directory, listed in full (rust) | 2.6 |  | 0.784 |
| walker |  | 2091 | 36 | Fs::DirListing { dir: samples/browser-esm-vite } |  |  | 0.784 |
| walker |  | 2109 | 18 | Json::Identity { file: samples/browser-esm-vite/package.json } |  |  | 0.784 |
| walker |  | 2128 | 19 | Fs::DirListing { dir: samples/browser-esm-vite-react/src } |  |  | 0.784 |
| walker |  | 2138 | 10 | Fs::DirListing { dir: samples/browser-esm-vite-react/src/components } |  |  | 0.784 |
| ns | 2147 |  | 84 | rust/register.ts registerLanguage call | 2.7 | 2.6 | 0.771 |
| walker |  | 2209 | 71 | Json::Identity { file: samples/package.json } |  |  | 0.771 |
| walker |  | 2221 | 12 | Json::Entry { file: samples/package.json } |  |  | 0.771 |
| walker |  | 2232 | 11 | Json::Dependencies { file: samples/browser-esm-vite/package.json } |  |  | 0.771 |
| walker |  | 2243 | 11 | Json::Dependencies { file: samples/browser-esm-vite-react/package.json } |  |  | 0.771 |
| walker |  | 2254 | 11 | Json::Dependencies { file: samples/browser-esm-webpack-typescript-react/package.json } |  |  | 0.771 |
| walker |  | 2274 | 20 | Fs::DirListing { dir: .github } |  |  | 0.771 |
| walker |  | 2282 | 8 | Fs::DirListing { dir: .github/workflows } |  |  | 0.771 |
| walker |  | 2293 | 11 | Json::Entry { file: samples/electron-esm-webpack/package.json } |  |  | 0.771 |
| ns | 2429 |  | 282 | _.contribution.ts: ILang/ILangImpl and registerLanguage() | 2.8 |  | 0.735 |
| walker |  | 2442 | 149 | Markdown::CommandBlock { file: CONTRIBUTING.md, row: 91 } |  |  | 0.737 |
| walker |  | 2479 | 37 | Fs::DirListing { dir: monaco-lsp-client/src/adapters } |  |  | 0.738 |
| walker |  | 2506 | 27 | Json::Entry { file: monaco-lsp-client/package.json } |  |  | 0.738 |
| ns | 2568 |  | 139 | _.contribution.ts: LazyLanguageLoader and loadLanguage() | 2.9 |  | 0.722 |
| walker |  | 2588 | 82 | Fs::DirListing { dir: samples/legacy } |  |  | 0.722 |
| walker |  | 2592 | 4 | Fs::DirListing { dir: samples/legacy/browser-amd-editor } |  |  | 0.722 |
| walker |  | 2596 | 4 | Fs::DirListing { dir: samples/legacy/browser-amd-localized } |  |  | 0.722 |
| walker |  | 2600 | 4 | Fs::DirListing { dir: samples/legacy/browser-amd-monarch } |  |  | 0.722 |
| walker |  | 2604 | 4 | Fs::DirListing { dir: samples/legacy/browser-amd-requirejs } |  |  | 0.722 |
| walker |  | 2608 | 4 | Fs::DirListing { dir: samples/legacy/browser-amd-shadow-dom } |  |  | 0.722 |
| walker |  | 2612 | 4 | Fs::DirListing { dir: samples/legacy/browser-amd-shared-model } |  |  | 0.722 |
| walker |  | 2616 | 4 | Fs::DirListing { dir: samples/legacy/browser-amd-trusted-types } |  |  | 0.722 |
| walker |  | 2624 | 8 | Fs::DirListing { dir: samples/legacy/browser-amd-iframe } |  |  | 0.722 |
| walker |  | 2636 | 12 | Fs::DirListing { dir: samples/legacy/browser-amd-diff-editor } |  |  | 0.722 |
| walker |  | 2653 | 17 | Fs::DirListing { dir: samples/legacy/electron-amd } |  |  | 0.722 |
| ns | 2657 |  | 89 | Grammar file shape: rust.ts conf + language exports | 2.10 |  | 0.711 |
| walker |  | 2669 | 16 | Json::Identity { file: samples/legacy/electron-amd/package.json } |  |  | 0.711 |
| walker |  | 2697 | 28 | Json::Entry { file: webpack-plugin/package.json } |  |  | 0.711 |
| walker |  | 2731 | 34 | Json::Dependencies { file: webpack-plugin/package.json } |  |  | 0.711 |
| walker |  | 2767 | 36 | Json::Dependencies { file: samples/package.json } |  |  | 0.711 |
| ns | 2797 |  | 140 | Tokenization test harness: testRunner.ts types + testTokenization(), and a test file head | 2.11 |  | 0.696 |
| walker |  | 2806 | 39 | Json::Scripts { file: samples/package.json } |  |  | 0.696 |
| ns | 2839 |  | 42 | src/languages/definitions/register.all.ts head, elided middle, tail | 2.12 |  | 0.690 |
| walker |  | 2952 | 146 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.728 |
| walker |  | 2965 | 13 | Json::Entry { file: samples/legacy/electron-amd/package.json } |  |  | 0.728 |
| ns | 2974 |  | 135 | src/deprecated/: the legacy import paths, all one-line re-exports | 2.13 |  | 0.715 |
| walker |  | 3008 | 43 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.715 |
| walker |  | 3088 | 80 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.715 |
| walker |  | 3148 | 60 | Fs::DirListing { dir: website } |  |  | 0.716 |
| walker |  | 3156 | 8 | Fs::DirListing { dir: website/static } |  |  | 0.716 |
| walker |  | 3165 | 9 | Fs::DirListing { dir: website/scripts } |  |  | 0.716 |
| walker |  | 3180 | 15 | Fs::DirListing { dir: website/static/monarch } |  |  | 0.716 |
| walker |  | 3203 | 23 | Fs::DirListing { dir: scripts/ci } |  |  | 0.716 |
| ns | 3343 |  | 369 | package.json scripts — build, test and dev commands (packaging variants elided) | 3.1 |  | 0.696 |
| ns | 3475 |  | 132 | CONTRIBUTING: the ordered editor-test command sequence | 3.2 |  | 0.700 |
| walker |  | 3497 | 294 | Code::CodeKey { rung: Names, file: monaco-lsp-client/generator/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.700 |
| walker |  | 3508 | 11 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 9, sub: 0, line: 82 } |  |  | 0.700 |
| walker |  | 3526 | 18 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 32, sub: 0, line: 259 } |  |  | 0.700 |
| ns | 3539 |  | 64 | test/ listing plus the full smoke-test directory | 3.3 |  | 0.686 |
| walker |  | 3547 | 21 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 21, sub: 0, line: 197 } |  |  | 0.686 |
| walker |  | 3568 | 21 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 22, sub: 0, line: 202 } |  |  | 0.686 |
| walker |  | 3589 | 21 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 24, sub: 0, line: 213 } |  |  | 0.686 |
| walker |  | 3610 | 21 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 25, sub: 0, line: 218 } |  |  | 0.686 |
| walker |  | 3631 | 21 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 26, sub: 0, line: 223 } |  |  | 0.686 |
| walker |  | 3653 | 22 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 27, sub: 0, line: 228 } |  |  | 0.686 |
| walker |  | 3675 | 22 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 29, sub: 0, line: 241 } |  |  | 0.686 |
| ns | 3690 |  | 151 | Listings of the five src/languages/features dirs — the uniform rich-service file set | 4.1 |  | 0.699 |
| walker |  | 3697 | 22 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 30, sub: 0, line: 246 } |  |  | 0.699 |
| walker |  | 3721 | 24 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 31, sub: 0, line: 251 } |  |  | 0.699 |
| walker |  | 3745 | 24 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 34, sub: 0, line: 684 } |  |  | 0.699 |
| walker |  | 3774 | 29 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 23, sub: 0, line: 207 } |  |  | 0.699 |
| walker |  | 3805 | 31 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 16, sub: 0, line: 154 } |  |  | 0.699 |
| walker |  | 3852 | 47 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 28, sub: 0, line: 233 } |  |  | 0.699 |
| walker |  | 3907 | 55 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 17, sub: 0, line: 159 } |  |  | 0.699 |
| ns | 3950 |  | 260 | css/register.ts — every top-level export (names only) | 4.2 |  | 0.682 |
| walker |  | 3964 | 57 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 15, sub: 0, line: 145 } |  |  | 0.682 |
| walker |  | 4024 | 60 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 8, sub: 0, line: 73 } |  |  | 0.682 |
| walker |  | 4088 | 64 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 13, sub: 0, line: 124 } |  |  | 0.682 |
| walker |  | 4162 | 74 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 12, sub: 0, line: 113 } |  |  | 0.682 |
| ns | 4235 |  | 285 | css: the default ModeConfiguration toggles, the three defaults objects, and the lazy onLanguage hookup | 4.3 | 4.2 | 0.655 |
| walker |  | 4239 | 77 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 14, sub: 0, line: 134 } |  |  | 0.655 |
| walker |  | 4318 | 79 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 1, sub: 0, line: 7 } |  |  | 0.655 |
| walker |  | 4405 | 87 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 11, sub: 0, line: 101 } |  |  | 0.655 |
| ns | 4494 |  | 259 | json/register.ts — every top-level export (names only) | 4.4 |  | 0.639 |
| walker |  | 4506 | 101 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 19, sub: 0, line: 170 } |  |  | 0.639 |
| walker |  | 4620 | 114 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 10, sub: 0, line: 86 } |  |  | 0.639 |
| walker |  | 4732 | 112 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 20, sub: 0, line: 183 } |  |  | 0.639 |
| walker |  | 4739 | 7 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 34, sub: 0, line: 684 } |  |  | 0.639 |
| ns | 4763 |  | 269 | json defaults: diagnostic option values, mode toggles, jsonDefaults, the worker interface | 4.5 | 4.4 | 0.619 |
| walker |  | 4769 | 30 | Json::Scripts { file: samples/browser-esm-esbuild/package.json } |  |  | 0.619 |
| walker |  | 4793 | 24 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.619 |
| walker |  | 4824 | 31 | Json::Scripts { file: samples/browser-esm-parcel/package.json } |  |  | 0.619 |
| walker |  | 4885 | 61 | Json::Scripts { file: monaco-lsp-client/package.json } |  |  | 0.619 |
| walker |  | 4906 | 21 | Json::ScriptsTail { file: monaco-lsp-client/package.json } |  |  | 0.619 |
| walker |  | 4938 | 32 | Json::Scripts { file: samples/browser-esm-webpack-small/package.json } |  |  | 0.619 |
| walker |  | 4962 | 24 | Json::ScriptsTail { file: samples/browser-esm-webpack-small/package.json } |  |  | 0.619 |
| walker |  | 4994 | 32 | Json::Scripts { file: samples/electron-esm-webpack/package.json } |  |  | 0.619 |
| walker |  | 5021 | 27 | Json::ScriptsTail { file: samples/electron-esm-webpack/package.json } |  |  | 0.619 |
| ns | 5025 |  | 262 | html/register.ts — export names plus the registerHTMLLanguageService factory | 4.6 |  | 0.605 |
| walker |  | 5033 | 12 | Code::CodeKey { rung: Names, file: samples/browser-esm-vite-react/src/main.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.605 |
| walker |  | 5077 | 44 | Code::CodeKey { rung: Decl, file: samples/browser-esm-vite-react/src/main.tsx, decl: 1, sub: 0, line: 6 } |  |  | 0.605 |
| walker |  | 5089 | 12 | Code::CodeKey { rung: Names, file: samples/browser-esm-webpack-typescript-react/src/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.605 |
| walker |  | 5133 | 44 | Code::CodeKey { rung: Decl, file: samples/browser-esm-webpack-typescript-react/src/index.tsx, decl: 1, sub: 0, line: 7 } |  |  | 0.605 |
| walker |  | 5159 | 26 | Fs::DirListing { dir: website/src } |  |  | 0.605 |
| walker |  | 5171 | 12 | Fs::DirListing { dir: website/src/runner } |  |  | 0.605 |
| walker |  | 5240 | 69 | Json::Scripts { file: webpack-plugin/package.json } |  |  | 0.605 |
| ns | 5309 |  | 284 | html: the three pre-registered services and their per-language toggles | 4.7 | 4.6 | 0.590 |
| walker |  | 5408 | 168 | Json::Entry { file: package.json } |  |  | 0.591 |
| walker |  | 5445 | 37 | Json::Scripts { file: samples/browser-esm-vite/package.json } |  |  | 0.591 |
| walker |  | 5462 | 17 | Json::ScriptsTail { file: samples/browser-esm-vite/package.json } |  |  | 0.591 |
| walker |  | 5499 | 37 | Json::Scripts { file: samples/browser-esm-vite-react/package.json } |  |  | 0.591 |
| walker |  | 5537 | 38 | Json::Scripts { file: samples/browser-esm-webpack/package.json } |  |  | 0.591 |
| ns | 5554 |  | 245 | typescript/register.ts — every top-level export (names only) | 4.8 |  | 0.580 |
| walker |  | 5575 | 38 | Json::Scripts { file: samples/browser-esm-webpack-monaco-plugin/package.json } |  |  | 0.580 |
| walker |  | 5677 | 102 | Code::CodeKey { rung: Names, file: samples/electron-esm-webpack/main.js, decl: 0, sub: 0, line: 0 } |  |  | 0.580 |
| walker |  | 5706 | 29 | Code::CodeKey { rung: Decl, file: samples/electron-esm-webpack/main.js, decl: 7, sub: 0, line: 28 } |  |  | 0.580 |
| walker |  | 5737 | 31 | Code::CodeKey { rung: Decl, file: samples/electron-esm-webpack/main.js, decl: 6, sub: 0, line: 22 } |  |  | 0.580 |
| ns | 5789 |  | 235 | LanguageServiceDefaults (typescript) — all members | 4.9 |  | 0.569 |
| walker |  | 5839 | 102 | Code::CodeKey { rung: Names, file: samples/legacy/electron-amd/main.js, decl: 0, sub: 0, line: 0 } |  |  | 0.569 |
| walker |  | 5868 | 29 | Code::CodeKey { rung: Decl, file: samples/legacy/electron-amd/main.js, decl: 7, sub: 0, line: 28 } |  |  | 0.569 |
| walker |  | 5899 | 31 | Code::CodeKey { rung: Decl, file: samples/legacy/electron-amd/main.js, decl: 6, sub: 0, line: 22 } |  |  | 0.569 |
| walker |  | 5943 | 44 | Code::CodeKey { rung: Names, file: webpack-plugin/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.569 |
| walker |  | 5989 | 46 | Code::CodeKey { rung: Decl, file: webpack-plugin/src/index.ts, decl: 2, sub: 0, line: 159 } |  |  | 0.569 |
| ns | 6025 |  | 236 | TypeScriptWorker — all 21 proxy methods (names only) | 4.10 | 4.8 | 0.558 |
| walker |  | 6070 | 81 | Json::Dependencies { file: monaco-lsp-client/package.json } |  |  | 0.558 |
| walker |  | 6100 | 30 | Code::CodeKey { rung: Names, file: src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.560 |
| walker |  | 6131 | 31 | Code::CodeKey { rung: Names, file: samples/browser-esm-webpack-monaco-plugin/index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.560 |
| walker |  | 6171 | 40 | Code::CodeKey { rung: Decl, file: samples/browser-esm-webpack-monaco-plugin/index.js, decl: 2, sub: 0, line: 12 } |  |  | 0.560 |
| walker |  | 6255 | 84 | Code::CodeKey { rung: Decl, file: samples/browser-esm-webpack-monaco-plugin/index.js, decl: 1, sub: 0, line: 3 } |  |  | 0.560 |
| walker |  | 6271 | 16 | Code::CodeKey { rung: Names, file: samples/browser-esm-webpack-typescript/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.560 |
| walker |  | 6312 | 41 | Code::CodeKey { rung: Decl, file: samples/browser-esm-webpack-typescript/src/index.ts, decl: 1, sub: 0, line: 23 } |  |  | 0.560 |
| ns | 6316 |  | 291 | typescript/javascript defaults and their divergence | 4.11 | 4.8 | 0.547 |
| walker |  | 6362 | 50 | Json::ScriptsTail { file: samples/browser-esm-vite-react/package.json } |  |  | 0.547 |
| walker |  | 6466 | 104 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.547 |
| walker |  | 6488 | 22 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 4, sub: 0, line: 41 } |  |  | 0.547 |
| walker |  | 6510 | 22 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 8, sub: 0, line: 73 } |  |  | 0.547 |
| walker |  | 6532 | 22 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 32, sub: 0, line: 259 } |  |  | 0.547 |
| ns | 6545 |  | 229 | The four *Mode.ts entry points and the tsMode re-export tail | 5.1 |  | 0.542 |
| walker |  | 6553 | 21 | Code::CodeKey { rung: Names, file: samples/browser-esm-esbuild/index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.542 |
| walker |  | 6594 | 41 | Code::CodeKey { rung: Decl, file: samples/browser-esm-esbuild/index.js, decl: 1, sub: 0, line: 21 } |  |  | 0.542 |
| walker |  | 6615 | 21 | Code::CodeKey { rung: Names, file: samples/browser-esm-parcel/src/index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.542 |
| walker |  | 6656 | 41 | Code::CodeKey { rung: Decl, file: samples/browser-esm-parcel/src/index.js, decl: 1, sub: 0, line: 26 } |  |  | 0.542 |
| walker |  | 6677 | 21 | Code::CodeKey { rung: Names, file: samples/browser-esm-webpack/index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.542 |
| walker |  | 6718 | 41 | Code::CodeKey { rung: Decl, file: samples/browser-esm-webpack/index.js, decl: 1, sub: 0, line: 21 } |  |  | 0.542 |
| walker |  | 6739 | 21 | Code::CodeKey { rung: Names, file: samples/browser-esm-webpack-small/index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.542 |
| ns | 6759 |  | 214 | cssMode.setupMode — WorkerManager, the worker accessor, and ModeConfiguration-gated provider registration | 5.2 | 5.1 | 0.532 |
| walker |  | 6933 | 194 | Code::CodeKey { rung: Decl, file: samples/browser-esm-webpack-small/index.js, decl: 1, sub: 0, line: 152 } |  |  | 0.532 |
| walker |  | 6954 | 21 | Code::CodeKey { rung: Names, file: samples/electron-esm-webpack/index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.532 |
| walker |  | 6995 | 41 | Code::CodeKey { rung: Decl, file: samples/electron-esm-webpack/index.js, decl: 1, sub: 0, line: 21 } |  |  | 0.532 |
| walker |  | 7018 | 23 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 5, sub: 0, line: 48 } |  |  | 0.532 |
| walker |  | 7103 | 85 | Code::CodeKey { rung: Names, file: samples/browser-esm-vite/main.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.532 |
| ns | 7112 |  | 353 | common/lspLanguageFeatures.ts — the shared provider adapters and LSP conversion helpers | 5.3 |  | 0.523 |
| walker |  | 7116 | 13 | Code::CodeKey { rung: Decl, file: samples/browser-esm-vite/main.ts, decl: 3, sub: 0, line: 18 } |  |  | 0.523 |
| walker |  | 7148 | 32 | Code::CodeKey { rung: Decl, file: samples/browser-esm-vite/main.ts, decl: 4, sub: 0, line: 21 } |  |  | 0.523 |
| walker |  | 7194 | 46 | Fs::DirListing { dir: website/src/website } |  |  | 0.523 |
| walker |  | 7209 | 15 | Fs::DirListing { dir: website/src/website/data } |  |  | 0.523 |
| walker |  | 7236 | 27 | Fs::DirListing { dir: website/src/website/pages } |  |  | 0.523 |
| walker |  | 7241 | 5 | Fs::DirListing { dir: website/src/website/pages/home } |  |  | 0.523 |
| walker |  | 7275 | 34 | Fs::DirListing { dir: website/src/website/data/playground-samples } |  |  | 0.523 |
| walker |  | 7294 | 19 | Fs::DirListing { dir: website/src/website/data/playground-samples/customizing-the-appearence } |  |  | 0.523 |
| walker |  | 7310 | 16 | Fs::DirListing { dir: website/src/website/data/playground-samples/customizing-the-appearence/scrollbars } |  |  | 0.523 |
| walker |  | 7328 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/customizing-the-appearence/exposed-colors } |  |  | 0.523 |
| walker |  | 7346 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/customizing-the-appearence/tokens-and-colors } |  |  | 0.523 |
| walker |  | 7367 | 21 | Fs::DirListing { dir: website/src/website/data/playground-samples/creating-the-diffeditor } |  |  | 0.523 |
| walker |  | 7385 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/creating-the-diffeditor/hello-diff-world } |  |  | 0.523 |
| walker |  | 7403 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/creating-the-diffeditor/inline-diff-example } |  |  | 0.523 |
| ns | 7419 |  | 307 | Web-worker plumbing: MonacoEnvironment hooks, createWebWorker, IWebWorkerOptions fields, worker entry points | 5.4 |  | 0.513 |
| walker |  | 7421 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/creating-the-diffeditor/multi-line-example } |  |  | 0.513 |
| walker |  | 7452 | 31 | Fs::DirListing { dir: website/src/website/data/playground-samples/creating-the-editor } |  |  | 0.513 |
| walker |  | 7470 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/creating-the-editor/editor-basic-options } |  |  | 0.513 |
| walker |  | 7488 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/creating-the-editor/hard-wrapping } |  |  | 0.513 |
| walker |  | 7506 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/creating-the-editor/hello-world } |  |  | 0.513 |
| walker |  | 7524 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/creating-the-editor/syntax-highlighting-for-html-elements } |  |  | 0.513 |
| walker |  | 7542 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/creating-the-editor/web-component } |  |  | 0.513 |
| ns | 7574 |  | 155 | WorkerManager lifecycle and the CSSWorker entry points | 5.5 |  | 0.509 |
| walker |  | 7582 | 40 | Fs::DirListing { dir: website/src/website/components } |  |  | 0.509 |
| walker |  | 7594 | 12 | Fs::DirListing { dir: website/src/website/components/monaco } |  |  | 0.509 |
| walker |  | 7636 | 42 | Fs::DirListing { dir: website/src/website/utils } |  |  | 0.509 |
| walker |  | 7670 | 34 | Json::Scripts { file: samples/legacy/electron-amd/package.json } |  |  | 0.509 |
| walker |  | 7721 | 51 | Fs::DirListing { dir: test/smoke } |  |  | 0.524 |
| walker |  | 7729 | 8 | Fs::DirListing { dir: test/smoke/esbuild } |  |  | 0.524 |
| walker |  | 7737 | 8 | Fs::DirListing { dir: test/smoke/vite } |  |  | 0.524 |
| walker |  | 7745 | 8 | Fs::DirListing { dir: test/smoke/webpack } |  |  | 0.524 |
| walker |  | 7757 | 12 | Fs::DirListing { dir: test/smoke/parcel } |  |  | 0.524 |
| walker |  | 7782 | 25 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 1, sub: 0, line: 7 } |  |  | 0.524 |
| walker |  | 7807 | 25 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 2, sub: 0, line: 15 } |  |  | 0.524 |
| ns | 7828 |  | 254 | typescript/languageFeatures.ts — the TypeScript-specific adapter roster | 5.6 |  | 0.518 |
| walker |  | 7832 | 25 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 7, sub: 0, line: 64 } |  |  | 0.518 |
| walker |  | 7857 | 25 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 33, sub: 0, line: 628 } |  |  | 0.518 |
| ns | 7907 |  | 79 | monaco-lsp-client package: complete tree listing | 6.1 |  | 0.526 |
| walker |  | 7920 | 63 | Json::Scripts { file: samples/browser-esm-webpack-typescript/package.json } |  |  | 0.526 |
| walker |  | 7946 | 26 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 3, sub: 0, line: 26 } |  |  | 0.526 |
| walker |  | 7972 | 26 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 6, sub: 0, line: 57 } |  |  | 0.526 |
| ns | 8114 |  | 207 | MonacoLspClient wiring + index exports | 6.2 | 6.1 | 0.521 |
| ns | 8163 |  | 49 | webpack-plugin package: complete tree listing | 6.3 |  | 0.527 |
| walker |  | 8166 | 194 | Code::CodeKey { rung: Decl, file: webpack-plugin/src/index.ts, decl: 1, sub: 0, line: 102 } |  |  | 0.527 |
| walker |  | 8360 | 194 | Code::CodeKey { rung: Decl, file: webpack-plugin/src/index.ts, decl: 1, sub: 1, line: 102 } |  |  | 0.527 |
| ns | 8447 |  | 284 | webpack-plugin option fields and peer-dependency contract | 6.4 | 6.3 | 0.529 |
| walker |  | 8593 | 233 | Code::CodeKey { rung: Decl, file: webpack-plugin/src/index.ts, decl: 1, sub: 2, line: 102 } |  |  | 0.532 |
| ns | 8611 |  | 164 | build/ and scripts/ tree listings | 7.1 |  | 0.522 |
| walker |  | 8659 | 66 | Json::Scripts { file: samples/browser-esm-webpack-typescript-react/package.json } |  |  | 0.522 |
| ns | 8760 |  | 149 | build-monaco-editor.ts run() — what the published package is made of | 7.2 |  | 0.518 |
| walker |  | 8811 | 152 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.519 |
| walker |  | 8839 | 28 | Code::CodeKey { rung: Names, file: monaco-lsp-client/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.519 |
| walker |  | 8906 | 67 | Fs::DirListing { dir: website/src/website/pages/playground } |  |  | 0.519 |
| walker |  | 8981 | 75 | Fs::DirListing { dir: website/src/website/data/playground-samples/extending-language-services } |  |  | 0.519 |
| ns | 8998 |  | 238 | check-samples.ts — the invariant every new language must satisfy | 7.3 |  | 0.512 |
| walker |  | 9055 | 74 | Fs::DirListing { dir: website/src/website/data/playground-samples/interacting-with-the-editor } |  |  | 0.512 |
| walker |  | 9071 | 16 | Fs::DirListing { dir: website/src/website/data/playground-samples/interacting-with-the-editor/line-and-inline-decorations } |  |  | 0.512 |
| walker |  | 9087 | 16 | Fs::DirListing { dir: website/src/website/data/playground-samples/interacting-with-the-editor/listening-to-mouse-events } |  |  | 0.512 |
| walker |  | 9103 | 16 | Fs::DirListing { dir: website/src/website/data/playground-samples/interacting-with-the-editor/rendering-glyphs-in-the-margin } |  |  | 0.512 |
| walker |  | 9121 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/interacting-with-the-editor/adding-a-command-to-an-editor-instance } |  |  | 0.512 |
| walker |  | 9139 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/interacting-with-the-editor/adding-a-keybinding-to-an-existing-command } |  |  | 0.512 |
| walker |  | 9157 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/interacting-with-the-editor/adding-an-action-to-an-editor-instance } |  |  | 0.512 |
| ns | 9164 |  | 166 | package.json distribution fields: typings, main, module, exports | 8.1 |  | 0.517 |
| walker |  | 9175 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/interacting-with-the-editor/customizing-the-line-numbers } |  |  | 0.517 |
| walker |  | 9193 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/interacting-with-the-editor/listening-to-key-events } |  |  | 0.517 |
| walker |  | 9211 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/interacting-with-the-editor/revealing-a-position } |  |  | 0.517 |
| walker |  | 9385 | 174 | Fs::DirListing { dir: monaco-lsp-client/src/adapters/languageFeatures } |  |  | 0.517 |
| ns | 9418 |  | 254 | README Installing + CHANGELOG head (0.55.x breaking changes) | 8.2 |  | 0.513 |
| walker |  | 9465 | 80 | Fs::DirListing { dir: test/manual } |  |  | 0.513 |
| walker |  | 9479 | 14 | Fs::DirListing { dir: test/manual/typescript } |  |  | 0.513 |
| walker |  | 9499 | 20 | Code::CodeKey { rung: Names, file: src/editor.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.514 |
| ns | 9522 |  | 104 | samples/ listing — every integration sample directory | 8.3 |  | 0.521 |
| ns | 9560 |  | 38 | docs/ listing | 8.4 |  | 0.523 |
| ns | 9643 |  | 83 | integrate-esm.md section headings | 8.5 | 8.4 | 0.521 |
| ns | 9769 |  | 126 | website/ and CI/publishing config listings | 8.6 |  | 0.532 |
| walker |  | 9984 | 485 | Json::ScriptsTail { file: package.json } |  |  | 0.540 |

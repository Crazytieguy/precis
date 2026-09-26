Score(3000)=0.715 I=0.808 C=0.632 ns_rows≤3K=23/56 grid(1000/1442/2080/3000/4327/6240/9000)=0.639/0.696/0.830/0.715/0.656/0.578/0.537

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
| walker |  | 281 | 70 | Json::Identity { file: package.json } |  |  | 0.519 |
| walker |  | 288 | 7 | Fs::DirListing { dir: src/languages/features/common } |  |  | 0.519 |
| ns | 332 |  | 116 | Complete repository root listing | 1.4 |  | 0.648 |
| walker |  | 355 | 67 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.845 |
| walker |  | 364 | 9 | Fs::DirListing { dir: src/internal/common } |  |  | 0.849 |
| walker |  | 375 | 11 | Fs::DirListing { dir: src/deprecated } |  |  | 0.853 |
| walker |  | 382 | 7 | Fs::DirListing { dir: src/deprecated/basic-languages } |  |  | 0.853 |
| walker |  | 392 | 10 | Fs::DirListing { dir: src/deprecated/editor } |  |  | 0.853 |
| walker |  | 405 | 13 | Fs::DirListing { dir: src/deprecated/language } |  |  | 0.860 |
| walker |  | 417 | 12 | Fs::DirListing { dir: src/deprecated/language/css } |  |  | 0.860 |
| walker |  | 429 | 12 | Fs::DirListing { dir: src/deprecated/language/html } |  |  | 0.860 |
| walker |  | 441 | 12 | Fs::DirListing { dir: src/deprecated/language/json } |  |  | 0.860 |
| walker |  | 453 | 12 | Fs::DirListing { dir: src/deprecated/language/typescript } |  |  | 0.860 |
| ns | 471 |  | 139 | package.json identity header (name, version, vscodeRef, license) | 1.5 |  | 0.799 |
| walker |  | 477 | 24 | Fs::DirListing { dir: src/languages/features/css } |  |  | 0.800 |
| walker |  | 501 | 24 | Fs::DirListing { dir: src/languages/features/html } |  |  | 0.801 |
| walker |  | 528 | 27 | Fs::DirListing { dir: monaco-lsp-client } |  |  | 0.802 |
| walker |  | 532 | 4 | Fs::DirListing { dir: monaco-lsp-client/generator } |  |  | 0.802 |
| walker |  | 547 | 15 | Fs::DirListing { dir: monaco-lsp-client/src } |  |  | 0.802 |
| walker |  | 576 | 29 | Fs::DirListing { dir: src/languages/features/json } |  |  | 0.804 |
| ns | 588 |  | 117 | The three one-to-six-line re-export files src/index.ts depends on | 1.6 | 1.2 | 0.738 |
| walker |  | 605 | 29 | Fs::DirListing { dir: webpack-plugin } |  |  | 0.738 |
| walker |  | 625 | 20 | Fs::DirListing { dir: webpack-plugin/src } |  |  | 0.739 |
| walker |  | 629 | 4 | Fs::DirListing { dir: webpack-plugin/src/loaders } |  |  | 0.739 |
| walker |  | 634 | 5 | Fs::DirListing { dir: .devcontainer } |  |  | 0.739 |
| walker |  | 639 | 5 | Fs::DirListing { dir: .husky } |  |  | 0.739 |
| walker |  | 647 | 8 | Fs::DirListing { dir: webpack-plugin/src/plugins } |  |  | 0.739 |
| walker |  | 685 | 38 | Fs::DirListing { dir: docs } |  |  | 0.740 |
| ns | 734 |  | 146 | All README section headings (locations) | 1.7 |  | 0.656 |
| walker |  | 746 | 61 | Json::Scripts { file: package.json } |  |  | 0.656 |
| walker |  | 752 | 6 | Fs::DirListing { dir: scripts } |  |  | 0.656 |
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
| walker |  | 1534 | 20 | Fs::DirListing { dir: .github } |  |  | 0.696 |
| walker |  | 1542 | 8 | Fs::DirListing { dir: .github/workflows } |  |  | 0.696 |
| walker |  | 1691 | 149 | Markdown::CommandBlock { file: CONTRIBUTING.md, row: 91 } |  |  | 0.698 |
| ns | 1702 |  | 272 | Complete listing of src/features/ — all 65 editor-feature shim directories | 2.3 |  | 0.772 |
| walker |  | 1728 | 37 | Fs::DirListing { dir: monaco-lsp-client/src/adapters } |  |  | 0.773 |
| ns | 1746 |  | 44 | src/features/register.all.ts head, elided middle, tail | 2.4 |  | 0.761 |
| walker |  | 1755 | 27 | Json::Entry { file: monaco-lsp-client/package.json } |  |  | 0.761 |
| walker |  | 1783 | 28 | Json::Entry { file: webpack-plugin/package.json } |  |  | 0.761 |
| walker |  | 1817 | 34 | Json::Dependencies { file: webpack-plugin/package.json } |  |  | 0.761 |
| walker |  | 1963 | 146 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.820 |
| walker |  | 2006 | 43 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.820 |
| ns | 2050 |  | 304 | Complete listing of src/languages/definitions/ — all 84 Monarch languages plus _.contribution.ts, register.all.ts and test/ | 2.5 |  | 0.835 |
| ns | 2063 |  | 13 | One language definition directory, listed in full (rust) | 2.6 |  | 0.830 |
| walker |  | 2086 | 80 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.830 |
| walker |  | 2146 | 60 | Fs::DirListing { dir: website } |  |  | 0.831 |
| ns | 2147 |  | 84 | rust/register.ts registerLanguage call | 2.7 | 2.6 | 0.817 |
| walker |  | 2154 | 8 | Fs::DirListing { dir: website/static } |  |  | 0.817 |
| walker |  | 2163 | 9 | Fs::DirListing { dir: website/scripts } |  |  | 0.817 |
| walker |  | 2178 | 15 | Fs::DirListing { dir: website/static/monarch } |  |  | 0.817 |
| walker |  | 2201 | 23 | Fs::DirListing { dir: scripts/ci } |  |  | 0.817 |
| ns | 2429 |  | 282 | _.contribution.ts: ILang/ILangImpl and registerLanguage() | 2.8 |  | 0.778 |
| walker |  | 2495 | 294 | Code::CodeKey { rung: Names, file: monaco-lsp-client/generator/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.778 |
| walker |  | 2506 | 11 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 9, sub: 0, line: 82 } |  |  | 0.778 |
| walker |  | 2524 | 18 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 32, sub: 0, line: 259 } |  |  | 0.778 |
| walker |  | 2545 | 21 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 21, sub: 0, line: 197 } |  |  | 0.778 |
| walker |  | 2566 | 21 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 22, sub: 0, line: 202 } |  |  | 0.778 |
| ns | 2568 |  | 139 | _.contribution.ts: LazyLanguageLoader and loadLanguage() | 2.9 |  | 0.761 |
| walker |  | 2587 | 21 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 24, sub: 0, line: 213 } |  |  | 0.761 |
| walker |  | 2608 | 21 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 25, sub: 0, line: 218 } |  |  | 0.761 |
| walker |  | 2629 | 21 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 26, sub: 0, line: 223 } |  |  | 0.761 |
| walker |  | 2651 | 22 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 27, sub: 0, line: 228 } |  |  | 0.761 |
| ns | 2657 |  | 89 | Grammar file shape: rust.ts conf + language exports | 2.10 |  | 0.750 |
| walker |  | 2673 | 22 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 29, sub: 0, line: 241 } |  |  | 0.750 |
| walker |  | 2695 | 22 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 30, sub: 0, line: 246 } |  |  | 0.750 |
| walker |  | 2719 | 24 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 31, sub: 0, line: 251 } |  |  | 0.750 |
| walker |  | 2743 | 24 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 34, sub: 0, line: 684 } |  |  | 0.750 |
| walker |  | 2772 | 29 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 23, sub: 0, line: 207 } |  |  | 0.750 |
| ns | 2797 |  | 140 | Tokenization test harness: testRunner.ts types + testTokenization(), and a test file head | 2.11 |  | 0.734 |
| walker |  | 2803 | 31 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 16, sub: 0, line: 154 } |  |  | 0.734 |
| ns | 2839 |  | 42 | src/languages/definitions/register.all.ts head, elided middle, tail | 2.12 |  | 0.728 |
| walker |  | 2850 | 47 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 28, sub: 0, line: 233 } |  |  | 0.728 |
| walker |  | 2905 | 55 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 17, sub: 0, line: 159 } |  |  | 0.728 |
| walker |  | 2962 | 57 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 15, sub: 0, line: 145 } |  |  | 0.728 |
| ns | 2974 |  | 135 | src/deprecated/: the legacy import paths, all one-line re-exports | 2.13 |  | 0.715 |
| walker |  | 3022 | 60 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 8, sub: 0, line: 73 } |  |  | 0.715 |
| walker |  | 3086 | 64 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 13, sub: 0, line: 124 } |  |  | 0.715 |
| walker |  | 3160 | 74 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 12, sub: 0, line: 113 } |  |  | 0.715 |
| walker |  | 3237 | 77 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 14, sub: 0, line: 134 } |  |  | 0.715 |
| walker |  | 3316 | 79 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 1, sub: 0, line: 7 } |  |  | 0.715 |
| ns | 3343 |  | 369 | package.json scripts — build, test and dev commands (packaging variants elided) | 3.1 |  | 0.694 |
| walker |  | 3403 | 87 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 11, sub: 0, line: 101 } |  |  | 0.694 |
| ns | 3475 |  | 132 | CONTRIBUTING: the ordered editor-test command sequence | 3.2 |  | 0.699 |
| walker |  | 3504 | 101 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 19, sub: 0, line: 170 } |  |  | 0.699 |
| ns | 3539 |  | 64 | test/ listing plus the full smoke-test directory | 3.3 |  | 0.685 |
| walker |  | 3618 | 114 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 10, sub: 0, line: 86 } |  |  | 0.685 |
| ns | 3690 |  | 151 | Listings of the five src/languages/features dirs — the uniform rich-service file set | 4.1 |  | 0.697 |
| walker |  | 3730 | 112 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/generator/index.ts, decl: 20, sub: 0, line: 183 } |  |  | 0.697 |
| walker |  | 3737 | 7 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 34, sub: 0, line: 684 } |  |  | 0.697 |
| walker |  | 3761 | 24 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.697 |
| walker |  | 3822 | 61 | Json::Scripts { file: monaco-lsp-client/package.json } |  |  | 0.697 |
| walker |  | 3843 | 21 | Json::ScriptsTail { file: monaco-lsp-client/package.json } |  |  | 0.697 |
| walker |  | 3869 | 26 | Fs::DirListing { dir: website/src } |  |  | 0.698 |
| walker |  | 3881 | 12 | Fs::DirListing { dir: website/src/runner } |  |  | 0.698 |
| walker |  | 3950 | 69 | Json::Scripts { file: webpack-plugin/package.json } |  |  | 0.681 |
| ns | 3950 |  | 260 | css/register.ts — every top-level export (names only) | 4.2 |  | 0.681 |
| walker |  | 4118 | 168 | Json::Entry { file: package.json } |  |  | 0.682 |
| walker |  | 4222 | 104 | Fs::DirListing { dir: samples } |  |  | 0.683 |
| ns | 4235 |  | 285 | css: the default ModeConfiguration toggles, the three defaults objects, and the lazy onLanguage hookup | 4.3 | 4.2 | 0.656 |
| walker |  | 4266 | 44 | Code::CodeKey { rung: Names, file: webpack-plugin/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.656 |
| walker |  | 4312 | 46 | Code::CodeKey { rung: Decl, file: webpack-plugin/src/index.ts, decl: 2, sub: 0, line: 159 } |  |  | 0.656 |
| walker |  | 4393 | 81 | Json::Dependencies { file: monaco-lsp-client/package.json } |  |  | 0.656 |
| walker |  | 4423 | 30 | Code::CodeKey { rung: Names, file: src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.658 |
| ns | 4494 |  | 259 | json/register.ts — every top-level export (names only) | 4.4 |  | 0.642 |
| walker |  | 4527 | 104 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.642 |
| walker |  | 4549 | 22 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 4, sub: 0, line: 41 } |  |  | 0.642 |
| walker |  | 4571 | 22 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 8, sub: 0, line: 73 } |  |  | 0.642 |
| walker |  | 4593 | 22 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 32, sub: 0, line: 259 } |  |  | 0.642 |
| walker |  | 4616 | 23 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 5, sub: 0, line: 48 } |  |  | 0.642 |
| walker |  | 4662 | 46 | Fs::DirListing { dir: website/src/website } |  |  | 0.642 |
| walker |  | 4677 | 15 | Fs::DirListing { dir: website/src/website/data } |  |  | 0.642 |
| walker |  | 4704 | 27 | Fs::DirListing { dir: website/src/website/pages } |  |  | 0.642 |
| walker |  | 4709 | 5 | Fs::DirListing { dir: website/src/website/pages/home } |  |  | 0.642 |
| walker |  | 4743 | 34 | Fs::DirListing { dir: website/src/website/data/playground-samples } |  |  | 0.642 |
| walker |  | 4762 | 19 | Fs::DirListing { dir: website/src/website/data/playground-samples/customizing-the-appearence } |  |  | 0.642 |
| ns | 4763 |  | 269 | json defaults: diagnostic option values, mode toggles, jsonDefaults, the worker interface | 4.5 | 4.4 | 0.622 |
| walker |  | 4778 | 16 | Fs::DirListing { dir: website/src/website/data/playground-samples/customizing-the-appearence/scrollbars } |  |  | 0.622 |
| walker |  | 4796 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/customizing-the-appearence/exposed-colors } |  |  | 0.622 |
| walker |  | 4814 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/customizing-the-appearence/tokens-and-colors } |  |  | 0.622 |
| walker |  | 4835 | 21 | Fs::DirListing { dir: website/src/website/data/playground-samples/creating-the-diffeditor } |  |  | 0.622 |
| walker |  | 4853 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/creating-the-diffeditor/hello-diff-world } |  |  | 0.622 |
| walker |  | 4871 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/creating-the-diffeditor/inline-diff-example } |  |  | 0.622 |
| walker |  | 4889 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/creating-the-diffeditor/multi-line-example } |  |  | 0.622 |
| walker |  | 4920 | 31 | Fs::DirListing { dir: website/src/website/data/playground-samples/creating-the-editor } |  |  | 0.622 |
| walker |  | 4938 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/creating-the-editor/editor-basic-options } |  |  | 0.622 |
| walker |  | 4956 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/creating-the-editor/hard-wrapping } |  |  | 0.622 |
| walker |  | 4974 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/creating-the-editor/hello-world } |  |  | 0.622 |
| walker |  | 4992 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/creating-the-editor/syntax-highlighting-for-html-elements } |  |  | 0.622 |
| walker |  | 5010 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/creating-the-editor/web-component } |  |  | 0.622 |
| ns | 5025 |  | 262 | html/register.ts — export names plus the registerHTMLLanguageService factory | 4.6 |  | 0.608 |
| walker |  | 5050 | 40 | Fs::DirListing { dir: website/src/website/components } |  |  | 0.608 |
| walker |  | 5062 | 12 | Fs::DirListing { dir: website/src/website/components/monaco } |  |  | 0.608 |
| walker |  | 5104 | 42 | Fs::DirListing { dir: website/src/website/utils } |  |  | 0.608 |
| walker |  | 5155 | 51 | Fs::DirListing { dir: test/smoke } |  |  | 0.626 |
| walker |  | 5163 | 8 | Fs::DirListing { dir: test/smoke/esbuild } |  |  | 0.626 |
| walker |  | 5171 | 8 | Fs::DirListing { dir: test/smoke/vite } |  |  | 0.626 |
| walker |  | 5179 | 8 | Fs::DirListing { dir: test/smoke/webpack } |  |  | 0.626 |
| walker |  | 5191 | 12 | Fs::DirListing { dir: test/smoke/parcel } |  |  | 0.626 |
| walker |  | 5216 | 25 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 1, sub: 0, line: 7 } |  |  | 0.626 |
| walker |  | 5241 | 25 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 2, sub: 0, line: 15 } |  |  | 0.626 |
| walker |  | 5266 | 25 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 7, sub: 0, line: 64 } |  |  | 0.626 |
| walker |  | 5291 | 25 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 33, sub: 0, line: 628 } |  |  | 0.626 |
| ns | 5309 |  | 284 | html: the three pre-registered services and their per-language toggles | 4.7 | 4.6 | 0.611 |
| walker |  | 5317 | 26 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 3, sub: 0, line: 26 } |  |  | 0.611 |
| walker |  | 5343 | 26 | Code::CodeKey { rung: Doc, file: monaco-lsp-client/generator/index.ts, decl: 6, sub: 0, line: 57 } |  |  | 0.611 |
| walker |  | 5537 | 194 | Code::CodeKey { rung: Decl, file: webpack-plugin/src/index.ts, decl: 1, sub: 0, line: 102 } |  |  | 0.611 |
| ns | 5554 |  | 245 | typescript/register.ts — every top-level export (names only) | 4.8 |  | 0.599 |
| walker |  | 5731 | 194 | Code::CodeKey { rung: Decl, file: webpack-plugin/src/index.ts, decl: 1, sub: 1, line: 102 } |  |  | 0.599 |
| ns | 5789 |  | 235 | LanguageServiceDefaults (typescript) — all members | 4.9 |  | 0.588 |
| walker |  | 5964 | 233 | Code::CodeKey { rung: Decl, file: webpack-plugin/src/index.ts, decl: 1, sub: 2, line: 102 } |  |  | 0.589 |
| ns | 6025 |  | 236 | TypeScriptWorker — all 21 proxy methods (names only) | 4.10 | 4.8 | 0.577 |
| walker |  | 6116 | 152 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.578 |
| walker |  | 6144 | 28 | Code::CodeKey { rung: Names, file: monaco-lsp-client/src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.578 |
| walker |  | 6211 | 67 | Fs::DirListing { dir: website/src/website/pages/playground } |  |  | 0.578 |
| walker |  | 6286 | 75 | Fs::DirListing { dir: website/src/website/data/playground-samples/extending-language-services } |  |  | 0.578 |
| ns | 6316 |  | 291 | typescript/javascript defaults and their divergence | 4.11 | 4.8 | 0.564 |
| walker |  | 6360 | 74 | Fs::DirListing { dir: website/src/website/data/playground-samples/interacting-with-the-editor } |  |  | 0.564 |
| walker |  | 6376 | 16 | Fs::DirListing { dir: website/src/website/data/playground-samples/interacting-with-the-editor/line-and-inline-decorations } |  |  | 0.564 |
| walker |  | 6392 | 16 | Fs::DirListing { dir: website/src/website/data/playground-samples/interacting-with-the-editor/listening-to-mouse-events } |  |  | 0.564 |
| walker |  | 6408 | 16 | Fs::DirListing { dir: website/src/website/data/playground-samples/interacting-with-the-editor/rendering-glyphs-in-the-margin } |  |  | 0.564 |
| walker |  | 6426 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/interacting-with-the-editor/adding-a-command-to-an-editor-instance } |  |  | 0.564 |
| walker |  | 6444 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/interacting-with-the-editor/adding-a-keybinding-to-an-existing-command } |  |  | 0.564 |
| walker |  | 6462 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/interacting-with-the-editor/adding-an-action-to-an-editor-instance } |  |  | 0.564 |
| walker |  | 6480 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/interacting-with-the-editor/customizing-the-line-numbers } |  |  | 0.564 |
| walker |  | 6498 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/interacting-with-the-editor/listening-to-key-events } |  |  | 0.564 |
| walker |  | 6516 | 18 | Fs::DirListing { dir: website/src/website/data/playground-samples/interacting-with-the-editor/revealing-a-position } |  |  | 0.564 |
| ns | 6545 |  | 229 | The four *Mode.ts entry points and the tsMode re-export tail | 5.1 |  | 0.559 |
| walker |  | 6690 | 174 | Fs::DirListing { dir: monaco-lsp-client/src/adapters/languageFeatures } |  |  | 0.559 |
| ns | 6759 |  | 214 | cssMode.setupMode — WorkerManager, the worker accessor, and ModeConfiguration-gated provider registration | 5.2 | 5.1 | 0.549 |
| walker |  | 6770 | 80 | Fs::DirListing { dir: test/manual } |  |  | 0.549 |
| walker |  | 6784 | 14 | Fs::DirListing { dir: test/manual/typescript } |  |  | 0.549 |
| walker |  | 6804 | 20 | Code::CodeKey { rung: Names, file: src/editor.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.549 |
| ns | 7112 |  | 353 | common/lspLanguageFeatures.ts — the shared provider adapters and LSP conversion helpers | 5.3 |  | 0.540 |
| walker |  | 7390 | 586 | Json::ScriptsTail { file: package.json } |  |  | 0.560 |
| walker |  | 7394 | 4 | Fs::DirListing { dir: test/smoke/amd } |  |  | 0.560 |
| ns | 7419 |  | 307 | Web-worker plumbing: MonacoEnvironment hooks, createWebWorker, IWebWorkerOptions fields, worker entry points | 5.4 |  | 0.549 |
| walker |  | 7429 | 35 | Code::CodeKey { rung: Names, file: monaco-lsp-client/src/utils.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.549 |
| walker |  | 7441 | 12 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/src/utils.ts, decl: 1, sub: 0, line: 1 } |  |  | 0.549 |
| walker |  | 7492 | 51 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/src/utils.ts, decl: 2, sub: 0, line: 5 } |  |  | 0.549 |
| walker |  | 7555 | 63 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/src/utils.ts, decl: 5, sub: 0, line: 24 } |  |  | 0.549 |
| ns | 7574 |  | 155 | WorkerManager lifecycle and the CSSWorker entry points | 5.5 |  | 0.545 |
| walker |  | 7579 | 24 | Code::CodeKey { rung: Names, file: src/languages/register.all.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.547 |
| walker |  | 7592 | 13 | Code::CodeKey { rung: Names, file: webpack-plugin/src/loader-utils.d.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.547 |
| walker |  | 7642 | 50 | Code::CodeKey { rung: Decl, file: webpack-plugin/src/loader-utils.d.ts, decl: 1, sub: 0, line: 1 } |  |  | 0.547 |
| walker |  | 7666 | 24 | Code::CodeKey { rung: Names, file: webpack-plugin/src/types.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.547 |
| walker |  | 7685 | 19 | Code::CodeKey { rung: Decl, file: webpack-plugin/src/types.ts, decl: 1, sub: 0, line: 1 } |  |  | 0.547 |
| walker |  | 7720 | 35 | Code::CodeKey { rung: Decl, file: webpack-plugin/src/types.ts, decl: 2, sub: 0, line: 6 } |  |  | 0.547 |
| ns | 7828 |  | 254 | typescript/languageFeatures.ts — the TypeScript-specific adapter roster | 5.6 |  | 0.540 |
| ns | 7907 |  | 79 | monaco-lsp-client package: complete tree listing | 6.1 |  | 0.548 |
| walker |  | 7956 | 236 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.551 |
| walker |  | 8068 | 112 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.552 |
| ns | 8114 |  | 207 | MonacoLspClient wiring + index exports | 6.2 | 6.1 | 0.547 |
| ns | 8163 |  | 49 | webpack-plugin package: complete tree listing | 6.3 |  | 0.552 |
| walker |  | 8195 | 127 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.555 |
| walker |  | 8205 | 10 | Code::CodeKey { rung: Body, file: monaco-lsp-client/src/utils.ts, decl: 4, sub: 0, line: 12 } |  |  | 0.555 |
| walker |  | 8215 | 10 | Fs::DirListing { dir: website/typedoc } |  |  | 0.555 |
| walker |  | 8400 | 185 | Plaintext::DeclSurface { file: ThirdPartyNotices.txt } |  |  | 0.555 |
| ns | 8447 |  | 284 | webpack-plugin option fields and peer-dependency contract | 6.4 | 6.3 | 0.558 |
| ns | 8611 |  | 164 | build/ and scripts/ tree listings | 7.1 |  | 0.548 |
| walker |  | 8617 | 217 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.548 |
| walker |  | 8627 | 10 | Code::CodeKey { rung: Names, file: src/deprecated/editor/editor.worker.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.548 |
| walker |  | 8637 | 10 | Code::CodeKey { rung: Names, file: src/deprecated/editor/editor.main.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.548 |
| ns | 8760 |  | 149 | build-monaco-editor.ts run() — what the published package is made of | 7.2 |  | 0.544 |
| walker |  | 8899 | 262 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.544 |
| ns | 8998 |  | 238 | check-samples.ts — the invariant every new language must satisfy | 7.3 |  | 0.537 |
| ns | 9164 |  | 166 | package.json distribution fields: typings, main, module, exports | 8.1 |  | 0.542 |
| walker |  | 9165 | 266 | Markdown::Section { file: README.md, section_index: 10, keeps_default_concavity: false } |  |  | 0.542 |
| walker |  | 9173 | 8 | Fs::DirListing { dir: website/index/samples } |  |  | 0.542 |
| walker |  | 9187 | 14 | Code::CodeKey { rung: Names, file: src/deprecated/basic-languages/monaco.contribution.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.542 |
| walker |  | 9216 | 29 | Code::CodeKey { rung: Names, file: src/internal/common/workers.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.542 |
| walker |  | 9239 | 23 | Code::CodeKey { rung: Decl, file: src/internal/common/workers.ts, decl: 1, sub: 0, line: 92 } |  |  | 0.543 |
| ns | 9418 |  | 254 | README Installing + CHANGELOG head (0.55.x breaking changes) | 8.2 |  | 0.539 |
| walker |  | 9487 | 248 | Code::CodeKey { rung: Decl, file: src/internal/common/workers.ts, decl: 2, sub: 0, line: 113 } |  |  | 0.541 |
| walker |  | 9518 | 31 | Json::Identity { file: website/package.json } |  |  | 0.541 |
| ns | 9522 |  | 104 | samples/ listing — every integration sample directory | 8.3 |  | 0.548 |
| walker |  | 9535 | 17 | Code::CodeKey { rung: Names, file: src/languages/features/register.all.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.549 |
| ns | 9560 |  | 38 | docs/ listing | 8.4 |  | 0.551 |
| walker |  | 9570 | 35 | Code::CodeKey { rung: Names, file: monaco-lsp-client/src/adapters/LspCapabilitiesRegistry.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.551 |
| walker |  | 9625 | 55 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/src/adapters/LspCapabilitiesRegistry.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.551 |
| ns | 9643 |  | 83 | integrate-esm.md section headings | 8.5 | 8.4 | 0.549 |
| walker |  | 9728 | 103 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/src/adapters/LspCapabilitiesRegistry.ts, decl: 2, sub: 0, line: 10 } |  |  | 0.549 |
| walker |  | 9748 | 20 | Code::CodeKey { rung: Decl, file: monaco-lsp-client/src/adapters/LspCapabilitiesRegistry.ts, decl: 3, sub: 0, line: 16 } |  |  | 0.549 |
| ns | 9769 |  | 126 | website/ and CI/publishing config listings | 8.6 |  | 0.559 |
| walker |  | 9785 | 37 | Code::CodeKey { rung: Names, file: webpack-plugin/src/loaders/include.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.559 |
| walker |  | 9799 | 14 | Code::CodeKey { rung: Decl, file: webpack-plugin/src/loaders/include.ts, decl: 2, sub: 0, line: 10 } |  |  | 0.559 |
| walker |  | 9833 | 34 | Code::CodeKey { rung: Decl, file: webpack-plugin/src/loaders/include.ts, decl: 1, sub: 0, line: 4 } |  |  | 0.559 |
| walker |  | 9889 | 56 | Code::CodeKey { rung: Names, file: src/languages/definitions/_.contribution.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.559 |
| walker |  | 9909 | 20 | Code::CodeKey { rung: Decl, file: src/languages/definitions/_.contribution.ts, decl: 1, sub: 0, line: 8 } |  |  | 0.560 |
| walker |  | 9947 | 38 | Code::CodeKey { rung: Names, file: webpack-plugin/src/plugins/AddWorkerEntryPointPlugin.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.560 |
| walker |  | 9975 | 28 | Code::CodeKey { rung: Decl, file: webpack-plugin/src/plugins/AddWorkerEntryPointPlugin.ts, decl: 2, sub: 0, line: 40 } |  |  | 0.560 |
| walker |  | 9998 | 23 | Code::CodeKey { rung: Decl, file: webpack-plugin/src/plugins/AddWorkerEntryPointPlugin.ts, decl: 1, sub: 0, line: 3 } |  |  | 0.560 |

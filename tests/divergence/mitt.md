Score(3000)=0.838 I=0.962 C=0.730 ns_rows≤3K=21/55 grid(1000/1442/2080/3000/4327/6240/9000)=0.741/0.803/0.816/0.838/0.859/0.907/0.838

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 29 | 29 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 32 |  | 32 | README title and one-line tagline | 1.1 |  | 0.000 |
| walker |  | 33 | 4 | Fs::DirListing { dir: src } |  |  | 0.000 |
| ns | 61 |  | 29 | Complete root directory listing | 1.2 |  | 0.572 |
| ns | 150 |  | 89 | Every exported name in src/index.ts (names only) | 1.3 |  | 0.420 |
| walker |  | 220 | 187 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.763 |
| ns | 266 |  | 116 | README feature bullets | 1.4 |  | 0.785 |
| walker |  | 284 | 64 | Json::Identity { file: package.json } |  |  | 0.788 |
| walker |  | 294 | 10 | Fs::DirListing { dir: .github } |  |  | 0.792 |
| ns | 295 |  | 29 | README: runtime support and dependency claim | 1.5 |  | 0.795 |
| walker |  | 303 | 9 | Fs::DirListing { dir: .github/workflows } |  |  | 0.803 |
| walker |  | 315 | 12 | Fs::DirListing { dir: test } |  |  | 0.816 |
| ns | 330 |  | 35 | Complete listings of src/, test/, .github/ and .github/workflows/ | 1.6 |  | 0.827 |
| ns | 397 |  | 67 | All seven README `##` section headings | 1.7 |  | 0.736 |
| walker |  | 408 | 93 | Json::Scripts { file: package.json } |  |  | 0.737 |
| ns | 540 |  | 143 | package.json identity and legacy entry-point fields | 1.8 |  | 0.659 |
| walker |  | 552 | 144 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.757 |
| walker |  | 552 | 0 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.757 |
| ns | 627 |  | 87 | package.json `exports` conditional map | 1.9 | 1.8 | 0.693 |
| walker |  | 696 | 144 | Json::ScriptsTail { file: package.json } |  |  | 0.696 |
| walker |  | 769 | 73 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.697 |
| ns | 826 |  | 199 | src/index.ts: the six exported type aliases in full | 1.10 | 1.3 | 0.580 |
| walker |  | 979 | 210 | Json::Entry { file: package.json } |  |  | 0.739 |
| ns | 1032 |  | 206 | The complete `Emitter<Events>` interface with all overloads | 2.1 | 1.3 | 0.658 |
| ns | 1110 |  | 78 | `mitt()` factory JSDoc and signature | 2.2 | 1.3 | 0.628 |
| walker |  | 1139 | 160 | Code::CodeKey { rung: Names, file: src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.736 |
| walker |  | 1153 | 14 | Code::CodeKey { rung: Decl, file: src/index.ts, decl: 5, sub: 0, line: 13 } |  |  | 0.744 |
| walker |  | 1176 | 23 | Code::CodeKey { rung: Decl, file: src/index.ts, decl: 8, sub: 0, line: 46 } |  |  | 0.753 |
| walker |  | 1204 | 28 | Code::CodeKey { rung: Decl, file: src/index.ts, decl: 3, sub: 0, line: 6 } |  |  | 0.768 |
| walker |  | 1239 | 35 | Code::CodeKey { rung: Decl, file: src/index.ts, decl: 6, sub: 0, line: 18 } |  |  | 0.792 |
| ns | 1332 |  | 222 | README Usage code block | 2.3 |  | 0.707 |
| walker |  | 1436 | 197 | Code::CodeKey { rung: Decl, file: src/index.ts, decl: 7, sub: 0, line: 23 } |  |  | 0.803 |
| walker |  | 1450 | 14 | Code::CodeKey { rung: Doc, file: src/index.ts, decl: 6, sub: 0, line: 18 } |  |  | 0.814 |
| walker |  | 1465 | 15 | Code::CodeKey { rung: Doc, file: src/index.ts, decl: 4, sub: 0, line: 12 } |  |  | 0.826 |
| walker |  | 1490 | 25 | Code::CodeKey { rung: Doc, file: src/index.ts, decl: 2, sub: 0, line: 5 } |  |  | 0.848 |
| ns | 1533 |  | 201 | README `### Typescript` section: strict mode and inference example | 2.4 |  | 0.784 |
| walker |  | 1536 | 46 | Code::CodeKey { rung: Doc, file: src/index.ts, decl: 8, sub: 0, line: 46 } |  |  | 0.815 |
| walker |  | 1637 | 101 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.815 |
| ns | 1638 |  | 105 | README: annotating a variable with the exported `Emitter` type | 2.5 |  | 0.778 |
| walker |  | 1867 | 230 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.873 |
| ns | 1913 |  | 275 | README Install section body | 2.6 |  | 0.806 |
| ns | 2007 |  | 94 | Factory body preamble: `GenericEventHandler`, default Map, `all` member | 3.1 |  | 0.778 |
| walker |  | 2147 | 280 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.858 |
| ns | 2208 |  | 201 | `on()` implementation with JSDoc | 3.2 | 3.1 | 0.823 |
| walker |  | 2445 | 298 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.917 |
| ns | 2448 |  | 240 | `off()` implementation with JSDoc | 3.3 |  | 0.875 |
| ns | 2585 |  | 137 | `emit()` JSDoc, including the wildcard-ordering contract | 3.4 |  | 0.853 |
| walker |  | 2747 | 302 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.855 |
| ns | 2799 |  | 214 | `emit()` body and the close of the factory | 3.5 | 3.4 | 0.813 |
| walker |  | 2910 | 163 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.814 |
| ns | 3034 |  | 235 | Complete package.json `scripts` block | 4.1 | 1.9 | 0.819 |
| walker |  | 3121 | 211 | Code::CodeKey { rung: Body, file: src/index.ts, decl: 8, sub: 0, line: 46 } |  |  | 0.858 |
| ns | 3183 |  | 149 | tsconfig.json in full | 4.2 |  | 0.831 |
| ns | 3284 |  | 101 | package.json `mocha` configuration block | 4.3 |  | 0.811 |
| walker |  | 3292 | 171 | Code::CodeKey { rung: Body, file: src/index.ts, decl: 8, sub: 1, line: 46 } |  |  | 0.837 |
| walker |  | 3481 | 189 | Code::CodeKey { rung: Body, file: src/index.ts, decl: 8, sub: 2, line: 46 } |  |  | 0.873 |
| ns | 3501 |  | 217 | .github/workflows/main.yml in full | 4.4 |  | 0.832 |
| ns | 3658 |  | 157 | package.json project metadata: repository, keywords, homepage, authors, license, files | 4.5 |  | 0.811 |
| walker |  | 3662 | 181 | Code::CodeKey { rung: Body, file: src/index.ts, decl: 8, sub: 3, line: 46 } |  |  | 0.834 |
| ns | 3736 |  | 78 | test/index_test.ts imports and chai setup | 5.1 |  | 0.825 |
| walker |  | 3808 | 146 | Code::CodeKey { rung: Body, file: src/index.ts, decl: 8, sub: 4, line: 46 } |  |  | 0.861 |
| walker |  | 3957 | 149 | Plaintext::Rest { file: tsconfig.json } |  |  | 0.889 |
| ns | 4153 |  | 417 | Every `describe` / `it` declaration line in test/index_test.ts | 5.2 |  | 0.854 |
| ns | 4355 |  | 202 | test/test-types-compilation.ts preamble: typed emitter and handler fixtures | 5.3 |  | 0.826 |
| ns | 4540 |  | 185 | index_test shared fixture: the `Events` map and `beforeEach` | 5.4 | 5.2 | 0.804 |
| ns | 4736 |  | 196 | Type test: `on` argument inference, including the `'*'` cases | 5.5 | 5.3 | 0.784 |
| walker |  | 4762 | 805 | Plaintext::Rest { file: test/test-types-compilation.ts } |  |  | 0.850 |
| ns | 4932 |  | 196 | Type test: `off` argument inference | 5.6 |  | 0.853 |
| ns | 5128 |  | 196 | Type test: `emit` argument inference and optional-payload rules | 5.7 |  | 0.856 |
| ns | 5281 |  | 153 | Tests: default export is a function; optional pre-seeded handler map | 6.1 | 5.2 | 0.838 |
| ns | 5452 |  | 171 | Tests: `all` property exposure and `on` registering new / arbitrary types | 6.2 | 5.2 | 0.815 |
| ns | 5679 |  | 227 | Tests: `on` appends to an existing type and does NOT normalize case | 6.3 | 5.2 | 0.796 |
| ns | 5838 |  | 159 | Tests: symbol event types and duplicate listener registration | 6.4 | 5.2 | 0.783 |
| ns | 6120 |  | 282 | Tests: `off` removes a handler and preserves case distinctions | 6.5 | 5.2 | 0.760 |
| ns | 6338 |  | 218 | Tests: `off` removes only the first match; `off(type)` clears the type | 6.6 | 5.2 | 0.744 |
| ns | 6613 |  | 275 | Tests: `emit` invokes the type handler with exactly one argument, case-sensitively | 6.7 | 5.2 | 0.724 |
| ns | 6777 |  | 164 | Tests: `*` handlers receive `(type, event)` for every emit | 6.8 | 5.2 | 0.714 |
| walker |  | 6873 | 2111 | Plaintext::Rest { file: test/index_test.ts } |  |  | 0.915 |
| ns | 6919 |  | 142 | README API section: generated-by marker and its table of contents | 7.1 |  | 0.906 |
| ns | 7025 |  | 106 | README Examples & Demos section | 7.2 |  | 0.904 |
| ns | 7257 |  | 232 | README generated API reference: `mitt`, `all`, `on` | 7.3 |  | 0.904 |
| ns | 7600 |  | 343 | README generated API reference: `off`, `emit` | 7.4 |  | 0.903 |
| ns | 7912 |  | 312 | README Contribute section: issue reporting and the PR checklist | 7.5 |  | 0.888 |
| ns | 7943 |  | 31 | README License line | 7.6 |  | 0.887 |
| ns | 8311 |  | 368 | package.json devDependencies in full | 7.7 |  | 0.871 |
| ns | 8536 |  | 225 | .eslintrc: ignore patterns, extended configs, parser, env, globals | 7.8 |  | 0.854 |
| ns | 8808 |  | 272 | .eslintrc rules block in full | 7.9 | 7.8 | 0.838 |
| ns | 9012 |  | 204 | Formatting config: package.json `prettier` block and .editorconfig | 7.10 |  | 0.824 |
| ns | 9132 |  | 120 | .github/workflows/compressed-size.yml in full | 7.11 |  | 0.818 |
| ns | 9206 |  | 74 | .gitignore in full | 7.12 |  | 0.813 |
| ns | 9389 |  | 183 | README badge header | 7.13 |  | 0.809 |
| ns | 9601 |  | 212 | .github/PULL_REQUEST_TEMPLATE.md in full | 7.14 |  | 0.798 |

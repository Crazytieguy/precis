Score(3000)=0.789 I=0.938 C=0.663 ns_rows≤3K=21/55 grid(1000/1442/2080/3000/4327/6240/9000)=0.730/0.711/0.748/0.789/0.726/0.711/0.703

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 29 | 29 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 32 |  | 32 | README title and one-line tagline | 1.1 |  | 0.000 |
| walker |  | 33 | 4 | Fs::DirListing { dir: src } |  |  | 0.000 |
| ns | 61 |  | 29 | Complete root directory listing | 1.2 |  | 0.572 |
| walker |  | 97 | 64 | Json::Identity { file: package.json } |  |  | 0.577 |
| walker |  | 107 | 10 | Fs::DirListing { dir: .github } |  |  | 0.585 |
| walker |  | 116 | 9 | Fs::DirListing { dir: .github/workflows } |  |  | 0.600 |
| walker |  | 128 | 12 | Fs::DirListing { dir: test } |  |  | 0.622 |
| ns | 150 |  | 89 | Every exported name in src/index.ts (names only) | 1.3 |  | 0.457 |
| ns | 266 |  | 116 | README feature bullets | 1.4 |  | 0.400 |
| ns | 295 |  | 29 | README: runtime support and dependency claim | 1.5 |  | 0.390 |
| walker |  | 315 | 187 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.816 |
| ns | 330 |  | 35 | Complete listings of src/, test/, .github/ and .github/workflows/ | 1.6 |  | 0.827 |
| ns | 397 |  | 67 | All seven README `##` section headings | 1.7 |  | 0.736 |
| walker |  | 459 | 144 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.850 |
| walker |  | 490 | 31 | Markdown::Section { file: README.md, section_index: 10, keeps_default_concavity: false } |  |  | 0.851 |
| ns | 540 |  | 143 | package.json identity and legacy entry-point fields | 1.8 |  | 0.757 |
| walker |  | 605 | 115 | Json::IdentityMeta { file: package.json } |  |  | 0.760 |
| ns | 627 |  | 87 | package.json `exports` conditional map | 1.9 | 1.8 | 0.695 |
| walker |  | 688 | 83 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.695 |
| ns | 826 |  | 199 | src/index.ts: the six exported type aliases in full | 1.10 | 1.3 | 0.579 |
| walker |  | 900 | 212 | Json::Entry { file: package.json } |  |  | 0.730 |
| ns | 1032 |  | 206 | The complete `Emitter<Events>` interface with all overloads | 2.1 | 1.3 | 0.650 |
| ns | 1110 |  | 78 | `mitt()` factory JSDoc and signature | 2.2 | 1.3 | 0.620 |
| walker |  | 1133 | 233 | Json::Scripts { file: package.json } |  |  | 0.631 |
| walker |  | 1293 | 160 | Code::CodeKey { rung: Names, file: src/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.739 |
| walker |  | 1307 | 14 | Code::CodeKey { rung: Decl, file: src/index.ts, decl: 5, sub: 0, line: 13 } |  |  | 0.747 |
| walker |  | 1330 | 23 | Code::CodeKey { rung: Decl, file: src/index.ts, decl: 8, sub: 0, line: 46 } |  |  | 0.756 |
| ns | 1332 |  | 222 | README Usage code block | 2.3 |  | 0.676 |
| walker |  | 1358 | 28 | Code::CodeKey { rung: Decl, file: src/index.ts, decl: 3, sub: 0, line: 6 } |  |  | 0.689 |
| walker |  | 1393 | 35 | Code::CodeKey { rung: Decl, file: src/index.ts, decl: 6, sub: 0, line: 18 } |  |  | 0.711 |
| ns | 1533 |  | 201 | README `### Typescript` section: strict mode and inference example | 2.4 |  | 0.657 |
| walker |  | 1590 | 197 | Code::CodeKey { rung: Decl, file: src/index.ts, decl: 7, sub: 0, line: 23 } |  |  | 0.746 |
| walker |  | 1636 | 46 | Code::CodeKey { rung: Doc, file: src/index.ts, decl: 8, sub: 0, line: 46 } |  |  | 0.777 |
| ns | 1638 |  | 105 | README: annotating a variable with the exported `Emitter` type | 2.5 |  | 0.741 |
| walker |  | 1747 | 111 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.742 |
| ns | 1913 |  | 275 | README Install section body | 2.6 |  | 0.685 |
| walker |  | 1977 | 230 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.774 |
| ns | 2007 |  | 94 | Factory body preamble: `GenericEventHandler`, default Map, `all` member | 3.1 |  | 0.748 |
| ns | 2208 |  | 201 | `on()` implementation with JSDoc | 3.2 | 3.1 | 0.718 |
| walker |  | 2275 | 298 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: true } |  |  | 0.817 |
| ns | 2448 |  | 240 | `off()` implementation with JSDoc | 3.3 |  | 0.780 |
| walker |  | 2555 | 280 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.850 |
| ns | 2585 |  | 137 | `emit()` JSDoc, including the wildcard-ordering contract | 3.4 |  | 0.828 |
| walker |  | 2767 | 212 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.830 |
| ns | 2799 |  | 214 | `emit()` body and the close of the factory | 3.5 | 3.4 | 0.789 |
| ns | 3034 |  | 235 | Complete package.json `scripts` block | 4.1 | 1.9 | 0.795 |
| walker |  | 3069 | 302 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: true } |  |  | 0.797 |
| ns | 3183 |  | 149 | tsconfig.json in full | 4.2 |  | 0.772 |
| walker |  | 3232 | 163 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.773 |
| ns | 3284 |  | 101 | package.json `mocha` configuration block | 4.3 |  | 0.755 |
| walker |  | 3381 | 149 | Json::Whole { file: tsconfig.json } |  |  | 0.789 |
| ns | 3501 |  | 217 | .github/workflows/main.yml in full | 4.4 |  | 0.753 |
| ns | 3658 |  | 157 | package.json project metadata: repository, keywords, homepage, authors, license, files | 4.5 |  | 0.761 |
| walker |  | 3676 | 295 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.763 |
| ns | 3736 |  | 78 | test/index_test.ts imports and chai setup | 5.1 |  | 0.755 |
| walker |  | 3796 | 120 | Plaintext::Whole { file: .github/workflows/compressed-size.yml } |  |  | 0.756 |
| ns | 4153 |  | 417 | Every `describe` / `it` declaration line in test/index_test.ts | 5.2 |  | 0.726 |
| ns | 4355 |  | 202 | test/test-types-compilation.ts preamble: typed emitter and handler fixtures | 5.3 |  | 0.703 |
| ns | 4540 |  | 185 | index_test shared fixture: the `Events` map and `beforeEach` | 5.4 | 5.2 | 0.683 |
| walker |  | 4694 | 898 | Code::CodeKey { rung: Body, file: src/index.ts, decl: 8, sub: 0, line: 46 } |  |  | 0.814 |
| ns | 4736 |  | 196 | Type test: `on` argument inference, including the `'*'` cases | 5.5 | 5.3 | 0.793 |
| walker |  | 4911 | 217 | Plaintext::Whole { file: .github/workflows/main.yml } |  |  | 0.833 |
| walker |  | 4930 | 19 | Markdown::Section { file: .github/PULL_REQUEST_TEMPLATE.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.833 |
| ns | 4932 |  | 196 | Type test: `off` argument inference | 5.6 |  | 0.813 |
| walker |  | 4946 | 16 | Markdown::Section { file: .github/PULL_REQUEST_TEMPLATE.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.813 |
| walker |  | 5020 | 74 | Plaintext::Whole { file: .gitignore } |  |  | 0.814 |
| walker |  | 5048 | 28 | Markdown::Section { file: .github/PULL_REQUEST_TEMPLATE.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.814 |
| ns | 5128 |  | 196 | Type test: `emit` argument inference and optional-payload rules | 5.7 |  | 0.795 |
| walker |  | 5158 | 110 | Markdown::Section { file: .github/PULL_REQUEST_TEMPLATE.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.795 |
| ns | 5281 |  | 153 | Tests: default export is a function; optional pre-seeded handler map | 6.1 | 5.2 | 0.779 |
| walker |  | 5323 | 165 | Plaintext::Whole { file: .editorconfig } |  |  | 0.780 |
| ns | 5452 |  | 171 | Tests: `all` property exposure and `on` registering new / arbitrary types | 6.2 | 5.2 | 0.758 |
| walker |  | 5632 | 309 | Plaintext::Whole { file: LICENSE } |  |  | 0.758 |
| ns | 5679 |  | 227 | Tests: `on` appends to an existing type and does NOT normalize case | 6.3 | 5.2 | 0.741 |
| ns | 5838 |  | 159 | Tests: symbol event types and duplicate listener registration | 6.4 | 5.2 | 0.729 |
| ns | 6120 |  | 282 | Tests: `off` removes a handler and preserves case distinctions | 6.5 | 5.2 | 0.707 |
| walker |  | 6132 | 500 | Plaintext::Whole { file: .eslintrc } |  |  | 0.711 |
| ns | 6338 |  | 218 | Tests: `off` removes only the first match; `off(type)` clears the type | 6.6 | 5.2 | 0.696 |
| ns | 6613 |  | 275 | Tests: `emit` invokes the type handler with exactly one argument, case-sensitively | 6.7 | 5.2 | 0.678 |
| ns | 6777 |  | 164 | Tests: `*` handlers receive `(type, event)` for every emit | 6.8 | 5.2 | 0.668 |
| ns | 6919 |  | 142 | README API section: generated-by marker and its table of contents | 7.1 |  | 0.674 |
| ns | 7025 |  | 106 | README Examples & Demos section | 7.2 |  | 0.677 |
| ns | 7257 |  | 232 | README generated API reference: `mitt`, `all`, `on` | 7.3 |  | 0.683 |
| ns | 7600 |  | 343 | README generated API reference: `off`, `emit` | 7.4 |  | 0.691 |
| ns | 7912 |  | 312 | README Contribute section: issue reporting and the PR checklist | 7.5 |  | 0.698 |
| ns | 7943 |  | 31 | README License line | 7.6 |  | 0.699 |
| ns | 8311 |  | 368 | package.json devDependencies in full | 7.7 |  | 0.686 |
| ns | 8536 |  | 225 | .eslintrc: ignore patterns, extended configs, parser, env, globals | 7.8 |  | 0.695 |
| ns | 8808 |  | 272 | .eslintrc rules block in full | 7.9 | 7.8 | 0.703 |
| ns | 9012 |  | 204 | Formatting config: package.json `prettier` block and .editorconfig | 7.10 |  | 0.703 |
| ns | 9132 |  | 120 | .github/workflows/compressed-size.yml in full | 7.11 |  | 0.707 |
| ns | 9206 |  | 74 | .gitignore in full | 7.12 |  | 0.709 |
| ns | 9389 |  | 183 | README badge header | 7.13 |  | 0.706 |
| ns | 9601 |  | 212 | .github/PULL_REQUEST_TEMPLATE.md in full | 7.14 |  | 0.705 |

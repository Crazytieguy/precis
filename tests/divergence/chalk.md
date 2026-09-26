Score(3000)=0.692 I=0.862 C=0.555 ns_rows≤3K=21/59 grid(1000/1442/2080/3000/4327/6240/9000)=0.867/0.826/0.761/0.692/0.746/0.702/0.599

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 46 | 46 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 54 |  | 54 | package.json name, version, description, license | 1.1 |  | 0.000 |
| walker |  | 68 | 22 | Fs::DirListing { dir: source } |  |  | 0.000 |
| walker |  | 77 | 9 | Fs::DirListing { dir: source/vendor } |  |  | 0.000 |
| walker |  | 86 | 9 | Fs::DirListing { dir: source/vendor/ansi-styles } |  |  | 0.000 |
| walker |  | 98 | 12 | Fs::DirListing { dir: media } |  |  | 0.000 |
| ns | 100 |  | 46 | Complete repository root listing | 1.2 |  | 0.613 |
| ns | 172 |  | 72 | package.json entry points: `main`, `exports`, `repository`, `type` | 1.3 |  | 0.529 |
| walker |  | 203 | 105 | Markdown::ReadmeHeadline { file: readme.md } |  |  | 0.530 |
| walker |  | 221 | 18 | Fs::DirListing { dir: source/vendor/supports-color } |  |  | 0.562 |
| ns | 230 |  | 58 | Complete `source/` tree including both vendored packages | 1.4 |  | 0.586 |
| walker |  | 287 | 66 | Json::Identity { file: package.json } |  |  | 0.909 |
| walker |  | 295 | 8 | Fs::DirListing { dir: examples } |  |  | 0.910 |
| walker |  | 306 | 11 | Fs::DirListing { dir: .github } |  |  | 0.913 |
| walker |  | 310 | 4 | Fs::DirListing { dir: .github/workflows } |  |  | 0.915 |
| walker |  | 340 | 30 | Json::Scripts { file: package.json } |  |  | 0.915 |
| walker |  | 361 | 21 | Json::ScriptsTail { file: package.json } |  |  | 0.767 |
| ns | 361 |  | 131 | readme tagline + every `##` section heading | 1.5 |  | 0.767 |
| walker |  | 405 | 44 | Json::Runtime { file: package.json } |  |  | 0.770 |
| walker |  | 432 | 27 | Fs::DirListing { dir: test } |  |  | 0.781 |
| ns | 568 |  | 207 | package.json `imports` subpath map, `types`, `engines`, `scripts` | 1.6 |  | 0.684 |
| walker |  | 669 | 237 | Code::CodeKey { rung: Names, file: source/index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.698 |
| walker |  | 684 | 15 | Code::CodeKey { rung: Decl, file: source/index.js, decl: 1, sub: 0, line: 34 } |  |  | 0.698 |
| ns | 708 |  | 140 | readme `###` subsection headings | 1.7 | 1.5 | 0.641 |
| ns | 770 |  | 62 | Complete listing of `test/`, `examples/`, `.github/`, `media/` | 1.8 |  | 0.687 |
| walker |  | 944 | 260 | Markdown::HeadingsOutline { file: readme.md } |  |  | 0.867 |
| walker |  | 968 | 24 | Markdown::Section { file: readme.md, section_index: 12, keeps_default_concavity: false } |  |  | 0.867 |
| walker |  | 1005 | 37 | Markdown::Section { file: readme.md, section_index: 13, keeps_default_concavity: false } |  |  | 0.867 |
| ns | 1030 |  | 260 | CI workflow in full | 1.9 |  | 0.747 |
| walker |  | 1180 | 175 | Json::Entry { file: package.json } |  |  | 0.845 |
| walker |  | 1189 | 9 | Code::CodeKey { rung: Body, file: source/index.js, decl: 3, sub: 0, line: 50 } |  |  | 0.845 |
| ns | 1244 |  | 214 | Runtime export surface of `source/index.js` | 2.1 |  | 0.854 |
| walker |  | 1280 | 91 | Markdown::Section { file: readme.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.854 |
| ns | 1358 |  | 114 | `source/index.d.ts` top-level declaration roster | 2.2 |  | 0.826 |
| walker |  | 1420 | 140 | Markdown::Section { file: readme.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.826 |
| ns | 1458 |  | 100 | readme: chaining semantics of `chalk.<style>[.<style>...]` | 2.3 | 1.7 | 0.804 |
| ns | 1686 |  | 228 | readme: `chalk.level` and the 0–3 colour-level table | 2.4 | 1.7 | 0.749 |
| walker |  | 1804 | 384 | Code::CodeKey { rung: Names, file: source/index.d.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.784 |
| ns | 1826 |  | 140 | `Options` interface body with level documentation | 2.5 | 2.2 | 0.752 |
| walker |  | 1944 | 140 | Code::CodeKey { rung: Decl, file: source/index.d.ts, decl: 1, sub: 0, line: 12 } |  |  | 0.798 |
| ns | 1970 |  | 144 | `ChalkInstance` call signature, `level`, and all six colour-model methods | 2.6 | 2.2 | 0.760 |
| ns | 2094 |  | 124 | All ten `ChalkInstance` modifier properties | 2.7 | 2.6 | 0.722 |
| walker |  | 2209 | 265 | Code::CodeKey { rung: Decl, file: source/index.d.ts, decl: 3, sub: 0, line: 32 } |  |  | 0.727 |
| ns | 2293 |  | 199 | All foreground colour properties, including the `gray`/`grey` aliases | 2.8 | 2.7 | 0.690 |
| ns | 2517 |  | 224 | All background colour properties and the close of `ChalkInstance` | 2.9 | 2.8 | 0.656 |
| walker |  | 2551 | 342 | Code::CodeKey { rung: Decl, file: source/index.d.ts, decl: 3, sub: 1, line: 32 } |  |  | 0.675 |
| ns | 2767 |  | 250 | readme: `supportsColor`, the `--color`/`FORCE_COLOR` overrides, `chalkStderr` | 2.10 | 1.7 | 0.659 |
| walker |  | 2783 | 232 | Code::CodeKey { rung: Decl, file: source/index.d.ts, decl: 3, sub: 2, line: 32 } |  |  | 0.676 |
| ns | 2832 |  | 65 | readme: the exported style-name arrays and their use | 2.11 | 1.7 | 0.669 |
| ns | 2962 |  | 130 | `index.d.ts` type re-export blocks from the vendored packages | 2.12 |  | 0.679 |
| walker |  | 3039 | 256 | Code::CodeKey { rung: Decl, file: source/index.d.ts, decl: 3, sub: 3, line: 32 } |  |  | 0.697 |
| ns | 3093 |  | 131 | Deprecated type/const aliases in `index.d.ts` | 2.13 | 2.2 | 0.688 |
| walker |  | 3264 | 225 | Code::CodeKey { rung: Decl, file: source/index.d.ts, decl: 3, sub: 4, line: 32 } |  |  | 0.717 |
| ns | 3267 |  | 174 | ansi-styles: the `styles.modifier` code table | 3.1 |  | 0.701 |
| walker |  | 3484 | 220 | Code::CodeKey { rung: Decl, file: source/index.d.ts, decl: 3, sub: 5, line: 32 } |  |  | 0.740 |
| ns | 3571 |  | 304 | ansi-styles: the `styles.color` foreground code table | 3.2 | 3.1 | 0.713 |
| walker |  | 3771 | 287 | Code::CodeKey { rung: Decl, file: source/index.d.ts, decl: 3, sub: 6, line: 32 } |  |  | 0.760 |
| walker |  | 3789 | 18 | Code::CodeKey { rung: Doc, file: source/index.d.ts, decl: 2, sub: 0, line: 30 } |  |  | 0.760 |
| walker |  | 3823 | 34 | Code::CodeKey { rung: Doc, file: source/index.d.ts, decl: 12, sub: 0, line: 302 } |  |  | 0.762 |
| walker |  | 3859 | 36 | Code::CodeKey { rung: Doc, file: source/index.d.ts, decl: 13, sub: 0, line: 309 } |  |  | 0.764 |
| ns | 3885 |  | 314 | ansi-styles: the `styles.bgColor` background code table | 3.3 | 3.2 | 0.736 |
| walker |  | 3895 | 36 | Code::CodeKey { rung: Doc, file: source/index.d.ts, decl: 14, sub: 0, line: 316 } |  |  | 0.738 |
| walker |  | 3938 | 43 | Code::CodeKey { rung: Doc, file: source/index.d.ts, decl: 15, sub: 0, line: 323 } |  |  | 0.739 |
| ns | 3950 |  | 65 | ansi-styles: the four exported style-name arrays | 3.4 | 3.3 | 0.735 |
| walker |  | 4107 | 169 | Markdown::Section { file: readme.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.753 |
| walker |  | 4157 | 50 | Code::CodeKey { rung: Doc, file: source/index.d.ts, decl: 8, sub: 0, line: 268 } |  |  | 0.759 |
| ns | 4159 |  | 209 | readme: modifier list with human descriptions | 3.5 | 1.7 | 0.745 |
| walker |  | 4183 | 26 | Code::CodeKey { rung: Body, file: source/index.js, decl: 2, sub: 0, line: 35 } |  |  | 0.745 |
| walker |  | 4229 | 46 | Code::CodeKey { rung: Names, file: source/utilities.js, decl: 0, sub: 0, line: 0 } |  |  | 0.746 |
| walker |  | 4248 | 19 | Code::CodeKey { rung: Doc, file: source/utilities.js, decl: 1, sub: 0, line: 2 } |  |  | 0.746 |
| walker |  | 4431 | 183 | Markdown::Section { file: readme.md, section_index: 10, keeps_default_concavity: false } |  |  | 0.746 |
| ns | 4467 |  | 308 | readme: 256/Truecolor section and the `rgb`/`hex`/`ansi256` colour models | 3.6 | 1.5 | 0.736 |
| ns | 4561 |  | 94 | `source/index.js` imports and the stdout/stderr colour split | 4.1 |  | 0.728 |
| walker |  | 4640 | 209 | Markdown::Section { file: readme.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.747 |
| walker |  | 4713 | 73 | Code::CodeKey { rung: Doc, file: source/index.d.ts, decl: 9, sub: 0, line: 277 } |  |  | 0.749 |
| walker |  | 4786 | 73 | Code::CodeKey { rung: Doc, file: source/index.d.ts, decl: 10, sub: 0, line: 286 } |  |  | 0.751 |
| ns | 4843 |  | 282 | `source/index.js`: complete top-level declaration roster | 4.2 |  | 0.732 |
| walker |  | 4866 | 80 | Code::CodeKey { rung: Doc, file: source/index.d.ts, decl: 11, sub: 0, line: 295 } |  |  | 0.734 |
| ns | 4982 |  | 139 | `Chalk` class, `chalkFactory`, `createChalk` | 4.3 | 4.2 | 0.721 |
| walker |  | 5072 | 206 | Markdown::Section { file: readme.md, section_index: 14, keeps_default_concavity: false } |  |  | 0.721 |
| ns | 5100 |  | 118 | `applyOptions`: level validation and auto-detection | 4.4 | 4.2 | 0.714 |
| ns | 5270 |  | 170 | Style-property generation loop, `visible`, and prototype installation | 4.5 | 4.2 | 0.700 |
| walker |  | 5307 | 235 | Markdown::Section { file: readme.md, section_index: 11, keeps_default_concavity: false } |  |  | 0.715 |
| ns | 5461 |  | 191 | `createBuilder`: the chainable callable | 4.6 | 4.2 | 0.703 |
| walker |  | 5521 | 214 | Markdown::Section { file: readme.md, section_index: 15, keeps_default_concavity: false } |  |  | 0.703 |
| ns | 5607 |  | 146 | `createStyler`: the open/close linked list | 4.7 | 4.2 | 0.689 |
| walker |  | 5614 | 93 | Code::CodeKey { rung: Doc, file: source/index.d.ts, decl: 4, sub: 0, line: 242 } |  |  | 0.689 |
| walker |  | 5653 | 39 | Code::CodeKey { rung: Names, file: source/vendor/supports-color/index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.689 |
| walker |  | 5700 | 47 | Code::CodeKey { rung: Decl, file: source/vendor/supports-color/index.js, decl: 2, sub: 0, line: 185 } |  |  | 0.689 |
| ns | 5907 |  | 300 | `applyStyle`: the string-wrapping algorithm | 4.8 | 4.2 | 0.667 |
| ns | 6000 |  | 93 | `proto` and the `level` getter/setter delegation | 4.9 | 4.2 | 0.659 |
| walker |  | 6018 | 318 | Markdown::Section { file: readme.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.690 |
| ns | 6056 |  | 56 | `levelMapping`: numeric level → ansi-styles method name | 4.10 | 4.2 | 0.685 |
| ns | 6255 |  | 199 | `getModelAnsi`: RGB/hex downsampling dispatch | 4.11 | 4.2 | 0.673 |
| walker |  | 6266 | 248 | Markdown::Section { file: readme.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.689 |
| ns | 6528 |  | 273 | Colour-model property generation for `rgb`/`hex`/`ansi256` and their `bg*` twins | 4.12 | 4.2 | 0.674 |
| walker |  | 6591 | 325 | Markdown::Section { file: readme.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.674 |
| ns | 6593 |  | 65 | `source/utilities.js`: both exported helpers, signatures only | 4.13 |  | 0.676 |
| walker |  | 6699 | 108 | Code::CodeKey { rung: Names, file: source/vendor/ansi-styles/index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.681 |
| ns | 6816 |  | 223 | supports-color: complete declaration roster and default export | 5.1 |  | 0.673 |
| walker |  | 6934 | 235 | Code::CodeKey { rung: Names, file: source/vendor/ansi-styles/index.d.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.674 |
| ns | 6960 |  | 144 | supports-color: the `--color` / `--no-color` flag block | 5.2 | 5.1 | 0.664 |
| walker |  | 7005 | 71 | Code::CodeKey { rung: Decl, file: source/vendor/ansi-styles/index.d.ts, decl: 15, sub: 0, line: 229 } |  |  | 0.665 |
| walker |  | 7085 | 80 | Code::CodeKey { rung: Decl, file: source/vendor/ansi-styles/index.d.ts, decl: 1, sub: 0, line: 1 } |  |  | 0.665 |
| ns | 7095 |  | 135 | supports-color: `envForceColor` (`FORCE_COLOR` parsing) | 5.3 | 5.1 | 0.657 |
| ns | 7185 |  | 90 | supports-color: `translateLevel` — the `ColorInfo` shape | 5.4 | 5.1 | 0.651 |
| walker |  | 7186 | 101 | Code::CodeKey { rung: Decl, file: source/vendor/ansi-styles/index.d.ts, decl: 2, sub: 0, line: 13 } |  |  | 0.651 |
| ns | 7409 |  | 224 | `_supportsColor`: force-colour precedence and explicit `--color=` levels | 5.5 | 5.1 | 0.638 |
| walker |  | 7458 | 272 | Code::CodeKey { rung: Decl, file: source/vendor/ansi-styles/index.d.ts, decl: 4, sub: 0, line: 75 } |  |  | 0.638 |
| walker |  | 7752 | 294 | Code::CodeKey { rung: Decl, file: source/vendor/ansi-styles/index.d.ts, decl: 5, sub: 0, line: 105 } |  |  | 0.638 |
| ns | 7848 |  | 439 | `_supportsColor`: TTY, Windows and CI detection | 5.6 | 5.5 | 0.618 |
| walker |  | 8154 | 402 | Code::CodeKey { rung: Decl, file: source/vendor/ansi-styles/index.d.ts, decl: 3, sub: 0, line: 26 } |  |  | 0.618 |
| walker |  | 8170 | 16 | Code::CodeKey { rung: Doc, file: source/vendor/ansi-styles/index.d.ts, decl: 11, sub: 0, line: 212 } |  |  | 0.618 |
| walker |  | 8187 | 17 | Code::CodeKey { rung: Doc, file: source/vendor/ansi-styles/index.d.ts, decl: 12, sub: 0, line: 217 } |  |  | 0.618 |
| walker |  | 8204 | 17 | Code::CodeKey { rung: Doc, file: source/vendor/ansi-styles/index.d.ts, decl: 13, sub: 0, line: 222 } |  |  | 0.618 |
| ns | 8249 |  | 401 | `_supportsColor`: TeamCity, `COLORTERM` and terminal-emulator sniffing | 5.7 | 5.6 | 0.605 |
| ns | 8309 |  | 60 | supports-color: `createSupportsColor` body | 5.8 | 5.1 | 0.602 |
| walker |  | 8319 | 115 | Code::CodeKey { rung: Names, file: source/vendor/supports-color/index.d.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.602 |
| walker |  | 8342 | 23 | Code::CodeKey { rung: Decl, file: source/vendor/supports-color/index.d.ts, decl: 6, sub: 0, line: 50 } |  |  | 0.602 |
| walker |  | 8411 | 69 | Code::CodeKey { rung: Decl, file: source/vendor/supports-color/index.d.ts, decl: 1, sub: 0, line: 3 } |  |  | 0.603 |
| ns | 8474 |  | 165 | supports-color `index.d.ts`: complete type roster | 5.9 |  | 0.608 |
| walker |  | 8564 | 153 | Code::CodeKey { rung: Decl, file: source/vendor/supports-color/index.d.ts, decl: 3, sub: 0, line: 24 } |  |  | 0.609 |
| walker |  | 8586 | 22 | Code::CodeKey { rung: Names, file: source/vendor/supports-color/browser.js, decl: 0, sub: 0, line: 0 } |  |  | 0.609 |
| walker |  | 8609 | 23 | Code::CodeKey { rung: Decl, file: source/vendor/supports-color/browser.js, decl: 1, sub: 0, line: 29 } |  |  | 0.609 |
| walker |  | 8622 | 13 | Code::CodeKey { rung: Names, file: source/vendor/supports-color/browser.d.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.609 |
| walker |  | 8640 | 18 | Code::CodeKey { rung: Doc, file: source/vendor/ansi-styles/index.d.ts, decl: 7, sub: 0, line: 186 } |  |  | 0.609 |
| ns | 8683 |  | 209 | supports-color browser build: `browser.js` level detection and export shape | 5.10 |  | 0.601 |
| ns | 8696 |  | 13 | `browser.d.ts` (one-line type re-export) | 5.11 |  | 0.601 |
| ns | 8768 |  | 72 | ava test harness setup in `test/chalk.js` | 6.1 |  | 0.599 |
| walker |  | 8823 | 183 | Code::CodeKey { rung: Decl, file: source/vendor/ansi-styles/index.d.ts, decl: 6, sub: 0, line: 135 } |  |  | 0.599 |
| walker |  | 8842 | 19 | Code::CodeKey { rung: Doc, file: source/vendor/supports-color/index.d.ts, decl: 3, sub: 0, line: 24 } |  |  | 0.599 |
| walker |  | 9155 | 313 | Code::CodeKey { rung: Decl, file: source/vendor/ansi-styles/index.d.ts, decl: 6, sub: 1, line: 135 } |  |  | 0.599 |
| ns | 9189 |  | 421 | Every test title in `test/chalk.js` | 6.2 | 6.1 | 0.589 |
| ns | 9425 |  | 236 | Every test title in the four remaining ava files | 6.3 |  | 0.585 |
| ns | 9522 |  | 97 | `test/_fixture.js` in full, and the child process that runs it | 6.4 | 6.3 | 0.583 |
| walker |  | 9632 | 477 | Markdown::Section { file: readme.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.583 |
| ns | 9753 |  | 231 | ansi-styles `index.d.ts`: complete declaration roster | 7.1 |  | 0.590 |
| walker |  | 9837 | 205 | Markdown::Section { file: readme.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.590 |
| walker |  | 9862 | 25 | Code::CodeKey { rung: Doc, file: source/vendor/ansi-styles/index.d.ts, decl: 14, sub: 0, line: 227 } |  |  | 0.590 |
| ns | 9896 |  | 143 | `source/index.test-d.ts`: what the tsd suite asserts | 7.2 |  | 0.585 |
| walker |  | 9976 | 114 | Markdown::Section { file: readme.md, section_index: 16, keeps_default_concavity: false } |  |  | 0.585 |
| ns | 9989 |  | 93 | `benchmark.js` and both `examples/` scripts: purpose lines | 7.3 |  | 0.582 |

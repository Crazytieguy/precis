Score(3000)=0.577 I=0.859 C=0.388 ns_rows≤3K=21/59 grid(1000/1442/2080/3000/4327/6240/9000)=0.855/0.728/0.626/0.577/0.624/0.729/0.650

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 46 | 46 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 54 |  | 54 | package.json name, version, description, license | 1.1 |  | 0.000 |
| walker |  | 58 | 12 | Fs::DirListing { dir: media } |  |  | 0.000 |
| walker |  | 80 | 22 | Fs::DirListing { dir: source } |  |  | 0.000 |
| walker |  | 89 | 9 | Fs::DirListing { dir: source/vendor } |  |  | 0.000 |
| ns | 100 |  | 46 | Complete repository root listing | 1.2 |  | 0.598 |
| walker |  | 107 | 18 | Fs::DirListing { dir: source/vendor/supports-color } |  |  | 0.629 |
| walker |  | 116 | 9 | Fs::DirListing { dir: source/vendor/ansi-styles } |  |  | 0.650 |
| ns | 172 |  | 72 | package.json entry points: `main`, `exports`, `repository`, `type` | 1.3 |  | 0.560 |
| walker |  | 182 | 66 | Json::Identity { file: package.json } |  |  | 0.892 |
| walker |  | 190 | 8 | Fs::DirListing { dir: examples } |  |  | 0.892 |
| walker |  | 201 | 11 | Fs::DirListing { dir: .github } |  |  | 0.892 |
| walker |  | 205 | 4 | Fs::DirListing { dir: .github/workflows } |  |  | 0.892 |
| ns | 230 |  | 58 | Complete `source/` tree including both vendored packages | 1.4 |  | 0.914 |
| walker |  | 310 | 105 | Markdown::ReadmeHeadline { file: readme.md } |  |  | 0.915 |
| walker |  | 356 | 46 | Json::Runtime { file: package.json } |  |  | 0.916 |
| ns | 361 |  | 131 | readme tagline + every `##` section heading | 1.5 |  | 0.767 |
| walker |  | 405 | 49 | Json::Scripts { file: package.json } |  |  | 0.770 |
| walker |  | 432 | 27 | Fs::DirListing { dir: test } |  |  | 0.781 |
| ns | 568 |  | 207 | package.json `imports` subpath map, `types`, `engines`, `scripts` | 1.6 |  | 0.684 |
| walker |  | 692 | 260 | Markdown::HeadingsOutline { file: readme.md } |  |  | 0.836 |
| ns | 708 |  | 140 | readme `###` subsection headings | 1.7 | 1.5 | 0.845 |
| walker |  | 716 | 24 | Markdown::Section { file: readme.md, section_index: 12, keeps_default_concavity: false } |  |  | 0.845 |
| ns | 770 |  | 62 | Complete listing of `test/`, `examples/`, `.github/`, `media/` | 1.8 |  | 0.852 |
| walker |  | 925 | 209 | Markdown::Section { file: readme.md, section_index: 8, keeps_default_concavity: true } |  |  | 0.855 |
| walker |  | 962 | 37 | Markdown::Section { file: readme.md, section_index: 13, keeps_default_concavity: false } |  |  | 0.855 |
| ns | 1030 |  | 260 | CI workflow in full | 1.9 |  | 0.736 |
| walker |  | 1137 | 175 | Json::Entry { file: package.json } |  |  | 0.833 |
| ns | 1244 |  | 214 | Runtime export surface of `source/index.js` | 2.1 |  | 0.753 |
| ns | 1358 |  | 114 | `source/index.d.ts` top-level declaration roster | 2.2 |  | 0.728 |
| ns | 1458 |  | 100 | readme: chaining semantics of `chalk.<style>[.<style>...]` | 2.3 | 1.7 | 0.710 |
| walker |  | 1462 | 325 | Markdown::Section { file: readme.md, section_index: 9, keeps_default_concavity: true } |  |  | 0.710 |
| walker |  | 1504 | 42 | Markdown::Section { file: readme.md, section_index: 17, keeps_default_concavity: false } |  |  | 0.710 |
| ns | 1686 |  | 228 | readme: `chalk.level` and the 0–3 colour-level table | 2.4 | 1.7 | 0.661 |
| walker |  | 1733 | 229 | Json::IdentityMeta { file: package.json } |  |  | 0.685 |
| walker |  | 1824 | 91 | Markdown::Section { file: readme.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.685 |
| ns | 1826 |  | 140 | `Options` interface body with level documentation | 2.5 | 2.2 | 0.657 |
| ns | 1970 |  | 144 | `ChalkInstance` call signature, `level`, and all six colour-model methods | 2.6 | 2.2 | 0.626 |
| ns | 2094 |  | 124 | All ten `ChalkInstance` modifier properties | 2.7 | 2.6 | 0.594 |
| ns | 2293 |  | 199 | All foreground colour properties, including the `gray`/`grey` aliases | 2.8 | 2.7 | 0.564 |
| walker |  | 2306 | 482 | Code::CodeKey { rung: Names, file: source/index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.640 |
| walker |  | 2319 | 13 | Code::CodeKey { rung: Decl, file: source/index.js, decl: 8, sub: 0, line: 34 } |  |  | 0.640 |
| walker |  | 2357 | 38 | Code::CodeKey { rung: Decl, file: source/index.js, decl: 5, sub: 0, line: 15 } |  |  | 0.640 |
| walker |  | 2366 | 9 | Code::CodeKey { rung: Body, file: source/index.js, decl: 11, sub: 0, line: 50 } |  |  | 0.640 |
| walker |  | 2459 | 93 | Code::CodeKey { rung: Decl, file: source/index.js, decl: 14, sub: 0, line: 119 } |  |  | 0.642 |
| walker |  | 2485 | 26 | Code::CodeKey { rung: Body, file: source/index.js, decl: 9, sub: 0, line: 35 } |  |  | 0.642 |
| ns | 2517 |  | 224 | All background colour properties and the close of `ChalkInstance` | 2.9 | 2.8 | 0.610 |
| walker |  | 2545 | 60 | Code::CodeKey { rung: Body, file: source/index.js, decl: 10, sub: 0, line: 41 } |  |  | 0.611 |
| walker |  | 2685 | 140 | Markdown::Section { file: readme.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.611 |
| ns | 2767 |  | 250 | readme: `supportsColor`, the `--color`/`FORCE_COLOR` overrides, `chalkStderr` | 2.10 | 1.7 | 0.597 |
| ns | 2832 |  | 65 | readme: the exported style-name arrays and their use | 2.11 | 1.7 | 0.591 |
| ns | 2962 |  | 130 | `index.d.ts` type re-export blocks from the vendored packages | 2.12 |  | 0.577 |
| walker |  | 3069 | 384 | Code::CodeKey { rung: Names, file: source/index.d.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.633 |
| walker |  | 3089 | 20 | Code::CodeKey { rung: Doc, file: source/index.d.ts, decl: 2, sub: 0, line: 30 } |  |  | 0.633 |
| ns | 3093 |  | 131 | Deprecated type/const aliases in `index.d.ts` | 2.13 | 2.2 | 0.628 |
| walker |  | 3227 | 138 | Code::CodeKey { rung: Decl, file: source/index.d.ts, decl: 1, sub: 0, line: 12 } |  |  | 0.660 |
| walker |  | 3261 | 34 | Code::CodeKey { rung: Doc, file: source/index.d.ts, decl: 12, sub: 0, line: 302 } |  |  | 0.663 |
| ns | 3267 |  | 174 | ansi-styles: the `styles.modifier` code table | 3.1 |  | 0.647 |
| walker |  | 3297 | 36 | Code::CodeKey { rung: Doc, file: source/index.d.ts, decl: 8, sub: 0, line: 268 } |  |  | 0.649 |
| walker |  | 3333 | 36 | Code::CodeKey { rung: Doc, file: source/index.d.ts, decl: 13, sub: 0, line: 309 } |  |  | 0.651 |
| walker |  | 3369 | 36 | Code::CodeKey { rung: Doc, file: source/index.d.ts, decl: 14, sub: 0, line: 316 } |  |  | 0.653 |
| walker |  | 3412 | 43 | Code::CodeKey { rung: Doc, file: source/index.d.ts, decl: 15, sub: 0, line: 323 } |  |  | 0.655 |
| walker |  | 3485 | 73 | Code::CodeKey { rung: Doc, file: source/index.d.ts, decl: 9, sub: 0, line: 277 } |  |  | 0.657 |
| walker |  | 3558 | 73 | Code::CodeKey { rung: Doc, file: source/index.d.ts, decl: 10, sub: 0, line: 286 } |  |  | 0.659 |
| ns | 3571 |  | 304 | ansi-styles: the `styles.color` foreground code table | 3.2 | 3.1 | 0.635 |
| walker |  | 3638 | 80 | Code::CodeKey { rung: Doc, file: source/index.d.ts, decl: 11, sub: 0, line: 295 } |  |  | 0.637 |
| walker |  | 3733 | 95 | Code::CodeKey { rung: Doc, file: source/index.d.ts, decl: 4, sub: 0, line: 242 } |  |  | 0.637 |
| ns | 3885 |  | 314 | ansi-styles: the `styles.bgColor` background code table | 3.3 | 3.2 | 0.613 |
| ns | 3950 |  | 65 | ansi-styles: the four exported style-name arrays | 3.4 | 3.3 | 0.609 |
| walker |  | 3996 | 263 | Code::CodeKey { rung: Decl, file: source/index.d.ts, decl: 3, sub: 0, line: 32 } |  |  | 0.613 |
| walker |  | 4042 | 46 | Code::CodeKey { rung: Names, file: source/utilities.js, decl: 0, sub: 0, line: 0 } |  |  | 0.614 |
| ns | 4159 |  | 209 | readme: modifier list with human descriptions | 3.5 | 1.7 | 0.624 |
| walker |  | 4384 | 342 | Code::CodeKey { rung: Decl, file: source/index.d.ts, decl: 3, sub: 1, line: 32 } |  |  | 0.638 |
| ns | 4467 |  | 308 | readme: 256/Truecolor section and the `rgb`/`hex`/`ansi256` colour models | 3.6 | 1.5 | 0.626 |
| ns | 4561 |  | 94 | `source/index.js` imports and the stdout/stderr colour split | 4.1 |  | 0.619 |
| walker |  | 4694 | 310 | Code::CodeKey { rung: Decl, file: source/index.d.ts, decl: 3, sub: 2, line: 32 } |  |  | 0.635 |
| ns | 4843 |  | 282 | `source/index.js`: complete top-level declaration roster | 4.2 |  | 0.640 |
| ns | 4982 |  | 139 | `Chalk` class, `chalkFactory`, `createChalk` | 4.3 | 4.2 | 0.640 |
| walker |  | 5010 | 316 | Code::CodeKey { rung: Decl, file: source/index.d.ts, decl: 3, sub: 3, line: 32 } |  |  | 0.666 |
| ns | 5100 |  | 118 | `applyOptions`: level validation and auto-detection | 4.4 | 4.2 | 0.659 |
| ns | 5270 |  | 170 | Style-property generation loop, `visible`, and prototype installation | 4.5 | 4.2 | 0.646 |
| walker |  | 5294 | 284 | Code::CodeKey { rung: Decl, file: source/index.d.ts, decl: 3, sub: 4, line: 32 } |  |  | 0.677 |
| ns | 5461 |  | 191 | `createBuilder`: the chainable callable | 4.6 | 4.2 | 0.665 |
| walker |  | 5604 | 310 | Code::CodeKey { rung: Decl, file: source/index.d.ts, decl: 3, sub: 5, line: 32 } |  |  | 0.710 |
| ns | 5607 |  | 146 | `createStyler`: the open/close linked list | 4.7 | 4.2 | 0.695 |
| walker |  | 5719 | 115 | Code::CodeKey { rung: Body, file: source/index.js, decl: 7, sub: 0, line: 24 } |  |  | 0.705 |
| walker |  | 5888 | 169 | Markdown::Section { file: readme.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.719 |
| ns | 5907 |  | 300 | `applyStyle`: the string-wrapping algorithm | 4.8 | 4.2 | 0.696 |
| ns | 6000 |  | 93 | `proto` and the `level` getter/setter delegation | 4.9 | 4.2 | 0.701 |
| ns | 6056 |  | 56 | `levelMapping`: numeric level → ansi-styles method name | 4.10 | 4.2 | 0.698 |
| walker |  | 6206 | 318 | Markdown::Section { file: readme.md, section_index: 6, keeps_default_concavity: true } |  |  | 0.729 |
| ns | 6255 |  | 199 | `getModelAnsi`: RGB/hex downsampling dispatch | 4.11 | 4.2 | 0.716 |
| walker |  | 6360 | 154 | Code::CodeKey { rung: Body, file: source/utilities.js, decl: 2, sub: 0, line: 21 } |  |  | 0.716 |
| ns | 6528 |  | 273 | Colour-model property generation for `rgb`/`hex`/`ansi256` and their `bg*` twins | 4.12 | 4.2 | 0.701 |
| walker |  | 6529 | 169 | Code::CodeKey { rung: Body, file: source/utilities.js, decl: 1, sub: 0, line: 2 } |  |  | 0.701 |
| ns | 6593 |  | 65 | `source/utilities.js`: both exported helpers, signatures only | 4.13 |  | 0.700 |
| walker |  | 6712 | 183 | Markdown::Section { file: readme.md, section_index: 10, keeps_default_concavity: false } |  |  | 0.704 |
| ns | 6816 |  | 223 | supports-color: complete declaration roster and default export | 5.1 |  | 0.694 |
| walker |  | 6947 | 235 | Markdown::Section { file: readme.md, section_index: 11, keeps_default_concavity: true } |  |  | 0.706 |
| ns | 6960 |  | 144 | supports-color: the `--color` / `--no-color` flag block | 5.2 | 5.1 | 0.696 |
| walker |  | 6969 | 22 | Code::CodeKey { rung: Names, file: source/vendor/supports-color/browser.js, decl: 0, sub: 0, line: 0 } |  |  | 0.696 |
| walker |  | 6992 | 23 | Code::CodeKey { rung: Decl, file: source/vendor/supports-color/browser.js, decl: 1, sub: 0, line: 29 } |  |  | 0.696 |
| ns | 7095 |  | 135 | supports-color: `envForceColor` (`FORCE_COLOR` parsing) | 5.3 | 5.1 | 0.688 |
| walker |  | 7135 | 143 | Code::CodeKey { rung: Body, file: source/index.js, decl: 15, sub: 0, line: 132 } |  |  | 0.707 |
| ns | 7185 |  | 90 | supports-color: `translateLevel` — the `ColorInfo` shape | 5.4 | 5.1 | 0.700 |
| walker |  | 7383 | 248 | Markdown::Section { file: readme.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.713 |
| walker |  | 7396 | 13 | Code::CodeKey { rung: Names, file: source/vendor/supports-color/browser.d.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.713 |
| ns | 7409 |  | 224 | `_supportsColor`: force-colour precedence and explicit `--color=` levels | 5.5 | 5.1 | 0.700 |
| walker |  | 7435 | 39 | Code::CodeKey { rung: Names, file: source/vendor/supports-color/index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.700 |
| walker |  | 7482 | 47 | Code::CodeKey { rung: Decl, file: source/vendor/supports-color/index.js, decl: 2, sub: 0, line: 185 } |  |  | 0.702 |
| walker |  | 7539 | 57 | Code::CodeKey { rung: Body, file: source/vendor/supports-color/index.js, decl: 1, sub: 0, line: 176 } |  |  | 0.703 |
| walker |  | 7745 | 206 | Markdown::Section { file: readme.md, section_index: 14, keeps_default_concavity: false } |  |  | 0.703 |
| ns | 7848 |  | 439 | `_supportsColor`: TTY, Windows and CI detection | 5.6 | 5.5 | 0.681 |
| walker |  | 7959 | 214 | Markdown::Section { file: readme.md, section_index: 15, keeps_default_concavity: true } |  |  | 0.681 |
| walker |  | 8194 | 235 | Code::CodeKey { rung: Names, file: source/vendor/ansi-styles/index.d.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.681 |
| ns | 8249 |  | 401 | `_supportsColor`: TeamCity, `COLORTERM` and terminal-emulator sniffing | 5.7 | 5.6 | 0.666 |
| walker |  | 8265 | 71 | Code::CodeKey { rung: Decl, file: source/vendor/ansi-styles/index.d.ts, decl: 15, sub: 0, line: 229 } |  |  | 0.667 |
| ns | 8309 |  | 60 | supports-color: `createSupportsColor` body | 5.8 | 5.1 | 0.669 |
| walker |  | 8345 | 80 | Code::CodeKey { rung: Decl, file: source/vendor/ansi-styles/index.d.ts, decl: 1, sub: 0, line: 1 } |  |  | 0.669 |
| walker |  | 8361 | 16 | Code::CodeKey { rung: Doc, file: source/vendor/ansi-styles/index.d.ts, decl: 11, sub: 0, line: 212 } |  |  | 0.669 |
| walker |  | 8378 | 17 | Code::CodeKey { rung: Doc, file: source/vendor/ansi-styles/index.d.ts, decl: 12, sub: 0, line: 217 } |  |  | 0.669 |
| walker |  | 8395 | 17 | Code::CodeKey { rung: Doc, file: source/vendor/ansi-styles/index.d.ts, decl: 13, sub: 0, line: 222 } |  |  | 0.669 |
| ns | 8474 |  | 165 | supports-color `index.d.ts`: complete type roster | 5.9 |  | 0.661 |
| walker |  | 8496 | 101 | Code::CodeKey { rung: Decl, file: source/vendor/ansi-styles/index.d.ts, decl: 2, sub: 0, line: 13 } |  |  | 0.661 |
| walker |  | 8514 | 18 | Code::CodeKey { rung: Doc, file: source/vendor/ansi-styles/index.d.ts, decl: 7, sub: 0, line: 186 } |  |  | 0.661 |
| ns | 8683 |  | 209 | supports-color browser build: `browser.js` level detection and export shape | 5.10 |  | 0.652 |
| ns | 8696 |  | 13 | `browser.d.ts` (one-line type re-export) | 5.11 |  | 0.652 |
| ns | 8768 |  | 72 | ava test harness setup in `test/chalk.js` | 6.1 |  | 0.650 |
| walker |  | 8786 | 272 | Code::CodeKey { rung: Decl, file: source/vendor/ansi-styles/index.d.ts, decl: 4, sub: 0, line: 75 } |  |  | 0.650 |
| walker |  | 9080 | 294 | Code::CodeKey { rung: Decl, file: source/vendor/ansi-styles/index.d.ts, decl: 5, sub: 0, line: 105 } |  |  | 0.650 |
| walker |  | 9134 | 54 | Code::CodeKey { rung: Doc, file: source/vendor/ansi-styles/index.d.ts, decl: 8, sub: 0, line: 193 } |  |  | 0.650 |
| walker |  | 9188 | 54 | Code::CodeKey { rung: Doc, file: source/vendor/ansi-styles/index.d.ts, decl: 9, sub: 0, line: 200 } |  |  | 0.650 |
| ns | 9189 |  | 421 | Every test title in `test/chalk.js` | 6.2 | 6.1 | 0.639 |
| walker |  | 9250 | 62 | Code::CodeKey { rung: Doc, file: source/vendor/ansi-styles/index.d.ts, decl: 10, sub: 0, line: 207 } |  |  | 0.639 |
| ns | 9425 |  | 236 | Every test title in the four remaining ava files | 6.3 |  | 0.635 |
| ns | 9522 |  | 97 | `test/_fixture.js` in full, and the child process that runs it | 6.4 | 6.3 | 0.632 |
| walker |  | 9652 | 402 | Code::CodeKey { rung: Decl, file: source/vendor/ansi-styles/index.d.ts, decl: 3, sub: 0, line: 26 } |  |  | 0.632 |
| ns | 9753 |  | 231 | ansi-styles `index.d.ts`: complete declaration roster | 7.1 |  | 0.639 |
| ns | 9896 |  | 143 | `source/index.test-d.ts`: what the tsd suite asserts | 7.2 |  | 0.633 |
| walker |  | 9907 | 255 | Code::CodeKey { rung: Decl, file: source/vendor/ansi-styles/index.d.ts, decl: 6, sub: 0, line: 135 } |  |  | 0.633 |
| ns | 9989 |  | 93 | `benchmark.js` and both `examples/` scripts: purpose lines | 7.3 |  | 0.630 |

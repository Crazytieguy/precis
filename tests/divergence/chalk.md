Score(3000)=0.661 I=0.877 C=0.498 ns_rows≤3K=21/59 grid(1000/1442/2080/3000/4327/6240/9000)=0.965/0.753/0.706/0.661/0.691/0.651/0.573

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
| walker |  | 753 | 37 | Markdown::Section { file: readme.md, section_index: 13, keeps_default_concavity: false } |  |  | 0.845 |
| ns | 770 |  | 62 | Complete listing of `test/`, `examples/`, `.github/`, `media/` | 1.8 |  | 0.852 |
| walker |  | 928 | 175 | Json::Entry { file: package.json } |  |  | 0.965 |
| walker |  | 970 | 42 | Markdown::Section { file: readme.md, section_index: 17, keeps_default_concavity: false } |  |  | 0.965 |
| ns | 1030 |  | 260 | CI workflow in full | 1.9 |  | 0.831 |
| walker |  | 1199 | 229 | Json::IdentityMeta { file: package.json } |  |  | 0.861 |
| ns | 1244 |  | 214 | Runtime export surface of `source/index.js` | 2.1 |  | 0.778 |
| walker |  | 1290 | 91 | Markdown::Section { file: readme.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.778 |
| ns | 1358 |  | 114 | `source/index.d.ts` top-level declaration roster | 2.2 |  | 0.753 |
| ns | 1458 |  | 100 | readme: chaining semantics of `chalk.<style>[.<style>...]` | 2.3 | 1.7 | 0.733 |
| walker |  | 1527 | 237 | Code::CodeKey { rung: Names, file: source/index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.830 |
| walker |  | 1542 | 15 | Code::CodeKey { rung: Decl, file: source/index.js, decl: 1, sub: 0, line: 34 } |  |  | 0.830 |
| walker |  | 1551 | 9 | Code::CodeKey { rung: Body, file: source/index.js, decl: 3, sub: 0, line: 50 } |  |  | 0.830 |
| walker |  | 1577 | 26 | Code::CodeKey { rung: Body, file: source/index.js, decl: 2, sub: 0, line: 35 } |  |  | 0.830 |
| ns | 1686 |  | 228 | readme: `chalk.level` and the 0–3 colour-level table | 2.4 | 1.7 | 0.773 |
| walker |  | 1717 | 140 | Markdown::Section { file: readme.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.773 |
| ns | 1826 |  | 140 | `Options` interface body with level documentation | 2.5 | 2.2 | 0.741 |
| ns | 1970 |  | 144 | `ChalkInstance` call signature, `level`, and all six colour-model methods | 2.6 | 2.2 | 0.706 |
| ns | 2094 |  | 124 | All ten `ChalkInstance` modifier properties | 2.7 | 2.6 | 0.670 |
| walker |  | 2101 | 384 | Code::CodeKey { rung: Names, file: source/index.d.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.701 |
| walker |  | 2121 | 20 | Code::CodeKey { rung: Doc, file: source/index.d.ts, decl: 2, sub: 0, line: 30 } |  |  | 0.701 |
| walker |  | 2259 | 138 | Code::CodeKey { rung: Decl, file: source/index.d.ts, decl: 1, sub: 0, line: 12 } |  |  | 0.743 |
| walker |  | 2293 | 34 | Code::CodeKey { rung: Doc, file: source/index.d.ts, decl: 12, sub: 0, line: 302 } |  |  | 0.705 |
| ns | 2293 |  | 199 | All foreground colour properties, including the `gray`/`grey` aliases | 2.8 | 2.7 | 0.705 |
| walker |  | 2329 | 36 | Code::CodeKey { rung: Doc, file: source/index.d.ts, decl: 8, sub: 0, line: 268 } |  |  | 0.705 |
| walker |  | 2365 | 36 | Code::CodeKey { rung: Doc, file: source/index.d.ts, decl: 13, sub: 0, line: 309 } |  |  | 0.705 |
| walker |  | 2401 | 36 | Code::CodeKey { rung: Doc, file: source/index.d.ts, decl: 14, sub: 0, line: 316 } |  |  | 0.706 |
| walker |  | 2444 | 43 | Code::CodeKey { rung: Doc, file: source/index.d.ts, decl: 15, sub: 0, line: 323 } |  |  | 0.706 |
| walker |  | 2517 | 73 | Code::CodeKey { rung: Doc, file: source/index.d.ts, decl: 9, sub: 0, line: 277 } |  |  | 0.671 |
| ns | 2517 |  | 224 | All background colour properties and the close of `ChalkInstance` | 2.9 | 2.8 | 0.671 |
| walker |  | 2590 | 73 | Code::CodeKey { rung: Doc, file: source/index.d.ts, decl: 10, sub: 0, line: 286 } |  |  | 0.671 |
| walker |  | 2670 | 80 | Code::CodeKey { rung: Doc, file: source/index.d.ts, decl: 11, sub: 0, line: 295 } |  |  | 0.672 |
| walker |  | 2765 | 95 | Code::CodeKey { rung: Doc, file: source/index.d.ts, decl: 4, sub: 0, line: 242 } |  |  | 0.672 |
| ns | 2767 |  | 250 | readme: `supportsColor`, the `--color`/`FORCE_COLOR` overrides, `chalkStderr` | 2.10 | 1.7 | 0.656 |
| walker |  | 2811 | 46 | Code::CodeKey { rung: Names, file: source/utilities.js, decl: 0, sub: 0, line: 0 } |  |  | 0.656 |
| ns | 2832 |  | 65 | readme: the exported style-name arrays and their use | 2.11 | 1.7 | 0.649 |
| ns | 2962 |  | 130 | `index.d.ts` type re-export blocks from the vendored packages | 2.12 |  | 0.661 |
| walker |  | 3074 | 263 | Code::CodeKey { rung: Decl, file: source/index.d.ts, decl: 3, sub: 0, line: 32 } |  |  | 0.666 |
| ns | 3093 |  | 131 | Deprecated type/const aliases in `index.d.ts` | 2.13 | 2.2 | 0.674 |
| ns | 3267 |  | 174 | ansi-styles: the `styles.modifier` code table | 3.1 |  | 0.658 |
| walker |  | 3416 | 342 | Code::CodeKey { rung: Decl, file: source/index.d.ts, decl: 3, sub: 1, line: 32 } |  |  | 0.674 |
| ns | 3571 |  | 304 | ansi-styles: the `styles.color` foreground code table | 3.2 | 3.1 | 0.649 |
| walker |  | 3726 | 310 | Code::CodeKey { rung: Decl, file: source/index.d.ts, decl: 3, sub: 2, line: 32 } |  |  | 0.666 |
| ns | 3885 |  | 314 | ansi-styles: the `styles.bgColor` background code table | 3.3 | 3.2 | 0.642 |
| ns | 3950 |  | 65 | ansi-styles: the four exported style-name arrays | 3.4 | 3.3 | 0.638 |
| walker |  | 4042 | 316 | Code::CodeKey { rung: Decl, file: source/index.d.ts, decl: 3, sub: 3, line: 32 } |  |  | 0.668 |
| ns | 4159 |  | 209 | readme: modifier list with human descriptions | 3.5 | 1.7 | 0.656 |
| walker |  | 4326 | 284 | Code::CodeKey { rung: Decl, file: source/index.d.ts, decl: 3, sub: 4, line: 32 } |  |  | 0.691 |
| ns | 4467 |  | 308 | readme: 256/Truecolor section and the `rgb`/`hex`/`ansi256` colour models | 3.6 | 1.5 | 0.678 |
| ns | 4561 |  | 94 | `source/index.js` imports and the stdout/stderr colour split | 4.1 |  | 0.671 |
| walker |  | 4636 | 310 | Code::CodeKey { rung: Decl, file: source/index.d.ts, decl: 3, sub: 5, line: 32 } |  |  | 0.722 |
| walker |  | 4805 | 169 | Markdown::Section { file: readme.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.739 |
| ns | 4843 |  | 282 | `source/index.js`: complete top-level declaration roster | 4.2 |  | 0.721 |
| walker |  | 4959 | 154 | Code::CodeKey { rung: Body, file: source/utilities.js, decl: 2, sub: 0, line: 21 } |  |  | 0.721 |
| ns | 4982 |  | 139 | `Chalk` class, `chalkFactory`, `createChalk` | 4.3 | 4.2 | 0.708 |
| ns | 5100 |  | 118 | `applyOptions`: level validation and auto-detection | 4.4 | 4.2 | 0.701 |
| walker |  | 5128 | 169 | Code::CodeKey { rung: Body, file: source/utilities.js, decl: 1, sub: 0, line: 2 } |  |  | 0.701 |
| walker |  | 5150 | 22 | Code::CodeKey { rung: Names, file: source/vendor/supports-color/browser.js, decl: 0, sub: 0, line: 0 } |  |  | 0.701 |
| walker |  | 5173 | 23 | Code::CodeKey { rung: Decl, file: source/vendor/supports-color/browser.js, decl: 1, sub: 0, line: 29 } |  |  | 0.701 |
| ns | 5270 |  | 170 | Style-property generation loop, `visible`, and prototype installation | 4.5 | 4.2 | 0.688 |
| walker |  | 5356 | 183 | Markdown::Section { file: readme.md, section_index: 10, keeps_default_concavity: false } |  |  | 0.692 |
| walker |  | 5369 | 13 | Code::CodeKey { rung: Names, file: source/vendor/supports-color/browser.d.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.692 |
| walker |  | 5408 | 39 | Code::CodeKey { rung: Names, file: source/vendor/supports-color/index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.692 |
| walker |  | 5455 | 47 | Code::CodeKey { rung: Decl, file: source/vendor/supports-color/index.js, decl: 2, sub: 0, line: 185 } |  |  | 0.692 |
| ns | 5461 |  | 191 | `createBuilder`: the chainable callable | 4.6 | 4.2 | 0.681 |
| walker |  | 5512 | 57 | Code::CodeKey { rung: Body, file: source/vendor/supports-color/index.js, decl: 1, sub: 0, line: 176 } |  |  | 0.681 |
| ns | 5607 |  | 146 | `createStyler`: the open/close linked list | 4.7 | 4.2 | 0.667 |
| walker |  | 5721 | 209 | Markdown::Section { file: readme.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.684 |
| ns | 5907 |  | 300 | `applyStyle`: the string-wrapping algorithm | 4.8 | 4.2 | 0.662 |
| walker |  | 5956 | 235 | Code::CodeKey { rung: Names, file: source/vendor/ansi-styles/index.d.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.663 |
| ns | 6000 |  | 93 | `proto` and the `level` getter/setter delegation | 4.9 | 4.2 | 0.655 |
| walker |  | 6027 | 71 | Code::CodeKey { rung: Decl, file: source/vendor/ansi-styles/index.d.ts, decl: 15, sub: 0, line: 229 } |  |  | 0.656 |
| ns | 6056 |  | 56 | `levelMapping`: numeric level → ansi-styles method name | 4.10 | 4.2 | 0.651 |
| walker |  | 6107 | 80 | Code::CodeKey { rung: Decl, file: source/vendor/ansi-styles/index.d.ts, decl: 1, sub: 0, line: 1 } |  |  | 0.651 |
| walker |  | 6123 | 16 | Code::CodeKey { rung: Doc, file: source/vendor/ansi-styles/index.d.ts, decl: 11, sub: 0, line: 212 } |  |  | 0.651 |
| walker |  | 6224 | 101 | Code::CodeKey { rung: Decl, file: source/vendor/ansi-styles/index.d.ts, decl: 2, sub: 0, line: 13 } |  |  | 0.651 |
| walker |  | 6241 | 17 | Code::CodeKey { rung: Doc, file: source/vendor/ansi-styles/index.d.ts, decl: 12, sub: 0, line: 217 } |  |  | 0.651 |
| ns | 6255 |  | 199 | `getModelAnsi`: RGB/hex downsampling dispatch | 4.11 | 4.2 | 0.639 |
| walker |  | 6258 | 17 | Code::CodeKey { rung: Doc, file: source/vendor/ansi-styles/index.d.ts, decl: 13, sub: 0, line: 222 } |  |  | 0.639 |
| walker |  | 6276 | 18 | Code::CodeKey { rung: Doc, file: source/vendor/ansi-styles/index.d.ts, decl: 7, sub: 0, line: 186 } |  |  | 0.639 |
| ns | 6528 |  | 273 | Colour-model property generation for `rgb`/`hex`/`ansi256` and their `bg*` twins | 4.12 | 4.2 | 0.626 |
| walker |  | 6548 | 272 | Code::CodeKey { rung: Decl, file: source/vendor/ansi-styles/index.d.ts, decl: 4, sub: 0, line: 75 } |  |  | 0.626 |
| ns | 6593 |  | 65 | `source/utilities.js`: both exported helpers, signatures only | 4.13 |  | 0.626 |
| ns | 6816 |  | 223 | supports-color: complete declaration roster and default export | 5.1 |  | 0.619 |
| walker |  | 6842 | 294 | Code::CodeKey { rung: Decl, file: source/vendor/ansi-styles/index.d.ts, decl: 5, sub: 0, line: 105 } |  |  | 0.619 |
| walker |  | 6896 | 54 | Code::CodeKey { rung: Doc, file: source/vendor/ansi-styles/index.d.ts, decl: 8, sub: 0, line: 193 } |  |  | 0.619 |
| walker |  | 6950 | 54 | Code::CodeKey { rung: Doc, file: source/vendor/ansi-styles/index.d.ts, decl: 9, sub: 0, line: 200 } |  |  | 0.619 |
| ns | 6960 |  | 144 | supports-color: the `--color` / `--no-color` flag block | 5.2 | 5.1 | 0.611 |
| walker |  | 7012 | 62 | Code::CodeKey { rung: Doc, file: source/vendor/ansi-styles/index.d.ts, decl: 10, sub: 0, line: 207 } |  |  | 0.611 |
| ns | 7095 |  | 135 | supports-color: `envForceColor` (`FORCE_COLOR` parsing) | 5.3 | 5.1 | 0.604 |
| ns | 7185 |  | 90 | supports-color: `translateLevel` — the `ColorInfo` shape | 5.4 | 5.1 | 0.598 |
| ns | 7409 |  | 224 | `_supportsColor`: force-colour precedence and explicit `--color=` levels | 5.5 | 5.1 | 0.586 |
| walker |  | 7414 | 402 | Code::CodeKey { rung: Decl, file: source/vendor/ansi-styles/index.d.ts, decl: 3, sub: 0, line: 26 } |  |  | 0.586 |
| walker |  | 7522 | 108 | Code::CodeKey { rung: Names, file: source/vendor/ansi-styles/index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.591 |
| walker |  | 7637 | 115 | Code::CodeKey { rung: Names, file: source/vendor/supports-color/index.d.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.592 |
| walker |  | 7660 | 23 | Code::CodeKey { rung: Decl, file: source/vendor/supports-color/index.d.ts, decl: 6, sub: 0, line: 50 } |  |  | 0.592 |
| walker |  | 7729 | 69 | Code::CodeKey { rung: Decl, file: source/vendor/supports-color/index.d.ts, decl: 1, sub: 0, line: 3 } |  |  | 0.592 |
| ns | 7848 |  | 439 | `_supportsColor`: TTY, Windows and CI detection | 5.6 | 5.5 | 0.574 |
| walker |  | 7882 | 153 | Code::CodeKey { rung: Decl, file: source/vendor/supports-color/index.d.ts, decl: 3, sub: 0, line: 24 } |  |  | 0.574 |
| walker |  | 7901 | 19 | Code::CodeKey { rung: Doc, file: source/vendor/supports-color/index.d.ts, decl: 3, sub: 0, line: 24 } |  |  | 0.574 |
| walker |  | 7975 | 74 | Code::CodeKey { rung: Doc, file: source/vendor/supports-color/index.d.ts, decl: 2, sub: 0, line: 19 } |  |  | 0.574 |
| walker |  | 8181 | 206 | Markdown::Section { file: readme.md, section_index: 14, keeps_default_concavity: false } |  |  | 0.574 |
| ns | 8249 |  | 401 | `_supportsColor`: TeamCity, `COLORTERM` and terminal-emulator sniffing | 5.7 | 5.6 | 0.561 |
| ns | 8309 |  | 60 | supports-color: `createSupportsColor` body | 5.8 | 5.1 | 0.564 |
| walker |  | 8436 | 255 | Code::CodeKey { rung: Decl, file: source/vendor/ansi-styles/index.d.ts, decl: 6, sub: 0, line: 135 } |  |  | 0.564 |
| ns | 8474 |  | 165 | supports-color `index.d.ts`: complete type roster | 5.9 |  | 0.572 |
| walker |  | 8671 | 235 | Markdown::Section { file: readme.md, section_index: 11, keeps_default_concavity: false } |  |  | 0.583 |
| ns | 8683 |  | 209 | supports-color browser build: `browser.js` level detection and export shape | 5.10 |  | 0.575 |
| ns | 8696 |  | 13 | `browser.d.ts` (one-line type re-export) | 5.11 |  | 0.576 |
| ns | 8768 |  | 72 | ava test harness setup in `test/chalk.js` | 6.1 |  | 0.573 |
| walker |  | 8912 | 241 | Code::CodeKey { rung: Decl, file: source/vendor/ansi-styles/index.d.ts, decl: 6, sub: 1, line: 135 } |  |  | 0.573 |
| walker |  | 9126 | 214 | Markdown::Section { file: readme.md, section_index: 15, keeps_default_concavity: false } |  |  | 0.573 |
| ns | 9189 |  | 421 | Every test title in `test/chalk.js` | 6.2 | 6.1 | 0.564 |
| ns | 9425 |  | 236 | Every test title in the four remaining ava files | 6.3 |  | 0.560 |
| walker |  | 9444 | 318 | Markdown::Section { file: readme.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.584 |
| ns | 9522 |  | 97 | `test/_fixture.js` in full, and the child process that runs it | 6.4 | 6.3 | 0.582 |
| walker |  | 9692 | 248 | Markdown::Section { file: readme.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.595 |
| ns | 9753 |  | 231 | ansi-styles `index.d.ts`: complete declaration roster | 7.1 |  | 0.602 |
| ns | 9896 |  | 143 | `source/index.test-d.ts`: what the tsd suite asserts | 7.2 |  | 0.597 |
| ns | 9989 |  | 93 | `benchmark.js` and both `examples/` scripts: purpose lines | 7.3 |  | 0.594 |

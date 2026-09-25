Score(3000)=0.671 I=0.862 C=0.523 ns_rows≤3K=21/59 grid(1000/1442/2080/3000/4327/6240/9000)=0.867/0.826/0.761/0.671/0.689/0.698/0.594

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 46 | 46 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 54 |  | 54 | package.json name, version, description, license | 1.1 |  | 0.000 |
| walker |  | 58 | 12 | Fs::DirListing { dir: media } |  |  | 0.000 |
| walker |  | 80 | 22 | Fs::DirListing { dir: source } |  |  | 0.000 |
| walker |  | 89 | 9 | Fs::DirListing { dir: source/vendor } |  |  | 0.000 |
| walker |  | 98 | 9 | Fs::DirListing { dir: source/vendor/ansi-styles } |  |  | 0.000 |
| ns | 100 |  | 46 | Complete repository root listing | 1.2 |  | 0.613 |
| walker |  | 116 | 18 | Fs::DirListing { dir: source/vendor/supports-color } |  |  | 0.650 |
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
| walker |  | 669 | 237 | Code::CodeKey { rung: Names, file: source/index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.698 |
| walker |  | 684 | 15 | Code::CodeKey { rung: Decl, file: source/index.js, decl: 1, sub: 0, line: 34 } |  |  | 0.698 |
| walker |  | 693 | 9 | Code::CodeKey { rung: Body, file: source/index.js, decl: 3, sub: 0, line: 50 } |  |  | 0.698 |
| ns | 708 |  | 140 | readme `###` subsection headings | 1.7 | 1.5 | 0.641 |
| ns | 770 |  | 62 | Complete listing of `test/`, `examples/`, `.github/`, `media/` | 1.8 |  | 0.687 |
| walker |  | 953 | 260 | Markdown::HeadingsOutline { file: readme.md } |  |  | 0.867 |
| walker |  | 977 | 24 | Markdown::Section { file: readme.md, section_index: 12, keeps_default_concavity: false } |  |  | 0.867 |
| walker |  | 1014 | 37 | Markdown::Section { file: readme.md, section_index: 13, keeps_default_concavity: false } |  |  | 0.867 |
| ns | 1030 |  | 260 | CI workflow in full | 1.9 |  | 0.747 |
| walker |  | 1189 | 175 | Json::Entry { file: package.json } |  |  | 0.845 |
| ns | 1244 |  | 214 | Runtime export surface of `source/index.js` | 2.1 |  | 0.854 |
| walker |  | 1280 | 91 | Markdown::Section { file: readme.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.854 |
| walker |  | 1306 | 26 | Code::CodeKey { rung: Body, file: source/index.js, decl: 2, sub: 0, line: 35 } |  |  | 0.854 |
| ns | 1358 |  | 114 | `source/index.d.ts` top-level declaration roster | 2.2 |  | 0.826 |
| walker |  | 1446 | 140 | Markdown::Section { file: readme.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.826 |
| ns | 1458 |  | 100 | readme: chaining semantics of `chalk.<style>[.<style>...]` | 2.3 | 1.7 | 0.804 |
| ns | 1686 |  | 228 | readme: `chalk.level` and the 0–3 colour-level table | 2.4 | 1.7 | 0.749 |
| ns | 1826 |  | 140 | `Options` interface body with level documentation | 2.5 | 2.2 | 0.718 |
| walker |  | 1830 | 384 | Code::CodeKey { rung: Names, file: source/index.d.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.752 |
| walker |  | 1970 | 140 | Code::CodeKey { rung: Decl, file: source/index.d.ts, decl: 1, sub: 0, line: 12 } |  |  | 0.761 |
| ns | 1970 |  | 144 | `ChalkInstance` call signature, `level`, and all six colour-model methods | 2.6 | 2.2 | 0.761 |
| walker |  | 1988 | 18 | Code::CodeKey { rung: Doc, file: source/index.d.ts, decl: 2, sub: 0, line: 30 } |  |  | 0.761 |
| walker |  | 2022 | 34 | Code::CodeKey { rung: Doc, file: source/index.d.ts, decl: 12, sub: 0, line: 302 } |  |  | 0.761 |
| walker |  | 2058 | 36 | Code::CodeKey { rung: Doc, file: source/index.d.ts, decl: 8, sub: 0, line: 268 } |  |  | 0.761 |
| walker |  | 2094 | 36 | Code::CodeKey { rung: Doc, file: source/index.d.ts, decl: 13, sub: 0, line: 309 } |  |  | 0.723 |
| ns | 2094 |  | 124 | All ten `ChalkInstance` modifier properties | 2.7 | 2.6 | 0.723 |
| walker |  | 2130 | 36 | Code::CodeKey { rung: Doc, file: source/index.d.ts, decl: 14, sub: 0, line: 316 } |  |  | 0.723 |
| walker |  | 2173 | 43 | Code::CodeKey { rung: Doc, file: source/index.d.ts, decl: 15, sub: 0, line: 323 } |  |  | 0.723 |
| ns | 2293 |  | 199 | All foreground colour properties, including the `gray`/`grey` aliases | 2.8 | 2.7 | 0.686 |
| walker |  | 2342 | 169 | Markdown::Section { file: readme.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.713 |
| walker |  | 2388 | 46 | Code::CodeKey { rung: Names, file: source/utilities.js, decl: 0, sub: 0, line: 0 } |  |  | 0.713 |
| walker |  | 2461 | 73 | Code::CodeKey { rung: Doc, file: source/index.d.ts, decl: 9, sub: 0, line: 277 } |  |  | 0.713 |
| ns | 2517 |  | 224 | All background colour properties and the close of `ChalkInstance` | 2.9 | 2.8 | 0.678 |
| walker |  | 2534 | 73 | Code::CodeKey { rung: Doc, file: source/index.d.ts, decl: 10, sub: 0, line: 286 } |  |  | 0.678 |
| walker |  | 2717 | 183 | Markdown::Section { file: readme.md, section_index: 10, keeps_default_concavity: false } |  |  | 0.679 |
| ns | 2767 |  | 250 | readme: `supportsColor`, the `--color`/`FORCE_COLOR` overrides, `chalkStderr` | 2.10 | 1.7 | 0.663 |
| ns | 2832 |  | 65 | readme: the exported style-name arrays and their use | 2.11 | 1.7 | 0.656 |
| ns | 2962 |  | 130 | `index.d.ts` type re-export blocks from the vendored packages | 2.12 |  | 0.667 |
| walker |  | 2982 | 265 | Code::CodeKey { rung: Decl, file: source/index.d.ts, decl: 3, sub: 0, line: 32 } |  |  | 0.671 |
| walker |  | 3062 | 80 | Code::CodeKey { rung: Doc, file: source/index.d.ts, decl: 11, sub: 0, line: 295 } |  |  | 0.672 |
| ns | 3093 |  | 131 | Deprecated type/const aliases in `index.d.ts` | 2.13 | 2.2 | 0.679 |
| ns | 3267 |  | 174 | ansi-styles: the `styles.modifier` code table | 3.1 |  | 0.663 |
| walker |  | 3271 | 209 | Markdown::Section { file: readme.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.665 |
| ns | 3571 |  | 304 | ansi-styles: the `styles.color` foreground code table | 3.2 | 3.1 | 0.640 |
| walker |  | 3613 | 342 | Code::CodeKey { rung: Decl, file: source/index.d.ts, decl: 3, sub: 1, line: 32 } |  |  | 0.655 |
| walker |  | 3845 | 232 | Code::CodeKey { rung: Decl, file: source/index.d.ts, decl: 3, sub: 2, line: 32 } |  |  | 0.669 |
| ns | 3885 |  | 314 | ansi-styles: the `styles.bgColor` background code table | 3.3 | 3.2 | 0.644 |
| ns | 3950 |  | 65 | ansi-styles: the four exported style-name arrays | 3.4 | 3.3 | 0.640 |
| walker |  | 4101 | 256 | Code::CodeKey { rung: Decl, file: source/index.d.ts, decl: 3, sub: 3, line: 32 } |  |  | 0.655 |
| ns | 4159 |  | 209 | readme: modifier list with human descriptions | 3.5 | 1.7 | 0.664 |
| walker |  | 4326 | 225 | Code::CodeKey { rung: Decl, file: source/index.d.ts, decl: 3, sub: 4, line: 32 } |  |  | 0.689 |
| ns | 4467 |  | 308 | readme: 256/Truecolor section and the `rgb`/`hex`/`ansi256` colour models | 3.6 | 1.5 | 0.680 |
| walker |  | 4546 | 220 | Code::CodeKey { rung: Decl, file: source/index.d.ts, decl: 3, sub: 5, line: 32 } |  |  | 0.713 |
| ns | 4561 |  | 94 | `source/index.js` imports and the stdout/stderr colour split | 4.1 |  | 0.705 |
| walker |  | 4833 | 287 | Code::CodeKey { rung: Decl, file: source/index.d.ts, decl: 3, sub: 6, line: 32 } |  |  | 0.747 |
| ns | 4843 |  | 282 | `source/index.js`: complete top-level declaration roster | 4.2 |  | 0.729 |
| walker |  | 4926 | 93 | Code::CodeKey { rung: Doc, file: source/index.d.ts, decl: 4, sub: 0, line: 242 } |  |  | 0.729 |
| ns | 4982 |  | 139 | `Chalk` class, `chalkFactory`, `createChalk` | 4.3 | 4.2 | 0.716 |
| ns | 5100 |  | 118 | `applyOptions`: level validation and auto-detection | 4.4 | 4.2 | 0.709 |
| walker |  | 5132 | 206 | Markdown::Section { file: readme.md, section_index: 14, keeps_default_concavity: false } |  |  | 0.709 |
| ns | 5270 |  | 170 | Style-property generation loop, `visible`, and prototype installation | 4.5 | 4.2 | 0.696 |
| walker |  | 5367 | 235 | Markdown::Section { file: readme.md, section_index: 11, keeps_default_concavity: false } |  |  | 0.711 |
| ns | 5461 |  | 191 | `createBuilder`: the chainable callable | 4.6 | 4.2 | 0.699 |
| walker |  | 5581 | 214 | Markdown::Section { file: readme.md, section_index: 15, keeps_default_concavity: false } |  |  | 0.699 |
| walker |  | 5603 | 22 | Code::CodeKey { rung: Names, file: source/vendor/supports-color/browser.js, decl: 0, sub: 0, line: 0 } |  |  | 0.699 |
| ns | 5607 |  | 146 | `createStyler`: the open/close linked list | 4.7 | 4.2 | 0.684 |
| walker |  | 5626 | 23 | Code::CodeKey { rung: Decl, file: source/vendor/supports-color/browser.js, decl: 1, sub: 0, line: 29 } |  |  | 0.684 |
| walker |  | 5639 | 13 | Code::CodeKey { rung: Names, file: source/vendor/supports-color/browser.d.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.684 |
| walker |  | 5678 | 39 | Code::CodeKey { rung: Names, file: source/vendor/supports-color/index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.684 |
| walker |  | 5725 | 47 | Code::CodeKey { rung: Decl, file: source/vendor/supports-color/index.js, decl: 2, sub: 0, line: 185 } |  |  | 0.684 |
| ns | 5907 |  | 300 | `applyStyle`: the string-wrapping algorithm | 4.8 | 4.2 | 0.662 |
| ns | 6000 |  | 93 | `proto` and the `level` getter/setter delegation | 4.9 | 4.2 | 0.655 |
| walker |  | 6043 | 318 | Markdown::Section { file: readme.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.686 |
| ns | 6056 |  | 56 | `levelMapping`: numeric level → ansi-styles method name | 4.10 | 4.2 | 0.681 |
| ns | 6255 |  | 199 | `getModelAnsi`: RGB/hex downsampling dispatch | 4.11 | 4.2 | 0.669 |
| walker |  | 6291 | 248 | Markdown::Section { file: readme.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.685 |
| ns | 6528 |  | 273 | Colour-model property generation for `rgb`/`hex`/`ansi256` and their `bg*` twins | 4.12 | 4.2 | 0.670 |
| ns | 6593 |  | 65 | `source/utilities.js`: both exported helpers, signatures only | 4.13 |  | 0.670 |
| walker |  | 6616 | 325 | Markdown::Section { file: readme.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.670 |
| ns | 6816 |  | 223 | supports-color: complete declaration roster and default export | 5.1 |  | 0.663 |
| walker |  | 6851 | 235 | Code::CodeKey { rung: Names, file: source/vendor/ansi-styles/index.d.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.663 |
| walker |  | 6922 | 71 | Code::CodeKey { rung: Decl, file: source/vendor/ansi-styles/index.d.ts, decl: 15, sub: 0, line: 229 } |  |  | 0.664 |
| ns | 6960 |  | 144 | supports-color: the `--color` / `--no-color` flag block | 5.2 | 5.1 | 0.654 |
| walker |  | 7002 | 80 | Code::CodeKey { rung: Decl, file: source/vendor/ansi-styles/index.d.ts, decl: 1, sub: 0, line: 1 } |  |  | 0.654 |
| ns | 7095 |  | 135 | supports-color: `envForceColor` (`FORCE_COLOR` parsing) | 5.3 | 5.1 | 0.647 |
| walker |  | 7103 | 101 | Code::CodeKey { rung: Decl, file: source/vendor/ansi-styles/index.d.ts, decl: 2, sub: 0, line: 13 } |  |  | 0.647 |
| ns | 7185 |  | 90 | supports-color: `translateLevel` — the `ColorInfo` shape | 5.4 | 5.1 | 0.641 |
| walker |  | 7375 | 272 | Code::CodeKey { rung: Decl, file: source/vendor/ansi-styles/index.d.ts, decl: 4, sub: 0, line: 75 } |  |  | 0.641 |
| ns | 7409 |  | 224 | `_supportsColor`: force-colour precedence and explicit `--color=` levels | 5.5 | 5.1 | 0.628 |
| walker |  | 7669 | 294 | Code::CodeKey { rung: Decl, file: source/vendor/ansi-styles/index.d.ts, decl: 5, sub: 0, line: 105 } |  |  | 0.628 |
| walker |  | 7685 | 16 | Code::CodeKey { rung: Doc, file: source/vendor/ansi-styles/index.d.ts, decl: 11, sub: 0, line: 212 } |  |  | 0.628 |
| walker |  | 7702 | 17 | Code::CodeKey { rung: Doc, file: source/vendor/ansi-styles/index.d.ts, decl: 12, sub: 0, line: 217 } |  |  | 0.628 |
| walker |  | 7719 | 17 | Code::CodeKey { rung: Doc, file: source/vendor/ansi-styles/index.d.ts, decl: 13, sub: 0, line: 222 } |  |  | 0.628 |
| ns | 7848 |  | 439 | `_supportsColor`: TTY, Windows and CI detection | 5.6 | 5.5 | 0.609 |
| walker |  | 8121 | 402 | Code::CodeKey { rung: Decl, file: source/vendor/ansi-styles/index.d.ts, decl: 3, sub: 0, line: 26 } |  |  | 0.609 |
| walker |  | 8139 | 18 | Code::CodeKey { rung: Doc, file: source/vendor/ansi-styles/index.d.ts, decl: 7, sub: 0, line: 186 } |  |  | 0.609 |
| walker |  | 8247 | 108 | Code::CodeKey { rung: Names, file: source/vendor/ansi-styles/index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.613 |
| ns | 8249 |  | 401 | `_supportsColor`: TeamCity, `COLORTERM` and terminal-emulator sniffing | 5.7 | 5.6 | 0.600 |
| ns | 8309 |  | 60 | supports-color: `createSupportsColor` body | 5.8 | 5.1 | 0.597 |
| walker |  | 8362 | 115 | Code::CodeKey { rung: Names, file: source/vendor/supports-color/index.d.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.597 |
| walker |  | 8385 | 23 | Code::CodeKey { rung: Decl, file: source/vendor/supports-color/index.d.ts, decl: 6, sub: 0, line: 50 } |  |  | 0.597 |
| walker |  | 8454 | 69 | Code::CodeKey { rung: Decl, file: source/vendor/supports-color/index.d.ts, decl: 1, sub: 0, line: 3 } |  |  | 0.598 |
| ns | 8474 |  | 165 | supports-color `index.d.ts`: complete type roster | 5.9 |  | 0.603 |
| walker |  | 8607 | 153 | Code::CodeKey { rung: Decl, file: source/vendor/supports-color/index.d.ts, decl: 3, sub: 0, line: 24 } |  |  | 0.604 |
| walker |  | 8626 | 19 | Code::CodeKey { rung: Doc, file: source/vendor/supports-color/index.d.ts, decl: 3, sub: 0, line: 24 } |  |  | 0.604 |
| ns | 8683 |  | 209 | supports-color browser build: `browser.js` level detection and export shape | 5.10 |  | 0.596 |
| ns | 8696 |  | 13 | `browser.d.ts` (one-line type re-export) | 5.11 |  | 0.596 |
| ns | 8768 |  | 72 | ava test harness setup in `test/chalk.js` | 6.1 |  | 0.594 |
| walker |  | 8809 | 183 | Code::CodeKey { rung: Decl, file: source/vendor/ansi-styles/index.d.ts, decl: 6, sub: 0, line: 135 } |  |  | 0.594 |
| walker |  | 9122 | 313 | Code::CodeKey { rung: Decl, file: source/vendor/ansi-styles/index.d.ts, decl: 6, sub: 1, line: 135 } |  |  | 0.594 |
| ns | 9189 |  | 421 | Every test title in `test/chalk.js` | 6.2 | 6.1 | 0.584 |
| ns | 9425 |  | 236 | Every test title in the four remaining ava files | 6.3 |  | 0.580 |
| ns | 9522 |  | 97 | `test/_fixture.js` in full, and the child process that runs it | 6.4 | 6.3 | 0.578 |
| walker |  | 9599 | 477 | Markdown::Section { file: readme.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.578 |
| ns | 9753 |  | 231 | ansi-styles `index.d.ts`: complete declaration roster | 7.1 |  | 0.586 |
| walker |  | 9804 | 205 | Markdown::Section { file: readme.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.586 |
| ns | 9896 |  | 143 | `source/index.test-d.ts`: what the tsd suite asserts | 7.2 |  | 0.580 |
| walker |  | 9972 | 168 | Markdown::Section { file: readme.md, section_index: 16, keeps_default_concavity: false } |  |  | 0.580 |
| ns | 9989 |  | 93 | `benchmark.js` and both `examples/` scripts: purpose lines | 7.3 |  | 0.578 |

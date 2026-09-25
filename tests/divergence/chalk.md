Score(3000)=0.649 I=0.862 C=0.488 ns_rows≤3K=21/59 grid(1000/1442/2080/3000/4327/6240/9000)=0.690/0.750/0.691/0.649/0.624/0.631/0.598

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 46 | 46 | listing of '.' |  |  | 0.000 |
| ns | 54 |  | 54 | package.json name, version, description, license | 1.1 |  | 0.000 |
| walker |  | 58 | 12 | listing of 'media' |  |  | 0.000 |
| walker |  | 80 | 22 | listing of 'source' |  |  | 0.000 |
| walker |  | 89 | 9 | listing of 'source/vendor' |  |  | 0.000 |
| ns | 100 |  | 46 | Complete repository root listing | 1.2 |  | 0.598 |
| walker |  | 107 | 18 | listing of 'source/vendor/supports-color' |  |  | 0.629 |
| walker |  | 116 | 9 | listing of 'source/vendor/ansi-styles' |  |  | 0.650 |
| ns | 172 |  | 72 | package.json entry points: `main`, `exports`, `repository`, `type` | 1.3 |  | 0.560 |
| walker |  | 182 | 66 | package identity in package.json |  |  | 0.892 |
| walker |  | 190 | 8 | listing of 'examples' |  |  | 0.892 |
| walker |  | 201 | 11 | listing of '.github' |  |  | 0.892 |
| walker |  | 205 | 4 | listing of '.github/workflows' |  |  | 0.892 |
| ns | 230 |  | 58 | Complete `source/` tree including both vendored packages | 1.4 |  | 0.914 |
| walker |  | 310 | 105 | README headline in readme.md |  |  | 0.915 |
| walker |  | 356 | 46 | package runtime metadata in package.json |  |  | 0.916 |
| ns | 361 |  | 131 | readme tagline + every `##` section heading | 1.5 |  | 0.767 |
| walker |  | 405 | 49 | package scripts in package.json |  |  | 0.770 |
| walker |  | 432 | 27 | listing of 'test' |  |  | 0.781 |
| ns | 568 |  | 207 | package.json `imports` subpath map, `types`, `engines`, `scripts` | 1.6 |  | 0.684 |
| ns | 708 |  | 140 | readme `###` subsection headings | 1.7 | 1.5 | 0.628 |
| ns | 770 |  | 62 | Complete listing of `test/`, `examples/`, `.github/`, `media/` | 1.8 |  | 0.674 |
| walker |  | 914 | 482 | ts names source/index.js |  |  | 0.690 |
| walker |  | 927 | 13 | ts decl source/index.js:34 |  |  | 0.690 |
| walker |  | 936 | 9 | ts body source/index.js:50 |  |  | 0.690 |
| walker |  | 974 | 38 | ts decl source/index.js:15 |  |  | 0.690 |
| walker |  | 1000 | 26 | ts body source/index.js:35 |  |  | 0.690 |
| ns | 1030 |  | 260 | CI workflow in full | 1.9 |  | 0.594 |
| walker |  | 1093 | 93 | ts decl source/index.js:119 |  |  | 0.596 |
| ns | 1244 |  | 214 | Runtime export surface of `source/index.js` | 2.1 |  | 0.646 |
| walker |  | 1353 | 260 | headings outline in readme.md |  |  | 0.775 |
| ns | 1358 |  | 114 | `source/index.d.ts` top-level declaration roster | 2.2 |  | 0.750 |
| walker |  | 1377 | 24 | readme.md section #12 |  |  | 0.750 |
| ns | 1458 |  | 100 | readme: chaining semantics of `chalk.<style>[.<style>...]` | 2.3 | 1.7 | 0.731 |
| walker |  | 1586 | 209 | readme.md section #8 |  |  | 0.733 |
| walker |  | 1623 | 37 | readme.md section #13 |  |  | 0.733 |
| ns | 1686 |  | 228 | readme: `chalk.level` and the 0–3 colour-level table | 2.4 | 1.7 | 0.682 |
| walker |  | 1798 | 175 | package entrypoints in package.json |  |  | 0.756 |
| ns | 1826 |  | 140 | `Options` interface body with level documentation | 2.5 | 2.2 | 0.725 |
| ns | 1970 |  | 144 | `ChalkInstance` call signature, `level`, and all six colour-model methods | 2.6 | 2.2 | 0.691 |
| ns | 2094 |  | 124 | All ten `ChalkInstance` modifier properties | 2.7 | 2.6 | 0.655 |
| walker |  | 2123 | 325 | readme.md section #9 |  |  | 0.655 |
| walker |  | 2165 | 42 | readme.md section #17 |  |  | 0.655 |
| ns | 2293 |  | 199 | All foreground colour properties, including the `gray`/`grey` aliases | 2.8 | 2.7 | 0.622 |
| ns | 2517 |  | 224 | All background colour properties and the close of `ChalkInstance` | 2.9 | 2.8 | 0.591 |
| walker |  | 2549 | 384 | ts names source/index.d.ts |  |  | 0.619 |
| walker |  | 2689 | 140 | ts decl source/index.d.ts:12 |  |  | 0.657 |
| walker |  | 2707 | 18 | ts doc source/index.d.ts:30 |  |  | 0.657 |
| walker |  | 2741 | 34 | ts doc source/index.d.ts:302 |  |  | 0.657 |
| ns | 2767 |  | 250 | readme: `supportsColor`, the `--color`/`FORCE_COLOR` overrides, `chalkStderr` | 2.10 | 1.7 | 0.643 |
| walker |  | 2777 | 36 | ts doc source/index.d.ts:268 |  |  | 0.643 |
| walker |  | 2790 | 13 | ts names source/vendor/supports-color/browser.d.ts |  |  | 0.643 |
| ns | 2832 |  | 65 | readme: the exported style-name arrays and their use | 2.11 | 1.7 | 0.636 |
| ns | 2962 |  | 130 | `index.d.ts` type re-export blocks from the vendored packages | 2.12 |  | 0.649 |
| walker |  | 3019 | 229 | package identity metadata in package.json |  |  | 0.666 |
| walker |  | 3055 | 36 | ts doc source/index.d.ts:309 |  |  | 0.667 |
| ns | 3093 |  | 131 | Deprecated type/const aliases in `index.d.ts` | 2.13 | 2.2 | 0.666 |
| walker |  | 3101 | 46 | ts names source/utilities.js |  |  | 0.666 |
| walker |  | 3192 | 91 | readme.md section #2 |  |  | 0.666 |
| ns | 3267 |  | 174 | ansi-styles: the `styles.modifier` code table | 3.1 |  | 0.650 |
| walker |  | 3300 | 108 | ts names source/vendor/ansi-styles/index.js |  |  | 0.651 |
| walker |  | 3336 | 36 | ts doc source/index.d.ts:316 |  |  | 0.653 |
| walker |  | 3476 | 140 | readme.md section #1 |  |  | 0.653 |
| walker |  | 3498 | 22 | ts names source/vendor/supports-color/browser.js |  |  | 0.653 |
| ns | 3571 |  | 304 | ansi-styles: the `styles.color` foreground code table | 3.2 | 3.1 | 0.629 |
| walker |  | 3733 | 235 | ts names source/vendor/ansi-styles/index.d.ts |  |  | 0.629 |
| walker |  | 3804 | 71 | ts decl source/vendor/ansi-styles/index.d.ts:229 |  |  | 0.630 |
| walker |  | 3884 | 80 | ts decl source/vendor/ansi-styles/index.d.ts:1 |  |  | 0.630 |
| ns | 3885 |  | 314 | ansi-styles: the `styles.bgColor` background code table | 3.3 | 3.2 | 0.607 |
| walker |  | 3900 | 16 | ts doc source/vendor/ansi-styles/index.d.ts:212 |  |  | 0.607 |
| walker |  | 3917 | 17 | ts doc source/vendor/ansi-styles/index.d.ts:217 |  |  | 0.607 |
| ns | 3950 |  | 65 | ansi-styles: the four exported style-name arrays | 3.4 | 3.3 | 0.611 |
| walker |  | 4018 | 101 | ts decl source/vendor/ansi-styles/index.d.ts:13 |  |  | 0.611 |
| walker |  | 4041 | 23 | ts decl source/vendor/supports-color/browser.js:29 |  |  | 0.611 |
| walker |  | 4101 | 60 | ts body source/index.js:41 |  |  | 0.612 |
| walker |  | 4118 | 17 | ts doc source/vendor/ansi-styles/index.d.ts:222 |  |  | 0.612 |
| ns | 4159 |  | 209 | readme: modifier list with human descriptions | 3.5 | 1.7 | 0.622 |
| walker |  | 4161 | 43 | ts doc source/index.d.ts:323 |  |  | 0.624 |
| walker |  | 4426 | 265 | ts decl source/index.d.ts:32 |  |  | 0.628 |
| walker |  | 4465 | 39 | ts names source/vendor/supports-color/index.js |  |  | 0.628 |
| ns | 4467 |  | 308 | readme: 256/Truecolor section and the `rgb`/`hex`/`ansi256` colour models | 3.6 | 1.5 | 0.616 |
| ns | 4561 |  | 94 | `source/index.js` imports and the stdout/stderr colour split | 4.1 |  | 0.610 |
| walker |  | 4737 | 272 | ts decl source/vendor/ansi-styles/index.d.ts:75 |  |  | 0.610 |
| ns | 4843 |  | 282 | `source/index.js`: complete top-level declaration roster | 4.2 |  | 0.615 |
| walker |  | 4906 | 169 | readme.md section #5 |  |  | 0.633 |
| ns | 4982 |  | 139 | `Chalk` class, `chalkFactory`, `createChalk` | 4.3 | 4.2 | 0.633 |
| ns | 5100 |  | 118 | `applyOptions`: level validation and auto-detection | 4.4 | 4.2 | 0.627 |
| walker |  | 5224 | 318 | readme.md section #6 |  |  | 0.664 |
| ns | 5270 |  | 170 | Style-property generation loop, `visible`, and prototype installation | 4.5 | 4.2 | 0.652 |
| ns | 5461 |  | 191 | `createBuilder`: the chainable callable | 4.6 | 4.2 | 0.641 |
| walker |  | 5518 | 294 | ts decl source/vendor/ansi-styles/index.d.ts:105 |  |  | 0.641 |
| walker |  | 5565 | 47 | ts decl source/vendor/supports-color/index.js:185 |  |  | 0.641 |
| ns | 5607 |  | 146 | `createStyler`: the open/close linked list | 4.7 | 4.2 | 0.628 |
| walker |  | 5748 | 183 | readme.md section #10 |  |  | 0.632 |
| ns | 5907 |  | 300 | `applyStyle`: the string-wrapping algorithm | 4.8 | 4.2 | 0.611 |
| walker |  | 5983 | 235 | readme.md section #11 |  |  | 0.626 |
| ns | 6000 |  | 93 | `proto` and the `level` getter/setter delegation | 4.9 | 4.2 | 0.633 |
| ns | 6056 |  | 56 | `levelMapping`: numeric level → ansi-styles method name | 4.10 | 4.2 | 0.631 |
| ns | 6255 |  | 199 | `getModelAnsi`: RGB/hex downsampling dispatch | 4.11 | 4.2 | 0.620 |
| walker |  | 6325 | 342 | ts decl source/index.d.ts:32 #1 |  |  | 0.629 |
| ns | 6528 |  | 273 | Colour-model property generation for `rgb`/`hex`/`ansi256` and their `bg*` twins | 4.12 | 4.2 | 0.616 |
| ns | 6593 |  | 65 | `source/utilities.js`: both exported helpers, signatures only | 4.13 |  | 0.616 |
| walker |  | 6727 | 402 | ts decl source/vendor/ansi-styles/index.d.ts:26 |  |  | 0.616 |
| ns | 6816 |  | 223 | supports-color: complete declaration roster and default export | 5.1 |  | 0.610 |
| ns | 6960 |  | 144 | supports-color: the `--color` / `--no-color` flag block | 5.2 | 5.1 | 0.601 |
| walker |  | 6975 | 248 | readme.md section #7 |  |  | 0.616 |
| walker |  | 7090 | 115 | ts body source/index.js:24 |  |  | 0.624 |
| ns | 7095 |  | 135 | supports-color: `envForceColor` (`FORCE_COLOR` parsing) | 5.3 | 5.1 | 0.618 |
| ns | 7185 |  | 90 | supports-color: `translateLevel` — the `ColorInfo` shape | 5.4 | 5.1 | 0.611 |
| walker |  | 7296 | 206 | readme.md section #14 |  |  | 0.611 |
| ns | 7409 |  | 224 | `_supportsColor`: force-colour precedence and explicit `--color=` levels | 5.5 | 5.1 | 0.600 |
| walker |  | 7510 | 214 | readme.md section #15 |  |  | 0.600 |
| walker |  | 7767 | 257 | ts decl source/vendor/ansi-styles/index.d.ts:135 |  |  | 0.600 |
| ns | 7848 |  | 439 | `_supportsColor`: TTY, Windows and CI detection | 5.6 | 5.5 | 0.581 |
| walker |  | 8077 | 310 | ts decl source/index.d.ts:32 #2 |  |  | 0.591 |
| walker |  | 8093 | 16 | ts doc source/vendor/ansi-styles/index.d.ts:186 |  |  | 0.591 |
| walker |  | 8236 | 143 | ts body source/index.js:132 |  |  | 0.610 |
| ns | 8249 |  | 401 | `_supportsColor`: TeamCity, `COLORTERM` and terminal-emulator sniffing | 5.7 | 5.6 | 0.597 |
| ns | 8309 |  | 60 | supports-color: `createSupportsColor` body | 5.8 | 5.1 | 0.593 |
| walker |  | 8351 | 115 | ts names source/vendor/supports-color/index.d.ts |  |  | 0.594 |
| walker |  | 8424 | 73 | ts doc source/index.d.ts:277 |  |  | 0.595 |
| ns | 8474 |  | 165 | supports-color `index.d.ts`: complete type roster | 5.9 |  | 0.593 |
| walker |  | 8497 | 73 | ts doc source/index.d.ts:286 |  |  | 0.594 |
| walker |  | 8577 | 80 | ts doc source/index.d.ts:295 |  |  | 0.595 |
| walker |  | 8670 | 93 | ts doc source/index.d.ts:242 |  |  | 0.595 |
| ns | 8683 |  | 209 | supports-color browser build: `browser.js` level detection and export shape | 5.10 |  | 0.587 |
| walker |  | 8693 | 23 | ts decl source/vendor/supports-color/index.d.ts:50 |  |  | 0.587 |
| ns | 8696 |  | 13 | `browser.d.ts` (one-line type re-export) | 5.11 |  | 0.588 |
| ns | 8768 |  | 72 | ava test harness setup in `test/chalk.js` | 6.1 |  | 0.585 |
| walker |  | 8881 | 188 | ts body source/index.js:152 |  |  | 0.598 |
| walker |  | 9077 | 196 | ts body source/index.js:74 |  |  | 0.615 |
| walker |  | 9131 | 54 | ts doc source/vendor/ansi-styles/index.d.ts:193 |  |  | 0.615 |
| walker |  | 9185 | 54 | ts doc source/vendor/ansi-styles/index.d.ts:200 |  |  | 0.615 |
| ns | 9189 |  | 421 | Every test title in `test/chalk.js` | 6.2 | 6.1 | 0.605 |
| walker |  | 9247 | 62 | ts doc source/vendor/ansi-styles/index.d.ts:207 |  |  | 0.605 |
| walker |  | 9316 | 69 | ts decl source/vendor/supports-color/index.d.ts:3 |  |  | 0.612 |
| ns | 9425 |  | 236 | Every test title in the four remaining ava files | 6.3 |  | 0.608 |
| ns | 9522 |  | 97 | `test/_fixture.js` in full, and the child process that runs it | 6.4 | 6.3 | 0.606 |
| walker |  | 9723 | 407 | ts body source/index.js:168 |  |  | 0.632 |
| ns | 9753 |  | 231 | ansi-styles `index.d.ts`: complete declaration roster | 7.1 |  | 0.638 |
| ns | 9896 |  | 143 | `source/index.test-d.ts`: what the tsd suite asserts | 7.2 |  | 0.632 |
| walker |  | 9964 | 241 | ts decl source/vendor/ansi-styles/index.d.ts:135 #1 |  |  | 0.632 |
| ns | 9989 |  | 93 | `benchmark.js` and both `examples/` scripts: purpose lines | 7.3 |  | 0.629 |

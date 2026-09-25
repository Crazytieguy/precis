Score(3000)=0.666 I=0.885 C=0.500 ns_rows≤3K=21/59 grid(1000/1442/2080/3000/4327/6240/9000)=0.690/0.750/0.721/0.666/0.624/0.623/0.636

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
| walker |  | 1377 | 24 | readme.md section #9 |  |  | 0.750 |
| walker |  | 1414 | 37 | readme.md section #10 |  |  | 0.750 |
| ns | 1458 |  | 100 | readme: chaining semantics of `chalk.<style>[.<style>...]` | 2.3 | 1.7 | 0.731 |
| walker |  | 1589 | 175 | package entrypoints in package.json |  |  | 0.809 |
| walker |  | 1631 | 42 | readme.md section #13 |  |  | 0.809 |
| ns | 1686 |  | 228 | readme: `chalk.level` and the 0–3 colour-level table | 2.4 | 1.7 | 0.754 |
| ns | 1826 |  | 140 | `Options` interface body with level documentation | 2.5 | 2.2 | 0.722 |
| ns | 1970 |  | 144 | `ChalkInstance` call signature, `level`, and all six colour-model methods | 2.6 | 2.2 | 0.688 |
| walker |  | 2015 | 384 | ts names source/index.d.ts |  |  | 0.721 |
| ns | 2094 |  | 124 | All ten `ChalkInstance` modifier properties | 2.7 | 2.6 | 0.685 |
| walker |  | 2155 | 140 | ts decl source/index.d.ts:12 |  |  | 0.726 |
| walker |  | 2173 | 18 | ts doc source/index.d.ts:30 |  |  | 0.726 |
| walker |  | 2207 | 34 | ts doc source/index.d.ts:302 |  |  | 0.727 |
| walker |  | 2243 | 36 | ts doc source/index.d.ts:268 |  |  | 0.727 |
| walker |  | 2256 | 13 | ts names source/vendor/supports-color/browser.d.ts |  |  | 0.727 |
| ns | 2293 |  | 199 | All foreground colour properties, including the `gray`/`grey` aliases | 2.8 | 2.7 | 0.690 |
| walker |  | 2485 | 229 | package identity metadata in package.json |  |  | 0.709 |
| ns | 2517 |  | 224 | All background colour properties and the close of `ChalkInstance` | 2.9 | 2.8 | 0.675 |
| walker |  | 2521 | 36 | ts doc source/index.d.ts:309 |  |  | 0.675 |
| walker |  | 2567 | 46 | ts names source/utilities.js |  |  | 0.675 |
| walker |  | 2658 | 91 | readme.md section #2 |  |  | 0.675 |
| walker |  | 2766 | 108 | ts names source/vendor/ansi-styles/index.js |  |  | 0.676 |
| ns | 2767 |  | 250 | readme: `supportsColor`, the `--color`/`FORCE_COLOR` overrides, `chalkStderr` | 2.10 | 1.7 | 0.660 |
| ns | 2832 |  | 65 | readme: the exported style-name arrays and their use | 2.11 | 1.7 | 0.653 |
| ns | 2962 |  | 130 | `index.d.ts` type re-export blocks from the vendored packages | 2.12 |  | 0.666 |
| ns | 3093 |  | 131 | Deprecated type/const aliases in `index.d.ts` | 2.13 | 2.2 | 0.665 |
| ns | 3267 |  | 174 | ansi-styles: the `styles.modifier` code table | 3.1 |  | 0.649 |
| walker |  | 3305 | 539 | readme.md section #7 |  |  | 0.651 |
| walker |  | 3341 | 36 | ts doc source/index.d.ts:316 |  |  | 0.653 |
| walker |  | 3481 | 140 | readme.md section #1 |  |  | 0.653 |
| walker |  | 3503 | 22 | ts names source/vendor/supports-color/browser.js |  |  | 0.653 |
| ns | 3571 |  | 304 | ansi-styles: the `styles.color` foreground code table | 3.2 | 3.1 | 0.629 |
| walker |  | 3738 | 235 | ts names source/vendor/ansi-styles/index.d.ts |  |  | 0.629 |
| walker |  | 3809 | 71 | ts decl source/vendor/ansi-styles/index.d.ts:229 |  |  | 0.630 |
| ns | 3885 |  | 314 | ansi-styles: the `styles.bgColor` background code table | 3.3 | 3.2 | 0.607 |
| walker |  | 3889 | 80 | ts decl source/vendor/ansi-styles/index.d.ts:1 |  |  | 0.607 |
| walker |  | 3905 | 16 | ts doc source/vendor/ansi-styles/index.d.ts:212 |  |  | 0.607 |
| walker |  | 3922 | 17 | ts doc source/vendor/ansi-styles/index.d.ts:217 |  |  | 0.607 |
| ns | 3950 |  | 65 | ansi-styles: the four exported style-name arrays | 3.4 | 3.3 | 0.611 |
| walker |  | 4023 | 101 | ts decl source/vendor/ansi-styles/index.d.ts:13 |  |  | 0.611 |
| walker |  | 4046 | 23 | ts decl source/vendor/supports-color/browser.js:29 |  |  | 0.611 |
| walker |  | 4106 | 60 | ts body source/index.js:41 |  |  | 0.612 |
| walker |  | 4123 | 17 | ts doc source/vendor/ansi-styles/index.d.ts:222 |  |  | 0.612 |
| ns | 4159 |  | 209 | readme: modifier list with human descriptions | 3.5 | 1.7 | 0.622 |
| walker |  | 4166 | 43 | ts doc source/index.d.ts:323 |  |  | 0.624 |
| walker |  | 4431 | 265 | ts decl source/index.d.ts:32 |  |  | 0.628 |
| ns | 4467 |  | 308 | readme: 256/Truecolor section and the `rgb`/`hex`/`ansi256` colour models | 3.6 | 1.5 | 0.616 |
| walker |  | 4470 | 39 | ts names source/vendor/supports-color/index.js |  |  | 0.616 |
| ns | 4561 |  | 94 | `source/index.js` imports and the stdout/stderr colour split | 4.1 |  | 0.610 |
| walker |  | 4742 | 272 | ts decl source/vendor/ansi-styles/index.d.ts:75 |  |  | 0.610 |
| ns | 4843 |  | 282 | `source/index.js`: complete top-level declaration roster | 4.2 |  | 0.615 |
| walker |  | 4911 | 169 | readme.md section #4 |  |  | 0.633 |
| ns | 4982 |  | 139 | `Chalk` class, `chalkFactory`, `createChalk` | 4.3 | 4.2 | 0.633 |
| ns | 5100 |  | 118 | `applyOptions`: level validation and auto-detection | 4.4 | 4.2 | 0.627 |
| walker |  | 5229 | 318 | readme.md section #5 |  |  | 0.664 |
| ns | 5270 |  | 170 | Style-property generation loop, `visible`, and prototype installation | 4.5 | 4.2 | 0.652 |
| ns | 5461 |  | 191 | `createBuilder`: the chainable callable | 4.6 | 4.2 | 0.641 |
| walker |  | 5523 | 294 | ts decl source/vendor/ansi-styles/index.d.ts:105 |  |  | 0.641 |
| walker |  | 5570 | 47 | ts decl source/vendor/supports-color/index.js:185 |  |  | 0.641 |
| ns | 5607 |  | 146 | `createStyler`: the open/close linked list | 4.7 | 4.2 | 0.628 |
| ns | 5907 |  | 300 | `applyStyle`: the string-wrapping algorithm | 4.8 | 4.2 | 0.607 |
| walker |  | 5912 | 342 | ts decl source/index.d.ts:32 #1 |  |  | 0.618 |
| ns | 6000 |  | 93 | `proto` and the `level` getter/setter delegation | 4.9 | 4.2 | 0.625 |
| ns | 6056 |  | 56 | `levelMapping`: numeric level → ansi-styles method name | 4.10 | 4.2 | 0.623 |
| ns | 6255 |  | 199 | `getModelAnsi`: RGB/hex downsampling dispatch | 4.11 | 4.2 | 0.612 |
| walker |  | 6314 | 402 | ts decl source/vendor/ansi-styles/index.d.ts:26 |  |  | 0.612 |
| walker |  | 6429 | 115 | ts body source/index.js:24 |  |  | 0.621 |
| ns | 6528 |  | 273 | Colour-model property generation for `rgb`/`hex`/`ansi256` and their `bg*` twins | 4.12 | 4.2 | 0.608 |
| ns | 6593 |  | 65 | `source/utilities.js`: both exported helpers, signatures only | 4.13 |  | 0.608 |
| walker |  | 6686 | 257 | ts decl source/vendor/ansi-styles/index.d.ts:135 |  |  | 0.608 |
| ns | 6816 |  | 223 | supports-color: complete declaration roster and default export | 5.1 |  | 0.602 |
| ns | 6960 |  | 144 | supports-color: the `--color` / `--no-color` flag block | 5.2 | 5.1 | 0.594 |
| walker |  | 6996 | 310 | ts decl source/index.d.ts:32 #2 |  |  | 0.605 |
| walker |  | 7012 | 16 | ts doc source/vendor/ansi-styles/index.d.ts:186 |  |  | 0.605 |
| ns | 7095 |  | 135 | supports-color: `envForceColor` (`FORCE_COLOR` parsing) | 5.3 | 5.1 | 0.598 |
| ns | 7185 |  | 90 | supports-color: `translateLevel` — the `ColorInfo` shape | 5.4 | 5.1 | 0.592 |
| walker |  | 7260 | 248 | readme.md section #6 |  |  | 0.606 |
| walker |  | 7403 | 143 | ts body source/index.js:132 |  |  | 0.626 |
| ns | 7409 |  | 224 | `_supportsColor`: force-colour precedence and explicit `--color=` levels | 5.5 | 5.1 | 0.614 |
| walker |  | 7518 | 115 | ts names source/vendor/supports-color/index.d.ts |  |  | 0.615 |
| walker |  | 7591 | 73 | ts doc source/index.d.ts:277 |  |  | 0.616 |
| walker |  | 7664 | 73 | ts doc source/index.d.ts:286 |  |  | 0.617 |
| walker |  | 7744 | 80 | ts doc source/index.d.ts:295 |  |  | 0.618 |
| walker |  | 7837 | 93 | ts doc source/index.d.ts:242 |  |  | 0.618 |
| ns | 7848 |  | 439 | `_supportsColor`: TTY, Windows and CI detection | 5.6 | 5.5 | 0.599 |
| walker |  | 7860 | 23 | ts decl source/vendor/supports-color/index.d.ts:50 |  |  | 0.599 |
| walker |  | 8048 | 188 | ts body source/index.js:152 |  |  | 0.613 |
| walker |  | 8244 | 196 | ts body source/index.js:74 |  |  | 0.631 |
| ns | 8249 |  | 401 | `_supportsColor`: TeamCity, `COLORTERM` and terminal-emulator sniffing | 5.7 | 5.6 | 0.618 |
| walker |  | 8298 | 54 | ts doc source/vendor/ansi-styles/index.d.ts:193 |  |  | 0.618 |
| ns | 8309 |  | 60 | supports-color: `createSupportsColor` body | 5.8 | 5.1 | 0.614 |
| walker |  | 8352 | 54 | ts doc source/vendor/ansi-styles/index.d.ts:200 |  |  | 0.614 |
| walker |  | 8414 | 62 | ts doc source/vendor/ansi-styles/index.d.ts:207 |  |  | 0.614 |
| ns | 8474 |  | 165 | supports-color `index.d.ts`: complete type roster | 5.9 |  | 0.613 |
| walker |  | 8483 | 69 | ts decl source/vendor/supports-color/index.d.ts:3 |  |  | 0.620 |
| ns | 8683 |  | 209 | supports-color browser build: `browser.js` level detection and export shape | 5.10 |  | 0.611 |
| ns | 8696 |  | 13 | `browser.d.ts` (one-line type re-export) | 5.11 |  | 0.612 |
| ns | 8768 |  | 72 | ava test harness setup in `test/chalk.js` | 6.1 |  | 0.609 |
| walker |  | 8890 | 407 | ts body source/index.js:168 |  |  | 0.636 |
| walker |  | 9131 | 241 | ts decl source/vendor/ansi-styles/index.d.ts:135 #1 |  |  | 0.636 |
| ns | 9189 |  | 421 | Every test title in `test/chalk.js` | 6.2 | 6.1 | 0.626 |
| ns | 9425 |  | 236 | Every test title in the four remaining ava files | 6.3 |  | 0.622 |
| walker |  | 9447 | 316 | ts decl source/index.d.ts:32 #3 |  |  | 0.638 |
| ns | 9522 |  | 97 | `test/_fixture.js` in full, and the child process that runs it | 6.4 | 6.3 | 0.635 |
| ns | 9753 |  | 231 | ansi-styles `index.d.ts`: complete declaration roster | 7.1 |  | 0.641 |
| walker |  | 9870 | 423 | readme.md section #8 |  |  | 0.653 |
| ns | 9896 |  | 143 | `source/index.test-d.ts`: what the tsd suite asserts | 7.2 |  | 0.647 |
| ns | 9989 |  | 93 | `benchmark.js` and both `examples/` scripts: purpose lines | 7.3 |  | 0.644 |

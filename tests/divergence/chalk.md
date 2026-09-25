Score(3000)=0.649 I=0.863 C=0.488 ns_rows≤3K=21/59 grid(1000/1442/2080/3000/4327/6240/9000)=0.690/0.750/0.691/0.649/0.626/0.634/0.592

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
| walker |  | 2569 | 20 | ts doc source/index.d.ts:30 |  |  | 0.619 |
| walker |  | 2707 | 138 | ts decl source/index.d.ts:12 |  |  | 0.657 |
| walker |  | 2741 | 34 | ts doc source/index.d.ts:302 |  |  | 0.657 |
| ns | 2767 |  | 250 | readme: `supportsColor`, the `--color`/`FORCE_COLOR` overrides, `chalkStderr` | 2.10 | 1.7 | 0.643 |
| walker |  | 2777 | 36 | ts doc source/index.d.ts:268 |  |  | 0.643 |
| walker |  | 2813 | 36 | ts doc source/index.d.ts:309 |  |  | 0.643 |
| ns | 2832 |  | 65 | readme: the exported style-name arrays and their use | 2.11 | 1.7 | 0.636 |
| walker |  | 2849 | 36 | ts doc source/index.d.ts:316 |  |  | 0.636 |
| walker |  | 2862 | 13 | ts names source/vendor/supports-color/browser.d.ts |  |  | 0.636 |
| walker |  | 2905 | 43 | ts doc source/index.d.ts:323 |  |  | 0.637 |
| ns | 2962 |  | 130 | `index.d.ts` type re-export blocks from the vendored packages | 2.12 |  | 0.649 |
| ns | 3093 |  | 131 | Deprecated type/const aliases in `index.d.ts` | 2.13 | 2.2 | 0.652 |
| walker |  | 3134 | 229 | package identity metadata in package.json |  |  | 0.670 |
| walker |  | 3180 | 46 | ts names source/utilities.js |  |  | 0.670 |
| ns | 3267 |  | 174 | ansi-styles: the `styles.modifier` code table | 3.1 |  | 0.654 |
| walker |  | 3271 | 91 | readme.md section #2 |  |  | 0.654 |
| walker |  | 3379 | 108 | ts names source/vendor/ansi-styles/index.js |  |  | 0.655 |
| walker |  | 3519 | 140 | readme.md section #1 |  |  | 0.655 |
| walker |  | 3541 | 22 | ts names source/vendor/supports-color/browser.js |  |  | 0.655 |
| ns | 3571 |  | 304 | ansi-styles: the `styles.color` foreground code table | 3.2 | 3.1 | 0.631 |
| walker |  | 3776 | 235 | ts names source/vendor/ansi-styles/index.d.ts |  |  | 0.631 |
| walker |  | 3847 | 71 | ts decl source/vendor/ansi-styles/index.d.ts:229 |  |  | 0.632 |
| ns | 3885 |  | 314 | ansi-styles: the `styles.bgColor` background code table | 3.3 | 3.2 | 0.609 |
| walker |  | 3927 | 80 | ts decl source/vendor/ansi-styles/index.d.ts:1 |  |  | 0.609 |
| walker |  | 3943 | 16 | ts doc source/vendor/ansi-styles/index.d.ts:212 |  |  | 0.609 |
| ns | 3950 |  | 65 | ansi-styles: the four exported style-name arrays | 3.4 | 3.3 | 0.612 |
| walker |  | 3960 | 17 | ts doc source/vendor/ansi-styles/index.d.ts:217 |  |  | 0.612 |
| walker |  | 4061 | 101 | ts decl source/vendor/ansi-styles/index.d.ts:13 |  |  | 0.612 |
| walker |  | 4084 | 23 | ts decl source/vendor/supports-color/browser.js:29 |  |  | 0.612 |
| walker |  | 4144 | 60 | ts body source/index.js:41 |  |  | 0.613 |
| ns | 4159 |  | 209 | readme: modifier list with human descriptions | 3.5 | 1.7 | 0.624 |
| walker |  | 4217 | 73 | ts doc source/index.d.ts:277 |  |  | 0.626 |
| walker |  | 4234 | 17 | ts doc source/vendor/ansi-styles/index.d.ts:222 |  |  | 0.626 |
| ns | 4467 |  | 308 | readme: 256/Truecolor section and the `rgb`/`hex`/`ansi256` colour models | 3.6 | 1.5 | 0.614 |
| walker |  | 4499 | 265 | ts decl source/index.d.ts:32 |  |  | 0.618 |
| walker |  | 4538 | 39 | ts names source/vendor/supports-color/index.js |  |  | 0.618 |
| ns | 4561 |  | 94 | `source/index.js` imports and the stdout/stderr colour split | 4.1 |  | 0.611 |
| walker |  | 4810 | 272 | ts decl source/vendor/ansi-styles/index.d.ts:75 |  |  | 0.611 |
| ns | 4843 |  | 282 | `source/index.js`: complete top-level declaration roster | 4.2 |  | 0.617 |
| walker |  | 4979 | 169 | readme.md section #5 |  |  | 0.634 |
| ns | 4982 |  | 139 | `Chalk` class, `chalkFactory`, `createChalk` | 4.3 | 4.2 | 0.635 |
| ns | 5100 |  | 118 | `applyOptions`: level validation and auto-detection | 4.4 | 4.2 | 0.628 |
| ns | 5270 |  | 170 | Style-property generation loop, `visible`, and prototype installation | 4.5 | 4.2 | 0.616 |
| walker |  | 5297 | 318 | readme.md section #6 |  |  | 0.653 |
| ns | 5461 |  | 191 | `createBuilder`: the chainable callable | 4.6 | 4.2 | 0.642 |
| walker |  | 5591 | 294 | ts decl source/vendor/ansi-styles/index.d.ts:105 |  |  | 0.642 |
| ns | 5607 |  | 146 | `createStyler`: the open/close linked list | 4.7 | 4.2 | 0.629 |
| walker |  | 5638 | 47 | ts decl source/vendor/supports-color/index.js:185 |  |  | 0.629 |
| walker |  | 5711 | 73 | ts doc source/index.d.ts:286 |  |  | 0.630 |
| walker |  | 5894 | 183 | readme.md section #10 |  |  | 0.634 |
| ns | 5907 |  | 300 | `applyStyle`: the string-wrapping algorithm | 4.8 | 4.2 | 0.614 |
| ns | 6000 |  | 93 | `proto` and the `level` getter/setter delegation | 4.9 | 4.2 | 0.621 |
| ns | 6056 |  | 56 | `levelMapping`: numeric level → ansi-styles method name | 4.10 | 4.2 | 0.620 |
| walker |  | 6129 | 235 | readme.md section #11 |  |  | 0.634 |
| ns | 6255 |  | 199 | `getModelAnsi`: RGB/hex downsampling dispatch | 4.11 | 4.2 | 0.622 |
| walker |  | 6471 | 342 | ts decl source/index.d.ts:32 #1 |  |  | 0.632 |
| ns | 6528 |  | 273 | Colour-model property generation for `rgb`/`hex`/`ansi256` and their `bg*` twins | 4.12 | 4.2 | 0.619 |
| ns | 6593 |  | 65 | `source/utilities.js`: both exported helpers, signatures only | 4.13 |  | 0.619 |
| ns | 6816 |  | 223 | supports-color: complete declaration roster and default export | 5.1 |  | 0.612 |
| walker |  | 6873 | 402 | ts decl source/vendor/ansi-styles/index.d.ts:26 |  |  | 0.612 |
| ns | 6960 |  | 144 | supports-color: the `--color` / `--no-color` flag block | 5.2 | 5.1 | 0.604 |
| ns | 7095 |  | 135 | supports-color: `envForceColor` (`FORCE_COLOR` parsing) | 5.3 | 5.1 | 0.597 |
| walker |  | 7121 | 248 | readme.md section #7 |  |  | 0.611 |
| ns | 7185 |  | 90 | supports-color: `translateLevel` — the `ColorInfo` shape | 5.4 | 5.1 | 0.605 |
| walker |  | 7236 | 115 | ts body source/index.js:24 |  |  | 0.614 |
| ns | 7409 |  | 224 | `_supportsColor`: force-colour precedence and explicit `--color=` levels | 5.5 | 5.1 | 0.602 |
| walker |  | 7442 | 206 | readme.md section #14 |  |  | 0.602 |
| walker |  | 7656 | 214 | readme.md section #15 |  |  | 0.602 |
| ns | 7848 |  | 439 | `_supportsColor`: TTY, Windows and CI detection | 5.6 | 5.5 | 0.583 |
| walker |  | 7913 | 257 | ts decl source/vendor/ansi-styles/index.d.ts:135 |  |  | 0.583 |
| walker |  | 7993 | 80 | ts doc source/index.d.ts:295 |  |  | 0.584 |
| ns | 8249 |  | 401 | `_supportsColor`: TeamCity, `COLORTERM` and terminal-emulator sniffing | 5.7 | 5.6 | 0.571 |
| walker |  | 8303 | 310 | ts decl source/index.d.ts:32 #2 |  |  | 0.581 |
| ns | 8309 |  | 60 | supports-color: `createSupportsColor` body | 5.8 | 5.1 | 0.578 |
| walker |  | 8319 | 16 | ts doc source/vendor/ansi-styles/index.d.ts:186 |  |  | 0.578 |
| walker |  | 8462 | 143 | ts body source/index.js:132 |  |  | 0.596 |
| ns | 8474 |  | 165 | supports-color `index.d.ts`: complete type roster | 5.9 |  | 0.590 |
| walker |  | 8577 | 115 | ts names source/vendor/supports-color/index.d.ts |  |  | 0.595 |
| walker |  | 8600 | 23 | ts decl source/vendor/supports-color/index.d.ts:50 |  |  | 0.596 |
| walker |  | 8669 | 69 | ts decl source/vendor/supports-color/index.d.ts:3 |  |  | 0.603 |
| ns | 8683 |  | 209 | supports-color browser build: `browser.js` level detection and export shape | 5.10 |  | 0.595 |
| ns | 8696 |  | 13 | `browser.d.ts` (one-line type re-export) | 5.11 |  | 0.595 |
| ns | 8768 |  | 72 | ava test harness setup in `test/chalk.js` | 6.1 |  | 0.592 |
| walker |  | 8910 | 241 | ts decl source/vendor/ansi-styles/index.d.ts:135 #1 |  |  | 0.592 |
| walker |  | 9003 | 93 | ts doc source/index.d.ts:242 |  |  | 0.592 |
| ns | 9189 |  | 421 | Every test title in `test/chalk.js` | 6.2 | 6.1 | 0.583 |
| walker |  | 9319 | 316 | ts decl source/index.d.ts:32 #3 |  |  | 0.600 |
| ns | 9425 |  | 236 | Every test title in the four remaining ava files | 6.3 |  | 0.595 |
| ns | 9522 |  | 97 | `test/_fixture.js` in full, and the child process that runs it | 6.4 | 6.3 | 0.593 |
| ns | 9753 |  | 231 | ansi-styles `index.d.ts`: complete declaration roster | 7.1 |  | 0.600 |
| walker |  | 9796 | 477 | readme.md section #3 |  |  | 0.600 |
| ns | 9896 |  | 143 | `source/index.test-d.ts`: what the tsd suite asserts | 7.2 |  | 0.595 |
| ns | 9989 |  | 93 | `benchmark.js` and both `examples/` scripts: purpose lines | 7.3 |  | 0.592 |

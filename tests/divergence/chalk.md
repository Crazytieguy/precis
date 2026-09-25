Score(3000)=0.669 I=0.886 C=0.505 ns_rows≤3K=21/59 grid(1000/1442/2080/3000/4327/6240/9000)=0.690/0.750/0.691/0.669/0.636/0.592/0.601

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
| walker |  | 1377 | 24 | readme.md section #30 |  |  | 0.750 |
| ns | 1458 |  | 100 | readme: chaining semantics of `chalk.<style>[.<style>...]` | 2.3 | 1.7 | 0.731 |
| walker |  | 1527 | 150 | readme.md section #27 |  |  | 0.731 |
| ns | 1686 |  | 228 | readme: `chalk.level` and the 0–3 colour-level table | 2.4 | 1.7 | 0.680 |
| walker |  | 1697 | 170 | readme.md section #28 |  |  | 0.680 |
| walker |  | 1709 | 12 | readme.md section #6 |  |  | 0.682 |
| walker |  | 1721 | 12 | readme.md section #7 |  |  | 0.683 |
| walker |  | 1758 | 37 | readme.md section #31 |  |  | 0.683 |
| ns | 1826 |  | 140 | `Options` interface body with level documentation | 2.5 | 2.2 | 0.655 |
| walker |  | 1933 | 175 | package entrypoints in package.json |  |  | 0.725 |
| walker |  | 1947 | 14 | readme.md section #17 |  |  | 0.725 |
| walker |  | 1961 | 14 | readme.md section #18 |  |  | 0.725 |
| ns | 1970 |  | 144 | `ChalkInstance` call signature, `level`, and all six colour-model methods | 2.6 | 2.2 | 0.691 |
| walker |  | 1977 | 16 | readme.md section #19 |  |  | 0.691 |
| walker |  | 2019 | 42 | readme.md section #40 |  |  | 0.691 |
| ns | 2094 |  | 124 | All ten `ChalkInstance` modifier properties | 2.7 | 2.6 | 0.656 |
| ns | 2293 |  | 199 | All foreground colour properties, including the `gray`/`grey` aliases | 2.8 | 2.7 | 0.622 |
| walker |  | 2403 | 384 | ts names source/index.d.ts |  |  | 0.652 |
| ns | 2517 |  | 224 | All background colour properties and the close of `ChalkInstance` | 2.9 | 2.8 | 0.620 |
| walker |  | 2543 | 140 | ts decl source/index.d.ts:12 |  |  | 0.657 |
| walker |  | 2561 | 18 | ts doc source/index.d.ts:30 |  |  | 0.657 |
| walker |  | 2595 | 34 | ts doc source/index.d.ts:302 |  |  | 0.658 |
| walker |  | 2631 | 36 | ts doc source/index.d.ts:268 |  |  | 0.658 |
| walker |  | 2650 | 19 | readme.md section #4 |  |  | 0.660 |
| walker |  | 2668 | 18 | readme.md section #23 |  |  | 0.660 |
| walker |  | 2685 | 17 | readme.md section #24 |  |  | 0.660 |
| walker |  | 2705 | 20 | readme.md section #20 |  |  | 0.660 |
| walker |  | 2718 | 13 | ts names source/vendor/supports-color/browser.d.ts |  |  | 0.661 |
| ns | 2767 |  | 250 | readme: `supportsColor`, the `--color`/`FORCE_COLOR` overrides, `chalkStderr` | 2.10 | 1.7 | 0.646 |
| ns | 2832 |  | 65 | readme: the exported style-name arrays and their use | 2.11 | 1.7 | 0.639 |
| walker |  | 2947 | 229 | package identity metadata in package.json |  |  | 0.657 |
| ns | 2962 |  | 130 | `index.d.ts` type re-export blocks from the vendored packages | 2.12 |  | 0.669 |
| walker |  | 2983 | 36 | ts doc source/index.d.ts:309 |  |  | 0.669 |
| walker |  | 3029 | 46 | ts names source/utilities.js |  |  | 0.669 |
| walker |  | 3052 | 23 | readme.md section #21 |  |  | 0.670 |
| walker |  | 3074 | 22 | readme.md section #22 |  |  | 0.670 |
| ns | 3093 |  | 131 | Deprecated type/const aliases in `index.d.ts` | 2.13 | 2.2 | 0.669 |
| walker |  | 3165 | 91 | readme.md section #2 |  |  | 0.669 |
| ns | 3267 |  | 174 | ansi-styles: the `styles.modifier` code table | 3.1 |  | 0.653 |
| walker |  | 3273 | 108 | ts names source/vendor/ansi-styles/index.js |  |  | 0.654 |
| walker |  | 3294 | 21 | readme.md section #37 |  |  | 0.654 |
| walker |  | 3330 | 36 | ts doc source/index.d.ts:316 |  |  | 0.656 |
| walker |  | 3360 | 30 | readme.md section #25 |  |  | 0.656 |
| walker |  | 3390 | 30 | readme.md section #26 |  |  | 0.657 |
| walker |  | 3530 | 140 | readme.md section #1 |  |  | 0.657 |
| walker |  | 3552 | 22 | ts names source/vendor/supports-color/browser.js |  |  | 0.657 |
| ns | 3571 |  | 304 | ansi-styles: the `styles.color` foreground code table | 3.2 | 3.1 | 0.632 |
| walker |  | 3586 | 34 | readme.md section #11 |  |  | 0.634 |
| walker |  | 3620 | 34 | readme.md section #13 |  |  | 0.636 |
| walker |  | 3855 | 235 | ts names source/vendor/ansi-styles/index.d.ts |  |  | 0.637 |
| ns | 3885 |  | 314 | ansi-styles: the `styles.bgColor` background code table | 3.3 | 3.2 | 0.613 |
| walker |  | 3926 | 71 | ts decl source/vendor/ansi-styles/index.d.ts:229 |  |  | 0.614 |
| ns | 3950 |  | 65 | ansi-styles: the four exported style-name arrays | 3.4 | 3.3 | 0.618 |
| walker |  | 4006 | 80 | ts decl source/vendor/ansi-styles/index.d.ts:1 |  |  | 0.618 |
| walker |  | 4022 | 16 | ts doc source/vendor/ansi-styles/index.d.ts:212 |  |  | 0.618 |
| walker |  | 4039 | 17 | ts doc source/vendor/ansi-styles/index.d.ts:217 |  |  | 0.618 |
| walker |  | 4140 | 101 | ts decl source/vendor/ansi-styles/index.d.ts:13 |  |  | 0.618 |
| ns | 4159 |  | 209 | readme: modifier list with human descriptions | 3.5 | 1.7 | 0.628 |
| walker |  | 4163 | 23 | ts decl source/vendor/supports-color/browser.js:29 |  |  | 0.628 |
| walker |  | 4200 | 37 | readme.md section #15 |  |  | 0.632 |
| walker |  | 4260 | 60 | ts body source/index.js:41 |  |  | 0.633 |
| walker |  | 4302 | 42 | readme.md section #8 |  |  | 0.636 |
| walker |  | 4319 | 17 | ts doc source/vendor/ansi-styles/index.d.ts:222 |  |  | 0.636 |
| walker |  | 4362 | 43 | ts doc source/index.d.ts:323 |  |  | 0.638 |
| ns | 4467 |  | 308 | readme: 256/Truecolor section and the `rgb`/`hex`/`ansi256` colour models | 3.6 | 1.5 | 0.626 |
| ns | 4561 |  | 94 | `source/index.js` imports and the stdout/stderr colour split | 4.1 |  | 0.619 |
| walker |  | 4627 | 265 | ts decl source/index.d.ts:32 |  |  | 0.623 |
| walker |  | 4666 | 39 | ts names source/vendor/supports-color/index.js |  |  | 0.623 |
| ns | 4843 |  | 282 | `source/index.js`: complete top-level declaration roster | 4.2 |  | 0.628 |
| walker |  | 4938 | 272 | ts decl source/vendor/ansi-styles/index.d.ts:75 |  |  | 0.628 |
| ns | 4982 |  | 139 | `Chalk` class, `chalkFactory`, `createChalk` | 4.3 | 4.2 | 0.628 |
| walker |  | 4992 | 54 | readme.md section #5 |  |  | 0.635 |
| ns | 5100 |  | 118 | `applyOptions`: level validation and auto-detection | 4.4 | 4.2 | 0.628 |
| ns | 5270 |  | 170 | Style-property generation loop, `visible`, and prototype installation | 4.5 | 4.2 | 0.616 |
| walker |  | 5286 | 294 | ts decl source/vendor/ansi-styles/index.d.ts:105 |  |  | 0.616 |
| walker |  | 5332 | 46 | readme.md section #36 |  |  | 0.616 |
| walker |  | 5379 | 47 | ts decl source/vendor/supports-color/index.js:185 |  |  | 0.617 |
| walker |  | 5435 | 56 | readme.md section #32 |  |  | 0.617 |
| ns | 5461 |  | 191 | `createBuilder`: the chainable callable | 4.6 | 4.2 | 0.606 |
| ns | 5607 |  | 146 | `createStyler`: the open/close linked list | 4.7 | 4.2 | 0.593 |
| walker |  | 5777 | 342 | ts decl source/index.d.ts:32 #1 |  |  | 0.605 |
| ns | 5907 |  | 300 | `applyStyle`: the string-wrapping algorithm | 4.8 | 4.2 | 0.585 |
| ns | 6000 |  | 93 | `proto` and the `level` getter/setter delegation | 4.9 | 4.2 | 0.593 |
| ns | 6056 |  | 56 | `levelMapping`: numeric level → ansi-styles method name | 4.10 | 4.2 | 0.592 |
| walker |  | 6179 | 402 | ts decl source/vendor/ansi-styles/index.d.ts:26 |  |  | 0.592 |
| walker |  | 6252 | 73 | readme.md section #9 |  |  | 0.601 |
| ns | 6255 |  | 199 | `getModelAnsi`: RGB/hex downsampling dispatch | 4.11 | 4.2 | 0.590 |
| walker |  | 6333 | 81 | readme.md section #10 |  |  | 0.605 |
| walker |  | 6448 | 115 | ts body source/index.js:24 |  |  | 0.614 |
| walker |  | 6515 | 67 | readme.md section #34 |  |  | 0.614 |
| ns | 6528 |  | 273 | Colour-model property generation for `rgb`/`hex`/`ansi256` and their `bg*` twins | 4.12 | 4.2 | 0.601 |
| walker |  | 6580 | 65 | readme.md section #35 |  |  | 0.601 |
| ns | 6593 |  | 65 | `source/utilities.js`: both exported helpers, signatures only | 4.13 |  | 0.601 |
| ns | 6816 |  | 223 | supports-color: complete declaration roster and default export | 5.1 |  | 0.595 |
| walker |  | 6837 | 257 | ts decl source/vendor/ansi-styles/index.d.ts:135 |  |  | 0.595 |
| walker |  | 6905 | 68 | readme.md section #33 |  |  | 0.595 |
| walker |  | 6959 | 54 | readme.md section #14 |  |  | 0.598 |
| ns | 6960 |  | 144 | supports-color: the `--color` / `--no-color` flag block | 5.2 | 5.1 | 0.590 |
| ns | 7095 |  | 135 | supports-color: `envForceColor` (`FORCE_COLOR` parsing) | 5.3 | 5.1 | 0.583 |
| ns | 7185 |  | 90 | supports-color: `translateLevel` — the `ColorInfo` shape | 5.4 | 5.1 | 0.577 |
| walker |  | 7269 | 310 | ts decl source/index.d.ts:32 #2 |  |  | 0.588 |
| walker |  | 7374 | 105 | readme.md section #16 |  |  | 0.592 |
| ns | 7409 |  | 224 | `_supportsColor`: force-colour precedence and explicit `--color=` levels | 5.5 | 5.1 | 0.580 |
| walker |  | 7482 | 108 | readme.md section #12 |  |  | 0.585 |
| walker |  | 7498 | 16 | ts doc source/vendor/ansi-styles/index.d.ts:186 |  |  | 0.585 |
| walker |  | 7641 | 143 | ts body source/index.js:132 |  |  | 0.605 |
| walker |  | 7756 | 115 | ts names source/vendor/supports-color/index.d.ts |  |  | 0.605 |
| walker |  | 7829 | 73 | ts doc source/index.d.ts:277 |  |  | 0.606 |
| ns | 7848 |  | 439 | `_supportsColor`: TTY, Windows and CI detection | 5.6 | 5.5 | 0.587 |
| walker |  | 7902 | 73 | ts doc source/index.d.ts:286 |  |  | 0.588 |
| walker |  | 7982 | 80 | ts doc source/index.d.ts:295 |  |  | 0.589 |
| walker |  | 8075 | 93 | ts doc source/index.d.ts:242 |  |  | 0.589 |
| walker |  | 8098 | 23 | ts decl source/vendor/supports-color/index.d.ts:50 |  |  | 0.590 |
| ns | 8249 |  | 401 | `_supportsColor`: TeamCity, `COLORTERM` and terminal-emulator sniffing | 5.7 | 5.6 | 0.577 |
| walker |  | 8286 | 188 | ts body source/index.js:152 |  |  | 0.591 |
| ns | 8309 |  | 60 | supports-color: `createSupportsColor` body | 5.8 | 5.1 | 0.588 |
| ns | 8474 |  | 165 | supports-color `index.d.ts`: complete type roster | 5.9 |  | 0.587 |
| walker |  | 8482 | 196 | ts body source/index.js:74 |  |  | 0.604 |
| walker |  | 8536 | 54 | ts doc source/vendor/ansi-styles/index.d.ts:193 |  |  | 0.604 |
| walker |  | 8590 | 54 | ts doc source/vendor/ansi-styles/index.d.ts:200 |  |  | 0.604 |
| walker |  | 8652 | 62 | ts doc source/vendor/ansi-styles/index.d.ts:207 |  |  | 0.604 |
| ns | 8683 |  | 209 | supports-color browser build: `browser.js` level detection and export shape | 5.10 |  | 0.596 |
| ns | 8696 |  | 13 | `browser.d.ts` (one-line type re-export) | 5.11 |  | 0.596 |
| walker |  | 8721 | 69 | ts decl source/vendor/supports-color/index.d.ts:3 |  |  | 0.603 |
| ns | 8768 |  | 72 | ava test harness setup in `test/chalk.js` | 6.1 |  | 0.601 |
| walker |  | 9128 | 407 | ts body source/index.js:168 |  |  | 0.628 |
| ns | 9189 |  | 421 | Every test title in `test/chalk.js` | 6.2 | 6.1 | 0.618 |
| walker |  | 9369 | 241 | ts decl source/vendor/ansi-styles/index.d.ts:135 #1 |  |  | 0.618 |
| ns | 9425 |  | 236 | Every test title in the four remaining ava files | 6.3 |  | 0.614 |
| ns | 9522 |  | 97 | `test/_fixture.js` in full, and the child process that runs it | 6.4 | 6.3 | 0.612 |
| walker |  | 9685 | 316 | ts decl source/index.d.ts:32 #3 |  |  | 0.628 |
| ns | 9753 |  | 231 | ansi-styles `index.d.ts`: complete declaration roster | 7.1 |  | 0.634 |
| ns | 9896 |  | 143 | `source/index.test-d.ts`: what the tsd suite asserts | 7.2 |  | 0.628 |
| ns | 9989 |  | 93 | `benchmark.js` and both `examples/` scripts: purpose lines | 7.3 |  | 0.625 |

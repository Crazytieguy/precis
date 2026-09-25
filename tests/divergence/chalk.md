Score(3000)=0.720 I=0.898 C=0.577 ns_rows≤3K=21/59 grid(1000/1442/2080/3000/4327/6240/9000)=0.678/0.738/0.759/0.720/0.663/0.596/0.544

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
| ns | 361 |  | 131 | readme tagline + every `##` section heading | 1.5 |  | 0.759 |
| walker |  | 465 | 260 | YAML config at .github/workflows/main.yml |  |  | 0.783 |
| ns | 568 |  | 207 | package.json `imports` subpath map, `types`, `engines`, `scripts` | 1.6 |  | 0.661 |
| walker |  | 570 | 105 | README headline in readme.md |  |  | 0.667 |
| walker |  | 583 | 13 | ts names source/vendor/supports-color/browser.d.ts |  |  | 0.667 |
| walker |  | 629 | 46 | ts names source/utilities.js |  |  | 0.667 |
| walker |  | 675 | 46 | package runtime metadata in package.json |  |  | 0.672 |
| ns | 708 |  | 140 | readme `###` subsection headings | 1.7 | 1.5 | 0.618 |
| walker |  | 749 | 74 | headings outline in code-of-conduct.md |  |  | 0.618 |
| walker |  | 749 | 0 | code-of-conduct.md section #0 |  |  | 0.618 |
| ns | 770 |  | 62 | Complete listing of `test/`, `examples/`, `.github/`, `media/` | 1.8 |  | 0.598 |
| walker |  | 776 | 27 | listing of 'test' |  |  | 0.678 |
| ns | 1030 |  | 260 | CI workflow in full | 1.9 |  | 0.724 |
| ns | 1244 |  | 214 | Runtime export surface of `source/index.js` | 2.1 |  | 0.654 |
| walker |  | 1258 | 482 | ts names source/index.js |  |  | 0.762 |
| walker |  | 1271 | 13 | ts decl source/index.js:34 |  |  | 0.762 |
| walker |  | 1280 | 9 | ts body source/index.js:50 |  |  | 0.762 |
| walker |  | 1318 | 38 | ts decl source/index.js:15 |  |  | 0.762 |
| walker |  | 1344 | 26 | ts body source/index.js:35 |  |  | 0.762 |
| ns | 1358 |  | 114 | `source/index.d.ts` top-level declaration roster | 2.2 |  | 0.738 |
| walker |  | 1366 | 22 | ts names source/vendor/supports-color/browser.js |  |  | 0.738 |
| ns | 1458 |  | 100 | readme: chaining semantics of `chalk.<style>[.<style>...]` | 2.3 | 1.7 | 0.718 |
| walker |  | 1459 | 93 | ts decl source/index.js:119 |  |  | 0.719 |
| walker |  | 1482 | 23 | ts decl source/vendor/supports-color/browser.js:29 |  |  | 0.719 |
| walker |  | 1521 | 39 | ts names source/vendor/supports-color/index.js |  |  | 0.720 |
| walker |  | 1568 | 47 | ts decl source/vendor/supports-color/index.js:185 |  |  | 0.720 |
| ns | 1686 |  | 228 | readme: `chalk.level` and the 0–3 colour-level table | 2.4 | 1.7 | 0.670 |
| ns | 1826 |  | 140 | `Options` interface body with level documentation | 2.5 | 2.2 | 0.642 |
| walker |  | 1828 | 260 | headings outline in readme.md |  |  | 0.747 |
| ns | 1970 |  | 144 | `ChalkInstance` call signature, `level`, and all six colour-model methods | 2.6 | 2.2 | 0.712 |
| walker |  | 2005 | 177 | package entrypoints in package.json |  |  | 0.759 |
| ns | 2094 |  | 124 | All ten `ChalkInstance` modifier properties | 2.7 | 2.6 | 0.720 |
| ns | 2293 |  | 199 | All foreground colour properties, including the `gray`/`grey` aliases | 2.8 | 2.7 | 0.683 |
| walker |  | 2389 | 384 | ts names source/index.d.ts |  |  | 0.712 |
| ns | 2517 |  | 224 | All background colour properties and the close of `ChalkInstance` | 2.9 | 2.8 | 0.677 |
| walker |  | 2529 | 140 | ts decl source/index.d.ts:12 |  |  | 0.713 |
| walker |  | 2547 | 18 | ts doc source/index.d.ts:30 |  |  | 0.713 |
| walker |  | 2581 | 34 | ts doc source/index.d.ts:302 |  |  | 0.713 |
| walker |  | 2617 | 36 | ts doc source/index.d.ts:268 |  |  | 0.713 |
| ns | 2767 |  | 250 | readme: `supportsColor`, the `--color`/`FORCE_COLOR` overrides, `chalkStderr` | 2.10 | 1.7 | 0.697 |
| ns | 2832 |  | 65 | readme: the exported style-name arrays and their use | 2.11 | 1.7 | 0.689 |
| walker |  | 2871 | 254 | package scripts in package.json |  |  | 0.711 |
| walker |  | 2907 | 36 | ts doc source/index.d.ts:309 |  |  | 0.711 |
| ns | 2962 |  | 130 | `index.d.ts` type re-export blocks from the vendored packages | 2.12 |  | 0.720 |
| walker |  | 3015 | 108 | ts names source/vendor/ansi-styles/index.js |  |  | 0.720 |
| ns | 3093 |  | 131 | Deprecated type/const aliases in `index.d.ts` | 2.13 | 2.2 | 0.716 |
| walker |  | 3130 | 115 | ts names source/vendor/supports-color/index.d.ts |  |  | 0.717 |
| walker |  | 3153 | 23 | ts decl source/vendor/supports-color/index.d.ts:50 |  |  | 0.717 |
| walker |  | 3222 | 69 | ts decl source/vendor/supports-color/index.d.ts:3 |  |  | 0.718 |
| ns | 3267 |  | 174 | ansi-styles: the `styles.modifier` code table | 3.1 |  | 0.701 |
| walker |  | 3451 | 229 | package identity metadata in package.json |  |  | 0.718 |
| walker |  | 3487 | 36 | ts doc source/index.d.ts:316 |  |  | 0.720 |
| ns | 3571 |  | 304 | ansi-styles: the `styles.color` foreground code table | 3.2 | 3.1 | 0.693 |
| walker |  | 3640 | 153 | ts decl source/vendor/supports-color/index.d.ts:24 |  |  | 0.693 |
| walker |  | 3659 | 19 | ts doc source/vendor/supports-color/index.d.ts:24 |  |  | 0.693 |
| ns | 3885 |  | 314 | ansi-styles: the `styles.bgColor` background code table | 3.3 | 3.2 | 0.668 |
| walker |  | 3894 | 235 | ts names source/vendor/ansi-styles/index.d.ts |  |  | 0.668 |
| ns | 3950 |  | 65 | ansi-styles: the four exported style-name arrays | 3.4 | 3.3 | 0.671 |
| walker |  | 3965 | 71 | ts decl source/vendor/ansi-styles/index.d.ts:229 |  |  | 0.672 |
| walker |  | 4045 | 80 | ts decl source/vendor/ansi-styles/index.d.ts:1 |  |  | 0.672 |
| walker |  | 4061 | 16 | ts doc source/vendor/ansi-styles/index.d.ts:212 |  |  | 0.672 |
| walker |  | 4078 | 17 | ts doc source/vendor/ansi-styles/index.d.ts:217 |  |  | 0.672 |
| ns | 4159 |  | 209 | readme: modifier list with human descriptions | 3.5 | 1.7 | 0.660 |
| walker |  | 4179 | 101 | ts decl source/vendor/ansi-styles/index.d.ts:13 |  |  | 0.660 |
| walker |  | 4239 | 60 | ts body source/index.js:41 |  |  | 0.661 |
| walker |  | 4256 | 17 | ts doc source/vendor/ansi-styles/index.d.ts:222 |  |  | 0.661 |
| walker |  | 4299 | 43 | ts doc source/index.d.ts:323 |  |  | 0.663 |
| ns | 4467 |  | 308 | readme: 256/Truecolor section and the `rgb`/`hex`/`ansi256` colour models | 3.6 | 1.5 | 0.650 |
| ns | 4561 |  | 94 | `source/index.js` imports and the stdout/stderr colour split | 4.1 |  | 0.643 |
| walker |  | 4564 | 265 | ts decl source/index.d.ts:32 |  |  | 0.647 |
| walker |  | 4836 | 272 | ts decl source/vendor/ansi-styles/index.d.ts:75 |  |  | 0.647 |
| ns | 4843 |  | 282 | `source/index.js`: complete top-level declaration roster | 4.2 |  | 0.651 |
| walker |  | 4928 | 92 | code-of-conduct.md section #6 |  |  | 0.651 |
| ns | 4982 |  | 139 | `Chalk` class, `chalkFactory`, `createChalk` | 4.3 | 4.2 | 0.651 |
| walker |  | 5082 | 154 | ts body source/utilities.js:21 |  |  | 0.651 |
| ns | 5100 |  | 118 | `applyOptions`: level validation and auto-detection | 4.4 | 4.2 | 0.645 |
| ns | 5270 |  | 170 | Style-property generation loop, `visible`, and prototype installation | 4.5 | 4.2 | 0.632 |
| walker |  | 5376 | 294 | ts decl source/vendor/ansi-styles/index.d.ts:105 |  |  | 0.632 |
| ns | 5461 |  | 191 | `createBuilder`: the chainable callable | 4.6 | 4.2 | 0.622 |
| walker |  | 5477 | 101 | code-of-conduct.md section #1 |  |  | 0.622 |
| ns | 5607 |  | 146 | `createStyler`: the open/close linked list | 4.7 | 4.2 | 0.609 |
| walker |  | 5646 | 169 | ts body source/utilities.js:2 |  |  | 0.609 |
| walker |  | 5816 | 170 | readme.md section #28 |  |  | 0.609 |
| walker |  | 5873 | 57 | ts body source/vendor/supports-color/index.js:176 |  |  | 0.609 |
| ns | 5907 |  | 300 | `applyStyle`: the string-wrapping algorithm | 4.8 | 4.2 | 0.590 |
| walker |  | 5984 | 111 | code-of-conduct.md section #4 |  |  | 0.590 |
| ns | 6000 |  | 93 | `proto` and the `level` getter/setter delegation | 4.9 | 4.2 | 0.598 |
| ns | 6056 |  | 56 | `levelMapping`: numeric level → ansi-styles method name | 4.10 | 4.2 | 0.596 |
| walker |  | 6124 | 140 | readme.md section #1 |  |  | 0.596 |
| ns | 6255 |  | 199 | `getModelAnsi`: RGB/hex downsampling dispatch | 4.11 | 4.2 | 0.585 |
| walker |  | 6466 | 342 | ts decl source/index.d.ts:32 #1 |  |  | 0.596 |
| ns | 6528 |  | 273 | Colour-model property generation for `rgb`/`hex`/`ansi256` and their `bg*` twins | 4.12 | 4.2 | 0.583 |
| ns | 6593 |  | 65 | `source/utilities.js`: both exported helpers, signatures only | 4.13 |  | 0.583 |
| walker |  | 6616 | 150 | readme.md section #27 |  |  | 0.583 |
| ns | 6816 |  | 223 | supports-color: complete declaration roster and default export | 5.1 |  | 0.578 |
| ns | 6960 |  | 144 | supports-color: the `--color` / `--no-color` flag block | 5.2 | 5.1 | 0.569 |
| walker |  | 7018 | 402 | ts decl source/vendor/ansi-styles/index.d.ts:26 |  |  | 0.569 |
| ns | 7095 |  | 135 | supports-color: `envForceColor` (`FORCE_COLOR` parsing) | 5.3 | 5.1 | 0.563 |
| walker |  | 7109 | 91 | readme.md section #2 |  |  | 0.563 |
| ns | 7185 |  | 90 | supports-color: `translateLevel` — the `ColorInfo` shape | 5.4 | 5.1 | 0.558 |
| ns | 7409 |  | 224 | `_supportsColor`: force-colour precedence and explicit `--color=` levels | 5.5 | 5.1 | 0.547 |
| walker |  | 7532 | 423 | readme.md section #29 |  |  | 0.563 |
| walker |  | 7647 | 115 | ts body source/index.js:24 |  |  | 0.572 |
| walker |  | 7790 | 143 | code-of-conduct.md section #3 |  |  | 0.572 |
| ns | 7848 |  | 439 | `_supportsColor`: TTY, Windows and CI detection | 5.6 | 5.5 | 0.554 |
| walker |  | 8047 | 257 | ts decl source/vendor/ansi-styles/index.d.ts:135 |  |  | 0.554 |
| ns | 8249 |  | 401 | `_supportsColor`: TeamCity, `COLORTERM` and terminal-emulator sniffing | 5.7 | 5.6 | 0.542 |
| ns | 8309 |  | 60 | supports-color: `createSupportsColor` body | 5.8 | 5.1 | 0.545 |
| ns | 8474 |  | 165 | supports-color `index.d.ts`: complete type roster | 5.9 |  | 0.554 |
| ns | 8683 |  | 209 | supports-color browser build: `browser.js` level detection and export shape | 5.10 |  | 0.546 |
| ns | 8696 |  | 13 | `browser.d.ts` (one-line type re-export) | 5.11 |  | 0.547 |
| walker |  | 8734 | 687 | readme.md section #3 |  |  | 0.547 |
| ns | 8768 |  | 72 | ava test harness setup in `test/chalk.js` | 6.1 |  | 0.544 |
| ns | 9189 |  | 421 | Every test title in `test/chalk.js` | 6.2 | 6.1 | 0.536 |
| walker |  | 9214 | 480 | readme.md section #39 |  |  | 0.536 |
| walker |  | 9377 | 163 | code-of-conduct.md section #5 |  |  | 0.536 |
| walker |  | 9414 | 37 | readme.md section #31 |  |  | 0.536 |
| ns | 9425 |  | 236 | Every test title in the four remaining ava files | 6.3 |  | 0.532 |
| ns | 9522 |  | 97 | `test/_fixture.js` in full, and the child process that runs it | 6.4 | 6.3 | 0.530 |
| walker |  | 9724 | 310 | ts decl source/index.d.ts:32 #2 |  |  | 0.539 |
| ns | 9753 |  | 231 | ansi-styles `index.d.ts`: complete declaration roster | 7.1 |  | 0.548 |
| walker |  | 9798 | 74 | ts doc source/vendor/supports-color/index.d.ts:19 |  |  | 0.548 |
| walker |  | 9822 | 24 | readme.md section #30 |  |  | 0.548 |
| walker |  | 9838 | 16 | ts doc source/vendor/ansi-styles/index.d.ts:186 |  |  | 0.548 |
| walker |  | 9880 | 42 | readme.md section #40 |  |  | 0.548 |
| ns | 9896 |  | 143 | `source/index.test-d.ts`: what the tsd suite asserts | 7.2 |  | 0.543 |
| ns | 9989 |  | 93 | `benchmark.js` and both `examples/` scripts: purpose lines | 7.3 |  | 0.541 |

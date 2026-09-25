Score(3000)=0.666 I=0.887 C=0.500 ns_rows≤3K=21/59 grid(1000/1442/2080/3000/4327/6240/9000)=0.674/0.750/0.721/0.666/0.605/0.558/0.521

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
| walker |  | 430 | 74 | headings outline in code-of-conduct.md |  |  | 0.767 |
| walker |  | 430 | 0 | code-of-conduct.md section #0 |  |  | 0.767 |
| walker |  | 457 | 27 | listing of 'test' |  |  | 0.778 |
| ns | 568 |  | 207 | package.json `imports` subpath map, `types`, `engines`, `scripts` | 1.6 |  | 0.662 |
| ns | 708 |  | 140 | readme `###` subsection headings | 1.7 | 1.5 | 0.608 |
| ns | 770 |  | 62 | Complete listing of `test/`, `examples/`, `.github/`, `media/` | 1.8 |  | 0.658 |
| walker |  | 939 | 482 | ts names source/index.js |  |  | 0.674 |
| walker |  | 952 | 13 | ts decl source/index.js:34 |  |  | 0.674 |
| walker |  | 961 | 9 | ts body source/index.js:50 |  |  | 0.674 |
| walker |  | 999 | 38 | ts decl source/index.js:15 |  |  | 0.674 |
| ns | 1030 |  | 260 | CI workflow in full | 1.9 |  | 0.580 |
| walker |  | 1048 | 49 | package scripts in package.json |  |  | 0.594 |
| walker |  | 1074 | 26 | ts body source/index.js:35 |  |  | 0.594 |
| walker |  | 1167 | 93 | ts decl source/index.js:119 |  |  | 0.596 |
| ns | 1244 |  | 214 | Runtime export surface of `source/index.js` | 2.1 |  | 0.646 |
| ns | 1358 |  | 114 | `source/index.d.ts` top-level declaration roster | 2.2 |  | 0.625 |
| walker |  | 1427 | 260 | headings outline in readme.md |  |  | 0.750 |
| ns | 1458 |  | 100 | readme: chaining semantics of `chalk.<style>[.<style>...]` | 2.3 | 1.7 | 0.731 |
| walker |  | 1602 | 175 | package entrypoints in package.json |  |  | 0.809 |
| ns | 1686 |  | 228 | readme: `chalk.level` and the 0–3 colour-level table | 2.4 | 1.7 | 0.754 |
| ns | 1826 |  | 140 | `Options` interface body with level documentation | 2.5 | 2.2 | 0.722 |
| ns | 1970 |  | 144 | `ChalkInstance` call signature, `level`, and all six colour-model methods | 2.6 | 2.2 | 0.688 |
| walker |  | 1986 | 384 | ts names source/index.d.ts |  |  | 0.721 |
| ns | 2094 |  | 124 | All ten `ChalkInstance` modifier properties | 2.7 | 2.6 | 0.685 |
| walker |  | 2126 | 140 | ts decl source/index.d.ts:12 |  |  | 0.726 |
| walker |  | 2144 | 18 | ts doc source/index.d.ts:30 |  |  | 0.726 |
| walker |  | 2178 | 34 | ts doc source/index.d.ts:302 |  |  | 0.727 |
| walker |  | 2214 | 36 | ts doc source/index.d.ts:268 |  |  | 0.727 |
| walker |  | 2227 | 13 | ts names source/vendor/supports-color/browser.d.ts |  |  | 0.727 |
| walker |  | 2263 | 36 | ts doc source/index.d.ts:309 |  |  | 0.727 |
| ns | 2293 |  | 199 | All foreground colour properties, including the `gray`/`grey` aliases | 2.8 | 2.7 | 0.690 |
| walker |  | 2309 | 46 | ts names source/utilities.js |  |  | 0.690 |
| walker |  | 2417 | 108 | ts names source/vendor/ansi-styles/index.js |  |  | 0.691 |
| ns | 2517 |  | 224 | All background colour properties and the close of `ChalkInstance` | 2.9 | 2.8 | 0.657 |
| walker |  | 2646 | 229 | package identity metadata in package.json |  |  | 0.676 |
| walker |  | 2682 | 36 | ts doc source/index.d.ts:316 |  |  | 0.676 |
| walker |  | 2704 | 22 | ts names source/vendor/supports-color/browser.js |  |  | 0.676 |
| ns | 2767 |  | 250 | readme: `supportsColor`, the `--color`/`FORCE_COLOR` overrides, `chalkStderr` | 2.10 | 1.7 | 0.661 |
| ns | 2832 |  | 65 | readme: the exported style-name arrays and their use | 2.11 | 1.7 | 0.654 |
| walker |  | 2939 | 235 | ts names source/vendor/ansi-styles/index.d.ts |  |  | 0.654 |
| ns | 2962 |  | 130 | `index.d.ts` type re-export blocks from the vendored packages | 2.12 |  | 0.666 |
| walker |  | 3010 | 71 | ts decl source/vendor/ansi-styles/index.d.ts:229 |  |  | 0.667 |
| walker |  | 3090 | 80 | ts decl source/vendor/ansi-styles/index.d.ts:1 |  |  | 0.667 |
| ns | 3093 |  | 131 | Deprecated type/const aliases in `index.d.ts` | 2.13 | 2.2 | 0.668 |
| walker |  | 3106 | 16 | ts doc source/vendor/ansi-styles/index.d.ts:212 |  |  | 0.668 |
| walker |  | 3123 | 17 | ts doc source/vendor/ansi-styles/index.d.ts:217 |  |  | 0.668 |
| walker |  | 3224 | 101 | ts decl source/vendor/ansi-styles/index.d.ts:13 |  |  | 0.668 |
| walker |  | 3247 | 23 | ts decl source/vendor/supports-color/browser.js:29 |  |  | 0.668 |
| ns | 3267 |  | 174 | ansi-styles: the `styles.modifier` code table | 3.1 |  | 0.652 |
| walker |  | 3307 | 60 | ts body source/index.js:41 |  |  | 0.653 |
| walker |  | 3324 | 17 | ts doc source/vendor/ansi-styles/index.d.ts:222 |  |  | 0.653 |
| walker |  | 3367 | 43 | ts doc source/index.d.ts:323 |  |  | 0.655 |
| ns | 3571 |  | 304 | ansi-styles: the `styles.color` foreground code table | 3.2 | 3.1 | 0.631 |
| walker |  | 3632 | 265 | ts decl source/index.d.ts:32 |  |  | 0.635 |
| walker |  | 3671 | 39 | ts names source/vendor/supports-color/index.js |  |  | 0.635 |
| ns | 3885 |  | 314 | ansi-styles: the `styles.bgColor` background code table | 3.3 | 3.2 | 0.612 |
| walker |  | 3943 | 272 | ts decl source/vendor/ansi-styles/index.d.ts:75 |  |  | 0.612 |
| ns | 3950 |  | 65 | ansi-styles: the four exported style-name arrays | 3.4 | 3.3 | 0.616 |
| walker |  | 4035 | 92 | code-of-conduct.md section #6 |  |  | 0.616 |
| ns | 4159 |  | 209 | readme: modifier list with human descriptions | 3.5 | 1.7 | 0.605 |
| walker |  | 4329 | 294 | ts decl source/vendor/ansi-styles/index.d.ts:105 |  |  | 0.605 |
| walker |  | 4430 | 101 | code-of-conduct.md section #1 |  |  | 0.605 |
| ns | 4467 |  | 308 | readme: 256/Truecolor section and the `rgb`/`hex`/`ansi256` colour models | 3.6 | 1.5 | 0.593 |
| walker |  | 4477 | 47 | ts decl source/vendor/supports-color/index.js:185 |  |  | 0.593 |
| ns | 4561 |  | 94 | `source/index.js` imports and the stdout/stderr colour split | 4.1 |  | 0.587 |
| walker |  | 4647 | 170 | readme.md section #28 |  |  | 0.587 |
| walker |  | 4758 | 111 | code-of-conduct.md section #4 |  |  | 0.587 |
| ns | 4843 |  | 282 | `source/index.js`: complete top-level declaration roster | 4.2 |  | 0.594 |
| walker |  | 4898 | 140 | readme.md section #1 |  |  | 0.594 |
| ns | 4982 |  | 139 | `Chalk` class, `chalkFactory`, `createChalk` | 4.3 | 4.2 | 0.596 |
| ns | 5100 |  | 118 | `applyOptions`: level validation and auto-detection | 4.4 | 4.2 | 0.590 |
| walker |  | 5240 | 342 | ts decl source/index.d.ts:32 #1 |  |  | 0.602 |
| ns | 5270 |  | 170 | Style-property generation loop, `visible`, and prototype installation | 4.5 | 4.2 | 0.591 |
| walker |  | 5390 | 150 | readme.md section #27 |  |  | 0.591 |
| ns | 5461 |  | 191 | `createBuilder`: the chainable callable | 4.6 | 4.2 | 0.581 |
| ns | 5607 |  | 146 | `createStyler`: the open/close linked list | 4.7 | 4.2 | 0.569 |
| walker |  | 5792 | 402 | ts decl source/vendor/ansi-styles/index.d.ts:26 |  |  | 0.569 |
| walker |  | 5883 | 91 | readme.md section #2 |  |  | 0.569 |
| ns | 5907 |  | 300 | `applyStyle`: the string-wrapping algorithm | 4.8 | 4.2 | 0.550 |
| ns | 6000 |  | 93 | `proto` and the `level` getter/setter delegation | 4.9 | 4.2 | 0.559 |
| ns | 6056 |  | 56 | `levelMapping`: numeric level → ansi-styles method name | 4.10 | 4.2 | 0.558 |
| ns | 6255 |  | 199 | `getModelAnsi`: RGB/hex downsampling dispatch | 4.11 | 4.2 | 0.548 |
| walker |  | 6306 | 423 | readme.md section #29 |  |  | 0.566 |
| walker |  | 6421 | 115 | ts body source/index.js:24 |  |  | 0.576 |
| ns | 6528 |  | 273 | Colour-model property generation for `rgb`/`hex`/`ansi256` and their `bg*` twins | 4.12 | 4.2 | 0.564 |
| walker |  | 6564 | 143 | code-of-conduct.md section #3 |  |  | 0.564 |
| ns | 6593 |  | 65 | `source/utilities.js`: both exported helpers, signatures only | 4.13 |  | 0.564 |
| ns | 6816 |  | 223 | supports-color: complete declaration roster and default export | 5.1 |  | 0.559 |
| walker |  | 6821 | 257 | ts decl source/vendor/ansi-styles/index.d.ts:135 |  |  | 0.559 |
| ns | 6960 |  | 144 | supports-color: the `--color` / `--no-color` flag block | 5.2 | 5.1 | 0.551 |
| ns | 7095 |  | 135 | supports-color: `envForceColor` (`FORCE_COLOR` parsing) | 5.3 | 5.1 | 0.545 |
| ns | 7185 |  | 90 | supports-color: `translateLevel` — the `ColorInfo` shape | 5.4 | 5.1 | 0.540 |
| ns | 7409 |  | 224 | `_supportsColor`: force-colour precedence and explicit `--color=` levels | 5.5 | 5.1 | 0.529 |
| walker |  | 7508 | 687 | readme.md section #3 |  |  | 0.529 |
| ns | 7848 |  | 439 | `_supportsColor`: TTY, Windows and CI detection | 5.6 | 5.5 | 0.513 |
| walker |  | 7988 | 480 | readme.md section #39 |  |  | 0.513 |
| walker |  | 8151 | 163 | code-of-conduct.md section #5 |  |  | 0.513 |
| walker |  | 8188 | 37 | readme.md section #31 |  |  | 0.513 |
| ns | 8249 |  | 401 | `_supportsColor`: TeamCity, `COLORTERM` and terminal-emulator sniffing | 5.7 | 5.6 | 0.502 |
| ns | 8309 |  | 60 | supports-color: `createSupportsColor` body | 5.8 | 5.1 | 0.499 |
| ns | 8474 |  | 165 | supports-color `index.d.ts`: complete type roster | 5.9 |  | 0.493 |
| walker |  | 8498 | 310 | ts decl source/index.d.ts:32 #2 |  |  | 0.504 |
| walker |  | 8522 | 24 | readme.md section #30 |  |  | 0.504 |
| walker |  | 8538 | 16 | ts doc source/vendor/ansi-styles/index.d.ts:186 |  |  | 0.504 |
| walker |  | 8580 | 42 | readme.md section #40 |  |  | 0.504 |
| ns | 8683 |  | 209 | supports-color browser build: `browser.js` level detection and export shape | 5.10 |  | 0.497 |
| ns | 8696 |  | 13 | `browser.d.ts` (one-line type re-export) | 5.11 |  | 0.498 |
| walker |  | 8723 | 143 | ts body source/index.js:132 |  |  | 0.516 |
| ns | 8768 |  | 72 | ava test harness setup in `test/chalk.js` | 6.1 |  | 0.514 |
| walker |  | 8838 | 115 | ts names source/vendor/supports-color/index.d.ts |  |  | 0.519 |
| walker |  | 8911 | 73 | ts doc source/index.d.ts:277 |  |  | 0.520 |
| walker |  | 8984 | 73 | ts doc source/index.d.ts:286 |  |  | 0.521 |
| walker |  | 9064 | 80 | ts doc source/index.d.ts:295 |  |  | 0.522 |
| walker |  | 9157 | 93 | ts doc source/index.d.ts:242 |  |  | 0.522 |
| walker |  | 9180 | 23 | ts decl source/vendor/supports-color/index.d.ts:50 |  |  | 0.523 |
| ns | 9189 |  | 421 | Every test title in `test/chalk.js` | 6.2 | 6.1 | 0.515 |
| walker |  | 9368 | 188 | ts body source/index.js:152 |  |  | 0.529 |
| ns | 9425 |  | 236 | Every test title in the four remaining ava files | 6.3 |  | 0.525 |
| ns | 9522 |  | 97 | `test/_fixture.js` in full, and the child process that runs it | 6.4 | 6.3 | 0.523 |
| walker |  | 9564 | 196 | ts body source/index.js:74 |  |  | 0.540 |
| walker |  | 9618 | 54 | ts doc source/vendor/ansi-styles/index.d.ts:193 |  |  | 0.540 |
| walker |  | 9672 | 54 | ts doc source/vendor/ansi-styles/index.d.ts:200 |  |  | 0.540 |
| walker |  | 9734 | 62 | ts doc source/vendor/ansi-styles/index.d.ts:207 |  |  | 0.540 |
| ns | 9753 |  | 231 | ansi-styles `index.d.ts`: complete declaration roster | 7.1 |  | 0.548 |
| walker |  | 9803 | 69 | ts decl source/vendor/supports-color/index.d.ts:3 |  |  | 0.555 |
| ns | 9896 |  | 143 | `source/index.test-d.ts`: what the tsd suite asserts | 7.2 |  | 0.551 |
| ns | 9989 |  | 93 | `benchmark.js` and both `examples/` scripts: purpose lines | 7.3 |  | 0.548 |

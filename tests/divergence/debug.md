Score(3000)=0.492 I=0.749 C=0.323 ns_rows≤3K=21/54 grid(1000/1442/2080/3000/4327/6240/9000)=0.714/0.614/0.576/0.492/0.569/0.532/0.566

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 31 | 31 | listing of '.' |  |  | 0.000 |
| ns | 45 |  | 45 | Repo identity: README title + one-sentence description | 1.1 |  | 0.000 |
| walker |  | 47 | 16 | listing of 'src' |  |  | 0.000 |
| walker |  | 90 | 43 | ts module doc src/index.js |  |  | 0.000 |
| ns | 92 |  | 47 | Complete file map: repo root and src/ | 1.2 |  | 0.633 |
| walker |  | 135 | 45 | README headline in README.md |  |  | 1.000 |
| walker |  | 200 | 65 | package identity in package.json |  |  | 1.000 |
| ns | 205 |  | 113 | src/index.js in full — the environment dispatch | 1.3 |  | 0.791 |
| walker |  | 232 | 32 | package runtime dependencies in package.json |  |  | 0.791 |
| walker |  | 254 | 22 | ts names src/common.js |  |  | 0.791 |
| walker |  | 286 | 32 | package runtime metadata in package.json |  |  | 0.800 |
| walker |  | 300 | 14 | ts names karma.conf.js |  |  | 0.800 |
| ns | 328 |  | 123 | package.json identity, entry points, engines, license | 1.4 |  | 0.759 |
| walker |  | 368 | 68 | package entrypoints in package.json |  |  | 0.830 |
| ns | 436 |  | 108 | package.json `scripts` — every build/test/lint entry point | 1.5 |  | 0.744 |
| walker |  | 474 | 106 | package scripts in package.json |  |  | 0.853 |
| ns | 616 |  | 180 | README section map — every `##` heading | 1.6 |  | 0.689 |
| walker |  | 637 | 163 | ts names src/browser.js |  |  | 0.695 |
| walker |  | 735 | 98 | ts decl src/browser.js:12 |  |  | 0.696 |
| ns | 765 |  | 149 | src/common.js — `setup(env)` and the complete `createDebug.*` public API attachment block | 2.1 |  | 0.610 |
| ns | 906 |  | 141 | src/common.js — module-level state: `names`, `skips`, `formatters` | 2.2 |  | 0.555 |
| walker |  | 926 | 191 | headings outline in README.md |  |  | 0.714 |
| walker |  | 951 | 25 | README.md section #1 |  |  | 0.714 |
| ns | 1061 |  | 155 | src/common.js — roster of every function definition plus the module tail | 2.3 |  | 0.659 |
| walker |  | 1154 | 203 | ts names src/node.js |  |  | 0.664 |
| walker |  | 1179 | 25 | ts decl src/node.js:124 |  |  | 0.664 |
| walker |  | 1226 | 47 | ts decl src/node.js:18 |  |  | 0.665 |
| ns | 1229 |  | 168 | src/node.js — the node adapter's export contract | 2.4 |  | 0.627 |
| walker |  | 1235 | 9 | ts body src/node.js:220 |  |  | 0.627 |
| walker |  | 1250 | 15 | ts doc src/node.js:27 |  |  | 0.627 |
| walker |  | 1273 | 23 | ts body src/node.js:193 |  |  | 0.627 |
| walker |  | 1302 | 29 | ts doc src/node.js:155 |  |  | 0.627 |
| walker |  | 1334 | 32 | ts doc src/node.js:193 |  |  | 0.627 |
| ns | 1343 |  | 114 | src/browser.js — the browser adapter's export contract | 2.5 |  | 0.614 |
| walker |  | 1370 | 36 | ts doc src/browser.js:149 |  |  | 0.614 |
| walker |  | 1407 | 37 | ts doc src/node.js:167 |  |  | 0.614 |
| walker |  | 1450 | 43 | README.md section #17 |  |  | 0.614 |
| walker |  | 1488 | 38 | ts doc src/common.js:7 |  |  | 0.623 |
| ns | 1517 |  | 174 | src/node.js — roster of every function, export and formatter | 2.6 |  | 0.618 |
| walker |  | 1527 | 39 | ts body src/node.js:155 |  |  | 0.618 |
| walker |  | 1571 | 44 | ts doc src/browser.js:200 |  |  | 0.618 |
| walker |  | 1617 | 46 | ts doc src/node.js:203 |  |  | 0.618 |
| ns | 1644 |  | 127 | src/browser.js — roster of every function, export and formatter | 2.7 |  | 0.621 |
| walker |  | 1668 | 51 | ts doc src/browser.js:219 |  |  | 0.621 |
| walker |  | 1719 | 51 | ts doc src/node.js:220 |  |  | 0.621 |
| ns | 1802 |  | 158 | README: the complete `DEBUG_*` environment variable table | 3.1 |  | 0.599 |
| walker |  | 1898 | 179 | package identity metadata in package.json |  |  | 0.599 |
| walker |  | 1957 | 59 | ts doc src/node.js:124 |  |  | 0.599 |
| ns | 1988 |  | 186 | README: the complete `%` formatter table | 3.2 |  | 0.576 |
| walker |  | 2021 | 64 | ts doc src/node.js:231 |  |  | 0.576 |
| walker |  | 2094 | 73 | ts body src/node.js:231 |  |  | 0.576 |
| ns | 2155 |  | 167 | README: wildcard and exclusion syntax for `DEBUG` | 3.3 |  | 0.558 |
| walker |  | 2177 | 83 | ts doc src/browser.js:192 |  |  | 0.558 |
| ns | 2217 |  | 62 | README: every sub-`##` heading (`###`/`####`/`#####`) | 3.4 |  | 0.549 |
| walker |  | 2268 | 91 | ts doc src/browser.js:247 |  |  | 0.549 |
| walker |  | 2357 | 89 | ts body src/node.js:203 |  |  | 0.550 |
| ns | 2438 |  | 221 | README: the canonical usage example | 3.5 |  | 0.522 |
| walker |  | 2484 | 127 | README.md section #7 |  |  | 0.522 |
| walker |  | 2590 | 106 | ts body src/browser.js:200 |  |  | 0.523 |
| ns | 2645 |  | 207 | README: namespace colors — when they turn on, per environment | 3.6 |  | 0.509 |
| walker |  | 2699 | 109 | ts body src/browser.js:247 |  |  | 0.509 |
| ns | 2855 |  | 210 | README: namespace naming conventions + the `DEBUG_*` → `util.inspect` options note | 3.7 |  | 0.504 |
| ns | 2949 |  | 94 | README: checking and forcing `debug.enabled` | 3.8 |  | 0.492 |
| walker |  | 2955 | 256 | ts decl src/browser.js:27 |  |  | 0.492 |
| walker |  | 2970 | 15 | ts doc src/browser.js:27 |  |  | 0.492 |
| ns | 3075 |  | 126 | README: `log.extend()` for sub-namespaces | 3.9 |  | 0.478 |
| walker |  | 3096 | 126 | README.md section #13 |  |  | 0.516 |
| ns | 3200 |  | 125 | README: enabling debug dynamically via `enable()` / `disable()` | 3.10 |  | 0.500 |
| walker |  | 3357 | 261 | README.md section #9 |  |  | 0.546 |
| ns | 3380 |  | 180 | README: `enable(namespaces)` / `disable()` contract and the round-trip caveat | 3.11 |  | 0.529 |
| walker |  | 3486 | 129 | README.md section #15 |  |  | 0.556 |
| ns | 3570 |  | 190 | README: adding a custom formatter | 3.12 |  | 0.540 |
| walker |  | 3653 | 167 | README.md section #8 |  |  | 0.566 |
| ns | 3742 |  | 172 | README: redirecting output by overriding `log` | 3.13 |  | 0.550 |
| walker |  | 3814 | 161 | ts body src/browser.js:219 |  |  | 0.551 |
| ns | 3973 |  | 231 | README: browser build, `localStorage.debug`, and the Chromium Verbose caveat | 3.14 |  | 0.539 |
| ns | 4066 |  | 93 | README: setting `DEBUG` on Windows (CMD and PowerShell) | 3.15 |  | 0.529 |
| walker |  | 4085 | 271 | README.md section #2 |  |  | 0.571 |
| ns | 4166 |  | 100 | README: the millisecond diff feature | 3.16 |  | 0.569 |
| walker |  | 4313 | 228 | README.md section #3 |  |  | 0.569 |
| ns | 4361 |  | 195 | README: colors in child processes (`DEBUG_COLORS=1`) | 3.17 |  | 0.555 |
| ns | 4476 |  | 115 | package.json: runtime dependency, optional peer dependency, and the xo lint override | 4.1 |  | 0.545 |
| walker |  | 4530 | 217 | README.md section #6 |  |  | 0.549 |
| walker |  | 4789 | 259 | ts decl src/browser.js:27 #1 |  |  | 0.549 |
| ns | 4792 |  | 316 | test.js: header plus every `describe`/`it` title | 4.2 |  | 0.533 |
| ns | 4970 |  | 178 | test.node.js: the node-only suite and its sinon harness | 4.3 |  | 0.519 |
| walker |  | 4991 | 202 | ts body src/node.js:167 |  |  | 0.520 |
| ns | 5114 |  | 144 | karma.conf.js: frameworks and the browser test file set | 4.4 |  | 0.512 |
| walker |  | 5228 | 237 | ts body src/node.js:124 |  |  | 0.514 |
| ns | 5246 |  | 132 | .travis.yml in full — the CI matrix | 4.5 |  | 0.502 |
| ns | 5533 |  | 287 | karma.conf.js: launcher, preprocessors and run mode | 4.6 | 4.4 | 0.485 |
| walker |  | 5617 | 389 | README.md section #10 |  |  | 0.534 |
| ns | 5732 |  | 199 | src/common.js: `enable()` body — parsing the namespace string | 5.1 | 2.3 | 0.521 |
| walker |  | 5846 | 229 | README.md section #16 |  |  | 0.545 |
| ns | 5950 |  | 218 | src/common.js: `debug(...args)` — enabled guard, ms-diff bookkeeping, `%O` coercion | 5.2 | 2.3 | 0.531 |
| walker |  | 6092 | 246 | ts decl src/browser.js:27 #2 |  |  | 0.532 |
| ns | 6264 |  | 314 | src/common.js: `debug(...args)` — the `%`-formatter substitution loop and log dispatch | 5.3 | 5.2 | 0.516 |
| walker |  | 6462 | 370 | README.md section #5 |  |  | 0.534 |
| ns | 6598 |  | 334 | src/common.js: per-instance properties and the `enabled` getter/setter | 5.4 | 2.3 | 0.516 |
| walker |  | 6728 | 266 | README.md section #12 |  |  | 0.537 |
| ns | 6990 |  | 392 | src/common.js: `matchesTemplate()` — the wildcard matcher | 5.5 | 2.3 | 0.520 |
| walker |  | 7118 | 390 | ts body src/browser.js:149 |  |  | 0.522 |
| ns | 7187 |  | 197 | src/common.js: `disable()` and `enabled()` bodies | 5.6 | 2.3 | 0.511 |
| ns | 7436 |  | 249 | src/common.js: `selectColor()` and `extend()` bodies | 5.7 | 2.3 | 0.503 |
| ns | 7571 |  | 135 | src/common.js: `coerce()` and the deprecated `destroy()` stub | 5.8 | 2.3 | 0.497 |
| walker |  | 7599 | 481 | ts body src/browser.js:115 |  |  | 0.498 |
| ns | 7647 |  | 76 | src/common.js: adapter-property spread and `createDebug`'s closure state | 5.9 | 2.1 | 0.494 |
| ns | 7942 |  | 295 | src/node.js: `inspectOpts` derivation from `DEBUG_*` environment variables | 6.1 | 2.6 | 0.510 |
| walker |  | 7949 | 350 | README.md section #20 |  |  | 0.510 |
| ns | 8206 |  | 264 | src/browser.js: `useColors()` — the inspector capability sniff | 6.2 | 2.7 | 0.517 |
| walker |  | 8351 | 402 | README.md section #11 |  |  | 0.533 |
| ns | 8528 |  | 322 | src/node.js: `useColors`, `formatArgs`, `getDate`, `log` bodies | 6.3 | 2.6 | 0.536 |
| walker |  | 8861 | 510 | README.md section #4 |  |  | 0.566 |
| ns | 8861 |  | 333 | src/browser.js: `formatArgs()` — `%c` CSS injection | 6.4 | 2.7 | 0.566 |
| ns | 9045 |  | 184 | src/node.js: `save`, `load`, `init` bodies | 6.5 | 2.6 | 0.571 |
| walker |  | 9338 | 477 | README.md section #14 |  |  | 0.601 |
| ns | 9397 |  | 352 | src/browser.js: `save`, `load`, `localstorage` bodies | 6.6 | 2.7 | 0.610 |
| ns | 9541 |  | 144 | Formatter implementations: node `%o`/`%O` and browser `%j` | 6.7 | 2.6 | 0.604 |
| ns | 9722 |  | 181 | Both color palettes, head and tail, with the elision marked | 6.8 | 2.7 | 0.597 |
| ns | 9932 |  | 210 | LICENSE header, plus .gitignore in full and the .editorconfig head | 7.1 |  | 0.587 |
| walker |  | 9975 | 637 | ts body karma.conf.js:1 |  |  | 0.620 |

Score(3000)=0.625 I=0.828 C=0.472 ns_rows≤3K=21/54 (reached=11 partial=0 missing=10) grid(1000/1442/2080/3000/4327/6240/9000)=0.790/0.615/0.573/0.625/0.658/0.705/0.720

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 32 | 32 | listing of '.' |  |  | 0.000 |
| ns | 45 |  | 45 | Repo identity: README title + one-sentence description | 1.1 |  | 0.000 |
| walker |  | 47 | 15 | listing of 'src' |  |  | 0.000 |
| walker |  | 92 | 45 | README headline in README.md |  |  | 1.000 |
| ns | 93 |  | 48 | Complete file map: repo root and src/ | 1.2 |  | 1.000 |
| walker |  | 157 | 65 | package identity in package.json |  |  | 1.000 |
| walker |  | 200 | 43 | module-doc lede in src/index.js |  |  | 1.000 |
| ns | 206 |  | 113 | src/index.js in full — the environment dispatch | 1.3 |  | 0.791 |
| walker |  | 232 | 32 | package runtime dependencies in package.json |  |  | 0.791 |
| walker |  | 264 | 32 | package runtime metadata in package.json |  |  | 0.800 |
| ns | 329 |  | 123 | package.json identity, entry points, engines, license | 1.4 |  | 0.758 |
| walker |  | 332 | 68 | package entrypoints in package.json |  |  | 0.830 |
| walker |  | 346 | 14 | export names surface in karma.conf.js |  |  | 0.830 |
| walker |  | 346 | 0 | export at karma.conf.js:1 |  |  | 0.830 |
| ns | 437 |  | 108 | package.json `scripts` — every build/test/lint entry point | 1.5 |  | 0.744 |
| walker |  | 452 | 106 | package scripts in package.json |  |  | 0.853 |
| ns | 617 |  | 180 | README section map — every `##` heading | 1.6 |  | 0.689 |
| walker |  | 643 | 191 | headings outline in README.md |  |  | 0.888 |
| walker |  | 668 | 25 | README.md section #1 |  |  | 0.888 |
| walker |  | 733 | 65 | module statements at src/index.js:6 |  |  | 0.987 |
| ns | 766 |  | 149 | src/common.js — `setup(env)` and the complete `createDebug.*` public API attachment block | 2.1 |  | 0.864 |
| walker |  | 865 | 132 | YAML config at .travis.yml |  |  | 0.868 |
| ns | 907 |  | 141 | src/common.js — module-level state: `names`, `skips`, `formatters` | 2.2 |  | 0.790 |
| walker |  | 908 | 43 | README.md section #17 |  |  | 0.790 |
| walker |  | 930 | 22 | export names surface in src/common.js |  |  | 0.790 |
| walker |  | 930 | 0 | export at src/common.js:7 |  |  | 0.790 |
| walker |  | 940 | 10 | export body at src/common.js:7 body 146 |  |  | 0.790 |
| walker |  | 951 | 11 | export body at src/common.js:7 body 61 |  |  | 0.790 |
| walker |  | 962 | 11 | export body at src/common.js:7 body 62 |  |  | 0.790 |
| walker |  | 971 | 9 | export body at src/common.js:7 body 63 |  |  | 0.790 |
| walker |  | 980 | 9 | export body at src/common.js:7 body 64 |  |  | 0.790 |
| ns | 1062 |  | 155 | src/common.js — roster of every function definition plus the module tail | 2.3 |  | 0.729 |
| walker |  | 1159 | 179 | package identity metadata in package.json |  |  | 0.729 |
| walker |  | 1171 | 12 | export body at src/common.js:7 body 116 |  |  | 0.729 |
| ns | 1230 |  | 168 | src/node.js — the node adapter's export contract | 2.4 |  | 0.652 |
| walker |  | 1298 | 127 | README.md section #7 |  |  | 0.652 |
| walker |  | 1310 | 12 | export body at src/common.js:7 body 119 |  |  | 0.653 |
| ns | 1344 |  | 114 | src/browser.js — the browser adapter's export contract | 2.5 |  | 0.612 |
| walker |  | 1436 | 126 | README.md section #13 |  |  | 0.615 |
| ns | 1518 |  | 174 | src/node.js — roster of every function, export and formatter | 2.6 |  | 0.583 |
| ns | 1645 |  | 127 | src/browser.js — roster of every function, export and formatter | 2.7 |  | 0.562 |
| walker |  | 1697 | 261 | README.md section #9 |  |  | 0.566 |
| ns | 1803 |  | 158 | README: the complete `DEBUG_*` environment variable table | 3.1 |  | 0.590 |
| walker |  | 1826 | 129 | README.md section #15 |  |  | 0.593 |
| ns | 1989 |  | 186 | README: the complete `%` formatter table | 3.2 |  | 0.570 |
| walker |  | 1993 | 167 | README.md section #8 |  |  | 0.573 |
| walker |  | 2005 | 12 | export body at src/common.js:7 body 166 |  |  | 0.573 |
| walker |  | 2031 | 26 | imports in test.js |  |  | 0.573 |
| ns | 2156 |  | 167 | README: wildcard and exclusion syntax for `DEBUG` | 3.3 |  | 0.593 |
| ns | 2218 |  | 62 | README: every sub-`##` heading (`###`/`####`/`#####`) | 3.4 |  | 0.583 |
| walker |  | 2302 | 271 | README.md section #2 |  |  | 0.589 |
| ns | 2439 |  | 221 | README: the canonical usage example | 3.5 |  | 0.619 |
| walker |  | 2530 | 228 | README.md section #3 |  |  | 0.619 |
| ns | 2646 |  | 207 | README: namespace colors — when they turn on, per environment | 3.6 |  | 0.603 |
| walker |  | 2747 | 217 | README.md section #6 |  |  | 0.603 |
| walker |  | 2758 | 11 | export body at src/common.js:7 body 167 |  |  | 0.603 |
| ns | 2856 |  | 210 | README: namespace naming conventions + the `DEBUG_*` → `util.inspect` options note | 3.7 |  | 0.612 |
| ns | 2950 |  | 94 | README: checking and forcing `debug.enabled` | 3.8 |  | 0.625 |
| ns | 3076 |  | 126 | README: `log.extend()` for sub-namespaces | 3.9 |  | 0.639 |
| walker |  | 3147 | 389 | README.md section #10 |  |  | 0.676 |
| walker |  | 3159 | 12 | export body at src/common.js:7 body 164 |  |  | 0.676 |
| walker |  | 3170 | 11 | export body at src/common.js:7 body 163 |  |  | 0.677 |
| ns | 3201 |  | 125 | README: enabling debug dynamically via `enable()` / `disable()` | 3.10 |  | 0.656 |
| ns | 3381 |  | 180 | README: `enable(namespaces)` / `disable()` contract and the round-trip caveat | 3.11 |  | 0.635 |
| walker |  | 3399 | 229 | README.md section #16 |  |  | 0.638 |
| walker |  | 3449 | 50 | imports in test.node.js |  |  | 0.638 |
| ns | 3571 |  | 190 | README: adding a custom formatter | 3.12 |  | 0.652 |
| ns | 3743 |  | 172 | README: redirecting output by overriding `log` | 3.13 |  | 0.634 |
| walker |  | 3819 | 370 | README.md section #5 |  |  | 0.659 |
| walker |  | 3971 | 152 | export names surface in src/browser.js |  |  | 0.682 |
| walker |  | 3971 | 0 | export at src/browser.js:7 |  |  | 0.682 |
| walker |  | 3971 | 0 | export at src/browser.js:115 |  |  | 0.682 |
| walker |  | 3971 | 0 | export at src/browser.js:149 |  |  | 0.682 |
| walker |  | 3971 | 0 | export at src/browser.js:192 |  |  | 0.682 |
| walker |  | 3971 | 0 | export at src/browser.js:200 |  |  | 0.682 |
| walker |  | 3971 | 0 | export at src/browser.js:219 |  |  | 0.682 |
| ns | 3974 |  | 231 | README: browser build, `localStorage.debug`, and the Chromium Verbose caveat | 3.14 |  | 0.666 |
| ns | 4067 |  | 93 | README: setting `DEBUG` on Windows (CMD and PowerShell) | 3.15 |  | 0.655 |
| walker |  | 4069 | 98 | export at src/browser.js:12 |  |  | 0.657 |
| ns | 4167 |  | 100 | README: the millisecond diff feature | 3.16 |  | 0.658 |
| walker |  | 4335 | 266 | README.md section #12 |  |  | 0.689 |
| ns | 4362 |  | 195 | README: colors in child processes (`DEBUG_COLORS=1`) | 3.17 |  | 0.697 |
| walker |  | 4441 | 106 | export body at src/browser.js:200 body 201 |  |  | 0.698 |
| walker |  | 4467 | 26 | export doc at src/browser.js:7 |  |  | 0.708 |
| ns | 4477 |  | 115 | package.json: runtime dependency, optional peer dependency, and the xo lint override | 4.1 |  | 0.694 |
| walker |  | 4493 | 26 | imports in src/node.js |  |  | 0.695 |
| walker |  | 4529 | 36 | export doc at src/browser.js:149 |  |  | 0.695 |
| walker |  | 4730 | 201 | export names surface in src/node.js |  |  | 0.721 |
| walker |  | 4730 | 0 | export at src/node.js:12 |  |  | 0.721 |
| walker |  | 4730 | 0 | export at src/node.js:27 |  |  | 0.721 |
| walker |  | 4730 | 0 | export at src/node.js:155 |  |  | 0.721 |
| walker |  | 4730 | 0 | export at src/node.js:167 |  |  | 0.721 |
| walker |  | 4730 | 0 | export at src/node.js:193 |  |  | 0.721 |
| walker |  | 4730 | 0 | export at src/node.js:203 |  |  | 0.721 |
| walker |  | 4730 | 0 | export at src/node.js:220 |  |  | 0.721 |
| walker |  | 4730 | 0 | export at src/node.js:231 |  |  | 0.721 |
| walker |  | 4744 | 14 | export body at src/common.js:7 body 117 |  |  | 0.721 |
| walker |  | 4756 | 12 | export body at src/common.js:7 body 118 |  |  | 0.721 |
| walker |  | 4770 | 14 | export body at src/common.js:7 body 193 |  |  | 0.721 |
| walker |  | 4782 | 12 | export body at src/common.js:7 body 194 |  |  | 0.721 |
| ns | 4793 |  | 316 | test.js: header plus every `describe`/`it` title | 4.2 |  | 0.700 |
| walker |  | 4794 | 12 | export body at src/common.js:7 body 195 |  |  | 0.700 |
| walker |  | 4806 | 12 | export body at src/common.js:7 body 196 |  |  | 0.700 |
| walker |  | 4820 | 14 | export body at src/common.js:7 body 224 |  |  | 0.700 |
| walker |  | 4835 | 15 | export body at src/common.js:7 body 219 |  |  | 0.700 |
| walker |  | 4852 | 17 | export body at src/common.js:7 body 141 |  |  | 0.701 |
| walker |  | 4875 | 23 | export body at src/node.js:193 body 194 |  |  | 0.701 |
| walker |  | 4900 | 25 | export body at src/common.js:7 body 120 |  |  | 0.701 |
| walker |  | 4931 | 31 | export body at src/common.js:7 body 142 |  |  | 0.701 |
| walker |  | 4966 | 35 | export body at src/common.js:7 body 220 |  |  | 0.701 |
| ns | 4971 |  | 178 | test.node.js: the node-only suite and its sinon harness | 4.3 |  | 0.685 |
| walker |  | 5013 | 47 | export at src/node.js:18 |  |  | 0.690 |
| walker |  | 5051 | 38 | export body at src/common.js:7 body 284 |  |  | 0.690 |
| walker |  | 5090 | 39 | export body at src/node.js:155 body 156 |  |  | 0.690 |
| ns | 5115 |  | 144 | karma.conf.js: frameworks and the browser test file set | 4.4 |  | 0.678 |
| walker |  | 5131 | 41 | export body at src/common.js:7 body 273 |  |  | 0.679 |
| walker |  | 5183 | 52 | export body at src/common.js:7 body 150 |  |  | 0.679 |
| walker |  | 5241 | 58 | export body at src/common.js:7 body 169 |  |  | 0.679 |
| ns | 5247 |  | 132 | .travis.yml in full — the CI matrix | 4.5 |  | 0.688 |
| walker |  | 5256 | 15 | export doc at src/node.js:27 |  |  | 0.688 |
| walker |  | 5322 | 66 | export body at src/common.js:7 body 234 |  |  | 0.688 |
| walker |  | 5398 | 76 | export body at src/common.js:7 body 175 |  |  | 0.690 |
| walker |  | 5486 | 88 | export body at src/common.js:7 body 8 |  |  | 0.704 |
| ns | 5534 |  | 287 | karma.conf.js: launcher, preprocessors and run mode | 4.6 | 4.4 | 0.680 |
| walker |  | 5575 | 89 | export body at src/node.js:203 body 204 |  |  | 0.681 |
| walker |  | 5667 | 92 | export body at src/common.js:7 body 41 |  |  | 0.695 |
| walker |  | 5691 | 24 | export doc at src/node.js:12 |  |  | 0.704 |
| walker |  | 5720 | 29 | export doc at src/node.js:155 |  |  | 0.704 |
| ns | 5733 |  | 199 | src/common.js: `enable()` body — parsing the namespace string | 5.1 | 2.3 | 0.707 |
| walker |  | 5829 | 109 | export body at src/common.js:7 body 42 |  |  | 0.708 |
| walker |  | 5861 | 32 | export doc at src/node.js:193 |  |  | 0.708 |
| ns | 5951 |  | 218 | src/common.js: `debug(...args)` — enabled guard, ms-diff bookkeeping, `%O` coercion | 5.2 | 2.3 | 0.690 |
| walker |  | 5982 | 121 | export body at src/common.js:7 body 250 |  |  | 0.691 |
| walker |  | 6019 | 37 | export doc at src/node.js:167 |  |  | 0.691 |
| walker |  | 6057 | 38 | export doc at src/common.js:7 |  |  | 0.705 |
| walker |  | 6101 | 44 | export doc at src/browser.js:200 |  |  | 0.705 |
| ns | 6265 |  | 314 | src/common.js: `debug(...args)` — the `%`-formatter substitution loop and log dispatch | 5.3 | 5.2 | 0.685 |
| walker |  | 6451 | 350 | README.md section #20 |  |  | 0.685 |
| walker |  | 6497 | 46 | export doc at src/node.js:203 |  |  | 0.685 |
| ns | 6599 |  | 334 | src/common.js: per-instance properties and the `enabled` getter/setter | 5.4 | 2.3 | 0.667 |
| walker |  | 6658 | 161 | export body at src/browser.js:219 body 220 |  |  | 0.668 |
| ns | 6991 |  | 392 | src/common.js: `matchesTemplate()` — the wildcard matcher | 5.5 | 2.3 | 0.650 |
| walker |  | 7060 | 402 | README.md section #11 |  |  | 0.666 |
| walker |  | 7111 | 51 | export doc at src/browser.js:219 |  |  | 0.666 |
| walker |  | 7162 | 51 | export doc at src/node.js:220 |  |  | 0.666 |
| ns | 7188 |  | 197 | src/common.js: `disable()` and `enabled()` bodies | 5.6 | 2.3 | 0.673 |
| walker |  | 7341 | 179 | export body at src/common.js:7 body 122 |  |  | 0.698 |
| ns | 7437 |  | 249 | src/common.js: `selectColor()` and `extend()` bodies | 5.7 | 2.3 | 0.693 |
| ns | 7572 |  | 135 | src/common.js: `coerce()` and the deprecated `destroy()` stub | 5.8 | 2.3 | 0.689 |
| ns | 7648 |  | 76 | src/common.js: adapter-property spread and `createDebug`'s closure state | 5.9 | 2.1 | 0.686 |
| walker |  | 7851 | 510 | README.md section #4 |  |  | 0.703 |
| ns | 7943 |  | 295 | src/node.js: `inspectOpts` derivation from `DEBUG_*` environment variables | 6.1 | 2.6 | 0.687 |
| walker |  | 8053 | 202 | export body at src/node.js:167 body 168 |  |  | 0.688 |
| ns | 8207 |  | 264 | src/browser.js: `useColors()` — the inspector capability sniff | 6.2 | 2.7 | 0.682 |
| walker |  | 8319 | 266 | export at src/node.js:124 |  |  | 0.703 |
| walker |  | 8383 | 64 | export doc at src/node.js:231 |  |  | 0.703 |
| ns | 8529 |  | 322 | src/node.js: `useColors`, `formatArgs`, `getDate`, `log` bodies | 6.3 | 2.6 | 0.702 |
| walker |  | 8860 | 477 | README.md section #14 |  |  | 0.732 |
| ns | 8862 |  | 333 | src/browser.js: `formatArgs()` — `%c` CSS injection | 6.4 | 2.7 | 0.716 |
| walker |  | 8919 | 59 | export doc at src/node.js:124 |  |  | 0.720 |
| ns | 9046 |  | 184 | src/node.js: `save`, `load`, `init` bodies | 6.5 | 2.6 | 0.714 |
| ns | 9398 |  | 352 | src/browser.js: `save`, `load`, `localstorage` bodies | 6.6 | 2.7 | 0.711 |
| ns | 9542 |  | 144 | Formatter implementations: node `%o`/`%O` and browser `%j` | 6.7 | 2.6 | 0.703 |
| walker |  | 9680 | 761 | export at src/browser.js:27 |  |  | 0.703 |
| walker |  | 9695 | 15 | export doc at src/browser.js:27 |  |  | 0.703 |
| ns | 9723 |  | 181 | Both color palettes, head and tail, with the elision marked | 6.8 | 2.7 | 0.695 |
| ns | 9933 |  | 210 | LICENSE header, plus .gitignore in full and the .editorconfig head | 7.1 |  | 0.684 |
| walker |  | 9954 | 259 | export body at src/common.js:7 body 198 |  |  | 0.706 |

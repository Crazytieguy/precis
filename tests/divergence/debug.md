Score(3000)=0.571 I=0.785 C=0.416 ns_rows≤3K=21/54 (reached=10 partial=0 missing=11)

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
| ns | 766 |  | 149 | src/common.js — `setup(env)` and the complete `createDebug.*` public API attachment block | 2.1 |  | 0.777 |
| walker |  | 800 | 132 | YAML config at .travis.yml |  |  | 0.781 |
| walker |  | 843 | 43 | README.md section #16 |  |  | 0.781 |
| walker |  | 865 | 22 | export names surface in src/common.js |  |  | 0.782 |
| walker |  | 865 | 0 | export at src/common.js:7 |  |  | 0.782 |
| walker |  | 875 | 10 | export body at src/common.js:7 body 146 |  |  | 0.782 |
| walker |  | 886 | 11 | export body at src/common.js:7 body 61 |  |  | 0.782 |
| walker |  | 897 | 11 | export body at src/common.js:7 body 62 |  |  | 0.782 |
| walker |  | 906 | 9 | export body at src/common.js:7 body 63 |  |  | 0.782 |
| ns | 907 |  | 141 | src/common.js — module-level state: `names`, `skips`, `formatters` | 2.2 |  | 0.711 |
| walker |  | 915 | 9 | export body at src/common.js:7 body 64 |  |  | 0.711 |
| ns | 1062 |  | 155 | src/common.js — roster of every function definition plus the module tail | 2.3 |  | 0.656 |
| walker |  | 1094 | 179 | package identity metadata in package.json |  |  | 0.656 |
| walker |  | 1106 | 12 | export body at src/common.js:7 body 116 |  |  | 0.656 |
| ns | 1230 |  | 168 | src/node.js — the node adapter's export contract | 2.4 |  | 0.587 |
| walker |  | 1233 | 127 | README.md section #6 |  |  | 0.587 |
| walker |  | 1245 | 12 | export body at src/common.js:7 body 119 |  |  | 0.587 |
| ns | 1344 |  | 114 | src/browser.js — the browser adapter's export contract | 2.5 |  | 0.551 |
| walker |  | 1371 | 126 | README.md section #12 |  |  | 0.554 |
| ns | 1518 |  | 174 | src/node.js — roster of every function, export and formatter | 2.6 |  | 0.525 |
| walker |  | 1632 | 261 | README.md section #8 |  |  | 0.530 |
| ns | 1645 |  | 127 | src/browser.js — roster of every function, export and formatter | 2.7 |  | 0.510 |
| walker |  | 1761 | 129 | README.md section #14 |  |  | 0.513 |
| ns | 1803 |  | 158 | README: the complete `DEBUG_*` environment variable table | 3.1 |  | 0.540 |
| walker |  | 1928 | 167 | README.md section #7 |  |  | 0.543 |
| walker |  | 1940 | 12 | export body at src/common.js:7 body 166 |  |  | 0.543 |
| walker |  | 1966 | 26 | imports in test.js |  |  | 0.543 |
| ns | 1989 |  | 186 | README: the complete `%` formatter table | 3.2 |  | 0.522 |
| ns | 2156 |  | 167 | README: wildcard and exclusion syntax for `DEBUG` | 3.3 |  | 0.545 |
| walker |  | 2183 | 217 | README.md section #5 |  |  | 0.545 |
| walker |  | 2194 | 11 | export body at src/common.js:7 body 167 |  |  | 0.545 |
| ns | 2218 |  | 62 | README: every sub-`##` heading (`###`/`####`/`#####`) | 3.4 |  | 0.536 |
| ns | 2439 |  | 221 | README: the canonical usage example | 3.5 |  | 0.509 |
| walker |  | 2583 | 389 | README.md section #9 |  |  | 0.557 |
| walker |  | 2595 | 12 | export body at src/common.js:7 body 164 |  |  | 0.557 |
| walker |  | 2606 | 11 | export body at src/common.js:7 body 163 |  |  | 0.557 |
| ns | 2646 |  | 207 | README: namespace colors — when they turn on, per environment | 3.6 |  | 0.543 |
| walker |  | 2835 | 229 | README.md section #15 |  |  | 0.545 |
| ns | 2856 |  | 210 | README: namespace naming conventions + the `DEBUG_*` → `util.inspect` options note | 3.7 |  | 0.556 |
| walker |  | 2885 | 50 | imports in test.node.js |  |  | 0.556 |
| ns | 2950 |  | 94 | README: checking and forcing `debug.enabled` | 3.8 |  | 0.571 |
| ns | 3076 |  | 126 | README: `log.extend()` for sub-namespaces | 3.9 |  | 0.588 |
| ns | 3201 |  | 125 | README: enabling debug dynamically via `enable()` / `disable()` | 3.10 |  | 0.570 |
| walker |  | 3255 | 370 | README.md section #4 |  |  | 0.600 |
| ns | 3381 |  | 180 | README: `enable(namespaces)` / `disable()` contract and the round-trip caveat | 3.11 |  | 0.581 |
| walker |  | 3407 | 152 | export names surface in src/browser.js |  |  | 0.607 |
| walker |  | 3407 | 0 | export at src/browser.js:7 |  |  | 0.607 |
| walker |  | 3407 | 0 | export at src/browser.js:115 |  |  | 0.607 |
| walker |  | 3407 | 0 | export at src/browser.js:149 |  |  | 0.607 |
| walker |  | 3407 | 0 | export at src/browser.js:192 |  |  | 0.607 |
| walker |  | 3407 | 0 | export at src/browser.js:200 |  |  | 0.607 |
| walker |  | 3407 | 0 | export at src/browser.js:219 |  |  | 0.607 |
| walker |  | 3421 | 14 | export body at src/common.js:7 body 117 |  |  | 0.607 |
| walker |  | 3433 | 12 | export body at src/common.js:7 body 118 |  |  | 0.607 |
| walker |  | 3447 | 14 | export body at src/common.js:7 body 193 |  |  | 0.607 |
| walker |  | 3459 | 12 | export body at src/common.js:7 body 194 |  |  | 0.607 |
| walker |  | 3471 | 12 | export body at src/common.js:7 body 195 |  |  | 0.607 |
| walker |  | 3483 | 12 | export body at src/common.js:7 body 196 |  |  | 0.607 |
| walker |  | 3497 | 14 | export body at src/common.js:7 body 224 |  |  | 0.607 |
| walker |  | 3512 | 15 | export body at src/common.js:7 body 219 |  |  | 0.607 |
| walker |  | 3529 | 17 | export body at src/common.js:7 body 141 |  |  | 0.607 |
| walker |  | 3554 | 25 | export body at src/common.js:7 body 120 |  |  | 0.607 |
| ns | 3571 |  | 190 | README: adding a custom formatter | 3.12 |  | 0.622 |
| walker |  | 3585 | 31 | export body at src/common.js:7 body 142 |  |  | 0.622 |
| walker |  | 3683 | 98 | export at src/browser.js:12 |  |  | 0.624 |
| walker |  | 3718 | 35 | export body at src/common.js:7 body 220 |  |  | 0.624 |
| ns | 3743 |  | 172 | README: redirecting output by overriding `log` | 3.13 |  | 0.607 |
| walker |  | 3756 | 38 | export body at src/common.js:7 body 284 |  |  | 0.607 |
| walker |  | 3797 | 41 | export body at src/common.js:7 body 273 |  |  | 0.607 |
| walker |  | 3849 | 52 | export body at src/common.js:7 body 150 |  |  | 0.607 |
| walker |  | 3907 | 58 | export body at src/common.js:7 body 169 |  |  | 0.608 |
| walker |  | 3973 | 66 | export body at src/common.js:7 body 234 |  |  | 0.608 |
| ns | 3974 |  | 231 | README: browser build, `localStorage.debug`, and the Chromium Verbose caveat | 3.14 |  | 0.594 |
| walker |  | 4049 | 76 | export body at src/common.js:7 body 175 |  |  | 0.595 |
| ns | 4067 |  | 93 | README: setting `DEBUG` on Windows (CMD and PowerShell) | 3.15 |  | 0.585 |
| walker |  | 4137 | 88 | export body at src/common.js:7 body 8 |  |  | 0.603 |
| ns | 4167 |  | 100 | README: the millisecond diff feature | 3.16 |  | 0.605 |
| walker |  | 4229 | 92 | export body at src/common.js:7 body 41 |  |  | 0.623 |
| ns | 4362 |  | 195 | README: colors in child processes (`DEBUG_COLORS=1`) | 3.17 |  | 0.635 |
| ns | 4477 |  | 115 | package.json: runtime dependency, optional peer dependency, and the xo lint override | 4.1 |  | 0.623 |
| walker |  | 4495 | 266 | README.md section #11 |  |  | 0.652 |
| walker |  | 4601 | 106 | export body at src/browser.js:200 body 201 |  |  | 0.652 |
| walker |  | 4627 | 26 | export doc at src/browser.js:7 |  |  | 0.662 |
| walker |  | 4736 | 109 | export body at src/common.js:7 body 42 |  |  | 0.662 |
| walker |  | 4762 | 26 | imports in src/node.js |  |  | 0.663 |
| ns | 4793 |  | 316 | test.js: header plus every `describe`/`it` title | 4.2 |  | 0.644 |
| ns | 4971 |  | 178 | test.node.js: the node-only suite and its sinon harness | 4.3 |  | 0.629 |
| ns | 5115 |  | 144 | karma.conf.js: frameworks and the browser test file set | 4.4 |  | 0.619 |
| ns | 5247 |  | 132 | .travis.yml in full — the CI matrix | 4.5 |  | 0.630 |
| walker |  | 5266 | 504 | README.md section #2 |  |  | 0.661 |
| walker |  | 5387 | 121 | export body at src/common.js:7 body 250 |  |  | 0.662 |
| walker |  | 5423 | 36 | export doc at src/browser.js:149 |  |  | 0.662 |
| ns | 5534 |  | 287 | karma.conf.js: launcher, preprocessors and run mode | 4.6 | 4.4 | 0.640 |
| walker |  | 5624 | 201 | export names surface in src/node.js |  |  | 0.662 |
| walker |  | 5624 | 0 | export at src/node.js:12 |  |  | 0.662 |
| walker |  | 5624 | 0 | export at src/node.js:27 |  |  | 0.662 |
| walker |  | 5624 | 0 | export at src/node.js:155 |  |  | 0.662 |
| walker |  | 5624 | 0 | export at src/node.js:167 |  |  | 0.662 |
| walker |  | 5624 | 0 | export at src/node.js:193 |  |  | 0.662 |
| walker |  | 5624 | 0 | export at src/node.js:203 |  |  | 0.662 |
| walker |  | 5624 | 0 | export at src/node.js:220 |  |  | 0.662 |
| walker |  | 5624 | 0 | export at src/node.js:231 |  |  | 0.662 |
| walker |  | 5647 | 23 | export body at src/node.js:193 body 194 |  |  | 0.662 |
| walker |  | 5694 | 47 | export at src/node.js:18 |  |  | 0.666 |
| walker |  | 5733 | 39 | export body at src/node.js:155 body 156 |  |  | 0.670 |
| ns | 5733 |  | 199 | src/common.js: `enable()` body — parsing the namespace string | 5.1 | 2.3 | 0.670 |
| walker |  | 5748 | 15 | export doc at src/node.js:27 |  |  | 0.670 |
| walker |  | 5837 | 89 | export body at src/node.js:203 body 204 |  |  | 0.671 |
| walker |  | 5861 | 24 | export doc at src/node.js:12 |  |  | 0.679 |
| walker |  | 5890 | 29 | export doc at src/node.js:155 |  |  | 0.679 |
| walker |  | 5922 | 32 | export doc at src/node.js:193 |  |  | 0.679 |
| ns | 5951 |  | 218 | src/common.js: `debug(...args)` — enabled guard, ms-diff bookkeeping, `%O` coercion | 5.2 | 2.3 | 0.662 |
| walker |  | 5959 | 37 | export doc at src/node.js:167 |  |  | 0.662 |
| walker |  | 5997 | 38 | export doc at src/common.js:7 |  |  | 0.675 |
| walker |  | 6041 | 44 | export doc at src/browser.js:200 |  |  | 0.675 |
| ns | 6265 |  | 314 | src/common.js: `debug(...args)` — the `%`-formatter substitution loop and log dispatch | 5.3 | 5.2 | 0.656 |
| walker |  | 6391 | 350 | README.md section #19 |  |  | 0.656 |
| walker |  | 6437 | 46 | export doc at src/node.js:203 |  |  | 0.656 |
| walker |  | 6598 | 161 | export body at src/browser.js:219 body 220 |  |  | 0.657 |
| ns | 6599 |  | 334 | src/common.js: per-instance properties and the `enabled` getter/setter | 5.4 | 2.3 | 0.640 |
| ns | 6991 |  | 392 | src/common.js: `matchesTemplate()` — the wildcard matcher | 5.5 | 2.3 | 0.623 |
| walker |  | 7000 | 402 | README.md section #10 |  |  | 0.639 |
| walker |  | 7051 | 51 | export doc at src/browser.js:219 |  |  | 0.639 |
| walker |  | 7102 | 51 | export doc at src/node.js:220 |  |  | 0.639 |
| ns | 7188 |  | 197 | src/common.js: `disable()` and `enabled()` bodies | 5.6 | 2.3 | 0.646 |
| walker |  | 7281 | 179 | export body at src/common.js:7 body 122 |  |  | 0.671 |
| ns | 7437 |  | 249 | src/common.js: `selectColor()` and `extend()` bodies | 5.7 | 2.3 | 0.666 |
| ns | 7572 |  | 135 | src/common.js: `coerce()` and the deprecated `destroy()` stub | 5.8 | 2.3 | 0.662 |
| ns | 7648 |  | 76 | src/common.js: adapter-property spread and `createDebug`'s closure state | 5.9 | 2.1 | 0.659 |
| walker |  | 7791 | 510 | README.md section #3 |  |  | 0.676 |
| ns | 7943 |  | 295 | src/node.js: `inspectOpts` derivation from `DEBUG_*` environment variables | 6.1 | 2.6 | 0.661 |
| walker |  | 7993 | 202 | export body at src/node.js:167 body 168 |  |  | 0.662 |
| ns | 8207 |  | 264 | src/browser.js: `useColors()` — the inspector capability sniff | 6.2 | 2.7 | 0.656 |
| walker |  | 8259 | 266 | export at src/node.js:124 |  |  | 0.677 |
| walker |  | 8323 | 64 | export doc at src/node.js:231 |  |  | 0.677 |
| ns | 8529 |  | 322 | src/node.js: `useColors`, `formatArgs`, `getDate`, `log` bodies | 6.3 | 2.6 | 0.676 |
| walker |  | 8800 | 477 | README.md section #13 |  |  | 0.706 |
| walker |  | 8859 | 59 | export doc at src/node.js:124 |  |  | 0.710 |
| ns | 8862 |  | 333 | src/browser.js: `formatArgs()` — `%c` CSS injection | 6.4 | 2.7 | 0.694 |
| ns | 9046 |  | 184 | src/node.js: `save`, `load`, `init` bodies | 6.5 | 2.6 | 0.688 |
| ns | 9398 |  | 352 | src/browser.js: `save`, `load`, `localstorage` bodies | 6.6 | 2.7 | 0.686 |
| ns | 9542 |  | 144 | Formatter implementations: node `%o`/`%O` and browser `%j` | 6.7 | 2.6 | 0.678 |
| walker |  | 9620 | 761 | export at src/browser.js:27 |  |  | 0.679 |
| walker |  | 9635 | 15 | export doc at src/browser.js:27 |  |  | 0.679 |
| ns | 9723 |  | 181 | Both color palettes, head and tail, with the elision marked | 6.8 | 2.7 | 0.671 |
| walker |  | 9894 | 259 | export body at src/common.js:7 body 198 |  |  | 0.692 |
| ns | 9933 |  | 210 | LICENSE header, plus .gitignore in full and the .editorconfig head | 7.1 |  | 0.681 |

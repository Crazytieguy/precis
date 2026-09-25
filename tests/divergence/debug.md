Score(3000)=0.582 I=0.778 C=0.436 ns_rows≤3K=21/54 grid(1000/1442/2080/3000/4327/6240/9000)=0.727/0.623/0.616/0.582/0.628/0.589/0.565

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 31 | 31 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 45 |  | 45 | Repo identity: README title + one-sentence description | 1.1 |  | 0.000 |
| walker |  | 47 | 16 | Fs::DirListing { dir: src } |  |  | 0.000 |
| walker |  | 90 | 43 | Code::CodeKey { rung: ModuleDoc, file: src/index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.000 |
| ns | 92 |  | 47 | Complete file map: repo root and src/ | 1.2 |  | 0.633 |
| walker |  | 135 | 45 | Markdown::ReadmeHeadline { file: README.md } |  |  | 1.000 |
| walker |  | 200 | 65 | Json::Identity { file: package.json } |  |  | 1.000 |
| ns | 205 |  | 113 | src/index.js in full — the environment dispatch | 1.3 |  | 0.791 |
| walker |  | 232 | 32 | Json::Dependencies { file: package.json } |  |  | 0.791 |
| walker |  | 254 | 22 | Code::CodeKey { rung: Names, file: src/common.js, decl: 0, sub: 0, line: 0 } |  |  | 0.791 |
| walker |  | 286 | 32 | Json::Runtime { file: package.json } |  |  | 0.800 |
| walker |  | 300 | 14 | Code::CodeKey { rung: Names, file: karma.conf.js, decl: 0, sub: 0, line: 0 } |  |  | 0.800 |
| ns | 328 |  | 123 | package.json identity, entry points, engines, license | 1.4 |  | 0.759 |
| walker |  | 368 | 68 | Json::Entry { file: package.json } |  |  | 0.830 |
| walker |  | 406 | 38 | Code::CodeKey { rung: Doc, file: src/common.js, decl: 1, sub: 0, line: 7 } |  |  | 0.833 |
| ns | 436 |  | 108 | package.json `scripts` — every build/test/lint entry point | 1.5 |  | 0.747 |
| walker |  | 512 | 106 | Json::Scripts { file: package.json } |  |  | 0.856 |
| ns | 616 |  | 180 | README section map — every `##` heading | 1.6 |  | 0.692 |
| walker |  | 675 | 163 | Code::CodeKey { rung: Names, file: src/browser.js, decl: 0, sub: 0, line: 0 } |  |  | 0.698 |
| ns | 765 |  | 149 | src/common.js — `setup(env)` and the complete `createDebug.*` public API attachment block | 2.1 |  | 0.625 |
| walker |  | 773 | 98 | Code::CodeKey { rung: Decl, file: src/browser.js, decl: 2, sub: 0, line: 12 } |  |  | 0.626 |
| ns | 906 |  | 141 | src/common.js — module-level state: `names`, `skips`, `formatters` | 2.2 |  | 0.569 |
| walker |  | 964 | 191 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.727 |
| walker |  | 989 | 25 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.727 |
| ns | 1061 |  | 155 | src/common.js — roster of every function definition plus the module tail | 2.3 |  | 0.670 |
| walker |  | 1192 | 203 | Code::CodeKey { rung: Names, file: src/node.js, decl: 0, sub: 0, line: 0 } |  |  | 0.676 |
| walker |  | 1217 | 25 | Code::CodeKey { rung: Decl, file: src/node.js, decl: 3, sub: 0, line: 124 } |  |  | 0.676 |
| ns | 1229 |  | 168 | src/node.js — the node adapter's export contract | 2.4 |  | 0.625 |
| walker |  | 1264 | 47 | Code::CodeKey { rung: Decl, file: src/node.js, decl: 1, sub: 0, line: 18 } |  |  | 0.637 |
| walker |  | 1273 | 9 | Code::CodeKey { rung: Body, file: src/node.js, decl: 8, sub: 0, line: 220 } |  |  | 0.637 |
| walker |  | 1288 | 15 | Code::CodeKey { rung: Doc, file: src/node.js, decl: 2, sub: 0, line: 27 } |  |  | 0.637 |
| ns | 1343 |  | 114 | src/browser.js — the browser adapter's export contract | 2.5 |  | 0.623 |
| ns | 1517 |  | 174 | src/node.js — roster of every function, export and formatter | 2.6 |  | 0.618 |
| walker |  | 1549 | 261 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: true } |  |  | 0.622 |
| walker |  | 1572 | 23 | Code::CodeKey { rung: Body, file: src/node.js, decl: 6, sub: 0, line: 193 } |  |  | 0.622 |
| walker |  | 1608 | 36 | Code::CodeKey { rung: Doc, file: src/browser.js, decl: 5, sub: 0, line: 149 } |  |  | 0.622 |
| ns | 1644 |  | 127 | src/browser.js — roster of every function, export and formatter | 2.7 |  | 0.625 |
| walker |  | 1651 | 43 | Markdown::Section { file: README.md, section_index: 17, keeps_default_concavity: false } |  |  | 0.625 |
| ns | 1802 |  | 158 | README: the complete `DEBUG_*` environment variable table | 3.1 |  | 0.641 |
| walker |  | 1830 | 179 | Json::IdentityMeta { file: package.json } |  |  | 0.641 |
| walker |  | 1874 | 44 | Code::CodeKey { rung: Doc, file: src/browser.js, decl: 7, sub: 0, line: 200 } |  |  | 0.641 |
| ns | 1988 |  | 186 | README: the complete `%` formatter table | 3.2 |  | 0.616 |
| ns | 2155 |  | 167 | README: wildcard and exclusion syntax for `DEBUG` | 3.3 |  | 0.597 |
| ns | 2217 |  | 62 | README: every sub-`##` heading (`###`/`####`/`#####`) | 3.4 |  | 0.587 |
| walker |  | 2263 | 389 | Markdown::Section { file: README.md, section_index: 10, keeps_default_concavity: true } |  |  | 0.634 |
| walker |  | 2314 | 51 | Code::CodeKey { rung: Doc, file: src/browser.js, decl: 8, sub: 0, line: 219 } |  |  | 0.634 |
| walker |  | 2343 | 29 | Code::CodeKey { rung: Doc, file: src/node.js, decl: 4, sub: 0, line: 155 } |  |  | 0.634 |
| walker |  | 2375 | 32 | Code::CodeKey { rung: Doc, file: src/node.js, decl: 6, sub: 0, line: 193 } |  |  | 0.634 |
| ns | 2438 |  | 221 | README: the canonical usage example | 3.5 |  | 0.603 |
| walker |  | 2502 | 127 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.604 |
| walker |  | 2585 | 83 | Code::CodeKey { rung: Doc, file: src/browser.js, decl: 6, sub: 0, line: 192 } |  |  | 0.604 |
| ns | 2645 |  | 207 | README: namespace colors — when they turn on, per environment | 3.6 |  | 0.588 |
| walker |  | 2841 | 256 | Code::CodeKey { rung: Decl, file: src/browser.js, decl: 3, sub: 0, line: 27 } |  |  | 0.588 |
| ns | 2855 |  | 210 | README: namespace naming conventions + the `DEBUG_*` → `util.inspect` options note | 3.7 |  | 0.596 |
| walker |  | 2856 | 15 | Code::CodeKey { rung: Doc, file: src/browser.js, decl: 3, sub: 0, line: 27 } |  |  | 0.596 |
| walker |  | 2893 | 37 | Code::CodeKey { rung: Doc, file: src/node.js, decl: 5, sub: 0, line: 167 } |  |  | 0.596 |
| ns | 2949 |  | 94 | README: checking and forcing `debug.enabled` | 3.8 |  | 0.582 |
| walker |  | 3019 | 126 | Markdown::Section { file: README.md, section_index: 13, keeps_default_concavity: false } |  |  | 0.585 |
| ns | 3075 |  | 126 | README: `log.extend()` for sub-namespaces | 3.9 |  | 0.600 |
| walker |  | 3148 | 129 | Markdown::Section { file: README.md, section_index: 15, keeps_default_concavity: false } |  |  | 0.628 |
| ns | 3200 |  | 125 | README: enabling debug dynamically via `enable()` / `disable()` | 3.10 |  | 0.609 |
| walker |  | 3315 | 167 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.636 |
| ns | 3380 |  | 180 | README: `enable(namespaces)` / `disable()` contract and the round-trip caveat | 3.11 |  | 0.616 |
| ns | 3570 |  | 190 | README: adding a custom formatter | 3.12 |  | 0.629 |
| walker |  | 3586 | 271 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.671 |
| ns | 3742 |  | 172 | README: redirecting output by overriding `log` | 3.13 |  | 0.653 |
| walker |  | 3814 | 228 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: true } |  |  | 0.653 |
| ns | 3973 |  | 231 | README: browser build, `localStorage.debug`, and the Chromium Verbose caveat | 3.14 |  | 0.637 |
| walker |  | 4031 | 217 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.638 |
| ns | 4066 |  | 93 | README: setting `DEBUG` on Windows (CMD and PowerShell) | 3.15 |  | 0.627 |
| walker |  | 4070 | 39 | Code::CodeKey { rung: Body, file: src/node.js, decl: 4, sub: 0, line: 155 } |  |  | 0.627 |
| ns | 4166 |  | 100 | README: the millisecond diff feature | 3.16 |  | 0.628 |
| walker |  | 4329 | 259 | Code::CodeKey { rung: Decl, file: src/browser.js, decl: 3, sub: 1, line: 27 } |  |  | 0.628 |
| ns | 4361 |  | 195 | README: colors in child processes (`DEBUG_COLORS=1`) | 3.17 |  | 0.613 |
| walker |  | 4375 | 46 | Code::CodeKey { rung: Doc, file: src/node.js, decl: 7, sub: 0, line: 203 } |  |  | 0.613 |
| ns | 4476 |  | 115 | package.json: runtime dependency, optional peer dependency, and the xo lint override | 4.1 |  | 0.601 |
| walker |  | 4604 | 229 | Markdown::Section { file: README.md, section_index: 16, keeps_default_concavity: false } |  |  | 0.630 |
| ns | 4792 |  | 316 | test.js: header plus every `describe`/`it` title | 4.2 |  | 0.611 |
| walker |  | 4850 | 246 | Code::CodeKey { rung: Decl, file: src/browser.js, decl: 3, sub: 2, line: 27 } |  |  | 0.611 |
| ns | 4970 |  | 178 | test.node.js: the node-only suite and its sinon harness | 4.3 |  | 0.596 |
| ns | 5114 |  | 144 | karma.conf.js: frameworks and the browser test file set | 4.4 |  | 0.586 |
| walker |  | 5220 | 370 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.606 |
| ns | 5246 |  | 132 | .travis.yml in full — the CI matrix | 4.5 |  | 0.592 |
| walker |  | 5486 | 266 | Markdown::Section { file: README.md, section_index: 12, keeps_default_concavity: false } |  |  | 0.617 |
| ns | 5533 |  | 287 | karma.conf.js: launcher, preprocessors and run mode | 4.6 | 4.4 | 0.596 |
| walker |  | 5537 | 51 | Code::CodeKey { rung: Doc, file: src/node.js, decl: 8, sub: 0, line: 220 } |  |  | 0.596 |
| ns | 5732 |  | 199 | src/common.js: `enable()` body — parsing the namespace string | 5.1 | 2.3 | 0.581 |
| ns | 5950 |  | 218 | src/common.js: `debug(...args)` — enabled guard, ms-diff bookkeeping, `%O` coercion | 5.2 | 2.3 | 0.566 |
| walker |  | 6047 | 510 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.589 |
| ns | 6264 |  | 314 | src/common.js: `debug(...args)` — the `%`-formatter substitution loop and log dispatch | 5.3 | 5.2 | 0.572 |
| walker |  | 6397 | 350 | Markdown::Section { file: README.md, section_index: 20, keeps_default_concavity: false } |  |  | 0.572 |
| walker |  | 6456 | 59 | Code::CodeKey { rung: Doc, file: src/node.js, decl: 3, sub: 0, line: 124 } |  |  | 0.572 |
| walker |  | 6547 | 91 | Code::CodeKey { rung: Doc, file: src/browser.js, decl: 9, sub: 0, line: 247 } |  |  | 0.572 |
| ns | 6598 |  | 334 | src/common.js: per-instance properties and the `enabled` getter/setter | 5.4 | 2.3 | 0.553 |
| walker |  | 6949 | 402 | Markdown::Section { file: README.md, section_index: 11, keeps_default_concavity: false } |  |  | 0.570 |
| ns | 6990 |  | 392 | src/common.js: `matchesTemplate()` — the wildcard matcher | 5.5 | 2.3 | 0.552 |
| walker |  | 7013 | 64 | Code::CodeKey { rung: Doc, file: src/node.js, decl: 9, sub: 0, line: 231 } |  |  | 0.552 |
| ns | 7187 |  | 197 | src/common.js: `disable()` and `enabled()` bodies | 5.6 | 2.3 | 0.541 |
| ns | 7436 |  | 249 | src/common.js: `selectColor()` and `extend()` bodies | 5.7 | 2.3 | 0.532 |
| walker |  | 7490 | 477 | Markdown::Section { file: README.md, section_index: 14, keeps_default_concavity: false } |  |  | 0.568 |
| ns | 7571 |  | 135 | src/common.js: `coerce()` and the deprecated `destroy()` stub | 5.8 | 2.3 | 0.562 |
| walker |  | 7596 | 106 | Code::CodeKey { rung: Body, file: src/browser.js, decl: 7, sub: 0, line: 200 } |  |  | 0.563 |
| ns | 7647 |  | 76 | src/common.js: adapter-property spread and `createDebug`'s closure state | 5.9 | 2.1 | 0.558 |
| walker |  | 7669 | 73 | Code::CodeKey { rung: Body, file: src/node.js, decl: 9, sub: 0, line: 231 } |  |  | 0.558 |
| walker |  | 7778 | 109 | Code::CodeKey { rung: Body, file: src/browser.js, decl: 9, sub: 0, line: 247 } |  |  | 0.559 |
| ns | 7942 |  | 295 | src/node.js: `inspectOpts` derivation from `DEBUG_*` environment variables | 6.1 | 2.6 | 0.547 |
| walker |  | 8047 | 269 | Code::CodeKey { rung: Body, file: src/common.js, decl: 1, sub: 0, line: 7 } |  |  | 0.592 |
| walker |  | 8136 | 89 | Code::CodeKey { rung: Body, file: src/node.js, decl: 7, sub: 0, line: 203 } |  |  | 0.592 |
| ns | 8206 |  | 264 | src/browser.js: `useColors()` — the inspector capability sniff | 6.2 | 2.7 | 0.587 |
| walker |  | 8297 | 161 | Code::CodeKey { rung: Body, file: src/browser.js, decl: 8, sub: 0, line: 219 } |  |  | 0.588 |
| ns | 8528 |  | 322 | src/node.js: `useColors`, `formatArgs`, `getDate`, `log` bodies | 6.3 | 2.6 | 0.578 |
| ns | 8861 |  | 333 | src/browser.js: `formatArgs()` — `%c` CSS injection | 6.4 | 2.7 | 0.565 |
| ns | 9045 |  | 184 | src/node.js: `save`, `load`, `init` bodies | 6.5 | 2.6 | 0.571 |
| ns | 9397 |  | 352 | src/browser.js: `save`, `load`, `localstorage` bodies | 6.6 | 2.7 | 0.583 |
| walker |  | 9520 | 1223 | Code::CodeKey { rung: Body, file: src/common.js, decl: 1, sub: 1, line: 7 } |  |  | 0.675 |
| ns | 9541 |  | 144 | Formatter implementations: node `%o`/`%O` and browser `%j` | 6.7 | 2.6 | 0.667 |
| ns | 9722 |  | 181 | Both color palettes, head and tail, with the elision marked | 6.8 | 2.7 | 0.660 |
| ns | 9932 |  | 210 | LICENSE header, plus .gitignore in full and the .editorconfig head | 7.1 |  | 0.649 |

Score(3000)=0.574 I=0.766 C=0.430 ns_rows≤3K=21/54 grid(1000/1442/2080/3000/4327/6240/9000)=0.698/0.540/0.610/0.574/0.579/0.592/0.597

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 31 | 31 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 45 |  | 45 | Repo identity: README title + one-sentence description | 1.1 |  | 0.000 |
| walker |  | 47 | 16 | Fs::DirListing { dir: src } |  |  | 0.000 |
| walker |  | 92 | 45 | Markdown::ReadmeHeadline { file: README.md } |  |  | 1.000 |
| ns | 92 |  | 47 | Complete file map: repo root and src/ | 1.2 |  | 1.000 |
| walker |  | 157 | 65 | Json::Identity { file: package.json } |  |  | 1.000 |
| walker |  | 189 | 32 | Json::Dependencies { file: package.json } |  |  | 1.000 |
| ns | 205 |  | 113 | src/index.js in full — the environment dispatch | 1.3 |  | 0.755 |
| walker |  | 221 | 32 | Json::Runtime { file: package.json } |  |  | 0.764 |
| walker |  | 289 | 68 | Json::Entry { file: package.json } |  |  | 0.776 |
| ns | 328 |  | 123 | package.json identity, entry points, engines, license | 1.4 |  | 0.803 |
| walker |  | 395 | 106 | Json::Scripts { file: package.json } |  |  | 0.821 |
| ns | 436 |  | 108 | package.json `scripts` — every build/test/lint entry point | 1.5 |  | 0.829 |
| walker |  | 586 | 191 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.866 |
| walker |  | 611 | 25 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.866 |
| ns | 616 |  | 180 | README section map — every `##` heading | 1.6 |  | 0.870 |
| walker |  | 625 | 14 | Code::CodeKey { rung: Names, file: karma.conf.js, decl: 0, sub: 0, line: 0 } |  |  | 0.870 |
| ns | 765 |  | 149 | src/common.js — `setup(env)` and the complete `createDebug.*` public API attachment block | 2.1 |  | 0.762 |
| walker |  | 886 | 261 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: true } |  |  | 0.768 |
| ns | 906 |  | 141 | src/common.js — module-level state: `names`, `skips`, `formatters` | 2.2 |  | 0.698 |
| walker |  | 929 | 43 | Markdown::Section { file: README.md, section_index: 17, keeps_default_concavity: false } |  |  | 0.698 |
| ns | 1061 |  | 155 | src/common.js — roster of every function definition plus the module tail | 2.3 |  | 0.644 |
| walker |  | 1108 | 179 | Json::IdentityMeta { file: package.json } |  |  | 0.644 |
| ns | 1229 |  | 168 | src/node.js — the node adapter's export contract | 2.4 |  | 0.576 |
| ns | 1343 |  | 114 | src/browser.js — the browser adapter's export contract | 2.5 |  | 0.540 |
| walker |  | 1497 | 389 | Markdown::Section { file: README.md, section_index: 10, keeps_default_concavity: true } |  |  | 0.548 |
| ns | 1517 |  | 174 | src/node.js — roster of every function, export and formatter | 2.6 |  | 0.519 |
| walker |  | 1519 | 22 | Code::CodeKey { rung: Names, file: src/common.js, decl: 0, sub: 0, line: 0 } |  |  | 0.520 |
| walker |  | 1557 | 38 | Code::CodeKey { rung: Doc, file: src/common.js, decl: 1, sub: 0, line: 7 } |  |  | 0.529 |
| ns | 1644 |  | 127 | src/browser.js — roster of every function, export and formatter | 2.7 |  | 0.510 |
| walker |  | 1720 | 163 | Code::CodeKey { rung: Names, file: src/browser.js, decl: 0, sub: 0, line: 0 } |  |  | 0.561 |
| ns | 1802 |  | 158 | README: the complete `DEBUG_*` environment variable table | 3.1 |  | 0.582 |
| walker |  | 1818 | 98 | Code::CodeKey { rung: Decl, file: src/browser.js, decl: 2, sub: 0, line: 12 } |  |  | 0.589 |
| walker |  | 1854 | 36 | Code::CodeKey { rung: Doc, file: src/browser.js, decl: 5, sub: 0, line: 149 } |  |  | 0.589 |
| walker |  | 1898 | 44 | Code::CodeKey { rung: Doc, file: src/browser.js, decl: 7, sub: 0, line: 200 } |  |  | 0.589 |
| walker |  | 1949 | 51 | Code::CodeKey { rung: Doc, file: src/browser.js, decl: 8, sub: 0, line: 219 } |  |  | 0.589 |
| ns | 1988 |  | 186 | README: the complete `%` formatter table | 3.2 |  | 0.610 |
| walker |  | 2152 | 203 | Code::CodeKey { rung: Names, file: src/node.js, decl: 0, sub: 0, line: 0 } |  |  | 0.649 |
| ns | 2155 |  | 167 | README: wildcard and exclusion syntax for `DEBUG` | 3.3 |  | 0.629 |
| walker |  | 2177 | 25 | Code::CodeKey { rung: Decl, file: src/node.js, decl: 3, sub: 0, line: 124 } |  |  | 0.629 |
| ns | 2217 |  | 62 | README: every sub-`##` heading (`###`/`####`/`#####`) | 3.4 |  | 0.618 |
| walker |  | 2224 | 47 | Code::CodeKey { rung: Decl, file: src/node.js, decl: 1, sub: 0, line: 18 } |  |  | 0.625 |
| walker |  | 2233 | 9 | Code::CodeKey { rung: Body, file: src/node.js, decl: 8, sub: 0, line: 220 } |  |  | 0.625 |
| walker |  | 2248 | 15 | Code::CodeKey { rung: Doc, file: src/node.js, decl: 2, sub: 0, line: 27 } |  |  | 0.625 |
| walker |  | 2277 | 29 | Code::CodeKey { rung: Doc, file: src/node.js, decl: 4, sub: 0, line: 155 } |  |  | 0.625 |
| walker |  | 2309 | 32 | Code::CodeKey { rung: Doc, file: src/node.js, decl: 6, sub: 0, line: 193 } |  |  | 0.625 |
| walker |  | 2346 | 37 | Code::CodeKey { rung: Doc, file: src/node.js, decl: 5, sub: 0, line: 167 } |  |  | 0.625 |
| walker |  | 2392 | 46 | Code::CodeKey { rung: Doc, file: src/node.js, decl: 7, sub: 0, line: 203 } |  |  | 0.625 |
| ns | 2438 |  | 221 | README: the canonical usage example | 3.5 |  | 0.594 |
| walker |  | 2443 | 51 | Code::CodeKey { rung: Doc, file: src/node.js, decl: 8, sub: 0, line: 220 } |  |  | 0.594 |
| walker |  | 2502 | 59 | Code::CodeKey { rung: Doc, file: src/node.js, decl: 3, sub: 0, line: 124 } |  |  | 0.594 |
| walker |  | 2566 | 64 | Code::CodeKey { rung: Doc, file: src/node.js, decl: 9, sub: 0, line: 231 } |  |  | 0.594 |
| ns | 2645 |  | 207 | README: namespace colors — when they turn on, per environment | 3.6 |  | 0.579 |
| walker |  | 2649 | 83 | Code::CodeKey { rung: Doc, file: src/browser.js, decl: 6, sub: 0, line: 192 } |  |  | 0.579 |
| walker |  | 2740 | 91 | Code::CodeKey { rung: Doc, file: src/browser.js, decl: 9, sub: 0, line: 247 } |  |  | 0.579 |
| ns | 2855 |  | 210 | README: namespace naming conventions + the `DEBUG_*` → `util.inspect` options note | 3.7 |  | 0.575 |
| walker |  | 2867 | 127 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.588 |
| ns | 2949 |  | 94 | README: checking and forcing `debug.enabled` | 3.8 |  | 0.574 |
| ns | 3075 |  | 126 | README: `log.extend()` for sub-namespaces | 3.9 |  | 0.558 |
| walker |  | 3123 | 256 | Code::CodeKey { rung: Decl, file: src/browser.js, decl: 3, sub: 0, line: 27 } |  |  | 0.558 |
| walker |  | 3138 | 15 | Code::CodeKey { rung: Doc, file: src/browser.js, decl: 3, sub: 0, line: 27 } |  |  | 0.558 |
| ns | 3200 |  | 125 | README: enabling debug dynamically via `enable()` / `disable()` | 3.10 |  | 0.541 |
| ns | 3380 |  | 180 | README: `enable(namespaces)` / `disable()` contract and the round-trip caveat | 3.11 |  | 0.524 |
| walker |  | 3397 | 259 | Code::CodeKey { rung: Decl, file: src/browser.js, decl: 3, sub: 1, line: 27 } |  |  | 0.524 |
| ns | 3570 |  | 190 | README: adding a custom formatter | 3.12 |  | 0.544 |
| walker |  | 3643 | 246 | Code::CodeKey { rung: Decl, file: src/browser.js, decl: 3, sub: 2, line: 27 } |  |  | 0.544 |
| ns | 3742 |  | 172 | README: redirecting output by overriding `log` | 3.13 |  | 0.529 |
| walker |  | 3769 | 126 | Markdown::Section { file: README.md, section_index: 13, keeps_default_concavity: false } |  |  | 0.558 |
| walker |  | 3898 | 129 | Markdown::Section { file: README.md, section_index: 15, keeps_default_concavity: false } |  |  | 0.581 |
| ns | 3973 |  | 231 | README: browser build, `localStorage.debug`, and the Chromium Verbose caveat | 3.14 |  | 0.568 |
| walker |  | 4065 | 167 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.591 |
| ns | 4066 |  | 93 | README: setting `DEBUG` on Windows (CMD and PowerShell) | 3.15 |  | 0.581 |
| ns | 4166 |  | 100 | README: the millisecond diff feature | 3.16 |  | 0.579 |
| walker |  | 4336 | 271 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.618 |
| ns | 4361 |  | 195 | README: colors in child processes (`DEBUG_COLORS=1`) | 3.17 |  | 0.602 |
| ns | 4476 |  | 115 | package.json: runtime dependency, optional peer dependency, and the xo lint override | 4.1 |  | 0.591 |
| walker |  | 4564 | 228 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: true } |  |  | 0.591 |
| walker |  | 4781 | 217 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.595 |
| ns | 4792 |  | 316 | test.js: header plus every `describe`/`it` title | 4.2 |  | 0.578 |
| walker |  | 4804 | 23 | Code::CodeKey { rung: Body, file: src/node.js, decl: 6, sub: 0, line: 193 } |  |  | 0.578 |
| ns | 4970 |  | 178 | test.node.js: the node-only suite and its sinon harness | 4.3 |  | 0.563 |
| ns | 5114 |  | 144 | karma.conf.js: frameworks and the browser test file set | 4.4 |  | 0.554 |
| ns | 5246 |  | 132 | .travis.yml in full — the CI matrix | 4.5 |  | 0.541 |
| walker |  | 5441 | 637 | Code::CodeKey { rung: Body, file: karma.conf.js, decl: 1, sub: 0, line: 1 } |  |  | 0.564 |
| ns | 5533 |  | 287 | karma.conf.js: launcher, preprocessors and run mode | 4.6 | 4.4 | 0.582 |
| walker |  | 5670 | 229 | Markdown::Section { file: README.md, section_index: 16, keeps_default_concavity: false } |  |  | 0.605 |
| ns | 5732 |  | 199 | src/common.js: `enable()` body — parsing the namespace string | 5.1 | 2.3 | 0.591 |
| ns | 5950 |  | 218 | src/common.js: `debug(...args)` — enabled guard, ms-diff bookkeeping, `%O` coercion | 5.2 | 2.3 | 0.576 |
| walker |  | 6040 | 370 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.592 |
| ns | 6264 |  | 314 | src/common.js: `debug(...args)` — the `%`-formatter substitution loop and log dispatch | 5.3 | 5.2 | 0.575 |
| walker |  | 6306 | 266 | Markdown::Section { file: README.md, section_index: 12, keeps_default_concavity: false } |  |  | 0.596 |
| ns | 6598 |  | 334 | src/common.js: per-instance properties and the `enabled` getter/setter | 5.4 | 2.3 | 0.576 |
| walker |  | 6816 | 510 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.596 |
| walker |  | 6855 | 39 | Code::CodeKey { rung: Body, file: src/node.js, decl: 4, sub: 0, line: 155 } |  |  | 0.596 |
| ns | 6990 |  | 392 | src/common.js: `matchesTemplate()` — the wildcard matcher | 5.5 | 2.3 | 0.577 |
| ns | 7187 |  | 197 | src/common.js: `disable()` and `enabled()` bodies | 5.6 | 2.3 | 0.565 |
| walker |  | 7205 | 350 | Markdown::Section { file: README.md, section_index: 20, keeps_default_concavity: false } |  |  | 0.565 |
| ns | 7436 |  | 249 | src/common.js: `selectColor()` and `extend()` bodies | 5.7 | 2.3 | 0.556 |
| ns | 7571 |  | 135 | src/common.js: `coerce()` and the deprecated `destroy()` stub | 5.8 | 2.3 | 0.550 |
| walker |  | 7607 | 402 | Markdown::Section { file: README.md, section_index: 11, keeps_default_concavity: false } |  |  | 0.565 |
| ns | 7647 |  | 76 | src/common.js: adapter-property spread and `createDebug`'s closure state | 5.9 | 2.1 | 0.561 |
| ns | 7942 |  | 295 | src/node.js: `inspectOpts` derivation from `DEBUG_*` environment variables | 6.1 | 2.6 | 0.549 |
| walker |  | 8084 | 477 | Markdown::Section { file: README.md, section_index: 14, keeps_default_concavity: false } |  |  | 0.583 |
| walker |  | 8190 | 106 | Code::CodeKey { rung: Body, file: src/browser.js, decl: 7, sub: 0, line: 200 } |  |  | 0.583 |
| ns | 8206 |  | 264 | src/browser.js: `useColors()` — the inspector capability sniff | 6.2 | 2.7 | 0.578 |
| walker |  | 8263 | 73 | Code::CodeKey { rung: Body, file: src/node.js, decl: 9, sub: 0, line: 231 } |  |  | 0.578 |
| walker |  | 8372 | 109 | Code::CodeKey { rung: Body, file: src/browser.js, decl: 9, sub: 0, line: 247 } |  |  | 0.578 |
| ns | 8528 |  | 322 | src/node.js: `useColors`, `formatArgs`, `getDate`, `log` bodies | 6.3 | 2.6 | 0.568 |
| walker |  | 8641 | 269 | Code::CodeKey { rung: Body, file: src/common.js, decl: 1, sub: 0, line: 7 } |  |  | 0.611 |
| ns | 8861 |  | 333 | src/browser.js: `formatArgs()` — `%c` CSS injection | 6.4 | 2.7 | 0.597 |
| ns | 9045 |  | 184 | src/node.js: `save`, `load`, `init` bodies | 6.5 | 2.6 | 0.593 |
| ns | 9397 |  | 352 | src/browser.js: `save`, `load`, `localstorage` bodies | 6.6 | 2.7 | 0.586 |
| ns | 9541 |  | 144 | Formatter implementations: node `%o`/`%O` and browser `%j` | 6.7 | 2.6 | 0.580 |
| ns | 9722 |  | 181 | Both color palettes, head and tail, with the elision marked | 6.8 | 2.7 | 0.574 |
| walker |  | 9864 | 1223 | Code::CodeKey { rung: Body, file: src/common.js, decl: 1, sub: 1, line: 7 } |  |  | 0.662 |
| ns | 9932 |  | 210 | LICENSE header, plus .gitignore in full and the .editorconfig head | 7.1 |  | 0.652 |

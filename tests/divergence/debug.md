Score(3000)=0.629 I=0.833 C=0.475 ns_rows≤3K=21/54 grid(1000/1442/2080/3000/4327/6240/9000)=0.699/0.574/0.551/0.629/0.661/0.658/0.698

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 31 | 31 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 45 |  | 45 | Repo identity: README title + one-sentence description | 1.1 |  | 0.000 |
| walker |  | 76 | 45 | Markdown::ReadmeHeadline { file: README.md } |  |  | 1.000 |
| walker |  | 92 | 16 | Fs::DirListing { dir: src } |  |  | 1.000 |
| ns | 92 |  | 47 | Complete file map: repo root and src/ | 1.2 |  | 1.000 |
| walker |  | 157 | 65 | Json::Identity { file: package.json } |  |  | 1.000 |
| walker |  | 189 | 32 | Json::Dependencies { file: package.json } |  |  | 1.000 |
| ns | 205 |  | 113 | src/index.js in full — the environment dispatch | 1.3 |  | 0.755 |
| walker |  | 221 | 32 | Json::Runtime { file: package.json } |  |  | 0.764 |
| walker |  | 266 | 45 | Json::Scripts { file: package.json } |  |  | 0.767 |
| walker |  | 327 | 61 | Json::ScriptsTail { file: package.json } |  |  | 0.782 |
| ns | 328 |  | 123 | package.json identity, entry points, engines, license | 1.4 |  | 0.748 |
| walker |  | 395 | 68 | Json::Entry { file: package.json } |  |  | 0.821 |
| ns | 436 |  | 108 | package.json `scripts` — every build/test/lint entry point | 1.5 |  | 0.829 |
| walker |  | 586 | 191 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.866 |
| walker |  | 611 | 25 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.866 |
| ns | 616 |  | 180 | README section map — every `##` heading | 1.6 |  | 0.870 |
| ns | 765 |  | 149 | src/common.js — `setup(env)` and the complete `createDebug.*` public API attachment block | 2.1 |  | 0.762 |
| walker |  | 872 | 261 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: true } |  |  | 0.768 |
| walker |  | 886 | 14 | Code::CodeKey { rung: Names, file: karma.conf.js, decl: 0, sub: 0, line: 0 } |  |  | 0.768 |
| ns | 906 |  | 141 | src/common.js — module-level state: `names`, `skips`, `formatters` | 2.2 |  | 0.698 |
| walker |  | 991 | 105 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.699 |
| walker |  | 1013 | 22 | Code::CodeKey { rung: Names, file: src/common.js, decl: 0, sub: 0, line: 0 } |  |  | 0.699 |
| ns | 1061 |  | 155 | src/common.js — roster of every function definition plus the module tail | 2.3 |  | 0.645 |
| walker |  | 1176 | 163 | Code::CodeKey { rung: Names, file: src/browser.js, decl: 0, sub: 0, line: 0 } |  |  | 0.650 |
| ns | 1229 |  | 168 | src/node.js — the node adapter's export contract | 2.4 |  | 0.582 |
| walker |  | 1274 | 98 | Code::CodeKey { rung: Decl, file: src/browser.js, decl: 2, sub: 0, line: 12 } |  |  | 0.582 |
| ns | 1343 |  | 114 | src/browser.js — the browser adapter's export contract | 2.5 |  | 0.574 |
| walker |  | 1473 | 199 | Code::CodeKey { rung: Decl, file: src/browser.js, decl: 3, sub: 0, line: 27 } |  |  | 0.574 |
| ns | 1517 |  | 174 | src/node.js — roster of every function, export and formatter | 2.6 |  | 0.544 |
| ns | 1644 |  | 127 | src/browser.js — roster of every function, export and formatter | 2.7 |  | 0.552 |
| walker |  | 1682 | 209 | Code::CodeKey { rung: Decl, file: src/browser.js, decl: 3, sub: 1, line: 27 } |  |  | 0.552 |
| ns | 1802 |  | 158 | README: the complete `DEBUG_*` environment variable table | 3.1 |  | 0.574 |
| walker |  | 1890 | 208 | Code::CodeKey { rung: Decl, file: src/browser.js, decl: 3, sub: 2, line: 27 } |  |  | 0.574 |
| ns | 1988 |  | 186 | README: the complete `%` formatter table | 3.2 |  | 0.551 |
| walker |  | 2035 | 145 | Code::CodeKey { rung: Decl, file: src/browser.js, decl: 3, sub: 3, line: 27 } |  |  | 0.551 |
| walker |  | 2150 | 115 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.552 |
| ns | 2155 |  | 167 | README: wildcard and exclusion syntax for `DEBUG` | 3.3 |  | 0.535 |
| ns | 2217 |  | 62 | README: every sub-`##` heading (`###`/`####`/`#####`) | 3.4 |  | 0.526 |
| walker |  | 2353 | 203 | Code::CodeKey { rung: Names, file: src/node.js, decl: 0, sub: 0, line: 0 } |  |  | 0.565 |
| walker |  | 2378 | 25 | Code::CodeKey { rung: Decl, file: src/node.js, decl: 3, sub: 0, line: 124 } |  |  | 0.565 |
| walker |  | 2425 | 47 | Code::CodeKey { rung: Decl, file: src/node.js, decl: 1, sub: 0, line: 18 } |  |  | 0.573 |
| ns | 2438 |  | 221 | README: the canonical usage example | 3.5 |  | 0.545 |
| walker |  | 2440 | 15 | Code::CodeKey { rung: Doc, file: src/browser.js, decl: 3, sub: 0, line: 27 } |  |  | 0.545 |
| walker |  | 2455 | 15 | Code::CodeKey { rung: Doc, file: src/node.js, decl: 2, sub: 0, line: 27 } |  |  | 0.545 |
| walker |  | 2490 | 35 | Code::CodeKey { rung: Names, file: src/index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.547 |
| walker |  | 2524 | 34 | Code::CodeKey { rung: Decl, file: src/index.js, decl: 1, sub: 0, line: 6 } |  |  | 0.563 |
| walker |  | 2562 | 38 | Code::CodeKey { rung: Doc, file: src/common.js, decl: 1, sub: 0, line: 7 } |  |  | 0.570 |
| walker |  | 2601 | 39 | Code::CodeKey { rung: Doc, file: src/index.js, decl: 1, sub: 0, line: 6 } |  |  | 0.604 |
| walker |  | 2610 | 9 | Code::CodeKey { rung: Body, file: src/node.js, decl: 8, sub: 0, line: 220 } |  |  | 0.604 |
| ns | 2645 |  | 207 | README: namespace colors — when they turn on, per environment | 3.6 |  | 0.589 |
| walker |  | 2736 | 126 | Markdown::Section { file: README.md, section_index: 13, keeps_default_concavity: false } |  |  | 0.592 |
| ns | 2843 |  | 198 | README: namespace naming conventions + the `DEBUG_*` → `util.inspect` options note | 3.7 |  | 0.601 |
| walker |  | 2865 | 129 | Markdown::Section { file: README.md, section_index: 15, keeps_default_concavity: false } |  |  | 0.603 |
| ns | 2937 |  | 94 | README: checking and forcing `debug.enabled` | 3.8 |  | 0.617 |
| walker |  | 3032 | 167 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.648 |
| ns | 3063 |  | 126 | README: `log.extend()` for sub-namespaces | 3.9 |  | 0.660 |
| ns | 3188 |  | 125 | README: enabling debug dynamically via `enable()` / `disable()` | 3.10 |  | 0.640 |
| walker |  | 3303 | 271 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.687 |
| ns | 3368 |  | 180 | README: `enable(namespaces)` / `disable()` contract and the round-trip caveat | 3.11 |  | 0.666 |
| walker |  | 3531 | 228 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.666 |
| ns | 3558 |  | 190 | README: adding a custom formatter | 3.12 |  | 0.646 |
| ns | 3730 |  | 172 | README: redirecting output by overriding `log` | 3.13 |  | 0.628 |
| walker |  | 3778 | 247 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.654 |
| walker |  | 3807 | 29 | Code::CodeKey { rung: Doc, file: src/node.js, decl: 4, sub: 0, line: 155 } |  |  | 0.654 |
| ns | 3961 |  | 231 | README: browser build, `localStorage.debug`, and the Chromium Verbose caveat | 3.14 |  | 0.639 |
| ns | 4054 |  | 93 | README: setting `DEBUG` on Windows (CMD and PowerShell) | 3.15 |  | 0.628 |
| walker |  | 4106 | 299 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.659 |
| walker |  | 4138 | 32 | Code::CodeKey { rung: Doc, file: src/node.js, decl: 6, sub: 0, line: 193 } |  |  | 0.659 |
| ns | 4154 |  | 100 | README: the millisecond diff feature | 3.16 |  | 0.661 |
| walker |  | 4174 | 36 | Code::CodeKey { rung: Doc, file: src/browser.js, decl: 5, sub: 0, line: 149 } |  |  | 0.661 |
| walker |  | 4211 | 37 | Code::CodeKey { rung: Doc, file: src/node.js, decl: 5, sub: 0, line: 167 } |  |  | 0.661 |
| walker |  | 4255 | 44 | Code::CodeKey { rung: Doc, file: src/browser.js, decl: 7, sub: 0, line: 200 } |  |  | 0.661 |
| walker |  | 4301 | 46 | Code::CodeKey { rung: Doc, file: src/node.js, decl: 7, sub: 0, line: 203 } |  |  | 0.661 |
| ns | 4349 |  | 195 | README: colors in child processes (`DEBUG_COLORS=1`) | 3.17 |  | 0.644 |
| ns | 4464 |  | 115 | package.json: runtime dependency, optional peer dependency, and the xo lint override | 4.1 |  | 0.632 |
| walker |  | 4530 | 229 | Markdown::Section { file: README.md, section_index: 16, keeps_default_concavity: false } |  |  | 0.661 |
| ns | 4780 |  | 316 | test.js: header plus every `describe`/`it` title | 4.2 |  | 0.642 |
| walker |  | 4796 | 266 | Markdown::Section { file: README.md, section_index: 12, keeps_default_concavity: false } |  |  | 0.669 |
| walker |  | 4847 | 51 | Code::CodeKey { rung: Doc, file: src/browser.js, decl: 8, sub: 0, line: 219 } |  |  | 0.669 |
| walker |  | 4898 | 51 | Code::CodeKey { rung: Doc, file: src/node.js, decl: 8, sub: 0, line: 220 } |  |  | 0.669 |
| walker |  | 4957 | 59 | Code::CodeKey { rung: Doc, file: src/node.js, decl: 3, sub: 0, line: 124 } |  |  | 0.669 |
| ns | 4958 |  | 178 | test.node.js: the node-only suite and its sinon harness | 4.3 |  | 0.652 |
| walker |  | 5021 | 64 | Code::CodeKey { rung: Doc, file: src/node.js, decl: 9, sub: 0, line: 231 } |  |  | 0.652 |
| ns | 5102 |  | 144 | karma.conf.js: frameworks and the browser test file set | 4.4 |  | 0.641 |
| ns | 5234 |  | 132 | .travis.yml in full — the CI matrix | 4.5 |  | 0.626 |
| walker |  | 5371 | 350 | Markdown::Section { file: README.md, section_index: 11, keeps_default_concavity: false } |  |  | 0.648 |
| ns | 5521 |  | 287 | karma.conf.js: launcher, preprocessors and run mode | 4.6 | 4.4 | 0.626 |
| ns | 5720 |  | 199 | src/common.js: `enable()` body — parsing the namespace string | 5.1 | 2.3 | 0.611 |
| walker |  | 5760 | 389 | Markdown::Section { file: README.md, section_index: 10, keeps_default_concavity: false } |  |  | 0.654 |
| walker |  | 5843 | 83 | Code::CodeKey { rung: Doc, file: src/browser.js, decl: 6, sub: 0, line: 192 } |  |  | 0.654 |
| walker |  | 5866 | 23 | Code::CodeKey { rung: Body, file: src/node.js, decl: 6, sub: 0, line: 193 } |  |  | 0.654 |
| ns | 5938 |  | 218 | src/common.js: `debug(...args)` — enabled guard, ms-diff bookkeeping, `%O` coercion | 5.2 | 2.3 | 0.637 |
| walker |  | 5957 | 91 | Code::CodeKey { rung: Doc, file: src/browser.js, decl: 9, sub: 0, line: 247 } |  |  | 0.637 |
| walker |  | 6058 | 101 | Code::CodeKey { rung: Doc, file: src/browser.js, decl: 4, sub: 0, line: 115 } |  |  | 0.637 |
| ns | 6252 |  | 314 | src/common.js: `debug(...args)` — the `%`-formatter substitution loop and log dispatch | 5.3 | 5.2 | 0.619 |
| walker |  | 6535 | 477 | Markdown::Section { file: README.md, section_index: 14, keeps_default_concavity: false } |  |  | 0.660 |
| walker |  | 6574 | 39 | Code::CodeKey { rung: Body, file: src/node.js, decl: 4, sub: 0, line: 155 } |  |  | 0.660 |
| ns | 6586 |  | 334 | src/common.js: per-instance properties and the `enabled` getter/setter | 5.4 | 2.3 | 0.638 |
| walker |  | 6762 | 188 | Code::CodeKey { rung: Body, file: karma.conf.js, decl: 1, sub: 0, line: 1 } |  |  | 0.653 |
| walker |  | 6835 | 73 | Code::CodeKey { rung: Body, file: src/node.js, decl: 9, sub: 0, line: 231 } |  |  | 0.653 |
| walker |  | 6924 | 89 | Code::CodeKey { rung: Body, file: src/node.js, decl: 7, sub: 0, line: 203 } |  |  | 0.654 |
| ns | 6978 |  | 392 | src/common.js: `matchesTemplate()` — the wildcard matcher | 5.5 | 2.3 | 0.633 |
| walker |  | 7121 | 197 | Code::CodeKey { rung: Body, file: karma.conf.js, decl: 1, sub: 1, line: 1 } |  |  | 0.638 |
| ns | 7175 |  | 197 | src/common.js: `disable()` and `enabled()` bodies | 5.6 | 2.3 | 0.626 |
| walker |  | 7373 | 252 | Code::CodeKey { rung: Body, file: karma.conf.js, decl: 1, sub: 2, line: 1 } |  |  | 0.651 |
| ns | 7424 |  | 249 | src/common.js: `selectColor()` and `extend()` bodies | 5.7 | 2.3 | 0.639 |
| ns | 7559 |  | 135 | src/common.js: `coerce()` and the deprecated `destroy()` stub | 5.8 | 2.3 | 0.633 |
| walker |  | 7575 | 202 | Code::CodeKey { rung: Body, file: src/node.js, decl: 5, sub: 0, line: 167 } |  |  | 0.634 |
| ns | 7635 |  | 76 | src/common.js: adapter-property spread and `createDebug`'s closure state | 5.9 | 2.1 | 0.629 |
| walker |  | 7681 | 106 | Code::CodeKey { rung: Body, file: src/browser.js, decl: 7, sub: 0, line: 200 } |  |  | 0.629 |
| ns | 7930 |  | 295 | src/node.js: `inspectOpts` derivation from `DEBUG_*` environment variables | 6.1 | 2.6 | 0.616 |
| walker |  | 7950 | 269 | Code::CodeKey { rung: Body, file: src/common.js, decl: 1, sub: 0, line: 7 } |  |  | 0.660 |
| walker |  | 8059 | 109 | Code::CodeKey { rung: Body, file: src/browser.js, decl: 9, sub: 0, line: 247 } |  |  | 0.660 |
| ns | 8194 |  | 264 | src/browser.js: `useColors()` — the inspector capability sniff | 6.2 | 2.7 | 0.654 |
| walker |  | 8296 | 237 | Code::CodeKey { rung: Body, file: src/node.js, decl: 3, sub: 0, line: 124 } |  |  | 0.679 |
| walker |  | 8503 | 207 | Code::CodeKey { rung: Body, file: src/common.js, decl: 1, sub: 1, line: 7 } |  |  | 0.690 |
| ns | 8516 |  | 322 | src/node.js: `useColors`, `formatArgs`, `getDate`, `log` bodies | 6.3 | 2.6 | 0.689 |
| walker |  | 8664 | 161 | Code::CodeKey { rung: Body, file: src/browser.js, decl: 8, sub: 0, line: 219 } |  |  | 0.691 |
| ns | 8849 |  | 333 | src/browser.js: `formatArgs()` — `%c` CSS injection | 6.4 | 2.7 | 0.675 |
| walker |  | 8891 | 227 | Code::CodeKey { rung: Body, file: src/common.js, decl: 1, sub: 2, line: 7 } |  |  | 0.687 |
| ns | 9033 |  | 184 | src/node.js: `save`, `load`, `init` bodies | 6.5 | 2.6 | 0.690 |
| walker |  | 9089 | 198 | Code::CodeKey { rung: Body, file: src/common.js, decl: 1, sub: 3, line: 7 } |  |  | 0.708 |
| ns | 9385 |  | 352 | src/browser.js: `save`, `load`, `localstorage` bodies | 6.6 | 2.7 | 0.715 |
| walker |  | 9479 | 390 | Code::CodeKey { rung: Body, file: src/browser.js, decl: 5, sub: 0, line: 149 } |  |  | 0.738 |
| ns | 9529 |  | 144 | Formatter implementations: node `%o`/`%O` and browser `%j` | 6.7 | 2.6 | 0.730 |
| walker |  | 9680 | 201 | Code::CodeKey { rung: Body, file: src/common.js, decl: 1, sub: 4, line: 7 } |  |  | 0.744 |
| ns | 9710 |  | 181 | Both color palettes, head and tail, with the elision marked | 6.8 | 2.7 | 0.736 |
| walker |  | 9875 | 195 | Code::CodeKey { rung: Body, file: src/common.js, decl: 1, sub: 5, line: 7 } |  |  | 0.747 |
| ns | 9920 |  | 210 | LICENSE header, plus .gitignore in full and the .editorconfig head | 7.1 |  | 0.735 |
| walker |  | 9972 | 97 | Code::CodeKey { rung: Body, file: src/browser.js, decl: 4, sub: 0, line: 115 } |  |  | 0.735 |

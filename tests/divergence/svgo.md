Score(3000)=0.649 I=0.850 C=0.496 ns_rows≤3K=14/54 grid(1000/1442/2080/3000/4327/6240/9000)=0.736/0.570/0.694/0.649/0.520/0.539/0.535

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 86 | 86 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 89 |  | 89 | SVGO identity and rationale (README lede) | 1.1 |  | 0.000 |
| walker |  | 91 | 5 | Fs::DirListing { dir: bin } |  |  | 0.000 |
| ns | 175 |  | 86 | Complete root directory listing | 1.2 |  | 0.754 |
| walker |  | 225 | 134 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.802 |
| walker |  | 234 | 9 | Fs::DirListing { dir: test-d/lib } |  |  | 0.802 |
| walker |  | 259 | 25 | Fs::DirListing { dir: logo } |  |  | 0.803 |
| walker |  | 264 | 5 | Fs::DirListing { dir: scripts } |  |  | 0.803 |
| ns | 297 |  | 122 | Complete lib/ engine listing (with lib/svgo/ and lib/util/) | 1.3 |  | 0.536 |
| walker |  | 307 | 43 | Fs::DirListing { dir: docs } |  |  | 0.536 |
| walker |  | 313 | 6 | Fs::DirListing { dir: .yarn } |  |  | 0.536 |
| walker |  | 332 | 19 | Fs::DirListing { dir: docs/02-usage } |  |  | 0.537 |
| ns | 386 |  | 89 | optimize() -- the core API, jsdoc and signature | 1.4 |  | 0.492 |
| walker |  | 410 | 78 | Json::Identity { file: package.json } |  |  | 0.493 |
| walker |  | 444 | 34 | Fs::DirListing { dir: docs/06-migrations } |  |  | 0.493 |
| walker |  | 456 | 12 | Fs::DirListing { dir: .github } |  |  | 0.493 |
| walker |  | 478 | 22 | Fs::DirListing { dir: .github/workflows } |  |  | 0.493 |
| ns | 496 |  | 110 | README section heading roster | 1.5 |  | 0.442 |
| walker |  | 568 | 90 | Fs::DirListing { dir: lib } |  |  | 0.594 |
| walker |  | 581 | 13 | Fs::DirListing { dir: lib/util } |  |  | 0.625 |
| walker |  | 600 | 19 | Fs::DirListing { dir: lib/svgo } |  |  | 0.694 |
| walker |  | 649 | 49 | Json::Runtime { file: package.json } |  |  | 0.695 |
| ns | 715 |  | 219 | lib/svgo.js module graph and public re-exports | 1.6 |  | 0.608 |
| walker |  | 804 | 155 | Markdown::CommandBlock { file: CONTRIBUTING.md, row: 30 } |  |  | 0.608 |
| walker |  | 816 | 12 | Fs::DirListing { dir: .yarn/plugins/@yarnpkg } |  |  | 0.608 |
| ns | 875 |  | 160 | Command-line usage (README) | 1.7 |  | 0.548 |
| walker |  | 922 | 106 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.626 |
| walker |  | 977 | 55 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.735 |
| walker |  | 1011 | 34 | Fs::DirListing { dir: test } |  |  | 0.736 |
| walker |  | 1021 | 10 | Fs::DirListing { dir: test/svg2js } |  |  | 0.736 |
| walker |  | 1034 | 13 | Fs::DirListing { dir: test/cli } |  |  | 0.736 |
| walker |  | 1120 | 86 | Json::Scripts { file: package.json } |  |  | 0.736 |
| ns | 1231 |  | 356 | Complete plugins/ listing (all 58 modules) | 1.8 |  | 0.570 |
| walker |  | 1404 | 284 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: true } |  |  | 0.570 |
| walker |  | 1423 | 19 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.570 |
| ns | 1470 |  | 239 | svgo.config.mjs shape (README Configuration) | 1.9 |  | 0.526 |
| walker |  | 1516 | 93 | Code::CodeKey { rung: Names, file: lib/svgo.js, decl: 0, sub: 0, line: 0 } |  |  | 0.537 |
| ns | 1669 |  | 199 | optimize() body, part 1: multipass loop and plugin resolution | 1.10 | 1.4 | 0.512 |
| walker |  | 1872 | 356 | Fs::DirListing { dir: plugins } |  |  | 0.732 |
| ns | 1873 |  | 204 | optimize() body, part 2: overrides, invoke, stringify, datauri | 1.11 | 1.10 | 0.693 |
| walker |  | 1929 | 57 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.693 |
| walker |  | 1958 | 29 | Fs::DirListing { dir: test/coa } |  |  | 0.693 |
| walker |  | 2163 | 205 | Json::Dependencies { file: package.json } |  |  | 0.694 |
| walker |  | 2198 | 35 | Fs::DirListing { dir: test/regression } |  |  | 0.695 |
| ns | 2423 |  | 550 | Complete exported type roster of lib/types.ts | 1.12 |  | 0.635 |
| walker |  | 2425 | 227 | Json::Entry { file: package.json } |  |  | 0.638 |
| walker |  | 2554 | 129 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.638 |
| walker |  | 2605 | 51 | Fs::DirListing { dir: test/fixtures/config-loader } |  |  | 0.638 |
| walker |  | 2611 | 6 | Fs::DirListing { dir: test/fixtures/config-loader/one/two } |  |  | 0.638 |
| walker |  | 2618 | 7 | Fs::DirListing { dir: test/fixtures/config-loader/cjs } |  |  | 0.638 |
| walker |  | 2625 | 7 | Fs::DirListing { dir: test/fixtures/config-loader/mjs } |  |  | 0.638 |
| ns | 2691 |  | 268 | Package identity, entry points and engines | 1.13 |  | 0.658 |
| ns | 2755 |  | 64 | Version constant | 1.14 |  | 0.649 |
| walker |  | 2920 | 295 | Json::ScriptsTail { file: package.json } |  |  | 0.649 |
| ns | 3125 |  | 370 | preset-default: the complete default pipeline in execution order | 2.1 |  | 0.602 |
| ns | 3259 |  | 134 | builtinPlugins registry | 2.2 |  | 0.591 |
| walker |  | 3308 | 388 | Fs::DirListing { dir: docs/04-plugins } |  |  | 0.591 |
| walker |  | 3426 | 118 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.591 |
| ns | 3448 |  | 189 | invokePlugins -- the plugin engine | 2.3 |  | 0.572 |
| walker |  | 3510 | 84 | Fs::DirListing { dir: test/svgo } |  |  | 0.572 |
| ns | 3669 |  | 221 | createPreset -- how a preset validates and forwards overrides | 2.4 |  | 0.555 |
| walker |  | 3712 | 202 | Code::CodeKey { rung: Names, file: lib/types.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.566 |
| walker |  | 3721 | 9 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 5, sub: 0, line: 108 } |  |  | 0.566 |
| walker |  | 3743 | 22 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 12, sub: 0, line: 175 } |  |  | 0.566 |
| walker |  | 3767 | 24 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 2, sub: 0, line: 67 } |  |  | 0.566 |
| walker |  | 3792 | 25 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 13, sub: 0, line: 180 } |  |  | 0.566 |
| walker |  | 3823 | 31 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 6, sub: 0, line: 111 } |  |  | 0.566 |
| walker |  | 3854 | 31 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 11, sub: 0, line: 169 } |  |  | 0.566 |
| walker |  | 3892 | 38 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 9, sub: 0, line: 155 } |  |  | 0.566 |
| walker |  | 3938 | 46 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 10, sub: 0, line: 161 } |  |  | 0.566 |
| ns | 3956 |  | 287 | visit() -- the AST traversal contract | 2.5 |  | 0.545 |
| walker |  | 3991 | 53 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 8, sub: 0, line: 133 } |  |  | 0.545 |
| walker |  | 4074 | 83 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 4, sub: 0, line: 100 } |  |  | 0.545 |
| ns | 4139 |  | 183 | Anatomy of a plugin module (removeComments as exemplar) | 2.6 |  | 0.533 |
| walker |  | 4247 | 173 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 1, sub: 0, line: 30 } |  |  | 0.533 |
| ns | 4291 |  | 152 | Overriding and disabling preset-default plugins (README) | 2.7 |  | 0.520 |
| ns | 4408 |  | 117 | Custom plugin objects (docs/05-plugins-api.mdx) | 2.8 |  | 0.515 |
| walker |  | 4513 | 266 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 1, sub: 1, line: 30 } |  |  | 0.515 |
| ns | 4603 |  | 195 | Node entry point: loadConfig and the Node optimize wrapper | 2.9 |  | 0.504 |
| walker |  | 4669 | 156 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 7, sub: 0, line: 117 } |  |  | 0.504 |
| ns | 4786 |  | 183 | Plugin descriptions: preset-default steps 1-9 | 3.1 |  | 0.498 |
| walker |  | 4959 | 290 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 3, sub: 0, line: 71 } |  |  | 0.498 |
| ns | 5033 |  | 247 | Plugin descriptions: preset-default steps 10-18 | 3.2 |  | 0.489 |
| walker |  | 5124 | 165 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.525 |
| ns | 5238 |  | 205 | Plugin descriptions: preset-default steps 19-26 | 3.3 |  | 0.518 |
| ns | 5405 |  | 167 | Plugin descriptions: preset-default steps 27-34 | 3.4 |  | 0.512 |
| walker |  | 5477 | 353 | Code::CodeKey { rung: Names, file: lib/types.ts, decl: 0, sub: 1, line: 0 } |  |  | 0.564 |
| walker |  | 5489 | 12 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 37, sub: 0, line: 357 } |  |  | 0.564 |
| walker |  | 5511 | 22 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 14, sub: 0, line: 185 } |  |  | 0.564 |
| walker |  | 5533 | 22 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 31, sub: 0, line: 300 } |  |  | 0.564 |
| walker |  | 5556 | 23 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 24, sub: 0, line: 263 } |  |  | 0.564 |
| walker |  | 5579 | 23 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 34, sub: 0, line: 329 } |  |  | 0.564 |
| walker |  | 5603 | 24 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 17, sub: 0, line: 205 } |  |  | 0.564 |
| walker |  | 5633 | 30 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 27, sub: 0, line: 276 } |  |  | 0.564 |
| walker |  | 5663 | 30 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 29, sub: 0, line: 289 } |  |  | 0.564 |
| walker |  | 5694 | 31 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 30, sub: 0, line: 294 } |  |  | 0.564 |
| walker |  | 5734 | 40 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 25, sub: 0, line: 268 } |  |  | 0.564 |
| walker |  | 5776 | 42 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 28, sub: 0, line: 282 } |  |  | 0.564 |
| ns | 5778 |  | 373 | Plugin descriptions: opt-in plugins with optional params | 3.5 |  | 0.552 |
| walker |  | 5821 | 45 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 15, sub: 0, line: 190 } |  |  | 0.552 |
| walker |  | 5868 | 47 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 22, sub: 0, line: 248 } |  |  | 0.552 |
| ns | 5895 |  | 117 | Plugin descriptions: plugins requiring params | 3.6 |  | 0.549 |
| walker |  | 5917 | 49 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 21, sub: 0, line: 243 } |  |  | 0.549 |
| walker |  | 5977 | 60 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 16, sub: 0, line: 197 } |  |  | 0.550 |
| walker |  | 6072 | 95 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 23, sub: 0, line: 253 } |  |  | 0.551 |
| ns | 6127 |  | 232 | Config and Output type bodies | 4.1 |  | 0.539 |
| walker |  | 6364 | 292 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 20, sub: 0, line: 214 } |  |  | 0.539 |
| ns | 6440 |  | 313 | Xast node type bodies | 4.2 |  | 0.559 |
| walker |  | 6542 | 178 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 33, sub: 0, line: 307 } |  |  | 0.559 |
| ns | 6652 |  | 212 | Visitor keys, VisitorNode, PluginInfo and Plugin bodies | 4.3 |  | 0.571 |
| walker |  | 6762 | 220 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 36, sub: 0, line: 336 } |  |  | 0.596 |
| ns | 6809 |  | 157 | coa.js top-level function roster | 5.1 |  | 0.589 |
| ns | 6933 |  | 124 | CLI program metadata and positional argument | 5.2 |  | 0.585 |
| walker |  | 6996 | 234 | Code::CodeKey { rung: Names, file: plugins/_collections.js, decl: 0, sub: 0, line: 0 } |  |  | 0.586 |
| walker |  | 7060 | 64 | Code::CodeKey { rung: Decl, file: plugins/_collections.js, decl: 14, sub: 0, line: 2387 } |  |  | 0.586 |
| walker |  | 7142 | 82 | Code::CodeKey { rung: Decl, file: plugins/_collections.js, decl: 11, sub: 0, line: 2179 } |  |  | 0.586 |
| ns | 7223 |  | 290 | CLI options, first half | 5.3 |  | 0.574 |
| walker |  | 7243 | 101 | Code::CodeKey { rung: Decl, file: plugins/_collections.js, decl: 9, sub: 0, line: 2111 } |  |  | 0.574 |
| walker |  | 7424 | 181 | Code::CodeKey { rung: Decl, file: plugins/_collections.js, decl: 13, sub: 0, line: 2348 } |  |  | 0.574 |
| ns | 7567 |  | 344 | CLI options, second half | 5.4 |  | 0.562 |
| walker |  | 7600 | 176 | Code::CodeKey { rung: Decl, file: plugins/_collections.js, decl: 12, sub: 0, line: 2194 } |  |  | 0.562 |
| ns | 7675 |  | 108 | bin/svgo.js executable entry | 5.5 |  | 0.557 |
| walker |  | 7758 | 158 | Code::CodeKey { rung: Decl, file: plugins/_collections.js, decl: 6, sub: 0, line: 389 } |  |  | 0.557 |
| ns | 7790 |  | 115 | parseSvg and the sax configuration | 6.1 |  | 0.552 |
| ns | 7913 |  | 123 | stringifier function roster | 6.2 |  | 0.548 |
| walker |  | 7948 | 190 | Code::CodeKey { rung: Decl, file: plugins/_collections.js, decl: 12, sub: 1, line: 2194 } |  |  | 0.548 |
| ns | 7996 |  | 83 | xast query helpers | 6.3 |  | 0.546 |
| walker |  | 8141 | 193 | Code::CodeKey { rung: Decl, file: plugins/_collections.js, decl: 12, sub: 2, line: 2194 } |  |  | 0.546 |
| ns | 8174 |  | 178 | lib/svgo/tools.js -- complete export set | 6.4 |  | 0.541 |
| walker |  | 8321 | 180 | Code::CodeKey { rung: Decl, file: plugins/_collections.js, decl: 12, sub: 3, line: 2194 } |  |  | 0.541 |
| ns | 8330 |  | 156 | lib/style.js -- exported and internal function roster | 6.5 |  | 0.536 |
| ns | 8476 |  | 146 | lib/path.js -- path data parse and stringify roster | 6.6 |  | 0.530 |
| walker |  | 8517 | 196 | Code::CodeKey { rung: Decl, file: plugins/_collections.js, decl: 12, sub: 4, line: 2194 } |  |  | 0.530 |
| walker |  | 8704 | 187 | Code::CodeKey { rung: Decl, file: plugins/_collections.js, decl: 12, sub: 5, line: 2194 } |  |  | 0.530 |
| ns | 8710 |  | 234 | plugins/_collections.js -- complete export roster | 6.7 |  | 0.539 |
| walker |  | 8893 | 189 | Code::CodeKey { rung: Decl, file: plugins/_collections.js, decl: 12, sub: 6, line: 2194 } |  |  | 0.539 |
| ns | 8904 |  | 194 | Shared plugin helper module rosters | 6.8 |  | 0.535 |
| ns | 9017 |  | 113 | css-select adapter | 6.9 |  | 0.531 |
| ns | 9034 |  | 17 | mapNodesToParents | 6.10 |  | 0.531 |
| walker |  | 9079 | 186 | Code::CodeKey { rung: Decl, file: plugins/_collections.js, decl: 12, sub: 7, line: 2194 } |  |  | 0.531 |
| ns | 9268 |  | 234 | package.json scripts -- how to build, test and lint | 7.1 |  | 0.536 |
| walker |  | 9269 | 190 | Code::CodeKey { rung: Decl, file: plugins/_collections.js, decl: 12, sub: 8, line: 2194 } |  |  | 0.536 |
| ns | 9337 |  | 69 | test/ tree listing | 7.2 |  | 0.545 |
| ns | 9452 |  | 115 | The fixture-driven plugin test format | 7.3 |  | 0.543 |
| walker |  | 9455 | 186 | Code::CodeKey { rung: Decl, file: plugins/_collections.js, decl: 12, sub: 9, line: 2194 } |  |  | 0.543 |
| ns | 9582 |  | 130 | docs/ and remaining leaf directory listings | 7.4 |  | 0.552 |
| walker |  | 9642 | 187 | Code::CodeKey { rung: Decl, file: plugins/_collections.js, decl: 12, sub: 10, line: 2194 } |  |  | 0.552 |
| ns | 9717 |  | 135 | Migration guide outlines | 7.5 |  | 0.548 |
| ns | 9841 |  | 124 | TypeScript and lint configuration | 7.6 |  | 0.545 |
| walker |  | 9939 | 297 | Code::CodeKey { rung: Decl, file: plugins/_collections.js, decl: 13, sub: 1, line: 2348 } |  |  | 0.545 |
| ns | 9966 |  | 125 | Runtime dependencies | 7.7 |  | 0.549 |
| walker |  | 9995 | 56 | Code::CodeKey { rung: Decl, file: plugins/_collections.js, decl: 12, sub: 11, line: 2194 } |  |  | 0.549 |

Score(3000)=0.644 I=0.848 C=0.489 ns_rows≤3K=14/54 grid(1000/1442/2080/3000/4327/6240/9000)=0.627/0.714/0.687/0.644/0.529/0.496/0.545

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 86 | 86 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 89 | 3 | Fs::DirListing { dir: test-d } |  |  | 0.000 |
| ns | 89 |  | 89 | SVGO identity and rationale (README lede) | 1.1 |  | 0.000 |
| walker |  | 94 | 5 | Fs::DirListing { dir: bin } |  |  | 0.000 |
| walker |  | 102 | 8 | Fs::DirListing { dir: test-d/lib } |  |  | 0.000 |
| walker |  | 127 | 25 | Fs::DirListing { dir: logo } |  |  | 0.000 |
| walker |  | 132 | 5 | Fs::DirListing { dir: scripts } |  |  | 0.000 |
| walker |  | 175 | 43 | Fs::DirListing { dir: docs } |  |  | 0.756 |
| ns | 175 |  | 86 | Complete root directory listing | 1.2 |  | 0.756 |
| walker |  | 181 | 6 | Fs::DirListing { dir: .yarn } |  |  | 0.756 |
| walker |  | 200 | 19 | Fs::DirListing { dir: docs/02-usage } |  |  | 0.757 |
| walker |  | 278 | 78 | Json::Identity { file: package.json } |  |  | 0.758 |
| ns | 297 |  | 122 | Complete lib/ engine listing (with lib/svgo/ and lib/util/) | 1.3 |  | 0.506 |
| walker |  | 312 | 34 | Fs::DirListing { dir: docs/06-migrations } |  |  | 0.506 |
| walker |  | 324 | 12 | Fs::DirListing { dir: .github } |  |  | 0.506 |
| walker |  | 346 | 22 | Fs::DirListing { dir: .github/workflows } |  |  | 0.506 |
| ns | 386 |  | 89 | optimize() -- the core API, jsdoc and signature | 1.4 |  | 0.464 |
| walker |  | 436 | 90 | Fs::DirListing { dir: lib } |  |  | 0.629 |
| walker |  | 455 | 19 | Fs::DirListing { dir: lib/svgo } |  |  | 0.700 |
| walker |  | 468 | 13 | Fs::DirListing { dir: lib/util } |  |  | 0.739 |
| walker |  | 475 | 7 | Code::CodeKey { rung: Names, file: lib/types.js, decl: 0, sub: 0, line: 0 } |  |  | 0.739 |
| ns | 496 |  | 110 | README section heading roster | 1.5 |  | 0.662 |
| walker |  | 516 | 41 | Code::CodeKey { rung: ModuleDoc, file: lib/path.js, decl: 0, sub: 0, line: 0 } |  |  | 0.662 |
| walker |  | 650 | 134 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.694 |
| walker |  | 699 | 49 | Json::Runtime { file: package.json } |  |  | 0.695 |
| walker |  | 715 | 16 | Code::CodeKey { rung: Names, file: lib/builtin.js, decl: 0, sub: 0, line: 0 } |  |  | 0.608 |
| ns | 715 |  | 219 | lib/svgo.js module graph and public re-exports | 1.6 |  | 0.608 |
| walker |  | 732 | 17 | Code::CodeKey { rung: Names, file: lib/version.js, decl: 0, sub: 0, line: 0 } |  |  | 0.608 |
| walker |  | 838 | 106 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.694 |
| walker |  | 872 | 34 | Fs::DirListing { dir: test } |  |  | 0.695 |
| ns | 875 |  | 160 | Command-line usage (README) | 1.7 |  | 0.627 |
| walker |  | 876 | 4 | Fs::DirListing { dir: test/fixtures } |  |  | 0.627 |
| walker |  | 898 | 22 | Code::CodeKey { rung: Names, file: lib/stringifier.js, decl: 0, sub: 0, line: 0 } |  |  | 0.627 |
| walker |  | 917 | 19 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.627 |
| walker |  | 947 | 30 | Code::CodeKey { rung: Names, file: lib/path.js, decl: 0, sub: 0, line: 0 } |  |  | 0.627 |
| walker |  | 979 | 32 | Code::CodeKey { rung: Decl, file: lib/path.js, decl: 2, sub: 0, line: 302 } |  |  | 0.627 |
| ns | 1231 |  | 356 | Complete plugins/ listing (all 58 modules) | 1.8 |  | 0.485 |
| walker |  | 1335 | 356 | Fs::DirListing { dir: plugins } |  |  | 0.714 |
| ns | 1470 |  | 239 | svgo.config.mjs shape (README Configuration) | 1.9 |  | 0.660 |
| walker |  | 1540 | 205 | Json::Dependencies { file: package.json } |  |  | 0.660 |
| walker |  | 1557 | 17 | Code::CodeKey { rung: Names, file: lib/util/map-nodes-to-parents.js, decl: 0, sub: 0, line: 0 } |  |  | 0.660 |
| walker |  | 1612 | 55 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.760 |
| walker |  | 1630 | 18 | Code::CodeKey { rung: Names, file: lib/svgo/css-select-adapter.js, decl: 0, sub: 0, line: 0 } |  |  | 0.760 |
| walker |  | 1665 | 35 | Code::CodeKey { rung: Names, file: lib/parser.js, decl: 0, sub: 0, line: 0 } |  |  | 0.760 |
| ns | 1669 |  | 199 | optimize() body, part 1: multipass loop and plugin resolution | 1.10 | 1.4 | 0.725 |
| walker |  | 1702 | 37 | Code::CodeKey { rung: Decl, file: lib/parser.js, decl: 1, sub: 0, line: 4 } |  |  | 0.725 |
| ns | 1873 |  | 204 | optimize() body, part 2: overrides, invoke, stringify, datauri | 1.11 | 1.10 | 0.686 |
| walker |  | 1986 | 284 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: true } |  |  | 0.686 |
| walker |  | 2021 | 35 | Fs::DirListing { dir: test/regression } |  |  | 0.687 |
| walker |  | 2077 | 56 | Code::CodeKey { rung: Names, file: lib/svgo-node.js, decl: 0, sub: 0, line: 0 } |  |  | 0.687 |
| walker |  | 2108 | 31 | Code::CodeKey { rung: Names, file: lib/svgo/coa.js, decl: 0, sub: 0, line: 0 } |  |  | 0.687 |
| walker |  | 2140 | 32 | Code::CodeKey { rung: Names, file: lib/svgo/plugins.js, decl: 0, sub: 0, line: 0 } |  |  | 0.687 |
| walker |  | 2183 | 43 | Code::CodeKey { rung: Decl, file: lib/svgo/plugins.js, decl: 1, sub: 0, line: 14 } |  |  | 0.688 |
| walker |  | 2216 | 33 | Code::CodeKey { rung: Names, file: lib/util/visit.js, decl: 0, sub: 0, line: 0 } |  |  | 0.688 |
| walker |  | 2252 | 36 | Code::CodeKey { rung: Doc, file: lib/path.js, decl: 2, sub: 0, line: 302 } |  |  | 0.688 |
| walker |  | 2321 | 69 | Code::CodeKey { rung: Names, file: lib/style.js, decl: 0, sub: 0, line: 0 } |  |  | 0.688 |
| walker |  | 2361 | 40 | Code::CodeKey { rung: Decl, file: lib/style.js, decl: 4, sub: 0, line: 286 } |  |  | 0.688 |
| ns | 2423 |  | 550 | Complete exported type roster of lib/types.ts | 1.12 |  | 0.629 |
| walker |  | 2588 | 227 | Json::Entry { file: package.json } |  |  | 0.632 |
| walker |  | 2628 | 40 | Code::CodeKey { rung: Doc, file: lib/path.js, decl: 1, sub: 0, line: 141 } |  |  | 0.632 |
| walker |  | 2677 | 49 | Fs::DirListing { dir: test/fixtures/config-loader } |  |  | 0.632 |
| ns | 2691 |  | 268 | Package identity, entry points and engines | 1.13 |  | 0.652 |
| ns | 2755 |  | 64 | Version constant | 1.14 |  | 0.644 |
| walker |  | 3065 | 388 | Fs::DirListing { dir: docs/04-plugins } |  |  | 0.644 |
| ns | 3125 |  | 370 | preset-default: the complete default pipeline in execution order | 2.1 |  | 0.597 |
| ns | 3259 |  | 134 | builtinPlugins registry | 2.2 |  | 0.587 |
| walker |  | 3338 | 273 | Code::CodeKey { rung: Names, file: lib/types.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.607 |
| walker |  | 3347 | 9 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 5, sub: 0, line: 108 } |  |  | 0.607 |
| walker |  | 3369 | 22 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 12, sub: 0, line: 175 } |  |  | 0.607 |
| walker |  | 3391 | 22 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 14, sub: 0, line: 185 } |  |  | 0.607 |
| walker |  | 3414 | 23 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 13, sub: 0, line: 180 } |  |  | 0.607 |
| walker |  | 3438 | 24 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 2, sub: 0, line: 67 } |  |  | 0.607 |
| ns | 3448 |  | 189 | invokePlugins -- the plugin engine | 2.3 |  | 0.591 |
| walker |  | 3462 | 24 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 17, sub: 0, line: 205 } |  |  | 0.591 |
| walker |  | 3493 | 31 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 6, sub: 0, line: 111 } |  |  | 0.591 |
| walker |  | 3524 | 31 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 11, sub: 0, line: 169 } |  |  | 0.591 |
| walker |  | 3562 | 38 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 9, sub: 0, line: 155 } |  |  | 0.591 |
| walker |  | 3607 | 45 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 15, sub: 0, line: 190 } |  |  | 0.592 |
| walker |  | 3653 | 46 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 10, sub: 0, line: 161 } |  |  | 0.592 |
| ns | 3669 |  | 221 | createPreset -- how a preset validates and forwards overrides | 2.4 |  | 0.574 |
| walker |  | 3706 | 53 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 8, sub: 0, line: 133 } |  |  | 0.574 |
| walker |  | 3766 | 60 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 16, sub: 0, line: 197 } |  |  | 0.575 |
| walker |  | 3849 | 83 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 4, sub: 0, line: 100 } |  |  | 0.575 |
| ns | 3956 |  | 287 | visit() -- the AST traversal contract | 2.5 |  | 0.554 |
| walker |  | 4005 | 156 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 7, sub: 0, line: 117 } |  |  | 0.554 |
| walker |  | 4050 | 45 | Code::CodeKey { rung: Doc, file: lib/style.js, decl: 2, sub: 0, line: 211 } |  |  | 0.554 |
| walker |  | 4107 | 57 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.554 |
| ns | 4139 |  | 183 | Anatomy of a plugin module (removeComments as exemplar) | 2.6 |  | 0.542 |
| walker |  | 4190 | 83 | Code::CodeKey { rung: Names, file: lib/xast.js, decl: 0, sub: 0, line: 0 } |  |  | 0.543 |
| walker |  | 4209 | 19 | Code::CodeKey { rung: Body, file: lib/xast.js, decl: 3, sub: 0, line: 42 } |  |  | 0.543 |
| walker |  | 4229 | 20 | Code::CodeKey { rung: Body, file: lib/xast.js, decl: 1, sub: 0, line: 22 } |  |  | 0.543 |
| walker |  | 4249 | 20 | Code::CodeKey { rung: Body, file: lib/xast.js, decl: 2, sub: 0, line: 32 } |  |  | 0.543 |
| walker |  | 4284 | 35 | Code::CodeKey { rung: Body, file: lib/xast.js, decl: 4, sub: 0, line: 50 } |  |  | 0.543 |
| ns | 4291 |  | 152 | Overriding and disabling preset-default plugins (README) | 2.7 |  | 0.529 |
| walker |  | 4331 | 47 | Code::CodeKey { rung: Doc, file: lib/version.js, decl: 1, sub: 0, line: 7 } |  |  | 0.542 |
| walker |  | 4380 | 49 | Code::CodeKey { rung: Doc, file: lib/xast.js, decl: 4, sub: 0, line: 50 } |  |  | 0.542 |
| ns | 4408 |  | 117 | Custom plugin objects (docs/05-plugins-api.mdx) | 2.8 |  | 0.536 |
| ns | 4603 |  | 195 | Node entry point: loadConfig and the Node optimize wrapper | 2.9 |  | 0.526 |
| walker |  | 4670 | 290 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 3, sub: 0, line: 71 } |  |  | 0.526 |
| walker |  | 4763 | 93 | Code::CodeKey { rung: Names, file: lib/svgo.js, decl: 0, sub: 0, line: 0 } |  |  | 0.531 |
| ns | 4786 |  | 183 | Plugin descriptions: preset-default steps 1-9 | 3.1 |  | 0.524 |
| walker |  | 4790 | 27 | Code::CodeKey { rung: Doc, file: lib/svgo/coa.js, decl: 2, sub: 0, line: 30 } |  |  | 0.524 |
| walker |  | 4853 | 63 | Code::CodeKey { rung: Doc, file: lib/style.js, decl: 3, sub: 0, line: 253 } |  |  | 0.524 |
| ns | 5033 |  | 247 | Plugin descriptions: preset-default steps 10-18 | 3.2 |  | 0.516 |
| walker |  | 5234 | 381 | Json::Scripts { file: package.json } |  |  | 0.516 |
| ns | 5238 |  | 205 | Plugin descriptions: preset-default steps 19-26 | 3.3 |  | 0.509 |
| walker |  | 5302 | 68 | Code::CodeKey { rung: Doc, file: lib/svgo.js, decl: 1, sub: 0, line: 81 } |  |  | 0.528 |
| walker |  | 5308 | 6 | Fs::DirListing { dir: .yarn/plugins } |  |  | 0.528 |
| walker |  | 5380 | 72 | Code::CodeKey { rung: Doc, file: lib/svgo-node.js, decl: 2, sub: 0, line: 83 } |  |  | 0.528 |
| ns | 5405 |  | 167 | Plugin descriptions: preset-default steps 27-34 | 3.4 |  | 0.522 |
| walker |  | 5454 | 74 | Code::CodeKey { rung: Doc, file: lib/parser.js, decl: 4, sub: 0, line: 80 } |  |  | 0.522 |
| walker |  | 5583 | 129 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.522 |
| ns | 5778 |  | 373 | Plugin descriptions: opt-in plugins with optional params | 3.5 |  | 0.510 |
| walker |  | 5799 | 216 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 1, sub: 0, line: 30 } |  |  | 0.510 |
| ns | 5895 |  | 117 | Plugin descriptions: plugins requiring params | 3.6 |  | 0.507 |
| walker |  | 6022 | 223 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 1, sub: 1, line: 30 } |  |  | 0.507 |
| walker |  | 6098 | 76 | Code::CodeKey { rung: Doc, file: lib/stringifier.js, decl: 1, sub: 0, line: 66 } |  |  | 0.507 |
| ns | 6127 |  | 232 | Config and Output type bodies | 4.1 |  | 0.496 |
| walker |  | 6180 | 82 | Code::CodeKey { rung: Doc, file: lib/xast.js, decl: 3, sub: 0, line: 42 } |  |  | 0.496 |
| ns | 6440 |  | 313 | Xast node type bodies | 4.2 |  | 0.520 |
| ns | 6652 |  | 212 | Visitor keys, VisitorNode, PluginInfo and Plugin bodies | 4.3 |  | 0.510 |
| walker |  | 6680 | 500 | Code::CodeKey { rung: Decl, file: lib/builtin.js, decl: 1, sub: 0, line: 62 } |  |  | 0.511 |
| ns | 6809 |  | 157 | coa.js top-level function roster | 5.1 |  | 0.505 |
| ns | 6933 |  | 124 | CLI program metadata and positional argument | 5.2 |  | 0.501 |
| walker |  | 6959 | 279 | Code::CodeKey { rung: Names, file: lib/types.ts, decl: 0, sub: 1, line: 0 } |  |  | 0.538 |
| walker |  | 6971 | 12 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 37, sub: 0, line: 357 } |  |  | 0.538 |
| walker |  | 6993 | 22 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 31, sub: 0, line: 300 } |  |  | 0.538 |
| walker |  | 7016 | 23 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 24, sub: 0, line: 263 } |  |  | 0.539 |
| walker |  | 7039 | 23 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 34, sub: 0, line: 329 } |  |  | 0.539 |
| walker |  | 7069 | 30 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 27, sub: 0, line: 276 } |  |  | 0.539 |
| walker |  | 7099 | 30 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 29, sub: 0, line: 289 } |  |  | 0.539 |
| walker |  | 7130 | 31 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 30, sub: 0, line: 294 } |  |  | 0.539 |
| walker |  | 7170 | 40 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 25, sub: 0, line: 268 } |  |  | 0.541 |
| walker |  | 7212 | 42 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 28, sub: 0, line: 282 } |  |  | 0.541 |
| ns | 7223 |  | 290 | CLI options, first half | 5.3 |  | 0.530 |
| walker |  | 7259 | 47 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 22, sub: 0, line: 248 } |  |  | 0.531 |
| walker |  | 7308 | 49 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 21, sub: 0, line: 243 } |  |  | 0.535 |
| walker |  | 7403 | 95 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 23, sub: 0, line: 253 } |  |  | 0.551 |
| ns | 7567 |  | 344 | CLI options, second half | 5.4 |  | 0.539 |
| walker |  | 7581 | 178 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 33, sub: 0, line: 307 } |  |  | 0.539 |
| ns | 7675 |  | 108 | bin/svgo.js executable entry | 5.5 |  | 0.534 |
| ns | 7790 |  | 115 | parseSvg and the sax configuration | 6.1 |  | 0.530 |
| walker |  | 7801 | 220 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 36, sub: 0, line: 336 } |  |  | 0.554 |
| ns | 7913 |  | 123 | stringifier function roster | 6.2 |  | 0.550 |
| ns | 7996 |  | 83 | xast query helpers | 6.3 |  | 0.552 |
| walker |  | 8093 | 292 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 20, sub: 0, line: 214 } |  |  | 0.552 |
| walker |  | 8145 | 52 | Code::CodeKey { rung: Doc, file: lib/svgo/coa.js, decl: 1, sub: 0, line: 19 } |  |  | 0.552 |
| ns | 8174 |  | 178 | lib/svgo/tools.js -- complete export set | 6.4 |  | 0.547 |
| walker |  | 8195 | 50 | Code::CodeKey { rung: Body, file: lib/svgo/coa.js, decl: 1, sub: 0, line: 19 } |  |  | 0.547 |
| walker |  | 8297 | 102 | Code::CodeKey { rung: Doc, file: lib/xast.js, decl: 1, sub: 0, line: 22 } |  |  | 0.547 |
| ns | 8330 |  | 156 | lib/style.js -- exported and internal function roster | 6.5 |  | 0.548 |
| walker |  | 8397 | 100 | Code::CodeKey { rung: Body, file: lib/style.js, decl: 1, sub: 0, line: 195 } |  |  | 0.548 |
| ns | 8476 |  | 146 | lib/path.js -- path data parse and stringify roster | 6.6 |  | 0.545 |
| walker |  | 8502 | 105 | Code::CodeKey { rung: Doc, file: lib/builtin.js, decl: 1, sub: 0, line: 62 } |  |  | 0.555 |
| walker |  | 8620 | 118 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.555 |
| ns | 8710 |  | 234 | plugins/_collections.js -- complete export roster | 6.7 |  | 0.549 |
| walker |  | 8729 | 109 | Code::CodeKey { rung: Doc, file: lib/xast.js, decl: 2, sub: 0, line: 32 } |  |  | 0.549 |
| walker |  | 8786 | 57 | Code::CodeKey { rung: Doc, file: lib/util/visit.js, decl: 2, sub: 0, line: 8 } |  |  | 0.549 |
| walker |  | 8898 | 112 | Code::CodeKey { rung: Doc, file: lib/svgo-node.js, decl: 1, sub: 0, line: 44 } |  |  | 0.549 |
| walker |  | 8901 | 3 | Fs::DirListing { dir: test/fixtures/config-loader/one } |  |  | 0.549 |
| ns | 8904 |  | 194 | Shared plugin helper module rosters | 6.8 |  | 0.545 |
| walker |  | 8911 | 10 | Fs::DirListing { dir: test/svg2js } |  |  | 0.545 |
| ns | 9017 |  | 113 | css-select adapter | 6.9 |  | 0.542 |
| ns | 9034 |  | 17 | mapNodesToParents | 6.10 |  | 0.542 |
| ns | 9268 |  | 234 | package.json scripts -- how to build, test and lint | 7.1 |  | 0.547 |
| ns | 9337 |  | 69 | test/ tree listing | 7.2 |  | 0.556 |
| walker |  | 9413 | 502 | Json::IdentityMeta { file: package.json } |  |  | 0.557 |
| walker |  | 9445 | 32 | Plaintext::DeclSurface { file: docs/04-plugins/cleanupListOfValues.mdx } |  |  | 0.557 |
| ns | 9452 |  | 115 | The fixture-driven plugin test format | 7.3 |  | 0.555 |
| walker |  | 9477 | 32 | Plaintext::DeclSurface { file: docs/04-plugins/convertEllipseToCircle.mdx } |  |  | 0.555 |
| ns | 9584 |  | 132 | docs/ and remaining leaf directory listings | 7.4 |  | 0.563 |
| walker |  | 9597 | 120 | Code::CodeKey { rung: Doc, file: lib/style.js, decl: 1, sub: 0, line: 195 } |  |  | 0.563 |
| walker |  | 9630 | 33 | Plaintext::DeclSurface { file: docs/04-plugins/convertOneStopGradients.mdx } |  |  | 0.563 |
| ns | 9719 |  | 135 | Migration guide outlines | 7.5 |  | 0.559 |
| walker |  | 9795 | 165 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.582 |
| ns | 9843 |  | 124 | TypeScript and lint configuration | 7.6 |  | 0.578 |
| walker |  | 9870 | 75 | Code::CodeKey { rung: Doc, file: lib/parser.js, decl: 2, sub: 0, line: 12 } |  |  | 0.578 |
| walker |  | 9946 | 76 | Code::CodeKey { rung: Doc, file: lib/util/map-nodes-to-parents.js, decl: 1, sub: 0, line: 9 } |  |  | 0.578 |
| walker |  | 9959 | 13 | Fs::DirListing { dir: test/cli } |  |  | 0.578 |
| ns | 9968 |  | 125 | Runtime dependencies | 7.7 |  | 0.582 |

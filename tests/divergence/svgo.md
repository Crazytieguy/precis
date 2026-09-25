Score(3000)=0.667 I=0.859 C=0.519 ns_rows≤3K=14/54 grid(1000/1442/2080/3000/4327/6240/9000)=0.626/0.715/0.691/0.667/0.524/0.534/0.551

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
| ns | 496 |  | 110 | README section heading roster | 1.5 |  | 0.662 |
| walker |  | 509 | 41 | Code::CodeKey { rung: ModuleDoc, file: lib/path.js, decl: 0, sub: 0, line: 0 } |  |  | 0.662 |
| walker |  | 643 | 134 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.694 |
| walker |  | 692 | 49 | Json::Runtime { file: package.json } |  |  | 0.695 |
| ns | 715 |  | 219 | lib/svgo.js module graph and public re-exports | 1.6 |  | 0.608 |
| walker |  | 798 | 106 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.694 |
| walker |  | 832 | 34 | Fs::DirListing { dir: test } |  |  | 0.694 |
| walker |  | 836 | 4 | Fs::DirListing { dir: test/fixtures } |  |  | 0.694 |
| walker |  | 855 | 19 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.694 |
| ns | 875 |  | 160 | Command-line usage (README) | 1.7 |  | 0.626 |
| walker |  | 1211 | 356 | Fs::DirListing { dir: plugins } |  |  | 0.663 |
| ns | 1231 |  | 356 | Complete plugins/ listing (all 58 modules) | 1.8 |  | 0.714 |
| walker |  | 1416 | 205 | Json::Dependencies { file: package.json } |  |  | 0.715 |
| ns | 1470 |  | 239 | svgo.config.mjs shape (README Configuration) | 1.9 |  | 0.660 |
| walker |  | 1471 | 55 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.759 |
| ns | 1669 |  | 199 | optimize() body, part 1: multipass loop and plugin resolution | 1.10 | 1.4 | 0.724 |
| walker |  | 1755 | 284 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: true } |  |  | 0.724 |
| walker |  | 1790 | 35 | Fs::DirListing { dir: test/regression } |  |  | 0.725 |
| ns | 1873 |  | 204 | optimize() body, part 2: overrides, invoke, stringify, datauri | 1.11 | 1.10 | 0.687 |
| walker |  | 2017 | 227 | Json::Entry { file: package.json } |  |  | 0.691 |
| walker |  | 2066 | 49 | Fs::DirListing { dir: test/fixtures/config-loader } |  |  | 0.691 |
| walker |  | 2073 | 7 | Code::CodeKey { rung: Names, file: lib/types.js, decl: 0, sub: 0, line: 0 } |  |  | 0.691 |
| walker |  | 2346 | 273 | Code::CodeKey { rung: Names, file: lib/types.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.693 |
| walker |  | 2355 | 9 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 5, sub: 0, line: 108 } |  |  | 0.693 |
| walker |  | 2377 | 22 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 12, sub: 0, line: 175 } |  |  | 0.693 |
| walker |  | 2399 | 22 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 14, sub: 0, line: 185 } |  |  | 0.693 |
| walker |  | 2422 | 23 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 13, sub: 0, line: 180 } |  |  | 0.694 |
| ns | 2423 |  | 550 | Complete exported type roster of lib/types.ts | 1.12 |  | 0.656 |
| walker |  | 2446 | 24 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 2, sub: 0, line: 67 } |  |  | 0.656 |
| walker |  | 2470 | 24 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 17, sub: 0, line: 205 } |  |  | 0.656 |
| walker |  | 2501 | 31 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 6, sub: 0, line: 111 } |  |  | 0.656 |
| walker |  | 2532 | 31 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 11, sub: 0, line: 169 } |  |  | 0.657 |
| walker |  | 2570 | 38 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 9, sub: 0, line: 155 } |  |  | 0.657 |
| walker |  | 2615 | 45 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 15, sub: 0, line: 190 } |  |  | 0.657 |
| walker |  | 2661 | 46 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 10, sub: 0, line: 161 } |  |  | 0.658 |
| ns | 2691 |  | 268 | Package identity, entry points and engines | 1.13 |  | 0.676 |
| walker |  | 2714 | 53 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 8, sub: 0, line: 133 } |  |  | 0.676 |
| ns | 2755 |  | 64 | Version constant | 1.14 |  | 0.666 |
| walker |  | 2774 | 60 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 16, sub: 0, line: 197 } |  |  | 0.667 |
| walker |  | 2857 | 83 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 4, sub: 0, line: 100 } |  |  | 0.667 |
| walker |  | 3013 | 156 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 7, sub: 0, line: 117 } |  |  | 0.667 |
| ns | 3125 |  | 370 | preset-default: the complete default pipeline in execution order | 2.1 |  | 0.618 |
| ns | 3259 |  | 134 | builtinPlugins registry | 2.2 |  | 0.608 |
| walker |  | 3401 | 388 | Fs::DirListing { dir: docs/04-plugins } |  |  | 0.608 |
| ns | 3448 |  | 189 | invokePlugins -- the plugin engine | 2.3 |  | 0.588 |
| walker |  | 3458 | 57 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.588 |
| ns | 3669 |  | 221 | createPreset -- how a preset validates and forwards overrides | 2.4 |  | 0.570 |
| walker |  | 3748 | 290 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 3, sub: 0, line: 71 } |  |  | 0.570 |
| ns | 3956 |  | 287 | visit() -- the AST traversal contract | 2.5 |  | 0.549 |
| walker |  | 4129 | 381 | Json::Scripts { file: package.json } |  |  | 0.550 |
| walker |  | 4135 | 6 | Fs::DirListing { dir: .yarn/plugins } |  |  | 0.550 |
| ns | 4139 |  | 183 | Anatomy of a plugin module (removeComments as exemplar) | 2.6 |  | 0.538 |
| walker |  | 4264 | 129 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.538 |
| ns | 4291 |  | 152 | Overriding and disabling preset-default plugins (README) | 2.7 |  | 0.524 |
| ns | 4408 |  | 117 | Custom plugin objects (docs/05-plugins-api.mdx) | 2.8 |  | 0.519 |
| walker |  | 4480 | 216 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 1, sub: 0, line: 30 } |  |  | 0.519 |
| ns | 4603 |  | 195 | Node entry point: loadConfig and the Node optimize wrapper | 2.9 |  | 0.509 |
| walker |  | 4703 | 223 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 1, sub: 1, line: 30 } |  |  | 0.509 |
| walker |  | 4733 | 30 | Code::CodeKey { rung: Names, file: lib/path.js, decl: 0, sub: 0, line: 0 } |  |  | 0.509 |
| walker |  | 4765 | 32 | Code::CodeKey { rung: Decl, file: lib/path.js, decl: 2, sub: 0, line: 302 } |  |  | 0.509 |
| ns | 4786 |  | 183 | Plugin descriptions: preset-default steps 1-9 | 3.1 |  | 0.502 |
| walker |  | 4801 | 36 | Code::CodeKey { rung: Doc, file: lib/path.js, decl: 2, sub: 0, line: 302 } |  |  | 0.502 |
| walker |  | 4841 | 40 | Code::CodeKey { rung: Doc, file: lib/path.js, decl: 1, sub: 0, line: 141 } |  |  | 0.502 |
| ns | 5033 |  | 247 | Plugin descriptions: preset-default steps 10-18 | 3.2 |  | 0.494 |
| walker |  | 5120 | 279 | Code::CodeKey { rung: Names, file: lib/types.ts, decl: 0, sub: 1, line: 0 } |  |  | 0.541 |
| walker |  | 5132 | 12 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 37, sub: 0, line: 357 } |  |  | 0.541 |
| walker |  | 5154 | 22 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 31, sub: 0, line: 300 } |  |  | 0.541 |
| walker |  | 5177 | 23 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 24, sub: 0, line: 263 } |  |  | 0.541 |
| walker |  | 5200 | 23 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 34, sub: 0, line: 329 } |  |  | 0.541 |
| walker |  | 5230 | 30 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 27, sub: 0, line: 276 } |  |  | 0.541 |
| ns | 5238 |  | 205 | Plugin descriptions: preset-default steps 19-26 | 3.3 |  | 0.534 |
| walker |  | 5260 | 30 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 29, sub: 0, line: 289 } |  |  | 0.534 |
| walker |  | 5291 | 31 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 30, sub: 0, line: 294 } |  |  | 0.534 |
| walker |  | 5331 | 40 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 25, sub: 0, line: 268 } |  |  | 0.534 |
| walker |  | 5373 | 42 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 28, sub: 0, line: 282 } |  |  | 0.534 |
| ns | 5405 |  | 167 | Plugin descriptions: preset-default steps 27-34 | 3.4 |  | 0.528 |
| walker |  | 5420 | 47 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 22, sub: 0, line: 248 } |  |  | 0.528 |
| walker |  | 5469 | 49 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 21, sub: 0, line: 243 } |  |  | 0.529 |
| walker |  | 5564 | 95 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 23, sub: 0, line: 253 } |  |  | 0.530 |
| walker |  | 5742 | 178 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 33, sub: 0, line: 307 } |  |  | 0.530 |
| ns | 5778 |  | 373 | Plugin descriptions: opt-in plugins with optional params | 3.5 |  | 0.518 |
| ns | 5895 |  | 117 | Plugin descriptions: plugins requiring params | 3.6 |  | 0.514 |
| walker |  | 5962 | 220 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 36, sub: 0, line: 336 } |  |  | 0.516 |
| ns | 6127 |  | 232 | Config and Output type bodies | 4.1 |  | 0.534 |
| walker |  | 6254 | 292 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 20, sub: 0, line: 214 } |  |  | 0.534 |
| walker |  | 6270 | 16 | Code::CodeKey { rung: Names, file: lib/builtin.js, decl: 0, sub: 0, line: 0 } |  |  | 0.534 |
| ns | 6440 |  | 313 | Xast node type bodies | 4.2 |  | 0.553 |
| ns | 6652 |  | 212 | Visitor keys, VisitorNode, PluginInfo and Plugin bodies | 4.3 |  | 0.565 |
| walker |  | 6770 | 500 | Code::CodeKey { rung: Decl, file: lib/builtin.js, decl: 1, sub: 0, line: 62 } |  |  | 0.566 |
| walker |  | 6787 | 17 | Code::CodeKey { rung: Names, file: lib/version.js, decl: 0, sub: 0, line: 0 } |  |  | 0.566 |
| ns | 6809 |  | 157 | coa.js top-level function roster | 5.1 |  | 0.559 |
| walker |  | 6834 | 47 | Code::CodeKey { rung: Doc, file: lib/version.js, decl: 1, sub: 0, line: 7 } |  |  | 0.567 |
| walker |  | 6903 | 69 | Code::CodeKey { rung: Names, file: lib/style.js, decl: 0, sub: 0, line: 0 } |  |  | 0.568 |
| ns | 6933 |  | 124 | CLI program metadata and positional argument | 5.2 |  | 0.563 |
| walker |  | 6943 | 40 | Code::CodeKey { rung: Decl, file: lib/style.js, decl: 4, sub: 0, line: 286 } |  |  | 0.564 |
| walker |  | 6988 | 45 | Code::CodeKey { rung: Doc, file: lib/style.js, decl: 2, sub: 0, line: 211 } |  |  | 0.564 |
| walker |  | 7051 | 63 | Code::CodeKey { rung: Doc, file: lib/style.js, decl: 3, sub: 0, line: 253 } |  |  | 0.564 |
| walker |  | 7151 | 100 | Code::CodeKey { rung: Body, file: lib/style.js, decl: 1, sub: 0, line: 195 } |  |  | 0.564 |
| ns | 7223 |  | 290 | CLI options, first half | 5.3 |  | 0.553 |
| walker |  | 7256 | 105 | Code::CodeKey { rung: Doc, file: lib/builtin.js, decl: 1, sub: 0, line: 62 } |  |  | 0.564 |
| walker |  | 7291 | 35 | Code::CodeKey { rung: Names, file: lib/parser.js, decl: 0, sub: 0, line: 0 } |  |  | 0.564 |
| walker |  | 7328 | 37 | Code::CodeKey { rung: Decl, file: lib/parser.js, decl: 1, sub: 0, line: 4 } |  |  | 0.564 |
| walker |  | 7402 | 74 | Code::CodeKey { rung: Doc, file: lib/parser.js, decl: 4, sub: 0, line: 80 } |  |  | 0.564 |
| walker |  | 7477 | 75 | Code::CodeKey { rung: Doc, file: lib/parser.js, decl: 2, sub: 0, line: 12 } |  |  | 0.564 |
| ns | 7567 |  | 344 | CLI options, second half | 5.4 |  | 0.552 |
| walker |  | 7595 | 118 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.552 |
| walker |  | 7651 | 56 | Code::CodeKey { rung: Names, file: lib/svgo-node.js, decl: 0, sub: 0, line: 0 } |  |  | 0.552 |
| ns | 7675 |  | 108 | bin/svgo.js executable entry | 5.5 |  | 0.548 |
| walker |  | 7723 | 72 | Code::CodeKey { rung: Doc, file: lib/svgo-node.js, decl: 2, sub: 0, line: 83 } |  |  | 0.548 |
| ns | 7790 |  | 115 | parseSvg and the sax configuration | 6.1 |  | 0.544 |
| walker |  | 7835 | 112 | Code::CodeKey { rung: Doc, file: lib/svgo-node.js, decl: 1, sub: 0, line: 44 } |  |  | 0.544 |
| walker |  | 7838 | 3 | Fs::DirListing { dir: test/fixtures/config-loader/one } |  |  | 0.544 |
| walker |  | 7848 | 10 | Fs::DirListing { dir: test/svg2js } |  |  | 0.544 |
| ns | 7913 |  | 123 | stringifier function roster | 6.2 |  | 0.539 |
| ns | 7996 |  | 83 | xast query helpers | 6.3 |  | 0.538 |
| ns | 8174 |  | 178 | lib/svgo/tools.js -- complete export set | 6.4 |  | 0.533 |
| ns | 8330 |  | 156 | lib/style.js -- exported and internal function roster | 6.5 |  | 0.534 |
| walker |  | 8350 | 502 | Json::IdentityMeta { file: package.json } |  |  | 0.535 |
| walker |  | 8382 | 32 | Plaintext::DeclSurface { file: docs/04-plugins/cleanupListOfValues.mdx } |  |  | 0.535 |
| walker |  | 8414 | 32 | Plaintext::DeclSurface { file: docs/04-plugins/convertEllipseToCircle.mdx } |  |  | 0.535 |
| ns | 8476 |  | 146 | lib/path.js -- path data parse and stringify roster | 6.6 |  | 0.532 |
| walker |  | 8534 | 120 | Code::CodeKey { rung: Doc, file: lib/style.js, decl: 1, sub: 0, line: 195 } |  |  | 0.532 |
| walker |  | 8567 | 33 | Plaintext::DeclSurface { file: docs/04-plugins/convertOneStopGradients.mdx } |  |  | 0.532 |
| ns | 8710 |  | 234 | plugins/_collections.js -- complete export roster | 6.7 |  | 0.527 |
| walker |  | 8732 | 165 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.551 |
| walker |  | 8851 | 119 | Code::CodeKey { rung: Body, file: lib/parser.js, decl: 2, sub: 0, line: 12 } |  |  | 0.551 |
| ns | 8904 |  | 194 | Shared plugin helper module rosters | 6.8 |  | 0.547 |
| walker |  | 8934 | 83 | Code::CodeKey { rung: Names, file: lib/xast.js, decl: 0, sub: 0, line: 0 } |  |  | 0.551 |
| walker |  | 8953 | 19 | Code::CodeKey { rung: Body, file: lib/xast.js, decl: 3, sub: 0, line: 42 } |  |  | 0.551 |
| walker |  | 8973 | 20 | Code::CodeKey { rung: Body, file: lib/xast.js, decl: 1, sub: 0, line: 22 } |  |  | 0.551 |
| walker |  | 8993 | 20 | Code::CodeKey { rung: Body, file: lib/xast.js, decl: 2, sub: 0, line: 32 } |  |  | 0.551 |
| ns | 9017 |  | 113 | css-select adapter | 6.9 |  | 0.547 |
| walker |  | 9028 | 35 | Code::CodeKey { rung: Body, file: lib/xast.js, decl: 4, sub: 0, line: 50 } |  |  | 0.547 |
| ns | 9034 |  | 17 | mapNodesToParents | 6.10 |  | 0.546 |
| walker |  | 9077 | 49 | Code::CodeKey { rung: Doc, file: lib/xast.js, decl: 4, sub: 0, line: 50 } |  |  | 0.546 |
| walker |  | 9159 | 82 | Code::CodeKey { rung: Doc, file: lib/xast.js, decl: 3, sub: 0, line: 42 } |  |  | 0.546 |
| walker |  | 9261 | 102 | Code::CodeKey { rung: Doc, file: lib/xast.js, decl: 1, sub: 0, line: 22 } |  |  | 0.546 |
| ns | 9268 |  | 234 | package.json scripts -- how to build, test and lint | 7.1 |  | 0.551 |
| ns | 9337 |  | 69 | test/ tree listing | 7.2 |  | 0.560 |
| walker |  | 9370 | 109 | Code::CodeKey { rung: Doc, file: lib/xast.js, decl: 2, sub: 0, line: 32 } |  |  | 0.560 |
| walker |  | 9392 | 22 | Code::CodeKey { rung: Names, file: lib/stringifier.js, decl: 0, sub: 0, line: 0 } |  |  | 0.560 |
| ns | 9452 |  | 115 | The fixture-driven plugin test format | 7.3 |  | 0.558 |
| walker |  | 9468 | 76 | Code::CodeKey { rung: Doc, file: lib/stringifier.js, decl: 1, sub: 0, line: 66 } |  |  | 0.558 |
| walker |  | 9481 | 13 | Fs::DirListing { dir: test/cli } |  |  | 0.558 |
| walker |  | 9524 | 43 | Plaintext::DeclSurface { file: docs/04-plugins/mergePaths.mdx } |  |  | 0.558 |
| ns | 9584 |  | 132 | docs/ and remaining leaf directory listings | 7.4 |  | 0.566 |
| walker |  | 9676 | 152 | Code::CodeKey { rung: Body, file: lib/svgo-node.js, decl: 2, sub: 0, line: 83 } |  |  | 0.570 |
| ns | 9719 |  | 135 | Migration guide outlines | 7.5 |  | 0.566 |
| walker |  | 9843 | 167 | Code::CodeKey { rung: Doc, file: lib/style.js, decl: 4, sub: 0, line: 286 } |  |  | 0.563 |
| ns | 9843 |  | 124 | TypeScript and lint configuration | 7.6 |  | 0.563 |
| walker |  | 9889 | 46 | Plaintext::DeclSurface { file: docs/04-plugins/mergeStyles.mdx } |  |  | 0.563 |
| walker |  | 9937 | 48 | Plaintext::DeclSurface { file: docs/04-plugins/addAttributesToSVGElement.mdx } |  |  | 0.563 |
| walker |  | 9945 | 8 | Fs::DirListing { dir: .yarn/plugins/@yarnpkg } |  |  | 0.563 |
| ns | 9968 |  | 125 | Runtime dependencies | 7.7 |  | 0.567 |
| walker |  | 9994 | 49 | Plaintext::DeclSurface { file: docs/04-plugins/moveElemsAttrsToGroup.mdx } |  |  | 0.567 |

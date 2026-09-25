Score(3000)=0.642 I=0.846 C=0.488 ns_rows≤3K=14/54 grid(1000/1442/2080/3000/4327/6240/9000)=0.626/0.714/0.691/0.642/0.578/0.549/0.510

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
| walker |  | 602 | 134 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.694 |
| walker |  | 643 | 41 | Code::CodeKey { rung: ModuleDoc, file: lib/path.js, decl: 0, sub: 0, line: 0 } |  |  | 0.694 |
| walker |  | 692 | 49 | Json::Runtime { file: package.json } |  |  | 0.695 |
| ns | 715 |  | 219 | lib/svgo.js module graph and public re-exports | 1.6 |  | 0.608 |
| walker |  | 798 | 106 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.694 |
| walker |  | 832 | 34 | Fs::DirListing { dir: test } |  |  | 0.694 |
| walker |  | 836 | 4 | Fs::DirListing { dir: test/fixtures } |  |  | 0.694 |
| walker |  | 855 | 19 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.694 |
| ns | 875 |  | 160 | Command-line usage (README) | 1.7 |  | 0.626 |
| walker |  | 1211 | 356 | Fs::DirListing { dir: plugins } |  |  | 0.663 |
| ns | 1231 |  | 356 | Complete plugins/ listing (all 58 modules) | 1.8 |  | 0.714 |
| walker |  | 1249 | 38 | Code::CodeKey { rung: ModuleDoc, file: plugins/_collections.js, decl: 0, sub: 0, line: 0 } |  |  | 0.714 |
| walker |  | 1454 | 205 | Json::Dependencies { file: package.json } |  |  | 0.715 |
| ns | 1470 |  | 239 | svgo.config.mjs shape (README Configuration) | 1.9 |  | 0.660 |
| walker |  | 1509 | 55 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.759 |
| ns | 1669 |  | 199 | optimize() body, part 1: multipass loop and plugin resolution | 1.10 | 1.4 | 0.724 |
| walker |  | 1793 | 284 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: true } |  |  | 0.724 |
| walker |  | 1828 | 35 | Fs::DirListing { dir: test/regression } |  |  | 0.725 |
| ns | 1873 |  | 204 | optimize() body, part 2: overrides, invoke, stringify, datauri | 1.11 | 1.10 | 0.687 |
| walker |  | 2055 | 227 | Json::Entry { file: package.json } |  |  | 0.691 |
| walker |  | 2104 | 49 | Fs::DirListing { dir: test/fixtures/config-loader } |  |  | 0.691 |
| walker |  | 2111 | 7 | Code::CodeKey { rung: Names, file: lib/types.js, decl: 0, sub: 0, line: 0 } |  |  | 0.691 |
| ns | 2423 |  | 550 | Complete exported type roster of lib/types.ts | 1.12 |  | 0.631 |
| walker |  | 2499 | 388 | Fs::DirListing { dir: docs/04-plugins } |  |  | 0.631 |
| walker |  | 2556 | 57 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.631 |
| ns | 2691 |  | 268 | Package identity, entry points and engines | 1.13 |  | 0.651 |
| ns | 2755 |  | 64 | Version constant | 1.14 |  | 0.642 |
| walker |  | 2937 | 381 | Json::Scripts { file: package.json } |  |  | 0.642 |
| walker |  | 2943 | 6 | Fs::DirListing { dir: .yarn/plugins } |  |  | 0.642 |
| walker |  | 2978 | 35 | Code::CodeKey { rung: Names, file: plugins/removeDimensions.js, decl: 0, sub: 0, line: 0 } |  |  | 0.642 |
| walker |  | 3002 | 24 | Code::CodeKey { rung: Decl, file: plugins/removeDimensions.js, decl: 2, sub: 0, line: 2 } |  |  | 0.642 |
| walker |  | 3026 | 24 | Code::CodeKey { rung: Names, file: plugins/preset-default.js, decl: 0, sub: 0, line: 0 } |  |  | 0.642 |
| ns | 3125 |  | 370 | preset-default: the complete default pipeline in execution order | 2.1 |  | 0.596 |
| ns | 3259 |  | 134 | builtinPlugins registry | 2.2 |  | 0.585 |
| walker |  | 3367 | 341 | Code::CodeKey { rung: Decl, file: plugins/preset-default.js, decl: 1, sub: 0, line: 37 } |  |  | 0.670 |
| ns | 3448 |  | 189 | invokePlugins -- the plugin engine | 2.3 |  | 0.649 |
| walker |  | 3496 | 129 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.649 |
| walker |  | 3535 | 39 | Code::CodeKey { rung: Names, file: plugins/removeOffCanvasPaths.js, decl: 0, sub: 0, line: 0 } |  |  | 0.649 |
| walker |  | 3553 | 18 | Code::CodeKey { rung: Decl, file: plugins/removeOffCanvasPaths.js, decl: 2, sub: 0, line: 7 } |  |  | 0.649 |
| walker |  | 3614 | 61 | Code::CodeKey { rung: Doc, file: plugins/removeOffCanvasPaths.js, decl: 3, sub: 0, line: 17 } |  |  | 0.649 |
| walker |  | 3653 | 39 | Code::CodeKey { rung: Names, file: plugins/reusePaths.js, decl: 0, sub: 0, line: 0 } |  |  | 0.649 |
| ns | 3669 |  | 221 | createPreset -- how a preset validates and forwards overrides | 2.4 |  | 0.629 |
| walker |  | 3708 | 55 | Code::CodeKey { rung: Decl, file: plugins/reusePaths.js, decl: 2, sub: 0, line: 5 } |  |  | 0.629 |
| walker |  | 3748 | 40 | Code::CodeKey { rung: Names, file: plugins/cleanupEnableBackground.js, decl: 0, sub: 0, line: 0 } |  |  | 0.629 |
| walker |  | 3763 | 15 | Code::CodeKey { rung: Decl, file: plugins/cleanupEnableBackground.js, decl: 2, sub: 0, line: 5 } |  |  | 0.629 |
| walker |  | 3803 | 40 | Code::CodeKey { rung: Names, file: plugins/moveGroupAttrsToElems.js, decl: 0, sub: 0, line: 0 } |  |  | 0.629 |
| walker |  | 3818 | 15 | Code::CodeKey { rung: Decl, file: plugins/moveGroupAttrsToElems.js, decl: 2, sub: 0, line: 5 } |  |  | 0.629 |
| walker |  | 3858 | 40 | Code::CodeKey { rung: Names, file: plugins/removeXlink.js, decl: 0, sub: 0, line: 0 } |  |  | 0.629 |
| walker |  | 3880 | 22 | Code::CodeKey { rung: Decl, file: plugins/removeXlink.js, decl: 2, sub: 0, line: 11 } |  |  | 0.629 |
| walker |  | 3956 | 76 | Code::CodeKey { rung: Doc, file: plugins/removeXlink.js, decl: 1, sub: 0, line: 10 } |  |  | 0.606 |
| ns | 3956 |  | 287 | visit() -- the AST traversal contract | 2.5 |  | 0.606 |
| walker |  | 3997 | 41 | Code::CodeKey { rung: Names, file: plugins/cleanupAttrs.js, decl: 0, sub: 0, line: 0 } |  |  | 0.606 |
| walker |  | 4015 | 18 | Code::CodeKey { rung: Decl, file: plugins/cleanupAttrs.js, decl: 2, sub: 0, line: 9 } |  |  | 0.606 |
| walker |  | 4071 | 56 | Code::CodeKey { rung: Doc, file: plugins/cleanupAttrs.js, decl: 1, sub: 0, line: 8 } |  |  | 0.606 |
| walker |  | 4135 | 64 | Code::CodeKey { rung: Doc, file: plugins/cleanupAttrs.js, decl: 3, sub: 0, line: 22 } |  |  | 0.606 |
| ns | 4139 |  | 183 | Anatomy of a plugin module (removeComments as exemplar) | 2.6 |  | 0.593 |
| walker |  | 4176 | 41 | Code::CodeKey { rung: Names, file: plugins/convertColors.js, decl: 0, sub: 0, line: 0 } |  |  | 0.593 |
| walker |  | 4202 | 26 | Code::CodeKey { rung: Decl, file: plugins/convertColors.js, decl: 2, sub: 0, line: 18 } |  |  | 0.593 |
| walker |  | 4243 | 41 | Code::CodeKey { rung: Names, file: plugins/convertTransform.js, decl: 0, sub: 0, line: 0 } |  |  | 0.593 |
| walker |  | 4258 | 15 | Code::CodeKey { rung: Decl, file: plugins/convertTransform.js, decl: 2, sub: 0, line: 44 } |  |  | 0.593 |
| ns | 4291 |  | 152 | Overriding and disabling preset-default plugins (README) | 2.7 |  | 0.578 |
| walker |  | 4300 | 42 | Code::CodeKey { rung: Names, file: plugins/cleanupNumericValues.js, decl: 0, sub: 0, line: 0 } |  |  | 0.578 |
| walker |  | 4322 | 22 | Code::CodeKey { rung: Decl, file: plugins/cleanupNumericValues.js, decl: 2, sub: 0, line: 12 } |  |  | 0.578 |
| walker |  | 4395 | 73 | Code::CodeKey { rung: Doc, file: plugins/cleanupNumericValues.js, decl: 3, sub: 0, line: 35 } |  |  | 0.578 |
| ns | 4408 |  | 117 | Custom plugin objects (docs/05-plugins-api.mdx) | 2.8 |  | 0.572 |
| walker |  | 4470 | 75 | Code::CodeKey { rung: Doc, file: plugins/cleanupNumericValues.js, decl: 1, sub: 0, line: 11 } |  |  | 0.572 |
| walker |  | 4512 | 42 | Code::CodeKey { rung: Names, file: plugins/convertOneStopGradients.js, decl: 0, sub: 0, line: 0 } |  |  | 0.572 |
| walker |  | 4532 | 20 | Code::CodeKey { rung: Decl, file: plugins/convertOneStopGradients.js, decl: 2, sub: 0, line: 10 } |  |  | 0.573 |
| walker |  | 4574 | 42 | Code::CodeKey { rung: Names, file: plugins/convertPathData.js, decl: 0, sub: 0, line: 0 } |  |  | 0.573 |
| walker |  | 4593 | 19 | Code::CodeKey { rung: Decl, file: plugins/convertPathData.js, decl: 2, sub: 0, line: 43 } |  |  | 0.573 |
| ns | 4603 |  | 195 | Node entry point: loadConfig and the Node optimize wrapper | 2.9 |  | 0.561 |
| walker |  | 4635 | 42 | Code::CodeKey { rung: Names, file: plugins/moveElemsAttrsToGroup.js, decl: 0, sub: 0, line: 0 } |  |  | 0.561 |
| walker |  | 4651 | 16 | Code::CodeKey { rung: Decl, file: plugins/moveElemsAttrsToGroup.js, decl: 2, sub: 0, line: 5 } |  |  | 0.561 |
| walker |  | 4693 | 42 | Code::CodeKey { rung: Names, file: plugins/removeEditorsNSData.js, decl: 0, sub: 0, line: 0 } |  |  | 0.561 |
| walker |  | 4708 | 15 | Code::CodeKey { rung: Decl, file: plugins/removeEditorsNSData.js, decl: 2, sub: 0, line: 10 } |  |  | 0.561 |
| walker |  | 4744 | 36 | Code::CodeKey { rung: Doc, file: plugins/removeEditorsNSData.js, decl: 1, sub: 0, line: 9 } |  |  | 0.561 |
| walker |  | 4786 | 42 | Code::CodeKey { rung: Names, file: plugins/removeHiddenElems.js, decl: 0, sub: 0, line: 0 } |  |  | 0.556 |
| ns | 4786 |  | 183 | Plugin descriptions: preset-default steps 1-9 | 3.1 |  | 0.556 |
| walker |  | 4804 | 18 | Code::CodeKey { rung: Decl, file: plugins/removeHiddenElems.js, decl: 2, sub: 0, line: 30 } |  |  | 0.556 |
| walker |  | 4846 | 42 | Code::CodeKey { rung: Names, file: plugins/removeNonInheritableGroupAttrs.js, decl: 0, sub: 0, line: 0 } |  |  | 0.556 |
| walker |  | 4863 | 17 | Code::CodeKey { rung: Decl, file: plugins/removeNonInheritableGroupAttrs.js, decl: 2, sub: 0, line: 8 } |  |  | 0.556 |
| walker |  | 4925 | 62 | Code::CodeKey { rung: Doc, file: plugins/removeNonInheritableGroupAttrs.js, decl: 3, sub: 0, line: 18 } |  |  | 0.556 |
| walker |  | 4967 | 42 | Code::CodeKey { rung: Names, file: plugins/removeScripts.js, decl: 0, sub: 0, line: 0 } |  |  | 0.556 |
| ns | 5033 |  | 247 | Plugin descriptions: preset-default steps 10-18 | 3.2 |  | 0.553 |
| walker |  | 5040 | 73 | Code::CodeKey { rung: Doc, file: plugins/removeScripts.js, decl: 3, sub: 0, line: 24 } |  |  | 0.553 |
| walker |  | 5083 | 43 | Code::CodeKey { rung: Names, file: plugins/removeAttributesBySelector.js, decl: 0, sub: 0, line: 0 } |  |  | 0.553 |
| walker |  | 5100 | 17 | Code::CodeKey { rung: Decl, file: plugins/removeAttributesBySelector.js, decl: 2, sub: 0, line: 22 } |  |  | 0.553 |
| walker |  | 5143 | 43 | Code::CodeKey { rung: Names, file: plugins/removeMetadata.js, decl: 0, sub: 0, line: 0 } |  |  | 0.554 |
| walker |  | 5222 | 79 | Code::CodeKey { rung: Doc, file: plugins/removeMetadata.js, decl: 3, sub: 0, line: 15 } |  |  | 0.554 |
| ns | 5238 |  | 205 | Plugin descriptions: preset-default steps 19-26 | 3.3 |  | 0.553 |
| walker |  | 5304 | 82 | Code::CodeKey { rung: Body, file: plugins/removeMetadata.js, decl: 3, sub: 0, line: 15 } |  |  | 0.553 |
| walker |  | 5347 | 43 | Code::CodeKey { rung: Names, file: plugins/removeTitle.js, decl: 0, sub: 0, line: 0 } |  |  | 0.553 |
| ns | 5405 |  | 167 | Plugin descriptions: preset-default steps 27-34 | 3.4 |  | 0.548 |
| walker |  | 5427 | 80 | Code::CodeKey { rung: Doc, file: plugins/removeTitle.js, decl: 3, sub: 0, line: 15 } |  |  | 0.548 |
| walker |  | 5509 | 82 | Code::CodeKey { rung: Body, file: plugins/removeTitle.js, decl: 3, sub: 0, line: 15 } |  |  | 0.548 |
| walker |  | 5552 | 43 | Code::CodeKey { rung: Names, file: plugins/removeUnusedNS.js, decl: 0, sub: 0, line: 0 } |  |  | 0.549 |
| walker |  | 5625 | 73 | Code::CodeKey { rung: Doc, file: plugins/removeUnusedNS.js, decl: 3, sub: 0, line: 12 } |  |  | 0.549 |
| ns | 5778 |  | 373 | Plugin descriptions: opt-in plugins with optional params | 3.5 |  | 0.548 |
| ns | 5895 |  | 117 | Plugin descriptions: plugins requiring params | 3.6 |  | 0.545 |
| walker |  | 5898 | 273 | Code::CodeKey { rung: Names, file: lib/types.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.559 |
| walker |  | 5907 | 9 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 5, sub: 0, line: 108 } |  |  | 0.559 |
| walker |  | 5929 | 22 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 12, sub: 0, line: 175 } |  |  | 0.559 |
| walker |  | 5951 | 22 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 14, sub: 0, line: 185 } |  |  | 0.559 |
| walker |  | 5974 | 23 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 13, sub: 0, line: 180 } |  |  | 0.559 |
| walker |  | 5998 | 24 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 2, sub: 0, line: 67 } |  |  | 0.559 |
| walker |  | 6022 | 24 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 17, sub: 0, line: 205 } |  |  | 0.560 |
| walker |  | 6053 | 31 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 6, sub: 0, line: 111 } |  |  | 0.560 |
| walker |  | 6084 | 31 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 11, sub: 0, line: 169 } |  |  | 0.560 |
| walker |  | 6122 | 38 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 9, sub: 0, line: 155 } |  |  | 0.560 |
| ns | 6127 |  | 232 | Config and Output type bodies | 4.1 |  | 0.548 |
| walker |  | 6167 | 45 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 15, sub: 0, line: 190 } |  |  | 0.548 |
| walker |  | 6213 | 46 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 10, sub: 0, line: 161 } |  |  | 0.549 |
| walker |  | 6266 | 53 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 8, sub: 0, line: 133 } |  |  | 0.549 |
| walker |  | 6326 | 60 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 16, sub: 0, line: 197 } |  |  | 0.550 |
| walker |  | 6409 | 83 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 4, sub: 0, line: 100 } |  |  | 0.550 |
| ns | 6440 |  | 313 | Xast node type bodies | 4.2 |  | 0.567 |
| walker |  | 6565 | 156 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 7, sub: 0, line: 117 } |  |  | 0.567 |
| ns | 6652 |  | 212 | Visitor keys, VisitorNode, PluginInfo and Plugin bodies | 4.3 |  | 0.557 |
| ns | 6809 |  | 157 | coa.js top-level function roster | 5.1 |  | 0.550 |
| walker |  | 6855 | 290 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 3, sub: 0, line: 71 } |  |  | 0.550 |
| ns | 6933 |  | 124 | CLI program metadata and positional argument | 5.2 |  | 0.546 |
| walker |  | 7071 | 216 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 1, sub: 0, line: 30 } |  |  | 0.546 |
| ns | 7223 |  | 290 | CLI options, first half | 5.3 |  | 0.536 |
| walker |  | 7294 | 223 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 1, sub: 1, line: 30 } |  |  | 0.536 |
| walker |  | 7338 | 44 | Code::CodeKey { rung: Names, file: plugins/removeEmptyAttrs.js, decl: 0, sub: 0, line: 0 } |  |  | 0.536 |
| walker |  | 7395 | 57 | Code::CodeKey { rung: Doc, file: plugins/removeEmptyAttrs.js, decl: 3, sub: 0, line: 13 } |  |  | 0.536 |
| walker |  | 7439 | 44 | Code::CodeKey { rung: Names, file: plugins/removeRasterImages.js, decl: 0, sub: 0, line: 0 } |  |  | 0.538 |
| walker |  | 7523 | 84 | Code::CodeKey { rung: Doc, file: plugins/removeRasterImages.js, decl: 3, sub: 0, line: 15 } |  |  | 0.538 |
| walker |  | 7567 | 44 | Code::CodeKey { rung: Names, file: plugins/removeUnknownsAndDefaults.js, decl: 0, sub: 0, line: 0 } |  |  | 0.527 |
| ns | 7567 |  | 344 | CLI options, second half | 5.4 |  | 0.527 |
| walker |  | 7587 | 20 | Code::CodeKey { rung: Decl, file: plugins/removeUnknownsAndDefaults.js, decl: 2, sub: 0, line: 32 } |  |  | 0.529 |
| walker |  | 7665 | 78 | Code::CodeKey { rung: Doc, file: plugins/removeUnknownsAndDefaults.js, decl: 3, sub: 0, line: 103 } |  |  | 0.529 |
| ns | 7675 |  | 108 | bin/svgo.js executable entry | 5.5 |  | 0.524 |
| walker |  | 7695 | 30 | Code::CodeKey { rung: Names, file: lib/path.js, decl: 0, sub: 0, line: 0 } |  |  | 0.524 |
| walker |  | 7727 | 32 | Code::CodeKey { rung: Decl, file: lib/path.js, decl: 2, sub: 0, line: 302 } |  |  | 0.525 |
| walker |  | 7763 | 36 | Code::CodeKey { rung: Doc, file: lib/path.js, decl: 2, sub: 0, line: 302 } |  |  | 0.525 |
| ns | 7790 |  | 115 | parseSvg and the sax configuration | 6.1 |  | 0.520 |
| walker |  | 7803 | 40 | Code::CodeKey { rung: Doc, file: lib/path.js, decl: 1, sub: 0, line: 141 } |  |  | 0.520 |
| walker |  | 7848 | 45 | Code::CodeKey { rung: Names, file: plugins/collapseGroups.js, decl: 0, sub: 0, line: 0 } |  |  | 0.522 |
| walker |  | 7893 | 45 | Code::CodeKey { rung: Names, file: plugins/mergeStyles.js, decl: 0, sub: 0, line: 0 } |  |  | 0.523 |
| ns | 7913 |  | 123 | stringifier function roster | 6.2 |  | 0.519 |
| walker |  | 7956 | 63 | Code::CodeKey { rung: Doc, file: plugins/mergeStyles.js, decl: 3, sub: 0, line: 14 } |  |  | 0.519 |
| ns | 7996 |  | 83 | xast query helpers | 6.3 |  | 0.517 |
| walker |  | 8001 | 45 | Code::CodeKey { rung: Names, file: plugins/removeDeprecatedAttrs.js, decl: 0, sub: 0, line: 0 } |  |  | 0.518 |
| walker |  | 8036 | 35 | Code::CodeKey { rung: Doc, file: plugins/removeDeprecatedAttrs.js, decl: 1, sub: 0, line: 10 } |  |  | 0.518 |
| walker |  | 8081 | 45 | Code::CodeKey { rung: Doc, file: plugins/removeDeprecatedAttrs.js, decl: 3, sub: 0, line: 76 } |  |  | 0.518 |
| walker |  | 8126 | 45 | Code::CodeKey { rung: Names, file: plugins/removeDoctype.js, decl: 0, sub: 0, line: 0 } |  |  | 0.520 |
| ns | 8174 |  | 178 | lib/svgo/tools.js -- complete export set | 6.4 |  | 0.516 |
| walker |  | 8189 | 63 | Code::CodeKey { rung: Body, file: plugins/removeDoctype.js, decl: 3, sub: 0, line: 30 } |  |  | 0.516 |
| walker |  | 8234 | 45 | Code::CodeKey { rung: Names, file: plugins/removeViewBox.js, decl: 0, sub: 0, line: 0 } |  |  | 0.517 |
| walker |  | 8321 | 87 | Code::CodeKey { rung: Doc, file: plugins/reusePaths.js, decl: 3, sub: 0, line: 18 } |  |  | 0.517 |
| ns | 8330 |  | 156 | lib/style.js -- exported and internal function roster | 6.5 |  | 0.512 |
| walker |  | 8367 | 46 | Code::CodeKey { rung: Names, file: plugins/removeComments.js, decl: 0, sub: 0, line: 0 } |  |  | 0.514 |
| walker |  | 8410 | 43 | Code::CodeKey { rung: Doc, file: plugins/removeComments.js, decl: 1, sub: 0, line: 8 } |  |  | 0.517 |
| walker |  | 8456 | 46 | Code::CodeKey { rung: Names, file: plugins/removeStyleElement.js, decl: 0, sub: 0, line: 0 } |  |  | 0.518 |
| ns | 8476 |  | 146 | lib/path.js -- path data parse and stringify roster | 6.6 |  | 0.516 |
| walker |  | 8538 | 82 | Code::CodeKey { rung: Body, file: plugins/removeStyleElement.js, decl: 3, sub: 0, line: 15 } |  |  | 0.516 |
| walker |  | 8621 | 83 | Code::CodeKey { rung: Doc, file: plugins/removeStyleElement.js, decl: 3, sub: 0, line: 15 } |  |  | 0.516 |
| walker |  | 8667 | 46 | Code::CodeKey { rung: Names, file: plugins/removeXMLNS.js, decl: 0, sub: 0, line: 0 } |  |  | 0.518 |
| ns | 8710 |  | 234 | plugins/_collections.js -- complete export roster | 6.7 |  | 0.512 |
| walker |  | 8743 | 76 | Code::CodeKey { rung: Body, file: plugins/removeXMLNS.js, decl: 3, sub: 0, line: 16 } |  |  | 0.512 |
| walker |  | 8789 | 46 | Code::CodeKey { rung: Names, file: plugins/removeXMLProcInst.js, decl: 0, sub: 0, line: 0 } |  |  | 0.514 |
| walker |  | 8871 | 82 | Code::CodeKey { rung: Body, file: plugins/removeXMLProcInst.js, decl: 3, sub: 0, line: 16 } |  |  | 0.514 |
| ns | 8904 |  | 194 | Shared plugin helper module rosters | 6.8 |  | 0.510 |
| walker |  | 8960 | 89 | Code::CodeKey { rung: Doc, file: plugins/removeXMLProcInst.js, decl: 3, sub: 0, line: 16 } |  |  | 0.510 |
| ns | 9017 |  | 113 | css-select adapter | 6.9 |  | 0.507 |
| ns | 9034 |  | 17 | mapNodesToParents | 6.10 |  | 0.506 |
| walker |  | 9192 | 232 | Code::CodeKey { rung: Names, file: plugins/_collections.js, decl: 0, sub: 0, line: 0 } |  |  | 0.521 |
| walker |  | 9256 | 64 | Code::CodeKey { rung: Decl, file: plugins/_collections.js, decl: 14, sub: 0, line: 2387 } |  |  | 0.521 |
| ns | 9268 |  | 234 | package.json scripts -- how to build, test and lint | 7.1 |  | 0.527 |
| ns | 9337 |  | 69 | test/ tree listing | 7.2 |  | 0.536 |
| walker |  | 9338 | 82 | Code::CodeKey { rung: Decl, file: plugins/_collections.js, decl: 11, sub: 0, line: 2179 } |  |  | 0.536 |
| walker |  | 9439 | 101 | Code::CodeKey { rung: Decl, file: plugins/_collections.js, decl: 9, sub: 0, line: 2111 } |  |  | 0.536 |
| ns | 9452 |  | 115 | The fixture-driven plugin test format | 7.3 |  | 0.533 |
| walker |  | 9461 | 22 | Code::CodeKey { rung: Doc, file: plugins/_collections.js, decl: 3, sub: 0, line: 117 } |  |  | 0.533 |
| ns | 9584 |  | 132 | docs/ and remaining leaf directory listings | 7.4 |  | 0.543 |
| walker |  | 9619 | 158 | Code::CodeKey { rung: Decl, file: plugins/_collections.js, decl: 6, sub: 0, line: 389 } |  |  | 0.543 |
| walker |  | 9646 | 27 | Code::CodeKey { rung: Doc, file: plugins/_collections.js, decl: 11, sub: 0, line: 2179 } |  |  | 0.543 |
| walker |  | 9698 | 52 | Code::CodeKey { rung: Doc, file: plugins/_collections.js, decl: 9, sub: 0, line: 2111 } |  |  | 0.543 |
| ns | 9719 |  | 135 | Migration guide outlines | 7.5 |  | 0.539 |
| walker |  | 9754 | 56 | Code::CodeKey { rung: Doc, file: plugins/_collections.js, decl: 14, sub: 0, line: 2387 } |  |  | 0.539 |
| walker |  | 9813 | 59 | Code::CodeKey { rung: Doc, file: plugins/_collections.js, decl: 6, sub: 0, line: 389 } |  |  | 0.539 |
| ns | 9843 |  | 124 | TypeScript and lint configuration | 7.6 |  | 0.535 |
| walker |  | 9890 | 77 | Code::CodeKey { rung: Doc, file: plugins/_collections.js, decl: 2, sub: 0, line: 112 } |  |  | 0.535 |
| ns | 9968 |  | 125 | Runtime dependencies | 7.7 |  | 0.539 |

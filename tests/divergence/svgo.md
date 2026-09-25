Score(3000)=0.650 I=0.865 C=0.488 ns_rows≤3K=14/54 grid(1000/1442/2080/3000/4327/6240/9000)=0.638/0.821/0.691/0.650/0.579/0.560/0.586

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
| walker |  | 651 | 49 | Json::Runtime { file: package.json } |  |  | 0.695 |
| ns | 715 |  | 219 | lib/svgo.js module graph and public re-exports | 1.6 |  | 0.608 |
| walker |  | 757 | 106 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.694 |
| walker |  | 791 | 34 | Fs::DirListing { dir: test } |  |  | 0.694 |
| walker |  | 795 | 4 | Fs::DirListing { dir: test/fixtures } |  |  | 0.694 |
| walker |  | 814 | 19 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.694 |
| ns | 875 |  | 160 | Command-line usage (README) | 1.7 |  | 0.626 |
| walker |  | 1170 | 356 | Fs::DirListing { dir: plugins } |  |  | 0.663 |
| walker |  | 1225 | 55 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.769 |
| ns | 1231 |  | 356 | Complete plugins/ listing (all 58 modules) | 1.8 |  | 0.821 |
| ns | 1470 |  | 239 | svgo.config.mjs shape (README Configuration) | 1.9 |  | 0.759 |
| walker |  | 1509 | 284 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: true } |  |  | 0.759 |
| walker |  | 1544 | 35 | Fs::DirListing { dir: test/regression } |  |  | 0.760 |
| ns | 1669 |  | 199 | optimize() body, part 1: multipass loop and plugin resolution | 1.10 | 1.4 | 0.724 |
| walker |  | 1749 | 205 | Json::Dependencies { file: package.json } |  |  | 0.725 |
| walker |  | 1756 | 7 | Code::CodeKey { rung: Names, file: lib/types.js, decl: 0, sub: 0, line: 0 } |  |  | 0.725 |
| ns | 1873 |  | 204 | optimize() body, part 2: overrides, invoke, stringify, datauri | 1.11 | 1.10 | 0.687 |
| walker |  | 1983 | 227 | Json::Entry { file: package.json } |  |  | 0.691 |
| walker |  | 2032 | 49 | Fs::DirListing { dir: test/fixtures/config-loader } |  |  | 0.691 |
| walker |  | 2420 | 388 | Fs::DirListing { dir: docs/04-plugins } |  |  | 0.691 |
| ns | 2423 |  | 550 | Complete exported type roster of lib/types.ts | 1.12 |  | 0.631 |
| walker |  | 2477 | 57 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.631 |
| walker |  | 2512 | 35 | Code::CodeKey { rung: Names, file: plugins/removeDimensions.js, decl: 0, sub: 0, line: 0 } |  |  | 0.631 |
| walker |  | 2536 | 24 | Code::CodeKey { rung: Decl, file: plugins/removeDimensions.js, decl: 2, sub: 0, line: 2 } |  |  | 0.631 |
| walker |  | 2560 | 24 | Code::CodeKey { rung: Names, file: plugins/preset-default.js, decl: 0, sub: 0, line: 0 } |  |  | 0.631 |
| ns | 2691 |  | 268 | Package identity, entry points and engines | 1.13 |  | 0.651 |
| ns | 2755 |  | 64 | Version constant | 1.14 |  | 0.642 |
| walker |  | 2901 | 341 | Code::CodeKey { rung: Decl, file: plugins/preset-default.js, decl: 1, sub: 0, line: 37 } |  |  | 0.649 |
| walker |  | 2940 | 39 | Code::CodeKey { rung: Names, file: plugins/removeOffCanvasPaths.js, decl: 0, sub: 0, line: 0 } |  |  | 0.649 |
| walker |  | 2958 | 18 | Code::CodeKey { rung: Decl, file: plugins/removeOffCanvasPaths.js, decl: 2, sub: 0, line: 7 } |  |  | 0.650 |
| walker |  | 2997 | 39 | Code::CodeKey { rung: Names, file: plugins/reusePaths.js, decl: 0, sub: 0, line: 0 } |  |  | 0.650 |
| walker |  | 3052 | 55 | Code::CodeKey { rung: Decl, file: plugins/reusePaths.js, decl: 2, sub: 0, line: 5 } |  |  | 0.650 |
| walker |  | 3113 | 61 | Code::CodeKey { rung: Doc, file: plugins/removeOffCanvasPaths.js, decl: 3, sub: 0, line: 17 } |  |  | 0.650 |
| ns | 3125 |  | 370 | preset-default: the complete default pipeline in execution order | 2.1 |  | 0.681 |
| ns | 3259 |  | 134 | builtinPlugins registry | 2.2 |  | 0.670 |
| ns | 3448 |  | 189 | invokePlugins -- the plugin engine | 2.3 |  | 0.648 |
| walker |  | 3494 | 381 | Json::Scripts { file: package.json } |  |  | 0.649 |
| walker |  | 3534 | 40 | Code::CodeKey { rung: Names, file: plugins/cleanupEnableBackground.js, decl: 0, sub: 0, line: 0 } |  |  | 0.649 |
| walker |  | 3549 | 15 | Code::CodeKey { rung: Decl, file: plugins/cleanupEnableBackground.js, decl: 2, sub: 0, line: 5 } |  |  | 0.649 |
| walker |  | 3589 | 40 | Code::CodeKey { rung: Names, file: plugins/moveGroupAttrsToElems.js, decl: 0, sub: 0, line: 0 } |  |  | 0.649 |
| walker |  | 3604 | 15 | Code::CodeKey { rung: Decl, file: plugins/moveGroupAttrsToElems.js, decl: 2, sub: 0, line: 5 } |  |  | 0.649 |
| walker |  | 3644 | 40 | Code::CodeKey { rung: Names, file: plugins/removeXlink.js, decl: 0, sub: 0, line: 0 } |  |  | 0.649 |
| walker |  | 3666 | 22 | Code::CodeKey { rung: Decl, file: plugins/removeXlink.js, decl: 2, sub: 0, line: 11 } |  |  | 0.649 |
| ns | 3669 |  | 221 | createPreset -- how a preset validates and forwards overrides | 2.4 |  | 0.629 |
| walker |  | 3707 | 41 | Code::CodeKey { rung: Names, file: plugins/cleanupAttrs.js, decl: 0, sub: 0, line: 0 } |  |  | 0.629 |
| walker |  | 3725 | 18 | Code::CodeKey { rung: Decl, file: plugins/cleanupAttrs.js, decl: 2, sub: 0, line: 9 } |  |  | 0.629 |
| walker |  | 3781 | 56 | Code::CodeKey { rung: Doc, file: plugins/cleanupAttrs.js, decl: 1, sub: 0, line: 8 } |  |  | 0.629 |
| walker |  | 3822 | 41 | Code::CodeKey { rung: Names, file: plugins/convertColors.js, decl: 0, sub: 0, line: 0 } |  |  | 0.629 |
| walker |  | 3848 | 26 | Code::CodeKey { rung: Decl, file: plugins/convertColors.js, decl: 2, sub: 0, line: 18 } |  |  | 0.630 |
| walker |  | 3889 | 41 | Code::CodeKey { rung: Names, file: plugins/convertTransform.js, decl: 0, sub: 0, line: 0 } |  |  | 0.630 |
| walker |  | 3904 | 15 | Code::CodeKey { rung: Decl, file: plugins/convertTransform.js, decl: 2, sub: 0, line: 44 } |  |  | 0.630 |
| ns | 3956 |  | 287 | visit() -- the AST traversal contract | 2.5 |  | 0.606 |
| walker |  | 3968 | 64 | Code::CodeKey { rung: Doc, file: plugins/cleanupAttrs.js, decl: 3, sub: 0, line: 22 } |  |  | 0.606 |
| walker |  | 4010 | 42 | Code::CodeKey { rung: Names, file: plugins/cleanupNumericValues.js, decl: 0, sub: 0, line: 0 } |  |  | 0.606 |
| walker |  | 4032 | 22 | Code::CodeKey { rung: Decl, file: plugins/cleanupNumericValues.js, decl: 2, sub: 0, line: 12 } |  |  | 0.606 |
| walker |  | 4074 | 42 | Code::CodeKey { rung: Names, file: plugins/convertOneStopGradients.js, decl: 0, sub: 0, line: 0 } |  |  | 0.606 |
| walker |  | 4094 | 20 | Code::CodeKey { rung: Decl, file: plugins/convertOneStopGradients.js, decl: 2, sub: 0, line: 10 } |  |  | 0.606 |
| walker |  | 4136 | 42 | Code::CodeKey { rung: Names, file: plugins/convertPathData.js, decl: 0, sub: 0, line: 0 } |  |  | 0.607 |
| ns | 4139 |  | 183 | Anatomy of a plugin module (removeComments as exemplar) | 2.6 |  | 0.593 |
| walker |  | 4155 | 19 | Code::CodeKey { rung: Decl, file: plugins/convertPathData.js, decl: 2, sub: 0, line: 43 } |  |  | 0.593 |
| walker |  | 4197 | 42 | Code::CodeKey { rung: Names, file: plugins/moveElemsAttrsToGroup.js, decl: 0, sub: 0, line: 0 } |  |  | 0.593 |
| walker |  | 4213 | 16 | Code::CodeKey { rung: Decl, file: plugins/moveElemsAttrsToGroup.js, decl: 2, sub: 0, line: 5 } |  |  | 0.593 |
| walker |  | 4255 | 42 | Code::CodeKey { rung: Names, file: plugins/removeEditorsNSData.js, decl: 0, sub: 0, line: 0 } |  |  | 0.593 |
| walker |  | 4270 | 15 | Code::CodeKey { rung: Decl, file: plugins/removeEditorsNSData.js, decl: 2, sub: 0, line: 10 } |  |  | 0.594 |
| ns | 4291 |  | 152 | Overriding and disabling preset-default plugins (README) | 2.7 |  | 0.579 |
| walker |  | 4306 | 36 | Code::CodeKey { rung: Doc, file: plugins/removeEditorsNSData.js, decl: 1, sub: 0, line: 9 } |  |  | 0.579 |
| walker |  | 4348 | 42 | Code::CodeKey { rung: Names, file: plugins/removeHiddenElems.js, decl: 0, sub: 0, line: 0 } |  |  | 0.579 |
| walker |  | 4366 | 18 | Code::CodeKey { rung: Decl, file: plugins/removeHiddenElems.js, decl: 2, sub: 0, line: 30 } |  |  | 0.579 |
| walker |  | 4408 | 42 | Code::CodeKey { rung: Names, file: plugins/removeNonInheritableGroupAttrs.js, decl: 0, sub: 0, line: 0 } |  |  | 0.573 |
| ns | 4408 |  | 117 | Custom plugin objects (docs/05-plugins-api.mdx) | 2.8 |  | 0.573 |
| walker |  | 4425 | 17 | Code::CodeKey { rung: Decl, file: plugins/removeNonInheritableGroupAttrs.js, decl: 2, sub: 0, line: 8 } |  |  | 0.573 |
| walker |  | 4487 | 62 | Code::CodeKey { rung: Doc, file: plugins/removeNonInheritableGroupAttrs.js, decl: 3, sub: 0, line: 18 } |  |  | 0.573 |
| walker |  | 4529 | 42 | Code::CodeKey { rung: Names, file: plugins/removeScripts.js, decl: 0, sub: 0, line: 0 } |  |  | 0.573 |
| walker |  | 4535 | 6 | Fs::DirListing { dir: .yarn/plugins } |  |  | 0.573 |
| walker |  | 4578 | 43 | Code::CodeKey { rung: Names, file: plugins/removeAttributesBySelector.js, decl: 0, sub: 0, line: 0 } |  |  | 0.573 |
| walker |  | 4595 | 17 | Code::CodeKey { rung: Decl, file: plugins/removeAttributesBySelector.js, decl: 2, sub: 0, line: 22 } |  |  | 0.573 |
| ns | 4603 |  | 195 | Node entry point: loadConfig and the Node optimize wrapper | 2.9 |  | 0.562 |
| walker |  | 4638 | 43 | Code::CodeKey { rung: Names, file: plugins/removeMetadata.js, decl: 0, sub: 0, line: 0 } |  |  | 0.562 |
| walker |  | 4681 | 43 | Code::CodeKey { rung: Names, file: plugins/removeTitle.js, decl: 0, sub: 0, line: 0 } |  |  | 0.562 |
| walker |  | 4724 | 43 | Code::CodeKey { rung: Names, file: plugins/removeUnusedNS.js, decl: 0, sub: 0, line: 0 } |  |  | 0.562 |
| ns | 4786 |  | 183 | Plugin descriptions: preset-default steps 1-9 | 3.1 |  | 0.558 |
| walker |  | 4997 | 273 | Code::CodeKey { rung: Names, file: lib/types.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.573 |
| walker |  | 5006 | 9 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 5, sub: 0, line: 108 } |  |  | 0.573 |
| walker |  | 5028 | 22 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 12, sub: 0, line: 175 } |  |  | 0.573 |
| ns | 5033 |  | 247 | Plugin descriptions: preset-default steps 10-18 | 3.2 |  | 0.569 |
| walker |  | 5050 | 22 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 14, sub: 0, line: 185 } |  |  | 0.569 |
| walker |  | 5073 | 23 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 13, sub: 0, line: 180 } |  |  | 0.570 |
| walker |  | 5097 | 24 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 2, sub: 0, line: 67 } |  |  | 0.570 |
| walker |  | 5121 | 24 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 17, sub: 0, line: 205 } |  |  | 0.570 |
| walker |  | 5152 | 31 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 6, sub: 0, line: 111 } |  |  | 0.570 |
| walker |  | 5183 | 31 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 11, sub: 0, line: 169 } |  |  | 0.570 |
| walker |  | 5221 | 38 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 9, sub: 0, line: 155 } |  |  | 0.570 |
| ns | 5238 |  | 205 | Plugin descriptions: preset-default steps 19-26 | 3.3 |  | 0.569 |
| walker |  | 5266 | 45 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 15, sub: 0, line: 190 } |  |  | 0.569 |
| walker |  | 5312 | 46 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 10, sub: 0, line: 161 } |  |  | 0.570 |
| walker |  | 5365 | 53 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 8, sub: 0, line: 133 } |  |  | 0.570 |
| ns | 5405 |  | 167 | Plugin descriptions: preset-default steps 27-34 | 3.4 |  | 0.565 |
| walker |  | 5425 | 60 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 16, sub: 0, line: 197 } |  |  | 0.566 |
| walker |  | 5508 | 83 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 4, sub: 0, line: 100 } |  |  | 0.566 |
| walker |  | 5664 | 156 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 7, sub: 0, line: 117 } |  |  | 0.566 |
| ns | 5778 |  | 373 | Plugin descriptions: opt-in plugins with optional params | 3.5 |  | 0.565 |
| ns | 5895 |  | 117 | Plugin descriptions: plugins requiring params | 3.6 |  | 0.562 |
| walker |  | 5954 | 290 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 3, sub: 0, line: 71 } |  |  | 0.562 |
| walker |  | 5998 | 44 | Code::CodeKey { rung: Names, file: plugins/removeEmptyAttrs.js, decl: 0, sub: 0, line: 0 } |  |  | 0.563 |
| walker |  | 6055 | 57 | Code::CodeKey { rung: Doc, file: plugins/removeEmptyAttrs.js, decl: 3, sub: 0, line: 13 } |  |  | 0.563 |
| walker |  | 6099 | 44 | Code::CodeKey { rung: Names, file: plugins/removeRasterImages.js, decl: 0, sub: 0, line: 0 } |  |  | 0.565 |
| ns | 6127 |  | 232 | Config and Output type bodies | 4.1 |  | 0.553 |
| walker |  | 6143 | 44 | Code::CodeKey { rung: Names, file: plugins/removeUnknownsAndDefaults.js, decl: 0, sub: 0, line: 0 } |  |  | 0.554 |
| walker |  | 6163 | 20 | Code::CodeKey { rung: Decl, file: plugins/removeUnknownsAndDefaults.js, decl: 2, sub: 0, line: 32 } |  |  | 0.556 |
| walker |  | 6208 | 45 | Code::CodeKey { rung: Names, file: plugins/collapseGroups.js, decl: 0, sub: 0, line: 0 } |  |  | 0.558 |
| walker |  | 6253 | 45 | Code::CodeKey { rung: Names, file: plugins/mergeStyles.js, decl: 0, sub: 0, line: 0 } |  |  | 0.560 |
| walker |  | 6316 | 63 | Code::CodeKey { rung: Doc, file: plugins/mergeStyles.js, decl: 3, sub: 0, line: 14 } |  |  | 0.560 |
| walker |  | 6361 | 45 | Code::CodeKey { rung: Names, file: plugins/removeDeprecatedAttrs.js, decl: 0, sub: 0, line: 0 } |  |  | 0.561 |
| walker |  | 6396 | 35 | Code::CodeKey { rung: Doc, file: plugins/removeDeprecatedAttrs.js, decl: 1, sub: 0, line: 10 } |  |  | 0.561 |
| ns | 6440 |  | 313 | Xast node type bodies | 4.2 |  | 0.578 |
| walker |  | 6441 | 45 | Code::CodeKey { rung: Doc, file: plugins/removeDeprecatedAttrs.js, decl: 3, sub: 0, line: 76 } |  |  | 0.578 |
| walker |  | 6486 | 45 | Code::CodeKey { rung: Names, file: plugins/removeDoctype.js, decl: 0, sub: 0, line: 0 } |  |  | 0.579 |
| walker |  | 6549 | 63 | Code::CodeKey { rung: Body, file: plugins/removeDoctype.js, decl: 3, sub: 0, line: 30 } |  |  | 0.579 |
| walker |  | 6594 | 45 | Code::CodeKey { rung: Names, file: plugins/removeViewBox.js, decl: 0, sub: 0, line: 0 } |  |  | 0.581 |
| walker |  | 6640 | 46 | Code::CodeKey { rung: Names, file: plugins/removeComments.js, decl: 0, sub: 0, line: 0 } |  |  | 0.584 |
| ns | 6652 |  | 212 | Visitor keys, VisitorNode, PluginInfo and Plugin bodies | 4.3 |  | 0.573 |
| walker |  | 6683 | 43 | Code::CodeKey { rung: Doc, file: plugins/removeComments.js, decl: 1, sub: 0, line: 8 } |  |  | 0.576 |
| walker |  | 6729 | 46 | Code::CodeKey { rung: Names, file: plugins/removeStyleElement.js, decl: 0, sub: 0, line: 0 } |  |  | 0.577 |
| walker |  | 6775 | 46 | Code::CodeKey { rung: Names, file: plugins/removeXMLNS.js, decl: 0, sub: 0, line: 0 } |  |  | 0.580 |
| ns | 6809 |  | 157 | coa.js top-level function roster | 5.1 |  | 0.572 |
| walker |  | 6821 | 46 | Code::CodeKey { rung: Names, file: plugins/removeXMLProcInst.js, decl: 0, sub: 0, line: 0 } |  |  | 0.575 |
| ns | 6933 |  | 124 | CLI program metadata and positional argument | 5.2 |  | 0.571 |
| walker |  | 6950 | 129 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.571 |
| ns | 7223 |  | 290 | CLI options, first half | 5.3 |  | 0.560 |
| walker |  | 7229 | 279 | Code::CodeKey { rung: Names, file: lib/types.ts, decl: 0, sub: 1, line: 0 } |  |  | 0.593 |
| walker |  | 7241 | 12 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 37, sub: 0, line: 357 } |  |  | 0.593 |
| walker |  | 7263 | 22 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 31, sub: 0, line: 300 } |  |  | 0.593 |
| walker |  | 7286 | 23 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 24, sub: 0, line: 263 } |  |  | 0.593 |
| walker |  | 7309 | 23 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 34, sub: 0, line: 329 } |  |  | 0.593 |
| walker |  | 7339 | 30 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 27, sub: 0, line: 276 } |  |  | 0.593 |
| walker |  | 7369 | 30 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 29, sub: 0, line: 289 } |  |  | 0.593 |
| walker |  | 7400 | 31 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 30, sub: 0, line: 294 } |  |  | 0.593 |
| walker |  | 7440 | 40 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 25, sub: 0, line: 268 } |  |  | 0.595 |
| walker |  | 7482 | 42 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 28, sub: 0, line: 282 } |  |  | 0.595 |
| walker |  | 7529 | 47 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 22, sub: 0, line: 248 } |  |  | 0.595 |
| ns | 7567 |  | 344 | CLI options, second half | 5.4 |  | 0.582 |
| walker |  | 7578 | 49 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 21, sub: 0, line: 243 } |  |  | 0.586 |
| walker |  | 7673 | 95 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 23, sub: 0, line: 253 } |  |  | 0.600 |
| ns | 7675 |  | 108 | bin/svgo.js executable entry | 5.5 |  | 0.595 |
| ns | 7790 |  | 115 | parseSvg and the sax configuration | 6.1 |  | 0.590 |
| walker |  | 7851 | 178 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 33, sub: 0, line: 307 } |  |  | 0.590 |
| ns | 7913 |  | 123 | stringifier function roster | 6.2 |  | 0.585 |
| ns | 7996 |  | 83 | xast query helpers | 6.3 |  | 0.583 |
| walker |  | 8071 | 220 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 36, sub: 0, line: 336 } |  |  | 0.605 |
| ns | 8174 |  | 178 | lib/svgo/tools.js -- complete export set | 6.4 |  | 0.600 |
| ns | 8330 |  | 156 | lib/style.js -- exported and internal function roster | 6.5 |  | 0.594 |
| walker |  | 8363 | 292 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 20, sub: 0, line: 214 } |  |  | 0.594 |
| ns | 8476 |  | 146 | lib/path.js -- path data parse and stringify roster | 6.6 |  | 0.588 |
| walker |  | 8579 | 216 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 1, sub: 0, line: 30 } |  |  | 0.588 |
| ns | 8710 |  | 234 | plugins/_collections.js -- complete export roster | 6.7 |  | 0.581 |
| walker |  | 8802 | 223 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 1, sub: 1, line: 30 } |  |  | 0.581 |
| ns | 8904 |  | 194 | Shared plugin helper module rosters | 6.8 |  | 0.577 |
| ns | 9017 |  | 113 | css-select adapter | 6.9 |  | 0.573 |
| ns | 9034 |  | 17 | mapNodesToParents | 6.10 |  | 0.572 |
| walker |  | 9036 | 234 | Code::CodeKey { rung: Names, file: plugins/_collections.js, decl: 0, sub: 0, line: 0 } |  |  | 0.586 |
| walker |  | 9100 | 64 | Code::CodeKey { rung: Decl, file: plugins/_collections.js, decl: 14, sub: 0, line: 2387 } |  |  | 0.586 |
| walker |  | 9182 | 82 | Code::CodeKey { rung: Decl, file: plugins/_collections.js, decl: 11, sub: 0, line: 2179 } |  |  | 0.586 |
| ns | 9268 |  | 234 | package.json scripts -- how to build, test and lint | 7.1 |  | 0.590 |
| walker |  | 9283 | 101 | Code::CodeKey { rung: Decl, file: plugins/_collections.js, decl: 9, sub: 0, line: 2111 } |  |  | 0.590 |
| walker |  | 9305 | 22 | Code::CodeKey { rung: Doc, file: plugins/_collections.js, decl: 3, sub: 0, line: 117 } |  |  | 0.590 |
| ns | 9337 |  | 69 | test/ tree listing | 7.2 |  | 0.597 |
| ns | 9452 |  | 115 | The fixture-driven plugin test format | 7.3 |  | 0.595 |
| walker |  | 9463 | 158 | Code::CodeKey { rung: Decl, file: plugins/_collections.js, decl: 6, sub: 0, line: 389 } |  |  | 0.595 |
| walker |  | 9490 | 27 | Code::CodeKey { rung: Doc, file: plugins/_collections.js, decl: 11, sub: 0, line: 2179 } |  |  | 0.595 |
| walker |  | 9542 | 52 | Code::CodeKey { rung: Doc, file: plugins/_collections.js, decl: 9, sub: 0, line: 2111 } |  |  | 0.595 |
| ns | 9584 |  | 132 | docs/ and remaining leaf directory listings | 7.4 |  | 0.602 |
| walker |  | 9598 | 56 | Code::CodeKey { rung: Doc, file: plugins/_collections.js, decl: 14, sub: 0, line: 2387 } |  |  | 0.602 |
| walker |  | 9657 | 59 | Code::CodeKey { rung: Doc, file: plugins/_collections.js, decl: 6, sub: 0, line: 389 } |  |  | 0.602 |
| ns | 9719 |  | 135 | Migration guide outlines | 7.5 |  | 0.598 |
| walker |  | 9730 | 73 | Code::CodeKey { rung: Doc, file: plugins/cleanupNumericValues.js, decl: 3, sub: 0, line: 35 } |  |  | 0.598 |
| walker |  | 9803 | 73 | Code::CodeKey { rung: Doc, file: plugins/removeScripts.js, decl: 3, sub: 0, line: 24 } |  |  | 0.598 |
| ns | 9843 |  | 124 | TypeScript and lint configuration | 7.6 |  | 0.594 |
| walker |  | 9876 | 73 | Code::CodeKey { rung: Doc, file: plugins/removeUnusedNS.js, decl: 3, sub: 0, line: 12 } |  |  | 0.594 |
| walker |  | 9923 | 47 | Code::CodeKey { rung: Names, file: plugins/prefixIds.js, decl: 0, sub: 0, line: 0 } |  |  | 0.595 |
| ns | 9968 |  | 125 | Runtime dependencies | 7.7 |  | 0.598 |
| walker |  | 9984 | 61 | Code::CodeKey { rung: Doc, file: plugins/prefixIds.js, decl: 3, sub: 0, line: 129 } |  |  | 0.598 |
| walker |  | 10000 | 16 | Code::CodeKey { rung: Names, file: plugins/removeAttrs.js, decl: 0, sub: 0, line: 0 } |  |  | 0.598 |

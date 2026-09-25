Score(3000)=0.649 I=0.865 C=0.488 ns_rows≤3K=14/54 grid(1000/1442/2080/3000/4327/6240/9000)=0.626/0.714/0.691/0.649/0.579/0.554/0.573

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
| walker |  | 1835 | 7 | Code::CodeKey { rung: Names, file: lib/types.js, decl: 0, sub: 0, line: 0 } |  |  | 0.725 |
| ns | 1873 |  | 204 | optimize() body, part 2: overrides, invoke, stringify, datauri | 1.11 | 1.10 | 0.687 |
| walker |  | 2062 | 227 | Json::Entry { file: package.json } |  |  | 0.691 |
| walker |  | 2111 | 49 | Fs::DirListing { dir: test/fixtures/config-loader } |  |  | 0.691 |
| ns | 2423 |  | 550 | Complete exported type roster of lib/types.ts | 1.12 |  | 0.631 |
| walker |  | 2499 | 388 | Fs::DirListing { dir: docs/04-plugins } |  |  | 0.631 |
| walker |  | 2556 | 57 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.631 |
| walker |  | 2591 | 35 | Code::CodeKey { rung: Names, file: plugins/removeDimensions.js, decl: 0, sub: 0, line: 0 } |  |  | 0.631 |
| walker |  | 2615 | 24 | Code::CodeKey { rung: Decl, file: plugins/removeDimensions.js, decl: 2, sub: 0, line: 2 } |  |  | 0.631 |
| walker |  | 2639 | 24 | Code::CodeKey { rung: Names, file: plugins/preset-default.js, decl: 0, sub: 0, line: 0 } |  |  | 0.631 |
| ns | 2691 |  | 268 | Package identity, entry points and engines | 1.13 |  | 0.651 |
| ns | 2755 |  | 64 | Version constant | 1.14 |  | 0.642 |
| walker |  | 2980 | 341 | Code::CodeKey { rung: Decl, file: plugins/preset-default.js, decl: 1, sub: 0, line: 37 } |  |  | 0.649 |
| walker |  | 3019 | 39 | Code::CodeKey { rung: Names, file: plugins/removeOffCanvasPaths.js, decl: 0, sub: 0, line: 0 } |  |  | 0.649 |
| walker |  | 3037 | 18 | Code::CodeKey { rung: Decl, file: plugins/removeOffCanvasPaths.js, decl: 2, sub: 0, line: 7 } |  |  | 0.650 |
| walker |  | 3076 | 39 | Code::CodeKey { rung: Names, file: plugins/reusePaths.js, decl: 0, sub: 0, line: 0 } |  |  | 0.650 |
| ns | 3125 |  | 370 | preset-default: the complete default pipeline in execution order | 2.1 |  | 0.681 |
| walker |  | 3131 | 55 | Code::CodeKey { rung: Decl, file: plugins/reusePaths.js, decl: 2, sub: 0, line: 5 } |  |  | 0.681 |
| walker |  | 3192 | 61 | Code::CodeKey { rung: Doc, file: plugins/removeOffCanvasPaths.js, decl: 3, sub: 0, line: 17 } |  |  | 0.681 |
| ns | 3259 |  | 134 | builtinPlugins registry | 2.2 |  | 0.670 |
| ns | 3448 |  | 189 | invokePlugins -- the plugin engine | 2.3 |  | 0.648 |
| walker |  | 3573 | 381 | Json::Scripts { file: package.json } |  |  | 0.649 |
| walker |  | 3613 | 40 | Code::CodeKey { rung: Names, file: plugins/cleanupEnableBackground.js, decl: 0, sub: 0, line: 0 } |  |  | 0.649 |
| walker |  | 3628 | 15 | Code::CodeKey { rung: Decl, file: plugins/cleanupEnableBackground.js, decl: 2, sub: 0, line: 5 } |  |  | 0.649 |
| walker |  | 3668 | 40 | Code::CodeKey { rung: Names, file: plugins/moveGroupAttrsToElems.js, decl: 0, sub: 0, line: 0 } |  |  | 0.649 |
| ns | 3669 |  | 221 | createPreset -- how a preset validates and forwards overrides | 2.4 |  | 0.629 |
| walker |  | 3683 | 15 | Code::CodeKey { rung: Decl, file: plugins/moveGroupAttrsToElems.js, decl: 2, sub: 0, line: 5 } |  |  | 0.629 |
| walker |  | 3723 | 40 | Code::CodeKey { rung: Names, file: plugins/removeXlink.js, decl: 0, sub: 0, line: 0 } |  |  | 0.629 |
| walker |  | 3745 | 22 | Code::CodeKey { rung: Decl, file: plugins/removeXlink.js, decl: 2, sub: 0, line: 11 } |  |  | 0.629 |
| walker |  | 3786 | 41 | Code::CodeKey { rung: Names, file: plugins/cleanupAttrs.js, decl: 0, sub: 0, line: 0 } |  |  | 0.629 |
| walker |  | 3804 | 18 | Code::CodeKey { rung: Decl, file: plugins/cleanupAttrs.js, decl: 2, sub: 0, line: 9 } |  |  | 0.629 |
| walker |  | 3860 | 56 | Code::CodeKey { rung: Doc, file: plugins/cleanupAttrs.js, decl: 1, sub: 0, line: 8 } |  |  | 0.629 |
| walker |  | 3901 | 41 | Code::CodeKey { rung: Names, file: plugins/convertColors.js, decl: 0, sub: 0, line: 0 } |  |  | 0.629 |
| walker |  | 3927 | 26 | Code::CodeKey { rung: Decl, file: plugins/convertColors.js, decl: 2, sub: 0, line: 18 } |  |  | 0.630 |
| ns | 3956 |  | 287 | visit() -- the AST traversal contract | 2.5 |  | 0.606 |
| walker |  | 3968 | 41 | Code::CodeKey { rung: Names, file: plugins/convertTransform.js, decl: 0, sub: 0, line: 0 } |  |  | 0.606 |
| walker |  | 3983 | 15 | Code::CodeKey { rung: Decl, file: plugins/convertTransform.js, decl: 2, sub: 0, line: 44 } |  |  | 0.606 |
| walker |  | 4047 | 64 | Code::CodeKey { rung: Doc, file: plugins/cleanupAttrs.js, decl: 3, sub: 0, line: 22 } |  |  | 0.606 |
| walker |  | 4089 | 42 | Code::CodeKey { rung: Names, file: plugins/cleanupNumericValues.js, decl: 0, sub: 0, line: 0 } |  |  | 0.606 |
| walker |  | 4111 | 22 | Code::CodeKey { rung: Decl, file: plugins/cleanupNumericValues.js, decl: 2, sub: 0, line: 12 } |  |  | 0.606 |
| ns | 4139 |  | 183 | Anatomy of a plugin module (removeComments as exemplar) | 2.6 |  | 0.593 |
| walker |  | 4153 | 42 | Code::CodeKey { rung: Names, file: plugins/convertOneStopGradients.js, decl: 0, sub: 0, line: 0 } |  |  | 0.593 |
| walker |  | 4173 | 20 | Code::CodeKey { rung: Decl, file: plugins/convertOneStopGradients.js, decl: 2, sub: 0, line: 10 } |  |  | 0.593 |
| walker |  | 4215 | 42 | Code::CodeKey { rung: Names, file: plugins/convertPathData.js, decl: 0, sub: 0, line: 0 } |  |  | 0.593 |
| walker |  | 4234 | 19 | Code::CodeKey { rung: Decl, file: plugins/convertPathData.js, decl: 2, sub: 0, line: 43 } |  |  | 0.593 |
| walker |  | 4276 | 42 | Code::CodeKey { rung: Names, file: plugins/moveElemsAttrsToGroup.js, decl: 0, sub: 0, line: 0 } |  |  | 0.593 |
| ns | 4291 |  | 152 | Overriding and disabling preset-default plugins (README) | 2.7 |  | 0.579 |
| walker |  | 4292 | 16 | Code::CodeKey { rung: Decl, file: plugins/moveElemsAttrsToGroup.js, decl: 2, sub: 0, line: 5 } |  |  | 0.579 |
| walker |  | 4334 | 42 | Code::CodeKey { rung: Names, file: plugins/removeEditorsNSData.js, decl: 0, sub: 0, line: 0 } |  |  | 0.579 |
| walker |  | 4349 | 15 | Code::CodeKey { rung: Decl, file: plugins/removeEditorsNSData.js, decl: 2, sub: 0, line: 10 } |  |  | 0.579 |
| walker |  | 4385 | 36 | Code::CodeKey { rung: Doc, file: plugins/removeEditorsNSData.js, decl: 1, sub: 0, line: 9 } |  |  | 0.579 |
| ns | 4408 |  | 117 | Custom plugin objects (docs/05-plugins-api.mdx) | 2.8 |  | 0.573 |
| walker |  | 4427 | 42 | Code::CodeKey { rung: Names, file: plugins/removeHiddenElems.js, decl: 0, sub: 0, line: 0 } |  |  | 0.573 |
| walker |  | 4445 | 18 | Code::CodeKey { rung: Decl, file: plugins/removeHiddenElems.js, decl: 2, sub: 0, line: 30 } |  |  | 0.573 |
| walker |  | 4487 | 42 | Code::CodeKey { rung: Names, file: plugins/removeNonInheritableGroupAttrs.js, decl: 0, sub: 0, line: 0 } |  |  | 0.573 |
| walker |  | 4504 | 17 | Code::CodeKey { rung: Decl, file: plugins/removeNonInheritableGroupAttrs.js, decl: 2, sub: 0, line: 8 } |  |  | 0.573 |
| walker |  | 4566 | 62 | Code::CodeKey { rung: Doc, file: plugins/removeNonInheritableGroupAttrs.js, decl: 3, sub: 0, line: 18 } |  |  | 0.573 |
| ns | 4603 |  | 195 | Node entry point: loadConfig and the Node optimize wrapper | 2.9 |  | 0.562 |
| walker |  | 4608 | 42 | Code::CodeKey { rung: Names, file: plugins/removeScripts.js, decl: 0, sub: 0, line: 0 } |  |  | 0.562 |
| walker |  | 4614 | 6 | Fs::DirListing { dir: .yarn/plugins } |  |  | 0.562 |
| walker |  | 4657 | 43 | Code::CodeKey { rung: Names, file: plugins/removeAttributesBySelector.js, decl: 0, sub: 0, line: 0 } |  |  | 0.562 |
| walker |  | 4674 | 17 | Code::CodeKey { rung: Decl, file: plugins/removeAttributesBySelector.js, decl: 2, sub: 0, line: 22 } |  |  | 0.562 |
| walker |  | 4717 | 43 | Code::CodeKey { rung: Names, file: plugins/removeMetadata.js, decl: 0, sub: 0, line: 0 } |  |  | 0.562 |
| walker |  | 4760 | 43 | Code::CodeKey { rung: Names, file: plugins/removeTitle.js, decl: 0, sub: 0, line: 0 } |  |  | 0.562 |
| ns | 4786 |  | 183 | Plugin descriptions: preset-default steps 1-9 | 3.1 |  | 0.558 |
| walker |  | 4803 | 43 | Code::CodeKey { rung: Names, file: plugins/removeUnusedNS.js, decl: 0, sub: 0, line: 0 } |  |  | 0.558 |
| ns | 5033 |  | 247 | Plugin descriptions: preset-default steps 10-18 | 3.2 |  | 0.554 |
| walker |  | 5076 | 273 | Code::CodeKey { rung: Names, file: lib/types.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.569 |
| walker |  | 5085 | 9 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 5, sub: 0, line: 108 } |  |  | 0.569 |
| walker |  | 5107 | 22 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 12, sub: 0, line: 175 } |  |  | 0.569 |
| walker |  | 5129 | 22 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 14, sub: 0, line: 185 } |  |  | 0.569 |
| walker |  | 5152 | 23 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 13, sub: 0, line: 180 } |  |  | 0.570 |
| walker |  | 5176 | 24 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 2, sub: 0, line: 67 } |  |  | 0.570 |
| walker |  | 5200 | 24 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 17, sub: 0, line: 205 } |  |  | 0.570 |
| walker |  | 5231 | 31 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 6, sub: 0, line: 111 } |  |  | 0.570 |
| ns | 5238 |  | 205 | Plugin descriptions: preset-default steps 19-26 | 3.3 |  | 0.568 |
| walker |  | 5262 | 31 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 11, sub: 0, line: 169 } |  |  | 0.569 |
| walker |  | 5300 | 38 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 9, sub: 0, line: 155 } |  |  | 0.569 |
| walker |  | 5345 | 45 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 15, sub: 0, line: 190 } |  |  | 0.569 |
| walker |  | 5391 | 46 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 10, sub: 0, line: 161 } |  |  | 0.570 |
| ns | 5405 |  | 167 | Plugin descriptions: preset-default steps 27-34 | 3.4 |  | 0.565 |
| walker |  | 5444 | 53 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 8, sub: 0, line: 133 } |  |  | 0.565 |
| walker |  | 5504 | 60 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 16, sub: 0, line: 197 } |  |  | 0.566 |
| walker |  | 5587 | 83 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 4, sub: 0, line: 100 } |  |  | 0.566 |
| walker |  | 5743 | 156 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 7, sub: 0, line: 117 } |  |  | 0.566 |
| ns | 5778 |  | 373 | Plugin descriptions: opt-in plugins with optional params | 3.5 |  | 0.565 |
| ns | 5895 |  | 117 | Plugin descriptions: plugins requiring params | 3.6 |  | 0.562 |
| walker |  | 6033 | 290 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 3, sub: 0, line: 71 } |  |  | 0.562 |
| walker |  | 6077 | 44 | Code::CodeKey { rung: Names, file: plugins/removeEmptyAttrs.js, decl: 0, sub: 0, line: 0 } |  |  | 0.563 |
| ns | 6127 |  | 232 | Config and Output type bodies | 4.1 |  | 0.551 |
| walker |  | 6134 | 57 | Code::CodeKey { rung: Doc, file: plugins/removeEmptyAttrs.js, decl: 3, sub: 0, line: 13 } |  |  | 0.551 |
| walker |  | 6178 | 44 | Code::CodeKey { rung: Names, file: plugins/removeRasterImages.js, decl: 0, sub: 0, line: 0 } |  |  | 0.553 |
| walker |  | 6222 | 44 | Code::CodeKey { rung: Names, file: plugins/removeUnknownsAndDefaults.js, decl: 0, sub: 0, line: 0 } |  |  | 0.554 |
| walker |  | 6242 | 20 | Code::CodeKey { rung: Decl, file: plugins/removeUnknownsAndDefaults.js, decl: 2, sub: 0, line: 32 } |  |  | 0.556 |
| walker |  | 6272 | 30 | Code::CodeKey { rung: Names, file: lib/path.js, decl: 0, sub: 0, line: 0 } |  |  | 0.556 |
| walker |  | 6304 | 32 | Code::CodeKey { rung: Decl, file: lib/path.js, decl: 2, sub: 0, line: 302 } |  |  | 0.556 |
| walker |  | 6340 | 36 | Code::CodeKey { rung: Doc, file: lib/path.js, decl: 2, sub: 0, line: 302 } |  |  | 0.556 |
| walker |  | 6380 | 40 | Code::CodeKey { rung: Doc, file: lib/path.js, decl: 1, sub: 0, line: 141 } |  |  | 0.556 |
| walker |  | 6425 | 45 | Code::CodeKey { rung: Names, file: plugins/collapseGroups.js, decl: 0, sub: 0, line: 0 } |  |  | 0.558 |
| ns | 6440 |  | 313 | Xast node type bodies | 4.2 |  | 0.575 |
| walker |  | 6470 | 45 | Code::CodeKey { rung: Names, file: plugins/mergeStyles.js, decl: 0, sub: 0, line: 0 } |  |  | 0.576 |
| walker |  | 6533 | 63 | Code::CodeKey { rung: Doc, file: plugins/mergeStyles.js, decl: 3, sub: 0, line: 14 } |  |  | 0.576 |
| walker |  | 6578 | 45 | Code::CodeKey { rung: Names, file: plugins/removeDeprecatedAttrs.js, decl: 0, sub: 0, line: 0 } |  |  | 0.578 |
| walker |  | 6613 | 35 | Code::CodeKey { rung: Doc, file: plugins/removeDeprecatedAttrs.js, decl: 1, sub: 0, line: 10 } |  |  | 0.578 |
| ns | 6652 |  | 212 | Visitor keys, VisitorNode, PluginInfo and Plugin bodies | 4.3 |  | 0.567 |
| walker |  | 6658 | 45 | Code::CodeKey { rung: Doc, file: plugins/removeDeprecatedAttrs.js, decl: 3, sub: 0, line: 76 } |  |  | 0.567 |
| walker |  | 6703 | 45 | Code::CodeKey { rung: Names, file: plugins/removeDoctype.js, decl: 0, sub: 0, line: 0 } |  |  | 0.569 |
| walker |  | 6766 | 63 | Code::CodeKey { rung: Body, file: plugins/removeDoctype.js, decl: 3, sub: 0, line: 30 } |  |  | 0.569 |
| ns | 6809 |  | 157 | coa.js top-level function roster | 5.1 |  | 0.562 |
| walker |  | 6811 | 45 | Code::CodeKey { rung: Names, file: plugins/removeViewBox.js, decl: 0, sub: 0, line: 0 } |  |  | 0.564 |
| walker |  | 6857 | 46 | Code::CodeKey { rung: Names, file: plugins/removeComments.js, decl: 0, sub: 0, line: 0 } |  |  | 0.566 |
| walker |  | 6900 | 43 | Code::CodeKey { rung: Doc, file: plugins/removeComments.js, decl: 1, sub: 0, line: 8 } |  |  | 0.568 |
| ns | 6933 |  | 124 | CLI program metadata and positional argument | 5.2 |  | 0.564 |
| walker |  | 6946 | 46 | Code::CodeKey { rung: Names, file: plugins/removeStyleElement.js, decl: 0, sub: 0, line: 0 } |  |  | 0.566 |
| walker |  | 6992 | 46 | Code::CodeKey { rung: Names, file: plugins/removeXMLNS.js, decl: 0, sub: 0, line: 0 } |  |  | 0.569 |
| walker |  | 7038 | 46 | Code::CodeKey { rung: Names, file: plugins/removeXMLProcInst.js, decl: 0, sub: 0, line: 0 } |  |  | 0.571 |
| walker |  | 7167 | 129 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.571 |
| ns | 7223 |  | 290 | CLI options, first half | 5.3 |  | 0.560 |
| walker |  | 7399 | 232 | Code::CodeKey { rung: Names, file: plugins/_collections.js, decl: 0, sub: 0, line: 0 } |  |  | 0.561 |
| walker |  | 7463 | 64 | Code::CodeKey { rung: Decl, file: plugins/_collections.js, decl: 14, sub: 0, line: 2387 } |  |  | 0.561 |
| walker |  | 7545 | 82 | Code::CodeKey { rung: Decl, file: plugins/_collections.js, decl: 11, sub: 0, line: 2179 } |  |  | 0.561 |
| ns | 7567 |  | 344 | CLI options, second half | 5.4 |  | 0.548 |
| walker |  | 7646 | 101 | Code::CodeKey { rung: Decl, file: plugins/_collections.js, decl: 9, sub: 0, line: 2111 } |  |  | 0.548 |
| walker |  | 7668 | 22 | Code::CodeKey { rung: Doc, file: plugins/_collections.js, decl: 3, sub: 0, line: 117 } |  |  | 0.548 |
| ns | 7675 |  | 108 | bin/svgo.js executable entry | 5.5 |  | 0.544 |
| ns | 7790 |  | 115 | parseSvg and the sax configuration | 6.1 |  | 0.539 |
| walker |  | 7826 | 158 | Code::CodeKey { rung: Decl, file: plugins/_collections.js, decl: 6, sub: 0, line: 389 } |  |  | 0.539 |
| walker |  | 7853 | 27 | Code::CodeKey { rung: Doc, file: plugins/_collections.js, decl: 11, sub: 0, line: 2179 } |  |  | 0.539 |
| walker |  | 7905 | 52 | Code::CodeKey { rung: Doc, file: plugins/_collections.js, decl: 9, sub: 0, line: 2111 } |  |  | 0.539 |
| ns | 7913 |  | 123 | stringifier function roster | 6.2 |  | 0.535 |
| walker |  | 7961 | 56 | Code::CodeKey { rung: Doc, file: plugins/_collections.js, decl: 14, sub: 0, line: 2387 } |  |  | 0.535 |
| ns | 7996 |  | 83 | xast query helpers | 6.3 |  | 0.533 |
| walker |  | 8020 | 59 | Code::CodeKey { rung: Doc, file: plugins/_collections.js, decl: 6, sub: 0, line: 389 } |  |  | 0.533 |
| ns | 8174 |  | 178 | lib/svgo/tools.js -- complete export set | 6.4 |  | 0.529 |
| walker |  | 8299 | 279 | Code::CodeKey { rung: Names, file: lib/types.ts, decl: 0, sub: 1, line: 0 } |  |  | 0.560 |
| walker |  | 8311 | 12 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 37, sub: 0, line: 357 } |  |  | 0.560 |
| ns | 8330 |  | 156 | lib/style.js -- exported and internal function roster | 6.5 |  | 0.555 |
| walker |  | 8333 | 22 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 31, sub: 0, line: 300 } |  |  | 0.555 |
| walker |  | 8356 | 23 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 24, sub: 0, line: 263 } |  |  | 0.555 |
| walker |  | 8379 | 23 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 34, sub: 0, line: 329 } |  |  | 0.555 |
| walker |  | 8409 | 30 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 27, sub: 0, line: 276 } |  |  | 0.555 |
| walker |  | 8439 | 30 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 29, sub: 0, line: 289 } |  |  | 0.555 |
| walker |  | 8470 | 31 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 30, sub: 0, line: 294 } |  |  | 0.555 |
| ns | 8476 |  | 146 | lib/path.js -- path data parse and stringify roster | 6.6 |  | 0.552 |
| walker |  | 8510 | 40 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 25, sub: 0, line: 268 } |  |  | 0.553 |
| walker |  | 8552 | 42 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 28, sub: 0, line: 282 } |  |  | 0.553 |
| walker |  | 8599 | 47 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 22, sub: 0, line: 248 } |  |  | 0.553 |
| walker |  | 8648 | 49 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 21, sub: 0, line: 243 } |  |  | 0.557 |
| ns | 8710 |  | 234 | plugins/_collections.js -- complete export roster | 6.7 |  | 0.564 |
| walker |  | 8743 | 95 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 23, sub: 0, line: 253 } |  |  | 0.578 |
| ns | 8904 |  | 194 | Shared plugin helper module rosters | 6.8 |  | 0.573 |
| walker |  | 8921 | 178 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 33, sub: 0, line: 307 } |  |  | 0.573 |
| ns | 9017 |  | 113 | css-select adapter | 6.9 |  | 0.569 |
| ns | 9034 |  | 17 | mapNodesToParents | 6.10 |  | 0.569 |
| walker |  | 9141 | 220 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 36, sub: 0, line: 336 } |  |  | 0.589 |
| ns | 9268 |  | 234 | package.json scripts -- how to build, test and lint | 7.1 |  | 0.593 |
| ns | 9337 |  | 69 | test/ tree listing | 7.2 |  | 0.600 |
| walker |  | 9433 | 292 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 20, sub: 0, line: 214 } |  |  | 0.600 |
| ns | 9452 |  | 115 | The fixture-driven plugin test format | 7.3 |  | 0.597 |
| ns | 9584 |  | 132 | docs/ and remaining leaf directory listings | 7.4 |  | 0.604 |
| walker |  | 9649 | 216 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 1, sub: 0, line: 30 } |  |  | 0.604 |
| ns | 9719 |  | 135 | Migration guide outlines | 7.5 |  | 0.600 |
| ns | 9843 |  | 124 | TypeScript and lint configuration | 7.6 |  | 0.596 |
| walker |  | 9872 | 223 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 1, sub: 1, line: 30 } |  |  | 0.596 |
| walker |  | 9945 | 73 | Code::CodeKey { rung: Doc, file: plugins/cleanupNumericValues.js, decl: 3, sub: 0, line: 35 } |  |  | 0.596 |
| ns | 9968 |  | 125 | Runtime dependencies | 7.7 |  | 0.599 |

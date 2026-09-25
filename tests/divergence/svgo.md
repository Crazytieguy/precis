Score(3000)=0.676 I=0.875 C=0.522 ns_rows≤3K=14/54 grid(1000/1442/2080/3000/4327/6240/9000)=0.736/0.741/0.728/0.676/0.603/0.571/0.617

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
| walker |  | 812 | 55 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.815 |
| walker |  | 846 | 34 | Fs::DirListing { dir: test } |  |  | 0.816 |
| walker |  | 850 | 4 | Fs::DirListing { dir: test/fixtures } |  |  | 0.816 |
| walker |  | 869 | 19 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.816 |
| ns | 875 |  | 160 | Command-line usage (README) | 1.7 |  | 0.736 |
| walker |  | 1153 | 284 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: true } |  |  | 0.736 |
| ns | 1231 |  | 356 | Complete plugins/ listing (all 58 modules) | 1.8 |  | 0.570 |
| ns | 1470 |  | 239 | svgo.config.mjs shape (README Configuration) | 1.9 |  | 0.526 |
| walker |  | 1509 | 356 | Fs::DirListing { dir: plugins } |  |  | 0.759 |
| walker |  | 1602 | 93 | Code::CodeKey { rung: Names, file: lib/svgo.js, decl: 0, sub: 0, line: 0 } |  |  | 0.768 |
| walker |  | 1659 | 57 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.768 |
| ns | 1669 |  | 199 | optimize() body, part 1: multipass loop and plugin resolution | 1.10 | 1.4 | 0.732 |
| walker |  | 1694 | 35 | Fs::DirListing { dir: test/regression } |  |  | 0.733 |
| walker |  | 1762 | 68 | Code::CodeKey { rung: Doc, file: lib/svgo.js, decl: 1, sub: 0, line: 81 } |  |  | 0.765 |
| ns | 1873 |  | 204 | optimize() body, part 2: overrides, invoke, stringify, datauri | 1.11 | 1.10 | 0.725 |
| walker |  | 1967 | 205 | Json::Dependencies { file: package.json } |  |  | 0.726 |
| walker |  | 1974 | 7 | Code::CodeKey { rung: Names, file: lib/types.js, decl: 0, sub: 0, line: 0 } |  |  | 0.726 |
| walker |  | 2201 | 227 | Json::Entry { file: package.json } |  |  | 0.730 |
| walker |  | 2250 | 49 | Fs::DirListing { dir: test/fixtures/config-loader } |  |  | 0.730 |
| ns | 2423 |  | 550 | Complete exported type roster of lib/types.ts | 1.12 |  | 0.666 |
| walker |  | 2638 | 388 | Fs::DirListing { dir: docs/04-plugins } |  |  | 0.666 |
| ns | 2691 |  | 268 | Package identity, entry points and engines | 1.13 |  | 0.684 |
| ns | 2755 |  | 64 | Version constant | 1.14 |  | 0.674 |
| walker |  | 2767 | 129 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.674 |
| walker |  | 2802 | 35 | Code::CodeKey { rung: Names, file: plugins/removeDimensions.js, decl: 0, sub: 0, line: 0 } |  |  | 0.674 |
| walker |  | 2826 | 24 | Code::CodeKey { rung: Decl, file: plugins/removeDimensions.js, decl: 2, sub: 0, line: 2 } |  |  | 0.674 |
| walker |  | 2850 | 24 | Code::CodeKey { rung: Names, file: plugins/preset-default.js, decl: 0, sub: 0, line: 0 } |  |  | 0.674 |
| ns | 3125 |  | 370 | preset-default: the complete default pipeline in execution order | 2.1 |  | 0.625 |
| walker |  | 3191 | 341 | Code::CodeKey { rung: Decl, file: plugins/preset-default.js, decl: 1, sub: 0, line: 37 } |  |  | 0.710 |
| walker |  | 3230 | 39 | Code::CodeKey { rung: Names, file: plugins/removeOffCanvasPaths.js, decl: 0, sub: 0, line: 0 } |  |  | 0.710 |
| walker |  | 3248 | 18 | Code::CodeKey { rung: Decl, file: plugins/removeOffCanvasPaths.js, decl: 2, sub: 0, line: 7 } |  |  | 0.710 |
| ns | 3259 |  | 134 | builtinPlugins registry | 2.2 |  | 0.698 |
| walker |  | 3287 | 39 | Code::CodeKey { rung: Names, file: plugins/reusePaths.js, decl: 0, sub: 0, line: 0 } |  |  | 0.698 |
| walker |  | 3342 | 55 | Code::CodeKey { rung: Decl, file: plugins/reusePaths.js, decl: 2, sub: 0, line: 5 } |  |  | 0.699 |
| ns | 3448 |  | 189 | invokePlugins -- the plugin engine | 2.3 |  | 0.676 |
| walker |  | 3460 | 118 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.676 |
| walker |  | 3521 | 61 | Code::CodeKey { rung: Doc, file: plugins/removeOffCanvasPaths.js, decl: 3, sub: 0, line: 17 } |  |  | 0.676 |
| ns | 3669 |  | 221 | createPreset -- how a preset validates and forwards overrides | 2.4 |  | 0.655 |
| walker |  | 3902 | 381 | Json::Scripts { file: package.json } |  |  | 0.656 |
| walker |  | 3942 | 40 | Code::CodeKey { rung: Names, file: plugins/cleanupEnableBackground.js, decl: 0, sub: 0, line: 0 } |  |  | 0.656 |
| ns | 3956 |  | 287 | visit() -- the AST traversal contract | 2.5 |  | 0.632 |
| walker |  | 3957 | 15 | Code::CodeKey { rung: Decl, file: plugins/cleanupEnableBackground.js, decl: 2, sub: 0, line: 5 } |  |  | 0.632 |
| walker |  | 3997 | 40 | Code::CodeKey { rung: Names, file: plugins/moveGroupAttrsToElems.js, decl: 0, sub: 0, line: 0 } |  |  | 0.632 |
| walker |  | 4012 | 15 | Code::CodeKey { rung: Decl, file: plugins/moveGroupAttrsToElems.js, decl: 2, sub: 0, line: 5 } |  |  | 0.632 |
| walker |  | 4052 | 40 | Code::CodeKey { rung: Names, file: plugins/removeXlink.js, decl: 0, sub: 0, line: 0 } |  |  | 0.632 |
| walker |  | 4074 | 22 | Code::CodeKey { rung: Decl, file: plugins/removeXlink.js, decl: 2, sub: 0, line: 11 } |  |  | 0.632 |
| walker |  | 4115 | 41 | Code::CodeKey { rung: Names, file: plugins/cleanupAttrs.js, decl: 0, sub: 0, line: 0 } |  |  | 0.632 |
| walker |  | 4133 | 18 | Code::CodeKey { rung: Decl, file: plugins/cleanupAttrs.js, decl: 2, sub: 0, line: 9 } |  |  | 0.632 |
| ns | 4139 |  | 183 | Anatomy of a plugin module (removeComments as exemplar) | 2.6 |  | 0.618 |
| walker |  | 4189 | 56 | Code::CodeKey { rung: Doc, file: plugins/cleanupAttrs.js, decl: 1, sub: 0, line: 8 } |  |  | 0.618 |
| walker |  | 4230 | 41 | Code::CodeKey { rung: Names, file: plugins/convertColors.js, decl: 0, sub: 0, line: 0 } |  |  | 0.618 |
| walker |  | 4256 | 26 | Code::CodeKey { rung: Decl, file: plugins/convertColors.js, decl: 2, sub: 0, line: 18 } |  |  | 0.618 |
| ns | 4291 |  | 152 | Overriding and disabling preset-default plugins (README) | 2.7 |  | 0.603 |
| walker |  | 4297 | 41 | Code::CodeKey { rung: Names, file: plugins/convertTransform.js, decl: 0, sub: 0, line: 0 } |  |  | 0.603 |
| walker |  | 4312 | 15 | Code::CodeKey { rung: Decl, file: plugins/convertTransform.js, decl: 2, sub: 0, line: 44 } |  |  | 0.603 |
| walker |  | 4376 | 64 | Code::CodeKey { rung: Doc, file: plugins/cleanupAttrs.js, decl: 3, sub: 0, line: 22 } |  |  | 0.603 |
| ns | 4408 |  | 117 | Custom plugin objects (docs/05-plugins-api.mdx) | 2.8 |  | 0.597 |
| walker |  | 4418 | 42 | Code::CodeKey { rung: Names, file: plugins/cleanupNumericValues.js, decl: 0, sub: 0, line: 0 } |  |  | 0.597 |
| walker |  | 4440 | 22 | Code::CodeKey { rung: Decl, file: plugins/cleanupNumericValues.js, decl: 2, sub: 0, line: 12 } |  |  | 0.597 |
| walker |  | 4482 | 42 | Code::CodeKey { rung: Names, file: plugins/convertOneStopGradients.js, decl: 0, sub: 0, line: 0 } |  |  | 0.597 |
| walker |  | 4502 | 20 | Code::CodeKey { rung: Decl, file: plugins/convertOneStopGradients.js, decl: 2, sub: 0, line: 10 } |  |  | 0.597 |
| walker |  | 4544 | 42 | Code::CodeKey { rung: Names, file: plugins/convertPathData.js, decl: 0, sub: 0, line: 0 } |  |  | 0.597 |
| walker |  | 4563 | 19 | Code::CodeKey { rung: Decl, file: plugins/convertPathData.js, decl: 2, sub: 0, line: 43 } |  |  | 0.597 |
| ns | 4603 |  | 195 | Node entry point: loadConfig and the Node optimize wrapper | 2.9 |  | 0.585 |
| walker |  | 4605 | 42 | Code::CodeKey { rung: Names, file: plugins/moveElemsAttrsToGroup.js, decl: 0, sub: 0, line: 0 } |  |  | 0.585 |
| walker |  | 4621 | 16 | Code::CodeKey { rung: Decl, file: plugins/moveElemsAttrsToGroup.js, decl: 2, sub: 0, line: 5 } |  |  | 0.585 |
| walker |  | 4663 | 42 | Code::CodeKey { rung: Names, file: plugins/removeEditorsNSData.js, decl: 0, sub: 0, line: 0 } |  |  | 0.585 |
| walker |  | 4678 | 15 | Code::CodeKey { rung: Decl, file: plugins/removeEditorsNSData.js, decl: 2, sub: 0, line: 10 } |  |  | 0.585 |
| walker |  | 4714 | 36 | Code::CodeKey { rung: Doc, file: plugins/removeEditorsNSData.js, decl: 1, sub: 0, line: 9 } |  |  | 0.585 |
| walker |  | 4756 | 42 | Code::CodeKey { rung: Names, file: plugins/removeHiddenElems.js, decl: 0, sub: 0, line: 0 } |  |  | 0.586 |
| walker |  | 4774 | 18 | Code::CodeKey { rung: Decl, file: plugins/removeHiddenElems.js, decl: 2, sub: 0, line: 30 } |  |  | 0.586 |
| ns | 4786 |  | 183 | Plugin descriptions: preset-default steps 1-9 | 3.1 |  | 0.580 |
| walker |  | 4816 | 42 | Code::CodeKey { rung: Names, file: plugins/removeNonInheritableGroupAttrs.js, decl: 0, sub: 0, line: 0 } |  |  | 0.580 |
| walker |  | 4833 | 17 | Code::CodeKey { rung: Decl, file: plugins/removeNonInheritableGroupAttrs.js, decl: 2, sub: 0, line: 8 } |  |  | 0.580 |
| walker |  | 4895 | 62 | Code::CodeKey { rung: Doc, file: plugins/removeNonInheritableGroupAttrs.js, decl: 3, sub: 0, line: 18 } |  |  | 0.580 |
| walker |  | 4937 | 42 | Code::CodeKey { rung: Names, file: plugins/removeScripts.js, decl: 0, sub: 0, line: 0 } |  |  | 0.580 |
| walker |  | 4943 | 6 | Fs::DirListing { dir: .yarn/plugins } |  |  | 0.580 |
| walker |  | 4986 | 43 | Code::CodeKey { rung: Names, file: plugins/removeAttributesBySelector.js, decl: 0, sub: 0, line: 0 } |  |  | 0.580 |
| walker |  | 5003 | 17 | Code::CodeKey { rung: Decl, file: plugins/removeAttributesBySelector.js, decl: 2, sub: 0, line: 22 } |  |  | 0.580 |
| ns | 5033 |  | 247 | Plugin descriptions: preset-default steps 10-18 | 3.2 |  | 0.576 |
| walker |  | 5046 | 43 | Code::CodeKey { rung: Names, file: plugins/removeMetadata.js, decl: 0, sub: 0, line: 0 } |  |  | 0.577 |
| walker |  | 5089 | 43 | Code::CodeKey { rung: Names, file: plugins/removeTitle.js, decl: 0, sub: 0, line: 0 } |  |  | 0.577 |
| walker |  | 5132 | 43 | Code::CodeKey { rung: Names, file: plugins/removeUnusedNS.js, decl: 0, sub: 0, line: 0 } |  |  | 0.578 |
| ns | 5238 |  | 205 | Plugin descriptions: preset-default steps 19-26 | 3.3 |  | 0.576 |
| walker |  | 5405 | 273 | Code::CodeKey { rung: Names, file: lib/types.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.586 |
| ns | 5405 |  | 167 | Plugin descriptions: preset-default steps 27-34 | 3.4 |  | 0.586 |
| walker |  | 5414 | 9 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 5, sub: 0, line: 108 } |  |  | 0.586 |
| walker |  | 5436 | 22 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 12, sub: 0, line: 175 } |  |  | 0.586 |
| walker |  | 5458 | 22 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 14, sub: 0, line: 185 } |  |  | 0.586 |
| walker |  | 5481 | 23 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 13, sub: 0, line: 180 } |  |  | 0.586 |
| walker |  | 5505 | 24 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 2, sub: 0, line: 67 } |  |  | 0.586 |
| walker |  | 5529 | 24 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 17, sub: 0, line: 205 } |  |  | 0.586 |
| walker |  | 5560 | 31 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 6, sub: 0, line: 111 } |  |  | 0.586 |
| walker |  | 5591 | 31 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 11, sub: 0, line: 169 } |  |  | 0.587 |
| walker |  | 5629 | 38 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 9, sub: 0, line: 155 } |  |  | 0.587 |
| walker |  | 5674 | 45 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 15, sub: 0, line: 190 } |  |  | 0.587 |
| walker |  | 5720 | 46 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 10, sub: 0, line: 161 } |  |  | 0.588 |
| walker |  | 5773 | 53 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 8, sub: 0, line: 133 } |  |  | 0.588 |
| ns | 5778 |  | 373 | Plugin descriptions: opt-in plugins with optional params | 3.5 |  | 0.586 |
| walker |  | 5833 | 60 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 16, sub: 0, line: 197 } |  |  | 0.587 |
| ns | 5895 |  | 117 | Plugin descriptions: plugins requiring params | 3.6 |  | 0.584 |
| walker |  | 5916 | 83 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 4, sub: 0, line: 100 } |  |  | 0.584 |
| walker |  | 6072 | 156 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 7, sub: 0, line: 117 } |  |  | 0.584 |
| ns | 6127 |  | 232 | Config and Output type bodies | 4.1 |  | 0.571 |
| walker |  | 6362 | 290 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 3, sub: 0, line: 71 } |  |  | 0.571 |
| walker |  | 6406 | 44 | Code::CodeKey { rung: Names, file: plugins/removeEmptyAttrs.js, decl: 0, sub: 0, line: 0 } |  |  | 0.572 |
| ns | 6440 |  | 313 | Xast node type bodies | 4.2 |  | 0.588 |
| walker |  | 6463 | 57 | Code::CodeKey { rung: Doc, file: plugins/removeEmptyAttrs.js, decl: 3, sub: 0, line: 13 } |  |  | 0.588 |
| walker |  | 6507 | 44 | Code::CodeKey { rung: Names, file: plugins/removeRasterImages.js, decl: 0, sub: 0, line: 0 } |  |  | 0.590 |
| walker |  | 6551 | 44 | Code::CodeKey { rung: Names, file: plugins/removeUnknownsAndDefaults.js, decl: 0, sub: 0, line: 0 } |  |  | 0.591 |
| walker |  | 6571 | 20 | Code::CodeKey { rung: Decl, file: plugins/removeUnknownsAndDefaults.js, decl: 2, sub: 0, line: 32 } |  |  | 0.593 |
| ns | 6652 |  | 212 | Visitor keys, VisitorNode, PluginInfo and Plugin bodies | 4.3 |  | 0.582 |
| walker |  | 6736 | 165 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.610 |
| walker |  | 6781 | 45 | Code::CodeKey { rung: Names, file: plugins/collapseGroups.js, decl: 0, sub: 0, line: 0 } |  |  | 0.612 |
| ns | 6809 |  | 157 | coa.js top-level function roster | 5.1 |  | 0.604 |
| walker |  | 6826 | 45 | Code::CodeKey { rung: Names, file: plugins/mergeStyles.js, decl: 0, sub: 0, line: 0 } |  |  | 0.605 |
| walker |  | 6889 | 63 | Code::CodeKey { rung: Doc, file: plugins/mergeStyles.js, decl: 3, sub: 0, line: 14 } |  |  | 0.605 |
| ns | 6933 |  | 124 | CLI program metadata and positional argument | 5.2 |  | 0.601 |
| walker |  | 6934 | 45 | Code::CodeKey { rung: Names, file: plugins/removeDeprecatedAttrs.js, decl: 0, sub: 0, line: 0 } |  |  | 0.602 |
| walker |  | 6969 | 35 | Code::CodeKey { rung: Doc, file: plugins/removeDeprecatedAttrs.js, decl: 1, sub: 0, line: 10 } |  |  | 0.602 |
| walker |  | 7014 | 45 | Code::CodeKey { rung: Doc, file: plugins/removeDeprecatedAttrs.js, decl: 3, sub: 0, line: 76 } |  |  | 0.602 |
| walker |  | 7059 | 45 | Code::CodeKey { rung: Names, file: plugins/removeDoctype.js, decl: 0, sub: 0, line: 0 } |  |  | 0.604 |
| walker |  | 7122 | 63 | Code::CodeKey { rung: Body, file: plugins/removeDoctype.js, decl: 3, sub: 0, line: 30 } |  |  | 0.604 |
| walker |  | 7167 | 45 | Code::CodeKey { rung: Names, file: plugins/removeViewBox.js, decl: 0, sub: 0, line: 0 } |  |  | 0.606 |
| walker |  | 7213 | 46 | Code::CodeKey { rung: Names, file: plugins/removeComments.js, decl: 0, sub: 0, line: 0 } |  |  | 0.608 |
| ns | 7223 |  | 290 | CLI options, first half | 5.3 |  | 0.596 |
| walker |  | 7256 | 43 | Code::CodeKey { rung: Doc, file: plugins/removeComments.js, decl: 1, sub: 0, line: 8 } |  |  | 0.599 |
| walker |  | 7302 | 46 | Code::CodeKey { rung: Names, file: plugins/removeStyleElement.js, decl: 0, sub: 0, line: 0 } |  |  | 0.601 |
| walker |  | 7348 | 46 | Code::CodeKey { rung: Names, file: plugins/removeXMLNS.js, decl: 0, sub: 0, line: 0 } |  |  | 0.603 |
| walker |  | 7394 | 46 | Code::CodeKey { rung: Names, file: plugins/removeXMLProcInst.js, decl: 0, sub: 0, line: 0 } |  |  | 0.605 |
| ns | 7567 |  | 344 | CLI options, second half | 5.4 |  | 0.592 |
| walker |  | 7673 | 279 | Code::CodeKey { rung: Names, file: lib/types.ts, decl: 0, sub: 1, line: 0 } |  |  | 0.624 |
| ns | 7675 |  | 108 | bin/svgo.js executable entry | 5.5 |  | 0.619 |
| walker |  | 7685 | 12 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 37, sub: 0, line: 357 } |  |  | 0.619 |
| walker |  | 7707 | 22 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 31, sub: 0, line: 300 } |  |  | 0.619 |
| walker |  | 7730 | 23 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 24, sub: 0, line: 263 } |  |  | 0.619 |
| walker |  | 7753 | 23 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 34, sub: 0, line: 329 } |  |  | 0.619 |
| walker |  | 7783 | 30 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 27, sub: 0, line: 276 } |  |  | 0.619 |
| ns | 7790 |  | 115 | parseSvg and the sax configuration | 6.1 |  | 0.614 |
| walker |  | 7813 | 30 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 29, sub: 0, line: 289 } |  |  | 0.614 |
| walker |  | 7844 | 31 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 30, sub: 0, line: 294 } |  |  | 0.614 |
| walker |  | 7884 | 40 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 25, sub: 0, line: 268 } |  |  | 0.615 |
| ns | 7913 |  | 123 | stringifier function roster | 6.2 |  | 0.610 |
| walker |  | 7926 | 42 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 28, sub: 0, line: 282 } |  |  | 0.610 |
| walker |  | 7973 | 47 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 22, sub: 0, line: 248 } |  |  | 0.611 |
| ns | 7996 |  | 83 | xast query helpers | 6.3 |  | 0.609 |
| walker |  | 8022 | 49 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 21, sub: 0, line: 243 } |  |  | 0.612 |
| walker |  | 8117 | 95 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 23, sub: 0, line: 253 } |  |  | 0.626 |
| ns | 8174 |  | 178 | lib/svgo/tools.js -- complete export set | 6.4 |  | 0.621 |
| walker |  | 8295 | 178 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 33, sub: 0, line: 307 } |  |  | 0.621 |
| ns | 8330 |  | 156 | lib/style.js -- exported and internal function roster | 6.5 |  | 0.615 |
| ns | 8476 |  | 146 | lib/path.js -- path data parse and stringify roster | 6.6 |  | 0.608 |
| walker |  | 8515 | 220 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 36, sub: 0, line: 336 } |  |  | 0.629 |
| ns | 8710 |  | 234 | plugins/_collections.js -- complete export roster | 6.7 |  | 0.622 |
| walker |  | 8807 | 292 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 20, sub: 0, line: 214 } |  |  | 0.622 |
| ns | 8904 |  | 194 | Shared plugin helper module rosters | 6.8 |  | 0.617 |
| ns | 9017 |  | 113 | css-select adapter | 6.9 |  | 0.613 |
| walker |  | 9023 | 216 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 1, sub: 0, line: 30 } |  |  | 0.613 |
| ns | 9034 |  | 17 | mapNodesToParents | 6.10 |  | 0.612 |
| walker |  | 9246 | 223 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 1, sub: 1, line: 30 } |  |  | 0.612 |
| ns | 9268 |  | 234 | package.json scripts -- how to build, test and lint | 7.1 |  | 0.617 |
| ns | 9337 |  | 69 | test/ tree listing | 7.2 |  | 0.623 |
| ns | 9452 |  | 115 | The fixture-driven plugin test format | 7.3 |  | 0.621 |
| walker |  | 9480 | 234 | Code::CodeKey { rung: Names, file: plugins/_collections.js, decl: 0, sub: 0, line: 0 } |  |  | 0.634 |
| walker |  | 9544 | 64 | Code::CodeKey { rung: Decl, file: plugins/_collections.js, decl: 14, sub: 0, line: 2387 } |  |  | 0.634 |
| ns | 9584 |  | 132 | docs/ and remaining leaf directory listings | 7.4 |  | 0.640 |
| walker |  | 9626 | 82 | Code::CodeKey { rung: Decl, file: plugins/_collections.js, decl: 11, sub: 0, line: 2179 } |  |  | 0.640 |
| ns | 9719 |  | 135 | Migration guide outlines | 7.5 |  | 0.636 |
| walker |  | 9727 | 101 | Code::CodeKey { rung: Decl, file: plugins/_collections.js, decl: 9, sub: 0, line: 2111 } |  |  | 0.636 |
| walker |  | 9749 | 22 | Code::CodeKey { rung: Doc, file: plugins/_collections.js, decl: 3, sub: 0, line: 117 } |  |  | 0.636 |
| ns | 9843 |  | 124 | TypeScript and lint configuration | 7.6 |  | 0.632 |
| walker |  | 9907 | 158 | Code::CodeKey { rung: Decl, file: plugins/_collections.js, decl: 6, sub: 0, line: 389 } |  |  | 0.632 |
| walker |  | 9934 | 27 | Code::CodeKey { rung: Doc, file: plugins/_collections.js, decl: 11, sub: 0, line: 2179 } |  |  | 0.632 |
| ns | 9968 |  | 125 | Runtime dependencies | 7.7 |  | 0.635 |
| walker |  | 9986 | 52 | Code::CodeKey { rung: Doc, file: plugins/_collections.js, decl: 9, sub: 0, line: 2111 } |  |  | 0.635 |
| walker |  | 9994 | 8 | Code::CodeKey { rung: Doc, file: plugins/_collections.js, decl: 14, sub: 0, line: 2387 } |  |  | 0.635 |

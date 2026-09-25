Score(3000)=0.680 I=0.884 C=0.522 ns_rows≤3K=14/54 grid(1000/1442/2080/3000/4327/6240/9000)=0.638/0.831/0.730/0.680/0.603/0.572/0.594

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
| walker |  | 1318 | 93 | Code::CodeKey { rung: Names, file: lib/svgo.js, decl: 0, sub: 0, line: 0 } |  |  | 0.831 |
| ns | 1470 |  | 239 | svgo.config.mjs shape (README Configuration) | 1.9 |  | 0.768 |
| walker |  | 1602 | 284 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: true } |  |  | 0.768 |
| walker |  | 1637 | 35 | Fs::DirListing { dir: test/regression } |  |  | 0.769 |
| ns | 1669 |  | 199 | optimize() body, part 1: multipass loop and plugin resolution | 1.10 | 1.4 | 0.733 |
| walker |  | 1705 | 68 | Code::CodeKey { rung: Doc, file: lib/svgo.js, decl: 1, sub: 0, line: 81 } |  |  | 0.765 |
| ns | 1873 |  | 204 | optimize() body, part 2: overrides, invoke, stringify, datauri | 1.11 | 1.10 | 0.725 |
| walker |  | 1910 | 205 | Json::Dependencies { file: package.json } |  |  | 0.726 |
| walker |  | 1917 | 7 | Code::CodeKey { rung: Names, file: lib/types.js, decl: 0, sub: 0, line: 0 } |  |  | 0.726 |
| walker |  | 2144 | 227 | Json::Entry { file: package.json } |  |  | 0.730 |
| walker |  | 2193 | 49 | Fs::DirListing { dir: test/fixtures/config-loader } |  |  | 0.730 |
| ns | 2423 |  | 550 | Complete exported type roster of lib/types.ts | 1.12 |  | 0.666 |
| walker |  | 2581 | 388 | Fs::DirListing { dir: docs/04-plugins } |  |  | 0.666 |
| walker |  | 2638 | 57 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.666 |
| walker |  | 2673 | 35 | Code::CodeKey { rung: Names, file: plugins/removeDimensions.js, decl: 0, sub: 0, line: 0 } |  |  | 0.666 |
| ns | 2691 |  | 268 | Package identity, entry points and engines | 1.13 |  | 0.684 |
| walker |  | 2697 | 24 | Code::CodeKey { rung: Decl, file: plugins/removeDimensions.js, decl: 2, sub: 0, line: 2 } |  |  | 0.684 |
| walker |  | 2721 | 24 | Code::CodeKey { rung: Names, file: plugins/preset-default.js, decl: 0, sub: 0, line: 0 } |  |  | 0.684 |
| ns | 2755 |  | 64 | Version constant | 1.14 |  | 0.674 |
| walker |  | 3062 | 341 | Code::CodeKey { rung: Decl, file: plugins/preset-default.js, decl: 1, sub: 0, line: 37 } |  |  | 0.682 |
| walker |  | 3101 | 39 | Code::CodeKey { rung: Names, file: plugins/removeOffCanvasPaths.js, decl: 0, sub: 0, line: 0 } |  |  | 0.682 |
| walker |  | 3119 | 18 | Code::CodeKey { rung: Decl, file: plugins/removeOffCanvasPaths.js, decl: 2, sub: 0, line: 7 } |  |  | 0.682 |
| ns | 3125 |  | 370 | preset-default: the complete default pipeline in execution order | 2.1 |  | 0.710 |
| walker |  | 3158 | 39 | Code::CodeKey { rung: Names, file: plugins/reusePaths.js, decl: 0, sub: 0, line: 0 } |  |  | 0.710 |
| walker |  | 3213 | 55 | Code::CodeKey { rung: Decl, file: plugins/reusePaths.js, decl: 2, sub: 0, line: 5 } |  |  | 0.711 |
| ns | 3259 |  | 134 | builtinPlugins registry | 2.2 |  | 0.699 |
| walker |  | 3274 | 61 | Code::CodeKey { rung: Doc, file: plugins/removeOffCanvasPaths.js, decl: 3, sub: 0, line: 17 } |  |  | 0.699 |
| ns | 3448 |  | 189 | invokePlugins -- the plugin engine | 2.3 |  | 0.676 |
| walker |  | 3655 | 381 | Json::Scripts { file: package.json } |  |  | 0.677 |
| ns | 3669 |  | 221 | createPreset -- how a preset validates and forwards overrides | 2.4 |  | 0.656 |
| walker |  | 3695 | 40 | Code::CodeKey { rung: Names, file: plugins/cleanupEnableBackground.js, decl: 0, sub: 0, line: 0 } |  |  | 0.656 |
| walker |  | 3710 | 15 | Code::CodeKey { rung: Decl, file: plugins/cleanupEnableBackground.js, decl: 2, sub: 0, line: 5 } |  |  | 0.656 |
| walker |  | 3750 | 40 | Code::CodeKey { rung: Names, file: plugins/moveGroupAttrsToElems.js, decl: 0, sub: 0, line: 0 } |  |  | 0.656 |
| walker |  | 3765 | 15 | Code::CodeKey { rung: Decl, file: plugins/moveGroupAttrsToElems.js, decl: 2, sub: 0, line: 5 } |  |  | 0.656 |
| walker |  | 3805 | 40 | Code::CodeKey { rung: Names, file: plugins/removeXlink.js, decl: 0, sub: 0, line: 0 } |  |  | 0.656 |
| walker |  | 3827 | 22 | Code::CodeKey { rung: Decl, file: plugins/removeXlink.js, decl: 2, sub: 0, line: 11 } |  |  | 0.656 |
| walker |  | 3868 | 41 | Code::CodeKey { rung: Names, file: plugins/cleanupAttrs.js, decl: 0, sub: 0, line: 0 } |  |  | 0.656 |
| walker |  | 3886 | 18 | Code::CodeKey { rung: Decl, file: plugins/cleanupAttrs.js, decl: 2, sub: 0, line: 9 } |  |  | 0.656 |
| walker |  | 3942 | 56 | Code::CodeKey { rung: Doc, file: plugins/cleanupAttrs.js, decl: 1, sub: 0, line: 8 } |  |  | 0.656 |
| ns | 3956 |  | 287 | visit() -- the AST traversal contract | 2.5 |  | 0.632 |
| walker |  | 3983 | 41 | Code::CodeKey { rung: Names, file: plugins/convertColors.js, decl: 0, sub: 0, line: 0 } |  |  | 0.632 |
| walker |  | 4009 | 26 | Code::CodeKey { rung: Decl, file: plugins/convertColors.js, decl: 2, sub: 0, line: 18 } |  |  | 0.632 |
| walker |  | 4050 | 41 | Code::CodeKey { rung: Names, file: plugins/convertTransform.js, decl: 0, sub: 0, line: 0 } |  |  | 0.632 |
| walker |  | 4065 | 15 | Code::CodeKey { rung: Decl, file: plugins/convertTransform.js, decl: 2, sub: 0, line: 44 } |  |  | 0.632 |
| walker |  | 4129 | 64 | Code::CodeKey { rung: Doc, file: plugins/cleanupAttrs.js, decl: 3, sub: 0, line: 22 } |  |  | 0.632 |
| ns | 4139 |  | 183 | Anatomy of a plugin module (removeComments as exemplar) | 2.6 |  | 0.618 |
| walker |  | 4171 | 42 | Code::CodeKey { rung: Names, file: plugins/cleanupNumericValues.js, decl: 0, sub: 0, line: 0 } |  |  | 0.618 |
| walker |  | 4193 | 22 | Code::CodeKey { rung: Decl, file: plugins/cleanupNumericValues.js, decl: 2, sub: 0, line: 12 } |  |  | 0.618 |
| walker |  | 4235 | 42 | Code::CodeKey { rung: Names, file: plugins/convertOneStopGradients.js, decl: 0, sub: 0, line: 0 } |  |  | 0.618 |
| walker |  | 4255 | 20 | Code::CodeKey { rung: Decl, file: plugins/convertOneStopGradients.js, decl: 2, sub: 0, line: 10 } |  |  | 0.619 |
| ns | 4291 |  | 152 | Overriding and disabling preset-default plugins (README) | 2.7 |  | 0.603 |
| walker |  | 4297 | 42 | Code::CodeKey { rung: Names, file: plugins/convertPathData.js, decl: 0, sub: 0, line: 0 } |  |  | 0.603 |
| walker |  | 4316 | 19 | Code::CodeKey { rung: Decl, file: plugins/convertPathData.js, decl: 2, sub: 0, line: 43 } |  |  | 0.603 |
| walker |  | 4358 | 42 | Code::CodeKey { rung: Names, file: plugins/moveElemsAttrsToGroup.js, decl: 0, sub: 0, line: 0 } |  |  | 0.603 |
| walker |  | 4374 | 16 | Code::CodeKey { rung: Decl, file: plugins/moveElemsAttrsToGroup.js, decl: 2, sub: 0, line: 5 } |  |  | 0.603 |
| ns | 4408 |  | 117 | Custom plugin objects (docs/05-plugins-api.mdx) | 2.8 |  | 0.597 |
| walker |  | 4416 | 42 | Code::CodeKey { rung: Names, file: plugins/removeEditorsNSData.js, decl: 0, sub: 0, line: 0 } |  |  | 0.597 |
| walker |  | 4431 | 15 | Code::CodeKey { rung: Decl, file: plugins/removeEditorsNSData.js, decl: 2, sub: 0, line: 10 } |  |  | 0.597 |
| walker |  | 4467 | 36 | Code::CodeKey { rung: Doc, file: plugins/removeEditorsNSData.js, decl: 1, sub: 0, line: 9 } |  |  | 0.597 |
| walker |  | 4509 | 42 | Code::CodeKey { rung: Names, file: plugins/removeHiddenElems.js, decl: 0, sub: 0, line: 0 } |  |  | 0.597 |
| walker |  | 4527 | 18 | Code::CodeKey { rung: Decl, file: plugins/removeHiddenElems.js, decl: 2, sub: 0, line: 30 } |  |  | 0.598 |
| walker |  | 4569 | 42 | Code::CodeKey { rung: Names, file: plugins/removeNonInheritableGroupAttrs.js, decl: 0, sub: 0, line: 0 } |  |  | 0.598 |
| walker |  | 4586 | 17 | Code::CodeKey { rung: Decl, file: plugins/removeNonInheritableGroupAttrs.js, decl: 2, sub: 0, line: 8 } |  |  | 0.598 |
| ns | 4603 |  | 195 | Node entry point: loadConfig and the Node optimize wrapper | 2.9 |  | 0.586 |
| walker |  | 4648 | 62 | Code::CodeKey { rung: Doc, file: plugins/removeNonInheritableGroupAttrs.js, decl: 3, sub: 0, line: 18 } |  |  | 0.586 |
| walker |  | 4690 | 42 | Code::CodeKey { rung: Names, file: plugins/removeScripts.js, decl: 0, sub: 0, line: 0 } |  |  | 0.586 |
| walker |  | 4696 | 6 | Fs::DirListing { dir: .yarn/plugins } |  |  | 0.586 |
| walker |  | 4739 | 43 | Code::CodeKey { rung: Names, file: plugins/removeAttributesBySelector.js, decl: 0, sub: 0, line: 0 } |  |  | 0.586 |
| walker |  | 4756 | 17 | Code::CodeKey { rung: Decl, file: plugins/removeAttributesBySelector.js, decl: 2, sub: 0, line: 22 } |  |  | 0.586 |
| ns | 4786 |  | 183 | Plugin descriptions: preset-default steps 1-9 | 3.1 |  | 0.580 |
| walker |  | 4799 | 43 | Code::CodeKey { rung: Names, file: plugins/removeMetadata.js, decl: 0, sub: 0, line: 0 } |  |  | 0.581 |
| walker |  | 4842 | 43 | Code::CodeKey { rung: Names, file: plugins/removeTitle.js, decl: 0, sub: 0, line: 0 } |  |  | 0.581 |
| walker |  | 4885 | 43 | Code::CodeKey { rung: Names, file: plugins/removeUnusedNS.js, decl: 0, sub: 0, line: 0 } |  |  | 0.581 |
| ns | 5033 |  | 247 | Plugin descriptions: preset-default steps 10-18 | 3.2 |  | 0.578 |
| walker |  | 5158 | 273 | Code::CodeKey { rung: Names, file: lib/types.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.592 |
| walker |  | 5167 | 9 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 5, sub: 0, line: 108 } |  |  | 0.592 |
| walker |  | 5189 | 22 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 12, sub: 0, line: 175 } |  |  | 0.592 |
| walker |  | 5211 | 22 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 14, sub: 0, line: 185 } |  |  | 0.593 |
| walker |  | 5234 | 23 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 13, sub: 0, line: 180 } |  |  | 0.593 |
| ns | 5238 |  | 205 | Plugin descriptions: preset-default steps 19-26 | 3.3 |  | 0.591 |
| walker |  | 5258 | 24 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 2, sub: 0, line: 67 } |  |  | 0.591 |
| walker |  | 5282 | 24 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 17, sub: 0, line: 205 } |  |  | 0.591 |
| walker |  | 5313 | 31 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 6, sub: 0, line: 111 } |  |  | 0.591 |
| walker |  | 5344 | 31 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 11, sub: 0, line: 169 } |  |  | 0.591 |
| walker |  | 5382 | 38 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 9, sub: 0, line: 155 } |  |  | 0.591 |
| ns | 5405 |  | 167 | Plugin descriptions: preset-default steps 27-34 | 3.4 |  | 0.587 |
| walker |  | 5427 | 45 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 15, sub: 0, line: 190 } |  |  | 0.587 |
| walker |  | 5473 | 46 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 10, sub: 0, line: 161 } |  |  | 0.588 |
| walker |  | 5526 | 53 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 8, sub: 0, line: 133 } |  |  | 0.588 |
| walker |  | 5586 | 60 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 16, sub: 0, line: 197 } |  |  | 0.589 |
| walker |  | 5669 | 83 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 4, sub: 0, line: 100 } |  |  | 0.589 |
| ns | 5778 |  | 373 | Plugin descriptions: opt-in plugins with optional params | 3.5 |  | 0.587 |
| walker |  | 5825 | 156 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 7, sub: 0, line: 117 } |  |  | 0.587 |
| ns | 5895 |  | 117 | Plugin descriptions: plugins requiring params | 3.6 |  | 0.584 |
| walker |  | 6115 | 290 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 3, sub: 0, line: 71 } |  |  | 0.584 |
| ns | 6127 |  | 232 | Config and Output type bodies | 4.1 |  | 0.571 |
| walker |  | 6159 | 44 | Code::CodeKey { rung: Names, file: plugins/removeEmptyAttrs.js, decl: 0, sub: 0, line: 0 } |  |  | 0.572 |
| walker |  | 6216 | 57 | Code::CodeKey { rung: Doc, file: plugins/removeEmptyAttrs.js, decl: 3, sub: 0, line: 13 } |  |  | 0.572 |
| walker |  | 6260 | 44 | Code::CodeKey { rung: Names, file: plugins/removeRasterImages.js, decl: 0, sub: 0, line: 0 } |  |  | 0.574 |
| walker |  | 6304 | 44 | Code::CodeKey { rung: Names, file: plugins/removeUnknownsAndDefaults.js, decl: 0, sub: 0, line: 0 } |  |  | 0.575 |
| walker |  | 6324 | 20 | Code::CodeKey { rung: Decl, file: plugins/removeUnknownsAndDefaults.js, decl: 2, sub: 0, line: 32 } |  |  | 0.577 |
| walker |  | 6369 | 45 | Code::CodeKey { rung: Names, file: plugins/collapseGroups.js, decl: 0, sub: 0, line: 0 } |  |  | 0.579 |
| walker |  | 6414 | 45 | Code::CodeKey { rung: Names, file: plugins/mergeStyles.js, decl: 0, sub: 0, line: 0 } |  |  | 0.581 |
| ns | 6440 |  | 313 | Xast node type bodies | 4.2 |  | 0.596 |
| walker |  | 6477 | 63 | Code::CodeKey { rung: Doc, file: plugins/mergeStyles.js, decl: 3, sub: 0, line: 14 } |  |  | 0.596 |
| walker |  | 6522 | 45 | Code::CodeKey { rung: Names, file: plugins/removeDeprecatedAttrs.js, decl: 0, sub: 0, line: 0 } |  |  | 0.598 |
| walker |  | 6557 | 35 | Code::CodeKey { rung: Doc, file: plugins/removeDeprecatedAttrs.js, decl: 1, sub: 0, line: 10 } |  |  | 0.598 |
| walker |  | 6602 | 45 | Code::CodeKey { rung: Doc, file: plugins/removeDeprecatedAttrs.js, decl: 3, sub: 0, line: 76 } |  |  | 0.598 |
| walker |  | 6647 | 45 | Code::CodeKey { rung: Names, file: plugins/removeDoctype.js, decl: 0, sub: 0, line: 0 } |  |  | 0.599 |
| ns | 6652 |  | 212 | Visitor keys, VisitorNode, PluginInfo and Plugin bodies | 4.3 |  | 0.588 |
| walker |  | 6710 | 63 | Code::CodeKey { rung: Body, file: plugins/removeDoctype.js, decl: 3, sub: 0, line: 30 } |  |  | 0.588 |
| walker |  | 6755 | 45 | Code::CodeKey { rung: Names, file: plugins/removeViewBox.js, decl: 0, sub: 0, line: 0 } |  |  | 0.590 |
| walker |  | 6801 | 46 | Code::CodeKey { rung: Names, file: plugins/removeComments.js, decl: 0, sub: 0, line: 0 } |  |  | 0.593 |
| ns | 6809 |  | 157 | coa.js top-level function roster | 5.1 |  | 0.585 |
| walker |  | 6844 | 43 | Code::CodeKey { rung: Doc, file: plugins/removeComments.js, decl: 1, sub: 0, line: 8 } |  |  | 0.588 |
| walker |  | 6890 | 46 | Code::CodeKey { rung: Names, file: plugins/removeStyleElement.js, decl: 0, sub: 0, line: 0 } |  |  | 0.590 |
| ns | 6933 |  | 124 | CLI program metadata and positional argument | 5.2 |  | 0.585 |
| walker |  | 6936 | 46 | Code::CodeKey { rung: Names, file: plugins/removeXMLNS.js, decl: 0, sub: 0, line: 0 } |  |  | 0.588 |
| walker |  | 6982 | 46 | Code::CodeKey { rung: Names, file: plugins/removeXMLProcInst.js, decl: 0, sub: 0, line: 0 } |  |  | 0.590 |
| walker |  | 7111 | 129 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.590 |
| ns | 7223 |  | 290 | CLI options, first half | 5.3 |  | 0.579 |
| walker |  | 7390 | 279 | Code::CodeKey { rung: Names, file: lib/types.ts, decl: 0, sub: 1, line: 0 } |  |  | 0.612 |
| walker |  | 7402 | 12 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 37, sub: 0, line: 357 } |  |  | 0.612 |
| walker |  | 7424 | 22 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 31, sub: 0, line: 300 } |  |  | 0.612 |
| walker |  | 7447 | 23 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 24, sub: 0, line: 263 } |  |  | 0.612 |
| walker |  | 7470 | 23 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 34, sub: 0, line: 329 } |  |  | 0.612 |
| walker |  | 7500 | 30 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 27, sub: 0, line: 276 } |  |  | 0.612 |
| walker |  | 7530 | 30 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 29, sub: 0, line: 289 } |  |  | 0.612 |
| walker |  | 7561 | 31 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 30, sub: 0, line: 294 } |  |  | 0.612 |
| ns | 7567 |  | 344 | CLI options, second half | 5.4 |  | 0.598 |
| walker |  | 7601 | 40 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 25, sub: 0, line: 268 } |  |  | 0.600 |
| walker |  | 7643 | 42 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 28, sub: 0, line: 282 } |  |  | 0.600 |
| ns | 7675 |  | 108 | bin/svgo.js executable entry | 5.5 |  | 0.595 |
| walker |  | 7690 | 47 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 22, sub: 0, line: 248 } |  |  | 0.595 |
| walker |  | 7739 | 49 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 21, sub: 0, line: 243 } |  |  | 0.599 |
| ns | 7790 |  | 115 | parseSvg and the sax configuration | 6.1 |  | 0.594 |
| walker |  | 7834 | 95 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 23, sub: 0, line: 253 } |  |  | 0.608 |
| ns | 7913 |  | 123 | stringifier function roster | 6.2 |  | 0.603 |
| ns | 7996 |  | 83 | xast query helpers | 6.3 |  | 0.601 |
| walker |  | 8012 | 178 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 33, sub: 0, line: 307 } |  |  | 0.601 |
| ns | 8174 |  | 178 | lib/svgo/tools.js -- complete export set | 6.4 |  | 0.596 |
| walker |  | 8232 | 220 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 36, sub: 0, line: 336 } |  |  | 0.617 |
| ns | 8330 |  | 156 | lib/style.js -- exported and internal function roster | 6.5 |  | 0.611 |
| ns | 8476 |  | 146 | lib/path.js -- path data parse and stringify roster | 6.6 |  | 0.605 |
| walker |  | 8524 | 292 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 20, sub: 0, line: 214 } |  |  | 0.605 |
| ns | 8710 |  | 234 | plugins/_collections.js -- complete export roster | 6.7 |  | 0.598 |
| walker |  | 8740 | 216 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 1, sub: 0, line: 30 } |  |  | 0.598 |
| ns | 8904 |  | 194 | Shared plugin helper module rosters | 6.8 |  | 0.594 |
| walker |  | 8963 | 223 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 1, sub: 1, line: 30 } |  |  | 0.594 |
| ns | 9017 |  | 113 | css-select adapter | 6.9 |  | 0.590 |
| ns | 9034 |  | 17 | mapNodesToParents | 6.10 |  | 0.589 |
| walker |  | 9197 | 234 | Code::CodeKey { rung: Names, file: plugins/_collections.js, decl: 0, sub: 0, line: 0 } |  |  | 0.603 |
| walker |  | 9261 | 64 | Code::CodeKey { rung: Decl, file: plugins/_collections.js, decl: 14, sub: 0, line: 2387 } |  |  | 0.603 |
| ns | 9268 |  | 234 | package.json scripts -- how to build, test and lint | 7.1 |  | 0.607 |
| ns | 9337 |  | 69 | test/ tree listing | 7.2 |  | 0.614 |
| walker |  | 9343 | 82 | Code::CodeKey { rung: Decl, file: plugins/_collections.js, decl: 11, sub: 0, line: 2179 } |  |  | 0.614 |
| walker |  | 9444 | 101 | Code::CodeKey { rung: Decl, file: plugins/_collections.js, decl: 9, sub: 0, line: 2111 } |  |  | 0.614 |
| ns | 9452 |  | 115 | The fixture-driven plugin test format | 7.3 |  | 0.611 |
| walker |  | 9466 | 22 | Code::CodeKey { rung: Doc, file: plugins/_collections.js, decl: 3, sub: 0, line: 117 } |  |  | 0.611 |
| ns | 9584 |  | 132 | docs/ and remaining leaf directory listings | 7.4 |  | 0.618 |
| walker |  | 9624 | 158 | Code::CodeKey { rung: Decl, file: plugins/_collections.js, decl: 6, sub: 0, line: 389 } |  |  | 0.618 |
| walker |  | 9651 | 27 | Code::CodeKey { rung: Doc, file: plugins/_collections.js, decl: 11, sub: 0, line: 2179 } |  |  | 0.618 |
| walker |  | 9703 | 52 | Code::CodeKey { rung: Doc, file: plugins/_collections.js, decl: 9, sub: 0, line: 2111 } |  |  | 0.618 |
| ns | 9719 |  | 135 | Migration guide outlines | 7.5 |  | 0.614 |
| walker |  | 9759 | 56 | Code::CodeKey { rung: Doc, file: plugins/_collections.js, decl: 14, sub: 0, line: 2387 } |  |  | 0.614 |
| walker |  | 9818 | 59 | Code::CodeKey { rung: Doc, file: plugins/_collections.js, decl: 6, sub: 0, line: 389 } |  |  | 0.614 |
| ns | 9843 |  | 124 | TypeScript and lint configuration | 7.6 |  | 0.610 |
| walker |  | 9891 | 73 | Code::CodeKey { rung: Doc, file: plugins/cleanupNumericValues.js, decl: 3, sub: 0, line: 35 } |  |  | 0.610 |
| walker |  | 9964 | 73 | Code::CodeKey { rung: Doc, file: plugins/removeScripts.js, decl: 3, sub: 0, line: 24 } |  |  | 0.610 |
| ns | 9968 |  | 125 | Runtime dependencies | 7.7 |  | 0.613 |
| walker |  | 9996 | 32 | Code::CodeKey { rung: Doc, file: plugins/removeUnusedNS.js, decl: 3, sub: 0, line: 12 } |  |  | 0.613 |

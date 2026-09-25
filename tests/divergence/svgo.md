Score(3000)=0.649 I=0.850 C=0.496 ns_rows≤3K=14/54 grid(1000/1442/2080/3000/4327/6240/9000)=0.736/0.641/0.694/0.649/0.583/0.571/0.615

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
| walker |  | 661 | 12 | Fs::DirListing { dir: .yarn/plugins/@yarnpkg } |  |  | 0.695 |
| ns | 715 |  | 219 | lib/svgo.js module graph and public re-exports | 1.6 |  | 0.608 |
| walker |  | 767 | 106 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.694 |
| walker |  | 822 | 55 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.815 |
| walker |  | 856 | 34 | Fs::DirListing { dir: test } |  |  | 0.816 |
| walker |  | 866 | 10 | Fs::DirListing { dir: test/svg2js } |  |  | 0.816 |
| ns | 875 |  | 160 | Command-line usage (README) | 1.7 |  | 0.736 |
| walker |  | 879 | 13 | Fs::DirListing { dir: test/cli } |  |  | 0.736 |
| walker |  | 1163 | 284 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: true } |  |  | 0.736 |
| walker |  | 1182 | 19 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.736 |
| ns | 1231 |  | 356 | Complete plugins/ listing (all 58 modules) | 1.8 |  | 0.570 |
| walker |  | 1275 | 93 | Code::CodeKey { rung: Names, file: lib/svgo.js, decl: 0, sub: 0, line: 0 } |  |  | 0.581 |
| ns | 1470 |  | 239 | svgo.config.mjs shape (README Configuration) | 1.9 |  | 0.537 |
| walker |  | 1631 | 356 | Fs::DirListing { dir: plugins } |  |  | 0.768 |
| ns | 1669 |  | 199 | optimize() body, part 1: multipass loop and plugin resolution | 1.10 | 1.4 | 0.732 |
| walker |  | 1786 | 155 | Json::Scripts { file: package.json } |  |  | 0.732 |
| walker |  | 1843 | 57 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.732 |
| walker |  | 1872 | 29 | Fs::DirListing { dir: test/coa } |  |  | 0.732 |
| ns | 1873 |  | 204 | optimize() body, part 2: overrides, invoke, stringify, datauri | 1.11 | 1.10 | 0.693 |
| walker |  | 2077 | 205 | Json::Dependencies { file: package.json } |  |  | 0.694 |
| walker |  | 2112 | 35 | Fs::DirListing { dir: test/regression } |  |  | 0.695 |
| walker |  | 2119 | 7 | Code::CodeKey { rung: Names, file: lib/types.js, decl: 0, sub: 0, line: 0 } |  |  | 0.695 |
| walker |  | 2345 | 226 | Json::ScriptsTail { file: package.json, chunk: 1 } |  |  | 0.696 |
| ns | 2423 |  | 550 | Complete exported type roster of lib/types.ts | 1.12 |  | 0.635 |
| walker |  | 2572 | 227 | Json::Entry { file: package.json } |  |  | 0.639 |
| ns | 2691 |  | 268 | Package identity, entry points and engines | 1.13 |  | 0.658 |
| walker |  | 2701 | 129 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.658 |
| walker |  | 2752 | 51 | Fs::DirListing { dir: test/fixtures/config-loader } |  |  | 0.658 |
| ns | 2755 |  | 64 | Version constant | 1.14 |  | 0.649 |
| walker |  | 2758 | 6 | Fs::DirListing { dir: test/fixtures/config-loader/one/two } |  |  | 0.649 |
| walker |  | 2765 | 7 | Fs::DirListing { dir: test/fixtures/config-loader/cjs } |  |  | 0.649 |
| walker |  | 2772 | 7 | Fs::DirListing { dir: test/fixtures/config-loader/mjs } |  |  | 0.649 |
| ns | 3125 |  | 370 | preset-default: the complete default pipeline in execution order | 2.1 |  | 0.602 |
| walker |  | 3160 | 388 | Fs::DirListing { dir: docs/04-plugins } |  |  | 0.602 |
| walker |  | 3195 | 35 | Code::CodeKey { rung: Names, file: plugins/removeDimensions.js, decl: 0, sub: 0, line: 0 } |  |  | 0.602 |
| walker |  | 3219 | 24 | Code::CodeKey { rung: Decl, file: plugins/removeDimensions.js, decl: 2, sub: 0, line: 2 } |  |  | 0.602 |
| walker |  | 3243 | 24 | Code::CodeKey { rung: Names, file: plugins/preset-default.js, decl: 0, sub: 0, line: 0 } |  |  | 0.602 |
| ns | 3259 |  | 134 | builtinPlugins registry | 2.2 |  | 0.592 |
| ns | 3448 |  | 189 | invokePlugins -- the plugin engine | 2.3 |  | 0.573 |
| walker |  | 3584 | 341 | Code::CodeKey { rung: Decl, file: plugins/preset-default.js, decl: 1, sub: 0, line: 37 } |  |  | 0.654 |
| walker |  | 3623 | 39 | Code::CodeKey { rung: Names, file: plugins/removeOffCanvasPaths.js, decl: 0, sub: 0, line: 0 } |  |  | 0.654 |
| walker |  | 3641 | 18 | Code::CodeKey { rung: Decl, file: plugins/removeOffCanvasPaths.js, decl: 2, sub: 0, line: 7 } |  |  | 0.654 |
| ns | 3669 |  | 221 | createPreset -- how a preset validates and forwards overrides | 2.4 |  | 0.634 |
| walker |  | 3680 | 39 | Code::CodeKey { rung: Names, file: plugins/reusePaths.js, decl: 0, sub: 0, line: 0 } |  |  | 0.634 |
| walker |  | 3735 | 55 | Code::CodeKey { rung: Decl, file: plugins/reusePaths.js, decl: 2, sub: 0, line: 5 } |  |  | 0.635 |
| walker |  | 3853 | 118 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.635 |
| walker |  | 3893 | 40 | Code::CodeKey { rung: Names, file: plugins/cleanupEnableBackground.js, decl: 0, sub: 0, line: 0 } |  |  | 0.635 |
| walker |  | 3908 | 15 | Code::CodeKey { rung: Decl, file: plugins/cleanupEnableBackground.js, decl: 2, sub: 0, line: 5 } |  |  | 0.635 |
| walker |  | 3948 | 40 | Code::CodeKey { rung: Names, file: plugins/moveGroupAttrsToElems.js, decl: 0, sub: 0, line: 0 } |  |  | 0.635 |
| ns | 3956 |  | 287 | visit() -- the AST traversal contract | 2.5 |  | 0.611 |
| walker |  | 3963 | 15 | Code::CodeKey { rung: Decl, file: plugins/moveGroupAttrsToElems.js, decl: 2, sub: 0, line: 5 } |  |  | 0.611 |
| walker |  | 4003 | 40 | Code::CodeKey { rung: Names, file: plugins/removeXlink.js, decl: 0, sub: 0, line: 0 } |  |  | 0.611 |
| walker |  | 4025 | 22 | Code::CodeKey { rung: Decl, file: plugins/removeXlink.js, decl: 2, sub: 0, line: 11 } |  |  | 0.611 |
| walker |  | 4066 | 41 | Code::CodeKey { rung: Names, file: plugins/cleanupAttrs.js, decl: 0, sub: 0, line: 0 } |  |  | 0.611 |
| walker |  | 4084 | 18 | Code::CodeKey { rung: Decl, file: plugins/cleanupAttrs.js, decl: 2, sub: 0, line: 9 } |  |  | 0.611 |
| walker |  | 4125 | 41 | Code::CodeKey { rung: Names, file: plugins/convertColors.js, decl: 0, sub: 0, line: 0 } |  |  | 0.611 |
| ns | 4139 |  | 183 | Anatomy of a plugin module (removeComments as exemplar) | 2.6 |  | 0.598 |
| walker |  | 4151 | 26 | Code::CodeKey { rung: Decl, file: plugins/convertColors.js, decl: 2, sub: 0, line: 18 } |  |  | 0.598 |
| walker |  | 4192 | 41 | Code::CodeKey { rung: Names, file: plugins/convertTransform.js, decl: 0, sub: 0, line: 0 } |  |  | 0.598 |
| walker |  | 4207 | 15 | Code::CodeKey { rung: Decl, file: plugins/convertTransform.js, decl: 2, sub: 0, line: 44 } |  |  | 0.598 |
| walker |  | 4249 | 42 | Code::CodeKey { rung: Names, file: plugins/cleanupNumericValues.js, decl: 0, sub: 0, line: 0 } |  |  | 0.598 |
| walker |  | 4271 | 22 | Code::CodeKey { rung: Decl, file: plugins/cleanupNumericValues.js, decl: 2, sub: 0, line: 12 } |  |  | 0.598 |
| ns | 4291 |  | 152 | Overriding and disabling preset-default plugins (README) | 2.7 |  | 0.583 |
| walker |  | 4313 | 42 | Code::CodeKey { rung: Names, file: plugins/convertOneStopGradients.js, decl: 0, sub: 0, line: 0 } |  |  | 0.583 |
| walker |  | 4333 | 20 | Code::CodeKey { rung: Decl, file: plugins/convertOneStopGradients.js, decl: 2, sub: 0, line: 10 } |  |  | 0.584 |
| walker |  | 4375 | 42 | Code::CodeKey { rung: Names, file: plugins/convertPathData.js, decl: 0, sub: 0, line: 0 } |  |  | 0.584 |
| walker |  | 4394 | 19 | Code::CodeKey { rung: Decl, file: plugins/convertPathData.js, decl: 2, sub: 0, line: 43 } |  |  | 0.584 |
| ns | 4408 |  | 117 | Custom plugin objects (docs/05-plugins-api.mdx) | 2.8 |  | 0.578 |
| walker |  | 4436 | 42 | Code::CodeKey { rung: Names, file: plugins/moveElemsAttrsToGroup.js, decl: 0, sub: 0, line: 0 } |  |  | 0.578 |
| walker |  | 4452 | 16 | Code::CodeKey { rung: Decl, file: plugins/moveElemsAttrsToGroup.js, decl: 2, sub: 0, line: 5 } |  |  | 0.578 |
| walker |  | 4494 | 42 | Code::CodeKey { rung: Names, file: plugins/removeEditorsNSData.js, decl: 0, sub: 0, line: 0 } |  |  | 0.578 |
| walker |  | 4509 | 15 | Code::CodeKey { rung: Decl, file: plugins/removeEditorsNSData.js, decl: 2, sub: 0, line: 10 } |  |  | 0.578 |
| walker |  | 4551 | 42 | Code::CodeKey { rung: Names, file: plugins/removeHiddenElems.js, decl: 0, sub: 0, line: 0 } |  |  | 0.578 |
| walker |  | 4569 | 18 | Code::CodeKey { rung: Decl, file: plugins/removeHiddenElems.js, decl: 2, sub: 0, line: 30 } |  |  | 0.578 |
| ns | 4603 |  | 195 | Node entry point: loadConfig and the Node optimize wrapper | 2.9 |  | 0.567 |
| walker |  | 4611 | 42 | Code::CodeKey { rung: Names, file: plugins/removeNonInheritableGroupAttrs.js, decl: 0, sub: 0, line: 0 } |  |  | 0.567 |
| walker |  | 4628 | 17 | Code::CodeKey { rung: Decl, file: plugins/removeNonInheritableGroupAttrs.js, decl: 2, sub: 0, line: 8 } |  |  | 0.567 |
| walker |  | 4670 | 42 | Code::CodeKey { rung: Names, file: plugins/removeScripts.js, decl: 0, sub: 0, line: 0 } |  |  | 0.567 |
| walker |  | 4754 | 84 | Fs::DirListing { dir: test/svgo } |  |  | 0.567 |
| ns | 4786 |  | 183 | Plugin descriptions: preset-default steps 1-9 | 3.1 |  | 0.561 |
| walker |  | 4956 | 202 | Code::CodeKey { rung: Names, file: lib/types.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.570 |
| walker |  | 4965 | 9 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 5, sub: 0, line: 108 } |  |  | 0.570 |
| walker |  | 4987 | 22 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 12, sub: 0, line: 175 } |  |  | 0.570 |
| walker |  | 5011 | 24 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 2, sub: 0, line: 67 } |  |  | 0.570 |
| ns | 5033 |  | 247 | Plugin descriptions: preset-default steps 10-18 | 3.2 |  | 0.566 |
| walker |  | 5036 | 25 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 13, sub: 0, line: 180 } |  |  | 0.566 |
| walker |  | 5067 | 31 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 6, sub: 0, line: 111 } |  |  | 0.566 |
| walker |  | 5098 | 31 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 11, sub: 0, line: 169 } |  |  | 0.567 |
| walker |  | 5136 | 38 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 9, sub: 0, line: 155 } |  |  | 0.567 |
| walker |  | 5182 | 46 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 10, sub: 0, line: 161 } |  |  | 0.567 |
| walker |  | 5235 | 53 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 8, sub: 0, line: 133 } |  |  | 0.567 |
| ns | 5238 |  | 205 | Plugin descriptions: preset-default steps 19-26 | 3.3 |  | 0.566 |
| walker |  | 5318 | 83 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 4, sub: 0, line: 100 } |  |  | 0.566 |
| ns | 5405 |  | 167 | Plugin descriptions: preset-default steps 27-34 | 3.4 |  | 0.560 |
| walker |  | 5474 | 156 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 7, sub: 0, line: 117 } |  |  | 0.560 |
| walker |  | 5764 | 290 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 3, sub: 0, line: 71 } |  |  | 0.560 |
| ns | 5778 |  | 373 | Plugin descriptions: opt-in plugins with optional params | 3.5 |  | 0.558 |
| walker |  | 5807 | 43 | Code::CodeKey { rung: Names, file: plugins/removeAttributesBySelector.js, decl: 0, sub: 0, line: 0 } |  |  | 0.558 |
| walker |  | 5824 | 17 | Code::CodeKey { rung: Decl, file: plugins/removeAttributesBySelector.js, decl: 2, sub: 0, line: 22 } |  |  | 0.558 |
| walker |  | 5867 | 43 | Code::CodeKey { rung: Names, file: plugins/removeMetadata.js, decl: 0, sub: 0, line: 0 } |  |  | 0.559 |
| ns | 5895 |  | 117 | Plugin descriptions: plugins requiring params | 3.6 |  | 0.556 |
| walker |  | 5910 | 43 | Code::CodeKey { rung: Names, file: plugins/removeTitle.js, decl: 0, sub: 0, line: 0 } |  |  | 0.558 |
| walker |  | 5953 | 43 | Code::CodeKey { rung: Names, file: plugins/removeUnusedNS.js, decl: 0, sub: 0, line: 0 } |  |  | 0.558 |
| walker |  | 5997 | 44 | Code::CodeKey { rung: Names, file: plugins/removeEmptyAttrs.js, decl: 0, sub: 0, line: 0 } |  |  | 0.559 |
| walker |  | 6041 | 44 | Code::CodeKey { rung: Names, file: plugins/removeRasterImages.js, decl: 0, sub: 0, line: 0 } |  |  | 0.561 |
| walker |  | 6085 | 44 | Code::CodeKey { rung: Names, file: plugins/removeUnknownsAndDefaults.js, decl: 0, sub: 0, line: 0 } |  |  | 0.562 |
| walker |  | 6105 | 20 | Code::CodeKey { rung: Decl, file: plugins/removeUnknownsAndDefaults.js, decl: 2, sub: 0, line: 32 } |  |  | 0.565 |
| ns | 6127 |  | 232 | Config and Output type bodies | 4.1 |  | 0.553 |
| walker |  | 6270 | 165 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.583 |
| walker |  | 6315 | 45 | Code::CodeKey { rung: Names, file: plugins/collapseGroups.js, decl: 0, sub: 0, line: 0 } |  |  | 0.585 |
| walker |  | 6360 | 45 | Code::CodeKey { rung: Names, file: plugins/mergeStyles.js, decl: 0, sub: 0, line: 0 } |  |  | 0.586 |
| walker |  | 6405 | 45 | Code::CodeKey { rung: Names, file: plugins/removeDeprecatedAttrs.js, decl: 0, sub: 0, line: 0 } |  |  | 0.588 |
| ns | 6440 |  | 313 | Xast node type bodies | 4.2 |  | 0.573 |
| walker |  | 6450 | 45 | Code::CodeKey { rung: Names, file: plugins/removeDoctype.js, decl: 0, sub: 0, line: 0 } |  |  | 0.574 |
| walker |  | 6495 | 45 | Code::CodeKey { rung: Names, file: plugins/removeViewBox.js, decl: 0, sub: 0, line: 0 } |  |  | 0.577 |
| walker |  | 6541 | 46 | Code::CodeKey { rung: Names, file: plugins/removeComments.js, decl: 0, sub: 0, line: 0 } |  |  | 0.579 |
| walker |  | 6587 | 46 | Code::CodeKey { rung: Names, file: plugins/removeStyleElement.js, decl: 0, sub: 0, line: 0 } |  |  | 0.581 |
| walker |  | 6633 | 46 | Code::CodeKey { rung: Names, file: plugins/removeXMLNS.js, decl: 0, sub: 0, line: 0 } |  |  | 0.583 |
| ns | 6652 |  | 212 | Visitor keys, VisitorNode, PluginInfo and Plugin bodies | 4.3 |  | 0.573 |
| walker |  | 6679 | 46 | Code::CodeKey { rung: Names, file: plugins/removeXMLProcInst.js, decl: 0, sub: 0, line: 0 } |  |  | 0.575 |
| ns | 6809 |  | 157 | coa.js top-level function roster | 5.1 |  | 0.568 |
| ns | 6933 |  | 124 | CLI program metadata and positional argument | 5.2 |  | 0.564 |
| walker |  | 7032 | 353 | Code::CodeKey { rung: Names, file: lib/types.ts, decl: 0, sub: 1, line: 0 } |  |  | 0.603 |
| walker |  | 7044 | 12 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 37, sub: 0, line: 357 } |  |  | 0.603 |
| walker |  | 7066 | 22 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 14, sub: 0, line: 185 } |  |  | 0.606 |
| walker |  | 7088 | 22 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 31, sub: 0, line: 300 } |  |  | 0.606 |
| walker |  | 7111 | 23 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 24, sub: 0, line: 263 } |  |  | 0.607 |
| walker |  | 7134 | 23 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 34, sub: 0, line: 329 } |  |  | 0.607 |
| walker |  | 7158 | 24 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 17, sub: 0, line: 205 } |  |  | 0.611 |
| walker |  | 7188 | 30 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 27, sub: 0, line: 276 } |  |  | 0.611 |
| walker |  | 7218 | 30 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 29, sub: 0, line: 289 } |  |  | 0.611 |
| ns | 7223 |  | 290 | CLI options, first half | 5.3 |  | 0.599 |
| walker |  | 7249 | 31 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 30, sub: 0, line: 294 } |  |  | 0.599 |
| walker |  | 7289 | 40 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 25, sub: 0, line: 268 } |  |  | 0.601 |
| walker |  | 7331 | 42 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 28, sub: 0, line: 282 } |  |  | 0.601 |
| walker |  | 7376 | 45 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 15, sub: 0, line: 190 } |  |  | 0.610 |
| walker |  | 7423 | 47 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 22, sub: 0, line: 248 } |  |  | 0.610 |
| walker |  | 7472 | 49 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 21, sub: 0, line: 243 } |  |  | 0.614 |
| walker |  | 7532 | 60 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 16, sub: 0, line: 197 } |  |  | 0.627 |
| ns | 7567 |  | 344 | CLI options, second half | 5.4 |  | 0.612 |
| walker |  | 7627 | 95 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 23, sub: 0, line: 253 } |  |  | 0.627 |
| ns | 7675 |  | 108 | bin/svgo.js executable entry | 5.5 |  | 0.622 |
| ns | 7790 |  | 115 | parseSvg and the sax configuration | 6.1 |  | 0.616 |
| walker |  | 7805 | 178 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 33, sub: 0, line: 307 } |  |  | 0.616 |
| ns | 7913 |  | 123 | stringifier function roster | 6.2 |  | 0.611 |
| ns | 7996 |  | 83 | xast query helpers | 6.3 |  | 0.609 |
| walker |  | 8025 | 220 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 36, sub: 0, line: 336 } |  |  | 0.631 |
| ns | 8174 |  | 178 | lib/svgo/tools.js -- complete export set | 6.4 |  | 0.625 |
| walker |  | 8317 | 292 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 20, sub: 0, line: 214 } |  |  | 0.625 |
| ns | 8330 |  | 156 | lib/style.js -- exported and internal function roster | 6.5 |  | 0.619 |
| ns | 8476 |  | 146 | lib/path.js -- path data parse and stringify roster | 6.6 |  | 0.613 |
| walker |  | 8495 | 178 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 1, sub: 0, line: 30 } |  |  | 0.613 |
| ns | 8710 |  | 234 | plugins/_collections.js -- complete export roster | 6.7 |  | 0.606 |
| walker |  | 8756 | 261 | Code::CodeKey { rung: Decl, file: lib/types.ts, decl: 1, sub: 1, line: 30 } |  |  | 0.606 |
| ns | 8904 |  | 194 | Shared plugin helper module rosters | 6.8 |  | 0.601 |
| walker |  | 8990 | 234 | Code::CodeKey { rung: Names, file: plugins/_collections.js, decl: 0, sub: 0, line: 0 } |  |  | 0.615 |
| ns | 9017 |  | 113 | css-select adapter | 6.9 |  | 0.611 |
| ns | 9034 |  | 17 | mapNodesToParents | 6.10 |  | 0.610 |
| walker |  | 9054 | 64 | Code::CodeKey { rung: Decl, file: plugins/_collections.js, decl: 14, sub: 0, line: 2387 } |  |  | 0.610 |
| walker |  | 9136 | 82 | Code::CodeKey { rung: Decl, file: plugins/_collections.js, decl: 11, sub: 0, line: 2179 } |  |  | 0.610 |
| walker |  | 9237 | 101 | Code::CodeKey { rung: Decl, file: plugins/_collections.js, decl: 9, sub: 0, line: 2111 } |  |  | 0.610 |
| ns | 9268 |  | 234 | package.json scripts -- how to build, test and lint | 7.1 |  | 0.614 |
| ns | 9337 |  | 69 | test/ tree listing | 7.2 |  | 0.621 |
| walker |  | 9395 | 158 | Code::CodeKey { rung: Decl, file: plugins/_collections.js, decl: 6, sub: 0, line: 389 } |  |  | 0.621 |
| walker |  | 9442 | 47 | Code::CodeKey { rung: Names, file: plugins/prefixIds.js, decl: 0, sub: 0, line: 0 } |  |  | 0.622 |
| ns | 9452 |  | 115 | The fixture-driven plugin test format | 7.3 |  | 0.620 |
| walker |  | 9489 | 47 | Code::CodeKey { rung: Names, file: plugins/removeAttrs.js, decl: 0, sub: 0, line: 0 } |  |  | 0.620 |
| walker |  | 9536 | 47 | Code::CodeKey { rung: Names, file: plugins/removeDesc.js, decl: 0, sub: 0, line: 0 } |  |  | 0.621 |
| ns | 9582 |  | 130 | docs/ and remaining leaf directory listings | 7.4 |  | 0.628 |
| walker |  | 9583 | 47 | Code::CodeKey { rung: Names, file: plugins/removeEmptyContainers.js, decl: 0, sub: 0, line: 0 } |  |  | 0.629 |
| walker |  | 9615 | 32 | Code::CodeKey { rung: Names, file: lib/path.js, decl: 0, sub: 0, line: 0 } |  |  | 0.629 |
| walker |  | 9647 | 32 | Code::CodeKey { rung: Decl, file: lib/path.js, decl: 2, sub: 0, line: 302 } |  |  | 0.631 |
| walker |  | 9695 | 48 | Code::CodeKey { rung: Names, file: plugins/inlineStyles.js, decl: 0, sub: 0, line: 0 } |  |  | 0.633 |
| ns | 9717 |  | 135 | Migration guide outlines | 7.5 |  | 0.628 |
| walker |  | 9743 | 48 | Code::CodeKey { rung: Names, file: plugins/sortDefsChildren.js, decl: 0, sub: 0, line: 0 } |  |  | 0.630 |
| walker |  | 9759 | 16 | Code::CodeKey { rung: Names, file: lib/builtin.js, decl: 0, sub: 0, line: 0 } |  |  | 0.630 |
| walker |  | 9808 | 49 | Code::CodeKey { rung: Names, file: plugins/sortAttrs.js, decl: 0, sub: 0, line: 0 } |  |  | 0.632 |
| ns | 9841 |  | 124 | TypeScript and lint configuration | 7.6 |  | 0.627 |
| ns | 9966 |  | 125 | Runtime dependencies | 7.7 |  | 0.630 |
| walker |  | 9997 | 189 | Code::CodeKey { rung: Decl, file: plugins/_collections.js, decl: 13, sub: 0, line: 2348 } |  |  | 0.630 |
| walker |  | 9997 | 0 | Code::CodeKey { rung: Decl, file: plugins/_collections.js, decl: 10, sub: 0, line: 2128 } |  |  | 0.630 |

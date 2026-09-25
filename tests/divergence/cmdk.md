Score(3000)=0.537 I=0.784 C=0.368 ns_rows≤3K=16/47 grid(1000/1442/2080/3000/4327/6240/9000)=0.382/0.300/0.277/0.537/0.811/0.772/0.660

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 62 | 62 | Fs::DirListing { dir: . } |  |  | 1.000 |
| ns | 62 |  | 62 | Repo root listing | 1.1 |  | 1.000 |
| walker |  | 75 | 13 | Fs::DirListing { dir: cmdk } |  |  | 1.000 |
| walker |  | 102 | 27 | Json::Identity { file: package.json } |  |  | 1.000 |
| walker |  | 112 | 10 | Fs::DirListing { dir: cmdk/src } |  |  | 1.000 |
| walker |  | 154 | 42 | Json::Identity { file: cmdk/package.json } |  |  | 1.000 |
| walker |  | 159 | 5 | Fs::DirListing { dir: .husky } |  |  | 1.000 |
| walker |  | 167 | 8 | Fs::DirListing { dir: .github } |  |  | 1.000 |
| walker |  | 171 | 4 | Fs::DirListing { dir: .github/workflows } |  |  | 1.000 |
| ns | 174 |  | 112 | README one-line identity + install | 1.2 |  | 0.846 |
| walker |  | 192 | 21 | Json::Runtime { file: package.json } |  |  | 0.846 |
| ns | 208 |  | 34 | pnpm workspace membership | 1.3 |  | 0.758 |
| ns | 343 |  | 135 | Listings of all three packages + library source dir | 1.4 |  | 0.483 |
| walker |  | 451 | 259 | Code::CodeKey { rung: Names, file: cmdk/src/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.488 |
| walker |  | 473 | 22 | Code::CodeKey { rung: Decl, file: cmdk/src/index.tsx, decl: 16, sub: 0, line: 149 } |  |  | 0.488 |
| walker |  | 508 | 35 | Code::CodeKey { rung: Decl, file: cmdk/src/index.tsx, decl: 5, sub: 0, line: 24 } |  |  | 0.488 |
| ns | 532 |  | 189 | Published package identity + entry points | 1.5 |  | 0.417 |
| walker |  | 560 | 52 | Code::CodeKey { rung: Decl, file: cmdk/src/index.tsx, decl: 7, sub: 0, line: 37 } |  |  | 0.417 |
| walker |  | 617 | 57 | Code::CodeKey { rung: Decl, file: cmdk/src/index.tsx, decl: 14, sub: 0, line: 137 } |  |  | 0.417 |
| walker |  | 691 | 74 | Code::CodeKey { rung: Decl, file: cmdk/src/index.tsx, decl: 15, sub: 0, line: 143 } |  |  | 0.418 |
| ns | 764 |  | 232 | Root package.json: identity and every workspace script | 1.6 |  | 0.380 |
| walker |  | 766 | 75 | Code::CodeKey { rung: Decl, file: cmdk/src/index.tsx, decl: 3, sub: 0, line: 13 } |  |  | 0.380 |
| walker |  | 851 | 85 | Code::CodeKey { rung: Decl, file: cmdk/src/index.tsx, decl: 10, sub: 0, line: 69 } |  |  | 0.382 |
| walker |  | 944 | 93 | Code::CodeKey { rung: Decl, file: cmdk/src/index.tsx, decl: 6, sub: 0, line: 28 } |  |  | 0.382 |
| ns | 1043 |  | 279 | README section map (all H2 + H3 headings) | 1.7 |  | 0.335 |
| walker |  | 1152 | 208 | Code::CodeKey { rung: Names, file: cmdk/src/index.tsx, decl: 0, sub: 1, line: 0 } |  |  | 0.336 |
| ns | 1225 |  | 182 | ARCHITECTURE: the core invariant + section map | 1.8 |  | 0.317 |
| walker |  | 1263 | 111 | Code::CodeKey { rung: Decl, file: cmdk/src/index.tsx, decl: 9, sub: 0, line: 60 } |  |  | 0.319 |
| walker |  | 1320 | 57 | Markdown::HeadingsOutline { file: ARCHITECTURE.md } |  |  | 0.327 |
| ns | 1361 |  | 136 | README testing steps (verbatim) | 1.9 | 1.7 | 0.300 |
| walker |  | 1416 | 96 | Json::Dependencies { file: cmdk/package.json } |  |  | 0.300 |
| ns | 1473 |  | 112 | index.tsx imports + 'use client' | 2.1 |  | 0.290 |
| walker |  | 1593 | 177 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.299 |
| ns | 1690 |  | 217 | Public export surface (Command.* object + named exports) | 2.2 |  | 0.273 |
| walker |  | 1758 | 165 | Code::CodeKey { rung: Decl, file: cmdk/src/index.tsx, decl: 13, sub: 0, line: 123 } |  |  | 0.275 |
| walker |  | 1963 | 205 | Code::CodeKey { rung: Names, file: cmdk/src/index.tsx, decl: 0, sub: 2, line: 0 } |  |  | 0.277 |
| ns | 2118 |  | 428 | README FAQ (all twelve entries) | 2.3 | 1.7 | 0.256 |
| walker |  | 2188 | 225 | Code::CodeKey { rung: Decl, file: cmdk/src/index.tsx, decl: 8, sub: 0, line: 44 } |  |  | 0.258 |
| walker |  | 2268 | 80 | Json::Scripts { file: cmdk/package.json } |  |  | 0.295 |
| ns | 2297 |  | 179 | Component declaration roster (all nine forwardRef components) | 2.4 |  | 0.326 |
| walker |  | 2322 | 54 | Fs::DirListing { dir: website } |  |  | 0.380 |
| walker |  | 2336 | 14 | Fs::DirListing { dir: website/components } |  |  | 0.380 |
| walker |  | 2394 | 58 | Fs::DirListing { dir: test } |  |  | 0.488 |
| ns | 2406 |  | 109 | Props type roster (all twelve type aliases) | 2.5 |  | 0.516 |
| ns | 2730 |  | 324 | Per-component JSDoc blocks | 2.6 | 2.4 | 0.478 |
| walker |  | 2776 | 382 | Code::CodeKey { rung: Names, file: cmdk/src/index.tsx, decl: 0, sub: 3, line: 0 } |  |  | 0.528 |
| walker |  | 2835 | 59 | Code::CodeKey { rung: Decl, file: cmdk/src/index.tsx, decl: 39, sub: 0, line: 930 } |  |  | 0.565 |
| walker |  | 2885 | 50 | Code::CodeKey { rung: Decl, file: cmdk/src/index.tsx, decl: 49, sub: 0, line: 1071 } |  |  | 0.565 |
| walker |  | 2950 | 65 | Code::CodeKey { rung: Decl, file: cmdk/src/index.tsx, decl: 46, sub: 0, line: 1010 } |  |  | 0.565 |
| ns | 2984 |  | 254 | CommandProps: label, shouldFilter, filter, defaultValue | 2.7 | 2.5 | 0.537 |
| walker |  | 3072 | 122 | Code::CodeKey { rung: Decl, file: cmdk/src/index.tsx, decl: 50, sub: 0, line: 1081 } |  |  | 0.537 |
| walker |  | 3089 | 17 | Fs::DirListing { dir: website/pages } |  |  | 0.537 |
| ns | 3203 |  | 219 | CommandProps: value, onValueChange, loop, disablePointerSelection, vimBindings | 2.8 | 2.7 | 0.515 |
| walker |  | 3234 | 145 | Json::Entry { file: cmdk/package.json } |  |  | 0.550 |
| ns | 3430 |  | 227 | ItemProps (full, with JSDoc) | 2.9 | 2.5 | 0.571 |
| walker |  | 3486 | 252 | Code::CodeKey { rung: Decl, file: cmdk/src/index.tsx, decl: 12, sub: 0, line: 80 } |  |  | 0.622 |
| walker |  | 3520 | 34 | Plaintext::Whole { file: pnpm-workspace.yaml } |  |  | 0.642 |
| walker |  | 3543 | 23 | Fs::DirListing { dir: website/components/cmdk } |  |  | 0.642 |
| ns | 3588 |  | 158 | GroupProps, SeparatorProps, EmptyProps | 2.10 | 2.5 | 0.654 |
| ns | 3715 |  | 127 | InputProps + CommandFilter signature | 2.11 | 2.5 | 0.663 |
| walker |  | 3831 | 288 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.722 |
| walker |  | 3857 | 26 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.738 |
| walker |  | 4059 | 202 | Json::Scripts { file: package.json } |  |  | 0.769 |
| ns | 4080 |  | 365 | ARCHITECTURE: the three rejected APIs and why selection tracks value | 3.1 | 1.8 | 0.764 |
| walker |  | 4091 | 32 | Markdown::Section { file: ARCHITECTURE.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.768 |
| walker |  | 4308 | 217 | Code::CodeKey { rung: Decl, file: cmdk/src/index.tsx, decl: 12, sub: 1, line: 80 } |  |  | 0.811 |
| walker |  | 4323 | 15 | Code::CodeKey { rung: Doc, file: cmdk/src/index.tsx, decl: 45, sub: 0, line: 1004 } |  |  | 0.811 |
| walker |  | 4349 | 26 | Code::CodeKey { rung: Names, file: cmdk/src/command-score.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.811 |
| walker |  | 4409 | 60 | Fs::DirListing { dir: test/pages } |  |  | 0.813 |
| walker |  | 4431 | 22 | Code::CodeKey { rung: Names, file: test/pages/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.813 |
| ns | 4438 |  | 358 | Internal types: Context, State, Store, Group | 3.2 |  | 0.821 |
| walker |  | 4603 | 172 | Json::IdentityMeta { file: cmdk/package.json } |  |  | 0.821 |
| walker |  | 4629 | 26 | Code::CodeKey { rung: Doc, file: cmdk/src/index.tsx, decl: 36, sub: 0, line: 882 } |  |  | 0.821 |
| ns | 4659 |  | 221 | DOM selector constants + the three React contexts | 3.3 |  | 0.824 |
| walker |  | 4697 | 68 | Markdown::Section { file: ARCHITECTURE.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.824 |
| walker |  | 4724 | 27 | Code::CodeKey { rung: Doc, file: cmdk/src/index.tsx, decl: 37, sub: 0, line: 899 } |  |  | 0.826 |
| walker |  | 4865 | 141 | Markdown::Section { file: README.md, section_index: 15, keeps_default_concavity: false } |  |  | 0.862 |
| ns | 4873 |  | 214 | Roster of Command's internal functions | 3.4 |  | 0.845 |
| ns | 5086 |  | 213 | Store: subscribe/snapshot and the 'search' setState branch | 3.5 |  | 0.821 |
| walker |  | 5175 | 310 | Markdown::Section { file: LICENSE.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.821 |
| ns | 5300 |  | 214 | Root keydown handler: IME guard, vim down bindings, ArrowDown | 3.6 | 3.4 | 0.794 |
| walker |  | 5326 | 151 | Markdown::Section { file: README.md, section_index: 14, keeps_default_concavity: false } |  |  | 0.794 |
| walker |  | 5356 | 30 | Code::CodeKey { rung: Doc, file: cmdk/src/index.tsx, decl: 38, sub: 0, line: 909 } |  |  | 0.796 |
| walker |  | 5536 | 180 | Markdown::Section { file: README.md, section_index: 12, keeps_default_concavity: false } |  |  | 0.802 |
| ns | 5678 |  | 378 | score() and sort(): per-group maximum score and the ordering rules | 4.1 | 3.4 | 0.765 |
| walker |  | 5784 | 248 | Markdown::Section { file: README.md, section_index: 13, keeps_default_concavity: true } |  |  | 0.796 |
| ns | 6027 |  | 349 | sort(): the DOM re-append loop | 4.2 | 4.1 | 0.769 |
| walker |  | 6223 | 439 | Markdown::Section { file: ARCHITECTURE.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.772 |
| walker |  | 6260 | 37 | Code::CodeKey { rung: Doc, file: cmdk/src/index.tsx, decl: 32, sub: 0, line: 729 } |  |  | 0.775 |
| walker |  | 6298 | 38 | Code::CodeKey { rung: Doc, file: cmdk/src/index.tsx, decl: 34, sub: 0, line: 787 } |  |  | 0.780 |
| walker |  | 6311 | 13 | Fs::DirListing { dir: website/styles } |  |  | 0.780 |
| ns | 6432 |  | 405 | filterItems(): scoring every item and deriving visible groups | 4.3 | 3.4 | 0.748 |
| walker |  | 6607 | 296 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.748 |
| ns | 6702 |  | 270 | Item: value inference, render gate, select handlers | 4.4 | 2.6 | 0.731 |
| walker |  | 6851 | 244 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: true } |  |  | 0.731 |
| ns | 6880 |  | 178 | Item: rendered element and its attributes | 4.5 | 4.4 | 0.719 |
| walker |  | 6923 | 72 | Code::CodeKey { rung: Names, file: website/components/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.719 |
| walker |  | 6971 | 48 | Code::CodeKey { rung: Doc, file: cmdk/src/index.tsx, decl: 33, sub: 0, line: 774 } |  |  | 0.726 |
| walker |  | 6981 | 10 | Fs::DirListing { dir: website/components/code } |  |  | 0.726 |
| walker |  | 6991 | 10 | Fs::DirListing { dir: website/components/icons } |  |  | 0.726 |
| walker |  | 7037 | 46 | Code::CodeKey { rung: Body, file: cmdk/src/index.tsx, decl: 45, sub: 0, line: 1004 } |  |  | 0.726 |
| walker |  | 7089 | 52 | Code::CodeKey { rung: Doc, file: cmdk/src/index.tsx, decl: 35, sub: 0, line: 833 } |  |  | 0.734 |
| ns | 7137 |  | 257 | Group: render gate and heading/items markup | 4.6 | 2.6 | 0.719 |
| ns | 7329 |  | 192 | Input and Separator markup | 4.7 | 2.6 | 0.708 |
| ns | 7595 |  | 266 | List, Dialog, Empty and Loading markup | 4.8 | 2.6 | 0.696 |
| walker |  | 7613 | 524 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.696 |
| ns | 7795 |  | 200 | Roster of module-level helpers | 4.9 |  | 0.698 |
| walker |  | 8039 | 426 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.698 |
| ns | 8098 |  | 303 | command-score: the full scoring weight table | 5.1 |  | 0.684 |
| ns | 8194 |  | 96 | command-score: exported signature and alias handling | 5.2 | 5.1 | 0.682 |
| ns | 8254 |  | 60 | Test fixture pages listing | 6.1 |  | 0.686 |
| ns | 8432 |  | 178 | Spec names: basic behaviour | 6.2 |  | 0.680 |
| walker |  | 8513 | 474 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.680 |
| ns | 8577 |  | 145 | dialog.test.ts in full - the whole spec idiom | 6.3 |  | 0.673 |
| walker |  | 8662 | 149 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.673 |
| ns | 8740 |  | 163 | Spec names: item lifecycle and item-advanced | 6.4 |  | 0.668 |
| walker |  | 8904 | 242 | Json::Whole { file: tsconfig.json } |  |  | 0.668 |
| ns | 8957 |  | 217 | Spec names: group, props matrix, numeric values | 6.5 |  | 0.660 |
| walker |  | 9148 | 244 | Markdown::Section { file: ARCHITECTURE.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.669 |
| ns | 9217 |  | 260 | Spec names: keybinds (four describe blocks) | 6.6 |  | 0.661 |
| walker |  | 9358 | 210 | Markdown::Section { file: ARCHITECTURE.md, section_index: 2, keeps_default_concavity: true } |  |  | 0.664 |
| ns | 9473 |  | 256 | test/pages/dialog.tsx - a working usage page | 6.7 |  | 0.652 |
| walker |  | 9560 | 202 | Code::CodeKey { rung: Names, file: website/pages/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.652 |
| walker |  | 9582 | 22 | Code::CodeKey { rung: Decl, file: website/pages/index.tsx, decl: 1, sub: 0, line: 20 } |  |  | 0.652 |
| walker |  | 9601 | 19 | Code::CodeKey { rung: Doc, file: cmdk/src/index.tsx, decl: 47, sub: 0, line: 1046 } |  |  | 0.654 |
| ns | 9749 |  | 276 | Playwright config (whole file) | 6.8 |  | 0.642 |
| ns | 9914 |  | 165 | CI workflow | 6.9 |  | 0.633 |
| ns | 9987 |  | 73 | Website source listings (demos, pages, drop-in stylesheets) | 7.1 |  | 0.633 |

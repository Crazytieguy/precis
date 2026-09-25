Score(3000)=0.629 I=0.900 C=0.440 ns_rows≤3K=16/47 grid(1000/1442/2080/3000/4327/6240/9000)=0.846/0.793/0.761/0.629/0.832/0.810/0.678

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
| walker |  | 249 | 57 | Markdown::HeadingsOutline { file: ARCHITECTURE.md } |  |  | 0.758 |
| ns | 343 |  | 135 | Listings of all three packages + library source dir | 1.4 |  | 0.484 |
| walker |  | 345 | 96 | Json::Dependencies { file: cmdk/package.json } |  |  | 0.484 |
| walker |  | 522 | 177 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.500 |
| ns | 532 |  | 189 | Published package identity + entry points | 1.5 |  | 0.426 |
| walker |  | 602 | 80 | Json::Scripts { file: cmdk/package.json } |  |  | 0.490 |
| walker |  | 656 | 54 | Fs::DirListing { dir: website } |  |  | 0.594 |
| walker |  | 670 | 14 | Fs::DirListing { dir: website/components } |  |  | 0.594 |
| walker |  | 687 | 17 | Fs::DirListing { dir: website/pages } |  |  | 0.595 |
| walker |  | 745 | 58 | Fs::DirListing { dir: test } |  |  | 0.797 |
| ns | 764 |  | 232 | Root package.json: identity and every workspace script | 1.6 |  | 0.722 |
| walker |  | 768 | 23 | Fs::DirListing { dir: website/components/cmdk } |  |  | 0.723 |
| walker |  | 913 | 145 | Json::Entry { file: cmdk/package.json } |  |  | 0.805 |
| walker |  | 947 | 34 | Plaintext::Whole { file: pnpm-workspace.yaml } |  |  | 0.846 |
| ns | 1043 |  | 279 | README section map (all H2 + H3 headings) | 1.7 |  | 0.742 |
| ns | 1225 |  | 182 | ARCHITECTURE: the core invariant + section map | 1.8 |  | 0.704 |
| walker |  | 1235 | 288 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.835 |
| walker |  | 1261 | 26 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.863 |
| ns | 1361 |  | 136 | README testing steps (verbatim) | 1.9 | 1.7 | 0.793 |
| walker |  | 1463 | 202 | Json::Scripts { file: package.json } |  |  | 0.854 |
| ns | 1473 |  | 112 | index.tsx imports + 'use client' | 2.1 |  | 0.826 |
| ns | 1690 |  | 217 | Public export surface (Command.* object + named exports) | 2.2 |  | 0.754 |
| walker |  | 1722 | 259 | Code::CodeKey { rung: Names, file: cmdk/src/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.758 |
| walker |  | 1744 | 22 | Code::CodeKey { rung: Decl, file: cmdk/src/index.tsx, decl: 16, sub: 0, line: 149 } |  |  | 0.759 |
| walker |  | 1779 | 35 | Code::CodeKey { rung: Decl, file: cmdk/src/index.tsx, decl: 5, sub: 0, line: 24 } |  |  | 0.759 |
| walker |  | 1831 | 52 | Code::CodeKey { rung: Decl, file: cmdk/src/index.tsx, decl: 7, sub: 0, line: 37 } |  |  | 0.759 |
| walker |  | 1888 | 57 | Code::CodeKey { rung: Decl, file: cmdk/src/index.tsx, decl: 14, sub: 0, line: 137 } |  |  | 0.760 |
| walker |  | 1962 | 74 | Code::CodeKey { rung: Decl, file: cmdk/src/index.tsx, decl: 15, sub: 0, line: 143 } |  |  | 0.761 |
| walker |  | 2037 | 75 | Code::CodeKey { rung: Decl, file: cmdk/src/index.tsx, decl: 3, sub: 0, line: 13 } |  |  | 0.761 |
| ns | 2118 |  | 428 | README FAQ (all twelve entries) | 2.3 | 1.7 | 0.705 |
| walker |  | 2122 | 85 | Code::CodeKey { rung: Decl, file: cmdk/src/index.tsx, decl: 10, sub: 0, line: 69 } |  |  | 0.707 |
| walker |  | 2215 | 93 | Code::CodeKey { rung: Decl, file: cmdk/src/index.tsx, decl: 6, sub: 0, line: 28 } |  |  | 0.707 |
| ns | 2297 |  | 179 | Component declaration roster (all nine forwardRef components) | 2.4 |  | 0.688 |
| walker |  | 2326 | 111 | Code::CodeKey { rung: Decl, file: cmdk/src/index.tsx, decl: 9, sub: 0, line: 60 } |  |  | 0.691 |
| ns | 2406 |  | 109 | Props type roster (all twelve type aliases) | 2.5 |  | 0.705 |
| walker |  | 2491 | 165 | Code::CodeKey { rung: Decl, file: cmdk/src/index.tsx, decl: 13, sub: 0, line: 123 } |  |  | 0.709 |
| walker |  | 2716 | 225 | Code::CodeKey { rung: Decl, file: cmdk/src/index.tsx, decl: 8, sub: 0, line: 44 } |  |  | 0.713 |
| ns | 2730 |  | 324 | Per-component JSDoc blocks | 2.6 | 2.4 | 0.660 |
| walker |  | 2924 | 208 | Code::CodeKey { rung: Names, file: cmdk/src/index.tsx, decl: 0, sub: 1, line: 0 } |  |  | 0.662 |
| ns | 2984 |  | 254 | CommandProps: label, shouldFilter, filter, defaultValue | 2.7 | 2.5 | 0.629 |
| walker |  | 3129 | 205 | Code::CodeKey { rung: Names, file: cmdk/src/index.tsx, decl: 0, sub: 2, line: 0 } |  |  | 0.650 |
| walker |  | 3155 | 26 | Code::CodeKey { rung: Doc, file: cmdk/src/index.tsx, decl: 36, sub: 0, line: 882 } |  |  | 0.651 |
| walker |  | 3182 | 27 | Code::CodeKey { rung: Doc, file: cmdk/src/index.tsx, decl: 37, sub: 0, line: 899 } |  |  | 0.653 |
| ns | 3203 |  | 219 | CommandProps: value, onValueChange, loop, disablePointerSelection, vimBindings | 2.8 | 2.7 | 0.626 |
| walker |  | 3219 | 37 | Code::CodeKey { rung: Doc, file: cmdk/src/index.tsx, decl: 32, sub: 0, line: 729 } |  |  | 0.630 |
| walker |  | 3257 | 38 | Code::CodeKey { rung: Doc, file: cmdk/src/index.tsx, decl: 34, sub: 0, line: 787 } |  |  | 0.637 |
| walker |  | 3305 | 48 | Code::CodeKey { rung: Doc, file: cmdk/src/index.tsx, decl: 33, sub: 0, line: 774 } |  |  | 0.648 |
| walker |  | 3357 | 52 | Code::CodeKey { rung: Doc, file: cmdk/src/index.tsx, decl: 35, sub: 0, line: 833 } |  |  | 0.663 |
| walker |  | 3423 | 66 | Code::CodeKey { rung: Doc, file: cmdk/src/index.tsx, decl: 31, sub: 0, line: 664 } |  |  | 0.688 |
| ns | 3430 |  | 227 | ItemProps (full, with JSDoc) | 2.9 | 2.5 | 0.702 |
| ns | 3588 |  | 158 | GroupProps, SeparatorProps, EmptyProps | 2.10 | 2.5 | 0.713 |
| walker |  | 3675 | 252 | Code::CodeKey { rung: Decl, file: cmdk/src/index.tsx, decl: 12, sub: 0, line: 80 } |  |  | 0.759 |
| ns | 3715 |  | 127 | InputProps + CommandFilter signature | 2.11 | 2.5 | 0.764 |
| walker |  | 3892 | 217 | Code::CodeKey { rung: Decl, file: cmdk/src/index.tsx, decl: 12, sub: 1, line: 80 } |  |  | 0.807 |
| ns | 4080 |  | 365 | ARCHITECTURE: the three rejected APIs and why selection tracks value | 3.1 | 1.8 | 0.802 |
| walker |  | 4274 | 382 | Code::CodeKey { rung: Names, file: cmdk/src/index.tsx, decl: 0, sub: 3, line: 0 } |  |  | 0.832 |
| walker |  | 4324 | 50 | Code::CodeKey { rung: Decl, file: cmdk/src/index.tsx, decl: 49, sub: 0, line: 1071 } |  |  | 0.832 |
| walker |  | 4383 | 59 | Code::CodeKey { rung: Decl, file: cmdk/src/index.tsx, decl: 39, sub: 0, line: 930 } |  |  | 0.856 |
| ns | 4438 |  | 358 | Internal types: Context, State, Store, Group | 3.2 |  | 0.860 |
| walker |  | 4448 | 65 | Code::CodeKey { rung: Decl, file: cmdk/src/index.tsx, decl: 46, sub: 0, line: 1010 } |  |  | 0.860 |
| walker |  | 4463 | 15 | Code::CodeKey { rung: Doc, file: cmdk/src/index.tsx, decl: 45, sub: 0, line: 1004 } |  |  | 0.861 |
| walker |  | 4482 | 19 | Code::CodeKey { rung: Doc, file: cmdk/src/index.tsx, decl: 47, sub: 0, line: 1046 } |  |  | 0.861 |
| walker |  | 4604 | 122 | Code::CodeKey { rung: Decl, file: cmdk/src/index.tsx, decl: 50, sub: 0, line: 1081 } |  |  | 0.861 |
| walker |  | 4634 | 30 | Code::CodeKey { rung: Doc, file: cmdk/src/index.tsx, decl: 38, sub: 0, line: 909 } |  |  | 0.871 |
| ns | 4659 |  | 221 | DOM selector constants + the three React contexts | 3.3 |  | 0.873 |
| walker |  | 4669 | 35 | Code::CodeKey { rung: Doc, file: cmdk/src/index.tsx, decl: 40, sub: 0, line: 963 } |  |  | 0.873 |
| walker |  | 4701 | 32 | Markdown::Section { file: ARCHITECTURE.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.876 |
| walker |  | 4761 | 60 | Fs::DirListing { dir: test/pages } |  |  | 0.878 |
| ns | 4873 |  | 214 | Roster of Command's internal functions | 3.4 |  | 0.860 |
| walker |  | 4933 | 172 | Json::IdentityMeta { file: cmdk/package.json } |  |  | 0.860 |
| walker |  | 5001 | 68 | Markdown::Section { file: ARCHITECTURE.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.860 |
| ns | 5086 |  | 213 | Store: subscribe/snapshot and the 'search' setState branch | 3.5 |  | 0.836 |
| walker |  | 5142 | 141 | Markdown::Section { file: README.md, section_index: 15, keeps_default_concavity: false } |  |  | 0.870 |
| ns | 5300 |  | 214 | Root keydown handler: IME guard, vim down bindings, ArrowDown | 3.6 | 3.4 | 0.841 |
| walker |  | 5452 | 310 | Markdown::Section { file: LICENSE.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.841 |
| walker |  | 5603 | 151 | Markdown::Section { file: README.md, section_index: 14, keeps_default_concavity: false } |  |  | 0.841 |
| walker |  | 5616 | 13 | Fs::DirListing { dir: website/styles } |  |  | 0.841 |
| ns | 5678 |  | 378 | score() and sort(): per-group maximum score and the ordering rules | 4.1 | 3.4 | 0.803 |
| walker |  | 5796 | 180 | Markdown::Section { file: README.md, section_index: 12, keeps_default_concavity: false } |  |  | 0.808 |
| ns | 6027 |  | 349 | sort(): the DOM re-append loop | 4.2 | 4.1 | 0.781 |
| walker |  | 6044 | 248 | Markdown::Section { file: README.md, section_index: 13, keeps_default_concavity: true } |  |  | 0.810 |
| ns | 6432 |  | 405 | filterItems(): scoring every item and deriving visible groups | 4.3 | 3.4 | 0.777 |
| walker |  | 6483 | 439 | Markdown::Section { file: ARCHITECTURE.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.780 |
| walker |  | 6493 | 10 | Fs::DirListing { dir: website/components/code } |  |  | 0.780 |
| walker |  | 6503 | 10 | Fs::DirListing { dir: website/components/icons } |  |  | 0.780 |
| walker |  | 6529 | 26 | Code::CodeKey { rung: Names, file: cmdk/src/command-score.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.780 |
| walker |  | 6657 | 128 | Code::CodeKey { rung: Body, file: cmdk/src/command-score.ts, decl: 1, sub: 0, line: 155 } |  |  | 0.780 |
| ns | 6702 |  | 270 | Item: value inference, render gate, select handlers | 4.4 | 2.6 | 0.762 |
| ns | 6880 |  | 178 | Item: rendered element and its attributes | 4.5 | 4.4 | 0.749 |
| walker |  | 6953 | 296 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.749 |
| ns | 7137 |  | 257 | Group: render gate and heading/items markup | 4.6 | 2.6 | 0.734 |
| walker |  | 7197 | 244 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: true } |  |  | 0.734 |
| walker |  | 7216 | 19 | Fs::DirListing { dir: website/styles/cmdk } |  |  | 0.734 |
| walker |  | 7260 | 44 | Fs::DirListing { dir: website/public } |  |  | 0.734 |
| ns | 7329 |  | 192 | Input and Separator markup | 4.7 | 2.6 | 0.722 |
| ns | 7595 |  | 266 | List, Dialog, Empty and Loading markup | 4.8 | 2.6 | 0.710 |
| walker |  | 7784 | 524 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.710 |
| ns | 7795 |  | 200 | Roster of module-level helpers | 4.9 |  | 0.715 |
| ns | 8098 |  | 303 | command-score: the full scoring weight table | 5.1 |  | 0.700 |
| ns | 8194 |  | 96 | command-score: exported signature and alias handling | 5.2 | 5.1 | 0.701 |
| walker |  | 8210 | 426 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.701 |
| ns | 8254 |  | 60 | Test fixture pages listing | 6.1 |  | 0.704 |
| ns | 8432 |  | 178 | Spec names: basic behaviour | 6.2 |  | 0.698 |
| ns | 8577 |  | 145 | dialog.test.ts in full - the whole spec idiom | 6.3 |  | 0.691 |
| walker |  | 8684 | 474 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.691 |
| ns | 8740 |  | 163 | Spec names: item lifecycle and item-advanced | 6.4 |  | 0.686 |
| walker |  | 8833 | 149 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.686 |
| ns | 8957 |  | 217 | Spec names: group, props matrix, numeric values | 6.5 |  | 0.678 |
| walker |  | 9075 | 242 | Json::Whole { file: tsconfig.json } |  |  | 0.678 |
| ns | 9217 |  | 260 | Spec names: keybinds (four describe blocks) | 6.6 |  | 0.670 |
| walker |  | 9319 | 244 | Markdown::Section { file: ARCHITECTURE.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.679 |
| ns | 9473 |  | 256 | test/pages/dialog.tsx - a working usage page | 6.7 |  | 0.666 |
| walker |  | 9529 | 210 | Markdown::Section { file: ARCHITECTURE.md, section_index: 2, keeps_default_concavity: true } |  |  | 0.669 |
| ns | 9749 |  | 276 | Playwright config (whole file) | 6.8 |  | 0.656 |
| ns | 9914 |  | 165 | CI workflow | 6.9 |  | 0.647 |
| ns | 9987 |  | 73 | Website source listings (demos, pages, drop-in stylesheets) | 7.1 |  | 0.653 |

Score(3000)=0.817 I=0.932 C=0.715 ns_rows≤3K=16/47 grid(1000/1442/2080/3000/4327/6240/9000)=0.857/0.825/0.767/0.817/0.765/0.605/0.514

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
| walker |  | 426 | 177 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.500 |
| walker |  | 506 | 80 | Json::Scripts { file: cmdk/package.json } |  |  | 0.506 |
| ns | 532 |  | 189 | Published package identity + entry points | 1.5 |  | 0.490 |
| walker |  | 602 | 96 | Json::Dependencies { file: cmdk/package.json } |  |  | 0.490 |
| walker |  | 656 | 54 | Fs::DirListing { dir: website } |  |  | 0.594 |
| walker |  | 670 | 14 | Fs::DirListing { dir: website/components } |  |  | 0.594 |
| walker |  | 687 | 17 | Fs::DirListing { dir: website/pages } |  |  | 0.595 |
| walker |  | 745 | 58 | Fs::DirListing { dir: test } |  |  | 0.797 |
| ns | 764 |  | 232 | Root package.json: identity and every workspace script | 1.6 |  | 0.722 |
| walker |  | 768 | 23 | Fs::DirListing { dir: website/components/cmdk } |  |  | 0.723 |
| walker |  | 913 | 145 | Json::Entry { file: cmdk/package.json } |  |  | 0.804 |
| walker |  | 947 | 34 | Plaintext::Whole { file: pnpm-workspace.yaml } |  |  | 0.846 |
| ns | 1043 |  | 279 | README section map (all H2 + H3 headings) | 1.7 |  | 0.741 |
| ns | 1225 |  | 182 | ARCHITECTURE: the core invariant + section map | 1.8 |  | 0.702 |
| walker |  | 1235 | 288 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.833 |
| walker |  | 1261 | 26 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.862 |
| ns | 1361 |  | 136 | README testing steps (verbatim) | 1.9 | 1.7 | 0.792 |
| walker |  | 1463 | 202 | Json::Scripts { file: package.json } |  |  | 0.853 |
| ns | 1473 |  | 112 | index.tsx imports + 'use client' | 2.1 |  | 0.825 |
| walker |  | 1685 | 222 | Code::CodeKey { rung: Names, file: cmdk/src/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.827 |
| ns | 1690 |  | 217 | Public export surface (Command.* object + named exports) | 2.2 |  | 0.755 |
| walker |  | 1709 | 24 | Code::CodeKey { rung: Decl, file: cmdk/src/index.tsx, decl: 1, sub: 0, line: 149 } |  |  | 0.755 |
| walker |  | 1735 | 26 | Code::CodeKey { rung: Doc, file: cmdk/src/index.tsx, decl: 9, sub: 0, line: 882 } |  |  | 0.755 |
| walker |  | 1772 | 37 | Code::CodeKey { rung: Doc, file: cmdk/src/index.tsx, decl: 5, sub: 0, line: 729 } |  |  | 0.755 |
| walker |  | 1810 | 38 | Code::CodeKey { rung: Doc, file: cmdk/src/index.tsx, decl: 7, sub: 0, line: 787 } |  |  | 0.756 |
| walker |  | 1858 | 48 | Code::CodeKey { rung: Doc, file: cmdk/src/index.tsx, decl: 6, sub: 0, line: 774 } |  |  | 0.757 |
| walker |  | 1910 | 52 | Code::CodeKey { rung: Doc, file: cmdk/src/index.tsx, decl: 8, sub: 0, line: 833 } |  |  | 0.758 |
| walker |  | 1976 | 66 | Code::CodeKey { rung: Doc, file: cmdk/src/index.tsx, decl: 4, sub: 0, line: 664 } |  |  | 0.761 |
| ns | 2118 |  | 428 | README FAQ (all twelve entries) | 2.3 | 1.7 | 0.706 |
| walker |  | 2208 | 232 | Code::CodeKey { rung: Names, file: cmdk/src/index.tsx, decl: 0, sub: 1, line: 0 } |  |  | 0.752 |
| walker |  | 2267 | 59 | Code::CodeKey { rung: Decl, file: cmdk/src/index.tsx, decl: 12, sub: 0, line: 930 } |  |  | 0.792 |
| walker |  | 2282 | 15 | Code::CodeKey { rung: Doc, file: cmdk/src/index.tsx, decl: 13, sub: 0, line: 1004 } |  |  | 0.792 |
| ns | 2297 |  | 179 | Component declaration roster (all nine forwardRef components) | 2.4 |  | 0.798 |
| walker |  | 2309 | 27 | Code::CodeKey { rung: Doc, file: cmdk/src/index.tsx, decl: 10, sub: 0, line: 899 } |  |  | 0.799 |
| walker |  | 2339 | 30 | Code::CodeKey { rung: Doc, file: cmdk/src/index.tsx, decl: 11, sub: 0, line: 909 } |  |  | 0.801 |
| walker |  | 2371 | 32 | Markdown::Section { file: ARCHITECTURE.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.807 |
| ns | 2406 |  | 109 | Props type roster (all twelve type aliases) | 2.5 |  | 0.780 |
| walker |  | 2431 | 60 | Fs::DirListing { dir: test/pages } |  |  | 0.781 |
| walker |  | 2572 | 141 | Markdown::Section { file: README.md, section_index: 15, keeps_default_concavity: false } |  |  | 0.842 |
| walker |  | 2723 | 151 | Markdown::Section { file: README.md, section_index: 14, keeps_default_concavity: false } |  |  | 0.842 |
| ns | 2730 |  | 324 | Per-component JSDoc blocks | 2.6 | 2.4 | 0.851 |
| walker |  | 2769 | 46 | Code::CodeKey { rung: Body, file: cmdk/src/index.tsx, decl: 13, sub: 0, line: 1004 } |  |  | 0.851 |
| walker |  | 2949 | 180 | Markdown::Section { file: README.md, section_index: 12, keeps_default_concavity: false } |  |  | 0.859 |
| ns | 2984 |  | 254 | CommandProps: label, shouldFilter, filter, defaultValue | 2.7 | 2.5 | 0.817 |
| walker |  | 3017 | 68 | Markdown::Section { file: ARCHITECTURE.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.817 |
| walker |  | 3030 | 13 | Fs::DirListing { dir: website/styles } |  |  | 0.817 |
| ns | 3203 |  | 219 | CommandProps: value, onValueChange, loop, disablePointerSelection, vimBindings | 2.8 | 2.7 | 0.782 |
| walker |  | 3278 | 248 | Markdown::Section { file: README.md, section_index: 13, keeps_default_concavity: false } |  |  | 0.828 |
| ns | 3430 |  | 227 | ItemProps (full, with JSDoc) | 2.9 | 2.5 | 0.802 |
| walker |  | 3574 | 296 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.802 |
| ns | 3588 |  | 158 | GroupProps, SeparatorProps, EmptyProps | 2.10 | 2.5 | 0.781 |
| ns | 3715 |  | 127 | InputProps + CommandFilter signature | 2.11 | 2.5 | 0.766 |
| walker |  | 3818 | 244 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.766 |
| walker |  | 3884 | 66 | Code::CodeKey { rung: Body, file: cmdk/src/index.tsx, decl: 10, sub: 0, line: 899 } |  |  | 0.766 |
| ns | 4080 |  | 365 | ARCHITECTURE: the three rejected APIs and why selection tracks value | 3.1 | 1.8 | 0.761 |
| walker |  | 4323 | 439 | Markdown::Section { file: ARCHITECTURE.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.765 |
| walker |  | 4333 | 10 | Fs::DirListing { dir: website/components/code } |  |  | 0.765 |
| walker |  | 4343 | 10 | Fs::DirListing { dir: website/components/icons } |  |  | 0.765 |
| walker |  | 4369 | 26 | Code::CodeKey { rung: Names, file: cmdk/src/command-score.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.765 |
| ns | 4438 |  | 358 | Internal types: Context, State, Store, Group | 3.2 |  | 0.727 |
| walker |  | 4497 | 128 | Code::CodeKey { rung: Body, file: cmdk/src/command-score.ts, decl: 1, sub: 0, line: 155 } |  |  | 0.728 |
| walker |  | 4601 | 104 | Code::CodeKey { rung: Body, file: cmdk/src/index.tsx, decl: 6, sub: 0, line: 774 } |  |  | 0.728 |
| ns | 4659 |  | 221 | DOM selector constants + the three React contexts | 3.3 |  | 0.712 |
| ns | 4873 |  | 214 | Roster of Command's internal functions | 3.4 |  | 0.698 |
| ns | 5086 |  | 213 | Store: subscribe/snapshot and the 'search' setState branch | 3.5 |  | 0.678 |
| walker |  | 5125 | 524 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.678 |
| ns | 5300 |  | 214 | Root keydown handler: IME guard, vim down bindings, ArrowDown | 3.6 | 3.4 | 0.656 |
| walker |  | 5551 | 426 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.656 |
| ns | 5678 |  | 378 | score() and sort(): per-group maximum score and the ordering rules | 4.1 | 3.4 | 0.626 |
| walker |  | 6025 | 474 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.626 |
| ns | 6027 |  | 349 | sort(): the DOM re-append loop | 4.2 | 4.1 | 0.605 |
| walker |  | 6174 | 149 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.605 |
| ns | 6432 |  | 405 | filterItems(): scoring every item and deriving visible groups | 4.3 | 3.4 | 0.580 |
| ns | 6702 |  | 270 | Item: value inference, render gate, select handlers | 4.4 | 2.6 | 0.567 |
| walker |  | 6815 | 641 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.567 |
| ns | 6880 |  | 178 | Item: rendered element and its attributes | 4.5 | 4.4 | 0.557 |
| ns | 7137 |  | 257 | Group: render gate and heading/items markup | 4.6 | 2.6 | 0.546 |
| ns | 7329 |  | 192 | Input and Separator markup | 4.7 | 2.6 | 0.537 |
| walker |  | 7392 | 577 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.537 |
| ns | 7595 |  | 266 | List, Dialog, Empty and Loading markup | 4.8 | 2.6 | 0.528 |
| ns | 7795 |  | 200 | Roster of module-level helpers | 4.9 |  | 0.523 |
| walker |  | 7927 | 535 | Markdown::Section { file: README.md, section_index: 10, keeps_default_concavity: false } |  |  | 0.523 |
| ns | 8098 |  | 303 | command-score: the full scoring weight table | 5.1 |  | 0.512 |
| ns | 8194 |  | 96 | command-score: exported signature and alias handling | 5.2 | 5.1 | 0.514 |
| walker |  | 8253 | 326 | Markdown::Section { file: README.md, section_index: 11, keeps_default_concavity: false } |  |  | 0.514 |
| ns | 8254 |  | 60 | Test fixture pages listing | 6.1 |  | 0.522 |
| walker |  | 8272 | 19 | Fs::DirListing { dir: website/styles/cmdk } |  |  | 0.522 |
| walker |  | 8316 | 44 | Fs::DirListing { dir: website/public } |  |  | 0.522 |
| ns | 8432 |  | 178 | Spec names: basic behaviour | 6.2 |  | 0.517 |
| walker |  | 8558 | 242 | Json::Whole { file: tsconfig.json } |  |  | 0.517 |
| ns | 8577 |  | 145 | dialog.test.ts in full - the whole spec idiom | 6.3 |  | 0.512 |
| ns | 8740 |  | 163 | Spec names: item lifecycle and item-advanced | 6.4 |  | 0.508 |
| walker |  | 8802 | 244 | Markdown::Section { file: ARCHITECTURE.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.518 |
| ns | 8957 |  | 217 | Spec names: group, props matrix, numeric values | 6.5 |  | 0.513 |
| walker |  | 9012 | 210 | Markdown::Section { file: ARCHITECTURE.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.517 |
| walker |  | 9192 | 180 | Code::CodeKey { rung: Body, file: cmdk/src/index.tsx, decl: 9, sub: 0, line: 882 } |  |  | 0.522 |
| ns | 9217 |  | 260 | Spec names: keybinds (four describe blocks) | 6.6 |  | 0.515 |
| ns | 9473 |  | 256 | test/pages/dialog.tsx - a working usage page | 6.7 |  | 0.506 |
| walker |  | 9492 | 300 | Markdown::Section { file: ARCHITECTURE.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.506 |
| walker |  | 9678 | 186 | Code::CodeKey { rung: Body, file: cmdk/src/index.tsx, decl: 11, sub: 0, line: 909 } |  |  | 0.511 |
| walker |  | 9713 | 35 | Markdown::HeadingsOutline { file: website/README.md } |  |  | 0.511 |
| ns | 9749 |  | 276 | Playwright config (whole file) | 6.8 |  | 0.501 |
| ns | 9914 |  | 165 | CI workflow | 6.9 |  | 0.495 |
| ns | 9987 |  | 73 | Website source listings (demos, pages, drop-in stylesheets) | 7.1 |  | 0.504 |
| walker |  | 9999 | 286 | Code::CodeKey { rung: Body, file: cmdk/src/index.tsx, decl: 7, sub: 0, line: 787 } |  |  | 0.515 |

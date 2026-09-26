Score(3000)=0.824 I=0.934 C=0.726 ns_rows≤3K=16/47 grid(1000/1442/2080/3000/4327/6240/9000)=0.858/0.851/0.846/0.824/0.756/0.601/0.510

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 62 | 62 | Fs::DirListing { dir: . } |  |  | 1.000 |
| ns | 62 |  | 62 | Repo root listing | 1.1 |  | 1.000 |
| walker |  | 75 | 13 | Fs::DirListing { dir: cmdk } |  |  | 1.000 |
| walker |  | 85 | 10 | Fs::DirListing { dir: cmdk/src } |  |  | 1.000 |
| walker |  | 112 | 27 | Json::Identity { file: package.json } |  |  | 1.000 |
| ns | 174 |  | 112 | README one-line identity + install | 1.2 |  | 0.845 |
| ns | 208 |  | 34 | pnpm workspace membership | 1.3 |  | 0.757 |
| walker |  | 289 | 177 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.783 |
| walker |  | 331 | 42 | Json::Identity { file: cmdk/package.json } |  |  | 0.784 |
| walker |  | 336 | 5 | Fs::DirListing { dir: .husky } |  |  | 0.784 |
| ns | 343 |  | 135 | Listings of all three packages + library source dir | 1.4 |  | 0.500 |
| walker |  | 344 | 8 | Fs::DirListing { dir: .github } |  |  | 0.500 |
| walker |  | 348 | 4 | Fs::DirListing { dir: .github/workflows } |  |  | 0.500 |
| walker |  | 369 | 21 | Json::Runtime { file: package.json } |  |  | 0.500 |
| walker |  | 449 | 80 | Json::Scripts { file: cmdk/package.json } |  |  | 0.506 |
| ns | 532 |  | 189 | Published package identity + entry points | 1.5 |  | 0.489 |
| walker |  | 545 | 96 | Json::Dependencies { file: cmdk/package.json } |  |  | 0.489 |
| walker |  | 599 | 54 | Fs::DirListing { dir: website } |  |  | 0.594 |
| walker |  | 613 | 14 | Fs::DirListing { dir: website/components } |  |  | 0.594 |
| walker |  | 623 | 10 | Fs::DirListing { dir: website/components/code } |  |  | 0.594 |
| walker |  | 633 | 10 | Fs::DirListing { dir: website/components/icons } |  |  | 0.594 |
| walker |  | 650 | 17 | Fs::DirListing { dir: website/pages } |  |  | 0.594 |
| walker |  | 708 | 58 | Fs::DirListing { dir: test } |  |  | 0.796 |
| walker |  | 731 | 23 | Fs::DirListing { dir: website/components/cmdk } |  |  | 0.797 |
| ns | 764 |  | 232 | Root package.json: identity and every workspace script | 1.6 |  | 0.722 |
| walker |  | 876 | 145 | Json::Entry { file: cmdk/package.json } |  |  | 0.804 |
| walker |  | 910 | 34 | Plaintext::Whole { file: pnpm-workspace.yaml } |  |  | 0.845 |
| ns | 1043 |  | 279 | README section map (all H2 + H3 headings) | 1.7 |  | 0.741 |
| walker |  | 1198 | 288 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.880 |
| walker |  | 1224 | 26 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.910 |
| ns | 1225 |  | 182 | ARCHITECTURE: the core invariant + section map | 1.8 |  | 0.858 |
| ns | 1361 |  | 136 | README testing steps (verbatim) | 1.9 | 1.7 | 0.789 |
| walker |  | 1426 | 202 | Json::Scripts { file: package.json } |  |  | 0.851 |
| ns | 1473 |  | 112 | index.tsx imports + 'use client' | 2.1 |  | 0.822 |
| walker |  | 1606 | 180 | Code::CodeKey { rung: Names, file: cmdk/src/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.823 |
| walker |  | 1630 | 24 | Code::CodeKey { rung: Decl, file: cmdk/src/index.tsx, decl: 2, sub: 0, line: 149 } |  |  | 0.823 |
| walker |  | 1689 | 59 | Code::CodeKey { rung: Decl, file: cmdk/src/index.tsx, decl: 1, sub: 0, line: 137 } |  |  | 0.824 |
| ns | 1690 |  | 217 | Public export surface (Command.* object + named exports) | 2.2 |  | 0.752 |
| walker |  | 1973 | 284 | Code::CodeKey { rung: Names, file: cmdk/src/index.tsx, decl: 0, sub: 1, line: 0 } |  |  | 0.803 |
| walker |  | 2032 | 59 | Code::CodeKey { rung: Decl, file: cmdk/src/index.tsx, decl: 13, sub: 0, line: 930 } |  |  | 0.846 |
| walker |  | 2047 | 15 | Code::CodeKey { rung: Doc, file: cmdk/src/index.tsx, decl: 14, sub: 0, line: 1004 } |  |  | 0.846 |
| walker |  | 2073 | 26 | Code::CodeKey { rung: Doc, file: cmdk/src/index.tsx, decl: 10, sub: 0, line: 882 } |  |  | 0.846 |
| walker |  | 2100 | 27 | Code::CodeKey { rung: Doc, file: cmdk/src/index.tsx, decl: 11, sub: 0, line: 899 } |  |  | 0.846 |
| ns | 2118 |  | 428 | README FAQ (all twelve entries) | 2.3 | 1.7 | 0.784 |
| walker |  | 2130 | 30 | Code::CodeKey { rung: Doc, file: cmdk/src/index.tsx, decl: 12, sub: 0, line: 909 } |  |  | 0.785 |
| walker |  | 2167 | 37 | Code::CodeKey { rung: Doc, file: cmdk/src/index.tsx, decl: 6, sub: 0, line: 729 } |  |  | 0.786 |
| walker |  | 2205 | 38 | Code::CodeKey { rung: Doc, file: cmdk/src/index.tsx, decl: 8, sub: 0, line: 787 } |  |  | 0.787 |
| walker |  | 2253 | 48 | Code::CodeKey { rung: Doc, file: cmdk/src/index.tsx, decl: 7, sub: 0, line: 774 } |  |  | 0.788 |
| ns | 2297 |  | 179 | Component declaration roster (all nine forwardRef components) | 2.4 |  | 0.794 |
| walker |  | 2305 | 52 | Code::CodeKey { rung: Doc, file: cmdk/src/index.tsx, decl: 9, sub: 0, line: 833 } |  |  | 0.796 |
| walker |  | 2371 | 66 | Code::CodeKey { rung: Doc, file: cmdk/src/index.tsx, decl: 5, sub: 0, line: 664 } |  |  | 0.800 |
| ns | 2406 |  | 109 | Props type roster (all twelve type aliases) | 2.5 |  | 0.773 |
| walker |  | 2431 | 60 | Fs::DirListing { dir: test/pages } |  |  | 0.774 |
| walker |  | 2572 | 141 | Markdown::Section { file: README.md, section_index: 15, keeps_default_concavity: false } |  |  | 0.834 |
| walker |  | 2723 | 151 | Markdown::Section { file: README.md, section_index: 14, keeps_default_concavity: false } |  |  | 0.834 |
| ns | 2730 |  | 324 | Per-component JSDoc blocks | 2.6 | 2.4 | 0.845 |
| walker |  | 2903 | 180 | Markdown::Section { file: README.md, section_index: 12, keeps_default_concavity: false } |  |  | 0.853 |
| ns | 2984 |  | 254 | CommandProps: label, shouldFilter, filter, defaultValue | 2.7 | 2.5 | 0.811 |
| walker |  | 3151 | 248 | Markdown::Section { file: README.md, section_index: 13, keeps_default_concavity: false } |  |  | 0.859 |
| ns | 3203 |  | 219 | CommandProps: value, onValueChange, loop, disablePointerSelection, vimBindings | 2.8 | 2.7 | 0.823 |
| ns | 3430 |  | 227 | ItemProps (full, with JSDoc) | 2.9 | 2.5 | 0.797 |
| walker |  | 3447 | 296 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.797 |
| ns | 3588 |  | 158 | GroupProps, SeparatorProps, EmptyProps | 2.10 | 2.5 | 0.776 |
| walker |  | 3691 | 244 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.776 |
| walker |  | 3704 | 13 | Fs::DirListing { dir: website/styles } |  |  | 0.776 |
| ns | 3715 |  | 127 | InputProps + CommandFilter signature | 2.11 | 2.5 | 0.761 |
| walker |  | 3750 | 46 | Code::CodeKey { rung: Body, file: cmdk/src/index.tsx, decl: 14, sub: 0, line: 1004 } |  |  | 0.761 |
| walker |  | 3776 | 26 | Code::CodeKey { rung: Names, file: cmdk/src/command-score.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.761 |
| ns | 4048 |  | 333 | ARCHITECTURE: the three rejected APIs and why selection tracks value | 3.1 | 1.8 | 0.756 |
| walker |  | 4300 | 524 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.756 |
| ns | 4406 |  | 358 | Internal types: Context, State, Store, Group | 3.2 |  | 0.723 |
| ns | 4627 |  | 221 | DOM selector constants + the three React contexts | 3.3 |  | 0.707 |
| walker |  | 4726 | 426 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.707 |
| ns | 4841 |  | 214 | Roster of Command's internal functions | 3.4 |  | 0.693 |
| ns | 5054 |  | 213 | Store: subscribe/snapshot and the 'search' setState branch | 3.5 |  | 0.673 |
| walker |  | 5200 | 474 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.673 |
| ns | 5268 |  | 214 | Root keydown handler: IME guard, vim down bindings, ArrowDown | 3.6 | 3.4 | 0.652 |
| walker |  | 5349 | 149 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.652 |
| walker |  | 5415 | 66 | Code::CodeKey { rung: Body, file: cmdk/src/index.tsx, decl: 11, sub: 0, line: 899 } |  |  | 0.652 |
| ns | 5646 |  | 378 | score() and sort(): per-group maximum score and the ordering rules | 4.1 | 3.4 | 0.622 |
| ns | 5995 |  | 349 | sort(): the DOM re-append loop | 4.2 | 4.1 | 0.601 |
| walker |  | 6056 | 641 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.601 |
| ns | 6400 |  | 405 | filterItems(): scoring every item and deriving visible groups | 4.3 | 3.4 | 0.576 |
| walker |  | 6633 | 577 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.576 |
| ns | 6670 |  | 270 | Item: value inference, render gate, select handlers | 4.4 | 2.6 | 0.563 |
| ns | 6848 |  | 178 | Item: rendered element and its attributes | 4.5 | 4.4 | 0.554 |
| ns | 7105 |  | 257 | Group: render gate and heading/items markup | 4.6 | 2.6 | 0.542 |
| walker |  | 7168 | 535 | Markdown::Section { file: README.md, section_index: 10, keeps_default_concavity: false } |  |  | 0.542 |
| ns | 7297 |  | 192 | Input and Separator markup | 4.7 | 2.6 | 0.533 |
| walker |  | 7494 | 326 | Markdown::Section { file: README.md, section_index: 11, keeps_default_concavity: false } |  |  | 0.533 |
| walker |  | 7513 | 19 | Fs::DirListing { dir: website/styles/cmdk } |  |  | 0.534 |
| ns | 7563 |  | 266 | List, Dialog, Empty and Loading markup | 4.8 | 2.6 | 0.525 |
| walker |  | 7755 | 242 | Json::Whole { file: tsconfig.json } |  |  | 0.525 |
| ns | 7763 |  | 200 | Roster of module-level helpers | 4.9 |  | 0.520 |
| walker |  | 7859 | 104 | Code::CodeKey { rung: Body, file: cmdk/src/index.tsx, decl: 7, sub: 0, line: 774 } |  |  | 0.520 |
| walker |  | 7903 | 44 | Fs::DirListing { dir: website/public } |  |  | 0.520 |
| walker |  | 8031 | 128 | Code::CodeKey { rung: Body, file: cmdk/src/command-score.ts, decl: 1, sub: 0, line: 155 } |  |  | 0.520 |
| ns | 8066 |  | 303 | command-score: the full scoring weight table | 5.1 |  | 0.509 |
| ns | 8162 |  | 96 | command-score: exported signature and alias handling | 5.2 | 5.1 | 0.511 |
| walker |  | 8211 | 180 | Code::CodeKey { rung: Body, file: cmdk/src/index.tsx, decl: 10, sub: 0, line: 882 } |  |  | 0.516 |
| ns | 8222 |  | 60 | Test fixture pages listing | 6.1 |  | 0.524 |
| walker |  | 8397 | 186 | Code::CodeKey { rung: Body, file: cmdk/src/index.tsx, decl: 12, sub: 0, line: 909 } |  |  | 0.530 |
| ns | 8400 |  | 178 | Spec names: basic behaviour | 6.2 |  | 0.525 |
| walker |  | 8419 | 22 | Code::CodeKey { rung: Names, file: test/pages/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.525 |
| walker |  | 8491 | 72 | Code::CodeKey { rung: Names, file: website/components/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.525 |
| walker |  | 8505 | 14 | Code::CodeKey { rung: Names, file: website/pages/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.525 |
| ns | 8545 |  | 145 | dialog.test.ts in full - the whole spec idiom | 6.3 |  | 0.520 |
| walker |  | 8673 | 168 | Code::CodeKey { rung: Names, file: website/components/icons/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.520 |
| walker |  | 8705 | 32 | Json::Identity { file: test/package.json } |  |  | 0.520 |
| ns | 8708 |  | 163 | Spec names: item lifecycle and item-advanced | 6.4 |  | 0.516 |
| walker |  | 8731 | 26 | Json::Scripts { file: test/package.json } |  |  | 0.516 |
| ns | 8925 |  | 217 | Spec names: group, props matrix, numeric values | 6.5 |  | 0.510 |
| walker |  | 9148 | 417 | Code::CodeKey { rung: Body, file: cmdk/src/index.tsx, decl: 8, sub: 0, line: 787 } |  |  | 0.530 |
| walker |  | 9168 | 20 | Code::CodeKey { rung: Names, file: website/components/code/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.530 |
| ns | 9185 |  | 260 | Spec names: keybinds (four describe blocks) | 6.6 |  | 0.524 |
| walker |  | 9194 | 26 | Plaintext::DeclSurface { file: test/style.css } |  |  | 0.524 |
| walker |  | 9197 | 3 | Plaintext::Whole { file: test/style.css } |  |  | 0.524 |
| walker |  | 9240 | 43 | Json::Identity { file: website/package.json } |  |  | 0.524 |
| walker |  | 9319 | 79 | Json::Scripts { file: website/package.json } |  |  | 0.524 |
| walker |  | 9334 | 15 | Plaintext::DeclSurface { file: website/public/robots.txt } |  |  | 0.524 |
| ns | 9441 |  | 256 | test/pages/dialog.tsx - a working usage page | 6.7 |  | 0.514 |
| walker |  | 9492 | 158 | Json::Dependencies { file: test/package.json } |  |  | 0.514 |
| walker |  | 9525 | 33 | Plaintext::DeclSurface { file: .husky/pre-commit } |  |  | 0.514 |
| walker |  | 9531 | 6 | Plaintext::Whole { file: .husky/pre-commit } |  |  | 0.514 |
| ns | 9717 |  | 276 | Playwright config (whole file) | 6.8 |  | 0.504 |
| walker |  | 9730 | 199 | Code::CodeKey { rung: Body, file: cmdk/src/index.tsx, decl: 6, sub: 0, line: 729 } |  |  | 0.505 |
| ns | 9882 |  | 165 | CI workflow | 6.9 |  | 0.498 |
| walker |  | 9927 | 197 | Json::Dependencies { file: website/package.json } |  |  | 0.498 |
| walker |  | 9951 | 24 | Code::CodeKey { rung: Body, file: cmdk/src/index.tsx, decl: 6, sub: 1, line: 729 } |  |  | 0.498 |
| ns | 9955 |  | 73 | Website source listings (demos, pages, drop-in stylesheets) | 7.1 |  | 0.507 |

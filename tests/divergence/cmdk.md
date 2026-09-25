Score(3000)=0.498 I=0.780 C=0.318 ns_rows≤3K=16/47 grid(1000/1442/2080/3000/4327/6240/9000)=0.394/0.303/0.277/0.498/0.691/0.735/0.623

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 62 | 62 | listing of '.' |  |  | 1.000 |
| ns | 62 |  | 62 | Repo root listing | 1.1 |  | 1.000 |
| walker |  | 75 | 13 | listing of 'cmdk' |  |  | 1.000 |
| walker |  | 102 | 27 | package identity in package.json |  |  | 1.000 |
| walker |  | 112 | 10 | listing of 'cmdk/src' |  |  | 1.000 |
| walker |  | 154 | 42 | package identity in cmdk/package.json |  |  | 1.000 |
| ns | 174 |  | 112 | README one-line identity + install | 1.2 |  | 0.846 |
| walker |  | 179 | 25 | ts names playwright.config.ts |  |  | 0.846 |
| walker |  | 184 | 5 | listing of '.husky' |  |  | 0.846 |
| walker |  | 197 | 13 | ts names cmdk/tsup.config.ts |  |  | 0.846 |
| walker |  | 205 | 8 | listing of '.github' |  |  | 0.846 |
| ns | 208 |  | 34 | pnpm workspace membership | 1.3 |  | 0.758 |
| walker |  | 209 | 4 | listing of '.github/workflows' |  |  | 0.758 |
| walker |  | 230 | 21 | package runtime metadata in package.json |  |  | 0.758 |
| walker |  | 322 | 92 | README headline in README.md |  |  | 0.784 |
| ns | 343 |  | 135 | Listings of all three packages + library source dir | 1.4 |  | 0.500 |
| ns | 532 |  | 189 | Published package identity + entry points | 1.5 |  | 0.426 |
| walker |  | 581 | 259 | ts names cmdk/src/index.tsx |  |  | 0.430 |
| walker |  | 603 | 22 | ts decl cmdk/src/index.tsx:149 |  |  | 0.430 |
| walker |  | 638 | 35 | ts decl cmdk/src/index.tsx:24 |  |  | 0.430 |
| walker |  | 690 | 52 | ts decl cmdk/src/index.tsx:37 |  |  | 0.430 |
| walker |  | 747 | 57 | ts decl cmdk/src/index.tsx:137 |  |  | 0.431 |
| ns | 764 |  | 232 | Root package.json: identity and every workspace script | 1.6 |  | 0.392 |
| walker |  | 821 | 74 | ts decl cmdk/src/index.tsx:143 |  |  | 0.393 |
| walker |  | 896 | 75 | ts decl cmdk/src/index.tsx:13 |  |  | 0.393 |
| walker |  | 981 | 85 | ts decl cmdk/src/index.tsx:69 |  |  | 0.394 |
| ns | 1043 |  | 279 | README section map (all H2 + H3 headings) | 1.7 |  | 0.346 |
| walker |  | 1074 | 93 | ts decl cmdk/src/index.tsx:28 |  |  | 0.346 |
| ns | 1225 |  | 182 | ARCHITECTURE: the core invariant + section map | 1.8 |  | 0.326 |
| walker |  | 1282 | 208 | ts names cmdk/src/index.tsx #1 |  |  | 0.328 |
| ns | 1361 |  | 136 | README testing steps (verbatim) | 1.9 | 1.7 | 0.301 |
| walker |  | 1393 | 111 | ts decl cmdk/src/index.tsx:60 |  |  | 0.303 |
| walker |  | 1450 | 57 | headings outline in ARCHITECTURE.md |  |  | 0.310 |
| ns | 1473 |  | 112 | index.tsx imports + 'use client' | 2.1 |  | 0.299 |
| walker |  | 1476 | 26 | ts names cmdk/src/command-score.ts |  |  | 0.299 |
| walker |  | 1641 | 165 | ts decl cmdk/src/index.tsx:123 |  |  | 0.302 |
| ns | 1690 |  | 217 | Public export surface (Command.* object + named exports) | 2.2 |  | 0.275 |
| walker |  | 1737 | 96 | package runtime dependencies in cmdk/package.json |  |  | 0.275 |
| walker |  | 1942 | 205 | ts names cmdk/src/index.tsx #2 |  |  | 0.277 |
| walker |  | 2015 | 73 | ts decl cmdk/tsup.config.ts:3 |  |  | 0.277 |
| ns | 2118 |  | 428 | README FAQ (all twelve entries) | 2.3 | 1.7 | 0.257 |
| walker |  | 2240 | 225 | ts decl cmdk/src/index.tsx:44 |  |  | 0.258 |
| ns | 2297 |  | 179 | Component declaration roster (all nine forwardRef components) | 2.4 |  | 0.294 |
| walker |  | 2320 | 80 | package scripts in cmdk/package.json |  |  | 0.326 |
| walker |  | 2374 | 54 | listing of 'website' |  |  | 0.380 |
| walker |  | 2388 | 14 | listing of 'website/components' |  |  | 0.380 |
| ns | 2406 |  | 109 | Props type roster (all twelve type aliases) | 2.5 |  | 0.418 |
| walker |  | 2446 | 58 | listing of 'test' |  |  | 0.516 |
| walker |  | 2648 | 202 | package scripts in package.json |  |  | 0.566 |
| ns | 2730 |  | 324 | Per-component JSDoc blocks | 2.6 | 2.4 | 0.524 |
| ns | 2984 |  | 254 | CommandProps: label, shouldFilter, filter, defaultValue | 2.7 | 2.5 | 0.498 |
| walker |  | 3030 | 382 | ts names cmdk/src/index.tsx #3 |  |  | 0.543 |
| walker |  | 3089 | 59 | ts decl cmdk/src/index.tsx:930 |  |  | 0.577 |
| walker |  | 3139 | 50 | ts decl cmdk/src/index.tsx:1071 |  |  | 0.577 |
| ns | 3203 |  | 219 | CommandProps: value, onValueChange, loop, disablePointerSelection, vimBindings | 2.8 | 2.7 | 0.553 |
| walker |  | 3204 | 65 | ts decl cmdk/src/index.tsx:1010 |  |  | 0.553 |
| walker |  | 3326 | 122 | ts decl cmdk/src/index.tsx:1081 |  |  | 0.553 |
| ns | 3430 |  | 227 | ItemProps (full, with JSDoc) | 2.9 | 2.5 | 0.574 |
| walker |  | 3582 | 256 | ts decl playwright.config.ts:3 |  |  | 0.576 |
| ns | 3588 |  | 158 | GroupProps, SeparatorProps, EmptyProps | 2.10 | 2.5 | 0.591 |
| walker |  | 3599 | 17 | listing of 'website/pages' |  |  | 0.591 |
| ns | 3715 |  | 127 | InputProps + CommandFilter signature | 2.11 | 2.5 | 0.602 |
| walker |  | 3744 | 145 | package entrypoints in cmdk/package.json |  |  | 0.632 |
| walker |  | 3996 | 252 | ts decl cmdk/src/index.tsx:80 |  |  | 0.677 |
| walker |  | 4030 | 34 | plaintext config pnpm-workspace.yaml |  |  | 0.696 |
| walker |  | 4053 | 23 | listing of 'website/components/cmdk' |  |  | 0.696 |
| ns | 4080 |  | 365 | ARCHITECTURE: the three rejected APIs and why selection tracks value | 3.1 | 1.8 | 0.691 |
| walker |  | 4341 | 288 | headings outline in README.md |  |  | 0.750 |
| walker |  | 4355 | 14 | README.md section #28 |  |  | 0.750 |
| walker |  | 4381 | 26 | README.md section #1 |  |  | 0.766 |
| walker |  | 4425 | 44 | README.md section #3 |  |  | 0.766 |
| ns | 4438 |  | 358 | Internal types: Context, State, Store, Group | 3.2 |  | 0.778 |
| walker |  | 4457 | 32 | ARCHITECTURE.md section #4 |  |  | 0.782 |
| walker |  | 4476 | 19 | README.md section #24 |  |  | 0.782 |
| ns | 4659 |  | 221 | DOM selector constants + the three React contexts | 3.3 |  | 0.787 |
| walker |  | 4693 | 217 | ts decl cmdk/src/index.tsx:80 #1 |  |  | 0.825 |
| walker |  | 4719 | 26 | README.md section #19 |  |  | 0.825 |
| walker |  | 4745 | 26 | README.md section #22 |  |  | 0.825 |
| walker |  | 4770 | 25 | README.md section #23 |  |  | 0.825 |
| walker |  | 4780 | 10 | ts names .prettierrc.js |  |  | 0.825 |
| walker |  | 4815 | 35 | README.md section #21 |  |  | 0.825 |
| ns | 4873 |  | 214 | Roster of Command's internal functions | 3.4 |  | 0.808 |
| walker |  | 4875 | 60 | listing of 'test/pages' |  |  | 0.810 |
| walker |  | 4897 | 22 | ts names test/pages/index.tsx |  |  | 0.810 |
| walker |  | 4988 | 91 | package dev/peer dependencies in cmdk/package.json |  |  | 0.812 |
| walker |  | 5056 | 68 | ARCHITECTURE.md section #3 |  |  | 0.812 |
| ns | 5086 |  | 213 | Store: subscribe/snapshot and the 'search' setState branch | 3.5 |  | 0.789 |
| walker |  | 5108 | 52 | README.md section #4 |  |  | 0.789 |
| walker |  | 5160 | 52 | README.md section #5 |  |  | 0.789 |
| walker |  | 5200 | 40 | README.md section #34 |  |  | 0.789 |
| walker |  | 5215 | 15 | ts doc cmdk/src/index.tsx:1004 |  |  | 0.789 |
| ns | 5300 |  | 214 | Root keydown handler: IME guard, vim down bindings, ArrowDown | 3.6 | 3.4 | 0.763 |
| walker |  | 5356 | 141 | README.md section #37 |  |  | 0.797 |
| walker |  | 5507 | 151 | README.md section #36 |  |  | 0.797 |
| walker |  | 5570 | 63 | README.md section #10 |  |  | 0.797 |
| walker |  | 5633 | 63 | README.md section #17 |  |  | 0.797 |
| ns | 5678 |  | 378 | score() and sort(): per-group maximum score and the ordering rules | 4.1 | 3.4 | 0.761 |
| walker |  | 5875 | 242 | json config tsconfig.json |  |  | 0.761 |
| walker |  | 5941 | 66 | README.md section #26 |  |  | 0.761 |
| walker |  | 6012 | 71 | README.md section #20 |  |  | 0.761 |
| ns | 6027 |  | 349 | sort(): the DOM re-append loop | 4.2 | 4.1 | 0.735 |
| walker |  | 6085 | 73 | README.md section #27 |  |  | 0.735 |
| walker |  | 6162 | 77 | README.md section #18 |  |  | 0.735 |
| ns | 6432 |  | 405 | filterItems(): scoring every item and deriving visible groups | 4.3 | 3.4 | 0.705 |
| walker |  | 6601 | 439 | ARCHITECTURE.md section #0 |  |  | 0.708 |
| walker |  | 6683 | 82 | README.md section #15 |  |  | 0.708 |
| walker |  | 6696 | 13 | listing of 'website/styles' |  |  | 0.708 |
| ns | 6702 |  | 270 | Item: value inference, render gate, select handlers | 4.4 | 2.6 | 0.692 |
| walker |  | 6722 | 26 | ts doc cmdk/src/index.tsx:882 |  |  | 0.692 |
| walker |  | 6850 | 128 | ts body cmdk/src/command-score.ts:155 |  |  | 0.692 |
| ns | 6880 |  | 178 | Item: rendered element and its attributes | 4.5 | 4.4 | 0.680 |
| walker |  | 6922 | 72 | ts names website/components/index.ts |  |  | 0.680 |
| ns | 7137 |  | 257 | Group: render gate and heading/items markup | 4.6 | 2.6 | 0.666 |
| walker |  | 7261 | 339 | YAML config at .github/workflows/test.yml |  |  | 0.668 |
| ns | 7329 |  | 192 | Input and Separator markup | 4.7 | 2.6 | 0.657 |
| walker |  | 7353 | 92 | README.md section #32 |  |  | 0.657 |
| walker |  | 7380 | 27 | ts doc cmdk/src/index.tsx:899 |  |  | 0.658 |
| walker |  | 7484 | 104 | README.md section #7 |  |  | 0.658 |
| walker |  | 7589 | 105 | README.md section #14 |  |  | 0.658 |
| ns | 7595 |  | 266 | List, Dialog, Empty and Loading markup | 4.8 | 2.6 | 0.647 |
| walker |  | 7678 | 89 | README.md section #25 |  |  | 0.647 |
| walker |  | 7772 | 94 | README.md section #13 |  |  | 0.647 |
| ns | 7795 |  | 200 | Roster of module-level helpers | 4.9 |  | 0.651 |
| walker |  | 7890 | 118 | README.md section #16 |  |  | 0.651 |
| walker |  | 7900 | 10 | listing of 'website/components/code' |  |  | 0.651 |
| walker |  | 7910 | 10 | listing of 'website/components/icons' |  |  | 0.651 |
| walker |  | 7940 | 30 | ts doc cmdk/src/index.tsx:909 |  |  | 0.652 |
| walker |  | 7963 | 23 | ts names website/next.config.js |  |  | 0.652 |
| walker |  | 8089 | 126 | README.md section #6 |  |  | 0.652 |
| ns | 8098 |  | 303 | command-score: the full scoring weight table | 5.1 |  | 0.639 |
| ns | 8194 |  | 96 | command-score: exported signature and alias handling | 5.2 | 5.1 | 0.639 |
| walker |  | 8220 | 131 | README.md section #9 |  |  | 0.639 |
| ns | 8254 |  | 60 | Test fixture pages listing | 6.1 |  | 0.644 |
| walker |  | 8275 | 55 | ts decl .prettierrc.js:1 |  |  | 0.644 |
| walker |  | 8410 | 135 | README.md section #8 |  |  | 0.644 |
| ns | 8432 |  | 178 | Spec names: basic behaviour | 6.2 |  | 0.639 |
| walker |  | 8436 | 26 | ts decl website/next.config.js:2 |  |  | 0.639 |
| walker |  | 8473 | 37 | ts doc cmdk/src/index.tsx:729 |  |  | 0.641 |
| ns | 8577 |  | 145 | dialog.test.ts in full - the whole spec idiom | 6.3 |  | 0.635 |
| walker |  | 8629 | 156 | README.md section #12 |  |  | 0.635 |
| ns | 8740 |  | 163 | Spec names: item lifecycle and item-advanced | 6.4 |  | 0.630 |
| walker |  | 8785 | 156 | README.md section #11 |  |  | 0.630 |
| walker |  | 8801 | 16 | ts names website/pages/_document.tsx |  |  | 0.630 |
| walker |  | 8814 | 13 | ts decl website/pages/_document.tsx:5 |  |  | 0.630 |
| ns | 8957 |  | 217 | Spec names: group, props matrix, numeric values | 6.5 |  | 0.623 |
| ns | 9217 |  | 260 | Spec names: keybinds (four describe blocks) | 6.6 |  | 0.616 |
| walker |  | 9247 | 433 | README.md section #35 |  |  | 0.644 |
| walker |  | 9285 | 38 | ts doc cmdk/src/index.tsx:787 |  |  | 0.647 |
| ns | 9473 |  | 256 | test/pages/dialog.tsx - a working usage page | 6.7 |  | 0.635 |
| walker |  | 9487 | 202 | ts names website/pages/index.tsx |  |  | 0.635 |
| walker |  | 9509 | 22 | ts decl website/pages/index.tsx:20 |  |  | 0.635 |
| ns | 9749 |  | 276 | Playwright config (whole file) | 6.8 |  | 0.641 |
| ns | 9914 |  | 165 | CI workflow | 6.9 |  | 0.648 |
| ns | 9987 |  | 73 | Website source listings (demos, pages, drop-in stylesheets) | 7.1 |  | 0.648 |

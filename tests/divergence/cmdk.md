Score(3000)=0.723 I=0.903 C=0.579 ns_rows≤3K=16/47 grid(1000/1442/2080/3000/4327/6240/9000)=0.814/0.854/0.801/0.723/0.802/0.791/0.661

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 62 | 62 | listing of '.' |  |  | 1.000 |
| ns | 62 |  | 62 | Repo root listing | 1.1 |  | 1.000 |
| walker |  | 75 | 13 | listing of 'cmdk' |  |  | 1.000 |
| walker |  | 102 | 27 | package identity in package.json |  |  | 1.000 |
| walker |  | 112 | 10 | listing of 'cmdk/src' |  |  | 1.000 |
| walker |  | 154 | 42 | package identity in cmdk/package.json |  |  | 1.000 |
| walker |  | 159 | 5 | listing of '.husky' |  |  | 1.000 |
| walker |  | 167 | 8 | listing of '.github' |  |  | 1.000 |
| walker |  | 171 | 4 | listing of '.github/workflows' |  |  | 1.000 |
| ns | 174 |  | 112 | README one-line identity + install | 1.2 |  | 0.846 |
| walker |  | 192 | 21 | package runtime metadata in package.json |  |  | 0.846 |
| ns | 208 |  | 34 | pnpm workspace membership | 1.3 |  | 0.758 |
| walker |  | 284 | 92 | README headline in README.md |  |  | 0.784 |
| walker |  | 341 | 57 | headings outline in ARCHITECTURE.md |  |  | 0.785 |
| ns | 343 |  | 135 | Listings of all three packages + library source dir | 1.4 |  | 0.500 |
| walker |  | 351 | 10 | export names surface in playwright.config.ts |  |  | 0.500 |
| walker |  | 351 | 0 | export at playwright.config.ts:27 |  |  | 0.500 |
| walker |  | 447 | 96 | package runtime dependencies in cmdk/package.json |  |  | 0.500 |
| walker |  | 527 | 80 | package scripts in cmdk/package.json |  |  | 0.506 |
| ns | 532 |  | 189 | Published package identity + entry points | 1.5 |  | 0.490 |
| walker |  | 581 | 54 | listing of 'website' |  |  | 0.594 |
| walker |  | 595 | 14 | listing of 'website/components' |  |  | 0.595 |
| walker |  | 653 | 58 | listing of 'test' |  |  | 0.797 |
| ns | 764 |  | 232 | Root package.json: identity and every workspace script | 1.6 |  | 0.722 |
| walker |  | 855 | 202 | package scripts in package.json |  |  | 0.814 |
| walker |  | 872 | 17 | listing of 'website/pages' |  |  | 0.814 |
| walker |  | 885 | 13 | export names surface in cmdk/tsup.config.ts |  |  | 0.814 |
| walker |  | 1030 | 145 | package entrypoints in cmdk/package.json |  |  | 0.892 |
| ns | 1043 |  | 279 | README section map (all H2 + H3 headings) | 1.7 |  | 0.782 |
| walker |  | 1064 | 34 | plaintext config pnpm-workspace.yaml |  |  | 0.817 |
| walker |  | 1087 | 23 | listing of 'website/components/cmdk' |  |  | 0.818 |
| ns | 1225 |  | 182 | ARCHITECTURE: the core invariant + section map | 1.8 |  | 0.775 |
| ns | 1361 |  | 136 | README testing steps (verbatim) | 1.9 | 1.7 | 0.712 |
| walker |  | 1375 | 288 | headings outline in README.md |  |  | 0.828 |
| walker |  | 1389 | 14 | README.md section #28 |  |  | 0.828 |
| walker |  | 1415 | 26 | README.md section #1 |  |  | 0.854 |
| walker |  | 1459 | 44 | README.md section #3 |  |  | 0.854 |
| ns | 1473 |  | 112 | index.tsx imports + 'use client' | 2.1 |  | 0.826 |
| ns | 1690 |  | 217 | Public export surface (Command.* object + named exports) | 2.2 |  | 0.754 |
| walker |  | 1870 | 411 | export names surface in cmdk/src/index.tsx |  |  | 0.798 |
| walker |  | 1870 | 0 | export at cmdk/src/index.tsx:169 |  |  | 0.798 |
| walker |  | 1870 | 0 | export at cmdk/src/index.tsx:664 |  |  | 0.798 |
| walker |  | 1870 | 0 | export at cmdk/src/index.tsx:729 |  |  | 0.798 |
| walker |  | 1870 | 0 | export at cmdk/src/index.tsx:774 |  |  | 0.798 |
| walker |  | 1870 | 0 | export at cmdk/src/index.tsx:787 |  |  | 0.798 |
| walker |  | 1870 | 0 | export at cmdk/src/index.tsx:833 |  |  | 0.798 |
| walker |  | 1870 | 0 | export at cmdk/src/index.tsx:882 |  |  | 0.798 |
| walker |  | 1870 | 0 | export at cmdk/src/index.tsx:899 |  |  | 0.798 |
| walker |  | 1870 | 0 | export at cmdk/src/index.tsx:909 |  |  | 0.798 |
| walker |  | 1870 | 0 | export at cmdk/src/index.tsx:1004 |  |  | 0.798 |
| walker |  | 1894 | 24 | export at cmdk/src/index.tsx:149 |  |  | 0.798 |
| walker |  | 1909 | 15 | export doc at cmdk/src/index.tsx:1004 |  |  | 0.798 |
| walker |  | 1935 | 26 | export doc at cmdk/src/index.tsx:882 |  |  | 0.798 |
| walker |  | 1962 | 27 | export doc at cmdk/src/index.tsx:899 |  |  | 0.799 |
| walker |  | 1992 | 30 | export doc at cmdk/src/index.tsx:909 |  |  | 0.799 |
| walker |  | 2029 | 37 | export doc at cmdk/src/index.tsx:729 |  |  | 0.800 |
| walker |  | 2067 | 38 | export doc at cmdk/src/index.tsx:787 |  |  | 0.801 |
| ns | 2118 |  | 428 | README FAQ (all twelve entries) | 2.3 | 1.7 | 0.742 |
| walker |  | 2177 | 110 | imports in cmdk/src/index.tsx |  |  | 0.772 |
| ns | 2297 |  | 179 | Component declaration roster (all nine forwardRef components) | 2.4 |  | 0.779 |
| ns | 2406 |  | 109 | Props type roster (all twelve type aliases) | 2.5 |  | 0.753 |
| ns | 2730 |  | 324 | Per-component JSDoc blocks | 2.6 | 2.4 | 0.716 |
| walker |  | 2816 | 639 | module item names surface in cmdk/src/index.tsx |  |  | 0.760 |
| walker |  | 2816 | 0 | module item at cmdk/src/index.tsx:10 |  |  | 0.760 |
| walker |  | 2816 | 0 | module item at cmdk/src/index.tsx:11 |  |  | 0.760 |
| walker |  | 2816 | 0 | module item at cmdk/src/index.tsx:23 |  |  | 0.760 |
| walker |  | 2816 | 0 | module item at cmdk/src/index.tsx:79 |  |  | 0.760 |
| walker |  | 2816 | 0 | module item at cmdk/src/index.tsx:154 |  |  | 0.760 |
| walker |  | 2816 | 0 | module item at cmdk/src/index.tsx:155 |  |  | 0.760 |
| walker |  | 2816 | 0 | module item at cmdk/src/index.tsx:156 |  |  | 0.760 |
| walker |  | 2816 | 0 | module item at cmdk/src/index.tsx:157 |  |  | 0.760 |
| walker |  | 2816 | 0 | module item at cmdk/src/index.tsx:158 |  |  | 0.760 |
| walker |  | 2816 | 0 | module item at cmdk/src/index.tsx:159 |  |  | 0.760 |
| walker |  | 2816 | 0 | module item at cmdk/src/index.tsx:160 |  |  | 0.760 |
| walker |  | 2816 | 0 | module item at cmdk/src/index.tsx:161 |  |  | 0.760 |
| walker |  | 2816 | 0 | module item at cmdk/src/index.tsx:163 |  |  | 0.760 |
| walker |  | 2816 | 0 | module item at cmdk/src/index.tsx:164 |  |  | 0.760 |
| walker |  | 2816 | 0 | module item at cmdk/src/index.tsx:165 |  |  | 0.760 |
| walker |  | 2816 | 0 | module item at cmdk/src/index.tsx:166 |  |  | 0.760 |
| walker |  | 2816 | 0 | module item at cmdk/src/index.tsx:167 |  |  | 0.760 |
| walker |  | 2816 | 0 | module item at cmdk/src/index.tsx:963 |  |  | 0.760 |
| walker |  | 2816 | 0 | module item at cmdk/src/index.tsx:972 |  |  | 0.760 |
| walker |  | 2816 | 0 | module item at cmdk/src/index.tsx:981 |  |  | 0.760 |
| walker |  | 2816 | 0 | module item at cmdk/src/index.tsx:991 |  |  | 0.760 |
| walker |  | 2816 | 0 | module item at cmdk/src/index.tsx:993 |  |  | 0.760 |
| walker |  | 2816 | 0 | module item at cmdk/src/index.tsx:1046 |  |  | 0.760 |
| walker |  | 2816 | 0 | module item at cmdk/src/index.tsx:1061 |  |  | 0.760 |
| walker |  | 2851 | 35 | module item at cmdk/src/index.tsx:24 |  |  | 0.761 |
| walker |  | 2903 | 52 | module item at cmdk/src/index.tsx:37 |  |  | 0.761 |
| walker |  | 2953 | 50 | module item at cmdk/src/index.tsx:1071 |  |  | 0.761 |
| ns | 2984 |  | 254 | CommandProps: label, shouldFilter, filter, defaultValue | 2.7 | 2.5 | 0.723 |
| walker |  | 3010 | 57 | module item at cmdk/src/index.tsx:137 |  |  | 0.724 |
| walker |  | 3075 | 65 | module item at cmdk/src/index.tsx:1010 |  |  | 0.724 |
| walker |  | 3149 | 74 | module item at cmdk/src/index.tsx:143 |  |  | 0.725 |
| ns | 3203 |  | 219 | CommandProps: value, onValueChange, loop, disablePointerSelection, vimBindings | 2.8 | 2.7 | 0.694 |
| walker |  | 3224 | 75 | module item at cmdk/src/index.tsx:13 |  |  | 0.694 |
| walker |  | 3309 | 85 | module item at cmdk/src/index.tsx:69 |  |  | 0.696 |
| walker |  | 3368 | 59 | module item at cmdk/src/index.tsx:930 |  |  | 0.726 |
| ns | 3430 |  | 227 | ItemProps (full, with JSDoc) | 2.9 | 2.5 | 0.703 |
| walker |  | 3461 | 93 | module item at cmdk/src/index.tsx:28 |  |  | 0.703 |
| walker |  | 3572 | 111 | module item at cmdk/src/index.tsx:60 |  |  | 0.705 |
| ns | 3588 |  | 158 | GroupProps, SeparatorProps, EmptyProps | 2.10 | 2.5 | 0.716 |
| ns | 3715 |  | 127 | InputProps + CommandFilter signature | 2.11 | 2.5 | 0.723 |
| walker |  | 3737 | 165 | module item at cmdk/src/index.tsx:123 |  |  | 0.727 |
| walker |  | 3859 | 122 | module item at cmdk/src/index.tsx:1081 |  |  | 0.727 |
| ns | 4080 |  | 365 | ARCHITECTURE: the three rejected APIs and why selection tracks value | 3.1 | 1.8 | 0.722 |
| walker |  | 4084 | 225 | module item at cmdk/src/index.tsx:44 |  |  | 0.755 |
| walker |  | 4132 | 48 | export doc at cmdk/src/index.tsx:774 |  |  | 0.766 |
| walker |  | 4184 | 52 | export doc at cmdk/src/index.tsx:833 |  |  | 0.780 |
| walker |  | 4250 | 66 | export doc at cmdk/src/index.tsx:664 |  |  | 0.802 |
| ns | 4438 |  | 358 | Internal types: Context, State, Store, Group | 3.2 |  | 0.812 |
| ns | 4659 |  | 221 | DOM selector constants + the three React contexts | 3.3 |  | 0.815 |
| walker |  | 4719 | 469 | module item at cmdk/src/index.tsx:80 |  |  | 0.888 |
| walker |  | 4751 | 32 | ARCHITECTURE.md section #4 |  |  | 0.892 |
| walker |  | 4770 | 19 | README.md section #24 |  |  | 0.892 |
| walker |  | 4796 | 26 | README.md section #19 |  |  | 0.892 |
| walker |  | 4822 | 26 | README.md section #22 |  |  | 0.892 |
| ns | 4873 |  | 214 | Roster of Command's internal functions | 3.4 |  | 0.874 |
| walker |  | 4895 | 73 | export at cmdk/tsup.config.ts:3 |  |  | 0.874 |
| walker |  | 4920 | 25 | README.md section #23 |  |  | 0.874 |
| walker |  | 4955 | 35 | README.md section #21 |  |  | 0.874 |
| walker |  | 5015 | 60 | listing of 'test/pages' |  |  | 0.875 |
| walker |  | 5034 | 19 | imports in playwright.config.ts |  |  | 0.875 |
| ns | 5086 |  | 213 | Store: subscribe/snapshot and the 'search' setState branch | 3.5 |  | 0.850 |
| walker |  | 5138 | 104 | export body at cmdk/src/index.tsx:774 body 775 |  |  | 0.850 |
| walker |  | 5229 | 91 | package dev/peer dependencies in cmdk/package.json |  |  | 0.853 |
| walker |  | 5255 | 26 | export names surface in cmdk/src/command-score.ts |  |  | 0.853 |
| walker |  | 5255 | 0 | export at cmdk/src/command-score.ts:155 |  |  | 0.853 |
| ns | 5300 |  | 214 | Root keydown handler: IME guard, vim down bindings, ArrowDown | 3.6 | 3.4 | 0.825 |
| walker |  | 5323 | 68 | ARCHITECTURE.md section #3 |  |  | 0.825 |
| walker |  | 5375 | 52 | README.md section #4 |  |  | 0.825 |
| walker |  | 5427 | 52 | README.md section #5 |  |  | 0.825 |
| walker |  | 5439 | 12 | imports in cmdk/tsup.config.ts |  |  | 0.825 |
| walker |  | 5479 | 40 | README.md section #34 |  |  | 0.825 |
| walker |  | 5620 | 141 | README.md section #37 |  |  | 0.858 |
| ns | 5678 |  | 378 | score() and sort(): per-group maximum score and the ordering rules | 4.1 | 3.4 | 0.819 |
| walker |  | 5771 | 151 | README.md section #36 |  |  | 0.819 |
| walker |  | 5834 | 63 | README.md section #10 |  |  | 0.819 |
| walker |  | 5897 | 63 | README.md section #17 |  |  | 0.819 |
| ns | 6027 |  | 349 | sort(): the DOM re-append loop | 4.2 | 4.1 | 0.791 |
| walker |  | 6139 | 242 | json config tsconfig.json |  |  | 0.791 |
| walker |  | 6205 | 66 | README.md section #26 |  |  | 0.791 |
| walker |  | 6276 | 71 | README.md section #20 |  |  | 0.791 |
| walker |  | 6349 | 73 | README.md section #27 |  |  | 0.791 |
| walker |  | 6426 | 77 | README.md section #18 |  |  | 0.791 |
| ns | 6432 |  | 405 | filterItems(): scoring every item and deriving visible groups | 4.3 | 3.4 | 0.759 |
| ns | 6702 |  | 270 | Item: value inference, render gate, select handlers | 4.4 | 2.6 | 0.741 |
| walker |  | 6865 | 439 | ARCHITECTURE.md section #0 |  |  | 0.744 |
| ns | 6880 |  | 178 | Item: rendered element and its attributes | 4.5 | 4.4 | 0.732 |
| walker |  | 7134 | 269 | export at playwright.config.ts:3 |  |  | 0.734 |
| ns | 7137 |  | 257 | Group: render gate and heading/items markup | 4.6 | 2.6 | 0.719 |
| walker |  | 7216 | 82 | README.md section #15 |  |  | 0.719 |
| walker |  | 7229 | 13 | listing of 'website/styles' |  |  | 0.719 |
| ns | 7329 |  | 192 | Input and Separator markup | 4.7 | 2.6 | 0.707 |
| walker |  | 7568 | 339 | YAML config at .github/workflows/test.yml |  |  | 0.709 |
| ns | 7595 |  | 266 | List, Dialog, Empty and Loading markup | 4.8 | 2.6 | 0.697 |
| walker |  | 7660 | 92 | README.md section #32 |  |  | 0.697 |
| walker |  | 7764 | 104 | README.md section #7 |  |  | 0.697 |
| ns | 7795 |  | 200 | Roster of module-level helpers | 4.9 |  | 0.699 |
| walker |  | 7869 | 105 | README.md section #14 |  |  | 0.699 |
| walker |  | 7958 | 89 | README.md section #25 |  |  | 0.699 |
| walker |  | 8052 | 94 | README.md section #13 |  |  | 0.699 |
| ns | 8098 |  | 303 | command-score: the full scoring weight table | 5.1 |  | 0.685 |
| walker |  | 8170 | 118 | README.md section #16 |  |  | 0.685 |
| walker |  | 8180 | 10 | listing of 'website/components/code' |  |  | 0.685 |
| walker |  | 8190 | 10 | listing of 'website/components/icons' |  |  | 0.685 |
| ns | 8194 |  | 96 | command-score: exported signature and alias handling | 5.2 | 5.1 | 0.683 |
| ns | 8254 |  | 60 | Test fixture pages listing | 6.1 |  | 0.687 |
| walker |  | 8316 | 126 | README.md section #6 |  |  | 0.687 |
| ns | 8432 |  | 178 | Spec names: basic behaviour | 6.2 |  | 0.681 |
| walker |  | 8447 | 131 | README.md section #9 |  |  | 0.681 |
| ns | 8577 |  | 145 | dialog.test.ts in full - the whole spec idiom | 6.3 |  | 0.674 |
| walker |  | 8582 | 135 | README.md section #8 |  |  | 0.674 |
| walker |  | 8592 | 10 | export names surface in test/pages/index.tsx |  |  | 0.674 |
| walker |  | 8592 | 0 | export at test/pages/index.tsx:28 |  |  | 0.674 |
| ns | 8740 |  | 163 | Spec names: item lifecycle and item-advanced | 6.4 |  | 0.669 |
| ns | 8957 |  | 217 | Spec names: group, props matrix, numeric values | 6.5 |  | 0.661 |
| walker |  | 9049 | 457 | export body at cmdk/src/index.tsx:729 body 730 |  |  | 0.682 |
| walker |  | 9205 | 156 | README.md section #12 |  |  | 0.682 |
| ns | 9217 |  | 260 | Spec names: keybinds (four describe blocks) | 6.6 |  | 0.674 |
| walker |  | 9361 | 156 | README.md section #11 |  |  | 0.674 |
| walker |  | 9373 | 12 | export at test/pages/index.tsx:3 |  |  | 0.674 |
| ns | 9473 |  | 256 | test/pages/dialog.tsx - a working usage page | 6.7 |  | 0.662 |
| ns | 9749 |  | 276 | Playwright config (whole file) | 6.8 |  | 0.670 |
| walker |  | 9806 | 433 | README.md section #35 |  |  | 0.696 |
| ns | 9914 |  | 165 | CI workflow | 6.9 |  | 0.701 |
| walker |  | 9934 | 128 | export body at cmdk/src/command-score.ts:155 body 156 |  |  | 0.703 |
| ns | 9987 |  | 73 | Website source listings (demos, pages, drop-in stylesheets) | 7.1 |  | 0.702 |

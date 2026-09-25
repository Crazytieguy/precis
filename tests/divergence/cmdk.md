Score(3000)=0.537 I=0.784 C=0.368 ns_rows≤3K=16/47 grid(1000/1442/2080/3000/4327/6240/9000)=0.382/0.300/0.277/0.537/0.811/0.772/0.660

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
| ns | 343 |  | 135 | Listings of all three packages + library source dir | 1.4 |  | 0.483 |
| walker |  | 451 | 259 | ts names cmdk/src/index.tsx |  |  | 0.488 |
| walker |  | 473 | 22 | ts decl cmdk/src/index.tsx:149 |  |  | 0.488 |
| walker |  | 508 | 35 | ts decl cmdk/src/index.tsx:24 |  |  | 0.488 |
| ns | 532 |  | 189 | Published package identity + entry points | 1.5 |  | 0.417 |
| walker |  | 560 | 52 | ts decl cmdk/src/index.tsx:37 |  |  | 0.417 |
| walker |  | 617 | 57 | ts decl cmdk/src/index.tsx:137 |  |  | 0.417 |
| walker |  | 691 | 74 | ts decl cmdk/src/index.tsx:143 |  |  | 0.418 |
| ns | 764 |  | 232 | Root package.json: identity and every workspace script | 1.6 |  | 0.380 |
| walker |  | 766 | 75 | ts decl cmdk/src/index.tsx:13 |  |  | 0.380 |
| walker |  | 851 | 85 | ts decl cmdk/src/index.tsx:69 |  |  | 0.382 |
| walker |  | 944 | 93 | ts decl cmdk/src/index.tsx:28 |  |  | 0.382 |
| ns | 1043 |  | 279 | README section map (all H2 + H3 headings) | 1.7 |  | 0.335 |
| walker |  | 1152 | 208 | ts names cmdk/src/index.tsx #1 |  |  | 0.336 |
| ns | 1225 |  | 182 | ARCHITECTURE: the core invariant + section map | 1.8 |  | 0.317 |
| walker |  | 1263 | 111 | ts decl cmdk/src/index.tsx:60 |  |  | 0.319 |
| walker |  | 1320 | 57 | headings outline in ARCHITECTURE.md |  |  | 0.327 |
| ns | 1361 |  | 136 | README testing steps (verbatim) | 1.9 | 1.7 | 0.300 |
| walker |  | 1416 | 96 | package runtime dependencies in cmdk/package.json |  |  | 0.300 |
| ns | 1473 |  | 112 | index.tsx imports + 'use client' | 2.1 |  | 0.290 |
| walker |  | 1593 | 177 | README headline in README.md |  |  | 0.299 |
| ns | 1690 |  | 217 | Public export surface (Command.* object + named exports) | 2.2 |  | 0.273 |
| walker |  | 1758 | 165 | ts decl cmdk/src/index.tsx:123 |  |  | 0.275 |
| walker |  | 1963 | 205 | ts names cmdk/src/index.tsx #2 |  |  | 0.277 |
| ns | 2118 |  | 428 | README FAQ (all twelve entries) | 2.3 | 1.7 | 0.256 |
| walker |  | 2188 | 225 | ts decl cmdk/src/index.tsx:44 |  |  | 0.258 |
| walker |  | 2268 | 80 | package scripts in cmdk/package.json |  |  | 0.295 |
| ns | 2297 |  | 179 | Component declaration roster (all nine forwardRef components) | 2.4 |  | 0.326 |
| walker |  | 2322 | 54 | listing of 'website' |  |  | 0.380 |
| walker |  | 2336 | 14 | listing of 'website/components' |  |  | 0.380 |
| walker |  | 2394 | 58 | listing of 'test' |  |  | 0.488 |
| ns | 2406 |  | 109 | Props type roster (all twelve type aliases) | 2.5 |  | 0.516 |
| ns | 2730 |  | 324 | Per-component JSDoc blocks | 2.6 | 2.4 | 0.478 |
| walker |  | 2776 | 382 | ts names cmdk/src/index.tsx #3 |  |  | 0.528 |
| walker |  | 2835 | 59 | ts decl cmdk/src/index.tsx:930 |  |  | 0.565 |
| walker |  | 2885 | 50 | ts decl cmdk/src/index.tsx:1071 |  |  | 0.565 |
| walker |  | 2950 | 65 | ts decl cmdk/src/index.tsx:1010 |  |  | 0.565 |
| ns | 2984 |  | 254 | CommandProps: label, shouldFilter, filter, defaultValue | 2.7 | 2.5 | 0.537 |
| walker |  | 3072 | 122 | ts decl cmdk/src/index.tsx:1081 |  |  | 0.537 |
| walker |  | 3089 | 17 | listing of 'website/pages' |  |  | 0.537 |
| ns | 3203 |  | 219 | CommandProps: value, onValueChange, loop, disablePointerSelection, vimBindings | 2.8 | 2.7 | 0.515 |
| walker |  | 3234 | 145 | package entrypoints in cmdk/package.json |  |  | 0.550 |
| ns | 3430 |  | 227 | ItemProps (full, with JSDoc) | 2.9 | 2.5 | 0.571 |
| walker |  | 3486 | 252 | ts decl cmdk/src/index.tsx:80 |  |  | 0.622 |
| walker |  | 3520 | 34 | plaintext config pnpm-workspace.yaml |  |  | 0.642 |
| walker |  | 3543 | 23 | listing of 'website/components/cmdk' |  |  | 0.642 |
| ns | 3588 |  | 158 | GroupProps, SeparatorProps, EmptyProps | 2.10 | 2.5 | 0.654 |
| ns | 3715 |  | 127 | InputProps + CommandFilter signature | 2.11 | 2.5 | 0.663 |
| walker |  | 3831 | 288 | headings outline in README.md |  |  | 0.722 |
| walker |  | 3857 | 26 | README.md section #1 |  |  | 0.738 |
| walker |  | 4059 | 202 | package scripts in package.json |  |  | 0.769 |
| ns | 4080 |  | 365 | ARCHITECTURE: the three rejected APIs and why selection tracks value | 3.1 | 1.8 | 0.764 |
| walker |  | 4091 | 32 | ARCHITECTURE.md section #5 |  |  | 0.768 |
| walker |  | 4308 | 217 | ts decl cmdk/src/index.tsx:80 #1 |  |  | 0.811 |
| walker |  | 4323 | 15 | ts doc cmdk/src/index.tsx:1004 |  |  | 0.811 |
| walker |  | 4349 | 26 | ts names cmdk/src/command-score.ts |  |  | 0.811 |
| walker |  | 4409 | 60 | listing of 'test/pages' |  |  | 0.813 |
| walker |  | 4431 | 22 | ts names test/pages/index.tsx |  |  | 0.813 |
| ns | 4438 |  | 358 | Internal types: Context, State, Store, Group | 3.2 |  | 0.821 |
| walker |  | 4603 | 172 | package identity metadata in cmdk/package.json |  |  | 0.821 |
| walker |  | 4629 | 26 | ts doc cmdk/src/index.tsx:882 |  |  | 0.821 |
| ns | 4659 |  | 221 | DOM selector constants + the three React contexts | 3.3 |  | 0.824 |
| walker |  | 4697 | 68 | ARCHITECTURE.md section #4 |  |  | 0.824 |
| walker |  | 4724 | 27 | ts doc cmdk/src/index.tsx:899 |  |  | 0.826 |
| walker |  | 4865 | 141 | README.md section #15 |  |  | 0.862 |
| ns | 4873 |  | 214 | Roster of Command's internal functions | 3.4 |  | 0.845 |
| ns | 5086 |  | 213 | Store: subscribe/snapshot and the 'search' setState branch | 3.5 |  | 0.821 |
| walker |  | 5175 | 310 | LICENSE.md section #0 |  |  | 0.821 |
| ns | 5300 |  | 214 | Root keydown handler: IME guard, vim down bindings, ArrowDown | 3.6 | 3.4 | 0.794 |
| walker |  | 5326 | 151 | README.md section #14 |  |  | 0.794 |
| walker |  | 5356 | 30 | ts doc cmdk/src/index.tsx:909 |  |  | 0.796 |
| walker |  | 5536 | 180 | README.md section #12 |  |  | 0.802 |
| ns | 5678 |  | 378 | score() and sort(): per-group maximum score and the ordering rules | 4.1 | 3.4 | 0.765 |
| walker |  | 5784 | 248 | README.md section #13 |  |  | 0.796 |
| ns | 6027 |  | 349 | sort(): the DOM re-append loop | 4.2 | 4.1 | 0.769 |
| walker |  | 6223 | 439 | ARCHITECTURE.md section #0 |  |  | 0.772 |
| walker |  | 6260 | 37 | ts doc cmdk/src/index.tsx:729 |  |  | 0.775 |
| walker |  | 6298 | 38 | ts doc cmdk/src/index.tsx:787 |  |  | 0.780 |
| walker |  | 6311 | 13 | listing of 'website/styles' |  |  | 0.780 |
| ns | 6432 |  | 405 | filterItems(): scoring every item and deriving visible groups | 4.3 | 3.4 | 0.748 |
| walker |  | 6607 | 296 | README.md section #3 |  |  | 0.748 |
| ns | 6702 |  | 270 | Item: value inference, render gate, select handlers | 4.4 | 2.6 | 0.731 |
| walker |  | 6851 | 244 | README.md section #4 |  |  | 0.731 |
| ns | 6880 |  | 178 | Item: rendered element and its attributes | 4.5 | 4.4 | 0.719 |
| walker |  | 6923 | 72 | ts names website/components/index.ts |  |  | 0.719 |
| walker |  | 6971 | 48 | ts doc cmdk/src/index.tsx:774 |  |  | 0.726 |
| walker |  | 6981 | 10 | listing of 'website/components/code' |  |  | 0.726 |
| walker |  | 6991 | 10 | listing of 'website/components/icons' |  |  | 0.726 |
| walker |  | 7037 | 46 | ts body cmdk/src/index.tsx:1004 |  |  | 0.726 |
| walker |  | 7089 | 52 | ts doc cmdk/src/index.tsx:833 |  |  | 0.734 |
| ns | 7137 |  | 257 | Group: render gate and heading/items markup | 4.6 | 2.6 | 0.719 |
| ns | 7329 |  | 192 | Input and Separator markup | 4.7 | 2.6 | 0.708 |
| ns | 7595 |  | 266 | List, Dialog, Empty and Loading markup | 4.8 | 2.6 | 0.696 |
| walker |  | 7613 | 524 | README.md section #5 |  |  | 0.696 |
| ns | 7795 |  | 200 | Roster of module-level helpers | 4.9 |  | 0.698 |
| walker |  | 8039 | 426 | README.md section #6 |  |  | 0.698 |
| ns | 8098 |  | 303 | command-score: the full scoring weight table | 5.1 |  | 0.684 |
| ns | 8194 |  | 96 | command-score: exported signature and alias handling | 5.2 | 5.1 | 0.682 |
| ns | 8254 |  | 60 | Test fixture pages listing | 6.1 |  | 0.686 |
| ns | 8432 |  | 178 | Spec names: basic behaviour | 6.2 |  | 0.680 |
| walker |  | 8513 | 474 | README.md section #7 |  |  | 0.680 |
| ns | 8577 |  | 145 | dialog.test.ts in full - the whole spec idiom | 6.3 |  | 0.673 |
| walker |  | 8662 | 149 | README.md section #8 |  |  | 0.673 |
| ns | 8740 |  | 163 | Spec names: item lifecycle and item-advanced | 6.4 |  | 0.668 |
| walker |  | 8904 | 242 | json config tsconfig.json |  |  | 0.668 |
| ns | 8957 |  | 217 | Spec names: group, props matrix, numeric values | 6.5 |  | 0.660 |
| walker |  | 9148 | 244 | ARCHITECTURE.md section #1 |  |  | 0.669 |
| ns | 9217 |  | 260 | Spec names: keybinds (four describe blocks) | 6.6 |  | 0.661 |
| walker |  | 9358 | 210 | ARCHITECTURE.md section #2 |  |  | 0.664 |
| ns | 9473 |  | 256 | test/pages/dialog.tsx - a working usage page | 6.7 |  | 0.652 |
| walker |  | 9560 | 202 | ts names website/pages/index.tsx |  |  | 0.652 |
| walker |  | 9582 | 22 | ts decl website/pages/index.tsx:20 |  |  | 0.652 |
| walker |  | 9601 | 19 | ts doc cmdk/src/index.tsx:1046 |  |  | 0.654 |
| ns | 9749 |  | 276 | Playwright config (whole file) | 6.8 |  | 0.642 |
| ns | 9914 |  | 165 | CI workflow | 6.9 |  | 0.633 |
| ns | 9987 |  | 73 | Website source listings (demos, pages, drop-in stylesheets) | 7.1 |  | 0.633 |

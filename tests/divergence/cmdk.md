Score(3000)=0.577 I=0.808 C=0.413 ns_rows≤3K=16/47 grid(1000/1442/2080/3000/4327/6240/9000)=0.394/0.309/0.277/0.577/0.811/0.736/0.651

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
| ns | 343 |  | 135 | Listings of all three packages + library source dir | 1.4 |  | 0.500 |
| ns | 532 |  | 189 | Published package identity + entry points | 1.5 |  | 0.426 |
| walker |  | 543 | 259 | ts names cmdk/src/index.tsx |  |  | 0.430 |
| walker |  | 565 | 22 | ts decl cmdk/src/index.tsx:149 |  |  | 0.430 |
| walker |  | 600 | 35 | ts decl cmdk/src/index.tsx:24 |  |  | 0.430 |
| walker |  | 652 | 52 | ts decl cmdk/src/index.tsx:37 |  |  | 0.430 |
| walker |  | 709 | 57 | ts decl cmdk/src/index.tsx:137 |  |  | 0.431 |
| ns | 764 |  | 232 | Root package.json: identity and every workspace script | 1.6 |  | 0.392 |
| walker |  | 783 | 74 | ts decl cmdk/src/index.tsx:143 |  |  | 0.393 |
| walker |  | 858 | 75 | ts decl cmdk/src/index.tsx:13 |  |  | 0.393 |
| walker |  | 943 | 85 | ts decl cmdk/src/index.tsx:69 |  |  | 0.394 |
| walker |  | 1036 | 93 | ts decl cmdk/src/index.tsx:28 |  |  | 0.394 |
| ns | 1043 |  | 279 | README section map (all H2 + H3 headings) | 1.7 |  | 0.346 |
| ns | 1225 |  | 182 | ARCHITECTURE: the core invariant + section map | 1.8 |  | 0.326 |
| walker |  | 1244 | 208 | ts names cmdk/src/index.tsx #1 |  |  | 0.328 |
| walker |  | 1355 | 111 | ts decl cmdk/src/index.tsx:60 |  |  | 0.329 |
| ns | 1361 |  | 136 | README testing steps (verbatim) | 1.9 | 1.7 | 0.303 |
| walker |  | 1412 | 57 | headings outline in ARCHITECTURE.md |  |  | 0.309 |
| ns | 1473 |  | 112 | index.tsx imports + 'use client' | 2.1 |  | 0.299 |
| walker |  | 1577 | 165 | ts decl cmdk/src/index.tsx:123 |  |  | 0.302 |
| walker |  | 1673 | 96 | package runtime dependencies in cmdk/package.json |  |  | 0.302 |
| ns | 1690 |  | 217 | Public export surface (Command.* object + named exports) | 2.2 |  | 0.275 |
| walker |  | 1878 | 205 | ts names cmdk/src/index.tsx #2 |  |  | 0.277 |
| walker |  | 2103 | 225 | ts decl cmdk/src/index.tsx:44 |  |  | 0.279 |
| ns | 2118 |  | 428 | README FAQ (all twelve entries) | 2.3 | 1.7 | 0.258 |
| walker |  | 2183 | 80 | package scripts in cmdk/package.json |  |  | 0.295 |
| walker |  | 2237 | 54 | listing of 'website' |  |  | 0.355 |
| walker |  | 2251 | 14 | listing of 'website/components' |  |  | 0.355 |
| ns | 2297 |  | 179 | Component declaration roster (all nine forwardRef components) | 2.4 |  | 0.380 |
| walker |  | 2309 | 58 | listing of 'test' |  |  | 0.488 |
| ns | 2406 |  | 109 | Props type roster (all twelve type aliases) | 2.5 |  | 0.516 |
| walker |  | 2511 | 202 | package scripts in package.json |  |  | 0.566 |
| ns | 2730 |  | 324 | Per-component JSDoc blocks | 2.6 | 2.4 | 0.524 |
| walker |  | 2893 | 382 | ts names cmdk/src/index.tsx #3 |  |  | 0.571 |
| walker |  | 2952 | 59 | ts decl cmdk/src/index.tsx:930 |  |  | 0.607 |
| ns | 2984 |  | 254 | CommandProps: label, shouldFilter, filter, defaultValue | 2.7 | 2.5 | 0.577 |
| walker |  | 3002 | 50 | ts decl cmdk/src/index.tsx:1071 |  |  | 0.577 |
| walker |  | 3067 | 65 | ts decl cmdk/src/index.tsx:1010 |  |  | 0.577 |
| walker |  | 3189 | 122 | ts decl cmdk/src/index.tsx:1081 |  |  | 0.577 |
| ns | 3203 |  | 219 | CommandProps: value, onValueChange, loop, disablePointerSelection, vimBindings | 2.8 | 2.7 | 0.553 |
| walker |  | 3206 | 17 | listing of 'website/pages' |  |  | 0.553 |
| walker |  | 3351 | 145 | package entrypoints in cmdk/package.json |  |  | 0.588 |
| ns | 3430 |  | 227 | ItemProps (full, with JSDoc) | 2.9 | 2.5 | 0.607 |
| ns | 3588 |  | 158 | GroupProps, SeparatorProps, EmptyProps | 2.10 | 2.5 | 0.621 |
| walker |  | 3603 | 252 | ts decl cmdk/src/index.tsx:80 |  |  | 0.668 |
| walker |  | 3637 | 34 | plaintext config pnpm-workspace.yaml |  |  | 0.687 |
| walker |  | 3660 | 23 | listing of 'website/components/cmdk' |  |  | 0.687 |
| ns | 3715 |  | 127 | InputProps + CommandFilter signature | 2.11 | 2.5 | 0.695 |
| walker |  | 3948 | 288 | headings outline in README.md |  |  | 0.753 |
| walker |  | 3962 | 14 | README.md section #28 |  |  | 0.753 |
| walker |  | 3988 | 26 | README.md section #1 |  |  | 0.769 |
| walker |  | 4032 | 44 | README.md section #3 |  |  | 0.769 |
| walker |  | 4064 | 32 | ARCHITECTURE.md section #4 |  |  | 0.774 |
| ns | 4080 |  | 365 | ARCHITECTURE: the three rejected APIs and why selection tracks value | 3.1 | 1.8 | 0.768 |
| walker |  | 4083 | 19 | README.md section #24 |  |  | 0.768 |
| walker |  | 4300 | 217 | ts decl cmdk/src/index.tsx:80 #1 |  |  | 0.811 |
| walker |  | 4326 | 26 | README.md section #19 |  |  | 0.811 |
| walker |  | 4352 | 26 | README.md section #22 |  |  | 0.811 |
| walker |  | 4378 | 26 | ts names cmdk/src/command-score.ts |  |  | 0.811 |
| walker |  | 4403 | 25 | README.md section #23 |  |  | 0.811 |
| walker |  | 4438 | 35 | README.md section #21 |  |  | 0.820 |
| ns | 4438 |  | 358 | Internal types: Context, State, Store, Group | 3.2 |  | 0.820 |
| walker |  | 4498 | 60 | listing of 'test/pages' |  |  | 0.821 |
| walker |  | 4520 | 22 | ts names test/pages/index.tsx |  |  | 0.821 |
| walker |  | 4611 | 91 | package dev/peer dependencies in cmdk/package.json |  |  | 0.823 |
| ns | 4659 |  | 221 | DOM selector constants + the three React contexts | 3.3 |  | 0.826 |
| walker |  | 4679 | 68 | ARCHITECTURE.md section #3 |  |  | 0.826 |
| walker |  | 4731 | 52 | README.md section #4 |  |  | 0.826 |
| walker |  | 4783 | 52 | README.md section #5 |  |  | 0.826 |
| walker |  | 4823 | 40 | README.md section #34 |  |  | 0.826 |
| walker |  | 4838 | 15 | ts doc cmdk/src/index.tsx:1004 |  |  | 0.827 |
| ns | 4873 |  | 214 | Roster of Command's internal functions | 3.4 |  | 0.810 |
| walker |  | 4979 | 141 | README.md section #37 |  |  | 0.846 |
| ns | 5086 |  | 213 | Store: subscribe/snapshot and the 'search' setState branch | 3.5 |  | 0.822 |
| walker |  | 5130 | 151 | README.md section #36 |  |  | 0.822 |
| walker |  | 5193 | 63 | README.md section #10 |  |  | 0.822 |
| walker |  | 5256 | 63 | README.md section #17 |  |  | 0.822 |
| ns | 5300 |  | 214 | Root keydown handler: IME guard, vim down bindings, ArrowDown | 3.6 | 3.4 | 0.795 |
| walker |  | 5498 | 242 | json config tsconfig.json |  |  | 0.795 |
| walker |  | 5564 | 66 | README.md section #26 |  |  | 0.795 |
| walker |  | 5635 | 71 | README.md section #20 |  |  | 0.795 |
| ns | 5678 |  | 378 | score() and sort(): per-group maximum score and the ordering rules | 4.1 | 3.4 | 0.759 |
| walker |  | 5708 | 73 | README.md section #27 |  |  | 0.759 |
| walker |  | 5785 | 77 | README.md section #18 |  |  | 0.759 |
| ns | 6027 |  | 349 | sort(): the DOM re-append loop | 4.2 | 4.1 | 0.733 |
| walker |  | 6224 | 439 | ARCHITECTURE.md section #0 |  |  | 0.736 |
| walker |  | 6306 | 82 | README.md section #15 |  |  | 0.736 |
| walker |  | 6319 | 13 | listing of 'website/styles' |  |  | 0.736 |
| walker |  | 6345 | 26 | ts doc cmdk/src/index.tsx:882 |  |  | 0.737 |
| walker |  | 6417 | 72 | ts names website/components/index.ts |  |  | 0.737 |
| ns | 6432 |  | 405 | filterItems(): scoring every item and deriving visible groups | 4.3 | 3.4 | 0.707 |
| ns | 6702 |  | 270 | Item: value inference, render gate, select handlers | 4.4 | 2.6 | 0.691 |
| walker |  | 6756 | 339 | YAML config at .github/workflows/test.yml |  |  | 0.692 |
| walker |  | 6848 | 92 | README.md section #32 |  |  | 0.692 |
| walker |  | 6875 | 27 | ts doc cmdk/src/index.tsx:899 |  |  | 0.693 |
| ns | 6880 |  | 178 | Item: rendered element and its attributes | 4.5 | 4.4 | 0.681 |
| walker |  | 6979 | 104 | README.md section #7 |  |  | 0.681 |
| walker |  | 7084 | 105 | README.md section #14 |  |  | 0.681 |
| ns | 7137 |  | 257 | Group: render gate and heading/items markup | 4.6 | 2.6 | 0.667 |
| walker |  | 7173 | 89 | README.md section #25 |  |  | 0.667 |
| walker |  | 7267 | 94 | README.md section #13 |  |  | 0.667 |
| ns | 7329 |  | 192 | Input and Separator markup | 4.7 | 2.6 | 0.656 |
| walker |  | 7385 | 118 | README.md section #16 |  |  | 0.656 |
| walker |  | 7395 | 10 | listing of 'website/components/code' |  |  | 0.656 |
| walker |  | 7405 | 10 | listing of 'website/components/icons' |  |  | 0.656 |
| walker |  | 7435 | 30 | ts doc cmdk/src/index.tsx:909 |  |  | 0.658 |
| walker |  | 7561 | 126 | README.md section #6 |  |  | 0.658 |
| ns | 7595 |  | 266 | List, Dialog, Empty and Loading markup | 4.8 | 2.6 | 0.647 |
| walker |  | 7692 | 131 | README.md section #9 |  |  | 0.647 |
| ns | 7795 |  | 200 | Roster of module-level helpers | 4.9 |  | 0.651 |
| walker |  | 7827 | 135 | README.md section #8 |  |  | 0.651 |
| walker |  | 7864 | 37 | ts doc cmdk/src/index.tsx:729 |  |  | 0.654 |
| walker |  | 8020 | 156 | README.md section #12 |  |  | 0.654 |
| ns | 8098 |  | 303 | command-score: the full scoring weight table | 5.1 |  | 0.640 |
| walker |  | 8176 | 156 | README.md section #11 |  |  | 0.640 |
| ns | 8194 |  | 96 | command-score: exported signature and alias handling | 5.2 | 5.1 | 0.638 |
| ns | 8254 |  | 60 | Test fixture pages listing | 6.1 |  | 0.643 |
| ns | 8432 |  | 178 | Spec names: basic behaviour | 6.2 |  | 0.637 |
| ns | 8577 |  | 145 | dialog.test.ts in full - the whole spec idiom | 6.3 |  | 0.631 |
| walker |  | 8609 | 433 | README.md section #35 |  |  | 0.660 |
| walker |  | 8647 | 38 | ts doc cmdk/src/index.tsx:787 |  |  | 0.664 |
| ns | 8740 |  | 163 | Spec names: item lifecycle and item-advanced | 6.4 |  | 0.658 |
| walker |  | 8849 | 202 | ts names website/pages/index.tsx |  |  | 0.658 |
| walker |  | 8871 | 22 | ts decl website/pages/index.tsx:20 |  |  | 0.658 |
| ns | 8957 |  | 217 | Spec names: group, props matrix, numeric values | 6.5 |  | 0.651 |
| ns | 9217 |  | 260 | Spec names: keybinds (four describe blocks) | 6.6 |  | 0.643 |
| ns | 9473 |  | 256 | test/pages/dialog.tsx - a working usage page | 6.7 |  | 0.631 |
| walker |  | 9512 | 641 | README.md section #2 |  |  | 0.631 |
| ns | 9749 |  | 276 | Playwright config (whole file) | 6.8 |  | 0.619 |
| walker |  | 9822 | 310 | LICENSE.md section #0 |  |  | 0.619 |
| ns | 9914 |  | 165 | CI workflow | 6.9 |  | 0.627 |
| ns | 9987 |  | 73 | Website source listings (demos, pages, drop-in stylesheets) | 7.1 |  | 0.627 |

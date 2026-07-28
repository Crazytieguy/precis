Score(3000)=0.720 I=0.891 C=0.581 ns_rows≤3K=16/47 (reached=9 partial=0 missing=7)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 67 | 67 | listing of '.' |  |  | 1.000 |
| ns | 67 |  | 67 | Repo root listing | 1.1 |  | 1.000 |
| walker |  | 80 | 13 | listing of 'cmdk' |  |  | 1.000 |
| walker |  | 107 | 27 | package identity in package.json |  |  | 1.000 |
| walker |  | 116 | 9 | listing of 'cmdk/src' |  |  | 1.000 |
| walker |  | 120 | 4 | listing of '.husky' |  |  | 1.000 |
| walker |  | 162 | 42 | package identity in cmdk/package.json |  |  | 1.000 |
| walker |  | 170 | 8 | listing of '.github' |  |  | 1.000 |
| walker |  | 173 | 3 | listing of '.github/workflows' |  |  | 1.000 |
| ns | 179 |  | 112 | README one-line identity + install | 1.2 |  | 0.846 |
| ns | 213 |  | 34 | pnpm workspace membership | 1.3 |  | 0.758 |
| ns | 351 |  | 138 | Listings of all three packages + library source dir | 1.4 |  | 0.483 |
| walker |  | 512 | 339 | YAML config at .github/workflows/test.yml |  |  | 0.485 |
| walker |  | 533 | 21 | package runtime metadata in package.json |  |  | 0.485 |
| ns | 540 |  | 189 | Published package identity + entry points | 1.5 |  | 0.414 |
| walker |  | 625 | 92 | README headline in README.md |  |  | 0.427 |
| walker |  | 682 | 57 | headings outline in ARCHITECTURE.md |  |  | 0.428 |
| walker |  | 692 | 10 | export names surface in playwright.config.ts |  |  | 0.428 |
| ns | 772 |  | 232 | Root package.json: identity and every workspace script | 1.6 |  | 0.389 |
| walker |  | 788 | 96 | package runtime dependencies in cmdk/package.json |  |  | 0.389 |
| walker |  | 868 | 80 | package scripts in cmdk/package.json |  |  | 0.447 |
| walker |  | 925 | 57 | listing of 'website' |  |  | 0.541 |
| walker |  | 941 | 16 | listing of 'website/components' |  |  | 0.541 |
| walker |  | 957 | 16 | listing of 'website/pages' |  |  | 0.541 |
| walker |  | 979 | 22 | listing of 'website/components/cmdk' |  |  | 0.542 |
| walker |  | 1037 | 58 | listing of 'test' |  |  | 0.725 |
| ns | 1051 |  | 279 | README section map (all H2 + H3 headings) | 1.7 |  | 0.635 |
| ns | 1233 |  | 182 | ARCHITECTURE: the core invariant + section map | 1.8 |  | 0.604 |
| walker |  | 1239 | 202 | package scripts in package.json |  |  | 0.679 |
| walker |  | 1252 | 13 | export names surface in cmdk/tsup.config.ts |  |  | 0.679 |
| ns | 1369 |  | 136 | README testing steps (verbatim) | 1.9 | 1.7 | 0.624 |
| walker |  | 1397 | 145 | package entrypoints in cmdk/package.json |  |  | 0.683 |
| walker |  | 1431 | 34 | plaintext config pnpm-workspace.yaml |  |  | 0.714 |
| ns | 1481 |  | 112 | index.tsx imports + 'use client' | 2.1 |  | 0.690 |
| ns | 1698 |  | 217 | Public export surface (Command.* object + named exports) | 2.2 |  | 0.630 |
| walker |  | 1719 | 288 | headings outline in README.md |  |  | 0.733 |
| walker |  | 1733 | 14 | README.md section #28 |  |  | 0.733 |
| walker |  | 1759 | 26 | README.md section #1 |  |  | 0.756 |
| walker |  | 1803 | 44 | README.md section #3 |  |  | 0.756 |
| ns | 2126 |  | 428 | README FAQ (all twelve entries) | 2.3 | 1.7 | 0.700 |
| walker |  | 2177 | 374 | export names surface in cmdk/src/index.tsx |  |  | 0.742 |
| walker |  | 2177 | 0 | export at cmdk/src/index.tsx:169 |  |  | 0.742 |
| walker |  | 2177 | 0 | export at cmdk/src/index.tsx:664 |  |  | 0.742 |
| walker |  | 2177 | 0 | export at cmdk/src/index.tsx:729 |  |  | 0.742 |
| walker |  | 2177 | 0 | export at cmdk/src/index.tsx:774 |  |  | 0.742 |
| walker |  | 2177 | 0 | export at cmdk/src/index.tsx:787 |  |  | 0.742 |
| walker |  | 2177 | 0 | export at cmdk/src/index.tsx:833 |  |  | 0.742 |
| walker |  | 2177 | 0 | export at cmdk/src/index.tsx:882 |  |  | 0.742 |
| walker |  | 2177 | 0 | export at cmdk/src/index.tsx:899 |  |  | 0.742 |
| walker |  | 2177 | 0 | export at cmdk/src/index.tsx:909 |  |  | 0.742 |
| walker |  | 2203 | 26 | export doc at cmdk/src/index.tsx:882 |  |  | 0.742 |
| walker |  | 2230 | 27 | export doc at cmdk/src/index.tsx:899 |  |  | 0.742 |
| ns | 2305 |  | 179 | Component declaration roster (all nine forwardRef components) | 2.4 |  | 0.750 |
| walker |  | 2334 | 104 | export body at cmdk/src/index.tsx:774 body 775 |  |  | 0.750 |
| walker |  | 2364 | 30 | export doc at cmdk/src/index.tsx:909 |  |  | 0.750 |
| walker |  | 2401 | 37 | export doc at cmdk/src/index.tsx:729 |  |  | 0.751 |
| ns | 2414 |  | 109 | Props type roster (all twelve type aliases) | 2.5 |  | 0.726 |
| walker |  | 2439 | 38 | export doc at cmdk/src/index.tsx:787 |  |  | 0.727 |
| walker |  | 2487 | 48 | export doc at cmdk/src/index.tsx:774 |  |  | 0.728 |
| walker |  | 2539 | 52 | export doc at cmdk/src/index.tsx:833 |  |  | 0.730 |
| walker |  | 2605 | 66 | export doc at cmdk/src/index.tsx:664 |  |  | 0.734 |
| ns | 2738 |  | 324 | Per-component JSDoc blocks | 2.6 | 2.4 | 0.757 |
| ns | 2992 |  | 254 | CommandProps: label, shouldFilter, filter, defaultValue | 2.7 | 2.5 | 0.720 |
| walker |  | 3100 | 495 | module item names surface in cmdk/src/index.tsx |  |  | 0.729 |
| walker |  | 3100 | 0 | module item at cmdk/src/index.tsx:10 |  |  | 0.729 |
| walker |  | 3100 | 0 | module item at cmdk/src/index.tsx:79 |  |  | 0.729 |
| walker |  | 3100 | 0 | module item at cmdk/src/index.tsx:154 |  |  | 0.729 |
| walker |  | 3100 | 0 | module item at cmdk/src/index.tsx:155 |  |  | 0.729 |
| walker |  | 3100 | 0 | module item at cmdk/src/index.tsx:156 |  |  | 0.729 |
| walker |  | 3100 | 0 | module item at cmdk/src/index.tsx:157 |  |  | 0.729 |
| walker |  | 3100 | 0 | module item at cmdk/src/index.tsx:158 |  |  | 0.729 |
| walker |  | 3100 | 0 | module item at cmdk/src/index.tsx:159 |  |  | 0.729 |
| walker |  | 3100 | 0 | module item at cmdk/src/index.tsx:160 |  |  | 0.729 |
| walker |  | 3100 | 0 | module item at cmdk/src/index.tsx:161 |  |  | 0.729 |
| walker |  | 3100 | 0 | module item at cmdk/src/index.tsx:163 |  |  | 0.729 |
| walker |  | 3100 | 0 | module item at cmdk/src/index.tsx:164 |  |  | 0.729 |
| walker |  | 3100 | 0 | module item at cmdk/src/index.tsx:165 |  |  | 0.729 |
| walker |  | 3100 | 0 | module item at cmdk/src/index.tsx:166 |  |  | 0.729 |
| walker |  | 3100 | 0 | module item at cmdk/src/index.tsx:167 |  |  | 0.729 |
| walker |  | 3100 | 0 | module item at cmdk/src/index.tsx:963 |  |  | 0.729 |
| walker |  | 3100 | 0 | module item at cmdk/src/index.tsx:972 |  |  | 0.729 |
| walker |  | 3100 | 0 | module item at cmdk/src/index.tsx:981 |  |  | 0.729 |
| walker |  | 3100 | 0 | module item at cmdk/src/index.tsx:991 |  |  | 0.729 |
| walker |  | 3100 | 0 | module item at cmdk/src/index.tsx:993 |  |  | 0.729 |
| walker |  | 3100 | 0 | module item at cmdk/src/index.tsx:1046 |  |  | 0.729 |
| walker |  | 3100 | 0 | module item at cmdk/src/index.tsx:1061 |  |  | 0.729 |
| walker |  | 3122 | 22 | module item at cmdk/src/index.tsx:149 |  |  | 0.729 |
| walker |  | 3172 | 50 | module item at cmdk/src/index.tsx:1071 |  |  | 0.729 |
| ns | 3211 |  | 219 | CommandProps: value, onValueChange, loop, disablePointerSelection, vimBindings | 2.8 | 2.7 | 0.699 |
| walker |  | 3229 | 57 | module item at cmdk/src/index.tsx:137 |  |  | 0.699 |
| walker |  | 3294 | 65 | module item at cmdk/src/index.tsx:1010 |  |  | 0.699 |
| walker |  | 3368 | 74 | module item at cmdk/src/index.tsx:143 |  |  | 0.700 |
| walker |  | 3427 | 59 | module item at cmdk/src/index.tsx:930 |  |  | 0.729 |
| ns | 3438 |  | 227 | ItemProps (full, with JSDoc) | 2.9 | 2.5 | 0.706 |
| walker |  | 3592 | 165 | module item at cmdk/src/index.tsx:123 |  |  | 0.710 |
| ns | 3596 |  | 158 | GroupProps, SeparatorProps, EmptyProps | 2.10 | 2.5 | 0.692 |
| walker |  | 3714 | 122 | module item at cmdk/src/index.tsx:1081 |  |  | 0.692 |
| ns | 3723 |  | 127 | InputProps + CommandFilter signature | 2.11 | 2.5 | 0.678 |
| walker |  | 3822 | 108 | imports in cmdk/src/index.tsx |  |  | 0.698 |
| walker |  | 3854 | 32 | ARCHITECTURE.md section #4 |  |  | 0.702 |
| walker |  | 3873 | 19 | README.md section #24 |  |  | 0.702 |
| walker |  | 4014 | 141 | README.md section #37 |  |  | 0.747 |
| walker |  | 4073 | 59 | listing of 'test/pages' |  |  | 0.748 |
| ns | 4088 |  | 365 | ARCHITECTURE: the three rejected APIs and why selection tracks value | 3.1 | 1.8 | 0.742 |
| walker |  | 4099 | 26 | README.md section #19 |  |  | 0.742 |
| walker |  | 4125 | 26 | README.md section #22 |  |  | 0.742 |
| walker |  | 4237 | 112 | package dev/peer dependencies in package.json |  |  | 0.742 |
| walker |  | 4310 | 73 | export at cmdk/tsup.config.ts:3 |  |  | 0.742 |
| walker |  | 4335 | 25 | README.md section #23 |  |  | 0.742 |
| walker |  | 4370 | 35 | README.md section #21 |  |  | 0.742 |
| walker |  | 4389 | 19 | imports in playwright.config.ts |  |  | 0.742 |
| ns | 4446 |  | 358 | Internal types: Context, State, Store, Group | 3.2 |  | 0.758 |
| walker |  | 4480 | 91 | package dev/peer dependencies in cmdk/package.json |  |  | 0.761 |
| walker |  | 4506 | 26 | export names surface in cmdk/src/command-score.ts |  |  | 0.761 |
| walker |  | 4506 | 0 | export at cmdk/src/command-score.ts:155 |  |  | 0.761 |
| walker |  | 4574 | 68 | ARCHITECTURE.md section #3 |  |  | 0.761 |
| walker |  | 4626 | 52 | README.md section #4 |  |  | 0.761 |
| ns | 4667 |  | 221 | DOM selector constants + the three React contexts | 3.3 |  | 0.767 |
| walker |  | 4678 | 52 | README.md section #5 |  |  | 0.767 |
| walker |  | 4690 | 12 | imports in cmdk/tsup.config.ts |  |  | 0.767 |
| walker |  | 4730 | 40 | README.md section #34 |  |  | 0.767 |
| walker |  | 4881 | 151 | README.md section #36 |  |  | 0.752 |
| ns | 4881 |  | 214 | Roster of Command's internal functions | 3.4 |  | 0.752 |
| walker |  | 4894 | 13 | listing of 'website/styles' |  |  | 0.752 |
| walker |  | 4957 | 63 | README.md section #10 |  |  | 0.752 |
| walker |  | 5020 | 63 | README.md section #17 |  |  | 0.752 |
| ns | 5094 |  | 213 | Store: subscribe/snapshot and the 'search' setState branch | 3.5 |  | 0.730 |
| walker |  | 5262 | 242 | json config tsconfig.json |  |  | 0.730 |
| ns | 5308 |  | 214 | Root keydown handler: IME guard, vim down bindings, ArrowDown | 3.6 | 3.4 | 0.707 |
| walker |  | 5328 | 66 | README.md section #26 |  |  | 0.707 |
| walker |  | 5399 | 71 | README.md section #20 |  |  | 0.707 |
| walker |  | 5472 | 73 | README.md section #27 |  |  | 0.707 |
| walker |  | 5481 | 9 | listing of 'website/components/code' |  |  | 0.707 |
| walker |  | 5490 | 9 | listing of 'website/components/icons' |  |  | 0.707 |
| walker |  | 5567 | 77 | README.md section #18 |  |  | 0.707 |
| ns | 5686 |  | 378 | score() and sort(): per-group maximum score and the ordering rules | 4.1 | 3.4 | 0.675 |
| walker |  | 6006 | 439 | ARCHITECTURE.md section #0 |  |  | 0.678 |
| ns | 6035 |  | 349 | sort(): the DOM re-append loop | 4.2 | 4.1 | 0.655 |
| walker |  | 6088 | 82 | README.md section #15 |  |  | 0.655 |
| walker |  | 6180 | 92 | README.md section #32 |  |  | 0.655 |
| walker |  | 6284 | 104 | README.md section #7 |  |  | 0.655 |
| walker |  | 6389 | 105 | README.md section #14 |  |  | 0.655 |
| ns | 6440 |  | 405 | filterItems(): scoring every item and deriving visible groups | 4.3 | 3.4 | 0.628 |
| ns | 6710 |  | 270 | Item: value inference, render gate, select handlers | 4.4 | 2.6 | 0.614 |
| walker |  | 6846 | 457 | export body at cmdk/src/index.tsx:729 body 730 |  |  | 0.616 |
| ns | 6888 |  | 178 | Item: rendered element and its attributes | 4.5 | 4.4 | 0.605 |
| walker |  | 6935 | 89 | README.md section #25 |  |  | 0.605 |
| walker |  | 7029 | 94 | README.md section #13 |  |  | 0.605 |
| ns | 7145 |  | 257 | Group: render gate and heading/items markup | 4.6 | 2.6 | 0.618 |
| walker |  | 7147 | 118 | README.md section #16 |  |  | 0.618 |
| walker |  | 7273 | 126 | README.md section #6 |  |  | 0.618 |
| ns | 7337 |  | 192 | Input and Separator markup | 4.7 | 2.6 | 0.609 |
| walker |  | 7404 | 131 | README.md section #9 |  |  | 0.609 |
| walker |  | 7422 | 18 | listing of 'website/styles/cmdk' |  |  | 0.609 |
| walker |  | 7557 | 135 | README.md section #8 |  |  | 0.609 |
| walker |  | 7567 | 10 | export names surface in test/pages/index.tsx |  |  | 0.609 |
| walker |  | 7567 | 0 | export at test/pages/index.tsx:28 |  |  | 0.609 |
| ns | 7603 |  | 266 | List, Dialog, Empty and Loading markup | 4.8 | 2.6 | 0.599 |
| walker |  | 7610 | 43 | listing of 'website/public' |  |  | 0.599 |
| walker |  | 7766 | 156 | README.md section #12 |  |  | 0.599 |
| ns | 7803 |  | 200 | Roster of module-level helpers | 4.9 |  | 0.599 |
| walker |  | 7922 | 156 | README.md section #11 |  |  | 0.599 |
| walker |  | 7934 | 12 | export at test/pages/index.tsx:3 |  |  | 0.599 |
| ns | 8106 |  | 303 | command-score: the full scoring weight table | 5.1 |  | 0.587 |
| ns | 8202 |  | 96 | command-score: exported signature and alias handling | 5.2 | 5.1 | 0.585 |
| ns | 8261 |  | 59 | Test fixture pages listing | 6.1 |  | 0.591 |
| walker |  | 8367 | 433 | README.md section #35 |  |  | 0.622 |
| ns | 8439 |  | 178 | Spec names: basic behaviour | 6.2 |  | 0.616 |
| ns | 8584 |  | 145 | dialog.test.ts in full - the whole spec idiom | 6.3 |  | 0.610 |
| ns | 8747 |  | 163 | Spec names: item lifecycle and item-advanced | 6.4 |  | 0.605 |
| ns | 8964 |  | 217 | Spec names: group, props matrix, numeric values | 6.5 |  | 0.598 |
| walker |  | 9050 | 683 | export body at cmdk/src/index.tsx:664 body 665 |  |  | 0.639 |
| walker |  | 9178 | 128 | export body at cmdk/src/command-score.ts:155 body 156 |  |  | 0.642 |
| ns | 9224 |  | 260 | Spec names: keybinds (four describe blocks) | 6.6 |  | 0.634 |
| ns | 9480 |  | 256 | test/pages/dialog.tsx - a working usage page | 6.7 |  | 0.623 |
| ns | 9756 |  | 276 | Playwright config (whole file) | 6.8 |  | 0.611 |
| walker |  | 9819 | 641 | README.md section #2 |  |  | 0.611 |
| ns | 9921 |  | 165 | CI workflow | 6.9 |  | 0.619 |
| ns | 9995 |  | 74 | Website source listings (demos, pages, drop-in stylesheets) | 7.1 |  | 0.625 |

Score(3000)=0.836 I=0.938 C=0.746 ns_rows≤3K=16/47 grid(1000/1442/2080/3000/4327/6240/9000)=0.858/0.851/0.846/0.836/0.755/0.597/0.507

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
| walker |  | 1596 | 170 | Code::CodeKey { rung: Names, file: cmdk/src/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.823 |
| walker |  | 1620 | 24 | Code::CodeKey { rung: Decl, file: cmdk/src/index.tsx, decl: 1, sub: 0, line: 149 } |  |  | 0.823 |
| ns | 1690 |  | 217 | Public export surface (Command.* object + named exports) | 2.2 |  | 0.751 |
| walker |  | 1904 | 284 | Code::CodeKey { rung: Names, file: cmdk/src/index.tsx, decl: 0, sub: 1, line: 0 } |  |  | 0.802 |
| walker |  | 1963 | 59 | Code::CodeKey { rung: Decl, file: cmdk/src/index.tsx, decl: 12, sub: 0, line: 930 } |  |  | 0.845 |
| walker |  | 1978 | 15 | Code::CodeKey { rung: Doc, file: cmdk/src/index.tsx, decl: 13, sub: 0, line: 1004 } |  |  | 0.845 |
| walker |  | 2004 | 26 | Code::CodeKey { rung: Doc, file: cmdk/src/index.tsx, decl: 9, sub: 0, line: 882 } |  |  | 0.845 |
| walker |  | 2031 | 27 | Code::CodeKey { rung: Doc, file: cmdk/src/index.tsx, decl: 10, sub: 0, line: 899 } |  |  | 0.846 |
| walker |  | 2061 | 30 | Code::CodeKey { rung: Doc, file: cmdk/src/index.tsx, decl: 11, sub: 0, line: 909 } |  |  | 0.846 |
| walker |  | 2098 | 37 | Code::CodeKey { rung: Doc, file: cmdk/src/index.tsx, decl: 5, sub: 0, line: 729 } |  |  | 0.847 |
| ns | 2118 |  | 428 | README FAQ (all twelve entries) | 2.3 | 1.7 | 0.785 |
| walker |  | 2136 | 38 | Code::CodeKey { rung: Doc, file: cmdk/src/index.tsx, decl: 7, sub: 0, line: 787 } |  |  | 0.786 |
| walker |  | 2184 | 48 | Code::CodeKey { rung: Doc, file: cmdk/src/index.tsx, decl: 6, sub: 0, line: 774 } |  |  | 0.788 |
| walker |  | 2236 | 52 | Code::CodeKey { rung: Doc, file: cmdk/src/index.tsx, decl: 8, sub: 0, line: 833 } |  |  | 0.790 |
| ns | 2297 |  | 179 | Component declaration roster (all nine forwardRef components) | 2.4 |  | 0.796 |
| walker |  | 2302 | 66 | Code::CodeKey { rung: Doc, file: cmdk/src/index.tsx, decl: 4, sub: 0, line: 664 } |  |  | 0.799 |
| walker |  | 2362 | 60 | Fs::DirListing { dir: test/pages } |  |  | 0.800 |
| ns | 2406 |  | 109 | Props type roster (all twelve type aliases) | 2.5 |  | 0.773 |
| walker |  | 2503 | 141 | Markdown::Section { file: README.md, section_index: 15, keeps_default_concavity: false } |  |  | 0.834 |
| walker |  | 2654 | 151 | Markdown::Section { file: README.md, section_index: 14, keeps_default_concavity: false } |  |  | 0.834 |
| ns | 2730 |  | 324 | Per-component JSDoc blocks | 2.6 | 2.4 | 0.844 |
| walker |  | 2834 | 180 | Markdown::Section { file: README.md, section_index: 12, keeps_default_concavity: false } |  |  | 0.852 |
| ns | 2984 |  | 254 | CommandProps: label, shouldFilter, filter, defaultValue | 2.7 | 2.5 | 0.810 |
| walker |  | 3082 | 248 | Markdown::Section { file: README.md, section_index: 13, keeps_default_concavity: false } |  |  | 0.858 |
| ns | 3203 |  | 219 | CommandProps: value, onValueChange, loop, disablePointerSelection, vimBindings | 2.8 | 2.7 | 0.822 |
| walker |  | 3378 | 296 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.822 |
| ns | 3430 |  | 227 | ItemProps (full, with JSDoc) | 2.9 | 2.5 | 0.797 |
| ns | 3588 |  | 158 | GroupProps, SeparatorProps, EmptyProps | 2.10 | 2.5 | 0.776 |
| walker |  | 3622 | 244 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.776 |
| walker |  | 3635 | 13 | Fs::DirListing { dir: website/styles } |  |  | 0.776 |
| walker |  | 3661 | 26 | Code::CodeKey { rung: Names, file: cmdk/src/command-score.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.776 |
| ns | 3715 |  | 127 | InputProps + CommandFilter signature | 2.11 | 2.5 | 0.760 |
| ns | 4048 |  | 333 | ARCHITECTURE: the three rejected APIs and why selection tracks value | 3.1 | 1.8 | 0.755 |
| walker |  | 4185 | 524 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.755 |
| ns | 4406 |  | 358 | Internal types: Context, State, Store, Group | 3.2 |  | 0.718 |
| walker |  | 4611 | 426 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.718 |
| ns | 4627 |  | 221 | DOM selector constants + the three React contexts | 3.3 |  | 0.703 |
| ns | 4841 |  | 214 | Roster of Command's internal functions | 3.4 |  | 0.689 |
| ns | 5054 |  | 213 | Store: subscribe/snapshot and the 'search' setState branch | 3.5 |  | 0.669 |
| walker |  | 5085 | 474 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.669 |
| walker |  | 5234 | 149 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.669 |
| ns | 5268 |  | 214 | Root keydown handler: IME guard, vim down bindings, ArrowDown | 3.6 | 3.4 | 0.647 |
| ns | 5646 |  | 378 | score() and sort(): per-group maximum score and the ordering rules | 4.1 | 3.4 | 0.618 |
| walker |  | 5875 | 641 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.618 |
| walker |  | 5921 | 46 | Code::CodeKey { rung: Body, file: cmdk/src/index.tsx, decl: 13, sub: 0, line: 1004 } |  |  | 0.618 |
| ns | 5995 |  | 349 | sort(): the DOM re-append loop | 4.2 | 4.1 | 0.597 |
| ns | 6400 |  | 405 | filterItems(): scoring every item and deriving visible groups | 4.3 | 3.4 | 0.573 |
| walker |  | 6498 | 577 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.573 |
| ns | 6670 |  | 270 | Item: value inference, render gate, select handlers | 4.4 | 2.6 | 0.560 |
| ns | 6848 |  | 178 | Item: rendered element and its attributes | 4.5 | 4.4 | 0.550 |
| walker |  | 7033 | 535 | Markdown::Section { file: README.md, section_index: 10, keeps_default_concavity: false } |  |  | 0.550 |
| ns | 7105 |  | 257 | Group: render gate and heading/items markup | 4.6 | 2.6 | 0.539 |
| ns | 7297 |  | 192 | Input and Separator markup | 4.7 | 2.6 | 0.530 |
| walker |  | 7359 | 326 | Markdown::Section { file: README.md, section_index: 11, keeps_default_concavity: false } |  |  | 0.530 |
| walker |  | 7378 | 19 | Fs::DirListing { dir: website/styles/cmdk } |  |  | 0.531 |
| ns | 7563 |  | 266 | List, Dialog, Empty and Loading markup | 4.8 | 2.6 | 0.522 |
| walker |  | 7620 | 242 | Json::Whole { file: tsconfig.json } |  |  | 0.522 |
| walker |  | 7664 | 44 | Fs::DirListing { dir: website/public } |  |  | 0.522 |
| walker |  | 7730 | 66 | Code::CodeKey { rung: Body, file: cmdk/src/index.tsx, decl: 10, sub: 0, line: 899 } |  |  | 0.522 |
| ns | 7763 |  | 200 | Roster of module-level helpers | 4.9 |  | 0.516 |
| walker |  | 7858 | 128 | Code::CodeKey { rung: Body, file: cmdk/src/command-score.ts, decl: 1, sub: 0, line: 155 } |  |  | 0.517 |
| walker |  | 7962 | 104 | Code::CodeKey { rung: Body, file: cmdk/src/index.tsx, decl: 6, sub: 0, line: 774 } |  |  | 0.517 |
| walker |  | 7984 | 22 | Code::CodeKey { rung: Names, file: test/pages/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.517 |
| ns | 8066 |  | 303 | command-score: the full scoring weight table | 5.1 |  | 0.506 |
| ns | 8162 |  | 96 | command-score: exported signature and alias handling | 5.2 | 5.1 | 0.508 |
| walker |  | 8164 | 180 | Code::CodeKey { rung: Body, file: cmdk/src/index.tsx, decl: 9, sub: 0, line: 882 } |  |  | 0.513 |
| ns | 8222 |  | 60 | Test fixture pages listing | 6.1 |  | 0.521 |
| walker |  | 8236 | 72 | Code::CodeKey { rung: Names, file: website/components/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.521 |
| ns | 8400 |  | 178 | Spec names: basic behaviour | 6.2 |  | 0.516 |
| walker |  | 8422 | 186 | Code::CodeKey { rung: Body, file: cmdk/src/index.tsx, decl: 11, sub: 0, line: 909 } |  |  | 0.522 |
| walker |  | 8436 | 14 | Code::CodeKey { rung: Names, file: website/pages/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.522 |
| ns | 8545 |  | 145 | dialog.test.ts in full - the whole spec idiom | 6.3 |  | 0.517 |
| walker |  | 8604 | 168 | Code::CodeKey { rung: Names, file: website/components/icons/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.517 |
| walker |  | 8636 | 32 | Json::Identity { file: test/package.json } |  |  | 0.517 |
| walker |  | 8662 | 26 | Json::Scripts { file: test/package.json } |  |  | 0.517 |
| walker |  | 8682 | 20 | Code::CodeKey { rung: Names, file: website/components/code/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.517 |
| walker |  | 8708 | 26 | Plaintext::DeclSurface { file: test/style.css } |  |  | 0.513 |
| ns | 8708 |  | 163 | Spec names: item lifecycle and item-advanced | 6.4 |  | 0.513 |
| walker |  | 8711 | 3 | Plaintext::Whole { file: test/style.css } |  |  | 0.513 |
| walker |  | 8754 | 43 | Json::Identity { file: website/package.json } |  |  | 0.513 |
| walker |  | 8833 | 79 | Json::Scripts { file: website/package.json } |  |  | 0.513 |
| walker |  | 8848 | 15 | Plaintext::DeclSurface { file: website/public/robots.txt } |  |  | 0.513 |
| walker |  | 8881 | 33 | Plaintext::DeclSurface { file: .husky/pre-commit } |  |  | 0.513 |
| walker |  | 8887 | 6 | Plaintext::Whole { file: .husky/pre-commit } |  |  | 0.513 |
| ns | 8925 |  | 217 | Spec names: group, props matrix, numeric values | 6.5 |  | 0.507 |
| ns | 9185 |  | 260 | Spec names: keybinds (four describe blocks) | 6.6 |  | 0.502 |
| walker |  | 9304 | 417 | Code::CodeKey { rung: Body, file: cmdk/src/index.tsx, decl: 7, sub: 0, line: 787 } |  |  | 0.521 |
| ns | 9441 |  | 256 | test/pages/dialog.tsx - a working usage page | 6.7 |  | 0.511 |
| walker |  | 9503 | 199 | Code::CodeKey { rung: Body, file: cmdk/src/index.tsx, decl: 5, sub: 0, line: 729 } |  |  | 0.512 |
| walker |  | 9525 | 22 | Code::CodeKey { rung: Names, file: test/pages/dialog.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.512 |
| walker |  | 9547 | 22 | Code::CodeKey { rung: Names, file: test/pages/group.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.512 |
| walker |  | 9569 | 22 | Code::CodeKey { rung: Names, file: test/pages/huge.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.512 |
| walker |  | 9591 | 22 | Code::CodeKey { rung: Names, file: test/pages/item-advanced.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.512 |
| walker |  | 9613 | 22 | Code::CodeKey { rung: Names, file: test/pages/item.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.512 |
| walker |  | 9635 | 22 | Code::CodeKey { rung: Names, file: test/pages/keybinds.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.512 |
| walker |  | 9657 | 22 | Code::CodeKey { rung: Names, file: test/pages/numeric.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.512 |
| walker |  | 9679 | 22 | Code::CodeKey { rung: Names, file: test/pages/portal.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.512 |
| walker |  | 9701 | 22 | Code::CodeKey { rung: Names, file: test/pages/props.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.512 |
| ns | 9717 |  | 276 | Playwright config (whole file) | 6.8 |  | 0.502 |
| ns | 9882 |  | 165 | CI workflow | 6.9 |  | 0.495 |
| walker |  | 9954 | 253 | Code::CodeKey { rung: Body, file: cmdk/src/index.tsx, decl: 5, sub: 1, line: 729 } |  |  | 0.517 |
| ns | 9955 |  | 73 | Website source listings (demos, pages, drop-in stylesheets) | 7.1 |  | 0.526 |
| walker |  | 9991 | 37 | Code::CodeKey { rung: Body, file: cmdk/src/index.tsx, decl: 8, sub: 0, line: 833 } |  |  | 0.526 |

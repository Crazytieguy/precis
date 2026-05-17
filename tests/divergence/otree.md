Score(3000)=0.608 I=0.801 C=0.461 ns_rows≤3K=20/50 (reached=7 partial=5 missing=8)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 35 | 35 | listing of '.' |  |  | 1.000 |
| ns | 35 |  | 35 | Repo root listing | 1.1 |  | 1.000 |
| walker |  | 39 | 4 | listing of 'assets' |  |  | 1.000 |
| ns | 54 |  | 19 | README H1 + crate one-liner | 1.2 |  | 0.911 |
| walker |  | 79 | 40 | README headline in README.md |  |  | 0.955 |
| walker |  | 87 | 8 | listing of 'config' |  |  | 0.955 |
| ns | 92 |  | 38 | src/ module layout | 1.3 |  | 0.690 |
| walker |  | 100 | 13 | listing of 'docs' |  |  | 0.690 |
| ns | 156 |  | 64 | main.rs module declarations | 1.4 |  | 0.569 |
| ns | 185 |  | 29 | README section headings | 1.5 |  | 0.532 |
| walker |  | 239 | 139 | [package] in Cargo.toml |  |  | 0.552 |
| walker |  | 277 | 38 | listing of 'src' |  |  | 0.772 |
| walker |  | 288 | 11 | macro_export names across src |  |  | 0.772 |
| walker |  | 298 | 10 | entry item at src/main.rs:111 |  |  | 0.772 |
| walker |  | 314 | 16 | listing of 'src/config' |  |  | 0.773 |
| walker |  | 327 | 13 | entry item at src/main.rs:27 |  |  | 0.773 |
| ns | 329 |  | 144 | Cargo package metadata | 1.6 |  | 0.789 |
| walker |  | 336 | 9 | entry item body at src/main.rs:27 body 52 |  |  | 0.789 |
| walker |  | 346 | 10 | entry item body at src/main.rs:27 body 108 |  |  | 0.789 |
| walker |  | 357 | 11 | entry item body at src/main.rs:27 body 54 |  |  | 0.789 |
| walker |  | 368 | 11 | entry item body at src/main.rs:27 body 58 |  |  | 0.789 |
| walker |  | 380 | 12 | entry item body at src/main.rs:27 body 38 |  |  | 0.789 |
| ns | 391 |  | 62 | ContentType variants — locations only | 2.1 |  | 0.721 |
| walker |  | 392 | 12 | entry item body at src/main.rs:27 body 99 |  |  | 0.721 |
| walker |  | 405 | 13 | entry item body at src/main.rs:27 body 41 |  |  | 0.721 |
| walker |  | 419 | 14 | entry item body at src/main.rs:27 body 39 |  |  | 0.721 |
| walker |  | 433 | 14 | entry item body at src/main.rs:27 body 55 |  |  | 0.722 |
| walker |  | 452 | 19 | entry item body at src/main.rs:27 body 101 |  |  | 0.722 |
| walker |  | 476 | 24 | entry item body at src/main.rs:27 body 51 |  |  | 0.722 |
| walker |  | 501 | 25 | entry item body at src/main.rs:27 body 43 |  |  | 0.723 |
| walker |  | 527 | 26 | entry item body at src/main.rs:27 body 98 |  |  | 0.723 |
| ns | 534 |  | 143 | ContentType enum context | 2.2 | 2.1 | 0.657 |
| walker |  | 554 | 27 | entry item body at src/main.rs:27 body 53 |  |  | 0.657 |
| walker |  | 584 | 30 | entry item body at src/main.rs:27 body 28 |  |  | 0.659 |
| walker |  | 615 | 31 | entry item body at src/main.rs:27 body 57 |  |  | 0.659 |
| walker |  | 650 | 35 | entry item body at src/main.rs:27 body 47 |  |  | 0.662 |
| ns | 653 |  | 119 | Parser trait method signatures | 2.3 |  | 0.624 |
| walker |  | 693 | 43 | entry item body at src/main.rs:111 body 112 |  |  | 0.624 |
| walker |  | 743 | 50 | entry item body at src/main.rs:27 body 32 |  |  | 0.628 |
| walker |  | 793 | 50 | entry item body at src/main.rs:27 body 103 |  |  | 0.629 |
| walker |  | 813 | 20 | pub item at src/clipboard.rs:28 |  |  | 0.629 |
| ns | 836 |  | 183 | ContentType::new_parser dispatch | 2.4 |  | 0.572 |
| walker |  | 848 | 35 | listing of 'src/ui' |  |  | 0.574 |
| walker |  | 866 | 18 | pub item at src/ui/mod.rs:35 |  |  | 0.574 |
| walker |  | 909 | 43 | listing of 'src/parse' |  |  | 0.576 |
| walker |  | 930 | 21 | pub-item names surface in src/parse/mod.rs |  |  | 0.579 |
| ns | 931 |  | 95 | Tree struct fields | 2.5 |  | 0.543 |
| walker |  | 958 | 28 | impl method sigs in src/parse/mod.rs |  |  | 0.546 |
| walker |  | 998 | 40 | pub item at src/edit.rs:10 |  |  | 0.546 |
| walker |  | 1089 | 91 | pub-item names surface in src/config/mod.rs |  |  | 0.546 |
| walker |  | 1116 | 27 | pub item at src/config/mod.rs:100 |  |  | 0.546 |
| ns | 1144 |  | 213 | ItemValue + FieldType | 2.6 |  | 0.477 |
| walker |  | 1157 | 41 | pub item at src/config/mod.rs:83 |  |  | 0.477 |
| walker |  | 1209 | 52 | pub item at src/config/mod.rs:91 |  |  | 0.477 |
| ns | 1215 |  | 71 | Tree public fn signatures | 2.7 |  | 0.462 |
| walker |  | 1264 | 55 | pub item at src/config/mod.rs:53 |  |  | 0.462 |
| walker |  | 1321 | 57 | pub item at src/config/mod.rs:74 |  |  | 0.462 |
| walker |  | 1400 | 79 | pub item at src/config/mod.rs:62 |  |  | 0.463 |
| ns | 1432 |  | 217 | main.rs run() — config + flag dispatch | 2.8 |  | 0.515 |
| walker |  | 1482 | 82 | pub item at src/config/mod.rs:106 |  |  | 0.516 |
| walker |  | 1564 | 82 | pub item at src/config/mod.rs:118 |  |  | 0.517 |
| walker |  | 1598 | 34 | pub-item names surface in src/debug.rs |  |  | 0.517 |
| walker |  | 1598 | 0 | pub item at src/debug.rs:19 |  |  | 0.517 |
| walker |  | 1598 | 0 | pub item at src/debug.rs:23 |  |  | 0.517 |
| walker |  | 1607 | 9 | pub item body at src/debug.rs:19 body 20 |  |  | 0.517 |
| walker |  | 1707 | 100 | entry item body at src/main.rs:27 body 94 |  |  | 0.519 |
| walker |  | 1728 | 21 | pub-item names surface in src/ui/header.rs |  |  | 0.519 |
| walker |  | 1751 | 23 | pub item at src/ui/header.rs:41 |  |  | 0.519 |
| ns | 1755 |  | 323 | main.rs run() — content type + data read | 2.9 |  | 0.484 |
| walker |  | 1772 | 21 | pub-item names surface in src/ui/popup.rs |  |  | 0.484 |
| ns | 1929 |  | 174 | main.rs run() — --to short-circuit | 2.10 |  | 0.463 |
| walker |  | 1933 | 161 | pub item at src/parse/mod.rs:18 |  |  | 0.526 |
| walker |  | 1998 | 65 | headings outline in README.md |  |  | 0.548 |
| walker |  | 2027 | 29 | pub item at src/ui/popup.rs:13 |  |  | 0.548 |
| walker |  | 2051 | 24 | pub-item names surface in src/ui/footer.rs |  |  | 0.548 |
| walker |  | 2065 | 14 | pub item at src/ui/footer.rs:16 |  |  | 0.548 |
| walker |  | 2099 | 34 | pub item at src/ui/footer.rs:10 |  |  | 0.549 |
| walker |  | 2129 | 30 | pub item at src/parse/any.rs:9 |  |  | 0.549 |
| ns | 2171 |  | 242 | main.rs run() — size check, Tree, App, ui::start | 2.11 |  | 0.562 |
| walker |  | 2175 | 46 | pub-item names surface in src/tree.rs |  |  | 0.565 |
| walker |  | 2201 | 26 | pub item at src/tree.rs:36 |  |  | 0.570 |
| ns | 2206 |  | 35 | ui/ module layout | 3.1 |  | 0.583 |
| walker |  | 2246 | 45 | pub item at src/tree.rs:42 |  |  | 0.597 |
| walker |  | 2308 | 62 | pub item at src/tree.rs:25 |  |  | 0.633 |
| walker |  | 2343 | 35 | pub item at src/ui/popup.rs:18 |  |  | 0.633 |
| walker |  | 2412 | 69 | pub item at src/live_reload.rs:16 |  |  | 0.633 |
| walker |  | 2443 | 31 | pub-item names surface in src/config/keys.rs |  |  | 0.634 |
| walker |  | 2467 | 24 | pub item at src/config/keys.rs:351 |  |  | 0.634 |
| ns | 2495 |  | 289 | App struct fields | 3.2 |  | 0.585 |
| walker |  | 2537 | 70 | pub item at src/tree.rs:14 |  |  | 0.608 |
| walker |  | 2701 | 164 | entry item body at src/main.rs:27 body 79 |  |  | 0.643 |
| walker |  | 2733 | 32 | pub-item names surface in src/ui/app.rs |  |  | 0.643 |
| ns | 2739 |  | 244 | ElementInFocus + Refresh + ShowResult | 3.3 |  | 0.604 |
| walker |  | 2750 | 17 | pub item at src/ui/app.rs:48 |  |  | 0.605 |
| walker |  | 2771 | 21 | pub item at src/ui/app.rs:86 |  |  | 0.608 |
| ns | 3020 |  | 281 | ui::start event loop | 3.4 |  | 0.576 |
| walker |  | 3074 | 303 | pub item at src/parse/mod.rs:36 |  |  | 0.599 |
| ns | 3205 |  | 185 | App method names | 3.5 |  | 0.577 |
| walker |  | 3265 | 191 | entry item body at src/main.rs:27 body 59 |  |  | 0.633 |
| walker |  | 3313 | 48 | pub item at src/ui/header.rs:11 |  |  | 0.634 |
| walker |  | 3526 | 213 | mod/use plumbing in src/main.rs |  |  | 0.670 |
| ns | 3560 |  | 355 | TreeOverview struct + impl method names | 3.6 |  | 0.634 |
| walker |  | 3567 | 41 | pub-item names surface in src/parse/json.rs |  |  | 0.634 |
| walker |  | 3567 | 0 | pub item at src/parse/json.rs:26 |  |  | 0.634 |
| walker |  | 3610 | 43 | pub-item names surface in src/ui/filter.rs |  |  | 0.634 |
| walker |  | 3634 | 24 | pub item at src/ui/filter.rs:34 |  |  | 0.634 |
| walker |  | 3665 | 31 | pub item at src/ui/filter.rs:20 |  |  | 0.634 |
| walker |  | 3700 | 35 | pub item at src/ui/filter.rs:27 |  |  | 0.635 |
| walker |  | 3747 | 47 | pub item at src/ui/filter.rs:13 |  |  | 0.636 |
| ns | 3838 |  | 278 | TreeOverview::on_key action dispatch | 3.7 | 3.6 | 0.616 |
| walker |  | 4019 | 272 | pub item at src/config/mod.rs:17 |  |  | 0.618 |
| walker |  | 4079 | 60 | pub-item names surface in src/parse/syntax.rs |  |  | 0.618 |
| walker |  | 4079 | 0 | pub item at src/parse/syntax.rs:139 |  |  | 0.618 |
| walker |  | 4079 | 0 | pub item at src/parse/syntax.rs:201 |  |  | 0.618 |
| ns | 4084 |  | 246 | DataBlock struct + method names | 3.8 |  | 0.595 |
| walker |  | 4101 | 22 | pub item at src/parse/syntax.rs:151 |  |  | 0.596 |
| walker |  | 4194 | 93 | pub item at src/parse/syntax.rs:11 |  |  | 0.597 |
| walker |  | 4309 | 115 | mod/use plumbing in src/parse/mod.rs |  |  | 0.597 |
| ns | 4359 |  | 275 | Filter widget surface | 3.9 |  | 0.595 |
| walker |  | 4418 | 109 | pub item at src/config/keys.rs:54 |  |  | 0.596 |
| walker |  | 4537 | 119 | mod/use plumbing in src/config/mod.rs |  |  | 0.596 |
| walker |  | 4612 | 75 | pub-item names surface in src/config/colors.rs |  |  | 0.596 |
| walker |  | 4641 | 29 | pub item at src/config/colors.rs:324 |  |  | 0.596 |
| ns | 4672 |  | 313 | Footer / Header / Popup surfaces | 3.10 |  | 0.605 |
| walker |  | 4706 | 65 | pub item at src/config/colors.rs:251 |  |  | 0.605 |
| ns | 4715 |  | 43 | parse/ module layout | 4.1 |  | 0.612 |
| walker |  | 4793 | 87 | pub item at src/config/colors.rs:286 |  |  | 0.612 |
| ns | 4844 |  | 129 | SyntaxToken enum | 4.2 |  | 0.616 |
| walker |  | 4885 | 92 | pub item at src/config/colors.rs:342 |  |  | 0.616 |
| walker |  | 4959 | 74 | README.md section #1 |  |  | 0.616 |
| walker |  | 5050 | 91 | macro_export body at src/debug.rs:8 |  |  | 0.616 |
| walker |  | 5174 | 124 | pub item at src/ui/data_block.rs:16 |  |  | 0.627 |
| ns | 5287 |  | 443 | Per-parser Parser impl headers | 4.3 |  | 0.594 |
| walker |  | 5362 | 188 | mod/use plumbing in src/ui/mod.rs |  |  | 0.594 |
| walker |  | 5463 | 101 | pub item body at src/debug.rs:23 body 24 |  |  | 0.594 |
| ns | 5491 |  | 204 | Parser::parse_root default body | 4.4 | 2.3 | 0.603 |
| walker |  | 5617 | 154 | pub item at src/ui/tree_overview.rs:19 |  |  | 0.612 |
| walker |  | 5780 | 163 | pub item at src/config/types.rs:28 |  |  | 0.612 |
| walker |  | 5788 | 8 | listing of 'config/themes' |  |  | 0.612 |
| ns | 5796 |  | 305 | AnyParser auto-detect parse_root | 4.5 |  | 0.594 |
| ns | 5872 |  | 76 | syntax helper signatures | 4.6 |  | 0.597 |
| ns | 5888 |  | 16 | config/ module layout | 5.1 |  | 0.599 |
| walker |  | 5974 | 186 | pub item at src/config/colors.rs:21 |  |  | 0.600 |
| ns | 6234 |  | 346 | Config struct field names | 5.2 |  | 0.605 |
| walker |  | 6399 | 425 | impl method sigs in src/config/mod.rs |  |  | 0.605 |
| ns | 6578 |  | 344 | Action enum variants | 5.3 |  | 0.589 |
| walker |  | 6633 | 234 | pub item at src/ui/app.rs:53 |  |  | 0.616 |
| walker |  | 6729 | 96 | README.md section #6 |  |  | 0.616 |
| walker |  | 6829 | 100 | pub item body at src/parse/syntax.rs:201 body 202 |  |  | 0.616 |
| ns | 6976 |  | 398 | Default key bindings | 5.4 |  | 0.600 |
| walker |  | 7088 | 259 | pub item at src/config/colors.rs:78 |  |  | 0.601 |
| ns | 7174 |  | 198 | Key enum + KeyAction | 5.5 |  | 0.603 |
| walker |  | 7409 | 321 | [dependencies] in Cargo.toml |  |  | 0.603 |
| ns | 7644 |  | 470 | Per-branch struct fields (Tree / Layout / Filter / Data) | 5.6 |  | 0.607 |
| walker |  | 7704 | 295 | pub item at src/config/colors.rs:162 |  |  | 0.608 |
| walker |  | 7822 | 118 | pub item body at src/parse/syntax.rs:139 body 140 |  |  | 0.608 |
| walker |  | 7878 | 56 | README.md section #3 |  |  | 0.608 |
| ns | 7894 |  | 250 | Editor + Header + Footer struct fields | 5.7 |  | 0.608 |
| walker |  | 8065 | 187 | README.md section #4 |  |  | 0.608 |
| walker |  | 8135 | 70 | README.md section #2 |  |  | 0.608 |
| ns | 8331 |  | 437 | Config::load + Config::parse | 5.8 |  | 0.592 |
| ns | 8613 |  | 282 | Config::get_path resolution | 5.9 |  | 0.583 |
| ns | 8986 |  | 373 | Color struct + Colors namespaces | 5.10 |  | 0.587 |
| walker |  | 9051 | 916 | pub item at src/cmd.rs:13 |  |  | 0.588 |
| walker |  | 9059 | 8 | listing of '.github' |  |  | 0.588 |
| walker |  | 9075 | 16 | listing of '.github/workflows' |  |  | 0.588 |
| ns | 9095 |  | 109 | DataColors fields | 5.11 |  | 0.592 |
| ns | 9221 |  | 126 | TreeColors fields | 5.12 |  | 0.596 |
| ns | 9311 |  | 90 | Types config struct | 5.13 |  | 0.597 |
| ns | 9599 |  | 288 | CommandArgs flag list | 6.1 |  | 0.605 |
| ns | 9652 |  | 53 | examples/ + docs/ listings | 6.2 |  | 0.602 |
| ns | 9808 |  | 156 | Changelog version headings | 6.3 |  | 0.597 |
| walker |  | 9945 | 870 | pub item at src/config/keys.rs:195 |  |  | 0.597 |
| walker |  | 9952 | 7 | pub item body at src/ui/mod.rs:35 body 60 |  |  | 0.597 |
| walker |  | 9959 | 7 | pub item body at src/parse/json.rs:26 body 80 |  |  | 0.597 |
| ns | 9967 |  | 159 | clipboard OS routing | 6.4 |  | 0.593 |
| walker |  | 9969 | 10 | pub item body at src/ui/mod.rs:35 body 59 |  |  | 0.593 |

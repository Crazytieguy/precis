Score(3000)=0.512 I=0.795 C=0.329 ns_rows≤3K=20/50 (reached=7 partial=3 missing=10)

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
| ns | 329 |  | 144 | Cargo package metadata | 1.6 |  | 0.789 |
| walker |  | 357 | 43 | entry item body at src/main.rs:111 body 112 |  |  | 0.789 |
| walker |  | 377 | 20 | pub item at src/clipboard.rs:28 |  |  | 0.789 |
| ns | 391 |  | 62 | ContentType variants — locations only | 2.1 |  | 0.721 |
| walker |  | 412 | 35 | listing of 'src/ui' |  |  | 0.724 |
| walker |  | 430 | 18 | pub item at src/ui/mod.rs:35 |  |  | 0.724 |
| walker |  | 473 | 43 | listing of 'src/parse' |  |  | 0.726 |
| walker |  | 494 | 21 | pub-item names surface in src/parse/mod.rs |  |  | 0.730 |
| walker |  | 522 | 28 | impl method sigs in src/parse/mod.rs |  |  | 0.730 |
| ns | 534 |  | 143 | ContentType enum context | 2.2 | 2.1 | 0.663 |
| walker |  | 562 | 40 | pub item at src/edit.rs:10 |  |  | 0.663 |
| walker |  | 653 | 91 | pub-item names surface in src/config/mod.rs |  |  | 0.626 |
| ns | 653 |  | 119 | Parser trait method signatures | 2.3 |  | 0.626 |
| walker |  | 680 | 27 | pub item at src/config/mod.rs:100 |  |  | 0.626 |
| walker |  | 721 | 41 | pub item at src/config/mod.rs:83 |  |  | 0.626 |
| walker |  | 773 | 52 | pub item at src/config/mod.rs:91 |  |  | 0.626 |
| walker |  | 828 | 55 | pub item at src/config/mod.rs:53 |  |  | 0.626 |
| ns | 836 |  | 183 | ContentType::new_parser dispatch | 2.4 |  | 0.571 |
| walker |  | 885 | 57 | pub item at src/config/mod.rs:74 |  |  | 0.572 |
| ns | 931 |  | 95 | Tree struct fields | 2.5 |  | 0.536 |
| walker |  | 964 | 79 | pub item at src/config/mod.rs:62 |  |  | 0.537 |
| walker |  | 1046 | 82 | pub item at src/config/mod.rs:106 |  |  | 0.538 |
| walker |  | 1128 | 82 | pub item at src/config/mod.rs:118 |  |  | 0.539 |
| ns | 1144 |  | 213 | ItemValue + FieldType | 2.6 |  | 0.470 |
| walker |  | 1162 | 34 | pub-item names surface in src/debug.rs |  |  | 0.470 |
| walker |  | 1162 | 0 | pub item at src/debug.rs:19 |  |  | 0.470 |
| walker |  | 1162 | 0 | pub item at src/debug.rs:23 |  |  | 0.470 |
| walker |  | 1171 | 9 | pub item body at src/debug.rs:19 body 20 |  |  | 0.470 |
| walker |  | 1192 | 21 | pub-item names surface in src/ui/header.rs |  |  | 0.471 |
| walker |  | 1215 | 23 | pub item at src/ui/header.rs:41 |  |  | 0.455 |
| ns | 1215 |  | 71 | Tree public fn signatures | 2.7 |  | 0.455 |
| walker |  | 1236 | 21 | pub-item names surface in src/ui/popup.rs |  |  | 0.456 |
| walker |  | 1397 | 161 | pub item at src/parse/mod.rs:18 |  |  | 0.545 |
| ns | 1432 |  | 217 | main.rs run() — config + flag dispatch | 2.8 |  | 0.495 |
| walker |  | 1462 | 65 | headings outline in README.md |  |  | 0.522 |
| walker |  | 1491 | 29 | pub item at src/ui/popup.rs:13 |  |  | 0.522 |
| walker |  | 1515 | 24 | pub-item names surface in src/ui/footer.rs |  |  | 0.522 |
| walker |  | 1529 | 14 | pub item at src/ui/footer.rs:16 |  |  | 0.522 |
| walker |  | 1563 | 34 | pub item at src/ui/footer.rs:10 |  |  | 0.523 |
| walker |  | 1593 | 30 | pub item at src/parse/any.rs:9 |  |  | 0.523 |
| walker |  | 1639 | 46 | pub-item names surface in src/tree.rs |  |  | 0.527 |
| walker |  | 1665 | 26 | pub item at src/tree.rs:36 |  |  | 0.534 |
| walker |  | 1710 | 45 | pub item at src/tree.rs:42 |  |  | 0.556 |
| ns | 1755 |  | 323 | main.rs run() — content type + data read | 2.9 |  | 0.505 |
| walker |  | 1772 | 62 | pub item at src/tree.rs:25 |  |  | 0.552 |
| walker |  | 1807 | 35 | pub item at src/ui/popup.rs:18 |  |  | 0.552 |
| walker |  | 1876 | 69 | pub item at src/live_reload.rs:16 |  |  | 0.552 |
| walker |  | 1907 | 31 | pub-item names surface in src/config/keys.rs |  |  | 0.552 |
| ns | 1929 |  | 174 | main.rs run() — --to short-circuit | 2.10 |  | 0.528 |
| walker |  | 1931 | 24 | pub item at src/config/keys.rs:351 |  |  | 0.529 |
| walker |  | 2001 | 70 | pub item at src/tree.rs:14 |  |  | 0.559 |
| walker |  | 2033 | 32 | pub-item names surface in src/ui/app.rs |  |  | 0.559 |
| walker |  | 2050 | 17 | pub item at src/ui/app.rs:48 |  |  | 0.559 |
| walker |  | 2071 | 21 | pub item at src/ui/app.rs:86 |  |  | 0.559 |
| ns | 2171 |  | 242 | main.rs run() — size check, Tree, App, ui::start | 2.11 |  | 0.534 |
| ns | 2206 |  | 35 | ui/ module layout | 3.1 |  | 0.549 |
| walker |  | 2374 | 303 | pub item at src/parse/mod.rs:36 |  |  | 0.579 |
| walker |  | 2422 | 48 | pub item at src/ui/header.rs:11 |  |  | 0.580 |
| walker |  | 2463 | 41 | pub-item names surface in src/parse/json.rs |  |  | 0.580 |
| walker |  | 2463 | 0 | pub item at src/parse/json.rs:26 |  |  | 0.580 |
| ns | 2495 |  | 289 | App struct fields | 3.2 |  | 0.536 |
| walker |  | 2506 | 43 | pub-item names surface in src/ui/filter.rs |  |  | 0.536 |
| walker |  | 2530 | 24 | pub item at src/ui/filter.rs:34 |  |  | 0.536 |
| walker |  | 2561 | 31 | pub item at src/ui/filter.rs:20 |  |  | 0.536 |
| walker |  | 2596 | 35 | pub item at src/ui/filter.rs:27 |  |  | 0.537 |
| walker |  | 2643 | 47 | pub item at src/ui/filter.rs:13 |  |  | 0.538 |
| ns | 2739 |  | 244 | ElementInFocus + Refresh + ShowResult | 3.3 |  | 0.509 |
| walker |  | 2915 | 272 | pub item at src/config/mod.rs:17 |  |  | 0.511 |
| walker |  | 2975 | 60 | pub-item names surface in src/parse/syntax.rs |  |  | 0.511 |
| walker |  | 2975 | 0 | pub item at src/parse/syntax.rs:139 |  |  | 0.511 |
| walker |  | 2975 | 0 | pub item at src/parse/syntax.rs:201 |  |  | 0.511 |
| walker |  | 2997 | 22 | pub item at src/parse/syntax.rs:151 |  |  | 0.512 |
| ns | 3020 |  | 281 | ui::start event loop | 3.4 |  | 0.485 |
| walker |  | 3090 | 93 | pub item at src/parse/syntax.rs:11 |  |  | 0.486 |
| walker |  | 3205 | 115 | mod/use plumbing in src/parse/mod.rs |  |  | 0.469 |
| ns | 3205 |  | 185 | App method names | 3.5 |  | 0.469 |
| walker |  | 3314 | 109 | pub item at src/config/keys.rs:54 |  |  | 0.470 |
| walker |  | 3433 | 119 | mod/use plumbing in src/config/mod.rs |  |  | 0.470 |
| walker |  | 3508 | 75 | pub-item names surface in src/config/colors.rs |  |  | 0.470 |
| walker |  | 3537 | 29 | pub item at src/config/colors.rs:324 |  |  | 0.470 |
| ns | 3560 |  | 355 | TreeOverview struct + impl method names | 3.6 |  | 0.444 |
| walker |  | 3602 | 65 | pub item at src/config/colors.rs:251 |  |  | 0.444 |
| walker |  | 3689 | 87 | pub item at src/config/colors.rs:286 |  |  | 0.444 |
| walker |  | 3781 | 92 | pub item at src/config/colors.rs:342 |  |  | 0.444 |
| ns | 3838 |  | 278 | TreeOverview::on_key action dispatch | 3.7 | 3.6 | 0.431 |
| walker |  | 3855 | 74 | README.md section #1 |  |  | 0.431 |
| walker |  | 3946 | 91 | macro_export body at src/debug.rs:8 |  |  | 0.431 |
| ns | 4084 |  | 246 | DataBlock struct + method names | 3.8 |  | 0.415 |
| walker |  | 4159 | 213 | mod/use plumbing in src/main.rs |  |  | 0.447 |
| walker |  | 4283 | 124 | pub item at src/ui/data_block.rs:16 |  |  | 0.464 |
| ns | 4359 |  | 275 | Filter widget surface | 3.9 |  | 0.473 |
| walker |  | 4471 | 188 | mod/use plumbing in src/ui/mod.rs |  |  | 0.473 |
| walker |  | 4572 | 101 | pub item body at src/debug.rs:23 body 24 |  |  | 0.473 |
| ns | 4672 |  | 313 | Footer / Header / Popup surfaces | 3.10 |  | 0.492 |
| ns | 4715 |  | 43 | parse/ module layout | 4.1 |  | 0.503 |
| walker |  | 4726 | 154 | pub item at src/ui/tree_overview.rs:19 |  |  | 0.514 |
| ns | 4844 |  | 129 | SyntaxToken enum | 4.2 |  | 0.521 |
| walker |  | 4889 | 163 | pub item at src/config/types.rs:28 |  |  | 0.522 |
| walker |  | 4897 | 8 | listing of 'config/themes' |  |  | 0.522 |
| walker |  | 5083 | 186 | pub item at src/config/colors.rs:21 |  |  | 0.523 |
| ns | 5287 |  | 443 | Per-parser Parser impl headers | 4.3 |  | 0.495 |
| ns | 5491 |  | 204 | Parser::parse_root default body | 4.4 | 2.3 | 0.509 |
| walker |  | 5508 | 425 | impl method sigs in src/config/mod.rs |  |  | 0.509 |
| walker |  | 5742 | 234 | pub item at src/ui/app.rs:53 |  |  | 0.543 |
| ns | 5796 |  | 305 | AnyParser auto-detect parse_root | 4.5 |  | 0.528 |
| walker |  | 5838 | 96 | README.md section #6 |  |  | 0.528 |
| ns | 5872 |  | 76 | syntax helper signatures | 4.6 |  | 0.531 |
| ns | 5888 |  | 16 | config/ module layout | 5.1 |  | 0.534 |
| walker |  | 5938 | 100 | pub item body at src/parse/syntax.rs:201 body 202 |  |  | 0.534 |
| walker |  | 6197 | 259 | pub item at src/config/colors.rs:78 |  |  | 0.535 |
| ns | 6234 |  | 346 | Config struct field names | 5.2 |  | 0.543 |
| walker |  | 6518 | 321 | [dependencies] in Cargo.toml |  |  | 0.543 |
| ns | 6578 |  | 344 | Action enum variants | 5.3 |  | 0.528 |
| walker |  | 6813 | 295 | pub item at src/config/colors.rs:162 |  |  | 0.529 |
| walker |  | 6931 | 118 | pub item body at src/parse/syntax.rs:139 body 140 |  |  | 0.529 |
| ns | 6976 |  | 398 | Default key bindings | 5.4 |  | 0.515 |
| walker |  | 7118 | 187 | README.md section #4 |  |  | 0.515 |
| ns | 7174 |  | 198 | Key enum + KeyAction | 5.5 |  | 0.520 |
| ns | 7644 |  | 470 | Per-branch struct fields (Tree / Layout / Filter / Data) | 5.6 |  | 0.528 |
| ns | 7894 |  | 250 | Editor + Header + Footer struct fields | 5.7 |  | 0.531 |
| walker |  | 8034 | 916 | pub item at src/cmd.rs:13 |  |  | 0.533 |
| walker |  | 8090 | 56 | README.md section #3 |  |  | 0.533 |
| walker |  | 8098 | 8 | listing of '.github' |  |  | 0.533 |
| walker |  | 8114 | 16 | listing of '.github/workflows' |  |  | 0.533 |
| ns | 8331 |  | 437 | Config::load + Config::parse | 5.8 |  | 0.519 |
| ns | 8613 |  | 282 | Config::get_path resolution | 5.9 |  | 0.511 |
| walker |  | 8984 | 870 | pub item at src/config/keys.rs:195 |  |  | 0.511 |
| ns | 8986 |  | 373 | Color struct + Colors namespaces | 5.10 |  | 0.517 |
| walker |  | 9054 | 70 | README.md section #2 |  |  | 0.517 |
| walker |  | 9061 | 7 | pub item body at src/ui/mod.rs:35 body 60 |  |  | 0.517 |
| walker |  | 9068 | 7 | pub item body at src/parse/json.rs:26 body 80 |  |  | 0.517 |
| walker |  | 9078 | 10 | pub item body at src/ui/mod.rs:35 body 59 |  |  | 0.517 |
| ns | 9095 |  | 109 | DataColors fields | 5.11 |  | 0.522 |
| ns | 9221 |  | 126 | TreeColors fields | 5.12 |  | 0.528 |
| ns | 9311 |  | 90 | Types config struct | 5.13 |  | 0.529 |
| ns | 9599 |  | 288 | CommandArgs flag list | 6.1 |  | 0.539 |
| ns | 9652 |  | 53 | examples/ + docs/ listings | 6.2 |  | 0.536 |
| walker |  | 9653 | 575 | README.md section #5 |  |  | 0.536 |
| walker |  | 9664 | 11 | pub item body at src/ui/mod.rs:35 body 36 |  |  | 0.536 |
| walker |  | 9675 | 11 | pub item body at src/ui/mod.rs:35 body 58 |  |  | 0.536 |
| walker |  | 9686 | 11 | pub item body at src/parse/json.rs:26 body 27 |  |  | 0.536 |
| walker |  | 9701 | 15 | pub item body at src/ui/mod.rs:35 body 37 |  |  | 0.537 |
| walker |  | 9714 | 13 | pub item body at src/parse/json.rs:26 body 78 |  |  | 0.537 |
| ns | 9808 |  | 156 | Changelog version headings | 6.3 |  | 0.533 |
| ns | 9967 |  | 159 | clipboard OS routing | 6.4 |  | 0.529 |

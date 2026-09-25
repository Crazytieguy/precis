Score(3000)=0.447 I=0.790 C=0.253 ns_rows≤3K=19/44 grid(1000/1442/2080/3000/4327/6240/9000)=0.656/0.593/0.549/0.447/0.490/0.474/0.522

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 35 | 35 | listing of '.' |  |  | 0.000 |
| walker |  | 39 | 4 | listing of 'assets' |  |  | 0.000 |
| ns | 44 |  | 44 | Repository identity — README title and one-line description | 1.1 |  | 0.000 |
| walker |  | 47 | 8 | listing of 'config' |  |  | 0.000 |
| walker |  | 62 | 15 | listing of 'docs' |  |  | 0.000 |
| walker |  | 70 | 8 | listing of 'config/themes' |  |  | 0.000 |
| ns | 79 |  | 35 | Complete repository root listing | 1.2 |  | 0.697 |
| walker |  | 108 | 38 | listing of 'src' |  |  | 0.785 |
| ns | 117 |  | 38 | Complete src/ listing — the module roster | 1.3 |  | 0.754 |
| walker |  | 124 | 16 | listing of 'src/config' |  |  | 0.756 |
| walker |  | 159 | 35 | listing of 'src/ui' |  |  | 0.785 |
| walker |  | 203 | 44 | README headline in README.md |  |  | 1.000 |
| ns | 211 |  | 94 | Complete listings of src/ui, src/parse, src/config | 1.4 | 1.3 | 0.783 |
| walker |  | 246 | 43 | listing of 'src/parse' |  |  | 1.000 |
| walker |  | 254 | 8 | listing of '.github' |  |  | 1.000 |
| ns | 268 |  | 57 | Cargo package identity — name, version, description | 1.5 |  | 0.967 |
| walker |  | 270 | 16 | listing of '.github/workflows' |  |  | 0.967 |
| ns | 303 |  | 35 | All README H2 section headings | 1.6 |  | 0.923 |
| walker |  | 359 | 89 | rust names src/main.rs |  |  | 0.924 |
| walker |  | 380 | 21 | rust names src/edit.rs |  |  | 0.924 |
| walker |  | 403 | 23 | rust names src/cmd.rs |  |  | 0.924 |
| ns | 411 |  | 108 | README Usage — how the binary is invoked | 1.7 | 1.6 | 0.833 |
| ns | 508 |  | 97 | README pointers to config file and reference docs | 1.8 | 1.7 | 0.796 |
| walker |  | 544 | 141 | [package] in Cargo.toml |  |  | 0.827 |
| walker |  | 574 | 30 | rust decl src/edit.rs:10 |  |  | 0.827 |
| ns | 583 |  | 75 | Complete listings of docs/, config/, config/themes/, examples/, assets/ | 1.9 | 1.2 | 0.751 |
| walker |  | 662 | 88 | rust names build.rs |  |  | 0.751 |
| walker |  | 700 | 38 | rust names src/clipboard.rs |  |  | 0.751 |
| ns | 722 |  | 139 | Cargo [dependencies] — first half | 1.10 |  | 0.699 |
| walker |  | 763 | 63 | headings outline in README.md |  |  | 0.731 |
| walker |  | 786 | 23 | rust names src/ui/data_block.rs |  |  | 0.731 |
| walker |  | 809 | 23 | rust names src/ui/tree_overview.rs |  |  | 0.731 |
| walker |  | 834 | 25 | rust names src/config/types.rs |  |  | 0.731 |
| walker |  | 884 | 50 | rust decl src/edit.rs:16 |  |  | 0.731 |
| ns | 909 |  | 187 | Cargo [dependencies] tail plus [build-dependencies] | 1.11 | 1.10 | 0.677 |
| walker |  | 941 | 57 | rust names src/live_reload.rs |  |  | 0.677 |
| ns | 982 |  | 73 | main.rs entry points — run() signature and main() | 2.1 |  | 0.656 |
| walker |  | 993 | 52 | rust decl src/live_reload.rs:27 |  |  | 0.656 |
| walker |  | 1023 | 30 | rust names src/parse/jsonl.rs |  |  | 0.656 |
| walker |  | 1055 | 32 | rust names src/ui/popup.rs |  |  | 0.656 |
| walker |  | 1084 | 29 | rust decl src/ui/popup.rs:13 |  |  | 0.656 |
| walker |  | 1144 | 60 | rust decl src/cmd.rs:111 |  |  | 0.656 |
| ns | 1188 |  | 206 | run(): CLI parse, config resolution, --show-config, --debug | 2.2 | 2.1 | 0.592 |
| walker |  | 1211 | 67 | rust names src/tree.rs |  |  | 0.592 |
| walker |  | 1251 | 40 | rust decl src/tree.rs:35 |  |  | 0.592 |
| walker |  | 1306 | 55 | rust decl src/tree.rs:41 |  |  | 0.593 |
| walker |  | 1341 | 35 | rust names src/parse/any.rs |  |  | 0.593 |
| walker |  | 1360 | 19 | rust decl src/parse/any.rs:9 |  |  | 0.593 |
| walker |  | 1395 | 35 | rust names src/ui/footer.rs |  |  | 0.593 |
| walker |  | 1409 | 14 | rust decl src/ui/footer.rs:16 |  |  | 0.593 |
| walker |  | 1478 | 69 | rust names src/debug.rs |  |  | 0.593 |
| ns | 1511 |  | 323 | run(): content-type inference, live-reload watcher, reading path or stdin | 2.3 | 2.2 | 0.534 |
| walker |  | 1512 | 34 | rust decl src/ui/footer.rs:10 |  |  | 0.534 |
| walker |  | 1523 | 11 | rust body src/debug.rs:19 |  |  | 0.534 |
| walker |  | 1591 | 68 | rust decl src/tree.rs:385 |  |  | 0.534 |
| walker |  | 1663 | 72 | rust decl src/tree.rs:25 |  |  | 0.535 |
| walker |  | 1736 | 73 | rust decl src/live_reload.rs:16 |  |  | 0.535 |
| walker |  | 1775 | 39 | rust decl src/parse/any.rs:13 |  |  | 0.535 |
| ns | 1790 |  | 279 | run(): the --to conversion short-circuit and the data-size guard | 2.4 | 2.3 | 0.504 |
| walker |  | 1817 | 42 | rust names src/ui/header.rs |  |  | 0.504 |
| walker |  | 1840 | 23 | rust decl src/ui/header.rs:41 |  |  | 0.504 |
| walker |  | 1872 | 32 | rust decl src/ui/header.rs:18 |  |  | 0.504 |
| walker |  | 1912 | 40 | listing of 'examples' |  |  | 0.568 |
| ns | 1925 |  | 135 | run(): tree construction, App::new, header context, ui::start | 2.5 | 2.4 | 0.547 |
| walker |  | 1997 | 85 | rust decl src/tree.rs:14 |  |  | 0.549 |
| walker |  | 2042 | 45 | rust decl src/ui/popup.rs:18 |  |  | 0.549 |
| walker |  | 2090 | 48 | rust decl src/ui/header.rs:11 |  |  | 0.550 |
| walker |  | 2140 | 50 | rust decl src/tree.rs:394 |  |  | 0.550 |
| walker |  | 2192 | 52 | rust decl src/live_reload.rs:28 |  |  | 0.550 |
| ns | 2233 |  | 308 | ui::start — the draw/edit/quit loop and terminal restoration | 2.6 |  | 0.507 |
| walker |  | 2243 | 51 | rust decl src/ui/footer.rs:20 |  |  | 0.507 |
| walker |  | 2342 | 99 | rust decl src/debug.rs:7 |  |  | 0.507 |
| walker |  | 2394 | 52 | rust decl src/ui/header.rs:46 |  |  | 0.507 |
| walker |  | 2452 | 58 | rust names src/parse/hcl.rs |  |  | 0.507 |
| walker |  | 2510 | 58 | rust names src/parse/json.rs |  |  | 0.507 |
| walker |  | 2569 | 59 | rust names src/parse/yaml.rs |  |  | 0.507 |
| walker |  | 2601 | 32 | rust decl src/live_reload.rs:93 |  |  | 0.507 |
| ns | 2628 |  | 395 | Complete CommandArgs field roster — every CLI flag with its type | 3.1 |  | 0.472 |
| walker |  | 2664 | 63 | rust names src/parse/toml.rs |  |  | 0.472 |
| walker |  | 2727 | 63 | rust names src/ui/app.rs |  |  | 0.472 |
| walker |  | 2748 | 21 | rust decl src/ui/app.rs:86 |  |  | 0.472 |
| walker |  | 2777 | 29 | rust decl src/ui/app.rs:47 |  |  | 0.472 |
| walker |  | 2893 | 116 | rust decl src/tree.rs:51 |  |  | 0.472 |
| walker |  | 2957 | 64 | rust names src/ui/filter.rs |  |  | 0.472 |
| walker |  | 2982 | 25 | rust decl src/ui/filter.rs:152 |  |  | 0.472 |
| ns | 2983 |  | 355 | The --help text for every flag (CommandArgs doc comments) | 3.2 | 3.1 | 0.447 |
| walker |  | 3013 | 31 | rust decl src/ui/filter.rs:20 |  |  | 0.447 |
| walker |  | 3049 | 36 | rust decl src/ui/filter.rs:33 |  |  | 0.447 |
| walker |  | 3084 | 35 | rust decl src/ui/filter.rs:27 |  |  | 0.447 |
| ns | 3091 |  | 108 | Every short-flag mapping in CommandArgs | 3.3 | 3.1 | 0.439 |
| walker |  | 3131 | 47 | rust decl src/ui/filter.rs:13 |  |  | 0.439 |
| walker |  | 3198 | 67 | rust names src/config/keys.rs |  |  | 0.439 |
| walker |  | 3227 | 29 | rust decl src/config/keys.rs:356 |  |  | 0.439 |
| walker |  | 3263 | 36 | rust decl src/config/keys.rs:350 |  |  | 0.439 |
| walker |  | 3330 | 67 | rust decl src/ui/app.rs:91 |  |  | 0.439 |
| walker |  | 3346 | 16 | rust decl src/ui/app.rs:101 |  |  | 0.439 |
| ns | 3448 |  | 357 | get_content_type in full, plus the name of every other CommandArgs method | 3.4 |  | 0.415 |
| walker |  | 3619 | 273 | [dependencies] in Cargo.toml |  |  | 0.473 |
| ns | 3657 |  | 209 | ContentType — the complete set of supported input formats | 4.1 |  | 0.458 |
| walker |  | 3660 | 41 | rust decl src/ui/app.rs:137 |  |  | 0.458 |
| walker |  | 3785 | 125 | manifest config in Cargo.toml |  |  | 0.458 |
| walker |  | 3875 | 90 | rust decl src/parse/hcl.rs:10 |  |  | 0.458 |
| walker |  | 3882 | 7 | rust body src/parse/hcl.rs:15 |  |  | 0.458 |
| walker |  | 3972 | 90 | rust decl src/parse/toml.rs:11 |  |  | 0.458 |
| ns | 3974 |  | 317 | The Parser trait surface and ContentType::new_parser | 4.2 | 4.1 | 0.441 |
| walker |  | 3979 | 7 | rust body src/parse/toml.rs:16 |  |  | 0.441 |
| walker |  | 4070 | 91 | rust decl src/parse/json.rs:8 |  |  | 0.441 |
| walker |  | 4077 | 7 | rust body src/parse/json.rs:13 |  |  | 0.441 |
| walker |  | 4168 | 91 | rust decl src/parse/yaml.rs:10 |  |  | 0.441 |
| walker |  | 4175 | 7 | rust body src/parse/yaml.rs:15 |  |  | 0.442 |
| walker |  | 4183 | 8 | rust body src/parse/json.rs:9 |  |  | 0.442 |
| walker |  | 4191 | 8 | rust body src/parse/yaml.rs:11 |  |  | 0.442 |
| walker |  | 4284 | 93 | rust decl src/parse/jsonl.rs:9 |  |  | 0.442 |
| walker |  | 4291 | 7 | rust body src/parse/jsonl.rs:14 |  |  | 0.490 |
| ns | 4291 |  | 317 | tree.rs — Tree, ItemValue, HighlightKeyword and FieldType | 4.3 |  | 0.490 |
| walker |  | 4299 | 8 | rust body src/parse/jsonl.rs:10 |  |  | 0.490 |
| walker |  | 4398 | 99 | rust names src/config/colors.rs |  |  | 0.490 |
| walker |  | 4417 | 19 | rust decl src/config/colors.rs:55 |  |  | 0.490 |
| walker |  | 4462 | 45 | rust decl src/config/colors.rs:323 |  |  | 0.490 |
| ns | 4498 |  | 207 | Parser::parse_root — UTF-8 decode and root-shape validation | 4.4 | 4.2 | 0.477 |
| walker |  | 4553 | 91 | rust decl src/config/colors.rs:250 |  |  | 0.477 |
| ns | 4631 |  | 133 | SyntaxToken — the complete highlight token vocabulary | 4.5 |  | 0.466 |
| walker |  | 4661 | 108 | rust names src/parse/mod.rs |  |  | 0.467 |
| walker |  | 4684 | 23 | rust decl src/parse/mod.rs:66 |  |  | 0.467 |
| walker |  | 4810 | 126 | rust names src/config/mod.rs |  |  | 0.467 |
| walker |  | 4853 | 43 | rust decl src/config/mod.rs:99 |  |  | 0.467 |
| ns | 4864 |  | 233 | JSON, JSONL and YAML parsers — types, extensions, array roots allowed | 4.6 | 4.2 | 0.478 |
| walker |  | 4912 | 59 | rust decl src/config/mod.rs:82 |  |  | 0.478 |
| walker |  | 4981 | 69 | rust decl src/config/mod.rs:90 |  |  | 0.479 |
| walker |  | 5057 | 76 | rust decl src/config/mod.rs:52 |  |  | 0.479 |
| ns | 5099 |  | 235 | TOML, XML and HCL parsers — types, extensions, array roots rejected | 4.7 | 4.6 | 0.473 |
| walker |  | 5133 | 76 | rust decl src/config/mod.rs:73 |  |  | 0.473 |
| walker |  | 5229 | 96 | rust decl src/config/mod.rs:117 |  |  | 0.474 |
| ns | 5302 |  | 203 | AnyParser — format auto-detection by trial parsing | 4.8 | 4.7 | 0.464 |
| walker |  | 5330 | 101 | rust decl src/config/mod.rs:61 |  |  | 0.465 |
| walker |  | 5339 | 9 | rust body src/parse/hcl.rs:11 |  |  | 0.467 |
| walker |  | 5348 | 9 | rust body src/parse/toml.rs:12 |  |  | 0.468 |
| walker |  | 5357 | 9 | rust body src/ui/footer.rs:21 |  |  | 0.468 |
| ns | 5432 |  | 130 | Every method on Tree and ItemValue (names only) | 4.9 | 4.3 | 0.469 |
| walker |  | 5461 | 104 | rust decl src/config/mod.rs:105 |  |  | 0.470 |
| walker |  | 5506 | 45 | rust body src/main.rs:111 |  |  | 0.481 |
| ns | 5538 |  | 106 | syntax.rs shared helpers — StringValue, quoting, wrapping, splitting | 4.10 | 4.5 | 0.475 |
| walker |  | 5619 | 113 | rust decl src/config/colors.rs:285 |  |  | 0.475 |
| ns | 5729 |  | 191 | Complete test inventory — every test module and test fn in the crate | 4.11 |  | 0.467 |
| walker |  | 5734 | 115 | rust decl src/config/mod.rs:127 |  |  | 0.467 |
| walker |  | 5856 | 122 | rust names src/parse/xml.rs |  |  | 0.469 |
| ns | 5896 |  | 167 | The Config struct — every configuration section | 5.1 |  | 0.462 |
| walker |  | 5946 | 90 | rust decl src/parse/xml.rs:14 |  |  | 0.468 |
| walker |  | 5953 | 7 | rust body src/parse/xml.rs:19 |  |  | 0.470 |
| walker |  | 5961 | 8 | rust body src/parse/xml.rs:15 |  |  | 0.472 |
| walker |  | 6081 | 120 | rust decl src/config/colors.rs:341 |  |  | 0.472 |
| walker |  | 6202 | 121 | rust decl src/parse/any.rs:25 |  |  | 0.474 |
| walker |  | 6330 | 128 | rust decl src/ui/data_block.rs:16 |  |  | 0.474 |
| ns | 6359 |  | 463 | Every scalar config section struct with its keys | 5.2 | 5.1 | 0.512 |
| walker |  | 6461 | 131 | rust decl src/parse/mod.rs:36 |  |  | 0.520 |
| walker |  | 6600 | 139 | rust names src/parse/syntax.rs |  |  | 0.523 |
| walker |  | 6622 | 22 | rust decl src/parse/syntax.rs:151 |  |  | 0.527 |
| walker |  | 6648 | 26 | rust decl src/parse/syntax.rs:156 |  |  | 0.529 |
| walker |  | 6686 | 38 | rust decl src/parse/syntax.rs:29 |  |  | 0.533 |
| ns | 6707 |  | 348 | generate_actions! — the complete Action vocabulary | 5.3 |  | 0.518 |
| walker |  | 6713 | 27 | rust decl src/parse/syntax.rs:136 |  |  | 0.518 |
| walker |  | 6762 | 49 | rust decl src/parse/syntax.rs:32 |  |  | 0.518 |
| walker |  | 6880 | 118 | rust decl src/parse/syntax.rs:11 |  |  | 0.539 |
| walker |  | 7016 | 136 | rust decl src/ui/filter.rs:40 |  |  | 0.540 |
| walker |  | 7057 | 41 | rust decl src/parse/xml.rs:37 |  |  | 0.540 |
| ns | 7113 |  | 406 | The complete colour schema — groups, per-group fields, and the Color type | 5.4 |  | 0.525 |
| walker |  | 7199 | 142 | rust decl src/ui/popup.rs:26 |  |  | 0.526 |
| walker |  | 7209 | 10 | rust body src/ui/popup.rs:87 |  |  | 0.526 |
| ns | 7236 |  | 123 | The Types section — customisable type labels | 5.5 | 5.1 | 0.521 |
| walker |  | 7352 | 143 | rust decl src/config/keys.rs:53 |  |  | 0.521 |
| ns | 7374 |  | 138 | Every method on Config (names only) | 5.6 | 5.1 | 0.520 |
| walker |  | 7495 | 143 | rust decl src/ui/tree_overview.rs:19 |  |  | 0.520 |
| ns | 7590 |  | 216 | The App control enums — Refresh, ElementInFocus, ScrollDirection | 6.1 |  | 0.511 |
| walker |  | 7680 | 185 | rust names src/ui/mod.rs |  |  | 0.512 |
| walker |  | 7690 | 10 | rust body src/tree.rs:94 |  |  | 0.512 |
| walker |  | 7702 | 12 | rust body src/tree.rs:98 |  |  | 0.512 |
| walker |  | 7716 | 14 | rust body src/tree.rs:386 |  |  | 0.512 |
| walker |  | 7733 | 17 | rust body src/tree.rs:390 |  |  | 0.512 |
| walker |  | 7743 | 10 | rust body src/ui/filter.rs:102 |  |  | 0.512 |
| walker |  | 7754 | 11 | rust body src/ui/popup.rs:35 |  |  | 0.512 |
| walker |  | 7767 | 13 | rust body src/parse/json.rs:21 |  |  | 0.512 |
| walker |  | 7781 | 14 | rust body src/parse/hcl.rs:23 |  |  | 0.512 |
| walker |  | 7795 | 14 | rust body src/parse/xml.rs:32 |  |  | 0.512 |
| walker |  | 7809 | 14 | rust body src/ui/filter.rs:106 |  |  | 0.512 |
| walker |  | 7858 | 49 | rust decl src/parse/xml.rs:42 |  |  | 0.512 |
| walker |  | 7873 | 15 | rust body src/parse/any.rs:20 |  |  | 0.512 |
| walker |  | 7888 | 15 | rust body src/parse/jsonl.rs:35 |  |  | 0.512 |
| walker |  | 7940 | 52 | rust decl src/ui/app.rs:38 |  |  | 0.518 |
| walker |  | 7993 | 53 | rust decl src/parse/toml.rs:88 |  |  | 0.518 |
| walker |  | 8009 | 16 | rust body src/parse/any.rs:26 |  |  | 0.518 |
| ns | 8139 |  | 549 | Every method on App, plus its layout and polling constants | 6.2 | 6.1 | 0.506 |
| walker |  | 8203 | 194 | rust decl src/config/types.rs:27 |  |  | 0.514 |
| walker |  | 8397 | 194 | rust decl src/parse/mod.rs:17 |  |  | 0.534 |
| walker |  | 8414 | 17 | rust body src/parse/any.rs:34 |  |  | 0.534 |
| walker |  | 8431 | 17 | rust body src/parse/json.rs:17 |  |  | 0.534 |
| walker |  | 8489 | 58 | rust decl src/parse/xml.rs:60 |  |  | 0.534 |
| walker |  | 8693 | 204 | rust decl src/ui/data_block.rs:32 |  |  | 0.535 |
| ns | 8734 |  | 595 | TreeOverview — complete method roster and its two constants | 6.3 |  | 0.522 |
| walker |  | 8754 | 61 | rust decl src/ui/data_block.rs:147 |  |  | 0.522 |
| walker |  | 8772 | 18 | rust body src/parse/any.rs:30 |  |  | 0.522 |
| walker |  | 8790 | 18 | rust body src/parse/hcl.rs:19 |  |  | 0.522 |
| walker |  | 8808 | 18 | rust body src/ui/app.rs:133 |  |  | 0.522 |
| walker |  | 8857 | 49 | rust body build.rs:8 |  |  | 0.522 |
| walker |  | 8961 | 104 | README.md section #4 |  |  | 0.522 |
| ns | 9136 |  | 402 | Filter widget — the filter types and complete method roster | 6.4 |  | 0.533 |
| walker |  | 9193 | 232 | rust decl src/config/colors.rs:20 |  |  | 0.541 |
| walker |  | 9237 | 44 | rust body src/live_reload.rs:64 |  |  | 0.541 |
| ns | 9456 |  | 320 | DataBlock — complete method roster and the scroll-retain constant | 6.5 |  | 0.541 |
| walker |  | 9506 | 269 | rust decl src/ui/tree_overview.rs:32 |  |  | 0.546 |
| walker |  | 9517 | 11 | rust body src/ui/tree_overview.rs:67 |  |  | 0.546 |
| walker |  | 9529 | 12 | rust body src/ui/tree_overview.rs:59 |  |  | 0.546 |
| walker |  | 9541 | 12 | rust body src/ui/tree_overview.rs:63 |  |  | 0.546 |
| walker |  | 9820 | 279 | rust decl src/ui/app.rs:53 |  |  | 0.546 |
| walker |  | 9867 | 47 | rust body src/tree.rs:52 |  |  | 0.547 |
| walker |  | 9893 | 26 | rust body src/parse/any.rs:14 |  |  | 0.547 |
| walker |  | 9983 | 90 | rust decl src/ui/app.rs:27 |  |  | 0.561 |
| ns | 9997 |  | 541 | Popup, Header and Footer — types and complete method rosters | 6.6 |  | 0.568 |

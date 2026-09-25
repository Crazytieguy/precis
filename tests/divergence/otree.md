Score(3000)=0.664 I=0.879 C=0.502 ns_rows≤3K=19/44 grid(1000/1442/2080/3000/4327/6240/9000)=0.877/0.792/0.652/0.664/0.749/0.632/0.603

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 35 | 35 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 39 | 4 | Fs::DirListing { dir: assets } |  |  | 0.000 |
| ns | 44 |  | 44 | Repository identity — README title and one-line description | 1.1 |  | 0.000 |
| walker |  | 47 | 8 | Fs::DirListing { dir: config } |  |  | 0.000 |
| walker |  | 62 | 15 | Fs::DirListing { dir: docs } |  |  | 0.000 |
| walker |  | 70 | 8 | Fs::DirListing { dir: config/themes } |  |  | 0.000 |
| ns | 79 |  | 35 | Complete repository root listing | 1.2 |  | 0.697 |
| walker |  | 108 | 38 | Fs::DirListing { dir: src } |  |  | 0.785 |
| ns | 117 |  | 38 | Complete src/ listing — the module roster | 1.3 |  | 0.754 |
| walker |  | 124 | 16 | Fs::DirListing { dir: src/config } |  |  | 0.756 |
| walker |  | 159 | 35 | Fs::DirListing { dir: src/ui } |  |  | 0.785 |
| walker |  | 202 | 43 | Fs::DirListing { dir: src/parse } |  |  | 0.848 |
| ns | 211 |  | 94 | Complete listings of src/ui, src/parse, src/config | 1.4 | 1.3 | 0.802 |
| walker |  | 246 | 44 | Markdown::ReadmeHeadline { file: README.md } |  |  | 1.000 |
| walker |  | 254 | 8 | Fs::DirListing { dir: .github } |  |  | 1.000 |
| ns | 268 |  | 57 | Cargo package identity — name, version, description | 1.5 |  | 0.967 |
| walker |  | 270 | 16 | Fs::DirListing { dir: .github/workflows } |  |  | 0.967 |
| ns | 303 |  | 35 | All README H2 section headings | 1.6 |  | 0.923 |
| walker |  | 411 | 141 | Toml::Identity { file: Cargo.toml } |  |  | 0.865 |
| ns | 411 |  | 108 | README Usage — how the binary is invoked | 1.7 | 1.6 | 0.865 |
| walker |  | 474 | 63 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.906 |
| ns | 508 |  | 97 | README pointers to config file and reference docs | 1.8 | 1.7 | 0.866 |
| walker |  | 514 | 40 | Fs::DirListing { dir: examples } |  |  | 0.881 |
| ns | 583 |  | 75 | Complete listings of docs/, config/, config/themes/, examples/, assets/ | 1.9 | 1.2 | 0.885 |
| walker |  | 603 | 89 | Code::CodeKey { rung: Names, file: src/main.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.886 |
| walker |  | 651 | 48 | Code::CodeKey { rung: Decl, file: src/main.rs, decl: 2, sub: 0, line: 111 } |  |  | 0.891 |
| ns | 722 |  | 139 | Cargo [dependencies] — first half | 1.10 |  | 0.828 |
| ns | 909 |  | 187 | Cargo [dependencies] tail plus [build-dependencies] | 1.11 | 1.10 | 0.768 |
| walker |  | 924 | 273 | Toml::Dependencies { file: Cargo.toml } |  |  | 0.875 |
| ns | 982 |  | 73 | main.rs entry points — run() signature and main() | 2.1 |  | 0.877 |
| walker |  | 1012 | 88 | Code::CodeKey { rung: Names, file: build.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.877 |
| walker |  | 1061 | 49 | Code::CodeKey { rung: Body, file: build.rs, decl: 1, sub: 0, line: 8 } |  |  | 0.877 |
| walker |  | 1168 | 107 | Code::CodeKey { rung: Body, file: build.rs, decl: 2, sub: 0, line: 14 } |  |  | 0.877 |
| ns | 1188 |  | 206 | run(): CLI parse, config resolution, --show-config, --debug | 2.2 | 2.1 | 0.791 |
| walker |  | 1189 | 21 | Code::CodeKey { rung: Names, file: src/edit.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.791 |
| walker |  | 1219 | 30 | Code::CodeKey { rung: Decl, file: src/edit.rs, decl: 1, sub: 0, line: 10 } |  |  | 0.791 |
| walker |  | 1269 | 50 | Code::CodeKey { rung: Decl, file: src/edit.rs, decl: 2, sub: 0, line: 16 } |  |  | 0.791 |
| walker |  | 1336 | 67 | Code::CodeKey { rung: Names, file: src/tree.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.791 |
| walker |  | 1376 | 40 | Code::CodeKey { rung: Decl, file: src/tree.rs, decl: 3, sub: 0, line: 35 } |  |  | 0.791 |
| walker |  | 1431 | 55 | Code::CodeKey { rung: Decl, file: src/tree.rs, decl: 4, sub: 0, line: 41 } |  |  | 0.792 |
| walker |  | 1499 | 68 | Code::CodeKey { rung: Decl, file: src/tree.rs, decl: 10, sub: 0, line: 385 } |  |  | 0.792 |
| ns | 1511 |  | 323 | run(): content-type inference, live-reload watcher, reading path or stdin | 2.3 | 2.2 | 0.713 |
| walker |  | 1549 | 50 | Code::CodeKey { rung: Decl, file: src/tree.rs, decl: 13, sub: 0, line: 394 } |  |  | 0.713 |
| walker |  | 1621 | 72 | Code::CodeKey { rung: Decl, file: src/tree.rs, decl: 2, sub: 0, line: 25 } |  |  | 0.715 |
| walker |  | 1706 | 85 | Code::CodeKey { rung: Decl, file: src/tree.rs, decl: 1, sub: 0, line: 14 } |  |  | 0.718 |
| ns | 1790 |  | 279 | run(): the --to conversion short-circuit and the data-size guard | 2.4 | 2.3 | 0.676 |
| walker |  | 1822 | 116 | Code::CodeKey { rung: Decl, file: src/tree.rs, decl: 5, sub: 0, line: 51 } |  |  | 0.676 |
| walker |  | 1832 | 10 | Code::CodeKey { rung: Body, file: src/tree.rs, decl: 8, sub: 0, line: 94 } |  |  | 0.676 |
| walker |  | 1855 | 23 | Code::CodeKey { rung: Names, file: src/cmd.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.676 |
| walker |  | 1915 | 60 | Code::CodeKey { rung: Decl, file: src/cmd.rs, decl: 2, sub: 0, line: 111 } |  |  | 0.676 |
| ns | 1925 |  | 135 | run(): tree construction, App::new, header context, ui::start | 2.5 | 2.4 | 0.652 |
| walker |  | 1927 | 12 | Code::CodeKey { rung: Body, file: src/tree.rs, decl: 9, sub: 0, line: 98 } |  |  | 0.652 |
| walker |  | 1984 | 57 | Code::CodeKey { rung: Names, file: src/live_reload.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.652 |
| walker |  | 2016 | 32 | Code::CodeKey { rung: Decl, file: src/live_reload.rs, decl: 6, sub: 0, line: 93 } |  |  | 0.652 |
| walker |  | 2068 | 52 | Code::CodeKey { rung: Decl, file: src/live_reload.rs, decl: 2, sub: 0, line: 27 } |  |  | 0.652 |
| walker |  | 2120 | 52 | Code::CodeKey { rung: Decl, file: src/live_reload.rs, decl: 3, sub: 0, line: 28 } |  |  | 0.652 |
| walker |  | 2193 | 73 | Code::CodeKey { rung: Decl, file: src/live_reload.rs, decl: 1, sub: 0, line: 16 } |  |  | 0.652 |
| ns | 2233 |  | 308 | ui::start — the draw/edit/quit loop and terminal restoration | 2.6 |  | 0.601 |
| walker |  | 2237 | 44 | Code::CodeKey { rung: Body, file: src/live_reload.rs, decl: 4, sub: 0, line: 64 } |  |  | 0.601 |
| walker |  | 2476 | 239 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.601 |
| ns | 2628 |  | 395 | Complete CommandArgs field roster — every CLI flag with its type | 3.1 |  | 0.559 |
| walker |  | 2777 | 301 | Code::CodeKey { rung: Decl, file: src/main.rs, decl: 1, sub: 0, line: 27 } |  |  | 0.635 |
| ns | 2983 |  | 355 | The --help text for every flag (CommandArgs doc comments) | 3.2 | 3.1 | 0.601 |
| walker |  | 3010 | 233 | Code::CodeKey { rung: Decl, file: src/main.rs, decl: 1, sub: 1, line: 27 } |  |  | 0.671 |
| ns | 3091 |  | 108 | Every short-flag mapping in CommandArgs | 3.3 | 3.1 | 0.659 |
| walker |  | 3289 | 279 | Code::CodeKey { rung: Decl, file: src/main.rs, decl: 1, sub: 2, line: 27 } |  |  | 0.706 |
| walker |  | 3419 | 130 | Code::CodeKey { rung: Decl, file: src/main.rs, decl: 1, sub: 3, line: 27 } |  |  | 0.736 |
| ns | 3448 |  | 357 | get_content_type in full, plus the name of every other CommandArgs method | 3.4 |  | 0.694 |
| walker |  | 3488 | 69 | Code::CodeKey { rung: Names, file: src/debug.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.694 |
| walker |  | 3499 | 11 | Code::CodeKey { rung: Body, file: src/debug.rs, decl: 3, sub: 0, line: 19 } |  |  | 0.694 |
| walker |  | 3598 | 99 | Code::CodeKey { rung: Decl, file: src/debug.rs, decl: 1, sub: 0, line: 7 } |  |  | 0.694 |
| ns | 3657 |  | 209 | ContentType — the complete set of supported input formats | 4.1 |  | 0.672 |
| walker |  | 3706 | 108 | Code::CodeKey { rung: Names, file: src/parse/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.673 |
| walker |  | 3729 | 23 | Code::CodeKey { rung: Decl, file: src/parse/mod.rs, decl: 8, sub: 0, line: 66 } |  |  | 0.673 |
| walker |  | 3860 | 131 | Code::CodeKey { rung: Decl, file: src/parse/mod.rs, decl: 2, sub: 0, line: 36 } |  |  | 0.674 |
| ns | 3974 |  | 317 | The Parser trait surface and ContentType::new_parser | 4.2 | 4.1 | 0.660 |
| walker |  | 4054 | 194 | Code::CodeKey { rung: Decl, file: src/parse/mod.rs, decl: 1, sub: 0, line: 17 } |  |  | 0.695 |
| walker |  | 4269 | 215 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.737 |
| ns | 4291 |  | 317 | tree.rs — Tree, ItemValue, HighlightKeyword and FieldType | 4.3 |  | 0.749 |
| walker |  | 4395 | 126 | Code::CodeKey { rung: Names, file: src/config/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.749 |
| walker |  | 4438 | 43 | Code::CodeKey { rung: Decl, file: src/config/mod.rs, decl: 7, sub: 0, line: 99 } |  |  | 0.749 |
| walker |  | 4497 | 59 | Code::CodeKey { rung: Decl, file: src/config/mod.rs, decl: 5, sub: 0, line: 82 } |  |  | 0.750 |
| ns | 4498 |  | 207 | Parser::parse_root — UTF-8 decode and root-shape validation | 4.4 | 4.2 | 0.730 |
| walker |  | 4566 | 69 | Code::CodeKey { rung: Decl, file: src/config/mod.rs, decl: 6, sub: 0, line: 90 } |  |  | 0.730 |
| ns | 4631 |  | 133 | SyntaxToken — the complete highlight token vocabulary | 4.5 |  | 0.714 |
| walker |  | 4642 | 76 | Code::CodeKey { rung: Decl, file: src/config/mod.rs, decl: 2, sub: 0, line: 52 } |  |  | 0.714 |
| walker |  | 4718 | 76 | Code::CodeKey { rung: Decl, file: src/config/mod.rs, decl: 4, sub: 0, line: 73 } |  |  | 0.715 |
| walker |  | 4814 | 96 | Code::CodeKey { rung: Decl, file: src/config/mod.rs, decl: 9, sub: 0, line: 117 } |  |  | 0.715 |
| ns | 4864 |  | 233 | JSON, JSONL and YAML parsers — types, extensions, array roots allowed | 4.6 | 4.2 | 0.694 |
| walker |  | 4915 | 101 | Code::CodeKey { rung: Decl, file: src/config/mod.rs, decl: 3, sub: 0, line: 61 } |  |  | 0.694 |
| walker |  | 5019 | 104 | Code::CodeKey { rung: Decl, file: src/config/mod.rs, decl: 8, sub: 0, line: 105 } |  |  | 0.695 |
| ns | 5099 |  | 235 | TOML, XML and HCL parsers — types, extensions, array roots rejected | 4.7 | 4.6 | 0.675 |
| walker |  | 5134 | 115 | Code::CodeKey { rung: Decl, file: src/config/mod.rs, decl: 10, sub: 0, line: 127 } |  |  | 0.675 |
| walker |  | 5148 | 14 | Code::CodeKey { rung: Body, file: src/tree.rs, decl: 11, sub: 0, line: 386 } |  |  | 0.675 |
| walker |  | 5186 | 38 | Code::CodeKey { rung: Names, file: src/clipboard.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.675 |
| walker |  | 5249 | 63 | Code::CodeKey { rung: Names, file: src/ui/app.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.676 |
| walker |  | 5270 | 21 | Code::CodeKey { rung: Decl, file: src/ui/app.rs, decl: 5, sub: 0, line: 86 } |  |  | 0.676 |
| walker |  | 5299 | 29 | Code::CodeKey { rung: Decl, file: src/ui/app.rs, decl: 3, sub: 0, line: 47 } |  |  | 0.676 |
| ns | 5302 |  | 203 | AnyParser — format auto-detection by trial parsing | 4.8 | 4.7 | 0.662 |
| walker |  | 5351 | 52 | Code::CodeKey { rung: Decl, file: src/ui/app.rs, decl: 2, sub: 0, line: 38 } |  |  | 0.662 |
| walker |  | 5418 | 67 | Code::CodeKey { rung: Decl, file: src/ui/app.rs, decl: 6, sub: 0, line: 91 } |  |  | 0.662 |
| ns | 5432 |  | 130 | Every method on Tree and ItemValue (names only) | 4.9 | 4.3 | 0.659 |
| walker |  | 5434 | 16 | Code::CodeKey { rung: Decl, file: src/ui/app.rs, decl: 7, sub: 0, line: 101 } |  |  | 0.659 |
| walker |  | 5475 | 41 | Code::CodeKey { rung: Decl, file: src/ui/app.rs, decl: 9, sub: 0, line: 137 } |  |  | 0.659 |
| ns | 5538 |  | 106 | syntax.rs shared helpers — StringValue, quoting, wrapping, splitting | 4.10 | 4.5 | 0.652 |
| walker |  | 5565 | 90 | Code::CodeKey { rung: Decl, file: src/ui/app.rs, decl: 1, sub: 0, line: 27 } |  |  | 0.653 |
| ns | 5729 |  | 191 | Complete test inventory — every test module and test fn in the crate | 4.11 |  | 0.642 |
| walker |  | 5844 | 279 | Code::CodeKey { rung: Decl, file: src/ui/app.rs, decl: 4, sub: 0, line: 53 } |  |  | 0.642 |
| walker |  | 5886 | 42 | Code::CodeKey { rung: Names, file: src/ui/header.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.642 |
| ns | 5896 |  | 167 | The Config struct — every configuration section | 5.1 |  | 0.632 |
| walker |  | 5909 | 23 | Code::CodeKey { rung: Decl, file: src/ui/header.rs, decl: 4, sub: 0, line: 41 } |  |  | 0.632 |
| walker |  | 5941 | 32 | Code::CodeKey { rung: Decl, file: src/ui/header.rs, decl: 2, sub: 0, line: 18 } |  |  | 0.632 |
| walker |  | 5989 | 48 | Code::CodeKey { rung: Decl, file: src/ui/header.rs, decl: 1, sub: 0, line: 11 } |  |  | 0.632 |
| walker |  | 6041 | 52 | Code::CodeKey { rung: Decl, file: src/ui/header.rs, decl: 5, sub: 0, line: 46 } |  |  | 0.632 |
| walker |  | 6105 | 64 | Code::CodeKey { rung: Names, file: src/ui/filter.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.632 |
| walker |  | 6130 | 25 | Code::CodeKey { rung: Decl, file: src/ui/filter.rs, decl: 12, sub: 0, line: 152 } |  |  | 0.632 |
| walker |  | 6161 | 31 | Code::CodeKey { rung: Decl, file: src/ui/filter.rs, decl: 2, sub: 0, line: 20 } |  |  | 0.632 |
| walker |  | 6197 | 36 | Code::CodeKey { rung: Decl, file: src/ui/filter.rs, decl: 4, sub: 0, line: 33 } |  |  | 0.632 |
| walker |  | 6232 | 35 | Code::CodeKey { rung: Decl, file: src/ui/filter.rs, decl: 3, sub: 0, line: 27 } |  |  | 0.632 |
| walker |  | 6279 | 47 | Code::CodeKey { rung: Decl, file: src/ui/filter.rs, decl: 1, sub: 0, line: 13 } |  |  | 0.633 |
| ns | 6359 |  | 463 | Every scalar config section struct with its keys | 5.2 | 5.1 | 0.657 |
| walker |  | 6415 | 136 | Code::CodeKey { rung: Decl, file: src/ui/filter.rs, decl: 5, sub: 0, line: 40 } |  |  | 0.658 |
| walker |  | 6425 | 10 | Code::CodeKey { rung: Body, file: src/ui/filter.rs, decl: 9, sub: 0, line: 102 } |  |  | 0.658 |
| walker |  | 6457 | 32 | Code::CodeKey { rung: Names, file: src/ui/popup.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.658 |
| walker |  | 6486 | 29 | Code::CodeKey { rung: Decl, file: src/ui/popup.rs, decl: 1, sub: 0, line: 13 } |  |  | 0.658 |
| walker |  | 6531 | 45 | Code::CodeKey { rung: Decl, file: src/ui/popup.rs, decl: 2, sub: 0, line: 18 } |  |  | 0.658 |
| walker |  | 6673 | 142 | Code::CodeKey { rung: Decl, file: src/ui/popup.rs, decl: 3, sub: 0, line: 26 } |  |  | 0.659 |
| walker |  | 6683 | 10 | Code::CodeKey { rung: Body, file: src/ui/popup.rs, decl: 9, sub: 0, line: 87 } |  |  | 0.659 |
| walker |  | 6694 | 11 | Code::CodeKey { rung: Body, file: src/ui/popup.rs, decl: 5, sub: 0, line: 35 } |  |  | 0.659 |
| ns | 6707 |  | 348 | generate_actions! — the complete Action vocabulary | 5.3 |  | 0.639 |
| walker |  | 6793 | 99 | Code::CodeKey { rung: Names, file: src/config/colors.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.639 |
| walker |  | 6812 | 19 | Code::CodeKey { rung: Decl, file: src/config/colors.rs, decl: 3, sub: 0, line: 55 } |  |  | 0.639 |
| walker |  | 6857 | 45 | Code::CodeKey { rung: Decl, file: src/config/colors.rs, decl: 9, sub: 0, line: 323 } |  |  | 0.639 |
| walker |  | 6948 | 91 | Code::CodeKey { rung: Decl, file: src/config/colors.rs, decl: 7, sub: 0, line: 250 } |  |  | 0.639 |
| walker |  | 7061 | 113 | Code::CodeKey { rung: Decl, file: src/config/colors.rs, decl: 8, sub: 0, line: 285 } |  |  | 0.639 |
| ns | 7113 |  | 406 | The complete colour schema — groups, per-group fields, and the Color type | 5.4 |  | 0.618 |
| walker |  | 7181 | 120 | Code::CodeKey { rung: Decl, file: src/config/colors.rs, decl: 1, sub: 0, line: 9 } |  |  | 0.618 |
| ns | 7236 |  | 123 | The Types section — customisable type labels | 5.5 | 5.1 | 0.613 |
| walker |  | 7301 | 120 | Code::CodeKey { rung: Decl, file: src/config/colors.rs, decl: 10, sub: 0, line: 341 } |  |  | 0.615 |
| ns | 7374 |  | 138 | Every method on Config (names only) | 5.6 | 5.1 | 0.613 |
| walker |  | 7531 | 230 | Code::CodeKey { rung: Decl, file: src/config/colors.rs, decl: 2, sub: 0, line: 20 } |  |  | 0.621 |
| ns | 7590 |  | 216 | The App control enums — Refresh, ElementInFocus, ScrollDirection | 6.1 |  | 0.631 |
| walker |  | 7598 | 67 | Code::CodeKey { rung: Names, file: src/config/keys.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.631 |
| walker |  | 7627 | 29 | Code::CodeKey { rung: Decl, file: src/config/keys.rs, decl: 6, sub: 0, line: 356 } |  |  | 0.631 |
| walker |  | 7663 | 36 | Code::CodeKey { rung: Decl, file: src/config/keys.rs, decl: 5, sub: 0, line: 350 } |  |  | 0.631 |
| walker |  | 7806 | 143 | Code::CodeKey { rung: Decl, file: src/config/keys.rs, decl: 3, sub: 0, line: 53 } |  |  | 0.631 |
| walker |  | 8004 | 198 | Code::CodeKey { rung: Decl, file: src/config/keys.rs, decl: 2, sub: 0, line: 32 } |  |  | 0.631 |
| ns | 8139 |  | 549 | Every method on App, plus its layout and polling constants | 6.2 | 6.1 | 0.617 |
| walker |  | 8218 | 214 | Code::CodeKey { rung: Decl, file: src/config/keys.rs, decl: 1, sub: 0, line: 7 } |  |  | 0.617 |
| walker |  | 8536 | 318 | Code::CodeKey { rung: Decl, file: src/config/colors.rs, decl: 5, sub: 0, line: 77 } |  |  | 0.617 |
| walker |  | 8550 | 14 | Code::CodeKey { rung: Body, file: src/ui/filter.rs, decl: 10, sub: 0, line: 106 } |  |  | 0.617 |
| walker |  | 8573 | 23 | Code::CodeKey { rung: Names, file: src/ui/data_block.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.617 |
| walker |  | 8701 | 128 | Code::CodeKey { rung: Decl, file: src/ui/data_block.rs, decl: 1, sub: 0, line: 16 } |  |  | 0.617 |
| ns | 8734 |  | 595 | TreeOverview — complete method roster and its two constants | 6.3 |  | 0.603 |
| walker |  | 8905 | 204 | Code::CodeKey { rung: Decl, file: src/ui/data_block.rs, decl: 2, sub: 0, line: 32 } |  |  | 0.603 |
| walker |  | 8966 | 61 | Code::CodeKey { rung: Decl, file: src/ui/data_block.rs, decl: 12, sub: 0, line: 147 } |  |  | 0.603 |
| walker |  | 9012 | 46 | Code::CodeKey { rung: Body, file: src/ui/data_block.rs, decl: 5, sub: 0, line: 62 } |  |  | 0.603 |
| walker |  | 9035 | 23 | Code::CodeKey { rung: Names, file: src/ui/tree_overview.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.603 |
| ns | 9136 |  | 402 | Filter widget — the filter types and complete method roster | 6.4 |  | 0.612 |
| walker |  | 9178 | 143 | Code::CodeKey { rung: Decl, file: src/ui/tree_overview.rs, decl: 1, sub: 0, line: 19 } |  |  | 0.612 |
| walker |  | 9447 | 269 | Code::CodeKey { rung: Decl, file: src/ui/tree_overview.rs, decl: 2, sub: 0, line: 32 } |  |  | 0.618 |
| ns | 9456 |  | 320 | DataBlock — complete method roster and the scroll-retain constant | 6.5 |  | 0.616 |
| walker |  | 9458 | 11 | Code::CodeKey { rung: Body, file: src/ui/tree_overview.rs, decl: 7, sub: 0, line: 67 } |  |  | 0.616 |
| walker |  | 9470 | 12 | Code::CodeKey { rung: Body, file: src/ui/tree_overview.rs, decl: 5, sub: 0, line: 59 } |  |  | 0.616 |
| walker |  | 9482 | 12 | Code::CodeKey { rung: Body, file: src/ui/tree_overview.rs, decl: 6, sub: 0, line: 63 } |  |  | 0.616 |
| walker |  | 9517 | 35 | Code::CodeKey { rung: Names, file: src/parse/any.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.617 |
| walker |  | 9536 | 19 | Code::CodeKey { rung: Decl, file: src/parse/any.rs, decl: 1, sub: 0, line: 9 } |  |  | 0.617 |
| walker |  | 9575 | 39 | Code::CodeKey { rung: Decl, file: src/parse/any.rs, decl: 2, sub: 0, line: 13 } |  |  | 0.617 |
| walker |  | 9590 | 15 | Code::CodeKey { rung: Body, file: src/parse/any.rs, decl: 4, sub: 0, line: 20 } |  |  | 0.617 |
| walker |  | 9711 | 121 | Code::CodeKey { rung: Decl, file: src/parse/any.rs, decl: 5, sub: 0, line: 25 } |  |  | 0.618 |
| walker |  | 9727 | 16 | Code::CodeKey { rung: Body, file: src/parse/any.rs, decl: 6, sub: 0, line: 26 } |  |  | 0.618 |
| walker |  | 9744 | 17 | Code::CodeKey { rung: Body, file: src/parse/any.rs, decl: 8, sub: 0, line: 34 } |  |  | 0.618 |
| walker |  | 9762 | 18 | Code::CodeKey { rung: Body, file: src/parse/any.rs, decl: 7, sub: 0, line: 30 } |  |  | 0.618 |
| walker |  | 9797 | 35 | Code::CodeKey { rung: Names, file: src/ui/footer.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.618 |
| walker |  | 9811 | 14 | Code::CodeKey { rung: Decl, file: src/ui/footer.rs, decl: 2, sub: 0, line: 16 } |  |  | 0.618 |
| walker |  | 9845 | 34 | Code::CodeKey { rung: Decl, file: src/ui/footer.rs, decl: 1, sub: 0, line: 10 } |  |  | 0.618 |
| walker |  | 9896 | 51 | Code::CodeKey { rung: Decl, file: src/ui/footer.rs, decl: 3, sub: 0, line: 20 } |  |  | 0.618 |
| walker |  | 9905 | 9 | Code::CodeKey { rung: Body, file: src/ui/footer.rs, decl: 4, sub: 0, line: 21 } |  |  | 0.618 |
| walker |  | 9991 | 86 | Code::CodeKey { rung: Decl, file: src/config/mod.rs, decl: 1, sub: 0, line: 16 } |  |  | 0.620 |
| ns | 9997 |  | 541 | Popup, Header and Footer — types and complete method rosters | 6.6 |  | 0.625 |

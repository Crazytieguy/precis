Score(3000)=0.753 I=0.909 C=0.623 ns_rows≤3K=19/44 grid(1000/1442/2080/3000/4327/6240/9000)=0.877/0.895/0.919/0.753/0.790/0.681/0.648

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 35 | 35 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 39 | 4 | Fs::DirListing { dir: assets } |  |  | 0.000 |
| ns | 44 |  | 44 | Repository identity — README title and one-line description | 1.1 |  | 0.000 |
| ns | 79 |  | 35 | Complete repository root listing | 1.2 |  | 0.689 |
| walker |  | 83 | 44 | Markdown::ReadmeHeadline { file: README.md } |  |  | 1.000 |
| walker |  | 91 | 8 | Fs::DirListing { dir: config } |  |  | 1.000 |
| walker |  | 106 | 15 | Fs::DirListing { dir: docs } |  |  | 1.000 |
| walker |  | 114 | 8 | Fs::DirListing { dir: config/themes } |  |  | 1.000 |
| ns | 117 |  | 38 | Complete src/ listing — the module roster | 1.3 |  | 0.727 |
| walker |  | 152 | 38 | Fs::DirListing { dir: src } |  |  | 1.000 |
| walker |  | 168 | 16 | Fs::DirListing { dir: src/config } |  |  | 1.000 |
| walker |  | 203 | 35 | Fs::DirListing { dir: src/ui } |  |  | 1.000 |
| ns | 211 |  | 94 | Complete listings of src/ui, src/parse, src/config | 1.4 | 1.3 | 0.783 |
| walker |  | 246 | 43 | Fs::DirListing { dir: src/parse } |  |  | 1.000 |
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
| ns | 1188 |  | 206 | run(): CLI parse, config resolution, --show-config, --debug | 2.2 | 2.1 | 0.791 |
| walker |  | 1223 | 211 | Code::CodeKey { rung: Decl, file: src/main.rs, decl: 1, sub: 0, line: 27 } |  |  | 0.893 |
| ns | 1511 |  | 323 | run(): content-type inference, live-reload watcher, reading path or stdin | 2.3 | 2.2 | 0.804 |
| walker |  | 1546 | 323 | Code::CodeKey { rung: Decl, file: src/main.rs, decl: 1, sub: 1, line: 27 } |  |  | 0.907 |
| ns | 1790 |  | 279 | run(): the --to conversion short-circuit and the data-size guard | 2.4 | 2.3 | 0.854 |
| walker |  | 1825 | 279 | Code::CodeKey { rung: Decl, file: src/main.rs, decl: 1, sub: 2, line: 27 } |  |  | 0.914 |
| ns | 1925 |  | 135 | run(): tree construction, App::new, header context, ui::start | 2.5 | 2.4 | 0.881 |
| walker |  | 1955 | 130 | Code::CodeKey { rung: Decl, file: src/main.rs, decl: 1, sub: 3, line: 27 } |  |  | 0.918 |
| walker |  | 2022 | 67 | Code::CodeKey { rung: Names, file: src/tree.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.919 |
| walker |  | 2062 | 40 | Code::CodeKey { rung: Decl, file: src/tree.rs, decl: 3, sub: 0, line: 35 } |  |  | 0.919 |
| walker |  | 2117 | 55 | Code::CodeKey { rung: Decl, file: src/tree.rs, decl: 4, sub: 0, line: 41 } |  |  | 0.920 |
| walker |  | 2185 | 68 | Code::CodeKey { rung: Decl, file: src/tree.rs, decl: 10, sub: 0, line: 385 } |  |  | 0.920 |
| ns | 2233 |  | 308 | ui::start — the draw/edit/quit loop and terminal restoration | 2.6 |  | 0.848 |
| walker |  | 2235 | 50 | Code::CodeKey { rung: Decl, file: src/tree.rs, decl: 13, sub: 0, line: 394 } |  |  | 0.848 |
| walker |  | 2307 | 72 | Code::CodeKey { rung: Decl, file: src/tree.rs, decl: 2, sub: 0, line: 25 } |  |  | 0.850 |
| walker |  | 2392 | 85 | Code::CodeKey { rung: Decl, file: src/tree.rs, decl: 1, sub: 0, line: 14 } |  |  | 0.854 |
| walker |  | 2508 | 116 | Code::CodeKey { rung: Decl, file: src/tree.rs, decl: 5, sub: 0, line: 51 } |  |  | 0.854 |
| walker |  | 2531 | 23 | Code::CodeKey { rung: Names, file: src/cmd.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.854 |
| walker |  | 2591 | 60 | Code::CodeKey { rung: Decl, file: src/cmd.rs, decl: 2, sub: 0, line: 111 } |  |  | 0.854 |
| ns | 2628 |  | 395 | Complete CommandArgs field roster — every CLI flag with its type | 3.1 |  | 0.794 |
| walker |  | 2648 | 57 | Code::CodeKey { rung: Names, file: src/live_reload.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.794 |
| walker |  | 2680 | 32 | Code::CodeKey { rung: Decl, file: src/live_reload.rs, decl: 6, sub: 0, line: 93 } |  |  | 0.794 |
| walker |  | 2732 | 52 | Code::CodeKey { rung: Decl, file: src/live_reload.rs, decl: 2, sub: 0, line: 27 } |  |  | 0.794 |
| walker |  | 2784 | 52 | Code::CodeKey { rung: Decl, file: src/live_reload.rs, decl: 3, sub: 0, line: 28 } |  |  | 0.794 |
| walker |  | 2857 | 73 | Code::CodeKey { rung: Decl, file: src/live_reload.rs, decl: 1, sub: 0, line: 16 } |  |  | 0.794 |
| walker |  | 2878 | 21 | Code::CodeKey { rung: Names, file: src/edit.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.794 |
| walker |  | 2908 | 30 | Code::CodeKey { rung: Decl, file: src/edit.rs, decl: 1, sub: 0, line: 10 } |  |  | 0.794 |
| walker |  | 2958 | 50 | Code::CodeKey { rung: Decl, file: src/edit.rs, decl: 2, sub: 0, line: 16 } |  |  | 0.794 |
| ns | 2983 |  | 355 | The --help text for every flag (CommandArgs doc comments) | 3.2 | 3.1 | 0.752 |
| ns | 3091 |  | 108 | Every short-flag mapping in CommandArgs | 3.3 | 3.1 | 0.738 |
| walker |  | 3135 | 177 | Code::CodeKey { rung: Decl, file: src/cmd.rs, decl: 1, sub: 0, line: 11 } |  |  | 0.745 |
| walker |  | 3374 | 239 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.745 |
| ns | 3448 |  | 357 | get_content_type in full, plus the name of every other CommandArgs method | 3.4 |  | 0.703 |
| walker |  | 3616 | 242 | Code::CodeKey { rung: Decl, file: src/cmd.rs, decl: 1, sub: 1, line: 11 } |  |  | 0.721 |
| ns | 3657 |  | 209 | ContentType — the complete set of supported input formats | 4.1 |  | 0.699 |
| walker |  | 3866 | 250 | Code::CodeKey { rung: Decl, file: src/cmd.rs, decl: 1, sub: 2, line: 11 } |  |  | 0.733 |
| ns | 3974 |  | 317 | The Parser trait surface and ContentType::new_parser | 4.2 | 4.1 | 0.706 |
| walker |  | 4081 | 215 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.747 |
| ns | 4291 |  | 317 | tree.rs — Tree, ItemValue, HighlightKeyword and FieldType | 4.3 |  | 0.759 |
| walker |  | 4312 | 231 | Code::CodeKey { rung: Decl, file: src/cmd.rs, decl: 1, sub: 3, line: 11 } |  |  | 0.790 |
| walker |  | 4435 | 123 | Code::CodeKey { rung: Decl, file: src/cmd.rs, decl: 1, sub: 4, line: 11 } |  |  | 0.815 |
| walker |  | 4473 | 38 | Code::CodeKey { rung: Names, file: src/clipboard.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.815 |
| ns | 4498 |  | 207 | Parser::parse_root — UTF-8 decode and root-shape validation | 4.4 | 4.2 | 0.793 |
| walker |  | 4542 | 69 | Code::CodeKey { rung: Names, file: src/debug.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.793 |
| ns | 4631 |  | 133 | SyntaxToken — the complete highlight token vocabulary | 4.5 |  | 0.775 |
| walker |  | 4641 | 99 | Code::CodeKey { rung: Decl, file: src/debug.rs, decl: 1, sub: 0, line: 7 } |  |  | 0.775 |
| walker |  | 4652 | 11 | Code::CodeKey { rung: Body, file: src/debug.rs, decl: 3, sub: 0, line: 19 } |  |  | 0.775 |
| walker |  | 4715 | 63 | Code::CodeKey { rung: Names, file: src/ui/app.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.775 |
| walker |  | 4736 | 21 | Code::CodeKey { rung: Decl, file: src/ui/app.rs, decl: 5, sub: 0, line: 86 } |  |  | 0.775 |
| walker |  | 4765 | 29 | Code::CodeKey { rung: Decl, file: src/ui/app.rs, decl: 3, sub: 0, line: 47 } |  |  | 0.775 |
| walker |  | 4817 | 52 | Code::CodeKey { rung: Decl, file: src/ui/app.rs, decl: 2, sub: 0, line: 38 } |  |  | 0.776 |
| ns | 4864 |  | 233 | JSON, JSONL and YAML parsers — types, extensions, array roots allowed | 4.6 | 4.2 | 0.752 |
| walker |  | 4884 | 67 | Code::CodeKey { rung: Decl, file: src/ui/app.rs, decl: 6, sub: 0, line: 91 } |  |  | 0.752 |
| walker |  | 4900 | 16 | Code::CodeKey { rung: Decl, file: src/ui/app.rs, decl: 7, sub: 0, line: 101 } |  |  | 0.752 |
| walker |  | 4941 | 41 | Code::CodeKey { rung: Decl, file: src/ui/app.rs, decl: 9, sub: 0, line: 137 } |  |  | 0.752 |
| walker |  | 5031 | 90 | Code::CodeKey { rung: Decl, file: src/ui/app.rs, decl: 1, sub: 0, line: 27 } |  |  | 0.753 |
| ns | 5099 |  | 235 | TOML, XML and HCL parsers — types, extensions, array roots rejected | 4.7 | 4.6 | 0.732 |
| ns | 5302 |  | 203 | AnyParser — format auto-detection by trial parsing | 4.8 | 4.7 | 0.716 |
| walker |  | 5310 | 279 | Code::CodeKey { rung: Decl, file: src/ui/app.rs, decl: 4, sub: 0, line: 53 } |  |  | 0.716 |
| walker |  | 5359 | 49 | Code::CodeKey { rung: Body, file: build.rs, decl: 1, sub: 0, line: 8 } |  |  | 0.716 |
| walker |  | 5426 | 67 | Code::CodeKey { rung: Names, file: src/config/keys.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.716 |
| ns | 5432 |  | 130 | Every method on Tree and ItemValue (names only) | 4.9 | 4.3 | 0.712 |
| walker |  | 5455 | 29 | Code::CodeKey { rung: Decl, file: src/config/keys.rs, decl: 6, sub: 0, line: 356 } |  |  | 0.712 |
| walker |  | 5491 | 36 | Code::CodeKey { rung: Decl, file: src/config/keys.rs, decl: 5, sub: 0, line: 350 } |  |  | 0.712 |
| ns | 5538 |  | 106 | syntax.rs shared helpers — StringValue, quoting, wrapping, splitting | 4.10 | 4.5 | 0.704 |
| walker |  | 5634 | 143 | Code::CodeKey { rung: Decl, file: src/config/keys.rs, decl: 3, sub: 0, line: 53 } |  |  | 0.704 |
| ns | 5729 |  | 191 | Complete test inventory — every test module and test fn in the crate | 4.11 |  | 0.692 |
| walker |  | 5832 | 198 | Code::CodeKey { rung: Decl, file: src/config/keys.rs, decl: 2, sub: 0, line: 32 } |  |  | 0.692 |
| ns | 5896 |  | 167 | The Config struct — every configuration section | 5.1 |  | 0.681 |
| walker |  | 6046 | 214 | Code::CodeKey { rung: Decl, file: src/config/keys.rs, decl: 1, sub: 0, line: 7 } |  |  | 0.681 |
| walker |  | 6145 | 99 | Code::CodeKey { rung: Names, file: src/config/colors.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.681 |
| walker |  | 6164 | 19 | Code::CodeKey { rung: Decl, file: src/config/colors.rs, decl: 3, sub: 0, line: 55 } |  |  | 0.681 |
| walker |  | 6209 | 45 | Code::CodeKey { rung: Decl, file: src/config/colors.rs, decl: 9, sub: 0, line: 323 } |  |  | 0.681 |
| walker |  | 6300 | 91 | Code::CodeKey { rung: Decl, file: src/config/colors.rs, decl: 7, sub: 0, line: 250 } |  |  | 0.681 |
| ns | 6359 |  | 463 | Every scalar config section struct with its keys | 5.2 | 5.1 | 0.651 |
| walker |  | 6413 | 113 | Code::CodeKey { rung: Decl, file: src/config/colors.rs, decl: 8, sub: 0, line: 285 } |  |  | 0.651 |
| walker |  | 6533 | 120 | Code::CodeKey { rung: Decl, file: src/config/colors.rs, decl: 1, sub: 0, line: 9 } |  |  | 0.651 |
| walker |  | 6653 | 120 | Code::CodeKey { rung: Decl, file: src/config/colors.rs, decl: 10, sub: 0, line: 341 } |  |  | 0.652 |
| ns | 6707 |  | 348 | generate_actions! — the complete Action vocabulary | 5.3 |  | 0.632 |
| walker |  | 6883 | 230 | Code::CodeKey { rung: Decl, file: src/config/colors.rs, decl: 2, sub: 0, line: 20 } |  |  | 0.633 |
| walker |  | 7009 | 126 | Code::CodeKey { rung: Names, file: src/config/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.634 |
| walker |  | 7052 | 43 | Code::CodeKey { rung: Decl, file: src/config/mod.rs, decl: 7, sub: 0, line: 99 } |  |  | 0.636 |
| walker |  | 7111 | 59 | Code::CodeKey { rung: Decl, file: src/config/mod.rs, decl: 5, sub: 0, line: 82 } |  |  | 0.640 |
| ns | 7113 |  | 406 | The complete colour schema — groups, per-group fields, and the Color type | 5.4 |  | 0.629 |
| walker |  | 7180 | 69 | Code::CodeKey { rung: Decl, file: src/config/mod.rs, decl: 6, sub: 0, line: 90 } |  |  | 0.633 |
| ns | 7236 |  | 123 | The Types section — customisable type labels | 5.5 | 5.1 | 0.627 |
| walker |  | 7256 | 76 | Code::CodeKey { rung: Decl, file: src/config/mod.rs, decl: 2, sub: 0, line: 52 } |  |  | 0.633 |
| walker |  | 7332 | 76 | Code::CodeKey { rung: Decl, file: src/config/mod.rs, decl: 4, sub: 0, line: 73 } |  |  | 0.639 |
| ns | 7374 |  | 138 | Every method on Config (names only) | 5.6 | 5.1 | 0.633 |
| walker |  | 7428 | 96 | Code::CodeKey { rung: Decl, file: src/config/mod.rs, decl: 9, sub: 0, line: 117 } |  |  | 0.641 |
| walker |  | 7529 | 101 | Code::CodeKey { rung: Decl, file: src/config/mod.rs, decl: 3, sub: 0, line: 61 } |  |  | 0.650 |
| ns | 7590 |  | 216 | The App control enums — Refresh, ElementInFocus, ScrollDirection | 6.1 |  | 0.658 |
| walker |  | 7633 | 104 | Code::CodeKey { rung: Decl, file: src/config/mod.rs, decl: 8, sub: 0, line: 105 } |  |  | 0.668 |
| walker |  | 7748 | 115 | Code::CodeKey { rung: Decl, file: src/config/mod.rs, decl: 10, sub: 0, line: 127 } |  |  | 0.672 |
| walker |  | 8066 | 318 | Code::CodeKey { rung: Decl, file: src/config/colors.rs, decl: 5, sub: 0, line: 77 } |  |  | 0.672 |
| walker |  | 8089 | 23 | Code::CodeKey { rung: Names, file: src/ui/tree_overview.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.672 |
| ns | 8139 |  | 549 | Every method on App, plus its layout and polling constants | 6.2 | 6.1 | 0.657 |
| walker |  | 8232 | 143 | Code::CodeKey { rung: Decl, file: src/ui/tree_overview.rs, decl: 1, sub: 0, line: 19 } |  |  | 0.657 |
| walker |  | 8501 | 269 | Code::CodeKey { rung: Decl, file: src/ui/tree_overview.rs, decl: 2, sub: 0, line: 32 } |  |  | 0.657 |
| walker |  | 8524 | 23 | Code::CodeKey { rung: Names, file: src/ui/data_block.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.657 |
| walker |  | 8652 | 128 | Code::CodeKey { rung: Decl, file: src/ui/data_block.rs, decl: 1, sub: 0, line: 16 } |  |  | 0.657 |
| ns | 8734 |  | 595 | TreeOverview — complete method roster and its two constants | 6.3 |  | 0.647 |
| walker |  | 8856 | 204 | Code::CodeKey { rung: Decl, file: src/ui/data_block.rs, decl: 2, sub: 0, line: 32 } |  |  | 0.648 |
| walker |  | 8917 | 61 | Code::CodeKey { rung: Decl, file: src/ui/data_block.rs, decl: 12, sub: 0, line: 147 } |  |  | 0.648 |
| walker |  | 8981 | 64 | Code::CodeKey { rung: Names, file: src/ui/filter.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.648 |
| walker |  | 9006 | 25 | Code::CodeKey { rung: Decl, file: src/ui/filter.rs, decl: 12, sub: 0, line: 152 } |  |  | 0.648 |
| walker |  | 9037 | 31 | Code::CodeKey { rung: Decl, file: src/ui/filter.rs, decl: 2, sub: 0, line: 20 } |  |  | 0.648 |
| walker |  | 9073 | 36 | Code::CodeKey { rung: Decl, file: src/ui/filter.rs, decl: 4, sub: 0, line: 33 } |  |  | 0.648 |
| walker |  | 9108 | 35 | Code::CodeKey { rung: Decl, file: src/ui/filter.rs, decl: 3, sub: 0, line: 27 } |  |  | 0.648 |
| ns | 9136 |  | 402 | Filter widget — the filter types and complete method roster | 6.4 |  | 0.638 |
| walker |  | 9155 | 47 | Code::CodeKey { rung: Decl, file: src/ui/filter.rs, decl: 1, sub: 0, line: 13 } |  |  | 0.642 |
| walker |  | 9291 | 136 | Code::CodeKey { rung: Decl, file: src/ui/filter.rs, decl: 5, sub: 0, line: 40 } |  |  | 0.656 |
| ns | 9456 |  | 320 | DataBlock — complete method roster and the scroll-retain constant | 6.5 |  | 0.654 |
| walker |  | 9490 | 199 | Code::CodeKey { rung: Decl, file: src/config/keys.rs, decl: 4, sub: 0, line: 194 } |  |  | 0.654 |
| walker |  | 9525 | 35 | Code::CodeKey { rung: Names, file: src/ui/footer.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.654 |
| walker |  | 9539 | 14 | Code::CodeKey { rung: Decl, file: src/ui/footer.rs, decl: 2, sub: 0, line: 16 } |  |  | 0.654 |
| walker |  | 9573 | 34 | Code::CodeKey { rung: Decl, file: src/ui/footer.rs, decl: 1, sub: 0, line: 10 } |  |  | 0.654 |
| walker |  | 9624 | 51 | Code::CodeKey { rung: Decl, file: src/ui/footer.rs, decl: 3, sub: 0, line: 20 } |  |  | 0.654 |
| walker |  | 9633 | 9 | Code::CodeKey { rung: Body, file: src/ui/footer.rs, decl: 4, sub: 0, line: 21 } |  |  | 0.654 |
| walker |  | 9665 | 32 | Code::CodeKey { rung: Names, file: src/ui/popup.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.654 |
| walker |  | 9694 | 29 | Code::CodeKey { rung: Decl, file: src/ui/popup.rs, decl: 1, sub: 0, line: 13 } |  |  | 0.654 |
| walker |  | 9739 | 45 | Code::CodeKey { rung: Decl, file: src/ui/popup.rs, decl: 2, sub: 0, line: 18 } |  |  | 0.654 |
| walker |  | 9881 | 142 | Code::CodeKey { rung: Decl, file: src/ui/popup.rs, decl: 3, sub: 0, line: 26 } |  |  | 0.655 |
| walker |  | 9982 | 101 | Code::CodeKey { rung: Decl, file: src/config/mod.rs, decl: 1, sub: 0, line: 16 } |  |  | 0.657 |
| ns | 9997 |  | 541 | Popup, Header and Footer — types and complete method rosters | 6.6 |  | 0.651 |

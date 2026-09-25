Score(3000)=0.527 I=0.827 C=0.335 ns_rows≤3K=19/44 grid(1000/1442/2080/3000/4327/6240/9000)=0.871/0.786/0.648/0.527/0.554/0.469/0.486

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
| walker |  | 648 | 45 | Code::CodeKey { rung: Body, file: src/main.rs, decl: 2, sub: 0, line: 111 } |  |  | 0.890 |
| ns | 722 |  | 139 | Cargo [dependencies] — first half | 1.10 |  | 0.828 |
| ns | 909 |  | 187 | Cargo [dependencies] tail plus [build-dependencies] | 1.11 | 1.10 | 0.767 |
| walker |  | 921 | 273 | Toml::Dependencies { file: Cargo.toml } |  |  | 0.874 |
| ns | 982 |  | 73 | main.rs entry points — run() signature and main() | 2.1 |  | 0.871 |
| walker |  | 1046 | 125 | Toml::Config { file: Cargo.toml } |  |  | 0.871 |
| walker |  | 1134 | 88 | Code::CodeKey { rung: Names, file: build.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.871 |
| walker |  | 1183 | 49 | Code::CodeKey { rung: Body, file: build.rs, decl: 1, sub: 0, line: 8 } |  |  | 0.871 |
| ns | 1188 |  | 206 | run(): CLI parse, config resolution, --show-config, --debug | 2.2 | 2.1 | 0.786 |
| walker |  | 1290 | 107 | Code::CodeKey { rung: Body, file: build.rs, decl: 2, sub: 0, line: 14 } |  |  | 0.786 |
| walker |  | 1311 | 21 | Code::CodeKey { rung: Names, file: src/edit.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.786 |
| walker |  | 1341 | 30 | Code::CodeKey { rung: Decl, file: src/edit.rs, decl: 1, sub: 0, line: 10 } |  |  | 0.786 |
| walker |  | 1391 | 50 | Code::CodeKey { rung: Decl, file: src/edit.rs, decl: 2, sub: 0, line: 16 } |  |  | 0.786 |
| walker |  | 1458 | 67 | Code::CodeKey { rung: Names, file: src/tree.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.786 |
| walker |  | 1498 | 40 | Code::CodeKey { rung: Decl, file: src/tree.rs, decl: 3, sub: 0, line: 35 } |  |  | 0.786 |
| ns | 1511 |  | 323 | run(): content-type inference, live-reload watcher, reading path or stdin | 2.3 | 2.2 | 0.708 |
| walker |  | 1553 | 55 | Code::CodeKey { rung: Decl, file: src/tree.rs, decl: 4, sub: 0, line: 41 } |  |  | 0.709 |
| walker |  | 1621 | 68 | Code::CodeKey { rung: Decl, file: src/tree.rs, decl: 10, sub: 0, line: 385 } |  |  | 0.709 |
| walker |  | 1671 | 50 | Code::CodeKey { rung: Decl, file: src/tree.rs, decl: 13, sub: 0, line: 394 } |  |  | 0.709 |
| walker |  | 1743 | 72 | Code::CodeKey { rung: Decl, file: src/tree.rs, decl: 2, sub: 0, line: 25 } |  |  | 0.710 |
| ns | 1790 |  | 279 | run(): the --to conversion short-circuit and the data-size guard | 2.4 | 2.3 | 0.669 |
| walker |  | 1828 | 85 | Code::CodeKey { rung: Decl, file: src/tree.rs, decl: 1, sub: 0, line: 14 } |  |  | 0.671 |
| ns | 1925 |  | 135 | run(): tree construction, App::new, header context, ui::start | 2.5 | 2.4 | 0.647 |
| walker |  | 1944 | 116 | Code::CodeKey { rung: Decl, file: src/tree.rs, decl: 5, sub: 0, line: 51 } |  |  | 0.647 |
| walker |  | 1954 | 10 | Code::CodeKey { rung: Body, file: src/tree.rs, decl: 8, sub: 0, line: 94 } |  |  | 0.647 |
| walker |  | 1977 | 23 | Code::CodeKey { rung: Names, file: src/cmd.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.647 |
| walker |  | 2037 | 60 | Code::CodeKey { rung: Decl, file: src/cmd.rs, decl: 2, sub: 0, line: 111 } |  |  | 0.648 |
| walker |  | 2049 | 12 | Code::CodeKey { rung: Body, file: src/tree.rs, decl: 9, sub: 0, line: 98 } |  |  | 0.648 |
| walker |  | 2106 | 57 | Code::CodeKey { rung: Names, file: src/live_reload.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.648 |
| walker |  | 2138 | 32 | Code::CodeKey { rung: Decl, file: src/live_reload.rs, decl: 6, sub: 0, line: 93 } |  |  | 0.648 |
| walker |  | 2190 | 52 | Code::CodeKey { rung: Decl, file: src/live_reload.rs, decl: 2, sub: 0, line: 27 } |  |  | 0.648 |
| ns | 2233 |  | 308 | ui::start — the draw/edit/quit loop and terminal restoration | 2.6 |  | 0.597 |
| walker |  | 2242 | 52 | Code::CodeKey { rung: Decl, file: src/live_reload.rs, decl: 3, sub: 0, line: 28 } |  |  | 0.597 |
| walker |  | 2315 | 73 | Code::CodeKey { rung: Decl, file: src/live_reload.rs, decl: 1, sub: 0, line: 16 } |  |  | 0.597 |
| walker |  | 2359 | 44 | Code::CodeKey { rung: Body, file: src/live_reload.rs, decl: 4, sub: 0, line: 64 } |  |  | 0.597 |
| walker |  | 2463 | 104 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.597 |
| walker |  | 2532 | 69 | Code::CodeKey { rung: Names, file: src/debug.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.597 |
| walker |  | 2543 | 11 | Code::CodeKey { rung: Body, file: src/debug.rs, decl: 3, sub: 0, line: 19 } |  |  | 0.597 |
| ns | 2628 |  | 395 | Complete CommandArgs field roster — every CLI flag with its type | 3.1 |  | 0.555 |
| walker |  | 2642 | 99 | Code::CodeKey { rung: Decl, file: src/debug.rs, decl: 1, sub: 0, line: 7 } |  |  | 0.555 |
| walker |  | 2750 | 108 | Code::CodeKey { rung: Names, file: src/parse/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.555 |
| walker |  | 2773 | 23 | Code::CodeKey { rung: Decl, file: src/parse/mod.rs, decl: 8, sub: 0, line: 66 } |  |  | 0.555 |
| walker |  | 2904 | 131 | Code::CodeKey { rung: Decl, file: src/parse/mod.rs, decl: 2, sub: 0, line: 36 } |  |  | 0.556 |
| ns | 2983 |  | 355 | The --help text for every flag (CommandArgs doc comments) | 3.2 | 3.1 | 0.527 |
| ns | 3091 |  | 108 | Every short-flag mapping in CommandArgs | 3.3 | 3.1 | 0.517 |
| walker |  | 3098 | 194 | Code::CodeKey { rung: Decl, file: src/parse/mod.rs, decl: 1, sub: 0, line: 17 } |  |  | 0.520 |
| walker |  | 3224 | 126 | Code::CodeKey { rung: Names, file: src/config/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.520 |
| walker |  | 3267 | 43 | Code::CodeKey { rung: Decl, file: src/config/mod.rs, decl: 7, sub: 0, line: 99 } |  |  | 0.520 |
| walker |  | 3326 | 59 | Code::CodeKey { rung: Decl, file: src/config/mod.rs, decl: 5, sub: 0, line: 82 } |  |  | 0.520 |
| walker |  | 3395 | 69 | Code::CodeKey { rung: Decl, file: src/config/mod.rs, decl: 6, sub: 0, line: 90 } |  |  | 0.520 |
| ns | 3448 |  | 357 | get_content_type in full, plus the name of every other CommandArgs method | 3.4 |  | 0.492 |
| walker |  | 3471 | 76 | Code::CodeKey { rung: Decl, file: src/config/mod.rs, decl: 2, sub: 0, line: 52 } |  |  | 0.492 |
| walker |  | 3547 | 76 | Code::CodeKey { rung: Decl, file: src/config/mod.rs, decl: 4, sub: 0, line: 73 } |  |  | 0.492 |
| walker |  | 3643 | 96 | Code::CodeKey { rung: Decl, file: src/config/mod.rs, decl: 9, sub: 0, line: 117 } |  |  | 0.493 |
| ns | 3657 |  | 209 | ContentType — the complete set of supported input formats | 4.1 |  | 0.521 |
| walker |  | 3744 | 101 | Code::CodeKey { rung: Decl, file: src/config/mod.rs, decl: 3, sub: 0, line: 61 } |  |  | 0.522 |
| walker |  | 3848 | 104 | Code::CodeKey { rung: Decl, file: src/config/mod.rs, decl: 8, sub: 0, line: 105 } |  |  | 0.523 |
| walker |  | 3963 | 115 | Code::CodeKey { rung: Decl, file: src/config/mod.rs, decl: 10, sub: 0, line: 127 } |  |  | 0.523 |
| ns | 3974 |  | 317 | The Parser trait surface and ContentType::new_parser | 4.2 | 4.1 | 0.517 |
| walker |  | 3977 | 14 | Code::CodeKey { rung: Body, file: src/tree.rs, decl: 11, sub: 0, line: 386 } |  |  | 0.517 |
| walker |  | 4015 | 38 | Code::CodeKey { rung: Names, file: src/clipboard.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.517 |
| walker |  | 4078 | 63 | Code::CodeKey { rung: Names, file: src/ui/app.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.517 |
| walker |  | 4099 | 21 | Code::CodeKey { rung: Decl, file: src/ui/app.rs, decl: 5, sub: 0, line: 86 } |  |  | 0.517 |
| walker |  | 4128 | 29 | Code::CodeKey { rung: Decl, file: src/ui/app.rs, decl: 3, sub: 0, line: 47 } |  |  | 0.517 |
| walker |  | 4180 | 52 | Code::CodeKey { rung: Decl, file: src/ui/app.rs, decl: 2, sub: 0, line: 38 } |  |  | 0.518 |
| walker |  | 4247 | 67 | Code::CodeKey { rung: Decl, file: src/ui/app.rs, decl: 6, sub: 0, line: 91 } |  |  | 0.518 |
| walker |  | 4263 | 16 | Code::CodeKey { rung: Decl, file: src/ui/app.rs, decl: 7, sub: 0, line: 101 } |  |  | 0.518 |
| ns | 4291 |  | 317 | tree.rs — Tree, ItemValue, HighlightKeyword and FieldType | 4.3 |  | 0.554 |
| walker |  | 4304 | 41 | Code::CodeKey { rung: Decl, file: src/ui/app.rs, decl: 9, sub: 0, line: 137 } |  |  | 0.554 |
| walker |  | 4394 | 90 | Code::CodeKey { rung: Decl, file: src/ui/app.rs, decl: 1, sub: 0, line: 27 } |  |  | 0.555 |
| ns | 4498 |  | 207 | Parser::parse_root — UTF-8 decode and root-shape validation | 4.4 | 4.2 | 0.541 |
| ns | 4631 |  | 133 | SyntaxToken — the complete highlight token vocabulary | 4.5 |  | 0.528 |
| walker |  | 4673 | 279 | Code::CodeKey { rung: Decl, file: src/ui/app.rs, decl: 4, sub: 0, line: 53 } |  |  | 0.528 |
| walker |  | 4715 | 42 | Code::CodeKey { rung: Names, file: src/ui/header.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.528 |
| walker |  | 4738 | 23 | Code::CodeKey { rung: Decl, file: src/ui/header.rs, decl: 4, sub: 0, line: 41 } |  |  | 0.528 |
| walker |  | 4770 | 32 | Code::CodeKey { rung: Decl, file: src/ui/header.rs, decl: 2, sub: 0, line: 18 } |  |  | 0.528 |
| walker |  | 4818 | 48 | Code::CodeKey { rung: Decl, file: src/ui/header.rs, decl: 1, sub: 0, line: 11 } |  |  | 0.528 |
| ns | 4864 |  | 233 | JSON, JSONL and YAML parsers — types, extensions, array roots allowed | 4.6 | 4.2 | 0.512 |
| walker |  | 4870 | 52 | Code::CodeKey { rung: Decl, file: src/ui/header.rs, decl: 5, sub: 0, line: 46 } |  |  | 0.512 |
| walker |  | 4934 | 64 | Code::CodeKey { rung: Names, file: src/ui/filter.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.512 |
| walker |  | 4959 | 25 | Code::CodeKey { rung: Decl, file: src/ui/filter.rs, decl: 12, sub: 0, line: 152 } |  |  | 0.512 |
| walker |  | 4990 | 31 | Code::CodeKey { rung: Decl, file: src/ui/filter.rs, decl: 2, sub: 0, line: 20 } |  |  | 0.512 |
| walker |  | 5026 | 36 | Code::CodeKey { rung: Decl, file: src/ui/filter.rs, decl: 4, sub: 0, line: 33 } |  |  | 0.512 |
| walker |  | 5061 | 35 | Code::CodeKey { rung: Decl, file: src/ui/filter.rs, decl: 3, sub: 0, line: 27 } |  |  | 0.513 |
| ns | 5099 |  | 235 | TOML, XML and HCL parsers — types, extensions, array roots rejected | 4.7 | 4.6 | 0.498 |
| walker |  | 5108 | 47 | Code::CodeKey { rung: Decl, file: src/ui/filter.rs, decl: 1, sub: 0, line: 13 } |  |  | 0.498 |
| walker |  | 5244 | 136 | Code::CodeKey { rung: Decl, file: src/ui/filter.rs, decl: 5, sub: 0, line: 40 } |  |  | 0.499 |
| walker |  | 5254 | 10 | Code::CodeKey { rung: Body, file: src/ui/filter.rs, decl: 9, sub: 0, line: 102 } |  |  | 0.499 |
| walker |  | 5286 | 32 | Code::CodeKey { rung: Names, file: src/ui/popup.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.499 |
| ns | 5302 |  | 203 | AnyParser — format auto-detection by trial parsing | 4.8 | 4.7 | 0.489 |
| walker |  | 5315 | 29 | Code::CodeKey { rung: Decl, file: src/ui/popup.rs, decl: 1, sub: 0, line: 13 } |  |  | 0.489 |
| walker |  | 5360 | 45 | Code::CodeKey { rung: Decl, file: src/ui/popup.rs, decl: 2, sub: 0, line: 18 } |  |  | 0.489 |
| ns | 5432 |  | 130 | Every method on Tree and ItemValue (names only) | 4.9 | 4.3 | 0.489 |
| walker |  | 5502 | 142 | Code::CodeKey { rung: Decl, file: src/ui/popup.rs, decl: 3, sub: 0, line: 26 } |  |  | 0.489 |
| walker |  | 5512 | 10 | Code::CodeKey { rung: Body, file: src/ui/popup.rs, decl: 9, sub: 0, line: 87 } |  |  | 0.489 |
| walker |  | 5523 | 11 | Code::CodeKey { rung: Body, file: src/ui/popup.rs, decl: 5, sub: 0, line: 35 } |  |  | 0.489 |
| ns | 5538 |  | 106 | syntax.rs shared helpers — StringValue, quoting, wrapping, splitting | 4.10 | 4.5 | 0.484 |
| walker |  | 5622 | 99 | Code::CodeKey { rung: Names, file: src/config/colors.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.484 |
| walker |  | 5641 | 19 | Code::CodeKey { rung: Decl, file: src/config/colors.rs, decl: 3, sub: 0, line: 55 } |  |  | 0.484 |
| walker |  | 5686 | 45 | Code::CodeKey { rung: Decl, file: src/config/colors.rs, decl: 9, sub: 0, line: 323 } |  |  | 0.484 |
| ns | 5729 |  | 191 | Complete test inventory — every test module and test fn in the crate | 4.11 |  | 0.476 |
| walker |  | 5777 | 91 | Code::CodeKey { rung: Decl, file: src/config/colors.rs, decl: 7, sub: 0, line: 250 } |  |  | 0.476 |
| walker |  | 5890 | 113 | Code::CodeKey { rung: Decl, file: src/config/colors.rs, decl: 8, sub: 0, line: 285 } |  |  | 0.476 |
| ns | 5896 |  | 167 | The Config struct — every configuration section | 5.1 |  | 0.468 |
| walker |  | 6010 | 120 | Code::CodeKey { rung: Decl, file: src/config/colors.rs, decl: 1, sub: 0, line: 9 } |  |  | 0.468 |
| walker |  | 6130 | 120 | Code::CodeKey { rung: Decl, file: src/config/colors.rs, decl: 10, sub: 0, line: 341 } |  |  | 0.469 |
| ns | 6359 |  | 463 | Every scalar config section struct with its keys | 5.2 | 5.1 | 0.509 |
| walker |  | 6360 | 230 | Code::CodeKey { rung: Decl, file: src/config/colors.rs, decl: 2, sub: 0, line: 20 } |  |  | 0.509 |
| walker |  | 6427 | 67 | Code::CodeKey { rung: Names, file: src/config/keys.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.509 |
| walker |  | 6456 | 29 | Code::CodeKey { rung: Decl, file: src/config/keys.rs, decl: 6, sub: 0, line: 356 } |  |  | 0.509 |
| walker |  | 6492 | 36 | Code::CodeKey { rung: Decl, file: src/config/keys.rs, decl: 5, sub: 0, line: 350 } |  |  | 0.509 |
| walker |  | 6635 | 143 | Code::CodeKey { rung: Decl, file: src/config/keys.rs, decl: 3, sub: 0, line: 53 } |  |  | 0.509 |
| ns | 6707 |  | 348 | generate_actions! — the complete Action vocabulary | 5.3 |  | 0.494 |
| walker |  | 6833 | 198 | Code::CodeKey { rung: Decl, file: src/config/keys.rs, decl: 2, sub: 0, line: 32 } |  |  | 0.494 |
| walker |  | 7047 | 214 | Code::CodeKey { rung: Decl, file: src/config/keys.rs, decl: 1, sub: 0, line: 7 } |  |  | 0.494 |
| ns | 7113 |  | 406 | The complete colour schema — groups, per-group fields, and the Color type | 5.4 |  | 0.490 |
| ns | 7236 |  | 123 | The Types section — customisable type labels | 5.5 | 5.1 | 0.485 |
| walker |  | 7365 | 318 | Code::CodeKey { rung: Decl, file: src/config/colors.rs, decl: 5, sub: 0, line: 77 } |  |  | 0.485 |
| ns | 7374 |  | 138 | Every method on Config (names only) | 5.6 | 5.1 | 0.485 |
| walker |  | 7379 | 14 | Code::CodeKey { rung: Body, file: src/ui/filter.rs, decl: 10, sub: 0, line: 106 } |  |  | 0.485 |
| walker |  | 7402 | 23 | Code::CodeKey { rung: Names, file: src/ui/data_block.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.485 |
| walker |  | 7530 | 128 | Code::CodeKey { rung: Decl, file: src/ui/data_block.rs, decl: 1, sub: 0, line: 16 } |  |  | 0.485 |
| ns | 7590 |  | 216 | The App control enums — Refresh, ElementInFocus, ScrollDirection | 6.1 |  | 0.500 |
| walker |  | 7734 | 204 | Code::CodeKey { rung: Decl, file: src/ui/data_block.rs, decl: 2, sub: 0, line: 32 } |  |  | 0.500 |
| walker |  | 7795 | 61 | Code::CodeKey { rung: Decl, file: src/ui/data_block.rs, decl: 12, sub: 0, line: 147 } |  |  | 0.500 |
| walker |  | 7841 | 46 | Code::CodeKey { rung: Body, file: src/ui/data_block.rs, decl: 5, sub: 0, line: 62 } |  |  | 0.500 |
| walker |  | 7864 | 23 | Code::CodeKey { rung: Names, file: src/ui/tree_overview.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.500 |
| walker |  | 8007 | 143 | Code::CodeKey { rung: Decl, file: src/ui/tree_overview.rs, decl: 1, sub: 0, line: 19 } |  |  | 0.500 |
| ns | 8139 |  | 549 | Every method on App, plus its layout and polling constants | 6.2 | 6.1 | 0.489 |
| walker |  | 8276 | 269 | Code::CodeKey { rung: Decl, file: src/ui/tree_overview.rs, decl: 2, sub: 0, line: 32 } |  |  | 0.490 |
| walker |  | 8287 | 11 | Code::CodeKey { rung: Body, file: src/ui/tree_overview.rs, decl: 7, sub: 0, line: 67 } |  |  | 0.490 |
| walker |  | 8299 | 12 | Code::CodeKey { rung: Body, file: src/ui/tree_overview.rs, decl: 5, sub: 0, line: 59 } |  |  | 0.490 |
| walker |  | 8311 | 12 | Code::CodeKey { rung: Body, file: src/ui/tree_overview.rs, decl: 6, sub: 0, line: 63 } |  |  | 0.490 |
| walker |  | 8346 | 35 | Code::CodeKey { rung: Names, file: src/parse/any.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.490 |
| walker |  | 8365 | 19 | Code::CodeKey { rung: Decl, file: src/parse/any.rs, decl: 1, sub: 0, line: 9 } |  |  | 0.490 |
| walker |  | 8404 | 39 | Code::CodeKey { rung: Decl, file: src/parse/any.rs, decl: 2, sub: 0, line: 13 } |  |  | 0.490 |
| walker |  | 8419 | 15 | Code::CodeKey { rung: Body, file: src/parse/any.rs, decl: 4, sub: 0, line: 20 } |  |  | 0.490 |
| walker |  | 8540 | 121 | Code::CodeKey { rung: Decl, file: src/parse/any.rs, decl: 5, sub: 0, line: 25 } |  |  | 0.491 |
| walker |  | 8556 | 16 | Code::CodeKey { rung: Body, file: src/parse/any.rs, decl: 6, sub: 0, line: 26 } |  |  | 0.491 |
| walker |  | 8573 | 17 | Code::CodeKey { rung: Body, file: src/parse/any.rs, decl: 8, sub: 0, line: 34 } |  |  | 0.491 |
| walker |  | 8591 | 18 | Code::CodeKey { rung: Body, file: src/parse/any.rs, decl: 7, sub: 0, line: 30 } |  |  | 0.491 |
| walker |  | 8626 | 35 | Code::CodeKey { rung: Names, file: src/ui/footer.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.491 |
| walker |  | 8640 | 14 | Code::CodeKey { rung: Decl, file: src/ui/footer.rs, decl: 2, sub: 0, line: 16 } |  |  | 0.491 |
| walker |  | 8674 | 34 | Code::CodeKey { rung: Decl, file: src/ui/footer.rs, decl: 1, sub: 0, line: 10 } |  |  | 0.491 |
| walker |  | 8725 | 51 | Code::CodeKey { rung: Decl, file: src/ui/footer.rs, decl: 3, sub: 0, line: 20 } |  |  | 0.491 |
| walker |  | 8734 | 9 | Code::CodeKey { rung: Body, file: src/ui/footer.rs, decl: 4, sub: 0, line: 21 } |  |  | 0.486 |
| ns | 8734 |  | 595 | TreeOverview — complete method roster and its two constants | 6.3 |  | 0.486 |
| walker |  | 9070 | 336 | Code::CodeKey { rung: Decl, file: src/config/mod.rs, decl: 1, sub: 0, line: 16 } |  |  | 0.502 |
| walker |  | 9095 | 25 | Code::CodeKey { rung: Names, file: src/config/types.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.502 |
| ns | 9136 |  | 402 | Filter widget — the filter types and complete method roster | 6.4 |  | 0.515 |
| walker |  | 9289 | 194 | Code::CodeKey { rung: Decl, file: src/config/types.rs, decl: 2, sub: 0, line: 27 } |  |  | 0.522 |
| ns | 9456 |  | 320 | DataBlock — complete method roster and the scroll-retain constant | 6.5 |  | 0.522 |
| walker |  | 9488 | 199 | Code::CodeKey { rung: Decl, file: src/config/types.rs, decl: 1, sub: 0, line: 3 } |  |  | 0.522 |
| walker |  | 9849 | 361 | Code::CodeKey { rung: Decl, file: src/config/colors.rs, decl: 6, sub: 0, line: 161 } |  |  | 0.522 |
| ns | 9997 |  | 541 | Popup, Header and Footer — types and complete method rosters | 6.6 |  | 0.531 |

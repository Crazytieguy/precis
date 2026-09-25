Score(3000)=0.529 I=0.836 C=0.335 ns_rows≤3K=19/44 grid(1000/1442/2080/3000/4327/6240/9000)=0.871/0.787/0.648/0.529/0.555/0.469/0.503

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
| walker |  | 1009 | 88 | Code::CodeKey { rung: Names, file: build.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.871 |
| walker |  | 1058 | 49 | Code::CodeKey { rung: Body, file: build.rs, decl: 1, sub: 0, line: 8 } |  |  | 0.871 |
| walker |  | 1165 | 107 | Code::CodeKey { rung: Body, file: build.rs, decl: 2, sub: 0, line: 14 } |  |  | 0.871 |
| walker |  | 1186 | 21 | Code::CodeKey { rung: Names, file: src/edit.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.871 |
| ns | 1188 |  | 206 | run(): CLI parse, config resolution, --show-config, --debug | 2.2 | 2.1 | 0.786 |
| walker |  | 1216 | 30 | Code::CodeKey { rung: Decl, file: src/edit.rs, decl: 1, sub: 0, line: 10 } |  |  | 0.786 |
| walker |  | 1266 | 50 | Code::CodeKey { rung: Decl, file: src/edit.rs, decl: 2, sub: 0, line: 16 } |  |  | 0.786 |
| walker |  | 1333 | 67 | Code::CodeKey { rung: Names, file: src/tree.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.786 |
| walker |  | 1373 | 40 | Code::CodeKey { rung: Decl, file: src/tree.rs, decl: 3, sub: 0, line: 35 } |  |  | 0.786 |
| walker |  | 1428 | 55 | Code::CodeKey { rung: Decl, file: src/tree.rs, decl: 4, sub: 0, line: 41 } |  |  | 0.787 |
| walker |  | 1496 | 68 | Code::CodeKey { rung: Decl, file: src/tree.rs, decl: 10, sub: 0, line: 385 } |  |  | 0.787 |
| ns | 1511 |  | 323 | run(): content-type inference, live-reload watcher, reading path or stdin | 2.3 | 2.2 | 0.709 |
| walker |  | 1546 | 50 | Code::CodeKey { rung: Decl, file: src/tree.rs, decl: 13, sub: 0, line: 394 } |  |  | 0.709 |
| walker |  | 1618 | 72 | Code::CodeKey { rung: Decl, file: src/tree.rs, decl: 2, sub: 0, line: 25 } |  |  | 0.710 |
| walker |  | 1703 | 85 | Code::CodeKey { rung: Decl, file: src/tree.rs, decl: 1, sub: 0, line: 14 } |  |  | 0.714 |
| ns | 1790 |  | 279 | run(): the --to conversion short-circuit and the data-size guard | 2.4 | 2.3 | 0.671 |
| walker |  | 1819 | 116 | Code::CodeKey { rung: Decl, file: src/tree.rs, decl: 5, sub: 0, line: 51 } |  |  | 0.672 |
| walker |  | 1829 | 10 | Code::CodeKey { rung: Body, file: src/tree.rs, decl: 8, sub: 0, line: 94 } |  |  | 0.672 |
| walker |  | 1852 | 23 | Code::CodeKey { rung: Names, file: src/cmd.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.672 |
| walker |  | 1912 | 60 | Code::CodeKey { rung: Decl, file: src/cmd.rs, decl: 2, sub: 0, line: 111 } |  |  | 0.672 |
| walker |  | 1924 | 12 | Code::CodeKey { rung: Body, file: src/tree.rs, decl: 9, sub: 0, line: 98 } |  |  | 0.672 |
| ns | 1925 |  | 135 | run(): tree construction, App::new, header context, ui::start | 2.5 | 2.4 | 0.648 |
| walker |  | 1981 | 57 | Code::CodeKey { rung: Names, file: src/live_reload.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.648 |
| walker |  | 2013 | 32 | Code::CodeKey { rung: Decl, file: src/live_reload.rs, decl: 6, sub: 0, line: 93 } |  |  | 0.648 |
| walker |  | 2065 | 52 | Code::CodeKey { rung: Decl, file: src/live_reload.rs, decl: 2, sub: 0, line: 27 } |  |  | 0.648 |
| walker |  | 2117 | 52 | Code::CodeKey { rung: Decl, file: src/live_reload.rs, decl: 3, sub: 0, line: 28 } |  |  | 0.648 |
| walker |  | 2190 | 73 | Code::CodeKey { rung: Decl, file: src/live_reload.rs, decl: 1, sub: 0, line: 16 } |  |  | 0.648 |
| ns | 2233 |  | 308 | ui::start — the draw/edit/quit loop and terminal restoration | 2.6 |  | 0.597 |
| walker |  | 2234 | 44 | Code::CodeKey { rung: Body, file: src/live_reload.rs, decl: 4, sub: 0, line: 64 } |  |  | 0.597 |
| walker |  | 2338 | 104 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.597 |
| walker |  | 2407 | 69 | Code::CodeKey { rung: Names, file: src/debug.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.597 |
| walker |  | 2418 | 11 | Code::CodeKey { rung: Body, file: src/debug.rs, decl: 3, sub: 0, line: 19 } |  |  | 0.597 |
| walker |  | 2517 | 99 | Code::CodeKey { rung: Decl, file: src/debug.rs, decl: 1, sub: 0, line: 7 } |  |  | 0.597 |
| walker |  | 2625 | 108 | Code::CodeKey { rung: Names, file: src/parse/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.597 |
| ns | 2628 |  | 395 | Complete CommandArgs field roster — every CLI flag with its type | 3.1 |  | 0.555 |
| walker |  | 2648 | 23 | Code::CodeKey { rung: Decl, file: src/parse/mod.rs, decl: 8, sub: 0, line: 66 } |  |  | 0.555 |
| walker |  | 2779 | 131 | Code::CodeKey { rung: Decl, file: src/parse/mod.rs, decl: 2, sub: 0, line: 36 } |  |  | 0.556 |
| walker |  | 2973 | 194 | Code::CodeKey { rung: Decl, file: src/parse/mod.rs, decl: 1, sub: 0, line: 17 } |  |  | 0.559 |
| ns | 2983 |  | 355 | The --help text for every flag (CommandArgs doc comments) | 3.2 | 3.1 | 0.529 |
| ns | 3091 |  | 108 | Every short-flag mapping in CommandArgs | 3.3 | 3.1 | 0.520 |
| walker |  | 3099 | 126 | Code::CodeKey { rung: Names, file: src/config/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.520 |
| walker |  | 3142 | 43 | Code::CodeKey { rung: Decl, file: src/config/mod.rs, decl: 7, sub: 0, line: 99 } |  |  | 0.520 |
| walker |  | 3201 | 59 | Code::CodeKey { rung: Decl, file: src/config/mod.rs, decl: 5, sub: 0, line: 82 } |  |  | 0.520 |
| walker |  | 3270 | 69 | Code::CodeKey { rung: Decl, file: src/config/mod.rs, decl: 6, sub: 0, line: 90 } |  |  | 0.520 |
| walker |  | 3346 | 76 | Code::CodeKey { rung: Decl, file: src/config/mod.rs, decl: 2, sub: 0, line: 52 } |  |  | 0.521 |
| walker |  | 3422 | 76 | Code::CodeKey { rung: Decl, file: src/config/mod.rs, decl: 4, sub: 0, line: 73 } |  |  | 0.521 |
| ns | 3448 |  | 357 | get_content_type in full, plus the name of every other CommandArgs method | 3.4 |  | 0.492 |
| walker |  | 3518 | 96 | Code::CodeKey { rung: Decl, file: src/config/mod.rs, decl: 9, sub: 0, line: 117 } |  |  | 0.493 |
| walker |  | 3619 | 101 | Code::CodeKey { rung: Decl, file: src/config/mod.rs, decl: 3, sub: 0, line: 61 } |  |  | 0.494 |
| ns | 3657 |  | 209 | ContentType — the complete set of supported input formats | 4.1 |  | 0.522 |
| walker |  | 3723 | 104 | Code::CodeKey { rung: Decl, file: src/config/mod.rs, decl: 8, sub: 0, line: 105 } |  |  | 0.523 |
| walker |  | 3838 | 115 | Code::CodeKey { rung: Decl, file: src/config/mod.rs, decl: 10, sub: 0, line: 127 } |  |  | 0.523 |
| walker |  | 3852 | 14 | Code::CodeKey { rung: Body, file: src/tree.rs, decl: 11, sub: 0, line: 386 } |  |  | 0.523 |
| walker |  | 3890 | 38 | Code::CodeKey { rung: Names, file: src/clipboard.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.523 |
| walker |  | 3953 | 63 | Code::CodeKey { rung: Names, file: src/ui/app.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.523 |
| walker |  | 3974 | 21 | Code::CodeKey { rung: Decl, file: src/ui/app.rs, decl: 5, sub: 0, line: 86 } |  |  | 0.517 |
| ns | 3974 |  | 317 | The Parser trait surface and ContentType::new_parser | 4.2 | 4.1 | 0.517 |
| walker |  | 4003 | 29 | Code::CodeKey { rung: Decl, file: src/ui/app.rs, decl: 3, sub: 0, line: 47 } |  |  | 0.517 |
| walker |  | 4055 | 52 | Code::CodeKey { rung: Decl, file: src/ui/app.rs, decl: 2, sub: 0, line: 38 } |  |  | 0.518 |
| walker |  | 4122 | 67 | Code::CodeKey { rung: Decl, file: src/ui/app.rs, decl: 6, sub: 0, line: 91 } |  |  | 0.518 |
| walker |  | 4138 | 16 | Code::CodeKey { rung: Decl, file: src/ui/app.rs, decl: 7, sub: 0, line: 101 } |  |  | 0.518 |
| walker |  | 4179 | 41 | Code::CodeKey { rung: Decl, file: src/ui/app.rs, decl: 9, sub: 0, line: 137 } |  |  | 0.518 |
| walker |  | 4269 | 90 | Code::CodeKey { rung: Decl, file: src/ui/app.rs, decl: 1, sub: 0, line: 27 } |  |  | 0.519 |
| ns | 4291 |  | 317 | tree.rs — Tree, ItemValue, HighlightKeyword and FieldType | 4.3 |  | 0.555 |
| ns | 4498 |  | 207 | Parser::parse_root — UTF-8 decode and root-shape validation | 4.4 | 4.2 | 0.541 |
| walker |  | 4548 | 279 | Code::CodeKey { rung: Decl, file: src/ui/app.rs, decl: 4, sub: 0, line: 53 } |  |  | 0.541 |
| walker |  | 4590 | 42 | Code::CodeKey { rung: Names, file: src/ui/header.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.541 |
| walker |  | 4613 | 23 | Code::CodeKey { rung: Decl, file: src/ui/header.rs, decl: 4, sub: 0, line: 41 } |  |  | 0.541 |
| ns | 4631 |  | 133 | SyntaxToken — the complete highlight token vocabulary | 4.5 |  | 0.528 |
| walker |  | 4645 | 32 | Code::CodeKey { rung: Decl, file: src/ui/header.rs, decl: 2, sub: 0, line: 18 } |  |  | 0.528 |
| walker |  | 4693 | 48 | Code::CodeKey { rung: Decl, file: src/ui/header.rs, decl: 1, sub: 0, line: 11 } |  |  | 0.528 |
| walker |  | 4745 | 52 | Code::CodeKey { rung: Decl, file: src/ui/header.rs, decl: 5, sub: 0, line: 46 } |  |  | 0.528 |
| walker |  | 4809 | 64 | Code::CodeKey { rung: Names, file: src/ui/filter.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.528 |
| walker |  | 4834 | 25 | Code::CodeKey { rung: Decl, file: src/ui/filter.rs, decl: 12, sub: 0, line: 152 } |  |  | 0.528 |
| ns | 4864 |  | 233 | JSON, JSONL and YAML parsers — types, extensions, array roots allowed | 4.6 | 4.2 | 0.512 |
| walker |  | 4865 | 31 | Code::CodeKey { rung: Decl, file: src/ui/filter.rs, decl: 2, sub: 0, line: 20 } |  |  | 0.512 |
| walker |  | 4901 | 36 | Code::CodeKey { rung: Decl, file: src/ui/filter.rs, decl: 4, sub: 0, line: 33 } |  |  | 0.512 |
| walker |  | 4936 | 35 | Code::CodeKey { rung: Decl, file: src/ui/filter.rs, decl: 3, sub: 0, line: 27 } |  |  | 0.513 |
| walker |  | 4983 | 47 | Code::CodeKey { rung: Decl, file: src/ui/filter.rs, decl: 1, sub: 0, line: 13 } |  |  | 0.513 |
| ns | 5099 |  | 235 | TOML, XML and HCL parsers — types, extensions, array roots rejected | 4.7 | 4.6 | 0.498 |
| walker |  | 5119 | 136 | Code::CodeKey { rung: Decl, file: src/ui/filter.rs, decl: 5, sub: 0, line: 40 } |  |  | 0.499 |
| walker |  | 5129 | 10 | Code::CodeKey { rung: Body, file: src/ui/filter.rs, decl: 9, sub: 0, line: 102 } |  |  | 0.499 |
| walker |  | 5161 | 32 | Code::CodeKey { rung: Names, file: src/ui/popup.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.499 |
| walker |  | 5190 | 29 | Code::CodeKey { rung: Decl, file: src/ui/popup.rs, decl: 1, sub: 0, line: 13 } |  |  | 0.499 |
| walker |  | 5235 | 45 | Code::CodeKey { rung: Decl, file: src/ui/popup.rs, decl: 2, sub: 0, line: 18 } |  |  | 0.499 |
| ns | 5302 |  | 203 | AnyParser — format auto-detection by trial parsing | 4.8 | 4.7 | 0.489 |
| walker |  | 5377 | 142 | Code::CodeKey { rung: Decl, file: src/ui/popup.rs, decl: 3, sub: 0, line: 26 } |  |  | 0.489 |
| walker |  | 5387 | 10 | Code::CodeKey { rung: Body, file: src/ui/popup.rs, decl: 9, sub: 0, line: 87 } |  |  | 0.489 |
| walker |  | 5398 | 11 | Code::CodeKey { rung: Body, file: src/ui/popup.rs, decl: 5, sub: 0, line: 35 } |  |  | 0.489 |
| ns | 5432 |  | 130 | Every method on Tree and ItemValue (names only) | 4.9 | 4.3 | 0.489 |
| walker |  | 5497 | 99 | Code::CodeKey { rung: Names, file: src/config/colors.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.489 |
| walker |  | 5516 | 19 | Code::CodeKey { rung: Decl, file: src/config/colors.rs, decl: 3, sub: 0, line: 55 } |  |  | 0.489 |
| ns | 5538 |  | 106 | syntax.rs shared helpers — StringValue, quoting, wrapping, splitting | 4.10 | 4.5 | 0.484 |
| walker |  | 5561 | 45 | Code::CodeKey { rung: Decl, file: src/config/colors.rs, decl: 9, sub: 0, line: 323 } |  |  | 0.484 |
| walker |  | 5652 | 91 | Code::CodeKey { rung: Decl, file: src/config/colors.rs, decl: 7, sub: 0, line: 250 } |  |  | 0.484 |
| ns | 5729 |  | 191 | Complete test inventory — every test module and test fn in the crate | 4.11 |  | 0.476 |
| walker |  | 5765 | 113 | Code::CodeKey { rung: Decl, file: src/config/colors.rs, decl: 8, sub: 0, line: 285 } |  |  | 0.476 |
| walker |  | 5885 | 120 | Code::CodeKey { rung: Decl, file: src/config/colors.rs, decl: 1, sub: 0, line: 9 } |  |  | 0.476 |
| ns | 5896 |  | 167 | The Config struct — every configuration section | 5.1 |  | 0.468 |
| walker |  | 6005 | 120 | Code::CodeKey { rung: Decl, file: src/config/colors.rs, decl: 10, sub: 0, line: 341 } |  |  | 0.469 |
| walker |  | 6235 | 230 | Code::CodeKey { rung: Decl, file: src/config/colors.rs, decl: 2, sub: 0, line: 20 } |  |  | 0.469 |
| walker |  | 6302 | 67 | Code::CodeKey { rung: Names, file: src/config/keys.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.469 |
| walker |  | 6331 | 29 | Code::CodeKey { rung: Decl, file: src/config/keys.rs, decl: 6, sub: 0, line: 356 } |  |  | 0.469 |
| ns | 6359 |  | 463 | Every scalar config section struct with its keys | 5.2 | 5.1 | 0.509 |
| walker |  | 6367 | 36 | Code::CodeKey { rung: Decl, file: src/config/keys.rs, decl: 5, sub: 0, line: 350 } |  |  | 0.509 |
| walker |  | 6510 | 143 | Code::CodeKey { rung: Decl, file: src/config/keys.rs, decl: 3, sub: 0, line: 53 } |  |  | 0.509 |
| ns | 6707 |  | 348 | generate_actions! — the complete Action vocabulary | 5.3 |  | 0.494 |
| walker |  | 6708 | 198 | Code::CodeKey { rung: Decl, file: src/config/keys.rs, decl: 2, sub: 0, line: 32 } |  |  | 0.494 |
| walker |  | 6922 | 214 | Code::CodeKey { rung: Decl, file: src/config/keys.rs, decl: 1, sub: 0, line: 7 } |  |  | 0.494 |
| ns | 7113 |  | 406 | The complete colour schema — groups, per-group fields, and the Color type | 5.4 |  | 0.490 |
| ns | 7236 |  | 123 | The Types section — customisable type labels | 5.5 | 5.1 | 0.485 |
| walker |  | 7240 | 318 | Code::CodeKey { rung: Decl, file: src/config/colors.rs, decl: 5, sub: 0, line: 77 } |  |  | 0.485 |
| walker |  | 7254 | 14 | Code::CodeKey { rung: Body, file: src/ui/filter.rs, decl: 10, sub: 0, line: 106 } |  |  | 0.485 |
| walker |  | 7277 | 23 | Code::CodeKey { rung: Names, file: src/ui/data_block.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.485 |
| ns | 7374 |  | 138 | Every method on Config (names only) | 5.6 | 5.1 | 0.485 |
| walker |  | 7405 | 128 | Code::CodeKey { rung: Decl, file: src/ui/data_block.rs, decl: 1, sub: 0, line: 16 } |  |  | 0.485 |
| ns | 7590 |  | 216 | The App control enums — Refresh, ElementInFocus, ScrollDirection | 6.1 |  | 0.500 |
| walker |  | 7609 | 204 | Code::CodeKey { rung: Decl, file: src/ui/data_block.rs, decl: 2, sub: 0, line: 32 } |  |  | 0.500 |
| walker |  | 7670 | 61 | Code::CodeKey { rung: Decl, file: src/ui/data_block.rs, decl: 12, sub: 0, line: 147 } |  |  | 0.500 |
| walker |  | 7716 | 46 | Code::CodeKey { rung: Body, file: src/ui/data_block.rs, decl: 5, sub: 0, line: 62 } |  |  | 0.500 |
| walker |  | 7739 | 23 | Code::CodeKey { rung: Names, file: src/ui/tree_overview.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.500 |
| walker |  | 7882 | 143 | Code::CodeKey { rung: Decl, file: src/ui/tree_overview.rs, decl: 1, sub: 0, line: 19 } |  |  | 0.500 |
| ns | 8139 |  | 549 | Every method on App, plus its layout and polling constants | 6.2 | 6.1 | 0.489 |
| walker |  | 8151 | 269 | Code::CodeKey { rung: Decl, file: src/ui/tree_overview.rs, decl: 2, sub: 0, line: 32 } |  |  | 0.490 |
| walker |  | 8162 | 11 | Code::CodeKey { rung: Body, file: src/ui/tree_overview.rs, decl: 7, sub: 0, line: 67 } |  |  | 0.490 |
| walker |  | 8174 | 12 | Code::CodeKey { rung: Body, file: src/ui/tree_overview.rs, decl: 5, sub: 0, line: 59 } |  |  | 0.490 |
| walker |  | 8186 | 12 | Code::CodeKey { rung: Body, file: src/ui/tree_overview.rs, decl: 6, sub: 0, line: 63 } |  |  | 0.490 |
| walker |  | 8221 | 35 | Code::CodeKey { rung: Names, file: src/parse/any.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.490 |
| walker |  | 8240 | 19 | Code::CodeKey { rung: Decl, file: src/parse/any.rs, decl: 1, sub: 0, line: 9 } |  |  | 0.490 |
| walker |  | 8279 | 39 | Code::CodeKey { rung: Decl, file: src/parse/any.rs, decl: 2, sub: 0, line: 13 } |  |  | 0.490 |
| walker |  | 8294 | 15 | Code::CodeKey { rung: Body, file: src/parse/any.rs, decl: 4, sub: 0, line: 20 } |  |  | 0.490 |
| walker |  | 8415 | 121 | Code::CodeKey { rung: Decl, file: src/parse/any.rs, decl: 5, sub: 0, line: 25 } |  |  | 0.491 |
| walker |  | 8431 | 16 | Code::CodeKey { rung: Body, file: src/parse/any.rs, decl: 6, sub: 0, line: 26 } |  |  | 0.491 |
| walker |  | 8448 | 17 | Code::CodeKey { rung: Body, file: src/parse/any.rs, decl: 8, sub: 0, line: 34 } |  |  | 0.491 |
| walker |  | 8466 | 18 | Code::CodeKey { rung: Body, file: src/parse/any.rs, decl: 7, sub: 0, line: 30 } |  |  | 0.491 |
| walker |  | 8501 | 35 | Code::CodeKey { rung: Names, file: src/ui/footer.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.491 |
| walker |  | 8515 | 14 | Code::CodeKey { rung: Decl, file: src/ui/footer.rs, decl: 2, sub: 0, line: 16 } |  |  | 0.491 |
| walker |  | 8549 | 34 | Code::CodeKey { rung: Decl, file: src/ui/footer.rs, decl: 1, sub: 0, line: 10 } |  |  | 0.491 |
| walker |  | 8600 | 51 | Code::CodeKey { rung: Decl, file: src/ui/footer.rs, decl: 3, sub: 0, line: 20 } |  |  | 0.491 |
| walker |  | 8609 | 9 | Code::CodeKey { rung: Body, file: src/ui/footer.rs, decl: 4, sub: 0, line: 21 } |  |  | 0.491 |
| ns | 8734 |  | 595 | TreeOverview — complete method roster and its two constants | 6.3 |  | 0.486 |
| walker |  | 8945 | 336 | Code::CodeKey { rung: Decl, file: src/config/mod.rs, decl: 1, sub: 0, line: 16 } |  |  | 0.502 |
| walker |  | 8970 | 25 | Code::CodeKey { rung: Names, file: src/config/types.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.502 |
| ns | 9136 |  | 402 | Filter widget — the filter types and complete method roster | 6.4 |  | 0.515 |
| walker |  | 9164 | 194 | Code::CodeKey { rung: Decl, file: src/config/types.rs, decl: 2, sub: 0, line: 27 } |  |  | 0.522 |
| walker |  | 9363 | 199 | Code::CodeKey { rung: Decl, file: src/config/types.rs, decl: 1, sub: 0, line: 3 } |  |  | 0.522 |
| ns | 9456 |  | 320 | DataBlock — complete method roster and the scroll-retain constant | 6.5 |  | 0.522 |
| walker |  | 9724 | 361 | Code::CodeKey { rung: Decl, file: src/config/colors.rs, decl: 6, sub: 0, line: 161 } |  |  | 0.522 |
| walker |  | 9963 | 239 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.522 |
| walker |  | 9997 | 34 | Code::CodeKey { rung: Body, file: src/debug.rs, decl: 4, sub: 0, line: 23 } |  |  | 0.531 |
| ns | 9997 |  | 541 | Popup, Header and Footer — types and complete method rosters | 6.6 |  | 0.531 |

Score(3000)=0.526 I=0.826 C=0.335 ns_rows≤3K=19/44 grid(1000/1442/2080/3000/4327/6240/9000)=0.739/0.759/0.642/0.526/0.503/0.459/0.493

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
| walker |  | 563 | 89 | Code::CodeKey { rung: Names, file: src/main.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.866 |
| ns | 583 |  | 75 | Complete listings of docs/, config/, config/themes/, examples/, assets/ | 1.9 | 1.2 | 0.786 |
| walker |  | 584 | 21 | Code::CodeKey { rung: Names, file: src/edit.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.786 |
| walker |  | 614 | 30 | Code::CodeKey { rung: Decl, file: src/edit.rs, decl: 1, sub: 0, line: 10 } |  |  | 0.786 |
| walker |  | 664 | 50 | Code::CodeKey { rung: Decl, file: src/edit.rs, decl: 2, sub: 0, line: 16 } |  |  | 0.786 |
| walker |  | 687 | 23 | Code::CodeKey { rung: Names, file: src/cmd.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.786 |
| ns | 722 |  | 139 | Cargo [dependencies] — first half | 1.10 |  | 0.731 |
| walker |  | 747 | 60 | Code::CodeKey { rung: Decl, file: src/cmd.rs, decl: 2, sub: 0, line: 111 } |  |  | 0.731 |
| walker |  | 787 | 40 | Fs::DirListing { dir: examples } |  |  | 0.824 |
| walker |  | 875 | 88 | Code::CodeKey { rung: Names, file: build.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.824 |
| ns | 909 |  | 187 | Cargo [dependencies] tail plus [build-dependencies] | 1.11 | 1.10 | 0.764 |
| walker |  | 913 | 38 | Code::CodeKey { rung: Names, file: src/clipboard.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.764 |
| ns | 982 |  | 73 | main.rs entry points — run() signature and main() | 2.1 |  | 0.739 |
| walker |  | 1186 | 273 | Toml::Dependencies { file: Cargo.toml } |  |  | 0.841 |
| ns | 1188 |  | 206 | run(): CLI parse, config resolution, --show-config, --debug | 2.2 | 2.1 | 0.759 |
| walker |  | 1209 | 23 | Code::CodeKey { rung: Names, file: src/ui/data_block.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.759 |
| walker |  | 1232 | 23 | Code::CodeKey { rung: Names, file: src/ui/tree_overview.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.759 |
| walker |  | 1257 | 25 | Code::CodeKey { rung: Names, file: src/config/types.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.759 |
| walker |  | 1382 | 125 | Toml::Config { file: Cargo.toml } |  |  | 0.759 |
| walker |  | 1439 | 57 | Code::CodeKey { rung: Names, file: src/live_reload.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.759 |
| walker |  | 1491 | 52 | Code::CodeKey { rung: Decl, file: src/live_reload.rs, decl: 2, sub: 0, line: 27 } |  |  | 0.759 |
| ns | 1511 |  | 323 | run(): content-type inference, live-reload watcher, reading path or stdin | 2.3 | 2.2 | 0.683 |
| walker |  | 1543 | 52 | Code::CodeKey { rung: Decl, file: src/live_reload.rs, decl: 3, sub: 0, line: 28 } |  |  | 0.683 |
| walker |  | 1616 | 73 | Code::CodeKey { rung: Decl, file: src/live_reload.rs, decl: 1, sub: 0, line: 16 } |  |  | 0.683 |
| walker |  | 1648 | 32 | Code::CodeKey { rung: Decl, file: src/live_reload.rs, decl: 6, sub: 0, line: 93 } |  |  | 0.683 |
| walker |  | 1678 | 30 | Code::CodeKey { rung: Names, file: src/parse/jsonl.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.683 |
| walker |  | 1771 | 93 | Code::CodeKey { rung: Decl, file: src/parse/jsonl.rs, decl: 2, sub: 0, line: 9 } |  |  | 0.683 |
| walker |  | 1778 | 7 | Code::CodeKey { rung: Body, file: src/parse/jsonl.rs, decl: 4, sub: 0, line: 14 } |  |  | 0.684 |
| walker |  | 1786 | 8 | Code::CodeKey { rung: Body, file: src/parse/jsonl.rs, decl: 3, sub: 0, line: 10 } |  |  | 0.684 |
| ns | 1790 |  | 279 | run(): the --to conversion short-circuit and the data-size guard | 2.4 | 2.3 | 0.643 |
| walker |  | 1801 | 15 | Code::CodeKey { rung: Body, file: src/parse/jsonl.rs, decl: 6, sub: 0, line: 35 } |  |  | 0.643 |
| walker |  | 1833 | 32 | Code::CodeKey { rung: Names, file: src/ui/popup.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.643 |
| walker |  | 1862 | 29 | Code::CodeKey { rung: Decl, file: src/ui/popup.rs, decl: 1, sub: 0, line: 13 } |  |  | 0.643 |
| walker |  | 1907 | 45 | Code::CodeKey { rung: Decl, file: src/ui/popup.rs, decl: 2, sub: 0, line: 18 } |  |  | 0.643 |
| ns | 1925 |  | 135 | run(): tree construction, App::new, header context, ui::start | 2.5 | 2.4 | 0.620 |
| walker |  | 1952 | 45 | Code::CodeKey { rung: Body, file: src/main.rs, decl: 2, sub: 0, line: 111 } |  |  | 0.642 |
| walker |  | 2019 | 67 | Code::CodeKey { rung: Names, file: src/tree.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.642 |
| walker |  | 2059 | 40 | Code::CodeKey { rung: Decl, file: src/tree.rs, decl: 3, sub: 0, line: 35 } |  |  | 0.642 |
| walker |  | 2114 | 55 | Code::CodeKey { rung: Decl, file: src/tree.rs, decl: 4, sub: 0, line: 41 } |  |  | 0.643 |
| walker |  | 2182 | 68 | Code::CodeKey { rung: Decl, file: src/tree.rs, decl: 10, sub: 0, line: 385 } |  |  | 0.643 |
| walker |  | 2232 | 50 | Code::CodeKey { rung: Decl, file: src/tree.rs, decl: 13, sub: 0, line: 394 } |  |  | 0.643 |
| ns | 2233 |  | 308 | ui::start — the draw/edit/quit loop and terminal restoration | 2.6 |  | 0.593 |
| walker |  | 2304 | 72 | Code::CodeKey { rung: Decl, file: src/tree.rs, decl: 2, sub: 0, line: 25 } |  |  | 0.594 |
| walker |  | 2389 | 85 | Code::CodeKey { rung: Decl, file: src/tree.rs, decl: 1, sub: 0, line: 14 } |  |  | 0.597 |
| walker |  | 2505 | 116 | Code::CodeKey { rung: Decl, file: src/tree.rs, decl: 5, sub: 0, line: 51 } |  |  | 0.597 |
| walker |  | 2540 | 35 | Code::CodeKey { rung: Names, file: src/parse/any.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.597 |
| walker |  | 2559 | 19 | Code::CodeKey { rung: Decl, file: src/parse/any.rs, decl: 1, sub: 0, line: 9 } |  |  | 0.597 |
| walker |  | 2598 | 39 | Code::CodeKey { rung: Decl, file: src/parse/any.rs, decl: 2, sub: 0, line: 13 } |  |  | 0.597 |
| walker |  | 2613 | 15 | Code::CodeKey { rung: Body, file: src/parse/any.rs, decl: 4, sub: 0, line: 20 } |  |  | 0.597 |
| ns | 2628 |  | 395 | Complete CommandArgs field roster — every CLI flag with its type | 3.1 |  | 0.555 |
| walker |  | 2648 | 35 | Code::CodeKey { rung: Names, file: src/ui/footer.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.555 |
| walker |  | 2662 | 14 | Code::CodeKey { rung: Decl, file: src/ui/footer.rs, decl: 2, sub: 0, line: 16 } |  |  | 0.555 |
| walker |  | 2696 | 34 | Code::CodeKey { rung: Decl, file: src/ui/footer.rs, decl: 1, sub: 0, line: 10 } |  |  | 0.555 |
| walker |  | 2747 | 51 | Code::CodeKey { rung: Decl, file: src/ui/footer.rs, decl: 3, sub: 0, line: 20 } |  |  | 0.556 |
| walker |  | 2756 | 9 | Code::CodeKey { rung: Body, file: src/ui/footer.rs, decl: 4, sub: 0, line: 21 } |  |  | 0.556 |
| walker |  | 2825 | 69 | Code::CodeKey { rung: Names, file: src/debug.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.556 |
| walker |  | 2836 | 11 | Code::CodeKey { rung: Body, file: src/debug.rs, decl: 3, sub: 0, line: 19 } |  |  | 0.556 |
| walker |  | 2935 | 99 | Code::CodeKey { rung: Decl, file: src/debug.rs, decl: 1, sub: 0, line: 7 } |  |  | 0.556 |
| walker |  | 2945 | 10 | Code::CodeKey { rung: Body, file: src/tree.rs, decl: 8, sub: 0, line: 94 } |  |  | 0.556 |
| ns | 2983 |  | 355 | The --help text for every flag (CommandArgs doc comments) | 3.2 | 3.1 | 0.526 |
| walker |  | 3066 | 121 | Code::CodeKey { rung: Decl, file: src/parse/any.rs, decl: 5, sub: 0, line: 25 } |  |  | 0.526 |
| ns | 3091 |  | 108 | Every short-flag mapping in CommandArgs | 3.3 | 3.1 | 0.517 |
| walker |  | 3194 | 128 | Code::CodeKey { rung: Decl, file: src/ui/data_block.rs, decl: 1, sub: 0, line: 16 } |  |  | 0.517 |
| walker |  | 3236 | 42 | Code::CodeKey { rung: Names, file: src/ui/header.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.517 |
| walker |  | 3259 | 23 | Code::CodeKey { rung: Decl, file: src/ui/header.rs, decl: 4, sub: 0, line: 41 } |  |  | 0.517 |
| walker |  | 3291 | 32 | Code::CodeKey { rung: Decl, file: src/ui/header.rs, decl: 2, sub: 0, line: 18 } |  |  | 0.517 |
| walker |  | 3339 | 48 | Code::CodeKey { rung: Decl, file: src/ui/header.rs, decl: 1, sub: 0, line: 11 } |  |  | 0.517 |
| walker |  | 3391 | 52 | Code::CodeKey { rung: Decl, file: src/ui/header.rs, decl: 5, sub: 0, line: 46 } |  |  | 0.517 |
| ns | 3448 |  | 357 | get_content_type in full, plus the name of every other CommandArgs method | 3.4 |  | 0.488 |
| walker |  | 3533 | 142 | Code::CodeKey { rung: Decl, file: src/ui/popup.rs, decl: 3, sub: 0, line: 26 } |  |  | 0.489 |
| walker |  | 3543 | 10 | Code::CodeKey { rung: Body, file: src/ui/popup.rs, decl: 9, sub: 0, line: 87 } |  |  | 0.489 |
| walker |  | 3554 | 11 | Code::CodeKey { rung: Body, file: src/ui/popup.rs, decl: 5, sub: 0, line: 35 } |  |  | 0.489 |
| ns | 3657 |  | 209 | ContentType — the complete set of supported input formats | 4.1 |  | 0.474 |
| walker |  | 3697 | 143 | Code::CodeKey { rung: Decl, file: src/ui/tree_overview.rs, decl: 1, sub: 0, line: 19 } |  |  | 0.474 |
| walker |  | 3713 | 16 | Code::CodeKey { rung: Body, file: src/parse/any.rs, decl: 6, sub: 0, line: 26 } |  |  | 0.474 |
| walker |  | 3771 | 58 | Code::CodeKey { rung: Names, file: src/parse/hcl.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.474 |
| walker |  | 3861 | 90 | Code::CodeKey { rung: Decl, file: src/parse/hcl.rs, decl: 2, sub: 0, line: 10 } |  |  | 0.474 |
| walker |  | 3868 | 7 | Code::CodeKey { rung: Body, file: src/parse/hcl.rs, decl: 4, sub: 0, line: 15 } |  |  | 0.474 |
| walker |  | 3877 | 9 | Code::CodeKey { rung: Body, file: src/parse/hcl.rs, decl: 3, sub: 0, line: 11 } |  |  | 0.474 |
| walker |  | 3891 | 14 | Code::CodeKey { rung: Body, file: src/parse/hcl.rs, decl: 6, sub: 0, line: 23 } |  |  | 0.474 |
| walker |  | 3909 | 18 | Code::CodeKey { rung: Body, file: src/parse/hcl.rs, decl: 5, sub: 0, line: 19 } |  |  | 0.474 |
| walker |  | 3967 | 58 | Code::CodeKey { rung: Names, file: src/parse/json.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.474 |
| ns | 3974 |  | 317 | The Parser trait surface and ContentType::new_parser | 4.2 | 4.1 | 0.456 |
| walker |  | 4058 | 91 | Code::CodeKey { rung: Decl, file: src/parse/json.rs, decl: 2, sub: 0, line: 8 } |  |  | 0.456 |
| walker |  | 4065 | 7 | Code::CodeKey { rung: Body, file: src/parse/json.rs, decl: 4, sub: 0, line: 13 } |  |  | 0.457 |
| walker |  | 4073 | 8 | Code::CodeKey { rung: Body, file: src/parse/json.rs, decl: 3, sub: 0, line: 9 } |  |  | 0.457 |
| walker |  | 4086 | 13 | Code::CodeKey { rung: Body, file: src/parse/json.rs, decl: 6, sub: 0, line: 21 } |  |  | 0.457 |
| walker |  | 4103 | 17 | Code::CodeKey { rung: Body, file: src/parse/json.rs, decl: 5, sub: 0, line: 17 } |  |  | 0.457 |
| walker |  | 4115 | 12 | Code::CodeKey { rung: Body, file: src/tree.rs, decl: 9, sub: 0, line: 98 } |  |  | 0.457 |
| walker |  | 4174 | 59 | Code::CodeKey { rung: Names, file: src/parse/yaml.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.457 |
| walker |  | 4265 | 91 | Code::CodeKey { rung: Decl, file: src/parse/yaml.rs, decl: 2, sub: 0, line: 10 } |  |  | 0.457 |
| walker |  | 4272 | 7 | Code::CodeKey { rung: Body, file: src/parse/yaml.rs, decl: 4, sub: 0, line: 15 } |  |  | 0.457 |
| walker |  | 4280 | 8 | Code::CodeKey { rung: Body, file: src/parse/yaml.rs, decl: 3, sub: 0, line: 11 } |  |  | 0.457 |
| ns | 4291 |  | 317 | tree.rs — Tree, ItemValue, HighlightKeyword and FieldType | 4.3 |  | 0.503 |
| walker |  | 4474 | 194 | Code::CodeKey { rung: Decl, file: src/config/types.rs, decl: 2, sub: 0, line: 27 } |  |  | 0.503 |
| ns | 4498 |  | 207 | Parser::parse_root — UTF-8 decode and root-shape validation | 4.4 | 4.2 | 0.490 |
| walker |  | 4537 | 63 | Code::CodeKey { rung: Names, file: src/parse/toml.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.490 |
| walker |  | 4627 | 90 | Code::CodeKey { rung: Decl, file: src/parse/toml.rs, decl: 2, sub: 0, line: 11 } |  |  | 0.490 |
| ns | 4631 |  | 133 | SyntaxToken — the complete highlight token vocabulary | 4.5 |  | 0.479 |
| walker |  | 4634 | 7 | Code::CodeKey { rung: Body, file: src/parse/toml.rs, decl: 4, sub: 0, line: 16 } |  |  | 0.479 |
| walker |  | 4643 | 9 | Code::CodeKey { rung: Body, file: src/parse/toml.rs, decl: 3, sub: 0, line: 12 } |  |  | 0.479 |
| walker |  | 4696 | 53 | Code::CodeKey { rung: Decl, file: src/parse/toml.rs, decl: 8, sub: 0, line: 88 } |  |  | 0.479 |
| walker |  | 4759 | 63 | Code::CodeKey { rung: Names, file: src/ui/app.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.479 |
| walker |  | 4780 | 21 | Code::CodeKey { rung: Decl, file: src/ui/app.rs, decl: 5, sub: 0, line: 86 } |  |  | 0.479 |
| walker |  | 4809 | 29 | Code::CodeKey { rung: Decl, file: src/ui/app.rs, decl: 3, sub: 0, line: 47 } |  |  | 0.479 |
| ns | 4864 |  | 233 | JSON, JSONL and YAML parsers — types, extensions, array roots allowed | 4.6 | 4.2 | 0.490 |
| walker |  | 4876 | 67 | Code::CodeKey { rung: Decl, file: src/ui/app.rs, decl: 6, sub: 0, line: 91 } |  |  | 0.490 |
| walker |  | 4892 | 16 | Code::CodeKey { rung: Decl, file: src/ui/app.rs, decl: 7, sub: 0, line: 101 } |  |  | 0.490 |
| walker |  | 4933 | 41 | Code::CodeKey { rung: Decl, file: src/ui/app.rs, decl: 9, sub: 0, line: 137 } |  |  | 0.490 |
| walker |  | 4985 | 52 | Code::CodeKey { rung: Decl, file: src/ui/app.rs, decl: 2, sub: 0, line: 38 } |  |  | 0.490 |
| ns | 5099 |  | 235 | TOML, XML and HCL parsers — types, extensions, array roots rejected | 4.7 | 4.6 | 0.486 |
| walker |  | 5189 | 204 | Code::CodeKey { rung: Decl, file: src/ui/data_block.rs, decl: 2, sub: 0, line: 32 } |  |  | 0.486 |
| walker |  | 5250 | 61 | Code::CodeKey { rung: Decl, file: src/ui/data_block.rs, decl: 12, sub: 0, line: 147 } |  |  | 0.486 |
| ns | 5302 |  | 203 | AnyParser — format auto-detection by trial parsing | 4.8 | 4.7 | 0.478 |
| walker |  | 5314 | 64 | Code::CodeKey { rung: Names, file: src/ui/filter.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.478 |
| walker |  | 5339 | 25 | Code::CodeKey { rung: Decl, file: src/ui/filter.rs, decl: 12, sub: 0, line: 152 } |  |  | 0.478 |
| walker |  | 5370 | 31 | Code::CodeKey { rung: Decl, file: src/ui/filter.rs, decl: 2, sub: 0, line: 20 } |  |  | 0.478 |
| walker |  | 5406 | 36 | Code::CodeKey { rung: Decl, file: src/ui/filter.rs, decl: 4, sub: 0, line: 33 } |  |  | 0.478 |
| ns | 5432 |  | 130 | Every method on Tree and ItemValue (names only) | 4.9 | 4.3 | 0.479 |
| walker |  | 5441 | 35 | Code::CodeKey { rung: Decl, file: src/ui/filter.rs, decl: 3, sub: 0, line: 27 } |  |  | 0.479 |
| walker |  | 5488 | 47 | Code::CodeKey { rung: Decl, file: src/ui/filter.rs, decl: 1, sub: 0, line: 13 } |  |  | 0.479 |
| ns | 5538 |  | 106 | syntax.rs shared helpers — StringValue, quoting, wrapping, splitting | 4.10 | 4.5 | 0.474 |
| walker |  | 5624 | 136 | Code::CodeKey { rung: Decl, file: src/ui/filter.rs, decl: 5, sub: 0, line: 40 } |  |  | 0.475 |
| walker |  | 5634 | 10 | Code::CodeKey { rung: Body, file: src/ui/filter.rs, decl: 9, sub: 0, line: 102 } |  |  | 0.475 |
| walker |  | 5678 | 44 | Code::CodeKey { rung: Body, file: src/live_reload.rs, decl: 4, sub: 0, line: 64 } |  |  | 0.475 |
| ns | 5729 |  | 191 | Complete test inventory — every test module and test fn in the crate | 4.11 |  | 0.467 |
| walker |  | 5745 | 67 | Code::CodeKey { rung: Names, file: src/config/keys.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.467 |
| walker |  | 5774 | 29 | Code::CodeKey { rung: Decl, file: src/config/keys.rs, decl: 6, sub: 0, line: 356 } |  |  | 0.467 |
| walker |  | 5810 | 36 | Code::CodeKey { rung: Decl, file: src/config/keys.rs, decl: 5, sub: 0, line: 350 } |  |  | 0.467 |
| ns | 5896 |  | 167 | The Config struct — every configuration section | 5.1 |  | 0.459 |
| walker |  | 5953 | 143 | Code::CodeKey { rung: Decl, file: src/config/keys.rs, decl: 3, sub: 0, line: 53 } |  |  | 0.459 |
| walker |  | 5970 | 17 | Code::CodeKey { rung: Body, file: src/parse/any.rs, decl: 8, sub: 0, line: 34 } |  |  | 0.459 |
| walker |  | 6019 | 49 | Code::CodeKey { rung: Body, file: build.rs, decl: 1, sub: 0, line: 8 } |  |  | 0.459 |
| walker |  | 6123 | 104 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.459 |
| ns | 6359 |  | 463 | Every scalar config section struct with its keys | 5.2 | 5.1 | 0.439 |
| walker |  | 6392 | 269 | Code::CodeKey { rung: Decl, file: src/ui/tree_overview.rs, decl: 2, sub: 0, line: 32 } |  |  | 0.440 |
| walker |  | 6403 | 11 | Code::CodeKey { rung: Body, file: src/ui/tree_overview.rs, decl: 7, sub: 0, line: 67 } |  |  | 0.440 |
| walker |  | 6415 | 12 | Code::CodeKey { rung: Body, file: src/ui/tree_overview.rs, decl: 5, sub: 0, line: 59 } |  |  | 0.440 |
| walker |  | 6427 | 12 | Code::CodeKey { rung: Body, file: src/ui/tree_overview.rs, decl: 6, sub: 0, line: 63 } |  |  | 0.440 |
| walker |  | 6441 | 14 | Code::CodeKey { rung: Body, file: src/tree.rs, decl: 11, sub: 0, line: 386 } |  |  | 0.440 |
| ns | 6707 |  | 348 | generate_actions! — the complete Action vocabulary | 5.3 |  | 0.427 |
| walker |  | 6720 | 279 | Code::CodeKey { rung: Decl, file: src/ui/app.rs, decl: 4, sub: 0, line: 53 } |  |  | 0.427 |
| walker |  | 6766 | 46 | Code::CodeKey { rung: Body, file: src/ui/data_block.rs, decl: 5, sub: 0, line: 62 } |  |  | 0.427 |
| walker |  | 6856 | 90 | Code::CodeKey { rung: Decl, file: src/ui/app.rs, decl: 1, sub: 0, line: 27 } |  |  | 0.428 |
| walker |  | 6874 | 18 | Code::CodeKey { rung: Body, file: src/parse/any.rs, decl: 7, sub: 0, line: 30 } |  |  | 0.428 |
| walker |  | 6973 | 99 | Code::CodeKey { rung: Names, file: src/config/colors.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.428 |
| walker |  | 6992 | 19 | Code::CodeKey { rung: Decl, file: src/config/colors.rs, decl: 3, sub: 0, line: 55 } |  |  | 0.428 |
| walker |  | 7037 | 45 | Code::CodeKey { rung: Decl, file: src/config/colors.rs, decl: 9, sub: 0, line: 323 } |  |  | 0.428 |
| ns | 7113 |  | 406 | The complete colour schema — groups, per-group fields, and the Color type | 5.4 |  | 0.414 |
| walker |  | 7128 | 91 | Code::CodeKey { rung: Decl, file: src/config/colors.rs, decl: 7, sub: 0, line: 250 } |  |  | 0.414 |
| ns | 7236 |  | 123 | The Types section — customisable type labels | 5.5 | 5.1 | 0.420 |
| walker |  | 7241 | 113 | Code::CodeKey { rung: Decl, file: src/config/colors.rs, decl: 8, sub: 0, line: 285 } |  |  | 0.420 |
| walker |  | 7361 | 120 | Code::CodeKey { rung: Decl, file: src/config/colors.rs, decl: 10, sub: 0, line: 341 } |  |  | 0.423 |
| ns | 7374 |  | 138 | Every method on Config (names only) | 5.6 | 5.1 | 0.419 |
| ns | 7590 |  | 216 | The App control enums — Refresh, ElementInFocus, ScrollDirection | 6.1 |  | 0.438 |
| walker |  | 7593 | 232 | Code::CodeKey { rung: Decl, file: src/config/colors.rs, decl: 2, sub: 0, line: 20 } |  |  | 0.448 |
| walker |  | 7911 | 318 | Code::CodeKey { rung: Decl, file: src/config/colors.rs, decl: 5, sub: 0, line: 77 } |  |  | 0.448 |
| walker |  | 8019 | 108 | Code::CodeKey { rung: Names, file: src/parse/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.448 |
| walker |  | 8042 | 23 | Code::CodeKey { rung: Decl, file: src/parse/mod.rs, decl: 8, sub: 0, line: 66 } |  |  | 0.448 |
| ns | 8139 |  | 549 | Every method on App, plus its layout and polling constants | 6.2 | 6.1 | 0.439 |
| walker |  | 8173 | 131 | Code::CodeKey { rung: Decl, file: src/parse/mod.rs, decl: 2, sub: 0, line: 36 } |  |  | 0.446 |
| walker |  | 8367 | 194 | Code::CodeKey { rung: Decl, file: src/parse/mod.rs, decl: 1, sub: 0, line: 17 } |  |  | 0.468 |
| walker |  | 8381 | 14 | Code::CodeKey { rung: Body, file: src/ui/filter.rs, decl: 10, sub: 0, line: 106 } |  |  | 0.468 |
| walker |  | 8507 | 126 | Code::CodeKey { rung: Names, file: src/config/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.469 |
| walker |  | 8550 | 43 | Code::CodeKey { rung: Decl, file: src/config/mod.rs, decl: 7, sub: 0, line: 99 } |  |  | 0.471 |
| walker |  | 8609 | 59 | Code::CodeKey { rung: Decl, file: src/config/mod.rs, decl: 5, sub: 0, line: 82 } |  |  | 0.475 |
| walker |  | 8678 | 69 | Code::CodeKey { rung: Decl, file: src/config/mod.rs, decl: 6, sub: 0, line: 90 } |  |  | 0.479 |
| ns | 8734 |  | 595 | TreeOverview — complete method roster and its two constants | 6.3 |  | 0.474 |
| walker |  | 8754 | 76 | Code::CodeKey { rung: Decl, file: src/config/mod.rs, decl: 2, sub: 0, line: 52 } |  |  | 0.479 |
| walker |  | 8830 | 76 | Code::CodeKey { rung: Decl, file: src/config/mod.rs, decl: 4, sub: 0, line: 73 } |  |  | 0.485 |
| walker |  | 8926 | 96 | Code::CodeKey { rung: Decl, file: src/config/mod.rs, decl: 9, sub: 0, line: 117 } |  |  | 0.493 |
| walker |  | 9027 | 101 | Code::CodeKey { rung: Decl, file: src/config/mod.rs, decl: 3, sub: 0, line: 61 } |  |  | 0.502 |
| walker |  | 9131 | 104 | Code::CodeKey { rung: Decl, file: src/config/mod.rs, decl: 8, sub: 0, line: 105 } |  |  | 0.512 |
| ns | 9136 |  | 402 | Filter widget — the filter types and complete method roster | 6.4 |  | 0.524 |
| walker |  | 9246 | 115 | Code::CodeKey { rung: Decl, file: src/config/mod.rs, decl: 10, sub: 0, line: 127 } |  |  | 0.527 |
| ns | 9456 |  | 320 | DataBlock — complete method roster and the scroll-retain constant | 6.5 |  | 0.527 |
| walker |  | 9582 | 336 | Code::CodeKey { rung: Decl, file: src/config/mod.rs, decl: 1, sub: 0, line: 16 } |  |  | 0.541 |
| walker |  | 9943 | 361 | Code::CodeKey { rung: Decl, file: src/config/colors.rs, decl: 6, sub: 0, line: 161 } |  |  | 0.541 |
| ns | 9997 |  | 541 | Popup, Header and Footer — types and complete method rosters | 6.6 |  | 0.549 |

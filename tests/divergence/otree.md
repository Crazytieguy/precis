Score(3000)=0.526 I=0.826 C=0.335 ns_rows≤3K=19/44 grid(1000/1442/2080/3000/4327/6240/9000)=0.739/0.759/0.642/0.526/0.503/0.459/0.512

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
| walker |  | 1564 | 73 | Code::CodeKey { rung: Decl, file: src/live_reload.rs, decl: 1, sub: 0, line: 16 } |  |  | 0.683 |
| walker |  | 1616 | 52 | Code::CodeKey { rung: Decl, file: src/live_reload.rs, decl: 3, sub: 0, line: 28 } |  |  | 0.683 |
| walker |  | 1648 | 32 | Code::CodeKey { rung: Decl, file: src/live_reload.rs, decl: 6, sub: 0, line: 93 } |  |  | 0.683 |
| walker |  | 1678 | 30 | Code::CodeKey { rung: Names, file: src/parse/jsonl.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.683 |
| walker |  | 1771 | 93 | Code::CodeKey { rung: Decl, file: src/parse/jsonl.rs, decl: 2, sub: 0, line: 9 } |  |  | 0.683 |
| walker |  | 1778 | 7 | Code::CodeKey { rung: Body, file: src/parse/jsonl.rs, decl: 4, sub: 0, line: 14 } |  |  | 0.684 |
| walker |  | 1786 | 8 | Code::CodeKey { rung: Body, file: src/parse/jsonl.rs, decl: 3, sub: 0, line: 10 } |  |  | 0.684 |
| ns | 1790 |  | 279 | run(): the --to conversion short-circuit and the data-size guard | 2.4 | 2.3 | 0.643 |
| walker |  | 1818 | 32 | Code::CodeKey { rung: Names, file: src/ui/popup.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.643 |
| walker |  | 1847 | 29 | Code::CodeKey { rung: Decl, file: src/ui/popup.rs, decl: 1, sub: 0, line: 13 } |  |  | 0.643 |
| walker |  | 1892 | 45 | Code::CodeKey { rung: Decl, file: src/ui/popup.rs, decl: 2, sub: 0, line: 18 } |  |  | 0.643 |
| ns | 1925 |  | 135 | run(): tree construction, App::new, header context, ui::start | 2.5 | 2.4 | 0.620 |
| walker |  | 1937 | 45 | Code::CodeKey { rung: Body, file: src/main.rs, decl: 2, sub: 0, line: 111 } |  |  | 0.642 |
| walker |  | 2004 | 67 | Code::CodeKey { rung: Names, file: src/tree.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.642 |
| walker |  | 2044 | 40 | Code::CodeKey { rung: Decl, file: src/tree.rs, decl: 3, sub: 0, line: 35 } |  |  | 0.642 |
| walker |  | 2099 | 55 | Code::CodeKey { rung: Decl, file: src/tree.rs, decl: 4, sub: 0, line: 41 } |  |  | 0.643 |
| walker |  | 2167 | 68 | Code::CodeKey { rung: Decl, file: src/tree.rs, decl: 10, sub: 0, line: 385 } |  |  | 0.643 |
| ns | 2233 |  | 308 | ui::start — the draw/edit/quit loop and terminal restoration | 2.6 |  | 0.593 |
| walker |  | 2239 | 72 | Code::CodeKey { rung: Decl, file: src/tree.rs, decl: 2, sub: 0, line: 25 } |  |  | 0.594 |
| walker |  | 2324 | 85 | Code::CodeKey { rung: Decl, file: src/tree.rs, decl: 1, sub: 0, line: 14 } |  |  | 0.597 |
| walker |  | 2374 | 50 | Code::CodeKey { rung: Decl, file: src/tree.rs, decl: 13, sub: 0, line: 394 } |  |  | 0.597 |
| walker |  | 2490 | 116 | Code::CodeKey { rung: Decl, file: src/tree.rs, decl: 5, sub: 0, line: 51 } |  |  | 0.597 |
| walker |  | 2525 | 35 | Code::CodeKey { rung: Names, file: src/parse/any.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.597 |
| walker |  | 2544 | 19 | Code::CodeKey { rung: Decl, file: src/parse/any.rs, decl: 1, sub: 0, line: 9 } |  |  | 0.597 |
| walker |  | 2583 | 39 | Code::CodeKey { rung: Decl, file: src/parse/any.rs, decl: 2, sub: 0, line: 13 } |  |  | 0.597 |
| walker |  | 2618 | 35 | Code::CodeKey { rung: Names, file: src/ui/footer.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.597 |
| ns | 2628 |  | 395 | Complete CommandArgs field roster — every CLI flag with its type | 3.1 |  | 0.555 |
| walker |  | 2632 | 14 | Code::CodeKey { rung: Decl, file: src/ui/footer.rs, decl: 2, sub: 0, line: 16 } |  |  | 0.555 |
| walker |  | 2666 | 34 | Code::CodeKey { rung: Decl, file: src/ui/footer.rs, decl: 1, sub: 0, line: 10 } |  |  | 0.555 |
| walker |  | 2717 | 51 | Code::CodeKey { rung: Decl, file: src/ui/footer.rs, decl: 3, sub: 0, line: 20 } |  |  | 0.556 |
| walker |  | 2726 | 9 | Code::CodeKey { rung: Body, file: src/ui/footer.rs, decl: 4, sub: 0, line: 21 } |  |  | 0.556 |
| walker |  | 2795 | 69 | Code::CodeKey { rung: Names, file: src/debug.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.556 |
| walker |  | 2806 | 11 | Code::CodeKey { rung: Body, file: src/debug.rs, decl: 3, sub: 0, line: 19 } |  |  | 0.556 |
| walker |  | 2905 | 99 | Code::CodeKey { rung: Decl, file: src/debug.rs, decl: 1, sub: 0, line: 7 } |  |  | 0.556 |
| ns | 2983 |  | 355 | The --help text for every flag (CommandArgs doc comments) | 3.2 | 3.1 | 0.526 |
| walker |  | 3026 | 121 | Code::CodeKey { rung: Decl, file: src/parse/any.rs, decl: 5, sub: 0, line: 25 } |  |  | 0.526 |
| ns | 3091 |  | 108 | Every short-flag mapping in CommandArgs | 3.3 | 3.1 | 0.517 |
| walker |  | 3154 | 128 | Code::CodeKey { rung: Decl, file: src/ui/data_block.rs, decl: 1, sub: 0, line: 16 } |  |  | 0.517 |
| walker |  | 3196 | 42 | Code::CodeKey { rung: Names, file: src/ui/header.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.517 |
| walker |  | 3219 | 23 | Code::CodeKey { rung: Decl, file: src/ui/header.rs, decl: 4, sub: 0, line: 41 } |  |  | 0.517 |
| walker |  | 3251 | 32 | Code::CodeKey { rung: Decl, file: src/ui/header.rs, decl: 2, sub: 0, line: 18 } |  |  | 0.517 |
| walker |  | 3299 | 48 | Code::CodeKey { rung: Decl, file: src/ui/header.rs, decl: 1, sub: 0, line: 11 } |  |  | 0.517 |
| walker |  | 3351 | 52 | Code::CodeKey { rung: Decl, file: src/ui/header.rs, decl: 5, sub: 0, line: 46 } |  |  | 0.517 |
| ns | 3448 |  | 357 | get_content_type in full, plus the name of every other CommandArgs method | 3.4 |  | 0.488 |
| walker |  | 3493 | 142 | Code::CodeKey { rung: Decl, file: src/ui/popup.rs, decl: 3, sub: 0, line: 26 } |  |  | 0.489 |
| walker |  | 3503 | 10 | Code::CodeKey { rung: Body, file: src/ui/popup.rs, decl: 9, sub: 0, line: 87 } |  |  | 0.489 |
| walker |  | 3646 | 143 | Code::CodeKey { rung: Decl, file: src/ui/tree_overview.rs, decl: 1, sub: 0, line: 19 } |  |  | 0.489 |
| ns | 3657 |  | 209 | ContentType — the complete set of supported input formats | 4.1 |  | 0.474 |
| walker |  | 3661 | 15 | Code::CodeKey { rung: Body, file: src/parse/any.rs, decl: 4, sub: 0, line: 20 } |  |  | 0.474 |
| walker |  | 3676 | 15 | Code::CodeKey { rung: Body, file: src/parse/jsonl.rs, decl: 6, sub: 0, line: 35 } |  |  | 0.474 |
| walker |  | 3734 | 58 | Code::CodeKey { rung: Names, file: src/parse/hcl.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.474 |
| walker |  | 3824 | 90 | Code::CodeKey { rung: Decl, file: src/parse/hcl.rs, decl: 2, sub: 0, line: 10 } |  |  | 0.474 |
| walker |  | 3831 | 7 | Code::CodeKey { rung: Body, file: src/parse/hcl.rs, decl: 4, sub: 0, line: 15 } |  |  | 0.474 |
| walker |  | 3840 | 9 | Code::CodeKey { rung: Body, file: src/parse/hcl.rs, decl: 3, sub: 0, line: 11 } |  |  | 0.474 |
| walker |  | 3854 | 14 | Code::CodeKey { rung: Body, file: src/parse/hcl.rs, decl: 6, sub: 0, line: 23 } |  |  | 0.474 |
| walker |  | 3912 | 58 | Code::CodeKey { rung: Names, file: src/parse/json.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.474 |
| ns | 3974 |  | 317 | The Parser trait surface and ContentType::new_parser | 4.2 | 4.1 | 0.456 |
| walker |  | 4003 | 91 | Code::CodeKey { rung: Decl, file: src/parse/json.rs, decl: 2, sub: 0, line: 8 } |  |  | 0.456 |
| walker |  | 4010 | 7 | Code::CodeKey { rung: Body, file: src/parse/json.rs, decl: 4, sub: 0, line: 13 } |  |  | 0.457 |
| walker |  | 4018 | 8 | Code::CodeKey { rung: Body, file: src/parse/json.rs, decl: 3, sub: 0, line: 9 } |  |  | 0.457 |
| walker |  | 4031 | 13 | Code::CodeKey { rung: Body, file: src/parse/json.rs, decl: 6, sub: 0, line: 21 } |  |  | 0.457 |
| walker |  | 4042 | 11 | Code::CodeKey { rung: Body, file: src/ui/popup.rs, decl: 5, sub: 0, line: 35 } |  |  | 0.457 |
| walker |  | 4101 | 59 | Code::CodeKey { rung: Names, file: src/parse/yaml.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.457 |
| walker |  | 4192 | 91 | Code::CodeKey { rung: Decl, file: src/parse/yaml.rs, decl: 2, sub: 0, line: 10 } |  |  | 0.457 |
| walker |  | 4199 | 7 | Code::CodeKey { rung: Body, file: src/parse/yaml.rs, decl: 4, sub: 0, line: 15 } |  |  | 0.457 |
| walker |  | 4207 | 8 | Code::CodeKey { rung: Body, file: src/parse/yaml.rs, decl: 3, sub: 0, line: 11 } |  |  | 0.457 |
| ns | 4291 |  | 317 | tree.rs — Tree, ItemValue, HighlightKeyword and FieldType | 4.3 |  | 0.503 |
| walker |  | 4401 | 194 | Code::CodeKey { rung: Decl, file: src/config/types.rs, decl: 2, sub: 0, line: 27 } |  |  | 0.503 |
| walker |  | 4464 | 63 | Code::CodeKey { rung: Names, file: src/parse/toml.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.504 |
| ns | 4498 |  | 207 | Parser::parse_root — UTF-8 decode and root-shape validation | 4.4 | 4.2 | 0.490 |
| walker |  | 4554 | 90 | Code::CodeKey { rung: Decl, file: src/parse/toml.rs, decl: 2, sub: 0, line: 11 } |  |  | 0.490 |
| walker |  | 4561 | 7 | Code::CodeKey { rung: Body, file: src/parse/toml.rs, decl: 4, sub: 0, line: 16 } |  |  | 0.490 |
| walker |  | 4570 | 9 | Code::CodeKey { rung: Body, file: src/parse/toml.rs, decl: 3, sub: 0, line: 12 } |  |  | 0.491 |
| walker |  | 4623 | 53 | Code::CodeKey { rung: Decl, file: src/parse/toml.rs, decl: 8, sub: 0, line: 88 } |  |  | 0.491 |
| ns | 4631 |  | 133 | SyntaxToken — the complete highlight token vocabulary | 4.5 |  | 0.479 |
| walker |  | 4686 | 63 | Code::CodeKey { rung: Names, file: src/ui/app.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.479 |
| walker |  | 4707 | 21 | Code::CodeKey { rung: Decl, file: src/ui/app.rs, decl: 5, sub: 0, line: 86 } |  |  | 0.479 |
| walker |  | 4736 | 29 | Code::CodeKey { rung: Decl, file: src/ui/app.rs, decl: 3, sub: 0, line: 47 } |  |  | 0.479 |
| walker |  | 4803 | 67 | Code::CodeKey { rung: Decl, file: src/ui/app.rs, decl: 6, sub: 0, line: 91 } |  |  | 0.480 |
| walker |  | 4819 | 16 | Code::CodeKey { rung: Decl, file: src/ui/app.rs, decl: 7, sub: 0, line: 101 } |  |  | 0.480 |
| walker |  | 4860 | 41 | Code::CodeKey { rung: Decl, file: src/ui/app.rs, decl: 9, sub: 0, line: 137 } |  |  | 0.480 |
| ns | 4864 |  | 233 | JSON, JSONL and YAML parsers — types, extensions, array roots allowed | 4.6 | 4.2 | 0.490 |
| walker |  | 4912 | 52 | Code::CodeKey { rung: Decl, file: src/ui/app.rs, decl: 2, sub: 0, line: 38 } |  |  | 0.490 |
| ns | 5099 |  | 235 | TOML, XML and HCL parsers — types, extensions, array roots rejected | 4.7 | 4.6 | 0.486 |
| walker |  | 5116 | 204 | Code::CodeKey { rung: Decl, file: src/ui/data_block.rs, decl: 2, sub: 0, line: 32 } |  |  | 0.486 |
| walker |  | 5177 | 61 | Code::CodeKey { rung: Decl, file: src/ui/data_block.rs, decl: 12, sub: 0, line: 147 } |  |  | 0.486 |
| walker |  | 5241 | 64 | Code::CodeKey { rung: Names, file: src/ui/filter.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.486 |
| walker |  | 5266 | 25 | Code::CodeKey { rung: Decl, file: src/ui/filter.rs, decl: 12, sub: 0, line: 152 } |  |  | 0.486 |
| walker |  | 5297 | 31 | Code::CodeKey { rung: Decl, file: src/ui/filter.rs, decl: 2, sub: 0, line: 20 } |  |  | 0.487 |
| ns | 5302 |  | 203 | AnyParser — format auto-detection by trial parsing | 4.8 | 4.7 | 0.478 |
| walker |  | 5333 | 36 | Code::CodeKey { rung: Decl, file: src/ui/filter.rs, decl: 4, sub: 0, line: 33 } |  |  | 0.478 |
| walker |  | 5368 | 35 | Code::CodeKey { rung: Decl, file: src/ui/filter.rs, decl: 3, sub: 0, line: 27 } |  |  | 0.478 |
| walker |  | 5415 | 47 | Code::CodeKey { rung: Decl, file: src/ui/filter.rs, decl: 1, sub: 0, line: 13 } |  |  | 0.479 |
| ns | 5432 |  | 130 | Every method on Tree and ItemValue (names only) | 4.9 | 4.3 | 0.479 |
| ns | 5538 |  | 106 | syntax.rs shared helpers — StringValue, quoting, wrapping, splitting | 4.10 | 4.5 | 0.474 |
| walker |  | 5551 | 136 | Code::CodeKey { rung: Decl, file: src/ui/filter.rs, decl: 5, sub: 0, line: 40 } |  |  | 0.475 |
| walker |  | 5618 | 67 | Code::CodeKey { rung: Names, file: src/config/keys.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.475 |
| walker |  | 5647 | 29 | Code::CodeKey { rung: Decl, file: src/config/keys.rs, decl: 6, sub: 0, line: 356 } |  |  | 0.475 |
| walker |  | 5683 | 36 | Code::CodeKey { rung: Decl, file: src/config/keys.rs, decl: 5, sub: 0, line: 350 } |  |  | 0.475 |
| ns | 5729 |  | 191 | Complete test inventory — every test module and test fn in the crate | 4.11 |  | 0.467 |
| walker |  | 5826 | 143 | Code::CodeKey { rung: Decl, file: src/config/keys.rs, decl: 3, sub: 0, line: 53 } |  |  | 0.467 |
| walker |  | 5875 | 49 | Code::CodeKey { rung: Body, file: build.rs, decl: 1, sub: 0, line: 8 } |  |  | 0.467 |
| walker |  | 5885 | 10 | Code::CodeKey { rung: Body, file: src/tree.rs, decl: 8, sub: 0, line: 94 } |  |  | 0.467 |
| ns | 5896 |  | 167 | The Config struct — every configuration section | 5.1 |  | 0.459 |
| walker |  | 5989 | 104 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.459 |
| walker |  | 6258 | 269 | Code::CodeKey { rung: Decl, file: src/ui/tree_overview.rs, decl: 2, sub: 0, line: 32 } |  |  | 0.460 |
| walker |  | 6269 | 11 | Code::CodeKey { rung: Body, file: src/ui/tree_overview.rs, decl: 7, sub: 0, line: 67 } |  |  | 0.460 |
| walker |  | 6281 | 12 | Code::CodeKey { rung: Body, file: src/ui/tree_overview.rs, decl: 5, sub: 0, line: 59 } |  |  | 0.460 |
| walker |  | 6293 | 12 | Code::CodeKey { rung: Body, file: src/ui/tree_overview.rs, decl: 6, sub: 0, line: 63 } |  |  | 0.460 |
| walker |  | 6309 | 16 | Code::CodeKey { rung: Body, file: src/parse/any.rs, decl: 6, sub: 0, line: 26 } |  |  | 0.460 |
| ns | 6359 |  | 463 | Every scalar config section struct with its keys | 5.2 | 5.1 | 0.440 |
| walker |  | 6588 | 279 | Code::CodeKey { rung: Decl, file: src/ui/app.rs, decl: 4, sub: 0, line: 53 } |  |  | 0.440 |
| walker |  | 6605 | 17 | Code::CodeKey { rung: Body, file: src/parse/json.rs, decl: 5, sub: 0, line: 17 } |  |  | 0.440 |
| walker |  | 6695 | 90 | Code::CodeKey { rung: Decl, file: src/ui/app.rs, decl: 1, sub: 0, line: 27 } |  |  | 0.440 |
| ns | 6707 |  | 348 | generate_actions! — the complete Action vocabulary | 5.3 |  | 0.428 |
| walker |  | 6713 | 18 | Code::CodeKey { rung: Body, file: src/parse/hcl.rs, decl: 5, sub: 0, line: 19 } |  |  | 0.428 |
| walker |  | 6812 | 99 | Code::CodeKey { rung: Names, file: src/config/colors.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.428 |
| walker |  | 6831 | 19 | Code::CodeKey { rung: Decl, file: src/config/colors.rs, decl: 3, sub: 0, line: 55 } |  |  | 0.428 |
| walker |  | 6876 | 45 | Code::CodeKey { rung: Decl, file: src/config/colors.rs, decl: 9, sub: 0, line: 323 } |  |  | 0.428 |
| walker |  | 6967 | 91 | Code::CodeKey { rung: Decl, file: src/config/colors.rs, decl: 7, sub: 0, line: 250 } |  |  | 0.428 |
| walker |  | 7080 | 113 | Code::CodeKey { rung: Decl, file: src/config/colors.rs, decl: 8, sub: 0, line: 285 } |  |  | 0.428 |
| ns | 7113 |  | 406 | The complete colour schema — groups, per-group fields, and the Color type | 5.4 |  | 0.414 |
| walker |  | 7200 | 120 | Code::CodeKey { rung: Decl, file: src/config/colors.rs, decl: 10, sub: 0, line: 341 } |  |  | 0.417 |
| ns | 7236 |  | 123 | The Types section — customisable type labels | 5.5 | 5.1 | 0.423 |
| ns | 7374 |  | 138 | Every method on Config (names only) | 5.6 | 5.1 | 0.419 |
| walker |  | 7432 | 232 | Code::CodeKey { rung: Decl, file: src/config/colors.rs, decl: 2, sub: 0, line: 20 } |  |  | 0.429 |
| ns | 7590 |  | 216 | The App control enums — Refresh, ElementInFocus, ScrollDirection | 6.1 |  | 0.448 |
| walker |  | 7750 | 318 | Code::CodeKey { rung: Decl, file: src/config/colors.rs, decl: 5, sub: 0, line: 77 } |  |  | 0.448 |
| walker |  | 7858 | 108 | Code::CodeKey { rung: Names, file: src/parse/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.448 |
| walker |  | 7881 | 23 | Code::CodeKey { rung: Decl, file: src/parse/mod.rs, decl: 8, sub: 0, line: 66 } |  |  | 0.448 |
| walker |  | 8012 | 131 | Code::CodeKey { rung: Decl, file: src/parse/mod.rs, decl: 2, sub: 0, line: 36 } |  |  | 0.456 |
| ns | 8139 |  | 549 | Every method on App, plus its layout and polling constants | 6.2 | 6.1 | 0.446 |
| walker |  | 8206 | 194 | Code::CodeKey { rung: Decl, file: src/parse/mod.rs, decl: 1, sub: 0, line: 17 } |  |  | 0.468 |
| walker |  | 8216 | 10 | Code::CodeKey { rung: Body, file: src/ui/filter.rs, decl: 9, sub: 0, line: 102 } |  |  | 0.468 |
| walker |  | 8342 | 126 | Code::CodeKey { rung: Names, file: src/config/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.469 |
| walker |  | 8385 | 43 | Code::CodeKey { rung: Decl, file: src/config/mod.rs, decl: 7, sub: 0, line: 99 } |  |  | 0.471 |
| walker |  | 8444 | 59 | Code::CodeKey { rung: Decl, file: src/config/mod.rs, decl: 5, sub: 0, line: 82 } |  |  | 0.475 |
| walker |  | 8513 | 69 | Code::CodeKey { rung: Decl, file: src/config/mod.rs, decl: 6, sub: 0, line: 90 } |  |  | 0.479 |
| walker |  | 8589 | 76 | Code::CodeKey { rung: Decl, file: src/config/mod.rs, decl: 2, sub: 0, line: 52 } |  |  | 0.484 |
| walker |  | 8665 | 76 | Code::CodeKey { rung: Decl, file: src/config/mod.rs, decl: 4, sub: 0, line: 73 } |  |  | 0.490 |
| ns | 8734 |  | 595 | TreeOverview — complete method roster and its two constants | 6.3 |  | 0.485 |
| walker |  | 8761 | 96 | Code::CodeKey { rung: Decl, file: src/config/mod.rs, decl: 9, sub: 0, line: 117 } |  |  | 0.493 |
| walker |  | 8862 | 101 | Code::CodeKey { rung: Decl, file: src/config/mod.rs, decl: 3, sub: 0, line: 61 } |  |  | 0.502 |
| walker |  | 8966 | 104 | Code::CodeKey { rung: Decl, file: src/config/mod.rs, decl: 8, sub: 0, line: 105 } |  |  | 0.512 |
| walker |  | 9081 | 115 | Code::CodeKey { rung: Decl, file: src/config/mod.rs, decl: 10, sub: 0, line: 127 } |  |  | 0.515 |
| ns | 9136 |  | 402 | Filter widget — the filter types and complete method roster | 6.4 |  | 0.527 |
| walker |  | 9417 | 336 | Code::CodeKey { rung: Decl, file: src/config/mod.rs, decl: 1, sub: 0, line: 16 } |  |  | 0.541 |
| walker |  | 9429 | 12 | Code::CodeKey { rung: Body, file: src/tree.rs, decl: 9, sub: 0, line: 98 } |  |  | 0.541 |
| ns | 9456 |  | 320 | DataBlock — complete method roster and the scroll-retain constant | 6.5 |  | 0.541 |
| walker |  | 9790 | 361 | Code::CodeKey { rung: Decl, file: src/config/colors.rs, decl: 6, sub: 0, line: 161 } |  |  | 0.541 |
| ns | 9997 |  | 541 | Popup, Header and Footer — types and complete method rosters | 6.6 |  | 0.549 |

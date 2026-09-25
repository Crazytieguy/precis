Score(3000)=0.779 I=0.928 C=0.654 ns_rows≤3K=19/49 grid(1000/1442/2080/3000/4327/6240/9000)=0.740/0.734/0.731/0.779/0.652/0.555/0.526

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 48 | 48 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 51 | 3 | Fs::DirListing { dir: src } |  |  | 0.000 |
| walker |  | 54 | 3 | Fs::DirListing { dir: .codex } |  |  | 0.000 |
| ns | 91 |  | 91 | README lede: what Posting is | 1.1 |  | 0.000 |
| walker |  | 94 | 40 | Fs::DirListing { dir: docs } |  |  | 0.000 |
| walker |  | 98 | 4 | Fs::DirListing { dir: docs/overrides } |  |  | 0.000 |
| walker |  | 102 | 4 | Fs::DirListing { dir: docs/stylesheets } |  |  | 0.000 |
| ns | 139 |  | 48 | Repository root listing | 1.2 |  | 0.539 |
| walker |  | 177 | 75 | Toml::Identity { file: pyproject.toml } |  |  | 0.542 |
| walker |  | 185 | 8 | Fs::DirListing { dir: .github } |  |  | 0.542 |
| walker |  | 197 | 12 | Fs::DirListing { dir: .github/workflows } |  |  | 0.542 |
| walker |  | 221 | 24 | Toml::Operational { file: pyproject.toml } |  |  | 0.545 |
| ns | 243 |  | 104 | README feature list, part 1: in-app capabilities | 1.3 |  | 0.419 |
| ns | 343 |  | 100 | README feature list, part 2: interop and the command palette | 1.4 |  | 0.370 |
| walker |  | 440 | 219 | Plaintext::Whole { file: Makefile } |  |  | 0.372 |
| ns | 501 |  | 158 | `src/posting/` module roster | 1.5 |  | 0.254 |
| walker |  | 531 | 91 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.468 |
| walker |  | 573 | 42 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.468 |
| ns | 604 |  | 103 | Widget package and importer package listings | 1.6 |  | 0.403 |
| walker |  | 629 | 56 | Fs::DirListing { dir: docs/guide } |  |  | 0.409 |
| ns | 750 |  | 146 | Package identity, build backend and console-script entry point | 1.7 |  | 0.401 |
| walker |  | 787 | 158 | Fs::DirListing { dir: src/posting } |  |  | 0.671 |
| walker |  | 803 | 16 | Code::CodeKey { rung: ModuleDoc, file: src/posting/__main__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.671 |
| walker |  | 817 | 14 | Fs::DirListing { dir: src/posting/importing } |  |  | 0.674 |
| walker |  | 906 | 89 | Fs::DirListing { dir: src/posting/widgets } |  |  | 0.805 |
| walker |  | 936 | 30 | Fs::DirListing { dir: src/posting/widgets/response } |  |  | 0.740 |
| ns | 936 |  | 186 | Runtime dependency pins | 1.8 | 1.7 | 0.740 |
| walker |  | 1004 | 68 | Fs::DirListing { dir: src/posting/widgets/request } |  |  | 0.748 |
| walker |  | 1014 | 10 | Fs::DirListing { dir: src/posting/widgets/collection } |  |  | 0.751 |
| ns | 1044 |  | 108 | UI sub-package listings: request, response, collection | 1.9 |  | 0.771 |
| walker |  | 1100 | 86 | Fs::DirListing { dir: docs/assets } |  |  | 0.771 |
| walker |  | 1110 | 10 | Code::CodeKey { rung: Names, file: src/posting/exit_codes.py, decl: 0, sub: 0, line: 0 } |  |  | 0.771 |
| walker |  | 1122 | 12 | Code::CodeKey { rung: Names, file: src/posting/help_data.py, decl: 0, sub: 0, line: 0 } |  |  | 0.771 |
| walker |  | 1134 | 12 | Code::CodeKey { rung: Names, file: src/posting/version.py, decl: 0, sub: 0, line: 0 } |  |  | 0.771 |
| ns | 1140 |  | 96 | Documentation tree listing | 1.10 |  | 0.785 |
| walker |  | 1148 | 14 | Code::CodeKey { rung: Names, file: src/posting/messages.py, decl: 0, sub: 0, line: 0 } |  |  | 0.785 |
| walker |  | 1163 | 15 | Code::CodeKey { rung: Decl, file: src/posting/messages.py, decl: 1, sub: 0, line: 6 } |  |  | 0.785 |
| walker |  | 1178 | 15 | Code::CodeKey { rung: Names, file: src/posting/_start_time.py, decl: 0, sub: 0, line: 0 } |  |  | 0.785 |
| ns | 1270 |  | 130 | Test tree and sample-collection listings | 1.11 |  | 0.731 |
| walker |  | 1315 | 137 | Code::CodeKey { rung: Names, file: src/posting/__main__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.732 |
| walker |  | 1327 | 12 | Code::CodeKey { rung: Decl, file: src/posting/__main__.py, decl: 7, sub: 0, line: 177 } |  |  | 0.732 |
| walker |  | 1349 | 22 | Code::CodeKey { rung: Decl, file: src/posting/__main__.py, decl: 3, sub: 0, line: 45 } |  |  | 0.733 |
| walker |  | 1381 | 32 | Code::CodeKey { rung: Decl, file: src/posting/__main__.py, decl: 5, sub: 0, line: 76 } |  |  | 0.733 |
| walker |  | 1425 | 44 | Code::CodeKey { rung: Decl, file: src/posting/__main__.py, decl: 8, sub: 0, line: 194 } |  |  | 0.733 |
| walker |  | 1437 | 12 | Code::CodeKey { rung: Doc, file: src/posting/__main__.py, decl: 7, sub: 0, line: 177 } |  |  | 0.734 |
| walker |  | 1451 | 14 | Code::CodeKey { rung: Doc, file: src/posting/__main__.py, decl: 3, sub: 0, line: 45 } |  |  | 0.734 |
| ns | 1460 |  | 190 | Complete `posting` subcommand roster | 2.1 |  | 0.735 |
| walker |  | 1468 | 17 | Code::CodeKey { rung: Doc, file: src/posting/__main__.py, decl: 8, sub: 0, line: 194 } |  |  | 0.736 |
| walker |  | 1595 | 127 | Code::CodeKey { rung: Decl, file: src/posting/__main__.py, decl: 4, sub: 0, line: 50 } |  |  | 0.738 |
| ns | 1689 |  | 229 | Every CLI argument and option | 2.2 |  | 0.717 |
| walker |  | 1727 | 132 | Code::CodeKey { rung: Decl, file: src/posting/__main__.py, decl: 6, sub: 0, line: 96 } |  |  | 0.754 |
| walker |  | 1743 | 16 | Code::CodeKey { rung: Doc, file: src/posting/__main__.py, decl: 6, sub: 0, line: 96 } |  |  | 0.760 |
| walker |  | 1784 | 41 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.760 |
| walker |  | 1801 | 17 | Code::CodeKey { rung: Names, file: src/posting/suggesters.py, decl: 0, sub: 0, line: 0 } |  |  | 0.760 |
| walker |  | 1818 | 17 | Code::CodeKey { rung: Names, file: src/posting/user_host.py, decl: 0, sub: 0, line: 0 } |  |  | 0.760 |
| walker |  | 1836 | 18 | Code::CodeKey { rung: Names, file: src/posting/auth.py, decl: 0, sub: 0, line: 0 } |  |  | 0.760 |
| walker |  | 1882 | 46 | Code::CodeKey { rung: Decl, file: src/posting/auth.py, decl: 1, sub: 0, line: 6 } |  |  | 0.760 |
| walker |  | 1890 | 8 | Code::CodeKey { rung: Body, file: src/posting/auth.py, decl: 2, sub: 0, line: 7 } |  |  | 0.760 |
| ns | 1907 |  | 218 | `make_posting`: CLI-to-app wiring | 2.3 |  | 0.738 |
| walker |  | 1949 | 59 | Code::CodeKey { rung: Decl, file: src/posting/help_data.py, decl: 1, sub: 0, line: 4 } |  |  | 0.738 |
| walker |  | 2037 | 88 | Toml::PackageMetadata { file: pyproject.toml } |  |  | 0.749 |
| ns | 2052 |  | 145 | XDG locations: config file, themes and default collection | 2.4 |  | 0.731 |
| walker |  | 2057 | 20 | Code::CodeKey { rung: Names, file: src/posting/jump_overlay.py, decl: 0, sub: 0, line: 0 } |  |  | 0.731 |
| walker |  | 2112 | 55 | Markdown::HeadingsOutline { file: docs/roadmap.md } |  |  | 0.731 |
| walker |  | 2124 | 12 | Code::CodeKey { rung: Names, file: src/posting/importing/curl.py, decl: 0, sub: 0, line: 0 } |  |  | 0.731 |
| walker |  | 2223 | 99 | Fs::DirListing { dir: tests } |  |  | 0.766 |
| walker |  | 2236 | 13 | Code::CodeKey { rung: Names, file: src/posting/widgets/input.py, decl: 0, sub: 0, line: 0 } |  |  | 0.766 |
| ns | 2359 |  | 307 | collection.py type roster | 3.1 |  | 0.728 |
| walker |  | 2534 | 298 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.741 |
| ns | 2534 |  | 175 | The `.posting.yaml` on-disk request format | 3.2 |  | 0.741 |
| walker |  | 2584 | 50 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.741 |
| walker |  | 2632 | 48 | Code::CodeKey { rung: Decl, file: src/posting/widgets/input.py, decl: 1, sub: 0, line: 9 } |  |  | 0.741 |
| walker |  | 2640 | 8 | Code::CodeKey { rung: Decl, file: src/posting/widgets/input.py, decl: 3, sub: 0, line: 18 } |  |  | 0.741 |
| walker |  | 2655 | 15 | Code::CodeKey { rung: Names, file: src/posting/widgets/variable_input.py, decl: 0, sub: 0, line: 0 } |  |  | 0.741 |
| ns | 2774 |  | 240 | RequestModel field roster | 3.3 | 3.1 | 0.716 |
| walker |  | 2912 | 257 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.791 |
| ns | 2934 |  | 160 | RequestModel behaviour roster | 3.4 |  | 0.779 |
| walker |  | 3138 | 226 | Code::CodeKey { rung: Names, file: src/posting/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.779 |
| ns | 3145 |  | 211 | Key-value, option and script models | 3.5 | 3.1 | 0.751 |
| walker |  | 3154 | 16 | Code::CodeKey { rung: Names, file: src/posting/widgets/tabbed_content.py, decl: 0, sub: 0, line: 0 } |  |  | 0.751 |
| walker |  | 3170 | 16 | Code::CodeKey { rung: Names, file: src/posting/widgets/variable_autocomplete.py, decl: 0, sub: 0, line: 0 } |  |  | 0.751 |
| walker |  | 3199 | 29 | Code::CodeKey { rung: Names, file: src/posting/types.py, decl: 0, sub: 0, line: 0 } |  |  | 0.751 |
| walker |  | 3271 | 72 | Code::CodeKey { rung: Decl, file: src/posting/types.py, decl: 2, sub: 0, line: 6 } |  |  | 0.751 |
| walker |  | 3288 | 17 | Code::CodeKey { rung: Names, file: src/posting/widgets/confirmation.py, decl: 0, sub: 0, line: 0 } |  |  | 0.751 |
| walker |  | 3305 | 17 | Code::CodeKey { rung: Doc, file: src/posting/help_data.py, decl: 1, sub: 0, line: 4 } |  |  | 0.751 |
| walker |  | 3323 | 18 | Code::CodeKey { rung: Names, file: src/posting/widgets/datatable.py, decl: 0, sub: 0, line: 0 } |  |  | 0.751 |
| ns | 3349 |  | 204 | Collection tree: loading a directory, saving it back | 3.6 | 3.1 | 0.730 |
| walker |  | 3354 | 31 | Code::CodeKey { rung: Names, file: src/posting/request_headers.py, decl: 0, sub: 0, line: 0 } |  |  | 0.730 |
| walker |  | 3397 | 43 | Code::CodeKey { rung: Decl, file: src/posting/request_headers.py, decl: 1, sub: 0, line: 4 } |  |  | 0.730 |
| walker |  | 3416 | 19 | Code::CodeKey { rung: Names, file: src/posting/widgets/center_middle.py, decl: 0, sub: 0, line: 0 } |  |  | 0.730 |
| walker |  | 3478 | 62 | Code::CodeKey { rung: Decl, file: src/posting/widgets/center_middle.py, decl: 1, sub: 0, line: 4 } |  |  | 0.730 |
| walker |  | 3512 | 34 | Code::CodeKey { rung: Names, file: src/posting/xresources.py, decl: 0, sub: 0, line: 0 } |  |  | 0.730 |
| ns | 3563 |  | 214 | config.py structure and settings-source configuration | 4.1 |  | 0.710 |
| walker |  | 3613 | 101 | Code::CodeKey { rung: Decl, file: src/posting/xresources.py, decl: 1, sub: 0, line: 8 } |  |  | 0.710 |
| ns | 3714 |  | 151 | Settings keys, part 1 | 4.2 |  | 0.696 |
| walker |  | 3723 | 110 | Code::CodeKey { rung: Decl, file: src/posting/suggesters.py, decl: 1, sub: 0, line: 2 } |  |  | 0.696 |
| walker |  | 3757 | 34 | Markdown::Section { file: docs/index.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.696 |
| walker |  | 3794 | 37 | Code::CodeKey { rung: Names, file: src/posting/jumper.py, decl: 0, sub: 0, line: 0 } |  |  | 0.696 |
| walker |  | 3813 | 19 | Code::CodeKey { rung: Decl, file: src/posting/jumper.py, decl: 1, sub: 0, line: 9 } |  |  | 0.696 |
| walker |  | 3864 | 51 | Code::CodeKey { rung: Decl, file: src/posting/jumper.py, decl: 3, sub: 0, line: 26 } |  |  | 0.696 |
| ns | 3892 |  | 178 | Settings keys, part 2 | 4.3 | 4.2 | 0.683 |
| walker |  | 3920 | 56 | Code::CodeKey { rung: Decl, file: src/posting/jumper.py, decl: 2, sub: 0, line: 16 } |  |  | 0.683 |
| walker |  | 3933 | 13 | Code::CodeKey { rung: Doc, file: src/posting/jumper.py, decl: 1, sub: 0, line: 9 } |  |  | 0.683 |
| walker |  | 3947 | 14 | Code::CodeKey { rung: Doc, file: src/posting/jumper.py, decl: 2, sub: 0, line: 16 } |  |  | 0.683 |
| walker |  | 3961 | 14 | Code::CodeKey { rung: Doc, file: src/posting/jumper.py, decl: 3, sub: 0, line: 26 } |  |  | 0.683 |
| walker |  | 3976 | 15 | Code::CodeKey { rung: Doc, file: src/posting/jumper.py, decl: 5, sub: 0, line: 34 } |  |  | 0.683 |
| walker |  | 3990 | 14 | Code::CodeKey { rung: Names, file: src/posting/widgets/response/cookies_table.py, decl: 0, sub: 0, line: 0 } |  |  | 0.683 |
| walker |  | 4003 | 13 | Code::CodeKey { rung: Doc, file: src/posting/widgets/center_middle.py, decl: 1, sub: 0, line: 4 } |  |  | 0.683 |
| walker |  | 4018 | 15 | Code::CodeKey { rung: Names, file: src/posting/widgets/request/request_metadata.py, decl: 0, sub: 0, line: 0 } |  |  | 0.683 |
| walker |  | 4033 | 15 | Code::CodeKey { rung: Names, file: src/posting/widgets/request/request_options.py, decl: 0, sub: 0, line: 0 } |  |  | 0.683 |
| walker |  | 4056 | 23 | Code::CodeKey { rung: Doc, file: src/posting/xresources.py, decl: 2, sub: 0, line: 20 } |  |  | 0.683 |
| walker |  | 4072 | 16 | Code::CodeKey { rung: Names, file: src/posting/widgets/request/method_selection.py, decl: 0, sub: 0, line: 0 } |  |  | 0.683 |
| walker |  | 4115 | 43 | Code::CodeKey { rung: Names, file: src/posting/commands.py, decl: 0, sub: 0, line: 0 } |  |  | 0.683 |
| walker |  | 4174 | 59 | Code::CodeKey { rung: Decl, file: src/posting/commands.py, decl: 2, sub: 0, line: 14 } |  |  | 0.683 |
| walker |  | 4182 | 8 | Code::CodeKey { rung: Decl, file: src/posting/commands.py, decl: 6, sub: 0, line: 237 } |  |  | 0.683 |
| walker |  | 4218 | 36 | Code::CodeKey { rung: Decl, file: src/posting/commands.py, decl: 3, sub: 0, line: 15 } |  |  | 0.683 |
| walker |  | 4230 | 12 | Code::CodeKey { rung: Body, file: src/posting/commands.py, decl: 6, sub: 0, line: 237 } |  |  | 0.683 |
| ns | 4250 |  | 358 | Configuration precedence and guide section map | 4.4 |  | 0.652 |
| walker |  | 4254 | 24 | Code::CodeKey { rung: Body, file: src/posting/auth.py, decl: 3, sub: 0, line: 10 } |  |  | 0.652 |
| walker |  | 4271 | 17 | Code::CodeKey { rung: Names, file: src/posting/widgets/response/response_body.py, decl: 0, sub: 0, line: 0 } |  |  | 0.652 |
| walker |  | 4288 | 17 | Code::CodeKey { rung: Names, file: src/posting/widgets/response/response_headers.py, decl: 0, sub: 0, line: 0 } |  |  | 0.652 |
| walker |  | 4301 | 13 | Code::CodeKey { rung: Decl, file: src/posting/widgets/response/response_headers.py, decl: 1, sub: 0, line: 4 } |  |  | 0.652 |
| walker |  | 4329 | 28 | Code::CodeKey { rung: Names, file: src/posting/widgets/key_value.py, decl: 0, sub: 0, line: 0 } |  |  | 0.653 |
| walker |  | 4378 | 49 | Code::CodeKey { rung: Names, file: src/posting/yaml.py, decl: 0, sub: 0, line: 0 } |  |  | 0.653 |
| walker |  | 4407 | 29 | Code::CodeKey { rung: Names, file: src/posting/widgets/rich_log.py, decl: 0, sub: 0, line: 0 } |  |  | 0.653 |
| walker |  | 4487 | 80 | Code::CodeKey { rung: Decl, file: src/posting/widgets/rich_log.py, decl: 1, sub: 0, line: 7 } |  |  | 0.653 |
| ns | 4526 |  | 276 | app.py class roster and MainScreen reactive state | 5.1 |  | 0.637 |
| walker |  | 4569 | 82 | Code::CodeKey { rung: Decl, file: src/posting/widgets/rich_log.py, decl: 6, sub: 0, line: 37 } |  |  | 0.637 |
| walker |  | 4574 | 5 | Fs::DirListing { dir: .codex/environments } |  |  | 0.637 |
| walker |  | 4579 | 5 | Fs::DirListing { dir: tests/__snapshots__ } |  |  | 0.637 |
| ns | 4716 |  | 190 | Main screen keybindings, part 1 | 5.2 | 5.1 | 0.620 |
| walker |  | 4825 | 246 | Toml::Config { file: pyproject.toml } |  |  | 0.631 |
| walker |  | 4929 | 104 | Code::CodeKey { rung: Decl, file: src/posting/widgets/variable_autocomplete.py, decl: 1, sub: 0, line: 18 } |  |  | 0.631 |
| ns | 4962 |  | 246 | Main screen keybindings, part 2 | 5.3 | 5.2 | 0.609 |
| walker |  | 4986 | 57 | Code::CodeKey { rung: Names, file: src/posting/app.py, decl: 0, sub: 0, line: 0 } |  |  | 0.611 |
| walker |  | 5001 | 15 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 1, sub: 0, line: 93 } |  |  | 0.611 |
| walker |  | 5011 | 10 | Code::CodeKey { rung: Doc, file: src/posting/app.py, decl: 1, sub: 0, line: 93 } |  |  | 0.611 |
| walker |  | 5021 | 10 | Code::CodeKey { rung: Doc, file: src/posting/app.py, decl: 3, sub: 0, line: 108 } |  |  | 0.612 |
| walker |  | 5078 | 57 | Code::CodeKey { rung: Names, file: src/posting/save_request.py, decl: 0, sub: 0, line: 0 } |  |  | 0.612 |
| walker |  | 5089 | 11 | Code::CodeKey { rung: Doc, file: src/posting/save_request.py, decl: 2, sub: 0, line: 8 } |  |  | 0.612 |
| walker |  | 5100 | 11 | Code::CodeKey { rung: Body, file: src/posting/save_request.py, decl: 3, sub: 0, line: 15 } |  |  | 0.612 |
| walker |  | 5117 | 17 | Code::CodeKey { rung: Doc, file: src/posting/save_request.py, decl: 3, sub: 0, line: 15 } |  |  | 0.612 |
| walker |  | 5152 | 35 | Code::CodeKey { rung: Names, file: src/posting/widgets/tree.py, decl: 0, sub: 0, line: 0 } |  |  | 0.612 |
| ns | 5155 |  | 193 | Global app keybindings | 5.4 | 5.1 | 0.599 |
| walker |  | 5213 | 61 | Code::CodeKey { rung: Names, file: src/posting/tuple_to_multidict.py, decl: 0, sub: 0, line: 0 } |  |  | 0.599 |
| walker |  | 5329 | 116 | Code::CodeKey { rung: Decl, file: src/posting/importing/curl.py, decl: 1, sub: 0, line: 22 } |  |  | 0.599 |
| walker |  | 5345 | 16 | Code::CodeKey { rung: Doc, file: src/posting/importing/curl.py, decl: 3, sub: 0, line: 156 } |  |  | 0.599 |
| walker |  | 5361 | 16 | Code::CodeKey { rung: Doc, file: src/posting/importing/curl.py, decl: 4, sub: 0, line: 170 } |  |  | 0.599 |
| walker |  | 5377 | 16 | Code::CodeKey { rung: Doc, file: src/posting/importing/curl.py, decl: 6, sub: 0, line: 261 } |  |  | 0.599 |
| walker |  | 5443 | 66 | Code::CodeKey { rung: Names, file: src/posting/help_screen.py, decl: 0, sub: 0, line: 0 } |  |  | 0.599 |
| ns | 5444 |  | 289 | MainScreen method roster, part 1 | 5.5 |  | 0.584 |
| walker |  | 5462 | 19 | Code::CodeKey { rung: Decl, file: src/posting/help_screen.py, decl: 1, sub: 0, line: 14 } |  |  | 0.584 |
| walker |  | 5518 | 56 | Code::CodeKey { rung: Decl, file: src/posting/help_screen.py, decl: 2, sub: 0, line: 22 } |  |  | 0.584 |
| walker |  | 5527 | 9 | Code::CodeKey { rung: Doc, file: src/posting/help_screen.py, decl: 2, sub: 0, line: 22 } |  |  | 0.584 |
| walker |  | 5583 | 56 | Code::CodeKey { rung: Decl, file: src/posting/help_screen.py, decl: 3, sub: 0, line: 33 } |  |  | 0.584 |
| walker |  | 5592 | 9 | Code::CodeKey { rung: Doc, file: src/posting/help_screen.py, decl: 3, sub: 0, line: 33 } |  |  | 0.584 |
| walker |  | 5603 | 11 | Code::CodeKey { rung: Doc, file: src/posting/help_screen.py, decl: 4, sub: 0, line: 44 } |  |  | 0.584 |
| walker |  | 5629 | 26 | Code::CodeKey { rung: Doc, file: src/posting/help_screen.py, decl: 1, sub: 0, line: 14 } |  |  | 0.584 |
| walker |  | 5654 | 25 | Code::CodeKey { rung: Names, file: src/posting/widgets/response/response_trace.py, decl: 0, sub: 0, line: 0 } |  |  | 0.585 |
| ns | 5679 |  | 235 | MainScreen method roster, part 2 | 5.6 |  | 0.574 |
| ns | 5823 |  | 144 | MainScreen.compose: the widget layout | 5.7 | 5.5 | 0.565 |
| walker |  | 5872 | 218 | Code::CodeKey { rung: Decl, file: src/posting/jump_overlay.py, decl: 1, sub: 0, line: 16 } |  |  | 0.565 |
| walker |  | 5879 | 7 | Code::CodeKey { rung: Body, file: src/posting/jump_overlay.py, decl: 4, sub: 0, line: 64 } |  |  | 0.565 |
| walker |  | 5945 | 66 | Code::CodeKey { rung: Decl, file: src/posting/jump_overlay.py, decl: 2, sub: 0, line: 32 } |  |  | 0.565 |
| walker |  | 5972 | 27 | Code::CodeKey { rung: Body, file: src/posting/jump_overlay.py, decl: 6, sub: 0, line: 80 } |  |  | 0.565 |
| ns | 6072 |  | 249 | `Posting` App method roster | 5.8 |  | 0.555 |
| walker |  | 6107 | 135 | Code::CodeKey { rung: Decl, file: src/posting/widgets/tabbed_content.py, decl: 1, sub: 0, line: 5 } |  |  | 0.555 |
| walker |  | 6134 | 27 | Code::CodeKey { rung: Names, file: src/posting/widgets/request/request_scripts.py, decl: 0, sub: 0, line: 0 } |  |  | 0.555 |
| walker |  | 6224 | 90 | Code::CodeKey { rung: Decl, file: src/posting/widgets/response/cookies_table.py, decl: 1, sub: 0, line: 9 } |  |  | 0.555 |
| walker |  | 6232 | 8 | Code::CodeKey { rung: Decl, file: src/posting/widgets/response/cookies_table.py, decl: 6, sub: 0, line: 31 } |  |  | 0.555 |
| walker |  | 6246 | 14 | Code::CodeKey { rung: Decl, file: src/posting/widgets/response/cookies_table.py, decl: 4, sub: 0, line: 21 } |  |  | 0.555 |
| walker |  | 6260 | 14 | Code::CodeKey { rung: Decl, file: src/posting/widgets/response/cookies_table.py, decl: 5, sub: 0, line: 26 } |  |  | 0.555 |
| walker |  | 6288 | 28 | Code::CodeKey { rung: Names, file: src/posting/widgets/request/form_editor.py, decl: 0, sub: 0, line: 0 } |  |  | 0.555 |
| ns | 6311 |  | 239 | Command palette: every command | 6.1 |  | 0.546 |
| walker |  | 6333 | 45 | Code::CodeKey { rung: Decl, file: src/posting/widgets/request/form_editor.py, decl: 4, sub: 0, line: 40 } |  |  | 0.546 |
| walker |  | 6368 | 35 | Code::CodeKey { rung: Decl, file: src/posting/widgets/request/form_editor.py, decl: 7, sub: 0, line: 56 } |  |  | 0.546 |
| walker |  | 6379 | 11 | Code::CodeKey { rung: Doc, file: src/posting/widgets/request/form_editor.py, decl: 4, sub: 0, line: 40 } |  |  | 0.546 |
| walker |  | 6392 | 13 | Code::CodeKey { rung: Body, file: src/posting/widgets/request/form_editor.py, decl: 6, sub: 0, line: 53 } |  |  | 0.546 |
| walker |  | 6480 | 88 | Code::CodeKey { rung: Decl, file: src/posting/widgets/request/form_editor.py, decl: 1, sub: 0, line: 12 } |  |  | 0.546 |
| walker |  | 6520 | 40 | Code::CodeKey { rung: Body, file: src/posting/save_request.py, decl: 2, sub: 0, line: 8 } |  |  | 0.546 |
| walker |  | 6549 | 29 | Code::CodeKey { rung: Names, file: src/posting/widgets/request/query_editor.py, decl: 0, sub: 0, line: 0 } |  |  | 0.546 |
| walker |  | 6580 | 31 | Code::CodeKey { rung: Decl, file: src/posting/widgets/request/query_editor.py, decl: 5, sub: 0, line: 49 } |  |  | 0.546 |
| walker |  | 6588 | 8 | Code::CodeKey { rung: Decl, file: src/posting/widgets/request/query_editor.py, decl: 7, sub: 0, line: 65 } |  |  | 0.546 |
| walker |  | 6602 | 14 | Code::CodeKey { rung: Body, file: src/posting/widgets/request/query_editor.py, decl: 7, sub: 0, line: 65 } |  |  | 0.546 |
| walker |  | 6628 | 26 | Code::CodeKey { rung: Doc, file: src/posting/importing/curl.py, decl: 1, sub: 0, line: 22 } |  |  | 0.546 |
| walker |  | 6658 | 30 | Code::CodeKey { rung: Names, file: src/posting/widgets/request/request_editor.py, decl: 0, sub: 0, line: 0 } |  |  | 0.546 |
| walker |  | 6663 | 5 | Code::CodeKey { rung: Decl, file: src/posting/widgets/request/request_editor.py, decl: 1, sub: 0, line: 29 } |  |  | 0.546 |
| walker |  | 6684 | 21 | Markdown::HeadingsOutline { file: docs/guide/command_palette.md } |  |  | 0.546 |
| ns | 6694 |  | 383 | Request-editor widget class roster | 6.2 |  | 0.539 |
| walker |  | 6715 | 31 | Code::CodeKey { rung: Names, file: src/posting/widgets/request/request_body.py, decl: 0, sub: 0, line: 0 } |  |  | 0.541 |
| walker |  | 6730 | 15 | Code::CodeKey { rung: Decl, file: src/posting/widgets/request/request_body.py, decl: 1, sub: 0, line: 12 } |  |  | 0.541 |
| walker |  | 6777 | 47 | Code::CodeKey { rung: Doc, file: src/posting/commands.py, decl: 4, sub: 0, line: 204 } |  |  | 0.541 |
| walker |  | 6862 | 85 | Code::CodeKey { rung: Names, file: src/posting/locations.py, decl: 0, sub: 0, line: 0 } |  |  | 0.544 |
| walker |  | 6874 | 12 | Code::CodeKey { rung: Body, file: src/posting/locations.py, decl: 6, sub: 0, line: 34 } |  |  | 0.545 |
| walker |  | 6887 | 13 | Code::CodeKey { rung: Body, file: src/posting/locations.py, decl: 4, sub: 0, line: 24 } |  |  | 0.545 |
| walker |  | 6901 | 14 | Code::CodeKey { rung: Doc, file: src/posting/locations.py, decl: 4, sub: 0, line: 24 } |  |  | 0.547 |
| ns | 6902 |  | 208 | Response and collection-browser widget class roster | 6.3 |  | 0.542 |
| walker |  | 6915 | 14 | Code::CodeKey { rung: Body, file: src/posting/locations.py, decl: 2, sub: 0, line: 12 } |  |  | 0.542 |
| walker |  | 6929 | 14 | Code::CodeKey { rung: Doc, file: src/posting/locations.py, decl: 2, sub: 0, line: 12 } |  |  | 0.544 |
| walker |  | 6944 | 15 | Code::CodeKey { rung: Doc, file: src/posting/locations.py, decl: 3, sub: 0, line: 17 } |  |  | 0.544 |
| walker |  | 6960 | 16 | Code::CodeKey { rung: Doc, file: src/posting/locations.py, decl: 5, sub: 0, line: 29 } |  |  | 0.547 |
| walker |  | 6968 | 8 | Fs::DirListing { dir: tests/resources } |  |  | 0.547 |
| walker |  | 7018 | 50 | Code::CodeKey { rung: Names, file: src/posting/widgets/key_value_copy_modal.py, decl: 0, sub: 0, line: 0 } |  |  | 0.547 |
| walker |  | 7030 | 12 | Code::CodeKey { rung: Body, file: src/posting/locations.py, decl: 5, sub: 0, line: 29 } |  |  | 0.547 |
| walker |  | 7057 | 27 | Code::CodeKey { rung: Body, file: src/posting/widgets/variable_autocomplete.py, decl: 6, sub: 0, line: 93 } |  |  | 0.547 |
| walker |  | 7080 | 23 | Markdown::HeadingsOutline { file: docs/guide/help_system.md } |  |  | 0.547 |
| walker |  | 7127 | 47 | Code::CodeKey { rung: Body, file: src/posting/tuple_to_multidict.py, decl: 3, sub: 0, line: 9 } |  |  | 0.547 |
| walker |  | 7218 | 91 | Code::CodeKey { rung: Names, file: src/posting/files.py, decl: 0, sub: 0, line: 0 } |  |  | 0.547 |
| ns | 7238 |  | 336 | Shared widget class roster | 6.4 |  | 0.546 |
| walker |  | 7328 | 110 | Code::CodeKey { rung: Decl, file: src/posting/widgets/request/query_editor.py, decl: 1, sub: 0, line: 13 } |  |  | 0.546 |
| walker |  | 7347 | 19 | Code::CodeKey { rung: Doc, file: src/posting/widgets/request/query_editor.py, decl: 1, sub: 0, line: 13 } |  |  | 0.546 |
| ns | 7440 |  | 202 | Scripting API: the `Posting` object handed to user scripts | 6.5 |  | 0.541 |
| walker |  | 7463 | 116 | Code::CodeKey { rung: Decl, file: src/posting/widgets/response/response_trace.py, decl: 2, sub: 0, line: 33 } |  |  | 0.541 |
| walker |  | 7470 | 7 | Code::CodeKey { rung: Body, file: src/posting/widgets/response/response_trace.py, decl: 6, sub: 0, line: 82 } |  |  | 0.541 |
| walker |  | 7536 | 66 | Code::CodeKey { rung: Decl, file: src/posting/widgets/response/response_trace.py, decl: 3, sub: 0, line: 40 } |  |  | 0.541 |
| walker |  | 7718 | 182 | Code::CodeKey { rung: Decl, file: src/posting/widgets/variable_autocomplete.py, decl: 2, sub: 0, line: 19 } |  |  | 0.541 |
| ns | 7732 |  | 292 | Variables subsystem | 6.6 |  | 0.535 |
| walker |  | 7738 | 20 | Code::CodeKey { rung: Doc, file: src/posting/widgets/request/query_editor.py, decl: 5, sub: 0, line: 49 } |  |  | 0.535 |
| walker |  | 7769 | 31 | Code::CodeKey { rung: Body, file: src/posting/widgets/tabbed_content.py, decl: 2, sub: 0, line: 13 } |  |  | 0.535 |
| walker |  | 7800 | 31 | Code::CodeKey { rung: Body, file: src/posting/widgets/tabbed_content.py, decl: 3, sub: 0, line: 18 } |  |  | 0.535 |
| walker |  | 7860 | 60 | Code::CodeKey { rung: Names, file: src/posting/widgets/select.py, decl: 0, sub: 0, line: 0 } |  |  | 0.539 |
| walker |  | 7988 | 128 | Code::CodeKey { rung: Decl, file: src/posting/widgets/select.py, decl: 2, sub: 0, line: 8 } |  |  | 0.539 |
| walker |  | 8003 | 15 | Code::CodeKey { rung: Body, file: src/posting/widgets/select.py, decl: 4, sub: 0, line: 34 } |  |  | 0.539 |
| ns | 8068 |  | 336 | Theme model | 6.7 |  | 0.529 |
| walker |  | 8159 | 156 | Code::CodeKey { rung: Decl, file: src/posting/widgets/select.py, decl: 5, sub: 0, line: 38 } |  |  | 0.529 |
| walker |  | 8167 | 8 | Code::CodeKey { rung: Decl, file: src/posting/widgets/select.py, decl: 10, sub: 0, line: 68 } |  |  | 0.529 |
| walker |  | 8175 | 8 | Code::CodeKey { rung: Body, file: src/posting/widgets/select.py, decl: 6, sub: 0, line: 45 } |  |  | 0.529 |
| walker |  | 8183 | 8 | Code::CodeKey { rung: Body, file: src/posting/widgets/select.py, decl: 8, sub: 0, line: 59 } |  |  | 0.529 |
| walker |  | 8243 | 60 | Code::CodeKey { rung: Body, file: src/posting/__main__.py, decl: 1, sub: 0, line: 21 } |  |  | 0.529 |
| ns | 8260 |  | 192 | Every builtin theme name | 6.8 |  | 0.523 |
| walker |  | 8346 | 103 | Code::CodeKey { rung: Names, file: src/posting/urls.py, decl: 0, sub: 0, line: 0 } |  |  | 0.524 |
| walker |  | 8396 | 50 | Code::CodeKey { rung: Body, file: src/posting/urls.py, decl: 2, sub: 0, line: 9 } |  |  | 0.524 |
| walker |  | 8406 | 10 | Fs::DirListing { dir: tests/sample-envs } |  |  | 0.524 |
| ns | 8413 |  | 153 | Importer entry points | 6.9 |  | 0.520 |
| ns | 8530 |  | 117 | URL and path-parameter helpers | 6.10 |  | 0.522 |
| walker |  | 8633 | 227 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.522 |
| ns | 8694 |  | 164 | HTTP header catalogue | 6.11 |  | 0.518 |
| walker |  | 8768 | 135 | Code::CodeKey { rung: Decl, file: src/posting/widgets/request/request_body.py, decl: 3, sub: 0, line: 50 } |  |  | 0.518 |
| walker |  | 8788 | 20 | Code::CodeKey { rung: Doc, file: src/posting/widgets/request/request_body.py, decl: 3, sub: 0, line: 50 } |  |  | 0.518 |
| walker |  | 8810 | 22 | Code::CodeKey { rung: Body, file: src/posting/widgets/request/request_body.py, decl: 4, sub: 0, line: 67 } |  |  | 0.518 |
| walker |  | 8825 | 15 | Code::CodeKey { rung: Body, file: src/posting/widgets/response/cookies_table.py, decl: 6, sub: 0, line: 31 } |  |  | 0.518 |
| walker |  | 8868 | 43 | Code::CodeKey { rung: Names, file: src/posting/widgets/request/path_editor.py, decl: 0, sub: 0, line: 0 } |  |  | 0.522 |
| walker |  | 8899 | 31 | Code::CodeKey { rung: Decl, file: src/posting/widgets/request/path_editor.py, decl: 13, sub: 0, line: 168 } |  |  | 0.522 |
| walker |  | 8907 | 8 | Code::CodeKey { rung: Decl, file: src/posting/widgets/request/path_editor.py, decl: 15, sub: 0, line: 176 } |  |  | 0.522 |
| walker |  | 8915 | 8 | Code::CodeKey { rung: Body, file: src/posting/widgets/request/path_editor.py, decl: 14, sub: 0, line: 173 } |  |  | 0.522 |
| walker |  | 8929 | 14 | Code::CodeKey { rung: Body, file: src/posting/widgets/request/path_editor.py, decl: 15, sub: 0, line: 176 } |  |  | 0.522 |
| walker |  | 8972 | 43 | Code::CodeKey { rung: Names, file: src/posting/widgets/response/script_output.py, decl: 0, sub: 0, line: 0 } |  |  | 0.522 |
| ns | 8989 |  | 295 | Test invocation: the Makefile is mandatory | 7.1 |  | 0.526 |
| walker |  | 9010 | 38 | Code::CodeKey { rung: Doc, file: src/posting/widgets/variable_autocomplete.py, decl: 4, sub: 0, line: 60 } |  |  | 0.526 |
| walker |  | 9021 | 11 | Fs::DirListing { dir: tests/sample-themes } |  |  | 0.526 |
| walker |  | 9086 | 65 | Code::CodeKey { rung: Doc, file: src/posting/jump_overlay.py, decl: 1, sub: 0, line: 16 } |  |  | 0.526 |
| ns | 9139 |  | 150 | Development environment and snapshot testing | 7.2 |  | 0.523 |
| walker |  | 9309 | 223 | Code::CodeKey { rung: Decl, file: src/posting/widgets/variable_input.py, decl: 1, sub: 0, line: 12 } |  |  | 0.523 |
| walker |  | 9382 | 73 | Code::CodeKey { rung: Decl, file: src/posting/widgets/variable_input.py, decl: 2, sub: 0, line: 26 } |  |  | 0.523 |
| ns | 9407 |  | 268 | Scripting and core workflow guide section maps | 7.3 |  | 0.515 |
| walker |  | 9413 | 31 | Markdown::HeadingsOutline { file: docs/guide/themes.md } |  |  | 0.515 |
| walker |  | 9558 | 145 | Code::CodeKey { rung: Decl, file: src/posting/widgets/request/request_metadata.py, decl: 1, sub: 0, line: 11 } |  |  | 0.515 |
| walker |  | 9566 | 8 | Code::CodeKey { rung: Decl, file: src/posting/widgets/request/request_metadata.py, decl: 4, sub: 0, line: 38 } |  |  | 0.515 |
| walker |  | 9574 | 8 | Code::CodeKey { rung: Decl, file: src/posting/widgets/request/request_metadata.py, decl: 5, sub: 0, line: 42 } |  |  | 0.515 |
| walker |  | 9582 | 8 | Code::CodeKey { rung: Decl, file: src/posting/widgets/request/request_metadata.py, decl: 6, sub: 0, line: 46 } |  |  | 0.515 |
| walker |  | 9590 | 8 | Code::CodeKey { rung: Decl, file: src/posting/widgets/request/request_metadata.py, decl: 7, sub: 0, line: 50 } |  |  | 0.515 |
| walker |  | 9598 | 8 | Code::CodeKey { rung: Decl, file: src/posting/widgets/request/request_metadata.py, decl: 8, sub: 0, line: 54 } |  |  | 0.515 |
| walker |  | 9613 | 15 | Code::CodeKey { rung: Doc, file: src/posting/widgets/request/request_metadata.py, decl: 2, sub: 0, line: 14 } |  |  | 0.515 |
| ns | 9692 |  | 285 | Remaining guide section maps | 7.4 |  | 0.507 |
| walker |  | 9841 | 228 | Code::CodeKey { rung: Decl, file: src/posting/widgets/key_value.py, decl: 1, sub: 0, line: 19 } |  |  | 0.507 |
| walker |  | 9849 | 8 | Code::CodeKey { rung: Decl, file: src/posting/widgets/key_value.py, decl: 5, sub: 0, line: 74 } |  |  | 0.507 |
| walker |  | 9857 | 8 | Code::CodeKey { rung: Decl, file: src/posting/widgets/key_value.py, decl: 8, sub: 0, line: 122 } |  |  | 0.507 |
| walker |  | 9868 | 11 | Code::CodeKey { rung: Decl, file: src/posting/widgets/key_value.py, decl: 6, sub: 0, line: 80 } |  |  | 0.507 |
| walker |  | 9890 | 22 | Code::CodeKey { rung: Decl, file: src/posting/widgets/key_value.py, decl: 7, sub: 0, line: 85 } |  |  | 0.507 |
| ns | 9981 |  | 289 | Remaining packaging metadata and CI | 7.5 | 1.7 | 0.507 |

Score(3000)=0.793 I=0.938 C=0.671 ns_rows≤3K=19/49 grid(1000/1442/2080/3000/4327/6240/9000)=0.740/0.788/0.821/0.793/0.674/0.617/0.566

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
| walker |  | 801 | 14 | Fs::DirListing { dir: src/posting/importing } |  |  | 0.674 |
| walker |  | 817 | 16 | Code::CodeKey { rung: ModuleDoc, file: src/posting/__main__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.674 |
| walker |  | 906 | 89 | Fs::DirListing { dir: src/posting/widgets } |  |  | 0.805 |
| walker |  | 936 | 30 | Fs::DirListing { dir: src/posting/widgets/response } |  |  | 0.740 |
| ns | 936 |  | 186 | Runtime dependency pins | 1.8 | 1.7 | 0.740 |
| walker |  | 1004 | 68 | Fs::DirListing { dir: src/posting/widgets/request } |  |  | 0.748 |
| walker |  | 1014 | 10 | Fs::DirListing { dir: src/posting/widgets/collection } |  |  | 0.751 |
| ns | 1044 |  | 108 | UI sub-package listings: request, response, collection | 1.9 |  | 0.771 |
| walker |  | 1100 | 86 | Fs::DirListing { dir: docs/assets } |  |  | 0.771 |
| ns | 1140 |  | 96 | Documentation tree listing | 1.10 |  | 0.785 |
| walker |  | 1141 | 41 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.785 |
| walker |  | 1229 | 88 | Toml::PackageMetadata { file: pyproject.toml } |  |  | 0.799 |
| ns | 1270 |  | 130 | Test tree and sample-collection listings | 1.11 |  | 0.744 |
| walker |  | 1284 | 55 | Markdown::HeadingsOutline { file: docs/roadmap.md } |  |  | 0.744 |
| walker |  | 1383 | 99 | Fs::DirListing { dir: tests } |  |  | 0.788 |
| ns | 1460 |  | 190 | Complete `posting` subcommand roster | 2.1 |  | 0.763 |
| walker |  | 1681 | 298 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.822 |
| ns | 1689 |  | 229 | Every CLI argument and option | 2.2 |  | 0.780 |
| walker |  | 1731 | 50 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.780 |
| ns | 1907 |  | 218 | `make_posting`: CLI-to-app wiring | 2.3 |  | 0.753 |
| walker |  | 1988 | 257 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.841 |
| walker |  | 2022 | 34 | Markdown::Section { file: docs/index.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.841 |
| ns | 2052 |  | 145 | XDG locations: config file, themes and default collection | 2.4 |  | 0.821 |
| walker |  | 2159 | 137 | Code::CodeKey { rung: Names, file: src/posting/__main__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.829 |
| walker |  | 2171 | 12 | Code::CodeKey { rung: Decl, file: src/posting/__main__.py, decl: 7, sub: 0, line: 177 } |  |  | 0.831 |
| walker |  | 2193 | 22 | Code::CodeKey { rung: Decl, file: src/posting/__main__.py, decl: 3, sub: 0, line: 45 } |  |  | 0.836 |
| walker |  | 2225 | 32 | Code::CodeKey { rung: Decl, file: src/posting/__main__.py, decl: 5, sub: 0, line: 76 } |  |  | 0.836 |
| walker |  | 2269 | 44 | Code::CodeKey { rung: Decl, file: src/posting/__main__.py, decl: 8, sub: 0, line: 194 } |  |  | 0.839 |
| walker |  | 2281 | 12 | Code::CodeKey { rung: Doc, file: src/posting/__main__.py, decl: 7, sub: 0, line: 177 } |  |  | 0.842 |
| walker |  | 2295 | 14 | Code::CodeKey { rung: Doc, file: src/posting/__main__.py, decl: 3, sub: 0, line: 45 } |  |  | 0.846 |
| walker |  | 2312 | 17 | Code::CodeKey { rung: Doc, file: src/posting/__main__.py, decl: 8, sub: 0, line: 194 } |  |  | 0.848 |
| ns | 2359 |  | 307 | collection.py type roster | 3.1 |  | 0.805 |
| walker |  | 2439 | 127 | Code::CodeKey { rung: Decl, file: src/posting/__main__.py, decl: 4, sub: 0, line: 50 } |  |  | 0.821 |
| ns | 2534 |  | 175 | The `.posting.yaml` on-disk request format | 3.2 |  | 0.784 |
| walker |  | 2571 | 132 | Code::CodeKey { rung: Decl, file: src/posting/__main__.py, decl: 6, sub: 0, line: 96 } |  |  | 0.814 |
| walker |  | 2587 | 16 | Code::CodeKey { rung: Doc, file: src/posting/__main__.py, decl: 6, sub: 0, line: 96 } |  |  | 0.819 |
| walker |  | 2592 | 5 | Fs::DirListing { dir: .codex/environments } |  |  | 0.819 |
| walker |  | 2597 | 5 | Fs::DirListing { dir: tests/__snapshots__ } |  |  | 0.819 |
| ns | 2774 |  | 240 | RequestModel field roster | 3.3 | 3.1 | 0.791 |
| walker |  | 2843 | 246 | Toml::Config { file: pyproject.toml } |  |  | 0.805 |
| walker |  | 2864 | 21 | Markdown::HeadingsOutline { file: docs/guide/command_palette.md } |  |  | 0.805 |
| walker |  | 2872 | 8 | Fs::DirListing { dir: tests/resources } |  |  | 0.805 |
| walker |  | 2895 | 23 | Markdown::HeadingsOutline { file: docs/guide/help_system.md } |  |  | 0.805 |
| walker |  | 2905 | 10 | Fs::DirListing { dir: tests/sample-envs } |  |  | 0.805 |
| ns | 2934 |  | 160 | RequestModel behaviour roster | 3.4 |  | 0.793 |
| walker |  | 3132 | 227 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.793 |
| walker |  | 3142 | 10 | Code::CodeKey { rung: Names, file: src/posting/exit_codes.py, decl: 0, sub: 0, line: 0 } |  |  | 0.793 |
| ns | 3145 |  | 211 | Key-value, option and script models | 3.5 | 3.1 | 0.765 |
| walker |  | 3153 | 11 | Fs::DirListing { dir: tests/sample-themes } |  |  | 0.765 |
| walker |  | 3184 | 31 | Markdown::HeadingsOutline { file: docs/guide/themes.md } |  |  | 0.765 |
| walker |  | 3196 | 12 | Code::CodeKey { rung: Names, file: src/posting/help_data.py, decl: 0, sub: 0, line: 0 } |  |  | 0.765 |
| walker |  | 3255 | 59 | Code::CodeKey { rung: Decl, file: src/posting/help_data.py, decl: 1, sub: 0, line: 4 } |  |  | 0.765 |
| walker |  | 3272 | 17 | Code::CodeKey { rung: Doc, file: src/posting/help_data.py, decl: 1, sub: 0, line: 4 } |  |  | 0.765 |
| walker |  | 3284 | 12 | Code::CodeKey { rung: Names, file: src/posting/version.py, decl: 0, sub: 0, line: 0 } |  |  | 0.765 |
| walker |  | 3321 | 37 | Code::CodeKey { rung: Names, file: src/posting/jumper.py, decl: 0, sub: 0, line: 0 } |  |  | 0.765 |
| walker |  | 3340 | 19 | Code::CodeKey { rung: Decl, file: src/posting/jumper.py, decl: 1, sub: 0, line: 9 } |  |  | 0.765 |
| ns | 3349 |  | 204 | Collection tree: loading a directory, saving it back | 3.6 | 3.1 | 0.743 |
| walker |  | 3391 | 51 | Code::CodeKey { rung: Decl, file: src/posting/jumper.py, decl: 3, sub: 0, line: 26 } |  |  | 0.743 |
| walker |  | 3447 | 56 | Code::CodeKey { rung: Decl, file: src/posting/jumper.py, decl: 2, sub: 0, line: 16 } |  |  | 0.743 |
| walker |  | 3460 | 13 | Code::CodeKey { rung: Doc, file: src/posting/jumper.py, decl: 1, sub: 0, line: 9 } |  |  | 0.743 |
| walker |  | 3474 | 14 | Code::CodeKey { rung: Doc, file: src/posting/jumper.py, decl: 2, sub: 0, line: 16 } |  |  | 0.743 |
| walker |  | 3488 | 14 | Code::CodeKey { rung: Doc, file: src/posting/jumper.py, decl: 3, sub: 0, line: 26 } |  |  | 0.743 |
| walker |  | 3503 | 15 | Code::CodeKey { rung: Doc, file: src/posting/jumper.py, decl: 5, sub: 0, line: 34 } |  |  | 0.743 |
| ns | 3563 |  | 214 | config.py structure and settings-source configuration | 4.1 |  | 0.722 |
| ns | 3714 |  | 151 | Settings keys, part 1 | 4.2 |  | 0.708 |
| walker |  | 3729 | 226 | Code::CodeKey { rung: Names, file: src/posting/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.708 |
| walker |  | 3860 | 131 | Code::CodeKey { rung: Names, file: src/posting/config.py, decl: 0, sub: 0, line: 0 } |  |  | 0.720 |
| ns | 3892 |  | 178 | Settings keys, part 2 | 4.3 | 4.2 | 0.706 |
| walker |  | 3894 | 34 | Code::CodeKey { rung: Decl, file: src/posting/config.py, decl: 6, sub: 0, line: 92 } |  |  | 0.706 |
| walker |  | 3935 | 41 | Code::CodeKey { rung: Decl, file: src/posting/config.py, decl: 7, sub: 0, line: 99 } |  |  | 0.706 |
| walker |  | 3945 | 10 | Code::CodeKey { rung: Doc, file: src/posting/config.py, decl: 6, sub: 0, line: 92 } |  |  | 0.706 |
| walker |  | 3955 | 10 | Code::CodeKey { rung: Doc, file: src/posting/config.py, decl: 7, sub: 0, line: 99 } |  |  | 0.706 |
| walker |  | 4029 | 74 | Code::CodeKey { rung: Decl, file: src/posting/config.py, decl: 3, sub: 0, line: 46 } |  |  | 0.706 |
| walker |  | 4039 | 10 | Code::CodeKey { rung: Doc, file: src/posting/config.py, decl: 3, sub: 0, line: 46 } |  |  | 0.706 |
| walker |  | 4113 | 74 | Code::CodeKey { rung: Decl, file: src/posting/config.py, decl: 8, sub: 0, line: 106 } |  |  | 0.706 |
| walker |  | 4123 | 10 | Code::CodeKey { rung: Doc, file: src/posting/config.py, decl: 8, sub: 0, line: 106 } |  |  | 0.706 |
| walker |  | 4244 | 121 | Code::CodeKey { rung: Decl, file: src/posting/config.py, decl: 5, sub: 0, line: 79 } |  |  | 0.706 |
| ns | 4250 |  | 358 | Configuration precedence and guide section map | 4.4 |  | 0.674 |
| walker |  | 4257 | 13 | Code::CodeKey { rung: Doc, file: src/posting/config.py, decl: 5, sub: 0, line: 79 } |  |  | 0.674 |
| walker |  | 4397 | 140 | Code::CodeKey { rung: Decl, file: src/posting/config.py, decl: 2, sub: 0, line: 34 } |  |  | 0.674 |
| ns | 4526 |  | 276 | app.py class roster and MainScreen reactive state | 5.1 |  | 0.658 |
| walker |  | 4566 | 169 | Code::CodeKey { rung: Decl, file: src/posting/config.py, decl: 1, sub: 0, line: 18 } |  |  | 0.658 |
| ns | 4716 |  | 190 | Main screen keybindings, part 1 | 5.2 | 5.1 | 0.640 |
| walker |  | 4860 | 294 | Code::CodeKey { rung: Decl, file: src/posting/config.py, decl: 4, sub: 0, line: 56 } |  |  | 0.640 |
| walker |  | 4869 | 9 | Code::CodeKey { rung: Doc, file: src/posting/config.py, decl: 4, sub: 0, line: 56 } |  |  | 0.640 |
| walker |  | 4935 | 66 | Code::CodeKey { rung: Names, file: src/posting/help_screen.py, decl: 0, sub: 0, line: 0 } |  |  | 0.640 |
| walker |  | 4954 | 19 | Code::CodeKey { rung: Decl, file: src/posting/help_screen.py, decl: 1, sub: 0, line: 14 } |  |  | 0.640 |
| ns | 4962 |  | 246 | Main screen keybindings, part 2 | 5.3 | 5.2 | 0.619 |
| walker |  | 5010 | 56 | Code::CodeKey { rung: Decl, file: src/posting/help_screen.py, decl: 2, sub: 0, line: 22 } |  |  | 0.619 |
| walker |  | 5019 | 9 | Code::CodeKey { rung: Doc, file: src/posting/help_screen.py, decl: 2, sub: 0, line: 22 } |  |  | 0.619 |
| walker |  | 5075 | 56 | Code::CodeKey { rung: Decl, file: src/posting/help_screen.py, decl: 3, sub: 0, line: 33 } |  |  | 0.619 |
| walker |  | 5084 | 9 | Code::CodeKey { rung: Doc, file: src/posting/help_screen.py, decl: 3, sub: 0, line: 33 } |  |  | 0.619 |
| walker |  | 5095 | 11 | Code::CodeKey { rung: Doc, file: src/posting/help_screen.py, decl: 4, sub: 0, line: 44 } |  |  | 0.619 |
| walker |  | 5121 | 26 | Code::CodeKey { rung: Doc, file: src/posting/help_screen.py, decl: 1, sub: 0, line: 14 } |  |  | 0.619 |
| ns | 5155 |  | 193 | Global app keybindings | 5.4 | 5.1 | 0.606 |
| walker |  | 5429 | 308 | Code::CodeKey { rung: Names, file: src/posting/collection.py, decl: 0, sub: 0, line: 0 } |  |  | 0.641 |
| walker |  | 5439 | 10 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 10, sub: 0, line: 66 } |  |  | 0.641 |
| ns | 5444 |  | 289 | MainScreen method roster, part 1 | 5.5 |  | 0.626 |
| walker |  | 5455 | 16 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 11, sub: 0, line: 70 } |  |  | 0.626 |
| walker |  | 5476 | 21 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 31, sub: 0, line: 407 } |  |  | 0.626 |
| walker |  | 5497 | 21 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 32, sub: 0, line: 412 } |  |  | 0.626 |
| walker |  | 5519 | 22 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 8, sub: 0, line: 56 } |  |  | 0.626 |
| walker |  | 5541 | 22 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 9, sub: 0, line: 61 } |  |  | 0.626 |
| walker |  | 5570 | 29 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 12, sub: 0, line: 75 } |  |  | 0.627 |
| walker |  | 5599 | 29 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 13, sub: 0, line: 81 } |  |  | 0.627 |
| walker |  | 5628 | 29 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 14, sub: 0, line: 87 } |  |  | 0.627 |
| walker |  | 5666 | 38 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 30, sub: 0, line: 401 } |  |  | 0.627 |
| ns | 5679 |  | 235 | MainScreen method roster, part 2 | 5.6 |  | 0.616 |
| walker |  | 5722 | 56 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 15, sub: 0, line: 93 } |  |  | 0.616 |
| walker |  | 5728 | 6 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 16, sub: 0, line: 98 } |  |  | 0.616 |
| walker |  | 5799 | 71 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 17, sub: 0, line: 103 } |  |  | 0.624 |
| ns | 5823 |  | 144 | MainScreen.compose: the widget layout | 5.7 | 5.5 | 0.614 |
| walker |  | 5884 | 85 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 33, sub: 0, line: 417 } |  |  | 0.614 |
| walker |  | 6009 | 125 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 21, sub: 0, line: 138 } |  |  | 0.622 |
| ns | 6072 |  | 249 | `Posting` App method roster | 5.8 |  | 0.610 |
| walker |  | 6140 | 131 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 18, sub: 0, line: 111 } |  |  | 0.617 |
| walker |  | 6280 | 140 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 34, sub: 0, line: 427 } |  |  | 0.623 |
| walker |  | 6288 | 8 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 37, sub: 0, line: 480 } |  |  | 0.625 |
| ns | 6311 |  | 239 | Command palette: every command | 6.1 |  | 0.615 |
| walker |  | 6325 | 37 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 36, sub: 0, line: 441 } |  |  | 0.615 |
| walker |  | 6366 | 41 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 35, sub: 0, line: 434 } |  |  | 0.615 |
| walker |  | 6381 | 15 | Code::CodeKey { rung: Doc, file: src/posting/collection.py, decl: 38, sub: 0, line: 541 } |  |  | 0.615 |
| walker |  | 6548 | 167 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 3, sub: 0, line: 23 } |  |  | 0.615 |
| walker |  | 6556 | 8 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 5, sub: 0, line: 41 } |  |  | 0.615 |
| walker |  | 6564 | 8 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 6, sub: 0, line: 45 } |  |  | 0.615 |
| walker |  | 6572 | 8 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 7, sub: 0, line: 51 } |  |  | 0.615 |
| walker |  | 6608 | 36 | Code::CodeKey { rung: Doc, file: src/posting/collection.py, decl: 21, sub: 0, line: 138 } |  |  | 0.615 |
| walker |  | 6685 | 77 | Code::CodeKey { rung: Doc, file: src/posting/collection.py, decl: 39, sub: 0, line: 553 } |  |  | 0.615 |
| ns | 6694 |  | 383 | Request-editor widget class roster | 6.2 |  | 0.600 |
| walker |  | 6699 | 14 | Code::CodeKey { rung: Names, file: src/posting/messages.py, decl: 0, sub: 0, line: 0 } |  |  | 0.600 |
| walker |  | 6714 | 15 | Code::CodeKey { rung: Decl, file: src/posting/messages.py, decl: 1, sub: 0, line: 6 } |  |  | 0.600 |
| walker |  | 6799 | 85 | Code::CodeKey { rung: Names, file: src/posting/locations.py, decl: 0, sub: 0, line: 0 } |  |  | 0.602 |
| walker |  | 6811 | 12 | Code::CodeKey { rung: Body, file: src/posting/locations.py, decl: 6, sub: 0, line: 34 } |  |  | 0.603 |
| walker |  | 6824 | 13 | Code::CodeKey { rung: Body, file: src/posting/locations.py, decl: 4, sub: 0, line: 24 } |  |  | 0.603 |
| walker |  | 6838 | 14 | Code::CodeKey { rung: Doc, file: src/posting/locations.py, decl: 4, sub: 0, line: 24 } |  |  | 0.605 |
| walker |  | 6852 | 14 | Code::CodeKey { rung: Body, file: src/posting/locations.py, decl: 2, sub: 0, line: 12 } |  |  | 0.605 |
| walker |  | 6866 | 14 | Code::CodeKey { rung: Doc, file: src/posting/locations.py, decl: 2, sub: 0, line: 12 } |  |  | 0.608 |
| walker |  | 6881 | 15 | Code::CodeKey { rung: Doc, file: src/posting/locations.py, decl: 3, sub: 0, line: 17 } |  |  | 0.608 |
| walker |  | 6897 | 16 | Code::CodeKey { rung: Doc, file: src/posting/locations.py, decl: 5, sub: 0, line: 29 } |  |  | 0.611 |
| ns | 6902 |  | 208 | Response and collection-browser widget class roster | 6.3 |  | 0.603 |
| walker |  | 6909 | 12 | Code::CodeKey { rung: Body, file: src/posting/locations.py, decl: 5, sub: 0, line: 29 } |  |  | 0.603 |
| walker |  | 6966 | 57 | Code::CodeKey { rung: Names, file: src/posting/app.py, decl: 0, sub: 0, line: 0 } |  |  | 0.604 |
| walker |  | 6981 | 15 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 1, sub: 0, line: 93 } |  |  | 0.604 |
| walker |  | 6991 | 10 | Code::CodeKey { rung: Doc, file: src/posting/app.py, decl: 1, sub: 0, line: 93 } |  |  | 0.604 |
| walker |  | 7001 | 10 | Code::CodeKey { rung: Doc, file: src/posting/app.py, decl: 3, sub: 0, line: 108 } |  |  | 0.605 |
| walker |  | 7042 | 41 | Markdown::HeadingsOutline { file: docs/guide/keymap.md } |  |  | 0.605 |
| walker |  | 7071 | 29 | Code::CodeKey { rung: Names, file: src/posting/types.py, decl: 0, sub: 0, line: 0 } |  |  | 0.605 |
| walker |  | 7143 | 72 | Code::CodeKey { rung: Decl, file: src/posting/types.py, decl: 2, sub: 0, line: 6 } |  |  | 0.605 |
| walker |  | 7233 | 90 | Code::CodeKey { rung: Doc, file: src/posting/collection.py, decl: 37, sub: 0, line: 480 } |  |  | 0.605 |
| ns | 7238 |  | 336 | Shared widget class roster | 6.4 |  | 0.594 |
| walker |  | 7248 | 15 | Code::CodeKey { rung: Names, file: src/posting/_start_time.py, decl: 0, sub: 0, line: 0 } |  |  | 0.594 |
| walker |  | 7370 | 122 | Code::CodeKey { rung: Names, file: src/posting/scripts.py, decl: 0, sub: 0, line: 0 } |  |  | 0.594 |
| walker |  | 7399 | 29 | Code::CodeKey { rung: Decl, file: src/posting/scripts.py, decl: 13, sub: 0, line: 164 } |  |  | 0.594 |
| walker |  | 7432 | 33 | Code::CodeKey { rung: Decl, file: src/posting/scripts.py, decl: 12, sub: 0, line: 121 } |  |  | 0.594 |
| ns | 7440 |  | 202 | Scripting API: the `Posting` object handed to user scripts | 6.5 |  | 0.588 |
| walker |  | 7446 | 14 | Code::CodeKey { rung: Body, file: src/posting/scripts.py, decl: 14, sub: 0, line: 191 } |  |  | 0.588 |
| walker |  | 7465 | 19 | Code::CodeKey { rung: Doc, file: src/posting/scripts.py, decl: 13, sub: 0, line: 164 } |  |  | 0.588 |
| walker |  | 7597 | 132 | Code::CodeKey { rung: Decl, file: src/posting/scripts.py, decl: 3, sub: 0, line: 23 } |  |  | 0.594 |
| walker |  | 7605 | 8 | Code::CodeKey { rung: Decl, file: src/posting/scripts.py, decl: 5, sub: 0, line: 36 } |  |  | 0.595 |
| walker |  | 7669 | 64 | Code::CodeKey { rung: Decl, file: src/posting/scripts.py, decl: 10, sub: 0, line: 86 } |  |  | 0.595 |
| walker |  | 7680 | 11 | Code::CodeKey { rung: Doc, file: src/posting/scripts.py, decl: 9, sub: 0, line: 81 } |  |  | 0.595 |
| walker |  | 7698 | 18 | Code::CodeKey { rung: Doc, file: src/posting/scripts.py, decl: 3, sub: 0, line: 23 } |  |  | 0.598 |
| walker |  | 7726 | 28 | Code::CodeKey { rung: Doc, file: src/posting/scripts.py, decl: 11, sub: 0, line: 113 } |  |  | 0.598 |
| ns | 7732 |  | 292 | Variables subsystem | 6.6 |  | 0.591 |
| walker |  | 7770 | 44 | Code::CodeKey { rung: Doc, file: src/posting/scripts.py, decl: 8, sub: 0, line: 71 } |  |  | 0.591 |
| walker |  | 7823 | 53 | Code::CodeKey { rung: Doc, file: src/posting/scripts.py, decl: 15, sub: 0, line: 204 } |  |  | 0.591 |
| walker |  | 7879 | 56 | Code::CodeKey { rung: Doc, file: src/posting/scripts.py, decl: 5, sub: 0, line: 36 } |  |  | 0.591 |
| walker |  | 7954 | 75 | Code::CodeKey { rung: Doc, file: src/posting/scripts.py, decl: 14, sub: 0, line: 191 } |  |  | 0.591 |
| walker |  | 8046 | 92 | Code::CodeKey { rung: Doc, file: src/posting/scripts.py, decl: 7, sub: 0, line: 59 } |  |  | 0.591 |
| ns | 8068 |  | 336 | Theme model | 6.7 |  | 0.580 |
| walker |  | 8077 | 31 | Code::CodeKey { rung: Names, file: src/posting/request_headers.py, decl: 0, sub: 0, line: 0 } |  |  | 0.580 |
| walker |  | 8120 | 43 | Code::CodeKey { rung: Decl, file: src/posting/request_headers.py, decl: 1, sub: 0, line: 4 } |  |  | 0.580 |
| ns | 8260 |  | 192 | Every builtin theme name | 6.8 |  | 0.574 |
| ns | 8413 |  | 153 | Importer entry points | 6.9 |  | 0.570 |
| walker |  | 8442 | 322 | Code::CodeKey { rung: Names, file: src/posting/themes.py, decl: 0, sub: 0, line: 0 } |  |  | 0.571 |
| walker |  | 8530 | 88 | Code::CodeKey { rung: Decl, file: src/posting/themes.py, decl: 11, sub: 0, line: 305 } |  |  | 0.568 |
| ns | 8530 |  | 117 | URL and path-parameter helpers | 6.10 |  | 0.568 |
| walker |  | 8621 | 91 | Code::CodeKey { rung: Decl, file: src/posting/themes.py, decl: 3, sub: 0, line: 54 } |  |  | 0.568 |
| walker |  | 8632 | 11 | Code::CodeKey { rung: Doc, file: src/posting/themes.py, decl: 3, sub: 0, line: 54 } |  |  | 0.568 |
| ns | 8694 |  | 164 | HTTP header catalogue | 6.11 |  | 0.564 |
| walker |  | 8767 | 135 | Code::CodeKey { rung: Decl, file: src/posting/themes.py, decl: 5, sub: 0, line: 72 } |  |  | 0.564 |
| walker |  | 8780 | 13 | Code::CodeKey { rung: Doc, file: src/posting/themes.py, decl: 5, sub: 0, line: 72 } |  |  | 0.564 |
| walker |  | 8915 | 135 | Code::CodeKey { rung: Decl, file: src/posting/themes.py, decl: 7, sub: 0, line: 94 } |  |  | 0.564 |
| walker |  | 8930 | 15 | Code::CodeKey { rung: Doc, file: src/posting/themes.py, decl: 7, sub: 0, line: 94 } |  |  | 0.564 |
| ns | 8989 |  | 295 | Test invocation: the Makefile is mandatory | 7.1 |  | 0.566 |
| walker |  | 9103 | 173 | Code::CodeKey { rung: Decl, file: src/posting/themes.py, decl: 2, sub: 0, line: 34 } |  |  | 0.566 |
| walker |  | 9126 | 23 | Code::CodeKey { rung: Doc, file: src/posting/themes.py, decl: 2, sub: 0, line: 34 } |  |  | 0.566 |
| ns | 9139 |  | 150 | Development environment and snapshot testing | 7.2 |  | 0.564 |
| walker |  | 9158 | 32 | Code::CodeKey { rung: Doc, file: src/posting/themes.py, decl: 4, sub: 0, line: 63 } |  |  | 0.564 |
| walker |  | 9190 | 32 | Code::CodeKey { rung: Doc, file: src/posting/themes.py, decl: 6, sub: 0, line: 84 } |  |  | 0.564 |
| walker |  | 9405 | 215 | Code::CodeKey { rung: Decl, file: src/posting/themes.py, decl: 1, sub: 0, line: 14 } |  |  | 0.564 |
| ns | 9407 |  | 268 | Scripting and core workflow guide section maps | 7.3 |  | 0.555 |
| ns | 9692 |  | 285 | Remaining guide section maps | 7.4 |  | 0.548 |
| walker |  | 9711 | 306 | Code::CodeKey { rung: Decl, file: src/posting/themes.py, decl: 8, sub: 0, line: 106 } |  |  | 0.555 |
| walker |  | 9928 | 217 | Code::CodeKey { rung: Decl, file: src/posting/themes.py, decl: 8, sub: 1, line: 106 } |  |  | 0.569 |
| walker |  | 9960 | 32 | Code::CodeKey { rung: Decl, file: src/posting/themes.py, decl: 10, sub: 0, line: 244 } |  |  | 0.569 |
| ns | 9981 |  | 289 | Remaining packaging metadata and CI | 7.5 | 1.7 | 0.566 |

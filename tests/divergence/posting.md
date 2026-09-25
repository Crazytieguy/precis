Score(3000)=0.793 I=0.938 C=0.671 ns_rows≤3K=19/49 grid(1000/1442/2080/3000/4327/6240/9000)=0.740/0.788/0.766/0.793/0.674/0.617/0.566

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
| ns | 1140 |  | 96 | Documentation tree listing | 1.10 |  | 0.785 |
| walker |  | 1141 | 41 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.785 |
| walker |  | 1229 | 88 | Toml::PackageMetadata { file: pyproject.toml } |  |  | 0.799 |
| ns | 1270 |  | 130 | Test tree and sample-collection listings | 1.11 |  | 0.744 |
| walker |  | 1284 | 55 | Markdown::HeadingsOutline { file: docs/roadmap.md } |  |  | 0.744 |
| walker |  | 1383 | 99 | Fs::DirListing { dir: tests } |  |  | 0.788 |
| ns | 1460 |  | 190 | Complete `posting` subcommand roster | 2.1 |  | 0.763 |
| walker |  | 1520 | 137 | Code::CodeKey { rung: Names, file: src/posting/__main__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.772 |
| walker |  | 1532 | 12 | Code::CodeKey { rung: Decl, file: src/posting/__main__.py, decl: 7, sub: 0, line: 177 } |  |  | 0.775 |
| walker |  | 1554 | 22 | Code::CodeKey { rung: Decl, file: src/posting/__main__.py, decl: 3, sub: 0, line: 45 } |  |  | 0.781 |
| walker |  | 1586 | 32 | Code::CodeKey { rung: Decl, file: src/posting/__main__.py, decl: 5, sub: 0, line: 76 } |  |  | 0.781 |
| walker |  | 1630 | 44 | Code::CodeKey { rung: Decl, file: src/posting/__main__.py, decl: 8, sub: 0, line: 194 } |  |  | 0.781 |
| walker |  | 1642 | 12 | Code::CodeKey { rung: Doc, file: src/posting/__main__.py, decl: 7, sub: 0, line: 177 } |  |  | 0.785 |
| walker |  | 1656 | 14 | Code::CodeKey { rung: Doc, file: src/posting/__main__.py, decl: 3, sub: 0, line: 45 } |  |  | 0.790 |
| walker |  | 1673 | 17 | Code::CodeKey { rung: Doc, file: src/posting/__main__.py, decl: 8, sub: 0, line: 194 } |  |  | 0.790 |
| ns | 1689 |  | 229 | Every CLI argument and option | 2.2 |  | 0.750 |
| walker |  | 1800 | 127 | Code::CodeKey { rung: Decl, file: src/posting/__main__.py, decl: 4, sub: 0, line: 50 } |  |  | 0.768 |
| ns | 1907 |  | 218 | `make_posting`: CLI-to-app wiring | 2.3 |  | 0.745 |
| walker |  | 1932 | 132 | Code::CodeKey { rung: Decl, file: src/posting/__main__.py, decl: 6, sub: 0, line: 96 } |  |  | 0.779 |
| walker |  | 1948 | 16 | Code::CodeKey { rung: Doc, file: src/posting/__main__.py, decl: 6, sub: 0, line: 96 } |  |  | 0.785 |
| ns | 2052 |  | 145 | XDG locations: config file, themes and default collection | 2.4 |  | 0.766 |
| walker |  | 2246 | 298 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.817 |
| walker |  | 2296 | 50 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.817 |
| ns | 2359 |  | 307 | collection.py type roster | 3.1 |  | 0.776 |
| ns | 2534 |  | 175 | The `.posting.yaml` on-disk request format | 3.2 |  | 0.741 |
| walker |  | 2553 | 257 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.819 |
| walker |  | 2587 | 34 | Markdown::Section { file: docs/index.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.819 |
| walker |  | 2592 | 5 | Fs::DirListing { dir: .codex/environments } |  |  | 0.819 |
| walker |  | 2597 | 5 | Fs::DirListing { dir: tests/__snapshots__ } |  |  | 0.819 |
| ns | 2774 |  | 240 | RequestModel field roster | 3.3 | 3.1 | 0.791 |
| walker |  | 2843 | 246 | Toml::Config { file: pyproject.toml } |  |  | 0.805 |
| ns | 2934 |  | 160 | RequestModel behaviour roster | 3.4 |  | 0.793 |
| walker |  | 3069 | 226 | Code::CodeKey { rung: Names, file: src/posting/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.793 |
| walker |  | 3090 | 21 | Markdown::HeadingsOutline { file: docs/guide/command_palette.md } |  |  | 0.793 |
| walker |  | 3098 | 8 | Fs::DirListing { dir: tests/resources } |  |  | 0.793 |
| walker |  | 3121 | 23 | Markdown::HeadingsOutline { file: docs/guide/help_system.md } |  |  | 0.793 |
| ns | 3145 |  | 211 | Key-value, option and script models | 3.5 | 3.1 | 0.765 |
| walker |  | 3181 | 60 | Code::CodeKey { rung: Body, file: src/posting/__main__.py, decl: 1, sub: 0, line: 21 } |  |  | 0.765 |
| walker |  | 3191 | 10 | Fs::DirListing { dir: tests/sample-envs } |  |  | 0.765 |
| ns | 3349 |  | 204 | Collection tree: loading a directory, saving it back | 3.6 | 3.1 | 0.743 |
| walker |  | 3418 | 227 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.743 |
| walker |  | 3428 | 10 | Code::CodeKey { rung: Names, file: src/posting/exit_codes.py, decl: 0, sub: 0, line: 0 } |  |  | 0.743 |
| walker |  | 3439 | 11 | Fs::DirListing { dir: tests/sample-themes } |  |  | 0.743 |
| walker |  | 3470 | 31 | Markdown::HeadingsOutline { file: docs/guide/themes.md } |  |  | 0.743 |
| walker |  | 3482 | 12 | Code::CodeKey { rung: Names, file: src/posting/help_data.py, decl: 0, sub: 0, line: 0 } |  |  | 0.743 |
| walker |  | 3541 | 59 | Code::CodeKey { rung: Decl, file: src/posting/help_data.py, decl: 1, sub: 0, line: 4 } |  |  | 0.743 |
| walker |  | 3558 | 17 | Code::CodeKey { rung: Doc, file: src/posting/help_data.py, decl: 1, sub: 0, line: 4 } |  |  | 0.743 |
| ns | 3563 |  | 214 | config.py structure and settings-source configuration | 4.1 |  | 0.722 |
| walker |  | 3570 | 12 | Code::CodeKey { rung: Names, file: src/posting/version.py, decl: 0, sub: 0, line: 0 } |  |  | 0.722 |
| walker |  | 3607 | 37 | Code::CodeKey { rung: Names, file: src/posting/jumper.py, decl: 0, sub: 0, line: 0 } |  |  | 0.722 |
| walker |  | 3626 | 19 | Code::CodeKey { rung: Decl, file: src/posting/jumper.py, decl: 1, sub: 0, line: 9 } |  |  | 0.722 |
| walker |  | 3677 | 51 | Code::CodeKey { rung: Decl, file: src/posting/jumper.py, decl: 3, sub: 0, line: 26 } |  |  | 0.722 |
| ns | 3714 |  | 151 | Settings keys, part 1 | 4.2 |  | 0.708 |
| walker |  | 3733 | 56 | Code::CodeKey { rung: Decl, file: src/posting/jumper.py, decl: 2, sub: 0, line: 16 } |  |  | 0.708 |
| walker |  | 3746 | 13 | Code::CodeKey { rung: Doc, file: src/posting/jumper.py, decl: 1, sub: 0, line: 9 } |  |  | 0.708 |
| walker |  | 3760 | 14 | Code::CodeKey { rung: Doc, file: src/posting/jumper.py, decl: 2, sub: 0, line: 16 } |  |  | 0.708 |
| walker |  | 3774 | 14 | Code::CodeKey { rung: Doc, file: src/posting/jumper.py, decl: 3, sub: 0, line: 26 } |  |  | 0.708 |
| walker |  | 3789 | 15 | Code::CodeKey { rung: Doc, file: src/posting/jumper.py, decl: 5, sub: 0, line: 34 } |  |  | 0.708 |
| ns | 3892 |  | 178 | Settings keys, part 2 | 4.3 | 4.2 | 0.695 |
| walker |  | 3920 | 131 | Code::CodeKey { rung: Names, file: src/posting/config.py, decl: 0, sub: 0, line: 0 } |  |  | 0.706 |
| walker |  | 3954 | 34 | Code::CodeKey { rung: Decl, file: src/posting/config.py, decl: 6, sub: 0, line: 92 } |  |  | 0.706 |
| walker |  | 3995 | 41 | Code::CodeKey { rung: Decl, file: src/posting/config.py, decl: 7, sub: 0, line: 99 } |  |  | 0.706 |
| walker |  | 4005 | 10 | Code::CodeKey { rung: Doc, file: src/posting/config.py, decl: 6, sub: 0, line: 92 } |  |  | 0.706 |
| walker |  | 4015 | 10 | Code::CodeKey { rung: Doc, file: src/posting/config.py, decl: 7, sub: 0, line: 99 } |  |  | 0.706 |
| walker |  | 4089 | 74 | Code::CodeKey { rung: Decl, file: src/posting/config.py, decl: 3, sub: 0, line: 46 } |  |  | 0.706 |
| walker |  | 4099 | 10 | Code::CodeKey { rung: Doc, file: src/posting/config.py, decl: 3, sub: 0, line: 46 } |  |  | 0.706 |
| walker |  | 4173 | 74 | Code::CodeKey { rung: Decl, file: src/posting/config.py, decl: 8, sub: 0, line: 106 } |  |  | 0.706 |
| walker |  | 4183 | 10 | Code::CodeKey { rung: Doc, file: src/posting/config.py, decl: 8, sub: 0, line: 106 } |  |  | 0.706 |
| ns | 4250 |  | 358 | Configuration precedence and guide section map | 4.4 |  | 0.674 |
| walker |  | 4304 | 121 | Code::CodeKey { rung: Decl, file: src/posting/config.py, decl: 5, sub: 0, line: 79 } |  |  | 0.674 |
| walker |  | 4317 | 13 | Code::CodeKey { rung: Doc, file: src/posting/config.py, decl: 5, sub: 0, line: 79 } |  |  | 0.674 |
| walker |  | 4457 | 140 | Code::CodeKey { rung: Decl, file: src/posting/config.py, decl: 2, sub: 0, line: 34 } |  |  | 0.674 |
| ns | 4526 |  | 276 | app.py class roster and MainScreen reactive state | 5.1 |  | 0.658 |
| walker |  | 4626 | 169 | Code::CodeKey { rung: Decl, file: src/posting/config.py, decl: 1, sub: 0, line: 18 } |  |  | 0.658 |
| ns | 4716 |  | 190 | Main screen keybindings, part 1 | 5.2 | 5.1 | 0.640 |
| walker |  | 4920 | 294 | Code::CodeKey { rung: Decl, file: src/posting/config.py, decl: 4, sub: 0, line: 56 } |  |  | 0.640 |
| walker |  | 4929 | 9 | Code::CodeKey { rung: Doc, file: src/posting/config.py, decl: 4, sub: 0, line: 56 } |  |  | 0.640 |
| ns | 4962 |  | 246 | Main screen keybindings, part 2 | 5.3 | 5.2 | 0.619 |
| walker |  | 4995 | 66 | Code::CodeKey { rung: Names, file: src/posting/help_screen.py, decl: 0, sub: 0, line: 0 } |  |  | 0.619 |
| walker |  | 5014 | 19 | Code::CodeKey { rung: Decl, file: src/posting/help_screen.py, decl: 1, sub: 0, line: 14 } |  |  | 0.619 |
| walker |  | 5070 | 56 | Code::CodeKey { rung: Decl, file: src/posting/help_screen.py, decl: 2, sub: 0, line: 22 } |  |  | 0.619 |
| walker |  | 5079 | 9 | Code::CodeKey { rung: Doc, file: src/posting/help_screen.py, decl: 2, sub: 0, line: 22 } |  |  | 0.619 |
| walker |  | 5135 | 56 | Code::CodeKey { rung: Decl, file: src/posting/help_screen.py, decl: 3, sub: 0, line: 33 } |  |  | 0.619 |
| walker |  | 5144 | 9 | Code::CodeKey { rung: Doc, file: src/posting/help_screen.py, decl: 3, sub: 0, line: 33 } |  |  | 0.619 |
| walker |  | 5155 | 11 | Code::CodeKey { rung: Doc, file: src/posting/help_screen.py, decl: 4, sub: 0, line: 44 } |  |  | 0.606 |
| ns | 5155 |  | 193 | Global app keybindings | 5.4 | 5.1 | 0.606 |
| walker |  | 5181 | 26 | Code::CodeKey { rung: Doc, file: src/posting/help_screen.py, decl: 1, sub: 0, line: 14 } |  |  | 0.606 |
| ns | 5444 |  | 289 | MainScreen method roster, part 1 | 5.5 |  | 0.591 |
| walker |  | 5489 | 308 | Code::CodeKey { rung: Names, file: src/posting/collection.py, decl: 0, sub: 0, line: 0 } |  |  | 0.626 |
| walker |  | 5499 | 10 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 10, sub: 0, line: 66 } |  |  | 0.626 |
| walker |  | 5515 | 16 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 11, sub: 0, line: 70 } |  |  | 0.626 |
| walker |  | 5536 | 21 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 31, sub: 0, line: 407 } |  |  | 0.626 |
| walker |  | 5557 | 21 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 32, sub: 0, line: 412 } |  |  | 0.626 |
| walker |  | 5579 | 22 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 8, sub: 0, line: 56 } |  |  | 0.626 |
| walker |  | 5601 | 22 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 9, sub: 0, line: 61 } |  |  | 0.626 |
| walker |  | 5630 | 29 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 12, sub: 0, line: 75 } |  |  | 0.627 |
| walker |  | 5659 | 29 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 13, sub: 0, line: 81 } |  |  | 0.627 |
| ns | 5679 |  | 235 | MainScreen method roster, part 2 | 5.6 |  | 0.616 |
| walker |  | 5688 | 29 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 14, sub: 0, line: 87 } |  |  | 0.616 |
| walker |  | 5726 | 38 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 30, sub: 0, line: 401 } |  |  | 0.616 |
| walker |  | 5782 | 56 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 15, sub: 0, line: 93 } |  |  | 0.616 |
| walker |  | 5788 | 6 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 16, sub: 0, line: 98 } |  |  | 0.616 |
| ns | 5823 |  | 144 | MainScreen.compose: the widget layout | 5.7 | 5.5 | 0.607 |
| walker |  | 5859 | 71 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 17, sub: 0, line: 103 } |  |  | 0.614 |
| walker |  | 5944 | 85 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 33, sub: 0, line: 417 } |  |  | 0.614 |
| walker |  | 6069 | 125 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 21, sub: 0, line: 138 } |  |  | 0.622 |
| ns | 6072 |  | 249 | `Posting` App method roster | 5.8 |  | 0.610 |
| walker |  | 6200 | 131 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 18, sub: 0, line: 111 } |  |  | 0.617 |
| ns | 6311 |  | 239 | Command palette: every command | 6.1 |  | 0.607 |
| walker |  | 6340 | 140 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 34, sub: 0, line: 427 } |  |  | 0.614 |
| walker |  | 6348 | 8 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 37, sub: 0, line: 480 } |  |  | 0.615 |
| walker |  | 6385 | 37 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 36, sub: 0, line: 441 } |  |  | 0.615 |
| walker |  | 6426 | 41 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 35, sub: 0, line: 434 } |  |  | 0.615 |
| walker |  | 6441 | 15 | Code::CodeKey { rung: Doc, file: src/posting/collection.py, decl: 38, sub: 0, line: 541 } |  |  | 0.615 |
| walker |  | 6608 | 167 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 3, sub: 0, line: 23 } |  |  | 0.615 |
| walker |  | 6616 | 8 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 5, sub: 0, line: 41 } |  |  | 0.615 |
| walker |  | 6624 | 8 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 6, sub: 0, line: 45 } |  |  | 0.615 |
| walker |  | 6632 | 8 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 7, sub: 0, line: 51 } |  |  | 0.615 |
| walker |  | 6668 | 36 | Code::CodeKey { rung: Doc, file: src/posting/collection.py, decl: 21, sub: 0, line: 138 } |  |  | 0.615 |
| ns | 6694 |  | 383 | Request-editor widget class roster | 6.2 |  | 0.600 |
| walker |  | 6745 | 77 | Code::CodeKey { rung: Doc, file: src/posting/collection.py, decl: 39, sub: 0, line: 553 } |  |  | 0.600 |
| walker |  | 6759 | 14 | Code::CodeKey { rung: Names, file: src/posting/messages.py, decl: 0, sub: 0, line: 0 } |  |  | 0.600 |
| walker |  | 6774 | 15 | Code::CodeKey { rung: Decl, file: src/posting/messages.py, decl: 1, sub: 0, line: 6 } |  |  | 0.600 |
| walker |  | 6859 | 85 | Code::CodeKey { rung: Names, file: src/posting/locations.py, decl: 0, sub: 0, line: 0 } |  |  | 0.602 |
| walker |  | 6871 | 12 | Code::CodeKey { rung: Body, file: src/posting/locations.py, decl: 6, sub: 0, line: 34 } |  |  | 0.603 |
| walker |  | 6884 | 13 | Code::CodeKey { rung: Body, file: src/posting/locations.py, decl: 4, sub: 0, line: 24 } |  |  | 0.603 |
| walker |  | 6898 | 14 | Code::CodeKey { rung: Doc, file: src/posting/locations.py, decl: 4, sub: 0, line: 24 } |  |  | 0.605 |
| ns | 6902 |  | 208 | Response and collection-browser widget class roster | 6.3 |  | 0.598 |
| walker |  | 6912 | 14 | Code::CodeKey { rung: Body, file: src/posting/locations.py, decl: 2, sub: 0, line: 12 } |  |  | 0.598 |
| walker |  | 6926 | 14 | Code::CodeKey { rung: Doc, file: src/posting/locations.py, decl: 2, sub: 0, line: 12 } |  |  | 0.600 |
| walker |  | 6941 | 15 | Code::CodeKey { rung: Doc, file: src/posting/locations.py, decl: 3, sub: 0, line: 17 } |  |  | 0.600 |
| walker |  | 6957 | 16 | Code::CodeKey { rung: Doc, file: src/posting/locations.py, decl: 5, sub: 0, line: 29 } |  |  | 0.603 |
| walker |  | 6969 | 12 | Code::CodeKey { rung: Body, file: src/posting/locations.py, decl: 5, sub: 0, line: 29 } |  |  | 0.603 |
| walker |  | 7026 | 57 | Code::CodeKey { rung: Names, file: src/posting/app.py, decl: 0, sub: 0, line: 0 } |  |  | 0.604 |
| walker |  | 7041 | 15 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 1, sub: 0, line: 93 } |  |  | 0.604 |
| walker |  | 7051 | 10 | Code::CodeKey { rung: Doc, file: src/posting/app.py, decl: 1, sub: 0, line: 93 } |  |  | 0.604 |
| walker |  | 7061 | 10 | Code::CodeKey { rung: Doc, file: src/posting/app.py, decl: 3, sub: 0, line: 108 } |  |  | 0.605 |
| walker |  | 7102 | 41 | Markdown::HeadingsOutline { file: docs/guide/keymap.md } |  |  | 0.605 |
| walker |  | 7131 | 29 | Code::CodeKey { rung: Names, file: src/posting/types.py, decl: 0, sub: 0, line: 0 } |  |  | 0.605 |
| walker |  | 7203 | 72 | Code::CodeKey { rung: Decl, file: src/posting/types.py, decl: 2, sub: 0, line: 6 } |  |  | 0.605 |
| ns | 7238 |  | 336 | Shared widget class roster | 6.4 |  | 0.594 |
| walker |  | 7293 | 90 | Code::CodeKey { rung: Doc, file: src/posting/collection.py, decl: 37, sub: 0, line: 480 } |  |  | 0.594 |
| walker |  | 7371 | 78 | Code::CodeKey { rung: Body, file: src/posting/__main__.py, decl: 2, sub: 0, line: 32 } |  |  | 0.594 |
| walker |  | 7386 | 15 | Code::CodeKey { rung: Names, file: src/posting/_start_time.py, decl: 0, sub: 0, line: 0 } |  |  | 0.594 |
| ns | 7440 |  | 202 | Scripting API: the `Posting` object handed to user scripts | 6.5 |  | 0.588 |
| walker |  | 7508 | 122 | Code::CodeKey { rung: Names, file: src/posting/scripts.py, decl: 0, sub: 0, line: 0 } |  |  | 0.588 |
| walker |  | 7541 | 33 | Code::CodeKey { rung: Decl, file: src/posting/scripts.py, decl: 12, sub: 0, line: 121 } |  |  | 0.588 |
| walker |  | 7570 | 29 | Code::CodeKey { rung: Decl, file: src/posting/scripts.py, decl: 13, sub: 0, line: 164 } |  |  | 0.588 |
| walker |  | 7590 | 20 | Code::CodeKey { rung: Body, file: src/posting/scripts.py, decl: 11, sub: 0, line: 113 } |  |  | 0.588 |
| walker |  | 7722 | 132 | Code::CodeKey { rung: Decl, file: src/posting/scripts.py, decl: 3, sub: 0, line: 23 } |  |  | 0.594 |
| walker |  | 7730 | 8 | Code::CodeKey { rung: Decl, file: src/posting/scripts.py, decl: 5, sub: 0, line: 36 } |  |  | 0.595 |
| ns | 7732 |  | 292 | Variables subsystem | 6.6 |  | 0.588 |
| walker |  | 7794 | 64 | Code::CodeKey { rung: Decl, file: src/posting/scripts.py, decl: 10, sub: 0, line: 86 } |  |  | 0.588 |
| walker |  | 7805 | 11 | Code::CodeKey { rung: Doc, file: src/posting/scripts.py, decl: 9, sub: 0, line: 81 } |  |  | 0.588 |
| walker |  | 7823 | 18 | Code::CodeKey { rung: Doc, file: src/posting/scripts.py, decl: 3, sub: 0, line: 23 } |  |  | 0.591 |
| walker |  | 7849 | 26 | Code::CodeKey { rung: Doc, file: src/posting/scripts.py, decl: 11, sub: 0, line: 113 } |  |  | 0.591 |
| walker |  | 7893 | 44 | Code::CodeKey { rung: Doc, file: src/posting/scripts.py, decl: 8, sub: 0, line: 71 } |  |  | 0.591 |
| walker |  | 7946 | 53 | Code::CodeKey { rung: Doc, file: src/posting/scripts.py, decl: 15, sub: 0, line: 204 } |  |  | 0.591 |
| walker |  | 8002 | 56 | Code::CodeKey { rung: Doc, file: src/posting/scripts.py, decl: 5, sub: 0, line: 36 } |  |  | 0.591 |
| walker |  | 8021 | 19 | Code::CodeKey { rung: Doc, file: src/posting/scripts.py, decl: 13, sub: 0, line: 164 } |  |  | 0.591 |
| ns | 8068 |  | 336 | Theme model | 6.7 |  | 0.580 |
| walker |  | 8113 | 92 | Code::CodeKey { rung: Doc, file: src/posting/scripts.py, decl: 7, sub: 0, line: 59 } |  |  | 0.580 |
| walker |  | 8144 | 31 | Code::CodeKey { rung: Names, file: src/posting/request_headers.py, decl: 0, sub: 0, line: 0 } |  |  | 0.580 |
| walker |  | 8187 | 43 | Code::CodeKey { rung: Decl, file: src/posting/request_headers.py, decl: 1, sub: 0, line: 4 } |  |  | 0.580 |
| ns | 8260 |  | 192 | Every builtin theme name | 6.8 |  | 0.574 |
| ns | 8413 |  | 153 | Importer entry points | 6.9 |  | 0.570 |
| walker |  | 8509 | 322 | Code::CodeKey { rung: Names, file: src/posting/themes.py, decl: 0, sub: 0, line: 0 } |  |  | 0.571 |
| ns | 8530 |  | 117 | URL and path-parameter helpers | 6.10 |  | 0.568 |
| walker |  | 8597 | 88 | Code::CodeKey { rung: Decl, file: src/posting/themes.py, decl: 11, sub: 0, line: 305 } |  |  | 0.568 |
| walker |  | 8688 | 91 | Code::CodeKey { rung: Decl, file: src/posting/themes.py, decl: 3, sub: 0, line: 54 } |  |  | 0.568 |
| ns | 8694 |  | 164 | HTTP header catalogue | 6.11 |  | 0.564 |
| walker |  | 8699 | 11 | Code::CodeKey { rung: Doc, file: src/posting/themes.py, decl: 3, sub: 0, line: 54 } |  |  | 0.564 |
| walker |  | 8834 | 135 | Code::CodeKey { rung: Decl, file: src/posting/themes.py, decl: 5, sub: 0, line: 72 } |  |  | 0.564 |
| walker |  | 8847 | 13 | Code::CodeKey { rung: Doc, file: src/posting/themes.py, decl: 5, sub: 0, line: 72 } |  |  | 0.564 |
| walker |  | 8982 | 135 | Code::CodeKey { rung: Decl, file: src/posting/themes.py, decl: 7, sub: 0, line: 94 } |  |  | 0.564 |
| ns | 8989 |  | 295 | Test invocation: the Makefile is mandatory | 7.1 |  | 0.566 |
| walker |  | 8997 | 15 | Code::CodeKey { rung: Doc, file: src/posting/themes.py, decl: 7, sub: 0, line: 94 } |  |  | 0.566 |
| ns | 9139 |  | 150 | Development environment and snapshot testing | 7.2 |  | 0.564 |
| walker |  | 9170 | 173 | Code::CodeKey { rung: Decl, file: src/posting/themes.py, decl: 2, sub: 0, line: 34 } |  |  | 0.564 |
| walker |  | 9193 | 23 | Code::CodeKey { rung: Doc, file: src/posting/themes.py, decl: 2, sub: 0, line: 34 } |  |  | 0.564 |
| walker |  | 9225 | 32 | Code::CodeKey { rung: Doc, file: src/posting/themes.py, decl: 4, sub: 0, line: 63 } |  |  | 0.564 |
| walker |  | 9257 | 32 | Code::CodeKey { rung: Doc, file: src/posting/themes.py, decl: 6, sub: 0, line: 84 } |  |  | 0.564 |
| ns | 9407 |  | 268 | Scripting and core workflow guide section maps | 7.3 |  | 0.555 |
| walker |  | 9472 | 215 | Code::CodeKey { rung: Decl, file: src/posting/themes.py, decl: 1, sub: 0, line: 14 } |  |  | 0.555 |
| ns | 9692 |  | 285 | Remaining guide section maps | 7.4 |  | 0.548 |
| walker |  | 9778 | 306 | Code::CodeKey { rung: Decl, file: src/posting/themes.py, decl: 8, sub: 0, line: 106 } |  |  | 0.555 |
| ns | 9981 |  | 289 | Remaining packaging metadata and CI | 7.5 | 1.7 | 0.553 |
| walker |  | 9995 | 217 | Code::CodeKey { rung: Decl, file: src/posting/themes.py, decl: 8, sub: 1, line: 106 } |  |  | 0.566 |

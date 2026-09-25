Score(3000)=0.769 I=0.922 C=0.642 ns_rows≤3K=19/49 grid(1000/1442/2080/3000/4327/6240/9000)=0.746/0.815/0.775/0.769/0.654/0.608/0.553

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
| walker |  | 1191 | 50 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.785 |
| ns | 1270 |  | 130 | Test tree and sample-collection listings | 1.11 |  | 0.731 |
| walker |  | 1448 | 257 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.830 |
| ns | 1460 |  | 190 | Complete `posting` subcommand roster | 2.1 |  | 0.803 |
| walker |  | 1674 | 226 | Code::CodeKey { rung: Names, file: src/posting/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.803 |
| ns | 1689 |  | 229 | Every CLI argument and option | 2.2 |  | 0.763 |
| walker |  | 1729 | 55 | Markdown::HeadingsOutline { file: docs/roadmap.md } |  |  | 0.763 |
| walker |  | 1828 | 99 | Fs::DirListing { dir: tests } |  |  | 0.803 |
| ns | 1907 |  | 218 | `make_posting`: CLI-to-app wiring | 2.3 |  | 0.774 |
| walker |  | 1965 | 137 | Code::CodeKey { rung: Names, file: src/posting/__main__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.783 |
| walker |  | 1977 | 12 | Code::CodeKey { rung: Decl, file: src/posting/__main__.py, decl: 7, sub: 0, line: 177 } |  |  | 0.786 |
| walker |  | 1999 | 22 | Code::CodeKey { rung: Decl, file: src/posting/__main__.py, decl: 3, sub: 0, line: 45 } |  |  | 0.791 |
| walker |  | 2031 | 32 | Code::CodeKey { rung: Decl, file: src/posting/__main__.py, decl: 5, sub: 0, line: 76 } |  |  | 0.791 |
| ns | 2052 |  | 145 | XDG locations: config file, themes and default collection | 2.4 |  | 0.772 |
| walker |  | 2075 | 44 | Code::CodeKey { rung: Decl, file: src/posting/__main__.py, decl: 8, sub: 0, line: 194 } |  |  | 0.775 |
| walker |  | 2087 | 12 | Code::CodeKey { rung: Doc, file: src/posting/__main__.py, decl: 7, sub: 0, line: 177 } |  |  | 0.778 |
| walker |  | 2101 | 14 | Code::CodeKey { rung: Doc, file: src/posting/__main__.py, decl: 3, sub: 0, line: 45 } |  |  | 0.782 |
| walker |  | 2118 | 17 | Code::CodeKey { rung: Doc, file: src/posting/__main__.py, decl: 8, sub: 0, line: 194 } |  |  | 0.785 |
| walker |  | 2245 | 127 | Code::CodeKey { rung: Decl, file: src/posting/__main__.py, decl: 4, sub: 0, line: 50 } |  |  | 0.801 |
| ns | 2359 |  | 307 | collection.py type roster | 3.1 |  | 0.761 |
| walker |  | 2377 | 132 | Code::CodeKey { rung: Decl, file: src/posting/__main__.py, decl: 6, sub: 0, line: 96 } |  |  | 0.793 |
| walker |  | 2393 | 16 | Code::CodeKey { rung: Doc, file: src/posting/__main__.py, decl: 6, sub: 0, line: 96 } |  |  | 0.798 |
| ns | 2534 |  | 175 | The `.posting.yaml` on-disk request format | 3.2 |  | 0.763 |
| walker |  | 2693 | 300 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.808 |
| walker |  | 2698 | 5 | Fs::DirListing { dir: .codex/environments } |  |  | 0.808 |
| walker |  | 2703 | 5 | Fs::DirListing { dir: tests/__snapshots__ } |  |  | 0.808 |
| ns | 2774 |  | 240 | RequestModel field roster | 3.3 | 3.1 | 0.781 |
| walker |  | 2930 | 227 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.781 |
| ns | 2934 |  | 160 | RequestModel behaviour roster | 3.4 |  | 0.769 |
| walker |  | 2951 | 21 | Markdown::HeadingsOutline { file: docs/guide/command_palette.md } |  |  | 0.769 |
| walker |  | 2959 | 8 | Fs::DirListing { dir: tests/resources } |  |  | 0.769 |
| walker |  | 2969 | 10 | Code::CodeKey { rung: Names, file: src/posting/exit_codes.py, decl: 0, sub: 0, line: 0 } |  |  | 0.769 |
| walker |  | 2992 | 23 | Markdown::HeadingsOutline { file: docs/guide/help_system.md } |  |  | 0.769 |
| walker |  | 3004 | 12 | Code::CodeKey { rung: Names, file: src/posting/help_data.py, decl: 0, sub: 0, line: 0 } |  |  | 0.769 |
| walker |  | 3063 | 59 | Code::CodeKey { rung: Decl, file: src/posting/help_data.py, decl: 1, sub: 0, line: 4 } |  |  | 0.769 |
| walker |  | 3080 | 17 | Code::CodeKey { rung: Doc, file: src/posting/help_data.py, decl: 1, sub: 0, line: 4 } |  |  | 0.769 |
| walker |  | 3092 | 12 | Code::CodeKey { rung: Names, file: src/posting/version.py, decl: 0, sub: 0, line: 0 } |  |  | 0.769 |
| walker |  | 3102 | 10 | Fs::DirListing { dir: tests/sample-envs } |  |  | 0.769 |
| walker |  | 3139 | 37 | Code::CodeKey { rung: Names, file: src/posting/jumper.py, decl: 0, sub: 0, line: 0 } |  |  | 0.769 |
| ns | 3145 |  | 211 | Key-value, option and script models | 3.5 | 3.1 | 0.741 |
| walker |  | 3158 | 19 | Code::CodeKey { rung: Decl, file: src/posting/jumper.py, decl: 1, sub: 0, line: 9 } |  |  | 0.741 |
| walker |  | 3209 | 51 | Code::CodeKey { rung: Decl, file: src/posting/jumper.py, decl: 3, sub: 0, line: 26 } |  |  | 0.741 |
| walker |  | 3265 | 56 | Code::CodeKey { rung: Decl, file: src/posting/jumper.py, decl: 2, sub: 0, line: 16 } |  |  | 0.741 |
| walker |  | 3278 | 13 | Code::CodeKey { rung: Doc, file: src/posting/jumper.py, decl: 1, sub: 0, line: 9 } |  |  | 0.741 |
| walker |  | 3292 | 14 | Code::CodeKey { rung: Doc, file: src/posting/jumper.py, decl: 2, sub: 0, line: 16 } |  |  | 0.741 |
| walker |  | 3306 | 14 | Code::CodeKey { rung: Doc, file: src/posting/jumper.py, decl: 3, sub: 0, line: 26 } |  |  | 0.741 |
| walker |  | 3321 | 15 | Code::CodeKey { rung: Doc, file: src/posting/jumper.py, decl: 5, sub: 0, line: 34 } |  |  | 0.741 |
| ns | 3349 |  | 204 | Collection tree: loading a directory, saving it back | 3.6 | 3.1 | 0.721 |
| walker |  | 3452 | 131 | Code::CodeKey { rung: Names, file: src/posting/config.py, decl: 0, sub: 0, line: 0 } |  |  | 0.722 |
| walker |  | 3486 | 34 | Code::CodeKey { rung: Decl, file: src/posting/config.py, decl: 6, sub: 0, line: 92 } |  |  | 0.722 |
| walker |  | 3527 | 41 | Code::CodeKey { rung: Decl, file: src/posting/config.py, decl: 7, sub: 0, line: 99 } |  |  | 0.722 |
| walker |  | 3537 | 10 | Code::CodeKey { rung: Doc, file: src/posting/config.py, decl: 6, sub: 0, line: 92 } |  |  | 0.722 |
| walker |  | 3547 | 10 | Code::CodeKey { rung: Doc, file: src/posting/config.py, decl: 7, sub: 0, line: 99 } |  |  | 0.722 |
| ns | 3563 |  | 214 | config.py structure and settings-source configuration | 4.1 |  | 0.712 |
| walker |  | 3621 | 74 | Code::CodeKey { rung: Decl, file: src/posting/config.py, decl: 3, sub: 0, line: 46 } |  |  | 0.712 |
| walker |  | 3631 | 10 | Code::CodeKey { rung: Doc, file: src/posting/config.py, decl: 3, sub: 0, line: 46 } |  |  | 0.712 |
| walker |  | 3705 | 74 | Code::CodeKey { rung: Decl, file: src/posting/config.py, decl: 8, sub: 0, line: 106 } |  |  | 0.712 |
| ns | 3714 |  | 151 | Settings keys, part 1 | 4.2 |  | 0.698 |
| walker |  | 3715 | 10 | Code::CodeKey { rung: Doc, file: src/posting/config.py, decl: 8, sub: 0, line: 106 } |  |  | 0.698 |
| walker |  | 3836 | 121 | Code::CodeKey { rung: Decl, file: src/posting/config.py, decl: 5, sub: 0, line: 79 } |  |  | 0.698 |
| walker |  | 3849 | 13 | Code::CodeKey { rung: Doc, file: src/posting/config.py, decl: 5, sub: 0, line: 79 } |  |  | 0.698 |
| ns | 3892 |  | 178 | Settings keys, part 2 | 4.3 | 4.2 | 0.685 |
| walker |  | 3989 | 140 | Code::CodeKey { rung: Decl, file: src/posting/config.py, decl: 2, sub: 0, line: 34 } |  |  | 0.685 |
| walker |  | 4158 | 169 | Code::CodeKey { rung: Decl, file: src/posting/config.py, decl: 1, sub: 0, line: 18 } |  |  | 0.685 |
| ns | 4250 |  | 358 | Configuration precedence and guide section map | 4.4 |  | 0.654 |
| walker |  | 4452 | 294 | Code::CodeKey { rung: Decl, file: src/posting/config.py, decl: 4, sub: 0, line: 56 } |  |  | 0.654 |
| walker |  | 4461 | 9 | Code::CodeKey { rung: Doc, file: src/posting/config.py, decl: 4, sub: 0, line: 56 } |  |  | 0.654 |
| ns | 4526 |  | 276 | app.py class roster and MainScreen reactive state | 5.1 |  | 0.638 |
| walker |  | 4527 | 66 | Code::CodeKey { rung: Names, file: src/posting/help_screen.py, decl: 0, sub: 0, line: 0 } |  |  | 0.638 |
| walker |  | 4546 | 19 | Code::CodeKey { rung: Decl, file: src/posting/help_screen.py, decl: 1, sub: 0, line: 14 } |  |  | 0.638 |
| walker |  | 4602 | 56 | Code::CodeKey { rung: Decl, file: src/posting/help_screen.py, decl: 2, sub: 0, line: 22 } |  |  | 0.638 |
| walker |  | 4611 | 9 | Code::CodeKey { rung: Doc, file: src/posting/help_screen.py, decl: 2, sub: 0, line: 22 } |  |  | 0.638 |
| walker |  | 4667 | 56 | Code::CodeKey { rung: Decl, file: src/posting/help_screen.py, decl: 3, sub: 0, line: 33 } |  |  | 0.638 |
| walker |  | 4676 | 9 | Code::CodeKey { rung: Doc, file: src/posting/help_screen.py, decl: 3, sub: 0, line: 33 } |  |  | 0.638 |
| walker |  | 4687 | 11 | Code::CodeKey { rung: Doc, file: src/posting/help_screen.py, decl: 4, sub: 0, line: 44 } |  |  | 0.638 |
| walker |  | 4713 | 26 | Code::CodeKey { rung: Doc, file: src/posting/help_screen.py, decl: 1, sub: 0, line: 14 } |  |  | 0.638 |
| ns | 4716 |  | 190 | Main screen keybindings, part 1 | 5.2 | 5.1 | 0.621 |
| walker |  | 4724 | 11 | Fs::DirListing { dir: tests/sample-themes } |  |  | 0.621 |
| walker |  | 4755 | 31 | Markdown::HeadingsOutline { file: docs/guide/themes.md } |  |  | 0.621 |
| ns | 4962 |  | 246 | Main screen keybindings, part 2 | 5.3 | 5.2 | 0.600 |
| walker |  | 5063 | 308 | Code::CodeKey { rung: Names, file: src/posting/collection.py, decl: 0, sub: 0, line: 0 } |  |  | 0.637 |
| walker |  | 5073 | 10 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 10, sub: 0, line: 66 } |  |  | 0.637 |
| walker |  | 5089 | 16 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 11, sub: 0, line: 70 } |  |  | 0.637 |
| walker |  | 5110 | 21 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 31, sub: 0, line: 407 } |  |  | 0.637 |
| walker |  | 5131 | 21 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 32, sub: 0, line: 412 } |  |  | 0.637 |
| walker |  | 5153 | 22 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 8, sub: 0, line: 56 } |  |  | 0.637 |
| ns | 5155 |  | 193 | Global app keybindings | 5.4 | 5.1 | 0.623 |
| walker |  | 5175 | 22 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 9, sub: 0, line: 61 } |  |  | 0.623 |
| walker |  | 5204 | 29 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 12, sub: 0, line: 75 } |  |  | 0.625 |
| walker |  | 5233 | 29 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 13, sub: 0, line: 81 } |  |  | 0.625 |
| walker |  | 5262 | 29 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 14, sub: 0, line: 87 } |  |  | 0.625 |
| walker |  | 5300 | 38 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 30, sub: 0, line: 401 } |  |  | 0.625 |
| walker |  | 5356 | 56 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 15, sub: 0, line: 93 } |  |  | 0.625 |
| walker |  | 5362 | 6 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 16, sub: 0, line: 98 } |  |  | 0.625 |
| walker |  | 5433 | 71 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 17, sub: 0, line: 103 } |  |  | 0.634 |
| ns | 5444 |  | 289 | MainScreen method roster, part 1 | 5.5 |  | 0.618 |
| walker |  | 5518 | 85 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 33, sub: 0, line: 417 } |  |  | 0.618 |
| walker |  | 5643 | 125 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 21, sub: 0, line: 138 } |  |  | 0.626 |
| ns | 5679 |  | 235 | MainScreen method roster, part 2 | 5.6 |  | 0.614 |
| walker |  | 5774 | 131 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 18, sub: 0, line: 111 } |  |  | 0.621 |
| ns | 5823 |  | 144 | MainScreen.compose: the widget layout | 5.7 | 5.5 | 0.612 |
| walker |  | 5914 | 140 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 34, sub: 0, line: 427 } |  |  | 0.619 |
| walker |  | 5922 | 8 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 37, sub: 0, line: 480 } |  |  | 0.620 |
| walker |  | 5959 | 37 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 36, sub: 0, line: 441 } |  |  | 0.620 |
| walker |  | 6000 | 41 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 35, sub: 0, line: 434 } |  |  | 0.620 |
| walker |  | 6015 | 15 | Code::CodeKey { rung: Doc, file: src/posting/collection.py, decl: 38, sub: 0, line: 541 } |  |  | 0.620 |
| ns | 6072 |  | 249 | `Posting` App method roster | 5.8 |  | 0.608 |
| walker |  | 6182 | 167 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 3, sub: 0, line: 23 } |  |  | 0.608 |
| walker |  | 6190 | 8 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 5, sub: 0, line: 41 } |  |  | 0.608 |
| walker |  | 6198 | 8 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 6, sub: 0, line: 45 } |  |  | 0.608 |
| walker |  | 6206 | 8 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 7, sub: 0, line: 51 } |  |  | 0.608 |
| walker |  | 6242 | 36 | Code::CodeKey { rung: Doc, file: src/posting/collection.py, decl: 21, sub: 0, line: 138 } |  |  | 0.608 |
| walker |  | 6256 | 14 | Code::CodeKey { rung: Names, file: src/posting/messages.py, decl: 0, sub: 0, line: 0 } |  |  | 0.608 |
| walker |  | 6271 | 15 | Code::CodeKey { rung: Decl, file: src/posting/messages.py, decl: 1, sub: 0, line: 6 } |  |  | 0.608 |
| ns | 6311 |  | 239 | Command palette: every command | 6.1 |  | 0.599 |
| walker |  | 6356 | 85 | Code::CodeKey { rung: Names, file: src/posting/locations.py, decl: 0, sub: 0, line: 0 } |  |  | 0.601 |
| walker |  | 6368 | 12 | Code::CodeKey { rung: Body, file: src/posting/locations.py, decl: 6, sub: 0, line: 34 } |  |  | 0.602 |
| walker |  | 6381 | 13 | Code::CodeKey { rung: Body, file: src/posting/locations.py, decl: 4, sub: 0, line: 24 } |  |  | 0.602 |
| walker |  | 6395 | 14 | Code::CodeKey { rung: Doc, file: src/posting/locations.py, decl: 4, sub: 0, line: 24 } |  |  | 0.605 |
| walker |  | 6409 | 14 | Code::CodeKey { rung: Body, file: src/posting/locations.py, decl: 2, sub: 0, line: 12 } |  |  | 0.605 |
| walker |  | 6423 | 14 | Code::CodeKey { rung: Doc, file: src/posting/locations.py, decl: 2, sub: 0, line: 12 } |  |  | 0.607 |
| walker |  | 6438 | 15 | Code::CodeKey { rung: Doc, file: src/posting/locations.py, decl: 3, sub: 0, line: 17 } |  |  | 0.607 |
| walker |  | 6454 | 16 | Code::CodeKey { rung: Doc, file: src/posting/locations.py, decl: 5, sub: 0, line: 29 } |  |  | 0.610 |
| walker |  | 6466 | 12 | Code::CodeKey { rung: Body, file: src/posting/locations.py, decl: 5, sub: 0, line: 29 } |  |  | 0.610 |
| walker |  | 6523 | 57 | Code::CodeKey { rung: Names, file: src/posting/app.py, decl: 0, sub: 0, line: 0 } |  |  | 0.611 |
| walker |  | 6538 | 15 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 1, sub: 0, line: 93 } |  |  | 0.611 |
| walker |  | 6548 | 10 | Code::CodeKey { rung: Doc, file: src/posting/app.py, decl: 1, sub: 0, line: 93 } |  |  | 0.611 |
| walker |  | 6558 | 10 | Code::CodeKey { rung: Doc, file: src/posting/app.py, decl: 3, sub: 0, line: 108 } |  |  | 0.612 |
| walker |  | 6587 | 29 | Code::CodeKey { rung: Names, file: src/posting/types.py, decl: 0, sub: 0, line: 0 } |  |  | 0.612 |
| walker |  | 6659 | 72 | Code::CodeKey { rung: Decl, file: src/posting/types.py, decl: 2, sub: 0, line: 6 } |  |  | 0.612 |
| walker |  | 6674 | 15 | Code::CodeKey { rung: Names, file: src/posting/_start_time.py, decl: 0, sub: 0, line: 0 } |  |  | 0.612 |
| ns | 6694 |  | 383 | Request-editor widget class roster | 6.2 |  | 0.597 |
| walker |  | 6796 | 122 | Code::CodeKey { rung: Names, file: src/posting/scripts.py, decl: 0, sub: 0, line: 0 } |  |  | 0.597 |
| walker |  | 6825 | 29 | Code::CodeKey { rung: Decl, file: src/posting/scripts.py, decl: 13, sub: 0, line: 164 } |  |  | 0.597 |
| walker |  | 6858 | 33 | Code::CodeKey { rung: Decl, file: src/posting/scripts.py, decl: 12, sub: 0, line: 121 } |  |  | 0.597 |
| walker |  | 6872 | 14 | Code::CodeKey { rung: Body, file: src/posting/scripts.py, decl: 14, sub: 0, line: 191 } |  |  | 0.597 |
| walker |  | 6891 | 19 | Code::CodeKey { rung: Doc, file: src/posting/scripts.py, decl: 13, sub: 0, line: 164 } |  |  | 0.597 |
| ns | 6902 |  | 208 | Response and collection-browser widget class roster | 6.3 |  | 0.590 |
| walker |  | 7023 | 132 | Code::CodeKey { rung: Decl, file: src/posting/scripts.py, decl: 3, sub: 0, line: 23 } |  |  | 0.590 |
| walker |  | 7031 | 8 | Code::CodeKey { rung: Decl, file: src/posting/scripts.py, decl: 5, sub: 0, line: 36 } |  |  | 0.590 |
| walker |  | 7095 | 64 | Code::CodeKey { rung: Decl, file: src/posting/scripts.py, decl: 10, sub: 0, line: 86 } |  |  | 0.590 |
| walker |  | 7106 | 11 | Code::CodeKey { rung: Doc, file: src/posting/scripts.py, decl: 9, sub: 0, line: 81 } |  |  | 0.590 |
| walker |  | 7124 | 18 | Code::CodeKey { rung: Doc, file: src/posting/scripts.py, decl: 3, sub: 0, line: 23 } |  |  | 0.590 |
| walker |  | 7152 | 28 | Code::CodeKey { rung: Doc, file: src/posting/scripts.py, decl: 11, sub: 0, line: 113 } |  |  | 0.590 |
| walker |  | 7196 | 44 | Code::CodeKey { rung: Doc, file: src/posting/scripts.py, decl: 8, sub: 0, line: 71 } |  |  | 0.590 |
| ns | 7238 |  | 336 | Shared widget class roster | 6.4 |  | 0.580 |
| walker |  | 7249 | 53 | Code::CodeKey { rung: Doc, file: src/posting/scripts.py, decl: 15, sub: 0, line: 204 } |  |  | 0.580 |
| walker |  | 7305 | 56 | Code::CodeKey { rung: Doc, file: src/posting/scripts.py, decl: 5, sub: 0, line: 36 } |  |  | 0.580 |
| walker |  | 7336 | 31 | Code::CodeKey { rung: Names, file: src/posting/request_headers.py, decl: 0, sub: 0, line: 0 } |  |  | 0.580 |
| walker |  | 7379 | 43 | Code::CodeKey { rung: Decl, file: src/posting/request_headers.py, decl: 1, sub: 0, line: 4 } |  |  | 0.580 |
| ns | 7440 |  | 202 | Scripting API: the `Posting` object handed to user scripts | 6.5 |  | 0.583 |
| walker |  | 7454 | 75 | Code::CodeKey { rung: Doc, file: src/posting/scripts.py, decl: 14, sub: 0, line: 191 } |  |  | 0.583 |
| ns | 7732 |  | 292 | Variables subsystem | 6.6 |  | 0.576 |
| walker |  | 7776 | 322 | Code::CodeKey { rung: Names, file: src/posting/themes.py, decl: 0, sub: 0, line: 0 } |  |  | 0.576 |
| walker |  | 7864 | 88 | Code::CodeKey { rung: Decl, file: src/posting/themes.py, decl: 11, sub: 0, line: 305 } |  |  | 0.576 |
| walker |  | 7955 | 91 | Code::CodeKey { rung: Decl, file: src/posting/themes.py, decl: 3, sub: 0, line: 54 } |  |  | 0.576 |
| walker |  | 7966 | 11 | Code::CodeKey { rung: Doc, file: src/posting/themes.py, decl: 3, sub: 0, line: 54 } |  |  | 0.576 |
| ns | 8068 |  | 336 | Theme model | 6.7 |  | 0.566 |
| walker |  | 8101 | 135 | Code::CodeKey { rung: Decl, file: src/posting/themes.py, decl: 5, sub: 0, line: 72 } |  |  | 0.566 |
| walker |  | 8114 | 13 | Code::CodeKey { rung: Doc, file: src/posting/themes.py, decl: 5, sub: 0, line: 72 } |  |  | 0.566 |
| walker |  | 8249 | 135 | Code::CodeKey { rung: Decl, file: src/posting/themes.py, decl: 7, sub: 0, line: 94 } |  |  | 0.566 |
| ns | 8260 |  | 192 | Every builtin theme name | 6.8 |  | 0.560 |
| walker |  | 8264 | 15 | Code::CodeKey { rung: Doc, file: src/posting/themes.py, decl: 7, sub: 0, line: 94 } |  |  | 0.560 |
| ns | 8413 |  | 153 | Importer entry points | 6.9 |  | 0.556 |
| walker |  | 8437 | 173 | Code::CodeKey { rung: Decl, file: src/posting/themes.py, decl: 2, sub: 0, line: 34 } |  |  | 0.556 |
| walker |  | 8460 | 23 | Code::CodeKey { rung: Doc, file: src/posting/themes.py, decl: 2, sub: 0, line: 34 } |  |  | 0.556 |
| walker |  | 8492 | 32 | Code::CodeKey { rung: Doc, file: src/posting/themes.py, decl: 4, sub: 0, line: 63 } |  |  | 0.556 |
| walker |  | 8524 | 32 | Code::CodeKey { rung: Doc, file: src/posting/themes.py, decl: 6, sub: 0, line: 84 } |  |  | 0.556 |
| ns | 8530 |  | 117 | URL and path-parameter helpers | 6.10 |  | 0.554 |
| ns | 8694 |  | 164 | HTTP header catalogue | 6.11 |  | 0.550 |
| walker |  | 8739 | 215 | Code::CodeKey { rung: Decl, file: src/posting/themes.py, decl: 1, sub: 0, line: 14 } |  |  | 0.550 |
| walker |  | 8816 | 77 | Code::CodeKey { rung: Doc, file: src/posting/collection.py, decl: 39, sub: 0, line: 553 } |  |  | 0.550 |
| walker |  | 8833 | 17 | Code::CodeKey { rung: Names, file: src/posting/suggesters.py, decl: 0, sub: 0, line: 0 } |  |  | 0.550 |
| walker |  | 8943 | 110 | Code::CodeKey { rung: Decl, file: src/posting/suggesters.py, decl: 1, sub: 0, line: 2 } |  |  | 0.550 |
| walker |  | 8960 | 17 | Code::CodeKey { rung: Names, file: src/posting/user_host.py, decl: 0, sub: 0, line: 0 } |  |  | 0.550 |
| ns | 8989 |  | 295 | Test invocation: the Makefile is mandatory | 7.1 |  | 0.553 |
| walker |  | 9030 | 70 | Code::CodeKey { rung: Body, file: src/posting/user_host.py, decl: 1, sub: 0, line: 9 } |  |  | 0.553 |
| walker |  | 9064 | 34 | Code::CodeKey { rung: Names, file: src/posting/xresources.py, decl: 0, sub: 0, line: 0 } |  |  | 0.553 |
| ns | 9139 |  | 150 | Development environment and snapshot testing | 7.2 |  | 0.550 |
| walker |  | 9165 | 101 | Code::CodeKey { rung: Decl, file: src/posting/xresources.py, decl: 1, sub: 0, line: 8 } |  |  | 0.550 |
| walker |  | 9188 | 23 | Code::CodeKey { rung: Doc, file: src/posting/xresources.py, decl: 2, sub: 0, line: 20 } |  |  | 0.550 |
| walker |  | 9229 | 41 | Markdown::HeadingsOutline { file: docs/guide/keymap.md } |  |  | 0.550 |
| walker |  | 9247 | 18 | Code::CodeKey { rung: Names, file: src/posting/auth.py, decl: 0, sub: 0, line: 0 } |  |  | 0.550 |
| walker |  | 9293 | 46 | Code::CodeKey { rung: Decl, file: src/posting/auth.py, decl: 1, sub: 0, line: 6 } |  |  | 0.550 |
| walker |  | 9301 | 8 | Code::CodeKey { rung: Body, file: src/posting/auth.py, decl: 2, sub: 0, line: 7 } |  |  | 0.550 |
| walker |  | 9325 | 24 | Code::CodeKey { rung: Body, file: src/posting/auth.py, decl: 3, sub: 0, line: 10 } |  |  | 0.550 |
| ns | 9407 |  | 268 | Scripting and core workflow guide section maps | 7.3 |  | 0.542 |
| walker |  | 9631 | 306 | Code::CodeKey { rung: Decl, file: src/posting/themes.py, decl: 8, sub: 0, line: 106 } |  |  | 0.550 |
| ns | 9692 |  | 285 | Remaining guide section maps | 7.4 |  | 0.542 |
| walker |  | 9848 | 217 | Code::CodeKey { rung: Decl, file: src/posting/themes.py, decl: 8, sub: 1, line: 106 } |  |  | 0.556 |
| walker |  | 9880 | 32 | Code::CodeKey { rung: Decl, file: src/posting/themes.py, decl: 10, sub: 0, line: 244 } |  |  | 0.556 |
| walker |  | 9930 | 50 | Code::CodeKey { rung: Doc, file: src/posting/themes.py, decl: 9, sub: 0, line: 144 } |  |  | 0.557 |
| ns | 9981 |  | 289 | Remaining packaging metadata and CI | 7.5 | 1.7 | 0.547 |
| walker |  | 9986 | 56 | Code::CodeKey { rung: Doc, file: src/posting/themes.py, decl: 10, sub: 0, line: 244 } |  |  | 0.547 |

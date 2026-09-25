Score(3000)=0.769 I=0.922 C=0.642 ns_rows≤3K=19/49 grid(1000/1442/2080/3000/4327/6240/9000)=0.746/0.830/0.790/0.769/0.654/0.614/0.553

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
| walker |  | 1150 | 50 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.785 |
| ns | 1270 |  | 130 | Test tree and sample-collection listings | 1.11 |  | 0.731 |
| walker |  | 1407 | 257 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.830 |
| ns | 1460 |  | 190 | Complete `posting` subcommand roster | 2.1 |  | 0.803 |
| walker |  | 1633 | 226 | Code::CodeKey { rung: Names, file: src/posting/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.803 |
| ns | 1689 |  | 229 | Every CLI argument and option | 2.2 |  | 0.763 |
| walker |  | 1732 | 99 | Fs::DirListing { dir: tests } |  |  | 0.803 |
| walker |  | 1869 | 137 | Code::CodeKey { rung: Names, file: src/posting/__main__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.812 |
| walker |  | 1881 | 12 | Code::CodeKey { rung: Decl, file: src/posting/__main__.py, decl: 7, sub: 0, line: 177 } |  |  | 0.814 |
| walker |  | 1903 | 22 | Code::CodeKey { rung: Decl, file: src/posting/__main__.py, decl: 3, sub: 0, line: 45 } |  |  | 0.819 |
| ns | 1907 |  | 218 | `make_posting`: CLI-to-app wiring | 2.3 |  | 0.791 |
| walker |  | 1935 | 32 | Code::CodeKey { rung: Decl, file: src/posting/__main__.py, decl: 5, sub: 0, line: 76 } |  |  | 0.791 |
| walker |  | 1979 | 44 | Code::CodeKey { rung: Decl, file: src/posting/__main__.py, decl: 8, sub: 0, line: 194 } |  |  | 0.794 |
| walker |  | 1991 | 12 | Code::CodeKey { rung: Doc, file: src/posting/__main__.py, decl: 7, sub: 0, line: 177 } |  |  | 0.797 |
| walker |  | 2005 | 14 | Code::CodeKey { rung: Doc, file: src/posting/__main__.py, decl: 3, sub: 0, line: 45 } |  |  | 0.802 |
| walker |  | 2022 | 17 | Code::CodeKey { rung: Doc, file: src/posting/__main__.py, decl: 8, sub: 0, line: 194 } |  |  | 0.804 |
| ns | 2052 |  | 145 | XDG locations: config file, themes and default collection | 2.4 |  | 0.785 |
| walker |  | 2149 | 127 | Code::CodeKey { rung: Decl, file: src/posting/__main__.py, decl: 4, sub: 0, line: 50 } |  |  | 0.801 |
| walker |  | 2281 | 132 | Code::CodeKey { rung: Decl, file: src/posting/__main__.py, decl: 6, sub: 0, line: 96 } |  |  | 0.835 |
| walker |  | 2297 | 16 | Code::CodeKey { rung: Doc, file: src/posting/__main__.py, decl: 6, sub: 0, line: 96 } |  |  | 0.841 |
| ns | 2359 |  | 307 | collection.py type roster | 3.1 |  | 0.798 |
| ns | 2534 |  | 175 | The `.posting.yaml` on-disk request format | 3.2 |  | 0.763 |
| walker |  | 2597 | 300 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.808 |
| walker |  | 2602 | 5 | Fs::DirListing { dir: .codex/environments } |  |  | 0.808 |
| walker |  | 2607 | 5 | Fs::DirListing { dir: tests/__snapshots__ } |  |  | 0.808 |
| ns | 2774 |  | 240 | RequestModel field roster | 3.3 | 3.1 | 0.781 |
| walker |  | 2834 | 227 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.781 |
| walker |  | 2842 | 8 | Fs::DirListing { dir: tests/resources } |  |  | 0.781 |
| walker |  | 2852 | 10 | Code::CodeKey { rung: Names, file: src/posting/exit_codes.py, decl: 0, sub: 0, line: 0 } |  |  | 0.781 |
| walker |  | 2864 | 12 | Code::CodeKey { rung: Names, file: src/posting/help_data.py, decl: 0, sub: 0, line: 0 } |  |  | 0.781 |
| walker |  | 2923 | 59 | Code::CodeKey { rung: Decl, file: src/posting/help_data.py, decl: 1, sub: 0, line: 4 } |  |  | 0.781 |
| ns | 2934 |  | 160 | RequestModel behaviour roster | 3.4 |  | 0.769 |
| walker |  | 2940 | 17 | Code::CodeKey { rung: Doc, file: src/posting/help_data.py, decl: 1, sub: 0, line: 4 } |  |  | 0.769 |
| walker |  | 2952 | 12 | Code::CodeKey { rung: Names, file: src/posting/version.py, decl: 0, sub: 0, line: 0 } |  |  | 0.769 |
| walker |  | 2962 | 10 | Fs::DirListing { dir: tests/sample-envs } |  |  | 0.769 |
| walker |  | 2999 | 37 | Code::CodeKey { rung: Names, file: src/posting/jumper.py, decl: 0, sub: 0, line: 0 } |  |  | 0.769 |
| walker |  | 3018 | 19 | Code::CodeKey { rung: Decl, file: src/posting/jumper.py, decl: 1, sub: 0, line: 9 } |  |  | 0.769 |
| walker |  | 3069 | 51 | Code::CodeKey { rung: Decl, file: src/posting/jumper.py, decl: 3, sub: 0, line: 26 } |  |  | 0.769 |
| walker |  | 3125 | 56 | Code::CodeKey { rung: Decl, file: src/posting/jumper.py, decl: 2, sub: 0, line: 16 } |  |  | 0.769 |
| walker |  | 3138 | 13 | Code::CodeKey { rung: Doc, file: src/posting/jumper.py, decl: 1, sub: 0, line: 9 } |  |  | 0.769 |
| ns | 3145 |  | 211 | Key-value, option and script models | 3.5 | 3.1 | 0.741 |
| walker |  | 3152 | 14 | Code::CodeKey { rung: Doc, file: src/posting/jumper.py, decl: 2, sub: 0, line: 16 } |  |  | 0.741 |
| walker |  | 3166 | 14 | Code::CodeKey { rung: Doc, file: src/posting/jumper.py, decl: 3, sub: 0, line: 26 } |  |  | 0.741 |
| walker |  | 3181 | 15 | Code::CodeKey { rung: Doc, file: src/posting/jumper.py, decl: 5, sub: 0, line: 34 } |  |  | 0.741 |
| walker |  | 3312 | 131 | Code::CodeKey { rung: Names, file: src/posting/config.py, decl: 0, sub: 0, line: 0 } |  |  | 0.742 |
| walker |  | 3346 | 34 | Code::CodeKey { rung: Decl, file: src/posting/config.py, decl: 6, sub: 0, line: 92 } |  |  | 0.742 |
| ns | 3349 |  | 204 | Collection tree: loading a directory, saving it back | 3.6 | 3.1 | 0.722 |
| walker |  | 3387 | 41 | Code::CodeKey { rung: Decl, file: src/posting/config.py, decl: 7, sub: 0, line: 99 } |  |  | 0.722 |
| walker |  | 3397 | 10 | Code::CodeKey { rung: Doc, file: src/posting/config.py, decl: 6, sub: 0, line: 92 } |  |  | 0.722 |
| walker |  | 3407 | 10 | Code::CodeKey { rung: Doc, file: src/posting/config.py, decl: 7, sub: 0, line: 99 } |  |  | 0.722 |
| walker |  | 3481 | 74 | Code::CodeKey { rung: Decl, file: src/posting/config.py, decl: 3, sub: 0, line: 46 } |  |  | 0.722 |
| walker |  | 3491 | 10 | Code::CodeKey { rung: Doc, file: src/posting/config.py, decl: 3, sub: 0, line: 46 } |  |  | 0.722 |
| ns | 3563 |  | 214 | config.py structure and settings-source configuration | 4.1 |  | 0.712 |
| walker |  | 3565 | 74 | Code::CodeKey { rung: Decl, file: src/posting/config.py, decl: 8, sub: 0, line: 106 } |  |  | 0.712 |
| walker |  | 3575 | 10 | Code::CodeKey { rung: Doc, file: src/posting/config.py, decl: 8, sub: 0, line: 106 } |  |  | 0.712 |
| walker |  | 3696 | 121 | Code::CodeKey { rung: Decl, file: src/posting/config.py, decl: 5, sub: 0, line: 79 } |  |  | 0.712 |
| walker |  | 3709 | 13 | Code::CodeKey { rung: Doc, file: src/posting/config.py, decl: 5, sub: 0, line: 79 } |  |  | 0.712 |
| ns | 3714 |  | 151 | Settings keys, part 1 | 4.2 |  | 0.698 |
| walker |  | 3849 | 140 | Code::CodeKey { rung: Decl, file: src/posting/config.py, decl: 2, sub: 0, line: 34 } |  |  | 0.698 |
| ns | 3892 |  | 178 | Settings keys, part 2 | 4.3 | 4.2 | 0.685 |
| walker |  | 4018 | 169 | Code::CodeKey { rung: Decl, file: src/posting/config.py, decl: 1, sub: 0, line: 18 } |  |  | 0.685 |
| ns | 4250 |  | 358 | Configuration precedence and guide section map | 4.4 |  | 0.654 |
| walker |  | 4312 | 294 | Code::CodeKey { rung: Decl, file: src/posting/config.py, decl: 4, sub: 0, line: 56 } |  |  | 0.654 |
| walker |  | 4321 | 9 | Code::CodeKey { rung: Doc, file: src/posting/config.py, decl: 4, sub: 0, line: 56 } |  |  | 0.654 |
| walker |  | 4387 | 66 | Code::CodeKey { rung: Names, file: src/posting/help_screen.py, decl: 0, sub: 0, line: 0 } |  |  | 0.654 |
| walker |  | 4406 | 19 | Code::CodeKey { rung: Decl, file: src/posting/help_screen.py, decl: 1, sub: 0, line: 14 } |  |  | 0.654 |
| walker |  | 4462 | 56 | Code::CodeKey { rung: Decl, file: src/posting/help_screen.py, decl: 2, sub: 0, line: 22 } |  |  | 0.654 |
| walker |  | 4471 | 9 | Code::CodeKey { rung: Doc, file: src/posting/help_screen.py, decl: 2, sub: 0, line: 22 } |  |  | 0.654 |
| ns | 4526 |  | 276 | app.py class roster and MainScreen reactive state | 5.1 |  | 0.638 |
| walker |  | 4527 | 56 | Code::CodeKey { rung: Decl, file: src/posting/help_screen.py, decl: 3, sub: 0, line: 33 } |  |  | 0.638 |
| walker |  | 4536 | 9 | Code::CodeKey { rung: Doc, file: src/posting/help_screen.py, decl: 3, sub: 0, line: 33 } |  |  | 0.638 |
| walker |  | 4547 | 11 | Code::CodeKey { rung: Doc, file: src/posting/help_screen.py, decl: 4, sub: 0, line: 44 } |  |  | 0.638 |
| walker |  | 4573 | 26 | Code::CodeKey { rung: Doc, file: src/posting/help_screen.py, decl: 1, sub: 0, line: 14 } |  |  | 0.638 |
| walker |  | 4584 | 11 | Fs::DirListing { dir: tests/sample-themes } |  |  | 0.638 |
| ns | 4716 |  | 190 | Main screen keybindings, part 1 | 5.2 | 5.1 | 0.621 |
| walker |  | 4892 | 308 | Code::CodeKey { rung: Names, file: src/posting/collection.py, decl: 0, sub: 0, line: 0 } |  |  | 0.659 |
| walker |  | 4902 | 10 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 10, sub: 0, line: 66 } |  |  | 0.659 |
| walker |  | 4918 | 16 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 11, sub: 0, line: 70 } |  |  | 0.659 |
| walker |  | 4939 | 21 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 31, sub: 0, line: 407 } |  |  | 0.659 |
| walker |  | 4960 | 21 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 32, sub: 0, line: 412 } |  |  | 0.659 |
| ns | 4962 |  | 246 | Main screen keybindings, part 2 | 5.3 | 5.2 | 0.637 |
| walker |  | 4982 | 22 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 8, sub: 0, line: 56 } |  |  | 0.637 |
| walker |  | 5004 | 22 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 9, sub: 0, line: 61 } |  |  | 0.637 |
| walker |  | 5033 | 29 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 12, sub: 0, line: 75 } |  |  | 0.639 |
| walker |  | 5062 | 29 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 13, sub: 0, line: 81 } |  |  | 0.639 |
| walker |  | 5091 | 29 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 14, sub: 0, line: 87 } |  |  | 0.639 |
| walker |  | 5129 | 38 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 30, sub: 0, line: 401 } |  |  | 0.639 |
| ns | 5155 |  | 193 | Global app keybindings | 5.4 | 5.1 | 0.625 |
| walker |  | 5185 | 56 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 15, sub: 0, line: 93 } |  |  | 0.625 |
| walker |  | 5191 | 6 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 16, sub: 0, line: 98 } |  |  | 0.625 |
| walker |  | 5262 | 71 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 17, sub: 0, line: 103 } |  |  | 0.634 |
| walker |  | 5347 | 85 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 33, sub: 0, line: 417 } |  |  | 0.634 |
| ns | 5444 |  | 289 | MainScreen method roster, part 1 | 5.5 |  | 0.618 |
| walker |  | 5472 | 125 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 21, sub: 0, line: 138 } |  |  | 0.626 |
| walker |  | 5603 | 131 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 18, sub: 0, line: 111 } |  |  | 0.633 |
| ns | 5679 |  | 235 | MainScreen method roster, part 2 | 5.6 |  | 0.621 |
| walker |  | 5743 | 140 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 34, sub: 0, line: 427 } |  |  | 0.628 |
| walker |  | 5751 | 8 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 37, sub: 0, line: 480 } |  |  | 0.629 |
| walker |  | 5788 | 37 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 36, sub: 0, line: 441 } |  |  | 0.629 |
| ns | 5823 |  | 144 | MainScreen.compose: the widget layout | 5.7 | 5.5 | 0.620 |
| walker |  | 5829 | 41 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 35, sub: 0, line: 434 } |  |  | 0.620 |
| walker |  | 5844 | 15 | Code::CodeKey { rung: Doc, file: src/posting/collection.py, decl: 38, sub: 0, line: 541 } |  |  | 0.620 |
| walker |  | 6011 | 167 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 3, sub: 0, line: 23 } |  |  | 0.620 |
| walker |  | 6019 | 8 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 5, sub: 0, line: 41 } |  |  | 0.620 |
| walker |  | 6027 | 8 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 6, sub: 0, line: 45 } |  |  | 0.620 |
| walker |  | 6035 | 8 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 7, sub: 0, line: 51 } |  |  | 0.620 |
| walker |  | 6071 | 36 | Code::CodeKey { rung: Doc, file: src/posting/collection.py, decl: 21, sub: 0, line: 138 } |  |  | 0.620 |
| ns | 6072 |  | 249 | `Posting` App method roster | 5.8 |  | 0.608 |
| walker |  | 6085 | 14 | Code::CodeKey { rung: Names, file: src/posting/messages.py, decl: 0, sub: 0, line: 0 } |  |  | 0.608 |
| walker |  | 6100 | 15 | Code::CodeKey { rung: Decl, file: src/posting/messages.py, decl: 1, sub: 0, line: 6 } |  |  | 0.608 |
| walker |  | 6185 | 85 | Code::CodeKey { rung: Names, file: src/posting/locations.py, decl: 0, sub: 0, line: 0 } |  |  | 0.611 |
| walker |  | 6197 | 12 | Code::CodeKey { rung: Body, file: src/posting/locations.py, decl: 6, sub: 0, line: 34 } |  |  | 0.612 |
| walker |  | 6210 | 13 | Code::CodeKey { rung: Body, file: src/posting/locations.py, decl: 4, sub: 0, line: 24 } |  |  | 0.612 |
| walker |  | 6224 | 14 | Code::CodeKey { rung: Doc, file: src/posting/locations.py, decl: 4, sub: 0, line: 24 } |  |  | 0.614 |
| walker |  | 6238 | 14 | Code::CodeKey { rung: Body, file: src/posting/locations.py, decl: 2, sub: 0, line: 12 } |  |  | 0.614 |
| walker |  | 6252 | 14 | Code::CodeKey { rung: Doc, file: src/posting/locations.py, decl: 2, sub: 0, line: 12 } |  |  | 0.617 |
| walker |  | 6267 | 15 | Code::CodeKey { rung: Doc, file: src/posting/locations.py, decl: 3, sub: 0, line: 17 } |  |  | 0.617 |
| walker |  | 6283 | 16 | Code::CodeKey { rung: Doc, file: src/posting/locations.py, decl: 5, sub: 0, line: 29 } |  |  | 0.620 |
| walker |  | 6295 | 12 | Code::CodeKey { rung: Body, file: src/posting/locations.py, decl: 5, sub: 0, line: 29 } |  |  | 0.620 |
| ns | 6311 |  | 239 | Command palette: every command | 6.1 |  | 0.610 |
| walker |  | 6352 | 57 | Code::CodeKey { rung: Names, file: src/posting/app.py, decl: 0, sub: 0, line: 0 } |  |  | 0.611 |
| walker |  | 6367 | 15 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 1, sub: 0, line: 93 } |  |  | 0.611 |
| walker |  | 6377 | 10 | Code::CodeKey { rung: Doc, file: src/posting/app.py, decl: 1, sub: 0, line: 93 } |  |  | 0.611 |
| walker |  | 6387 | 10 | Code::CodeKey { rung: Doc, file: src/posting/app.py, decl: 3, sub: 0, line: 108 } |  |  | 0.612 |
| walker |  | 6416 | 29 | Code::CodeKey { rung: Names, file: src/posting/types.py, decl: 0, sub: 0, line: 0 } |  |  | 0.612 |
| walker |  | 6488 | 72 | Code::CodeKey { rung: Decl, file: src/posting/types.py, decl: 2, sub: 0, line: 6 } |  |  | 0.612 |
| walker |  | 6503 | 15 | Code::CodeKey { rung: Names, file: src/posting/_start_time.py, decl: 0, sub: 0, line: 0 } |  |  | 0.612 |
| walker |  | 6625 | 122 | Code::CodeKey { rung: Names, file: src/posting/scripts.py, decl: 0, sub: 0, line: 0 } |  |  | 0.612 |
| walker |  | 6654 | 29 | Code::CodeKey { rung: Decl, file: src/posting/scripts.py, decl: 13, sub: 0, line: 164 } |  |  | 0.612 |
| walker |  | 6687 | 33 | Code::CodeKey { rung: Decl, file: src/posting/scripts.py, decl: 12, sub: 0, line: 121 } |  |  | 0.612 |
| ns | 6694 |  | 383 | Request-editor widget class roster | 6.2 |  | 0.597 |
| walker |  | 6701 | 14 | Code::CodeKey { rung: Body, file: src/posting/scripts.py, decl: 14, sub: 0, line: 191 } |  |  | 0.597 |
| walker |  | 6720 | 19 | Code::CodeKey { rung: Doc, file: src/posting/scripts.py, decl: 13, sub: 0, line: 164 } |  |  | 0.597 |
| walker |  | 6852 | 132 | Code::CodeKey { rung: Decl, file: src/posting/scripts.py, decl: 3, sub: 0, line: 23 } |  |  | 0.597 |
| walker |  | 6860 | 8 | Code::CodeKey { rung: Decl, file: src/posting/scripts.py, decl: 5, sub: 0, line: 36 } |  |  | 0.597 |
| ns | 6902 |  | 208 | Response and collection-browser widget class roster | 6.3 |  | 0.590 |
| walker |  | 6924 | 64 | Code::CodeKey { rung: Decl, file: src/posting/scripts.py, decl: 10, sub: 0, line: 86 } |  |  | 0.590 |
| walker |  | 6935 | 11 | Code::CodeKey { rung: Doc, file: src/posting/scripts.py, decl: 9, sub: 0, line: 81 } |  |  | 0.590 |
| walker |  | 6953 | 18 | Code::CodeKey { rung: Doc, file: src/posting/scripts.py, decl: 3, sub: 0, line: 23 } |  |  | 0.590 |
| walker |  | 6981 | 28 | Code::CodeKey { rung: Doc, file: src/posting/scripts.py, decl: 11, sub: 0, line: 113 } |  |  | 0.590 |
| walker |  | 7025 | 44 | Code::CodeKey { rung: Doc, file: src/posting/scripts.py, decl: 8, sub: 0, line: 71 } |  |  | 0.590 |
| walker |  | 7078 | 53 | Code::CodeKey { rung: Doc, file: src/posting/scripts.py, decl: 15, sub: 0, line: 204 } |  |  | 0.590 |
| walker |  | 7134 | 56 | Code::CodeKey { rung: Doc, file: src/posting/scripts.py, decl: 5, sub: 0, line: 36 } |  |  | 0.590 |
| walker |  | 7165 | 31 | Code::CodeKey { rung: Names, file: src/posting/request_headers.py, decl: 0, sub: 0, line: 0 } |  |  | 0.590 |
| walker |  | 7208 | 43 | Code::CodeKey { rung: Decl, file: src/posting/request_headers.py, decl: 1, sub: 0, line: 4 } |  |  | 0.590 |
| ns | 7238 |  | 336 | Shared widget class roster | 6.4 |  | 0.580 |
| walker |  | 7283 | 75 | Code::CodeKey { rung: Doc, file: src/posting/scripts.py, decl: 14, sub: 0, line: 191 } |  |  | 0.580 |
| ns | 7440 |  | 202 | Scripting API: the `Posting` object handed to user scripts | 6.5 |  | 0.583 |
| walker |  | 7605 | 322 | Code::CodeKey { rung: Names, file: src/posting/themes.py, decl: 0, sub: 0, line: 0 } |  |  | 0.583 |
| walker |  | 7693 | 88 | Code::CodeKey { rung: Decl, file: src/posting/themes.py, decl: 11, sub: 0, line: 305 } |  |  | 0.583 |
| ns | 7732 |  | 292 | Variables subsystem | 6.6 |  | 0.576 |
| walker |  | 7784 | 91 | Code::CodeKey { rung: Decl, file: src/posting/themes.py, decl: 3, sub: 0, line: 54 } |  |  | 0.576 |
| walker |  | 7795 | 11 | Code::CodeKey { rung: Doc, file: src/posting/themes.py, decl: 3, sub: 0, line: 54 } |  |  | 0.576 |
| walker |  | 7930 | 135 | Code::CodeKey { rung: Decl, file: src/posting/themes.py, decl: 5, sub: 0, line: 72 } |  |  | 0.576 |
| walker |  | 7943 | 13 | Code::CodeKey { rung: Doc, file: src/posting/themes.py, decl: 5, sub: 0, line: 72 } |  |  | 0.576 |
| ns | 8068 |  | 336 | Theme model | 6.7 |  | 0.566 |
| walker |  | 8078 | 135 | Code::CodeKey { rung: Decl, file: src/posting/themes.py, decl: 7, sub: 0, line: 94 } |  |  | 0.566 |
| walker |  | 8093 | 15 | Code::CodeKey { rung: Doc, file: src/posting/themes.py, decl: 7, sub: 0, line: 94 } |  |  | 0.566 |
| ns | 8260 |  | 192 | Every builtin theme name | 6.8 |  | 0.560 |
| walker |  | 8266 | 173 | Code::CodeKey { rung: Decl, file: src/posting/themes.py, decl: 2, sub: 0, line: 34 } |  |  | 0.560 |
| walker |  | 8289 | 23 | Code::CodeKey { rung: Doc, file: src/posting/themes.py, decl: 2, sub: 0, line: 34 } |  |  | 0.560 |
| walker |  | 8321 | 32 | Code::CodeKey { rung: Doc, file: src/posting/themes.py, decl: 4, sub: 0, line: 63 } |  |  | 0.560 |
| walker |  | 8353 | 32 | Code::CodeKey { rung: Doc, file: src/posting/themes.py, decl: 6, sub: 0, line: 84 } |  |  | 0.560 |
| ns | 8413 |  | 153 | Importer entry points | 6.9 |  | 0.556 |
| ns | 8530 |  | 117 | URL and path-parameter helpers | 6.10 |  | 0.554 |
| walker |  | 8568 | 215 | Code::CodeKey { rung: Decl, file: src/posting/themes.py, decl: 1, sub: 0, line: 14 } |  |  | 0.554 |
| walker |  | 8645 | 77 | Code::CodeKey { rung: Doc, file: src/posting/collection.py, decl: 39, sub: 0, line: 553 } |  |  | 0.554 |
| walker |  | 8662 | 17 | Code::CodeKey { rung: Names, file: src/posting/suggesters.py, decl: 0, sub: 0, line: 0 } |  |  | 0.554 |
| ns | 8694 |  | 164 | HTTP header catalogue | 6.11 |  | 0.550 |
| walker |  | 8772 | 110 | Code::CodeKey { rung: Decl, file: src/posting/suggesters.py, decl: 1, sub: 0, line: 2 } |  |  | 0.550 |
| walker |  | 8789 | 17 | Code::CodeKey { rung: Names, file: src/posting/user_host.py, decl: 0, sub: 0, line: 0 } |  |  | 0.550 |
| walker |  | 8859 | 70 | Code::CodeKey { rung: Body, file: src/posting/user_host.py, decl: 1, sub: 0, line: 9 } |  |  | 0.550 |
| walker |  | 8893 | 34 | Code::CodeKey { rung: Names, file: src/posting/xresources.py, decl: 0, sub: 0, line: 0 } |  |  | 0.550 |
| ns | 8989 |  | 295 | Test invocation: the Makefile is mandatory | 7.1 |  | 0.553 |
| walker |  | 8994 | 101 | Code::CodeKey { rung: Decl, file: src/posting/xresources.py, decl: 1, sub: 0, line: 8 } |  |  | 0.553 |
| walker |  | 9017 | 23 | Code::CodeKey { rung: Doc, file: src/posting/xresources.py, decl: 2, sub: 0, line: 20 } |  |  | 0.553 |
| walker |  | 9035 | 18 | Code::CodeKey { rung: Names, file: src/posting/auth.py, decl: 0, sub: 0, line: 0 } |  |  | 0.553 |
| walker |  | 9081 | 46 | Code::CodeKey { rung: Decl, file: src/posting/auth.py, decl: 1, sub: 0, line: 6 } |  |  | 0.553 |
| walker |  | 9089 | 8 | Code::CodeKey { rung: Body, file: src/posting/auth.py, decl: 2, sub: 0, line: 7 } |  |  | 0.553 |
| walker |  | 9113 | 24 | Code::CodeKey { rung: Body, file: src/posting/auth.py, decl: 3, sub: 0, line: 10 } |  |  | 0.553 |
| ns | 9139 |  | 150 | Development environment and snapshot testing | 7.2 |  | 0.550 |
| ns | 9407 |  | 268 | Scripting and core workflow guide section maps | 7.3 |  | 0.541 |
| walker |  | 9419 | 306 | Code::CodeKey { rung: Decl, file: src/posting/themes.py, decl: 8, sub: 0, line: 106 } |  |  | 0.549 |
| walker |  | 9636 | 217 | Code::CodeKey { rung: Decl, file: src/posting/themes.py, decl: 8, sub: 1, line: 106 } |  |  | 0.563 |
| walker |  | 9668 | 32 | Code::CodeKey { rung: Decl, file: src/posting/themes.py, decl: 10, sub: 0, line: 244 } |  |  | 0.563 |
| ns | 9692 |  | 285 | Remaining guide section maps | 7.4 |  | 0.554 |
| walker |  | 9718 | 50 | Code::CodeKey { rung: Doc, file: src/posting/themes.py, decl: 9, sub: 0, line: 144 } |  |  | 0.555 |
| walker |  | 9802 | 84 | Code::CodeKey { rung: Doc, file: src/posting/themes.py, decl: 10, sub: 0, line: 244 } |  |  | 0.555 |
| ns | 9981 |  | 289 | Remaining packaging metadata and CI | 7.5 | 1.7 | 0.545 |
| walker |  | 9990 | 188 | Code::CodeKey { rung: Names, file: src/posting/variables.py, decl: 0, sub: 0, line: 0 } |  |  | 0.551 |

Score(3000)=0.797 I=0.932 C=0.682 ns_rows≤3K=19/49 grid(1000/1442/2080/3000/4327/6240/9000)=0.747/0.830/0.799/0.797/0.740/0.658/0.634

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 48 | 48 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 91 |  | 91 | README lede: what Posting is | 1.1 |  | 0.000 |
| walker |  | 139 | 91 | Markdown::ReadmeHeadline { file: README.md } |  |  | 1.000 |
| ns | 139 |  | 48 | Repository root listing | 1.2 |  | 1.000 |
| walker |  | 179 | 40 | Fs::DirListing { dir: docs } |  |  | 1.000 |
| walker |  | 183 | 4 | Fs::DirListing { dir: docs/overrides } |  |  | 1.000 |
| walker |  | 187 | 4 | Fs::DirListing { dir: docs/stylesheets } |  |  | 1.000 |
| ns | 243 |  | 104 | README feature list, part 1: in-app capabilities | 1.3 |  | 0.771 |
| walker |  | 262 | 75 | Toml::Identity { file: pyproject.toml } |  |  | 0.772 |
| walker |  | 270 | 8 | Fs::DirListing { dir: .github } |  |  | 0.772 |
| walker |  | 282 | 12 | Fs::DirListing { dir: .github/workflows } |  |  | 0.772 |
| walker |  | 306 | 24 | Toml::Operational { file: pyproject.toml } |  |  | 0.774 |
| ns | 343 |  | 100 | README feature list, part 2: interop and the command palette | 1.4 |  | 0.685 |
| ns | 501 |  | 158 | `src/posting/` module roster | 1.5 |  | 0.468 |
| walker |  | 525 | 219 | Plaintext::Whole { file: Makefile } |  |  | 0.468 |
| walker |  | 567 | 42 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.468 |
| ns | 604 |  | 103 | Widget package and importer package listings | 1.6 |  | 0.403 |
| walker |  | 623 | 56 | Fs::DirListing { dir: docs/guide } |  |  | 0.409 |
| ns | 750 |  | 146 | Package identity, build backend and console-script entry point | 1.7 |  | 0.401 |
| walker |  | 783 | 160 | Fs::DirListing { dir: src/posting } |  |  | 0.671 |
| walker |  | 797 | 14 | Fs::DirListing { dir: src/posting/importing } |  |  | 0.674 |
| walker |  | 813 | 16 | Code::CodeKey { rung: ModuleDoc, file: src/posting/__main__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.674 |
| walker |  | 902 | 89 | Fs::DirListing { dir: src/posting/widgets } |  |  | 0.805 |
| walker |  | 912 | 10 | Fs::DirListing { dir: src/posting/widgets/collection } |  |  | 0.805 |
| ns | 936 |  | 186 | Runtime dependency pins | 1.8 | 1.7 | 0.739 |
| walker |  | 942 | 30 | Fs::DirListing { dir: src/posting/widgets/response } |  |  | 0.740 |
| walker |  | 1010 | 68 | Fs::DirListing { dir: src/posting/widgets/request } |  |  | 0.751 |
| ns | 1044 |  | 108 | UI sub-package listings: request, response, collection | 1.9 |  | 0.771 |
| walker |  | 1096 | 86 | Fs::DirListing { dir: docs/assets } |  |  | 0.771 |
| ns | 1140 |  | 96 | Documentation tree listing | 1.10 |  | 0.785 |
| walker |  | 1146 | 50 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.785 |
| ns | 1270 |  | 130 | Test tree and sample-collection listings | 1.11 |  | 0.731 |
| walker |  | 1403 | 257 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.830 |
| ns | 1460 |  | 190 | Complete `posting` subcommand roster | 2.1 |  | 0.803 |
| walker |  | 1629 | 226 | Code::CodeKey { rung: Names, file: src/posting/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.803 |
| ns | 1689 |  | 229 | Every CLI argument and option | 2.2 |  | 0.763 |
| walker |  | 1728 | 99 | Fs::DirListing { dir: tests } |  |  | 0.803 |
| walker |  | 1759 | 31 | Fs::DirListing { dir: tests/sample-collections } |  |  | 0.829 |
| walker |  | 1764 | 5 | Fs::DirListing { dir: tests/sample-collections/scripts } |  |  | 0.829 |
| walker |  | 1901 | 137 | Code::CodeKey { rung: Names, file: src/posting/__main__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.838 |
| ns | 1907 |  | 218 | `make_posting`: CLI-to-app wiring | 2.3 |  | 0.809 |
| walker |  | 1913 | 12 | Code::CodeKey { rung: Decl, file: src/posting/__main__.py, decl: 7, sub: 0, line: 177 } |  |  | 0.811 |
| walker |  | 1935 | 22 | Code::CodeKey { rung: Decl, file: src/posting/__main__.py, decl: 3, sub: 0, line: 45 } |  |  | 0.816 |
| walker |  | 1967 | 32 | Code::CodeKey { rung: Decl, file: src/posting/__main__.py, decl: 5, sub: 0, line: 76 } |  |  | 0.816 |
| walker |  | 2011 | 44 | Code::CodeKey { rung: Decl, file: src/posting/__main__.py, decl: 8, sub: 0, line: 194 } |  |  | 0.819 |
| ns | 2052 |  | 145 | XDG locations: config file, themes and default collection | 2.4 |  | 0.799 |
| walker |  | 2140 | 129 | Code::CodeKey { rung: Decl, file: src/posting/__main__.py, decl: 4, sub: 0, line: 50 } |  |  | 0.816 |
| walker |  | 2272 | 132 | Code::CodeKey { rung: Decl, file: src/posting/__main__.py, decl: 6, sub: 0, line: 96 } |  |  | 0.849 |
| walker |  | 2284 | 12 | Code::CodeKey { rung: Doc, file: src/posting/__main__.py, decl: 3, sub: 0, line: 45 } |  |  | 0.852 |
| walker |  | 2296 | 12 | Code::CodeKey { rung: Doc, file: src/posting/__main__.py, decl: 7, sub: 0, line: 177 } |  |  | 0.856 |
| walker |  | 2312 | 16 | Code::CodeKey { rung: Doc, file: src/posting/__main__.py, decl: 6, sub: 0, line: 96 } |  |  | 0.862 |
| walker |  | 2329 | 17 | Code::CodeKey { rung: Doc, file: src/posting/__main__.py, decl: 8, sub: 0, line: 194 } |  |  | 0.864 |
| ns | 2359 |  | 307 | collection.py type roster | 3.1 |  | 0.821 |
| ns | 2534 |  | 175 | The `.posting.yaml` on-disk request format | 3.2 |  | 0.784 |
| walker |  | 2629 | 300 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.829 |
| ns | 2774 |  | 240 | RequestModel field roster | 3.3 | 3.1 | 0.801 |
| walker |  | 2856 | 227 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.801 |
| ns | 2934 |  | 160 | RequestModel behaviour roster | 3.4 |  | 0.789 |
| ns | 3145 |  | 211 | Key-value, option and script models | 3.5 | 3.1 | 0.760 |
| walker |  | 3164 | 308 | Code::CodeKey { rung: Names, file: src/posting/collection.py, decl: 0, sub: 0, line: 0 } |  |  | 0.805 |
| walker |  | 3174 | 10 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 10, sub: 0, line: 66 } |  |  | 0.805 |
| walker |  | 3190 | 16 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 11, sub: 0, line: 70 } |  |  | 0.805 |
| walker |  | 3211 | 21 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 31, sub: 0, line: 407 } |  |  | 0.805 |
| walker |  | 3232 | 21 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 32, sub: 0, line: 412 } |  |  | 0.805 |
| walker |  | 3254 | 22 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 8, sub: 0, line: 56 } |  |  | 0.805 |
| walker |  | 3276 | 22 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 9, sub: 0, line: 61 } |  |  | 0.805 |
| walker |  | 3305 | 29 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 12, sub: 0, line: 75 } |  |  | 0.807 |
| walker |  | 3334 | 29 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 13, sub: 0, line: 81 } |  |  | 0.807 |
| ns | 3349 |  | 204 | Collection tree: loading a directory, saving it back | 3.6 | 3.1 | 0.786 |
| walker |  | 3363 | 29 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 14, sub: 0, line: 87 } |  |  | 0.786 |
| walker |  | 3401 | 38 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 30, sub: 0, line: 401 } |  |  | 0.786 |
| walker |  | 3457 | 56 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 15, sub: 0, line: 93 } |  |  | 0.786 |
| walker |  | 3463 | 6 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 16, sub: 0, line: 98 } |  |  | 0.786 |
| walker |  | 3534 | 71 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 17, sub: 0, line: 103 } |  |  | 0.796 |
| ns | 3563 |  | 214 | config.py structure and settings-source configuration | 4.1 |  | 0.774 |
| walker |  | 3619 | 85 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 33, sub: 0, line: 417 } |  |  | 0.774 |
| ns | 3714 |  | 151 | Settings keys, part 1 | 4.2 |  | 0.759 |
| walker |  | 3744 | 125 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 21, sub: 0, line: 138 } |  |  | 0.768 |
| walker |  | 3875 | 131 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 18, sub: 0, line: 111 } |  |  | 0.777 |
| ns | 3892 |  | 178 | Settings keys, part 2 | 4.3 | 4.2 | 0.762 |
| walker |  | 4015 | 140 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 34, sub: 0, line: 427 } |  |  | 0.770 |
| walker |  | 4023 | 8 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 37, sub: 0, line: 480 } |  |  | 0.772 |
| walker |  | 4060 | 37 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 36, sub: 0, line: 441 } |  |  | 0.772 |
| walker |  | 4101 | 41 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 35, sub: 0, line: 434 } |  |  | 0.772 |
| ns | 4250 |  | 358 | Configuration precedence and guide section map | 4.4 |  | 0.737 |
| walker |  | 4268 | 167 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 3, sub: 0, line: 23 } |  |  | 0.737 |
| walker |  | 4276 | 8 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 5, sub: 0, line: 41 } |  |  | 0.737 |
| walker |  | 4284 | 8 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 6, sub: 0, line: 45 } |  |  | 0.737 |
| walker |  | 4292 | 8 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 7, sub: 0, line: 51 } |  |  | 0.737 |
| walker |  | 4505 | 213 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 22, sub: 0, line: 154 } |  |  | 0.745 |
| ns | 4526 |  | 276 | app.py class roster and MainScreen reactive state | 5.1 |  | 0.726 |
| walker |  | 4704 | 199 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 22, sub: 1, line: 154 } |  |  | 0.737 |
| ns | 4716 |  | 190 | Main screen keybindings, part 1 | 5.2 | 5.1 | 0.717 |
| ns | 4962 |  | 246 | Main screen keybindings, part 2 | 5.3 | 5.2 | 0.693 |
| walker |  | 5018 | 314 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 22, sub: 2, line: 154 } |  |  | 0.714 |
| walker |  | 5032 | 14 | Code::CodeKey { rung: Doc, file: src/posting/collection.py, decl: 23, sub: 0, line: 205 } |  |  | 0.717 |
| walker |  | 5047 | 15 | Code::CodeKey { rung: Doc, file: src/posting/collection.py, decl: 25, sub: 0, line: 291 } |  |  | 0.717 |
| walker |  | 5062 | 15 | Code::CodeKey { rung: Doc, file: src/posting/collection.py, decl: 38, sub: 0, line: 541 } |  |  | 0.717 |
| walker |  | 5078 | 16 | Code::CodeKey { rung: Doc, file: src/posting/collection.py, decl: 24, sub: 0, line: 269 } |  |  | 0.717 |
| walker |  | 5093 | 15 | Code::CodeKey { rung: Names, file: src/posting/_start_time.py, decl: 0, sub: 0, line: 0 } |  |  | 0.717 |
| ns | 5155 |  | 193 | Global app keybindings | 5.4 | 5.1 | 0.702 |
| walker |  | 5215 | 122 | Code::CodeKey { rung: Names, file: src/posting/scripts.py, decl: 0, sub: 0, line: 0 } |  |  | 0.702 |
| walker |  | 5244 | 29 | Code::CodeKey { rung: Decl, file: src/posting/scripts.py, decl: 13, sub: 0, line: 164 } |  |  | 0.702 |
| walker |  | 5277 | 33 | Code::CodeKey { rung: Decl, file: src/posting/scripts.py, decl: 12, sub: 0, line: 121 } |  |  | 0.702 |
| walker |  | 5409 | 132 | Code::CodeKey { rung: Decl, file: src/posting/scripts.py, decl: 3, sub: 0, line: 23 } |  |  | 0.702 |
| walker |  | 5417 | 8 | Code::CodeKey { rung: Decl, file: src/posting/scripts.py, decl: 5, sub: 0, line: 36 } |  |  | 0.702 |
| ns | 5444 |  | 289 | MainScreen method roster, part 1 | 5.5 |  | 0.685 |
| walker |  | 5481 | 64 | Code::CodeKey { rung: Decl, file: src/posting/scripts.py, decl: 10, sub: 0, line: 86 } |  |  | 0.685 |
| walker |  | 5492 | 11 | Code::CodeKey { rung: Doc, file: src/posting/scripts.py, decl: 9, sub: 0, line: 81 } |  |  | 0.685 |
| walker |  | 5510 | 18 | Code::CodeKey { rung: Doc, file: src/posting/scripts.py, decl: 3, sub: 0, line: 23 } |  |  | 0.685 |
| walker |  | 5529 | 19 | Code::CodeKey { rung: Doc, file: src/posting/scripts.py, decl: 13, sub: 0, line: 164 } |  |  | 0.685 |
| walker |  | 5536 | 7 | Fs::DirListing { dir: .codex/environments } |  |  | 0.685 |
| walker |  | 5546 | 10 | Code::CodeKey { rung: Names, file: src/posting/exit_codes.py, decl: 0, sub: 0, line: 0 } |  |  | 0.685 |
| walker |  | 5554 | 8 | Fs::DirListing { dir: tests/resources } |  |  | 0.685 |
| walker |  | 5563 | 9 | Code::CodeKey { rung: Body, file: src/posting/scripts.py, decl: 5, sub: 0, line: 36 } |  |  | 0.685 |
| walker |  | 5575 | 12 | Code::CodeKey { rung: Names, file: src/posting/help_data.py, decl: 0, sub: 0, line: 0 } |  |  | 0.685 |
| walker |  | 5634 | 59 | Code::CodeKey { rung: Decl, file: src/posting/help_data.py, decl: 1, sub: 0, line: 4 } |  |  | 0.685 |
| walker |  | 5646 | 12 | Code::CodeKey { rung: Names, file: src/posting/version.py, decl: 0, sub: 0, line: 0 } |  |  | 0.685 |
| ns | 5679 |  | 235 | MainScreen method roster, part 2 | 5.6 |  | 0.672 |
| walker |  | 5683 | 37 | Code::CodeKey { rung: Names, file: src/posting/jumper.py, decl: 0, sub: 0, line: 0 } |  |  | 0.672 |
| walker |  | 5702 | 19 | Code::CodeKey { rung: Decl, file: src/posting/jumper.py, decl: 1, sub: 0, line: 9 } |  |  | 0.672 |
| walker |  | 5753 | 51 | Code::CodeKey { rung: Decl, file: src/posting/jumper.py, decl: 3, sub: 0, line: 26 } |  |  | 0.672 |
| walker |  | 5809 | 56 | Code::CodeKey { rung: Decl, file: src/posting/jumper.py, decl: 2, sub: 0, line: 16 } |  |  | 0.672 |
| walker |  | 5822 | 13 | Code::CodeKey { rung: Doc, file: src/posting/jumper.py, decl: 1, sub: 0, line: 9 } |  |  | 0.672 |
| ns | 5823 |  | 144 | MainScreen.compose: the widget layout | 5.7 | 5.5 | 0.662 |
| walker |  | 5836 | 14 | Code::CodeKey { rung: Doc, file: src/posting/jumper.py, decl: 2, sub: 0, line: 16 } |  |  | 0.662 |
| walker |  | 5850 | 14 | Code::CodeKey { rung: Doc, file: src/posting/jumper.py, decl: 3, sub: 0, line: 26 } |  |  | 0.662 |
| walker |  | 5865 | 15 | Code::CodeKey { rung: Doc, file: src/posting/jumper.py, decl: 5, sub: 0, line: 34 } |  |  | 0.662 |
| walker |  | 5996 | 131 | Code::CodeKey { rung: Names, file: src/posting/config.py, decl: 0, sub: 0, line: 0 } |  |  | 0.671 |
| walker |  | 6030 | 34 | Code::CodeKey { rung: Decl, file: src/posting/config.py, decl: 6, sub: 0, line: 92 } |  |  | 0.671 |
| walker |  | 6071 | 41 | Code::CodeKey { rung: Decl, file: src/posting/config.py, decl: 7, sub: 0, line: 99 } |  |  | 0.671 |
| ns | 6072 |  | 249 | `Posting` App method roster | 5.8 |  | 0.658 |
| walker |  | 6145 | 74 | Code::CodeKey { rung: Decl, file: src/posting/config.py, decl: 3, sub: 0, line: 46 } |  |  | 0.658 |
| walker |  | 6219 | 74 | Code::CodeKey { rung: Decl, file: src/posting/config.py, decl: 8, sub: 0, line: 106 } |  |  | 0.658 |
| ns | 6311 |  | 239 | Command palette: every command | 6.1 |  | 0.648 |
| walker |  | 6340 | 121 | Code::CodeKey { rung: Decl, file: src/posting/config.py, decl: 5, sub: 0, line: 79 } |  |  | 0.648 |
| walker |  | 6480 | 140 | Code::CodeKey { rung: Decl, file: src/posting/config.py, decl: 2, sub: 0, line: 34 } |  |  | 0.648 |
| walker |  | 6649 | 169 | Code::CodeKey { rung: Decl, file: src/posting/config.py, decl: 1, sub: 0, line: 18 } |  |  | 0.648 |
| walker |  | 6659 | 10 | Code::CodeKey { rung: Doc, file: src/posting/config.py, decl: 3, sub: 0, line: 46 } |  |  | 0.648 |
| walker |  | 6669 | 10 | Code::CodeKey { rung: Doc, file: src/posting/config.py, decl: 6, sub: 0, line: 92 } |  |  | 0.648 |
| walker |  | 6679 | 10 | Code::CodeKey { rung: Doc, file: src/posting/config.py, decl: 7, sub: 0, line: 99 } |  |  | 0.648 |
| walker |  | 6689 | 10 | Code::CodeKey { rung: Doc, file: src/posting/config.py, decl: 8, sub: 0, line: 106 } |  |  | 0.648 |
| ns | 6694 |  | 383 | Request-editor widget class roster | 6.2 |  | 0.632 |
| walker |  | 6897 | 208 | Code::CodeKey { rung: Decl, file: src/posting/config.py, decl: 9, sub: 0, line: 116 } |  |  | 0.647 |
| ns | 6902 |  | 208 | Response and collection-browser widget class roster | 6.3 |  | 0.639 |
| walker |  | 7083 | 186 | Code::CodeKey { rung: Decl, file: src/posting/config.py, decl: 9, sub: 1, line: 116 } |  |  | 0.643 |
| ns | 7238 |  | 336 | Shared widget class roster | 6.4 |  | 0.631 |
| walker |  | 7274 | 191 | Code::CodeKey { rung: Decl, file: src/posting/config.py, decl: 9, sub: 2, line: 116 } |  |  | 0.641 |
| ns | 7440 |  | 202 | Scripting API: the `Posting` object handed to user scripts | 6.5 |  | 0.643 |
| walker |  | 7479 | 205 | Code::CodeKey { rung: Decl, file: src/posting/config.py, decl: 9, sub: 3, line: 116 } |  |  | 0.646 |
| walker |  | 7673 | 194 | Code::CodeKey { rung: Decl, file: src/posting/config.py, decl: 9, sub: 4, line: 116 } |  |  | 0.648 |
| ns | 7732 |  | 292 | Variables subsystem | 6.6 |  | 0.641 |
| walker |  | 7908 | 235 | Code::CodeKey { rung: Decl, file: src/posting/config.py, decl: 9, sub: 5, line: 116 } |  |  | 0.651 |
| walker |  | 8012 | 104 | Code::CodeKey { rung: Decl, file: src/posting/config.py, decl: 10, sub: 0, line: 218 } |  |  | 0.651 |
| walker |  | 8025 | 13 | Code::CodeKey { rung: Doc, file: src/posting/config.py, decl: 5, sub: 0, line: 79 } |  |  | 0.651 |
| ns | 8068 |  | 336 | Theme model | 6.7 |  | 0.639 |
| ns | 8260 |  | 192 | Every builtin theme name | 6.8 |  | 0.632 |
| walker |  | 8319 | 294 | Code::CodeKey { rung: Decl, file: src/posting/config.py, decl: 4, sub: 0, line: 56 } |  |  | 0.632 |
| walker |  | 8328 | 9 | Code::CodeKey { rung: Doc, file: src/posting/config.py, decl: 4, sub: 0, line: 56 } |  |  | 0.632 |
| walker |  | 8356 | 28 | Code::CodeKey { rung: Doc, file: src/posting/scripts.py, decl: 11, sub: 0, line: 113 } |  |  | 0.632 |
| ns | 8413 |  | 153 | Importer entry points | 6.9 |  | 0.628 |
| walker |  | 8422 | 66 | Code::CodeKey { rung: Names, file: src/posting/help_screen.py, decl: 0, sub: 0, line: 0 } |  |  | 0.628 |
| walker |  | 8441 | 19 | Code::CodeKey { rung: Decl, file: src/posting/help_screen.py, decl: 1, sub: 0, line: 14 } |  |  | 0.628 |
| walker |  | 8497 | 56 | Code::CodeKey { rung: Decl, file: src/posting/help_screen.py, decl: 2, sub: 0, line: 22 } |  |  | 0.628 |
| ns | 8530 |  | 117 | URL and path-parameter helpers | 6.10 |  | 0.626 |
| walker |  | 8553 | 56 | Code::CodeKey { rung: Decl, file: src/posting/help_screen.py, decl: 3, sub: 0, line: 33 } |  |  | 0.626 |
| walker |  | 8562 | 9 | Code::CodeKey { rung: Doc, file: src/posting/help_screen.py, decl: 2, sub: 0, line: 22 } |  |  | 0.626 |
| walker |  | 8571 | 9 | Code::CodeKey { rung: Doc, file: src/posting/help_screen.py, decl: 3, sub: 0, line: 33 } |  |  | 0.626 |
| walker |  | 8582 | 11 | Code::CodeKey { rung: Doc, file: src/posting/help_screen.py, decl: 4, sub: 0, line: 44 } |  |  | 0.626 |
| walker |  | 8599 | 17 | Code::CodeKey { rung: Doc, file: src/posting/help_data.py, decl: 1, sub: 0, line: 4 } |  |  | 0.626 |
| walker |  | 8613 | 14 | Code::CodeKey { rung: Names, file: src/posting/messages.py, decl: 0, sub: 0, line: 0 } |  |  | 0.626 |
| walker |  | 8628 | 15 | Code::CodeKey { rung: Decl, file: src/posting/messages.py, decl: 1, sub: 0, line: 6 } |  |  | 0.626 |
| ns | 8694 |  | 164 | HTTP header catalogue | 6.11 |  | 0.618 |
| walker |  | 8713 | 85 | Code::CodeKey { rung: Names, file: src/posting/locations.py, decl: 0, sub: 0, line: 0 } |  |  | 0.620 |
| walker |  | 8728 | 15 | Code::CodeKey { rung: Doc, file: src/posting/locations.py, decl: 3, sub: 0, line: 17 } |  |  | 0.620 |
| walker |  | 8740 | 12 | Code::CodeKey { rung: Body, file: src/posting/locations.py, decl: 6, sub: 0, line: 34 } |  |  | 0.621 |
| walker |  | 8756 | 16 | Code::CodeKey { rung: Doc, file: src/posting/locations.py, decl: 2, sub: 0, line: 12 } |  |  | 0.623 |
| walker |  | 8772 | 16 | Code::CodeKey { rung: Doc, file: src/posting/locations.py, decl: 4, sub: 0, line: 24 } |  |  | 0.625 |
| walker |  | 8788 | 16 | Code::CodeKey { rung: Doc, file: src/posting/locations.py, decl: 5, sub: 0, line: 29 } |  |  | 0.627 |
| walker |  | 8799 | 11 | Code::CodeKey { rung: Body, file: src/posting/locations.py, decl: 4, sub: 0, line: 24 } |  |  | 0.627 |
| walker |  | 8809 | 10 | Fs::DirListing { dir: tests/sample-envs } |  |  | 0.627 |
| walker |  | 8866 | 57 | Code::CodeKey { rung: Names, file: src/posting/app.py, decl: 0, sub: 0, line: 0 } |  |  | 0.628 |
| walker |  | 8881 | 15 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 1, sub: 0, line: 93 } |  |  | 0.628 |
| walker |  | 8891 | 10 | Code::CodeKey { rung: Doc, file: src/posting/app.py, decl: 1, sub: 0, line: 93 } |  |  | 0.628 |
| walker |  | 8901 | 10 | Code::CodeKey { rung: Doc, file: src/posting/app.py, decl: 3, sub: 0, line: 108 } |  |  | 0.629 |
| ns | 8989 |  | 295 | Test invocation: the Makefile is mandatory | 7.1 |  | 0.630 |
| ns | 9139 |  | 150 | Development environment and snapshot testing | 7.2 |  | 0.627 |
| walker |  | 9247 | 346 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 66, sub: 0, line: 1205 } |  |  | 0.650 |
| ns | 9407 |  | 268 | Scripting and core workflow guide section maps | 7.3 |  | 0.639 |
| walker |  | 9440 | 193 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 66, sub: 1, line: 1205 } |  |  | 0.642 |
| walker |  | 9458 | 18 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 70, sub: 0, line: 1302 } |  |  | 0.642 |
| walker |  | 9476 | 18 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 71, sub: 0, line: 1330 } |  |  | 0.642 |
| walker |  | 9494 | 18 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 72, sub: 0, line: 1361 } |  |  | 0.642 |
| walker |  | 9561 | 67 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 67, sub: 0, line: 1239 } |  |  | 0.642 |
| ns | 9692 |  | 285 | Remaining guide section maps | 7.4 |  | 0.632 |
| walker |  | 9750 | 189 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 66, sub: 2, line: 1205 } |  |  | 0.646 |
| walker |  | 9763 | 13 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 81, sub: 0, line: 1581 } |  |  | 0.646 |
| walker |  | 9776 | 13 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 83, sub: 0, line: 1610 } |  |  | 0.646 |
| walker |  | 9794 | 18 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 80, sub: 0, line: 1576 } |  |  | 0.646 |
| walker |  | 9832 | 38 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 82, sub: 0, line: 1591 } |  |  | 0.646 |
| walker |  | 9893 | 61 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 84, sub: 0, line: 1620 } |  |  | 0.646 |
| walker |  | 9955 | 62 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 86, sub: 0, line: 1655 } |  |  | 0.646 |
| walker |  | 9969 | 14 | Code::CodeKey { rung: Doc, file: src/posting/app.py, decl: 72, sub: 0, line: 1361 } |  |  | 0.646 |
| ns | 9981 |  | 289 | Remaining packaging metadata and CI | 7.5 | 1.7 | 0.634 |
| walker |  | 9995 | 26 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 4, sub: 0, line: 112 } |  |  | 0.636 |

Score(3000)=0.796 I=0.932 C=0.680 ns_rows≤3K=19/49 grid(1000/1442/2080/3000/4327/6240/9000)=0.746/0.830/0.804/0.796/0.737/0.622/0.593

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
| walker |  | 916 | 10 | Fs::DirListing { dir: src/posting/widgets/collection } |  |  | 0.805 |
| ns | 936 |  | 186 | Runtime dependency pins | 1.8 | 1.7 | 0.739 |
| walker |  | 946 | 30 | Fs::DirListing { dir: src/posting/widgets/response } |  |  | 0.740 |
| walker |  | 1014 | 68 | Fs::DirListing { dir: src/posting/widgets/request } |  |  | 0.751 |
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
| walker |  | 1763 | 31 | Fs::DirListing { dir: tests/sample-collections } |  |  | 0.829 |
| walker |  | 1768 | 5 | Fs::DirListing { dir: tests/sample-collections/scripts } |  |  | 0.829 |
| walker |  | 1905 | 137 | Code::CodeKey { rung: Names, file: src/posting/__main__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.838 |
| ns | 1907 |  | 218 | `make_posting`: CLI-to-app wiring | 2.3 |  | 0.809 |
| walker |  | 1917 | 12 | Code::CodeKey { rung: Decl, file: src/posting/__main__.py, decl: 7, sub: 0, line: 177 } |  |  | 0.811 |
| walker |  | 1939 | 22 | Code::CodeKey { rung: Decl, file: src/posting/__main__.py, decl: 3, sub: 0, line: 45 } |  |  | 0.816 |
| walker |  | 1971 | 32 | Code::CodeKey { rung: Decl, file: src/posting/__main__.py, decl: 5, sub: 0, line: 76 } |  |  | 0.816 |
| walker |  | 2015 | 44 | Code::CodeKey { rung: Decl, file: src/posting/__main__.py, decl: 8, sub: 0, line: 194 } |  |  | 0.819 |
| ns | 2052 |  | 145 | XDG locations: config file, themes and default collection | 2.4 |  | 0.799 |
| walker |  | 2144 | 129 | Code::CodeKey { rung: Decl, file: src/posting/__main__.py, decl: 4, sub: 0, line: 50 } |  |  | 0.816 |
| walker |  | 2276 | 132 | Code::CodeKey { rung: Decl, file: src/posting/__main__.py, decl: 6, sub: 0, line: 96 } |  |  | 0.849 |
| walker |  | 2288 | 12 | Code::CodeKey { rung: Doc, file: src/posting/__main__.py, decl: 3, sub: 0, line: 45 } |  |  | 0.852 |
| walker |  | 2300 | 12 | Code::CodeKey { rung: Doc, file: src/posting/__main__.py, decl: 7, sub: 0, line: 177 } |  |  | 0.856 |
| walker |  | 2316 | 16 | Code::CodeKey { rung: Doc, file: src/posting/__main__.py, decl: 6, sub: 0, line: 96 } |  |  | 0.862 |
| walker |  | 2333 | 17 | Code::CodeKey { rung: Doc, file: src/posting/__main__.py, decl: 8, sub: 0, line: 194 } |  |  | 0.864 |
| ns | 2359 |  | 307 | collection.py type roster | 3.1 |  | 0.821 |
| ns | 2534 |  | 175 | The `.posting.yaml` on-disk request format | 3.2 |  | 0.784 |
| walker |  | 2633 | 300 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.829 |
| walker |  | 2638 | 5 | Fs::DirListing { dir: .codex/environments } |  |  | 0.829 |
| walker |  | 2643 | 5 | Fs::DirListing { dir: tests/__snapshots__ } |  |  | 0.829 |
| ns | 2774 |  | 240 | RequestModel field roster | 3.3 | 3.1 | 0.801 |
| walker |  | 2870 | 227 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.801 |
| ns | 2934 |  | 160 | RequestModel behaviour roster | 3.4 |  | 0.789 |
| ns | 3145 |  | 211 | Key-value, option and script models | 3.5 | 3.1 | 0.760 |
| walker |  | 3178 | 308 | Code::CodeKey { rung: Names, file: src/posting/collection.py, decl: 0, sub: 0, line: 0 } |  |  | 0.805 |
| walker |  | 3188 | 10 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 10, sub: 0, line: 66 } |  |  | 0.805 |
| walker |  | 3204 | 16 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 11, sub: 0, line: 70 } |  |  | 0.805 |
| walker |  | 3225 | 21 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 31, sub: 0, line: 407 } |  |  | 0.805 |
| walker |  | 3246 | 21 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 32, sub: 0, line: 412 } |  |  | 0.805 |
| walker |  | 3268 | 22 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 8, sub: 0, line: 56 } |  |  | 0.805 |
| walker |  | 3290 | 22 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 9, sub: 0, line: 61 } |  |  | 0.805 |
| walker |  | 3319 | 29 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 12, sub: 0, line: 75 } |  |  | 0.807 |
| walker |  | 3348 | 29 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 13, sub: 0, line: 81 } |  |  | 0.807 |
| ns | 3349 |  | 204 | Collection tree: loading a directory, saving it back | 3.6 | 3.1 | 0.786 |
| walker |  | 3377 | 29 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 14, sub: 0, line: 87 } |  |  | 0.786 |
| walker |  | 3415 | 38 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 30, sub: 0, line: 401 } |  |  | 0.786 |
| walker |  | 3471 | 56 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 15, sub: 0, line: 93 } |  |  | 0.786 |
| walker |  | 3477 | 6 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 16, sub: 0, line: 98 } |  |  | 0.786 |
| walker |  | 3548 | 71 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 17, sub: 0, line: 103 } |  |  | 0.796 |
| ns | 3563 |  | 214 | config.py structure and settings-source configuration | 4.1 |  | 0.774 |
| walker |  | 3633 | 85 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 33, sub: 0, line: 417 } |  |  | 0.774 |
| ns | 3714 |  | 151 | Settings keys, part 1 | 4.2 |  | 0.759 |
| walker |  | 3758 | 125 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 21, sub: 0, line: 138 } |  |  | 0.768 |
| walker |  | 3889 | 131 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 18, sub: 0, line: 111 } |  |  | 0.777 |
| ns | 3892 |  | 178 | Settings keys, part 2 | 4.3 | 4.2 | 0.762 |
| walker |  | 4029 | 140 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 34, sub: 0, line: 427 } |  |  | 0.770 |
| walker |  | 4037 | 8 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 37, sub: 0, line: 480 } |  |  | 0.772 |
| walker |  | 4074 | 37 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 36, sub: 0, line: 441 } |  |  | 0.772 |
| walker |  | 4115 | 41 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 35, sub: 0, line: 434 } |  |  | 0.772 |
| ns | 4250 |  | 358 | Configuration precedence and guide section map | 4.4 |  | 0.737 |
| walker |  | 4282 | 167 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 3, sub: 0, line: 23 } |  |  | 0.737 |
| walker |  | 4290 | 8 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 5, sub: 0, line: 41 } |  |  | 0.737 |
| walker |  | 4298 | 8 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 6, sub: 0, line: 45 } |  |  | 0.737 |
| walker |  | 4306 | 8 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 7, sub: 0, line: 51 } |  |  | 0.737 |
| walker |  | 4321 | 15 | Code::CodeKey { rung: Doc, file: src/posting/collection.py, decl: 38, sub: 0, line: 541 } |  |  | 0.737 |
| walker |  | 4336 | 15 | Code::CodeKey { rung: Names, file: src/posting/_start_time.py, decl: 0, sub: 0, line: 0 } |  |  | 0.737 |
| walker |  | 4458 | 122 | Code::CodeKey { rung: Names, file: src/posting/scripts.py, decl: 0, sub: 0, line: 0 } |  |  | 0.737 |
| walker |  | 4487 | 29 | Code::CodeKey { rung: Decl, file: src/posting/scripts.py, decl: 13, sub: 0, line: 164 } |  |  | 0.737 |
| walker |  | 4520 | 33 | Code::CodeKey { rung: Decl, file: src/posting/scripts.py, decl: 12, sub: 0, line: 121 } |  |  | 0.737 |
| ns | 4526 |  | 276 | app.py class roster and MainScreen reactive state | 5.1 |  | 0.719 |
| walker |  | 4652 | 132 | Code::CodeKey { rung: Decl, file: src/posting/scripts.py, decl: 3, sub: 0, line: 23 } |  |  | 0.720 |
| walker |  | 4660 | 8 | Code::CodeKey { rung: Decl, file: src/posting/scripts.py, decl: 5, sub: 0, line: 36 } |  |  | 0.720 |
| ns | 4716 |  | 190 | Main screen keybindings, part 1 | 5.2 | 5.1 | 0.700 |
| walker |  | 4724 | 64 | Code::CodeKey { rung: Decl, file: src/posting/scripts.py, decl: 10, sub: 0, line: 86 } |  |  | 0.700 |
| walker |  | 4735 | 11 | Code::CodeKey { rung: Doc, file: src/posting/scripts.py, decl: 9, sub: 0, line: 81 } |  |  | 0.700 |
| walker |  | 4753 | 18 | Code::CodeKey { rung: Doc, file: src/posting/scripts.py, decl: 3, sub: 0, line: 23 } |  |  | 0.700 |
| walker |  | 4772 | 19 | Code::CodeKey { rung: Doc, file: src/posting/scripts.py, decl: 13, sub: 0, line: 164 } |  |  | 0.700 |
| walker |  | 4780 | 8 | Fs::DirListing { dir: tests/resources } |  |  | 0.700 |
| walker |  | 4790 | 10 | Code::CodeKey { rung: Names, file: src/posting/exit_codes.py, decl: 0, sub: 0, line: 0 } |  |  | 0.700 |
| walker |  | 4802 | 12 | Code::CodeKey { rung: Names, file: src/posting/help_data.py, decl: 0, sub: 0, line: 0 } |  |  | 0.700 |
| walker |  | 4861 | 59 | Code::CodeKey { rung: Decl, file: src/posting/help_data.py, decl: 1, sub: 0, line: 4 } |  |  | 0.700 |
| walker |  | 4873 | 12 | Code::CodeKey { rung: Names, file: src/posting/version.py, decl: 0, sub: 0, line: 0 } |  |  | 0.700 |
| walker |  | 4883 | 10 | Fs::DirListing { dir: tests/sample-envs } |  |  | 0.700 |
| walker |  | 4920 | 37 | Code::CodeKey { rung: Names, file: src/posting/jumper.py, decl: 0, sub: 0, line: 0 } |  |  | 0.700 |
| walker |  | 4939 | 19 | Code::CodeKey { rung: Decl, file: src/posting/jumper.py, decl: 1, sub: 0, line: 9 } |  |  | 0.700 |
| ns | 4962 |  | 246 | Main screen keybindings, part 2 | 5.3 | 5.2 | 0.677 |
| walker |  | 4990 | 51 | Code::CodeKey { rung: Decl, file: src/posting/jumper.py, decl: 3, sub: 0, line: 26 } |  |  | 0.677 |
| walker |  | 5046 | 56 | Code::CodeKey { rung: Decl, file: src/posting/jumper.py, decl: 2, sub: 0, line: 16 } |  |  | 0.677 |
| walker |  | 5059 | 13 | Code::CodeKey { rung: Doc, file: src/posting/jumper.py, decl: 1, sub: 0, line: 9 } |  |  | 0.677 |
| walker |  | 5073 | 14 | Code::CodeKey { rung: Doc, file: src/posting/jumper.py, decl: 2, sub: 0, line: 16 } |  |  | 0.677 |
| walker |  | 5087 | 14 | Code::CodeKey { rung: Doc, file: src/posting/jumper.py, decl: 3, sub: 0, line: 26 } |  |  | 0.677 |
| walker |  | 5102 | 15 | Code::CodeKey { rung: Doc, file: src/posting/jumper.py, decl: 5, sub: 0, line: 34 } |  |  | 0.677 |
| ns | 5155 |  | 193 | Global app keybindings | 5.4 | 5.1 | 0.663 |
| walker |  | 5233 | 131 | Code::CodeKey { rung: Names, file: src/posting/config.py, decl: 0, sub: 0, line: 0 } |  |  | 0.672 |
| walker |  | 5267 | 34 | Code::CodeKey { rung: Decl, file: src/posting/config.py, decl: 6, sub: 0, line: 92 } |  |  | 0.672 |
| walker |  | 5308 | 41 | Code::CodeKey { rung: Decl, file: src/posting/config.py, decl: 7, sub: 0, line: 99 } |  |  | 0.672 |
| walker |  | 5382 | 74 | Code::CodeKey { rung: Decl, file: src/posting/config.py, decl: 3, sub: 0, line: 46 } |  |  | 0.672 |
| ns | 5444 |  | 289 | MainScreen method roster, part 1 | 5.5 |  | 0.655 |
| walker |  | 5456 | 74 | Code::CodeKey { rung: Decl, file: src/posting/config.py, decl: 8, sub: 0, line: 106 } |  |  | 0.655 |
| walker |  | 5577 | 121 | Code::CodeKey { rung: Decl, file: src/posting/config.py, decl: 5, sub: 0, line: 79 } |  |  | 0.655 |
| ns | 5679 |  | 235 | MainScreen method roster, part 2 | 5.6 |  | 0.643 |
| walker |  | 5717 | 140 | Code::CodeKey { rung: Decl, file: src/posting/config.py, decl: 2, sub: 0, line: 34 } |  |  | 0.643 |
| ns | 5823 |  | 144 | MainScreen.compose: the widget layout | 5.7 | 5.5 | 0.634 |
| walker |  | 5886 | 169 | Code::CodeKey { rung: Decl, file: src/posting/config.py, decl: 1, sub: 0, line: 18 } |  |  | 0.634 |
| walker |  | 5896 | 10 | Code::CodeKey { rung: Doc, file: src/posting/config.py, decl: 3, sub: 0, line: 46 } |  |  | 0.634 |
| walker |  | 5906 | 10 | Code::CodeKey { rung: Doc, file: src/posting/config.py, decl: 6, sub: 0, line: 92 } |  |  | 0.634 |
| walker |  | 5916 | 10 | Code::CodeKey { rung: Doc, file: src/posting/config.py, decl: 7, sub: 0, line: 99 } |  |  | 0.634 |
| walker |  | 5926 | 10 | Code::CodeKey { rung: Doc, file: src/posting/config.py, decl: 8, sub: 0, line: 106 } |  |  | 0.634 |
| walker |  | 5939 | 13 | Code::CodeKey { rung: Doc, file: src/posting/config.py, decl: 5, sub: 0, line: 79 } |  |  | 0.634 |
| ns | 6072 |  | 249 | `Posting` App method roster | 5.8 |  | 0.622 |
| walker |  | 6233 | 294 | Code::CodeKey { rung: Decl, file: src/posting/config.py, decl: 4, sub: 0, line: 56 } |  |  | 0.622 |
| walker |  | 6242 | 9 | Code::CodeKey { rung: Doc, file: src/posting/config.py, decl: 4, sub: 0, line: 56 } |  |  | 0.622 |
| walker |  | 6270 | 28 | Code::CodeKey { rung: Doc, file: src/posting/scripts.py, decl: 11, sub: 0, line: 113 } |  |  | 0.622 |
| ns | 6311 |  | 239 | Command palette: every command | 6.1 |  | 0.612 |
| walker |  | 6336 | 66 | Code::CodeKey { rung: Names, file: src/posting/help_screen.py, decl: 0, sub: 0, line: 0 } |  |  | 0.612 |
| walker |  | 6355 | 19 | Code::CodeKey { rung: Decl, file: src/posting/help_screen.py, decl: 1, sub: 0, line: 14 } |  |  | 0.612 |
| walker |  | 6411 | 56 | Code::CodeKey { rung: Decl, file: src/posting/help_screen.py, decl: 2, sub: 0, line: 22 } |  |  | 0.612 |
| walker |  | 6467 | 56 | Code::CodeKey { rung: Decl, file: src/posting/help_screen.py, decl: 3, sub: 0, line: 33 } |  |  | 0.612 |
| walker |  | 6476 | 9 | Code::CodeKey { rung: Doc, file: src/posting/help_screen.py, decl: 2, sub: 0, line: 22 } |  |  | 0.612 |
| walker |  | 6485 | 9 | Code::CodeKey { rung: Doc, file: src/posting/help_screen.py, decl: 3, sub: 0, line: 33 } |  |  | 0.612 |
| walker |  | 6496 | 11 | Code::CodeKey { rung: Doc, file: src/posting/help_screen.py, decl: 4, sub: 0, line: 44 } |  |  | 0.612 |
| walker |  | 6507 | 11 | Fs::DirListing { dir: tests/sample-themes } |  |  | 0.612 |
| walker |  | 6524 | 17 | Code::CodeKey { rung: Doc, file: src/posting/help_data.py, decl: 1, sub: 0, line: 4 } |  |  | 0.612 |
| walker |  | 6538 | 14 | Code::CodeKey { rung: Names, file: src/posting/messages.py, decl: 0, sub: 0, line: 0 } |  |  | 0.612 |
| walker |  | 6553 | 15 | Code::CodeKey { rung: Decl, file: src/posting/messages.py, decl: 1, sub: 0, line: 6 } |  |  | 0.612 |
| walker |  | 6638 | 85 | Code::CodeKey { rung: Names, file: src/posting/locations.py, decl: 0, sub: 0, line: 0 } |  |  | 0.614 |
| walker |  | 6650 | 12 | Code::CodeKey { rung: Body, file: src/posting/locations.py, decl: 6, sub: 0, line: 34 } |  |  | 0.616 |
| walker |  | 6663 | 13 | Code::CodeKey { rung: Body, file: src/posting/locations.py, decl: 4, sub: 0, line: 24 } |  |  | 0.616 |
| walker |  | 6677 | 14 | Code::CodeKey { rung: Doc, file: src/posting/locations.py, decl: 4, sub: 0, line: 24 } |  |  | 0.618 |
| walker |  | 6691 | 14 | Code::CodeKey { rung: Body, file: src/posting/locations.py, decl: 2, sub: 0, line: 12 } |  |  | 0.618 |
| ns | 6694 |  | 383 | Request-editor widget class roster | 6.2 |  | 0.603 |
| walker |  | 6705 | 14 | Code::CodeKey { rung: Doc, file: src/posting/locations.py, decl: 2, sub: 0, line: 12 } |  |  | 0.605 |
| walker |  | 6720 | 15 | Code::CodeKey { rung: Doc, file: src/posting/locations.py, decl: 3, sub: 0, line: 17 } |  |  | 0.605 |
| walker |  | 6736 | 16 | Code::CodeKey { rung: Doc, file: src/posting/locations.py, decl: 5, sub: 0, line: 29 } |  |  | 0.608 |
| walker |  | 6793 | 57 | Code::CodeKey { rung: Names, file: src/posting/app.py, decl: 0, sub: 0, line: 0 } |  |  | 0.609 |
| walker |  | 6808 | 15 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 1, sub: 0, line: 93 } |  |  | 0.609 |
| walker |  | 6818 | 10 | Code::CodeKey { rung: Doc, file: src/posting/app.py, decl: 1, sub: 0, line: 93 } |  |  | 0.609 |
| walker |  | 6828 | 10 | Code::CodeKey { rung: Doc, file: src/posting/app.py, decl: 3, sub: 0, line: 108 } |  |  | 0.610 |
| ns | 6902 |  | 208 | Response and collection-browser widget class roster | 6.3 |  | 0.602 |
| walker |  | 7041 | 213 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 22, sub: 0, line: 154 } |  |  | 0.608 |
| walker |  | 7070 | 29 | Code::CodeKey { rung: Names, file: src/posting/types.py, decl: 0, sub: 0, line: 0 } |  |  | 0.608 |
| walker |  | 7142 | 72 | Code::CodeKey { rung: Decl, file: src/posting/types.py, decl: 2, sub: 0, line: 6 } |  |  | 0.608 |
| ns | 7238 |  | 336 | Shared widget class roster | 6.4 |  | 0.597 |
| walker |  | 7341 | 199 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 22, sub: 1, line: 154 } |  |  | 0.606 |
| walker |  | 7372 | 31 | Code::CodeKey { rung: Names, file: src/posting/request_headers.py, decl: 0, sub: 0, line: 0 } |  |  | 0.606 |
| walker |  | 7415 | 43 | Code::CodeKey { rung: Decl, file: src/posting/request_headers.py, decl: 1, sub: 0, line: 4 } |  |  | 0.606 |
| ns | 7440 |  | 202 | Scripting API: the `Posting` object handed to user scripts | 6.5 |  | 0.608 |
| walker |  | 7729 | 314 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 22, sub: 2, line: 154 } |  |  | 0.625 |
| ns | 7732 |  | 292 | Variables subsystem | 6.6 |  | 0.618 |
| walker |  | 7743 | 14 | Code::CodeKey { rung: Doc, file: src/posting/collection.py, decl: 23, sub: 0, line: 205 } |  |  | 0.620 |
| walker |  | 7758 | 15 | Code::CodeKey { rung: Doc, file: src/posting/collection.py, decl: 25, sub: 0, line: 291 } |  |  | 0.620 |
| walker |  | 7774 | 16 | Code::CodeKey { rung: Doc, file: src/posting/collection.py, decl: 24, sub: 0, line: 269 } |  |  | 0.620 |
| ns | 8068 |  | 336 | Theme model | 6.7 |  | 0.609 |
| walker |  | 8096 | 322 | Code::CodeKey { rung: Names, file: src/posting/themes.py, decl: 0, sub: 0, line: 0 } |  |  | 0.609 |
| walker |  | 8184 | 88 | Code::CodeKey { rung: Decl, file: src/posting/themes.py, decl: 11, sub: 0, line: 305 } |  |  | 0.609 |
| ns | 8260 |  | 192 | Every builtin theme name | 6.8 |  | 0.603 |
| walker |  | 8275 | 91 | Code::CodeKey { rung: Decl, file: src/posting/themes.py, decl: 3, sub: 0, line: 54 } |  |  | 0.603 |
| walker |  | 8410 | 135 | Code::CodeKey { rung: Decl, file: src/posting/themes.py, decl: 5, sub: 0, line: 72 } |  |  | 0.603 |
| ns | 8413 |  | 153 | Importer entry points | 6.9 |  | 0.599 |
| ns | 8530 |  | 117 | URL and path-parameter helpers | 6.10 |  | 0.597 |
| walker |  | 8545 | 135 | Code::CodeKey { rung: Decl, file: src/posting/themes.py, decl: 7, sub: 0, line: 94 } |  |  | 0.597 |
| ns | 8694 |  | 164 | HTTP header catalogue | 6.11 |  | 0.592 |
| walker |  | 8718 | 173 | Code::CodeKey { rung: Decl, file: src/posting/themes.py, decl: 2, sub: 0, line: 34 } |  |  | 0.592 |
| walker |  | 8933 | 215 | Code::CodeKey { rung: Decl, file: src/posting/themes.py, decl: 1, sub: 0, line: 14 } |  |  | 0.592 |
| walker |  | 8944 | 11 | Code::CodeKey { rung: Doc, file: src/posting/themes.py, decl: 3, sub: 0, line: 54 } |  |  | 0.592 |
| walker |  | 8957 | 13 | Code::CodeKey { rung: Doc, file: src/posting/themes.py, decl: 5, sub: 0, line: 72 } |  |  | 0.592 |
| walker |  | 8972 | 15 | Code::CodeKey { rung: Doc, file: src/posting/themes.py, decl: 7, sub: 0, line: 94 } |  |  | 0.592 |
| ns | 8989 |  | 295 | Test invocation: the Makefile is mandatory | 7.1 |  | 0.593 |
| walker |  | 9008 | 36 | Code::CodeKey { rung: Doc, file: src/posting/collection.py, decl: 21, sub: 0, line: 138 } |  |  | 0.593 |
| walker |  | 9025 | 17 | Code::CodeKey { rung: Names, file: src/posting/suggesters.py, decl: 0, sub: 0, line: 0 } |  |  | 0.593 |
| walker |  | 9135 | 110 | Code::CodeKey { rung: Decl, file: src/posting/suggesters.py, decl: 1, sub: 0, line: 2 } |  |  | 0.593 |
| ns | 9139 |  | 150 | Development environment and snapshot testing | 7.2 |  | 0.591 |
| walker |  | 9152 | 17 | Code::CodeKey { rung: Names, file: src/posting/user_host.py, decl: 0, sub: 0, line: 0 } |  |  | 0.591 |
| walker |  | 9186 | 34 | Code::CodeKey { rung: Names, file: src/posting/xresources.py, decl: 0, sub: 0, line: 0 } |  |  | 0.591 |
| walker |  | 9287 | 101 | Code::CodeKey { rung: Decl, file: src/posting/xresources.py, decl: 1, sub: 0, line: 8 } |  |  | 0.591 |
| walker |  | 9305 | 18 | Code::CodeKey { rung: Names, file: src/posting/auth.py, decl: 0, sub: 0, line: 0 } |  |  | 0.591 |
| walker |  | 9351 | 46 | Code::CodeKey { rung: Decl, file: src/posting/auth.py, decl: 1, sub: 0, line: 6 } |  |  | 0.591 |
| walker |  | 9359 | 8 | Code::CodeKey { rung: Body, file: src/posting/auth.py, decl: 2, sub: 0, line: 7 } |  |  | 0.591 |
| walker |  | 9382 | 23 | Code::CodeKey { rung: Doc, file: src/posting/themes.py, decl: 2, sub: 0, line: 34 } |  |  | 0.591 |
| walker |  | 9405 | 23 | Code::CodeKey { rung: Doc, file: src/posting/xresources.py, decl: 2, sub: 0, line: 20 } |  |  | 0.591 |
| ns | 9407 |  | 268 | Scripting and core workflow guide section maps | 7.3 |  | 0.581 |
| walker |  | 9668 | 263 | Code::CodeKey { rung: Names, file: src/posting/variables.py, decl: 0, sub: 0, line: 0 } |  |  | 0.593 |
| walker |  | 9677 | 9 | Code::CodeKey { rung: Decl, file: src/posting/variables.py, decl: 11, sub: 0, line: 85 } |  |  | 0.593 |
| walker |  | 9686 | 9 | Code::CodeKey { rung: Decl, file: src/posting/variables.py, decl: 12, sub: 0, line: 94 } |  |  | 0.593 |
| ns | 9692 |  | 285 | Remaining guide section maps | 7.4 |  | 0.583 |
| walker |  | 9695 | 9 | Code::CodeKey { rung: Decl, file: src/posting/variables.py, decl: 17, sub: 0, line: 133 } |  |  | 0.583 |
| walker |  | 9743 | 48 | Code::CodeKey { rung: Decl, file: src/posting/variables.py, decl: 9, sub: 0, line: 36 } |  |  | 0.583 |
| walker |  | 9797 | 54 | Code::CodeKey { rung: Decl, file: src/posting/variables.py, decl: 1, sub: 0, line: 10 } |  |  | 0.585 |
| walker |  | 9868 | 71 | Code::CodeKey { rung: Decl, file: src/posting/variables.py, decl: 2, sub: 0, line: 15 } |  |  | 0.585 |
| walker |  | 9884 | 16 | Code::CodeKey { rung: Doc, file: src/posting/variables.py, decl: 18, sub: 0, line: 157 } |  |  | 0.585 |
| walker |  | 9908 | 24 | Code::CodeKey { rung: Body, file: src/posting/auth.py, decl: 3, sub: 0, line: 10 } |  |  | 0.585 |
| walker |  | 9965 | 57 | Code::CodeKey { rung: Names, file: src/posting/save_request.py, decl: 0, sub: 0, line: 0 } |  |  | 0.585 |
| walker |  | 9976 | 11 | Code::CodeKey { rung: Doc, file: src/posting/save_request.py, decl: 2, sub: 0, line: 8 } |  |  | 0.585 |
| ns | 9981 |  | 289 | Remaining packaging metadata and CI | 7.5 | 1.7 | 0.575 |
| walker |  | 9987 | 11 | Code::CodeKey { rung: Body, file: src/posting/save_request.py, decl: 3, sub: 0, line: 15 } |  |  | 0.575 |

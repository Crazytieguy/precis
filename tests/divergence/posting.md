Score(3000)=0.789 I=0.930 C=0.670 ns_rows≤3K=19/49 grid(1000/1442/2080/3000/4327/6240/9000)=0.747/0.830/0.800/0.789/0.741/0.703/0.709

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
| ns | 1907 |  | 218 | `make_posting`: CLI-to-app wiring | 2.3 |  | 0.800 |
| walker |  | 1997 | 233 | Code::CodeKey { rung: Names, file: src/posting/__main__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.816 |
| walker |  | 2019 | 22 | Code::CodeKey { rung: Decl, file: src/posting/__main__.py, decl: 5, sub: 0, line: 76 } |  |  | 0.818 |
| ns | 2052 |  | 145 | XDG locations: config file, themes and default collection | 2.4 |  | 0.798 |
| walker |  | 2053 | 34 | Code::CodeKey { rung: Decl, file: src/posting/__main__.py, decl: 8, sub: 0, line: 194 } |  |  | 0.800 |
| walker |  | 2153 | 100 | Code::CodeKey { rung: Decl, file: src/posting/__main__.py, decl: 6, sub: 0, line: 96 } |  |  | 0.812 |
| walker |  | 2272 | 119 | Code::CodeKey { rung: Decl, file: src/posting/__main__.py, decl: 4, sub: 0, line: 50 } |  |  | 0.849 |
| walker |  | 2284 | 12 | Code::CodeKey { rung: Doc, file: src/posting/__main__.py, decl: 3, sub: 0, line: 45 } |  |  | 0.852 |
| walker |  | 2296 | 12 | Code::CodeKey { rung: Doc, file: src/posting/__main__.py, decl: 7, sub: 0, line: 177 } |  |  | 0.856 |
| walker |  | 2312 | 16 | Code::CodeKey { rung: Doc, file: src/posting/__main__.py, decl: 6, sub: 0, line: 96 } |  |  | 0.862 |
| walker |  | 2329 | 17 | Code::CodeKey { rung: Doc, file: src/posting/__main__.py, decl: 8, sub: 0, line: 194 } |  |  | 0.864 |
| ns | 2359 |  | 307 | collection.py type roster | 3.1 |  | 0.821 |
| ns | 2534 |  | 175 | The `.posting.yaml` on-disk request format | 3.2 |  | 0.784 |
| walker |  | 2629 | 300 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.829 |
| ns | 2774 |  | 240 | RequestModel field roster | 3.3 | 3.1 | 0.801 |
| walker |  | 2856 | 227 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.801 |
| walker |  | 2863 | 7 | Fs::DirListing { dir: .codex/environments } |  |  | 0.801 |
| walker |  | 2871 | 8 | Fs::DirListing { dir: tests/resources } |  |  | 0.801 |
| walker |  | 2881 | 10 | Fs::DirListing { dir: tests/sample-envs } |  |  | 0.801 |
| ns | 2934 |  | 160 | RequestModel behaviour roster | 3.4 |  | 0.789 |
| walker |  | 2938 | 57 | Code::CodeKey { rung: Names, file: src/posting/app.py, decl: 0, sub: 0, line: 0 } |  |  | 0.789 |
| walker |  | 2953 | 15 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 1, sub: 0, line: 93 } |  |  | 0.789 |
| ns | 3145 |  | 211 | Key-value, option and script models | 3.5 | 3.1 | 0.760 |
| walker |  | 3270 | 317 | Code::CodeKey { rung: Names, file: src/posting/collection.py, decl: 0, sub: 0, line: 0 } |  |  | 0.808 |
| walker |  | 3280 | 10 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 10, sub: 0, line: 66 } |  |  | 0.808 |
| walker |  | 3296 | 16 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 11, sub: 0, line: 70 } |  |  | 0.808 |
| walker |  | 3317 | 21 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 31, sub: 0, line: 407 } |  |  | 0.808 |
| walker |  | 3338 | 21 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 32, sub: 0, line: 412 } |  |  | 0.808 |
| ns | 3349 |  | 204 | Collection tree: loading a directory, saving it back | 3.6 | 3.1 | 0.787 |
| walker |  | 3360 | 22 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 8, sub: 0, line: 56 } |  |  | 0.787 |
| walker |  | 3382 | 22 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 9, sub: 0, line: 61 } |  |  | 0.787 |
| walker |  | 3411 | 29 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 12, sub: 0, line: 75 } |  |  | 0.789 |
| walker |  | 3440 | 29 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 13, sub: 0, line: 81 } |  |  | 0.789 |
| walker |  | 3469 | 29 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 14, sub: 0, line: 87 } |  |  | 0.789 |
| walker |  | 3507 | 38 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 30, sub: 0, line: 401 } |  |  | 0.789 |
| ns | 3563 |  | 214 | config.py structure and settings-source configuration | 4.1 |  | 0.767 |
| walker |  | 3574 | 67 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 15, sub: 0, line: 93 } |  |  | 0.767 |
| walker |  | 3645 | 71 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 17, sub: 0, line: 103 } |  |  | 0.777 |
| ns | 3714 |  | 151 | Settings keys, part 1 | 4.2 |  | 0.761 |
| walker |  | 3730 | 85 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 33, sub: 0, line: 417 } |  |  | 0.761 |
| walker |  | 3853 | 123 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 21, sub: 0, line: 138 } |  |  | 0.771 |
| ns | 3892 |  | 178 | Settings keys, part 2 | 4.3 | 4.2 | 0.756 |
| walker |  | 3984 | 131 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 18, sub: 0, line: 111 } |  |  | 0.765 |
| walker |  | 4173 | 189 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 34, sub: 0, line: 427 } |  |  | 0.775 |
| walker |  | 4191 | 18 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 36, sub: 0, line: 441 } |  |  | 0.775 |
| walker |  | 4215 | 24 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 35, sub: 0, line: 434 } |  |  | 0.775 |
| ns | 4250 |  | 358 | Configuration precedence and guide section map | 4.4 |  | 0.741 |
| walker |  | 4406 | 191 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 3, sub: 0, line: 23 } |  |  | 0.741 |
| ns | 4526 |  | 276 | app.py class roster and MainScreen reactive state | 5.1 |  | 0.724 |
| walker |  | 4612 | 206 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 22, sub: 0, line: 154 } |  |  | 0.729 |
| ns | 4716 |  | 190 | Main screen keybindings, part 1 | 5.2 | 5.1 | 0.709 |
| walker |  | 4811 | 199 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 22, sub: 1, line: 154 } |  |  | 0.719 |
| ns | 4962 |  | 246 | Main screen keybindings, part 2 | 5.3 | 5.2 | 0.695 |
| walker |  | 5125 | 314 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 22, sub: 2, line: 154 } |  |  | 0.716 |
| walker |  | 5139 | 14 | Code::CodeKey { rung: Doc, file: src/posting/collection.py, decl: 23, sub: 0, line: 205 } |  |  | 0.719 |
| walker |  | 5154 | 15 | Code::CodeKey { rung: Doc, file: src/posting/collection.py, decl: 25, sub: 0, line: 291 } |  |  | 0.719 |
| ns | 5155 |  | 193 | Global app keybindings | 5.4 | 5.1 | 0.703 |
| walker |  | 5169 | 15 | Code::CodeKey { rung: Doc, file: src/posting/collection.py, decl: 38, sub: 0, line: 541 } |  |  | 0.703 |
| walker |  | 5185 | 16 | Code::CodeKey { rung: Doc, file: src/posting/collection.py, decl: 24, sub: 0, line: 269 } |  |  | 0.703 |
| walker |  | 5195 | 10 | Code::CodeKey { rung: Doc, file: src/posting/app.py, decl: 1, sub: 0, line: 93 } |  |  | 0.704 |
| walker |  | 5205 | 10 | Code::CodeKey { rung: Doc, file: src/posting/app.py, decl: 3, sub: 0, line: 108 } |  |  | 0.705 |
| ns | 5444 |  | 289 | MainScreen method roster, part 1 | 5.5 |  | 0.687 |
| walker |  | 5551 | 346 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 66, sub: 0, line: 1205 } |  |  | 0.718 |
| ns | 5679 |  | 235 | MainScreen method roster, part 2 | 5.6 |  | 0.704 |
| walker |  | 5751 | 200 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 66, sub: 1, line: 1205 } |  |  | 0.704 |
| walker |  | 5806 | 55 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 67, sub: 0, line: 1239 } |  |  | 0.704 |
| ns | 5823 |  | 144 | MainScreen.compose: the widget layout | 5.7 | 5.5 | 0.694 |
| walker |  | 6001 | 195 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 66, sub: 2, line: 1205 } |  |  | 0.695 |
| walker |  | 6007 | 6 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 80, sub: 0, line: 1576 } |  |  | 0.695 |
| walker |  | 6021 | 14 | Code::CodeKey { rung: Doc, file: src/posting/app.py, decl: 72, sub: 0, line: 1361 } |  |  | 0.695 |
| walker |  | 6036 | 15 | Code::CodeKey { rung: Doc, file: src/posting/app.py, decl: 71, sub: 0, line: 1330 } |  |  | 0.695 |
| ns | 6072 |  | 249 | `Posting` App method roster | 5.8 |  | 0.694 |
| walker |  | 6177 | 141 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 66, sub: 3, line: 1205 } |  |  | 0.703 |
| walker |  | 6190 | 13 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 82, sub: 0, line: 1591 } |  |  | 0.703 |
| walker |  | 6238 | 48 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 84, sub: 0, line: 1620 } |  |  | 0.703 |
| walker |  | 6288 | 50 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 86, sub: 0, line: 1655 } |  |  | 0.703 |
| ns | 6311 |  | 239 | Command palette: every command | 6.1 |  | 0.692 |
| ns | 6694 |  | 383 | Request-editor widget class roster | 6.2 |  | 0.675 |
| ns | 6902 |  | 208 | Response and collection-browser widget class roster | 6.3 |  | 0.667 |
| walker |  | 6966 | 678 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 4, sub: 0, line: 112 } |  |  | 0.726 |
| walker |  | 7132 | 166 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 4, sub: 1, line: 112 } |  |  | 0.737 |
| ns | 7238 |  | 336 | Shared widget class roster | 6.4 |  | 0.724 |
| walker |  | 7354 | 222 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 4, sub: 2, line: 112 } |  |  | 0.727 |
| walker |  | 7392 | 38 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 5, sub: 0, line: 196 } |  |  | 0.727 |
| ns | 7440 |  | 202 | Scripting API: the `Posting` object handed to user scripts | 6.5 |  | 0.720 |
| walker |  | 7443 | 51 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 9, sub: 0, line: 271 } |  |  | 0.720 |
| walker |  | 7453 | 10 | Code::CodeKey { rung: Doc, file: src/posting/app.py, decl: 13, sub: 0, line: 520 } |  |  | 0.720 |
| walker |  | 7643 | 190 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 4, sub: 3, line: 112 } |  |  | 0.724 |
| walker |  | 7656 | 13 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 16, sub: 0, line: 560 } |  |  | 0.724 |
| walker |  | 7673 | 17 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 17, sub: 0, line: 567 } |  |  | 0.724 |
| walker |  | 7692 | 19 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 18, sub: 0, line: 605 } |  |  | 0.724 |
| ns | 7732 |  | 292 | Variables subsystem | 6.6 |  | 0.716 |
| walker |  | 7880 | 188 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 4, sub: 4, line: 112 } |  |  | 0.729 |
| walker |  | 7890 | 10 | Code::CodeKey { rung: Doc, file: src/posting/app.py, decl: 20, sub: 0, line: 663 } |  |  | 0.729 |
| walker |  | 7901 | 11 | Code::CodeKey { rung: Doc, file: src/posting/app.py, decl: 22, sub: 0, line: 673 } |  |  | 0.729 |
| walker |  | 7914 | 13 | Code::CodeKey { rung: Doc, file: src/posting/app.py, decl: 21, sub: 0, line: 667 } |  |  | 0.729 |
| ns | 8068 |  | 336 | Theme model | 6.7 |  | 0.716 |
| walker |  | 8105 | 191 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 4, sub: 5, line: 112 } |  |  | 0.719 |
| walker |  | 8122 | 17 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 30, sub: 0, line: 759 } |  |  | 0.719 |
| walker |  | 8141 | 19 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 32, sub: 0, line: 780 } |  |  | 0.719 |
| walker |  | 8153 | 12 | Code::CodeKey { rung: Doc, file: src/posting/app.py, decl: 28, sub: 0, line: 748 } |  |  | 0.719 |
| walker |  | 8165 | 12 | Code::CodeKey { rung: Doc, file: src/posting/app.py, decl: 30, sub: 0, line: 759 } |  |  | 0.719 |
| walker |  | 8179 | 14 | Code::CodeKey { rung: Doc, file: src/posting/app.py, decl: 14, sub: 0, line: 526 } |  |  | 0.719 |
| ns | 8260 |  | 192 | Every builtin theme name | 6.8 |  | 0.712 |
| walker |  | 8369 | 190 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 4, sub: 6, line: 112 } |  |  | 0.715 |
| walker |  | 8383 | 14 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 35, sub: 0, line: 816 } |  |  | 0.715 |
| walker |  | 8402 | 19 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 33, sub: 0, line: 792 } |  |  | 0.715 |
| ns | 8413 |  | 153 | Importer entry points | 6.9 |  | 0.710 |
| walker |  | 8421 | 19 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 34, sub: 0, line: 804 } |  |  | 0.710 |
| walker |  | 8449 | 28 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 36, sub: 0, line: 824 } |  |  | 0.710 |
| ns | 8530 |  | 117 | URL and path-parameter helpers | 6.10 |  | 0.707 |
| walker |  | 8643 | 194 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 4, sub: 7, line: 112 } |  |  | 0.718 |
| walker |  | 8661 | 18 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 38, sub: 0, line: 857 } |  |  | 0.718 |
| walker |  | 8680 | 19 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 43, sub: 0, line: 999 } |  |  | 0.718 |
| walker |  | 8692 | 12 | Code::CodeKey { rung: Doc, file: src/posting/app.py, decl: 42, sub: 0, line: 969 } |  |  | 0.718 |
| ns | 8694 |  | 164 | HTTP header catalogue | 6.11 |  | 0.710 |
| walker |  | 8706 | 14 | Code::CodeKey { rung: Doc, file: src/posting/app.py, decl: 39, sub: 0, line: 901 } |  |  | 0.710 |
| walker |  | 8721 | 15 | Code::CodeKey { rung: Doc, file: src/posting/app.py, decl: 25, sub: 0, line: 696 } |  |  | 0.710 |
| walker |  | 8736 | 15 | Code::CodeKey { rung: Doc, file: src/posting/app.py, decl: 26, sub: 0, line: 704 } |  |  | 0.710 |
| walker |  | 8751 | 15 | Code::CodeKey { rung: Doc, file: src/posting/app.py, decl: 36, sub: 0, line: 824 } |  |  | 0.710 |
| walker |  | 8762 | 11 | Fs::DirListing { dir: tests/sample-themes } |  |  | 0.710 |
| walker |  | 8779 | 17 | Code::CodeKey { rung: Doc, file: src/posting/app.py, decl: 16, sub: 0, line: 560 } |  |  | 0.710 |
| walker |  | 8796 | 17 | Code::CodeKey { rung: Doc, file: src/posting/app.py, decl: 35, sub: 0, line: 816 } |  |  | 0.710 |
| walker |  | 8813 | 17 | Code::CodeKey { rung: Doc, file: src/posting/app.py, decl: 70, sub: 0, line: 1302 } |  |  | 0.710 |
| ns | 8989 |  | 295 | Test invocation: the Makefile is mandatory | 7.1 |  | 0.709 |
| walker |  | 9135 | 322 | Code::CodeKey { rung: Names, file: src/posting/themes.py, decl: 0, sub: 0, line: 0 } |  |  | 0.709 |
| ns | 9139 |  | 150 | Development environment and snapshot testing | 7.2 |  | 0.706 |
| walker |  | 9223 | 88 | Code::CodeKey { rung: Decl, file: src/posting/themes.py, decl: 11, sub: 0, line: 305 } |  |  | 0.706 |
| walker |  | 9314 | 91 | Code::CodeKey { rung: Decl, file: src/posting/themes.py, decl: 3, sub: 0, line: 54 } |  |  | 0.706 |
| ns | 9407 |  | 268 | Scripting and core workflow guide section maps | 7.3 |  | 0.694 |
| walker |  | 9449 | 135 | Code::CodeKey { rung: Decl, file: src/posting/themes.py, decl: 5, sub: 0, line: 72 } |  |  | 0.694 |
| walker |  | 9584 | 135 | Code::CodeKey { rung: Decl, file: src/posting/themes.py, decl: 7, sub: 0, line: 94 } |  |  | 0.694 |
| ns | 9692 |  | 285 | Remaining guide section maps | 7.4 |  | 0.683 |
| walker |  | 9757 | 173 | Code::CodeKey { rung: Decl, file: src/posting/themes.py, decl: 2, sub: 0, line: 34 } |  |  | 0.683 |
| walker |  | 9972 | 215 | Code::CodeKey { rung: Decl, file: src/posting/themes.py, decl: 1, sub: 0, line: 14 } |  |  | 0.683 |
| ns | 9981 |  | 289 | Remaining packaging metadata and CI | 7.5 | 1.7 | 0.671 |
| walker |  | 9995 | 23 | Code::CodeKey { rung: Decl, file: src/posting/themes.py, decl: 8, sub: 0, line: 106 } |  |  | 0.671 |

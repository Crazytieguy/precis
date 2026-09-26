Score(3000)=0.789 I=0.930 C=0.670 ns_rows≤3K=19/49 grid(1000/1442/2080/3000/4327/6240/9000)=0.747/0.830/0.799/0.789/0.737/0.703/0.709

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
| walker |  | 2863 | 7 | Fs::DirListing { dir: .codex/environments } |  |  | 0.801 |
| walker |  | 2871 | 8 | Fs::DirListing { dir: tests/resources } |  |  | 0.801 |
| walker |  | 2881 | 10 | Fs::DirListing { dir: tests/sample-envs } |  |  | 0.801 |
| ns | 2934 |  | 160 | RequestModel behaviour roster | 3.4 |  | 0.789 |
| walker |  | 2938 | 57 | Code::CodeKey { rung: Names, file: src/posting/app.py, decl: 0, sub: 0, line: 0 } |  |  | 0.789 |
| walker |  | 2953 | 15 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 1, sub: 0, line: 93 } |  |  | 0.789 |
| walker |  | 2963 | 10 | Code::CodeKey { rung: Doc, file: src/posting/app.py, decl: 1, sub: 0, line: 93 } |  |  | 0.789 |
| walker |  | 2973 | 10 | Code::CodeKey { rung: Doc, file: src/posting/app.py, decl: 3, sub: 0, line: 108 } |  |  | 0.789 |
| ns | 3145 |  | 211 | Key-value, option and script models | 3.5 | 3.1 | 0.761 |
| walker |  | 3281 | 308 | Code::CodeKey { rung: Names, file: src/posting/collection.py, decl: 0, sub: 0, line: 0 } |  |  | 0.805 |
| walker |  | 3291 | 10 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 10, sub: 0, line: 66 } |  |  | 0.805 |
| walker |  | 3307 | 16 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 11, sub: 0, line: 70 } |  |  | 0.805 |
| walker |  | 3328 | 21 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 31, sub: 0, line: 407 } |  |  | 0.805 |
| walker |  | 3349 | 21 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 32, sub: 0, line: 412 } |  |  | 0.784 |
| ns | 3349 |  | 204 | Collection tree: loading a directory, saving it back | 3.6 | 3.1 | 0.784 |
| walker |  | 3371 | 22 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 8, sub: 0, line: 56 } |  |  | 0.784 |
| walker |  | 3393 | 22 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 9, sub: 0, line: 61 } |  |  | 0.784 |
| walker |  | 3422 | 29 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 12, sub: 0, line: 75 } |  |  | 0.786 |
| walker |  | 3451 | 29 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 13, sub: 0, line: 81 } |  |  | 0.786 |
| walker |  | 3480 | 29 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 14, sub: 0, line: 87 } |  |  | 0.786 |
| walker |  | 3518 | 38 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 30, sub: 0, line: 401 } |  |  | 0.786 |
| ns | 3563 |  | 214 | config.py structure and settings-source configuration | 4.1 |  | 0.764 |
| walker |  | 3574 | 56 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 15, sub: 0, line: 93 } |  |  | 0.764 |
| walker |  | 3580 | 6 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 16, sub: 0, line: 98 } |  |  | 0.764 |
| walker |  | 3651 | 71 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 17, sub: 0, line: 103 } |  |  | 0.774 |
| ns | 3714 |  | 151 | Settings keys, part 1 | 4.2 |  | 0.759 |
| walker |  | 3736 | 85 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 33, sub: 0, line: 417 } |  |  | 0.759 |
| walker |  | 3861 | 125 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 21, sub: 0, line: 138 } |  |  | 0.768 |
| ns | 3892 |  | 178 | Settings keys, part 2 | 4.3 | 4.2 | 0.754 |
| walker |  | 3992 | 131 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 18, sub: 0, line: 111 } |  |  | 0.763 |
| walker |  | 4132 | 140 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 34, sub: 0, line: 427 } |  |  | 0.771 |
| walker |  | 4140 | 8 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 37, sub: 0, line: 480 } |  |  | 0.772 |
| walker |  | 4177 | 37 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 36, sub: 0, line: 441 } |  |  | 0.772 |
| walker |  | 4218 | 41 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 35, sub: 0, line: 434 } |  |  | 0.772 |
| ns | 4250 |  | 358 | Configuration precedence and guide section map | 4.4 |  | 0.737 |
| walker |  | 4385 | 167 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 3, sub: 0, line: 23 } |  |  | 0.737 |
| walker |  | 4393 | 8 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 5, sub: 0, line: 41 } |  |  | 0.737 |
| walker |  | 4401 | 8 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 6, sub: 0, line: 45 } |  |  | 0.737 |
| walker |  | 4409 | 8 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 7, sub: 0, line: 51 } |  |  | 0.737 |
| ns | 4526 |  | 276 | app.py class roster and MainScreen reactive state | 5.1 |  | 0.722 |
| walker |  | 4622 | 213 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 22, sub: 0, line: 154 } |  |  | 0.729 |
| ns | 4716 |  | 190 | Main screen keybindings, part 1 | 5.2 | 5.1 | 0.709 |
| walker |  | 4821 | 199 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 22, sub: 1, line: 154 } |  |  | 0.720 |
| ns | 4962 |  | 246 | Main screen keybindings, part 2 | 5.3 | 5.2 | 0.695 |
| walker |  | 5135 | 314 | Code::CodeKey { rung: Decl, file: src/posting/collection.py, decl: 22, sub: 2, line: 154 } |  |  | 0.716 |
| walker |  | 5149 | 14 | Code::CodeKey { rung: Doc, file: src/posting/collection.py, decl: 23, sub: 0, line: 205 } |  |  | 0.719 |
| ns | 5155 |  | 193 | Global app keybindings | 5.4 | 5.1 | 0.704 |
| walker |  | 5164 | 15 | Code::CodeKey { rung: Doc, file: src/posting/collection.py, decl: 25, sub: 0, line: 291 } |  |  | 0.704 |
| walker |  | 5179 | 15 | Code::CodeKey { rung: Doc, file: src/posting/collection.py, decl: 38, sub: 0, line: 541 } |  |  | 0.704 |
| walker |  | 5195 | 16 | Code::CodeKey { rung: Doc, file: src/posting/collection.py, decl: 24, sub: 0, line: 269 } |  |  | 0.704 |
| ns | 5444 |  | 289 | MainScreen method roster, part 1 | 5.5 |  | 0.687 |
| walker |  | 5541 | 346 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 66, sub: 0, line: 1205 } |  |  | 0.717 |
| ns | 5679 |  | 235 | MainScreen method roster, part 2 | 5.6 |  | 0.703 |
| walker |  | 5734 | 193 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 66, sub: 1, line: 1205 } |  |  | 0.704 |
| walker |  | 5752 | 18 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 70, sub: 0, line: 1302 } |  |  | 0.704 |
| walker |  | 5770 | 18 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 71, sub: 0, line: 1330 } |  |  | 0.704 |
| walker |  | 5788 | 18 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 72, sub: 0, line: 1361 } |  |  | 0.704 |
| ns | 5823 |  | 144 | MainScreen.compose: the widget layout | 5.7 | 5.5 | 0.693 |
| walker |  | 5855 | 67 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 67, sub: 0, line: 1239 } |  |  | 0.693 |
| walker |  | 6044 | 189 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 66, sub: 2, line: 1205 } |  |  | 0.695 |
| walker |  | 6057 | 13 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 81, sub: 0, line: 1581 } |  |  | 0.695 |
| walker |  | 6070 | 13 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 83, sub: 0, line: 1610 } |  |  | 0.695 |
| ns | 6072 |  | 249 | `Posting` App method roster | 5.8 |  | 0.703 |
| walker |  | 6088 | 18 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 80, sub: 0, line: 1576 } |  |  | 0.703 |
| walker |  | 6126 | 38 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 82, sub: 0, line: 1591 } |  |  | 0.703 |
| walker |  | 6187 | 61 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 84, sub: 0, line: 1620 } |  |  | 0.703 |
| walker |  | 6249 | 62 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 86, sub: 0, line: 1655 } |  |  | 0.703 |
| walker |  | 6263 | 14 | Code::CodeKey { rung: Doc, file: src/posting/app.py, decl: 72, sub: 0, line: 1361 } |  |  | 0.703 |
| ns | 6311 |  | 239 | Command palette: every command | 6.1 |  | 0.691 |
| ns | 6694 |  | 383 | Request-editor widget class roster | 6.2 |  | 0.675 |
| ns | 6902 |  | 208 | Response and collection-browser widget class roster | 6.3 |  | 0.666 |
| walker |  | 6941 | 678 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 4, sub: 0, line: 112 } |  |  | 0.725 |
| walker |  | 7107 | 166 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 4, sub: 1, line: 112 } |  |  | 0.736 |
| ns | 7238 |  | 336 | Shared widget class roster | 6.4 |  | 0.723 |
| walker |  | 7296 | 189 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 4, sub: 2, line: 112 } |  |  | 0.728 |
| walker |  | 7307 | 11 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 11, sub: 0, line: 512 } |  |  | 0.728 |
| walker |  | 7318 | 11 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 14, sub: 0, line: 526 } |  |  | 0.728 |
| walker |  | 7330 | 12 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 12, sub: 0, line: 516 } |  |  | 0.728 |
| walker |  | 7342 | 12 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 15, sub: 0, line: 544 } |  |  | 0.728 |
| walker |  | 7375 | 33 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 13, sub: 0, line: 520 } |  |  | 0.728 |
| walker |  | 7424 | 49 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 5, sub: 0, line: 196 } |  |  | 0.728 |
| ns | 7440 |  | 202 | Scripting API: the `Posting` object handed to user scripts | 6.5 |  | 0.721 |
| walker |  | 7486 | 62 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 9, sub: 0, line: 271 } |  |  | 0.721 |
| walker |  | 7496 | 10 | Code::CodeKey { rung: Doc, file: src/posting/app.py, decl: 13, sub: 0, line: 520 } |  |  | 0.721 |
| walker |  | 7683 | 187 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 4, sub: 3, line: 112 } |  |  | 0.735 |
| walker |  | 7698 | 15 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 19, sub: 0, line: 625 } |  |  | 0.735 |
| ns | 7732 |  | 292 | Variables subsystem | 6.6 |  | 0.726 |
| walker |  | 7735 | 37 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 16, sub: 0, line: 560 } |  |  | 0.726 |
| walker |  | 7781 | 46 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 17, sub: 0, line: 567 } |  |  | 0.726 |
| walker |  | 7830 | 49 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 18, sub: 0, line: 605 } |  |  | 0.726 |
| walker |  | 7840 | 10 | Code::CodeKey { rung: Doc, file: src/posting/app.py, decl: 20, sub: 0, line: 663 } |  |  | 0.726 |
| walker |  | 7851 | 11 | Code::CodeKey { rung: Doc, file: src/posting/app.py, decl: 22, sub: 0, line: 673 } |  |  | 0.726 |
| walker |  | 8038 | 187 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 4, sub: 4, line: 112 } |  |  | 0.735 |
| walker |  | 8059 | 21 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 31, sub: 0, line: 771 } |  |  | 0.735 |
| ns | 8068 |  | 336 | Theme model | 6.7 |  | 0.721 |
| walker |  | 8087 | 28 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 30, sub: 0, line: 759 } |  |  | 0.721 |
| walker |  | 8126 | 39 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 35, sub: 0, line: 816 } |  |  | 0.721 |
| walker |  | 8167 | 41 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 36, sub: 0, line: 824 } |  |  | 0.721 |
| walker |  | 8235 | 68 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 32, sub: 0, line: 780 } |  |  | 0.721 |
| ns | 8260 |  | 192 | Every builtin theme name | 6.8 |  | 0.714 |
| walker |  | 8303 | 68 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 33, sub: 0, line: 792 } |  |  | 0.714 |
| walker |  | 8373 | 70 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 34, sub: 0, line: 804 } |  |  | 0.714 |
| walker |  | 8385 | 12 | Code::CodeKey { rung: Doc, file: src/posting/app.py, decl: 28, sub: 0, line: 748 } |  |  | 0.714 |
| walker |  | 8397 | 12 | Code::CodeKey { rung: Doc, file: src/posting/app.py, decl: 30, sub: 0, line: 759 } |  |  | 0.714 |
| ns | 8413 |  | 153 | Importer entry points | 6.9 |  | 0.709 |
| ns | 8530 |  | 117 | URL and path-parameter helpers | 6.10 |  | 0.707 |
| walker |  | 8574 | 177 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 4, sub: 5, line: 112 } |  |  | 0.719 |
| walker |  | 8590 | 16 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 37, sub: 0, line: 835 } |  |  | 0.719 |
| walker |  | 8619 | 29 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 38, sub: 0, line: 857 } |  |  | 0.719 |
| walker |  | 8650 | 31 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 43, sub: 0, line: 999 } |  |  | 0.719 |
| walker |  | 8662 | 12 | Code::CodeKey { rung: Doc, file: src/posting/app.py, decl: 42, sub: 0, line: 969 } |  |  | 0.719 |
| walker |  | 8675 | 13 | Code::CodeKey { rung: Doc, file: src/posting/app.py, decl: 21, sub: 0, line: 667 } |  |  | 0.719 |
| ns | 8694 |  | 164 | HTTP header catalogue | 6.11 |  | 0.711 |
| walker |  | 8883 | 208 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 4, sub: 6, line: 112 } |  |  | 0.711 |
| walker |  | 8892 | 9 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 46, sub: 0, line: 1124 } |  |  | 0.711 |
| walker |  | 8901 | 9 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 47, sub: 0, line: 1128 } |  |  | 0.711 |
| walker |  | 8910 | 9 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 48, sub: 0, line: 1132 } |  |  | 0.711 |
| walker |  | 8919 | 9 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 49, sub: 0, line: 1136 } |  |  | 0.711 |
| walker |  | 8928 | 9 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 50, sub: 0, line: 1140 } |  |  | 0.711 |
| walker |  | 8937 | 9 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 51, sub: 0, line: 1144 } |  |  | 0.711 |
| walker |  | 8946 | 9 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 52, sub: 0, line: 1148 } |  |  | 0.711 |
| walker |  | 8955 | 9 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 53, sub: 0, line: 1152 } |  |  | 0.711 |
| walker |  | 8964 | 9 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 54, sub: 0, line: 1156 } |  |  | 0.711 |
| walker |  | 8973 | 9 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 55, sub: 0, line: 1160 } |  |  | 0.711 |
| walker |  | 8982 | 9 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 56, sub: 0, line: 1164 } |  |  | 0.711 |
| ns | 8989 |  | 295 | Test invocation: the Makefile is mandatory | 7.1 |  | 0.709 |
| walker |  | 8991 | 9 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 57, sub: 0, line: 1168 } |  |  | 0.709 |
| walker |  | 9005 | 14 | Code::CodeKey { rung: Doc, file: src/posting/app.py, decl: 14, sub: 0, line: 526 } |  |  | 0.709 |
| walker |  | 9019 | 14 | Code::CodeKey { rung: Doc, file: src/posting/app.py, decl: 39, sub: 0, line: 901 } |  |  | 0.709 |
| ns | 9139 |  | 150 | Development environment and snapshot testing | 7.2 |  | 0.706 |
| walker |  | 9156 | 137 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 4, sub: 7, line: 112 } |  |  | 0.706 |
| walker |  | 9165 | 9 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 58, sub: 0, line: 1172 } |  |  | 0.706 |
| walker |  | 9174 | 9 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 59, sub: 0, line: 1176 } |  |  | 0.706 |
| walker |  | 9183 | 9 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 60, sub: 0, line: 1180 } |  |  | 0.706 |
| walker |  | 9192 | 9 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 61, sub: 0, line: 1184 } |  |  | 0.706 |
| walker |  | 9201 | 9 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 62, sub: 0, line: 1188 } |  |  | 0.706 |
| walker |  | 9210 | 9 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 63, sub: 0, line: 1192 } |  |  | 0.706 |
| walker |  | 9219 | 9 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 64, sub: 0, line: 1196 } |  |  | 0.706 |
| walker |  | 9228 | 9 | Code::CodeKey { rung: Decl, file: src/posting/app.py, decl: 65, sub: 0, line: 1200 } |  |  | 0.706 |
| walker |  | 9243 | 15 | Code::CodeKey { rung: Doc, file: src/posting/app.py, decl: 25, sub: 0, line: 696 } |  |  | 0.706 |
| walker |  | 9258 | 15 | Code::CodeKey { rung: Doc, file: src/posting/app.py, decl: 26, sub: 0, line: 704 } |  |  | 0.706 |
| walker |  | 9273 | 15 | Code::CodeKey { rung: Doc, file: src/posting/app.py, decl: 36, sub: 0, line: 824 } |  |  | 0.706 |
| walker |  | 9288 | 15 | Code::CodeKey { rung: Doc, file: src/posting/app.py, decl: 71, sub: 0, line: 1330 } |  |  | 0.706 |
| walker |  | 9305 | 17 | Code::CodeKey { rung: Doc, file: src/posting/app.py, decl: 16, sub: 0, line: 560 } |  |  | 0.706 |
| walker |  | 9322 | 17 | Code::CodeKey { rung: Doc, file: src/posting/app.py, decl: 35, sub: 0, line: 816 } |  |  | 0.706 |
| walker |  | 9339 | 17 | Code::CodeKey { rung: Doc, file: src/posting/app.py, decl: 70, sub: 0, line: 1302 } |  |  | 0.706 |
| walker |  | 9357 | 18 | Code::CodeKey { rung: Doc, file: src/posting/app.py, decl: 31, sub: 0, line: 771 } |  |  | 0.706 |
| walker |  | 9375 | 18 | Code::CodeKey { rung: Doc, file: src/posting/app.py, decl: 32, sub: 0, line: 780 } |  |  | 0.706 |
| walker |  | 9393 | 18 | Code::CodeKey { rung: Doc, file: src/posting/app.py, decl: 33, sub: 0, line: 792 } |  |  | 0.706 |
| ns | 9407 |  | 268 | Scripting and core workflow guide section maps | 7.3 |  | 0.695 |
| walker |  | 9412 | 19 | Code::CodeKey { rung: Doc, file: src/posting/app.py, decl: 15, sub: 0, line: 544 } |  |  | 0.695 |
| walker |  | 9431 | 19 | Code::CodeKey { rung: Doc, file: src/posting/app.py, decl: 29, sub: 0, line: 752 } |  |  | 0.695 |
| walker |  | 9450 | 19 | Code::CodeKey { rung: Doc, file: src/posting/app.py, decl: 34, sub: 0, line: 804 } |  |  | 0.695 |
| walker |  | 9461 | 11 | Fs::DirListing { dir: tests/sample-themes } |  |  | 0.695 |
| walker |  | 9481 | 20 | Code::CodeKey { rung: Doc, file: src/posting/app.py, decl: 37, sub: 0, line: 835 } |  |  | 0.695 |
| ns | 9692 |  | 285 | Remaining guide section maps | 7.4 |  | 0.684 |
| walker |  | 9803 | 322 | Code::CodeKey { rung: Names, file: src/posting/themes.py, decl: 0, sub: 0, line: 0 } |  |  | 0.684 |
| walker |  | 9891 | 88 | Code::CodeKey { rung: Decl, file: src/posting/themes.py, decl: 11, sub: 0, line: 305 } |  |  | 0.684 |
| ns | 9981 |  | 289 | Remaining packaging metadata and CI | 7.5 | 1.7 | 0.672 |
| walker |  | 9982 | 91 | Code::CodeKey { rung: Decl, file: src/posting/themes.py, decl: 3, sub: 0, line: 54 } |  |  | 0.672 |
| walker |  | 9999 | 17 | Code::CodeKey { rung: Decl, file: src/posting/themes.py, decl: 5, sub: 0, line: 72 } |  |  | 0.672 |

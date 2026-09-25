Score(3000)=0.638 I=0.882 C=0.462 ns_rows≤3K=22/55 grid(1000/1442/2080/3000/4327/6240/9000)=0.708/0.795/0.700/0.638/0.579/0.547/0.597

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 72 | 72 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 81 | 9 | Fs::DirListing { dir: lib } |  |  | 0.000 |
| ns | 95 |  | 95 | Package identity: name, version, description, entry point, bin name | 1.1 |  | 0.000 |
| walker |  | 105 | 24 | Fs::DirListing { dir: hooks } |  |  | 0.000 |
| walker |  | 109 | 4 | Fs::DirListing { dir: .vscode } |  |  | 0.000 |
| walker |  | 114 | 5 | Fs::DirListing { dir: .devcontainer } |  |  | 0.000 |
| walker |  | 151 | 37 | Fs::DirListing { dir: widgets } |  |  | 0.000 |
| ns | 167 |  | 72 | Complete root directory listing | 1.2 |  | 0.546 |
| walker |  | 190 | 39 | Fs::DirListing { dir: src } |  |  | 0.610 |
| walker |  | 199 | 9 | Fs::DirListing { dir: src/themes } |  |  | 0.612 |
| walker |  | 230 | 31 | Fs::DirListing { dir: src/widgetsTemplates } |  |  | 0.626 |
| walker |  | 249 | 19 | Fs::DirListing { dir: widgets/images } |  |  | 0.627 |
| ns | 276 |  | 109 | Complete listings of the four runtime code directories: src/, hooks/, lib/, widgets/ | 1.3 |  | 0.626 |
| walker |  | 315 | 66 | Json::Identity { file: package.json } |  |  | 0.817 |
| walker |  | 346 | 31 | Fs::DirListing { dir: widgets/services } |  |  | 0.824 |
| ns | 371 |  | 95 | Complete listings of the per-mode widget directories: widgets/containers, widgets/images, widgets/services | 1.4 |  | 0.738 |
| walker |  | 391 | 45 | Fs::DirListing { dir: widgets/containers } |  |  | 0.833 |
| walker |  | 401 | 10 | Code::CodeKey { rung: Names, file: lib/node.version.js, decl: 0, sub: 0, line: 0 } |  |  | 0.833 |
| ns | 411 |  | 40 | Complete listings of src/widgetsTemplates and src/themes | 1.5 |  | 0.827 |
| walker |  | 418 | 17 | Code::CodeKey { rung: Decl, file: lib/node.version.js, decl: 1, sub: 0, line: 1 } |  |  | 0.828 |
| walker |  | 452 | 34 | Json::Runtime { file: package.json } |  |  | 0.829 |
| walker |  | 464 | 12 | Code::CodeKey { rung: Names, file: lib/modes.js, decl: 0, sub: 0, line: 0 } |  |  | 0.829 |
| walker |  | 497 | 33 | Code::CodeKey { rung: Decl, file: lib/modes.js, decl: 1, sub: 0, line: 3 } |  |  | 0.831 |
| ns | 533 |  | 122 | npm scripts block | 1.6 |  | 0.787 |
| walker |  | 551 | 54 | Code::CodeKey { rung: Names, file: index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.788 |
| walker |  | 570 | 19 | Fs::DirListing { dir: .github } |  |  | 0.788 |
| walker |  | 590 | 20 | Fs::DirListing { dir: .github/workflows } |  |  | 0.788 |
| ns | 630 |  | 97 | Published `files` allow-list and the node engines floor | 1.7 |  | 0.735 |
| walker |  | 753 | 163 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.736 |
| ns | 759 |  | 129 | README tagline plus every top-level heading location | 1.8 |  | 0.688 |
| walker |  | 773 | 20 | Code::CodeKey { rung: Names, file: src/dockerUtil.js, decl: 0, sub: 0, line: 0 } |  |  | 0.688 |
| walker |  | 793 | 20 | Code::CodeKey { rung: Names, file: src/screen.js, decl: 0, sub: 0, line: 0 } |  |  | 0.688 |
| ns | 891 |  | 132 | index.js: shebang and the module wiring of the executable | 1.9 |  | 0.648 |
| walker |  | 896 | 103 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.708 |
| walker |  | 918 | 22 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.708 |
| walker |  | 1027 | 109 | Plaintext::Whole { file: dockerRunScript.sh } |  |  | 0.709 |
| ns | 1053 |  | 162 | index.js: pre-flight CLI dispatch (--help, --version, node version floor) | 1.10 | 1.9 | 0.656 |
| ns | 1219 |  | 166 | index.js: bootstrap promise chain and the three helper function signatures | 1.11 | 1.10 | 0.609 |
| walker |  | 1222 | 195 | Json::Dependencies { file: package.json } |  |  | 0.615 |
| walker |  | 1329 | 107 | Json::Entry { file: package.json } |  |  | 0.783 |
| walker |  | 1381 | 52 | Fs::DirListing { dir: docs } |  |  | 0.784 |
| walker |  | 1390 | 9 | Fs::DirListing { dir: docs/src } |  |  | 0.784 |
| walker |  | 1403 | 13 | Fs::DirListing { dir: docs/src/pages } |  |  | 0.784 |
| ns | 1414 |  | 195 | Runtime dependency list from package.json | 1.12 |  | 0.795 |
| walker |  | 1523 | 120 | Json::Scripts { file: package.json } |  |  | 0.826 |
| walker |  | 1548 | 25 | Fs::DirListing { dir: docs/src/components } |  |  | 0.827 |
| walker |  | 1718 | 170 | Plaintext::Whole { file: Dockerfile } |  |  | 0.832 |
| ns | 1737 |  | 323 | The in-app keybinding table (src/widgetsTemplates/help.widget.template.js:102-125) | 2.1 |  | 0.767 |
| ns | 1857 |  | 120 | src/cli.js: the complete flag roster (all eight option names) | 2.2 |  | 0.744 |
| walker |  | 1890 | 172 | Code::CodeKey { rung: Decl, file: src/screen.js, decl: 1, sub: 0, line: 18 } |  |  | 0.747 |
| walker |  | 1899 | 9 | Code::CodeKey { rung: Body, file: src/screen.js, decl: 16, sub: 0, line: 192 } |  |  | 0.747 |
| walker |  | 1963 | 64 | Code::CodeKey { rung: Names, file: src/cli.js, decl: 0, sub: 0, line: 0 } |  |  | 0.749 |
| walker |  | 1976 | 13 | Code::CodeKey { rung: Body, file: src/cli.js, decl: 4, sub: 0, line: 93 } |  |  | 0.749 |
| walker |  | 2009 | 33 | Code::CodeKey { rung: Doc, file: src/cli.js, decl: 4, sub: 0, line: 93 } |  |  | 0.749 |
| walker |  | 2019 | 10 | Code::CodeKey { rung: Names, file: src/enum.js, decl: 0, sub: 0, line: 0 } |  |  | 0.749 |
| ns | 2043 |  | 186 | src/cli.js: the four docker-connection options in full (socketPath, host, port, protocol) | 2.3 | 2.2 | 0.700 |
| ns | 2206 |  | 163 | src/cli.js: the four behavioural options in full (help, version, containerFilters, theme) | 2.4 | 2.2 | 0.660 |
| ns | 2269 |  | 63 | src/cli.js: every prototype method and the singleton export | 2.5 |  | 0.662 |
| walker |  | 2316 | 297 | Code::CodeKey { rung: Decl, file: src/dockerUtil.js, decl: 1, sub: 0, line: 5 } |  |  | 0.664 |
| ns | 2339 |  | 70 | README: install and launch commands | 2.6 |  | 0.654 |
| walker |  | 2342 | 26 | Code::CodeKey { rung: Body, file: src/dockerUtil.js, decl: 14, sub: 0, line: 165 } |  |  | 0.654 |
| walker |  | 2368 | 26 | Code::CodeKey { rung: Body, file: src/dockerUtil.js, decl: 15, sub: 0, line: 170 } |  |  | 0.655 |
| walker |  | 2394 | 26 | Code::CodeKey { rung: Body, file: src/dockerUtil.js, decl: 16, sub: 0, line: 175 } |  |  | 0.655 |
| walker |  | 2399 | 5 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.655 |
| walker |  | 2431 | 32 | Code::CodeKey { rung: Body, file: src/screen.js, decl: 11, sub: 0, line: 126 } |  |  | 0.655 |
| ns | 2455 |  | 116 | README: --containerFilters semantics | 2.7 |  | 0.649 |
| walker |  | 2463 | 32 | Code::CodeKey { rung: Body, file: src/screen.js, decl: 12, sub: 0, line: 132 } |  |  | 0.649 |
| walker |  | 2524 | 61 | Code::CodeKey { rung: Body, file: src/cli.js, decl: 2, sub: 0, line: 59 } |  |  | 0.652 |
| walker |  | 2543 | 19 | Code::CodeKey { rung: Names, file: src/baseWidget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.652 |
| ns | 2625 |  | 170 | Dockerfile in full | 2.8 |  | 0.670 |
| walker |  | 2638 | 95 | Code::CodeKey { rung: Body, file: index.js, decl: 3, sub: 0, line: 72 } |  |  | 0.670 |
| walker |  | 2665 | 27 | Code::CodeKey { rung: Body, file: src/dockerUtil.js, decl: 12, sub: 0, line: 155 } |  |  | 0.670 |
| walker |  | 2687 | 22 | Code::CodeKey { rung: Names, file: src/assetsLoader.js, decl: 0, sub: 0, line: 0 } |  |  | 0.670 |
| ns | 2724 |  | 99 | README: running and building the docker image | 2.9 |  | 0.660 |
| walker |  | 2729 | 42 | Code::CodeKey { rung: Decl, file: src/assetsLoader.js, decl: 1, sub: 0, line: 10 } |  |  | 0.660 |
| walker |  | 2765 | 36 | Code::CodeKey { rung: Body, file: src/screen.js, decl: 10, sub: 0, line: 120 } |  |  | 0.660 |
| ns | 2876 |  | 152 | .github/workflows/main.yml: the lint job | 2.10 |  | 0.638 |
| ns | 3087 |  | 211 | .github/workflows/main.yml: the semantic-release job | 2.11 |  | 0.620 |
| ns | 3170 |  | 83 | lib/modes.js and lib/node.version.js in full | 3.1 |  | 0.624 |
| walker |  | 3232 | 467 | Json::IdentityMeta { file: package.json } |  |  | 0.624 |
| walker |  | 3246 | 14 | Code::CodeKey { rung: Names, file: src/themes/theme.selector.js, decl: 0, sub: 0, line: 0 } |  |  | 0.624 |
| ns | 3375 |  | 205 | src/screen.js: imports and the mode -> grid-layout table | 3.2 |  | 0.608 |
| walker |  | 3385 | 139 | Code::CodeKey { rung: Body, file: index.js, decl: 4, sub: 0, line: 86 } |  |  | 0.610 |
| walker |  | 3476 | 91 | Code::CodeKey { rung: Decl, file: src/enum.js, decl: 1, sub: 0, line: 1 } |  |  | 0.611 |
| walker |  | 3503 | 27 | Code::CodeKey { rung: Body, file: src/dockerUtil.js, decl: 13, sub: 0, line: 160 } |  |  | 0.611 |
| ns | 3573 |  | 198 | src/screen.js: complete method roster of the `screen` class | 3.3 |  | 0.618 |
| walker |  | 3663 | 160 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.634 |
| ns | 3770 |  | 197 | src/screen.js: init() — the full boot sequence | 3.4 | 3.3 | 0.615 |
| walker |  | 3808 | 145 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.615 |
| ns | 3900 |  | 130 | src/screen.js: constructor state fields | 3.5 | 3.3 | 0.602 |
| walker |  | 3992 | 184 | Code::CodeKey { rung: Body, file: index.js, decl: 2, sub: 0, line: 47 } |  |  | 0.603 |
| ns | 4024 |  | 124 | src/screen.js: initScreen() — blessed screen and the 12x12 grid | 3.6 | 3.3 | 0.594 |
| walker |  | 4029 | 37 | Code::CodeKey { rung: Body, file: src/screen.js, decl: 9, sub: 0, line: 114 } |  |  | 0.596 |
| walker |  | 4213 | 184 | Code::CodeKey { rung: Body, file: src/cli.js, decl: 3, sub: 0, line: 66 } |  |  | 0.598 |
| walker |  | 4239 | 26 | Code::CodeKey { rung: Names, file: src/widgetsTemplates/help.widget.template.js, decl: 0, sub: 0, line: 0 } |  |  | 0.598 |
| walker |  | 4265 | 26 | Code::CodeKey { rung: Names, file: src/widgetsTemplates/info.widget.template.js, decl: 0, sub: 0, line: 0 } |  |  | 0.598 |
| ns | 4286 |  | 262 | src/screen.js: initHooks() and initWidgets() — instantiation and layout gating | 3.7 | 3.3 | 0.579 |
| walker |  | 4291 | 26 | Code::CodeKey { rung: Names, file: src/widgetsTemplates/logs.widget.template.js, decl: 0, sub: 0, line: 0 } |  |  | 0.579 |
| walker |  | 4306 | 15 | Code::CodeKey { rung: Body, file: src/assetsLoader.js, decl: 4, sub: 0, line: 39 } |  |  | 0.579 |
| walker |  | 4334 | 28 | Code::CodeKey { rung: Names, file: src/widgetsTemplates/base.hook.template.js, decl: 0, sub: 0, line: 0 } |  |  | 0.579 |
| walker |  | 4363 | 29 | Code::CodeKey { rung: Decl, file: src/widgetsTemplates/base.hook.template.js, decl: 1, sub: 0, line: 6 } |  |  | 0.579 |
| walker |  | 4391 | 28 | Code::CodeKey { rung: Names, file: src/widgetsTemplates/list.widget.template.js, decl: 0, sub: 0, line: 0 } |  |  | 0.579 |
| walker |  | 4482 | 91 | Code::CodeKey { rung: Decl, file: src/widgetsTemplates/logs.widget.template.js, decl: 1, sub: 0, line: 7 } |  |  | 0.579 |
| walker |  | 4524 | 42 | Code::CodeKey { rung: Body, file: src/dockerUtil.js, decl: 17, sub: 0, line: 180 } |  |  | 0.579 |
| ns | 4580 |  | 294 | src/screen.js: toggleMode() and registerEvents() — the global q/v keys and focus borders | 3.8 | 3.3 | 0.559 |
| walker |  | 4636 | 112 | Code::CodeKey { rung: Decl, file: src/widgetsTemplates/info.widget.template.js, decl: 1, sub: 0, line: 6 } |  |  | 0.560 |
| walker |  | 4644 | 8 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/info.widget.template.js, decl: 7, sub: 0, line: 107 } |  |  | 0.560 |
| walker |  | 4657 | 13 | Fs::DirListing { dir: docs/src/assets } |  |  | 0.560 |
| walker |  | 4663 | 6 | Fs::DirListing { dir: docs/src/assets/css } |  |  | 0.560 |
| ns | 4668 |  | 88 | src/assetsLoader.js: the glob patterns that define the plugin conventions | 3.9 |  | 0.555 |
| walker |  | 4714 | 51 | Code::CodeKey { rung: Body, file: src/screen.js, decl: 8, sub: 0, line: 107 } |  |  | 0.557 |
| walker |  | 4838 | 124 | Code::CodeKey { rung: Decl, file: src/widgetsTemplates/help.widget.template.js, decl: 1, sub: 0, line: 5 } |  |  | 0.557 |
| walker |  | 4846 | 8 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/help.widget.template.js, decl: 8, sub: 0, line: 130 } |  |  | 0.557 |
| ns | 4871 |  | 203 | src/assetsLoader.js: the loader body and the asset naming rule | 3.10 | 3.9 | 0.547 |
| ns | 5149 |  | 278 | src/grid.config.js: the containers-mode layout in full | 3.11 |  | 0.539 |
| walker |  | 5225 | 379 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.558 |
| walker |  | 5246 | 21 | Code::CodeKey { rung: Body, file: src/assetsLoader.js, decl: 2, sub: 0, line: 11 } |  |  | 0.560 |
| walker |  | 5257 | 11 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/logs.widget.template.js, decl: 6, sub: 0, line: 76 } |  |  | 0.560 |
| walker |  | 5268 | 11 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/logs.widget.template.js, decl: 7, sub: 0, line: 80 } |  |  | 0.560 |
| walker |  | 5310 | 42 | Code::CodeKey { rung: Body, file: src/dockerUtil.js, decl: 18, sub: 0, line: 187 } |  |  | 0.560 |
| ns | 5554 |  | 405 | src/grid.config.js: the services and images layouts plus the export block | 3.12 | 3.11 | 0.546 |
| walker |  | 5600 | 290 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.546 |
| walker |  | 5651 | 51 | Code::CodeKey { rung: Names, file: src/themes/styles.js, decl: 0, sub: 0, line: 0 } |  |  | 0.546 |
| walker |  | 5705 | 54 | Code::CodeKey { rung: Body, file: src/screen.js, decl: 6, sub: 0, line: 79 } |  |  | 0.548 |
| walker |  | 5720 | 15 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/help.widget.template.js, decl: 9, sub: 0, line: 134 } |  |  | 0.548 |
| walker |  | 5735 | 15 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/info.widget.template.js, decl: 8, sub: 0, line: 111 } |  |  | 0.548 |
| walker |  | 5750 | 15 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/logs.widget.template.js, decl: 5, sub: 0, line: 72 } |  |  | 0.548 |
| ns | 5884 |  | 330 | src/dockerUtil.js: complete method roster of the dockerode wrapper | 4.1 |  | 0.558 |
| walker |  | 5927 | 177 | Code::CodeKey { rung: Decl, file: src/widgetsTemplates/list.widget.template.js, decl: 1, sub: 0, line: 8 } |  |  | 0.559 |
| walker |  | 5942 | 15 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/list.widget.template.js, decl: 7, sub: 0, line: 127 } |  |  | 0.559 |
| walker |  | 5957 | 15 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/list.widget.template.js, decl: 12, sub: 0, line: 147 } |  |  | 0.559 |
| walker |  | 5972 | 15 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/list.widget.template.js, decl: 13, sub: 0, line: 151 } |  |  | 0.559 |
| walker |  | 5988 | 16 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/base.hook.template.js, decl: 3, sub: 0, line: 29 } |  |  | 0.559 |
| walker |  | 6005 | 17 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/help.widget.template.js, decl: 4, sub: 0, line: 50 } |  |  | 0.559 |
| walker |  | 6022 | 17 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/info.widget.template.js, decl: 4, sub: 0, line: 64 } |  |  | 0.559 |
| ns | 6164 |  | 280 | src/dockerUtil.js: constructor — connection config and containerFilters parsing | 4.2 | 4.1 | 0.547 |
| ns | 6314 |  | 150 | src/dockerUtil.js: getInfo() — the host fields surfaced to the status widgets | 4.3 | 4.1 | 0.540 |
| ns | 6429 |  | 115 | src/dockerUtil.js: log streaming options for containers and services | 4.4 | 4.1 | 0.534 |
| walker |  | 6453 | 431 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.534 |
| walker |  | 6599 | 146 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.534 |
| ns | 6656 |  | 227 | src/baseWidget.js — the mixin every widget and hook extends | 5.1 |  | 0.523 |
| walker |  | 6805 | 206 | Code::CodeKey { rung: Decl, file: src/themes/styles.js, decl: 2, sub: 0, line: 28 } |  |  | 0.523 |
| ns | 6888 |  | 232 | src/widgetsTemplates/base.hook.template.js — the hook base class | 5.2 |  | 0.514 |
| walker |  | 7012 | 207 | Code::CodeKey { rung: Decl, file: src/themes/styles.js, decl: 1, sub: 0, line: 1 } |  |  | 0.514 |
| walker |  | 7078 | 66 | Json::Identity { file: docs/package.json } |  |  | 0.514 |
| walker |  | 7085 | 7 | Plaintext::Whole { file: .nvmrc } |  |  | 0.514 |
| ns | 7121 |  | 233 | src/widgetsTemplates/list.widget.template.js: complete method roster | 5.3 |  | 0.520 |
| walker |  | 7122 | 37 | Markdown::HeadingsOutline { file: SECURITY.md } |  |  | 0.520 |
| walker |  | 7185 | 63 | Code::CodeKey { rung: Body, file: src/dockerUtil.js, decl: 5, sub: 0, line: 55 } |  |  | 0.521 |
| walker |  | 7200 | 15 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/logs.widget.template.js, decl: 8, sub: 0, line: 84 } |  |  | 0.521 |
| walker |  | 7216 | 16 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/list.widget.template.js, decl: 8, sub: 0, line: 131 } |  |  | 0.521 |
| walker |  | 7233 | 17 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/help.widget.template.js, decl: 5, sub: 0, line: 54 } |  |  | 0.521 |
| walker |  | 7250 | 17 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/info.widget.template.js, decl: 5, sub: 0, line: 68 } |  |  | 0.521 |
| ns | 7409 |  | 288 | src/widgetsTemplates/info.widget.template.js and logs.widget.template.js: method rosters | 5.4 |  | 0.525 |
| ns | 7570 |  | 161 | src/widgetsTemplates/help.widget.template.js: method roster around the help text | 5.5 |  | 0.524 |
| walker |  | 7702 | 452 | Code::CodeKey { rung: Body, file: src/cli.js, decl: 1, sub: 0, line: 8 } |  |  | 0.591 |
| walker |  | 7765 | 63 | Markdown::ReadmeHeadline { file: docs/README.md } |  |  | 0.591 |
| walker |  | 7783 | 18 | Markdown::HeadingsOutline { file: docs/README.md } |  |  | 0.591 |
| walker |  | 7856 | 73 | Code::CodeKey { rung: Body, file: src/screen.js, decl: 5, sub: 0, line: 71 } |  |  | 0.594 |
| ns | 7919 |  | 349 | hooks/: complete method rosters of all four hooks | 6.1 |  | 0.579 |
| walker |  | 7922 | 66 | Code::CodeKey { rung: Body, file: src/dockerUtil.js, decl: 3, sub: 0, line: 33 } |  |  | 0.580 |
| walker |  | 7938 | 16 | Markdown::Section { file: docs/README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.580 |
| walker |  | 7992 | 54 | Markdown::HeadingsOutline { file: CONTRIBUTING.md } |  |  | 0.580 |
| walker |  | 8028 | 36 | Markdown::HeadingsOutline { file: .github/PULL_REQUEST_TEMPLATE.md } |  |  | 0.580 |
| walker |  | 8044 | 16 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/list.widget.template.js, decl: 9, sub: 0, line: 135 } |  |  | 0.580 |
| walker |  | 8070 | 26 | Fs::DirListing { dir: docs/src/assets/scss } |  |  | 0.580 |
| walker |  | 8081 | 11 | Fs::DirListing { dir: docs/src/assets/scss/base } |  |  | 0.580 |
| ns | 8128 |  | 209 | hooks/containers.hook.js: the toolbar key dispatch inside init() | 6.2 | 6.1 | 0.572 |
| walker |  | 8150 | 69 | Code::CodeKey { rung: Body, file: src/themes/theme.selector.js, decl: 1, sub: 0, line: 13 } |  |  | 0.572 |
| walker |  | 8191 | 41 | Markdown::Section { file: docs/README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.572 |
| walker |  | 8263 | 72 | Code::CodeKey { rung: Body, file: src/dockerUtil.js, decl: 7, sub: 0, line: 78 } |  |  | 0.573 |
| walker |  | 8283 | 20 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/help.widget.template.js, decl: 10, sub: 0, line: 138 } |  |  | 0.573 |
| walker |  | 8303 | 20 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/info.widget.template.js, decl: 9, sub: 0, line: 115 } |  |  | 0.576 |
| walker |  | 8377 | 74 | Markdown::HeadingsOutline { file: CODE_OF_CONDUCT.md } |  |  | 0.576 |
| walker |  | 8393 | 16 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/list.widget.template.js, decl: 10, sub: 0, line: 139 } |  |  | 0.576 |
| walker |  | 8415 | 22 | Fs::DirListing { dir: docs/src/assets/scss/libs } |  |  | 0.576 |
| ns | 8482 |  | 354 | hooks/shell.hook.js: openShell() plus dockerRunScript.sh's docker exec line | 6.3 | 6.1 | 0.567 |
| ns | 8583 |  | 101 | src/enum.js in full — the ContainerState vocabulary | 7.1 |  | 0.572 |
| walker |  | 8601 | 186 | Code::CodeKey { rung: Body, file: src/baseWidget.js, decl: 1, sub: 0, line: 4 } |  |  | 0.590 |
| walker |  | 8626 | 25 | Fs::DirListing { dir: docs/src/assets/scss/layout } |  |  | 0.590 |
| walker |  | 8746 | 120 | Code::CodeKey { rung: Body, file: src/screen.js, decl: 3, sub: 0, line: 35 } |  |  | 0.600 |
| walker |  | 8828 | 82 | Code::CodeKey { rung: Body, file: src/dockerUtil.js, decl: 4, sub: 0, line: 43 } |  |  | 0.601 |
| walker |  | 8873 | 45 | Fs::DirListing { dir: docs/src/assets/fonts } |  |  | 0.601 |
| walker |  | 8889 | 16 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/list.widget.template.js, decl: 11, sub: 0, line: 143 } |  |  | 0.601 |
| ns | 8893 |  | 310 | src/themes/theme.selector.js in full and the dark/light style keys | 7.2 |  | 0.597 |
| ns | 9179 |  | 286 | widgets/containers/containerList.widget.js: complete method roster | 7.3 |  | 0.590 |
| walker |  | 9386 | 497 | Plaintext::Whole { file: .github/workflows/main.yml } |  |  | 0.620 |
| ns | 9448 |  | 269 | widgets/toolbar.widget.js: the per-mode command extension map | 7.4 |  | 0.612 |
| walker |  | 9512 | 126 | Code::CodeKey { rung: Body, file: src/screen.js, decl: 2, sub: 0, line: 19 } |  |  | 0.624 |
| ns | 9596 |  | 148 | widgets/actionsMenu.widget.js: the `m` menu's action table | 7.5 |  | 0.619 |
| walker |  | 9598 | 86 | Code::CodeKey { rung: Body, file: src/dockerUtil.js, decl: 8, sub: 0, line: 88 } |  |  | 0.620 |
| ns | 9640 |  | 44 | Complete listings of .github/ and its subdirectories | 8.1 |  | 0.623 |
| ns | 9739 |  | 99 | Complete listings of the docs/ Gatsby site | 8.2 |  | 0.630 |
| ns | 9988 |  | 249 | Secondary ops config: codefresh.yml, devcontainer image, VS Code attach config, .nvmrc | 8.3 |  | 0.622 |

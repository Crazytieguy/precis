Score(3000)=0.708 I=0.917 C=0.547 ns_rows≤3K=22/55 grid(1000/1442/2080/3000/4327/6240/9000)=0.944/0.838/0.788/0.708/0.621/0.548/0.535

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 72 | 72 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 81 | 9 | Fs::DirListing { dir: lib } |  |  | 0.000 |
| ns | 95 |  | 95 | Package identity: name, version, description, entry point, bin name | 1.1 |  | 0.000 |
| walker |  | 164 | 83 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.000 |
| ns | 167 |  | 72 | Complete root directory listing | 1.2 |  | 0.514 |
| walker |  | 188 | 24 | Fs::DirListing { dir: hooks } |  |  | 0.521 |
| walker |  | 192 | 4 | Fs::DirListing { dir: .vscode } |  |  | 0.521 |
| walker |  | 197 | 5 | Fs::DirListing { dir: .devcontainer } |  |  | 0.521 |
| walker |  | 234 | 37 | Fs::DirListing { dir: widgets } |  |  | 0.548 |
| walker |  | 273 | 39 | Fs::DirListing { dir: src } |  |  | 0.612 |
| ns | 276 |  | 109 | Complete listings of the four runtime code directories: src/, hooks/, lib/, widgets/ | 1.3 |  | 0.610 |
| walker |  | 282 | 9 | Fs::DirListing { dir: src/themes } |  |  | 0.612 |
| walker |  | 313 | 31 | Fs::DirListing { dir: src/widgetsTemplates } |  |  | 0.626 |
| ns | 371 |  | 95 | Complete listings of the per-mode widget directories: widgets/containers, widgets/images, widgets/services | 1.4 |  | 0.536 |
| walker |  | 379 | 66 | Json::Identity { file: package.json } |  |  | 0.700 |
| walker |  | 398 | 19 | Fs::DirListing { dir: widgets/images } |  |  | 0.706 |
| ns | 411 |  | 40 | Complete listings of src/widgetsTemplates and src/themes | 1.5 |  | 0.712 |
| walker |  | 429 | 31 | Fs::DirListing { dir: widgets/services } |  |  | 0.742 |
| walker |  | 463 | 34 | Json::Runtime { file: package.json } |  |  | 0.743 |
| walker |  | 508 | 45 | Fs::DirListing { dir: widgets/containers } |  |  | 0.829 |
| walker |  | 527 | 19 | Fs::DirListing { dir: .github } |  |  | 0.830 |
| ns | 533 |  | 122 | npm scripts block | 1.6 |  | 0.786 |
| walker |  | 547 | 20 | Fs::DirListing { dir: .github/workflows } |  |  | 0.786 |
| ns | 630 |  | 97 | Published `files` allow-list and the node engines floor | 1.7 |  | 0.733 |
| walker |  | 650 | 103 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.744 |
| walker |  | 672 | 22 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.745 |
| ns | 759 |  | 129 | README tagline plus every top-level heading location | 1.8 |  | 0.749 |
| walker |  | 779 | 107 | Json::Entry { file: package.json } |  |  | 0.958 |
| ns | 891 |  | 132 | index.js: shebang and the module wiring of the executable | 1.9 |  | 0.903 |
| walker |  | 899 | 120 | Json::Scripts { file: package.json } |  |  | 0.943 |
| walker |  | 951 | 52 | Fs::DirListing { dir: docs } |  |  | 0.943 |
| walker |  | 960 | 9 | Fs::DirListing { dir: docs/src } |  |  | 0.944 |
| walker |  | 973 | 13 | Fs::DirListing { dir: docs/src/pages } |  |  | 0.944 |
| ns | 1053 |  | 162 | index.js: pre-flight CLI dispatch (--help, --version, node version floor) | 1.10 | 1.9 | 0.874 |
| walker |  | 1143 | 170 | Plaintext::Whole { file: Dockerfile } |  |  | 0.879 |
| walker |  | 1168 | 25 | Fs::DirListing { dir: docs/src/components } |  |  | 0.880 |
| ns | 1219 |  | 166 | index.js: bootstrap promise chain and the three helper function signatures | 1.11 | 1.10 | 0.812 |
| walker |  | 1290 | 122 | Code::CodeKey { rung: Names, file: index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.825 |
| walker |  | 1312 | 22 | Code::CodeKey { rung: Decl, file: index.js, decl: 3, sub: 0, line: 18 } |  |  | 0.833 |
| walker |  | 1334 | 22 | Code::CodeKey { rung: Decl, file: index.js, decl: 4, sub: 0, line: 23 } |  |  | 0.843 |
| walker |  | 1385 | 51 | Code::CodeKey { rung: Decl, file: index.js, decl: 5, sub: 0, line: 28 } |  |  | 0.879 |
| ns | 1414 |  | 195 | Runtime dependency list from package.json | 1.12 |  | 0.838 |
| walker |  | 1493 | 108 | Code::CodeKey { rung: Decl, file: index.js, decl: 6, sub: 0, line: 34 } |  |  | 0.889 |
| walker |  | 1688 | 195 | Json::Dependencies { file: package.json } |  |  | 0.939 |
| walker |  | 1708 | 20 | Code::CodeKey { rung: Names, file: src/dockerUtil.js, decl: 0, sub: 0, line: 0 } |  |  | 0.939 |
| ns | 1737 |  | 323 | The in-app keybinding table (src/widgetsTemplates/help.widget.template.js:102-125) | 2.1 |  | 0.866 |
| ns | 1857 |  | 120 | src/cli.js: the complete flag roster (all eight option names) | 2.2 |  | 0.840 |
| walker |  | 2005 | 297 | Code::CodeKey { rung: Decl, file: src/dockerUtil.js, decl: 1, sub: 0, line: 5 } |  |  | 0.843 |
| walker |  | 2025 | 20 | Code::CodeKey { rung: Names, file: src/screen.js, decl: 0, sub: 0, line: 0 } |  |  | 0.843 |
| ns | 2043 |  | 186 | src/cli.js: the four docker-connection options in full (socketPath, host, port, protocol) | 2.3 | 2.2 | 0.788 |
| walker |  | 2197 | 172 | Code::CodeKey { rung: Decl, file: src/screen.js, decl: 1, sub: 0, line: 18 } |  |  | 0.791 |
| ns | 2206 |  | 163 | src/cli.js: the four behavioural options in full (help, version, containerFilters, theme) | 2.4 | 2.2 | 0.745 |
| walker |  | 2220 | 23 | Code::CodeKey { rung: Names, file: hooks/containers.hook.js, decl: 0, sub: 0, line: 0 } |  |  | 0.745 |
| ns | 2269 |  | 63 | src/cli.js: every prototype method and the singleton export | 2.5 |  | 0.733 |
| walker |  | 2320 | 100 | Code::CodeKey { rung: Decl, file: hooks/containers.hook.js, decl: 1, sub: 0, line: 5 } |  |  | 0.733 |
| ns | 2339 |  | 70 | README: install and launch commands | 2.6 |  | 0.722 |
| walker |  | 2343 | 23 | Code::CodeKey { rung: Names, file: hooks/services.hook.js, decl: 0, sub: 0, line: 0 } |  |  | 0.722 |
| walker |  | 2407 | 64 | Code::CodeKey { rung: Decl, file: hooks/services.hook.js, decl: 1, sub: 0, line: 5 } |  |  | 0.722 |
| walker |  | 2430 | 23 | Code::CodeKey { rung: Names, file: hooks/images.hook.js, decl: 0, sub: 0, line: 0 } |  |  | 0.722 |
| ns | 2455 |  | 116 | README: --containerFilters semantics | 2.7 |  | 0.716 |
| walker |  | 2479 | 49 | Code::CodeKey { rung: Decl, file: hooks/images.hook.js, decl: 1, sub: 0, line: 5 } |  |  | 0.716 |
| walker |  | 2488 | 9 | Code::CodeKey { rung: Body, file: src/screen.js, decl: 16, sub: 0, line: 192 } |  |  | 0.717 |
| walker |  | 2512 | 24 | Code::CodeKey { rung: Names, file: hooks/shell.hook.js, decl: 0, sub: 0, line: 0 } |  |  | 0.717 |
| walker |  | 2552 | 40 | Code::CodeKey { rung: Decl, file: hooks/shell.hook.js, decl: 1, sub: 0, line: 8 } |  |  | 0.717 |
| walker |  | 2564 | 12 | Code::CodeKey { rung: Names, file: lib/modes.js, decl: 0, sub: 0, line: 0 } |  |  | 0.717 |
| walker |  | 2597 | 33 | Code::CodeKey { rung: Decl, file: lib/modes.js, decl: 1, sub: 0, line: 3 } |  |  | 0.718 |
| walker |  | 2607 | 10 | Code::CodeKey { rung: Names, file: lib/node.version.js, decl: 0, sub: 0, line: 0 } |  |  | 0.718 |
| walker |  | 2624 | 17 | Code::CodeKey { rung: Decl, file: lib/node.version.js, decl: 1, sub: 0, line: 1 } |  |  | 0.719 |
| ns | 2625 |  | 170 | Dockerfile in full | 2.8 |  | 0.732 |
| walker |  | 2688 | 64 | Code::CodeKey { rung: Names, file: src/cli.js, decl: 0, sub: 0, line: 0 } |  |  | 0.740 |
| walker |  | 2701 | 13 | Code::CodeKey { rung: Body, file: src/cli.js, decl: 4, sub: 0, line: 93 } |  |  | 0.744 |
| ns | 2724 |  | 99 | README: running and building the docker image | 2.9 |  | 0.732 |
| walker |  | 2727 | 26 | Code::CodeKey { rung: Names, file: widgets/actionsMenu.widget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.732 |
| walker |  | 2833 | 106 | Code::CodeKey { rung: Decl, file: widgets/actionsMenu.widget.js, decl: 1, sub: 0, line: 5 } |  |  | 0.732 |
| walker |  | 2841 | 8 | Code::CodeKey { rung: Body, file: widgets/actionsMenu.widget.js, decl: 9, sub: 0, line: 148 } |  |  | 0.732 |
| walker |  | 2846 | 5 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.732 |
| walker |  | 2874 | 28 | Code::CodeKey { rung: Names, file: widgets/toolbar.widget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.732 |
| ns | 2876 |  | 152 | .github/workflows/main.yml: the lint job | 2.10 |  | 0.708 |
| walker |  | 2933 | 59 | Code::CodeKey { rung: Decl, file: widgets/toolbar.widget.js, decl: 1, sub: 0, line: 7 } |  |  | 0.708 |
| walker |  | 2941 | 8 | Code::CodeKey { rung: Body, file: widgets/toolbar.widget.js, decl: 3, sub: 0, line: 21 } |  |  | 0.708 |
| walker |  | 2950 | 9 | Code::CodeKey { rung: Body, file: widgets/toolbar.widget.js, decl: 4, sub: 0, line: 25 } |  |  | 0.708 |
| walker |  | 2978 | 28 | Code::CodeKey { rung: Names, file: widgets/searchInput.widget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.708 |
| walker |  | 3060 | 82 | Code::CodeKey { rung: Decl, file: widgets/searchInput.widget.js, decl: 1, sub: 0, line: 9 } |  |  | 0.708 |
| walker |  | 3067 | 7 | Code::CodeKey { rung: Body, file: widgets/searchInput.widget.js, decl: 3, sub: 0, line: 24 } |  |  | 0.708 |
| walker |  | 3075 | 8 | Code::CodeKey { rung: Body, file: widgets/searchInput.widget.js, decl: 4, sub: 0, line: 28 } |  |  | 0.708 |
| ns | 3087 |  | 211 | .github/workflows/main.yml: the semantic-release job | 2.11 |  | 0.688 |
| walker |  | 3103 | 28 | Code::CodeKey { rung: Names, file: widgets/actionStatus.widget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.688 |
| ns | 3170 |  | 83 | lib/modes.js and lib/node.version.js in full | 3.1 |  | 0.690 |
| walker |  | 3173 | 70 | Code::CodeKey { rung: Decl, file: widgets/actionStatus.widget.js, decl: 1, sub: 0, line: 6 } |  |  | 0.690 |
| walker |  | 3181 | 8 | Code::CodeKey { rung: Body, file: widgets/actionStatus.widget.js, decl: 4, sub: 0, line: 24 } |  |  | 0.690 |
| walker |  | 3206 | 25 | Code::CodeKey { rung: Names, file: widgets/help.widget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.690 |
| walker |  | 3220 | 14 | Code::CodeKey { rung: Decl, file: widgets/help.widget.js, decl: 1, sub: 0, line: 5 } |  |  | 0.690 |
| walker |  | 3231 | 11 | Code::CodeKey { rung: Body, file: widgets/help.widget.js, decl: 2, sub: 0, line: 6 } |  |  | 0.690 |
| ns | 3375 |  | 205 | src/screen.js: imports and the mode -> grid-layout table | 3.2 |  | 0.673 |
| walker |  | 3391 | 160 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.690 |
| walker |  | 3536 | 145 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.690 |
| ns | 3573 |  | 198 | src/screen.js: complete method roster of the `screen` class | 3.3 |  | 0.684 |
| walker |  | 3632 | 96 | Plaintext::DeclSurface { file: dockerRunScript.sh } |  |  | 0.684 |
| walker |  | 3645 | 13 | Plaintext::Whole { file: dockerRunScript.sh } |  |  | 0.684 |
| walker |  | 3696 | 51 | Code::CodeKey { rung: Names, file: src/themes/styles.js, decl: 0, sub: 0, line: 0 } |  |  | 0.684 |
| ns | 3770 |  | 197 | src/screen.js: init() — the full boot sequence | 3.4 | 3.3 | 0.664 |
| ns | 3900 |  | 130 | src/screen.js: constructor state fields | 3.5 | 3.3 | 0.651 |
| walker |  | 3902 | 206 | Code::CodeKey { rung: Decl, file: src/themes/styles.js, decl: 2, sub: 0, line: 28 } |  |  | 0.651 |
| ns | 4024 |  | 124 | src/screen.js: initScreen() — blessed screen and the 12x12 grid | 3.6 | 3.3 | 0.641 |
| walker |  | 4109 | 207 | Code::CodeKey { rung: Decl, file: src/themes/styles.js, decl: 1, sub: 0, line: 1 } |  |  | 0.641 |
| walker |  | 4134 | 25 | Code::CodeKey { rung: Names, file: widgets/containers/containerList.widget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.641 |
| ns | 4286 |  | 262 | src/screen.js: initHooks() and initWidgets() — instantiation and layout gating | 3.7 | 3.3 | 0.620 |
| walker |  | 4322 | 188 | Code::CodeKey { rung: Decl, file: widgets/containers/containerList.widget.js, decl: 1, sub: 0, line: 10 } |  |  | 0.621 |
| walker |  | 4331 | 9 | Code::CodeKey { rung: Body, file: widgets/containers/containerList.widget.js, decl: 3, sub: 0, line: 18 } |  |  | 0.621 |
| walker |  | 4356 | 25 | Code::CodeKey { rung: Names, file: widgets/images/imageList.widget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.621 |
| walker |  | 4491 | 135 | Code::CodeKey { rung: Decl, file: widgets/images/imageList.widget.js, decl: 1, sub: 0, line: 5 } |  |  | 0.621 |
| walker |  | 4500 | 9 | Code::CodeKey { rung: Body, file: widgets/images/imageList.widget.js, decl: 3, sub: 0, line: 11 } |  |  | 0.621 |
| walker |  | 4525 | 25 | Code::CodeKey { rung: Names, file: widgets/services/servicesList.widget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.621 |
| ns | 4580 |  | 294 | src/screen.js: toggleMode() and registerEvents() — the global q/v keys and focus borders | 3.8 | 3.3 | 0.600 |
| walker |  | 4657 | 132 | Code::CodeKey { rung: Decl, file: widgets/services/servicesList.widget.js, decl: 1, sub: 0, line: 5 } |  |  | 0.600 |
| walker |  | 4666 | 9 | Code::CodeKey { rung: Body, file: widgets/services/servicesList.widget.js, decl: 3, sub: 0, line: 11 } |  |  | 0.600 |
| ns | 4668 |  | 88 | src/assetsLoader.js: the glob patterns that define the plugin conventions | 3.9 |  | 0.594 |
| walker |  | 4692 | 26 | Code::CodeKey { rung: Names, file: src/widgetsTemplates/help.widget.template.js, decl: 0, sub: 0, line: 0 } |  |  | 0.594 |
| walker |  | 4816 | 124 | Code::CodeKey { rung: Decl, file: src/widgetsTemplates/help.widget.template.js, decl: 1, sub: 0, line: 5 } |  |  | 0.594 |
| walker |  | 4824 | 8 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/help.widget.template.js, decl: 8, sub: 0, line: 130 } |  |  | 0.594 |
| walker |  | 4850 | 26 | Code::CodeKey { rung: Names, file: widgets/containers/containerUtilization.widget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.594 |
| ns | 4871 |  | 203 | src/assetsLoader.js: the loader body and the asset naming rule | 3.10 | 3.9 | 0.581 |
| walker |  | 4909 | 59 | Code::CodeKey { rung: Decl, file: widgets/containers/containerUtilization.widget.js, decl: 1, sub: 0, line: 4 } |  |  | 0.581 |
| walker |  | 4935 | 26 | Code::CodeKey { rung: Names, file: widgets/images/imageUtilization.widget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.581 |
| walker |  | 4994 | 59 | Code::CodeKey { rung: Decl, file: widgets/images/imageUtilization.widget.js, decl: 1, sub: 0, line: 5 } |  |  | 0.581 |
| walker |  | 5019 | 25 | Code::CodeKey { rung: Names, file: widgets/images/imageInfo.widget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.581 |
| walker |  | 5064 | 45 | Code::CodeKey { rung: Decl, file: widgets/images/imageInfo.widget.js, decl: 1, sub: 0, line: 5 } |  |  | 0.581 |
| walker |  | 5074 | 10 | Code::CodeKey { rung: Body, file: widgets/images/imageInfo.widget.js, decl: 2, sub: 0, line: 6 } |  |  | 0.581 |
| walker |  | 5100 | 26 | Code::CodeKey { rung: Names, file: widgets/services/servicesStatus.widget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.581 |
| ns | 5149 |  | 278 | src/grid.config.js: the containers-mode layout in full | 3.11 |  | 0.572 |
| walker |  | 5159 | 59 | Code::CodeKey { rung: Decl, file: widgets/services/servicesStatus.widget.js, decl: 1, sub: 0, line: 5 } |  |  | 0.572 |
| walker |  | 5185 | 26 | Code::CodeKey { rung: Names, file: widgets/services/servicesVsImages.widget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.572 |
| walker |  | 5244 | 59 | Code::CodeKey { rung: Decl, file: widgets/services/servicesVsImages.widget.js, decl: 1, sub: 0, line: 4 } |  |  | 0.572 |
| walker |  | 5269 | 25 | Code::CodeKey { rung: Names, file: widgets/services/servicesInfo.widget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.572 |
| walker |  | 5314 | 45 | Code::CodeKey { rung: Decl, file: widgets/services/servicesInfo.widget.js, decl: 1, sub: 0, line: 5 } |  |  | 0.572 |
| walker |  | 5324 | 10 | Code::CodeKey { rung: Body, file: widgets/services/servicesInfo.widget.js, decl: 2, sub: 0, line: 6 } |  |  | 0.572 |
| walker |  | 5349 | 25 | Code::CodeKey { rung: Names, file: widgets/services/servicesLogs.widget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.572 |
| walker |  | 5374 | 25 | Code::CodeKey { rung: Decl, file: widgets/services/servicesLogs.widget.js, decl: 1, sub: 0, line: 5 } |  |  | 0.572 |
| walker |  | 5384 | 10 | Code::CodeKey { rung: Body, file: widgets/services/servicesLogs.widget.js, decl: 2, sub: 0, line: 6 } |  |  | 0.572 |
| walker |  | 5398 | 14 | Code::CodeKey { rung: Body, file: widgets/services/servicesLogs.widget.js, decl: 3, sub: 0, line: 10 } |  |  | 0.572 |
| walker |  | 5431 | 33 | Code::CodeKey { rung: Doc, file: src/cli.js, decl: 4, sub: 0, line: 93 } |  |  | 0.572 |
| walker |  | 5445 | 14 | Code::CodeKey { rung: Names, file: src/themes/theme.selector.js, decl: 0, sub: 0, line: 0 } |  |  | 0.572 |
| walker |  | 5473 | 28 | Code::CodeKey { rung: Names, file: src/widgetsTemplates/list.widget.template.js, decl: 0, sub: 0, line: 0 } |  |  | 0.572 |
| ns | 5554 |  | 405 | src/grid.config.js: the services and images layouts plus the export block | 3.12 | 3.11 | 0.557 |
| walker |  | 5650 | 177 | Code::CodeKey { rung: Decl, file: src/widgetsTemplates/list.widget.template.js, decl: 1, sub: 0, line: 8 } |  |  | 0.558 |
| walker |  | 5676 | 26 | Code::CodeKey { rung: Names, file: src/widgetsTemplates/info.widget.template.js, decl: 0, sub: 0, line: 0 } |  |  | 0.558 |
| walker |  | 5788 | 112 | Code::CodeKey { rung: Decl, file: src/widgetsTemplates/info.widget.template.js, decl: 1, sub: 0, line: 6 } |  |  | 0.558 |
| walker |  | 5796 | 8 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/info.widget.template.js, decl: 7, sub: 0, line: 107 } |  |  | 0.558 |
| walker |  | 5822 | 26 | Code::CodeKey { rung: Names, file: src/widgetsTemplates/logs.widget.template.js, decl: 0, sub: 0, line: 0 } |  |  | 0.558 |
| ns | 5884 |  | 330 | src/dockerUtil.js: complete method roster of the dockerode wrapper | 4.1 |  | 0.560 |
| walker |  | 5913 | 91 | Code::CodeKey { rung: Decl, file: src/widgetsTemplates/logs.widget.template.js, decl: 1, sub: 0, line: 7 } |  |  | 0.561 |
| walker |  | 5924 | 11 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/logs.widget.template.js, decl: 6, sub: 0, line: 76 } |  |  | 0.561 |
| walker |  | 5935 | 11 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/logs.widget.template.js, decl: 7, sub: 0, line: 80 } |  |  | 0.561 |
| walker |  | 5963 | 28 | Code::CodeKey { rung: Names, file: src/widgetsTemplates/base.hook.template.js, decl: 0, sub: 0, line: 0 } |  |  | 0.561 |
| walker |  | 5992 | 29 | Code::CodeKey { rung: Decl, file: src/widgetsTemplates/base.hook.template.js, decl: 1, sub: 0, line: 6 } |  |  | 0.561 |
| walker |  | 6008 | 16 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/base.hook.template.js, decl: 3, sub: 0, line: 29 } |  |  | 0.561 |
| walker |  | 6038 | 30 | Code::CodeKey { rung: Names, file: widgets/containers/containerSortList.widget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.561 |
| walker |  | 6157 | 119 | Code::CodeKey { rung: Decl, file: widgets/containers/containerSortList.widget.js, decl: 1, sub: 0, line: 4 } |  |  | 0.561 |
| walker |  | 6161 | 4 | Code::CodeKey { rung: Decl, file: widgets/containers/containerSortList.widget.js, decl: 10, sub: 0, line: 99 } |  |  | 0.561 |
| ns | 6164 |  | 280 | src/dockerUtil.js: constructor — connection config and containerFilters parsing | 4.2 | 4.1 | 0.548 |
| walker |  | 6187 | 26 | Code::CodeKey { rung: Names, file: widgets/containers/containerStatus.widget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.548 |
| walker |  | 6246 | 59 | Code::CodeKey { rung: Decl, file: widgets/containers/containerStatus.widget.js, decl: 1, sub: 0, line: 5 } |  |  | 0.548 |
| walker |  | 6272 | 26 | Code::CodeKey { rung: Names, file: widgets/containers/containerVsImages.widget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.548 |
| ns | 6314 |  | 150 | src/dockerUtil.js: getInfo() — the host fields surfaced to the status widgets | 4.3 | 4.1 | 0.542 |
| walker |  | 6331 | 59 | Code::CodeKey { rung: Decl, file: widgets/containers/containerVsImages.widget.js, decl: 1, sub: 0, line: 4 } |  |  | 0.542 |
| walker |  | 6356 | 25 | Code::CodeKey { rung: Names, file: widgets/containers/containerInfo.widget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.542 |
| walker |  | 6401 | 45 | Code::CodeKey { rung: Decl, file: widgets/containers/containerInfo.widget.js, decl: 1, sub: 0, line: 5 } |  |  | 0.542 |
| walker |  | 6411 | 10 | Code::CodeKey { rung: Body, file: widgets/containers/containerInfo.widget.js, decl: 2, sub: 0, line: 6 } |  |  | 0.542 |
| ns | 6429 |  | 115 | src/dockerUtil.js: log streaming options for containers and services | 4.4 | 4.1 | 0.536 |
| walker |  | 6436 | 25 | Code::CodeKey { rung: Names, file: widgets/containers/containerLogs.widget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.536 |
| walker |  | 6461 | 25 | Code::CodeKey { rung: Decl, file: widgets/containers/containerLogs.widget.js, decl: 1, sub: 0, line: 5 } |  |  | 0.536 |
| walker |  | 6471 | 10 | Code::CodeKey { rung: Body, file: widgets/containers/containerLogs.widget.js, decl: 2, sub: 0, line: 6 } |  |  | 0.536 |
| walker |  | 6485 | 14 | Code::CodeKey { rung: Body, file: widgets/containers/containerLogs.widget.js, decl: 3, sub: 0, line: 10 } |  |  | 0.536 |
| walker |  | 6516 | 31 | Code::CodeKey { rung: Body, file: widgets/actionStatus.widget.js, decl: 3, sub: 0, line: 18 } |  |  | 0.536 |
| walker |  | 6531 | 15 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/logs.widget.template.js, decl: 5, sub: 0, line: 72 } |  |  | 0.536 |
| walker |  | 6546 | 15 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/info.widget.template.js, decl: 8, sub: 0, line: 111 } |  |  | 0.536 |
| walker |  | 6564 | 18 | Code::CodeKey { rung: Body, file: widgets/containers/containerInfo.widget.js, decl: 3, sub: 0, line: 10 } |  |  | 0.536 |
| walker |  | 6582 | 18 | Code::CodeKey { rung: Body, file: widgets/images/imageInfo.widget.js, decl: 3, sub: 0, line: 10 } |  |  | 0.536 |
| walker |  | 6600 | 18 | Code::CodeKey { rung: Body, file: widgets/services/servicesInfo.widget.js, decl: 3, sub: 0, line: 10 } |  |  | 0.536 |
| walker |  | 6615 | 15 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/logs.widget.template.js, decl: 8, sub: 0, line: 84 } |  |  | 0.536 |
| walker |  | 6630 | 15 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/help.widget.template.js, decl: 9, sub: 0, line: 134 } |  |  | 0.536 |
| ns | 6656 |  | 227 | src/baseWidget.js — the mixin every widget and hook extends | 5.1 |  | 0.524 |
| ns | 6888 |  | 232 | src/widgetsTemplates/base.hook.template.js — the hook base class | 5.2 |  | 0.515 |
| walker |  | 7009 | 379 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.530 |
| walker |  | 7025 | 16 | Code::CodeKey { rung: Body, file: widgets/services/servicesList.widget.js, decl: 6, sub: 0, line: 23 } |  |  | 0.530 |
| walker |  | 7041 | 16 | Code::CodeKey { rung: Body, file: widgets/images/imageList.widget.js, decl: 4, sub: 0, line: 15 } |  |  | 0.530 |
| walker |  | 7056 | 15 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/list.widget.template.js, decl: 7, sub: 0, line: 127 } |  |  | 0.531 |
| walker |  | 7072 | 16 | Code::CodeKey { rung: Body, file: widgets/services/servicesList.widget.js, decl: 7, sub: 0, line: 27 } |  |  | 0.531 |
| walker |  | 7089 | 17 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/info.widget.template.js, decl: 4, sub: 0, line: 64 } |  |  | 0.531 |
| walker |  | 7109 | 20 | Code::CodeKey { rung: Body, file: widgets/containers/containerInfo.widget.js, decl: 4, sub: 0, line: 14 } |  |  | 0.531 |
| ns | 7121 |  | 233 | src/widgetsTemplates/list.widget.template.js: complete method roster | 5.3 |  | 0.536 |
| walker |  | 7129 | 20 | Code::CodeKey { rung: Body, file: widgets/images/imageInfo.widget.js, decl: 4, sub: 0, line: 14 } |  |  | 0.536 |
| walker |  | 7149 | 20 | Code::CodeKey { rung: Body, file: widgets/services/servicesInfo.widget.js, decl: 4, sub: 0, line: 14 } |  |  | 0.536 |
| walker |  | 7164 | 15 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/list.widget.template.js, decl: 12, sub: 0, line: 147 } |  |  | 0.536 |
| walker |  | 7181 | 17 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/help.widget.template.js, decl: 4, sub: 0, line: 50 } |  |  | 0.536 |
| walker |  | 7198 | 17 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/info.widget.template.js, decl: 5, sub: 0, line: 68 } |  |  | 0.536 |
| walker |  | 7213 | 15 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/list.widget.template.js, decl: 13, sub: 0, line: 151 } |  |  | 0.536 |
| walker |  | 7230 | 17 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/help.widget.template.js, decl: 5, sub: 0, line: 54 } |  |  | 0.536 |
| walker |  | 7262 | 32 | Code::CodeKey { rung: Body, file: src/screen.js, decl: 11, sub: 0, line: 126 } |  |  | 0.538 |
| walker |  | 7278 | 16 | Code::CodeKey { rung: Body, file: widgets/containers/containerList.widget.js, decl: 7, sub: 0, line: 40 } |  |  | 0.538 |
| walker |  | 7295 | 17 | Code::CodeKey { rung: Body, file: widgets/services/servicesList.widget.js, decl: 5, sub: 0, line: 19 } |  |  | 0.538 |
| walker |  | 7313 | 18 | Code::CodeKey { rung: Body, file: widgets/images/imageList.widget.js, decl: 10, sub: 0, line: 123 } |  |  | 0.538 |
| walker |  | 7339 | 26 | Code::CodeKey { rung: Body, file: src/dockerUtil.js, decl: 14, sub: 0, line: 165 } |  |  | 0.539 |
| walker |  | 7371 | 32 | Code::CodeKey { rung: Body, file: src/screen.js, decl: 12, sub: 0, line: 132 } |  |  | 0.540 |
| walker |  | 7387 | 16 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/list.widget.template.js, decl: 8, sub: 0, line: 131 } |  |  | 0.540 |
| ns | 7409 |  | 288 | src/widgetsTemplates/info.widget.template.js and logs.widget.template.js: method rosters | 5.4 |  | 0.543 |
| walker |  | 7436 | 49 | Code::CodeKey { rung: Body, file: hooks/shell.hook.js, decl: 4, sub: 0, line: 59 } |  |  | 0.543 |
| walker |  | 7462 | 26 | Code::CodeKey { rung: Body, file: src/dockerUtil.js, decl: 15, sub: 0, line: 170 } |  |  | 0.544 |
| walker |  | 7478 | 16 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/list.widget.template.js, decl: 9, sub: 0, line: 135 } |  |  | 0.544 |
| walker |  | 7494 | 16 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/list.widget.template.js, decl: 10, sub: 0, line: 139 } |  |  | 0.544 |
| walker |  | 7520 | 26 | Code::CodeKey { rung: Body, file: src/dockerUtil.js, decl: 16, sub: 0, line: 175 } |  |  | 0.545 |
| walker |  | 7540 | 20 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/info.widget.template.js, decl: 9, sub: 0, line: 115 } |  |  | 0.549 |
| ns | 7570 |  | 161 | src/widgetsTemplates/help.widget.template.js: method roster around the help text | 5.5 |  | 0.548 |
| walker |  | 7593 | 53 | Code::CodeKey { rung: Body, file: hooks/images.hook.js, decl: 3, sub: 0, line: 27 } |  |  | 0.548 |
| walker |  | 7609 | 16 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/list.widget.template.js, decl: 11, sub: 0, line: 143 } |  |  | 0.548 |
| walker |  | 7629 | 20 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/help.widget.template.js, decl: 10, sub: 0, line: 138 } |  |  | 0.548 |
| walker |  | 7682 | 53 | Code::CodeKey { rung: Body, file: hooks/services.hook.js, decl: 6, sub: 0, line: 86 } |  |  | 0.548 |
| walker |  | 7709 | 27 | Code::CodeKey { rung: Body, file: src/dockerUtil.js, decl: 12, sub: 0, line: 155 } |  |  | 0.549 |
| walker |  | 7745 | 36 | Code::CodeKey { rung: Body, file: src/screen.js, decl: 10, sub: 0, line: 120 } |  |  | 0.550 |
| walker |  | 7766 | 21 | Code::CodeKey { rung: Body, file: widgets/services/servicesList.widget.js, decl: 4, sub: 0, line: 15 } |  |  | 0.550 |
| walker |  | 7786 | 20 | Code::CodeKey { rung: Body, file: widgets/containers/containerList.widget.js, decl: 9, sub: 0, line: 75 } |  |  | 0.550 |
| ns | 7919 |  | 349 | hooks/: complete method rosters of all four hooks | 6.1 |  | 0.560 |
| ns | 8128 |  | 209 | hooks/containers.hook.js: the toolbar key dispatch inside init() | 6.2 | 6.1 | 0.552 |
| walker |  | 8217 | 431 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.552 |
| walker |  | 8363 | 146 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.552 |
| walker |  | 8416 | 53 | Code::CodeKey { rung: Body, file: hooks/containers.hook.js, decl: 9, sub: 0, line: 168 } |  |  | 0.552 |
| walker |  | 8443 | 27 | Code::CodeKey { rung: Body, file: src/dockerUtil.js, decl: 13, sub: 0, line: 160 } |  |  | 0.553 |
| walker |  | 8480 | 37 | Code::CodeKey { rung: Body, file: src/screen.js, decl: 9, sub: 0, line: 114 } |  |  | 0.554 |
| ns | 8482 |  | 354 | hooks/shell.hook.js: openShell() plus dockerRunScript.sh's docker exec line | 6.3 | 6.1 | 0.544 |
| walker |  | 8522 | 42 | Code::CodeKey { rung: Doc, file: widgets/containers/containerList.widget.js, decl: 13, sub: 0, line: 169 } |  |  | 0.544 |
| walker |  | 8564 | 42 | Code::CodeKey { rung: Doc, file: widgets/images/imageList.widget.js, decl: 9, sub: 0, line: 119 } |  |  | 0.544 |
| ns | 8583 |  | 101 | src/enum.js in full — the ContainerState vocabulary | 7.1 |  | 0.540 |
| walker |  | 8606 | 42 | Code::CodeKey { rung: Doc, file: widgets/services/servicesList.widget.js, decl: 10, sub: 0, line: 87 } |  |  | 0.540 |
| walker |  | 8667 | 61 | Code::CodeKey { rung: Body, file: src/cli.js, decl: 2, sub: 0, line: 59 } |  |  | 0.541 |
| walker |  | 8680 | 13 | Fs::DirListing { dir: docs/src/assets } |  |  | 0.541 |
| walker |  | 8686 | 6 | Fs::DirListing { dir: docs/src/assets/css } |  |  | 0.541 |
| walker |  | 8781 | 95 | Code::CodeKey { rung: Body, file: index.js, decl: 8, sub: 0, line: 72 } |  |  | 0.542 |
| walker |  | 8803 | 22 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/list.widget.template.js, decl: 14, sub: 0, line: 155 } |  |  | 0.542 |
| walker |  | 8829 | 26 | Code::CodeKey { rung: Body, file: widgets/images/imageList.widget.js, decl: 2, sub: 0, line: 6 } |  |  | 0.542 |
| walker |  | 8855 | 26 | Code::CodeKey { rung: Body, file: widgets/services/servicesList.widget.js, decl: 2, sub: 0, line: 6 } |  |  | 0.542 |
| walker |  | 8882 | 27 | Code::CodeKey { rung: Body, file: widgets/images/imageList.widget.js, decl: 9, sub: 0, line: 119 } |  |  | 0.542 |
| ns | 8893 |  | 310 | src/themes/theme.selector.js in full and the dark/light style keys | 7.2 |  | 0.535 |
| walker |  | 8937 | 55 | Code::CodeKey { rung: Doc, file: widgets/images/imageList.widget.js, decl: 8, sub: 0, line: 96 } |  |  | 0.535 |
| walker |  | 8964 | 27 | Code::CodeKey { rung: Body, file: widgets/containers/containerList.widget.js, decl: 13, sub: 0, line: 169 } |  |  | 0.535 |
| walker |  | 9041 | 77 | Code::CodeKey { rung: Body, file: widgets/actionStatus.widget.js, decl: 2, sub: 0, line: 7 } |  |  | 0.535 |
| walker |  | 9068 | 27 | Code::CodeKey { rung: Body, file: widgets/services/servicesList.widget.js, decl: 10, sub: 0, line: 87 } |  |  | 0.535 |
| walker |  | 9119 | 51 | Code::CodeKey { rung: Body, file: src/screen.js, decl: 8, sub: 0, line: 107 } |  |  | 0.536 |
| walker |  | 9161 | 42 | Code::CodeKey { rung: Body, file: src/dockerUtil.js, decl: 17, sub: 0, line: 180 } |  |  | 0.536 |
| ns | 9179 |  | 286 | widgets/containers/containerList.widget.js: complete method roster | 7.3 |  | 0.538 |
| walker |  | 9225 | 64 | Code::CodeKey { rung: Doc, file: widgets/images/imageList.widget.js, decl: 7, sub: 0, line: 81 } |  |  | 0.538 |
| walker |  | 9277 | 52 | Code::CodeKey { rung: Body, file: widgets/services/servicesStatus.widget.js, decl: 3, sub: 0, line: 22 } |  |  | 0.538 |
| walker |  | 9322 | 45 | Code::CodeKey { rung: Body, file: widgets/containers/containerSortList.widget.js, decl: 5, sub: 0, line: 47 } |  |  | 0.538 |
| walker |  | 9364 | 42 | Code::CodeKey { rung: Body, file: src/dockerUtil.js, decl: 18, sub: 0, line: 187 } |  |  | 0.539 |
| ns | 9448 |  | 269 | widgets/toolbar.widget.js: the per-mode command extension map | 7.4 |  | 0.531 |
| walker |  | 9464 | 100 | Code::CodeKey { rung: Body, file: widgets/toolbar.widget.js, decl: 2, sub: 0, line: 8 } |  |  | 0.531 |
| walker |  | 9558 | 94 | Code::CodeKey { rung: Body, file: hooks/services.hook.js, decl: 4, sub: 0, line: 61 } |  |  | 0.531 |
| ns | 9596 |  | 148 | widgets/actionsMenu.widget.js: the `m` menu's action table | 7.5 |  | 0.528 |
| walker |  | 9612 | 54 | Code::CodeKey { rung: Body, file: src/screen.js, decl: 6, sub: 0, line: 79 } |  |  | 0.529 |
| ns | 9640 |  | 44 | Complete listings of .github/ and its subdirectories | 8.1 |  | 0.533 |
| walker |  | 9681 | 69 | Code::CodeKey { rung: Body, file: src/themes/theme.selector.js, decl: 1, sub: 0, line: 13 } |  |  | 0.540 |
| ns | 9739 |  | 99 | Complete listings of the docs/ Gatsby site | 8.2 |  | 0.550 |
| walker |  | 9781 | 100 | Code::CodeKey { rung: Body, file: widgets/searchInput.widget.js, decl: 7, sub: 0, line: 94 } |  |  | 0.550 |
| walker |  | 9847 | 66 | Json::Identity { file: docs/package.json } |  |  | 0.550 |
| walker |  | 9854 | 7 | Plaintext::Whole { file: .nvmrc } |  |  | 0.550 |
| walker |  | 9948 | 94 | Code::CodeKey { rung: Body, file: hooks/containers.hook.js, decl: 4, sub: 0, line: 70 } |  |  | 0.550 |
| ns | 9988 |  | 249 | Secondary ops config: codefresh.yml, devcontainer image, VS Code attach config, .nvmrc | 8.3 |  | 0.543 |
| walker |  | 9997 | 49 | Code::CodeKey { rung: Body, file: hooks/shell.hook.js, decl: 2, sub: 0, line: 9 } |  |  | 0.544 |

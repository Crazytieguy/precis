Score(3000)=0.709 I=0.919 C=0.547 ns_rows≤3K=22/55 grid(1000/1442/2080/3000/4327/6240/9000)=0.944/0.854/0.790/0.709/0.621/0.557/0.545

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
| ns | 276 |  | 109 | Complete listings of the four runtime code directories: src/, hooks/, lib/, widgets/ | 1.3 |  | 0.624 |
| walker |  | 296 | 66 | Json::Identity { file: package.json } |  |  | 0.816 |
| walker |  | 315 | 19 | Fs::DirListing { dir: widgets/images } |  |  | 0.817 |
| ns | 371 |  | 95 | Complete listings of the per-mode widget directories: widgets/containers, widgets/images, widgets/services | 1.4 |  | 0.705 |
| walker |  | 398 | 83 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.706 |
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
| walker |  | 1698 | 10 | Code::CodeKey { rung: Names, file: lib/node.version.js, decl: 0, sub: 0, line: 0 } |  |  | 0.939 |
| walker |  | 1715 | 17 | Code::CodeKey { rung: Decl, file: lib/node.version.js, decl: 1, sub: 0, line: 1 } |  |  | 0.939 |
| walker |  | 1725 | 10 | Code::CodeKey { rung: Names, file: src/enum.js, decl: 0, sub: 0, line: 0 } |  |  | 0.939 |
| ns | 1737 |  | 323 | The in-app keybinding table (src/widgetsTemplates/help.widget.template.js:102-125) | 2.1 |  | 0.866 |
| walker |  | 1816 | 91 | Code::CodeKey { rung: Decl, file: src/enum.js, decl: 1, sub: 0, line: 1 } |  |  | 0.867 |
| walker |  | 1836 | 20 | Code::CodeKey { rung: Names, file: src/dockerUtil.js, decl: 0, sub: 0, line: 0 } |  |  | 0.867 |
| walker |  | 1856 | 20 | Code::CodeKey { rung: Names, file: src/screen.js, decl: 0, sub: 0, line: 0 } |  |  | 0.867 |
| ns | 1857 |  | 120 | src/cli.js: the complete flag roster (all eight option names) | 2.2 |  | 0.841 |
| walker |  | 2028 | 172 | Code::CodeKey { rung: Decl, file: src/screen.js, decl: 1, sub: 0, line: 18 } |  |  | 0.844 |
| walker |  | 2037 | 9 | Code::CodeKey { rung: Body, file: src/screen.js, decl: 16, sub: 0, line: 192 } |  |  | 0.845 |
| ns | 2043 |  | 186 | src/cli.js: the four docker-connection options in full (socketPath, host, port, protocol) | 2.3 | 2.2 | 0.790 |
| ns | 2206 |  | 163 | src/cli.js: the four behavioural options in full (help, version, containerFilters, theme) | 2.4 | 2.2 | 0.744 |
| ns | 2269 |  | 63 | src/cli.js: every prototype method and the singleton export | 2.5 |  | 0.732 |
| walker |  | 2334 | 297 | Code::CodeKey { rung: Decl, file: src/dockerUtil.js, decl: 1, sub: 0, line: 5 } |  |  | 0.734 |
| ns | 2339 |  | 70 | README: install and launch commands | 2.6 |  | 0.723 |
| walker |  | 2356 | 22 | Code::CodeKey { rung: Names, file: src/assetsLoader.js, decl: 0, sub: 0, line: 0 } |  |  | 0.723 |
| walker |  | 2398 | 42 | Code::CodeKey { rung: Decl, file: src/assetsLoader.js, decl: 1, sub: 0, line: 10 } |  |  | 0.723 |
| walker |  | 2421 | 23 | Code::CodeKey { rung: Names, file: hooks/containers.hook.js, decl: 0, sub: 0, line: 0 } |  |  | 0.723 |
| ns | 2455 |  | 116 | README: --containerFilters semantics | 2.7 |  | 0.716 |
| walker |  | 2521 | 100 | Code::CodeKey { rung: Decl, file: hooks/containers.hook.js, decl: 1, sub: 0, line: 5 } |  |  | 0.717 |
| walker |  | 2544 | 23 | Code::CodeKey { rung: Names, file: hooks/images.hook.js, decl: 0, sub: 0, line: 0 } |  |  | 0.717 |
| walker |  | 2593 | 49 | Code::CodeKey { rung: Decl, file: hooks/images.hook.js, decl: 1, sub: 0, line: 5 } |  |  | 0.717 |
| walker |  | 2616 | 23 | Code::CodeKey { rung: Names, file: hooks/services.hook.js, decl: 0, sub: 0, line: 0 } |  |  | 0.717 |
| ns | 2625 |  | 170 | Dockerfile in full | 2.8 |  | 0.730 |
| walker |  | 2680 | 64 | Code::CodeKey { rung: Decl, file: hooks/services.hook.js, decl: 1, sub: 0, line: 5 } |  |  | 0.731 |
| walker |  | 2695 | 15 | Code::CodeKey { rung: Body, file: src/assetsLoader.js, decl: 4, sub: 0, line: 39 } |  |  | 0.731 |
| walker |  | 2719 | 24 | Code::CodeKey { rung: Names, file: hooks/shell.hook.js, decl: 0, sub: 0, line: 0 } |  |  | 0.731 |
| ns | 2724 |  | 99 | README: running and building the docker image | 2.9 |  | 0.719 |
| walker |  | 2759 | 40 | Code::CodeKey { rung: Decl, file: hooks/shell.hook.js, decl: 1, sub: 0, line: 8 } |  |  | 0.720 |
| walker |  | 2771 | 12 | Code::CodeKey { rung: Names, file: lib/modes.js, decl: 0, sub: 0, line: 0 } |  |  | 0.720 |
| walker |  | 2804 | 33 | Code::CodeKey { rung: Decl, file: lib/modes.js, decl: 1, sub: 0, line: 3 } |  |  | 0.721 |
| walker |  | 2829 | 25 | Code::CodeKey { rung: Names, file: widgets/help.widget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.721 |
| walker |  | 2843 | 14 | Code::CodeKey { rung: Decl, file: widgets/help.widget.js, decl: 1, sub: 0, line: 5 } |  |  | 0.721 |
| walker |  | 2854 | 11 | Code::CodeKey { rung: Body, file: widgets/help.widget.js, decl: 2, sub: 0, line: 6 } |  |  | 0.721 |
| ns | 2876 |  | 152 | .github/workflows/main.yml: the lint job | 2.10 |  | 0.697 |
| walker |  | 2918 | 64 | Code::CodeKey { rung: Names, file: src/cli.js, decl: 0, sub: 0, line: 0 } |  |  | 0.705 |
| walker |  | 2931 | 13 | Code::CodeKey { rung: Body, file: src/cli.js, decl: 4, sub: 0, line: 93 } |  |  | 0.709 |
| walker |  | 2957 | 26 | Code::CodeKey { rung: Names, file: widgets/actionsMenu.widget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.709 |
| walker |  | 3063 | 106 | Code::CodeKey { rung: Decl, file: widgets/actionsMenu.widget.js, decl: 1, sub: 0, line: 5 } |  |  | 0.709 |
| walker |  | 3071 | 8 | Code::CodeKey { rung: Body, file: widgets/actionsMenu.widget.js, decl: 9, sub: 0, line: 148 } |  |  | 0.709 |
| walker |  | 3076 | 5 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.709 |
| ns | 3087 |  | 211 | .github/workflows/main.yml: the semantic-release job | 2.11 |  | 0.689 |
| walker |  | 3104 | 28 | Code::CodeKey { rung: Names, file: widgets/actionStatus.widget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.689 |
| ns | 3170 |  | 83 | lib/modes.js and lib/node.version.js in full | 3.1 |  | 0.691 |
| walker |  | 3174 | 70 | Code::CodeKey { rung: Decl, file: widgets/actionStatus.widget.js, decl: 1, sub: 0, line: 6 } |  |  | 0.691 |
| walker |  | 3182 | 8 | Code::CodeKey { rung: Body, file: widgets/actionStatus.widget.js, decl: 4, sub: 0, line: 24 } |  |  | 0.691 |
| walker |  | 3210 | 28 | Code::CodeKey { rung: Names, file: widgets/searchInput.widget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.691 |
| walker |  | 3292 | 82 | Code::CodeKey { rung: Decl, file: widgets/searchInput.widget.js, decl: 1, sub: 0, line: 9 } |  |  | 0.691 |
| walker |  | 3299 | 7 | Code::CodeKey { rung: Body, file: widgets/searchInput.widget.js, decl: 3, sub: 0, line: 24 } |  |  | 0.691 |
| walker |  | 3307 | 8 | Code::CodeKey { rung: Body, file: widgets/searchInput.widget.js, decl: 4, sub: 0, line: 28 } |  |  | 0.691 |
| walker |  | 3335 | 28 | Code::CodeKey { rung: Names, file: widgets/toolbar.widget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.691 |
| ns | 3375 |  | 205 | src/screen.js: imports and the mode -> grid-layout table | 3.2 |  | 0.674 |
| walker |  | 3394 | 59 | Code::CodeKey { rung: Decl, file: widgets/toolbar.widget.js, decl: 1, sub: 0, line: 7 } |  |  | 0.674 |
| walker |  | 3402 | 8 | Code::CodeKey { rung: Body, file: widgets/toolbar.widget.js, decl: 3, sub: 0, line: 21 } |  |  | 0.674 |
| walker |  | 3411 | 9 | Code::CodeKey { rung: Body, file: widgets/toolbar.widget.js, decl: 4, sub: 0, line: 25 } |  |  | 0.674 |
| walker |  | 3571 | 160 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.691 |
| ns | 3573 |  | 198 | src/screen.js: complete method roster of the `screen` class | 3.3 |  | 0.685 |
| walker |  | 3716 | 145 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.685 |
| ns | 3770 |  | 197 | src/screen.js: init() — the full boot sequence | 3.4 | 3.3 | 0.664 |
| walker |  | 3812 | 96 | Plaintext::DeclSurface { file: dockerRunScript.sh } |  |  | 0.664 |
| walker |  | 3825 | 13 | Plaintext::Whole { file: dockerRunScript.sh } |  |  | 0.664 |
| walker |  | 3876 | 51 | Code::CodeKey { rung: Names, file: src/themes/styles.js, decl: 0, sub: 0, line: 0 } |  |  | 0.665 |
| ns | 3900 |  | 130 | src/screen.js: constructor state fields | 3.5 | 3.3 | 0.651 |
| ns | 4024 |  | 124 | src/screen.js: initScreen() — blessed screen and the 12x12 grid | 3.6 | 3.3 | 0.642 |
| walker |  | 4082 | 206 | Code::CodeKey { rung: Decl, file: src/themes/styles.js, decl: 2, sub: 0, line: 28 } |  |  | 0.642 |
| ns | 4286 |  | 262 | src/screen.js: initHooks() and initWidgets() — instantiation and layout gating | 3.7 | 3.3 | 0.621 |
| walker |  | 4289 | 207 | Code::CodeKey { rung: Decl, file: src/themes/styles.js, decl: 1, sub: 0, line: 1 } |  |  | 0.621 |
| walker |  | 4310 | 21 | Code::CodeKey { rung: Body, file: src/assetsLoader.js, decl: 2, sub: 0, line: 11 } |  |  | 0.621 |
| walker |  | 4329 | 19 | Code::CodeKey { rung: Names, file: src/baseWidget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.621 |
| walker |  | 4355 | 26 | Code::CodeKey { rung: Body, file: src/dockerUtil.js, decl: 14, sub: 0, line: 165 } |  |  | 0.621 |
| walker |  | 4381 | 26 | Code::CodeKey { rung: Body, file: src/dockerUtil.js, decl: 15, sub: 0, line: 170 } |  |  | 0.622 |
| walker |  | 4407 | 26 | Code::CodeKey { rung: Body, file: src/dockerUtil.js, decl: 16, sub: 0, line: 175 } |  |  | 0.622 |
| walker |  | 4432 | 25 | Code::CodeKey { rung: Names, file: widgets/containers/containerInfo.widget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.622 |
| walker |  | 4477 | 45 | Code::CodeKey { rung: Decl, file: widgets/containers/containerInfo.widget.js, decl: 1, sub: 0, line: 5 } |  |  | 0.622 |
| walker |  | 4487 | 10 | Code::CodeKey { rung: Body, file: widgets/containers/containerInfo.widget.js, decl: 2, sub: 0, line: 6 } |  |  | 0.622 |
| walker |  | 4512 | 25 | Code::CodeKey { rung: Names, file: widgets/containers/containerList.widget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.622 |
| ns | 4580 |  | 294 | src/screen.js: toggleMode() and registerEvents() — the global q/v keys and focus borders | 3.8 | 3.3 | 0.600 |
| ns | 4668 |  | 88 | src/assetsLoader.js: the glob patterns that define the plugin conventions | 3.9 |  | 0.595 |
| walker |  | 4700 | 188 | Code::CodeKey { rung: Decl, file: widgets/containers/containerList.widget.js, decl: 1, sub: 0, line: 10 } |  |  | 0.595 |
| walker |  | 4709 | 9 | Code::CodeKey { rung: Body, file: widgets/containers/containerList.widget.js, decl: 3, sub: 0, line: 18 } |  |  | 0.595 |
| walker |  | 4734 | 25 | Code::CodeKey { rung: Names, file: widgets/containers/containerLogs.widget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.595 |
| walker |  | 4759 | 25 | Code::CodeKey { rung: Decl, file: widgets/containers/containerLogs.widget.js, decl: 1, sub: 0, line: 5 } |  |  | 0.595 |
| walker |  | 4769 | 10 | Code::CodeKey { rung: Body, file: widgets/containers/containerLogs.widget.js, decl: 2, sub: 0, line: 6 } |  |  | 0.595 |
| walker |  | 4783 | 14 | Code::CodeKey { rung: Body, file: widgets/containers/containerLogs.widget.js, decl: 3, sub: 0, line: 10 } |  |  | 0.595 |
| walker |  | 4808 | 25 | Code::CodeKey { rung: Names, file: widgets/images/imageInfo.widget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.595 |
| walker |  | 4853 | 45 | Code::CodeKey { rung: Decl, file: widgets/images/imageInfo.widget.js, decl: 1, sub: 0, line: 5 } |  |  | 0.595 |
| walker |  | 4863 | 10 | Code::CodeKey { rung: Body, file: widgets/images/imageInfo.widget.js, decl: 2, sub: 0, line: 6 } |  |  | 0.595 |
| ns | 4871 |  | 203 | src/assetsLoader.js: the loader body and the asset naming rule | 3.10 | 3.9 | 0.587 |
| walker |  | 4888 | 25 | Code::CodeKey { rung: Names, file: widgets/images/imageList.widget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.587 |
| walker |  | 5023 | 135 | Code::CodeKey { rung: Decl, file: widgets/images/imageList.widget.js, decl: 1, sub: 0, line: 5 } |  |  | 0.587 |
| walker |  | 5032 | 9 | Code::CodeKey { rung: Body, file: widgets/images/imageList.widget.js, decl: 3, sub: 0, line: 11 } |  |  | 0.587 |
| walker |  | 5057 | 25 | Code::CodeKey { rung: Names, file: widgets/services/servicesInfo.widget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.587 |
| walker |  | 5102 | 45 | Code::CodeKey { rung: Decl, file: widgets/services/servicesInfo.widget.js, decl: 1, sub: 0, line: 5 } |  |  | 0.587 |
| walker |  | 5112 | 10 | Code::CodeKey { rung: Body, file: widgets/services/servicesInfo.widget.js, decl: 2, sub: 0, line: 6 } |  |  | 0.587 |
| walker |  | 5137 | 25 | Code::CodeKey { rung: Names, file: widgets/services/servicesList.widget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.587 |
| ns | 5149 |  | 278 | src/grid.config.js: the containers-mode layout in full | 3.11 |  | 0.578 |
| walker |  | 5269 | 132 | Code::CodeKey { rung: Decl, file: widgets/services/servicesList.widget.js, decl: 1, sub: 0, line: 5 } |  |  | 0.578 |
| walker |  | 5278 | 9 | Code::CodeKey { rung: Body, file: widgets/services/servicesList.widget.js, decl: 3, sub: 0, line: 11 } |  |  | 0.578 |
| walker |  | 5303 | 25 | Code::CodeKey { rung: Names, file: widgets/services/servicesLogs.widget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.578 |
| walker |  | 5328 | 25 | Code::CodeKey { rung: Decl, file: widgets/services/servicesLogs.widget.js, decl: 1, sub: 0, line: 5 } |  |  | 0.578 |
| walker |  | 5338 | 10 | Code::CodeKey { rung: Body, file: widgets/services/servicesLogs.widget.js, decl: 2, sub: 0, line: 6 } |  |  | 0.578 |
| walker |  | 5352 | 14 | Code::CodeKey { rung: Body, file: widgets/services/servicesLogs.widget.js, decl: 3, sub: 0, line: 10 } |  |  | 0.578 |
| walker |  | 5368 | 16 | Code::CodeKey { rung: Body, file: widgets/containers/containerList.widget.js, decl: 7, sub: 0, line: 40 } |  |  | 0.578 |
| walker |  | 5384 | 16 | Code::CodeKey { rung: Body, file: widgets/images/imageList.widget.js, decl: 4, sub: 0, line: 15 } |  |  | 0.578 |
| walker |  | 5400 | 16 | Code::CodeKey { rung: Body, file: widgets/services/servicesList.widget.js, decl: 6, sub: 0, line: 23 } |  |  | 0.578 |
| walker |  | 5416 | 16 | Code::CodeKey { rung: Body, file: widgets/services/servicesList.widget.js, decl: 7, sub: 0, line: 27 } |  |  | 0.578 |
| walker |  | 5447 | 31 | Code::CodeKey { rung: Body, file: widgets/actionStatus.widget.js, decl: 3, sub: 0, line: 18 } |  |  | 0.578 |
| walker |  | 5473 | 26 | Code::CodeKey { rung: Names, file: src/widgetsTemplates/help.widget.template.js, decl: 0, sub: 0, line: 0 } |  |  | 0.578 |
| ns | 5554 |  | 405 | src/grid.config.js: the services and images layouts plus the export block | 3.12 | 3.11 | 0.563 |
| walker |  | 5597 | 124 | Code::CodeKey { rung: Decl, file: src/widgetsTemplates/help.widget.template.js, decl: 1, sub: 0, line: 5 } |  |  | 0.564 |
| walker |  | 5605 | 8 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/help.widget.template.js, decl: 8, sub: 0, line: 130 } |  |  | 0.564 |
| walker |  | 5620 | 15 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/help.widget.template.js, decl: 9, sub: 0, line: 134 } |  |  | 0.564 |
| walker |  | 5646 | 26 | Code::CodeKey { rung: Names, file: src/widgetsTemplates/info.widget.template.js, decl: 0, sub: 0, line: 0 } |  |  | 0.564 |
| walker |  | 5758 | 112 | Code::CodeKey { rung: Decl, file: src/widgetsTemplates/info.widget.template.js, decl: 1, sub: 0, line: 6 } |  |  | 0.564 |
| walker |  | 5766 | 8 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/info.widget.template.js, decl: 7, sub: 0, line: 107 } |  |  | 0.564 |
| walker |  | 5781 | 15 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/info.widget.template.js, decl: 8, sub: 0, line: 111 } |  |  | 0.564 |
| walker |  | 5807 | 26 | Code::CodeKey { rung: Names, file: src/widgetsTemplates/logs.widget.template.js, decl: 0, sub: 0, line: 0 } |  |  | 0.564 |
| ns | 5884 |  | 330 | src/dockerUtil.js: complete method roster of the dockerode wrapper | 4.1 |  | 0.569 |
| walker |  | 5898 | 91 | Code::CodeKey { rung: Decl, file: src/widgetsTemplates/logs.widget.template.js, decl: 1, sub: 0, line: 7 } |  |  | 0.570 |
| walker |  | 5909 | 11 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/logs.widget.template.js, decl: 6, sub: 0, line: 76 } |  |  | 0.570 |
| walker |  | 5920 | 11 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/logs.widget.template.js, decl: 7, sub: 0, line: 80 } |  |  | 0.570 |
| walker |  | 5935 | 15 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/logs.widget.template.js, decl: 5, sub: 0, line: 72 } |  |  | 0.570 |
| walker |  | 5961 | 26 | Code::CodeKey { rung: Names, file: widgets/containers/containerStatus.widget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.570 |
| walker |  | 6020 | 59 | Code::CodeKey { rung: Decl, file: widgets/containers/containerStatus.widget.js, decl: 1, sub: 0, line: 5 } |  |  | 0.570 |
| walker |  | 6046 | 26 | Code::CodeKey { rung: Names, file: widgets/containers/containerUtilization.widget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.570 |
| walker |  | 6105 | 59 | Code::CodeKey { rung: Decl, file: widgets/containers/containerUtilization.widget.js, decl: 1, sub: 0, line: 4 } |  |  | 0.570 |
| walker |  | 6131 | 26 | Code::CodeKey { rung: Names, file: widgets/containers/containerVsImages.widget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.570 |
| ns | 6164 |  | 280 | src/dockerUtil.js: constructor — connection config and containerFilters parsing | 4.2 | 4.1 | 0.557 |
| walker |  | 6190 | 59 | Code::CodeKey { rung: Decl, file: widgets/containers/containerVsImages.widget.js, decl: 1, sub: 0, line: 4 } |  |  | 0.557 |
| walker |  | 6216 | 26 | Code::CodeKey { rung: Names, file: widgets/images/imageUtilization.widget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.557 |
| walker |  | 6275 | 59 | Code::CodeKey { rung: Decl, file: widgets/images/imageUtilization.widget.js, decl: 1, sub: 0, line: 5 } |  |  | 0.557 |
| walker |  | 6301 | 26 | Code::CodeKey { rung: Names, file: widgets/services/servicesStatus.widget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.557 |
| ns | 6314 |  | 150 | src/dockerUtil.js: getInfo() — the host fields surfaced to the status widgets | 4.3 | 4.1 | 0.551 |
| walker |  | 6360 | 59 | Code::CodeKey { rung: Decl, file: widgets/services/servicesStatus.widget.js, decl: 1, sub: 0, line: 5 } |  |  | 0.551 |
| walker |  | 6386 | 26 | Code::CodeKey { rung: Names, file: widgets/services/servicesVsImages.widget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.551 |
| ns | 6429 |  | 115 | src/dockerUtil.js: log streaming options for containers and services | 4.4 | 4.1 | 0.544 |
| walker |  | 6445 | 59 | Code::CodeKey { rung: Decl, file: widgets/services/servicesVsImages.widget.js, decl: 1, sub: 0, line: 4 } |  |  | 0.544 |
| walker |  | 6477 | 32 | Code::CodeKey { rung: Body, file: src/screen.js, decl: 11, sub: 0, line: 126 } |  |  | 0.546 |
| walker |  | 6509 | 32 | Code::CodeKey { rung: Body, file: src/screen.js, decl: 12, sub: 0, line: 132 } |  |  | 0.547 |
| walker |  | 6526 | 17 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/help.widget.template.js, decl: 4, sub: 0, line: 50 } |  |  | 0.547 |
| walker |  | 6543 | 17 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/info.widget.template.js, decl: 4, sub: 0, line: 64 } |  |  | 0.547 |
| walker |  | 6576 | 33 | Code::CodeKey { rung: Doc, file: src/cli.js, decl: 4, sub: 0, line: 93 } |  |  | 0.547 |
| walker |  | 6590 | 14 | Code::CodeKey { rung: Names, file: src/themes/theme.selector.js, decl: 0, sub: 0, line: 0 } |  |  | 0.547 |
| walker |  | 6618 | 28 | Code::CodeKey { rung: Names, file: src/widgetsTemplates/base.hook.template.js, decl: 0, sub: 0, line: 0 } |  |  | 0.547 |
| walker |  | 6647 | 29 | Code::CodeKey { rung: Decl, file: src/widgetsTemplates/base.hook.template.js, decl: 1, sub: 0, line: 6 } |  |  | 0.547 |
| ns | 6656 |  | 227 | src/baseWidget.js — the mixin every widget and hook extends | 5.1 |  | 0.535 |
| walker |  | 6663 | 16 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/base.hook.template.js, decl: 3, sub: 0, line: 29 } |  |  | 0.535 |
| walker |  | 6691 | 28 | Code::CodeKey { rung: Names, file: src/widgetsTemplates/list.widget.template.js, decl: 0, sub: 0, line: 0 } |  |  | 0.535 |
| walker |  | 6868 | 177 | Code::CodeKey { rung: Decl, file: src/widgetsTemplates/list.widget.template.js, decl: 1, sub: 0, line: 8 } |  |  | 0.536 |
| walker |  | 6883 | 15 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/list.widget.template.js, decl: 7, sub: 0, line: 127 } |  |  | 0.536 |
| ns | 6888 |  | 232 | src/widgetsTemplates/base.hook.template.js — the hook base class | 5.2 |  | 0.527 |
| walker |  | 6898 | 15 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/list.widget.template.js, decl: 12, sub: 0, line: 147 } |  |  | 0.527 |
| walker |  | 6913 | 15 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/list.widget.template.js, decl: 13, sub: 0, line: 151 } |  |  | 0.527 |
| walker |  | 6931 | 18 | Code::CodeKey { rung: Body, file: widgets/containers/containerInfo.widget.js, decl: 3, sub: 0, line: 10 } |  |  | 0.527 |
| walker |  | 6949 | 18 | Code::CodeKey { rung: Body, file: widgets/images/imageInfo.widget.js, decl: 3, sub: 0, line: 10 } |  |  | 0.527 |
| walker |  | 6967 | 18 | Code::CodeKey { rung: Body, file: widgets/images/imageList.widget.js, decl: 10, sub: 0, line: 123 } |  |  | 0.527 |
| walker |  | 6985 | 18 | Code::CodeKey { rung: Body, file: widgets/services/servicesInfo.widget.js, decl: 3, sub: 0, line: 10 } |  |  | 0.527 |
| walker |  | 7015 | 30 | Code::CodeKey { rung: Names, file: widgets/containers/containerSortList.widget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.527 |
| ns | 7121 |  | 233 | src/widgetsTemplates/list.widget.template.js: complete method roster | 5.3 |  | 0.533 |
| walker |  | 7134 | 119 | Code::CodeKey { rung: Decl, file: widgets/containers/containerSortList.widget.js, decl: 1, sub: 0, line: 4 } |  |  | 0.533 |
| walker |  | 7138 | 4 | Code::CodeKey { rung: Decl, file: widgets/containers/containerSortList.widget.js, decl: 10, sub: 0, line: 99 } |  |  | 0.533 |
| walker |  | 7158 | 20 | Code::CodeKey { rung: Body, file: widgets/containers/containerInfo.widget.js, decl: 4, sub: 0, line: 14 } |  |  | 0.533 |
| walker |  | 7178 | 20 | Code::CodeKey { rung: Body, file: widgets/containers/containerList.widget.js, decl: 9, sub: 0, line: 75 } |  |  | 0.533 |
| walker |  | 7198 | 20 | Code::CodeKey { rung: Body, file: widgets/images/imageInfo.widget.js, decl: 4, sub: 0, line: 14 } |  |  | 0.533 |
| walker |  | 7218 | 20 | Code::CodeKey { rung: Body, file: widgets/services/servicesInfo.widget.js, decl: 4, sub: 0, line: 14 } |  |  | 0.533 |
| walker |  | 7245 | 27 | Code::CodeKey { rung: Body, file: src/dockerUtil.js, decl: 12, sub: 0, line: 155 } |  |  | 0.534 |
| walker |  | 7260 | 15 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/logs.widget.template.js, decl: 8, sub: 0, line: 84 } |  |  | 0.534 |
| walker |  | 7276 | 16 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/list.widget.template.js, decl: 8, sub: 0, line: 131 } |  |  | 0.534 |
| ns | 7409 |  | 288 | src/widgetsTemplates/info.widget.template.js and logs.widget.template.js: method rosters | 5.4 |  | 0.538 |
| ns | 7570 |  | 161 | src/widgetsTemplates/help.widget.template.js: method roster around the help text | 5.5 |  | 0.537 |
| walker |  | 7655 | 379 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.550 |
| walker |  | 7672 | 17 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/help.widget.template.js, decl: 5, sub: 0, line: 54 } |  |  | 0.550 |
| walker |  | 7689 | 17 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/info.widget.template.js, decl: 5, sub: 0, line: 68 } |  |  | 0.550 |
| walker |  | 7706 | 17 | Code::CodeKey { rung: Body, file: widgets/services/servicesList.widget.js, decl: 5, sub: 0, line: 19 } |  |  | 0.550 |
| walker |  | 7755 | 49 | Code::CodeKey { rung: Body, file: hooks/shell.hook.js, decl: 4, sub: 0, line: 59 } |  |  | 0.550 |
| walker |  | 7808 | 53 | Code::CodeKey { rung: Body, file: hooks/containers.hook.js, decl: 9, sub: 0, line: 168 } |  |  | 0.550 |
| walker |  | 7861 | 53 | Code::CodeKey { rung: Body, file: hooks/images.hook.js, decl: 3, sub: 0, line: 27 } |  |  | 0.550 |
| walker |  | 7914 | 53 | Code::CodeKey { rung: Body, file: hooks/services.hook.js, decl: 6, sub: 0, line: 86 } |  |  | 0.550 |
| ns | 7919 |  | 349 | hooks/: complete method rosters of all four hooks | 6.1 |  | 0.560 |
| walker |  | 7950 | 36 | Code::CodeKey { rung: Body, file: src/screen.js, decl: 10, sub: 0, line: 120 } |  |  | 0.561 |
| walker |  | 7977 | 27 | Code::CodeKey { rung: Body, file: src/dockerUtil.js, decl: 13, sub: 0, line: 160 } |  |  | 0.562 |
| walker |  | 8072 | 95 | Code::CodeKey { rung: Body, file: index.js, decl: 8, sub: 0, line: 72 } |  |  | 0.563 |
| ns | 8128 |  | 209 | hooks/containers.hook.js: the toolbar key dispatch inside init() | 6.2 | 6.1 | 0.555 |
| walker |  | 8133 | 61 | Code::CodeKey { rung: Body, file: src/cli.js, decl: 2, sub: 0, line: 59 } |  |  | 0.556 |
| walker |  | 8149 | 16 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/list.widget.template.js, decl: 9, sub: 0, line: 135 } |  |  | 0.556 |
| ns | 8482 |  | 354 | hooks/shell.hook.js: openShell() plus dockerRunScript.sh's docker exec line | 6.3 | 6.1 | 0.547 |
| walker |  | 8580 | 431 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.547 |
| ns | 8583 |  | 101 | src/enum.js in full — the ContainerState vocabulary | 7.1 |  | 0.552 |
| walker |  | 8726 | 146 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.552 |
| walker |  | 8752 | 26 | Code::CodeKey { rung: Body, file: widgets/images/imageList.widget.js, decl: 2, sub: 0, line: 6 } |  |  | 0.552 |
| walker |  | 8829 | 77 | Code::CodeKey { rung: Body, file: widgets/actionStatus.widget.js, decl: 2, sub: 0, line: 7 } |  |  | 0.552 |
| walker |  | 8856 | 27 | Code::CodeKey { rung: Body, file: widgets/containers/containerList.widget.js, decl: 13, sub: 0, line: 169 } |  |  | 0.552 |
| walker |  | 8893 | 37 | Code::CodeKey { rung: Body, file: src/screen.js, decl: 9, sub: 0, line: 114 } |  |  | 0.545 |
| ns | 8893 |  | 310 | src/themes/theme.selector.js in full and the dark/light style keys | 7.2 |  | 0.545 |
| walker |  | 8935 | 42 | Code::CodeKey { rung: Doc, file: widgets/containers/containerList.widget.js, decl: 13, sub: 0, line: 169 } |  |  | 0.545 |
| walker |  | 8977 | 42 | Code::CodeKey { rung: Doc, file: widgets/images/imageList.widget.js, decl: 9, sub: 0, line: 119 } |  |  | 0.545 |
| walker |  | 9019 | 42 | Code::CodeKey { rung: Doc, file: widgets/services/servicesList.widget.js, decl: 10, sub: 0, line: 87 } |  |  | 0.545 |
| walker |  | 9039 | 20 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/help.widget.template.js, decl: 10, sub: 0, line: 138 } |  |  | 0.545 |
| walker |  | 9059 | 20 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/info.widget.template.js, decl: 9, sub: 0, line: 115 } |  |  | 0.548 |
| walker |  | 9104 | 45 | Code::CodeKey { rung: Body, file: widgets/containers/containerSortList.widget.js, decl: 5, sub: 0, line: 47 } |  |  | 0.548 |
| walker |  | 9117 | 13 | Fs::DirListing { dir: docs/src/assets } |  |  | 0.548 |
| walker |  | 9123 | 6 | Fs::DirListing { dir: docs/src/assets/css } |  |  | 0.548 |
| walker |  | 9139 | 16 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/list.widget.template.js, decl: 10, sub: 0, line: 139 } |  |  | 0.548 |
| ns | 9179 |  | 286 | widgets/containers/containerList.widget.js: complete method roster | 7.3 |  | 0.549 |
| walker |  | 9233 | 94 | Code::CodeKey { rung: Body, file: hooks/containers.hook.js, decl: 4, sub: 0, line: 70 } |  |  | 0.549 |
| walker |  | 9327 | 94 | Code::CodeKey { rung: Body, file: hooks/services.hook.js, decl: 4, sub: 0, line: 61 } |  |  | 0.549 |
| walker |  | 9427 | 100 | Code::CodeKey { rung: Body, file: widgets/searchInput.widget.js, decl: 7, sub: 0, line: 94 } |  |  | 0.549 |
| ns | 9448 |  | 269 | widgets/toolbar.widget.js: the per-mode command extension map | 7.4 |  | 0.542 |
| walker |  | 9527 | 100 | Code::CodeKey { rung: Body, file: widgets/toolbar.widget.js, decl: 2, sub: 0, line: 8 } |  |  | 0.542 |
| walker |  | 9579 | 52 | Code::CodeKey { rung: Body, file: widgets/services/servicesStatus.widget.js, decl: 3, sub: 0, line: 22 } |  |  | 0.542 |
| ns | 9596 |  | 148 | widgets/actionsMenu.widget.js: the `m` menu's action table | 7.5 |  | 0.539 |
| walker |  | 9634 | 55 | Code::CodeKey { rung: Doc, file: widgets/images/imageList.widget.js, decl: 8, sub: 0, line: 96 } |  |  | 0.539 |
| ns | 9640 |  | 44 | Complete listings of .github/ and its subdirectories | 8.1 |  | 0.542 |
| ns | 9739 |  | 99 | Complete listings of the docs/ Gatsby site | 8.2 |  | 0.552 |
| walker |  | 9747 | 113 | Code::CodeKey { rung: Body, file: widgets/actionsMenu.widget.js, decl: 4, sub: 0, line: 41 } |  |  | 0.552 |
| walker |  | 9860 | 113 | Code::CodeKey { rung: Body, file: widgets/actionsMenu.widget.js, decl: 5, sub: 0, line: 53 } |  |  | 0.552 |
| walker |  | 9920 | 60 | Code::CodeKey { rung: Body, file: widgets/containers/containerSortList.widget.js, decl: 7, sub: 0, line: 63 } |  |  | 0.552 |
| walker |  | 9941 | 21 | Code::CodeKey { rung: Body, file: widgets/services/servicesList.widget.js, decl: 4, sub: 0, line: 15 } |  |  | 0.552 |
| ns | 9988 |  | 249 | Secondary ops config: codefresh.yml, devcontainer image, VS Code attach config, .nvmrc | 8.3 |  | 0.545 |
| walker |  | 9990 | 49 | Code::CodeKey { rung: Body, file: hooks/shell.hook.js, decl: 2, sub: 0, line: 9 } |  |  | 0.546 |

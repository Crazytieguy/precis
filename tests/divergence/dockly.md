Score(3000)=0.624 I=0.881 C=0.442 ns_rows≤3K=22/55 grid(1000/1442/2080/3000/4327/6240/9000)=0.903/0.778/0.700/0.624/0.562/0.512/0.504

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
| ns | 411 |  | 40 | Complete listings of src/widgetsTemplates and src/themes | 1.5 |  | 0.827 |
| walker |  | 425 | 34 | Json::Runtime { file: package.json } |  |  | 0.828 |
| walker |  | 444 | 19 | Fs::DirListing { dir: .github } |  |  | 0.829 |
| walker |  | 464 | 20 | Fs::DirListing { dir: .github/workflows } |  |  | 0.829 |
| ns | 533 |  | 122 | npm scripts block | 1.6 |  | 0.785 |
| walker |  | 627 | 163 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.786 |
| ns | 630 |  | 97 | Published `files` allow-list and the node engines floor | 1.7 |  | 0.733 |
| walker |  | 730 | 103 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.744 |
| walker |  | 752 | 22 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.745 |
| ns | 759 |  | 129 | README tagline plus every top-level heading location | 1.8 |  | 0.749 |
| walker |  | 861 | 109 | Plaintext::Whole { file: dockerRunScript.sh } |  |  | 0.749 |
| ns | 891 |  | 132 | index.js: shebang and the module wiring of the executable | 1.9 |  | 0.706 |
| walker |  | 968 | 107 | Json::Entry { file: package.json } |  |  | 0.903 |
| walker |  | 1020 | 52 | Fs::DirListing { dir: docs } |  |  | 0.903 |
| walker |  | 1029 | 9 | Fs::DirListing { dir: docs/src } |  |  | 0.904 |
| walker |  | 1042 | 13 | Fs::DirListing { dir: docs/src/pages } |  |  | 0.904 |
| ns | 1053 |  | 162 | index.js: pre-flight CLI dispatch (--help, --version, node version floor) | 1.10 | 1.9 | 0.837 |
| walker |  | 1162 | 120 | Json::Scripts { file: package.json } |  |  | 0.874 |
| walker |  | 1187 | 25 | Fs::DirListing { dir: docs/src/components } |  |  | 0.875 |
| ns | 1219 |  | 166 | index.js: bootstrap promise chain and the three helper function signatures | 1.11 | 1.10 | 0.808 |
| walker |  | 1357 | 170 | Plaintext::Whole { file: Dockerfile } |  |  | 0.812 |
| walker |  | 1411 | 54 | Code::CodeKey { rung: Names, file: index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.816 |
| ns | 1414 |  | 195 | Runtime dependency list from package.json | 1.12 |  | 0.777 |
| walker |  | 1606 | 195 | Json::Dependencies { file: package.json } |  |  | 0.830 |
| walker |  | 1701 | 95 | Code::CodeKey { rung: Body, file: index.js, decl: 3, sub: 0, line: 72 } |  |  | 0.831 |
| walker |  | 1711 | 10 | Code::CodeKey { rung: Names, file: lib/node.version.js, decl: 0, sub: 0, line: 0 } |  |  | 0.831 |
| walker |  | 1728 | 17 | Code::CodeKey { rung: Decl, file: lib/node.version.js, decl: 1, sub: 0, line: 1 } |  |  | 0.831 |
| ns | 1737 |  | 323 | The in-app keybinding table (src/widgetsTemplates/help.widget.template.js:102-125) | 2.1 |  | 0.767 |
| walker |  | 1738 | 10 | Code::CodeKey { rung: Names, file: src/enum.js, decl: 0, sub: 0, line: 0 } |  |  | 0.767 |
| walker |  | 1829 | 91 | Code::CodeKey { rung: Decl, file: src/enum.js, decl: 1, sub: 0, line: 1 } |  |  | 0.768 |
| walker |  | 1849 | 20 | Code::CodeKey { rung: Names, file: src/dockerUtil.js, decl: 0, sub: 0, line: 0 } |  |  | 0.768 |
| ns | 1857 |  | 120 | src/cli.js: the complete flag roster (all eight option names) | 2.2 |  | 0.745 |
| walker |  | 1869 | 20 | Code::CodeKey { rung: Names, file: src/screen.js, decl: 0, sub: 0, line: 0 } |  |  | 0.745 |
| walker |  | 2041 | 172 | Code::CodeKey { rung: Decl, file: src/screen.js, decl: 1, sub: 0, line: 18 } |  |  | 0.748 |
| ns | 2043 |  | 186 | src/cli.js: the four docker-connection options in full (socketPath, host, port, protocol) | 2.3 | 2.2 | 0.699 |
| walker |  | 2050 | 9 | Code::CodeKey { rung: Body, file: src/screen.js, decl: 16, sub: 0, line: 192 } |  |  | 0.699 |
| walker |  | 2082 | 32 | Code::CodeKey { rung: Body, file: src/screen.js, decl: 11, sub: 0, line: 126 } |  |  | 0.700 |
| walker |  | 2114 | 32 | Code::CodeKey { rung: Body, file: src/screen.js, decl: 12, sub: 0, line: 132 } |  |  | 0.700 |
| ns | 2206 |  | 163 | src/cli.js: the four behavioural options in full (help, version, containerFilters, theme) | 2.4 | 2.2 | 0.659 |
| ns | 2269 |  | 63 | src/cli.js: every prototype method and the singleton export | 2.5 |  | 0.648 |
| ns | 2339 |  | 70 | README: install and launch commands | 2.6 |  | 0.638 |
| walker |  | 2411 | 297 | Code::CodeKey { rung: Decl, file: src/dockerUtil.js, decl: 1, sub: 0, line: 5 } |  |  | 0.640 |
| walker |  | 2437 | 26 | Code::CodeKey { rung: Body, file: src/dockerUtil.js, decl: 14, sub: 0, line: 165 } |  |  | 0.641 |
| ns | 2455 |  | 116 | README: --containerFilters semantics | 2.7 |  | 0.635 |
| walker |  | 2463 | 26 | Code::CodeKey { rung: Body, file: src/dockerUtil.js, decl: 15, sub: 0, line: 170 } |  |  | 0.635 |
| walker |  | 2489 | 26 | Code::CodeKey { rung: Body, file: src/dockerUtil.js, decl: 16, sub: 0, line: 175 } |  |  | 0.635 |
| walker |  | 2516 | 27 | Code::CodeKey { rung: Body, file: src/dockerUtil.js, decl: 12, sub: 0, line: 155 } |  |  | 0.635 |
| walker |  | 2538 | 22 | Code::CodeKey { rung: Names, file: src/assetsLoader.js, decl: 0, sub: 0, line: 0 } |  |  | 0.635 |
| walker |  | 2580 | 42 | Code::CodeKey { rung: Decl, file: src/assetsLoader.js, decl: 1, sub: 0, line: 10 } |  |  | 0.635 |
| walker |  | 2595 | 15 | Code::CodeKey { rung: Body, file: src/assetsLoader.js, decl: 4, sub: 0, line: 39 } |  |  | 0.635 |
| walker |  | 2616 | 21 | Code::CodeKey { rung: Body, file: src/assetsLoader.js, decl: 2, sub: 0, line: 11 } |  |  | 0.636 |
| ns | 2625 |  | 170 | Dockerfile in full | 2.8 |  | 0.654 |
| walker |  | 2639 | 23 | Code::CodeKey { rung: Names, file: hooks/containers.hook.js, decl: 0, sub: 0, line: 0 } |  |  | 0.654 |
| ns | 2724 |  | 99 | README: running and building the docker image | 2.9 |  | 0.644 |
| walker |  | 2739 | 100 | Code::CodeKey { rung: Decl, file: hooks/containers.hook.js, decl: 1, sub: 0, line: 5 } |  |  | 0.644 |
| walker |  | 2792 | 53 | Code::CodeKey { rung: Body, file: hooks/containers.hook.js, decl: 9, sub: 0, line: 168 } |  |  | 0.644 |
| walker |  | 2815 | 23 | Code::CodeKey { rung: Names, file: hooks/images.hook.js, decl: 0, sub: 0, line: 0 } |  |  | 0.644 |
| walker |  | 2864 | 49 | Code::CodeKey { rung: Decl, file: hooks/images.hook.js, decl: 1, sub: 0, line: 5 } |  |  | 0.644 |
| ns | 2876 |  | 152 | .github/workflows/main.yml: the lint job | 2.10 |  | 0.623 |
| walker |  | 2917 | 53 | Code::CodeKey { rung: Body, file: hooks/images.hook.js, decl: 3, sub: 0, line: 27 } |  |  | 0.623 |
| walker |  | 2940 | 23 | Code::CodeKey { rung: Names, file: hooks/services.hook.js, decl: 0, sub: 0, line: 0 } |  |  | 0.623 |
| walker |  | 3004 | 64 | Code::CodeKey { rung: Decl, file: hooks/services.hook.js, decl: 1, sub: 0, line: 5 } |  |  | 0.624 |
| walker |  | 3057 | 53 | Code::CodeKey { rung: Body, file: hooks/services.hook.js, decl: 6, sub: 0, line: 86 } |  |  | 0.624 |
| ns | 3087 |  | 211 | .github/workflows/main.yml: the semantic-release job | 2.11 |  | 0.606 |
| walker |  | 3093 | 36 | Code::CodeKey { rung: Body, file: src/screen.js, decl: 10, sub: 0, line: 120 } |  |  | 0.606 |
| walker |  | 3098 | 5 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.606 |
| walker |  | 3122 | 24 | Code::CodeKey { rung: Names, file: hooks/shell.hook.js, decl: 0, sub: 0, line: 0 } |  |  | 0.606 |
| walker |  | 3162 | 40 | Code::CodeKey { rung: Decl, file: hooks/shell.hook.js, decl: 1, sub: 0, line: 8 } |  |  | 0.607 |
| ns | 3170 |  | 83 | lib/modes.js and lib/node.version.js in full | 3.1 |  | 0.598 |
| walker |  | 3211 | 49 | Code::CodeKey { rung: Body, file: hooks/shell.hook.js, decl: 4, sub: 0, line: 59 } |  |  | 0.598 |
| walker |  | 3223 | 12 | Code::CodeKey { rung: Names, file: lib/modes.js, decl: 0, sub: 0, line: 0 } |  |  | 0.600 |
| walker |  | 3256 | 33 | Code::CodeKey { rung: Decl, file: lib/modes.js, decl: 1, sub: 0, line: 3 } |  |  | 0.613 |
| walker |  | 3283 | 27 | Code::CodeKey { rung: Body, file: src/dockerUtil.js, decl: 13, sub: 0, line: 160 } |  |  | 0.613 |
| walker |  | 3308 | 25 | Code::CodeKey { rung: Names, file: widgets/help.widget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.613 |
| walker |  | 3322 | 14 | Code::CodeKey { rung: Decl, file: widgets/help.widget.js, decl: 1, sub: 0, line: 5 } |  |  | 0.613 |
| walker |  | 3333 | 11 | Code::CodeKey { rung: Body, file: widgets/help.widget.js, decl: 2, sub: 0, line: 6 } |  |  | 0.613 |
| ns | 3375 |  | 205 | src/screen.js: imports and the mode -> grid-layout table | 3.2 |  | 0.597 |
| walker |  | 3472 | 139 | Code::CodeKey { rung: Body, file: index.js, decl: 4, sub: 0, line: 86 } |  |  | 0.599 |
| walker |  | 3536 | 64 | Code::CodeKey { rung: Names, file: src/cli.js, decl: 0, sub: 0, line: 0 } |  |  | 0.607 |
| walker |  | 3549 | 13 | Code::CodeKey { rung: Body, file: src/cli.js, decl: 4, sub: 0, line: 93 } |  |  | 0.611 |
| ns | 3573 |  | 198 | src/screen.js: complete method roster of the `screen` class | 3.3 |  | 0.618 |
| walker |  | 3582 | 33 | Code::CodeKey { rung: Doc, file: src/cli.js, decl: 4, sub: 0, line: 93 } |  |  | 0.618 |
| walker |  | 3608 | 26 | Code::CodeKey { rung: Names, file: widgets/actionsMenu.widget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.618 |
| walker |  | 3714 | 106 | Code::CodeKey { rung: Decl, file: widgets/actionsMenu.widget.js, decl: 1, sub: 0, line: 5 } |  |  | 0.618 |
| walker |  | 3722 | 8 | Code::CodeKey { rung: Body, file: widgets/actionsMenu.widget.js, decl: 9, sub: 0, line: 148 } |  |  | 0.618 |
| ns | 3770 |  | 197 | src/screen.js: init() — the full boot sequence | 3.4 | 3.3 | 0.599 |
| walker |  | 3783 | 61 | Code::CodeKey { rung: Body, file: src/cli.js, decl: 2, sub: 0, line: 59 } |  |  | 0.601 |
| walker |  | 3811 | 28 | Code::CodeKey { rung: Names, file: widgets/actionStatus.widget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.601 |
| walker |  | 3881 | 70 | Code::CodeKey { rung: Decl, file: widgets/actionStatus.widget.js, decl: 1, sub: 0, line: 6 } |  |  | 0.601 |
| walker |  | 3889 | 8 | Code::CodeKey { rung: Body, file: widgets/actionStatus.widget.js, decl: 4, sub: 0, line: 24 } |  |  | 0.601 |
| ns | 3900 |  | 130 | src/screen.js: constructor state fields | 3.5 | 3.3 | 0.589 |
| walker |  | 3920 | 31 | Code::CodeKey { rung: Body, file: widgets/actionStatus.widget.js, decl: 3, sub: 0, line: 18 } |  |  | 0.589 |
| walker |  | 3948 | 28 | Code::CodeKey { rung: Names, file: widgets/searchInput.widget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.589 |
| ns | 4024 |  | 124 | src/screen.js: initScreen() — blessed screen and the 12x12 grid | 3.6 | 3.3 | 0.580 |
| walker |  | 4030 | 82 | Code::CodeKey { rung: Decl, file: widgets/searchInput.widget.js, decl: 1, sub: 0, line: 9 } |  |  | 0.580 |
| walker |  | 4037 | 7 | Code::CodeKey { rung: Body, file: widgets/searchInput.widget.js, decl: 3, sub: 0, line: 24 } |  |  | 0.580 |
| walker |  | 4045 | 8 | Code::CodeKey { rung: Body, file: widgets/searchInput.widget.js, decl: 4, sub: 0, line: 28 } |  |  | 0.580 |
| walker |  | 4073 | 28 | Code::CodeKey { rung: Names, file: widgets/toolbar.widget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.580 |
| walker |  | 4132 | 59 | Code::CodeKey { rung: Decl, file: widgets/toolbar.widget.js, decl: 1, sub: 0, line: 7 } |  |  | 0.580 |
| walker |  | 4140 | 8 | Code::CodeKey { rung: Body, file: widgets/toolbar.widget.js, decl: 3, sub: 0, line: 21 } |  |  | 0.580 |
| walker |  | 4149 | 9 | Code::CodeKey { rung: Body, file: widgets/toolbar.widget.js, decl: 4, sub: 0, line: 25 } |  |  | 0.580 |
| walker |  | 4226 | 77 | Code::CodeKey { rung: Body, file: widgets/actionStatus.widget.js, decl: 2, sub: 0, line: 7 } |  |  | 0.580 |
| ns | 4286 |  | 262 | src/screen.js: initHooks() and initWidgets() — instantiation and layout gating | 3.7 | 3.3 | 0.562 |
| walker |  | 4410 | 184 | Code::CodeKey { rung: Body, file: index.js, decl: 2, sub: 0, line: 47 } |  |  | 0.562 |
| walker |  | 4447 | 37 | Code::CodeKey { rung: Body, file: src/screen.js, decl: 9, sub: 0, line: 114 } |  |  | 0.564 |
| walker |  | 4466 | 19 | Code::CodeKey { rung: Names, file: src/baseWidget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.564 |
| walker |  | 4560 | 94 | Code::CodeKey { rung: Body, file: hooks/containers.hook.js, decl: 4, sub: 0, line: 70 } |  |  | 0.564 |
| ns | 4580 |  | 294 | src/screen.js: toggleMode() and registerEvents() — the global q/v keys and focus borders | 3.8 | 3.3 | 0.545 |
| walker |  | 4654 | 94 | Code::CodeKey { rung: Body, file: hooks/services.hook.js, decl: 4, sub: 0, line: 61 } |  |  | 0.545 |
| ns | 4668 |  | 88 | src/assetsLoader.js: the glob patterns that define the plugin conventions | 3.9 |  | 0.540 |
| walker |  | 4754 | 100 | Code::CodeKey { rung: Body, file: widgets/searchInput.widget.js, decl: 7, sub: 0, line: 94 } |  |  | 0.540 |
| walker |  | 4854 | 100 | Code::CodeKey { rung: Body, file: widgets/toolbar.widget.js, decl: 2, sub: 0, line: 8 } |  |  | 0.540 |
| ns | 4871 |  | 203 | src/assetsLoader.js: the loader body and the asset naming rule | 3.10 | 3.9 | 0.533 |
| walker |  | 4879 | 25 | Code::CodeKey { rung: Names, file: widgets/containers/containerInfo.widget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.533 |
| walker |  | 4924 | 45 | Code::CodeKey { rung: Decl, file: widgets/containers/containerInfo.widget.js, decl: 1, sub: 0, line: 5 } |  |  | 0.533 |
| walker |  | 4934 | 10 | Code::CodeKey { rung: Body, file: widgets/containers/containerInfo.widget.js, decl: 2, sub: 0, line: 6 } |  |  | 0.533 |
| walker |  | 4952 | 18 | Code::CodeKey { rung: Body, file: widgets/containers/containerInfo.widget.js, decl: 3, sub: 0, line: 10 } |  |  | 0.533 |
| walker |  | 4972 | 20 | Code::CodeKey { rung: Body, file: widgets/containers/containerInfo.widget.js, decl: 4, sub: 0, line: 14 } |  |  | 0.533 |
| walker |  | 4997 | 25 | Code::CodeKey { rung: Names, file: widgets/containers/containerList.widget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.533 |
| ns | 5149 |  | 278 | src/grid.config.js: the containers-mode layout in full | 3.11 |  | 0.525 |
| walker |  | 5185 | 188 | Code::CodeKey { rung: Decl, file: widgets/containers/containerList.widget.js, decl: 1, sub: 0, line: 10 } |  |  | 0.526 |
| walker |  | 5194 | 9 | Code::CodeKey { rung: Body, file: widgets/containers/containerList.widget.js, decl: 3, sub: 0, line: 18 } |  |  | 0.526 |
| walker |  | 5210 | 16 | Code::CodeKey { rung: Body, file: widgets/containers/containerList.widget.js, decl: 7, sub: 0, line: 40 } |  |  | 0.526 |
| walker |  | 5230 | 20 | Code::CodeKey { rung: Body, file: widgets/containers/containerList.widget.js, decl: 9, sub: 0, line: 75 } |  |  | 0.526 |
| walker |  | 5257 | 27 | Code::CodeKey { rung: Body, file: widgets/containers/containerList.widget.js, decl: 13, sub: 0, line: 169 } |  |  | 0.526 |
| walker |  | 5299 | 42 | Code::CodeKey { rung: Doc, file: widgets/containers/containerList.widget.js, decl: 13, sub: 0, line: 169 } |  |  | 0.526 |
| walker |  | 5324 | 25 | Code::CodeKey { rung: Names, file: widgets/containers/containerLogs.widget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.526 |
| walker |  | 5349 | 25 | Code::CodeKey { rung: Decl, file: widgets/containers/containerLogs.widget.js, decl: 1, sub: 0, line: 5 } |  |  | 0.526 |
| walker |  | 5359 | 10 | Code::CodeKey { rung: Body, file: widgets/containers/containerLogs.widget.js, decl: 2, sub: 0, line: 6 } |  |  | 0.526 |
| walker |  | 5373 | 14 | Code::CodeKey { rung: Body, file: widgets/containers/containerLogs.widget.js, decl: 3, sub: 0, line: 10 } |  |  | 0.526 |
| walker |  | 5398 | 25 | Code::CodeKey { rung: Names, file: widgets/images/imageInfo.widget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.526 |
| walker |  | 5443 | 45 | Code::CodeKey { rung: Decl, file: widgets/images/imageInfo.widget.js, decl: 1, sub: 0, line: 5 } |  |  | 0.526 |
| walker |  | 5453 | 10 | Code::CodeKey { rung: Body, file: widgets/images/imageInfo.widget.js, decl: 2, sub: 0, line: 6 } |  |  | 0.526 |
| walker |  | 5471 | 18 | Code::CodeKey { rung: Body, file: widgets/images/imageInfo.widget.js, decl: 3, sub: 0, line: 10 } |  |  | 0.526 |
| walker |  | 5491 | 20 | Code::CodeKey { rung: Body, file: widgets/images/imageInfo.widget.js, decl: 4, sub: 0, line: 14 } |  |  | 0.526 |
| walker |  | 5516 | 25 | Code::CodeKey { rung: Names, file: widgets/images/imageList.widget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.526 |
| ns | 5554 |  | 405 | src/grid.config.js: the services and images layouts plus the export block | 3.12 | 3.11 | 0.512 |
| walker |  | 5651 | 135 | Code::CodeKey { rung: Decl, file: widgets/images/imageList.widget.js, decl: 1, sub: 0, line: 5 } |  |  | 0.512 |
| walker |  | 5660 | 9 | Code::CodeKey { rung: Body, file: widgets/images/imageList.widget.js, decl: 3, sub: 0, line: 11 } |  |  | 0.512 |
| walker |  | 5676 | 16 | Code::CodeKey { rung: Body, file: widgets/images/imageList.widget.js, decl: 4, sub: 0, line: 15 } |  |  | 0.512 |
| walker |  | 5694 | 18 | Code::CodeKey { rung: Body, file: widgets/images/imageList.widget.js, decl: 10, sub: 0, line: 123 } |  |  | 0.512 |
| walker |  | 5720 | 26 | Code::CodeKey { rung: Body, file: widgets/images/imageList.widget.js, decl: 2, sub: 0, line: 6 } |  |  | 0.512 |
| walker |  | 5762 | 42 | Code::CodeKey { rung: Doc, file: widgets/images/imageList.widget.js, decl: 9, sub: 0, line: 119 } |  |  | 0.512 |
| walker |  | 5817 | 55 | Code::CodeKey { rung: Doc, file: widgets/images/imageList.widget.js, decl: 8, sub: 0, line: 96 } |  |  | 0.512 |
| walker |  | 5842 | 25 | Code::CodeKey { rung: Names, file: widgets/services/servicesInfo.widget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.512 |
| ns | 5884 |  | 330 | src/dockerUtil.js: complete method roster of the dockerode wrapper | 4.1 |  | 0.523 |
| walker |  | 5887 | 45 | Code::CodeKey { rung: Decl, file: widgets/services/servicesInfo.widget.js, decl: 1, sub: 0, line: 5 } |  |  | 0.523 |
| walker |  | 5897 | 10 | Code::CodeKey { rung: Body, file: widgets/services/servicesInfo.widget.js, decl: 2, sub: 0, line: 6 } |  |  | 0.523 |
| walker |  | 5915 | 18 | Code::CodeKey { rung: Body, file: widgets/services/servicesInfo.widget.js, decl: 3, sub: 0, line: 10 } |  |  | 0.523 |
| walker |  | 5935 | 20 | Code::CodeKey { rung: Body, file: widgets/services/servicesInfo.widget.js, decl: 4, sub: 0, line: 14 } |  |  | 0.523 |
| walker |  | 5960 | 25 | Code::CodeKey { rung: Names, file: widgets/services/servicesList.widget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.523 |
| walker |  | 6092 | 132 | Code::CodeKey { rung: Decl, file: widgets/services/servicesList.widget.js, decl: 1, sub: 0, line: 5 } |  |  | 0.523 |
| walker |  | 6101 | 9 | Code::CodeKey { rung: Body, file: widgets/services/servicesList.widget.js, decl: 3, sub: 0, line: 11 } |  |  | 0.523 |
| walker |  | 6117 | 16 | Code::CodeKey { rung: Body, file: widgets/services/servicesList.widget.js, decl: 6, sub: 0, line: 23 } |  |  | 0.523 |
| walker |  | 6133 | 16 | Code::CodeKey { rung: Body, file: widgets/services/servicesList.widget.js, decl: 7, sub: 0, line: 27 } |  |  | 0.523 |
| walker |  | 6150 | 17 | Code::CodeKey { rung: Body, file: widgets/services/servicesList.widget.js, decl: 5, sub: 0, line: 19 } |  |  | 0.523 |
| ns | 6164 |  | 280 | src/dockerUtil.js: constructor — connection config and containerFilters parsing | 4.2 | 4.1 | 0.512 |
| walker |  | 6192 | 42 | Code::CodeKey { rung: Doc, file: widgets/services/servicesList.widget.js, decl: 10, sub: 0, line: 87 } |  |  | 0.512 |
| walker |  | 6217 | 25 | Code::CodeKey { rung: Names, file: widgets/services/servicesLogs.widget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.512 |
| walker |  | 6242 | 25 | Code::CodeKey { rung: Decl, file: widgets/services/servicesLogs.widget.js, decl: 1, sub: 0, line: 5 } |  |  | 0.512 |
| walker |  | 6252 | 10 | Code::CodeKey { rung: Body, file: widgets/services/servicesLogs.widget.js, decl: 2, sub: 0, line: 6 } |  |  | 0.512 |
| walker |  | 6266 | 14 | Code::CodeKey { rung: Body, file: widgets/services/servicesLogs.widget.js, decl: 3, sub: 0, line: 10 } |  |  | 0.512 |
| ns | 6314 |  | 150 | src/dockerUtil.js: getInfo() — the host fields surfaced to the status widgets | 4.3 | 4.1 | 0.506 |
| walker |  | 6426 | 160 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.517 |
| ns | 6429 |  | 115 | src/dockerUtil.js: log streaming options for containers and services | 4.4 | 4.1 | 0.511 |
| walker |  | 6539 | 113 | Code::CodeKey { rung: Body, file: widgets/actionsMenu.widget.js, decl: 4, sub: 0, line: 41 } |  |  | 0.511 |
| walker |  | 6652 | 113 | Code::CodeKey { rung: Body, file: widgets/actionsMenu.widget.js, decl: 5, sub: 0, line: 53 } |  |  | 0.511 |
| ns | 6656 |  | 227 | src/baseWidget.js — the mixin every widget and hook extends | 5.1 |  | 0.500 |
| walker |  | 6678 | 26 | Code::CodeKey { rung: Names, file: src/widgetsTemplates/help.widget.template.js, decl: 0, sub: 0, line: 0 } |  |  | 0.500 |
| walker |  | 6802 | 124 | Code::CodeKey { rung: Decl, file: src/widgetsTemplates/help.widget.template.js, decl: 1, sub: 0, line: 5 } |  |  | 0.501 |
| walker |  | 6810 | 8 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/help.widget.template.js, decl: 8, sub: 0, line: 130 } |  |  | 0.501 |
| walker |  | 6825 | 15 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/help.widget.template.js, decl: 9, sub: 0, line: 134 } |  |  | 0.501 |
| walker |  | 6842 | 17 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/help.widget.template.js, decl: 4, sub: 0, line: 50 } |  |  | 0.501 |
| walker |  | 6859 | 17 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/help.widget.template.js, decl: 5, sub: 0, line: 54 } |  |  | 0.501 |
| walker |  | 6879 | 20 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/help.widget.template.js, decl: 10, sub: 0, line: 138 } |  |  | 0.501 |
| ns | 6888 |  | 232 | src/widgetsTemplates/base.hook.template.js — the hook base class | 5.2 |  | 0.490 |
| walker |  | 6905 | 26 | Code::CodeKey { rung: Names, file: src/widgetsTemplates/info.widget.template.js, decl: 0, sub: 0, line: 0 } |  |  | 0.490 |
| walker |  | 7017 | 112 | Code::CodeKey { rung: Decl, file: src/widgetsTemplates/info.widget.template.js, decl: 1, sub: 0, line: 6 } |  |  | 0.490 |
| walker |  | 7025 | 8 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/info.widget.template.js, decl: 7, sub: 0, line: 107 } |  |  | 0.490 |
| walker |  | 7040 | 15 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/info.widget.template.js, decl: 8, sub: 0, line: 111 } |  |  | 0.490 |
| walker |  | 7057 | 17 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/info.widget.template.js, decl: 4, sub: 0, line: 64 } |  |  | 0.490 |
| walker |  | 7074 | 17 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/info.widget.template.js, decl: 5, sub: 0, line: 68 } |  |  | 0.490 |
| walker |  | 7094 | 20 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/info.widget.template.js, decl: 9, sub: 0, line: 115 } |  |  | 0.490 |
| walker |  | 7120 | 26 | Code::CodeKey { rung: Names, file: src/widgetsTemplates/logs.widget.template.js, decl: 0, sub: 0, line: 0 } |  |  | 0.490 |
| ns | 7121 |  | 233 | src/widgetsTemplates/list.widget.template.js: complete method roster | 5.3 |  | 0.481 |
| walker |  | 7211 | 91 | Code::CodeKey { rung: Decl, file: src/widgetsTemplates/logs.widget.template.js, decl: 1, sub: 0, line: 7 } |  |  | 0.482 |
| walker |  | 7222 | 11 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/logs.widget.template.js, decl: 6, sub: 0, line: 76 } |  |  | 0.482 |
| walker |  | 7233 | 11 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/logs.widget.template.js, decl: 7, sub: 0, line: 80 } |  |  | 0.482 |
| walker |  | 7248 | 15 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/logs.widget.template.js, decl: 5, sub: 0, line: 72 } |  |  | 0.482 |
| walker |  | 7263 | 15 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/logs.widget.template.js, decl: 8, sub: 0, line: 84 } |  |  | 0.482 |
| walker |  | 7289 | 26 | Code::CodeKey { rung: Names, file: widgets/containers/containerStatus.widget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.482 |
| walker |  | 7348 | 59 | Code::CodeKey { rung: Decl, file: widgets/containers/containerStatus.widget.js, decl: 1, sub: 0, line: 5 } |  |  | 0.482 |
| walker |  | 7374 | 26 | Code::CodeKey { rung: Names, file: widgets/containers/containerUtilization.widget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.482 |
| ns | 7409 |  | 288 | src/widgetsTemplates/info.widget.template.js and logs.widget.template.js: method rosters | 5.4 |  | 0.491 |
| walker |  | 7433 | 59 | Code::CodeKey { rung: Decl, file: widgets/containers/containerUtilization.widget.js, decl: 1, sub: 0, line: 4 } |  |  | 0.491 |
| walker |  | 7459 | 26 | Code::CodeKey { rung: Names, file: widgets/containers/containerVsImages.widget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.491 |
| walker |  | 7518 | 59 | Code::CodeKey { rung: Decl, file: widgets/containers/containerVsImages.widget.js, decl: 1, sub: 0, line: 4 } |  |  | 0.491 |
| walker |  | 7544 | 26 | Code::CodeKey { rung: Names, file: widgets/images/imageUtilization.widget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.491 |
| ns | 7570 |  | 161 | src/widgetsTemplates/help.widget.template.js: method roster around the help text | 5.5 |  | 0.491 |
| walker |  | 7603 | 59 | Code::CodeKey { rung: Decl, file: widgets/images/imageUtilization.widget.js, decl: 1, sub: 0, line: 5 } |  |  | 0.491 |
| walker |  | 7629 | 26 | Code::CodeKey { rung: Names, file: widgets/services/servicesStatus.widget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.491 |
| walker |  | 7688 | 59 | Code::CodeKey { rung: Decl, file: widgets/services/servicesStatus.widget.js, decl: 1, sub: 0, line: 5 } |  |  | 0.491 |
| walker |  | 7740 | 52 | Code::CodeKey { rung: Body, file: widgets/services/servicesStatus.widget.js, decl: 3, sub: 0, line: 22 } |  |  | 0.491 |
| walker |  | 7766 | 26 | Code::CodeKey { rung: Names, file: widgets/services/servicesVsImages.widget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.491 |
| walker |  | 7825 | 59 | Code::CodeKey { rung: Decl, file: widgets/services/servicesVsImages.widget.js, decl: 1, sub: 0, line: 4 } |  |  | 0.491 |
| walker |  | 7846 | 21 | Code::CodeKey { rung: Body, file: widgets/services/servicesList.widget.js, decl: 4, sub: 0, line: 15 } |  |  | 0.491 |
| ns | 7919 |  | 349 | hooks/: complete method rosters of all four hooks | 6.1 |  | 0.504 |
| walker |  | 7964 | 118 | Code::CodeKey { rung: Body, file: hooks/shell.hook.js, decl: 2, sub: 0, line: 9 } |  |  | 0.505 |
| walker |  | 8084 | 120 | Code::CodeKey { rung: Body, file: hooks/containers.hook.js, decl: 6, sub: 0, line: 94 } |  |  | 0.505 |
| ns | 8128 |  | 209 | hooks/containers.hook.js: the toolbar key dispatch inside init() | 6.2 | 6.1 | 0.498 |
| walker |  | 8204 | 120 | Code::CodeKey { rung: Body, file: hooks/services.hook.js, decl: 5, sub: 0, line: 72 } |  |  | 0.498 |
| walker |  | 8246 | 42 | Code::CodeKey { rung: Body, file: src/dockerUtil.js, decl: 17, sub: 0, line: 180 } |  |  | 0.499 |
| walker |  | 8310 | 64 | Code::CodeKey { rung: Doc, file: widgets/images/imageList.widget.js, decl: 7, sub: 0, line: 81 } |  |  | 0.499 |
| walker |  | 8455 | 145 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.499 |
| walker |  | 8469 | 14 | Code::CodeKey { rung: Names, file: src/themes/theme.selector.js, decl: 0, sub: 0, line: 0 } |  |  | 0.499 |
| ns | 8482 |  | 354 | hooks/shell.hook.js: openShell() plus dockerRunScript.sh's docker exec line | 6.3 | 6.1 | 0.490 |
| walker |  | 8497 | 28 | Code::CodeKey { rung: Names, file: src/widgetsTemplates/base.hook.template.js, decl: 0, sub: 0, line: 0 } |  |  | 0.491 |
| walker |  | 8526 | 29 | Code::CodeKey { rung: Decl, file: src/widgetsTemplates/base.hook.template.js, decl: 1, sub: 0, line: 6 } |  |  | 0.492 |
| walker |  | 8542 | 16 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/base.hook.template.js, decl: 3, sub: 0, line: 29 } |  |  | 0.493 |
| walker |  | 8570 | 28 | Code::CodeKey { rung: Names, file: src/widgetsTemplates/list.widget.template.js, decl: 0, sub: 0, line: 0 } |  |  | 0.493 |
| ns | 8583 |  | 101 | src/enum.js in full — the ContainerState vocabulary | 7.1 |  | 0.499 |
| walker |  | 8747 | 177 | Code::CodeKey { rung: Decl, file: src/widgetsTemplates/list.widget.template.js, decl: 1, sub: 0, line: 8 } |  |  | 0.510 |
| walker |  | 8762 | 15 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/list.widget.template.js, decl: 7, sub: 0, line: 127 } |  |  | 0.513 |
| walker |  | 8777 | 15 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/list.widget.template.js, decl: 12, sub: 0, line: 147 } |  |  | 0.513 |
| walker |  | 8792 | 15 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/list.widget.template.js, decl: 13, sub: 0, line: 151 } |  |  | 0.513 |
| walker |  | 8808 | 16 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/list.widget.template.js, decl: 8, sub: 0, line: 131 } |  |  | 0.513 |
| walker |  | 8824 | 16 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/list.widget.template.js, decl: 9, sub: 0, line: 135 } |  |  | 0.513 |
| walker |  | 8840 | 16 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/list.widget.template.js, decl: 10, sub: 0, line: 139 } |  |  | 0.513 |
| walker |  | 8856 | 16 | Code::CodeKey { rung: Body, file: src/widgetsTemplates/list.widget.template.js, decl: 11, sub: 0, line: 143 } |  |  | 0.513 |
| ns | 8893 |  | 310 | src/themes/theme.selector.js in full and the dark/light style keys | 7.2 |  | 0.502 |
| walker |  | 8925 | 69 | Code::CodeKey { rung: Body, file: src/themes/theme.selector.js, decl: 1, sub: 0, line: 13 } |  |  | 0.504 |
| walker |  | 8955 | 30 | Code::CodeKey { rung: Names, file: widgets/containers/containerSortList.widget.js, decl: 0, sub: 0, line: 0 } |  |  | 0.504 |
| walker |  | 9074 | 119 | Code::CodeKey { rung: Decl, file: widgets/containers/containerSortList.widget.js, decl: 1, sub: 0, line: 4 } |  |  | 0.504 |
| walker |  | 9078 | 4 | Code::CodeKey { rung: Decl, file: widgets/containers/containerSortList.widget.js, decl: 10, sub: 0, line: 99 } |  |  | 0.504 |
| walker |  | 9123 | 45 | Code::CodeKey { rung: Body, file: widgets/containers/containerSortList.widget.js, decl: 5, sub: 0, line: 47 } |  |  | 0.504 |
| ns | 9179 |  | 286 | widgets/containers/containerList.widget.js: complete method roster | 7.3 |  | 0.506 |
| walker |  | 9183 | 60 | Code::CodeKey { rung: Body, file: widgets/containers/containerSortList.widget.js, decl: 7, sub: 0, line: 63 } |  |  | 0.506 |
| walker |  | 9256 | 73 | Code::CodeKey { rung: Body, file: widgets/services/servicesVsImages.widget.js, decl: 5, sub: 0, line: 48 } |  |  | 0.506 |
| walker |  | 9330 | 74 | Code::CodeKey { rung: Body, file: widgets/containers/containerVsImages.widget.js, decl: 5, sub: 0, line: 49 } |  |  | 0.506 |
| walker |  | 9381 | 51 | Code::CodeKey { rung: Body, file: src/screen.js, decl: 8, sub: 0, line: 107 } |  |  | 0.507 |
| ns | 9448 |  | 269 | widgets/toolbar.widget.js: the per-mode command extension map | 7.4 |  | 0.500 |
| walker |  | 9460 | 79 | Code::CodeKey { rung: Body, file: widgets/containers/containerVsImages.widget.js, decl: 2, sub: 0, line: 5 } |  |  | 0.500 |
| walker |  | 9539 | 79 | Code::CodeKey { rung: Body, file: widgets/services/servicesVsImages.widget.js, decl: 2, sub: 0, line: 5 } |  |  | 0.500 |
| walker |  | 9590 | 51 | Code::CodeKey { rung: Names, file: src/themes/styles.js, decl: 0, sub: 0, line: 0 } |  |  | 0.503 |
| ns | 9596 |  | 148 | widgets/actionsMenu.widget.js: the `m` menu's action table | 7.5 |  | 0.500 |
| ns | 9640 |  | 44 | Complete listings of .github/ and its subdirectories | 8.1 |  | 0.505 |
| ns | 9739 |  | 99 | Complete listings of the docs/ Gatsby site | 8.2 |  | 0.515 |
| walker |  | 9796 | 206 | Code::CodeKey { rung: Decl, file: src/themes/styles.js, decl: 2, sub: 0, line: 28 } |  |  | 0.516 |
| ns | 9988 |  | 249 | Secondary ops config: codefresh.yml, devcontainer image, VS Code attach config, .nvmrc | 8.3 |  | 0.509 |
| walker |  | 10000 | 204 | Code::CodeKey { rung: Decl, file: src/themes/styles.js, decl: 1, sub: 0, line: 1 } |  |  | 0.514 |

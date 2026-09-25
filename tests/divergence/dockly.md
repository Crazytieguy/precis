Score(3000)=0.638 I=0.883 C=0.462 ns_rows≤3K=22/55 grid(1000/1442/2080/3000/4327/6240/9000)=0.710/0.633/0.700/0.638/0.577/0.547/0.597

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 72 | 72 | listing of '.' |  |  | 0.000 |
| walker |  | 81 | 9 | listing of 'lib' |  |  | 0.000 |
| ns | 95 |  | 95 | Package identity: name, version, description, entry point, bin name | 1.1 |  | 0.000 |
| walker |  | 105 | 24 | listing of 'hooks' |  |  | 0.000 |
| walker |  | 109 | 4 | listing of '.vscode' |  |  | 0.000 |
| walker |  | 119 | 10 | ts names lib/node.version.js |  |  | 0.000 |
| walker |  | 124 | 5 | listing of '.devcontainer' |  |  | 0.000 |
| walker |  | 163 | 39 | listing of 'src' |  |  | 0.000 |
| ns | 167 |  | 72 | Complete root directory listing | 1.2 |  | 0.560 |
| walker |  | 172 | 9 | listing of 'src/themes' |  |  | 0.561 |
| walker |  | 203 | 31 | listing of 'src/widgetsTemplates' |  |  | 0.577 |
| walker |  | 240 | 37 | listing of 'widgets' |  |  | 0.626 |
| walker |  | 252 | 12 | ts names lib/modes.js |  |  | 0.626 |
| ns | 276 |  | 109 | Complete listings of the four runtime code directories: src/, hooks/, lib/, widgets/ | 1.3 |  | 0.625 |
| walker |  | 306 | 54 | ts names index.js |  |  | 0.625 |
| walker |  | 325 | 19 | listing of 'widgets/images' |  |  | 0.627 |
| ns | 371 |  | 95 | Complete listings of the per-mode widget directories: widgets/containers, widgets/images, widgets/services | 1.4 |  | 0.541 |
| walker |  | 391 | 66 | package identity in package.json |  |  | 0.706 |
| walker |  | 408 | 17 | ts decl lib/node.version.js:1 |  |  | 0.706 |
| ns | 411 |  | 40 | Complete listings of src/widgetsTemplates and src/themes | 1.5 |  | 0.712 |
| walker |  | 428 | 20 | ts names src/dockerUtil.js |  |  | 0.712 |
| walker |  | 448 | 20 | ts names src/screen.js |  |  | 0.712 |
| walker |  | 479 | 31 | listing of 'widgets/services' |  |  | 0.742 |
| walker |  | 524 | 45 | listing of 'widgets/containers' |  |  | 0.829 |
| ns | 533 |  | 122 | npm scripts block | 1.6 |  | 0.785 |
| walker |  | 557 | 33 | ts decl lib/modes.js:3 |  |  | 0.787 |
| walker |  | 591 | 34 | package runtime metadata in package.json |  |  | 0.788 |
| walker |  | 610 | 19 | listing of '.github' |  |  | 0.788 |
| walker |  | 630 | 20 | listing of '.github/workflows' |  |  | 0.735 |
| ns | 630 |  | 97 | Published `files` allow-list and the node engines floor | 1.7 |  | 0.735 |
| ns | 759 |  | 129 | README tagline plus every top-level heading location | 1.8 |  | 0.683 |
| walker |  | 793 | 163 | README headline in README.md |  |  | 0.688 |
| walker |  | 857 | 64 | ts names src/cli.js |  |  | 0.689 |
| ns | 891 |  | 132 | index.js: shebang and the module wiring of the executable | 1.9 |  | 0.649 |
| walker |  | 960 | 103 | headings outline in README.md |  |  | 0.709 |
| walker |  | 982 | 22 | README.md section #0 |  |  | 0.710 |
| walker |  | 992 | 10 | ts names src/enum.js |  |  | 0.710 |
| walker |  | 1005 | 13 | ts body src/cli.js:93 |  |  | 0.710 |
| ns | 1053 |  | 162 | index.js: pre-flight CLI dispatch (--help, --version, node version floor) | 1.10 | 1.9 | 0.658 |
| walker |  | 1114 | 109 | plaintext config dockerRunScript.sh |  |  | 0.658 |
| ns | 1219 |  | 166 | index.js: bootstrap promise chain and the three helper function signatures | 1.11 | 1.10 | 0.610 |
| walker |  | 1309 | 195 | package runtime dependencies in package.json |  |  | 0.617 |
| walker |  | 1361 | 52 | listing of 'docs' |  |  | 0.617 |
| walker |  | 1370 | 9 | listing of 'docs/src' |  |  | 0.618 |
| ns | 1414 |  | 195 | Runtime dependency list from package.json | 1.12 |  | 0.633 |
| walker |  | 1477 | 107 | package entrypoints in package.json |  |  | 0.796 |
| walker |  | 1490 | 13 | listing of 'docs/src/pages' |  |  | 0.796 |
| walker |  | 1610 | 120 | package scripts in package.json |  |  | 0.828 |
| walker |  | 1629 | 19 | ts names src/baseWidget.js |  |  | 0.828 |
| ns | 1737 |  | 323 | The in-app keybinding table (src/widgetsTemplates/help.widget.template.js:102-125) | 2.1 |  | 0.764 |
| walker |  | 1799 | 170 | plaintext config Dockerfile |  |  | 0.768 |
| walker |  | 1821 | 22 | ts names src/assetsLoader.js |  |  | 0.768 |
| ns | 1857 |  | 120 | src/cli.js: the complete flag roster (all eight option names) | 2.2 |  | 0.745 |
| walker |  | 1993 | 172 | ts decl src/screen.js:18 |  |  | 0.748 |
| walker |  | 2002 | 9 | ts body src/screen.js:192 |  |  | 0.748 |
| walker |  | 2027 | 25 | listing of 'docs/src/components' |  |  | 0.749 |
| walker |  | 2041 | 14 | ts names src/themes/theme.selector.js |  |  | 0.749 |
| ns | 2043 |  | 186 | src/cli.js: the four docker-connection options in full (socketPath, host, port, protocol) | 2.3 | 2.2 | 0.700 |
| walker |  | 2074 | 33 | ts doc src/cli.js:93 |  |  | 0.700 |
| ns | 2206 |  | 163 | src/cli.js: the four behavioural options in full (help, version, containerFilters, theme) | 2.4 | 2.2 | 0.660 |
| ns | 2269 |  | 63 | src/cli.js: every prototype method and the singleton export | 2.5 |  | 0.662 |
| ns | 2339 |  | 70 | README: install and launch commands | 2.6 |  | 0.652 |
| walker |  | 2371 | 297 | ts decl src/dockerUtil.js:5 |  |  | 0.654 |
| walker |  | 2397 | 26 | ts body src/dockerUtil.js:165 |  |  | 0.654 |
| walker |  | 2423 | 26 | ts body src/dockerUtil.js:170 |  |  | 0.655 |
| walker |  | 2449 | 26 | ts body src/dockerUtil.js:175 |  |  | 0.655 |
| ns | 2455 |  | 116 | README: --containerFilters semantics | 2.7 |  | 0.649 |
| walker |  | 2491 | 42 | ts decl src/assetsLoader.js:10 |  |  | 0.649 |
| walker |  | 2517 | 26 | ts names src/widgetsTemplates/help.widget.template.js |  |  | 0.649 |
| walker |  | 2543 | 26 | ts names src/widgetsTemplates/info.widget.template.js |  |  | 0.649 |
| walker |  | 2569 | 26 | ts names src/widgetsTemplates/logs.widget.template.js |  |  | 0.649 |
| walker |  | 2601 | 32 | ts body src/screen.js:126 |  |  | 0.649 |
| ns | 2625 |  | 170 | Dockerfile in full | 2.8 |  | 0.667 |
| walker |  | 2633 | 32 | ts body src/screen.js:132 |  |  | 0.667 |
| walker |  | 2694 | 61 | ts body src/cli.js:59 |  |  | 0.670 |
| walker |  | 2722 | 28 | ts names src/widgetsTemplates/base.hook.template.js |  |  | 0.670 |
| ns | 2724 |  | 99 | README: running and building the docker image | 2.9 |  | 0.659 |
| walker |  | 2750 | 28 | ts names src/widgetsTemplates/list.widget.template.js |  |  | 0.659 |
| walker |  | 2779 | 29 | ts decl src/widgetsTemplates/base.hook.template.js:6 |  |  | 0.659 |
| walker |  | 2874 | 95 | ts body index.js:72 |  |  | 0.660 |
| ns | 2876 |  | 152 | .github/workflows/main.yml: the lint job | 2.10 |  | 0.638 |
| walker |  | 2901 | 27 | ts body src/dockerUtil.js:155 |  |  | 0.638 |
| walker |  | 2906 | 5 | listing of '.github/ISSUE_TEMPLATE' |  |  | 0.638 |
| walker |  | 2942 | 36 | ts body src/screen.js:120 |  |  | 0.638 |
| ns | 3087 |  | 211 | .github/workflows/main.yml: the semantic-release job | 2.11 |  | 0.620 |
| ns | 3170 |  | 83 | lib/modes.js and lib/node.version.js in full | 3.1 |  | 0.624 |
| ns | 3375 |  | 205 | src/screen.js: imports and the mode -> grid-layout table | 3.2 |  | 0.609 |
| walker |  | 3409 | 467 | package identity metadata in package.json |  |  | 0.609 |
| walker |  | 3548 | 139 | ts body index.js:86 |  |  | 0.611 |
| ns | 3573 |  | 198 | src/screen.js: complete method roster of the `screen` class | 3.3 |  | 0.617 |
| walker |  | 3639 | 91 | ts decl src/enum.js:1 |  |  | 0.618 |
| walker |  | 3666 | 27 | ts body src/dockerUtil.js:160 |  |  | 0.618 |
| walker |  | 3717 | 51 | ts names src/themes/styles.js |  |  | 0.618 |
| ns | 3770 |  | 197 | src/screen.js: init() — the full boot sequence | 3.4 | 3.3 | 0.599 |
| walker |  | 3877 | 160 | README.md section #2 |  |  | 0.615 |
| ns | 3900 |  | 130 | src/screen.js: constructor state fields | 3.5 | 3.3 | 0.603 |
| walker |  | 4022 | 145 | README.md section #5 |  |  | 0.603 |
| ns | 4024 |  | 124 | src/screen.js: initScreen() — blessed screen and the 12x12 grid | 3.6 | 3.3 | 0.594 |
| walker |  | 4206 | 184 | ts body index.js:47 |  |  | 0.594 |
| walker |  | 4243 | 37 | ts body src/screen.js:114 |  |  | 0.596 |
| ns | 4286 |  | 262 | src/screen.js: initHooks() and initWidgets() — instantiation and layout gating | 3.7 | 3.3 | 0.577 |
| walker |  | 4427 | 184 | ts body src/cli.js:66 |  |  | 0.579 |
| walker |  | 4442 | 15 | ts body src/assetsLoader.js:39 |  |  | 0.579 |
| walker |  | 4533 | 91 | ts decl src/widgetsTemplates/logs.widget.template.js:7 |  |  | 0.579 |
| walker |  | 4575 | 42 | ts body src/dockerUtil.js:180 |  |  | 0.579 |
| ns | 4580 |  | 294 | src/screen.js: toggleMode() and registerEvents() — the global q/v keys and focus borders | 3.8 | 3.3 | 0.560 |
| ns | 4668 |  | 88 | src/assetsLoader.js: the glob patterns that define the plugin conventions | 3.9 |  | 0.554 |
| walker |  | 4687 | 112 | ts decl src/widgetsTemplates/info.widget.template.js:6 |  |  | 0.555 |
| walker |  | 4695 | 8 | ts body src/widgetsTemplates/info.widget.template.js:107 |  |  | 0.555 |
| walker |  | 4746 | 51 | ts body src/screen.js:107 |  |  | 0.557 |
| walker |  | 4870 | 124 | ts decl src/widgetsTemplates/help.widget.template.js:5 |  |  | 0.557 |
| ns | 4871 |  | 203 | src/assetsLoader.js: the loader body and the asset naming rule | 3.10 | 3.9 | 0.548 |
| walker |  | 4878 | 8 | ts body src/widgetsTemplates/help.widget.template.js:130 |  |  | 0.548 |
| ns | 5149 |  | 278 | src/grid.config.js: the containers-mode layout in full | 3.11 |  | 0.539 |
| walker |  | 5257 | 379 | README.md section #1 |  |  | 0.558 |
| walker |  | 5278 | 21 | ts body src/assetsLoader.js:11 |  |  | 0.560 |
| walker |  | 5289 | 11 | ts body src/widgetsTemplates/logs.widget.template.js:76 |  |  | 0.560 |
| walker |  | 5300 | 11 | ts body src/widgetsTemplates/logs.widget.template.js:80 |  |  | 0.560 |
| walker |  | 5342 | 42 | ts body src/dockerUtil.js:187 |  |  | 0.560 |
| ns | 5554 |  | 405 | src/grid.config.js: the services and images layouts plus the export block | 3.12 | 3.11 | 0.546 |
| walker |  | 5632 | 290 | README.md section #6 |  |  | 0.546 |
| walker |  | 5686 | 54 | ts body src/screen.js:79 |  |  | 0.548 |
| walker |  | 5701 | 15 | ts body src/widgetsTemplates/help.widget.template.js:134 |  |  | 0.548 |
| walker |  | 5716 | 15 | ts body src/widgetsTemplates/info.widget.template.js:111 |  |  | 0.548 |
| walker |  | 5731 | 15 | ts body src/widgetsTemplates/logs.widget.template.js:72 |  |  | 0.548 |
| ns | 5884 |  | 330 | src/dockerUtil.js: complete method roster of the dockerode wrapper | 4.1 |  | 0.558 |
| walker |  | 5908 | 177 | ts decl src/widgetsTemplates/list.widget.template.js:8 |  |  | 0.559 |
| walker |  | 5923 | 15 | ts body src/widgetsTemplates/list.widget.template.js:127 |  |  | 0.559 |
| walker |  | 5938 | 15 | ts body src/widgetsTemplates/list.widget.template.js:147 |  |  | 0.559 |
| walker |  | 5953 | 15 | ts body src/widgetsTemplates/list.widget.template.js:151 |  |  | 0.559 |
| walker |  | 5969 | 16 | ts body src/widgetsTemplates/base.hook.template.js:29 |  |  | 0.559 |
| walker |  | 5982 | 13 | listing of 'docs/src/assets' |  |  | 0.559 |
| walker |  | 5988 | 6 | listing of 'docs/src/assets/css' |  |  | 0.559 |
| walker |  | 6005 | 17 | ts body src/widgetsTemplates/help.widget.template.js:50 |  |  | 0.559 |
| walker |  | 6022 | 17 | ts body src/widgetsTemplates/info.widget.template.js:64 |  |  | 0.559 |
| ns | 6164 |  | 280 | src/dockerUtil.js: constructor — connection config and containerFilters parsing | 4.2 | 4.1 | 0.547 |
| ns | 6314 |  | 150 | src/dockerUtil.js: getInfo() — the host fields surfaced to the status widgets | 4.3 | 4.1 | 0.540 |
| ns | 6429 |  | 115 | src/dockerUtil.js: log streaming options for containers and services | 4.4 | 4.1 | 0.534 |
| walker |  | 6453 | 431 | README.md section #3 |  |  | 0.534 |
| walker |  | 6599 | 146 | README.md section #4 |  |  | 0.534 |
| ns | 6656 |  | 227 | src/baseWidget.js — the mixin every widget and hook extends | 5.1 |  | 0.523 |
| walker |  | 6805 | 206 | ts decl src/themes/styles.js:28 |  |  | 0.523 |
| ns | 6888 |  | 232 | src/widgetsTemplates/base.hook.template.js — the hook base class | 5.2 |  | 0.514 |
| walker |  | 7012 | 207 | ts decl src/themes/styles.js:1 |  |  | 0.514 |
| walker |  | 7078 | 66 | package identity in docs/package.json |  |  | 0.514 |
| walker |  | 7085 | 7 | plaintext config .nvmrc |  |  | 0.514 |
| ns | 7121 |  | 233 | src/widgetsTemplates/list.widget.template.js: complete method roster | 5.3 |  | 0.520 |
| walker |  | 7122 | 37 | headings outline in SECURITY.md |  |  | 0.520 |
| walker |  | 7185 | 63 | ts body src/dockerUtil.js:55 |  |  | 0.521 |
| walker |  | 7200 | 15 | ts body src/widgetsTemplates/logs.widget.template.js:84 |  |  | 0.521 |
| walker |  | 7216 | 16 | ts body src/widgetsTemplates/list.widget.template.js:131 |  |  | 0.521 |
| walker |  | 7233 | 17 | ts body src/widgetsTemplates/help.widget.template.js:54 |  |  | 0.521 |
| walker |  | 7250 | 17 | ts body src/widgetsTemplates/info.widget.template.js:68 |  |  | 0.521 |
| ns | 7409 |  | 288 | src/widgetsTemplates/info.widget.template.js and logs.widget.template.js: method rosters | 5.4 |  | 0.525 |
| ns | 7570 |  | 161 | src/widgetsTemplates/help.widget.template.js: method roster around the help text | 5.5 |  | 0.524 |
| walker |  | 7702 | 452 | ts body src/cli.js:8 |  |  | 0.591 |
| walker |  | 7765 | 63 | README headline in docs/README.md |  |  | 0.591 |
| walker |  | 7783 | 18 | headings outline in docs/README.md |  |  | 0.591 |
| walker |  | 7856 | 73 | ts body src/screen.js:71 |  |  | 0.594 |
| ns | 7919 |  | 349 | hooks/: complete method rosters of all four hooks | 6.1 |  | 0.579 |
| walker |  | 7922 | 66 | ts body src/dockerUtil.js:33 |  |  | 0.580 |
| walker |  | 7938 | 16 | docs/README.md section #1 |  |  | 0.580 |
| walker |  | 7992 | 54 | headings outline in CONTRIBUTING.md |  |  | 0.580 |
| walker |  | 8028 | 36 | headings outline in .github/PULL_REQUEST_TEMPLATE.md |  |  | 0.580 |
| walker |  | 8044 | 16 | ts body src/widgetsTemplates/list.widget.template.js:135 |  |  | 0.580 |
| walker |  | 8113 | 69 | ts body src/themes/theme.selector.js:13 |  |  | 0.581 |
| ns | 8128 |  | 209 | hooks/containers.hook.js: the toolbar key dispatch inside init() | 6.2 | 6.1 | 0.572 |
| walker |  | 8154 | 41 | docs/README.md section #0 |  |  | 0.572 |
| walker |  | 8226 | 72 | ts body src/dockerUtil.js:78 |  |  | 0.573 |
| walker |  | 8246 | 20 | ts body src/widgetsTemplates/help.widget.template.js:138 |  |  | 0.573 |
| walker |  | 8266 | 20 | ts body src/widgetsTemplates/info.widget.template.js:115 |  |  | 0.576 |
| walker |  | 8288 | 22 | ts names docs/src/components/Header.js |  |  | 0.576 |
| walker |  | 8310 | 22 | ts names docs/src/components/Scroll.js |  |  | 0.576 |
| walker |  | 8332 | 22 | ts names docs/src/components/layout.js |  |  | 0.576 |
| walker |  | 8354 | 22 | ts names docs/src/pages/generic.js |  |  | 0.576 |
| walker |  | 8367 | 13 | ts decl docs/src/pages/generic.js:8 |  |  | 0.576 |
| walker |  | 8389 | 22 | ts names docs/src/pages/index.js |  |  | 0.576 |
| walker |  | 8463 | 74 | headings outline in CODE_OF_CONDUCT.md |  |  | 0.576 |
| walker |  | 8479 | 16 | ts body src/widgetsTemplates/list.widget.template.js:139 |  |  | 0.576 |
| ns | 8482 |  | 354 | hooks/shell.hook.js: openShell() plus dockerRunScript.sh's docker exec line | 6.3 | 6.1 | 0.567 |
| walker |  | 8503 | 24 | ts names docs/src/components/Footer.js |  |  | 0.567 |
| walker |  | 8527 | 24 | ts names docs/src/components/Nav.js |  |  | 0.567 |
| walker |  | 8553 | 26 | ts names docs/src/components/HeaderGeneric.js |  |  | 0.567 |
| walker |  | 8579 | 26 | ts names docs/src/pages/404.js |  |  | 0.567 |
| ns | 8583 |  | 101 | src/enum.js in full — the ContainerState vocabulary | 7.1 |  | 0.572 |
| walker |  | 8765 | 186 | ts body src/baseWidget.js:4 |  |  | 0.590 |
| walker |  | 8885 | 120 | ts body src/screen.js:35 |  |  | 0.600 |
| ns | 8893 |  | 310 | src/themes/theme.selector.js in full and the dark/light style keys | 7.2 |  | 0.597 |
| walker |  | 8967 | 82 | ts body src/dockerUtil.js:43 |  |  | 0.597 |
| walker |  | 8993 | 26 | listing of 'docs/src/assets/scss' |  |  | 0.597 |
| walker |  | 9004 | 11 | listing of 'docs/src/assets/scss/base' |  |  | 0.597 |
| walker |  | 9020 | 16 | ts body src/widgetsTemplates/list.widget.template.js:143 |  |  | 0.597 |
| ns | 9179 |  | 286 | widgets/containers/containerList.widget.js: complete method roster | 7.3 |  | 0.590 |
| ns | 9448 |  | 269 | widgets/toolbar.widget.js: the per-mode command extension map | 7.4 |  | 0.582 |
| walker |  | 9517 | 497 | plaintext config .github/workflows/main.yml |  |  | 0.612 |
| ns | 9596 |  | 148 | widgets/actionsMenu.widget.js: the `m` menu's action table | 7.5 |  | 0.607 |
| ns | 9640 |  | 44 | Complete listings of .github/ and its subdirectories | 8.1 |  | 0.610 |
| walker |  | 9643 | 126 | ts body src/screen.js:19 |  |  | 0.622 |
| walker |  | 9729 | 86 | ts body src/dockerUtil.js:88 |  |  | 0.623 |
| ns | 9739 |  | 99 | Complete listings of the docs/ Gatsby site | 8.2 |  | 0.630 |
| walker |  | 9751 | 22 | listing of 'docs/src/assets/scss/libs' |  |  | 0.630 |
| ns | 9988 |  | 249 | Secondary ops config: codefresh.yml, devcontainer image, VS Code attach config, .nvmrc | 8.3 |  | 0.622 |

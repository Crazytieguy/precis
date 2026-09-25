Score(3000)=0.623 I=0.878 C=0.442 ns_rows≤3K=22/55 grid(1000/1442/2080/3000/4327/6240/9000)=0.648/0.581/0.670/0.623/0.552/0.538/0.604

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
| walker |  | 213 | 10 | ts names src/enum.js |  |  | 0.577 |
| walker |  | 250 | 37 | listing of 'widgets' |  |  | 0.626 |
| walker |  | 262 | 12 | ts names lib/modes.js |  |  | 0.626 |
| ns | 276 |  | 109 | Complete listings of the four runtime code directories: src/, hooks/, lib/, widgets/ | 1.3 |  | 0.625 |
| walker |  | 316 | 54 | ts names index.js |  |  | 0.625 |
| walker |  | 335 | 19 | listing of 'widgets/images' |  |  | 0.627 |
| ns | 371 |  | 95 | Complete listings of the per-mode widget directories: widgets/containers, widgets/images, widgets/services | 1.4 |  | 0.541 |
| walker |  | 401 | 66 | package identity in package.json |  |  | 0.706 |
| ns | 411 |  | 40 | Complete listings of src/widgetsTemplates and src/themes | 1.5 |  | 0.712 |
| walker |  | 418 | 17 | ts decl lib/node.version.js:1 |  |  | 0.712 |
| walker |  | 437 | 19 | ts names src/baseWidget.js |  |  | 0.712 |
| walker |  | 457 | 20 | ts names src/dockerUtil.js |  |  | 0.712 |
| walker |  | 477 | 20 | ts names src/screen.js |  |  | 0.712 |
| walker |  | 508 | 31 | listing of 'widgets/services' |  |  | 0.742 |
| walker |  | 530 | 22 | ts names src/assetsLoader.js |  |  | 0.742 |
| ns | 533 |  | 122 | npm scripts block | 1.6 |  | 0.703 |
| walker |  | 552 | 22 | ts names src/cli.js |  |  | 0.703 |
| walker |  | 566 | 14 | ts names src/themes/theme.selector.js |  |  | 0.703 |
| walker |  | 611 | 45 | listing of 'widgets/containers' |  |  | 0.785 |
| ns | 630 |  | 97 | Published `files` allow-list and the node engines floor | 1.7 |  | 0.727 |
| walker |  | 644 | 33 | ts decl lib/modes.js:3 |  |  | 0.729 |
| walker |  | 678 | 34 | package runtime metadata in package.json |  |  | 0.735 |
| walker |  | 697 | 19 | listing of '.github' |  |  | 0.735 |
| walker |  | 717 | 20 | listing of '.github/workflows' |  |  | 0.735 |
| ns | 759 |  | 129 | README tagline plus every top-level heading location | 1.8 |  | 0.683 |
| walker |  | 857 | 140 | README headline in README.md |  |  | 0.688 |
| walker |  | 880 | 23 | README prelude in README.md |  |  | 0.688 |
| ns | 891 |  | 132 | index.js: shebang and the module wiring of the executable | 1.9 |  | 0.648 |
| walker |  | 922 | 42 | ts decl src/assetsLoader.js:10 |  |  | 0.648 |
| walker |  | 948 | 26 | ts names src/widgetsTemplates/help.widget.template.js |  |  | 0.648 |
| walker |  | 974 | 26 | ts names src/widgetsTemplates/info.widget.template.js |  |  | 0.648 |
| walker |  | 1000 | 26 | ts names src/widgetsTemplates/logs.widget.template.js |  |  | 0.648 |
| walker |  | 1028 | 28 | ts names src/widgetsTemplates/base.hook.template.js |  |  | 0.648 |
| ns | 1053 |  | 162 | index.js: pre-flight CLI dispatch (--help, --version, node version floor) | 1.10 | 1.9 | 0.601 |
| walker |  | 1056 | 28 | ts names src/widgetsTemplates/list.widget.template.js |  |  | 0.601 |
| walker |  | 1085 | 29 | ts decl src/widgetsTemplates/base.hook.template.js:6 |  |  | 0.601 |
| walker |  | 1188 | 103 | headings outline in README.md |  |  | 0.657 |
| walker |  | 1210 | 22 | README.md section #0 |  |  | 0.657 |
| ns | 1219 |  | 166 | index.js: bootstrap promise chain and the three helper function signatures | 1.11 | 1.10 | 0.609 |
| walker |  | 1319 | 109 | plaintext config dockerRunScript.sh |  |  | 0.609 |
| walker |  | 1410 | 91 | ts decl src/enum.js:1 |  |  | 0.610 |
| ns | 1414 |  | 195 | Runtime dependency list from package.json | 1.12 |  | 0.581 |
| walker |  | 1580 | 170 | plaintext config Dockerfile |  |  | 0.586 |
| walker |  | 1631 | 51 | ts names src/themes/styles.js |  |  | 0.586 |
| walker |  | 1683 | 52 | listing of 'docs' |  |  | 0.586 |
| walker |  | 1692 | 9 | listing of 'docs/src' |  |  | 0.586 |
| ns | 1737 |  | 323 | The in-app keybinding table (src/widgetsTemplates/help.widget.template.js:102-125) | 2.1 |  | 0.541 |
| walker |  | 1799 | 107 | package entrypoints in package.json |  |  | 0.688 |
| ns | 1857 |  | 120 | src/cli.js: the complete flag roster (all eight option names) | 2.2 |  | 0.668 |
| walker |  | 1994 | 195 | package runtime dependencies in package.json |  |  | 0.717 |
| walker |  | 2007 | 13 | listing of 'docs/src/pages' |  |  | 0.717 |
| ns | 2043 |  | 186 | src/cli.js: the four docker-connection options in full (socketPath, host, port, protocol) | 2.3 | 2.2 | 0.670 |
| walker |  | 2127 | 120 | package scripts in package.json |  |  | 0.697 |
| ns | 2206 |  | 163 | src/cli.js: the four behavioural options in full (help, version, containerFilters, theme) | 2.4 | 2.2 | 0.656 |
| ns | 2269 |  | 63 | src/cli.js: every prototype method and the singleton export | 2.5 |  | 0.646 |
| walker |  | 2299 | 172 | ts decl src/screen.js:18 |  |  | 0.648 |
| walker |  | 2308 | 9 | ts body src/screen.js:192 |  |  | 0.649 |
| walker |  | 2323 | 15 | ts body src/assetsLoader.js:39 |  |  | 0.649 |
| ns | 2339 |  | 70 | README: install and launch commands | 2.6 |  | 0.639 |
| walker |  | 2414 | 91 | ts decl src/widgetsTemplates/logs.widget.template.js:7 |  |  | 0.639 |
| walker |  | 2439 | 25 | listing of 'docs/src/components' |  |  | 0.640 |
| ns | 2455 |  | 116 | README: --containerFilters semantics | 2.7 |  | 0.634 |
| walker |  | 2551 | 112 | ts decl src/widgetsTemplates/info.widget.template.js:6 |  |  | 0.635 |
| walker |  | 2559 | 8 | ts body src/widgetsTemplates/info.widget.template.js:107 |  |  | 0.635 |
| ns | 2625 |  | 170 | Dockerfile in full | 2.8 |  | 0.654 |
| walker |  | 2683 | 124 | ts decl src/widgetsTemplates/help.widget.template.js:5 |  |  | 0.654 |
| walker |  | 2691 | 8 | ts body src/widgetsTemplates/help.widget.template.js:130 |  |  | 0.654 |
| walker |  | 2712 | 21 | ts body src/assetsLoader.js:11 |  |  | 0.654 |
| walker |  | 2723 | 11 | ts body src/widgetsTemplates/logs.widget.template.js:76 |  |  | 0.654 |
| ns | 2724 |  | 99 | README: running and building the docker image | 2.9 |  | 0.644 |
| walker |  | 2734 | 11 | ts body src/widgetsTemplates/logs.widget.template.js:80 |  |  | 0.644 |
| ns | 2876 |  | 152 | .github/workflows/main.yml: the lint job | 2.10 |  | 0.623 |
| walker |  | 3031 | 297 | ts decl src/dockerUtil.js:5 |  |  | 0.625 |
| walker |  | 3057 | 26 | ts body src/dockerUtil.js:165 |  |  | 0.625 |
| walker |  | 3083 | 26 | ts body src/dockerUtil.js:170 |  |  | 0.625 |
| ns | 3087 |  | 211 | .github/workflows/main.yml: the semantic-release job | 2.11 |  | 0.607 |
| walker |  | 3109 | 26 | ts body src/dockerUtil.js:175 |  |  | 0.607 |
| walker |  | 3124 | 15 | ts body src/widgetsTemplates/help.widget.template.js:134 |  |  | 0.607 |
| walker |  | 3139 | 15 | ts body src/widgetsTemplates/info.widget.template.js:111 |  |  | 0.607 |
| walker |  | 3154 | 15 | ts body src/widgetsTemplates/logs.widget.template.js:72 |  |  | 0.607 |
| ns | 3170 |  | 83 | lib/modes.js and lib/node.version.js in full | 3.1 |  | 0.612 |
| walker |  | 3331 | 177 | ts decl src/widgetsTemplates/list.widget.template.js:8 |  |  | 0.613 |
| walker |  | 3346 | 15 | ts body src/widgetsTemplates/list.widget.template.js:127 |  |  | 0.613 |
| walker |  | 3361 | 15 | ts body src/widgetsTemplates/list.widget.template.js:147 |  |  | 0.613 |
| ns | 3375 |  | 205 | src/screen.js: imports and the mode -> grid-layout table | 3.2 |  | 0.597 |
| walker |  | 3376 | 15 | ts body src/widgetsTemplates/list.widget.template.js:151 |  |  | 0.597 |
| walker |  | 3392 | 16 | ts body src/widgetsTemplates/base.hook.template.js:29 |  |  | 0.598 |
| walker |  | 3424 | 32 | ts body src/screen.js:126 |  |  | 0.598 |
| walker |  | 3456 | 32 | ts body src/screen.js:132 |  |  | 0.598 |
| walker |  | 3473 | 17 | ts body src/widgetsTemplates/help.widget.template.js:50 |  |  | 0.598 |
| walker |  | 3490 | 17 | ts body src/widgetsTemplates/info.widget.template.js:64 |  |  | 0.598 |
| ns | 3573 |  | 198 | src/screen.js: complete method roster of the `screen` class | 3.3 |  | 0.604 |
| walker |  | 3696 | 206 | ts decl src/themes/styles.js:28 |  |  | 0.604 |
| ns | 3770 |  | 197 | src/screen.js: init() — the full boot sequence | 3.4 | 3.3 | 0.586 |
| ns | 3900 |  | 130 | src/screen.js: constructor state fields | 3.5 | 3.3 | 0.574 |
| walker |  | 3903 | 207 | ts decl src/themes/styles.js:1 |  |  | 0.575 |
| walker |  | 3998 | 95 | ts body index.js:72 |  |  | 0.575 |
| ns | 4024 |  | 124 | src/screen.js: initScreen() — blessed screen and the 12x12 grid | 3.6 | 3.3 | 0.566 |
| walker |  | 4025 | 27 | ts body src/dockerUtil.js:155 |  |  | 0.567 |
| walker |  | 4040 | 15 | ts body src/widgetsTemplates/logs.widget.template.js:84 |  |  | 0.567 |
| walker |  | 4045 | 5 | listing of '.github/ISSUE_TEMPLATE' |  |  | 0.567 |
| walker |  | 4061 | 16 | ts body src/widgetsTemplates/list.widget.template.js:131 |  |  | 0.567 |
| walker |  | 4078 | 17 | ts body src/widgetsTemplates/help.widget.template.js:54 |  |  | 0.567 |
| walker |  | 4095 | 17 | ts body src/widgetsTemplates/info.widget.template.js:68 |  |  | 0.567 |
| walker |  | 4131 | 36 | ts body src/screen.js:120 |  |  | 0.569 |
| walker |  | 4270 | 139 | ts body index.js:86 |  |  | 0.570 |
| ns | 4286 |  | 262 | src/screen.js: initHooks() and initWidgets() — instantiation and layout gating | 3.7 | 3.3 | 0.552 |
| walker |  | 4297 | 27 | ts body src/dockerUtil.js:160 |  |  | 0.552 |
| walker |  | 4457 | 160 | README.md section #2 |  |  | 0.567 |
| ns | 4580 |  | 294 | src/screen.js: toggleMode() and registerEvents() — the global q/v keys and focus borders | 3.8 | 3.3 | 0.547 |
| walker |  | 4602 | 145 | README.md section #4 |  |  | 0.547 |
| walker |  | 4618 | 16 | ts body src/widgetsTemplates/list.widget.template.js:135 |  |  | 0.547 |
| ns | 4668 |  | 88 | src/assetsLoader.js: the glob patterns that define the plugin conventions | 3.9 |  | 0.542 |
| ns | 4871 |  | 203 | src/assetsLoader.js: the loader body and the asset naming rule | 3.10 | 3.9 | 0.536 |
| walker |  | 5085 | 467 | package identity metadata in package.json |  |  | 0.536 |
| ns | 5149 |  | 278 | src/grid.config.js: the containers-mode layout in full | 3.11 |  | 0.527 |
| walker |  | 5154 | 69 | ts body src/themes/theme.selector.js:13 |  |  | 0.528 |
| walker |  | 5338 | 184 | ts body index.js:47 |  |  | 0.528 |
| walker |  | 5375 | 37 | ts body src/screen.js:114 |  |  | 0.530 |
| walker |  | 5395 | 20 | ts body src/widgetsTemplates/help.widget.template.js:138 |  |  | 0.530 |
| walker |  | 5415 | 20 | ts body src/widgetsTemplates/info.widget.template.js:115 |  |  | 0.530 |
| walker |  | 5431 | 16 | ts body src/widgetsTemplates/list.widget.template.js:139 |  |  | 0.530 |
| ns | 5554 |  | 405 | src/grid.config.js: the services and images layouts plus the export block | 3.12 | 3.11 | 0.517 |
| walker |  | 5617 | 186 | ts body src/baseWidget.js:4 |  |  | 0.518 |
| walker |  | 5633 | 16 | ts body src/widgetsTemplates/list.widget.template.js:143 |  |  | 0.518 |
| walker |  | 5675 | 42 | ts body src/dockerUtil.js:180 |  |  | 0.518 |
| walker |  | 5726 | 51 | ts body src/screen.js:107 |  |  | 0.520 |
| ns | 5884 |  | 330 | src/dockerUtil.js: complete method roster of the dockerode wrapper | 4.1 |  | 0.532 |
| walker |  | 6105 | 379 | README.md section #1 |  |  | 0.549 |
| walker |  | 6147 | 42 | ts body src/dockerUtil.js:187 |  |  | 0.550 |
| ns | 6164 |  | 280 | src/dockerUtil.js: constructor — connection config and containerFilters parsing | 4.2 | 4.1 | 0.538 |
| ns | 6314 |  | 150 | src/dockerUtil.js: getInfo() — the host fields surfaced to the status widgets | 4.3 | 4.1 | 0.532 |
| ns | 6429 |  | 115 | src/dockerUtil.js: log streaming options for containers and services | 4.4 | 4.1 | 0.526 |
| walker |  | 6437 | 290 | README.md section #5 |  |  | 0.526 |
| walker |  | 6636 | 199 | ts body src/assetsLoader.js:16 |  |  | 0.542 |
| ns | 6656 |  | 227 | src/baseWidget.js — the mixin every widget and hook extends | 5.1 |  | 0.553 |
| walker |  | 6690 | 54 | ts body src/screen.js:79 |  |  | 0.554 |
| walker |  | 6712 | 22 | ts body src/widgetsTemplates/list.widget.template.js:155 |  |  | 0.554 |
| walker |  | 6725 | 13 | listing of 'docs/src/assets' |  |  | 0.554 |
| walker |  | 6731 | 6 | listing of 'docs/src/assets/css' |  |  | 0.554 |
| ns | 6888 |  | 232 | src/widgetsTemplates/base.hook.template.js — the hook base class | 5.2 |  | 0.544 |
| ns | 7121 |  | 233 | src/widgetsTemplates/list.widget.template.js: complete method roster | 5.3 |  | 0.549 |
| walker |  | 7183 | 452 | ts body src/cli.js:8 |  |  | 0.619 |
| walker |  | 7249 | 66 | package identity in docs/package.json |  |  | 0.619 |
| walker |  | 7256 | 7 | plaintext config .nvmrc |  |  | 0.619 |
| walker |  | 7293 | 37 | headings outline in SECURITY.md |  |  | 0.619 |
| walker |  | 7293 | 0 | SECURITY.md section #0 |  |  | 0.619 |
| walker |  | 7356 | 63 | ts body src/dockerUtil.js:55 |  |  | 0.620 |
| ns | 7409 |  | 288 | src/widgetsTemplates/info.widget.template.js and logs.widget.template.js: method rosters | 5.4 |  | 0.623 |
| walker |  | 7434 | 78 | ts body src/widgetsTemplates/logs.widget.template.js:8 |  |  | 0.624 |
| ns | 7570 |  | 161 | src/widgetsTemplates/help.widget.template.js: method roster around the help text | 5.5 |  | 0.621 |
| walker |  | 7612 | 178 | ts body src/widgetsTemplates/base.hook.template.js:7 |  |  | 0.632 |
| ns | 7919 |  | 349 | hooks/: complete method rosters of all four hooks | 6.1 |  | 0.616 |
| ns | 8128 |  | 209 | hooks/containers.hook.js: the toolbar key dispatch inside init() | 6.2 | 6.1 | 0.607 |
| walker |  | 8194 | 582 | README.md section #3 |  |  | 0.607 |
| walker |  | 8257 | 63 | README headline in docs/README.md |  |  | 0.607 |
| walker |  | 8275 | 18 | headings outline in docs/README.md |  |  | 0.607 |
| walker |  | 8348 | 73 | ts body src/screen.js:71 |  |  | 0.610 |
| walker |  | 8414 | 66 | ts body src/dockerUtil.js:33 |  |  | 0.611 |
| walker |  | 8430 | 16 | docs/README.md section #1 |  |  | 0.611 |
| ns | 8482 |  | 354 | hooks/shell.hook.js: openShell() plus dockerRunScript.sh's docker exec line | 6.3 | 6.1 | 0.600 |
| walker |  | 8484 | 54 | headings outline in CONTRIBUTING.md |  |  | 0.600 |
| walker |  | 8520 | 36 | headings outline in .github/PULL_REQUEST_TEMPLATE.md |  |  | 0.600 |
| walker |  | 8520 | 0 | .github/PULL_REQUEST_TEMPLATE.md section #0 |  |  | 0.600 |
| walker |  | 8551 | 31 | package dev/peer dependencies in package.json |  |  | 0.600 |
| ns | 8583 |  | 101 | src/enum.js in full — the ContainerState vocabulary | 7.1 |  | 0.605 |
| walker |  | 8592 | 41 | docs/README.md section #0 |  |  | 0.605 |
| walker |  | 8687 | 95 | ts body src/widgetsTemplates/help.widget.template.js:6 |  |  | 0.605 |
| walker |  | 8782 | 95 | ts body src/widgetsTemplates/info.widget.template.js:7 |  |  | 0.606 |
| walker |  | 8854 | 72 | ts body src/dockerUtil.js:78 |  |  | 0.607 |
| ns | 8893 |  | 310 | src/themes/theme.selector.js in full and the dark/light style keys | 7.2 |  | 0.603 |
| walker |  | 8903 | 49 | ts body src/widgetsTemplates/list.widget.template.js:119 |  |  | 0.604 |
| walker |  | 8925 | 22 | ts names docs/src/components/Header.js |  |  | 0.604 |
| walker |  | 8947 | 22 | ts names docs/src/components/Scroll.js |  |  | 0.604 |
| walker |  | 8969 | 22 | ts names docs/src/components/layout.js |  |  | 0.604 |
| walker |  | 8991 | 22 | ts names docs/src/pages/generic.js |  |  | 0.604 |
| walker |  | 9004 | 13 | ts decl docs/src/pages/generic.js:8 |  |  | 0.604 |
| walker |  | 9026 | 22 | ts names docs/src/pages/index.js |  |  | 0.604 |
| walker |  | 9100 | 74 | headings outline in CODE_OF_CONDUCT.md |  |  | 0.604 |
| walker |  | 9100 | 0 | CODE_OF_CONDUCT.md section #0 |  |  | 0.604 |
| walker |  | 9124 | 24 | ts names docs/src/components/Footer.js |  |  | 0.604 |
| walker |  | 9148 | 24 | ts names docs/src/components/Nav.js |  |  | 0.604 |
| walker |  | 9174 | 26 | ts names docs/src/components/HeaderGeneric.js |  |  | 0.604 |
| ns | 9179 |  | 286 | widgets/containers/containerList.widget.js: complete method roster | 7.3 |  | 0.596 |
| walker |  | 9200 | 26 | ts names docs/src/pages/404.js |  |  | 0.596 |
| walker |  | 9320 | 120 | ts body src/screen.js:35 |  |  | 0.606 |
| walker |  | 9402 | 82 | ts body src/dockerUtil.js:43 |  |  | 0.607 |
| walker |  | 9428 | 26 | listing of 'docs/src/assets/scss' |  |  | 0.607 |
| walker |  | 9439 | 11 | listing of 'docs/src/assets/scss/base' |  |  | 0.607 |
| ns | 9448 |  | 269 | widgets/toolbar.widget.js: the per-mode command extension map | 7.4 |  | 0.599 |
| walker |  | 9523 | 84 | CONTRIBUTING.md section #0 |  |  | 0.599 |
| ns | 9596 |  | 148 | widgets/actionsMenu.widget.js: the `m` menu's action table | 7.5 |  | 0.594 |
| ns | 9640 |  | 44 | Complete listings of .github/ and its subdirectories | 8.1 |  | 0.597 |
| walker |  | 9649 | 126 | ts body src/screen.js:19 |  |  | 0.609 |
| walker |  | 9735 | 86 | ts body src/dockerUtil.js:88 |  |  | 0.610 |
| ns | 9739 |  | 99 | Complete listings of the docs/ Gatsby site | 8.2 |  | 0.617 |
| walker |  | 9757 | 22 | listing of 'docs/src/assets/scss/libs' |  |  | 0.617 |
| walker |  | 9843 | 86 | ts body src/dockerUtil.js:100 |  |  | 0.618 |
| walker |  | 9868 | 25 | listing of 'docs/src/assets/scss/layout' |  |  | 0.618 |
| ns | 9988 |  | 249 | Secondary ops config: codefresh.yml, devcontainer image, VS Code attach config, .nvmrc | 8.3 |  | 0.611 |

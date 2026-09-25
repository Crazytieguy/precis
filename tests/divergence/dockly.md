Score(3000)=0.635 I=0.881 C=0.458 ns_rows≤3K=22/55 grid(1000/1442/2080/3000/4327/6240/9000)=0.648/0.582/0.671/0.635/0.561/0.530/0.608

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
| walker |  | 544 | 14 | ts names src/themes/theme.selector.js |  |  | 0.703 |
| walker |  | 589 | 45 | listing of 'widgets/containers' |  |  | 0.785 |
| walker |  | 622 | 33 | ts decl lib/modes.js:3 |  |  | 0.787 |
| ns | 630 |  | 97 | Published `files` allow-list and the node engines floor | 1.7 |  | 0.729 |
| walker |  | 656 | 34 | package runtime metadata in package.json |  |  | 0.734 |
| walker |  | 675 | 19 | listing of '.github' |  |  | 0.735 |
| walker |  | 695 | 20 | listing of '.github/workflows' |  |  | 0.735 |
| ns | 759 |  | 129 | README tagline plus every top-level heading location | 1.8 |  | 0.683 |
| walker |  | 835 | 140 | README headline in README.md |  |  | 0.688 |
| walker |  | 858 | 23 | README prelude in README.md |  |  | 0.688 |
| ns | 891 |  | 132 | index.js: shebang and the module wiring of the executable | 1.9 |  | 0.648 |
| walker |  | 900 | 42 | ts decl src/assetsLoader.js:10 |  |  | 0.648 |
| walker |  | 926 | 26 | ts names src/widgetsTemplates/help.widget.template.js |  |  | 0.648 |
| walker |  | 952 | 26 | ts names src/widgetsTemplates/info.widget.template.js |  |  | 0.648 |
| walker |  | 978 | 26 | ts names src/widgetsTemplates/logs.widget.template.js |  |  | 0.648 |
| walker |  | 1006 | 28 | ts names src/widgetsTemplates/base.hook.template.js |  |  | 0.648 |
| walker |  | 1034 | 28 | ts names src/widgetsTemplates/list.widget.template.js |  |  | 0.648 |
| ns | 1053 |  | 162 | index.js: pre-flight CLI dispatch (--help, --version, node version floor) | 1.10 | 1.9 | 0.601 |
| walker |  | 1063 | 29 | ts decl src/widgetsTemplates/base.hook.template.js:6 |  |  | 0.601 |
| walker |  | 1127 | 64 | ts names src/cli.js |  |  | 0.602 |
| ns | 1219 |  | 166 | index.js: bootstrap promise chain and the three helper function signatures | 1.11 | 1.10 | 0.558 |
| walker |  | 1230 | 103 | headings outline in README.md |  |  | 0.610 |
| walker |  | 1252 | 22 | README.md section #0 |  |  | 0.610 |
| walker |  | 1265 | 13 | ts body src/cli.js:93 |  |  | 0.611 |
| walker |  | 1374 | 109 | plaintext config dockerRunScript.sh |  |  | 0.611 |
| ns | 1414 |  | 195 | Runtime dependency list from package.json | 1.12 |  | 0.582 |
| walker |  | 1465 | 91 | ts decl src/enum.js:1 |  |  | 0.583 |
| walker |  | 1635 | 170 | plaintext config Dockerfile |  |  | 0.587 |
| walker |  | 1686 | 51 | ts names src/themes/styles.js |  |  | 0.587 |
| ns | 1737 |  | 323 | The in-app keybinding table (src/widgetsTemplates/help.widget.template.js:102-125) | 2.1 |  | 0.542 |
| walker |  | 1738 | 52 | listing of 'docs' |  |  | 0.542 |
| walker |  | 1747 | 9 | listing of 'docs/src' |  |  | 0.542 |
| walker |  | 1854 | 107 | package entrypoints in package.json |  |  | 0.690 |
| ns | 1857 |  | 120 | src/cli.js: the complete flag roster (all eight option names) | 2.2 |  | 0.669 |
| ns | 2043 |  | 186 | src/cli.js: the four docker-connection options in full (socketPath, host, port, protocol) | 2.3 | 2.2 | 0.626 |
| walker |  | 2049 | 195 | package runtime dependencies in package.json |  |  | 0.671 |
| walker |  | 2062 | 13 | listing of 'docs/src/pages' |  |  | 0.671 |
| walker |  | 2182 | 120 | package scripts in package.json |  |  | 0.698 |
| ns | 2206 |  | 163 | src/cli.js: the four behavioural options in full (help, version, containerFilters, theme) | 2.4 | 2.2 | 0.657 |
| ns | 2269 |  | 63 | src/cli.js: every prototype method and the singleton export | 2.5 |  | 0.660 |
| ns | 2339 |  | 70 | README: install and launch commands | 2.6 |  | 0.650 |
| walker |  | 2354 | 172 | ts decl src/screen.js:18 |  |  | 0.652 |
| walker |  | 2363 | 9 | ts body src/screen.js:192 |  |  | 0.653 |
| walker |  | 2378 | 15 | ts body src/assetsLoader.js:39 |  |  | 0.653 |
| ns | 2455 |  | 116 | README: --containerFilters semantics | 2.7 |  | 0.647 |
| walker |  | 2469 | 91 | ts decl src/widgetsTemplates/logs.widget.template.js:7 |  |  | 0.647 |
| walker |  | 2494 | 25 | listing of 'docs/src/components' |  |  | 0.648 |
| walker |  | 2527 | 33 | ts doc src/cli.js:93 |  |  | 0.648 |
| ns | 2625 |  | 170 | Dockerfile in full | 2.8 |  | 0.666 |
| walker |  | 2639 | 112 | ts decl src/widgetsTemplates/info.widget.template.js:6 |  |  | 0.667 |
| walker |  | 2647 | 8 | ts body src/widgetsTemplates/info.widget.template.js:107 |  |  | 0.667 |
| ns | 2724 |  | 99 | README: running and building the docker image | 2.9 |  | 0.656 |
| walker |  | 2771 | 124 | ts decl src/widgetsTemplates/help.widget.template.js:5 |  |  | 0.656 |
| walker |  | 2779 | 8 | ts body src/widgetsTemplates/help.widget.template.js:130 |  |  | 0.656 |
| walker |  | 2800 | 21 | ts body src/assetsLoader.js:11 |  |  | 0.656 |
| walker |  | 2811 | 11 | ts body src/widgetsTemplates/logs.widget.template.js:76 |  |  | 0.656 |
| walker |  | 2822 | 11 | ts body src/widgetsTemplates/logs.widget.template.js:80 |  |  | 0.656 |
| ns | 2876 |  | 152 | .github/workflows/main.yml: the lint job | 2.10 |  | 0.635 |
| ns | 3087 |  | 211 | .github/workflows/main.yml: the semantic-release job | 2.11 |  | 0.616 |
| walker |  | 3119 | 297 | ts decl src/dockerUtil.js:5 |  |  | 0.618 |
| walker |  | 3145 | 26 | ts body src/dockerUtil.js:165 |  |  | 0.618 |
| ns | 3170 |  | 83 | lib/modes.js and lib/node.version.js in full | 3.1 |  | 0.623 |
| walker |  | 3171 | 26 | ts body src/dockerUtil.js:170 |  |  | 0.623 |
| walker |  | 3197 | 26 | ts body src/dockerUtil.js:175 |  |  | 0.623 |
| walker |  | 3212 | 15 | ts body src/widgetsTemplates/help.widget.template.js:134 |  |  | 0.623 |
| walker |  | 3227 | 15 | ts body src/widgetsTemplates/info.widget.template.js:111 |  |  | 0.623 |
| walker |  | 3242 | 15 | ts body src/widgetsTemplates/logs.widget.template.js:72 |  |  | 0.623 |
| ns | 3375 |  | 205 | src/screen.js: imports and the mode -> grid-layout table | 3.2 |  | 0.607 |
| walker |  | 3419 | 177 | ts decl src/widgetsTemplates/list.widget.template.js:8 |  |  | 0.608 |
| walker |  | 3434 | 15 | ts body src/widgetsTemplates/list.widget.template.js:127 |  |  | 0.609 |
| walker |  | 3449 | 15 | ts body src/widgetsTemplates/list.widget.template.js:147 |  |  | 0.609 |
| walker |  | 3464 | 15 | ts body src/widgetsTemplates/list.widget.template.js:151 |  |  | 0.609 |
| walker |  | 3480 | 16 | ts body src/widgetsTemplates/base.hook.template.js:29 |  |  | 0.609 |
| walker |  | 3512 | 32 | ts body src/screen.js:126 |  |  | 0.609 |
| walker |  | 3544 | 32 | ts body src/screen.js:132 |  |  | 0.609 |
| ns | 3573 |  | 198 | src/screen.js: complete method roster of the `screen` class | 3.3 |  | 0.614 |
| walker |  | 3605 | 61 | ts body src/cli.js:59 |  |  | 0.616 |
| walker |  | 3622 | 17 | ts body src/widgetsTemplates/help.widget.template.js:50 |  |  | 0.616 |
| walker |  | 3639 | 17 | ts body src/widgetsTemplates/info.widget.template.js:64 |  |  | 0.616 |
| ns | 3770 |  | 197 | src/screen.js: init() — the full boot sequence | 3.4 | 3.3 | 0.598 |
| walker |  | 3845 | 206 | ts decl src/themes/styles.js:28 |  |  | 0.598 |
| ns | 3900 |  | 130 | src/screen.js: constructor state fields | 3.5 | 3.3 | 0.586 |
| ns | 4024 |  | 124 | src/screen.js: initScreen() — blessed screen and the 12x12 grid | 3.6 | 3.3 | 0.577 |
| walker |  | 4052 | 207 | ts decl src/themes/styles.js:1 |  |  | 0.577 |
| walker |  | 4147 | 95 | ts body index.js:72 |  |  | 0.578 |
| walker |  | 4174 | 27 | ts body src/dockerUtil.js:155 |  |  | 0.578 |
| walker |  | 4189 | 15 | ts body src/widgetsTemplates/logs.widget.template.js:84 |  |  | 0.578 |
| walker |  | 4194 | 5 | listing of '.github/ISSUE_TEMPLATE' |  |  | 0.578 |
| walker |  | 4210 | 16 | ts body src/widgetsTemplates/list.widget.template.js:131 |  |  | 0.578 |
| walker |  | 4227 | 17 | ts body src/widgetsTemplates/help.widget.template.js:54 |  |  | 0.578 |
| walker |  | 4244 | 17 | ts body src/widgetsTemplates/info.widget.template.js:68 |  |  | 0.578 |
| walker |  | 4280 | 36 | ts body src/screen.js:120 |  |  | 0.580 |
| ns | 4286 |  | 262 | src/screen.js: initHooks() and initWidgets() — instantiation and layout gating | 3.7 | 3.3 | 0.561 |
| walker |  | 4419 | 139 | ts body index.js:86 |  |  | 0.563 |
| walker |  | 4446 | 27 | ts body src/dockerUtil.js:160 |  |  | 0.563 |
| ns | 4580 |  | 294 | src/screen.js: toggleMode() and registerEvents() — the global q/v keys and focus borders | 3.8 | 3.3 | 0.544 |
| walker |  | 4606 | 160 | README.md section #2 |  |  | 0.558 |
| ns | 4668 |  | 88 | src/assetsLoader.js: the glob patterns that define the plugin conventions | 3.9 |  | 0.552 |
| walker |  | 4751 | 145 | README.md section #4 |  |  | 0.552 |
| walker |  | 4767 | 16 | ts body src/widgetsTemplates/list.widget.template.js:135 |  |  | 0.552 |
| ns | 4871 |  | 203 | src/assetsLoader.js: the loader body and the asset naming rule | 3.10 | 3.9 | 0.546 |
| ns | 5149 |  | 278 | src/grid.config.js: the containers-mode layout in full | 3.11 |  | 0.537 |
| walker |  | 5234 | 467 | package identity metadata in package.json |  |  | 0.537 |
| walker |  | 5303 | 69 | ts body src/themes/theme.selector.js:13 |  |  | 0.538 |
| walker |  | 5487 | 184 | ts body index.js:47 |  |  | 0.538 |
| walker |  | 5524 | 37 | ts body src/screen.js:114 |  |  | 0.540 |
| walker |  | 5544 | 20 | ts body src/widgetsTemplates/help.widget.template.js:138 |  |  | 0.540 |
| ns | 5554 |  | 405 | src/grid.config.js: the services and images layouts plus the export block | 3.12 | 3.11 | 0.526 |
| walker |  | 5564 | 20 | ts body src/widgetsTemplates/info.widget.template.js:115 |  |  | 0.526 |
| walker |  | 5580 | 16 | ts body src/widgetsTemplates/list.widget.template.js:139 |  |  | 0.526 |
| walker |  | 5764 | 184 | ts body src/cli.js:66 |  |  | 0.528 |
| ns | 5884 |  | 330 | src/dockerUtil.js: complete method roster of the dockerode wrapper | 4.1 |  | 0.538 |
| walker |  | 5950 | 186 | ts body src/baseWidget.js:4 |  |  | 0.539 |
| walker |  | 5966 | 16 | ts body src/widgetsTemplates/list.widget.template.js:143 |  |  | 0.539 |
| walker |  | 6008 | 42 | ts body src/dockerUtil.js:180 |  |  | 0.541 |
| walker |  | 6059 | 51 | ts body src/screen.js:107 |  |  | 0.542 |
| ns | 6164 |  | 280 | src/dockerUtil.js: constructor — connection config and containerFilters parsing | 4.2 | 4.1 | 0.530 |
| ns | 6314 |  | 150 | src/dockerUtil.js: getInfo() — the host fields surfaced to the status widgets | 4.3 | 4.1 | 0.524 |
| ns | 6429 |  | 115 | src/dockerUtil.js: log streaming options for containers and services | 4.4 | 4.1 | 0.518 |
| walker |  | 6438 | 379 | README.md section #1 |  |  | 0.534 |
| walker |  | 6480 | 42 | ts body src/dockerUtil.js:187 |  |  | 0.535 |
| ns | 6656 |  | 227 | src/baseWidget.js — the mixin every widget and hook extends | 5.1 |  | 0.546 |
| walker |  | 6770 | 290 | README.md section #5 |  |  | 0.546 |
| ns | 6888 |  | 232 | src/widgetsTemplates/base.hook.template.js — the hook base class | 5.2 |  | 0.537 |
| walker |  | 6969 | 199 | ts body src/assetsLoader.js:16 |  |  | 0.552 |
| walker |  | 7023 | 54 | ts body src/screen.js:79 |  |  | 0.553 |
| walker |  | 7045 | 22 | ts body src/widgetsTemplates/list.widget.template.js:155 |  |  | 0.553 |
| walker |  | 7058 | 13 | listing of 'docs/src/assets' |  |  | 0.553 |
| walker |  | 7064 | 6 | listing of 'docs/src/assets/css' |  |  | 0.553 |
| ns | 7121 |  | 233 | src/widgetsTemplates/list.widget.template.js: complete method roster | 5.3 |  | 0.558 |
| walker |  | 7130 | 66 | package identity in docs/package.json |  |  | 0.558 |
| walker |  | 7137 | 7 | plaintext config .nvmrc |  |  | 0.558 |
| walker |  | 7174 | 37 | headings outline in SECURITY.md |  |  | 0.558 |
| walker |  | 7174 | 0 | SECURITY.md section #0 |  |  | 0.558 |
| walker |  | 7237 | 63 | ts body src/dockerUtil.js:55 |  |  | 0.559 |
| walker |  | 7315 | 78 | ts body src/widgetsTemplates/logs.widget.template.js:8 |  |  | 0.559 |
| ns | 7409 |  | 288 | src/widgetsTemplates/info.widget.template.js and logs.widget.template.js: method rosters | 5.4 |  | 0.565 |
| walker |  | 7493 | 178 | ts body src/widgetsTemplates/base.hook.template.js:7 |  |  | 0.577 |
| ns | 7570 |  | 161 | src/widgetsTemplates/help.widget.template.js: method roster around the help text | 5.5 |  | 0.575 |
| ns | 7919 |  | 349 | hooks/: complete method rosters of all four hooks | 6.1 |  | 0.561 |
| walker |  | 8075 | 582 | README.md section #3 |  |  | 0.561 |
| ns | 8128 |  | 209 | hooks/containers.hook.js: the toolbar key dispatch inside init() | 6.2 | 6.1 | 0.552 |
| ns | 8482 |  | 354 | hooks/shell.hook.js: openShell() plus dockerRunScript.sh's docker exec line | 6.3 | 6.1 | 0.543 |
| walker |  | 8527 | 452 | ts body src/cli.js:8 |  |  | 0.604 |
| ns | 8583 |  | 101 | src/enum.js in full — the ContainerState vocabulary | 7.1 |  | 0.608 |
| walker |  | 8590 | 63 | README headline in docs/README.md |  |  | 0.608 |
| walker |  | 8608 | 18 | headings outline in docs/README.md |  |  | 0.608 |
| walker |  | 8681 | 73 | ts body src/screen.js:71 |  |  | 0.611 |
| walker |  | 8747 | 66 | ts body src/dockerUtil.js:33 |  |  | 0.611 |
| walker |  | 8763 | 16 | docs/README.md section #1 |  |  | 0.611 |
| walker |  | 8817 | 54 | headings outline in CONTRIBUTING.md |  |  | 0.611 |
| walker |  | 8853 | 36 | headings outline in .github/PULL_REQUEST_TEMPLATE.md |  |  | 0.611 |
| walker |  | 8853 | 0 | .github/PULL_REQUEST_TEMPLATE.md section #0 |  |  | 0.611 |
| walker |  | 8884 | 31 | package dev/peer dependencies in package.json |  |  | 0.611 |
| ns | 8893 |  | 310 | src/themes/theme.selector.js in full and the dark/light style keys | 7.2 |  | 0.608 |
| walker |  | 8925 | 41 | docs/README.md section #0 |  |  | 0.608 |
| walker |  | 9020 | 95 | ts body src/widgetsTemplates/help.widget.template.js:6 |  |  | 0.608 |
| walker |  | 9115 | 95 | ts body src/widgetsTemplates/info.widget.template.js:7 |  |  | 0.609 |
| ns | 9179 |  | 286 | widgets/containers/containerList.widget.js: complete method roster | 7.3 |  | 0.601 |
| walker |  | 9187 | 72 | ts body src/dockerUtil.js:78 |  |  | 0.602 |
| walker |  | 9236 | 49 | ts body src/widgetsTemplates/list.widget.template.js:119 |  |  | 0.603 |
| walker |  | 9258 | 22 | ts names docs/src/components/Header.js |  |  | 0.603 |
| walker |  | 9280 | 22 | ts names docs/src/components/Scroll.js |  |  | 0.603 |
| walker |  | 9302 | 22 | ts names docs/src/components/layout.js |  |  | 0.603 |
| walker |  | 9324 | 22 | ts names docs/src/pages/generic.js |  |  | 0.603 |
| walker |  | 9337 | 13 | ts decl docs/src/pages/generic.js:8 |  |  | 0.603 |
| walker |  | 9359 | 22 | ts names docs/src/pages/index.js |  |  | 0.603 |
| walker |  | 9433 | 74 | headings outline in CODE_OF_CONDUCT.md |  |  | 0.603 |
| walker |  | 9433 | 0 | CODE_OF_CONDUCT.md section #0 |  |  | 0.603 |
| ns | 9448 |  | 269 | widgets/toolbar.widget.js: the per-mode command extension map | 7.4 |  | 0.595 |
| walker |  | 9457 | 24 | ts names docs/src/components/Footer.js |  |  | 0.595 |
| walker |  | 9481 | 24 | ts names docs/src/components/Nav.js |  |  | 0.595 |
| walker |  | 9507 | 26 | ts names docs/src/components/HeaderGeneric.js |  |  | 0.595 |
| walker |  | 9533 | 26 | ts names docs/src/pages/404.js |  |  | 0.595 |
| ns | 9596 |  | 148 | widgets/actionsMenu.widget.js: the `m` menu's action table | 7.5 |  | 0.591 |
| ns | 9640 |  | 44 | Complete listings of .github/ and its subdirectories | 8.1 |  | 0.594 |
| walker |  | 9653 | 120 | ts body src/screen.js:35 |  |  | 0.603 |
| walker |  | 9735 | 82 | ts body src/dockerUtil.js:43 |  |  | 0.604 |
| ns | 9739 |  | 99 | Complete listings of the docs/ Gatsby site | 8.2 |  | 0.611 |
| walker |  | 9761 | 26 | listing of 'docs/src/assets/scss' |  |  | 0.611 |
| walker |  | 9772 | 11 | listing of 'docs/src/assets/scss/base' |  |  | 0.611 |
| walker |  | 9856 | 84 | CONTRIBUTING.md section #0 |  |  | 0.611 |
| walker |  | 9982 | 126 | ts body src/screen.js:19 |  |  | 0.623 |
| ns | 9988 |  | 249 | Secondary ops config: codefresh.yml, devcontainer image, VS Code attach config, .nvmrc | 8.3 |  | 0.615 |

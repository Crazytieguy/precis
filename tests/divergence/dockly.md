Score(3000)=0.447 I=0.613 C=0.327 ns_rows≤3K=22/55 grid(1000/1442/2080/3000/4327/6240/9000)=0.644/0.531/0.486/0.447/0.509/0.545/0.537

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
| walker |  | 575 | 23 | ts names hooks/containers.hook.js |  |  | 0.703 |
| walker |  | 598 | 23 | ts names hooks/images.hook.js |  |  | 0.703 |
| walker |  | 621 | 23 | ts names hooks/services.hook.js |  |  | 0.703 |
| ns | 630 |  | 97 | Published `files` allow-list and the node engines floor | 1.7 |  | 0.651 |
| walker |  | 645 | 24 | ts names hooks/shell.hook.js |  |  | 0.652 |
| walker |  | 670 | 25 | ts names widgets/help.widget.js |  |  | 0.652 |
| walker |  | 684 | 14 | ts decl widgets/help.widget.js:5 |  |  | 0.652 |
| walker |  | 710 | 26 | ts names widgets/actionsMenu.widget.js |  |  | 0.652 |
| walker |  | 724 | 14 | ts names src/themes/theme.selector.js |  |  | 0.652 |
| walker |  | 752 | 28 | ts names widgets/actionStatus.widget.js |  |  | 0.652 |
| ns | 759 |  | 129 | README tagline plus every top-level heading location | 1.8 |  | 0.606 |
| walker |  | 780 | 28 | ts names widgets/searchInput.widget.js |  |  | 0.606 |
| walker |  | 808 | 28 | ts names widgets/toolbar.widget.js |  |  | 0.606 |
| walker |  | 853 | 45 | listing of 'widgets/containers' |  |  | 0.676 |
| walker |  | 886 | 33 | ts decl lib/modes.js:3 |  |  | 0.678 |
| ns | 891 |  | 132 | index.js: shebang and the module wiring of the executable | 1.9 |  | 0.639 |
| walker |  | 920 | 34 | package runtime metadata in package.json |  |  | 0.643 |
| walker |  | 939 | 19 | listing of '.github' |  |  | 0.644 |
| walker |  | 959 | 20 | listing of '.github/workflows' |  |  | 0.644 |
| ns | 1053 |  | 162 | index.js: pre-flight CLI dispatch (--help, --version, node version floor) | 1.10 | 1.9 | 0.597 |
| walker |  | 1099 | 140 | README headline in README.md |  |  | 0.601 |
| walker |  | 1122 | 23 | README prelude in README.md |  |  | 0.601 |
| walker |  | 1162 | 40 | ts decl hooks/shell.hook.js:8 |  |  | 0.601 |
| walker |  | 1204 | 42 | ts decl src/assetsLoader.js:10 |  |  | 0.601 |
| ns | 1219 |  | 166 | index.js: bootstrap promise chain and the three helper function signatures | 1.11 | 1.10 | 0.558 |
| walker |  | 1229 | 25 | ts names widgets/containers/containerInfo.widget.js |  |  | 0.558 |
| walker |  | 1254 | 25 | ts names widgets/containers/containerList.widget.js |  |  | 0.558 |
| walker |  | 1279 | 25 | ts names widgets/containers/containerLogs.widget.js |  |  | 0.558 |
| walker |  | 1304 | 25 | ts names widgets/images/imageInfo.widget.js |  |  | 0.558 |
| walker |  | 1329 | 25 | ts names widgets/images/imageList.widget.js |  |  | 0.558 |
| walker |  | 1354 | 25 | ts names widgets/services/servicesInfo.widget.js |  |  | 0.558 |
| walker |  | 1379 | 25 | ts names widgets/services/servicesList.widget.js |  |  | 0.558 |
| walker |  | 1404 | 25 | ts names widgets/services/servicesLogs.widget.js |  |  | 0.558 |
| ns | 1414 |  | 195 | Runtime dependency list from package.json | 1.12 |  | 0.531 |
| walker |  | 1430 | 26 | ts names src/widgetsTemplates/help.widget.template.js |  |  | 0.531 |
| walker |  | 1456 | 26 | ts names src/widgetsTemplates/info.widget.template.js |  |  | 0.531 |
| walker |  | 1482 | 26 | ts names src/widgetsTemplates/logs.widget.template.js |  |  | 0.531 |
| walker |  | 1508 | 26 | ts names widgets/containers/containerStatus.widget.js |  |  | 0.531 |
| walker |  | 1534 | 26 | ts names widgets/containers/containerUtilization.widget.js |  |  | 0.531 |
| walker |  | 1560 | 26 | ts names widgets/containers/containerVsImages.widget.js |  |  | 0.531 |
| walker |  | 1586 | 26 | ts names widgets/images/imageUtilization.widget.js |  |  | 0.531 |
| walker |  | 1612 | 26 | ts names widgets/services/servicesStatus.widget.js |  |  | 0.531 |
| walker |  | 1638 | 26 | ts names widgets/services/servicesVsImages.widget.js |  |  | 0.531 |
| walker |  | 1663 | 25 | ts decl widgets/containers/containerLogs.widget.js:5 |  |  | 0.531 |
| walker |  | 1688 | 25 | ts decl widgets/services/servicesLogs.widget.js:5 |  |  | 0.531 |
| walker |  | 1737 | 49 | ts decl hooks/images.hook.js:5 |  |  | 0.490 |
| ns | 1737 |  | 323 | The in-app keybinding table (src/widgetsTemplates/help.widget.template.js:102-125) | 2.1 |  | 0.490 |
| walker |  | 1765 | 28 | ts names src/widgetsTemplates/base.hook.template.js |  |  | 0.490 |
| walker |  | 1793 | 28 | ts names src/widgetsTemplates/list.widget.template.js |  |  | 0.490 |
| walker |  | 1823 | 30 | ts names widgets/containers/containerSortList.widget.js |  |  | 0.490 |
| walker |  | 1852 | 29 | ts decl src/widgetsTemplates/base.hook.template.js:6 |  |  | 0.490 |
| ns | 1857 |  | 120 | src/cli.js: the complete flag roster (all eight option names) | 2.2 |  | 0.476 |
| walker |  | 1911 | 59 | ts decl widgets/toolbar.widget.js:7 |  |  | 0.476 |
| walker |  | 1975 | 64 | ts decl hooks/services.hook.js:5 |  |  | 0.476 |
| ns | 2043 |  | 186 | src/cli.js: the four docker-connection options in full (socketPath, host, port, protocol) | 2.3 | 2.2 | 0.445 |
| walker |  | 2078 | 103 | headings outline in README.md |  |  | 0.486 |
| walker |  | 2100 | 22 | README.md section #0 |  |  | 0.487 |
| walker |  | 2170 | 70 | ts decl widgets/actionStatus.widget.js:6 |  |  | 0.487 |
| ns | 2206 |  | 163 | src/cli.js: the four behavioural options in full (help, version, containerFilters, theme) | 2.4 | 2.2 | 0.458 |
| walker |  | 2252 | 82 | ts names src/grid.config.js |  |  | 0.458 |
| ns | 2269 |  | 63 | src/cli.js: every prototype method and the singleton export | 2.5 |  | 0.451 |
| walker |  | 2334 | 82 | ts decl widgets/searchInput.widget.js:9 |  |  | 0.451 |
| ns | 2339 |  | 70 | README: install and launch commands | 2.6 |  | 0.445 |
| walker |  | 2341 | 7 | ts body widgets/searchInput.widget.js:24 |  |  | 0.445 |
| walker |  | 2386 | 45 | ts decl widgets/containers/containerInfo.widget.js:5 |  |  | 0.445 |
| walker |  | 2431 | 45 | ts decl widgets/images/imageInfo.widget.js:5 |  |  | 0.445 |
| ns | 2455 |  | 116 | README: --containerFilters semantics | 2.7 |  | 0.441 |
| walker |  | 2476 | 45 | ts decl widgets/services/servicesInfo.widget.js:5 |  |  | 0.441 |
| walker |  | 2585 | 109 | plaintext config dockerRunScript.sh |  |  | 0.441 |
| ns | 2625 |  | 170 | Dockerfile in full | 2.8 |  | 0.426 |
| walker |  | 2676 | 91 | ts decl src/enum.js:1 |  |  | 0.427 |
| walker |  | 2684 | 8 | ts body widgets/actionStatus.widget.js:24 |  |  | 0.427 |
| walker |  | 2692 | 8 | ts body widgets/searchInput.widget.js:28 |  |  | 0.427 |
| walker |  | 2700 | 8 | ts body widgets/toolbar.widget.js:21 |  |  | 0.427 |
| ns | 2724 |  | 99 | README: running and building the docker image | 2.9 |  | 0.420 |
| walker |  | 2870 | 170 | plaintext config Dockerfile |  |  | 0.462 |
| ns | 2876 |  | 152 | .github/workflows/main.yml: the lint job | 2.10 |  | 0.447 |
| walker |  | 2921 | 51 | ts names src/themes/styles.js |  |  | 0.447 |
| walker |  | 2973 | 52 | listing of 'docs' |  |  | 0.447 |
| walker |  | 2982 | 9 | listing of 'docs/src' |  |  | 0.447 |
| walker |  | 3082 | 100 | ts decl hooks/containers.hook.js:5 |  |  | 0.448 |
| ns | 3087 |  | 211 | .github/workflows/main.yml: the semantic-release job | 2.11 |  | 0.435 |
| walker |  | 3091 | 9 | ts body widgets/toolbar.widget.js:25 |  |  | 0.435 |
| ns | 3170 |  | 83 | lib/modes.js and lib/node.version.js in full | 3.1 |  | 0.443 |
| walker |  | 3197 | 106 | ts decl widgets/actionsMenu.widget.js:5 |  |  | 0.443 |
| walker |  | 3205 | 8 | ts body widgets/actionsMenu.widget.js:148 |  |  | 0.443 |
| walker |  | 3312 | 107 | package entrypoints in package.json |  |  | 0.553 |
| ns | 3375 |  | 205 | src/screen.js: imports and the mode -> grid-layout table | 3.2 |  | 0.539 |
| walker |  | 3507 | 195 | package runtime dependencies in package.json |  |  | 0.572 |
| walker |  | 3520 | 13 | listing of 'docs/src/pages' |  |  | 0.572 |
| ns | 3573 |  | 198 | src/screen.js: complete method roster of the `screen` class | 3.3 |  | 0.543 |
| walker |  | 3579 | 59 | ts decl widgets/containers/containerStatus.widget.js:5 |  |  | 0.543 |
| walker |  | 3638 | 59 | ts decl widgets/containers/containerUtilization.widget.js:4 |  |  | 0.543 |
| walker |  | 3697 | 59 | ts decl widgets/containers/containerVsImages.widget.js:4 |  |  | 0.543 |
| walker |  | 3756 | 59 | ts decl widgets/images/imageUtilization.widget.js:5 |  |  | 0.543 |
| ns | 3770 |  | 197 | src/screen.js: init() — the full boot sequence | 3.4 | 3.3 | 0.526 |
| walker |  | 3815 | 59 | ts decl widgets/services/servicesStatus.widget.js:5 |  |  | 0.526 |
| walker |  | 3874 | 59 | ts decl widgets/services/servicesVsImages.widget.js:4 |  |  | 0.526 |
| walker |  | 3885 | 11 | ts body widgets/help.widget.js:6 |  |  | 0.526 |
| ns | 3900 |  | 130 | src/screen.js: constructor state fields | 3.5 | 3.3 | 0.516 |
| walker |  | 4005 | 120 | package scripts in package.json |  |  | 0.534 |
| ns | 4024 |  | 124 | src/screen.js: initScreen() — blessed screen and the 12x12 grid | 3.6 | 3.3 | 0.526 |
| walker |  | 4156 | 151 | ts decl src/grid.config.js:29 |  |  | 0.526 |
| ns | 4286 |  | 262 | src/screen.js: initHooks() and initWidgets() — instantiation and layout gating | 3.7 | 3.3 | 0.509 |
| walker |  | 4328 | 172 | ts decl src/screen.js:18 |  |  | 0.537 |
| walker |  | 4337 | 9 | ts body src/screen.js:192 |  |  | 0.541 |
| walker |  | 4352 | 15 | ts body src/assetsLoader.js:39 |  |  | 0.541 |
| walker |  | 4443 | 91 | ts decl src/widgetsTemplates/logs.widget.template.js:7 |  |  | 0.541 |
| walker |  | 4468 | 25 | listing of 'docs/src/components' |  |  | 0.542 |
| ns | 4580 |  | 294 | src/screen.js: toggleMode() and registerEvents() — the global q/v keys and focus borders | 3.8 | 3.3 | 0.523 |
| ns | 4668 |  | 88 | src/assetsLoader.js: the glob patterns that define the plugin conventions | 3.9 |  | 0.518 |
| walker |  | 4684 | 216 | ts decl src/grid.config.js:16 |  |  | 0.520 |
| walker |  | 4796 | 112 | ts decl src/widgetsTemplates/info.widget.template.js:6 |  |  | 0.521 |
| walker |  | 4804 | 8 | ts body src/widgetsTemplates/info.widget.template.js:107 |  |  | 0.521 |
| walker |  | 4814 | 10 | ts body widgets/containers/containerInfo.widget.js:6 |  |  | 0.521 |
| walker |  | 4824 | 10 | ts body widgets/containers/containerLogs.widget.js:6 |  |  | 0.521 |
| walker |  | 4834 | 10 | ts body widgets/images/imageInfo.widget.js:6 |  |  | 0.521 |
| walker |  | 4844 | 10 | ts body widgets/services/servicesInfo.widget.js:6 |  |  | 0.521 |
| walker |  | 4854 | 10 | ts body widgets/services/servicesLogs.widget.js:6 |  |  | 0.521 |
| ns | 4871 |  | 203 | src/assetsLoader.js: the loader body and the asset naming rule | 3.10 | 3.9 | 0.512 |
| walker |  | 4973 | 119 | ts decl widgets/containers/containerSortList.widget.js:4 |  |  | 0.512 |
| walker |  | 4977 | 4 | ts decl widgets/containers/containerSortList.widget.js:99 |  |  | 0.512 |
| walker |  | 5101 | 124 | ts decl src/widgetsTemplates/help.widget.template.js:5 |  |  | 0.512 |
| walker |  | 5109 | 8 | ts body src/widgetsTemplates/help.widget.template.js:130 |  |  | 0.512 |
| walker |  | 5130 | 21 | ts body src/assetsLoader.js:11 |  |  | 0.515 |
| walker |  | 5141 | 11 | ts body src/widgetsTemplates/logs.widget.template.js:76 |  |  | 0.515 |
| ns | 5149 |  | 278 | src/grid.config.js: the containers-mode layout in full | 3.11 |  | 0.507 |
| walker |  | 5152 | 11 | ts body src/widgetsTemplates/logs.widget.template.js:80 |  |  | 0.507 |
| walker |  | 5284 | 132 | ts decl widgets/services/servicesList.widget.js:5 |  |  | 0.507 |
| walker |  | 5293 | 9 | ts body widgets/services/servicesList.widget.js:11 |  |  | 0.507 |
| walker |  | 5428 | 135 | ts decl widgets/images/imageList.widget.js:5 |  |  | 0.507 |
| walker |  | 5437 | 9 | ts body widgets/images/imageList.widget.js:11 |  |  | 0.507 |
| ns | 5554 |  | 405 | src/grid.config.js: the services and images layouts plus the export block | 3.12 | 3.11 | 0.530 |
| walker |  | 5699 | 262 | ts decl src/grid.config.js:1 |  |  | 0.550 |
| ns | 5884 |  | 330 | src/dockerUtil.js: complete method roster of the dockerode wrapper | 4.1 |  | 0.526 |
| walker |  | 5996 | 297 | ts decl src/dockerUtil.js:5 |  |  | 0.554 |
| walker |  | 6022 | 26 | ts body src/dockerUtil.js:165 |  |  | 0.555 |
| walker |  | 6048 | 26 | ts body src/dockerUtil.js:170 |  |  | 0.556 |
| walker |  | 6074 | 26 | ts body src/dockerUtil.js:175 |  |  | 0.557 |
| walker |  | 6088 | 14 | ts body widgets/containers/containerLogs.widget.js:10 |  |  | 0.557 |
| walker |  | 6102 | 14 | ts body widgets/services/servicesLogs.widget.js:10 |  |  | 0.557 |
| walker |  | 6117 | 15 | ts body src/widgetsTemplates/help.widget.template.js:134 |  |  | 0.557 |
| walker |  | 6132 | 15 | ts body src/widgetsTemplates/info.widget.template.js:111 |  |  | 0.557 |
| walker |  | 6147 | 15 | ts body src/widgetsTemplates/logs.widget.template.js:72 |  |  | 0.557 |
| ns | 6164 |  | 280 | src/dockerUtil.js: constructor — connection config and containerFilters parsing | 4.2 | 4.1 | 0.545 |
| ns | 6314 |  | 150 | src/dockerUtil.js: getInfo() — the host fields surfaced to the status widgets | 4.3 | 4.1 | 0.539 |
| walker |  | 6324 | 177 | ts decl src/widgetsTemplates/list.widget.template.js:8 |  |  | 0.540 |
| walker |  | 6339 | 15 | ts body src/widgetsTemplates/list.widget.template.js:127 |  |  | 0.540 |
| walker |  | 6354 | 15 | ts body src/widgetsTemplates/list.widget.template.js:147 |  |  | 0.540 |
| walker |  | 6369 | 15 | ts body src/widgetsTemplates/list.widget.template.js:151 |  |  | 0.540 |
| walker |  | 6385 | 16 | ts body src/widgetsTemplates/base.hook.template.js:29 |  |  | 0.540 |
| walker |  | 6401 | 16 | ts body widgets/images/imageList.widget.js:15 |  |  | 0.540 |
| walker |  | 6417 | 16 | ts body widgets/services/servicesList.widget.js:23 |  |  | 0.540 |
| ns | 6429 |  | 115 | src/dockerUtil.js: log streaming options for containers and services | 4.4 | 4.1 | 0.534 |
| walker |  | 6433 | 16 | ts body widgets/services/servicesList.widget.js:27 |  |  | 0.534 |
| walker |  | 6464 | 31 | ts body widgets/actionStatus.widget.js:18 |  |  | 0.534 |
| walker |  | 6652 | 188 | ts decl widgets/containers/containerList.widget.js:10 |  |  | 0.534 |
| ns | 6656 |  | 227 | src/baseWidget.js — the mixin every widget and hook extends | 5.1 |  | 0.523 |
| walker |  | 6661 | 9 | ts body widgets/containers/containerList.widget.js:18 |  |  | 0.523 |
| walker |  | 6677 | 16 | ts body widgets/containers/containerList.widget.js:40 |  |  | 0.523 |
| walker |  | 6709 | 32 | ts body src/screen.js:126 |  |  | 0.524 |
| walker |  | 6741 | 32 | ts body src/screen.js:132 |  |  | 0.525 |
| walker |  | 6758 | 17 | ts body src/widgetsTemplates/help.widget.template.js:50 |  |  | 0.525 |
| walker |  | 6775 | 17 | ts body src/widgetsTemplates/info.widget.template.js:64 |  |  | 0.525 |
| ns | 6888 |  | 232 | src/widgetsTemplates/base.hook.template.js — the hook base class | 5.2 |  | 0.516 |
| walker |  | 6981 | 206 | ts decl src/themes/styles.js:28 |  |  | 0.516 |
| ns | 7121 |  | 233 | src/widgetsTemplates/list.widget.template.js: complete method roster | 5.3 |  | 0.522 |
| walker |  | 7188 | 207 | ts decl src/themes/styles.js:1 |  |  | 0.522 |
| walker |  | 7206 | 18 | ts body widgets/containers/containerInfo.widget.js:10 |  |  | 0.522 |
| walker |  | 7224 | 18 | ts body widgets/images/imageInfo.widget.js:10 |  |  | 0.522 |
| walker |  | 7242 | 18 | ts body widgets/images/imageList.widget.js:123 |  |  | 0.522 |
| walker |  | 7260 | 18 | ts body widgets/services/servicesInfo.widget.js:10 |  |  | 0.522 |
| walker |  | 7355 | 95 | ts body index.js:72 |  |  | 0.523 |
| walker |  | 7375 | 20 | ts body widgets/containers/containerInfo.widget.js:14 |  |  | 0.523 |
| walker |  | 7395 | 20 | ts body widgets/containers/containerList.widget.js:75 |  |  | 0.523 |
| ns | 7409 |  | 288 | src/widgetsTemplates/info.widget.template.js and logs.widget.template.js: method rosters | 5.4 |  | 0.526 |
| walker |  | 7415 | 20 | ts body widgets/images/imageInfo.widget.js:14 |  |  | 0.526 |
| walker |  | 7435 | 20 | ts body widgets/services/servicesInfo.widget.js:14 |  |  | 0.526 |
| walker |  | 7462 | 27 | ts body src/dockerUtil.js:155 |  |  | 0.527 |
| walker |  | 7477 | 15 | ts body src/widgetsTemplates/logs.widget.template.js:84 |  |  | 0.527 |
| walker |  | 7482 | 5 | listing of '.github/ISSUE_TEMPLATE' |  |  | 0.527 |
| walker |  | 7498 | 16 | ts body src/widgetsTemplates/list.widget.template.js:131 |  |  | 0.527 |
| walker |  | 7515 | 17 | ts body src/widgetsTemplates/help.widget.template.js:54 |  |  | 0.527 |
| walker |  | 7532 | 17 | ts body src/widgetsTemplates/info.widget.template.js:68 |  |  | 0.527 |
| walker |  | 7549 | 17 | ts body widgets/services/servicesList.widget.js:19 |  |  | 0.527 |
| ns | 7570 |  | 161 | src/widgetsTemplates/help.widget.template.js: method roster around the help text | 5.5 |  | 0.526 |
| walker |  | 7598 | 49 | ts body hooks/shell.hook.js:59 |  |  | 0.526 |
| walker |  | 7651 | 53 | ts body hooks/containers.hook.js:168 |  |  | 0.526 |
| walker |  | 7704 | 53 | ts body hooks/images.hook.js:27 |  |  | 0.526 |
| walker |  | 7757 | 53 | ts body hooks/services.hook.js:86 |  |  | 0.526 |
| walker |  | 7793 | 36 | ts body src/screen.js:120 |  |  | 0.527 |
| ns | 7919 |  | 349 | hooks/: complete method rosters of all four hooks | 6.1 |  | 0.538 |
| walker |  | 7932 | 139 | ts body index.js:86 |  |  | 0.539 |
| walker |  | 7959 | 27 | ts body src/dockerUtil.js:160 |  |  | 0.540 |
| walker |  | 8119 | 160 | README.md section #2 |  |  | 0.549 |
| ns | 8128 |  | 209 | hooks/containers.hook.js: the toolbar key dispatch inside init() | 6.2 | 6.1 | 0.541 |
| walker |  | 8264 | 145 | README.md section #4 |  |  | 0.541 |
| walker |  | 8280 | 16 | ts body src/widgetsTemplates/list.widget.template.js:135 |  |  | 0.541 |
| ns | 8482 |  | 354 | hooks/shell.hook.js: openShell() plus dockerRunScript.sh's docker exec line | 6.3 | 6.1 | 0.531 |
| ns | 8583 |  | 101 | src/enum.js in full — the ContainerState vocabulary | 7.1 |  | 0.537 |
| walker |  | 8747 | 467 | package identity metadata in package.json |  |  | 0.537 |
| walker |  | 8816 | 69 | ts body src/themes/theme.selector.js:13 |  |  | 0.538 |
| ns | 8893 |  | 310 | src/themes/theme.selector.js in full and the dark/light style keys | 7.2 |  | 0.536 |
| walker |  | 9000 | 184 | ts body index.js:47 |  |  | 0.537 |
| walker |  | 9026 | 26 | ts body widgets/images/imageList.widget.js:6 |  |  | 0.537 |
| walker |  | 9103 | 77 | ts body widgets/actionStatus.widget.js:7 |  |  | 0.537 |
| walker |  | 9130 | 27 | ts body widgets/containers/containerList.widget.js:169 |  |  | 0.537 |
| walker |  | 9167 | 37 | ts body src/screen.js:114 |  |  | 0.538 |
| ns | 9179 |  | 286 | widgets/containers/containerList.widget.js: complete method roster | 7.3 |  | 0.538 |
| walker |  | 9187 | 20 | ts body src/widgetsTemplates/help.widget.template.js:138 |  |  | 0.538 |
| walker |  | 9207 | 20 | ts body src/widgetsTemplates/info.widget.template.js:115 |  |  | 0.542 |
| walker |  | 9217 | 10 | ts names docs/gatsby-config.js |  |  | 0.542 |
| walker |  | 9238 | 21 | ts body widgets/services/servicesList.widget.js:15 |  |  | 0.542 |
| walker |  | 9283 | 45 | ts body widgets/containers/containerSortList.widget.js:47 |  |  | 0.542 |
| walker |  | 9299 | 16 | ts body src/widgetsTemplates/list.widget.template.js:139 |  |  | 0.542 |
| walker |  | 9393 | 94 | ts body hooks/containers.hook.js:70 |  |  | 0.542 |
| ns | 9448 |  | 269 | widgets/toolbar.widget.js: the per-mode command extension map | 7.4 |  | 0.534 |
| walker |  | 9487 | 94 | ts body hooks/services.hook.js:61 |  |  | 0.534 |
| ns | 9596 |  | 148 | widgets/actionsMenu.widget.js: the `m` menu's action table | 7.5 |  | 0.531 |
| ns | 9640 |  | 44 | Complete listings of .github/ and its subdirectories | 8.1 |  | 0.535 |
| walker |  | 9673 | 186 | ts body src/baseWidget.js:4 |  |  | 0.551 |
| ns | 9739 |  | 99 | Complete listings of the docs/ Gatsby site | 8.2 |  | 0.560 |
| walker |  | 9773 | 100 | ts body widgets/searchInput.widget.js:94 |  |  | 0.560 |
| walker |  | 9873 | 100 | ts body widgets/toolbar.widget.js:8 |  |  | 0.560 |
| walker |  | 9925 | 52 | ts body widgets/services/servicesStatus.widget.js:22 |  |  | 0.560 |
| walker |  | 9952 | 27 | ts body widgets/images/imageList.widget.js:119 |  |  | 0.560 |
| ns | 9988 |  | 249 | Secondary ops config: codefresh.yml, devcontainer image, VS Code attach config, .nvmrc | 8.3 |  | 0.553 |

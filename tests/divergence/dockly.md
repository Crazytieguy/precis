Score(3000)=0.772 I=0.943 C=0.632 ns_rows≤3K=22/55 grid(1000/1442/2080/3000/4327/6240/9000)=0.710/0.585/0.698/0.772/0.695/0.651/0.684

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 72 | 72 | listing of '.' |  |  | 0.000 |
| walker |  | 81 | 9 | listing of 'lib' |  |  | 0.000 |
| ns | 95 |  | 95 | Package identity: name, version, description, entry point, bin name | 1.1 |  | 0.000 |
| walker |  | 105 | 24 | listing of 'hooks' |  |  | 0.000 |
| walker |  | 109 | 4 | listing of '.vscode' |  |  | 0.000 |
| walker |  | 114 | 5 | listing of '.devcontainer' |  |  | 0.000 |
| walker |  | 153 | 39 | listing of 'src' |  |  | 0.000 |
| walker |  | 162 | 9 | listing of 'src/themes' |  |  | 0.000 |
| ns | 167 |  | 72 | Complete root directory listing | 1.2 |  | 0.561 |
| walker |  | 193 | 31 | listing of 'src/widgetsTemplates' |  |  | 0.576 |
| walker |  | 230 | 37 | listing of 'widgets' |  |  | 0.626 |
| walker |  | 249 | 19 | listing of 'widgets/images' |  |  | 0.627 |
| ns | 276 |  | 109 | Complete listings of the four runtime code directories: src/, hooks/, lib/, widgets/ | 1.3 |  | 0.626 |
| walker |  | 315 | 66 | package identity in package.json |  |  | 0.817 |
| walker |  | 325 | 10 | export names surface in lib/node.version.js |  |  | 0.817 |
| walker |  | 356 | 31 | listing of 'widgets/services' |  |  | 0.824 |
| walker |  | 368 | 12 | export names surface in lib/modes.js |  |  | 0.824 |
| ns | 371 |  | 95 | Complete listings of the per-mode widget directories: widgets/containers, widgets/images, widgets/services | 1.4 |  | 0.738 |
| walker |  | 388 | 20 | export names surface in src/dockerUtil.js |  |  | 0.738 |
| walker |  | 388 | 0 | export at src/dockerUtil.js:5 |  |  | 0.738 |
| walker |  | 408 | 20 | export names surface in src/screen.js |  |  | 0.738 |
| walker |  | 408 | 0 | export at src/screen.js:18 |  |  | 0.738 |
| ns | 411 |  | 40 | Complete listings of src/widgetsTemplates and src/themes | 1.5 |  | 0.741 |
| walker |  | 453 | 45 | listing of 'widgets/containers' |  |  | 0.828 |
| walker |  | 463 | 10 | export member at src/dockerUtil.js:5 member 6 |  |  | 0.828 |
| walker |  | 473 | 10 | export member at src/screen.js:18 member 48 |  |  | 0.828 |
| walker |  | 483 | 10 | export member at src/screen.js:18 member 180 |  |  | 0.828 |
| walker |  | 493 | 10 | export member at src/screen.js:18 member 192 |  |  | 0.828 |
| walker |  | 502 | 9 | export body at src/screen.js:18 body 193 |  |  | 0.828 |
| ns | 533 |  | 122 | npm scripts block | 1.6 |  | 0.785 |
| walker |  | 536 | 34 | package runtime metadata in package.json |  |  | 0.786 |
| walker |  | 555 | 19 | listing of '.github' |  |  | 0.786 |
| walker |  | 575 | 20 | listing of '.github/workflows' |  |  | 0.786 |
| ns | 630 |  | 97 | Published `files` allow-list and the node engines floor | 1.7 |  | 0.733 |
| walker |  | 715 | 140 | README headline in README.md |  |  | 0.734 |
| walker |  | 738 | 23 | README prelude in README.md |  |  | 0.734 |
| walker |  | 755 | 17 | export at lib/node.version.js:1 |  |  | 0.734 |
| ns | 759 |  | 129 | README tagline plus every top-level heading location | 1.8 |  | 0.686 |
| walker |  | 767 | 12 | export member at src/dockerUtil.js:5 member 33 |  |  | 0.686 |
| walker |  | 780 | 13 | export member at src/dockerUtil.js:5 member 43 |  |  | 0.686 |
| walker |  | 793 | 13 | export member at src/dockerUtil.js:5 member 55 |  |  | 0.687 |
| walker |  | 804 | 11 | export member at src/screen.js:18 member 35 |  |  | 0.687 |
| ns | 891 |  | 132 | index.js: shebang and the module wiring of the executable | 1.9 |  | 0.647 |
| walker |  | 907 | 103 | headings outline in README.md |  |  | 0.707 |
| walker |  | 929 | 22 | README.md section #0 |  |  | 0.707 |
| walker |  | 942 | 13 | export member at src/dockerUtil.js:5 member 65 |  |  | 0.708 |
| walker |  | 975 | 33 | export at lib/modes.js:3 |  |  | 0.709 |
| walker |  | 981 | 6 | imports in lib/modes.js |  |  | 0.710 |
| walker |  | 992 | 11 | export member at src/screen.js:18 member 71 |  |  | 0.710 |
| ns | 1053 |  | 162 | index.js: pre-flight CLI dispatch (--help, --version, node version floor) | 1.10 | 1.9 | 0.657 |
| walker |  | 1101 | 109 | plaintext config dockerRunScript.sh |  |  | 0.657 |
| ns | 1219 |  | 166 | index.js: bootstrap promise chain and the three helper function signatures | 1.11 | 1.10 | 0.607 |
| walker |  | 1271 | 170 | plaintext config Dockerfile |  |  | 0.612 |
| walker |  | 1284 | 13 | export member at src/dockerUtil.js:5 member 78 |  |  | 0.612 |
| walker |  | 1338 | 54 | export names surface in src/cli.js |  |  | 0.613 |
| walker |  | 1338 | 0 | export at src/cli.js:59 |  |  | 0.613 |
| walker |  | 1338 | 0 | export at src/cli.js:66 |  |  | 0.613 |
| walker |  | 1338 | 0 | export at src/cli.js:93 |  |  | 0.613 |
| walker |  | 1351 | 13 | export body at src/cli.js:93 body 94 |  |  | 0.614 |
| walker |  | 1361 | 10 | module item at src/cli.js:8 |  |  | 0.614 |
| walker |  | 1413 | 52 | listing of 'docs' |  |  | 0.614 |
| ns | 1414 |  | 195 | Runtime dependency list from package.json | 1.12 |  | 0.585 |
| walker |  | 1422 | 9 | listing of 'docs/src' |  |  | 0.585 |
| walker |  | 1529 | 107 | package entrypoints in package.json |  |  | 0.745 |
| walker |  | 1540 | 11 | export member at src/screen.js:18 member 79 |  |  | 0.745 |
| walker |  | 1735 | 195 | package runtime dependencies in package.json |  |  | 0.800 |
| ns | 1737 |  | 323 | The in-app keybinding table (src/widgetsTemplates/help.widget.template.js:102-125) | 2.1 |  | 0.738 |
| walker |  | 1748 | 13 | listing of 'docs/src/pages' |  |  | 0.738 |
| walker |  | 1758 | 10 | export names surface in src/enum.js |  |  | 0.738 |
| ns | 1857 |  | 120 | src/cli.js: the complete flag roster (all eight option names) | 2.2 |  | 0.716 |
| walker |  | 1878 | 120 | package scripts in package.json |  |  | 0.745 |
| walker |  | 1891 | 13 | export member at src/dockerUtil.js:5 member 124 |  |  | 0.745 |
| walker |  | 1902 | 11 | export member at src/screen.js:18 member 86 |  |  | 0.745 |
| walker |  | 1927 | 25 | listing of 'docs/src/components' |  |  | 0.746 |
| walker |  | 1988 | 61 | export body at src/cli.js:59 body 60 |  |  | 0.746 |
| walker |  | 1999 | 11 | export member at src/screen.js:18 member 107 |  |  | 0.746 |
| walker |  | 2013 | 14 | export member at src/dockerUtil.js:5 member 88 |  |  | 0.746 |
| walker |  | 2024 | 11 | export member at src/screen.js:18 member 132 |  |  | 0.747 |
| walker |  | 2043 | 19 | export names surface in src/baseWidget.js |  |  | 0.698 |
| walker |  | 2043 | 0 | export at src/baseWidget.js:4 |  |  | 0.698 |
| ns | 2043 |  | 186 | src/cli.js: the four docker-connection options in full (socketPath, host, port, protocol) | 2.3 | 2.2 | 0.698 |
| walker |  | 2057 | 14 | export member at src/dockerUtil.js:5 member 100 |  |  | 0.698 |
| walker |  | 2079 | 22 | export names surface in src/assetsLoader.js |  |  | 0.698 |
| walker |  | 2090 | 11 | export member at src/screen.js:18 member 138 |  |  | 0.699 |
| walker |  | 2114 | 24 | imports in src/dockerUtil.js |  |  | 0.699 |
| walker |  | 2126 | 12 | export names surface in src/themes/styles.js |  |  | 0.699 |
| walker |  | 2140 | 14 | export member at src/dockerUtil.js:5 member 112 |  |  | 0.699 |
| walker |  | 2173 | 33 | export doc at src/cli.js:93 |  |  | 0.699 |
| ns | 2206 |  | 163 | src/cli.js: the four behavioural options in full (help, version, containerFilters, theme) | 2.4 | 2.2 | 0.658 |
| ns | 2269 |  | 63 | src/cli.js: every prototype method and the singleton export | 2.5 |  | 0.667 |
| walker |  | 2284 | 111 | imports in index.js |  |  | 0.691 |
| walker |  | 2296 | 12 | module item at index.js:47 |  |  | 0.692 |
| walker |  | 2309 | 13 | module item at index.js:72 |  |  | 0.693 |
| walker |  | 2322 | 13 | module item at index.js:86 |  |  | 0.696 |
| walker |  | 2334 | 12 | module item at index.js:17 |  |  | 0.696 |
| ns | 2339 |  | 70 | README: install and launch commands | 2.6 |  | 0.685 |
| walker |  | 2347 | 13 | module statements at index.js:5 |  |  | 0.691 |
| walker |  | 2382 | 35 | module statements at index.js:18 |  |  | 0.695 |
| walker |  | 2417 | 35 | module statements at index.js:23 |  |  | 0.702 |
| ns | 2455 |  | 116 | README: --containerFilters semantics | 2.7 |  | 0.696 |
| walker |  | 2487 | 70 | module statements at index.js:28 |  |  | 0.726 |
| walker |  | 2603 | 116 | module statements at index.js:34 |  |  | 0.770 |
| ns | 2625 |  | 170 | Dockerfile in full | 2.8 |  | 0.780 |
| walker |  | 2711 | 108 | literal roster at src/cli.js:8 |  |  | 0.808 |
| walker |  | 2722 | 11 | export member at src/screen.js:18 member 152 |  |  | 0.809 |
| ns | 2724 |  | 99 | README: running and building the docker image | 2.9 |  | 0.795 |
| walker |  | 2736 | 14 | export names surface in src/themes/theme.selector.js |  |  | 0.795 |
| walker |  | 2736 | 0 | export at src/themes/theme.selector.js:13 |  |  | 0.795 |
| walker |  | 2773 | 37 | export at src/assetsLoader.js:10 |  |  | 0.796 |
| walker |  | 2789 | 16 | export member at src/dockerUtil.js:5 member 155 |  |  | 0.796 |
| walker |  | 2801 | 12 | export member at src/screen.js:18 member 114 |  |  | 0.796 |
| walker |  | 2829 | 28 | export names surface in src/widgetsTemplates/list.widget.template.js |  |  | 0.796 |
| walker |  | 2829 | 0 | export at src/widgetsTemplates/list.widget.template.js:8 |  |  | 0.796 |
| walker |  | 2839 | 10 | export member at src/widgetsTemplates/list.widget.template.js:8 member 21 |  |  | 0.796 |
| walker |  | 2844 | 5 | listing of '.github/ISSUE_TEMPLATE' |  |  | 0.797 |
| walker |  | 2855 | 11 | export member at src/widgetsTemplates/list.widget.template.js:8 member 88 |  |  | 0.797 |
| walker |  | 2866 | 11 | export member at src/widgetsTemplates/list.widget.template.js:8 member 107 |  |  | 0.797 |
| ns | 2876 |  | 152 | .github/workflows/main.yml: the lint job | 2.10 |  | 0.771 |
| walker |  | 2877 | 11 | export member at src/widgetsTemplates/list.widget.template.js:8 member 127 |  |  | 0.771 |
| walker |  | 2889 | 12 | export member at src/screen.js:18 member 120 |  |  | 0.771 |
| walker |  | 2905 | 16 | export member at src/dockerUtil.js:5 member 160 |  |  | 0.772 |
| walker |  | 2924 | 19 | export at src/themes/styles.js:55 |  |  | 0.772 |
| walker |  | 2950 | 26 | export names surface in src/widgetsTemplates/help.widget.template.js |  |  | 0.772 |
| walker |  | 2976 | 26 | export names surface in src/widgetsTemplates/info.widget.template.js |  |  | 0.772 |
| walker |  | 3002 | 26 | export names surface in src/widgetsTemplates/logs.widget.template.js |  |  | 0.772 |
| walker |  | 3014 | 12 | export member at src/screen.js:18 member 126 |  |  | 0.772 |
| ns | 3087 |  | 211 | .github/workflows/main.yml: the semantic-release job | 2.11 |  | 0.750 |
| ns | 3170 |  | 83 | lib/modes.js and lib/node.version.js in full | 3.1 |  | 0.753 |
| walker |  | 3174 | 160 | README.md section #2 |  |  | 0.770 |
| walker |  | 3190 | 16 | export member at src/dockerUtil.js:5 member 165 |  |  | 0.770 |
| walker |  | 3218 | 28 | export names surface in src/widgetsTemplates/base.hook.template.js |  |  | 0.770 |
| walker |  | 3242 | 24 | export at src/widgetsTemplates/base.hook.template.js:6 |  |  | 0.770 |
| walker |  | 3257 | 15 | export body at src/widgetsTemplates/list.widget.template.js:8 body 128 |  |  | 0.771 |
| ns | 3375 |  | 205 | src/screen.js: imports and the mode -> grid-layout table | 3.2 |  | 0.751 |
| walker |  | 3402 | 145 | README.md section #4 |  |  | 0.751 |
| walker |  | 3462 | 60 | imports in src/cli.js |  |  | 0.751 |
| ns | 3573 |  | 198 | src/screen.js: complete method roster of the `screen` class | 3.3 |  | 0.757 |
| ns | 3770 |  | 197 | src/screen.js: init() — the full boot sequence | 3.4 | 3.3 | 0.734 |
| ns | 3900 |  | 130 | src/screen.js: constructor state fields | 3.5 | 3.3 | 0.719 |
| walker |  | 3929 | 467 | package identity metadata in package.json |  |  | 0.719 |
| walker |  | 3945 | 16 | export member at src/dockerUtil.js:5 member 170 |  |  | 0.720 |
| walker |  | 3958 | 13 | export member at src/screen.js:18 member 19 |  |  | 0.728 |
| walker |  | 3974 | 16 | export member at src/dockerUtil.js:5 member 175 |  |  | 0.728 |
| ns | 4024 |  | 124 | src/screen.js: initScreen() — blessed screen and the 12x12 grid | 3.6 | 3.3 | 0.717 |
| walker |  | 4069 | 95 | module item body at index.js:72 body 73 |  |  | 0.717 |
| walker |  | 4253 | 184 | export body at src/cli.js:66 body 67 |  |  | 0.717 |
| walker |  | 4269 | 16 | export member at src/dockerUtil.js:5 member 180 |  |  | 0.718 |
| walker |  | 4281 | 12 | export member at src/widgetsTemplates/list.widget.template.js:8 member 155 |  |  | 0.718 |
| ns | 4286 |  | 262 | src/screen.js: initHooks() and initWidgets() — instantiation and layout gating | 3.7 | 3.3 | 0.695 |
| walker |  | 4297 | 16 | export member at src/dockerUtil.js:5 member 187 |  |  | 0.695 |
| walker |  | 4406 | 109 | imports in src/screen.js |  |  | 0.702 |
| walker |  | 4497 | 91 | export at src/enum.js:1 |  |  | 0.703 |
| walker |  | 4523 | 26 | export body at src/dockerUtil.js:5 body 166 |  |  | 0.703 |
| ns | 4580 |  | 294 | src/screen.js: toggleMode() and registerEvents() — the global q/v keys and focus borders | 3.8 | 3.3 | 0.679 |
| walker |  | 4609 | 86 | export at src/widgetsTemplates/logs.widget.template.js:7 |  |  | 0.679 |
| walker |  | 4641 | 32 | export body at src/screen.js:18 body 127 |  |  | 0.679 |
| ns | 4668 |  | 88 | src/assetsLoader.js: the glob patterns that define the plugin conventions | 3.9 |  | 0.673 |
| ns | 4871 |  | 203 | src/assetsLoader.js: the loader body and the asset naming rule | 3.10 | 3.9 | 0.659 |
| walker |  | 5020 | 379 | README.md section #1 |  |  | 0.677 |
| walker |  | 5046 | 26 | export body at src/dockerUtil.js:5 body 171 |  |  | 0.677 |
| walker |  | 5066 | 20 | imports in src/baseWidget.js |  |  | 0.677 |
| ns | 5149 |  | 278 | src/grid.config.js: the containers-mode layout in full | 3.11 |  | 0.666 |
| walker |  | 5205 | 139 | module item body at index.js:86 body 87 |  |  | 0.669 |
| walker |  | 5218 | 13 | export member at src/widgetsTemplates/list.widget.template.js:8 member 147 |  |  | 0.669 |
| walker |  | 5233 | 15 | export body at src/widgetsTemplates/list.widget.template.js:8 body 148 |  |  | 0.669 |
| walker |  | 5265 | 32 | export body at src/screen.js:18 body 133 |  |  | 0.669 |
| ns | 5554 |  | 405 | src/grid.config.js: the services and images layouts plus the export block | 3.12 | 3.11 | 0.652 |
| walker |  | 5555 | 290 | README.md section #5 |  |  | 0.652 |
| walker |  | 5581 | 26 | export body at src/dockerUtil.js:5 body 176 |  |  | 0.652 |
| walker |  | 5688 | 107 | export at src/widgetsTemplates/info.widget.template.js:6 |  |  | 0.653 |
| walker |  | 5807 | 119 | export at src/widgetsTemplates/help.widget.template.js:5 |  |  | 0.654 |
| walker |  | 5834 | 27 | export body at src/dockerUtil.js:5 body 156 |  |  | 0.654 |
| walker |  | 5870 | 36 | export body at src/screen.js:18 body 121 |  |  | 0.654 |
| walker |  | 5883 | 13 | listing of 'docs/src/assets' |  |  | 0.654 |
| ns | 5884 |  | 330 | src/dockerUtil.js: complete method roster of the dockerode wrapper | 4.1 |  | 0.661 |
| walker |  | 5889 | 6 | listing of 'docs/src/assets/css' |  |  | 0.661 |
| walker |  | 5916 | 27 | export body at src/dockerUtil.js:5 body 161 |  |  | 0.661 |
| walker |  | 5985 | 69 | export body at src/themes/theme.selector.js:13 body 14 |  |  | 0.661 |
| walker |  | 6022 | 37 | export body at src/screen.js:18 body 115 |  |  | 0.661 |
| ns | 6164 |  | 280 | src/dockerUtil.js: constructor — connection config and containerFilters parsing | 4.2 | 4.1 | 0.646 |
| walker |  | 6206 | 184 | module item body at index.js:47 body 48 |  |  | 0.646 |
| walker |  | 6223 | 17 | export member at src/dockerUtil.js:5 member 194 |  |  | 0.651 |
| walker |  | 6289 | 66 | package identity in docs/package.json |  |  | 0.651 |
| walker |  | 6296 | 7 | plaintext config .nvmrc |  |  | 0.651 |
| ns | 6314 |  | 150 | src/dockerUtil.js: getInfo() — the host fields surfaced to the status widgets | 4.3 | 4.1 | 0.644 |
| walker |  | 6331 | 35 | imports in src/assetsLoader.js |  |  | 0.645 |
| walker |  | 6344 | 13 | export member at src/widgetsTemplates/list.widget.template.js:8 member 151 |  |  | 0.646 |
| walker |  | 6359 | 15 | export body at src/widgetsTemplates/list.widget.template.js:8 body 152 |  |  | 0.646 |
| walker |  | 6376 | 17 | export member at src/dockerUtil.js:5 member 208 |  |  | 0.651 |
| walker |  | 6413 | 37 | headings outline in SECURITY.md |  |  | 0.651 |
| walker |  | 6413 | 0 | SECURITY.md section #0 |  |  | 0.651 |
| ns | 6429 |  | 115 | src/dockerUtil.js: log streaming options for containers and services | 4.4 | 4.1 | 0.643 |
| walker |  | 6430 | 17 | export member at src/dockerUtil.js:5 member 220 |  |  | 0.648 |
| walker |  | 6616 | 186 | export body at src/baseWidget.js:4 body 5 |  |  | 0.650 |
| ns | 6656 |  | 227 | src/baseWidget.js — the mixin every widget and hook extends | 5.1 |  | 0.659 |
| ns | 6888 |  | 232 | src/widgetsTemplates/base.hook.template.js — the hook base class | 5.2 |  | 0.646 |
| ns | 7121 |  | 233 | src/widgetsTemplates/list.widget.template.js: complete method roster | 5.3 |  | 0.640 |
| walker |  | 7198 | 582 | README.md section #3 |  |  | 0.640 |
| ns | 7409 |  | 288 | src/widgetsTemplates/info.widget.template.js and logs.widget.template.js: method rosters | 5.4 |  | 0.644 |
| walker |  | 7433 | 235 | export body at src/assetsLoader.js:10 body 12 |  |  | 0.660 |
| walker |  | 7496 | 63 | README headline in docs/README.md |  |  | 0.660 |
| walker |  | 7514 | 18 | headings outline in docs/README.md |  |  | 0.660 |
| walker |  | 7539 | 25 | imports in src/widgetsTemplates/help.widget.template.js |  |  | 0.660 |
| ns | 7570 |  | 161 | src/widgetsTemplates/help.widget.template.js: method roster around the help text | 5.5 |  | 0.658 |
| walker |  | 7590 | 51 | export body at src/screen.js:18 body 108 |  |  | 0.658 |
| walker |  | 7617 | 27 | imports in src/widgetsTemplates/logs.widget.template.js |  |  | 0.658 |
| walker |  | 7645 | 28 | imports in src/themes/theme.selector.js |  |  | 0.658 |
| walker |  | 7661 | 16 | docs/README.md section #1 |  |  | 0.658 |
| walker |  | 7715 | 54 | headings outline in CONTRIBUTING.md |  |  | 0.658 |
| walker |  | 7751 | 36 | headings outline in .github/PULL_REQUEST_TEMPLATE.md |  |  | 0.658 |
| walker |  | 7751 | 0 | .github/PULL_REQUEST_TEMPLATE.md section #0 |  |  | 0.658 |
| walker |  | 7805 | 54 | export body at src/screen.js:18 body 80 |  |  | 0.658 |
| walker |  | 7819 | 14 | export member at src/widgetsTemplates/list.widget.template.js:8 member 119 |  |  | 0.660 |
| walker |  | 7850 | 31 | package dev/peer dependencies in package.json |  |  | 0.660 |
| walker |  | 7891 | 41 | docs/README.md section #0 |  |  | 0.660 |
| ns | 7919 |  | 349 | hooks/: complete method rosters of all four hooks | 6.1 |  | 0.643 |
| ns | 8128 |  | 209 | hooks/containers.hook.js: the toolbar key dispatch inside init() | 6.2 | 6.1 | 0.634 |
| walker |  | 8235 | 344 | module item body at src/cli.js:8 body 9 |  |  | 0.682 |
| walker |  | 8271 | 36 | imports in src/widgetsTemplates/base.hook.template.js |  |  | 0.684 |
| walker |  | 8307 | 36 | imports in src/widgetsTemplates/info.widget.template.js |  |  | 0.684 |
| walker |  | 8349 | 42 | export body at src/dockerUtil.js:5 body 181 |  |  | 0.684 |
| ns | 8482 |  | 354 | hooks/shell.hook.js: openShell() plus dockerRunScript.sh's docker exec line | 6.3 | 6.1 | 0.672 |
| walker |  | 8543 | 194 | export body at src/widgetsTemplates/base.hook.template.js:6 body 8 |  |  | 0.686 |
| walker |  | 8557 | 14 | export member at src/widgetsTemplates/list.widget.template.js:8 member 131 |  |  | 0.687 |
| walker |  | 8573 | 16 | export body at src/widgetsTemplates/list.widget.template.js:8 body 132 |  |  | 0.687 |
| ns | 8583 |  | 101 | src/enum.js in full — the ContainerState vocabulary | 7.1 |  | 0.690 |
| walker |  | 8647 | 74 | headings outline in CODE_OF_CONDUCT.md |  |  | 0.690 |
| walker |  | 8647 | 0 | CODE_OF_CONDUCT.md section #0 |  |  | 0.690 |
| walker |  | 8689 | 42 | export body at src/dockerUtil.js:5 body 188 |  |  | 0.690 |
| walker |  | 8762 | 73 | export body at src/screen.js:18 body 72 |  |  | 0.691 |
| walker |  | 8788 | 26 | listing of 'docs/src/assets/scss' |  |  | 0.691 |
| walker |  | 8799 | 11 | listing of 'docs/src/assets/scss/base' |  |  | 0.691 |
| walker |  | 8851 | 52 | imports in src/widgetsTemplates/list.widget.template.js |  |  | 0.693 |
| ns | 8893 |  | 310 | src/themes/theme.selector.js in full and the dark/light style keys | 7.2 |  | 0.683 |
| walker |  | 8935 | 84 | CONTRIBUTING.md section #0 |  |  | 0.683 |
| walker |  | 8949 | 14 | export member at src/widgetsTemplates/list.widget.template.js:8 member 135 |  |  | 0.684 |
| walker |  | 8965 | 16 | export body at src/widgetsTemplates/list.widget.template.js:8 body 136 |  |  | 0.684 |
| walker |  | 9028 | 63 | export body at src/dockerUtil.js:5 body 56 |  |  | 0.684 |
| walker |  | 9050 | 22 | listing of 'docs/src/assets/scss/libs' |  |  | 0.684 |
| walker |  | 9116 | 66 | export body at src/dockerUtil.js:5 body 34 |  |  | 0.684 |
| walker |  | 9141 | 25 | listing of 'docs/src/assets/scss/layout' |  |  | 0.684 |
| walker |  | 9155 | 14 | export member at src/widgetsTemplates/list.widget.template.js:8 member 139 |  |  | 0.686 |
| walker |  | 9171 | 16 | export body at src/widgetsTemplates/list.widget.template.js:8 body 140 |  |  | 0.686 |
| ns | 9179 |  | 286 | widgets/containers/containerList.widget.js: complete method roster | 7.3 |  | 0.677 |
| walker |  | 9291 | 120 | export body at src/screen.js:18 body 36 |  |  | 0.685 |
| walker |  | 9336 | 45 | listing of 'docs/src/assets/fonts' |  |  | 0.685 |
| walker |  | 9408 | 72 | export body at src/dockerUtil.js:5 body 79 |  |  | 0.685 |
| ns | 9448 |  | 269 | widgets/toolbar.widget.js: the per-mode command extension map | 7.4 |  | 0.676 |
| walker |  | 9534 | 126 | export body at src/screen.js:18 body 20 |  |  | 0.687 |
| walker |  | 9548 | 14 | export member at src/widgetsTemplates/list.widget.template.js:8 member 143 |  |  | 0.688 |
| walker |  | 9564 | 16 | export body at src/widgetsTemplates/list.widget.template.js:8 body 144 |  |  | 0.688 |
| ns | 9596 |  | 148 | widgets/actionsMenu.widget.js: the `m` menu's action table | 7.5 |  | 0.683 |
| ns | 9640 |  | 44 | Complete listings of .github/ and its subdirectories | 8.1 |  | 0.685 |
| walker |  | 9646 | 82 | export body at src/dockerUtil.js:5 body 44 |  |  | 0.685 |
| ns | 9739 |  | 99 | Complete listings of the docs/ Gatsby site | 8.2 |  | 0.691 |
| walker |  | 9779 | 133 | export body at src/screen.js:18 body 139 |  |  | 0.695 |
| ns | 9988 |  | 249 | Secondary ops config: codefresh.yml, devcontainer image, VS Code attach config, .nvmrc | 8.3 |  | 0.687 |

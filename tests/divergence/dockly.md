Score(3000)=0.772 I=0.943 C=0.632 ns_rows≤3K=22/55 (reached=15 partial=0 missing=7)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 80 | 80 | listing of '.' |  |  | 0.000 |
| walker |  | 88 | 8 | listing of 'lib' |  |  | 0.000 |
| walker |  | 91 | 3 | listing of '.vscode' |  |  | 0.000 |
| ns | 95 |  | 95 | Package identity: name, version, description, entry point, bin name | 1.1 |  | 0.000 |
| walker |  | 114 | 23 | listing of 'hooks' |  |  | 0.000 |
| walker |  | 118 | 4 | listing of '.devcontainer' |  |  | 0.000 |
| walker |  | 158 | 40 | listing of 'src' |  |  | 0.000 |
| walker |  | 166 | 8 | listing of 'src/themes' |  |  | 0.000 |
| ns | 175 |  | 80 | Complete root directory listing | 1.2 |  | 0.561 |
| walker |  | 196 | 30 | listing of 'src/widgetsTemplates' |  |  | 0.576 |
| walker |  | 235 | 39 | listing of 'widgets' |  |  | 0.626 |
| walker |  | 253 | 18 | listing of 'widgets/images' |  |  | 0.627 |
| ns | 285 |  | 110 | Complete listings of the four runtime code directories: src/, hooks/, lib/, widgets/ | 1.3 |  | 0.626 |
| walker |  | 319 | 66 | package identity in package.json |  |  | 0.817 |
| walker |  | 329 | 10 | export names surface in lib/node.version.js |  |  | 0.817 |
| walker |  | 359 | 30 | listing of 'widgets/services' |  |  | 0.824 |
| walker |  | 371 | 12 | export names surface in lib/modes.js |  |  | 0.824 |
| ns | 377 |  | 92 | Complete listings of the per-mode widget directories: widgets/containers, widgets/images, widgets/services | 1.4 |  | 0.738 |
| walker |  | 391 | 20 | export names surface in src/dockerUtil.js |  |  | 0.738 |
| walker |  | 391 | 0 | export at src/dockerUtil.js:5 |  |  | 0.738 |
| walker |  | 411 | 20 | export names surface in src/screen.js |  |  | 0.738 |
| walker |  | 411 | 0 | export at src/screen.js:18 |  |  | 0.738 |
| ns | 415 |  | 38 | Complete listings of src/widgetsTemplates and src/themes | 1.5 |  | 0.741 |
| walker |  | 455 | 44 | listing of 'widgets/containers' |  |  | 0.828 |
| walker |  | 465 | 10 | export member at src/dockerUtil.js:5 member 6 |  |  | 0.828 |
| walker |  | 475 | 10 | export member at src/screen.js:18 member 48 |  |  | 0.828 |
| walker |  | 485 | 10 | export member at src/screen.js:18 member 180 |  |  | 0.828 |
| walker |  | 495 | 10 | export member at src/screen.js:18 member 192 |  |  | 0.828 |
| walker |  | 504 | 9 | export body at src/screen.js:18 body 193 |  |  | 0.828 |
| ns | 537 |  | 122 | npm scripts block | 1.6 |  | 0.785 |
| walker |  | 538 | 34 | package runtime metadata in package.json |  |  | 0.786 |
| ns | 634 |  | 97 | Published `files` allow-list and the node engines floor | 1.7 |  | 0.732 |
| walker |  | 678 | 140 | README headline in README.md |  |  | 0.733 |
| walker |  | 701 | 23 | README prelude in README.md |  |  | 0.733 |
| walker |  | 718 | 17 | export at lib/node.version.js:1 |  |  | 0.734 |
| walker |  | 738 | 20 | listing of '.github' |  |  | 0.734 |
| walker |  | 757 | 19 | listing of '.github/workflows' |  |  | 0.734 |
| ns | 763 |  | 129 | README tagline plus every top-level heading location | 1.8 |  | 0.686 |
| walker |  | 769 | 12 | export member at src/dockerUtil.js:5 member 33 |  |  | 0.686 |
| walker |  | 782 | 13 | export member at src/dockerUtil.js:5 member 43 |  |  | 0.686 |
| walker |  | 795 | 13 | export member at src/dockerUtil.js:5 member 55 |  |  | 0.687 |
| walker |  | 806 | 11 | export member at src/screen.js:18 member 35 |  |  | 0.687 |
| ns | 895 |  | 132 | index.js: shebang and the module wiring of the executable | 1.9 |  | 0.647 |
| walker |  | 909 | 103 | headings outline in README.md |  |  | 0.707 |
| walker |  | 931 | 22 | README.md section #0 |  |  | 0.707 |
| walker |  | 944 | 13 | export member at src/dockerUtil.js:5 member 65 |  |  | 0.708 |
| walker |  | 977 | 33 | export at lib/modes.js:3 |  |  | 0.709 |
| walker |  | 983 | 6 | imports in lib/modes.js |  |  | 0.710 |
| walker |  | 994 | 11 | export member at src/screen.js:18 member 71 |  |  | 0.710 |
| ns | 1057 |  | 162 | index.js: pre-flight CLI dispatch (--help, --version, node version floor) | 1.10 | 1.9 | 0.657 |
| walker |  | 1103 | 109 | plaintext config dockerRunScript.sh |  |  | 0.657 |
| ns | 1223 |  | 166 | index.js: bootstrap promise chain and the three helper function signatures | 1.11 | 1.10 | 0.607 |
| walker |  | 1273 | 170 | plaintext config Dockerfile |  |  | 0.612 |
| walker |  | 1286 | 13 | export member at src/dockerUtil.js:5 member 78 |  |  | 0.612 |
| walker |  | 1340 | 54 | export names surface in src/cli.js |  |  | 0.613 |
| walker |  | 1340 | 0 | export at src/cli.js:59 |  |  | 0.613 |
| walker |  | 1340 | 0 | export at src/cli.js:66 |  |  | 0.613 |
| walker |  | 1340 | 0 | export at src/cli.js:93 |  |  | 0.613 |
| walker |  | 1353 | 13 | export body at src/cli.js:93 body 94 |  |  | 0.614 |
| walker |  | 1363 | 10 | module item at src/cli.js:8 |  |  | 0.614 |
| walker |  | 1415 | 52 | listing of 'docs' |  |  | 0.614 |
| ns | 1418 |  | 195 | Runtime dependency list from package.json | 1.12 |  | 0.585 |
| walker |  | 1426 | 11 | listing of 'docs/src' |  |  | 0.585 |
| walker |  | 1438 | 12 | listing of 'docs/src/pages' |  |  | 0.586 |
| walker |  | 1545 | 107 | package entrypoints in package.json |  |  | 0.745 |
| walker |  | 1556 | 11 | export member at src/screen.js:18 member 79 |  |  | 0.745 |
| ns | 1741 |  | 323 | The in-app keybinding table (src/widgetsTemplates/help.widget.template.js:102-125) | 2.1 |  | 0.688 |
| walker |  | 1751 | 195 | package runtime dependencies in package.json |  |  | 0.738 |
| walker |  | 1761 | 10 | export names surface in src/enum.js |  |  | 0.738 |
| ns | 1861 |  | 120 | src/cli.js: the complete flag roster (all eight option names) | 2.2 |  | 0.716 |
| walker |  | 1881 | 120 | package scripts in package.json |  |  | 0.745 |
| walker |  | 1894 | 13 | export member at src/dockerUtil.js:5 member 124 |  |  | 0.745 |
| walker |  | 1905 | 11 | export member at src/screen.js:18 member 86 |  |  | 0.745 |
| walker |  | 1929 | 24 | listing of 'docs/src/components' |  |  | 0.746 |
| walker |  | 1990 | 61 | export body at src/cli.js:59 body 60 |  |  | 0.746 |
| walker |  | 2001 | 11 | export member at src/screen.js:18 member 107 |  |  | 0.746 |
| walker |  | 2015 | 14 | export member at src/dockerUtil.js:5 member 88 |  |  | 0.746 |
| walker |  | 2026 | 11 | export member at src/screen.js:18 member 132 |  |  | 0.747 |
| walker |  | 2045 | 19 | export names surface in src/baseWidget.js |  |  | 0.747 |
| walker |  | 2045 | 0 | export at src/baseWidget.js:4 |  |  | 0.747 |
| ns | 2047 |  | 186 | src/cli.js: the four docker-connection options in full (socketPath, host, port, protocol) | 2.3 | 2.2 | 0.698 |
| walker |  | 2059 | 14 | export member at src/dockerUtil.js:5 member 100 |  |  | 0.698 |
| walker |  | 2081 | 22 | export names surface in src/assetsLoader.js |  |  | 0.698 |
| walker |  | 2092 | 11 | export member at src/screen.js:18 member 138 |  |  | 0.699 |
| walker |  | 2116 | 24 | imports in src/dockerUtil.js |  |  | 0.699 |
| walker |  | 2128 | 12 | export names surface in src/themes/styles.js |  |  | 0.699 |
| walker |  | 2142 | 14 | export member at src/dockerUtil.js:5 member 112 |  |  | 0.699 |
| walker |  | 2175 | 33 | export doc at src/cli.js:93 |  |  | 0.699 |
| ns | 2210 |  | 163 | src/cli.js: the four behavioural options in full (help, version, containerFilters, theme) | 2.4 | 2.2 | 0.658 |
| ns | 2273 |  | 63 | src/cli.js: every prototype method and the singleton export | 2.5 |  | 0.667 |
| walker |  | 2286 | 111 | imports in index.js |  |  | 0.691 |
| walker |  | 2298 | 12 | module item at index.js:47 |  |  | 0.692 |
| walker |  | 2311 | 13 | module item at index.js:72 |  |  | 0.693 |
| walker |  | 2324 | 13 | module item at index.js:86 |  |  | 0.696 |
| walker |  | 2336 | 12 | module item at index.js:17 |  |  | 0.696 |
| ns | 2343 |  | 70 | README: install and launch commands | 2.6 |  | 0.685 |
| walker |  | 2349 | 13 | module statements at index.js:5 |  |  | 0.691 |
| walker |  | 2384 | 35 | module statements at index.js:18 |  |  | 0.695 |
| walker |  | 2419 | 35 | module statements at index.js:23 |  |  | 0.702 |
| ns | 2459 |  | 116 | README: --containerFilters semantics | 2.7 |  | 0.696 |
| walker |  | 2489 | 70 | module statements at index.js:28 |  |  | 0.726 |
| walker |  | 2605 | 116 | module statements at index.js:34 |  |  | 0.770 |
| ns | 2629 |  | 170 | Dockerfile in full | 2.8 |  | 0.780 |
| walker |  | 2713 | 108 | literal roster at src/cli.js:8 |  |  | 0.808 |
| walker |  | 2724 | 11 | export member at src/screen.js:18 member 152 |  |  | 0.809 |
| ns | 2728 |  | 99 | README: running and building the docker image | 2.9 |  | 0.795 |
| walker |  | 2738 | 14 | export names surface in src/themes/theme.selector.js |  |  | 0.795 |
| walker |  | 2738 | 0 | export at src/themes/theme.selector.js:13 |  |  | 0.795 |
| walker |  | 2775 | 37 | export at src/assetsLoader.js:10 |  |  | 0.796 |
| walker |  | 2779 | 4 | listing of '.github/ISSUE_TEMPLATE' |  |  | 0.796 |
| walker |  | 2795 | 16 | export member at src/dockerUtil.js:5 member 155 |  |  | 0.796 |
| walker |  | 2807 | 12 | export member at src/screen.js:18 member 114 |  |  | 0.797 |
| walker |  | 2835 | 28 | export names surface in src/widgetsTemplates/list.widget.template.js |  |  | 0.797 |
| walker |  | 2835 | 0 | export at src/widgetsTemplates/list.widget.template.js:8 |  |  | 0.797 |
| walker |  | 2845 | 10 | export member at src/widgetsTemplates/list.widget.template.js:8 member 21 |  |  | 0.797 |
| walker |  | 2856 | 11 | export member at src/widgetsTemplates/list.widget.template.js:8 member 88 |  |  | 0.797 |
| walker |  | 2867 | 11 | export member at src/widgetsTemplates/list.widget.template.js:8 member 107 |  |  | 0.797 |
| walker |  | 2878 | 11 | export member at src/widgetsTemplates/list.widget.template.js:8 member 127 |  |  | 0.797 |
| ns | 2880 |  | 152 | .github/workflows/main.yml: the lint job | 2.10 |  | 0.771 |
| walker |  | 2890 | 12 | export member at src/screen.js:18 member 120 |  |  | 0.771 |
| walker |  | 2906 | 16 | export member at src/dockerUtil.js:5 member 160 |  |  | 0.772 |
| walker |  | 2925 | 19 | export at src/themes/styles.js:55 |  |  | 0.772 |
| walker |  | 2951 | 26 | export names surface in src/widgetsTemplates/help.widget.template.js |  |  | 0.772 |
| walker |  | 2977 | 26 | export names surface in src/widgetsTemplates/info.widget.template.js |  |  | 0.772 |
| walker |  | 3003 | 26 | export names surface in src/widgetsTemplates/logs.widget.template.js |  |  | 0.772 |
| walker |  | 3015 | 12 | export member at src/screen.js:18 member 126 |  |  | 0.772 |
| ns | 3091 |  | 211 | .github/workflows/main.yml: the semantic-release job | 2.11 |  | 0.750 |
| ns | 3174 |  | 83 | lib/modes.js and lib/node.version.js in full | 3.1 |  | 0.753 |
| walker |  | 3175 | 160 | README.md section #2 |  |  | 0.770 |
| walker |  | 3191 | 16 | export member at src/dockerUtil.js:5 member 165 |  |  | 0.770 |
| walker |  | 3219 | 28 | export names surface in src/widgetsTemplates/base.hook.template.js |  |  | 0.770 |
| walker |  | 3243 | 24 | export at src/widgetsTemplates/base.hook.template.js:6 |  |  | 0.770 |
| walker |  | 3258 | 15 | export body at src/widgetsTemplates/list.widget.template.js:8 body 128 |  |  | 0.771 |
| ns | 3379 |  | 205 | src/screen.js: imports and the mode -> grid-layout table | 3.2 |  | 0.751 |
| walker |  | 3403 | 145 | README.md section #4 |  |  | 0.751 |
| walker |  | 3463 | 60 | imports in src/cli.js |  |  | 0.751 |
| ns | 3577 |  | 198 | src/screen.js: complete method roster of the `screen` class | 3.3 |  | 0.757 |
| ns | 3774 |  | 197 | src/screen.js: init() — the full boot sequence | 3.4 | 3.3 | 0.734 |
| ns | 3904 |  | 130 | src/screen.js: constructor state fields | 3.5 | 3.3 | 0.719 |
| walker |  | 3930 | 467 | package identity metadata in package.json |  |  | 0.719 |
| walker |  | 3946 | 16 | export member at src/dockerUtil.js:5 member 170 |  |  | 0.720 |
| walker |  | 3959 | 13 | export member at src/screen.js:18 member 19 |  |  | 0.728 |
| walker |  | 3975 | 16 | export member at src/dockerUtil.js:5 member 175 |  |  | 0.728 |
| ns | 4028 |  | 124 | src/screen.js: initScreen() — blessed screen and the 12x12 grid | 3.6 | 3.3 | 0.717 |
| walker |  | 4070 | 95 | module item body at index.js:72 body 73 |  |  | 0.717 |
| walker |  | 4254 | 184 | export body at src/cli.js:66 body 67 |  |  | 0.717 |
| walker |  | 4270 | 16 | export member at src/dockerUtil.js:5 member 180 |  |  | 0.718 |
| walker |  | 4282 | 12 | export member at src/widgetsTemplates/list.widget.template.js:8 member 155 |  |  | 0.718 |
| ns | 4290 |  | 262 | src/screen.js: initHooks() and initWidgets() — instantiation and layout gating | 3.7 | 3.3 | 0.695 |
| walker |  | 4298 | 16 | export member at src/dockerUtil.js:5 member 187 |  |  | 0.695 |
| walker |  | 4407 | 109 | imports in src/screen.js |  |  | 0.702 |
| walker |  | 4498 | 91 | export at src/enum.js:1 |  |  | 0.703 |
| walker |  | 4524 | 26 | export body at src/dockerUtil.js:5 body 166 |  |  | 0.703 |
| ns | 4584 |  | 294 | src/screen.js: toggleMode() and registerEvents() — the global q/v keys and focus borders | 3.8 | 3.3 | 0.679 |
| walker |  | 4610 | 86 | export at src/widgetsTemplates/logs.widget.template.js:7 |  |  | 0.679 |
| walker |  | 4642 | 32 | export body at src/screen.js:18 body 127 |  |  | 0.679 |
| ns | 4672 |  | 88 | src/assetsLoader.js: the glob patterns that define the plugin conventions | 3.9 |  | 0.673 |
| ns | 4875 |  | 203 | src/assetsLoader.js: the loader body and the asset naming rule | 3.10 | 3.9 | 0.659 |
| walker |  | 5021 | 379 | README.md section #1 |  |  | 0.677 |
| walker |  | 5047 | 26 | export body at src/dockerUtil.js:5 body 171 |  |  | 0.677 |
| walker |  | 5067 | 20 | imports in src/baseWidget.js |  |  | 0.677 |
| ns | 5153 |  | 278 | src/grid.config.js: the containers-mode layout in full | 3.11 |  | 0.666 |
| walker |  | 5206 | 139 | module item body at index.js:86 body 87 |  |  | 0.669 |
| walker |  | 5219 | 13 | export member at src/widgetsTemplates/list.widget.template.js:8 member 147 |  |  | 0.669 |
| walker |  | 5234 | 15 | export body at src/widgetsTemplates/list.widget.template.js:8 body 148 |  |  | 0.669 |
| walker |  | 5266 | 32 | export body at src/screen.js:18 body 133 |  |  | 0.669 |
| walker |  | 5556 | 290 | README.md section #5 |  |  | 0.669 |
| ns | 5558 |  | 405 | src/grid.config.js: the services and images layouts plus the export block | 3.12 | 3.11 | 0.652 |
| walker |  | 5582 | 26 | export body at src/dockerUtil.js:5 body 176 |  |  | 0.652 |
| walker |  | 5689 | 107 | export at src/widgetsTemplates/info.widget.template.js:6 |  |  | 0.653 |
| walker |  | 5808 | 119 | export at src/widgetsTemplates/help.widget.template.js:5 |  |  | 0.654 |
| walker |  | 5835 | 27 | export body at src/dockerUtil.js:5 body 156 |  |  | 0.654 |
| walker |  | 5871 | 36 | export body at src/screen.js:18 body 121 |  |  | 0.654 |
| ns | 5888 |  | 330 | src/dockerUtil.js: complete method roster of the dockerode wrapper | 4.1 |  | 0.661 |
| walker |  | 5898 | 27 | export body at src/dockerUtil.js:5 body 161 |  |  | 0.661 |
| walker |  | 5967 | 69 | export body at src/themes/theme.selector.js:13 body 14 |  |  | 0.661 |
| walker |  | 6004 | 37 | export body at src/screen.js:18 body 115 |  |  | 0.661 |
| ns | 6168 |  | 280 | src/dockerUtil.js: constructor — connection config and containerFilters parsing | 4.2 | 4.1 | 0.646 |
| walker |  | 6188 | 184 | module item body at index.js:47 body 48 |  |  | 0.646 |
| walker |  | 6205 | 17 | export member at src/dockerUtil.js:5 member 194 |  |  | 0.651 |
| walker |  | 6271 | 66 | package identity in docs/package.json |  |  | 0.651 |
| walker |  | 6278 | 7 | plaintext config .nvmrc |  |  | 0.651 |
| walker |  | 6313 | 35 | imports in src/assetsLoader.js |  |  | 0.653 |
| ns | 6318 |  | 150 | src/dockerUtil.js: getInfo() — the host fields surfaced to the status widgets | 4.3 | 4.1 | 0.645 |
| walker |  | 6326 | 13 | export member at src/widgetsTemplates/list.widget.template.js:8 member 151 |  |  | 0.646 |
| walker |  | 6341 | 15 | export body at src/widgetsTemplates/list.widget.template.js:8 body 152 |  |  | 0.646 |
| walker |  | 6357 | 16 | listing of 'docs/src/assets' |  |  | 0.646 |
| walker |  | 6362 | 5 | listing of 'docs/src/assets/css' |  |  | 0.646 |
| walker |  | 6379 | 17 | export member at src/dockerUtil.js:5 member 208 |  |  | 0.651 |
| walker |  | 6416 | 37 | headings outline in SECURITY.md |  |  | 0.651 |
| walker |  | 6416 | 0 | SECURITY.md section #0 |  |  | 0.651 |
| walker |  | 6433 | 17 | export member at src/dockerUtil.js:5 member 220 |  |  | 0.648 |
| ns | 6433 |  | 115 | src/dockerUtil.js: log streaming options for containers and services | 4.4 | 4.1 | 0.648 |
| walker |  | 6619 | 186 | export body at src/baseWidget.js:4 body 5 |  |  | 0.650 |
| ns | 6660 |  | 227 | src/baseWidget.js — the mixin every widget and hook extends | 5.1 |  | 0.659 |
| ns | 6892 |  | 232 | src/widgetsTemplates/base.hook.template.js — the hook base class | 5.2 |  | 0.646 |
| ns | 7125 |  | 233 | src/widgetsTemplates/list.widget.template.js: complete method roster | 5.3 |  | 0.640 |
| walker |  | 7201 | 582 | README.md section #3 |  |  | 0.640 |
| ns | 7413 |  | 288 | src/widgetsTemplates/info.widget.template.js and logs.widget.template.js: method rosters | 5.4 |  | 0.644 |
| walker |  | 7436 | 235 | export body at src/assetsLoader.js:10 body 12 |  |  | 0.660 |
| walker |  | 7499 | 63 | README headline in docs/README.md |  |  | 0.660 |
| walker |  | 7517 | 18 | headings outline in docs/README.md |  |  | 0.660 |
| walker |  | 7542 | 25 | imports in src/widgetsTemplates/help.widget.template.js |  |  | 0.660 |
| ns | 7574 |  | 161 | src/widgetsTemplates/help.widget.template.js: method roster around the help text | 5.5 |  | 0.658 |
| walker |  | 7593 | 51 | export body at src/screen.js:18 body 108 |  |  | 0.658 |
| walker |  | 7620 | 27 | imports in src/widgetsTemplates/logs.widget.template.js |  |  | 0.658 |
| walker |  | 7648 | 28 | imports in src/themes/theme.selector.js |  |  | 0.658 |
| walker |  | 7664 | 16 | docs/README.md section #1 |  |  | 0.658 |
| walker |  | 7718 | 54 | headings outline in CONTRIBUTING.md |  |  | 0.658 |
| walker |  | 7754 | 36 | headings outline in .github/PULL_REQUEST_TEMPLATE.md |  |  | 0.658 |
| walker |  | 7754 | 0 | .github/PULL_REQUEST_TEMPLATE.md section #0 |  |  | 0.658 |
| walker |  | 7808 | 54 | export body at src/screen.js:18 body 80 |  |  | 0.658 |
| walker |  | 7822 | 14 | export member at src/widgetsTemplates/list.widget.template.js:8 member 119 |  |  | 0.660 |
| walker |  | 7853 | 31 | package dev/peer dependencies in package.json |  |  | 0.660 |
| walker |  | 7894 | 41 | docs/README.md section #0 |  |  | 0.660 |
| ns | 7923 |  | 349 | hooks/: complete method rosters of all four hooks | 6.1 |  | 0.643 |
| ns | 8132 |  | 209 | hooks/containers.hook.js: the toolbar key dispatch inside init() | 6.2 | 6.1 | 0.634 |
| walker |  | 8238 | 344 | module item body at src/cli.js:8 body 9 |  |  | 0.682 |
| walker |  | 8274 | 36 | imports in src/widgetsTemplates/base.hook.template.js |  |  | 0.684 |
| walker |  | 8310 | 36 | imports in src/widgetsTemplates/info.widget.template.js |  |  | 0.684 |
| walker |  | 8352 | 42 | export body at src/dockerUtil.js:5 body 181 |  |  | 0.684 |
| ns | 8486 |  | 354 | hooks/shell.hook.js: openShell() plus dockerRunScript.sh's docker exec line | 6.3 | 6.1 | 0.672 |
| walker |  | 8546 | 194 | export body at src/widgetsTemplates/base.hook.template.js:6 body 8 |  |  | 0.686 |
| walker |  | 8560 | 14 | export member at src/widgetsTemplates/list.widget.template.js:8 member 131 |  |  | 0.687 |
| walker |  | 8576 | 16 | export body at src/widgetsTemplates/list.widget.template.js:8 body 132 |  |  | 0.687 |
| ns | 8587 |  | 101 | src/enum.js in full — the ContainerState vocabulary | 7.1 |  | 0.690 |
| walker |  | 8650 | 74 | headings outline in CODE_OF_CONDUCT.md |  |  | 0.690 |
| walker |  | 8650 | 0 | CODE_OF_CONDUCT.md section #0 |  |  | 0.690 |
| walker |  | 8692 | 42 | export body at src/dockerUtil.js:5 body 188 |  |  | 0.690 |
| walker |  | 8765 | 73 | export body at src/screen.js:18 body 72 |  |  | 0.691 |
| walker |  | 8817 | 52 | imports in src/widgetsTemplates/list.widget.template.js |  |  | 0.693 |
| ns | 8897 |  | 310 | src/themes/theme.selector.js in full and the dark/light style keys | 7.2 |  | 0.683 |
| walker |  | 8901 | 84 | CONTRIBUTING.md section #0 |  |  | 0.683 |
| walker |  | 8915 | 14 | export member at src/widgetsTemplates/list.widget.template.js:8 member 135 |  |  | 0.684 |
| walker |  | 8931 | 16 | export body at src/widgetsTemplates/list.widget.template.js:8 body 136 |  |  | 0.684 |
| walker |  | 8960 | 29 | listing of 'docs/src/assets/scss' |  |  | 0.684 |
| walker |  | 8970 | 10 | listing of 'docs/src/assets/scss/base' |  |  | 0.684 |
| walker |  | 8991 | 21 | listing of 'docs/src/assets/scss/libs' |  |  | 0.684 |
| walker |  | 9054 | 63 | export body at src/dockerUtil.js:5 body 56 |  |  | 0.684 |
| walker |  | 9078 | 24 | listing of 'docs/src/assets/scss/layout' |  |  | 0.684 |
| walker |  | 9144 | 66 | export body at src/dockerUtil.js:5 body 34 |  |  | 0.684 |
| walker |  | 9158 | 14 | export member at src/widgetsTemplates/list.widget.template.js:8 member 139 |  |  | 0.686 |
| walker |  | 9174 | 16 | export body at src/widgetsTemplates/list.widget.template.js:8 body 140 |  |  | 0.686 |
| ns | 9183 |  | 286 | widgets/containers/containerList.widget.js: complete method roster | 7.3 |  | 0.677 |
| walker |  | 9294 | 120 | export body at src/screen.js:18 body 36 |  |  | 0.685 |
| walker |  | 9338 | 44 | listing of 'docs/src/assets/fonts' |  |  | 0.685 |
| walker |  | 9410 | 72 | export body at src/dockerUtil.js:5 body 79 |  |  | 0.685 |
| ns | 9452 |  | 269 | widgets/toolbar.widget.js: the per-mode command extension map | 7.4 |  | 0.676 |
| walker |  | 9536 | 126 | export body at src/screen.js:18 body 20 |  |  | 0.687 |
| walker |  | 9550 | 14 | export member at src/widgetsTemplates/list.widget.template.js:8 member 143 |  |  | 0.688 |
| walker |  | 9566 | 16 | export body at src/widgetsTemplates/list.widget.template.js:8 body 144 |  |  | 0.688 |
| ns | 9600 |  | 148 | widgets/actionsMenu.widget.js: the `m` menu's action table | 7.5 |  | 0.683 |
| ns | 9645 |  | 45 | Complete listings of .github/ and its subdirectories | 8.1 |  | 0.685 |
| walker |  | 9648 | 82 | export body at src/dockerUtil.js:5 body 44 |  |  | 0.685 |
| ns | 9747 |  | 102 | Complete listings of the docs/ Gatsby site | 8.2 |  | 0.691 |
| walker |  | 9781 | 133 | export body at src/screen.js:18 body 139 |  |  | 0.695 |
| walker |  | 9842 | 61 | listing of 'docs/src/assets/images' |  |  | 0.695 |
| ns | 9996 |  | 249 | Secondary ops config: codefresh.yml, devcontainer image, VS Code attach config, .nvmrc | 8.3 |  | 0.687 |

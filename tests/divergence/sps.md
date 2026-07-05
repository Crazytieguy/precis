Score(3000)=0.662 I=0.793 C=0.552 ns_rows≤3K=19/49 (reached=7 partial=5 missing=7)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 47 | 47 | listing of '.' |  |  | 1.000 |
| ns | 47 |  | 47 | Top-level repo listing | 1.1 |  | 1.000 |
| walker |  | 86 | 39 | README headline in README.md |  |  | 1.000 |
| ns | 148 |  | 101 | README lede — what sps is | 1.2 |  | 0.772 |
| walker |  | 163 | 77 | [package] in Cargo.toml |  |  | 0.810 |
| walker |  | 171 | 8 | listing of 'sps' |  |  | 0.810 |
| walker |  | 189 | 18 | listing of 'sps/src' |  |  | 0.811 |
| walker |  | 201 | 12 | listing of 'sps/src/pipeline' |  |  | 0.811 |
| walker |  | 218 | 17 | entry item at sps/src/main.rs:57 |  |  | 0.811 |
| ns | 225 |  | 77 | Workspace members | 1.3 |  | 0.830 |
| walker |  | 227 | 9 | entry item body at sps/src/main.rs:57 body 190 |  |  | 0.830 |
| walker |  | 241 | 14 | entry item body at sps/src/main.rs:57 body 58 |  |  | 0.830 |
| walker |  | 256 | 15 | entry item body at sps/src/main.rs:57 body 90 |  |  | 0.830 |
| walker |  | 275 | 19 | entry item body at sps/src/main.rs:57 body 158 |  |  | 0.830 |
| walker |  | 294 | 19 | entry item body at sps/src/main.rs:57 body 189 |  |  | 0.830 |
| walker |  | 320 | 26 | entry item body at sps/src/main.rs:57 body 83 |  |  | 0.830 |
| walker |  | 328 | 8 | listing of 'sps-common' |  |  | 0.830 |
| walker |  | 336 | 8 | listing of 'sps-core' |  |  | 0.830 |
| ns | 337 |  | 112 | CLI subcommand enum | 1.4 |  | 0.666 |
| walker |  | 344 | 8 | listing of 'sps-net' |  |  | 0.666 |
| walker |  | 395 | 51 | entry item body at sps/src/main.rs:57 body 85 |  |  | 0.666 |
| walker |  | 453 | 58 | entry item body at sps/src/main.rs:57 body 78 |  |  | 0.666 |
| walker |  | 511 | 58 | entry item body at sps/src/main.rs:57 body 141 |  |  | 0.667 |
| ns | 513 |  | 176 | CLI per-subcommand module list | 1.5 |  | 0.526 |
| walker |  | 551 | 40 | listing of 'sps/src/cli' |  |  | 0.528 |
| walker |  | 617 | 66 | entry item body at sps/src/main.rs:57 body 72 |  |  | 0.533 |
| walker |  | 689 | 72 | entry item body at sps/src/main.rs:57 body 135 |  |  | 0.535 |
| ns | 692 |  | 179 | main.rs — Tokio entry signature + Init early-out | 1.6 |  | 0.466 |
| ns | 758 |  | 66 | main.rs — Config::load failure hint | 1.7 | 1.6 | 0.498 |
| walker |  | 769 | 80 | entry item body at sps/src/main.rs:57 body 159 |  |  | 0.504 |
| walker |  | 792 | 23 | pub-item names surface in sps/src/cli.rs |  |  | 0.505 |
| walker |  | 813 | 21 | listing of 'sps-net/src' |  |  | 0.506 |
| walker |  | 939 | 126 | entry item body at sps/src/main.rs:57 body 60 |  |  | 0.626 |
| ns | 952 |  | 194 | main.rs — auto-update gate | 1.8 | 1.7 | 0.570 |
| walker |  | 1068 | 129 | entry item body at sps/src/main.rs:57 body 146 |  |  | 0.673 |
| walker |  | 1093 | 25 | listing of 'sps-core/src' |  |  | 0.677 |
| walker |  | 1105 | 12 | listing of 'sps-core/src/check' |  |  | 0.677 |
| walker |  | 1117 | 12 | listing of 'sps-core/src/pipeline' |  |  | 0.677 |
| ns | 1121 |  | 169 | main.rs — Cache + Command::run dispatch | 1.9 | 1.7 | 0.700 |
| walker |  | 1131 | 14 | listing of 'sps-core/src/utils' |  |  | 0.700 |
| walker |  | 1148 | 17 | listing of 'sps-core/src/uninstall' |  |  | 0.701 |
| walker |  | 1165 | 17 | listing of 'sps-core/src/upgrade' |  |  | 0.703 |
| walker |  | 1185 | 20 | listing of 'sps-core/src/install' |  |  | 0.703 |
| walker |  | 1214 | 29 | pub item at sps-core/src/install/mod.rs:17 |  |  | 0.703 |
| walker |  | 1230 | 16 | mod/use plumbing in sps-core/src/pipeline/mod.rs |  |  | 0.703 |
| walker |  | 1255 | 25 | pub item body at sps-core/src/install/mod.rs:17 body 18 |  |  | 0.703 |
| walker |  | 1277 | 22 | mod/use plumbing in sps-core/src/utils/mod.rs |  |  | 0.703 |
| walker |  | 1362 | 85 | mod/use plumbing in sps-core/src/lib.rs |  |  | 0.704 |
| ns | 1393 |  | 272 | main.rs — pipeline-aware error printing on failure | 1.10 | 1.9 | 0.635 |
| walker |  | 1404 | 42 | mod/use plumbing in sps-core/src/check/mod.rs |  |  | 0.635 |
| walker |  | 1429 | 25 | pub item at sps/src/cli/update.rs:11 |  |  | 0.635 |
| ns | 1492 |  | 99 | Per-crate src/ listings | 2.1 |  | 0.616 |
| walker |  | 1542 | 113 | headings outline in README.md |  |  | 0.616 |
| walker |  | 1600 | 58 | README.md section #0 |  |  | 0.646 |
| walker |  | 1635 | 35 | listing of 'sps-common/src' |  |  | 0.711 |
| walker |  | 1651 | 16 | listing of 'sps-common/src/dependency' |  |  | 0.711 |
| ns | 1667 |  | 175 | sps-common lib.rs — module tree + re-exports | 2.2 |  | 0.669 |
| walker |  | 1676 | 25 | listing of 'sps-common/src/model' |  |  | 0.671 |
| walker |  | 1728 | 52 | pub item at sps-common/src/model/mod.rs:17 |  |  | 0.671 |
| walker |  | 1852 | 124 | mod/use plumbing in sps-common/src/lib.rs |  |  | 0.698 |
| ns | 1875 |  | 208 | sps-core lib.rs — module tree | 2.3 |  | 0.662 |
| walker |  | 1882 | 30 | pub item at sps/src/cli/status.rs:399 |  |  | 0.662 |
| walker |  | 1957 | 75 | mod/use plumbing in sps-core/src/upgrade/mod.rs |  |  | 0.662 |
| walker |  | 2034 | 77 | mod/use plumbing in sps-core/src/uninstall/mod.rs |  |  | 0.662 |
| walker |  | 2112 | 78 | mod/use plumbing in sps-core/src/install/mod.rs |  |  | 0.663 |
| ns | 2236 |  | 361 | sps-net lib.rs — re-exports + dependency on sps-common | 2.4 |  | 0.612 |
| walker |  | 2377 | 265 | entry item body at sps/src/main.rs:57 body 167 |  |  | 0.678 |
| ns | 2418 |  | 182 | sps-common — model module exports | 2.5 | 2.2 | 0.649 |
| walker |  | 2464 | 87 | mod/use plumbing in sps-common/src/model/mod.rs |  |  | 0.674 |
| ns | 2533 |  | 115 | sps-common — dependency module exports | 2.6 | 2.2 | 0.659 |
| walker |  | 2743 | 279 | mod/use plumbing in sps-net/src/lib.rs |  |  | 0.704 |
| ns | 2760 |  | 227 | sps-core/install — module roster | 2.7 | 2.3 | 0.683 |
| ns | 2775 |  | 15 | sps-core/install/cask sub-listing | 2.8 | 2.7 | 0.677 |
| walker |  | 2813 | 70 | pub item at sps/src/cli.rs:35 |  |  | 0.677 |
| ns | 2890 |  | 115 | Cask artifact handler enumeration | 2.9 | 2.8 | 0.646 |
| walker |  | 2912 | 99 | mod/use plumbing in sps-common/src/dependency/mod.rs |  |  | 0.662 |
| walker |  | 3010 | 98 | pub item at sps/src/cli.rs:44 |  |  | 0.709 |
| walker |  | 3034 | 24 | mod/use plumbing in sps/src/pipeline.rs |  |  | 0.709 |
| walker |  | 3109 | 75 | README.md section #1 |  |  | 0.709 |
| walker |  | 3124 | 15 | listing of 'sps-core/src/install/cask' |  |  | 0.717 |
| walker |  | 3140 | 16 | listing of 'sps-core/src/install/bottle' |  |  | 0.718 |
| walker |  | 3217 | 77 | pub-item names surface in sps-core/src/install/bottle/mod.rs |  |  | 0.718 |
| walker |  | 3217 | 0 | pub item at sps-core/src/install/bottle/mod.rs:34 |  |  | 0.718 |
| walker |  | 3217 | 0 | pub item at sps-core/src/install/bottle/mod.rs:159 |  |  | 0.718 |
| walker |  | 3261 | 44 | pub item at sps-core/src/install/bottle/mod.rs:18 |  |  | 0.718 |
| walker |  | 3311 | 50 | pub item at sps-core/src/install/bottle/mod.rs:165 |  |  | 0.718 |
| walker |  | 3336 | 25 | pub item body at sps-core/src/install/bottle/mod.rs:159 body 160 |  |  | 0.718 |
| walker |  | 3349 | 13 | pub-item doc lede at sps-core/src/install/bottle/mod.rs:18 |  |  | 0.718 |
| walker |  | 3369 | 20 | pub-item doc lede at sps-core/src/install/bottle/mod.rs:34 |  |  | 0.718 |
| ns | 3447 |  | 557 | Cask artifacts mod.rs — re-export wall | 2.10 | 2.9 | 0.661 |
| walker |  | 3461 | 92 | pub item body at sps-core/src/install/bottle/mod.rs:18 body 23 |  |  | 0.661 |
| ns | 3463 |  | 16 | sps-core/install/bottle sub-listing | 2.11 | 2.7 | 0.663 |
| ns | 3508 |  | 45 | sps-core/build — compile sub-listing | 2.12 | 2.3 | 0.652 |
| walker |  | 3578 | 117 | pub item body at sps-core/src/install/bottle/mod.rs:34 body 35 |  |  | 0.652 |
| ns | 3580 |  | 72 | sps-core — pipeline / check / uninstall / upgrade / utils sub-listings | 2.13 | 2.3 | 0.663 |
| ns | 3621 |  | 41 | sps-common — sub-tree listings (model / dependency) | 2.14 | 2.2 | 0.669 |
| walker |  | 3632 | 54 | pub-item names surface in sps/src/pipeline/planner.rs |  |  | 0.669 |
| ns | 3673 |  | 52 | sps bin — pipeline & cli sub-listings | 2.15 | 1.5 | 0.676 |
| walker |  | 3845 | 213 | mod/use plumbing in sps/src/main.rs |  |  | 0.676 |
| walker |  | 3999 | 154 | mod/use plumbing in sps-core/src/install/bottle/mod.rs |  |  | 0.676 |
| walker |  | 4070 | 71 | pub item at sps/src/cli/init.rs:16 |  |  | 0.676 |
| walker |  | 4095 | 25 | pub item at sps-core/src/utils/applescript.rs:75 |  |  | 0.676 |
| walker |  | 4160 | 65 | pub-item names surface in sps/src/cli/search.rs |  |  | 0.676 |
| walker |  | 4160 | 0 | pub item at sps/src/cli/search.rs:240 |  |  | 0.676 |
| ns | 4185 |  | 512 | Per-crate Cargo manifest signatures (name + deps highlights) | 2.16 |  | 0.642 |
| walker |  | 4187 | 27 | pub item at sps/src/cli/search.rs:23 |  |  | 0.642 |
| walker |  | 4239 | 52 | pub item at sps/src/cli/search.rs:42 |  |  | 0.642 |
| ns | 4301 |  | 116 | Config struct fields | 3.1 |  | 0.635 |
| walker |  | 4315 | 76 | pub item at sps/src/cli/search.rs:15 |  |  | 0.635 |
| ns | 4636 |  | 335 | Config path-method roster | 3.2 | 3.1 | 0.616 |
| walker |  | 4881 | 566 | entry item body at sps/src/main.rs:57 body 91 |  |  | 0.616 |
| walker |  | 4947 | 66 | pub-item names surface in sps/src/pipeline/runner.rs |  |  | 0.616 |
| walker |  | 4947 | 0 | pub item at sps/src/pipeline/runner.rs:56 |  |  | 0.616 |
| walker |  | 4993 | 46 | pub item at sps/src/pipeline/runner.rs:31 |  |  | 0.616 |
| walker |  | 5041 | 48 | pub item at sps/src/pipeline/runner.rs:38 |  |  | 0.616 |
| walker |  | 5106 | 65 | pub item at sps/src/pipeline/runner.rs:67 |  |  | 0.617 |
| ns | 5152 |  | 516 | Config — sps_root resolution + cellar/cask path bodies | 3.3 | 3.2 | 0.592 |
| walker |  | 5176 | 70 | README.md section #3 |  |  | 0.592 |
| walker |  | 5265 | 89 | pub item at sps/src/cli/info.rs:14 |  |  | 0.592 |
| walker |  | 5318 | 53 | pub item at sps-common/src/cache.rs:15 |  |  | 0.592 |
| walker |  | 5538 | 220 | pub-item names surface in sps-core/src/install/cask/mod.rs |  |  | 0.592 |
| walker |  | 5538 | 0 | pub item at sps-core/src/install/cask/mod.rs:37 |  |  | 0.592 |
| walker |  | 5538 | 0 | pub item at sps-core/src/install/cask/mod.rs:43 |  |  | 0.592 |
| walker |  | 5538 | 0 | pub item at sps-core/src/install/cask/mod.rs:49 |  |  | 0.592 |
| walker |  | 5538 | 0 | pub item at sps-core/src/install/cask/mod.rs:73 |  |  | 0.592 |
| walker |  | 5538 | 0 | pub item at sps-core/src/install/cask/mod.rs:686 |  |  | 0.592 |
| walker |  | 5554 | 16 | pub item body at sps-core/src/install/cask/mod.rs:43 body 44 |  |  | 0.592 |
| ns | 5564 |  | 412 | SpsError variant signatures | 3.4 |  | 0.572 |
| walker |  | 5573 | 19 | pub item body at sps-core/src/install/cask/mod.rs:73 body 74 |  |  | 0.572 |
| walker |  | 5618 | 45 | pub item at sps-core/src/install/cask/mod.rs:580 |  |  | 0.572 |
| walker |  | 5664 | 46 | pub item at sps-core/src/install/cask/mod.rs:77 |  |  | 0.572 |
| walker |  | 5711 | 47 | pub item at sps-core/src/install/cask/mod.rs:612 |  |  | 0.572 |
| walker |  | 5765 | 54 | pub item at sps-core/src/install/cask/mod.rs:232 |  |  | 0.572 |
| walker |  | 5807 | 42 | pub item body at sps-core/src/install/cask/mod.rs:37 body 38 |  |  | 0.572 |
| ns | 5898 |  | 334 | InstallTargetIdentifier + Formula struct fields | 3.5 | 2.5 | 0.553 |
| walker |  | 5941 | 134 | pub item at sps-core/src/install/cask/mod.rs:25 |  |  | 0.553 |
| walker |  | 5959 | 18 | pub-item doc lede at sps-core/src/install/cask/mod.rs:37 |  |  | 0.553 |
| walker |  | 5979 | 20 | pub-item doc lede at sps-core/src/install/cask/mod.rs:43 |  |  | 0.553 |
| walker |  | 6023 | 44 | pub-item doc lede at sps-core/src/install/cask/mod.rs:49 |  |  | 0.553 |
| walker |  | 6074 | 51 | pub-item doc lede at sps-core/src/install/cask/mod.rs:686 |  |  | 0.553 |
| walker |  | 6083 | 9 | pub item body at sps-core/src/install/cask/mod.rs:49 body 70 |  |  | 0.553 |
| walker |  | 6177 | 94 | pub item at sps/src/cli/upgrade.rs:12 |  |  | 0.553 |
| walker |  | 6238 | 61 | pub item at sps-common/src/formulary.rs:12 |  |  | 0.553 |
| ns | 6295 |  | 397 | Cask struct fields | 3.6 |  | 0.533 |
| walker |  | 6346 | 108 | pub item at sps/src/cli/reinstall.rs:12 |  |  | 0.533 |
| walker |  | 6385 | 39 | pub item at sps-common/src/model/version.rs:11 |  |  | 0.533 |
| ns | 6433 |  | 138 | InstalledArtifact variants | 3.7 |  | 0.528 |
| walker |  | 6479 | 94 | README.md section #2 |  |  | 0.528 |
| ns | 6715 |  | 282 | Dependency + DependencyTag bitflags | 3.8 |  | 0.518 |
| walker |  | 6725 | 246 | mod/use plumbing in sps-core/src/install/cask/mod.rs |  |  | 0.518 |
| walker |  | 6737 | 12 | pub-item doc lede at sps-common/src/cache.rs:15 |  |  | 0.518 |
| walker |  | 6803 | 66 | README.md section #9 |  |  | 0.518 |
| ns | 6913 |  | 198 | Requirement enum (macOS / Xcode / Other) | 3.9 |  | 0.511 |
| walker |  | 6937 | 134 | pub item at sps/src/cli/list.rs:16 |  |  | 0.511 |
| walker |  | 6960 | 23 | mod/use plumbing in sps-common/src/error.rs |  |  | 0.511 |
| walker |  | 7024 | 64 | pub item at sps-common/src/dependency/requirement.rs:7 |  |  | 0.514 |
| walker |  | 7108 | 84 | README.md section #6 |  |  | 0.514 |
| ns | 7119 |  | 206 | Cache API surface | 3.10 |  | 0.508 |
| walker |  | 7134 | 26 | pub-item names surface in sps-common/src/keg.rs |  |  | 0.508 |
| walker |  | 7156 | 22 | pub item at sps-common/src/keg.rs:21 |  |  | 0.509 |
| walker |  | 7208 | 52 | pub item at sps-common/src/keg.rs:13 |  |  | 0.509 |
| walker |  | 7221 | 13 | pub-item doc lede at sps-common/src/keg.rs:21 |  |  | 0.509 |
| walker |  | 7236 | 15 | pub-item doc lede at sps-common/src/keg.rs:13 |  |  | 0.509 |
| ns | 7312 |  | 193 | InstalledKeg + KegRegistry signatures | 3.11 |  | 0.508 |
| walker |  | 7323 | 87 | README.md section #7 |  |  | 0.508 |
| walker |  | 7352 | 29 | pub-item names surface in sps-common/src/config.rs |  |  | 0.508 |
| walker |  | 7352 | 0 | pub item at sps-common/src/config.rs:199 |  |  | 0.508 |
| walker |  | 7361 | 9 | pub item body at sps-common/src/config.rs:199 body 200 |  |  | 0.508 |
| walker |  | 7465 | 104 | pub item at sps-common/src/config.rs:15 |  |  | 0.518 |
| walker |  | 7488 | 23 | pub item body at sps-core/src/install/cask/mod.rs:49 body 50 |  |  | 0.518 |
| ns | 7654 |  | 342 | Pipeline shared types — JobAction / PlannedJob / WorkerJob / PipelineEvent header | 4.1 |  | 0.504 |
| walker |  | 7673 | 185 | pub item at sps/src/cli/uninstall.rs:17 |  |  | 0.504 |
| walker |  | 7706 | 33 | pub-item names surface in sps-common/src/error.rs |  |  | 0.504 |
| walker |  | 7726 | 20 | mod/use plumbing in sps-common/src/dependency/requirement.rs |  |  | 0.504 |
| walker |  | 7753 | 27 | pub item body at sps-core/src/install/cask/mod.rs:686 body 687 |  |  | 0.504 |
| walker |  | 7776 | 23 | pub-item names surface in sps-common/src/dependency/definition.rs |  |  | 0.505 |
| walker |  | 7839 | 63 | pub item at sps-common/src/dependency/definition.rs:53 |  |  | 0.507 |
| walker |  | 7895 | 56 | pub item at sps-common/src/dependency/definition.rs:31 |  |  | 0.511 |
| walker |  | 7910 | 15 | pub-item doc lede at sps-common/src/model/version.rs:11 |  |  | 0.511 |
| walker |  | 7987 | 77 | pub item at sps-core/src/upgrade/cask.rs:15 |  |  | 0.511 |
| walker |  | 8065 | 78 | pub item at sps-core/src/uninstall/formula.rs:10 |  |  | 0.511 |
| walker |  | 8135 | 70 | mod/use plumbing in sps/src/cli/update.rs |  |  | 0.511 |
| ns | 8151 |  | 497 | JobProcessingState + DownloadOutcome + PlannedOperations | 4.2 | 4.1 | 0.497 |
| walker |  | 8161 | 26 | pub-item names surface in sps-core/src/check/update.rs |  |  | 0.497 |
| walker |  | 8207 | 46 | pub item at sps-core/src/check/update.rs:89 |  |  | 0.497 |
| walker |  | 8279 | 72 | pub item at sps-core/src/check/update.rs:23 |  |  | 0.497 |
| ns | 8373 |  | 222 | CommandType + PipelineFlags + run_pipeline signature | 4.3 |  | 0.505 |
| walker |  | 8392 | 113 | README.md section #8 |  |  | 0.505 |
| walker |  | 8419 | 27 | mod/use plumbing in sps-common/src/model/artifact.rs |  |  | 0.505 |
| walker |  | 8647 | 228 | [dependencies] in sps-net/Cargo.toml |  |  | 0.505 |
| walker |  | 8681 | 34 | impl method sigs in sps/src/cli.rs |  |  | 0.505 |
| ns | 8687 |  | 314 | OperationPlanner + plan_operations signature | 4.4 |  | 0.494 |
| walker |  | 8791 | 110 | [package] in sps-net/Cargo.toml |  |  | 0.496 |
| walker |  | 8869 | 78 | mod/use plumbing in sps/src/cli/reinstall.rs |  |  | 0.496 |
| ns | 9007 |  | 320 | DependencyResolver — strategy + status + ResolvedDependency | 4.5 |  | 0.485 |
| walker |  | 9107 | 238 | [dependencies] in sps-common/Cargo.toml |  |  | 0.485 |
| walker |  | 9217 | 110 | [package] in sps-common/Cargo.toml |  |  | 0.491 |
| walker |  | 9331 | 114 | [package] in sps-core/Cargo.toml |  |  | 0.491 |
| ns | 9331 |  | 324 | DependencyResolver — ResolutionContext + ResolvedGraph | 4.6 | 4.5 | 0.491 |
| walker |  | 9426 | 95 | pub item at sps-core/src/upgrade/source.rs:19 |  |  | 0.491 |
| walker |  | 9444 | 18 | pub-item doc lede at sps-core/src/upgrade/cask.rs:15 |  |  | 0.491 |
| ns | 9518 |  | 187 | core worker entry — execute_sync_job | 4.7 |  | 0.486 |
| ns | 9675 |  | 157 | core worker pool — start_worker_pool_manager | 4.8 | 4.7 | 0.483 |
| ns | 9911 |  | 236 | Install entry-points — install_bottle / build_from_source / install_cask signatures | 4.9 |  | 0.479 |
| ns | 10046 |  | 135 | Bottle platform selection — get_bottle_for_platform | 4.10 | 4.9 | 0.478 |
| ns | 10196 |  | 150 | API entry-points (sps-net::api) | 4.11 |  | 0.475 |
| ns | 10289 |  | 93 | Uninstall + Upgrade entry-point fn names | 4.12 |  | 0.474 |

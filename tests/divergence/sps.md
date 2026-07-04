Score(3000)=0.727 I=0.868 C=0.609 ns_rows≤3K=19/49 (reached=9 partial=4 missing=6)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 47 | 47 | listing of '.' |  |  | 1.000 |
| ns | 47 |  | 47 | Top-level repo listing | 1.1 |  | 1.000 |
| walker |  | 82 | 35 | README headline in README.md |  |  | 1.000 |
| ns | 144 |  | 97 | README lede — what sps is | 1.2 |  | 0.772 |
| walker |  | 157 | 75 | [package] in Cargo.toml |  |  | 0.810 |
| walker |  | 165 | 8 | listing of 'sps' |  |  | 0.810 |
| walker |  | 183 | 18 | listing of 'sps/src' |  |  | 0.811 |
| walker |  | 195 | 12 | listing of 'sps/src/pipeline' |  |  | 0.811 |
| walker |  | 210 | 15 | entry item at sps/src/main.rs:57 |  |  | 0.811 |
| walker |  | 217 | 7 | entry item body at sps/src/main.rs:57 body 190 |  |  | 0.811 |
| ns | 219 |  | 75 | Workspace members | 1.3 |  | 0.830 |
| walker |  | 229 | 12 | entry item body at sps/src/main.rs:57 body 58 |  |  | 0.830 |
| walker |  | 242 | 13 | entry item body at sps/src/main.rs:57 body 90 |  |  | 0.830 |
| walker |  | 259 | 17 | entry item body at sps/src/main.rs:57 body 158 |  |  | 0.830 |
| walker |  | 278 | 19 | entry item body at sps/src/main.rs:57 body 189 |  |  | 0.830 |
| walker |  | 302 | 24 | entry item body at sps/src/main.rs:57 body 83 |  |  | 0.830 |
| walker |  | 310 | 8 | listing of 'sps-common' |  |  | 0.830 |
| walker |  | 318 | 8 | listing of 'sps-core' |  |  | 0.830 |
| walker |  | 326 | 8 | listing of 'sps-net' |  |  | 0.830 |
| ns | 327 |  | 108 | CLI subcommand enum | 1.4 |  | 0.666 |
| walker |  | 379 | 53 | entry item body at sps/src/main.rs:57 body 85 |  |  | 0.666 |
| walker |  | 435 | 56 | entry item body at sps/src/main.rs:57 body 141 |  |  | 0.667 |
| walker |  | 493 | 58 | entry item body at sps/src/main.rs:57 body 78 |  |  | 0.667 |
| ns | 503 |  | 176 | CLI per-subcommand module list | 1.5 |  | 0.526 |
| walker |  | 533 | 40 | listing of 'sps/src/cli' |  |  | 0.528 |
| walker |  | 599 | 66 | entry item body at sps/src/main.rs:57 body 72 |  |  | 0.533 |
| walker |  | 671 | 72 | entry item body at sps/src/main.rs:57 body 135 |  |  | 0.535 |
| ns | 678 |  | 175 | main.rs — Tokio entry signature + Init early-out | 1.6 |  | 0.466 |
| ns | 744 |  | 66 | main.rs — Config::load failure hint | 1.7 | 1.6 | 0.498 |
| walker |  | 751 | 80 | entry item body at sps/src/main.rs:57 body 159 |  |  | 0.504 |
| walker |  | 772 | 21 | pub-item names surface in sps/src/cli.rs |  |  | 0.505 |
| walker |  | 793 | 21 | listing of 'sps-net/src' |  |  | 0.506 |
| walker |  | 921 | 128 | entry item body at sps/src/main.rs:57 body 60 |  |  | 0.626 |
| ns | 936 |  | 192 | main.rs — auto-update gate | 1.8 | 1.7 | 0.570 |
| walker |  | 1052 | 131 | entry item body at sps/src/main.rs:57 body 146 |  |  | 0.673 |
| walker |  | 1073 | 21 | pub item at sps/src/cli/update.rs:11 |  |  | 0.673 |
| walker |  | 1098 | 25 | listing of 'sps-core/src' |  |  | 0.677 |
| ns | 1105 |  | 169 | main.rs — Cache + Command::run dispatch | 1.9 | 1.7 | 0.699 |
| walker |  | 1110 | 12 | listing of 'sps-core/src/check' |  |  | 0.699 |
| walker |  | 1122 | 12 | listing of 'sps-core/src/pipeline' |  |  | 0.700 |
| walker |  | 1136 | 14 | listing of 'sps-core/src/utils' |  |  | 0.700 |
| walker |  | 1153 | 17 | listing of 'sps-core/src/uninstall' |  |  | 0.701 |
| walker |  | 1170 | 17 | listing of 'sps-core/src/upgrade' |  |  | 0.703 |
| walker |  | 1190 | 20 | listing of 'sps-core/src/install' |  |  | 0.703 |
| walker |  | 1217 | 27 | pub item at sps-core/src/install/mod.rs:17 |  |  | 0.703 |
| walker |  | 1233 | 16 | mod/use plumbing in sps-core/src/pipeline/mod.rs |  |  | 0.703 |
| walker |  | 1256 | 23 | pub item body at sps-core/src/install/mod.rs:17 body 18 |  |  | 0.703 |
| walker |  | 1274 | 18 | mod/use plumbing in sps-core/src/utils/mod.rs |  |  | 0.703 |
| walker |  | 1351 | 77 | mod/use plumbing in sps-core/src/lib.rs |  |  | 0.704 |
| ns | 1377 |  | 272 | main.rs — pipeline-aware error printing on failure | 1.10 | 1.9 | 0.635 |
| walker |  | 1398 | 47 | mod/use plumbing in sps-core/src/check/mod.rs |  |  | 0.635 |
| ns | 1476 |  | 99 | Per-crate src/ listings | 2.1 |  | 0.616 |
| walker |  | 1511 | 113 | headings outline in README.md |  |  | 0.616 |
| walker |  | 1581 | 70 | README.md section #0 |  |  | 0.662 |
| walker |  | 1616 | 35 | listing of 'sps-common/src' |  |  | 0.726 |
| walker |  | 1632 | 16 | listing of 'sps-common/src/dependency' |  |  | 0.727 |
| ns | 1651 |  | 175 | sps-common lib.rs — module tree + re-exports | 2.2 |  | 0.684 |
| walker |  | 1657 | 25 | listing of 'sps-common/src/model' |  |  | 0.686 |
| walker |  | 1707 | 50 | pub item at sps-common/src/model/mod.rs:17 |  |  | 0.686 |
| walker |  | 1825 | 118 | mod/use plumbing in sps-common/src/lib.rs |  |  | 0.712 |
| walker |  | 1853 | 28 | pub item at sps/src/cli/status.rs:399 |  |  | 0.712 |
| ns | 1859 |  | 208 | sps-core lib.rs — module tree | 2.3 |  | 0.676 |
| walker |  | 1924 | 71 | mod/use plumbing in sps-core/src/upgrade/mod.rs |  |  | 0.676 |
| walker |  | 1997 | 73 | mod/use plumbing in sps-core/src/uninstall/mod.rs |  |  | 0.676 |
| walker |  | 2076 | 79 | mod/use plumbing in sps-core/src/install/mod.rs |  |  | 0.677 |
| ns | 2220 |  | 361 | sps-net lib.rs — re-exports + dependency on sps-common | 2.4 |  | 0.625 |
| walker |  | 2348 | 272 | entry item body at sps/src/main.rs:57 body 167 |  |  | 0.695 |
| ns | 2402 |  | 182 | sps-common — model module exports | 2.5 | 2.2 | 0.664 |
| ns | 2517 |  | 115 | sps-common — dependency module exports | 2.6 | 2.2 | 0.649 |
| walker |  | 2628 | 280 | mod/use plumbing in sps-net/src/lib.rs |  |  | 0.698 |
| walker |  | 2718 | 90 | mod/use plumbing in sps-common/src/model/mod.rs |  |  | 0.724 |
| ns | 2744 |  | 227 | sps-core/install — module roster | 2.7 | 2.3 | 0.703 |
| ns | 2759 |  | 15 | sps-core/install/cask sub-listing | 2.8 | 2.7 | 0.697 |
| walker |  | 2789 | 71 | pub item at sps/src/cli.rs:35 |  |  | 0.697 |
| ns | 2874 |  | 115 | Cask artifact handler enumeration | 2.9 | 2.8 | 0.664 |
| walker |  | 2886 | 97 | mod/use plumbing in sps-common/src/dependency/mod.rs |  |  | 0.680 |
| walker |  | 2984 | 98 | pub item at sps/src/cli.rs:44 |  |  | 0.727 |
| walker |  | 3008 | 24 | mod/use plumbing in sps/src/pipeline.rs |  |  | 0.727 |
| walker |  | 3023 | 15 | listing of 'sps-core/src/install/cask' |  |  | 0.736 |
| walker |  | 3075 | 52 | pub-item names surface in sps/src/pipeline/planner.rs |  |  | 0.736 |
| walker |  | 3091 | 16 | listing of 'sps-core/src/install/bottle' |  |  | 0.737 |
| walker |  | 3166 | 75 | pub-item names surface in sps-core/src/install/bottle/mod.rs |  |  | 0.737 |
| walker |  | 3166 | 0 | pub item at sps-core/src/install/bottle/mod.rs:34 |  |  | 0.737 |
| walker |  | 3166 | 0 | pub item at sps-core/src/install/bottle/mod.rs:159 |  |  | 0.737 |
| walker |  | 3210 | 44 | pub item at sps-core/src/install/bottle/mod.rs:18 |  |  | 0.737 |
| walker |  | 3260 | 50 | pub item at sps-core/src/install/bottle/mod.rs:165 |  |  | 0.737 |
| walker |  | 3283 | 23 | pub item body at sps-core/src/install/bottle/mod.rs:159 body 160 |  |  | 0.737 |
| walker |  | 3296 | 13 | pub-item doc lede at sps-core/src/install/bottle/mod.rs:18 |  |  | 0.737 |
| walker |  | 3316 | 20 | pub-item doc lede at sps-core/src/install/bottle/mod.rs:34 |  |  | 0.737 |
| walker |  | 3406 | 90 | pub item body at sps-core/src/install/bottle/mod.rs:18 body 23 |  |  | 0.737 |
| ns | 3431 |  | 557 | Cask artifacts mod.rs — re-export wall | 2.10 | 2.9 | 0.678 |
| ns | 3447 |  | 16 | sps-core/install/bottle sub-listing | 2.11 | 2.7 | 0.680 |
| ns | 3492 |  | 45 | sps-core/build — compile sub-listing | 2.12 | 2.3 | 0.668 |
| walker |  | 3521 | 115 | pub item body at sps-core/src/install/bottle/mod.rs:34 body 35 |  |  | 0.668 |
| ns | 3564 |  | 72 | sps-core — pipeline / check / uninstall / upgrade / utils sub-listings | 2.13 | 2.3 | 0.679 |
| walker |  | 3588 | 67 | pub item at sps/src/cli/init.rs:16 |  |  | 0.679 |
| ns | 3605 |  | 41 | sps-common — sub-tree listings (model / dependency) | 2.14 | 2.2 | 0.685 |
| ns | 3657 |  | 52 | sps bin — pipeline & cli sub-listings | 2.15 | 1.5 | 0.692 |
| walker |  | 3745 | 157 | mod/use plumbing in sps-core/src/install/bottle/mod.rs |  |  | 0.692 |
| walker |  | 3964 | 219 | mod/use plumbing in sps/src/main.rs |  |  | 0.692 |
| walker |  | 3987 | 23 | pub item at sps-core/src/utils/applescript.rs:75 |  |  | 0.692 |
| walker |  | 4077 | 90 | README.md section #1 |  |  | 0.692 |
| walker |  | 4140 | 63 | pub-item names surface in sps/src/cli/search.rs |  |  | 0.692 |
| walker |  | 4140 | 0 | pub item at sps/src/cli/search.rs:240 |  |  | 0.692 |
| ns | 4161 |  | 504 | Per-crate Cargo manifest signatures (name + deps highlights) | 2.16 |  | 0.657 |
| walker |  | 4165 | 25 | pub item at sps/src/cli/search.rs:23 |  |  | 0.657 |
| walker |  | 4217 | 52 | pub item at sps/src/cli/search.rs:42 |  |  | 0.657 |
| ns | 4273 |  | 112 | Config struct fields | 3.1 |  | 0.650 |
| walker |  | 4293 | 76 | pub item at sps/src/cli/search.rs:15 |  |  | 0.650 |
| walker |  | 4357 | 64 | pub-item names surface in sps/src/pipeline/runner.rs |  |  | 0.650 |
| walker |  | 4357 | 0 | pub item at sps/src/pipeline/runner.rs:56 |  |  | 0.650 |
| walker |  | 4401 | 44 | pub item at sps/src/pipeline/runner.rs:31 |  |  | 0.650 |
| walker |  | 4449 | 48 | pub item at sps/src/pipeline/runner.rs:38 |  |  | 0.650 |
| walker |  | 4514 | 65 | pub item at sps/src/pipeline/runner.rs:67 |  |  | 0.651 |
| ns | 4558 |  | 285 | Config path-method roster | 3.2 | 3.1 | 0.631 |
| walker |  | 4563 | 49 | pub item at sps-common/src/cache.rs:15 |  |  | 0.631 |
| ns | 5074 |  | 516 | Config — sps_root resolution + cellar/cask path bodies | 3.3 | 3.2 | 0.605 |
| walker |  | 5151 | 588 | entry item body at sps/src/main.rs:57 body 91 |  |  | 0.605 |
| walker |  | 5241 | 90 | pub item at sps/src/cli/info.rs:14 |  |  | 0.605 |
| ns | 5428 |  | 354 | SpsError variant signatures | 3.4 |  | 0.585 |
| walker |  | 5459 | 218 | pub-item names surface in sps-core/src/install/cask/mod.rs |  |  | 0.585 |
| walker |  | 5459 | 0 | pub item at sps-core/src/install/cask/mod.rs:37 |  |  | 0.585 |
| walker |  | 5459 | 0 | pub item at sps-core/src/install/cask/mod.rs:43 |  |  | 0.585 |
| walker |  | 5459 | 0 | pub item at sps-core/src/install/cask/mod.rs:49 |  |  | 0.585 |
| walker |  | 5459 | 0 | pub item at sps-core/src/install/cask/mod.rs:73 |  |  | 0.585 |
| walker |  | 5459 | 0 | pub item at sps-core/src/install/cask/mod.rs:686 |  |  | 0.585 |
| walker |  | 5473 | 14 | pub item body at sps-core/src/install/cask/mod.rs:43 body 44 |  |  | 0.585 |
| walker |  | 5490 | 17 | pub item body at sps-core/src/install/cask/mod.rs:73 body 74 |  |  | 0.585 |
| walker |  | 5535 | 45 | pub item at sps-core/src/install/cask/mod.rs:580 |  |  | 0.585 |
| walker |  | 5581 | 46 | pub item at sps-core/src/install/cask/mod.rs:77 |  |  | 0.585 |
| walker |  | 5628 | 47 | pub item at sps-core/src/install/cask/mod.rs:612 |  |  | 0.585 |
| walker |  | 5682 | 54 | pub item at sps-core/src/install/cask/mod.rs:232 |  |  | 0.585 |
| walker |  | 5722 | 40 | pub item body at sps-core/src/install/cask/mod.rs:37 body 38 |  |  | 0.585 |
| ns | 5758 |  | 330 | InstallTargetIdentifier + Formula struct fields | 3.5 | 2.5 | 0.566 |
| walker |  | 5854 | 132 | pub item at sps-core/src/install/cask/mod.rs:25 |  |  | 0.566 |
| walker |  | 5874 | 20 | pub-item doc lede at sps-core/src/install/cask/mod.rs:37 |  |  | 0.566 |
| walker |  | 5894 | 20 | pub-item doc lede at sps-core/src/install/cask/mod.rs:43 |  |  | 0.566 |
| walker |  | 5938 | 44 | pub-item doc lede at sps-core/src/install/cask/mod.rs:49 |  |  | 0.566 |
| walker |  | 5989 | 51 | pub-item doc lede at sps-core/src/install/cask/mod.rs:686 |  |  | 0.566 |
| walker |  | 5996 | 7 | pub item body at sps-core/src/install/cask/mod.rs:49 body 70 |  |  | 0.566 |
| walker |  | 6053 | 57 | pub item at sps-common/src/formulary.rs:12 |  |  | 0.566 |
| walker |  | 6088 | 35 | pub item at sps-common/src/model/version.rs:11 |  |  | 0.566 |
| ns | 6151 |  | 393 | Cask struct fields | 3.6 |  | 0.545 |
| walker |  | 6188 | 100 | pub item at sps/src/cli/upgrade.rs:12 |  |  | 0.545 |
| walker |  | 6268 | 80 | README.md section #3 |  |  | 0.545 |
| ns | 6269 |  | 118 | InstalledArtifact variants | 3.7 |  | 0.540 |
| walker |  | 6377 | 109 | pub item at sps/src/cli/reinstall.rs:12 |  |  | 0.540 |
| walker |  | 6389 | 12 | pub-item doc lede at sps-common/src/cache.rs:15 |  |  | 0.540 |
| ns | 6543 |  | 274 | Dependency + DependencyTag bitflags | 3.8 |  | 0.530 |
| walker |  | 6652 | 263 | mod/use plumbing in sps-core/src/install/cask/mod.rs |  |  | 0.530 |
| ns | 6739 |  | 196 | Requirement enum (macOS / Xcode / Other) | 3.9 |  | 0.523 |
| walker |  | 6756 | 104 | README.md section #2 |  |  | 0.523 |
| walker |  | 6886 | 130 | pub item at sps/src/cli/list.rs:16 |  |  | 0.523 |
| ns | 6925 |  | 186 | Cache API surface | 3.10 |  | 0.518 |
| walker |  | 6946 | 60 | pub item at sps-common/src/dependency/requirement.rs:7 |  |  | 0.520 |
| walker |  | 6970 | 24 | pub-item names surface in sps-common/src/keg.rs |  |  | 0.520 |
| walker |  | 6990 | 20 | pub item at sps-common/src/keg.rs:21 |  |  | 0.520 |
| walker |  | 7040 | 50 | pub item at sps-common/src/keg.rs:13 |  |  | 0.520 |
| walker |  | 7055 | 15 | pub-item doc lede at sps-common/src/keg.rs:13 |  |  | 0.520 |
| walker |  | 7070 | 15 | pub-item doc lede at sps-common/src/keg.rs:21 |  |  | 0.520 |
| walker |  | 7096 | 26 | mod/use plumbing in sps-common/src/error.rs |  |  | 0.520 |
| ns | 7098 |  | 173 | InstalledKeg + KegRegistry signatures | 3.11 |  | 0.519 |
| walker |  | 7123 | 27 | pub-item names surface in sps-common/src/config.rs |  |  | 0.519 |
| walker |  | 7123 | 0 | pub item at sps-common/src/config.rs:199 |  |  | 0.519 |
| walker |  | 7130 | 7 | pub item body at sps-common/src/config.rs:199 body 200 |  |  | 0.519 |
| walker |  | 7151 | 21 | pub item body at sps-core/src/install/cask/mod.rs:49 body 50 |  |  | 0.519 |
| walker |  | 7232 | 81 | README.md section #9 |  |  | 0.519 |
| walker |  | 7334 | 102 | pub item at sps-common/src/config.rs:15 |  |  | 0.529 |
| ns | 7436 |  | 338 | Pipeline shared types — JobAction / PlannedJob / WorkerJob / PipelineEvent header | 4.1 |  | 0.515 |
| walker |  | 7515 | 181 | pub item at sps/src/cli/uninstall.rs:17 |  |  | 0.515 |
| walker |  | 7540 | 25 | pub item body at sps-core/src/install/cask/mod.rs:686 body 687 |  |  | 0.515 |
| walker |  | 7573 | 33 | pub-item names surface in sps-common/src/error.rs |  |  | 0.516 |
| walker |  | 7594 | 21 | pub-item names surface in sps-common/src/dependency/definition.rs |  |  | 0.516 |
| walker |  | 7655 | 61 | pub item at sps-common/src/dependency/definition.rs:53 |  |  | 0.518 |
| walker |  | 7709 | 54 | pub item at sps-common/src/dependency/definition.rs:31 |  |  | 0.522 |
| walker |  | 7818 | 109 | README.md section #6 |  |  | 0.522 |
| walker |  | 7833 | 15 | pub-item doc lede at sps-common/src/model/version.rs:11 |  |  | 0.522 |
| walker |  | 7857 | 24 | pub-item names surface in sps-core/src/check/update.rs |  |  | 0.522 |
| walker |  | 7903 | 46 | pub item at sps-core/src/check/update.rs:89 |  |  | 0.522 |
| ns | 7933 |  | 497 | JobProcessingState + DownloadOutcome + PlannedOperations | 4.2 | 4.1 | 0.508 |
| walker |  | 7973 | 70 | pub item at sps-core/src/check/update.rs:23 |  |  | 0.508 |
| walker |  | 8085 | 112 | README.md section #7 |  |  | 0.508 |
| ns | 8149 |  | 216 | CommandType + PipelineFlags + run_pipeline signature | 4.3 |  | 0.515 |
| walker |  | 8160 | 75 | pub item at sps-core/src/upgrade/cask.rs:15 |  |  | 0.515 |
| walker |  | 8236 | 76 | pub item at sps-core/src/uninstall/formula.rs:10 |  |  | 0.515 |
| walker |  | 8261 | 25 | mod/use plumbing in sps-common/src/dependency/requirement.rs |  |  | 0.515 |
| walker |  | 8336 | 75 | mod/use plumbing in sps/src/cli/update.rs |  |  | 0.515 |
| walker |  | 8364 | 28 | mod/use plumbing in sps-common/src/model/artifact.rs |  |  | 0.515 |
| ns | 8457 |  | 308 | OperationPlanner + plan_operations signature | 4.4 |  | 0.504 |
| walker |  | 8600 | 236 | [dependencies] in sps-common/Cargo.toml |  |  | 0.504 |
| ns | 8771 |  | 314 | DependencyResolver — strategy + status + ResolvedDependency | 4.5 |  | 0.493 |
| walker |  | 8836 | 236 | [dependencies] in sps-net/Cargo.toml |  |  | 0.493 |
| walker |  | 8948 | 112 | [package] in sps-common/Cargo.toml |  |  | 0.495 |
| walker |  | 9060 | 112 | [package] in sps-core/Cargo.toml |  |  | 0.501 |
| ns | 9097 |  | 326 | DependencyResolver — ResolutionContext + ResolvedGraph | 4.6 | 4.5 | 0.492 |
| walker |  | 9172 | 112 | [package] in sps-net/Cargo.toml |  |  | 0.501 |
| walker |  | 9208 | 36 | impl method sigs in sps/src/cli.rs |  |  | 0.501 |
| ns | 9280 |  | 183 | core worker entry — execute_sync_job | 4.7 |  | 0.496 |
| walker |  | 9301 | 93 | pub item at sps-core/src/upgrade/source.rs:19 |  |  | 0.496 |
| walker |  | 9319 | 18 | pub-item doc lede at sps-core/src/upgrade/cask.rs:15 |  |  | 0.496 |
| ns | 9433 |  | 153 | core worker pool — start_worker_pool_manager | 4.8 | 4.7 | 0.493 |
| walker |  | 9452 | 133 | README.md section #8 |  |  | 0.493 |
| walker |  | 9540 | 88 | mod/use plumbing in sps/src/cli/reinstall.rs |  |  | 0.493 |
| ns | 9653 |  | 220 | Install entry-points — install_bottle / build_from_source / install_cask signatures | 4.9 |  | 0.489 |
| ns | 9784 |  | 131 | Bottle platform selection — get_bottle_for_platform | 4.10 | 4.9 | 0.487 |
| ns | 9912 |  | 128 | API entry-points (sps-net::api) | 4.11 |  | 0.484 |
| ns | 9983 |  | 71 | Uninstall + Upgrade entry-point fn names | 4.12 |  | 0.484 |

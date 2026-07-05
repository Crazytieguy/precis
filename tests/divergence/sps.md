Score(3000)=0.680 I=0.813 C=0.569 ns_rows≤3K=19/49 (reached=8 partial=4 missing=7)

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
| walker |  | 1409 | 47 | mod/use plumbing in sps-core/src/check/mod.rs |  |  | 0.635 |
| walker |  | 1434 | 25 | pub item at sps/src/cli/update.rs:11 |  |  | 0.635 |
| ns | 1492 |  | 99 | Per-crate src/ listings | 2.1 |  | 0.616 |
| walker |  | 1547 | 113 | headings outline in README.md |  |  | 0.616 |
| walker |  | 1615 | 68 | README.md section #0 |  |  | 0.662 |
| walker |  | 1650 | 35 | listing of 'sps-common/src' |  |  | 0.726 |
| walker |  | 1666 | 16 | listing of 'sps-common/src/dependency' |  |  | 0.727 |
| ns | 1667 |  | 175 | sps-common lib.rs — module tree + re-exports | 2.2 |  | 0.684 |
| walker |  | 1691 | 25 | listing of 'sps-common/src/model' |  |  | 0.686 |
| walker |  | 1743 | 52 | pub item at sps-common/src/model/mod.rs:17 |  |  | 0.686 |
| walker |  | 1867 | 124 | mod/use plumbing in sps-common/src/lib.rs |  |  | 0.712 |
| ns | 1875 |  | 208 | sps-core lib.rs — module tree | 2.3 |  | 0.676 |
| walker |  | 1897 | 30 | pub item at sps/src/cli/status.rs:399 |  |  | 0.676 |
| walker |  | 1972 | 75 | mod/use plumbing in sps-core/src/upgrade/mod.rs |  |  | 0.676 |
| walker |  | 2049 | 77 | mod/use plumbing in sps-core/src/uninstall/mod.rs |  |  | 0.676 |
| walker |  | 2132 | 83 | mod/use plumbing in sps-core/src/install/mod.rs |  |  | 0.677 |
| ns | 2236 |  | 361 | sps-net lib.rs — re-exports + dependency on sps-common | 2.4 |  | 0.625 |
| walker |  | 2402 | 270 | entry item body at sps/src/main.rs:57 body 167 |  |  | 0.695 |
| ns | 2418 |  | 182 | sps-common — model module exports | 2.5 | 2.2 | 0.664 |
| ns | 2533 |  | 115 | sps-common — dependency module exports | 2.6 | 2.2 | 0.649 |
| walker |  | 2686 | 284 | mod/use plumbing in sps-net/src/lib.rs |  |  | 0.698 |
| ns | 2760 |  | 227 | sps-core/install — module roster | 2.7 | 2.3 | 0.678 |
| ns | 2775 |  | 15 | sps-core/install/cask sub-listing | 2.8 | 2.7 | 0.673 |
| walker |  | 2778 | 92 | mod/use plumbing in sps-common/src/model/mod.rs |  |  | 0.697 |
| walker |  | 2877 | 99 | mod/use plumbing in sps-common/src/dependency/mod.rs |  |  | 0.714 |
| ns | 2890 |  | 115 | Cask artifact handler enumeration | 2.9 | 2.8 | 0.680 |
| walker |  | 2952 | 75 | pub item at sps/src/cli.rs:35 |  |  | 0.680 |
| walker |  | 3050 | 98 | pub item at sps/src/cli.rs:44 |  |  | 0.727 |
| walker |  | 3074 | 24 | mod/use plumbing in sps/src/pipeline.rs |  |  | 0.727 |
| walker |  | 3089 | 15 | listing of 'sps-core/src/install/cask' |  |  | 0.736 |
| walker |  | 3105 | 16 | listing of 'sps-core/src/install/bottle' |  |  | 0.737 |
| walker |  | 3182 | 77 | pub-item names surface in sps-core/src/install/bottle/mod.rs |  |  | 0.737 |
| walker |  | 3182 | 0 | pub item at sps-core/src/install/bottle/mod.rs:34 |  |  | 0.737 |
| walker |  | 3182 | 0 | pub item at sps-core/src/install/bottle/mod.rs:159 |  |  | 0.737 |
| walker |  | 3226 | 44 | pub item at sps-core/src/install/bottle/mod.rs:18 |  |  | 0.737 |
| walker |  | 3276 | 50 | pub item at sps-core/src/install/bottle/mod.rs:165 |  |  | 0.737 |
| walker |  | 3301 | 25 | pub item body at sps-core/src/install/bottle/mod.rs:159 body 160 |  |  | 0.737 |
| walker |  | 3314 | 13 | pub-item doc lede at sps-core/src/install/bottle/mod.rs:18 |  |  | 0.737 |
| walker |  | 3334 | 20 | pub-item doc lede at sps-core/src/install/bottle/mod.rs:34 |  |  | 0.737 |
| walker |  | 3426 | 92 | pub item body at sps-core/src/install/bottle/mod.rs:18 body 23 |  |  | 0.737 |
| ns | 3447 |  | 557 | Cask artifacts mod.rs — re-export wall | 2.10 | 2.9 | 0.678 |
| ns | 3463 |  | 16 | sps-core/install/bottle sub-listing | 2.11 | 2.7 | 0.680 |
| ns | 3508 |  | 45 | sps-core/build — compile sub-listing | 2.12 | 2.3 | 0.668 |
| walker |  | 3543 | 117 | pub item body at sps-core/src/install/bottle/mod.rs:34 body 35 |  |  | 0.668 |
| ns | 3580 |  | 72 | sps-core — pipeline / check / uninstall / upgrade / utils sub-listings | 2.13 | 2.3 | 0.679 |
| walker |  | 3597 | 54 | pub-item names surface in sps/src/pipeline/planner.rs |  |  | 0.679 |
| ns | 3621 |  | 41 | sps-common — sub-tree listings (model / dependency) | 2.14 | 2.2 | 0.685 |
| walker |  | 3668 | 71 | pub item at sps/src/cli/init.rs:16 |  |  | 0.685 |
| ns | 3673 |  | 52 | sps bin — pipeline & cli sub-listings | 2.15 | 1.5 | 0.692 |
| walker |  | 3827 | 159 | mod/use plumbing in sps-core/src/install/bottle/mod.rs |  |  | 0.692 |
| walker |  | 4050 | 223 | mod/use plumbing in sps/src/main.rs |  |  | 0.692 |
| walker |  | 4140 | 90 | README.md section #1 |  |  | 0.692 |
| walker |  | 4165 | 25 | pub item at sps-core/src/utils/applescript.rs:75 |  |  | 0.692 |
| ns | 4185 |  | 512 | Per-crate Cargo manifest signatures (name + deps highlights) | 2.16 |  | 0.657 |
| walker |  | 4230 | 65 | pub-item names surface in sps/src/cli/search.rs |  |  | 0.657 |
| walker |  | 4230 | 0 | pub item at sps/src/cli/search.rs:240 |  |  | 0.657 |
| walker |  | 4257 | 27 | pub item at sps/src/cli/search.rs:23 |  |  | 0.657 |
| ns | 4301 |  | 116 | Config struct fields | 3.1 |  | 0.650 |
| walker |  | 4309 | 52 | pub item at sps/src/cli/search.rs:42 |  |  | 0.650 |
| walker |  | 4385 | 76 | pub item at sps/src/cli/search.rs:15 |  |  | 0.650 |
| walker |  | 4451 | 66 | pub-item names surface in sps/src/pipeline/runner.rs |  |  | 0.650 |
| walker |  | 4451 | 0 | pub item at sps/src/pipeline/runner.rs:56 |  |  | 0.650 |
| walker |  | 4497 | 46 | pub item at sps/src/pipeline/runner.rs:31 |  |  | 0.650 |
| walker |  | 4545 | 48 | pub item at sps/src/pipeline/runner.rs:38 |  |  | 0.650 |
| walker |  | 4610 | 65 | pub item at sps/src/pipeline/runner.rs:67 |  |  | 0.651 |
| ns | 4636 |  | 335 | Config path-method roster | 3.2 | 3.1 | 0.631 |
| ns | 5152 |  | 516 | Config — sps_root resolution + cellar/cask path bodies | 3.3 | 3.2 | 0.605 |
| walker |  | 5196 | 586 | entry item body at sps/src/main.rs:57 body 91 |  |  | 0.605 |
| walker |  | 5249 | 53 | pub item at sps-common/src/cache.rs:15 |  |  | 0.605 |
| walker |  | 5469 | 220 | pub-item names surface in sps-core/src/install/cask/mod.rs |  |  | 0.605 |
| walker |  | 5469 | 0 | pub item at sps-core/src/install/cask/mod.rs:37 |  |  | 0.605 |
| walker |  | 5469 | 0 | pub item at sps-core/src/install/cask/mod.rs:43 |  |  | 0.605 |
| walker |  | 5469 | 0 | pub item at sps-core/src/install/cask/mod.rs:49 |  |  | 0.605 |
| walker |  | 5469 | 0 | pub item at sps-core/src/install/cask/mod.rs:73 |  |  | 0.605 |
| walker |  | 5469 | 0 | pub item at sps-core/src/install/cask/mod.rs:686 |  |  | 0.605 |
| walker |  | 5485 | 16 | pub item body at sps-core/src/install/cask/mod.rs:43 body 44 |  |  | 0.605 |
| walker |  | 5504 | 19 | pub item body at sps-core/src/install/cask/mod.rs:73 body 74 |  |  | 0.605 |
| walker |  | 5549 | 45 | pub item at sps-core/src/install/cask/mod.rs:580 |  |  | 0.605 |
| ns | 5564 |  | 412 | SpsError variant signatures | 3.4 |  | 0.585 |
| walker |  | 5595 | 46 | pub item at sps-core/src/install/cask/mod.rs:77 |  |  | 0.585 |
| walker |  | 5642 | 47 | pub item at sps-core/src/install/cask/mod.rs:612 |  |  | 0.585 |
| walker |  | 5696 | 54 | pub item at sps-core/src/install/cask/mod.rs:232 |  |  | 0.585 |
| walker |  | 5738 | 42 | pub item body at sps-core/src/install/cask/mod.rs:37 body 38 |  |  | 0.585 |
| walker |  | 5872 | 134 | pub item at sps-core/src/install/cask/mod.rs:25 |  |  | 0.585 |
| walker |  | 5890 | 18 | pub-item doc lede at sps-core/src/install/cask/mod.rs:37 |  |  | 0.585 |
| ns | 5898 |  | 334 | InstallTargetIdentifier + Formula struct fields | 3.5 | 2.5 | 0.566 |
| walker |  | 5910 | 20 | pub-item doc lede at sps-core/src/install/cask/mod.rs:43 |  |  | 0.566 |
| walker |  | 5954 | 44 | pub-item doc lede at sps-core/src/install/cask/mod.rs:49 |  |  | 0.566 |
| walker |  | 6005 | 51 | pub-item doc lede at sps-core/src/install/cask/mod.rs:686 |  |  | 0.566 |
| walker |  | 6014 | 9 | pub item body at sps-core/src/install/cask/mod.rs:49 body 70 |  |  | 0.566 |
| walker |  | 6108 | 94 | pub item at sps/src/cli/info.rs:14 |  |  | 0.566 |
| walker |  | 6169 | 61 | pub item at sps-common/src/formulary.rs:12 |  |  | 0.566 |
| walker |  | 6273 | 104 | pub item at sps/src/cli/upgrade.rs:12 |  |  | 0.566 |
| ns | 6295 |  | 397 | Cask struct fields | 3.6 |  | 0.545 |
| walker |  | 6353 | 80 | README.md section #3 |  |  | 0.545 |
| walker |  | 6392 | 39 | pub item at sps-common/src/model/version.rs:11 |  |  | 0.545 |
| walker |  | 6404 | 12 | pub-item doc lede at sps-common/src/cache.rs:15 |  |  | 0.545 |
| ns | 6433 |  | 138 | InstalledArtifact variants | 3.7 |  | 0.540 |
| walker |  | 6517 | 113 | pub item at sps/src/cli/reinstall.rs:12 |  |  | 0.540 |
| ns | 6715 |  | 282 | Dependency + DependencyTag bitflags | 3.8 |  | 0.530 |
| walker |  | 6778 | 261 | mod/use plumbing in sps-core/src/install/cask/mod.rs |  |  | 0.530 |
| walker |  | 6882 | 104 | README.md section #2 |  |  | 0.530 |
| ns | 6913 |  | 198 | Requirement enum (macOS / Xcode / Other) | 3.9 |  | 0.523 |
| walker |  | 7016 | 134 | pub item at sps/src/cli/list.rs:16 |  |  | 0.523 |
| walker |  | 7080 | 64 | pub item at sps-common/src/dependency/requirement.rs:7 |  |  | 0.525 |
| walker |  | 7106 | 26 | pub-item names surface in sps-common/src/keg.rs |  |  | 0.525 |
| ns | 7119 |  | 206 | Cache API surface | 3.10 |  | 0.520 |
| walker |  | 7128 | 22 | pub item at sps-common/src/keg.rs:21 |  |  | 0.520 |
| walker |  | 7180 | 52 | pub item at sps-common/src/keg.rs:13 |  |  | 0.520 |
| walker |  | 7193 | 13 | pub-item doc lede at sps-common/src/keg.rs:21 |  |  | 0.520 |
| walker |  | 7208 | 15 | pub-item doc lede at sps-common/src/keg.rs:13 |  |  | 0.520 |
| walker |  | 7236 | 28 | mod/use plumbing in sps-common/src/error.rs |  |  | 0.520 |
| walker |  | 7265 | 29 | pub-item names surface in sps-common/src/config.rs |  |  | 0.520 |
| walker |  | 7265 | 0 | pub item at sps-common/src/config.rs:199 |  |  | 0.520 |
| walker |  | 7274 | 9 | pub item body at sps-common/src/config.rs:199 body 200 |  |  | 0.520 |
| ns | 7312 |  | 193 | InstalledKeg + KegRegistry signatures | 3.11 |  | 0.519 |
| walker |  | 7355 | 81 | README.md section #9 |  |  | 0.519 |
| walker |  | 7459 | 104 | pub item at sps-common/src/config.rs:15 |  |  | 0.529 |
| walker |  | 7482 | 23 | pub item body at sps-core/src/install/cask/mod.rs:49 body 50 |  |  | 0.529 |
| ns | 7654 |  | 342 | Pipeline shared types — JobAction / PlannedJob / WorkerJob / PipelineEvent header | 4.1 |  | 0.515 |
| walker |  | 7667 | 185 | pub item at sps/src/cli/uninstall.rs:17 |  |  | 0.515 |
| walker |  | 7700 | 33 | pub-item names surface in sps-common/src/error.rs |  |  | 0.516 |
| walker |  | 7727 | 27 | pub item body at sps-core/src/install/cask/mod.rs:686 body 687 |  |  | 0.516 |
| walker |  | 7836 | 109 | README.md section #6 |  |  | 0.516 |
| walker |  | 7859 | 23 | pub-item names surface in sps-common/src/dependency/definition.rs |  |  | 0.516 |
| walker |  | 7922 | 63 | pub item at sps-common/src/dependency/definition.rs:53 |  |  | 0.518 |
| walker |  | 7978 | 56 | pub item at sps-common/src/dependency/definition.rs:31 |  |  | 0.522 |
| walker |  | 7993 | 15 | pub-item doc lede at sps-common/src/model/version.rs:11 |  |  | 0.522 |
| walker |  | 8105 | 112 | README.md section #7 |  |  | 0.522 |
| ns | 8151 |  | 497 | JobProcessingState + DownloadOutcome + PlannedOperations | 4.2 | 4.1 | 0.508 |
| walker |  | 8182 | 77 | pub item at sps-core/src/upgrade/cask.rs:15 |  |  | 0.508 |
| walker |  | 8260 | 78 | pub item at sps-core/src/uninstall/formula.rs:10 |  |  | 0.508 |
| walker |  | 8285 | 25 | mod/use plumbing in sps-common/src/dependency/requirement.rs |  |  | 0.508 |
| walker |  | 8311 | 26 | pub-item names surface in sps-core/src/check/update.rs |  |  | 0.508 |
| walker |  | 8357 | 46 | pub item at sps-core/src/check/update.rs:89 |  |  | 0.508 |
| ns | 8373 |  | 222 | CommandType + PipelineFlags + run_pipeline signature | 4.3 |  | 0.515 |
| walker |  | 8429 | 72 | pub item at sps-core/src/check/update.rs:23 |  |  | 0.515 |
| walker |  | 8504 | 75 | mod/use plumbing in sps/src/cli/update.rs |  |  | 0.515 |
| walker |  | 8538 | 34 | impl method sigs in sps/src/cli.rs |  |  | 0.515 |
| ns | 8687 |  | 314 | OperationPlanner + plan_operations signature | 4.4 |  | 0.504 |
| walker |  | 8776 | 238 | [dependencies] in sps-common/Cargo.toml |  |  | 0.504 |
| walker |  | 8886 | 110 | [package] in sps-common/Cargo.toml |  |  | 0.506 |
| ns | 9007 |  | 320 | DependencyResolver — strategy + status + ResolvedDependency | 4.5 |  | 0.495 |
| walker |  | 9124 | 238 | [dependencies] in sps-net/Cargo.toml |  |  | 0.495 |
| walker |  | 9234 | 110 | [package] in sps-net/Cargo.toml |  |  | 0.501 |
| ns | 9331 |  | 324 | DependencyResolver — ResolutionContext + ResolvedGraph | 4.6 | 4.5 | 0.492 |
| walker |  | 9348 | 114 | [package] in sps-core/Cargo.toml |  |  | 0.501 |
| walker |  | 9443 | 95 | pub item at sps-core/src/upgrade/source.rs:19 |  |  | 0.501 |
| walker |  | 9461 | 18 | pub-item doc lede at sps-core/src/upgrade/cask.rs:15 |  |  | 0.501 |
| ns | 9518 |  | 187 | core worker entry — execute_sync_job | 4.7 |  | 0.496 |
| walker |  | 9594 | 133 | README.md section #8 |  |  | 0.496 |
| ns | 9675 |  | 157 | core worker pool — start_worker_pool_manager | 4.8 | 4.7 | 0.493 |
| walker |  | 9682 | 88 | mod/use plumbing in sps/src/cli/reinstall.rs |  |  | 0.493 |
| walker |  | 9714 | 32 | mod/use plumbing in sps-common/src/model/artifact.rs |  |  | 0.493 |
| ns | 9911 |  | 236 | Install entry-points — install_bottle / build_from_source / install_cask signatures | 4.9 |  | 0.489 |
| ns | 10046 |  | 135 | Bottle platform selection — get_bottle_for_platform | 4.10 | 4.9 | 0.487 |
| ns | 10196 |  | 150 | API entry-points (sps-net::api) | 4.11 |  | 0.484 |
| ns | 10289 |  | 93 | Uninstall + Upgrade entry-point fn names | 4.12 |  | 0.484 |

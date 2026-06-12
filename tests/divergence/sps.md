Score(3000)=0.717 I=0.851 C=0.605 ns_rows≤3K=19/49 (reached=9 partial=5 missing=5)

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
| walker |  | 1393 | 42 | mod/use plumbing in sps-core/src/check/mod.rs |  |  | 0.635 |
| ns | 1476 |  | 99 | Per-crate src/ listings | 2.1 |  | 0.616 |
| walker |  | 1506 | 113 | headings outline in README.md |  |  | 0.616 |
| walker |  | 1566 | 60 | README.md section #0 |  |  | 0.646 |
| walker |  | 1601 | 35 | listing of 'sps-common/src' |  |  | 0.711 |
| walker |  | 1617 | 16 | listing of 'sps-common/src/dependency' |  |  | 0.711 |
| walker |  | 1642 | 25 | listing of 'sps-common/src/model' |  |  | 0.713 |
| ns | 1651 |  | 175 | sps-common lib.rs — module tree + re-exports | 2.2 |  | 0.671 |
| walker |  | 1692 | 50 | pub item at sps-common/src/model/mod.rs:17 |  |  | 0.671 |
| walker |  | 1810 | 118 | mod/use plumbing in sps-common/src/lib.rs |  |  | 0.698 |
| walker |  | 1838 | 28 | pub item at sps/src/cli/status.rs:399 |  |  | 0.698 |
| ns | 1859 |  | 208 | sps-core lib.rs — module tree | 2.3 |  | 0.662 |
| walker |  | 1909 | 71 | mod/use plumbing in sps-core/src/upgrade/mod.rs |  |  | 0.662 |
| walker |  | 1982 | 73 | mod/use plumbing in sps-core/src/uninstall/mod.rs |  |  | 0.662 |
| walker |  | 2056 | 74 | mod/use plumbing in sps-core/src/install/mod.rs |  |  | 0.663 |
| ns | 2220 |  | 361 | sps-net lib.rs — re-exports + dependency on sps-common | 2.4 |  | 0.612 |
| walker |  | 2323 | 267 | entry item body at sps/src/main.rs:57 body 167 |  |  | 0.678 |
| ns | 2402 |  | 182 | sps-common — model module exports | 2.5 | 2.2 | 0.649 |
| walker |  | 2408 | 85 | mod/use plumbing in sps-common/src/model/mod.rs |  |  | 0.674 |
| ns | 2517 |  | 115 | sps-common — dependency module exports | 2.6 | 2.2 | 0.659 |
| walker |  | 2683 | 275 | mod/use plumbing in sps-net/src/lib.rs |  |  | 0.704 |
| ns | 2744 |  | 227 | sps-core/install — module roster | 2.7 | 2.3 | 0.683 |
| walker |  | 2749 | 66 | pub item at sps/src/cli.rs:35 |  |  | 0.683 |
| ns | 2759 |  | 15 | sps-core/install/cask sub-listing | 2.8 | 2.7 | 0.677 |
| walker |  | 2846 | 97 | mod/use plumbing in sps-common/src/dependency/mod.rs |  |  | 0.694 |
| ns | 2874 |  | 115 | Cask artifact handler enumeration | 2.9 | 2.8 | 0.662 |
| walker |  | 2944 | 98 | pub item at sps/src/cli.rs:44 |  |  | 0.709 |
| walker |  | 2968 | 24 | mod/use plumbing in sps/src/pipeline.rs |  |  | 0.709 |
| walker |  | 2983 | 15 | listing of 'sps-core/src/install/cask' |  |  | 0.717 |
| walker |  | 3060 | 77 | README.md section #1 |  |  | 0.717 |
| walker |  | 3112 | 52 | pub-item names surface in sps/src/pipeline/planner.rs |  |  | 0.717 |
| walker |  | 3128 | 16 | listing of 'sps-core/src/install/bottle' |  |  | 0.718 |
| walker |  | 3203 | 75 | pub-item names surface in sps-core/src/install/bottle/mod.rs |  |  | 0.718 |
| walker |  | 3203 | 0 | pub item at sps-core/src/install/bottle/mod.rs:34 |  |  | 0.718 |
| walker |  | 3203 | 0 | pub item at sps-core/src/install/bottle/mod.rs:159 |  |  | 0.718 |
| walker |  | 3247 | 44 | pub item at sps-core/src/install/bottle/mod.rs:18 |  |  | 0.718 |
| walker |  | 3297 | 50 | pub item at sps-core/src/install/bottle/mod.rs:165 |  |  | 0.718 |
| walker |  | 3320 | 23 | pub item body at sps-core/src/install/bottle/mod.rs:159 body 160 |  |  | 0.718 |
| walker |  | 3333 | 13 | pub-item doc lede at sps-core/src/install/bottle/mod.rs:18 |  |  | 0.718 |
| walker |  | 3353 | 20 | pub-item doc lede at sps-core/src/install/bottle/mod.rs:34 |  |  | 0.718 |
| ns | 3431 |  | 557 | Cask artifacts mod.rs — re-export wall | 2.10 | 2.9 | 0.661 |
| walker |  | 3443 | 90 | pub item body at sps-core/src/install/bottle/mod.rs:18 body 23 |  |  | 0.661 |
| ns | 3447 |  | 16 | sps-core/install/bottle sub-listing | 2.11 | 2.7 | 0.663 |
| ns | 3492 |  | 45 | sps-core/build — compile sub-listing | 2.12 | 2.3 | 0.652 |
| walker |  | 3558 | 115 | pub item body at sps-core/src/install/bottle/mod.rs:34 body 35 |  |  | 0.652 |
| ns | 3564 |  | 72 | sps-core — pipeline / check / uninstall / upgrade / utils sub-listings | 2.13 | 2.3 | 0.663 |
| ns | 3605 |  | 41 | sps-common — sub-tree listings (model / dependency) | 2.14 | 2.2 | 0.669 |
| walker |  | 3625 | 67 | pub item at sps/src/cli/init.rs:16 |  |  | 0.669 |
| ns | 3657 |  | 52 | sps bin — pipeline & cli sub-listings | 2.15 | 1.5 | 0.676 |
| walker |  | 3834 | 209 | mod/use plumbing in sps/src/main.rs |  |  | 0.676 |
| walker |  | 3986 | 152 | mod/use plumbing in sps-core/src/install/bottle/mod.rs |  |  | 0.676 |
| walker |  | 4009 | 23 | pub item at sps-core/src/utils/applescript.rs:75 |  |  | 0.676 |
| walker |  | 4072 | 63 | pub-item names surface in sps/src/cli/search.rs |  |  | 0.676 |
| walker |  | 4072 | 0 | pub item at sps/src/cli/search.rs:240 |  |  | 0.676 |
| walker |  | 4097 | 25 | pub item at sps/src/cli/search.rs:23 |  |  | 0.676 |
| walker |  | 4149 | 52 | pub item at sps/src/cli/search.rs:42 |  |  | 0.676 |
| ns | 4161 |  | 504 | Per-crate Cargo manifest signatures (name + deps highlights) | 2.16 |  | 0.642 |
| walker |  | 4225 | 76 | pub item at sps/src/cli/search.rs:15 |  |  | 0.642 |
| ns | 4273 |  | 112 | Config struct fields | 3.1 |  | 0.635 |
| walker |  | 4289 | 64 | pub-item names surface in sps/src/pipeline/runner.rs |  |  | 0.635 |
| walker |  | 4289 | 0 | pub item at sps/src/pipeline/runner.rs:56 |  |  | 0.635 |
| walker |  | 4333 | 44 | pub item at sps/src/pipeline/runner.rs:31 |  |  | 0.635 |
| walker |  | 4381 | 48 | pub item at sps/src/pipeline/runner.rs:38 |  |  | 0.635 |
| walker |  | 4446 | 65 | pub item at sps/src/pipeline/runner.rs:67 |  |  | 0.636 |
| ns | 4558 |  | 285 | Config path-method roster | 3.2 | 3.1 | 0.617 |
| walker |  | 5014 | 568 | entry item body at sps/src/main.rs:57 body 91 |  |  | 0.617 |
| walker |  | 5063 | 49 | pub item at sps-common/src/cache.rs:15 |  |  | 0.617 |
| ns | 5074 |  | 516 | Config — sps_root resolution + cellar/cask path bodies | 3.3 | 3.2 | 0.592 |
| walker |  | 5148 | 85 | pub item at sps/src/cli/info.rs:14 |  |  | 0.592 |
| walker |  | 5238 | 90 | pub item at sps/src/cli/upgrade.rs:12 |  |  | 0.592 |
| ns | 5428 |  | 354 | SpsError variant signatures | 3.4 |  | 0.572 |
| walker |  | 5456 | 218 | pub-item names surface in sps-core/src/install/cask/mod.rs |  |  | 0.572 |
| walker |  | 5456 | 0 | pub item at sps-core/src/install/cask/mod.rs:37 |  |  | 0.572 |
| walker |  | 5456 | 0 | pub item at sps-core/src/install/cask/mod.rs:43 |  |  | 0.572 |
| walker |  | 5456 | 0 | pub item at sps-core/src/install/cask/mod.rs:49 |  |  | 0.572 |
| walker |  | 5456 | 0 | pub item at sps-core/src/install/cask/mod.rs:73 |  |  | 0.572 |
| walker |  | 5456 | 0 | pub item at sps-core/src/install/cask/mod.rs:686 |  |  | 0.572 |
| walker |  | 5470 | 14 | pub item body at sps-core/src/install/cask/mod.rs:43 body 44 |  |  | 0.572 |
| walker |  | 5487 | 17 | pub item body at sps-core/src/install/cask/mod.rs:73 body 74 |  |  | 0.572 |
| walker |  | 5532 | 45 | pub item at sps-core/src/install/cask/mod.rs:580 |  |  | 0.572 |
| walker |  | 5578 | 46 | pub item at sps-core/src/install/cask/mod.rs:77 |  |  | 0.572 |
| walker |  | 5625 | 47 | pub item at sps-core/src/install/cask/mod.rs:612 |  |  | 0.572 |
| walker |  | 5679 | 54 | pub item at sps-core/src/install/cask/mod.rs:232 |  |  | 0.572 |
| walker |  | 5719 | 40 | pub item body at sps-core/src/install/cask/mod.rs:37 body 38 |  |  | 0.572 |
| ns | 5758 |  | 330 | InstallTargetIdentifier + Formula struct fields | 3.5 | 2.5 | 0.553 |
| walker |  | 5851 | 132 | pub item at sps-core/src/install/cask/mod.rs:25 |  |  | 0.553 |
| walker |  | 5871 | 20 | pub-item doc lede at sps-core/src/install/cask/mod.rs:37 |  |  | 0.553 |
| walker |  | 5891 | 20 | pub-item doc lede at sps-core/src/install/cask/mod.rs:43 |  |  | 0.553 |
| walker |  | 5935 | 44 | pub-item doc lede at sps-core/src/install/cask/mod.rs:49 |  |  | 0.553 |
| walker |  | 5986 | 51 | pub-item doc lede at sps-core/src/install/cask/mod.rs:686 |  |  | 0.553 |
| walker |  | 5993 | 7 | pub item body at sps-core/src/install/cask/mod.rs:49 body 70 |  |  | 0.553 |
| walker |  | 6065 | 72 | README.md section #3 |  |  | 0.553 |
| walker |  | 6122 | 57 | pub item at sps-common/src/formulary.rs:12 |  |  | 0.553 |
| ns | 6151 |  | 393 | Cask struct fields | 3.6 |  | 0.533 |
| walker |  | 6157 | 35 | pub item at sps-common/src/model/version.rs:11 |  |  | 0.533 |
| walker |  | 6261 | 104 | pub item at sps/src/cli/reinstall.rs:12 |  |  | 0.533 |
| ns | 6269 |  | 118 | InstalledArtifact variants | 3.7 |  | 0.528 |
| walker |  | 6357 | 96 | README.md section #2 |  |  | 0.528 |
| walker |  | 6369 | 12 | pub-item doc lede at sps-common/src/cache.rs:15 |  |  | 0.528 |
| ns | 6543 |  | 274 | Dependency + DependencyTag bitflags | 3.8 |  | 0.518 |
| walker |  | 6617 | 248 | mod/use plumbing in sps-core/src/install/cask/mod.rs |  |  | 0.518 |
| walker |  | 6638 | 21 | mod/use plumbing in sps-common/src/error.rs |  |  | 0.518 |
| ns | 6739 |  | 196 | Requirement enum (macOS / Xcode / Other) | 3.9 |  | 0.511 |
| walker |  | 6768 | 130 | pub item at sps/src/cli/list.rs:16 |  |  | 0.511 |
| walker |  | 6828 | 60 | pub item at sps-common/src/dependency/requirement.rs:7 |  |  | 0.514 |
| walker |  | 6896 | 68 | README.md section #9 |  |  | 0.514 |
| walker |  | 6920 | 24 | pub-item names surface in sps-common/src/keg.rs |  |  | 0.514 |
| ns | 6925 |  | 186 | Cache API surface | 3.10 |  | 0.508 |
| walker |  | 6940 | 20 | pub item at sps-common/src/keg.rs:21 |  |  | 0.509 |
| walker |  | 6990 | 50 | pub item at sps-common/src/keg.rs:13 |  |  | 0.509 |
| walker |  | 7005 | 15 | pub-item doc lede at sps-common/src/keg.rs:13 |  |  | 0.509 |
| walker |  | 7020 | 15 | pub-item doc lede at sps-common/src/keg.rs:21 |  |  | 0.509 |
| ns | 7098 |  | 173 | InstalledKeg + KegRegistry signatures | 3.11 |  | 0.508 |
| walker |  | 7106 | 86 | README.md section #6 |  |  | 0.508 |
| walker |  | 7133 | 27 | pub-item names surface in sps-common/src/config.rs |  |  | 0.508 |
| walker |  | 7133 | 0 | pub item at sps-common/src/config.rs:199 |  |  | 0.508 |
| walker |  | 7140 | 7 | pub item body at sps-common/src/config.rs:199 body 200 |  |  | 0.508 |
| walker |  | 7161 | 21 | pub item body at sps-core/src/install/cask/mod.rs:49 body 50 |  |  | 0.508 |
| walker |  | 7263 | 102 | pub item at sps-common/src/config.rs:15 |  |  | 0.518 |
| walker |  | 7352 | 89 | README.md section #7 |  |  | 0.518 |
| ns | 7436 |  | 338 | Pipeline shared types — JobAction / PlannedJob / WorkerJob / PipelineEvent header | 4.1 |  | 0.504 |
| walker |  | 7533 | 181 | pub item at sps/src/cli/uninstall.rs:17 |  |  | 0.504 |
| walker |  | 7558 | 25 | pub item body at sps-core/src/install/cask/mod.rs:686 body 687 |  |  | 0.504 |
| walker |  | 7591 | 33 | pub-item names surface in sps-common/src/error.rs |  |  | 0.504 |
| walker |  | 7611 | 20 | mod/use plumbing in sps-common/src/dependency/requirement.rs |  |  | 0.504 |
| walker |  | 7632 | 21 | pub-item names surface in sps-common/src/dependency/definition.rs |  |  | 0.505 |
| walker |  | 7693 | 61 | pub item at sps-common/src/dependency/definition.rs:53 |  |  | 0.507 |
| walker |  | 7747 | 54 | pub item at sps-common/src/dependency/definition.rs:31 |  |  | 0.511 |
| walker |  | 7770 | 23 | mod/use plumbing in sps-common/src/model/artifact.rs |  |  | 0.511 |
| walker |  | 7785 | 15 | pub-item doc lede at sps-common/src/model/version.rs:11 |  |  | 0.511 |
| walker |  | 7809 | 24 | pub-item names surface in sps-core/src/check/update.rs |  |  | 0.511 |
| walker |  | 7855 | 46 | pub item at sps-core/src/check/update.rs:89 |  |  | 0.511 |
| walker |  | 7925 | 70 | pub item at sps-core/src/check/update.rs:23 |  |  | 0.511 |
| ns | 7933 |  | 497 | JobProcessingState + DownloadOutcome + PlannedOperations | 4.2 | 4.1 | 0.497 |
| walker |  | 8000 | 75 | pub item at sps-core/src/upgrade/cask.rs:15 |  |  | 0.497 |
| walker |  | 8076 | 76 | pub item at sps-core/src/uninstall/formula.rs:10 |  |  | 0.497 |
| walker |  | 8146 | 70 | mod/use plumbing in sps/src/cli/update.rs |  |  | 0.497 |
| ns | 8149 |  | 216 | CommandType + PipelineFlags + run_pipeline signature | 4.3 |  | 0.505 |
| walker |  | 8261 | 115 | README.md section #8 |  |  | 0.505 |
| ns | 8457 |  | 308 | OperationPlanner + plan_operations signature | 4.4 |  | 0.494 |
| walker |  | 8487 | 226 | [dependencies] in sps-net/Cargo.toml |  |  | 0.494 |
| walker |  | 8565 | 78 | mod/use plumbing in sps/src/cli/reinstall.rs |  |  | 0.494 |
| ns | 8771 |  | 314 | DependencyResolver — strategy + status + ResolvedDependency | 4.5 |  | 0.483 |
| walker |  | 8801 | 236 | [dependencies] in sps-common/Cargo.toml |  |  | 0.483 |
| walker |  | 8913 | 112 | [package] in sps-common/Cargo.toml |  |  | 0.485 |
| walker |  | 9025 | 112 | [package] in sps-core/Cargo.toml |  |  | 0.490 |
| ns | 9097 |  | 326 | DependencyResolver — ResolutionContext + ResolvedGraph | 4.6 | 4.5 | 0.482 |
| walker |  | 9137 | 112 | [package] in sps-net/Cargo.toml |  |  | 0.491 |
| walker |  | 9173 | 36 | impl method sigs in sps/src/cli.rs |  |  | 0.491 |
| walker |  | 9266 | 93 | pub item at sps-core/src/upgrade/source.rs:19 |  |  | 0.491 |
| ns | 9280 |  | 183 | core worker entry — execute_sync_job | 4.7 |  | 0.486 |
| walker |  | 9284 | 18 | pub-item doc lede at sps-core/src/upgrade/cask.rs:15 |  |  | 0.486 |
| walker |  | 9315 | 31 | mod/use plumbing in sps-common/src/dependency/definition.rs |  |  | 0.486 |
| walker |  | 9402 | 87 | mod/use plumbing in sps/src/cli/install.rs |  |  | 0.486 |
| ns | 9433 |  | 153 | core worker pool — start_worker_pool_manager | 4.8 | 4.7 | 0.483 |
| walker |  | 9513 | 111 | pub item at sps-common/src/model/tap.rs:10 |  |  | 0.483 |
| walker |  | 9530 | 17 | pub-item doc lede at sps-common/src/model/tap.rs:10 |  |  | 0.483 |
| ns | 9653 |  | 220 | Install entry-points — install_bottle / build_from_source / install_cask signatures | 4.9 |  | 0.479 |
| ns | 9784 |  | 131 | Bottle platform selection — get_bottle_for_platform | 4.10 | 4.9 | 0.478 |
| ns | 9912 |  | 128 | API entry-points (sps-net::api) | 4.11 |  | 0.475 |
| ns | 9983 |  | 71 | Uninstall + Upgrade entry-point fn names | 4.12 |  | 0.474 |

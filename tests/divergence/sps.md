Score(3000)=0.457 I=0.680 C=0.306 ns_rows≤3K=19/49 (reached=5 partial=1 missing=13)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 47 | 47 | listing of '.' |  |  | 1.000 |
| ns | 47 |  | 47 | Top-level repo listing | 1.1 |  | 1.000 |
| walker |  | 55 | 8 | listing of 'sps' |  |  | 1.000 |
| walker |  | 73 | 18 | listing of 'sps/src' |  |  | 1.000 |
| walker |  | 85 | 12 | listing of 'sps/src/pipeline' |  |  | 1.000 |
| walker |  | 100 | 15 | entry item at sps/src/main.rs:57 |  |  | 1.000 |
| walker |  | 108 | 8 | listing of 'sps-common' |  |  | 1.000 |
| walker |  | 116 | 8 | listing of 'sps-core' |  |  | 1.000 |
| walker |  | 124 | 8 | listing of 'sps-net' |  |  | 1.000 |
| ns | 144 |  | 97 | README lede — what sps is | 1.2 |  | 0.759 |
| walker |  | 199 | 75 | [package] in Cargo.toml |  |  | 0.797 |
| ns | 219 |  | 75 | Workspace members | 1.3 |  | 0.820 |
| walker |  | 239 | 40 | listing of 'sps/src/cli' |  |  | 0.823 |
| walker |  | 249 | 10 | pub-item names surface in sps/src/cli/info.rs |  |  | 0.823 |
| walker |  | 259 | 10 | pub-item names surface in sps/src/cli/list.rs |  |  | 0.823 |
| walker |  | 269 | 10 | pub-item names surface in sps/src/cli/update.rs |  |  | 0.823 |
| walker |  | 280 | 11 | pub-item names surface in sps/src/cli/init.rs |  |  | 0.823 |
| walker |  | 291 | 11 | pub-item names surface in sps/src/cli/install.rs |  |  | 0.823 |
| walker |  | 302 | 11 | pub-item names surface in sps/src/cli/uninstall.rs |  |  | 0.823 |
| walker |  | 313 | 11 | pub-item names surface in sps/src/cli/upgrade.rs |  |  | 0.823 |
| walker |  | 325 | 12 | pub-item names surface in sps/src/cli/reinstall.rs |  |  | 0.823 |
| ns | 327 |  | 108 | CLI subcommand enum | 1.4 |  | 0.660 |
| walker |  | 346 | 21 | pub-item names surface in sps/src/cli.rs |  |  | 0.662 |
| walker |  | 359 | 13 | pub-item names surface in sps/src/pipeline/downloader.rs |  |  | 0.662 |
| walker |  | 380 | 21 | listing of 'sps-net/src' |  |  | 0.663 |
| walker |  | 405 | 25 | listing of 'sps-core/src' |  |  | 0.668 |
| walker |  | 417 | 12 | listing of 'sps-core/src/check' |  |  | 0.669 |
| walker |  | 429 | 12 | listing of 'sps-core/src/pipeline' |  |  | 0.669 |
| walker |  | 443 | 14 | listing of 'sps-core/src/utils' |  |  | 0.670 |
| walker |  | 460 | 17 | listing of 'sps-core/src/uninstall' |  |  | 0.671 |
| walker |  | 477 | 17 | listing of 'sps-core/src/upgrade' |  |  | 0.673 |
| walker |  | 493 | 16 | mod/use plumbing in sps-core/src/pipeline/mod.rs |  |  | 0.673 |
| ns | 503 |  | 176 | CLI per-subcommand module list | 1.5 |  | 0.531 |
| walker |  | 513 | 20 | listing of 'sps-core/src/install' |  |  | 0.531 |
| walker |  | 540 | 27 | pub-item names surface in sps-core/src/install/mod.rs |  |  | 0.531 |
| walker |  | 540 | 0 | pub item at sps-core/src/install/mod.rs:17 |  |  | 0.531 |
| walker |  | 563 | 23 | pub item body at sps-core/src/install/mod.rs:17 body 18 |  |  | 0.531 |
| walker |  | 581 | 18 | mod/use plumbing in sps-core/src/utils/mod.rs |  |  | 0.531 |
| walker |  | 594 | 13 | pub-item names surface in sps-core/src/pipeline/engine.rs |  |  | 0.531 |
| walker |  | 607 | 13 | pub-item names surface in sps-core/src/uninstall/formula.rs |  |  | 0.531 |
| walker |  | 620 | 13 | pub-item names surface in sps-core/src/upgrade/source.rs |  |  | 0.531 |
| walker |  | 634 | 14 | pub-item names surface in sps-core/src/pipeline/worker.rs |  |  | 0.531 |
| walker |  | 648 | 14 | pub-item names surface in sps-core/src/upgrade/bottle.rs |  |  | 0.531 |
| walker |  | 662 | 14 | pub-item names surface in sps-core/src/upgrade/cask.rs |  |  | 0.531 |
| ns | 678 |  | 175 | main.rs — Tokio entry signature + Init early-out | 1.6 |  | 0.460 |
| walker |  | 704 | 42 | mod/use plumbing in sps-core/src/check/mod.rs |  |  | 0.460 |
| ns | 744 |  | 66 | main.rs — Config::load failure hint | 1.7 | 1.6 | 0.443 |
| walker |  | 781 | 77 | mod/use plumbing in sps-core/src/lib.rs |  |  | 0.444 |
| walker |  | 804 | 23 | pub-item names surface in sps-core/src/utils/applescript.rs |  |  | 0.444 |
| walker |  | 804 | 0 | pub item at sps-core/src/utils/applescript.rs:75 |  |  | 0.444 |
| walker |  | 828 | 24 | pub-item names surface in sps-core/src/check/update.rs |  |  | 0.444 |
| walker |  | 863 | 35 | listing of 'sps-common/src' |  |  | 0.450 |
| walker |  | 879 | 16 | listing of 'sps-common/src/dependency' |  |  | 0.450 |
| walker |  | 889 | 10 | pub-item names surface in sps-common/src/cache.rs |  |  | 0.450 |
| walker |  | 901 | 12 | pub-item names surface in sps-common/src/formulary.rs |  |  | 0.450 |
| walker |  | 926 | 25 | listing of 'sps-common/src/model' |  |  | 0.451 |
| ns | 936 |  | 192 | main.rs — auto-update gate | 1.8 | 1.7 | 0.405 |
| walker |  | 938 | 12 | pub-item names surface in sps-common/src/model/mod.rs |  |  | 0.405 |
| walker |  | 966 | 28 | pub item at sps-common/src/model/mod.rs:17 |  |  | 0.405 |
| walker |  | 976 | 10 | pub-item names surface in sps-common/src/dependency/requirement.rs |  |  | 0.405 |
| walker |  | 986 | 10 | pub-item names surface in sps-common/src/model/tap.rs |  |  | 0.405 |
| walker |  | 997 | 11 | pub-item names surface in sps-common/src/model/artifact.rs |  |  | 0.405 |
| walker |  | 1021 | 24 | pub-item names surface in sps-common/src/keg.rs |  |  | 0.405 |
| walker |  | 1033 | 12 | pub item at sps-common/src/keg.rs:21 |  |  | 0.405 |
| walker |  | 1048 | 15 | pub-item names surface in sps-common/src/model/version.rs |  |  | 0.405 |
| walker |  | 1048 | 0 | pub item at sps-common/src/model/version.rs:11 |  |  | 0.405 |
| walker |  | 1075 | 27 | pub-item names surface in sps-common/src/config.rs |  |  | 0.405 |
| walker |  | 1075 | 0 | pub item at sps-common/src/config.rs:199 |  |  | 0.405 |
| walker |  | 1082 | 7 | pub item body at sps-common/src/config.rs:199 body 200 |  |  | 0.405 |
| ns | 1105 |  | 169 | main.rs — Cache + Command::run dispatch | 1.9 | 1.7 | 0.376 |
| walker |  | 1117 | 35 | pub item at sps-common/src/keg.rs:13 |  |  | 0.376 |
| walker |  | 1146 | 29 | pub item at sps-common/src/dependency/requirement.rs:7 |  |  | 0.376 |
| walker |  | 1184 | 38 | pub item at sps-common/src/formulary.rs:12 |  |  | 0.376 |
| walker |  | 1223 | 39 | pub item at sps-common/src/cache.rs:15 |  |  | 0.376 |
| walker |  | 1256 | 33 | pub-item names surface in sps-common/src/error.rs |  |  | 0.376 |
| walker |  | 1277 | 21 | pub-item names surface in sps-common/src/dependency/definition.rs |  |  | 0.376 |
| walker |  | 1348 | 71 | mod/use plumbing in sps-core/src/upgrade/mod.rs |  |  | 0.376 |
| ns | 1377 |  | 272 | main.rs — pipeline-aware error printing on failure | 1.10 | 1.9 | 0.339 |
| walker |  | 1381 | 33 | pub item at sps-common/src/dependency/definition.rs:31 |  |  | 0.339 |
| walker |  | 1454 | 73 | mod/use plumbing in sps-core/src/uninstall/mod.rs |  |  | 0.339 |
| ns | 1476 |  | 99 | Per-crate src/ listings | 2.1 |  | 0.457 |
| walker |  | 1528 | 74 | mod/use plumbing in sps-core/src/install/mod.rs |  |  | 0.458 |
| walker |  | 1584 | 56 | pub item at sps/src/cli.rs:35 |  |  | 0.458 |
| walker |  | 1612 | 28 | pub-item names surface in sps/src/cli/status.rs |  |  | 0.458 |
| walker |  | 1612 | 0 | pub item at sps/src/cli/status.rs:399 |  |  | 0.458 |
| ns | 1651 |  | 175 | sps-common lib.rs — module tree + re-exports | 2.2 |  | 0.431 |
| walker |  | 1730 | 118 | mod/use plumbing in sps-common/src/lib.rs |  |  | 0.463 |
| walker |  | 1815 | 85 | mod/use plumbing in sps-common/src/model/mod.rs |  |  | 0.465 |
| walker |  | 1827 | 12 | pub-item doc lede at sps-common/src/cache.rs:15 |  |  | 0.465 |
| ns | 1859 |  | 208 | sps-core lib.rs — module tree | 2.3 |  | 0.446 |
| walker |  | 1914 | 87 | pub item at sps/src/cli.rs:44 |  |  | 0.499 |
| walker |  | 1975 | 61 | pub item at sps-common/src/dependency/definition.rs:53 |  |  | 0.499 |
| walker |  | 2072 | 97 | mod/use plumbing in sps-common/src/dependency/mod.rs |  |  | 0.501 |
| walker |  | 2117 | 45 | pub item at sps/src/cli/init.rs:16 |  |  | 0.501 |
| walker |  | 2138 | 21 | mod/use plumbing in sps-common/src/error.rs |  |  | 0.501 |
| walker |  | 2153 | 15 | pub-item doc lede at sps-common/src/keg.rs:13 |  |  | 0.501 |
| walker |  | 2168 | 15 | pub-item doc lede at sps-common/src/keg.rs:21 |  |  | 0.501 |
| walker |  | 2192 | 24 | mod/use plumbing in sps/src/pipeline.rs |  |  | 0.501 |
| ns | 2220 |  | 361 | sps-net lib.rs — re-exports + dependency on sps-common | 2.4 |  | 0.462 |
| walker |  | 2238 | 46 | pub item at sps-core/src/check/update.rs:89 |  |  | 0.462 |
| walker |  | 2308 | 70 | pub-item names surface in sps-net/src/validation.rs |  |  | 0.462 |
| walker |  | 2308 | 0 | pub item at sps-net/src/validation.rs:67 |  |  | 0.462 |
| walker |  | 2308 | 0 | pub item at sps-net/src/validation.rs:93 |  |  | 0.462 |
| walker |  | 2308 | 0 | pub item at sps-net/src/validation.rs:121 |  |  | 0.462 |
| walker |  | 2352 | 44 | pub-item names surface in sps-core/src/uninstall/cask.rs |  |  | 0.462 |
| walker |  | 2352 | 0 | pub item at sps-core/src/uninstall/cask.rs:41 |  |  | 0.462 |
| walker |  | 2396 | 44 | pub item at sps-core/src/uninstall/cask.rs:134 |  |  | 0.462 |
| ns | 2402 |  | 182 | sps-common — model module exports | 2.5 | 2.2 | 0.468 |
| walker |  | 2488 | 92 | pub item at sps-common/src/config.rs:15 |  |  | 0.469 |
| ns | 2517 |  | 115 | sps-common — dependency module exports | 2.6 | 2.2 | 0.479 |
| walker |  | 2548 | 60 | pub item at sps-core/src/check/update.rs:23 |  |  | 0.479 |
| walker |  | 2563 | 15 | listing of 'sps-core/src/install/cask' |  |  | 0.480 |
| ns | 2744 |  | 227 | sps-core/install — module roster | 2.7 | 2.3 | 0.472 |
| ns | 2759 |  | 15 | sps-core/install/cask sub-listing | 2.8 | 2.7 | 0.479 |
| walker |  | 2781 | 218 | pub-item names surface in sps-core/src/install/cask/mod.rs |  |  | 0.479 |
| walker |  | 2781 | 0 | pub item at sps-core/src/install/cask/mod.rs:37 |  |  | 0.479 |
| walker |  | 2781 | 0 | pub item at sps-core/src/install/cask/mod.rs:43 |  |  | 0.479 |
| walker |  | 2781 | 0 | pub item at sps-core/src/install/cask/mod.rs:49 |  |  | 0.479 |
| walker |  | 2781 | 0 | pub item at sps-core/src/install/cask/mod.rs:73 |  |  | 0.479 |
| walker |  | 2781 | 0 | pub item at sps-core/src/install/cask/mod.rs:686 |  |  | 0.479 |
| walker |  | 2795 | 14 | pub item body at sps-core/src/install/cask/mod.rs:43 body 44 |  |  | 0.479 |
| walker |  | 2812 | 17 | pub item body at sps-core/src/install/cask/mod.rs:73 body 74 |  |  | 0.479 |
| walker |  | 2857 | 45 | pub item at sps-core/src/install/cask/mod.rs:580 |  |  | 0.479 |
| ns | 2874 |  | 115 | Cask artifact handler enumeration | 2.9 | 2.8 | 0.457 |
| walker |  | 2903 | 46 | pub item at sps-core/src/install/cask/mod.rs:77 |  |  | 0.457 |
| walker |  | 2950 | 47 | pub item at sps-core/src/install/cask/mod.rs:612 |  |  | 0.457 |
| walker |  | 3004 | 54 | pub item at sps-core/src/install/cask/mod.rs:232 |  |  | 0.457 |
| walker |  | 3044 | 40 | pub item body at sps-core/src/install/cask/mod.rs:37 body 38 |  |  | 0.457 |
| walker |  | 3162 | 118 | pub item at sps-core/src/install/cask/mod.rs:25 |  |  | 0.457 |
| walker |  | 3182 | 20 | pub-item doc lede at sps-core/src/install/cask/mod.rs:37 |  |  | 0.457 |
| walker |  | 3202 | 20 | pub-item doc lede at sps-core/src/install/cask/mod.rs:43 |  |  | 0.457 |
| walker |  | 3246 | 44 | pub-item doc lede at sps-core/src/install/cask/mod.rs:49 |  |  | 0.457 |
| walker |  | 3297 | 51 | pub-item doc lede at sps-core/src/install/cask/mod.rs:686 |  |  | 0.457 |
| walker |  | 3304 | 7 | pub item body at sps-core/src/install/cask/mod.rs:49 body 70 |  |  | 0.457 |
| ns | 3431 |  | 557 | Cask artifacts mod.rs — re-export wall | 2.10 | 2.9 | 0.420 |
| ns | 3447 |  | 16 | sps-core/install/bottle sub-listing | 2.11 | 2.7 | 0.417 |
| ns | 3492 |  | 45 | sps-core/build — compile sub-listing | 2.12 | 2.3 | 0.410 |
| ns | 3564 |  | 72 | sps-core — pipeline / check / uninstall / upgrade / utils sub-listings | 2.13 | 2.3 | 0.434 |
| ns | 3605 |  | 41 | sps-common — sub-tree listings (model / dependency) | 2.14 | 2.2 | 0.447 |
| ns | 3657 |  | 52 | sps bin — pipeline & cli sub-listings | 2.15 | 1.5 | 0.462 |
| walker |  | 3834 | 530 | README headline in README.md |  |  | 0.464 |
| walker |  | 3947 | 113 | headings outline in README.md |  |  | 0.464 |
| walker |  | 4007 | 60 | README.md section #0 |  |  | 0.482 |
| walker |  | 4084 | 77 | README.md section #1 |  |  | 0.482 |
| walker |  | 4101 | 17 | pub-item doc lede at sps-net/src/validation.rs:121 |  |  | 0.482 |
| ns | 4161 |  | 504 | Per-crate Cargo manifest signatures (name + deps highlights) | 2.16 |  | 0.458 |
| walker |  | 4165 | 64 | pub item at sps/src/cli/info.rs:14 |  |  | 0.458 |
| walker |  | 4217 | 52 | pub-item names surface in sps/src/pipeline/planner.rs |  |  | 0.458 |
| walker |  | 4233 | 16 | listing of 'sps-core/src/install/bottle' |  |  | 0.465 |
| ns | 4273 |  | 112 | Config struct fields | 3.1 |  | 0.472 |
| walker |  | 4308 | 75 | pub-item names surface in sps-core/src/install/bottle/mod.rs |  |  | 0.472 |
| walker |  | 4308 | 0 | pub item at sps-core/src/install/bottle/mod.rs:34 |  |  | 0.472 |
| walker |  | 4308 | 0 | pub item at sps-core/src/install/bottle/mod.rs:159 |  |  | 0.472 |
| walker |  | 4352 | 44 | pub item at sps-core/src/install/bottle/mod.rs:18 |  |  | 0.472 |
| walker |  | 4402 | 50 | pub item at sps-core/src/install/bottle/mod.rs:165 |  |  | 0.472 |
| walker |  | 4425 | 23 | pub item body at sps-core/src/install/bottle/mod.rs:159 body 160 |  |  | 0.472 |
| walker |  | 4438 | 13 | pub-item doc lede at sps-core/src/install/bottle/mod.rs:18 |  |  | 0.472 |
| walker |  | 4458 | 20 | pub-item doc lede at sps-core/src/install/bottle/mod.rs:34 |  |  | 0.472 |
| walker |  | 4548 | 90 | pub item body at sps-core/src/install/bottle/mod.rs:18 body 23 |  |  | 0.472 |
| ns | 4558 |  | 285 | Config path-method roster | 3.2 | 3.1 | 0.457 |
| walker |  | 4663 | 115 | pub item body at sps-core/src/install/bottle/mod.rs:34 body 35 |  |  | 0.457 |
| walker |  | 4689 | 26 | pub-item names surface in sps-core/src/install/bottle/link.rs |  |  | 0.457 |
| walker |  | 4776 | 87 | pub-item names surface in sps-net/src/http.rs |  |  | 0.457 |
| walker |  | 4819 | 43 | pub item at sps-net/src/http.rs:147 |  |  | 0.457 |
| walker |  | 4838 | 19 | pub item body at sps-net/src/http.rs:147 body 152 |  |  | 0.457 |
| walker |  | 4894 | 56 | pub item at sps-net/src/http.rs:155 |  |  | 0.457 |
| walker |  | 4958 | 64 | pub item at sps-net/src/http.rs:24 |  |  | 0.457 |
| walker |  | 5035 | 77 | pub item at sps-net/src/http.rs:42 |  |  | 0.457 |
| ns | 5074 |  | 516 | Config — sps_root resolution + cellar/cask path bodies | 3.3 | 3.2 | 0.439 |
| walker |  | 5103 | 68 | pub item at sps/src/cli/upgrade.rs:12 |  |  | 0.439 |
| walker |  | 5312 | 209 | mod/use plumbing in sps/src/main.rs |  |  | 0.439 |
| walker |  | 5402 | 90 | pub-item names surface in sps-common/src/pipeline.rs |  |  | 0.439 |
| walker |  | 5420 | 18 | pub item at sps-common/src/pipeline.rs:14 |  |  | 0.439 |
| ns | 5428 |  | 354 | SpsError variant signatures | 3.4 |  | 0.424 |
| walker |  | 5468 | 48 | pub item at sps-common/src/pipeline.rs:184 |  |  | 0.424 |
| walker |  | 5521 | 53 | pub item at sps-common/src/pipeline.rs:42 |  |  | 0.425 |
| walker |  | 5601 | 80 | pub item at sps-common/src/pipeline.rs:20 |  |  | 0.425 |
| walker |  | 5666 | 65 | pub item at sps-common/src/pipeline.rs:33 |  |  | 0.426 |
| ns | 5758 |  | 330 | InstallTargetIdentifier + Formula struct fields | 3.5 | 2.5 | 0.412 |
| walker |  | 5761 | 95 | pub item at sps-common/src/pipeline.rs:191 |  |  | 0.412 |
| walker |  | 5779 | 18 | pub-item doc lede at sps-common/src/pipeline.rs:191 |  |  | 0.412 |
| walker |  | 5840 | 61 | pub item at sps-core/src/upgrade/cask.rs:15 |  |  | 0.412 |
| walker |  | 5992 | 152 | mod/use plumbing in sps-core/src/install/bottle/mod.rs |  |  | 0.412 |
| walker |  | 6048 | 56 | pub-item names surface in sps-core/src/install/extract.rs |  |  | 0.412 |
| walker |  | 6048 | 0 | pub item at sps-core/src/install/extract.rs:205 |  |  | 0.412 |
| walker |  | 6100 | 52 | pub item at sps-core/src/install/extract.rs:235 |  |  | 0.412 |
| walker |  | 6120 | 20 | mod/use plumbing in sps-common/src/dependency/requirement.rs |  |  | 0.412 |
| ns | 6151 |  | 393 | Cask struct fields | 3.6 |  | 0.397 |
| walker |  | 6183 | 63 | pub item at sps-core/src/uninstall/formula.rs:10 |  |  | 0.397 |
| walker |  | 6203 | 20 | pub-item doc lede at sps-net/src/validation.rs:93 |  |  | 0.397 |
| walker |  | 6266 | 63 | pub-item names surface in sps/src/cli/search.rs |  |  | 0.397 |
| walker |  | 6266 | 0 | pub item at sps/src/cli/search.rs:240 |  |  | 0.397 |
| ns | 6269 |  | 118 | InstalledArtifact variants | 3.7 |  | 0.393 |
| walker |  | 6291 | 25 | pub item at sps/src/cli/search.rs:23 |  |  | 0.393 |
| walker |  | 6343 | 52 | pub item at sps/src/cli/search.rs:42 |  |  | 0.393 |
| walker |  | 6408 | 65 | pub item at sps/src/cli/search.rs:15 |  |  | 0.393 |
| walker |  | 6454 | 46 | pub item at sps-core/src/install/bottle/link.rs:19 |  |  | 0.393 |
| walker |  | 6518 | 64 | pub-item names surface in sps/src/pipeline/runner.rs |  |  | 0.393 |
| walker |  | 6518 | 0 | pub item at sps/src/pipeline/runner.rs:56 |  |  | 0.393 |
| ns | 6543 |  | 274 | Dependency + DependencyTag bitflags | 3.8 |  | 0.392 |
| walker |  | 6547 | 29 | pub item at sps/src/pipeline/runner.rs:31 |  |  | 0.392 |
| walker |  | 6585 | 38 | pub item at sps/src/pipeline/runner.rs:38 |  |  | 0.392 |
| walker |  | 6650 | 65 | pub item at sps/src/pipeline/runner.rs:67 |  |  | 0.392 |
| walker |  | 6714 | 64 | pub-item names surface in sps-core/src/check/installed.rs |  |  | 0.392 |
| walker |  | 6714 | 0 | pub item at sps-core/src/check/installed.rs:39 |  |  | 0.392 |
| walker |  | 6732 | 18 | pub item at sps-core/src/check/installed.rs:14 |  |  | 0.392 |
| ns | 6739 |  | 196 | Requirement enum (macOS / Xcode / Other) | 3.9 |  | 0.388 |
| walker |  | 6766 | 34 | pub item at sps-core/src/check/installed.rs:140 |  |  | 0.388 |
| walker |  | 6821 | 55 | pub item at sps-core/src/check/installed.rs:20 |  |  | 0.388 |
| walker |  | 6902 | 81 | pub item at sps/src/cli/reinstall.rs:12 |  |  | 0.388 |
| walker |  | 6925 | 23 | mod/use plumbing in sps-common/src/model/artifact.rs |  |  | 0.385 |
| ns | 6925 |  | 186 | Cache API surface | 3.10 |  | 0.385 |
| walker |  | 6940 | 15 | pub-item doc lede at sps-common/src/model/version.rs:11 |  |  | 0.385 |
| ns | 7098 |  | 173 | InstalledKeg + KegRegistry signatures | 3.11 |  | 0.383 |
| walker |  | 7215 | 275 | mod/use plumbing in sps-net/src/lib.rs |  |  | 0.407 |
| walker |  | 7295 | 80 | pub item at sps-core/src/upgrade/source.rs:19 |  |  | 0.407 |
| walker |  | 7368 | 73 | pub-item names surface in sps-core/src/install/devtools.rs |  |  | 0.407 |
| walker |  | 7368 | 0 | pub item at sps-core/src/install/devtools.rs:9 |  |  | 0.407 |
| walker |  | 7368 | 0 | pub item at sps-core/src/install/devtools.rs:78 |  |  | 0.407 |
| walker |  | 7368 | 0 | pub item at sps-core/src/install/devtools.rs:123 |  |  | 0.407 |
| walker |  | 7368 | 0 | pub item at sps-core/src/install/devtools.rs:160 |  |  | 0.407 |
| ns | 7436 |  | 338 | Pipeline shared types — JobAction / PlannedJob / WorkerJob / PipelineEvent header | 4.1 |  | 0.419 |
| walker |  | 7440 | 72 | README.md section #3 |  |  | 0.419 |
| walker |  | 7563 | 123 | pub-item names surface in sps-net/src/oci.rs |  |  | 0.419 |
| walker |  | 7563 | 0 | pub item at sps-net/src/oci.rs:190 |  |  | 0.419 |
| walker |  | 7604 | 41 | pub item at sps-net/src/oci.rs:37 |  |  | 0.419 |
| walker |  | 7648 | 44 | pub item at sps-net/src/oci.rs:182 |  |  | 0.419 |
| walker |  | 7672 | 24 | pub item body at sps-net/src/oci.rs:182 body 187 |  |  | 0.419 |
| walker |  | 7735 | 63 | pub item at sps-net/src/oci.rs:45 |  |  | 0.419 |
| walker |  | 7797 | 62 | pub item at sps-net/src/oci.rs:96 |  |  | 0.419 |
| walker |  | 7876 | 79 | pub item at sps-net/src/oci.rs:55 |  |  | 0.419 |
| ns | 7933 |  | 497 | JobProcessingState + DownloadOutcome + PlannedOperations | 4.2 | 4.1 | 0.413 |
| walker |  | 7951 | 75 | pub item at sps-net/src/oci.rs:114 |  |  | 0.413 |
| walker |  | 8001 | 50 | pub-item names surface in sps-core/src/install/bottle/macho.rs |  |  | 0.413 |
| walker |  | 8026 | 25 | pub item at sps-core/src/install/bottle/macho.rs:55 |  |  | 0.413 |
| walker |  | 8051 | 25 | pub item at sps-core/src/install/bottle/macho.rs:74 |  |  | 0.413 |
| walker |  | 8092 | 41 | pub item at sps-core/src/install/bottle/macho.rs:64 |  |  | 0.413 |
| walker |  | 8105 | 13 | pub item body at sps-core/src/install/bottle/macho.rs:64 body 68 |  |  | 0.413 |
| walker |  | 8149 | 44 | pub item at sps-core/src/install/bottle/macho.rs:81 |  |  | 0.419 |
| ns | 8149 |  | 216 | CommandType + PipelineFlags + run_pipeline signature | 4.3 |  | 0.419 |
| walker |  | 8160 | 11 | pub item body at sps-core/src/install/bottle/macho.rs:81 body 85 |  |  | 0.419 |
| walker |  | 8215 | 55 | pub item at sps-core/src/install/bottle/link.rs:476 |  |  | 0.419 |
| walker |  | 8293 | 78 | pub-item names surface in sps-common/src/model/formula.rs |  |  | 0.419 |
| walker |  | 8310 | 17 | pub item at sps-common/src/model/formula.rs:34 |  |  | 0.419 |
| walker |  | 8334 | 24 | pub item at sps-common/src/model/formula.rs:28 |  |  | 0.419 |
| walker |  | 8373 | 39 | pub item at sps-common/src/model/formula.rs:39 |  |  | 0.419 |
| walker |  | 8417 | 44 | pub item at sps-common/src/model/formula.rs:47 |  |  | 0.419 |
| ns | 8457 |  | 308 | OperationPlanner + plan_operations signature | 4.4 |  | 0.410 |
| walker |  | 8467 | 50 | pub item at sps-common/src/model/formula.rs:19 |  |  | 0.410 |
| walker |  | 8564 | 97 | pub item at sps-common/src/model/formula.rs:360 |  |  | 0.410 |
| walker |  | 8643 | 79 | pub-item names surface in sps-core/src/utils/xattr.rs |  |  | 0.410 |
| walker |  | 8643 | 0 | pub item at sps-core/src/utils/xattr.rs:26 |  |  | 0.410 |
| walker |  | 8643 | 0 | pub item at sps-core/src/utils/xattr.rs:41 |  |  | 0.410 |
| walker |  | 8643 | 0 | pub item at sps-core/src/utils/xattr.rs:53 |  |  | 0.410 |
| walker |  | 8744 | 101 | pub item at sps-common/src/model/tap.rs:10 |  |  | 0.410 |
| walker |  | 8761 | 17 | pub-item doc lede at sps-common/src/model/tap.rs:10 |  |  | 0.410 |
| ns | 8771 |  | 314 | DependencyResolver — strategy + status + ResolvedDependency | 4.5 |  | 0.401 |
| walker |  | 8851 | 90 | pub item at sps-core/src/pipeline/engine.rs:16 |  |  | 0.401 |
| walker |  | 9069 | 218 | pub item at sps-common/src/pipeline.rs:163 |  |  | 0.420 |
| walker |  | 9086 | 17 | pub-item doc lede at sps-common/src/pipeline.rs:163 |  |  | 0.422 |
| ns | 9097 |  | 326 | DependencyResolver — ResolutionContext + ResolvedGraph | 4.6 | 4.5 | 0.415 |
| walker |  | 9155 | 69 | pub item body at sps-net/src/oci.rs:96 body 103 |  |  | 0.415 |
| walker |  | 9173 | 18 | pub-item doc lede at sps-core/src/upgrade/cask.rs:15 |  |  | 0.415 |
| ns | 9280 |  | 183 | core worker entry — execute_sync_job | 4.7 |  | 0.411 |
| walker |  | 9282 | 109 | pub item at sps/src/cli/list.rs:16 |  |  | 0.411 |
| walker |  | 9353 | 71 | pub item body at sps-net/src/http.rs:24 body 31 |  |  | 0.411 |
| ns | 9433 |  | 153 | core worker pool — start_worker_pool_manager | 4.8 | 4.7 | 0.413 |
| walker |  | 9441 | 88 | pub-item names surface in sps-common/src/dependency/resolver.rs |  |  | 0.414 |
| walker |  | 9469 | 28 | pub item at sps-common/src/dependency/resolver.rs:15 |  |  | 0.415 |
| walker |  | 9503 | 34 | pub item at sps-common/src/dependency/resolver.rs:22 |  |  | 0.417 |
| walker |  | 9551 | 48 | pub item at sps-common/src/dependency/resolver.rs:53 |  |  | 0.421 |
| walker |  | 9615 | 64 | pub item at sps-common/src/dependency/resolver.rs:63 |  |  | 0.423 |
| ns | 9653 |  | 220 | Install entry-points — install_bottle / build_from_source / install_cask signatures | 4.9 |  | 0.423 |
| walker |  | 9689 | 74 | pub item at sps-common/src/dependency/resolver.rs:77 |  |  | 0.423 |
| walker |  | 9779 | 90 | pub item at sps-common/src/dependency/resolver.rs:42 |  |  | 0.434 |
| ns | 9784 |  | 131 | Bottle platform selection — get_bottle_for_platform | 4.10 | 4.9 | 0.432 |
| walker |  | 9810 | 31 | mod/use plumbing in sps-common/src/dependency/definition.rs |  |  | 0.432 |
| walker |  | 9906 | 96 | README.md section #2 |  |  | 0.432 |
| ns | 9912 |  | 128 | API entry-points (sps-net::api) | 4.11 |  | 0.430 |
| ns | 9983 |  | 71 | Uninstall + Upgrade entry-point fn names | 4.12 |  | 0.433 |

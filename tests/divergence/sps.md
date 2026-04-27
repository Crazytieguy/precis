scores: Sim=0.392 Reached=17/49 Early=7 Late=3 Partial=4 Missing=28 Used=9805/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 10 | 3 | 1 | 6 | 0.35 |
| 2 | 16 | 7 | 2 | 7 | 0.56 |
| 3 | 11 | 1 | 0 | 10 | 0.25 |
| 4 | 12 | 6 | 1 | 5 | 0.50 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 144 | — | — | 0.57 | partial | README lede — what sps is | README.md section #0 (t=3061, 3 atoms) |
| 1.4 | 327 | 635 | +308 | 0.92 | late | CLI subcommand enum | pub item at sps/src/cli.rs:44 (t=635, 11 atoms) |
| 1.5 | 503 | — | — | 0.00 | missing | CLI per-subcommand module list |  |
| 1.6 | 678 | — | — | 0.00 | missing | main.rs — Tokio entry signature + Init early-out |  |
| 1.7 | 744 | — | — | 0.00 | missing | main.rs — Config::load failure hint |  |
| 1.8 | 936 | — | — | 0.00 | missing | main.rs — auto-update gate |  |
| 1.9 | 1105 | — | — | 0.00 | missing | main.rs — Cache + Command::run dispatch |  |
| 1.10 | 1377 | — | — | 0.00 | missing | main.rs — pipeline-aware error printing on failure |  |
| 2.1 | 1476 | 758 | -718 | 1.00 | early | Per-crate src/ listings |  |
| 2.2 | 1651 | — | — | 0.71 | partial | sps-common lib.rs — module tree + re-exports | mod/use plumbing in sps-common/src/lib.rs (t=5774, 12 atoms) |
| 2.3 | 1859 | — | — | 0.40 | missing | sps-core lib.rs — module tree | mod/use plumbing in sps-core/src/lib.rs (t=1528, 8 atoms) |
| 2.4 | 2220 | — | — | 0.00 | missing | sps-net lib.rs — re-exports + dependency on sps-common |  |
| 2.5 | 2402 | — | — | 0.65 | partial | sps-common — model module exports | mod/use plumbing in sps-common/src/model/mod.rs (t=5656, 9 atoms) |
| 2.6 | 2517 | 6823 | +4306 | 0.80 | late | sps-common — dependency module exports | mod/use plumbing in sps-common/src/dependency/mod.rs (t=6823, 8 atoms) |
| 2.7 | 2744 | — | — | 0.37 | missing | sps-core/install — module roster | mod/use plumbing in sps-core/src/install/mod.rs (t=5123, 7 atoms) |
| 2.8 | 2759 | 1695 | -1064 | 1.00 | early | sps-core/install/cask sub-listing |  |
| 2.9 | 2874 | — | — | 0.00 | missing | Cask artifact handler enumeration |  |
| 2.10 | 3431 | — | — | 0.00 | missing | Cask artifacts mod.rs — re-export wall |  |
| 2.12 | 3492 | — | — | 0.00 | missing | sps-core/build — compile sub-listing |  |
| 2.13 | 3564 | 682 | -2882 | 1.00 | early | sps-core — pipeline / check / uninstall / upgrade / utils sub-listings |  |
| 2.15 | 3657 | 7291 | +3634 | 1.00 | late | sps bin — pipeline & cli sub-listings |  |
| 2.16 | 4161 | — | — | 0.00 | missing | Per-crate Cargo manifest signatures (name + deps highlights) |  |
| 3.1 | 4273 | 1620 | -2653 | 0.89 | early | Config struct fields | pub item at sps-common/src/config.rs:15 (t=1620, 8 atoms) |
| 3.2 | 4558 | — | — | 0.00 | missing | Config path-method roster |  |
| 3.3 | 5074 | — | — | 0.00 | missing | Config — sps_root resolution + cellar/cask path bodies | pub item at sps-common/src/config.rs:15 (t=1620, 8 atoms) |
| 3.4 | 5428 | — | — | 0.06 | missing | SpsError variant signatures | pub-item names surface in sps-common/src/error.rs (t=988, 3 atoms) |
| 3.5 | 5758 | — | — | 0.15 | missing | InstallTargetIdentifier + Formula struct fields | pub item at sps-common/src/model/mod.rs:17 (t=3126, 4 atoms) |
| 3.6 | 6151 | — | — | 0.00 | missing | Cask struct fields |  |
| 3.7 | 6269 | — | — | 0.10 | missing | InstalledArtifact variants | pub-item names surface in sps-common/src/model/artifact.rs (t=3147, 2 atoms) |
| 3.8 | 6543 | — | — | 0.48 | missing | Dependency + DependencyTag bitflags | pub item at sps-common/src/dependency/definition.rs:31 (t=1097, 5 atoms) |
| 3.9 | 6739 | — | — | 0.31 | missing | Requirement enum (macOS / Xcode / Other) | pub item at sps-common/src/dependency/requirement.rs:7 (t=1043, 5 atoms) |
| 3.10 | 6925 | — | — | 0.29 | missing | Cache API surface | pub item at sps-common/src/cache.rs:15 (t=955, 4 atoms) |
| 3.11 | 7098 | — | — | 0.47 | missing | InstalledKeg + KegRegistry signatures | pub item at sps-common/src/keg.rs:13 (t=878, 5 atoms) |
| 4.1 | 7436 | 4233 | -3203 | 0.82 | early | Pipeline shared types — JobAction / PlannedJob / WorkerJob / PipelineEvent header | pub item at sps-common/src/pipeline.rs:20 (t=4168, 11 atoms) |
| 4.2 | 7933 | 8294 | +361 | 0.83 | aligned | JobProcessingState + DownloadOutcome + PlannedOperations | pub item at sps-common/src/pipeline.rs:163 (t=8294, 18 atoms) |
| 4.3 | 8149 | 5385 | -2764 | 0.81 | early | CommandType + PipelineFlags + run_pipeline signature | pub-item names surface in sps/src/pipeline/runner.rs (t=5253, 8 atoms) |
| 4.4 | 8457 | — | — | 0.03 | missing | OperationPlanner + plan_operations signature | pub-item names surface in sps/src/pipeline/planner.rs (t=3308, 2 atoms) |
| 4.5 | 8771 | 8875 | +104 | 0.81 | aligned | DependencyResolver — strategy + status + ResolvedDependency | pub-item names surface in sps-common/src/dependency/resolver.rs (t=8537, 10 atoms) |
| 4.6 | 9097 | — | — | 0.26 | missing | DependencyResolver — ResolutionContext + ResolvedGraph | pub item at sps-common/src/dependency/resolver.rs:42 (t=8875, 9 atoms) |
| 4.7 | 9280 | — | — | 0.07 | missing | core worker entry — execute_sync_job | pub-item names surface in sps-core/src/pipeline/worker.rs (t=451, 2 atoms) |
| 4.8 | 9433 | 8076 | -1357 | 0.80 | aligned | core worker pool — start_worker_pool_manager | pub item at sps-core/src/pipeline/engine.rs:16 (t=8076, 9 atoms) |
| 4.9 | 9653 | — | — | 0.61 | partial | Install entry-points — install_bottle / build_from_source / install_cask signatures | pub item at sps-core/src/install/cask/mod.rs:232 (t=2105, 6 atoms) |
| 4.10 | 9784 | — | — | 0.00 | missing | Bottle platform selection — get_bottle_for_platform |  |
| 4.11 | 9912 | — | — | 0.00 | missing | API entry-points (sps-net::api) |  |
| 4.12 | 9983 | 1407 | -8576 | 1.00 | early | Uninstall + Upgrade entry-point fn names | pub-item names surface in sps-core/src/uninstall/cask.rs (t=1407, 3 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 4 | 313 | README.md section #<n> |
| 4 | 279 | pub item at sps-net/src/oci.rs:<n> |
| 3 | 197 | pub item at sps-net/src/http.rs:<n> |
| 2 | 147 | pub item at sps-common/src/model/formula.rs:<n> |
| 2 | 117 | pub item at sps/src/cli/search.rs:<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 503 | 0.95 | 530 | 2888 | README headline in README.md |
| 272 | 1.00 | 272 | 4858 | headings outline in CONTRIBUTING.md |
| 209 | 1.00 | 209 | 9805 | mod/use plumbing in sps/src/main.rs |
| 206 | 0.94 | 218 | 1913 | pub-item names surface in sps-core/src/install/cask/mod.rs |
| 152 | 1.00 | 152 | 9596 | mod/use plumbing in sps-core/src/install/bottle/mod.rs |
| 123 | 1.00 | 123 | 6122 | pub-item names surface in sps-net/src/oci.rs |
| 118 | 1.00 | 118 | 2223 | pub item at sps-core/src/install/cask/mod.rs:25 |
| 113 | 1.00 | 113 | 3001 | headings outline in README.md |
| 109 | 1.00 | 109 | 8449 | pub item at sps/src/cli/list.rs:16 |
| 101 | 1.00 | 101 | 7969 | pub item at sps-common/src/model/tap.rs:10 |
| 100 | 1.00 | 100 | 9234 | pub-item names surface in sps-core/src/uninstall/common.rs |
| 99 | 1.00 | 99 | 9070 | pub item at sps-core/src/upgrade/bottle.rs:20 |
| 97 | 1.00 | 97 | 7172 | pub item at sps-common/src/model/formula.rs:360 |
| 96 | 1.00 | 96 | 8971 | README.md section #2 |
| 87 | 1.00 | 87 | 3639 | pub-item names surface in sps-net/src/http.rs |
| 81 | 1.00 | 81 | 7868 | pub item at sps/src/cli/reinstall.rs:12 |
| 80 | 1.00 | 80 | 5854 | pub item at sps-core/src/upgrade/source.rs:19 |
| 79 | 1.00 | 79 | 6411 | pub item at sps-net/src/oci.rs:55 |
| 79 | 1.00 | 79 | 7251 | pub-item names surface in sps-core/src/utils/xattr.rs |
| 77 | 1.00 | 77 | 3239 | README.md section #1 |
| 77 | 1.00 | 77 | 3879 | pub item at sps-net/src/http.rs:42 |
| 77 | 1.00 | 77 | 9428 | pub-item names surface in sps-core/src/install/cask/dmg.rs |
| 75 | 1.00 | 75 | 6486 | pub item at sps-net/src/oci.rs:114 |
| 75 | 1.00 | 75 | 3399 | pub-item names surface in sps-core/src/install/bottle/mod.rs |
| 74 | 1.00 | 74 | 8785 | pub item at sps-common/src/dependency/resolver.rs:77 |
| 73 | 1.00 | 73 | 5049 | mod/use plumbing in sps-core/src/uninstall/mod.rs |
| 73 | 1.00 | 73 | 5927 | pub-item names surface in sps-core/src/install/devtools.rs |
| 72 | 1.00 | 72 | 5999 | README.md section #3 |
| 71 | 1.00 | 71 | 4478 | mod/use plumbing in sps-core/src/upgrade/mod.rs |
| 70 | 1.00 | 70 | 1363 | pub-item names surface in sps-net/src/validation.rs |
| 68 | 1.00 | 68 | 9317 | README.md section #9 |
| 68 | 1.00 | 68 | 7582 | pub item at sps/src/cli/upgrade.rs:12 |
| 68 | 0.87 | 78 | 6901 | pub-item names surface in sps-common/src/model/formula.rs |
| 65 | 1.00 | 65 | 7787 | pub item at sps/src/cli/search.rs:15 |
| 64 | 1.00 | 64 | 3802 | pub item at sps-net/src/http.rs:24 |
| 64 | 1.00 | 64 | 7514 | pub item at sps/src/cli/info.rs:14 |
| 64 | 1.00 | 64 | 5449 | pub-item names surface in sps-core/src/check/installed.rs |
| 64 | 1.00 | 64 | 9134 | pub-item names surface in sps-core/src/install/cask/helpers.rs |
| 63 | 1.00 | 63 | 4976 | pub item at sps-core/src/uninstall/formula.rs:10 |
| 63 | 1.00 | 63 | 6270 | pub item at sps-net/src/oci.rs:45 |
| 63 | 1.00 | 63 | 7645 | pub-item names surface in sps/src/cli/search.rs |
| 62 | 1.00 | 62 | 6332 | pub item at sps-net/src/oci.rs:96 |
| 61 | 1.00 | 61 | 4407 | pub item at sps-core/src/upgrade/cask.rs:15 |
| 60 | 1.00 | 60 | 1680 | pub item at sps-core/src/check/update.rs:23 |
| 56 | 1.00 | 56 | 3738 | pub item at sps-net/src/http.rs:155 |
| 56 | 1.00 | 56 | 335 | pub item at sps/src/cli.rs:35 |
| 56 | 1.00 | 56 | 4534 | pub-item names surface in sps-core/src/install/extract.rs |
| 55 | 1.00 | 55 | 4913 | CONTRIBUTING.md section #0 |
| 55 | 1.00 | 55 | 5556 | pub item at sps-core/src/check/installed.rs:20 |
| 55 | 1.00 | 55 | 6726 | pub item at sps-core/src/install/bottle/link.rs:476 |
| 52 | 1.00 | 52 | 4586 | pub item at sps-core/src/install/extract.rs:235 |
| 52 | 1.00 | 52 | 7722 | pub item at sps/src/cli/search.rs:42 |
| 51 | 1.00 | 51 | 2358 | pub-item doc lede at sps-core/src/install/cask/mod.rs:686 |
| 50 | 1.00 | 50 | 7075 | pub item at sps-common/src/model/formula.rs:19 |
| 50 | 1.00 | 50 | 3493 | pub item at sps-core/src/install/bottle/mod.rs:165 |
| 50 | 1.00 | 50 | 6536 | pub-item names surface in sps-core/src/install/bottle/macho.rs |

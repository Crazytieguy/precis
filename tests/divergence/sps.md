scores: Score(3000)=0.457 ns_rows≤3K=19/49 (reached=5 partial=1 missing=13)

## Per-budget scores

| B | A_B | I(B) | C(B) | compl(B) | Score(B) | walker_used |
|--:|----:|-----:|-----:|---------:|---------:|------------:|
| 1000 | 96 | 0.743 | 0.220 | 0.539 | 0.405 | 978 |
| 1442 | 130 | 0.705 | 0.163 | 0.539 | 0.339 | 1421 |
| 2080 | 194 | 0.721 | 0.346 | 0.690 | 0.499 | 2015 |
| 3000 | 303 | 0.680 | 0.306 | 0.726 | 0.457 | 2990 |
| 4327 | 452 | 0.678 | 0.328 | 0.818 | 0.472 | 4281 |
| 6240 | 617 | 0.653 | 0.241 | 0.740 | 0.397 | 6168 |
| 9000 | 852 | 0.638 | 0.252 | 0.630 | 0.401 | 8939 |

## Top opportunities

### Additive (close partial / missing rows)

| intervention | rows | gap@1k | gap@3k | gap@9k | evidence | top row ids |
|:-------------|-----:|-------:|-------:|-------:|:---------|:------------|
| split wrong-slice walker batches | 26 | 2.45 | 2.34 | 1.96 | nearby candidates have low exact atom overlap | 1.2, 1.5, 1.6, 1.8, 2.4, ... |
| tune ranking for high-overlap unscheduled candidates | 7 | 0.44 | 0.44 | 0.44 | high-overlap candidates not in the schedule by T_max, exact total=125/136 | 1.10, 1.7, 3.5, 3.4, 3.2, ... |
| promote listing of 'sps-core/src/install/cask/artifacts' | 1 | 0.14 | 0.14 | 0.14 | 0 files, exact total=45/47 | 2.10 |

### Subtractive (suppress consistently off-NS batches)

| pattern | batches | freed@1k | freed@3k | freed@9k | evidence | top batch ids |
|:--------|--------:|---------:|---------:|---------:|:---------|:--------------|
| pub-item names surface in sps-core/src/install/cask/mod.rs | 1 | 0 | 206 | 206 | off_3k=218 | pub-item names surface in sps-core/src/install/cask/mod.rs |
| mod/use plumbing in sps-core/src/uninstall/mod.rs | 1 | 0 | 73 | 73 | off_3k=73 | mod/use plumbing in sps-core/src/uninstall/mod.rs |
| mod/use plumbing in sps-core/src/upgrade/mod.rs | 1 | 0 | 71 | 71 | off_3k=71 | mod/use plumbing in sps-core/src/upgrade/mod.rs |
| pub-item names surface in sps-net/src/validation.rs | 1 | 0 | 70 | 70 | off_3k=70 | pub-item names surface in sps-net/src/validation.rs |
| pub item at sps-core/src/check/update.rs:<n> | 1 | 0 | 60 | 60 | off_3k=60 | pub item at sps-core/src/check/update.rs:23 |

Top missed paths (NS rows ≤ 3K): sps/src/main.rs (5 rows, 71 atoms), sps-net/src/lib.rs (1 row, 29 atoms), sps-core/src/install/cask/artifacts (1 row, 24 atoms), sps-core/src/install/mod.rs (1 row, 22 atoms), sps-common/src/model/mod.rs (1 row, 20 atoms), sps-core/src/lib.rs (1 row, 20 atoms), sps/src/cli.rs (1 row, 19 atoms), sps-common/src/lib.rs (1 row, 17 atoms), +2 more

## Arrival ledger by diagnosis

### ranking-recoverable

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 1.7 | 744 | 0.00 | missing | main.rs — Config::load failure hint | [unscheduled bbox exact=5/5] entry item body at sps/src/main.rs:57 body 72 (5 atoms, too expensive at final margin) |
| 1.10 | 1377 | 0.00 | missing | main.rs — pipeline-aware error printing on failure | [unscheduled bbox exact=20/21] entry item body at sps/src/main.rs:57 body 167 (20 atoms, too expensive at final margin) |
| 2.10 | 3431 | 0.00 | missing | Cask artifacts mod.rs — re-export wall | [unscheduled bbox exact=45/47] mod/use plumbing in sps-core/src/install/cask/artifacts/mod.rs (45 atoms, predecessor not scheduled: listing of 'sps-core/src/install/cask/artifacts') |
| 3.2 | 4558 | 0.00 | missing | Config path-method roster | [unscheduled bbox exact=25/25] impl method sigs in sps-common/src/config.rs (54 atoms, too expensive at final margin) |
| 3.4 | 5428 | 0.06 | missing | SpsError variant signatures | [scheduled bbox exact=2/31] pub-item names surface in sps-common/src/error.rs (t=1296, 3 atoms); better unscheduled exact=29/31: pub item at sps-common/src/error.rs:6 (56 atoms, too expensive at final margin) |
| 3.5 | 5758 | 0.12 | missing | InstallTargetIdentifier + Formula struct fields | [scheduled bbox exact=4/34] pub item at sps-common/src/model/mod.rs:17 (t=1006, 4 atoms); better unscheduled exact=28/34: pub item at sps-common/src/model/formula.rs:57 (28 atoms, too expensive at final margin) |
| 3.7 | 6269 | 0.10 | missing | InstalledArtifact variants | [scheduled bbox exact=1/10] pub-item names surface in sps-common/src/model/artifact.rs (t=1037, 2 atoms); better unscheduled exact=9/10: pub item at sps-common/src/model/artifact.rs:9 (30 atoms, too expensive at final margin) |
| 4.11 | 9912 | 0.00 | missing | API entry-points (sps-net::api) | [unscheduled bbox exact=9/10] pub-item names surface in sps-net/src/api.rs (17 atoms, too expensive at final margin) |

### wrong-slice / granularity

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 1.2 | 144 | 0.00 | missing | README lede — what sps is | [scheduled bbox exact=3/7] README.md section #0 (t=4047, 3 atoms) |
| 1.5 | 503 | 0.00 | missing | CLI per-subcommand module list | [unscheduled bbox exact=14/19] mod/use plumbing in sps/src/cli.rs (14 atoms, too expensive at final margin) |
| 1.6 | 678 | 0.06 | missing | main.rs — Tokio entry signature + Init early-out | [scheduled bbox exact=2/16] entry item at sps/src/main.rs:57 (t=108, 2 atoms); better unscheduled exact=11/16: entry item body at sps/src/main.rs:57 body 60 (11 atoms, too expensive at final margin) |
| 1.8 | 936 | 0.00 | missing | main.rs — auto-update gate | [unscheduled bbox exact=11/16] entry item body at sps/src/main.rs:57 body 146 (11 atoms, too expensive at final margin) |
| 1.9 | 1105 | 0.00 | missing | main.rs — Cache + Command::run dispatch | [unscheduled bbox exact=7/13] entry item body at sps/src/main.rs:57 body 159 (7 atoms, too expensive at final margin) |
| 2.2 | 1651 | 0.71 | missing | sps-common lib.rs — module tree + re-exports | [scheduled bbox exact=12/17] mod/use plumbing in sps-common/src/lib.rs (t=1770, 12 atoms) |
| 2.3 | 1859 | 0.40 | missing | sps-core lib.rs — module tree | [scheduled bbox exact=8/20] mod/use plumbing in sps-core/src/lib.rs (t=821, 8 atoms) |
| 2.4 | 2220 | 0.00 | missing | sps-net lib.rs — re-exports + dependency on sps-common | [scheduled bbox exact=23/29] mod/use plumbing in sps-net/src/lib.rs (t=7590, 23 atoms) |
| 2.5 | 2402 | 0.65 | missing | sps-common — model module exports | [scheduled bbox exact=9/20] mod/use plumbing in sps-common/src/model/mod.rs (t=1855, 9 atoms) |
| 2.7 | 2744 | 0.45 | missing | sps-core/install — module roster | [scheduled bbox exact=7/22] mod/use plumbing in sps-core/src/install/mod.rs (t=1568, 7 atoms) |
| 2.16 | 4161 | 0.00 | missing | Per-crate Cargo manifest signatures (name + deps highlights) | [unscheduled bbox exact=11/38] [package] in sps/Cargo.toml (11 atoms, too expensive at final margin) |
| 3.3 | 5074 | 0.00 | missing | Config — sps_root resolution + cellar/cask path bodies | [scheduled bbox exact=0/36] pub item at sps-common/src/config.rs:15 (t=2528, 8 atoms); better unscheduled exact=2/36: impl method sigs in sps-common/src/config.rs (3 atoms, too expensive at final margin) |
| 3.6 | 6151 | 0.00 | missing | Cask struct fields | [unscheduled bbox exact=28/39] pub item at sps-common/src/model/cask.rs:116 (28 atoms, predecessor not scheduled: pub-item names surface in sps-common/src/model/cask.rs) |
| 3.8 | 6543 | 0.48 | missing | Dependency + DependencyTag bitflags | [scheduled bbox exact=5/21] pub item at sps-common/src/dependency/definition.rs:53 (t=2015, 5 atoms) |
| 3.9 | 6739 | 0.31 | missing | Requirement enum (macOS / Xcode / Other) | [scheduled bbox exact=5/16] pub item at sps-common/src/dependency/requirement.rs:7 (t=1186, 5 atoms) |
| 3.10 | 6925 | 0.29 | missing | Cache API surface | [scheduled bbox exact=4/17] pub item at sps-common/src/cache.rs:15 (t=1263, 4 atoms); better unscheduled exact=9/17: impl method sigs in sps-common/src/cache.rs (16 atoms, too expensive at final margin) |
| 3.11 | 7098 | 0.47 | missing | InstalledKeg + KegRegistry signatures | [scheduled bbox exact=5/17] pub item at sps-common/src/keg.rs:13 (t=1157, 5 atoms); better unscheduled exact=7/17: impl method sigs in sps-common/src/keg.rs (14 atoms, too expensive at final margin) |
| 4.1 | 7436 | 0.00 | missing | Pipeline shared types — JobAction / PlannedJob / WorkerJob / PipelineEvent header | [scheduled bbox exact=11/34] pub item at sps-common/src/pipeline.rs:20 (t=5649, 11 atoms) |
| 4.2 | 7933 | 0.00 | missing | JobProcessingState + DownloadOutcome + PlannedOperations | [scheduled bbox exact=18/36] pub item at sps-common/src/pipeline.rs:163 (t=9444, 18 atoms) |
| 4.3 | 8149 | 0.00 | missing | CommandType + PipelineFlags + run_pipeline signature | [scheduled bbox exact=6/21] pub-item names surface in sps/src/pipeline/runner.rs (t=6893, 8 atoms) |
| 4.4 | 8457 | 0.00 | missing | OperationPlanner + plan_operations signature | [scheduled bbox exact=2/31] pub-item names surface in sps/src/pipeline/planner.rs (t=4265, 2 atoms); better unscheduled exact=14/31: impl method sigs in sps/src/pipeline/planner.rs (34 atoms, too expensive at final margin) |
| 4.5 | 8771 | 0.00 | missing | DependencyResolver — strategy + status + ResolvedDependency | [scheduled bbox exact=8/32] pub-item names surface in sps-common/src/dependency/resolver.rs (t=9827, 10 atoms); better unscheduled exact=9/32: pub item at sps-common/src/dependency/resolver.rs:42 (9 atoms, too expensive at final margin) |
| 4.6 | 9097 | 0.00 | missing | DependencyResolver — ResolutionContext + ResolvedGraph | [scheduled bbox exact=0/27] pub item at sps-common/src/dependency/resolver.rs:53 (t=9937, 8 atoms); better unscheduled exact=13/27: pub item at sps-common/src/dependency/resolver.rs:27 (13 atoms, too expensive at final margin) |
| 4.7 | 9280 | 0.07 | missing | core worker entry — execute_sync_job | [scheduled bbox exact=2/14] pub-item names surface in sps-core/src/pipeline/worker.rs (t=674, 2 atoms); better unscheduled exact=7/14: pub item at sps-core/src/pipeline/worker.rs:21 (7 atoms, too expensive at final margin) |
| 4.9 | 9653 | 0.06 | missing | Install entry-points — install_bottle / build_from_source / install_cask signatures | [scheduled bbox exact=6/18] pub item at sps-core/src/install/cask/mod.rs:232 (t=3044, 6 atoms) |
| 4.10 | 9784 | 0.00 | missing | Bottle platform selection — get_bottle_for_platform | [unscheduled bbox exact=3/5] pub item body at sps-core/src/install/bottle/exec.rs:210 body 247 (3 atoms, predecessor not scheduled: pub item at sps-core/src/install/bottle/exec.rs:210) |

### fs/listing

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 2.9 | 2874 | 0.00 | missing | Cask artifact handler enumeration | fs-only |
| 2.11 | 3447 | 0.00 | missing | sps-core/install/bottle sub-listing | fs-only |
| 2.12 | 3492 | 0.00 | missing | sps-core/build — compile sub-listing | fs-only |

### mixed/unknown

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 2.6 | 2517 | 0.80 | partial | sps-common — dependency module exports | [scheduled bbox exact=8/10] mod/use plumbing in sps-common/src/dependency/mod.rs (t=2112, 8 atoms) |
| 4.8 | 9433 | 0.11 | missing | core worker pool — start_worker_pool_manager | [scheduled bbox exact=9/10] pub item at sps-core/src/pipeline/engine.rs:16 (t=9226, 9 atoms) |

Top wasted paths (off-NS at 3K): sps-core/src/install/cask/mod.rs (272t, 2 batches), sps-common/src/config.rs (92t, 1 batch), sps-core/src/uninstall/mod.rs (73t, 1 batch), sps-core/src/upgrade/mod.rs (71t, 1 batch), sps-net/src/validation.rs (70t, 1 batch), sps-common/src/dependency/definition.rs (61t, 1 batch), sps-core/src/check/update.rs (60t, 1 batch), sps/src/cli.rs (56t, 1 batch)

## Walker waste (walker_t ≤ 3000, off_3k ≥ 50)

| off_3k | off_3k_ratio | off_any | cost | walker_t | batch |
|-------:|-------------:|--------:|-----:|---------:|:------|
| 218 | 1.00 | 206 | 218 | 2603 | pub-item names surface in sps-core/src/install/cask/mod.rs |
| 92 | 1.00 | 0 | 92 | 2436 | pub item at sps-common/src/config.rs:15 |
| 73 | 1.00 | 73 | 73 | 1421 | mod/use plumbing in sps-core/src/uninstall/mod.rs |
| 71 | 1.00 | 71 | 71 | 1317 | mod/use plumbing in sps-core/src/upgrade/mod.rs |
| 70 | 1.00 | 70 | 70 | 2278 | pub-item names surface in sps-net/src/validation.rs |
| 61 | 1.00 | 0 | 61 | 1954 | pub item at sps-common/src/dependency/definition.rs:53 |
| 60 | 1.00 | 60 | 60 | 2528 | pub item at sps-core/src/check/update.rs:23 |
| 56 | 1.00 | 56 | 56 | 1568 | pub item at sps/src/cli.rs:35 |
| 54 | 1.00 | 2 | 54 | 2990 | pub item at sps-core/src/install/cask/mod.rs:232 |

scores: Sim=0.375 Reached=17/49 Early=7 Late=3 Partial=4 Missing=28 Used=9805/10000

## Verdict

Verdict: wrong-slice bound
Likely primary lever: split walker batches to match NS semantic slices
Evidence: 4 ranking-recoverable (w×gap=0.16), 25 wrong-slice/granularity (w×gap=5.72), 1 no-discovered (w×gap=0.18)
Secondary intervention: free final budget for 4 too-expensive candidates
Loss reasons: 0 predecessor-gated, 4 too-expensive, 0 discovered-unscheduled
Top rows: 1.5, 1.6, 1.7, 1.8, 1.9, ...
Note: likely lever is heuristic; verify `Sim` moves, not just bucket counts.

## Top opportunities

_`w(t)×gap` is a non-additive priority score: Σ exp(-exp_t/τ) × (1 - credit) per row, τ=2000. Same time weighting and credit gap as Sim, but rows can overlap between opportunities — sums across rows are an upper bound on Sim impact, not an additive estimate._

| intervention | rows | w(t)×gap | bands ≤3k/≤6k/total | evidence | top row ids |
|:-------------|-----:|---------:|:----------------------|:---------|:------------|
| split wrong-slice walker batches | 25 | 5.72 | 12/15/25 | nearby candidates have low exact atom overlap | 1.5, 1.6, 1.7, 1.8, 1.9, ... |
| add walker candidates for no-discovered rows | 1 | 0.18 | 0/1/1 | NS rows have no discovered line candidate | 2.10 |
| free final budget / demote late waste | 4 | 0.16 | 0/2/4 | high-overlap candidates exceed final remaining budget, exact total=75/85 | 3.4, 3.5, 3.7, 4.11 |

Tiers: 1=3/10 reached, 1 partial, 6 missing, avg=0.35; 2=7/16 reached, 2 partial, 7 missing, avg=0.56; 3=1/11 reached, 0 partial, 10 missing, avg=0.25; 4=6/12 reached, 1 partial, 5 missing, avg=0.50

## Diagnosis rollup

| diagnosis | rows | missing | partial | timing | likely lever |
|:----------|-----:|--------:|--------:|-------:|:-------------|
| ranking-recoverable | 4 | 4 | 0 | 0 | value/ranking |
| wrong-slice / granularity | 25 | 21 | 4 | 0 | walker granularity / wrong slice |
| no discovered candidate | 1 | 1 | 0 | 0 | walker coverage or predecessor-gated emit |
| fs/listing | 2 | 2 | 0 | 0 | filesystem/listing value |
| timing-only | 13 | 0 | 0 | 13 | usually no code change |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | w(t)×gap | likely lever |
|:------------|-----:|---------:|:-------------|
| too expensive at final margin | 4 | 0.16 | free final budget |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=26, unscheduled bbox=5, scheduled same-file=7, fs-only=6, no discovered candidate=1

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | aligned | low | 2 |
| scheduled bbox | aligned | high | 1 |
| scheduled bbox | early | low | 3 |
| scheduled bbox | early | high | 1 |
| scheduled bbox | late | high | 2 |
| scheduled bbox | missing | none | 2 |
| scheduled bbox | missing | low | 11 |
| scheduled bbox | partial | low | 4 |
| unscheduled bbox | missing | low | 4 |
| unscheduled bbox | missing | high | 1 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 3.4 | 5428 | — | — | 0.06 | missing | SpsError variant signatures | [scheduled bbox exact=2/31] pub-item names surface in sps-common/src/error.rs (t=660, 3 atoms); better unscheduled exact=29/31: pub item at sps-common/src/error.rs:6 (56 atoms, too expensive at final margin) |
| 3.5 | 5758 | — | — | 0.15 | missing | InstallTargetIdentifier + Formula struct fields | [scheduled bbox exact=4/34] pub item at sps-common/src/model/mod.rs:17 (t=3126, 4 atoms); better unscheduled exact=28/34: pub item at sps-common/src/model/formula.rs:57 (28 atoms, too expensive at final margin) |
| 3.7 | 6269 | — | — | 0.10 | missing | InstalledArtifact variants | [scheduled bbox exact=1/10] pub-item names surface in sps-common/src/model/artifact.rs (t=3147, 2 atoms); better unscheduled exact=9/10: pub item at sps-common/src/model/artifact.rs:9 (30 atoms, too expensive at final margin) |
| 4.11 | 9912 | — | — | 0.00 | missing | API entry-points (sps-net::api) | [unscheduled bbox exact=9/10] pub-item names surface in sps-net/src/api.rs (17 atoms, too expensive at final margin) |

### wrong-slice / granularity

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 144 | — | — | 0.57 | partial | README lede — what sps is | [scheduled bbox exact=3/7] README.md section #0 (t=3061, 3 atoms) |
| 1.5 | 503 | — | — | 0.00 | missing | CLI per-subcommand module list | [scheduled same-file] pub item at sps/src/cli.rs:44 (t=1021, 11 atoms) |
| 1.6 | 678 | — | — | 0.00 | missing | main.rs — Tokio entry signature + Init early-out | [scheduled same-file] mod/use plumbing in sps/src/main.rs (t=9805, 16 atoms) |
| 1.7 | 744 | — | — | 0.00 | missing | main.rs — Config::load failure hint | [scheduled same-file] mod/use plumbing in sps/src/main.rs (t=9805, 16 atoms) |
| 1.8 | 936 | — | — | 0.00 | missing | main.rs — auto-update gate | [scheduled same-file] mod/use plumbing in sps/src/main.rs (t=9805, 16 atoms) |
| 1.9 | 1105 | — | — | 0.00 | missing | main.rs — Cache + Command::run dispatch | [scheduled same-file] mod/use plumbing in sps/src/main.rs (t=9805, 16 atoms) |
| 1.10 | 1377 | — | — | 0.00 | missing | main.rs — pipeline-aware error printing on failure | [scheduled same-file] mod/use plumbing in sps/src/main.rs (t=9805, 16 atoms) |
| 2.2 | 1651 | — | — | 0.71 | partial | sps-common lib.rs — module tree + re-exports | [scheduled bbox exact=12/17] mod/use plumbing in sps-common/src/lib.rs (t=5774, 12 atoms) |
| 2.3 | 1859 | — | — | 0.40 | missing | sps-core lib.rs — module tree | [scheduled bbox exact=8/20] mod/use plumbing in sps-core/src/lib.rs (t=1528, 8 atoms) |
| 2.4 | 2220 | — | — | 0.00 | missing | sps-net lib.rs — re-exports + dependency on sps-common | [unscheduled bbox exact=23/29] mod/use plumbing in sps-net/src/lib.rs (23 atoms, too expensive at final margin) |
| 2.5 | 2402 | — | — | 0.65 | partial | sps-common — model module exports | [scheduled bbox exact=9/20] mod/use plumbing in sps-common/src/model/mod.rs (t=5656, 9 atoms) |
| 2.7 | 2744 | — | — | 0.37 | missing | sps-core/install — module roster | [scheduled bbox exact=7/22] mod/use plumbing in sps-core/src/install/mod.rs (t=5123, 7 atoms) |
| 2.16 | 4161 | — | — | 0.00 | missing | Per-crate Cargo manifest signatures (name + deps highlights) | [unscheduled bbox exact=11/38] [package] in sps/Cargo.toml (11 atoms, discovered unscheduled) |
| 3.2 | 4558 | — | — | 0.00 | missing | Config path-method roster | [scheduled same-file] pub item at sps-common/src/config.rs:15 (t=1620, 8 atoms) |
| 3.3 | 5074 | — | — | 0.00 | missing | Config — sps_root resolution + cellar/cask path bodies | [scheduled bbox exact=0/36] pub item at sps-common/src/config.rs:15 (t=1620, 8 atoms) |
| 3.6 | 6151 | — | — | 0.00 | missing | Cask struct fields | [unscheduled bbox exact=28/39] pub item at sps-common/src/model/cask.rs:116 (28 atoms, predecessor not scheduled: pub-item names surface in sps-common/src/model/cask.rs) |
| 3.8 | 6543 | — | — | 0.48 | missing | Dependency + DependencyTag bitflags | [scheduled bbox exact=5/21] pub item at sps-common/src/dependency/definition.rs:53 (t=1170, 5 atoms) |
| 3.9 | 6739 | — | — | 0.31 | missing | Requirement enum (macOS / Xcode / Other) | [scheduled bbox exact=5/16] pub item at sps-common/src/dependency/requirement.rs:7 (t=826, 5 atoms) |
| 3.10 | 6925 | — | — | 0.29 | missing | Cache API surface | [scheduled bbox exact=4/17] pub item at sps-common/src/cache.rs:15 (t=627, 4 atoms) |
| 3.11 | 7098 | — | — | 0.47 | missing | InstalledKeg + KegRegistry signatures | [scheduled bbox exact=5/17] pub item at sps-common/src/keg.rs:13 (t=550, 5 atoms) |
| 4.4 | 8457 | — | — | 0.03 | missing | OperationPlanner + plan_operations signature | [scheduled bbox exact=2/31] pub-item names surface in sps/src/pipeline/planner.rs (t=3308, 2 atoms); better unscheduled exact=6/31: pub item at sps/src/pipeline/planner.rs:184 (6 atoms, discovered unscheduled) |
| 4.6 | 9097 | — | — | 0.26 | missing | DependencyResolver — ResolutionContext + ResolvedGraph | [scheduled bbox exact=0/27] pub item at sps-common/src/dependency/resolver.rs:42 (t=8875, 9 atoms); better unscheduled exact=13/27: pub item at sps-common/src/dependency/resolver.rs:27 (13 atoms, discovered unscheduled) |
| 4.7 | 9280 | — | — | 0.07 | missing | core worker entry — execute_sync_job | [scheduled bbox exact=2/14] pub-item names surface in sps-core/src/pipeline/worker.rs (t=395, 2 atoms); better unscheduled exact=7/14: pub item at sps-core/src/pipeline/worker.rs:21 (7 atoms, discovered unscheduled) |
| 4.9 | 9653 | — | — | 0.61 | partial | Install entry-points — install_bottle / build_from_source / install_cask signatures | [scheduled bbox exact=6/18] pub item at sps-core/src/install/cask/mod.rs:232 (t=2105, 6 atoms) |
| 4.10 | 9784 | — | — | 0.00 | missing | Bottle platform selection — get_bottle_for_platform | [unscheduled bbox exact=1/5] pub item at sps-core/src/install/bottle/exec.rs:210 (2 atoms, predecessor not scheduled: pub-item names surface in sps-core/src/install/bottle/exec.rs) |

### no discovered candidate

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 2.10 | 3431 | — | — | 0.00 | missing | Cask artifacts mod.rs — re-export wall | no discovered line candidate |

### fs/listing

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 2.9 | 2874 | — | — | 0.00 | missing | Cask artifact handler enumeration | fs-only |
| 2.12 | 3492 | — | — | 0.00 | missing | sps-core/build — compile sub-listing | fs-only |

### timing-only

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.4 | 327 | 1021 | +694 | 0.92 | late | CLI subcommand enum | [scheduled bbox exact=11/12] pub item at sps/src/cli.rs:44 (t=1021, 11 atoms) |
| 2.1 | 1476 | 430 | -1046 | 1.00 | early | Per-crate src/ listings | fs-only |
| 2.6 | 2517 | 6823 | +4306 | 0.80 | late | sps-common — dependency module exports | [scheduled bbox exact=8/10] mod/use plumbing in sps-common/src/dependency/mod.rs (t=6823, 8 atoms) |
| 2.8 | 2759 | 1695 | -1064 | 1.00 | early | sps-core/install/cask sub-listing | fs-only |
| 2.13 | 3564 | 1068 | -2496 | 1.00 | early | sps-core — pipeline / check / uninstall / upgrade / utils sub-listings | fs-only |
| 2.15 | 3657 | 7291 | +3634 | 1.00 | late | sps bin — pipeline & cli sub-listings | fs-only |
| 3.1 | 4273 | 1620 | -2653 | 0.89 | early | Config struct fields | [scheduled bbox exact=8/9] pub item at sps-common/src/config.rs:15 (t=1620, 8 atoms) |
| 4.1 | 7436 | 4233 | -3203 | 0.82 | early | Pipeline shared types — JobAction / PlannedJob / WorkerJob / PipelineEvent header | [scheduled bbox exact=11/34] pub item at sps-common/src/pipeline.rs:20 (t=4168, 11 atoms) |
| 4.2 | 7933 | 8294 | +361 | 0.83 | aligned | JobProcessingState + DownloadOutcome + PlannedOperations | [scheduled bbox exact=18/36] pub item at sps-common/src/pipeline.rs:163 (t=8294, 18 atoms) |
| 4.3 | 8149 | 5385 | -2764 | 0.81 | early | CommandType + PipelineFlags + run_pipeline signature | [scheduled bbox exact=6/21] pub-item names surface in sps/src/pipeline/runner.rs (t=5253, 8 atoms) |
| 4.5 | 8771 | 8875 | +104 | 0.81 | aligned | DependencyResolver — strategy + status + ResolvedDependency | [scheduled bbox exact=8/32] pub-item names surface in sps-common/src/dependency/resolver.rs (t=8537, 10 atoms) |
| 4.8 | 9433 | 8076 | -1357 | 0.80 | aligned | core worker pool — start_worker_pool_manager | [scheduled bbox exact=9/10] pub item at sps-core/src/pipeline/engine.rs:16 (t=8076, 9 atoms) |
| 4.12 | 9983 | 1407 | -8576 | 1.00 | early | Uninstall + Upgrade entry-point fn names | [scheduled bbox exact=2/6] pub-item names surface in sps-core/src/uninstall/cask.rs (t=1407, 3 atoms) |

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
| 3147 | — | — | — | +46 more rows |

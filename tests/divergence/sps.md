scores: Sim=0.401 Reached=16/49 Early=5 Late=2 Partial=6 Missing=27 Used=9937/10000

## Verdict

Verdict: wrong-slice bound
Likely primary lever: split walker batches to match NS semantic slices
Evidence: 8 ranking-recoverable (w×gap=1.63), 23 wrong-slice/granularity (w×gap=4.10), 0 no-discovered (w×gap=0.00)
Secondary intervention: free final budget for 7 too-expensive candidates
Loss reasons: 1 predecessor-gated, 7 too-expensive, 0 discovered-unscheduled
Top rows: 1.5, 1.6, 1.8, 1.9, 1.2, ...
Note: likely lever is heuristic; verify `Sim` moves, not just bucket counts.

## Top opportunities

_`w(t)×gap` is a non-additive priority score: Σ exp(-exp_t/τ) × (1 - credit) per row, τ=2000. Same time weighting and credit gap as Sim, but rows can overlap between opportunities — sums across rows are an upper bound on Sim impact, not an additive estimate._

| intervention | rows | w(t)×gap | bands ≤3k/≤6k/total | evidence | top row ids |
|:-------------|-----:|---------:|:----------------------|:---------|:------------|
| split wrong-slice walker batches | 23 | 4.10 | 10/12/23 | nearby candidates have low exact atom overlap | 1.5, 1.6, 1.8, 1.9, 1.2, ... |
| free final budget / demote late waste | 7 | 1.45 | 2/5/7 | high-overlap candidates exceed final remaining budget, exact total=125/136 | 1.7, 1.10, 3.2, 3.4, 3.5, ... |
| promote listing of 'sps-core/src/install/cask/artifacts' | 1 | 0.18 | 0/1/1 | 0 files, exact total=45/47 | 2.10 |

Tiers: 1=3/10 reached, 1 partial, 6 missing, avg=0.36; 2=7/16 reached, 3 partial, 6 missing, avg=0.61; 3=1/11 reached, 0 partial, 10 missing, avg=0.25; 4=5/12 reached, 2 partial, 5 missing, avg=0.47

## Diagnosis rollup

| diagnosis | rows | missing | partial | timing | likely lever |
|:----------|-----:|--------:|--------:|-------:|:-------------|
| ranking-recoverable | 8 | 8 | 0 | 0 | value/ranking |
| wrong-slice / granularity | 23 | 17 | 6 | 0 | walker granularity / wrong slice |
| fs/listing | 2 | 2 | 0 | 0 | filesystem/listing value |
| timing-only | 11 | 0 | 0 | 11 | usually no code change |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | w(t)×gap | likely lever |
|:------------|-----:|---------:|:-------------|
| predecessor not scheduled | 1 | 0.18 | promote predecessor |
| too expensive at final margin | 7 | 1.45 | free final budget |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=28, unscheduled bbox=11, fs-only=5

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | aligned | low | 3 |
| scheduled bbox | aligned | high | 1 |
| scheduled bbox | early | low | 1 |
| scheduled bbox | early | high | 2 |
| scheduled bbox | late | high | 1 |
| scheduled bbox | missing | none | 2 |
| scheduled bbox | missing | low | 12 |
| scheduled bbox | partial | low | 6 |
| unscheduled bbox | missing | low | 6 |
| unscheduled bbox | missing | high | 3 |
| unscheduled bbox | missing | full | 2 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.7 | 744 | — | — | 0.00 | missing | main.rs — Config::load failure hint | [unscheduled bbox exact=5/5] entry item body at sps/src/main.rs:57 body 72 (5 atoms, too expensive at final margin) |
| 1.10 | 1377 | — | — | 0.00 | missing | main.rs — pipeline-aware error printing on failure | [unscheduled bbox exact=20/21] entry item body at sps/src/main.rs:57 body 167 (20 atoms, too expensive at final margin) |
| 2.10 | 3431 | — | — | 0.00 | missing | Cask artifacts mod.rs — re-export wall | [unscheduled bbox exact=45/47] mod/use plumbing in sps-core/src/install/cask/artifacts/mod.rs (45 atoms, predecessor not scheduled: listing of 'sps-core/src/install/cask/artifacts') |
| 3.2 | 4558 | — | — | 0.00 | missing | Config path-method roster | [unscheduled bbox exact=25/25] impl method sigs in sps-common/src/config.rs (54 atoms, too expensive at final margin) |
| 3.4 | 5428 | — | — | 0.06 | missing | SpsError variant signatures | [scheduled bbox exact=2/31] pub-item names surface in sps-common/src/error.rs (t=801, 3 atoms); better unscheduled exact=29/31: pub item at sps-common/src/error.rs:6 (56 atoms, too expensive at final margin) |
| 3.5 | 5758 | — | — | 0.15 | missing | InstallTargetIdentifier + Formula struct fields | [scheduled bbox exact=4/34] pub item at sps-common/src/model/mod.rs:17 (t=3735, 4 atoms); better unscheduled exact=28/34: pub item at sps-common/src/model/formula.rs:57 (28 atoms, too expensive at final margin) |
| 3.7 | 6269 | — | — | 0.10 | missing | InstalledArtifact variants | [scheduled bbox exact=1/10] pub-item names surface in sps-common/src/model/artifact.rs (t=3756, 2 atoms); better unscheduled exact=9/10: pub item at sps-common/src/model/artifact.rs:9 (30 atoms, too expensive at final margin) |
| 4.11 | 9912 | — | — | 0.00 | missing | API entry-points (sps-net::api) | [unscheduled bbox exact=9/10] pub-item names surface in sps-net/src/api.rs (17 atoms, too expensive at final margin) |

### wrong-slice / granularity

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 144 | — | — | 0.57 | partial | README lede — what sps is | [scheduled bbox exact=3/7] README.md section #0 (t=3662, 3 atoms) |
| 1.5 | 503 | — | — | 0.00 | missing | CLI per-subcommand module list | [unscheduled bbox exact=14/19] mod/use plumbing in sps/src/cli.rs (14 atoms, too expensive at final margin) |
| 1.6 | 678 | — | — | 0.06 | missing | main.rs — Tokio entry signature + Init early-out | [scheduled bbox exact=2/16] entry item at sps/src/main.rs:57 (t=227, 2 atoms); better unscheduled exact=11/16: entry item body at sps/src/main.rs:57 body 60 (11 atoms, too expensive at final margin) |
| 1.8 | 936 | — | — | 0.00 | missing | main.rs — auto-update gate | [unscheduled bbox exact=11/16] entry item body at sps/src/main.rs:57 body 146 (11 atoms, too expensive at final margin) |
| 1.9 | 1105 | — | — | 0.00 | missing | main.rs — Cache + Command::run dispatch | [unscheduled bbox exact=7/13] entry item body at sps/src/main.rs:57 body 159 (7 atoms, too expensive at final margin) |
| 2.2 | 1651 | — | — | 0.71 | partial | sps-common lib.rs — module tree + re-exports | [scheduled bbox exact=12/17] mod/use plumbing in sps-common/src/lib.rs (t=1030, 12 atoms) |
| 2.3 | 1859 | — | — | 0.40 | missing | sps-core lib.rs — module tree | [scheduled bbox exact=8/20] mod/use plumbing in sps-core/src/lib.rs (t=371, 8 atoms) |
| 2.4 | 2220 | — | — | 0.79 | partial | sps-net lib.rs — re-exports + dependency on sps-common | [scheduled bbox exact=23/29] mod/use plumbing in sps-net/src/lib.rs (t=6973, 23 atoms) |
| 2.5 | 2402 | — | — | 0.65 | partial | sps-common — model module exports | [scheduled bbox exact=9/20] mod/use plumbing in sps-common/src/model/mod.rs (t=3856, 9 atoms) |
| 2.7 | 2744 | — | — | 0.45 | missing | sps-core/install — module roster | [scheduled bbox exact=7/22] mod/use plumbing in sps-core/src/install/mod.rs (t=1793, 7 atoms) |
| 2.16 | 4161 | — | — | 0.00 | missing | Per-crate Cargo manifest signatures (name + deps highlights) | [unscheduled bbox exact=11/38] [package] in sps/Cargo.toml (11 atoms, too expensive at final margin) |
| 3.3 | 5074 | — | — | 0.00 | missing | Config — sps_root resolution + cellar/cask path bodies | [scheduled bbox exact=0/36] pub item at sps-common/src/config.rs:15 (t=2143, 8 atoms); better unscheduled exact=2/36: impl method sigs in sps-common/src/config.rs (3 atoms, too expensive at final margin) |
| 3.6 | 6151 | — | — | 0.00 | missing | Cask struct fields | [unscheduled bbox exact=28/39] pub item at sps-common/src/model/cask.rs:116 (28 atoms, predecessor not scheduled: pub-item names surface in sps-common/src/model/cask.rs) |
| 3.8 | 6543 | — | — | 0.48 | missing | Dependency + DependencyTag bitflags | [scheduled bbox exact=5/21] pub item at sps-common/src/dependency/definition.rs:53 (t=1531, 5 atoms) |
| 3.9 | 6739 | — | — | 0.31 | missing | Requirement enum (macOS / Xcode / Other) | [scheduled bbox exact=5/16] pub item at sps-common/src/dependency/requirement.rs:7 (t=1085, 5 atoms) |
| 3.10 | 6925 | — | — | 0.29 | missing | Cache API surface | [scheduled bbox exact=4/17] pub item at sps-common/src/cache.rs:15 (t=768, 4 atoms); better unscheduled exact=9/17: impl method sigs in sps-common/src/cache.rs (16 atoms, too expensive at final margin) |
| 3.11 | 7098 | — | — | 0.47 | missing | InstalledKeg + KegRegistry signatures | [scheduled bbox exact=5/17] pub item at sps-common/src/keg.rs:13 (t=691, 5 atoms); better unscheduled exact=7/17: impl method sigs in sps-common/src/keg.rs (14 atoms, too expensive at final margin) |
| 4.4 | 8457 | — | — | 0.03 | missing | OperationPlanner + plan_operations signature | [scheduled bbox exact=2/31] pub-item names surface in sps/src/pipeline/planner.rs (t=4002, 2 atoms); better unscheduled exact=14/31: impl method sigs in sps/src/pipeline/planner.rs (34 atoms, too expensive at final margin) |
| 4.5 | 8771 | — | — | 0.56 | partial | DependencyResolver — strategy + status + ResolvedDependency | [scheduled bbox exact=8/32] pub-item names surface in sps-common/src/dependency/resolver.rs (t=9827, 10 atoms); better unscheduled exact=9/32: pub item at sps-common/src/dependency/resolver.rs:42 (9 atoms, too expensive at final margin) |
| 4.6 | 9097 | — | — | 0.08 | missing | DependencyResolver — ResolutionContext + ResolvedGraph | [scheduled bbox exact=0/27] pub item at sps-common/src/dependency/resolver.rs:53 (t=9937, 8 atoms); better unscheduled exact=13/27: pub item at sps-common/src/dependency/resolver.rs:27 (13 atoms, too expensive at final margin) |
| 4.7 | 9280 | — | — | 0.07 | missing | core worker entry — execute_sync_job | [scheduled bbox exact=2/14] pub-item names surface in sps-core/src/pipeline/worker.rs (t=529, 2 atoms); better unscheduled exact=7/14: pub item at sps-core/src/pipeline/worker.rs:21 (7 atoms, too expensive at final margin) |
| 4.9 | 9653 | — | — | 0.61 | partial | Install entry-points — install_bottle / build_from_source / install_cask signatures | [scheduled bbox exact=6/18] pub item at sps-core/src/install/cask/mod.rs:232 (t=2659, 6 atoms) |
| 4.10 | 9784 | — | — | 0.00 | missing | Bottle platform selection — get_bottle_for_platform | [unscheduled bbox exact=3/5] pub item body at sps-core/src/install/bottle/exec.rs:210 body 247 (3 atoms, predecessor not scheduled: pub item at sps-core/src/install/bottle/exec.rs:210) |

### fs/listing

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 2.9 | 2874 | — | — | 0.00 | missing | Cask artifact handler enumeration | fs-only |
| 2.12 | 3492 | — | — | 0.00 | missing | sps-core/build — compile sub-listing | fs-only |

### timing-only

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.4 | 327 | 1238 | +911 | 0.92 | late | CLI subcommand enum | [scheduled bbox exact=11/12] pub item at sps/src/cli.rs:44 (t=1238, 11 atoms) |
| 2.1 | 1476 | 564 | -912 | 1.00 | early | Per-crate src/ listings | fs-only |
| 2.6 | 2517 | 1628 | -889 | 0.80 | early | sps-common — dependency module exports | [scheduled bbox exact=8/10] mod/use plumbing in sps-common/src/dependency/mod.rs (t=1628, 8 atoms) |
| 2.13 | 3564 | 1358 | -2206 | 1.00 | early | sps-core — pipeline / check / uninstall / upgrade / utils sub-listings | fs-only |
| 2.15 | 3657 | 8441 | +4784 | 1.00 | late | sps bin — pipeline & cli sub-listings | fs-only |
| 3.1 | 4273 | 2143 | -2130 | 0.89 | early | Config struct fields | [scheduled bbox exact=8/9] pub item at sps-common/src/config.rs:15 (t=2143, 8 atoms) |
| 4.1 | 7436 | 5383 | -2053 | 0.82 | aligned | Pipeline shared types — JobAction / PlannedJob / WorkerJob / PipelineEvent header | [scheduled bbox exact=11/34] pub item at sps-common/src/pipeline.rs:20 (t=5318, 11 atoms) |
| 4.2 | 7933 | 9444 | +1511 | 0.83 | aligned | JobProcessingState + DownloadOutcome + PlannedOperations | [scheduled bbox exact=18/36] pub item at sps-common/src/pipeline.rs:163 (t=9444, 18 atoms) |
| 4.3 | 8149 | 6489 | -1660 | 0.81 | aligned | CommandType + PipelineFlags + run_pipeline signature | [scheduled bbox exact=6/21] pub-item names surface in sps/src/pipeline/runner.rs (t=6357, 8 atoms) |
| 4.8 | 9433 | 9226 | -207 | 0.80 | aligned | core worker pool — start_worker_pool_manager | [scheduled bbox exact=9/10] pub item at sps-core/src/pipeline/engine.rs:16 (t=9226, 9 atoms) |
| 4.12 | 9983 | 2007 | -7976 | 1.00 | early | Uninstall + Upgrade entry-point fn names | [scheduled bbox exact=2/6] pub-item names surface in sps-core/src/uninstall/cask.rs (t=2007, 3 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 4 | 279 | pub item at sps-net/src/oci.rs:<n> |
| 3 | 197 | pub item at sps-net/src/http.rs:<n> |
| 2 | 149 | README.md section #<n> |
| 2 | 147 | pub item at sps-common/src/model/formula.rs:<n> |
| 2 | 117 | pub item at sps/src/cli/search.rs:<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 503 | 0.95 | 530 | 3489 | README headline in README.md |
| 272 | 1.00 | 272 | 6109 | headings outline in CONTRIBUTING.md |
| 209 | 1.00 | 209 | 5029 | mod/use plumbing in sps/src/main.rs |
| 206 | 0.94 | 218 | 2436 | pub-item names surface in sps-core/src/install/cask/mod.rs |
| 152 | 1.00 | 152 | 5709 | mod/use plumbing in sps-core/src/install/bottle/mod.rs |
| 123 | 1.00 | 123 | 7321 | pub-item names surface in sps-net/src/oci.rs |
| 118 | 1.00 | 118 | 2817 | pub item at sps-core/src/install/cask/mod.rs:25 |
| 115 | 1.00 | 115 | 4448 | pub item body at sps-core/src/install/bottle/mod.rs:34 body 35 |
| 113 | 1.00 | 113 | 3602 | headings outline in README.md |
| 109 | 1.00 | 109 | 9668 | pub item at sps/src/cli/list.rs:16 |
| 2900 | — | — | — | +43 more rows |

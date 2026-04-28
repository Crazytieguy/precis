scores: Sim=0.305 Reached=8/55 Early=4 Late=3 Partial=7 Missing=40 Used=9981/10000

## Verdict

Verdict: coverage-gap bound
Likely primary lever: add walker candidates for no-discovered NS rows
Evidence: 6 ranking-recoverable (w×gap=0.91), 22 wrong-slice/granularity (w×gap=2.82), 9 no-discovered (w×gap=3.69)
Secondary intervention: free final budget for 4 too-expensive candidates
Loss reasons: 2 predecessor-gated, 4 too-expensive, 0 discovered-unscheduled
Top rows: 2.1, 2.2, 2.3, 2.4, 2.5, ...
Note: likely lever is heuristic; verify `Sim` moves, not just bucket counts.

## Top opportunities

_`w(t)×gap` is a non-additive priority score: Σ exp(-exp_t/τ) × (1 - credit) per row, τ=2000. Same time weighting and credit gap as Sim, but rows can overlap between opportunities — sums across rows are an upper bound on Sim impact, not an additive estimate._

| intervention | rows | w(t)×gap | bands ≤3k/≤6k/total | evidence | top row ids |
|:-------------|-----:|---------:|:----------------------|:---------|:------------|
| add walker candidates for no-discovered rows | 9 | 3.69 | 5/6/9 | NS rows have no discovered line candidate | 2.1, 2.2, 2.3, 2.4, 2.5, ... |
| split wrong-slice walker batches | 22 | 2.82 | 7/15/22 | nearby candidates have low exact atom overlap | 3.2, 1.1, 3.5, 4.1, 3.6, ... |
| free final budget / demote late waste | 4 | 0.84 | 2/2/4 | high-overlap candidates exceed final remaining budget, exact total=41/47 | 3.3, 3.4, 7.6, 7.7 |
| promote headings outline in docs/architecture/query-engine.md | 1 | 0.05 | 0/1/1 | 0 files, exact total=18/19 | 6.6 |
| promote headings outline in docs/CHANGE_GUIDE.md | 1 | 0.02 | 0/0/1 | 0 files, exact total=10/12 | 8.5 |

Tiers: 1=4/6 reached, 1 partial, 1 missing, avg=0.77; 2=0/5 reached, 0 partial, 5 missing, avg=0.00; 3=0/6 reached, 1 partial, 5 missing, avg=0.20; 4=0/2 reached, 0 partial, 2 missing, avg=0.03; 5=1/8 reached, 2 partial, 5 missing, avg=0.33; 6=0/6 reached, 0 partial, 6 missing, avg=0.00; 7=2/8 reached, 2 partial, 4 missing, avg=0.44; 8=0/5 reached, 0 partial, 5 missing, avg=0.11; 9=1/4 reached, 1 partial, 2 missing, avg=0.50; 10=0/5 reached, 0 partial, 5 missing, avg=0.06

## Diagnosis rollup

| diagnosis | rows | missing | partial | timing | likely lever |
|:----------|-----:|--------:|--------:|-------:|:-------------|
| ranking-recoverable | 6 | 6 | 0 | 0 | value/ranking |
| wrong-slice / granularity | 22 | 15 | 7 | 0 | walker granularity / wrong slice |
| no discovered candidate | 9 | 9 | 0 | 0 | walker coverage or predecessor-gated emit |
| fs/listing | 10 | 10 | 0 | 0 | filesystem/listing value |
| timing-only | 7 | 0 | 0 | 7 | usually no code change |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | w(t)×gap | likely lever |
|:------------|-----:|---------:|:-------------|
| predecessor not scheduled | 2 | 0.07 | promote predecessor |
| too expensive at final margin | 4 | 0.84 | free final budget |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=23, unscheduled bbox=3, scheduled same-file=4, fs-only=15, no discovered candidate=9

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | late | low | 1 |
| scheduled bbox | late | full | 1 |
| scheduled bbox | missing | low | 14 |
| scheduled bbox | partial | low | 7 |
| unscheduled bbox | missing | low | 1 |
| unscheduled bbox | missing | high | 2 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 3.3 | 1641 | — | — | 0.06 | missing | ARCHITECTURE.md: toasty crate role | [scheduled bbox exact=2/16] headings outline in docs/ARCHITECTURE.md (t=3323, 2 atoms); better unscheduled exact=13/16: docs/ARCHITECTURE.md section #2 (13 atoms, too expensive at final margin) |
| 3.4 | 1823 | — | — | 0.08 | missing | ARCHITECTURE.md: toasty-core crate role | [scheduled bbox exact=2/12] headings outline in docs/ARCHITECTURE.md (t=3323, 2 atoms); better unscheduled exact=11/12: docs/ARCHITECTURE.md section #3 (11 atoms, too expensive at final margin) |
| 6.6 | 5815 | — | — | 0.00 | missing | Query engine doc: 5-phase compilation pipeline diagram | [unscheduled bbox exact=18/19] docs/architecture/query-engine.md section #3 (18 atoms, predecessor not scheduled: headings outline in docs/architecture/query-engine.md) |
| 7.6 | 6768 | — | — | 0.12 | missing | Driver trait: 6 method signatures | [scheduled bbox exact=1/8] pub-item names surface in crates/toasty-core/src/driver.rs (t=2958, 2 atoms); better unscheduled exact=7/8: pub item at crates/toasty-core/src/driver.rs:18 (19 atoms, too expensive at final margin) |
| 7.7 | 6931 | — | — | 0.09 | missing | Connection trait: 4 method signatures | [scheduled bbox exact=1/11] pub-item names surface in crates/toasty-core/src/driver.rs (t=2958, 2 atoms); better unscheduled exact=10/11: pub item at crates/toasty-core/src/driver.rs:45 (15 atoms, too expensive at final margin) |
| 8.5 | 7917 | — | — | 0.00 | missing | docs/CHANGE_GUIDE.md: where-changes-go matrix | [unscheduled bbox exact=10/12] docs/CHANGE_GUIDE.md section #1 (10 atoms, predecessor not scheduled: headings outline in docs/CHANGE_GUIDE.md) |

### wrong-slice / granularity

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.1 | 55 | — | — | 0.60 | partial | Repo lede: title + status | [scheduled bbox exact=3/5] README headline in README.md (t=93, 3 atoms) |
| 3.1 | 1145 | — | — | 0.71 | partial | ARCHITECTURE.md: opening + Crates header | [scheduled bbox exact=5/7] headings outline in docs/ARCHITECTURE.md (t=3323, 5 atoms) |
| 3.2 | 1404 | — | — | 0.05 | missing | Engine pipeline phases (canonical phase list) | [scheduled bbox exact=2/20] pub-item names surface in crates/toasty/src/engine.rs (t=5641, 2 atoms); better unscheduled exact=12/20: pub-item doc lede at crates/toasty/src/engine.rs:38 (12 atoms, predecessor not scheduled: pub item at crates/toasty/src/engine.rs:38) |
| 3.5 | 2115 | — | — | 0.09 | missing | ARCHITECTURE.md: codegen + drivers | [scheduled bbox exact=4/23] headings outline in docs/ARCHITECTURE.md (t=3323, 4 atoms); better unscheduled exact=13/23: docs/ARCHITECTURE.md section #4 (13 atoms, too expensive at final margin) |
| 3.6 | 2306 | — | — | 0.20 | missing | ARCHITECTURE.md: toasty-sql + further reading | [scheduled bbox exact=4/15] headings outline in docs/ARCHITECTURE.md (t=3323, 4 atoms); better unscheduled exact=9/15: docs/ARCHITECTURE.md section #6 (9 atoms, too expensive at final margin) |
| 4.1 | 2598 | — | — | 0.00 | missing | toasty/src/lib.rs: public re-exports | [unscheduled bbox exact=22/35] mod/use plumbing in crates/toasty/src/lib.rs (24 atoms, too expensive at final margin) |
| 4.2 | 2901 | — | — | 0.07 | missing | Db: struct + every public fn signature | [scheduled bbox exact=1/15] pub item at crates/toasty/src/db.rs:18 (t=8866, 10 atoms) |
| 5.1 | 3247 | — | — | 0.00 | missing | toasty/src/stmt.rs: typed Statement<M> wrapper | [scheduled same-file] pub item at crates/toasty/src/stmt.rs:51 (t=5735, 4 atoms) |
| 5.4 | 3724 | — | — | 0.50 | partial | Cursor<M>: stream API | [scheduled bbox exact=5/14] pub item at crates/toasty/src/cursor.rs:6 (t=5833, 5 atoms) |
| 5.5 | 3967 | — | — | 0.58 | partial | Page<M>: pagination return shape | [scheduled bbox exact=10/19] pub item at crates/toasty/src/page.rs:10 (t=9379, 10 atoms) |
| 5.6 | 4309 | — | — | 0.43 | missing | Relation trait: HasMany / BelongsTo / HasOne / Option | [scheduled bbox exact=18/42] pub item at crates/toasty/src/relation.rs:16 (t=9557, 18 atoms) |
| 5.8 | 4693 | — | — | 0.30 | missing | HasMany<T>: load + get + is_unloaded + unload | [scheduled bbox exact=3/10] pub item at crates/toasty/src/relation/has_many.rs:8 (t=6323, 3 atoms) |
| 6.1 | 4879 | — | — | 0.00 | missing | Engine module: every phase mod declared | [scheduled same-file] pub-item names surface in crates/toasty/src/engine.rs (t=5641, 2 atoms) |
| 6.2 | 5328 | — | — | 0.00 | missing | Engine: exec entrypoint with full phase invocation | [scheduled same-file] pub-item names surface in crates/toasty/src/engine.rs (t=5641, 2 atoms) |
| 7.1 | 5974 | — | — | 0.67 | partial | toasty-core lib.rs | [scheduled bbox exact=9/18] mod/use plumbing in crates/toasty-core/src/lib.rs (t=3038, 9 atoms) |
| 7.5 | 6632 | — | — | 0.67 | partial | Statement enum: 4 variants | [scheduled bbox exact=10/15] pub item at crates/toasty-core/src/stmt.rs:253 (t=4008, 10 atoms) |
| 8.1 | 7159 | — | — | 0.11 | missing | toasty-macros: derive entrypoints | [scheduled bbox exact=4/18] pub-item names surface in crates/toasty-macros/src/lib.rs (t=1012, 4 atoms) |
| 8.2 | 7366 | — | — | 0.28 | missing | toasty-codegen lib.rs entrypoints | [scheduled bbox exact=4/18] pub-item names surface in crates/toasty-codegen/src/lib.rs (t=1494, 4 atoms) |
| 8.4 | 7723 | — | — | 0.18 | missing | CLAUDE.md: layer-to-crate decision tree | [scheduled bbox exact=4/22] headings outline in CLAUDE.md (t=3759, 4 atoms); better unscheduled exact=9/22: CLAUDE.md section #4 (9 atoms, too expensive at final margin) |
| 9.1 | 7984 | — | — | 0.75 | partial | toasty-sql lib.rs | [scheduled bbox exact=6/8] mod/use plumbing in crates/toasty-sql/src/lib.rs (t=2498, 6 atoms) |
| 9.3 | 8457 | — | — | 0.25 | missing | SQLite driver: enum + Driver impl signatures | [scheduled bbox exact=8/36] impl method sigs in crates/toasty-driver-sqlite/src/lib.rs (t=6862, 12 atoms) |
| 9.4 | 8694 | — | — | 0.00 | missing | Connect: URL scheme dispatch (sqlite/postgres/mysql/dynamodb) | [scheduled same-file] pub item at crates/toasty/src/db/connect.rs:14 (t=5880, 3 atoms) |

### no discovered candidate

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 2.1 | 444 | — | — | 0.00 | missing | Hello-toasty: User model | no discovered line candidate |
| 2.2 | 572 | — | — | 0.00 | missing | Hello-toasty: Todo model with belongs_to | no discovered line candidate |
| 2.3 | 729 | — | — | 0.00 | missing | Hello-toasty: Db::builder + connect | no discovered line candidate |
| 2.4 | 830 | — | — | 0.00 | missing | Hello-toasty: User::create + get_by_id + get_by_email | no discovered line candidate |
| 2.5 | 1071 | — | — | 0.00 | missing | Hello-toasty: relation use + delete + create_many + nested create | no discovered line candidate |
| 5.3 | 3556 | — | — | 0.00 | missing | Select<M>: every public method signature | no discovered line candidate |
| 10.3 | 9368 | — | — | 0.00 | missing | Per-driver test entry: sqlite.rs full | no discovered line candidate |
| 10.4 | 9633 | — | — | 0.00 | missing | Examples: composite-key model | no discovered line candidate |
| 10.5 | 9856 | — | — | 0.00 | missing | Examples: user-has-one-profile model | no discovered line candidate |

### fs/listing

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.5 | 282 | — | — | 0.00 | missing | Examples dir listing | fs-only |
| 5.2 | 3309 | — | — | 0.00 | missing | toasty/src/stmt/ dir listing | fs-only |
| 6.3 | 5431 | — | — | 0.00 | missing | engine/simplify/ dir listing | fs-only |
| 6.4 | 5539 | — | — | 0.00 | missing | engine/lower/ + engine/plan/ + engine/mir/ dir listings | fs-only |
| 6.5 | 5632 | — | — | 0.00 | missing | engine/exec/ + engine/eval/ + engine/index/ dir listings | fs-only |
| 7.3 | 6095 | — | — | 0.00 | missing | toasty-core schema/app + schema/db submodule listings | fs-only |
| 7.4 | 6504 | — | — | 0.00 | missing | toasty-core/src/stmt/ dir listing — full AST node surface | fs-only |
| 8.3 | 7463 | — | — | 0.00 | missing | codegen/expand/ + codegen/schema/ submodule listings | fs-only |
| 10.1 | 8797 | — | — | 0.00 | missing | Workspace tests/ crate dir listing | fs-only |
| 10.2 | 9040 | — | — | 0.29 | missing | Driver integration suite: tests/ dir listing | fs-only |

### timing-only

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 131 | 257 | +126 | 1.00 | late | Repo lede: SQL+NoSQL claim | [scheduled bbox exact=4/4] README.md section #0 (t=257, 4 atoms) |
| 1.3 | 176 | 45 | -131 | 1.00 | early | Top-level repo file listing | fs-only |
| 1.4 | 261 | 848 | +587 | 1.00 | late | Crates dir listing (12 crates) | fs-only |
| 5.7 | 4579 | 9062 | +4483 | 0.81 | late | Model trait + Register trait | [scheduled bbox exact=14/26] pub item at crates/toasty/src/model.rs:41 (t=9062, 14 atoms) |
| 7.2 | 6010 | 2783 | -3227 | 1.00 | early | toasty-core src dir listing | fs-only |
| 7.8 | 6946 | 3053 | -3893 | 1.00 | early | toasty-core/src/driver/ submodule listing | fs-only |
| 9.2 | 8058 | 1836 | -6222 | 1.00 | early | toasty-sql + driver-crate src dir listings | fs-only |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 3 | 235 | pub item at crates/toasty-driver-integration-suite-macros/src/parse.rs:<n> |
| 2 | 144 | pub item at crates/toasty-driver-integration-suite/src/logging_driver.rs:<n> |
| 2 | 141 | README.md section #<n> |
| 2 | 108 | pub item at crates/toasty-driver-integration-suite/src/helpers.rs:<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 513 | 1.00 | 513 | 8394 | impl method sigs in crates/toasty-driver-dynamodb/src/lib.rs |
| 483 | 1.00 | 483 | 7368 | impl method sigs in crates/toasty-driver-postgresql/src/lib.rs |
| 414 | 1.00 | 414 | 5359 | impl method sigs in crates/toasty-driver-mysql/src/lib.rs |
| 398 | 0.84 | 473 | 6862 | impl method sigs in crates/toasty-driver-sqlite/src/lib.rs |
| 323 | 1.00 | 323 | 682 | [package] in Cargo.toml |
| 225 | 0.88 | 255 | 3759 | headings outline in CLAUDE.md |
| 177 | 1.00 | 177 | 2062 | impl method sigs in crates/toasty-cli/src/lib.rs |
| 173 | 1.00 | 173 | 9981 | mod/use plumbing in crates/toasty-driver-integration-suite/src/lib.rs |
| 172 | 1.00 | 172 | 9808 | mod/use plumbing in crates/toasty-driver-mysql/src/lib.rs |
| 154 | 1.00 | 154 | 8735 | pub item at crates/std-util/src/slice.rs:4 |
| 2369 | — | — | — | +32 more rows |

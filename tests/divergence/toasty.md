scores: Sim=0.306 Reached=7/55 Early=4 Late=2 Partial=5 Missing=43 Used=9933/10000

## Verdict

Verdict: coverage-gap bound
Likely primary lever: add walker candidates for no-discovered NS rows
Evidence: 6 ranking-recoverable (w×gap=0.91), 11 wrong-slice/granularity (w×gap=1.22), 21 no-discovered (w×gap=5.66)
Secondary intervention: free final budget for 4 too-expensive candidates
Loss reasons: 2 predecessor-gated, 4 too-expensive, 0 discovered-unscheduled
Top rows: 2.1, 2.2, 2.3, 2.4, 2.5, ...
Note: likely lever is heuristic; verify `Sim` moves, not just bucket counts.

## Top opportunities

_`w(t)×gap` is a non-additive priority score: Σ exp(-exp_t/τ) × (1 - credit) per row, τ=2000. Same time weighting and credit gap as Sim, but rows can overlap between opportunities — sums across rows are an upper bound on Sim impact, not an additive estimate._

| intervention | rows | w(t)×gap | bands ≤3k/≤6k/total | evidence | top row ids |
|:-------------|-----:|---------:|:----------------------|:---------|:------------|
| add walker candidates for no-discovered rows | 21 | 5.66 | 8/17/21 | NS rows have no discovered line candidate | 2.1, 2.2, 2.3, 2.4, 2.5, ... |
| split wrong-slice walker batches | 11 | 1.22 | 4/5/11 | nearby candidates have low exact atom overlap | 1.1, 3.5, 3.6, 3.1, 8.1, ... |
| free final budget / demote late waste | 4 | 0.84 | 2/2/4 | high-overlap candidates exceed final remaining budget, exact total=41/47 | 3.3, 3.4, 7.6, 7.7 |
| promote headings outline in docs/architecture/query-engine.md | 1 | 0.05 | 0/1/1 | 0 files, exact total=18/19 | 6.6 |
| promote headings outline in docs/CHANGE_GUIDE.md | 1 | 0.02 | 0/0/1 | 0 files, exact total=10/12 | 8.5 |

Tiers: 1=4/6 reached, 1 partial, 1 missing, avg=0.77; 2=0/5 reached, 0 partial, 5 missing, avg=0.00; 3=0/6 reached, 1 partial, 5 missing, avg=0.19; 4=0/2 reached, 0 partial, 2 missing, avg=0.00; 5=0/8 reached, 0 partial, 8 missing, avg=0.00; 6=0/6 reached, 0 partial, 6 missing, avg=0.00; 7=2/8 reached, 2 partial, 4 missing, avg=0.44; 8=0/5 reached, 0 partial, 5 missing, avg=0.11; 9=1/4 reached, 1 partial, 2 missing, avg=0.50; 10=0/5 reached, 0 partial, 5 missing, avg=0.06

## Diagnosis rollup

| diagnosis | rows | missing | partial | timing | likely lever |
|:----------|-----:|--------:|--------:|-------:|:-------------|
| ranking-recoverable | 6 | 6 | 0 | 0 | value/ranking |
| wrong-slice / granularity | 11 | 6 | 5 | 0 | walker granularity / wrong slice |
| no discovered candidate | 21 | 21 | 0 | 0 | walker coverage or predecessor-gated emit |
| fs/listing | 10 | 10 | 0 | 0 | filesystem/listing value |
| timing-only | 6 | 0 | 0 | 6 | usually no code change |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | w(t)×gap | likely lever |
|:------------|-----:|---------:|:-------------|
| predecessor not scheduled | 2 | 0.07 | promote predecessor |
| too expensive at final margin | 4 | 0.84 | free final budget |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=16, unscheduled bbox=2, fs-only=15, no discovered candidate=21

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | late | full | 1 |
| scheduled bbox | missing | low | 10 |
| scheduled bbox | partial | low | 5 |
| unscheduled bbox | missing | high | 2 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 3.3 | 1641 | — | — | 0.06 | missing | ARCHITECTURE.md: toasty crate role | [scheduled bbox exact=2/16] headings outline in docs/ARCHITECTURE.md (t=2941, 2 atoms); better unscheduled exact=13/16: docs/ARCHITECTURE.md section #2 (13 atoms, too expensive at final margin) |
| 3.4 | 1823 | — | — | 0.08 | missing | ARCHITECTURE.md: toasty-core crate role | [scheduled bbox exact=2/12] headings outline in docs/ARCHITECTURE.md (t=2941, 2 atoms); better unscheduled exact=11/12: docs/ARCHITECTURE.md section #3 (11 atoms, too expensive at final margin) |
| 6.6 | 5815 | — | — | 0.00 | missing | Query engine doc: 5-phase compilation pipeline diagram | [unscheduled bbox exact=18/19] docs/architecture/query-engine.md section #3 (18 atoms, predecessor not scheduled: headings outline in docs/architecture/query-engine.md) |
| 7.6 | 6768 | — | — | 0.12 | missing | Driver trait: 6 method signatures | [scheduled bbox exact=1/8] pub-item names surface in crates/toasty-core/src/driver.rs (t=4226, 2 atoms); better unscheduled exact=7/8: pub item at crates/toasty-core/src/driver.rs:18 (19 atoms, too expensive at final margin) |
| 7.7 | 6931 | — | — | 0.09 | missing | Connection trait: 4 method signatures | [scheduled bbox exact=1/11] pub-item names surface in crates/toasty-core/src/driver.rs (t=4226, 2 atoms); better unscheduled exact=10/11: pub item at crates/toasty-core/src/driver.rs:45 (15 atoms, too expensive at final margin) |
| 8.5 | 7917 | — | — | 0.00 | missing | docs/CHANGE_GUIDE.md: where-changes-go matrix | [unscheduled bbox exact=10/12] docs/CHANGE_GUIDE.md section #1 (10 atoms, predecessor not scheduled: headings outline in docs/CHANGE_GUIDE.md) |

### wrong-slice / granularity

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.1 | 55 | — | — | 0.60 | partial | Repo lede: title + status | [scheduled bbox exact=3/5] README headline in README.md (t=93, 3 atoms) |
| 3.1 | 1145 | — | — | 0.71 | partial | ARCHITECTURE.md: opening + Crates header | [scheduled bbox exact=5/7] headings outline in docs/ARCHITECTURE.md (t=2941, 5 atoms) |
| 3.5 | 2115 | — | — | 0.09 | missing | ARCHITECTURE.md: codegen + drivers | [scheduled bbox exact=4/23] headings outline in docs/ARCHITECTURE.md (t=2941, 4 atoms); better unscheduled exact=13/23: docs/ARCHITECTURE.md section #4 (13 atoms, too expensive at final margin) |
| 3.6 | 2306 | — | — | 0.20 | missing | ARCHITECTURE.md: toasty-sql + further reading | [scheduled bbox exact=4/15] headings outline in docs/ARCHITECTURE.md (t=2941, 4 atoms); better unscheduled exact=9/15: docs/ARCHITECTURE.md section #6 (9 atoms, too expensive at final margin) |
| 7.1 | 5974 | — | — | 0.67 | partial | toasty-core lib.rs | [scheduled bbox exact=9/18] mod/use plumbing in crates/toasty-core/src/lib.rs (t=4306, 9 atoms) |
| 7.5 | 6632 | — | — | 0.67 | partial | Statement enum: 4 variants | [scheduled bbox exact=10/15] pub item at crates/toasty-core/src/stmt.rs:253 (t=4500, 10 atoms) |
| 8.1 | 7159 | — | — | 0.11 | missing | toasty-macros: derive entrypoints | [scheduled bbox exact=4/18] pub-item names surface in crates/toasty-macros/src/lib.rs (t=1012, 4 atoms) |
| 8.2 | 7366 | — | — | 0.28 | missing | toasty-codegen lib.rs entrypoints | [scheduled bbox exact=4/18] pub-item names surface in crates/toasty-codegen/src/lib.rs (t=1512, 4 atoms) |
| 8.4 | 7723 | — | — | 0.18 | missing | CLAUDE.md: layer-to-crate decision tree | [scheduled bbox exact=4/22] headings outline in CLAUDE.md (t=3377, 4 atoms); better unscheduled exact=9/22: CLAUDE.md section #4 (9 atoms, too expensive at final margin) |
| 9.1 | 7984 | — | — | 0.75 | partial | toasty-sql lib.rs | [scheduled bbox exact=6/8] mod/use plumbing in crates/toasty-sql/src/lib.rs (t=2326, 6 atoms) |
| 9.3 | 8457 | — | — | 0.25 | missing | SQLite driver: enum + Driver impl signatures | [scheduled bbox exact=8/36] impl method sigs in crates/toasty-driver-sqlite/src/lib.rs (t=5207, 12 atoms) |

### no discovered candidate

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 2.1 | 444 | — | — | 0.00 | missing | Hello-toasty: User model | no discovered line candidate |
| 2.2 | 572 | — | — | 0.00 | missing | Hello-toasty: Todo model with belongs_to | no discovered line candidate |
| 2.3 | 729 | — | — | 0.00 | missing | Hello-toasty: Db::builder + connect | no discovered line candidate |
| 2.4 | 830 | — | — | 0.00 | missing | Hello-toasty: User::create + get_by_id + get_by_email | no discovered line candidate |
| 2.5 | 1071 | — | — | 0.00 | missing | Hello-toasty: relation use + delete + create_many + nested create | no discovered line candidate |
| 3.2 | 1404 | — | — | 0.00 | missing | Engine pipeline phases (canonical phase list) | no discovered line candidate |
| 4.1 | 2598 | — | — | 0.00 | missing | toasty/src/lib.rs: public re-exports | no discovered line candidate |
| 4.2 | 2901 | — | — | 0.00 | missing | Db: struct + every public fn signature | no discovered line candidate |
| 5.1 | 3247 | — | — | 0.00 | missing | toasty/src/stmt.rs: typed Statement<M> wrapper | no discovered line candidate |
| 5.3 | 3556 | — | — | 0.00 | missing | Select<M>: every public method signature | no discovered line candidate |
| 5.4 | 3724 | — | — | 0.00 | missing | Cursor<M>: stream API | no discovered line candidate |
| 5.5 | 3967 | — | — | 0.00 | missing | Page<M>: pagination return shape | no discovered line candidate |
| 5.6 | 4309 | — | — | 0.00 | missing | Relation trait: HasMany / BelongsTo / HasOne / Option | no discovered line candidate |
| 5.7 | 4579 | — | — | 0.00 | missing | Model trait + Register trait | no discovered line candidate |
| 5.8 | 4693 | — | — | 0.00 | missing | HasMany<T>: load + get + is_unloaded + unload | no discovered line candidate |
| 6.1 | 4879 | — | — | 0.00 | missing | Engine module: every phase mod declared | no discovered line candidate |
| 6.2 | 5328 | — | — | 0.00 | missing | Engine: exec entrypoint with full phase invocation | no discovered line candidate |
| 9.4 | 8694 | — | — | 0.00 | missing | Connect: URL scheme dispatch (sqlite/postgres/mysql/dynamodb) | no discovered line candidate |
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
| 7.2 | 6010 | 4051 | -1959 | 1.00 | early | toasty-core src dir listing | fs-only |
| 7.8 | 6946 | 4321 | -2625 | 1.00 | early | toasty-core/src/driver/ submodule listing | fs-only |
| 9.2 | 8058 | 2220 | -5838 | 1.00 | early | toasty-sql + driver-crate src dir listings | fs-only |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 4 | 369 | pub item at crates/toasty-driver-integration-suite-macros/src/parse.rs:<n> |
| 2 | 144 | pub item at crates/toasty-driver-integration-suite/src/logging_driver.rs:<n> |
| 2 | 141 | README.md section #<n> |
| 2 | 108 | pub item at crates/toasty-driver-integration-suite/src/helpers.rs:<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 513 | 1.00 | 513 | 6721 | impl method sigs in crates/toasty-driver-dynamodb/src/lib.rs |
| 483 | 1.00 | 483 | 5713 | impl method sigs in crates/toasty-driver-postgresql/src/lib.rs |
| 414 | 1.00 | 414 | 3985 | impl method sigs in crates/toasty-driver-mysql/src/lib.rs |
| 398 | 0.84 | 473 | 5207 | impl method sigs in crates/toasty-driver-sqlite/src/lib.rs |
| 323 | 1.00 | 323 | 682 | [package] in Cargo.toml |
| 225 | 0.88 | 255 | 3377 | headings outline in CLAUDE.md |
| 186 | 1.00 | 186 | 9058 | mod/use plumbing in crates/toasty-driver-sqlite/src/lib.rs |
| 177 | 1.00 | 177 | 1870 | impl method sigs in crates/toasty-cli/src/lib.rs |
| 173 | 1.00 | 173 | 8549 | mod/use plumbing in crates/toasty-driver-integration-suite/src/lib.rs |
| 172 | 1.00 | 172 | 8376 | mod/use plumbing in crates/toasty-driver-mysql/src/lib.rs |
| 2991 | — | — | — | +38 more rows |

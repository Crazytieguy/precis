scores: Sim=0.342 Reached=7/55 Early=4 Late=3 Partial=11 Missing=37 Used=9934/10000

## Verdict

Verdict: wrong-slice bound
Likely primary lever: split walker batches to match NS semantic slices
Evidence: 11 ranking-recoverable (w×gap=3.16), 27 wrong-slice/granularity (w×gap=3.19), 0 no-discovered (w×gap=0.00)
Secondary intervention: free final budget for 9 too-expensive candidates
Loss reasons: 2 predecessor-gated, 9 too-expensive, 0 discovered-unscheduled
Top rows: 3.2, 1.1, 3.5, 3.6, 2.1, ...
Note: likely lever is heuristic; verify `Sim` moves, not just bucket counts.

## Top opportunities

_`w(t)×gap` is a non-additive priority score: Σ exp(-exp_t/τ) × (1 - credit) per row, τ=2000. Same time weighting and credit gap as Sim, but rows can overlap between opportunities — sums across rows are an upper bound on Sim impact, not an additive estimate._

| intervention | rows | w(t)×gap | bands ≤3k/≤6k/total | evidence | top row ids |
|:-------------|-----:|---------:|:----------------------|:---------|:------------|
| split wrong-slice walker batches | 27 | 3.19 | 8/17/27 | nearby candidates have low exact atom overlap | 3.2, 1.1, 3.5, 3.6, 2.1, ... |
| free final budget / demote late waste | 9 | 3.08 | 6/7/9 | high-overlap candidates exceed final remaining budget, exact total=109/123 | 2.3, 2.4, 2.5, 3.3, 3.4, ... |
| promote headings outline in docs/architecture/query-engine.md | 1 | 0.05 | 0/1/1 | 0 files, exact total=18/19 | 6.6 |
| promote headings outline in docs/CHANGE_GUIDE.md | 1 | 0.02 | 0/0/1 | 0 files, exact total=10/12 | 8.5 |

Tiers: 1=4/6 reached, 1 partial, 1 missing, avg=0.77; 2=0/5 reached, 2 partial, 3 missing, avg=0.28; 3=0/6 reached, 1 partial, 5 missing, avg=0.20; 4=0/2 reached, 1 partial, 1 missing, avg=0.33; 5=0/8 reached, 1 partial, 7 missing, avg=0.15; 6=0/6 reached, 0 partial, 6 missing, avg=0.00; 7=2/8 reached, 2 partial, 4 missing, avg=0.44; 8=0/5 reached, 0 partial, 5 missing, avg=0.11; 9=1/4 reached, 1 partial, 2 missing, avg=0.50; 10=0/5 reached, 2 partial, 3 missing, avg=0.32

## Diagnosis rollup

| diagnosis | rows | missing | partial | timing | likely lever |
|:----------|-----:|--------:|--------:|-------:|:-------------|
| ranking-recoverable | 11 | 11 | 0 | 0 | value/ranking |
| wrong-slice / granularity | 27 | 16 | 11 | 0 | walker granularity / wrong slice |
| fs/listing | 10 | 10 | 0 | 0 | filesystem/listing value |
| timing-only | 7 | 0 | 0 | 7 | usually no code change |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | w(t)×gap | likely lever |
|:------------|-----:|---------:|:-------------|
| predecessor not scheduled | 2 | 0.07 | promote predecessor |
| too expensive at final margin | 9 | 3.08 | free final budget |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=28, unscheduled bbox=10, scheduled same-file=1, fs-only=16

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | late | full | 1 |
| scheduled bbox | missing | low | 16 |
| scheduled bbox | partial | low | 11 |
| unscheduled bbox | missing | low | 4 |
| unscheduled bbox | missing | high | 4 |
| unscheduled bbox | missing | full | 2 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 2.3 | 729 | — | — | 0.00 | missing | Hello-toasty: Db::builder + connect | [unscheduled bbox exact=12/15] entry item at examples/hello-toasty/src/main.rs:34 (12 atoms, too expensive at final margin) |
| 2.4 | 830 | — | — | 0.00 | missing | Hello-toasty: User::create + get_by_id + get_by_email | [unscheduled bbox exact=7/7] entry item at examples/hello-toasty/src/main.rs:34 (18 atoms, too expensive at final margin) |
| 2.5 | 1071 | — | — | 0.00 | missing | Hello-toasty: relation use + delete + create_many + nested create | [unscheduled bbox exact=16/16] entry item at examples/hello-toasty/src/main.rs:34 (27 atoms, too expensive at final margin) |
| 3.3 | 1641 | — | — | 0.06 | missing | ARCHITECTURE.md: toasty crate role | [scheduled bbox exact=2/16] headings outline in docs/ARCHITECTURE.md (t=4241, 2 atoms); better unscheduled exact=13/16: docs/ARCHITECTURE.md section #2 (13 atoms, too expensive at final margin) |
| 3.4 | 1823 | — | — | 0.08 | missing | ARCHITECTURE.md: toasty-core crate role | [scheduled bbox exact=2/12] headings outline in docs/ARCHITECTURE.md (t=4241, 2 atoms); better unscheduled exact=11/12: docs/ARCHITECTURE.md section #3 (11 atoms, too expensive at final margin) |
| 4.2 | 2901 | — | — | 0.07 | missing | Db: struct + every public fn signature | [scheduled bbox exact=1/15] pub-item names surface in crates/toasty/src/db.rs (t=7295, 2 atoms); better unscheduled exact=13/15: impl method sigs in crates/toasty/src/db.rs (26 atoms, too expensive at final margin) |
| 6.1 | 4879 | — | — | 0.00 | missing | Engine module: every phase mod declared | [unscheduled bbox exact=20/23] mod/use plumbing in crates/toasty/src/engine.rs (20 atoms, too expensive at final margin) |
| 6.6 | 5815 | — | — | 0.00 | missing | Query engine doc: 5-phase compilation pipeline diagram | [unscheduled bbox exact=18/19] docs/architecture/query-engine.md section #3 (18 atoms, predecessor not scheduled: headings outline in docs/architecture/query-engine.md) |
| 7.6 | 6768 | — | — | 0.12 | missing | Driver trait: 6 method signatures | [scheduled bbox exact=1/8] pub-item names surface in crates/toasty-core/src/driver.rs (t=3759, 2 atoms); better unscheduled exact=7/8: pub item at crates/toasty-core/src/driver.rs:18 (19 atoms, too expensive at final margin) |
| 7.7 | 6931 | — | — | 0.09 | missing | Connection trait: 4 method signatures | [scheduled bbox exact=1/11] pub-item names surface in crates/toasty-core/src/driver.rs (t=3759, 2 atoms); better unscheduled exact=10/11: pub item at crates/toasty-core/src/driver.rs:45 (15 atoms, too expensive at final margin) |
| 8.5 | 7917 | — | — | 0.00 | missing | docs/CHANGE_GUIDE.md: where-changes-go matrix | [unscheduled bbox exact=10/12] docs/CHANGE_GUIDE.md section #1 (10 atoms, predecessor not scheduled: headings outline in docs/CHANGE_GUIDE.md) |

### wrong-slice / granularity

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.1 | 55 | — | — | 0.60 | partial | Repo lede: title + status | [scheduled bbox exact=3/5] README headline in README.md (t=93, 3 atoms) |
| 2.1 | 444 | — | — | 0.69 | partial | Hello-toasty: User model | [scheduled bbox exact=11/16] entry item at examples/hello-toasty/src/main.rs:2 (t=562, 11 atoms) |
| 2.2 | 572 | — | — | 0.71 | partial | Hello-toasty: Todo model with belongs_to | [scheduled bbox exact=10/14] entry item at examples/hello-toasty/src/main.rs:19 (t=662, 10 atoms) |
| 3.1 | 1145 | — | — | 0.71 | partial | ARCHITECTURE.md: opening + Crates header | [scheduled bbox exact=5/7] headings outline in docs/ARCHITECTURE.md (t=4241, 5 atoms) |
| 3.2 | 1404 | — | — | 0.05 | missing | Engine pipeline phases (canonical phase list) | [scheduled bbox exact=2/20] pub-item names surface in crates/toasty/src/engine.rs (t=7317, 2 atoms); better unscheduled exact=12/20: pub-item doc lede at crates/toasty/src/engine.rs:38 (12 atoms, predecessor not scheduled: pub item at crates/toasty/src/engine.rs:38) |
| 3.5 | 2115 | — | — | 0.09 | missing | ARCHITECTURE.md: codegen + drivers | [scheduled bbox exact=4/23] headings outline in docs/ARCHITECTURE.md (t=4241, 4 atoms); better unscheduled exact=13/23: docs/ARCHITECTURE.md section #4 (13 atoms, too expensive at final margin) |
| 3.6 | 2306 | — | — | 0.20 | missing | ARCHITECTURE.md: toasty-sql + further reading | [scheduled bbox exact=4/15] headings outline in docs/ARCHITECTURE.md (t=4241, 4 atoms); better unscheduled exact=9/15: docs/ARCHITECTURE.md section #6 (9 atoms, too expensive at final margin) |
| 4.1 | 2598 | — | — | 0.60 | partial | toasty/src/lib.rs: public re-exports | [scheduled bbox exact=22/35] mod/use plumbing in crates/toasty/src/lib.rs (t=8171, 24 atoms) |
| 5.1 | 3247 | — | — | 0.00 | missing | toasty/src/stmt.rs: typed Statement<M> wrapper | [unscheduled bbox exact=28/43] mod/use plumbing in crates/toasty/src/stmt.rs (28 atoms, too expensive at final margin) |
| 5.3 | 3556 | — | — | 0.00 | missing | Select<M>: every public method signature | [unscheduled bbox exact=10/16] impl method sigs in crates/toasty/src/stmt/select.rs (21 atoms, predecessor not scheduled: listing of 'crates/toasty/src/stmt') |
| 5.4 | 3724 | — | — | 0.50 | partial | Cursor<M>: stream API | [scheduled bbox exact=5/14] pub item at crates/toasty/src/cursor.rs:6 (t=7509, 5 atoms) |
| 5.5 | 3967 | — | — | 0.05 | missing | Page<M>: pagination return shape | [scheduled bbox exact=2/19] pub-item names surface in crates/toasty/src/page.rs (t=7329, 2 atoms); better unscheduled exact=10/19: pub item at crates/toasty/src/page.rs:10 (10 atoms, too expensive at final margin) |
| 5.6 | 4309 | — | — | 0.02 | missing | Relation trait: HasMany / BelongsTo / HasOne / Option | [scheduled bbox exact=2/42] pub-item names surface in crates/toasty/src/relation.rs (t=7305, 2 atoms); better unscheduled exact=18/42: pub item at crates/toasty/src/relation.rs:16 (18 atoms, too expensive at final margin) |
| 5.7 | 4579 | — | — | 0.31 | missing | Model trait + Register trait | [scheduled bbox exact=7/26] pub item at crates/toasty/src/model.rs:26 (t=7922, 8 atoms); better unscheduled exact=14/26: pub item at crates/toasty/src/model.rs:41 (14 atoms, too expensive at final margin) |
| 5.8 | 4693 | — | — | 0.30 | missing | HasMany<T>: load + get + is_unloaded + unload | [scheduled bbox exact=3/10] pub item at crates/toasty/src/relation/has_many.rs:8 (t=8266, 3 atoms); better unscheduled exact=4/10: impl method sigs in crates/toasty/src/relation/has_many.rs (8 atoms, too expensive at final margin) |
| 6.2 | 5328 | — | — | 0.00 | missing | Engine: exec entrypoint with full phase invocation | [unscheduled bbox exact=7/39] impl method sigs in crates/toasty/src/engine.rs (7 atoms, too expensive at final margin) |
| 7.1 | 5974 | — | — | 0.67 | partial | toasty-core lib.rs | [scheduled bbox exact=9/18] mod/use plumbing in crates/toasty-core/src/lib.rs (t=3723, 9 atoms) |
| 7.5 | 6632 | — | — | 0.67 | partial | Statement enum: 4 variants | [scheduled bbox exact=10/15] pub item at crates/toasty-core/src/stmt.rs:253 (t=5135, 10 atoms) |
| 8.1 | 7159 | — | — | 0.11 | missing | toasty-macros: derive entrypoints | [scheduled bbox exact=4/18] pub-item names surface in crates/toasty-macros/src/lib.rs (t=1581, 4 atoms) |
| 8.2 | 7366 | — | — | 0.28 | missing | toasty-codegen lib.rs entrypoints | [scheduled bbox exact=4/18] pub-item names surface in crates/toasty-codegen/src/lib.rs (t=2043, 4 atoms) |
| 8.4 | 7723 | — | — | 0.18 | missing | CLAUDE.md: layer-to-crate decision tree | [scheduled bbox exact=4/22] headings outline in CLAUDE.md (t=4677, 4 atoms); better unscheduled exact=9/22: CLAUDE.md section #4 (9 atoms, too expensive at final margin) |
| 9.1 | 7984 | — | — | 0.75 | partial | toasty-sql lib.rs | [scheduled bbox exact=6/8] mod/use plumbing in crates/toasty-sql/src/lib.rs (t=2531, 6 atoms) |
| 9.3 | 8457 | — | — | 0.25 | missing | SQLite driver: enum + Driver impl signatures | [scheduled bbox exact=8/36] impl method sigs in crates/toasty-driver-sqlite/src/lib.rs (t=9133, 12 atoms) |
| 9.4 | 8694 | — | — | 0.00 | missing | Connect: URL scheme dispatch (sqlite/postgres/mysql/dynamodb) | [scheduled same-file] pub item at crates/toasty/src/db/connect.rs:14 (t=7556, 3 atoms) |
| 10.3 | 9368 | — | — | 0.00 | missing | Per-driver test entry: sqlite.rs full | [unscheduled bbox exact=8/32] impl method sigs in tests/tests/sqlite.rs (8 atoms, predecessor not scheduled: Fs(DirListing { dir: "/Users/yoav/projects/precis/tests/fixtures/toasty/tests/tests" })) |
| 10.4 | 9633 | — | — | 0.63 | partial | Examples: composite-key model | [scheduled bbox exact=10/30] entry item at examples/composite-key/src/main.rs:2 (t=275, 10 atoms) |
| 10.5 | 9856 | — | — | 0.68 | partial | Examples: user-has-one-profile model | [scheduled bbox exact=9/25] entry item at examples/user-has-one-profile/src/main.rs:14 (t=463, 9 atoms) |

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
| 1.2 | 131 | 806 | +675 | 1.00 | late | Repo lede: SQL+NoSQL claim | [scheduled bbox exact=4/4] README.md section #0 (t=806, 4 atoms) |
| 1.3 | 176 | 45 | -131 | 1.00 | early | Top-level repo file listing | fs-only |
| 1.4 | 261 | 1397 | +1136 | 1.00 | late | Crates dir listing (12 crates) | fs-only |
| 1.6 | 312 | 836 | +524 | 1.00 | late | Top-level docs dir listing | fs-only |
| 7.2 | 6010 | 3504 | -2506 | 1.00 | early | toasty-core src dir listing | fs-only |
| 7.8 | 6946 | 3960 | -2986 | 1.00 | early | toasty-core/src/driver/ submodule listing | fs-only |
| 9.2 | 8058 | 2450 | -5608 | 1.00 | early | toasty-sql + driver-crate src dir listings | fs-only |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 2 | 144 | pub item at crates/toasty-driver-integration-suite/src/logging_driver.rs:<n> |
| 2 | 108 | pub item at crates/toasty-driver-integration-suite/src/helpers.rs:<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 483 | 1.00 | 483 | 9693 | impl method sigs in crates/toasty-driver-postgresql/src/lib.rs |
| 414 | 1.00 | 414 | 6753 | impl method sigs in crates/toasty-driver-mysql/src/lib.rs |
| 398 | 0.84 | 473 | 9133 | impl method sigs in crates/toasty-driver-sqlite/src/lib.rs |
| 323 | 1.00 | 323 | 1231 | [package] in Cargo.toml |
| 289 | 1.00 | 289 | 8660 | mod/use plumbing in crates/toasty-driver-dynamodb/src/lib.rs |
| 282 | 1.00 | 282 | 7049 | entry item at examples/user-has-one-profile/src/main.rs:27 |
| 225 | 0.88 | 255 | 4677 | headings outline in CLAUDE.md |
| 209 | 1.00 | 209 | 5048 | mod/use plumbing in crates/toasty-driver-postgresql/src/lib.rs |
| 186 | 1.00 | 186 | 3945 | mod/use plumbing in crates/toasty-driver-sqlite/src/lib.rs |
| 177 | 1.00 | 177 | 2733 | impl method sigs in crates/toasty-cli/src/lib.rs |
| 2133 | — | — | — | +28 more rows |

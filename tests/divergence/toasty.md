scores: Score(3000)=0.395 ns_rows≤3K=19/55 (reached=4 partial=3 missing=12)

## Per-budget scores

| B | A_B | I(B) | C(B) | Score(B) | walker_used |
|--:|----:|-----:|-----:|---------:|------------:|
| 1000 | 100 | 0.678 | 0.466 | 0.562 | 908 |
| 1442 | 143 | 0.711 | 0.417 | 0.545 | 1440 |
| 2080 | 171 | 0.694 | 0.349 | 0.492 | 2029 |
| 3000 | 259 | 0.676 | 0.230 | 0.395 | 2983 |
| 4327 | 407 | 0.645 | 0.181 | 0.342 | 4321 |
| 6240 | 635 | 0.606 | 0.282 | 0.414 | 6216 |
| 9000 | 951 | 0.590 | 0.272 | 0.400 | 8989 |

## Verdict

Verdict: wrong-slice bound
Likely primary lever: split walker batches to match NS semantic slices
Evidence: 9 ranking-recoverable (gap@3k=0.58), 29 wrong-slice/granularity (gap@3k=2.46), 0 no-discovered (gap@3k=0.00)
Secondary intervention: promote predecessors for 2 gated candidates
Top rows: 1.1, 4.1, 5.1, 3.2, 3.5, ...

## Top opportunities

_`gap@B` is a non-additive priority score: `Σ over atoms in row: (1 − damped_credit(a)) / rank(a)` evaluated at budget B's walker state. Approximates how much closing the row would lift `Score(B)` (via the Importance numerator); not an exact delta. `gap@3k` is the primary sort key. `gap@1k` and `gap@9k` show how the same intervention scales across the budget vector — gap is monotone non-increasing in B (walker has more budget at higher B). Rows can overlap between opportunities; sums are upper bounds on Score(B) impact, not additive estimates._

| intervention | rows | gap@1k | gap@3k | gap@9k | evidence | top row ids |
|:-------------|-----:|-------:|-------:|-------:|:---------|:------------|
| split wrong-slice walker batches | 29 | 2.48 | 2.46 | 2.41 | nearby candidates have low exact atom overlap | 1.1, 4.1, 5.1, 3.2, 3.5, ... |
| tune ranking for high-overlap unscheduled candidates | 7 | 0.53 | 0.53 | 0.53 | high-overlap candidates not in the schedule by T_max, exact total=85/95 | 2.3, 2.5, 2.4, 4.2, 6.1, ... |
| promote headings outline in docs/architecture/query-engine.md | 1 | 0.03 | 0.03 | 0.03 | 0 files, exact total=18/19 | 6.6 |
| promote headings outline in docs/CHANGE_GUIDE.md | 1 | 0.01 | 0.01 | 0.01 | 0 files, exact total=10/12 | 8.5 |

## Diagnosis rollup

| diagnosis | rows | missing | partial | likely lever |
|:----------|-----:|--------:|--------:|:-------------|
| ranking-recoverable | 9 | 9 | 0 | value/ranking |
| wrong-slice / granularity | 29 | 24 | 5 | walker granularity / wrong slice |
| fs/listing | 7 | 7 | 0 | filesystem/listing value |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | gap@3k | likely lever |
|:------------|-----:|-------:|:-------------|
| predecessor not scheduled | 2 | 0.05 | promote predecessor |
| too expensive at final margin | 7 | 0.53 | tune ranking |

Candidate hint kinds: scheduled bbox=27, unscheduled bbox=10, scheduled same-file=1, fs-only=7 _(candidates are walker batches discovered this run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` isn't proof that no emit path exists)._

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | missing | low | 22 |
| scheduled bbox | partial | low | 5 |
| unscheduled bbox | missing | low | 4 |
| unscheduled bbox | missing | high | 4 |
| unscheduled bbox | missing | full | 2 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 2.3 | 729 | 0.00 | 0.00 | missing | Hello-toasty: Db::builder + connect | [unscheduled bbox exact=12/15] entry item at examples/hello-toasty/src/main.rs:34 (12 atoms, too expensive at final margin) |
| 2.4 | 830 | 0.00 | 0.00 | missing | Hello-toasty: User::create + get_by_id + get_by_email | [unscheduled bbox exact=7/7] entry item at examples/hello-toasty/src/main.rs:34 (18 atoms, too expensive at final margin) |
| 2.5 | 1071 | 0.00 | 0.00 | missing | Hello-toasty: relation use + delete + create_many + nested create | [unscheduled bbox exact=16/16] entry item at examples/hello-toasty/src/main.rs:34 (27 atoms, too expensive at final margin) |
| 4.2 | 2901 | 0.07 | 0.02 | missing | Db: struct + every public fn signature | [scheduled bbox exact=1/15] pub-item names surface in crates/toasty/src/db.rs (t=2300, 2 atoms); better unscheduled exact=13/15: impl method sigs in crates/toasty/src/db.rs (26 atoms, too expensive at final margin) |
| 6.1 | 4879 | 0.00 | 0.00 | missing | Engine module: every phase mod declared | [unscheduled bbox exact=20/23] mod/use plumbing in crates/toasty/src/engine.rs (20 atoms, too expensive at final margin) |
| 6.6 | 5815 | 0.00 | 0.00 | missing | Query engine doc: 5-phase compilation pipeline diagram | [unscheduled bbox exact=18/19] docs/architecture/query-engine.md section #6 (18 atoms, predecessor not scheduled: headings outline in docs/architecture/query-engine.md) |
| 7.6 | 6768 | 0.00 | 0.00 | missing | Driver trait: 6 method signatures | [scheduled bbox exact=1/8] pub-item names surface in crates/toasty-core/src/driver.rs (t=7142, 2 atoms); better unscheduled exact=7/8: pub item at crates/toasty-core/src/driver.rs:18 (19 atoms, too expensive at final margin) |
| 7.7 | 6931 | 0.00 | 0.00 | missing | Connection trait: 4 method signatures | [scheduled bbox exact=1/11] pub-item names surface in crates/toasty-core/src/driver.rs (t=7142, 2 atoms); better unscheduled exact=10/11: pub item at crates/toasty-core/src/driver.rs:45 (15 atoms, too expensive at final margin) |
| 8.5 | 7917 | 0.00 | 0.00 | missing | docs/CHANGE_GUIDE.md: where-changes-go matrix | [unscheduled bbox exact=10/12] docs/CHANGE_GUIDE.md section #1 (10 atoms, predecessor not scheduled: headings outline in docs/CHANGE_GUIDE.md) |

### wrong-slice / granularity

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 1.1 | 55 | 0.60 | 0.99 | partial | Repo lede: title + status | [scheduled bbox exact=3/5] README headline in README.md (t=93, 3 atoms) |
| 2.1 | 444 | 0.69 | 0.83 | partial | Hello-toasty: User model | [scheduled bbox exact=11/16] entry item at examples/hello-toasty/src/main.rs:2 (t=562, 11 atoms) |
| 2.2 | 572 | 0.71 | 0.85 | partial | Hello-toasty: Todo model with belongs_to | [scheduled bbox exact=10/14] entry item at examples/hello-toasty/src/main.rs:19 (t=662, 10 atoms) |
| 3.1 | 1145 | 0.00 | 0.00 | missing | ARCHITECTURE.md: opening + Crates header | [scheduled bbox exact=5/7] headings outline in docs/ARCHITECTURE.md (t=8971, 5 atoms) |
| 3.2 | 1404 | 0.05 | 0.03 | missing | Engine pipeline phases (canonical phase list) | [scheduled bbox exact=2/20] pub-item names surface in crates/toasty/src/engine.rs (t=2384, 2 atoms); better unscheduled exact=12/20: pub-item doc lede at crates/toasty/src/engine.rs:38 (12 atoms, predecessor not scheduled: pub item at crates/toasty/src/engine.rs:38) |
| 3.3 | 1641 | 0.00 | 0.00 | missing | ARCHITECTURE.md: toasty crate role | [scheduled bbox exact=2/16] headings outline in docs/ARCHITECTURE.md (t=8971, 2 atoms); better unscheduled exact=6/16: docs/ARCHITECTURE.md section #3 (6 atoms, too expensive at final margin) |
| 3.4 | 1823 | 0.00 | 0.00 | missing | ARCHITECTURE.md: toasty-core crate role | [scheduled bbox exact=2/12] headings outline in docs/ARCHITECTURE.md (t=8971, 2 atoms); better unscheduled exact=9/12: docs/ARCHITECTURE.md section #7 (9 atoms, too expensive at final margin) |
| 3.5 | 2115 | 0.00 | 0.00 | missing | ARCHITECTURE.md: codegen + drivers | [scheduled bbox exact=4/23] headings outline in docs/ARCHITECTURE.md (t=8971, 4 atoms); better unscheduled exact=11/23: docs/ARCHITECTURE.md section #9 (11 atoms, too expensive at final margin) |
| 3.6 | 2306 | 0.00 | 0.00 | missing | ARCHITECTURE.md: toasty-sql + further reading | [scheduled bbox exact=4/15] headings outline in docs/ARCHITECTURE.md (t=8971, 4 atoms); better unscheduled exact=7/15: docs/ARCHITECTURE.md section #12 (7 atoms, too expensive at final margin) |
| 4.1 | 2598 | 0.00 | 0.00 | missing | toasty/src/lib.rs: public re-exports | [unscheduled bbox exact=22/35] mod/use plumbing in crates/toasty/src/lib.rs (24 atoms, too expensive at final margin) |
| 5.1 | 3247 | 0.00 | 0.00 | missing | toasty/src/stmt.rs: typed Statement<M> wrapper | [unscheduled bbox exact=28/43] mod/use plumbing in crates/toasty/src/stmt.rs (28 atoms, too expensive at final margin) |
| 5.3 | 3556 | 0.00 | 0.00 | missing | Select<M>: every public method signature | [scheduled bbox exact=5/16] pub item at crates/toasty/src/stmt/select.rs:6 (t=9339, 5 atoms); better unscheduled exact=10/16: impl method sigs in crates/toasty/src/stmt/select.rs (21 atoms, too expensive at final margin) |
| 5.4 | 3724 | 0.00 | 0.00 | missing | Cursor<M>: stream API | [scheduled bbox exact=5/14] pub item at crates/toasty/src/cursor.rs:6 (t=6878, 5 atoms) |
| 5.5 | 3967 | 0.05 | 0.03 | missing | Page<M>: pagination return shape | [scheduled bbox exact=2/19] pub-item names surface in crates/toasty/src/page.rs (t=2396, 2 atoms); better unscheduled exact=10/19: pub item at crates/toasty/src/page.rs:10 (10 atoms, too expensive at final margin) |
| 5.6 | 4309 | 0.02 | 0.04 | missing | Relation trait: HasMany / BelongsTo / HasOne / Option | [scheduled bbox exact=2/42] pub-item names surface in crates/toasty/src/relation.rs (t=2310, 2 atoms); better unscheduled exact=18/42: pub item at crates/toasty/src/relation.rs:16 (18 atoms, too expensive at final margin) |
| 5.7 | 4579 | 0.00 | 0.00 | missing | Model trait + Register trait | [scheduled bbox exact=7/26] pub item at crates/toasty/src/model.rs:26 (t=9174, 8 atoms); better unscheduled exact=14/26: pub item at crates/toasty/src/model.rs:41 (14 atoms, too expensive at final margin) |
| 5.8 | 4693 | 0.00 | 0.00 | missing | HasMany<T>: load + get + is_unloaded + unload | [scheduled bbox exact=3/10] pub item at crates/toasty/src/relation/has_many.rs:8 (t=3783, 3 atoms); better unscheduled exact=4/10: impl method sigs in crates/toasty/src/relation/has_many.rs (8 atoms, too expensive at final margin) |
| 6.2 | 5328 | 0.00 | 0.00 | missing | Engine: exec entrypoint with full phase invocation | [unscheduled bbox exact=7/39] impl method sigs in crates/toasty/src/engine.rs (7 atoms, too expensive at final margin) |
| 7.1 | 5974 | 0.11 | 0.20 | missing | toasty-core lib.rs | [scheduled bbox exact=9/18] mod/use plumbing in crates/toasty-core/src/lib.rs (t=4209, 9 atoms) |
| 7.5 | 6632 | 0.07 | 0.07 | missing | Statement enum: 4 variants | [scheduled bbox exact=2/15] pub-item names surface in crates/toasty-core/src/stmt.rs (t=2983, 2 atoms); better unscheduled exact=10/15: pub item at crates/toasty-core/src/stmt.rs:253 (10 atoms, too expensive at final margin) |
| 8.1 | 7159 | 0.56 | 0.70 | missing | toasty-macros: derive entrypoints | [scheduled bbox exact=4/18] pub item body at crates/toasty-macros/src/lib.rs:18 body 19 (t=1716, 4 atoms) |
| 8.2 | 7366 | 0.61 | 0.99 | partial | toasty-codegen lib.rs entrypoints | [scheduled bbox exact=4/18] pub-item names surface in crates/toasty-codegen/src/lib.rs (t=1867, 4 atoms) |
| 8.4 | 7723 | 0.00 | 0.00 | missing | CLAUDE.md: layer-to-crate decision tree | [scheduled bbox exact=4/22] headings outline in CLAUDE.md (t=9691, 4 atoms); better unscheduled exact=9/22: CLAUDE.md section #4 (9 atoms, too expensive at final margin) |
| 9.1 | 7984 | 0.00 | 0.00 | missing | toasty-sql lib.rs | [scheduled bbox exact=6/8] mod/use plumbing in crates/toasty-sql/src/lib.rs (t=3457, 6 atoms) |
| 9.3 | 8457 | 0.11 | 0.05 | missing | SQLite driver: enum + Driver impl signatures | [scheduled bbox exact=4/36] pub item at crates/toasty-driver-sqlite/src/lib.rs:23 (t=2167, 4 atoms); better unscheduled exact=8/36: impl method sigs in crates/toasty-driver-sqlite/src/lib.rs (12 atoms, too expensive at final margin) |
| 9.4 | 8694 | 0.00 | 0.00 | missing | Connect: URL scheme dispatch (sqlite/postgres/mysql/dynamodb) | [scheduled same-file] pub item at crates/toasty/src/db/connect.rs:14 (t=3653, 3 atoms) |
| 10.3 | 9368 | 0.00 | 0.00 | missing | Per-driver test entry: sqlite.rs full | [unscheduled bbox exact=8/32] impl method sigs in tests/tests/sqlite.rs (8 atoms, predecessor not scheduled: listing of 'tests/tests') |
| 10.4 | 9633 | 0.63 | 0.75 | missing | Examples: composite-key model | [scheduled bbox exact=10/30] entry item at examples/composite-key/src/main.rs:2 (t=275, 10 atoms) |
| 10.5 | 9856 | 0.68 | 0.83 | partial | Examples: user-has-one-profile model | [scheduled bbox exact=9/25] entry item at examples/user-has-one-profile/src/main.rs:14 (t=463, 9 atoms) |

### fs/listing

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 1.5 | 282 | 0.00 | 0.00 | missing | Examples dir listing | fs-only |
| 6.3 | 5431 | 0.00 | 0.00 | missing | engine/simplify/ dir listing | fs-only |
| 7.3 | 6095 | 0.36 | 0.36 | missing | toasty-core schema/app + schema/db submodule listings | fs-only |
| 7.4 | 6504 | 0.00 | 0.00 | missing | toasty-core/src/stmt/ dir listing — full AST node surface | fs-only |
| 9.2 | 8058 | 0.21 | 0.21 | missing | toasty-sql + driver-crate src dir listings | fs-only |
| 10.1 | 8797 | 0.13 | 0.13 | missing | Workspace tests/ crate dir listing | fs-only |
| 10.2 | 9040 | 0.00 | 0.00 | missing | Driver integration suite: tests/ dir listing | fs-only |

## Walker waste, primary-actionable (first_t ≤ 3000, off-NS spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 323 | 1.00 | 323 | 1231 | [package] in Cargo.toml |
| 68 | 1.00 | 68 | 730 | headings outline in README.md |
| 65 | 1.00 | 65 | 2473 | listing of 'crates/toasty/src/engine' |
| 60 | 1.00 | 60 | 2255 | listing of 'crates/toasty/src' |
| 58 | 0.60 | 96 | 1592 | pub-item names surface in crates/toasty-macros/src/lib.rs |

## Walker waste, late (first_t > 3000, off-NS spend ≥ 50) — higher-budget calibration only

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 225 | 0.88 | 255 | 9691 | headings outline in CLAUDE.md |
| 186 | 1.00 | 186 | 8524 | mod/use plumbing in crates/toasty-driver-sqlite/src/lib.rs |
| 177 | 1.00 | 177 | 6021 | impl method sigs in crates/toasty-cli/src/lib.rs |
| 172 | 1.00 | 172 | 8051 | mod/use plumbing in crates/toasty-driver-mysql/src/lib.rs |
| 104 | 1.00 | 104 | 6480 | pub item body at crates/toasty-driver-integration-suite-macros/src/lib.rs:28 body 29 |
| 87 | 1.00 | 87 | 6303 | pub-item names surface in crates/toasty-driver-integration-suite-macros/src/lib.rs |
| 84 | 1.00 | 84 | 9258 | [features] in crates/toasty-core/Cargo.toml |
| 82 | 1.00 | 82 | 3199 | listing of 'crates/toasty-core/src/error' |
| 80 | 1.00 | 80 | 8604 | [features] in crates/toasty-driver-postgresql/Cargo.toml |
| 80 | 1.00 | 80 | 8131 | pub item at crates/toasty/src/apply_update.rs:9 |
| 544 | — | — | — | +9 more rows |

scores: Sim=0.300 Reached=8/55 Early=2 Late=3 Partial=4 Missing=43 Used=9957/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 6 | 4 | 1 | 1 | 0.77 |
| 2 | 5 | 0 | 0 | 5 | 0.00 |
| 3 | 6 | 1 | 0 | 5 | 0.24 |
| 4 | 2 | 0 | 0 | 2 | 0.00 |
| 5 | 8 | 0 | 0 | 8 | 0.00 |
| 6 | 6 | 0 | 0 | 6 | 0.00 |
| 7 | 8 | 2 | 2 | 4 | 0.44 |
| 8 | 5 | 0 | 0 | 5 | 0.11 |
| 9 | 4 | 1 | 1 | 2 | 0.50 |
| 10 | 5 | 0 | 0 | 5 | 0.06 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.1 | 55 | — | — | 0.60 | partial | Repo lede: title + status | README headline in README.md (t=93, 3 atoms) |
| 1.2 | 131 | 257 | +126 | 1.00 | late | Repo lede: SQL+NoSQL claim | README.md section #0 (t=257, 4 atoms) |
| 1.3 | 176 | 45 | -131 | 1.00 | early | Top-level repo file listing |  |
| 1.4 | 261 | 989 | +728 | 1.00 | late | Crates dir listing (12 crates) |  |
| 1.5 | 282 | — | — | 0.00 | missing | Examples dir listing |  |
| 2.1 | 444 | — | — | 0.00 | missing | Hello-toasty: User model |  |
| 2.2 | 572 | — | — | 0.00 | missing | Hello-toasty: Todo model with belongs_to |  |
| 2.3 | 729 | — | — | 0.00 | missing | Hello-toasty: Db::builder + connect |  |
| 2.4 | 830 | — | — | 0.00 | missing | Hello-toasty: User::create + get_by_id + get_by_email |  |
| 2.5 | 1071 | — | — | 0.00 | missing | Hello-toasty: relation use + delete + create_many + nested create |  |
| 3.1 | 1145 | 4179 | +3034 | 0.86 | late | ARCHITECTURE.md: opening + Crates header | headings outline in docs/ARCHITECTURE.md (t=3082, 5 atoms) |
| 3.2 | 1404 | — | — | 0.00 | missing | Engine pipeline phases (canonical phase list) |  |
| 3.3 | 1641 | — | — | 0.06 | missing | ARCHITECTURE.md: toasty crate role | headings outline in docs/ARCHITECTURE.md (t=3082, 2 atoms) |
| 3.4 | 1823 | — | — | 0.08 | missing | ARCHITECTURE.md: toasty-core crate role | headings outline in docs/ARCHITECTURE.md (t=3082, 2 atoms) |
| 3.5 | 2115 | — | — | 0.09 | missing | ARCHITECTURE.md: codegen + drivers | headings outline in docs/ARCHITECTURE.md (t=3082, 4 atoms) |
| 3.6 | 2306 | — | — | 0.33 | missing | ARCHITECTURE.md: toasty-sql + further reading | headings outline in docs/ARCHITECTURE.md (t=3082, 4 atoms) |
| 4.1 | 2598 | — | — | 0.00 | missing | toasty/src/lib.rs: public re-exports |  |
| 4.2 | 2901 | — | — | 0.00 | missing | Db: struct + every public fn signature |  |
| 5.1 | 3247 | — | — | 0.00 | missing | toasty/src/stmt.rs: typed Statement<M> wrapper |  |
| 5.2 | 3309 | — | — | 0.00 | missing | toasty/src/stmt/ dir listing |  |
| 5.3 | 3556 | — | — | 0.00 | missing | Select<M>: every public method signature |  |
| 5.4 | 3724 | — | — | 0.00 | missing | Cursor<M>: stream API |  |
| 5.5 | 3967 | — | — | 0.00 | missing | Page<M>: pagination return shape |  |
| 5.6 | 4309 | — | — | 0.00 | missing | Relation trait: HasMany / BelongsTo / HasOne / Option |  |
| 5.7 | 4579 | — | — | 0.00 | missing | Model trait + Register trait |  |
| 5.8 | 4693 | — | — | 0.00 | missing | HasMany<T>: load + get + is_unloaded + unload |  |
| 6.1 | 4879 | — | — | 0.00 | missing | Engine module: every phase mod declared |  |
| 6.2 | 5328 | — | — | 0.00 | missing | Engine: exec entrypoint with full phase invocation |  |
| 6.3 | 5431 | — | — | 0.00 | missing | engine/simplify/ dir listing |  |
| 6.4 | 5539 | — | — | 0.00 | missing | engine/lower/ + engine/plan/ + engine/mir/ dir listings |  |
| 6.5 | 5632 | — | — | 0.00 | missing | engine/exec/ + engine/eval/ + engine/index/ dir listings |  |
| 6.6 | 5815 | — | — | 0.00 | missing | Query engine doc: 5-phase compilation pipeline diagram |  |
| 7.1 | 5974 | — | — | 0.67 | partial | toasty-core lib.rs | mod/use plumbing in crates/toasty-core/src/lib.rs (t=4962, 9 atoms) |
| 7.3 | 6095 | — | — | 0.00 | missing | toasty-core schema/app + schema/db submodule listings |  |
| 7.4 | 6504 | — | — | 0.00 | missing | toasty-core/src/stmt/ dir listing — full AST node surface |  |
| 7.5 | 6632 | — | — | 0.67 | partial | Statement enum: 4 variants | pub item at crates/toasty-core/src/stmt.rs:253 (t=5156, 10 atoms) |
| 7.6 | 6768 | — | — | 0.12 | missing | Driver trait: 6 method signatures | pub-item names surface in crates/toasty-core/src/driver.rs (t=4882, 2 atoms) |
| 7.7 | 6931 | — | — | 0.09 | missing | Connection trait: 4 method signatures | pub-item names surface in crates/toasty-core/src/driver.rs (t=4882, 2 atoms) |
| 8.1 | 7159 | — | — | 0.11 | missing | toasty-macros: derive entrypoints | pub-item names surface in crates/toasty-macros/src/lib.rs (t=1153, 4 atoms) |
| 8.2 | 7366 | — | — | 0.28 | missing | toasty-codegen lib.rs entrypoints | pub-item names surface in crates/toasty-codegen/src/lib.rs (t=1653, 4 atoms) |
| 8.3 | 7463 | — | — | 0.00 | missing | codegen/expand/ + codegen/schema/ submodule listings |  |
| 8.4 | 7723 | — | — | 0.18 | missing | CLAUDE.md: layer-to-crate decision tree | headings outline in CLAUDE.md (t=3725, 4 atoms) |
| 8.5 | 7917 | — | — | 0.00 | missing | docs/CHANGE_GUIDE.md: where-changes-go matrix |  |
| 9.1 | 7984 | — | — | 0.75 | partial | toasty-sql lib.rs | mod/use plumbing in crates/toasty-sql/src/lib.rs (t=2467, 6 atoms) |
| 9.2 | 8058 | 2361 | -5697 | 1.00 | early | toasty-sql + driver-crate src dir listings |  |
| 9.3 | 8457 | — | — | 0.25 | missing | SQLite driver: enum + Driver impl signatures | impl method sigs in crates/toasty-driver-sqlite/src/lib.rs (t=5834, 12 atoms) |
| 9.4 | 8694 | — | — | 0.00 | missing | Connect: URL scheme dispatch (sqlite/postgres/mysql/dynamodb) |  |
| 10.1 | 8797 | — | — | 0.00 | missing | Workspace tests/ crate dir listing |  |
| 10.2 | 9040 | — | — | 0.29 | missing | Driver integration suite: tests/ dir listing |  |
| 10.3 | 9368 | — | — | 0.00 | missing | Per-driver test entry: sqlite.rs full |  |
| 10.4 | 9633 | — | — | 0.00 | missing | Examples: composite-key model |  |
| 10.5 | 9856 | — | — | 0.00 | missing | Examples: user-has-one-profile model |  |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 5 | 1044 | README.md section #<n> |
| 3 | 235 | pub item at crates/toasty-driver-integration-suite-macros/src/parse.rs:<n> |
| 2 | 144 | pub item at crates/toasty-driver-integration-suite/src/logging_driver.rs:<n> |
| 2 | 108 | pub item at crates/toasty-driver-integration-suite/src/helpers.rs:<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 537 | 1.00 | 537 | 9695 | README.md section #1 |
| 513 | 1.00 | 513 | 7348 | impl method sigs in crates/toasty-driver-dynamodb/src/lib.rs |
| 483 | 1.00 | 483 | 6340 | impl method sigs in crates/toasty-driver-postgresql/src/lib.rs |
| 414 | 1.00 | 414 | 4641 | impl method sigs in crates/toasty-driver-mysql/src/lib.rs |
| 398 | 0.84 | 473 | 5834 | impl method sigs in crates/toasty-driver-sqlite/src/lib.rs |
| 323 | 1.00 | 323 | 754 | [package] in Cargo.toml |
| 225 | 0.88 | 255 | 3725 | headings outline in CLAUDE.md |
| 207 | 1.00 | 207 | 3373 | README.md section #2 |
| 177 | 1.00 | 177 | 2011 | impl method sigs in crates/toasty-cli/src/lib.rs |
| 173 | 1.00 | 173 | 9099 | mod/use plumbing in crates/toasty-driver-integration-suite/src/lib.rs |
| 172 | 1.00 | 172 | 8926 | mod/use plumbing in crates/toasty-driver-mysql/src/lib.rs |
| 159 | 1.00 | 159 | 4078 | README.md section #4 |
| 154 | 1.00 | 154 | 7659 | pub item at crates/std-util/src/slice.rs:4 |
| 143 | 1.00 | 143 | 7802 | pub item at crates/toasty-sql/src/stmt.rs:46 |
| 128 | 1.00 | 128 | 9925 | headings outline in docs/roadmap/README.md |
| 108 | 1.00 | 108 | 7505 | CLAUDE.md section #17 |
| 106 | 1.00 | 106 | 6775 | pub item at crates/toasty-driver-integration-suite-macros/src/parse.rs:220 |
| 106 | 1.00 | 106 | 8586 | pub item at crates/toasty-driver-integration-suite/src/stmt.rs:28 |
| 87 | 1.00 | 87 | 2779 | pub-item names surface in crates/toasty-driver-integration-suite-macros/src/lib.rs |
| 84 | 1.00 | 84 | 3166 | [features] in crates/toasty-core/Cargo.toml |
| 83 | 1.00 | 83 | 8251 | pub item at crates/toasty-driver-integration-suite/src/setup.rs:4 |
| 80 | 1.00 | 80 | 2968 | [features] in crates/toasty-driver-postgresql/Cargo.toml |
| 79 | 1.00 | 79 | 7881 | pub item at crates/toasty-core/src/driver/capability.rs:103 |
| 79 | 1.00 | 79 | 8680 | pub item at crates/toasty-driver-integration-suite/src/logging_driver.rs:70 |
| 79 | 1.00 | 79 | 6419 | pub-item names surface in crates/toasty-driver-integration-suite-macros/src/parse.rs |
| 76 | 1.00 | 76 | 5232 | pub item at crates/toasty-core/src/schema.rs:22 |
| 73 | 1.00 | 73 | 6654 | pub item at crates/toasty-driver-integration-suite-macros/src/parse.rs:252 |
| 72 | 1.00 | 72 | 431 | README.md section #3 |
| 72 | 1.00 | 72 | 5318 | pub item at crates/toasty-core/src/driver/response.rs:9 |
| 69 | 1.00 | 69 | 877 | README.md section #5 |
| 68 | 1.00 | 68 | 2325 | [features] in crates/toasty-driver-mysql/Cargo.toml |
| 68 | 1.00 | 68 | 181 | headings outline in README.md |
| 68 | 1.00 | 68 | 4146 | pub item at crates/toasty-sql/src/serializer.rs:43 |
| 65 | 1.00 | 65 | 2076 | mod/use plumbing in crates/toasty-cli/src/lib.rs |
| 65 | 1.00 | 65 | 8316 | pub item at crates/toasty-driver-integration-suite/src/logging_driver.rs:14 |
| 60 | 1.00 | 60 | 9797 | [dependencies] in crates/toasty-macros/Cargo.toml |
| 60 | 1.00 | 60 | 2668 | macro_export names across crates/std-util/src |
| 59 | 1.00 | 59 | 9158 | docs/CONTEXT.md section #2 |
| 58 | 0.60 | 96 | 1153 | pub-item names surface in crates/toasty-macros/src/lib.rs |
| 56 | 1.00 | 56 | 6542 | pub item at crates/toasty-driver-integration-suite-macros/src/parse.rs:4 |
| 56 | 1.00 | 56 | 8480 | pub item at crates/toasty-driver-integration-suite/src/helpers.rs:38 |
| 56 | 1.00 | 56 | 8372 | pub-item names surface in crates/toasty-driver-integration-suite/src/helpers.rs |
| 54 | 1.00 | 54 | 3919 | pub-item names surface in crates/std-util/src/slice.rs |
| 53 | 1.00 | 53 | 3426 | pub-item names surface in crates/toasty-sql/src/serializer.rs |
| 52 | 1.00 | 52 | 8424 | pub item at crates/toasty-driver-integration-suite/src/helpers.rs:29 |
| 51 | 1.00 | 51 | 2608 | mod/use plumbing in crates/std-util/src/lib.rs |

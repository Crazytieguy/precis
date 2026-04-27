scores: Sim=0.306 Reached=7/55 Early=4 Late=2 Partial=5 Missing=43 Used=9933/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 6 | 4 | 1 | 1 | 0.77 |
| 2 | 5 | 0 | 0 | 5 | 0.00 |
| 3 | 6 | 0 | 1 | 5 | 0.19 |
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
| 1.4 | 261 | 848 | +587 | 1.00 | late | Crates dir listing (12 crates) |  |
| 1.5 | 282 | — | — | 0.00 | missing | Examples dir listing |  |
| 2.1 | 444 | — | — | 0.00 | missing | Hello-toasty: User model |  |
| 2.2 | 572 | — | — | 0.00 | missing | Hello-toasty: Todo model with belongs_to |  |
| 2.3 | 729 | — | — | 0.00 | missing | Hello-toasty: Db::builder + connect |  |
| 2.4 | 830 | — | — | 0.00 | missing | Hello-toasty: User::create + get_by_id + get_by_email |  |
| 2.5 | 1071 | — | — | 0.00 | missing | Hello-toasty: relation use + delete + create_many + nested create |  |
| 3.1 | 1145 | — | — | 0.71 | partial | ARCHITECTURE.md: opening + Crates header | headings outline in docs/ARCHITECTURE.md (t=2941, 5 atoms) |
| 3.2 | 1404 | — | — | 0.00 | missing | Engine pipeline phases (canonical phase list) |  |
| 3.3 | 1641 | — | — | 0.06 | missing | ARCHITECTURE.md: toasty crate role | headings outline in docs/ARCHITECTURE.md (t=2941, 2 atoms) |
| 3.4 | 1823 | — | — | 0.08 | missing | ARCHITECTURE.md: toasty-core crate role | headings outline in docs/ARCHITECTURE.md (t=2941, 2 atoms) |
| 3.5 | 2115 | — | — | 0.09 | missing | ARCHITECTURE.md: codegen + drivers | headings outline in docs/ARCHITECTURE.md (t=2941, 4 atoms) |
| 3.6 | 2306 | — | — | 0.20 | missing | ARCHITECTURE.md: toasty-sql + further reading | headings outline in docs/ARCHITECTURE.md (t=2941, 4 atoms) |
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
| 7.1 | 5974 | — | — | 0.67 | partial | toasty-core lib.rs | mod/use plumbing in crates/toasty-core/src/lib.rs (t=4306, 9 atoms) |
| 7.2 | 6010 | 4051 | -1959 | 1.00 | early | toasty-core src dir listing |  |
| 7.3 | 6095 | — | — | 0.00 | missing | toasty-core schema/app + schema/db submodule listings |  |
| 7.4 | 6504 | — | — | 0.00 | missing | toasty-core/src/stmt/ dir listing — full AST node surface |  |
| 7.5 | 6632 | — | — | 0.67 | partial | Statement enum: 4 variants | pub item at crates/toasty-core/src/stmt.rs:253 (t=4500, 10 atoms) |
| 7.6 | 6768 | — | — | 0.12 | missing | Driver trait: 6 method signatures | pub-item names surface in crates/toasty-core/src/driver.rs (t=4226, 2 atoms) |
| 7.7 | 6931 | — | — | 0.09 | missing | Connection trait: 4 method signatures | pub-item names surface in crates/toasty-core/src/driver.rs (t=4226, 2 atoms) |
| 7.8 | 6946 | 4321 | -2625 | 1.00 | early | toasty-core/src/driver/ submodule listing |  |
| 8.1 | 7159 | — | — | 0.11 | missing | toasty-macros: derive entrypoints | pub-item names surface in crates/toasty-macros/src/lib.rs (t=1012, 4 atoms) |
| 8.2 | 7366 | — | — | 0.28 | missing | toasty-codegen lib.rs entrypoints | pub-item names surface in crates/toasty-codegen/src/lib.rs (t=1512, 4 atoms) |
| 8.3 | 7463 | — | — | 0.00 | missing | codegen/expand/ + codegen/schema/ submodule listings |  |
| 8.4 | 7723 | — | — | 0.18 | missing | CLAUDE.md: layer-to-crate decision tree | headings outline in CLAUDE.md (t=3377, 4 atoms) |
| 8.5 | 7917 | — | — | 0.00 | missing | docs/CHANGE_GUIDE.md: where-changes-go matrix |  |
| 9.1 | 7984 | — | — | 0.75 | partial | toasty-sql lib.rs | mod/use plumbing in crates/toasty-sql/src/lib.rs (t=2326, 6 atoms) |
| 9.2 | 8058 | 2220 | -5838 | 1.00 | early | toasty-sql + driver-crate src dir listings |  |
| 9.3 | 8457 | — | — | 0.25 | missing | SQLite driver: enum + Driver impl signatures | impl method sigs in crates/toasty-driver-sqlite/src/lib.rs (t=5207, 12 atoms) |
| 9.4 | 8694 | — | — | 0.00 | missing | Connect: URL scheme dispatch (sqlite/postgres/mysql/dynamodb) |  |
| 10.1 | 8797 | — | — | 0.00 | missing | Workspace tests/ crate dir listing |  |
| 10.2 | 9040 | — | — | 0.29 | missing | Driver integration suite: tests/ dir listing |  |
| 10.3 | 9368 | — | — | 0.00 | missing | Per-driver test entry: sqlite.rs full |  |
| 10.4 | 9633 | — | — | 0.00 | missing | Examples: composite-key model |  |
| 10.5 | 9856 | — | — | 0.00 | missing | Examples: user-has-one-profile model |  |

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
| 154 | 1.00 | 154 | 7030 | pub item at crates/std-util/src/slice.rs:4 |
| 143 | 1.00 | 143 | 7212 | pub item at crates/toasty-sql/src/stmt.rs:46 |
| 137 | 1.00 | 137 | 9406 | headings outline in docs/roadmap/query-constraints.md |
| 134 | 1.00 | 134 | 9192 | pub item at crates/toasty-driver-integration-suite-macros/src/parse.rs:77 |
| 128 | 1.00 | 128 | 8779 | headings outline in docs/roadmap/README.md |
| 106 | 1.00 | 106 | 6148 | pub item at crates/toasty-driver-integration-suite-macros/src/parse.rs:220 |
| 106 | 1.00 | 106 | 8036 | pub item at crates/toasty-driver-integration-suite/src/stmt.rs:28 |
| 87 | 1.00 | 87 | 2638 | pub-item names surface in crates/toasty-driver-integration-suite-macros/src/lib.rs |
| 84 | 1.00 | 84 | 3025 | [features] in crates/toasty-core/Cargo.toml |
| 83 | 1.00 | 83 | 7701 | pub item at crates/toasty-driver-integration-suite/src/setup.rs:4 |
| 80 | 1.00 | 80 | 2827 | [features] in crates/toasty-driver-postgresql/Cargo.toml |
| 79 | 1.00 | 79 | 7331 | pub item at crates/toasty-core/src/driver/capability.rs:103 |
| 79 | 1.00 | 79 | 8130 | pub item at crates/toasty-driver-integration-suite/src/logging_driver.rs:70 |
| 79 | 1.00 | 79 | 5792 | pub-item names surface in crates/toasty-driver-integration-suite-macros/src/parse.rs |
| 76 | 1.00 | 76 | 4576 | pub item at crates/toasty-core/src/schema.rs:22 |
| 75 | 1.00 | 75 | 9904 | pub item at crates/toasty-cli/src/migration/drop.rs:11 |
| 73 | 1.00 | 73 | 6027 | pub item at crates/toasty-driver-integration-suite-macros/src/parse.rs:252 |
| 72 | 1.00 | 72 | 4734 | README.md section #3 |
| 72 | 1.00 | 72 | 4662 | pub item at crates/toasty-core/src/driver/response.rs:9 |
| 69 | 1.00 | 69 | 6876 | README.md section #5 |
| 68 | 1.00 | 68 | 2184 | [features] in crates/toasty-driver-mysql/Cargo.toml |
| 68 | 1.00 | 68 | 181 | headings outline in README.md |
| 68 | 1.00 | 68 | 3523 | pub item at crates/toasty-sql/src/serializer.rs:43 |
| 65 | 1.00 | 65 | 1935 | mod/use plumbing in crates/toasty-cli/src/lib.rs |
| 65 | 1.00 | 65 | 7766 | pub item at crates/toasty-driver-integration-suite/src/logging_driver.rs:14 |
| 63 | 1.00 | 63 | 9269 | macro_export body at crates/std-util/src/option.rs:2 |
| 61 | 1.00 | 61 | 8872 | macro_export body at crates/toasty-core/src/macros.rs:2 |
| 61 | 1.00 | 61 | 9817 | pub item at crates/toasty-cli/src/migration/config.rs:27 |
| 60 | 1.00 | 60 | 8651 | [dependencies] in crates/toasty-macros/Cargo.toml |
| 60 | 1.00 | 60 | 2527 | macro_export names across crates/std-util/src |
| 58 | 0.60 | 96 | 1012 | pub-item names surface in crates/toasty-macros/src/lib.rs |
| 56 | 1.00 | 56 | 5915 | pub item at crates/toasty-driver-integration-suite-macros/src/parse.rs:4 |
| 56 | 1.00 | 56 | 7930 | pub item at crates/toasty-driver-integration-suite/src/helpers.rs:38 |
| 56 | 1.00 | 56 | 7822 | pub-item names surface in crates/toasty-driver-integration-suite/src/helpers.rs |
| 54 | 1.00 | 54 | 3455 | pub-item names surface in crates/std-util/src/slice.rs |
| 53 | 1.00 | 53 | 3078 | pub-item names surface in crates/toasty-sql/src/serializer.rs |
| 52 | 1.00 | 52 | 7874 | pub item at crates/toasty-driver-integration-suite/src/helpers.rs:29 |
| 51 | 1.00 | 51 | 2467 | mod/use plumbing in crates/std-util/src/lib.rs |

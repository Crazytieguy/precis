Score(3000)=0.636 I=0.925 C=0.437 ns_rows≤3K=18/52 grid(1000/1442/2080/3000/4327/6240/9000)=0.379/0.512/0.547/0.636/0.554/0.569/0.631

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 47 | 47 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 68 | 21 | Fs::DirListing { dir: examples } |  |  | 0.000 |
| walker |  | 76 | 8 | Fs::DirListing { dir: examples/hello-toasty } |  |  | 0.000 |
| walker |  | 80 | 4 | Fs::DirListing { dir: examples/hello-toasty/src } |  |  | 0.000 |
| walker |  | 83 | 3 | Fs::DirListing { dir: .github } |  |  | 0.000 |
| ns | 88 |  | 88 | README lede: Toasty is a Rust ORM for SQL and NoSQL | 1.1 |  | 0.000 |
| walker |  | 92 | 9 | Fs::DirListing { dir: .github/workflows } |  |  | 0.000 |
| walker |  | 122 | 30 | Fs::DirListing { dir: docs } |  |  | 0.000 |
| walker |  | 131 | 9 | Fs::DirListing { dir: docs/guide } |  |  | 0.000 |
| ns | 135 |  | 47 | Repository root listing (complete) | 1.2 |  | 0.545 |
| walker |  | 141 | 10 | Fs::DirListing { dir: docs/architecture } |  |  | 0.554 |
| walker |  | 155 | 14 | Fs::DirListing { dir: docs/design } |  |  | 0.564 |
| walker |  | 207 | 52 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.575 |
| ns | 220 |  | 85 | The 13 workspace crates under crates/ (complete) | 1.3 |  | 0.420 |
| walker |  | 230 | 23 | Fs::DirListing { dir: docs/roadmap } |  |  | 0.437 |
| walker |  | 239 | 9 | Fs::DirListing { dir: benches } |  |  | 0.437 |
| walker |  | 250 | 11 | Fs::DirListing { dir: tests } |  |  | 0.438 |
| walker |  | 287 | 37 | Code::CodeKey { rung: Names, file: examples/hello-toasty/src/main.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.438 |
| walker |  | 297 | 10 | Code::CodeKey { rung: Decl, file: examples/hello-toasty/src/main.rs, decl: 3, sub: 0, line: 33 } |  |  | 0.438 |
| walker |  | 308 | 11 | Fs::DirListing { dir: scripts } |  |  | 0.438 |
| ns | 321 |  | 101 | README incubating-status caveat and section headings | 1.4 |  | 0.395 |
| walker |  | 393 | 85 | Fs::DirListing { dir: crates } |  |  | 0.603 |
| walker |  | 401 | 8 | Fs::DirListing { dir: crates/std-util } |  |  | 0.603 |
| ns | 407 |  | 86 | docs/ tree: every documentation file (complete) | 1.5 |  | 0.615 |
| walker |  | 409 | 8 | Fs::DirListing { dir: crates/toasty-cli } |  |  | 0.615 |
| walker |  | 417 | 8 | Fs::DirListing { dir: crates/toasty-driver-dynamodb } |  |  | 0.615 |
| walker |  | 425 | 8 | Fs::DirListing { dir: crates/toasty-driver-integration-suite } |  |  | 0.615 |
| walker |  | 433 | 8 | Fs::DirListing { dir: crates/toasty-driver-integration-suite-macros } |  |  | 0.615 |
| walker |  | 441 | 8 | Fs::DirListing { dir: crates/toasty-driver-mysql } |  |  | 0.615 |
| walker |  | 449 | 8 | Fs::DirListing { dir: crates/toasty-driver-postgresql } |  |  | 0.615 |
| walker |  | 457 | 8 | Fs::DirListing { dir: crates/toasty-macros } |  |  | 0.615 |
| walker |  | 461 | 4 | Fs::DirListing { dir: crates/toasty-macros/src } |  |  | 0.615 |
| walker |  | 474 | 13 | Fs::DirListing { dir: crates/toasty-codegen } |  |  | 0.615 |
| walker |  | 492 | 18 | Fs::DirListing { dir: crates/toasty-codegen/src } |  |  | 0.615 |
| walker |  | 505 | 13 | Fs::DirListing { dir: crates/toasty-driver-sqlite } |  |  | 0.615 |
| walker |  | 513 | 8 | Fs::DirListing { dir: crates/toasty-driver-mysql/src } |  |  | 0.615 |
| walker |  | 521 | 8 | Fs::DirListing { dir: crates/toasty-driver-sqlite/src } |  |  | 0.616 |
| walker |  | 537 | 16 | Fs::DirListing { dir: crates/toasty } |  |  | 0.616 |
| ns | 538 |  | 131 | toasty crate public exports, first half of src/lib.rs | 1.6 |  | 0.532 |
| walker |  | 553 | 16 | Fs::DirListing { dir: crates/toasty-core } |  |  | 0.532 |
| walker |  | 569 | 16 | Fs::DirListing { dir: crates/toasty-sql } |  |  | 0.532 |
| walker |  | 591 | 22 | Fs::DirListing { dir: crates/toasty-sql/src } |  |  | 0.532 |
| walker |  | 627 | 36 | Fs::DirListing { dir: crates/toasty-codegen/src/expand } |  |  | 0.533 |
| walker |  | 663 | 36 | Fs::DirListing { dir: crates/toasty-core/src } |  |  | 0.534 |
| walker |  | 678 | 15 | Fs::DirListing { dir: crates/toasty-core/src/driver } |  |  | 0.534 |
| walker |  | 717 | 39 | Fs::DirListing { dir: crates/toasty-core/src/schema } |  |  | 0.536 |
| ns | 719 |  | 181 | toasty crate public exports, remainder of src/lib.rs | 1.7 | 1.6 | 0.458 |
| walker |  | 721 | 4 | Fs::DirListing { dir: crates/toasty-core/src/schema/builder } |  |  | 0.458 |
| walker |  | 728 | 7 | Fs::DirListing { dir: crates/toasty-core/src/schema/verify } |  |  | 0.459 |
| walker |  | 736 | 8 | Fs::DirListing { dir: crates/toasty-core/src/schema/mapping } |  |  | 0.459 |
| walker |  | 768 | 32 | Fs::DirListing { dir: crates/toasty-core/src/schema/db } |  |  | 0.460 |
| walker |  | 811 | 43 | Fs::DirListing { dir: crates/toasty-core/src/driver/operation } |  |  | 0.462 |
| walker |  | 864 | 53 | Fs::DirListing { dir: crates/toasty-core/src/schema/app } |  |  | 0.466 |
| walker |  | 868 | 4 | Fs::DirListing { dir: crates/toasty-core/src/schema/app/constraint } |  |  | 0.466 |
| walker |  | 872 | 4 | Fs::DirListing { dir: crates/toasty-core/src/schema/app/field } |  |  | 0.467 |
| walker |  | 887 | 15 | Fs::DirListing { dir: crates/toasty-core/src/schema/app/relation } |  |  | 0.467 |
| ns | 892 |  | 173 | crates/toasty source tree (complete) | 2.1 |  | 0.378 |
| walker |  | 941 | 54 | Fs::DirListing { dir: crates/toasty-sql/src/serializer } |  |  | 0.379 |
| walker |  | 1001 | 60 | Fs::DirListing { dir: crates/toasty/src } |  |  | 0.422 |
| walker |  | 1005 | 4 | Fs::DirListing { dir: crates/toasty/src/batch } |  |  | 0.427 |
| walker |  | 1017 | 12 | Fs::DirListing { dir: crates/toasty/src/db } |  |  | 0.441 |
| walker |  | 1036 | 19 | Fs::DirListing { dir: crates/toasty/src/relation } |  |  | 0.463 |
| walker |  | 1097 | 61 | Fs::DirListing { dir: crates/toasty-codegen/src/schema } |  |  | 0.465 |
| ns | 1126 |  | 234 | Complete method roster of `Db` (src/db.rs) | 2.2 |  | 0.435 |
| walker |  | 1159 | 62 | Fs::DirListing { dir: crates/toasty/src/stmt } |  |  | 0.526 |
| walker |  | 1176 | 17 | Fs::DirListing { dir: crates/toasty-driver-postgresql/src } |  |  | 0.526 |
| walker |  | 1241 | 65 | Fs::DirListing { dir: crates/toasty/src/engine } |  |  | 0.527 |
| walker |  | 1246 | 5 | Fs::DirListing { dir: crates/toasty/src/engine/eval } |  |  | 0.527 |
| ns | 1251 |  | 125 | Doc comments on the `Db` methods | 2.3 |  | 0.508 |
| walker |  | 1256 | 10 | Fs::DirListing { dir: crates/toasty/src/engine/index } |  |  | 0.508 |
| walker |  | 1269 | 13 | Fs::DirListing { dir: crates/toasty/src/engine/plan } |  |  | 0.508 |
| walker |  | 1285 | 16 | Fs::DirListing { dir: crates/toasty/src/engine/lower } |  |  | 0.508 |
| walker |  | 1356 | 71 | Fs::DirListing { dir: crates/toasty-sql/src/stmt } |  |  | 0.510 |
| walker |  | 1375 | 19 | Fs::DirListing { dir: crates/toasty-cli/src } |  |  | 0.511 |
| walker |  | 1409 | 34 | Fs::DirListing { dir: crates/toasty-cli/src/migration } |  |  | 0.512 |
| walker |  | 1428 | 19 | Fs::DirListing { dir: crates/toasty-driver-dynamodb/src } |  |  | 0.512 |
| ns | 1447 |  | 196 | `Db::builder()` — the complete `Builder` API (src/db/builder.rs) | 2.4 |  | 0.491 |
| walker |  | 1467 | 39 | Fs::DirListing { dir: crates/toasty-driver-dynamodb/src/op } |  |  | 0.493 |
| walker |  | 1545 | 78 | Fs::DirListing { dir: crates/toasty/src/engine/exec } |  |  | 0.494 |
| ns | 1606 |  | 159 | `Register`, `Model` and `Embed` trait signatures (src/model.rs) | 2.5 |  | 0.473 |
| walker |  | 1624 | 79 | Fs::DirListing { dir: crates/toasty/src/engine/mir } |  |  | 0.475 |
| ns | 1679 |  | 73 | What an embedded type is (model.rs doc on `Embed`) | 2.6 | 2.5 | 0.468 |
| walker |  | 1706 | 82 | Fs::DirListing { dir: crates/toasty-core/src/error } |  |  | 0.472 |
| walker |  | 1730 | 24 | Fs::DirListing { dir: crates/std-util/src } |  |  | 0.472 |
| walker |  | 1772 | 42 | Code::CodeKey { rung: Names, file: crates/std-util/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.472 |
| walker |  | 1796 | 24 | Fs::DirListing { dir: crates/toasty-driver-integration-suite-macros/src } |  |  | 0.472 |
| ns | 1895 |  | 216 | The derive macros and their attributes (toasty-macros/src/lib.rs) | 2.7 |  | 0.451 |
| walker |  | 2059 | 263 | Code::CodeKey { rung: Names, file: crates/toasty/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.547 |
| ns | 2121 |  | 226 | Typed statement builder exports (crates/toasty/src/stmt.rs) | 2.8 |  | 0.522 |
| walker |  | 2162 | 103 | Fs::DirListing { dir: crates/toasty/src/engine/simplify } |  |  | 0.525 |
| walker |  | 2224 | 62 | Code::CodeKey { rung: Names, file: crates/toasty-codegen/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.525 |
| walker |  | 2291 | 67 | Code::CodeKey { rung: Names, file: crates/toasty-sql/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.526 |
| ns | 2334 |  | 213 | `Path<T>` filter and ordering operators (src/stmt/path.rs) | 2.9 |  | 0.506 |
| walker |  | 2359 | 68 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.535 |
| walker |  | 2437 | 78 | Code::CodeKey { rung: Names, file: crates/toasty-driver-sqlite/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.535 |
| walker |  | 2460 | 23 | Code::CodeKey { rung: Decl, file: crates/toasty-driver-sqlite/src/lib.rs, decl: 13, sub: 0, line: 122 } |  |  | 0.535 |
| walker |  | 2488 | 28 | Code::CodeKey { rung: Decl, file: crates/toasty-driver-sqlite/src/lib.rs, decl: 1, sub: 0, line: 22 } |  |  | 0.535 |
| walker |  | 2533 | 45 | Code::CodeKey { rung: Decl, file: crates/toasty-driver-sqlite/src/lib.rs, decl: 14, sub: 0, line: 127 } |  |  | 0.535 |
| walker |  | 2599 | 66 | Code::CodeKey { rung: Decl, file: crates/toasty-driver-sqlite/src/lib.rs, decl: 2, sub: 0, line: 28 } |  |  | 0.535 |
| ns | 2657 |  | 323 | `Select<M>` query builder and `Expr<T>` combinators | 2.10 |  | 0.509 |
| walker |  | 2673 | 74 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.666 |
| walker |  | 2758 | 85 | Code::CodeKey { rung: Names, file: crates/toasty-cli/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.666 |
| walker |  | 2779 | 21 | Code::CodeKey { rung: Decl, file: crates/toasty-cli/src/lib.rs, decl: 1, sub: 0, line: 13 } |  |  | 0.666 |
| walker |  | 2790 | 11 | Code::CodeKey { rung: Names, file: crates/toasty/src/engine/ty.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.666 |
| walker |  | 2884 | 94 | Code::CodeKey { rung: Names, file: crates/toasty-driver-mysql/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.666 |
| walker |  | 2904 | 20 | Code::CodeKey { rung: Decl, file: crates/toasty-driver-mysql/src/lib.rs, decl: 10, sub: 0, line: 132 } |  |  | 0.666 |
| ns | 2919 |  | 262 | The `Relation` trait and the three relation wrapper types | 2.11 |  | 0.636 |
| walker |  | 2925 | 21 | Code::CodeKey { rung: Decl, file: crates/toasty-driver-mysql/src/lib.rs, decl: 14, sub: 0, line: 184 } |  |  | 0.636 |
| walker |  | 2951 | 26 | Code::CodeKey { rung: Decl, file: crates/toasty-driver-mysql/src/lib.rs, decl: 2, sub: 0, line: 27 } |  |  | 0.636 |
| walker |  | 2980 | 29 | Code::CodeKey { rung: Decl, file: crates/toasty-driver-mysql/src/lib.rs, decl: 1, sub: 0, line: 21 } |  |  | 0.636 |
| walker |  | 3030 | 50 | Code::CodeKey { rung: Decl, file: crates/toasty-driver-mysql/src/lib.rs, decl: 11, sub: 0, line: 137 } |  |  | 0.636 |
| ns | 3118 |  | 199 | Result streaming: `Cursor<M>` and `Page<M>` | 2.12 |  | 0.621 |
| walker |  | 3119 | 89 | Code::CodeKey { rung: Decl, file: crates/toasty-driver-mysql/src/lib.rs, decl: 16, sub: 0, line: 190 } |  |  | 0.621 |
| walker |  | 3150 | 31 | Code::CodeKey { rung: Decl, file: crates/toasty-driver-mysql/src/lib.rs, decl: 19, sub: 0, line: 329 } |  |  | 0.621 |
| walker |  | 3168 | 18 | Code::CodeKey { rung: Names, file: crates/toasty/src/batch.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.621 |
| walker |  | 3184 | 16 | Code::CodeKey { rung: Doc, file: crates/toasty-cli/src/lib.rs, decl: 1, sub: 0, line: 13 } |  |  | 0.621 |
| walker |  | 3282 | 98 | Code::CodeKey { rung: Names, file: crates/toasty-macros/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.625 |
| walker |  | 3290 | 8 | Code::CodeKey { rung: Decl, file: crates/toasty-macros/src/lib.rs, decl: 3, sub: 0, line: 25 } |  |  | 0.626 |
| walker |  | 3298 | 8 | Code::CodeKey { rung: Decl, file: crates/toasty-macros/src/lib.rs, decl: 4, sub: 0, line: 30 } |  |  | 0.627 |
| walker |  | 3306 | 8 | Code::CodeKey { rung: Decl, file: crates/toasty-macros/src/lib.rs, decl: 5, sub: 0, line: 35 } |  |  | 0.629 |
| walker |  | 3321 | 15 | Code::CodeKey { rung: Decl, file: crates/toasty-macros/src/lib.rs, decl: 2, sub: 0, line: 17 } |  |  | 0.632 |
| ns | 3353 |  | 235 | Write-side builders: `CreateMany` and `Association` linking | 2.13 |  | 0.620 |
| walker |  | 3370 | 49 | Code::CodeKey { rung: Decl, file: crates/toasty-macros/src/lib.rs, decl: 1, sub: 0, line: 6 } |  |  | 0.631 |
| walker |  | 3419 | 49 | Fs::DirListing { dir: crates/toasty-driver-integration-suite/src } |  |  | 0.632 |
| ns | 3475 |  | 122 | examples/ tree, all four examples (complete) | 3.1 |  | 0.606 |
| walker |  | 3514 | 95 | Code::CodeKey { rung: Decl, file: crates/toasty-driver-sqlite/src/lib.rs, decl: 17, sub: 0, line: 142 } |  |  | 0.606 |
| walker |  | 3545 | 31 | Code::CodeKey { rung: Decl, file: crates/toasty-driver-sqlite/src/lib.rs, decl: 20, sub: 0, line: 256 } |  |  | 0.606 |
| walker |  | 3558 | 13 | Code::CodeKey { rung: ModuleDoc, file: crates/toasty/src/db/pool.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.606 |
| walker |  | 3597 | 39 | Markdown::HeadingsOutline { file: docs/CONTEXT.md } |  |  | 0.606 |
| ns | 3742 |  | 267 | Canonical model definitions (examples/hello-toasty/src/main.rs:1-31) | 3.2 |  | 0.577 |
| walker |  | 3791 | 194 | Fs::DirListing { dir: crates/toasty-driver-integration-suite/src/tests } |  |  | 0.578 |
| ns | 3894 |  | 152 | Opening a database: builder, connection URL, push_schema | 3.3 |  | 0.567 |
| walker |  | 3899 | 108 | Code::CodeKey { rung: Names, file: crates/toasty-driver-postgresql/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.567 |
| walker |  | 3920 | 21 | Code::CodeKey { rung: Decl, file: crates/toasty-driver-postgresql/src/lib.rs, decl: 15, sub: 0, line: 258 } |  |  | 0.567 |
| walker |  | 3948 | 28 | Code::CodeKey { rung: Decl, file: crates/toasty-driver-postgresql/src/lib.rs, decl: 2, sub: 0, line: 29 } |  |  | 0.567 |
| walker |  | 3977 | 29 | Code::CodeKey { rung: Decl, file: crates/toasty-driver-postgresql/src/lib.rs, decl: 1, sub: 0, line: 23 } |  |  | 0.567 |
| walker |  | 4008 | 31 | Code::CodeKey { rung: Decl, file: crates/toasty-driver-postgresql/src/lib.rs, decl: 10, sub: 0, line: 177 } |  |  | 0.567 |
| walker |  | 4085 | 77 | Code::CodeKey { rung: Decl, file: crates/toasty-driver-postgresql/src/lib.rs, decl: 11, sub: 0, line: 183 } |  |  | 0.567 |
| ns | 4116 |  | 222 | CRUD and relation walkthrough (hello-toasty main body) | 3.4 |  | 0.554 |
| walker |  | 4126 | 41 | Code::CodeKey { rung: Decl, file: crates/toasty-driver-postgresql/src/lib.rs, decl: 13, sub: 0, line: 195 } |  |  | 0.554 |
| walker |  | 4215 | 89 | Code::CodeKey { rung: Decl, file: crates/toasty-driver-postgresql/src/lib.rs, decl: 17, sub: 0, line: 267 } |  |  | 0.554 |
| walker |  | 4246 | 31 | Code::CodeKey { rung: Decl, file: crates/toasty-driver-postgresql/src/lib.rs, decl: 20, sub: 0, line: 355 } |  |  | 0.554 |
| walker |  | 4259 | 13 | Code::CodeKey { rung: Names, file: crates/toasty/src/stmt/into_insert.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.554 |
| walker |  | 4272 | 13 | Code::CodeKey { rung: Names, file: crates/toasty/src/stmt/to_statement.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.554 |
| walker |  | 4280 | 8 | Code::CodeKey { rung: Body, file: crates/toasty-macros/src/lib.rs, decl: 3, sub: 0, line: 25 } |  |  | 0.554 |
| walker |  | 4336 | 56 | Code::CodeKey { rung: Decl, file: crates/toasty-driver-mysql/src/lib.rs, decl: 20, sub: 0, line: 358 } |  |  | 0.554 |
| ns | 4343 |  | 227 | Composite keys and has_one: the two variant example models | 3.5 |  | 0.537 |
| walker |  | 4392 | 56 | Code::CodeKey { rung: Decl, file: crates/toasty-driver-postgresql/src/lib.rs, decl: 21, sub: 0, line: 390 } |  |  | 0.537 |
| walker |  | 4448 | 56 | Code::CodeKey { rung: Decl, file: crates/toasty-driver-sqlite/src/lib.rs, decl: 21, sub: 0, line: 288 } |  |  | 0.537 |
| ns | 4547 |  | 204 | todo-with-cli: `create_db`, the CLI binary, and Toasty.toml | 3.6 |  | 0.521 |
| walker |  | 4558 | 110 | Code::CodeKey { rung: Decl, file: crates/toasty-cli/src/lib.rs, decl: 2, sub: 0, line: 18 } |  |  | 0.521 |
| walker |  | 4602 | 44 | Code::CodeKey { rung: Decl, file: crates/toasty-cli/src/lib.rs, decl: 7, sub: 0, line: 44 } |  |  | 0.521 |
| walker |  | 4725 | 123 | Code::CodeKey { rung: Names, file: crates/toasty-driver-integration-suite-macros/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.521 |
| walker |  | 4733 | 8 | Code::CodeKey { rung: Decl, file: crates/toasty-driver-integration-suite-macros/src/lib.rs, decl: 2, sub: 0, line: 15 } |  |  | 0.521 |
| walker |  | 4741 | 8 | Code::CodeKey { rung: Decl, file: crates/toasty-driver-integration-suite-macros/src/lib.rs, decl: 3, sub: 0, line: 20 } |  |  | 0.521 |
| walker |  | 4749 | 8 | Code::CodeKey { rung: Decl, file: crates/toasty-driver-integration-suite-macros/src/lib.rs, decl: 4, sub: 0, line: 27 } |  |  | 0.521 |
| walker |  | 4758 | 9 | Code::CodeKey { rung: Decl, file: crates/toasty-driver-integration-suite-macros/src/lib.rs, decl: 1, sub: 0, line: 10 } |  |  | 0.521 |
| walker |  | 4769 | 11 | Code::CodeKey { rung: Body, file: crates/toasty-driver-integration-suite-macros/src/lib.rs, decl: 2, sub: 0, line: 15 } |  |  | 0.521 |
| walker |  | 4807 | 38 | Markdown::ReadmeHeadline { file: docs/roadmap/README.md } |  |  | 0.521 |
| ns | 4916 |  | 369 | toasty-core source tree: driver, error, schema (complete) | 4.1 |  | 0.595 |
| ns | 5075 |  | 159 | toasty-core public surface (src/lib.rs, complete file) | 4.2 |  | 0.583 |
| walker |  | 5162 | 355 | Toml::Identity { file: Cargo.toml } |  |  | 0.583 |
| walker |  | 5197 | 35 | Code::CodeKey { rung: Decl, file: crates/toasty-cli/src/lib.rs, decl: 9, sub: 0, line: 69 } |  |  | 0.583 |
| ns | 5306 |  | 231 | The `Schema` triple: app schema, db schema, mapping | 4.3 |  | 0.569 |
| walker |  | 5317 | 120 | Code::CodeKey { rung: Decl, file: crates/toasty-driver-mysql/src/lib.rs, decl: 4, sub: 0, line: 62 } |  |  | 0.569 |
| walker |  | 5441 | 124 | Code::CodeKey { rung: Decl, file: crates/toasty-driver-postgresql/src/lib.rs, decl: 4, sub: 0, line: 79 } |  |  | 0.569 |
| ns | 5567 |  | 261 | The `Driver` trait — the database extension point | 4.4 |  | 0.559 |
| walker |  | 5579 | 138 | Code::CodeKey { rung: Names, file: crates/toasty-core/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.576 |
| walker |  | 5595 | 16 | Code::CodeKey { rung: Doc, file: crates/toasty-core/src/lib.rs, decl: 1, sub: 0, line: 16 } |  |  | 0.581 |
| ns | 5750 |  | 183 | The `Connection` trait: exec, push_schema, migrations | 4.5 |  | 0.574 |
| walker |  | 5771 | 176 | Code::CodeKey { rung: Names, file: crates/toasty-driver-integration-suite/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.574 |
| walker |  | 5788 | 17 | Code::CodeKey { rung: Names, file: crates/toasty/src/stmt/primitive_jiff.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.574 |
| ns | 5869 |  | 119 | The eight driver `Operation` enum variants | 4.6 |  | 0.569 |
| walker |  | 5929 | 141 | Code::CodeKey { rung: Decl, file: crates/toasty-driver-sqlite/src/lib.rs, decl: 6, sub: 0, line: 59 } |  |  | 0.569 |
| walker |  | 6121 | 192 | Code::CodeKey { rung: Names, file: crates/toasty-driver-dynamodb/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.569 |
| walker |  | 6141 | 20 | Code::CodeKey { rung: Decl, file: crates/toasty-driver-dynamodb/src/lib.rs, decl: 1, sub: 0, line: 29 } |  |  | 0.569 |
| walker |  | 6163 | 22 | Code::CodeKey { rung: Decl, file: crates/toasty-driver-dynamodb/src/lib.rs, decl: 2, sub: 0, line: 34 } |  |  | 0.569 |
| walker |  | 6196 | 33 | Code::CodeKey { rung: Decl, file: crates/toasty-driver-dynamodb/src/lib.rs, decl: 10, sub: 0, line: 95 } |  |  | 0.569 |
| walker |  | 6257 | 61 | Code::CodeKey { rung: Decl, file: crates/toasty-driver-dynamodb/src/lib.rs, decl: 11, sub: 0, line: 101 } |  |  | 0.569 |
| ns | 6278 |  | 409 | The statement AST node roster (crates/toasty-core/src/stmt/, complete) | 4.7 |  | 0.529 |
| walker |  | 6348 | 91 | Code::CodeKey { rung: Decl, file: crates/toasty-driver-dynamodb/src/lib.rs, decl: 15, sub: 0, line: 156 } |  |  | 0.529 |
| walker |  | 6379 | 31 | Code::CodeKey { rung: Decl, file: crates/toasty-driver-dynamodb/src/lib.rs, decl: 18, sub: 0, line: 169 } |  |  | 0.529 |
| ns | 6405 |  | 127 | The top-level `Statement` enum (toasty-core/src/stmt.rs) | 4.8 |  | 0.523 |
| walker |  | 6438 | 59 | Code::CodeKey { rung: Decl, file: crates/toasty-driver-dynamodb/src/lib.rs, decl: 19, sub: 0, line: 175 } |  |  | 0.523 |
| walker |  | 6563 | 125 | Code::CodeKey { rung: Decl, file: crates/toasty-driver-dynamodb/src/lib.rs, decl: 4, sub: 0, line: 40 } |  |  | 0.523 |
| walker |  | 6582 | 19 | Code::CodeKey { rung: Names, file: crates/toasty/src/relation/option.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.523 |
| walker |  | 6626 | 44 | Code::CodeKey { rung: Decl, file: crates/toasty-driver-dynamodb/src/lib.rs, decl: 22, sub: 0, line: 247 } |  |  | 0.523 |
| walker |  | 6638 | 12 | Code::CodeKey { rung: Body, file: crates/toasty-driver-integration-suite-macros/src/lib.rs, decl: 3, sub: 0, line: 20 } |  |  | 0.523 |
| walker |  | 6652 | 14 | Code::CodeKey { rung: Names, file: crates/toasty/src/engine/exec/output.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.515 |
| ns | 6652 |  | 247 | `Capability`: the complete feature-advertisement field set | 4.9 |  | 0.515 |
| walker |  | 6673 | 21 | Code::CodeKey { rung: Names, file: crates/toasty/src/db/builder.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.515 |
| walker |  | 6696 | 23 | Fs::DirListing { dir: examples/todo-with-cli } |  |  | 0.519 |
| walker |  | 6703 | 7 | Fs::DirListing { dir: examples/todo-with-cli/src } |  |  | 0.522 |
| walker |  | 6718 | 15 | Code::CodeKey { rung: Names, file: crates/toasty/src/engine/exec/plan.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.522 |
| walker |  | 6733 | 15 | Code::CodeKey { rung: Names, file: crates/toasty/src/engine/plan/execution.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.522 |
| walker |  | 6748 | 15 | Code::CodeKey { rung: Names, file: crates/toasty/src/engine/simplify/association.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.522 |
| walker |  | 6763 | 15 | Code::CodeKey { rung: Names, file: crates/toasty/src/engine/simplify/expr_and.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.522 |
| walker |  | 6778 | 15 | Code::CodeKey { rung: Names, file: crates/toasty/src/engine/simplify/expr_any.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.522 |
| walker |  | 6793 | 15 | Code::CodeKey { rung: Names, file: crates/toasty/src/engine/simplify/expr_binary_op.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.522 |
| walker |  | 6808 | 15 | Code::CodeKey { rung: Names, file: crates/toasty/src/engine/simplify/expr_cast.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.522 |
| walker |  | 6823 | 15 | Code::CodeKey { rung: Names, file: crates/toasty/src/engine/simplify/expr_concat_str.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.522 |
| walker |  | 6838 | 15 | Code::CodeKey { rung: Names, file: crates/toasty/src/engine/simplify/expr_exists.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.522 |
| walker |  | 6853 | 15 | Code::CodeKey { rung: Names, file: crates/toasty/src/engine/simplify/expr_in_list.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.522 |
| walker |  | 6868 | 15 | Code::CodeKey { rung: Names, file: crates/toasty/src/engine/simplify/expr_is_null.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.522 |
| walker |  | 6883 | 15 | Code::CodeKey { rung: Names, file: crates/toasty/src/engine/simplify/expr_list.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.522 |
| walker |  | 6898 | 15 | Code::CodeKey { rung: Names, file: crates/toasty/src/engine/simplify/expr_map.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.522 |
| ns | 6902 |  | 250 | `StorageTypes`, `SchemaMutations`, and the four per-database presets | 4.10 |  | 0.515 |
| walker |  | 6913 | 15 | Code::CodeKey { rung: Names, file: crates/toasty/src/engine/simplify/expr_not.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.515 |
| walker |  | 6928 | 15 | Code::CodeKey { rung: Names, file: crates/toasty/src/engine/simplify/expr_or.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.515 |
| walker |  | 6943 | 15 | Code::CodeKey { rung: Names, file: crates/toasty/src/engine/simplify/expr_project.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.515 |
| walker |  | 6958 | 15 | Code::CodeKey { rung: Names, file: crates/toasty/src/engine/simplify/expr_record.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.515 |
| walker |  | 6973 | 15 | Code::CodeKey { rung: Names, file: crates/toasty/src/engine/simplify/lift_pk_select.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.515 |
| walker |  | 6988 | 15 | Code::CodeKey { rung: Names, file: crates/toasty/src/engine/simplify/rewrite_root_path_expr.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.515 |
| walker |  | 7003 | 15 | Code::CodeKey { rung: Names, file: crates/toasty/src/engine/simplify/stmt_query.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.515 |
| walker |  | 7016 | 13 | Code::CodeKey { rung: Names, file: crates/toasty-core/src/macros.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.515 |
| walker |  | 7032 | 16 | Code::CodeKey { rung: Names, file: crates/toasty/src/engine/lower/paginate.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.515 |
| walker |  | 7087 | 55 | Code::CodeKey { rung: Decl, file: crates/toasty-driver-dynamodb/src/lib.rs, decl: 23, sub: 0, line: 264 } |  |  | 0.515 |
| ns | 7104 |  | 202 | The fifteen `ErrorKind` variants (toasty-core/src/error.rs) | 4.11 |  | 0.508 |
| walker |  | 7125 | 38 | Code::CodeKey { rung: Names, file: crates/toasty/src/apply_update.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.508 |
| walker |  | 7137 | 12 | Code::CodeKey { rung: Decl, file: crates/toasty/src/apply_update.rs, decl: 3, sub: 0, line: 18 } |  |  | 0.508 |
| walker |  | 7162 | 25 | Code::CodeKey { rung: Decl, file: crates/toasty/src/apply_update.rs, decl: 4, sub: 0, line: 21 } |  |  | 0.508 |
| walker |  | 7189 | 27 | Code::CodeKey { rung: Decl, file: crates/toasty/src/apply_update.rs, decl: 1, sub: 0, line: 9 } |  |  | 0.508 |
| walker |  | 7246 | 57 | Code::CodeKey { rung: Decl, file: crates/toasty-driver-dynamodb/src/lib.rs, decl: 24, sub: 0, line: 344 } |  |  | 0.508 |
| walker |  | 7260 | 14 | Code::CodeKey { rung: Names, file: crates/std-util/src/option.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.508 |
| ns | 7473 |  | 369 | crates/toasty/src/engine tree (complete) | 5.1 |  | 0.553 |
| walker |  | 7669 | 409 | Fs::DirListing { dir: crates/toasty-core/src/stmt } |  |  | 0.628 |
| walker |  | 7696 | 27 | Code::CodeKey { rung: Names, file: crates/toasty/src/engine/kv.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.628 |
| walker |  | 7717 | 21 | Markdown::Section { file: docs/CONTEXT.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.628 |
| walker |  | 7728 | 11 | Markdown::Section { file: docs/roadmap/composite-keys.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.628 |
| walker |  | 7737 | 9 | Code::CodeKey { rung: Body, file: crates/toasty-cli/src/lib.rs, decl: 5, sub: 0, line: 33 } |  |  | 0.628 |
| ns | 7756 |  | 283 | `Engine`: the four-phase pipeline doc and `exec` entry point | 5.2 |  | 0.621 |
| walker |  | 7781 | 44 | Code::CodeKey { rung: Names, file: crates/toasty/src/schema.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.621 |
| walker |  | 7792 | 11 | Code::CodeKey { rung: ModuleDoc, file: crates/toasty-core/src/schema/app.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.621 |
| walker |  | 7837 | 45 | Code::CodeKey { rung: Names, file: crates/toasty/src/page.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.621 |
| ns | 7872 |  | 116 | engine.rs module roster and private-module boundary | 5.3 |  | 0.614 |
| walker |  | 7875 | 38 | Code::CodeKey { rung: Decl, file: crates/toasty/src/page.rs, decl: 8, sub: 0, line: 107 } |  |  | 0.614 |
| walker |  | 7895 | 20 | Toml::Dependencies { file: crates/toasty-sql/Cargo.toml } |  |  | 0.614 |
| walker |  | 7908 | 13 | Code::CodeKey { rung: Body, file: crates/toasty-driver-integration-suite-macros/src/lib.rs, decl: 1, sub: 0, line: 10 } |  |  | 0.614 |
| walker |  | 7921 | 13 | Code::CodeKey { rung: Body, file: crates/toasty-macros/src/lib.rs, decl: 4, sub: 0, line: 30 } |  |  | 0.616 |
| walker |  | 7991 | 70 | Code::CodeKey { rung: Decl, file: crates/toasty-cli/src/lib.rs, decl: 8, sub: 0, line: 60 } |  |  | 0.616 |
| walker |  | 8002 | 11 | Code::CodeKey { rung: Names, file: crates/toasty-core/src/stmt/ty_jiff.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.616 |
| walker |  | 8013 | 11 | Code::CodeKey { rung: Names, file: crates/toasty-driver-dynamodb/src/op/create_table.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.616 |
| walker |  | 8024 | 11 | Code::CodeKey { rung: Names, file: crates/toasty-driver-dynamodb/src/op/delete_by_key.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.616 |
| walker |  | 8035 | 11 | Code::CodeKey { rung: Names, file: crates/toasty-driver-dynamodb/src/op/find_pk_by_index.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.616 |
| walker |  | 8046 | 11 | Code::CodeKey { rung: Names, file: crates/toasty-driver-dynamodb/src/op/get_by_key.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.616 |
| walker |  | 8057 | 11 | Code::CodeKey { rung: Names, file: crates/toasty-driver-dynamodb/src/op/insert.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.616 |
| walker |  | 8068 | 11 | Code::CodeKey { rung: Names, file: crates/toasty-driver-dynamodb/src/op/query_pk.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.616 |
| walker |  | 8079 | 11 | Code::CodeKey { rung: Names, file: crates/toasty-driver-dynamodb/src/op/update_by_key.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.616 |
| ns | 8097 |  | 225 | The engine as a mini-program: query-engine.md execution model | 5.4 |  | 0.611 |
| walker |  | 8110 | 31 | Code::CodeKey { rung: Names, file: crates/toasty/src/stmt/into_select.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.611 |
| walker |  | 8122 | 12 | Code::CodeKey { rung: Names, file: crates/toasty-core/src/stmt/association.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.611 |
| walker |  | 8134 | 12 | Code::CodeKey { rung: Names, file: crates/toasty-core/src/stmt/direction.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.611 |
| walker |  | 8146 | 12 | Code::CodeKey { rung: Names, file: crates/toasty-core/src/stmt/limit.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.611 |
| walker |  | 8158 | 12 | Code::CodeKey { rung: Names, file: crates/toasty-core/src/stmt/offset.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.611 |
| walker |  | 8171 | 13 | Code::CodeKey { rung: ModuleDoc, file: crates/toasty-driver-integration-suite/src/tests/one_model_batch_create.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.611 |
| walker |  | 8190 | 19 | Code::CodeKey { rung: Names, file: crates/toasty-cli/src/theme.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.611 |
| walker |  | 8209 | 19 | Code::CodeKey { rung: Names, file: crates/toasty-driver-integration-suite/src/setup.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.611 |
| ns | 8225 |  | 128 | toasty-codegen tree: attribute parsing and expansion (complete) | 6.1 |  | 0.621 |
| walker |  | 8241 | 32 | Code::CodeKey { rung: Decl, file: crates/toasty/src/stmt/to_statement.rs, decl: 1, sub: 0, line: 3 } |  |  | 0.621 |
| walker |  | 8264 | 23 | Code::CodeKey { rung: Names, file: crates/toasty/src/engine/mir/node.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.621 |
| walker |  | 8302 | 38 | Fs::DirListing { dir: crates/toasty-sql/tests } |  |  | 0.622 |
| walker |  | 8336 | 34 | Code::CodeKey { rung: Decl, file: crates/toasty/src/stmt/into_insert.rs, decl: 1, sub: 0, line: 4 } |  |  | 0.622 |
| walker |  | 8370 | 34 | Code::CodeKey { rung: Decl, file: crates/toasty/src/stmt/into_select.rs, decl: 1, sub: 0, line: 4 } |  |  | 0.622 |
| walker |  | 8383 | 13 | Code::CodeKey { rung: Names, file: crates/toasty-core/src/stmt/cte.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.622 |
| walker |  | 8396 | 13 | Code::CodeKey { rung: Names, file: crates/toasty-core/src/stmt/expr_pattern.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.622 |
| walker |  | 8409 | 13 | Code::CodeKey { rung: Names, file: crates/toasty-core/src/stmt/table_derived.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.622 |
| ns | 8412 |  | 187 | The codegen entry points: `generate_model` / `generate_embed` | 6.2 |  | 0.617 |
| walker |  | 8422 | 13 | Code::CodeKey { rung: Names, file: crates/toasty-core/src/stmt/table_factor.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.617 |
| walker |  | 8443 | 21 | Code::CodeKey { rung: Names, file: crates/toasty-cli/src/config.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.617 |
| walker |  | 8563 | 120 | Markdown::HeadingsOutline { file: docs/ARCHITECTURE.md } |  |  | 0.617 |
| walker |  | 8576 | 13 | Code::CodeKey { rung: Body, file: crates/toasty-macros/src/lib.rs, decl: 5, sub: 0, line: 35 } |  |  | 0.619 |
| walker |  | 8591 | 15 | Code::CodeKey { rung: ModuleDoc, file: crates/toasty-driver-integration-suite/src/tests/has_many_scoped_query.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.619 |
| walker |  | 8606 | 15 | Code::CodeKey { rung: ModuleDoc, file: crates/toasty-driver-integration-suite/src/tests/one_model_sort_limit.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.619 |
| walker |  | 8620 | 14 | Code::CodeKey { rung: Names, file: crates/toasty-codegen/src/expand/create.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.619 |
| walker |  | 8634 | 14 | Code::CodeKey { rung: Names, file: crates/toasty-codegen/src/expand/fields.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.619 |
| ns | 8638 |  | 226 | The inherent methods generated on every model | 6.3 |  | 0.615 |
| walker |  | 8648 | 14 | Code::CodeKey { rung: Names, file: crates/toasty-codegen/src/expand/model.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.615 |
| walker |  | 8662 | 14 | Code::CodeKey { rung: Names, file: crates/toasty-codegen/src/expand/query.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.615 |
| walker |  | 8676 | 14 | Code::CodeKey { rung: Names, file: crates/toasty-codegen/src/expand/relation.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.615 |
| walker |  | 8690 | 14 | Code::CodeKey { rung: Names, file: crates/toasty-codegen/src/expand/update.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.615 |
| walker |  | 8727 | 37 | Code::CodeKey { rung: Decl, file: crates/toasty/src/stmt/into_select.rs, decl: 3, sub: 0, line: 10 } |  |  | 0.615 |
| walker |  | 8754 | 27 | Code::CodeKey { rung: Names, file: crates/toasty/src/engine/exec/action.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.615 |
| walker |  | 8770 | 16 | Markdown::Section { file: crates/toasty-driver-sqlite/CONTEXT.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.615 |
| walker |  | 8793 | 23 | Code::CodeKey { rung: Names, file: crates/toasty-driver-integration-suite/src/suite.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.615 |
| walker |  | 8808 | 15 | Code::CodeKey { rung: Decl, file: crates/toasty-driver-integration-suite/src/suite.rs, decl: 1, sub: 0, line: 5 } |  |  | 0.615 |
| walker |  | 8824 | 16 | Code::CodeKey { rung: ModuleDoc, file: crates/toasty-driver-integration-suite/src/tests/has_many_link_unlink.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.615 |
| ns | 8839 |  | 201 | toasty-sql tree: serializer and DDL statement types (complete) | 6.4 |  | 0.628 |
| walker |  | 8840 | 16 | Code::CodeKey { rung: ModuleDoc, file: crates/toasty-driver-integration-suite/src/tests/one_model_query.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.628 |
| walker |  | 8868 | 28 | Code::CodeKey { rung: Names, file: crates/toasty/src/engine/mir/operation.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.628 |
| walker |  | 8887 | 19 | Code::CodeKey { rung: Decl, file: crates/toasty/src/engine/mir/operation.rs, decl: 2, sub: 0, line: 52 } |  |  | 0.628 |
| walker |  | 8902 | 15 | Code::CodeKey { rung: Names, file: crates/toasty-codegen/src/schema/pk.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.628 |
| ns | 8906 |  | 67 | toasty-sql public surface (src/lib.rs, complete file) | 6.5 |  | 0.631 |
| walker |  | 8917 | 15 | Code::CodeKey { rung: Names, file: crates/toasty-core/src/stmt/table_with_joins.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.631 |
| walker |  | 8940 | 23 | Code::CodeKey { rung: Decl, file: crates/toasty-driver-integration-suite/src/suite.rs, decl: 2, sub: 0, line: 9 } |  |  | 0.631 |
| walker |  | 8969 | 29 | Code::CodeKey { rung: Names, file: crates/toasty/src/engine/plan/nested_merge.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.631 |
| walker |  | 9026 | 57 | Code::CodeKey { rung: Body, file: crates/toasty-codegen/src/lib.rs, decl: 1, sub: 0, line: 6 } |  |  | 0.633 |
| ns | 9034 |  | 128 | The four driver crates and their source files (complete) | 6.6 |  | 0.641 |
| walker |  | 9083 | 57 | Code::CodeKey { rung: Body, file: crates/toasty-codegen/src/lib.rs, decl: 2, sub: 0, line: 13 } |  |  | 0.646 |
| walker |  | 9113 | 30 | Code::CodeKey { rung: Names, file: crates/toasty/src/engine/lower/insert.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.646 |
| walker |  | 9129 | 16 | Code::CodeKey { rung: Names, file: crates/toasty-codegen/src/schema/fk.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.646 |
| walker |  | 9160 | 31 | Code::CodeKey { rung: Names, file: crates/toasty/src/engine/index/index_plan.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.646 |
| ns | 9175 |  | 141 | The SQLite driver as the reference `Driver` implementation | 6.7 |  | 0.647 |
| walker |  | 9186 | 26 | Code::CodeKey { rung: Names, file: crates/std-util/src/result.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.647 |
| walker |  | 9204 | 18 | Code::CodeKey { rung: ModuleDoc, file: crates/toasty-driver-integration-suite/src/tests/has_many_n_1.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.647 |
| ns | 9236 |  | 61 | toasty-cli tree: the migration subcommand files (complete) | 6.8 |  | 0.651 |
| walker |  | 9240 | 36 | Markdown::Section { file: docs/ARCHITECTURE.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.651 |
| walker |  | 9257 | 17 | Code::CodeKey { rung: Names, file: crates/toasty-core/src/stmt/value_jiff.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.651 |
| walker |  | 9274 | 17 | Code::CodeKey { rung: Names, file: crates/toasty-sql/src/serializer/name.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.651 |
| walker |  | 9291 | 17 | Code::CodeKey { rung: Names, file: crates/toasty-sql/src/serializer/ty.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.651 |
| walker |  | 9318 | 27 | Code::CodeKey { rung: Names, file: crates/toasty-driver-dynamodb/src/type.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.651 |
| walker |  | 9341 | 23 | Code::CodeKey { rung: Decl, file: crates/toasty-driver-dynamodb/src/type.rs, decl: 1, sub: 0, line: 4 } |  |  | 0.651 |
| walker |  | 9364 | 23 | Code::CodeKey { rung: Decl, file: crates/toasty-driver-dynamodb/src/type.rs, decl: 3, sub: 0, line: 9 } |  |  | 0.651 |
| walker |  | 9391 | 27 | Code::CodeKey { rung: Names, file: crates/toasty-driver-integration-suite/src/macros.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.651 |
| walker |  | 9418 | 27 | Code::CodeKey { rung: Names, file: crates/toasty-driver-postgresql/src/type.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.651 |
| walker |  | 9439 | 21 | Code::CodeKey { rung: Decl, file: crates/toasty-driver-postgresql/src/type.rs, decl: 1, sub: 0, line: 4 } |  |  | 0.651 |
| ns | 9452 |  | 216 | The workspace `tests/` crate and `benches/` (complete) | 7.1 |  | 0.638 |
| walker |  | 9460 | 21 | Code::CodeKey { rung: Decl, file: crates/toasty-driver-postgresql/src/type.rs, decl: 3, sub: 0, line: 9 } |  |  | 0.638 |
| walker |  | 9479 | 19 | Code::CodeKey { rung: ModuleDoc, file: crates/toasty-driver-integration-suite/src/tests/has_many_crud_multi_relations.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.638 |
| walker |  | 9491 | 12 | Code::CodeKey { rung: Names, file: crates/toasty-core/src/schema/app/arg.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.638 |
| walker |  | 9568 | 77 | Code::CodeKey { rung: Names, file: crates/toasty/src/cursor.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.638 |
| walker |  | 9611 | 43 | Code::CodeKey { rung: Decl, file: crates/toasty/src/cursor.rs, decl: 1, sub: 0, line: 6 } |  |  | 0.638 |
| walker |  | 9629 | 18 | Code::CodeKey { rung: Names, file: crates/toasty-sql/src/serializer/column_def.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.638 |
| walker |  | 9705 | 76 | Code::CodeKey { rung: Decl, file: crates/toasty/src/cursor.rs, decl: 4, sub: 0, line: 16 } |  |  | 0.640 |
| walker |  | 9729 | 24 | Code::CodeKey { rung: Decl, file: crates/toasty/src/cursor.rs, decl: 7, sub: 0, line: 36 } |  |  | 0.640 |
| ns | 9735 |  | 283 | The driver integration suite and its test roster (complete) | 7.2 |  | 0.651 |
| walker |  | 9780 | 51 | Code::CodeKey { rung: Names, file: crates/toasty/src/batch/create.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.651 |
| walker |  | 9796 | 16 | Code::CodeKey { rung: Decl, file: crates/toasty/src/batch/create.rs, decl: 6, sub: 0, line: 46 } |  |  | 0.651 |
| walker |  | 9847 | 51 | Code::CodeKey { rung: Names, file: crates/toasty/src/stmt/delete.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.651 |
| ns | 9858 |  | 123 | The `toasty` crate's complete feature-flag set | 7.3 |  | 0.648 |
| walker |  | 9867 | 20 | Code::CodeKey { rung: Decl, file: crates/toasty/src/stmt/delete.rs, decl: 4, sub: 0, line: 20 } |  |  | 0.648 |
| walker |  | 9893 | 26 | Code::CodeKey { rung: Decl, file: crates/toasty/src/stmt/delete.rs, decl: 2, sub: 0, line: 11 } |  |  | 0.648 |
| walker |  | 9923 | 30 | Code::CodeKey { rung: Decl, file: crates/toasty/src/stmt/delete.rs, decl: 1, sub: 0, line: 6 } |  |  | 0.648 |

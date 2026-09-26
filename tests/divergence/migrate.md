Score(3000)=0.546 I=0.696 C=0.428 ns_rows≤3K=17/48 grid(1000/1442/2080/3000/4327/6240/9000)=0.719/0.644/0.622/0.546/0.518/0.533/0.546

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 51 |  | 51 | Repository identity: name + one-line definition | 1.1 |  | 0.000 |
| walker |  | 125 | 125 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 144 |  | 93 | FAQ codebase-layout block | 1.2 |  | 0.000 |
| walker |  | 176 | 51 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.538 |
| walker |  | 182 | 6 | Fs::DirListing { dir: internal } |  |  | 0.538 |
| walker |  | 193 | 11 | Fs::DirListing { dir: dktesting } |  |  | 0.538 |
| walker |  | 205 | 12 | Fs::DirListing { dir: cli } |  |  | 0.538 |
| walker |  | 214 | 9 | Fs::DirListing { dir: internal/url } |  |  | 0.538 |
| ns | 244 |  | 100 | Core design premise: dumb drivers, migrate glues | 1.3 |  | 0.413 |
| walker |  | 249 | 35 | GoMod::Identity { file: go.mod } |  |  | 0.413 |
| walker |  | 253 | 4 | Fs::DirListing { dir: .circleci } |  |  | 0.413 |
| walker |  | 270 | 17 | Fs::DirListing { dir: cmd/migrate } |  |  | 0.413 |
| walker |  | 277 | 7 | Fs::DirListing { dir: .github } |  |  | 0.413 |
| walker |  | 281 | 4 | Fs::DirListing { dir: .github/workflows } |  |  | 0.413 |
| ns | 323 |  | 79 | Root package doc comment (migrate.go) | 1.4 |  | 0.343 |
| walker |  | 370 | 89 | Fs::DirListing { dir: source } |  |  | 0.357 |
| walker |  | 379 | 9 | Fs::DirListing { dir: source/stub } |  |  | 0.357 |
| walker |  | 392 | 13 | Fs::DirListing { dir: source/file } |  |  | 0.357 |
| walker |  | 405 | 13 | Fs::DirListing { dir: source/google_cloud_storage } |  |  | 0.357 |
| walker |  | 420 | 15 | Fs::DirListing { dir: source/aws_s3 } |  |  | 0.357 |
| walker |  | 435 | 15 | Fs::DirListing { dir: source/pkger } |  |  | 0.357 |
| ns | 448 |  | 125 | Complete root directory listing | 1.5 |  | 0.677 |
| walker |  | 453 | 18 | Fs::DirListing { dir: source/godoc_vfs } |  |  | 0.677 |
| walker |  | 473 | 20 | Fs::DirListing { dir: source/bitbucket } |  |  | 0.677 |
| walker |  | 494 | 21 | Fs::DirListing { dir: source/github } |  |  | 0.677 |
| walker |  | 516 | 22 | Fs::DirListing { dir: source/github_ee } |  |  | 0.677 |
| walker |  | 539 | 23 | Fs::DirListing { dir: source/gitlab } |  |  | 0.677 |
| walker |  | 563 | 24 | Fs::DirListing { dir: source/go_bindata } |  |  | 0.677 |
| ns | 570 |  | 122 | Complete database/ listing — every database driver package | 1.6 |  | 0.521 |
| walker |  | 591 | 28 | Fs::DirListing { dir: source/httpfs } |  |  | 0.521 |
| walker |  | 619 | 28 | Fs::DirListing { dir: source/iofs } |  |  | 0.521 |
| ns | 659 |  | 89 | Complete source/ listing — every source driver package | 1.7 |  | 0.569 |
| walker |  | 691 | 72 | Code::CodeKey { rung: ModuleDoc, file: migrate.go, decl: 0, sub: 0, line: 0 } |  |  | 0.607 |
| walker |  | 704 | 13 | Fs::DirListing { dir: testing } |  |  | 0.607 |
| ns | 796 |  | 137 | Complete README section-heading roster | 1.8 |  | 0.567 |
| walker |  | 826 | 122 | Fs::DirListing { dir: database } |  |  | 0.733 |
| walker |  | 832 | 6 | Fs::DirListing { dir: database/crate } |  |  | 0.733 |
| walker |  | 838 | 6 | Fs::DirListing { dir: database/shell } |  |  | 0.733 |
| walker |  | 847 | 9 | Fs::DirListing { dir: database/multistmt } |  |  | 0.733 |
| walker |  | 856 | 9 | Fs::DirListing { dir: database/snowflake } |  |  | 0.733 |
| walker |  | 865 | 9 | Fs::DirListing { dir: database/stub } |  |  | 0.733 |
| walker |  | 881 | 16 | Fs::DirListing { dir: database/mongodb } |  |  | 0.733 |
| walker |  | 897 | 16 | Fs::DirListing { dir: database/mysql } |  |  | 0.733 |
| walker |  | 913 | 16 | Fs::DirListing { dir: database/sqlite } |  |  | 0.733 |
| ns | 917 |  | 121 | All four Migrate constructors (full signatures) | 2.1 |  | 0.719 |
| walker |  | 931 | 18 | Fs::DirListing { dir: database/cassandra } |  |  | 0.719 |
| walker |  | 949 | 18 | Fs::DirListing { dir: database/clickhouse } |  |  | 0.719 |
| walker |  | 967 | 18 | Fs::DirListing { dir: database/firebird } |  |  | 0.719 |
| walker |  | 985 | 18 | Fs::DirListing { dir: database/redshift } |  |  | 0.719 |
| walker |  | 1003 | 18 | Fs::DirListing { dir: database/spanner } |  |  | 0.719 |
| walker |  | 1021 | 18 | Fs::DirListing { dir: database/sqlcipher } |  |  | 0.719 |
| walker |  | 1039 | 18 | Fs::DirListing { dir: database/sqlite3 } |  |  | 0.719 |
| walker |  | 1057 | 18 | Fs::DirListing { dir: database/sqlserver } |  |  | 0.719 |
| walker |  | 1077 | 20 | Fs::DirListing { dir: database/ql } |  |  | 0.719 |
| walker |  | 1097 | 20 | Fs::DirListing { dir: database/rqlite } |  |  | 0.690 |
| ns | 1097 |  | 180 | Complete exported *Migrate method set (full signatures) | 2.2 |  | 0.690 |
| walker |  | 1119 | 22 | Fs::DirListing { dir: database/pgx } |  |  | 0.690 |
| walker |  | 1141 | 22 | Fs::DirListing { dir: database/postgres } |  |  | 0.690 |
| walker |  | 1165 | 24 | Fs::DirListing { dir: database/yugabytedb } |  |  | 0.690 |
| walker |  | 1191 | 26 | Fs::DirListing { dir: database/cockroachdb } |  |  | 0.690 |
| walker |  | 1217 | 26 | Fs::DirListing { dir: database/neo4j } |  |  | 0.690 |
| walker |  | 1232 | 15 | Fs::DirListing { dir: database/pgx/v5 } |  |  | 0.690 |
| walker |  | 1253 | 21 | Code::CodeKey { rung: ModuleDoc, file: database/multistmt/parse.go, decl: 0, sub: 0, line: 0 } |  |  | 0.690 |
| walker |  | 1262 | 9 | Fs::DirListing { dir: database/testing } |  |  | 0.690 |
| walker |  | 1272 | 10 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.690 |
| walker |  | 1330 | 58 | Code::CodeKey { rung: ModuleDoc, file: database/driver.go, decl: 0, sub: 0, line: 0 } |  |  | 0.690 |
| ns | 1349 |  | 252 | Package-level sentinel errors and tuning defaults | 2.3 |  | 0.643 |
| walker |  | 1388 | 58 | Code::CodeKey { rung: ModuleDoc, file: source/driver.go, decl: 0, sub: 0, line: 0 } |  |  | 0.644 |
| ns | 1462 |  | 113 | Logger interface (log.go, whole file) | 2.4 |  | 0.615 |
| walker |  | 1465 | 77 | Plaintext::Whole { file: Makefile } |  |  | 0.615 |
| walker |  | 1602 | 137 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.667 |
| walker |  | 1620 | 18 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.667 |
| ns | 1738 |  | 276 | The Migrate struct and its documented public knobs | 2.5 |  | 0.611 |
| walker |  | 1745 | 125 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.656 |
| ns | 1916 |  | 178 | Structured error types ErrShortLimit and ErrDirty | 2.6 |  | 0.622 |
| walker |  | 1932 | 187 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: true } |  |  | 0.622 |
| walker |  | 1979 | 47 | Markdown::Section { file: README.md, section_index: 10, keeps_default_concavity: false } |  |  | 0.622 |
| walker |  | 1983 | 4 | Fs::DirListing { dir: source/testing } |  |  | 0.622 |
| walker |  | 2039 | 56 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.622 |
| ns | 2132 |  | 216 | Complete unexported machinery roster of migrate.go (names only) | 2.7 |  | 0.598 |
| walker |  | 2245 | 206 | Fs::DirListing { dir: internal/cli } |  |  | 0.602 |
| walker |  | 2250 | 5 | Fs::DirListing { dir: source/go_bindata/testdata } |  |  | 0.602 |
| ns | 2336 |  | 204 | Up() in full — the canonical lock/dirty/read/run pipeline | 2.8 | 2.2 | 0.570 |
| walker |  | 2342 | 92 | Code::CodeKey { rung: ModuleDoc, file: source/godoc_vfs/vfs.go, decl: 0, sub: 0, line: 0 } |  |  | 0.570 |
| walker |  | 2603 | 261 | Plaintext::Whole { file: Dockerfile } |  |  | 0.570 |
| walker |  | 2713 | 110 | Code::CodeKey { rung: ModuleDoc, file: source/iofs/doc.go, decl: 0, sub: 0, line: 0 } |  |  | 0.570 |
| walker |  | 2720 | 7 | Fs::DirListing { dir: source/go_bindata/examples/migrations } |  |  | 0.570 |
| ns | 2737 |  | 401 | runMigrations(): the dirty-flag write protocol | 2.9 | 2.7 | 0.525 |
| walker |  | 2913 | 193 | Code::CodeKey { rung: Names, file: migrate.go, decl: 0, sub: 0, line: 0 } |  |  | 0.539 |
| walker |  | 2918 | 5 | Code::CodeKey { rung: Decl, file: migrate.go, decl: 3, sub: 0, line: 29 } |  |  | 0.540 |
| walker |  | 2928 | 10 | Code::CodeKey { rung: Decl, file: migrate.go, decl: 6, sub: 0, line: 48 } |  |  | 0.542 |
| walker |  | 2941 | 13 | Code::CodeKey { rung: Decl, file: migrate.go, decl: 4, sub: 0, line: 39 } |  |  | 0.545 |
| walker |  | 3123 | 182 | Code::CodeKey { rung: Decl, file: migrate.go, decl: 8, sub: 0, line: 56 } |  |  | 0.575 |
| walker |  | 3132 | 9 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 5, sub: 0, line: 44 } |  |  | 0.578 |
| ns | 3189 |  | 452 | lock(): mutex, ErrLocked and the LockTimeout race | 2.10 | 2.7 | 0.526 |
| walker |  | 3230 | 98 | Code::CodeKey { rung: Names, file: migration.go, decl: 0, sub: 0, line: 0 } |  |  | 0.526 |
| walker |  | 3247 | 17 | Code::CodeKey { rung: Decl, file: migration.go, decl: 3, sub: 0, line: 77 } |  |  | 0.526 |
| ns | 3449 |  | 260 | FilterCustomQuery and suint (util.go:42-63) | 2.11 |  | 0.507 |
| ns | 3579 |  | 130 | Package docs for the two driver packages | 3.1 |  | 0.512 |
| walker |  | 3635 | 388 | Code::CodeKey { rung: Decl, file: migration.go, decl: 2, sub: 0, line: 18 } |  |  | 0.512 |
| ns | 3648 |  | 69 | database.Driver method roster (names only) | 3.2 |  | 0.506 |
| walker |  | 3651 | 16 | Code::CodeKey { rung: Doc, file: migration.go, decl: 4, sub: 0, line: 107 } |  |  | 0.506 |
| walker |  | 3667 | 16 | Code::CodeKey { rung: Doc, file: migration.go, decl: 5, sub: 0, line: 112 } |  |  | 0.506 |
| ns | 3710 |  | 62 | source.Driver method roster (names only) | 3.3 |  | 0.500 |
| ns | 3798 |  | 88 | Driver registries: Open / Register / List in both packages | 3.4 |  | 0.496 |
| walker |  | 3831 | 164 | Code::CodeKey { rung: Names, file: migrate.go, decl: 0, sub: 1, line: 0 } |  |  | 0.506 |
| walker |  | 3844 | 13 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 13, sub: 0, line: 193 } |  |  | 0.506 |
| ns | 3892 |  | 94 | database package constants, sentinel errors and the registry map | 3.5 |  | 0.499 |
| walker |  | 3912 | 68 | Code::CodeKey { rung: Names, file: util.go, decl: 0, sub: 0, line: 0 } |  |  | 0.499 |
| walker |  | 3947 | 35 | Code::CodeKey { rung: Decl, file: util.go, decl: 2, sub: 0, line: 19 } |  |  | 0.499 |
| walker |  | 3996 | 49 | Code::CodeKey { rung: Decl, file: util.go, decl: 1, sub: 0, line: 11 } |  |  | 0.499 |
| walker |  | 4008 | 12 | Code::CodeKey { rung: Names, file: log.go, decl: 0, sub: 0, line: 0 } |  |  | 0.500 |
| walker |  | 4074 | 66 | Code::CodeKey { rung: Decl, file: log.go, decl: 1, sub: 0, line: 5 } |  |  | 0.510 |
| walker |  | 4090 | 16 | Code::CodeKey { rung: Doc, file: util.go, decl: 1, sub: 0, line: 11 } |  |  | 0.510 |
| walker |  | 4108 | 18 | Code::CodeKey { rung: Doc, file: util.go, decl: 4, sub: 0, line: 53 } |  |  | 0.511 |
| walker |  | 4127 | 19 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 2, sub: 0, line: 27 } |  |  | 0.514 |
| walker |  | 4146 | 19 | Code::CodeKey { rung: Doc, file: util.go, decl: 2, sub: 0, line: 19 } |  |  | 0.514 |
| walker |  | 4165 | 19 | Code::CodeKey { rung: Doc, file: util.go, decl: 3, sub: 0, line: 32 } |  |  | 0.514 |
| ns | 4219 |  | 327 | "How to implement a database driver" — the 7-step checklist | 3.6 |  | 0.500 |
| walker |  | 4302 | 137 | Code::CodeKey { rung: Names, file: migrate.go, decl: 0, sub: 2, line: 0 } |  |  | 0.518 |
| walker |  | 4314 | 12 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 18, sub: 0, line: 307 } |  |  | 0.518 |
| walker |  | 4339 | 25 | Code::CodeKey { rung: Doc, file: log.go, decl: 1, sub: 0, line: 5 } |  |  | 0.526 |
| walker |  | 4425 | 86 | Markdown::Section { file: README.md, section_index: 12, keeps_default_concavity: false } |  |  | 0.526 |
| walker |  | 4453 | 28 | Code::CodeKey { rung: Doc, file: migration.go, decl: 6, sub: 0, line: 122 } |  |  | 0.526 |
| walker |  | 4465 | 12 | Code::CodeKey { rung: Names, file: cli/main.go, decl: 0, sub: 0, line: 0 } |  |  | 0.526 |
| walker |  | 4478 | 13 | Code::CodeKey { rung: Decl, file: cli/main.go, decl: 1, sub: 0, line: 6 } |  |  | 0.526 |
| walker |  | 4491 | 13 | Code::CodeKey { rung: Doc, file: cli/main.go, decl: 1, sub: 0, line: 6 } |  |  | 0.526 |
| walker |  | 4503 | 12 | Code::CodeKey { rung: Names, file: cli/version.go, decl: 0, sub: 0, line: 0 } |  |  | 0.513 |
| ns | 4503 |  | 284 | "How to implement a source driver" — checklist and guidelines | 3.7 |  | 0.513 |
| walker |  | 4534 | 31 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 4, sub: 0, line: 39 } |  |  | 0.520 |
| walker |  | 4641 | 107 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.520 |
| walker |  | 4673 | 32 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 14, sub: 0, line: 212 } |  |  | 0.520 |
| walker |  | 4706 | 33 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 9, sub: 0, line: 85 } |  |  | 0.520 |
| walker |  | 4739 | 33 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 16, sub: 0, line: 265 } |  |  | 0.522 |
| walker |  | 4772 | 33 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 17, sub: 0, line: 287 } |  |  | 0.522 |
| walker |  | 4806 | 34 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 21, sub: 0, line: 383 } |  |  | 0.522 |
| walker |  | 4916 | 110 | Code::CodeKey { rung: Names, file: database/driver.go, decl: 0, sub: 0, line: 0 } |  |  | 0.529 |
| walker |  | 4919 | 3 | Code::CodeKey { rung: Decl, file: database/driver.go, decl: 1, sub: 0, line: 15 } |  |  | 0.530 |
| walker |  | 4930 | 11 | Code::CodeKey { rung: Doc, file: database/driver.go, decl: 5, sub: 0, line: 102 } |  |  | 0.530 |
| walker |  | 4941 | 11 | Code::CodeKey { rung: Doc, file: database/driver.go, decl: 6, sub: 0, line: 115 } |  |  | 0.530 |
| ns | 4943 |  | 440 | database.Driver in full: per-method semantic contracts | 3.8 | 3.2 | 0.505 |
| walker |  | 4953 | 12 | Code::CodeKey { rung: Doc, file: database/driver.go, decl: 4, sub: 0, line: 85 } |  |  | 0.505 |
| walker |  | 4980 | 27 | Code::CodeKey { rung: Names, file: database/error.go, decl: 0, sub: 0, line: 0 } |  |  | 0.505 |
| walker |  | 5084 | 104 | Code::CodeKey { rung: Decl, file: database/error.go, decl: 1, sub: 0, line: 8 } |  |  | 0.505 |
| walker |  | 5099 | 15 | Code::CodeKey { rung: Doc, file: cli/version.go, decl: 1, sub: 0, line: 4 } |  |  | 0.505 |
| walker |  | 5187 | 88 | Plaintext::DeclSurface { file: docker-deploy.sh } |  |  | 0.505 |
| walker |  | 5198 | 11 | Plaintext::Whole { file: docker-deploy.sh } |  |  | 0.505 |
| walker |  | 5235 | 37 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 15, sub: 0, line: 234 } |  |  | 0.505 |
| walker |  | 5272 | 37 | Code::CodeKey { rung: Doc, file: migration.go, decl: 1, sub: 0, line: 13 } |  |  | 0.506 |
| walker |  | 5312 | 40 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 20, sub: 0, line: 365 } |  |  | 0.506 |
| walker |  | 5352 | 40 | Code::CodeKey { rung: Doc, file: migration.go, decl: 2, sub: 0, line: 18 } |  |  | 0.506 |
| walker |  | 5370 | 18 | Code::CodeKey { rung: Doc, file: database/error.go, decl: 1, sub: 0, line: 8 } |  |  | 0.506 |
| ns | 5452 |  | 509 | source.Driver in full: per-method semantic contracts | 3.9 | 3.3 | 0.484 |
| walker |  | 5571 | 201 | Code::CodeKey { rung: Decl, file: database/driver.go, decl: 3, sub: 0, line: 45 } |  |  | 0.495 |
| ns | 5693 |  | 241 | database.Error: the query-level error type | 3.10 |  | 0.496 |
| ns | 5832 |  | 139 | Shared driver helpers: GenerateAdvisoryLockId and CasRestoreOnErr | 3.11 |  | 0.492 |
| walker |  | 5864 | 293 | Code::CodeKey { rung: Decl, file: database/driver.go, decl: 3, sub: 1, line: 45 } |  |  | 0.543 |
| walker |  | 5917 | 53 | Code::CodeKey { rung: Names, file: dktesting/dktesting.go, decl: 0, sub: 0, line: 0 } |  |  | 0.543 |
| walker |  | 5935 | 18 | Code::CodeKey { rung: Decl, file: dktesting/dktesting.go, decl: 3, sub: 0, line: 44 } |  |  | 0.543 |
| walker |  | 5960 | 25 | Code::CodeKey { rung: Decl, file: dktesting/dktesting.go, decl: 1, sub: 0, line: 14 } |  |  | 0.543 |
| walker |  | 5973 | 13 | Code::CodeKey { rung: Doc, file: dktesting/dktesting.go, decl: 1, sub: 0, line: 14 } |  |  | 0.543 |
| walker |  | 5986 | 13 | Code::CodeKey { rung: Doc, file: dktesting/dktesting.go, decl: 3, sub: 0, line: 44 } |  |  | 0.543 |
| ns | 6016 |  | 184 | internal/url: SchemeFromURL | 3.12 |  | 0.533 |
| walker |  | 6218 | 232 | Code::CodeKey { rung: Names, file: source/migration.go, decl: 0, sub: 0, line: 0 } |  |  | 0.533 |
| walker |  | 6223 | 5 | Code::CodeKey { rung: Decl, file: source/migration.go, decl: 2, sub: 0, line: 10 } |  |  | 0.533 |
| walker |  | 6228 | 5 | Code::CodeKey { rung: Decl, file: source/migration.go, decl: 4, sub: 0, line: 36 } |  |  | 0.533 |
| ns | 6248 |  | 232 | Exemplar driver: postgres registration, defaults and Config | 3.13 |  | 0.521 |
| walker |  | 6366 | 138 | Code::CodeKey { rung: Decl, file: source/migration.go, decl: 3, sub: 0, line: 18 } |  |  | 0.522 |
| walker |  | 6378 | 12 | Code::CodeKey { rung: Doc, file: source/migration.go, decl: 1, sub: 0, line: 8 } |  |  | 0.522 |
| walker |  | 6432 | 54 | Code::CodeKey { rung: Names, file: source/driver.go, decl: 0, sub: 0, line: 0 } |  |  | 0.528 |
| walker |  | 6443 | 11 | Code::CodeKey { rung: Doc, file: source/driver.go, decl: 3, sub: 0, line: 97 } |  |  | 0.528 |
| walker |  | 6454 | 11 | Code::CodeKey { rung: Doc, file: source/driver.go, decl: 4, sub: 0, line: 110 } |  |  | 0.528 |
| ns | 6457 |  | 209 | Migration filename grammar (MIGRATIONS.md:10-23) | 4.1 |  | 0.520 |
| walker |  | 6466 | 12 | Code::CodeKey { rung: Doc, file: source/driver.go, decl: 2, sub: 0, line: 76 } |  |  | 0.520 |
| walker |  | 6578 | 112 | Code::CodeKey { rung: Names, file: source/parse.go, decl: 0, sub: 0, line: 0 } |  |  | 0.521 |
| walker |  | 6581 | 3 | Code::CodeKey { rung: Decl, file: source/parse.go, decl: 1, sub: 0, line: 9 } |  |  | 0.521 |
| walker |  | 6586 | 5 | Code::CodeKey { rung: Decl, file: source/parse.go, decl: 2, sub: 0, line: 13 } |  |  | 0.521 |
| walker |  | 6597 | 11 | Code::CodeKey { rung: Doc, file: source/parse.go, decl: 4, sub: 0, line: 25 } |  |  | 0.521 |
| walker |  | 6628 | 31 | Code::CodeKey { rung: Names, file: source/errors.go, decl: 0, sub: 0, line: 0 } |  |  | 0.521 |
| ns | 6639 |  | 182 | source/parse.go: ErrParse, the Regex, and the Parse signature | 4.2 |  | 0.521 |
| walker |  | 6648 | 20 | Code::CodeKey { rung: Decl, file: source/errors.go, decl: 1, sub: 0, line: 7 } |  |  | 0.521 |
| walker |  | 6656 | 8 | Code::CodeKey { rung: Doc, file: source/errors.go, decl: 2, sub: 0, line: 13 } |  |  | 0.521 |
| walker |  | 6671 | 15 | Code::CodeKey { rung: Body, file: source/errors.go, decl: 2, sub: 0, line: 13 } |  |  | 0.521 |
| walker |  | 6692 | 21 | Code::CodeKey { rung: Doc, file: dktesting/dktesting.go, decl: 2, sub: 0, line: 20 } |  |  | 0.521 |
| walker |  | 6877 | 185 | Code::CodeKey { rung: Decl, file: source/driver.go, decl: 1, sub: 0, line: 35 } |  |  | 0.528 |
| ns | 6897 |  | 258 | source.Direction and source.Migration | 4.3 |  | 0.532 |
| walker |  | 7140 | 263 | Code::CodeKey { rung: Decl, file: source/driver.go, decl: 1, sub: 1, line: 35 } |  |  | 0.559 |
| ns | 7180 |  | 283 | source.Migrations: the in-memory index every directory-tree driver uses | 4.4 |  | 0.551 |
| walker |  | 7243 | 103 | Code::CodeKey { rung: Decl, file: source/driver.go, decl: 1, sub: 2, line: 35 } |  |  | 0.566 |
| walker |  | 7247 | 4 | Fs::DirListing { dir: cmd/migrate/examples } |  |  | 0.566 |
| walker |  | 7271 | 24 | Code::CodeKey { rung: Doc, file: source/errors.go, decl: 1, sub: 0, line: 7 } |  |  | 0.566 |
| walker |  | 7296 | 25 | Code::CodeKey { rung: Doc, file: source/migration.go, decl: 4, sub: 0, line: 36 } |  |  | 0.568 |
| walker |  | 7308 | 12 | Code::CodeKey { rung: Names, file: cmd/migrate/main.go, decl: 0, sub: 0, line: 0 } |  |  | 0.568 |
| walker |  | 7321 | 13 | Code::CodeKey { rung: Decl, file: cmd/migrate/main.go, decl: 1, sub: 0, line: 5 } |  |  | 0.568 |
| walker |  | 7333 | 12 | Code::CodeKey { rung: Names, file: cmd/migrate/version.go, decl: 0, sub: 0, line: 0 } |  |  | 0.569 |
| ns | 7384 |  | 204 | migrate.Migration: type location, buffer default, and complete method set | 4.5 |  | 0.574 |
| walker |  | 7495 | 162 | Markdown::Section { file: README.md, section_index: 11, keeps_default_concavity: false } |  |  | 0.574 |
| walker |  | 7557 | 62 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 19, sub: 0, line: 321 } |  |  | 0.574 |
| ns | 7583 |  | 199 | CLI entry points and the cli/ deprecation | 5.1 |  | 0.566 |
| ns | 7789 |  | 206 | Complete internal/cli listing — the build-tag matrix | 5.2 |  | 0.578 |
| walker |  | 7854 | 297 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.578 |
| walker |  | 7869 | 15 | Code::CodeKey { rung: Doc, file: cmd/migrate/version.go, decl: 1, sub: 0, line: 4 } |  |  | 0.579 |
| walker |  | 7883 | 14 | Code::CodeKey { rung: Names, file: internal/cli/main.go, decl: 0, sub: 0, line: 0 } |  |  | 0.579 |
| ns | 7955 |  | 166 | cli.Main and the complete global flag set | 5.3 |  | 0.575 |
| ns | 8287 |  | 332 | The `migrate -help` usage text, from source | 5.4 |  | 0.563 |
| ns | 8594 |  | 307 | Subcommand usage strings (internal/cli/main.go:18-33) | 5.5 |  | 0.557 |
| walker |  | 8706 | 823 | GoMod::File { file: go.mod } |  |  | 0.557 |
| ns | 8847 |  | 253 | Complete command-implementation roster (internal/cli/commands.go) | 5.6 |  | 0.552 |
| ns | 8984 |  | 137 | A build-tag shim in full, and the tag-name traps | 5.7 |  | 0.546 |
| walker |  | 9097 | 391 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.546 |
| walker |  | 9171 | 74 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 12, sub: 0, line: 171 } |  |  | 0.546 |
| walker |  | 9249 | 78 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 11, sub: 0, line: 145 } |  |  | 0.546 |
| ns | 9275 |  | 291 | Complete FAQ question roster (every #### heading) | 6.1 |  | 0.540 |
| walker |  | 9328 | 79 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 10, sub: 0, line: 119 } |  |  | 0.540 |
| ns | 9546 |  | 271 | Makefile: default driver tag sets and the test targets | 6.2 |  | 0.534 |
| walker |  | 9737 | 409 | Code::CodeKey { rung: Names, file: database/pgx/pgx.go, decl: 0, sub: 0, line: 0 } |  |  | 0.534 |
| walker |  | 9740 | 3 | Code::CodeKey { rung: Decl, file: database/pgx/pgx.go, decl: 2, sub: 0, line: 38 } |  |  | 0.534 |
| walker |  | 9743 | 3 | Code::CodeKey { rung: Decl, file: database/pgx/pgx.go, decl: 3, sub: 0, line: 47 } |  |  | 0.534 |
| walker |  | 9748 | 5 | Code::CodeKey { rung: Decl, file: database/pgx/pgx.go, decl: 1, sub: 0, line: 27 } |  |  | 0.534 |
| walker |  | 9753 | 5 | Code::CodeKey { rung: Decl, file: database/pgx/pgx.go, decl: 5, sub: 0, line: 68 } |  |  | 0.534 |
| walker |  | 9848 | 95 | Code::CodeKey { rung: Decl, file: database/pgx/pgx.go, decl: 4, sub: 0, line: 54 } |  |  | 0.534 |
| ns | 9863 |  | 317 | Shared conformance-test harnesses (complete function rosters) | 6.3 |  | 0.528 |
| walker |  | 9884 | 36 | Code::CodeKey { rung: Doc, file: source/parse.go, decl: 3, sub: 0, line: 22 } |  |  | 0.533 |
| ns | 9944 |  | 81 | Remaining doc heading rosters (MIGRATIONS.md, GETTING_STARTED.md) | 6.4 |  | 0.531 |
| walker |  | 9995 | 111 | Code::CodeKey { rung: Names, file: database/ql/ql.go, decl: 0, sub: 0, line: 0 } |  |  | 0.531 |

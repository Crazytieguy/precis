Score(3000)=0.539 I=0.695 C=0.419 ns_rows≤3K=17/48 grid(1000/1442/2080/3000/4327/6240/9000)=0.719/0.644/0.622/0.539/0.531/0.528/0.589

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
| walker |  | 568 | 5 | Fs::DirListing { dir: source/go_bindata/testdata } |  |  | 0.677 |
| ns | 570 |  | 122 | Complete database/ listing — every database driver package | 1.6 |  | 0.521 |
| walker |  | 596 | 28 | Fs::DirListing { dir: source/httpfs } |  |  | 0.521 |
| walker |  | 624 | 28 | Fs::DirListing { dir: source/iofs } |  |  | 0.521 |
| walker |  | 635 | 11 | Fs::DirListing { dir: source/httpfs/testdata } |  |  | 0.521 |
| walker |  | 641 | 6 | Fs::DirListing { dir: source/httpfs/testdata/no-migrations } |  |  | 0.521 |
| ns | 659 |  | 89 | Complete source/ listing — every source driver package | 1.7 |  | 0.569 |
| walker |  | 713 | 72 | Code::CodeKey { rung: ModuleDoc, file: migrate.go, decl: 0, sub: 0, line: 0 } |  |  | 0.607 |
| walker |  | 726 | 13 | Fs::DirListing { dir: testing } |  |  | 0.607 |
| ns | 796 |  | 137 | Complete README section-heading roster | 1.8 |  | 0.567 |
| walker |  | 848 | 122 | Fs::DirListing { dir: database } |  |  | 0.733 |
| walker |  | 854 | 6 | Fs::DirListing { dir: database/crate } |  |  | 0.733 |
| walker |  | 860 | 6 | Fs::DirListing { dir: database/shell } |  |  | 0.733 |
| walker |  | 869 | 9 | Fs::DirListing { dir: database/multistmt } |  |  | 0.733 |
| walker |  | 878 | 9 | Fs::DirListing { dir: database/snowflake } |  |  | 0.733 |
| walker |  | 887 | 9 | Fs::DirListing { dir: database/stub } |  |  | 0.733 |
| walker |  | 903 | 16 | Fs::DirListing { dir: database/mongodb } |  |  | 0.733 |
| ns | 917 |  | 121 | All four Migrate constructors (full signatures) | 2.1 |  | 0.719 |
| walker |  | 919 | 16 | Fs::DirListing { dir: database/mysql } |  |  | 0.719 |
| walker |  | 935 | 16 | Fs::DirListing { dir: database/sqlite } |  |  | 0.719 |
| walker |  | 953 | 18 | Fs::DirListing { dir: database/cassandra } |  |  | 0.719 |
| walker |  | 971 | 18 | Fs::DirListing { dir: database/clickhouse } |  |  | 0.719 |
| walker |  | 989 | 18 | Fs::DirListing { dir: database/firebird } |  |  | 0.719 |
| walker |  | 1007 | 18 | Fs::DirListing { dir: database/redshift } |  |  | 0.719 |
| walker |  | 1025 | 18 | Fs::DirListing { dir: database/spanner } |  |  | 0.719 |
| walker |  | 1043 | 18 | Fs::DirListing { dir: database/sqlcipher } |  |  | 0.719 |
| walker |  | 1061 | 18 | Fs::DirListing { dir: database/sqlite3 } |  |  | 0.719 |
| walker |  | 1079 | 18 | Fs::DirListing { dir: database/sqlserver } |  |  | 0.719 |
| ns | 1097 |  | 180 | Complete exported *Migrate method set (full signatures) | 2.2 |  | 0.690 |
| walker |  | 1099 | 20 | Fs::DirListing { dir: database/ql } |  |  | 0.690 |
| walker |  | 1119 | 20 | Fs::DirListing { dir: database/rqlite } |  |  | 0.690 |
| walker |  | 1141 | 22 | Fs::DirListing { dir: database/pgx } |  |  | 0.690 |
| walker |  | 1163 | 22 | Fs::DirListing { dir: database/postgres } |  |  | 0.690 |
| walker |  | 1187 | 24 | Fs::DirListing { dir: database/yugabytedb } |  |  | 0.690 |
| walker |  | 1213 | 26 | Fs::DirListing { dir: database/cockroachdb } |  |  | 0.690 |
| walker |  | 1239 | 26 | Fs::DirListing { dir: database/neo4j } |  |  | 0.690 |
| walker |  | 1254 | 15 | Fs::DirListing { dir: database/pgx/v5 } |  |  | 0.690 |
| walker |  | 1275 | 21 | Code::CodeKey { rung: ModuleDoc, file: database/multistmt/parse.go, decl: 0, sub: 0, line: 0 } |  |  | 0.690 |
| walker |  | 1293 | 18 | Fs::DirListing { dir: source/httpfs/testdata/duplicates } |  |  | 0.690 |
| walker |  | 1302 | 9 | Fs::DirListing { dir: database/testing } |  |  | 0.690 |
| walker |  | 1312 | 10 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.690 |
| ns | 1349 |  | 252 | Package-level sentinel errors and tuning defaults | 2.3 |  | 0.642 |
| walker |  | 1370 | 58 | Code::CodeKey { rung: ModuleDoc, file: database/driver.go, decl: 0, sub: 0, line: 0 } |  |  | 0.643 |
| walker |  | 1428 | 58 | Code::CodeKey { rung: ModuleDoc, file: source/driver.go, decl: 0, sub: 0, line: 0 } |  |  | 0.644 |
| ns | 1462 |  | 113 | Logger interface (log.go, whole file) | 2.4 |  | 0.615 |
| walker |  | 1505 | 77 | Plaintext::Whole { file: Makefile } |  |  | 0.615 |
| walker |  | 1642 | 137 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.667 |
| walker |  | 1660 | 18 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.667 |
| ns | 1738 |  | 276 | The Migrate struct and its documented public knobs | 2.5 |  | 0.611 |
| walker |  | 1785 | 125 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.656 |
| ns | 1916 |  | 178 | Structured error types ErrShortLimit and ErrDirty | 2.6 |  | 0.622 |
| walker |  | 1972 | 187 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: true } |  |  | 0.622 |
| walker |  | 2019 | 47 | Markdown::Section { file: README.md, section_index: 10, keeps_default_concavity: false } |  |  | 0.622 |
| walker |  | 2023 | 4 | Fs::DirListing { dir: source/testing } |  |  | 0.622 |
| walker |  | 2079 | 56 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.622 |
| walker |  | 2091 | 12 | Code::CodeKey { rung: Names, file: log.go, decl: 0, sub: 0, line: 0 } |  |  | 0.622 |
| ns | 2132 |  | 216 | Complete unexported machinery roster of migrate.go (names only) | 2.7 |  | 0.599 |
| walker |  | 2157 | 66 | Code::CodeKey { rung: Decl, file: log.go, decl: 1, sub: 0, line: 5 } |  |  | 0.615 |
| ns | 2336 |  | 204 | Up() in full — the canonical lock/dirty/read/run pipeline | 2.8 | 2.2 | 0.583 |
| walker |  | 2363 | 206 | Fs::DirListing { dir: internal/cli } |  |  | 0.586 |
| walker |  | 2429 | 66 | Fs::DirListing { dir: source/iofs/testdata/migrations } |  |  | 0.586 |
| walker |  | 2521 | 92 | Code::CodeKey { rung: ModuleDoc, file: source/godoc_vfs/vfs.go, decl: 0, sub: 0, line: 0 } |  |  | 0.586 |
| walker |  | 2619 | 98 | Code::CodeKey { rung: Names, file: migration.go, decl: 0, sub: 0, line: 0 } |  |  | 0.586 |
| walker |  | 2636 | 17 | Code::CodeKey { rung: Decl, file: migration.go, decl: 3, sub: 0, line: 77 } |  |  | 0.586 |
| walker |  | 2652 | 16 | Code::CodeKey { rung: Doc, file: migration.go, decl: 4, sub: 0, line: 107 } |  |  | 0.586 |
| walker |  | 2668 | 16 | Code::CodeKey { rung: Doc, file: migration.go, decl: 5, sub: 0, line: 112 } |  |  | 0.586 |
| ns | 2737 |  | 401 | runMigrations(): the dirty-flag write protocol | 2.9 | 2.7 | 0.539 |
| walker |  | 3107 | 439 | Code::CodeKey { rung: Decl, file: migration.go, decl: 2, sub: 0, line: 18 } |  |  | 0.539 |
| walker |  | 3189 | 82 | Code::CodeKey { rung: Names, file: util.go, decl: 0, sub: 0, line: 0 } |  |  | 0.491 |
| ns | 3189 |  | 452 | lock(): mutex, ErrLocked and the LockTimeout race | 2.10 | 2.7 | 0.491 |
| walker |  | 3203 | 14 | Code::CodeKey { rung: Decl, file: util.go, decl: 1, sub: 0, line: 13 } |  |  | 0.491 |
| walker |  | 3221 | 18 | Code::CodeKey { rung: Doc, file: util.go, decl: 5, sub: 0, line: 53 } |  |  | 0.491 |
| walker |  | 3240 | 19 | Code::CodeKey { rung: Doc, file: util.go, decl: 3, sub: 0, line: 32 } |  |  | 0.491 |
| ns | 3449 |  | 260 | FilterCustomQuery and suint (util.go:42-63) | 2.11 |  | 0.475 |
| walker |  | 3501 | 261 | Plaintext::Whole { file: Dockerfile } |  |  | 0.475 |
| ns | 3579 |  | 130 | Package docs for the two driver packages | 3.1 |  | 0.482 |
| walker |  | 3583 | 82 | Fs::DirListing { dir: source/httpfs/testdata/sql } |  |  | 0.482 |
| walker |  | 3589 | 6 | Fs::DirListing { dir: source/httpfs/testdata/sql/subdirs-are-ignored } |  |  | 0.482 |
| ns | 3648 |  | 69 | database.Driver method roster (names only) | 3.2 |  | 0.475 |
| walker |  | 3699 | 110 | Code::CodeKey { rung: ModuleDoc, file: source/iofs/doc.go, decl: 0, sub: 0, line: 0 } |  |  | 0.475 |
| walker |  | 3706 | 7 | Fs::DirListing { dir: source/go_bindata/examples/migrations } |  |  | 0.475 |
| ns | 3710 |  | 62 | source.Driver method roster (names only) | 3.3 |  | 0.470 |
| walker |  | 3731 | 25 | Code::CodeKey { rung: Doc, file: log.go, decl: 1, sub: 0, line: 5 } |  |  | 0.480 |
| ns | 3798 |  | 88 | Driver registries: Open / Register / List in both packages | 3.4 |  | 0.475 |
| ns | 3892 |  | 94 | database package constants, sentinel errors and the registry map | 3.5 |  | 0.469 |
| walker |  | 3937 | 206 | Code::CodeKey { rung: Names, file: migrate.go, decl: 0, sub: 0, line: 0 } |  |  | 0.478 |
| walker |  | 3946 | 9 | Code::CodeKey { rung: Decl, file: migrate.go, decl: 3, sub: 0, line: 29 } |  |  | 0.481 |
| walker |  | 3956 | 10 | Code::CodeKey { rung: Decl, file: migrate.go, decl: 6, sub: 0, line: 48 } |  |  | 0.482 |
| walker |  | 3969 | 13 | Code::CodeKey { rung: Decl, file: migrate.go, decl: 4, sub: 0, line: 39 } |  |  | 0.484 |
| walker |  | 3978 | 9 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 5, sub: 0, line: 44 } |  |  | 0.486 |
| ns | 4219 |  | 327 | "How to implement a database driver" — the 7-step checklist | 3.6 |  | 0.472 |
| walker |  | 4243 | 265 | Code::CodeKey { rung: Decl, file: migrate.go, decl: 8, sub: 0, line: 56 } |  |  | 0.524 |
| walker |  | 4262 | 19 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 2, sub: 0, line: 27 } |  |  | 0.527 |
| walker |  | 4418 | 156 | Code::CodeKey { rung: Names, file: migrate.go, decl: 0, sub: 1, line: 0 } |  |  | 0.536 |
| walker |  | 4431 | 13 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 14, sub: 0, line: 193 } |  |  | 0.536 |
| ns | 4503 |  | 284 | "How to implement a source driver" — checklist and guidelines | 3.7 |  | 0.523 |
| walker |  | 4621 | 190 | Code::CodeKey { rung: Names, file: migrate.go, decl: 0, sub: 2, line: 0 } |  |  | 0.541 |
| walker |  | 4633 | 12 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 19, sub: 0, line: 307 } |  |  | 0.541 |
| walker |  | 4872 | 239 | Code::CodeKey { rung: Names, file: migrate.go, decl: 0, sub: 3, line: 0 } |  |  | 0.565 |
| walker |  | 4887 | 15 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 33, sub: 0, line: 961 } |  |  | 0.565 |
| walker |  | 4903 | 16 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 35, sub: 0, line: 975 } |  |  | 0.565 |
| walker |  | 4925 | 22 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 34, sub: 0, line: 968 } |  |  | 0.565 |
| ns | 4943 |  | 440 | database.Driver in full: per-method semantic contracts | 3.8 | 3.2 | 0.538 |
| walker |  | 4953 | 28 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 32, sub: 0, line: 953 } |  |  | 0.538 |
| walker |  | 4981 | 28 | Code::CodeKey { rung: Doc, file: migration.go, decl: 6, sub: 0, line: 122 } |  |  | 0.538 |
| walker |  | 5011 | 30 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 27, sub: 0, line: 776 } |  |  | 0.538 |
| walker |  | 5041 | 30 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 29, sub: 0, line: 832 } |  |  | 0.538 |
| walker |  | 5072 | 31 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 4, sub: 0, line: 39 } |  |  | 0.544 |
| walker |  | 5104 | 32 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 15, sub: 0, line: 212 } |  |  | 0.544 |
| walker |  | 5137 | 33 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 9, sub: 0, line: 85 } |  |  | 0.544 |
| walker |  | 5170 | 33 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 17, sub: 0, line: 265 } |  |  | 0.546 |
| walker |  | 5203 | 33 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 18, sub: 0, line: 287 } |  |  | 0.546 |
| walker |  | 5237 | 34 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 22, sub: 0, line: 383 } |  |  | 0.546 |
| walker |  | 5271 | 34 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 30, sub: 0, line: 887 } |  |  | 0.546 |
| walker |  | 5283 | 12 | Code::CodeKey { rung: Names, file: cli/main.go, decl: 0, sub: 0, line: 0 } |  |  | 0.546 |
| walker |  | 5296 | 13 | Code::CodeKey { rung: Decl, file: cli/main.go, decl: 1, sub: 0, line: 6 } |  |  | 0.546 |
| walker |  | 5309 | 13 | Code::CodeKey { rung: Doc, file: cli/main.go, decl: 1, sub: 0, line: 6 } |  |  | 0.546 |
| walker |  | 5321 | 12 | Code::CodeKey { rung: Names, file: cli/version.go, decl: 0, sub: 0, line: 0 } |  |  | 0.546 |
| walker |  | 5336 | 15 | Code::CodeKey { rung: Doc, file: cli/version.go, decl: 1, sub: 0, line: 4 } |  |  | 0.546 |
| walker |  | 5443 | 107 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.546 |
| ns | 5452 |  | 509 | source.Driver in full: per-method semantic contracts | 3.9 | 3.3 | 0.522 |
| walker |  | 5480 | 37 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 16, sub: 0, line: 234 } |  |  | 0.522 |
| walker |  | 5517 | 37 | Code::CodeKey { rung: Doc, file: migration.go, decl: 1, sub: 0, line: 13 } |  |  | 0.522 |
| walker |  | 5616 | 99 | Markdown::Section { file: README.md, section_index: 12, keeps_default_concavity: false } |  |  | 0.522 |
| walker |  | 5656 | 40 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 21, sub: 0, line: 365 } |  |  | 0.522 |
| ns | 5693 |  | 241 | database.Error: the query-level error type | 3.10 |  | 0.510 |
| walker |  | 5696 | 40 | Code::CodeKey { rung: Doc, file: migration.go, decl: 2, sub: 0, line: 18 } |  |  | 0.510 |
| walker |  | 5723 | 27 | Code::CodeKey { rung: Names, file: database/error.go, decl: 0, sub: 0, line: 0 } |  |  | 0.511 |
| walker |  | 5827 | 104 | Code::CodeKey { rung: Decl, file: database/error.go, decl: 1, sub: 0, line: 8 } |  |  | 0.520 |
| ns | 5832 |  | 139 | Shared driver helpers: GenerateAdvisoryLockId and CasRestoreOnErr | 3.11 |  | 0.515 |
| walker |  | 5908 | 81 | Code::CodeKey { rung: Names, file: source/driver.go, decl: 0, sub: 0, line: 0 } |  |  | 0.517 |
| walker |  | 5919 | 11 | Code::CodeKey { rung: Doc, file: source/driver.go, decl: 5, sub: 0, line: 97 } |  |  | 0.517 |
| walker |  | 5930 | 11 | Code::CodeKey { rung: Doc, file: source/driver.go, decl: 6, sub: 0, line: 110 } |  |  | 0.517 |
| walker |  | 5942 | 12 | Code::CodeKey { rung: Doc, file: source/driver.go, decl: 4, sub: 0, line: 76 } |  |  | 0.517 |
| walker |  | 5960 | 18 | Code::CodeKey { rung: Doc, file: database/error.go, decl: 1, sub: 0, line: 8 } |  |  | 0.521 |
| walker |  | 6003 | 43 | Code::CodeKey { rung: Doc, file: util.go, decl: 4, sub: 0, line: 45 } |  |  | 0.524 |
| ns | 6016 |  | 184 | internal/url: SchemeFromURL | 3.12 |  | 0.514 |
| walker |  | 6091 | 88 | Plaintext::DeclSurface { file: docker-deploy.sh } |  |  | 0.514 |
| walker |  | 6102 | 11 | Plaintext::Whole { file: docker-deploy.sh } |  |  | 0.514 |
| walker |  | 6236 | 134 | Code::CodeKey { rung: Names, file: database/driver.go, decl: 0, sub: 0, line: 0 } |  |  | 0.528 |
| walker |  | 6245 | 9 | Code::CodeKey { rung: Decl, file: database/driver.go, decl: 1, sub: 0, line: 15 } |  |  | 0.531 |
| ns | 6248 |  | 232 | Exemplar driver: postgres registration, defaults and Config | 3.13 |  | 0.518 |
| walker |  | 6256 | 11 | Code::CodeKey { rung: Doc, file: database/driver.go, decl: 7, sub: 0, line: 102 } |  |  | 0.518 |
| walker |  | 6267 | 11 | Code::CodeKey { rung: Doc, file: database/driver.go, decl: 8, sub: 0, line: 115 } |  |  | 0.518 |
| walker |  | 6279 | 12 | Code::CodeKey { rung: Doc, file: database/driver.go, decl: 6, sub: 0, line: 85 } |  |  | 0.518 |
| walker |  | 6325 | 46 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 31, sub: 0, line: 938 } |  |  | 0.518 |
| walker |  | 6356 | 31 | Code::CodeKey { rung: Names, file: source/errors.go, decl: 0, sub: 0, line: 0 } |  |  | 0.518 |
| walker |  | 6376 | 20 | Code::CodeKey { rung: Decl, file: source/errors.go, decl: 1, sub: 0, line: 7 } |  |  | 0.518 |
| walker |  | 6384 | 8 | Code::CodeKey { rung: Doc, file: source/errors.go, decl: 2, sub: 0, line: 13 } |  |  | 0.518 |
| walker |  | 6399 | 15 | Code::CodeKey { rung: Body, file: source/errors.go, decl: 2, sub: 0, line: 13 } |  |  | 0.518 |
| walker |  | 6450 | 51 | Code::CodeKey { rung: Doc, file: util.go, decl: 1, sub: 0, line: 13 } |  |  | 0.518 |
| ns | 6457 |  | 209 | Migration filename grammar (MIGRATIONS.md:10-23) | 4.1 |  | 0.511 |
| ns | 6639 |  | 182 | source/parse.go: ErrParse, the Regex, and the Parse signature | 4.2 |  | 0.503 |
| walker |  | 6651 | 201 | Code::CodeKey { rung: Decl, file: database/driver.go, decl: 5, sub: 0, line: 45 } |  |  | 0.512 |
| ns | 6897 |  | 258 | source.Direction and source.Migration | 4.3 |  | 0.500 |
| walker |  | 6944 | 293 | Code::CodeKey { rung: Decl, file: database/driver.go, decl: 5, sub: 1, line: 45 } |  |  | 0.544 |
| walker |  | 6996 | 52 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 28, sub: 0, line: 815 } |  |  | 0.544 |
| walker |  | 7049 | 53 | Code::CodeKey { rung: Names, file: dktesting/dktesting.go, decl: 0, sub: 0, line: 0 } |  |  | 0.544 |
| walker |  | 7067 | 18 | Code::CodeKey { rung: Decl, file: dktesting/dktesting.go, decl: 3, sub: 0, line: 44 } |  |  | 0.544 |
| walker |  | 7092 | 25 | Code::CodeKey { rung: Decl, file: dktesting/dktesting.go, decl: 1, sub: 0, line: 14 } |  |  | 0.544 |
| walker |  | 7105 | 13 | Code::CodeKey { rung: Doc, file: dktesting/dktesting.go, decl: 1, sub: 0, line: 14 } |  |  | 0.544 |
| walker |  | 7118 | 13 | Code::CodeKey { rung: Doc, file: dktesting/dktesting.go, decl: 3, sub: 0, line: 44 } |  |  | 0.544 |
| walker |  | 7139 | 21 | Code::CodeKey { rung: Doc, file: dktesting/dktesting.go, decl: 2, sub: 0, line: 20 } |  |  | 0.544 |
| ns | 7180 |  | 283 | source.Migrations: the in-memory index every directory-tree driver uses | 4.4 |  | 0.533 |
| walker |  | 7193 | 54 | Code::CodeKey { rung: Doc, file: util.go, decl: 2, sub: 0, line: 21 } |  |  | 0.533 |
| ns | 7384 |  | 204 | migrate.Migration: type location, buffer default, and complete method set | 4.5 |  | 0.540 |
| walker |  | 7486 | 293 | Code::CodeKey { rung: Names, file: source/migration.go, decl: 0, sub: 0, line: 0 } |  |  | 0.547 |
| walker |  | 7495 | 9 | Code::CodeKey { rung: Decl, file: source/migration.go, decl: 2, sub: 0, line: 10 } |  |  | 0.547 |
| walker |  | 7523 | 28 | Code::CodeKey { rung: Decl, file: source/migration.go, decl: 4, sub: 0, line: 36 } |  |  | 0.551 |
| ns | 7583 |  | 199 | CLI entry points and the cli/ deprecation | 5.1 |  | 0.541 |
| walker |  | 7661 | 138 | Code::CodeKey { rung: Decl, file: source/migration.go, decl: 3, sub: 0, line: 18 } |  |  | 0.554 |
| walker |  | 7673 | 12 | Code::CodeKey { rung: Doc, file: source/migration.go, decl: 1, sub: 0, line: 8 } |  |  | 0.556 |
| walker |  | 7697 | 24 | Code::CodeKey { rung: Doc, file: source/errors.go, decl: 1, sub: 0, line: 7 } |  |  | 0.556 |
| walker |  | 7722 | 25 | Code::CodeKey { rung: Doc, file: source/migration.go, decl: 4, sub: 0, line: 36 } |  |  | 0.560 |
| ns | 7789 |  | 206 | Complete internal/cli listing — the build-tag matrix | 5.2 |  | 0.573 |
| walker |  | 7822 | 100 | Code::CodeKey { rung: Names, file: source/parse.go, decl: 0, sub: 0, line: 0 } |  |  | 0.576 |
| walker |  | 7833 | 11 | Code::CodeKey { rung: Decl, file: source/parse.go, decl: 1, sub: 0, line: 9 } |  |  | 0.577 |
| walker |  | 7842 | 9 | Code::CodeKey { rung: Decl, file: source/parse.go, decl: 2, sub: 0, line: 13 } |  |  | 0.578 |
| walker |  | 7853 | 11 | Code::CodeKey { rung: Doc, file: source/parse.go, decl: 4, sub: 0, line: 25 } |  |  | 0.580 |
| ns | 7955 |  | 166 | cli.Main and the complete global flag set | 5.3 |  | 0.576 |
| walker |  | 8038 | 185 | Code::CodeKey { rung: Decl, file: source/driver.go, decl: 3, sub: 0, line: 35 } |  |  | 0.582 |
| ns | 8287 |  | 332 | The `migrate -help` usage text, from source | 5.4 |  | 0.570 |
| walker |  | 8301 | 263 | Code::CodeKey { rung: Decl, file: source/driver.go, decl: 3, sub: 1, line: 35 } |  |  | 0.591 |
| walker |  | 8404 | 103 | Code::CodeKey { rung: Decl, file: source/driver.go, decl: 3, sub: 2, line: 35 } |  |  | 0.604 |
| walker |  | 8466 | 62 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 20, sub: 0, line: 321 } |  |  | 0.604 |
| walker |  | 8470 | 4 | Fs::DirListing { dir: cmd/migrate/examples } |  |  | 0.604 |
| walker |  | 8482 | 12 | Code::CodeKey { rung: Names, file: cmd/migrate/main.go, decl: 0, sub: 0, line: 0 } |  |  | 0.604 |
| walker |  | 8495 | 13 | Code::CodeKey { rung: Decl, file: cmd/migrate/main.go, decl: 1, sub: 0, line: 5 } |  |  | 0.605 |
| walker |  | 8507 | 12 | Code::CodeKey { rung: Names, file: cmd/migrate/version.go, decl: 0, sub: 0, line: 0 } |  |  | 0.606 |
| walker |  | 8522 | 15 | Code::CodeKey { rung: Doc, file: cmd/migrate/version.go, decl: 1, sub: 0, line: 4 } |  |  | 0.607 |
| walker |  | 8534 | 12 | Code::CodeKey { rung: Names, file: internal/cli/build_github.go, decl: 0, sub: 0, line: 0 } |  |  | 0.607 |
| walker |  | 8546 | 12 | Code::CodeKey { rung: Names, file: internal/cli/build_github_ee.go, decl: 0, sub: 0, line: 0 } |  |  | 0.607 |
| walker |  | 8558 | 12 | Code::CodeKey { rung: Names, file: internal/cli/build_mongodb.go, decl: 0, sub: 0, line: 0 } |  |  | 0.607 |
| walker |  | 8570 | 12 | Code::CodeKey { rung: Names, file: internal/cli/build_mysql.go, decl: 0, sub: 0, line: 0 } |  |  | 0.607 |
| walker |  | 8582 | 12 | Code::CodeKey { rung: Names, file: internal/cli/build_postgres.go, decl: 0, sub: 0, line: 0 } |  |  | 0.607 |
| walker |  | 8594 | 12 | Code::CodeKey { rung: Names, file: internal/cli/build_sqlite.go, decl: 0, sub: 0, line: 0 } |  |  | 0.600 |
| ns | 8594 |  | 307 | Subcommand usage strings (internal/cli/main.go:18-33) | 5.5 |  | 0.600 |
| walker |  | 8756 | 162 | Markdown::Section { file: README.md, section_index: 11, keeps_default_concavity: false } |  |  | 0.600 |
| walker |  | 8830 | 74 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 12, sub: 0, line: 171 } |  |  | 0.600 |
| walker |  | 8843 | 13 | Code::CodeKey { rung: Names, file: internal/cli/build_bitbucket.go, decl: 0, sub: 0, line: 0 } |  |  | 0.600 |
| ns | 8847 |  | 253 | Complete command-implementation roster (internal/cli/commands.go) | 5.6 |  | 0.595 |
| walker |  | 8856 | 13 | Code::CodeKey { rung: Names, file: internal/cli/build_cassandra.go, decl: 0, sub: 0, line: 0 } |  |  | 0.595 |
| walker |  | 8869 | 13 | Code::CodeKey { rung: Names, file: internal/cli/build_clickhouse.go, decl: 0, sub: 0, line: 0 } |  |  | 0.595 |
| walker |  | 8882 | 13 | Code::CodeKey { rung: Names, file: internal/cli/build_firebird.go, decl: 0, sub: 0, line: 0 } |  |  | 0.595 |
| walker |  | 8895 | 13 | Code::CodeKey { rung: Names, file: internal/cli/build_gitlab.go, decl: 0, sub: 0, line: 0 } |  |  | 0.595 |
| walker |  | 8908 | 13 | Code::CodeKey { rung: Names, file: internal/cli/build_pgx.go, decl: 0, sub: 0, line: 0 } |  |  | 0.595 |
| walker |  | 8921 | 13 | Code::CodeKey { rung: Names, file: internal/cli/build_ql.go, decl: 0, sub: 0, line: 0 } |  |  | 0.595 |
| walker |  | 8934 | 13 | Code::CodeKey { rung: Names, file: internal/cli/build_redshift.go, decl: 0, sub: 0, line: 0 } |  |  | 0.595 |
| walker |  | 8947 | 13 | Code::CodeKey { rung: Names, file: internal/cli/build_snowflake.go, decl: 0, sub: 0, line: 0 } |  |  | 0.595 |
| walker |  | 8960 | 13 | Code::CodeKey { rung: Names, file: internal/cli/build_spanner.go, decl: 0, sub: 0, line: 0 } |  |  | 0.595 |
| walker |  | 8973 | 13 | Code::CodeKey { rung: Names, file: internal/cli/build_sqlcipher.go, decl: 0, sub: 0, line: 0 } |  |  | 0.595 |
| ns | 8984 |  | 137 | A build-tag shim in full, and the tag-name traps | 5.7 |  | 0.589 |
| walker |  | 8986 | 13 | Code::CodeKey { rung: Names, file: internal/cli/build_sqlite3.go, decl: 0, sub: 0, line: 0 } |  |  | 0.589 |
| walker |  | 8999 | 13 | Code::CodeKey { rung: Names, file: internal/cli/build_sqlserver.go, decl: 0, sub: 0, line: 0 } |  |  | 0.589 |
| walker |  | 9075 | 76 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 23, sub: 0, line: 400 } |  |  | 0.589 |
| walker |  | 9153 | 78 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 11, sub: 0, line: 145 } |  |  | 0.589 |
| walker |  | 9232 | 79 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 10, sub: 0, line: 119 } |  |  | 0.589 |
| ns | 9275 |  | 291 | Complete FAQ question roster (every #### heading) | 6.1 |  | 0.582 |
| walker |  | 9529 | 297 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.582 |
| walker |  | 9543 | 14 | Code::CodeKey { rung: Names, file: internal/cli/build_aws-s3.go, decl: 0, sub: 0, line: 0 } |  |  | 0.582 |
| ns | 9546 |  | 271 | Makefile: default driver tag sets and the test targets | 6.2 |  | 0.575 |
| walker |  | 9557 | 14 | Code::CodeKey { rung: Names, file: internal/cli/build_cockroachdb.go, decl: 0, sub: 0, line: 0 } |  |  | 0.575 |
| walker |  | 9571 | 14 | Code::CodeKey { rung: Names, file: internal/cli/build_go-bindata.go, decl: 0, sub: 0, line: 0 } |  |  | 0.575 |
| walker |  | 9585 | 14 | Code::CodeKey { rung: Names, file: internal/cli/build_google-cloud-storage.go, decl: 0, sub: 0, line: 0 } |  |  | 0.575 |
| walker |  | 9599 | 14 | Code::CodeKey { rung: Names, file: internal/cli/build_neo4j.go, decl: 0, sub: 0, line: 0 } |  |  | 0.575 |
| walker |  | 9613 | 14 | Code::CodeKey { rung: Names, file: internal/cli/build_pgxv5.go, decl: 0, sub: 0, line: 0 } |  |  | 0.576 |
| walker |  | 9627 | 14 | Code::CodeKey { rung: Names, file: internal/cli/build_rqlite.go, decl: 0, sub: 0, line: 0 } |  |  | 0.576 |
| ns | 9863 |  | 317 | Shared conformance-test harnesses (complete function rosters) | 6.3 |  | 0.570 |
| ns | 9944 |  | 81 | Remaining doc heading rosters (MIGRATIONS.md, GETTING_STARTED.md) | 6.4 |  | 0.567 |
| walker |  | 9987 | 360 | GoMod::File { file: go.mod } |  |  | 0.567 |

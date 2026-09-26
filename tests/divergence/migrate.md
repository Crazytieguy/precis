Score(3000)=0.525 I=0.688 C=0.401 ns_rows≤3K=17/48 grid(1000/1442/2080/3000/4327/6240/9000)=0.719/0.644/0.622/0.525/0.522/0.525/0.589

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
| ns | 2132 |  | 216 | Complete unexported machinery roster of migrate.go (names only) | 2.7 |  | 0.598 |
| walker |  | 2285 | 206 | Fs::DirListing { dir: internal/cli } |  |  | 0.602 |
| ns | 2336 |  | 204 | Up() in full — the canonical lock/dirty/read/run pipeline | 2.8 | 2.2 | 0.570 |
| walker |  | 2351 | 66 | Fs::DirListing { dir: source/iofs/testdata/migrations } |  |  | 0.570 |
| walker |  | 2443 | 92 | Code::CodeKey { rung: ModuleDoc, file: source/godoc_vfs/vfs.go, decl: 0, sub: 0, line: 0 } |  |  | 0.570 |
| walker |  | 2704 | 261 | Plaintext::Whole { file: Dockerfile } |  |  | 0.570 |
| ns | 2737 |  | 401 | runMigrations(): the dirty-flag write protocol | 2.9 | 2.7 | 0.525 |
| walker |  | 2786 | 82 | Fs::DirListing { dir: source/httpfs/testdata/sql } |  |  | 0.525 |
| walker |  | 2792 | 6 | Fs::DirListing { dir: source/httpfs/testdata/sql/subdirs-are-ignored } |  |  | 0.525 |
| walker |  | 2902 | 110 | Code::CodeKey { rung: ModuleDoc, file: source/iofs/doc.go, decl: 0, sub: 0, line: 0 } |  |  | 0.525 |
| walker |  | 2909 | 7 | Fs::DirListing { dir: source/go_bindata/examples/migrations } |  |  | 0.525 |
| walker |  | 3115 | 206 | Code::CodeKey { rung: Names, file: migrate.go, decl: 0, sub: 0, line: 0 } |  |  | 0.537 |
| walker |  | 3124 | 9 | Code::CodeKey { rung: Decl, file: migrate.go, decl: 3, sub: 0, line: 29 } |  |  | 0.539 |
| walker |  | 3134 | 10 | Code::CodeKey { rung: Decl, file: migrate.go, decl: 6, sub: 0, line: 48 } |  |  | 0.541 |
| walker |  | 3147 | 13 | Code::CodeKey { rung: Decl, file: migrate.go, decl: 4, sub: 0, line: 39 } |  |  | 0.544 |
| walker |  | 3156 | 9 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 5, sub: 0, line: 44 } |  |  | 0.547 |
| ns | 3189 |  | 452 | lock(): mutex, ErrLocked and the LockTimeout race | 2.10 | 2.7 | 0.497 |
| walker |  | 3421 | 265 | Code::CodeKey { rung: Decl, file: migrate.go, decl: 8, sub: 0, line: 56 } |  |  | 0.558 |
| walker |  | 3440 | 19 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 2, sub: 0, line: 27 } |  |  | 0.562 |
| ns | 3449 |  | 260 | FilterCustomQuery and suint (util.go:42-63) | 2.11 |  | 0.541 |
| walker |  | 3538 | 98 | Code::CodeKey { rung: Names, file: migration.go, decl: 0, sub: 0, line: 0 } |  |  | 0.541 |
| walker |  | 3555 | 17 | Code::CodeKey { rung: Decl, file: migration.go, decl: 3, sub: 0, line: 77 } |  |  | 0.541 |
| walker |  | 3571 | 16 | Code::CodeKey { rung: Doc, file: migration.go, decl: 4, sub: 0, line: 107 } |  |  | 0.541 |
| ns | 3579 |  | 130 | Package docs for the two driver packages | 3.1 |  | 0.546 |
| walker |  | 3587 | 16 | Code::CodeKey { rung: Doc, file: migration.go, decl: 5, sub: 0, line: 112 } |  |  | 0.546 |
| ns | 3648 |  | 69 | database.Driver method roster (names only) | 3.2 |  | 0.539 |
| ns | 3710 |  | 62 | source.Driver method roster (names only) | 3.3 |  | 0.533 |
| ns | 3798 |  | 88 | Driver registries: Open / Register / List in both packages | 3.4 |  | 0.528 |
| ns | 3892 |  | 94 | database package constants, sentinel errors and the registry map | 3.5 |  | 0.521 |
| walker |  | 4026 | 439 | Code::CodeKey { rung: Decl, file: migration.go, decl: 2, sub: 0, line: 18 } |  |  | 0.521 |
| walker |  | 4108 | 82 | Code::CodeKey { rung: Names, file: util.go, decl: 0, sub: 0, line: 0 } |  |  | 0.522 |
| walker |  | 4122 | 14 | Code::CodeKey { rung: Decl, file: util.go, decl: 1, sub: 0, line: 13 } |  |  | 0.522 |
| walker |  | 4134 | 12 | Code::CodeKey { rung: Names, file: log.go, decl: 0, sub: 0, line: 0 } |  |  | 0.522 |
| walker |  | 4200 | 66 | Code::CodeKey { rung: Decl, file: log.go, decl: 1, sub: 0, line: 5 } |  |  | 0.533 |
| walker |  | 4218 | 18 | Code::CodeKey { rung: Doc, file: util.go, decl: 5, sub: 0, line: 53 } |  |  | 0.534 |
| ns | 4219 |  | 327 | "How to implement a database driver" — the 7-step checklist | 3.6 |  | 0.519 |
| walker |  | 4237 | 19 | Code::CodeKey { rung: Doc, file: util.go, decl: 3, sub: 0, line: 32 } |  |  | 0.519 |
| walker |  | 4393 | 156 | Code::CodeKey { rung: Names, file: migrate.go, decl: 0, sub: 1, line: 0 } |  |  | 0.528 |
| walker |  | 4406 | 13 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 14, sub: 0, line: 193 } |  |  | 0.528 |
| walker |  | 4431 | 25 | Code::CodeKey { rung: Doc, file: log.go, decl: 1, sub: 0, line: 5 } |  |  | 0.536 |
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
| walker |  | 5739 | 43 | Code::CodeKey { rung: Doc, file: util.go, decl: 4, sub: 0, line: 45 } |  |  | 0.513 |
| walker |  | 5827 | 88 | Plaintext::DeclSurface { file: docker-deploy.sh } |  |  | 0.513 |
| ns | 5832 |  | 139 | Shared driver helpers: GenerateAdvisoryLockId and CasRestoreOnErr | 3.11 |  | 0.509 |
| walker |  | 5838 | 11 | Plaintext::Whole { file: docker-deploy.sh } |  |  | 0.509 |
| walker |  | 5972 | 134 | Code::CodeKey { rung: Names, file: database/driver.go, decl: 0, sub: 0, line: 0 } |  |  | 0.519 |
| walker |  | 5981 | 9 | Code::CodeKey { rung: Decl, file: database/driver.go, decl: 1, sub: 0, line: 15 } |  |  | 0.522 |
| walker |  | 5992 | 11 | Code::CodeKey { rung: Doc, file: database/driver.go, decl: 7, sub: 0, line: 102 } |  |  | 0.522 |
| walker |  | 6003 | 11 | Code::CodeKey { rung: Doc, file: database/driver.go, decl: 8, sub: 0, line: 115 } |  |  | 0.522 |
| walker |  | 6015 | 12 | Code::CodeKey { rung: Doc, file: database/driver.go, decl: 6, sub: 0, line: 85 } |  |  | 0.522 |
| ns | 6016 |  | 184 | internal/url: SchemeFromURL | 3.12 |  | 0.512 |
| walker |  | 6042 | 27 | Code::CodeKey { rung: Names, file: database/error.go, decl: 0, sub: 0, line: 0 } |  |  | 0.513 |
| walker |  | 6146 | 104 | Code::CodeKey { rung: Decl, file: database/error.go, decl: 1, sub: 0, line: 8 } |  |  | 0.521 |
| walker |  | 6164 | 18 | Code::CodeKey { rung: Doc, file: database/error.go, decl: 1, sub: 0, line: 8 } |  |  | 0.525 |
| walker |  | 6210 | 46 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 31, sub: 0, line: 938 } |  |  | 0.525 |
| ns | 6248 |  | 232 | Exemplar driver: postgres registration, defaults and Config | 3.13 |  | 0.512 |
| walker |  | 6261 | 51 | Code::CodeKey { rung: Doc, file: util.go, decl: 1, sub: 0, line: 13 } |  |  | 0.512 |
| ns | 6457 |  | 209 | Migration filename grammar (MIGRATIONS.md:10-23) | 4.1 |  | 0.505 |
| walker |  | 6462 | 201 | Code::CodeKey { rung: Decl, file: database/driver.go, decl: 5, sub: 0, line: 45 } |  |  | 0.515 |
| ns | 6639 |  | 182 | source/parse.go: ErrParse, the Regex, and the Parse signature | 4.2 |  | 0.506 |
| walker |  | 6755 | 293 | Code::CodeKey { rung: Decl, file: database/driver.go, decl: 5, sub: 1, line: 45 } |  |  | 0.552 |
| walker |  | 6807 | 52 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 28, sub: 0, line: 815 } |  |  | 0.552 |
| walker |  | 6860 | 53 | Code::CodeKey { rung: Names, file: dktesting/dktesting.go, decl: 0, sub: 0, line: 0 } |  |  | 0.552 |
| walker |  | 6878 | 18 | Code::CodeKey { rung: Decl, file: dktesting/dktesting.go, decl: 3, sub: 0, line: 44 } |  |  | 0.552 |
| ns | 6897 |  | 258 | source.Direction and source.Migration | 4.3 |  | 0.539 |
| walker |  | 6903 | 25 | Code::CodeKey { rung: Decl, file: dktesting/dktesting.go, decl: 1, sub: 0, line: 14 } |  |  | 0.539 |
| walker |  | 6916 | 13 | Code::CodeKey { rung: Doc, file: dktesting/dktesting.go, decl: 1, sub: 0, line: 14 } |  |  | 0.539 |
| walker |  | 6929 | 13 | Code::CodeKey { rung: Doc, file: dktesting/dktesting.go, decl: 3, sub: 0, line: 44 } |  |  | 0.539 |
| walker |  | 6950 | 21 | Code::CodeKey { rung: Doc, file: dktesting/dktesting.go, decl: 2, sub: 0, line: 20 } |  |  | 0.539 |
| walker |  | 7004 | 54 | Code::CodeKey { rung: Doc, file: util.go, decl: 2, sub: 0, line: 21 } |  |  | 0.539 |
| ns | 7180 |  | 283 | source.Migrations: the in-memory index every directory-tree driver uses | 4.4 |  | 0.528 |
| walker |  | 7297 | 293 | Code::CodeKey { rung: Names, file: source/migration.go, decl: 0, sub: 0, line: 0 } |  |  | 0.535 |
| walker |  | 7306 | 9 | Code::CodeKey { rung: Decl, file: source/migration.go, decl: 2, sub: 0, line: 10 } |  |  | 0.536 |
| walker |  | 7334 | 28 | Code::CodeKey { rung: Decl, file: source/migration.go, decl: 4, sub: 0, line: 36 } |  |  | 0.539 |
| ns | 7384 |  | 204 | migrate.Migration: type location, buffer default, and complete method set | 4.5 |  | 0.546 |
| walker |  | 7472 | 138 | Code::CodeKey { rung: Decl, file: source/migration.go, decl: 3, sub: 0, line: 18 } |  |  | 0.559 |
| walker |  | 7484 | 12 | Code::CodeKey { rung: Doc, file: source/migration.go, decl: 1, sub: 0, line: 8 } |  |  | 0.561 |
| walker |  | 7565 | 81 | Code::CodeKey { rung: Names, file: source/driver.go, decl: 0, sub: 0, line: 0 } |  |  | 0.566 |
| walker |  | 7576 | 11 | Code::CodeKey { rung: Doc, file: source/driver.go, decl: 5, sub: 0, line: 97 } |  |  | 0.566 |
| ns | 7583 |  | 199 | CLI entry points and the cli/ deprecation | 5.1 |  | 0.556 |
| walker |  | 7587 | 11 | Code::CodeKey { rung: Doc, file: source/driver.go, decl: 6, sub: 0, line: 110 } |  |  | 0.556 |
| walker |  | 7599 | 12 | Code::CodeKey { rung: Doc, file: source/driver.go, decl: 4, sub: 0, line: 76 } |  |  | 0.556 |
| walker |  | 7624 | 25 | Code::CodeKey { rung: Doc, file: source/migration.go, decl: 4, sub: 0, line: 36 } |  |  | 0.560 |
| walker |  | 7724 | 100 | Code::CodeKey { rung: Names, file: source/parse.go, decl: 0, sub: 0, line: 0 } |  |  | 0.563 |
| walker |  | 7735 | 11 | Code::CodeKey { rung: Decl, file: source/parse.go, decl: 1, sub: 0, line: 9 } |  |  | 0.564 |
| walker |  | 7744 | 9 | Code::CodeKey { rung: Decl, file: source/parse.go, decl: 2, sub: 0, line: 13 } |  |  | 0.566 |
| walker |  | 7755 | 11 | Code::CodeKey { rung: Doc, file: source/parse.go, decl: 4, sub: 0, line: 25 } |  |  | 0.568 |
| walker |  | 7786 | 31 | Code::CodeKey { rung: Names, file: source/errors.go, decl: 0, sub: 0, line: 0 } |  |  | 0.568 |
| ns | 7789 |  | 206 | Complete internal/cli listing — the build-tag matrix | 5.2 |  | 0.580 |
| walker |  | 7806 | 20 | Code::CodeKey { rung: Decl, file: source/errors.go, decl: 1, sub: 0, line: 7 } |  |  | 0.580 |
| walker |  | 7814 | 8 | Code::CodeKey { rung: Doc, file: source/errors.go, decl: 2, sub: 0, line: 13 } |  |  | 0.580 |
| walker |  | 7829 | 15 | Code::CodeKey { rung: Body, file: source/errors.go, decl: 2, sub: 0, line: 13 } |  |  | 0.580 |
| walker |  | 7853 | 24 | Code::CodeKey { rung: Doc, file: source/errors.go, decl: 1, sub: 0, line: 7 } |  |  | 0.580 |
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
| ns | 8594 |  | 307 | Subcommand usage strings (internal/cli/main.go:18-33) | 5.5 |  | 0.600 |
| walker |  | 8684 | 162 | Markdown::Section { file: README.md, section_index: 11, keeps_default_concavity: false } |  |  | 0.600 |
| walker |  | 8758 | 74 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 12, sub: 0, line: 171 } |  |  | 0.600 |
| walker |  | 8834 | 76 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 23, sub: 0, line: 400 } |  |  | 0.600 |
| ns | 8847 |  | 253 | Complete command-implementation roster (internal/cli/commands.go) | 5.6 |  | 0.595 |
| walker |  | 8912 | 78 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 11, sub: 0, line: 145 } |  |  | 0.595 |
| ns | 8984 |  | 137 | A build-tag shim in full, and the tag-name traps | 5.7 |  | 0.589 |
| walker |  | 8991 | 79 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 10, sub: 0, line: 119 } |  |  | 0.589 |
| ns | 9275 |  | 291 | Complete FAQ question roster (every #### heading) | 6.1 |  | 0.582 |
| walker |  | 9288 | 297 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.582 |
| ns | 9546 |  | 271 | Makefile: default driver tag sets and the test targets | 6.2 |  | 0.575 |
| ns | 9863 |  | 317 | Shared conformance-test harnesses (complete function rosters) | 6.3 |  | 0.569 |
| ns | 9944 |  | 81 | Remaining doc heading rosters (MIGRATIONS.md, GETTING_STARTED.md) | 6.4 |  | 0.567 |
| walker |  | 9984 | 696 | GoMod::File { file: go.mod } |  |  | 0.567 |

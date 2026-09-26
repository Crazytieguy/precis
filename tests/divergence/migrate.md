Score(3000)=0.545 I=0.696 C=0.427 ns_rows≤3K=17/48 grid(1000/1442/2080/3000/4327/6240/9000)=0.719/0.644/0.622/0.545/0.526/0.534/0.548

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
| walker |  | 841 | 9 | Fs::DirListing { dir: database/multistmt } |  |  | 0.733 |
| walker |  | 850 | 9 | Fs::DirListing { dir: database/snowflake } |  |  | 0.733 |
| walker |  | 859 | 9 | Fs::DirListing { dir: database/stub } |  |  | 0.733 |
| walker |  | 875 | 16 | Fs::DirListing { dir: database/mongodb } |  |  | 0.733 |
| walker |  | 891 | 16 | Fs::DirListing { dir: database/mysql } |  |  | 0.733 |
| walker |  | 907 | 16 | Fs::DirListing { dir: database/sqlite } |  |  | 0.733 |
| ns | 917 |  | 121 | All four Migrate constructors (full signatures) | 2.1 |  | 0.719 |
| walker |  | 925 | 18 | Fs::DirListing { dir: database/cassandra } |  |  | 0.719 |
| walker |  | 943 | 18 | Fs::DirListing { dir: database/clickhouse } |  |  | 0.719 |
| walker |  | 961 | 18 | Fs::DirListing { dir: database/firebird } |  |  | 0.719 |
| walker |  | 979 | 18 | Fs::DirListing { dir: database/redshift } |  |  | 0.719 |
| walker |  | 997 | 18 | Fs::DirListing { dir: database/spanner } |  |  | 0.719 |
| walker |  | 1015 | 18 | Fs::DirListing { dir: database/sqlcipher } |  |  | 0.719 |
| walker |  | 1033 | 18 | Fs::DirListing { dir: database/sqlite3 } |  |  | 0.719 |
| walker |  | 1051 | 18 | Fs::DirListing { dir: database/sqlserver } |  |  | 0.719 |
| walker |  | 1071 | 20 | Fs::DirListing { dir: database/ql } |  |  | 0.719 |
| walker |  | 1091 | 20 | Fs::DirListing { dir: database/rqlite } |  |  | 0.719 |
| ns | 1097 |  | 180 | Complete exported *Migrate method set (full signatures) | 2.2 |  | 0.690 |
| walker |  | 1113 | 22 | Fs::DirListing { dir: database/pgx } |  |  | 0.690 |
| walker |  | 1135 | 22 | Fs::DirListing { dir: database/postgres } |  |  | 0.690 |
| walker |  | 1159 | 24 | Fs::DirListing { dir: database/yugabytedb } |  |  | 0.690 |
| walker |  | 1185 | 26 | Fs::DirListing { dir: database/cockroachdb } |  |  | 0.690 |
| walker |  | 1211 | 26 | Fs::DirListing { dir: database/neo4j } |  |  | 0.690 |
| walker |  | 1226 | 15 | Fs::DirListing { dir: database/pgx/v5 } |  |  | 0.690 |
| walker |  | 1247 | 21 | Code::CodeKey { rung: ModuleDoc, file: database/multistmt/parse.go, decl: 0, sub: 0, line: 0 } |  |  | 0.690 |
| walker |  | 1256 | 9 | Fs::DirListing { dir: database/testing } |  |  | 0.690 |
| walker |  | 1266 | 10 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.690 |
| walker |  | 1324 | 58 | Code::CodeKey { rung: ModuleDoc, file: database/driver.go, decl: 0, sub: 0, line: 0 } |  |  | 0.690 |
| ns | 1349 |  | 252 | Package-level sentinel errors and tuning defaults | 2.3 |  | 0.643 |
| walker |  | 1382 | 58 | Code::CodeKey { rung: ModuleDoc, file: source/driver.go, decl: 0, sub: 0, line: 0 } |  |  | 0.644 |
| walker |  | 1459 | 77 | Plaintext::Whole { file: Makefile } |  |  | 0.644 |
| ns | 1462 |  | 113 | Logger interface (log.go, whole file) | 2.4 |  | 0.615 |
| walker |  | 1596 | 137 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.667 |
| walker |  | 1614 | 18 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.667 |
| ns | 1738 |  | 276 | The Migrate struct and its documented public knobs | 2.5 |  | 0.611 |
| walker |  | 1739 | 125 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.656 |
| ns | 1916 |  | 178 | Structured error types ErrShortLimit and ErrDirty | 2.6 |  | 0.622 |
| walker |  | 1926 | 187 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: true } |  |  | 0.622 |
| walker |  | 1973 | 47 | Markdown::Section { file: README.md, section_index: 10, keeps_default_concavity: false } |  |  | 0.622 |
| walker |  | 1977 | 4 | Fs::DirListing { dir: source/testing } |  |  | 0.622 |
| walker |  | 2033 | 56 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.622 |
| ns | 2132 |  | 216 | Complete unexported machinery roster of migrate.go (names only) | 2.7 |  | 0.598 |
| walker |  | 2239 | 206 | Fs::DirListing { dir: internal/cli } |  |  | 0.602 |
| walker |  | 2244 | 5 | Fs::DirListing { dir: source/go_bindata/testdata } |  |  | 0.602 |
| walker |  | 2336 | 92 | Code::CodeKey { rung: ModuleDoc, file: source/godoc_vfs/vfs.go, decl: 0, sub: 0, line: 0 } |  |  | 0.570 |
| ns | 2336 |  | 204 | Up() in full — the canonical lock/dirty/read/run pipeline | 2.8 | 2.2 | 0.570 |
| walker |  | 2597 | 261 | Plaintext::Whole { file: Dockerfile } |  |  | 0.570 |
| walker |  | 2707 | 110 | Code::CodeKey { rung: ModuleDoc, file: source/iofs/doc.go, decl: 0, sub: 0, line: 0 } |  |  | 0.570 |
| walker |  | 2714 | 7 | Fs::DirListing { dir: source/go_bindata/examples/migrations } |  |  | 0.570 |
| ns | 2737 |  | 401 | runMigrations(): the dirty-flag write protocol | 2.9 | 2.7 | 0.525 |
| walker |  | 2907 | 193 | Code::CodeKey { rung: Names, file: migrate.go, decl: 0, sub: 0, line: 0 } |  |  | 0.539 |
| walker |  | 2912 | 5 | Code::CodeKey { rung: Decl, file: migrate.go, decl: 3, sub: 0, line: 29 } |  |  | 0.540 |
| walker |  | 2922 | 10 | Code::CodeKey { rung: Decl, file: migrate.go, decl: 6, sub: 0, line: 48 } |  |  | 0.542 |
| walker |  | 2935 | 13 | Code::CodeKey { rung: Decl, file: migrate.go, decl: 4, sub: 0, line: 39 } |  |  | 0.545 |
| walker |  | 3117 | 182 | Code::CodeKey { rung: Decl, file: migrate.go, decl: 8, sub: 0, line: 56 } |  |  | 0.575 |
| walker |  | 3126 | 9 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 5, sub: 0, line: 44 } |  |  | 0.578 |
| ns | 3189 |  | 452 | lock(): mutex, ErrLocked and the LockTimeout race | 2.10 | 2.7 | 0.526 |
| walker |  | 3224 | 98 | Code::CodeKey { rung: Names, file: migration.go, decl: 0, sub: 0, line: 0 } |  |  | 0.526 |
| walker |  | 3241 | 17 | Code::CodeKey { rung: Decl, file: migration.go, decl: 3, sub: 0, line: 77 } |  |  | 0.526 |
| ns | 3449 |  | 260 | FilterCustomQuery and suint (util.go:42-63) | 2.11 |  | 0.507 |
| ns | 3579 |  | 130 | Package docs for the two driver packages | 3.1 |  | 0.512 |
| walker |  | 3629 | 388 | Code::CodeKey { rung: Decl, file: migration.go, decl: 2, sub: 0, line: 18 } |  |  | 0.512 |
| walker |  | 3645 | 16 | Code::CodeKey { rung: Doc, file: migration.go, decl: 4, sub: 0, line: 107 } |  |  | 0.512 |
| ns | 3648 |  | 69 | database.Driver method roster (names only) | 3.2 |  | 0.506 |
| walker |  | 3661 | 16 | Code::CodeKey { rung: Doc, file: migration.go, decl: 5, sub: 0, line: 112 } |  |  | 0.506 |
| ns | 3710 |  | 62 | source.Driver method roster (names only) | 3.3 |  | 0.500 |
| ns | 3798 |  | 88 | Driver registries: Open / Register / List in both packages | 3.4 |  | 0.496 |
| walker |  | 3825 | 164 | Code::CodeKey { rung: Names, file: migrate.go, decl: 0, sub: 1, line: 0 } |  |  | 0.506 |
| walker |  | 3838 | 13 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 13, sub: 0, line: 193 } |  |  | 0.506 |
| ns | 3892 |  | 94 | database package constants, sentinel errors and the registry map | 3.5 |  | 0.499 |
| walker |  | 3906 | 68 | Code::CodeKey { rung: Names, file: util.go, decl: 0, sub: 0, line: 0 } |  |  | 0.499 |
| walker |  | 3920 | 14 | Code::CodeKey { rung: Decl, file: util.go, decl: 1, sub: 0, line: 13 } |  |  | 0.499 |
| walker |  | 3932 | 12 | Code::CodeKey { rung: Names, file: log.go, decl: 0, sub: 0, line: 0 } |  |  | 0.500 |
| walker |  | 3998 | 66 | Code::CodeKey { rung: Decl, file: log.go, decl: 1, sub: 0, line: 5 } |  |  | 0.510 |
| walker |  | 4016 | 18 | Code::CodeKey { rung: Doc, file: util.go, decl: 4, sub: 0, line: 53 } |  |  | 0.511 |
| walker |  | 4035 | 19 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 2, sub: 0, line: 27 } |  |  | 0.514 |
| walker |  | 4054 | 19 | Code::CodeKey { rung: Doc, file: util.go, decl: 3, sub: 0, line: 32 } |  |  | 0.514 |
| walker |  | 4191 | 137 | Code::CodeKey { rung: Names, file: migrate.go, decl: 0, sub: 2, line: 0 } |  |  | 0.533 |
| walker |  | 4203 | 12 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 18, sub: 0, line: 307 } |  |  | 0.533 |
| ns | 4219 |  | 327 | "How to implement a database driver" — the 7-step checklist | 3.6 |  | 0.518 |
| walker |  | 4228 | 25 | Code::CodeKey { rung: Doc, file: log.go, decl: 1, sub: 0, line: 5 } |  |  | 0.526 |
| walker |  | 4256 | 28 | Code::CodeKey { rung: Doc, file: migration.go, decl: 6, sub: 0, line: 122 } |  |  | 0.526 |
| walker |  | 4268 | 12 | Code::CodeKey { rung: Names, file: cli/main.go, decl: 0, sub: 0, line: 0 } |  |  | 0.526 |
| walker |  | 4281 | 13 | Code::CodeKey { rung: Decl, file: cli/main.go, decl: 1, sub: 0, line: 6 } |  |  | 0.526 |
| walker |  | 4294 | 13 | Code::CodeKey { rung: Doc, file: cli/main.go, decl: 1, sub: 0, line: 6 } |  |  | 0.526 |
| walker |  | 4306 | 12 | Code::CodeKey { rung: Names, file: cli/version.go, decl: 0, sub: 0, line: 0 } |  |  | 0.526 |
| walker |  | 4337 | 31 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 4, sub: 0, line: 39 } |  |  | 0.533 |
| walker |  | 4444 | 107 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.533 |
| walker |  | 4476 | 32 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 14, sub: 0, line: 212 } |  |  | 0.533 |
| ns | 4503 |  | 284 | "How to implement a source driver" — checklist and guidelines | 3.7 |  | 0.520 |
| walker |  | 4509 | 33 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 9, sub: 0, line: 85 } |  |  | 0.520 |
| walker |  | 4542 | 33 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 16, sub: 0, line: 265 } |  |  | 0.522 |
| walker |  | 4575 | 33 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 17, sub: 0, line: 287 } |  |  | 0.522 |
| walker |  | 4674 | 99 | Markdown::Section { file: README.md, section_index: 12, keeps_default_concavity: false } |  |  | 0.522 |
| walker |  | 4708 | 34 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 21, sub: 0, line: 383 } |  |  | 0.522 |
| walker |  | 4818 | 110 | Code::CodeKey { rung: Names, file: database/driver.go, decl: 0, sub: 0, line: 0 } |  |  | 0.529 |
| walker |  | 4821 | 3 | Code::CodeKey { rung: Decl, file: database/driver.go, decl: 1, sub: 0, line: 15 } |  |  | 0.530 |
| walker |  | 4832 | 11 | Code::CodeKey { rung: Doc, file: database/driver.go, decl: 5, sub: 0, line: 102 } |  |  | 0.530 |
| walker |  | 4843 | 11 | Code::CodeKey { rung: Doc, file: database/driver.go, decl: 6, sub: 0, line: 115 } |  |  | 0.530 |
| walker |  | 4855 | 12 | Code::CodeKey { rung: Doc, file: database/driver.go, decl: 4, sub: 0, line: 85 } |  |  | 0.530 |
| walker |  | 4882 | 27 | Code::CodeKey { rung: Names, file: database/error.go, decl: 0, sub: 0, line: 0 } |  |  | 0.530 |
| ns | 4943 |  | 440 | database.Driver in full: per-method semantic contracts | 3.8 | 3.2 | 0.505 |
| walker |  | 4986 | 104 | Code::CodeKey { rung: Decl, file: database/error.go, decl: 1, sub: 0, line: 8 } |  |  | 0.505 |
| walker |  | 5001 | 15 | Code::CodeKey { rung: Doc, file: cli/version.go, decl: 1, sub: 0, line: 4 } |  |  | 0.505 |
| walker |  | 5089 | 88 | Plaintext::DeclSurface { file: docker-deploy.sh } |  |  | 0.505 |
| walker |  | 5100 | 11 | Plaintext::Whole { file: docker-deploy.sh } |  |  | 0.505 |
| walker |  | 5137 | 37 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 15, sub: 0, line: 234 } |  |  | 0.505 |
| walker |  | 5174 | 37 | Code::CodeKey { rung: Doc, file: migration.go, decl: 1, sub: 0, line: 13 } |  |  | 0.506 |
| walker |  | 5214 | 40 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 20, sub: 0, line: 365 } |  |  | 0.506 |
| walker |  | 5254 | 40 | Code::CodeKey { rung: Doc, file: migration.go, decl: 2, sub: 0, line: 18 } |  |  | 0.506 |
| walker |  | 5272 | 18 | Code::CodeKey { rung: Doc, file: database/error.go, decl: 1, sub: 0, line: 8 } |  |  | 0.506 |
| ns | 5452 |  | 509 | source.Driver in full: per-method semantic contracts | 3.9 | 3.3 | 0.484 |
| walker |  | 5468 | 196 | Code::CodeKey { rung: Decl, file: database/driver.go, decl: 3, sub: 0, line: 45 } |  |  | 0.495 |
| ns | 5693 |  | 241 | database.Error: the query-level error type | 3.10 |  | 0.496 |
| walker |  | 5766 | 298 | Code::CodeKey { rung: Decl, file: database/driver.go, decl: 3, sub: 1, line: 45 } |  |  | 0.548 |
| walker |  | 5819 | 53 | Code::CodeKey { rung: Names, file: dktesting/dktesting.go, decl: 0, sub: 0, line: 0 } |  |  | 0.548 |
| ns | 5832 |  | 139 | Shared driver helpers: GenerateAdvisoryLockId and CasRestoreOnErr | 3.11 |  | 0.543 |
| walker |  | 5837 | 18 | Code::CodeKey { rung: Decl, file: dktesting/dktesting.go, decl: 3, sub: 0, line: 44 } |  |  | 0.543 |
| walker |  | 5862 | 25 | Code::CodeKey { rung: Decl, file: dktesting/dktesting.go, decl: 1, sub: 0, line: 14 } |  |  | 0.543 |
| walker |  | 5875 | 13 | Code::CodeKey { rung: Doc, file: dktesting/dktesting.go, decl: 1, sub: 0, line: 14 } |  |  | 0.543 |
| walker |  | 5888 | 13 | Code::CodeKey { rung: Doc, file: dktesting/dktesting.go, decl: 3, sub: 0, line: 44 } |  |  | 0.543 |
| ns | 6016 |  | 184 | internal/url: SchemeFromURL | 3.12 |  | 0.533 |
| walker |  | 6120 | 232 | Code::CodeKey { rung: Names, file: source/migration.go, decl: 0, sub: 0, line: 0 } |  |  | 0.533 |
| walker |  | 6125 | 5 | Code::CodeKey { rung: Decl, file: source/migration.go, decl: 2, sub: 0, line: 10 } |  |  | 0.533 |
| walker |  | 6153 | 28 | Code::CodeKey { rung: Decl, file: source/migration.go, decl: 4, sub: 0, line: 36 } |  |  | 0.534 |
| ns | 6248 |  | 232 | Exemplar driver: postgres registration, defaults and Config | 3.13 |  | 0.521 |
| walker |  | 6291 | 138 | Code::CodeKey { rung: Decl, file: source/migration.go, decl: 3, sub: 0, line: 18 } |  |  | 0.522 |
| walker |  | 6303 | 12 | Code::CodeKey { rung: Doc, file: source/migration.go, decl: 1, sub: 0, line: 8 } |  |  | 0.522 |
| walker |  | 6357 | 54 | Code::CodeKey { rung: Names, file: source/driver.go, decl: 0, sub: 0, line: 0 } |  |  | 0.528 |
| walker |  | 6368 | 11 | Code::CodeKey { rung: Doc, file: source/driver.go, decl: 3, sub: 0, line: 97 } |  |  | 0.528 |
| walker |  | 6379 | 11 | Code::CodeKey { rung: Doc, file: source/driver.go, decl: 4, sub: 0, line: 110 } |  |  | 0.528 |
| walker |  | 6391 | 12 | Code::CodeKey { rung: Doc, file: source/driver.go, decl: 2, sub: 0, line: 76 } |  |  | 0.528 |
| ns | 6457 |  | 209 | Migration filename grammar (MIGRATIONS.md:10-23) | 4.1 |  | 0.521 |
| walker |  | 6503 | 112 | Code::CodeKey { rung: Names, file: source/parse.go, decl: 0, sub: 0, line: 0 } |  |  | 0.521 |
| walker |  | 6506 | 3 | Code::CodeKey { rung: Decl, file: source/parse.go, decl: 1, sub: 0, line: 9 } |  |  | 0.521 |
| walker |  | 6511 | 5 | Code::CodeKey { rung: Decl, file: source/parse.go, decl: 2, sub: 0, line: 13 } |  |  | 0.521 |
| walker |  | 6522 | 11 | Code::CodeKey { rung: Doc, file: source/parse.go, decl: 4, sub: 0, line: 25 } |  |  | 0.521 |
| walker |  | 6553 | 31 | Code::CodeKey { rung: Names, file: source/errors.go, decl: 0, sub: 0, line: 0 } |  |  | 0.521 |
| walker |  | 6573 | 20 | Code::CodeKey { rung: Decl, file: source/errors.go, decl: 1, sub: 0, line: 7 } |  |  | 0.521 |
| walker |  | 6581 | 8 | Code::CodeKey { rung: Doc, file: source/errors.go, decl: 2, sub: 0, line: 13 } |  |  | 0.521 |
| walker |  | 6596 | 15 | Code::CodeKey { rung: Body, file: source/errors.go, decl: 2, sub: 0, line: 13 } |  |  | 0.521 |
| walker |  | 6617 | 21 | Code::CodeKey { rung: Doc, file: dktesting/dktesting.go, decl: 2, sub: 0, line: 20 } |  |  | 0.521 |
| ns | 6639 |  | 182 | source/parse.go: ErrParse, the Regex, and the Parse signature | 4.2 |  | 0.521 |
| walker |  | 6668 | 51 | Code::CodeKey { rung: Doc, file: util.go, decl: 1, sub: 0, line: 13 } |  |  | 0.521 |
| walker |  | 6848 | 180 | Code::CodeKey { rung: Decl, file: source/driver.go, decl: 1, sub: 0, line: 35 } |  |  | 0.528 |
| ns | 6897 |  | 258 | source.Direction and source.Migration | 4.3 |  | 0.532 |
| walker |  | 7111 | 263 | Code::CodeKey { rung: Decl, file: source/driver.go, decl: 1, sub: 1, line: 35 } |  |  | 0.558 |
| ns | 7180 |  | 283 | source.Migrations: the in-memory index every directory-tree driver uses | 4.4 |  | 0.552 |
| walker |  | 7219 | 108 | Code::CodeKey { rung: Decl, file: source/driver.go, decl: 1, sub: 2, line: 35 } |  |  | 0.568 |
| walker |  | 7273 | 54 | Code::CodeKey { rung: Doc, file: util.go, decl: 2, sub: 0, line: 21 } |  |  | 0.568 |
| walker |  | 7277 | 4 | Fs::DirListing { dir: cmd/migrate/examples } |  |  | 0.568 |
| walker |  | 7301 | 24 | Code::CodeKey { rung: Doc, file: source/errors.go, decl: 1, sub: 0, line: 7 } |  |  | 0.568 |
| walker |  | 7326 | 25 | Code::CodeKey { rung: Doc, file: source/migration.go, decl: 4, sub: 0, line: 36 } |  |  | 0.571 |
| walker |  | 7338 | 12 | Code::CodeKey { rung: Names, file: cmd/migrate/main.go, decl: 0, sub: 0, line: 0 } |  |  | 0.571 |
| walker |  | 7351 | 13 | Code::CodeKey { rung: Decl, file: cmd/migrate/main.go, decl: 1, sub: 0, line: 5 } |  |  | 0.571 |
| walker |  | 7363 | 12 | Code::CodeKey { rung: Names, file: cmd/migrate/version.go, decl: 0, sub: 0, line: 0 } |  |  | 0.571 |
| ns | 7384 |  | 204 | migrate.Migration: type location, buffer default, and complete method set | 4.5 |  | 0.576 |
| walker |  | 7525 | 162 | Markdown::Section { file: README.md, section_index: 11, keeps_default_concavity: false } |  |  | 0.576 |
| ns | 7583 |  | 199 | CLI entry points and the cli/ deprecation | 5.1 |  | 0.568 |
| walker |  | 7587 | 62 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 19, sub: 0, line: 321 } |  |  | 0.568 |
| ns | 7789 |  | 206 | Complete internal/cli listing — the build-tag matrix | 5.2 |  | 0.580 |
| walker |  | 7884 | 297 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.580 |
| walker |  | 7899 | 15 | Code::CodeKey { rung: Doc, file: cmd/migrate/version.go, decl: 1, sub: 0, line: 4 } |  |  | 0.582 |
| walker |  | 7913 | 14 | Code::CodeKey { rung: Names, file: internal/cli/main.go, decl: 0, sub: 0, line: 0 } |  |  | 0.582 |
| ns | 7955 |  | 166 | cli.Main and the complete global flag set | 5.3 |  | 0.577 |
| ns | 8287 |  | 332 | The `migrate -help` usage text, from source | 5.4 |  | 0.565 |
| ns | 8594 |  | 307 | Subcommand usage strings (internal/cli/main.go:18-33) | 5.5 |  | 0.559 |
| walker |  | 8736 | 823 | GoMod::File { file: go.mod } |  |  | 0.559 |
| ns | 8847 |  | 253 | Complete command-implementation roster (internal/cli/commands.go) | 5.6 |  | 0.554 |
| ns | 8984 |  | 137 | A build-tag shim in full, and the tag-name traps | 5.7 |  | 0.548 |
| walker |  | 9127 | 391 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.548 |
| walker |  | 9201 | 74 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 12, sub: 0, line: 171 } |  |  | 0.548 |
| ns | 9275 |  | 291 | Complete FAQ question roster (every #### heading) | 6.1 |  | 0.542 |
| walker |  | 9279 | 78 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 11, sub: 0, line: 145 } |  |  | 0.542 |
| walker |  | 9358 | 79 | Code::CodeKey { rung: Doc, file: migrate.go, decl: 10, sub: 0, line: 119 } |  |  | 0.542 |
| ns | 9546 |  | 271 | Makefile: default driver tag sets and the test targets | 6.2 |  | 0.536 |
| walker |  | 9767 | 409 | Code::CodeKey { rung: Names, file: database/pgx/pgx.go, decl: 0, sub: 0, line: 0 } |  |  | 0.536 |
| walker |  | 9770 | 3 | Code::CodeKey { rung: Decl, file: database/pgx/pgx.go, decl: 2, sub: 0, line: 38 } |  |  | 0.536 |
| walker |  | 9773 | 3 | Code::CodeKey { rung: Decl, file: database/pgx/pgx.go, decl: 3, sub: 0, line: 47 } |  |  | 0.536 |
| walker |  | 9778 | 5 | Code::CodeKey { rung: Decl, file: database/pgx/pgx.go, decl: 1, sub: 0, line: 27 } |  |  | 0.536 |
| walker |  | 9859 | 81 | Code::CodeKey { rung: Decl, file: database/pgx/pgx.go, decl: 5, sub: 0, line: 68 } |  |  | 0.536 |
| ns | 9863 |  | 317 | Shared conformance-test harnesses (complete function rosters) | 6.3 |  | 0.530 |
| ns | 9944 |  | 81 | Remaining doc heading rosters (MIGRATIONS.md, GETTING_STARTED.md) | 6.4 |  | 0.528 |
| walker |  | 9954 | 95 | Code::CodeKey { rung: Decl, file: database/pgx/pgx.go, decl: 4, sub: 0, line: 54 } |  |  | 0.528 |
| walker |  | 9990 | 36 | Code::CodeKey { rung: Doc, file: source/parse.go, decl: 3, sub: 0, line: 22 } |  |  | 0.533 |

Score(3000)=0.745 I=0.914 C=0.607 ns_rows≤3K=21/54 grid(1000/1442/2080/3000/4327/6240/9000)=0.673/0.741/0.770/0.745/0.661/0.621/0.668

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 38 | 38 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 54 |  | 54 | Identity: README title + package name/version/description | 1.1 |  | 0.000 |
| ns | 92 |  | 38 | Complete repository root listing | 1.2 |  | 0.536 |
| walker |  | 135 | 97 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.568 |
| walker |  | 163 | 28 | Fs::DirListing { dir: images } |  |  | 0.569 |
| ns | 173 |  | 81 | README lede paragraph — scope and supported databases | 1.3 | 1.1 | 0.569 |
| walker |  | 197 | 34 | Fs::DirListing { dir: peepdb } |  |  | 0.601 |
| walker |  | 238 | 41 | Fs::DirListing { dir: docs } |  |  | 0.616 |
| ns | 252 |  | 79 | Source roster: complete listing of peepdb/ and peepdb/db/ | 1.4 |  | 0.441 |
| walker |  | 283 | 45 | Fs::DirListing { dir: peepdb/db } |  |  | 0.692 |
| ns | 297 |  | 45 | peepdb/core.py: every top-level function, names only | 1.5 |  | 0.647 |
| walker |  | 359 | 76 | Toml::Identity { file: project.toml } |  |  | 0.945 |
| walker |  | 369 | 10 | Fs::DirListing { dir: .github/workflows } |  |  | 0.947 |
| ns | 378 |  | 81 | peepdb/cli.py: every top-level definition, names only | 1.6 |  | 0.845 |
| walker |  | 396 | 27 | Toml::Operational { file: project.toml } |  |  | 0.846 |
| ns | 497 |  | 119 | peepdb/db/__init__.py — backend class ↔ module map | 1.7 |  | 0.744 |
| walker |  | 501 | 105 | Markdown::CommandBlock { file: CONTRIBUTING.md, row: 32 } |  |  | 0.744 |
| ns | 676 |  | 179 | README feature bullets | 1.8 |  | 0.682 |
| walker |  | 694 | 193 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.692 |
| walker |  | 712 | 18 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.692 |
| ns | 831 |  | 155 | peepdb/config.py: every top-level definition, names only | 1.9 |  | 0.622 |
| walker |  | 880 | 168 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.708 |
| walker |  | 907 | 27 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.708 |
| ns | 908 |  | 77 | Entry points: console script, __main__.py, exceptions.py in full | 1.10 |  | 0.673 |
| walker |  | 935 | 28 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.673 |
| walker |  | 981 | 46 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.673 |
| ns | 1027 |  | 119 | Complete listings of peepdb/tests, docs, images, .github/workflows | 1.11 |  | 0.655 |
| walker |  | 1173 | 192 | Toml::Dependencies { file: project.toml } |  |  | 0.656 |
| ns | 1209 |  | 182 | README section-heading map (every H2 and H3 after the feature list) | 1.12 | 1.1 | 0.684 |
| walker |  | 1299 | 126 | Plaintext::DeclSurface { file: docs/Gemfile } |  |  | 0.684 |
| walker |  | 1341 | 42 | Fs::DirListing { dir: peepdb/tests } |  |  | 0.749 |
| ns | 1441 |  | 232 | `peepdb save` full option decorator block (cli.py 53-64) | 2.1 | 1.6 | 0.710 |
| walker |  | 1452 | 111 | Code::CodeKey { rung: Names, file: peepdb/cli.py, decl: 0, sub: 0, line: 0 } |  |  | 0.758 |
| walker |  | 1460 | 8 | Code::CodeKey { rung: Decl, file: peepdb/cli.py, decl: 5, sub: 0, line: 86 } |  |  | 0.759 |
| walker |  | 1471 | 11 | Code::CodeKey { rung: Decl, file: peepdb/cli.py, decl: 1, sub: 0, line: 16 } |  |  | 0.759 |
| walker |  | 1488 | 17 | Code::CodeKey { rung: Decl, file: peepdb/cli.py, decl: 3, sub: 0, line: 25 } |  |  | 0.759 |
| walker |  | 1519 | 31 | Code::CodeKey { rung: Decl, file: peepdb/cli.py, decl: 7, sub: 0, line: 113 } |  |  | 0.760 |
| walker |  | 1560 | 41 | Code::CodeKey { rung: Decl, file: peepdb/cli.py, decl: 6, sub: 0, line: 97 } |  |  | 0.763 |
| ns | 1583 |  | 142 | `peepdb view` full option decorator block (cli.py 126-132) | 2.2 | 1.6 | 0.741 |
| ns | 1680 |  | 97 | Remaining Click decorators: group, version option, confirmations | 2.3 | 1.6 | 0.749 |
| walker |  | 1702 | 142 | Code::CodeKey { rung: Decl, file: peepdb/cli.py, decl: 8, sub: 0, line: 126 } |  |  | 0.779 |
| ns | 1922 |  | 242 | `view` docstring + connection resolution (cli.py 134-155) | 2.4 | 1.6 | 0.720 |
| walker |  | 1934 | 232 | Code::CodeKey { rung: Decl, file: peepdb/cli.py, decl: 4, sub: 0, line: 53 } |  |  | 0.767 |
| walker |  | 2206 | 272 | Code::CodeKey { rung: Names, file: peepdb/config.py, decl: 0, sub: 0, line: 0 } |  |  | 0.823 |
| ns | 2207 |  | 285 | `view` dispatch into peep_db + output rendering (cli.py 156-178) | 2.5 | 2.4 | 0.765 |
| walker |  | 2224 | 18 | Code::CodeKey { rung: Decl, file: peepdb/config.py, decl: 8, sub: 0, line: 29 } |  |  | 0.765 |
| walker |  | 2244 | 20 | Code::CodeKey { rung: Decl, file: peepdb/config.py, decl: 9, sub: 0, line: 41 } |  |  | 0.765 |
| walker |  | 2274 | 30 | Code::CodeKey { rung: Decl, file: peepdb/config.py, decl: 1, sub: 0, line: 15 } |  |  | 0.765 |
| walker |  | 2285 | 11 | Code::CodeKey { rung: Names, file: peepdb/exceptions.py, decl: 0, sub: 0, line: 0 } |  |  | 0.769 |
| walker |  | 2290 | 5 | Code::CodeKey { rung: Decl, file: peepdb/exceptions.py, decl: 1, sub: 0, line: 1 } |  |  | 0.772 |
| walker |  | 2304 | 14 | Code::CodeKey { rung: Names, file: peepdb/db/base.py, decl: 0, sub: 0, line: 0 } |  |  | 0.772 |
| ns | 2425 |  | 218 | `save` docstring and body (cli.py 66-83) | 2.6 | 1.6 | 0.734 |
| walker |  | 2492 | 188 | Code::CodeKey { rung: Decl, file: peepdb/db/base.py, decl: 1, sub: 0, line: 5 } |  |  | 0.736 |
| walker |  | 2497 | 5 | Code::CodeKey { rung: Body, file: peepdb/db/base.py, decl: 3, sub: 0, line: 17 } |  |  | 0.737 |
| walker |  | 2502 | 5 | Code::CodeKey { rung: Body, file: peepdb/db/base.py, decl: 4, sub: 0, line: 21 } |  |  | 0.737 |
| walker |  | 2507 | 5 | Code::CodeKey { rung: Body, file: peepdb/db/base.py, decl: 5, sub: 0, line: 25 } |  |  | 0.737 |
| walker |  | 2512 | 5 | Code::CodeKey { rung: Body, file: peepdb/db/base.py, decl: 6, sub: 0, line: 29 } |  |  | 0.738 |
| ns | 2515 |  | 90 | Bodies of `list`, `remove` and `remove-all` (cli.py) | 2.7 | 1.6 | 0.718 |
| walker |  | 2518 | 6 | Code::CodeKey { rung: Body, file: peepdb/db/base.py, decl: 8, sub: 0, line: 37 } |  |  | 0.719 |
| walker |  | 2652 | 134 | Code::CodeKey { rung: Names, file: peepdb/core.py, decl: 0, sub: 0, line: 0 } |  |  | 0.737 |
| walker |  | 2745 | 93 | Code::CodeKey { rung: Decl, file: peepdb/core.py, decl: 4, sub: 0, line: 56 } |  |  | 0.740 |
| ns | 2746 |  | 231 | BaseDatabase: class line + the four abstract methods + context manager | 3.1 |  | 0.736 |
| walker |  | 2871 | 126 | Plaintext::DeclSurface { file: requirements.txt } |  |  | 0.736 |
| ns | 2913 |  | 167 | BaseDatabase.__init__ and base.py imports | 3.2 | 3.1 | 0.714 |
| walker |  | 2981 | 110 | Code::CodeKey { rung: Names, file: peepdb/db/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.745 |
| walker |  | 2996 | 15 | Code::CodeKey { rung: Names, file: peepdb/db/mssql.py, decl: 0, sub: 0, line: 0 } |  |  | 0.745 |
| ns | 3026 |  | 113 | core.peep_db full signature (core.py 56-66) | 3.3 | 1.5 | 0.752 |
| walker |  | 3121 | 125 | Code::CodeKey { rung: Decl, file: peepdb/db/mssql.py, decl: 1, sub: 0, line: 6 } |  |  | 0.752 |
| walker |  | 3135 | 14 | Code::CodeKey { rung: Names, file: peepdb/db/sqlite.py, decl: 0, sub: 0, line: 0 } |  |  | 0.752 |
| walker |  | 3214 | 79 | Code::CodeKey { rung: Decl, file: peepdb/db/sqlite.py, decl: 1, sub: 0, line: 6 } |  |  | 0.753 |
| walker |  | 3245 | 31 | Code::CodeKey { rung: Names, file: peepdb/db/firebase.py, decl: 0, sub: 0, line: 0 } |  |  | 0.753 |
| ns | 3292 |  | 266 | connect_to_database: the first five engine branches (core.py 28-42) | 3.4 | 1.5 | 0.728 |
| walker |  | 3341 | 96 | Code::CodeKey { rung: Decl, file: peepdb/db/firebase.py, decl: 2, sub: 0, line: 9 } |  |  | 0.729 |
| walker |  | 3356 | 15 | Code::CodeKey { rung: Names, file: peepdb/db/mongodb.py, decl: 0, sub: 0, line: 0 } |  |  | 0.729 |
| walker |  | 3426 | 70 | Code::CodeKey { rung: Decl, file: peepdb/db/mongodb.py, decl: 1, sub: 0, line: 8 } |  |  | 0.730 |
| ns | 3453 |  | 161 | connect_to_database: the irregular branches and the failure path (core.py 43-53) | 3.5 | 3.4 | 0.714 |
| walker |  | 3465 | 39 | Code::CodeKey { rung: Decl, file: peepdb/db/mongodb.py, decl: 5, sub: 0, line: 42 } |  |  | 0.714 |
| walker |  | 3475 | 10 | Code::CodeKey { rung: Body, file: peepdb/db/mongodb.py, decl: 4, sub: 0, line: 39 } |  |  | 0.714 |
| walker |  | 3489 | 14 | Code::CodeKey { rung: Names, file: peepdb/db/oracle.py, decl: 0, sub: 0, line: 0 } |  |  | 0.714 |
| walker |  | 3556 | 67 | Code::CodeKey { rung: Decl, file: peepdb/db/oracle.py, decl: 1, sub: 0, line: 7 } |  |  | 0.715 |
| walker |  | 3580 | 24 | Code::CodeKey { rung: Decl, file: peepdb/db/oracle.py, decl: 5, sub: 0, line: 36 } |  |  | 0.715 |
| walker |  | 3595 | 15 | Code::CodeKey { rung: Names, file: peepdb/db/postgresql.py, decl: 0, sub: 0, line: 0 } |  |  | 0.715 |
| ns | 3651 |  | 198 | peep_db body: connect, fetch one table or all (core.py 67-79) | 3.6 | 3.3 | 0.698 |
| walker |  | 3674 | 79 | Code::CodeKey { rung: Decl, file: peepdb/db/postgresql.py, decl: 1, sub: 0, line: 6 } |  |  | 0.699 |
| walker |  | 3689 | 15 | Code::CodeKey { rung: Names, file: peepdb/db/mariadb.py, decl: 0, sub: 0, line: 0 } |  |  | 0.699 |
| walker |  | 3768 | 79 | Code::CodeKey { rung: Decl, file: peepdb/db/mariadb.py, decl: 1, sub: 0, line: 5 } |  |  | 0.700 |
| walker |  | 3783 | 15 | Code::CodeKey { rung: Names, file: peepdb/db/mysql.py, decl: 0, sub: 0, line: 0 } |  |  | 0.700 |
| walker |  | 3862 | 79 | Code::CodeKey { rung: Decl, file: peepdb/db/mysql.py, decl: 1, sub: 0, line: 5 } |  |  | 0.701 |
| walker |  | 3869 | 7 | Code::CodeKey { rung: Body, file: peepdb/cli.py, decl: 3, sub: 0, line: 25 } |  |  | 0.701 |
| walker |  | 3876 | 7 | Code::CodeKey { rung: Body, file: peepdb/cli.py, decl: 9, sub: 0, line: 181 } |  |  | 0.701 |
| ns | 3887 |  | 236 | peep_db body: table-vs-JSON post-processing and the finally-disconnect (core.py 80-98) | 3.7 | 3.6 | 0.678 |
| walker |  | 3890 | 14 | Code::CodeKey { rung: Body, file: peepdb/db/base.py, decl: 7, sub: 0, line: 33 } |  |  | 0.686 |
| walker |  | 3916 | 26 | Code::CodeKey { rung: Doc, file: peepdb/db/mongodb.py, decl: 5, sub: 0, line: 42 } |  |  | 0.686 |
| walker |  | 3924 | 8 | Code::CodeKey { rung: Body, file: peepdb/cli.py, decl: 5, sub: 0, line: 86 } |  |  | 0.686 |
| ns | 4134 |  | 247 | format_value: numeric/date rendering rules (core.py 119-140) | 3.8 | 1.5 | 0.661 |
| walker |  | 4156 | 232 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.661 |
| ns | 4335 |  | 201 | format_as_table: the tabulate grid layout (core.py 102-115) | 3.9 | 1.5 | 0.646 |
| walker |  | 4357 | 201 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.646 |
| walker |  | 4378 | 21 | Code::CodeKey { rung: Body, file: peepdb/db/firebase.py, decl: 5, sub: 0, line: 26 } |  |  | 0.646 |
| walker |  | 4393 | 15 | Code::CodeKey { rung: Body, file: peepdb/config.py, decl: 12, sub: 0, line: 70 } |  |  | 0.646 |
| walker |  | 4408 | 15 | Code::CodeKey { rung: Body, file: peepdb/config.py, decl: 13, sub: 0, line: 73 } |  |  | 0.646 |
| walker |  | 4449 | 41 | Code::CodeKey { rung: Doc, file: peepdb/cli.py, decl: 5, sub: 0, line: 86 } |  |  | 0.646 |
| ns | 4459 |  | 124 | core.py import header and module logger setup | 3.10 |  | 0.634 |
| walker |  | 4494 | 45 | Code::CodeKey { rung: Doc, file: peepdb/cli.py, decl: 7, sub: 0, line: 113 } |  |  | 0.634 |
| walker |  | 4525 | 31 | Code::CodeKey { rung: Body, file: peepdb/db/mariadb.py, decl: 4, sub: 0, line: 30 } |  |  | 0.634 |
| walker |  | 4572 | 47 | Code::CodeKey { rung: Doc, file: peepdb/cli.py, decl: 6, sub: 0, line: 97 } |  |  | 0.634 |
| walker |  | 4604 | 32 | Code::CodeKey { rung: Body, file: peepdb/db/oracle.py, decl: 4, sub: 0, line: 32 } |  |  | 0.634 |
| walker |  | 4638 | 34 | Code::CodeKey { rung: Body, file: peepdb/db/mysql.py, decl: 4, sub: 0, line: 29 } |  |  | 0.634 |
| walker |  | 4690 | 52 | Code::CodeKey { rung: Doc, file: peepdb/cli.py, decl: 8, sub: 0, line: 126 } |  |  | 0.636 |
| walker |  | 4723 | 33 | Code::CodeKey { rung: Body, file: peepdb/db/firebase.py, decl: 3, sub: 0, line: 10 } |  |  | 0.636 |
| walker |  | 4763 | 40 | Code::CodeKey { rung: Body, file: peepdb/db/postgresql.py, decl: 4, sub: 0, line: 30 } |  |  | 0.637 |
| walker |  | 4829 | 66 | Code::CodeKey { rung: Doc, file: peepdb/cli.py, decl: 4, sub: 0, line: 53 } |  |  | 0.640 |
| walker |  | 4874 | 45 | Code::CodeKey { rung: Body, file: peepdb/db/mssql.py, decl: 2, sub: 0, line: 7 } |  |  | 0.640 |
| walker |  | 4926 | 52 | Code::CodeKey { rung: Body, file: peepdb/db/sqlite.py, decl: 3, sub: 0, line: 22 } |  |  | 0.640 |
| ns | 4964 |  | 505 | Every class and method name across all eight backend modules | 4.1 | 3.1 | 0.670 |
| walker |  | 4974 | 48 | Code::CodeKey { rung: Body, file: peepdb/db/mongodb.py, decl: 3, sub: 0, line: 32 } |  |  | 0.670 |
| walker |  | 5028 | 54 | Code::CodeKey { rung: Body, file: peepdb/db/mariadb.py, decl: 3, sub: 0, line: 23 } |  |  | 0.670 |
| walker |  | 5082 | 54 | Code::CodeKey { rung: Body, file: peepdb/db/mysql.py, decl: 3, sub: 0, line: 22 } |  |  | 0.670 |
| walker |  | 5135 | 53 | Code::CodeKey { rung: Body, file: peepdb/db/oracle.py, decl: 3, sub: 0, line: 25 } |  |  | 0.670 |
| walker |  | 5189 | 54 | Code::CodeKey { rung: Body, file: peepdb/db/postgresql.py, decl: 3, sub: 0, line: 23 } |  |  | 0.670 |
| walker |  | 5243 | 54 | Code::CodeKey { rung: Body, file: peepdb/db/mssql.py, decl: 4, sub: 0, line: 28 } |  |  | 0.670 |
| ns | 5263 |  | 299 | The table/collection-listing query of every backend | 4.2 | 4.1 | 0.657 |
| ns | 5595 |  | 332 | MySQLDatabase — the canonical backend implementation | 4.3 | 4.1 | 0.634 |
| walker |  | 5897 | 654 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.613 |
| ns | 5897 |  | 302 | PostgreSQL, MariaDB and Oracle connect() — drivers, default ports, cursor factories | 4.4 | 4.1 | 0.613 |
| walker |  | 5978 | 81 | Code::CodeKey { rung: Body, file: peepdb/db/firebase.py, decl: 6, sub: 0, line: 30 } |  |  | 0.620 |
| walker |  | 6005 | 27 | Code::CodeKey { rung: Body, file: peepdb/cli.py, decl: 7, sub: 0, line: 113 } |  |  | 0.623 |
| walker |  | 6082 | 77 | Code::CodeKey { rung: Body, file: peepdb/config.py, decl: 9, sub: 0, line: 41 } |  |  | 0.623 |
| ns | 6156 |  | 259 | MongoDBDatabase.connect — URI construction from the generic parameters | 4.5 | 4.1 | 0.609 |
| walker |  | 6178 | 96 | Code::CodeKey { rung: Body, file: peepdb/db/base.py, decl: 2, sub: 0, line: 6 } |  |  | 0.621 |
| walker |  | 6281 | 103 | Code::CodeKey { rung: Body, file: peepdb/db/mssql.py, decl: 5, sub: 0, line: 35 } |  |  | 0.631 |
| walker |  | 6395 | 114 | Code::CodeKey { rung: Body, file: peepdb/db/firebase.py, decl: 4, sub: 0, line: 15 } |  |  | 0.632 |
| ns | 6435 |  | 279 | MSSQLDatabase: constructor extras and the ODBC connection string | 4.6 | 4.1 | 0.621 |
| walker |  | 6549 | 154 | Code::CodeKey { rung: Body, file: peepdb/db/sqlite.py, decl: 4, sub: 0, line: 29 } |  |  | 0.630 |
| ns | 6582 |  | 147 | FirebaseDatabase: the backend that does not call super().__init__ | 4.7 | 4.1 | 0.637 |
| walker |  | 6826 | 277 | Code::CodeKey { rung: Doc, file: peepdb/cli.py, decl: 3, sub: 0, line: 25 } |  |  | 0.637 |
| ns | 6899 |  | 317 | SQLiteDatabase connect and fetch_data — file-path handling and print-based logging | 4.8 | 4.1 | 0.623 |
| walker |  | 6981 | 155 | Code::CodeKey { rung: Body, file: peepdb/db/mysql.py, decl: 2, sub: 0, line: 6 } |  |  | 0.633 |
| walker |  | 7143 | 162 | Code::CodeKey { rung: Body, file: peepdb/db/mariadb.py, decl: 5, sub: 0, line: 34 } |  |  | 0.633 |
| ns | 7201 |  | 302 | The non-LIMIT/OFFSET pagination variants (MSSQL, Oracle, MongoDB, Firebase) | 4.9 | 4.1 | 0.620 |
| walker |  | 7248 | 105 | Code::CodeKey { rung: Body, file: peepdb/config.py, decl: 11, sub: 0, line: 60 } |  |  | 0.621 |
| ns | 7310 |  | 109 | Where credentials live: config paths and KeySecurity modes | 5.1 | 1.9 | 0.624 |
| walker |  | 7409 | 161 | Code::CodeKey { rung: Body, file: peepdb/db/oracle.py, decl: 2, sub: 0, line: 8 } |  |  | 0.626 |
| ns | 7495 |  | 185 | get_key_security_config / get_key / encrypt / decrypt bodies | 5.2 | 1.9 | 0.626 |
| walker |  | 7571 | 162 | Code::CodeKey { rung: Body, file: peepdb/db/postgresql.py, decl: 5, sub: 0, line: 34 } |  |  | 0.626 |
| walker |  | 7732 | 161 | Code::CodeKey { rung: Body, file: peepdb/db/mongodb.py, decl: 5, sub: 0, line: 42 } |  |  | 0.629 |
| ns | 7742 |  | 247 | The two key sources: PBKDF2 derivation and keyring fetch-or-create | 5.3 | 1.9 | 0.623 |
| walker |  | 7933 | 201 | Code::CodeKey { rung: Body, file: peepdb/core.py, decl: 5, sub: 0, line: 101 } |  |  | 0.640 |
| ns | 7987 |  | 245 | save_connection: the on-disk record shape and what is encrypted | 5.4 | 1.9 | 0.627 |
| walker |  | 8041 | 108 | Code::CodeKey { rung: Body, file: peepdb/config.py, decl: 10, sub: 0, line: 49 } |  |  | 0.635 |
| ns | 8167 |  | 180 | get_connection: the tuple contract and InvalidPassword | 5.5 | 1.9 | 0.625 |
| walker |  | 8203 | 162 | Code::CodeKey { rung: Body, file: peepdb/db/mysql.py, decl: 5, sub: 0, line: 33 } |  |  | 0.644 |
| ns | 8309 |  | 142 | add_key_security: first-run choice between keyring and password | 5.6 | 1.9 | 0.639 |
| walker |  | 8381 | 178 | Code::CodeKey { rung: Body, file: peepdb/db/sqlite.py, decl: 2, sub: 0, line: 7 } |  |  | 0.647 |
| ns | 8403 |  | 94 | list/remove/remove_all_connections — the distinctive lines only | 5.7 | 1.9 | 0.641 |
| walker |  | 8545 | 164 | Code::CodeKey { rung: Body, file: peepdb/db/postgresql.py, decl: 2, sub: 0, line: 7 } |  |  | 0.649 |
| walker |  | 8713 | 168 | Code::CodeKey { rung: Body, file: peepdb/db/mariadb.py, decl: 2, sub: 0, line: 6 } |  |  | 0.663 |
| ns | 8760 |  | 357 | Every test (and fixture) name in peepdb/tests, across all six modules | 6.1 | 1.11 | 0.649 |
| walker |  | 8764 | 51 | Code::CodeKey { rung: Body, file: peepdb/cli.py, decl: 6, sub: 0, line: 97 } |  |  | 0.658 |
| walker |  | 8818 | 54 | Code::CodeKey { rung: Body, file: peepdb/cli.py, decl: 2, sub: 0, line: 17 } |  |  | 0.658 |
| ns | 8866 |  | 106 | How the suite is run: CONTRIBUTING instructions and the CI invocation | 6.2 |  | 0.655 |
| walker |  | 8948 | 130 | Code::CodeKey { rung: Body, file: peepdb/config.py, decl: 8, sub: 0, line: 29 } |  |  | 0.667 |
| ns | 9081 |  | 215 | A representative test body: patching style and asserted output strings | 6.3 | 6.1 | 0.660 |
| ns | 9171 |  | 90 | `peepdb --help` summary lines from the cli group docstring | 7.1 | 1.6 | 0.662 |
| walker |  | 9182 | 234 | Code::CodeKey { rung: Body, file: peepdb/db/mssql.py, decl: 3, sub: 0, line: 12 } |  |  | 0.678 |
| walker |  | 9415 | 233 | Code::CodeKey { rung: Body, file: peepdb/db/oracle.py, decl: 5, sub: 0, line: 36 } |  |  | 0.680 |
| ns | 9427 |  | 256 | The two disagreeing dependency lists (setup.py vs project.toml) | 7.2 |  | 0.674 |
| ns | 9564 |  | 137 | Build backend, Python floor, packaging includes | 7.3 | 1.1 | 0.668 |
| walker |  | 9662 | 247 | Code::CodeKey { rung: Body, file: peepdb/core.py, decl: 6, sub: 0, line: 118 } |  |  | 0.689 |
| ns | 9752 |  | 188 | CI: what triggers the workflows and on what Python versions | 7.4 |  | 0.680 |
| walker |  | 9796 | 134 | Code::CodeKey { rung: Body, file: peepdb/config.py, decl: 17, sub: 0, line: 161 } |  |  | 0.680 |
| ns | 9913 |  | 161 | The docs/ site: Jekyll theme config, and two stale-template markers | 7.5 | 1.11 | 0.678 |
| ns | 9978 |  | 65 | CustomEncoder.default — the JSON serializer for Decimal and date | 7.6 | 1.6 | 0.679 |

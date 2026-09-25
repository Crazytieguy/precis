Score(3000)=0.724 I=0.896 C=0.585 ns_rows≤3K=21/54 grid(1000/1442/2080/3000/4327/6240/9000)=0.708/0.712/0.808/0.724/0.656/0.622/0.666

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 38 | 38 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 41 | 3 | Fs::DirListing { dir: .github } |  |  | 0.000 |
| walker |  | 49 | 8 | Fs::DirListing { dir: .github/workflows } |  |  | 0.000 |
| ns | 54 |  | 54 | Identity: README title + package name/version/description | 1.1 |  | 0.000 |
| walker |  | 77 | 28 | Fs::DirListing { dir: images } |  |  | 0.000 |
| ns | 92 |  | 38 | Complete repository root listing | 1.2 |  | 0.539 |
| walker |  | 111 | 34 | Fs::DirListing { dir: peepdb } |  |  | 0.573 |
| walker |  | 152 | 41 | Fs::DirListing { dir: docs } |  |  | 0.591 |
| ns | 173 |  | 81 | README lede paragraph — scope and supported databases | 1.3 | 1.1 | 0.568 |
| walker |  | 197 | 45 | Fs::DirListing { dir: peepdb/db } |  |  | 0.658 |
| ns | 252 |  | 79 | Source roster: complete listing of peepdb/ and peepdb/db/ | 1.4 |  | 0.661 |
| walker |  | 273 | 76 | Toml::Identity { file: project.toml } |  |  | 0.837 |
| ns | 297 |  | 45 | peepdb/core.py: every top-level function, names only | 1.5 |  | 0.783 |
| walker |  | 300 | 27 | Toml::Operational { file: project.toml } |  |  | 0.783 |
| ns | 378 |  | 81 | peepdb/cli.py: every top-level definition, names only | 1.6 |  | 0.699 |
| walker |  | 397 | 97 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.846 |
| ns | 497 |  | 119 | peepdb/db/__init__.py — backend class ↔ module map | 1.7 |  | 0.744 |
| walker |  | 590 | 193 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.754 |
| ns | 676 |  | 179 | README feature bullets | 1.8 |  | 0.691 |
| walker |  | 758 | 168 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.788 |
| walker |  | 793 | 35 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.788 |
| walker |  | 821 | 28 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.788 |
| ns | 831 |  | 155 | peepdb/config.py: every top-level definition, names only | 1.9 |  | 0.708 |
| walker |  | 867 | 46 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.708 |
| ns | 908 |  | 77 | Entry points: console script, __main__.py, exceptions.py in full | 1.10 |  | 0.673 |
| walker |  | 923 | 56 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.673 |
| ns | 1027 |  | 119 | Complete listings of peepdb/tests, docs, images, .github/workflows | 1.11 |  | 0.655 |
| walker |  | 1138 | 215 | Code::CodeKey { rung: Names, file: peepdb/db/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.733 |
| ns | 1209 |  | 182 | README section-heading map (every H2 and H3 after the feature list) | 1.12 | 1.1 | 0.752 |
| walker |  | 1330 | 192 | Toml::Dependencies { file: project.toml } |  |  | 0.752 |
| ns | 1441 |  | 232 | `peepdb save` full option decorator block (cli.py 53-64) | 2.1 | 1.6 | 0.712 |
| walker |  | 1456 | 126 | Plaintext::DeclSurface { file: docs/Gemfile } |  |  | 0.712 |
| walker |  | 1498 | 42 | Fs::DirListing { dir: peepdb/tests } |  |  | 0.773 |
| walker |  | 1509 | 11 | Code::CodeKey { rung: Names, file: peepdb/exceptions.py, decl: 0, sub: 0, line: 0 } |  |  | 0.778 |
| walker |  | 1514 | 5 | Code::CodeKey { rung: Decl, file: peepdb/exceptions.py, decl: 1, sub: 0, line: 1 } |  |  | 0.782 |
| ns | 1583 |  | 142 | `peepdb view` full option decorator block (cli.py 126-132) | 2.2 | 1.6 | 0.759 |
| walker |  | 1625 | 111 | Code::CodeKey { rung: Names, file: peepdb/cli.py, decl: 0, sub: 0, line: 0 } |  |  | 0.805 |
| walker |  | 1633 | 8 | Code::CodeKey { rung: Decl, file: peepdb/cli.py, decl: 5, sub: 0, line: 86 } |  |  | 0.805 |
| walker |  | 1644 | 11 | Code::CodeKey { rung: Decl, file: peepdb/cli.py, decl: 1, sub: 0, line: 16 } |  |  | 0.805 |
| walker |  | 1661 | 17 | Code::CodeKey { rung: Decl, file: peepdb/cli.py, decl: 3, sub: 0, line: 25 } |  |  | 0.806 |
| ns | 1680 |  | 97 | Remaining Click decorators: group, version option, confirmations | 2.3 | 1.6 | 0.782 |
| walker |  | 1692 | 31 | Code::CodeKey { rung: Decl, file: peepdb/cli.py, decl: 7, sub: 0, line: 113 } |  |  | 0.792 |
| walker |  | 1733 | 41 | Code::CodeKey { rung: Decl, file: peepdb/cli.py, decl: 6, sub: 0, line: 97 } |  |  | 0.814 |
| walker |  | 1875 | 142 | Code::CodeKey { rung: Decl, file: peepdb/cli.py, decl: 8, sub: 0, line: 126 } |  |  | 0.844 |
| ns | 1922 |  | 242 | `view` docstring + connection resolution (cli.py 134-155) | 2.4 | 1.6 | 0.779 |
| walker |  | 2107 | 232 | Code::CodeKey { rung: Decl, file: peepdb/cli.py, decl: 4, sub: 0, line: 53 } |  |  | 0.825 |
| ns | 2207 |  | 285 | `view` dispatch into peep_db + output rendering (cli.py 156-178) | 2.5 | 2.4 | 0.768 |
| walker |  | 2379 | 272 | Code::CodeKey { rung: Names, file: peepdb/config.py, decl: 0, sub: 0, line: 0 } |  |  | 0.819 |
| walker |  | 2397 | 18 | Code::CodeKey { rung: Decl, file: peepdb/config.py, decl: 8, sub: 0, line: 29 } |  |  | 0.819 |
| walker |  | 2417 | 20 | Code::CodeKey { rung: Decl, file: peepdb/config.py, decl: 9, sub: 0, line: 41 } |  |  | 0.819 |
| ns | 2425 |  | 218 | `save` docstring and body (cli.py 66-83) | 2.6 | 1.6 | 0.778 |
| walker |  | 2447 | 30 | Code::CodeKey { rung: Decl, file: peepdb/config.py, decl: 1, sub: 0, line: 15 } |  |  | 0.779 |
| walker |  | 2462 | 15 | Code::CodeKey { rung: Body, file: peepdb/config.py, decl: 12, sub: 0, line: 70 } |  |  | 0.779 |
| walker |  | 2476 | 14 | Code::CodeKey { rung: Names, file: peepdb/db/base.py, decl: 0, sub: 0, line: 0 } |  |  | 0.779 |
| ns | 2515 |  | 90 | Bodies of `list`, `remove` and `remove-all` (cli.py) | 2.7 | 1.6 | 0.758 |
| walker |  | 2628 | 152 | Code::CodeKey { rung: Decl, file: peepdb/db/base.py, decl: 1, sub: 0, line: 5 } |  |  | 0.760 |
| walker |  | 2637 | 9 | Code::CodeKey { rung: Decl, file: peepdb/db/base.py, decl: 3, sub: 0, line: 17 } |  |  | 0.760 |
| walker |  | 2646 | 9 | Code::CodeKey { rung: Decl, file: peepdb/db/base.py, decl: 4, sub: 0, line: 21 } |  |  | 0.760 |
| walker |  | 2655 | 9 | Code::CodeKey { rung: Decl, file: peepdb/db/base.py, decl: 5, sub: 0, line: 25 } |  |  | 0.761 |
| walker |  | 2664 | 9 | Code::CodeKey { rung: Decl, file: peepdb/db/base.py, decl: 6, sub: 0, line: 29 } |  |  | 0.761 |
| walker |  | 2669 | 5 | Code::CodeKey { rung: Body, file: peepdb/db/base.py, decl: 3, sub: 0, line: 17 } |  |  | 0.761 |
| walker |  | 2674 | 5 | Code::CodeKey { rung: Body, file: peepdb/db/base.py, decl: 4, sub: 0, line: 21 } |  |  | 0.762 |
| walker |  | 2688 | 14 | Code::CodeKey { rung: Names, file: peepdb/db/oracle.py, decl: 0, sub: 0, line: 0 } |  |  | 0.762 |
| walker |  | 2741 | 53 | Code::CodeKey { rung: Decl, file: peepdb/db/oracle.py, decl: 1, sub: 0, line: 7 } |  |  | 0.762 |
| ns | 2746 |  | 231 | BaseDatabase: class line + the four abstract methods + context manager | 3.1 |  | 0.746 |
| walker |  | 2779 | 38 | Code::CodeKey { rung: Decl, file: peepdb/db/oracle.py, decl: 5, sub: 0, line: 36 } |  |  | 0.746 |
| walker |  | 2793 | 14 | Code::CodeKey { rung: Names, file: peepdb/db/sqlite.py, decl: 0, sub: 0, line: 0 } |  |  | 0.746 |
| walker |  | 2872 | 79 | Code::CodeKey { rung: Decl, file: peepdb/db/sqlite.py, decl: 1, sub: 0, line: 6 } |  |  | 0.746 |
| walker |  | 2887 | 15 | Code::CodeKey { rung: Names, file: peepdb/db/mariadb.py, decl: 0, sub: 0, line: 0 } |  |  | 0.746 |
| ns | 2913 |  | 167 | BaseDatabase.__init__ and base.py imports | 3.2 | 3.1 | 0.723 |
| walker |  | 2966 | 79 | Code::CodeKey { rung: Decl, file: peepdb/db/mariadb.py, decl: 1, sub: 0, line: 5 } |  |  | 0.724 |
| walker |  | 2981 | 15 | Code::CodeKey { rung: Names, file: peepdb/db/mongodb.py, decl: 0, sub: 0, line: 0 } |  |  | 0.724 |
| ns | 3026 |  | 113 | core.peep_db full signature (core.py 56-66) | 3.3 | 1.5 | 0.707 |
| walker |  | 3035 | 54 | Code::CodeKey { rung: Decl, file: peepdb/db/mongodb.py, decl: 1, sub: 0, line: 8 } |  |  | 0.707 |
| walker |  | 3090 | 55 | Code::CodeKey { rung: Decl, file: peepdb/db/mongodb.py, decl: 5, sub: 0, line: 42 } |  |  | 0.707 |
| walker |  | 3100 | 10 | Code::CodeKey { rung: Body, file: peepdb/db/mongodb.py, decl: 4, sub: 0, line: 39 } |  |  | 0.707 |
| walker |  | 3115 | 15 | Code::CodeKey { rung: Names, file: peepdb/db/mssql.py, decl: 0, sub: 0, line: 0 } |  |  | 0.708 |
| walker |  | 3240 | 125 | Code::CodeKey { rung: Decl, file: peepdb/db/mssql.py, decl: 1, sub: 0, line: 6 } |  |  | 0.709 |
| walker |  | 3255 | 15 | Code::CodeKey { rung: Names, file: peepdb/db/mysql.py, decl: 0, sub: 0, line: 0 } |  |  | 0.709 |
| ns | 3292 |  | 266 | connect_to_database: the first five engine branches (core.py 28-42) | 3.4 | 1.5 | 0.686 |
| walker |  | 3334 | 79 | Code::CodeKey { rung: Decl, file: peepdb/db/mysql.py, decl: 1, sub: 0, line: 5 } |  |  | 0.687 |
| walker |  | 3349 | 15 | Code::CodeKey { rung: Names, file: peepdb/db/postgresql.py, decl: 0, sub: 0, line: 0 } |  |  | 0.687 |
| walker |  | 3428 | 79 | Code::CodeKey { rung: Decl, file: peepdb/db/postgresql.py, decl: 1, sub: 0, line: 6 } |  |  | 0.688 |
| walker |  | 3433 | 5 | Code::CodeKey { rung: Body, file: peepdb/db/base.py, decl: 5, sub: 0, line: 25 } |  |  | 0.691 |
| walker |  | 3448 | 15 | Code::CodeKey { rung: Body, file: peepdb/config.py, decl: 13, sub: 0, line: 73 } |  |  | 0.691 |
| ns | 3453 |  | 161 | connect_to_database: the irregular branches and the failure path (core.py 43-53) | 3.5 | 3.4 | 0.676 |
| walker |  | 3479 | 31 | Code::CodeKey { rung: Names, file: peepdb/db/firebase.py, decl: 0, sub: 0, line: 0 } |  |  | 0.676 |
| walker |  | 3575 | 96 | Code::CodeKey { rung: Decl, file: peepdb/db/firebase.py, decl: 2, sub: 0, line: 9 } |  |  | 0.678 |
| walker |  | 3596 | 21 | Code::CodeKey { rung: Body, file: peepdb/db/firebase.py, decl: 5, sub: 0, line: 26 } |  |  | 0.678 |
| ns | 3651 |  | 198 | peep_db body: connect, fetch one table or all (core.py 67-79) | 3.6 | 3.3 | 0.661 |
| walker |  | 3714 | 118 | Code::CodeKey { rung: Names, file: peepdb/core.py, decl: 0, sub: 0, line: 0 } |  |  | 0.676 |
| walker |  | 3823 | 109 | Code::CodeKey { rung: Decl, file: peepdb/core.py, decl: 4, sub: 0, line: 56 } |  |  | 0.702 |
| ns | 3887 |  | 236 | peep_db body: table-vs-JSON post-processing and the finally-disconnect (core.py 80-98) | 3.7 | 3.6 | 0.678 |
| walker |  | 3949 | 126 | Plaintext::DeclSurface { file: requirements.txt } |  |  | 0.678 |
| walker |  | 3956 | 7 | Code::CodeKey { rung: Body, file: peepdb/cli.py, decl: 3, sub: 0, line: 25 } |  |  | 0.678 |
| walker |  | 3961 | 5 | Code::CodeKey { rung: Body, file: peepdb/db/base.py, decl: 6, sub: 0, line: 29 } |  |  | 0.681 |
| walker |  | 3987 | 26 | Code::CodeKey { rung: Doc, file: peepdb/db/mongodb.py, decl: 5, sub: 0, line: 42 } |  |  | 0.681 |
| walker |  | 3994 | 7 | Code::CodeKey { rung: Body, file: peepdb/cli.py, decl: 9, sub: 0, line: 181 } |  |  | 0.681 |
| walker |  | 4025 | 31 | Code::CodeKey { rung: Body, file: peepdb/db/mariadb.py, decl: 4, sub: 0, line: 30 } |  |  | 0.681 |
| ns | 4134 |  | 247 | format_value: numeric/date rendering rules (core.py 119-140) | 3.8 | 1.5 | 0.656 |
| walker |  | 4257 | 232 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.656 |
| ns | 4335 |  | 201 | format_as_table: the tabulate grid layout (core.py 102-115) | 3.9 | 1.5 | 0.642 |
| walker |  | 4458 | 201 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.642 |
| ns | 4459 |  | 124 | core.py import header and module logger setup | 3.10 |  | 0.629 |
| walker |  | 4490 | 32 | Code::CodeKey { rung: Body, file: peepdb/db/oracle.py, decl: 4, sub: 0, line: 32 } |  |  | 0.629 |
| walker |  | 4523 | 33 | Code::CodeKey { rung: Body, file: peepdb/db/firebase.py, decl: 3, sub: 0, line: 10 } |  |  | 0.629 |
| walker |  | 4557 | 34 | Code::CodeKey { rung: Body, file: peepdb/db/mysql.py, decl: 4, sub: 0, line: 29 } |  |  | 0.629 |
| walker |  | 4600 | 43 | Code::CodeKey { rung: Doc, file: peepdb/cli.py, decl: 5, sub: 0, line: 86 } |  |  | 0.629 |
| walker |  | 4606 | 6 | Code::CodeKey { rung: Body, file: peepdb/db/base.py, decl: 8, sub: 0, line: 37 } |  |  | 0.633 |
| walker |  | 4651 | 45 | Code::CodeKey { rung: Doc, file: peepdb/cli.py, decl: 7, sub: 0, line: 113 } |  |  | 0.633 |
| walker |  | 4691 | 40 | Code::CodeKey { rung: Body, file: peepdb/db/postgresql.py, decl: 4, sub: 0, line: 30 } |  |  | 0.633 |
| walker |  | 4738 | 47 | Code::CodeKey { rung: Doc, file: peepdb/cli.py, decl: 6, sub: 0, line: 97 } |  |  | 0.633 |
| walker |  | 4783 | 45 | Code::CodeKey { rung: Body, file: peepdb/db/mssql.py, decl: 2, sub: 0, line: 7 } |  |  | 0.633 |
| walker |  | 4835 | 52 | Code::CodeKey { rung: Doc, file: peepdb/cli.py, decl: 8, sub: 0, line: 126 } |  |  | 0.635 |
| walker |  | 4887 | 52 | Code::CodeKey { rung: Body, file: peepdb/db/sqlite.py, decl: 3, sub: 0, line: 22 } |  |  | 0.635 |
| walker |  | 4940 | 53 | Code::CodeKey { rung: Body, file: peepdb/db/oracle.py, decl: 3, sub: 0, line: 25 } |  |  | 0.635 |
| ns | 4964 |  | 505 | Every class and method name across all eight backend modules | 4.1 | 3.1 | 0.667 |
| walker |  | 4994 | 54 | Code::CodeKey { rung: Body, file: peepdb/db/mariadb.py, decl: 3, sub: 0, line: 23 } |  |  | 0.667 |
| walker |  | 5048 | 54 | Code::CodeKey { rung: Body, file: peepdb/db/mssql.py, decl: 4, sub: 0, line: 28 } |  |  | 0.667 |
| walker |  | 5102 | 54 | Code::CodeKey { rung: Body, file: peepdb/db/mysql.py, decl: 3, sub: 0, line: 22 } |  |  | 0.667 |
| walker |  | 5156 | 54 | Code::CodeKey { rung: Body, file: peepdb/db/postgresql.py, decl: 3, sub: 0, line: 23 } |  |  | 0.667 |
| walker |  | 5222 | 66 | Code::CodeKey { rung: Doc, file: peepdb/cli.py, decl: 4, sub: 0, line: 53 } |  |  | 0.669 |
| ns | 5263 |  | 299 | The table/collection-listing query of every backend | 4.2 | 4.1 | 0.656 |
| walker |  | 5270 | 48 | Code::CodeKey { rung: Body, file: peepdb/db/mongodb.py, decl: 3, sub: 0, line: 32 } |  |  | 0.656 |
| walker |  | 5351 | 81 | Code::CodeKey { rung: Body, file: peepdb/db/firebase.py, decl: 6, sub: 0, line: 30 } |  |  | 0.664 |
| walker |  | 5357 | 6 | Code::CodeKey { rung: Body, file: peepdb/cli.py, decl: 5, sub: 0, line: 86 } |  |  | 0.665 |
| walker |  | 5460 | 103 | Code::CodeKey { rung: Body, file: peepdb/db/mssql.py, decl: 5, sub: 0, line: 35 } |  |  | 0.675 |
| walker |  | 5474 | 14 | Code::CodeKey { rung: Body, file: peepdb/db/base.py, decl: 7, sub: 0, line: 33 } |  |  | 0.681 |
| ns | 5595 |  | 332 | MySQLDatabase — the canonical backend implementation | 4.3 | 4.1 | 0.657 |
| ns | 5897 |  | 302 | PostgreSQL, MariaDB and Oracle connect() — drivers, default ports, cursor factories | 4.4 | 4.1 | 0.635 |
| walker |  | 6128 | 654 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.635 |
| ns | 6156 |  | 259 | MongoDBDatabase.connect — URI construction from the generic parameters | 4.5 | 4.1 | 0.620 |
| walker |  | 6205 | 77 | Code::CodeKey { rung: Body, file: peepdb/config.py, decl: 9, sub: 0, line: 41 } |  |  | 0.621 |
| walker |  | 6359 | 154 | Code::CodeKey { rung: Body, file: peepdb/db/sqlite.py, decl: 4, sub: 0, line: 29 } |  |  | 0.630 |
| ns | 6435 |  | 279 | MSSQLDatabase: constructor extras and the ODBC connection string | 4.6 | 4.1 | 0.619 |
| walker |  | 6514 | 155 | Code::CodeKey { rung: Body, file: peepdb/db/mysql.py, decl: 2, sub: 0, line: 6 } |  |  | 0.630 |
| ns | 6582 |  | 147 | FirebaseDatabase: the backend that does not call super().__init__ | 4.7 | 4.1 | 0.623 |
| walker |  | 6676 | 162 | Code::CodeKey { rung: Body, file: peepdb/db/mariadb.py, decl: 5, sub: 0, line: 34 } |  |  | 0.623 |
| walker |  | 6838 | 162 | Code::CodeKey { rung: Body, file: peepdb/db/postgresql.py, decl: 5, sub: 0, line: 34 } |  |  | 0.623 |
| ns | 6899 |  | 317 | SQLiteDatabase connect and fetch_data — file-path handling and print-based logging | 4.8 | 4.1 | 0.609 |
| walker |  | 6952 | 114 | Code::CodeKey { rung: Body, file: peepdb/db/firebase.py, decl: 4, sub: 0, line: 15 } |  |  | 0.624 |
| walker |  | 7153 | 201 | Code::CodeKey { rung: Body, file: peepdb/core.py, decl: 5, sub: 0, line: 101 } |  |  | 0.642 |
| ns | 7201 |  | 302 | The non-LIMIT/OFFSET pagination variants (MSSQL, Oracle, MongoDB, Firebase) | 4.9 | 4.1 | 0.629 |
| ns | 7310 |  | 109 | Where credentials live: config paths and KeySecurity modes | 5.1 | 1.9 | 0.632 |
| walker |  | 7331 | 178 | Code::CodeKey { rung: Body, file: peepdb/db/sqlite.py, decl: 2, sub: 0, line: 7 } |  |  | 0.641 |
| ns | 7495 |  | 185 | get_key_security_config / get_key / encrypt / decrypt bodies | 5.2 | 1.9 | 0.633 |
| walker |  | 7578 | 247 | Code::CodeKey { rung: Body, file: peepdb/core.py, decl: 6, sub: 0, line: 118 } |  |  | 0.660 |
| walker |  | 7739 | 161 | Code::CodeKey { rung: Body, file: peepdb/db/oracle.py, decl: 2, sub: 0, line: 8 } |  |  | 0.662 |
| ns | 7742 |  | 247 | The two key sources: PBKDF2 derivation and keyring fetch-or-create | 5.3 | 1.9 | 0.656 |
| ns | 7987 |  | 245 | save_connection: the on-disk record shape and what is encrypted | 5.4 | 1.9 | 0.642 |
| walker |  | 8016 | 277 | Code::CodeKey { rung: Doc, file: peepdb/cli.py, decl: 3, sub: 0, line: 25 } |  |  | 0.643 |
| ns | 8167 |  | 180 | get_connection: the tuple contract and InvalidPassword | 5.5 | 1.9 | 0.633 |
| walker |  | 8178 | 162 | Code::CodeKey { rung: Body, file: peepdb/db/mysql.py, decl: 5, sub: 0, line: 33 } |  |  | 0.652 |
| ns | 8309 |  | 142 | add_key_security: first-run choice between keyring and password | 5.6 | 1.9 | 0.646 |
| walker |  | 8342 | 164 | Code::CodeKey { rung: Body, file: peepdb/db/postgresql.py, decl: 2, sub: 0, line: 7 } |  |  | 0.655 |
| ns | 8403 |  | 94 | list/remove/remove_all_connections — the distinctive lines only | 5.7 | 1.9 | 0.649 |
| walker |  | 8510 | 168 | Code::CodeKey { rung: Body, file: peepdb/db/mariadb.py, decl: 2, sub: 0, line: 6 } |  |  | 0.663 |
| walker |  | 8615 | 105 | Code::CodeKey { rung: Body, file: peepdb/config.py, decl: 11, sub: 0, line: 60 } |  |  | 0.670 |
| ns | 8760 |  | 357 | Every test (and fixture) name in peepdb/tests, across all six modules | 6.1 | 1.11 | 0.656 |
| walker |  | 8776 | 161 | Code::CodeKey { rung: Body, file: peepdb/db/mongodb.py, decl: 5, sub: 0, line: 42 } |  |  | 0.658 |
| ns | 8866 |  | 106 | How the suite is run: CONTRIBUTING instructions and the CI invocation | 6.2 |  | 0.653 |
| walker |  | 9010 | 234 | Code::CodeKey { rung: Body, file: peepdb/db/mssql.py, decl: 3, sub: 0, line: 12 } |  |  | 0.670 |
| ns | 9081 |  | 215 | A representative test body: patching style and asserted output strings | 6.3 | 6.1 | 0.662 |
| walker |  | 9118 | 108 | Code::CodeKey { rung: Body, file: peepdb/config.py, decl: 10, sub: 0, line: 49 } |  |  | 0.669 |
| ns | 9171 |  | 90 | `peepdb --help` summary lines from the cli group docstring | 7.1 | 1.6 | 0.671 |
| ns | 9427 |  | 256 | The two disagreeing dependency lists (setup.py vs project.toml) | 7.2 |  | 0.665 |
| walker |  | 9545 | 427 | Code::CodeKey { rung: Body, file: peepdb/core.py, decl: 3, sub: 0, line: 27 } |  |  | 0.692 |
| ns | 9564 |  | 137 | Build backend, Python floor, packaging includes | 7.3 | 1.1 | 0.686 |
| ns | 9752 |  | 188 | CI: what triggers the workflows and on what Python versions | 7.4 |  | 0.677 |
| walker |  | 9830 | 285 | Code::CodeKey { rung: Body, file: peepdb/db/sqlite.py, decl: 5, sub: 0, line: 43 } |  |  | 0.690 |
| ns | 9913 |  | 161 | The docs/ site: Jekyll theme config, and two stale-template markers | 7.5 | 1.11 | 0.687 |
| ns | 9978 |  | 65 | CustomEncoder.default — the JSON serializer for Decimal and date | 7.6 | 1.6 | 0.685 |
| walker |  | 9996 | 166 | Code::CodeKey { rung: Body, file: peepdb/db/oracle.py, decl: 5, sub: 0, line: 36 } |  |  | 0.687 |

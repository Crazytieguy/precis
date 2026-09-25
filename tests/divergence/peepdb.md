Score(3000)=0.610 I=0.861 C=0.432 ns_rows≤3K=21/54 grid(1000/1442/2080/3000/4327/6240/9000)=0.699/0.652/0.686/0.610/0.629/0.595/0.573

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 38 | 38 | listing of '.' |  |  | 0.000 |
| walker |  | 41 | 3 | listing of '.github' |  |  | 0.000 |
| walker |  | 49 | 8 | listing of '.github/workflows' |  |  | 0.000 |
| ns | 54 |  | 54 | Identity: README title + package name/version/description | 1.1 |  | 0.000 |
| walker |  | 77 | 28 | listing of 'images' |  |  | 0.000 |
| ns | 92 |  | 38 | Complete repository root listing | 1.2 |  | 0.539 |
| walker |  | 111 | 34 | listing of 'peepdb' |  |  | 0.573 |
| walker |  | 122 | 11 | python names peepdb/exceptions.py |  |  | 0.573 |
| walker |  | 127 | 5 | python decl peepdb/exceptions.py:1 |  |  | 0.574 |
| walker |  | 168 | 41 | listing of 'docs' |  |  | 0.592 |
| ns | 173 |  | 81 | README lede paragraph — scope and supported databases | 1.3 | 1.1 | 0.569 |
| walker |  | 213 | 45 | listing of 'peepdb/db' |  |  | 0.659 |
| ns | 252 |  | 79 | Source roster: complete listing of peepdb/ and peepdb/db/ | 1.4 |  | 0.662 |
| walker |  | 289 | 76 | [package] in project.toml |  |  | 0.838 |
| ns | 297 |  | 45 | peepdb/core.py: every top-level function, names only | 1.5 |  | 0.783 |
| walker |  | 303 | 14 | python names peepdb/db/base.py |  |  | 0.783 |
| walker |  | 317 | 14 | python names peepdb/db/oracle.py |  |  | 0.783 |
| walker |  | 331 | 14 | python names peepdb/db/sqlite.py |  |  | 0.783 |
| walker |  | 358 | 27 | [features] / entry-point scripts in project.toml |  |  | 0.786 |
| ns | 378 |  | 81 | peepdb/cli.py: every top-level definition, names only | 1.6 |  | 0.701 |
| walker |  | 455 | 97 | README headline in README.md |  |  | 0.848 |
| walker |  | 470 | 15 | python names peepdb/db/mariadb.py |  |  | 0.848 |
| walker |  | 485 | 15 | python names peepdb/db/mongodb.py |  |  | 0.848 |
| ns | 497 |  | 119 | peepdb/db/__init__.py — backend class ↔ module map | 1.7 |  | 0.746 |
| walker |  | 500 | 15 | python names peepdb/db/mssql.py |  |  | 0.746 |
| walker |  | 515 | 15 | python names peepdb/db/mysql.py |  |  | 0.746 |
| walker |  | 530 | 15 | python names peepdb/db/postgresql.py |  |  | 0.746 |
| ns | 676 |  | 179 | README feature bullets | 1.8 |  | 0.684 |
| walker |  | 745 | 215 | python names peepdb/db/__init__.py |  |  | 0.803 |
| walker |  | 776 | 31 | python names peepdb/db/firebase.py |  |  | 0.803 |
| ns | 831 |  | 155 | peepdb/config.py: every top-level definition, names only | 1.9 |  | 0.722 |
| walker |  | 832 | 56 | package metadata in project.toml |  |  | 0.722 |
| ns | 908 |  | 77 | Entry points: console script, __main__.py, exceptions.py in full | 1.10 |  | 0.699 |
| walker |  | 1024 | 192 | [dependencies] in project.toml |  |  | 0.699 |
| ns | 1027 |  | 119 | Complete listings of peepdb/tests, docs, images, .github/workflows | 1.11 |  | 0.675 |
| walker |  | 1077 | 53 | python decl peepdb/db/oracle.py:7 |  |  | 0.676 |
| walker |  | 1131 | 54 | python decl peepdb/db/mongodb.py:8 |  |  | 0.676 |
| ns | 1209 |  | 182 | README section-heading map (every H2 and H3 after the feature list) | 1.12 | 1.1 | 0.630 |
| walker |  | 1242 | 111 | python names peepdb/cli.py |  |  | 0.685 |
| walker |  | 1250 | 8 | python decl peepdb/cli.py:86 |  |  | 0.685 |
| walker |  | 1261 | 11 | python decl peepdb/cli.py:16 |  |  | 0.685 |
| walker |  | 1278 | 17 | python decl peepdb/cli.py:25 |  |  | 0.685 |
| walker |  | 1309 | 31 | python decl peepdb/cli.py:113 |  |  | 0.686 |
| walker |  | 1350 | 41 | python decl peepdb/cli.py:97 |  |  | 0.689 |
| walker |  | 1357 | 7 | python body peepdb/cli.py:181 |  |  | 0.689 |
| ns | 1441 |  | 232 | `peepdb save` full option decorator block (cli.py 53-64) | 2.1 | 1.6 | 0.652 |
| walker |  | 1475 | 118 | python names peepdb/core.py |  |  | 0.679 |
| ns | 1583 |  | 142 | `peepdb view` full option decorator block (cli.py 126-132) | 2.2 | 1.6 | 0.659 |
| walker |  | 1584 | 109 | python decl peepdb/core.py:56 |  |  | 0.662 |
| ns | 1680 |  | 97 | Remaining Click decorators: group, version option, confirmations | 2.3 | 1.6 | 0.676 |
| walker |  | 1777 | 193 | headings outline in README.md |  |  | 0.742 |
| walker |  | 1812 | 35 | README.md section #2 |  |  | 0.742 |
| walker |  | 1840 | 28 | README.md section #11 |  |  | 0.742 |
| walker |  | 1870 | 30 | README.md section #10 |  |  | 0.742 |
| ns | 1922 |  | 242 | `view` docstring + connection resolution (cli.py 134-155) | 2.4 | 1.6 | 0.686 |
| walker |  | 1966 | 96 | README headline in docs/README.md |  |  | 0.686 |
| walker |  | 2004 | 38 | python decl peepdb/db/oracle.py:36 |  |  | 0.686 |
| walker |  | 2039 | 35 | README.md section #9 |  |  | 0.686 |
| walker |  | 2047 | 8 | python body peepdb/cli.py:86 |  |  | 0.686 |
| walker |  | 2189 | 142 | python decl peepdb/cli.py:126 |  |  | 0.714 |
| ns | 2207 |  | 285 | `view` dispatch into peep_db + output rendering (cli.py 156-178) | 2.5 | 2.4 | 0.664 |
| ns | 2425 |  | 218 | `save` docstring and body (cli.py 66-83) | 2.6 | 1.6 | 0.631 |
| walker |  | 2461 | 272 | python names peepdb/config.py |  |  | 0.683 |
| walker |  | 2479 | 18 | python decl peepdb/config.py:29 |  |  | 0.683 |
| walker |  | 2499 | 20 | python decl peepdb/config.py:41 |  |  | 0.683 |
| ns | 2515 |  | 90 | Bodies of `list`, `remove` and `remove-all` (cli.py) | 2.7 | 1.6 | 0.665 |
| walker |  | 2529 | 30 | python decl peepdb/config.py:15 |  |  | 0.665 |
| walker |  | 2544 | 15 | python body peepdb/config.py:70 |  |  | 0.665 |
| walker |  | 2559 | 15 | python body peepdb/config.py:73 |  |  | 0.665 |
| walker |  | 2638 | 79 | python decl peepdb/db/mariadb.py:5 |  |  | 0.666 |
| walker |  | 2717 | 79 | python decl peepdb/db/mysql.py:5 |  |  | 0.667 |
| ns | 2746 |  | 231 | BaseDatabase: class line + the four abstract methods + context manager | 3.1 |  | 0.628 |
| walker |  | 2796 | 79 | python decl peepdb/db/postgresql.py:6 |  |  | 0.629 |
| walker |  | 2875 | 79 | python decl peepdb/db/sqlite.py:6 |  |  | 0.630 |
| ns | 2913 |  | 167 | BaseDatabase.__init__ and base.py imports | 3.2 | 3.1 | 0.610 |
| walker |  | 2921 | 46 | README.md section #4 |  |  | 0.610 |
| walker |  | 2975 | 54 | headings outline in docs/installation.md |  |  | 0.610 |
| ns | 3026 |  | 113 | core.peep_db full signature (core.py 56-66) | 3.3 | 1.5 | 0.624 |
| walker |  | 3102 | 127 | manifest config in project.toml |  |  | 0.624 |
| walker |  | 3158 | 56 | headings outline in docs/index.md |  |  | 0.624 |
| walker |  | 3214 | 56 | README.md section #3 |  |  | 0.624 |
| ns | 3292 |  | 266 | connect_to_database: the first five engine branches (core.py 28-42) | 3.4 | 1.5 | 0.604 |
| walker |  | 3310 | 96 | python decl peepdb/db/firebase.py:9 |  |  | 0.605 |
| walker |  | 3365 | 55 | python decl peepdb/db/mongodb.py:42 |  |  | 0.605 |
| walker |  | 3375 | 10 | python body peepdb/db/mongodb.py:39 |  |  | 0.605 |
| walker |  | 3417 | 42 | listing of 'peepdb/tests' |  |  | 0.641 |
| ns | 3453 |  | 161 | connect_to_database: the irregular branches and the failure path (core.py 43-53) | 3.5 | 3.4 | 0.627 |
| walker |  | 3649 | 232 | python decl peepdb/cli.py:53 |  |  | 0.659 |
| ns | 3651 |  | 198 | peep_db body: connect, fetch one table or all (core.py 67-79) | 3.6 | 3.3 | 0.643 |
| walker |  | 3775 | 126 | declaration surface of docs/Gemfile |  |  | 0.644 |
| ns | 3887 |  | 236 | peep_db body: table-vs-JSON post-processing and the finally-disconnect (core.py 80-98) | 3.7 | 3.6 | 0.622 |
| walker |  | 3900 | 125 | python decl peepdb/db/mssql.py:6 |  |  | 0.624 |
| walker |  | 3907 | 7 | python body peepdb/cli.py:25 |  |  | 0.624 |
| walker |  | 4059 | 152 | python decl peepdb/db/base.py:5 |  |  | 0.635 |
| walker |  | 4068 | 9 | python decl peepdb/db/base.py:17 |  |  | 0.637 |
| walker |  | 4077 | 9 | python decl peepdb/db/base.py:21 |  |  | 0.640 |
| walker |  | 4086 | 9 | python decl peepdb/db/base.py:25 |  |  | 0.642 |
| walker |  | 4095 | 9 | python decl peepdb/db/base.py:29 |  |  | 0.645 |
| walker |  | 4100 | 5 | python body peepdb/db/base.py:17 |  |  | 0.648 |
| ns | 4134 |  | 247 | format_value: numeric/date rendering rules (core.py 119-140) | 3.8 | 1.5 | 0.624 |
| walker |  | 4197 | 97 | headings outline in docs/usage.md |  |  | 0.624 |
| walker |  | 4229 | 32 | docs/usage.md section #0 |  |  | 0.624 |
| walker |  | 4234 | 5 | python body peepdb/db/base.py:21 |  |  | 0.626 |
| walker |  | 4239 | 5 | python body peepdb/db/base.py:25 |  |  | 0.629 |
| walker |  | 4266 | 27 | docs/installation.md section #1 |  |  | 0.629 |
| ns | 4335 |  | 201 | format_as_table: the tabulate grid layout (core.py 102-115) | 3.9 | 1.5 | 0.615 |
| walker |  | 4434 | 168 | README.md section #1 |  |  | 0.640 |
| ns | 4459 |  | 124 | core.py import header and module logger setup | 3.10 |  | 0.627 |
| walker |  | 4506 | 72 | docs/index.md section #0 |  |  | 0.627 |
| walker |  | 4579 | 73 | docs/installation.md section #0 |  |  | 0.627 |
| walker |  | 4600 | 21 | python body peepdb/db/firebase.py:26 |  |  | 0.627 |
| walker |  | 4754 | 154 | headings outline in docs/README.md |  |  | 0.627 |
| walker |  | 4774 | 20 | docs/README.md section #2 |  |  | 0.627 |
| walker |  | 4792 | 18 | docs/README.md section #6 |  |  | 0.627 |
| walker |  | 4850 | 58 | docs/README.md section #1 |  |  | 0.627 |
| ns | 4964 |  | 505 | Every class and method name across all eight backend modules | 4.1 | 3.1 | 0.660 |
| walker |  | 5065 | 215 | docs/README.md section #0 |  |  | 0.660 |
| walker |  | 5116 | 51 | docs/README.md section #7 |  |  | 0.660 |
| walker |  | 5121 | 5 | python body peepdb/db/base.py:29 |  |  | 0.662 |
| walker |  | 5147 | 26 | python doc peepdb/db/mongodb.py:42 |  |  | 0.662 |
| walker |  | 5183 | 36 | docs/installation.md section #4 |  |  | 0.662 |
| ns | 5263 |  | 299 | The table/collection-listing query of every backend | 4.2 | 4.1 | 0.642 |
| walker |  | 5307 | 124 | README.md section #8 |  |  | 0.642 |
| walker |  | 5338 | 31 | python body peepdb/db/mariadb.py:30 |  |  | 0.643 |
| walker |  | 5370 | 32 | python body peepdb/db/oracle.py:32 |  |  | 0.644 |
| walker |  | 5403 | 33 | python body peepdb/db/firebase.py:10 |  |  | 0.644 |
| walker |  | 5437 | 34 | python body peepdb/db/mysql.py:29 |  |  | 0.646 |
| walker |  | 5478 | 41 | docs/usage.md section #2 |  |  | 0.646 |
| walker |  | 5484 | 6 | python body peepdb/db/base.py:37 |  |  | 0.649 |
| walker |  | 5524 | 40 | python body peepdb/db/postgresql.py:30 |  |  | 0.653 |
| walker |  | 5574 | 50 | docs/index.md section #4 |  |  | 0.653 |
| ns | 5595 |  | 332 | MySQLDatabase — the canonical backend implementation | 4.3 | 4.1 | 0.630 |
| walker |  | 5651 | 77 | python body peepdb/config.py:41 |  |  | 0.630 |
| walker |  | 5696 | 45 | python body peepdb/db/mssql.py:7 |  |  | 0.630 |
| walker |  | 5751 | 55 | docs/usage.md section #7 |  |  | 0.630 |
| walker |  | 5803 | 52 | python body peepdb/db/sqlite.py:22 |  |  | 0.630 |
| walker |  | 5856 | 53 | python body peepdb/db/oracle.py:25 |  |  | 0.630 |
| ns | 5897 |  | 302 | PostgreSQL, MariaDB and Oracle connect() — drivers, default ports, cursor factories | 4.4 | 4.1 | 0.609 |
| walker |  | 5910 | 54 | python body peepdb/db/mariadb.py:23 |  |  | 0.609 |
| walker |  | 5964 | 54 | python body peepdb/db/mssql.py:28 |  |  | 0.609 |
| walker |  | 6018 | 54 | python body peepdb/db/mysql.py:22 |  |  | 0.609 |
| walker |  | 6072 | 54 | python body peepdb/db/postgresql.py:23 |  |  | 0.609 |
| ns | 6156 |  | 259 | MongoDBDatabase.connect — URI construction from the generic parameters | 4.5 | 4.1 | 0.595 |
| walker |  | 6273 | 201 | python body peepdb/core.py:101 |  |  | 0.616 |
| walker |  | 6337 | 64 | docs/usage.md section #4 |  |  | 0.616 |
| walker |  | 6401 | 64 | docs/usage.md section #5 |  |  | 0.616 |
| walker |  | 6430 | 29 | python body peepdb/cli.py:113 |  |  | 0.617 |
| ns | 6435 |  | 279 | MSSQLDatabase: constructor extras and the ODBC connection string | 4.6 | 4.1 | 0.607 |
| ns | 6582 |  | 147 | FirebaseDatabase: the backend that does not call super().__init__ | 4.7 | 4.1 | 0.600 |
| walker |  | 6662 | 232 | README.md section #5 |  |  | 0.600 |
| walker |  | 6863 | 201 | README.md section #6 |  |  | 0.600 |
| ns | 6899 |  | 317 | SQLiteDatabase connect and fetch_data — file-path handling and print-based logging | 4.8 | 4.1 | 0.587 |
| walker |  | 6985 | 122 | docs/README.md section #5 |  |  | 0.587 |
| walker |  | 7052 | 67 | docs/installation.md section #2 |  |  | 0.587 |
| ns | 7201 |  | 302 | The non-LIMIT/OFFSET pagination variants (MSSQL, Oracle, MongoDB, Firebase) | 4.9 | 4.1 | 0.574 |
| walker |  | 7299 | 247 | python body peepdb/core.py:118 |  |  | 0.604 |
| ns | 7310 |  | 109 | Where credentials live: config paths and KeySecurity modes | 5.1 | 1.9 | 0.608 |
| walker |  | 7347 | 48 | python body peepdb/db/mongodb.py:32 |  |  | 0.608 |
| ns | 7495 |  | 185 | get_key_security_config / get_key / encrypt / decrypt bodies | 5.2 | 1.9 | 0.600 |
| ns | 7742 |  | 247 | The two key sources: PBKDF2 derivation and keyring fetch-or-create | 5.3 | 1.9 | 0.595 |
| ns | 7987 |  | 245 | save_connection: the on-disk record shape and what is encrypted | 5.4 | 1.9 | 0.583 |
| ns | 8167 |  | 180 | get_connection: the tuple contract and InvalidPassword | 5.5 | 1.9 | 0.574 |
| walker |  | 8214 | 867 | plaintext config requirements.txt |  |  | 0.574 |
| walker |  | 8295 | 81 | python body peepdb/db/firebase.py:30 |  |  | 0.579 |
| ns | 8309 |  | 142 | add_key_security: first-run choice between keyring and password | 5.6 | 1.9 | 0.575 |
| walker |  | 8400 | 105 | python body peepdb/config.py:60 |  |  | 0.583 |
| ns | 8403 |  | 94 | list/remove/remove_all_connections — the distinctive lines only | 5.7 | 1.9 | 0.578 |
| walker |  | 8488 | 88 | docs/index.md section #2 |  |  | 0.578 |
| walker |  | 8584 | 96 | docs/usage.md section #3 |  |  | 0.578 |
| walker |  | 8625 | 41 | python doc peepdb/cli.py:86 |  |  | 0.578 |
| walker |  | 8728 | 103 | python body peepdb/db/mssql.py:35 |  |  | 0.586 |
| ns | 8760 |  | 357 | Every test (and fixture) name in peepdb/tests, across all six modules | 6.1 | 1.11 | 0.573 |
| walker |  | 8828 | 100 | docs/usage.md section #1 |  |  | 0.573 |
| walker |  | 8842 | 14 | python body peepdb/db/base.py:33 |  |  | 0.577 |
| ns | 8866 |  | 106 | How the suite is run: CONTRIBUTING instructions and the CI invocation | 6.2 |  | 0.573 |
| walker |  | 9065 | 223 | docs/README.md section #3 |  |  | 0.573 |
| ns | 9081 |  | 215 | A representative test body: patching style and asserted output strings | 6.3 | 6.1 | 0.567 |
| ns | 9171 |  | 90 | `peepdb --help` summary lines from the cli group docstring | 7.1 | 1.6 | 0.565 |
| walker |  | 9173 | 108 | python body peepdb/config.py:49 |  |  | 0.573 |
| ns | 9427 |  | 256 | The two disagreeing dependency lists (setup.py vs project.toml) | 7.2 |  | 0.568 |
| ns | 9564 |  | 137 | Build backend, Python floor, packaging includes | 7.3 | 1.1 | 0.570 |
| walker |  | 9600 | 427 | python body peepdb/core.py:27 |  |  | 0.599 |
| walker |  | 9714 | 114 | docs/usage.md section #6 |  |  | 0.599 |
| ns | 9752 |  | 188 | CI: what triggers the workflows and on what Python versions | 7.4 |  | 0.592 |
| walker |  | 9757 | 43 | python doc peepdb/cli.py:113 |  |  | 0.592 |
| walker |  | 9773 | 16 | python names peepdb/tests/test_mongodb_uri.py |  |  | 0.592 |
| walker |  | 9789 | 16 | python names peepdb/tests/test_sqlite.py |  |  | 0.592 |
| ns | 9913 |  | 161 | The docs/ site: Jekyll theme config, and two stale-template markers | 7.5 | 1.11 | 0.592 |
| walker |  | 9917 | 128 | docs/installation.md section #3 |  |  | 0.592 |
| ns | 9978 |  | 65 | CustomEncoder.default — the JSON serializer for Decimal and date | 7.6 | 1.6 | 0.589 |

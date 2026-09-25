Score(3000)=0.615 I=0.863 C=0.438 ns_rows≤3K=21/54 grid(1000/1442/2080/3000/4327/6240/9000)=0.696/0.650/0.689/0.615/0.626/0.594/0.560

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
| walker |  | 374 | 85 | README headline in README.md |  |  | 0.944 |
| ns | 378 |  | 81 | peepdb/cli.py: every top-level definition, names only | 1.6 |  | 0.842 |
| walker |  | 388 | 14 | python names peepdb/db/base.py |  |  | 0.842 |
| walker |  | 402 | 14 | python names peepdb/db/oracle.py |  |  | 0.842 |
| walker |  | 416 | 14 | python names peepdb/db/sqlite.py |  |  | 0.842 |
| walker |  | 443 | 27 | [features] / entry-point scripts in project.toml |  |  | 0.844 |
| walker |  | 458 | 15 | python names peepdb/db/mariadb.py |  |  | 0.844 |
| walker |  | 473 | 15 | python names peepdb/db/mongodb.py |  |  | 0.844 |
| walker |  | 488 | 15 | python names peepdb/db/mssql.py |  |  | 0.844 |
| ns | 497 |  | 119 | peepdb/db/__init__.py — backend class ↔ module map | 1.7 |  | 0.742 |
| walker |  | 503 | 15 | python names peepdb/db/mysql.py |  |  | 0.742 |
| walker |  | 518 | 15 | python names peepdb/db/postgresql.py |  |  | 0.743 |
| ns | 676 |  | 179 | README feature bullets | 1.8 |  | 0.680 |
| walker |  | 733 | 215 | python names peepdb/db/__init__.py |  |  | 0.800 |
| walker |  | 764 | 31 | python names peepdb/db/firebase.py |  |  | 0.800 |
| walker |  | 820 | 56 | package metadata in project.toml |  |  | 0.800 |
| ns | 831 |  | 155 | peepdb/config.py: every top-level definition, names only | 1.9 |  | 0.719 |
| ns | 908 |  | 77 | Entry points: console script, __main__.py, exceptions.py in full | 1.10 |  | 0.696 |
| walker |  | 1012 | 192 | [dependencies] in project.toml |  |  | 0.697 |
| ns | 1027 |  | 119 | Complete listings of peepdb/tests, docs, images, .github/workflows | 1.11 |  | 0.673 |
| walker |  | 1065 | 53 | python decl peepdb/db/oracle.py:7 |  |  | 0.673 |
| walker |  | 1119 | 54 | python decl peepdb/db/mongodb.py:8 |  |  | 0.674 |
| ns | 1209 |  | 182 | README section-heading map (every H2 and H3 after the feature list) | 1.12 | 1.1 | 0.628 |
| walker |  | 1230 | 111 | python names peepdb/cli.py |  |  | 0.683 |
| walker |  | 1238 | 8 | python decl peepdb/cli.py:86 |  |  | 0.683 |
| walker |  | 1249 | 11 | python decl peepdb/cli.py:16 |  |  | 0.683 |
| walker |  | 1266 | 17 | python decl peepdb/cli.py:25 |  |  | 0.683 |
| walker |  | 1297 | 31 | python decl peepdb/cli.py:113 |  |  | 0.684 |
| walker |  | 1338 | 41 | python decl peepdb/cli.py:97 |  |  | 0.687 |
| walker |  | 1345 | 7 | python body peepdb/cli.py:181 |  |  | 0.687 |
| ns | 1441 |  | 232 | `peepdb save` full option decorator block (cli.py 53-64) | 2.1 | 1.6 | 0.650 |
| walker |  | 1463 | 118 | python names peepdb/core.py |  |  | 0.677 |
| walker |  | 1572 | 109 | python decl peepdb/core.py:56 |  |  | 0.680 |
| ns | 1583 |  | 142 | `peepdb view` full option decorator block (cli.py 126-132) | 2.2 | 1.6 | 0.660 |
| ns | 1680 |  | 97 | Remaining Click decorators: group, version option, confirmations | 2.3 | 1.6 | 0.674 |
| walker |  | 1765 | 193 | headings outline in README.md |  |  | 0.741 |
| walker |  | 1807 | 42 | README.md section #19 |  |  | 0.741 |
| walker |  | 1842 | 35 | README.md section #8 |  |  | 0.741 |
| walker |  | 1872 | 30 | README.md section #11 |  |  | 0.741 |
| walker |  | 1900 | 28 | README.md section #30 |  |  | 0.741 |
| walker |  | 1919 | 19 | README.md section #5 |  |  | 0.742 |
| ns | 1922 |  | 242 | `view` docstring + connection resolution (cli.py 134-155) | 2.4 | 1.6 | 0.685 |
| walker |  | 1949 | 30 | README.md section #29 |  |  | 0.685 |
| walker |  | 2045 | 96 | README headline in docs/README.md |  |  | 0.685 |
| walker |  | 2066 | 21 | README.md section #7 |  |  | 0.689 |
| walker |  | 2086 | 20 | README.md section #6 |  |  | 0.693 |
| walker |  | 2124 | 38 | python decl peepdb/db/oracle.py:36 |  |  | 0.693 |
| walker |  | 2159 | 35 | README.md section #28 |  |  | 0.693 |
| walker |  | 2167 | 8 | python body peepdb/cli.py:86 |  |  | 0.693 |
| ns | 2207 |  | 285 | `view` dispatch into peep_db + output rendering (cli.py 156-178) | 2.5 | 2.4 | 0.644 |
| walker |  | 2309 | 142 | python decl peepdb/cli.py:126 |  |  | 0.671 |
| ns | 2425 |  | 218 | `save` docstring and body (cli.py 66-83) | 2.6 | 1.6 | 0.637 |
| ns | 2515 |  | 90 | Bodies of `list`, `remove` and `remove-all` (cli.py) | 2.7 | 1.6 | 0.621 |
| walker |  | 2581 | 272 | python names peepdb/config.py |  |  | 0.671 |
| walker |  | 2599 | 18 | python decl peepdb/config.py:29 |  |  | 0.671 |
| walker |  | 2619 | 20 | python decl peepdb/config.py:41 |  |  | 0.671 |
| walker |  | 2649 | 30 | python decl peepdb/config.py:15 |  |  | 0.671 |
| walker |  | 2664 | 15 | python body peepdb/config.py:70 |  |  | 0.671 |
| walker |  | 2679 | 15 | python body peepdb/config.py:73 |  |  | 0.671 |
| ns | 2746 |  | 231 | BaseDatabase: class line + the four abstract methods + context manager | 3.1 |  | 0.633 |
| walker |  | 2758 | 79 | python decl peepdb/db/mariadb.py:5 |  |  | 0.633 |
| walker |  | 2837 | 79 | python decl peepdb/db/mysql.py:5 |  |  | 0.634 |
| ns | 2913 |  | 167 | BaseDatabase.__init__ and base.py imports | 3.2 | 3.1 | 0.614 |
| walker |  | 2916 | 79 | python decl peepdb/db/postgresql.py:6 |  |  | 0.615 |
| walker |  | 2995 | 79 | python decl peepdb/db/sqlite.py:6 |  |  | 0.615 |
| walker |  | 3018 | 23 | README.md section #3 |  |  | 0.619 |
| ns | 3026 |  | 113 | core.peep_db full signature (core.py 56-66) | 3.3 | 1.5 | 0.633 |
| walker |  | 3040 | 22 | README.md section #4 |  |  | 0.638 |
| walker |  | 3063 | 23 | README.md section #2 |  |  | 0.645 |
| walker |  | 3134 | 71 | README.md section #13 |  |  | 0.645 |
| walker |  | 3180 | 46 | README.md section #10 |  |  | 0.645 |
| walker |  | 3234 | 54 | headings outline in docs/installation.md |  |  | 0.645 |
| ns | 3292 |  | 266 | connect_to_database: the first five engine branches (core.py 28-42) | 3.4 | 1.5 | 0.624 |
| walker |  | 3361 | 127 | manifest config in project.toml |  |  | 0.624 |
| walker |  | 3417 | 56 | headings outline in docs/index.md |  |  | 0.624 |
| ns | 3453 |  | 161 | connect_to_database: the irregular branches and the failure path (core.py 43-53) | 3.5 | 3.4 | 0.611 |
| walker |  | 3473 | 56 | README.md section #9 |  |  | 0.611 |
| walker |  | 3569 | 96 | python decl peepdb/db/firebase.py:9 |  |  | 0.612 |
| walker |  | 3624 | 55 | python decl peepdb/db/mongodb.py:42 |  |  | 0.612 |
| walker |  | 3634 | 10 | python body peepdb/db/mongodb.py:39 |  |  | 0.612 |
| ns | 3651 |  | 198 | peep_db body: connect, fetch one table or all (core.py 67-79) | 3.6 | 3.3 | 0.597 |
| walker |  | 3676 | 42 | listing of 'peepdb/tests' |  |  | 0.631 |
| ns | 3887 |  | 236 | peep_db body: table-vs-JSON post-processing and the finally-disconnect (core.py 80-98) | 3.7 | 3.6 | 0.610 |
| walker |  | 3908 | 232 | python decl peepdb/cli.py:53 |  |  | 0.640 |
| walker |  | 4034 | 126 | declaration surface of docs/Gemfile |  |  | 0.640 |
| ns | 4134 |  | 247 | format_value: numeric/date rendering rules (core.py 119-140) | 3.8 | 1.5 | 0.617 |
| walker |  | 4159 | 125 | python decl peepdb/db/mssql.py:6 |  |  | 0.618 |
| walker |  | 4194 | 35 | README.md section #1 |  |  | 0.626 |
| walker |  | 4201 | 7 | python body peepdb/cli.py:25 |  |  | 0.626 |
| ns | 4335 |  | 201 | format_as_table: the tabulate grid layout (core.py 102-115) | 3.9 | 1.5 | 0.612 |
| walker |  | 4353 | 152 | python decl peepdb/db/base.py:5 |  |  | 0.622 |
| walker |  | 4362 | 9 | python decl peepdb/db/base.py:17 |  |  | 0.625 |
| walker |  | 4371 | 9 | python decl peepdb/db/base.py:21 |  |  | 0.627 |
| walker |  | 4380 | 9 | python decl peepdb/db/base.py:25 |  |  | 0.629 |
| walker |  | 4389 | 9 | python decl peepdb/db/base.py:29 |  |  | 0.632 |
| walker |  | 4394 | 5 | python body peepdb/db/base.py:17 |  |  | 0.634 |
| walker |  | 4417 | 23 | README.md section #16 |  |  | 0.634 |
| ns | 4459 |  | 124 | core.py import header and module logger setup | 3.10 |  | 0.621 |
| walker |  | 4514 | 97 | headings outline in docs/usage.md |  |  | 0.621 |
| walker |  | 4546 | 32 | docs/usage.md section #0 |  |  | 0.621 |
| walker |  | 4551 | 5 | python body peepdb/db/base.py:21 |  |  | 0.624 |
| walker |  | 4658 | 107 | README.md section #22 |  |  | 0.624 |
| walker |  | 4663 | 5 | python body peepdb/db/base.py:25 |  |  | 0.626 |
| walker |  | 4690 | 27 | docs/installation.md section #1 |  |  | 0.626 |
| walker |  | 4721 | 31 | README.md section #18 |  |  | 0.626 |
| walker |  | 4793 | 72 | docs/index.md section #0 |  |  | 0.626 |
| walker |  | 4866 | 73 | docs/installation.md section #0 |  |  | 0.626 |
| walker |  | 4887 | 21 | python body peepdb/db/firebase.py:26 |  |  | 0.626 |
| ns | 4964 |  | 505 | Every class and method name across all eight backend modules | 4.1 | 3.1 | 0.658 |
| walker |  | 5041 | 154 | headings outline in docs/README.md |  |  | 0.658 |
| walker |  | 5061 | 20 | docs/README.md section #2 |  |  | 0.658 |
| walker |  | 5079 | 18 | docs/README.md section #8 |  |  | 0.658 |
| walker |  | 5137 | 58 | docs/README.md section #1 |  |  | 0.658 |
| ns | 5263 |  | 299 | The table/collection-listing query of every backend | 4.2 | 4.1 | 0.639 |
| walker |  | 5352 | 215 | docs/README.md section #0 |  |  | 0.639 |
| walker |  | 5386 | 34 | README.md section #20 |  |  | 0.639 |
| walker |  | 5437 | 51 | docs/README.md section #9 |  |  | 0.639 |
| walker |  | 5442 | 5 | python body peepdb/db/base.py:29 |  |  | 0.641 |
| walker |  | 5468 | 26 | python doc peepdb/db/mongodb.py:42 |  |  | 0.641 |
| walker |  | 5507 | 39 | README.md section #23 |  |  | 0.641 |
| walker |  | 5544 | 37 | README.md section #24 |  |  | 0.641 |
| walker |  | 5580 | 36 | docs/installation.md section #4 |  |  | 0.641 |
| ns | 5595 |  | 332 | MySQLDatabase — the canonical backend implementation | 4.3 | 4.1 | 0.618 |
| walker |  | 5603 | 23 | docs/README.md section #5 |  |  | 0.618 |
| walker |  | 5727 | 124 | README.md section #27 |  |  | 0.618 |
| walker |  | 5758 | 31 | python body peepdb/db/mariadb.py:30 |  |  | 0.619 |
| walker |  | 5790 | 32 | python body peepdb/db/oracle.py:32 |  |  | 0.620 |
| walker |  | 5835 | 45 | README.md section #12 |  |  | 0.620 |
| walker |  | 5868 | 33 | python body peepdb/db/firebase.py:10 |  |  | 0.620 |
| ns | 5897 |  | 302 | PostgreSQL, MariaDB and Oracle connect() — drivers, default ports, cursor factories | 4.4 | 4.1 | 0.600 |
| walker |  | 5902 | 34 | python body peepdb/db/mysql.py:29 |  |  | 0.602 |
| walker |  | 5943 | 41 | docs/usage.md section #2 |  |  | 0.602 |
| walker |  | 5993 | 50 | README.md section #15 |  |  | 0.602 |
| walker |  | 5999 | 6 | python body peepdb/db/base.py:37 |  |  | 0.604 |
| walker |  | 6039 | 40 | python body peepdb/db/postgresql.py:30 |  |  | 0.608 |
| walker |  | 6089 | 50 | docs/index.md section #10 |  |  | 0.608 |
| ns | 6156 |  | 259 | MongoDBDatabase.connect — URI construction from the generic parameters | 4.5 | 4.1 | 0.594 |
| walker |  | 6166 | 77 | python body peepdb/config.py:41 |  |  | 0.594 |
| walker |  | 6211 | 45 | python body peepdb/db/mssql.py:7 |  |  | 0.594 |
| walker |  | 6238 | 27 | docs/README.md section #12 |  |  | 0.594 |
| walker |  | 6293 | 55 | docs/usage.md section #7 |  |  | 0.594 |
| walker |  | 6357 | 64 | README.md section #14 |  |  | 0.594 |
| walker |  | 6409 | 52 | python body peepdb/db/sqlite.py:22 |  |  | 0.594 |
| ns | 6435 |  | 279 | MSSQLDatabase: constructor extras and the ODBC connection string | 4.6 | 4.1 | 0.584 |
| walker |  | 6462 | 53 | python body peepdb/db/oracle.py:25 |  |  | 0.584 |
| walker |  | 6516 | 54 | python body peepdb/db/mariadb.py:23 |  |  | 0.584 |
| walker |  | 6570 | 54 | python body peepdb/db/mssql.py:28 |  |  | 0.584 |
| ns | 6582 |  | 147 | FirebaseDatabase: the backend that does not call super().__init__ | 4.7 | 4.1 | 0.577 |
| walker |  | 6624 | 54 | python body peepdb/db/mysql.py:22 |  |  | 0.577 |
| walker |  | 6678 | 54 | python body peepdb/db/postgresql.py:23 |  |  | 0.577 |
| walker |  | 6697 | 19 | docs/index.md section #5 |  |  | 0.577 |
| walker |  | 6898 | 201 | python body peepdb/core.py:101 |  |  | 0.597 |
| ns | 6899 |  | 317 | SQLiteDatabase connect and fetch_data — file-path handling and print-based logging | 4.8 | 4.1 | 0.584 |
| walker |  | 6997 | 99 | docs/README.md section #10 |  |  | 0.584 |
| walker |  | 7061 | 64 | docs/usage.md section #4 |  |  | 0.584 |
| walker |  | 7125 | 64 | docs/usage.md section #5 |  |  | 0.584 |
| walker |  | 7154 | 29 | python body peepdb/cli.py:113 |  |  | 0.586 |
| walker |  | 7175 | 21 | docs/index.md section #7 |  |  | 0.586 |
| walker |  | 7195 | 20 | docs/index.md section #6 |  |  | 0.586 |
| ns | 7201 |  | 302 | The non-LIMIT/OFFSET pagination variants (MSSQL, Oracle, MongoDB, Firebase) | 4.9 | 4.1 | 0.573 |
| ns | 7310 |  | 109 | Where credentials live: config paths and KeySecurity modes | 5.1 | 1.9 | 0.578 |
| walker |  | 7317 | 122 | docs/README.md section #7 |  |  | 0.578 |
| walker |  | 7384 | 67 | docs/installation.md section #2 |  |  | 0.578 |
| ns | 7495 |  | 185 | get_key_security_config / get_key / encrypt / decrypt bodies | 5.2 | 1.9 | 0.570 |
| walker |  | 7631 | 247 | python body peepdb/core.py:118 |  |  | 0.599 |
| walker |  | 7654 | 23 | docs/index.md section #3 |  |  | 0.599 |
| walker |  | 7676 | 22 | docs/index.md section #4 |  |  | 0.599 |
| walker |  | 7699 | 23 | docs/index.md section #2 |  |  | 0.599 |
| ns | 7742 |  | 247 | The two key sources: PBKDF2 derivation and keyring fetch-or-create | 5.3 | 1.9 | 0.594 |
| walker |  | 7747 | 48 | python body peepdb/db/mongodb.py:32 |  |  | 0.594 |
| ns | 7987 |  | 245 | save_connection: the on-disk record shape and what is encrypted | 5.4 | 1.9 | 0.582 |
| ns | 8167 |  | 180 | get_connection: the tuple contract and InvalidPassword | 5.5 | 1.9 | 0.573 |
| ns | 8309 |  | 142 | add_key_security: first-run choice between keyring and password | 5.6 | 1.9 | 0.568 |
| ns | 8403 |  | 94 | list/remove/remove_all_connections — the distinctive lines only | 5.7 | 1.9 | 0.563 |
| walker |  | 8614 | 867 | plaintext config requirements.txt |  |  | 0.563 |
| walker |  | 8687 | 73 | README.md section #26 |  |  | 0.563 |
| ns | 8760 |  | 357 | Every test (and fixture) name in peepdb/tests, across all six modules | 6.1 | 1.11 | 0.551 |
| walker |  | 8768 | 81 | python body peepdb/db/firebase.py:30 |  |  | 0.557 |
| walker |  | 8845 | 77 | README.md section #17 |  |  | 0.557 |
| ns | 8866 |  | 106 | How the suite is run: CONTRIBUTING instructions and the CI invocation | 6.2 |  | 0.553 |
| walker |  | 8950 | 105 | python body peepdb/config.py:60 |  |  | 0.560 |
| walker |  | 9038 | 88 | docs/index.md section #8 |  |  | 0.560 |
| ns | 9081 |  | 215 | A representative test body: patching style and asserted output strings | 6.3 | 6.1 | 0.554 |
| walker |  | 9084 | 46 | docs/README.md section #11 |  |  | 0.554 |
| ns | 9171 |  | 90 | `peepdb --help` summary lines from the cli group docstring | 7.1 | 1.6 | 0.552 |
| walker |  | 9180 | 96 | docs/usage.md section #3 |  |  | 0.552 |
| walker |  | 9221 | 41 | python doc peepdb/cli.py:86 |  |  | 0.553 |
| walker |  | 9324 | 103 | python body peepdb/db/mssql.py:35 |  |  | 0.560 |
| walker |  | 9424 | 100 | docs/usage.md section #1 |  |  | 0.560 |
| ns | 9427 |  | 256 | The two disagreeing dependency lists (setup.py vs project.toml) | 7.2 |  | 0.556 |
| walker |  | 9438 | 14 | python body peepdb/db/base.py:33 |  |  | 0.560 |
| ns | 9564 |  | 137 | Build backend, Python floor, packaging includes | 7.3 | 1.1 | 0.562 |
| walker |  | 9661 | 223 | docs/README.md section #3 |  |  | 0.562 |
| ns | 9752 |  | 188 | CI: what triggers the workflows and on what Python versions | 7.4 |  | 0.555 |
| walker |  | 9769 | 108 | python body peepdb/config.py:49 |  |  | 0.562 |
| ns | 9913 |  | 161 | The docs/ site: Jekyll theme config, and two stale-template markers | 7.5 | 1.11 | 0.562 |
| ns | 9978 |  | 65 | CustomEncoder.default — the JSON serializer for Decimal and date | 7.6 | 1.6 | 0.560 |

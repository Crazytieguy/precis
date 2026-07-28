Score(3000)=0.497 I=0.777 C=0.318 ns_rows≤3K=21/54 (reached=6 partial=1 missing=14)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 42 | 42 | listing of '.' |  |  | 0.000 |
| walker |  | 45 | 3 | listing of '.github' |  |  | 0.000 |
| walker |  | 52 | 7 | listing of '.github/workflows' |  |  | 0.000 |
| ns | 54 |  | 54 | Identity: README title + package name/version/description | 1.1 |  | 0.000 |
| walker |  | 79 | 27 | listing of 'images' |  |  | 0.000 |
| ns | 96 |  | 42 | Complete repository root listing | 1.2 |  | 0.539 |
| walker |  | 108 | 29 | entry-point scripts in project.toml |  |  | 0.541 |
| walker |  | 143 | 35 | listing of 'peepdb' |  |  | 0.574 |
| ns | 177 |  | 81 | README lede paragraph — scope and supported databases | 1.3 | 1.1 | 0.552 |
| walker |  | 183 | 40 | listing of 'docs' |  |  | 0.569 |
| walker |  | 227 | 44 | listing of 'peepdb/db' |  |  | 0.659 |
| ns | 257 |  | 80 | Source roster: complete listing of peepdb/ and peepdb/db/ | 1.4 |  | 0.662 |
| ns | 302 |  | 45 | peepdb/core.py: every top-level function, names only | 1.5 |  | 0.619 |
| walker |  | 312 | 85 | README headline in README.md |  |  | 0.647 |
| ns | 383 |  | 81 | peepdb/cli.py: every top-level definition, names only | 1.6 |  | 0.577 |
| ns | 502 |  | 119 | peepdb/db/__init__.py — backend class ↔ module map | 1.7 |  | 0.508 |
| ns | 681 |  | 179 | README feature bullets | 1.8 |  | 0.465 |
| ns | 836 |  | 155 | peepdb/config.py: every top-level definition, names only | 1.9 |  | 0.418 |
| walker |  | 905 | 593 | YAML config at .github/workflows/test.yml |  |  | 0.419 |
| ns | 913 |  | 77 | Entry points: console script, __main__.py, exceptions.py in full | 1.10 |  | 0.400 |
| walker |  | 1001 | 96 | README headline in docs/README.md |  |  | 0.400 |
| walker |  | 1013 | 12 | python imports in peepdb/__main__.py |  |  | 0.405 |
| walker |  | 1027 | 14 | python imports in setup.py |  |  | 0.405 |
| ns | 1029 |  | 116 | Complete listings of peepdb/tests, docs, images, .github/workflows | 1.11 |  | 0.409 |
| walker |  | 1081 | 54 | headings outline in docs/installation.md |  |  | 0.409 |
| walker |  | 1137 | 56 | headings outline in docs/index.md |  |  | 0.409 |
| ns | 1211 |  | 182 | README section-heading map (every H2 and H3 after the feature list) | 1.12 | 1.1 | 0.382 |
| walker |  | 1352 | 215 | python imports in peepdb/db/__init__.py |  |  | 0.446 |
| walker |  | 1426 | 74 | [package] in project.toml |  |  | 0.621 |
| ns | 1443 |  | 232 | `peepdb save` full option decorator block (cli.py 53-64) | 2.1 | 1.6 | 0.588 |
| walker |  | 1482 | 56 | package metadata in project.toml |  |  | 0.588 |
| ns | 1585 |  | 142 | `peepdb view` full option decorator block (cli.py 126-132) | 2.2 | 1.6 | 0.571 |
| walker |  | 1675 | 193 | headings outline in README.md |  |  | 0.645 |
| ns | 1682 |  | 97 | Remaining Click decorators: group, version option, confirmations | 2.3 | 1.6 | 0.625 |
| walker |  | 1717 | 42 | README.md section #19 |  |  | 0.625 |
| walker |  | 1752 | 35 | README.md section #8 |  |  | 0.625 |
| walker |  | 1782 | 30 | README.md section #11 |  |  | 0.625 |
| walker |  | 1810 | 28 | README.md section #30 |  |  | 0.625 |
| walker |  | 1829 | 19 | README.md section #5 |  |  | 0.626 |
| walker |  | 1859 | 30 | README.md section #29 |  |  | 0.626 |
| walker |  | 1880 | 21 | README.md section #7 |  |  | 0.630 |
| walker |  | 1900 | 20 | README.md section #6 |  |  | 0.635 |
| ns | 1924 |  | 242 | `view` docstring + connection resolution (cli.py 134-155) | 2.4 | 1.6 | 0.586 |
| walker |  | 1935 | 35 | README.md section #28 |  |  | 0.586 |
| walker |  | 2032 | 97 | headings outline in docs/usage.md |  |  | 0.586 |
| walker |  | 2055 | 23 | README.md section #3 |  |  | 0.592 |
| walker |  | 2077 | 22 | README.md section #4 |  |  | 0.600 |
| walker |  | 2100 | 23 | README.md section #2 |  |  | 0.609 |
| walker |  | 2171 | 71 | README.md section #13 |  |  | 0.609 |
| ns | 2209 |  | 285 | `view` dispatch into peep_db + output rendering (cli.py 156-178) | 2.5 | 2.4 | 0.566 |
| walker |  | 2217 | 46 | README.md section #10 |  |  | 0.566 |
| walker |  | 2409 | 192 | [dependencies] in project.toml |  |  | 0.566 |
| ns | 2427 |  | 218 | `save` docstring and body (cli.py 66-83) | 2.6 | 1.6 | 0.538 |
| ns | 2517 |  | 90 | Bodies of `list`, `remove` and `remove-all` (cli.py) | 2.7 | 1.6 | 0.524 |
| walker |  | 2536 | 127 | manifest config in project.toml |  |  | 0.525 |
| walker |  | 2592 | 56 | README.md section #9 |  |  | 0.525 |
| walker |  | 2624 | 32 | docs/usage.md section #0 |  |  | 0.525 |
| ns | 2748 |  | 231 | BaseDatabase: class line + the four abstract methods + context manager | 3.1 |  | 0.495 |
| walker |  | 2778 | 154 | headings outline in docs/README.md |  |  | 0.495 |
| walker |  | 2798 | 20 | docs/README.md section #2 |  |  | 0.495 |
| walker |  | 2816 | 18 | docs/README.md section #8 |  |  | 0.495 |
| ns | 2915 |  | 167 | BaseDatabase.__init__ and base.py imports | 3.2 | 3.1 | 0.479 |
| walker |  | 2942 | 126 | declaration surface of docs/Gemfile |  |  | 0.479 |
| walker |  | 2953 | 11 | python decl names surface in peepdb/exceptions.py |  |  | 0.484 |
| walker |  | 2953 | 0 | python decl at peepdb/exceptions.py:1 |  |  | 0.484 |
| walker |  | 2958 | 5 | python class body at peepdb/exceptions.py:1 |  |  | 0.488 |
| walker |  | 2993 | 35 | README.md section #1 |  |  | 0.497 |
| ns | 3028 |  | 113 | core.peep_db full signature (core.py 56-66) | 3.3 | 1.5 | 0.485 |
| walker |  | 3178 | 185 | python setup manifest at setup.py:16 |  |  | 0.486 |
| walker |  | 3236 | 58 | docs/README.md section #1 |  |  | 0.486 |
| ns | 3294 |  | 266 | connect_to_database: the first five engine branches (core.py 28-42) | 3.4 | 1.5 | 0.471 |
| walker |  | 3320 | 84 | python decl names surface in peepdb/cli.py |  |  | 0.477 |
| walker |  | 3320 | 0 | python decl at peepdb/cli.py:16 |  |  | 0.477 |
| walker |  | 3320 | 0 | python decl at peepdb/cli.py:181 |  |  | 0.477 |
| walker |  | 3327 | 7 | python decl at peepdb/cli.py:86 |  |  | 0.479 |
| walker |  | 3343 | 16 | python decl at peepdb/cli.py:25 |  |  | 0.484 |
| walker |  | 3350 | 7 | python decl body at peepdb/cli.py:25 body 50 |  |  | 0.484 |
| walker |  | 3357 | 7 | python decl body at peepdb/cli.py:181 body 182 |  |  | 0.484 |
| walker |  | 3365 | 8 | python decl body at peepdb/cli.py:86 body 94 |  |  | 0.484 |
| walker |  | 3396 | 31 | python decl at peepdb/cli.py:113 |  |  | 0.495 |
| walker |  | 3438 | 42 | python decl at peepdb/cli.py:97 |  |  | 0.516 |
| walker |  | 3449 | 11 | python method sigs in peepdb/cli.py |  |  | 0.516 |
| walker |  | 3449 | 0 | python method at peepdb/cli.py:17 |  |  | 0.516 |
| ns | 3455 |  | 161 | connect_to_database: the irregular branches and the failure path (core.py 43-53) | 3.5 | 3.4 | 0.505 |
| walker |  | 3490 | 41 | python decl doc at peepdb/cli.py:86 |  |  | 0.505 |
| walker |  | 3535 | 45 | python decl doc at peepdb/cli.py:113 |  |  | 0.506 |
| walker |  | 3582 | 47 | python decl doc at peepdb/cli.py:97 |  |  | 0.506 |
| walker |  | 3596 | 14 | python decl names surface in peepdb/db/base.py |  |  | 0.506 |
| walker |  | 3596 | 0 | python decl at peepdb/db/base.py:5 |  |  | 0.506 |
| ns | 3653 |  | 198 | peep_db body: connect, fetch one table or all (core.py 67-79) | 3.6 | 3.3 | 0.494 |
| walker |  | 3748 | 152 | python method sigs in peepdb/db/base.py |  |  | 0.508 |
| walker |  | 3748 | 0 | python method at peepdb/db/base.py:6 |  |  | 0.508 |
| walker |  | 3748 | 0 | python method at peepdb/db/base.py:33 |  |  | 0.508 |
| walker |  | 3748 | 0 | python method at peepdb/db/base.py:37 |  |  | 0.508 |
| walker |  | 3757 | 9 | python method at peepdb/db/base.py:17 |  |  | 0.511 |
| walker |  | 3766 | 9 | python method at peepdb/db/base.py:21 |  |  | 0.514 |
| walker |  | 3775 | 9 | python method at peepdb/db/base.py:25 |  |  | 0.517 |
| walker |  | 3784 | 9 | python method at peepdb/db/base.py:29 |  |  | 0.520 |
| walker |  | 3789 | 5 | python method body at peepdb/db/base.py:17 body 19 |  |  | 0.523 |
| walker |  | 3812 | 23 | README.md section #16 |  |  | 0.523 |
| ns | 3889 |  | 236 | peep_db body: table-vs-JSON post-processing and the finally-disconnect (core.py 80-98) | 3.7 | 3.6 | 0.505 |
| walker |  | 3922 | 110 | plaintext config requirements.txt |  |  | 0.505 |
| walker |  | 3963 | 41 | listing of 'peepdb/tests' |  |  | 0.539 |
| ns | 4136 |  | 247 | format_value: numeric/date rendering rules (core.py 119-140) | 3.8 | 1.5 | 0.519 |
| walker |  | 4178 | 215 | docs/README.md section #0 |  |  | 0.519 |
| walker |  | 4285 | 107 | README.md section #22 |  |  | 0.519 |
| walker |  | 4290 | 5 | python method body at peepdb/db/base.py:21 body 23 |  |  | 0.522 |
| walker |  | 4317 | 27 | docs/installation.md section #1 |  |  | 0.522 |
| ns | 4337 |  | 201 | format_as_table: the tabulate grid layout (core.py 102-115) | 3.9 | 1.5 | 0.510 |
| walker |  | 4348 | 31 | README.md section #18 |  |  | 0.510 |
| walker |  | 4420 | 72 | docs/index.md section #0 |  |  | 0.510 |
| walker |  | 4425 | 5 | python method body at peepdb/db/base.py:25 body 27 |  |  | 0.513 |
| ns | 4461 |  | 124 | core.py import header and module logger setup | 3.10 |  | 0.502 |
| walker |  | 4498 | 73 | docs/installation.md section #0 |  |  | 0.502 |
| walker |  | 4790 | 292 | python decl names surface in peepdb/config.py |  |  | 0.523 |
| walker |  | 4790 | 0 | python decl at peepdb/config.py:49 |  |  | 0.523 |
| walker |  | 4790 | 0 | python decl at peepdb/config.py:60 |  |  | 0.523 |
| walker |  | 4790 | 0 | python decl at peepdb/config.py:70 |  |  | 0.523 |
| walker |  | 4790 | 0 | python decl at peepdb/config.py:73 |  |  | 0.523 |
| walker |  | 4790 | 0 | python decl at peepdb/config.py:76 |  |  | 0.523 |
| walker |  | 4790 | 0 | python decl at peepdb/config.py:112 |  |  | 0.523 |
| walker |  | 4790 | 0 | python decl at peepdb/config.py:144 |  |  | 0.523 |
| walker |  | 4790 | 0 | python decl at peepdb/config.py:161 |  |  | 0.523 |
| walker |  | 4790 | 0 | python decl at peepdb/config.py:178 |  |  | 0.523 |
| walker |  | 4790 | 0 | python decl at peepdb/config.py:196 |  |  | 0.523 |
| walker |  | 4798 | 8 | python decl at peepdb/config.py:15 |  |  | 0.526 |
| walker |  | 4809 | 11 | python decl at peepdb/config.py:41 |  |  | 0.531 |
| walker |  | 4821 | 12 | python decl at peepdb/config.py:29 |  |  | 0.537 |
| walker |  | 4836 | 15 | python decl body at peepdb/config.py:70 body 71 |  |  | 0.537 |
| walker |  | 4851 | 15 | python decl body at peepdb/config.py:73 body 74 |  |  | 0.537 |
| walker |  | 4873 | 22 | python class body at peepdb/config.py:15 |  |  | 0.537 |
| walker |  | 4907 | 34 | README.md section #20 |  |  | 0.537 |
| walker |  | 4958 | 51 | docs/README.md section #9 |  |  | 0.537 |
| ns | 4966 |  | 505 | Every class and method name across all eight backend modules | 4.1 | 3.1 | 0.506 |
| walker |  | 5076 | 118 | python decl names surface in peepdb/core.py |  |  | 0.520 |
| walker |  | 5076 | 0 | python decl at peepdb/core.py:27 |  |  | 0.520 |
| walker |  | 5076 | 0 | python decl at peepdb/core.py:101 |  |  | 0.520 |
| walker |  | 5076 | 0 | python decl at peepdb/core.py:118 |  |  | 0.520 |
| walker |  | 5185 | 109 | python decl at peepdb/core.py:56 |  |  | 0.542 |
| walker |  | 5224 | 39 | README.md section #23 |  |  | 0.542 |
| walker |  | 5261 | 37 | README.md section #24 |  |  | 0.542 |
| ns | 5265 |  | 299 | The table/collection-listing query of every backend | 4.2 | 4.1 | 0.526 |
| walker |  | 5266 | 5 | python method body at peepdb/db/base.py:29 body 31 |  |  | 0.528 |
| walker |  | 5302 | 36 | docs/installation.md section #4 |  |  | 0.528 |
| walker |  | 5456 | 154 | python decl at peepdb/cli.py:126 |  |  | 0.546 |
| walker |  | 5508 | 52 | python decl doc at peepdb/cli.py:126 |  |  | 0.548 |
| walker |  | 5531 | 23 | docs/README.md section #5 |  |  | 0.548 |
| ns | 5597 |  | 332 | MySQLDatabase — the canonical backend implementation | 4.3 | 4.1 | 0.528 |
| walker |  | 5655 | 124 | README.md section #27 |  |  | 0.528 |
| walker |  | 5700 | 45 | README.md section #12 |  |  | 0.528 |
| walker |  | 5730 | 30 | python imports in peepdb/db/base.py |  |  | 0.531 |
| walker |  | 5771 | 41 | docs/usage.md section #2 |  |  | 0.531 |
| walker |  | 5805 | 34 | python imports in peepdb/db/mariadb.py |  |  | 0.531 |
| walker |  | 5816 | 11 | python decl names surface in peepdb/db/mariadb.py |  |  | 0.531 |
| walker |  | 5816 | 0 | python decl at peepdb/db/mariadb.py:5 |  |  | 0.531 |
| walker |  | 5895 | 79 | python method sigs in peepdb/db/mariadb.py |  |  | 0.532 |
| walker |  | 5895 | 0 | python method at peepdb/db/mariadb.py:6 |  |  | 0.532 |
| walker |  | 5895 | 0 | python method at peepdb/db/mariadb.py:23 |  |  | 0.532 |
| walker |  | 5895 | 0 | python method at peepdb/db/mariadb.py:30 |  |  | 0.532 |
| walker |  | 5895 | 0 | python method at peepdb/db/mariadb.py:34 |  |  | 0.532 |
| ns | 5899 |  | 302 | PostgreSQL, MariaDB and Oracle connect() — drivers, default ports, cursor factories | 4.4 | 4.1 | 0.514 |
| walker |  | 5929 | 34 | python imports in peepdb/db/mysql.py |  |  | 0.514 |
| walker |  | 5940 | 11 | python decl names surface in peepdb/db/mysql.py |  |  | 0.515 |
| walker |  | 5940 | 0 | python decl at peepdb/db/mysql.py:5 |  |  | 0.515 |
| walker |  | 6019 | 79 | python method sigs in peepdb/db/mysql.py |  |  | 0.517 |
| walker |  | 6019 | 0 | python method at peepdb/db/mysql.py:6 |  |  | 0.517 |
| walker |  | 6019 | 0 | python method at peepdb/db/mysql.py:22 |  |  | 0.517 |
| walker |  | 6019 | 0 | python method at peepdb/db/mysql.py:29 |  |  | 0.517 |
| walker |  | 6019 | 0 | python method at peepdb/db/mysql.py:33 |  |  | 0.517 |
| walker |  | 6069 | 50 | README.md section #15 |  |  | 0.517 |
| walker |  | 6083 | 14 | python decl names surface in peepdb/db/oracle.py |  |  | 0.518 |
| walker |  | 6083 | 0 | python decl at peepdb/db/oracle.py:7 |  |  | 0.518 |
| walker |  | 6136 | 53 | python method sigs in peepdb/db/oracle.py |  |  | 0.521 |
| walker |  | 6136 | 0 | python method at peepdb/db/oracle.py:8 |  |  | 0.521 |
| walker |  | 6136 | 0 | python method at peepdb/db/oracle.py:25 |  |  | 0.521 |
| walker |  | 6136 | 0 | python method at peepdb/db/oracle.py:32 |  |  | 0.521 |
| ns | 6158 |  | 259 | MongoDBDatabase.connect — URI construction from the generic parameters | 4.5 | 4.1 | 0.509 |
| walker |  | 6175 | 39 | python imports in peepdb/db/oracle.py |  |  | 0.509 |
| walker |  | 6189 | 14 | python decl names surface in peepdb/db/sqlite.py |  |  | 0.511 |
| walker |  | 6189 | 0 | python decl at peepdb/db/sqlite.py:6 |  |  | 0.511 |
| walker |  | 6268 | 79 | python method sigs in peepdb/db/sqlite.py |  |  | 0.515 |
| walker |  | 6268 | 0 | python method at peepdb/db/sqlite.py:7 |  |  | 0.515 |
| walker |  | 6268 | 0 | python method at peepdb/db/sqlite.py:22 |  |  | 0.515 |
| walker |  | 6268 | 0 | python method at peepdb/db/sqlite.py:29 |  |  | 0.515 |
| walker |  | 6268 | 0 | python method at peepdb/db/sqlite.py:43 |  |  | 0.515 |
| walker |  | 6305 | 37 | python imports in peepdb/db/sqlite.py |  |  | 0.515 |
| walker |  | 6311 | 6 | python method body at peepdb/db/base.py:37 body 38 |  |  | 0.518 |
| walker |  | 6361 | 50 | docs/index.md section #10 |  |  | 0.518 |
| walker |  | 6403 | 42 | python imports in peepdb/db/mssql.py |  |  | 0.518 |
| walker |  | 6414 | 11 | python decl names surface in peepdb/db/mssql.py |  |  | 0.520 |
| walker |  | 6414 | 0 | python decl at peepdb/db/mssql.py:6 |  |  | 0.520 |
| ns | 6437 |  | 279 | MSSQLDatabase: constructor extras and the ODBC connection string | 4.6 | 4.1 | 0.510 |
| walker |  | 6539 | 125 | python method sigs in peepdb/db/mssql.py |  |  | 0.521 |
| walker |  | 6539 | 0 | python method at peepdb/db/mssql.py:7 |  |  | 0.521 |
| walker |  | 6539 | 0 | python method at peepdb/db/mssql.py:12 |  |  | 0.521 |
| walker |  | 6539 | 0 | python method at peepdb/db/mssql.py:28 |  |  | 0.521 |
| walker |  | 6539 | 0 | python method at peepdb/db/mssql.py:45 |  |  | 0.521 |
| walker |  | 6552 | 13 | python method at peepdb/db/mssql.py:35 |  |  | 0.521 |
| walker |  | 6567 | 15 | python decl names surface in peepdb/db/mongodb.py |  |  | 0.524 |
| walker |  | 6567 | 0 | python decl at peepdb/db/mongodb.py:8 |  |  | 0.524 |
| ns | 6584 |  | 147 | FirebaseDatabase: the backend that does not call super().__init__ | 4.7 | 4.1 | 0.517 |
| walker |  | 6621 | 54 | python method sigs in peepdb/db/mongodb.py |  |  | 0.525 |
| walker |  | 6621 | 0 | python method at peepdb/db/mongodb.py:9 |  |  | 0.525 |
| walker |  | 6621 | 0 | python method at peepdb/db/mongodb.py:32 |  |  | 0.525 |
| walker |  | 6621 | 0 | python method at peepdb/db/mongodb.py:39 |  |  | 0.525 |
| walker |  | 6631 | 10 | python method body at peepdb/db/mongodb.py:39 body 40 |  |  | 0.525 |
| walker |  | 6646 | 15 | python decl names surface in peepdb/db/postgresql.py |  |  | 0.528 |
| walker |  | 6646 | 0 | python decl at peepdb/db/postgresql.py:6 |  |  | 0.528 |
| walker |  | 6660 | 14 | python method body at peepdb/db/base.py:33 body 34 |  |  | 0.533 |
| walker |  | 6711 | 51 | python decl body at peepdb/cli.py:97 body 107 |  |  | 0.541 |
| walker |  | 6722 | 11 | python decl body at peepdb/cli.py:113 body 122 |  |  | 0.544 |
| walker |  | 6801 | 79 | python method sigs in peepdb/db/postgresql.py |  |  | 0.552 |
| walker |  | 6801 | 0 | python method at peepdb/db/postgresql.py:7 |  |  | 0.552 |
| walker |  | 6801 | 0 | python method at peepdb/db/postgresql.py:23 |  |  | 0.552 |
| walker |  | 6801 | 0 | python method at peepdb/db/postgresql.py:30 |  |  | 0.552 |
| walker |  | 6801 | 0 | python method at peepdb/db/postgresql.py:34 |  |  | 0.552 |
| walker |  | 6817 | 16 | python decl body at peepdb/cli.py:113 body 123 |  |  | 0.557 |
| walker |  | 6894 | 77 | python decl body at peepdb/config.py:41 body 43 |  |  | 0.557 |
| ns | 6901 |  | 317 | SQLiteDatabase connect and fetch_data — file-path handling and print-based logging | 4.8 | 4.1 | 0.545 |
| walker |  | 6921 | 27 | docs/README.md section #12 |  |  | 0.545 |
| walker |  | 6976 | 55 | docs/usage.md section #7 |  |  | 0.545 |
| walker |  | 7022 | 46 | python imports in peepdb/db/mongodb.py |  |  | 0.545 |
| walker |  | 7053 | 31 | python decl names surface in peepdb/db/firebase.py |  |  | 0.548 |
| walker |  | 7053 | 0 | python decl at peepdb/db/firebase.py:9 |  |  | 0.548 |
| walker |  | 7149 | 96 | python method sigs in peepdb/db/firebase.py |  |  | 0.561 |
| walker |  | 7149 | 0 | python method at peepdb/db/firebase.py:10 |  |  | 0.561 |
| walker |  | 7149 | 0 | python method at peepdb/db/firebase.py:15 |  |  | 0.561 |
| walker |  | 7149 | 0 | python method at peepdb/db/firebase.py:30 |  |  | 0.561 |
| walker |  | 7149 | 0 | python method at peepdb/db/firebase.py:39 |  |  | 0.561 |
| walker |  | 7165 | 16 | python method at peepdb/db/firebase.py:26 |  |  | 0.561 |
| walker |  | 7170 | 5 | python method body at peepdb/db/firebase.py:26 body 28 |  |  | 0.561 |
| ns | 7203 |  | 302 | The non-LIMIT/OFFSET pagination variants (MSSQL, Oracle, MongoDB, Firebase) | 4.9 | 4.1 | 0.550 |
| walker |  | 7234 | 64 | README.md section #14 |  |  | 0.550 |
| walker |  | 7282 | 48 | python imports in peepdb/db/postgresql.py |  |  | 0.550 |
| ns | 7312 |  | 109 | Where credentials live: config paths and KeySecurity modes | 5.1 | 1.9 | 0.555 |
| ns | 7497 |  | 185 | get_key_security_config / get_key / encrypt / decrypt bodies | 5.2 | 1.9 | 0.548 |
| walker |  | 7530 | 248 | python decl at peepdb/cli.py:53 |  |  | 0.570 |
| walker |  | 7596 | 66 | python decl doc at peepdb/cli.py:53 |  |  | 0.572 |
| walker |  | 7645 | 49 | python imports in peepdb/db/firebase.py |  |  | 0.572 |
| walker |  | 7664 | 19 | docs/index.md section #5 |  |  | 0.572 |
| walker |  | 7695 | 31 | python method body at peepdb/db/mariadb.py:30 body 31 |  |  | 0.573 |
| walker |  | 7727 | 32 | python method body at peepdb/db/oracle.py:32 body 33 |  |  | 0.574 |
| ns | 7744 |  | 247 | The two key sources: PBKDF2 derivation and keyring fetch-or-create | 5.3 | 1.9 | 0.570 |
| walker |  | 7826 | 99 | docs/README.md section #10 |  |  | 0.570 |
| walker |  | 7835 | 9 | python decl body at peepdb/core.py:101 body 102 |  |  | 0.570 |
| walker |  | 7889 | 54 | python method body at peepdb/cli.py:17 body 18 |  |  | 0.570 |
| walker |  | 7953 | 64 | docs/usage.md section #4 |  |  | 0.570 |
| ns | 7989 |  | 245 | save_connection: the on-disk record shape and what is encrypted | 5.4 | 1.9 | 0.559 |
| walker |  | 8017 | 64 | docs/usage.md section #5 |  |  | 0.559 |
| walker |  | 8122 | 105 | python decl body at peepdb/config.py:60 body 61 |  |  | 0.568 |
| walker |  | 8155 | 33 | python method body at peepdb/db/firebase.py:10 body 11 |  |  | 0.569 |
| ns | 8169 |  | 180 | get_connection: the tuple contract and InvalidPassword | 5.5 | 1.9 | 0.560 |
| walker |  | 8176 | 21 | docs/index.md section #7 |  |  | 0.560 |
| walker |  | 8196 | 20 | docs/index.md section #6 |  |  | 0.560 |
| walker |  | 8304 | 108 | python decl body at peepdb/config.py:49 body 50 |  |  | 0.567 |
| ns | 8311 |  | 142 | add_key_security: first-run choice between keyring and password | 5.6 | 1.9 | 0.563 |
| ns | 8405 |  | 94 | list/remove/remove_all_connections — the distinctive lines only | 5.7 | 1.9 | 0.558 |
| walker |  | 8426 | 122 | docs/README.md section #7 |  |  | 0.558 |
| walker |  | 8493 | 67 | docs/installation.md section #2 |  |  | 0.558 |
| walker |  | 8527 | 34 | python method body at peepdb/db/mysql.py:29 body 30 |  |  | 0.560 |
| walker |  | 8642 | 115 | python imports in peepdb/cli.py |  |  | 0.560 |
| walker |  | 8665 | 23 | docs/index.md section #3 |  |  | 0.560 |
| walker |  | 8687 | 22 | docs/index.md section #4 |  |  | 0.560 |
| walker |  | 8710 | 23 | docs/index.md section #2 |  |  | 0.560 |
| walker |  | 8748 | 38 | python method at peepdb/db/oracle.py:36 |  |  | 0.560 |
| ns | 8762 |  | 357 | Every test (and fixture) name in peepdb/tests, across all six modules | 6.1 | 1.11 | 0.548 |
| ns | 8868 |  | 106 | How the suite is run: CONTRIBUTING instructions and the CI invocation | 6.2 |  | 0.544 |
| walker |  | 9025 | 277 | python decl doc at peepdb/cli.py:25 |  |  | 0.545 |
| ns | 9083 |  | 215 | A representative test body: patching style and asserted output strings | 6.3 | 6.1 | 0.539 |
| walker |  | 9162 | 137 | python imports in peepdb/core.py |  |  | 0.545 |
| ns | 9173 |  | 90 | `peepdb --help` summary lines from the cli group docstring | 7.1 | 1.6 | 0.548 |
| walker |  | 9235 | 73 | README.md section #26 |  |  | 0.548 |
| walker |  | 9246 | 11 | python decl body at peepdb/core.py:118 body 119 |  |  | 0.548 |
| walker |  | 9386 | 140 | python imports in peepdb/config.py |  |  | 0.548 |
| walker |  | 9426 | 40 | python method body at peepdb/db/postgresql.py:30 body 31 |  |  | 0.551 |
| ns | 9429 |  | 256 | The two disagreeing dependency lists (setup.py vs project.toml) | 7.2 |  | 0.562 |
| walker |  | 9556 | 130 | python decl body at peepdb/config.py:29 body 31 |  |  | 0.574 |
| ns | 9566 |  | 137 | Build backend, Python floor, packaging includes | 7.3 | 1.1 | 0.576 |
| walker |  | 9633 | 77 | README.md section #17 |  |  | 0.576 |
| ns | 9754 |  | 188 | CI: what triggers the workflows and on what Python versions | 7.4 |  | 0.577 |
| walker |  | 9767 | 134 | python decl body at peepdb/config.py:161 body 162 |  |  | 0.577 |
| walker |  | 9855 | 88 | docs/index.md section #8 |  |  | 0.577 |
| walker |  | 9900 | 45 | python method body at peepdb/db/mssql.py:7 body 8 |  |  | 0.578 |
| ns | 9915 |  | 161 | The docs/ site: Jekyll theme config, and two stale-template markers | 7.5 | 1.11 | 0.577 |
| walker |  | 9946 | 46 | docs/README.md section #11 |  |  | 0.577 |
| ns | 9980 |  | 65 | CustomEncoder.default — the JSON serializer for Decimal and date | 7.6 | 1.6 | 0.580 |
| walker |  | 9994 | 48 | python method body at peepdb/db/mongodb.py:32 body 33 |  |  | 0.580 |

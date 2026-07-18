Score(3000)=0.617 I=0.809 C=0.470 ns_rows≤3K=19/53 (reached=9 partial=1 missing=9)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 38 | 38 | listing of '.' |  |  | 1.000 |
| ns | 38 |  | 38 | Root directory listing | 1.1 |  | 1.000 |
| walker |  | 67 | 29 | entry-point scripts in project.toml |  |  | 1.000 |
| walker |  | 70 | 3 | listing of '.github' |  |  | 1.000 |
| ns | 72 |  | 34 | peepdb/ package listing | 1.2 |  | 0.720 |
| walker |  | 78 | 8 | listing of '.github/workflows' |  |  | 0.724 |
| ns | 117 |  | 45 | peepdb/db/ backend listing | 1.3 |  | 0.561 |
| ns | 159 |  | 42 | peepdb/tests/ listing | 1.4 |  | 0.493 |
| walker |  | 175 | 97 | README headline in README.md |  |  | 0.497 |
| ns | 200 |  | 41 | docs/ listing (Jekyll site) | 1.5 |  | 0.429 |
| walker |  | 203 | 28 | listing of 'images' |  |  | 0.434 |
| ns | 208 |  | 8 | .github/workflows/ listing | 1.6 |  | 0.457 |
| ns | 236 |  | 28 | images/ listing | 1.7 |  | 0.494 |
| walker |  | 237 | 34 | listing of 'peepdb' |  |  | 0.637 |
| walker |  | 278 | 41 | listing of 'docs' |  |  | 0.773 |
| walker |  | 323 | 45 | listing of 'peepdb/db' |  |  | 0.916 |
| ns | 333 |  | 97 | README title + one-line description | 1.8 |  | 0.914 |
| walker |  | 419 | 96 | README headline in docs/README.md |  |  | 0.914 |
| ns | 512 |  | 179 | README feature list | 1.9 |  | 0.835 |
| ns | 693 |  | 181 | MANIFEST.in + .gitignore | 2.1 |  | 0.687 |
| ns | 978 |  | 285 | Installation: pip install + system deps + verification | 2.2 |  | 0.588 |
| walker |  | 1012 | 593 | YAML config at .github/workflows/test.yml |  |  | 0.600 |
| walker |  | 1023 | 11 | python decl names surface in peepdb/exceptions.py |  |  | 0.600 |
| walker |  | 1023 | 0 | python decl at peepdb/exceptions.py:1 |  |  | 0.600 |
| walker |  | 1028 | 5 | python class body at peepdb/exceptions.py:1 |  |  | 0.600 |
| walker |  | 1042 | 14 | python decl names surface in peepdb/db/base.py |  |  | 0.600 |
| walker |  | 1042 | 0 | python decl at peepdb/db/base.py:5 |  |  | 0.600 |
| walker |  | 1054 | 12 | python imports in peepdb/__main__.py |  |  | 0.600 |
| walker |  | 1068 | 14 | python imports in setup.py |  |  | 0.600 |
| walker |  | 1122 | 54 | headings outline in docs/installation.md |  |  | 0.602 |
| ns | 1150 |  | 172 | project.toml — build system + project identity | 2.3 |  | 0.563 |
| walker |  | 1178 | 56 | headings outline in docs/index.md |  |  | 0.563 |
| walker |  | 1393 | 215 | python imports in peepdb/db/__init__.py |  |  | 0.565 |
| walker |  | 1467 | 74 | [package] in project.toml |  |  | 0.579 |
| ns | 1476 |  | 326 | project.toml — dependencies, entry point, packaging | 2.4 |  | 0.517 |
| walker |  | 1660 | 193 | headings outline in README.md |  |  | 0.520 |
| ns | 1663 |  | 187 | setup.py — where it diverges from project.toml | 2.5 |  | 0.496 |
| walker |  | 1695 | 35 | README.md section #8 |  |  | 0.496 |
| walker |  | 1725 | 30 | README.md section #11 |  |  | 0.496 |
| ns | 1738 |  | 75 | requirements.txt (pinned freeze, sampled) | 2.6 |  | 0.490 |
| walker |  | 1753 | 28 | README.md section #30 |  |  | 0.490 |
| walker |  | 1772 | 19 | README.md section #5 |  |  | 0.491 |
| walker |  | 1802 | 30 | README.md section #29 |  |  | 0.491 |
| walker |  | 1823 | 21 | README.md section #7 |  |  | 0.495 |
| walker |  | 1843 | 20 | README.md section #6 |  |  | 0.500 |
| walker |  | 1878 | 35 | README.md section #28 |  |  | 0.500 |
| walker |  | 1892 | 14 | python decl names surface in peepdb/db/oracle.py |  |  | 0.500 |
| walker |  | 1892 | 0 | python decl at peepdb/db/oracle.py:7 |  |  | 0.500 |
| walker |  | 1906 | 14 | python decl names surface in peepdb/db/sqlite.py |  |  | 0.500 |
| walker |  | 1906 | 0 | python decl at peepdb/db/sqlite.py:6 |  |  | 0.500 |
| walker |  | 2003 | 97 | headings outline in docs/usage.md |  |  | 0.500 |
| walker |  | 2026 | 23 | README.md section #3 |  |  | 0.506 |
| walker |  | 2048 | 22 | README.md section #4 |  |  | 0.515 |
| walker |  | 2071 | 23 | README.md section #2 |  |  | 0.525 |
| ns | 2089 |  | 351 | CI test job + publish-job trigger gating (test.yml) | 2.7 |  | 0.598 |
| walker |  | 2117 | 46 | README.md section #10 |  |  | 0.605 |
| walker |  | 2132 | 15 | python decl names surface in peepdb/db/mariadb.py |  |  | 0.605 |
| walker |  | 2132 | 0 | python decl at peepdb/db/mariadb.py:5 |  |  | 0.605 |
| ns | 2137 |  | 48 | cli.py: all 5 command names | 3.1 |  | 0.597 |
| walker |  | 2147 | 15 | python decl names surface in peepdb/db/mongodb.py |  |  | 0.597 |
| walker |  | 2147 | 0 | python decl at peepdb/db/mongodb.py:8 |  |  | 0.597 |
| walker |  | 2162 | 15 | python decl names surface in peepdb/db/mssql.py |  |  | 0.597 |
| walker |  | 2162 | 0 | python decl at peepdb/db/mssql.py:6 |  |  | 0.597 |
| walker |  | 2177 | 15 | python decl names surface in peepdb/db/mysql.py |  |  | 0.597 |
| walker |  | 2177 | 0 | python decl at peepdb/db/mysql.py:5 |  |  | 0.597 |
| walker |  | 2192 | 15 | python decl names surface in peepdb/db/postgresql.py |  |  | 0.597 |
| walker |  | 2192 | 0 | python decl at peepdb/db/postgresql.py:6 |  |  | 0.597 |
| ns | 2337 |  | 200 | cli.py: imports + CustomEncoder | 3.2 |  | 0.564 |
| walker |  | 2384 | 192 | [dependencies] in project.toml |  |  | 0.600 |
| walker |  | 2511 | 127 | manifest config in project.toml |  |  | 0.654 |
| walker |  | 2567 | 56 | README.md section #9 |  |  | 0.654 |
| walker |  | 2599 | 32 | docs/usage.md section #0 |  |  | 0.654 |
| ns | 2647 |  | 310 | cli.py: `cli` group docstring | 3.3 |  | 0.616 |
| walker |  | 2751 | 152 | python method sigs in peepdb/db/base.py |  |  | 0.616 |
| walker |  | 2751 | 0 | python method at peepdb/db/base.py:6 |  |  | 0.616 |
| walker |  | 2751 | 0 | python method at peepdb/db/base.py:33 |  |  | 0.616 |
| walker |  | 2751 | 0 | python method at peepdb/db/base.py:37 |  |  | 0.616 |
| walker |  | 2760 | 9 | python method at peepdb/db/base.py:17 |  |  | 0.616 |
| walker |  | 2769 | 9 | python method at peepdb/db/base.py:21 |  |  | 0.616 |
| walker |  | 2778 | 9 | python method at peepdb/db/base.py:25 |  |  | 0.617 |
| walker |  | 2787 | 9 | python method at peepdb/db/base.py:29 |  |  | 0.617 |
| walker |  | 2941 | 154 | headings outline in docs/README.md |  |  | 0.617 |
| walker |  | 2961 | 20 | docs/README.md section #2 |  |  | 0.617 |
| walker |  | 2979 | 18 | docs/README.md section #8 |  |  | 0.617 |
| walker |  | 3063 | 84 | python decl names surface in peepdb/cli.py |  |  | 0.617 |
| walker |  | 3063 | 0 | python decl at peepdb/cli.py:16 |  |  | 0.617 |
| walker |  | 3063 | 0 | python decl at peepdb/cli.py:181 |  |  | 0.617 |
| walker |  | 3070 | 7 | python decl at peepdb/cli.py:86 |  |  | 0.618 |
| walker |  | 3081 | 11 | python method sigs in peepdb/cli.py |  |  | 0.618 |
| walker |  | 3081 | 0 | python method at peepdb/cli.py:17 |  |  | 0.618 |
| walker |  | 3097 | 16 | python decl at peepdb/cli.py:25 |  |  | 0.618 |
| walker |  | 3104 | 7 | python decl body at peepdb/cli.py:25 body 50 |  |  | 0.619 |
| walker |  | 3111 | 7 | python decl body at peepdb/cli.py:181 body 182 |  |  | 0.619 |
| ns | 3114 |  | 467 | cli.py: `save` command | 3.4 | 3.1 | 0.580 |
| walker |  | 3142 | 31 | python decl at peepdb/cli.py:113 |  |  | 0.583 |
| ns | 3171 |  | 57 | cli.py: `list` command | 3.5 | 3.1 | 0.574 |
| walker |  | 3184 | 42 | python decl at peepdb/cli.py:97 |  |  | 0.577 |
| walker |  | 3192 | 8 | python decl body at peepdb/cli.py:86 body 94 |  |  | 0.579 |
| walker |  | 3233 | 41 | python decl doc at peepdb/cli.py:86 |  |  | 0.597 |
| walker |  | 3238 | 5 | python method body at peepdb/db/base.py:17 body 19 |  |  | 0.597 |
| walker |  | 3273 | 35 | README.md section #1 |  |  | 0.605 |
| ns | 3312 |  | 141 | cli.py: `remove` command | 3.6 | 3.1 | 0.593 |
| ns | 3415 |  | 103 | cli.py: `remove-all` command | 3.7 | 3.1 | 0.584 |
| walker |  | 3458 | 185 | python setup manifest at setup.py:16 |  |  | 0.615 |
| walker |  | 3503 | 45 | python decl doc at peepdb/cli.py:113 |  |  | 0.625 |
| walker |  | 3561 | 58 | docs/README.md section #1 |  |  | 0.625 |
| walker |  | 3608 | 47 | python decl doc at peepdb/cli.py:97 |  |  | 0.634 |
| walker |  | 3661 | 53 | python method sigs in peepdb/db/oracle.py |  |  | 0.634 |
| walker |  | 3661 | 0 | python method at peepdb/db/oracle.py:8 |  |  | 0.634 |
| walker |  | 3661 | 0 | python method at peepdb/db/oracle.py:25 |  |  | 0.634 |
| walker |  | 3661 | 0 | python method at peepdb/db/oracle.py:32 |  |  | 0.634 |
| walker |  | 3715 | 54 | python method sigs in peepdb/db/mongodb.py |  |  | 0.634 |
| walker |  | 3715 | 0 | python method at peepdb/db/mongodb.py:9 |  |  | 0.634 |
| walker |  | 3715 | 0 | python method at peepdb/db/mongodb.py:32 |  |  | 0.634 |
| walker |  | 3715 | 0 | python method at peepdb/db/mongodb.py:39 |  |  | 0.634 |
| walker |  | 3725 | 10 | python method body at peepdb/db/mongodb.py:39 body 40 |  |  | 0.634 |
| walker |  | 3748 | 23 | README.md section #16 |  |  | 0.634 |
| ns | 3805 |  | 390 | cli.py: `view` — options + connection resolution | 3.8 | 3.1 | 0.604 |
| walker |  | 3858 | 110 | plaintext config requirements.txt |  |  | 0.605 |
| walker |  | 4073 | 215 | docs/README.md section #0 |  |  | 0.605 |
| ns | 4092 |  | 287 | cli.py: `view` — peep_db call + output rendering | 3.9 |  | 0.584 |
| walker |  | 4115 | 42 | listing of 'peepdb/tests' |  |  | 0.608 |
| ns | 4133 |  | 41 | cli.py: `main()` entry point | 3.10 |  | 0.604 |
| walker |  | 4146 | 31 | python decl names surface in peepdb/db/firebase.py |  |  | 0.604 |
| walker |  | 4146 | 0 | python decl at peepdb/db/firebase.py:9 |  |  | 0.604 |
| walker |  | 4151 | 5 | python method body at peepdb/db/base.py:21 body 23 |  |  | 0.604 |
| ns | 4178 |  | 45 | core.py: all 4 top-level function names | 4.1 |  | 0.600 |
| ns | 4384 |  | 206 | core.py: imports + module logger | 4.2 |  | 0.580 |
| walker |  | 4443 | 292 | python decl names surface in peepdb/config.py |  |  | 0.581 |
| walker |  | 4443 | 0 | python decl at peepdb/config.py:49 |  |  | 0.581 |
| walker |  | 4443 | 0 | python decl at peepdb/config.py:60 |  |  | 0.581 |
| walker |  | 4443 | 0 | python decl at peepdb/config.py:70 |  |  | 0.581 |
| walker |  | 4443 | 0 | python decl at peepdb/config.py:73 |  |  | 0.581 |
| walker |  | 4443 | 0 | python decl at peepdb/config.py:76 |  |  | 0.581 |
| walker |  | 4443 | 0 | python decl at peepdb/config.py:112 |  |  | 0.581 |
| walker |  | 4443 | 0 | python decl at peepdb/config.py:144 |  |  | 0.581 |
| walker |  | 4443 | 0 | python decl at peepdb/config.py:161 |  |  | 0.581 |
| walker |  | 4443 | 0 | python decl at peepdb/config.py:178 |  |  | 0.581 |
| walker |  | 4443 | 0 | python decl at peepdb/config.py:196 |  |  | 0.581 |
| walker |  | 4451 | 8 | python decl at peepdb/config.py:15 |  |  | 0.581 |
| walker |  | 4462 | 11 | python decl at peepdb/config.py:41 |  |  | 0.582 |
| walker |  | 4474 | 12 | python decl at peepdb/config.py:29 |  |  | 0.582 |
| walker |  | 4496 | 22 | python class body at peepdb/config.py:15 |  |  | 0.582 |
| walker |  | 4511 | 15 | python decl body at peepdb/config.py:70 body 71 |  |  | 0.582 |
| walker |  | 4526 | 15 | python decl body at peepdb/config.py:73 body 74 |  |  | 0.582 |
| walker |  | 4531 | 5 | python method body at peepdb/db/base.py:25 body 27 |  |  | 0.582 |
| walker |  | 4536 | 5 | python method body at peepdb/db/base.py:29 body 31 |  |  | 0.582 |
| walker |  | 4542 | 6 | python method body at peepdb/db/base.py:37 body 38 |  |  | 0.582 |
| walker |  | 4621 | 79 | python method sigs in peepdb/db/mariadb.py |  |  | 0.582 |
| walker |  | 4621 | 0 | python method at peepdb/db/mariadb.py:6 |  |  | 0.582 |
| walker |  | 4621 | 0 | python method at peepdb/db/mariadb.py:23 |  |  | 0.582 |
| walker |  | 4621 | 0 | python method at peepdb/db/mariadb.py:30 |  |  | 0.582 |
| walker |  | 4621 | 0 | python method at peepdb/db/mariadb.py:34 |  |  | 0.582 |
| walker |  | 4700 | 79 | python method sigs in peepdb/db/mysql.py |  |  | 0.582 |
| walker |  | 4700 | 0 | python method at peepdb/db/mysql.py:6 |  |  | 0.582 |
| walker |  | 4700 | 0 | python method at peepdb/db/mysql.py:22 |  |  | 0.582 |
| walker |  | 4700 | 0 | python method at peepdb/db/mysql.py:29 |  |  | 0.582 |
| walker |  | 4700 | 0 | python method at peepdb/db/mysql.py:33 |  |  | 0.582 |
| walker |  | 4779 | 79 | python method sigs in peepdb/db/postgresql.py |  |  | 0.582 |
| walker |  | 4779 | 0 | python method at peepdb/db/postgresql.py:7 |  |  | 0.582 |
| walker |  | 4779 | 0 | python method at peepdb/db/postgresql.py:23 |  |  | 0.582 |
| walker |  | 4779 | 0 | python method at peepdb/db/postgresql.py:30 |  |  | 0.582 |
| walker |  | 4779 | 0 | python method at peepdb/db/postgresql.py:34 |  |  | 0.582 |
| ns | 4834 |  | 450 | core.py: connect_to_database | 4.3 | 4.1 | 0.562 |
| walker |  | 4858 | 79 | python method sigs in peepdb/db/sqlite.py |  |  | 0.562 |
| walker |  | 4858 | 0 | python method at peepdb/db/sqlite.py:7 |  |  | 0.562 |
| walker |  | 4858 | 0 | python method at peepdb/db/sqlite.py:22 |  |  | 0.562 |
| walker |  | 4858 | 0 | python method at peepdb/db/sqlite.py:29 |  |  | 0.562 |
| walker |  | 4858 | 0 | python method at peepdb/db/sqlite.py:43 |  |  | 0.562 |
| walker |  | 4885 | 27 | docs/installation.md section #1 |  |  | 0.562 |
| walker |  | 4899 | 14 | python method body at peepdb/db/base.py:33 body 34 |  |  | 0.562 |
| walker |  | 4971 | 72 | docs/index.md section #0 |  |  | 0.562 |
| walker |  | 5044 | 73 | docs/installation.md section #0 |  |  | 0.562 |
| ns | 5140 |  | 306 | core.py: peep_db — signature + fetch | 4.4 | 4.1 | 0.546 |
| walker |  | 5162 | 118 | python decl names surface in peepdb/core.py |  |  | 0.553 |
| walker |  | 5162 | 0 | python decl at peepdb/core.py:27 |  |  | 0.553 |
| walker |  | 5162 | 0 | python decl at peepdb/core.py:101 |  |  | 0.553 |
| walker |  | 5162 | 0 | python decl at peepdb/core.py:118 |  |  | 0.553 |
| walker |  | 5271 | 109 | python decl at peepdb/core.py:56 |  |  | 0.560 |
| walker |  | 5304 | 33 | README.md section #18 |  |  | 0.560 |
| ns | 5376 |  | 236 | core.py: peep_db — format branch + disconnect | 4.5 |  | 0.547 |
| walker |  | 5400 | 96 | python method sigs in peepdb/db/firebase.py |  |  | 0.547 |
| walker |  | 5400 | 0 | python method at peepdb/db/firebase.py:10 |  |  | 0.547 |
| walker |  | 5400 | 0 | python method at peepdb/db/firebase.py:15 |  |  | 0.547 |
| walker |  | 5400 | 0 | python method at peepdb/db/firebase.py:30 |  |  | 0.547 |
| walker |  | 5400 | 0 | python method at peepdb/db/firebase.py:39 |  |  | 0.547 |
| walker |  | 5416 | 16 | python method at peepdb/db/firebase.py:26 |  |  | 0.547 |
| walker |  | 5421 | 5 | python method body at peepdb/db/firebase.py:26 body 28 |  |  | 0.547 |
| walker |  | 5472 | 51 | docs/README.md section #9 |  |  | 0.547 |
| walker |  | 5523 | 51 | python decl body at peepdb/cli.py:97 body 107 |  |  | 0.561 |
| walker |  | 5559 | 36 | README.md section #20 |  |  | 0.561 |
| walker |  | 5570 | 11 | python decl body at peepdb/cli.py:113 body 122 |  |  | 0.564 |
| ns | 5586 |  | 210 | core.py: format_as_table | 4.6 | 4.1 | 0.554 |
| walker |  | 5608 | 38 | README.md section #19 |  |  | 0.554 |
| walker |  | 5647 | 39 | README.md section #23 |  |  | 0.554 |
| walker |  | 5684 | 37 | README.md section #24 |  |  | 0.554 |
| walker |  | 5720 | 36 | docs/installation.md section #4 |  |  | 0.554 |
| ns | 5843 |  | 257 | core.py: format_value | 4.7 | 4.1 | 0.540 |
| walker |  | 5874 | 154 | python decl at peepdb/cli.py:126 |  |  | 0.548 |
| walker |  | 5926 | 52 | python decl doc at peepdb/cli.py:126 |  |  | 0.554 |
| walker |  | 5949 | 23 | docs/README.md section #5 |  |  | 0.554 |
| ns | 5972 |  | 129 | config.py: all 12 top-level function names | 5.1 |  | 0.563 |
| walker |  | 6073 | 124 | README.md section #27 |  |  | 0.564 |
| ns | 6109 |  | 137 | README security section | 5.2 |  | 0.569 |
| walker |  | 6198 | 125 | python method sigs in peepdb/db/mssql.py |  |  | 0.569 |
| walker |  | 6198 | 0 | python method at peepdb/db/mssql.py:7 |  |  | 0.569 |
| walker |  | 6198 | 0 | python method at peepdb/db/mssql.py:12 |  |  | 0.569 |
| walker |  | 6198 | 0 | python method at peepdb/db/mssql.py:28 |  |  | 0.569 |
| walker |  | 6198 | 0 | python method at peepdb/db/mssql.py:45 |  |  | 0.569 |
| walker |  | 6211 | 13 | python method at peepdb/db/mssql.py:35 |  |  | 0.569 |
| walker |  | 6241 | 30 | python imports in peepdb/db/base.py |  |  | 0.569 |
| ns | 6269 |  | 160 | config.py: KeySecurity + on-disk paths + module logger | 5.3 |  | 0.573 |
| walker |  | 6271 | 30 | python imports in peepdb/db/mariadb.py |  |  | 0.573 |
| walker |  | 6301 | 30 | python imports in peepdb/db/mysql.py |  |  | 0.573 |
| walker |  | 6342 | 41 | docs/usage.md section #2 |  |  | 0.573 |
| walker |  | 6389 | 47 | README.md section #12 |  |  | 0.573 |
| walker |  | 6439 | 50 | README.md section #15 |  |  | 0.573 |
| walker |  | 6455 | 16 | python decl body at peepdb/cli.py:113 body 123 |  |  | 0.577 |
| walker |  | 6492 | 37 | python imports in peepdb/db/sqlite.py |  |  | 0.577 |
| ns | 6521 |  | 252 | config.py: key derivation (password path + keyring path) | 5.4 | 5.1 | 0.567 |
| walker |  | 6569 | 77 | python decl body at peepdb/config.py:41 body 43 |  |  | 0.572 |
| walker |  | 6607 | 38 | python imports in peepdb/db/mssql.py |  |  | 0.572 |
| walker |  | 6646 | 39 | python imports in peepdb/db/oracle.py |  |  | 0.572 |
| walker |  | 6696 | 50 | docs/index.md section #10 |  |  | 0.572 |
| walker |  | 6723 | 27 | docs/README.md section #12 |  |  | 0.572 |
| ns | 6739 |  | 218 | config.py: get_key_security_config + get_key | 5.5 | 5.1 | 0.561 |
| walker |  | 6778 | 55 | docs/usage.md section #7 |  |  | 0.561 |
| walker |  | 6824 | 46 | python imports in peepdb/db/mongodb.py |  |  | 0.561 |
| walker |  | 6872 | 48 | python imports in peepdb/db/postgresql.py |  |  | 0.561 |
| walker |  | 6938 | 66 | README.md section #14 |  |  | 0.561 |
| ns | 7134 |  | 395 | config.py: save_connection | 5.6 | 5.1 | 0.543 |
| walker |  | 7186 | 248 | python decl at peepdb/cli.py:53 |  |  | 0.556 |
| walker |  | 7252 | 66 | python decl doc at peepdb/cli.py:53 |  |  | 0.563 |
| walker |  | 7301 | 49 | python imports in peepdb/db/firebase.py |  |  | 0.563 |
| walker |  | 7368 | 67 | README.md section #13 |  |  | 0.563 |
| walker |  | 7387 | 19 | docs/index.md section #5 |  |  | 0.563 |
| walker |  | 7418 | 31 | python method body at peepdb/db/mariadb.py:30 body 31 |  |  | 0.563 |
| ns | 7432 |  | 298 | config.py: get_connection | 5.7 | 5.1 | 0.548 |
| walker |  | 7450 | 32 | python method body at peepdb/db/oracle.py:32 body 33 |  |  | 0.548 |
| walker |  | 7549 | 99 | docs/README.md section #10 |  |  | 0.548 |
| walker |  | 7558 | 9 | python decl body at peepdb/core.py:101 body 102 |  |  | 0.548 |
| ns | 7586 |  | 154 | config.py: list_connections | 5.8 | 5.1 | 0.541 |
| walker |  | 7612 | 54 | python method body at peepdb/cli.py:17 body 18 |  |  | 0.544 |
| walker |  | 7676 | 64 | docs/usage.md section #4 |  |  | 0.544 |
| walker |  | 7740 | 64 | docs/usage.md section #5 |  |  | 0.544 |
| ns | 7801 |  | 215 | db/__init__.py — backend export roster | 6.1 |  | 0.557 |
| walker |  | 7845 | 105 | python decl body at peepdb/config.py:60 body 61 |  |  | 0.563 |
| walker |  | 7878 | 33 | python method body at peepdb/db/firebase.py:10 body 11 |  |  | 0.563 |
| walker |  | 7899 | 21 | docs/index.md section #7 |  |  | 0.563 |
| walker |  | 7919 | 20 | docs/index.md section #6 |  |  | 0.563 |
| walker |  | 8027 | 108 | python decl body at peepdb/config.py:49 body 50 |  |  | 0.578 |
| walker |  | 8149 | 122 | docs/README.md section #7 |  |  | 0.578 |
| ns | 8204 |  | 403 | db/base.py — BaseDatabase ABC contract | 6.2 |  | 0.575 |
| walker |  | 8216 | 67 | docs/installation.md section #2 |  |  | 0.583 |
| walker |  | 8250 | 34 | python method body at peepdb/db/mysql.py:29 body 30 |  |  | 0.583 |
| walker |  | 8365 | 115 | python imports in peepdb/cli.py |  |  | 0.603 |
| walker |  | 8388 | 23 | docs/index.md section #3 |  |  | 0.603 |
| walker |  | 8410 | 22 | docs/index.md section #4 |  |  | 0.603 |
| walker |  | 8433 | 23 | docs/index.md section #2 |  |  | 0.603 |
| ns | 8452 |  | 248 | db/sqlite.py — connect() | 6.3 |  | 0.595 |
| walker |  | 8471 | 38 | python method at peepdb/db/oracle.py:36 |  |  | 0.595 |
| ns | 8592 |  | 140 | db/firebase.py — constructor | 6.4 |  | 0.600 |
| walker |  | 8748 | 277 | python decl doc at peepdb/cli.py:25 |  |  | 0.627 |
| ns | 8763 |  | 171 | db/mongodb.py — connect() URI construction | 6.5 |  | 0.622 |
| ns | 8813 |  | 50 | exceptions.py + __main__.py | 7.1 |  | 0.621 |
| walker |  | 8885 | 137 | python imports in peepdb/core.py |  |  | 0.634 |
| walker |  | 8958 | 73 | README.md section #26 |  |  | 0.634 |
| walker |  | 8969 | 11 | python decl body at peepdb/core.py:118 body 119 |  |  | 0.634 |
| ns | 9075 |  | 262 | Test function locations, 4 of 6 test modules | 8.1 |  | 0.626 |
| walker |  | 9109 | 140 | python imports in peepdb/config.py |  |  | 0.626 |
| walker |  | 9149 | 40 | python method body at peepdb/db/postgresql.py:30 body 31 |  |  | 0.626 |
| walker |  | 9279 | 130 | python decl body at peepdb/config.py:29 body 31 |  |  | 0.637 |
| ns | 9348 |  | 273 | test_mongodb_uri.py (full) | 8.2 |  | 0.627 |
| walker |  | 9356 | 77 | README.md section #17 |  |  | 0.627 |
| walker |  | 9490 | 134 | python decl body at peepdb/config.py:161 body 162 |  |  | 0.627 |
| walker |  | 9578 | 88 | docs/index.md section #8 |  |  | 0.627 |
| walker |  | 9623 | 45 | python method body at peepdb/db/mssql.py:7 body 8 |  |  | 0.627 |
| walker |  | 9669 | 46 | docs/README.md section #11 |  |  | 0.627 |
| ns | 9699 |  | 351 | test_connection.py — unused/dead test helper | 8.3 |  | 0.616 |
| walker |  | 9717 | 48 | python method body at peepdb/db/mongodb.py:32 body 33 |  |  | 0.616 |
| walker |  | 9871 | 154 | python decl body at peepdb/config.py:144 body 145 |  |  | 0.629 |
| ns | 9916 |  | 217 | README example output (table + JSON) | 9.1 |  | 0.621 |
| ns | 9962 |  | 46 | docs/index.md — encryption key storage claim | 9.2 |  | 0.620 |
| walker |  | 9967 | 96 | docs/usage.md section #3 |  |  | 0.620 |
| ns | 9990 |  | 28 | LICENSE identity: GPLv3 | 9.3 |  | 0.618 |

Score(3000)=0.612 I=0.805 C=0.465 ns_rows≤3K=19/53 (reached=8 partial=2 missing=9)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 38 | 38 | listing of '.' |  |  | 1.000 |
| ns | 38 |  | 38 | Root directory listing | 1.1 |  | 1.000 |
| walker |  | 41 | 3 | listing of '.github' |  |  | 1.000 |
| walker |  | 49 | 8 | listing of '.github/workflows' |  |  | 1.000 |
| ns | 72 |  | 34 | peepdb/ package listing | 1.2 |  | 0.724 |
| walker |  | 77 | 28 | listing of 'images' |  |  | 0.733 |
| walker |  | 106 | 29 | entry-point scripts in project.toml |  |  | 0.733 |
| ns | 117 |  | 45 | peepdb/db/ backend listing | 1.3 |  | 0.568 |
| walker |  | 140 | 34 | listing of 'peepdb' |  |  | 0.788 |
| ns | 159 |  | 42 | peepdb/tests/ listing | 1.4 |  | 0.692 |
| walker |  | 181 | 41 | listing of 'docs' |  |  | 0.713 |
| ns | 200 |  | 41 | docs/ listing (Jekyll site) | 1.5 |  | 0.754 |
| ns | 208 |  | 8 | .github/workflows/ listing | 1.6 |  | 0.759 |
| walker |  | 226 | 45 | listing of 'peepdb/db' |  |  | 0.913 |
| ns | 236 |  | 28 | images/ listing | 1.7 |  | 0.910 |
| walker |  | 311 | 85 | README headline in README.md |  |  | 0.914 |
| ns | 333 |  | 97 | README title + one-line description | 1.8 |  | 0.899 |
| ns | 512 |  | 179 | README feature list | 1.9 |  | 0.822 |
| ns | 693 |  | 181 | MANIFEST.in + .gitignore | 2.1 |  | 0.676 |
| walker |  | 904 | 593 | YAML config at .github/workflows/test.yml |  |  | 0.690 |
| walker |  | 915 | 11 | python decl names surface in peepdb/exceptions.py |  |  | 0.690 |
| walker |  | 915 | 0 | python decl at peepdb/exceptions.py:1 |  |  | 0.690 |
| walker |  | 920 | 5 | python class body at peepdb/exceptions.py:1 |  |  | 0.690 |
| walker |  | 934 | 14 | python decl names surface in peepdb/db/base.py |  |  | 0.690 |
| walker |  | 934 | 0 | python decl at peepdb/db/base.py:5 |  |  | 0.690 |
| ns | 978 |  | 285 | Installation: pip install + system deps + verification | 2.2 |  | 0.590 |
| walker |  | 1030 | 96 | README headline in docs/README.md |  |  | 0.590 |
| walker |  | 1042 | 12 | python imports in peepdb/__main__.py |  |  | 0.590 |
| walker |  | 1056 | 14 | python imports in setup.py |  |  | 0.590 |
| walker |  | 1110 | 54 | headings outline in docs/installation.md |  |  | 0.592 |
| ns | 1150 |  | 172 | project.toml — build system + project identity | 2.3 |  | 0.554 |
| walker |  | 1166 | 56 | headings outline in docs/index.md |  |  | 0.554 |
| walker |  | 1381 | 215 | python imports in peepdb/db/__init__.py |  |  | 0.556 |
| walker |  | 1455 | 74 | [package] in project.toml |  |  | 0.570 |
| ns | 1476 |  | 326 | project.toml — dependencies, entry point, packaging | 2.4 |  | 0.509 |
| walker |  | 1648 | 193 | headings outline in README.md |  |  | 0.512 |
| ns | 1663 |  | 187 | setup.py — where it diverges from project.toml | 2.5 |  | 0.489 |
| walker |  | 1690 | 42 | README.md section #19 |  |  | 0.489 |
| walker |  | 1725 | 35 | README.md section #8 |  |  | 0.489 |
| ns | 1738 |  | 75 | requirements.txt (pinned freeze, sampled) | 2.6 |  | 0.482 |
| walker |  | 1755 | 30 | README.md section #11 |  |  | 0.482 |
| walker |  | 1783 | 28 | README.md section #30 |  |  | 0.482 |
| walker |  | 1802 | 19 | README.md section #5 |  |  | 0.484 |
| walker |  | 1832 | 30 | README.md section #29 |  |  | 0.484 |
| walker |  | 1853 | 21 | README.md section #7 |  |  | 0.488 |
| walker |  | 1873 | 20 | README.md section #6 |  |  | 0.492 |
| walker |  | 1908 | 35 | README.md section #28 |  |  | 0.492 |
| walker |  | 1922 | 14 | python decl names surface in peepdb/db/oracle.py |  |  | 0.492 |
| walker |  | 1922 | 0 | python decl at peepdb/db/oracle.py:7 |  |  | 0.492 |
| walker |  | 1936 | 14 | python decl names surface in peepdb/db/sqlite.py |  |  | 0.492 |
| walker |  | 1936 | 0 | python decl at peepdb/db/sqlite.py:6 |  |  | 0.492 |
| walker |  | 2033 | 97 | headings outline in docs/usage.md |  |  | 0.492 |
| walker |  | 2056 | 23 | README.md section #3 |  |  | 0.499 |
| walker |  | 2078 | 22 | README.md section #4 |  |  | 0.508 |
| ns | 2089 |  | 351 | CI test job + publish-job trigger gating (test.yml) | 2.7 |  | 0.585 |
| walker |  | 2101 | 23 | README.md section #2 |  |  | 0.592 |
| ns | 2137 |  | 48 | cli.py: all 5 command names | 3.1 |  | 0.584 |
| walker |  | 2172 | 71 | README.md section #13 |  |  | 0.584 |
| walker |  | 2218 | 46 | README.md section #10 |  |  | 0.591 |
| walker |  | 2233 | 15 | python decl names surface in peepdb/db/mariadb.py |  |  | 0.591 |
| walker |  | 2233 | 0 | python decl at peepdb/db/mariadb.py:5 |  |  | 0.591 |
| walker |  | 2248 | 15 | python decl names surface in peepdb/db/mongodb.py |  |  | 0.591 |
| walker |  | 2248 | 0 | python decl at peepdb/db/mongodb.py:8 |  |  | 0.591 |
| walker |  | 2263 | 15 | python decl names surface in peepdb/db/mssql.py |  |  | 0.591 |
| walker |  | 2263 | 0 | python decl at peepdb/db/mssql.py:6 |  |  | 0.591 |
| walker |  | 2278 | 15 | python decl names surface in peepdb/db/mysql.py |  |  | 0.591 |
| walker |  | 2278 | 0 | python decl at peepdb/db/mysql.py:5 |  |  | 0.591 |
| walker |  | 2293 | 15 | python decl names surface in peepdb/db/postgresql.py |  |  | 0.591 |
| walker |  | 2293 | 0 | python decl at peepdb/db/postgresql.py:6 |  |  | 0.591 |
| ns | 2337 |  | 200 | cli.py: imports + CustomEncoder | 3.2 |  | 0.559 |
| walker |  | 2485 | 192 | [dependencies] in project.toml |  |  | 0.594 |
| walker |  | 2612 | 127 | manifest config in project.toml |  |  | 0.649 |
| ns | 2647 |  | 310 | cli.py: `cli` group docstring | 3.3 |  | 0.611 |
| walker |  | 2668 | 56 | README.md section #9 |  |  | 0.611 |
| walker |  | 2700 | 32 | docs/usage.md section #0 |  |  | 0.611 |
| walker |  | 2852 | 152 | python method sigs in peepdb/db/base.py |  |  | 0.611 |
| walker |  | 2852 | 0 | python method at peepdb/db/base.py:6 |  |  | 0.611 |
| walker |  | 2852 | 0 | python method at peepdb/db/base.py:33 |  |  | 0.611 |
| walker |  | 2852 | 0 | python method at peepdb/db/base.py:37 |  |  | 0.611 |
| walker |  | 2861 | 9 | python method at peepdb/db/base.py:17 |  |  | 0.612 |
| walker |  | 2870 | 9 | python method at peepdb/db/base.py:21 |  |  | 0.612 |
| walker |  | 2879 | 9 | python method at peepdb/db/base.py:25 |  |  | 0.612 |
| walker |  | 2888 | 9 | python method at peepdb/db/base.py:29 |  |  | 0.612 |
| walker |  | 3042 | 154 | headings outline in docs/README.md |  |  | 0.612 |
| walker |  | 3062 | 20 | docs/README.md section #2 |  |  | 0.612 |
| walker |  | 3080 | 18 | docs/README.md section #8 |  |  | 0.612 |
| ns | 3114 |  | 467 | cli.py: `save` command | 3.4 | 3.1 | 0.574 |
| walker |  | 3164 | 84 | python decl names surface in peepdb/cli.py |  |  | 0.574 |
| walker |  | 3164 | 0 | python decl at peepdb/cli.py:16 |  |  | 0.574 |
| walker |  | 3164 | 0 | python decl at peepdb/cli.py:181 |  |  | 0.574 |
| walker |  | 3171 | 7 | python decl at peepdb/cli.py:86 |  |  | 0.566 |
| ns | 3171 |  | 57 | cli.py: `list` command | 3.5 | 3.1 | 0.566 |
| walker |  | 3182 | 11 | python method sigs in peepdb/cli.py |  |  | 0.566 |
| walker |  | 3182 | 0 | python method at peepdb/cli.py:17 |  |  | 0.566 |
| walker |  | 3198 | 16 | python decl at peepdb/cli.py:25 |  |  | 0.567 |
| walker |  | 3205 | 7 | python decl body at peepdb/cli.py:25 body 50 |  |  | 0.567 |
| walker |  | 3212 | 7 | python decl body at peepdb/cli.py:181 body 182 |  |  | 0.567 |
| walker |  | 3243 | 31 | python decl at peepdb/cli.py:113 |  |  | 0.569 |
| walker |  | 3285 | 42 | python decl at peepdb/cli.py:97 |  |  | 0.572 |
| walker |  | 3293 | 8 | python decl body at peepdb/cli.py:86 body 94 |  |  | 0.574 |
| ns | 3312 |  | 141 | cli.py: `remove` command | 3.6 | 3.1 | 0.563 |
| walker |  | 3334 | 41 | python decl doc at peepdb/cli.py:86 |  |  | 0.581 |
| walker |  | 3339 | 5 | python method body at peepdb/db/base.py:17 body 19 |  |  | 0.581 |
| walker |  | 3374 | 35 | README.md section #1 |  |  | 0.589 |
| ns | 3415 |  | 103 | cli.py: `remove-all` command | 3.7 | 3.1 | 0.580 |
| walker |  | 3559 | 185 | python setup manifest at setup.py:16 |  |  | 0.611 |
| walker |  | 3604 | 45 | python decl doc at peepdb/cli.py:113 |  |  | 0.621 |
| walker |  | 3662 | 58 | docs/README.md section #1 |  |  | 0.621 |
| walker |  | 3709 | 47 | python decl doc at peepdb/cli.py:97 |  |  | 0.630 |
| walker |  | 3762 | 53 | python method sigs in peepdb/db/oracle.py |  |  | 0.630 |
| walker |  | 3762 | 0 | python method at peepdb/db/oracle.py:8 |  |  | 0.630 |
| walker |  | 3762 | 0 | python method at peepdb/db/oracle.py:25 |  |  | 0.630 |
| walker |  | 3762 | 0 | python method at peepdb/db/oracle.py:32 |  |  | 0.630 |
| ns | 3805 |  | 390 | cli.py: `view` — options + connection resolution | 3.8 | 3.1 | 0.600 |
| walker |  | 3816 | 54 | python method sigs in peepdb/db/mongodb.py |  |  | 0.600 |
| walker |  | 3816 | 0 | python method at peepdb/db/mongodb.py:9 |  |  | 0.600 |
| walker |  | 3816 | 0 | python method at peepdb/db/mongodb.py:32 |  |  | 0.600 |
| walker |  | 3816 | 0 | python method at peepdb/db/mongodb.py:39 |  |  | 0.600 |
| walker |  | 3826 | 10 | python method body at peepdb/db/mongodb.py:39 body 40 |  |  | 0.600 |
| walker |  | 3849 | 23 | README.md section #16 |  |  | 0.600 |
| walker |  | 3959 | 110 | plaintext config requirements.txt |  |  | 0.601 |
| ns | 4092 |  | 287 | cli.py: `view` — peep_db call + output rendering | 3.9 |  | 0.580 |
| ns | 4133 |  | 41 | cli.py: `main()` entry point | 3.10 |  | 0.576 |
| walker |  | 4174 | 215 | docs/README.md section #0 |  |  | 0.576 |
| ns | 4178 |  | 45 | core.py: all 4 top-level function names | 4.1 |  | 0.573 |
| walker |  | 4216 | 42 | listing of 'peepdb/tests' |  |  | 0.596 |
| walker |  | 4323 | 107 | README.md section #22 |  |  | 0.596 |
| walker |  | 4354 | 31 | python decl names surface in peepdb/db/firebase.py |  |  | 0.596 |
| walker |  | 4354 | 0 | python decl at peepdb/db/firebase.py:9 |  |  | 0.596 |
| walker |  | 4359 | 5 | python method body at peepdb/db/base.py:21 body 23 |  |  | 0.596 |
| ns | 4384 |  | 206 | core.py: imports + module logger | 4.2 |  | 0.576 |
| walker |  | 4651 | 292 | python decl names surface in peepdb/config.py |  |  | 0.578 |
| walker |  | 4651 | 0 | python decl at peepdb/config.py:49 |  |  | 0.578 |
| walker |  | 4651 | 0 | python decl at peepdb/config.py:60 |  |  | 0.578 |
| walker |  | 4651 | 0 | python decl at peepdb/config.py:70 |  |  | 0.578 |
| walker |  | 4651 | 0 | python decl at peepdb/config.py:73 |  |  | 0.578 |
| walker |  | 4651 | 0 | python decl at peepdb/config.py:76 |  |  | 0.578 |
| walker |  | 4651 | 0 | python decl at peepdb/config.py:112 |  |  | 0.578 |
| walker |  | 4651 | 0 | python decl at peepdb/config.py:144 |  |  | 0.578 |
| walker |  | 4651 | 0 | python decl at peepdb/config.py:161 |  |  | 0.578 |
| walker |  | 4651 | 0 | python decl at peepdb/config.py:178 |  |  | 0.578 |
| walker |  | 4651 | 0 | python decl at peepdb/config.py:196 |  |  | 0.578 |
| walker |  | 4659 | 8 | python decl at peepdb/config.py:15 |  |  | 0.578 |
| walker |  | 4670 | 11 | python decl at peepdb/config.py:41 |  |  | 0.578 |
| walker |  | 4682 | 12 | python decl at peepdb/config.py:29 |  |  | 0.578 |
| walker |  | 4704 | 22 | python class body at peepdb/config.py:15 |  |  | 0.579 |
| walker |  | 4719 | 15 | python decl body at peepdb/config.py:70 body 71 |  |  | 0.579 |
| walker |  | 4734 | 15 | python decl body at peepdb/config.py:73 body 74 |  |  | 0.579 |
| walker |  | 4739 | 5 | python method body at peepdb/db/base.py:25 body 27 |  |  | 0.579 |
| walker |  | 4744 | 5 | python method body at peepdb/db/base.py:29 body 31 |  |  | 0.579 |
| walker |  | 4750 | 6 | python method body at peepdb/db/base.py:37 body 38 |  |  | 0.579 |
| walker |  | 4829 | 79 | python method sigs in peepdb/db/mariadb.py |  |  | 0.579 |
| walker |  | 4829 | 0 | python method at peepdb/db/mariadb.py:6 |  |  | 0.579 |
| walker |  | 4829 | 0 | python method at peepdb/db/mariadb.py:23 |  |  | 0.579 |
| walker |  | 4829 | 0 | python method at peepdb/db/mariadb.py:30 |  |  | 0.579 |
| walker |  | 4829 | 0 | python method at peepdb/db/mariadb.py:34 |  |  | 0.579 |
| ns | 4834 |  | 450 | core.py: connect_to_database | 4.3 | 4.1 | 0.558 |
| walker |  | 4908 | 79 | python method sigs in peepdb/db/mysql.py |  |  | 0.558 |
| walker |  | 4908 | 0 | python method at peepdb/db/mysql.py:6 |  |  | 0.558 |
| walker |  | 4908 | 0 | python method at peepdb/db/mysql.py:22 |  |  | 0.558 |
| walker |  | 4908 | 0 | python method at peepdb/db/mysql.py:29 |  |  | 0.558 |
| walker |  | 4908 | 0 | python method at peepdb/db/mysql.py:33 |  |  | 0.558 |
| walker |  | 4987 | 79 | python method sigs in peepdb/db/postgresql.py |  |  | 0.558 |
| walker |  | 4987 | 0 | python method at peepdb/db/postgresql.py:7 |  |  | 0.558 |
| walker |  | 4987 | 0 | python method at peepdb/db/postgresql.py:23 |  |  | 0.558 |
| walker |  | 4987 | 0 | python method at peepdb/db/postgresql.py:30 |  |  | 0.558 |
| walker |  | 4987 | 0 | python method at peepdb/db/postgresql.py:34 |  |  | 0.558 |
| walker |  | 5066 | 79 | python method sigs in peepdb/db/sqlite.py |  |  | 0.558 |
| walker |  | 5066 | 0 | python method at peepdb/db/sqlite.py:7 |  |  | 0.558 |
| walker |  | 5066 | 0 | python method at peepdb/db/sqlite.py:22 |  |  | 0.558 |
| walker |  | 5066 | 0 | python method at peepdb/db/sqlite.py:29 |  |  | 0.558 |
| walker |  | 5066 | 0 | python method at peepdb/db/sqlite.py:43 |  |  | 0.558 |
| walker |  | 5093 | 27 | docs/installation.md section #1 |  |  | 0.558 |
| walker |  | 5124 | 31 | README.md section #18 |  |  | 0.558 |
| walker |  | 5138 | 14 | python method body at peepdb/db/base.py:33 body 34 |  |  | 0.558 |
| ns | 5140 |  | 306 | core.py: peep_db — signature + fetch | 4.4 | 4.1 | 0.542 |
| walker |  | 5210 | 72 | docs/index.md section #0 |  |  | 0.542 |
| walker |  | 5283 | 73 | docs/installation.md section #0 |  |  | 0.542 |
| ns | 5376 |  | 236 | core.py: peep_db — format branch + disconnect | 4.5 |  | 0.530 |
| walker |  | 5401 | 118 | python decl names surface in peepdb/core.py |  |  | 0.537 |
| walker |  | 5401 | 0 | python decl at peepdb/core.py:27 |  |  | 0.537 |
| walker |  | 5401 | 0 | python decl at peepdb/core.py:101 |  |  | 0.537 |
| walker |  | 5401 | 0 | python decl at peepdb/core.py:118 |  |  | 0.537 |
| walker |  | 5510 | 109 | python decl at peepdb/core.py:56 |  |  | 0.544 |
| walker |  | 5544 | 34 | README.md section #20 |  |  | 0.544 |
| ns | 5586 |  | 210 | core.py: format_as_table | 4.6 | 4.1 | 0.534 |
| walker |  | 5640 | 96 | python method sigs in peepdb/db/firebase.py |  |  | 0.534 |
| walker |  | 5640 | 0 | python method at peepdb/db/firebase.py:10 |  |  | 0.534 |
| walker |  | 5640 | 0 | python method at peepdb/db/firebase.py:15 |  |  | 0.534 |
| walker |  | 5640 | 0 | python method at peepdb/db/firebase.py:30 |  |  | 0.534 |
| walker |  | 5640 | 0 | python method at peepdb/db/firebase.py:39 |  |  | 0.534 |
| walker |  | 5656 | 16 | python method at peepdb/db/firebase.py:26 |  |  | 0.534 |
| walker |  | 5661 | 5 | python method body at peepdb/db/firebase.py:26 body 28 |  |  | 0.534 |
| walker |  | 5712 | 51 | docs/README.md section #9 |  |  | 0.534 |
| walker |  | 5763 | 51 | python decl body at peepdb/cli.py:97 body 107 |  |  | 0.548 |
| walker |  | 5774 | 11 | python decl body at peepdb/cli.py:113 body 122 |  |  | 0.551 |
| walker |  | 5813 | 39 | README.md section #23 |  |  | 0.551 |
| ns | 5843 |  | 257 | core.py: format_value | 4.7 | 4.1 | 0.537 |
| walker |  | 5850 | 37 | README.md section #24 |  |  | 0.537 |
| walker |  | 5886 | 36 | docs/installation.md section #4 |  |  | 0.537 |
| ns | 5972 |  | 129 | config.py: all 12 top-level function names | 5.1 |  | 0.547 |
| walker |  | 6040 | 154 | python decl at peepdb/cli.py:126 |  |  | 0.554 |
| walker |  | 6092 | 52 | python decl doc at peepdb/cli.py:126 |  |  | 0.560 |
| ns | 6109 |  | 137 | README security section | 5.2 |  | 0.556 |
| walker |  | 6115 | 23 | docs/README.md section #5 |  |  | 0.556 |
| walker |  | 6239 | 124 | README.md section #27 |  |  | 0.566 |
| ns | 6269 |  | 160 | config.py: KeySecurity + on-disk paths + module logger | 5.3 |  | 0.570 |
| walker |  | 6364 | 125 | python method sigs in peepdb/db/mssql.py |  |  | 0.570 |
| walker |  | 6364 | 0 | python method at peepdb/db/mssql.py:7 |  |  | 0.570 |
| walker |  | 6364 | 0 | python method at peepdb/db/mssql.py:12 |  |  | 0.570 |
| walker |  | 6364 | 0 | python method at peepdb/db/mssql.py:28 |  |  | 0.570 |
| walker |  | 6364 | 0 | python method at peepdb/db/mssql.py:45 |  |  | 0.570 |
| walker |  | 6377 | 13 | python method at peepdb/db/mssql.py:35 |  |  | 0.570 |
| walker |  | 6422 | 45 | README.md section #12 |  |  | 0.570 |
| walker |  | 6452 | 30 | python imports in peepdb/db/base.py |  |  | 0.570 |
| walker |  | 6482 | 30 | python imports in peepdb/db/mariadb.py |  |  | 0.570 |
| walker |  | 6512 | 30 | python imports in peepdb/db/mysql.py |  |  | 0.570 |
| ns | 6521 |  | 252 | config.py: key derivation (password path + keyring path) | 5.4 | 5.1 | 0.560 |
| walker |  | 6553 | 41 | docs/usage.md section #2 |  |  | 0.560 |
| walker |  | 6603 | 50 | README.md section #15 |  |  | 0.560 |
| walker |  | 6619 | 16 | python decl body at peepdb/cli.py:113 body 123 |  |  | 0.564 |
| walker |  | 6656 | 37 | python imports in peepdb/db/sqlite.py |  |  | 0.564 |
| walker |  | 6733 | 77 | python decl body at peepdb/config.py:41 body 43 |  |  | 0.569 |
| ns | 6739 |  | 218 | config.py: get_key_security_config + get_key | 5.5 | 5.1 | 0.558 |
| walker |  | 6771 | 38 | python imports in peepdb/db/mssql.py |  |  | 0.558 |
| walker |  | 6810 | 39 | python imports in peepdb/db/oracle.py |  |  | 0.558 |
| walker |  | 6860 | 50 | docs/index.md section #10 |  |  | 0.558 |
| walker |  | 6887 | 27 | docs/README.md section #12 |  |  | 0.558 |
| walker |  | 6942 | 55 | docs/usage.md section #7 |  |  | 0.558 |
| walker |  | 6988 | 46 | python imports in peepdb/db/mongodb.py |  |  | 0.559 |
| walker |  | 7052 | 64 | README.md section #14 |  |  | 0.559 |
| walker |  | 7100 | 48 | python imports in peepdb/db/postgresql.py |  |  | 0.559 |
| ns | 7134 |  | 395 | config.py: save_connection | 5.6 | 5.1 | 0.541 |
| walker |  | 7348 | 248 | python decl at peepdb/cli.py:53 |  |  | 0.553 |
| walker |  | 7414 | 66 | python decl doc at peepdb/cli.py:53 |  |  | 0.560 |
| ns | 7432 |  | 298 | config.py: get_connection | 5.7 | 5.1 | 0.545 |
| walker |  | 7463 | 49 | python imports in peepdb/db/firebase.py |  |  | 0.545 |
| walker |  | 7482 | 19 | docs/index.md section #5 |  |  | 0.545 |
| walker |  | 7513 | 31 | python method body at peepdb/db/mariadb.py:30 body 31 |  |  | 0.545 |
| walker |  | 7545 | 32 | python method body at peepdb/db/oracle.py:32 body 33 |  |  | 0.545 |
| ns | 7586 |  | 154 | config.py: list_connections | 5.8 | 5.1 | 0.538 |
| walker |  | 7644 | 99 | docs/README.md section #10 |  |  | 0.538 |
| walker |  | 7653 | 9 | python decl body at peepdb/core.py:101 body 102 |  |  | 0.538 |
| walker |  | 7707 | 54 | python method body at peepdb/cli.py:17 body 18 |  |  | 0.542 |
| walker |  | 7771 | 64 | docs/usage.md section #4 |  |  | 0.542 |
| ns | 7801 |  | 215 | db/__init__.py — backend export roster | 6.1 |  | 0.554 |
| walker |  | 7835 | 64 | docs/usage.md section #5 |  |  | 0.554 |
| walker |  | 7940 | 105 | python decl body at peepdb/config.py:60 body 61 |  |  | 0.560 |
| walker |  | 7973 | 33 | python method body at peepdb/db/firebase.py:10 body 11 |  |  | 0.561 |
| walker |  | 7994 | 21 | docs/index.md section #7 |  |  | 0.561 |
| walker |  | 8014 | 20 | docs/index.md section #6 |  |  | 0.561 |
| walker |  | 8122 | 108 | python decl body at peepdb/config.py:49 body 50 |  |  | 0.575 |
| ns | 8204 |  | 403 | db/base.py — BaseDatabase ABC contract | 6.2 |  | 0.573 |
| walker |  | 8244 | 122 | docs/README.md section #7 |  |  | 0.573 |
| walker |  | 8311 | 67 | docs/installation.md section #2 |  |  | 0.580 |
| walker |  | 8345 | 34 | python method body at peepdb/db/mysql.py:29 body 30 |  |  | 0.580 |
| ns | 8452 |  | 248 | db/sqlite.py — connect() | 6.3 |  | 0.572 |
| walker |  | 8460 | 115 | python imports in peepdb/cli.py |  |  | 0.592 |
| walker |  | 8483 | 23 | docs/index.md section #3 |  |  | 0.592 |
| walker |  | 8505 | 22 | docs/index.md section #4 |  |  | 0.592 |
| walker |  | 8528 | 23 | docs/index.md section #2 |  |  | 0.592 |
| walker |  | 8566 | 38 | python method at peepdb/db/oracle.py:36 |  |  | 0.592 |
| ns | 8592 |  | 140 | db/firebase.py — constructor | 6.4 |  | 0.597 |
| ns | 8763 |  | 171 | db/mongodb.py — connect() URI construction | 6.5 |  | 0.592 |
| ns | 8813 |  | 50 | exceptions.py + __main__.py | 7.1 |  | 0.591 |
| walker |  | 8843 | 277 | python decl doc at peepdb/cli.py:25 |  |  | 0.618 |
| walker |  | 8980 | 137 | python imports in peepdb/core.py |  |  | 0.632 |
| walker |  | 9053 | 73 | README.md section #26 |  |  | 0.632 |
| walker |  | 9064 | 11 | python decl body at peepdb/core.py:118 body 119 |  |  | 0.632 |
| ns | 9075 |  | 262 | Test function locations, 4 of 6 test modules | 8.1 |  | 0.623 |
| walker |  | 9204 | 140 | python imports in peepdb/config.py |  |  | 0.623 |
| walker |  | 9244 | 40 | python method body at peepdb/db/postgresql.py:30 body 31 |  |  | 0.623 |
| ns | 9348 |  | 273 | test_mongodb_uri.py (full) | 8.2 |  | 0.613 |
| walker |  | 9374 | 130 | python decl body at peepdb/config.py:29 body 31 |  |  | 0.625 |
| walker |  | 9451 | 77 | README.md section #17 |  |  | 0.625 |
| walker |  | 9585 | 134 | python decl body at peepdb/config.py:161 body 162 |  |  | 0.625 |
| walker |  | 9673 | 88 | docs/index.md section #8 |  |  | 0.625 |
| ns | 9699 |  | 351 | test_connection.py — unused/dead test helper | 8.3 |  | 0.614 |
| walker |  | 9718 | 45 | python method body at peepdb/db/mssql.py:7 body 8 |  |  | 0.614 |
| walker |  | 9764 | 46 | docs/README.md section #11 |  |  | 0.614 |
| walker |  | 9812 | 48 | python method body at peepdb/db/mongodb.py:32 body 33 |  |  | 0.614 |
| ns | 9916 |  | 217 | README example output (table + JSON) | 9.1 |  | 0.605 |
| ns | 9962 |  | 46 | docs/index.md — encryption key storage claim | 9.2 |  | 0.604 |
| walker |  | 9966 | 154 | python decl body at peepdb/config.py:144 body 145 |  |  | 0.617 |
| ns | 9990 |  | 28 | LICENSE identity: GPLv3 | 9.3 |  | 0.616 |

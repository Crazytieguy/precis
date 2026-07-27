Score(3000)=0.612 I=0.805 C=0.465 ns_rows≤3K=19/53 (reached=8 partial=2 missing=9)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 42 | 42 | listing of '.' |  |  | 1.000 |
| ns | 42 |  | 42 | Root directory listing | 1.1 |  | 1.000 |
| walker |  | 45 | 3 | listing of '.github' |  |  | 1.000 |
| walker |  | 52 | 7 | listing of '.github/workflows' |  |  | 1.000 |
| ns | 77 |  | 35 | peepdb/ package listing | 1.2 |  | 0.724 |
| walker |  | 79 | 27 | listing of 'images' |  |  | 0.733 |
| walker |  | 108 | 29 | entry-point scripts in project.toml |  |  | 0.733 |
| ns | 121 |  | 44 | peepdb/db/ backend listing | 1.3 |  | 0.568 |
| walker |  | 143 | 35 | listing of 'peepdb' |  |  | 0.788 |
| ns | 162 |  | 41 | peepdb/tests/ listing | 1.4 |  | 0.692 |
| walker |  | 183 | 40 | listing of 'docs' |  |  | 0.713 |
| ns | 202 |  | 40 | docs/ listing (Jekyll site) | 1.5 |  | 0.754 |
| ns | 210 |  | 8 | .github/workflows/ listing | 1.6 |  | 0.759 |
| walker |  | 227 | 44 | listing of 'peepdb/db' |  |  | 0.913 |
| ns | 237 |  | 27 | images/ listing | 1.7 |  | 0.910 |
| walker |  | 312 | 85 | README headline in README.md |  |  | 0.914 |
| ns | 334 |  | 97 | README title + one-line description | 1.8 |  | 0.899 |
| ns | 513 |  | 179 | README feature list | 1.9 |  | 0.822 |
| ns | 694 |  | 181 | MANIFEST.in + .gitignore | 2.1 |  | 0.676 |
| walker |  | 905 | 593 | YAML config at .github/workflows/test.yml |  |  | 0.690 |
| walker |  | 916 | 11 | python decl names surface in peepdb/exceptions.py |  |  | 0.690 |
| walker |  | 916 | 0 | python decl at peepdb/exceptions.py:1 |  |  | 0.690 |
| walker |  | 921 | 5 | python class body at peepdb/exceptions.py:1 |  |  | 0.690 |
| walker |  | 935 | 14 | python decl names surface in peepdb/db/base.py |  |  | 0.690 |
| walker |  | 935 | 0 | python decl at peepdb/db/base.py:5 |  |  | 0.690 |
| ns | 979 |  | 285 | Installation: pip install + system deps + verification | 2.2 |  | 0.590 |
| walker |  | 1031 | 96 | README headline in docs/README.md |  |  | 0.590 |
| walker |  | 1043 | 12 | python imports in peepdb/__main__.py |  |  | 0.590 |
| walker |  | 1057 | 14 | python imports in setup.py |  |  | 0.590 |
| walker |  | 1111 | 54 | headings outline in docs/installation.md |  |  | 0.592 |
| ns | 1151 |  | 172 | project.toml — build system + project identity | 2.3 |  | 0.554 |
| walker |  | 1167 | 56 | headings outline in docs/index.md |  |  | 0.554 |
| walker |  | 1382 | 215 | python imports in peepdb/db/__init__.py |  |  | 0.556 |
| walker |  | 1456 | 74 | [package] in project.toml |  |  | 0.570 |
| ns | 1477 |  | 326 | project.toml — dependencies, entry point, packaging | 2.4 |  | 0.509 |
| walker |  | 1649 | 193 | headings outline in README.md |  |  | 0.512 |
| ns | 1664 |  | 187 | setup.py — where it diverges from project.toml | 2.5 |  | 0.489 |
| walker |  | 1691 | 42 | README.md section #19 |  |  | 0.489 |
| walker |  | 1726 | 35 | README.md section #8 |  |  | 0.489 |
| ns | 1739 |  | 75 | requirements.txt (pinned freeze, sampled) | 2.6 |  | 0.482 |
| walker |  | 1756 | 30 | README.md section #11 |  |  | 0.482 |
| walker |  | 1784 | 28 | README.md section #30 |  |  | 0.482 |
| walker |  | 1803 | 19 | README.md section #5 |  |  | 0.484 |
| walker |  | 1833 | 30 | README.md section #29 |  |  | 0.484 |
| walker |  | 1854 | 21 | README.md section #7 |  |  | 0.488 |
| walker |  | 1874 | 20 | README.md section #6 |  |  | 0.492 |
| walker |  | 1909 | 35 | README.md section #28 |  |  | 0.492 |
| walker |  | 1923 | 14 | python decl names surface in peepdb/db/oracle.py |  |  | 0.492 |
| walker |  | 1923 | 0 | python decl at peepdb/db/oracle.py:7 |  |  | 0.492 |
| walker |  | 1937 | 14 | python decl names surface in peepdb/db/sqlite.py |  |  | 0.492 |
| walker |  | 1937 | 0 | python decl at peepdb/db/sqlite.py:6 |  |  | 0.492 |
| walker |  | 2034 | 97 | headings outline in docs/usage.md |  |  | 0.492 |
| walker |  | 2057 | 23 | README.md section #3 |  |  | 0.499 |
| walker |  | 2079 | 22 | README.md section #4 |  |  | 0.508 |
| ns | 2090 |  | 351 | CI test job + publish-job trigger gating (test.yml) | 2.7 |  | 0.585 |
| walker |  | 2102 | 23 | README.md section #2 |  |  | 0.592 |
| ns | 2138 |  | 48 | cli.py: all 5 command names | 3.1 |  | 0.584 |
| walker |  | 2173 | 71 | README.md section #13 |  |  | 0.584 |
| walker |  | 2219 | 46 | README.md section #10 |  |  | 0.591 |
| walker |  | 2234 | 15 | python decl names surface in peepdb/db/mariadb.py |  |  | 0.591 |
| walker |  | 2234 | 0 | python decl at peepdb/db/mariadb.py:5 |  |  | 0.591 |
| walker |  | 2249 | 15 | python decl names surface in peepdb/db/mongodb.py |  |  | 0.591 |
| walker |  | 2249 | 0 | python decl at peepdb/db/mongodb.py:8 |  |  | 0.591 |
| walker |  | 2264 | 15 | python decl names surface in peepdb/db/mssql.py |  |  | 0.591 |
| walker |  | 2264 | 0 | python decl at peepdb/db/mssql.py:6 |  |  | 0.591 |
| walker |  | 2279 | 15 | python decl names surface in peepdb/db/mysql.py |  |  | 0.591 |
| walker |  | 2279 | 0 | python decl at peepdb/db/mysql.py:5 |  |  | 0.591 |
| walker |  | 2294 | 15 | python decl names surface in peepdb/db/postgresql.py |  |  | 0.591 |
| walker |  | 2294 | 0 | python decl at peepdb/db/postgresql.py:6 |  |  | 0.591 |
| ns | 2338 |  | 200 | cli.py: imports + CustomEncoder | 3.2 |  | 0.559 |
| walker |  | 2486 | 192 | [dependencies] in project.toml |  |  | 0.594 |
| walker |  | 2613 | 127 | manifest config in project.toml |  |  | 0.649 |
| ns | 2648 |  | 310 | cli.py: `cli` group docstring | 3.3 |  | 0.611 |
| walker |  | 2669 | 56 | README.md section #9 |  |  | 0.611 |
| walker |  | 2701 | 32 | docs/usage.md section #0 |  |  | 0.611 |
| walker |  | 2853 | 152 | python method sigs in peepdb/db/base.py |  |  | 0.611 |
| walker |  | 2853 | 0 | python method at peepdb/db/base.py:6 |  |  | 0.611 |
| walker |  | 2853 | 0 | python method at peepdb/db/base.py:33 |  |  | 0.611 |
| walker |  | 2853 | 0 | python method at peepdb/db/base.py:37 |  |  | 0.611 |
| walker |  | 2862 | 9 | python method at peepdb/db/base.py:17 |  |  | 0.612 |
| walker |  | 2871 | 9 | python method at peepdb/db/base.py:21 |  |  | 0.612 |
| walker |  | 2880 | 9 | python method at peepdb/db/base.py:25 |  |  | 0.612 |
| walker |  | 2889 | 9 | python method at peepdb/db/base.py:29 |  |  | 0.612 |
| walker |  | 3043 | 154 | headings outline in docs/README.md |  |  | 0.612 |
| walker |  | 3063 | 20 | docs/README.md section #2 |  |  | 0.612 |
| walker |  | 3081 | 18 | docs/README.md section #8 |  |  | 0.612 |
| ns | 3115 |  | 467 | cli.py: `save` command | 3.4 | 3.1 | 0.574 |
| walker |  | 3165 | 84 | python decl names surface in peepdb/cli.py |  |  | 0.574 |
| walker |  | 3165 | 0 | python decl at peepdb/cli.py:16 |  |  | 0.574 |
| walker |  | 3165 | 0 | python decl at peepdb/cli.py:181 |  |  | 0.574 |
| walker |  | 3172 | 7 | python decl at peepdb/cli.py:86 |  |  | 0.566 |
| ns | 3172 |  | 57 | cli.py: `list` command | 3.5 | 3.1 | 0.566 |
| walker |  | 3183 | 11 | python method sigs in peepdb/cli.py |  |  | 0.566 |
| walker |  | 3183 | 0 | python method at peepdb/cli.py:17 |  |  | 0.566 |
| walker |  | 3199 | 16 | python decl at peepdb/cli.py:25 |  |  | 0.567 |
| walker |  | 3206 | 7 | python decl body at peepdb/cli.py:25 body 50 |  |  | 0.567 |
| walker |  | 3213 | 7 | python decl body at peepdb/cli.py:181 body 182 |  |  | 0.567 |
| walker |  | 3244 | 31 | python decl at peepdb/cli.py:113 |  |  | 0.569 |
| walker |  | 3286 | 42 | python decl at peepdb/cli.py:97 |  |  | 0.572 |
| walker |  | 3294 | 8 | python decl body at peepdb/cli.py:86 body 94 |  |  | 0.574 |
| ns | 3313 |  | 141 | cli.py: `remove` command | 3.6 | 3.1 | 0.563 |
| walker |  | 3335 | 41 | python decl doc at peepdb/cli.py:86 |  |  | 0.581 |
| walker |  | 3340 | 5 | python method body at peepdb/db/base.py:17 body 19 |  |  | 0.581 |
| walker |  | 3375 | 35 | README.md section #1 |  |  | 0.589 |
| ns | 3416 |  | 103 | cli.py: `remove-all` command | 3.7 | 3.1 | 0.580 |
| walker |  | 3560 | 185 | python setup manifest at setup.py:16 |  |  | 0.611 |
| walker |  | 3605 | 45 | python decl doc at peepdb/cli.py:113 |  |  | 0.621 |
| walker |  | 3663 | 58 | docs/README.md section #1 |  |  | 0.621 |
| walker |  | 3710 | 47 | python decl doc at peepdb/cli.py:97 |  |  | 0.630 |
| walker |  | 3763 | 53 | python method sigs in peepdb/db/oracle.py |  |  | 0.630 |
| walker |  | 3763 | 0 | python method at peepdb/db/oracle.py:8 |  |  | 0.630 |
| walker |  | 3763 | 0 | python method at peepdb/db/oracle.py:25 |  |  | 0.630 |
| walker |  | 3763 | 0 | python method at peepdb/db/oracle.py:32 |  |  | 0.630 |
| ns | 3806 |  | 390 | cli.py: `view` — options + connection resolution | 3.8 | 3.1 | 0.600 |
| walker |  | 3817 | 54 | python method sigs in peepdb/db/mongodb.py |  |  | 0.600 |
| walker |  | 3817 | 0 | python method at peepdb/db/mongodb.py:9 |  |  | 0.600 |
| walker |  | 3817 | 0 | python method at peepdb/db/mongodb.py:32 |  |  | 0.600 |
| walker |  | 3817 | 0 | python method at peepdb/db/mongodb.py:39 |  |  | 0.600 |
| walker |  | 3827 | 10 | python method body at peepdb/db/mongodb.py:39 body 40 |  |  | 0.600 |
| walker |  | 3850 | 23 | README.md section #16 |  |  | 0.600 |
| walker |  | 3960 | 110 | plaintext config requirements.txt |  |  | 0.601 |
| walker |  | 4001 | 41 | listing of 'peepdb/tests' |  |  | 0.626 |
| ns | 4093 |  | 287 | cli.py: `view` — peep_db call + output rendering | 3.9 |  | 0.604 |
| ns | 4134 |  | 41 | cli.py: `main()` entry point | 3.10 |  | 0.600 |
| ns | 4179 |  | 45 | core.py: all 4 top-level function names | 4.1 |  | 0.596 |
| walker |  | 4216 | 215 | docs/README.md section #0 |  |  | 0.596 |
| walker |  | 4323 | 107 | README.md section #22 |  |  | 0.596 |
| walker |  | 4354 | 31 | python decl names surface in peepdb/db/firebase.py |  |  | 0.596 |
| walker |  | 4354 | 0 | python decl at peepdb/db/firebase.py:9 |  |  | 0.596 |
| walker |  | 4359 | 5 | python method body at peepdb/db/base.py:21 body 23 |  |  | 0.596 |
| ns | 4385 |  | 206 | core.py: imports + module logger | 4.2 |  | 0.576 |
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
| ns | 4835 |  | 450 | core.py: connect_to_database | 4.3 | 4.1 | 0.558 |
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
| ns | 5141 |  | 306 | core.py: peep_db — signature + fetch | 4.4 | 4.1 | 0.542 |
| walker |  | 5210 | 72 | docs/index.md section #0 |  |  | 0.542 |
| walker |  | 5283 | 73 | docs/installation.md section #0 |  |  | 0.542 |
| ns | 5377 |  | 236 | core.py: peep_db — format branch + disconnect | 4.5 |  | 0.530 |
| walker |  | 5401 | 118 | python decl names surface in peepdb/core.py |  |  | 0.537 |
| walker |  | 5401 | 0 | python decl at peepdb/core.py:27 |  |  | 0.537 |
| walker |  | 5401 | 0 | python decl at peepdb/core.py:101 |  |  | 0.537 |
| walker |  | 5401 | 0 | python decl at peepdb/core.py:118 |  |  | 0.537 |
| walker |  | 5510 | 109 | python decl at peepdb/core.py:56 |  |  | 0.544 |
| walker |  | 5544 | 34 | README.md section #20 |  |  | 0.544 |
| ns | 5587 |  | 210 | core.py: format_as_table | 4.6 | 4.1 | 0.534 |
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
| ns | 5844 |  | 257 | core.py: format_value | 4.7 | 4.1 | 0.537 |
| walker |  | 5850 | 37 | README.md section #24 |  |  | 0.537 |
| walker |  | 5886 | 36 | docs/installation.md section #4 |  |  | 0.537 |
| ns | 5973 |  | 129 | config.py: all 12 top-level function names | 5.1 |  | 0.547 |
| walker |  | 6040 | 154 | python decl at peepdb/cli.py:126 |  |  | 0.554 |
| walker |  | 6092 | 52 | python decl doc at peepdb/cli.py:126 |  |  | 0.560 |
| ns | 6110 |  | 137 | README security section | 5.2 |  | 0.556 |
| walker |  | 6115 | 23 | docs/README.md section #5 |  |  | 0.556 |
| walker |  | 6239 | 124 | README.md section #27 |  |  | 0.566 |
| ns | 6270 |  | 160 | config.py: KeySecurity + on-disk paths + module logger | 5.3 |  | 0.570 |
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
| ns | 6522 |  | 252 | config.py: key derivation (password path + keyring path) | 5.4 | 5.1 | 0.560 |
| walker |  | 6553 | 41 | docs/usage.md section #2 |  |  | 0.560 |
| walker |  | 6603 | 50 | README.md section #15 |  |  | 0.560 |
| walker |  | 6619 | 16 | python decl body at peepdb/cli.py:113 body 123 |  |  | 0.564 |
| walker |  | 6656 | 37 | python imports in peepdb/db/sqlite.py |  |  | 0.564 |
| walker |  | 6733 | 77 | python decl body at peepdb/config.py:41 body 43 |  |  | 0.569 |
| ns | 6740 |  | 218 | config.py: get_key_security_config + get_key | 5.5 | 5.1 | 0.558 |
| walker |  | 6771 | 38 | python imports in peepdb/db/mssql.py |  |  | 0.558 |
| walker |  | 6810 | 39 | python imports in peepdb/db/oracle.py |  |  | 0.558 |
| walker |  | 6860 | 50 | docs/index.md section #10 |  |  | 0.558 |
| walker |  | 6887 | 27 | docs/README.md section #12 |  |  | 0.558 |
| walker |  | 6942 | 55 | docs/usage.md section #7 |  |  | 0.558 |
| walker |  | 6988 | 46 | python imports in peepdb/db/mongodb.py |  |  | 0.559 |
| walker |  | 7052 | 64 | README.md section #14 |  |  | 0.559 |
| walker |  | 7100 | 48 | python imports in peepdb/db/postgresql.py |  |  | 0.559 |
| ns | 7135 |  | 395 | config.py: save_connection | 5.6 | 5.1 | 0.541 |
| walker |  | 7348 | 248 | python decl at peepdb/cli.py:53 |  |  | 0.553 |
| walker |  | 7414 | 66 | python decl doc at peepdb/cli.py:53 |  |  | 0.560 |
| ns | 7433 |  | 298 | config.py: get_connection | 5.7 | 5.1 | 0.545 |
| walker |  | 7463 | 49 | python imports in peepdb/db/firebase.py |  |  | 0.545 |
| walker |  | 7482 | 19 | docs/index.md section #5 |  |  | 0.545 |
| walker |  | 7513 | 31 | python method body at peepdb/db/mariadb.py:30 body 31 |  |  | 0.545 |
| walker |  | 7545 | 32 | python method body at peepdb/db/oracle.py:32 body 33 |  |  | 0.545 |
| ns | 7587 |  | 154 | config.py: list_connections | 5.8 | 5.1 | 0.538 |
| walker |  | 7644 | 99 | docs/README.md section #10 |  |  | 0.538 |
| walker |  | 7653 | 9 | python decl body at peepdb/core.py:101 body 102 |  |  | 0.538 |
| walker |  | 7707 | 54 | python method body at peepdb/cli.py:17 body 18 |  |  | 0.542 |
| walker |  | 7771 | 64 | docs/usage.md section #4 |  |  | 0.542 |
| ns | 7802 |  | 215 | db/__init__.py — backend export roster | 6.1 |  | 0.554 |
| walker |  | 7835 | 64 | docs/usage.md section #5 |  |  | 0.554 |
| walker |  | 7940 | 105 | python decl body at peepdb/config.py:60 body 61 |  |  | 0.560 |
| walker |  | 7973 | 33 | python method body at peepdb/db/firebase.py:10 body 11 |  |  | 0.561 |
| walker |  | 7994 | 21 | docs/index.md section #7 |  |  | 0.561 |
| walker |  | 8014 | 20 | docs/index.md section #6 |  |  | 0.561 |
| walker |  | 8122 | 108 | python decl body at peepdb/config.py:49 body 50 |  |  | 0.575 |
| ns | 8205 |  | 403 | db/base.py — BaseDatabase ABC contract | 6.2 |  | 0.573 |
| walker |  | 8244 | 122 | docs/README.md section #7 |  |  | 0.573 |
| walker |  | 8311 | 67 | docs/installation.md section #2 |  |  | 0.580 |
| walker |  | 8345 | 34 | python method body at peepdb/db/mysql.py:29 body 30 |  |  | 0.580 |
| ns | 8453 |  | 248 | db/sqlite.py — connect() | 6.3 |  | 0.572 |
| walker |  | 8460 | 115 | python imports in peepdb/cli.py |  |  | 0.592 |
| walker |  | 8483 | 23 | docs/index.md section #3 |  |  | 0.592 |
| walker |  | 8505 | 22 | docs/index.md section #4 |  |  | 0.592 |
| walker |  | 8528 | 23 | docs/index.md section #2 |  |  | 0.592 |
| walker |  | 8566 | 38 | python method at peepdb/db/oracle.py:36 |  |  | 0.592 |
| ns | 8593 |  | 140 | db/firebase.py — constructor | 6.4 |  | 0.597 |
| ns | 8764 |  | 171 | db/mongodb.py — connect() URI construction | 6.5 |  | 0.592 |
| ns | 8814 |  | 50 | exceptions.py + __main__.py | 7.1 |  | 0.591 |
| walker |  | 8843 | 277 | python decl doc at peepdb/cli.py:25 |  |  | 0.618 |
| walker |  | 8980 | 137 | python imports in peepdb/core.py |  |  | 0.632 |
| walker |  | 9053 | 73 | README.md section #26 |  |  | 0.632 |
| walker |  | 9064 | 11 | python decl body at peepdb/core.py:118 body 119 |  |  | 0.632 |
| ns | 9076 |  | 262 | Test function locations, 4 of 6 test modules | 8.1 |  | 0.623 |
| walker |  | 9204 | 140 | python imports in peepdb/config.py |  |  | 0.623 |
| walker |  | 9244 | 40 | python method body at peepdb/db/postgresql.py:30 body 31 |  |  | 0.623 |
| ns | 9349 |  | 273 | test_mongodb_uri.py (full) | 8.2 |  | 0.613 |
| walker |  | 9374 | 130 | python decl body at peepdb/config.py:29 body 31 |  |  | 0.625 |
| walker |  | 9451 | 77 | README.md section #17 |  |  | 0.625 |
| walker |  | 9585 | 134 | python decl body at peepdb/config.py:161 body 162 |  |  | 0.625 |
| walker |  | 9673 | 88 | docs/index.md section #8 |  |  | 0.625 |
| ns | 9700 |  | 351 | test_connection.py — unused/dead test helper | 8.3 |  | 0.614 |
| walker |  | 9718 | 45 | python method body at peepdb/db/mssql.py:7 body 8 |  |  | 0.614 |
| walker |  | 9764 | 46 | docs/README.md section #11 |  |  | 0.614 |
| walker |  | 9812 | 48 | python method body at peepdb/db/mongodb.py:32 body 33 |  |  | 0.614 |
| ns | 9917 |  | 217 | README example output (table + JSON) | 9.1 |  | 0.605 |
| ns | 9963 |  | 46 | docs/index.md — encryption key storage claim | 9.2 |  | 0.604 |
| walker |  | 9966 | 154 | python decl body at peepdb/config.py:144 body 145 |  |  | 0.617 |
| ns | 9991 |  | 28 | LICENSE identity: GPLv3 | 9.3 |  | 0.616 |

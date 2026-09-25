Score(3000)=0.625 I=0.836 C=0.468 ns_rows≤3K=21/54 grid(1000/1442/2080/3000/4327/6240/9000)=0.592/0.595/0.681/0.625/0.627/0.599/0.567

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
| walker |  | 453 | 56 | Toml::PackageMetadata { file: project.toml } |  |  | 0.846 |
| ns | 497 |  | 119 | peepdb/db/__init__.py — backend class ↔ module map | 1.7 |  | 0.744 |
| walker |  | 645 | 192 | Toml::Dependencies { file: project.toml } |  |  | 0.744 |
| ns | 676 |  | 179 | README feature bullets | 1.8 |  | 0.682 |
| ns | 831 |  | 155 | peepdb/config.py: every top-level definition, names only | 1.9 |  | 0.613 |
| walker |  | 838 | 193 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.622 |
| walker |  | 873 | 35 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.622 |
| walker |  | 901 | 28 | Markdown::Section { file: README.md, section_index: 11, keeps_default_concavity: false } |  |  | 0.622 |
| ns | 908 |  | 77 | Entry points: console script, __main__.py, exceptions.py in full | 1.10 |  | 0.592 |
| walker |  | 931 | 30 | Markdown::Section { file: README.md, section_index: 10, keeps_default_concavity: false } |  |  | 0.592 |
| walker |  | 1027 | 96 | Markdown::ReadmeHeadline { file: docs/README.md } |  |  | 0.591 |
| ns | 1027 |  | 119 | Complete listings of peepdb/tests, docs, images, .github/workflows | 1.11 |  | 0.591 |
| walker |  | 1062 | 35 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.591 |
| walker |  | 1108 | 46 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.591 |
| walker |  | 1162 | 54 | Markdown::HeadingsOutline { file: docs/installation.md } |  |  | 0.591 |
| ns | 1209 |  | 182 | README section-heading map (every H2 and H3 after the feature list) | 1.12 | 1.1 | 0.628 |
| walker |  | 1289 | 127 | Toml::Config { file: project.toml } |  |  | 0.628 |
| walker |  | 1345 | 56 | Markdown::HeadingsOutline { file: docs/index.md } |  |  | 0.628 |
| walker |  | 1401 | 56 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.628 |
| ns | 1441 |  | 232 | `peepdb save` full option decorator block (cli.py 53-64) | 2.1 | 1.6 | 0.595 |
| walker |  | 1443 | 42 | Fs::DirListing { dir: peepdb/tests } |  |  | 0.660 |
| walker |  | 1569 | 126 | Plaintext::DeclSurface { file: docs/Gemfile } |  |  | 0.660 |
| ns | 1583 |  | 142 | `peepdb view` full option decorator block (cli.py 126-132) | 2.2 | 1.6 | 0.641 |
| ns | 1680 |  | 97 | Remaining Click decorators: group, version option, confirmations | 2.3 | 1.6 | 0.621 |
| walker |  | 1784 | 215 | Code::CodeKey { rung: Names, file: peepdb/db/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.681 |
| walker |  | 1881 | 97 | Markdown::HeadingsOutline { file: docs/usage.md } |  |  | 0.681 |
| walker |  | 1913 | 32 | Markdown::Section { file: docs/usage.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.681 |
| ns | 1922 |  | 242 | `view` docstring + connection resolution (cli.py 134-155) | 2.4 | 1.6 | 0.629 |
| walker |  | 1924 | 11 | Code::CodeKey { rung: Names, file: peepdb/exceptions.py, decl: 0, sub: 0, line: 0 } |  |  | 0.634 |
| walker |  | 1929 | 5 | Code::CodeKey { rung: Decl, file: peepdb/exceptions.py, decl: 1, sub: 0, line: 1 } |  |  | 0.637 |
| walker |  | 2040 | 111 | Code::CodeKey { rung: Names, file: peepdb/cli.py, decl: 0, sub: 0, line: 0 } |  |  | 0.679 |
| walker |  | 2048 | 8 | Code::CodeKey { rung: Decl, file: peepdb/cli.py, decl: 5, sub: 0, line: 86 } |  |  | 0.679 |
| walker |  | 2059 | 11 | Code::CodeKey { rung: Decl, file: peepdb/cli.py, decl: 1, sub: 0, line: 16 } |  |  | 0.679 |
| walker |  | 2076 | 17 | Code::CodeKey { rung: Decl, file: peepdb/cli.py, decl: 3, sub: 0, line: 25 } |  |  | 0.681 |
| walker |  | 2107 | 31 | Code::CodeKey { rung: Decl, file: peepdb/cli.py, decl: 7, sub: 0, line: 113 } |  |  | 0.690 |
| walker |  | 2148 | 41 | Code::CodeKey { rung: Decl, file: peepdb/cli.py, decl: 6, sub: 0, line: 97 } |  |  | 0.711 |
| walker |  | 2155 | 7 | Code::CodeKey { rung: Body, file: peepdb/cli.py, decl: 9, sub: 0, line: 181 } |  |  | 0.711 |
| walker |  | 2163 | 8 | Code::CodeKey { rung: Body, file: peepdb/cli.py, decl: 5, sub: 0, line: 86 } |  |  | 0.712 |
| ns | 2207 |  | 285 | `view` dispatch into peep_db + output rendering (cli.py 156-178) | 2.5 | 2.4 | 0.662 |
| walker |  | 2305 | 142 | Code::CodeKey { rung: Decl, file: peepdb/cli.py, decl: 8, sub: 0, line: 126 } |  |  | 0.687 |
| ns | 2425 |  | 218 | `save` docstring and body (cli.py 66-83) | 2.6 | 1.6 | 0.653 |
| ns | 2515 |  | 90 | Bodies of `list`, `remove` and `remove-all` (cli.py) | 2.7 | 1.6 | 0.636 |
| walker |  | 2537 | 232 | Code::CodeKey { rung: Decl, file: peepdb/cli.py, decl: 4, sub: 0, line: 53 } |  |  | 0.677 |
| walker |  | 2578 | 41 | Code::CodeKey { rung: Doc, file: peepdb/cli.py, decl: 5, sub: 0, line: 86 } |  |  | 0.677 |
| walker |  | 2623 | 45 | Code::CodeKey { rung: Doc, file: peepdb/cli.py, decl: 7, sub: 0, line: 113 } |  |  | 0.677 |
| walker |  | 2670 | 47 | Code::CodeKey { rung: Doc, file: peepdb/cli.py, decl: 6, sub: 0, line: 97 } |  |  | 0.677 |
| walker |  | 2722 | 52 | Code::CodeKey { rung: Doc, file: peepdb/cli.py, decl: 8, sub: 0, line: 126 } |  |  | 0.680 |
| ns | 2746 |  | 231 | BaseDatabase: class line + the four abstract methods + context manager | 3.1 |  | 0.641 |
| walker |  | 2788 | 66 | Code::CodeKey { rung: Doc, file: peepdb/cli.py, decl: 4, sub: 0, line: 53 } |  |  | 0.646 |
| ns | 2913 |  | 167 | BaseDatabase.__init__ and base.py imports | 3.2 | 3.1 | 0.625 |
| ns | 3026 |  | 113 | core.peep_db full signature (core.py 56-66) | 3.3 | 1.5 | 0.611 |
| walker |  | 3060 | 272 | Code::CodeKey { rung: Names, file: peepdb/config.py, decl: 0, sub: 0, line: 0 } |  |  | 0.653 |
| walker |  | 3078 | 18 | Code::CodeKey { rung: Decl, file: peepdb/config.py, decl: 8, sub: 0, line: 29 } |  |  | 0.653 |
| walker |  | 3098 | 20 | Code::CodeKey { rung: Decl, file: peepdb/config.py, decl: 9, sub: 0, line: 41 } |  |  | 0.653 |
| walker |  | 3128 | 30 | Code::CodeKey { rung: Decl, file: peepdb/config.py, decl: 1, sub: 0, line: 15 } |  |  | 0.654 |
| walker |  | 3143 | 15 | Code::CodeKey { rung: Body, file: peepdb/config.py, decl: 12, sub: 0, line: 70 } |  |  | 0.654 |
| walker |  | 3158 | 15 | Code::CodeKey { rung: Body, file: peepdb/config.py, decl: 13, sub: 0, line: 73 } |  |  | 0.654 |
| walker |  | 3185 | 27 | Markdown::Section { file: docs/installation.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.654 |
| ns | 3292 |  | 266 | connect_to_database: the first five engine branches (core.py 28-42) | 3.4 | 1.5 | 0.633 |
| walker |  | 3353 | 168 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.662 |
| walker |  | 3425 | 72 | Markdown::Section { file: docs/index.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.662 |
| ns | 3453 |  | 161 | connect_to_database: the irregular branches and the failure path (core.py 43-53) | 3.5 | 3.4 | 0.648 |
| walker |  | 3498 | 73 | Markdown::Section { file: docs/installation.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.648 |
| ns | 3651 |  | 198 | peep_db body: connect, fetch one table or all (core.py 67-79) | 3.6 | 3.3 | 0.632 |
| walker |  | 3652 | 154 | Markdown::HeadingsOutline { file: docs/README.md } |  |  | 0.632 |
| walker |  | 3672 | 20 | Markdown::Section { file: docs/README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.632 |
| walker |  | 3690 | 18 | Markdown::Section { file: docs/README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.632 |
| walker |  | 3748 | 58 | Markdown::Section { file: docs/README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.632 |
| ns | 3887 |  | 236 | peep_db body: table-vs-JSON post-processing and the finally-disconnect (core.py 80-98) | 3.7 | 3.6 | 0.611 |
| walker |  | 3963 | 215 | Markdown::Section { file: docs/README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.611 |
| walker |  | 4014 | 51 | Markdown::Section { file: docs/README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.611 |
| walker |  | 4132 | 118 | Code::CodeKey { rung: Names, file: peepdb/core.py, decl: 0, sub: 0, line: 0 } |  |  | 0.626 |
| ns | 4134 |  | 247 | format_value: numeric/date rendering rules (core.py 119-140) | 3.8 | 1.5 | 0.603 |
| walker |  | 4241 | 109 | Code::CodeKey { rung: Decl, file: peepdb/core.py, decl: 4, sub: 0, line: 56 } |  |  | 0.627 |
| walker |  | 4277 | 36 | Markdown::Section { file: docs/installation.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.627 |
| ns | 4335 |  | 201 | format_as_table: the tabulate grid layout (core.py 102-115) | 3.9 | 1.5 | 0.613 |
| walker |  | 4401 | 124 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.613 |
| walker |  | 4408 | 7 | Code::CodeKey { rung: Body, file: peepdb/cli.py, decl: 3, sub: 0, line: 25 } |  |  | 0.613 |
| walker |  | 4449 | 41 | Markdown::Section { file: docs/usage.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.613 |
| ns | 4459 |  | 124 | core.py import header and module logger setup | 3.10 |  | 0.601 |
| walker |  | 4463 | 14 | Code::CodeKey { rung: Names, file: peepdb/db/base.py, decl: 0, sub: 0, line: 0 } |  |  | 0.601 |
| walker |  | 4615 | 152 | Code::CodeKey { rung: Decl, file: peepdb/db/base.py, decl: 1, sub: 0, line: 5 } |  |  | 0.611 |
| walker |  | 4624 | 9 | Code::CodeKey { rung: Decl, file: peepdb/db/base.py, decl: 3, sub: 0, line: 17 } |  |  | 0.613 |
| walker |  | 4633 | 9 | Code::CodeKey { rung: Decl, file: peepdb/db/base.py, decl: 4, sub: 0, line: 21 } |  |  | 0.616 |
| walker |  | 4642 | 9 | Code::CodeKey { rung: Decl, file: peepdb/db/base.py, decl: 5, sub: 0, line: 25 } |  |  | 0.618 |
| walker |  | 4651 | 9 | Code::CodeKey { rung: Decl, file: peepdb/db/base.py, decl: 6, sub: 0, line: 29 } |  |  | 0.620 |
| walker |  | 4656 | 5 | Code::CodeKey { rung: Body, file: peepdb/db/base.py, decl: 3, sub: 0, line: 17 } |  |  | 0.622 |
| walker |  | 4661 | 5 | Code::CodeKey { rung: Body, file: peepdb/db/base.py, decl: 4, sub: 0, line: 21 } |  |  | 0.625 |
| walker |  | 4666 | 5 | Code::CodeKey { rung: Body, file: peepdb/db/base.py, decl: 5, sub: 0, line: 25 } |  |  | 0.627 |
| walker |  | 4671 | 5 | Code::CodeKey { rung: Body, file: peepdb/db/base.py, decl: 6, sub: 0, line: 29 } |  |  | 0.630 |
| walker |  | 4677 | 6 | Code::CodeKey { rung: Body, file: peepdb/db/base.py, decl: 8, sub: 0, line: 37 } |  |  | 0.633 |
| walker |  | 4691 | 14 | Code::CodeKey { rung: Names, file: peepdb/db/oracle.py, decl: 0, sub: 0, line: 0 } |  |  | 0.633 |
| walker |  | 4744 | 53 | Code::CodeKey { rung: Decl, file: peepdb/db/oracle.py, decl: 1, sub: 0, line: 7 } |  |  | 0.633 |
| walker |  | 4782 | 38 | Code::CodeKey { rung: Decl, file: peepdb/db/oracle.py, decl: 5, sub: 0, line: 36 } |  |  | 0.633 |
| walker |  | 4814 | 32 | Code::CodeKey { rung: Body, file: peepdb/db/oracle.py, decl: 4, sub: 0, line: 32 } |  |  | 0.633 |
| walker |  | 4867 | 53 | Code::CodeKey { rung: Body, file: peepdb/db/oracle.py, decl: 3, sub: 0, line: 25 } |  |  | 0.633 |
| walker |  | 4881 | 14 | Code::CodeKey { rung: Names, file: peepdb/db/sqlite.py, decl: 0, sub: 0, line: 0 } |  |  | 0.633 |
| walker |  | 4960 | 79 | Code::CodeKey { rung: Decl, file: peepdb/db/sqlite.py, decl: 1, sub: 0, line: 6 } |  |  | 0.633 |
| ns | 4964 |  | 505 | Every class and method name across all eight backend modules | 4.1 | 3.1 | 0.600 |
| walker |  | 5012 | 52 | Code::CodeKey { rung: Body, file: peepdb/db/sqlite.py, decl: 3, sub: 0, line: 22 } |  |  | 0.600 |
| walker |  | 5027 | 15 | Code::CodeKey { rung: Names, file: peepdb/db/mariadb.py, decl: 0, sub: 0, line: 0 } |  |  | 0.601 |
| walker |  | 5106 | 79 | Code::CodeKey { rung: Decl, file: peepdb/db/mariadb.py, decl: 1, sub: 0, line: 5 } |  |  | 0.605 |
| walker |  | 5137 | 31 | Code::CodeKey { rung: Body, file: peepdb/db/mariadb.py, decl: 4, sub: 0, line: 30 } |  |  | 0.605 |
| walker |  | 5191 | 54 | Code::CodeKey { rung: Body, file: peepdb/db/mariadb.py, decl: 3, sub: 0, line: 23 } |  |  | 0.605 |
| walker |  | 5206 | 15 | Code::CodeKey { rung: Names, file: peepdb/db/mongodb.py, decl: 0, sub: 0, line: 0 } |  |  | 0.606 |
| walker |  | 5260 | 54 | Code::CodeKey { rung: Decl, file: peepdb/db/mongodb.py, decl: 1, sub: 0, line: 8 } |  |  | 0.611 |
| ns | 5263 |  | 299 | The table/collection-listing query of every backend | 4.2 | 4.1 | 0.594 |
| walker |  | 5315 | 55 | Code::CodeKey { rung: Decl, file: peepdb/db/mongodb.py, decl: 5, sub: 0, line: 42 } |  |  | 0.594 |
| walker |  | 5325 | 10 | Code::CodeKey { rung: Body, file: peepdb/db/mongodb.py, decl: 4, sub: 0, line: 39 } |  |  | 0.595 |
| walker |  | 5351 | 26 | Code::CodeKey { rung: Doc, file: peepdb/db/mongodb.py, decl: 5, sub: 0, line: 42 } |  |  | 0.595 |
| walker |  | 5366 | 15 | Code::CodeKey { rung: Names, file: peepdb/db/mssql.py, decl: 0, sub: 0, line: 0 } |  |  | 0.597 |
| walker |  | 5491 | 125 | Code::CodeKey { rung: Decl, file: peepdb/db/mssql.py, decl: 1, sub: 0, line: 6 } |  |  | 0.609 |
| walker |  | 5536 | 45 | Code::CodeKey { rung: Body, file: peepdb/db/mssql.py, decl: 2, sub: 0, line: 7 } |  |  | 0.609 |
| walker |  | 5590 | 54 | Code::CodeKey { rung: Body, file: peepdb/db/mssql.py, decl: 4, sub: 0, line: 28 } |  |  | 0.609 |
| ns | 5595 |  | 332 | MySQLDatabase — the canonical backend implementation | 4.3 | 4.1 | 0.588 |
| walker |  | 5605 | 15 | Code::CodeKey { rung: Names, file: peepdb/db/mysql.py, decl: 0, sub: 0, line: 0 } |  |  | 0.590 |
| walker |  | 5684 | 79 | Code::CodeKey { rung: Decl, file: peepdb/db/mysql.py, decl: 1, sub: 0, line: 5 } |  |  | 0.598 |
| walker |  | 5718 | 34 | Code::CodeKey { rung: Body, file: peepdb/db/mysql.py, decl: 4, sub: 0, line: 29 } |  |  | 0.600 |
| walker |  | 5772 | 54 | Code::CodeKey { rung: Body, file: peepdb/db/mysql.py, decl: 3, sub: 0, line: 22 } |  |  | 0.600 |
| walker |  | 5787 | 15 | Code::CodeKey { rung: Names, file: peepdb/db/postgresql.py, decl: 0, sub: 0, line: 0 } |  |  | 0.603 |
| walker |  | 5866 | 79 | Code::CodeKey { rung: Decl, file: peepdb/db/postgresql.py, decl: 1, sub: 0, line: 6 } |  |  | 0.612 |
| ns | 5897 |  | 302 | PostgreSQL, MariaDB and Oracle connect() — drivers, default ports, cursor factories | 4.4 | 4.1 | 0.592 |
| walker |  | 5906 | 40 | Code::CodeKey { rung: Body, file: peepdb/db/postgresql.py, decl: 4, sub: 0, line: 30 } |  |  | 0.595 |
| walker |  | 5960 | 54 | Code::CodeKey { rung: Body, file: peepdb/db/postgresql.py, decl: 3, sub: 0, line: 23 } |  |  | 0.595 |
| walker |  | 6008 | 48 | Code::CodeKey { rung: Body, file: peepdb/db/mongodb.py, decl: 3, sub: 0, line: 32 } |  |  | 0.595 |
| walker |  | 6039 | 31 | Code::CodeKey { rung: Names, file: peepdb/db/firebase.py, decl: 0, sub: 0, line: 0 } |  |  | 0.598 |
| walker |  | 6135 | 96 | Code::CodeKey { rung: Decl, file: peepdb/db/firebase.py, decl: 2, sub: 0, line: 9 } |  |  | 0.613 |
| walker |  | 6156 | 21 | Code::CodeKey { rung: Body, file: peepdb/db/firebase.py, decl: 5, sub: 0, line: 26 } |  |  | 0.599 |
| ns | 6156 |  | 259 | MongoDBDatabase.connect — URI construction from the generic parameters | 4.5 | 4.1 | 0.599 |
| walker |  | 6189 | 33 | Code::CodeKey { rung: Body, file: peepdb/db/firebase.py, decl: 3, sub: 0, line: 10 } |  |  | 0.599 |
| walker |  | 6270 | 81 | Code::CodeKey { rung: Body, file: peepdb/db/firebase.py, decl: 6, sub: 0, line: 30 } |  |  | 0.606 |
| walker |  | 6320 | 50 | Markdown::Section { file: docs/index.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.606 |
| walker |  | 6397 | 77 | Code::CodeKey { rung: Body, file: peepdb/config.py, decl: 9, sub: 0, line: 41 } |  |  | 0.606 |
| ns | 6435 |  | 279 | MSSQLDatabase: constructor extras and the ODBC connection string | 4.6 | 4.1 | 0.596 |
| walker |  | 6452 | 55 | Markdown::Section { file: docs/usage.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.596 |
| walker |  | 6555 | 103 | Code::CodeKey { rung: Body, file: peepdb/db/mssql.py, decl: 5, sub: 0, line: 35 } |  |  | 0.606 |
| ns | 6582 |  | 147 | FirebaseDatabase: the backend that does not call super().__init__ | 4.7 | 4.1 | 0.599 |
| walker |  | 6756 | 201 | Code::CodeKey { rung: Body, file: peepdb/core.py, decl: 5, sub: 0, line: 101 } |  |  | 0.618 |
| walker |  | 6770 | 14 | Code::CodeKey { rung: Body, file: peepdb/db/base.py, decl: 7, sub: 0, line: 33 } |  |  | 0.623 |
| walker |  | 6834 | 64 | Markdown::Section { file: docs/usage.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.623 |
| walker |  | 6898 | 64 | Markdown::Section { file: docs/usage.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.623 |
| ns | 6899 |  | 317 | SQLiteDatabase connect and fetch_data — file-path handling and print-based logging | 4.8 | 4.1 | 0.610 |
| walker |  | 7130 | 232 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.610 |
| ns | 7201 |  | 302 | The non-LIMIT/OFFSET pagination variants (MSSQL, Oracle, MongoDB, Firebase) | 4.9 | 4.1 | 0.597 |
| ns | 7310 |  | 109 | Where credentials live: config paths and KeySecurity modes | 5.1 | 1.9 | 0.601 |
| walker |  | 7331 | 201 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.601 |
| walker |  | 7453 | 122 | Markdown::Section { file: docs/README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.601 |
| ns | 7495 |  | 185 | get_key_security_config / get_key / encrypt / decrypt bodies | 5.2 | 1.9 | 0.593 |
| walker |  | 7520 | 67 | Markdown::Section { file: docs/installation.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.593 |
| ns | 7742 |  | 247 | The two key sources: PBKDF2 derivation and keyring fetch-or-create | 5.3 | 1.9 | 0.588 |
| walker |  | 7767 | 247 | Code::CodeKey { rung: Body, file: peepdb/core.py, decl: 6, sub: 0, line: 118 } |  |  | 0.616 |
| ns | 7987 |  | 245 | save_connection: the on-disk record shape and what is encrypted | 5.4 | 1.9 | 0.603 |
| walker |  | 8044 | 277 | Code::CodeKey { rung: Doc, file: peepdb/cli.py, decl: 3, sub: 0, line: 25 } |  |  | 0.604 |
| ns | 8167 |  | 180 | get_connection: the tuple contract and InvalidPassword | 5.5 | 1.9 | 0.594 |
| ns | 8309 |  | 142 | add_key_security: first-run choice between keyring and password | 5.6 | 1.9 | 0.589 |
| ns | 8403 |  | 94 | list/remove/remove_all_connections — the distinctive lines only | 5.7 | 1.9 | 0.584 |
| ns | 8760 |  | 357 | Every test (and fixture) name in peepdb/tests, across all six modules | 6.1 | 1.11 | 0.571 |
| ns | 8866 |  | 106 | How the suite is run: CONTRIBUTING instructions and the CI invocation | 6.2 |  | 0.567 |
| walker |  | 8911 | 867 | Plaintext::Whole { file: requirements.txt } |  |  | 0.567 |
| walker |  | 9065 | 154 | Code::CodeKey { rung: Body, file: peepdb/db/sqlite.py, decl: 4, sub: 0, line: 29 } |  |  | 0.575 |
| ns | 9081 |  | 215 | A representative test body: patching style and asserted output strings | 6.3 | 6.1 | 0.568 |
| ns | 9171 |  | 90 | `peepdb --help` summary lines from the cli group docstring | 7.1 | 1.6 | 0.571 |
| walker |  | 9220 | 155 | Code::CodeKey { rung: Body, file: peepdb/db/mysql.py, decl: 2, sub: 0, line: 6 } |  |  | 0.580 |
| walker |  | 9325 | 105 | Code::CodeKey { rung: Body, file: peepdb/config.py, decl: 11, sub: 0, line: 60 } |  |  | 0.587 |
| ns | 9427 |  | 256 | The two disagreeing dependency lists (setup.py vs project.toml) | 7.2 |  | 0.582 |
| walker |  | 9487 | 162 | Code::CodeKey { rung: Body, file: peepdb/db/mariadb.py, decl: 5, sub: 0, line: 34 } |  |  | 0.582 |
| ns | 9564 |  | 137 | Build backend, Python floor, packaging includes | 7.3 | 1.1 | 0.584 |
| walker |  | 9649 | 162 | Code::CodeKey { rung: Body, file: peepdb/db/postgresql.py, decl: 5, sub: 0, line: 34 } |  |  | 0.584 |
| ns | 9752 |  | 188 | CI: what triggers the workflows and on what Python versions | 7.4 |  | 0.577 |
| walker |  | 9763 | 114 | Code::CodeKey { rung: Body, file: peepdb/db/firebase.py, decl: 4, sub: 0, line: 15 } |  |  | 0.588 |
| walker |  | 9851 | 88 | Markdown::Section { file: docs/index.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.588 |
| ns | 9913 |  | 161 | The docs/ site: Jekyll theme config, and two stale-template markers | 7.5 | 1.11 | 0.587 |
| ns | 9978 |  | 65 | CustomEncoder.default — the JSON serializer for Decimal and date | 7.6 | 1.6 | 0.585 |

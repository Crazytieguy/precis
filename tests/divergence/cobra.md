Score(3000)=0.629 I=0.780 C=0.508 ns_rows≤3K=19/50 grid(1000/1442/2080/3000/4327/6240/9000)=0.596/0.797/0.741/0.629/0.580/0.526/0.500

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 98 |  | 98 | Identity: the README lede and the package doc comment | 1.1 |  | 0.000 |
| walker |  | 191 | 191 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 194 | 3 | Fs::DirListing { dir: site } |  |  | 0.000 |
| walker |  | 199 | 5 | Fs::DirListing { dir: assets } |  |  | 0.000 |
| ns | 227 |  | 129 | go.mod: module path, Go version floor, and the four dependencies | 1.2 |  | 0.000 |
| walker |  | 229 | 30 | GoMod::Identity { file: go.mod } |  |  | 0.121 |
| walker |  | 288 | 59 | Fs::DirListing { dir: doc } |  |  | 0.124 |
| walker |  | 314 | 26 | Fs::DirListing { dir: site/content } |  |  | 0.125 |
| walker |  | 362 | 48 | Code::CodeKey { rung: ModuleDoc, file: command.go, decl: 0, sub: 0, line: 0 } |  |  | 0.243 |
| ns | 366 |  | 139 | Complete .go roster of the root package: sources and colocated tests | 1.3 |  | 0.509 |
| walker |  | 375 | 13 | Fs::DirListing { dir: .github } |  |  | 0.515 |
| walker |  | 384 | 9 | Fs::DirListing { dir: .github/workflows } |  |  | 0.519 |
| walker |  | 480 | 96 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.764 |
| walker |  | 501 | 21 | Fs::DirListing { dir: site/content/docgen } |  |  | 0.768 |
| ns | 507 |  | 141 | README 'Overview': what the library does, first half | 1.4 |  | 0.670 |
| walker |  | 524 | 23 | Fs::DirListing { dir: site/content/completions } |  |  | 0.675 |
| walker |  | 587 | 63 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.675 |
| walker |  | 599 | 12 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.675 |
| ns | 724 |  | 217 | README: the remaining capabilities, and who builds on Cobra | 1.5 | 1.4 | 0.604 |
| ns | 926 |  | 202 | Canonical usage: the rootCmd literal and Execute() from the user guide | 1.6 |  | 0.520 |
| ns | 978 |  | 52 | Complete non-Go root listing: build, config, docs, governance | 1.7 |  | 0.577 |
| walker |  | 984 | 385 | Plaintext::Whole { file: Makefile } |  |  | 0.596 |
| walker |  | 1195 | 211 | Markdown::Prelude { file: README.md } |  |  | 0.605 |
| ns | 1218 |  | 240 | Makefile: every target and its recipe | 1.8 |  | 0.651 |
| walker |  | 1294 | 99 | GoMod::File { file: go.mod } |  |  | 0.776 |
| ns | 1304 |  | 86 | doc/ subpackage, CI workflows, and assets listings | 1.9 |  | 0.788 |
| ns | 1374 |  | 70 | Documentation site tree: site/content and its two subdirectories | 1.10 |  | 0.795 |
| walker |  | 1535 | 241 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 0, line: 0 } |  |  | 0.797 |
| walker |  | 1546 | 11 | Code::CodeKey { rung: Decl, file: command.go, decl: 1, sub: 0, line: 33 } |  |  | 0.798 |
| walker |  | 1567 | 21 | Code::CodeKey { rung: Decl, file: command.go, decl: 3, sub: 0, line: 45 } |  |  | 0.799 |
| walker |  | 1575 | 8 | Code::CodeKey { rung: Body, file: command.go, decl: 5, sub: 0, line: 269 } |  |  | 0.799 |
| walker |  | 1584 | 9 | Code::CodeKey { rung: Body, file: command.go, decl: 6, sub: 0, line: 275 } |  |  | 0.799 |
| walker |  | 1595 | 11 | Code::CodeKey { rung: Doc, file: command.go, decl: 3, sub: 0, line: 45 } |  |  | 0.800 |
| walker |  | 1611 | 16 | Code::CodeKey { rung: Doc, file: command.go, decl: 2, sub: 0, line: 42 } |  |  | 0.801 |
| ns | 1612 |  | 238 | command.go: exported constants, FParseErrWhitelist, Group, Command's doc | 2.1 |  | 0.776 |
| walker |  | 1620 | 9 | Code::CodeKey { rung: Body, file: command.go, decl: 7, sub: 0, line: 281 } |  |  | 0.776 |
| walker |  | 1651 | 31 | Code::CodeKey { rung: Doc, file: command.go, decl: 9, sub: 0, line: 296 } |  |  | 0.776 |
| walker |  | 1682 | 31 | Code::CodeKey { rung: Doc, file: command.go, decl: 10, sub: 0, line: 302 } |  |  | 0.776 |
| ns | 1684 |  | 72 | Command fields: naming and help text (Use .. Example) | 2.2 | 2.1 | 0.758 |
| walker |  | 1713 | 31 | Code::CodeKey { rung: Doc, file: command.go, decl: 11, sub: 0, line: 308 } |  |  | 0.758 |
| walker |  | 1747 | 34 | Code::CodeKey { rung: Doc, file: command.go, decl: 6, sub: 0, line: 275 } |  |  | 0.758 |
| ns | 1776 |  | 92 | Command fields: argument validation, completion, metadata | 2.3 | 2.1 | 0.738 |
| walker |  | 1789 | 42 | Code::CodeKey { rung: Doc, file: command.go, decl: 7, sub: 0, line: 281 } |  |  | 0.738 |
| walker |  | 1842 | 53 | Code::CodeKey { rung: Doc, file: command.go, decl: 8, sub: 0, line: 289 } |  |  | 0.738 |
| walker |  | 2055 | 213 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 1, line: 0 } |  |  | 0.741 |
| walker |  | 2067 | 12 | Code::CodeKey { rung: Doc, file: command.go, decl: 16, sub: 0, line: 338 } |  |  | 0.741 |
| walker |  | 2085 | 18 | Code::CodeKey { rung: Doc, file: command.go, decl: 13, sub: 0, line: 318 } |  |  | 0.741 |
| walker |  | 2103 | 18 | Code::CodeKey { rung: Doc, file: command.go, decl: 15, sub: 0, line: 333 } |  |  | 0.741 |
| walker |  | 2122 | 19 | Code::CodeKey { rung: Doc, file: command.go, decl: 12, sub: 0, line: 313 } |  |  | 0.741 |
| ns | 2125 |  | 349 | Command fields: the ten *Run hooks with the documented ordering comment | 2.4 | 2.1 | 0.696 |
| walker |  | 2141 | 19 | Code::CodeKey { rung: Doc, file: command.go, decl: 17, sub: 0, line: 343 } |  |  | 0.696 |
| walker |  | 2160 | 19 | Code::CodeKey { rung: Doc, file: command.go, decl: 18, sub: 0, line: 352 } |  |  | 0.696 |
| walker |  | 2184 | 24 | Code::CodeKey { rung: Doc, file: command.go, decl: 19, sub: 0, line: 358 } |  |  | 0.696 |
| walker |  | 2208 | 24 | Code::CodeKey { rung: Doc, file: command.go, decl: 20, sub: 0, line: 367 } |  |  | 0.696 |
| walker |  | 2233 | 25 | Code::CodeKey { rung: Doc, file: command.go, decl: 21, sub: 0, line: 376 } |  |  | 0.696 |
| walker |  | 2260 | 27 | Code::CodeKey { rung: Doc, file: command.go, decl: 14, sub: 0, line: 328 } |  |  | 0.696 |
| ns | 2264 |  | 139 | Command fields: behaviour toggles, with ellipses over the unexported state | 2.5 | 2.1 | 0.670 |
| ns | 2372 |  | 108 | Command method roster: execution and lifecycle | 3.1 |  | 0.655 |
| walker |  | 2487 | 227 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 2, line: 0 } |  |  | 0.657 |
| walker |  | 2501 | 14 | Code::CodeKey { rung: Doc, file: command.go, decl: 23, sub: 0, line: 393 } |  |  | 0.657 |
| ns | 2514 |  | 142 | Command method roster: building and traversing the command tree | 3.2 |  | 0.636 |
| walker |  | 2515 | 14 | Code::CodeKey { rung: Doc, file: command.go, decl: 24, sub: 0, line: 398 } |  |  | 0.636 |
| walker |  | 2529 | 14 | Code::CodeKey { rung: Doc, file: command.go, decl: 25, sub: 0, line: 403 } |  |  | 0.636 |
| walker |  | 2543 | 14 | Code::CodeKey { rung: Doc, file: command.go, decl: 26, sub: 0, line: 408 } |  |  | 0.636 |
| walker |  | 2574 | 31 | Code::CodeKey { rung: Doc, file: command.go, decl: 31, sub: 0, line: 464 } |  |  | 0.636 |
| walker |  | 2611 | 37 | Code::CodeKey { rung: Doc, file: command.go, decl: 30, sub: 0, line: 444 } |  |  | 0.636 |
| walker |  | 2650 | 39 | Code::CodeKey { rung: Doc, file: command.go, decl: 22, sub: 0, line: 382 } |  |  | 0.636 |
| ns | 2740 |  | 226 | Command method roster: the complete flag API | 3.3 |  | 0.609 |
| walker |  | 2888 | 238 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 3, line: 0 } |  |  | 0.610 |
| walker |  | 2899 | 11 | Code::CodeKey { rung: Doc, file: command.go, decl: 36, sub: 0, line: 526 } |  |  | 0.610 |
| walker |  | 2910 | 11 | Code::CodeKey { rung: Doc, file: command.go, decl: 39, sub: 0, line: 563 } |  |  | 0.610 |
| walker |  | 2921 | 11 | Code::CodeKey { rung: Doc, file: command.go, decl: 43, sub: 0, line: 583 } |  |  | 0.610 |
| walker |  | 2934 | 13 | Code::CodeKey { rung: Doc, file: command.go, decl: 41, sub: 0, line: 573 } |  |  | 0.610 |
| ns | 2955 |  | 215 | Command method roster: I/O wiring and the Set* customisation hooks | 3.4 |  | 0.629 |
| walker |  | 2964 | 30 | Code::CodeKey { rung: Doc, file: command.go, decl: 44, sub: 0, line: 592 } |  |  | 0.629 |
| walker |  | 2995 | 31 | Code::CodeKey { rung: Doc, file: command.go, decl: 34, sub: 0, line: 505 } |  |  | 0.629 |
| walker |  | 3034 | 39 | Code::CodeKey { rung: Doc, file: command.go, decl: 33, sub: 0, line: 484 } |  |  | 0.629 |
| walker |  | 3076 | 42 | Code::CodeKey { rung: Doc, file: command.go, decl: 32, sub: 0, line: 478 } |  |  | 0.629 |
| walker |  | 3119 | 43 | Code::CodeKey { rung: Doc, file: command.go, decl: 35, sub: 0, line: 520 } |  |  | 0.629 |
| ns | 3142 |  | 187 | Command method roster: help, usage, templates, default-command injection | 3.5 |  | 0.622 |
| walker |  | 3167 | 48 | Code::CodeKey { rung: Doc, file: command.go, decl: 37, sub: 0, line: 547 } |  |  | 0.622 |
| ns | 3332 |  | 190 | Command method roster: naming and introspection predicates | 3.6 |  | 0.603 |
| walker |  | 3379 | 212 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 4, line: 0 } |  |  | 0.612 |
| walker |  | 3394 | 15 | Code::CodeKey { rung: Doc, file: command.go, decl: 48, sub: 0, line: 643 } |  |  | 0.612 |
| walker |  | 3424 | 30 | Code::CodeKey { rung: Doc, file: command.go, decl: 45, sub: 0, line: 605 } |  |  | 0.612 |
| walker |  | 3454 | 30 | Code::CodeKey { rung: Doc, file: command.go, decl: 46, sub: 0, line: 618 } |  |  | 0.612 |
| walker |  | 3485 | 31 | Code::CodeKey { rung: Doc, file: command.go, decl: 47, sub: 0, line: 631 } |  |  | 0.612 |
| ns | 3516 |  | 184 | user_guide.md: complete H2 section map | 4.1 |  | 0.595 |
| walker |  | 3520 | 35 | Code::CodeKey { rung: Doc, file: command.go, decl: 54, sub: 0, line: 757 } |  |  | 0.595 |
| walker |  | 3589 | 69 | Code::CodeKey { rung: Doc, file: command.go, decl: 52, sub: 0, line: 715 } |  |  | 0.595 |
| ns | 3754 |  | 238 | user_guide.md: complete H3/H4 subsection map | 4.2 | 4.1 | 0.574 |
| walker |  | 3809 | 220 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 5, line: 0 } |  |  | 0.581 |
| walker |  | 3819 | 10 | Code::CodeKey { rung: Doc, file: command.go, decl: 60, sub: 0, line: 892 } |  |  | 0.581 |
| walker |  | 3833 | 14 | Code::CodeKey { rung: Doc, file: command.go, decl: 58, sub: 0, line: 863 } |  |  | 0.581 |
| walker |  | 3853 | 20 | Code::CodeKey { rung: Doc, file: command.go, decl: 59, sub: 0, line: 884 } |  |  | 0.581 |
| walker |  | 3880 | 27 | Code::CodeKey { rung: Doc, file: command.go, decl: 57, sub: 0, line: 821 } |  |  | 0.581 |
| walker |  | 3916 | 36 | Code::CodeKey { rung: Doc, file: command.go, decl: 61, sub: 0, line: 901 } |  |  | 0.581 |
| ns | 3950 |  | 196 | Completion and Active Help documentation: H1/H2 maps | 4.3 |  | 0.566 |
| walker |  | 3968 | 52 | Code::CodeKey { rung: Doc, file: command.go, decl: 65, sub: 0, line: 1062 } |  |  | 0.566 |
| ns | 4133 |  | 183 | Test harness: the helpers every root-package test is written against | 4.4 |  | 0.551 |
| walker |  | 4212 | 244 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 6, line: 0 } |  |  | 0.580 |
| walker |  | 4224 | 12 | Code::CodeKey { rung: Doc, file: command.go, decl: 68, sub: 0, line: 1084 } |  |  | 0.580 |
| walker |  | 4237 | 13 | Code::CodeKey { rung: Doc, file: command.go, decl: 76, sub: 0, line: 1325 } |  |  | 0.580 |
| walker |  | 4256 | 19 | Code::CodeKey { rung: Doc, file: command.go, decl: 75, sub: 0, line: 1317 } |  |  | 0.580 |
| walker |  | 4277 | 21 | Code::CodeKey { rung: Doc, file: command.go, decl: 70, sub: 0, line: 1180 } |  |  | 0.580 |
| walker |  | 4323 | 46 | Code::CodeKey { rung: Doc, file: command.go, decl: 71, sub: 0, line: 1205 } |  |  | 0.580 |
| walker |  | 4371 | 48 | Code::CodeKey { rung: Doc, file: command.go, decl: 66, sub: 0, line: 1070 } |  |  | 0.580 |
| walker |  | 4425 | 54 | Code::CodeKey { rung: Doc, file: command.go, decl: 67, sub: 0, line: 1078 } |  |  | 0.581 |
| ns | 4458 |  | 325 | CI: the four jobs and the platform / Go-version matrix | 4.5 |  | 0.558 |
| walker |  | 4481 | 56 | Code::CodeKey { rung: Doc, file: command.go, decl: 72, sub: 0, line: 1219 } |  |  | 0.558 |
| walker |  | 4543 | 62 | Code::CodeKey { rung: Doc, file: command.go, decl: 74, sub: 0, line: 1263 } |  |  | 0.558 |
| walker |  | 4611 | 68 | Code::CodeKey { rung: Doc, file: command.go, decl: 73, sub: 0, line: 1238 } |  |  | 0.558 |
| ns | 4706 |  | 248 | golangci-lint configuration: the enabled linter set | 4.6 |  | 0.537 |
| walker |  | 4823 | 212 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 7, line: 0 } |  |  | 0.555 |
| walker |  | 4836 | 13 | Code::CodeKey { rung: Doc, file: command.go, decl: 80, sub: 0, line: 1332 } |  |  | 0.555 |
| walker |  | 4851 | 15 | Code::CodeKey { rung: Doc, file: command.go, decl: 82, sub: 0, line: 1371 } |  |  | 0.555 |
| walker |  | 4869 | 18 | Code::CodeKey { rung: Doc, file: command.go, decl: 81, sub: 0, line: 1342 } |  |  | 0.555 |
| walker |  | 4887 | 18 | Code::CodeKey { rung: Doc, file: command.go, decl: 86, sub: 0, line: 1401 } |  |  | 0.555 |
| walker |  | 4906 | 19 | Code::CodeKey { rung: Doc, file: command.go, decl: 85, sub: 0, line: 1396 } |  |  | 0.555 |
| walker |  | 4926 | 20 | Code::CodeKey { rung: Doc, file: command.go, decl: 84, sub: 0, line: 1386 } |  |  | 0.555 |
| walker |  | 4948 | 22 | Code::CodeKey { rung: Doc, file: command.go, decl: 83, sub: 0, line: 1376 } |  |  | 0.555 |
| ns | 5045 |  | 339 | Execute / ExecuteC: doc comments and the root-redirect rule | 5.1 | 3.1 | 0.549 |
| walker |  | 5183 | 235 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 8, line: 0 } |  |  | 0.552 |
| walker |  | 5199 | 16 | Code::CodeKey { rung: Doc, file: command.go, decl: 93, sub: 0, line: 1465 } |  |  | 0.552 |
| walker |  | 5219 | 20 | Code::CodeKey { rung: Doc, file: command.go, decl: 98, sub: 0, line: 1551 } |  |  | 0.552 |
| walker |  | 5240 | 21 | Code::CodeKey { rung: Doc, file: command.go, decl: 95, sub: 0, line: 1482 } |  |  | 0.552 |
| walker |  | 5261 | 21 | Code::CodeKey { rung: Doc, file: command.go, decl: 97, sub: 0, line: 1541 } |  |  | 0.552 |
| walker |  | 5287 | 26 | Code::CodeKey { rung: Doc, file: command.go, decl: 87, sub: 0, line: 1435 } |  |  | 0.552 |
| walker |  | 5315 | 28 | Code::CodeKey { rung: Doc, file: command.go, decl: 88, sub: 0, line: 1440 } |  |  | 0.552 |
| walker |  | 5343 | 28 | Code::CodeKey { rung: Doc, file: command.go, decl: 89, sub: 0, line: 1445 } |  |  | 0.552 |
| walker |  | 5371 | 28 | Code::CodeKey { rung: Doc, file: command.go, decl: 90, sub: 0, line: 1450 } |  |  | 0.552 |
| walker |  | 5401 | 30 | Code::CodeKey { rung: Doc, file: command.go, decl: 91, sub: 0, line: 1455 } |  |  | 0.552 |
| walker |  | 5431 | 30 | Code::CodeKey { rung: Doc, file: command.go, decl: 92, sub: 0, line: 1460 } |  |  | 0.552 |
| walker |  | 5461 | 30 | Code::CodeKey { rung: Doc, file: command.go, decl: 96, sub: 0, line: 1501 } |  |  | 0.552 |
| walker |  | 5498 | 37 | Code::CodeKey { rung: Doc, file: command.go, decl: 94, sub: 0, line: 1474 } |  |  | 0.552 |
| ns | 5583 |  | 538 | execute(): the ordered run pipeline | 5.2 | 5.1 | 0.520 |
| walker |  | 5742 | 244 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 9, line: 0 } |  |  | 0.541 |
| walker |  | 5757 | 15 | Code::CodeKey { rung: Doc, file: command.go, decl: 102, sub: 0, line: 1591 } |  |  | 0.541 |
| walker |  | 5772 | 15 | Code::CodeKey { rung: Doc, file: command.go, decl: 103, sub: 0, line: 1596 } |  |  | 0.541 |
| walker |  | 5789 | 17 | Code::CodeKey { rung: Doc, file: command.go, decl: 104, sub: 0, line: 1601 } |  |  | 0.541 |
| walker |  | 5806 | 17 | Code::CodeKey { rung: Doc, file: command.go, decl: 109, sub: 0, line: 1677 } |  |  | 0.541 |
| ns | 5809 |  | 226 | cobra.go: the package-level behaviour switches | 5.3 |  | 0.532 |
| walker |  | 5826 | 20 | Code::CodeKey { rung: Doc, file: command.go, decl: 101, sub: 0, line: 1586 } |  |  | 0.532 |
| walker |  | 5847 | 21 | Code::CodeKey { rung: Doc, file: command.go, decl: 110, sub: 0, line: 1682 } |  |  | 0.532 |
| walker |  | 5878 | 31 | Code::CodeKey { rung: Doc, file: command.go, decl: 100, sub: 0, line: 1571 } |  |  | 0.532 |
| ns | 5891 |  | 82 | cobra.go: package-level function roster | 5.4 |  | 0.527 |
| walker |  | 5915 | 37 | Code::CodeKey { rung: Doc, file: command.go, decl: 105, sub: 0, line: 1607 } |  |  | 0.527 |
| walker |  | 5955 | 40 | Code::CodeKey { rung: Doc, file: command.go, decl: 99, sub: 0, line: 1562 } |  |  | 0.527 |
| walker |  | 5996 | 41 | Code::CodeKey { rung: Doc, file: command.go, decl: 108, sub: 0, line: 1662 } |  |  | 0.527 |
| walker |  | 6049 | 53 | Code::CodeKey { rung: Doc, file: command.go, decl: 107, sub: 0, line: 1648 } |  |  | 0.527 |
| ns | 6188 |  | 297 | The default usage template, plus the help and version template constants | 5.5 |  | 0.519 |
| walker |  | 6289 | 240 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 10, line: 0 } |  |  | 0.531 |
| walker |  | 6303 | 14 | Code::CodeKey { rung: Doc, file: command.go, decl: 117, sub: 0, line: 1787 } |  |  | 0.531 |
| walker |  | 6320 | 17 | Code::CodeKey { rung: Doc, file: command.go, decl: 119, sub: 0, line: 1806 } |  |  | 0.531 |
| walker |  | 6339 | 19 | Code::CodeKey { rung: Doc, file: command.go, decl: 120, sub: 0, line: 1811 } |  |  | 0.531 |
| walker |  | 6359 | 20 | Code::CodeKey { rung: Doc, file: command.go, decl: 116, sub: 0, line: 1775 } |  |  | 0.531 |
| walker |  | 6380 | 21 | Code::CodeKey { rung: Doc, file: command.go, decl: 121, sub: 0, line: 1816 } |  |  | 0.531 |
| walker |  | 6404 | 24 | Code::CodeKey { rung: Doc, file: command.go, decl: 118, sub: 0, line: 1801 } |  |  | 0.531 |
| walker |  | 6439 | 35 | Code::CodeKey { rung: Doc, file: command.go, decl: 111, sub: 0, line: 1688 } |  |  | 0.531 |
| walker |  | 6478 | 39 | Code::CodeKey { rung: Doc, file: command.go, decl: 122, sub: 0, line: 1822 } |  |  | 0.531 |
| walker |  | 6524 | 46 | Code::CodeKey { rung: Doc, file: command.go, decl: 114, sub: 0, line: 1744 } |  |  | 0.531 |
| walker |  | 6571 | 47 | Code::CodeKey { rung: Doc, file: command.go, decl: 113, sub: 0, line: 1716 } |  |  | 0.531 |
| walker |  | 6618 | 47 | Code::CodeKey { rung: Doc, file: command.go, decl: 115, sub: 0, line: 1770 } |  |  | 0.531 |
| ns | 6635 |  | 447 | args.go: every PositionalArgs validator with its defining doc line | 6.1 |  | 0.519 |
| walker |  | 6669 | 51 | Code::CodeKey { rung: Doc, file: command.go, decl: 112, sub: 0, line: 1702 } |  |  | 0.519 |
| ns | 6898 |  | 263 | flag_groups.go: the three group markers and their annotation keys | 6.2 |  | 0.512 |
| walker |  | 6905 | 236 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 11, line: 0 } |  |  | 0.526 |
| walker |  | 6933 | 28 | Code::CodeKey { rung: Decl, file: command.go, decl: 133, sub: 0, line: 1937 } |  |  | 0.526 |
| walker |  | 6946 | 13 | Code::CodeKey { rung: Doc, file: command.go, decl: 127, sub: 0, line: 1855 } |  |  | 0.526 |
| walker |  | 6959 | 13 | Code::CodeKey { rung: Doc, file: command.go, decl: 129, sub: 0, line: 1892 } |  |  | 0.526 |
| walker |  | 6975 | 16 | Code::CodeKey { rung: Doc, file: command.go, decl: 128, sub: 0, line: 1868 } |  |  | 0.526 |
| walker |  | 6992 | 17 | Code::CodeKey { rung: Doc, file: command.go, decl: 126, sub: 0, line: 1844 } |  |  | 0.526 |
| walker |  | 7012 | 20 | Code::CodeKey { rung: Doc, file: command.go, decl: 133, sub: 0, line: 1937 } |  |  | 0.526 |
| ns | 7027 |  | 129 | shell_completions.go: the complete Mark* helper family | 6.3 |  | 0.520 |
| walker |  | 7036 | 24 | Code::CodeKey { rung: Doc, file: command.go, decl: 123, sub: 0, line: 1827 } |  |  | 0.520 |
| walker |  | 7069 | 33 | Code::CodeKey { rung: Doc, file: command.go, decl: 124, sub: 0, line: 1833 } |  |  | 0.520 |
| walker |  | 7102 | 33 | Code::CodeKey { rung: Doc, file: command.go, decl: 130, sub: 0, line: 1898 } |  |  | 0.520 |
| walker |  | 7137 | 35 | Code::CodeKey { rung: Doc, file: command.go, decl: 125, sub: 0, line: 1839 } |  |  | 0.520 |
| walker |  | 7180 | 43 | Code::CodeKey { rung: Doc, file: command.go, decl: 132, sub: 0, line: 1928 } |  |  | 0.520 |
| walker |  | 7229 | 49 | Code::CodeKey { rung: Doc, file: command.go, decl: 131, sub: 0, line: 1907 } |  |  | 0.520 |
| ns | 7363 |  | 336 | completions.go: the completion type vocabulary | 7.1 |  | 0.509 |
| ns | 7610 |  | 247 | The ShellCompDirective bit-mask values | 7.2 |  | 0.502 |
| ns | 7747 |  | 137 | completions.go: exported completion API roster | 7.3 |  | 0.497 |
| walker |  | 7768 | 539 | Code::CodeKey { rung: Decl, file: command.go, decl: 134, sub: 0, line: 1942 } |  |  | 0.509 |
| walker |  | 7895 | 127 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 12, line: 0 } |  |  | 0.515 |
| walker |  | 7901 | 6 | Code::CodeKey { rung: Decl, file: command.go, decl: 138, sub: 0, line: 2064 } |  |  | 0.515 |
| walker |  | 7931 | 30 | Code::CodeKey { rung: Decl, file: command.go, decl: 136, sub: 0, line: 2042 } |  |  | 0.515 |
| walker |  | 7954 | 23 | Code::CodeKey { rung: Doc, file: command.go, decl: 135, sub: 0, line: 1974 } |  |  | 0.515 |
| walker |  | 7977 | 23 | Code::CodeKey { rung: Doc, file: command.go, decl: 137, sub: 0, line: 2047 } |  |  | 0.515 |
| walker |  | 8000 | 23 | Code::CodeKey { rung: Doc, file: command.go, decl: 139, sub: 0, line: 2068 } |  |  | 0.515 |
| ns | 8040 |  | 293 | The __complete protocol and its environment configuration | 7.4 |  | 0.507 |
| walker |  | 8104 | 104 | Code::CodeKey { rung: Doc, file: command.go, decl: 106, sub: 0, line: 1628 } |  |  | 0.507 |
| ns | 8192 |  | 152 | The generated `completion` command tree | 7.5 | 7.3 | 0.503 |
| walker |  | 8219 | 115 | Code::CodeKey { rung: Doc, file: command.go, decl: 5, sub: 0, line: 269 } |  |  | 0.503 |
| walker |  | 8247 | 28 | Code::CodeKey { rung: Names, file: command_notwin.go, decl: 0, sub: 0, line: 0 } |  |  | 0.503 |
| ns | 8419 |  | 227 | Shell script generators: every exported Gen*Completion entry point | 8.1 |  | 0.497 |
| ns | 8515 |  | 96 | Legacy bash completion annotations | 8.2 |  | 0.494 |
| walker |  | 8668 | 421 | Code::CodeKey { rung: Names, file: cobra.go, decl: 0, sub: 0, line: 0 } |  |  | 0.507 |
| walker |  | 8677 | 9 | Code::CodeKey { rung: Decl, file: cobra.go, decl: 4, sub: 0, line: 45 } |  |  | 0.508 |
| walker |  | 8703 | 26 | Code::CodeKey { rung: Decl, file: cobra.go, decl: 9, sub: 0, line: 72 } |  |  | 0.508 |
| ns | 8706 |  | 191 | active_help.go: the whole feature in one batch | 8.3 |  | 0.503 |
| walker |  | 8715 | 12 | Code::CodeKey { rung: Body, file: cobra.go, decl: 11, sub: 0, line: 85 } |  |  | 0.503 |
| walker |  | 8728 | 13 | Code::CodeKey { rung: Body, file: cobra.go, decl: 13, sub: 0, line: 99 } |  |  | 0.503 |
| walker |  | 8744 | 16 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 19, sub: 0, line: 174 } |  |  | 0.503 |
| walker |  | 8844 | 100 | Code::CodeKey { rung: Decl, file: cobra.go, decl: 1, sub: 0, line: 32 } |  |  | 0.503 |
| walker |  | 8863 | 19 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 7, sub: 0, line: 62 } |  |  | 0.505 |
| walker |  | 8882 | 19 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 21, sub: 0, line: 192 } |  |  | 0.505 |
| walker |  | 8907 | 25 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 11, sub: 0, line: 85 } |  |  | 0.505 |
| ns | 8920 |  | 214 | The Windows mousetrap hook and its no-op counterpart | 8.4 | 5.3 | 0.500 |
| walker |  | 8932 | 25 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 24, sub: 0, line: 243 } |  |  | 0.500 |
| walker |  | 8960 | 28 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 12, sub: 0, line: 91 } |  |  | 0.500 |
| walker |  | 8988 | 28 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 16, sub: 0, line: 144 } |  |  | 0.500 |
| walker |  | 9017 | 29 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 13, sub: 0, line: 99 } |  |  | 0.500 |
| walker |  | 9046 | 29 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 14, sub: 0, line: 105 } |  |  | 0.500 |
| walker |  | 9076 | 30 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 18, sub: 0, line: 166 } |  |  | 0.500 |
| ns | 9098 |  | 178 | doc/: every exported generator across all four output formats | 9.1 |  | 0.494 |
| walker |  | 9110 | 34 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 23, sub: 0, line: 235 } |  |  | 0.494 |
| walker |  | 9145 | 35 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 6, sub: 0, line: 59 } |  |  | 0.495 |
| walker |  | 9188 | 43 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 8, sub: 0, line: 66 } |  |  | 0.497 |
| walker |  | 9234 | 46 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 5, sub: 0, line: 55 } |  |  | 0.504 |
| walker |  | 9296 | 62 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 9, sub: 0, line: 72 } |  |  | 0.508 |
| ns | 9326 |  | 228 | doc/: GenManTreeOptions and GenManHeader, field by field | 9.2 |  | 0.500 |
| walker |  | 9366 | 70 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 15, sub: 0, line: 114 } |  |  | 0.500 |
| walker |  | 9414 | 48 | Code::CodeKey { rung: Names, file: command_win.go, decl: 0, sub: 0, line: 0 } |  |  | 0.504 |
| walker |  | 9541 | 127 | Code::CodeKey { rung: Names, file: active_help.go, decl: 0, sub: 0, line: 0 } |  |  | 0.507 |
| ns | 9563 |  | 237 | doc/: the YAML document schema and the shared helpers | 9.3 |  | 0.500 |
| walker |  | 9590 | 49 | Code::CodeKey { rung: Decl, file: active_help.go, decl: 1, sub: 0, line: 22 } |  |  | 0.501 |
| walker |  | 9606 | 16 | Code::CodeKey { rung: Body, file: active_help.go, decl: 4, sub: 0, line: 58 } |  |  | 0.501 |
| walker |  | 9629 | 23 | Code::CodeKey { rung: Body, file: active_help.go, decl: 2, sub: 0, line: 38 } |  |  | 0.501 |
| ns | 9677 |  | 114 | docgen documentation pages: heading map | 9.4 |  | 0.498 |
| walker |  | 9694 | 65 | Code::CodeKey { rung: Body, file: active_help.go, decl: 3, sub: 0, line: 47 } |  |  | 0.498 |
| walker |  | 9765 | 71 | Code::CodeKey { rung: Doc, file: active_help.go, decl: 4, sub: 0, line: 58 } |  |  | 0.498 |
| walker |  | 9851 | 86 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 10, sub: 0, line: 81 } |  |  | 0.498 |
| ns | 9929 |  | 252 | Project policy documents: section maps | 10.1 |  | 0.490 |
| walker |  | 9989 | 138 | Code::CodeKey { rung: Names, file: args.go, decl: 0, sub: 0, line: 0 } |  |  | 0.492 |
| ns | 9993 |  | 64 | Repository metadata: maintainers and local ignore rules | 10.2 |  | 0.490 |

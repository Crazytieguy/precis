Score(3000)=0.665 I=0.791 C=0.559 ns_rows≤3K=19/50 grid(1000/1442/2080/3000/4327/6240/9000)=0.602/0.777/0.715/0.665/0.648/0.593/0.567

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 98 |  | 98 | Identity: the README lede and the package doc comment | 1.1 |  | 0.000 |
| walker |  | 191 | 191 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 196 | 5 | Fs::DirListing { dir: assets } |  |  | 0.000 |
| ns | 227 |  | 129 | go.mod: module path, Go version floor, and the four dependencies | 1.2 |  | 0.000 |
| walker |  | 292 | 96 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.161 |
| walker |  | 322 | 30 | GoMod::Identity { file: go.mod } |  |  | 0.220 |
| ns | 366 |  | 139 | Complete .go roster of the root package: sources and colocated tests | 1.3 |  | 0.526 |
| walker |  | 381 | 59 | Fs::DirListing { dir: doc } |  |  | 0.536 |
| walker |  | 429 | 48 | Code::CodeKey { rung: ModuleDoc, file: command.go, decl: 0, sub: 0, line: 0 } |  |  | 0.755 |
| walker |  | 456 | 27 | Fs::DirListing { dir: site/content } |  |  | 0.756 |
| walker |  | 469 | 13 | Fs::DirListing { dir: .github } |  |  | 0.761 |
| walker |  | 478 | 9 | Fs::DirListing { dir: .github/workflows } |  |  | 0.764 |
| walker |  | 499 | 21 | Fs::DirListing { dir: site/content/docgen } |  |  | 0.768 |
| ns | 507 |  | 141 | README 'Overview': what the library does, first half | 1.4 |  | 0.670 |
| walker |  | 522 | 23 | Fs::DirListing { dir: site/content/completions } |  |  | 0.675 |
| walker |  | 624 | 102 | Markdown::Prelude { file: README.md } |  |  | 0.677 |
| ns | 724 |  | 217 | README: the remaining capabilities, and who builds on Cobra | 1.5 | 1.4 | 0.618 |
| ns | 926 |  | 202 | Canonical usage: the rootCmd literal and Execute() from the user guide | 1.6 |  | 0.531 |
| walker |  | 974 | 350 | Plaintext::Whole { file: Makefile } |  |  | 0.546 |
| ns | 978 |  | 52 | Complete non-Go root listing: build, config, docs, governance | 1.7 |  | 0.602 |
| walker |  | 1037 | 63 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.602 |
| walker |  | 1136 | 99 | GoMod::File { file: go.mod } |  |  | 0.739 |
| ns | 1218 |  | 240 | Makefile: every target and its recipe | 1.8 |  | 0.748 |
| ns | 1304 |  | 86 | doc/ subpackage, CI workflows, and assets listings | 1.9 |  | 0.764 |
| walker |  | 1333 | 197 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 0, line: 0 } |  |  | 0.765 |
| walker |  | 1340 | 7 | Code::CodeKey { rung: Decl, file: command.go, decl: 1, sub: 0, line: 33 } |  |  | 0.765 |
| walker |  | 1361 | 21 | Code::CodeKey { rung: Decl, file: command.go, decl: 3, sub: 0, line: 45 } |  |  | 0.766 |
| walker |  | 1372 | 11 | Code::CodeKey { rung: Doc, file: command.go, decl: 3, sub: 0, line: 45 } |  |  | 0.767 |
| ns | 1374 |  | 70 | Documentation site tree: site/content and its two subdirectories | 1.10 |  | 0.777 |
| walker |  | 1388 | 16 | Code::CodeKey { rung: Doc, file: command.go, decl: 2, sub: 0, line: 42 } |  |  | 0.777 |
| walker |  | 1396 | 8 | Code::CodeKey { rung: Body, file: command.go, decl: 5, sub: 0, line: 269 } |  |  | 0.777 |
| walker |  | 1608 | 212 | Code::CodeKey { rung: Decl, file: command.go, decl: 4, sub: 0, line: 54 } |  |  | 0.777 |
| ns | 1612 |  | 238 | command.go: exported constants, FParseErrWhitelist, Group, Command's doc | 2.1 |  | 0.745 |
| ns | 1684 |  | 72 | Command fields: naming and help text (Use .. Example) | 2.2 | 2.1 | 0.728 |
| ns | 1776 |  | 92 | Command fields: argument validation, completion, metadata | 2.3 | 2.1 | 0.710 |
| walker |  | 1784 | 176 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 1, line: 0 } |  |  | 0.711 |
| walker |  | 1796 | 12 | Code::CodeKey { rung: Doc, file: command.go, decl: 16, sub: 0, line: 338 } |  |  | 0.711 |
| walker |  | 1814 | 18 | Code::CodeKey { rung: Doc, file: command.go, decl: 13, sub: 0, line: 318 } |  |  | 0.711 |
| walker |  | 1832 | 18 | Code::CodeKey { rung: Doc, file: command.go, decl: 15, sub: 0, line: 333 } |  |  | 0.711 |
| walker |  | 1851 | 19 | Code::CodeKey { rung: Doc, file: command.go, decl: 12, sub: 0, line: 313 } |  |  | 0.711 |
| walker |  | 1870 | 19 | Code::CodeKey { rung: Doc, file: command.go, decl: 17, sub: 0, line: 343 } |  |  | 0.711 |
| walker |  | 1889 | 19 | Code::CodeKey { rung: Doc, file: command.go, decl: 18, sub: 0, line: 352 } |  |  | 0.711 |
| walker |  | 2059 | 170 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 2, line: 0 } |  |  | 0.715 |
| walker |  | 2073 | 14 | Code::CodeKey { rung: Doc, file: command.go, decl: 23, sub: 0, line: 393 } |  |  | 0.715 |
| walker |  | 2087 | 14 | Code::CodeKey { rung: Doc, file: command.go, decl: 24, sub: 0, line: 398 } |  |  | 0.715 |
| walker |  | 2101 | 14 | Code::CodeKey { rung: Doc, file: command.go, decl: 25, sub: 0, line: 403 } |  |  | 0.715 |
| walker |  | 2115 | 14 | Code::CodeKey { rung: Doc, file: command.go, decl: 26, sub: 0, line: 408 } |  |  | 0.715 |
| ns | 2125 |  | 349 | Command fields: the ten *Run hooks with the documented ordering comment | 2.4 | 2.1 | 0.671 |
| ns | 2264 |  | 139 | Command fields: behaviour toggles, with ellipses over the unexported state | 2.5 | 2.1 | 0.646 |
| walker |  | 2290 | 175 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 3, line: 0 } |  |  | 0.647 |
| walker |  | 2301 | 11 | Code::CodeKey { rung: Doc, file: command.go, decl: 31, sub: 0, line: 526 } |  |  | 0.647 |
| walker |  | 2314 | 13 | Code::CodeKey { rung: Doc, file: command.go, decl: 33, sub: 0, line: 563 } |  |  | 0.647 |
| walker |  | 2327 | 13 | Code::CodeKey { rung: Doc, file: command.go, decl: 35, sub: 0, line: 583 } |  |  | 0.647 |
| walker |  | 2342 | 15 | Code::CodeKey { rung: Doc, file: command.go, decl: 34, sub: 0, line: 573 } |  |  | 0.647 |
| ns | 2372 |  | 108 | Command method roster: execution and lifecycle | 3.1 |  | 0.633 |
| ns | 2514 |  | 142 | Command method roster: building and traversing the command tree | 3.2 |  | 0.613 |
| walker |  | 2521 | 179 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 4, line: 0 } |  |  | 0.619 |
| walker |  | 2531 | 10 | Code::CodeKey { rung: Doc, file: command.go, decl: 44, sub: 0, line: 892 } |  |  | 0.619 |
| walker |  | 2545 | 14 | Code::CodeKey { rung: Doc, file: command.go, decl: 42, sub: 0, line: 863 } |  |  | 0.619 |
| walker |  | 2560 | 15 | Code::CodeKey { rung: Doc, file: command.go, decl: 39, sub: 0, line: 643 } |  |  | 0.619 |
| walker |  | 2580 | 20 | Code::CodeKey { rung: Doc, file: command.go, decl: 43, sub: 0, line: 884 } |  |  | 0.619 |
| ns | 2740 |  | 226 | Command method roster: the complete flag API | 3.3 |  | 0.592 |
| walker |  | 2766 | 186 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 5, line: 0 } |  |  | 0.619 |
| walker |  | 2778 | 12 | Code::CodeKey { rung: Doc, file: command.go, decl: 49, sub: 0, line: 1084 } |  |  | 0.619 |
| walker |  | 2799 | 21 | Code::CodeKey { rung: Doc, file: command.go, decl: 51, sub: 0, line: 1180 } |  |  | 0.619 |
| ns | 2955 |  | 215 | Command method roster: I/O wiring and the Set* customisation hooks | 3.4 |  | 0.637 |
| walker |  | 2997 | 198 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 6, line: 0 } |  |  | 0.665 |
| walker |  | 3012 | 15 | Code::CodeKey { rung: Doc, file: command.go, decl: 56, sub: 0, line: 1332 } |  |  | 0.665 |
| walker |  | 3027 | 15 | Code::CodeKey { rung: Doc, file: command.go, decl: 58, sub: 0, line: 1371 } |  |  | 0.665 |
| walker |  | 3045 | 18 | Code::CodeKey { rung: Doc, file: command.go, decl: 57, sub: 0, line: 1342 } |  |  | 0.665 |
| walker |  | 3063 | 18 | Code::CodeKey { rung: Doc, file: command.go, decl: 62, sub: 0, line: 1401 } |  |  | 0.665 |
| walker |  | 3082 | 19 | Code::CodeKey { rung: Doc, file: command.go, decl: 55, sub: 0, line: 1317 } |  |  | 0.665 |
| walker |  | 3101 | 19 | Code::CodeKey { rung: Doc, file: command.go, decl: 61, sub: 0, line: 1396 } |  |  | 0.665 |
| walker |  | 3121 | 20 | Code::CodeKey { rung: Doc, file: command.go, decl: 60, sub: 0, line: 1386 } |  |  | 0.665 |
| ns | 3142 |  | 187 | Command method roster: help, usage, templates, default-command injection | 3.5 |  | 0.676 |
| walker |  | 3143 | 22 | Code::CodeKey { rung: Doc, file: command.go, decl: 59, sub: 0, line: 1376 } |  |  | 0.676 |
| ns | 3332 |  | 190 | Command method roster: naming and introspection predicates | 3.6 |  | 0.656 |
| walker |  | 3339 | 196 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 7, line: 0 } |  |  | 0.659 |
| walker |  | 3355 | 16 | Code::CodeKey { rung: Doc, file: command.go, decl: 69, sub: 0, line: 1465 } |  |  | 0.659 |
| walker |  | 3376 | 21 | Code::CodeKey { rung: Doc, file: command.go, decl: 71, sub: 0, line: 1482 } |  |  | 0.659 |
| walker |  | 3397 | 21 | Code::CodeKey { rung: Doc, file: command.go, decl: 73, sub: 0, line: 1541 } |  |  | 0.659 |
| ns | 3516 |  | 184 | user_guide.md: complete H2 section map | 4.1 |  | 0.640 |
| walker |  | 3588 | 191 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 8, line: 0 } |  |  | 0.666 |
| walker |  | 3603 | 15 | Code::CodeKey { rung: Doc, file: command.go, decl: 77, sub: 0, line: 1591 } |  |  | 0.666 |
| walker |  | 3618 | 15 | Code::CodeKey { rung: Doc, file: command.go, decl: 78, sub: 0, line: 1596 } |  |  | 0.666 |
| walker |  | 3635 | 17 | Code::CodeKey { rung: Doc, file: command.go, decl: 79, sub: 0, line: 1601 } |  |  | 0.666 |
| walker |  | 3655 | 20 | Code::CodeKey { rung: Doc, file: command.go, decl: 74, sub: 0, line: 1551 } |  |  | 0.666 |
| walker |  | 3675 | 20 | Code::CodeKey { rung: Doc, file: command.go, decl: 76, sub: 0, line: 1586 } |  |  | 0.666 |
| ns | 3754 |  | 238 | user_guide.md: complete H3/H4 subsection map | 4.2 | 4.1 | 0.642 |
| walker |  | 3854 | 179 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 9, line: 0 } |  |  | 0.651 |
| walker |  | 3871 | 17 | Code::CodeKey { rung: Doc, file: command.go, decl: 84, sub: 0, line: 1677 } |  |  | 0.651 |
| walker |  | 3891 | 20 | Code::CodeKey { rung: Doc, file: command.go, decl: 91, sub: 0, line: 1775 } |  |  | 0.651 |
| walker |  | 3912 | 21 | Code::CodeKey { rung: Doc, file: command.go, decl: 85, sub: 0, line: 1682 } |  |  | 0.651 |
| ns | 3950 |  | 196 | Completion and Active Help documentation: H1/H2 maps | 4.3 |  | 0.634 |
| ns | 4133 |  | 183 | Test harness: the helpers every root-package test is written against | 4.4 |  | 0.617 |
| walker |  | 4146 | 234 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 10, line: 0 } |  |  | 0.647 |
| walker |  | 4159 | 13 | Code::CodeKey { rung: Doc, file: command.go, decl: 103, sub: 0, line: 1892 } |  |  | 0.647 |
| walker |  | 4173 | 14 | Code::CodeKey { rung: Doc, file: command.go, decl: 92, sub: 0, line: 1787 } |  |  | 0.647 |
| walker |  | 4189 | 16 | Code::CodeKey { rung: Doc, file: command.go, decl: 102, sub: 0, line: 1868 } |  |  | 0.647 |
| walker |  | 4206 | 17 | Code::CodeKey { rung: Doc, file: command.go, decl: 94, sub: 0, line: 1806 } |  |  | 0.647 |
| walker |  | 4223 | 17 | Code::CodeKey { rung: Doc, file: command.go, decl: 101, sub: 0, line: 1844 } |  |  | 0.647 |
| walker |  | 4242 | 19 | Code::CodeKey { rung: Doc, file: command.go, decl: 95, sub: 0, line: 1811 } |  |  | 0.647 |
| walker |  | 4263 | 21 | Code::CodeKey { rung: Doc, file: command.go, decl: 96, sub: 0, line: 1816 } |  |  | 0.647 |
| walker |  | 4452 | 189 | Code::CodeKey { rung: Decl, file: command.go, decl: 4, sub: 1, line: 54 } |  |  | 0.657 |
| ns | 4458 |  | 325 | CI: the four jobs and the platform / Go-version matrix | 4.5 |  | 0.631 |
| walker |  | 4674 | 222 | Code::CodeKey { rung: Decl, file: command.go, decl: 4, sub: 2, line: 54 } |  |  | 0.638 |
| ns | 4706 |  | 248 | golangci-lint configuration: the enabled linter set | 4.6 |  | 0.614 |
| walker |  | 4923 | 249 | Code::CodeKey { rung: Decl, file: command.go, decl: 4, sub: 3, line: 54 } |  |  | 0.624 |
| ns | 5045 |  | 339 | Execute / ExecuteC: doc comments and the root-redirect rule | 5.1 | 3.1 | 0.604 |
| walker |  | 5096 | 173 | Code::CodeKey { rung: Decl, file: command.go, decl: 4, sub: 4, line: 54 } |  |  | 0.614 |
| walker |  | 5271 | 175 | Code::CodeKey { rung: Decl, file: command.go, decl: 4, sub: 5, line: 54 } |  |  | 0.625 |
| walker |  | 5448 | 177 | Code::CodeKey { rung: Decl, file: command.go, decl: 4, sub: 6, line: 54 } |  |  | 0.638 |
| ns | 5583 |  | 538 | execute(): the ordered run pipeline | 5.2 | 5.1 | 0.601 |
| walker |  | 5663 | 215 | Code::CodeKey { rung: Decl, file: command.go, decl: 4, sub: 7, line: 54 } |  |  | 0.608 |
| ns | 5809 |  | 226 | cobra.go: the package-level behaviour switches | 5.3 |  | 0.597 |
| walker |  | 5865 | 202 | Code::CodeKey { rung: Decl, file: command.go, decl: 4, sub: 8, line: 54 } |  |  | 0.608 |
| walker |  | 5889 | 24 | Code::CodeKey { rung: Doc, file: command.go, decl: 19, sub: 0, line: 358 } |  |  | 0.608 |
| ns | 5891 |  | 82 | cobra.go: package-level function roster | 5.4 |  | 0.603 |
| walker |  | 5913 | 24 | Code::CodeKey { rung: Doc, file: command.go, decl: 20, sub: 0, line: 367 } |  |  | 0.603 |
| walker |  | 5937 | 24 | Code::CodeKey { rung: Doc, file: command.go, decl: 93, sub: 0, line: 1801 } |  |  | 0.603 |
| walker |  | 5961 | 24 | Code::CodeKey { rung: Doc, file: command.go, decl: 98, sub: 0, line: 1827 } |  |  | 0.603 |
| walker |  | 5986 | 25 | Code::CodeKey { rung: Doc, file: command.go, decl: 21, sub: 0, line: 376 } |  |  | 0.603 |
| walker |  | 6012 | 26 | Code::CodeKey { rung: Doc, file: command.go, decl: 63, sub: 0, line: 1435 } |  |  | 0.603 |
| walker |  | 6039 | 27 | Code::CodeKey { rung: Doc, file: command.go, decl: 14, sub: 0, line: 328 } |  |  | 0.603 |
| walker |  | 6066 | 27 | Code::CodeKey { rung: Doc, file: command.go, decl: 41, sub: 0, line: 821 } |  |  | 0.603 |
| walker |  | 6094 | 28 | Code::CodeKey { rung: Doc, file: command.go, decl: 64, sub: 0, line: 1440 } |  |  | 0.603 |
| walker |  | 6122 | 28 | Code::CodeKey { rung: Doc, file: command.go, decl: 65, sub: 0, line: 1445 } |  |  | 0.603 |
| walker |  | 6150 | 28 | Code::CodeKey { rung: Doc, file: command.go, decl: 66, sub: 0, line: 1450 } |  |  | 0.603 |
| walker |  | 6180 | 30 | Code::CodeKey { rung: Doc, file: command.go, decl: 36, sub: 0, line: 592 } |  |  | 0.603 |
| ns | 6188 |  | 297 | The default usage template, plus the help and version template constants | 5.5 |  | 0.593 |
| walker |  | 6210 | 30 | Code::CodeKey { rung: Doc, file: command.go, decl: 37, sub: 0, line: 605 } |  |  | 0.593 |
| walker |  | 6240 | 30 | Code::CodeKey { rung: Doc, file: command.go, decl: 38, sub: 0, line: 618 } |  |  | 0.593 |
| walker |  | 6270 | 30 | Code::CodeKey { rung: Doc, file: command.go, decl: 67, sub: 0, line: 1455 } |  |  | 0.593 |
| walker |  | 6300 | 30 | Code::CodeKey { rung: Doc, file: command.go, decl: 68, sub: 0, line: 1460 } |  |  | 0.593 |
| walker |  | 6330 | 30 | Code::CodeKey { rung: Doc, file: command.go, decl: 72, sub: 0, line: 1501 } |  |  | 0.593 |
| walker |  | 6361 | 31 | Code::CodeKey { rung: Doc, file: command.go, decl: 9, sub: 0, line: 296 } |  |  | 0.593 |
| walker |  | 6392 | 31 | Code::CodeKey { rung: Doc, file: command.go, decl: 10, sub: 0, line: 302 } |  |  | 0.593 |
| walker |  | 6423 | 31 | Code::CodeKey { rung: Doc, file: command.go, decl: 11, sub: 0, line: 308 } |  |  | 0.593 |
| walker |  | 6456 | 33 | Code::CodeKey { rung: Doc, file: command.go, decl: 99, sub: 0, line: 1833 } |  |  | 0.593 |
| walker |  | 6490 | 34 | Code::CodeKey { rung: Doc, file: command.go, decl: 6, sub: 0, line: 275 } |  |  | 0.593 |
| walker |  | 6525 | 35 | Code::CodeKey { rung: Doc, file: command.go, decl: 40, sub: 0, line: 757 } |  |  | 0.593 |
| walker |  | 6560 | 35 | Code::CodeKey { rung: Doc, file: command.go, decl: 86, sub: 0, line: 1688 } |  |  | 0.593 |
| walker |  | 6595 | 35 | Code::CodeKey { rung: Doc, file: command.go, decl: 100, sub: 0, line: 1839 } |  |  | 0.593 |
| walker |  | 6631 | 36 | Code::CodeKey { rung: Doc, file: command.go, decl: 45, sub: 0, line: 901 } |  |  | 0.593 |
| ns | 6635 |  | 447 | args.go: every PositionalArgs validator with its defining doc line | 6.1 |  | 0.579 |
| walker |  | 6668 | 37 | Code::CodeKey { rung: Doc, file: command.go, decl: 27, sub: 0, line: 444 } |  |  | 0.579 |
| walker |  | 6705 | 37 | Code::CodeKey { rung: Doc, file: command.go, decl: 70, sub: 0, line: 1474 } |  |  | 0.579 |
| walker |  | 6742 | 37 | Code::CodeKey { rung: Doc, file: command.go, decl: 80, sub: 0, line: 1607 } |  |  | 0.579 |
| walker |  | 6781 | 39 | Code::CodeKey { rung: Doc, file: command.go, decl: 22, sub: 0, line: 382 } |  |  | 0.579 |
| walker |  | 6820 | 39 | Code::CodeKey { rung: Doc, file: command.go, decl: 29, sub: 0, line: 484 } |  |  | 0.579 |
| walker |  | 6859 | 39 | Code::CodeKey { rung: Doc, file: command.go, decl: 97, sub: 0, line: 1822 } |  |  | 0.579 |
| ns | 6898 |  | 263 | flag_groups.go: the three group markers and their annotation keys | 6.2 |  | 0.571 |
| walker |  | 6899 | 40 | Code::CodeKey { rung: Doc, file: command.go, decl: 75, sub: 0, line: 1562 } |  |  | 0.571 |
| walker |  | 6940 | 41 | Code::CodeKey { rung: Doc, file: command.go, decl: 83, sub: 0, line: 1662 } |  |  | 0.571 |
| walker |  | 6982 | 42 | Code::CodeKey { rung: Doc, file: command.go, decl: 7, sub: 0, line: 281 } |  |  | 0.571 |
| walker |  | 7024 | 42 | Code::CodeKey { rung: Doc, file: command.go, decl: 28, sub: 0, line: 478 } |  |  | 0.571 |
| ns | 7027 |  | 129 | shell_completions.go: the complete Mark* helper family | 6.3 |  | 0.565 |
| walker |  | 7067 | 43 | Code::CodeKey { rung: Doc, file: command.go, decl: 30, sub: 0, line: 520 } |  |  | 0.565 |
| ns | 7363 |  | 336 | completions.go: the completion type vocabulary | 7.1 |  | 0.553 |
| walker |  | 7540 | 473 | Code::CodeKey { rung: Names, file: completions.go, decl: 0, sub: 0, line: 0 } |  |  | 0.555 |
| walker |  | 7574 | 34 | Code::CodeKey { rung: Decl, file: completions.go, decl: 13, sub: 0, line: 311 } |  |  | 0.555 |
| ns | 7610 |  | 247 | The ShellCompDirective bit-mask values | 7.2 |  | 0.550 |
| walker |  | 7667 | 93 | Code::CodeKey { rung: Decl, file: completions.go, decl: 1, sub: 0, line: 28 } |  |  | 0.550 |
| ns | 7747 |  | 137 | completions.go: exported completion API roster | 7.3 |  | 0.553 |
| walker |  | 7855 | 188 | Code::CodeKey { rung: Decl, file: completions.go, decl: 4, sub: 0, line: 107 } |  |  | 0.567 |
| walker |  | 7868 | 13 | Code::CodeKey { rung: Doc, file: completions.go, decl: 7, sub: 0, line: 139 } |  |  | 0.569 |
| ns | 8040 |  | 293 | The __complete protocol and its environment configuration | 7.4 |  | 0.564 |
| walker |  | 8071 | 203 | Code::CodeKey { rung: Decl, file: completions.go, decl: 3, sub: 0, line: 56 } |  |  | 0.570 |
| walker |  | 8086 | 15 | Code::CodeKey { rung: Doc, file: completions.go, decl: 4, sub: 0, line: 107 } |  |  | 0.572 |
| walker |  | 8101 | 15 | Code::CodeKey { rung: Doc, file: completions.go, decl: 17, sub: 0, line: 985 } |  |  | 0.572 |
| ns | 8192 |  | 152 | The generated `completion` command tree | 7.5 | 7.3 | 0.568 |
| walker |  | 8284 | 183 | Code::CodeKey { rung: Decl, file: completions.go, decl: 3, sub: 1, line: 56 } |  |  | 0.574 |
| ns | 8419 |  | 227 | Shell script generators: every exported Gen*Completion entry point | 8.1 |  | 0.566 |
| ns | 8515 |  | 96 | Legacy bash completion annotations | 8.2 |  | 0.564 |
| walker |  | 8518 | 234 | Code::CodeKey { rung: Names, file: cobra.go, decl: 0, sub: 0, line: 0 } |  |  | 0.573 |
| walker |  | 8544 | 26 | Code::CodeKey { rung: Decl, file: cobra.go, decl: 5, sub: 0, line: 72 } |  |  | 0.573 |
| walker |  | 8563 | 19 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 3, sub: 0, line: 62 } |  |  | 0.574 |
| walker |  | 8609 | 46 | Code::CodeKey { rung: Doc, file: command.go, decl: 89, sub: 0, line: 1744 } |  |  | 0.574 |
| walker |  | 8656 | 47 | Code::CodeKey { rung: Doc, file: command.go, decl: 88, sub: 0, line: 1716 } |  |  | 0.574 |
| walker |  | 8703 | 47 | Code::CodeKey { rung: Doc, file: command.go, decl: 90, sub: 0, line: 1770 } |  |  | 0.574 |
| ns | 8706 |  | 191 | active_help.go: the whole feature in one batch | 8.3 |  | 0.569 |
| walker |  | 8751 | 48 | Code::CodeKey { rung: Doc, file: command.go, decl: 32, sub: 0, line: 547 } |  |  | 0.569 |
| walker |  | 8799 | 48 | Code::CodeKey { rung: Doc, file: command.go, decl: 47, sub: 0, line: 1070 } |  |  | 0.571 |
| walker |  | 8850 | 51 | Code::CodeKey { rung: Doc, file: command.go, decl: 87, sub: 0, line: 1702 } |  |  | 0.571 |
| walker |  | 8872 | 22 | Code::CodeKey { rung: Doc, file: completions.go, decl: 8, sub: 0, line: 142 } |  |  | 0.571 |
| walker |  | 8894 | 22 | Code::CodeKey { rung: Doc, file: completions.go, decl: 18, sub: 0, line: 991 } |  |  | 0.571 |
| ns | 8920 |  | 214 | The Windows mousetrap hook and its no-op counterpart | 8.4 | 5.3 | 0.564 |
| walker |  | 8946 | 52 | Code::CodeKey { rung: Doc, file: command.go, decl: 46, sub: 0, line: 1062 } |  |  | 0.567 |
| walker |  | 8999 | 53 | Code::CodeKey { rung: Doc, file: command.go, decl: 8, sub: 0, line: 289 } |  |  | 0.567 |
| walker |  | 9052 | 53 | Code::CodeKey { rung: Doc, file: command.go, decl: 82, sub: 0, line: 1648 } |  |  | 0.567 |
| walker |  | 9094 | 42 | Code::CodeKey { rung: Names, file: active_help.go, decl: 0, sub: 0, line: 0 } |  |  | 0.568 |
| ns | 9098 |  | 178 | doc/: every exported generator across all four output formats | 9.1 |  | 0.561 |
| walker |  | 9231 | 137 | Code::CodeKey { rung: Names, file: bash_completions.go, decl: 0, sub: 0, line: 0 } |  |  | 0.566 |
| walker |  | 9236 | 5 | Code::CodeKey { rung: Decl, file: bash_completions.go, decl: 1, sub: 0, line: 29 } |  |  | 0.567 |
| walker |  | 9247 | 11 | Code::CodeKey { rung: Doc, file: bash_completions.go, decl: 1, sub: 0, line: 29 } |  |  | 0.567 |
| walker |  | 9262 | 15 | Code::CodeKey { rung: Doc, file: bash_completions.go, decl: 3, sub: 0, line: 701 } |  |  | 0.567 |
| walker |  | 9282 | 20 | Code::CodeKey { rung: Doc, file: bash_completions.go, decl: 2, sub: 0, line: 683 } |  |  | 0.567 |
| ns | 9326 |  | 228 | doc/: GenManTreeOptions and GenManHeader, field by field | 9.2 |  | 0.559 |
| walker |  | 9490 | 208 | Code::CodeKey { rung: Names, file: args.go, decl: 0, sub: 0, line: 0 } |  |  | 0.563 |
| walker |  | 9497 | 7 | Code::CodeKey { rung: Body, file: args.go, decl: 5, sub: 0, line: 82 } |  |  | 0.563 |
| walker |  | 9510 | 13 | Code::CodeKey { rung: Doc, file: args.go, decl: 5, sub: 0, line: 82 } |  |  | 0.563 |
| walker |  | 9526 | 16 | Code::CodeKey { rung: Doc, file: args.go, decl: 2, sub: 0, line: 42 } |  |  | 0.564 |
| walker |  | 9544 | 18 | Code::CodeKey { rung: Doc, file: args.go, decl: 8, sub: 0, line: 107 } |  |  | 0.565 |
| walker |  | 9562 | 18 | Code::CodeKey { rung: Doc, file: args.go, decl: 10, sub: 0, line: 127 } |  |  | 0.567 |
| ns | 9563 |  | 237 | doc/: the YAML document schema and the shared helpers | 9.3 |  | 0.558 |
| walker |  | 9581 | 19 | Code::CodeKey { rung: Doc, file: args.go, decl: 4, sub: 0, line: 69 } |  |  | 0.560 |
| walker |  | 9600 | 19 | Code::CodeKey { rung: Doc, file: args.go, decl: 7, sub: 0, line: 97 } |  |  | 0.561 |
| walker |  | 9620 | 20 | Code::CodeKey { rung: Doc, file: args.go, decl: 6, sub: 0, line: 87 } |  |  | 0.562 |
| walker |  | 9642 | 22 | Code::CodeKey { rung: Doc, file: args.go, decl: 9, sub: 0, line: 117 } |  |  | 0.564 |
| ns | 9677 |  | 114 | docgen documentation pages: heading map | 9.4 |  | 0.561 |
| walker |  | 9696 | 54 | Code::CodeKey { rung: Doc, file: command.go, decl: 48, sub: 0, line: 1078 } |  |  | 0.564 |
| walker |  | 9752 | 56 | Code::CodeKey { rung: Doc, file: command.go, decl: 52, sub: 0, line: 1219 } |  |  | 0.564 |
| walker |  | 9776 | 24 | Code::CodeKey { rung: Doc, file: completions.go, decl: 12, sub: 0, line: 186 } |  |  | 0.564 |
| walker |  | 9788 | 12 | Code::CodeKey { rung: Body, file: cobra.go, decl: 7, sub: 0, line: 85 } |  |  | 0.564 |
| walker |  | 9813 | 25 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 7, sub: 0, line: 85 } |  |  | 0.564 |
| walker |  | 9838 | 25 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 14, sub: 0, line: 243 } |  |  | 0.564 |
| walker |  | 9861 | 23 | Code::CodeKey { rung: Body, file: active_help.go, decl: 1, sub: 0, line: 38 } |  |  | 0.564 |
| ns | 9929 |  | 252 | Project policy documents: section maps | 10.1 |  | 0.555 |
| walker |  | 9973 | 112 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.555 |
| ns | 9993 |  | 64 | Repository metadata: maintainers and local ignore rules | 10.2 |  | 0.552 |

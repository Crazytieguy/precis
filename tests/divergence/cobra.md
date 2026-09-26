Score(3000)=0.670 I=0.796 C=0.564 ns_rows≤3K=19/50 grid(1000/1442/2080/3000/4327/6240/9000)=0.597/0.799/0.733/0.670/0.658/0.601/0.574

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
| walker |  | 687 | 63 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.677 |
| walker |  | 699 | 12 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.677 |
| ns | 724 |  | 217 | README: the remaining capabilities, and who builds on Cobra | 1.5 | 1.4 | 0.618 |
| ns | 926 |  | 202 | Canonical usage: the rootCmd literal and Execute() from the user guide | 1.6 |  | 0.531 |
| ns | 978 |  | 52 | Complete non-Go root listing: build, config, docs, governance | 1.7 |  | 0.586 |
| walker |  | 1084 | 385 | Plaintext::Whole { file: Makefile } |  |  | 0.605 |
| walker |  | 1183 | 99 | GoMod::File { file: go.mod } |  |  | 0.743 |
| ns | 1218 |  | 240 | Makefile: every target and its recipe | 1.8 |  | 0.776 |
| ns | 1304 |  | 86 | doc/ subpackage, CI workflows, and assets listings | 1.9 |  | 0.788 |
| ns | 1374 |  | 70 | Documentation site tree: site/content and its two subdirectories | 1.10 |  | 0.795 |
| walker |  | 1380 | 197 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 0, line: 0 } |  |  | 0.797 |
| walker |  | 1387 | 7 | Code::CodeKey { rung: Decl, file: command.go, decl: 1, sub: 0, line: 33 } |  |  | 0.797 |
| walker |  | 1408 | 21 | Code::CodeKey { rung: Decl, file: command.go, decl: 3, sub: 0, line: 45 } |  |  | 0.798 |
| walker |  | 1419 | 11 | Code::CodeKey { rung: Doc, file: command.go, decl: 3, sub: 0, line: 45 } |  |  | 0.798 |
| walker |  | 1435 | 16 | Code::CodeKey { rung: Doc, file: command.go, decl: 2, sub: 0, line: 42 } |  |  | 0.799 |
| walker |  | 1443 | 8 | Code::CodeKey { rung: Body, file: command.go, decl: 5, sub: 0, line: 269 } |  |  | 0.799 |
| ns | 1612 |  | 238 | command.go: exported constants, FParseErrWhitelist, Group, Command's doc | 2.1 |  | 0.765 |
| walker |  | 1655 | 212 | Code::CodeKey { rung: Decl, file: command.go, decl: 4, sub: 0, line: 54 } |  |  | 0.765 |
| ns | 1684 |  | 72 | Command fields: naming and help text (Use .. Example) | 2.2 | 2.1 | 0.748 |
| ns | 1776 |  | 92 | Command fields: argument validation, completion, metadata | 2.3 | 2.1 | 0.728 |
| walker |  | 1831 | 176 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 1, line: 0 } |  |  | 0.730 |
| walker |  | 1843 | 12 | Code::CodeKey { rung: Doc, file: command.go, decl: 16, sub: 0, line: 338 } |  |  | 0.730 |
| walker |  | 1861 | 18 | Code::CodeKey { rung: Doc, file: command.go, decl: 13, sub: 0, line: 318 } |  |  | 0.730 |
| walker |  | 1879 | 18 | Code::CodeKey { rung: Doc, file: command.go, decl: 15, sub: 0, line: 333 } |  |  | 0.730 |
| walker |  | 1898 | 19 | Code::CodeKey { rung: Doc, file: command.go, decl: 12, sub: 0, line: 313 } |  |  | 0.730 |
| walker |  | 1917 | 19 | Code::CodeKey { rung: Doc, file: command.go, decl: 17, sub: 0, line: 343 } |  |  | 0.730 |
| walker |  | 1936 | 19 | Code::CodeKey { rung: Doc, file: command.go, decl: 18, sub: 0, line: 352 } |  |  | 0.730 |
| walker |  | 2106 | 170 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 2, line: 0 } |  |  | 0.734 |
| walker |  | 2120 | 14 | Code::CodeKey { rung: Doc, file: command.go, decl: 23, sub: 0, line: 393 } |  |  | 0.734 |
| ns | 2125 |  | 349 | Command fields: the ten *Run hooks with the documented ordering comment | 2.4 | 2.1 | 0.689 |
| walker |  | 2134 | 14 | Code::CodeKey { rung: Doc, file: command.go, decl: 24, sub: 0, line: 398 } |  |  | 0.689 |
| walker |  | 2148 | 14 | Code::CodeKey { rung: Doc, file: command.go, decl: 25, sub: 0, line: 403 } |  |  | 0.689 |
| walker |  | 2162 | 14 | Code::CodeKey { rung: Doc, file: command.go, decl: 26, sub: 0, line: 408 } |  |  | 0.689 |
| ns | 2264 |  | 139 | Command fields: behaviour toggles, with ellipses over the unexported state | 2.5 | 2.1 | 0.663 |
| walker |  | 2337 | 175 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 3, line: 0 } |  |  | 0.664 |
| walker |  | 2348 | 11 | Code::CodeKey { rung: Doc, file: command.go, decl: 31, sub: 0, line: 526 } |  |  | 0.664 |
| walker |  | 2361 | 13 | Code::CodeKey { rung: Doc, file: command.go, decl: 33, sub: 0, line: 563 } |  |  | 0.664 |
| ns | 2372 |  | 108 | Command method roster: execution and lifecycle | 3.1 |  | 0.649 |
| walker |  | 2374 | 13 | Code::CodeKey { rung: Doc, file: command.go, decl: 35, sub: 0, line: 583 } |  |  | 0.649 |
| walker |  | 2389 | 15 | Code::CodeKey { rung: Doc, file: command.go, decl: 34, sub: 0, line: 573 } |  |  | 0.649 |
| ns | 2514 |  | 142 | Command method roster: building and traversing the command tree | 3.2 |  | 0.629 |
| walker |  | 2568 | 179 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 4, line: 0 } |  |  | 0.635 |
| walker |  | 2578 | 10 | Code::CodeKey { rung: Doc, file: command.go, decl: 44, sub: 0, line: 892 } |  |  | 0.635 |
| walker |  | 2592 | 14 | Code::CodeKey { rung: Doc, file: command.go, decl: 42, sub: 0, line: 863 } |  |  | 0.635 |
| walker |  | 2607 | 15 | Code::CodeKey { rung: Doc, file: command.go, decl: 39, sub: 0, line: 643 } |  |  | 0.635 |
| walker |  | 2627 | 20 | Code::CodeKey { rung: Doc, file: command.go, decl: 43, sub: 0, line: 884 } |  |  | 0.635 |
| ns | 2740 |  | 226 | Command method roster: the complete flag API | 3.3 |  | 0.608 |
| walker |  | 2813 | 186 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 5, line: 0 } |  |  | 0.634 |
| walker |  | 2825 | 12 | Code::CodeKey { rung: Doc, file: command.go, decl: 49, sub: 0, line: 1084 } |  |  | 0.634 |
| walker |  | 2846 | 21 | Code::CodeKey { rung: Doc, file: command.go, decl: 51, sub: 0, line: 1180 } |  |  | 0.634 |
| ns | 2955 |  | 215 | Command method roster: I/O wiring and the Set* customisation hooks | 3.4 |  | 0.651 |
| walker |  | 3044 | 198 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 6, line: 0 } |  |  | 0.679 |
| walker |  | 3059 | 15 | Code::CodeKey { rung: Doc, file: command.go, decl: 56, sub: 0, line: 1332 } |  |  | 0.679 |
| walker |  | 3074 | 15 | Code::CodeKey { rung: Doc, file: command.go, decl: 58, sub: 0, line: 1371 } |  |  | 0.679 |
| walker |  | 3092 | 18 | Code::CodeKey { rung: Doc, file: command.go, decl: 57, sub: 0, line: 1342 } |  |  | 0.679 |
| walker |  | 3110 | 18 | Code::CodeKey { rung: Doc, file: command.go, decl: 62, sub: 0, line: 1401 } |  |  | 0.679 |
| walker |  | 3129 | 19 | Code::CodeKey { rung: Doc, file: command.go, decl: 55, sub: 0, line: 1317 } |  |  | 0.679 |
| ns | 3142 |  | 187 | Command method roster: help, usage, templates, default-command injection | 3.5 |  | 0.689 |
| walker |  | 3148 | 19 | Code::CodeKey { rung: Doc, file: command.go, decl: 61, sub: 0, line: 1396 } |  |  | 0.689 |
| walker |  | 3168 | 20 | Code::CodeKey { rung: Doc, file: command.go, decl: 60, sub: 0, line: 1386 } |  |  | 0.689 |
| walker |  | 3190 | 22 | Code::CodeKey { rung: Doc, file: command.go, decl: 59, sub: 0, line: 1376 } |  |  | 0.689 |
| ns | 3332 |  | 190 | Command method roster: naming and introspection predicates | 3.6 |  | 0.668 |
| walker |  | 3386 | 196 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 7, line: 0 } |  |  | 0.671 |
| walker |  | 3402 | 16 | Code::CodeKey { rung: Doc, file: command.go, decl: 69, sub: 0, line: 1465 } |  |  | 0.671 |
| walker |  | 3423 | 21 | Code::CodeKey { rung: Doc, file: command.go, decl: 71, sub: 0, line: 1482 } |  |  | 0.671 |
| walker |  | 3444 | 21 | Code::CodeKey { rung: Doc, file: command.go, decl: 73, sub: 0, line: 1541 } |  |  | 0.671 |
| ns | 3516 |  | 184 | user_guide.md: complete H2 section map | 4.1 |  | 0.652 |
| walker |  | 3635 | 191 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 8, line: 0 } |  |  | 0.678 |
| walker |  | 3650 | 15 | Code::CodeKey { rung: Doc, file: command.go, decl: 77, sub: 0, line: 1591 } |  |  | 0.678 |
| walker |  | 3665 | 15 | Code::CodeKey { rung: Doc, file: command.go, decl: 78, sub: 0, line: 1596 } |  |  | 0.678 |
| walker |  | 3682 | 17 | Code::CodeKey { rung: Doc, file: command.go, decl: 79, sub: 0, line: 1601 } |  |  | 0.678 |
| walker |  | 3702 | 20 | Code::CodeKey { rung: Doc, file: command.go, decl: 74, sub: 0, line: 1551 } |  |  | 0.678 |
| walker |  | 3722 | 20 | Code::CodeKey { rung: Doc, file: command.go, decl: 76, sub: 0, line: 1586 } |  |  | 0.678 |
| ns | 3754 |  | 238 | user_guide.md: complete H3/H4 subsection map | 4.2 | 4.1 | 0.654 |
| walker |  | 3901 | 179 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 9, line: 0 } |  |  | 0.662 |
| walker |  | 3918 | 17 | Code::CodeKey { rung: Doc, file: command.go, decl: 84, sub: 0, line: 1677 } |  |  | 0.662 |
| walker |  | 3938 | 20 | Code::CodeKey { rung: Doc, file: command.go, decl: 91, sub: 0, line: 1775 } |  |  | 0.662 |
| ns | 3950 |  | 196 | Completion and Active Help documentation: H1/H2 maps | 4.3 |  | 0.645 |
| walker |  | 3959 | 21 | Code::CodeKey { rung: Doc, file: command.go, decl: 85, sub: 0, line: 1682 } |  |  | 0.645 |
| ns | 4133 |  | 183 | Test harness: the helpers every root-package test is written against | 4.4 |  | 0.628 |
| walker |  | 4193 | 234 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 10, line: 0 } |  |  | 0.658 |
| walker |  | 4206 | 13 | Code::CodeKey { rung: Doc, file: command.go, decl: 103, sub: 0, line: 1892 } |  |  | 0.658 |
| walker |  | 4220 | 14 | Code::CodeKey { rung: Doc, file: command.go, decl: 92, sub: 0, line: 1787 } |  |  | 0.658 |
| walker |  | 4236 | 16 | Code::CodeKey { rung: Doc, file: command.go, decl: 102, sub: 0, line: 1868 } |  |  | 0.658 |
| walker |  | 4253 | 17 | Code::CodeKey { rung: Doc, file: command.go, decl: 94, sub: 0, line: 1806 } |  |  | 0.658 |
| walker |  | 4270 | 17 | Code::CodeKey { rung: Doc, file: command.go, decl: 101, sub: 0, line: 1844 } |  |  | 0.658 |
| walker |  | 4289 | 19 | Code::CodeKey { rung: Doc, file: command.go, decl: 95, sub: 0, line: 1811 } |  |  | 0.658 |
| walker |  | 4310 | 21 | Code::CodeKey { rung: Doc, file: command.go, decl: 96, sub: 0, line: 1816 } |  |  | 0.658 |
| ns | 4458 |  | 325 | CI: the four jobs and the platform / Go-version matrix | 4.5 |  | 0.632 |
| walker |  | 4499 | 189 | Code::CodeKey { rung: Decl, file: command.go, decl: 4, sub: 1, line: 54 } |  |  | 0.641 |
| ns | 4706 |  | 248 | golangci-lint configuration: the enabled linter set | 4.6 |  | 0.617 |
| walker |  | 4721 | 222 | Code::CodeKey { rung: Decl, file: command.go, decl: 4, sub: 2, line: 54 } |  |  | 0.624 |
| walker |  | 4970 | 249 | Code::CodeKey { rung: Decl, file: command.go, decl: 4, sub: 3, line: 54 } |  |  | 0.634 |
| ns | 5045 |  | 339 | Execute / ExecuteC: doc comments and the root-redirect rule | 5.1 | 3.1 | 0.613 |
| walker |  | 5143 | 173 | Code::CodeKey { rung: Decl, file: command.go, decl: 4, sub: 4, line: 54 } |  |  | 0.624 |
| walker |  | 5318 | 175 | Code::CodeKey { rung: Decl, file: command.go, decl: 4, sub: 5, line: 54 } |  |  | 0.635 |
| walker |  | 5495 | 177 | Code::CodeKey { rung: Decl, file: command.go, decl: 4, sub: 6, line: 54 } |  |  | 0.648 |
| ns | 5583 |  | 538 | execute(): the ordered run pipeline | 5.2 | 5.1 | 0.610 |
| walker |  | 5710 | 215 | Code::CodeKey { rung: Decl, file: command.go, decl: 4, sub: 7, line: 54 } |  |  | 0.617 |
| ns | 5809 |  | 226 | cobra.go: the package-level behaviour switches | 5.3 |  | 0.606 |
| ns | 5891 |  | 82 | cobra.go: package-level function roster | 5.4 |  | 0.601 |
| walker |  | 5912 | 202 | Code::CodeKey { rung: Decl, file: command.go, decl: 4, sub: 8, line: 54 } |  |  | 0.611 |
| walker |  | 5936 | 24 | Code::CodeKey { rung: Doc, file: command.go, decl: 19, sub: 0, line: 358 } |  |  | 0.611 |
| walker |  | 5960 | 24 | Code::CodeKey { rung: Doc, file: command.go, decl: 20, sub: 0, line: 367 } |  |  | 0.611 |
| walker |  | 5984 | 24 | Code::CodeKey { rung: Doc, file: command.go, decl: 93, sub: 0, line: 1801 } |  |  | 0.611 |
| walker |  | 6008 | 24 | Code::CodeKey { rung: Doc, file: command.go, decl: 98, sub: 0, line: 1827 } |  |  | 0.611 |
| walker |  | 6033 | 25 | Code::CodeKey { rung: Doc, file: command.go, decl: 21, sub: 0, line: 376 } |  |  | 0.611 |
| walker |  | 6059 | 26 | Code::CodeKey { rung: Doc, file: command.go, decl: 63, sub: 0, line: 1435 } |  |  | 0.611 |
| walker |  | 6086 | 27 | Code::CodeKey { rung: Doc, file: command.go, decl: 14, sub: 0, line: 328 } |  |  | 0.611 |
| walker |  | 6113 | 27 | Code::CodeKey { rung: Doc, file: command.go, decl: 41, sub: 0, line: 821 } |  |  | 0.611 |
| walker |  | 6141 | 28 | Code::CodeKey { rung: Doc, file: command.go, decl: 64, sub: 0, line: 1440 } |  |  | 0.611 |
| walker |  | 6169 | 28 | Code::CodeKey { rung: Doc, file: command.go, decl: 65, sub: 0, line: 1445 } |  |  | 0.611 |
| ns | 6188 |  | 297 | The default usage template, plus the help and version template constants | 5.5 |  | 0.601 |
| walker |  | 6197 | 28 | Code::CodeKey { rung: Doc, file: command.go, decl: 66, sub: 0, line: 1450 } |  |  | 0.601 |
| walker |  | 6227 | 30 | Code::CodeKey { rung: Doc, file: command.go, decl: 36, sub: 0, line: 592 } |  |  | 0.601 |
| walker |  | 6257 | 30 | Code::CodeKey { rung: Doc, file: command.go, decl: 37, sub: 0, line: 605 } |  |  | 0.601 |
| walker |  | 6287 | 30 | Code::CodeKey { rung: Doc, file: command.go, decl: 38, sub: 0, line: 618 } |  |  | 0.601 |
| walker |  | 6317 | 30 | Code::CodeKey { rung: Doc, file: command.go, decl: 67, sub: 0, line: 1455 } |  |  | 0.601 |
| walker |  | 6347 | 30 | Code::CodeKey { rung: Doc, file: command.go, decl: 68, sub: 0, line: 1460 } |  |  | 0.601 |
| walker |  | 6377 | 30 | Code::CodeKey { rung: Doc, file: command.go, decl: 72, sub: 0, line: 1501 } |  |  | 0.601 |
| walker |  | 6408 | 31 | Code::CodeKey { rung: Doc, file: command.go, decl: 9, sub: 0, line: 296 } |  |  | 0.601 |
| walker |  | 6439 | 31 | Code::CodeKey { rung: Doc, file: command.go, decl: 10, sub: 0, line: 302 } |  |  | 0.601 |
| walker |  | 6470 | 31 | Code::CodeKey { rung: Doc, file: command.go, decl: 11, sub: 0, line: 308 } |  |  | 0.601 |
| walker |  | 6503 | 33 | Code::CodeKey { rung: Doc, file: command.go, decl: 99, sub: 0, line: 1833 } |  |  | 0.601 |
| walker |  | 6537 | 34 | Code::CodeKey { rung: Doc, file: command.go, decl: 6, sub: 0, line: 275 } |  |  | 0.601 |
| walker |  | 6572 | 35 | Code::CodeKey { rung: Doc, file: command.go, decl: 40, sub: 0, line: 757 } |  |  | 0.601 |
| walker |  | 6607 | 35 | Code::CodeKey { rung: Doc, file: command.go, decl: 86, sub: 0, line: 1688 } |  |  | 0.601 |
| ns | 6635 |  | 447 | args.go: every PositionalArgs validator with its defining doc line | 6.1 |  | 0.587 |
| walker |  | 6642 | 35 | Code::CodeKey { rung: Doc, file: command.go, decl: 100, sub: 0, line: 1839 } |  |  | 0.587 |
| walker |  | 6678 | 36 | Code::CodeKey { rung: Doc, file: command.go, decl: 45, sub: 0, line: 901 } |  |  | 0.587 |
| walker |  | 6715 | 37 | Code::CodeKey { rung: Doc, file: command.go, decl: 27, sub: 0, line: 444 } |  |  | 0.587 |
| walker |  | 6752 | 37 | Code::CodeKey { rung: Doc, file: command.go, decl: 70, sub: 0, line: 1474 } |  |  | 0.587 |
| walker |  | 6789 | 37 | Code::CodeKey { rung: Doc, file: command.go, decl: 80, sub: 0, line: 1607 } |  |  | 0.587 |
| walker |  | 6828 | 39 | Code::CodeKey { rung: Doc, file: command.go, decl: 22, sub: 0, line: 382 } |  |  | 0.587 |
| walker |  | 6867 | 39 | Code::CodeKey { rung: Doc, file: command.go, decl: 29, sub: 0, line: 484 } |  |  | 0.587 |
| ns | 6898 |  | 263 | flag_groups.go: the three group markers and their annotation keys | 6.2 |  | 0.579 |
| walker |  | 6906 | 39 | Code::CodeKey { rung: Doc, file: command.go, decl: 97, sub: 0, line: 1822 } |  |  | 0.579 |
| walker |  | 6946 | 40 | Code::CodeKey { rung: Doc, file: command.go, decl: 75, sub: 0, line: 1562 } |  |  | 0.579 |
| walker |  | 6987 | 41 | Code::CodeKey { rung: Doc, file: command.go, decl: 83, sub: 0, line: 1662 } |  |  | 0.579 |
| ns | 7027 |  | 129 | shell_completions.go: the complete Mark* helper family | 6.3 |  | 0.573 |
| walker |  | 7029 | 42 | Code::CodeKey { rung: Doc, file: command.go, decl: 7, sub: 0, line: 281 } |  |  | 0.573 |
| walker |  | 7071 | 42 | Code::CodeKey { rung: Doc, file: command.go, decl: 28, sub: 0, line: 478 } |  |  | 0.573 |
| walker |  | 7114 | 43 | Code::CodeKey { rung: Doc, file: command.go, decl: 30, sub: 0, line: 520 } |  |  | 0.573 |
| ns | 7363 |  | 336 | completions.go: the completion type vocabulary | 7.1 |  | 0.560 |
| walker |  | 7587 | 473 | Code::CodeKey { rung: Names, file: completions.go, decl: 0, sub: 0, line: 0 } |  |  | 0.562 |
| ns | 7610 |  | 247 | The ShellCompDirective bit-mask values | 7.2 |  | 0.557 |
| walker |  | 7621 | 34 | Code::CodeKey { rung: Decl, file: completions.go, decl: 13, sub: 0, line: 311 } |  |  | 0.557 |
| walker |  | 7714 | 93 | Code::CodeKey { rung: Decl, file: completions.go, decl: 1, sub: 0, line: 28 } |  |  | 0.558 |
| ns | 7747 |  | 137 | completions.go: exported completion API roster | 7.3 |  | 0.560 |
| walker |  | 7902 | 188 | Code::CodeKey { rung: Decl, file: completions.go, decl: 4, sub: 0, line: 107 } |  |  | 0.575 |
| walker |  | 7915 | 13 | Code::CodeKey { rung: Doc, file: completions.go, decl: 7, sub: 0, line: 139 } |  |  | 0.577 |
| ns | 8040 |  | 293 | The __complete protocol and its environment configuration | 7.4 |  | 0.571 |
| walker |  | 8118 | 203 | Code::CodeKey { rung: Decl, file: completions.go, decl: 3, sub: 0, line: 56 } |  |  | 0.577 |
| walker |  | 8133 | 15 | Code::CodeKey { rung: Doc, file: completions.go, decl: 4, sub: 0, line: 107 } |  |  | 0.579 |
| walker |  | 8148 | 15 | Code::CodeKey { rung: Doc, file: completions.go, decl: 17, sub: 0, line: 985 } |  |  | 0.579 |
| ns | 8192 |  | 152 | The generated `completion` command tree | 7.5 | 7.3 | 0.575 |
| walker |  | 8331 | 183 | Code::CodeKey { rung: Decl, file: completions.go, decl: 3, sub: 1, line: 56 } |  |  | 0.581 |
| ns | 8419 |  | 227 | Shell script generators: every exported Gen*Completion entry point | 8.1 |  | 0.573 |
| ns | 8515 |  | 96 | Legacy bash completion annotations | 8.2 |  | 0.571 |
| walker |  | 8565 | 234 | Code::CodeKey { rung: Names, file: cobra.go, decl: 0, sub: 0, line: 0 } |  |  | 0.580 |
| walker |  | 8591 | 26 | Code::CodeKey { rung: Decl, file: cobra.go, decl: 5, sub: 0, line: 72 } |  |  | 0.580 |
| walker |  | 8610 | 19 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 3, sub: 0, line: 62 } |  |  | 0.581 |
| walker |  | 8656 | 46 | Code::CodeKey { rung: Doc, file: command.go, decl: 89, sub: 0, line: 1744 } |  |  | 0.581 |
| walker |  | 8703 | 47 | Code::CodeKey { rung: Doc, file: command.go, decl: 88, sub: 0, line: 1716 } |  |  | 0.581 |
| ns | 8706 |  | 191 | active_help.go: the whole feature in one batch | 8.3 |  | 0.576 |
| walker |  | 8750 | 47 | Code::CodeKey { rung: Doc, file: command.go, decl: 90, sub: 0, line: 1770 } |  |  | 0.576 |
| walker |  | 8798 | 48 | Code::CodeKey { rung: Doc, file: command.go, decl: 32, sub: 0, line: 547 } |  |  | 0.576 |
| walker |  | 8846 | 48 | Code::CodeKey { rung: Doc, file: command.go, decl: 47, sub: 0, line: 1070 } |  |  | 0.578 |
| walker |  | 8897 | 51 | Code::CodeKey { rung: Doc, file: command.go, decl: 87, sub: 0, line: 1702 } |  |  | 0.578 |
| walker |  | 8919 | 22 | Code::CodeKey { rung: Doc, file: completions.go, decl: 8, sub: 0, line: 142 } |  |  | 0.578 |
| ns | 8920 |  | 214 | The Windows mousetrap hook and its no-op counterpart | 8.4 | 5.3 | 0.571 |
| walker |  | 8941 | 22 | Code::CodeKey { rung: Doc, file: completions.go, decl: 18, sub: 0, line: 991 } |  |  | 0.571 |
| walker |  | 8993 | 52 | Code::CodeKey { rung: Doc, file: command.go, decl: 46, sub: 0, line: 1062 } |  |  | 0.574 |
| walker |  | 9046 | 53 | Code::CodeKey { rung: Doc, file: command.go, decl: 8, sub: 0, line: 289 } |  |  | 0.574 |
| ns | 9098 |  | 178 | doc/: every exported generator across all four output formats | 9.1 |  | 0.568 |
| walker |  | 9099 | 53 | Code::CodeKey { rung: Doc, file: command.go, decl: 82, sub: 0, line: 1648 } |  |  | 0.568 |
| walker |  | 9141 | 42 | Code::CodeKey { rung: Names, file: active_help.go, decl: 0, sub: 0, line: 0 } |  |  | 0.568 |
| walker |  | 9278 | 137 | Code::CodeKey { rung: Names, file: bash_completions.go, decl: 0, sub: 0, line: 0 } |  |  | 0.573 |
| walker |  | 9283 | 5 | Code::CodeKey { rung: Decl, file: bash_completions.go, decl: 1, sub: 0, line: 29 } |  |  | 0.574 |
| walker |  | 9294 | 11 | Code::CodeKey { rung: Doc, file: bash_completions.go, decl: 1, sub: 0, line: 29 } |  |  | 0.574 |
| walker |  | 9309 | 15 | Code::CodeKey { rung: Doc, file: bash_completions.go, decl: 3, sub: 0, line: 701 } |  |  | 0.574 |
| ns | 9326 |  | 228 | doc/: GenManTreeOptions and GenManHeader, field by field | 9.2 |  | 0.565 |
| walker |  | 9329 | 20 | Code::CodeKey { rung: Doc, file: bash_completions.go, decl: 2, sub: 0, line: 683 } |  |  | 0.565 |
| walker |  | 9537 | 208 | Code::CodeKey { rung: Names, file: args.go, decl: 0, sub: 0, line: 0 } |  |  | 0.569 |
| walker |  | 9544 | 7 | Code::CodeKey { rung: Body, file: args.go, decl: 5, sub: 0, line: 82 } |  |  | 0.569 |
| walker |  | 9557 | 13 | Code::CodeKey { rung: Doc, file: args.go, decl: 5, sub: 0, line: 82 } |  |  | 0.570 |
| ns | 9563 |  | 237 | doc/: the YAML document schema and the shared helpers | 9.3 |  | 0.562 |
| walker |  | 9573 | 16 | Code::CodeKey { rung: Doc, file: args.go, decl: 2, sub: 0, line: 42 } |  |  | 0.563 |
| walker |  | 9591 | 18 | Code::CodeKey { rung: Doc, file: args.go, decl: 8, sub: 0, line: 107 } |  |  | 0.564 |
| walker |  | 9609 | 18 | Code::CodeKey { rung: Doc, file: args.go, decl: 10, sub: 0, line: 127 } |  |  | 0.565 |
| walker |  | 9628 | 19 | Code::CodeKey { rung: Doc, file: args.go, decl: 4, sub: 0, line: 69 } |  |  | 0.566 |
| walker |  | 9647 | 19 | Code::CodeKey { rung: Doc, file: args.go, decl: 7, sub: 0, line: 97 } |  |  | 0.568 |
| walker |  | 9667 | 20 | Code::CodeKey { rung: Doc, file: args.go, decl: 6, sub: 0, line: 87 } |  |  | 0.569 |
| ns | 9677 |  | 114 | docgen documentation pages: heading map | 9.4 |  | 0.566 |
| walker |  | 9689 | 22 | Code::CodeKey { rung: Doc, file: args.go, decl: 9, sub: 0, line: 117 } |  |  | 0.568 |
| walker |  | 9743 | 54 | Code::CodeKey { rung: Doc, file: command.go, decl: 48, sub: 0, line: 1078 } |  |  | 0.571 |
| walker |  | 9799 | 56 | Code::CodeKey { rung: Doc, file: command.go, decl: 52, sub: 0, line: 1219 } |  |  | 0.571 |
| walker |  | 9823 | 24 | Code::CodeKey { rung: Doc, file: completions.go, decl: 12, sub: 0, line: 186 } |  |  | 0.571 |
| walker |  | 9835 | 12 | Code::CodeKey { rung: Body, file: cobra.go, decl: 7, sub: 0, line: 85 } |  |  | 0.571 |
| walker |  | 9860 | 25 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 7, sub: 0, line: 85 } |  |  | 0.571 |
| walker |  | 9885 | 25 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 14, sub: 0, line: 243 } |  |  | 0.571 |
| walker |  | 9908 | 23 | Code::CodeKey { rung: Body, file: active_help.go, decl: 1, sub: 0, line: 38 } |  |  | 0.571 |
| ns | 9929 |  | 252 | Project policy documents: section maps | 10.1 |  | 0.561 |
| ns | 9993 |  | 64 | Repository metadata: maintainers and local ignore rules | 10.2 |  | 0.559 |
| walker |  | 9995 | 87 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.559 |

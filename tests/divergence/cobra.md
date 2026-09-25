Score(3000)=0.644 I=0.786 C=0.528 ns_rows≤3K=19/50 grid(1000/1442/2080/3000/4327/6240/9000)=0.597/0.799/0.744/0.644/0.629/0.556/0.523

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
| walker |  | 1384 | 201 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 0, line: 0 } |  |  | 0.797 |
| walker |  | 1395 | 11 | Code::CodeKey { rung: Decl, file: command.go, decl: 1, sub: 0, line: 33 } |  |  | 0.798 |
| walker |  | 1416 | 21 | Code::CodeKey { rung: Decl, file: command.go, decl: 3, sub: 0, line: 45 } |  |  | 0.799 |
| walker |  | 1424 | 8 | Code::CodeKey { rung: Body, file: command.go, decl: 5, sub: 0, line: 269 } |  |  | 0.799 |
| walker |  | 1433 | 9 | Code::CodeKey { rung: Body, file: command.go, decl: 6, sub: 0, line: 275 } |  |  | 0.799 |
| walker |  | 1444 | 11 | Code::CodeKey { rung: Doc, file: command.go, decl: 3, sub: 0, line: 45 } |  |  | 0.800 |
| walker |  | 1460 | 16 | Code::CodeKey { rung: Doc, file: command.go, decl: 2, sub: 0, line: 42 } |  |  | 0.801 |
| ns | 1612 |  | 238 | command.go: exported constants, FParseErrWhitelist, Group, Command's doc | 2.1 |  | 0.776 |
| walker |  | 1634 | 174 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 1, line: 0 } |  |  | 0.777 |
| walker |  | 1646 | 12 | Code::CodeKey { rung: Doc, file: command.go, decl: 16, sub: 0, line: 338 } |  |  | 0.777 |
| walker |  | 1664 | 18 | Code::CodeKey { rung: Doc, file: command.go, decl: 13, sub: 0, line: 318 } |  |  | 0.777 |
| walker |  | 1682 | 18 | Code::CodeKey { rung: Doc, file: command.go, decl: 15, sub: 0, line: 333 } |  |  | 0.777 |
| ns | 1684 |  | 72 | Command fields: naming and help text (Use .. Example) | 2.2 | 2.1 | 0.759 |
| walker |  | 1701 | 19 | Code::CodeKey { rung: Doc, file: command.go, decl: 12, sub: 0, line: 313 } |  |  | 0.759 |
| walker |  | 1720 | 19 | Code::CodeKey { rung: Doc, file: command.go, decl: 17, sub: 0, line: 343 } |  |  | 0.759 |
| ns | 1776 |  | 92 | Command fields: argument validation, completion, metadata | 2.3 | 2.1 | 0.740 |
| walker |  | 1892 | 172 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 2, line: 0 } |  |  | 0.743 |
| walker |  | 1906 | 14 | Code::CodeKey { rung: Doc, file: command.go, decl: 23, sub: 0, line: 393 } |  |  | 0.743 |
| walker |  | 1920 | 14 | Code::CodeKey { rung: Doc, file: command.go, decl: 24, sub: 0, line: 398 } |  |  | 0.743 |
| walker |  | 1934 | 14 | Code::CodeKey { rung: Doc, file: command.go, decl: 25, sub: 0, line: 403 } |  |  | 0.743 |
| walker |  | 1953 | 19 | Code::CodeKey { rung: Doc, file: command.go, decl: 18, sub: 0, line: 352 } |  |  | 0.743 |
| walker |  | 1977 | 24 | Code::CodeKey { rung: Doc, file: command.go, decl: 19, sub: 0, line: 358 } |  |  | 0.743 |
| walker |  | 2001 | 24 | Code::CodeKey { rung: Doc, file: command.go, decl: 20, sub: 0, line: 367 } |  |  | 0.743 |
| ns | 2125 |  | 349 | Command fields: the ten *Run hooks with the documented ordering comment | 2.4 | 2.1 | 0.698 |
| walker |  | 2174 | 173 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 3, line: 0 } |  |  | 0.698 |
| walker |  | 2188 | 14 | Code::CodeKey { rung: Doc, file: command.go, decl: 26, sub: 0, line: 408 } |  |  | 0.698 |
| walker |  | 2213 | 25 | Code::CodeKey { rung: Doc, file: command.go, decl: 21, sub: 0, line: 376 } |  |  | 0.698 |
| ns | 2264 |  | 139 | Command fields: behaviour toggles, with ellipses over the unexported state | 2.5 | 2.1 | 0.672 |
| ns | 2372 |  | 108 | Command method roster: execution and lifecycle | 3.1 |  | 0.657 |
| walker |  | 2412 | 199 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 4, line: 0 } |  |  | 0.658 |
| walker |  | 2423 | 11 | Code::CodeKey { rung: Doc, file: command.go, decl: 36, sub: 0, line: 526 } |  |  | 0.658 |
| walker |  | 2434 | 11 | Code::CodeKey { rung: Doc, file: command.go, decl: 39, sub: 0, line: 563 } |  |  | 0.658 |
| walker |  | 2445 | 11 | Code::CodeKey { rung: Doc, file: command.go, decl: 43, sub: 0, line: 583 } |  |  | 0.658 |
| walker |  | 2458 | 13 | Code::CodeKey { rung: Doc, file: command.go, decl: 41, sub: 0, line: 573 } |  |  | 0.658 |
| ns | 2514 |  | 142 | Command method roster: building and traversing the command tree | 3.2 |  | 0.638 |
| walker |  | 2629 | 171 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 5, line: 0 } |  |  | 0.638 |
| walker |  | 2644 | 15 | Code::CodeKey { rung: Doc, file: command.go, decl: 48, sub: 0, line: 643 } |  |  | 0.638 |
| ns | 2740 |  | 226 | Command method roster: the complete flag API | 3.3 |  | 0.611 |
| walker |  | 2828 | 184 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 6, line: 0 } |  |  | 0.618 |
| walker |  | 2838 | 10 | Code::CodeKey { rung: Doc, file: command.go, decl: 60, sub: 0, line: 892 } |  |  | 0.618 |
| walker |  | 2852 | 14 | Code::CodeKey { rung: Doc, file: command.go, decl: 58, sub: 0, line: 863 } |  |  | 0.618 |
| walker |  | 2872 | 20 | Code::CodeKey { rung: Doc, file: command.go, decl: 59, sub: 0, line: 884 } |  |  | 0.618 |
| ns | 2955 |  | 215 | Command method roster: I/O wiring and the Set* customisation hooks | 3.4 |  | 0.636 |
| walker |  | 3057 | 185 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 7, line: 0 } |  |  | 0.657 |
| walker |  | 3069 | 12 | Code::CodeKey { rung: Doc, file: command.go, decl: 68, sub: 0, line: 1084 } |  |  | 0.657 |
| walker |  | 3090 | 21 | Code::CodeKey { rung: Doc, file: command.go, decl: 70, sub: 0, line: 1180 } |  |  | 0.657 |
| ns | 3142 |  | 187 | Command method roster: help, usage, templates, default-command injection | 3.5 |  | 0.657 |
| walker |  | 3295 | 205 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 8, line: 0 } |  |  | 0.672 |
| walker |  | 3308 | 13 | Code::CodeKey { rung: Doc, file: command.go, decl: 76, sub: 0, line: 1325 } |  |  | 0.672 |
| walker |  | 3327 | 19 | Code::CodeKey { rung: Doc, file: command.go, decl: 75, sub: 0, line: 1317 } |  |  | 0.672 |
| ns | 3332 |  | 190 | Command method roster: naming and introspection predicates | 3.6 |  | 0.652 |
| ns | 3516 |  | 184 | user_guide.md: complete H2 section map | 4.1 |  | 0.634 |
| walker |  | 3532 | 205 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 9, line: 0 } |  |  | 0.655 |
| walker |  | 3545 | 13 | Code::CodeKey { rung: Doc, file: command.go, decl: 80, sub: 0, line: 1332 } |  |  | 0.655 |
| walker |  | 3560 | 15 | Code::CodeKey { rung: Doc, file: command.go, decl: 82, sub: 0, line: 1371 } |  |  | 0.655 |
| walker |  | 3578 | 18 | Code::CodeKey { rung: Doc, file: command.go, decl: 81, sub: 0, line: 1342 } |  |  | 0.655 |
| walker |  | 3596 | 18 | Code::CodeKey { rung: Doc, file: command.go, decl: 86, sub: 0, line: 1401 } |  |  | 0.655 |
| walker |  | 3615 | 19 | Code::CodeKey { rung: Doc, file: command.go, decl: 85, sub: 0, line: 1396 } |  |  | 0.655 |
| walker |  | 3635 | 20 | Code::CodeKey { rung: Doc, file: command.go, decl: 84, sub: 0, line: 1386 } |  |  | 0.655 |
| walker |  | 3657 | 22 | Code::CodeKey { rung: Doc, file: command.go, decl: 83, sub: 0, line: 1376 } |  |  | 0.655 |
| walker |  | 3683 | 26 | Code::CodeKey { rung: Doc, file: command.go, decl: 87, sub: 0, line: 1435 } |  |  | 0.655 |
| walker |  | 3710 | 27 | Code::CodeKey { rung: Doc, file: command.go, decl: 14, sub: 0, line: 328 } |  |  | 0.655 |
| walker |  | 3737 | 27 | Code::CodeKey { rung: Doc, file: command.go, decl: 57, sub: 0, line: 821 } |  |  | 0.655 |
| ns | 3754 |  | 238 | user_guide.md: complete H3/H4 subsection map | 4.2 | 4.1 | 0.632 |
| walker |  | 3928 | 191 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 10, line: 0 } |  |  | 0.638 |
| walker |  | 3944 | 16 | Code::CodeKey { rung: Doc, file: command.go, decl: 93, sub: 0, line: 1465 } |  |  | 0.638 |
| ns | 3950 |  | 196 | Completion and Active Help documentation: H1/H2 maps | 4.3 |  | 0.621 |
| walker |  | 3964 | 20 | Code::CodeKey { rung: Doc, file: command.go, decl: 98, sub: 0, line: 1551 } |  |  | 0.621 |
| walker |  | 3985 | 21 | Code::CodeKey { rung: Doc, file: command.go, decl: 95, sub: 0, line: 1482 } |  |  | 0.621 |
| walker |  | 4006 | 21 | Code::CodeKey { rung: Doc, file: command.go, decl: 97, sub: 0, line: 1541 } |  |  | 0.621 |
| ns | 4133 |  | 183 | Test harness: the helpers every root-package test is written against | 4.4 |  | 0.604 |
| walker |  | 4200 | 194 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 11, line: 0 } |  |  | 0.629 |
| walker |  | 4215 | 15 | Code::CodeKey { rung: Doc, file: command.go, decl: 102, sub: 0, line: 1591 } |  |  | 0.629 |
| walker |  | 4230 | 15 | Code::CodeKey { rung: Doc, file: command.go, decl: 103, sub: 0, line: 1596 } |  |  | 0.629 |
| walker |  | 4247 | 17 | Code::CodeKey { rung: Doc, file: command.go, decl: 104, sub: 0, line: 1601 } |  |  | 0.629 |
| walker |  | 4264 | 17 | Code::CodeKey { rung: Doc, file: command.go, decl: 109, sub: 0, line: 1677 } |  |  | 0.629 |
| walker |  | 4284 | 20 | Code::CodeKey { rung: Doc, file: command.go, decl: 101, sub: 0, line: 1586 } |  |  | 0.629 |
| ns | 4458 |  | 325 | CI: the four jobs and the platform / Go-version matrix | 4.5 |  | 0.604 |
| walker |  | 4462 | 178 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 12, line: 0 } |  |  | 0.610 |
| walker |  | 4476 | 14 | Code::CodeKey { rung: Doc, file: command.go, decl: 117, sub: 0, line: 1787 } |  |  | 0.610 |
| walker |  | 4496 | 20 | Code::CodeKey { rung: Doc, file: command.go, decl: 116, sub: 0, line: 1775 } |  |  | 0.610 |
| walker |  | 4517 | 21 | Code::CodeKey { rung: Doc, file: command.go, decl: 110, sub: 0, line: 1682 } |  |  | 0.610 |
| ns | 4706 |  | 248 | golangci-lint configuration: the enabled linter set | 4.6 |  | 0.587 |
| walker |  | 4720 | 203 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 13, line: 0 } |  |  | 0.608 |
| walker |  | 4733 | 13 | Code::CodeKey { rung: Doc, file: command.go, decl: 127, sub: 0, line: 1855 } |  |  | 0.608 |
| walker |  | 4750 | 17 | Code::CodeKey { rung: Doc, file: command.go, decl: 119, sub: 0, line: 1806 } |  |  | 0.608 |
| walker |  | 4767 | 17 | Code::CodeKey { rung: Doc, file: command.go, decl: 126, sub: 0, line: 1844 } |  |  | 0.608 |
| walker |  | 4786 | 19 | Code::CodeKey { rung: Doc, file: command.go, decl: 120, sub: 0, line: 1811 } |  |  | 0.608 |
| walker |  | 4807 | 21 | Code::CodeKey { rung: Doc, file: command.go, decl: 121, sub: 0, line: 1816 } |  |  | 0.608 |
| walker |  | 4831 | 24 | Code::CodeKey { rung: Doc, file: command.go, decl: 118, sub: 0, line: 1801 } |  |  | 0.608 |
| walker |  | 4855 | 24 | Code::CodeKey { rung: Doc, file: command.go, decl: 123, sub: 0, line: 1827 } |  |  | 0.608 |
| ns | 5045 |  | 339 | Execute / ExecuteC: doc comments and the root-redirect rule | 5.1 | 3.1 | 0.588 |
| walker |  | 5109 | 254 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 14, line: 0 } |  |  | 0.593 |
| walker |  | 5115 | 6 | Code::CodeKey { rung: Decl, file: command.go, decl: 138, sub: 0, line: 2064 } |  |  | 0.593 |
| walker |  | 5143 | 28 | Code::CodeKey { rung: Decl, file: command.go, decl: 133, sub: 0, line: 1937 } |  |  | 0.593 |
| walker |  | 5173 | 30 | Code::CodeKey { rung: Decl, file: command.go, decl: 136, sub: 0, line: 2042 } |  |  | 0.593 |
| walker |  | 5186 | 13 | Code::CodeKey { rung: Doc, file: command.go, decl: 129, sub: 0, line: 1892 } |  |  | 0.593 |
| walker |  | 5202 | 16 | Code::CodeKey { rung: Doc, file: command.go, decl: 128, sub: 0, line: 1868 } |  |  | 0.593 |
| walker |  | 5222 | 20 | Code::CodeKey { rung: Doc, file: command.go, decl: 133, sub: 0, line: 1937 } |  |  | 0.593 |
| walker |  | 5245 | 23 | Code::CodeKey { rung: Doc, file: command.go, decl: 137, sub: 0, line: 2047 } |  |  | 0.593 |
| walker |  | 5268 | 23 | Code::CodeKey { rung: Doc, file: command.go, decl: 139, sub: 0, line: 2068 } |  |  | 0.593 |
| ns | 5583 |  | 538 | execute(): the ordered run pipeline | 5.2 | 5.1 | 0.558 |
| walker |  | 5807 | 539 | Code::CodeKey { rung: Decl, file: command.go, decl: 134, sub: 0, line: 1942 } |  |  | 0.560 |
| ns | 5809 |  | 226 | cobra.go: the package-level behaviour switches | 5.3 |  | 0.550 |
| walker |  | 5830 | 23 | Code::CodeKey { rung: Doc, file: command.go, decl: 135, sub: 0, line: 1974 } |  |  | 0.550 |
| walker |  | 5858 | 28 | Code::CodeKey { rung: Doc, file: command.go, decl: 88, sub: 0, line: 1440 } |  |  | 0.550 |
| walker |  | 5886 | 28 | Code::CodeKey { rung: Doc, file: command.go, decl: 89, sub: 0, line: 1445 } |  |  | 0.550 |
| ns | 5891 |  | 82 | cobra.go: package-level function roster | 5.4 |  | 0.546 |
| walker |  | 5914 | 28 | Code::CodeKey { rung: Doc, file: command.go, decl: 90, sub: 0, line: 1450 } |  |  | 0.546 |
| walker |  | 5944 | 30 | Code::CodeKey { rung: Doc, file: command.go, decl: 44, sub: 0, line: 592 } |  |  | 0.546 |
| walker |  | 5974 | 30 | Code::CodeKey { rung: Doc, file: command.go, decl: 45, sub: 0, line: 605 } |  |  | 0.546 |
| walker |  | 6004 | 30 | Code::CodeKey { rung: Doc, file: command.go, decl: 46, sub: 0, line: 618 } |  |  | 0.546 |
| walker |  | 6034 | 30 | Code::CodeKey { rung: Doc, file: command.go, decl: 91, sub: 0, line: 1455 } |  |  | 0.546 |
| walker |  | 6064 | 30 | Code::CodeKey { rung: Doc, file: command.go, decl: 92, sub: 0, line: 1460 } |  |  | 0.546 |
| walker |  | 6094 | 30 | Code::CodeKey { rung: Doc, file: command.go, decl: 96, sub: 0, line: 1501 } |  |  | 0.546 |
| walker |  | 6125 | 31 | Code::CodeKey { rung: Doc, file: command.go, decl: 9, sub: 0, line: 296 } |  |  | 0.546 |
| walker |  | 6156 | 31 | Code::CodeKey { rung: Doc, file: command.go, decl: 10, sub: 0, line: 302 } |  |  | 0.546 |
| walker |  | 6187 | 31 | Code::CodeKey { rung: Doc, file: command.go, decl: 11, sub: 0, line: 308 } |  |  | 0.546 |
| ns | 6188 |  | 297 | The default usage template, plus the help and version template constants | 5.5 |  | 0.556 |
| walker |  | 6218 | 31 | Code::CodeKey { rung: Doc, file: command.go, decl: 31, sub: 0, line: 464 } |  |  | 0.556 |
| walker |  | 6249 | 31 | Code::CodeKey { rung: Doc, file: command.go, decl: 34, sub: 0, line: 505 } |  |  | 0.556 |
| walker |  | 6280 | 31 | Code::CodeKey { rung: Doc, file: command.go, decl: 47, sub: 0, line: 631 } |  |  | 0.556 |
| walker |  | 6311 | 31 | Code::CodeKey { rung: Doc, file: command.go, decl: 100, sub: 0, line: 1571 } |  |  | 0.556 |
| walker |  | 6344 | 33 | Code::CodeKey { rung: Doc, file: command.go, decl: 124, sub: 0, line: 1833 } |  |  | 0.556 |
| walker |  | 6377 | 33 | Code::CodeKey { rung: Doc, file: command.go, decl: 130, sub: 0, line: 1898 } |  |  | 0.556 |
| walker |  | 6411 | 34 | Code::CodeKey { rung: Doc, file: command.go, decl: 6, sub: 0, line: 275 } |  |  | 0.556 |
| walker |  | 6446 | 35 | Code::CodeKey { rung: Doc, file: command.go, decl: 54, sub: 0, line: 757 } |  |  | 0.556 |
| walker |  | 6481 | 35 | Code::CodeKey { rung: Doc, file: command.go, decl: 111, sub: 0, line: 1688 } |  |  | 0.556 |
| walker |  | 6516 | 35 | Code::CodeKey { rung: Doc, file: command.go, decl: 125, sub: 0, line: 1839 } |  |  | 0.556 |
| walker |  | 6552 | 36 | Code::CodeKey { rung: Doc, file: command.go, decl: 61, sub: 0, line: 901 } |  |  | 0.556 |
| walker |  | 6589 | 37 | Code::CodeKey { rung: Doc, file: command.go, decl: 30, sub: 0, line: 444 } |  |  | 0.556 |
| walker |  | 6626 | 37 | Code::CodeKey { rung: Doc, file: command.go, decl: 94, sub: 0, line: 1474 } |  |  | 0.556 |
| ns | 6635 |  | 447 | args.go: every PositionalArgs validator with its defining doc line | 6.1 |  | 0.543 |
| walker |  | 6663 | 37 | Code::CodeKey { rung: Doc, file: command.go, decl: 105, sub: 0, line: 1607 } |  |  | 0.543 |
| walker |  | 6702 | 39 | Code::CodeKey { rung: Doc, file: command.go, decl: 22, sub: 0, line: 382 } |  |  | 0.543 |
| walker |  | 6741 | 39 | Code::CodeKey { rung: Doc, file: command.go, decl: 33, sub: 0, line: 484 } |  |  | 0.543 |
| walker |  | 6780 | 39 | Code::CodeKey { rung: Doc, file: command.go, decl: 122, sub: 0, line: 1822 } |  |  | 0.543 |
| walker |  | 6820 | 40 | Code::CodeKey { rung: Doc, file: command.go, decl: 99, sub: 0, line: 1562 } |  |  | 0.543 |
| walker |  | 6861 | 41 | Code::CodeKey { rung: Doc, file: command.go, decl: 108, sub: 0, line: 1662 } |  |  | 0.543 |
| walker |  | 6889 | 28 | Code::CodeKey { rung: Names, file: command_notwin.go, decl: 0, sub: 0, line: 0 } |  |  | 0.543 |
| ns | 6898 |  | 263 | flag_groups.go: the three group markers and their annotation keys | 6.2 |  | 0.536 |
| walker |  | 6931 | 42 | Code::CodeKey { rung: Doc, file: command.go, decl: 7, sub: 0, line: 281 } |  |  | 0.536 |
| walker |  | 6973 | 42 | Code::CodeKey { rung: Doc, file: command.go, decl: 32, sub: 0, line: 478 } |  |  | 0.536 |
| walker |  | 7016 | 43 | Code::CodeKey { rung: Doc, file: command.go, decl: 35, sub: 0, line: 520 } |  |  | 0.536 |
| ns | 7027 |  | 129 | shell_completions.go: the complete Mark* helper family | 6.3 |  | 0.530 |
| walker |  | 7059 | 43 | Code::CodeKey { rung: Doc, file: command.go, decl: 132, sub: 0, line: 1928 } |  |  | 0.530 |
| walker |  | 7105 | 46 | Code::CodeKey { rung: Doc, file: command.go, decl: 71, sub: 0, line: 1205 } |  |  | 0.530 |
| walker |  | 7151 | 46 | Code::CodeKey { rung: Doc, file: command.go, decl: 114, sub: 0, line: 1744 } |  |  | 0.530 |
| ns | 7363 |  | 336 | completions.go: the completion type vocabulary | 7.1 |  | 0.518 |
| walker |  | 7572 | 421 | Code::CodeKey { rung: Names, file: cobra.go, decl: 0, sub: 0, line: 0 } |  |  | 0.532 |
| walker |  | 7581 | 9 | Code::CodeKey { rung: Decl, file: cobra.go, decl: 4, sub: 0, line: 45 } |  |  | 0.533 |
| walker |  | 7607 | 26 | Code::CodeKey { rung: Decl, file: cobra.go, decl: 9, sub: 0, line: 72 } |  |  | 0.533 |
| ns | 7610 |  | 247 | The ShellCompDirective bit-mask values | 7.2 |  | 0.525 |
| walker |  | 7707 | 100 | Code::CodeKey { rung: Decl, file: cobra.go, decl: 1, sub: 0, line: 32 } |  |  | 0.525 |
| walker |  | 7719 | 12 | Code::CodeKey { rung: Body, file: cobra.go, decl: 11, sub: 0, line: 85 } |  |  | 0.525 |
| walker |  | 7735 | 16 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 19, sub: 0, line: 174 } |  |  | 0.525 |
| ns | 7747 |  | 137 | completions.go: exported completion API roster | 7.3 |  | 0.520 |
| walker |  | 7754 | 19 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 7, sub: 0, line: 62 } |  |  | 0.522 |
| walker |  | 7773 | 19 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 21, sub: 0, line: 192 } |  |  | 0.522 |
| walker |  | 7820 | 47 | Code::CodeKey { rung: Doc, file: command.go, decl: 113, sub: 0, line: 1716 } |  |  | 0.522 |
| walker |  | 7867 | 47 | Code::CodeKey { rung: Doc, file: command.go, decl: 115, sub: 0, line: 1770 } |  |  | 0.522 |
| walker |  | 7915 | 48 | Code::CodeKey { rung: Names, file: command_win.go, decl: 0, sub: 0, line: 0 } |  |  | 0.522 |
| walker |  | 7963 | 48 | Code::CodeKey { rung: Doc, file: command.go, decl: 37, sub: 0, line: 547 } |  |  | 0.522 |
| walker |  | 8011 | 48 | Code::CodeKey { rung: Doc, file: command.go, decl: 66, sub: 0, line: 1070 } |  |  | 0.524 |
| ns | 8040 |  | 293 | The __complete protocol and its environment configuration | 7.4 |  | 0.516 |
| walker |  | 8060 | 49 | Code::CodeKey { rung: Doc, file: command.go, decl: 131, sub: 0, line: 1907 } |  |  | 0.516 |
| walker |  | 8111 | 51 | Code::CodeKey { rung: Doc, file: command.go, decl: 112, sub: 0, line: 1702 } |  |  | 0.516 |
| walker |  | 8163 | 52 | Code::CodeKey { rung: Doc, file: command.go, decl: 65, sub: 0, line: 1062 } |  |  | 0.519 |
| ns | 8192 |  | 152 | The generated `completion` command tree | 7.5 | 7.3 | 0.515 |
| walker |  | 8216 | 53 | Code::CodeKey { rung: Doc, file: command.go, decl: 8, sub: 0, line: 289 } |  |  | 0.515 |
| walker |  | 8269 | 53 | Code::CodeKey { rung: Doc, file: command.go, decl: 107, sub: 0, line: 1648 } |  |  | 0.515 |
| walker |  | 8323 | 54 | Code::CodeKey { rung: Doc, file: command.go, decl: 67, sub: 0, line: 1078 } |  |  | 0.519 |
| ns | 8419 |  | 227 | Shell script generators: every exported Gen*Completion entry point | 8.1 |  | 0.512 |
| walker |  | 8450 | 127 | Code::CodeKey { rung: Names, file: active_help.go, decl: 0, sub: 0, line: 0 } |  |  | 0.512 |
| walker |  | 8499 | 49 | Code::CodeKey { rung: Decl, file: active_help.go, decl: 1, sub: 0, line: 22 } |  |  | 0.512 |
| walker |  | 8515 | 16 | Code::CodeKey { rung: Body, file: active_help.go, decl: 4, sub: 0, line: 58 } |  |  | 0.510 |
| ns | 8515 |  | 96 | Legacy bash completion annotations | 8.2 |  | 0.510 |
| walker |  | 8538 | 23 | Code::CodeKey { rung: Body, file: active_help.go, decl: 2, sub: 0, line: 38 } |  |  | 0.510 |
| walker |  | 8594 | 56 | Code::CodeKey { rung: Doc, file: command.go, decl: 72, sub: 0, line: 1219 } |  |  | 0.510 |
| ns | 8706 |  | 191 | active_help.go: the whole feature in one batch | 8.3 |  | 0.510 |
| walker |  | 8821 | 227 | Code::CodeKey { rung: Names, file: args.go, decl: 0, sub: 0, line: 0 } |  |  | 0.515 |
| walker |  | 8828 | 7 | Code::CodeKey { rung: Body, file: args.go, decl: 6, sub: 0, line: 82 } |  |  | 0.515 |
| walker |  | 8841 | 13 | Code::CodeKey { rung: Doc, file: args.go, decl: 6, sub: 0, line: 82 } |  |  | 0.516 |
| walker |  | 8857 | 16 | Code::CodeKey { rung: Doc, file: args.go, decl: 3, sub: 0, line: 42 } |  |  | 0.517 |
| walker |  | 8873 | 16 | Code::CodeKey { rung: Body, file: args.go, decl: 12, sub: 0, line: 142 } |  |  | 0.517 |
| walker |  | 8891 | 18 | Code::CodeKey { rung: Doc, file: args.go, decl: 9, sub: 0, line: 107 } |  |  | 0.519 |
| walker |  | 8909 | 18 | Code::CodeKey { rung: Doc, file: args.go, decl: 11, sub: 0, line: 127 } |  |  | 0.520 |
| ns | 8920 |  | 214 | The Windows mousetrap hook and its no-op counterpart | 8.4 | 5.3 | 0.516 |
| walker |  | 8928 | 19 | Code::CodeKey { rung: Doc, file: args.go, decl: 5, sub: 0, line: 69 } |  |  | 0.518 |
| walker |  | 8947 | 19 | Code::CodeKey { rung: Doc, file: args.go, decl: 8, sub: 0, line: 97 } |  |  | 0.519 |
| walker |  | 8967 | 20 | Code::CodeKey { rung: Doc, file: args.go, decl: 7, sub: 0, line: 87 } |  |  | 0.521 |
| walker |  | 8989 | 22 | Code::CodeKey { rung: Doc, file: args.go, decl: 10, sub: 0, line: 117 } |  |  | 0.523 |
| walker |  | 9014 | 25 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 11, sub: 0, line: 85 } |  |  | 0.523 |
| walker |  | 9039 | 25 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 24, sub: 0, line: 243 } |  |  | 0.523 |
| ns | 9098 |  | 178 | doc/: every exported generator across all four output formats | 9.1 |  | 0.517 |
| walker |  | 9212 | 173 | Code::CodeKey { rung: Names, file: bash_completions.go, decl: 0, sub: 0, line: 0 } |  |  | 0.521 |
| walker |  | 9221 | 9 | Code::CodeKey { rung: Decl, file: bash_completions.go, decl: 1, sub: 0, line: 29 } |  |  | 0.523 |
| walker |  | 9232 | 11 | Code::CodeKey { rung: Doc, file: bash_completions.go, decl: 1, sub: 0, line: 29 } |  |  | 0.523 |
| walker |  | 9294 | 62 | Code::CodeKey { rung: Doc, file: command.go, decl: 74, sub: 0, line: 1263 } |  |  | 0.523 |
| ns | 9326 |  | 228 | doc/: GenManTreeOptions and GenManHeader, field by field | 9.2 |  | 0.515 |
| ns | 9563 |  | 237 | doc/: the YAML document schema and the shared helpers | 9.3 |  | 0.508 |
| walker |  | 9585 | 291 | Code::CodeKey { rung: Names, file: bash_completions.go, decl: 0, sub: 1, line: 0 } |  |  | 0.508 |
| walker |  | 9600 | 15 | Code::CodeKey { rung: Doc, file: bash_completions.go, decl: 19, sub: 0, line: 701 } |  |  | 0.508 |
| walker |  | 9620 | 20 | Code::CodeKey { rung: Doc, file: bash_completions.go, decl: 10, sub: 0, line: 536 } |  |  | 0.508 |
| walker |  | 9640 | 20 | Code::CodeKey { rung: Doc, file: bash_completions.go, decl: 17, sub: 0, line: 683 } |  |  | 0.508 |
| walker |  | 9668 | 28 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 12, sub: 0, line: 91 } |  |  | 0.508 |
| ns | 9677 |  | 114 | docgen documentation pages: heading map | 9.4 |  | 0.505 |
| walker |  | 9696 | 28 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 16, sub: 0, line: 144 } |  |  | 0.505 |
| ns | 9929 |  | 252 | Project policy documents: section maps | 10.1 |  | 0.497 |
| walker |  | 9988 | 292 | Code::CodeKey { rung: Names, file: flag_groups.go, decl: 0, sub: 0, line: 0 } |  |  | 0.501 |
| ns | 9993 |  | 64 | Repository metadata: maintainers and local ignore rules | 10.2 |  | 0.498 |

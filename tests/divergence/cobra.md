Score(3000)=0.626 I=0.777 C=0.504 ns_rows≤3K=19/50 grid(1000/1442/2080/3000/4327/6240/9000)=0.596/0.792/0.736/0.626/0.577/0.529/0.500

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
| walker |  | 1078 | 94 | Markdown::HeadingsOutline { file: CONDUCT.md } |  |  | 0.596 |
| walker |  | 1177 | 99 | GoMod::File { file: go.mod } |  |  | 0.734 |
| walker |  | 1205 | 28 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.734 |
| ns | 1218 |  | 240 | Makefile: every target and its recipe | 1.8 |  | 0.769 |
| ns | 1304 |  | 86 | doc/ subpackage, CI workflows, and assets listings | 1.9 |  | 0.782 |
| ns | 1374 |  | 70 | Documentation site tree: site/content and its two subdirectories | 1.10 |  | 0.789 |
| walker |  | 1446 | 241 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 0, line: 0 } |  |  | 0.792 |
| walker |  | 1457 | 11 | Code::CodeKey { rung: Decl, file: command.go, decl: 1, sub: 0, line: 33 } |  |  | 0.792 |
| walker |  | 1478 | 21 | Code::CodeKey { rung: Decl, file: command.go, decl: 3, sub: 0, line: 45 } |  |  | 0.793 |
| walker |  | 1486 | 8 | Code::CodeKey { rung: Body, file: command.go, decl: 5, sub: 0, line: 269 } |  |  | 0.793 |
| walker |  | 1495 | 9 | Code::CodeKey { rung: Body, file: command.go, decl: 6, sub: 0, line: 275 } |  |  | 0.793 |
| walker |  | 1506 | 11 | Code::CodeKey { rung: Doc, file: command.go, decl: 3, sub: 0, line: 45 } |  |  | 0.794 |
| walker |  | 1522 | 16 | Code::CodeKey { rung: Doc, file: command.go, decl: 2, sub: 0, line: 42 } |  |  | 0.795 |
| walker |  | 1531 | 9 | Code::CodeKey { rung: Body, file: command.go, decl: 7, sub: 0, line: 281 } |  |  | 0.795 |
| walker |  | 1562 | 31 | Code::CodeKey { rung: Doc, file: command.go, decl: 9, sub: 0, line: 296 } |  |  | 0.795 |
| walker |  | 1593 | 31 | Code::CodeKey { rung: Doc, file: command.go, decl: 10, sub: 0, line: 302 } |  |  | 0.795 |
| ns | 1612 |  | 238 | command.go: exported constants, FParseErrWhitelist, Group, Command's doc | 2.1 |  | 0.771 |
| walker |  | 1624 | 31 | Code::CodeKey { rung: Doc, file: command.go, decl: 11, sub: 0, line: 308 } |  |  | 0.771 |
| walker |  | 1658 | 34 | Code::CodeKey { rung: Doc, file: command.go, decl: 6, sub: 0, line: 275 } |  |  | 0.771 |
| ns | 1684 |  | 72 | Command fields: naming and help text (Use .. Example) | 2.2 | 2.1 | 0.753 |
| walker |  | 1700 | 42 | Code::CodeKey { rung: Doc, file: command.go, decl: 7, sub: 0, line: 281 } |  |  | 0.753 |
| walker |  | 1753 | 53 | Code::CodeKey { rung: Doc, file: command.go, decl: 8, sub: 0, line: 289 } |  |  | 0.753 |
| ns | 1776 |  | 92 | Command fields: argument validation, completion, metadata | 2.3 | 2.1 | 0.734 |
| walker |  | 1966 | 213 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 1, line: 0 } |  |  | 0.736 |
| walker |  | 1978 | 12 | Code::CodeKey { rung: Doc, file: command.go, decl: 16, sub: 0, line: 338 } |  |  | 0.736 |
| walker |  | 1996 | 18 | Code::CodeKey { rung: Doc, file: command.go, decl: 13, sub: 0, line: 318 } |  |  | 0.736 |
| walker |  | 2014 | 18 | Code::CodeKey { rung: Doc, file: command.go, decl: 15, sub: 0, line: 333 } |  |  | 0.736 |
| walker |  | 2033 | 19 | Code::CodeKey { rung: Doc, file: command.go, decl: 12, sub: 0, line: 313 } |  |  | 0.736 |
| walker |  | 2052 | 19 | Code::CodeKey { rung: Doc, file: command.go, decl: 17, sub: 0, line: 343 } |  |  | 0.736 |
| walker |  | 2071 | 19 | Code::CodeKey { rung: Doc, file: command.go, decl: 18, sub: 0, line: 352 } |  |  | 0.736 |
| walker |  | 2095 | 24 | Code::CodeKey { rung: Doc, file: command.go, decl: 19, sub: 0, line: 358 } |  |  | 0.736 |
| walker |  | 2119 | 24 | Code::CodeKey { rung: Doc, file: command.go, decl: 20, sub: 0, line: 367 } |  |  | 0.736 |
| ns | 2125 |  | 349 | Command fields: the ten *Run hooks with the documented ordering comment | 2.4 | 2.1 | 0.692 |
| walker |  | 2144 | 25 | Code::CodeKey { rung: Doc, file: command.go, decl: 21, sub: 0, line: 376 } |  |  | 0.692 |
| walker |  | 2171 | 27 | Code::CodeKey { rung: Doc, file: command.go, decl: 14, sub: 0, line: 328 } |  |  | 0.692 |
| ns | 2264 |  | 139 | Command fields: behaviour toggles, with ellipses over the unexported state | 2.5 | 2.1 | 0.665 |
| ns | 2372 |  | 108 | Command method roster: execution and lifecycle | 3.1 |  | 0.651 |
| walker |  | 2398 | 227 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 2, line: 0 } |  |  | 0.653 |
| walker |  | 2412 | 14 | Code::CodeKey { rung: Doc, file: command.go, decl: 23, sub: 0, line: 393 } |  |  | 0.653 |
| walker |  | 2426 | 14 | Code::CodeKey { rung: Doc, file: command.go, decl: 24, sub: 0, line: 398 } |  |  | 0.653 |
| walker |  | 2440 | 14 | Code::CodeKey { rung: Doc, file: command.go, decl: 25, sub: 0, line: 403 } |  |  | 0.653 |
| walker |  | 2454 | 14 | Code::CodeKey { rung: Doc, file: command.go, decl: 26, sub: 0, line: 408 } |  |  | 0.653 |
| walker |  | 2485 | 31 | Code::CodeKey { rung: Doc, file: command.go, decl: 31, sub: 0, line: 464 } |  |  | 0.653 |
| ns | 2514 |  | 142 | Command method roster: building and traversing the command tree | 3.2 |  | 0.632 |
| walker |  | 2522 | 37 | Code::CodeKey { rung: Doc, file: command.go, decl: 30, sub: 0, line: 444 } |  |  | 0.632 |
| walker |  | 2561 | 39 | Code::CodeKey { rung: Doc, file: command.go, decl: 22, sub: 0, line: 382 } |  |  | 0.632 |
| ns | 2740 |  | 226 | Command method roster: the complete flag API | 3.3 |  | 0.605 |
| walker |  | 2799 | 238 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 3, line: 0 } |  |  | 0.606 |
| walker |  | 2810 | 11 | Code::CodeKey { rung: Doc, file: command.go, decl: 36, sub: 0, line: 526 } |  |  | 0.606 |
| walker |  | 2821 | 11 | Code::CodeKey { rung: Doc, file: command.go, decl: 39, sub: 0, line: 563 } |  |  | 0.606 |
| walker |  | 2832 | 11 | Code::CodeKey { rung: Doc, file: command.go, decl: 43, sub: 0, line: 583 } |  |  | 0.606 |
| walker |  | 2845 | 13 | Code::CodeKey { rung: Doc, file: command.go, decl: 41, sub: 0, line: 573 } |  |  | 0.606 |
| walker |  | 2875 | 30 | Code::CodeKey { rung: Doc, file: command.go, decl: 44, sub: 0, line: 592 } |  |  | 0.606 |
| walker |  | 2906 | 31 | Code::CodeKey { rung: Doc, file: command.go, decl: 34, sub: 0, line: 505 } |  |  | 0.606 |
| walker |  | 2945 | 39 | Code::CodeKey { rung: Doc, file: command.go, decl: 33, sub: 0, line: 484 } |  |  | 0.606 |
| ns | 2955 |  | 215 | Command method roster: I/O wiring and the Set* customisation hooks | 3.4 |  | 0.626 |
| walker |  | 2987 | 42 | Code::CodeKey { rung: Doc, file: command.go, decl: 32, sub: 0, line: 478 } |  |  | 0.626 |
| walker |  | 3030 | 43 | Code::CodeKey { rung: Doc, file: command.go, decl: 35, sub: 0, line: 520 } |  |  | 0.626 |
| walker |  | 3078 | 48 | Code::CodeKey { rung: Doc, file: command.go, decl: 37, sub: 0, line: 547 } |  |  | 0.626 |
| ns | 3142 |  | 187 | Command method roster: help, usage, templates, default-command injection | 3.5 |  | 0.618 |
| walker |  | 3290 | 212 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 4, line: 0 } |  |  | 0.628 |
| walker |  | 3305 | 15 | Code::CodeKey { rung: Doc, file: command.go, decl: 48, sub: 0, line: 643 } |  |  | 0.628 |
| ns | 3332 |  | 190 | Command method roster: naming and introspection predicates | 3.6 |  | 0.609 |
| walker |  | 3335 | 30 | Code::CodeKey { rung: Doc, file: command.go, decl: 45, sub: 0, line: 605 } |  |  | 0.609 |
| walker |  | 3365 | 30 | Code::CodeKey { rung: Doc, file: command.go, decl: 46, sub: 0, line: 618 } |  |  | 0.609 |
| walker |  | 3396 | 31 | Code::CodeKey { rung: Doc, file: command.go, decl: 47, sub: 0, line: 631 } |  |  | 0.609 |
| walker |  | 3431 | 35 | Code::CodeKey { rung: Doc, file: command.go, decl: 54, sub: 0, line: 757 } |  |  | 0.609 |
| walker |  | 3500 | 69 | Code::CodeKey { rung: Doc, file: command.go, decl: 52, sub: 0, line: 715 } |  |  | 0.609 |
| ns | 3516 |  | 184 | user_guide.md: complete H2 section map | 4.1 |  | 0.591 |
| walker |  | 3720 | 220 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 5, line: 0 } |  |  | 0.599 |
| walker |  | 3730 | 10 | Code::CodeKey { rung: Doc, file: command.go, decl: 60, sub: 0, line: 892 } |  |  | 0.599 |
| walker |  | 3744 | 14 | Code::CodeKey { rung: Doc, file: command.go, decl: 58, sub: 0, line: 863 } |  |  | 0.599 |
| ns | 3754 |  | 238 | user_guide.md: complete H3/H4 subsection map | 4.2 | 4.1 | 0.578 |
| walker |  | 3764 | 20 | Code::CodeKey { rung: Doc, file: command.go, decl: 59, sub: 0, line: 884 } |  |  | 0.578 |
| walker |  | 3791 | 27 | Code::CodeKey { rung: Doc, file: command.go, decl: 57, sub: 0, line: 821 } |  |  | 0.578 |
| walker |  | 3827 | 36 | Code::CodeKey { rung: Doc, file: command.go, decl: 61, sub: 0, line: 901 } |  |  | 0.578 |
| walker |  | 3879 | 52 | Code::CodeKey { rung: Doc, file: command.go, decl: 65, sub: 0, line: 1062 } |  |  | 0.578 |
| ns | 3950 |  | 196 | Completion and Active Help documentation: H1/H2 maps | 4.3 |  | 0.563 |
| walker |  | 4123 | 244 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 6, line: 0 } |  |  | 0.593 |
| ns | 4133 |  | 183 | Test harness: the helpers every root-package test is written against | 4.4 |  | 0.577 |
| walker |  | 4135 | 12 | Code::CodeKey { rung: Doc, file: command.go, decl: 68, sub: 0, line: 1084 } |  |  | 0.577 |
| walker |  | 4148 | 13 | Code::CodeKey { rung: Doc, file: command.go, decl: 76, sub: 0, line: 1325 } |  |  | 0.577 |
| walker |  | 4167 | 19 | Code::CodeKey { rung: Doc, file: command.go, decl: 75, sub: 0, line: 1317 } |  |  | 0.577 |
| walker |  | 4188 | 21 | Code::CodeKey { rung: Doc, file: command.go, decl: 70, sub: 0, line: 1180 } |  |  | 0.577 |
| walker |  | 4234 | 46 | Code::CodeKey { rung: Doc, file: command.go, decl: 71, sub: 0, line: 1205 } |  |  | 0.577 |
| walker |  | 4282 | 48 | Code::CodeKey { rung: Doc, file: command.go, decl: 66, sub: 0, line: 1070 } |  |  | 0.577 |
| walker |  | 4336 | 54 | Code::CodeKey { rung: Doc, file: command.go, decl: 67, sub: 0, line: 1078 } |  |  | 0.578 |
| walker |  | 4392 | 56 | Code::CodeKey { rung: Doc, file: command.go, decl: 72, sub: 0, line: 1219 } |  |  | 0.578 |
| walker |  | 4454 | 62 | Code::CodeKey { rung: Doc, file: command.go, decl: 74, sub: 0, line: 1263 } |  |  | 0.578 |
| ns | 4458 |  | 325 | CI: the four jobs and the platform / Go-version matrix | 4.5 |  | 0.555 |
| walker |  | 4522 | 68 | Code::CodeKey { rung: Doc, file: command.go, decl: 73, sub: 0, line: 1238 } |  |  | 0.555 |
| ns | 4706 |  | 248 | golangci-lint configuration: the enabled linter set | 4.6 |  | 0.534 |
| walker |  | 4734 | 212 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 7, line: 0 } |  |  | 0.553 |
| walker |  | 4747 | 13 | Code::CodeKey { rung: Doc, file: command.go, decl: 80, sub: 0, line: 1332 } |  |  | 0.553 |
| walker |  | 4762 | 15 | Code::CodeKey { rung: Doc, file: command.go, decl: 82, sub: 0, line: 1371 } |  |  | 0.553 |
| walker |  | 4780 | 18 | Code::CodeKey { rung: Doc, file: command.go, decl: 81, sub: 0, line: 1342 } |  |  | 0.553 |
| walker |  | 4798 | 18 | Code::CodeKey { rung: Doc, file: command.go, decl: 86, sub: 0, line: 1401 } |  |  | 0.553 |
| walker |  | 4817 | 19 | Code::CodeKey { rung: Doc, file: command.go, decl: 85, sub: 0, line: 1396 } |  |  | 0.553 |
| walker |  | 4837 | 20 | Code::CodeKey { rung: Doc, file: command.go, decl: 84, sub: 0, line: 1386 } |  |  | 0.553 |
| walker |  | 4859 | 22 | Code::CodeKey { rung: Doc, file: command.go, decl: 83, sub: 0, line: 1376 } |  |  | 0.553 |
| ns | 5045 |  | 339 | Execute / ExecuteC: doc comments and the root-redirect rule | 5.1 | 3.1 | 0.546 |
| walker |  | 5094 | 235 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 8, line: 0 } |  |  | 0.550 |
| walker |  | 5110 | 16 | Code::CodeKey { rung: Doc, file: command.go, decl: 93, sub: 0, line: 1465 } |  |  | 0.550 |
| walker |  | 5130 | 20 | Code::CodeKey { rung: Doc, file: command.go, decl: 98, sub: 0, line: 1551 } |  |  | 0.550 |
| walker |  | 5151 | 21 | Code::CodeKey { rung: Doc, file: command.go, decl: 95, sub: 0, line: 1482 } |  |  | 0.550 |
| walker |  | 5172 | 21 | Code::CodeKey { rung: Doc, file: command.go, decl: 97, sub: 0, line: 1541 } |  |  | 0.550 |
| walker |  | 5198 | 26 | Code::CodeKey { rung: Doc, file: command.go, decl: 87, sub: 0, line: 1435 } |  |  | 0.550 |
| walker |  | 5226 | 28 | Code::CodeKey { rung: Doc, file: command.go, decl: 88, sub: 0, line: 1440 } |  |  | 0.550 |
| walker |  | 5254 | 28 | Code::CodeKey { rung: Doc, file: command.go, decl: 89, sub: 0, line: 1445 } |  |  | 0.550 |
| walker |  | 5282 | 28 | Code::CodeKey { rung: Doc, file: command.go, decl: 90, sub: 0, line: 1450 } |  |  | 0.550 |
| walker |  | 5312 | 30 | Code::CodeKey { rung: Doc, file: command.go, decl: 91, sub: 0, line: 1455 } |  |  | 0.550 |
| walker |  | 5342 | 30 | Code::CodeKey { rung: Doc, file: command.go, decl: 92, sub: 0, line: 1460 } |  |  | 0.550 |
| walker |  | 5372 | 30 | Code::CodeKey { rung: Doc, file: command.go, decl: 96, sub: 0, line: 1501 } |  |  | 0.550 |
| walker |  | 5409 | 37 | Code::CodeKey { rung: Doc, file: command.go, decl: 94, sub: 0, line: 1474 } |  |  | 0.550 |
| ns | 5583 |  | 538 | execute(): the ordered run pipeline | 5.2 | 5.1 | 0.517 |
| walker |  | 5653 | 244 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 9, line: 0 } |  |  | 0.539 |
| walker |  | 5668 | 15 | Code::CodeKey { rung: Doc, file: command.go, decl: 102, sub: 0, line: 1591 } |  |  | 0.539 |
| walker |  | 5683 | 15 | Code::CodeKey { rung: Doc, file: command.go, decl: 103, sub: 0, line: 1596 } |  |  | 0.539 |
| walker |  | 5700 | 17 | Code::CodeKey { rung: Doc, file: command.go, decl: 104, sub: 0, line: 1601 } |  |  | 0.539 |
| walker |  | 5717 | 17 | Code::CodeKey { rung: Doc, file: command.go, decl: 109, sub: 0, line: 1677 } |  |  | 0.539 |
| walker |  | 5737 | 20 | Code::CodeKey { rung: Doc, file: command.go, decl: 101, sub: 0, line: 1586 } |  |  | 0.539 |
| walker |  | 5758 | 21 | Code::CodeKey { rung: Doc, file: command.go, decl: 110, sub: 0, line: 1682 } |  |  | 0.539 |
| walker |  | 5789 | 31 | Code::CodeKey { rung: Doc, file: command.go, decl: 100, sub: 0, line: 1571 } |  |  | 0.539 |
| ns | 5809 |  | 226 | cobra.go: the package-level behaviour switches | 5.3 |  | 0.529 |
| walker |  | 5826 | 37 | Code::CodeKey { rung: Doc, file: command.go, decl: 105, sub: 0, line: 1607 } |  |  | 0.529 |
| walker |  | 5866 | 40 | Code::CodeKey { rung: Doc, file: command.go, decl: 99, sub: 0, line: 1562 } |  |  | 0.529 |
| ns | 5891 |  | 82 | cobra.go: package-level function roster | 5.4 |  | 0.525 |
| walker |  | 5907 | 41 | Code::CodeKey { rung: Doc, file: command.go, decl: 108, sub: 0, line: 1662 } |  |  | 0.525 |
| walker |  | 5960 | 53 | Code::CodeKey { rung: Doc, file: command.go, decl: 107, sub: 0, line: 1648 } |  |  | 0.525 |
| ns | 6188 |  | 297 | The default usage template, plus the help and version template constants | 5.5 |  | 0.516 |
| walker |  | 6200 | 240 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 10, line: 0 } |  |  | 0.529 |
| walker |  | 6214 | 14 | Code::CodeKey { rung: Doc, file: command.go, decl: 117, sub: 0, line: 1787 } |  |  | 0.529 |
| walker |  | 6231 | 17 | Code::CodeKey { rung: Doc, file: command.go, decl: 119, sub: 0, line: 1806 } |  |  | 0.529 |
| walker |  | 6250 | 19 | Code::CodeKey { rung: Doc, file: command.go, decl: 120, sub: 0, line: 1811 } |  |  | 0.529 |
| walker |  | 6270 | 20 | Code::CodeKey { rung: Doc, file: command.go, decl: 116, sub: 0, line: 1775 } |  |  | 0.529 |
| walker |  | 6291 | 21 | Code::CodeKey { rung: Doc, file: command.go, decl: 121, sub: 0, line: 1816 } |  |  | 0.529 |
| walker |  | 6315 | 24 | Code::CodeKey { rung: Doc, file: command.go, decl: 118, sub: 0, line: 1801 } |  |  | 0.529 |
| walker |  | 6350 | 35 | Code::CodeKey { rung: Doc, file: command.go, decl: 111, sub: 0, line: 1688 } |  |  | 0.529 |
| walker |  | 6389 | 39 | Code::CodeKey { rung: Doc, file: command.go, decl: 122, sub: 0, line: 1822 } |  |  | 0.529 |
| walker |  | 6435 | 46 | Code::CodeKey { rung: Doc, file: command.go, decl: 114, sub: 0, line: 1744 } |  |  | 0.529 |
| walker |  | 6482 | 47 | Code::CodeKey { rung: Doc, file: command.go, decl: 113, sub: 0, line: 1716 } |  |  | 0.529 |
| walker |  | 6529 | 47 | Code::CodeKey { rung: Doc, file: command.go, decl: 115, sub: 0, line: 1770 } |  |  | 0.529 |
| walker |  | 6580 | 51 | Code::CodeKey { rung: Doc, file: command.go, decl: 112, sub: 0, line: 1702 } |  |  | 0.529 |
| ns | 6635 |  | 447 | args.go: every PositionalArgs validator with its defining doc line | 6.1 |  | 0.516 |
| walker |  | 6816 | 236 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 11, line: 0 } |  |  | 0.531 |
| walker |  | 6844 | 28 | Code::CodeKey { rung: Decl, file: command.go, decl: 133, sub: 0, line: 1937 } |  |  | 0.531 |
| walker |  | 6857 | 13 | Code::CodeKey { rung: Doc, file: command.go, decl: 127, sub: 0, line: 1855 } |  |  | 0.531 |
| walker |  | 6870 | 13 | Code::CodeKey { rung: Doc, file: command.go, decl: 129, sub: 0, line: 1892 } |  |  | 0.531 |
| walker |  | 6886 | 16 | Code::CodeKey { rung: Doc, file: command.go, decl: 128, sub: 0, line: 1868 } |  |  | 0.531 |
| ns | 6898 |  | 263 | flag_groups.go: the three group markers and their annotation keys | 6.2 |  | 0.524 |
| walker |  | 6903 | 17 | Code::CodeKey { rung: Doc, file: command.go, decl: 126, sub: 0, line: 1844 } |  |  | 0.524 |
| walker |  | 6923 | 20 | Code::CodeKey { rung: Doc, file: command.go, decl: 133, sub: 0, line: 1937 } |  |  | 0.524 |
| walker |  | 6947 | 24 | Code::CodeKey { rung: Doc, file: command.go, decl: 123, sub: 0, line: 1827 } |  |  | 0.524 |
| walker |  | 6980 | 33 | Code::CodeKey { rung: Doc, file: command.go, decl: 124, sub: 0, line: 1833 } |  |  | 0.524 |
| walker |  | 7013 | 33 | Code::CodeKey { rung: Doc, file: command.go, decl: 130, sub: 0, line: 1898 } |  |  | 0.524 |
| ns | 7027 |  | 129 | shell_completions.go: the complete Mark* helper family | 6.3 |  | 0.518 |
| walker |  | 7048 | 35 | Code::CodeKey { rung: Doc, file: command.go, decl: 125, sub: 0, line: 1839 } |  |  | 0.518 |
| walker |  | 7091 | 43 | Code::CodeKey { rung: Doc, file: command.go, decl: 132, sub: 0, line: 1928 } |  |  | 0.518 |
| walker |  | 7140 | 49 | Code::CodeKey { rung: Doc, file: command.go, decl: 131, sub: 0, line: 1907 } |  |  | 0.518 |
| ns | 7363 |  | 336 | completions.go: the completion type vocabulary | 7.1 |  | 0.507 |
| ns | 7610 |  | 247 | The ShellCompDirective bit-mask values | 7.2 |  | 0.500 |
| walker |  | 7679 | 539 | Code::CodeKey { rung: Decl, file: command.go, decl: 134, sub: 0, line: 1942 } |  |  | 0.512 |
| ns | 7747 |  | 137 | completions.go: exported completion API roster | 7.3 |  | 0.507 |
| walker |  | 7806 | 127 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 12, line: 0 } |  |  | 0.513 |
| walker |  | 7812 | 6 | Code::CodeKey { rung: Decl, file: command.go, decl: 138, sub: 0, line: 2064 } |  |  | 0.513 |
| walker |  | 7842 | 30 | Code::CodeKey { rung: Decl, file: command.go, decl: 136, sub: 0, line: 2042 } |  |  | 0.513 |
| walker |  | 7865 | 23 | Code::CodeKey { rung: Doc, file: command.go, decl: 135, sub: 0, line: 1974 } |  |  | 0.513 |
| walker |  | 7888 | 23 | Code::CodeKey { rung: Doc, file: command.go, decl: 137, sub: 0, line: 2047 } |  |  | 0.513 |
| walker |  | 7911 | 23 | Code::CodeKey { rung: Doc, file: command.go, decl: 139, sub: 0, line: 2068 } |  |  | 0.513 |
| walker |  | 8015 | 104 | Code::CodeKey { rung: Doc, file: command.go, decl: 106, sub: 0, line: 1628 } |  |  | 0.513 |
| ns | 8040 |  | 293 | The __complete protocol and its environment configuration | 7.4 |  | 0.505 |
| walker |  | 8130 | 115 | Code::CodeKey { rung: Doc, file: command.go, decl: 5, sub: 0, line: 269 } |  |  | 0.505 |
| ns | 8192 |  | 152 | The generated `completion` command tree | 7.5 | 7.3 | 0.501 |
| walker |  | 8341 | 211 | Markdown::Prelude { file: README.md } |  |  | 0.503 |
| walker |  | 8356 | 15 | Code::CodeKey { rung: Names, file: command_notwin.go, decl: 0, sub: 0, line: 0 } |  |  | 0.503 |
| ns | 8419 |  | 227 | Shell script generators: every exported Gen*Completion entry point | 8.1 |  | 0.497 |
| ns | 8515 |  | 96 | Legacy bash completion annotations | 8.2 |  | 0.495 |
| ns | 8706 |  | 191 | active_help.go: the whole feature in one batch | 8.3 |  | 0.490 |
| walker |  | 8777 | 421 | Code::CodeKey { rung: Names, file: cobra.go, decl: 0, sub: 0, line: 0 } |  |  | 0.503 |
| walker |  | 8786 | 9 | Code::CodeKey { rung: Decl, file: cobra.go, decl: 4, sub: 0, line: 45 } |  |  | 0.503 |
| walker |  | 8812 | 26 | Code::CodeKey { rung: Decl, file: cobra.go, decl: 9, sub: 0, line: 72 } |  |  | 0.503 |
| walker |  | 8824 | 12 | Code::CodeKey { rung: Body, file: cobra.go, decl: 11, sub: 0, line: 85 } |  |  | 0.503 |
| walker |  | 8837 | 13 | Code::CodeKey { rung: Body, file: cobra.go, decl: 13, sub: 0, line: 99 } |  |  | 0.503 |
| walker |  | 8853 | 16 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 19, sub: 0, line: 174 } |  |  | 0.503 |
| ns | 8920 |  | 214 | The Windows mousetrap hook and its no-op counterpart | 8.4 | 5.3 | 0.498 |
| walker |  | 8953 | 100 | Code::CodeKey { rung: Decl, file: cobra.go, decl: 1, sub: 0, line: 32 } |  |  | 0.498 |
| walker |  | 8972 | 19 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 7, sub: 0, line: 62 } |  |  | 0.500 |
| walker |  | 8991 | 19 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 21, sub: 0, line: 192 } |  |  | 0.500 |
| walker |  | 9016 | 25 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 11, sub: 0, line: 85 } |  |  | 0.500 |
| walker |  | 9041 | 25 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 24, sub: 0, line: 243 } |  |  | 0.500 |
| walker |  | 9069 | 28 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 12, sub: 0, line: 91 } |  |  | 0.500 |
| walker |  | 9097 | 28 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 16, sub: 0, line: 144 } |  |  | 0.500 |
| ns | 9098 |  | 178 | doc/: every exported generator across all four output formats | 9.1 |  | 0.494 |
| walker |  | 9126 | 29 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 13, sub: 0, line: 99 } |  |  | 0.494 |
| walker |  | 9155 | 29 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 14, sub: 0, line: 105 } |  |  | 0.494 |
| walker |  | 9185 | 30 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 18, sub: 0, line: 166 } |  |  | 0.494 |
| walker |  | 9219 | 34 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 23, sub: 0, line: 235 } |  |  | 0.494 |
| walker |  | 9254 | 35 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 6, sub: 0, line: 59 } |  |  | 0.495 |
| walker |  | 9297 | 43 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 8, sub: 0, line: 66 } |  |  | 0.497 |
| ns | 9326 |  | 228 | doc/: GenManTreeOptions and GenManHeader, field by field | 9.2 |  | 0.490 |
| walker |  | 9343 | 46 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 5, sub: 0, line: 55 } |  |  | 0.496 |
| walker |  | 9405 | 62 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 9, sub: 0, line: 72 } |  |  | 0.499 |
| walker |  | 9475 | 70 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 15, sub: 0, line: 114 } |  |  | 0.499 |
| walker |  | 9511 | 36 | Code::CodeKey { rung: Names, file: command_win.go, decl: 0, sub: 0, line: 0 } |  |  | 0.502 |
| ns | 9563 |  | 237 | doc/: the YAML document schema and the shared helpers | 9.3 |  | 0.495 |
| walker |  | 9597 | 86 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 10, sub: 0, line: 81 } |  |  | 0.495 |
| ns | 9677 |  | 114 | docgen documentation pages: heading map | 9.4 |  | 0.492 |
| walker |  | 9824 | 227 | Code::CodeKey { rung: Names, file: args.go, decl: 0, sub: 0, line: 0 } |  |  | 0.497 |
| walker |  | 9831 | 7 | Code::CodeKey { rung: Body, file: args.go, decl: 6, sub: 0, line: 82 } |  |  | 0.497 |
| walker |  | 9844 | 13 | Code::CodeKey { rung: Doc, file: args.go, decl: 6, sub: 0, line: 82 } |  |  | 0.498 |
| walker |  | 9860 | 16 | Code::CodeKey { rung: Doc, file: args.go, decl: 3, sub: 0, line: 42 } |  |  | 0.499 |
| walker |  | 9876 | 16 | Code::CodeKey { rung: Body, file: args.go, decl: 12, sub: 0, line: 142 } |  |  | 0.499 |
| walker |  | 9894 | 18 | Code::CodeKey { rung: Doc, file: args.go, decl: 9, sub: 0, line: 107 } |  |  | 0.500 |
| walker |  | 9912 | 18 | Code::CodeKey { rung: Doc, file: args.go, decl: 11, sub: 0, line: 127 } |  |  | 0.501 |
| ns | 9929 |  | 252 | Project policy documents: section maps | 10.1 |  | 0.496 |
| walker |  | 9931 | 19 | Code::CodeKey { rung: Doc, file: args.go, decl: 5, sub: 0, line: 69 } |  |  | 0.497 |
| walker |  | 9950 | 19 | Code::CodeKey { rung: Doc, file: args.go, decl: 8, sub: 0, line: 97 } |  |  | 0.499 |
| walker |  | 9970 | 20 | Code::CodeKey { rung: Doc, file: args.go, decl: 7, sub: 0, line: 87 } |  |  | 0.500 |
| walker |  | 9992 | 22 | Code::CodeKey { rung: Doc, file: args.go, decl: 10, sub: 0, line: 117 } |  |  | 0.502 |
| ns | 9993 |  | 64 | Repository metadata: maintainers and local ignore rules | 10.2 |  | 0.500 |

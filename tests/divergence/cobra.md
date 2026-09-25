Score(3000)=0.629 I=0.780 C=0.508 ns_rows≤3K=19/50 grid(1000/1442/2080/3000/4327/6240/9000)=0.596/0.795/0.740/0.629/0.580/0.520/0.500

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
| walker |  | 1012 | 28 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.596 |
| ns | 1218 |  | 240 | Makefile: every target and its recipe | 1.8 |  | 0.644 |
| walker |  | 1223 | 211 | Markdown::Prelude { file: README.md } |  |  | 0.651 |
| ns | 1304 |  | 86 | doc/ subpackage, CI workflows, and assets listings | 1.9 |  | 0.669 |
| walker |  | 1317 | 94 | Markdown::HeadingsOutline { file: CONDUCT.md } |  |  | 0.669 |
| ns | 1374 |  | 70 | Documentation site tree: site/content and its two subdirectories | 1.10 |  | 0.680 |
| walker |  | 1416 | 99 | GoMod::File { file: go.mod } |  |  | 0.795 |
| ns | 1612 |  | 238 | command.go: exported constants, FParseErrWhitelist, Group, Command's doc | 2.1 |  | 0.739 |
| walker |  | 1657 | 241 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 0, line: 0 } |  |  | 0.752 |
| walker |  | 1668 | 11 | Code::CodeKey { rung: Decl, file: command.go, decl: 1, sub: 0, line: 33 } |  |  | 0.756 |
| ns | 1684 |  | 72 | Command fields: naming and help text (Use .. Example) | 2.2 | 2.1 | 0.738 |
| walker |  | 1689 | 21 | Code::CodeKey { rung: Decl, file: command.go, decl: 3, sub: 0, line: 45 } |  |  | 0.745 |
| walker |  | 1697 | 8 | Code::CodeKey { rung: Body, file: command.go, decl: 5, sub: 0, line: 269 } |  |  | 0.745 |
| walker |  | 1706 | 9 | Code::CodeKey { rung: Body, file: command.go, decl: 6, sub: 0, line: 275 } |  |  | 0.745 |
| walker |  | 1717 | 11 | Code::CodeKey { rung: Doc, file: command.go, decl: 3, sub: 0, line: 45 } |  |  | 0.751 |
| walker |  | 1733 | 16 | Code::CodeKey { rung: Doc, file: command.go, decl: 2, sub: 0, line: 42 } |  |  | 0.758 |
| walker |  | 1742 | 9 | Code::CodeKey { rung: Body, file: command.go, decl: 7, sub: 0, line: 281 } |  |  | 0.758 |
| walker |  | 1773 | 31 | Code::CodeKey { rung: Doc, file: command.go, decl: 9, sub: 0, line: 296 } |  |  | 0.758 |
| ns | 1776 |  | 92 | Command fields: argument validation, completion, metadata | 2.3 | 2.1 | 0.739 |
| walker |  | 1804 | 31 | Code::CodeKey { rung: Doc, file: command.go, decl: 10, sub: 0, line: 302 } |  |  | 0.739 |
| walker |  | 1835 | 31 | Code::CodeKey { rung: Doc, file: command.go, decl: 11, sub: 0, line: 308 } |  |  | 0.739 |
| walker |  | 1869 | 34 | Code::CodeKey { rung: Doc, file: command.go, decl: 6, sub: 0, line: 275 } |  |  | 0.739 |
| walker |  | 1911 | 42 | Code::CodeKey { rung: Doc, file: command.go, decl: 7, sub: 0, line: 281 } |  |  | 0.739 |
| walker |  | 1964 | 53 | Code::CodeKey { rung: Doc, file: command.go, decl: 8, sub: 0, line: 289 } |  |  | 0.739 |
| ns | 2125 |  | 349 | Command fields: the ten *Run hooks with the documented ordering comment | 2.4 | 2.1 | 0.694 |
| walker |  | 2177 | 213 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 1, line: 0 } |  |  | 0.696 |
| walker |  | 2189 | 12 | Code::CodeKey { rung: Doc, file: command.go, decl: 16, sub: 0, line: 338 } |  |  | 0.696 |
| walker |  | 2207 | 18 | Code::CodeKey { rung: Doc, file: command.go, decl: 13, sub: 0, line: 318 } |  |  | 0.696 |
| walker |  | 2225 | 18 | Code::CodeKey { rung: Doc, file: command.go, decl: 15, sub: 0, line: 333 } |  |  | 0.696 |
| walker |  | 2244 | 19 | Code::CodeKey { rung: Doc, file: command.go, decl: 12, sub: 0, line: 313 } |  |  | 0.696 |
| walker |  | 2263 | 19 | Code::CodeKey { rung: Doc, file: command.go, decl: 17, sub: 0, line: 343 } |  |  | 0.696 |
| ns | 2264 |  | 139 | Command fields: behaviour toggles, with ellipses over the unexported state | 2.5 | 2.1 | 0.670 |
| walker |  | 2282 | 19 | Code::CodeKey { rung: Doc, file: command.go, decl: 18, sub: 0, line: 352 } |  |  | 0.670 |
| walker |  | 2306 | 24 | Code::CodeKey { rung: Doc, file: command.go, decl: 19, sub: 0, line: 358 } |  |  | 0.670 |
| walker |  | 2330 | 24 | Code::CodeKey { rung: Doc, file: command.go, decl: 20, sub: 0, line: 367 } |  |  | 0.670 |
| walker |  | 2355 | 25 | Code::CodeKey { rung: Doc, file: command.go, decl: 21, sub: 0, line: 376 } |  |  | 0.670 |
| ns | 2372 |  | 108 | Command method roster: execution and lifecycle | 3.1 |  | 0.655 |
| walker |  | 2382 | 27 | Code::CodeKey { rung: Doc, file: command.go, decl: 14, sub: 0, line: 328 } |  |  | 0.655 |
| ns | 2514 |  | 142 | Command method roster: building and traversing the command tree | 3.2 |  | 0.634 |
| walker |  | 2609 | 227 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 2, line: 0 } |  |  | 0.636 |
| walker |  | 2623 | 14 | Code::CodeKey { rung: Doc, file: command.go, decl: 23, sub: 0, line: 393 } |  |  | 0.636 |
| walker |  | 2637 | 14 | Code::CodeKey { rung: Doc, file: command.go, decl: 24, sub: 0, line: 398 } |  |  | 0.636 |
| walker |  | 2651 | 14 | Code::CodeKey { rung: Doc, file: command.go, decl: 25, sub: 0, line: 403 } |  |  | 0.636 |
| walker |  | 2665 | 14 | Code::CodeKey { rung: Doc, file: command.go, decl: 26, sub: 0, line: 408 } |  |  | 0.636 |
| walker |  | 2696 | 31 | Code::CodeKey { rung: Doc, file: command.go, decl: 31, sub: 0, line: 464 } |  |  | 0.636 |
| walker |  | 2733 | 37 | Code::CodeKey { rung: Doc, file: command.go, decl: 30, sub: 0, line: 444 } |  |  | 0.636 |
| ns | 2740 |  | 226 | Command method roster: the complete flag API | 3.3 |  | 0.609 |
| walker |  | 2772 | 39 | Code::CodeKey { rung: Doc, file: command.go, decl: 22, sub: 0, line: 382 } |  |  | 0.609 |
| ns | 2955 |  | 215 | Command method roster: I/O wiring and the Set* customisation hooks | 3.4 |  | 0.628 |
| walker |  | 3010 | 238 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 3, line: 0 } |  |  | 0.630 |
| walker |  | 3021 | 11 | Code::CodeKey { rung: Doc, file: command.go, decl: 36, sub: 0, line: 526 } |  |  | 0.630 |
| walker |  | 3032 | 11 | Code::CodeKey { rung: Doc, file: command.go, decl: 39, sub: 0, line: 563 } |  |  | 0.630 |
| walker |  | 3043 | 11 | Code::CodeKey { rung: Doc, file: command.go, decl: 43, sub: 0, line: 583 } |  |  | 0.630 |
| walker |  | 3056 | 13 | Code::CodeKey { rung: Doc, file: command.go, decl: 41, sub: 0, line: 573 } |  |  | 0.630 |
| walker |  | 3086 | 30 | Code::CodeKey { rung: Doc, file: command.go, decl: 44, sub: 0, line: 592 } |  |  | 0.630 |
| walker |  | 3117 | 31 | Code::CodeKey { rung: Doc, file: command.go, decl: 34, sub: 0, line: 505 } |  |  | 0.630 |
| ns | 3142 |  | 187 | Command method roster: help, usage, templates, default-command injection | 3.5 |  | 0.622 |
| walker |  | 3156 | 39 | Code::CodeKey { rung: Doc, file: command.go, decl: 33, sub: 0, line: 484 } |  |  | 0.622 |
| walker |  | 3198 | 42 | Code::CodeKey { rung: Doc, file: command.go, decl: 32, sub: 0, line: 478 } |  |  | 0.622 |
| walker |  | 3241 | 43 | Code::CodeKey { rung: Doc, file: command.go, decl: 35, sub: 0, line: 520 } |  |  | 0.622 |
| walker |  | 3289 | 48 | Code::CodeKey { rung: Doc, file: command.go, decl: 37, sub: 0, line: 547 } |  |  | 0.622 |
| ns | 3332 |  | 190 | Command method roster: naming and introspection predicates | 3.6 |  | 0.603 |
| walker |  | 3501 | 212 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 4, line: 0 } |  |  | 0.612 |
| walker |  | 3516 | 15 | Code::CodeKey { rung: Doc, file: command.go, decl: 48, sub: 0, line: 643 } |  |  | 0.595 |
| ns | 3516 |  | 184 | user_guide.md: complete H2 section map | 4.1 |  | 0.595 |
| walker |  | 3546 | 30 | Code::CodeKey { rung: Doc, file: command.go, decl: 45, sub: 0, line: 605 } |  |  | 0.595 |
| walker |  | 3576 | 30 | Code::CodeKey { rung: Doc, file: command.go, decl: 46, sub: 0, line: 618 } |  |  | 0.595 |
| walker |  | 3607 | 31 | Code::CodeKey { rung: Doc, file: command.go, decl: 47, sub: 0, line: 631 } |  |  | 0.595 |
| walker |  | 3642 | 35 | Code::CodeKey { rung: Doc, file: command.go, decl: 54, sub: 0, line: 757 } |  |  | 0.595 |
| walker |  | 3711 | 69 | Code::CodeKey { rung: Doc, file: command.go, decl: 52, sub: 0, line: 715 } |  |  | 0.595 |
| ns | 3754 |  | 238 | user_guide.md: complete H3/H4 subsection map | 4.2 | 4.1 | 0.574 |
| walker |  | 3931 | 220 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 5, line: 0 } |  |  | 0.581 |
| walker |  | 3941 | 10 | Code::CodeKey { rung: Doc, file: command.go, decl: 60, sub: 0, line: 892 } |  |  | 0.581 |
| ns | 3950 |  | 196 | Completion and Active Help documentation: H1/H2 maps | 4.3 |  | 0.566 |
| walker |  | 3955 | 14 | Code::CodeKey { rung: Doc, file: command.go, decl: 58, sub: 0, line: 863 } |  |  | 0.566 |
| walker |  | 3975 | 20 | Code::CodeKey { rung: Doc, file: command.go, decl: 59, sub: 0, line: 884 } |  |  | 0.566 |
| walker |  | 4002 | 27 | Code::CodeKey { rung: Doc, file: command.go, decl: 57, sub: 0, line: 821 } |  |  | 0.566 |
| walker |  | 4038 | 36 | Code::CodeKey { rung: Doc, file: command.go, decl: 61, sub: 0, line: 901 } |  |  | 0.566 |
| walker |  | 4090 | 52 | Code::CodeKey { rung: Doc, file: command.go, decl: 65, sub: 0, line: 1062 } |  |  | 0.566 |
| ns | 4133 |  | 183 | Test harness: the helpers every root-package test is written against | 4.4 |  | 0.551 |
| walker |  | 4334 | 244 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 6, line: 0 } |  |  | 0.580 |
| walker |  | 4346 | 12 | Code::CodeKey { rung: Doc, file: command.go, decl: 68, sub: 0, line: 1084 } |  |  | 0.580 |
| walker |  | 4359 | 13 | Code::CodeKey { rung: Doc, file: command.go, decl: 76, sub: 0, line: 1325 } |  |  | 0.580 |
| walker |  | 4378 | 19 | Code::CodeKey { rung: Doc, file: command.go, decl: 75, sub: 0, line: 1317 } |  |  | 0.580 |
| walker |  | 4399 | 21 | Code::CodeKey { rung: Doc, file: command.go, decl: 70, sub: 0, line: 1180 } |  |  | 0.580 |
| walker |  | 4445 | 46 | Code::CodeKey { rung: Doc, file: command.go, decl: 71, sub: 0, line: 1205 } |  |  | 0.580 |
| ns | 4458 |  | 325 | CI: the four jobs and the platform / Go-version matrix | 4.5 |  | 0.557 |
| walker |  | 4493 | 48 | Code::CodeKey { rung: Doc, file: command.go, decl: 66, sub: 0, line: 1070 } |  |  | 0.557 |
| walker |  | 4547 | 54 | Code::CodeKey { rung: Doc, file: command.go, decl: 67, sub: 0, line: 1078 } |  |  | 0.558 |
| walker |  | 4603 | 56 | Code::CodeKey { rung: Doc, file: command.go, decl: 72, sub: 0, line: 1219 } |  |  | 0.558 |
| walker |  | 4665 | 62 | Code::CodeKey { rung: Doc, file: command.go, decl: 74, sub: 0, line: 1263 } |  |  | 0.558 |
| ns | 4706 |  | 248 | golangci-lint configuration: the enabled linter set | 4.6 |  | 0.537 |
| walker |  | 4733 | 68 | Code::CodeKey { rung: Doc, file: command.go, decl: 73, sub: 0, line: 1238 } |  |  | 0.537 |
| walker |  | 4945 | 212 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 7, line: 0 } |  |  | 0.555 |
| walker |  | 4958 | 13 | Code::CodeKey { rung: Doc, file: command.go, decl: 80, sub: 0, line: 1332 } |  |  | 0.555 |
| walker |  | 4973 | 15 | Code::CodeKey { rung: Doc, file: command.go, decl: 82, sub: 0, line: 1371 } |  |  | 0.555 |
| walker |  | 4991 | 18 | Code::CodeKey { rung: Doc, file: command.go, decl: 81, sub: 0, line: 1342 } |  |  | 0.555 |
| walker |  | 5009 | 18 | Code::CodeKey { rung: Doc, file: command.go, decl: 86, sub: 0, line: 1401 } |  |  | 0.555 |
| walker |  | 5028 | 19 | Code::CodeKey { rung: Doc, file: command.go, decl: 85, sub: 0, line: 1396 } |  |  | 0.555 |
| ns | 5045 |  | 339 | Execute / ExecuteC: doc comments and the root-redirect rule | 5.1 | 3.1 | 0.549 |
| walker |  | 5048 | 20 | Code::CodeKey { rung: Doc, file: command.go, decl: 84, sub: 0, line: 1386 } |  |  | 0.549 |
| walker |  | 5070 | 22 | Code::CodeKey { rung: Doc, file: command.go, decl: 83, sub: 0, line: 1376 } |  |  | 0.549 |
| walker |  | 5305 | 235 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 8, line: 0 } |  |  | 0.552 |
| walker |  | 5321 | 16 | Code::CodeKey { rung: Doc, file: command.go, decl: 93, sub: 0, line: 1465 } |  |  | 0.552 |
| walker |  | 5341 | 20 | Code::CodeKey { rung: Doc, file: command.go, decl: 98, sub: 0, line: 1551 } |  |  | 0.552 |
| walker |  | 5362 | 21 | Code::CodeKey { rung: Doc, file: command.go, decl: 95, sub: 0, line: 1482 } |  |  | 0.552 |
| walker |  | 5383 | 21 | Code::CodeKey { rung: Doc, file: command.go, decl: 97, sub: 0, line: 1541 } |  |  | 0.552 |
| walker |  | 5409 | 26 | Code::CodeKey { rung: Doc, file: command.go, decl: 87, sub: 0, line: 1435 } |  |  | 0.552 |
| walker |  | 5437 | 28 | Code::CodeKey { rung: Doc, file: command.go, decl: 88, sub: 0, line: 1440 } |  |  | 0.552 |
| walker |  | 5465 | 28 | Code::CodeKey { rung: Doc, file: command.go, decl: 89, sub: 0, line: 1445 } |  |  | 0.552 |
| walker |  | 5493 | 28 | Code::CodeKey { rung: Doc, file: command.go, decl: 90, sub: 0, line: 1450 } |  |  | 0.552 |
| walker |  | 5523 | 30 | Code::CodeKey { rung: Doc, file: command.go, decl: 91, sub: 0, line: 1455 } |  |  | 0.552 |
| walker |  | 5553 | 30 | Code::CodeKey { rung: Doc, file: command.go, decl: 92, sub: 0, line: 1460 } |  |  | 0.552 |
| walker |  | 5583 | 30 | Code::CodeKey { rung: Doc, file: command.go, decl: 96, sub: 0, line: 1501 } |  |  | 0.520 |
| ns | 5583 |  | 538 | execute(): the ordered run pipeline | 5.2 | 5.1 | 0.520 |
| walker |  | 5620 | 37 | Code::CodeKey { rung: Doc, file: command.go, decl: 94, sub: 0, line: 1474 } |  |  | 0.520 |
| ns | 5809 |  | 226 | cobra.go: the package-level behaviour switches | 5.3 |  | 0.511 |
| walker |  | 5864 | 244 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 9, line: 0 } |  |  | 0.532 |
| walker |  | 5879 | 15 | Code::CodeKey { rung: Doc, file: command.go, decl: 102, sub: 0, line: 1591 } |  |  | 0.532 |
| ns | 5891 |  | 82 | cobra.go: package-level function roster | 5.4 |  | 0.527 |
| walker |  | 5894 | 15 | Code::CodeKey { rung: Doc, file: command.go, decl: 103, sub: 0, line: 1596 } |  |  | 0.527 |
| walker |  | 5911 | 17 | Code::CodeKey { rung: Doc, file: command.go, decl: 104, sub: 0, line: 1601 } |  |  | 0.527 |
| walker |  | 5928 | 17 | Code::CodeKey { rung: Doc, file: command.go, decl: 109, sub: 0, line: 1677 } |  |  | 0.527 |
| walker |  | 5948 | 20 | Code::CodeKey { rung: Doc, file: command.go, decl: 101, sub: 0, line: 1586 } |  |  | 0.527 |
| walker |  | 5969 | 21 | Code::CodeKey { rung: Doc, file: command.go, decl: 110, sub: 0, line: 1682 } |  |  | 0.527 |
| walker |  | 6000 | 31 | Code::CodeKey { rung: Doc, file: command.go, decl: 100, sub: 0, line: 1571 } |  |  | 0.527 |
| walker |  | 6037 | 37 | Code::CodeKey { rung: Doc, file: command.go, decl: 105, sub: 0, line: 1607 } |  |  | 0.527 |
| walker |  | 6077 | 40 | Code::CodeKey { rung: Doc, file: command.go, decl: 99, sub: 0, line: 1562 } |  |  | 0.527 |
| walker |  | 6118 | 41 | Code::CodeKey { rung: Doc, file: command.go, decl: 108, sub: 0, line: 1662 } |  |  | 0.527 |
| walker |  | 6171 | 53 | Code::CodeKey { rung: Doc, file: command.go, decl: 107, sub: 0, line: 1648 } |  |  | 0.527 |
| ns | 6188 |  | 297 | The default usage template, plus the help and version template constants | 5.5 |  | 0.519 |
| walker |  | 6411 | 240 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 10, line: 0 } |  |  | 0.531 |
| walker |  | 6425 | 14 | Code::CodeKey { rung: Doc, file: command.go, decl: 117, sub: 0, line: 1787 } |  |  | 0.531 |
| walker |  | 6442 | 17 | Code::CodeKey { rung: Doc, file: command.go, decl: 119, sub: 0, line: 1806 } |  |  | 0.531 |
| walker |  | 6461 | 19 | Code::CodeKey { rung: Doc, file: command.go, decl: 120, sub: 0, line: 1811 } |  |  | 0.531 |
| walker |  | 6481 | 20 | Code::CodeKey { rung: Doc, file: command.go, decl: 116, sub: 0, line: 1775 } |  |  | 0.531 |
| walker |  | 6502 | 21 | Code::CodeKey { rung: Doc, file: command.go, decl: 121, sub: 0, line: 1816 } |  |  | 0.531 |
| walker |  | 6526 | 24 | Code::CodeKey { rung: Doc, file: command.go, decl: 118, sub: 0, line: 1801 } |  |  | 0.531 |
| walker |  | 6561 | 35 | Code::CodeKey { rung: Doc, file: command.go, decl: 111, sub: 0, line: 1688 } |  |  | 0.531 |
| walker |  | 6600 | 39 | Code::CodeKey { rung: Doc, file: command.go, decl: 122, sub: 0, line: 1822 } |  |  | 0.531 |
| ns | 6635 |  | 447 | args.go: every PositionalArgs validator with its defining doc line | 6.1 |  | 0.519 |
| walker |  | 6646 | 46 | Code::CodeKey { rung: Doc, file: command.go, decl: 114, sub: 0, line: 1744 } |  |  | 0.519 |
| walker |  | 6693 | 47 | Code::CodeKey { rung: Doc, file: command.go, decl: 113, sub: 0, line: 1716 } |  |  | 0.519 |
| walker |  | 6740 | 47 | Code::CodeKey { rung: Doc, file: command.go, decl: 115, sub: 0, line: 1770 } |  |  | 0.519 |
| walker |  | 6791 | 51 | Code::CodeKey { rung: Doc, file: command.go, decl: 112, sub: 0, line: 1702 } |  |  | 0.519 |
| ns | 6898 |  | 263 | flag_groups.go: the three group markers and their annotation keys | 6.2 |  | 0.512 |
| walker |  | 7027 | 236 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 11, line: 0 } |  |  | 0.521 |
| ns | 7027 |  | 129 | shell_completions.go: the complete Mark* helper family | 6.3 |  | 0.521 |
| walker |  | 7055 | 28 | Code::CodeKey { rung: Decl, file: command.go, decl: 133, sub: 0, line: 1937 } |  |  | 0.521 |
| walker |  | 7068 | 13 | Code::CodeKey { rung: Doc, file: command.go, decl: 127, sub: 0, line: 1855 } |  |  | 0.521 |
| walker |  | 7081 | 13 | Code::CodeKey { rung: Doc, file: command.go, decl: 129, sub: 0, line: 1892 } |  |  | 0.521 |
| walker |  | 7097 | 16 | Code::CodeKey { rung: Doc, file: command.go, decl: 128, sub: 0, line: 1868 } |  |  | 0.521 |
| walker |  | 7114 | 17 | Code::CodeKey { rung: Doc, file: command.go, decl: 126, sub: 0, line: 1844 } |  |  | 0.521 |
| walker |  | 7134 | 20 | Code::CodeKey { rung: Doc, file: command.go, decl: 133, sub: 0, line: 1937 } |  |  | 0.521 |
| walker |  | 7158 | 24 | Code::CodeKey { rung: Doc, file: command.go, decl: 123, sub: 0, line: 1827 } |  |  | 0.521 |
| walker |  | 7191 | 33 | Code::CodeKey { rung: Doc, file: command.go, decl: 124, sub: 0, line: 1833 } |  |  | 0.521 |
| walker |  | 7224 | 33 | Code::CodeKey { rung: Doc, file: command.go, decl: 130, sub: 0, line: 1898 } |  |  | 0.521 |
| walker |  | 7259 | 35 | Code::CodeKey { rung: Doc, file: command.go, decl: 125, sub: 0, line: 1839 } |  |  | 0.521 |
| walker |  | 7302 | 43 | Code::CodeKey { rung: Doc, file: command.go, decl: 132, sub: 0, line: 1928 } |  |  | 0.521 |
| walker |  | 7351 | 49 | Code::CodeKey { rung: Doc, file: command.go, decl: 131, sub: 0, line: 1907 } |  |  | 0.521 |
| ns | 7363 |  | 336 | completions.go: the completion type vocabulary | 7.1 |  | 0.509 |
| ns | 7610 |  | 247 | The ShellCompDirective bit-mask values | 7.2 |  | 0.502 |
| ns | 7747 |  | 137 | completions.go: exported completion API roster | 7.3 |  | 0.497 |
| walker |  | 7890 | 539 | Code::CodeKey { rung: Decl, file: command.go, decl: 134, sub: 0, line: 1942 } |  |  | 0.509 |
| walker |  | 8017 | 127 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 12, line: 0 } |  |  | 0.516 |
| walker |  | 8023 | 6 | Code::CodeKey { rung: Decl, file: command.go, decl: 138, sub: 0, line: 2064 } |  |  | 0.516 |
| ns | 8040 |  | 293 | The __complete protocol and its environment configuration | 7.4 |  | 0.507 |
| walker |  | 8053 | 30 | Code::CodeKey { rung: Decl, file: command.go, decl: 136, sub: 0, line: 2042 } |  |  | 0.507 |
| walker |  | 8076 | 23 | Code::CodeKey { rung: Doc, file: command.go, decl: 135, sub: 0, line: 1974 } |  |  | 0.507 |
| walker |  | 8099 | 23 | Code::CodeKey { rung: Doc, file: command.go, decl: 137, sub: 0, line: 2047 } |  |  | 0.507 |
| walker |  | 8122 | 23 | Code::CodeKey { rung: Doc, file: command.go, decl: 139, sub: 0, line: 2068 } |  |  | 0.507 |
| ns | 8192 |  | 152 | The generated `completion` command tree | 7.5 | 7.3 | 0.503 |
| walker |  | 8226 | 104 | Code::CodeKey { rung: Doc, file: command.go, decl: 106, sub: 0, line: 1628 } |  |  | 0.503 |
| walker |  | 8341 | 115 | Code::CodeKey { rung: Doc, file: command.go, decl: 5, sub: 0, line: 269 } |  |  | 0.503 |
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
| walker |  | 9638 | 127 | Code::CodeKey { rung: Names, file: active_help.go, decl: 0, sub: 0, line: 0 } |  |  | 0.498 |
| ns | 9677 |  | 114 | docgen documentation pages: heading map | 9.4 |  | 0.495 |
| walker |  | 9687 | 49 | Code::CodeKey { rung: Decl, file: active_help.go, decl: 1, sub: 0, line: 22 } |  |  | 0.496 |
| walker |  | 9703 | 16 | Code::CodeKey { rung: Body, file: active_help.go, decl: 4, sub: 0, line: 58 } |  |  | 0.496 |
| walker |  | 9726 | 23 | Code::CodeKey { rung: Body, file: active_help.go, decl: 2, sub: 0, line: 38 } |  |  | 0.496 |
| walker |  | 9791 | 65 | Code::CodeKey { rung: Body, file: active_help.go, decl: 3, sub: 0, line: 47 } |  |  | 0.496 |
| walker |  | 9862 | 71 | Code::CodeKey { rung: Doc, file: active_help.go, decl: 4, sub: 0, line: 58 } |  |  | 0.496 |
| ns | 9929 |  | 252 | Project policy documents: section maps | 10.1 |  | 0.491 |
| walker |  | 9948 | 86 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 10, sub: 0, line: 81 } |  |  | 0.491 |
| walker |  | 9990 | 42 | Code::CodeKey { rung: Names, file: args.go, decl: 0, sub: 0, line: 0 } |  |  | 0.491 |
| ns | 9993 |  | 64 | Repository metadata: maintainers and local ignore rules | 10.2 |  | 0.489 |

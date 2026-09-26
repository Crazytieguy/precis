Score(3000)=0.673 I=0.796 C=0.569 ns_rows≤3K=19/50 grid(1000/1442/2080/3000/4327/6240/9000)=0.597/0.800/0.769/0.673/0.594/0.560/0.537

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
| walker |  | 1427 | 11 | Code::CodeKey { rung: Doc, file: command.go, decl: 3, sub: 0, line: 45 } |  |  | 0.800 |
| walker |  | 1443 | 16 | Code::CodeKey { rung: Doc, file: command.go, decl: 2, sub: 0, line: 42 } |  |  | 0.801 |
| walker |  | 1451 | 8 | Code::CodeKey { rung: Body, file: command.go, decl: 5, sub: 0, line: 269 } |  |  | 0.801 |
| ns | 1612 |  | 238 | command.go: exported constants, FParseErrWhitelist, Group, Command's doc | 2.1 |  | 0.776 |
| walker |  | 1663 | 212 | Code::CodeKey { rung: Decl, file: command.go, decl: 4, sub: 0, line: 54 } |  |  | 0.776 |
| ns | 1684 |  | 72 | Command fields: naming and help text (Use .. Example) | 2.2 | 2.1 | 0.758 |
| ns | 1776 |  | 92 | Command fields: argument validation, completion, metadata | 2.3 | 2.1 | 0.739 |
| walker |  | 1852 | 189 | Code::CodeKey { rung: Decl, file: command.go, decl: 4, sub: 1, line: 54 } |  |  | 0.756 |
| walker |  | 2074 | 222 | Code::CodeKey { rung: Decl, file: command.go, decl: 4, sub: 2, line: 54 } |  |  | 0.769 |
| ns | 2125 |  | 349 | Command fields: the ten *Run hooks with the documented ordering comment | 2.4 | 2.1 | 0.722 |
| ns | 2264 |  | 139 | Command fields: behaviour toggles, with ellipses over the unexported state | 2.5 | 2.1 | 0.695 |
| walker |  | 2323 | 249 | Code::CodeKey { rung: Decl, file: command.go, decl: 4, sub: 3, line: 54 } |  |  | 0.713 |
| ns | 2372 |  | 108 | Command method roster: execution and lifecycle | 3.1 |  | 0.697 |
| walker |  | 2496 | 173 | Code::CodeKey { rung: Decl, file: command.go, decl: 4, sub: 4, line: 54 } |  |  | 0.715 |
| ns | 2514 |  | 142 | Command method roster: building and traversing the command tree | 3.2 |  | 0.692 |
| walker |  | 2671 | 175 | Code::CodeKey { rung: Decl, file: command.go, decl: 4, sub: 5, line: 54 } |  |  | 0.711 |
| ns | 2740 |  | 226 | Command method roster: the complete flag API | 3.3 |  | 0.681 |
| walker |  | 2841 | 170 | Code::CodeKey { rung: Decl, file: command.go, decl: 4, sub: 6, line: 54 } |  |  | 0.701 |
| ns | 2955 |  | 215 | Command method roster: I/O wiring and the Set* customisation hooks | 3.4 |  | 0.673 |
| walker |  | 3036 | 195 | Code::CodeKey { rung: Decl, file: command.go, decl: 4, sub: 7, line: 54 } |  |  | 0.673 |
| ns | 3142 |  | 187 | Command method roster: help, usage, templates, default-command injection | 3.5 |  | 0.652 |
| walker |  | 3231 | 195 | Code::CodeKey { rung: Decl, file: command.go, decl: 4, sub: 8, line: 54 } |  |  | 0.652 |
| ns | 3332 |  | 190 | Command method roster: naming and introspection predicates | 3.6 |  | 0.632 |
| walker |  | 3426 | 195 | Code::CodeKey { rung: Decl, file: command.go, decl: 4, sub: 9, line: 54 } |  |  | 0.632 |
| ns | 3516 |  | 184 | user_guide.md: complete H2 section map | 4.1 |  | 0.614 |
| walker |  | 3645 | 219 | Code::CodeKey { rung: Decl, file: command.go, decl: 4, sub: 10, line: 54 } |  |  | 0.617 |
| ns | 3754 |  | 238 | user_guide.md: complete H3/H4 subsection map | 4.2 | 4.1 | 0.596 |
| walker |  | 3891 | 246 | Code::CodeKey { rung: Decl, file: command.go, decl: 4, sub: 11, line: 54 } |  |  | 0.599 |
| ns | 3950 |  | 196 | Completion and Active Help documentation: H1/H2 maps | 4.3 |  | 0.583 |
| ns | 4133 |  | 183 | Test harness: the helpers every root-package test is written against | 4.4 |  | 0.568 |
| walker |  | 4209 | 318 | Code::CodeKey { rung: Decl, file: command.go, decl: 4, sub: 12, line: 54 } |  |  | 0.590 |
| walker |  | 4383 | 174 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 1, line: 0 } |  |  | 0.599 |
| walker |  | 4395 | 12 | Code::CodeKey { rung: Doc, file: command.go, decl: 16, sub: 0, line: 338 } |  |  | 0.599 |
| walker |  | 4413 | 18 | Code::CodeKey { rung: Doc, file: command.go, decl: 13, sub: 0, line: 318 } |  |  | 0.599 |
| walker |  | 4431 | 18 | Code::CodeKey { rung: Doc, file: command.go, decl: 15, sub: 0, line: 333 } |  |  | 0.599 |
| walker |  | 4450 | 19 | Code::CodeKey { rung: Doc, file: command.go, decl: 12, sub: 0, line: 313 } |  |  | 0.599 |
| ns | 4458 |  | 325 | CI: the four jobs and the platform / Go-version matrix | 4.5 |  | 0.575 |
| walker |  | 4469 | 19 | Code::CodeKey { rung: Doc, file: command.go, decl: 17, sub: 0, line: 343 } |  |  | 0.575 |
| walker |  | 4641 | 172 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 2, line: 0 } |  |  | 0.597 |
| walker |  | 4655 | 14 | Code::CodeKey { rung: Doc, file: command.go, decl: 23, sub: 0, line: 393 } |  |  | 0.597 |
| walker |  | 4669 | 14 | Code::CodeKey { rung: Doc, file: command.go, decl: 24, sub: 0, line: 398 } |  |  | 0.597 |
| walker |  | 4683 | 14 | Code::CodeKey { rung: Doc, file: command.go, decl: 25, sub: 0, line: 403 } |  |  | 0.597 |
| walker |  | 4702 | 19 | Code::CodeKey { rung: Doc, file: command.go, decl: 18, sub: 0, line: 352 } |  |  | 0.597 |
| ns | 4706 |  | 248 | golangci-lint configuration: the enabled linter set | 4.6 |  | 0.575 |
| walker |  | 4726 | 24 | Code::CodeKey { rung: Doc, file: command.go, decl: 19, sub: 0, line: 358 } |  |  | 0.575 |
| walker |  | 4750 | 24 | Code::CodeKey { rung: Doc, file: command.go, decl: 20, sub: 0, line: 367 } |  |  | 0.575 |
| walker |  | 4923 | 173 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 3, line: 0 } |  |  | 0.579 |
| walker |  | 4937 | 14 | Code::CodeKey { rung: Doc, file: command.go, decl: 26, sub: 0, line: 408 } |  |  | 0.579 |
| walker |  | 4962 | 25 | Code::CodeKey { rung: Doc, file: command.go, decl: 21, sub: 0, line: 376 } |  |  | 0.579 |
| ns | 5045 |  | 339 | Execute / ExecuteC: doc comments and the root-redirect rule | 5.1 | 3.1 | 0.559 |
| walker |  | 5161 | 199 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 4, line: 0 } |  |  | 0.567 |
| walker |  | 5172 | 11 | Code::CodeKey { rung: Doc, file: command.go, decl: 36, sub: 0, line: 526 } |  |  | 0.567 |
| walker |  | 5183 | 11 | Code::CodeKey { rung: Doc, file: command.go, decl: 39, sub: 0, line: 563 } |  |  | 0.567 |
| walker |  | 5194 | 11 | Code::CodeKey { rung: Doc, file: command.go, decl: 43, sub: 0, line: 583 } |  |  | 0.567 |
| walker |  | 5207 | 13 | Code::CodeKey { rung: Doc, file: command.go, decl: 41, sub: 0, line: 573 } |  |  | 0.567 |
| walker |  | 5378 | 171 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 5, line: 0 } |  |  | 0.574 |
| walker |  | 5393 | 15 | Code::CodeKey { rung: Doc, file: command.go, decl: 48, sub: 0, line: 643 } |  |  | 0.574 |
| walker |  | 5577 | 184 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 6, line: 0 } |  |  | 0.578 |
| ns | 5583 |  | 538 | execute(): the ordered run pipeline | 5.2 | 5.1 | 0.544 |
| walker |  | 5587 | 10 | Code::CodeKey { rung: Doc, file: command.go, decl: 60, sub: 0, line: 892 } |  |  | 0.544 |
| walker |  | 5601 | 14 | Code::CodeKey { rung: Doc, file: command.go, decl: 58, sub: 0, line: 863 } |  |  | 0.544 |
| walker |  | 5621 | 20 | Code::CodeKey { rung: Doc, file: command.go, decl: 59, sub: 0, line: 884 } |  |  | 0.544 |
| walker |  | 5806 | 185 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 7, line: 0 } |  |  | 0.558 |
| ns | 5809 |  | 226 | cobra.go: the package-level behaviour switches | 5.3 |  | 0.548 |
| walker |  | 5818 | 12 | Code::CodeKey { rung: Doc, file: command.go, decl: 68, sub: 0, line: 1084 } |  |  | 0.549 |
| walker |  | 5839 | 21 | Code::CodeKey { rung: Doc, file: command.go, decl: 70, sub: 0, line: 1180 } |  |  | 0.549 |
| ns | 5891 |  | 82 | cobra.go: package-level function roster | 5.4 |  | 0.544 |
| walker |  | 6044 | 205 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 8, line: 0 } |  |  | 0.554 |
| walker |  | 6057 | 13 | Code::CodeKey { rung: Doc, file: command.go, decl: 76, sub: 0, line: 1325 } |  |  | 0.554 |
| walker |  | 6076 | 19 | Code::CodeKey { rung: Doc, file: command.go, decl: 75, sub: 0, line: 1317 } |  |  | 0.554 |
| ns | 6188 |  | 297 | The default usage template, plus the help and version template constants | 5.5 |  | 0.545 |
| walker |  | 6281 | 205 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 9, line: 0 } |  |  | 0.560 |
| walker |  | 6294 | 13 | Code::CodeKey { rung: Doc, file: command.go, decl: 80, sub: 0, line: 1332 } |  |  | 0.560 |
| walker |  | 6309 | 15 | Code::CodeKey { rung: Doc, file: command.go, decl: 82, sub: 0, line: 1371 } |  |  | 0.560 |
| walker |  | 6327 | 18 | Code::CodeKey { rung: Doc, file: command.go, decl: 81, sub: 0, line: 1342 } |  |  | 0.560 |
| walker |  | 6345 | 18 | Code::CodeKey { rung: Doc, file: command.go, decl: 86, sub: 0, line: 1401 } |  |  | 0.560 |
| walker |  | 6364 | 19 | Code::CodeKey { rung: Doc, file: command.go, decl: 85, sub: 0, line: 1396 } |  |  | 0.560 |
| walker |  | 6384 | 20 | Code::CodeKey { rung: Doc, file: command.go, decl: 84, sub: 0, line: 1386 } |  |  | 0.560 |
| walker |  | 6406 | 22 | Code::CodeKey { rung: Doc, file: command.go, decl: 83, sub: 0, line: 1376 } |  |  | 0.560 |
| walker |  | 6432 | 26 | Code::CodeKey { rung: Doc, file: command.go, decl: 87, sub: 0, line: 1435 } |  |  | 0.560 |
| walker |  | 6459 | 27 | Code::CodeKey { rung: Doc, file: command.go, decl: 14, sub: 0, line: 328 } |  |  | 0.560 |
| walker |  | 6486 | 27 | Code::CodeKey { rung: Doc, file: command.go, decl: 57, sub: 0, line: 821 } |  |  | 0.560 |
| ns | 6635 |  | 447 | args.go: every PositionalArgs validator with its defining doc line | 6.1 |  | 0.547 |
| walker |  | 6677 | 191 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 10, line: 0 } |  |  | 0.551 |
| walker |  | 6693 | 16 | Code::CodeKey { rung: Doc, file: command.go, decl: 93, sub: 0, line: 1465 } |  |  | 0.551 |
| walker |  | 6713 | 20 | Code::CodeKey { rung: Doc, file: command.go, decl: 98, sub: 0, line: 1551 } |  |  | 0.551 |
| walker |  | 6734 | 21 | Code::CodeKey { rung: Doc, file: command.go, decl: 95, sub: 0, line: 1482 } |  |  | 0.551 |
| walker |  | 6755 | 21 | Code::CodeKey { rung: Doc, file: command.go, decl: 97, sub: 0, line: 1541 } |  |  | 0.551 |
| ns | 6898 |  | 263 | flag_groups.go: the three group markers and their annotation keys | 6.2 |  | 0.543 |
| walker |  | 6949 | 194 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 11, line: 0 } |  |  | 0.561 |
| walker |  | 6964 | 15 | Code::CodeKey { rung: Doc, file: command.go, decl: 102, sub: 0, line: 1591 } |  |  | 0.561 |
| walker |  | 6979 | 15 | Code::CodeKey { rung: Doc, file: command.go, decl: 103, sub: 0, line: 1596 } |  |  | 0.561 |
| walker |  | 6996 | 17 | Code::CodeKey { rung: Doc, file: command.go, decl: 104, sub: 0, line: 1601 } |  |  | 0.561 |
| walker |  | 7013 | 17 | Code::CodeKey { rung: Doc, file: command.go, decl: 109, sub: 0, line: 1677 } |  |  | 0.561 |
| ns | 7027 |  | 129 | shell_completions.go: the complete Mark* helper family | 6.3 |  | 0.555 |
| walker |  | 7033 | 20 | Code::CodeKey { rung: Doc, file: command.go, decl: 101, sub: 0, line: 1586 } |  |  | 0.555 |
| walker |  | 7211 | 178 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 12, line: 0 } |  |  | 0.559 |
| walker |  | 7225 | 14 | Code::CodeKey { rung: Doc, file: command.go, decl: 117, sub: 0, line: 1787 } |  |  | 0.559 |
| walker |  | 7245 | 20 | Code::CodeKey { rung: Doc, file: command.go, decl: 116, sub: 0, line: 1775 } |  |  | 0.559 |
| walker |  | 7266 | 21 | Code::CodeKey { rung: Doc, file: command.go, decl: 110, sub: 0, line: 1682 } |  |  | 0.559 |
| ns | 7363 |  | 336 | completions.go: the completion type vocabulary | 7.1 |  | 0.547 |
| walker |  | 7469 | 203 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 13, line: 0 } |  |  | 0.563 |
| walker |  | 7482 | 13 | Code::CodeKey { rung: Doc, file: command.go, decl: 127, sub: 0, line: 1855 } |  |  | 0.563 |
| walker |  | 7499 | 17 | Code::CodeKey { rung: Doc, file: command.go, decl: 119, sub: 0, line: 1806 } |  |  | 0.563 |
| walker |  | 7516 | 17 | Code::CodeKey { rung: Doc, file: command.go, decl: 126, sub: 0, line: 1844 } |  |  | 0.563 |
| walker |  | 7535 | 19 | Code::CodeKey { rung: Doc, file: command.go, decl: 120, sub: 0, line: 1811 } |  |  | 0.563 |
| walker |  | 7556 | 21 | Code::CodeKey { rung: Doc, file: command.go, decl: 121, sub: 0, line: 1816 } |  |  | 0.563 |
| walker |  | 7580 | 24 | Code::CodeKey { rung: Doc, file: command.go, decl: 118, sub: 0, line: 1801 } |  |  | 0.563 |
| walker |  | 7604 | 24 | Code::CodeKey { rung: Doc, file: command.go, decl: 123, sub: 0, line: 1827 } |  |  | 0.563 |
| ns | 7610 |  | 247 | The ShellCompDirective bit-mask values | 7.2 |  | 0.555 |
| ns | 7747 |  | 137 | completions.go: exported completion API roster | 7.3 |  | 0.549 |
| walker |  | 7858 | 254 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 14, line: 0 } |  |  | 0.554 |
| walker |  | 7864 | 6 | Code::CodeKey { rung: Decl, file: command.go, decl: 138, sub: 0, line: 2064 } |  |  | 0.554 |
| walker |  | 7892 | 28 | Code::CodeKey { rung: Decl, file: command.go, decl: 133, sub: 0, line: 1937 } |  |  | 0.554 |
| walker |  | 7922 | 30 | Code::CodeKey { rung: Decl, file: command.go, decl: 136, sub: 0, line: 2042 } |  |  | 0.554 |
| walker |  | 7935 | 13 | Code::CodeKey { rung: Doc, file: command.go, decl: 129, sub: 0, line: 1892 } |  |  | 0.554 |
| walker |  | 7951 | 16 | Code::CodeKey { rung: Doc, file: command.go, decl: 128, sub: 0, line: 1868 } |  |  | 0.554 |
| walker |  | 7971 | 20 | Code::CodeKey { rung: Doc, file: command.go, decl: 133, sub: 0, line: 1937 } |  |  | 0.554 |
| walker |  | 7994 | 23 | Code::CodeKey { rung: Doc, file: command.go, decl: 137, sub: 0, line: 2047 } |  |  | 0.554 |
| walker |  | 8017 | 23 | Code::CodeKey { rung: Doc, file: command.go, decl: 139, sub: 0, line: 2068 } |  |  | 0.554 |
| ns | 8040 |  | 293 | The __complete protocol and its environment configuration | 7.4 |  | 0.546 |
| ns | 8192 |  | 152 | The generated `completion` command tree | 7.5 | 7.3 | 0.541 |
| ns | 8419 |  | 227 | Shell script generators: every exported Gen*Completion entry point | 8.1 |  | 0.534 |
| ns | 8515 |  | 96 | Legacy bash completion annotations | 8.2 |  | 0.532 |
| walker |  | 8556 | 539 | Code::CodeKey { rung: Decl, file: command.go, decl: 134, sub: 0, line: 1942 } |  |  | 0.548 |
| walker |  | 8579 | 23 | Code::CodeKey { rung: Doc, file: command.go, decl: 135, sub: 0, line: 1974 } |  |  | 0.548 |
| walker |  | 8607 | 28 | Code::CodeKey { rung: Doc, file: command.go, decl: 88, sub: 0, line: 1440 } |  |  | 0.548 |
| walker |  | 8635 | 28 | Code::CodeKey { rung: Doc, file: command.go, decl: 89, sub: 0, line: 1445 } |  |  | 0.548 |
| walker |  | 8663 | 28 | Code::CodeKey { rung: Doc, file: command.go, decl: 90, sub: 0, line: 1450 } |  |  | 0.548 |
| walker |  | 8693 | 30 | Code::CodeKey { rung: Doc, file: command.go, decl: 44, sub: 0, line: 592 } |  |  | 0.548 |
| ns | 8706 |  | 191 | active_help.go: the whole feature in one batch | 8.3 |  | 0.543 |
| walker |  | 8723 | 30 | Code::CodeKey { rung: Doc, file: command.go, decl: 45, sub: 0, line: 605 } |  |  | 0.543 |
| walker |  | 8753 | 30 | Code::CodeKey { rung: Doc, file: command.go, decl: 46, sub: 0, line: 618 } |  |  | 0.543 |
| walker |  | 8783 | 30 | Code::CodeKey { rung: Doc, file: command.go, decl: 91, sub: 0, line: 1455 } |  |  | 0.543 |
| walker |  | 8813 | 30 | Code::CodeKey { rung: Doc, file: command.go, decl: 92, sub: 0, line: 1460 } |  |  | 0.543 |
| walker |  | 8843 | 30 | Code::CodeKey { rung: Doc, file: command.go, decl: 96, sub: 0, line: 1501 } |  |  | 0.543 |
| walker |  | 8874 | 31 | Code::CodeKey { rung: Doc, file: command.go, decl: 9, sub: 0, line: 296 } |  |  | 0.543 |
| walker |  | 8905 | 31 | Code::CodeKey { rung: Doc, file: command.go, decl: 10, sub: 0, line: 302 } |  |  | 0.543 |
| ns | 8920 |  | 214 | The Windows mousetrap hook and its no-op counterpart | 8.4 | 5.3 | 0.537 |
| walker |  | 8936 | 31 | Code::CodeKey { rung: Doc, file: command.go, decl: 11, sub: 0, line: 308 } |  |  | 0.537 |
| walker |  | 8967 | 31 | Code::CodeKey { rung: Doc, file: command.go, decl: 31, sub: 0, line: 464 } |  |  | 0.537 |
| walker |  | 8998 | 31 | Code::CodeKey { rung: Doc, file: command.go, decl: 34, sub: 0, line: 505 } |  |  | 0.537 |
| walker |  | 9029 | 31 | Code::CodeKey { rung: Doc, file: command.go, decl: 47, sub: 0, line: 631 } |  |  | 0.537 |
| walker |  | 9060 | 31 | Code::CodeKey { rung: Doc, file: command.go, decl: 100, sub: 0, line: 1571 } |  |  | 0.537 |
| walker |  | 9093 | 33 | Code::CodeKey { rung: Doc, file: command.go, decl: 124, sub: 0, line: 1833 } |  |  | 0.537 |
| ns | 9098 |  | 178 | doc/: every exported generator across all four output formats | 9.1 |  | 0.531 |
| walker |  | 9126 | 33 | Code::CodeKey { rung: Doc, file: command.go, decl: 130, sub: 0, line: 1898 } |  |  | 0.531 |
| walker |  | 9160 | 34 | Code::CodeKey { rung: Doc, file: command.go, decl: 6, sub: 0, line: 275 } |  |  | 0.531 |
| walker |  | 9195 | 35 | Code::CodeKey { rung: Doc, file: command.go, decl: 54, sub: 0, line: 757 } |  |  | 0.531 |
| walker |  | 9230 | 35 | Code::CodeKey { rung: Doc, file: command.go, decl: 111, sub: 0, line: 1688 } |  |  | 0.531 |
| walker |  | 9265 | 35 | Code::CodeKey { rung: Doc, file: command.go, decl: 125, sub: 0, line: 1839 } |  |  | 0.531 |
| walker |  | 9301 | 36 | Code::CodeKey { rung: Doc, file: command.go, decl: 61, sub: 0, line: 901 } |  |  | 0.531 |
| ns | 9326 |  | 228 | doc/: GenManTreeOptions and GenManHeader, field by field | 9.2 |  | 0.523 |
| walker |  | 9338 | 37 | Code::CodeKey { rung: Doc, file: command.go, decl: 30, sub: 0, line: 444 } |  |  | 0.523 |
| walker |  | 9375 | 37 | Code::CodeKey { rung: Doc, file: command.go, decl: 94, sub: 0, line: 1474 } |  |  | 0.523 |
| walker |  | 9412 | 37 | Code::CodeKey { rung: Doc, file: command.go, decl: 105, sub: 0, line: 1607 } |  |  | 0.523 |
| walker |  | 9451 | 39 | Code::CodeKey { rung: Doc, file: command.go, decl: 22, sub: 0, line: 382 } |  |  | 0.523 |
| walker |  | 9490 | 39 | Code::CodeKey { rung: Doc, file: command.go, decl: 33, sub: 0, line: 484 } |  |  | 0.523 |
| walker |  | 9529 | 39 | Code::CodeKey { rung: Doc, file: command.go, decl: 122, sub: 0, line: 1822 } |  |  | 0.523 |
| ns | 9563 |  | 237 | doc/: the YAML document schema and the shared helpers | 9.3 |  | 0.515 |
| walker |  | 9569 | 40 | Code::CodeKey { rung: Doc, file: command.go, decl: 99, sub: 0, line: 1562 } |  |  | 0.515 |
| walker |  | 9610 | 41 | Code::CodeKey { rung: Doc, file: command.go, decl: 108, sub: 0, line: 1662 } |  |  | 0.515 |
| walker |  | 9638 | 28 | Code::CodeKey { rung: Names, file: command_notwin.go, decl: 0, sub: 0, line: 0 } |  |  | 0.516 |
| ns | 9677 |  | 114 | docgen documentation pages: heading map | 9.4 |  | 0.513 |
| walker |  | 9680 | 42 | Code::CodeKey { rung: Doc, file: command.go, decl: 7, sub: 0, line: 281 } |  |  | 0.513 |
| walker |  | 9722 | 42 | Code::CodeKey { rung: Doc, file: command.go, decl: 32, sub: 0, line: 478 } |  |  | 0.513 |
| walker |  | 9765 | 43 | Code::CodeKey { rung: Doc, file: command.go, decl: 35, sub: 0, line: 520 } |  |  | 0.513 |
| walker |  | 9808 | 43 | Code::CodeKey { rung: Doc, file: command.go, decl: 132, sub: 0, line: 1928 } |  |  | 0.513 |
| walker |  | 9854 | 46 | Code::CodeKey { rung: Doc, file: command.go, decl: 71, sub: 0, line: 1205 } |  |  | 0.513 |
| walker |  | 9900 | 46 | Code::CodeKey { rung: Doc, file: command.go, decl: 114, sub: 0, line: 1744 } |  |  | 0.513 |
| ns | 9929 |  | 252 | Project policy documents: section maps | 10.1 |  | 0.504 |
| ns | 9993 |  | 64 | Repository metadata: maintainers and local ignore rules | 10.2 |  | 0.502 |
| walker |  | 9999 | 99 | Code::CodeKey { rung: Names, file: cobra.go, decl: 0, sub: 0, line: 0 } |  |  | 0.503 |

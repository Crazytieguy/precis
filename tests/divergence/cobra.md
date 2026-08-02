Score(3000)=0.681 I=0.798 C=0.582 ns_rows≤3K=19/50 (reached=10 partial=1 missing=8)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 98 |  | 98 | Identity: the README lede and the package doc comment | 1.1 |  | 0.000 |
| walker |  | 195 | 195 | listing of '.' |  |  | 0.000 |
| walker |  | 198 | 3 | listing of 'site' |  |  | 0.000 |
| walker |  | 202 | 4 | listing of 'assets' |  |  | 0.000 |
| ns | 227 |  | 129 | go.mod: module path, Go version floor, and the four dependencies | 1.2 |  | 0.000 |
| walker |  | 232 | 30 | go module identity in go.mod |  |  | 0.121 |
| walker |  | 280 | 48 | go package doc lede in command.go |  |  | 0.237 |
| walker |  | 307 | 27 | listing of 'site/content' |  |  | 0.238 |
| walker |  | 365 | 58 | listing of 'doc' |  |  | 0.243 |
| ns | 366 |  | 139 | Complete .go roster of the root package: sources and colocated tests | 1.3 |  | 0.509 |
| walker |  | 385 | 20 | listing of 'site/content/docgen' |  |  | 0.514 |
| walker |  | 398 | 13 | listing of '.github' |  |  | 0.520 |
| walker |  | 406 | 8 | listing of '.github/workflows' |  |  | 0.524 |
| ns | 507 |  | 141 | README 'Overview': what the library does, first half | 1.4 |  | 0.457 |
| ns | 724 |  | 217 | README: the remaining capabilities, and who builds on Cobra | 1.5 | 1.4 | 0.409 |
| ns | 926 |  | 202 | Canonical usage: the rootCmd literal and Execute() from the user guide | 1.6 |  | 0.352 |
| walker |  | 951 | 545 | YAML config at .github/workflows/test.yml |  |  | 0.353 |
| ns | 982 |  | 56 | Complete non-Go root listing: build, config, docs, governance | 1.7 |  | 0.399 |
| walker |  | 1047 | 96 | README headline in README.md |  |  | 0.573 |
| walker |  | 1069 | 22 | listing of 'site/content/completions' |  |  | 0.578 |
| ns | 1222 |  | 240 | Makefile: every target and its recipe | 1.8 |  | 0.501 |
| ns | 1306 |  | 84 | doc/ subpackage, CI workflows, and assets listings | 1.9 |  | 0.543 |
| ns | 1378 |  | 72 | Documentation site tree: site/content and its two subdirectories | 1.10 |  | 0.568 |
| walker |  | 1454 | 385 | plaintext config Makefile |  |  | 0.675 |
| ns | 1616 |  | 238 | command.go: exported constants, FParseErrWhitelist, Group, Command's doc | 2.1 |  | 0.627 |
| walker |  | 1651 | 197 | go decl names surface in command.go |  |  | 0.643 |
| walker |  | 1651 | 0 | go decl at command.go:42 |  |  | 0.643 |
| walker |  | 1651 | 0 | go decl at command.go:269 |  |  | 0.643 |
| walker |  | 1651 | 0 | go decl at command.go:275 |  |  | 0.643 |
| walker |  | 1651 | 0 | go decl at command.go:281 |  |  | 0.643 |
| walker |  | 1651 | 0 | go decl at command.go:289 |  |  | 0.643 |
| walker |  | 1658 | 7 | go decl at command.go:54 |  |  | 0.643 |
| walker |  | 1669 | 11 | go decl at command.go:33 |  |  | 0.645 |
| ns | 1688 |  | 72 | Command fields: naming and help text (Use .. Example) | 2.2 | 2.1 | 0.630 |
| walker |  | 1690 | 21 | go decl at command.go:45 |  |  | 0.637 |
| walker |  | 1699 | 9 | go struct field group at command.go:54 group 80 |  |  | 0.637 |
| walker |  | 1708 | 9 | go struct field group at command.go:54 group 115 |  |  | 0.637 |
| walker |  | 1718 | 10 | go struct field group at command.go:54 group 64 |  |  | 0.638 |
| walker |  | 1728 | 10 | go struct field group at command.go:54 group 74 |  |  | 0.640 |
| walker |  | 1738 | 10 | go struct field group at command.go:54 group 77 |  |  | 0.643 |
| walker |  | 1748 | 10 | go struct field group at command.go:54 group 83 |  |  | 0.647 |
| walker |  | 1758 | 10 | go struct field group at command.go:54 group 105 |  |  | 0.648 |
| walker |  | 1768 | 10 | go struct field group at command.go:54 group 195 |  |  | 0.648 |
| walker |  | 1778 | 10 | go struct field group at command.go:54 group 218 |  |  | 0.648 |
| ns | 1780 |  | 92 | Command fields: argument validation, completion, metadata | 2.3 | 2.1 | 0.632 |
| walker |  | 1788 | 10 | go struct field group at command.go:54 group 233 |  |  | 0.632 |
| walker |  | 1798 | 10 | go struct field group at command.go:54 group 259 |  |  | 0.632 |
| walker |  | 1809 | 11 | go struct field group at command.go:54 group 67 |  |  | 0.637 |
| walker |  | 1820 | 11 | go struct field group at command.go:54 group 149 |  |  | 0.637 |
| walker |  | 1831 | 11 | go struct field group at command.go:54 group 230 |  |  | 0.638 |
| walker |  | 1842 | 11 | go struct field group at command.go:54 group 255 |  |  | 0.638 |
| walker |  | 1854 | 12 | go struct field group at command.go:54 group 71 |  |  | 0.645 |
| walker |  | 1866 | 12 | go struct field group at command.go:54 group 93 |  |  | 0.647 |
| walker |  | 1878 | 12 | go struct field group at command.go:54 group 98 |  |  | 0.649 |
| walker |  | 1890 | 12 | go struct field group at command.go:54 group 102 |  |  | 0.653 |
| walker |  | 1902 | 12 | go struct field group at command.go:54 group 109 |  |  | 0.657 |
| walker |  | 1914 | 12 | go struct field group at command.go:54 group 192 |  |  | 0.657 |
| walker |  | 1926 | 12 | go struct field group at command.go:54 group 208 |  |  | 0.658 |
| walker |  | 1938 | 12 | go struct field group at command.go:54 group 236 |  |  | 0.658 |
| walker |  | 1950 | 12 | go struct field group at command.go:54 group 239 |  |  | 0.659 |
| walker |  | 1962 | 12 | go struct field group at command.go:54 group 243 |  |  | 0.659 |
| walker |  | 1975 | 13 | go struct field group at command.go:54 group 189 |  |  | 0.659 |
| walker |  | 1988 | 13 | go struct field group at command.go:54 group 247 |  |  | 0.660 |
| walker |  | 2002 | 14 | go struct field group at command.go:54 group 251 |  |  | 0.661 |
| walker |  | 2017 | 15 | go struct field group at command.go:54 group 205 |  |  | 0.662 |
| walker |  | 2028 | 11 | go decl doc at command.go:45 |  |  | 0.667 |
| walker |  | 2044 | 16 | go decl doc at command.go:42 |  |  | 0.674 |
| walker |  | 2069 | 25 | go struct field group at command.go:54 group 86 |  |  | 0.686 |
| walker |  | 2102 | 33 | go struct field group at command.go:54 group 198 |  |  | 0.686 |
| ns | 2129 |  | 349 | Command fields: the ten *Run hooks with the documented ordering comment | 2.4 | 2.1 | 0.644 |
| walker |  | 2147 | 45 | go struct field group at command.go:54 group 211 |  |  | 0.644 |
| walker |  | 2181 | 34 | go decl doc at command.go:275 |  |  | 0.644 |
| walker |  | 2218 | 37 | go decl doc at command.go:289 |  |  | 0.644 |
| ns | 2268 |  | 139 | Command fields: behaviour toggles, with ellipses over the unexported state | 2.5 | 2.1 | 0.648 |
| walker |  | 2281 | 63 | headings outline in README.md |  |  | 0.648 |
| walker |  | 2293 | 12 | README.md section #0 |  |  | 0.648 |
| ns | 2376 |  | 108 | Command method roster: execution and lifecycle | 3.1 |  | 0.633 |
| walker |  | 2392 | 99 | go module file go.mod |  |  | 0.728 |
| walker |  | 2434 | 42 | go decl doc at command.go:281 |  |  | 0.728 |
| walker |  | 2493 | 59 | go struct field group at command.go:54 group 221 |  |  | 0.728 |
| ns | 2518 |  | 142 | Command method roster: building and traversing the command tree | 3.2 |  | 0.706 |
| walker |  | 2560 | 67 | go decl doc at command.go:54 |  |  | 0.731 |
| walker |  | 2732 | 172 | go decl names surface #1 in command.go |  |  | 0.733 |
| walker |  | 2732 | 0 | go decl at command.go:296 |  |  | 0.733 |
| walker |  | 2732 | 0 | go decl at command.go:302 |  |  | 0.733 |
| walker |  | 2732 | 0 | go decl at command.go:308 |  |  | 0.733 |
| walker |  | 2732 | 0 | go decl at command.go:313 |  |  | 0.733 |
| walker |  | 2732 | 0 | go decl at command.go:318 |  |  | 0.733 |
| walker |  | 2732 | 0 | go decl at command.go:328 |  |  | 0.733 |
| walker |  | 2732 | 0 | go decl at command.go:333 |  |  | 0.733 |
| walker |  | 2732 | 0 | go decl at command.go:338 |  |  | 0.733 |
| walker |  | 2742 | 10 | go decl body at command.go:333 |  |  | 0.733 |
| ns | 2744 |  | 226 | Command method roster: the complete flag API | 3.3 |  | 0.701 |
| walker |  | 2752 | 10 | go decl body at command.go:338 |  |  | 0.701 |
| walker |  | 2764 | 12 | go decl doc at command.go:338 |  |  | 0.701 |
| walker |  | 2775 | 11 | go decl body at command.go:296 |  |  | 0.701 |
| walker |  | 2793 | 18 | go decl doc at command.go:318 |  |  | 0.701 |
| walker |  | 2811 | 18 | go decl doc at command.go:333 |  |  | 0.701 |
| walker |  | 2830 | 19 | go decl doc at command.go:313 |  |  | 0.701 |
| walker |  | 2857 | 27 | go decl doc at command.go:328 |  |  | 0.701 |
| walker |  | 2888 | 31 | go decl doc at command.go:296 |  |  | 0.701 |
| walker |  | 2919 | 31 | go decl doc at command.go:302 |  |  | 0.701 |
| walker |  | 2950 | 31 | go decl doc at command.go:308 |  |  | 0.701 |
| ns | 2959 |  | 215 | Command method roster: I/O wiring and the Set* customisation hooks | 3.4 |  | 0.681 |
| walker |  | 3056 | 106 | go package + imports in command.go |  |  | 0.702 |
| walker |  | 3131 | 75 | go decl names surface in fish_completions.go |  |  | 0.702 |
| walker |  | 3131 | 0 | go decl at fish_completions.go:25 |  |  | 0.702 |
| walker |  | 3131 | 0 | go decl at fish_completions.go:276 |  |  | 0.702 |
| walker |  | 3131 | 0 | go decl at fish_completions.go:284 |  |  | 0.702 |
| walker |  | 3145 | 14 | go decl doc at fish_completions.go:284 |  |  | 0.702 |
| ns | 3146 |  | 187 | Command method roster: help, usage, templates, default-command injection | 3.5 |  | 0.680 |
| walker |  | 3164 | 19 | go decl doc at fish_completions.go:276 |  |  | 0.680 |
| walker |  | 3258 | 94 | headings outline in CONDUCT.md |  |  | 0.680 |
| ns | 3336 |  | 190 | Command method roster: naming and introspection predicates | 3.6 |  | 0.660 |
| walker |  | 3469 | 211 | README prelude in README.md |  |  | 0.664 |
| ns | 3520 |  | 184 | user_guide.md: complete H2 section map | 4.1 |  | 0.645 |
| walker |  | 3577 | 108 | go decl names surface in bash_completionsV2.go |  |  | 0.645 |
| walker |  | 3577 | 0 | go decl at bash_completionsV2.go:24 |  |  | 0.645 |
| walker |  | 3577 | 0 | go decl at bash_completionsV2.go:31 |  |  | 0.645 |
| walker |  | 3577 | 0 | go decl at bash_completionsV2.go:470 |  |  | 0.645 |
| walker |  | 3577 | 0 | go decl at bash_completionsV2.go:482 |  |  | 0.645 |
| walker |  | 3596 | 19 | go decl doc at bash_completionsV2.go:470 |  |  | 0.645 |
| walker |  | 3611 | 15 | go decl body at bash_completionsV2.go:482 |  |  | 0.645 |
| walker |  | 3643 | 32 | go decl doc at bash_completionsV2.go:482 |  |  | 0.645 |
| walker |  | 3742 | 99 | go struct field group at command.go:54 group 172 |  |  | 0.645 |
| ns | 3758 |  | 238 | user_guide.md: complete H3/H4 subsection map | 4.2 | 4.1 | 0.622 |
| walker |  | 3916 | 174 | go decl names surface #2 in command.go |  |  | 0.646 |
| walker |  | 3916 | 0 | go decl at command.go:343 |  |  | 0.646 |
| walker |  | 3916 | 0 | go decl at command.go:352 |  |  | 0.646 |
| walker |  | 3916 | 0 | go decl at command.go:358 |  |  | 0.646 |
| walker |  | 3916 | 0 | go decl at command.go:367 |  |  | 0.646 |
| walker |  | 3916 | 0 | go decl at command.go:376 |  |  | 0.646 |
| walker |  | 3916 | 0 | go decl at command.go:382 |  |  | 0.646 |
| walker |  | 3916 | 0 | go decl at command.go:393 |  |  | 0.646 |
| walker |  | 3916 | 0 | go decl at command.go:398 |  |  | 0.646 |
| walker |  | 3926 | 10 | go decl body at command.go:376 |  |  | 0.646 |
| walker |  | 3940 | 14 | go decl doc at command.go:393 |  |  | 0.646 |
| walker |  | 3954 | 14 | go decl doc at command.go:398 |  |  | 0.629 |
| ns | 3954 |  | 196 | Completion and Active Help documentation: H1/H2 maps | 4.3 |  | 0.629 |
| walker |  | 3966 | 12 | go decl body at command.go:393 |  |  | 0.629 |
| walker |  | 3985 | 19 | go decl doc at command.go:343 |  |  | 0.629 |
| walker |  | 4004 | 19 | go decl doc at command.go:352 |  |  | 0.629 |
| walker |  | 4028 | 24 | go decl doc at command.go:358 |  |  | 0.629 |
| walker |  | 4052 | 24 | go decl doc at command.go:367 |  |  | 0.629 |
| walker |  | 4077 | 25 | go decl doc at command.go:376 |  |  | 0.629 |
| walker |  | 4116 | 39 | go decl doc at command.go:382 |  |  | 0.629 |
| ns | 4137 |  | 183 | Test harness: the helpers every root-package test is written against | 4.4 |  | 0.612 |
| walker |  | 4144 | 28 | README.md section #5 |  |  | 0.612 |
| walker |  | 4271 | 127 | go decl names surface in active_help.go |  |  | 0.613 |
| walker |  | 4271 | 0 | go decl at active_help.go:38 |  |  | 0.613 |
| walker |  | 4271 | 0 | go decl at active_help.go:47 |  |  | 0.613 |
| walker |  | 4271 | 0 | go decl at active_help.go:58 |  |  | 0.613 |
| walker |  | 4294 | 23 | go decl body at active_help.go:38 |  |  | 0.613 |
| walker |  | 4408 | 114 | go struct field group at command.go:54 group 152 |  |  | 0.613 |
| walker |  | 4420 | 12 | go decl body at command.go:398 |  |  | 0.613 |
| ns | 4462 |  | 325 | CI: the four jobs and the platform / Go-version matrix | 4.5 |  | 0.594 |
| walker |  | 4533 | 113 | go decl doc at command.go:269 |  |  | 0.594 |
| ns | 4710 |  | 248 | golangci-lint configuration: the enabled linter set | 4.6 |  | 0.572 |
| walker |  | 4954 | 421 | go decl names surface in cobra.go |  |  | 0.573 |
| walker |  | 4954 | 0 | go decl at cobra.go:55 |  |  | 0.573 |
| walker |  | 4954 | 0 | go decl at cobra.go:59 |  |  | 0.573 |
| walker |  | 4954 | 0 | go decl at cobra.go:62 |  |  | 0.573 |
| walker |  | 4954 | 0 | go decl at cobra.go:66 |  |  | 0.573 |
| walker |  | 4954 | 0 | go decl at cobra.go:81 |  |  | 0.573 |
| walker |  | 4954 | 0 | go decl at cobra.go:85 |  |  | 0.573 |
| walker |  | 4954 | 0 | go decl at cobra.go:91 |  |  | 0.573 |
| walker |  | 4954 | 0 | go decl at cobra.go:99 |  |  | 0.573 |
| walker |  | 4954 | 0 | go decl at cobra.go:105 |  |  | 0.573 |
| walker |  | 4954 | 0 | go decl at cobra.go:114 |  |  | 0.573 |
| walker |  | 4954 | 0 | go decl at cobra.go:144 |  |  | 0.573 |
| walker |  | 4954 | 0 | go decl at cobra.go:159 |  |  | 0.573 |
| walker |  | 4954 | 0 | go decl at cobra.go:166 |  |  | 0.573 |
| walker |  | 4954 | 0 | go decl at cobra.go:174 |  |  | 0.573 |
| walker |  | 4954 | 0 | go decl at cobra.go:179 |  |  | 0.573 |
| walker |  | 4954 | 0 | go decl at cobra.go:192 |  |  | 0.573 |
| walker |  | 4954 | 0 | go decl at cobra.go:225 |  |  | 0.573 |
| walker |  | 4954 | 0 | go decl at cobra.go:235 |  |  | 0.573 |
| walker |  | 4954 | 0 | go decl at cobra.go:243 |  |  | 0.573 |
| walker |  | 4963 | 9 | go decl at cobra.go:45 |  |  | 0.573 |
| walker |  | 4975 | 12 | go decl body at cobra.go:85 |  |  | 0.573 |
| walker |  | 4988 | 13 | go decl body at cobra.go:99 |  |  | 0.573 |
| walker |  | 5001 | 13 | go decl body at cobra.go:105 |  |  | 0.573 |
| walker |  | 5027 | 26 | go decl at cobra.go:72 |  |  | 0.573 |
| walker |  | 5046 | 19 | go decl doc at cobra.go:62 |  |  | 0.574 |
| ns | 5049 |  | 339 | Execute / ExecuteC: doc comments and the root-redirect rule | 5.1 | 3.1 | 0.554 |
| walker |  | 5071 | 25 | go decl doc at cobra.go:85 |  |  | 0.554 |
| walker |  | 5096 | 25 | go decl doc at cobra.go:243 |  |  | 0.554 |
| walker |  | 5124 | 28 | go decl doc at cobra.go:91 |  |  | 0.554 |
| walker |  | 5152 | 28 | go decl doc at cobra.go:144 |  |  | 0.554 |
| walker |  | 5181 | 29 | go decl doc at cobra.go:99 |  |  | 0.554 |
| walker |  | 5210 | 29 | go decl doc at cobra.go:105 |  |  | 0.554 |
| walker |  | 5244 | 34 | go decl doc at cobra.go:235 |  |  | 0.554 |
| walker |  | 5260 | 16 | go decl doc at cobra.go:174 |  |  | 0.554 |
| walker |  | 5279 | 19 | go decl doc at cobra.go:192 |  |  | 0.554 |
| walker |  | 5314 | 35 | go decl doc at cobra.go:59 |  |  | 0.554 |
| walker |  | 5357 | 43 | go decl doc at cobra.go:66 |  |  | 0.554 |
| walker |  | 5403 | 46 | go decl doc at cobra.go:55 |  |  | 0.555 |
| walker |  | 5433 | 30 | go decl doc at cobra.go:166 |  |  | 0.555 |
| walker |  | 5503 | 70 | go decl doc at cobra.go:114 |  |  | 0.555 |
| walker |  | 5565 | 62 | go decl doc at cobra.go:72 |  |  | 0.555 |
| ns | 5587 |  | 538 | execute(): the ordered run pipeline | 5.2 | 5.1 | 0.522 |
| walker |  | 5651 | 86 | go decl doc at cobra.go:81 |  |  | 0.522 |
| walker |  | 5740 | 89 | go package + imports in cobra.go |  |  | 0.522 |
| walker |  | 5751 | 11 | go decl body at command.go:302 |  |  | 0.522 |
| walker |  | 5790 | 39 | go package + imports in active_help.go |  |  | 0.522 |
| ns | 5813 |  | 226 | cobra.go: the package-level behaviour switches | 5.3 |  | 0.535 |
| ns | 5895 |  | 82 | cobra.go: package-level function roster | 5.4 |  | 0.542 |
| walker |  | 5960 | 170 | go decl names surface in powershell_completions.go |  |  | 0.542 |
| walker |  | 5960 | 0 | go decl at powershell_completions.go:28 |  |  | 0.542 |
| walker |  | 5960 | 0 | go decl at powershell_completions.go:313 |  |  | 0.542 |
| walker |  | 5960 | 0 | go decl at powershell_completions.go:320 |  |  | 0.542 |
| walker |  | 5960 | 0 | go decl at powershell_completions.go:331 |  |  | 0.542 |
| walker |  | 5960 | 0 | go decl at powershell_completions.go:337 |  |  | 0.542 |
| walker |  | 5960 | 0 | go decl at powershell_completions.go:342 |  |  | 0.542 |
| walker |  | 5960 | 0 | go decl at powershell_completions.go:348 |  |  | 0.542 |
| walker |  | 5974 | 14 | go decl body at powershell_completions.go:337 |  |  | 0.542 |
| walker |  | 5988 | 14 | go decl body at powershell_completions.go:348 |  |  | 0.542 |
| walker |  | 6006 | 18 | go decl doc at powershell_completions.go:331 |  |  | 0.542 |
| walker |  | 6021 | 15 | go decl body at powershell_completions.go:331 |  |  | 0.542 |
| walker |  | 6041 | 20 | go decl doc at powershell_completions.go:342 |  |  | 0.542 |
| walker |  | 6071 | 30 | go decl doc at powershell_completions.go:337 |  |  | 0.542 |
| walker |  | 6103 | 32 | go decl doc at powershell_completions.go:348 |  |  | 0.542 |
| walker |  | 6119 | 16 | go decl body at active_help.go:58 |  |  | 0.542 |
| ns | 6192 |  | 297 | The default usage template, plus the help and version template constants | 5.5 |  | 0.533 |
| walker |  | 6289 | 170 | go decl names surface #3 in command.go |  |  | 0.538 |
| walker |  | 6289 | 0 | go decl at command.go:403 |  |  | 0.538 |
| walker |  | 6289 | 0 | go decl at command.go:408 |  |  | 0.538 |
| walker |  | 6289 | 0 | go decl at command.go:412 |  |  | 0.538 |
| walker |  | 6289 | 0 | go decl at command.go:422 |  |  | 0.538 |
| walker |  | 6289 | 0 | go decl at command.go:432 |  |  | 0.538 |
| walker |  | 6289 | 0 | go decl at command.go:444 |  |  | 0.538 |
| walker |  | 6289 | 0 | go decl at command.go:464 |  |  | 0.538 |
| walker |  | 6289 | 0 | go decl at command.go:478 |  |  | 0.538 |
| walker |  | 6303 | 14 | go decl doc at command.go:403 |  |  | 0.538 |
| walker |  | 6317 | 14 | go decl doc at command.go:408 |  |  | 0.538 |
| walker |  | 6329 | 12 | go decl body at command.go:403 |  |  | 0.538 |
| walker |  | 6341 | 12 | go decl body at command.go:408 |  |  | 0.538 |
| walker |  | 6353 | 12 | go decl body at command.go:478 |  |  | 0.538 |
| walker |  | 6390 | 37 | go decl doc at command.go:444 |  |  | 0.538 |
| walker |  | 6432 | 42 | go decl doc at command.go:478 |  |  | 0.538 |
| walker |  | 6463 | 31 | go decl doc at command.go:464 |  |  | 0.538 |
| walker |  | 6474 | 11 | go decl body at command.go:308 |  |  | 0.538 |
| walker |  | 6505 | 31 | headings outline in site/content/completions/bash.md |  |  | 0.538 |
| walker |  | 6533 | 28 | go decl names surface in command_notwin.go |  |  | 0.539 |
| walker |  | 6540 | 7 | go package + imports in command_notwin.go |  |  | 0.539 |
| walker |  | 6591 | 51 | go package + imports in bash_completionsV2.go |  |  | 0.539 |
| ns | 6639 |  | 447 | args.go: every PositionalArgs validator with its defining doc line | 6.1 |  | 0.526 |
| walker |  | 6821 | 230 | go decl names surface in zsh_completions.go |  |  | 0.526 |
| walker |  | 6821 | 0 | go decl at zsh_completions.go:25 |  |  | 0.526 |
| walker |  | 6821 | 0 | go decl at zsh_completions.go:31 |  |  | 0.526 |
| walker |  | 6821 | 0 | go decl at zsh_completions.go:36 |  |  | 0.526 |
| walker |  | 6821 | 0 | go decl at zsh_completions.go:42 |  |  | 0.526 |
| walker |  | 6821 | 0 | go decl at zsh_completions.go:55 |  |  | 0.526 |
| walker |  | 6821 | 0 | go decl at zsh_completions.go:66 |  |  | 0.526 |
| walker |  | 6821 | 0 | go decl at zsh_completions.go:70 |  |  | 0.526 |
| walker |  | 6821 | 0 | go decl at zsh_completions.go:80 |  |  | 0.526 |
| walker |  | 6821 | 0 | go decl at zsh_completions.go:87 |  |  | 0.526 |
| walker |  | 6828 | 7 | go decl body at zsh_completions.go:55 |  |  | 0.526 |
| walker |  | 6835 | 7 | go decl body at zsh_completions.go:66 |  |  | 0.526 |
| walker |  | 6849 | 14 | go decl body at zsh_completions.go:31 |  |  | 0.526 |
| walker |  | 6863 | 14 | go decl body at zsh_completions.go:42 |  |  | 0.526 |
| walker |  | 6881 | 18 | go decl doc at zsh_completions.go:25 |  |  | 0.526 |
| walker |  | 6901 | 20 | go decl doc at zsh_completions.go:36 |  |  | 0.526 |
| ns | 6902 |  | 263 | flag_groups.go: the three group markers and their annotation keys | 6.2 |  | 0.519 |
| walker |  | 6931 | 30 | go decl doc at zsh_completions.go:31 |  |  | 0.519 |
| walker |  | 6963 | 32 | go decl doc at zsh_completions.go:42 |  |  | 0.519 |
| walker |  | 7014 | 51 | go package + imports in zsh_completions.go |  |  | 0.519 |
| walker |  | 7025 | 11 | go decl body at command.go:313 |  |  | 0.519 |
| ns | 7031 |  | 129 | shell_completions.go: the complete Mark* helper family | 6.3 |  | 0.514 |
| walker |  | 7181 | 156 | go decl names surface #4 in command.go |  |  | 0.517 |
| walker |  | 7181 | 0 | go decl at command.go:484 |  |  | 0.517 |
| walker |  | 7181 | 0 | go decl at command.go:505 |  |  | 0.517 |
| walker |  | 7181 | 0 | go decl at command.go:520 |  |  | 0.517 |
| walker |  | 7181 | 0 | go decl at command.go:526 |  |  | 0.517 |
| walker |  | 7181 | 0 | go decl at command.go:547 |  |  | 0.517 |
| walker |  | 7181 | 0 | go decl at command.go:563 |  |  | 0.517 |
| walker |  | 7192 | 11 | go decl doc at command.go:526 |  |  | 0.517 |
| walker |  | 7203 | 11 | go decl doc at command.go:563 |  |  | 0.517 |
| walker |  | 7223 | 20 | go decl body at command.go:520 |  |  | 0.517 |
| walker |  | 7262 | 39 | go decl doc at command.go:484 |  |  | 0.517 |
| walker |  | 7305 | 43 | go decl doc at command.go:520 |  |  | 0.517 |
| walker |  | 7353 | 48 | go decl doc at command.go:547 |  |  | 0.517 |
| ns | 7367 |  | 336 | completions.go: the completion type vocabulary | 7.1 |  | 0.506 |
| walker |  | 7384 | 31 | go decl doc at command.go:505 |  |  | 0.506 |
| ns | 7614 |  | 247 | The ShellCompDirective bit-mask values | 7.2 |  | 0.499 |
| walker |  | 7632 | 248 | go decl names surface in args.go |  |  | 0.507 |
| walker |  | 7632 | 0 | go decl at args.go:28 |  |  | 0.507 |
| walker |  | 7632 | 0 | go decl at args.go:42 |  |  | 0.507 |
| walker |  | 7632 | 0 | go decl at args.go:51 |  |  | 0.507 |
| walker |  | 7632 | 0 | go decl at args.go:69 |  |  | 0.507 |
| walker |  | 7632 | 0 | go decl at args.go:82 |  |  | 0.507 |
| walker |  | 7632 | 0 | go decl at args.go:87 |  |  | 0.507 |
| walker |  | 7632 | 0 | go decl at args.go:97 |  |  | 0.507 |
| walker |  | 7632 | 0 | go decl at args.go:107 |  |  | 0.507 |
| walker |  | 7632 | 0 | go decl at args.go:117 |  |  | 0.507 |
| walker |  | 7632 | 0 | go decl at args.go:127 |  |  | 0.507 |
| walker |  | 7632 | 0 | go decl at args.go:142 |  |  | 0.507 |
| walker |  | 7639 | 7 | go decl body at args.go:82 |  |  | 0.507 |
| walker |  | 7652 | 13 | go decl doc at args.go:82 |  |  | 0.508 |
| walker |  | 7668 | 16 | go decl doc at args.go:42 |  |  | 0.509 |
| walker |  | 7686 | 18 | go decl doc at args.go:107 |  |  | 0.511 |
| walker |  | 7704 | 18 | go decl doc at args.go:127 |  |  | 0.513 |
| walker |  | 7723 | 19 | go decl doc at args.go:69 |  |  | 0.515 |
| walker |  | 7742 | 19 | go decl doc at args.go:97 |  |  | 0.517 |
| ns | 7751 |  | 137 | completions.go: exported completion API roster | 7.3 |  | 0.511 |
| walker |  | 7762 | 20 | go decl doc at args.go:87 |  |  | 0.513 |
| walker |  | 7784 | 22 | go decl doc at args.go:117 |  |  | 0.516 |
| walker |  | 7821 | 37 | go decl doc at args.go:51 |  |  | 0.518 |
| walker |  | 7871 | 50 | go decl doc at args.go:142 |  |  | 0.518 |
| walker |  | 7908 | 37 | go package + imports in args.go |  |  | 0.518 |
| walker |  | 7966 | 58 | go package + imports in fish_completions.go |  |  | 0.518 |
| walker |  | 8024 | 58 | go package + imports in powershell_completions.go |  |  | 0.518 |
| ns | 8044 |  | 293 | The __complete protocol and its environment configuration | 7.4 |  | 0.510 |
| ns | 8196 |  | 152 | The generated `completion` command tree | 7.5 | 7.3 | 0.506 |
| walker |  | 8278 | 254 | go decl names surface in shell_completions.go |  |  | 0.519 |
| walker |  | 8278 | 0 | go decl at shell_completions.go:24 |  |  | 0.519 |
| walker |  | 8278 | 0 | go decl at shell_completions.go:31 |  |  | 0.519 |
| walker |  | 8278 | 0 | go decl at shell_completions.go:38 |  |  | 0.519 |
| walker |  | 8278 | 0 | go decl at shell_completions.go:44 |  |  | 0.519 |
| walker |  | 8278 | 0 | go decl at shell_completions.go:54 |  |  | 0.519 |
| walker |  | 8278 | 0 | go decl at shell_completions.go:61 |  |  | 0.519 |
| walker |  | 8278 | 0 | go decl at shell_completions.go:67 |  |  | 0.519 |
| walker |  | 8278 | 0 | go decl at shell_completions.go:77 |  |  | 0.519 |
| walker |  | 8278 | 0 | go decl at shell_completions.go:83 |  |  | 0.519 |
| walker |  | 8278 | 0 | go decl at shell_completions.go:90 |  |  | 0.519 |
| walker |  | 8278 | 0 | go decl at shell_completions.go:96 |  |  | 0.519 |
| walker |  | 8291 | 13 | go decl body at shell_completions.go:24 |  |  | 0.519 |
| walker |  | 8305 | 14 | go decl body at shell_completions.go:31 |  |  | 0.519 |
| walker |  | 8319 | 14 | go decl body at shell_completions.go:83 |  |  | 0.519 |
| walker |  | 8334 | 15 | go decl body at shell_completions.go:44 |  |  | 0.519 |
| walker |  | 8349 | 15 | go decl body at shell_completions.go:54 |  |  | 0.519 |
| walker |  | 8383 | 34 | go decl doc at shell_completions.go:96 |  |  | 0.519 |
| walker |  | 8418 | 35 | go decl doc at shell_completions.go:67 |  |  | 0.519 |
| ns | 8423 |  | 227 | Shell script generators: every exported Gen*Completion entry point | 8.1 |  | 0.525 |
| walker |  | 8452 | 34 | go decl doc at shell_completions.go:83 |  |  | 0.525 |
| walker |  | 8487 | 35 | go decl doc at shell_completions.go:44 |  |  | 0.525 |
| ns | 8519 |  | 96 | Legacy bash completion annotations | 8.2 |  | 0.523 |
| walker |  | 8529 | 42 | go decl doc at shell_completions.go:90 |  |  | 0.523 |
| walker |  | 8572 | 43 | go decl doc at shell_completions.go:61 |  |  | 0.523 |
| walker |  | 8621 | 49 | go decl doc at shell_completions.go:38 |  |  | 0.523 |
| walker |  | 8659 | 38 | go package + imports in shell_completions.go |  |  | 0.523 |
| walker |  | 8706 | 47 | go decl doc at shell_completions.go:24 |  |  | 0.523 |
| ns | 8710 |  | 191 | active_help.go: the whole feature in one batch | 8.3 |  | 0.522 |
| walker |  | 8757 | 51 | go decl doc at shell_completions.go:31 |  |  | 0.522 |
| ns | 8924 |  | 214 | The Windows mousetrap hook and its no-op counterpart | 8.4 | 5.3 | 0.520 |
| walker |  | 8952 | 195 | go struct field group at command.go:54 group 128 |  |  | 0.527 |
| walker |  | 8997 | 45 | go decl body at fish_completions.go:276 |  |  | 0.527 |
| walker |  | 9040 | 43 | headings outline in site/content/docgen/_index.md |  |  | 0.527 |
| walker |  | 9051 | 11 | go decl body at command.go:328 |  |  | 0.527 |
| walker |  | 9098 | 47 | go decl at active_help.go:22 |  |  | 0.527 |
| ns | 9102 |  | 178 | doc/: every exported generator across all four output formats | 9.1 |  | 0.521 |
| walker |  | 9194 | 96 | go decl doc at active_help.go:47 |  |  | 0.525 |
| walker |  | 9291 | 97 | go decl doc at shell_completions.go:77 |  |  | 0.525 |
| ns | 9330 |  | 228 | doc/: GenManTreeOptions and GenManHeader, field by field | 9.2 |  | 0.518 |
| ns | 9567 |  | 237 | doc/: the YAML document schema and the shared helpers | 9.3 |  | 0.510 |
| walker |  | 9602 | 311 | go decl names surface in flag_groups.go |  |  | 0.515 |
| walker |  | 9602 | 0 | go decl at flag_groups.go:33 |  |  | 0.515 |
| walker |  | 9602 | 0 | go decl at flag_groups.go:49 |  |  | 0.515 |
| walker |  | 9602 | 0 | go decl at flag_groups.go:65 |  |  | 0.515 |
| walker |  | 9602 | 0 | go decl at flag_groups.go:81 |  |  | 0.515 |
| walker |  | 9602 | 0 | go decl at flag_groups.go:111 |  |  | 0.515 |
| walker |  | 9602 | 0 | go decl at flag_groups.go:121 |  |  | 0.515 |
| walker |  | 9602 | 0 | go decl at flag_groups.go:144 |  |  | 0.515 |
| walker |  | 9602 | 0 | go decl at flag_groups.go:167 |  |  | 0.515 |
| walker |  | 9602 | 0 | go decl at flag_groups.go:188 |  |  | 0.515 |
| walker |  | 9602 | 0 | go decl at flag_groups.go:209 |  |  | 0.515 |
| walker |  | 9602 | 0 | go decl at flag_groups.go:225 |  |  | 0.515 |
| walker |  | 9613 | 11 | go decl at flag_groups.go:25 |  |  | 0.516 |
| walker |  | 9646 | 33 | go decl doc at flag_groups.go:81 |  |  | 0.518 |
| ns | 9681 |  | 114 | docgen documentation pages: heading map | 9.4 |  | 0.516 |
| walker |  | 9687 | 41 | go decl doc at flag_groups.go:33 |  |  | 0.518 |
| walker |  | 9729 | 42 | go decl doc at flag_groups.go:49 |  |  | 0.520 |
| walker |  | 9772 | 43 | go decl doc at flag_groups.go:65 |  |  | 0.522 |
| walker |  | 9836 | 64 | go package + imports in flag_groups.go |  |  | 0.522 |
| ns | 9933 |  | 252 | Project policy documents: section maps | 10.1 |  | 0.516 |
| walker |  | 9969 | 133 | go decl names surface in doc/util.go |  |  | 0.517 |
| walker |  | 9969 | 0 | go decl at doc/util.go:26 |  |  | 0.517 |
| walker |  | 9969 | 0 | go decl at doc/util.go:41 |  |  | 0.517 |
| ns | 9997 |  | 64 | Repository metadata: maintainers and local ignore rules | 10.2 |  | 0.514 |

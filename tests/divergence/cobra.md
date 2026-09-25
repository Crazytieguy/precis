Score(3000)=0.572 I=0.768 C=0.426 ns_rows≤3K=19/50 grid(1000/1442/2080/3000/4327/6240/9000)=0.577/0.569/0.700/0.572/0.483/0.415/0.511

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 98 |  | 98 | Identity: the README lede and the package doc comment | 1.1 |  | 0.000 |
| walker |  | 191 | 191 | listing of '.' |  |  | 0.000 |
| walker |  | 194 | 3 | listing of 'site' |  |  | 0.000 |
| walker |  | 199 | 5 | listing of 'assets' |  |  | 0.000 |
| walker |  | 214 | 15 | go names command_notwin.go |  |  | 0.000 |
| ns | 227 |  | 129 | go.mod: module path, Go version floor, and the four dependencies | 1.2 |  | 0.000 |
| walker |  | 244 | 30 | go module identity in go.mod |  |  | 0.121 |
| walker |  | 280 | 36 | go names command_win.go |  |  | 0.121 |
| walker |  | 306 | 26 | listing of 'site/content' |  |  | 0.122 |
| walker |  | 354 | 48 | go module doc command.go |  |  | 0.238 |
| ns | 366 |  | 139 | Complete .go roster of the root package: sources and colocated tests | 1.3 |  | 0.499 |
| walker |  | 413 | 59 | listing of 'doc' |  |  | 0.509 |
| walker |  | 426 | 13 | listing of '.github' |  |  | 0.515 |
| walker |  | 435 | 9 | listing of '.github/workflows' |  |  | 0.519 |
| ns | 507 |  | 141 | README 'Overview': what the library does, first half | 1.4 |  | 0.453 |
| walker |  | 531 | 96 | README headline in README.md |  |  | 0.666 |
| walker |  | 552 | 21 | listing of 'site/content/docgen' |  |  | 0.670 |
| walker |  | 575 | 23 | listing of 'site/content/completions' |  |  | 0.675 |
| walker |  | 650 | 75 | go names fish_completions.go |  |  | 0.675 |
| walker |  | 664 | 14 | go doc fish_completions.go:284 |  |  | 0.675 |
| ns | 724 |  | 217 | README: the remaining capabilities, and who builds on Cobra | 1.5 | 1.4 | 0.605 |
| walker |  | 727 | 63 | headings outline in README.md |  |  | 0.605 |
| walker |  | 739 | 12 | README.md section #0 |  |  | 0.605 |
| ns | 926 |  | 202 | Canonical usage: the rootCmd literal and Execute() from the user guide | 1.6 |  | 0.520 |
| ns | 978 |  | 52 | Complete non-Go root listing: build, config, docs, governance | 1.7 |  | 0.577 |
| walker |  | 1160 | 421 | go names cobra.go |  |  | 0.579 |
| walker |  | 1186 | 26 | go decl cobra.go:72 |  |  | 0.579 |
| walker |  | 1195 | 9 | go decl cobra.go:45 |  |  | 0.579 |
| walker |  | 1207 | 12 | go body cobra.go:85 |  |  | 0.579 |
| ns | 1218 |  | 240 | Makefile: every target and its recipe | 1.8 |  | 0.502 |
| walker |  | 1220 | 13 | go body cobra.go:99 |  |  | 0.502 |
| walker |  | 1233 | 13 | go body cobra.go:105 |  |  | 0.502 |
| ns | 1304 |  | 86 | doc/ subpackage, CI workflows, and assets listings | 1.9 |  | 0.544 |
| ns | 1374 |  | 70 | Documentation site tree: site/content and its two subdirectories | 1.10 |  | 0.569 |
| ns | 1612 |  | 238 | command.go: exported constants, FParseErrWhitelist, Group, Command's doc | 2.1 |  | 0.529 |
| walker |  | 1618 | 385 | plaintext config Makefile |  |  | 0.628 |
| ns | 1684 |  | 72 | Command fields: naming and help text (Use .. Example) | 2.2 | 2.1 | 0.614 |
| walker |  | 1717 | 99 | go module file go.mod |  |  | 0.718 |
| ns | 1776 |  | 92 | Command fields: argument validation, completion, metadata | 2.3 | 2.1 | 0.700 |
| walker |  | 1825 | 108 | go names bash_completionsV2.go |  |  | 0.700 |
| walker |  | 1840 | 15 | go body bash_completionsV2.go:482 |  |  | 0.700 |
| walker |  | 1859 | 19 | go doc bash_completionsV2.go:470 |  |  | 0.700 |
| walker |  | 1878 | 19 | go doc fish_completions.go:276 |  |  | 0.700 |
| walker |  | 2095 | 217 | go names bash_completions.go |  |  | 0.700 |
| walker |  | 2104 | 9 | go decl bash_completions.go:29 |  |  | 0.700 |
| walker |  | 2115 | 11 | go doc bash_completions.go:29 |  |  | 0.700 |
| ns | 2125 |  | 349 | Command fields: the ten *Run hooks with the documented ordering comment | 2.4 | 2.1 | 0.658 |
| walker |  | 2242 | 127 | go names active_help.go |  |  | 0.658 |
| walker |  | 2261 | 19 | go doc cobra.go:62 |  |  | 0.658 |
| ns | 2264 |  | 139 | Command fields: behaviour toggles, with ellipses over the unexported state | 2.5 | 2.1 | 0.633 |
| walker |  | 2284 | 23 | go body active_help.go:38 |  |  | 0.633 |
| ns | 2372 |  | 108 | Command method roster: execution and lifecycle | 3.1 |  | 0.617 |
| ns | 2514 |  | 142 | Command method roster: building and traversing the command tree | 3.2 |  | 0.598 |
| walker |  | 2582 | 298 | go names completions.go |  |  | 0.598 |
| walker |  | 2602 | 20 | go decl completions.go:47 |  |  | 0.598 |
| walker |  | 2625 | 23 | go decl completions.go:98 |  |  | 0.598 |
| walker |  | 2726 | 101 | go decl completions.go:28 |  |  | 0.599 |
| ns | 2740 |  | 226 | Command method roster: the complete flag API | 3.3 |  | 0.573 |
| ns | 2955 |  | 215 | Command method roster: I/O wiring and the Set* customisation hooks | 3.4 |  | 0.550 |
| walker |  | 2967 | 241 | go names command.go |  |  | 0.564 |
| walker |  | 2978 | 11 | go decl command.go:33 |  |  | 0.567 |
| walker |  | 2999 | 21 | go decl command.go:45 |  |  | 0.572 |
| walker |  | 3007 | 8 | go body command.go:269 |  |  | 0.572 |
| walker |  | 3016 | 9 | go body command.go:275 |  |  | 0.572 |
| walker |  | 3025 | 9 | go body command.go:281 |  |  | 0.572 |
| walker |  | 3036 | 11 | go doc command.go:45 |  |  | 0.576 |
| ns | 3142 |  | 187 | Command method roster: help, usage, templates, default-command injection | 3.5 |  | 0.558 |
| walker |  | 3206 | 170 | go names powershell_completions.go |  |  | 0.558 |
| walker |  | 3220 | 14 | go body powershell_completions.go:337 |  |  | 0.558 |
| walker |  | 3234 | 14 | go body powershell_completions.go:348 |  |  | 0.558 |
| walker |  | 3249 | 15 | go body powershell_completions.go:331 |  |  | 0.558 |
| walker |  | 3264 | 15 | go body powershell_completions.go:342 |  |  | 0.558 |
| walker |  | 3282 | 18 | go doc powershell_completions.go:331 |  |  | 0.558 |
| walker |  | 3331 | 49 | go decl active_help.go:22 |  |  | 0.558 |
| ns | 3332 |  | 190 | Command method roster: naming and introspection predicates | 3.6 |  | 0.542 |
| walker |  | 3431 | 100 | go decl cobra.go:32 |  |  | 0.542 |
| walker |  | 3463 | 32 | go doc bash_completionsV2.go:482 |  |  | 0.542 |
| walker |  | 3474 | 11 | go body command.go:296 |  |  | 0.542 |
| ns | 3516 |  | 184 | user_guide.md: complete H2 section map | 4.1 |  | 0.526 |
| walker |  | 3701 | 227 | go names args.go |  |  | 0.527 |
| walker |  | 3708 | 7 | go body args.go:82 |  |  | 0.527 |
| walker |  | 3721 | 13 | go doc args.go:82 |  |  | 0.527 |
| walker |  | 3737 | 16 | go doc args.go:42 |  |  | 0.527 |
| walker |  | 3753 | 16 | go body args.go:142 |  |  | 0.527 |
| ns | 3754 |  | 238 | user_guide.md: complete H3/H4 subsection map | 4.2 | 4.1 | 0.508 |
| walker |  | 3771 | 18 | go doc args.go:107 |  |  | 0.509 |
| walker |  | 3805 | 34 | go doc completions.go:45 |  |  | 0.509 |
| walker |  | 3899 | 94 | headings outline in CONDUCT.md |  |  | 0.509 |
| ns | 3950 |  | 196 | Completion and Active Help documentation: H1/H2 maps | 4.3 |  | 0.496 |
| walker |  | 4129 | 230 | go names zsh_completions.go |  |  | 0.496 |
| ns | 4133 |  | 183 | Test harness: the helpers every root-package test is written against | 4.4 |  | 0.483 |
| walker |  | 4136 | 7 | go body zsh_completions.go:55 |  |  | 0.483 |
| walker |  | 4143 | 7 | go body zsh_completions.go:66 |  |  | 0.483 |
| walker |  | 4157 | 14 | go body zsh_completions.go:31 |  |  | 0.483 |
| walker |  | 4171 | 14 | go body zsh_completions.go:42 |  |  | 0.483 |
| walker |  | 4186 | 15 | go body zsh_completions.go:25 |  |  | 0.483 |
| walker |  | 4201 | 15 | go body zsh_completions.go:36 |  |  | 0.483 |
| walker |  | 4219 | 18 | go doc args.go:127 |  |  | 0.483 |
| walker |  | 4247 | 28 | README.md section #5 |  |  | 0.483 |
| ns | 4458 |  | 325 | CI: the four jobs and the platform / Go-version matrix | 4.5 |  | 0.464 |
| walker |  | 4501 | 254 | go names shell_completions.go |  |  | 0.465 |
| walker |  | 4514 | 13 | go body shell_completions.go:24 |  |  | 0.465 |
| walker |  | 4528 | 14 | go body shell_completions.go:31 |  |  | 0.465 |
| walker |  | 4542 | 14 | go body shell_completions.go:83 |  |  | 0.465 |
| walker |  | 4557 | 15 | go body shell_completions.go:44 |  |  | 0.465 |
| walker |  | 4572 | 15 | go body shell_completions.go:54 |  |  | 0.465 |
| walker |  | 4587 | 15 | go body shell_completions.go:90 |  |  | 0.465 |
| walker |  | 4607 | 20 | go doc powershell_completions.go:342 |  |  | 0.465 |
| walker |  | 4618 | 11 | go body command.go:302 |  |  | 0.465 |
| walker |  | 4639 | 21 | go body cobra.go:243 |  |  | 0.465 |
| ns | 4706 |  | 248 | golangci-lint configuration: the enabled linter set | 4.6 |  | 0.447 |
| walker |  | 4950 | 311 | go names flag_groups.go |  |  | 0.448 |
| walker |  | 4961 | 11 | go decl flag_groups.go:25 |  |  | 0.448 |
| walker |  | 4994 | 33 | go doc flag_groups.go:81 |  |  | 0.448 |
| walker |  | 5035 | 41 | go doc flag_groups.go:33 |  |  | 0.448 |
| ns | 5045 |  | 339 | Execute / ExecuteC: doc comments and the root-redirect rule | 5.1 | 3.1 | 0.433 |
| walker |  | 5077 | 42 | go doc flag_groups.go:49 |  |  | 0.433 |
| walker |  | 5122 | 45 | go body fish_completions.go:276 |  |  | 0.433 |
| walker |  | 5138 | 16 | go body shell_completions.go:61 |  |  | 0.433 |
| walker |  | 5271 | 133 | go names doc/util.go |  |  | 0.433 |
| walker |  | 5289 | 18 | go doc zsh_completions.go:25 |  |  | 0.433 |
| walker |  | 5500 | 211 | README prelude in README.md |  |  | 0.436 |
| ns | 5583 |  | 538 | execute(): the ordered run pipeline | 5.2 | 5.1 | 0.410 |
| walker |  | 5642 | 142 | go names doc/md_docs.go |  |  | 0.410 |
| walker |  | 5653 | 11 | go doc doc/md_docs.go:52 |  |  | 0.410 |
| walker |  | 5666 | 13 | go doc doc/md_docs.go:57 |  |  | 0.410 |
| walker |  | 5687 | 21 | go body doc/md_docs.go:52 |  |  | 0.410 |
| walker |  | 5706 | 19 | go doc args.go:69 |  |  | 0.410 |
| ns | 5809 |  | 226 | cobra.go: the package-level behaviour switches | 5.3 |  | 0.413 |
| walker |  | 5851 | 145 | go names doc/yaml_docs.go |  |  | 0.413 |
| walker |  | 5862 | 11 | go doc doc/yaml_docs.go:88 |  |  | 0.413 |
| walker |  | 5875 | 13 | go doc doc/yaml_docs.go:93 |  |  | 0.413 |
| walker |  | 5890 | 15 | go doc doc/yaml_docs.go:60 |  |  | 0.413 |
| ns | 5891 |  | 82 | cobra.go: package-level function roster | 5.4 |  | 0.422 |
| walker |  | 5911 | 21 | go body doc/yaml_docs.go:88 |  |  | 0.422 |
| walker |  | 5922 | 11 | go body command.go:308 |  |  | 0.422 |
| walker |  | 5938 | 16 | go body active_help.go:58 |  |  | 0.422 |
| walker |  | 5981 | 43 | go doc flag_groups.go:65 |  |  | 0.422 |
| walker |  | 6006 | 25 | go doc cobra.go:85 |  |  | 0.422 |
| walker |  | 6019 | 13 | go doc completions.go:41 |  |  | 0.422 |
| ns | 6188 |  | 297 | The default usage template, plus the help and version template constants | 5.5 |  | 0.415 |
| walker |  | 6192 | 173 | go names doc/rest_docs.go |  |  | 0.415 |
| walker |  | 6206 | 14 | go doc doc/rest_docs.go:57 |  |  | 0.415 |
| walker |  | 6222 | 16 | go doc doc/rest_docs.go:62 |  |  | 0.415 |
| walker |  | 6239 | 17 | go body doc/rest_docs.go:57 |  |  | 0.415 |
| walker |  | 6444 | 205 | go names completions.go #1 |  |  | 0.416 |
| walker |  | 6632 | 188 | go decl completions.go:107 |  |  | 0.417 |
| ns | 6635 |  | 447 | args.go: every PositionalArgs validator with its defining doc line | 6.1 |  | 0.423 |
| walker |  | 6645 | 13 | go doc completions.go:107 |  |  | 0.423 |
| walker |  | 6658 | 13 | go doc completions.go:139 |  |  | 0.423 |
| walker |  | 6723 | 65 | go body active_help.go:47 |  |  | 0.423 |
| walker |  | 6788 | 65 | go body fish_completions.go:284 |  |  | 0.423 |
| walker |  | 6805 | 17 | go body shell_completions.go:67 |  |  | 0.423 |
| ns | 6898 |  | 263 | flag_groups.go: the three group markers and their annotation keys | 6.2 |  | 0.437 |
| ns | 7027 |  | 129 | shell_completions.go: the complete Mark* helper family | 6.3 |  | 0.447 |
| walker |  | 7052 | 247 | go names bash_completions.go #1 |  |  | 0.447 |
| walker |  | 7067 | 15 | go doc bash_completions.go:701 |  |  | 0.447 |
| walker |  | 7087 | 20 | go doc bash_completions.go:683 |  |  | 0.447 |
| walker |  | 7155 | 68 | go body bash_completionsV2.go:470 |  |  | 0.447 |
| walker |  | 7207 | 52 | go decl doc/yaml_docs.go:30 |  |  | 0.448 |
| ns | 7363 |  | 336 | completions.go: the completion type vocabulary | 7.1 |  | 0.466 |
| walker |  | 7416 | 209 | go decl completions.go:56 |  |  | 0.467 |
| walker |  | 7435 | 19 | go doc args.go:97 |  |  | 0.469 |
| walker |  | 7455 | 20 | go doc zsh_completions.go:36 |  |  | 0.469 |
| walker |  | 7488 | 33 | go doc doc/md_docs.go:133 |  |  | 0.469 |
| ns | 7610 |  | 247 | The ShellCompDirective bit-mask values | 7.2 |  | 0.473 |
| walker |  | 7701 | 213 | go names command.go #1 |  |  | 0.486 |
| walker |  | 7711 | 10 | go body command.go:333 |  |  | 0.486 |
| walker |  | 7746 | 35 | go doc doc/rest_docs.go:145 |  |  | 0.486 |
| ns | 7747 |  | 137 | completions.go: exported completion API roster | 7.3 |  | 0.484 |
| walker |  | 7771 | 25 | go doc cobra.go:243 |  |  | 0.484 |
| walker |  | 7801 | 30 | go doc powershell_completions.go:337 |  |  | 0.484 |
| walker |  | 8028 | 227 | go names doc/man_docs.go |  |  | 0.484 |
| ns | 8040 |  | 293 | The __complete protocol and its environment configuration | 7.4 |  | 0.487 |
| walker |  | 8061 | 33 | go decl doc/man_docs.go:84 |  |  | 0.487 |
| walker |  | 8119 | 58 | go decl doc/man_docs.go:94 |  |  | 0.487 |
| walker |  | 8151 | 32 | go doc doc/man_docs.go:84 |  |  | 0.487 |
| walker |  | 8187 | 36 | go doc doc/man_docs.go:48 |  |  | 0.487 |
| ns | 8192 |  | 152 | The generated `completion` command tree | 7.5 | 7.3 | 0.484 |
| walker |  | 8206 | 19 | go body shell_completions.go:77 |  |  | 0.484 |
| walker |  | 8226 | 20 | go doc args.go:87 |  |  | 0.486 |
| walker |  | 8236 | 10 | go body command.go:338 |  |  | 0.486 |
| walker |  | 8249 | 13 | go body completions.go:142 |  |  | 0.486 |
| ns | 8419 |  | 227 | Shell script generators: every exported Gen*Completion entry point | 8.1 |  | 0.496 |
| walker |  | 8454 | 205 | go names completions.go #2 |  |  | 0.500 |
| walker |  | 8486 | 32 | go decl completions.go:311 |  |  | 0.500 |
| walker |  | 8514 | 28 | go doc cobra.go:91 |  |  | 0.500 |
| ns | 8515 |  | 96 | Legacy bash completion annotations | 8.2 |  | 0.504 |
| walker |  | 8611 | 97 | headings outline in site/content/active_help.md |  |  | 0.506 |
| walker |  | 8621 | 10 | go body command.go:376 |  |  | 0.506 |
| ns | 8706 |  | 191 | active_help.go: the whole feature in one batch | 8.3 |  | 0.506 |
| walker |  | 8848 | 227 | go names command.go #2 |  |  | 0.516 |
| walker |  | 8880 | 32 | go doc powershell_completions.go:348 |  |  | 0.516 |
| ns | 8920 |  | 214 | The Windows mousetrap hook and its no-op counterpart | 8.4 | 5.3 | 0.511 |
| ns | 9098 |  | 178 | doc/: every exported generator across all four output formats | 9.1 |  | 0.519 |
| walker |  | 9109 | 229 | go decl completions.go:56 #1 |  |  | 0.525 |
| walker |  | 9126 | 17 | go body bash_completions.go:696 |  |  | 0.525 |
| walker |  | 9146 | 20 | go body shell_completions.go:96 |  |  | 0.525 |
| walker |  | 9177 | 31 | headings outline in site/content/completions/bash.md |  |  | 0.525 |
| walker |  | 9216 | 39 | go doc doc/man_docs.go:105 |  |  | 0.525 |
| walker |  | 9238 | 22 | go doc args.go:117 |  |  | 0.527 |
| walker |  | 9276 | 38 | go body doc/rest_docs.go:138 |  |  | 0.527 |
| walker |  | 9304 | 28 | go doc cobra.go:144 |  |  | 0.527 |
| ns | 9326 |  | 228 | doc/: GenManTreeOptions and GenManHeader, field by field | 9.2 |  | 0.526 |
| walker |  | 9400 | 96 | go doc active_help.go:47 |  |  | 0.530 |
| walker |  | 9430 | 30 | go doc zsh_completions.go:31 |  |  | 0.530 |
| walker |  | 9551 | 121 | go decl doc/yaml_docs.go:37 |  |  | 0.531 |
| ns | 9563 |  | 237 | doc/: the YAML document schema and the shared helpers | 9.3 |  | 0.539 |
| walker |  | 9572 | 21 | go body shell_completions.go:38 |  |  | 0.539 |
| ns | 9677 |  | 114 | docgen documentation pages: heading map | 9.4 |  | 0.536 |
| walker |  | 9784 | 212 | go decl command.go:54 |  |  | 0.536 |
| walker |  | 9813 | 29 | go doc cobra.go:99 |  |  | 0.536 |
| walker |  | 9856 | 43 | headings outline in site/content/docgen/_index.md |  |  | 0.537 |
| ns | 9929 |  | 252 | Project policy documents: section maps | 10.1 |  | 0.531 |
| ns | 9993 |  | 64 | Repository metadata: maintainers and local ignore rules | 10.2 |  | 0.528 |

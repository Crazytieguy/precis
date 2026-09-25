Score(3000)=0.572 I=0.768 C=0.426 ns_rows≤3K=19/50 grid(1000/1442/2080/3000/4327/6240/9000)=0.577/0.569/0.700/0.572/0.483/0.419/0.514

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 98 |  | 98 | Identity: the README lede and the package doc comment | 1.1 |  | 0.000 |
| walker |  | 191 | 191 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 194 | 3 | Fs::DirListing { dir: site } |  |  | 0.000 |
| walker |  | 199 | 5 | Fs::DirListing { dir: assets } |  |  | 0.000 |
| walker |  | 214 | 15 | Code::CodeKey { rung: Names, file: command_notwin.go, decl: 0, sub: 0, line: 0 } |  |  | 0.000 |
| ns | 227 |  | 129 | go.mod: module path, Go version floor, and the four dependencies | 1.2 |  | 0.000 |
| walker |  | 244 | 30 | GoMod::Identity { file: go.mod } |  |  | 0.121 |
| walker |  | 280 | 36 | Code::CodeKey { rung: Names, file: command_win.go, decl: 0, sub: 0, line: 0 } |  |  | 0.121 |
| walker |  | 339 | 59 | Fs::DirListing { dir: doc } |  |  | 0.124 |
| walker |  | 365 | 26 | Fs::DirListing { dir: site/content } |  |  | 0.125 |
| ns | 366 |  | 139 | Complete .go roster of the root package: sources and colocated tests | 1.3 |  | 0.443 |
| walker |  | 413 | 48 | Code::CodeKey { rung: ModuleDoc, file: command.go, decl: 0, sub: 0, line: 0 } |  |  | 0.509 |
| walker |  | 426 | 13 | Fs::DirListing { dir: .github } |  |  | 0.515 |
| walker |  | 435 | 9 | Fs::DirListing { dir: .github/workflows } |  |  | 0.519 |
| ns | 507 |  | 141 | README 'Overview': what the library does, first half | 1.4 |  | 0.453 |
| walker |  | 531 | 96 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.666 |
| walker |  | 552 | 21 | Fs::DirListing { dir: site/content/docgen } |  |  | 0.670 |
| walker |  | 575 | 23 | Fs::DirListing { dir: site/content/completions } |  |  | 0.675 |
| walker |  | 650 | 75 | Code::CodeKey { rung: Names, file: fish_completions.go, decl: 0, sub: 0, line: 0 } |  |  | 0.675 |
| walker |  | 664 | 14 | Code::CodeKey { rung: Doc, file: fish_completions.go, decl: 3, sub: 0, line: 284 } |  |  | 0.675 |
| ns | 724 |  | 217 | README: the remaining capabilities, and who builds on Cobra | 1.5 | 1.4 | 0.605 |
| walker |  | 727 | 63 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.605 |
| walker |  | 739 | 12 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.605 |
| ns | 926 |  | 202 | Canonical usage: the rootCmd literal and Execute() from the user guide | 1.6 |  | 0.520 |
| ns | 978 |  | 52 | Complete non-Go root listing: build, config, docs, governance | 1.7 |  | 0.577 |
| walker |  | 1160 | 421 | Code::CodeKey { rung: Names, file: cobra.go, decl: 0, sub: 0, line: 0 } |  |  | 0.579 |
| walker |  | 1186 | 26 | Code::CodeKey { rung: Decl, file: cobra.go, decl: 9, sub: 0, line: 72 } |  |  | 0.579 |
| walker |  | 1195 | 9 | Code::CodeKey { rung: Decl, file: cobra.go, decl: 4, sub: 0, line: 45 } |  |  | 0.579 |
| walker |  | 1207 | 12 | Code::CodeKey { rung: Body, file: cobra.go, decl: 11, sub: 0, line: 85 } |  |  | 0.579 |
| ns | 1218 |  | 240 | Makefile: every target and its recipe | 1.8 |  | 0.502 |
| walker |  | 1220 | 13 | Code::CodeKey { rung: Body, file: cobra.go, decl: 13, sub: 0, line: 99 } |  |  | 0.502 |
| walker |  | 1233 | 13 | Code::CodeKey { rung: Body, file: cobra.go, decl: 14, sub: 0, line: 105 } |  |  | 0.502 |
| ns | 1304 |  | 86 | doc/ subpackage, CI workflows, and assets listings | 1.9 |  | 0.544 |
| ns | 1374 |  | 70 | Documentation site tree: site/content and its two subdirectories | 1.10 |  | 0.569 |
| ns | 1612 |  | 238 | command.go: exported constants, FParseErrWhitelist, Group, Command's doc | 2.1 |  | 0.529 |
| walker |  | 1618 | 385 | Plaintext::Whole { file: Makefile } |  |  | 0.628 |
| ns | 1684 |  | 72 | Command fields: naming and help text (Use .. Example) | 2.2 | 2.1 | 0.614 |
| walker |  | 1717 | 99 | GoMod::File { file: go.mod } |  |  | 0.718 |
| ns | 1776 |  | 92 | Command fields: argument validation, completion, metadata | 2.3 | 2.1 | 0.700 |
| walker |  | 1825 | 108 | Code::CodeKey { rung: Names, file: bash_completionsV2.go, decl: 0, sub: 0, line: 0 } |  |  | 0.700 |
| walker |  | 1840 | 15 | Code::CodeKey { rung: Body, file: bash_completionsV2.go, decl: 4, sub: 0, line: 482 } |  |  | 0.700 |
| walker |  | 1859 | 19 | Code::CodeKey { rung: Doc, file: bash_completionsV2.go, decl: 3, sub: 0, line: 470 } |  |  | 0.700 |
| walker |  | 1878 | 19 | Code::CodeKey { rung: Doc, file: fish_completions.go, decl: 2, sub: 0, line: 276 } |  |  | 0.700 |
| walker |  | 2095 | 217 | Code::CodeKey { rung: Names, file: bash_completions.go, decl: 0, sub: 0, line: 0 } |  |  | 0.700 |
| walker |  | 2104 | 9 | Code::CodeKey { rung: Decl, file: bash_completions.go, decl: 1, sub: 0, line: 29 } |  |  | 0.700 |
| walker |  | 2115 | 11 | Code::CodeKey { rung: Doc, file: bash_completions.go, decl: 1, sub: 0, line: 29 } |  |  | 0.700 |
| ns | 2125 |  | 349 | Command fields: the ten *Run hooks with the documented ordering comment | 2.4 | 2.1 | 0.658 |
| walker |  | 2242 | 127 | Code::CodeKey { rung: Names, file: active_help.go, decl: 0, sub: 0, line: 0 } |  |  | 0.658 |
| walker |  | 2261 | 19 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 7, sub: 0, line: 62 } |  |  | 0.658 |
| ns | 2264 |  | 139 | Command fields: behaviour toggles, with ellipses over the unexported state | 2.5 | 2.1 | 0.633 |
| walker |  | 2284 | 23 | Code::CodeKey { rung: Body, file: active_help.go, decl: 2, sub: 0, line: 38 } |  |  | 0.633 |
| ns | 2372 |  | 108 | Command method roster: execution and lifecycle | 3.1 |  | 0.617 |
| ns | 2514 |  | 142 | Command method roster: building and traversing the command tree | 3.2 |  | 0.598 |
| walker |  | 2582 | 298 | Code::CodeKey { rung: Names, file: completions.go, decl: 0, sub: 0, line: 0 } |  |  | 0.598 |
| walker |  | 2602 | 20 | Code::CodeKey { rung: Decl, file: completions.go, decl: 5, sub: 0, line: 47 } |  |  | 0.598 |
| walker |  | 2625 | 23 | Code::CodeKey { rung: Decl, file: completions.go, decl: 8, sub: 0, line: 98 } |  |  | 0.598 |
| walker |  | 2726 | 101 | Code::CodeKey { rung: Decl, file: completions.go, decl: 1, sub: 0, line: 28 } |  |  | 0.599 |
| ns | 2740 |  | 226 | Command method roster: the complete flag API | 3.3 |  | 0.573 |
| ns | 2955 |  | 215 | Command method roster: I/O wiring and the Set* customisation hooks | 3.4 |  | 0.550 |
| walker |  | 2967 | 241 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 0, line: 0 } |  |  | 0.564 |
| walker |  | 2978 | 11 | Code::CodeKey { rung: Decl, file: command.go, decl: 1, sub: 0, line: 33 } |  |  | 0.567 |
| walker |  | 2999 | 21 | Code::CodeKey { rung: Decl, file: command.go, decl: 3, sub: 0, line: 45 } |  |  | 0.572 |
| walker |  | 3007 | 8 | Code::CodeKey { rung: Body, file: command.go, decl: 5, sub: 0, line: 269 } |  |  | 0.572 |
| walker |  | 3016 | 9 | Code::CodeKey { rung: Body, file: command.go, decl: 6, sub: 0, line: 275 } |  |  | 0.572 |
| walker |  | 3025 | 9 | Code::CodeKey { rung: Body, file: command.go, decl: 7, sub: 0, line: 281 } |  |  | 0.572 |
| walker |  | 3036 | 11 | Code::CodeKey { rung: Doc, file: command.go, decl: 3, sub: 0, line: 45 } |  |  | 0.576 |
| walker |  | 3047 | 11 | Code::CodeKey { rung: Body, file: command.go, decl: 9, sub: 0, line: 296 } |  |  | 0.576 |
| walker |  | 3058 | 11 | Code::CodeKey { rung: Body, file: command.go, decl: 10, sub: 0, line: 302 } |  |  | 0.576 |
| walker |  | 3069 | 11 | Code::CodeKey { rung: Body, file: command.go, decl: 11, sub: 0, line: 308 } |  |  | 0.576 |
| ns | 3142 |  | 187 | Command method roster: help, usage, templates, default-command injection | 3.5 |  | 0.558 |
| walker |  | 3239 | 170 | Code::CodeKey { rung: Names, file: powershell_completions.go, decl: 0, sub: 0, line: 0 } |  |  | 0.558 |
| walker |  | 3253 | 14 | Code::CodeKey { rung: Body, file: powershell_completions.go, decl: 5, sub: 0, line: 337 } |  |  | 0.558 |
| walker |  | 3267 | 14 | Code::CodeKey { rung: Body, file: powershell_completions.go, decl: 7, sub: 0, line: 348 } |  |  | 0.558 |
| walker |  | 3282 | 15 | Code::CodeKey { rung: Body, file: powershell_completions.go, decl: 4, sub: 0, line: 331 } |  |  | 0.558 |
| walker |  | 3297 | 15 | Code::CodeKey { rung: Body, file: powershell_completions.go, decl: 6, sub: 0, line: 342 } |  |  | 0.558 |
| walker |  | 3315 | 18 | Code::CodeKey { rung: Doc, file: powershell_completions.go, decl: 4, sub: 0, line: 331 } |  |  | 0.558 |
| ns | 3332 |  | 190 | Command method roster: naming and introspection predicates | 3.6 |  | 0.542 |
| walker |  | 3364 | 49 | Code::CodeKey { rung: Decl, file: active_help.go, decl: 1, sub: 0, line: 22 } |  |  | 0.542 |
| walker |  | 3464 | 100 | Code::CodeKey { rung: Decl, file: cobra.go, decl: 1, sub: 0, line: 32 } |  |  | 0.542 |
| walker |  | 3496 | 32 | Code::CodeKey { rung: Doc, file: bash_completionsV2.go, decl: 4, sub: 0, line: 482 } |  |  | 0.542 |
| ns | 3516 |  | 184 | user_guide.md: complete H2 section map | 4.1 |  | 0.526 |
| walker |  | 3723 | 227 | Code::CodeKey { rung: Names, file: args.go, decl: 0, sub: 0, line: 0 } |  |  | 0.527 |
| walker |  | 3730 | 7 | Code::CodeKey { rung: Body, file: args.go, decl: 6, sub: 0, line: 82 } |  |  | 0.527 |
| walker |  | 3743 | 13 | Code::CodeKey { rung: Doc, file: args.go, decl: 6, sub: 0, line: 82 } |  |  | 0.527 |
| ns | 3754 |  | 238 | user_guide.md: complete H3/H4 subsection map | 4.2 | 4.1 | 0.508 |
| walker |  | 3759 | 16 | Code::CodeKey { rung: Doc, file: args.go, decl: 3, sub: 0, line: 42 } |  |  | 0.508 |
| walker |  | 3775 | 16 | Code::CodeKey { rung: Body, file: args.go, decl: 12, sub: 0, line: 142 } |  |  | 0.508 |
| walker |  | 3793 | 18 | Code::CodeKey { rung: Doc, file: args.go, decl: 9, sub: 0, line: 107 } |  |  | 0.509 |
| walker |  | 3827 | 34 | Code::CodeKey { rung: Doc, file: completions.go, decl: 4, sub: 0, line: 45 } |  |  | 0.509 |
| walker |  | 3921 | 94 | Markdown::HeadingsOutline { file: CONDUCT.md } |  |  | 0.509 |
| ns | 3950 |  | 196 | Completion and Active Help documentation: H1/H2 maps | 4.3 |  | 0.496 |
| ns | 4133 |  | 183 | Test harness: the helpers every root-package test is written against | 4.4 |  | 0.482 |
| walker |  | 4151 | 230 | Code::CodeKey { rung: Names, file: zsh_completions.go, decl: 0, sub: 0, line: 0 } |  |  | 0.483 |
| walker |  | 4158 | 7 | Code::CodeKey { rung: Body, file: zsh_completions.go, decl: 5, sub: 0, line: 55 } |  |  | 0.483 |
| walker |  | 4165 | 7 | Code::CodeKey { rung: Body, file: zsh_completions.go, decl: 6, sub: 0, line: 66 } |  |  | 0.483 |
| walker |  | 4179 | 14 | Code::CodeKey { rung: Body, file: zsh_completions.go, decl: 2, sub: 0, line: 31 } |  |  | 0.483 |
| walker |  | 4193 | 14 | Code::CodeKey { rung: Body, file: zsh_completions.go, decl: 4, sub: 0, line: 42 } |  |  | 0.483 |
| walker |  | 4208 | 15 | Code::CodeKey { rung: Body, file: zsh_completions.go, decl: 1, sub: 0, line: 25 } |  |  | 0.483 |
| walker |  | 4223 | 15 | Code::CodeKey { rung: Body, file: zsh_completions.go, decl: 3, sub: 0, line: 36 } |  |  | 0.483 |
| walker |  | 4241 | 18 | Code::CodeKey { rung: Doc, file: args.go, decl: 11, sub: 0, line: 127 } |  |  | 0.483 |
| walker |  | 4269 | 28 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.483 |
| ns | 4458 |  | 325 | CI: the four jobs and the platform / Go-version matrix | 4.5 |  | 0.464 |
| walker |  | 4523 | 254 | Code::CodeKey { rung: Names, file: shell_completions.go, decl: 0, sub: 0, line: 0 } |  |  | 0.465 |
| walker |  | 4536 | 13 | Code::CodeKey { rung: Body, file: shell_completions.go, decl: 1, sub: 0, line: 24 } |  |  | 0.465 |
| walker |  | 4550 | 14 | Code::CodeKey { rung: Body, file: shell_completions.go, decl: 2, sub: 0, line: 31 } |  |  | 0.465 |
| walker |  | 4564 | 14 | Code::CodeKey { rung: Body, file: shell_completions.go, decl: 9, sub: 0, line: 83 } |  |  | 0.465 |
| walker |  | 4579 | 15 | Code::CodeKey { rung: Body, file: shell_completions.go, decl: 4, sub: 0, line: 44 } |  |  | 0.465 |
| walker |  | 4594 | 15 | Code::CodeKey { rung: Body, file: shell_completions.go, decl: 5, sub: 0, line: 54 } |  |  | 0.465 |
| walker |  | 4609 | 15 | Code::CodeKey { rung: Body, file: shell_completions.go, decl: 10, sub: 0, line: 90 } |  |  | 0.465 |
| walker |  | 4629 | 20 | Code::CodeKey { rung: Doc, file: powershell_completions.go, decl: 6, sub: 0, line: 342 } |  |  | 0.465 |
| walker |  | 4645 | 16 | Code::CodeKey { rung: Doc, file: command.go, decl: 2, sub: 0, line: 42 } |  |  | 0.469 |
| walker |  | 4666 | 21 | Code::CodeKey { rung: Body, file: cobra.go, decl: 24, sub: 0, line: 243 } |  |  | 0.469 |
| ns | 4706 |  | 248 | golangci-lint configuration: the enabled linter set | 4.6 |  | 0.452 |
| walker |  | 4977 | 311 | Code::CodeKey { rung: Names, file: flag_groups.go, decl: 0, sub: 0, line: 0 } |  |  | 0.452 |
| walker |  | 4988 | 11 | Code::CodeKey { rung: Decl, file: flag_groups.go, decl: 1, sub: 0, line: 25 } |  |  | 0.452 |
| walker |  | 5021 | 33 | Code::CodeKey { rung: Doc, file: flag_groups.go, decl: 5, sub: 0, line: 81 } |  |  | 0.452 |
| ns | 5045 |  | 339 | Execute / ExecuteC: doc comments and the root-redirect rule | 5.1 | 3.1 | 0.437 |
| walker |  | 5062 | 41 | Code::CodeKey { rung: Doc, file: flag_groups.go, decl: 2, sub: 0, line: 33 } |  |  | 0.437 |
| walker |  | 5104 | 42 | Code::CodeKey { rung: Doc, file: flag_groups.go, decl: 3, sub: 0, line: 49 } |  |  | 0.437 |
| walker |  | 5149 | 45 | Code::CodeKey { rung: Body, file: fish_completions.go, decl: 2, sub: 0, line: 276 } |  |  | 0.437 |
| walker |  | 5165 | 16 | Code::CodeKey { rung: Body, file: shell_completions.go, decl: 6, sub: 0, line: 61 } |  |  | 0.437 |
| walker |  | 5298 | 133 | Code::CodeKey { rung: Names, file: doc/util.go, decl: 0, sub: 0, line: 0 } |  |  | 0.437 |
| walker |  | 5316 | 18 | Code::CodeKey { rung: Doc, file: zsh_completions.go, decl: 1, sub: 0, line: 25 } |  |  | 0.437 |
| walker |  | 5527 | 211 | Markdown::Prelude { file: README.md } |  |  | 0.440 |
| ns | 5583 |  | 538 | execute(): the ordered run pipeline | 5.2 | 5.1 | 0.414 |
| walker |  | 5669 | 142 | Code::CodeKey { rung: Names, file: doc/md_docs.go, decl: 0, sub: 0, line: 0 } |  |  | 0.414 |
| walker |  | 5680 | 11 | Code::CodeKey { rung: Doc, file: doc/md_docs.go, decl: 3, sub: 0, line: 52 } |  |  | 0.414 |
| walker |  | 5693 | 13 | Code::CodeKey { rung: Doc, file: doc/md_docs.go, decl: 4, sub: 0, line: 57 } |  |  | 0.414 |
| walker |  | 5714 | 21 | Code::CodeKey { rung: Body, file: doc/md_docs.go, decl: 3, sub: 0, line: 52 } |  |  | 0.414 |
| walker |  | 5733 | 19 | Code::CodeKey { rung: Doc, file: args.go, decl: 5, sub: 0, line: 69 } |  |  | 0.414 |
| ns | 5809 |  | 226 | cobra.go: the package-level behaviour switches | 5.3 |  | 0.416 |
| walker |  | 5878 | 145 | Code::CodeKey { rung: Names, file: doc/yaml_docs.go, decl: 0, sub: 0, line: 0 } |  |  | 0.417 |
| walker |  | 5889 | 11 | Code::CodeKey { rung: Doc, file: doc/yaml_docs.go, decl: 5, sub: 0, line: 88 } |  |  | 0.417 |
| ns | 5891 |  | 82 | cobra.go: package-level function roster | 5.4 |  | 0.426 |
| walker |  | 5902 | 13 | Code::CodeKey { rung: Doc, file: doc/yaml_docs.go, decl: 6, sub: 0, line: 93 } |  |  | 0.426 |
| walker |  | 5917 | 15 | Code::CodeKey { rung: Doc, file: doc/yaml_docs.go, decl: 4, sub: 0, line: 60 } |  |  | 0.426 |
| walker |  | 5938 | 21 | Code::CodeKey { rung: Body, file: doc/yaml_docs.go, decl: 5, sub: 0, line: 88 } |  |  | 0.426 |
| walker |  | 5954 | 16 | Code::CodeKey { rung: Body, file: active_help.go, decl: 4, sub: 0, line: 58 } |  |  | 0.426 |
| walker |  | 5997 | 43 | Code::CodeKey { rung: Doc, file: flag_groups.go, decl: 4, sub: 0, line: 65 } |  |  | 0.426 |
| walker |  | 6022 | 25 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 11, sub: 0, line: 85 } |  |  | 0.426 |
| walker |  | 6035 | 13 | Code::CodeKey { rung: Doc, file: completions.go, decl: 3, sub: 0, line: 41 } |  |  | 0.426 |
| ns | 6188 |  | 297 | The default usage template, plus the help and version template constants | 5.5 |  | 0.419 |
| walker |  | 6208 | 173 | Code::CodeKey { rung: Names, file: doc/rest_docs.go, decl: 0, sub: 0, line: 0 } |  |  | 0.419 |
| walker |  | 6222 | 14 | Code::CodeKey { rung: Doc, file: doc/rest_docs.go, decl: 3, sub: 0, line: 57 } |  |  | 0.419 |
| walker |  | 6238 | 16 | Code::CodeKey { rung: Doc, file: doc/rest_docs.go, decl: 4, sub: 0, line: 62 } |  |  | 0.419 |
| walker |  | 6255 | 17 | Code::CodeKey { rung: Body, file: doc/rest_docs.go, decl: 3, sub: 0, line: 57 } |  |  | 0.419 |
| walker |  | 6460 | 205 | Code::CodeKey { rung: Names, file: completions.go, decl: 0, sub: 1, line: 0 } |  |  | 0.419 |
| ns | 6635 |  | 447 | args.go: every PositionalArgs validator with its defining doc line | 6.1 |  | 0.426 |
| walker |  | 6648 | 188 | Code::CodeKey { rung: Decl, file: completions.go, decl: 9, sub: 0, line: 107 } |  |  | 0.427 |
| walker |  | 6661 | 13 | Code::CodeKey { rung: Doc, file: completions.go, decl: 9, sub: 0, line: 107 } |  |  | 0.427 |
| walker |  | 6674 | 13 | Code::CodeKey { rung: Doc, file: completions.go, decl: 12, sub: 0, line: 139 } |  |  | 0.427 |
| walker |  | 6739 | 65 | Code::CodeKey { rung: Body, file: active_help.go, decl: 3, sub: 0, line: 47 } |  |  | 0.427 |
| walker |  | 6804 | 65 | Code::CodeKey { rung: Body, file: fish_completions.go, decl: 3, sub: 0, line: 284 } |  |  | 0.427 |
| walker |  | 6821 | 17 | Code::CodeKey { rung: Body, file: shell_completions.go, decl: 7, sub: 0, line: 67 } |  |  | 0.427 |
| walker |  | 6841 | 20 | Code::CodeKey { rung: Body, file: command.go, decl: 8, sub: 0, line: 289 } |  |  | 0.427 |
| ns | 6898 |  | 263 | flag_groups.go: the three group markers and their annotation keys | 6.2 |  | 0.441 |
| ns | 7027 |  | 129 | shell_completions.go: the complete Mark* helper family | 6.3 |  | 0.450 |
| walker |  | 7088 | 247 | Code::CodeKey { rung: Names, file: bash_completions.go, decl: 0, sub: 1, line: 0 } |  |  | 0.451 |
| walker |  | 7103 | 15 | Code::CodeKey { rung: Doc, file: bash_completions.go, decl: 19, sub: 0, line: 701 } |  |  | 0.451 |
| walker |  | 7123 | 20 | Code::CodeKey { rung: Doc, file: bash_completions.go, decl: 17, sub: 0, line: 683 } |  |  | 0.451 |
| walker |  | 7191 | 68 | Code::CodeKey { rung: Body, file: bash_completionsV2.go, decl: 3, sub: 0, line: 470 } |  |  | 0.451 |
| walker |  | 7243 | 52 | Code::CodeKey { rung: Decl, file: doc/yaml_docs.go, decl: 1, sub: 0, line: 30 } |  |  | 0.451 |
| ns | 7363 |  | 336 | completions.go: the completion type vocabulary | 7.1 |  | 0.469 |
| walker |  | 7452 | 209 | Code::CodeKey { rung: Decl, file: completions.go, decl: 7, sub: 0, line: 56 } |  |  | 0.470 |
| walker |  | 7471 | 19 | Code::CodeKey { rung: Doc, file: args.go, decl: 8, sub: 0, line: 97 } |  |  | 0.472 |
| walker |  | 7491 | 20 | Code::CodeKey { rung: Doc, file: zsh_completions.go, decl: 3, sub: 0, line: 36 } |  |  | 0.472 |
| walker |  | 7524 | 33 | Code::CodeKey { rung: Doc, file: doc/md_docs.go, decl: 6, sub: 0, line: 133 } |  |  | 0.472 |
| ns | 7610 |  | 247 | The ShellCompDirective bit-mask values | 7.2 |  | 0.476 |
| walker |  | 7737 | 213 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 1, line: 0 } |  |  | 0.489 |
| walker |  | 7747 | 10 | Code::CodeKey { rung: Body, file: command.go, decl: 15, sub: 0, line: 333 } |  |  | 0.487 |
| ns | 7747 |  | 137 | completions.go: exported completion API roster | 7.3 |  | 0.487 |
| walker |  | 7757 | 10 | Code::CodeKey { rung: Body, file: command.go, decl: 16, sub: 0, line: 338 } |  |  | 0.487 |
| walker |  | 7767 | 10 | Code::CodeKey { rung: Body, file: command.go, decl: 21, sub: 0, line: 376 } |  |  | 0.487 |
| walker |  | 7802 | 35 | Code::CodeKey { rung: Doc, file: doc/rest_docs.go, decl: 6, sub: 0, line: 145 } |  |  | 0.487 |
| walker |  | 7827 | 25 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 24, sub: 0, line: 243 } |  |  | 0.487 |
| walker |  | 7857 | 30 | Code::CodeKey { rung: Doc, file: powershell_completions.go, decl: 5, sub: 0, line: 337 } |  |  | 0.487 |
| ns | 8040 |  | 293 | The __complete protocol and its environment configuration | 7.4 |  | 0.490 |
| walker |  | 8084 | 227 | Code::CodeKey { rung: Names, file: doc/man_docs.go, decl: 0, sub: 0, line: 0 } |  |  | 0.490 |
| walker |  | 8117 | 33 | Code::CodeKey { rung: Decl, file: doc/man_docs.go, decl: 3, sub: 0, line: 84 } |  |  | 0.490 |
| walker |  | 8175 | 58 | Code::CodeKey { rung: Decl, file: doc/man_docs.go, decl: 4, sub: 0, line: 94 } |  |  | 0.490 |
| ns | 8192 |  | 152 | The generated `completion` command tree | 7.5 | 7.3 | 0.487 |
| walker |  | 8207 | 32 | Code::CodeKey { rung: Doc, file: doc/man_docs.go, decl: 3, sub: 0, line: 84 } |  |  | 0.487 |
| walker |  | 8243 | 36 | Code::CodeKey { rung: Doc, file: doc/man_docs.go, decl: 2, sub: 0, line: 48 } |  |  | 0.487 |
| walker |  | 8254 | 11 | Code::CodeKey { rung: Body, file: command.go, decl: 12, sub: 0, line: 313 } |  |  | 0.487 |
| walker |  | 8273 | 19 | Code::CodeKey { rung: Body, file: shell_completions.go, decl: 8, sub: 0, line: 77 } |  |  | 0.487 |
| walker |  | 8293 | 20 | Code::CodeKey { rung: Doc, file: args.go, decl: 7, sub: 0, line: 87 } |  |  | 0.488 |
| walker |  | 8306 | 13 | Code::CodeKey { rung: Body, file: completions.go, decl: 13, sub: 0, line: 142 } |  |  | 0.488 |
| ns | 8419 |  | 227 | Shell script generators: every exported Gen*Completion entry point | 8.1 |  | 0.498 |
| walker |  | 8511 | 205 | Code::CodeKey { rung: Names, file: completions.go, decl: 0, sub: 2, line: 0 } |  |  | 0.503 |
| ns | 8515 |  | 96 | Legacy bash completion annotations | 8.2 |  | 0.506 |
| walker |  | 8543 | 32 | Code::CodeKey { rung: Decl, file: completions.go, decl: 20, sub: 0, line: 311 } |  |  | 0.506 |
| walker |  | 8554 | 11 | Code::CodeKey { rung: Body, file: command.go, decl: 14, sub: 0, line: 328 } |  |  | 0.506 |
| walker |  | 8582 | 28 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 12, sub: 0, line: 91 } |  |  | 0.506 |
| walker |  | 8679 | 97 | Markdown::HeadingsOutline { file: site/content/active_help.md } |  |  | 0.509 |
| ns | 8706 |  | 191 | active_help.go: the whole feature in one batch | 8.3 |  | 0.509 |
| walker |  | 8906 | 227 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 2, line: 0 } |  |  | 0.519 |
| ns | 8920 |  | 214 | The Windows mousetrap hook and its no-op counterpart | 8.4 | 5.3 | 0.514 |
| walker |  | 8938 | 32 | Code::CodeKey { rung: Doc, file: powershell_completions.go, decl: 7, sub: 0, line: 348 } |  |  | 0.514 |
| ns | 9098 |  | 178 | doc/: every exported generator across all four output formats | 9.1 |  | 0.522 |
| walker |  | 9167 | 229 | Code::CodeKey { rung: Decl, file: completions.go, decl: 7, sub: 1, line: 56 } |  |  | 0.528 |
| walker |  | 9184 | 17 | Code::CodeKey { rung: Body, file: bash_completions.go, decl: 18, sub: 0, line: 696 } |  |  | 0.528 |
| walker |  | 9204 | 20 | Code::CodeKey { rung: Body, file: shell_completions.go, decl: 11, sub: 0, line: 96 } |  |  | 0.528 |
| walker |  | 9235 | 31 | Markdown::HeadingsOutline { file: site/content/completions/bash.md } |  |  | 0.528 |
| walker |  | 9274 | 39 | Code::CodeKey { rung: Doc, file: doc/man_docs.go, decl: 5, sub: 0, line: 105 } |  |  | 0.528 |
| walker |  | 9296 | 22 | Code::CodeKey { rung: Doc, file: args.go, decl: 10, sub: 0, line: 117 } |  |  | 0.530 |
| ns | 9326 |  | 228 | doc/: GenManTreeOptions and GenManHeader, field by field | 9.2 |  | 0.529 |
| walker |  | 9334 | 38 | Code::CodeKey { rung: Body, file: doc/rest_docs.go, decl: 5, sub: 0, line: 138 } |  |  | 0.529 |
| walker |  | 9362 | 28 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 16, sub: 0, line: 144 } |  |  | 0.529 |
| walker |  | 9458 | 96 | Code::CodeKey { rung: Doc, file: active_help.go, decl: 3, sub: 0, line: 47 } |  |  | 0.532 |
| walker |  | 9470 | 12 | Code::CodeKey { rung: Doc, file: command.go, decl: 16, sub: 0, line: 338 } |  |  | 0.532 |
| walker |  | 9500 | 30 | Code::CodeKey { rung: Doc, file: zsh_completions.go, decl: 2, sub: 0, line: 31 } |  |  | 0.532 |
| ns | 9563 |  | 237 | doc/: the YAML document schema and the shared helpers | 9.3 |  | 0.528 |
| walker |  | 9621 | 121 | Code::CodeKey { rung: Decl, file: doc/yaml_docs.go, decl: 2, sub: 0, line: 37 } |  |  | 0.541 |
| ns | 9677 |  | 114 | docgen documentation pages: heading map | 9.4 |  | 0.538 |
| walker |  | 9776 | 155 | Markdown::Section { file: CONDUCT.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.538 |
| walker |  | 9913 | 137 | Markdown::Section { file: CONDUCT.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.538 |
| ns | 9929 |  | 252 | Project policy documents: section maps | 10.1 |  | 0.532 |
| walker |  | 9934 | 21 | Code::CodeKey { rung: Body, file: shell_completions.go, decl: 3, sub: 0, line: 38 } |  |  | 0.532 |
| ns | 9993 |  | 64 | Repository metadata: maintainers and local ignore rules | 10.2 |  | 0.529 |

Score(3000)=0.554 I=0.764 C=0.402 ns_rows≤3K=19/50 grid(1000/1442/2080/3000/4327/6240/9000)=0.596/0.790/0.705/0.554/0.464/0.436/0.492

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
| walker |  | 377 | 15 | Code::CodeKey { rung: Names, file: command_notwin.go, decl: 0, sub: 0, line: 0 } |  |  | 0.509 |
| walker |  | 390 | 13 | Fs::DirListing { dir: .github } |  |  | 0.515 |
| walker |  | 399 | 9 | Fs::DirListing { dir: .github/workflows } |  |  | 0.519 |
| walker |  | 495 | 96 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.764 |
| ns | 507 |  | 141 | README 'Overview': what the library does, first half | 1.4 |  | 0.666 |
| walker |  | 516 | 21 | Fs::DirListing { dir: site/content/docgen } |  |  | 0.670 |
| walker |  | 539 | 23 | Fs::DirListing { dir: site/content/completions } |  |  | 0.675 |
| walker |  | 602 | 63 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.675 |
| walker |  | 614 | 12 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.675 |
| ns | 724 |  | 217 | README: the remaining capabilities, and who builds on Cobra | 1.5 | 1.4 | 0.604 |
| ns | 926 |  | 202 | Canonical usage: the rootCmd literal and Execute() from the user guide | 1.6 |  | 0.520 |
| ns | 978 |  | 52 | Complete non-Go root listing: build, config, docs, governance | 1.7 |  | 0.577 |
| walker |  | 999 | 385 | Plaintext::Whole { file: Makefile } |  |  | 0.596 |
| walker |  | 1098 | 99 | GoMod::File { file: go.mod } |  |  | 0.734 |
| walker |  | 1134 | 36 | Code::CodeKey { rung: Names, file: command_win.go, decl: 0, sub: 0, line: 0 } |  |  | 0.734 |
| ns | 1218 |  | 240 | Makefile: every target and its recipe | 1.8 |  | 0.769 |
| walker |  | 1228 | 94 | Markdown::HeadingsOutline { file: CONDUCT.md } |  |  | 0.769 |
| walker |  | 1256 | 28 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.769 |
| ns | 1304 |  | 86 | doc/ subpackage, CI workflows, and assets listings | 1.9 |  | 0.782 |
| walker |  | 1331 | 75 | Code::CodeKey { rung: Names, file: fish_completions.go, decl: 0, sub: 0, line: 0 } |  |  | 0.782 |
| walker |  | 1345 | 14 | Code::CodeKey { rung: Doc, file: fish_completions.go, decl: 3, sub: 0, line: 284 } |  |  | 0.782 |
| walker |  | 1364 | 19 | Code::CodeKey { rung: Doc, file: fish_completions.go, decl: 2, sub: 0, line: 276 } |  |  | 0.782 |
| ns | 1374 |  | 70 | Documentation site tree: site/content and its two subdirectories | 1.10 |  | 0.790 |
| walker |  | 1409 | 45 | Code::CodeKey { rung: Body, file: fish_completions.go, decl: 2, sub: 0, line: 276 } |  |  | 0.790 |
| ns | 1612 |  | 238 | command.go: exported constants, FParseErrWhitelist, Group, Command's doc | 2.1 |  | 0.734 |
| walker |  | 1620 | 211 | Markdown::Prelude { file: README.md } |  |  | 0.739 |
| ns | 1684 |  | 72 | Command fields: naming and help text (Use .. Example) | 2.2 | 2.1 | 0.722 |
| ns | 1776 |  | 92 | Command fields: argument validation, completion, metadata | 2.3 | 2.1 | 0.703 |
| walker |  | 2041 | 421 | Code::CodeKey { rung: Names, file: cobra.go, decl: 0, sub: 0, line: 0 } |  |  | 0.705 |
| walker |  | 2067 | 26 | Code::CodeKey { rung: Decl, file: cobra.go, decl: 9, sub: 0, line: 72 } |  |  | 0.705 |
| walker |  | 2076 | 9 | Code::CodeKey { rung: Decl, file: cobra.go, decl: 4, sub: 0, line: 45 } |  |  | 0.705 |
| walker |  | 2088 | 12 | Code::CodeKey { rung: Body, file: cobra.go, decl: 11, sub: 0, line: 85 } |  |  | 0.705 |
| walker |  | 2101 | 13 | Code::CodeKey { rung: Body, file: cobra.go, decl: 13, sub: 0, line: 99 } |  |  | 0.705 |
| walker |  | 2120 | 19 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 7, sub: 0, line: 62 } |  |  | 0.705 |
| ns | 2125 |  | 349 | Command fields: the ten *Run hooks with the documented ordering comment | 2.4 | 2.1 | 0.662 |
| walker |  | 2145 | 25 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 11, sub: 0, line: 85 } |  |  | 0.662 |
| walker |  | 2170 | 25 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 24, sub: 0, line: 243 } |  |  | 0.662 |
| walker |  | 2198 | 28 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 12, sub: 0, line: 91 } |  |  | 0.662 |
| walker |  | 2226 | 28 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 16, sub: 0, line: 144 } |  |  | 0.662 |
| walker |  | 2255 | 29 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 13, sub: 0, line: 99 } |  |  | 0.662 |
| ns | 2264 |  | 139 | Command fields: behaviour toggles, with ellipses over the unexported state | 2.5 | 2.1 | 0.637 |
| walker |  | 2284 | 29 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 14, sub: 0, line: 105 } |  |  | 0.637 |
| walker |  | 2318 | 34 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 23, sub: 0, line: 235 } |  |  | 0.637 |
| walker |  | 2353 | 35 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 6, sub: 0, line: 59 } |  |  | 0.637 |
| ns | 2372 |  | 108 | Command method roster: execution and lifecycle | 3.1 |  | 0.621 |
| walker |  | 2396 | 43 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 8, sub: 0, line: 66 } |  |  | 0.621 |
| walker |  | 2442 | 46 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 5, sub: 0, line: 55 } |  |  | 0.622 |
| walker |  | 2458 | 16 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 19, sub: 0, line: 174 } |  |  | 0.622 |
| ns | 2514 |  | 142 | Command method roster: building and traversing the command tree | 3.2 |  | 0.602 |
| walker |  | 2558 | 100 | Code::CodeKey { rung: Decl, file: cobra.go, decl: 1, sub: 0, line: 32 } |  |  | 0.602 |
| walker |  | 2620 | 62 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 9, sub: 0, line: 72 } |  |  | 0.603 |
| walker |  | 2639 | 19 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 21, sub: 0, line: 192 } |  |  | 0.603 |
| walker |  | 2709 | 70 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 15, sub: 0, line: 114 } |  |  | 0.603 |
| ns | 2740 |  | 226 | Command method roster: the complete flag API | 3.3 |  | 0.577 |
| walker |  | 2795 | 86 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 10, sub: 0, line: 81 } |  |  | 0.577 |
| walker |  | 2825 | 30 | Code::CodeKey { rung: Doc, file: cobra.go, decl: 18, sub: 0, line: 166 } |  |  | 0.577 |
| walker |  | 2933 | 108 | Code::CodeKey { rung: Names, file: bash_completionsV2.go, decl: 0, sub: 0, line: 0 } |  |  | 0.577 |
| walker |  | 2948 | 15 | Code::CodeKey { rung: Body, file: bash_completionsV2.go, decl: 4, sub: 0, line: 482 } |  |  | 0.577 |
| ns | 2955 |  | 215 | Command method roster: I/O wiring and the Set* customisation hooks | 3.4 |  | 0.554 |
| walker |  | 2967 | 19 | Code::CodeKey { rung: Doc, file: bash_completionsV2.go, decl: 3, sub: 0, line: 470 } |  |  | 0.554 |
| walker |  | 2999 | 32 | Code::CodeKey { rung: Doc, file: bash_completionsV2.go, decl: 4, sub: 0, line: 482 } |  |  | 0.554 |
| walker |  | 3064 | 65 | Code::CodeKey { rung: Body, file: fish_completions.go, decl: 3, sub: 0, line: 284 } |  |  | 0.554 |
| ns | 3142 |  | 187 | Command method roster: help, usage, templates, default-command injection | 3.5 |  | 0.537 |
| walker |  | 3281 | 217 | Code::CodeKey { rung: Names, file: bash_completions.go, decl: 0, sub: 0, line: 0 } |  |  | 0.537 |
| walker |  | 3290 | 9 | Code::CodeKey { rung: Decl, file: bash_completions.go, decl: 1, sub: 0, line: 29 } |  |  | 0.537 |
| walker |  | 3301 | 11 | Code::CodeKey { rung: Doc, file: bash_completions.go, decl: 1, sub: 0, line: 29 } |  |  | 0.537 |
| ns | 3332 |  | 190 | Command method roster: naming and introspection predicates | 3.6 |  | 0.521 |
| walker |  | 3428 | 127 | Code::CodeKey { rung: Names, file: active_help.go, decl: 0, sub: 0, line: 0 } |  |  | 0.521 |
| walker |  | 3451 | 23 | Code::CodeKey { rung: Body, file: active_help.go, decl: 2, sub: 0, line: 38 } |  |  | 0.521 |
| walker |  | 3500 | 49 | Code::CodeKey { rung: Decl, file: active_help.go, decl: 1, sub: 0, line: 22 } |  |  | 0.521 |
| walker |  | 3516 | 16 | Code::CodeKey { rung: Body, file: active_help.go, decl: 4, sub: 0, line: 58 } |  |  | 0.507 |
| ns | 3516 |  | 184 | user_guide.md: complete H2 section map | 4.1 |  | 0.507 |
| walker |  | 3581 | 65 | Code::CodeKey { rung: Body, file: active_help.go, decl: 3, sub: 0, line: 47 } |  |  | 0.507 |
| walker |  | 3649 | 68 | Code::CodeKey { rung: Body, file: bash_completionsV2.go, decl: 3, sub: 0, line: 470 } |  |  | 0.507 |
| ns | 3754 |  | 238 | user_guide.md: complete H3/H4 subsection map | 4.2 | 4.1 | 0.489 |
| walker |  | 3947 | 298 | Code::CodeKey { rung: Names, file: completions.go, decl: 0, sub: 0, line: 0 } |  |  | 0.489 |
| ns | 3950 |  | 196 | Completion and Active Help documentation: H1/H2 maps | 4.3 |  | 0.477 |
| walker |  | 3967 | 20 | Code::CodeKey { rung: Decl, file: completions.go, decl: 5, sub: 0, line: 47 } |  |  | 0.477 |
| walker |  | 3990 | 23 | Code::CodeKey { rung: Decl, file: completions.go, decl: 8, sub: 0, line: 98 } |  |  | 0.477 |
| walker |  | 4091 | 101 | Code::CodeKey { rung: Decl, file: completions.go, decl: 1, sub: 0, line: 28 } |  |  | 0.477 |
| walker |  | 4125 | 34 | Code::CodeKey { rung: Doc, file: completions.go, decl: 4, sub: 0, line: 45 } |  |  | 0.477 |
| ns | 4133 |  | 183 | Test harness: the helpers every root-package test is written against | 4.4 |  | 0.464 |
| walker |  | 4138 | 13 | Code::CodeKey { rung: Doc, file: completions.go, decl: 3, sub: 0, line: 41 } |  |  | 0.464 |
| walker |  | 4347 | 209 | Code::CodeKey { rung: Decl, file: completions.go, decl: 7, sub: 0, line: 56 } |  |  | 0.465 |
| ns | 4458 |  | 325 | CI: the four jobs and the platform / Go-version matrix | 4.5 |  | 0.446 |
| walker |  | 4588 | 241 | Code::CodeKey { rung: Names, file: command.go, decl: 0, sub: 0, line: 0 } |  |  | 0.457 |
| walker |  | 4599 | 11 | Code::CodeKey { rung: Decl, file: command.go, decl: 1, sub: 0, line: 33 } |  |  | 0.460 |
| walker |  | 4620 | 21 | Code::CodeKey { rung: Decl, file: command.go, decl: 3, sub: 0, line: 45 } |  |  | 0.464 |
| walker |  | 4628 | 8 | Code::CodeKey { rung: Body, file: command.go, decl: 5, sub: 0, line: 269 } |  |  | 0.464 |
| walker |  | 4637 | 9 | Code::CodeKey { rung: Body, file: command.go, decl: 6, sub: 0, line: 275 } |  |  | 0.464 |
| walker |  | 4648 | 11 | Code::CodeKey { rung: Doc, file: command.go, decl: 3, sub: 0, line: 45 } |  |  | 0.467 |
| walker |  | 4664 | 16 | Code::CodeKey { rung: Doc, file: command.go, decl: 2, sub: 0, line: 42 } |  |  | 0.472 |
| walker |  | 4673 | 9 | Code::CodeKey { rung: Body, file: command.go, decl: 7, sub: 0, line: 281 } |  |  | 0.472 |
| walker |  | 4704 | 31 | Code::CodeKey { rung: Doc, file: command.go, decl: 9, sub: 0, line: 296 } |  |  | 0.472 |
| ns | 4706 |  | 248 | golangci-lint configuration: the enabled linter set | 4.6 |  | 0.454 |
| walker |  | 4735 | 31 | Code::CodeKey { rung: Doc, file: command.go, decl: 10, sub: 0, line: 302 } |  |  | 0.454 |
| walker |  | 4766 | 31 | Code::CodeKey { rung: Doc, file: command.go, decl: 11, sub: 0, line: 308 } |  |  | 0.454 |
| walker |  | 4800 | 34 | Code::CodeKey { rung: Doc, file: command.go, decl: 6, sub: 0, line: 275 } |  |  | 0.454 |
| walker |  | 4842 | 42 | Code::CodeKey { rung: Doc, file: command.go, decl: 7, sub: 0, line: 281 } |  |  | 0.454 |
| walker |  | 4895 | 53 | Code::CodeKey { rung: Doc, file: command.go, decl: 8, sub: 0, line: 289 } |  |  | 0.454 |
| walker |  | 5010 | 115 | Code::CodeKey { rung: Doc, file: command.go, decl: 5, sub: 0, line: 269 } |  |  | 0.454 |
| walker |  | 5021 | 11 | Code::CodeKey { rung: Body, file: command.go, decl: 9, sub: 0, line: 296 } |  |  | 0.454 |
| ns | 5045 |  | 339 | Execute / ExecuteC: doc comments and the root-redirect rule | 5.1 | 3.1 | 0.438 |
| walker |  | 5048 | 27 | Code::CodeKey { rung: Doc, file: completions.go, decl: 2, sub: 0, line: 38 } |  |  | 0.438 |
| walker |  | 5059 | 11 | Code::CodeKey { rung: Body, file: command.go, decl: 10, sub: 0, line: 302 } |  |  | 0.438 |
| walker |  | 5229 | 170 | Code::CodeKey { rung: Names, file: powershell_completions.go, decl: 0, sub: 0, line: 0 } |  |  | 0.439 |
| walker |  | 5243 | 14 | Code::CodeKey { rung: Body, file: powershell_completions.go, decl: 5, sub: 0, line: 337 } |  |  | 0.439 |
| walker |  | 5257 | 14 | Code::CodeKey { rung: Body, file: powershell_completions.go, decl: 7, sub: 0, line: 348 } |  |  | 0.439 |
| walker |  | 5272 | 15 | Code::CodeKey { rung: Body, file: powershell_completions.go, decl: 4, sub: 0, line: 331 } |  |  | 0.439 |
| walker |  | 5287 | 15 | Code::CodeKey { rung: Body, file: powershell_completions.go, decl: 6, sub: 0, line: 342 } |  |  | 0.439 |
| walker |  | 5305 | 18 | Code::CodeKey { rung: Doc, file: powershell_completions.go, decl: 4, sub: 0, line: 331 } |  |  | 0.439 |
| walker |  | 5325 | 20 | Code::CodeKey { rung: Doc, file: powershell_completions.go, decl: 6, sub: 0, line: 342 } |  |  | 0.439 |
| walker |  | 5355 | 30 | Code::CodeKey { rung: Doc, file: powershell_completions.go, decl: 5, sub: 0, line: 337 } |  |  | 0.439 |
| walker |  | 5387 | 32 | Code::CodeKey { rung: Doc, file: powershell_completions.go, decl: 7, sub: 0, line: 348 } |  |  | 0.439 |
| walker |  | 5483 | 96 | Code::CodeKey { rung: Doc, file: active_help.go, decl: 3, sub: 0, line: 47 } |  |  | 0.439 |
| walker |  | 5494 | 11 | Code::CodeKey { rung: Body, file: command.go, decl: 11, sub: 0, line: 308 } |  |  | 0.439 |
| ns | 5583 |  | 538 | execute(): the ordered run pipeline | 5.2 | 5.1 | 0.413 |
| walker |  | 5721 | 227 | Code::CodeKey { rung: Names, file: args.go, decl: 0, sub: 0, line: 0 } |  |  | 0.413 |
| walker |  | 5728 | 7 | Code::CodeKey { rung: Body, file: args.go, decl: 6, sub: 0, line: 82 } |  |  | 0.413 |
| walker |  | 5741 | 13 | Code::CodeKey { rung: Doc, file: args.go, decl: 6, sub: 0, line: 82 } |  |  | 0.413 |
| walker |  | 5757 | 16 | Code::CodeKey { rung: Doc, file: args.go, decl: 3, sub: 0, line: 42 } |  |  | 0.414 |
| walker |  | 5773 | 16 | Code::CodeKey { rung: Body, file: args.go, decl: 12, sub: 0, line: 142 } |  |  | 0.414 |
| walker |  | 5791 | 18 | Code::CodeKey { rung: Doc, file: args.go, decl: 9, sub: 0, line: 107 } |  |  | 0.414 |
| walker |  | 5809 | 18 | Code::CodeKey { rung: Doc, file: args.go, decl: 11, sub: 0, line: 127 } |  |  | 0.430 |
| ns | 5809 |  | 226 | cobra.go: the package-level behaviour switches | 5.3 |  | 0.430 |
| walker |  | 5828 | 19 | Code::CodeKey { rung: Doc, file: args.go, decl: 5, sub: 0, line: 69 } |  |  | 0.431 |
| walker |  | 5847 | 19 | Code::CodeKey { rung: Doc, file: args.go, decl: 8, sub: 0, line: 97 } |  |  | 0.431 |
| walker |  | 5867 | 20 | Code::CodeKey { rung: Doc, file: args.go, decl: 7, sub: 0, line: 87 } |  |  | 0.431 |
| walker |  | 5889 | 22 | Code::CodeKey { rung: Doc, file: args.go, decl: 10, sub: 0, line: 117 } |  |  | 0.431 |
| ns | 5891 |  | 82 | cobra.go: package-level function roster | 5.4 |  | 0.439 |
| walker |  | 5926 | 37 | Code::CodeKey { rung: Doc, file: args.go, decl: 4, sub: 0, line: 51 } |  |  | 0.440 |
| walker |  | 5997 | 71 | Code::CodeKey { rung: Doc, file: args.go, decl: 12, sub: 0, line: 142 } |  |  | 0.440 |
| walker |  | 6094 | 97 | Markdown::HeadingsOutline { file: site/content/active_help.md } |  |  | 0.443 |
| ns | 6188 |  | 297 | The default usage template, plus the help and version template constants | 5.5 |  | 0.436 |
| walker |  | 6323 | 229 | Code::CodeKey { rung: Decl, file: completions.go, decl: 7, sub: 1, line: 56 } |  |  | 0.437 |
| walker |  | 6553 | 230 | Code::CodeKey { rung: Names, file: zsh_completions.go, decl: 0, sub: 0, line: 0 } |  |  | 0.437 |
| walker |  | 6560 | 7 | Code::CodeKey { rung: Body, file: zsh_completions.go, decl: 5, sub: 0, line: 55 } |  |  | 0.437 |
| walker |  | 6567 | 7 | Code::CodeKey { rung: Body, file: zsh_completions.go, decl: 6, sub: 0, line: 66 } |  |  | 0.437 |
| walker |  | 6581 | 14 | Code::CodeKey { rung: Body, file: zsh_completions.go, decl: 2, sub: 0, line: 31 } |  |  | 0.437 |
| walker |  | 6595 | 14 | Code::CodeKey { rung: Body, file: zsh_completions.go, decl: 4, sub: 0, line: 42 } |  |  | 0.437 |
| walker |  | 6613 | 18 | Code::CodeKey { rung: Doc, file: zsh_completions.go, decl: 1, sub: 0, line: 25 } |  |  | 0.437 |
| walker |  | 6633 | 20 | Code::CodeKey { rung: Doc, file: zsh_completions.go, decl: 3, sub: 0, line: 36 } |  |  | 0.437 |
| ns | 6635 |  | 447 | args.go: every PositionalArgs validator with its defining doc line | 6.1 |  | 0.454 |
| walker |  | 6663 | 30 | Code::CodeKey { rung: Doc, file: zsh_completions.go, decl: 2, sub: 0, line: 31 } |  |  | 0.454 |
| walker |  | 6695 | 32 | Code::CodeKey { rung: Doc, file: zsh_completions.go, decl: 4, sub: 0, line: 42 } |  |  | 0.454 |
| walker |  | 6710 | 15 | Code::CodeKey { rung: Body, file: zsh_completions.go, decl: 1, sub: 0, line: 25 } |  |  | 0.454 |
| walker |  | 6806 | 96 | Code::CodeKey { rung: Doc, file: zsh_completions.go, decl: 6, sub: 0, line: 66 } |  |  | 0.454 |
| walker |  | 6821 | 15 | Code::CodeKey { rung: Body, file: zsh_completions.go, decl: 3, sub: 0, line: 36 } |  |  | 0.454 |
| walker |  | 6852 | 31 | Markdown::HeadingsOutline { file: site/content/completions/bash.md } |  |  | 0.454 |
| ns | 6898 |  | 263 | flag_groups.go: the three group markers and their annotation keys | 6.2 |  | 0.448 |
| walker |  | 6988 | 136 | Code::CodeKey { rung: Doc, file: active_help.go, decl: 2, sub: 0, line: 38 } |  |  | 0.449 |
| ns | 7027 |  | 129 | shell_completions.go: the complete Mark* helper family | 6.3 |  | 0.444 |
| walker |  | 7242 | 254 | Code::CodeKey { rung: Names, file: shell_completions.go, decl: 0, sub: 0, line: 0 } |  |  | 0.459 |
| walker |  | 7255 | 13 | Code::CodeKey { rung: Body, file: shell_completions.go, decl: 1, sub: 0, line: 24 } |  |  | 0.459 |
| walker |  | 7269 | 14 | Code::CodeKey { rung: Body, file: shell_completions.go, decl: 2, sub: 0, line: 31 } |  |  | 0.459 |
| walker |  | 7283 | 14 | Code::CodeKey { rung: Body, file: shell_completions.go, decl: 9, sub: 0, line: 83 } |  |  | 0.459 |
| walker |  | 7298 | 15 | Code::CodeKey { rung: Body, file: shell_completions.go, decl: 4, sub: 0, line: 44 } |  |  | 0.459 |
| walker |  | 7313 | 15 | Code::CodeKey { rung: Body, file: shell_completions.go, decl: 5, sub: 0, line: 54 } |  |  | 0.459 |
| walker |  | 7328 | 15 | Code::CodeKey { rung: Body, file: shell_completions.go, decl: 10, sub: 0, line: 90 } |  |  | 0.459 |
| walker |  | 7362 | 34 | Code::CodeKey { rung: Doc, file: shell_completions.go, decl: 9, sub: 0, line: 83 } |  |  | 0.459 |
| ns | 7363 |  | 336 | completions.go: the completion type vocabulary | 7.1 |  | 0.449 |
| walker |  | 7396 | 34 | Code::CodeKey { rung: Doc, file: shell_completions.go, decl: 11, sub: 0, line: 96 } |  |  | 0.449 |
| walker |  | 7431 | 35 | Code::CodeKey { rung: Doc, file: shell_completions.go, decl: 4, sub: 0, line: 44 } |  |  | 0.449 |
| walker |  | 7466 | 35 | Code::CodeKey { rung: Doc, file: shell_completions.go, decl: 7, sub: 0, line: 67 } |  |  | 0.449 |
| walker |  | 7508 | 42 | Code::CodeKey { rung: Doc, file: shell_completions.go, decl: 10, sub: 0, line: 90 } |  |  | 0.449 |
| walker |  | 7551 | 43 | Code::CodeKey { rung: Doc, file: shell_completions.go, decl: 6, sub: 0, line: 61 } |  |  | 0.449 |
| walker |  | 7600 | 49 | Code::CodeKey { rung: Doc, file: shell_completions.go, decl: 1, sub: 0, line: 24 } |  |  | 0.449 |
| ns | 7610 |  | 247 | The ShellCompDirective bit-mask values | 7.2 |  | 0.462 |
| walker |  | 7649 | 49 | Code::CodeKey { rung: Doc, file: shell_completions.go, decl: 3, sub: 0, line: 38 } |  |  | 0.462 |
| walker |  | 7700 | 51 | Code::CodeKey { rung: Doc, file: shell_completions.go, decl: 2, sub: 0, line: 31 } |  |  | 0.462 |
| ns | 7747 |  | 137 | completions.go: exported completion API roster | 7.3 |  | 0.457 |
| walker |  | 7797 | 97 | Code::CodeKey { rung: Doc, file: shell_completions.go, decl: 5, sub: 0, line: 54 } |  |  | 0.457 |
| walker |  | 7894 | 97 | Code::CodeKey { rung: Doc, file: shell_completions.go, decl: 8, sub: 0, line: 77 } |  |  | 0.457 |
| walker |  | 8037 | 143 | Code::CodeKey { rung: Doc, file: zsh_completions.go, decl: 5, sub: 0, line: 55 } |  |  | 0.457 |
| ns | 8040 |  | 293 | The __complete protocol and its environment configuration | 7.4 |  | 0.461 |
| ns | 8192 |  | 152 | The generated `completion` command tree | 7.5 | 7.3 | 0.458 |
| walker |  | 8348 | 311 | Code::CodeKey { rung: Names, file: flag_groups.go, decl: 0, sub: 0, line: 0 } |  |  | 0.463 |
| walker |  | 8359 | 11 | Code::CodeKey { rung: Decl, file: flag_groups.go, decl: 1, sub: 0, line: 25 } |  |  | 0.465 |
| walker |  | 8392 | 33 | Code::CodeKey { rung: Doc, file: flag_groups.go, decl: 5, sub: 0, line: 81 } |  |  | 0.467 |
| ns | 8419 |  | 227 | Shell script generators: every exported Gen*Completion entry point | 8.1 |  | 0.474 |
| walker |  | 8433 | 41 | Code::CodeKey { rung: Doc, file: flag_groups.go, decl: 2, sub: 0, line: 33 } |  |  | 0.476 |
| walker |  | 8475 | 42 | Code::CodeKey { rung: Doc, file: flag_groups.go, decl: 3, sub: 0, line: 49 } |  |  | 0.478 |
| ns | 8515 |  | 96 | Legacy bash completion annotations | 8.2 |  | 0.482 |
| walker |  | 8518 | 43 | Code::CodeKey { rung: Doc, file: flag_groups.go, decl: 4, sub: 0, line: 65 } |  |  | 0.485 |
| walker |  | 8673 | 155 | Markdown::Section { file: CONDUCT.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.485 |
| ns | 8706 |  | 191 | active_help.go: the whole feature in one batch | 8.3 |  | 0.492 |
| walker |  | 8810 | 137 | Markdown::Section { file: CONDUCT.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.492 |
| ns | 8920 |  | 214 | The Windows mousetrap hook and its no-op counterpart | 8.4 | 5.3 | 0.492 |
| walker |  | 9020 | 210 | Code::CodeKey { rung: Decl, file: command.go, decl: 4, sub: 0, line: 54 } |  |  | 0.492 |
| walker |  | 9087 | 67 | Code::CodeKey { rung: Doc, file: command.go, decl: 4, sub: 0, line: 54 } |  |  | 0.503 |
| ns | 9098 |  | 178 | doc/: every exported generator across all four output formats | 9.1 |  | 0.498 |
| walker |  | 9220 | 133 | Code::CodeKey { rung: Names, file: doc/util.go, decl: 0, sub: 0, line: 0 } |  |  | 0.498 |
| walker |  | 9263 | 43 | Markdown::HeadingsOutline { file: site/content/docgen/_index.md } |  |  | 0.498 |
| ns | 9326 |  | 228 | doc/: GenManTreeOptions and GenManHeader, field by field | 9.2 |  | 0.490 |
| walker |  | 9405 | 142 | Code::CodeKey { rung: Names, file: doc/md_docs.go, decl: 0, sub: 0, line: 0 } |  |  | 0.491 |
| walker |  | 9416 | 11 | Code::CodeKey { rung: Doc, file: doc/md_docs.go, decl: 3, sub: 0, line: 52 } |  |  | 0.491 |
| walker |  | 9429 | 13 | Code::CodeKey { rung: Doc, file: doc/md_docs.go, decl: 4, sub: 0, line: 57 } |  |  | 0.491 |
| walker |  | 9450 | 21 | Code::CodeKey { rung: Body, file: doc/md_docs.go, decl: 3, sub: 0, line: 52 } |  |  | 0.491 |
| walker |  | 9483 | 33 | Code::CodeKey { rung: Doc, file: doc/md_docs.go, decl: 6, sub: 0, line: 133 } |  |  | 0.491 |
| walker |  | 9534 | 51 | Code::CodeKey { rung: Body, file: doc/md_docs.go, decl: 5, sub: 0, line: 125 } |  |  | 0.491 |
| ns | 9563 |  | 237 | doc/: the YAML document schema and the shared helpers | 9.3 |  | 0.485 |
| ns | 9677 |  | 114 | docgen documentation pages: heading map | 9.4 |  | 0.483 |
| walker |  | 9679 | 145 | Code::CodeKey { rung: Names, file: doc/yaml_docs.go, decl: 0, sub: 0, line: 0 } |  |  | 0.487 |
| walker |  | 9690 | 11 | Code::CodeKey { rung: Doc, file: doc/yaml_docs.go, decl: 5, sub: 0, line: 88 } |  |  | 0.487 |
| walker |  | 9703 | 13 | Code::CodeKey { rung: Doc, file: doc/yaml_docs.go, decl: 6, sub: 0, line: 93 } |  |  | 0.487 |
| walker |  | 9718 | 15 | Code::CodeKey { rung: Doc, file: doc/yaml_docs.go, decl: 4, sub: 0, line: 60 } |  |  | 0.487 |
| walker |  | 9739 | 21 | Code::CodeKey { rung: Body, file: doc/yaml_docs.go, decl: 5, sub: 0, line: 88 } |  |  | 0.487 |
| walker |  | 9791 | 52 | Code::CodeKey { rung: Decl, file: doc/yaml_docs.go, decl: 1, sub: 0, line: 30 } |  |  | 0.490 |
| walker |  | 9912 | 121 | Code::CodeKey { rung: Decl, file: doc/yaml_docs.go, decl: 2, sub: 0, line: 37 } |  |  | 0.504 |
| ns | 9929 |  | 252 | Project policy documents: section maps | 10.1 |  | 0.498 |
| walker |  | 9935 | 23 | Markdown::Section { file: site/content/completions/bash.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.498 |
| ns | 9993 |  | 64 | Repository metadata: maintainers and local ignore rules | 10.2 |  | 0.495 |

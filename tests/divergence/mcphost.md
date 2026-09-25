Score(3000)=0.726 I=0.879 C=0.600 ns_rows≤3K=18/51 grid(1000/1442/2080/3000/4327/6240/9000)=0.891/0.864/0.807/0.726/0.609/0.532/0.516

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 40 | 40 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 55 | 15 | Fs::DirListing { dir: contribute } |  |  | 0.000 |
| walker |  | 63 | 8 | Fs::DirListing { dir: contribute/conf } |  |  | 0.000 |
| ns | 63 |  | 63 | README title and one-line identity | 1.1 |  | 0.000 |
| walker |  | 66 | 3 | Fs::DirListing { dir: .github } |  |  | 0.000 |
| walker |  | 74 | 8 | Fs::DirListing { dir: .github/workflows } |  |  | 0.000 |
| walker |  | 98 | 24 | Fs::DirListing { dir: sdk } |  |  | 0.000 |
| ns | 103 |  | 40 | Complete repository root listing | 1.2 |  | 0.633 |
| walker |  | 128 | 30 | Fs::DirListing { dir: internal } |  |  | 0.713 |
| ns | 133 |  | 30 | Complete internal/ package listing | 1.3 |  | 0.697 |
| walker |  | 136 | 8 | Fs::DirListing { dir: internal/session } |  |  | 0.697 |
| walker |  | 148 | 12 | Fs::DirListing { dir: internal/agent } |  |  | 0.697 |
| walker |  | 161 | 13 | Fs::DirListing { dir: internal/tokens } |  |  | 0.698 |
| walker |  | 197 | 36 | Fs::DirListing { dir: cmd } |  |  | 0.734 |
| ns | 207 |  | 74 | Complete cmd/ and sdk/ listings | 1.4 |  | 0.645 |
| walker |  | 250 | 53 | GoMod::Identity { file: go.mod } |  |  | 0.660 |
| ns | 260 |  | 53 | Module path, Go version and toolchain | 1.5 |  | 0.662 |
| walker |  | 267 | 17 | Fs::DirListing { dir: internal/auth } |  |  | 0.663 |
| walker |  | 273 | 6 | Fs::DirListing { dir: examples } |  |  | 0.665 |
| walker |  | 336 | 63 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.910 |
| walker |  | 360 | 24 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.910 |
| walker |  | 395 | 35 | Fs::DirListing { dir: internal/models } |  |  | 0.915 |
| walker |  | 400 | 5 | Fs::DirListing { dir: internal/models/anthropic } |  |  | 0.915 |
| walker |  | 405 | 5 | Fs::DirListing { dir: internal/models/gemini } |  |  | 0.916 |
| walker |  | 410 | 5 | Fs::DirListing { dir: internal/models/openai } |  |  | 0.917 |
| ns | 427 |  | 167 | All README section headings (locations only) | 1.6 |  | 0.785 |
| walker |  | 449 | 39 | Fs::DirListing { dir: internal/hooks } |  |  | 0.789 |
| walker |  | 459 | 10 | Fs::DirListing { dir: internal/hooks/testdata } |  |  | 0.790 |
| walker |  | 499 | 40 | Fs::DirListing { dir: internal/builtin } |  |  | 0.797 |
| ns | 534 |  | 107 | README feature list, first half | 1.7 | 1.6 | 0.741 |
| walker |  | 544 | 45 | Fs::DirListing { dir: internal/config } |  |  | 0.752 |
| walker |  | 577 | 33 | Markdown::HeadingsOutline { file: contribute/contribute.md } |  |  | 0.752 |
| ns | 648 |  | 114 | README feature list, second half | 1.8 |  | 0.704 |
| walker |  | 663 | 86 | Fs::DirListing { dir: internal/ui } |  |  | 0.720 |
| walker |  | 668 | 5 | Fs::DirListing { dir: internal/ui/progress } |  |  | 0.721 |
| ns | 781 |  | 133 | Complete listings for the config / agent / tools / models packages | 1.9 |  | 0.726 |
| walker |  | 897 | 229 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: true } |  |  | 0.829 |
| walker |  | 907 | 10 | Markdown::Section { file: contribute/contribute.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.829 |
| walker |  | 933 | 26 | Fs::DirListing { dir: internal/tools } |  |  | 0.887 |
| walker |  | 953 | 20 | Markdown::Section { file: contribute/contribute.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.887 |
| walker |  | 980 | 27 | Code::CodeKey { rung: Names, file: main.go, decl: 0, sub: 0, line: 0 } |  |  | 0.887 |
| ns | 999 |  | 218 | Complete listings for the builtin / hooks / session / auth / tokens / ui packages | 1.10 |  | 0.891 |
| ns | 1089 |  | 90 | Complete listings for examples/, contribute/ and .github/ | 1.11 |  | 0.848 |
| walker |  | 1110 | 130 | Code::CodeKey { rung: Decl, file: main.go, decl: 2, sub: 0, line: 14 } |  |  | 0.854 |
| walker |  | 1158 | 48 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.855 |
| walker |  | 1198 | 40 | Markdown::Section { file: README.md, section_index: 33, keeps_default_concavity: false } |  |  | 0.856 |
| ns | 1243 |  | 154 | main.go entry point | 2.1 |  | 0.859 |
| walker |  | 1331 | 133 | Markdown::HeadingsOutline { file: sdk/README.md } |  |  | 0.859 |
| walker |  | 1358 | 27 | Markdown::Section { file: sdk/README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.859 |
| walker |  | 1371 | 13 | Markdown::Section { file: sdk/README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.859 |
| walker |  | 1429 | 58 | Markdown::Section { file: README.md, section_index: 31, keeps_default_concavity: false } |  |  | 0.861 |
| walker |  | 1495 | 66 | Markdown::Section { file: README.md, section_index: 34, keeps_default_concavity: false } |  |  | 0.864 |
| ns | 1504 |  | 261 | Complete cobra command tree: script, auth (login/logout/status), hooks (list/validate/init) | 2.2 |  | 0.819 |
| walker |  | 1693 | 198 | Code::CodeKey { rung: ModuleDoc, file: internal/tokens/anthropic.go, decl: 0, sub: 0, line: 0 } |  |  | 0.819 |
| walker |  | 1699 | 6 | Fs::DirListing { dir: sdk/examples } |  |  | 0.835 |
| ns | 1754 |  | 250 | Persistent flag registration, part 1: config, system-prompt, model, debug, prompt, quiet | 2.3 |  | 0.803 |
| walker |  | 1770 | 71 | Code::CodeKey { rung: Names, file: cmd/hooks.go, decl: 0, sub: 0, line: 0 } |  |  | 0.803 |
| walker |  | 1817 | 47 | Code::CodeKey { rung: Decl, file: cmd/hooks.go, decl: 1, sub: 0, line: 16 } |  |  | 0.804 |
| walker |  | 2002 | 185 | Code::CodeKey { rung: Decl, file: cmd/hooks.go, decl: 3, sub: 0, line: 57 } |  |  | 0.805 |
| ns | 2098 |  | 344 | Persistent flag registration, part 2: no-exit, max-steps, stream, compact, no-hooks, approve-tool-run, session flags | 2.4 |  | 0.765 |
| ns | 2224 |  | 126 | Provider and TLS flag registration | 2.5 |  | 0.758 |
| walker |  | 2322 | 320 | Code::CodeKey { rung: Decl, file: cmd/hooks.go, decl: 2, sub: 0, line: 25 } |  |  | 0.760 |
| walker |  | 2373 | 51 | Code::CodeKey { rung: Doc, file: cmd/hooks.go, decl: 3, sub: 0, line: 57 } |  |  | 0.760 |
| walker |  | 2427 | 54 | Code::CodeKey { rung: Doc, file: cmd/hooks.go, decl: 2, sub: 0, line: 25 } |  |  | 0.760 |
| walker |  | 2486 | 59 | Code::CodeKey { rung: Doc, file: cmd/hooks.go, decl: 1, sub: 0, line: 16 } |  |  | 0.760 |
| ns | 2502 |  | 278 | Generation-parameter and Ollama flag registration, with the hidden flag | 2.6 |  | 0.741 |
| walker |  | 2526 | 40 | Plaintext::DeclSurface { file: contribute/build.sh } |  |  | 0.741 |
| walker |  | 2537 | 11 | Plaintext::Whole { file: contribute/build.sh } |  |  | 0.741 |
| walker |  | 2640 | 103 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.744 |
| ns | 2749 |  | 247 | MCPServerConfig: complete field set including the legacy block | 3.1 |  | 0.718 |
| walker |  | 2800 | 160 | Code::CodeKey { rung: Names, file: cmd/auth.go, decl: 0, sub: 0, line: 0 } |  |  | 0.718 |
| walker |  | 2919 | 119 | Code::CodeKey { rung: Decl, file: cmd/auth.go, decl: 4, sub: 0, line: 78 } |  |  | 0.721 |
| ns | 3069 |  | 320 | Config struct: application-level keys | 3.2 |  | 0.699 |
| walker |  | 3080 | 161 | Code::CodeKey { rung: Decl, file: cmd/auth.go, decl: 1, sub: 0, line: 17 } |  |  | 0.704 |
| walker |  | 3247 | 167 | Code::CodeKey { rung: Decl, file: cmd/auth.go, decl: 3, sub: 0, line: 58 } |  |  | 0.711 |
| ns | 3250 |  | 181 | Config struct: generation-parameter and TLS keys | 3.3 |  | 0.695 |
| walker |  | 3422 | 175 | Code::CodeKey { rung: Decl, file: cmd/auth.go, decl: 2, sub: 0, line: 37 } |  |  | 0.703 |
| walker |  | 3473 | 51 | Code::CodeKey { rung: Doc, file: cmd/auth.go, decl: 3, sub: 0, line: 58 } |  |  | 0.703 |
| ns | 3514 |  | 264 | GetTransportType: type-to-transport mapping and legacy inference | 3.4 |  | 0.667 |
| walker |  | 3525 | 52 | Code::CodeKey { rung: Doc, file: cmd/auth.go, decl: 2, sub: 0, line: 37 } |  |  | 0.667 |
| walker |  | 3579 | 54 | Code::CodeKey { rung: Doc, file: cmd/auth.go, decl: 4, sub: 0, line: 78 } |  |  | 0.667 |
| walker |  | 3643 | 64 | Code::CodeKey { rung: Doc, file: cmd/auth.go, decl: 1, sub: 0, line: 17 } |  |  | 0.667 |
| walker |  | 3708 | 65 | Code::CodeKey { rung: Names, file: sdk/types.go, decl: 0, sub: 0, line: 0 } |  |  | 0.667 |
| walker |  | 3719 | 11 | Code::CodeKey { rung: Body, file: sdk/types.go, decl: 3, sub: 0, line: 18 } |  |  | 0.667 |
| walker |  | 3731 | 12 | Code::CodeKey { rung: Body, file: sdk/types.go, decl: 4, sub: 0, line: 24 } |  |  | 0.667 |
| walker |  | 3763 | 32 | Code::CodeKey { rung: Doc, file: sdk/types.go, decl: 3, sub: 0, line: 18 } |  |  | 0.667 |
| walker |  | 3796 | 33 | Code::CodeKey { rung: Doc, file: sdk/types.go, decl: 1, sub: 0, line: 10 } |  |  | 0.667 |
| walker |  | 3830 | 34 | Code::CodeKey { rung: Doc, file: sdk/types.go, decl: 2, sub: 0, line: 14 } |  |  | 0.667 |
| walker |  | 3864 | 34 | Code::CodeKey { rung: Doc, file: sdk/types.go, decl: 4, sub: 0, line: 24 } |  |  | 0.667 |
| ns | 3938 |  | 424 | Config.Validate: required fields per transport and filter exclusivity | 3.5 |  | 0.634 |
| ns | 4141 |  | 203 | Substitution engine: the two regexes plus every symbol in substitution.go | 3.6 |  | 0.622 |
| ns | 4295 |  | 154 | Remaining top-level symbols of internal/config/config.go and all of merger.go (locations) | 3.7 |  | 0.609 |
| walker |  | 4378 | 514 | GoMod::File { file: go.mod } |  |  | 0.609 |
| walker |  | 4382 | 4 | Fs::DirListing { dir: sdk/examples/basic } |  |  | 0.615 |
| walker |  | 4386 | 4 | Fs::DirListing { dir: sdk/examples/scripting } |  |  | 0.622 |
| ns | 4541 |  | 246 | Agent struct and its seven callback handler types | 4.1 |  | 0.606 |
| walker |  | 4599 | 213 | Code::CodeKey { rung: Names, file: sdk/mcphost.go, decl: 0, sub: 0, line: 0 } |  |  | 0.607 |
| walker |  | 4634 | 35 | Code::CodeKey { rung: Decl, file: sdk/mcphost.go, decl: 1, sub: 0, line: 19 } |  |  | 0.607 |
| walker |  | 4643 | 9 | Code::CodeKey { rung: Body, file: sdk/mcphost.go, decl: 6, sub: 0, line: 201 } |  |  | 0.607 |
| walker |  | 4652 | 9 | Code::CodeKey { rung: Body, file: sdk/mcphost.go, decl: 10, sub: 0, line: 230 } |  |  | 0.607 |
| walker |  | 4661 | 9 | Code::CodeKey { rung: Body, file: sdk/mcphost.go, decl: 11, sub: 0, line: 237 } |  |  | 0.607 |
| walker |  | 4729 | 68 | Code::CodeKey { rung: Decl, file: sdk/mcphost.go, decl: 5, sub: 0, line: 163 } |  |  | 0.607 |
| ns | 4815 |  | 274 | Every top-level symbol of internal/agent/agent.go (locations) | 4.2 |  | 0.591 |
| walker |  | 4836 | 107 | Code::CodeKey { rung: Decl, file: sdk/mcphost.go, decl: 2, sub: 0, line: 28 } |  |  | 0.592 |
| walker |  | 4865 | 29 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 6, sub: 0, line: 201 } |  |  | 0.592 |
| walker |  | 4894 | 29 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 9, sub: 0, line: 224 } |  |  | 0.592 |
| walker |  | 4931 | 37 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 8, sub: 0, line: 218 } |  |  | 0.592 |
| walker |  | 4970 | 39 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 7, sub: 0, line: 207 } |  |  | 0.592 |
| ns | 5007 |  | 192 | The tool-calling loop: step bound, tool-call branch and the approval gate | 4.3 | 4.2 | 0.582 |
| walker |  | 5017 | 47 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 2, sub: 0, line: 28 } |  |  | 0.582 |
| walker |  | 5065 | 48 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 11, sub: 0, line: 237 } |  |  | 0.582 |
| walker |  | 5114 | 49 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 10, sub: 0, line: 230 } |  |  | 0.582 |
| walker |  | 5166 | 52 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 1, sub: 0, line: 19 } |  |  | 0.582 |
| ns | 5186 |  | 179 | agent factory: AgentCreationOptions fields and both functions | 4.4 |  | 0.572 |
| walker |  | 5222 | 56 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 5, sub: 0, line: 163 } |  |  | 0.572 |
| walker |  | 5279 | 57 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 3, sub: 0, line: 40 } |  |  | 0.572 |
| walker |  | 5337 | 58 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 4, sub: 0, line: 130 } |  |  | 0.572 |
| ns | 5549 |  | 363 | Every top-level symbol of internal/tools/mcp.go (locations) | 4.5 |  | 0.556 |
| walker |  | 5617 | 280 | Code::CodeKey { rung: Names, file: cmd/script.go, decl: 0, sub: 0, line: 0 } |  |  | 0.557 |
| walker |  | 5682 | 65 | Code::CodeKey { rung: Decl, file: cmd/script.go, decl: 9, sub: 0, line: 423 } |  |  | 0.557 |
| walker |  | 5693 | 11 | Code::CodeKey { rung: Body, file: cmd/script.go, decl: 2, sub: 0, line: 78 } |  |  | 0.557 |
| walker |  | 5709 | 16 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 4, sub: 0, line: 148 } |  |  | 0.557 |
| walker |  | 5725 | 16 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 7, sub: 0, line: 279 } |  |  | 0.557 |
| walker |  | 5743 | 18 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 14, sub: 0, line: 511 } |  |  | 0.557 |
| walker |  | 5762 | 19 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 8, sub: 0, line: 288 } |  |  | 0.557 |
| ns | 5765 |  | 216 | AgenticLoopConfig head and every mode-driving function in cmd/root.go (locations) | 4.6 |  | 0.546 |
| walker |  | 5782 | 20 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 6, sub: 0, line: 255 } |  |  | 0.546 |
| walker |  | 5813 | 31 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 12, sub: 0, line: 480 } |  |  | 0.546 |
| walker |  | 5846 | 33 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 10, sub: 0, line: 431 } |  |  | 0.546 |
| walker |  | 5882 | 36 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 11, sub: 0, line: 442 } |  |  | 0.546 |
| ns | 5907 |  | 142 | CreateProvider: the complete list of supported providers | 5.1 |  | 0.538 |
| walker |  | 5926 | 44 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 13, sub: 0, line: 499 } |  |  | 0.538 |
| walker |  | 5973 | 47 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 3, sub: 0, line: 84 } |  |  | 0.538 |
| walker |  | 6026 | 53 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 9, sub: 0, line: 423 } |  |  | 0.538 |
| walker |  | 6059 | 33 | Markdown::Section { file: sdk/README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.538 |
| walker |  | 6151 | 92 | Markdown::Section { file: README.md, section_index: 32, keeps_default_concavity: false } |  |  | 0.540 |
| ns | 6235 |  | 328 | Every top-level symbol of internal/models/providers.go (locations) | 5.2 | 5.1 | 0.525 |
| ns | 6385 |  | 150 | ModelsRegistry: model validation and suggestion API | 5.3 |  | 0.519 |
| ns | 6477 |  | 92 | The generated model catalogue: generator types and the DO-NOT-EDIT header | 5.4 |  | 0.515 |
| ns | 6653 |  | 176 | Builtin server registry: the complete set of in-process servers | 6.1 |  | 0.506 |
| walker |  | 6781 | 630 | Code::CodeKey { rung: Decl, file: cmd/script.go, decl: 1, sub: 0, line: 28 } |  |  | 0.514 |
| ns | 6825 |  | 172 | Every top-level symbol of internal/builtin/registry.go (locations) | 6.2 | 6.1 | 0.508 |
| walker |  | 6842 | 61 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 1, sub: 0, line: 28 } |  |  | 0.508 |
| walker |  | 6903 | 61 | Code::CodeKey { rung: Names, file: internal/models/models_data.go, decl: 0, sub: 0, line: 0 } |  |  | 0.508 |
| walker |  | 6923 | 20 | Code::CodeKey { rung: Decl, file: internal/models/models_data.go, decl: 3, sub: 0, line: 26 } |  |  | 0.508 |
| walker |  | 6969 | 46 | Code::CodeKey { rung: Decl, file: internal/models/models_data.go, decl: 2, sub: 0, line: 18 } |  |  | 0.508 |
| walker |  | 7021 | 52 | Code::CodeKey { rung: Decl, file: internal/models/models_data.go, decl: 4, sub: 0, line: 32 } |  |  | 0.508 |
| ns | 7042 |  | 217 | Bash builtin: output/timeout limits and the complete banned-command list | 6.3 |  | 0.496 |
| walker |  | 7087 | 66 | Code::CodeKey { rung: Decl, file: internal/models/models_data.go, decl: 1, sub: 0, line: 7 } |  |  | 0.496 |
| walker |  | 7099 | 12 | Code::CodeKey { rung: Doc, file: internal/models/models_data.go, decl: 2, sub: 0, line: 18 } |  |  | 0.496 |
| walker |  | 7111 | 12 | Code::CodeKey { rung: Doc, file: internal/models/models_data.go, decl: 4, sub: 0, line: 32 } |  |  | 0.496 |
| walker |  | 7125 | 14 | Code::CodeKey { rung: Doc, file: internal/models/models_data.go, decl: 1, sub: 0, line: 7 } |  |  | 0.496 |
| walker |  | 7139 | 14 | Code::CodeKey { rung: Doc, file: internal/models/models_data.go, decl: 3, sub: 0, line: 26 } |  |  | 0.496 |
| walker |  | 7154 | 15 | Code::CodeKey { rung: Doc, file: internal/models/models_data.go, decl: 5, sub: 0, line: 41 } |  |  | 0.496 |
| walker |  | 7228 | 74 | Code::CodeKey { rung: Names, file: internal/hooks/schemas.go, decl: 0, sub: 0, line: 0 } |  |  | 0.496 |
| ns | 7240 |  | 198 | The http builtin: its four tools and every symbol in http.go (locations) | 6.4 |  | 0.488 |
| walker |  | 7252 | 24 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 4, sub: 0, line: 42 } |  |  | 0.488 |
| walker |  | 7296 | 44 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 2, sub: 0, line: 23 } |  |  | 0.489 |
| ns | 7352 |  | 112 | HookEvent: the complete set of hook events | 7.1 |  | 0.485 |
| walker |  | 7357 | 61 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 3, sub: 0, line: 32 } |  |  | 0.485 |
| walker |  | 7446 | 89 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 6, sub: 0, line: 62 } |  |  | 0.485 |
| walker |  | 7553 | 107 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 5, sub: 0, line: 50 } |  |  | 0.486 |
| ns | 7567 |  | 215 | Hook configuration schema: HookConfig, HookMatcher, HookEntry | 7.2 |  | 0.479 |
| walker |  | 7712 | 159 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 1, sub: 0, line: 10 } |  |  | 0.479 |
| walker |  | 7763 | 51 | Code::CodeKey { rung: Doc, file: internal/hooks/schemas.go, decl: 2, sub: 0, line: 23 } |  |  | 0.479 |
| walker |  | 7816 | 53 | Code::CodeKey { rung: Doc, file: internal/hooks/schemas.go, decl: 1, sub: 0, line: 10 } |  |  | 0.479 |
| ns | 7839 |  | 272 | Hook wire protocol: CommonInput and HookOutput | 7.3 |  | 0.493 |
| walker |  | 7869 | 53 | Code::CodeKey { rung: Doc, file: internal/hooks/schemas.go, decl: 4, sub: 0, line: 42 } |  |  | 0.493 |
| walker |  | 7924 | 55 | Code::CodeKey { rung: Doc, file: internal/hooks/schemas.go, decl: 5, sub: 0, line: 50 } |  |  | 0.493 |
| walker |  | 7986 | 62 | Plaintext::DeclSurface { file: contribute/boost.sh } |  |  | 0.493 |
| ns | 8030 |  | 191 | Per-event hook input structs | 7.4 |  | 0.503 |
| walker |  | 8062 | 76 | Code::CodeKey { rung: Names, file: internal/models/generate_models.go, decl: 0, sub: 0, line: 0 } |  |  | 0.506 |
| walker |  | 8119 | 57 | Code::CodeKey { rung: Decl, file: internal/models/generate_models.go, decl: 3, sub: 0, line: 50 } |  |  | 0.506 |
| walker |  | 8250 | 131 | Code::CodeKey { rung: Decl, file: internal/models/generate_models.go, decl: 2, sub: 0, line: 37 } |  |  | 0.506 |
| ns | 8336 |  | 306 | Hook executor and validator symbol rosters | 7.5 |  | 0.496 |
| walker |  | 8392 | 142 | Code::CodeKey { rung: Decl, file: internal/models/generate_models.go, decl: 4, sub: 0, line: 60 } |  |  | 0.496 |
| walker |  | 8423 | 31 | Code::CodeKey { rung: Doc, file: internal/models/generate_models.go, decl: 2, sub: 0, line: 37 } |  |  | 0.496 |
| walker |  | 8455 | 32 | Code::CodeKey { rung: Doc, file: internal/models/generate_models.go, decl: 3, sub: 0, line: 50 } |  |  | 0.496 |
| ns | 8493 |  | 157 | README: hooks.yml file locations and the --no-hooks escape hatch | 7.6 | 1.6 | 0.493 |
| walker |  | 8654 | 199 | Code::CodeKey { rung: Decl, file: internal/models/generate_models.go, decl: 1, sub: 0, line: 18 } |  |  | 0.493 |
| ns | 8682 |  | 189 | Script mode: a worked frontmatter example and the variable rules | 8.1 |  | 0.503 |
| walker |  | 8695 | 41 | Code::CodeKey { rung: Doc, file: internal/models/generate_models.go, decl: 4, sub: 0, line: 60 } |  |  | 0.503 |
| walker |  | 8743 | 48 | Code::CodeKey { rung: Doc, file: internal/models/generate_models.go, decl: 1, sub: 0, line: 18 } |  |  | 0.503 |
| walker |  | 8804 | 61 | Code::CodeKey { rung: Doc, file: internal/hooks/schemas.go, decl: 3, sub: 0, line: 32 } |  |  | 0.503 |
| ns | 8813 |  | 131 | Every top-level symbol of cmd/script.go (locations) | 8.2 |  | 0.512 |
| walker |  | 8832 | 28 | Code::CodeKey { rung: Names, file: internal/tokens/init.go, decl: 0, sub: 0, line: 0 } |  |  | 0.512 |
| walker |  | 8847 | 15 | Code::CodeKey { rung: Body, file: internal/tokens/init.go, decl: 1, sub: 0, line: 21 } |  |  | 0.512 |
| walker |  | 8862 | 15 | Code::CodeKey { rung: Body, file: internal/tokens/init.go, decl: 2, sub: 0, line: 50 } |  |  | 0.512 |
| walker |  | 8950 | 88 | Code::CodeKey { rung: Names, file: internal/hooks/config.go, decl: 0, sub: 0, line: 0 } |  |  | 0.513 |
| walker |  | 8977 | 27 | Code::CodeKey { rung: Decl, file: internal/hooks/config.go, decl: 1, sub: 0, line: 14 } |  |  | 0.515 |
| walker |  | 9032 | 55 | Code::CodeKey { rung: Decl, file: internal/hooks/config.go, decl: 3, sub: 0, line: 30 } |  |  | 0.521 |
| walker |  | 9097 | 65 | Code::CodeKey { rung: Decl, file: internal/hooks/config.go, decl: 2, sub: 0, line: 21 } |  |  | 0.530 |
| walker |  | 9111 | 14 | Code::CodeKey { rung: Doc, file: internal/hooks/config.go, decl: 6, sub: 0, line: 113 } |  |  | 0.521 |
| ns | 9111 |  | 298 | Session file format: Session, Metadata, Message and ToolCall fields | 8.3 |  | 0.521 |
| walker |  | 9130 | 19 | Code::CodeKey { rung: Doc, file: internal/hooks/config.go, decl: 5, sub: 0, line: 97 } |  |  | 0.521 |
| walker |  | 9159 | 29 | Code::CodeKey { rung: Doc, file: internal/hooks/config.go, decl: 1, sub: 0, line: 14 } |  |  | 0.521 |
| walker |  | 9215 | 56 | Code::CodeKey { rung: Doc, file: internal/hooks/config.go, decl: 3, sub: 0, line: 30 } |  |  | 0.521 |
| walker |  | 9275 | 60 | Code::CodeKey { rung: Doc, file: internal/hooks/config.go, decl: 2, sub: 0, line: 21 } |  |  | 0.521 |
| walker |  | 9305 | 30 | Code::CodeKey { rung: Names, file: internal/auth/browser.go, decl: 0, sub: 0, line: 0 } |  |  | 0.521 |
| walker |  | 9336 | 31 | Code::CodeKey { rung: Body, file: internal/auth/browser.go, decl: 2, sub: 0, line: 34 } |  |  | 0.521 |
| walker |  | 9396 | 60 | Code::CodeKey { rung: Doc, file: internal/auth/browser.go, decl: 2, sub: 0, line: 34 } |  |  | 0.521 |
| ns | 9405 |  | 294 | The public SDK surface: Options and every exported symbol | 8.4 |  | 0.533 |
| walker |  | 9456 | 60 | Code::CodeKey { rung: Names, file: internal/ui/commands.go, decl: 0, sub: 0, line: 0 } |  |  | 0.533 |
| walker |  | 9507 | 51 | Code::CodeKey { rung: Decl, file: internal/ui/commands.go, decl: 1, sub: 0, line: 6 } |  |  | 0.533 |
| walker |  | 9555 | 48 | Code::CodeKey { rung: Doc, file: internal/ui/commands.go, decl: 1, sub: 0, line: 6 } |  |  | 0.533 |
| ns | 9607 |  | 202 | The complete slash-command table: names and descriptions | 8.5 |  | 0.527 |
| walker |  | 9610 | 55 | Code::CodeKey { rung: Doc, file: internal/ui/commands.go, decl: 3, sub: 0, line: 65 } |  |  | 0.527 |
| walker |  | 9668 | 58 | Code::CodeKey { rung: Doc, file: internal/ui/commands.go, decl: 4, sub: 0, line: 83 } |  |  | 0.527 |
| ns | 9828 |  | 221 | ui.SetupCLI: the AgentInterface contract and CLISetupOptions | 8.6 |  | 0.520 |
| walker |  | 9880 | 212 | Code::CodeKey { rung: Names, file: internal/config/config.go, decl: 0, sub: 0, line: 0 } |  |  | 0.528 |
| walker |  | 9922 | 42 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 3, sub: 0, line: 109 } |  |  | 0.528 |
| walker |  | 9931 | 9 | Code::CodeKey { rung: Body, file: internal/config/config.go, decl: 14, sub: 0, line: 459 } |  |  | 0.528 |
| walker |  | 9954 | 23 | Code::CodeKey { rung: Doc, file: internal/config/config.go, decl: 11, sub: 0, line: 296 } |  |  | 0.528 |
| ns | 9984 |  | 156 | Release packaging: the goreleaser build matrix | 9.1 |  | 0.522 |
| walker |  | 9991 | 37 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 5, sub: 0, line: 137 } |  |  | 0.522 |

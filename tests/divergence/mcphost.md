Score(3000)=0.726 I=0.879 C=0.600 ns_rows≤3K=18/51 grid(1000/1442/2080/3000/4327/6240/9000)=0.892/0.864/0.807/0.726/0.609/0.532/0.516

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
| walker |  | 1028 | 48 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.892 |
| walker |  | 1068 | 40 | Markdown::Section { file: README.md, section_index: 33, keeps_default_concavity: false } |  |  | 0.893 |
| ns | 1089 |  | 90 | Complete listings for examples/, contribute/ and .github/ | 1.11 |  | 0.850 |
| walker |  | 1201 | 133 | Markdown::HeadingsOutline { file: sdk/README.md } |  |  | 0.850 |
| walker |  | 1228 | 27 | Markdown::Section { file: sdk/README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.850 |
| walker |  | 1241 | 13 | Markdown::Section { file: sdk/README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.850 |
| ns | 1243 |  | 154 | main.go entry point | 2.1 |  | 0.814 |
| walker |  | 1299 | 58 | Markdown::Section { file: README.md, section_index: 31, keeps_default_concavity: false } |  |  | 0.816 |
| walker |  | 1426 | 127 | Code::CodeKey { rung: Body, file: main.go, decl: 2, sub: 0, line: 14 } |  |  | 0.861 |
| walker |  | 1492 | 66 | Markdown::Section { file: README.md, section_index: 34, keeps_default_concavity: false } |  |  | 0.864 |
| ns | 1504 |  | 261 | Complete cobra command tree: script, auth (login/logout/status), hooks (list/validate/init) | 2.2 |  | 0.819 |
| walker |  | 1690 | 198 | Code::CodeKey { rung: ModuleDoc, file: internal/tokens/anthropic.go, decl: 0, sub: 0, line: 0 } |  |  | 0.819 |
| walker |  | 1696 | 6 | Fs::DirListing { dir: sdk/examples } |  |  | 0.835 |
| ns | 1754 |  | 250 | Persistent flag registration, part 1: config, system-prompt, model, debug, prompt, quiet | 2.3 |  | 0.803 |
| walker |  | 1767 | 71 | Code::CodeKey { rung: Names, file: cmd/hooks.go, decl: 0, sub: 0, line: 0 } |  |  | 0.803 |
| walker |  | 1814 | 47 | Code::CodeKey { rung: Decl, file: cmd/hooks.go, decl: 1, sub: 0, line: 16 } |  |  | 0.804 |
| walker |  | 1999 | 185 | Code::CodeKey { rung: Decl, file: cmd/hooks.go, decl: 3, sub: 0, line: 57 } |  |  | 0.805 |
| ns | 2098 |  | 344 | Persistent flag registration, part 2: no-exit, max-steps, stream, compact, no-hooks, approve-tool-run, session flags | 2.4 |  | 0.765 |
| ns | 2224 |  | 126 | Provider and TLS flag registration | 2.5 |  | 0.758 |
| walker |  | 2319 | 320 | Code::CodeKey { rung: Decl, file: cmd/hooks.go, decl: 2, sub: 0, line: 25 } |  |  | 0.760 |
| walker |  | 2370 | 51 | Code::CodeKey { rung: Doc, file: cmd/hooks.go, decl: 3, sub: 0, line: 57 } |  |  | 0.760 |
| walker |  | 2424 | 54 | Code::CodeKey { rung: Doc, file: cmd/hooks.go, decl: 2, sub: 0, line: 25 } |  |  | 0.760 |
| walker |  | 2483 | 59 | Code::CodeKey { rung: Doc, file: cmd/hooks.go, decl: 1, sub: 0, line: 16 } |  |  | 0.760 |
| ns | 2502 |  | 278 | Generation-parameter and Ollama flag registration, with the hidden flag | 2.6 |  | 0.741 |
| walker |  | 2523 | 40 | Plaintext::DeclSurface { file: contribute/build.sh } |  |  | 0.741 |
| walker |  | 2534 | 11 | Plaintext::Whole { file: contribute/build.sh } |  |  | 0.741 |
| walker |  | 2637 | 103 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.744 |
| ns | 2749 |  | 247 | MCPServerConfig: complete field set including the legacy block | 3.1 |  | 0.718 |
| walker |  | 2797 | 160 | Code::CodeKey { rung: Names, file: cmd/auth.go, decl: 0, sub: 0, line: 0 } |  |  | 0.718 |
| walker |  | 2916 | 119 | Code::CodeKey { rung: Decl, file: cmd/auth.go, decl: 4, sub: 0, line: 78 } |  |  | 0.721 |
| ns | 3069 |  | 320 | Config struct: application-level keys | 3.2 |  | 0.699 |
| walker |  | 3077 | 161 | Code::CodeKey { rung: Decl, file: cmd/auth.go, decl: 1, sub: 0, line: 17 } |  |  | 0.704 |
| walker |  | 3244 | 167 | Code::CodeKey { rung: Decl, file: cmd/auth.go, decl: 3, sub: 0, line: 58 } |  |  | 0.711 |
| ns | 3250 |  | 181 | Config struct: generation-parameter and TLS keys | 3.3 |  | 0.695 |
| walker |  | 3419 | 175 | Code::CodeKey { rung: Decl, file: cmd/auth.go, decl: 2, sub: 0, line: 37 } |  |  | 0.703 |
| walker |  | 3470 | 51 | Code::CodeKey { rung: Doc, file: cmd/auth.go, decl: 3, sub: 0, line: 58 } |  |  | 0.703 |
| ns | 3514 |  | 264 | GetTransportType: type-to-transport mapping and legacy inference | 3.4 |  | 0.667 |
| walker |  | 3522 | 52 | Code::CodeKey { rung: Doc, file: cmd/auth.go, decl: 2, sub: 0, line: 37 } |  |  | 0.667 |
| walker |  | 3576 | 54 | Code::CodeKey { rung: Doc, file: cmd/auth.go, decl: 4, sub: 0, line: 78 } |  |  | 0.667 |
| walker |  | 3640 | 64 | Code::CodeKey { rung: Doc, file: cmd/auth.go, decl: 1, sub: 0, line: 17 } |  |  | 0.667 |
| walker |  | 3705 | 65 | Code::CodeKey { rung: Names, file: sdk/types.go, decl: 0, sub: 0, line: 0 } |  |  | 0.667 |
| walker |  | 3716 | 11 | Code::CodeKey { rung: Body, file: sdk/types.go, decl: 3, sub: 0, line: 18 } |  |  | 0.667 |
| walker |  | 3728 | 12 | Code::CodeKey { rung: Body, file: sdk/types.go, decl: 4, sub: 0, line: 24 } |  |  | 0.667 |
| walker |  | 3760 | 32 | Code::CodeKey { rung: Doc, file: sdk/types.go, decl: 3, sub: 0, line: 18 } |  |  | 0.667 |
| walker |  | 3793 | 33 | Code::CodeKey { rung: Doc, file: sdk/types.go, decl: 1, sub: 0, line: 10 } |  |  | 0.667 |
| walker |  | 3827 | 34 | Code::CodeKey { rung: Doc, file: sdk/types.go, decl: 2, sub: 0, line: 14 } |  |  | 0.667 |
| walker |  | 3861 | 34 | Code::CodeKey { rung: Doc, file: sdk/types.go, decl: 4, sub: 0, line: 24 } |  |  | 0.667 |
| ns | 3938 |  | 424 | Config.Validate: required fields per transport and filter exclusivity | 3.5 |  | 0.634 |
| ns | 4141 |  | 203 | Substitution engine: the two regexes plus every symbol in substitution.go | 3.6 |  | 0.622 |
| ns | 4295 |  | 154 | Remaining top-level symbols of internal/config/config.go and all of merger.go (locations) | 3.7 |  | 0.609 |
| walker |  | 4375 | 514 | GoMod::File { file: go.mod } |  |  | 0.609 |
| walker |  | 4379 | 4 | Fs::DirListing { dir: sdk/examples/basic } |  |  | 0.615 |
| walker |  | 4383 | 4 | Fs::DirListing { dir: sdk/examples/scripting } |  |  | 0.622 |
| ns | 4541 |  | 246 | Agent struct and its seven callback handler types | 4.1 |  | 0.606 |
| walker |  | 4596 | 213 | Code::CodeKey { rung: Names, file: sdk/mcphost.go, decl: 0, sub: 0, line: 0 } |  |  | 0.607 |
| walker |  | 4631 | 35 | Code::CodeKey { rung: Decl, file: sdk/mcphost.go, decl: 1, sub: 0, line: 19 } |  |  | 0.607 |
| walker |  | 4640 | 9 | Code::CodeKey { rung: Body, file: sdk/mcphost.go, decl: 6, sub: 0, line: 201 } |  |  | 0.607 |
| walker |  | 4649 | 9 | Code::CodeKey { rung: Body, file: sdk/mcphost.go, decl: 10, sub: 0, line: 230 } |  |  | 0.607 |
| walker |  | 4658 | 9 | Code::CodeKey { rung: Body, file: sdk/mcphost.go, decl: 11, sub: 0, line: 237 } |  |  | 0.607 |
| walker |  | 4726 | 68 | Code::CodeKey { rung: Decl, file: sdk/mcphost.go, decl: 5, sub: 0, line: 163 } |  |  | 0.607 |
| ns | 4815 |  | 274 | Every top-level symbol of internal/agent/agent.go (locations) | 4.2 |  | 0.591 |
| walker |  | 4833 | 107 | Code::CodeKey { rung: Decl, file: sdk/mcphost.go, decl: 2, sub: 0, line: 28 } |  |  | 0.592 |
| walker |  | 4862 | 29 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 6, sub: 0, line: 201 } |  |  | 0.592 |
| walker |  | 4891 | 29 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 9, sub: 0, line: 224 } |  |  | 0.592 |
| walker |  | 4928 | 37 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 8, sub: 0, line: 218 } |  |  | 0.592 |
| walker |  | 4967 | 39 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 7, sub: 0, line: 207 } |  |  | 0.592 |
| ns | 5007 |  | 192 | The tool-calling loop: step bound, tool-call branch and the approval gate | 4.3 | 4.2 | 0.582 |
| walker |  | 5014 | 47 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 2, sub: 0, line: 28 } |  |  | 0.582 |
| walker |  | 5062 | 48 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 11, sub: 0, line: 237 } |  |  | 0.582 |
| walker |  | 5111 | 49 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 10, sub: 0, line: 230 } |  |  | 0.582 |
| walker |  | 5163 | 52 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 1, sub: 0, line: 19 } |  |  | 0.582 |
| ns | 5186 |  | 179 | agent factory: AgentCreationOptions fields and both functions | 4.4 |  | 0.572 |
| walker |  | 5219 | 56 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 5, sub: 0, line: 163 } |  |  | 0.572 |
| walker |  | 5276 | 57 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 3, sub: 0, line: 40 } |  |  | 0.572 |
| walker |  | 5334 | 58 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 4, sub: 0, line: 130 } |  |  | 0.572 |
| ns | 5549 |  | 363 | Every top-level symbol of internal/tools/mcp.go (locations) | 4.5 |  | 0.556 |
| walker |  | 5614 | 280 | Code::CodeKey { rung: Names, file: cmd/script.go, decl: 0, sub: 0, line: 0 } |  |  | 0.557 |
| walker |  | 5679 | 65 | Code::CodeKey { rung: Decl, file: cmd/script.go, decl: 9, sub: 0, line: 423 } |  |  | 0.557 |
| walker |  | 5690 | 11 | Code::CodeKey { rung: Body, file: cmd/script.go, decl: 2, sub: 0, line: 78 } |  |  | 0.557 |
| walker |  | 5706 | 16 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 4, sub: 0, line: 148 } |  |  | 0.557 |
| walker |  | 5722 | 16 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 7, sub: 0, line: 279 } |  |  | 0.557 |
| walker |  | 5740 | 18 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 14, sub: 0, line: 511 } |  |  | 0.557 |
| walker |  | 5759 | 19 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 8, sub: 0, line: 288 } |  |  | 0.557 |
| ns | 5765 |  | 216 | AgenticLoopConfig head and every mode-driving function in cmd/root.go (locations) | 4.6 |  | 0.546 |
| walker |  | 5779 | 20 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 6, sub: 0, line: 255 } |  |  | 0.546 |
| walker |  | 5810 | 31 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 12, sub: 0, line: 480 } |  |  | 0.546 |
| walker |  | 5843 | 33 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 10, sub: 0, line: 431 } |  |  | 0.546 |
| walker |  | 5879 | 36 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 11, sub: 0, line: 442 } |  |  | 0.546 |
| ns | 5907 |  | 142 | CreateProvider: the complete list of supported providers | 5.1 |  | 0.538 |
| walker |  | 5923 | 44 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 13, sub: 0, line: 499 } |  |  | 0.538 |
| walker |  | 5970 | 47 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 3, sub: 0, line: 84 } |  |  | 0.538 |
| walker |  | 6023 | 53 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 9, sub: 0, line: 423 } |  |  | 0.538 |
| walker |  | 6056 | 33 | Markdown::Section { file: sdk/README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.538 |
| walker |  | 6148 | 92 | Markdown::Section { file: README.md, section_index: 32, keeps_default_concavity: false } |  |  | 0.540 |
| ns | 6235 |  | 328 | Every top-level symbol of internal/models/providers.go (locations) | 5.2 | 5.1 | 0.525 |
| ns | 6385 |  | 150 | ModelsRegistry: model validation and suggestion API | 5.3 |  | 0.519 |
| ns | 6477 |  | 92 | The generated model catalogue: generator types and the DO-NOT-EDIT header | 5.4 |  | 0.515 |
| ns | 6653 |  | 176 | Builtin server registry: the complete set of in-process servers | 6.1 |  | 0.506 |
| walker |  | 6778 | 630 | Code::CodeKey { rung: Decl, file: cmd/script.go, decl: 1, sub: 0, line: 28 } |  |  | 0.514 |
| ns | 6825 |  | 172 | Every top-level symbol of internal/builtin/registry.go (locations) | 6.2 | 6.1 | 0.508 |
| walker |  | 6839 | 61 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 1, sub: 0, line: 28 } |  |  | 0.508 |
| walker |  | 6900 | 61 | Code::CodeKey { rung: Names, file: internal/models/models_data.go, decl: 0, sub: 0, line: 0 } |  |  | 0.508 |
| walker |  | 6920 | 20 | Code::CodeKey { rung: Decl, file: internal/models/models_data.go, decl: 3, sub: 0, line: 26 } |  |  | 0.508 |
| walker |  | 6966 | 46 | Code::CodeKey { rung: Decl, file: internal/models/models_data.go, decl: 2, sub: 0, line: 18 } |  |  | 0.508 |
| walker |  | 7018 | 52 | Code::CodeKey { rung: Decl, file: internal/models/models_data.go, decl: 4, sub: 0, line: 32 } |  |  | 0.508 |
| ns | 7042 |  | 217 | Bash builtin: output/timeout limits and the complete banned-command list | 6.3 |  | 0.496 |
| walker |  | 7084 | 66 | Code::CodeKey { rung: Decl, file: internal/models/models_data.go, decl: 1, sub: 0, line: 7 } |  |  | 0.496 |
| walker |  | 7096 | 12 | Code::CodeKey { rung: Doc, file: internal/models/models_data.go, decl: 2, sub: 0, line: 18 } |  |  | 0.496 |
| walker |  | 7108 | 12 | Code::CodeKey { rung: Doc, file: internal/models/models_data.go, decl: 4, sub: 0, line: 32 } |  |  | 0.496 |
| walker |  | 7122 | 14 | Code::CodeKey { rung: Doc, file: internal/models/models_data.go, decl: 1, sub: 0, line: 7 } |  |  | 0.496 |
| walker |  | 7136 | 14 | Code::CodeKey { rung: Doc, file: internal/models/models_data.go, decl: 3, sub: 0, line: 26 } |  |  | 0.496 |
| walker |  | 7151 | 15 | Code::CodeKey { rung: Doc, file: internal/models/models_data.go, decl: 5, sub: 0, line: 41 } |  |  | 0.496 |
| walker |  | 7225 | 74 | Code::CodeKey { rung: Names, file: internal/hooks/schemas.go, decl: 0, sub: 0, line: 0 } |  |  | 0.496 |
| ns | 7240 |  | 198 | The http builtin: its four tools and every symbol in http.go (locations) | 6.4 |  | 0.488 |
| walker |  | 7249 | 24 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 4, sub: 0, line: 42 } |  |  | 0.488 |
| walker |  | 7293 | 44 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 2, sub: 0, line: 23 } |  |  | 0.489 |
| ns | 7352 |  | 112 | HookEvent: the complete set of hook events | 7.1 |  | 0.485 |
| walker |  | 7354 | 61 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 3, sub: 0, line: 32 } |  |  | 0.485 |
| walker |  | 7443 | 89 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 6, sub: 0, line: 62 } |  |  | 0.485 |
| walker |  | 7550 | 107 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 5, sub: 0, line: 50 } |  |  | 0.486 |
| ns | 7567 |  | 215 | Hook configuration schema: HookConfig, HookMatcher, HookEntry | 7.2 |  | 0.479 |
| walker |  | 7709 | 159 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 1, sub: 0, line: 10 } |  |  | 0.479 |
| walker |  | 7760 | 51 | Code::CodeKey { rung: Doc, file: internal/hooks/schemas.go, decl: 2, sub: 0, line: 23 } |  |  | 0.479 |
| walker |  | 7813 | 53 | Code::CodeKey { rung: Doc, file: internal/hooks/schemas.go, decl: 1, sub: 0, line: 10 } |  |  | 0.479 |
| ns | 7839 |  | 272 | Hook wire protocol: CommonInput and HookOutput | 7.3 |  | 0.493 |
| walker |  | 7866 | 53 | Code::CodeKey { rung: Doc, file: internal/hooks/schemas.go, decl: 4, sub: 0, line: 42 } |  |  | 0.493 |
| walker |  | 7921 | 55 | Code::CodeKey { rung: Doc, file: internal/hooks/schemas.go, decl: 5, sub: 0, line: 50 } |  |  | 0.493 |
| walker |  | 7983 | 62 | Plaintext::DeclSurface { file: contribute/boost.sh } |  |  | 0.493 |
| ns | 8030 |  | 191 | Per-event hook input structs | 7.4 |  | 0.503 |
| walker |  | 8059 | 76 | Code::CodeKey { rung: Names, file: internal/models/generate_models.go, decl: 0, sub: 0, line: 0 } |  |  | 0.506 |
| walker |  | 8116 | 57 | Code::CodeKey { rung: Decl, file: internal/models/generate_models.go, decl: 3, sub: 0, line: 50 } |  |  | 0.506 |
| walker |  | 8247 | 131 | Code::CodeKey { rung: Decl, file: internal/models/generate_models.go, decl: 2, sub: 0, line: 37 } |  |  | 0.506 |
| ns | 8336 |  | 306 | Hook executor and validator symbol rosters | 7.5 |  | 0.496 |
| walker |  | 8389 | 142 | Code::CodeKey { rung: Decl, file: internal/models/generate_models.go, decl: 4, sub: 0, line: 60 } |  |  | 0.496 |
| walker |  | 8420 | 31 | Code::CodeKey { rung: Doc, file: internal/models/generate_models.go, decl: 2, sub: 0, line: 37 } |  |  | 0.496 |
| walker |  | 8452 | 32 | Code::CodeKey { rung: Doc, file: internal/models/generate_models.go, decl: 3, sub: 0, line: 50 } |  |  | 0.496 |
| ns | 8493 |  | 157 | README: hooks.yml file locations and the --no-hooks escape hatch | 7.6 | 1.6 | 0.493 |
| walker |  | 8651 | 199 | Code::CodeKey { rung: Decl, file: internal/models/generate_models.go, decl: 1, sub: 0, line: 18 } |  |  | 0.493 |
| ns | 8682 |  | 189 | Script mode: a worked frontmatter example and the variable rules | 8.1 |  | 0.503 |
| walker |  | 8692 | 41 | Code::CodeKey { rung: Doc, file: internal/models/generate_models.go, decl: 4, sub: 0, line: 60 } |  |  | 0.503 |
| walker |  | 8740 | 48 | Code::CodeKey { rung: Doc, file: internal/models/generate_models.go, decl: 1, sub: 0, line: 18 } |  |  | 0.503 |
| walker |  | 8801 | 61 | Code::CodeKey { rung: Doc, file: internal/hooks/schemas.go, decl: 3, sub: 0, line: 32 } |  |  | 0.503 |
| ns | 8813 |  | 131 | Every top-level symbol of cmd/script.go (locations) | 8.2 |  | 0.512 |
| walker |  | 8829 | 28 | Code::CodeKey { rung: Names, file: internal/tokens/init.go, decl: 0, sub: 0, line: 0 } |  |  | 0.512 |
| walker |  | 8844 | 15 | Code::CodeKey { rung: Body, file: internal/tokens/init.go, decl: 1, sub: 0, line: 21 } |  |  | 0.512 |
| walker |  | 8859 | 15 | Code::CodeKey { rung: Body, file: internal/tokens/init.go, decl: 2, sub: 0, line: 50 } |  |  | 0.512 |
| walker |  | 8947 | 88 | Code::CodeKey { rung: Names, file: internal/hooks/config.go, decl: 0, sub: 0, line: 0 } |  |  | 0.513 |
| walker |  | 8974 | 27 | Code::CodeKey { rung: Decl, file: internal/hooks/config.go, decl: 1, sub: 0, line: 14 } |  |  | 0.515 |
| walker |  | 9029 | 55 | Code::CodeKey { rung: Decl, file: internal/hooks/config.go, decl: 3, sub: 0, line: 30 } |  |  | 0.521 |
| walker |  | 9094 | 65 | Code::CodeKey { rung: Decl, file: internal/hooks/config.go, decl: 2, sub: 0, line: 21 } |  |  | 0.530 |
| walker |  | 9108 | 14 | Code::CodeKey { rung: Doc, file: internal/hooks/config.go, decl: 6, sub: 0, line: 113 } |  |  | 0.530 |
| ns | 9111 |  | 298 | Session file format: Session, Metadata, Message and ToolCall fields | 8.3 |  | 0.521 |
| walker |  | 9127 | 19 | Code::CodeKey { rung: Doc, file: internal/hooks/config.go, decl: 5, sub: 0, line: 97 } |  |  | 0.521 |
| walker |  | 9156 | 29 | Code::CodeKey { rung: Doc, file: internal/hooks/config.go, decl: 1, sub: 0, line: 14 } |  |  | 0.521 |
| walker |  | 9212 | 56 | Code::CodeKey { rung: Doc, file: internal/hooks/config.go, decl: 3, sub: 0, line: 30 } |  |  | 0.521 |
| walker |  | 9272 | 60 | Code::CodeKey { rung: Doc, file: internal/hooks/config.go, decl: 2, sub: 0, line: 21 } |  |  | 0.521 |
| walker |  | 9302 | 30 | Code::CodeKey { rung: Names, file: internal/auth/browser.go, decl: 0, sub: 0, line: 0 } |  |  | 0.521 |
| walker |  | 9333 | 31 | Code::CodeKey { rung: Body, file: internal/auth/browser.go, decl: 2, sub: 0, line: 34 } |  |  | 0.521 |
| walker |  | 9393 | 60 | Code::CodeKey { rung: Doc, file: internal/auth/browser.go, decl: 2, sub: 0, line: 34 } |  |  | 0.521 |
| ns | 9405 |  | 294 | The public SDK surface: Options and every exported symbol | 8.4 |  | 0.533 |
| walker |  | 9453 | 60 | Code::CodeKey { rung: Names, file: internal/ui/commands.go, decl: 0, sub: 0, line: 0 } |  |  | 0.533 |
| walker |  | 9504 | 51 | Code::CodeKey { rung: Decl, file: internal/ui/commands.go, decl: 1, sub: 0, line: 6 } |  |  | 0.533 |
| walker |  | 9552 | 48 | Code::CodeKey { rung: Doc, file: internal/ui/commands.go, decl: 1, sub: 0, line: 6 } |  |  | 0.533 |
| walker |  | 9607 | 55 | Code::CodeKey { rung: Doc, file: internal/ui/commands.go, decl: 3, sub: 0, line: 65 } |  |  | 0.527 |
| ns | 9607 |  | 202 | The complete slash-command table: names and descriptions | 8.5 |  | 0.527 |
| walker |  | 9665 | 58 | Code::CodeKey { rung: Doc, file: internal/ui/commands.go, decl: 4, sub: 0, line: 83 } |  |  | 0.527 |
| ns | 9828 |  | 221 | ui.SetupCLI: the AgentInterface contract and CLISetupOptions | 8.6 |  | 0.520 |
| walker |  | 9877 | 212 | Code::CodeKey { rung: Names, file: internal/config/config.go, decl: 0, sub: 0, line: 0 } |  |  | 0.528 |
| walker |  | 9919 | 42 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 3, sub: 0, line: 109 } |  |  | 0.528 |
| walker |  | 9928 | 9 | Code::CodeKey { rung: Body, file: internal/config/config.go, decl: 14, sub: 0, line: 459 } |  |  | 0.528 |
| walker |  | 9951 | 23 | Code::CodeKey { rung: Doc, file: internal/config/config.go, decl: 11, sub: 0, line: 296 } |  |  | 0.528 |
| ns | 9984 |  | 156 | Release packaging: the goreleaser build matrix | 9.1 |  | 0.522 |
| walker |  | 9988 | 37 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 5, sub: 0, line: 137 } |  |  | 0.522 |

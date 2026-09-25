Score(3000)=0.787 I=0.910 C=0.681 ns_rows≤3K=18/51 grid(1000/1442/2080/3000/4327/6240/9000)=0.938/0.942/0.864/0.787/0.650/0.553/0.547

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
| walker |  | 505 | 6 | Fs::DirListing { dir: sdk/examples } |  |  | 0.836 |
| walker |  | 509 | 4 | Fs::DirListing { dir: sdk/examples/basic } |  |  | 0.857 |
| walker |  | 513 | 4 | Fs::DirListing { dir: sdk/examples/scripting } |  |  | 0.880 |
| ns | 534 |  | 107 | README feature list, first half | 1.7 | 1.6 | 0.818 |
| walker |  | 558 | 45 | Fs::DirListing { dir: internal/config } |  |  | 0.828 |
| walker |  | 644 | 86 | Fs::DirListing { dir: internal/ui } |  |  | 0.828 |
| ns | 648 |  | 114 | README feature list, second half | 1.8 |  | 0.784 |
| walker |  | 649 | 5 | Fs::DirListing { dir: internal/ui/progress } |  |  | 0.784 |
| ns | 781 |  | 133 | Complete listings for the config / agent / tools / models packages | 1.9 |  | 0.781 |
| walker |  | 878 | 229 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: true } |  |  | 0.879 |
| walker |  | 895 | 17 | Fs::DirListing { dir: examples/hooks } |  |  | 0.879 |
| walker |  | 943 | 48 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.880 |
| walker |  | 969 | 26 | Fs::DirListing { dir: internal/tools } |  |  | 0.928 |
| ns | 999 |  | 218 | Complete listings for the builtin / hooks / session / auth / tokens / ui packages | 1.10 |  | 0.934 |
| walker |  | 1002 | 33 | Fs::DirListing { dir: examples/scripts } |  |  | 0.939 |
| walker |  | 1029 | 27 | Code::CodeKey { rung: Names, file: main.go, decl: 0, sub: 0, line: 0 } |  |  | 0.939 |
| ns | 1089 |  | 90 | Complete listings for examples/, contribute/ and .github/ | 1.11 |  | 0.935 |
| walker |  | 1159 | 130 | Code::CodeKey { rung: Decl, file: main.go, decl: 2, sub: 0, line: 14 } |  |  | 0.941 |
| walker |  | 1217 | 58 | Markdown::Section { file: README.md, section_index: 31, keeps_default_concavity: false } |  |  | 0.943 |
| ns | 1243 |  | 154 | main.go entry point | 2.1 |  | 0.940 |
| walker |  | 1320 | 103 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.942 |
| ns | 1504 |  | 261 | Complete cobra command tree: script, auth (login/logout/status), hooks (list/validate/init) | 2.2 |  | 0.893 |
| walker |  | 1518 | 198 | Code::CodeKey { rung: ModuleDoc, file: internal/tokens/anthropic.go, decl: 0, sub: 0, line: 0 } |  |  | 0.893 |
| walker |  | 1589 | 71 | Code::CodeKey { rung: Names, file: cmd/hooks.go, decl: 0, sub: 0, line: 0 } |  |  | 0.893 |
| walker |  | 1636 | 47 | Code::CodeKey { rung: Decl, file: cmd/hooks.go, decl: 1, sub: 0, line: 16 } |  |  | 0.893 |
| ns | 1754 |  | 250 | Persistent flag registration, part 1: config, system-prompt, model, debug, prompt, quiet | 2.3 |  | 0.860 |
| walker |  | 1821 | 185 | Code::CodeKey { rung: Decl, file: cmd/hooks.go, decl: 3, sub: 0, line: 57 } |  |  | 0.861 |
| ns | 2098 |  | 344 | Persistent flag registration, part 2: no-exit, max-steps, stream, compact, no-hooks, approve-tool-run, session flags | 2.4 |  | 0.819 |
| walker |  | 2141 | 320 | Code::CodeKey { rung: Decl, file: cmd/hooks.go, decl: 2, sub: 0, line: 25 } |  |  | 0.821 |
| walker |  | 2181 | 40 | Plaintext::DeclSurface { file: contribute/build.sh } |  |  | 0.821 |
| walker |  | 2192 | 11 | Plaintext::Whole { file: contribute/build.sh } |  |  | 0.821 |
| ns | 2224 |  | 126 | Provider and TLS flag registration | 2.5 |  | 0.813 |
| walker |  | 2352 | 160 | Code::CodeKey { rung: Names, file: cmd/auth.go, decl: 0, sub: 0, line: 0 } |  |  | 0.813 |
| walker |  | 2471 | 119 | Code::CodeKey { rung: Decl, file: cmd/auth.go, decl: 4, sub: 0, line: 78 } |  |  | 0.817 |
| ns | 2502 |  | 278 | Generation-parameter and Ollama flag registration, with the hidden flag | 2.6 |  | 0.796 |
| walker |  | 2632 | 161 | Code::CodeKey { rung: Decl, file: cmd/auth.go, decl: 1, sub: 0, line: 17 } |  |  | 0.801 |
| ns | 2749 |  | 247 | MCPServerConfig: complete field set including the legacy block | 3.1 |  | 0.773 |
| walker |  | 2799 | 167 | Code::CodeKey { rung: Decl, file: cmd/auth.go, decl: 3, sub: 0, line: 58 } |  |  | 0.780 |
| walker |  | 2974 | 175 | Code::CodeKey { rung: Decl, file: cmd/auth.go, decl: 2, sub: 0, line: 37 } |  |  | 0.787 |
| walker |  | 3039 | 65 | Code::CodeKey { rung: Names, file: sdk/types.go, decl: 0, sub: 0, line: 0 } |  |  | 0.788 |
| walker |  | 3050 | 11 | Code::CodeKey { rung: Body, file: sdk/types.go, decl: 3, sub: 0, line: 18 } |  |  | 0.788 |
| walker |  | 3062 | 12 | Code::CodeKey { rung: Body, file: sdk/types.go, decl: 4, sub: 0, line: 24 } |  |  | 0.788 |
| ns | 3069 |  | 320 | Config struct: application-level keys | 3.2 |  | 0.763 |
| ns | 3250 |  | 181 | Config struct: generation-parameter and TLS keys | 3.3 |  | 0.747 |
| ns | 3514 |  | 264 | GetTransportType: type-to-transport mapping and legacy inference | 3.4 |  | 0.709 |
| walker |  | 3576 | 514 | GoMod::File { file: go.mod } |  |  | 0.709 |
| walker |  | 3789 | 213 | Code::CodeKey { rung: Names, file: sdk/mcphost.go, decl: 0, sub: 0, line: 0 } |  |  | 0.709 |
| walker |  | 3824 | 35 | Code::CodeKey { rung: Decl, file: sdk/mcphost.go, decl: 1, sub: 0, line: 19 } |  |  | 0.709 |
| walker |  | 3892 | 68 | Code::CodeKey { rung: Decl, file: sdk/mcphost.go, decl: 5, sub: 0, line: 163 } |  |  | 0.709 |
| ns | 3938 |  | 424 | Config.Validate: required fields per transport and filter exclusivity | 3.5 |  | 0.674 |
| walker |  | 3999 | 107 | Code::CodeKey { rung: Decl, file: sdk/mcphost.go, decl: 2, sub: 0, line: 28 } |  |  | 0.675 |
| walker |  | 4008 | 9 | Code::CodeKey { rung: Body, file: sdk/mcphost.go, decl: 6, sub: 0, line: 201 } |  |  | 0.675 |
| walker |  | 4017 | 9 | Code::CodeKey { rung: Body, file: sdk/mcphost.go, decl: 10, sub: 0, line: 230 } |  |  | 0.675 |
| walker |  | 4026 | 9 | Code::CodeKey { rung: Body, file: sdk/mcphost.go, decl: 11, sub: 0, line: 237 } |  |  | 0.675 |
| ns | 4141 |  | 203 | Substitution engine: the two regexes plus every symbol in substitution.go | 3.6 |  | 0.662 |
| ns | 4295 |  | 154 | Remaining top-level symbols of internal/config/config.go and all of merger.go (locations) | 3.7 |  | 0.649 |
| walker |  | 4306 | 280 | Code::CodeKey { rung: Names, file: cmd/script.go, decl: 0, sub: 0, line: 0 } |  |  | 0.650 |
| walker |  | 4371 | 65 | Code::CodeKey { rung: Decl, file: cmd/script.go, decl: 9, sub: 0, line: 423 } |  |  | 0.650 |
| walker |  | 4382 | 11 | Code::CodeKey { rung: Body, file: cmd/script.go, decl: 2, sub: 0, line: 78 } |  |  | 0.650 |
| walker |  | 4398 | 16 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 4, sub: 0, line: 148 } |  |  | 0.650 |
| walker |  | 4414 | 16 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 7, sub: 0, line: 279 } |  |  | 0.650 |
| walker |  | 4432 | 18 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 14, sub: 0, line: 511 } |  |  | 0.650 |
| walker |  | 4451 | 19 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 8, sub: 0, line: 288 } |  |  | 0.650 |
| walker |  | 4471 | 20 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 6, sub: 0, line: 255 } |  |  | 0.650 |
| ns | 4541 |  | 246 | Agent struct and its seven callback handler types | 4.1 |  | 0.634 |
| ns | 4815 |  | 274 | Every top-level symbol of internal/agent/agent.go (locations) | 4.2 |  | 0.618 |
| ns | 5007 |  | 192 | The tool-calling loop: step bound, tool-call branch and the approval gate | 4.3 | 4.2 | 0.607 |
| walker |  | 5101 | 630 | Code::CodeKey { rung: Decl, file: cmd/script.go, decl: 1, sub: 0, line: 28 } |  |  | 0.616 |
| walker |  | 5130 | 29 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 6, sub: 0, line: 201 } |  |  | 0.616 |
| walker |  | 5159 | 29 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 9, sub: 0, line: 224 } |  |  | 0.616 |
| ns | 5186 |  | 179 | agent factory: AgentCreationOptions fields and both functions | 4.4 |  | 0.605 |
| walker |  | 5220 | 61 | Code::CodeKey { rung: Names, file: internal/models/models_data.go, decl: 0, sub: 0, line: 0 } |  |  | 0.605 |
| walker |  | 5240 | 20 | Code::CodeKey { rung: Decl, file: internal/models/models_data.go, decl: 3, sub: 0, line: 26 } |  |  | 0.605 |
| walker |  | 5286 | 46 | Code::CodeKey { rung: Decl, file: internal/models/models_data.go, decl: 2, sub: 0, line: 18 } |  |  | 0.605 |
| walker |  | 5338 | 52 | Code::CodeKey { rung: Decl, file: internal/models/models_data.go, decl: 4, sub: 0, line: 32 } |  |  | 0.605 |
| walker |  | 5404 | 66 | Code::CodeKey { rung: Decl, file: internal/models/models_data.go, decl: 1, sub: 0, line: 7 } |  |  | 0.605 |
| walker |  | 5416 | 12 | Code::CodeKey { rung: Doc, file: internal/models/models_data.go, decl: 2, sub: 0, line: 18 } |  |  | 0.605 |
| walker |  | 5428 | 12 | Code::CodeKey { rung: Doc, file: internal/models/models_data.go, decl: 4, sub: 0, line: 32 } |  |  | 0.605 |
| walker |  | 5442 | 14 | Code::CodeKey { rung: Doc, file: internal/models/models_data.go, decl: 1, sub: 0, line: 7 } |  |  | 0.605 |
| walker |  | 5456 | 14 | Code::CodeKey { rung: Doc, file: internal/models/models_data.go, decl: 3, sub: 0, line: 26 } |  |  | 0.605 |
| walker |  | 5471 | 15 | Code::CodeKey { rung: Doc, file: internal/models/models_data.go, decl: 5, sub: 0, line: 41 } |  |  | 0.605 |
| ns | 5549 |  | 363 | Every top-level symbol of internal/tools/mcp.go (locations) | 4.5 |  | 0.588 |
| walker |  | 5557 | 86 | Code::CodeKey { rung: Names, file: internal/models/generate_models.go, decl: 0, sub: 0, line: 0 } |  |  | 0.589 |
| walker |  | 5614 | 57 | Code::CodeKey { rung: Decl, file: internal/models/generate_models.go, decl: 3, sub: 0, line: 50 } |  |  | 0.589 |
| walker |  | 5745 | 131 | Code::CodeKey { rung: Decl, file: internal/models/generate_models.go, decl: 2, sub: 0, line: 37 } |  |  | 0.589 |
| ns | 5765 |  | 216 | AgenticLoopConfig head and every mode-driving function in cmd/root.go (locations) | 4.6 |  | 0.577 |
| walker |  | 5887 | 142 | Code::CodeKey { rung: Decl, file: internal/models/generate_models.go, decl: 4, sub: 0, line: 60 } |  |  | 0.577 |
| ns | 5907 |  | 142 | CreateProvider: the complete list of supported providers | 5.1 |  | 0.568 |
| walker |  | 6088 | 201 | Code::CodeKey { rung: Decl, file: internal/models/generate_models.go, decl: 1, sub: 0, line: 18 } |  |  | 0.568 |
| walker |  | 6162 | 74 | Code::CodeKey { rung: Names, file: internal/hooks/schemas.go, decl: 0, sub: 0, line: 0 } |  |  | 0.569 |
| walker |  | 6186 | 24 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 4, sub: 0, line: 42 } |  |  | 0.569 |
| walker |  | 6230 | 44 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 2, sub: 0, line: 23 } |  |  | 0.569 |
| ns | 6235 |  | 328 | Every top-level symbol of internal/models/providers.go (locations) | 5.2 | 5.1 | 0.553 |
| walker |  | 6291 | 61 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 3, sub: 0, line: 32 } |  |  | 0.553 |
| walker |  | 6380 | 89 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 6, sub: 0, line: 62 } |  |  | 0.553 |
| ns | 6385 |  | 150 | ModelsRegistry: model validation and suggestion API | 5.3 |  | 0.547 |
| ns | 6477 |  | 92 | The generated model catalogue: generator types and the DO-NOT-EDIT header | 5.4 |  | 0.546 |
| walker |  | 6487 | 107 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 5, sub: 0, line: 50 } |  |  | 0.546 |
| walker |  | 6646 | 159 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 1, sub: 0, line: 10 } |  |  | 0.547 |
| ns | 6653 |  | 176 | Builtin server registry: the complete set of in-process servers | 6.1 |  | 0.538 |
| walker |  | 6708 | 62 | Plaintext::DeclSurface { file: contribute/boost.sh } |  |  | 0.538 |
| ns | 6825 |  | 172 | Every top-level symbol of internal/builtin/registry.go (locations) | 6.2 | 6.1 | 0.532 |
| walker |  | 6982 | 274 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.533 |
| walker |  | 7013 | 31 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 12, sub: 0, line: 480 } |  |  | 0.533 |
| ns | 7042 |  | 217 | Bash builtin: output/timeout limits and the complete banned-command list | 6.3 |  | 0.520 |
| walker |  | 7045 | 32 | Code::CodeKey { rung: Doc, file: sdk/types.go, decl: 3, sub: 0, line: 18 } |  |  | 0.520 |
| ns | 7240 |  | 198 | The http builtin: its four tools and every symbol in http.go (locations) | 6.4 |  | 0.512 |
| walker |  | 7330 | 285 | Code::CodeKey { rung: Names, file: cmd/root.go, decl: 0, sub: 0, line: 0 } |  |  | 0.512 |
| ns | 7352 |  | 112 | HookEvent: the complete set of hook events | 7.1 |  | 0.508 |
| walker |  | 7404 | 74 | Code::CodeKey { rung: Decl, file: cmd/root.go, decl: 1, sub: 0, line: 27 } |  |  | 0.508 |
| ns | 7567 |  | 215 | Hook configuration schema: HookConfig, HookMatcher, HookEntry | 7.2 |  | 0.501 |
| walker |  | 7600 | 196 | Code::CodeKey { rung: Names, file: cmd/root.go, decl: 0, sub: 1, line: 0 } |  |  | 0.501 |
| walker |  | 7613 | 13 | Code::CodeKey { rung: Decl, file: cmd/root.go, decl: 2, sub: 0, line: 67 } |  |  | 0.501 |
| walker |  | 7623 | 10 | Code::CodeKey { rung: Body, file: cmd/root.go, decl: 12, sub: 0, line: 361 } |  |  | 0.501 |
| walker |  | 7638 | 15 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 2, sub: 0, line: 67 } |  |  | 0.501 |
| walker |  | 7826 | 188 | Code::CodeKey { rung: Names, file: cmd/root.go, decl: 0, sub: 2, line: 0 } |  |  | 0.504 |
| ns | 7839 |  | 272 | Hook wire protocol: CommonInput and HookOutput | 7.3 |  | 0.517 |
| ns | 8030 |  | 191 | Per-event hook input structs | 7.4 |  | 0.526 |
| walker |  | 8042 | 216 | Code::CodeKey { rung: Decl, file: cmd/root.go, decl: 14, sub: 0, line: 758 } |  |  | 0.537 |
| walker |  | 8252 | 210 | Code::CodeKey { rung: Names, file: cmd/root.go, decl: 0, sub: 3, line: 0 } |  |  | 0.545 |
| walker |  | 8267 | 15 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 22, sub: 0, line: 1349 } |  |  | 0.545 |
| walker |  | 8285 | 18 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 21, sub: 0, line: 1316 } |  |  | 0.545 |
| walker |  | 8304 | 19 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 16, sub: 0, line: 794 } |  |  | 0.545 |
| walker |  | 8323 | 19 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 17, sub: 0, line: 811 } |  |  | 0.545 |
| ns | 8336 |  | 306 | Hook executor and validator symbol rosters | 7.5 |  | 0.535 |
| walker |  | 8342 | 19 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 20, sub: 0, line: 1229 } |  |  | 0.535 |
| walker |  | 8362 | 20 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 15, sub: 0, line: 777 } |  |  | 0.535 |
| walker |  | 8382 | 20 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 19, sub: 0, line: 1195 } |  |  | 0.535 |
| ns | 8493 |  | 157 | README: hooks.yml file locations and the --no-hooks escape hatch | 7.6 | 1.6 | 0.531 |
| ns | 8682 |  | 189 | Script mode: a worked frontmatter example and the variable rules | 8.1 |  | 0.540 |
| ns | 8813 |  | 131 | Every top-level symbol of cmd/script.go (locations) | 8.2 |  | 0.547 |
| walker |  | 8852 | 470 | Code::CodeKey { rung: Decl, file: cmd/root.go, decl: 6, sub: 0, line: 92 } |  |  | 0.547 |
| walker |  | 8875 | 23 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 18, sub: 0, line: 879 } |  |  | 0.547 |
| walker |  | 8908 | 33 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 10, sub: 0, line: 431 } |  |  | 0.547 |
| walker |  | 8941 | 33 | Code::CodeKey { rung: Doc, file: sdk/types.go, decl: 1, sub: 0, line: 10 } |  |  | 0.547 |
| walker |  | 8975 | 34 | Code::CodeKey { rung: Doc, file: sdk/types.go, decl: 2, sub: 0, line: 14 } |  |  | 0.547 |
| walker |  | 9009 | 34 | Code::CodeKey { rung: Doc, file: sdk/types.go, decl: 4, sub: 0, line: 24 } |  |  | 0.547 |
| walker |  | 9037 | 28 | Code::CodeKey { rung: Names, file: internal/tokens/init.go, decl: 0, sub: 0, line: 0 } |  |  | 0.547 |
| walker |  | 9052 | 15 | Code::CodeKey { rung: Body, file: internal/tokens/init.go, decl: 1, sub: 0, line: 21 } |  |  | 0.547 |
| walker |  | 9067 | 15 | Code::CodeKey { rung: Body, file: internal/tokens/init.go, decl: 2, sub: 0, line: 50 } |  |  | 0.547 |
| ns | 9111 |  | 298 | Session file format: Session, Metadata, Message and ToolCall fields | 8.3 |  | 0.538 |
| walker |  | 9155 | 88 | Code::CodeKey { rung: Names, file: internal/hooks/config.go, decl: 0, sub: 0, line: 0 } |  |  | 0.540 |
| walker |  | 9182 | 27 | Code::CodeKey { rung: Decl, file: internal/hooks/config.go, decl: 1, sub: 0, line: 14 } |  |  | 0.541 |
| walker |  | 9237 | 55 | Code::CodeKey { rung: Decl, file: internal/hooks/config.go, decl: 3, sub: 0, line: 30 } |  |  | 0.547 |
| walker |  | 9302 | 65 | Code::CodeKey { rung: Decl, file: internal/hooks/config.go, decl: 2, sub: 0, line: 21 } |  |  | 0.556 |
| walker |  | 9316 | 14 | Code::CodeKey { rung: Doc, file: internal/hooks/config.go, decl: 6, sub: 0, line: 113 } |  |  | 0.556 |
| walker |  | 9352 | 36 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 11, sub: 0, line: 442 } |  |  | 0.556 |
| walker |  | 9382 | 30 | Code::CodeKey { rung: Names, file: internal/auth/browser.go, decl: 0, sub: 0, line: 0 } |  |  | 0.556 |
| ns | 9405 |  | 294 | The public SDK surface: Options and every exported symbol | 8.4 |  | 0.566 |
| walker |  | 9442 | 60 | Code::CodeKey { rung: Names, file: internal/ui/commands.go, decl: 0, sub: 0, line: 0 } |  |  | 0.566 |
| walker |  | 9493 | 51 | Code::CodeKey { rung: Decl, file: internal/ui/commands.go, decl: 1, sub: 0, line: 6 } |  |  | 0.566 |
| walker |  | 9512 | 19 | Code::CodeKey { rung: Doc, file: internal/hooks/config.go, decl: 5, sub: 0, line: 97 } |  |  | 0.566 |
| walker |  | 9549 | 37 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 8, sub: 0, line: 218 } |  |  | 0.566 |
| ns | 9607 |  | 202 | The complete slash-command table: names and descriptions | 8.5 |  | 0.560 |
| walker |  | 9761 | 212 | Code::CodeKey { rung: Names, file: internal/config/config.go, decl: 0, sub: 0, line: 0 } |  |  | 0.568 |
| walker |  | 9803 | 42 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 3, sub: 0, line: 109 } |  |  | 0.568 |
| walker |  | 9812 | 9 | Code::CodeKey { rung: Body, file: internal/config/config.go, decl: 14, sub: 0, line: 459 } |  |  | 0.568 |
| ns | 9828 |  | 221 | ui.SetupCLI: the AgentInterface contract and CLISetupOptions | 8.6 |  | 0.560 |
| ns | 9984 |  | 156 | Release packaging: the goreleaser build matrix | 9.1 |  | 0.554 |
| walker |  | 9991 | 179 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 5, sub: 0, line: 137 } |  |  | 0.554 |

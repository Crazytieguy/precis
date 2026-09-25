Score(3000)=0.831 I=0.948 C=0.729 ns_rows≤3K=18/51 grid(1000/1442/2080/3000/4327/6240/9000)=0.940/0.996/0.913/0.831/0.685/0.582/0.570

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 40 | 40 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 63 |  | 63 | README title and one-line identity | 1.1 |  | 0.000 |
| walker |  | 103 | 63 | Markdown::ReadmeHeadline { file: README.md } |  |  | 1.000 |
| ns | 103 |  | 40 | Complete repository root listing | 1.2 |  | 1.000 |
| walker |  | 118 | 15 | Fs::DirListing { dir: contribute } |  |  | 1.000 |
| walker |  | 126 | 8 | Fs::DirListing { dir: contribute/conf } |  |  | 1.000 |
| ns | 133 |  | 30 | Complete internal/ package listing | 1.3 |  | 0.747 |
| walker |  | 150 | 24 | Fs::DirListing { dir: sdk } |  |  | 0.752 |
| walker |  | 180 | 30 | Fs::DirListing { dir: internal } |  |  | 1.000 |
| walker |  | 188 | 8 | Fs::DirListing { dir: internal/session } |  |  | 1.000 |
| walker |  | 200 | 12 | Fs::DirListing { dir: internal/agent } |  |  | 1.000 |
| ns | 207 |  | 74 | Complete cmd/ and sdk/ listings | 1.4 |  | 0.777 |
| walker |  | 213 | 13 | Fs::DirListing { dir: internal/tokens } |  |  | 0.777 |
| ns | 260 |  | 53 | Module path, Go version and toolchain | 1.5 |  | 0.726 |
| walker |  | 266 | 53 | GoMod::Identity { file: go.mod } |  |  | 0.798 |
| walker |  | 302 | 36 | Fs::DirListing { dir: cmd } |  |  | 0.907 |
| walker |  | 319 | 17 | Fs::DirListing { dir: internal/auth } |  |  | 0.907 |
| walker |  | 325 | 6 | Fs::DirListing { dir: examples } |  |  | 0.908 |
| walker |  | 335 | 10 | Fs::DirListing { dir: .github/workflows } |  |  | 0.910 |
| walker |  | 370 | 35 | Fs::DirListing { dir: internal/models } |  |  | 0.915 |
| walker |  | 375 | 5 | Fs::DirListing { dir: internal/models/anthropic } |  |  | 0.915 |
| walker |  | 380 | 5 | Fs::DirListing { dir: internal/models/gemini } |  |  | 0.916 |
| walker |  | 385 | 5 | Fs::DirListing { dir: internal/models/openai } |  |  | 0.917 |
| walker |  | 424 | 39 | Fs::DirListing { dir: internal/hooks } |  |  | 0.922 |
| ns | 427 |  | 167 | All README section headings (locations only) | 1.6 |  | 0.789 |
| walker |  | 434 | 10 | Fs::DirListing { dir: internal/hooks/testdata } |  |  | 0.790 |
| walker |  | 474 | 40 | Fs::DirListing { dir: internal/builtin } |  |  | 0.797 |
| walker |  | 480 | 6 | Fs::DirListing { dir: sdk/examples } |  |  | 0.836 |
| walker |  | 484 | 4 | Fs::DirListing { dir: sdk/examples/basic } |  |  | 0.857 |
| walker |  | 488 | 4 | Fs::DirListing { dir: sdk/examples/scripting } |  |  | 0.880 |
| walker |  | 533 | 45 | Fs::DirListing { dir: internal/config } |  |  | 0.880 |
| ns | 534 |  | 107 | README feature list, first half | 1.7 | 1.6 | 0.828 |
| walker |  | 619 | 86 | Fs::DirListing { dir: internal/ui } |  |  | 0.828 |
| walker |  | 624 | 5 | Fs::DirListing { dir: internal/ui/progress } |  |  | 0.828 |
| walker |  | 641 | 17 | Fs::DirListing { dir: examples/hooks } |  |  | 0.828 |
| ns | 648 |  | 114 | README feature list, second half | 1.8 |  | 0.784 |
| ns | 781 |  | 133 | Complete listings for the config / agent / tools / models packages | 1.9 |  | 0.783 |
| walker |  | 808 | 167 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.874 |
| walker |  | 830 | 22 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.874 |
| ns | 999 |  | 218 | Complete listings for the builtin / hooks / session / auth / tokens / ui packages | 1.10 |  | 0.893 |
| walker |  | 1046 | 216 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: true } |  |  | 0.962 |
| walker |  | 1081 | 35 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.962 |
| ns | 1088 |  | 89 | Complete listings for examples/, contribute/ and .github/ | 1.11 |  | 0.925 |
| walker |  | 1124 | 43 | Markdown::Section { file: README.md, section_index: 31, keeps_default_concavity: false } |  |  | 0.925 |
| walker |  | 1150 | 26 | Fs::DirListing { dir: internal/tools } |  |  | 0.961 |
| walker |  | 1177 | 27 | Code::CodeKey { rung: Names, file: main.go, decl: 0, sub: 0, line: 0 } |  |  | 0.961 |
| ns | 1242 |  | 154 | main.go entry point | 2.1 |  | 0.919 |
| walker |  | 1307 | 130 | Code::CodeKey { rung: Decl, file: main.go, decl: 2, sub: 0, line: 14 } |  |  | 0.963 |
| walker |  | 1340 | 33 | Fs::DirListing { dir: examples/scripts } |  |  | 0.996 |
| walker |  | 1432 | 92 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.996 |
| ns | 1503 |  | 261 | Complete cobra command tree: script, auth (login/logout/status), hooks (list/validate/init) | 2.2 |  | 0.944 |
| walker |  | 1630 | 198 | Code::CodeKey { rung: ModuleDoc, file: internal/tokens/anthropic.go, decl: 0, sub: 0, line: 0 } |  |  | 0.944 |
| walker |  | 1701 | 71 | Code::CodeKey { rung: Names, file: cmd/hooks.go, decl: 0, sub: 0, line: 0 } |  |  | 0.944 |
| walker |  | 1748 | 47 | Code::CodeKey { rung: Decl, file: cmd/hooks.go, decl: 1, sub: 0, line: 16 } |  |  | 0.945 |
| ns | 1753 |  | 250 | Persistent flag registration, part 1: config, system-prompt, model, debug, prompt, quiet | 2.3 |  | 0.909 |
| walker |  | 1933 | 185 | Code::CodeKey { rung: Decl, file: cmd/hooks.go, decl: 3, sub: 0, line: 57 } |  |  | 0.911 |
| ns | 2097 |  | 344 | Persistent flag registration, part 2: no-exit, max-steps, stream, compact, no-hooks, approve-tool-run, session flags | 2.4 |  | 0.866 |
| ns | 2223 |  | 126 | Provider and TLS flag registration | 2.5 |  | 0.857 |
| walker |  | 2253 | 320 | Code::CodeKey { rung: Decl, file: cmd/hooks.go, decl: 2, sub: 0, line: 25 } |  |  | 0.859 |
| walker |  | 2293 | 40 | Plaintext::DeclSurface { file: contribute/build.sh } |  |  | 0.859 |
| walker |  | 2304 | 11 | Plaintext::Whole { file: contribute/build.sh } |  |  | 0.859 |
| walker |  | 2464 | 160 | Code::CodeKey { rung: Names, file: cmd/auth.go, decl: 0, sub: 0, line: 0 } |  |  | 0.859 |
| ns | 2501 |  | 278 | Generation-parameter and Ollama flag registration, with the hidden flag | 2.6 |  | 0.838 |
| walker |  | 2583 | 119 | Code::CodeKey { rung: Decl, file: cmd/auth.go, decl: 4, sub: 0, line: 78 } |  |  | 0.841 |
| walker |  | 2744 | 161 | Code::CodeKey { rung: Decl, file: cmd/auth.go, decl: 1, sub: 0, line: 17 } |  |  | 0.846 |
| ns | 2748 |  | 247 | MCPServerConfig: complete field set including the legacy block | 3.1 |  | 0.817 |
| walker |  | 2911 | 167 | Code::CodeKey { rung: Decl, file: cmd/auth.go, decl: 3, sub: 0, line: 58 } |  |  | 0.823 |
| ns | 3068 |  | 320 | Config struct: application-level keys | 3.2 |  | 0.798 |
| walker |  | 3086 | 175 | Code::CodeKey { rung: Decl, file: cmd/auth.go, decl: 2, sub: 0, line: 37 } |  |  | 0.805 |
| walker |  | 3151 | 65 | Code::CodeKey { rung: Names, file: sdk/types.go, decl: 0, sub: 0, line: 0 } |  |  | 0.805 |
| walker |  | 3162 | 11 | Code::CodeKey { rung: Body, file: sdk/types.go, decl: 3, sub: 0, line: 18 } |  |  | 0.805 |
| walker |  | 3174 | 12 | Code::CodeKey { rung: Body, file: sdk/types.go, decl: 4, sub: 0, line: 24 } |  |  | 0.805 |
| ns | 3249 |  | 181 | Config struct: generation-parameter and TLS keys | 3.3 |  | 0.788 |
| ns | 3513 |  | 264 | GetTransportType: type-to-transport mapping and legacy inference | 3.4 |  | 0.748 |
| walker |  | 3688 | 514 | GoMod::File { file: go.mod } |  |  | 0.748 |
| walker |  | 3901 | 213 | Code::CodeKey { rung: Names, file: sdk/mcphost.go, decl: 0, sub: 0, line: 0 } |  |  | 0.748 |
| walker |  | 3936 | 35 | Code::CodeKey { rung: Decl, file: sdk/mcphost.go, decl: 1, sub: 0, line: 19 } |  |  | 0.748 |
| ns | 3937 |  | 424 | Config.Validate: required fields per transport and filter exclusivity | 3.5 |  | 0.711 |
| walker |  | 4004 | 68 | Code::CodeKey { rung: Decl, file: sdk/mcphost.go, decl: 5, sub: 0, line: 163 } |  |  | 0.711 |
| walker |  | 4111 | 107 | Code::CodeKey { rung: Decl, file: sdk/mcphost.go, decl: 2, sub: 0, line: 28 } |  |  | 0.712 |
| walker |  | 4120 | 9 | Code::CodeKey { rung: Body, file: sdk/mcphost.go, decl: 6, sub: 0, line: 201 } |  |  | 0.712 |
| walker |  | 4129 | 9 | Code::CodeKey { rung: Body, file: sdk/mcphost.go, decl: 10, sub: 0, line: 230 } |  |  | 0.712 |
| ns | 4140 |  | 203 | Substitution engine: the two regexes plus every symbol in substitution.go | 3.6 |  | 0.699 |
| ns | 4294 |  | 154 | Remaining top-level symbols of internal/config/config.go and all of merger.go (locations) | 3.7 |  | 0.685 |
| walker |  | 4409 | 280 | Code::CodeKey { rung: Names, file: cmd/script.go, decl: 0, sub: 0, line: 0 } |  |  | 0.686 |
| walker |  | 4474 | 65 | Code::CodeKey { rung: Decl, file: cmd/script.go, decl: 9, sub: 0, line: 423 } |  |  | 0.686 |
| walker |  | 4490 | 16 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 4, sub: 0, line: 148 } |  |  | 0.686 |
| walker |  | 4506 | 16 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 7, sub: 0, line: 279 } |  |  | 0.686 |
| walker |  | 4524 | 18 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 14, sub: 0, line: 511 } |  |  | 0.686 |
| ns | 4540 |  | 246 | Agent struct and its seven callback handler types | 4.1 |  | 0.669 |
| walker |  | 4543 | 19 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 8, sub: 0, line: 288 } |  |  | 0.669 |
| walker |  | 4563 | 20 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 6, sub: 0, line: 255 } |  |  | 0.669 |
| walker |  | 4572 | 9 | Code::CodeKey { rung: Body, file: sdk/mcphost.go, decl: 11, sub: 0, line: 237 } |  |  | 0.669 |
| ns | 4814 |  | 274 | Every top-level symbol of internal/agent/agent.go (locations) | 4.2 |  | 0.652 |
| ns | 5006 |  | 192 | The tool-calling loop: step bound, tool-call branch and the approval gate | 4.3 | 4.2 | 0.640 |
| ns | 5185 |  | 179 | agent factory: AgentCreationOptions fields and both functions | 4.4 |  | 0.629 |
| walker |  | 5202 | 630 | Code::CodeKey { rung: Decl, file: cmd/script.go, decl: 1, sub: 0, line: 28 } |  |  | 0.638 |
| walker |  | 5465 | 263 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.638 |
| walker |  | 5494 | 29 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 6, sub: 0, line: 201 } |  |  | 0.638 |
| walker |  | 5523 | 29 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 9, sub: 0, line: 224 } |  |  | 0.638 |
| ns | 5548 |  | 363 | Every top-level symbol of internal/tools/mcp.go (locations) | 4.5 |  | 0.620 |
| walker |  | 5584 | 61 | Code::CodeKey { rung: Names, file: internal/models/models_data.go, decl: 0, sub: 0, line: 0 } |  |  | 0.620 |
| walker |  | 5604 | 20 | Code::CodeKey { rung: Decl, file: internal/models/models_data.go, decl: 3, sub: 0, line: 26 } |  |  | 0.620 |
| walker |  | 5650 | 46 | Code::CodeKey { rung: Decl, file: internal/models/models_data.go, decl: 2, sub: 0, line: 18 } |  |  | 0.620 |
| walker |  | 5702 | 52 | Code::CodeKey { rung: Decl, file: internal/models/models_data.go, decl: 4, sub: 0, line: 32 } |  |  | 0.620 |
| ns | 5764 |  | 216 | AgenticLoopConfig head and every mode-driving function in cmd/root.go (locations) | 4.6 |  | 0.608 |
| walker |  | 5768 | 66 | Code::CodeKey { rung: Decl, file: internal/models/models_data.go, decl: 1, sub: 0, line: 7 } |  |  | 0.608 |
| walker |  | 5780 | 12 | Code::CodeKey { rung: Doc, file: internal/models/models_data.go, decl: 2, sub: 0, line: 18 } |  |  | 0.608 |
| walker |  | 5792 | 12 | Code::CodeKey { rung: Doc, file: internal/models/models_data.go, decl: 4, sub: 0, line: 32 } |  |  | 0.608 |
| walker |  | 5806 | 14 | Code::CodeKey { rung: Doc, file: internal/models/models_data.go, decl: 1, sub: 0, line: 7 } |  |  | 0.608 |
| walker |  | 5820 | 14 | Code::CodeKey { rung: Doc, file: internal/models/models_data.go, decl: 3, sub: 0, line: 26 } |  |  | 0.608 |
| walker |  | 5835 | 15 | Code::CodeKey { rung: Doc, file: internal/models/models_data.go, decl: 5, sub: 0, line: 41 } |  |  | 0.608 |
| ns | 5906 |  | 142 | CreateProvider: the complete list of supported providers | 5.1 |  | 0.599 |
| walker |  | 5921 | 86 | Code::CodeKey { rung: Names, file: internal/models/generate_models.go, decl: 0, sub: 0, line: 0 } |  |  | 0.599 |
| walker |  | 5978 | 57 | Code::CodeKey { rung: Decl, file: internal/models/generate_models.go, decl: 3, sub: 0, line: 50 } |  |  | 0.599 |
| walker |  | 6109 | 131 | Code::CodeKey { rung: Decl, file: internal/models/generate_models.go, decl: 2, sub: 0, line: 37 } |  |  | 0.599 |
| ns | 6234 |  | 328 | Every top-level symbol of internal/models/providers.go (locations) | 5.2 | 5.1 | 0.582 |
| walker |  | 6251 | 142 | Code::CodeKey { rung: Decl, file: internal/models/generate_models.go, decl: 4, sub: 0, line: 60 } |  |  | 0.582 |
| ns | 6384 |  | 150 | ModelsRegistry: model validation and suggestion API | 5.3 |  | 0.576 |
| walker |  | 6452 | 201 | Code::CodeKey { rung: Decl, file: internal/models/generate_models.go, decl: 1, sub: 0, line: 18 } |  |  | 0.576 |
| ns | 6476 |  | 92 | The generated model catalogue: generator types and the DO-NOT-EDIT header | 5.4 |  | 0.575 |
| walker |  | 6526 | 74 | Code::CodeKey { rung: Names, file: internal/hooks/schemas.go, decl: 0, sub: 0, line: 0 } |  |  | 0.575 |
| walker |  | 6550 | 24 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 4, sub: 0, line: 42 } |  |  | 0.575 |
| walker |  | 6594 | 44 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 2, sub: 0, line: 23 } |  |  | 0.575 |
| ns | 6652 |  | 176 | Builtin server registry: the complete set of in-process servers | 6.1 |  | 0.565 |
| walker |  | 6655 | 61 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 3, sub: 0, line: 32 } |  |  | 0.565 |
| walker |  | 6744 | 89 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 6, sub: 0, line: 62 } |  |  | 0.565 |
| ns | 6824 |  | 172 | Every top-level symbol of internal/builtin/registry.go (locations) | 6.2 | 6.1 | 0.558 |
| walker |  | 6851 | 107 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 5, sub: 0, line: 50 } |  |  | 0.559 |
| walker |  | 7010 | 159 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 1, sub: 0, line: 10 } |  |  | 0.560 |
| ns | 7041 |  | 217 | Bash builtin: output/timeout limits and the complete banned-command list | 6.3 |  | 0.547 |
| walker |  | 7072 | 62 | Plaintext::DeclSurface { file: contribute/boost.sh } |  |  | 0.547 |
| walker |  | 7103 | 31 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 12, sub: 0, line: 480 } |  |  | 0.547 |
| walker |  | 7135 | 32 | Code::CodeKey { rung: Doc, file: sdk/types.go, decl: 3, sub: 0, line: 18 } |  |  | 0.547 |
| ns | 7239 |  | 198 | The http builtin: its four tools and every symbol in http.go (locations) | 6.4 |  | 0.538 |
| ns | 7351 |  | 112 | HookEvent: the complete set of hook events | 7.1 |  | 0.534 |
| walker |  | 7420 | 285 | Code::CodeKey { rung: Names, file: cmd/root.go, decl: 0, sub: 0, line: 0 } |  |  | 0.534 |
| walker |  | 7494 | 74 | Code::CodeKey { rung: Decl, file: cmd/root.go, decl: 1, sub: 0, line: 27 } |  |  | 0.534 |
| ns | 7566 |  | 215 | Hook configuration schema: HookConfig, HookMatcher, HookEntry | 7.2 |  | 0.527 |
| walker |  | 7690 | 196 | Code::CodeKey { rung: Names, file: cmd/root.go, decl: 0, sub: 1, line: 0 } |  |  | 0.527 |
| walker |  | 7703 | 13 | Code::CodeKey { rung: Decl, file: cmd/root.go, decl: 2, sub: 0, line: 67 } |  |  | 0.527 |
| walker |  | 7718 | 15 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 2, sub: 0, line: 67 } |  |  | 0.527 |
| ns | 7838 |  | 272 | Hook wire protocol: CommonInput and HookOutput | 7.3 |  | 0.539 |
| walker |  | 7906 | 188 | Code::CodeKey { rung: Names, file: cmd/root.go, decl: 0, sub: 2, line: 0 } |  |  | 0.542 |
| ns | 8029 |  | 191 | Per-event hook input structs | 7.4 |  | 0.550 |
| walker |  | 8122 | 216 | Code::CodeKey { rung: Decl, file: cmd/root.go, decl: 14, sub: 0, line: 758 } |  |  | 0.561 |
| walker |  | 8332 | 210 | Code::CodeKey { rung: Names, file: cmd/root.go, decl: 0, sub: 3, line: 0 } |  |  | 0.569 |
| ns | 8335 |  | 306 | Hook executor and validator symbol rosters | 7.5 |  | 0.559 |
| walker |  | 8347 | 15 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 22, sub: 0, line: 1349 } |  |  | 0.559 |
| walker |  | 8365 | 18 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 21, sub: 0, line: 1316 } |  |  | 0.559 |
| walker |  | 8384 | 19 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 16, sub: 0, line: 794 } |  |  | 0.559 |
| walker |  | 8403 | 19 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 17, sub: 0, line: 811 } |  |  | 0.559 |
| walker |  | 8422 | 19 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 20, sub: 0, line: 1229 } |  |  | 0.559 |
| walker |  | 8442 | 20 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 15, sub: 0, line: 777 } |  |  | 0.559 |
| walker |  | 8462 | 20 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 19, sub: 0, line: 1195 } |  |  | 0.559 |
| ns | 8492 |  | 157 | README: hooks.yml file locations and the --no-hooks escape hatch | 7.6 | 1.6 | 0.554 |
| ns | 8681 |  | 189 | Script mode: a worked frontmatter example and the variable rules | 8.1 |  | 0.563 |
| ns | 8812 |  | 131 | Every top-level symbol of cmd/script.go (locations) | 8.2 |  | 0.570 |
| walker |  | 8932 | 470 | Code::CodeKey { rung: Decl, file: cmd/root.go, decl: 6, sub: 0, line: 92 } |  |  | 0.570 |
| walker |  | 8955 | 23 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 18, sub: 0, line: 879 } |  |  | 0.570 |
| walker |  | 8988 | 33 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 10, sub: 0, line: 431 } |  |  | 0.570 |
| walker |  | 9021 | 33 | Code::CodeKey { rung: Doc, file: sdk/types.go, decl: 1, sub: 0, line: 10 } |  |  | 0.570 |
| walker |  | 9055 | 34 | Code::CodeKey { rung: Doc, file: sdk/types.go, decl: 2, sub: 0, line: 14 } |  |  | 0.570 |
| walker |  | 9089 | 34 | Code::CodeKey { rung: Doc, file: sdk/types.go, decl: 4, sub: 0, line: 24 } |  |  | 0.570 |
| ns | 9110 |  | 298 | Session file format: Session, Metadata, Message and ToolCall fields | 8.3 |  | 0.561 |
| walker |  | 9117 | 28 | Code::CodeKey { rung: Names, file: internal/tokens/init.go, decl: 0, sub: 0, line: 0 } |  |  | 0.561 |
| walker |  | 9132 | 15 | Code::CodeKey { rung: Body, file: internal/tokens/init.go, decl: 1, sub: 0, line: 21 } |  |  | 0.561 |
| walker |  | 9147 | 15 | Code::CodeKey { rung: Body, file: internal/tokens/init.go, decl: 2, sub: 0, line: 50 } |  |  | 0.561 |
| walker |  | 9338 | 191 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.561 |
| ns | 9404 |  | 294 | The public SDK surface: Options and every exported symbol | 8.4 |  | 0.572 |
| walker |  | 9426 | 88 | Code::CodeKey { rung: Names, file: internal/hooks/config.go, decl: 0, sub: 0, line: 0 } |  |  | 0.573 |
| walker |  | 9453 | 27 | Code::CodeKey { rung: Decl, file: internal/hooks/config.go, decl: 1, sub: 0, line: 14 } |  |  | 0.575 |
| walker |  | 9508 | 55 | Code::CodeKey { rung: Decl, file: internal/hooks/config.go, decl: 3, sub: 0, line: 30 } |  |  | 0.580 |
| walker |  | 9573 | 65 | Code::CodeKey { rung: Decl, file: internal/hooks/config.go, decl: 2, sub: 0, line: 21 } |  |  | 0.588 |
| walker |  | 9587 | 14 | Code::CodeKey { rung: Doc, file: internal/hooks/config.go, decl: 6, sub: 0, line: 113 } |  |  | 0.588 |
| ns | 9606 |  | 202 | The complete slash-command table: names and descriptions | 8.5 |  | 0.582 |
| walker |  | 9623 | 36 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 11, sub: 0, line: 442 } |  |  | 0.582 |
| walker |  | 9653 | 30 | Code::CodeKey { rung: Names, file: internal/auth/browser.go, decl: 0, sub: 0, line: 0 } |  |  | 0.582 |
| walker |  | 9713 | 60 | Code::CodeKey { rung: Names, file: internal/ui/commands.go, decl: 0, sub: 0, line: 0 } |  |  | 0.582 |
| walker |  | 9764 | 51 | Code::CodeKey { rung: Decl, file: internal/ui/commands.go, decl: 1, sub: 0, line: 6 } |  |  | 0.582 |
| walker |  | 9783 | 19 | Code::CodeKey { rung: Doc, file: internal/hooks/config.go, decl: 5, sub: 0, line: 97 } |  |  | 0.582 |
| walker |  | 9820 | 37 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 8, sub: 0, line: 218 } |  |  | 0.582 |
| ns | 9827 |  | 221 | ui.SetupCLI: the AgentInterface contract and CLISetupOptions | 8.6 |  | 0.573 |
| ns | 9983 |  | 156 | Release packaging: the goreleaser build matrix | 9.1 |  | 0.567 |
| walker |  | 9985 | 165 | Code::CodeKey { rung: Names, file: internal/config/config.go, decl: 0, sub: 0, line: 0 } |  |  | 0.571 |

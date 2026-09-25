Score(3000)=0.831 I=0.948 C=0.729 ns_rows≤3K=18/51 grid(1000/1442/2080/3000/4327/6240/9000)=0.940/0.996/0.913/0.831/0.685/0.582/0.570

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 40 | 40 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 63 |  | 63 | README title and one-line identity | 1.1 |  | 0.000 |
| walker |  | 103 | 63 | Markdown::ReadmeHeadline { file: README.md } |  |  | 1.000 |
| ns | 103 |  | 40 | Complete repository root listing | 1.2 |  | 1.000 |
| walker |  | 118 | 15 | Fs::DirListing { dir: contribute } |  |  | 1.000 |
| walker |  | 126 | 8 | Fs::DirListing { dir: contribute/conf } |  |  | 1.000 |
| walker |  | 129 | 3 | Fs::DirListing { dir: .github } |  |  | 1.000 |
| ns | 133 |  | 30 | Complete internal/ package listing | 1.3 |  | 0.748 |
| walker |  | 137 | 8 | Fs::DirListing { dir: .github/workflows } |  |  | 0.749 |
| walker |  | 161 | 24 | Fs::DirListing { dir: sdk } |  |  | 0.753 |
| walker |  | 191 | 30 | Fs::DirListing { dir: internal } |  |  | 1.000 |
| walker |  | 199 | 8 | Fs::DirListing { dir: internal/session } |  |  | 1.000 |
| ns | 207 |  | 74 | Complete cmd/ and sdk/ listings | 1.4 |  | 0.777 |
| walker |  | 211 | 12 | Fs::DirListing { dir: internal/agent } |  |  | 0.778 |
| walker |  | 224 | 13 | Fs::DirListing { dir: internal/tokens } |  |  | 0.778 |
| ns | 260 |  | 53 | Module path, Go version and toolchain | 1.5 |  | 0.727 |
| walker |  | 277 | 53 | GoMod::Identity { file: go.mod } |  |  | 0.799 |
| walker |  | 313 | 36 | Fs::DirListing { dir: cmd } |  |  | 0.908 |
| walker |  | 330 | 17 | Fs::DirListing { dir: internal/auth } |  |  | 0.909 |
| walker |  | 336 | 6 | Fs::DirListing { dir: examples } |  |  | 0.910 |
| walker |  | 371 | 35 | Fs::DirListing { dir: internal/models } |  |  | 0.915 |
| walker |  | 376 | 5 | Fs::DirListing { dir: internal/models/anthropic } |  |  | 0.915 |
| walker |  | 381 | 5 | Fs::DirListing { dir: internal/models/gemini } |  |  | 0.916 |
| walker |  | 386 | 5 | Fs::DirListing { dir: internal/models/openai } |  |  | 0.917 |
| walker |  | 425 | 39 | Fs::DirListing { dir: internal/hooks } |  |  | 0.922 |
| ns | 427 |  | 167 | All README section headings (locations only) | 1.6 |  | 0.789 |
| walker |  | 435 | 10 | Fs::DirListing { dir: internal/hooks/testdata } |  |  | 0.790 |
| walker |  | 475 | 40 | Fs::DirListing { dir: internal/builtin } |  |  | 0.797 |
| walker |  | 481 | 6 | Fs::DirListing { dir: sdk/examples } |  |  | 0.836 |
| walker |  | 485 | 4 | Fs::DirListing { dir: sdk/examples/basic } |  |  | 0.857 |
| walker |  | 489 | 4 | Fs::DirListing { dir: sdk/examples/scripting } |  |  | 0.880 |
| walker |  | 534 | 45 | Fs::DirListing { dir: internal/config } |  |  | 0.828 |
| ns | 534 |  | 107 | README feature list, first half | 1.7 | 1.6 | 0.828 |
| walker |  | 620 | 86 | Fs::DirListing { dir: internal/ui } |  |  | 0.828 |
| walker |  | 625 | 5 | Fs::DirListing { dir: internal/ui/progress } |  |  | 0.828 |
| walker |  | 642 | 17 | Fs::DirListing { dir: examples/hooks } |  |  | 0.828 |
| ns | 648 |  | 114 | README feature list, second half | 1.8 |  | 0.784 |
| ns | 781 |  | 133 | Complete listings for the config / agent / tools / models packages | 1.9 |  | 0.783 |
| walker |  | 809 | 167 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.874 |
| walker |  | 831 | 22 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.874 |
| ns | 999 |  | 218 | Complete listings for the builtin / hooks / session / auth / tokens / ui packages | 1.10 |  | 0.893 |
| walker |  | 1047 | 216 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: true } |  |  | 0.962 |
| walker |  | 1082 | 35 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.962 |
| ns | 1089 |  | 90 | Complete listings for examples/, contribute/ and .github/ | 1.11 |  | 0.925 |
| walker |  | 1125 | 43 | Markdown::Section { file: README.md, section_index: 31, keeps_default_concavity: false } |  |  | 0.925 |
| walker |  | 1151 | 26 | Fs::DirListing { dir: internal/tools } |  |  | 0.961 |
| walker |  | 1178 | 27 | Code::CodeKey { rung: Names, file: main.go, decl: 0, sub: 0, line: 0 } |  |  | 0.961 |
| ns | 1243 |  | 154 | main.go entry point | 2.1 |  | 0.919 |
| walker |  | 1308 | 130 | Code::CodeKey { rung: Decl, file: main.go, decl: 2, sub: 0, line: 14 } |  |  | 0.963 |
| walker |  | 1341 | 33 | Fs::DirListing { dir: examples/scripts } |  |  | 0.996 |
| walker |  | 1433 | 92 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.996 |
| ns | 1504 |  | 261 | Complete cobra command tree: script, auth (login/logout/status), hooks (list/validate/init) | 2.2 |  | 0.944 |
| walker |  | 1631 | 198 | Code::CodeKey { rung: ModuleDoc, file: internal/tokens/anthropic.go, decl: 0, sub: 0, line: 0 } |  |  | 0.944 |
| walker |  | 1702 | 71 | Code::CodeKey { rung: Names, file: cmd/hooks.go, decl: 0, sub: 0, line: 0 } |  |  | 0.944 |
| walker |  | 1749 | 47 | Code::CodeKey { rung: Decl, file: cmd/hooks.go, decl: 1, sub: 0, line: 16 } |  |  | 0.945 |
| ns | 1754 |  | 250 | Persistent flag registration, part 1: config, system-prompt, model, debug, prompt, quiet | 2.3 |  | 0.909 |
| walker |  | 1934 | 185 | Code::CodeKey { rung: Decl, file: cmd/hooks.go, decl: 3, sub: 0, line: 57 } |  |  | 0.911 |
| ns | 2098 |  | 344 | Persistent flag registration, part 2: no-exit, max-steps, stream, compact, no-hooks, approve-tool-run, session flags | 2.4 |  | 0.866 |
| ns | 2224 |  | 126 | Provider and TLS flag registration | 2.5 |  | 0.857 |
| walker |  | 2254 | 320 | Code::CodeKey { rung: Decl, file: cmd/hooks.go, decl: 2, sub: 0, line: 25 } |  |  | 0.859 |
| walker |  | 2294 | 40 | Plaintext::DeclSurface { file: contribute/build.sh } |  |  | 0.859 |
| walker |  | 2305 | 11 | Plaintext::Whole { file: contribute/build.sh } |  |  | 0.859 |
| walker |  | 2465 | 160 | Code::CodeKey { rung: Names, file: cmd/auth.go, decl: 0, sub: 0, line: 0 } |  |  | 0.859 |
| ns | 2502 |  | 278 | Generation-parameter and Ollama flag registration, with the hidden flag | 2.6 |  | 0.838 |
| walker |  | 2584 | 119 | Code::CodeKey { rung: Decl, file: cmd/auth.go, decl: 4, sub: 0, line: 78 } |  |  | 0.841 |
| walker |  | 2745 | 161 | Code::CodeKey { rung: Decl, file: cmd/auth.go, decl: 1, sub: 0, line: 17 } |  |  | 0.846 |
| ns | 2749 |  | 247 | MCPServerConfig: complete field set including the legacy block | 3.1 |  | 0.817 |
| walker |  | 2912 | 167 | Code::CodeKey { rung: Decl, file: cmd/auth.go, decl: 3, sub: 0, line: 58 } |  |  | 0.823 |
| ns | 3069 |  | 320 | Config struct: application-level keys | 3.2 |  | 0.798 |
| walker |  | 3087 | 175 | Code::CodeKey { rung: Decl, file: cmd/auth.go, decl: 2, sub: 0, line: 37 } |  |  | 0.805 |
| walker |  | 3152 | 65 | Code::CodeKey { rung: Names, file: sdk/types.go, decl: 0, sub: 0, line: 0 } |  |  | 0.805 |
| walker |  | 3163 | 11 | Code::CodeKey { rung: Body, file: sdk/types.go, decl: 3, sub: 0, line: 18 } |  |  | 0.805 |
| walker |  | 3175 | 12 | Code::CodeKey { rung: Body, file: sdk/types.go, decl: 4, sub: 0, line: 24 } |  |  | 0.805 |
| ns | 3250 |  | 181 | Config struct: generation-parameter and TLS keys | 3.3 |  | 0.788 |
| ns | 3514 |  | 264 | GetTransportType: type-to-transport mapping and legacy inference | 3.4 |  | 0.748 |
| walker |  | 3689 | 514 | GoMod::File { file: go.mod } |  |  | 0.748 |
| walker |  | 3902 | 213 | Code::CodeKey { rung: Names, file: sdk/mcphost.go, decl: 0, sub: 0, line: 0 } |  |  | 0.748 |
| walker |  | 3937 | 35 | Code::CodeKey { rung: Decl, file: sdk/mcphost.go, decl: 1, sub: 0, line: 19 } |  |  | 0.748 |
| ns | 3938 |  | 424 | Config.Validate: required fields per transport and filter exclusivity | 3.5 |  | 0.711 |
| walker |  | 4005 | 68 | Code::CodeKey { rung: Decl, file: sdk/mcphost.go, decl: 5, sub: 0, line: 163 } |  |  | 0.711 |
| walker |  | 4112 | 107 | Code::CodeKey { rung: Decl, file: sdk/mcphost.go, decl: 2, sub: 0, line: 28 } |  |  | 0.712 |
| walker |  | 4121 | 9 | Code::CodeKey { rung: Body, file: sdk/mcphost.go, decl: 6, sub: 0, line: 201 } |  |  | 0.712 |
| walker |  | 4130 | 9 | Code::CodeKey { rung: Body, file: sdk/mcphost.go, decl: 10, sub: 0, line: 230 } |  |  | 0.712 |
| walker |  | 4139 | 9 | Code::CodeKey { rung: Body, file: sdk/mcphost.go, decl: 11, sub: 0, line: 237 } |  |  | 0.712 |
| ns | 4141 |  | 203 | Substitution engine: the two regexes plus every symbol in substitution.go | 3.6 |  | 0.699 |
| ns | 4295 |  | 154 | Remaining top-level symbols of internal/config/config.go and all of merger.go (locations) | 3.7 |  | 0.685 |
| walker |  | 4419 | 280 | Code::CodeKey { rung: Names, file: cmd/script.go, decl: 0, sub: 0, line: 0 } |  |  | 0.686 |
| walker |  | 4484 | 65 | Code::CodeKey { rung: Decl, file: cmd/script.go, decl: 9, sub: 0, line: 423 } |  |  | 0.686 |
| walker |  | 4495 | 11 | Code::CodeKey { rung: Body, file: cmd/script.go, decl: 2, sub: 0, line: 78 } |  |  | 0.686 |
| walker |  | 4511 | 16 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 4, sub: 0, line: 148 } |  |  | 0.686 |
| walker |  | 4527 | 16 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 7, sub: 0, line: 279 } |  |  | 0.686 |
| ns | 4541 |  | 246 | Agent struct and its seven callback handler types | 4.1 |  | 0.669 |
| walker |  | 4545 | 18 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 14, sub: 0, line: 511 } |  |  | 0.669 |
| walker |  | 4564 | 19 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 8, sub: 0, line: 288 } |  |  | 0.669 |
| walker |  | 4584 | 20 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 6, sub: 0, line: 255 } |  |  | 0.669 |
| ns | 4815 |  | 274 | Every top-level symbol of internal/agent/agent.go (locations) | 4.2 |  | 0.652 |
| ns | 5007 |  | 192 | The tool-calling loop: step bound, tool-call branch and the approval gate | 4.3 | 4.2 | 0.640 |
| ns | 5186 |  | 179 | agent factory: AgentCreationOptions fields and both functions | 4.4 |  | 0.629 |
| walker |  | 5214 | 630 | Code::CodeKey { rung: Decl, file: cmd/script.go, decl: 1, sub: 0, line: 28 } |  |  | 0.638 |
| walker |  | 5477 | 263 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.638 |
| walker |  | 5506 | 29 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 6, sub: 0, line: 201 } |  |  | 0.638 |
| walker |  | 5535 | 29 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 9, sub: 0, line: 224 } |  |  | 0.638 |
| ns | 5549 |  | 363 | Every top-level symbol of internal/tools/mcp.go (locations) | 4.5 |  | 0.620 |
| walker |  | 5596 | 61 | Code::CodeKey { rung: Names, file: internal/models/models_data.go, decl: 0, sub: 0, line: 0 } |  |  | 0.620 |
| walker |  | 5616 | 20 | Code::CodeKey { rung: Decl, file: internal/models/models_data.go, decl: 3, sub: 0, line: 26 } |  |  | 0.620 |
| walker |  | 5662 | 46 | Code::CodeKey { rung: Decl, file: internal/models/models_data.go, decl: 2, sub: 0, line: 18 } |  |  | 0.620 |
| walker |  | 5714 | 52 | Code::CodeKey { rung: Decl, file: internal/models/models_data.go, decl: 4, sub: 0, line: 32 } |  |  | 0.620 |
| ns | 5765 |  | 216 | AgenticLoopConfig head and every mode-driving function in cmd/root.go (locations) | 4.6 |  | 0.608 |
| walker |  | 5780 | 66 | Code::CodeKey { rung: Decl, file: internal/models/models_data.go, decl: 1, sub: 0, line: 7 } |  |  | 0.608 |
| walker |  | 5792 | 12 | Code::CodeKey { rung: Doc, file: internal/models/models_data.go, decl: 2, sub: 0, line: 18 } |  |  | 0.608 |
| walker |  | 5804 | 12 | Code::CodeKey { rung: Doc, file: internal/models/models_data.go, decl: 4, sub: 0, line: 32 } |  |  | 0.608 |
| walker |  | 5818 | 14 | Code::CodeKey { rung: Doc, file: internal/models/models_data.go, decl: 1, sub: 0, line: 7 } |  |  | 0.608 |
| walker |  | 5832 | 14 | Code::CodeKey { rung: Doc, file: internal/models/models_data.go, decl: 3, sub: 0, line: 26 } |  |  | 0.608 |
| walker |  | 5847 | 15 | Code::CodeKey { rung: Doc, file: internal/models/models_data.go, decl: 5, sub: 0, line: 41 } |  |  | 0.608 |
| ns | 5907 |  | 142 | CreateProvider: the complete list of supported providers | 5.1 |  | 0.599 |
| walker |  | 5933 | 86 | Code::CodeKey { rung: Names, file: internal/models/generate_models.go, decl: 0, sub: 0, line: 0 } |  |  | 0.599 |
| walker |  | 5990 | 57 | Code::CodeKey { rung: Decl, file: internal/models/generate_models.go, decl: 3, sub: 0, line: 50 } |  |  | 0.599 |
| walker |  | 6121 | 131 | Code::CodeKey { rung: Decl, file: internal/models/generate_models.go, decl: 2, sub: 0, line: 37 } |  |  | 0.599 |
| ns | 6235 |  | 328 | Every top-level symbol of internal/models/providers.go (locations) | 5.2 | 5.1 | 0.582 |
| walker |  | 6263 | 142 | Code::CodeKey { rung: Decl, file: internal/models/generate_models.go, decl: 4, sub: 0, line: 60 } |  |  | 0.582 |
| ns | 6385 |  | 150 | ModelsRegistry: model validation and suggestion API | 5.3 |  | 0.576 |
| walker |  | 6464 | 201 | Code::CodeKey { rung: Decl, file: internal/models/generate_models.go, decl: 1, sub: 0, line: 18 } |  |  | 0.576 |
| ns | 6477 |  | 92 | The generated model catalogue: generator types and the DO-NOT-EDIT header | 5.4 |  | 0.575 |
| walker |  | 6538 | 74 | Code::CodeKey { rung: Names, file: internal/hooks/schemas.go, decl: 0, sub: 0, line: 0 } |  |  | 0.575 |
| walker |  | 6562 | 24 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 4, sub: 0, line: 42 } |  |  | 0.575 |
| walker |  | 6606 | 44 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 2, sub: 0, line: 23 } |  |  | 0.575 |
| ns | 6653 |  | 176 | Builtin server registry: the complete set of in-process servers | 6.1 |  | 0.565 |
| walker |  | 6667 | 61 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 3, sub: 0, line: 32 } |  |  | 0.565 |
| walker |  | 6756 | 89 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 6, sub: 0, line: 62 } |  |  | 0.565 |
| ns | 6825 |  | 172 | Every top-level symbol of internal/builtin/registry.go (locations) | 6.2 | 6.1 | 0.558 |
| walker |  | 6863 | 107 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 5, sub: 0, line: 50 } |  |  | 0.559 |
| walker |  | 7022 | 159 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 1, sub: 0, line: 10 } |  |  | 0.560 |
| ns | 7042 |  | 217 | Bash builtin: output/timeout limits and the complete banned-command list | 6.3 |  | 0.547 |
| walker |  | 7084 | 62 | Plaintext::DeclSurface { file: contribute/boost.sh } |  |  | 0.547 |
| walker |  | 7115 | 31 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 12, sub: 0, line: 480 } |  |  | 0.547 |
| walker |  | 7147 | 32 | Code::CodeKey { rung: Doc, file: sdk/types.go, decl: 3, sub: 0, line: 18 } |  |  | 0.547 |
| ns | 7240 |  | 198 | The http builtin: its four tools and every symbol in http.go (locations) | 6.4 |  | 0.538 |
| ns | 7352 |  | 112 | HookEvent: the complete set of hook events | 7.1 |  | 0.534 |
| walker |  | 7432 | 285 | Code::CodeKey { rung: Names, file: cmd/root.go, decl: 0, sub: 0, line: 0 } |  |  | 0.534 |
| walker |  | 7506 | 74 | Code::CodeKey { rung: Decl, file: cmd/root.go, decl: 1, sub: 0, line: 27 } |  |  | 0.534 |
| ns | 7567 |  | 215 | Hook configuration schema: HookConfig, HookMatcher, HookEntry | 7.2 |  | 0.527 |
| walker |  | 7702 | 196 | Code::CodeKey { rung: Names, file: cmd/root.go, decl: 0, sub: 1, line: 0 } |  |  | 0.527 |
| walker |  | 7715 | 13 | Code::CodeKey { rung: Decl, file: cmd/root.go, decl: 2, sub: 0, line: 67 } |  |  | 0.527 |
| walker |  | 7725 | 10 | Code::CodeKey { rung: Body, file: cmd/root.go, decl: 12, sub: 0, line: 361 } |  |  | 0.527 |
| walker |  | 7740 | 15 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 2, sub: 0, line: 67 } |  |  | 0.527 |
| ns | 7839 |  | 272 | Hook wire protocol: CommonInput and HookOutput | 7.3 |  | 0.539 |
| walker |  | 7928 | 188 | Code::CodeKey { rung: Names, file: cmd/root.go, decl: 0, sub: 2, line: 0 } |  |  | 0.542 |
| ns | 8030 |  | 191 | Per-event hook input structs | 7.4 |  | 0.550 |
| walker |  | 8144 | 216 | Code::CodeKey { rung: Decl, file: cmd/root.go, decl: 14, sub: 0, line: 758 } |  |  | 0.561 |
| ns | 8336 |  | 306 | Hook executor and validator symbol rosters | 7.5 |  | 0.551 |
| walker |  | 8354 | 210 | Code::CodeKey { rung: Names, file: cmd/root.go, decl: 0, sub: 3, line: 0 } |  |  | 0.559 |
| walker |  | 8369 | 15 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 22, sub: 0, line: 1349 } |  |  | 0.559 |
| walker |  | 8387 | 18 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 21, sub: 0, line: 1316 } |  |  | 0.559 |
| walker |  | 8406 | 19 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 16, sub: 0, line: 794 } |  |  | 0.559 |
| walker |  | 8425 | 19 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 17, sub: 0, line: 811 } |  |  | 0.559 |
| walker |  | 8444 | 19 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 20, sub: 0, line: 1229 } |  |  | 0.559 |
| walker |  | 8464 | 20 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 15, sub: 0, line: 777 } |  |  | 0.559 |
| walker |  | 8484 | 20 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 19, sub: 0, line: 1195 } |  |  | 0.559 |
| ns | 8493 |  | 157 | README: hooks.yml file locations and the --no-hooks escape hatch | 7.6 | 1.6 | 0.554 |
| ns | 8682 |  | 189 | Script mode: a worked frontmatter example and the variable rules | 8.1 |  | 0.563 |
| ns | 8813 |  | 131 | Every top-level symbol of cmd/script.go (locations) | 8.2 |  | 0.570 |
| walker |  | 8954 | 470 | Code::CodeKey { rung: Decl, file: cmd/root.go, decl: 6, sub: 0, line: 92 } |  |  | 0.570 |
| walker |  | 8977 | 23 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 18, sub: 0, line: 879 } |  |  | 0.570 |
| walker |  | 9010 | 33 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 10, sub: 0, line: 431 } |  |  | 0.570 |
| walker |  | 9043 | 33 | Code::CodeKey { rung: Doc, file: sdk/types.go, decl: 1, sub: 0, line: 10 } |  |  | 0.570 |
| walker |  | 9077 | 34 | Code::CodeKey { rung: Doc, file: sdk/types.go, decl: 2, sub: 0, line: 14 } |  |  | 0.570 |
| walker |  | 9111 | 34 | Code::CodeKey { rung: Doc, file: sdk/types.go, decl: 4, sub: 0, line: 24 } |  |  | 0.561 |
| ns | 9111 |  | 298 | Session file format: Session, Metadata, Message and ToolCall fields | 8.3 |  | 0.561 |
| walker |  | 9139 | 28 | Code::CodeKey { rung: Names, file: internal/tokens/init.go, decl: 0, sub: 0, line: 0 } |  |  | 0.561 |
| walker |  | 9154 | 15 | Code::CodeKey { rung: Body, file: internal/tokens/init.go, decl: 1, sub: 0, line: 21 } |  |  | 0.561 |
| walker |  | 9169 | 15 | Code::CodeKey { rung: Body, file: internal/tokens/init.go, decl: 2, sub: 0, line: 50 } |  |  | 0.561 |
| walker |  | 9360 | 191 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.561 |
| ns | 9405 |  | 294 | The public SDK surface: Options and every exported symbol | 8.4 |  | 0.572 |
| walker |  | 9448 | 88 | Code::CodeKey { rung: Names, file: internal/hooks/config.go, decl: 0, sub: 0, line: 0 } |  |  | 0.573 |
| walker |  | 9475 | 27 | Code::CodeKey { rung: Decl, file: internal/hooks/config.go, decl: 1, sub: 0, line: 14 } |  |  | 0.575 |
| walker |  | 9530 | 55 | Code::CodeKey { rung: Decl, file: internal/hooks/config.go, decl: 3, sub: 0, line: 30 } |  |  | 0.580 |
| walker |  | 9595 | 65 | Code::CodeKey { rung: Decl, file: internal/hooks/config.go, decl: 2, sub: 0, line: 21 } |  |  | 0.588 |
| ns | 9607 |  | 202 | The complete slash-command table: names and descriptions | 8.5 |  | 0.582 |
| walker |  | 9609 | 14 | Code::CodeKey { rung: Doc, file: internal/hooks/config.go, decl: 6, sub: 0, line: 113 } |  |  | 0.582 |
| walker |  | 9645 | 36 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 11, sub: 0, line: 442 } |  |  | 0.582 |
| walker |  | 9675 | 30 | Code::CodeKey { rung: Names, file: internal/auth/browser.go, decl: 0, sub: 0, line: 0 } |  |  | 0.582 |
| walker |  | 9735 | 60 | Code::CodeKey { rung: Names, file: internal/ui/commands.go, decl: 0, sub: 0, line: 0 } |  |  | 0.582 |
| walker |  | 9786 | 51 | Code::CodeKey { rung: Decl, file: internal/ui/commands.go, decl: 1, sub: 0, line: 6 } |  |  | 0.582 |
| walker |  | 9805 | 19 | Code::CodeKey { rung: Doc, file: internal/hooks/config.go, decl: 5, sub: 0, line: 97 } |  |  | 0.582 |
| ns | 9828 |  | 221 | ui.SetupCLI: the AgentInterface contract and CLISetupOptions | 8.6 |  | 0.573 |
| walker |  | 9842 | 37 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 8, sub: 0, line: 218 } |  |  | 0.573 |
| ns | 9984 |  | 156 | Release packaging: the goreleaser build matrix | 9.1 |  | 0.567 |
| walker |  | 9991 | 149 | Code::CodeKey { rung: Names, file: internal/config/config.go, decl: 0, sub: 0, line: 0 } |  |  | 0.570 |

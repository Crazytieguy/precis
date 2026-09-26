Score(3000)=0.809 I=0.940 C=0.697 ns_rows≤3K=18/51 grid(1000/1442/2080/3000/4327/6240/9000)=0.924/0.983/0.897/0.809/0.676/0.610/0.564

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
| walker |  | 464 | 40 | Fs::DirListing { dir: internal/builtin } |  |  | 0.795 |
| walker |  | 470 | 6 | Fs::DirListing { dir: sdk/examples } |  |  | 0.834 |
| walker |  | 474 | 4 | Fs::DirListing { dir: sdk/examples/basic } |  |  | 0.856 |
| walker |  | 478 | 4 | Fs::DirListing { dir: sdk/examples/scripting } |  |  | 0.878 |
| walker |  | 523 | 45 | Fs::DirListing { dir: internal/config } |  |  | 0.880 |
| ns | 534 |  | 107 | README feature list, first half | 1.7 | 1.6 | 0.828 |
| walker |  | 609 | 86 | Fs::DirListing { dir: internal/ui } |  |  | 0.828 |
| walker |  | 614 | 5 | Fs::DirListing { dir: internal/ui/progress } |  |  | 0.828 |
| walker |  | 631 | 17 | Fs::DirListing { dir: examples/hooks } |  |  | 0.828 |
| ns | 648 |  | 114 | README feature list, second half | 1.8 |  | 0.784 |
| ns | 781 |  | 133 | Complete listings for the config / agent / tools / models packages | 1.9 |  | 0.781 |
| walker |  | 798 | 167 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.874 |
| walker |  | 798 | 0 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.874 |
| walker |  | 820 | 22 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.874 |
| ns | 999 |  | 218 | Complete listings for the builtin / hooks / session / auth / tokens / ui packages | 1.10 |  | 0.877 |
| walker |  | 1036 | 216 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: true } |  |  | 0.946 |
| walker |  | 1071 | 35 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.946 |
| ns | 1088 |  | 89 | Complete listings for examples/, contribute/ and .github/ | 1.11 |  | 0.911 |
| walker |  | 1114 | 43 | Markdown::Section { file: README.md, section_index: 31, keeps_default_concavity: false } |  |  | 0.911 |
| walker |  | 1140 | 26 | Fs::DirListing { dir: internal/tools } |  |  | 0.947 |
| walker |  | 1167 | 27 | Code::CodeKey { rung: Names, file: main.go, decl: 0, sub: 0, line: 0 } |  |  | 0.947 |
| ns | 1242 |  | 154 | main.go entry point | 2.1 |  | 0.906 |
| walker |  | 1297 | 130 | Code::CodeKey { rung: Decl, file: main.go, decl: 2, sub: 0, line: 14 } |  |  | 0.950 |
| walker |  | 1330 | 33 | Fs::DirListing { dir: examples/scripts } |  |  | 0.983 |
| walker |  | 1422 | 92 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.983 |
| ns | 1503 |  | 261 | Complete cobra command tree: script, auth (login/logout/status), hooks (list/validate/init) | 2.2 |  | 0.932 |
| walker |  | 1620 | 198 | Code::CodeKey { rung: ModuleDoc, file: internal/tokens/anthropic.go, decl: 0, sub: 0, line: 0 } |  |  | 0.932 |
| walker |  | 1660 | 40 | Plaintext::DeclSurface { file: contribute/build.sh } |  |  | 0.932 |
| walker |  | 1671 | 11 | Plaintext::Whole { file: contribute/build.sh } |  |  | 0.932 |
| walker |  | 1733 | 62 | Code::CodeKey { rung: Names, file: cmd/root.go, decl: 0, sub: 0, line: 0 } |  |  | 0.932 |
| ns | 1753 |  | 250 | Persistent flag registration, part 1: config, system-prompt, model, debug, prompt, quiet | 2.3 |  | 0.897 |
| walker |  | 1949 | 216 | Code::CodeKey { rung: Decl, file: cmd/root.go, decl: 4, sub: 0, line: 758 } |  |  | 0.897 |
| walker |  | 1961 | 12 | Code::CodeKey { rung: Names, file: cmd/script.go, decl: 0, sub: 0, line: 0 } |  |  | 0.897 |
| walker |  | 2026 | 65 | Code::CodeKey { rung: Decl, file: cmd/script.go, decl: 1, sub: 0, line: 423 } |  |  | 0.897 |
| ns | 2097 |  | 344 | Persistent flag registration, part 2: no-exit, max-steps, stream, compact, no-hooks, approve-tool-run, session flags | 2.4 |  | 0.853 |
| walker |  | 2186 | 160 | Code::CodeKey { rung: Names, file: cmd/auth.go, decl: 0, sub: 0, line: 0 } |  |  | 0.853 |
| ns | 2223 |  | 126 | Provider and TLS flag registration | 2.5 |  | 0.845 |
| walker |  | 2305 | 119 | Code::CodeKey { rung: Decl, file: cmd/auth.go, decl: 4, sub: 0, line: 78 } |  |  | 0.846 |
| walker |  | 2466 | 161 | Code::CodeKey { rung: Decl, file: cmd/auth.go, decl: 1, sub: 0, line: 17 } |  |  | 0.847 |
| ns | 2501 |  | 278 | Generation-parameter and Ollama flag registration, with the hidden flag | 2.6 |  | 0.826 |
| walker |  | 2633 | 167 | Code::CodeKey { rung: Decl, file: cmd/auth.go, decl: 3, sub: 0, line: 58 } |  |  | 0.829 |
| ns | 2748 |  | 247 | MCPServerConfig: complete field set including the legacy block | 3.1 |  | 0.801 |
| walker |  | 2808 | 175 | Code::CodeKey { rung: Decl, file: cmd/auth.go, decl: 2, sub: 0, line: 37 } |  |  | 0.805 |
| walker |  | 2879 | 71 | Code::CodeKey { rung: Names, file: cmd/hooks.go, decl: 0, sub: 0, line: 0 } |  |  | 0.805 |
| walker |  | 2926 | 47 | Code::CodeKey { rung: Decl, file: cmd/hooks.go, decl: 1, sub: 0, line: 16 } |  |  | 0.809 |
| ns | 3068 |  | 320 | Config struct: application-level keys | 3.2 |  | 0.785 |
| walker |  | 3111 | 185 | Code::CodeKey { rung: Decl, file: cmd/hooks.go, decl: 3, sub: 0, line: 57 } |  |  | 0.790 |
| ns | 3249 |  | 181 | Config struct: generation-parameter and TLS keys | 3.3 |  | 0.773 |
| walker |  | 3431 | 320 | Code::CodeKey { rung: Decl, file: cmd/hooks.go, decl: 2, sub: 0, line: 25 } |  |  | 0.779 |
| ns | 3513 |  | 264 | GetTransportType: type-to-transport mapping and legacy inference | 3.4 |  | 0.739 |
| ns | 3937 |  | 424 | Config.Validate: required fields per transport and filter exclusivity | 3.5 |  | 0.702 |
| walker |  | 3945 | 514 | GoMod::File { file: go.mod } |  |  | 0.702 |
| ns | 4140 |  | 203 | Substitution engine: the two regexes plus every symbol in substitution.go | 3.6 |  | 0.689 |
| walker |  | 4158 | 213 | Code::CodeKey { rung: Names, file: sdk/mcphost.go, decl: 0, sub: 0, line: 0 } |  |  | 0.689 |
| walker |  | 4193 | 35 | Code::CodeKey { rung: Decl, file: sdk/mcphost.go, decl: 1, sub: 0, line: 19 } |  |  | 0.689 |
| walker |  | 4261 | 68 | Code::CodeKey { rung: Decl, file: sdk/mcphost.go, decl: 5, sub: 0, line: 163 } |  |  | 0.689 |
| ns | 4294 |  | 154 | Remaining top-level symbols of internal/config/config.go and all of merger.go (locations) | 3.7 |  | 0.676 |
| walker |  | 4368 | 107 | Code::CodeKey { rung: Decl, file: sdk/mcphost.go, decl: 2, sub: 0, line: 28 } |  |  | 0.676 |
| walker |  | 4433 | 65 | Code::CodeKey { rung: Names, file: sdk/types.go, decl: 0, sub: 0, line: 0 } |  |  | 0.677 |
| walker |  | 4444 | 11 | Code::CodeKey { rung: Body, file: sdk/types.go, decl: 3, sub: 0, line: 18 } |  |  | 0.677 |
| walker |  | 4456 | 12 | Code::CodeKey { rung: Body, file: sdk/types.go, decl: 4, sub: 0, line: 24 } |  |  | 0.677 |
| walker |  | 4465 | 9 | Code::CodeKey { rung: Body, file: sdk/mcphost.go, decl: 6, sub: 0, line: 201 } |  |  | 0.677 |
| ns | 4540 |  | 246 | Agent struct and its seven callback handler types | 4.1 |  | 0.660 |
| walker |  | 4728 | 263 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.660 |
| walker |  | 4737 | 9 | Code::CodeKey { rung: Body, file: sdk/mcphost.go, decl: 10, sub: 0, line: 230 } |  |  | 0.660 |
| walker |  | 4746 | 9 | Code::CodeKey { rung: Body, file: sdk/mcphost.go, decl: 11, sub: 0, line: 237 } |  |  | 0.660 |
| walker |  | 4808 | 62 | Plaintext::DeclSurface { file: contribute/boost.sh } |  |  | 0.660 |
| ns | 4814 |  | 274 | Every top-level symbol of internal/agent/agent.go (locations) | 4.2 |  | 0.643 |
| walker |  | 4837 | 29 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 6, sub: 0, line: 201 } |  |  | 0.643 |
| walker |  | 4866 | 29 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 9, sub: 0, line: 224 } |  |  | 0.643 |
| walker |  | 4894 | 28 | Code::CodeKey { rung: Names, file: internal/tokens/init.go, decl: 0, sub: 0, line: 0 } |  |  | 0.643 |
| ns | 5006 |  | 192 | The tool-calling loop: step bound, tool-call branch and the approval gate | 4.3 | 4.2 | 0.632 |
| walker |  | 5085 | 191 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.632 |
| walker |  | 5100 | 15 | Code::CodeKey { rung: Body, file: internal/tokens/init.go, decl: 1, sub: 0, line: 21 } |  |  | 0.632 |
| walker |  | 5132 | 32 | Code::CodeKey { rung: Doc, file: sdk/types.go, decl: 3, sub: 0, line: 18 } |  |  | 0.632 |
| ns | 5185 |  | 179 | agent factory: AgentCreationOptions fields and both functions | 4.4 |  | 0.621 |
| walker |  | 5317 | 185 | Code::CodeKey { rung: Names, file: internal/config/config.go, decl: 0, sub: 0, line: 0 } |  |  | 0.630 |
| walker |  | 5359 | 42 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 3, sub: 0, line: 109 } |  |  | 0.630 |
| ns | 5548 |  | 363 | Every top-level symbol of internal/tools/mcp.go (locations) | 4.5 |  | 0.613 |
| walker |  | 5578 | 219 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 5, sub: 0, line: 137 } |  |  | 0.613 |
| ns | 5764 |  | 216 | AgenticLoopConfig head and every mode-driving function in cmd/root.go (locations) | 4.6 |  | 0.607 |
| walker |  | 5877 | 299 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 4, sub: 0, line: 116 } |  |  | 0.607 |
| ns | 5906 |  | 142 | CreateProvider: the complete list of supported providers | 5.1 |  | 0.598 |
| walker |  | 6110 | 233 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 1, sub: 0, line: 17 } |  |  | 0.625 |
| ns | 6234 |  | 328 | Every top-level symbol of internal/models/providers.go (locations) | 5.2 | 5.1 | 0.607 |
| walker |  | 6269 | 159 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 6, sub: 0, line: 155 } |  |  | 0.614 |
| ns | 6384 |  | 150 | ModelsRegistry: model validation and suggestion API | 5.3 |  | 0.607 |
| walker |  | 6420 | 151 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 6, sub: 1, line: 155 } |  |  | 0.623 |
| ns | 6476 |  | 92 | The generated model catalogue: generator types and the DO-NOT-EDIT header | 5.4 |  | 0.619 |
| walker |  | 6596 | 176 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 6, sub: 2, line: 155 } |  |  | 0.634 |
| walker |  | 6611 | 15 | Code::CodeKey { rung: Body, file: internal/tokens/init.go, decl: 2, sub: 0, line: 50 } |  |  | 0.634 |
| ns | 6652 |  | 176 | Builtin server registry: the complete set of in-process servers | 6.1 |  | 0.622 |
| walker |  | 6736 | 125 | Code::CodeKey { rung: Names, file: internal/hooks/events.go, decl: 0, sub: 0, line: 0 } |  |  | 0.623 |
| ns | 6824 |  | 172 | Every top-level symbol of internal/builtin/registry.go (locations) | 6.2 | 6.1 | 0.616 |
| walker |  | 6829 | 93 | Code::CodeKey { rung: Decl, file: internal/hooks/events.go, decl: 2, sub: 0, line: 7 } |  |  | 0.616 |
| walker |  | 6886 | 57 | Code::CodeKey { rung: Names, file: internal/hooks/config.go, decl: 0, sub: 0, line: 0 } |  |  | 0.616 |
| walker |  | 6913 | 27 | Code::CodeKey { rung: Decl, file: internal/hooks/config.go, decl: 1, sub: 0, line: 14 } |  |  | 0.616 |
| walker |  | 6968 | 55 | Code::CodeKey { rung: Decl, file: internal/hooks/config.go, decl: 3, sub: 0, line: 30 } |  |  | 0.616 |
| walker |  | 7033 | 65 | Code::CodeKey { rung: Decl, file: internal/hooks/config.go, decl: 2, sub: 0, line: 21 } |  |  | 0.617 |
| ns | 7041 |  | 217 | Bash builtin: output/timeout limits and the complete banned-command list | 6.3 |  | 0.602 |
| walker |  | 7205 | 172 | Code::CodeKey { rung: Names, file: internal/session/session.go, decl: 0, sub: 0, line: 0 } |  |  | 0.602 |
| ns | 7239 |  | 198 | The http builtin: its four tools and every symbol in http.go (locations) | 6.4 |  | 0.593 |
| walker |  | 7300 | 95 | Code::CodeKey { rung: Decl, file: internal/session/session.go, decl: 4, sub: 0, line: 66 } |  |  | 0.593 |
| ns | 7351 |  | 112 | HookEvent: the complete set of hook events | 7.1 |  | 0.598 |
| walker |  | 7411 | 111 | Code::CodeKey { rung: Decl, file: internal/session/session.go, decl: 2, sub: 0, line: 36 } |  |  | 0.599 |
| walker |  | 7565 | 154 | Code::CodeKey { rung: Decl, file: internal/session/session.go, decl: 1, sub: 0, line: 19 } |  |  | 0.599 |
| ns | 7566 |  | 215 | Hook configuration schema: HookConfig, HookMatcher, HookEntry | 7.2 |  | 0.605 |
| walker |  | 7767 | 202 | Code::CodeKey { rung: Decl, file: internal/session/session.go, decl: 3, sub: 0, line: 48 } |  |  | 0.606 |
| walker |  | 7800 | 33 | Code::CodeKey { rung: Doc, file: sdk/types.go, decl: 1, sub: 0, line: 10 } |  |  | 0.606 |
| ns | 7838 |  | 272 | Hook wire protocol: CommonInput and HookOutput | 7.3 |  | 0.597 |
| walker |  | 7895 | 95 | Code::CodeKey { rung: Names, file: internal/models/providers.go, decl: 0, sub: 0, line: 0 } |  |  | 0.598 |
| walker |  | 7955 | 60 | Code::CodeKey { rung: Decl, file: internal/models/providers.go, decl: 1, sub: 0, line: 28 } |  |  | 0.598 |
| ns | 8029 |  | 191 | Per-event hook input structs | 7.4 |  | 0.592 |
| walker |  | 8051 | 96 | Code::CodeKey { rung: Decl, file: internal/models/providers.go, decl: 3, sub: 0, line: 124 } |  |  | 0.592 |
| walker |  | 8166 | 115 | Code::CodeKey { rung: Decl, file: internal/models/providers.go, decl: 5, sub: 0, line: 539 } |  |  | 0.592 |
| ns | 8335 |  | 306 | Hook executor and validator symbol rosters | 7.5 |  | 0.581 |
| walker |  | 8410 | 244 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.581 |
| ns | 8492 |  | 157 | README: hooks.yml file locations and the --no-hooks escape hatch | 7.6 | 1.6 | 0.576 |
| walker |  | 8639 | 229 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.576 |
| walker |  | 8655 | 16 | Code::CodeKey { rung: Names, file: internal/tokens/counter.go, decl: 0, sub: 0, line: 0 } |  |  | 0.576 |
| ns | 8681 |  | 189 | Script mode: a worked frontmatter example and the variable rules | 8.1 |  | 0.570 |
| walker |  | 8689 | 34 | Code::CodeKey { rung: Doc, file: sdk/types.go, decl: 2, sub: 0, line: 14 } |  |  | 0.570 |
| walker |  | 8723 | 34 | Code::CodeKey { rung: Doc, file: sdk/types.go, decl: 4, sub: 0, line: 24 } |  |  | 0.570 |
| ns | 8812 |  | 131 | Every top-level symbol of cmd/script.go (locations) | 8.2 |  | 0.564 |
| walker |  | 8925 | 202 | Code::CodeKey { rung: Decl, file: internal/models/providers.go, decl: 2, sub: 0, line: 70 } |  |  | 0.564 |
| walker |  | 8943 | 18 | Code::CodeKey { rung: Body, file: cmd/root.go, decl: 1, sub: 0, line: 131 } |  |  | 0.564 |
| walker |  | 8980 | 37 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 8, sub: 0, line: 218 } |  |  | 0.564 |
| ns | 9110 |  | 298 | Session file format: Session, Metadata, Message and ToolCall fields | 8.3 |  | 0.577 |
| walker |  | 9290 | 310 | Code::CodeKey { rung: Names, file: internal/agent/agent.go, decl: 0, sub: 0, line: 0 } |  |  | 0.586 |
| walker |  | 9350 | 60 | Code::CodeKey { rung: Decl, file: internal/agent/agent.go, decl: 12, sub: 0, line: 135 } |  |  | 0.586 |
| ns | 9404 |  | 294 | The public SDK surface: Options and every exported symbol | 8.4 |  | 0.596 |
| walker |  | 9417 | 67 | Code::CodeKey { rung: Decl, file: internal/agent/agent.go, decl: 13, sub: 0, line: 144 } |  |  | 0.596 |
| walker |  | 9493 | 76 | Code::CodeKey { rung: Decl, file: internal/agent/agent.go, decl: 11, sub: 0, line: 125 } |  |  | 0.596 |
| walker |  | 9596 | 103 | Code::CodeKey { rung: Decl, file: internal/agent/agent.go, decl: 9, sub: 0, line: 67 } |  |  | 0.608 |
| ns | 9606 |  | 202 | The complete slash-command table: names and descriptions | 8.5 |  | 0.601 |
| walker |  | 9760 | 164 | Code::CodeKey { rung: Decl, file: internal/agent/agent.go, decl: 1, sub: 0, line: 22 } |  |  | 0.601 |
| ns | 9827 |  | 221 | ui.SetupCLI: the AgentInterface contract and CLISetupOptions | 8.6 |  | 0.593 |
| walker |  | 9983 | 223 | Code::CodeKey { rung: Names, file: internal/session/manager.go, decl: 0, sub: 0, line: 0 } |  |  | 0.586 |
| ns | 9983 |  | 156 | Release packaging: the goreleaser build matrix | 9.1 |  | 0.586 |
| walker |  | 10000 | 17 | Code::CodeKey { rung: Decl, file: internal/session/manager.go, decl: 1, sub: 0, line: 15 } |  |  | 0.586 |

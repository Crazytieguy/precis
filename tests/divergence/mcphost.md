Score(3000)=0.820 I=0.945 C=0.711 ns_rows≤3K=18/51 grid(1000/1442/2080/3000/4327/6240/9000)=0.940/0.996/0.909/0.820/0.684/0.600/0.570

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
| walker |  | 1670 | 40 | Plaintext::DeclSurface { file: contribute/build.sh } |  |  | 0.944 |
| walker |  | 1681 | 11 | Plaintext::Whole { file: contribute/build.sh } |  |  | 0.944 |
| walker |  | 1743 | 62 | Code::CodeKey { rung: Names, file: cmd/root.go, decl: 0, sub: 0, line: 0 } |  |  | 0.944 |
| ns | 1753 |  | 250 | Persistent flag registration, part 1: config, system-prompt, model, debug, prompt, quiet | 2.3 |  | 0.909 |
| walker |  | 1959 | 216 | Code::CodeKey { rung: Decl, file: cmd/root.go, decl: 4, sub: 0, line: 758 } |  |  | 0.909 |
| walker |  | 1971 | 12 | Code::CodeKey { rung: Names, file: cmd/script.go, decl: 0, sub: 0, line: 0 } |  |  | 0.909 |
| walker |  | 2036 | 65 | Code::CodeKey { rung: Decl, file: cmd/script.go, decl: 1, sub: 0, line: 423 } |  |  | 0.909 |
| ns | 2097 |  | 344 | Persistent flag registration, part 2: no-exit, max-steps, stream, compact, no-hooks, approve-tool-run, session flags | 2.4 |  | 0.864 |
| walker |  | 2196 | 160 | Code::CodeKey { rung: Names, file: cmd/auth.go, decl: 0, sub: 0, line: 0 } |  |  | 0.864 |
| ns | 2223 |  | 126 | Provider and TLS flag registration | 2.5 |  | 0.856 |
| walker |  | 2315 | 119 | Code::CodeKey { rung: Decl, file: cmd/auth.go, decl: 4, sub: 0, line: 78 } |  |  | 0.857 |
| walker |  | 2476 | 161 | Code::CodeKey { rung: Decl, file: cmd/auth.go, decl: 1, sub: 0, line: 17 } |  |  | 0.859 |
| ns | 2501 |  | 278 | Generation-parameter and Ollama flag registration, with the hidden flag | 2.6 |  | 0.837 |
| walker |  | 2643 | 167 | Code::CodeKey { rung: Decl, file: cmd/auth.go, decl: 3, sub: 0, line: 58 } |  |  | 0.840 |
| ns | 2748 |  | 247 | MCPServerConfig: complete field set including the legacy block | 3.1 |  | 0.811 |
| walker |  | 2818 | 175 | Code::CodeKey { rung: Decl, file: cmd/auth.go, decl: 2, sub: 0, line: 37 } |  |  | 0.816 |
| walker |  | 2889 | 71 | Code::CodeKey { rung: Names, file: cmd/hooks.go, decl: 0, sub: 0, line: 0 } |  |  | 0.816 |
| walker |  | 2936 | 47 | Code::CodeKey { rung: Decl, file: cmd/hooks.go, decl: 1, sub: 0, line: 16 } |  |  | 0.820 |
| ns | 3068 |  | 320 | Config struct: application-level keys | 3.2 |  | 0.795 |
| walker |  | 3121 | 185 | Code::CodeKey { rung: Decl, file: cmd/hooks.go, decl: 3, sub: 0, line: 57 } |  |  | 0.800 |
| ns | 3249 |  | 181 | Config struct: generation-parameter and TLS keys | 3.3 |  | 0.783 |
| walker |  | 3441 | 320 | Code::CodeKey { rung: Decl, file: cmd/hooks.go, decl: 2, sub: 0, line: 25 } |  |  | 0.789 |
| ns | 3513 |  | 264 | GetTransportType: type-to-transport mapping and legacy inference | 3.4 |  | 0.748 |
| ns | 3937 |  | 424 | Config.Validate: required fields per transport and filter exclusivity | 3.5 |  | 0.711 |
| walker |  | 3955 | 514 | GoMod::File { file: go.mod } |  |  | 0.711 |
| ns | 4140 |  | 203 | Substitution engine: the two regexes plus every symbol in substitution.go | 3.6 |  | 0.698 |
| walker |  | 4168 | 213 | Code::CodeKey { rung: Names, file: sdk/mcphost.go, decl: 0, sub: 0, line: 0 } |  |  | 0.698 |
| walker |  | 4203 | 35 | Code::CodeKey { rung: Decl, file: sdk/mcphost.go, decl: 1, sub: 0, line: 19 } |  |  | 0.698 |
| walker |  | 4271 | 68 | Code::CodeKey { rung: Decl, file: sdk/mcphost.go, decl: 5, sub: 0, line: 163 } |  |  | 0.698 |
| ns | 4294 |  | 154 | Remaining top-level symbols of internal/config/config.go and all of merger.go (locations) | 3.7 |  | 0.684 |
| walker |  | 4378 | 107 | Code::CodeKey { rung: Decl, file: sdk/mcphost.go, decl: 2, sub: 0, line: 28 } |  |  | 0.685 |
| walker |  | 4443 | 65 | Code::CodeKey { rung: Names, file: sdk/types.go, decl: 0, sub: 0, line: 0 } |  |  | 0.685 |
| walker |  | 4454 | 11 | Code::CodeKey { rung: Body, file: sdk/types.go, decl: 3, sub: 0, line: 18 } |  |  | 0.685 |
| walker |  | 4466 | 12 | Code::CodeKey { rung: Body, file: sdk/types.go, decl: 4, sub: 0, line: 24 } |  |  | 0.685 |
| walker |  | 4475 | 9 | Code::CodeKey { rung: Body, file: sdk/mcphost.go, decl: 6, sub: 0, line: 201 } |  |  | 0.685 |
| walker |  | 4484 | 9 | Code::CodeKey { rung: Body, file: sdk/mcphost.go, decl: 10, sub: 0, line: 230 } |  |  | 0.685 |
| walker |  | 4493 | 9 | Code::CodeKey { rung: Body, file: sdk/mcphost.go, decl: 11, sub: 0, line: 237 } |  |  | 0.685 |
| ns | 4540 |  | 246 | Agent struct and its seven callback handler types | 4.1 |  | 0.668 |
| walker |  | 4756 | 263 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.668 |
| walker |  | 4785 | 29 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 6, sub: 0, line: 201 } |  |  | 0.668 |
| walker |  | 4814 | 29 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 9, sub: 0, line: 224 } |  |  | 0.651 |
| ns | 4814 |  | 274 | Every top-level symbol of internal/agent/agent.go (locations) | 4.2 |  | 0.651 |
| walker |  | 4876 | 62 | Plaintext::DeclSurface { file: contribute/boost.sh } |  |  | 0.651 |
| walker |  | 4908 | 32 | Code::CodeKey { rung: Doc, file: sdk/types.go, decl: 3, sub: 0, line: 18 } |  |  | 0.651 |
| walker |  | 4941 | 33 | Code::CodeKey { rung: Doc, file: sdk/types.go, decl: 1, sub: 0, line: 10 } |  |  | 0.651 |
| walker |  | 4975 | 34 | Code::CodeKey { rung: Doc, file: sdk/types.go, decl: 2, sub: 0, line: 14 } |  |  | 0.651 |
| ns | 5006 |  | 192 | The tool-calling loop: step bound, tool-call branch and the approval gate | 4.3 | 4.2 | 0.640 |
| walker |  | 5009 | 34 | Code::CodeKey { rung: Doc, file: sdk/types.go, decl: 4, sub: 0, line: 24 } |  |  | 0.640 |
| walker |  | 5037 | 28 | Code::CodeKey { rung: Names, file: internal/tokens/init.go, decl: 0, sub: 0, line: 0 } |  |  | 0.640 |
| walker |  | 5052 | 15 | Code::CodeKey { rung: Body, file: internal/tokens/init.go, decl: 1, sub: 0, line: 21 } |  |  | 0.640 |
| walker |  | 5067 | 15 | Code::CodeKey { rung: Body, file: internal/tokens/init.go, decl: 2, sub: 0, line: 50 } |  |  | 0.640 |
| ns | 5185 |  | 179 | agent factory: AgentCreationOptions fields and both functions | 4.4 |  | 0.629 |
| walker |  | 5258 | 191 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.629 |
| walker |  | 5276 | 18 | Code::CodeKey { rung: Body, file: cmd/root.go, decl: 1, sub: 0, line: 131 } |  |  | 0.629 |
| walker |  | 5313 | 37 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 8, sub: 0, line: 218 } |  |  | 0.629 |
| walker |  | 5498 | 185 | Code::CodeKey { rung: Names, file: internal/config/config.go, decl: 0, sub: 0, line: 0 } |  |  | 0.638 |
| walker |  | 5540 | 42 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 3, sub: 0, line: 109 } |  |  | 0.638 |
| ns | 5548 |  | 363 | Every top-level symbol of internal/tools/mcp.go (locations) | 4.5 |  | 0.621 |
| walker |  | 5759 | 219 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 5, sub: 0, line: 137 } |  |  | 0.621 |
| ns | 5764 |  | 216 | AgenticLoopConfig head and every mode-driving function in cmd/root.go (locations) | 4.6 |  | 0.614 |
| ns | 5906 |  | 142 | CreateProvider: the complete list of supported providers | 5.1 |  | 0.606 |
| walker |  | 6058 | 299 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 4, sub: 0, line: 116 } |  |  | 0.606 |
| ns | 6234 |  | 328 | Every top-level symbol of internal/models/providers.go (locations) | 5.2 | 5.1 | 0.589 |
| walker |  | 6291 | 233 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 1, sub: 0, line: 17 } |  |  | 0.614 |
| ns | 6384 |  | 150 | ModelsRegistry: model validation and suggestion API | 5.3 |  | 0.607 |
| walker |  | 6457 | 166 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 6, sub: 0, line: 155 } |  |  | 0.614 |
| ns | 6476 |  | 92 | The generated model catalogue: generator types and the DO-NOT-EDIT header | 5.4 |  | 0.609 |
| walker |  | 6608 | 151 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 6, sub: 1, line: 155 } |  |  | 0.625 |
| ns | 6652 |  | 176 | Builtin server registry: the complete set of in-process servers | 6.1 |  | 0.614 |
| walker |  | 6777 | 169 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 6, sub: 2, line: 155 } |  |  | 0.629 |
| ns | 6824 |  | 172 | Every top-level symbol of internal/builtin/registry.go (locations) | 6.2 | 6.1 | 0.622 |
| walker |  | 6902 | 125 | Code::CodeKey { rung: Names, file: internal/hooks/events.go, decl: 0, sub: 0, line: 0 } |  |  | 0.622 |
| walker |  | 6995 | 93 | Code::CodeKey { rung: Decl, file: internal/hooks/events.go, decl: 2, sub: 0, line: 7 } |  |  | 0.622 |
| ns | 7041 |  | 217 | Bash builtin: output/timeout limits and the complete banned-command list | 6.3 |  | 0.608 |
| walker |  | 7052 | 57 | Code::CodeKey { rung: Names, file: internal/hooks/config.go, decl: 0, sub: 0, line: 0 } |  |  | 0.608 |
| walker |  | 7079 | 27 | Code::CodeKey { rung: Decl, file: internal/hooks/config.go, decl: 1, sub: 0, line: 14 } |  |  | 0.608 |
| walker |  | 7134 | 55 | Code::CodeKey { rung: Decl, file: internal/hooks/config.go, decl: 3, sub: 0, line: 30 } |  |  | 0.608 |
| walker |  | 7199 | 65 | Code::CodeKey { rung: Decl, file: internal/hooks/config.go, decl: 2, sub: 0, line: 21 } |  |  | 0.609 |
| ns | 7239 |  | 198 | The http builtin: its four tools and every symbol in http.go (locations) | 6.4 |  | 0.599 |
| ns | 7351 |  | 112 | HookEvent: the complete set of hook events | 7.1 |  | 0.605 |
| walker |  | 7371 | 172 | Code::CodeKey { rung: Names, file: internal/session/session.go, decl: 0, sub: 0, line: 0 } |  |  | 0.605 |
| walker |  | 7466 | 95 | Code::CodeKey { rung: Decl, file: internal/session/session.go, decl: 4, sub: 0, line: 66 } |  |  | 0.605 |
| ns | 7566 |  | 215 | Hook configuration schema: HookConfig, HookMatcher, HookEntry | 7.2 |  | 0.611 |
| walker |  | 7577 | 111 | Code::CodeKey { rung: Decl, file: internal/session/session.go, decl: 2, sub: 0, line: 36 } |  |  | 0.611 |
| walker |  | 7731 | 154 | Code::CodeKey { rung: Decl, file: internal/session/session.go, decl: 1, sub: 0, line: 19 } |  |  | 0.611 |
| ns | 7838 |  | 272 | Hook wire protocol: CommonInput and HookOutput | 7.3 |  | 0.603 |
| walker |  | 7933 | 202 | Code::CodeKey { rung: Decl, file: internal/session/session.go, decl: 3, sub: 0, line: 48 } |  |  | 0.603 |
| walker |  | 8028 | 95 | Code::CodeKey { rung: Names, file: internal/models/providers.go, decl: 0, sub: 0, line: 0 } |  |  | 0.604 |
| ns | 8029 |  | 191 | Per-event hook input structs | 7.4 |  | 0.598 |
| walker |  | 8088 | 60 | Code::CodeKey { rung: Decl, file: internal/models/providers.go, decl: 1, sub: 0, line: 28 } |  |  | 0.598 |
| walker |  | 8184 | 96 | Code::CodeKey { rung: Decl, file: internal/models/providers.go, decl: 3, sub: 0, line: 124 } |  |  | 0.598 |
| walker |  | 8299 | 115 | Code::CodeKey { rung: Decl, file: internal/models/providers.go, decl: 5, sub: 0, line: 539 } |  |  | 0.598 |
| ns | 8335 |  | 306 | Hook executor and validator symbol rosters | 7.5 |  | 0.587 |
| ns | 8492 |  | 157 | README: hooks.yml file locations and the --no-hooks escape hatch | 7.6 | 1.6 | 0.582 |
| walker |  | 8543 | 244 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.582 |
| ns | 8681 |  | 189 | Script mode: a worked frontmatter example and the variable rules | 8.1 |  | 0.575 |
| walker |  | 8772 | 229 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.575 |
| walker |  | 8811 | 39 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 7, sub: 0, line: 207 } |  |  | 0.575 |
| ns | 8812 |  | 131 | Every top-level symbol of cmd/script.go (locations) | 8.2 |  | 0.570 |
| walker |  | 8827 | 16 | Code::CodeKey { rung: Names, file: internal/tokens/counter.go, decl: 0, sub: 0, line: 0 } |  |  | 0.570 |
| walker |  | 9036 | 209 | Code::CodeKey { rung: Decl, file: internal/models/providers.go, decl: 2, sub: 0, line: 70 } |  |  | 0.570 |
| ns | 9110 |  | 298 | Session file format: Session, Metadata, Message and ToolCall fields | 8.3 |  | 0.582 |
| ns | 9404 |  | 294 | The public SDK surface: Options and every exported symbol | 8.4 |  | 0.592 |
| walker |  | 9500 | 464 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.592 |
| ns | 9606 |  | 202 | The complete slash-command table: names and descriptions | 8.5 |  | 0.585 |
| walker |  | 9810 | 310 | Code::CodeKey { rung: Names, file: internal/agent/agent.go, decl: 0, sub: 0, line: 0 } |  |  | 0.594 |
| ns | 9827 |  | 221 | ui.SetupCLI: the AgentInterface contract and CLISetupOptions | 8.6 |  | 0.586 |
| walker |  | 9870 | 60 | Code::CodeKey { rung: Decl, file: internal/agent/agent.go, decl: 12, sub: 0, line: 135 } |  |  | 0.586 |
| walker |  | 9937 | 67 | Code::CodeKey { rung: Decl, file: internal/agent/agent.go, decl: 13, sub: 0, line: 144 } |  |  | 0.586 |
| walker |  | 9972 | 35 | Code::CodeKey { rung: Decl, file: internal/agent/agent.go, decl: 11, sub: 0, line: 125 } |  |  | 0.586 |
| ns | 9983 |  | 156 | Release packaging: the goreleaser build matrix | 9.1 |  | 0.579 |

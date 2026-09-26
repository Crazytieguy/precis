Score(3000)=0.807 I=0.943 C=0.690 ns_rows≤3K=18/51 grid(1000/1442/2080/3000/4327/6240/9000)=0.940/0.996/0.909/0.807/0.665/0.601/0.566

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
| ns | 1753 |  | 250 | Persistent flag registration, part 1: config, system-prompt, model, debug, prompt, quiet | 2.3 |  | 0.909 |
| ns | 2097 |  | 344 | Persistent flag registration, part 2: no-exit, max-steps, stream, compact, no-hooks, approve-tool-run, session flags | 2.4 |  | 0.864 |
| walker |  | 2195 | 514 | GoMod::File { file: go.mod } |  |  | 0.864 |
| ns | 2223 |  | 126 | Provider and TLS flag registration | 2.5 |  | 0.855 |
| walker |  | 2408 | 213 | Code::CodeKey { rung: Names, file: sdk/mcphost.go, decl: 0, sub: 0, line: 0 } |  |  | 0.856 |
| walker |  | 2443 | 35 | Code::CodeKey { rung: Decl, file: sdk/mcphost.go, decl: 1, sub: 0, line: 19 } |  |  | 0.856 |
| ns | 2501 |  | 278 | Generation-parameter and Ollama flag registration, with the hidden flag | 2.6 |  | 0.834 |
| walker |  | 2511 | 68 | Code::CodeKey { rung: Decl, file: sdk/mcphost.go, decl: 5, sub: 0, line: 163 } |  |  | 0.834 |
| walker |  | 2618 | 107 | Code::CodeKey { rung: Decl, file: sdk/mcphost.go, decl: 2, sub: 0, line: 28 } |  |  | 0.835 |
| walker |  | 2683 | 65 | Code::CodeKey { rung: Names, file: sdk/types.go, decl: 0, sub: 0, line: 0 } |  |  | 0.836 |
| walker |  | 2694 | 11 | Code::CodeKey { rung: Body, file: sdk/types.go, decl: 3, sub: 0, line: 18 } |  |  | 0.836 |
| walker |  | 2706 | 12 | Code::CodeKey { rung: Body, file: sdk/types.go, decl: 4, sub: 0, line: 24 } |  |  | 0.836 |
| walker |  | 2715 | 9 | Code::CodeKey { rung: Body, file: sdk/mcphost.go, decl: 6, sub: 0, line: 201 } |  |  | 0.836 |
| walker |  | 2724 | 9 | Code::CodeKey { rung: Body, file: sdk/mcphost.go, decl: 10, sub: 0, line: 230 } |  |  | 0.836 |
| walker |  | 2733 | 9 | Code::CodeKey { rung: Body, file: sdk/mcphost.go, decl: 11, sub: 0, line: 237 } |  |  | 0.836 |
| ns | 2748 |  | 247 | MCPServerConfig: complete field set including the legacy block | 3.1 |  | 0.807 |
| walker |  | 2996 | 263 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.807 |
| walker |  | 3025 | 29 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 6, sub: 0, line: 201 } |  |  | 0.807 |
| walker |  | 3054 | 29 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 9, sub: 0, line: 224 } |  |  | 0.807 |
| ns | 3068 |  | 320 | Config struct: application-level keys | 3.2 |  | 0.782 |
| walker |  | 3116 | 62 | Plaintext::DeclSurface { file: contribute/boost.sh } |  |  | 0.782 |
| walker |  | 3148 | 32 | Code::CodeKey { rung: Doc, file: sdk/types.go, decl: 3, sub: 0, line: 18 } |  |  | 0.782 |
| ns | 3249 |  | 181 | Config struct: generation-parameter and TLS keys | 3.3 |  | 0.765 |
| walker |  | 3433 | 285 | Code::CodeKey { rung: Names, file: cmd/root.go, decl: 0, sub: 0, line: 0 } |  |  | 0.765 |
| walker |  | 3507 | 74 | Code::CodeKey { rung: Decl, file: cmd/root.go, decl: 1, sub: 0, line: 27 } |  |  | 0.765 |
| ns | 3513 |  | 264 | GetTransportType: type-to-transport mapping and legacy inference | 3.4 |  | 0.726 |
| walker |  | 3787 | 280 | Code::CodeKey { rung: Names, file: cmd/script.go, decl: 0, sub: 0, line: 0 } |  |  | 0.727 |
| walker |  | 3852 | 65 | Code::CodeKey { rung: Decl, file: cmd/script.go, decl: 9, sub: 0, line: 423 } |  |  | 0.727 |
| walker |  | 3868 | 16 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 4, sub: 0, line: 148 } |  |  | 0.727 |
| walker |  | 3884 | 16 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 7, sub: 0, line: 279 } |  |  | 0.727 |
| walker |  | 3902 | 18 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 14, sub: 0, line: 511 } |  |  | 0.727 |
| walker |  | 3921 | 19 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 8, sub: 0, line: 288 } |  |  | 0.727 |
| ns | 3937 |  | 424 | Config.Validate: required fields per transport and filter exclusivity | 3.5 |  | 0.691 |
| walker |  | 3941 | 20 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 6, sub: 0, line: 255 } |  |  | 0.691 |
| walker |  | 4101 | 160 | Code::CodeKey { rung: Names, file: cmd/auth.go, decl: 0, sub: 0, line: 0 } |  |  | 0.691 |
| ns | 4140 |  | 203 | Substitution engine: the two regexes plus every symbol in substitution.go | 3.6 |  | 0.678 |
| walker |  | 4220 | 119 | Code::CodeKey { rung: Decl, file: cmd/auth.go, decl: 4, sub: 0, line: 78 } |  |  | 0.678 |
| ns | 4294 |  | 154 | Remaining top-level symbols of internal/config/config.go and all of merger.go (locations) | 3.7 |  | 0.665 |
| walker |  | 4381 | 161 | Code::CodeKey { rung: Decl, file: cmd/auth.go, decl: 1, sub: 0, line: 17 } |  |  | 0.666 |
| ns | 4540 |  | 246 | Agent struct and its seven callback handler types | 4.1 |  | 0.650 |
| walker |  | 4548 | 167 | Code::CodeKey { rung: Decl, file: cmd/auth.go, decl: 3, sub: 0, line: 58 } |  |  | 0.652 |
| walker |  | 4723 | 175 | Code::CodeKey { rung: Decl, file: cmd/auth.go, decl: 2, sub: 0, line: 37 } |  |  | 0.656 |
| walker |  | 4794 | 71 | Code::CodeKey { rung: Names, file: cmd/hooks.go, decl: 0, sub: 0, line: 0 } |  |  | 0.656 |
| ns | 4814 |  | 274 | Every top-level symbol of internal/agent/agent.go (locations) | 4.2 |  | 0.639 |
| walker |  | 4841 | 47 | Code::CodeKey { rung: Decl, file: cmd/hooks.go, decl: 1, sub: 0, line: 16 } |  |  | 0.643 |
| ns | 5006 |  | 192 | The tool-calling loop: step bound, tool-call branch and the approval gate | 4.3 | 4.2 | 0.631 |
| walker |  | 5026 | 185 | Code::CodeKey { rung: Decl, file: cmd/hooks.go, decl: 3, sub: 0, line: 57 } |  |  | 0.636 |
| ns | 5185 |  | 179 | agent factory: AgentCreationOptions fields and both functions | 4.4 |  | 0.625 |
| walker |  | 5346 | 320 | Code::CodeKey { rung: Decl, file: cmd/hooks.go, decl: 2, sub: 0, line: 25 } |  |  | 0.629 |
| walker |  | 5542 | 196 | Code::CodeKey { rung: Names, file: cmd/root.go, decl: 0, sub: 1, line: 0 } |  |  | 0.629 |
| ns | 5548 |  | 363 | Every top-level symbol of internal/tools/mcp.go (locations) | 4.5 |  | 0.612 |
| walker |  | 5555 | 13 | Code::CodeKey { rung: Decl, file: cmd/root.go, decl: 2, sub: 0, line: 67 } |  |  | 0.612 |
| walker |  | 5570 | 15 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 2, sub: 0, line: 67 } |  |  | 0.612 |
| walker |  | 5758 | 188 | Code::CodeKey { rung: Names, file: cmd/root.go, decl: 0, sub: 2, line: 0 } |  |  | 0.612 |
| ns | 5764 |  | 216 | AgenticLoopConfig head and every mode-driving function in cmd/root.go (locations) | 4.6 |  | 0.603 |
| ns | 5906 |  | 142 | CreateProvider: the complete list of supported providers | 5.1 |  | 0.594 |
| walker |  | 5974 | 216 | Code::CodeKey { rung: Decl, file: cmd/root.go, decl: 14, sub: 0, line: 758 } |  |  | 0.608 |
| walker |  | 6184 | 210 | Code::CodeKey { rung: Names, file: cmd/root.go, decl: 0, sub: 3, line: 0 } |  |  | 0.618 |
| walker |  | 6199 | 15 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 22, sub: 0, line: 1349 } |  |  | 0.618 |
| walker |  | 6217 | 18 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 21, sub: 0, line: 1316 } |  |  | 0.618 |
| ns | 6234 |  | 328 | Every top-level symbol of internal/models/providers.go (locations) | 5.2 | 5.1 | 0.601 |
| walker |  | 6236 | 19 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 16, sub: 0, line: 794 } |  |  | 0.601 |
| walker |  | 6255 | 19 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 17, sub: 0, line: 811 } |  |  | 0.601 |
| walker |  | 6274 | 19 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 20, sub: 0, line: 1229 } |  |  | 0.601 |
| walker |  | 6294 | 20 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 15, sub: 0, line: 777 } |  |  | 0.601 |
| walker |  | 6314 | 20 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 19, sub: 0, line: 1195 } |  |  | 0.601 |
| ns | 6384 |  | 150 | ModelsRegistry: model validation and suggestion API | 5.3 |  | 0.594 |
| ns | 6476 |  | 92 | The generated model catalogue: generator types and the DO-NOT-EDIT header | 5.4 |  | 0.590 |
| ns | 6652 |  | 176 | Builtin server registry: the complete set of in-process servers | 6.1 |  | 0.579 |
| walker |  | 6784 | 470 | Code::CodeKey { rung: Decl, file: cmd/root.go, decl: 6, sub: 0, line: 92 } |  |  | 0.579 |
| walker |  | 6807 | 23 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 18, sub: 0, line: 879 } |  |  | 0.579 |
| ns | 6824 |  | 172 | Every top-level symbol of internal/builtin/registry.go (locations) | 6.2 | 6.1 | 0.573 |
| ns | 7041 |  | 217 | Bash builtin: output/timeout limits and the complete banned-command list | 6.3 |  | 0.559 |
| ns | 7239 |  | 198 | The http builtin: its four tools and every symbol in http.go (locations) | 6.4 |  | 0.550 |
| ns | 7351 |  | 112 | HookEvent: the complete set of hook events | 7.1 |  | 0.546 |
| walker |  | 7437 | 630 | Code::CodeKey { rung: Decl, file: cmd/script.go, decl: 1, sub: 0, line: 28 } |  |  | 0.553 |
| walker |  | 7468 | 31 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 12, sub: 0, line: 480 } |  |  | 0.553 |
| walker |  | 7501 | 33 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 10, sub: 0, line: 431 } |  |  | 0.553 |
| walker |  | 7534 | 33 | Code::CodeKey { rung: Doc, file: sdk/types.go, decl: 1, sub: 0, line: 10 } |  |  | 0.553 |
| ns | 7566 |  | 215 | Hook configuration schema: HookConfig, HookMatcher, HookEntry | 7.2 |  | 0.545 |
| walker |  | 7568 | 34 | Code::CodeKey { rung: Doc, file: sdk/types.go, decl: 2, sub: 0, line: 14 } |  |  | 0.545 |
| walker |  | 7602 | 34 | Code::CodeKey { rung: Doc, file: sdk/types.go, decl: 4, sub: 0, line: 24 } |  |  | 0.545 |
| walker |  | 7630 | 28 | Code::CodeKey { rung: Names, file: internal/tokens/init.go, decl: 0, sub: 0, line: 0 } |  |  | 0.545 |
| walker |  | 7645 | 15 | Code::CodeKey { rung: Body, file: internal/tokens/init.go, decl: 1, sub: 0, line: 21 } |  |  | 0.545 |
| walker |  | 7660 | 15 | Code::CodeKey { rung: Body, file: internal/tokens/init.go, decl: 2, sub: 0, line: 50 } |  |  | 0.545 |
| ns | 7838 |  | 272 | Hook wire protocol: CommonInput and HookOutput | 7.3 |  | 0.537 |
| walker |  | 7851 | 191 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.537 |
| walker |  | 7887 | 36 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 11, sub: 0, line: 442 } |  |  | 0.537 |
| walker |  | 7924 | 37 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 8, sub: 0, line: 218 } |  |  | 0.537 |
| ns | 8029 |  | 191 | Per-event hook input structs | 7.4 |  | 0.532 |
| walker |  | 8136 | 212 | Code::CodeKey { rung: Names, file: internal/config/config.go, decl: 0, sub: 0, line: 0 } |  |  | 0.542 |
| walker |  | 8178 | 42 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 3, sub: 0, line: 109 } |  |  | 0.542 |
| ns | 8335 |  | 306 | Hook executor and validator symbol rosters | 7.5 |  | 0.532 |
| walker |  | 8397 | 219 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 5, sub: 0, line: 137 } |  |  | 0.532 |
| ns | 8492 |  | 157 | README: hooks.yml file locations and the --no-hooks escape hatch | 7.6 | 1.6 | 0.528 |
| ns | 8681 |  | 189 | Script mode: a worked frontmatter example and the variable rules | 8.1 |  | 0.538 |
| walker |  | 8696 | 299 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 4, sub: 0, line: 116 } |  |  | 0.538 |
| ns | 8812 |  | 131 | Every top-level symbol of cmd/script.go (locations) | 8.2 |  | 0.546 |
| walker |  | 8929 | 233 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 1, sub: 0, line: 17 } |  |  | 0.566 |
| walker |  | 9095 | 166 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 6, sub: 0, line: 155 } |  |  | 0.571 |
| ns | 9110 |  | 298 | Session file format: Session, Metadata, Message and ToolCall fields | 8.3 |  | 0.562 |
| walker |  | 9246 | 151 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 6, sub: 1, line: 155 } |  |  | 0.574 |
| ns | 9404 |  | 294 | The public SDK surface: Options and every exported symbol | 8.4 |  | 0.585 |
| walker |  | 9415 | 169 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 6, sub: 2, line: 155 } |  |  | 0.596 |
| walker |  | 9600 | 185 | Code::CodeKey { rung: Names, file: internal/session/session.go, decl: 0, sub: 0, line: 0 } |  |  | 0.596 |
| ns | 9606 |  | 202 | The complete slash-command table: names and descriptions | 8.5 |  | 0.590 |
| walker |  | 9695 | 95 | Code::CodeKey { rung: Decl, file: internal/session/session.go, decl: 4, sub: 0, line: 66 } |  |  | 0.591 |
| walker |  | 9806 | 111 | Code::CodeKey { rung: Decl, file: internal/session/session.go, decl: 2, sub: 0, line: 36 } |  |  | 0.593 |
| ns | 9827 |  | 221 | ui.SetupCLI: the AgentInterface contract and CLISetupOptions | 8.6 |  | 0.585 |
| walker |  | 9960 | 154 | Code::CodeKey { rung: Decl, file: internal/session/session.go, decl: 1, sub: 0, line: 19 } |  |  | 0.591 |
| ns | 9983 |  | 156 | Release packaging: the goreleaser build matrix | 9.1 |  | 0.584 |
| walker |  | 9999 | 39 | Code::CodeKey { rung: Decl, file: internal/session/session.go, decl: 3, sub: 0, line: 48 } |  |  | 0.585 |

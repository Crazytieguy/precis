Score(3000)=0.659 I=0.850 C=0.511 ns_rows≤3K=18/51 grid(1000/1442/2080/3000/4327/6240/9000)=0.781/0.777/0.741/0.659/0.575/0.496/0.502

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 40 | 40 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 55 | 15 | Fs::DirListing { dir: contribute } |  |  | 0.000 |
| walker |  | 63 | 8 | Fs::DirListing { dir: contribute/conf } |  |  | 0.000 |
| ns | 63 |  | 63 | README title and one-line identity | 1.1 |  | 0.000 |
| walker |  | 90 | 27 | Code::CodeKey { rung: Names, file: main.go, decl: 0, sub: 0, line: 0 } |  |  | 0.000 |
| walker |  | 93 | 3 | Fs::DirListing { dir: .github } |  |  | 0.000 |
| walker |  | 101 | 8 | Fs::DirListing { dir: .github/workflows } |  |  | 0.000 |
| ns | 103 |  | 40 | Complete repository root listing | 1.2 |  | 0.625 |
| walker |  | 125 | 24 | Fs::DirListing { dir: sdk } |  |  | 0.633 |
| ns | 133 |  | 30 | Complete internal/ package listing | 1.3 |  | 0.472 |
| walker |  | 155 | 30 | Fs::DirListing { dir: internal } |  |  | 0.697 |
| walker |  | 163 | 8 | Fs::DirListing { dir: internal/session } |  |  | 0.697 |
| walker |  | 175 | 12 | Fs::DirListing { dir: internal/agent } |  |  | 0.698 |
| walker |  | 188 | 13 | Fs::DirListing { dir: internal/tokens } |  |  | 0.698 |
| ns | 207 |  | 74 | Complete cmd/ and sdk/ listings | 1.4 |  | 0.540 |
| walker |  | 224 | 36 | Fs::DirListing { dir: cmd } |  |  | 0.646 |
| walker |  | 241 | 17 | Fs::DirListing { dir: internal/auth } |  |  | 0.647 |
| walker |  | 247 | 6 | Fs::DirListing { dir: examples } |  |  | 0.649 |
| ns | 260 |  | 53 | Module path, Go version and toolchain | 1.5 |  | 0.606 |
| walker |  | 310 | 63 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.842 |
| walker |  | 334 | 24 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.842 |
| walker |  | 387 | 53 | GoMod::Identity { file: go.mod } |  |  | 0.910 |
| walker |  | 422 | 35 | Fs::DirListing { dir: internal/models } |  |  | 0.915 |
| walker |  | 427 | 5 | Fs::DirListing { dir: internal/models/anthropic } |  |  | 0.784 |
| ns | 427 |  | 167 | All README section headings (locations only) | 1.6 |  | 0.784 |
| walker |  | 432 | 5 | Fs::DirListing { dir: internal/models/gemini } |  |  | 0.784 |
| walker |  | 437 | 5 | Fs::DirListing { dir: internal/models/openai } |  |  | 0.785 |
| walker |  | 476 | 39 | Fs::DirListing { dir: internal/hooks } |  |  | 0.789 |
| walker |  | 486 | 10 | Fs::DirListing { dir: internal/hooks/testdata } |  |  | 0.790 |
| walker |  | 526 | 40 | Markdown::ReadmeHeadline { file: sdk/README.md } |  |  | 0.790 |
| ns | 534 |  | 107 | README feature list, first half | 1.7 | 1.6 | 0.734 |
| walker |  | 566 | 40 | Fs::DirListing { dir: internal/builtin } |  |  | 0.741 |
| walker |  | 582 | 16 | Code::CodeKey { rung: Names, file: internal/tokens/counter.go, decl: 0, sub: 0, line: 0 } |  |  | 0.741 |
| walker |  | 627 | 45 | Fs::DirListing { dir: internal/config } |  |  | 0.752 |
| ns | 648 |  | 114 | README feature list, second half | 1.8 |  | 0.704 |
| walker |  | 660 | 33 | Markdown::HeadingsOutline { file: contribute/contribute.md } |  |  | 0.704 |
| walker |  | 688 | 28 | Code::CodeKey { rung: Names, file: internal/tokens/init.go, decl: 0, sub: 0, line: 0 } |  |  | 0.704 |
| walker |  | 718 | 30 | Code::CodeKey { rung: Names, file: internal/auth/browser.go, decl: 0, sub: 0, line: 0 } |  |  | 0.704 |
| ns | 781 |  | 133 | Complete listings for the config / agent / tools / models packages | 1.9 |  | 0.709 |
| walker |  | 804 | 86 | Fs::DirListing { dir: internal/ui } |  |  | 0.725 |
| walker |  | 809 | 5 | Fs::DirListing { dir: internal/ui/progress } |  |  | 0.726 |
| walker |  | 830 | 21 | Code::CodeKey { rung: Names, file: internal/ui/callbacks.go, decl: 0, sub: 0, line: 0 } |  |  | 0.726 |
| walker |  | 895 | 65 | Code::CodeKey { rung: Names, file: sdk/types.go, decl: 0, sub: 0, line: 0 } |  |  | 0.726 |
| walker |  | 931 | 36 | Code::CodeKey { rung: Names, file: internal/agent/streaming.go, decl: 0, sub: 0, line: 0 } |  |  | 0.726 |
| walker |  | 942 | 11 | Code::CodeKey { rung: Body, file: sdk/types.go, decl: 3, sub: 0, line: 18 } |  |  | 0.726 |
| ns | 999 |  | 218 | Complete listings for the builtin / hooks / session / auth / tokens / ui packages | 1.10 |  | 0.781 |
| walker |  | 1013 | 71 | Code::CodeKey { rung: Names, file: cmd/hooks.go, decl: 0, sub: 0, line: 0 } |  |  | 0.781 |
| walker |  | 1025 | 12 | Code::CodeKey { rung: Body, file: sdk/types.go, decl: 4, sub: 0, line: 24 } |  |  | 0.781 |
| walker |  | 1082 | 57 | Code::CodeKey { rung: Names, file: internal/config/merger.go, decl: 0, sub: 0, line: 0 } |  |  | 0.781 |
| ns | 1089 |  | 90 | Complete listings for examples/, contribute/ and .github/ | 1.11 |  | 0.747 |
| ns | 1243 |  | 154 | main.go entry point | 2.1 |  | 0.715 |
| walker |  | 1311 | 229 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: true } |  |  | 0.777 |
| walker |  | 1371 | 60 | Code::CodeKey { rung: Names, file: internal/ui/commands.go, decl: 0, sub: 0, line: 0 } |  |  | 0.777 |
| walker |  | 1422 | 51 | Code::CodeKey { rung: Decl, file: internal/ui/commands.go, decl: 1, sub: 0, line: 6 } |  |  | 0.777 |
| walker |  | 1483 | 61 | Code::CodeKey { rung: Names, file: internal/models/models_data.go, decl: 0, sub: 0, line: 0 } |  |  | 0.777 |
| walker |  | 1503 | 20 | Code::CodeKey { rung: Decl, file: internal/models/models_data.go, decl: 3, sub: 0, line: 26 } |  |  | 0.777 |
| ns | 1504 |  | 261 | Complete cobra command tree: script, auth (login/logout/status), hooks (list/validate/init) | 2.2 |  | 0.737 |
| walker |  | 1549 | 46 | Code::CodeKey { rung: Decl, file: internal/models/models_data.go, decl: 2, sub: 0, line: 18 } |  |  | 0.737 |
| walker |  | 1601 | 52 | Code::CodeKey { rung: Decl, file: internal/models/models_data.go, decl: 4, sub: 0, line: 32 } |  |  | 0.737 |
| walker |  | 1668 | 67 | Code::CodeKey { rung: Names, file: internal/ui/factory.go, decl: 0, sub: 0, line: 0 } |  |  | 0.737 |
| walker |  | 1721 | 53 | Code::CodeKey { rung: Decl, file: internal/ui/factory.go, decl: 1, sub: 0, line: 13 } |  |  | 0.737 |
| walker |  | 1731 | 10 | Markdown::Section { file: contribute/contribute.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.737 |
| ns | 1754 |  | 250 | Persistent flag registration, part 1: config, system-prompt, model, debug, prompt, quiet | 2.3 |  | 0.709 |
| walker |  | 1757 | 26 | Fs::DirListing { dir: internal/tools } |  |  | 0.741 |
| walker |  | 1823 | 66 | Code::CodeKey { rung: Decl, file: internal/models/models_data.go, decl: 1, sub: 0, line: 7 } |  |  | 0.741 |
| walker |  | 1843 | 20 | Markdown::Section { file: contribute/contribute.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.741 |
| walker |  | 1917 | 74 | Code::CodeKey { rung: Names, file: internal/hooks/schemas.go, decl: 0, sub: 0, line: 0 } |  |  | 0.741 |
| walker |  | 1941 | 24 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 4, sub: 0, line: 42 } |  |  | 0.741 |
| walker |  | 1985 | 44 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 2, sub: 0, line: 23 } |  |  | 0.741 |
| walker |  | 2046 | 61 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 3, sub: 0, line: 32 } |  |  | 0.741 |
| ns | 2098 |  | 344 | Persistent flag registration, part 2: no-exit, max-steps, stream, compact, no-hooks, approve-tool-run, session flags | 2.4 |  | 0.704 |
| walker |  | 2120 | 74 | Code::CodeKey { rung: Names, file: internal/ui/fuzzy.go, decl: 0, sub: 0, line: 0 } |  |  | 0.704 |
| walker |  | 2143 | 23 | Code::CodeKey { rung: Decl, file: internal/ui/fuzzy.go, decl: 1, sub: 0, line: 10 } |  |  | 0.704 |
| walker |  | 2219 | 76 | Code::CodeKey { rung: Names, file: internal/models/generate_models.go, decl: 0, sub: 0, line: 0 } |  |  | 0.705 |
| ns | 2224 |  | 126 | Provider and TLS flag registration | 2.5 |  | 0.698 |
| walker |  | 2276 | 57 | Code::CodeKey { rung: Decl, file: internal/models/generate_models.go, decl: 3, sub: 0, line: 50 } |  |  | 0.698 |
| walker |  | 2355 | 79 | Code::CodeKey { rung: Names, file: internal/ui/debug_logger.go, decl: 0, sub: 0, line: 0 } |  |  | 0.698 |
| walker |  | 2369 | 14 | Code::CodeKey { rung: Decl, file: internal/ui/debug_logger.go, decl: 1, sub: 0, line: 13 } |  |  | 0.698 |
| walker |  | 2449 | 80 | Code::CodeKey { rung: Names, file: internal/agent/factory.go, decl: 0, sub: 0, line: 0 } |  |  | 0.698 |
| ns | 2502 |  | 278 | Generation-parameter and Ollama flag registration, with the hidden flag | 2.6 |  | 0.680 |
| walker |  | 2609 | 160 | Code::CodeKey { rung: Names, file: cmd/auth.go, decl: 0, sub: 0, line: 0 } |  |  | 0.680 |
| walker |  | 2689 | 80 | Code::CodeKey { rung: Decl, file: internal/ui/factory.go, decl: 2, sub: 0, line: 22 } |  |  | 0.681 |
| walker |  | 2736 | 47 | Code::CodeKey { rung: Decl, file: cmd/hooks.go, decl: 1, sub: 0, line: 16 } |  |  | 0.681 |
| ns | 2749 |  | 247 | MCPServerConfig: complete field set including the legacy block | 3.1 |  | 0.658 |
| walker |  | 2824 | 88 | Code::CodeKey { rung: Names, file: internal/hooks/config.go, decl: 0, sub: 0, line: 0 } |  |  | 0.658 |
| walker |  | 2851 | 27 | Code::CodeKey { rung: Decl, file: internal/hooks/config.go, decl: 1, sub: 0, line: 14 } |  |  | 0.658 |
| walker |  | 2906 | 55 | Code::CodeKey { rung: Decl, file: internal/hooks/config.go, decl: 3, sub: 0, line: 30 } |  |  | 0.659 |
| walker |  | 2971 | 65 | Code::CodeKey { rung: Decl, file: internal/hooks/config.go, decl: 2, sub: 0, line: 21 } |  |  | 0.659 |
| walker |  | 2985 | 14 | Code::CodeKey { rung: Body, file: internal/ui/debug_logger.go, decl: 4, sub: 0, line: 84 } |  |  | 0.659 |
| ns | 3069 |  | 320 | Config struct: application-level keys | 3.2 |  | 0.639 |
| walker |  | 3074 | 89 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 6, sub: 0, line: 62 } |  |  | 0.639 |
| walker |  | 3201 | 127 | Code::CodeKey { rung: Body, file: main.go, decl: 2, sub: 0, line: 14 } |  |  | 0.675 |
| walker |  | 3216 | 15 | Code::CodeKey { rung: Body, file: internal/tokens/init.go, decl: 1, sub: 0, line: 21 } |  |  | 0.675 |
| walker |  | 3231 | 15 | Code::CodeKey { rung: Body, file: internal/tokens/init.go, decl: 2, sub: 0, line: 50 } |  |  | 0.675 |
| walker |  | 3246 | 15 | Code::CodeKey { rung: Body, file: internal/ui/debug_logger.go, decl: 2, sub: 0, line: 20 } |  |  | 0.675 |
| ns | 3250 |  | 181 | Config struct: generation-parameter and TLS keys | 3.3 |  | 0.660 |
| walker |  | 3366 | 120 | Markdown::HeadingsOutline { file: sdk/README.md } |  |  | 0.660 |
| walker |  | 3379 | 13 | Markdown::Section { file: sdk/README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.660 |
| walker |  | 3412 | 33 | Markdown::Section { file: sdk/README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.660 |
| walker |  | 3508 | 96 | Markdown::Section { file: sdk/README.md, section_index: 6, keeps_default_concavity: true } |  |  | 0.660 |
| ns | 3514 |  | 264 | GetTransportType: type-to-transport mapping and legacy inference | 3.4 |  | 0.627 |
| walker |  | 3540 | 32 | Code::CodeKey { rung: Doc, file: sdk/types.go, decl: 3, sub: 0, line: 18 } |  |  | 0.627 |
| walker |  | 3644 | 104 | Code::CodeKey { rung: Names, file: internal/ui/tool_approval_input.go, decl: 0, sub: 0, line: 0 } |  |  | 0.627 |
| walker |  | 3653 | 9 | Code::CodeKey { rung: Body, file: internal/ui/tool_approval_input.go, decl: 3, sub: 0, line: 48 } |  |  | 0.627 |
| walker |  | 3729 | 76 | Code::CodeKey { rung: Decl, file: internal/ui/tool_approval_input.go, decl: 1, sub: 0, line: 12 } |  |  | 0.627 |
| walker |  | 3762 | 33 | Code::CodeKey { rung: Doc, file: sdk/types.go, decl: 1, sub: 0, line: 10 } |  |  | 0.627 |
| ns | 3938 |  | 424 | Config.Validate: required fields per transport and filter exclusivity | 3.5 |  | 0.596 |
| walker |  | 3993 | 231 | Code::CodeKey { rung: Names, file: internal/ui/cli.go, decl: 0, sub: 0, line: 0 } |  |  | 0.596 |
| walker |  | 4005 | 12 | Code::CodeKey { rung: Body, file: internal/ui/cli.go, decl: 5, sub: 0, line: 65 } |  |  | 0.596 |
| walker |  | 4019 | 14 | Code::CodeKey { rung: Body, file: internal/ui/cli.go, decl: 10, sub: 0, line: 152 } |  |  | 0.596 |
| walker |  | 4067 | 48 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.596 |
| walker |  | 4107 | 40 | Markdown::Section { file: README.md, section_index: 33, keeps_default_concavity: false } |  |  | 0.597 |
| walker |  | 4119 | 12 | Code::CodeKey { rung: Doc, file: internal/models/models_data.go, decl: 2, sub: 0, line: 18 } |  |  | 0.597 |
| ns | 4141 |  | 203 | Substitution engine: the two regexes plus every symbol in substitution.go | 3.6 |  | 0.586 |
| ns | 4295 |  | 154 | Remaining top-level symbols of internal/config/config.go and all of merger.go (locations) | 3.7 |  | 0.575 |
| walker |  | 4332 | 213 | Code::CodeKey { rung: Names, file: sdk/mcphost.go, decl: 0, sub: 0, line: 0 } |  |  | 0.576 |
| walker |  | 4367 | 35 | Code::CodeKey { rung: Decl, file: sdk/mcphost.go, decl: 1, sub: 0, line: 19 } |  |  | 0.576 |
| walker |  | 4376 | 9 | Code::CodeKey { rung: Body, file: sdk/mcphost.go, decl: 6, sub: 0, line: 201 } |  |  | 0.576 |
| walker |  | 4385 | 9 | Code::CodeKey { rung: Body, file: sdk/mcphost.go, decl: 10, sub: 0, line: 230 } |  |  | 0.576 |
| walker |  | 4394 | 9 | Code::CodeKey { rung: Body, file: sdk/mcphost.go, decl: 11, sub: 0, line: 237 } |  |  | 0.576 |
| walker |  | 4462 | 68 | Code::CodeKey { rung: Decl, file: sdk/mcphost.go, decl: 5, sub: 0, line: 163 } |  |  | 0.576 |
| ns | 4541 |  | 246 | Agent struct and its seven callback handler types | 4.1 |  | 0.561 |
| walker |  | 4569 | 107 | Code::CodeKey { rung: Decl, file: sdk/mcphost.go, decl: 2, sub: 0, line: 28 } |  |  | 0.562 |
| walker |  | 4676 | 107 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 5, sub: 0, line: 50 } |  |  | 0.563 |
| ns | 4815 |  | 274 | Every top-level symbol of internal/agent/agent.go (locations) | 4.2 |  | 0.549 |
| walker |  | 4956 | 280 | Code::CodeKey { rung: Names, file: cmd/script.go, decl: 0, sub: 0, line: 0 } |  |  | 0.550 |
| ns | 5007 |  | 192 | The tool-calling loop: step bound, tool-call branch and the approval gate | 4.3 | 4.2 | 0.540 |
| walker |  | 5021 | 65 | Code::CodeKey { rung: Decl, file: cmd/script.go, decl: 9, sub: 0, line: 423 } |  |  | 0.540 |
| walker |  | 5135 | 114 | Code::CodeKey { rung: Names, file: internal/hooks/events.go, decl: 0, sub: 0, line: 0 } |  |  | 0.540 |
| ns | 5186 |  | 179 | agent factory: AgentCreationOptions fields and both functions | 4.4 |  | 0.533 |
| walker |  | 5234 | 99 | Code::CodeKey { rung: Decl, file: internal/hooks/events.go, decl: 2, sub: 0, line: 7 } |  |  | 0.533 |
| walker |  | 5251 | 17 | Code::CodeKey { rung: Body, file: internal/hooks/events.go, decl: 4, sub: 0, line: 34 } |  |  | 0.533 |
| walker |  | 5263 | 12 | Code::CodeKey { rung: Body, file: sdk/mcphost.go, decl: 9, sub: 0, line: 224 } |  |  | 0.533 |
| ns | 5549 |  | 363 | Every top-level symbol of internal/tools/mcp.go (locations) | 4.5 |  | 0.519 |
| ns | 5765 |  | 216 | AgenticLoopConfig head and every mode-driving function in cmd/root.go (locations) | 4.6 |  | 0.508 |
| walker |  | 5777 | 514 | GoMod::File { file: go.mod } |  |  | 0.508 |
| walker |  | 5788 | 11 | Code::CodeKey { rung: Body, file: cmd/script.go, decl: 2, sub: 0, line: 78 } |  |  | 0.508 |
| ns | 5907 |  | 142 | CreateProvider: the complete list of supported providers | 5.1 |  | 0.501 |
| walker |  | 6024 | 236 | Code::CodeKey { rung: Names, file: internal/agent/agent.go, decl: 0, sub: 0, line: 0 } |  |  | 0.511 |
| walker |  | 6084 | 60 | Code::CodeKey { rung: Decl, file: internal/agent/agent.go, decl: 12, sub: 0, line: 135 } |  |  | 0.511 |
| walker |  | 6151 | 67 | Code::CodeKey { rung: Decl, file: internal/agent/agent.go, decl: 13, sub: 0, line: 144 } |  |  | 0.511 |
| walker |  | 6227 | 76 | Code::CodeKey { rung: Decl, file: internal/agent/agent.go, decl: 11, sub: 0, line: 125 } |  |  | 0.511 |
| ns | 6235 |  | 328 | Every top-level symbol of internal/models/providers.go (locations) | 5.2 | 5.1 | 0.496 |
| walker |  | 6330 | 103 | Code::CodeKey { rung: Decl, file: internal/agent/agent.go, decl: 9, sub: 0, line: 67 } |  |  | 0.515 |
| ns | 6385 |  | 150 | ModelsRegistry: model validation and suggestion API | 5.3 |  | 0.510 |
| ns | 6477 |  | 92 | The generated model catalogue: generator types and the DO-NOT-EDIT header | 5.4 |  | 0.509 |
| walker |  | 6588 | 258 | Markdown::Section { file: sdk/README.md, section_index: 2, keeps_default_concavity: true } |  |  | 0.509 |
| ns | 6653 |  | 176 | Builtin server registry: the complete set of in-process servers | 6.1 |  | 0.500 |
| ns | 6825 |  | 172 | Every top-level symbol of internal/builtin/registry.go (locations) | 6.2 | 6.1 | 0.494 |
| walker |  | 6827 | 239 | Code::CodeKey { rung: Names, file: internal/ui/messages.go, decl: 0, sub: 0, line: 0 } |  |  | 0.494 |
| walker |  | 6836 | 9 | Code::CodeKey { rung: Decl, file: internal/ui/messages.go, decl: 2, sub: 0, line: 17 } |  |  | 0.494 |
| walker |  | 6855 | 19 | Code::CodeKey { rung: Decl, file: internal/ui/messages.go, decl: 5, sub: 0, line: 47 } |  |  | 0.494 |
| walker |  | 6864 | 9 | Code::CodeKey { rung: Body, file: internal/ui/messages.go, decl: 8, sub: 0, line: 79 } |  |  | 0.494 |
| walker |  | 6929 | 65 | Code::CodeKey { rung: Decl, file: internal/ui/messages.go, decl: 3, sub: 0, line: 29 } |  |  | 0.494 |
| ns | 7042 |  | 217 | Bash builtin: output/timeout limits and the complete banned-command list | 6.3 |  | 0.482 |
| walker |  | 7154 | 225 | Code::CodeKey { rung: Names, file: internal/ui/compact_renderer.go, decl: 0, sub: 0, line: 0 } |  |  | 0.482 |
| walker |  | 7173 | 19 | Code::CodeKey { rung: Decl, file: internal/ui/compact_renderer.go, decl: 1, sub: 0, line: 14 } |  |  | 0.482 |
| walker |  | 7182 | 9 | Code::CodeKey { rung: Body, file: internal/ui/compact_renderer.go, decl: 3, sub: 0, line: 31 } |  |  | 0.482 |
| ns | 7240 |  | 198 | The http builtin: its four tools and every symbol in http.go (locations) | 6.4 |  | 0.475 |
| walker |  | 7313 | 131 | Code::CodeKey { rung: Decl, file: internal/models/generate_models.go, decl: 2, sub: 0, line: 37 } |  |  | 0.475 |
| ns | 7352 |  | 112 | HookEvent: the complete set of hook events | 7.1 |  | 0.482 |
| walker |  | 7451 | 138 | Code::CodeKey { rung: Names, file: internal/ui/styles.go, decl: 0, sub: 0, line: 0 } |  |  | 0.482 |
| walker |  | 7462 | 11 | Code::CodeKey { rung: Body, file: internal/ui/styles.go, decl: 5, sub: 0, line: 21 } |  |  | 0.482 |
| ns | 7567 |  | 215 | Hook configuration schema: HookConfig, HookMatcher, HookEntry | 7.2 |  | 0.496 |
| walker |  | 7681 | 219 | Code::CodeKey { rung: Names, file: internal/models/providers.go, decl: 0, sub: 0, line: 0 } |  |  | 0.499 |
| walker |  | 7749 | 68 | Code::CodeKey { rung: Decl, file: internal/models/providers.go, decl: 1, sub: 0, line: 28 } |  |  | 0.499 |
| ns | 7839 |  | 272 | Hook wire protocol: CommonInput and HookOutput | 7.3 |  | 0.495 |
| walker |  | 7845 | 96 | Code::CodeKey { rung: Decl, file: internal/models/providers.go, decl: 4, sub: 0, line: 124 } |  |  | 0.495 |
| walker |  | 7987 | 142 | Code::CodeKey { rung: Names, file: internal/builtin/bash.go, decl: 0, sub: 0, line: 0 } |  |  | 0.498 |
| walker |  | 7996 | 9 | Code::CodeKey { rung: Decl, file: internal/builtin/bash.go, decl: 1, sub: 0, line: 14 } |  |  | 0.499 |
| ns | 8030 |  | 191 | Per-event hook input structs | 7.4 |  | 0.508 |
| walker |  | 8313 | 317 | Code::CodeKey { rung: Names, file: cmd/root.go, decl: 0, sub: 0, line: 0 } |  |  | 0.508 |
| walker |  | 8326 | 13 | Code::CodeKey { rung: Decl, file: cmd/root.go, decl: 2, sub: 0, line: 67 } |  |  | 0.508 |
| ns | 8336 |  | 306 | Hook executor and validator symbol rosters | 7.5 |  | 0.498 |
| walker |  | 8337 | 11 | Code::CodeKey { rung: Body, file: cmd/root.go, decl: 3, sub: 0, line: 71 } |  |  | 0.498 |
| walker |  | 8411 | 74 | Code::CodeKey { rung: Decl, file: cmd/root.go, decl: 1, sub: 0, line: 27 } |  |  | 0.498 |
| ns | 8493 |  | 157 | README: hooks.yml file locations and the --no-hooks escape hatch | 7.6 | 1.6 | 0.495 |
| walker |  | 8558 | 147 | Code::CodeKey { rung: Names, file: internal/hooks/validator.go, decl: 0, sub: 0, line: 0 } |  |  | 0.499 |
| walker |  | 8569 | 11 | Code::CodeKey { rung: Decl, file: internal/hooks/validator.go, decl: 1, sub: 0, line: 10 } |  |  | 0.500 |
| ns | 8682 |  | 189 | Script mode: a worked frontmatter example and the variable rules | 8.1 |  | 0.494 |
| walker |  | 8711 | 142 | Code::CodeKey { rung: Decl, file: internal/models/generate_models.go, decl: 4, sub: 0, line: 60 } |  |  | 0.494 |
| ns | 8813 |  | 131 | Every top-level symbol of cmd/script.go (locations) | 8.2 |  | 0.502 |
| walker |  | 8958 | 247 | Markdown::Section { file: sdk/README.md, section_index: 3, keeps_default_concavity: true } |  |  | 0.502 |
| ns | 9111 |  | 298 | Session file format: Session, Metadata, Message and ToolCall fields | 8.3 |  | 0.494 |
| walker |  | 9169 | 211 | Code::CodeKey { rung: Names, file: internal/builtin/http.go, decl: 0, sub: 0, line: 0 } |  |  | 0.497 |
| walker |  | 9180 | 11 | Code::CodeKey { rung: Decl, file: internal/builtin/http.go, decl: 1, sub: 0, line: 22 } |  |  | 0.497 |
| walker |  | 9192 | 12 | Code::CodeKey { rung: Doc, file: internal/models/models_data.go, decl: 4, sub: 0, line: 32 } |  |  | 0.497 |
| walker |  | 9226 | 34 | Code::CodeKey { rung: Doc, file: sdk/types.go, decl: 2, sub: 0, line: 14 } |  |  | 0.497 |
| walker |  | 9388 | 162 | Code::CodeKey { rung: Names, file: internal/ui/spinner.go, decl: 0, sub: 0, line: 0 } |  |  | 0.497 |
| ns | 9405 |  | 294 | The public SDK surface: Options and every exported symbol | 8.4 |  | 0.509 |
| walker |  | 9440 | 52 | Code::CodeKey { rung: Decl, file: internal/ui/spinner.go, decl: 1, sub: 0, line: 16 } |  |  | 0.509 |
| walker |  | 9456 | 16 | Code::CodeKey { rung: Body, file: internal/ui/spinner.go, decl: 10, sub: 0, line: 149 } |  |  | 0.509 |
| walker |  | 9486 | 30 | Code::CodeKey { rung: Decl, file: internal/ui/spinner.go, decl: 2, sub: 0, line: 25 } |  |  | 0.509 |
| walker |  | 9501 | 15 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 2, sub: 0, line: 67 } |  |  | 0.509 |
| ns | 9607 |  | 202 | The complete slash-command table: names and descriptions | 8.5 |  | 0.504 |
| walker |  | 9713 | 212 | Code::CodeKey { rung: Names, file: internal/config/config.go, decl: 0, sub: 0, line: 0 } |  |  | 0.517 |
| walker |  | 9755 | 42 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 3, sub: 0, line: 109 } |  |  | 0.517 |
| walker |  | 9764 | 9 | Code::CodeKey { rung: Body, file: internal/config/config.go, decl: 14, sub: 0, line: 459 } |  |  | 0.517 |
| ns | 9828 |  | 221 | ui.SetupCLI: the AgentInterface contract and CLISetupOptions | 8.6 |  | 0.520 |
| walker |  | 9923 | 159 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 1, sub: 0, line: 10 } |  |  | 0.533 |
| walker |  | 9976 | 53 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 9, sub: 0, line: 423 } |  |  | 0.533 |
| ns | 9984 |  | 156 | Release packaging: the goreleaser build matrix | 9.1 |  | 0.527 |

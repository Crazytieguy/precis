Score(3000)=0.694 I=0.855 C=0.563 ns_rows≤3K=18/51 grid(1000/1442/2080/3000/4327/6240/9000)=0.852/0.857/0.783/0.694/0.590/0.510/0.508

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
| walker |  | 214 | 17 | Fs::DirListing { dir: internal/auth } |  |  | 0.647 |
| walker |  | 220 | 6 | Fs::DirListing { dir: examples } |  |  | 0.648 |
| ns | 260 |  | 53 | Module path, Go version and toolchain | 1.5 |  | 0.606 |
| walker |  | 283 | 63 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.842 |
| walker |  | 307 | 24 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.842 |
| walker |  | 360 | 53 | GoMod::Identity { file: go.mod } |  |  | 0.910 |
| walker |  | 387 | 27 | Code::CodeKey { rung: Names, file: main.go, decl: 0, sub: 0, line: 0 } |  |  | 0.910 |
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
| walker |  | 611 | 45 | Fs::DirListing { dir: internal/config } |  |  | 0.752 |
| walker |  | 644 | 33 | Markdown::HeadingsOutline { file: contribute/contribute.md } |  |  | 0.752 |
| ns | 648 |  | 114 | README feature list, second half | 1.8 |  | 0.704 |
| walker |  | 730 | 86 | Fs::DirListing { dir: internal/ui } |  |  | 0.720 |
| walker |  | 735 | 5 | Fs::DirListing { dir: internal/ui/progress } |  |  | 0.721 |
| walker |  | 751 | 16 | Code::CodeKey { rung: Names, file: internal/tokens/counter.go, decl: 0, sub: 0, line: 0 } |  |  | 0.721 |
| ns | 781 |  | 133 | Complete listings for the config / agent / tools / models packages | 1.9 |  | 0.726 |
| walker |  | 980 | 229 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: true } |  |  | 0.829 |
| walker |  | 990 | 10 | Markdown::Section { file: contribute/contribute.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.829 |
| ns | 999 |  | 218 | Complete listings for the builtin / hooks / session / auth / tokens / ui packages | 1.10 |  | 0.852 |
| walker |  | 1016 | 26 | Fs::DirListing { dir: internal/tools } |  |  | 0.891 |
| walker |  | 1036 | 20 | Markdown::Section { file: contribute/contribute.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.891 |
| walker |  | 1057 | 21 | Code::CodeKey { rung: Names, file: internal/ui/callbacks.go, decl: 0, sub: 0, line: 0 } |  |  | 0.891 |
| ns | 1089 |  | 90 | Complete listings for examples/, contribute/ and .github/ | 1.11 |  | 0.848 |
| walker |  | 1184 | 127 | Code::CodeKey { rung: Body, file: main.go, decl: 2, sub: 0, line: 14 } |  |  | 0.854 |
| walker |  | 1212 | 28 | Code::CodeKey { rung: Names, file: internal/tokens/init.go, decl: 0, sub: 0, line: 0 } |  |  | 0.854 |
| walker |  | 1227 | 15 | Code::CodeKey { rung: Body, file: internal/tokens/init.go, decl: 1, sub: 0, line: 21 } |  |  | 0.854 |
| walker |  | 1242 | 15 | Code::CodeKey { rung: Body, file: internal/tokens/init.go, decl: 2, sub: 0, line: 50 } |  |  | 0.854 |
| ns | 1243 |  | 154 | main.go entry point | 2.1 |  | 0.857 |
| walker |  | 1362 | 120 | Markdown::HeadingsOutline { file: sdk/README.md } |  |  | 0.857 |
| walker |  | 1375 | 13 | Markdown::Section { file: sdk/README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.857 |
| walker |  | 1408 | 33 | Markdown::Section { file: sdk/README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.857 |
| walker |  | 1504 | 96 | Markdown::Section { file: sdk/README.md, section_index: 6, keeps_default_concavity: true } |  |  | 0.812 |
| ns | 1504 |  | 261 | Complete cobra command tree: script, auth (login/logout/status), hooks (list/validate/init) | 2.2 |  | 0.812 |
| walker |  | 1534 | 30 | Code::CodeKey { rung: Names, file: internal/auth/browser.go, decl: 0, sub: 0, line: 0 } |  |  | 0.812 |
| walker |  | 1582 | 48 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.813 |
| walker |  | 1622 | 40 | Markdown::Section { file: README.md, section_index: 33, keeps_default_concavity: false } |  |  | 0.814 |
| walker |  | 1687 | 65 | Code::CodeKey { rung: Names, file: sdk/types.go, decl: 0, sub: 0, line: 0 } |  |  | 0.814 |
| walker |  | 1698 | 11 | Code::CodeKey { rung: Body, file: sdk/types.go, decl: 3, sub: 0, line: 18 } |  |  | 0.814 |
| walker |  | 1710 | 12 | Code::CodeKey { rung: Body, file: sdk/types.go, decl: 4, sub: 0, line: 24 } |  |  | 0.814 |
| walker |  | 1742 | 32 | Code::CodeKey { rung: Doc, file: sdk/types.go, decl: 3, sub: 0, line: 18 } |  |  | 0.814 |
| ns | 1754 |  | 250 | Persistent flag registration, part 1: config, system-prompt, model, debug, prompt, quiet | 2.3 |  | 0.783 |
| walker |  | 1775 | 33 | Code::CodeKey { rung: Doc, file: sdk/types.go, decl: 1, sub: 0, line: 10 } |  |  | 0.783 |
| walker |  | 1811 | 36 | Code::CodeKey { rung: Names, file: internal/agent/streaming.go, decl: 0, sub: 0, line: 0 } |  |  | 0.783 |
| ns | 2098 |  | 344 | Persistent flag registration, part 2: no-exit, max-steps, stream, compact, no-hooks, approve-tool-run, session flags | 2.4 |  | 0.745 |
| ns | 2224 |  | 126 | Provider and TLS flag registration | 2.5 |  | 0.738 |
| walker |  | 2325 | 514 | GoMod::File { file: go.mod } |  |  | 0.738 |
| walker |  | 2396 | 71 | Code::CodeKey { rung: Names, file: cmd/hooks.go, decl: 0, sub: 0, line: 0 } |  |  | 0.738 |
| walker |  | 2443 | 47 | Code::CodeKey { rung: Decl, file: cmd/hooks.go, decl: 1, sub: 0, line: 16 } |  |  | 0.738 |
| ns | 2502 |  | 278 | Generation-parameter and Ollama flag registration, with the hidden flag | 2.6 |  | 0.719 |
| walker |  | 2701 | 258 | Markdown::Section { file: sdk/README.md, section_index: 2, keeps_default_concavity: true } |  |  | 0.719 |
| ns | 2749 |  | 247 | MCPServerConfig: complete field set including the legacy block | 3.1 |  | 0.694 |
| walker |  | 2948 | 247 | Markdown::Section { file: sdk/README.md, section_index: 3, keeps_default_concavity: true } |  |  | 0.694 |
| walker |  | 2982 | 34 | Code::CodeKey { rung: Doc, file: sdk/types.go, decl: 2, sub: 0, line: 14 } |  |  | 0.694 |
| walker |  | 3040 | 58 | Markdown::Section { file: README.md, section_index: 31, keeps_default_concavity: false } |  |  | 0.696 |
| ns | 3069 |  | 320 | Config struct: application-level keys | 3.2 |  | 0.675 |
| walker |  | 3238 | 198 | Code::CodeKey { rung: ModuleDoc, file: internal/tokens/anthropic.go, decl: 0, sub: 0, line: 0 } |  |  | 0.675 |
| ns | 3250 |  | 181 | Config struct: generation-parameter and TLS keys | 3.3 |  | 0.660 |
| walker |  | 3268 | 30 | Code::CodeKey { rung: Body, file: internal/tokens/counter.go, decl: 1, sub: 0, line: 21 } |  |  | 0.660 |
| walker |  | 3325 | 57 | Code::CodeKey { rung: Names, file: internal/config/merger.go, decl: 0, sub: 0, line: 0 } |  |  | 0.660 |
| walker |  | 3356 | 31 | Code::CodeKey { rung: Body, file: internal/auth/browser.go, decl: 2, sub: 0, line: 34 } |  |  | 0.660 |
| walker |  | 3416 | 60 | Code::CodeKey { rung: Names, file: internal/ui/commands.go, decl: 0, sub: 0, line: 0 } |  |  | 0.660 |
| walker |  | 3467 | 51 | Code::CodeKey { rung: Decl, file: internal/ui/commands.go, decl: 1, sub: 0, line: 6 } |  |  | 0.660 |
| ns | 3514 |  | 264 | GetTransportType: type-to-transport mapping and legacy inference | 3.4 |  | 0.627 |
| walker |  | 3528 | 61 | Code::CodeKey { rung: Names, file: internal/models/models_data.go, decl: 0, sub: 0, line: 0 } |  |  | 0.627 |
| walker |  | 3548 | 20 | Code::CodeKey { rung: Decl, file: internal/models/models_data.go, decl: 3, sub: 0, line: 26 } |  |  | 0.627 |
| walker |  | 3594 | 46 | Code::CodeKey { rung: Decl, file: internal/models/models_data.go, decl: 2, sub: 0, line: 18 } |  |  | 0.627 |
| walker |  | 3646 | 52 | Code::CodeKey { rung: Decl, file: internal/models/models_data.go, decl: 4, sub: 0, line: 32 } |  |  | 0.627 |
| walker |  | 3712 | 66 | Code::CodeKey { rung: Decl, file: internal/models/models_data.go, decl: 1, sub: 0, line: 7 } |  |  | 0.627 |
| walker |  | 3724 | 12 | Code::CodeKey { rung: Doc, file: internal/models/models_data.go, decl: 2, sub: 0, line: 18 } |  |  | 0.627 |
| walker |  | 3736 | 12 | Code::CodeKey { rung: Doc, file: internal/models/models_data.go, decl: 4, sub: 0, line: 32 } |  |  | 0.627 |
| walker |  | 3802 | 66 | Markdown::Section { file: README.md, section_index: 34, keeps_default_concavity: false } |  |  | 0.629 |
| walker |  | 3808 | 6 | Fs::DirListing { dir: sdk/examples } |  |  | 0.641 |
| walker |  | 3875 | 67 | Code::CodeKey { rung: Names, file: internal/ui/factory.go, decl: 0, sub: 0, line: 0 } |  |  | 0.641 |
| walker |  | 3928 | 53 | Code::CodeKey { rung: Decl, file: internal/ui/factory.go, decl: 1, sub: 0, line: 13 } |  |  | 0.641 |
| ns | 3938 |  | 424 | Config.Validate: required fields per transport and filter exclusivity | 3.5 |  | 0.609 |
| walker |  | 4008 | 80 | Code::CodeKey { rung: Decl, file: internal/ui/factory.go, decl: 2, sub: 0, line: 22 } |  |  | 0.610 |
| walker |  | 4041 | 33 | Code::CodeKey { rung: Doc, file: internal/ui/factory.go, decl: 1, sub: 0, line: 13 } |  |  | 0.610 |
| walker |  | 4075 | 34 | Code::CodeKey { rung: Doc, file: sdk/types.go, decl: 4, sub: 0, line: 24 } |  |  | 0.610 |
| walker |  | 4115 | 40 | Plaintext::DeclSurface { file: contribute/build.sh } |  |  | 0.610 |
| ns | 4141 |  | 203 | Substitution engine: the two regexes plus every symbol in substitution.go | 3.6 |  | 0.598 |
| walker |  | 4218 | 103 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.600 |
| walker |  | 4232 | 14 | Code::CodeKey { rung: Doc, file: internal/models/models_data.go, decl: 1, sub: 0, line: 7 } |  |  | 0.600 |
| ns | 4295 |  | 154 | Remaining top-level symbols of internal/config/config.go and all of merger.go (locations) | 3.7 |  | 0.590 |
| walker |  | 4306 | 74 | Code::CodeKey { rung: Names, file: internal/hooks/schemas.go, decl: 0, sub: 0, line: 0 } |  |  | 0.590 |
| walker |  | 4330 | 24 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 4, sub: 0, line: 42 } |  |  | 0.590 |
| walker |  | 4374 | 44 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 2, sub: 0, line: 23 } |  |  | 0.590 |
| walker |  | 4435 | 61 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 3, sub: 0, line: 32 } |  |  | 0.590 |
| walker |  | 4524 | 89 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 6, sub: 0, line: 62 } |  |  | 0.590 |
| ns | 4541 |  | 246 | Agent struct and its seven callback handler types | 4.1 |  | 0.575 |
| walker |  | 4631 | 107 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 5, sub: 0, line: 50 } |  |  | 0.576 |
| walker |  | 4790 | 159 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 1, sub: 0, line: 10 } |  |  | 0.577 |
| ns | 4815 |  | 274 | Every top-level symbol of internal/agent/agent.go (locations) | 4.2 |  | 0.563 |
| walker |  | 4864 | 74 | Code::CodeKey { rung: Names, file: internal/ui/fuzzy.go, decl: 0, sub: 0, line: 0 } |  |  | 0.563 |
| walker |  | 4887 | 23 | Code::CodeKey { rung: Decl, file: internal/ui/fuzzy.go, decl: 1, sub: 0, line: 10 } |  |  | 0.563 |
| walker |  | 4963 | 76 | Code::CodeKey { rung: Names, file: internal/models/generate_models.go, decl: 0, sub: 0, line: 0 } |  |  | 0.563 |
| ns | 5007 |  | 192 | The tool-calling loop: step bound, tool-call branch and the approval gate | 4.3 | 4.2 | 0.553 |
| walker |  | 5020 | 57 | Code::CodeKey { rung: Decl, file: internal/models/generate_models.go, decl: 3, sub: 0, line: 50 } |  |  | 0.553 |
| walker |  | 5151 | 131 | Code::CodeKey { rung: Decl, file: internal/models/generate_models.go, decl: 2, sub: 0, line: 37 } |  |  | 0.553 |
| ns | 5186 |  | 179 | agent factory: AgentCreationOptions fields and both functions | 4.4 |  | 0.544 |
| walker |  | 5293 | 142 | Code::CodeKey { rung: Decl, file: internal/models/generate_models.go, decl: 4, sub: 0, line: 60 } |  |  | 0.544 |
| walker |  | 5324 | 31 | Code::CodeKey { rung: Doc, file: internal/models/generate_models.go, decl: 2, sub: 0, line: 37 } |  |  | 0.544 |
| walker |  | 5523 | 199 | Code::CodeKey { rung: Decl, file: internal/models/generate_models.go, decl: 1, sub: 0, line: 18 } |  |  | 0.544 |
| ns | 5549 |  | 363 | Every top-level symbol of internal/tools/mcp.go (locations) | 4.5 |  | 0.529 |
| walker |  | 5566 | 43 | Code::CodeKey { rung: Doc, file: internal/ui/factory.go, decl: 2, sub: 0, line: 22 } |  |  | 0.529 |
| walker |  | 5645 | 79 | Code::CodeKey { rung: Names, file: internal/ui/debug_logger.go, decl: 0, sub: 0, line: 0 } |  |  | 0.529 |
| walker |  | 5659 | 14 | Code::CodeKey { rung: Decl, file: internal/ui/debug_logger.go, decl: 1, sub: 0, line: 13 } |  |  | 0.529 |
| walker |  | 5673 | 14 | Code::CodeKey { rung: Body, file: internal/ui/debug_logger.go, decl: 4, sub: 0, line: 84 } |  |  | 0.529 |
| walker |  | 5688 | 15 | Code::CodeKey { rung: Body, file: internal/ui/debug_logger.go, decl: 2, sub: 0, line: 20 } |  |  | 0.529 |
| walker |  | 5732 | 44 | Code::CodeKey { rung: Doc, file: internal/ui/fuzzy.go, decl: 1, sub: 0, line: 10 } |  |  | 0.529 |
| ns | 5765 |  | 216 | AgenticLoopConfig head and every mode-driving function in cmd/root.go (locations) | 4.6 |  | 0.518 |
| walker |  | 5812 | 80 | Code::CodeKey { rung: Names, file: internal/agent/factory.go, decl: 0, sub: 0, line: 0 } |  |  | 0.520 |
| walker |  | 5850 | 38 | Code::CodeKey { rung: Doc, file: internal/agent/factory.go, decl: 1, sub: 0, line: 15 } |  |  | 0.520 |
| walker |  | 5854 | 4 | Fs::DirListing { dir: sdk/examples/basic } |  |  | 0.525 |
| walker |  | 5858 | 4 | Fs::DirListing { dir: sdk/examples/scripting } |  |  | 0.531 |
| ns | 5907 |  | 142 | CreateProvider: the complete list of supported providers | 5.1 |  | 0.523 |
| walker |  | 6018 | 160 | Code::CodeKey { rung: Names, file: cmd/auth.go, decl: 0, sub: 0, line: 0 } |  |  | 0.523 |
| walker |  | 6137 | 119 | Code::CodeKey { rung: Decl, file: cmd/auth.go, decl: 4, sub: 0, line: 78 } |  |  | 0.525 |
| ns | 6235 |  | 328 | Every top-level symbol of internal/models/providers.go (locations) | 5.2 | 5.1 | 0.510 |
| ns | 6385 |  | 150 | ModelsRegistry: model validation and suggestion API | 5.3 |  | 0.504 |
| walker |  | 6405 | 268 | Code::CodeKey { rung: Decl, file: internal/agent/factory.go, decl: 2, sub: 0, line: 19 } |  |  | 0.523 |
| walker |  | 6437 | 32 | Code::CodeKey { rung: Doc, file: internal/agent/factory.go, decl: 2, sub: 0, line: 19 } |  |  | 0.523 |
| ns | 6477 |  | 92 | The generated model catalogue: generator types and the DO-NOT-EDIT header | 5.4 |  | 0.522 |
| walker |  | 6484 | 47 | Code::CodeKey { rung: Doc, file: internal/config/merger.go, decl: 2, sub: 0, line: 28 } |  |  | 0.522 |
| walker |  | 6532 | 48 | Code::CodeKey { rung: Doc, file: internal/ui/commands.go, decl: 1, sub: 0, line: 6 } |  |  | 0.522 |
| walker |  | 6620 | 88 | Code::CodeKey { rung: Names, file: internal/hooks/config.go, decl: 0, sub: 0, line: 0 } |  |  | 0.522 |
| walker |  | 6647 | 27 | Code::CodeKey { rung: Decl, file: internal/hooks/config.go, decl: 1, sub: 0, line: 14 } |  |  | 0.523 |
| ns | 6653 |  | 176 | Builtin server registry: the complete set of in-process servers | 6.1 |  | 0.513 |
| walker |  | 6702 | 55 | Code::CodeKey { rung: Decl, file: internal/hooks/config.go, decl: 3, sub: 0, line: 30 } |  |  | 0.514 |
| walker |  | 6767 | 65 | Code::CodeKey { rung: Decl, file: internal/hooks/config.go, decl: 2, sub: 0, line: 21 } |  |  | 0.514 |
| walker |  | 6796 | 29 | Code::CodeKey { rung: Doc, file: internal/hooks/config.go, decl: 1, sub: 0, line: 14 } |  |  | 0.514 |
| ns | 6825 |  | 172 | Every top-level symbol of internal/builtin/registry.go (locations) | 6.2 | 6.1 | 0.508 |
| walker |  | 6957 | 161 | Code::CodeKey { rung: Decl, file: cmd/auth.go, decl: 1, sub: 0, line: 17 } |  |  | 0.510 |
| ns | 7042 |  | 217 | Bash builtin: output/timeout limits and the complete banned-command list | 6.3 |  | 0.498 |
| walker |  | 7124 | 167 | Code::CodeKey { rung: Decl, file: cmd/auth.go, decl: 3, sub: 0, line: 58 } |  |  | 0.501 |
| walker |  | 7139 | 15 | Code::CodeKey { rung: Doc, file: internal/ui/fuzzy.go, decl: 4, sub: 0, line: 117 } |  |  | 0.501 |
| walker |  | 7231 | 92 | Markdown::Section { file: README.md, section_index: 32, keeps_default_concavity: false } |  |  | 0.503 |
| ns | 7240 |  | 198 | The http builtin: its four tools and every symbol in http.go (locations) | 6.4 |  | 0.495 |
| walker |  | 7283 | 52 | Code::CodeKey { rung: Doc, file: internal/config/merger.go, decl: 1, sub: 0, line: 13 } |  |  | 0.495 |
| ns | 7352 |  | 112 | HookEvent: the complete set of hook events | 7.1 |  | 0.492 |
| walker |  | 7458 | 175 | Code::CodeKey { rung: Decl, file: cmd/auth.go, decl: 2, sub: 0, line: 37 } |  |  | 0.495 |
| walker |  | 7472 | 14 | Code::CodeKey { rung: Doc, file: internal/models/models_data.go, decl: 3, sub: 0, line: 26 } |  |  | 0.495 |
| walker |  | 7526 | 54 | Code::CodeKey { rung: Doc, file: internal/ui/debug_logger.go, decl: 4, sub: 0, line: 84 } |  |  | 0.495 |
| walker |  | 7542 | 16 | Code::CodeKey { rung: Doc, file: internal/ui/fuzzy.go, decl: 3, sub: 0, line: 60 } |  |  | 0.495 |
| ns | 7567 |  | 215 | Hook configuration schema: HookConfig, HookMatcher, HookEntry | 7.2 |  | 0.509 |
| walker |  | 7597 | 55 | Code::CodeKey { rung: Doc, file: internal/ui/commands.go, decl: 3, sub: 0, line: 65 } |  |  | 0.509 |
| walker |  | 7782 | 185 | Code::CodeKey { rung: Decl, file: cmd/hooks.go, decl: 3, sub: 0, line: 57 } |  |  | 0.512 |
| walker |  | 7839 | 57 | Code::CodeKey { rung: Doc, file: internal/agent/factory.go, decl: 3, sub: 0, line: 43 } |  |  | 0.524 |
| ns | 7839 |  | 272 | Hook wire protocol: CommonInput and HookOutput | 7.3 |  | 0.524 |
| walker |  | 7943 | 104 | Code::CodeKey { rung: Names, file: internal/ui/tool_approval_input.go, decl: 0, sub: 0, line: 0 } |  |  | 0.524 |
| walker |  | 7952 | 9 | Code::CodeKey { rung: Body, file: internal/ui/tool_approval_input.go, decl: 3, sub: 0, line: 48 } |  |  | 0.524 |
| walker |  | 8028 | 76 | Code::CodeKey { rung: Decl, file: internal/ui/tool_approval_input.go, decl: 1, sub: 0, line: 12 } |  |  | 0.524 |
| ns | 8030 |  | 191 | Per-event hook input structs | 7.4 |  | 0.533 |
| walker |  | 8086 | 58 | Code::CodeKey { rung: Doc, file: internal/ui/commands.go, decl: 4, sub: 0, line: 83 } |  |  | 0.533 |
| walker |  | 8317 | 231 | Code::CodeKey { rung: Names, file: internal/ui/cli.go, decl: 0, sub: 0, line: 0 } |  |  | 0.533 |
| walker |  | 8329 | 12 | Code::CodeKey { rung: Body, file: internal/ui/cli.go, decl: 5, sub: 0, line: 65 } |  |  | 0.533 |
| ns | 8336 |  | 306 | Hook executor and validator symbol rosters | 7.5 |  | 0.523 |
| walker |  | 8343 | 14 | Code::CodeKey { rung: Body, file: internal/ui/cli.go, decl: 10, sub: 0, line: 152 } |  |  | 0.523 |
| ns | 8493 |  | 157 | README: hooks.yml file locations and the --no-hooks escape hatch | 7.6 | 1.6 | 0.519 |
| walker |  | 8504 | 161 | Code::CodeKey { rung: Decl, file: internal/ui/cli.go, decl: 2, sub: 0, line: 22 } |  |  | 0.519 |
| walker |  | 8542 | 38 | Code::CodeKey { rung: Doc, file: internal/ui/cli.go, decl: 5, sub: 0, line: 65 } |  |  | 0.519 |
| walker |  | 8581 | 39 | Code::CodeKey { rung: Doc, file: internal/ui/cli.go, decl: 6, sub: 0, line: 71 } |  |  | 0.519 |
| walker |  | 8641 | 60 | Code::CodeKey { rung: Doc, file: internal/auth/browser.go, decl: 2, sub: 0, line: 34 } |  |  | 0.519 |
| ns | 8682 |  | 189 | Script mode: a worked frontmatter example and the variable rules | 8.1 |  | 0.513 |
| walker |  | 8703 | 62 | Plaintext::DeclSurface { file: contribute/boost.sh } |  |  | 0.513 |
| ns | 8813 |  | 131 | Every top-level symbol of cmd/script.go (locations) | 8.2 |  | 0.508 |
| walker |  | 8916 | 213 | Code::CodeKey { rung: Names, file: sdk/mcphost.go, decl: 0, sub: 0, line: 0 } |  |  | 0.508 |
| walker |  | 8951 | 35 | Code::CodeKey { rung: Decl, file: sdk/mcphost.go, decl: 1, sub: 0, line: 19 } |  |  | 0.508 |
| walker |  | 8960 | 9 | Code::CodeKey { rung: Body, file: sdk/mcphost.go, decl: 6, sub: 0, line: 201 } |  |  | 0.508 |
| walker |  | 8969 | 9 | Code::CodeKey { rung: Body, file: sdk/mcphost.go, decl: 10, sub: 0, line: 230 } |  |  | 0.508 |
| walker |  | 8978 | 9 | Code::CodeKey { rung: Body, file: sdk/mcphost.go, decl: 11, sub: 0, line: 237 } |  |  | 0.508 |
| walker |  | 9046 | 68 | Code::CodeKey { rung: Decl, file: sdk/mcphost.go, decl: 5, sub: 0, line: 163 } |  |  | 0.508 |
| ns | 9111 |  | 298 | Session file format: Session, Metadata, Message and ToolCall fields | 8.3 |  | 0.500 |
| walker |  | 9153 | 107 | Code::CodeKey { rung: Decl, file: sdk/mcphost.go, decl: 2, sub: 0, line: 28 } |  |  | 0.501 |
| walker |  | 9165 | 12 | Code::CodeKey { rung: Body, file: sdk/mcphost.go, decl: 9, sub: 0, line: 224 } |  |  | 0.501 |
| walker |  | 9181 | 16 | Code::CodeKey { rung: Body, file: sdk/mcphost.go, decl: 8, sub: 0, line: 218 } |  |  | 0.501 |
| ns | 9405 |  | 294 | The public SDK surface: Options and every exported symbol | 8.4 |  | 0.513 |
| walker |  | 9461 | 280 | Code::CodeKey { rung: Names, file: cmd/script.go, decl: 0, sub: 0, line: 0 } |  |  | 0.526 |
| walker |  | 9526 | 65 | Code::CodeKey { rung: Decl, file: cmd/script.go, decl: 9, sub: 0, line: 423 } |  |  | 0.526 |
| walker |  | 9537 | 11 | Code::CodeKey { rung: Body, file: cmd/script.go, decl: 2, sub: 0, line: 78 } |  |  | 0.526 |
| walker |  | 9590 | 53 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 9, sub: 0, line: 423 } |  |  | 0.526 |
| walker |  | 9606 | 16 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 4, sub: 0, line: 148 } |  |  | 0.526 |
| ns | 9607 |  | 202 | The complete slash-command table: names and descriptions | 8.5 |  | 0.521 |
| walker |  | 9622 | 16 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 7, sub: 0, line: 279 } |  |  | 0.521 |
| walker |  | 9736 | 114 | Code::CodeKey { rung: Names, file: internal/hooks/events.go, decl: 0, sub: 0, line: 0 } |  |  | 0.527 |
| ns | 9828 |  | 221 | ui.SetupCLI: the AgentInterface contract and CLISetupOptions | 8.6 |  | 0.536 |
| walker |  | 9835 | 99 | Code::CodeKey { rung: Decl, file: internal/hooks/events.go, decl: 2, sub: 0, line: 7 } |  |  | 0.539 |
| walker |  | 9852 | 17 | Code::CodeKey { rung: Body, file: internal/hooks/events.go, decl: 4, sub: 0, line: 34 } |  |  | 0.539 |
| walker |  | 9891 | 39 | Code::CodeKey { rung: Doc, file: internal/hooks/events.go, decl: 3, sub: 0, line: 23 } |  |  | 0.539 |
| walker |  | 9932 | 41 | Code::CodeKey { rung: Doc, file: internal/hooks/events.go, decl: 1, sub: 0, line: 5 } |  |  | 0.539 |
| walker |  | 9950 | 18 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 14, sub: 0, line: 511 } |  |  | 0.539 |
| walker |  | 9982 | 32 | Code::CodeKey { rung: Doc, file: internal/models/generate_models.go, decl: 3, sub: 0, line: 50 } |  |  | 0.539 |
| ns | 9984 |  | 156 | Release packaging: the goreleaser build matrix | 9.1 |  | 0.532 |
| walker |  | 9996 | 14 | Code::CodeKey { rung: Doc, file: internal/hooks/config.go, decl: 6, sub: 0, line: 113 } |  |  | 0.532 |

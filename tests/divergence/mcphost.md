Score(3000)=0.714 I=0.875 C=0.582 ns_rows≤3K=18/51 grid(1000/1442/2080/3000/4327/6240/9000)=0.891/0.857/0.783/0.714/0.614/0.521/0.506

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
| walker |  | 395 | 35 | Fs::DirListing { dir: internal/models } |  |  | 0.915 |
| walker |  | 400 | 5 | Fs::DirListing { dir: internal/models/anthropic } |  |  | 0.915 |
| walker |  | 405 | 5 | Fs::DirListing { dir: internal/models/gemini } |  |  | 0.916 |
| walker |  | 410 | 5 | Fs::DirListing { dir: internal/models/openai } |  |  | 0.917 |
| ns | 427 |  | 167 | All README section headings (locations only) | 1.6 |  | 0.785 |
| walker |  | 449 | 39 | Fs::DirListing { dir: internal/hooks } |  |  | 0.789 |
| walker |  | 459 | 10 | Fs::DirListing { dir: internal/hooks/testdata } |  |  | 0.790 |
| walker |  | 499 | 40 | Markdown::ReadmeHeadline { file: sdk/README.md } |  |  | 0.790 |
| ns | 534 |  | 107 | README feature list, first half | 1.7 | 1.6 | 0.734 |
| walker |  | 539 | 40 | Fs::DirListing { dir: internal/builtin } |  |  | 0.741 |
| walker |  | 584 | 45 | Fs::DirListing { dir: internal/config } |  |  | 0.752 |
| walker |  | 617 | 33 | Markdown::HeadingsOutline { file: contribute/contribute.md } |  |  | 0.752 |
| ns | 648 |  | 114 | README feature list, second half | 1.8 |  | 0.704 |
| walker |  | 703 | 86 | Fs::DirListing { dir: internal/ui } |  |  | 0.720 |
| walker |  | 708 | 5 | Fs::DirListing { dir: internal/ui/progress } |  |  | 0.721 |
| walker |  | 735 | 27 | Code::CodeKey { rung: Names, file: main.go, decl: 0, sub: 0, line: 0 } |  |  | 0.721 |
| ns | 781 |  | 133 | Complete listings for the config / agent / tools / models packages | 1.9 |  | 0.726 |
| walker |  | 964 | 229 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: true } |  |  | 0.829 |
| walker |  | 974 | 10 | Markdown::Section { file: contribute/contribute.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.829 |
| ns | 999 |  | 218 | Complete listings for the builtin / hooks / session / auth / tokens / ui packages | 1.10 |  | 0.852 |
| walker |  | 1000 | 26 | Fs::DirListing { dir: internal/tools } |  |  | 0.891 |
| walker |  | 1020 | 20 | Markdown::Section { file: contribute/contribute.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.891 |
| ns | 1089 |  | 90 | Complete listings for examples/, contribute/ and .github/ | 1.11 |  | 0.848 |
| walker |  | 1147 | 127 | Code::CodeKey { rung: Body, file: main.go, decl: 2, sub: 0, line: 14 } |  |  | 0.854 |
| ns | 1243 |  | 154 | main.go entry point | 2.1 |  | 0.857 |
| walker |  | 1267 | 120 | Markdown::HeadingsOutline { file: sdk/README.md } |  |  | 0.857 |
| walker |  | 1280 | 13 | Markdown::Section { file: sdk/README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.857 |
| walker |  | 1313 | 33 | Markdown::Section { file: sdk/README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.857 |
| walker |  | 1409 | 96 | Markdown::Section { file: sdk/README.md, section_index: 6, keeps_default_concavity: true } |  |  | 0.857 |
| walker |  | 1457 | 48 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.858 |
| walker |  | 1497 | 40 | Markdown::Section { file: README.md, section_index: 33, keeps_default_concavity: false } |  |  | 0.859 |
| ns | 1504 |  | 261 | Complete cobra command tree: script, auth (login/logout/status), hooks (list/validate/init) | 2.2 |  | 0.814 |
| ns | 1754 |  | 250 | Persistent flag registration, part 1: config, system-prompt, model, debug, prompt, quiet | 2.3 |  | 0.783 |
| walker |  | 2011 | 514 | GoMod::File { file: go.mod } |  |  | 0.783 |
| ns | 2098 |  | 344 | Persistent flag registration, part 2: no-exit, max-steps, stream, compact, no-hooks, approve-tool-run, session flags | 2.4 |  | 0.745 |
| ns | 2224 |  | 126 | Provider and TLS flag registration | 2.5 |  | 0.738 |
| walker |  | 2269 | 258 | Markdown::Section { file: sdk/README.md, section_index: 2, keeps_default_concavity: true } |  |  | 0.738 |
| ns | 2502 |  | 278 | Generation-parameter and Ollama flag registration, with the hidden flag | 2.6 |  | 0.719 |
| walker |  | 2516 | 247 | Markdown::Section { file: sdk/README.md, section_index: 3, keeps_default_concavity: true } |  |  | 0.719 |
| walker |  | 2574 | 58 | Markdown::Section { file: README.md, section_index: 31, keeps_default_concavity: false } |  |  | 0.721 |
| ns | 2749 |  | 247 | MCPServerConfig: complete field set including the legacy block | 3.1 |  | 0.696 |
| walker |  | 2772 | 198 | Code::CodeKey { rung: ModuleDoc, file: internal/tokens/anthropic.go, decl: 0, sub: 0, line: 0 } |  |  | 0.696 |
| walker |  | 2838 | 66 | Markdown::Section { file: README.md, section_index: 34, keeps_default_concavity: false } |  |  | 0.698 |
| walker |  | 2844 | 6 | Fs::DirListing { dir: sdk/examples } |  |  | 0.711 |
| walker |  | 2884 | 40 | Plaintext::DeclSurface { file: contribute/build.sh } |  |  | 0.711 |
| walker |  | 2987 | 103 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.714 |
| walker |  | 3058 | 71 | Code::CodeKey { rung: Names, file: cmd/hooks.go, decl: 0, sub: 0, line: 0 } |  |  | 0.714 |
| ns | 3069 |  | 320 | Config struct: application-level keys | 3.2 |  | 0.692 |
| walker |  | 3105 | 47 | Code::CodeKey { rung: Decl, file: cmd/hooks.go, decl: 1, sub: 0, line: 16 } |  |  | 0.692 |
| walker |  | 3109 | 4 | Fs::DirListing { dir: sdk/examples/basic } |  |  | 0.700 |
| walker |  | 3113 | 4 | Fs::DirListing { dir: sdk/examples/scripting } |  |  | 0.707 |
| ns | 3250 |  | 181 | Config struct: generation-parameter and TLS keys | 3.3 |  | 0.692 |
| walker |  | 3273 | 160 | Code::CodeKey { rung: Names, file: cmd/auth.go, decl: 0, sub: 0, line: 0 } |  |  | 0.692 |
| walker |  | 3392 | 119 | Code::CodeKey { rung: Decl, file: cmd/auth.go, decl: 4, sub: 0, line: 78 } |  |  | 0.693 |
| ns | 3514 |  | 264 | GetTransportType: type-to-transport mapping and legacy inference | 3.4 |  | 0.658 |
| walker |  | 3553 | 161 | Code::CodeKey { rung: Decl, file: cmd/auth.go, decl: 1, sub: 0, line: 17 } |  |  | 0.660 |
| walker |  | 3618 | 65 | Code::CodeKey { rung: Names, file: sdk/types.go, decl: 0, sub: 0, line: 0 } |  |  | 0.660 |
| walker |  | 3629 | 11 | Code::CodeKey { rung: Body, file: sdk/types.go, decl: 3, sub: 0, line: 18 } |  |  | 0.660 |
| walker |  | 3641 | 12 | Code::CodeKey { rung: Body, file: sdk/types.go, decl: 4, sub: 0, line: 24 } |  |  | 0.660 |
| walker |  | 3673 | 32 | Code::CodeKey { rung: Doc, file: sdk/types.go, decl: 3, sub: 0, line: 18 } |  |  | 0.660 |
| walker |  | 3706 | 33 | Code::CodeKey { rung: Doc, file: sdk/types.go, decl: 1, sub: 0, line: 10 } |  |  | 0.660 |
| walker |  | 3740 | 34 | Code::CodeKey { rung: Doc, file: sdk/types.go, decl: 2, sub: 0, line: 14 } |  |  | 0.660 |
| walker |  | 3774 | 34 | Code::CodeKey { rung: Doc, file: sdk/types.go, decl: 4, sub: 0, line: 24 } |  |  | 0.660 |
| ns | 3938 |  | 424 | Config.Validate: required fields per transport and filter exclusivity | 3.5 |  | 0.628 |
| walker |  | 3941 | 167 | Code::CodeKey { rung: Decl, file: cmd/auth.go, decl: 3, sub: 0, line: 58 } |  |  | 0.632 |
| walker |  | 4033 | 92 | Markdown::Section { file: README.md, section_index: 32, keeps_default_concavity: false } |  |  | 0.634 |
| ns | 4141 |  | 203 | Substitution engine: the two regexes plus every symbol in substitution.go | 3.6 |  | 0.622 |
| walker |  | 4208 | 175 | Code::CodeKey { rung: Decl, file: cmd/auth.go, decl: 2, sub: 0, line: 37 } |  |  | 0.627 |
| ns | 4295 |  | 154 | Remaining top-level symbols of internal/config/config.go and all of merger.go (locations) | 3.7 |  | 0.614 |
| walker |  | 4393 | 185 | Code::CodeKey { rung: Decl, file: cmd/hooks.go, decl: 3, sub: 0, line: 57 } |  |  | 0.619 |
| ns | 4541 |  | 246 | Agent struct and its seven callback handler types | 4.1 |  | 0.604 |
| walker |  | 4606 | 213 | Code::CodeKey { rung: Names, file: sdk/mcphost.go, decl: 0, sub: 0, line: 0 } |  |  | 0.604 |
| walker |  | 4641 | 35 | Code::CodeKey { rung: Decl, file: sdk/mcphost.go, decl: 1, sub: 0, line: 19 } |  |  | 0.604 |
| walker |  | 4650 | 9 | Code::CodeKey { rung: Body, file: sdk/mcphost.go, decl: 6, sub: 0, line: 201 } |  |  | 0.604 |
| walker |  | 4659 | 9 | Code::CodeKey { rung: Body, file: sdk/mcphost.go, decl: 10, sub: 0, line: 230 } |  |  | 0.604 |
| walker |  | 4668 | 9 | Code::CodeKey { rung: Body, file: sdk/mcphost.go, decl: 11, sub: 0, line: 237 } |  |  | 0.604 |
| walker |  | 4736 | 68 | Code::CodeKey { rung: Decl, file: sdk/mcphost.go, decl: 5, sub: 0, line: 163 } |  |  | 0.604 |
| ns | 4815 |  | 274 | Every top-level symbol of internal/agent/agent.go (locations) | 4.2 |  | 0.589 |
| walker |  | 4843 | 107 | Code::CodeKey { rung: Decl, file: sdk/mcphost.go, decl: 2, sub: 0, line: 28 } |  |  | 0.590 |
| walker |  | 4872 | 29 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 6, sub: 0, line: 201 } |  |  | 0.590 |
| walker |  | 4901 | 29 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 9, sub: 0, line: 224 } |  |  | 0.590 |
| walker |  | 4938 | 37 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 8, sub: 0, line: 218 } |  |  | 0.590 |
| walker |  | 4977 | 39 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 7, sub: 0, line: 207 } |  |  | 0.590 |
| ns | 5007 |  | 192 | The tool-calling loop: step bound, tool-call branch and the approval gate | 4.3 | 4.2 | 0.579 |
| walker |  | 5024 | 47 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 2, sub: 0, line: 28 } |  |  | 0.579 |
| walker |  | 5072 | 48 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 11, sub: 0, line: 237 } |  |  | 0.579 |
| walker |  | 5121 | 49 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 10, sub: 0, line: 230 } |  |  | 0.579 |
| walker |  | 5173 | 52 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 1, sub: 0, line: 19 } |  |  | 0.579 |
| ns | 5186 |  | 179 | agent factory: AgentCreationOptions fields and both functions | 4.4 |  | 0.570 |
| walker |  | 5229 | 56 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 5, sub: 0, line: 163 } |  |  | 0.570 |
| walker |  | 5286 | 57 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 3, sub: 0, line: 40 } |  |  | 0.570 |
| walker |  | 5344 | 58 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 4, sub: 0, line: 130 } |  |  | 0.570 |
| walker |  | 5406 | 62 | Plaintext::DeclSurface { file: contribute/boost.sh } |  |  | 0.570 |
| ns | 5549 |  | 363 | Every top-level symbol of internal/tools/mcp.go (locations) | 4.5 |  | 0.554 |
| walker |  | 5686 | 280 | Code::CodeKey { rung: Names, file: cmd/script.go, decl: 0, sub: 0, line: 0 } |  |  | 0.555 |
| walker |  | 5751 | 65 | Code::CodeKey { rung: Decl, file: cmd/script.go, decl: 9, sub: 0, line: 423 } |  |  | 0.555 |
| walker |  | 5762 | 11 | Code::CodeKey { rung: Body, file: cmd/script.go, decl: 2, sub: 0, line: 78 } |  |  | 0.555 |
| ns | 5765 |  | 216 | AgenticLoopConfig head and every mode-driving function in cmd/root.go (locations) | 4.6 |  | 0.543 |
| walker |  | 5815 | 53 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 9, sub: 0, line: 423 } |  |  | 0.543 |
| walker |  | 5831 | 16 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 4, sub: 0, line: 148 } |  |  | 0.543 |
| walker |  | 5847 | 16 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 7, sub: 0, line: 279 } |  |  | 0.543 |
| walker |  | 5865 | 18 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 14, sub: 0, line: 511 } |  |  | 0.543 |
| walker |  | 5884 | 19 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 8, sub: 0, line: 288 } |  |  | 0.543 |
| walker |  | 5904 | 20 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 6, sub: 0, line: 255 } |  |  | 0.543 |
| ns | 5907 |  | 142 | CreateProvider: the complete list of supported providers | 5.1 |  | 0.536 |
| walker |  | 5935 | 31 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 12, sub: 0, line: 480 } |  |  | 0.536 |
| walker |  | 5968 | 33 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 10, sub: 0, line: 431 } |  |  | 0.536 |
| walker |  | 6004 | 36 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 11, sub: 0, line: 442 } |  |  | 0.536 |
| walker |  | 6065 | 61 | Code::CodeKey { rung: Names, file: internal/models/models_data.go, decl: 0, sub: 0, line: 0 } |  |  | 0.536 |
| walker |  | 6085 | 20 | Code::CodeKey { rung: Decl, file: internal/models/models_data.go, decl: 3, sub: 0, line: 26 } |  |  | 0.536 |
| walker |  | 6131 | 46 | Code::CodeKey { rung: Decl, file: internal/models/models_data.go, decl: 2, sub: 0, line: 18 } |  |  | 0.536 |
| walker |  | 6183 | 52 | Code::CodeKey { rung: Decl, file: internal/models/models_data.go, decl: 4, sub: 0, line: 32 } |  |  | 0.536 |
| ns | 6235 |  | 328 | Every top-level symbol of internal/models/providers.go (locations) | 5.2 | 5.1 | 0.521 |
| walker |  | 6249 | 66 | Code::CodeKey { rung: Decl, file: internal/models/models_data.go, decl: 1, sub: 0, line: 7 } |  |  | 0.521 |
| walker |  | 6261 | 12 | Code::CodeKey { rung: Doc, file: internal/models/models_data.go, decl: 2, sub: 0, line: 18 } |  |  | 0.521 |
| walker |  | 6273 | 12 | Code::CodeKey { rung: Doc, file: internal/models/models_data.go, decl: 4, sub: 0, line: 32 } |  |  | 0.521 |
| walker |  | 6287 | 14 | Code::CodeKey { rung: Doc, file: internal/models/models_data.go, decl: 1, sub: 0, line: 7 } |  |  | 0.521 |
| walker |  | 6301 | 14 | Code::CodeKey { rung: Doc, file: internal/models/models_data.go, decl: 3, sub: 0, line: 26 } |  |  | 0.521 |
| walker |  | 6316 | 15 | Code::CodeKey { rung: Doc, file: internal/models/models_data.go, decl: 5, sub: 0, line: 41 } |  |  | 0.521 |
| ns | 6385 |  | 150 | ModelsRegistry: model validation and suggestion API | 5.3 |  | 0.515 |
| walker |  | 6390 | 74 | Code::CodeKey { rung: Names, file: internal/hooks/schemas.go, decl: 0, sub: 0, line: 0 } |  |  | 0.515 |
| walker |  | 6414 | 24 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 4, sub: 0, line: 42 } |  |  | 0.515 |
| walker |  | 6458 | 44 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 2, sub: 0, line: 23 } |  |  | 0.515 |
| ns | 6477 |  | 92 | The generated model catalogue: generator types and the DO-NOT-EDIT header | 5.4 |  | 0.511 |
| walker |  | 6519 | 61 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 3, sub: 0, line: 32 } |  |  | 0.511 |
| walker |  | 6608 | 89 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 6, sub: 0, line: 62 } |  |  | 0.511 |
| ns | 6653 |  | 176 | Builtin server registry: the complete set of in-process servers | 6.1 |  | 0.502 |
| walker |  | 6715 | 107 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 5, sub: 0, line: 50 } |  |  | 0.503 |
| ns | 6825 |  | 172 | Every top-level symbol of internal/builtin/registry.go (locations) | 6.2 | 6.1 | 0.497 |
| walker |  | 6874 | 159 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 1, sub: 0, line: 10 } |  |  | 0.498 |
| walker |  | 6925 | 51 | Code::CodeKey { rung: Doc, file: internal/hooks/schemas.go, decl: 2, sub: 0, line: 23 } |  |  | 0.498 |
| walker |  | 6978 | 53 | Code::CodeKey { rung: Doc, file: internal/hooks/schemas.go, decl: 1, sub: 0, line: 10 } |  |  | 0.498 |
| walker |  | 7031 | 53 | Code::CodeKey { rung: Doc, file: internal/hooks/schemas.go, decl: 4, sub: 0, line: 42 } |  |  | 0.498 |
| ns | 7042 |  | 217 | Bash builtin: output/timeout limits and the complete banned-command list | 6.3 |  | 0.486 |
| walker |  | 7086 | 55 | Code::CodeKey { rung: Doc, file: internal/hooks/schemas.go, decl: 5, sub: 0, line: 50 } |  |  | 0.486 |
| walker |  | 7147 | 61 | Code::CodeKey { rung: Doc, file: internal/hooks/schemas.go, decl: 3, sub: 0, line: 32 } |  |  | 0.486 |
| walker |  | 7218 | 71 | Code::CodeKey { rung: Doc, file: internal/hooks/schemas.go, decl: 6, sub: 0, line: 62 } |  |  | 0.486 |
| ns | 7240 |  | 198 | The http builtin: its four tools and every symbol in http.go (locations) | 6.4 |  | 0.479 |
| walker |  | 7294 | 76 | Code::CodeKey { rung: Names, file: internal/models/generate_models.go, decl: 0, sub: 0, line: 0 } |  |  | 0.482 |
| walker |  | 7351 | 57 | Code::CodeKey { rung: Decl, file: internal/models/generate_models.go, decl: 3, sub: 0, line: 50 } |  |  | 0.482 |
| ns | 7352 |  | 112 | HookEvent: the complete set of hook events | 7.1 |  | 0.478 |
| walker |  | 7482 | 131 | Code::CodeKey { rung: Decl, file: internal/models/generate_models.go, decl: 2, sub: 0, line: 37 } |  |  | 0.478 |
| ns | 7567 |  | 215 | Hook configuration schema: HookConfig, HookMatcher, HookEntry | 7.2 |  | 0.471 |
| walker |  | 7624 | 142 | Code::CodeKey { rung: Decl, file: internal/models/generate_models.go, decl: 4, sub: 0, line: 60 } |  |  | 0.471 |
| walker |  | 7655 | 31 | Code::CodeKey { rung: Doc, file: internal/models/generate_models.go, decl: 2, sub: 0, line: 37 } |  |  | 0.471 |
| walker |  | 7687 | 32 | Code::CodeKey { rung: Doc, file: internal/models/generate_models.go, decl: 3, sub: 0, line: 50 } |  |  | 0.471 |
| ns | 7839 |  | 272 | Hook wire protocol: CommonInput and HookOutput | 7.3 |  | 0.486 |
| walker |  | 7886 | 199 | Code::CodeKey { rung: Decl, file: internal/models/generate_models.go, decl: 1, sub: 0, line: 18 } |  |  | 0.486 |
| walker |  | 7927 | 41 | Code::CodeKey { rung: Doc, file: internal/models/generate_models.go, decl: 4, sub: 0, line: 60 } |  |  | 0.486 |
| walker |  | 7975 | 48 | Code::CodeKey { rung: Doc, file: internal/models/generate_models.go, decl: 1, sub: 0, line: 18 } |  |  | 0.486 |
| walker |  | 8019 | 44 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 13, sub: 0, line: 499 } |  |  | 0.486 |
| ns | 8030 |  | 191 | Per-event hook input structs | 7.4 |  | 0.496 |
| walker |  | 8066 | 47 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 3, sub: 0, line: 84 } |  |  | 0.496 |
| walker |  | 8094 | 28 | Code::CodeKey { rung: Names, file: internal/tokens/init.go, decl: 0, sub: 0, line: 0 } |  |  | 0.496 |
| walker |  | 8109 | 15 | Code::CodeKey { rung: Body, file: internal/tokens/init.go, decl: 1, sub: 0, line: 21 } |  |  | 0.496 |
| walker |  | 8124 | 15 | Code::CodeKey { rung: Body, file: internal/tokens/init.go, decl: 2, sub: 0, line: 50 } |  |  | 0.496 |
| walker |  | 8212 | 88 | Code::CodeKey { rung: Names, file: internal/hooks/config.go, decl: 0, sub: 0, line: 0 } |  |  | 0.497 |
| walker |  | 8239 | 27 | Code::CodeKey { rung: Decl, file: internal/hooks/config.go, decl: 1, sub: 0, line: 14 } |  |  | 0.499 |
| walker |  | 8294 | 55 | Code::CodeKey { rung: Decl, file: internal/hooks/config.go, decl: 3, sub: 0, line: 30 } |  |  | 0.506 |
| ns | 8336 |  | 306 | Hook executor and validator symbol rosters | 7.5 |  | 0.496 |
| walker |  | 8359 | 65 | Code::CodeKey { rung: Decl, file: internal/hooks/config.go, decl: 2, sub: 0, line: 21 } |  |  | 0.507 |
| walker |  | 8388 | 29 | Code::CodeKey { rung: Doc, file: internal/hooks/config.go, decl: 1, sub: 0, line: 14 } |  |  | 0.507 |
| walker |  | 8402 | 14 | Code::CodeKey { rung: Doc, file: internal/hooks/config.go, decl: 6, sub: 0, line: 113 } |  |  | 0.507 |
| walker |  | 8458 | 56 | Code::CodeKey { rung: Doc, file: internal/hooks/config.go, decl: 3, sub: 0, line: 30 } |  |  | 0.507 |
| ns | 8493 |  | 157 | README: hooks.yml file locations and the --no-hooks escape hatch | 7.6 | 1.6 | 0.503 |
| walker |  | 8518 | 60 | Code::CodeKey { rung: Doc, file: internal/hooks/config.go, decl: 2, sub: 0, line: 21 } |  |  | 0.503 |
| walker |  | 8537 | 19 | Code::CodeKey { rung: Doc, file: internal/hooks/config.go, decl: 5, sub: 0, line: 97 } |  |  | 0.503 |
| walker |  | 8626 | 89 | Code::CodeKey { rung: Doc, file: internal/hooks/config.go, decl: 4, sub: 0, line: 41 } |  |  | 0.503 |
| walker |  | 8677 | 51 | Code::CodeKey { rung: Doc, file: cmd/auth.go, decl: 3, sub: 0, line: 58 } |  |  | 0.503 |
| ns | 8682 |  | 189 | Script mode: a worked frontmatter example and the variable rules | 8.1 |  | 0.497 |
| walker |  | 8707 | 30 | Code::CodeKey { rung: Names, file: internal/auth/browser.go, decl: 0, sub: 0, line: 0 } |  |  | 0.497 |
| walker |  | 8738 | 31 | Code::CodeKey { rung: Body, file: internal/auth/browser.go, decl: 2, sub: 0, line: 34 } |  |  | 0.497 |
| walker |  | 8798 | 60 | Code::CodeKey { rung: Doc, file: internal/auth/browser.go, decl: 2, sub: 0, line: 34 } |  |  | 0.497 |
| ns | 8813 |  | 131 | Every top-level symbol of cmd/script.go (locations) | 8.2 |  | 0.506 |
| walker |  | 8886 | 88 | Code::CodeKey { rung: Doc, file: internal/auth/browser.go, decl: 1, sub: 0, line: 14 } |  |  | 0.506 |
| walker |  | 8946 | 60 | Code::CodeKey { rung: Names, file: internal/ui/commands.go, decl: 0, sub: 0, line: 0 } |  |  | 0.506 |
| walker |  | 8997 | 51 | Code::CodeKey { rung: Decl, file: internal/ui/commands.go, decl: 1, sub: 0, line: 6 } |  |  | 0.506 |
| walker |  | 9045 | 48 | Code::CodeKey { rung: Doc, file: internal/ui/commands.go, decl: 1, sub: 0, line: 6 } |  |  | 0.506 |
| walker |  | 9100 | 55 | Code::CodeKey { rung: Doc, file: internal/ui/commands.go, decl: 3, sub: 0, line: 65 } |  |  | 0.506 |
| ns | 9111 |  | 298 | Session file format: Session, Metadata, Message and ToolCall fields | 8.3 |  | 0.497 |
| walker |  | 9158 | 58 | Code::CodeKey { rung: Doc, file: internal/ui/commands.go, decl: 4, sub: 0, line: 83 } |  |  | 0.497 |
| ns | 9405 |  | 294 | The public SDK surface: Options and every exported symbol | 8.4 |  | 0.510 |
| ns | 9607 |  | 202 | The complete slash-command table: names and descriptions | 8.5 |  | 0.504 |
| walker |  | 9620 | 462 | Code::CodeKey { rung: Decl, file: internal/ui/commands.go, decl: 2, sub: 0, line: 16 } |  |  | 0.520 |
| walker |  | 9672 | 52 | Code::CodeKey { rung: Doc, file: internal/ui/commands.go, decl: 2, sub: 0, line: 16 } |  |  | 0.520 |
| ns | 9828 |  | 221 | ui.SetupCLI: the AgentInterface contract and CLISetupOptions | 8.6 |  | 0.513 |
| walker |  | 9884 | 212 | Code::CodeKey { rung: Names, file: internal/config/config.go, decl: 0, sub: 0, line: 0 } |  |  | 0.522 |
| walker |  | 9926 | 42 | Code::CodeKey { rung: Decl, file: internal/config/config.go, decl: 3, sub: 0, line: 109 } |  |  | 0.522 |
| walker |  | 9935 | 9 | Code::CodeKey { rung: Body, file: internal/config/config.go, decl: 14, sub: 0, line: 459 } |  |  | 0.522 |
| ns | 9984 |  | 156 | Release packaging: the goreleaser build matrix | 9.1 |  | 0.516 |

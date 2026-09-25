Score(3000)=0.712 I=0.873 C=0.580 ns_rows≤3K=18/51 grid(1000/1442/2080/3000/4327/6240/9000)=0.891/0.814/0.742/0.712/0.597/0.523/0.512

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
| ns | 781 |  | 133 | Complete listings for the config / agent / tools / models packages | 1.9 |  | 0.726 |
| walker |  | 937 | 229 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: true } |  |  | 0.829 |
| walker |  | 947 | 10 | Markdown::Section { file: contribute/contribute.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.829 |
| walker |  | 973 | 26 | Fs::DirListing { dir: internal/tools } |  |  | 0.887 |
| walker |  | 993 | 20 | Markdown::Section { file: contribute/contribute.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.887 |
| ns | 999 |  | 218 | Complete listings for the builtin / hooks / session / auth / tokens / ui packages | 1.10 |  | 0.891 |
| walker |  | 1020 | 27 | Code::CodeKey { rung: Names, file: main.go, decl: 0, sub: 0, line: 0 } |  |  | 0.891 |
| ns | 1089 |  | 90 | Complete listings for examples/, contribute/ and .github/ | 1.11 |  | 0.848 |
| walker |  | 1140 | 120 | Markdown::HeadingsOutline { file: sdk/README.md } |  |  | 0.848 |
| walker |  | 1153 | 13 | Markdown::Section { file: sdk/README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.848 |
| walker |  | 1186 | 33 | Markdown::Section { file: sdk/README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.848 |
| ns | 1243 |  | 154 | main.go entry point | 2.1 |  | 0.812 |
| walker |  | 1282 | 96 | Markdown::Section { file: sdk/README.md, section_index: 6, keeps_default_concavity: true } |  |  | 0.812 |
| walker |  | 1330 | 48 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.812 |
| walker |  | 1370 | 40 | Markdown::Section { file: README.md, section_index: 33, keeps_default_concavity: false } |  |  | 0.814 |
| ns | 1504 |  | 261 | Complete cobra command tree: script, auth (login/logout/status), hooks (list/validate/init) | 2.2 |  | 0.771 |
| ns | 1754 |  | 250 | Persistent flag registration, part 1: config, system-prompt, model, debug, prompt, quiet | 2.3 |  | 0.742 |
| walker |  | 1884 | 514 | GoMod::File { file: go.mod } |  |  | 0.742 |
| ns | 2098 |  | 344 | Persistent flag registration, part 2: no-exit, max-steps, stream, compact, no-hooks, approve-tool-run, session flags | 2.4 |  | 0.705 |
| walker |  | 2142 | 258 | Markdown::Section { file: sdk/README.md, section_index: 2, keeps_default_concavity: true } |  |  | 0.705 |
| ns | 2224 |  | 126 | Provider and TLS flag registration | 2.5 |  | 0.699 |
| walker |  | 2389 | 247 | Markdown::Section { file: sdk/README.md, section_index: 3, keeps_default_concavity: true } |  |  | 0.699 |
| walker |  | 2447 | 58 | Markdown::Section { file: README.md, section_index: 31, keeps_default_concavity: false } |  |  | 0.701 |
| ns | 2502 |  | 278 | Generation-parameter and Ollama flag registration, with the hidden flag | 2.6 |  | 0.683 |
| walker |  | 2574 | 127 | Code::CodeKey { rung: Body, file: main.go, decl: 2, sub: 0, line: 14 } |  |  | 0.721 |
| walker |  | 2640 | 66 | Markdown::Section { file: README.md, section_index: 34, keeps_default_concavity: false } |  |  | 0.723 |
| ns | 2749 |  | 247 | MCPServerConfig: complete field set including the legacy block | 3.1 |  | 0.698 |
| walker |  | 2838 | 198 | Code::CodeKey { rung: ModuleDoc, file: internal/tokens/anthropic.go, decl: 0, sub: 0, line: 0 } |  |  | 0.698 |
| walker |  | 2844 | 6 | Fs::DirListing { dir: sdk/examples } |  |  | 0.711 |
| walker |  | 2915 | 71 | Code::CodeKey { rung: Names, file: cmd/hooks.go, decl: 0, sub: 0, line: 0 } |  |  | 0.711 |
| walker |  | 2962 | 47 | Code::CodeKey { rung: Decl, file: cmd/hooks.go, decl: 1, sub: 0, line: 16 } |  |  | 0.712 |
| ns | 3069 |  | 320 | Config struct: application-level keys | 3.2 |  | 0.690 |
| walker |  | 3147 | 185 | Code::CodeKey { rung: Decl, file: cmd/hooks.go, decl: 3, sub: 0, line: 57 } |  |  | 0.691 |
| ns | 3250 |  | 181 | Config struct: generation-parameter and TLS keys | 3.3 |  | 0.676 |
| walker |  | 3467 | 320 | Code::CodeKey { rung: Decl, file: cmd/hooks.go, decl: 2, sub: 0, line: 25 } |  |  | 0.678 |
| ns | 3514 |  | 264 | GetTransportType: type-to-transport mapping and legacy inference | 3.4 |  | 0.644 |
| walker |  | 3518 | 51 | Code::CodeKey { rung: Doc, file: cmd/hooks.go, decl: 3, sub: 0, line: 57 } |  |  | 0.644 |
| walker |  | 3572 | 54 | Code::CodeKey { rung: Doc, file: cmd/hooks.go, decl: 2, sub: 0, line: 25 } |  |  | 0.644 |
| walker |  | 3631 | 59 | Code::CodeKey { rung: Doc, file: cmd/hooks.go, decl: 1, sub: 0, line: 16 } |  |  | 0.644 |
| walker |  | 3671 | 40 | Plaintext::DeclSurface { file: contribute/build.sh } |  |  | 0.644 |
| walker |  | 3774 | 103 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.646 |
| walker |  | 3934 | 160 | Code::CodeKey { rung: Names, file: cmd/auth.go, decl: 0, sub: 0, line: 0 } |  |  | 0.646 |
| ns | 3938 |  | 424 | Config.Validate: required fields per transport and filter exclusivity | 3.5 |  | 0.614 |
| walker |  | 4053 | 119 | Code::CodeKey { rung: Decl, file: cmd/auth.go, decl: 4, sub: 0, line: 78 } |  |  | 0.617 |
| ns | 4141 |  | 203 | Substitution engine: the two regexes plus every symbol in substitution.go | 3.6 |  | 0.605 |
| walker |  | 4214 | 161 | Code::CodeKey { rung: Decl, file: cmd/auth.go, decl: 1, sub: 0, line: 17 } |  |  | 0.609 |
| ns | 4295 |  | 154 | Remaining top-level symbols of internal/config/config.go and all of merger.go (locations) | 3.7 |  | 0.597 |
| walker |  | 4381 | 167 | Code::CodeKey { rung: Decl, file: cmd/auth.go, decl: 3, sub: 0, line: 58 } |  |  | 0.603 |
| ns | 4541 |  | 246 | Agent struct and its seven callback handler types | 4.1 |  | 0.588 |
| walker |  | 4556 | 175 | Code::CodeKey { rung: Decl, file: cmd/auth.go, decl: 2, sub: 0, line: 37 } |  |  | 0.594 |
| walker |  | 4607 | 51 | Code::CodeKey { rung: Doc, file: cmd/auth.go, decl: 3, sub: 0, line: 58 } |  |  | 0.594 |
| walker |  | 4659 | 52 | Code::CodeKey { rung: Doc, file: cmd/auth.go, decl: 2, sub: 0, line: 37 } |  |  | 0.594 |
| walker |  | 4713 | 54 | Code::CodeKey { rung: Doc, file: cmd/auth.go, decl: 4, sub: 0, line: 78 } |  |  | 0.594 |
| walker |  | 4777 | 64 | Code::CodeKey { rung: Doc, file: cmd/auth.go, decl: 1, sub: 0, line: 17 } |  |  | 0.594 |
| ns | 4815 |  | 274 | Every top-level symbol of internal/agent/agent.go (locations) | 4.2 |  | 0.579 |
| walker |  | 4842 | 65 | Code::CodeKey { rung: Names, file: sdk/types.go, decl: 0, sub: 0, line: 0 } |  |  | 0.579 |
| walker |  | 4853 | 11 | Code::CodeKey { rung: Body, file: sdk/types.go, decl: 3, sub: 0, line: 18 } |  |  | 0.579 |
| walker |  | 4865 | 12 | Code::CodeKey { rung: Body, file: sdk/types.go, decl: 4, sub: 0, line: 24 } |  |  | 0.579 |
| walker |  | 4897 | 32 | Code::CodeKey { rung: Doc, file: sdk/types.go, decl: 3, sub: 0, line: 18 } |  |  | 0.579 |
| walker |  | 4930 | 33 | Code::CodeKey { rung: Doc, file: sdk/types.go, decl: 1, sub: 0, line: 10 } |  |  | 0.579 |
| walker |  | 4964 | 34 | Code::CodeKey { rung: Doc, file: sdk/types.go, decl: 2, sub: 0, line: 14 } |  |  | 0.579 |
| walker |  | 4998 | 34 | Code::CodeKey { rung: Doc, file: sdk/types.go, decl: 4, sub: 0, line: 24 } |  |  | 0.579 |
| walker |  | 5002 | 4 | Fs::DirListing { dir: sdk/examples/basic } |  |  | 0.585 |
| walker |  | 5006 | 4 | Fs::DirListing { dir: sdk/examples/scripting } |  |  | 0.591 |
| ns | 5007 |  | 192 | The tool-calling loop: step bound, tool-call branch and the approval gate | 4.3 | 4.2 | 0.581 |
| ns | 5186 |  | 179 | agent factory: AgentCreationOptions fields and both functions | 4.4 |  | 0.571 |
| walker |  | 5219 | 213 | Code::CodeKey { rung: Names, file: sdk/mcphost.go, decl: 0, sub: 0, line: 0 } |  |  | 0.571 |
| walker |  | 5254 | 35 | Code::CodeKey { rung: Decl, file: sdk/mcphost.go, decl: 1, sub: 0, line: 19 } |  |  | 0.571 |
| walker |  | 5263 | 9 | Code::CodeKey { rung: Body, file: sdk/mcphost.go, decl: 6, sub: 0, line: 201 } |  |  | 0.571 |
| walker |  | 5272 | 9 | Code::CodeKey { rung: Body, file: sdk/mcphost.go, decl: 10, sub: 0, line: 230 } |  |  | 0.571 |
| walker |  | 5281 | 9 | Code::CodeKey { rung: Body, file: sdk/mcphost.go, decl: 11, sub: 0, line: 237 } |  |  | 0.571 |
| walker |  | 5349 | 68 | Code::CodeKey { rung: Decl, file: sdk/mcphost.go, decl: 5, sub: 0, line: 163 } |  |  | 0.571 |
| walker |  | 5456 | 107 | Code::CodeKey { rung: Decl, file: sdk/mcphost.go, decl: 2, sub: 0, line: 28 } |  |  | 0.572 |
| walker |  | 5485 | 29 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 6, sub: 0, line: 201 } |  |  | 0.572 |
| walker |  | 5514 | 29 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 9, sub: 0, line: 224 } |  |  | 0.572 |
| ns | 5549 |  | 363 | Every top-level symbol of internal/tools/mcp.go (locations) | 4.5 |  | 0.556 |
| walker |  | 5551 | 37 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 8, sub: 0, line: 218 } |  |  | 0.556 |
| walker |  | 5590 | 39 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 7, sub: 0, line: 207 } |  |  | 0.556 |
| walker |  | 5637 | 47 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 2, sub: 0, line: 28 } |  |  | 0.556 |
| walker |  | 5685 | 48 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 11, sub: 0, line: 237 } |  |  | 0.556 |
| walker |  | 5734 | 49 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 10, sub: 0, line: 230 } |  |  | 0.556 |
| ns | 5765 |  | 216 | AgenticLoopConfig head and every mode-driving function in cmd/root.go (locations) | 4.6 |  | 0.545 |
| walker |  | 5786 | 52 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 1, sub: 0, line: 19 } |  |  | 0.545 |
| walker |  | 5842 | 56 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 5, sub: 0, line: 163 } |  |  | 0.545 |
| walker |  | 5899 | 57 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 3, sub: 0, line: 40 } |  |  | 0.545 |
| ns | 5907 |  | 142 | CreateProvider: the complete list of supported providers | 5.1 |  | 0.537 |
| walker |  | 5957 | 58 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 4, sub: 0, line: 130 } |  |  | 0.537 |
| ns | 6235 |  | 328 | Every top-level symbol of internal/models/providers.go (locations) | 5.2 | 5.1 | 0.522 |
| walker |  | 6237 | 280 | Code::CodeKey { rung: Names, file: cmd/script.go, decl: 0, sub: 0, line: 0 } |  |  | 0.523 |
| walker |  | 6302 | 65 | Code::CodeKey { rung: Decl, file: cmd/script.go, decl: 9, sub: 0, line: 423 } |  |  | 0.523 |
| walker |  | 6313 | 11 | Code::CodeKey { rung: Body, file: cmd/script.go, decl: 2, sub: 0, line: 78 } |  |  | 0.523 |
| walker |  | 6329 | 16 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 4, sub: 0, line: 148 } |  |  | 0.523 |
| walker |  | 6345 | 16 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 7, sub: 0, line: 279 } |  |  | 0.523 |
| walker |  | 6363 | 18 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 14, sub: 0, line: 511 } |  |  | 0.523 |
| walker |  | 6382 | 19 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 8, sub: 0, line: 288 } |  |  | 0.523 |
| ns | 6385 |  | 150 | ModelsRegistry: model validation and suggestion API | 5.3 |  | 0.517 |
| walker |  | 6402 | 20 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 6, sub: 0, line: 255 } |  |  | 0.517 |
| walker |  | 6433 | 31 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 12, sub: 0, line: 480 } |  |  | 0.517 |
| walker |  | 6466 | 33 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 10, sub: 0, line: 431 } |  |  | 0.517 |
| ns | 6477 |  | 92 | The generated model catalogue: generator types and the DO-NOT-EDIT header | 5.4 |  | 0.513 |
| walker |  | 6502 | 36 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 11, sub: 0, line: 442 } |  |  | 0.513 |
| walker |  | 6546 | 44 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 13, sub: 0, line: 499 } |  |  | 0.513 |
| walker |  | 6593 | 47 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 3, sub: 0, line: 84 } |  |  | 0.513 |
| walker |  | 6646 | 53 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 9, sub: 0, line: 423 } |  |  | 0.513 |
| ns | 6653 |  | 176 | Builtin server registry: the complete set of in-process servers | 6.1 |  | 0.504 |
| walker |  | 6738 | 92 | Markdown::Section { file: README.md, section_index: 32, keeps_default_concavity: false } |  |  | 0.506 |
| ns | 6825 |  | 172 | Every top-level symbol of internal/builtin/registry.go (locations) | 6.2 | 6.1 | 0.500 |
| ns | 7042 |  | 217 | Bash builtin: output/timeout limits and the complete banned-command list | 6.3 |  | 0.489 |
| ns | 7240 |  | 198 | The http builtin: its four tools and every symbol in http.go (locations) | 6.4 |  | 0.481 |
| ns | 7352 |  | 112 | HookEvent: the complete set of hook events | 7.1 |  | 0.478 |
| walker |  | 7368 | 630 | Code::CodeKey { rung: Decl, file: cmd/script.go, decl: 1, sub: 0, line: 28 } |  |  | 0.485 |
| walker |  | 7429 | 61 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 1, sub: 0, line: 28 } |  |  | 0.485 |
| walker |  | 7490 | 61 | Code::CodeKey { rung: Names, file: internal/models/models_data.go, decl: 0, sub: 0, line: 0 } |  |  | 0.485 |
| walker |  | 7510 | 20 | Code::CodeKey { rung: Decl, file: internal/models/models_data.go, decl: 3, sub: 0, line: 26 } |  |  | 0.485 |
| walker |  | 7556 | 46 | Code::CodeKey { rung: Decl, file: internal/models/models_data.go, decl: 2, sub: 0, line: 18 } |  |  | 0.485 |
| ns | 7567 |  | 215 | Hook configuration schema: HookConfig, HookMatcher, HookEntry | 7.2 |  | 0.478 |
| walker |  | 7608 | 52 | Code::CodeKey { rung: Decl, file: internal/models/models_data.go, decl: 4, sub: 0, line: 32 } |  |  | 0.478 |
| walker |  | 7674 | 66 | Code::CodeKey { rung: Decl, file: internal/models/models_data.go, decl: 1, sub: 0, line: 7 } |  |  | 0.478 |
| walker |  | 7686 | 12 | Code::CodeKey { rung: Doc, file: internal/models/models_data.go, decl: 2, sub: 0, line: 18 } |  |  | 0.478 |
| walker |  | 7698 | 12 | Code::CodeKey { rung: Doc, file: internal/models/models_data.go, decl: 4, sub: 0, line: 32 } |  |  | 0.478 |
| walker |  | 7712 | 14 | Code::CodeKey { rung: Doc, file: internal/models/models_data.go, decl: 1, sub: 0, line: 7 } |  |  | 0.478 |
| walker |  | 7726 | 14 | Code::CodeKey { rung: Doc, file: internal/models/models_data.go, decl: 3, sub: 0, line: 26 } |  |  | 0.478 |
| walker |  | 7741 | 15 | Code::CodeKey { rung: Doc, file: internal/models/models_data.go, decl: 5, sub: 0, line: 41 } |  |  | 0.478 |
| walker |  | 7815 | 74 | Code::CodeKey { rung: Names, file: internal/hooks/schemas.go, decl: 0, sub: 0, line: 0 } |  |  | 0.478 |
| walker |  | 7839 | 24 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 4, sub: 0, line: 42 } |  |  | 0.471 |
| ns | 7839 |  | 272 | Hook wire protocol: CommonInput and HookOutput | 7.3 |  | 0.471 |
| walker |  | 7883 | 44 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 2, sub: 0, line: 23 } |  |  | 0.471 |
| walker |  | 7944 | 61 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 3, sub: 0, line: 32 } |  |  | 0.471 |
| ns | 8030 |  | 191 | Per-event hook input structs | 7.4 |  | 0.470 |
| walker |  | 8033 | 89 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 6, sub: 0, line: 62 } |  |  | 0.474 |
| walker |  | 8140 | 107 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 5, sub: 0, line: 50 } |  |  | 0.486 |
| walker |  | 8299 | 159 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 1, sub: 0, line: 10 } |  |  | 0.503 |
| ns | 8336 |  | 306 | Hook executor and validator symbol rosters | 7.5 |  | 0.494 |
| walker |  | 8350 | 51 | Code::CodeKey { rung: Doc, file: internal/hooks/schemas.go, decl: 2, sub: 0, line: 23 } |  |  | 0.494 |
| walker |  | 8403 | 53 | Code::CodeKey { rung: Doc, file: internal/hooks/schemas.go, decl: 1, sub: 0, line: 10 } |  |  | 0.494 |
| walker |  | 8456 | 53 | Code::CodeKey { rung: Doc, file: internal/hooks/schemas.go, decl: 4, sub: 0, line: 42 } |  |  | 0.494 |
| ns | 8493 |  | 157 | README: hooks.yml file locations and the --no-hooks escape hatch | 7.6 | 1.6 | 0.490 |
| walker |  | 8511 | 55 | Code::CodeKey { rung: Doc, file: internal/hooks/schemas.go, decl: 5, sub: 0, line: 50 } |  |  | 0.490 |
| walker |  | 8573 | 62 | Plaintext::DeclSurface { file: contribute/boost.sh } |  |  | 0.490 |
| walker |  | 8649 | 76 | Code::CodeKey { rung: Names, file: internal/models/generate_models.go, decl: 0, sub: 0, line: 0 } |  |  | 0.493 |
| ns | 8682 |  | 189 | Script mode: a worked frontmatter example and the variable rules | 8.1 |  | 0.503 |
| walker |  | 8706 | 57 | Code::CodeKey { rung: Decl, file: internal/models/generate_models.go, decl: 3, sub: 0, line: 50 } |  |  | 0.503 |
| ns | 8813 |  | 131 | Every top-level symbol of cmd/script.go (locations) | 8.2 |  | 0.512 |
| walker |  | 8837 | 131 | Code::CodeKey { rung: Decl, file: internal/models/generate_models.go, decl: 2, sub: 0, line: 37 } |  |  | 0.512 |
| walker |  | 8979 | 142 | Code::CodeKey { rung: Decl, file: internal/models/generate_models.go, decl: 4, sub: 0, line: 60 } |  |  | 0.512 |
| walker |  | 9010 | 31 | Code::CodeKey { rung: Doc, file: internal/models/generate_models.go, decl: 2, sub: 0, line: 37 } |  |  | 0.512 |
| walker |  | 9042 | 32 | Code::CodeKey { rung: Doc, file: internal/models/generate_models.go, decl: 3, sub: 0, line: 50 } |  |  | 0.512 |
| ns | 9111 |  | 298 | Session file format: Session, Metadata, Message and ToolCall fields | 8.3 |  | 0.503 |
| walker |  | 9241 | 199 | Code::CodeKey { rung: Decl, file: internal/models/generate_models.go, decl: 1, sub: 0, line: 18 } |  |  | 0.503 |
| walker |  | 9282 | 41 | Code::CodeKey { rung: Doc, file: internal/models/generate_models.go, decl: 4, sub: 0, line: 60 } |  |  | 0.503 |
| walker |  | 9330 | 48 | Code::CodeKey { rung: Doc, file: internal/models/generate_models.go, decl: 1, sub: 0, line: 18 } |  |  | 0.503 |
| walker |  | 9391 | 61 | Code::CodeKey { rung: Doc, file: internal/hooks/schemas.go, decl: 3, sub: 0, line: 32 } |  |  | 0.503 |
| ns | 9405 |  | 294 | The public SDK surface: Options and every exported symbol | 8.4 |  | 0.516 |
| walker |  | 9419 | 28 | Code::CodeKey { rung: Names, file: internal/tokens/init.go, decl: 0, sub: 0, line: 0 } |  |  | 0.516 |
| walker |  | 9434 | 15 | Code::CodeKey { rung: Body, file: internal/tokens/init.go, decl: 1, sub: 0, line: 21 } |  |  | 0.516 |
| walker |  | 9449 | 15 | Code::CodeKey { rung: Body, file: internal/tokens/init.go, decl: 2, sub: 0, line: 50 } |  |  | 0.516 |
| walker |  | 9537 | 88 | Code::CodeKey { rung: Names, file: internal/hooks/config.go, decl: 0, sub: 0, line: 0 } |  |  | 0.517 |
| walker |  | 9564 | 27 | Code::CodeKey { rung: Decl, file: internal/hooks/config.go, decl: 1, sub: 0, line: 14 } |  |  | 0.519 |
| ns | 9607 |  | 202 | The complete slash-command table: names and descriptions | 8.5 |  | 0.513 |
| walker |  | 9619 | 55 | Code::CodeKey { rung: Decl, file: internal/hooks/config.go, decl: 3, sub: 0, line: 30 } |  |  | 0.518 |
| walker |  | 9684 | 65 | Code::CodeKey { rung: Decl, file: internal/hooks/config.go, decl: 2, sub: 0, line: 21 } |  |  | 0.527 |
| walker |  | 9698 | 14 | Code::CodeKey { rung: Doc, file: internal/hooks/config.go, decl: 6, sub: 0, line: 113 } |  |  | 0.527 |
| walker |  | 9717 | 19 | Code::CodeKey { rung: Doc, file: internal/hooks/config.go, decl: 5, sub: 0, line: 97 } |  |  | 0.527 |
| walker |  | 9746 | 29 | Code::CodeKey { rung: Doc, file: internal/hooks/config.go, decl: 1, sub: 0, line: 14 } |  |  | 0.527 |
| walker |  | 9802 | 56 | Code::CodeKey { rung: Doc, file: internal/hooks/config.go, decl: 3, sub: 0, line: 30 } |  |  | 0.527 |
| ns | 9828 |  | 221 | ui.SetupCLI: the AgentInterface contract and CLISetupOptions | 8.6 |  | 0.519 |
| walker |  | 9862 | 60 | Code::CodeKey { rung: Doc, file: internal/hooks/config.go, decl: 2, sub: 0, line: 21 } |  |  | 0.519 |
| walker |  | 9892 | 30 | Code::CodeKey { rung: Names, file: internal/auth/browser.go, decl: 0, sub: 0, line: 0 } |  |  | 0.519 |
| walker |  | 9923 | 31 | Code::CodeKey { rung: Body, file: internal/auth/browser.go, decl: 2, sub: 0, line: 34 } |  |  | 0.519 |
| walker |  | 9983 | 60 | Code::CodeKey { rung: Doc, file: internal/auth/browser.go, decl: 2, sub: 0, line: 34 } |  |  | 0.519 |
| ns | 9984 |  | 156 | Release packaging: the goreleaser build matrix | 9.1 |  | 0.514 |

Score(3000)=0.736 I=0.880 C=0.616 ns_rows≤3K=18/51 grid(1000/1442/2080/3000/4327/6240/9000)=0.893/0.862/0.805/0.736/0.618/0.528/0.509

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
| ns | 534 |  | 107 | README feature list, first half | 1.7 | 1.6 | 0.741 |
| walker |  | 544 | 45 | Fs::DirListing { dir: internal/config } |  |  | 0.752 |
| walker |  | 630 | 86 | Fs::DirListing { dir: internal/ui } |  |  | 0.765 |
| walker |  | 635 | 5 | Fs::DirListing { dir: internal/ui/progress } |  |  | 0.765 |
| ns | 648 |  | 114 | README feature list, second half | 1.8 |  | 0.721 |
| ns | 781 |  | 133 | Complete listings for the config / agent / tools / models packages | 1.9 |  | 0.726 |
| walker |  | 864 | 229 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: true } |  |  | 0.829 |
| walker |  | 912 | 48 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.831 |
| walker |  | 938 | 26 | Fs::DirListing { dir: internal/tools } |  |  | 0.888 |
| walker |  | 965 | 27 | Code::CodeKey { rung: Names, file: main.go, decl: 0, sub: 0, line: 0 } |  |  | 0.888 |
| ns | 999 |  | 218 | Complete listings for the builtin / hooks / session / auth / tokens / ui packages | 1.10 |  | 0.892 |
| ns | 1089 |  | 90 | Complete listings for examples/, contribute/ and .github/ | 1.11 |  | 0.849 |
| walker |  | 1095 | 130 | Code::CodeKey { rung: Decl, file: main.go, decl: 2, sub: 0, line: 14 } |  |  | 0.855 |
| walker |  | 1153 | 58 | Markdown::Section { file: README.md, section_index: 31, keeps_default_concavity: false } |  |  | 0.857 |
| ns | 1243 |  | 154 | main.go entry point | 2.1 |  | 0.860 |
| walker |  | 1256 | 103 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.862 |
| walker |  | 1454 | 198 | Code::CodeKey { rung: ModuleDoc, file: internal/tokens/anthropic.go, decl: 0, sub: 0, line: 0 } |  |  | 0.862 |
| walker |  | 1460 | 6 | Fs::DirListing { dir: sdk/examples } |  |  | 0.878 |
| ns | 1504 |  | 261 | Complete cobra command tree: script, auth (login/logout/status), hooks (list/validate/init) | 2.2 |  | 0.832 |
| walker |  | 1531 | 71 | Code::CodeKey { rung: Names, file: cmd/hooks.go, decl: 0, sub: 0, line: 0 } |  |  | 0.832 |
| walker |  | 1578 | 47 | Code::CodeKey { rung: Decl, file: cmd/hooks.go, decl: 1, sub: 0, line: 16 } |  |  | 0.833 |
| ns | 1754 |  | 250 | Persistent flag registration, part 1: config, system-prompt, model, debug, prompt, quiet | 2.3 |  | 0.801 |
| walker |  | 1763 | 185 | Code::CodeKey { rung: Decl, file: cmd/hooks.go, decl: 3, sub: 0, line: 57 } |  |  | 0.803 |
| walker |  | 2083 | 320 | Code::CodeKey { rung: Decl, file: cmd/hooks.go, decl: 2, sub: 0, line: 25 } |  |  | 0.805 |
| ns | 2098 |  | 344 | Persistent flag registration, part 2: no-exit, max-steps, stream, compact, no-hooks, approve-tool-run, session flags | 2.4 |  | 0.765 |
| walker |  | 2134 | 51 | Code::CodeKey { rung: Doc, file: cmd/hooks.go, decl: 3, sub: 0, line: 57 } |  |  | 0.765 |
| walker |  | 2188 | 54 | Code::CodeKey { rung: Doc, file: cmd/hooks.go, decl: 2, sub: 0, line: 25 } |  |  | 0.765 |
| ns | 2224 |  | 126 | Provider and TLS flag registration | 2.5 |  | 0.758 |
| walker |  | 2247 | 59 | Code::CodeKey { rung: Doc, file: cmd/hooks.go, decl: 1, sub: 0, line: 16 } |  |  | 0.758 |
| walker |  | 2287 | 40 | Plaintext::DeclSurface { file: contribute/build.sh } |  |  | 0.758 |
| walker |  | 2298 | 11 | Plaintext::Whole { file: contribute/build.sh } |  |  | 0.758 |
| walker |  | 2458 | 160 | Code::CodeKey { rung: Names, file: cmd/auth.go, decl: 0, sub: 0, line: 0 } |  |  | 0.758 |
| ns | 2502 |  | 278 | Generation-parameter and Ollama flag registration, with the hidden flag | 2.6 |  | 0.739 |
| walker |  | 2577 | 119 | Code::CodeKey { rung: Decl, file: cmd/auth.go, decl: 4, sub: 0, line: 78 } |  |  | 0.743 |
| walker |  | 2738 | 161 | Code::CodeKey { rung: Decl, file: cmd/auth.go, decl: 1, sub: 0, line: 17 } |  |  | 0.748 |
| ns | 2749 |  | 247 | MCPServerConfig: complete field set including the legacy block | 3.1 |  | 0.722 |
| walker |  | 2905 | 167 | Code::CodeKey { rung: Decl, file: cmd/auth.go, decl: 3, sub: 0, line: 58 } |  |  | 0.729 |
| ns | 3069 |  | 320 | Config struct: application-level keys | 3.2 |  | 0.706 |
| walker |  | 3080 | 175 | Code::CodeKey { rung: Decl, file: cmd/auth.go, decl: 2, sub: 0, line: 37 } |  |  | 0.714 |
| walker |  | 3131 | 51 | Code::CodeKey { rung: Doc, file: cmd/auth.go, decl: 3, sub: 0, line: 58 } |  |  | 0.714 |
| walker |  | 3183 | 52 | Code::CodeKey { rung: Doc, file: cmd/auth.go, decl: 2, sub: 0, line: 37 } |  |  | 0.714 |
| walker |  | 3237 | 54 | Code::CodeKey { rung: Doc, file: cmd/auth.go, decl: 4, sub: 0, line: 78 } |  |  | 0.714 |
| ns | 3250 |  | 181 | Config struct: generation-parameter and TLS keys | 3.3 |  | 0.698 |
| walker |  | 3301 | 64 | Code::CodeKey { rung: Doc, file: cmd/auth.go, decl: 1, sub: 0, line: 17 } |  |  | 0.698 |
| walker |  | 3366 | 65 | Code::CodeKey { rung: Names, file: sdk/types.go, decl: 0, sub: 0, line: 0 } |  |  | 0.698 |
| walker |  | 3377 | 11 | Code::CodeKey { rung: Body, file: sdk/types.go, decl: 3, sub: 0, line: 18 } |  |  | 0.698 |
| walker |  | 3389 | 12 | Code::CodeKey { rung: Body, file: sdk/types.go, decl: 4, sub: 0, line: 24 } |  |  | 0.698 |
| walker |  | 3421 | 32 | Code::CodeKey { rung: Doc, file: sdk/types.go, decl: 3, sub: 0, line: 18 } |  |  | 0.698 |
| walker |  | 3454 | 33 | Code::CodeKey { rung: Doc, file: sdk/types.go, decl: 1, sub: 0, line: 10 } |  |  | 0.698 |
| walker |  | 3488 | 34 | Code::CodeKey { rung: Doc, file: sdk/types.go, decl: 2, sub: 0, line: 14 } |  |  | 0.698 |
| ns | 3514 |  | 264 | GetTransportType: type-to-transport mapping and legacy inference | 3.4 |  | 0.663 |
| walker |  | 3522 | 34 | Code::CodeKey { rung: Doc, file: sdk/types.go, decl: 4, sub: 0, line: 24 } |  |  | 0.663 |
| ns | 3938 |  | 424 | Config.Validate: required fields per transport and filter exclusivity | 3.5 |  | 0.630 |
| walker |  | 4036 | 514 | GoMod::File { file: go.mod } |  |  | 0.630 |
| walker |  | 4040 | 4 | Fs::DirListing { dir: sdk/examples/basic } |  |  | 0.636 |
| walker |  | 4044 | 4 | Fs::DirListing { dir: sdk/examples/scripting } |  |  | 0.643 |
| ns | 4141 |  | 203 | Substitution engine: the two regexes plus every symbol in substitution.go | 3.6 |  | 0.630 |
| walker |  | 4257 | 213 | Code::CodeKey { rung: Names, file: sdk/mcphost.go, decl: 0, sub: 0, line: 0 } |  |  | 0.631 |
| walker |  | 4292 | 35 | Code::CodeKey { rung: Decl, file: sdk/mcphost.go, decl: 1, sub: 0, line: 19 } |  |  | 0.631 |
| ns | 4295 |  | 154 | Remaining top-level symbols of internal/config/config.go and all of merger.go (locations) | 3.7 |  | 0.618 |
| walker |  | 4301 | 9 | Code::CodeKey { rung: Body, file: sdk/mcphost.go, decl: 6, sub: 0, line: 201 } |  |  | 0.618 |
| walker |  | 4310 | 9 | Code::CodeKey { rung: Body, file: sdk/mcphost.go, decl: 10, sub: 0, line: 230 } |  |  | 0.618 |
| walker |  | 4319 | 9 | Code::CodeKey { rung: Body, file: sdk/mcphost.go, decl: 11, sub: 0, line: 237 } |  |  | 0.618 |
| walker |  | 4387 | 68 | Code::CodeKey { rung: Decl, file: sdk/mcphost.go, decl: 5, sub: 0, line: 163 } |  |  | 0.618 |
| walker |  | 4494 | 107 | Code::CodeKey { rung: Decl, file: sdk/mcphost.go, decl: 2, sub: 0, line: 28 } |  |  | 0.619 |
| walker |  | 4523 | 29 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 6, sub: 0, line: 201 } |  |  | 0.619 |
| ns | 4541 |  | 246 | Agent struct and its seven callback handler types | 4.1 |  | 0.604 |
| walker |  | 4552 | 29 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 9, sub: 0, line: 224 } |  |  | 0.604 |
| walker |  | 4589 | 37 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 8, sub: 0, line: 218 } |  |  | 0.604 |
| walker |  | 4628 | 39 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 7, sub: 0, line: 207 } |  |  | 0.604 |
| walker |  | 4675 | 47 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 2, sub: 0, line: 28 } |  |  | 0.604 |
| walker |  | 4723 | 48 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 11, sub: 0, line: 237 } |  |  | 0.604 |
| walker |  | 4772 | 49 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 10, sub: 0, line: 230 } |  |  | 0.604 |
| ns | 4815 |  | 274 | Every top-level symbol of internal/agent/agent.go (locations) | 4.2 |  | 0.589 |
| walker |  | 4824 | 52 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 1, sub: 0, line: 19 } |  |  | 0.589 |
| walker |  | 4880 | 56 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 5, sub: 0, line: 163 } |  |  | 0.589 |
| walker |  | 4937 | 57 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 3, sub: 0, line: 40 } |  |  | 0.589 |
| walker |  | 4995 | 58 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 4, sub: 0, line: 130 } |  |  | 0.589 |
| ns | 5007 |  | 192 | The tool-calling loop: step bound, tool-call branch and the approval gate | 4.3 | 4.2 | 0.578 |
| ns | 5186 |  | 179 | agent factory: AgentCreationOptions fields and both functions | 4.4 |  | 0.568 |
| walker |  | 5275 | 280 | Code::CodeKey { rung: Names, file: cmd/script.go, decl: 0, sub: 0, line: 0 } |  |  | 0.569 |
| walker |  | 5340 | 65 | Code::CodeKey { rung: Decl, file: cmd/script.go, decl: 9, sub: 0, line: 423 } |  |  | 0.569 |
| walker |  | 5351 | 11 | Code::CodeKey { rung: Body, file: cmd/script.go, decl: 2, sub: 0, line: 78 } |  |  | 0.569 |
| walker |  | 5367 | 16 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 4, sub: 0, line: 148 } |  |  | 0.569 |
| walker |  | 5383 | 16 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 7, sub: 0, line: 279 } |  |  | 0.569 |
| walker |  | 5401 | 18 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 14, sub: 0, line: 511 } |  |  | 0.569 |
| walker |  | 5420 | 19 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 8, sub: 0, line: 288 } |  |  | 0.569 |
| walker |  | 5440 | 20 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 6, sub: 0, line: 255 } |  |  | 0.569 |
| walker |  | 5471 | 31 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 12, sub: 0, line: 480 } |  |  | 0.569 |
| walker |  | 5504 | 33 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 10, sub: 0, line: 431 } |  |  | 0.569 |
| walker |  | 5540 | 36 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 11, sub: 0, line: 442 } |  |  | 0.569 |
| ns | 5549 |  | 363 | Every top-level symbol of internal/tools/mcp.go (locations) | 4.5 |  | 0.554 |
| walker |  | 5584 | 44 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 13, sub: 0, line: 499 } |  |  | 0.554 |
| walker |  | 5631 | 47 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 3, sub: 0, line: 84 } |  |  | 0.554 |
| walker |  | 5684 | 53 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 9, sub: 0, line: 423 } |  |  | 0.554 |
| ns | 5765 |  | 216 | AgenticLoopConfig head and every mode-driving function in cmd/root.go (locations) | 4.6 |  | 0.542 |
| ns | 5907 |  | 142 | CreateProvider: the complete list of supported providers | 5.1 |  | 0.535 |
| ns | 6235 |  | 328 | Every top-level symbol of internal/models/providers.go (locations) | 5.2 | 5.1 | 0.520 |
| walker |  | 6314 | 630 | Code::CodeKey { rung: Decl, file: cmd/script.go, decl: 1, sub: 0, line: 28 } |  |  | 0.528 |
| walker |  | 6375 | 61 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 1, sub: 0, line: 28 } |  |  | 0.528 |
| ns | 6385 |  | 150 | ModelsRegistry: model validation and suggestion API | 5.3 |  | 0.522 |
| walker |  | 6436 | 61 | Code::CodeKey { rung: Names, file: internal/models/models_data.go, decl: 0, sub: 0, line: 0 } |  |  | 0.522 |
| walker |  | 6456 | 20 | Code::CodeKey { rung: Decl, file: internal/models/models_data.go, decl: 3, sub: 0, line: 26 } |  |  | 0.522 |
| ns | 6477 |  | 92 | The generated model catalogue: generator types and the DO-NOT-EDIT header | 5.4 |  | 0.518 |
| walker |  | 6502 | 46 | Code::CodeKey { rung: Decl, file: internal/models/models_data.go, decl: 2, sub: 0, line: 18 } |  |  | 0.518 |
| walker |  | 6554 | 52 | Code::CodeKey { rung: Decl, file: internal/models/models_data.go, decl: 4, sub: 0, line: 32 } |  |  | 0.518 |
| walker |  | 6620 | 66 | Code::CodeKey { rung: Decl, file: internal/models/models_data.go, decl: 1, sub: 0, line: 7 } |  |  | 0.518 |
| walker |  | 6632 | 12 | Code::CodeKey { rung: Doc, file: internal/models/models_data.go, decl: 2, sub: 0, line: 18 } |  |  | 0.518 |
| walker |  | 6644 | 12 | Code::CodeKey { rung: Doc, file: internal/models/models_data.go, decl: 4, sub: 0, line: 32 } |  |  | 0.518 |
| ns | 6653 |  | 176 | Builtin server registry: the complete set of in-process servers | 6.1 |  | 0.509 |
| walker |  | 6658 | 14 | Code::CodeKey { rung: Doc, file: internal/models/models_data.go, decl: 1, sub: 0, line: 7 } |  |  | 0.509 |
| walker |  | 6672 | 14 | Code::CodeKey { rung: Doc, file: internal/models/models_data.go, decl: 3, sub: 0, line: 26 } |  |  | 0.509 |
| walker |  | 6687 | 15 | Code::CodeKey { rung: Doc, file: internal/models/models_data.go, decl: 5, sub: 0, line: 41 } |  |  | 0.509 |
| walker |  | 6761 | 74 | Code::CodeKey { rung: Names, file: internal/hooks/schemas.go, decl: 0, sub: 0, line: 0 } |  |  | 0.509 |
| walker |  | 6785 | 24 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 4, sub: 0, line: 42 } |  |  | 0.509 |
| ns | 6825 |  | 172 | Every top-level symbol of internal/builtin/registry.go (locations) | 6.2 | 6.1 | 0.503 |
| walker |  | 6829 | 44 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 2, sub: 0, line: 23 } |  |  | 0.503 |
| walker |  | 6890 | 61 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 3, sub: 0, line: 32 } |  |  | 0.503 |
| walker |  | 6979 | 89 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 6, sub: 0, line: 62 } |  |  | 0.503 |
| ns | 7042 |  | 217 | Bash builtin: output/timeout limits and the complete banned-command list | 6.3 |  | 0.491 |
| walker |  | 7086 | 107 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 5, sub: 0, line: 50 } |  |  | 0.492 |
| ns | 7240 |  | 198 | The http builtin: its four tools and every symbol in http.go (locations) | 6.4 |  | 0.485 |
| walker |  | 7245 | 159 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 1, sub: 0, line: 10 } |  |  | 0.485 |
| walker |  | 7296 | 51 | Code::CodeKey { rung: Doc, file: internal/hooks/schemas.go, decl: 2, sub: 0, line: 23 } |  |  | 0.485 |
| walker |  | 7349 | 53 | Code::CodeKey { rung: Doc, file: internal/hooks/schemas.go, decl: 1, sub: 0, line: 10 } |  |  | 0.485 |
| ns | 7352 |  | 112 | HookEvent: the complete set of hook events | 7.1 |  | 0.482 |
| walker |  | 7402 | 53 | Code::CodeKey { rung: Doc, file: internal/hooks/schemas.go, decl: 4, sub: 0, line: 42 } |  |  | 0.482 |
| walker |  | 7457 | 55 | Code::CodeKey { rung: Doc, file: internal/hooks/schemas.go, decl: 5, sub: 0, line: 50 } |  |  | 0.482 |
| walker |  | 7519 | 62 | Plaintext::DeclSurface { file: contribute/boost.sh } |  |  | 0.482 |
| ns | 7567 |  | 215 | Hook configuration schema: HookConfig, HookMatcher, HookEntry | 7.2 |  | 0.475 |
| walker |  | 7793 | 274 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.476 |
| ns | 7839 |  | 272 | Hook wire protocol: CommonInput and HookOutput | 7.3 |  | 0.490 |
| walker |  | 7869 | 76 | Code::CodeKey { rung: Names, file: internal/models/generate_models.go, decl: 0, sub: 0, line: 0 } |  |  | 0.493 |
| walker |  | 7926 | 57 | Code::CodeKey { rung: Decl, file: internal/models/generate_models.go, decl: 3, sub: 0, line: 50 } |  |  | 0.493 |
| ns | 8030 |  | 191 | Per-event hook input structs | 7.4 |  | 0.502 |
| walker |  | 8057 | 131 | Code::CodeKey { rung: Decl, file: internal/models/generate_models.go, decl: 2, sub: 0, line: 37 } |  |  | 0.502 |
| walker |  | 8199 | 142 | Code::CodeKey { rung: Decl, file: internal/models/generate_models.go, decl: 4, sub: 0, line: 60 } |  |  | 0.502 |
| walker |  | 8230 | 31 | Code::CodeKey { rung: Doc, file: internal/models/generate_models.go, decl: 2, sub: 0, line: 37 } |  |  | 0.502 |
| walker |  | 8262 | 32 | Code::CodeKey { rung: Doc, file: internal/models/generate_models.go, decl: 3, sub: 0, line: 50 } |  |  | 0.502 |
| ns | 8336 |  | 306 | Hook executor and validator symbol rosters | 7.5 |  | 0.493 |
| walker |  | 8461 | 199 | Code::CodeKey { rung: Decl, file: internal/models/generate_models.go, decl: 1, sub: 0, line: 18 } |  |  | 0.493 |
| ns | 8493 |  | 157 | README: hooks.yml file locations and the --no-hooks escape hatch | 7.6 | 1.6 | 0.489 |
| walker |  | 8502 | 41 | Code::CodeKey { rung: Doc, file: internal/models/generate_models.go, decl: 4, sub: 0, line: 60 } |  |  | 0.489 |
| walker |  | 8550 | 48 | Code::CodeKey { rung: Doc, file: internal/models/generate_models.go, decl: 1, sub: 0, line: 18 } |  |  | 0.489 |
| walker |  | 8611 | 61 | Code::CodeKey { rung: Doc, file: internal/hooks/schemas.go, decl: 3, sub: 0, line: 32 } |  |  | 0.489 |
| ns | 8682 |  | 189 | Script mode: a worked frontmatter example and the variable rules | 8.1 |  | 0.500 |
| ns | 8813 |  | 131 | Every top-level symbol of cmd/script.go (locations) | 8.2 |  | 0.509 |
| walker |  | 8928 | 317 | Code::CodeKey { rung: Names, file: cmd/root.go, decl: 0, sub: 0, line: 0 } |  |  | 0.509 |
| walker |  | 8941 | 13 | Code::CodeKey { rung: Decl, file: cmd/root.go, decl: 2, sub: 0, line: 67 } |  |  | 0.509 |
| walker |  | 8952 | 11 | Code::CodeKey { rung: Body, file: cmd/root.go, decl: 3, sub: 0, line: 71 } |  |  | 0.509 |
| walker |  | 9026 | 74 | Code::CodeKey { rung: Decl, file: cmd/root.go, decl: 1, sub: 0, line: 27 } |  |  | 0.509 |
| walker |  | 9041 | 15 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 2, sub: 0, line: 67 } |  |  | 0.509 |
| ns | 9111 |  | 298 | Session file format: Session, Metadata, Message and ToolCall fields | 8.3 |  | 0.500 |
| walker |  | 9290 | 249 | Code::CodeKey { rung: Names, file: cmd/root.go, decl: 0, sub: 1, line: 0 } |  |  | 0.501 |
| walker |  | 9309 | 19 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 16, sub: 0, line: 794 } |  |  | 0.501 |
| walker |  | 9331 | 22 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 15, sub: 0, line: 777 } |  |  | 0.501 |
| ns | 9405 |  | 294 | The public SDK surface: Options and every exported symbol | 8.4 |  | 0.514 |
| walker |  | 9545 | 214 | Code::CodeKey { rung: Decl, file: cmd/root.go, decl: 14, sub: 0, line: 758 } |  |  | 0.522 |
| walker |  | 9555 | 10 | Code::CodeKey { rung: Body, file: cmd/root.go, decl: 12, sub: 0, line: 361 } |  |  | 0.522 |
| ns | 9607 |  | 202 | The complete slash-command table: names and descriptions | 8.5 |  | 0.516 |
| walker |  | 9608 | 53 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 7, sub: 0, line: 131 } |  |  | 0.516 |
| walker |  | 9666 | 58 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 14, sub: 0, line: 758 } |  |  | 0.516 |
| ns | 9828 |  | 221 | ui.SetupCLI: the AgentInterface contract and CLISetupOptions | 8.6 |  | 0.509 |
| walker |  | 9848 | 182 | Code::CodeKey { rung: Names, file: cmd/root.go, decl: 0, sub: 2, line: 0 } |  |  | 0.514 |
| walker |  | 9867 | 19 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 17, sub: 0, line: 811 } |  |  | 0.514 |
| walker |  | 9886 | 19 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 20, sub: 0, line: 1229 } |  |  | 0.514 |
| walker |  | 9906 | 20 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 19, sub: 0, line: 1195 } |  |  | 0.514 |
| walker |  | 9929 | 23 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 18, sub: 0, line: 879 } |  |  | 0.514 |
| ns | 9984 |  | 156 | Release packaging: the goreleaser build matrix | 9.1 |  | 0.509 |
| walker |  | 9996 | 67 | Code::CodeKey { rung: Names, file: cmd/root.go, decl: 0, sub: 3, line: 0 } |  |  | 0.510 |

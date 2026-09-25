Score(3000)=0.736 I=0.884 C=0.613 ns_rows≤3K=18/51 grid(1000/1442/2080/3000/4327/6240/9000)=0.893/0.871/0.814/0.736/0.624/0.533/0.513

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
| walker |  | 952 | 40 | Markdown::Section { file: README.md, section_index: 33, keeps_default_concavity: false } |  |  | 0.832 |
| walker |  | 978 | 26 | Fs::DirListing { dir: internal/tools } |  |  | 0.890 |
| ns | 999 |  | 218 | Complete listings for the builtin / hooks / session / auth / tokens / ui packages | 1.10 |  | 0.893 |
| walker |  | 1005 | 27 | Code::CodeKey { rung: Names, file: main.go, decl: 0, sub: 0, line: 0 } |  |  | 0.893 |
| ns | 1089 |  | 90 | Complete listings for examples/, contribute/ and .github/ | 1.11 |  | 0.850 |
| walker |  | 1135 | 130 | Code::CodeKey { rung: Decl, file: main.go, decl: 2, sub: 0, line: 14 } |  |  | 0.856 |
| walker |  | 1193 | 58 | Markdown::Section { file: README.md, section_index: 31, keeps_default_concavity: false } |  |  | 0.859 |
| ns | 1243 |  | 154 | main.go entry point | 2.1 |  | 0.861 |
| walker |  | 1259 | 66 | Markdown::Section { file: README.md, section_index: 34, keeps_default_concavity: false } |  |  | 0.864 |
| walker |  | 1362 | 103 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.868 |
| walker |  | 1454 | 92 | Markdown::Section { file: README.md, section_index: 32, keeps_default_concavity: false } |  |  | 0.871 |
| ns | 1504 |  | 261 | Complete cobra command tree: script, auth (login/logout/status), hooks (list/validate/init) | 2.2 |  | 0.826 |
| walker |  | 1652 | 198 | Code::CodeKey { rung: ModuleDoc, file: internal/tokens/anthropic.go, decl: 0, sub: 0, line: 0 } |  |  | 0.826 |
| walker |  | 1658 | 6 | Fs::DirListing { dir: sdk/examples } |  |  | 0.841 |
| walker |  | 1729 | 71 | Code::CodeKey { rung: Names, file: cmd/hooks.go, decl: 0, sub: 0, line: 0 } |  |  | 0.841 |
| ns | 1754 |  | 250 | Persistent flag registration, part 1: config, system-prompt, model, debug, prompt, quiet | 2.3 |  | 0.810 |
| walker |  | 1776 | 47 | Code::CodeKey { rung: Decl, file: cmd/hooks.go, decl: 1, sub: 0, line: 16 } |  |  | 0.810 |
| walker |  | 1961 | 185 | Code::CodeKey { rung: Decl, file: cmd/hooks.go, decl: 3, sub: 0, line: 57 } |  |  | 0.812 |
| ns | 2098 |  | 344 | Persistent flag registration, part 2: no-exit, max-steps, stream, compact, no-hooks, approve-tool-run, session flags | 2.4 |  | 0.771 |
| ns | 2224 |  | 126 | Provider and TLS flag registration | 2.5 |  | 0.764 |
| walker |  | 2281 | 320 | Code::CodeKey { rung: Decl, file: cmd/hooks.go, decl: 2, sub: 0, line: 25 } |  |  | 0.766 |
| walker |  | 2332 | 51 | Code::CodeKey { rung: Doc, file: cmd/hooks.go, decl: 3, sub: 0, line: 57 } |  |  | 0.766 |
| walker |  | 2386 | 54 | Code::CodeKey { rung: Doc, file: cmd/hooks.go, decl: 2, sub: 0, line: 25 } |  |  | 0.766 |
| walker |  | 2445 | 59 | Code::CodeKey { rung: Doc, file: cmd/hooks.go, decl: 1, sub: 0, line: 16 } |  |  | 0.766 |
| walker |  | 2485 | 40 | Plaintext::DeclSurface { file: contribute/build.sh } |  |  | 0.766 |
| walker |  | 2496 | 11 | Plaintext::Whole { file: contribute/build.sh } |  |  | 0.766 |
| ns | 2502 |  | 278 | Generation-parameter and Ollama flag registration, with the hidden flag | 2.6 |  | 0.747 |
| walker |  | 2656 | 160 | Code::CodeKey { rung: Names, file: cmd/auth.go, decl: 0, sub: 0, line: 0 } |  |  | 0.747 |
| ns | 2749 |  | 247 | MCPServerConfig: complete field set including the legacy block | 3.1 |  | 0.721 |
| walker |  | 2775 | 119 | Code::CodeKey { rung: Decl, file: cmd/auth.go, decl: 4, sub: 0, line: 78 } |  |  | 0.725 |
| walker |  | 2936 | 161 | Code::CodeKey { rung: Decl, file: cmd/auth.go, decl: 1, sub: 0, line: 17 } |  |  | 0.729 |
| ns | 3069 |  | 320 | Config struct: application-level keys | 3.2 |  | 0.707 |
| walker |  | 3103 | 167 | Code::CodeKey { rung: Decl, file: cmd/auth.go, decl: 3, sub: 0, line: 58 } |  |  | 0.714 |
| ns | 3250 |  | 181 | Config struct: generation-parameter and TLS keys | 3.3 |  | 0.698 |
| walker |  | 3278 | 175 | Code::CodeKey { rung: Decl, file: cmd/auth.go, decl: 2, sub: 0, line: 37 } |  |  | 0.706 |
| walker |  | 3329 | 51 | Code::CodeKey { rung: Doc, file: cmd/auth.go, decl: 3, sub: 0, line: 58 } |  |  | 0.706 |
| walker |  | 3381 | 52 | Code::CodeKey { rung: Doc, file: cmd/auth.go, decl: 2, sub: 0, line: 37 } |  |  | 0.706 |
| walker |  | 3435 | 54 | Code::CodeKey { rung: Doc, file: cmd/auth.go, decl: 4, sub: 0, line: 78 } |  |  | 0.706 |
| walker |  | 3499 | 64 | Code::CodeKey { rung: Doc, file: cmd/auth.go, decl: 1, sub: 0, line: 17 } |  |  | 0.706 |
| ns | 3514 |  | 264 | GetTransportType: type-to-transport mapping and legacy inference | 3.4 |  | 0.670 |
| walker |  | 3564 | 65 | Code::CodeKey { rung: Names, file: sdk/types.go, decl: 0, sub: 0, line: 0 } |  |  | 0.670 |
| walker |  | 3575 | 11 | Code::CodeKey { rung: Body, file: sdk/types.go, decl: 3, sub: 0, line: 18 } |  |  | 0.670 |
| walker |  | 3587 | 12 | Code::CodeKey { rung: Body, file: sdk/types.go, decl: 4, sub: 0, line: 24 } |  |  | 0.670 |
| walker |  | 3619 | 32 | Code::CodeKey { rung: Doc, file: sdk/types.go, decl: 3, sub: 0, line: 18 } |  |  | 0.670 |
| walker |  | 3652 | 33 | Code::CodeKey { rung: Doc, file: sdk/types.go, decl: 1, sub: 0, line: 10 } |  |  | 0.670 |
| walker |  | 3686 | 34 | Code::CodeKey { rung: Doc, file: sdk/types.go, decl: 2, sub: 0, line: 14 } |  |  | 0.670 |
| walker |  | 3720 | 34 | Code::CodeKey { rung: Doc, file: sdk/types.go, decl: 4, sub: 0, line: 24 } |  |  | 0.670 |
| ns | 3938 |  | 424 | Config.Validate: required fields per transport and filter exclusivity | 3.5 |  | 0.636 |
| ns | 4141 |  | 203 | Substitution engine: the two regexes plus every symbol in substitution.go | 3.6 |  | 0.624 |
| walker |  | 4234 | 514 | GoMod::File { file: go.mod } |  |  | 0.624 |
| walker |  | 4238 | 4 | Fs::DirListing { dir: sdk/examples/basic } |  |  | 0.630 |
| walker |  | 4242 | 4 | Fs::DirListing { dir: sdk/examples/scripting } |  |  | 0.637 |
| ns | 4295 |  | 154 | Remaining top-level symbols of internal/config/config.go and all of merger.go (locations) | 3.7 |  | 0.624 |
| walker |  | 4455 | 213 | Code::CodeKey { rung: Names, file: sdk/mcphost.go, decl: 0, sub: 0, line: 0 } |  |  | 0.625 |
| walker |  | 4490 | 35 | Code::CodeKey { rung: Decl, file: sdk/mcphost.go, decl: 1, sub: 0, line: 19 } |  |  | 0.625 |
| walker |  | 4499 | 9 | Code::CodeKey { rung: Body, file: sdk/mcphost.go, decl: 6, sub: 0, line: 201 } |  |  | 0.625 |
| walker |  | 4508 | 9 | Code::CodeKey { rung: Body, file: sdk/mcphost.go, decl: 10, sub: 0, line: 230 } |  |  | 0.625 |
| walker |  | 4517 | 9 | Code::CodeKey { rung: Body, file: sdk/mcphost.go, decl: 11, sub: 0, line: 237 } |  |  | 0.625 |
| ns | 4541 |  | 246 | Agent struct and its seven callback handler types | 4.1 |  | 0.609 |
| walker |  | 4585 | 68 | Code::CodeKey { rung: Decl, file: sdk/mcphost.go, decl: 5, sub: 0, line: 163 } |  |  | 0.609 |
| walker |  | 4692 | 107 | Code::CodeKey { rung: Decl, file: sdk/mcphost.go, decl: 2, sub: 0, line: 28 } |  |  | 0.610 |
| walker |  | 4721 | 29 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 6, sub: 0, line: 201 } |  |  | 0.610 |
| walker |  | 4750 | 29 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 9, sub: 0, line: 224 } |  |  | 0.610 |
| walker |  | 4787 | 37 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 8, sub: 0, line: 218 } |  |  | 0.610 |
| ns | 4815 |  | 274 | Every top-level symbol of internal/agent/agent.go (locations) | 4.2 |  | 0.595 |
| walker |  | 4826 | 39 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 7, sub: 0, line: 207 } |  |  | 0.595 |
| walker |  | 4873 | 47 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 2, sub: 0, line: 28 } |  |  | 0.595 |
| walker |  | 4921 | 48 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 11, sub: 0, line: 237 } |  |  | 0.595 |
| walker |  | 4970 | 49 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 10, sub: 0, line: 230 } |  |  | 0.595 |
| ns | 5007 |  | 192 | The tool-calling loop: step bound, tool-call branch and the approval gate | 4.3 | 4.2 | 0.584 |
| walker |  | 5022 | 52 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 1, sub: 0, line: 19 } |  |  | 0.584 |
| walker |  | 5078 | 56 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 5, sub: 0, line: 163 } |  |  | 0.584 |
| walker |  | 5135 | 57 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 3, sub: 0, line: 40 } |  |  | 0.584 |
| ns | 5186 |  | 179 | agent factory: AgentCreationOptions fields and both functions | 4.4 |  | 0.574 |
| walker |  | 5193 | 58 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 4, sub: 0, line: 130 } |  |  | 0.574 |
| walker |  | 5473 | 280 | Code::CodeKey { rung: Names, file: cmd/script.go, decl: 0, sub: 0, line: 0 } |  |  | 0.575 |
| walker |  | 5538 | 65 | Code::CodeKey { rung: Decl, file: cmd/script.go, decl: 9, sub: 0, line: 423 } |  |  | 0.575 |
| walker |  | 5549 | 11 | Code::CodeKey { rung: Body, file: cmd/script.go, decl: 2, sub: 0, line: 78 } |  |  | 0.559 |
| ns | 5549 |  | 363 | Every top-level symbol of internal/tools/mcp.go (locations) | 4.5 |  | 0.559 |
| walker |  | 5565 | 16 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 4, sub: 0, line: 148 } |  |  | 0.559 |
| walker |  | 5581 | 16 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 7, sub: 0, line: 279 } |  |  | 0.559 |
| walker |  | 5599 | 18 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 14, sub: 0, line: 511 } |  |  | 0.559 |
| walker |  | 5618 | 19 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 8, sub: 0, line: 288 } |  |  | 0.559 |
| walker |  | 5638 | 20 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 6, sub: 0, line: 255 } |  |  | 0.559 |
| walker |  | 5669 | 31 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 12, sub: 0, line: 480 } |  |  | 0.559 |
| walker |  | 5702 | 33 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 10, sub: 0, line: 431 } |  |  | 0.559 |
| walker |  | 5738 | 36 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 11, sub: 0, line: 442 } |  |  | 0.559 |
| ns | 5765 |  | 216 | AgenticLoopConfig head and every mode-driving function in cmd/root.go (locations) | 4.6 |  | 0.548 |
| walker |  | 5782 | 44 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 13, sub: 0, line: 499 } |  |  | 0.548 |
| walker |  | 5829 | 47 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 3, sub: 0, line: 84 } |  |  | 0.548 |
| walker |  | 5882 | 53 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 9, sub: 0, line: 423 } |  |  | 0.548 |
| ns | 5907 |  | 142 | CreateProvider: the complete list of supported providers | 5.1 |  | 0.540 |
| ns | 6235 |  | 328 | Every top-level symbol of internal/models/providers.go (locations) | 5.2 | 5.1 | 0.525 |
| ns | 6385 |  | 150 | ModelsRegistry: model validation and suggestion API | 5.3 |  | 0.519 |
| ns | 6477 |  | 92 | The generated model catalogue: generator types and the DO-NOT-EDIT header | 5.4 |  | 0.515 |
| walker |  | 6512 | 630 | Code::CodeKey { rung: Decl, file: cmd/script.go, decl: 1, sub: 0, line: 28 } |  |  | 0.523 |
| walker |  | 6573 | 61 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 1, sub: 0, line: 28 } |  |  | 0.523 |
| walker |  | 6634 | 61 | Code::CodeKey { rung: Names, file: internal/models/models_data.go, decl: 0, sub: 0, line: 0 } |  |  | 0.523 |
| ns | 6653 |  | 176 | Builtin server registry: the complete set of in-process servers | 6.1 |  | 0.514 |
| walker |  | 6654 | 20 | Code::CodeKey { rung: Decl, file: internal/models/models_data.go, decl: 3, sub: 0, line: 26 } |  |  | 0.514 |
| walker |  | 6700 | 46 | Code::CodeKey { rung: Decl, file: internal/models/models_data.go, decl: 2, sub: 0, line: 18 } |  |  | 0.514 |
| walker |  | 6752 | 52 | Code::CodeKey { rung: Decl, file: internal/models/models_data.go, decl: 4, sub: 0, line: 32 } |  |  | 0.514 |
| walker |  | 6818 | 66 | Code::CodeKey { rung: Decl, file: internal/models/models_data.go, decl: 1, sub: 0, line: 7 } |  |  | 0.514 |
| ns | 6825 |  | 172 | Every top-level symbol of internal/builtin/registry.go (locations) | 6.2 | 6.1 | 0.508 |
| walker |  | 6830 | 12 | Code::CodeKey { rung: Doc, file: internal/models/models_data.go, decl: 2, sub: 0, line: 18 } |  |  | 0.508 |
| walker |  | 6842 | 12 | Code::CodeKey { rung: Doc, file: internal/models/models_data.go, decl: 4, sub: 0, line: 32 } |  |  | 0.508 |
| walker |  | 6856 | 14 | Code::CodeKey { rung: Doc, file: internal/models/models_data.go, decl: 1, sub: 0, line: 7 } |  |  | 0.508 |
| walker |  | 6870 | 14 | Code::CodeKey { rung: Doc, file: internal/models/models_data.go, decl: 3, sub: 0, line: 26 } |  |  | 0.508 |
| walker |  | 6885 | 15 | Code::CodeKey { rung: Doc, file: internal/models/models_data.go, decl: 5, sub: 0, line: 41 } |  |  | 0.508 |
| walker |  | 6959 | 74 | Code::CodeKey { rung: Names, file: internal/hooks/schemas.go, decl: 0, sub: 0, line: 0 } |  |  | 0.508 |
| walker |  | 6983 | 24 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 4, sub: 0, line: 42 } |  |  | 0.508 |
| walker |  | 7027 | 44 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 2, sub: 0, line: 23 } |  |  | 0.508 |
| ns | 7042 |  | 217 | Bash builtin: output/timeout limits and the complete banned-command list | 6.3 |  | 0.496 |
| walker |  | 7088 | 61 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 3, sub: 0, line: 32 } |  |  | 0.496 |
| walker |  | 7177 | 89 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 6, sub: 0, line: 62 } |  |  | 0.496 |
| ns | 7240 |  | 198 | The http builtin: its four tools and every symbol in http.go (locations) | 6.4 |  | 0.489 |
| walker |  | 7284 | 107 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 5, sub: 0, line: 50 } |  |  | 0.489 |
| ns | 7352 |  | 112 | HookEvent: the complete set of hook events | 7.1 |  | 0.486 |
| walker |  | 7443 | 159 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 1, sub: 0, line: 10 } |  |  | 0.487 |
| walker |  | 7494 | 51 | Code::CodeKey { rung: Doc, file: internal/hooks/schemas.go, decl: 2, sub: 0, line: 23 } |  |  | 0.487 |
| walker |  | 7547 | 53 | Code::CodeKey { rung: Doc, file: internal/hooks/schemas.go, decl: 1, sub: 0, line: 10 } |  |  | 0.487 |
| ns | 7567 |  | 215 | Hook configuration schema: HookConfig, HookMatcher, HookEntry | 7.2 |  | 0.479 |
| walker |  | 7600 | 53 | Code::CodeKey { rung: Doc, file: internal/hooks/schemas.go, decl: 4, sub: 0, line: 42 } |  |  | 0.479 |
| walker |  | 7655 | 55 | Code::CodeKey { rung: Doc, file: internal/hooks/schemas.go, decl: 5, sub: 0, line: 50 } |  |  | 0.479 |
| walker |  | 7717 | 62 | Plaintext::DeclSurface { file: contribute/boost.sh } |  |  | 0.479 |
| ns | 7839 |  | 272 | Hook wire protocol: CommonInput and HookOutput | 7.3 |  | 0.493 |
| walker |  | 7991 | 274 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.495 |
| ns | 8030 |  | 191 | Per-event hook input structs | 7.4 |  | 0.505 |
| walker |  | 8067 | 76 | Code::CodeKey { rung: Names, file: internal/models/generate_models.go, decl: 0, sub: 0, line: 0 } |  |  | 0.508 |
| walker |  | 8124 | 57 | Code::CodeKey { rung: Decl, file: internal/models/generate_models.go, decl: 3, sub: 0, line: 50 } |  |  | 0.508 |
| walker |  | 8255 | 131 | Code::CodeKey { rung: Decl, file: internal/models/generate_models.go, decl: 2, sub: 0, line: 37 } |  |  | 0.508 |
| ns | 8336 |  | 306 | Hook executor and validator symbol rosters | 7.5 |  | 0.498 |
| walker |  | 8397 | 142 | Code::CodeKey { rung: Decl, file: internal/models/generate_models.go, decl: 4, sub: 0, line: 60 } |  |  | 0.498 |
| walker |  | 8428 | 31 | Code::CodeKey { rung: Doc, file: internal/models/generate_models.go, decl: 2, sub: 0, line: 37 } |  |  | 0.498 |
| walker |  | 8460 | 32 | Code::CodeKey { rung: Doc, file: internal/models/generate_models.go, decl: 3, sub: 0, line: 50 } |  |  | 0.498 |
| ns | 8493 |  | 157 | README: hooks.yml file locations and the --no-hooks escape hatch | 7.6 | 1.6 | 0.494 |
| walker |  | 8659 | 199 | Code::CodeKey { rung: Decl, file: internal/models/generate_models.go, decl: 1, sub: 0, line: 18 } |  |  | 0.494 |
| ns | 8682 |  | 189 | Script mode: a worked frontmatter example and the variable rules | 8.1 |  | 0.505 |
| walker |  | 8700 | 41 | Code::CodeKey { rung: Doc, file: internal/models/generate_models.go, decl: 4, sub: 0, line: 60 } |  |  | 0.505 |
| walker |  | 8748 | 48 | Code::CodeKey { rung: Doc, file: internal/models/generate_models.go, decl: 1, sub: 0, line: 18 } |  |  | 0.505 |
| walker |  | 8809 | 61 | Code::CodeKey { rung: Doc, file: internal/hooks/schemas.go, decl: 3, sub: 0, line: 32 } |  |  | 0.505 |
| ns | 8813 |  | 131 | Every top-level symbol of cmd/script.go (locations) | 8.2 |  | 0.513 |
| ns | 9111 |  | 298 | Session file format: Session, Metadata, Message and ToolCall fields | 8.3 |  | 0.505 |
| walker |  | 9126 | 317 | Code::CodeKey { rung: Names, file: cmd/root.go, decl: 0, sub: 0, line: 0 } |  |  | 0.505 |
| walker |  | 9139 | 13 | Code::CodeKey { rung: Decl, file: cmd/root.go, decl: 2, sub: 0, line: 67 } |  |  | 0.505 |
| walker |  | 9150 | 11 | Code::CodeKey { rung: Body, file: cmd/root.go, decl: 3, sub: 0, line: 71 } |  |  | 0.505 |
| walker |  | 9224 | 74 | Code::CodeKey { rung: Decl, file: cmd/root.go, decl: 1, sub: 0, line: 27 } |  |  | 0.505 |
| walker |  | 9239 | 15 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 2, sub: 0, line: 67 } |  |  | 0.505 |
| ns | 9405 |  | 294 | The public SDK surface: Options and every exported symbol | 8.4 |  | 0.518 |
| walker |  | 9488 | 249 | Code::CodeKey { rung: Names, file: cmd/root.go, decl: 0, sub: 1, line: 0 } |  |  | 0.519 |
| walker |  | 9507 | 19 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 16, sub: 0, line: 794 } |  |  | 0.519 |
| walker |  | 9529 | 22 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 15, sub: 0, line: 777 } |  |  | 0.519 |
| ns | 9607 |  | 202 | The complete slash-command table: names and descriptions | 8.5 |  | 0.513 |
| walker |  | 9743 | 214 | Code::CodeKey { rung: Decl, file: cmd/root.go, decl: 14, sub: 0, line: 758 } |  |  | 0.521 |
| walker |  | 9753 | 10 | Code::CodeKey { rung: Body, file: cmd/root.go, decl: 12, sub: 0, line: 361 } |  |  | 0.521 |
| walker |  | 9806 | 53 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 7, sub: 0, line: 131 } |  |  | 0.521 |
| ns | 9828 |  | 221 | ui.SetupCLI: the AgentInterface contract and CLISetupOptions | 8.6 |  | 0.513 |
| walker |  | 9864 | 58 | Code::CodeKey { rung: Doc, file: cmd/root.go, decl: 14, sub: 0, line: 758 } |  |  | 0.513 |
| ns | 9984 |  | 156 | Release packaging: the goreleaser build matrix | 9.1 |  | 0.508 |
| walker |  | 9999 | 135 | Code::CodeKey { rung: Names, file: cmd/root.go, decl: 0, sub: 2, line: 0 } |  |  | 0.512 |

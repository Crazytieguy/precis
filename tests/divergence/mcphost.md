Score(3000)=0.714 I=0.875 C=0.582 ns_rows≤3K=18/51 grid(1000/1442/2080/3000/4327/6240/9000)=0.891/0.857/0.783/0.714/0.614/0.520/0.472

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
| walker |  | 4624 | 231 | Code::CodeKey { rung: Names, file: internal/ui/cli.go, decl: 0, sub: 0, line: 0 } |  |  | 0.604 |
| walker |  | 4636 | 12 | Code::CodeKey { rung: Body, file: internal/ui/cli.go, decl: 5, sub: 0, line: 65 } |  |  | 0.604 |
| walker |  | 4650 | 14 | Code::CodeKey { rung: Body, file: internal/ui/cli.go, decl: 10, sub: 0, line: 152 } |  |  | 0.604 |
| walker |  | 4811 | 161 | Code::CodeKey { rung: Decl, file: internal/ui/cli.go, decl: 2, sub: 0, line: 22 } |  |  | 0.604 |
| ns | 4815 |  | 274 | Every top-level symbol of internal/agent/agent.go (locations) | 4.2 |  | 0.588 |
| walker |  | 4849 | 38 | Code::CodeKey { rung: Doc, file: internal/ui/cli.go, decl: 5, sub: 0, line: 65 } |  |  | 0.588 |
| walker |  | 4888 | 39 | Code::CodeKey { rung: Doc, file: internal/ui/cli.go, decl: 6, sub: 0, line: 71 } |  |  | 0.588 |
| walker |  | 4937 | 49 | Code::CodeKey { rung: Doc, file: internal/ui/cli.go, decl: 4, sub: 0, line: 56 } |  |  | 0.588 |
| walker |  | 4988 | 51 | Code::CodeKey { rung: Doc, file: internal/ui/cli.go, decl: 10, sub: 0, line: 152 } |  |  | 0.588 |
| ns | 5007 |  | 192 | The tool-calling loop: step bound, tool-call branch and the approval gate | 4.3 | 4.2 | 0.578 |
| walker |  | 5040 | 52 | Code::CodeKey { rung: Doc, file: internal/ui/cli.go, decl: 8, sub: 0, line: 124 } |  |  | 0.578 |
| walker |  | 5093 | 53 | Code::CodeKey { rung: Doc, file: internal/ui/cli.go, decl: 9, sub: 0, line: 138 } |  |  | 0.578 |
| walker |  | 5149 | 56 | Code::CodeKey { rung: Doc, file: internal/ui/cli.go, decl: 11, sub: 0, line: 159 } |  |  | 0.578 |
| ns | 5186 |  | 179 | agent factory: AgentCreationOptions fields and both functions | 4.4 |  | 0.568 |
| walker |  | 5362 | 213 | Code::CodeKey { rung: Names, file: sdk/mcphost.go, decl: 0, sub: 0, line: 0 } |  |  | 0.569 |
| walker |  | 5397 | 35 | Code::CodeKey { rung: Decl, file: sdk/mcphost.go, decl: 1, sub: 0, line: 19 } |  |  | 0.569 |
| walker |  | 5406 | 9 | Code::CodeKey { rung: Body, file: sdk/mcphost.go, decl: 6, sub: 0, line: 201 } |  |  | 0.569 |
| walker |  | 5415 | 9 | Code::CodeKey { rung: Body, file: sdk/mcphost.go, decl: 10, sub: 0, line: 230 } |  |  | 0.569 |
| walker |  | 5424 | 9 | Code::CodeKey { rung: Body, file: sdk/mcphost.go, decl: 11, sub: 0, line: 237 } |  |  | 0.569 |
| walker |  | 5492 | 68 | Code::CodeKey { rung: Decl, file: sdk/mcphost.go, decl: 5, sub: 0, line: 163 } |  |  | 0.569 |
| ns | 5549 |  | 363 | Every top-level symbol of internal/tools/mcp.go (locations) | 4.5 |  | 0.553 |
| walker |  | 5599 | 107 | Code::CodeKey { rung: Decl, file: sdk/mcphost.go, decl: 2, sub: 0, line: 28 } |  |  | 0.554 |
| walker |  | 5628 | 29 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 6, sub: 0, line: 201 } |  |  | 0.554 |
| walker |  | 5657 | 29 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 9, sub: 0, line: 224 } |  |  | 0.554 |
| walker |  | 5694 | 37 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 8, sub: 0, line: 218 } |  |  | 0.554 |
| walker |  | 5733 | 39 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 7, sub: 0, line: 207 } |  |  | 0.554 |
| ns | 5765 |  | 216 | AgenticLoopConfig head and every mode-driving function in cmd/root.go (locations) | 4.6 |  | 0.542 |
| walker |  | 5780 | 47 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 2, sub: 0, line: 28 } |  |  | 0.542 |
| walker |  | 5828 | 48 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 11, sub: 0, line: 237 } |  |  | 0.542 |
| walker |  | 5877 | 49 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 10, sub: 0, line: 230 } |  |  | 0.542 |
| ns | 5907 |  | 142 | CreateProvider: the complete list of supported providers | 5.1 |  | 0.535 |
| walker |  | 5929 | 52 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 1, sub: 0, line: 19 } |  |  | 0.535 |
| walker |  | 5985 | 56 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 5, sub: 0, line: 163 } |  |  | 0.535 |
| walker |  | 6042 | 57 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 3, sub: 0, line: 40 } |  |  | 0.535 |
| walker |  | 6100 | 58 | Code::CodeKey { rung: Doc, file: sdk/mcphost.go, decl: 4, sub: 0, line: 130 } |  |  | 0.535 |
| walker |  | 6162 | 62 | Plaintext::DeclSurface { file: contribute/boost.sh } |  |  | 0.535 |
| ns | 6235 |  | 328 | Every top-level symbol of internal/models/providers.go (locations) | 5.2 | 5.1 | 0.520 |
| ns | 6385 |  | 150 | ModelsRegistry: model validation and suggestion API | 5.3 |  | 0.514 |
| walker |  | 6442 | 280 | Code::CodeKey { rung: Names, file: cmd/script.go, decl: 0, sub: 0, line: 0 } |  |  | 0.515 |
| ns | 6477 |  | 92 | The generated model catalogue: generator types and the DO-NOT-EDIT header | 5.4 |  | 0.511 |
| walker |  | 6507 | 65 | Code::CodeKey { rung: Decl, file: cmd/script.go, decl: 9, sub: 0, line: 423 } |  |  | 0.511 |
| walker |  | 6518 | 11 | Code::CodeKey { rung: Body, file: cmd/script.go, decl: 2, sub: 0, line: 78 } |  |  | 0.511 |
| walker |  | 6571 | 53 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 9, sub: 0, line: 423 } |  |  | 0.511 |
| walker |  | 6587 | 16 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 4, sub: 0, line: 148 } |  |  | 0.511 |
| walker |  | 6603 | 16 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 7, sub: 0, line: 279 } |  |  | 0.511 |
| walker |  | 6621 | 18 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 14, sub: 0, line: 511 } |  |  | 0.511 |
| walker |  | 6640 | 19 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 8, sub: 0, line: 288 } |  |  | 0.511 |
| ns | 6653 |  | 176 | Builtin server registry: the complete set of in-process servers | 6.1 |  | 0.502 |
| walker |  | 6660 | 20 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 6, sub: 0, line: 255 } |  |  | 0.502 |
| walker |  | 6691 | 31 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 12, sub: 0, line: 480 } |  |  | 0.502 |
| walker |  | 6724 | 33 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 10, sub: 0, line: 431 } |  |  | 0.502 |
| walker |  | 6760 | 36 | Code::CodeKey { rung: Doc, file: cmd/script.go, decl: 11, sub: 0, line: 442 } |  |  | 0.502 |
| ns | 6825 |  | 172 | Every top-level symbol of internal/builtin/registry.go (locations) | 6.2 | 6.1 | 0.496 |
| walker |  | 6826 | 66 | Code::CodeKey { rung: Doc, file: internal/ui/cli.go, decl: 3, sub: 0, line: 40 } |  |  | 0.496 |
| ns | 7042 |  | 217 | Bash builtin: output/timeout limits and the complete banned-command list | 6.3 |  | 0.485 |
| walker |  | 7062 | 236 | Code::CodeKey { rung: Names, file: internal/agent/agent.go, decl: 0, sub: 0, line: 0 } |  |  | 0.493 |
| walker |  | 7122 | 60 | Code::CodeKey { rung: Decl, file: internal/agent/agent.go, decl: 12, sub: 0, line: 135 } |  |  | 0.493 |
| walker |  | 7189 | 67 | Code::CodeKey { rung: Decl, file: internal/agent/agent.go, decl: 13, sub: 0, line: 144 } |  |  | 0.493 |
| ns | 7240 |  | 198 | The http builtin: its four tools and every symbol in http.go (locations) | 6.4 |  | 0.486 |
| walker |  | 7265 | 76 | Code::CodeKey { rung: Decl, file: internal/agent/agent.go, decl: 11, sub: 0, line: 125 } |  |  | 0.486 |
| ns | 7352 |  | 112 | HookEvent: the complete set of hook events | 7.1 |  | 0.482 |
| walker |  | 7368 | 103 | Code::CodeKey { rung: Decl, file: internal/agent/agent.go, decl: 9, sub: 0, line: 67 } |  |  | 0.499 |
| walker |  | 7532 | 164 | Code::CodeKey { rung: Decl, file: internal/agent/agent.go, decl: 1, sub: 0, line: 22 } |  |  | 0.499 |
| walker |  | 7562 | 30 | Code::CodeKey { rung: Doc, file: internal/agent/agent.go, decl: 5, sub: 0, line: 51 } |  |  | 0.499 |
| ns | 7567 |  | 215 | Hook configuration schema: HookConfig, HookMatcher, HookEntry | 7.2 |  | 0.492 |
| walker |  | 7596 | 34 | Code::CodeKey { rung: Doc, file: internal/agent/agent.go, decl: 1, sub: 0, line: 22 } |  |  | 0.492 |
| walker |  | 7630 | 34 | Code::CodeKey { rung: Doc, file: internal/agent/agent.go, decl: 6, sub: 0, line: 55 } |  |  | 0.492 |
| walker |  | 7667 | 37 | Code::CodeKey { rung: Doc, file: internal/agent/agent.go, decl: 7, sub: 0, line: 59 } |  |  | 0.492 |
| walker |  | 7705 | 38 | Code::CodeKey { rung: Doc, file: internal/agent/agent.go, decl: 4, sub: 0, line: 47 } |  |  | 0.492 |
| walker |  | 7744 | 39 | Code::CodeKey { rung: Doc, file: internal/agent/agent.go, decl: 8, sub: 0, line: 63 } |  |  | 0.492 |
| walker |  | 7784 | 40 | Code::CodeKey { rung: Doc, file: internal/agent/agent.go, decl: 2, sub: 0, line: 39 } |  |  | 0.492 |
| walker |  | 7824 | 40 | Code::CodeKey { rung: Doc, file: internal/agent/agent.go, decl: 9, sub: 0, line: 67 } |  |  | 0.492 |
| ns | 7839 |  | 272 | Hook wire protocol: CommonInput and HookOutput | 7.3 |  | 0.485 |
| walker |  | 7864 | 40 | Code::CodeKey { rung: Doc, file: internal/agent/agent.go, decl: 11, sub: 0, line: 125 } |  |  | 0.485 |
| walker |  | 7905 | 41 | Code::CodeKey { rung: Doc, file: internal/agent/agent.go, decl: 3, sub: 0, line: 43 } |  |  | 0.485 |
| walker |  | 7970 | 65 | Code::CodeKey { rung: Doc, file: internal/agent/agent.go, decl: 12, sub: 0, line: 135 } |  |  | 0.485 |
| ns | 8030 |  | 191 | Per-event hook input structs | 7.4 |  | 0.479 |
| walker |  | 8037 | 67 | Code::CodeKey { rung: Doc, file: internal/agent/agent.go, decl: 10, sub: 0, line: 81 } |  |  | 0.479 |
| walker |  | 8109 | 72 | Code::CodeKey { rung: Doc, file: internal/ui/cli.go, decl: 2, sub: 0, line: 22 } |  |  | 0.479 |
| ns | 8336 |  | 306 | Hook executor and validator symbol rosters | 7.5 |  | 0.470 |
| walker |  | 8348 | 239 | Code::CodeKey { rung: Names, file: internal/ui/messages.go, decl: 0, sub: 0, line: 0 } |  |  | 0.470 |
| walker |  | 8357 | 9 | Code::CodeKey { rung: Decl, file: internal/ui/messages.go, decl: 2, sub: 0, line: 17 } |  |  | 0.470 |
| walker |  | 8376 | 19 | Code::CodeKey { rung: Decl, file: internal/ui/messages.go, decl: 5, sub: 0, line: 47 } |  |  | 0.470 |
| walker |  | 8385 | 9 | Code::CodeKey { rung: Body, file: internal/ui/messages.go, decl: 8, sub: 0, line: 79 } |  |  | 0.470 |
| walker |  | 8450 | 65 | Code::CodeKey { rung: Decl, file: internal/ui/messages.go, decl: 3, sub: 0, line: 29 } |  |  | 0.470 |
| walker |  | 8481 | 31 | Code::CodeKey { rung: Doc, file: internal/ui/messages.go, decl: 1, sub: 0, line: 15 } |  |  | 0.470 |
| ns | 8493 |  | 157 | README: hooks.yml file locations and the --no-hooks escape hatch | 7.6 | 1.6 | 0.467 |
| walker |  | 8514 | 33 | Code::CodeKey { rung: Doc, file: internal/ui/messages.go, decl: 8, sub: 0, line: 79 } |  |  | 0.467 |
| walker |  | 8524 | 10 | Code::CodeKey { rung: Doc, file: internal/ui/messages.go, decl: 4, sub: 0, line: 40 } |  |  | 0.467 |
| walker |  | 8571 | 47 | Code::CodeKey { rung: Doc, file: internal/ui/messages.go, decl: 7, sub: 0, line: 70 } |  |  | 0.467 |
| walker |  | 8622 | 51 | Code::CodeKey { rung: Doc, file: internal/ui/messages.go, decl: 3, sub: 0, line: 29 } |  |  | 0.467 |
| walker |  | 8674 | 52 | Code::CodeKey { rung: Doc, file: internal/ui/messages.go, decl: 5, sub: 0, line: 47 } |  |  | 0.467 |
| ns | 8682 |  | 189 | Script mode: a worked frontmatter example and the variable rules | 8.1 |  | 0.462 |
| walker |  | 8731 | 57 | Code::CodeKey { rung: Doc, file: internal/ui/messages.go, decl: 9, sub: 0, line: 86 } |  |  | 0.462 |
| walker |  | 8748 | 17 | Code::CodeKey { rung: Doc, file: internal/ui/messages.go, decl: 6, sub: 0, line: 53 } |  |  | 0.462 |
| walker |  | 8809 | 61 | Code::CodeKey { rung: Names, file: internal/models/models_data.go, decl: 0, sub: 0, line: 0 } |  |  | 0.462 |
| ns | 8813 |  | 131 | Every top-level symbol of cmd/script.go (locations) | 8.2 |  | 0.472 |
| walker |  | 8829 | 20 | Code::CodeKey { rung: Decl, file: internal/models/models_data.go, decl: 3, sub: 0, line: 26 } |  |  | 0.472 |
| walker |  | 8875 | 46 | Code::CodeKey { rung: Decl, file: internal/models/models_data.go, decl: 2, sub: 0, line: 18 } |  |  | 0.472 |
| walker |  | 8927 | 52 | Code::CodeKey { rung: Decl, file: internal/models/models_data.go, decl: 4, sub: 0, line: 32 } |  |  | 0.472 |
| walker |  | 8993 | 66 | Code::CodeKey { rung: Decl, file: internal/models/models_data.go, decl: 1, sub: 0, line: 7 } |  |  | 0.472 |
| walker |  | 9005 | 12 | Code::CodeKey { rung: Doc, file: internal/models/models_data.go, decl: 2, sub: 0, line: 18 } |  |  | 0.472 |
| walker |  | 9017 | 12 | Code::CodeKey { rung: Doc, file: internal/models/models_data.go, decl: 4, sub: 0, line: 32 } |  |  | 0.472 |
| walker |  | 9031 | 14 | Code::CodeKey { rung: Doc, file: internal/models/models_data.go, decl: 1, sub: 0, line: 7 } |  |  | 0.472 |
| walker |  | 9045 | 14 | Code::CodeKey { rung: Doc, file: internal/models/models_data.go, decl: 3, sub: 0, line: 26 } |  |  | 0.472 |
| walker |  | 9060 | 15 | Code::CodeKey { rung: Doc, file: internal/models/models_data.go, decl: 5, sub: 0, line: 41 } |  |  | 0.472 |
| ns | 9111 |  | 298 | Session file format: Session, Metadata, Message and ToolCall fields | 8.3 |  | 0.464 |
| walker |  | 9134 | 74 | Code::CodeKey { rung: Names, file: internal/hooks/schemas.go, decl: 0, sub: 0, line: 0 } |  |  | 0.465 |
| walker |  | 9158 | 24 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 4, sub: 0, line: 42 } |  |  | 0.465 |
| walker |  | 9202 | 44 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 2, sub: 0, line: 23 } |  |  | 0.467 |
| walker |  | 9263 | 61 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 3, sub: 0, line: 32 } |  |  | 0.467 |
| walker |  | 9352 | 89 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 6, sub: 0, line: 62 } |  |  | 0.471 |
| ns | 9405 |  | 294 | The public SDK surface: Options and every exported symbol | 8.4 |  | 0.485 |
| walker |  | 9459 | 107 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 5, sub: 0, line: 50 } |  |  | 0.495 |
| ns | 9607 |  | 202 | The complete slash-command table: names and descriptions | 8.5 |  | 0.490 |
| walker |  | 9618 | 159 | Code::CodeKey { rung: Decl, file: internal/hooks/schemas.go, decl: 1, sub: 0, line: 10 } |  |  | 0.504 |
| walker |  | 9669 | 51 | Code::CodeKey { rung: Doc, file: internal/hooks/schemas.go, decl: 2, sub: 0, line: 23 } |  |  | 0.504 |
| walker |  | 9722 | 53 | Code::CodeKey { rung: Doc, file: internal/hooks/schemas.go, decl: 1, sub: 0, line: 10 } |  |  | 0.504 |
| walker |  | 9775 | 53 | Code::CodeKey { rung: Doc, file: internal/hooks/schemas.go, decl: 4, sub: 0, line: 42 } |  |  | 0.504 |
| ns | 9828 |  | 221 | ui.SetupCLI: the AgentInterface contract and CLISetupOptions | 8.6 |  | 0.497 |
| walker |  | 9830 | 55 | Code::CodeKey { rung: Doc, file: internal/hooks/schemas.go, decl: 5, sub: 0, line: 50 } |  |  | 0.497 |
| walker |  | 9891 | 61 | Code::CodeKey { rung: Doc, file: internal/hooks/schemas.go, decl: 3, sub: 0, line: 32 } |  |  | 0.497 |
| walker |  | 9962 | 71 | Code::CodeKey { rung: Doc, file: internal/hooks/schemas.go, decl: 6, sub: 0, line: 62 } |  |  | 0.497 |
| ns | 9984 |  | 156 | Release packaging: the goreleaser build matrix | 9.1 |  | 0.491 |

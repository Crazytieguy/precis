Score(3000)=0.588 I=0.844 C=0.410 ns_rows≤3K=18/46 (reached=7 partial=1 missing=10)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 40 | 40 | listing of '.' |  |  | 1.000 |
| ns | 40 |  | 40 | Repo root listing | 1.1 |  | 1.000 |
| ns | 52 |  | 12 | README title | 1.2 |  | 0.956 |
| walker |  | 103 | 63 | README headline in README.md |  |  | 1.000 |
| ns | 105 |  | 53 | go.mod module identity | 1.3 |  | 0.832 |
| walker |  | 127 | 24 | README.md section #0 |  |  | 0.832 |
| walker |  | 154 | 27 | go decl names surface in main.go |  |  | 0.832 |
| walker |  | 154 | 0 | go decl at main.go:14 |  |  | 0.832 |
| walker |  | 207 | 53 | go module identity in go.mod |  |  | 1.000 |
| walker |  | 222 | 15 | listing of 'contribute' |  |  | 1.000 |
| ns | 224 |  | 119 | Top package-dir listings (cmd/, internal/, sdk/, examples/, contribute/) | 2.1 |  | 0.574 |
| walker |  | 255 | 33 | headings outline in contribute/contribute.md |  |  | 0.574 |
| walker |  | 263 | 8 | listing of 'contribute/conf' |  |  | 0.574 |
| walker |  | 277 | 14 | README.md section #1 |  |  | 0.574 |
| walker |  | 289 | 12 | README.md section #2 |  |  | 0.574 |
| walker |  | 301 | 12 | README.md section #3 |  |  | 0.574 |
| walker |  | 304 | 3 | listing of '.github' |  |  | 0.574 |
| walker |  | 312 | 8 | listing of '.github/workflows' |  |  | 0.585 |
| ns | 377 |  | 153 | internal/{agent,auth,builtin,config,hooks} file listings | 2.2 |  | 0.427 |
| walker |  | 466 | 154 | YAML config at .github/workflows/ci.yml |  |  | 0.427 |
| ns | 474 |  | 97 | internal/{models,session,tokens,tools} + models subpackage listings | 2.3 |  | 0.374 |
| walker |  | 490 | 24 | listing of 'sdk' |  |  | 0.402 |
| walker |  | 530 | 40 | README headline in sdk/README.md |  |  | 0.402 |
| walker |  | 536 | 6 | listing of 'examples' |  |  | 0.417 |
| ns | 565 |  | 91 | internal/ui/{.,progress} file listing | 2.4 |  | 0.378 |
| walker |  | 566 | 30 | listing of 'internal' |  |  | 0.478 |
| walker |  | 574 | 8 | listing of 'internal/session' |  |  | 0.479 |
| walker |  | 586 | 12 | listing of 'internal/agent' |  |  | 0.481 |
| walker |  | 599 | 13 | listing of 'internal/tokens' |  |  | 0.488 |
| walker |  | 615 | 16 | go decl names surface in internal/tokens/counter.go |  |  | 0.488 |
| walker |  | 615 | 0 | go decl at internal/tokens/counter.go:21 |  |  | 0.488 |
| walker |  | 632 | 17 | listing of 'internal/auth' |  |  | 0.497 |
| ns | 641 |  | 76 | sdk/examples, examples/{hooks,scripts}, contribute/conf, hooks testdata listings | 2.5 |  | 0.466 |
| walker |  | 668 | 36 | listing of 'cmd' |  |  | 0.560 |
| walker |  | 682 | 14 | README.md section #5 |  |  | 0.560 |
| walker |  | 695 | 13 | README.md section #4 |  |  | 0.560 |
| walker |  | 709 | 14 | README.md section #12 |  |  | 0.560 |
| walker |  | 723 | 14 | README.md section #11 |  |  | 0.560 |
| walker |  | 751 | 28 | go decl names surface in internal/tokens/init.go |  |  | 0.560 |
| walker |  | 751 | 0 | go decl at internal/tokens/init.go:21 |  |  | 0.560 |
| walker |  | 751 | 0 | go decl at internal/tokens/init.go:50 |  |  | 0.560 |
| walker |  | 766 | 15 | README.md section #6 |  |  | 0.560 |
| walker |  | 796 | 30 | go decl names surface in internal/auth/browser.go |  |  | 0.560 |
| walker |  | 796 | 0 | go decl at internal/auth/browser.go:14 |  |  | 0.560 |
| walker |  | 796 | 0 | go decl at internal/auth/browser.go:34 |  |  | 0.560 |
| walker |  | 803 | 7 | go package + imports in internal/tokens/counter.go |  |  | 0.533 |
| ns | 803 |  | 162 | README Overview: host/client/server framing | 3.1 |  | 0.533 |
| walker |  | 810 | 7 | go package + imports in internal/tokens/init.go |  |  | 0.533 |
| walker |  | 826 | 16 | README.md section #13 |  |  | 0.533 |
| walker |  | 905 | 79 | go package + imports in main.go |  |  | 0.534 |
| ns | 922 |  | 119 | README Overview: supported models bullet list | 3.2 |  | 0.520 |
| walker |  | 970 | 65 | go decl names surface in sdk/types.go |  |  | 0.520 |
| walker |  | 970 | 0 | go decl at sdk/types.go:10 |  |  | 0.520 |
| walker |  | 970 | 0 | go decl at sdk/types.go:14 |  |  | 0.520 |
| walker |  | 970 | 0 | go decl at sdk/types.go:18 |  |  | 0.520 |
| walker |  | 970 | 0 | go decl at sdk/types.go:24 |  |  | 0.520 |
| walker |  | 981 | 11 | go decl body at sdk/types.go:18 |  |  | 0.520 |
| walker |  | 993 | 12 | go decl body at sdk/types.go:24 |  |  | 0.520 |
| walker |  | 1029 | 36 | go decl names surface in internal/agent/streaming.go |  |  | 0.520 |
| walker |  | 1029 | 0 | go decl at internal/agent/streaming.go:19 |  |  | 0.520 |
| walker |  | 1039 | 10 | contribute/contribute.md section #2 |  |  | 0.520 |
| walker |  | 1059 | 20 | contribute/contribute.md section #0 |  |  | 0.520 |
| walker |  | 1072 | 13 | README.md section #20 |  |  | 0.520 |
| walker |  | 1085 | 13 | README.md section #21 |  |  | 0.520 |
| walker |  | 1098 | 13 | README.md section #22 |  |  | 0.520 |
| walker |  | 1111 | 13 | README.md section #23 |  |  | 0.520 |
| walker |  | 1120 | 9 | go package + imports in internal/tokens/anthropic.go |  |  | 0.520 |
| ns | 1171 |  | 249 | go.mod direct deps (part 1: content-processing libs + eino model layer) | 3.3 |  | 0.500 |
| walker |  | 1191 | 71 | go decl names surface in cmd/hooks.go |  |  | 0.500 |
| walker |  | 1191 | 0 | go decl at cmd/hooks.go:176 |  |  | 0.500 |
| walker |  | 1226 | 35 | listing of 'internal/models' |  |  | 0.536 |
| walker |  | 1231 | 5 | listing of 'internal/models/anthropic' |  |  | 0.543 |
| walker |  | 1236 | 5 | listing of 'internal/models/gemini' |  |  | 0.550 |
| walker |  | 1241 | 5 | listing of 'internal/models/openai' |  |  | 0.557 |
| walker |  | 1261 | 20 | README.md section #9 |  |  | 0.557 |
| walker |  | 1279 | 18 | README.md section #10 |  |  | 0.557 |
| walker |  | 1293 | 14 | README.md section #15 |  |  | 0.557 |
| walker |  | 1307 | 14 | README.md section #18 |  |  | 0.557 |
| walker |  | 1321 | 14 | README.md section #24 |  |  | 0.557 |
| ns | 1354 |  | 183 | go.mod direct deps (part 2: schema/MCP/CLI-framework libs + remainder) | 3.4 | 3.3 | 0.539 |
| walker |  | 1360 | 39 | listing of 'internal/hooks' |  |  | 0.569 |
| walker |  | 1370 | 10 | listing of 'internal/hooks/testdata' |  |  | 0.572 |
| walker |  | 1379 | 9 | go package + imports in internal/hooks/events.go |  |  | 0.572 |
| walker |  | 1419 | 40 | listing of 'internal/builtin' |  |  | 0.626 |
| walker |  | 1430 | 11 | go package + imports in internal/models/models_data.go |  |  | 0.626 |
| walker |  | 1463 | 33 | go decl doc at sdk/types.go:10 |  |  | 0.626 |
| walker |  | 1497 | 34 | go decl doc at sdk/types.go:14 |  |  | 0.626 |
| walker |  | 1513 | 16 | README.md section #16 |  |  | 0.626 |
| walker |  | 1527 | 14 | README.md section #17 |  |  | 0.626 |
| walker |  | 1572 | 45 | listing of 'internal/config' |  |  | 0.698 |
| ns | 1595 |  | 241 | main.go (full) | 3.5 |  | 0.660 |
| walker |  | 1692 | 120 | headings outline in sdk/README.md |  |  | 0.660 |
| walker |  | 1705 | 13 | sdk/README.md section #17 |  |  | 0.660 |
| walker |  | 1738 | 33 | sdk/README.md section #1 |  |  | 0.660 |
| walker |  | 1755 | 17 | README.md section #27 |  |  | 0.660 |
| walker |  | 1771 | 16 | README.md section #26 |  |  | 0.660 |
| walker |  | 1788 | 17 | README.md section #25 |  |  | 0.660 |
| walker |  | 1820 | 32 | go decl doc at sdk/types.go:18 |  |  | 0.660 |
| walker |  | 1877 | 57 | go decl names surface in internal/config/merger.go |  |  | 0.660 |
| walker |  | 1877 | 0 | go decl at internal/config/merger.go:13 |  |  | 0.660 |
| walker |  | 1877 | 0 | go decl at internal/config/merger.go:28 |  |  | 0.660 |
| walker |  | 1877 | 0 | go decl at internal/config/merger.go:48 |  |  | 0.660 |
| walker |  | 1936 | 59 | go decl names surface in internal/models/models_data.go |  |  | 0.660 |
| walker |  | 1936 | 0 | go decl at internal/models/models_data.go:41 |  |  | 0.660 |
| ns | 1954 |  | 359 | cmd/root.go package-level flag vars | 4.1 |  | 0.598 |
| walker |  | 1956 | 20 | go decl at internal/models/models_data.go:26 |  |  | 0.598 |
| walker |  | 1972 | 16 | go decl doc at internal/models/models_data.go:26 |  |  | 0.598 |
| walker |  | 1989 | 17 | go decl doc at internal/models/models_data.go:41 |  |  | 0.598 |
| ns | 1998 |  | 44 | cmd/root.go rootCmd identity (Use/Short) | 4.2 | 4.1 | 0.592 |
| walker |  | 2033 | 44 | go decl at internal/models/models_data.go:18 |  |  | 0.592 |
| walker |  | 2047 | 14 | go decl doc at internal/models/models_data.go:18 |  |  | 0.592 |
| walker |  | 2081 | 34 | go decl doc at sdk/types.go:24 |  |  | 0.592 |
| walker |  | 2096 | 15 | go decl body at internal/tokens/init.go:21 |  |  | 0.592 |
| ns | 2107 |  | 109 | cmd/root.go: hidden Ollama-only flags | 4.3 | 4.1 | 0.587 |
| walker |  | 2111 | 15 | go decl body at internal/tokens/init.go:50 |  |  | 0.587 |
| walker |  | 2161 | 50 | go decl at internal/models/models_data.go:32 |  |  | 0.587 |
| walker |  | 2173 | 12 | go decl doc at internal/models/models_data.go:32 |  |  | 0.587 |
| walker |  | 2221 | 48 | README.md section #31 |  |  | 0.587 |
| walker |  | 2261 | 40 | README.md section #38 |  |  | 0.587 |
| walker |  | 2281 | 20 | README.md section #19 |  |  | 0.587 |
| walker |  | 2302 | 21 | README.md section #28 |  |  | 0.587 |
| walker |  | 2376 | 74 | go decl names surface in internal/hooks/schemas.go |  |  | 0.587 |
| walker |  | 2400 | 24 | go decl at internal/hooks/schemas.go:42 |  |  | 0.587 |
| walker |  | 2444 | 44 | go decl at internal/hooks/schemas.go:23 |  |  | 0.587 |
| ns | 2592 |  | 485 | cmd/root.go InitConfig(): config file search-path precedence | 4.4 |  | 0.543 |
| ns | 2747 |  | 155 | cmd/root.go InitConfig(): hooks bootstrap | 4.5 | 4.4 | 0.529 |
| walker |  | 2958 | 514 | go module file go.mod |  |  | 0.588 |
| walker |  | 3054 | 96 | sdk/README.md section #16 |  |  | 0.588 |
| walker |  | 3130 | 76 | go decl names surface in internal/models/generate_models.go |  |  | 0.588 |
| walker |  | 3130 | 0 | go decl at internal/models/generate_models.go:156 |  |  | 0.588 |
| walker |  | 3187 | 57 | go decl at internal/models/generate_models.go:50 |  |  | 0.588 |
| walker |  | 3267 | 80 | go decl names surface in internal/agent/factory.go |  |  | 0.588 |
| walker |  | 3267 | 0 | go decl at internal/agent/factory.go:15 |  |  | 0.588 |
| walker |  | 3267 | 0 | go decl at internal/agent/factory.go:43 |  |  | 0.588 |
| walker |  | 3267 | 0 | go decl at internal/agent/factory.go:76 |  |  | 0.588 |
| walker |  | 3328 | 61 | go decl at internal/hooks/schemas.go:32 |  |  | 0.588 |
| ns | 3338 |  | 591 | README: Flags + Authentication Subcommands + Model Generation Parameters tables | 4.6 |  | 0.559 |
| walker |  | 3416 | 88 | go decl names surface in internal/hooks/config.go |  |  | 0.559 |
| walker |  | 3416 | 0 | go decl at internal/hooks/config.go:41 |  |  | 0.559 |
| walker |  | 3416 | 0 | go decl at internal/hooks/config.go:97 |  |  | 0.559 |
| walker |  | 3416 | 0 | go decl at internal/hooks/config.go:113 |  |  | 0.559 |
| ns | 3438 |  | 100 | cmd/root.go runNormalMode(): flag-combination validation | 4.7 |  | 0.551 |
| walker |  | 3443 | 27 | go decl at internal/hooks/config.go:14 |  |  | 0.551 |
| walker |  | 3498 | 55 | go decl at internal/hooks/config.go:30 |  |  | 0.551 |
| walker |  | 3527 | 29 | go decl doc at internal/hooks/config.go:14 |  |  | 0.551 |
| walker |  | 3687 | 160 | go decl names surface in cmd/auth.go |  |  | 0.551 |
| walker |  | 3687 | 0 | go decl at cmd/auth.go:91 |  |  | 0.551 |
| walker |  | 3687 | 0 | go decl at cmd/auth.go:97 |  |  | 0.551 |
| walker |  | 3687 | 0 | go decl at cmd/auth.go:108 |  |  | 0.551 |
| walker |  | 3687 | 0 | go decl at cmd/auth.go:119 |  |  | 0.551 |
| walker |  | 3687 | 0 | go decl at cmd/auth.go:165 |  |  | 0.551 |
| walker |  | 3687 | 0 | go decl at cmd/auth.go:239 |  |  | 0.551 |
| ns | 3743 |  | 305 | internal/config/config.go: MCPServerConfig struct fields | 5.1 |  | 0.534 |
| walker |  | 3751 | 64 | go decl at internal/models/models_data.go:7 |  |  | 0.534 |
| walker |  | 3763 | 12 | go decl doc at internal/models/models_data.go:7 |  |  | 0.534 |
| walker |  | 3797 | 34 | go decl doc at internal/models/generate_models.go:50 |  |  | 0.534 |
| walker |  | 3862 | 65 | go decl at internal/hooks/config.go:21 |  |  | 0.534 |
| walker |  | 3989 | 127 | go decl body at main.go:14 |  |  | 0.579 |
| walker |  | 4003 | 14 | go decl doc at internal/hooks/config.go:113 |  |  | 0.579 |
| walker |  | 4089 | 86 | listing of 'internal/ui' |  |  | 0.621 |
| walker |  | 4094 | 5 | listing of 'internal/ui/progress' |  |  | 0.626 |
| walker |  | 4115 | 21 | go decl names surface in internal/ui/callbacks.go |  |  | 0.626 |
| walker |  | 4115 | 0 | go decl at internal/ui/callbacks.go:17 |  |  | 0.626 |
| walker |  | 4124 | 9 | go package + imports in internal/ui/commands.go |  |  | 0.626 |
| walker |  | 4182 | 58 | go decl names surface in internal/ui/commands.go |  |  | 0.626 |
| walker |  | 4182 | 0 | go decl at internal/ui/commands.go:65 |  |  | 0.626 |
| walker |  | 4182 | 0 | go decl at internal/ui/commands.go:83 |  |  | 0.626 |
| walker |  | 4233 | 51 | go decl at internal/ui/commands.go:6 |  |  | 0.626 |
| ns | 4291 |  | 548 | internal/config/config.go: Config struct (top-level app settings) | 5.2 |  | 0.598 |
| walker |  | 4300 | 67 | go decl names surface in internal/ui/factory.go |  |  | 0.598 |
| walker |  | 4300 | 0 | go decl at internal/ui/factory.go:33 |  |  | 0.598 |
| walker |  | 4300 | 0 | go decl at internal/ui/factory.go:45 |  |  | 0.598 |
| walker |  | 4353 | 53 | go decl at internal/ui/factory.go:13 |  |  | 0.598 |
| walker |  | 4427 | 74 | go decl names surface in internal/ui/fuzzy.go |  |  | 0.598 |
| walker |  | 4427 | 0 | go decl at internal/ui/fuzzy.go:18 |  |  | 0.598 |
| walker |  | 4427 | 0 | go decl at internal/ui/fuzzy.go:60 |  |  | 0.598 |
| walker |  | 4427 | 0 | go decl at internal/ui/fuzzy.go:117 |  |  | 0.598 |
| walker |  | 4450 | 23 | go decl at internal/ui/fuzzy.go:10 |  |  | 0.598 |
| walker |  | 4529 | 79 | go decl names surface in internal/ui/debug_logger.go |  |  | 0.598 |
| walker |  | 4529 | 0 | go decl at internal/ui/debug_logger.go:20 |  |  | 0.598 |
| walker |  | 4529 | 0 | go decl at internal/ui/debug_logger.go:28 |  |  | 0.598 |
| walker |  | 4529 | 0 | go decl at internal/ui/debug_logger.go:84 |  |  | 0.598 |
| walker |  | 4543 | 14 | go decl at internal/ui/debug_logger.go:13 |  |  | 0.598 |
| walker |  | 4558 | 15 | go decl body at internal/ui/debug_logger.go:20 |  |  | 0.598 |
| walker |  | 4572 | 14 | go decl body at internal/ui/debug_logger.go:84 |  |  | 0.598 |
| walker |  | 4605 | 33 | go decl doc at internal/ui/factory.go:13 |  |  | 0.598 |
| walker |  | 4643 | 38 | go decl doc at internal/agent/factory.go:15 |  |  | 0.598 |
| walker |  | 4656 | 13 | sdk/README.md section #15 |  |  | 0.598 |
| ns | 4669 |  | 378 | internal/config/config.go: Validate() | 5.3 | 5.1 | 0.575 |
| walker |  | 4760 | 104 | go decl names surface in internal/ui/tool_approval_input.go |  |  | 0.575 |
| walker |  | 4760 | 0 | go decl at internal/ui/tool_approval_input.go:22 |  |  | 0.575 |
| walker |  | 4760 | 0 | go decl at internal/ui/tool_approval_input.go:48 |  |  | 0.575 |
| walker |  | 4760 | 0 | go decl at internal/ui/tool_approval_input.go:52 |  |  | 0.575 |
| walker |  | 4760 | 0 | go decl at internal/ui/tool_approval_input.go:83 |  |  | 0.575 |
| walker |  | 4769 | 9 | go decl body at internal/ui/tool_approval_input.go:48 |  |  | 0.575 |
| walker |  | 4784 | 15 | go decl doc at internal/ui/fuzzy.go:117 |  |  | 0.575 |
| walker |  | 4842 | 58 | README.md section #36 |  |  | 0.575 |
| walker |  | 4918 | 76 | go decl at internal/ui/tool_approval_input.go:12 |  |  | 0.575 |
| walker |  | 4934 | 16 | go decl doc at internal/ui/fuzzy.go:60 |  |  | 0.575 |
| walker |  | 5046 | 112 | go decl names surface in internal/hooks/events.go |  |  | 0.575 |
| walker |  | 5046 | 0 | go decl at internal/hooks/events.go:5 |  |  | 0.575 |
| walker |  | 5046 | 0 | go decl at internal/hooks/events.go:23 |  |  | 0.575 |
| walker |  | 5046 | 0 | go decl at internal/hooks/events.go:34 |  |  | 0.575 |
| walker |  | 5063 | 17 | go decl body at internal/hooks/events.go:34 |  |  | 0.575 |
| walker |  | 5102 | 39 | go decl doc at internal/hooks/events.go:5 |  |  | 0.576 |
| walker |  | 5146 | 44 | go decl doc at internal/ui/fuzzy.go:10 |  |  | 0.576 |
| ns | 5178 |  | 509 | README: Builtin Servers config shape + available builtin names | 5.4 | 5.1 | 0.548 |
| walker |  | 5359 | 213 | go decl names surface in sdk/mcphost.go |  |  | 0.548 |
| walker |  | 5359 | 0 | go decl at sdk/mcphost.go:40 |  |  | 0.548 |
| walker |  | 5359 | 0 | go decl at sdk/mcphost.go:130 |  |  | 0.548 |
| walker |  | 5359 | 0 | go decl at sdk/mcphost.go:201 |  |  | 0.548 |
| walker |  | 5359 | 0 | go decl at sdk/mcphost.go:207 |  |  | 0.548 |
| walker |  | 5359 | 0 | go decl at sdk/mcphost.go:218 |  |  | 0.548 |
| walker |  | 5359 | 0 | go decl at sdk/mcphost.go:224 |  |  | 0.548 |
| walker |  | 5359 | 0 | go decl at sdk/mcphost.go:230 |  |  | 0.548 |
| walker |  | 5359 | 0 | go decl at sdk/mcphost.go:237 |  |  | 0.548 |
| walker |  | 5368 | 9 | go decl body at sdk/mcphost.go:201 |  |  | 0.548 |
| walker |  | 5377 | 9 | go decl body at sdk/mcphost.go:230 |  |  | 0.548 |
| walker |  | 5386 | 9 | go decl body at sdk/mcphost.go:237 |  |  | 0.548 |
| walker |  | 5421 | 35 | go decl at sdk/mcphost.go:19 |  |  | 0.548 |
| walker |  | 5433 | 12 | go decl body at sdk/mcphost.go:224 |  |  | 0.548 |
| walker |  | 5449 | 16 | go decl body at sdk/mcphost.go:218 |  |  | 0.548 |
| walker |  | 5478 | 29 | go decl doc at sdk/mcphost.go:201 |  |  | 0.548 |
| walker |  | 5507 | 29 | go decl doc at sdk/mcphost.go:224 |  |  | 0.548 |
| ns | 5516 |  | 338 | README: Transport Types + one Legacy STDIO example | 5.5 | 5.3 | 0.530 |
| walker |  | 5544 | 37 | go decl doc at sdk/mcphost.go:218 |  |  | 0.530 |
| walker |  | 5583 | 39 | go decl doc at sdk/mcphost.go:207 |  |  | 0.530 |
| walker |  | 5651 | 68 | go decl at sdk/mcphost.go:163 |  |  | 0.530 |
| walker |  | 5703 | 52 | go decl doc at sdk/mcphost.go:19 |  |  | 0.530 |
| walker |  | 5751 | 48 | go decl doc at sdk/mcphost.go:237 |  |  | 0.530 |
| walker |  | 5800 | 49 | go decl doc at sdk/mcphost.go:230 |  |  | 0.530 |
| ns | 5857 |  | 341 | internal/config/substitution.go: env-var and script-arg substitution patterns | 5.6 |  | 0.516 |
| walker |  | 5907 | 107 | go decl at sdk/mcphost.go:28 |  |  | 0.516 |
| walker |  | 5954 | 47 | go decl doc at sdk/mcphost.go:28 |  |  | 0.516 |
| walker |  | 6011 | 57 | go decl doc at sdk/mcphost.go:40 |  |  | 0.516 |
| walker |  | 6067 | 56 | go decl doc at sdk/mcphost.go:163 |  |  | 0.516 |
| walker |  | 6125 | 58 | go decl doc at sdk/mcphost.go:130 |  |  | 0.516 |
| walker |  | 6142 | 17 | go decl doc at internal/ui/factory.go:33 |  |  | 0.516 |
| ns | 6157 |  | 300 | README: Environment Variable Substitution syntax | 5.7 | 5.6 | 0.504 |
| walker |  | 6220 | 78 | go decl at internal/ui/factory.go:22 |  |  | 0.504 |
| walker |  | 6263 | 43 | go decl doc at internal/ui/factory.go:22 |  |  | 0.504 |
| walker |  | 6291 | 28 | go package + imports in internal/ui/fuzzy.go |  |  | 0.504 |
| walker |  | 6337 | 46 | go decl doc at internal/ui/commands.go:6 |  |  | 0.504 |
| walker |  | 6389 | 52 | go package + imports in sdk/types.go |  |  | 0.504 |
| walker |  | 6455 | 66 | README.md section #39 |  |  | 0.504 |
| walker |  | 6474 | 19 | go decl doc at internal/hooks/config.go:97 |  |  | 0.504 |
| walker |  | 6490 | 16 | sdk/README.md section #12 |  |  | 0.504 |
| walker |  | 6506 | 16 | sdk/README.md section #11 |  |  | 0.504 |
| walker |  | 6522 | 16 | sdk/README.md section #10 |  |  | 0.504 |
| walker |  | 6538 | 16 | sdk/README.md section #14 |  |  | 0.504 |
| ns | 6550 |  | 393 | internal/config/merger.go: MergeConfigs + LoadAndValidateConfig | 5.8 |  | 0.487 |
| walker |  | 6554 | 16 | sdk/README.md section #13 |  |  | 0.487 |
| walker |  | 6585 | 31 | go package + imports in internal/hooks/schemas.go |  |  | 0.487 |
| walker |  | 6611 | 26 | listing of 'internal/tools' |  |  | 0.504 |
| ns | 6682 |  | 132 | contribute/conf/demo.yml (legacy sample config) | 5.9 |  | 0.496 |
| walker |  | 6700 | 89 | go decl at internal/hooks/schemas.go:62 |  |  | 0.496 |
| walker |  | 6838 | 138 | go decl names surface in internal/ui/styles.go |  |  | 0.496 |
| walker |  | 6838 | 0 | go decl at internal/ui/styles.go:14 |  |  | 0.496 |
| walker |  | 6838 | 0 | go decl at internal/ui/styles.go:21 |  |  | 0.496 |
| walker |  | 6838 | 0 | go decl at internal/ui/styles.go:28 |  |  | 0.496 |
| walker |  | 6838 | 0 | go decl at internal/ui/styles.go:37 |  |  | 0.496 |
| walker |  | 6838 | 0 | go decl at internal/ui/styles.go:359 |  |  | 0.496 |
| walker |  | 6849 | 11 | go decl body at internal/ui/styles.go:21 |  |  | 0.496 |
| walker |  | 6858 | 9 | go decl doc at internal/ui/styles.go:14 |  |  | 0.496 |
| walker |  | 6871 | 13 | go decl doc at internal/ui/styles.go:359 |  |  | 0.496 |
| walker |  | 6889 | 18 | go decl doc at internal/ui/styles.go:37 |  |  | 0.496 |
| walker |  | 6930 | 41 | go decl doc at internal/hooks/events.go:23 |  |  | 0.496 |
| walker |  | 6983 | 53 | go decl doc at internal/hooks/schemas.go:23 |  |  | 0.496 |
| walker |  | 7036 | 53 | go decl doc at internal/hooks/schemas.go:42 |  |  | 0.496 |
| ns | 7048 |  | 366 | internal/agent/agent.go: Agent + AgentConfig structs | 6.1 |  | 0.483 |
| ns | 7166 |  | 118 | Agent loop + tool-manager entry-point locations | 6.2 | 6.1 | 0.481 |
| walker |  | 7178 | 142 | go decl names surface in internal/builtin/bash.go |  |  | 0.481 |
| walker |  | 7178 | 0 | go decl at internal/builtin/bash.go:44 |  |  | 0.481 |
| walker |  | 7178 | 0 | go decl at internal/builtin/bash.go:71 |  |  | 0.481 |
| walker |  | 7187 | 9 | go decl at internal/builtin/bash.go:14 |  |  | 0.481 |
| walker |  | 7203 | 16 | go decl doc at internal/builtin/bash.go:71 |  |  | 0.481 |
| walker |  | 7306 | 103 | README.md section #29 |  |  | 0.481 |
| walker |  | 7336 | 30 | go decl body at internal/tokens/counter.go:21 |  |  | 0.481 |
| ns | 7451 |  | 285 | internal/tools/mcp.go: tool name prefixing (serverName__toolName) | 6.3 |  | 0.471 |
| walker |  | 7483 | 147 | go decl names surface in internal/hooks/validator.go |  |  | 0.471 |
| walker |  | 7483 | 0 | go decl at internal/hooks/validator.go:17 |  |  | 0.471 |
| walker |  | 7483 | 0 | go decl at internal/hooks/validator.go:44 |  |  | 0.471 |
| walker |  | 7483 | 0 | go decl at internal/hooks/validator.go:75 |  |  | 0.471 |
| walker |  | 7483 | 0 | go decl at internal/hooks/validator.go:110 |  |  | 0.471 |
| walker |  | 7494 | 11 | go decl at internal/hooks/validator.go:10 |  |  | 0.471 |
| walker |  | 7508 | 14 | go decl doc at internal/hooks/validator.go:17 |  |  | 0.471 |
| walker |  | 7522 | 14 | go decl doc at internal/hooks/validator.go:110 |  |  | 0.471 |
| walker |  | 7538 | 16 | go decl doc at internal/hooks/validator.go:44 |  |  | 0.471 |
| walker |  | 7551 | 13 | go decl doc at internal/hooks/validator.go:10 |  |  | 0.471 |
| walker |  | 7598 | 47 | go decl doc at internal/config/merger.go:28 |  |  | 0.472 |
| walker |  | 7645 | 47 | go decl doc at internal/ui/styles.go:21 |  |  | 0.472 |
| walker |  | 7701 | 56 | go decl doc at internal/hooks/config.go:30 |  |  | 0.472 |
| walker |  | 7732 | 31 | go decl body at internal/auth/browser.go:34 |  |  | 0.472 |
| ns | 7800 |  | 349 | internal/tools/connection_pool.go: pool config + defaults | 6.4 |  | 0.463 |
| walker |  | 8012 | 280 | go decl names surface in cmd/script.go |  |  | 0.463 |
| walker |  | 8012 | 0 | go decl at cmd/script.go:78 |  |  | 0.463 |
| walker |  | 8012 | 0 | go decl at cmd/script.go:84 |  |  | 0.463 |
| walker |  | 8012 | 0 | go decl at cmd/script.go:148 |  |  | 0.463 |
| walker |  | 8012 | 0 | go decl at cmd/script.go:208 |  |  | 0.463 |
| walker |  | 8012 | 0 | go decl at cmd/script.go:255 |  |  | 0.463 |
| walker |  | 8012 | 0 | go decl at cmd/script.go:279 |  |  | 0.463 |
| walker |  | 8012 | 0 | go decl at cmd/script.go:288 |  |  | 0.463 |
| walker |  | 8012 | 0 | go decl at cmd/script.go:431 |  |  | 0.463 |
| walker |  | 8012 | 0 | go decl at cmd/script.go:442 |  |  | 0.463 |
| walker |  | 8012 | 0 | go decl at cmd/script.go:480 |  |  | 0.463 |
| walker |  | 8012 | 0 | go decl at cmd/script.go:499 |  |  | 0.463 |
| walker |  | 8012 | 0 | go decl at cmd/script.go:511 |  |  | 0.463 |
| walker |  | 8077 | 65 | go decl at cmd/script.go:423 |  |  | 0.463 |
| walker |  | 8088 | 11 | go decl body at cmd/script.go:78 |  |  | 0.463 |
| walker |  | 8104 | 16 | go decl doc at cmd/script.go:148 |  |  | 0.463 |
| ns | 8112 |  | 312 | Likely-dead duplicate: MCPToolManager.createMCPClient vs MCPConnectionPool.createMCPClient | 6.5 | 6.4 | 0.454 |
| walker |  | 8120 | 16 | go decl doc at cmd/script.go:279 |  |  | 0.454 |
| walker |  | 8138 | 18 | go decl doc at cmd/script.go:511 |  |  | 0.454 |
| walker |  | 8157 | 19 | go decl doc at cmd/script.go:288 |  |  | 0.454 |
| walker |  | 8177 | 20 | go decl doc at cmd/script.go:255 |  |  | 0.454 |
| walker |  | 8230 | 53 | go decl doc at cmd/script.go:423 |  |  | 0.454 |
| ns | 8253 |  | 141 | internal/builtin/registry.go: the 5 registered builtin server names | 7.1 |  | 0.449 |
| walker |  | 8261 | 31 | go decl doc at cmd/script.go:480 |  |  | 0.449 |
| ns | 8291 |  | 38 | Two different builtin tools both named "fetch" | 7.2 | 7.1 | 0.448 |
| walker |  | 8294 | 33 | go decl doc at cmd/script.go:431 |  |  | 0.448 |
| walker |  | 8330 | 36 | go decl doc at cmd/script.go:442 |  |  | 0.448 |
| walker |  | 8348 | 18 | sdk/README.md section #7 |  |  | 0.448 |
| walker |  | 8366 | 18 | sdk/README.md section #8 |  |  | 0.448 |
| walker |  | 8426 | 60 | go decl doc at internal/hooks/config.go:21 |  |  | 0.448 |
| ns | 8457 |  | 166 | internal/builtin/bash.go: banned command prefixes | 7.3 |  | 0.441 |
| ns | 8560 |  | 103 | internal/builtin/todo.go + http.go: remaining tool-name locations | 7.4 | 7.2 | 0.439 |
| walker |  | 8588 | 162 | go decl names surface in internal/ui/spinner.go |  |  | 0.439 |
| walker |  | 8588 | 0 | go decl at internal/ui/spinner.go:31 |  |  | 0.439 |
| walker |  | 8588 | 0 | go decl at internal/ui/spinner.go:35 |  |  | 0.439 |
| walker |  | 8588 | 0 | go decl at internal/ui/spinner.go:52 |  |  | 0.439 |
| walker |  | 8588 | 0 | go decl at internal/ui/spinner.go:75 |  |  | 0.439 |
| walker |  | 8588 | 0 | go decl at internal/ui/spinner.go:80 |  |  | 0.439 |
| walker |  | 8588 | 0 | go decl at internal/ui/spinner.go:107 |  |  | 0.439 |
| walker |  | 8588 | 0 | go decl at internal/ui/spinner.go:133 |  |  | 0.439 |
| walker |  | 8588 | 0 | go decl at internal/ui/spinner.go:149 |  |  | 0.439 |
| walker |  | 8640 | 52 | go decl at internal/ui/spinner.go:16 |  |  | 0.439 |
| walker |  | 8656 | 16 | go decl body at internal/ui/spinner.go:149 |  |  | 0.439 |
| ns | 8664 |  | 104 | internal/models/providers.go: CreateProvider's actual provider roster | 8.1 |  | 0.436 |
| walker |  | 8665 | 9 | go decl body at internal/ui/spinner.go:31 |  |  | 0.436 |
| walker |  | 8682 | 17 | go decl doc at internal/ui/spinner.go:75 |  |  | 0.436 |
| walker |  | 8712 | 30 | go decl at internal/ui/spinner.go:25 |  |  | 0.436 |
| walker |  | 8725 | 13 | go decl doc at internal/ui/spinner.go:25 |  |  | 0.436 |
| walker |  | 8763 | 38 | go decl doc at internal/ui/spinner.go:149 |  |  | 0.436 |
| walker |  | 8815 | 52 | go decl doc at internal/ui/spinner.go:16 |  |  | 0.436 |
| walker |  | 8876 | 61 | go decl doc at internal/hooks/schemas.go:32 |  |  | 0.436 |
| ns | 8889 |  | 225 | internal/hooks/events.go: the actual 4 hook events (vs README's 6) | 9.1 |  | 0.438 |
| walker |  | 8928 | 52 | go decl doc at internal/config/merger.go:13 |  |  | 0.438 |
| walker |  | 8952 | 24 | go decl doc at internal/config/merger.go:48 |  |  | 0.438 |
| walker |  | 9005 | 53 | go decl doc at internal/ui/spinner.go:107 |  |  | 0.438 |
| walker |  | 9068 | 63 | go decl doc at internal/ui/debug_logger.go:13 |  |  | 0.438 |
| ns | 9094 |  | 205 | internal/hooks/executor.go: blocking via exit code 2 | 9.2 | 9.1 | 0.432 |
| walker |  | 9112 | 44 | go decl doc at cmd/script.go:499 |  |  | 0.432 |
| walker |  | 9166 | 54 | go decl doc at internal/ui/styles.go:28 |  |  | 0.432 |
| ns | 9189 |  | 95 | internal/ui/commands.go: the real slash-command registry (vs README's /history claim) | 10.1 |  | 0.430 |
| ns | 9268 |  | 79 | internal/auth/credentials.go: API key precedence order | 11.1 |  | 0.429 |
| walker |  | 9273 | 107 | go decl at internal/hooks/schemas.go:50 |  |  | 0.429 |
| walker |  | 9328 | 55 | go decl doc at internal/hooks/schemas.go:50 |  |  | 0.429 |
| walker |  | 9383 | 55 | go decl doc at internal/ui/commands.go:65 |  |  | 0.429 |
| walker |  | 9438 | 55 | go decl doc at internal/ui/debug_logger.go:20 |  |  | 0.429 |
| walker |  | 9614 | 176 | go decl names surface in internal/ui/slash_command_input.go |  |  | 0.429 |
| walker |  | 9614 | 0 | go decl at internal/ui/slash_command_input.go:34 |  |  | 0.429 |
| walker |  | 9614 | 0 | go decl at internal/ui/slash_command_input.go:63 |  |  | 0.429 |
| walker |  | 9614 | 0 | go decl at internal/ui/slash_command_input.go:70 |  |  | 0.429 |
| walker |  | 9614 | 0 | go decl at internal/ui/slash_command_input.go:181 |  |  | 0.429 |
| walker |  | 9614 | 0 | go decl at internal/ui/slash_command_input.go:248 |  |  | 0.429 |
| walker |  | 9614 | 0 | go decl at internal/ui/slash_command_input.go:338 |  |  | 0.429 |
| walker |  | 9614 | 0 | go decl at internal/ui/slash_command_input.go:344 |  |  | 0.429 |
| walker |  | 9614 | 0 | go decl at internal/ui/slash_command_input.go:351 |  |  | 0.429 |
| walker |  | 9622 | 8 | go decl body at internal/ui/slash_command_input.go:338 |  |  | 0.429 |
| walker |  | 9631 | 9 | go decl body at internal/ui/slash_command_input.go:63 |  |  | 0.429 |
| walker |  | 9641 | 10 | go decl body at internal/ui/slash_command_input.go:351 |  |  | 0.429 |
| ns | 9646 |  | 378 | internal/session/session.go: Session/Message struct fields | 12.1 |  | 0.421 |
| walker |  | 9654 | 13 | go decl body at internal/ui/slash_command_input.go:344 |  |  | 0.421 |
| walker |  | 9666 | 12 | go decl doc at internal/ui/slash_command_input.go:248 |  |  | 0.421 |
| walker |  | 9699 | 33 | go decl doc at internal/ui/slash_command_input.go:338 |  |  | 0.421 |
| ns | 9700 |  | 54 | sdk/mcphost.go: New + Prompt entry points | 12.2 | 12.1 | 0.422 |
| walker |  | 9733 | 34 | go decl doc at internal/ui/slash_command_input.go:63 |  |  | 0.422 |
| walker |  | 9771 | 38 | go decl doc at internal/ui/slash_command_input.go:344 |  |  | 0.422 |
| walker |  | 9820 | 49 | go decl doc at internal/ui/slash_command_input.go:70 |  |  | 0.422 |
| walker |  | 9876 | 56 | go decl doc at internal/ui/spinner.go:80 |  |  | 0.422 |
| walker |  | 9917 | 41 | go package + imports in internal/ui/block_renderer.go |  |  | 0.422 |
| walker |  | 9937 | 20 | sdk/README.md section #9 |  |  | 0.422 |
| ns | 9967 |  | 267 | cmd/script.go: substitution order (env vars, then script args) | 13.1 |  | 0.417 |
| walker |  | 9994 | 57 | go decl doc at internal/ui/fuzzy.go:18 |  |  | 0.417 |

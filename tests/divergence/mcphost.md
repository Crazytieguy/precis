Score(3000)=0.648 I=0.837 C=0.502 ns_rows≤3K=18/51 grid(1000/1442/2080/3000/4327/6240/9000)=0.781/0.757/0.716/0.648/0.574/0.490/0.476

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 40 | 40 | listing of '.' |  |  | 0.000 |
| walker |  | 55 | 15 | listing of 'contribute' |  |  | 0.000 |
| walker |  | 63 | 8 | listing of 'contribute/conf' |  |  | 0.000 |
| ns | 63 |  | 63 | README title and one-line identity | 1.1 |  | 0.000 |
| walker |  | 66 | 3 | listing of '.github' |  |  | 0.000 |
| walker |  | 74 | 8 | listing of '.github/workflows' |  |  | 0.000 |
| walker |  | 98 | 24 | listing of 'sdk' |  |  | 0.000 |
| ns | 103 |  | 40 | Complete repository root listing | 1.2 |  | 0.633 |
| ns | 133 |  | 30 | Complete internal/ package listing | 1.3 |  | 0.472 |
| ns | 207 |  | 74 | Complete cmd/ and sdk/ listings | 1.4 |  | 0.373 |
| walker |  | 252 | 154 | YAML config at .github/workflows/ci.yml |  |  | 0.373 |
| ns | 260 |  | 53 | Module path, Go version and toolchain | 1.5 |  | 0.349 |
| walker |  | 279 | 27 | go decl names surface in main.go |  |  | 0.349 |
| walker |  | 279 | 0 | go decl at main.go:14 |  |  | 0.349 |
| walker |  | 309 | 30 | listing of 'internal' |  |  | 0.504 |
| walker |  | 317 | 8 | listing of 'internal/session' |  |  | 0.504 |
| walker |  | 329 | 12 | listing of 'internal/agent' |  |  | 0.504 |
| walker |  | 342 | 13 | listing of 'internal/tokens' |  |  | 0.505 |
| walker |  | 378 | 36 | listing of 'cmd' |  |  | 0.604 |
| walker |  | 395 | 17 | listing of 'internal/auth' |  |  | 0.605 |
| walker |  | 401 | 6 | listing of 'examples' |  |  | 0.606 |
| ns | 427 |  | 167 | All README section headings (locations only) | 1.6 |  | 0.519 |
| walker |  | 464 | 63 | README headline in README.md |  |  | 0.721 |
| walker |  | 488 | 24 | README.md section #0 |  |  | 0.721 |
| ns | 534 |  | 107 | README feature list, first half | 1.7 | 1.6 | 0.670 |
| walker |  | 541 | 53 | go module identity in go.mod |  |  | 0.724 |
| walker |  | 576 | 35 | listing of 'internal/models' |  |  | 0.728 |
| walker |  | 581 | 5 | listing of 'internal/models/anthropic' |  |  | 0.728 |
| walker |  | 586 | 5 | listing of 'internal/models/gemini' |  |  | 0.729 |
| walker |  | 591 | 5 | listing of 'internal/models/openai' |  |  | 0.730 |
| walker |  | 630 | 39 | listing of 'internal/hooks' |  |  | 0.733 |
| walker |  | 640 | 10 | listing of 'internal/hooks/testdata' |  |  | 0.734 |
| ns | 648 |  | 114 | README feature list, second half | 1.8 |  | 0.688 |
| walker |  | 680 | 40 | README headline in sdk/README.md |  |  | 0.688 |
| walker |  | 720 | 40 | listing of 'internal/builtin' |  |  | 0.694 |
| walker |  | 765 | 45 | listing of 'internal/config' |  |  | 0.704 |
| ns | 781 |  | 133 | Complete listings for the config / agent / tools / models packages | 1.9 |  | 0.709 |
| walker |  | 798 | 33 | headings outline in contribute/contribute.md |  |  | 0.709 |
| walker |  | 884 | 86 | listing of 'internal/ui' |  |  | 0.725 |
| walker |  | 889 | 5 | listing of 'internal/ui/progress' |  |  | 0.726 |
| walker |  | 905 | 16 | go decl names surface in internal/tokens/counter.go |  |  | 0.726 |
| walker |  | 905 | 0 | go decl at internal/tokens/counter.go:21 |  |  | 0.726 |
| walker |  | 926 | 21 | go decl names surface in internal/ui/callbacks.go |  |  | 0.726 |
| walker |  | 926 | 0 | go decl at internal/ui/callbacks.go:17 |  |  | 0.726 |
| walker |  | 940 | 14 | README.md section #2 |  |  | 0.726 |
| walker |  | 952 | 12 | README.md section #3 |  |  | 0.726 |
| walker |  | 966 | 14 | README.md section #5 |  |  | 0.726 |
| walker |  | 979 | 13 | README.md section #4 |  |  | 0.726 |
| walker |  | 993 | 14 | README.md section #12 |  |  | 0.726 |
| ns | 999 |  | 218 | Complete listings for the builtin / hooks / session / auth / tokens / ui packages | 1.10 |  | 0.781 |
| walker |  | 1007 | 14 | README.md section #11 |  |  | 0.781 |
| walker |  | 1022 | 15 | README.md section #6 |  |  | 0.781 |
| walker |  | 1038 | 16 | README.md section #13 |  |  | 0.781 |
| walker |  | 1066 | 28 | go decl names surface in internal/tokens/init.go |  |  | 0.781 |
| walker |  | 1066 | 0 | go decl at internal/tokens/init.go:21 |  |  | 0.781 |
| walker |  | 1066 | 0 | go decl at internal/tokens/init.go:50 |  |  | 0.781 |
| ns | 1089 |  | 90 | Complete listings for examples/, contribute/ and .github/ | 1.11 |  | 0.747 |
| walker |  | 1145 | 79 | go package + imports in main.go |  |  | 0.747 |
| walker |  | 1175 | 30 | go decl names surface in internal/auth/browser.go |  |  | 0.747 |
| walker |  | 1175 | 0 | go decl at internal/auth/browser.go:14 |  |  | 0.747 |
| walker |  | 1175 | 0 | go decl at internal/auth/browser.go:34 |  |  | 0.747 |
| walker |  | 1182 | 7 | go package + imports in internal/tokens/counter.go |  |  | 0.747 |
| walker |  | 1189 | 7 | go package + imports in internal/tokens/init.go |  |  | 0.747 |
| walker |  | 1215 | 26 | listing of 'internal/tools' |  |  | 0.784 |
| walker |  | 1228 | 13 | README.md section #20 |  |  | 0.784 |
| walker |  | 1241 | 13 | README.md section #21 |  |  | 0.786 |
| ns | 1243 |  | 154 | main.go entry point | 2.1 |  | 0.752 |
| walker |  | 1254 | 13 | README.md section #22 |  |  | 0.753 |
| walker |  | 1267 | 13 | README.md section #23 |  |  | 0.754 |
| walker |  | 1332 | 65 | go decl names surface in sdk/types.go |  |  | 0.754 |
| walker |  | 1332 | 0 | go decl at sdk/types.go:10 |  |  | 0.754 |
| walker |  | 1332 | 0 | go decl at sdk/types.go:14 |  |  | 0.754 |
| walker |  | 1332 | 0 | go decl at sdk/types.go:18 |  |  | 0.754 |
| walker |  | 1332 | 0 | go decl at sdk/types.go:24 |  |  | 0.754 |
| walker |  | 1343 | 11 | go decl body at sdk/types.go:18 |  |  | 0.754 |
| walker |  | 1355 | 12 | go decl body at sdk/types.go:24 |  |  | 0.754 |
| walker |  | 1365 | 10 | contribute/contribute.md section #2 |  |  | 0.754 |
| walker |  | 1385 | 20 | README.md section #9 |  |  | 0.754 |
| walker |  | 1403 | 18 | README.md section #10 |  |  | 0.754 |
| walker |  | 1423 | 20 | contribute/contribute.md section #0 |  |  | 0.754 |
| walker |  | 1437 | 14 | README.md section #18 |  |  | 0.757 |
| walker |  | 1451 | 14 | README.md section #24 |  |  | 0.759 |
| walker |  | 1487 | 36 | go decl names surface in internal/agent/streaming.go |  |  | 0.759 |
| walker |  | 1487 | 0 | go decl at internal/agent/streaming.go:19 |  |  | 0.759 |
| ns | 1504 |  | 261 | Complete cobra command tree: script, auth (login/logout/status), hooks (list/validate/init) | 2.2 |  | 0.720 |
| walker |  | 1558 | 71 | go decl names surface in cmd/hooks.go |  |  | 0.720 |
| walker |  | 1558 | 0 | go decl at cmd/hooks.go:176 |  |  | 0.720 |
| walker |  | 1574 | 16 | README.md section #17 |  |  | 0.724 |
| walker |  | 1590 | 16 | README.md section #16 |  |  | 0.729 |
| walker |  | 1614 | 24 | README.md section #1 |  |  | 0.729 |
| walker |  | 1647 | 33 | go decl doc at sdk/types.go:10 |  |  | 0.729 |
| walker |  | 1664 | 17 | README.md section #27 |  |  | 0.732 |
| walker |  | 1680 | 16 | README.md section #26 |  |  | 0.737 |
| walker |  | 1697 | 17 | README.md section #25 |  |  | 0.744 |
| walker |  | 1731 | 34 | go decl doc at sdk/types.go:14 |  |  | 0.744 |
| ns | 1754 |  | 250 | Persistent flag registration, part 1: config, system-prompt, model, debug, prompt, quiet | 2.3 |  | 0.716 |
| walker |  | 1851 | 120 | headings outline in sdk/README.md |  |  | 0.716 |
| walker |  | 1864 | 13 | sdk/README.md section #17 |  |  | 0.716 |
| walker |  | 1897 | 33 | sdk/README.md section #1 |  |  | 0.716 |
| walker |  | 1971 | 74 | go decl names surface in cmd/root.go |  |  | 0.716 |
| walker |  | 1971 | 0 | go decl at cmd/root.go:131 |  |  | 0.716 |
| walker |  | 1971 | 0 | go decl at cmd/root.go:143 |  |  | 0.716 |
| walker |  | 1971 | 0 | go decl at cmd/root.go:225 |  |  | 0.716 |
| walker |  | 1986 | 15 | go decl at cmd/root.go:67 |  |  | 0.716 |
| walker |  | 2004 | 18 | go decl body at cmd/root.go:131 |  |  | 0.716 |
| walker |  | 2036 | 32 | go decl doc at sdk/types.go:18 |  |  | 0.716 |
| walker |  | 2053 | 17 | go decl doc at cmd/root.go:67 |  |  | 0.716 |
| ns | 2098 |  | 344 | Persistent flag registration, part 2: no-exit, max-steps, stream, compact, no-hooks, approve-tool-run, session flags | 2.4 |  | 0.680 |
| walker |  | 2101 | 48 | README.md section #31 |  |  | 0.681 |
| walker |  | 2135 | 34 | go decl doc at sdk/types.go:24 |  |  | 0.681 |
| walker |  | 2175 | 40 | README.md section #57 |  |  | 0.682 |
| walker |  | 2195 | 20 | README.md section #19 |  |  | 0.688 |
| ns | 2224 |  | 126 | Provider and TLS flag registration | 2.5 |  | 0.682 |
| walker |  | 2252 | 57 | go decl names surface in internal/config/merger.go |  |  | 0.682 |
| walker |  | 2252 | 0 | go decl at internal/config/merger.go:13 |  |  | 0.682 |
| walker |  | 2252 | 0 | go decl at internal/config/merger.go:28 |  |  | 0.682 |
| walker |  | 2252 | 0 | go decl at internal/config/merger.go:48 |  |  | 0.682 |
| walker |  | 2267 | 15 | go decl body at internal/tokens/init.go:21 |  |  | 0.682 |
| walker |  | 2282 | 15 | go decl body at internal/tokens/init.go:50 |  |  | 0.682 |
| walker |  | 2342 | 60 | go decl names surface in internal/ui/commands.go |  |  | 0.682 |
| walker |  | 2342 | 0 | go decl at internal/ui/commands.go:65 |  |  | 0.682 |
| walker |  | 2342 | 0 | go decl at internal/ui/commands.go:83 |  |  | 0.682 |
| walker |  | 2349 | 7 | go package + imports in internal/ui/commands.go |  |  | 0.682 |
| walker |  | 2370 | 21 | README.md section #28 |  |  | 0.689 |
| walker |  | 2431 | 61 | go decl names surface in internal/models/models_data.go |  |  | 0.689 |
| walker |  | 2431 | 0 | go decl at internal/models/models_data.go:41 |  |  | 0.689 |
| walker |  | 2451 | 20 | go decl at internal/models/models_data.go:26 |  |  | 0.689 |
| walker |  | 2460 | 9 | go package + imports in internal/models/models_data.go |  |  | 0.689 |
| walker |  | 2476 | 16 | go decl doc at internal/models/models_data.go:26 |  |  | 0.689 |
| walker |  | 2493 | 17 | go decl doc at internal/models/models_data.go:41 |  |  | 0.689 |
| ns | 2502 |  | 278 | Generation-parameter and Ollama flag registration, with the hidden flag | 2.6 |  | 0.672 |
| walker |  | 2537 | 44 | go decl at internal/models/models_data.go:18 |  |  | 0.672 |
| walker |  | 2551 | 14 | go decl doc at internal/models/models_data.go:18 |  |  | 0.672 |
| ns | 2749 |  | 247 | MCPServerConfig: complete field set including the legacy block | 3.1 |  | 0.648 |
| walker |  | 3065 | 514 | go module file go.mod |  |  | 0.648 |
| ns | 3069 |  | 320 | Config struct: application-level keys | 3.2 |  | 0.628 |
| walker |  | 3115 | 50 | go decl at internal/models/models_data.go:32 |  |  | 0.628 |
| walker |  | 3127 | 12 | go decl doc at internal/models/models_data.go:32 |  |  | 0.628 |
| walker |  | 3178 | 51 | go decl at internal/ui/commands.go:6 |  |  | 0.628 |
| walker |  | 3245 | 67 | go decl names surface in internal/ui/factory.go |  |  | 0.628 |
| walker |  | 3245 | 0 | go decl at internal/ui/factory.go:33 |  |  | 0.628 |
| walker |  | 3245 | 0 | go decl at internal/ui/factory.go:45 |  |  | 0.628 |
| ns | 3250 |  | 181 | Config struct: generation-parameter and TLS keys | 3.3 |  | 0.615 |
| walker |  | 3298 | 53 | go decl at internal/ui/factory.go:13 |  |  | 0.615 |
| ns | 3514 |  | 264 | GetTransportType: type-to-transport mapping and legacy inference | 3.4 |  | 0.584 |
| walker |  | 3556 | 258 | sdk/README.md section #2 |  |  | 0.584 |
| walker |  | 3630 | 74 | go decl names surface in internal/hooks/schemas.go |  |  | 0.584 |
| walker |  | 3654 | 24 | go decl at internal/hooks/schemas.go:42 |  |  | 0.584 |
| walker |  | 3698 | 44 | go decl at internal/hooks/schemas.go:23 |  |  | 0.584 |
| walker |  | 3772 | 74 | go decl names surface in internal/ui/fuzzy.go |  |  | 0.584 |
| walker |  | 3772 | 0 | go decl at internal/ui/fuzzy.go:18 |  |  | 0.584 |
| walker |  | 3772 | 0 | go decl at internal/ui/fuzzy.go:60 |  |  | 0.584 |
| walker |  | 3772 | 0 | go decl at internal/ui/fuzzy.go:117 |  |  | 0.584 |
| walker |  | 3795 | 23 | go decl at internal/ui/fuzzy.go:10 |  |  | 0.584 |
| walker |  | 3922 | 127 | go decl body at main.go:14 |  |  | 0.617 |
| ns | 3938 |  | 424 | Config.Validate: required fields per transport and filter exclusivity | 3.5 |  | 0.586 |
| walker |  | 4001 | 79 | go decl names surface in internal/ui/debug_logger.go |  |  | 0.586 |
| walker |  | 4001 | 0 | go decl at internal/ui/debug_logger.go:20 |  |  | 0.586 |
| walker |  | 4001 | 0 | go decl at internal/ui/debug_logger.go:28 |  |  | 0.586 |
| walker |  | 4001 | 0 | go decl at internal/ui/debug_logger.go:84 |  |  | 0.586 |
| walker |  | 4015 | 14 | go decl at internal/ui/debug_logger.go:13 |  |  | 0.586 |
| walker |  | 4030 | 15 | go decl body at internal/ui/debug_logger.go:20 |  |  | 0.586 |
| walker |  | 4044 | 14 | go decl body at internal/ui/debug_logger.go:84 |  |  | 0.586 |
| walker |  | 4070 | 26 | README.md section #15 |  |  | 0.595 |
| ns | 4141 |  | 203 | Substitution engine: the two regexes plus every symbol in substitution.go | 3.6 |  | 0.584 |
| walker |  | 4150 | 80 | go decl names surface in internal/agent/factory.go |  |  | 0.584 |
| walker |  | 4150 | 0 | go decl at internal/agent/factory.go:15 |  |  | 0.584 |
| walker |  | 4150 | 0 | go decl at internal/agent/factory.go:43 |  |  | 0.584 |
| walker |  | 4150 | 0 | go decl at internal/agent/factory.go:76 |  |  | 0.584 |
| walker |  | 4211 | 61 | go decl names surface in internal/builtin/http.go |  |  | 0.584 |
| walker |  | 4211 | 0 | go decl at internal/builtin/http.go:36 |  |  | 0.584 |
| walker |  | 4211 | 0 | go decl at internal/builtin/http.go:116 |  |  | 0.584 |
| ns | 4295 |  | 154 | Remaining top-level symbols of internal/config/config.go and all of merger.go (locations) | 3.7 |  | 0.574 |
| walker |  | 4371 | 160 | go decl names surface in cmd/auth.go |  |  | 0.574 |
| walker |  | 4371 | 0 | go decl at cmd/auth.go:91 |  |  | 0.574 |
| walker |  | 4371 | 0 | go decl at cmd/auth.go:97 |  |  | 0.574 |
| walker |  | 4371 | 0 | go decl at cmd/auth.go:108 |  |  | 0.574 |
| walker |  | 4371 | 0 | go decl at cmd/auth.go:119 |  |  | 0.574 |
| walker |  | 4371 | 0 | go decl at cmd/auth.go:165 |  |  | 0.574 |
| walker |  | 4371 | 0 | go decl at cmd/auth.go:239 |  |  | 0.574 |
| walker |  | 4433 | 62 | go decl names surface in internal/models/providers.go |  |  | 0.574 |
| walker |  | 4433 | 0 | go decl at internal/models/providers.go:153 |  |  | 0.574 |
| walker |  | 4494 | 61 | go decl at internal/hooks/schemas.go:32 |  |  | 0.574 |
| ns | 4541 |  | 246 | Agent struct and its seven callback handler types | 4.1 |  | 0.560 |
| walker |  | 4580 | 86 | go decl names surface in internal/models/generate_models.go |  |  | 0.560 |
| walker |  | 4580 | 0 | go decl at internal/models/generate_models.go:156 |  |  | 0.560 |
| walker |  | 4637 | 57 | go decl at internal/models/generate_models.go:50 |  |  | 0.560 |
| walker |  | 4690 | 53 | go decl doc at cmd/root.go:131 |  |  | 0.560 |
| walker |  | 4778 | 88 | go decl names surface in internal/hooks/config.go |  |  | 0.560 |
| walker |  | 4778 | 0 | go decl at internal/hooks/config.go:41 |  |  | 0.560 |
| walker |  | 4778 | 0 | go decl at internal/hooks/config.go:97 |  |  | 0.560 |
| walker |  | 4778 | 0 | go decl at internal/hooks/config.go:113 |  |  | 0.560 |
| walker |  | 4805 | 27 | go decl at internal/hooks/config.go:14 |  |  | 0.560 |
| ns | 4815 |  | 274 | Every top-level symbol of internal/agent/agent.go (locations) | 4.2 |  | 0.546 |
| walker |  | 4860 | 55 | go decl at internal/hooks/config.go:30 |  |  | 0.546 |
| walker |  | 4889 | 29 | go decl doc at internal/hooks/config.go:14 |  |  | 0.546 |
| walker |  | 4922 | 33 | go decl doc at internal/ui/factory.go:13 |  |  | 0.546 |
| walker |  | 4986 | 64 | go decl at internal/models/models_data.go:7 |  |  | 0.546 |
| walker |  | 4998 | 12 | go decl doc at internal/models/models_data.go:7 |  |  | 0.546 |
| ns | 5007 |  | 192 | The tool-calling loop: step bound, tool-call branch and the approval gate | 4.3 | 4.2 | 0.537 |
| walker |  | 5056 | 58 | README.md section #55 |  |  | 0.539 |
| walker |  | 5090 | 34 | go decl doc at internal/models/generate_models.go:50 |  |  | 0.539 |
| walker |  | 5155 | 65 | go decl at internal/hooks/config.go:21 |  |  | 0.539 |
| ns | 5186 |  | 179 | agent factory: AgentCreationOptions fields and both functions | 4.4 |  | 0.532 |
| walker |  | 5224 | 69 | go decl names surface in internal/ui/enhanced_styles.go |  |  | 0.532 |
| walker |  | 5224 | 0 | go decl at internal/ui/enhanced_styles.go:16 |  |  | 0.532 |
| walker |  | 5224 | 0 | go decl at internal/ui/enhanced_styles.go:22 |  |  | 0.532 |
| walker |  | 5224 | 0 | go decl at internal/ui/enhanced_styles.go:51 |  |  | 0.532 |
| walker |  | 5224 | 0 | go decl at internal/ui/enhanced_styles.go:123 |  |  | 0.532 |
| walker |  | 5232 | 8 | go decl body at internal/ui/enhanced_styles.go:16 |  |  | 0.532 |
| walker |  | 5241 | 9 | go decl body at internal/ui/enhanced_styles.go:22 |  |  | 0.532 |
| walker |  | 5254 | 13 | sdk/README.md section #15 |  |  | 0.532 |
| walker |  | 5268 | 14 | go decl doc at internal/builtin/http.go:116 |  |  | 0.532 |
| walker |  | 5282 | 14 | go decl doc at internal/hooks/config.go:113 |  |  | 0.532 |
| walker |  | 5356 | 74 | go decl names surface in internal/agent/agent.go |  |  | 0.534 |
| walker |  | 5356 | 0 | go decl at internal/agent/agent.go:39 |  |  | 0.534 |
| walker |  | 5356 | 0 | go decl at internal/agent/agent.go:43 |  |  | 0.534 |
| walker |  | 5356 | 0 | go decl at internal/agent/agent.go:47 |  |  | 0.534 |
| walker |  | 5394 | 38 | go decl doc at internal/agent/agent.go:47 |  |  | 0.534 |
| walker |  | 5432 | 38 | go decl doc at internal/agent/factory.go:15 |  |  | 0.534 |
| walker |  | 5536 | 104 | go decl names surface in internal/ui/tool_approval_input.go |  |  | 0.534 |
| walker |  | 5536 | 0 | go decl at internal/ui/tool_approval_input.go:22 |  |  | 0.534 |
| walker |  | 5536 | 0 | go decl at internal/ui/tool_approval_input.go:48 |  |  | 0.534 |
| walker |  | 5536 | 0 | go decl at internal/ui/tool_approval_input.go:52 |  |  | 0.534 |
| walker |  | 5536 | 0 | go decl at internal/ui/tool_approval_input.go:83 |  |  | 0.534 |
| walker |  | 5545 | 9 | go decl body at internal/ui/tool_approval_input.go:48 |  |  | 0.534 |
| ns | 5549 |  | 363 | Every top-level symbol of internal/tools/mcp.go (locations) | 4.5 |  | 0.519 |
| walker |  | 5578 | 33 | go decl doc at internal/ui/enhanced_styles.go:16 |  |  | 0.519 |
| walker |  | 5593 | 15 | go decl doc at internal/ui/fuzzy.go:117 |  |  | 0.519 |
| walker |  | 5671 | 78 | go decl names surface in internal/auth/credentials.go |  |  | 0.519 |
| walker |  | 5671 | 0 | go decl at internal/auth/credentials.go:62 |  |  | 0.519 |
| walker |  | 5671 | 0 | go decl at internal/auth/credentials.go:247 |  |  | 0.519 |
| walker |  | 5685 | 14 | go decl at internal/auth/credentials.go:54 |  |  | 0.519 |
| walker |  | 5708 | 23 | go decl at internal/auth/credentials.go:14 |  |  | 0.519 |
| walker |  | 5742 | 34 | go decl doc at internal/auth/credentials.go:14 |  |  | 0.519 |
| ns | 5765 |  | 216 | AgenticLoopConfig head and every mode-driving function in cmd/root.go (locations) | 4.6 |  | 0.508 |
| walker |  | 5808 | 66 | README.md section #58 |  |  | 0.510 |
| walker |  | 5849 | 41 | go decl doc at internal/agent/agent.go:43 |  |  | 0.510 |
| ns | 5907 |  | 142 | CreateProvider: the complete list of supported providers | 5.1 |  | 0.503 |
| walker |  | 6062 | 213 | go decl names surface in sdk/mcphost.go |  |  | 0.504 |
| walker |  | 6062 | 0 | go decl at sdk/mcphost.go:40 |  |  | 0.504 |
| walker |  | 6062 | 0 | go decl at sdk/mcphost.go:130 |  |  | 0.504 |
| walker |  | 6062 | 0 | go decl at sdk/mcphost.go:201 |  |  | 0.504 |
| walker |  | 6062 | 0 | go decl at sdk/mcphost.go:207 |  |  | 0.504 |
| walker |  | 6062 | 0 | go decl at sdk/mcphost.go:218 |  |  | 0.504 |
| walker |  | 6062 | 0 | go decl at sdk/mcphost.go:224 |  |  | 0.504 |
| walker |  | 6062 | 0 | go decl at sdk/mcphost.go:230 |  |  | 0.504 |
| walker |  | 6062 | 0 | go decl at sdk/mcphost.go:237 |  |  | 0.504 |
| walker |  | 6071 | 9 | go decl body at sdk/mcphost.go:201 |  |  | 0.504 |
| walker |  | 6080 | 9 | go decl body at sdk/mcphost.go:230 |  |  | 0.504 |
| walker |  | 6089 | 9 | go decl body at sdk/mcphost.go:237 |  |  | 0.504 |
| walker |  | 6124 | 35 | go decl at sdk/mcphost.go:19 |  |  | 0.504 |
| walker |  | 6136 | 12 | go decl body at sdk/mcphost.go:224 |  |  | 0.504 |
| walker |  | 6165 | 29 | go decl doc at sdk/mcphost.go:201 |  |  | 0.504 |
| walker |  | 6194 | 29 | go decl doc at sdk/mcphost.go:224 |  |  | 0.504 |
| walker |  | 6231 | 37 | go decl doc at sdk/mcphost.go:218 |  |  | 0.504 |
| ns | 6235 |  | 328 | Every top-level symbol of internal/models/providers.go (locations) | 5.2 | 5.1 | 0.490 |
| walker |  | 6270 | 39 | go decl doc at sdk/mcphost.go:207 |  |  | 0.490 |
| walker |  | 6338 | 68 | go decl at sdk/mcphost.go:163 |  |  | 0.490 |
| ns | 6385 |  | 150 | ModelsRegistry: model validation and suggestion API | 5.3 |  | 0.485 |
| walker |  | 6390 | 52 | go decl doc at sdk/mcphost.go:19 |  |  | 0.485 |
| walker |  | 6438 | 48 | go decl doc at sdk/mcphost.go:237 |  |  | 0.485 |
| ns | 6477 |  | 92 | The generated model catalogue: generator types and the DO-NOT-EDIT header | 5.4 |  | 0.484 |
| walker |  | 6487 | 49 | go decl doc at sdk/mcphost.go:230 |  |  | 0.484 |
| walker |  | 6594 | 107 | go decl at sdk/mcphost.go:28 |  |  | 0.485 |
| walker |  | 6641 | 47 | go decl doc at sdk/mcphost.go:28 |  |  | 0.485 |
| ns | 6653 |  | 176 | Builtin server registry: the complete set of in-process servers | 6.1 |  | 0.476 |
| walker |  | 6698 | 57 | go decl doc at sdk/mcphost.go:40 |  |  | 0.476 |
| walker |  | 6754 | 56 | go decl doc at sdk/mcphost.go:163 |  |  | 0.476 |
| walker |  | 6812 | 58 | go decl doc at sdk/mcphost.go:130 |  |  | 0.476 |
| ns | 6825 |  | 172 | Every top-level symbol of internal/builtin/registry.go (locations) | 6.2 | 6.1 | 0.471 |
| walker |  | 6847 | 35 | go decl doc at internal/ui/enhanced_styles.go:22 |  |  | 0.471 |
| walker |  | 6895 | 48 | go decl names surface in internal/models/gemini/gemini.go |  |  | 0.471 |
| walker |  | 6895 | 0 | go decl at internal/models/gemini/gemini.go:43 |  |  | 0.471 |
| walker |  | 6901 | 6 | listing of 'sdk/examples' |  |  | 0.480 |
| walker |  | 6977 | 76 | go decl at internal/ui/tool_approval_input.go:12 |  |  | 0.480 |
| walker |  | 6993 | 16 | go decl doc at internal/ui/fuzzy.go:60 |  |  | 0.480 |
| walker |  | 7035 | 42 | go decl doc at internal/agent/agent.go:39 |  |  | 0.480 |
| ns | 7042 |  | 217 | Bash builtin: output/timeout limits and the complete banned-command list | 6.3 |  | 0.468 |
| walker |  | 7149 | 114 | go decl names surface in internal/hooks/events.go |  |  | 0.469 |
| walker |  | 7149 | 0 | go decl at internal/hooks/events.go:5 |  |  | 0.469 |
| walker |  | 7149 | 0 | go decl at internal/hooks/events.go:23 |  |  | 0.469 |
| walker |  | 7149 | 0 | go decl at internal/hooks/events.go:34 |  |  | 0.469 |
| walker |  | 7156 | 7 | go package + imports in internal/hooks/events.go |  |  | 0.469 |
| walker |  | 7173 | 17 | go decl body at internal/hooks/events.go:34 |  |  | 0.469 |
| walker |  | 7212 | 39 | go decl doc at internal/hooks/events.go:5 |  |  | 0.469 |
| ns | 7240 |  | 198 | The http builtin: its four tools and every symbol in http.go (locations) | 6.4 |  | 0.462 |
| walker |  | 7264 | 52 | go package + imports in sdk/types.go |  |  | 0.462 |
| walker |  | 7308 | 44 | go decl doc at internal/ui/fuzzy.go:10 |  |  | 0.462 |
| walker |  | 7325 | 17 | go decl doc at internal/ui/factory.go:33 |  |  | 0.462 |
| ns | 7352 |  | 112 | HookEvent: the complete set of hook events | 7.1 |  | 0.468 |
| walker |  | 7403 | 78 | go decl at internal/ui/factory.go:22 |  |  | 0.468 |
| walker |  | 7446 | 43 | go decl doc at internal/ui/factory.go:22 |  |  | 0.469 |
| walker |  | 7491 | 45 | go decl doc at internal/auth/credentials.go:54 |  |  | 0.469 |
| walker |  | 7519 | 28 | go package + imports in internal/ui/fuzzy.go |  |  | 0.469 |
| walker |  | 7565 | 46 | go decl doc at internal/ui/commands.go:6 |  |  | 0.469 |
| ns | 7567 |  | 215 | Hook configuration schema: HookConfig, HookMatcher, HookEntry | 7.2 |  | 0.483 |
| walker |  | 7668 | 103 | README.md section #29 |  |  | 0.485 |
| walker |  | 7684 | 16 | sdk/README.md section #12 |  |  | 0.485 |
| walker |  | 7700 | 16 | sdk/README.md section #11 |  |  | 0.485 |
| walker |  | 7716 | 16 | sdk/README.md section #10 |  |  | 0.485 |
| walker |  | 7732 | 16 | sdk/README.md section #14 |  |  | 0.485 |
| walker |  | 7748 | 16 | sdk/README.md section #13 |  |  | 0.485 |
| walker |  | 7767 | 19 | go decl doc at internal/hooks/config.go:97 |  |  | 0.485 |
| walker |  | 7798 | 31 | go package + imports in internal/hooks/schemas.go |  |  | 0.485 |
| ns | 7839 |  | 272 | Hook wire protocol: CommonInput and HookOutput | 7.3 |  | 0.478 |
| walker |  | 7887 | 89 | go decl at internal/hooks/schemas.go:62 |  |  | 0.482 |
| walker |  | 8025 | 138 | go decl names surface in internal/ui/styles.go |  |  | 0.482 |
| walker |  | 8025 | 0 | go decl at internal/ui/styles.go:14 |  |  | 0.482 |
| walker |  | 8025 | 0 | go decl at internal/ui/styles.go:21 |  |  | 0.482 |
| walker |  | 8025 | 0 | go decl at internal/ui/styles.go:28 |  |  | 0.482 |
| walker |  | 8025 | 0 | go decl at internal/ui/styles.go:37 |  |  | 0.482 |
| walker |  | 8025 | 0 | go decl at internal/ui/styles.go:359 |  |  | 0.482 |
| ns | 8030 |  | 191 | Per-event hook input structs | 7.4 |  | 0.480 |
| walker |  | 8036 | 11 | go decl body at internal/ui/styles.go:21 |  |  | 0.480 |
| walker |  | 8045 | 9 | go decl doc at internal/ui/styles.go:14 |  |  | 0.480 |
| walker |  | 8058 | 13 | go decl doc at internal/ui/styles.go:359 |  |  | 0.480 |
| walker |  | 8076 | 18 | go decl doc at internal/ui/styles.go:37 |  |  | 0.480 |
| walker |  | 8136 | 60 | go decl names surface in internal/models/openai/openai.go |  |  | 0.480 |
| walker |  | 8136 | 0 | go decl at internal/models/openai/openai.go:53 |  |  | 0.480 |
| walker |  | 8170 | 34 | go decl at internal/models/openai/openai.go:31 |  |  | 0.480 |
| walker |  | 8207 | 37 | go decl at internal/models/openai/openai.go:21 |  |  | 0.480 |
| walker |  | 8248 | 41 | go decl doc at internal/hooks/events.go:23 |  |  | 0.480 |
| walker |  | 8301 | 53 | go decl doc at internal/hooks/schemas.go:23 |  |  | 0.480 |
| ns | 8336 |  | 306 | Hook executor and validator symbol rosters | 7.5 |  | 0.471 |
| walker |  | 8354 | 53 | go decl doc at internal/hooks/schemas.go:42 |  |  | 0.471 |
| ns | 8493 |  | 157 | README: hooks.yml file locations and the --no-hooks escape hatch | 7.6 | 1.6 | 0.468 |
| walker |  | 8496 | 142 | go decl names surface in internal/builtin/bash.go |  |  | 0.471 |
| walker |  | 8496 | 0 | go decl at internal/builtin/bash.go:44 |  |  | 0.471 |
| walker |  | 8496 | 0 | go decl at internal/builtin/bash.go:71 |  |  | 0.471 |
| walker |  | 8505 | 9 | go decl at internal/builtin/bash.go:14 |  |  | 0.471 |
| walker |  | 8521 | 16 | go decl doc at internal/builtin/bash.go:71 |  |  | 0.471 |
| ns | 8682 |  | 189 | Script mode: a worked frontmatter example and the variable rules | 8.1 |  | 0.466 |
| walker |  | 8801 | 280 | go decl names surface in cmd/script.go |  |  | 0.467 |
| walker |  | 8801 | 0 | go decl at cmd/script.go:78 |  |  | 0.467 |
| walker |  | 8801 | 0 | go decl at cmd/script.go:84 |  |  | 0.467 |
| walker |  | 8801 | 0 | go decl at cmd/script.go:148 |  |  | 0.467 |
| walker |  | 8801 | 0 | go decl at cmd/script.go:208 |  |  | 0.467 |
| walker |  | 8801 | 0 | go decl at cmd/script.go:255 |  |  | 0.467 |
| walker |  | 8801 | 0 | go decl at cmd/script.go:279 |  |  | 0.467 |
| walker |  | 8801 | 0 | go decl at cmd/script.go:288 |  |  | 0.467 |
| walker |  | 8801 | 0 | go decl at cmd/script.go:431 |  |  | 0.467 |
| walker |  | 8801 | 0 | go decl at cmd/script.go:442 |  |  | 0.467 |
| walker |  | 8801 | 0 | go decl at cmd/script.go:480 |  |  | 0.467 |
| walker |  | 8801 | 0 | go decl at cmd/script.go:499 |  |  | 0.467 |
| walker |  | 8801 | 0 | go decl at cmd/script.go:511 |  |  | 0.467 |
| ns | 8813 |  | 131 | Every top-level symbol of cmd/script.go (locations) | 8.2 |  | 0.476 |
| walker |  | 8866 | 65 | go decl at cmd/script.go:423 |  |  | 0.476 |
| walker |  | 8877 | 11 | go decl body at cmd/script.go:78 |  |  | 0.476 |
| walker |  | 8893 | 16 | go decl doc at cmd/script.go:148 |  |  | 0.476 |
| walker |  | 8909 | 16 | go decl doc at cmd/script.go:279 |  |  | 0.476 |
| walker |  | 8927 | 18 | go decl doc at cmd/script.go:511 |  |  | 0.476 |
| walker |  | 8946 | 19 | go decl doc at cmd/script.go:288 |  |  | 0.476 |
| walker |  | 8966 | 20 | go decl doc at cmd/script.go:255 |  |  | 0.476 |
| walker |  | 9019 | 53 | go decl doc at cmd/script.go:423 |  |  | 0.476 |
| walker |  | 9050 | 31 | go decl doc at cmd/script.go:480 |  |  | 0.476 |
| walker |  | 9083 | 33 | go decl doc at cmd/script.go:431 |  |  | 0.476 |
| ns | 9111 |  | 298 | Session file format: Session, Metadata, Message and ToolCall fields | 8.3 |  | 0.468 |
| walker |  | 9119 | 36 | go decl doc at cmd/script.go:442 |  |  | 0.468 |
| walker |  | 9137 | 18 | sdk/README.md section #7 |  |  | 0.468 |
| walker |  | 9155 | 18 | sdk/README.md section #8 |  |  | 0.468 |
| walker |  | 9185 | 30 | go decl body at internal/tokens/counter.go:21 |  |  | 0.468 |
| walker |  | 9332 | 147 | go decl names surface in internal/hooks/validator.go |  |  | 0.473 |
| walker |  | 9332 | 0 | go decl at internal/hooks/validator.go:17 |  |  | 0.473 |
| walker |  | 9332 | 0 | go decl at internal/hooks/validator.go:44 |  |  | 0.473 |
| walker |  | 9332 | 0 | go decl at internal/hooks/validator.go:75 |  |  | 0.473 |
| walker |  | 9332 | 0 | go decl at internal/hooks/validator.go:110 |  |  | 0.473 |
| walker |  | 9343 | 11 | go decl at internal/hooks/validator.go:10 |  |  | 0.474 |
| walker |  | 9357 | 14 | go decl doc at internal/hooks/validator.go:17 |  |  | 0.474 |
| walker |  | 9371 | 14 | go decl doc at internal/hooks/validator.go:110 |  |  | 0.474 |
| walker |  | 9387 | 16 | go decl doc at internal/hooks/validator.go:44 |  |  | 0.474 |
| walker |  | 9400 | 13 | go decl doc at internal/hooks/validator.go:10 |  |  | 0.474 |
| ns | 9405 |  | 294 | The public SDK surface: Options and every exported symbol | 8.4 |  | 0.487 |
| walker |  | 9447 | 47 | go decl doc at internal/config/merger.go:28 |  |  | 0.487 |
| walker |  | 9494 | 47 | go decl doc at internal/ui/styles.go:21 |  |  | 0.487 |
| walker |  | 9590 | 96 | sdk/README.md section #16 |  |  | 0.487 |
| ns | 9607 |  | 202 | The complete slash-command table: names and descriptions | 8.5 |  | 0.482 |
| walker |  | 9646 | 56 | go decl doc at internal/hooks/config.go:30 |  |  | 0.482 |
| walker |  | 9742 | 96 | go decl at internal/models/providers.go:124 |  |  | 0.482 |
| walker |  | 9786 | 44 | go decl doc at internal/models/providers.go:124 |  |  | 0.482 |
| walker |  | 9817 | 31 | go decl body at internal/auth/browser.go:34 |  |  | 0.482 |
| ns | 9828 |  | 221 | ui.SetupCLI: the AgentInterface contract and CLISetupOptions | 8.6 |  | 0.493 |
| walker |  | 9866 | 49 | go decl doc at internal/ui/enhanced_styles.go:123 |  |  | 0.493 |
| walker |  | 9910 | 44 | go decl doc at cmd/script.go:499 |  |  | 0.493 |
| walker |  | 9979 | 69 | go decl names surface in internal/ui/progress/ollama.go |  |  | 0.493 |
| walker |  | 9979 | 0 | go decl at internal/ui/progress/ollama.go:58 |  |  | 0.493 |
| walker |  | 9979 | 0 | go decl at internal/ui/progress/ollama.go:156 |  |  | 0.493 |
| ns | 9984 |  | 156 | Release packaging: the goreleaser build matrix | 9.1 |  | 0.487 |

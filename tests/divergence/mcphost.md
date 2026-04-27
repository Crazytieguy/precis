scores: Sim=0.385 Reached=6/40 Early=1 Late=3 Partial=5 Missing=29 Used=9994/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 10 | 4 | 3 | 3 | 0.59 |
| 2 | 10 | 0 | 0 | 10 | 0.00 |
| 3 | 6 | 0 | 0 | 6 | 0.01 |
| 4 | 8 | 1 | 2 | 5 | 0.28 |
| 5 | 6 | 1 | 0 | 5 | 0.17 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 135 | — | — | 0.50 | partial | README lede | README headline in README.md (t=99, 2 atoms) |
| 1.3 | 201 | 449 | +248 | 1.00 | late | internal/ + cmd/ subtree listings |  |
| 1.4 | 257 | — | — | 0.50 | partial | go.mod — module declaration | go module file go.mod (t=2207, 3 atoms) |
| 1.5 | 353 | — | — | 0.75 | partial | main.go — package + imports + version literal | go package + imports in main.go (t=2278, 8 atoms) |
| 1.6 | 498 | — | — | 0.14 | missing | main.go — version-flag short-circuit + fang.Execute | go decl names surface in main.go (t=145, 3 atoms) |
| 1.7 | 730 | — | — | 0.00 | missing | README — supported features list |  |
| 1.8 | 1004 | — | — | 0.00 | missing | README — host/client/server architecture model |  |
| 1.9 | 1339 | 2207 | +868 | 1.00 | late | go.mod — direct deps (CLI / config / MCP / LLM core) | go module file go.mod (t=2207, 15 atoms) |
| 1.10 | 1436 | 2207 | +771 | 1.00 | late | go.mod — direct deps (UI / Gemini / misc) | go module file go.mod (t=2207, 6 atoms) |
| 2.1 | 1737 | — | — | 0.00 | missing | MCPServerConfig — struct definition (the central config type) |  |
| 2.2 | 2004 | — | — | 0.00 | missing | MCPServerConfig.GetTransportType — type→transport mapping |  |
| 2.3 | 2486 | — | — | 0.00 | missing | rootCmd — Use / Short / Long with examples |  |
| 2.4 | 3017 | — | — | 0.00 | missing | Config — top-level config struct (fields) |  |
| 2.5 | 3154 | — | — | 0.00 | missing | Builtin server registry — names + factory dispatch |  |
| 2.6 | 3631 | — | — | 0.00 | missing | README — Builtin servers catalogue |  |
| 2.7 | 3928 | — | — | 0.00 | missing | rootCmd flags — enumerated (locations only) |  |
| 2.8 | 4183 | — | — | 0.03 | missing | Subcommand cobra Use/Short — auth / hooks / script (locations) | go decl names surface in cmd/hooks.go (t=772, 7 atoms) |
| 2.9 | 4519 | — | — | 0.00 | missing | Agent — type + GenerateWithLoopAndStreaming signature |  |
| 2.10 | 4895 | — | — | 0.00 | missing | MCPServerConfig.Validate — per-transport requirements |  |
| 3.1 | 5136 | — | — | 0.00 | missing | Agent loop — outer for + LLM call + tool-call branch arm |  |
| 3.2 | 5506 | — | — | 0.00 | missing | MCPToolManager — type |  |
| 3.3 | 5811 | — | — | 0.00 | missing | MCPToolManager.LoadTools + tool prefixing rule |  |
| 3.4 | 6037 | — | — | 0.00 | missing | Connection pool — type + DefaultConnectionPoolConfig |  |
| 3.5 | 6266 | — | — | 0.00 | missing | Connection pool — createMCPClient transport switch |  |
| 3.6 | 6549 | — | — | 0.06 | missing | Streaming aggregator — StreamWithCallback signature + provider note | go decl names surface in internal/agent/streaming.go (t=601, 2 atoms) |
| 4.1 | 7004 | — | — | 0.00 | missing | CreateProvider — provider switch |  |
| 4.2 | 7225 | — | — | 0.59 | partial | HookEvent constants + RequiresMatcher rule | go decl names surface in internal/hooks/events.go (t=4812, 12 atoms) |
| 4.3 | 7736 | — | — | 0.77 | partial | Hook input/output JSON schemas | go decl names surface in internal/hooks/schemas.go (t=2350, 12 atoms) |
| 4.4 | 8045 | 7252 | -793 | 0.86 | aligned | Hook config — HookConfig / HookMatcher / HookEntry | go decl names surface in internal/hooks/config.go (t=3040, 6 atoms) |
| 4.5 | 8222 | — | — | 0.00 | missing | Hook executor — ExecuteHooks signature |  |
| 4.6 | 8365 | — | — | 0.00 | missing | Auth — AnthropicCredentials Type field |  |
| 4.7 | 8574 | — | — | 0.00 | missing | OAuth client — Anthropic endpoints |  |
| 4.8 | 8752 | — | — | 0.00 | missing | Session.Message + Builtin tool inventory (locations) |  |
| 5.1 | 9076 | — | — | 0.00 | missing | Script mode — frontmatter format example |  |
| 5.2 | 9165 | — | — | 0.00 | missing | Env-var + script-args substitution patterns |  |
| 5.3 | 9466 | 5784 | -3682 | 1.00 | early | SDK — Options struct + entry signatures | go decl names surface in sdk/mcphost.go (t=5252, 19 atoms) |
| 5.4 | 9559 | — | — | 0.00 | missing | UI CLI — slash command names (locations) |  |
| 5.5 | 9719 | — | — | 0.00 | missing | InitConfig — config search order |  |
| 5.6 | 9983 | — | — | 0.00 | missing | Anthropic alias resolution map |  |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 4 | 296 | README.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 276 | 0.99 | 278 | 8109 | go decl names surface in cmd/script.go |
| 175 | 1.00 | 175 | 7427 | AGENTS.md section #0 |
| 160 | 1.00 | 160 | 8879 | go decl names surface in internal/ui/spinner.go |
| 150 | 0.95 | 158 | 3368 | go decl names surface in cmd/auth.go |
| 145 | 1.00 | 145 | 7122 | go decl names surface in internal/hooks/validator.go |
| 140 | 1.00 | 140 | 6905 | go decl names surface in internal/builtin/bash.go |
| 136 | 1.00 | 136 | 6665 | go decl names surface in internal/ui/styles.go |
| 116 | 1.00 | 116 | 1323 | headings outline in sdk/README.md |
| 102 | 1.00 | 102 | 4322 | go decl names surface in internal/ui/tool_approval_input.go |
| 98 | 1.00 | 98 | 6246 | README.md section #4 |
| 91 | 1.00 | 91 | 3497 | go decl names surface in internal/tools/buffered_logger.go |
| 86 | 1.00 | 86 | 3660 | listing of 'internal/ui' |
| 84 | 1.00 | 84 | 9487 | README.md section #12 |
| 84 | 0.15 | 557 | 2207 | go module file go.mod |
| 82 | 1.00 | 82 | 2796 | go decl names surface in internal/tools/debug_logger.go |
| 81 | 0.38 | 211 | 5252 | go decl names surface in sdk/mcphost.go |
| 80 | 1.00 | 80 | 5013 | go decl at internal/tools/debug_logger.go:7 |
| 79 | 1.00 | 79 | 9994 | go package + imports in cmd/auth.go |
| 78 | 1.00 | 78 | 4592 | go decl at internal/ui/factory.go:22 |
| 78 | 1.00 | 78 | 2680 | go decl names surface in internal/agent/factory.go |
| 77 | 1.00 | 77 | 4084 | go decl names surface in internal/ui/debug_logger.go |
| 76 | 1.00 | 76 | 4514 | go decl at internal/ui/tool_approval_input.go:12 |
| 74 | 1.00 | 74 | 2488 | go decl names surface in internal/models/generate_models.go |
| 72 | 1.00 | 72 | 3986 | go decl names surface in internal/ui/fuzzy.go |
| 70 | 1.00 | 70 | 9673 | go decl doc at internal/tools/buffered_logger.go:11 |
| 70 | 1.00 | 70 | 9743 | go decl doc at internal/tools/debug_logger.go:19 |
| 67 | 1.00 | 67 | 8638 | go decl doc at internal/tools/debug_logger.go:7 |
| 65 | 1.00 | 65 | 3863 | go decl names surface in internal/ui/factory.go |
| 64 | 1.00 | 64 | 2940 | go decl at internal/models/models_data.go:7 |
| 63 | 1.00 | 63 | 4700 | README.md section #14 |
| 63 | 1.00 | 63 | 8181 | go decl at cmd/script.go:423 |
| 63 | 1.00 | 63 | 7542 | go decl doc at internal/hooks/schemas.go:32 |
| 63 | 1.00 | 63 | 7605 | go decl doc at internal/ui/debug_logger.go:13 |
| 63 | 1.00 | 63 | 664 | go decl names surface in sdk/types.go |
| 61 | 0.88 | 69 | 772 | go decl names surface in cmd/hooks.go |
| 59 | 1.00 | 59 | 9802 | go decl doc at internal/agent/factory.go:43 |
| 59 | 1.00 | 59 | 9861 | go decl doc at internal/ui/fuzzy.go:18 |
| 59 | 1.00 | 59 | 5892 | go decl doc at sdk/mcphost.go:40 |
| 59 | 1.00 | 59 | 1477 | go decl names surface in internal/models/models_data.go |
| 58 | 1.00 | 58 | 9545 | go decl doc at internal/ui/commands.go:83 |
| 58 | 1.00 | 58 | 9603 | go decl doc at internal/ui/spinner.go:80 |
| 58 | 1.00 | 58 | 6006 | go decl doc at sdk/mcphost.go:130 |
| 58 | 1.00 | 58 | 3749 | go decl names surface in internal/ui/commands.go |
| 57 | 1.00 | 57 | 9361 | go decl doc at internal/hooks/schemas.go:50 |
| 57 | 1.00 | 57 | 8719 | go decl doc at internal/ui/debug_logger.go:20 |
| 56 | 1.00 | 56 | 5948 | go decl doc at sdk/mcphost.go:163 |
| 55 | 1.00 | 55 | 2543 | go decl at internal/models/generate_models.go:50 |
| 55 | 1.00 | 55 | 6301 | go decl doc at internal/hooks/schemas.go:42 |
| 55 | 1.00 | 55 | 8530 | go decl doc at internal/ui/commands.go:65 |
| 55 | 1.00 | 55 | 1418 | go decl names surface in internal/config/merger.go |
| 54 | 1.00 | 54 | 7737 | go decl doc at internal/tools/buffered_logger.go:20 |
| 54 | 1.00 | 54 | 9915 | go decl doc at internal/ui/debug_logger.go:84 |
| 54 | 1.00 | 54 | 7791 | go decl doc at internal/ui/styles.go:28 |
| 53 | 1.00 | 53 | 8284 | go decl doc at cmd/script.go:423 |
| 53 | 1.00 | 53 | 6088 | go decl doc at internal/hooks/schemas.go:23 |
| 53 | 1.00 | 53 | 9155 | go decl doc at internal/ui/spinner.go:107 |
| 53 | 0.62 | 86 | 3040 | go decl names surface in internal/hooks/config.go |
| 53 | 1.00 | 53 | 229 | headings outline in AGENTS.md |
| 52 | 1.00 | 52 | 7479 | go decl doc at internal/config/merger.go:13 |
| 52 | 1.00 | 52 | 9102 | go decl doc at internal/ui/spinner.go:16 |
| 52 | 1.00 | 52 | 5516 | go decl doc at sdk/mcphost.go:19 |
| 51 | 1.00 | 51 | 2876 | README.md section #11 |
| 51 | 1.00 | 51 | 3914 | go decl at internal/ui/factory.go:13 |
| 50 | 1.00 | 50 | 1636 | go decl at internal/models/models_data.go:32 |
| 50 | 1.00 | 50 | 8950 | go decl at internal/ui/spinner.go:16 |
| 50 | 1.00 | 50 | 8475 | go decl doc at internal/tools/debug_logger.go:43 |

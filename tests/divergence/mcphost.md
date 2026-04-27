scores: Sim=0.375 Reached=5/40 Early=0 Late=4 Partial=1 Missing=34 Used=9550/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 10 | 4 | 1 | 5 | 0.42 |
| 2 | 10 | 1 | 0 | 9 | 0.09 |
| 3 | 6 | 0 | 0 | 6 | 0.00 |
| 4 | 8 | 0 | 0 | 8 | 0.00 |
| 5 | 6 | 0 | 0 | 6 | 0.00 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 135 | — | — | 0.50 | partial | README lede | README headline in README.md (t=99, 2 atoms) |
| 1.3 | 201 | 413 | +212 | 1.00 | late | internal/ + cmd/ subtree listings |  |
| 1.4 | 257 | — | — | 0.00 | missing | go.mod — module declaration |  |
| 1.5 | 353 | — | — | 0.00 | missing | main.go — package + imports + version literal |  |
| 1.6 | 498 | — | — | 0.00 | missing | main.go — version-flag short-circuit + fang.Execute |  |
| 1.7 | 730 | 1848 | +1118 | 0.88 | late | README — supported features list | README.md section #3 (t=1848, 15 atoms) |
| 1.8 | 1004 | 2107 | +1103 | 0.83 | late | README — host/client/server architecture model | README.md section #2 (t=2107, 15 atoms) |
| 1.9 | 1339 | — | — | 0.00 | missing | go.mod — direct deps (CLI / config / MCP / LLM core) |  |
| 1.10 | 1436 | — | — | 0.00 | missing | go.mod — direct deps (UI / Gemini / misc) |  |
| 2.1 | 1737 | — | — | 0.00 | missing | MCPServerConfig — struct definition (the central config type) |  |
| 2.2 | 2004 | — | — | 0.00 | missing | MCPServerConfig.GetTransportType — type→transport mapping |  |
| 2.3 | 2486 | — | — | 0.00 | missing | rootCmd — Use / Short / Long with examples |  |
| 2.4 | 3017 | — | — | 0.00 | missing | Config — top-level config struct (fields) |  |
| 2.5 | 3154 | — | — | 0.00 | missing | Builtin server registry — names + factory dispatch |  |
| 2.6 | 3631 | 9550 | +5919 | 0.92 | late | README — Builtin servers catalogue | README.md section #8 (t=9550, 36 atoms) |
| 2.7 | 3928 | — | — | 0.00 | missing | rootCmd flags — enumerated (locations only) |  |
| 2.8 | 4183 | — | — | 0.00 | missing | Subcommand cobra Use/Short — auth / hooks / script (locations) |  |
| 2.9 | 4519 | — | — | 0.00 | missing | Agent — type + GenerateWithLoopAndStreaming signature |  |
| 2.10 | 4895 | — | — | 0.00 | missing | MCPServerConfig.Validate — per-transport requirements |  |
| 3.1 | 5136 | — | — | 0.00 | missing | Agent loop — outer for + LLM call + tool-call branch arm |  |
| 3.2 | 5506 | — | — | 0.00 | missing | MCPToolManager — type |  |
| 3.3 | 5811 | — | — | 0.00 | missing | MCPToolManager.LoadTools + tool prefixing rule |  |
| 3.4 | 6037 | — | — | 0.00 | missing | Connection pool — type + DefaultConnectionPoolConfig |  |
| 3.5 | 6266 | — | — | 0.00 | missing | Connection pool — createMCPClient transport switch |  |
| 3.6 | 6549 | — | — | 0.00 | missing | Streaming aggregator — StreamWithCallback signature + provider note |  |
| 4.1 | 7004 | — | — | 0.00 | missing | CreateProvider — provider switch |  |
| 4.2 | 7225 | — | — | 0.00 | missing | HookEvent constants + RequiresMatcher rule |  |
| 4.3 | 7736 | — | — | 0.00 | missing | Hook input/output JSON schemas |  |
| 4.4 | 8045 | — | — | 0.00 | missing | Hook config — HookConfig / HookMatcher / HookEntry |  |
| 4.5 | 8222 | — | — | 0.00 | missing | Hook executor — ExecuteHooks signature |  |
| 4.6 | 8365 | — | — | 0.00 | missing | Auth — AnthropicCredentials Type field |  |
| 4.7 | 8574 | — | — | 0.00 | missing | OAuth client — Anthropic endpoints |  |
| 4.8 | 8752 | — | — | 0.00 | missing | Session.Message + Builtin tool inventory (locations) |  |
| 5.1 | 9076 | — | — | 0.00 | missing | Script mode — frontmatter format example |  |
| 5.2 | 9165 | — | — | 0.00 | missing | Env-var + script-args substitution patterns |  |
| 5.3 | 9466 | — | — | 0.00 | missing | SDK — Options struct + entry signatures |  |
| 5.4 | 9559 | — | — | 0.00 | missing | UI CLI — slash command names (locations) |  |
| 5.5 | 9719 | — | — | 0.00 | missing | InitConfig — config search order |  |
| 5.6 | 9983 | — | — | 0.00 | missing | Anthropic alias resolution map |  |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 9 | 5360 | README.md section #<n> |
| 7 | 1101 | sdk/README.md section #<n> |
| 2 | 449 | AGENTS.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 2914 | 0.86 | 3376 | 9550 | README.md section #8 |
| 832 | 1.00 | 832 | 5494 | README.md section #10 |
| 470 | 1.00 | 470 | 2757 | README.md section #1 |
| 460 | 1.00 | 460 | 4075 | README.md section #5 |
| 388 | 1.00 | 388 | 3615 | README.md section #7 |
| 274 | 1.00 | 274 | 4349 | AGENTS.md section #1 |
| 243 | 1.00 | 243 | 4662 | contribute/contribute.md section #1 |
| 231 | 1.00 | 231 | 3219 | sdk/README.md section #3 |
| 225 | 1.00 | 225 | 2982 | sdk/README.md section #2 |
| 211 | 1.00 | 211 | 6060 | sdk/README.md section #4 |
| 180 | 1.00 | 180 | 2287 | json config contribute/conf/demo.json |
| 175 | 1.00 | 175 | 1361 | AGENTS.md section #0 |
| 151 | 1.00 | 151 | 5849 | sdk/README.md section #7 |
| 142 | 1.00 | 142 | 5698 | sdk/README.md section #5 |
| 116 | 1.00 | 116 | 843 | headings outline in sdk/README.md |
| 98 | 1.00 | 98 | 1186 | README.md section #4 |
| 89 | 1.00 | 89 | 6149 | headings outline in examples/scripts/README.md |
| 88 | 1.00 | 88 | 1533 | sdk/README.md section #8 |
| 87 | 1.00 | 87 | 1620 | plaintext config .gitignore |
| 86 | 1.00 | 86 | 1020 | listing of 'internal/ui' |
| 84 | 1.00 | 84 | 1445 | README.md section #12 |
| 63 | 1.00 | 63 | 1088 | README.md section #14 |
| 53 | 1.00 | 53 | 207 | headings outline in AGENTS.md |
| 53 | 1.00 | 53 | 4402 | sdk/README.md section #6 |
| 51 | 1.00 | 51 | 934 | README.md section #11 |

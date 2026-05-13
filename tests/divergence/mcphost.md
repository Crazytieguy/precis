scores: Score(3000)=0.551 ns_rows≤3K=13/40 (reached=5 partial=1 missing=7)

## Per-budget scores

| B | A_B | I(B) | C(B) | compl(B) | Score(B) | walker_used |
|--:|----:|-----:|-----:|---------:|---------:|------------:|
| 1000 | 86 | 0.807 | 0.422 | 0.589 | 0.584 | 990 |
| 1442 | 125 | 0.765 | 0.336 | 0.650 | 0.507 | 1409 |
| 2080 | 173 | 0.730 | 0.265 | 0.681 | 0.440 | 2005 |
| 3000 | 208 | 0.786 | 0.387 | 0.884 | 0.551 | 2979 |
| 4327 | 333 | 0.730 | 0.242 | 0.797 | 0.420 | 4304 |
| 6240 | 468 | 0.698 | 0.172 | 0.797 | 0.346 | 6237 |
| 9000 | 652 | 0.669 | 0.172 | 0.707 | 0.339 | 8981 |

## Top opportunities

### Additive (close partial / missing rows)

| intervention | rows | gap@1k | gap@3k | gap@9k | evidence | top row ids |
|:-------------|-----:|-------:|-------:|-------:|:---------|:------------|
| split wrong-slice walker batches | 20 | 1.53 | 1.28 | 1.20 | nearby candidates have low exact atom overlap | 1.6, 1.2, 2.1, 1.4, 3.2, ... |
| promote go decl signature batches | 7 | 0.49 | 0.49 | 0.49 | 5 files, exact total=147/159 | 2.2, 2.7, 2.10, 4.1, 3.1, ... |
| tune ranking for high-overlap unscheduled candidates | 4 | 0.39 | 0.39 | 0.39 | high-overlap candidates not in the schedule by T_max, exact total=84/93 | 1.8, 2.6, 5.1, 5.4 |
| promote go decl names surfaces | 4 | 0.33 | 0.33 | 0.33 | 4 files, exact total=66/75 | 2.3, 2.4, 4.6, 5.2 |

### Subtractive (suppress consistently off-NS batches)

| pattern | batches | freed@1k | freed@3k | freed@9k | evidence | top batch ids |
|:--------|--------:|---------:|---------:|---------:|:---------|:--------------|
| headings outline in sdk/README.md | 1 | 0 | 116 | 116 | off_3k=116 | headings outline in sdk/README.md |
| go module file go.mod | 1 | 0 | 84 | 84 | off_3k=84 | go module file go.mod |
| go decl names surface in internal/agent/factory.go | 1 | 0 | 78 | 78 | off_3k=78 | go decl names surface in internal/agent/factory.go |
| go decl names surface in internal/models/generate_models.go | 1 | 0 | 74 | 74 | off_3k=74 | go decl names surface in internal/models/generate_models.go |
| go decl names surface in sdk/types.go | 1 | 63 | 63 | 63 | off_3k=63 | go decl names surface in sdk/types.go |

Top missed paths (NS rows ≤ 3K): internal/config/config.go (2 rows, 48 atoms), cmd/root.go (1 row, 35 atoms), main.go (2 rows, 26 atoms), README.md (2 rows, 24 atoms), go.mod (1 row, 6 atoms)

## Arrival ledger by diagnosis

### ranking-recoverable

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 1.8 | 1004 | 0.00 | missing | README — host/client/server architecture model | [unscheduled bbox exact=15/18] README.md section #14 (15 atoms, too expensive at final margin) |
| 2.2 | 2004 | 0.00 | missing | MCPServerConfig.GetTransportType — type→transport mapping | [unscheduled bbox exact=25/29] go decl body at internal/config/config.go:185 (25 atoms, predecessor not scheduled: go decl at internal/config/config.go:185) |
| 2.3 | 2486 | 0.00 | missing | rootCmd — Use / Short / Long with examples | [unscheduled bbox exact=30/35] go decl at cmd/root.go:92 (30 atoms, predecessor not scheduled: go decl names surface in cmd/root.go) |
| 2.4 | 3017 | 0.00 | missing | Config — top-level config struct (fields) | [unscheduled bbox exact=24/28] go decl at internal/config/config.go:155 (24 atoms, predecessor not scheduled: go decl names surface in internal/config/config.go) |
| 2.6 | 3631 | 0.00 | missing | README — Builtin servers catalogue | [unscheduled bbox exact=36/39] README.md section #33 (36 atoms, too expensive at final margin) |
| 2.7 | 3928 | 0.00 | missing | rootCmd flags — enumerated (locations only) | [unscheduled bbox exact=25/25] go decl body at cmd/root.go:273 (45 atoms, predecessor not scheduled: go decl at cmd/root.go:273) |
| 2.10 | 4895 | 0.00 | missing | MCPServerConfig.Validate — per-transport requirements | [unscheduled bbox exact=24/27] go decl body at internal/config/config.go:218 (24 atoms, predecessor not scheduled: go decl at internal/config/config.go:218) |
| 3.1 | 5136 | 0.00 | missing | Agent loop — outer for + LLM call + tool-call branch arm | [unscheduled bbox exact=17/20] go decl body at internal/agent/agent.go:144 (17 atoms, predecessor not scheduled: go decl at internal/agent/agent.go:144) |
| 4.1 | 7004 | 0.00 | missing | CreateProvider — provider switch | [unscheduled bbox exact=36/37] go decl body at internal/models/providers.go:153 (36 atoms, predecessor not scheduled: go decl at internal/models/providers.go:153) |
| 4.6 | 8365 | 0.00 | missing | Auth — AnthropicCredentials Type field | [unscheduled bbox exact=8/8] go decl at internal/auth/credentials.go:22 (8 atoms, predecessor not scheduled: go decl names surface in internal/auth/credentials.go) |
| 4.7 | 8574 | 0.00 | missing | OAuth client — Anthropic endpoints | [unscheduled bbox exact=10/10] go decl body at internal/auth/oauth.go:39 (10 atoms, predecessor not scheduled: go decl at internal/auth/oauth.go:39) |
| 5.1 | 9076 | 0.00 | missing | Script mode — frontmatter format example | [unscheduled bbox exact=26/29] README.md section #34 (26 atoms, too expensive at final margin) |
| 5.2 | 9165 | 0.00 | missing | Env-var + script-args substitution patterns | [unscheduled bbox exact=4/4] go decl at internal/config/substitution.go:11 (4 atoms, predecessor not scheduled: go decl names surface in internal/config/substitution.go) |
| 5.4 | 9559 | 0.00 | missing | UI CLI — slash command names (locations) | [unscheduled bbox exact=7/7] go decl at internal/ui/commands.go:16 (37 atoms, too expensive at final margin) |
| 5.6 | 9983 | 0.00 | missing | Anthropic alias resolution map | [unscheduled bbox exact=10/11] go decl body at internal/models/providers.go:36 (10 atoms, predecessor not scheduled: go decl at internal/models/providers.go:36) |

### wrong-slice / granularity

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 1.2 | 135 | 0.50 | missing | README lede | [scheduled bbox exact=2/6] README headline in README.md (t=99, 2 atoms) |
| 1.4 | 257 | 0.50 | missing | go.mod — module declaration | [scheduled bbox exact=3/6] go module file go.mod (t=2562, 3 atoms) |
| 1.5 | 353 | 0.75 | partial | main.go — package + imports + version literal | [scheduled bbox exact=8/12] go package + imports in main.go (t=2633, 8 atoms) |
| 1.6 | 498 | 0.14 | missing | main.go — version-flag short-circuit + fang.Execute | [scheduled bbox exact=3/14] go decl names surface in main.go (t=145, 3 atoms); better unscheduled exact=10/14: go decl body at main.go:14 (10 atoms, too expensive at final margin) |
| 2.1 | 1737 | 0.00 | missing | MCPServerConfig — struct definition (the central config type) | [unscheduled bbox exact=15/19] go decl at internal/config/config.go:17 (15 atoms, predecessor not scheduled: go decl names surface in internal/config/config.go) |
| 2.5 | 3154 | 0.00 | missing | Builtin server registry — names + factory dispatch | [unscheduled bbox exact=10/14] go decl body at internal/builtin/registry.go:42 (10 atoms, predecessor not scheduled: go decl at internal/builtin/registry.go:42) |
| 2.8 | 4183 | 0.01 | missing | Subcommand cobra Use/Short — auth / hooks / script (locations) | [scheduled bbox exact=4/19] go decl names surface in cmd/auth.go (t=3745, 7 atoms) |
| 2.9 | 4519 | 0.00 | missing | Agent — type + GenerateWithLoopAndStreaming signature | [unscheduled bbox exact=9/21] go decl at internal/agent/agent.go:67 (9 atoms, predecessor not scheduled: go decl names surface in internal/agent/agent.go) |
| 3.2 | 5506 | 0.00 | missing | MCPToolManager — type | [unscheduled bbox exact=9/29] go decl at internal/tools/mcp.go:27 (9 atoms, predecessor not scheduled: go decl names surface in internal/tools/mcp.go) |
| 3.3 | 5811 | 0.00 | missing | MCPToolManager.LoadTools + tool prefixing rule | [unscheduled bbox exact=16/22] go decl body at internal/tools/mcp.go:149 (21 atoms, predecessor not scheduled: go decl at internal/tools/mcp.go:149) |
| 3.4 | 6037 | 0.00 | missing | Connection pool — type + DefaultConnectionPoolConfig | [unscheduled bbox exact=7/16] go decl body at internal/tools/connection_pool.go:32 (7 atoms, predecessor not scheduled: go decl at internal/tools/connection_pool.go:32) |
| 3.5 | 6266 | 0.00 | missing | Connection pool — createMCPClient transport switch | [unscheduled bbox exact=13/17] go decl body at internal/tools/connection_pool.go:262 (13 atoms, predecessor not scheduled: go decl at internal/tools/connection_pool.go:262) |
| 3.6 | 6549 | 0.06 | missing | Streaming aggregator — StreamWithCallback signature + provider note | [scheduled bbox exact=2/17] go decl names surface in internal/agent/streaming.go (t=762, 2 atoms); better unscheduled exact=8/17: go decl doc at internal/agent/streaming.go:19 (8 atoms, too expensive at final margin) |
| 4.2 | 7225 | 0.00 | missing | HookEvent constants + RequiresMatcher rule | [scheduled bbox exact=10/17] go decl names surface in internal/hooks/events.go (t=5216, 12 atoms) |
| 4.3 | 7736 | 0.42 | missing | Hook input/output JSON schemas | [scheduled bbox exact=10/34] go decl names surface in internal/hooks/schemas.go (t=2727, 12 atoms) |
| 4.4 | 8045 | 0.00 | missing | Hook config — HookConfig / HookMatcher / HookEntry | [scheduled bbox exact=6/22] go decl names surface in internal/hooks/config.go (t=3417, 6 atoms) |
| 4.5 | 8222 | 0.00 | missing | Hook executor — ExecuteHooks signature | [unscheduled bbox exact=4/11] go decl body at internal/hooks/executor.go:122 (16 atoms, predecessor not scheduled: go decl at internal/hooks/executor.go:122) |
| 4.8 | 8752 | 0.00 | missing | Session.Message + Builtin tool inventory (locations) | [unscheduled bbox exact=4/11] go decl body at internal/builtin/http.go:36 (47 atoms, predecessor not scheduled: go decl at internal/builtin/http.go:36) |
| 5.3 | 9466 | 0.00 | missing | SDK — Options struct + entry signatures | [scheduled bbox exact=9/20] go decl names surface in sdk/mcphost.go (t=5656, 19 atoms) |
| 5.5 | 9719 | 0.00 | missing | InitConfig — config search order | [unscheduled bbox exact=8/11] go decl body at cmd/root.go:143 (8 atoms, predecessor not scheduled: go decl at cmd/root.go:143) |

Top wasted paths (off-NS at 3K): internal/hooks/schemas.go (131t, 2 batches), internal/models/generate_models.go (129t, 2 batches), sdk/README.md (116t, 1 batch), internal/models/models_data.go (109t, 2 batches), go.mod (84t, 1 batch), internal/agent/factory.go (78t, 1 batch), cmd/hooks.go (69t, 1 batch), sdk/types.go (63t, 1 batch), +2 more

## Walker waste (walker_t ≤ 3000, off_3k ≥ 50)

| off_3k | off_3k_ratio | off_any | cost | walker_t | batch |
|-------:|-------------:|--------:|-----:|---------:|:------|
| 116 | 1.00 | 116 | 116 | 1522 | headings outline in sdk/README.md |
| 84 | 0.15 | 84 | 557 | 2005 | go module file go.mod |
| 78 | 1.00 | 78 | 78 | 2979 | go decl names surface in internal/agent/factory.go |
| 74 | 1.00 | 74 | 74 | 2791 | go decl names surface in internal/models/generate_models.go |
| 72 | 1.00 | 13 | 72 | 2655 | go decl names surface in internal/hooks/schemas.go |
| 69 | 1.00 | 61 | 69 | 921 | go decl names surface in cmd/hooks.go |
| 63 | 1.00 | 63 | 63 | 780 | go decl names surface in sdk/types.go |
| 59 | 1.00 | 0 | 59 | 2920 | go decl at internal/hooks/schemas.go:32 |
| 59 | 1.00 | 59 | 59 | 1733 | go decl names surface in internal/models/models_data.go |
| 55 | 1.00 | 55 | 55 | 2865 | go decl at internal/models/generate_models.go:50 |
| 158 | — | — | — | — | +3 more rows |

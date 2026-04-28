scores: Sim=0.385 Reached=6/40 Early=1 Late=3 Partial=5 Missing=29 Used=9994/10000

## Verdict

Verdict: wrong-slice bound
Likely primary lever: split walker batches to match NS semantic slices
Evidence: 16 ranking-recoverable (w×gap=2.74), 18 wrong-slice/granularity (w×gap=2.93), 0 no-discovered (w×gap=0.00)
Secondary intervention: free final budget for 5 too-expensive candidates
Loss reasons: 11 predecessor-gated, 5 too-expensive, 0 discovered-unscheduled
Top rows: 1.6, 1.2, 1.4, 2.1, 1.5, ...
Note: likely lever is heuristic; verify `Sim` moves, not just bucket counts.

## Top opportunities

_`w(t)×gap` is a non-additive priority score: Σ exp(-exp_t/τ) × (1 - credit) per row, τ=2000. Same time weighting and credit gap as Sim, but rows can overlap between opportunities — sums across rows are an upper bound on Sim impact, not an additive estimate._

| intervention | rows | w(t)×gap | bands ≤3k/≤6k/total | evidence | top row ids |
|:-------------|-----:|---------:|:----------------------|:---------|:------------|
| split wrong-slice walker batches | 18 | 2.93 | 5/10/18 | nearby candidates have low exact atom overlap | 1.6, 1.2, 1.4, 2.1, 1.5, ... |
| free final budget / demote late waste | 5 | 1.48 | 2/3/5 | high-overlap candidates exceed final remaining budget, exact total=99/110 | 1.7, 1.8, 2.6, 5.1, 5.4 |
| promote go decl signature batches | 7 | 0.72 | 1/4/7 | 5 files, exact total=147/159 | 2.2, 2.7, 2.10, 3.1, 4.1, ... |
| promote go decl names surfaces | 4 | 0.54 | 1/2/4 | 4 files, exact total=66/75 | 2.3, 2.4, 4.6, 5.2 |

Tiers: 1=4/10 reached, 3 partial, 3 missing, avg=0.59; 2=0/10 reached, 0 partial, 10 missing, avg=0.00; 3=0/6 reached, 0 partial, 6 missing, avg=0.01; 4=1/8 reached, 2 partial, 5 missing, avg=0.28; 5=1/6 reached, 0 partial, 5 missing, avg=0.17

## Diagnosis rollup

| diagnosis | rows | missing | partial | timing | likely lever |
|:----------|-----:|--------:|--------:|-------:|:-------------|
| ranking-recoverable | 16 | 16 | 0 | 0 | value/ranking |
| wrong-slice / granularity | 18 | 13 | 5 | 0 | walker granularity / wrong slice |
| timing-only | 5 | 0 | 0 | 5 | usually no code change |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | w(t)×gap | likely lever |
|:------------|-----:|---------:|:-------------|
| predecessor not scheduled | 11 | 1.26 | promote predecessor |
| too expensive at final margin | 5 | 1.48 | free final budget |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=12, unscheduled bbox=26, fs-only=1

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | aligned | low | 1 |
| scheduled bbox | early | low | 1 |
| scheduled bbox | late | full | 2 |
| scheduled bbox | missing | low | 3 |
| scheduled bbox | partial | low | 5 |
| unscheduled bbox | missing | none | 1 |
| unscheduled bbox | missing | low | 9 |
| unscheduled bbox | missing | high | 11 |
| unscheduled bbox | missing | full | 5 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.7 | 730 | — | — | 0.00 | missing | README — supported features list | [unscheduled bbox exact=15/17] README.md section #3 (15 atoms, too expensive at final margin) |
| 1.8 | 1004 | — | — | 0.00 | missing | README — host/client/server architecture model | [unscheduled bbox exact=15/18] README.md section #2 (15 atoms, too expensive at final margin) |
| 2.2 | 2004 | — | — | 0.00 | missing | MCPServerConfig.GetTransportType — type→transport mapping | [unscheduled bbox exact=25/29] go decl body at internal/config/config.go:185 (25 atoms, predecessor not scheduled: go decl at internal/config/config.go:185) |
| 2.3 | 2486 | — | — | 0.00 | missing | rootCmd — Use / Short / Long with examples | [unscheduled bbox exact=30/35] go decl at cmd/root.go:92 (30 atoms, predecessor not scheduled: go decl names surface in cmd/root.go) |
| 2.4 | 3017 | — | — | 0.00 | missing | Config — top-level config struct (fields) | [unscheduled bbox exact=24/28] go decl at internal/config/config.go:155 (24 atoms, predecessor not scheduled: go decl names surface in internal/config/config.go) |
| 2.6 | 3631 | — | — | 0.00 | missing | README — Builtin servers catalogue | [unscheduled bbox exact=36/39] README.md section #8 (36 atoms, too expensive at final margin) |
| 2.7 | 3928 | — | — | 0.00 | missing | rootCmd flags — enumerated (locations only) | [unscheduled bbox exact=25/25] go decl body at cmd/root.go:273 (45 atoms, predecessor not scheduled: go decl at cmd/root.go:273) |
| 2.10 | 4895 | — | — | 0.00 | missing | MCPServerConfig.Validate — per-transport requirements | [unscheduled bbox exact=24/27] go decl body at internal/config/config.go:218 (24 atoms, predecessor not scheduled: go decl at internal/config/config.go:218) |
| 3.1 | 5136 | — | — | 0.00 | missing | Agent loop — outer for + LLM call + tool-call branch arm | [unscheduled bbox exact=17/20] go decl body at internal/agent/agent.go:144 (17 atoms, predecessor not scheduled: go decl at internal/agent/agent.go:144) |
| 4.1 | 7004 | — | — | 0.00 | missing | CreateProvider — provider switch | [unscheduled bbox exact=36/37] go decl body at internal/models/providers.go:153 (36 atoms, predecessor not scheduled: go decl at internal/models/providers.go:153) |
| 4.6 | 8365 | — | — | 0.00 | missing | Auth — AnthropicCredentials Type field | [unscheduled bbox exact=8/8] go decl at internal/auth/credentials.go:22 (8 atoms, predecessor not scheduled: go decl names surface in internal/auth/credentials.go) |
| 4.7 | 8574 | — | — | 0.00 | missing | OAuth client — Anthropic endpoints | [unscheduled bbox exact=10/10] go decl body at internal/auth/oauth.go:39 (10 atoms, predecessor not scheduled: go decl at internal/auth/oauth.go:39) |
| 5.1 | 9076 | — | — | 0.00 | missing | Script mode — frontmatter format example | [unscheduled bbox exact=26/29] README.md section #9 (26 atoms, too expensive at final margin) |
| 5.2 | 9165 | — | — | 0.00 | missing | Env-var + script-args substitution patterns | [unscheduled bbox exact=4/4] go decl at internal/config/substitution.go:11 (4 atoms, predecessor not scheduled: go decl names surface in internal/config/substitution.go) |
| 5.4 | 9559 | — | — | 0.00 | missing | UI CLI — slash command names (locations) | [unscheduled bbox exact=7/7] go decl at internal/ui/commands.go:16 (37 atoms, too expensive at final margin) |
| 5.6 | 9983 | — | — | 0.00 | missing | Anthropic alias resolution map | [unscheduled bbox exact=10/11] go decl body at internal/models/providers.go:36 (10 atoms, predecessor not scheduled: go decl at internal/models/providers.go:36) |

### wrong-slice / granularity

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 135 | — | — | 0.50 | partial | README lede | [scheduled bbox exact=2/6] README headline in README.md (t=99, 2 atoms) |
| 1.4 | 257 | — | — | 0.50 | partial | go.mod — module declaration | [scheduled bbox exact=3/6] go module file go.mod (t=2207, 3 atoms) |
| 1.5 | 353 | — | — | 0.75 | partial | main.go — package + imports + version literal | [scheduled bbox exact=8/12] go package + imports in main.go (t=2278, 8 atoms) |
| 1.6 | 498 | — | — | 0.14 | missing | main.go — version-flag short-circuit + fang.Execute | [scheduled bbox exact=3/14] go decl names surface in main.go (t=145, 3 atoms); better unscheduled exact=10/14: go decl body at main.go:14 (10 atoms, too expensive at final margin) |
| 2.1 | 1737 | — | — | 0.00 | missing | MCPServerConfig — struct definition (the central config type) | [unscheduled bbox exact=15/19] go decl at internal/config/config.go:17 (15 atoms, predecessor not scheduled: go decl names surface in internal/config/config.go) |
| 2.5 | 3154 | — | — | 0.00 | missing | Builtin server registry — names + factory dispatch | [unscheduled bbox exact=10/14] go decl body at internal/builtin/registry.go:42 (10 atoms, predecessor not scheduled: go decl at internal/builtin/registry.go:42) |
| 2.8 | 4183 | — | — | 0.03 | missing | Subcommand cobra Use/Short — auth / hooks / script (locations) | [scheduled bbox exact=4/19] go decl names surface in cmd/auth.go (t=3368, 7 atoms) |
| 2.9 | 4519 | — | — | 0.00 | missing | Agent — type + GenerateWithLoopAndStreaming signature | [unscheduled bbox exact=0/21] go decl body at internal/agent/agent.go:81 (33 atoms, predecessor not scheduled: go decl at internal/agent/agent.go:81) |
| 3.2 | 5506 | — | — | 0.00 | missing | MCPToolManager — type | [unscheduled bbox exact=9/29] go decl at internal/tools/mcp.go:27 (9 atoms, predecessor not scheduled: go decl names surface in internal/tools/mcp.go) |
| 3.3 | 5811 | — | — | 0.00 | missing | MCPToolManager.LoadTools + tool prefixing rule | [unscheduled bbox exact=2/22] go decl body at internal/tools/mcp.go:178 (57 atoms, predecessor not scheduled: go decl at internal/tools/mcp.go:178) |
| 3.4 | 6037 | — | — | 0.00 | missing | Connection pool — type + DefaultConnectionPoolConfig | [unscheduled bbox exact=7/16] go decl body at internal/tools/connection_pool.go:32 (7 atoms, predecessor not scheduled: go decl at internal/tools/connection_pool.go:32) |
| 3.5 | 6266 | — | — | 0.00 | missing | Connection pool — createMCPClient transport switch | [unscheduled bbox exact=13/17] go decl body at internal/tools/connection_pool.go:262 (13 atoms, predecessor not scheduled: go decl at internal/tools/connection_pool.go:262) |
| 3.6 | 6549 | — | — | 0.06 | missing | Streaming aggregator — StreamWithCallback signature + provider note | [scheduled bbox exact=2/17] go decl names surface in internal/agent/streaming.go (t=601, 2 atoms); better unscheduled exact=8/17: go decl doc at internal/agent/streaming.go:19 (8 atoms, too expensive at final margin) |
| 4.2 | 7225 | — | — | 0.59 | partial | HookEvent constants + RequiresMatcher rule | [scheduled bbox exact=10/17] go decl names surface in internal/hooks/events.go (t=4812, 12 atoms) |
| 4.3 | 7736 | — | — | 0.77 | partial | Hook input/output JSON schemas | [scheduled bbox exact=10/34] go decl names surface in internal/hooks/schemas.go (t=2350, 12 atoms) |
| 4.5 | 8222 | — | — | 0.00 | missing | Hook executor — ExecuteHooks signature | [unscheduled bbox exact=1/11] go decl body at internal/hooks/executor.go:77 (34 atoms, predecessor not scheduled: go decl at internal/hooks/executor.go:77) |
| 4.8 | 8752 | — | — | 0.00 | missing | Session.Message + Builtin tool inventory (locations) | [unscheduled bbox exact=4/11] go decl body at internal/builtin/http.go:36 (47 atoms, predecessor not scheduled: go decl at internal/builtin/http.go:36) |
| 5.5 | 9719 | — | — | 0.00 | missing | InitConfig — config search order | [unscheduled bbox exact=8/11] go decl body at cmd/root.go:143 (8 atoms, predecessor not scheduled: go decl at cmd/root.go:143) |

### timing-only

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.3 | 201 | 449 | +248 | 1.00 | late | internal/ + cmd/ subtree listings | fs-only |
| 1.9 | 1339 | 2207 | +868 | 1.00 | late | go.mod — direct deps (CLI / config / MCP / LLM core) | [scheduled bbox exact=15/15] go module file go.mod (t=2207, 15 atoms) |
| 1.10 | 1436 | 2207 | +771 | 1.00 | late | go.mod — direct deps (UI / Gemini / misc) | [scheduled bbox exact=6/6] go module file go.mod (t=2207, 6 atoms) |
| 4.4 | 8045 | 7252 | -793 | 0.86 | aligned | Hook config — HookConfig / HookMatcher / HookEntry | [scheduled bbox exact=6/22] go decl names surface in internal/hooks/config.go (t=3040, 6 atoms) |
| 5.3 | 9466 | 5784 | -3682 | 1.00 | early | SDK — Options struct + entry signatures | [scheduled bbox exact=9/20] go decl names surface in sdk/mcphost.go (t=5252, 19 atoms) |

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
| 3527 | — | — | — | +56 more rows |

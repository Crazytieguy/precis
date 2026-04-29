scores: Score(3000)=0.551 ns_rows≤3K=13/40 (reached=5 partial=1 missing=7)

## Per-budget scores

| B | A_B | I(B) | C(B) | Score(B) | walker_used |
|--:|----:|-----:|-----:|---------:|------------:|
| 1000 | 86 | 0.807 | 0.422 | 0.584 | 984 |
| 1442 | 125 | 0.765 | 0.336 | 0.507 | 1403 |
| 2080 | 173 | 0.730 | 0.265 | 0.440 | 1999 |
| 3000 | 208 | 0.786 | 0.387 | 0.551 | 2973 |
| 4327 | 333 | 0.730 | 0.242 | 0.420 | 4298 |
| 6240 | 468 | 0.698 | 0.172 | 0.346 | 6231 |
| 9000 | 652 | 0.669 | 0.172 | 0.339 | 8975 |

## Verdict

Verdict: wrong-slice bound
Likely primary lever: split walker batches to match NS semantic slices
Evidence: 15 ranking-recoverable (gap@3k=1.21), 20 wrong-slice/granularity (gap@3k=1.28), 0 no-discovered (gap@3k=0.00)
Secondary intervention: free T_max budget for 4 too-expensive candidates
Top rows: 1.6, 1.2, 2.1, 1.4, 3.2, ...

## Top opportunities

_`gap@B` is a non-additive priority score: `Σ over atoms in row: (1 − damped_credit(a)) / rank(a)` evaluated at budget B's walker state. Approximates how much closing the row would lift `Score(B)` (via the Importance numerator); not an exact delta. `gap@3k` is the primary sort key. `gap@1k` and `gap@9k` show how the same intervention scales across the budget vector — gap is monotone non-increasing in B (walker has more budget at higher B). Rows can overlap between opportunities; sums are upper bounds on Score(B) impact, not additive estimates._

| intervention | rows | gap@1k | gap@3k | gap@9k | evidence | top row ids |
|:-------------|-----:|-------:|-------:|-------:|:---------|:------------|
| split wrong-slice walker batches | 20 | 1.53 | 1.28 | 1.20 | nearby candidates have low exact atom overlap | 1.6, 1.2, 2.1, 1.4, 3.2, ... |
| promote go decl signature batches | 7 | 0.49 | 0.49 | 0.49 | 5 files, exact total=147/159 | 2.2, 2.7, 2.10, 4.1, 3.1, ... |
| free T_max budget / demote late waste | 4 | 0.39 | 0.39 | 0.39 | high-overlap candidates exceed remaining budget at T_max (caveat: not 3K-budget — see below), exact total=84/93 | 1.8, 2.6, 5.1, 5.4 |
| promote go decl names surfaces | 4 | 0.33 | 0.33 | 0.33 | 4 files, exact total=66/75 | 2.3, 2.4, 4.6, 5.2 |
| finish partially-delivered NS batches | 1 | 0.06 | 0.05 | 0.04 | avg batch completion=0.25 | 4.3 |

## Diagnosis rollup

| diagnosis | rows | missing | partial | likely lever |
|:----------|-----:|--------:|--------:|:-------------|
| ranking-recoverable | 15 | 15 | 0 | value/ranking |
| wrong-slice / granularity | 20 | 19 | 1 | walker granularity / wrong slice |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | gap@3k | likely lever |
|:------------|-----:|-------:|:-------------|
| predecessor not scheduled | 11 | 0.82 | promote predecessor |
| too expensive at final margin | 4 | 0.39 | free T_max budget |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=10, unscheduled bbox=25

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | missing | low | 9 |
| scheduled bbox | partial | low | 1 |
| unscheduled bbox | missing | low | 10 |
| unscheduled bbox | missing | high | 10 |
| unscheduled bbox | missing | full | 5 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 1.8 | 1004 | 0.00 | 0.00 | missing | README — host/client/server architecture model | [unscheduled bbox exact=15/18] README.md section #14 (15 atoms, too expensive at final margin) |
| 2.2 | 2004 | 0.00 | 0.00 | missing | MCPServerConfig.GetTransportType — type→transport mapping | [unscheduled bbox exact=25/29] go decl body at internal/config/config.go:185 (25 atoms, predecessor not scheduled: go decl at internal/config/config.go:185) |
| 2.3 | 2486 | 0.00 | 0.00 | missing | rootCmd — Use / Short / Long with examples | [unscheduled bbox exact=30/35] go decl at cmd/root.go:92 (30 atoms, predecessor not scheduled: go decl names surface in cmd/root.go) |
| 2.4 | 3017 | 0.00 | 0.00 | missing | Config — top-level config struct (fields) | [unscheduled bbox exact=24/28] go decl at internal/config/config.go:155 (24 atoms, predecessor not scheduled: go decl names surface in internal/config/config.go) |
| 2.6 | 3631 | 0.00 | 0.00 | missing | README — Builtin servers catalogue | [unscheduled bbox exact=36/39] README.md section #33 (36 atoms, too expensive at final margin) |
| 2.7 | 3928 | 0.00 | 0.00 | missing | rootCmd flags — enumerated (locations only) | [unscheduled bbox exact=25/25] go decl body at cmd/root.go:273 (45 atoms, predecessor not scheduled: go decl at cmd/root.go:273) |
| 2.10 | 4895 | 0.00 | 0.00 | missing | MCPServerConfig.Validate — per-transport requirements | [unscheduled bbox exact=24/27] go decl body at internal/config/config.go:218 (24 atoms, predecessor not scheduled: go decl at internal/config/config.go:218) |
| 3.1 | 5136 | 0.00 | 0.00 | missing | Agent loop — outer for + LLM call + tool-call branch arm | [unscheduled bbox exact=17/20] go decl body at internal/agent/agent.go:144 (17 atoms, predecessor not scheduled: go decl at internal/agent/agent.go:144) |
| 4.1 | 7004 | 0.00 | 0.00 | missing | CreateProvider — provider switch | [unscheduled bbox exact=36/37] go decl body at internal/models/providers.go:153 (36 atoms, predecessor not scheduled: go decl at internal/models/providers.go:153) |
| 4.6 | 8365 | 0.00 | 0.00 | missing | Auth — AnthropicCredentials Type field | [unscheduled bbox exact=8/8] go decl at internal/auth/credentials.go:22 (8 atoms, predecessor not scheduled: go decl names surface in internal/auth/credentials.go) |
| 4.7 | 8574 | 0.00 | 0.00 | missing | OAuth client — Anthropic endpoints | [unscheduled bbox exact=10/10] go decl body at internal/auth/oauth.go:39 (10 atoms, predecessor not scheduled: go decl at internal/auth/oauth.go:39) |
| 5.1 | 9076 | 0.00 | 0.00 | missing | Script mode — frontmatter format example | [unscheduled bbox exact=26/29] README.md section #34 (26 atoms, too expensive at final margin) |
| 5.2 | 9165 | 0.00 | 0.00 | missing | Env-var + script-args substitution patterns | [unscheduled bbox exact=4/4] go decl at internal/config/substitution.go:11 (4 atoms, predecessor not scheduled: go decl names surface in internal/config/substitution.go) |
| 5.4 | 9559 | 0.00 | 0.00 | missing | UI CLI — slash command names (locations) | [unscheduled bbox exact=7/7] go decl at internal/ui/commands.go:16 (37 atoms, too expensive at final margin) |
| 5.6 | 9983 | 0.00 | 0.00 | missing | Anthropic alias resolution map | [unscheduled bbox exact=10/11] go decl body at internal/models/providers.go:36 (10 atoms, predecessor not scheduled: go decl at internal/models/providers.go:36) |

### wrong-slice / granularity

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 1.2 | 135 | 0.50 | 0.99 | missing | README lede | [scheduled bbox exact=2/6] README headline in README.md (t=99, 2 atoms) |
| 1.4 | 257 | 0.50 | 0.95 | missing | go.mod — module declaration | [scheduled bbox exact=3/6] go module file go.mod (t=2556, 3 atoms) |
| 1.5 | 353 | 0.75 | 0.98 | partial | main.go — package + imports + version literal | [scheduled bbox exact=8/12] go package + imports in main.go (t=2627, 8 atoms) |
| 1.6 | 498 | 0.14 | 0.05 | missing | main.go — version-flag short-circuit + fang.Execute | [scheduled bbox exact=3/14] go decl names surface in main.go (t=145, 3 atoms); better unscheduled exact=10/14: go decl body at main.go:14 (10 atoms, too expensive at final margin) |
| 2.1 | 1737 | 0.00 | 0.00 | missing | MCPServerConfig — struct definition (the central config type) | [unscheduled bbox exact=15/19] go decl at internal/config/config.go:17 (15 atoms, predecessor not scheduled: go decl names surface in internal/config/config.go) |
| 2.5 | 3154 | 0.00 | 0.00 | missing | Builtin server registry — names + factory dispatch | [unscheduled bbox exact=10/14] go decl body at internal/builtin/registry.go:42 (10 atoms, predecessor not scheduled: go decl at internal/builtin/registry.go:42) |
| 2.8 | 4183 | 0.01 | 0.01 | missing | Subcommand cobra Use/Short — auth / hooks / script (locations) | [scheduled bbox exact=4/19] go decl names surface in cmd/auth.go (t=3739, 7 atoms) |
| 2.9 | 4519 | 0.00 | 0.00 | missing | Agent — type + GenerateWithLoopAndStreaming signature | [unscheduled bbox exact=9/21] go decl at internal/agent/agent.go:67 (9 atoms, predecessor not scheduled: go decl names surface in internal/agent/agent.go) |
| 3.2 | 5506 | 0.00 | 0.00 | missing | MCPToolManager — type | [unscheduled bbox exact=9/29] go decl at internal/tools/mcp.go:27 (9 atoms, predecessor not scheduled: go decl names surface in internal/tools/mcp.go) |
| 3.3 | 5811 | 0.00 | 0.00 | missing | MCPToolManager.LoadTools + tool prefixing rule | [unscheduled bbox exact=16/22] go decl body at internal/tools/mcp.go:149 (21 atoms, predecessor not scheduled: go decl at internal/tools/mcp.go:149) |
| 3.4 | 6037 | 0.00 | 0.00 | missing | Connection pool — type + DefaultConnectionPoolConfig | [unscheduled bbox exact=7/16] go decl body at internal/tools/connection_pool.go:32 (7 atoms, predecessor not scheduled: go decl at internal/tools/connection_pool.go:32) |
| 3.5 | 6266 | 0.00 | 0.00 | missing | Connection pool — createMCPClient transport switch | [unscheduled bbox exact=13/17] go decl body at internal/tools/connection_pool.go:262 (13 atoms, predecessor not scheduled: go decl at internal/tools/connection_pool.go:262) |
| 3.6 | 6549 | 0.06 | 0.13 | missing | Streaming aggregator — StreamWithCallback signature + provider note | [scheduled bbox exact=2/17] go decl names surface in internal/agent/streaming.go (t=756, 2 atoms); better unscheduled exact=8/17: go decl doc at internal/agent/streaming.go:19 (8 atoms, too expensive at final margin) |
| 4.2 | 7225 | 0.00 | 0.00 | missing | HookEvent constants + RequiresMatcher rule | [scheduled bbox exact=10/17] go decl names surface in internal/hooks/events.go (t=5210, 12 atoms) |
| 4.3 | 7736 | 0.42 | 0.25 | missing | Hook input/output JSON schemas | [scheduled bbox exact=10/34] go decl names surface in internal/hooks/schemas.go (t=2721, 12 atoms) |
| 4.4 | 8045 | 0.00 | 0.00 | missing | Hook config — HookConfig / HookMatcher / HookEntry | [scheduled bbox exact=6/22] go decl names surface in internal/hooks/config.go (t=3411, 6 atoms) |
| 4.5 | 8222 | 0.00 | 0.00 | missing | Hook executor — ExecuteHooks signature | [unscheduled bbox exact=4/11] go decl body at internal/hooks/executor.go:122 (16 atoms, predecessor not scheduled: go decl at internal/hooks/executor.go:122) |
| 4.8 | 8752 | 0.00 | 0.00 | missing | Session.Message + Builtin tool inventory (locations) | [unscheduled bbox exact=4/11] go decl body at internal/builtin/http.go:36 (47 atoms, predecessor not scheduled: go decl at internal/builtin/http.go:36) |
| 5.3 | 9466 | 0.00 | 0.00 | missing | SDK — Options struct + entry signatures | [scheduled bbox exact=9/20] go decl names surface in sdk/mcphost.go (t=5650, 19 atoms) |
| 5.5 | 9719 | 0.00 | 0.00 | missing | InitConfig — config search order | [unscheduled bbox exact=8/11] go decl body at cmd/root.go:143 (8 atoms, predecessor not scheduled: go decl at cmd/root.go:143) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 4 | 296 | README.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 276 | 0.99 | 278 | 8609 | go decl names surface in cmd/script.go |
| 175 | 1.00 | 175 | 7927 | AGENTS.md section #0 |
| 160 | 1.00 | 160 | 9379 | go decl names surface in internal/ui/spinner.go |
| 150 | 0.95 | 158 | 3739 | go decl names surface in cmd/auth.go |
| 145 | 1.00 | 145 | 7568 | go decl names surface in internal/hooks/validator.go |
| 140 | 1.00 | 140 | 7351 | go decl names surface in internal/builtin/bash.go |
| 136 | 1.00 | 136 | 7111 | go decl names surface in internal/ui/styles.go |
| 116 | 1.00 | 116 | 1632 | headings outline in sdk/README.md |
| 102 | 1.00 | 102 | 4706 | go decl names surface in internal/ui/tool_approval_input.go |
| 98 | 1.00 | 98 | 6692 | README.md section #29 |
| 3020 | — | — | — | +48 more rows |

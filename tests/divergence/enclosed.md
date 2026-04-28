scores: Sim=0.532 Reached=14/45 Early=6 Late=6 Partial=1 Missing=30 Used=9915/10000

## Verdict

Verdict: budget-pressure bound
Likely primary lever: free final budget / demote late low-value spend
Evidence: 7 ranking-recoverable (w×gap=2.91), 12 wrong-slice/granularity (w×gap=2.21), 11 no-discovered (w×gap=0.62)
Secondary intervention: promote predecessors for 1 gated candidate
Loss reasons: 1 predecessor-gated, 6 too-expensive, 0 discovered-unscheduled
Top rows: 1.4, 1.7, 1.12, 1.11, 4.1, ...
Note: likely lever is heuristic; verify `Sim` moves, not just bucket counts.

## Top opportunities

_`w(t)×gap` is a non-additive priority score: Σ exp(-exp_t/τ) × (1 - credit) per row, τ=2000. Same time weighting and credit gap as Sim, but rows can overlap between opportunities — sums across rows are an upper bound on Sim impact, not an additive estimate._

| intervention | rows | w(t)×gap | bands ≤3k/≤6k/total | evidence | top row ids |
|:-------------|-----:|---------:|:----------------------|:---------|:------------|
| free final budget / demote late waste | 6 | 2.65 | 4/6/6 | high-overlap candidates exceed final remaining budget, exact total=95/97 | 1.4, 1.7, 1.12, 1.11, 4.1, ... |
| split wrong-slice walker batches | 12 | 2.21 | 4/7/12 | nearby candidates have low exact atom overlap | 1.2, 1.9, 2.7, 3.3, 3.4, ... |
| add walker candidates for no-discovered rows | 11 | 0.62 | 0/4/11 | NS rows have no discovered line candidate | 3.5, 3.6, 3.7, 3.8, 4.6, ... |
| promote package scripts in packages/lib/package.json | 1 | 0.26 | 1/1/1 | 0 files, exact total=8/8 | 2.8 |

Tiers: 1=6/12 reached, 0 partial, 6 missing, avg=0.51; 2=6/8 reached, 0 partial, 2 missing, avg=0.77; 3=1/8 reached, 1 partial, 6 missing, avg=0.21; 4=0/7 reached, 0 partial, 7 missing, avg=0.06; 5=0/5 reached, 0 partial, 5 missing, avg=0.00; 6=1/5 reached, 0 partial, 4 missing, avg=0.27

## Diagnosis rollup

| diagnosis | rows | missing | partial | timing | likely lever |
|:----------|-----:|--------:|--------:|-------:|:-------------|
| ranking-recoverable | 7 | 7 | 0 | 0 | value/ranking |
| wrong-slice / granularity | 12 | 12 | 0 | 0 | walker granularity / wrong slice |
| no discovered candidate | 11 | 11 | 0 | 0 | walker coverage or predecessor-gated emit |
| fs/listing | 1 | 0 | 1 | 0 | filesystem/listing value |
| timing-only | 12 | 0 | 0 | 12 | usually no code change |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | w(t)×gap | likely lever |
|:------------|-----:|---------:|:-------------|
| predecessor not scheduled | 1 | 0.26 | promote predecessor |
| too expensive at final margin | 6 | 2.65 | free final budget |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=14, unscheduled bbox=6, scheduled same-file=2, unscheduled same-file=2, fs-only=8, no discovered candidate=11

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | late | low | 5 |
| scheduled bbox | missing | low | 9 |
| unscheduled bbox | missing | high | 1 |
| unscheduled bbox | missing | full | 5 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.4 | 168 | — | — | 0.00 | missing | pnpm workspace glob | [unscheduled bbox exact=2/2] plaintext config pnpm-workspace.yaml (2 atoms, too expensive at final margin) |
| 1.7 | 497 | — | — | 0.00 | missing | pnpm-workspace.yaml — catalog versions | [unscheduled bbox exact=9/9] plaintext config pnpm-workspace.yaml (9 atoms, too expensive at final margin) |
| 1.11 | 1499 | — | — | 0.20 | missing | README — how-it-works (creator side) | [scheduled bbox exact=2/10] headings outline in README.md (t=2180, 2 atoms); better unscheduled exact=9/10: README.md section #17 (9 atoms, too expensive at final margin) |
| 1.12 | 1700 | — | — | 0.00 | missing | README — how-it-works (recipient side) | [unscheduled bbox exact=6/7] README.md section #17 (6 atoms, too expensive at final margin) |
| 2.8 | 2657 | — | — | 0.00 | missing | lib package.json — runtime dependencies | [unscheduled bbox exact=8/8] package dependencies in packages/lib/package.json (8 atoms, predecessor not scheduled: package scripts in packages/lib/package.json) |
| 4.1 | 5055 | — | — | 0.00 | missing | Config — env var name catalog (locations only) | [unscheduled bbox exact=26/26] export at packages/app-server/src/modules/app/config/config.ts:5 (242 atoms, too expensive at final margin) |
| 4.2 | 5587 | — | — | 0.00 | missing | Config — defaults for the most-asked-about env vars | [unscheduled bbox exact=43/43] export at packages/app-server/src/modules/app/config/config.ts:5 (43 atoms, too expensive at final margin) |

### wrong-slice / granularity

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 127 | — | — | 0.00 | missing | README h1 + tagline | [scheduled same-file] headings outline in README.md (t=2180, 38 atoms) |
| 1.9 | 872 | — | — | 0.20 | missing | README — project structure (per-package one-liners) | [scheduled bbox exact=2/10] headings outline in README.md (t=2180, 2 atoms); better unscheduled exact=7/10: README.md section #23 (7 atoms, too expensive at final margin) |
| 2.7 | 2545 | — | — | 0.19 | missing | @enclosed/lib README — install + usage example | [scheduled bbox exact=4/31] headings outline in packages/lib/README.md (t=1212, 4 atoms); better unscheduled exact=11/31: packages/lib/README.md section #2 (11 atoms, too expensive at final margin) |
| 3.3 | 2947 | — | — | 0.12 | missing | Cloudflare Workers entrypoint (full) | [scheduled bbox exact=1/8] export at packages/app-server/src/index.cloudflare.ts:8 (t=844, 1 atoms); better unscheduled exact=2/8: imports in packages/app-server/src/index.cloudflare.ts (2 atoms, discovered unscheduled) |
| 3.4 | 3451 | — | — | 0.05 | missing | Hono createServer factory + middleware stack | [scheduled bbox exact=2/41] export names surface in packages/app-server/src/modules/app/server.ts (t=5625, 2 atoms); better unscheduled exact=15/41: imports in packages/app-server/src/modules/app/server.ts (15 atoms, too expensive at final margin) |
| 4.3 | 5731 | — | — | 0.43 | missing | Config — getConfig export + figue setup | [scheduled bbox exact=5/14] export body at packages/app-server/src/modules/app/config/config.ts:258 body 259 (t=8792, 5 atoms) |
| 4.4 | 5875 | — | — | 0.00 | missing | Node.js entrypoint — config + storage + server build | [unscheduled same-file] imports in packages/app-server/src/index.node.ts (14 atoms, too expensive at final margin) |
| 4.5 | 6202 | — | — | 0.00 | missing | Node.js entrypoint — static + SPA fallback + cron + listen | [unscheduled same-file] imports in packages/app-server/src/index.node.ts (14 atoms, too expensive at final margin) |
| 6.1 | 9134 | — | — | 0.00 | missing | Lib — encryptNote body (crypto.usecases) | [scheduled same-file] export names surface in packages/lib/src/crypto/crypto.usecases.ts (t=3892, 2 atoms) |
| 6.2 | 9232 | — | — | 0.29 | missing | Lib — note URL hash-fragment markers (`pw` / `dar`) | [scheduled bbox exact=2/7] export names surface in packages/lib/src/notes/notes.models.ts (t=4122, 2 atoms) |
| 6.3 | 9539 | — | — | 0.04 | missing | CLI dispatcher + create-note args | [scheduled bbox exact=2/30] export names surface in packages/cli/src/create-note/create-note.command.ts (t=3074, 2 atoms); better unscheduled exact=12/30: export at packages/cli/src/create-note/create-note.command.ts:13 (12 atoms, too expensive at final margin) |
| 6.4 | 9903 | — | — | 0.03 | missing | App-client — Solid Router routes | [scheduled bbox exact=2/32] export at packages/app-client/src/routes.tsx:11 (t=3441, 2 atoms); better unscheduled exact=19/32: export body at packages/app-client/src/routes.tsx:11 body 12 (19 atoms, too expensive at final margin) |

### no discovered candidate

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 3.5 | 3749 | — | — | 0.00 | missing | Notes routes — endpoint registrations (locations only) | no discovered line candidate |
| 3.6 | 4200 | — | — | 0.00 | missing | Notes routes — POST /api/notes payload schema + validation handler | no discovered line candidate |
| 3.7 | 4659 | — | — | 0.00 | missing | Notes routes — GET /api/notes/:noteId + private-note auth gating | no discovered line candidate |
| 3.8 | 4821 | — | — | 0.00 | missing | Notes routes — GET /api/notes/:noteId/exists | no discovered line candidate |
| 4.6 | 6457 | — | — | 0.00 | missing | Storage drivers — memory + fs-lite | no discovered line candidate |
| 4.7 | 6892 | — | — | 0.00 | missing | Storage driver — Cloudflare KV (with 413 translation) | no discovered line candidate |
| 5.1 | 7406 | — | — | 0.00 | missing | Notes — models + types + constants | no discovered line candidate |
| 5.2 | 7835 | — | — | 0.00 | missing | Notes repository — exports + factory + getRefreshedNote usecase | no discovered line candidate |
| 5.3 | 8111 | — | — | 0.00 | missing | Auth catalog — errors + models + services signatures | no discovered line candidate |
| 5.4 | 8429 | — | — | 0.00 | missing | Auth middleware — gating + protected-route guard | no discovered line candidate |
| 5.5 | 8821 | — | — | 0.00 | missing | Auth login route — handler body (timing-safe bcrypt + JWT issue) | no discovered line candidate |

### fs/listing

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 3.2 | 2851 | — | — | 0.51 | partial | app-server/src/modules — module map | fs-only |

### timing-only

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.5 | 250 | 1729 | +1479 | 0.83 | late | Root package.json — name + version + license | [scheduled bbox exact=4/6] package identity in package.json (t=332, 4 atoms) |
| 1.6 | 357 | 1729 | +1372 | 1.00 | late | Root package.json — author + repo + engines | [scheduled bbox exact=5/8] package identity in package.json (t=332, 5 atoms) |
| 1.8 | 694 | 1729 | +1035 | 0.95 | late | Root package.json — scripts + keywords + devDeps | [scheduled bbox exact=10/19] package identity in package.json (t=332, 10 atoms) |
| 1.10 | 1225 | 4500 | +3275 | 0.94 | late | README — features list | [scheduled bbox exact=2/16] headings outline in README.md (t=2180, 2 atoms) |
| 2.1 | 1726 | 401 | -1325 | 1.00 | early | Top-level lib package contents | fs-only |
| 2.2 | 1742 | 498 | -1244 | 1.00 | early | lib/src — top-level source modules | fs-only |
| 2.3 | 2101 | 5439 | +3338 | 0.96 | late | @enclosed/lib public API — index.ts re-exports | [scheduled bbox exact=19/28] export at packages/lib/src/index.ts:10 (t=670, 19 atoms) |
| 2.4 | 2124 | 734 | -1390 | 1.00 | early | Top-level app-server package contents | fs-only |
| 2.5 | 2187 | 3316 | +1129 | 1.00 | late | Top-level app-client package contents | fs-only |
| 2.6 | 2259 | 1241 | -1018 | 1.00 | early | Top-level cli + crypto + docs package contents | fs-only |
| 3.1 | 2678 | 836 | -1842 | 1.00 | early | app-server/src — top-level entry layout | fs-only |
| 6.5 | 9990 | 5099 | -4891 | 1.00 | early | Docs site — page map (VitePress src layout) | fs-only |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 3 | 324 | README.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 545 | 1.00 | 545 | 8405 | export at packages/app-client/playwright.config.ts:12 |
| 254 | 1.00 | 254 | 3853 | export at packages/app-client/vite.config.ts:8 |
| 171 | 1.00 | 171 | 1599 | headings outline in CONTRIBUTING.md |
| 166 | 1.00 | 166 | 5068 | imports in packages/app-client/src/index.tsx |
| 165 | 0.85 | 195 | 2180 | headings outline in README.md |
| 137 | 1.00 | 137 | 7088 | export at packages/crypto/src/index.node.ts:8 |
| 137 | 1.00 | 137 | 7225 | export at packages/crypto/src/index.web.ts:8 |
| 114 | 1.00 | 114 | 9627 | README.md section #26 |
| 112 | 1.00 | 112 | 8672 | README.md section #24 |
| 112 | 1.00 | 112 | 4373 | headings outline in packages/app-client/README.md |
| 2239 | — | — | — | +32 more rows |

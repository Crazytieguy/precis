scores: Score(3000)=0.542 ns_rows≤3K=23/45 (reached=10 partial=0 missing=13)

## Per-budget scores

| B | A_B | I(B) | C(B) | Score(B) | walker_used |
|--:|----:|-----:|-----:|---------:|------------:|
| 1000 | 82 | 0.849 | 0.515 | 0.661 | 985 |
| 1442 | 98 | 0.834 | 0.431 | 0.599 | 1428 |
| 2080 | 126 | 0.837 | 0.507 | 0.652 | 1973 |
| 3000 | 277 | 0.746 | 0.394 | 0.542 | 2976 |
| 4327 | 379 | 0.723 | 0.325 | 0.485 | 4234 |
| 6240 | 572 | 0.714 | 0.271 | 0.440 | 6228 |
| 9000 | 809 | 0.681 | 0.196 | 0.365 | 8997 |

## Verdict

Verdict: wrong-slice bound
Likely primary lever: split walker batches to match NS semantic slices
Evidence: 7 ranking-recoverable (gap@3k=0.59), 14 wrong-slice/granularity (gap@3k=1.24), 11 no-discovered (gap@3k=0.67)
Secondary intervention: promote predecessors for 1 gated candidate
Top rows: 1.2, 2.3, 2.7, 3.4, 1.9, ...

## Top opportunities

_`gap@B` is a non-additive priority score: `Σ over atoms in row: (1 − damped_credit(a)) / rank(a)` evaluated at budget B's walker state. Approximates how much closing the row would lift `Score(B)` (via the Importance numerator); not an exact delta. `gap@3k` is the primary sort key. `gap@1k` and `gap@9k` show how the same intervention scales across the budget vector — gap is monotone non-increasing in B (walker has more budget at higher B). Rows can overlap between opportunities; sums are upper bounds on Score(B) impact, not additive estimates._

| intervention | rows | gap@1k | gap@3k | gap@9k | evidence | top row ids |
|:-------------|-----:|-------:|-------:|-------:|:---------|:------------|
| split wrong-slice walker batches | 14 | 1.33 | 1.24 | 1.04 | nearby candidates have low exact atom overlap | 1.2, 2.3, 2.7, 3.4, 1.9, ... |
| add walker candidates for no-discovered rows | 11 | 0.67 | 0.67 | 0.67 | NS rows have no discovered line candidate | 3.7, 3.6, 5.2, 5.1, 3.5, ... |
| tune ranking for high-overlap unscheduled candidates | 6 | 0.56 | 0.56 | 0.56 | high-overlap candidates not in the schedule by T_max, exact total=95/97 | 1.7, 1.11, 4.2, 1.4, 1.12, ... |
| promote package scripts in packages/lib/package.json | 1 | 0.04 | 0.04 | 0.04 | 0 files, exact total=8/8 | 2.8 |

## Diagnosis rollup

| diagnosis | rows | missing | partial | likely lever |
|:----------|-----:|--------:|--------:|:-------------|
| ranking-recoverable | 7 | 7 | 0 | value/ranking |
| wrong-slice / granularity | 14 | 14 | 0 | walker granularity / wrong slice |
| no discovered candidate | 11 | 11 | 0 | walker coverage or predecessor-gated emit |
| fs/listing | 3 | 3 | 0 | filesystem/listing value |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | gap@3k | likely lever |
|:------------|-----:|-------:|:-------------|
| predecessor not scheduled | 1 | 0.04 | promote predecessor |
| too expensive at final margin | 6 | 0.56 | tune ranking |

Candidate hint kinds: scheduled bbox=11, unscheduled bbox=6, scheduled same-file=2, unscheduled same-file=2, fs-only=3, no discovered candidate=11 _(candidates are walker batches discovered this run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` isn't proof that no emit path exists)._

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | missing | low | 11 |
| unscheduled bbox | missing | high | 1 |
| unscheduled bbox | missing | full | 5 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 1.4 | 168 | 0.00 | 0.00 | missing | pnpm workspace glob | [unscheduled bbox exact=2/2] plaintext config pnpm-workspace.yaml (2 atoms, too expensive at final margin) |
| 1.7 | 497 | 0.00 | 0.00 | missing | pnpm-workspace.yaml — catalog versions | [unscheduled bbox exact=9/9] plaintext config pnpm-workspace.yaml (9 atoms, too expensive at final margin) |
| 1.11 | 1499 | 0.20 | 0.02 | missing | README — how-it-works (creator side) | [scheduled bbox exact=2/10] headings outline in README.md (t=2168, 2 atoms); better unscheduled exact=9/10: README.md section #17 (9 atoms, too expensive at final margin) |
| 1.12 | 1700 | 0.00 | 0.00 | missing | README — how-it-works (recipient side) | [unscheduled bbox exact=6/7] README.md section #17 (6 atoms, too expensive at final margin) |
| 2.8 | 2657 | 0.00 | 0.00 | missing | lib package.json — runtime dependencies | [unscheduled bbox exact=8/8] package dependencies in packages/lib/package.json (8 atoms, predecessor not scheduled: package scripts in packages/lib/package.json) |
| 4.1 | 5055 | 0.00 | 0.00 | missing | Config — env var name catalog (locations only) | [unscheduled bbox exact=26/26] export at packages/app-server/src/modules/app/config/config.ts:5 (242 atoms, too expensive at final margin) |
| 4.2 | 5587 | 0.00 | 0.00 | missing | Config — defaults for the most-asked-about env vars | [unscheduled bbox exact=43/43] export at packages/app-server/src/modules/app/config/config.ts:5 (43 atoms, too expensive at final margin) |

### wrong-slice / granularity

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 1.2 | 127 | 0.00 | 0.00 | missing | README h1 + tagline | [scheduled same-file] headings outline in README.md (t=2168, 38 atoms) |
| 1.9 | 872 | 0.20 | 0.04 | missing | README — project structure (per-package one-liners) | [scheduled bbox exact=2/10] headings outline in README.md (t=2168, 2 atoms); better unscheduled exact=7/10: README.md section #23 (7 atoms, too expensive at final margin) |
| 1.10 | 1225 | 0.75 | 0.61 | missing | README — features list | [scheduled bbox exact=2/16] headings outline in README.md (t=2168, 2 atoms) |
| 2.3 | 2101 | 0.68 | 0.35 | missing | @enclosed/lib public API — index.ts re-exports | [scheduled bbox exact=19/28] export at packages/lib/src/index.ts:10 (t=601, 19 atoms) |
| 2.7 | 2545 | 0.19 | 0.40 | missing | @enclosed/lib README — install + usage example | [scheduled bbox exact=4/31] headings outline in packages/lib/README.md (t=1212, 4 atoms); better unscheduled exact=11/31: packages/lib/README.md section #2 (11 atoms, too expensive at final margin) |
| 3.3 | 2947 | 0.12 | 0.07 | missing | Cloudflare Workers entrypoint (full) | [scheduled bbox exact=1/8] export at packages/app-server/src/index.cloudflare.ts:8 (t=844, 1 atoms); better unscheduled exact=2/8: imports in packages/app-server/src/index.cloudflare.ts (2 atoms, discovered unscheduled) |
| 3.4 | 3451 | 0.00 | 0.00 | missing | Hono createServer factory + middleware stack | [scheduled bbox exact=2/41] export names surface in packages/app-server/src/modules/app/server.ts (t=5613, 2 atoms); better unscheduled exact=15/41: imports in packages/app-server/src/modules/app/server.ts (15 atoms, too expensive at final margin) |
| 4.3 | 5731 | 0.00 | 0.00 | missing | Config — getConfig export + figue setup | [scheduled bbox exact=5/14] export body at packages/app-server/src/modules/app/config/config.ts:258 body 259 (t=8780, 5 atoms) |
| 4.4 | 5875 | 0.00 | 0.00 | missing | Node.js entrypoint — config + storage + server build | [unscheduled same-file] imports in packages/app-server/src/index.node.ts (14 atoms, too expensive at final margin) |
| 4.5 | 6202 | 0.00 | 0.00 | missing | Node.js entrypoint — static + SPA fallback + cron + listen | [unscheduled same-file] imports in packages/app-server/src/index.node.ts (14 atoms, too expensive at final margin) |
| 6.1 | 9134 | 0.00 | 0.00 | missing | Lib — encryptNote body (crypto.usecases) | [scheduled same-file] export names surface in packages/lib/src/crypto/crypto.usecases.ts (t=3865, 2 atoms) |
| 6.2 | 9232 | 0.00 | 0.00 | missing | Lib — note URL hash-fragment markers (`pw` / `dar`) | [scheduled bbox exact=2/7] export names surface in packages/lib/src/notes/notes.models.ts (t=4095, 2 atoms) |
| 6.3 | 9539 | 0.00 | 0.00 | missing | CLI dispatcher + create-note args | [scheduled bbox exact=2/30] export names surface in packages/cli/src/create-note/create-note.command.ts (t=3062, 2 atoms); better unscheduled exact=12/30: export at packages/cli/src/create-note/create-note.command.ts:13 (12 atoms, too expensive at final margin) |
| 6.4 | 9903 | 0.00 | 0.00 | missing | App-client — Solid Router routes | [scheduled bbox exact=2/32] export at packages/app-client/src/routes.tsx:11 (t=5056, 2 atoms); better unscheduled exact=19/32: export body at packages/app-client/src/routes.tsx:11 body 12 (19 atoms, too expensive at final margin) |

### no discovered candidate

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 3.5 | 3749 | 0.00 | 0.00 | missing | Notes routes — endpoint registrations (locations only) | no discovered line candidate |
| 3.6 | 4200 | 0.00 | 0.00 | missing | Notes routes — POST /api/notes payload schema + validation handler | no discovered line candidate |
| 3.7 | 4659 | 0.00 | 0.00 | missing | Notes routes — GET /api/notes/:noteId + private-note auth gating | no discovered line candidate |
| 3.8 | 4821 | 0.00 | 0.00 | missing | Notes routes — GET /api/notes/:noteId/exists | no discovered line candidate |
| 4.6 | 6457 | 0.00 | 0.00 | missing | Storage drivers — memory + fs-lite | no discovered line candidate |
| 4.7 | 6892 | 0.00 | 0.00 | missing | Storage driver — Cloudflare KV (with 413 translation) | no discovered line candidate |
| 5.1 | 7406 | 0.00 | 0.00 | missing | Notes — models + types + constants | no discovered line candidate |
| 5.2 | 7835 | 0.00 | 0.00 | missing | Notes repository — exports + factory + getRefreshedNote usecase | no discovered line candidate |
| 5.3 | 8111 | 0.00 | 0.00 | missing | Auth catalog — errors + models + services signatures | no discovered line candidate |
| 5.4 | 8429 | 0.00 | 0.00 | missing | Auth middleware — gating + protected-route guard | no discovered line candidate |
| 5.5 | 8821 | 0.00 | 0.00 | missing | Auth login route — handler body (timing-safe bcrypt + JWT issue) | no discovered line candidate |

### fs/listing

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 2.5 | 2187 | 0.00 | 0.00 | missing | Top-level app-client package contents | fs-only |
| 3.2 | 2851 | 0.23 | 0.23 | missing | app-server/src/modules — module map | fs-only |
| 6.5 | 9990 | 0.68 | 0.68 | missing | Docs site — page map (VitePress src layout) | fs-only |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 3 | 324 | README.md section #<n> |

## Walker waste, primary-actionable (first_t ≤ 3000, off-NS spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 171 | 1.00 | 171 | 1599 | headings outline in CONTRIBUTING.md |
| 165 | 0.85 | 195 | 2168 | headings outline in README.md |
| 98 | 1.00 | 98 | 2266 | README.md section #0 |
| 92 | 1.00 | 92 | 1172 | export at packages/crypto/build.config.ts:3 |
| 81 | 1.00 | 81 | 815 | export at packages/lib/build.config.ts:3 |
| 67 | 1.00 | 67 | 2560 | export at packages/docs/src/data/i18n.data.ts:59 |
| 64 | 1.00 | 64 | 1049 | README headline in packages/crypto/README.md |
| 62 | 1.00 | 62 | 1370 | export at packages/cli/build.config.ts:3 |
| 50 | 1.00 | 50 | 1308 | README headline in packages/cli/README.md |

## Walker waste, late (first_t > 3000, off-NS spend ≥ 50) — higher-budget calibration only

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 545 | 1.00 | 545 | 8393 | export at packages/app-client/playwright.config.ts:12 |
| 254 | 1.00 | 254 | 3826 | export at packages/app-client/vite.config.ts:8 |
| 166 | 1.00 | 166 | 5041 | imports in packages/app-client/src/index.tsx |
| 137 | 1.00 | 137 | 7076 | export at packages/crypto/src/index.node.ts:8 |
| 137 | 1.00 | 137 | 7213 | export at packages/crypto/src/index.web.ts:8 |
| 114 | 1.00 | 114 | 9615 | README.md section #26 |
| 112 | 1.00 | 112 | 8660 | README.md section #24 |
| 112 | 1.00 | 112 | 4346 | headings outline in packages/app-client/README.md |
| 104 | 1.00 | 104 | 9346 | headings outline in packages/docs/src/resources/brand-kit.md |
| 102 | 1.00 | 102 | 3967 | headings outline in packages/cli/README.md |
| 1519 | — | — | — | +23 more rows |

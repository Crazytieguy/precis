scores: Score(3000)=0.542 ns_rows≤3K=23/45 (reached=10 partial=0 missing=13)

## Per-budget scores

| B | A_B | I(B) | C(B) | compl(B) | Score(B) | walker_used |
|--:|----:|-----:|-----:|---------:|---------:|------------:|
| 1000 | 82 | 0.849 | 0.515 | 0.892 | 0.661 | 985 |
| 1442 | 98 | 0.834 | 0.431 | 0.892 | 0.599 | 1428 |
| 2080 | 126 | 0.837 | 0.507 | 0.999 | 0.652 | 1973 |
| 3000 | 277 | 0.746 | 0.394 | 0.688 | 0.542 | 2994 |
| 4327 | 379 | 0.723 | 0.325 | 0.705 | 0.485 | 4234 |
| 6240 | 572 | 0.714 | 0.271 | 0.729 | 0.440 | 6209 |
| 9000 | 809 | 0.681 | 0.196 | 0.720 | 0.365 | 8972 |

## Top opportunities

### Additive (close partial / missing rows)

| intervention | rows | gap@1k | gap@3k | gap@9k | evidence | top row ids |
|:-------------|-----:|-------:|-------:|-------:|:---------|:------------|
| split wrong-slice walker batches | 14 | 1.33 | 1.24 | 1.04 | nearby candidates have low exact atom overlap | 1.2, 2.3, 2.7, 3.4, 1.9, ... |
| add walker candidates for no-discovered rows | 11 | 0.67 | 0.67 | 0.67 | NS rows have no discovered line candidate | 3.7, 3.6, 5.2, 5.1, 3.5, ... |
| tune ranking for high-overlap unscheduled candidates | 4 | 0.30 | 0.30 | 0.30 | high-overlap candidates not in the schedule by T_max, exact total=84/86 | 1.11, 4.2, 1.12, 4.1 |
| promote package scripts in packages/lib/package.json | 1 | 0.04 | 0.04 | 0.04 | 0 files, exact total=8/8 | 2.8 |

### Subtractive (suppress consistently off-NS batches)

| pattern | batches | freed@1k | freed@3k | freed@9k | evidence | top batch ids |
|:--------|--------:|---------:|---------:|---------:|:---------|:--------------|
| headings outline in CONTRIBUTING.md | 1 | 0 | 171 | 171 | off_3k=171 | headings outline in CONTRIBUTING.md |
| headings outline in README.md | 1 | 0 | 165 | 165 | off_3k=165 | headings outline in README.md |
| README.md section #<n> | 1 | 0 | 98 | 210 | off_3k=98 | README.md section #0 |
| export at packages/crypto/build.config.ts:<n> | 1 | 0 | 92 | 92 | off_3k=92 | export at packages/crypto/build.config.ts:3 |
| export at packages/lib/build.config.ts:<n> | 1 | 81 | 81 | 81 | off_3k=81 | export at packages/lib/build.config.ts:3 |

Top missed paths (NS rows ≤ 3K): README.md (5 rows, 49 atoms), packages/app-server/src/modules/notes (1 row, 35 atoms), packages/lib/README.md (1 row, 31 atoms), packages/lib/src/index.ts (1 row, 28 atoms), packages/app-client (1 row, 14 atoms), pnpm-workspace.yaml (2 rows, 11 atoms), packages/app-server/src/index.cloudflare.ts (1 row, 8 atoms), packages/lib/package.json (1 row, 8 atoms)

## Arrival ledger by diagnosis

### ranking-recoverable

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 1.11 | 1499 | 0.20 | missing | README — how-it-works (creator side) | [scheduled bbox exact=2/10] headings outline in README.md (t=2168, 2 atoms); better unscheduled exact=9/10: README.md section #17 (9 atoms, too expensive at final margin) |
| 1.12 | 1700 | 0.00 | missing | README — how-it-works (recipient side) | [unscheduled bbox exact=6/7] README.md section #17 (6 atoms, too expensive at final margin) |
| 2.8 | 2657 | 0.00 | missing | lib package.json — runtime dependencies | [unscheduled bbox exact=8/8] package dependencies in packages/lib/package.json (8 atoms, predecessor not scheduled: package scripts in packages/lib/package.json) |
| 4.1 | 5055 | 0.00 | missing | Config — env var name catalog (locations only) | [unscheduled bbox exact=26/26] export at packages/app-server/src/modules/app/config/config.ts:5 (242 atoms, too expensive at final margin) |
| 4.2 | 5587 | 0.00 | missing | Config — defaults for the most-asked-about env vars | [unscheduled bbox exact=43/43] export at packages/app-server/src/modules/app/config/config.ts:5 (43 atoms, too expensive at final margin) |

### wrong-slice / granularity

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 1.2 | 127 | 0.00 | missing | README h1 + tagline | [scheduled same-file] headings outline in README.md (t=2168, 38 atoms) |
| 1.9 | 872 | 0.20 | missing | README — project structure (per-package one-liners) | [scheduled bbox exact=2/10] headings outline in README.md (t=2168, 2 atoms); better unscheduled exact=7/10: README.md section #23 (7 atoms, too expensive at final margin) |
| 1.10 | 1225 | 0.75 | missing | README — features list | [scheduled bbox exact=2/16] headings outline in README.md (t=2168, 2 atoms) |
| 2.3 | 2101 | 0.68 | missing | @enclosed/lib public API — index.ts re-exports | [scheduled bbox exact=19/28] export at packages/lib/src/index.ts:10 (t=601, 19 atoms) |
| 2.7 | 2545 | 0.19 | missing | @enclosed/lib README — install + usage example | [scheduled bbox exact=4/31] headings outline in packages/lib/README.md (t=1212, 4 atoms); better unscheduled exact=11/31: packages/lib/README.md section #2 (11 atoms, too expensive at final margin) |
| 3.3 | 2947 | 0.12 | missing | Cloudflare Workers entrypoint (full) | [scheduled bbox exact=1/8] export at packages/app-server/src/index.cloudflare.ts:8 (t=844, 1 atoms); better unscheduled exact=2/8: imports in packages/app-server/src/index.cloudflare.ts (2 atoms, too expensive at final margin) |
| 3.4 | 3451 | 0.00 | missing | Hono createServer factory + middleware stack | [scheduled bbox exact=2/41] export names surface in packages/app-server/src/modules/app/server.ts (t=5542, 2 atoms); better unscheduled exact=15/41: imports in packages/app-server/src/modules/app/server.ts (15 atoms, too expensive at final margin) |
| 4.3 | 5731 | 0.00 | missing | Config — getConfig export + figue setup | [scheduled bbox exact=5/14] export body at packages/app-server/src/modules/app/config/config.ts:258 body 259 (t=8510, 5 atoms) |
| 4.4 | 5875 | 0.00 | missing | Node.js entrypoint — config + storage + server build | [unscheduled same-file] imports in packages/app-server/src/index.node.ts (14 atoms, too expensive at final margin) |
| 4.5 | 6202 | 0.00 | missing | Node.js entrypoint — static + SPA fallback + cron + listen | [unscheduled same-file] imports in packages/app-server/src/index.node.ts (14 atoms, too expensive at final margin) |
| 6.1 | 9134 | 0.00 | missing | Lib — encryptNote body (crypto.usecases) | [scheduled same-file] export names surface in packages/lib/src/crypto/crypto.usecases.ts (t=3865, 2 atoms) |
| 6.2 | 9232 | 0.00 | missing | Lib — note URL hash-fragment markers (`pw` / `dar`) | [scheduled bbox exact=2/7] export names surface in packages/lib/src/notes/notes.models.ts (t=4095, 2 atoms) |
| 6.3 | 9539 | 0.00 | missing | CLI dispatcher + create-note args | [scheduled bbox exact=2/30] export names surface in packages/cli/src/create-note/create-note.command.ts (t=3043, 2 atoms); better unscheduled exact=12/30: export at packages/cli/src/create-note/create-note.command.ts:13 (12 atoms, too expensive at final margin) |
| 6.4 | 9903 | 0.00 | missing | App-client — Solid Router routes | [scheduled bbox exact=2/32] export at packages/app-client/src/routes.tsx:11 (t=5056, 2 atoms); better unscheduled exact=19/32: export body at packages/app-client/src/routes.tsx:11 body 12 (19 atoms, too expensive at final margin) |

### no discovered candidate

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 3.5 | 3749 | 0.00 | missing | Notes routes — endpoint registrations (locations only) | no discovered line candidate |
| 3.6 | 4200 | 0.00 | missing | Notes routes — POST /api/notes payload schema + validation handler | no discovered line candidate |
| 3.7 | 4659 | 0.00 | missing | Notes routes — GET /api/notes/:noteId + private-note auth gating | no discovered line candidate |
| 3.8 | 4821 | 0.00 | missing | Notes routes — GET /api/notes/:noteId/exists | no discovered line candidate |
| 4.6 | 6457 | 0.00 | missing | Storage drivers — memory + fs-lite | no discovered line candidate |
| 4.7 | 6892 | 0.00 | missing | Storage driver — Cloudflare KV (with 413 translation) | no discovered line candidate |
| 5.1 | 7406 | 0.00 | missing | Notes — models + types + constants | no discovered line candidate |
| 5.2 | 7835 | 0.00 | missing | Notes repository — exports + factory + getRefreshedNote usecase | no discovered line candidate |
| 5.3 | 8111 | 0.00 | missing | Auth catalog — errors + models + services signatures | no discovered line candidate |
| 5.4 | 8429 | 0.00 | missing | Auth middleware — gating + protected-route guard | no discovered line candidate |
| 5.5 | 8821 | 0.00 | missing | Auth login route — handler body (timing-safe bcrypt + JWT issue) | no discovered line candidate |

### fs/listing

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 2.5 | 2187 | 0.00 | missing | Top-level app-client package contents | fs-only |
| 3.2 | 2851 | 0.23 | missing | app-server/src/modules — module map | fs-only |
| 6.5 | 9990 | 0.68 | missing | Docs site — page map (VitePress src layout) | fs-only |

### mixed/unknown

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 1.4 | 168 | 0.00 | missing | pnpm workspace glob | [scheduled bbox exact=2/2] plaintext config pnpm-workspace.yaml (t=9992, 2 atoms) |
| 1.7 | 497 | 0.00 | missing | pnpm-workspace.yaml — catalog versions | [scheduled bbox exact=9/9] plaintext config pnpm-workspace.yaml (t=9992, 9 atoms) |

Top wasted paths (off-NS at 3K): README.md (263t, 2 batches), CONTRIBUTING.md (171t, 1 batch), packages/crypto/build.config.ts (92t, 1 batch), packages/lib/build.config.ts (81t, 1 batch), packages/docs/src/data/i18n.data.ts (67t, 1 batch), packages/crypto/README.md (64t, 1 batch), packages/cli/build.config.ts (62t, 1 batch), packages/cli/README.md (50t, 1 batch)

## Walker waste (walker_t ≤ 3000, off_3k ≥ 50)

| off_3k | off_3k_ratio | off_any | cost | walker_t | batch |
|-------:|-------------:|--------:|-----:|---------:|:------|
| 171 | 1.00 | 171 | 171 | 1428 | headings outline in CONTRIBUTING.md |
| 165 | 0.85 | 165 | 195 | 1973 | headings outline in README.md |
| 98 | 1.00 | 98 | 98 | 2168 | README.md section #0 |
| 92 | 1.00 | 92 | 92 | 1080 | export at packages/crypto/build.config.ts:3 |
| 81 | 1.00 | 81 | 81 | 734 | export at packages/lib/build.config.ts:3 |
| 67 | 1.00 | 67 | 67 | 2465 | export at packages/docs/src/data/i18n.data.ts:59 |
| 64 | 1.00 | 64 | 64 | 985 | README headline in packages/crypto/README.md |
| 62 | 1.00 | 62 | 62 | 1308 | export at packages/cli/build.config.ts:3 |
| 50 | 1.00 | 50 | 50 | 1258 | README headline in packages/cli/README.md |

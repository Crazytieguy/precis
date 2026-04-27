scores: Sim=0.517 Reached=13/45 Early=5 Late=5 Partial=1 Missing=31 Used=9858/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 12 | 5 | 0 | 7 | 0.44 |
| 2 | 8 | 6 | 0 | 2 | 0.77 |
| 3 | 8 | 1 | 1 | 6 | 0.21 |
| 4 | 7 | 0 | 0 | 7 | 0.00 |
| 5 | 5 | 0 | 0 | 5 | 0.00 |
| 6 | 5 | 1 | 0 | 4 | 0.27 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 127 | — | — | 0.00 | missing | README h1 + tagline |  |
| 1.4 | 168 | — | — | 0.00 | missing | pnpm workspace glob |  |
| 1.5 | 250 | 1814 | +1564 | 0.83 | late | Root package.json — name + version + license | package identity in package.json (t=332, 4 atoms) |
| 1.6 | 357 | 1814 | +1457 | 1.00 | late | Root package.json — author + repo + engines | package identity in package.json (t=332, 5 atoms) |
| 1.7 | 497 | — | — | 0.00 | missing | pnpm-workspace.yaml — catalog versions |  |
| 1.8 | 694 | 1814 | +1120 | 0.95 | late | Root package.json — scripts + keywords + devDeps | package identity in package.json (t=332, 10 atoms) |
| 1.9 | 872 | — | — | 0.20 | missing | README — project structure (per-package one-liners) | headings outline in README.md (t=2925, 2 atoms) |
| 1.10 | 1225 | — | — | 0.12 | missing | README — features list | headings outline in README.md (t=2925, 2 atoms) |
| 1.11 | 1499 | — | — | 0.20 | missing | README — how-it-works (creator side) | headings outline in README.md (t=2925, 2 atoms) |
| 1.12 | 1700 | — | — | 0.00 | missing | README — how-it-works (recipient side) |  |
| 2.1 | 1726 | 713 | -1013 | 1.00 | early | Top-level lib package contents |  |
| 2.2 | 1742 | 966 | -776 | 1.00 | early | lib/src — top-level source modules |  |
| 2.3 | 2101 | 7212 | +5111 | 0.96 | late | @enclosed/lib public API — index.ts re-exports | export at packages/lib/src/index.ts:10 (t=1138, 19 atoms) |
| 2.4 | 2124 | 529 | -1595 | 1.00 | early | Top-level app-server package contents |  |
| 2.5 | 2187 | 4397 | +2210 | 1.00 | late | Top-level app-client package contents |  |
| 2.6 | 2259 | 1212 | -1047 | 1.00 | early | Top-level cli + crypto + docs package contents |  |
| 2.7 | 2545 | — | — | 0.19 | missing | @enclosed/lib README — install + usage example | headings outline in packages/lib/README.md (t=1183, 4 atoms) |
| 2.8 | 2657 | — | — | 0.00 | missing | lib package.json — runtime dependencies |  |
| 3.2 | 2851 | — | — | 0.51 | partial | app-server/src/modules — module map |  |
| 3.3 | 2947 | — | — | 0.12 | missing | Cloudflare Workers entrypoint (full) | export names surface in packages/app-server/src/index.cloudflare.ts (t=3131, 1 atoms) |
| 3.4 | 3451 | — | — | 0.05 | missing | Hono createServer factory + middleware stack | export names surface in packages/app-server/src/modules/app/server.ts (t=7435, 2 atoms) |
| 3.5 | 3749 | — | — | 0.00 | missing | Notes routes — endpoint registrations (locations only) |  |
| 3.6 | 4200 | — | — | 0.00 | missing | Notes routes — POST /api/notes payload schema + validation handler |  |
| 3.7 | 4659 | — | — | 0.00 | missing | Notes routes — GET /api/notes/:noteId + private-note auth gating |  |
| 3.8 | 4821 | — | — | 0.00 | missing | Notes routes — GET /api/notes/:noteId/exists |  |
| 4.1 | 5055 | — | — | 0.00 | missing | Config — env var name catalog (locations only) |  |
| 4.2 | 5587 | — | — | 0.00 | missing | Config — defaults for the most-asked-about env vars |  |
| 4.3 | 5731 | — | — | 0.00 | missing | Config — getConfig export + figue setup |  |
| 4.4 | 5875 | — | — | 0.00 | missing | Node.js entrypoint — config + storage + server build |  |
| 4.5 | 6202 | — | — | 0.00 | missing | Node.js entrypoint — static + SPA fallback + cron + listen |  |
| 4.6 | 6457 | — | — | 0.00 | missing | Storage drivers — memory + fs-lite |  |
| 4.7 | 6892 | — | — | 0.00 | missing | Storage driver — Cloudflare KV (with 413 translation) |  |
| 5.1 | 7406 | — | — | 0.00 | missing | Notes — models + types + constants |  |
| 5.2 | 7835 | — | — | 0.00 | missing | Notes repository — exports + factory + getRefreshedNote usecase |  |
| 5.3 | 8111 | — | — | 0.00 | missing | Auth catalog — errors + models + services signatures |  |
| 5.4 | 8429 | — | — | 0.00 | missing | Auth middleware — gating + protected-route guard |  |
| 5.5 | 8821 | — | — | 0.00 | missing | Auth login route — handler body (timing-safe bcrypt + JWT issue) |  |
| 6.1 | 9134 | — | — | 0.00 | missing | Lib — encryptNote body (crypto.usecases) |  |
| 6.2 | 9232 | — | — | 0.29 | missing | Lib — note URL hash-fragment markers (`pw` / `dar`) | export names surface in packages/lib/src/notes/notes.models.ts (t=5347, 2 atoms) |
| 6.3 | 9539 | — | — | 0.04 | missing | CLI dispatcher + create-note args | export names surface in packages/cli/src/create-note/create-note.command.ts (t=4155, 2 atoms) |
| 6.4 | 9903 | — | — | 0.03 | missing | App-client — Solid Router routes | export names surface in packages/app-client/src/routes.tsx (t=4666, 2 atoms) |
| 6.5 | 9990 | 6721 | -3269 | 1.00 | early | Docs site — page map (VitePress src layout) |  |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 254 | 1.00 | 254 | 5078 | export at packages/app-client/vite.config.ts:8 |
| 200 | 1.00 | 200 | 8696 | package scripts in packages/app-client/package.json |
| 172 | 1.00 | 172 | 8898 | json config packages/app-server/tsconfig.json |
| 171 | 1.00 | 171 | 1512 | headings outline in CONTRIBUTING.md |
| 166 | 1.00 | 166 | 6420 | imports in packages/app-client/src/index.tsx |
| 165 | 0.85 | 195 | 2925 | headings outline in README.md |
| 153 | 1.00 | 153 | 2718 | package identity in packages/docs/package.json |
| 151 | 1.00 | 151 | 6950 | package scripts in packages/cli/package.json |
| 149 | 1.00 | 149 | 6690 | package scripts in packages/lib/package.json |
| 147 | 1.00 | 147 | 2457 | package identity in packages/cli/package.json |
| 146 | 1.00 | 146 | 2164 | package identity in packages/crypto/package.json |
| 146 | 1.00 | 146 | 2310 | package identity in packages/lib/package.json |
| 144 | 1.00 | 144 | 4621 | package identity in packages/app-client/package.json |
| 144 | 1.00 | 144 | 1958 | package identity in packages/app-server/package.json |
| 137 | 1.00 | 137 | 9242 | export at packages/crypto/src/index.node.ts:8 |
| 137 | 1.00 | 137 | 9379 | export at packages/crypto/src/index.web.ts:8 |
| 137 | 1.00 | 137 | 1685 | package identity in packages/deploy-cloudflare/package.json |
| 124 | 1.00 | 124 | 3391 | package entrypoints in packages/lib/package.json |
| 121 | 1.00 | 121 | 6541 | json config packages/crypto/tsconfig.json |
| 112 | 1.00 | 112 | 5647 | headings outline in packages/app-client/README.md |
| 109 | 1.00 | 109 | 5852 | json config packages/lib/tsconfig.json |
| 102 | 1.00 | 102 | 5219 | headings outline in packages/cli/README.md |
| 98 | 1.00 | 98 | 3023 | README.md section #0 |
| 92 | 1.00 | 92 | 950 | export at packages/crypto/build.config.ts:3 |
| 91 | 1.00 | 91 | 7709 | export at packages/lib/src/notes/notes.types.ts:17 |
| 87 | 1.00 | 87 | 8265 | export at packages/cli/src/create-note/create-note.usecases.ts:3 |
| 87 | 1.00 | 87 | 3538 | package scripts in packages/docs/package.json |
| 84 | 1.00 | 84 | 2541 | package entrypoints in packages/cli/package.json |
| 82 | 1.00 | 82 | 9488 | headings outline in packages/docs/src/self-hosting/other-platforms.md |
| 81 | 1.00 | 81 | 858 | export at packages/lib/build.config.ts:3 |
| 81 | 1.00 | 81 | 5535 | listing of '.github/workflows' |
| 74 | 1.00 | 74 | 9105 | export names surface in packages/app-server/src/modules/storage/storage.types.ts |
| 74 | 1.00 | 74 | 7839 | export names surface in packages/cli/src/files/files.services.ts |
| 72 | 1.00 | 72 | 9753 | export at packages/crypto/src/encryption-algorithms/encryption-algorithms.types.ts:3 |
| 71 | 1.00 | 71 | 7312 | export names surface in packages/lib/src/crypto/crypto.types.ts |
| 69 | 1.00 | 69 | 5716 | packages/app-client/README.md section #0 |
| 67 | 1.00 | 67 | 3775 | export at packages/docs/src/data/i18n.data.ts:59 |
| 64 | 1.00 | 64 | 656 | README headline in packages/crypto/README.md |
| 64 | 1.00 | 64 | 8378 | listing of 'packages/app-client/public' |
| 62 | 1.00 | 62 | 1341 | export at packages/cli/build.config.ts:3 |
| 60 | 1.00 | 60 | 2018 | json config renovate.json |
| 56 | 1.00 | 56 | 7765 | headings outline in packages/docs/src/self-hosting/configuration.md |
| 52 | 1.00 | 52 | 9681 | export names surface in packages/crypto/src/encryption-algorithms/encryption-algorithms.types.ts |
| 51 | 1.00 | 51 | 7890 | export body at packages/cli/src/files/files.services.ts:5 |
| 51 | 1.00 | 51 | 9014 | export names surface in packages/app-client/src/modules/shared/utils/copy.tsx |
| 51 | 1.00 | 51 | 6799 | headings outline in packages/docs/src/integrations/npm-package.md |
| 50 | 1.00 | 50 | 7000 | CONTRIBUTING.md section #1 |
| 50 | 1.00 | 50 | 1279 | README headline in packages/cli/README.md |
| 50 | 1.00 | 50 | 9538 | listing of 'packages/crypto/src/encryption-algorithms' |

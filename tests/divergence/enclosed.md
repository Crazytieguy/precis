scores: Sim=0.516 Reached=15/45 Early=4 Late=7 Partial=2 Missing=28 Used=9972/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 12 | 7 | 0 | 5 | 0.56 |
| 2 | 8 | 6 | 1 | 1 | 0.84 |
| 3 | 8 | 1 | 1 | 6 | 0.21 |
| 4 | 7 | 0 | 0 | 7 | 0.00 |
| 5 | 5 | 0 | 0 | 5 | 0.00 |
| 6 | 5 | 1 | 0 | 4 | 0.27 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 127 | — | — | 0.00 | missing | README h1 + tagline |  |
| 1.4 | 168 | — | — | 0.00 | missing | pnpm workspace glob |  |
| 1.5 | 250 | 1940 | +1690 | 0.83 | late | Root package.json — name + version + license | package identity in package.json (t=332, 4 atoms) |
| 1.6 | 357 | 1940 | +1583 | 1.00 | late | Root package.json — author + repo + engines | package identity in package.json (t=332, 5 atoms) |
| 1.7 | 497 | — | — | 0.00 | missing | pnpm-workspace.yaml — catalog versions |  |
| 1.8 | 694 | 1940 | +1246 | 0.95 | late | Root package.json — scripts + keywords + devDeps | package identity in package.json (t=332, 10 atoms) |
| 1.9 | 872 | 4429 | +3557 | 0.80 | late | README — project structure (per-package one-liners) | README.md section #11 (t=4429, 7 atoms) |
| 1.10 | 1225 | 6355 | +5130 | 0.94 | late | README — features list | README.md section #1 (t=6355, 14 atoms) |
| 1.11 | 1499 | — | — | 0.20 | missing | README — how-it-works (creator side) | headings outline in README.md (t=3081, 2 atoms) |
| 1.12 | 1700 | — | — | 0.00 | missing | README — how-it-works (recipient side) |  |
| 2.1 | 1726 | 736 | -990 | 1.00 | early | Top-level lib package contents |  |
| 2.2 | 1742 | 1019 | -723 | 1.00 | early | lib/src — top-level source modules |  |
| 2.3 | 2101 | 8299 | +6198 | 0.96 | late | @enclosed/lib public API — index.ts re-exports | export at packages/lib/src/index.ts:10 (t=1191, 19 atoms) |
| 2.4 | 2124 | 529 | -1595 | 1.00 | early | Top-level app-server package contents |  |
| 2.5 | 2187 | 4987 | +2800 | 1.00 | late | Top-level app-client package contents |  |
| 2.6 | 2259 | 1265 | -994 | 1.00 | early | Top-level cli + crypto + docs package contents |  |
| 2.7 | 2545 | — | — | 0.77 | partial | @enclosed/lib README — install + usage example | packages/lib/README.md section #2 (t=9423, 11 atoms) |
| 2.8 | 2657 | — | — | 0.00 | missing | lib package.json — runtime dependencies |  |
| 3.2 | 2851 | — | — | 0.51 | partial | app-server/src/modules — module map |  |
| 3.3 | 2947 | — | — | 0.12 | missing | Cloudflare Workers entrypoint (full) | export names surface in packages/app-server/src/index.cloudflare.ts (t=3287, 1 atoms) |
| 3.4 | 3451 | — | — | 0.05 | missing | Hono createServer factory + middleware stack | export names surface in packages/app-server/src/modules/app/server.ts (t=8670, 2 atoms) |
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
| 6.2 | 9232 | — | — | 0.29 | missing | Lib — note URL hash-fragment markers (`pw` / `dar`) | export names surface in packages/lib/src/notes/notes.models.ts (t=6435, 2 atoms) |
| 6.3 | 9539 | — | — | 0.04 | missing | CLI dispatcher + create-note args | export names surface in packages/cli/src/create-note/create-note.command.ts (t=4745, 2 atoms) |
| 6.4 | 9903 | — | — | 0.03 | missing | App-client — Solid Router routes | export names surface in packages/app-client/src/routes.tsx (t=5256, 2 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 3 | 324 | README.md section #<n> |
| 2 | 208 | packages/app-client/README.md section #<n> |
| 2 | 124 | CONTRIBUTING.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 254 | 1.00 | 254 | 5719 | export at packages/app-client/vite.config.ts:8 |
| 171 | 1.00 | 171 | 1588 | headings outline in CONTRIBUTING.md |
| 166 | 1.00 | 166 | 7552 | imports in packages/app-client/src/index.tsx |
| 165 | 0.85 | 195 | 3081 | headings outline in README.md |
| 153 | 1.00 | 153 | 2874 | package identity in packages/docs/package.json |
| 151 | 1.00 | 151 | 8082 | package scripts in packages/cli/package.json |
| 149 | 1.00 | 149 | 7822 | package scripts in packages/lib/package.json |
| 148 | 1.00 | 148 | 8447 | packages/cli/README.md section #1 |
| 147 | 1.00 | 147 | 2583 | package identity in packages/cli/package.json |
| 146 | 1.00 | 146 | 2290 | package identity in packages/crypto/package.json |
| 146 | 1.00 | 146 | 2436 | package identity in packages/lib/package.json |
| 144 | 1.00 | 144 | 5211 | package identity in packages/app-client/package.json |
| 144 | 1.00 | 144 | 2084 | package identity in packages/app-server/package.json |
| 139 | 1.00 | 139 | 9972 | packages/app-client/README.md section #1 |
| 137 | 1.00 | 137 | 1811 | package identity in packages/deploy-cloudflare/package.json |
| 124 | 1.00 | 124 | 3547 | package entrypoints in packages/lib/package.json |
| 121 | 1.00 | 121 | 7673 | json config packages/crypto/tsconfig.json |
| 114 | 1.00 | 114 | 3864 | README.md section #14 |
| 112 | 1.00 | 112 | 3659 | README.md section #12 |
| 112 | 1.00 | 112 | 6735 | headings outline in packages/app-client/README.md |
| 109 | 1.00 | 109 | 6940 | json config packages/lib/tsconfig.json |
| 102 | 1.00 | 102 | 5860 | headings outline in packages/cli/README.md |
| 98 | 1.00 | 98 | 3258 | README.md section #0 |
| 92 | 1.00 | 92 | 1003 | export at packages/crypto/build.config.ts:3 |
| 91 | 1.00 | 91 | 8944 | export at packages/lib/src/notes/notes.types.ts:17 |
| 87 | 1.00 | 87 | 9602 | export at packages/cli/src/create-note/create-note.usecases.ts:3 |
| 87 | 1.00 | 87 | 3996 | package scripts in packages/docs/package.json |
| 84 | 1.00 | 84 | 2667 | package entrypoints in packages/cli/package.json |
| 81 | 1.00 | 81 | 881 | export at packages/lib/build.config.ts:3 |
| 81 | 1.00 | 81 | 6623 | listing of '.github/workflows' |
| 76 | 1.00 | 76 | 3750 | packages/crypto/README.md section #1 |
| 74 | 1.00 | 74 | 7014 | CONTRIBUTING.md section #3 |
| 74 | 1.00 | 74 | 9074 | export names surface in packages/cli/src/files/files.services.ts |
| 71 | 1.00 | 71 | 8547 | export names surface in packages/lib/src/crypto/crypto.types.ts |
| 69 | 1.00 | 69 | 6831 | packages/app-client/README.md section #0 |
| 67 | 1.00 | 67 | 4233 | export at packages/docs/src/data/i18n.data.ts:59 |
| 64 | 1.00 | 64 | 656 | README headline in packages/crypto/README.md |
| 64 | 1.00 | 64 | 9715 | listing of 'packages/app-client/public' |
| 62 | 1.00 | 62 | 1394 | export at packages/cli/build.config.ts:3 |
| 60 | 1.00 | 60 | 2144 | json config renovate.json |
| 56 | 1.00 | 56 | 9000 | headings outline in packages/docs/src/self-hosting/configuration.md |
| 51 | 1.00 | 51 | 9125 | export body at packages/cli/src/files/files.services.ts:5 |
| 51 | 1.00 | 51 | 7931 | headings outline in packages/docs/src/integrations/npm-package.md |
| 50 | 1.00 | 50 | 1674 | CONTRIBUTING.md section #1 |
| 50 | 1.00 | 50 | 1332 | README headline in packages/cli/README.md |

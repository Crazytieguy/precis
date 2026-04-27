scores: Sim=0.534 Reached=13/45 Early=5 Late=5 Partial=1 Missing=31 Used=9894/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 12 | 5 | 0 | 7 | 0.44 |
| 2 | 8 | 6 | 0 | 2 | 0.77 |
| 3 | 8 | 1 | 1 | 6 | 0.21 |
| 4 | 7 | 0 | 0 | 7 | 0.06 |
| 5 | 5 | 0 | 0 | 5 | 0.00 |
| 6 | 5 | 1 | 0 | 4 | 0.27 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 127 | — | — | 0.00 | missing | README h1 + tagline |  |
| 1.4 | 168 | — | — | 0.00 | missing | pnpm workspace glob |  |
| 1.5 | 250 | 1631 | +1381 | 0.83 | late | Root package.json — name + version + license | package identity in package.json (t=332, 4 atoms) |
| 1.6 | 357 | 1631 | +1274 | 1.00 | late | Root package.json — author + repo + engines | package identity in package.json (t=332, 5 atoms) |
| 1.7 | 497 | — | — | 0.00 | missing | pnpm-workspace.yaml — catalog versions |  |
| 1.8 | 694 | 1631 | +937 | 0.95 | late | Root package.json — scripts + keywords + devDeps | package identity in package.json (t=332, 10 atoms) |
| 1.9 | 872 | — | — | 0.20 | missing | README — project structure (per-package one-liners) | headings outline in README.md (t=1850, 2 atoms) |
| 1.10 | 1225 | — | — | 0.12 | missing | README — features list | headings outline in README.md (t=1850, 2 atoms) |
| 1.11 | 1499 | — | — | 0.20 | missing | README — how-it-works (creator side) | headings outline in README.md (t=1850, 2 atoms) |
| 1.12 | 1700 | — | — | 0.00 | missing | README — how-it-works (recipient side) |  |
| 2.1 | 1726 | 713 | -1013 | 1.00 | early | Top-level lib package contents |  |
| 2.2 | 1742 | 966 | -776 | 1.00 | early | lib/src — top-level source modules |  |
| 2.3 | 2101 | 5252 | +3151 | 0.96 | late | @enclosed/lib public API — index.ts re-exports | export at packages/lib/src/index.ts:10 (t=1138, 19 atoms) |
| 2.4 | 2124 | 451 | -1673 | 1.00 | early | Top-level app-server package contents |  |
| 2.5 | 2187 | 3111 | +924 | 1.00 | late | Top-level app-client package contents |  |
| 2.6 | 2259 | 1212 | -1047 | 1.00 | early | Top-level cli + crypto + docs package contents |  |
| 2.7 | 2545 | — | — | 0.19 | missing | @enclosed/lib README — install + usage example | headings outline in packages/lib/README.md (t=1183, 4 atoms) |
| 2.8 | 2657 | — | — | 0.00 | missing | lib package.json — runtime dependencies |  |
| 3.2 | 2851 | — | — | 0.51 | partial | app-server/src/modules — module map |  |
| 3.3 | 2947 | — | — | 0.12 | missing | Cloudflare Workers entrypoint (full) | export names surface in packages/app-server/src/index.cloudflare.ts (t=2056, 1 atoms) |
| 3.4 | 3451 | — | — | 0.05 | missing | Hono createServer factory + middleware stack | export names surface in packages/app-server/src/modules/app/server.ts (t=5475, 2 atoms) |
| 3.5 | 3749 | — | — | 0.00 | missing | Notes routes — endpoint registrations (locations only) |  |
| 3.6 | 4200 | — | — | 0.00 | missing | Notes routes — POST /api/notes payload schema + validation handler |  |
| 3.7 | 4659 | — | — | 0.00 | missing | Notes routes — GET /api/notes/:noteId + private-note auth gating |  |
| 3.8 | 4821 | — | — | 0.00 | missing | Notes routes — GET /api/notes/:noteId/exists |  |
| 4.1 | 5055 | — | — | 0.00 | missing | Config — env var name catalog (locations only) |  |
| 4.2 | 5587 | — | — | 0.00 | missing | Config — defaults for the most-asked-about env vars |  |
| 4.3 | 5731 | — | — | 0.43 | missing | Config — getConfig export + figue setup | export body at packages/app-server/src/modules/app/config/config.ts:258 (t=8666, 5 atoms) |
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
| 6.2 | 9232 | — | — | 0.29 | missing | Lib — note URL hash-fragment markers (`pw` / `dar`) | export names surface in packages/lib/src/notes/notes.models.ts (t=3917, 2 atoms) |
| 6.3 | 9539 | — | — | 0.04 | missing | CLI dispatcher + create-note args | export names surface in packages/cli/src/create-note/create-note.command.ts (t=2869, 2 atoms) |
| 6.4 | 9903 | — | — | 0.03 | missing | App-client — Solid Router routes | export names surface in packages/app-client/src/routes.tsx (t=3236, 2 atoms) |
| 6.5 | 9990 | 4912 | -5078 | 1.00 | early | Docs site — page map (VitePress src layout) |  |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 3 | 324 | README.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 545 | 1.00 | 545 | 8279 | export at packages/app-client/playwright.config.ts:12 |
| 254 | 1.00 | 254 | 3648 | export at packages/app-client/vite.config.ts:8 |
| 171 | 1.00 | 171 | 1512 | headings outline in CONTRIBUTING.md |
| 166 | 1.00 | 166 | 4881 | imports in packages/app-client/src/index.tsx |
| 165 | 0.85 | 195 | 1850 | headings outline in README.md |
| 137 | 1.00 | 137 | 6970 | export at packages/crypto/src/index.node.ts:8 |
| 137 | 1.00 | 137 | 7107 | export at packages/crypto/src/index.web.ts:8 |
| 114 | 1.00 | 114 | 9610 | README.md section #14 |
| 112 | 1.00 | 112 | 8546 | README.md section #12 |
| 112 | 1.00 | 112 | 4217 | headings outline in packages/app-client/README.md |
| 109 | 1.00 | 109 | 8775 | export at packages/app-client/src/modules/config/config.types.ts:1 |
| 104 | 1.00 | 104 | 9341 | headings outline in packages/docs/src/resources/brand-kit.md |
| 103 | 1.00 | 103 | 9894 | export body at packages/cli/src/files/files.services.ts:26 |
| 102 | 1.00 | 102 | 3789 | headings outline in packages/cli/README.md |
| 101 | 1.00 | 101 | 9767 | export body at packages/cli/src/files/files.services.ts:14 |
| 98 | 1.00 | 98 | 1948 | README.md section #0 |
| 92 | 1.00 | 92 | 950 | export at packages/crypto/build.config.ts:3 |
| 91 | 1.00 | 91 | 5749 | export at packages/lib/src/notes/notes.types.ts:17 |
| 87 | 1.00 | 87 | 6305 | export at packages/cli/src/create-note/create-note.usecases.ts:3 |
| 82 | 1.00 | 82 | 7216 | headings outline in packages/docs/src/self-hosting/other-platforms.md |
| 81 | 1.00 | 81 | 858 | export at packages/lib/build.config.ts:3 |
| 81 | 1.00 | 81 | 4105 | listing of '.github/workflows' |
| 79 | 1.00 | 79 | 7717 | export names surface in packages/lib/src/crypto/serialization/serialization.registry.ts |
| 76 | 1.00 | 76 | 8622 | packages/crypto/README.md section #1 |
| 74 | 1.00 | 74 | 6833 | export names surface in packages/app-server/src/modules/storage/storage.types.ts |
| 74 | 1.00 | 74 | 5879 | export names surface in packages/cli/src/files/files.services.ts |
| 72 | 1.00 | 72 | 7481 | export at packages/crypto/src/encryption-algorithms/encryption-algorithms.types.ts:3 |
| 71 | 1.00 | 71 | 5352 | export names surface in packages/lib/src/crypto/crypto.types.ts |
| 69 | 1.00 | 69 | 4286 | packages/app-client/README.md section #0 |
| 68 | 1.00 | 68 | 9496 | export body at packages/app-client/src/modules/theme/theme.store.ts:4 |
| 67 | 1.00 | 67 | 2489 | export at packages/docs/src/data/i18n.data.ts:59 |
| 64 | 1.00 | 64 | 656 | README headline in packages/crypto/README.md |
| 64 | 1.00 | 64 | 9206 | export body at packages/app-client/src/modules/notes/notes.models.ts:16 |
| 64 | 1.00 | 64 | 6478 | listing of 'packages/app-client/public' |
| 62 | 1.00 | 62 | 1341 | export at packages/cli/build.config.ts:3 |
| 60 | 1.00 | 60 | 6414 | json config renovate.json |
| 56 | 1.00 | 56 | 9127 | export body at packages/app-client/src/modules/notes/notes.context.tsx:27 |
| 56 | 1.00 | 56 | 5805 | headings outline in packages/docs/src/self-hosting/configuration.md |
| 52 | 1.00 | 52 | 7409 | export names surface in packages/crypto/src/encryption-algorithms/encryption-algorithms.types.ts |
| 52 | 1.00 | 52 | 7638 | listing of 'packages/docs/src/public' |
| 51 | 1.00 | 51 | 5930 | export body at packages/cli/src/files/files.services.ts:5 |
| 51 | 1.00 | 51 | 6742 | export names surface in packages/app-client/src/modules/shared/utils/copy.tsx |
| 51 | 1.00 | 51 | 4990 | headings outline in packages/docs/src/integrations/npm-package.md |
| 50 | 1.00 | 50 | 5040 | CONTRIBUTING.md section #1 |
| 50 | 1.00 | 50 | 1279 | README headline in packages/cli/README.md |
| 50 | 1.00 | 50 | 7266 | listing of 'packages/crypto/src/encryption-algorithms' |

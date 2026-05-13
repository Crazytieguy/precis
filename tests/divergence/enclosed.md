Score(3000)=0.542 I=0.746 C=0.394 ns_rows≤3K=23/45 (reached=10 partial=0 missing=13)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 57 | 57 | listing of '.' |  |  | 1.000 |
| ns | 57 |  | 57 | Repo root listing | 1.1 |  | 1.000 |
| walker |  | 86 | 29 | README headline in README.md |  |  | 1.000 |
| ns | 127 |  | 70 | README h1 + tagline | 1.2 |  | 0.806 |
| walker |  | 144 | 58 | package identity in package.json |  |  | 0.817 |
| walker |  | 150 | 6 | plaintext config .nvmrc |  |  | 0.817 |
| ns | 152 |  | 25 | packages/ directory listing | 1.3 |  | 0.682 |
| ns | 168 |  | 16 | pnpm workspace glob | 1.4 |  | 0.653 |
| walker |  | 175 | 25 | listing of 'packages' |  |  | 0.823 |
| walker |  | 201 | 26 | listing of 'packages/lib' |  |  | 0.829 |
| walker |  | 217 | 16 | listing of 'packages/lib/src' |  |  | 0.833 |
| walker |  | 225 | 8 | export names surface in packages/lib/src/index.ts |  |  | 0.833 |
| walker |  | 237 | 12 | export names surface in packages/lib/build.config.ts |  |  | 0.833 |
| ns | 250 |  | 82 | Root package.json — name + version + license | 1.5 |  | 0.795 |
| ns | 357 |  | 107 | Root package.json — author + repo + engines | 1.6 | 1.5 | 0.703 |
| walker |  | 401 | 164 | export at packages/lib/src/index.ts:10 |  |  | 0.707 |
| walker |  | 406 | 5 | listing of 'packages/lib/src/files' |  |  | 0.707 |
| walker |  | 458 | 52 | README headline in packages/lib/README.md |  |  | 0.707 |
| walker |  | 475 | 17 | listing of 'packages/docs' |  |  | 0.708 |
| walker |  | 486 | 11 | listing of 'packages/docs/.vitepress' |  |  | 0.708 |
| walker |  | 497 | 11 | export names surface in packages/docs/.vitepress/config.ts |  |  | 0.632 |
| ns | 497 |  | 140 | pnpm-workspace.yaml — catalog versions | 1.7 | 1.4 | 0.632 |
| walker |  | 511 | 14 | export names surface in packages/docs/.vitepress/plausible.ts |  |  | 0.632 |
| walker |  | 534 | 23 | listing of 'packages/app-server' |  |  | 0.634 |
| walker |  | 615 | 81 | export at packages/lib/build.config.ts:3 |  |  | 0.634 |
| walker |  | 636 | 21 | listing of 'packages/app-server/src' |  |  | 0.636 |
| walker |  | 644 | 8 | export names surface in packages/app-server/src/index.cloudflare.ts |  |  | 0.636 |
| walker |  | 669 | 25 | listing of 'packages/deploy-cloudflare' |  |  | 0.636 |
| ns | 694 |  | 197 | Root package.json — scripts + keywords + devDeps | 1.8 | 1.6 | 0.528 |
| walker |  | 695 | 26 | listing of 'packages/crypto' |  |  | 0.530 |
| walker |  | 707 | 12 | export names surface in packages/crypto/build.config.ts |  |  | 0.530 |
| walker |  | 771 | 64 | README headline in packages/crypto/README.md |  |  | 0.530 |
| walker |  | 802 | 31 | headings outline in packages/crypto/README.md |  |  | 0.530 |
| ns | 872 |  | 178 | README — project structure (per-package one-liners) | 1.9 |  | 0.490 |
| walker |  | 894 | 92 | export at packages/crypto/build.config.ts:3 |  |  | 0.490 |
| walker |  | 934 | 40 | headings outline in packages/lib/README.md |  |  | 0.491 |
| walker |  | 963 | 29 | listing of 'packages/cli' |  |  | 0.495 |
| walker |  | 975 | 12 | export names surface in packages/cli/build.config.ts |  |  | 0.495 |
| walker |  | 980 | 5 | listing of 'packages/cli/bin' |  |  | 0.495 |
| walker |  | 1030 | 50 | README headline in packages/cli/README.md |  |  | 0.495 |
| walker |  | 1092 | 62 | export at packages/cli/build.config.ts:3 |  |  | 0.495 |
| walker |  | 1113 | 21 | listing of 'packages/cli/src' |  |  | 0.495 |
| walker |  | 1123 | 10 | listing of 'packages/cli/src/shared' |  |  | 0.495 |
| walker |  | 1136 | 13 | export names surface in packages/cli/src/shared/cli.models.ts |  |  | 0.495 |
| walker |  | 1150 | 14 | export names surface in packages/cli/src/shared/http.models.ts |  |  | 0.495 |
| walker |  | 1161 | 11 | listing of 'packages/cli/src/files' |  |  | 0.495 |
| walker |  | 1192 | 31 | listing of 'packages/docs/src' |  |  | 0.495 |
| walker |  | 1202 | 10 | listing of 'packages/docs/src/components' |  |  | 0.495 |
| walker |  | 1213 | 11 | listing of 'packages/docs/src/resources' |  |  | 0.495 |
| walker |  | 1225 | 12 | listing of 'packages/cli/src/view-note' |  |  | 0.445 |
| ns | 1225 |  | 353 | README — features list | 1.10 |  | 0.445 |
| walker |  | 1237 | 12 | export names surface in packages/cli/src/view-note/view-note.models.ts |  |  | 0.445 |
| walker |  | 1252 | 15 | export names surface in packages/cli/src/view-note/view-note.command.ts |  |  | 0.445 |
| walker |  | 1264 | 12 | listing of 'packages/docs/.vitepress/theme' |  |  | 0.445 |
| walker |  | 1276 | 12 | listing of 'packages/docs/src/data' |  |  | 0.445 |
| walker |  | 1285 | 9 | export names surface in packages/docs/src/data/configuration.data.ts |  |  | 0.445 |
| walker |  | 1294 | 9 | export names surface in packages/docs/src/data/i18n.data.ts |  |  | 0.445 |
| walker |  | 1341 | 47 | export at packages/docs/src/data/configuration.data.ts:55 |  |  | 0.445 |
| walker |  | 1373 | 32 | listing of 'packages/crypto/src' |  |  | 0.445 |
| walker |  | 1382 | 9 | export names surface in packages/crypto/src/index.node.ts |  |  | 0.445 |
| walker |  | 1391 | 9 | export names surface in packages/crypto/src/index.web.ts |  |  | 0.445 |
| walker |  | 1405 | 14 | export names surface in packages/crypto/src/api-definition.ts |  |  | 0.445 |
| ns | 1499 |  | 274 | README — how-it-works (creator side) | 1.11 |  | 0.420 |
| walker |  | 1600 | 195 | headings outline in README.md |  |  | 0.421 |
| walker |  | 1698 | 98 | README.md section #0 |  |  | 0.421 |
| ns | 1700 |  | 201 | README — how-it-works (recipient side) | 1.12 | 1.11 | 0.406 |
| walker |  | 1716 | 18 | README.md section #32 |  |  | 0.406 |
| ns | 1726 |  | 26 | Top-level lib package contents | 2.1 |  | 0.438 |
| ns | 1742 |  | 16 | lib/src — top-level source modules | 2.2 |  | 0.461 |
| walker |  | 1747 | 31 | README.md section #18 |  |  | 0.461 |
| walker |  | 1777 | 30 | README.md section #25 |  |  | 0.461 |
| walker |  | 1794 | 17 | README.md section #6 |  |  | 0.462 |
| walker |  | 1811 | 17 | README.md section #7 |  |  | 0.464 |
| walker |  | 1968 | 157 | package identity metadata in package.json |  |  | 0.521 |
| walker |  | 2019 | 51 | package runtime metadata in package.json |  |  | 0.564 |
| walker |  | 2097 | 78 | package scripts in package.json |  |  | 0.625 |
| ns | 2101 |  | 359 | @enclosed/lib public API — index.ts re-exports | 2.3 |  | 0.585 |
| ns | 2124 |  | 23 | Top-level app-server package contents | 2.4 |  | 0.595 |
| walker |  | 2129 | 32 | package dependencies in package.json |  |  | 0.620 |
| walker |  | 2147 | 18 | README.md section #2 |  |  | 0.622 |
| walker |  | 2161 | 14 | listing of 'packages/docs/src/integrations' |  |  | 0.622 |
| walker |  | 2180 | 19 | README.md section #8 |  |  | 0.625 |
| ns | 2187 |  | 63 | Top-level app-client package contents | 2.5 |  | 0.594 |
| walker |  | 2195 | 15 | listing of 'packages/app-server/src/modules' |  |  | 0.594 |
| walker |  | 2215 | 20 | README.md section #12 |  |  | 0.597 |
| ns | 2259 |  | 72 | Top-level cli + crypto + docs package contents | 2.6 |  | 0.624 |
| walker |  | 2282 | 67 | export at packages/docs/src/data/i18n.data.ts:59 |  |  | 0.624 |
| walker |  | 2323 | 41 | headings outline in packages/docs/src/index.md |  |  | 0.624 |
| walker |  | 2344 | 21 | README.md section #5 |  |  | 0.628 |
| walker |  | 2365 | 21 | README.md section #9 |  |  | 0.631 |
| walker |  | 2380 | 15 | imports in packages/cli/build.config.ts |  |  | 0.631 |
| walker |  | 2395 | 15 | imports in packages/crypto/build.config.ts |  |  | 0.631 |
| walker |  | 2410 | 15 | imports in packages/lib/build.config.ts |  |  | 0.631 |
| walker |  | 2438 | 28 | export names surface in packages/lib/src/files/files.models.ts |  |  | 0.631 |
| walker |  | 2450 | 12 | listing of 'packages/app-server/src/modules/shared' |  |  | 0.631 |
| walker |  | 2454 | 4 | listing of 'packages/app-server/src/modules/shared/utils' |  |  | 0.631 |
| walker |  | 2465 | 11 | export names surface in packages/app-server/src/modules/shared/utils/random.ts |  |  | 0.631 |
| walker |  | 2488 | 23 | README.md section #11 |  |  | 0.636 |
| walker |  | 2497 | 9 | listing of 'packages/app-server/src/modules/shared/errors' |  |  | 0.636 |
| walker |  | 2506 | 9 | listing of 'packages/app-server/src/modules/shared/validation' |  |  | 0.636 |
| walker |  | 2519 | 13 | listing of 'packages/app-server/src/modules/storage' |  |  | 0.636 |
| walker |  | 2535 | 16 | export names surface in packages/app-server/src/modules/storage/storage.models.ts |  |  | 0.636 |
| ns | 2545 |  | 286 | @enclosed/lib README — install + usage example | 2.7 |  | 0.589 |
| walker |  | 2559 | 24 | README.md section #10 |  |  | 0.593 |
| walker |  | 2582 | 23 | packages/crypto/README.md section #3 |  |  | 0.593 |
| walker |  | 2602 | 20 | listing of 'packages/app-server/src/scripts' |  |  | 0.593 |
| walker |  | 2627 | 25 | README.md section #3 |  |  | 0.599 |
| walker |  | 2648 | 21 | listing of 'packages/cli/src/config' |  |  | 0.599 |
| walker |  | 2657 | 9 | export names surface in packages/cli/src/config/config.usecases.ts |  |  | 0.586 |
| ns | 2657 |  | 112 | lib package.json — runtime dependencies | 2.8 |  | 0.586 |
| walker |  | 2669 | 12 | export names surface in packages/cli/src/config/config.constants.ts |  |  | 0.586 |
| ns | 2678 |  | 21 | app-server/src — top-level entry layout | 3.1 |  | 0.593 |
| walker |  | 2683 | 14 | export names surface in packages/cli/src/config/config.command.ts |  |  | 0.593 |
| walker |  | 2707 | 24 | export names surface in packages/cli/src/config/config.models.ts |  |  | 0.593 |
| walker |  | 2744 | 37 | export at packages/cli/src/config/config.usecases.ts:3 |  |  | 0.593 |
| walker |  | 2765 | 21 | listing of 'packages/cli/src/create-note' |  |  | 0.593 |
| walker |  | 2778 | 13 | export names surface in packages/cli/src/create-note/create-note.usecases.ts |  |  | 0.593 |
| walker |  | 2793 | 15 | export names surface in packages/cli/src/create-note/create-note.command.ts |  |  | 0.593 |
| walker |  | 2814 | 21 | listing of 'packages/crypto/src/node' |  |  | 0.593 |
| walker |  | 2846 | 32 | export names surface in packages/crypto/src/node/crypto.node.usecases.ts |  |  | 0.593 |
| ns | 2851 |  | 173 | app-server/src/modules — module map | 3.2 |  | 0.552 |
| walker |  | 2867 | 21 | listing of 'packages/crypto/src/web' |  |  | 0.552 |
| walker |  | 2899 | 32 | export names surface in packages/crypto/src/web/crypto.web.usecases.ts |  |  | 0.552 |
| walker |  | 2920 | 21 | listing of 'packages/lib/src/api' |  |  | 0.552 |
| walker |  | 2931 | 11 | export names surface in packages/lib/src/api/api.client.ts |  |  | 0.552 |
| ns | 2947 |  | 96 | Cloudflare Workers entrypoint (full) | 3.3 |  | 0.542 |
| walker |  | 2949 | 18 | export names surface in packages/lib/src/api/api.constants.ts |  |  | 0.542 |
| walker |  | 2972 | 23 | export names surface in packages/lib/src/api/api.models.ts |  |  | 0.542 |
| walker |  | 3035 | 63 | listing of 'packages/app-client' |  |  | 0.581 |
| walker |  | 3046 | 11 | export names surface in packages/app-client/playwright.config.ts |  |  | 0.581 |
| walker |  | 3057 | 11 | export names surface in packages/app-client/uno.config.ts |  |  | 0.581 |
| walker |  | 3068 | 11 | export names surface in packages/app-client/vite.config.ts |  |  | 0.581 |
| walker |  | 3104 | 36 | README headline in packages/app-client/README.md |  |  | 0.581 |
| walker |  | 3115 | 11 | listing of 'packages/app-client/e2e-tests' |  |  | 0.581 |
| walker |  | 3141 | 26 | listing of 'packages/app-client/src' |  |  | 0.581 |
| walker |  | 3145 | 4 | listing of 'packages/app-client/src/assets' |  |  | 0.581 |
| walker |  | 3157 | 12 | listing of 'packages/app-client/src/scripts' |  |  | 0.581 |
| walker |  | 3192 | 35 | headings outline in packages/docs/src/resources/i18n.md |  |  | 0.581 |
| walker |  | 3211 | 19 | export names surface in packages/app-server/src/modules/shared/errors/errors.ts |  |  | 0.581 |
| walker |  | 3241 | 30 | packages/crypto/README.md section #2 |  |  | 0.581 |
| walker |  | 3254 | 13 | imports in packages/app-server/src/reset.d.ts |  |  | 0.581 |
| walker |  | 3299 | 45 | export at packages/cli/src/config/config.constants.ts:3 |  |  | 0.581 |
| walker |  | 3322 | 23 | packages/lib/README.md section #4 |  |  | 0.581 |
| ns | 3451 |  | 504 | Hono createServer factory + middleware stack | 3.4 |  | 0.537 |
| walker |  | 3576 | 254 | export at packages/app-client/vite.config.ts:8 |  |  | 0.537 |
| walker |  | 3601 | 25 | listing of 'packages/lib/src/crypto' |  |  | 0.537 |
| walker |  | 3615 | 14 | export names surface in packages/lib/src/crypto/crypto.usecases.ts |  |  | 0.537 |
| walker |  | 3717 | 102 | headings outline in packages/cli/README.md |  |  | 0.537 |
| walker |  | 3742 | 25 | packages/cli/README.md section #2 |  |  | 0.537 |
| ns | 3749 |  | 298 | Notes routes — endpoint registrations (locations only) | 3.5 |  | 0.518 |
| walker |  | 3765 | 23 | packages/cli/README.md section #7 |  |  | 0.518 |
| walker |  | 3792 | 27 | listing of 'packages/lib/src/notes' |  |  | 0.518 |
| walker |  | 3803 | 11 | export names surface in packages/lib/src/notes/notes.usecases.ts |  |  | 0.518 |
| walker |  | 3817 | 14 | export names surface in packages/lib/src/notes/notes.services.ts |  |  | 0.518 |
| walker |  | 3845 | 28 | export names surface in packages/lib/src/notes/notes.models.ts |  |  | 0.518 |
| walker |  | 3881 | 36 | export names surface in packages/lib/src/notes/notes.types.ts |  |  | 0.518 |
| walker |  | 3903 | 22 | export at packages/lib/src/notes/notes.types.ts:12 |  |  | 0.518 |
| walker |  | 4015 | 112 | headings outline in packages/app-client/README.md |  |  | 0.518 |
| walker |  | 4015 | 0 | packages/app-client/README.md section #3 |  |  | 0.518 |
| walker |  | 4084 | 69 | packages/app-client/README.md section #0 |  |  | 0.518 |
| walker |  | 4111 | 27 | packages/app-client/README.md section #2 |  |  | 0.518 |
| walker |  | 4142 | 31 | README.md section #4 |  |  | 0.523 |
| walker |  | 4172 | 30 | packages/cli/README.md section #6 |  |  | 0.523 |
| ns | 4200 |  | 451 | Notes routes — POST /api/notes payload schema + validation handler | 3.6 | 3.5 | 0.490 |
| walker |  | 4201 | 29 | listing of 'packages/app-client/src/modules' |  |  | 0.490 |
| walker |  | 4206 | 5 | listing of 'packages/app-client/src/modules/theme' |  |  | 0.490 |
| walker |  | 4212 | 6 | listing of 'packages/app-client/src/modules/ui' |  |  | 0.490 |
| walker |  | 4225 | 13 | export names surface in packages/app-client/src/modules/theme/theme.store.ts |  |  | 0.490 |
| walker |  | 4225 | 0 | export at packages/app-client/src/modules/theme/theme.store.ts:4 |  |  | 0.490 |
| walker |  | 4231 | 6 | listing of 'packages/app-client/src/modules/ui/layouts' |  |  | 0.490 |
| walker |  | 4242 | 11 | listing of 'packages/app-client/src/modules/docs' |  |  | 0.490 |
| walker |  | 4254 | 12 | export names surface in packages/app-client/src/modules/docs/docs.models.ts |  |  | 0.490 |
| walker |  | 4265 | 11 | listing of 'packages/app-client/src/modules/files' |  |  | 0.490 |
| walker |  | 4280 | 15 | listing of 'packages/app-client/src/modules/shared' |  |  | 0.490 |
| walker |  | 4284 | 4 | listing of 'packages/app-client/src/modules/shared/hooks' |  |  | 0.490 |
| walker |  | 4288 | 4 | listing of 'packages/app-client/src/modules/shared/style' |  |  | 0.490 |
| walker |  | 4293 | 5 | listing of 'packages/app-client/src/modules/shared/utils' |  |  | 0.490 |
| walker |  | 4304 | 11 | export names surface in packages/app-client/src/modules/shared/hooks/hooks.ts |  |  | 0.490 |
| walker |  | 4312 | 8 | listing of 'packages/app-client/src/modules/shared/files' |  |  | 0.490 |
| walker |  | 4327 | 15 | export names surface in packages/app-client/src/modules/shared/files/convert.ts |  |  | 0.490 |
| walker |  | 4353 | 26 | export names surface in packages/app-client/src/modules/files/files.models.ts |  |  | 0.490 |
| walker |  | 4373 | 20 | export names surface in packages/app-client/src/modules/shared/files/download.ts |  |  | 0.490 |
| walker |  | 4389 | 16 | listing of 'packages/app-client/src/modules/config' |  |  | 0.490 |
| walker |  | 4397 | 8 | export names surface in packages/app-client/src/modules/config/config.provider.tsx |  |  | 0.490 |
| walker |  | 4408 | 11 | export at packages/app-client/src/modules/config/config.provider.tsx:5 |  |  | 0.490 |
| walker |  | 4419 | 11 | export names surface in packages/app-client/src/modules/config/config.types.ts |  |  | 0.490 |
| walker |  | 4434 | 15 | export names surface in packages/app-client/src/modules/config/config.constants.ts |  |  | 0.490 |
| walker |  | 4452 | 18 | listing of 'packages/app-client/src/modules/auth' |  |  | 0.490 |
| walker |  | 4460 | 8 | export names surface in packages/app-client/src/modules/auth/auth.models.ts |  |  | 0.490 |
| walker |  | 4473 | 13 | export at packages/app-client/src/modules/auth/auth.models.ts:1 |  |  | 0.490 |
| walker |  | 4483 | 10 | export names surface in packages/app-client/src/modules/auth/auth.services.ts |  |  | 0.490 |
| walker |  | 4489 | 6 | listing of 'packages/app-client/src/modules/auth/pages' |  |  | 0.490 |
| walker |  | 4505 | 16 | export names surface in packages/app-client/src/modules/auth/auth.store.ts |  |  | 0.490 |
| walker |  | 4505 | 0 | export at packages/app-client/src/modules/auth/auth.store.ts:6 |  |  | 0.490 |
| walker |  | 4521 | 16 | export names surface in packages/app-client/src/modules/auth/pages/login.page.tsx |  |  | 0.490 |
| walker |  | 4521 | 0 | export at packages/app-client/src/modules/auth/pages/login.page.tsx:41 |  |  | 0.490 |
| walker |  | 4544 | 23 | export names surface in packages/app-client/src/modules/shared/style/cn.ts |  |  | 0.490 |
| ns | 4659 |  | 459 | Notes routes — GET /api/notes/:noteId + private-note auth gating | 3.7 | 3.5 | 0.458 |
| walker |  | 4710 | 166 | imports in packages/app-client/src/index.tsx |  |  | 0.458 |
| walker |  | 4725 | 15 | export names surface in packages/app-client/src/routes.tsx |  |  | 0.458 |
| walker |  | 4725 | 0 | export at packages/app-client/src/routes.tsx:11 |  |  | 0.458 |
| walker |  | 4756 | 31 | listing of 'packages/docs/src/self-hosting' |  |  | 0.458 |
| walker |  | 4783 | 27 | headings outline in packages/docs/src/self-hosting/troubleshooting.md |  |  | 0.458 |
| walker |  | 4783 | 0 | packages/docs/src/self-hosting/troubleshooting.md section #0 |  |  | 0.458 |
| ns | 4821 |  | 162 | Notes routes — GET /api/notes/:noteId/exists | 3.8 | 3.5 | 0.449 |
| walker |  | 4834 | 51 | headings outline in packages/docs/src/integrations/npm-package.md |  |  | 0.449 |
| walker |  | 4864 | 30 | packages/lib/README.md section #3 |  |  | 0.449 |
| walker |  | 5046 | 182 | imports in packages/lib/src/index.ts |  |  | 0.492 |
| ns | 5055 |  | 234 | Config — env var name catalog (locations only) | 4.1 |  | 0.476 |
| walker |  | 5075 | 29 | imports in packages/app-client/playwright.config.ts |  |  | 0.476 |
| walker |  | 5090 | 15 | listing of 'packages/app-server/src/modules/shared/logger' |  |  | 0.476 |
| walker |  | 5101 | 11 | export names surface in packages/app-server/src/modules/shared/logger/logger.ts |  |  | 0.476 |
| walker |  | 5114 | 13 | export names surface in packages/app-server/src/modules/shared/logger/logger.test-utils.ts |  |  | 0.476 |
| walker |  | 5114 | 0 | export at packages/app-server/src/modules/shared/logger/logger.test-utils.ts:3 |  |  | 0.476 |
| walker |  | 5128 | 14 | export names surface in packages/app-server/src/modules/shared/logger/logger.types.ts |  |  | 0.476 |
| walker |  | 5150 | 22 | listing of 'packages/app-server/src/modules/app' |  |  | 0.483 |
| walker |  | 5161 | 11 | export names surface in packages/app-server/src/modules/app/server.ts |  |  | 0.483 |
| walker |  | 5166 | 5 | listing of 'packages/app-server/src/modules/app/users' |  |  | 0.483 |
| walker |  | 5178 | 12 | export names surface in packages/app-server/src/modules/app/users/users.repository.ts |  |  | 0.483 |
| walker |  | 5200 | 22 | listing of 'packages/app-server/src/modules/tasks' |  |  | 0.490 |
| walker |  | 5211 | 11 | export names surface in packages/app-server/src/modules/tasks/tasks.models.ts |  |  | 0.490 |
| walker |  | 5223 | 12 | export names surface in packages/app-server/src/modules/tasks/task-scheduler.ts |  |  | 0.490 |
| walker |  | 5238 | 15 | export names surface in packages/app-server/src/modules/tasks/tasks.types.ts |  |  | 0.490 |
| walker |  | 5250 | 12 | imports in packages/cli/src/shared/cli.models.ts |  |  | 0.490 |
| walker |  | 5306 | 56 | headings outline in packages/docs/src/self-hosting/configuration.md |  |  | 0.490 |
| walker |  | 5329 | 23 | listing of 'packages/app-client/src/modules/i18n' |  |  | 0.490 |
| walker |  | 5363 | 34 | export names surface in packages/app-client/src/modules/i18n/i18n.models.ts |  |  | 0.490 |
| walker |  | 5363 | 0 | export at packages/app-client/src/modules/i18n/i18n.models.ts:8 |  |  | 0.490 |
| walker |  | 5386 | 23 | listing of 'packages/crypto/src/node/encryption-algorithms' |  |  | 0.490 |
| walker |  | 5406 | 20 | export names surface in packages/crypto/src/node/encryption-algorithms/crypto.node.aes-256-gcm.ts |  |  | 0.490 |
| walker |  | 5429 | 23 | listing of 'packages/crypto/src/web/encryption-algorithms' |  |  | 0.490 |
| walker |  | 5449 | 20 | export names surface in packages/crypto/src/web/encryption-algorithms/crypto.web.aes-256-gcm.ts |  |  | 0.490 |
| walker |  | 5487 | 38 | README.md section #1 |  |  | 0.494 |
| walker |  | 5500 | 13 | imports in packages/cli/src/config/config.constants.ts |  |  | 0.494 |
| walker |  | 5513 | 13 | imports in packages/cli/src/shared/http.models.ts |  |  | 0.494 |
| walker |  | 5537 | 24 | listing of 'packages/lib/src/crypto/encryption-algorithms' |  |  | 0.494 |
| walker |  | 5549 | 12 | export names surface in packages/lib/src/crypto/encryption-algorithms/encryption-algorithms.models.ts |  |  | 0.494 |
| walker |  | 5562 | 13 | export names surface in packages/lib/src/crypto/encryption-algorithms/encryption-algorithms.registry.ts |  |  | 0.494 |
| walker |  | 5579 | 17 | export names surface in packages/lib/src/crypto/encryption-algorithms/encryption-algorithms.types.ts |  |  | 0.494 |
| ns | 5587 |  | 532 | Config — defaults for the most-asked-about env vars | 4.2 | 4.1 | 0.470 |
| walker |  | 5619 | 40 | export names surface in packages/app-client/src/modules/i18n/i18n.provider.tsx |  |  | 0.470 |
| walker |  | 5619 | 0 | export at packages/app-client/src/modules/i18n/i18n.provider.tsx:44 |  |  | 0.470 |
| walker |  | 5632 | 13 | export at packages/app-client/src/modules/i18n/i18n.provider.tsx:10 |  |  | 0.470 |
| walker |  | 5719 | 87 | export at packages/cli/src/create-note/create-note.usecases.ts:3 |  |  | 0.470 |
| ns | 5731 |  | 144 | Config — getConfig export + figue setup | 4.3 | 4.1 | 0.463 |
| walker |  | 5768 | 49 | export body at packages/cli/src/create-note/create-note.usecases.ts:3 body 12 |  |  | 0.463 |
| walker |  | 5828 | 60 | json config renovate.json |  |  | 0.463 |
| ns | 5875 |  | 144 | Node.js entrypoint — config + storage + server build | 4.4 |  | 0.457 |
| walker |  | 5892 | 64 | listing of 'packages/app-client/public' |  |  | 0.457 |
| walker |  | 5918 | 26 | listing of 'packages/lib/src/crypto/serialization' |  |  | 0.457 |
| walker |  | 5930 | 12 | export names surface in packages/lib/src/crypto/serialization/serialization.models.ts |  |  | 0.457 |
| walker |  | 5943 | 13 | export names surface in packages/lib/src/crypto/serialization/serialization.test-utils.ts |  |  | 0.457 |
| walker |  | 5977 | 34 | export names surface in packages/lib/src/crypto/serialization/serialization.types.ts |  |  | 0.457 |
| walker |  | 5992 | 15 | listing of 'packages/lib/src/crypto/serialization/cbor-array' |  |  | 0.457 |
| walker |  | 6010 | 18 | export names surface in packages/lib/src/crypto/serialization/cbor-array/cbor-array.serialization.ts |  |  | 0.457 |
| walker |  | 6025 | 15 | imports in packages/cli/src/config/config.usecases.ts |  |  | 0.457 |
| walker |  | 6040 | 15 | imports in packages/lib/src/notes/notes.services.ts |  |  | 0.457 |
| walker |  | 6056 | 16 | imports in packages/lib/src/api/api.models.ts |  |  | 0.457 |
| walker |  | 6105 | 49 | export names surface in packages/app-server/src/modules/app/server.types.ts |  |  | 0.457 |
| walker |  | 6176 | 71 | export names surface in packages/lib/src/crypto/crypto.types.ts |  |  | 0.457 |
| ns | 6202 |  | 327 | Node.js entrypoint — static + SPA fallback + cron + listen | 4.5 | 4.4 | 0.440 |
| walker |  | 6213 | 37 | export at packages/lib/src/crypto/crypto.types.ts:7 |  |  | 0.440 |
| walker |  | 6230 | 17 | imports in packages/cli/src/view-note/view-note.models.ts |  |  | 0.440 |
| walker |  | 6259 | 29 | README.md section #31 |  |  | 0.440 |
| walker |  | 6396 | 137 | export at packages/crypto/src/index.node.ts:8 |  |  | 0.440 |
| ns | 6457 |  | 255 | Storage drivers — memory + fs-lite | 4.6 |  | 0.430 |
| walker |  | 6533 | 137 | export at packages/crypto/src/index.web.ts:8 |  |  | 0.430 |
| walker |  | 6607 | 74 | export names surface in packages/cli/src/files/files.services.ts |  |  | 0.430 |
| walker |  | 6607 | 0 | export at packages/cli/src/files/files.services.ts:5 |  |  | 0.430 |
| walker |  | 6607 | 0 | export at packages/cli/src/files/files.services.ts:14 |  |  | 0.430 |
| walker |  | 6607 | 0 | export at packages/cli/src/files/files.services.ts:26 |  |  | 0.430 |
| walker |  | 6658 | 51 | export body at packages/cli/src/files/files.services.ts:5 body 6 |  |  | 0.430 |
| walker |  | 6685 | 27 | imports in packages/docs/.vitepress/plausible.ts |  |  | 0.430 |
| walker |  | 6767 | 82 | headings outline in packages/docs/src/self-hosting/other-platforms.md |  |  | 0.430 |
| walker |  | 6767 | 0 | packages/docs/src/self-hosting/other-platforms.md section #0 |  |  | 0.430 |
| walker |  | 6817 | 50 | listing of 'packages/crypto/src/encryption-algorithms' |  |  | 0.430 |
| walker |  | 6825 | 8 | export names surface in packages/crypto/src/encryption-algorithms/encryption-algorithms.test-utils.ts |  |  | 0.430 |
| walker |  | 6839 | 14 | export at packages/crypto/src/encryption-algorithms/encryption-algorithms.test-utils.ts:5 |  |  | 0.430 |
| walker |  | 6851 | 12 | export names surface in packages/crypto/src/encryption-algorithms/encryption-algorithms.models.ts |  |  | 0.430 |
| walker |  | 6864 | 13 | export names surface in packages/crypto/src/encryption-algorithms/encryption-algorithms.registry.ts |  |  | 0.430 |
| ns | 6892 |  | 435 | Storage driver — Cloudflare KV (with 413 translation) | 4.7 |  | 0.418 |
| walker |  | 6908 | 44 | export names surface in packages/crypto/src/encryption-algorithms/encryption-algorithms.constants.ts |  |  | 0.418 |
| walker |  | 6960 | 52 | export names surface in packages/crypto/src/encryption-algorithms/encryption-algorithms.types.ts |  |  | 0.418 |
| walker |  | 6983 | 23 | listing of 'packages/app-client/src/modules/shared/http' |  |  | 0.418 |
| walker |  | 6994 | 11 | export names surface in packages/app-client/src/modules/shared/http/http-client.ts |  |  | 0.418 |
| walker |  | 7017 | 23 | export names surface in packages/app-client/src/modules/shared/http/http-client.models.ts |  |  | 0.418 |
| walker |  | 7043 | 26 | export names surface in packages/app-client/src/modules/shared/http/http-errors.ts |  |  | 0.418 |
| walker |  | 7092 | 49 | export at packages/lib/src/notes/notes.types.ts:4 |  |  | 0.418 |
| walker |  | 7114 | 22 | packages/docs/src/resources/i18n.md section #0 |  |  | 0.418 |
| walker |  | 7166 | 52 | listing of 'packages/docs/src/public' |  |  | 0.418 |
| walker |  | 7197 | 31 | README.md section #30 |  |  | 0.418 |
| ns | 7406 |  | 514 | Notes — models + types + constants | 5.1 |  | 0.402 |
| walker |  | 7742 | 545 | export at packages/app-client/playwright.config.ts:12 |  |  | 0.402 |
| walker |  | 7768 | 26 | export doc at packages/app-client/playwright.config.ts:12 |  |  | 0.402 |
| walker |  | 7793 | 25 | listing of 'packages/app-server/src/modules/app/config' |  |  | 0.402 |
| walker |  | 7804 | 11 | export names surface in packages/app-server/src/modules/app/config/config.test-utils.ts |  |  | 0.402 |
| walker |  | 7816 | 12 | export names surface in packages/app-server/src/modules/app/config/config.routes.ts |  |  | 0.402 |
| walker |  | 7830 | 14 | export names surface in packages/app-server/src/modules/app/config/config.models.ts |  |  | 0.402 |
| ns | 7835 |  | 429 | Notes repository — exports + factory + getRefreshedNote usecase | 5.2 |  | 0.387 |
| walker |  | 7844 | 14 | export names surface in packages/app-server/src/modules/app/config/config.types.ts |  |  | 0.387 |
| walker |  | 7883 | 39 | export names surface in packages/app-server/src/modules/app/config/config.ts |  |  | 0.387 |
| walker |  | 7883 | 0 | export at packages/app-server/src/modules/app/config/config.ts:258 |  |  | 0.387 |
| walker |  | 7897 | 14 | imports in packages/app-client/src/modules/config/config.constants.ts |  |  | 0.387 |
| walker |  | 8009 | 112 | README.md section #24 |  |  | 0.387 |
| walker |  | 8085 | 76 | packages/crypto/README.md section #1 |  |  | 0.387 |
| ns | 8111 |  | 276 | Auth catalog — errors + models + services signatures | 5.3 |  | 0.379 |
| walker |  | 8129 | 44 | export body at packages/app-server/src/modules/app/config/config.ts:258 body 259 |  |  | 0.383 |
| walker |  | 8168 | 39 | listing of 'packages/app-client/src/modules/notes' |  |  | 0.383 |
| walker |  | 8181 | 13 | export names surface in packages/app-client/src/modules/notes/notes.usecases.ts |  |  | 0.383 |
| walker |  | 8201 | 20 | export names surface in packages/app-client/src/modules/notes/notes.services.ts |  |  | 0.383 |
| walker |  | 8232 | 31 | export names surface in packages/app-client/src/modules/notes/notes.constants.ts |  |  | 0.383 |
| walker |  | 8265 | 33 | export names surface in packages/app-client/src/modules/notes/notes.context.tsx |  |  | 0.383 |
| walker |  | 8265 | 0 | export at packages/app-client/src/modules/notes/notes.context.tsx:11 |  |  | 0.383 |
| walker |  | 8265 | 0 | export at packages/app-client/src/modules/notes/notes.context.tsx:27 |  |  | 0.383 |
| walker |  | 8279 | 14 | listing of 'packages/app-client/src/modules/notes/components' |  |  | 0.383 |
| walker |  | 8295 | 16 | export names surface in packages/app-client/src/modules/notes/components/file-uploader.tsx |  |  | 0.383 |
| walker |  | 8309 | 14 | listing of 'packages/app-client/src/modules/notes/pages' |  |  | 0.383 |
| walker |  | 8326 | 17 | export names surface in packages/app-client/src/modules/notes/pages/create-note.page.tsx |  |  | 0.383 |
| walker |  | 8326 | 0 | export at packages/app-client/src/modules/notes/pages/create-note.page.tsx:109 |  |  | 0.383 |
| walker |  | 8343 | 17 | export names surface in packages/app-client/src/modules/notes/pages/view-note.page.tsx |  |  | 0.383 |
| walker |  | 8343 | 0 | export at packages/app-client/src/modules/notes/pages/view-note.page.tsx:65 |  |  | 0.383 |
| walker |  | 8380 | 37 | export names surface in packages/app-client/src/modules/notes/notes.models.ts |  |  | 0.383 |
| walker |  | 8380 | 0 | export at packages/app-client/src/modules/notes/notes.models.ts:16 |  |  | 0.383 |
| ns | 8429 |  | 318 | Auth middleware — gating + protected-route guard | 5.4 |  | 0.375 |
| walker |  | 8436 | 56 | export body at packages/app-client/src/modules/notes/notes.context.tsx:27 body 28 |  |  | 0.375 |
| walker |  | 8481 | 45 | export names surface in packages/app-client/src/modules/notes/components/note-password-field.tsx |  |  | 0.375 |
| walker |  | 8481 | 0 | export at packages/app-client/src/modules/notes/components/note-password-field.tsx:7 |  |  | 0.375 |
| walker |  | 8496 | 15 | imports in packages/app-server/src/modules/tasks/tasks.types.ts |  |  | 0.375 |
| walker |  | 8560 | 64 | export body at packages/app-client/src/modules/notes/notes.models.ts:16 body 17 |  |  | 0.375 |
| walker |  | 8609 | 49 | export names surface in packages/app-client/src/modules/ui/layouts/app.layout.tsx |  |  | 0.375 |
| walker |  | 8609 | 0 | export at packages/app-client/src/modules/ui/layouts/app.layout.tsx:72 |  |  | 0.375 |
| walker |  | 8609 | 0 | export at packages/app-client/src/modules/ui/layouts/app.layout.tsx:209 |  |  | 0.375 |
| walker |  | 8609 | 0 | export at packages/app-client/src/modules/ui/layouts/app.layout.tsx:240 |  |  | 0.375 |
| walker |  | 8713 | 104 | headings outline in packages/docs/src/resources/brand-kit.md |  |  | 0.375 |
| walker |  | 8736 | 23 | imports in packages/crypto/src/encryption-algorithms/encryption-algorithms.types.ts |  |  | 0.375 |
| walker |  | 8752 | 16 | imports in packages/app-client/src/modules/auth/auth.services.ts |  |  | 0.375 |
| walker |  | 8768 | 16 | imports in packages/app-client/src/modules/i18n/i18n.models.ts |  |  | 0.375 |
| walker |  | 8784 | 16 | imports in packages/app-client/src/modules/notes/notes.services.ts |  |  | 0.375 |
| walker |  | 8800 | 16 | imports in packages/lib/src/crypto/serialization/serialization.models.ts |  |  | 0.375 |
| ns | 8821 |  | 392 | Auth login route — handler body (timing-safe bcrypt + JWT issue) | 5.5 |  | 0.365 |
| walker |  | 8868 | 68 | export body at packages/app-client/src/modules/theme/theme.store.ts:4 body 5 |  |  | 0.365 |
| walker |  | 8982 | 114 | README.md section #26 |  |  | 0.365 |
| walker |  | 9033 | 51 | export names surface in packages/app-client/src/modules/shared/utils/copy.tsx |  |  | 0.365 |
| walker |  | 9033 | 0 | export at packages/app-client/src/modules/shared/utils/copy.tsx:6 |  |  | 0.365 |
| walker |  | 9033 | 0 | export at packages/app-client/src/modules/shared/utils/copy.tsx:19 |  |  | 0.365 |
| walker |  | 9045 | 12 | imports in packages/app-server/src/modules/shared/logger/logger.ts |  |  | 0.365 |
| walker |  | 9089 | 44 | listing of 'packages/docs/src/public/logos' |  |  | 0.365 |
| walker |  | 9134 | 45 | export at packages/app-server/src/modules/app/server.types.ts:5 |  |  | 0.358 |
| ns | 9134 |  | 313 | Lib — encryptNote body (crypto.usecases) | 6.1 |  | 0.358 |
| ns | 9232 |  | 98 | Lib — note URL hash-fragment markers (`pw` / `dar`) | 6.2 |  | 0.357 |
| walker |  | 9235 | 101 | export body at packages/cli/src/files/files.services.ts:14 body 15 |  |  | 0.357 |
| walker |  | 9297 | 62 | README.md section #13 |  |  | 0.363 |
| walker |  | 9321 | 24 | imports in packages/cli/src/create-note/create-note.usecases.ts |  |  | 0.363 |
| walker |  | 9424 | 103 | export body at packages/cli/src/files/files.services.ts:26 body 27 |  |  | 0.363 |
| walker |  | 9486 | 62 | package identity in packages/deploy-cloudflare/package.json |  |  | 0.363 |
| ns | 9539 |  | 307 | CLI dispatcher + create-note args | 6.3 |  | 0.356 |
| walker |  | 9642 | 156 | plaintext config pnpm-workspace.yaml |  |  | 0.377 |
| walker |  | 9716 | 74 | export names surface in packages/app-server/src/modules/storage/storage.types.ts |  |  | 0.377 |
| walker |  | 9729 | 13 | imports in packages/app-client/src/modules/shared/http/http-errors.ts |  |  | 0.377 |
| walker |  | 9769 | 40 | README.md section #29 |  |  | 0.377 |
| walker |  | 9836 | 67 | export at packages/app-client/src/modules/notes/components/file-uploader.tsx:62 |  |  | 0.377 |
| ns | 9903 |  | 364 | App-client — Solid Router routes | 6.4 |  | 0.369 |
| walker |  | 9909 | 73 | packages/lib/README.md section #1 |  |  | 0.376 |
| walker |  | 9981 | 72 | export at packages/crypto/src/encryption-algorithms/encryption-algorithms.types.ts:3 |  |  | 0.376 |
| ns | 9990 |  | 87 | Docs site — page map (VitePress src layout) | 6.5 |  | 0.391 |

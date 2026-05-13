Score(3000)=0.542 I=0.746 C=0.394 ns_rows≤3K=23/45 (reached=10 partial=0 missing=13)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 57 | 57 | listing of '.' |  |  | 1.000 |
| ns | 57 |  | 57 | Repo root listing | 1.1 |  | 1.000 |
| walker |  | 86 | 29 | README headline in README.md |  |  | 1.000 |
| walker |  | 92 | 6 | plaintext config .nvmrc |  |  | 1.000 |
| walker |  | 117 | 25 | listing of 'packages' |  |  | 1.000 |
| ns | 127 |  | 70 | README h1 + tagline | 1.2 |  | 0.840 |
| ns | 152 |  | 25 | packages/ directory listing | 1.3 |  | 0.849 |
| ns | 168 |  | 16 | pnpm workspace glob | 1.4 |  | 0.813 |
| ns | 250 |  | 82 | Root package.json — name + version + license | 1.5 |  | 0.726 |
| walker |  | 332 | 215 | package identity in package.json |  |  | 0.803 |
| ns | 357 |  | 107 | Root package.json — author + repo + engines | 1.6 | 1.5 | 0.764 |
| walker |  | 358 | 26 | listing of 'packages/lib' |  |  | 0.769 |
| walker |  | 374 | 16 | listing of 'packages/lib/src' |  |  | 0.773 |
| walker |  | 382 | 8 | export names surface in packages/lib/src/index.ts |  |  | 0.773 |
| walker |  | 394 | 12 | export names surface in packages/lib/build.config.ts |  |  | 0.773 |
| ns | 497 |  | 140 | pnpm-workspace.yaml — catalog versions | 1.7 | 1.4 | 0.690 |
| walker |  | 558 | 164 | export at packages/lib/src/index.ts:10 |  |  | 0.694 |
| walker |  | 563 | 5 | listing of 'packages/lib/src/files' |  |  | 0.694 |
| walker |  | 615 | 52 | README headline in packages/lib/README.md |  |  | 0.695 |
| walker |  | 632 | 17 | listing of 'packages/docs' |  |  | 0.695 |
| walker |  | 643 | 11 | listing of 'packages/docs/.vitepress' |  |  | 0.695 |
| walker |  | 654 | 11 | export names surface in packages/docs/.vitepress/config.ts |  |  | 0.695 |
| walker |  | 668 | 14 | export names surface in packages/docs/.vitepress/plausible.ts |  |  | 0.695 |
| walker |  | 691 | 23 | listing of 'packages/app-server' |  |  | 0.698 |
| ns | 694 |  | 197 | Root package.json — scripts + keywords + devDeps | 1.8 | 1.6 | 0.616 |
| walker |  | 772 | 81 | export at packages/lib/build.config.ts:3 |  |  | 0.616 |
| walker |  | 793 | 21 | listing of 'packages/app-server/src' |  |  | 0.618 |
| walker |  | 801 | 8 | export names surface in packages/app-server/src/index.cloudflare.ts |  |  | 0.618 |
| ns | 872 |  | 178 | README — project structure (per-package one-liners) | 1.9 |  | 0.571 |
| walker |  | 879 | 78 | package scripts in package.json |  |  | 0.659 |
| walker |  | 904 | 25 | listing of 'packages/deploy-cloudflare' |  |  | 0.659 |
| walker |  | 930 | 26 | listing of 'packages/crypto' |  |  | 0.661 |
| walker |  | 942 | 12 | export names surface in packages/crypto/build.config.ts |  |  | 0.661 |
| walker |  | 1006 | 64 | README headline in packages/crypto/README.md |  |  | 0.661 |
| walker |  | 1037 | 31 | headings outline in packages/crypto/README.md |  |  | 0.661 |
| walker |  | 1129 | 92 | export at packages/crypto/build.config.ts:3 |  |  | 0.661 |
| walker |  | 1169 | 40 | headings outline in packages/lib/README.md |  |  | 0.662 |
| walker |  | 1198 | 29 | listing of 'packages/cli' |  |  | 0.667 |
| walker |  | 1210 | 12 | export names surface in packages/cli/build.config.ts |  |  | 0.667 |
| walker |  | 1215 | 5 | listing of 'packages/cli/bin' |  |  | 0.667 |
| ns | 1225 |  | 353 | README — features list | 1.10 |  | 0.599 |
| walker |  | 1265 | 50 | README headline in packages/cli/README.md |  |  | 0.599 |
| walker |  | 1327 | 62 | export at packages/cli/build.config.ts:3 |  |  | 0.599 |
| walker |  | 1348 | 21 | listing of 'packages/cli/src' |  |  | 0.599 |
| walker |  | 1358 | 10 | listing of 'packages/cli/src/shared' |  |  | 0.599 |
| walker |  | 1371 | 13 | export names surface in packages/cli/src/shared/cli.models.ts |  |  | 0.599 |
| walker |  | 1385 | 14 | export names surface in packages/cli/src/shared/http.models.ts |  |  | 0.599 |
| ns | 1499 |  | 274 | README — how-it-works (creator side) | 1.11 |  | 0.566 |
| walker |  | 1556 | 171 | headings outline in CONTRIBUTING.md |  |  | 0.566 |
| walker |  | 1592 | 36 | CONTRIBUTING.md section #0 |  |  | 0.566 |
| walker |  | 1603 | 11 | listing of 'packages/cli/src/files' |  |  | 0.566 |
| walker |  | 1686 | 83 | package dependencies in package.json |  |  | 0.649 |
| ns | 1700 |  | 201 | README — how-it-works (recipient side) | 1.12 | 1.11 | 0.626 |
| walker |  | 1717 | 31 | listing of 'packages/docs/src' |  |  | 0.626 |
| ns | 1726 |  | 26 | Top-level lib package contents | 2.1 |  | 0.641 |
| walker |  | 1727 | 10 | listing of 'packages/docs/src/components' |  |  | 0.641 |
| walker |  | 1738 | 11 | listing of 'packages/docs/src/resources' |  |  | 0.641 |
| ns | 1742 |  | 16 | lib/src — top-level source modules | 2.2 |  | 0.652 |
| walker |  | 1750 | 12 | listing of 'packages/cli/src/view-note' |  |  | 0.652 |
| walker |  | 1762 | 12 | export names surface in packages/cli/src/view-note/view-note.models.ts |  |  | 0.652 |
| walker |  | 1777 | 15 | export names surface in packages/cli/src/view-note/view-note.command.ts |  |  | 0.652 |
| walker |  | 1789 | 12 | listing of 'packages/docs/.vitepress/theme' |  |  | 0.652 |
| walker |  | 1801 | 12 | listing of 'packages/docs/src/data' |  |  | 0.652 |
| walker |  | 1810 | 9 | export names surface in packages/docs/src/data/configuration.data.ts |  |  | 0.652 |
| walker |  | 1819 | 9 | export names surface in packages/docs/src/data/i18n.data.ts |  |  | 0.652 |
| walker |  | 1866 | 47 | export at packages/docs/src/data/configuration.data.ts:55 |  |  | 0.652 |
| walker |  | 1898 | 32 | listing of 'packages/crypto/src' |  |  | 0.652 |
| walker |  | 1907 | 9 | export names surface in packages/crypto/src/index.node.ts |  |  | 0.652 |
| walker |  | 1916 | 9 | export names surface in packages/crypto/src/index.web.ts |  |  | 0.652 |
| walker |  | 1930 | 14 | export names surface in packages/crypto/src/api-definition.ts |  |  | 0.652 |
| ns | 2101 |  | 359 | @enclosed/lib public API — index.ts re-exports | 2.3 |  | 0.608 |
| ns | 2124 |  | 23 | Top-level app-server package contents | 2.4 |  | 0.617 |
| walker |  | 2125 | 195 | headings outline in README.md |  |  | 0.618 |
| ns | 2187 |  | 63 | Top-level app-client package contents | 2.5 |  | 0.588 |
| walker |  | 2223 | 98 | README.md section #0 |  |  | 0.588 |
| walker |  | 2241 | 18 | README.md section #32 |  |  | 0.588 |
| ns | 2259 |  | 72 | Top-level cli + crypto + docs package contents | 2.6 |  | 0.616 |
| walker |  | 2272 | 31 | README.md section #18 |  |  | 0.616 |
| walker |  | 2302 | 30 | README.md section #25 |  |  | 0.616 |
| walker |  | 2319 | 17 | README.md section #6 |  |  | 0.617 |
| walker |  | 2336 | 17 | README.md section #7 |  |  | 0.618 |
| walker |  | 2354 | 18 | README.md section #2 |  |  | 0.619 |
| walker |  | 2368 | 14 | listing of 'packages/docs/src/integrations' |  |  | 0.620 |
| walker |  | 2387 | 19 | README.md section #8 |  |  | 0.622 |
| walker |  | 2402 | 15 | listing of 'packages/app-server/src/modules' |  |  | 0.622 |
| walker |  | 2422 | 20 | README.md section #12 |  |  | 0.624 |
| walker |  | 2489 | 67 | export at packages/docs/src/data/i18n.data.ts:59 |  |  | 0.624 |
| walker |  | 2530 | 41 | headings outline in packages/docs/src/index.md |  |  | 0.624 |
| ns | 2545 |  | 286 | @enclosed/lib README — install + usage example | 2.7 |  | 0.579 |
| walker |  | 2551 | 21 | README.md section #5 |  |  | 0.582 |
| walker |  | 2572 | 21 | README.md section #9 |  |  | 0.585 |
| walker |  | 2587 | 15 | imports in packages/cli/build.config.ts |  |  | 0.585 |
| walker |  | 2602 | 15 | imports in packages/crypto/build.config.ts |  |  | 0.585 |
| walker |  | 2617 | 15 | imports in packages/lib/build.config.ts |  |  | 0.585 |
| walker |  | 2645 | 28 | export names surface in packages/lib/src/files/files.models.ts |  |  | 0.585 |
| walker |  | 2657 | 12 | listing of 'packages/app-server/src/modules/shared' |  |  | 0.573 |
| ns | 2657 |  | 112 | lib package.json — runtime dependencies | 2.8 |  | 0.573 |
| walker |  | 2661 | 4 | listing of 'packages/app-server/src/modules/shared/utils' |  |  | 0.573 |
| walker |  | 2672 | 11 | export names surface in packages/app-server/src/modules/shared/utils/random.ts |  |  | 0.573 |
| ns | 2678 |  | 21 | app-server/src — top-level entry layout | 3.1 |  | 0.580 |
| walker |  | 2695 | 23 | README.md section #11 |  |  | 0.584 |
| walker |  | 2704 | 9 | listing of 'packages/app-server/src/modules/shared/errors' |  |  | 0.584 |
| walker |  | 2713 | 9 | listing of 'packages/app-server/src/modules/shared/validation' |  |  | 0.584 |
| walker |  | 2726 | 13 | listing of 'packages/app-server/src/modules/storage' |  |  | 0.584 |
| walker |  | 2742 | 16 | export names surface in packages/app-server/src/modules/storage/storage.models.ts |  |  | 0.584 |
| walker |  | 2766 | 24 | README.md section #10 |  |  | 0.588 |
| walker |  | 2789 | 23 | packages/crypto/README.md section #3 |  |  | 0.588 |
| walker |  | 2809 | 20 | listing of 'packages/app-server/src/scripts' |  |  | 0.588 |
| walker |  | 2834 | 25 | README.md section #3 |  |  | 0.593 |
| ns | 2851 |  | 173 | app-server/src/modules — module map | 3.2 |  | 0.552 |
| walker |  | 2855 | 21 | listing of 'packages/cli/src/config' |  |  | 0.552 |
| walker |  | 2864 | 9 | export names surface in packages/cli/src/config/config.usecases.ts |  |  | 0.552 |
| walker |  | 2876 | 12 | export names surface in packages/cli/src/config/config.constants.ts |  |  | 0.552 |
| walker |  | 2890 | 14 | export names surface in packages/cli/src/config/config.command.ts |  |  | 0.552 |
| walker |  | 2914 | 24 | export names surface in packages/cli/src/config/config.models.ts |  |  | 0.552 |
| ns | 2947 |  | 96 | Cloudflare Workers entrypoint (full) | 3.3 |  | 0.542 |
| walker |  | 2951 | 37 | export at packages/cli/src/config/config.usecases.ts:3 |  |  | 0.542 |
| walker |  | 2972 | 21 | listing of 'packages/cli/src/create-note' |  |  | 0.542 |
| walker |  | 2985 | 13 | export names surface in packages/cli/src/create-note/create-note.usecases.ts |  |  | 0.542 |
| walker |  | 3000 | 15 | export names surface in packages/cli/src/create-note/create-note.command.ts |  |  | 0.542 |
| walker |  | 3021 | 21 | listing of 'packages/crypto/src/node' |  |  | 0.542 |
| walker |  | 3053 | 32 | export names surface in packages/crypto/src/node/crypto.node.usecases.ts |  |  | 0.542 |
| walker |  | 3074 | 21 | listing of 'packages/crypto/src/web' |  |  | 0.542 |
| walker |  | 3106 | 32 | export names surface in packages/crypto/src/web/crypto.web.usecases.ts |  |  | 0.542 |
| walker |  | 3127 | 21 | listing of 'packages/lib/src/api' |  |  | 0.542 |
| walker |  | 3138 | 11 | export names surface in packages/lib/src/api/api.client.ts |  |  | 0.542 |
| walker |  | 3156 | 18 | export names surface in packages/lib/src/api/api.constants.ts |  |  | 0.542 |
| walker |  | 3179 | 23 | export names surface in packages/lib/src/api/api.models.ts |  |  | 0.542 |
| walker |  | 3242 | 63 | listing of 'packages/app-client' |  |  | 0.581 |
| walker |  | 3253 | 11 | export names surface in packages/app-client/playwright.config.ts |  |  | 0.581 |
| walker |  | 3264 | 11 | export names surface in packages/app-client/uno.config.ts |  |  | 0.581 |
| walker |  | 3275 | 11 | export names surface in packages/app-client/vite.config.ts |  |  | 0.581 |
| walker |  | 3311 | 36 | README headline in packages/app-client/README.md |  |  | 0.581 |
| walker |  | 3322 | 11 | listing of 'packages/app-client/e2e-tests' |  |  | 0.581 |
| walker |  | 3348 | 26 | listing of 'packages/app-client/src' |  |  | 0.581 |
| walker |  | 3352 | 4 | listing of 'packages/app-client/src/assets' |  |  | 0.581 |
| walker |  | 3364 | 12 | listing of 'packages/app-client/src/scripts' |  |  | 0.581 |
| walker |  | 3399 | 35 | headings outline in packages/docs/src/resources/i18n.md |  |  | 0.581 |
| walker |  | 3418 | 19 | export names surface in packages/app-server/src/modules/shared/errors/errors.ts |  |  | 0.581 |
| walker |  | 3448 | 30 | packages/crypto/README.md section #2 |  |  | 0.581 |
| ns | 3451 |  | 504 | Hono createServer factory + middleware stack | 3.4 |  | 0.537 |
| walker |  | 3461 | 13 | imports in packages/app-server/src/reset.d.ts |  |  | 0.537 |
| walker |  | 3506 | 45 | export at packages/cli/src/config/config.constants.ts:3 |  |  | 0.537 |
| walker |  | 3529 | 23 | packages/lib/README.md section #4 |  |  | 0.537 |
| ns | 3749 |  | 298 | Notes routes — endpoint registrations (locations only) | 3.5 |  | 0.518 |
| walker |  | 3783 | 254 | export at packages/app-client/vite.config.ts:8 |  |  | 0.518 |
| walker |  | 3808 | 25 | listing of 'packages/lib/src/crypto' |  |  | 0.518 |
| walker |  | 3822 | 14 | export names surface in packages/lib/src/crypto/crypto.usecases.ts |  |  | 0.518 |
| walker |  | 3924 | 102 | headings outline in packages/cli/README.md |  |  | 0.518 |
| walker |  | 3949 | 25 | packages/cli/README.md section #2 |  |  | 0.518 |
| walker |  | 3972 | 23 | packages/cli/README.md section #7 |  |  | 0.518 |
| walker |  | 3999 | 27 | listing of 'packages/lib/src/notes' |  |  | 0.518 |
| walker |  | 4010 | 11 | export names surface in packages/lib/src/notes/notes.usecases.ts |  |  | 0.518 |
| walker |  | 4024 | 14 | export names surface in packages/lib/src/notes/notes.services.ts |  |  | 0.518 |
| walker |  | 4052 | 28 | export names surface in packages/lib/src/notes/notes.models.ts |  |  | 0.518 |
| walker |  | 4088 | 36 | export names surface in packages/lib/src/notes/notes.types.ts |  |  | 0.518 |
| walker |  | 4110 | 22 | export at packages/lib/src/notes/notes.types.ts:12 |  |  | 0.518 |
| ns | 4200 |  | 451 | Notes routes — POST /api/notes payload schema + validation handler | 3.6 | 3.5 | 0.485 |
| walker |  | 4222 | 112 | headings outline in packages/app-client/README.md |  |  | 0.485 |
| walker |  | 4222 | 0 | packages/app-client/README.md section #3 |  |  | 0.485 |
| walker |  | 4291 | 69 | packages/app-client/README.md section #0 |  |  | 0.485 |
| walker |  | 4318 | 27 | packages/app-client/README.md section #2 |  |  | 0.485 |
| walker |  | 4349 | 31 | README.md section #4 |  |  | 0.490 |
| walker |  | 4379 | 30 | packages/cli/README.md section #6 |  |  | 0.490 |
| walker |  | 4408 | 29 | listing of 'packages/app-client/src/modules' |  |  | 0.490 |
| walker |  | 4413 | 5 | listing of 'packages/app-client/src/modules/theme' |  |  | 0.490 |
| walker |  | 4419 | 6 | listing of 'packages/app-client/src/modules/ui' |  |  | 0.490 |
| walker |  | 4432 | 13 | export names surface in packages/app-client/src/modules/theme/theme.store.ts |  |  | 0.490 |
| walker |  | 4432 | 0 | export at packages/app-client/src/modules/theme/theme.store.ts:4 |  |  | 0.490 |
| walker |  | 4438 | 6 | listing of 'packages/app-client/src/modules/ui/layouts' |  |  | 0.490 |
| walker |  | 4449 | 11 | listing of 'packages/app-client/src/modules/docs' |  |  | 0.490 |
| walker |  | 4461 | 12 | export names surface in packages/app-client/src/modules/docs/docs.models.ts |  |  | 0.490 |
| walker |  | 4472 | 11 | listing of 'packages/app-client/src/modules/files' |  |  | 0.490 |
| walker |  | 4487 | 15 | listing of 'packages/app-client/src/modules/shared' |  |  | 0.490 |
| walker |  | 4491 | 4 | listing of 'packages/app-client/src/modules/shared/hooks' |  |  | 0.490 |
| walker |  | 4495 | 4 | listing of 'packages/app-client/src/modules/shared/style' |  |  | 0.490 |
| walker |  | 4500 | 5 | listing of 'packages/app-client/src/modules/shared/utils' |  |  | 0.490 |
| walker |  | 4511 | 11 | export names surface in packages/app-client/src/modules/shared/hooks/hooks.ts |  |  | 0.490 |
| walker |  | 4519 | 8 | listing of 'packages/app-client/src/modules/shared/files' |  |  | 0.490 |
| walker |  | 4534 | 15 | export names surface in packages/app-client/src/modules/shared/files/convert.ts |  |  | 0.490 |
| walker |  | 4560 | 26 | export names surface in packages/app-client/src/modules/files/files.models.ts |  |  | 0.490 |
| walker |  | 4580 | 20 | export names surface in packages/app-client/src/modules/shared/files/download.ts |  |  | 0.490 |
| walker |  | 4596 | 16 | listing of 'packages/app-client/src/modules/config' |  |  | 0.490 |
| walker |  | 4604 | 8 | export names surface in packages/app-client/src/modules/config/config.provider.tsx |  |  | 0.490 |
| walker |  | 4615 | 11 | export at packages/app-client/src/modules/config/config.provider.tsx:5 |  |  | 0.490 |
| walker |  | 4626 | 11 | export names surface in packages/app-client/src/modules/config/config.types.ts |  |  | 0.490 |
| walker |  | 4641 | 15 | export names surface in packages/app-client/src/modules/config/config.constants.ts |  |  | 0.490 |
| walker |  | 4659 | 18 | listing of 'packages/app-client/src/modules/auth' |  |  | 0.458 |
| ns | 4659 |  | 459 | Notes routes — GET /api/notes/:noteId + private-note auth gating | 3.7 | 3.5 | 0.458 |
| walker |  | 4667 | 8 | export names surface in packages/app-client/src/modules/auth/auth.models.ts |  |  | 0.458 |
| walker |  | 4680 | 13 | export at packages/app-client/src/modules/auth/auth.models.ts:1 |  |  | 0.458 |
| walker |  | 4690 | 10 | export names surface in packages/app-client/src/modules/auth/auth.services.ts |  |  | 0.458 |
| walker |  | 4696 | 6 | listing of 'packages/app-client/src/modules/auth/pages' |  |  | 0.458 |
| walker |  | 4712 | 16 | export names surface in packages/app-client/src/modules/auth/auth.store.ts |  |  | 0.458 |
| walker |  | 4712 | 0 | export at packages/app-client/src/modules/auth/auth.store.ts:6 |  |  | 0.458 |
| walker |  | 4728 | 16 | export names surface in packages/app-client/src/modules/auth/pages/login.page.tsx |  |  | 0.458 |
| walker |  | 4728 | 0 | export at packages/app-client/src/modules/auth/pages/login.page.tsx:41 |  |  | 0.458 |
| walker |  | 4751 | 23 | export names surface in packages/app-client/src/modules/shared/style/cn.ts |  |  | 0.458 |
| ns | 4821 |  | 162 | Notes routes — GET /api/notes/:noteId/exists | 3.8 | 3.5 | 0.449 |
| walker |  | 4917 | 166 | imports in packages/app-client/src/index.tsx |  |  | 0.449 |
| walker |  | 4932 | 15 | export names surface in packages/app-client/src/routes.tsx |  |  | 0.449 |
| walker |  | 4932 | 0 | export at packages/app-client/src/routes.tsx:11 |  |  | 0.449 |
| walker |  | 4963 | 31 | listing of 'packages/docs/src/self-hosting' |  |  | 0.449 |
| walker |  | 4990 | 27 | headings outline in packages/docs/src/self-hosting/troubleshooting.md |  |  | 0.449 |
| walker |  | 4990 | 0 | packages/docs/src/self-hosting/troubleshooting.md section #0 |  |  | 0.449 |
| walker |  | 5041 | 51 | headings outline in packages/docs/src/integrations/npm-package.md |  |  | 0.449 |
| ns | 5055 |  | 234 | Config — env var name catalog (locations only) | 4.1 |  | 0.435 |
| walker |  | 5091 | 50 | CONTRIBUTING.md section #1 |  |  | 0.435 |
| walker |  | 5121 | 30 | packages/lib/README.md section #3 |  |  | 0.435 |
| walker |  | 5303 | 182 | imports in packages/lib/src/index.ts |  |  | 0.476 |
| walker |  | 5332 | 29 | imports in packages/app-client/playwright.config.ts |  |  | 0.476 |
| walker |  | 5347 | 15 | listing of 'packages/app-server/src/modules/shared/logger' |  |  | 0.476 |
| walker |  | 5358 | 11 | export names surface in packages/app-server/src/modules/shared/logger/logger.ts |  |  | 0.476 |
| walker |  | 5371 | 13 | export names surface in packages/app-server/src/modules/shared/logger/logger.test-utils.ts |  |  | 0.476 |
| walker |  | 5371 | 0 | export at packages/app-server/src/modules/shared/logger/logger.test-utils.ts:3 |  |  | 0.476 |
| walker |  | 5385 | 14 | export names surface in packages/app-server/src/modules/shared/logger/logger.types.ts |  |  | 0.476 |
| walker |  | 5407 | 22 | listing of 'packages/app-server/src/modules/app' |  |  | 0.483 |
| walker |  | 5418 | 11 | export names surface in packages/app-server/src/modules/app/server.ts |  |  | 0.483 |
| walker |  | 5423 | 5 | listing of 'packages/app-server/src/modules/app/users' |  |  | 0.483 |
| walker |  | 5435 | 12 | export names surface in packages/app-server/src/modules/app/users/users.repository.ts |  |  | 0.483 |
| walker |  | 5457 | 22 | listing of 'packages/app-server/src/modules/tasks' |  |  | 0.490 |
| walker |  | 5468 | 11 | export names surface in packages/app-server/src/modules/tasks/tasks.models.ts |  |  | 0.490 |
| walker |  | 5480 | 12 | export names surface in packages/app-server/src/modules/tasks/task-scheduler.ts |  |  | 0.490 |
| walker |  | 5495 | 15 | export names surface in packages/app-server/src/modules/tasks/tasks.types.ts |  |  | 0.490 |
| walker |  | 5507 | 12 | imports in packages/cli/src/shared/cli.models.ts |  |  | 0.490 |
| walker |  | 5563 | 56 | headings outline in packages/docs/src/self-hosting/configuration.md |  |  | 0.490 |
| walker |  | 5586 | 23 | listing of 'packages/app-client/src/modules/i18n' |  |  | 0.490 |
| ns | 5587 |  | 532 | Config — defaults for the most-asked-about env vars | 4.2 | 4.1 | 0.466 |
| walker |  | 5620 | 34 | export names surface in packages/app-client/src/modules/i18n/i18n.models.ts |  |  | 0.466 |
| walker |  | 5620 | 0 | export at packages/app-client/src/modules/i18n/i18n.models.ts:8 |  |  | 0.466 |
| walker |  | 5643 | 23 | listing of 'packages/crypto/src/node/encryption-algorithms' |  |  | 0.466 |
| walker |  | 5663 | 20 | export names surface in packages/crypto/src/node/encryption-algorithms/crypto.node.aes-256-gcm.ts |  |  | 0.466 |
| walker |  | 5686 | 23 | listing of 'packages/crypto/src/web/encryption-algorithms' |  |  | 0.466 |
| walker |  | 5706 | 20 | export names surface in packages/crypto/src/web/encryption-algorithms/crypto.web.aes-256-gcm.ts |  |  | 0.466 |
| ns | 5731 |  | 144 | Config — getConfig export + figue setup | 4.3 | 4.1 | 0.458 |
| walker |  | 5744 | 38 | README.md section #1 |  |  | 0.463 |
| walker |  | 5757 | 13 | imports in packages/cli/src/config/config.constants.ts |  |  | 0.463 |
| walker |  | 5770 | 13 | imports in packages/cli/src/shared/http.models.ts |  |  | 0.463 |
| walker |  | 5794 | 24 | listing of 'packages/lib/src/crypto/encryption-algorithms' |  |  | 0.463 |
| walker |  | 5806 | 12 | export names surface in packages/lib/src/crypto/encryption-algorithms/encryption-algorithms.models.ts |  |  | 0.463 |
| walker |  | 5819 | 13 | export names surface in packages/lib/src/crypto/encryption-algorithms/encryption-algorithms.registry.ts |  |  | 0.463 |
| walker |  | 5836 | 17 | export names surface in packages/lib/src/crypto/encryption-algorithms/encryption-algorithms.types.ts |  |  | 0.463 |
| ns | 5875 |  | 144 | Node.js entrypoint — config + storage + server build | 4.4 |  | 0.457 |
| walker |  | 5876 | 40 | export names surface in packages/app-client/src/modules/i18n/i18n.provider.tsx |  |  | 0.457 |
| walker |  | 5876 | 0 | export at packages/app-client/src/modules/i18n/i18n.provider.tsx:44 |  |  | 0.457 |
| walker |  | 5889 | 13 | export at packages/app-client/src/modules/i18n/i18n.provider.tsx:10 |  |  | 0.457 |
| walker |  | 5976 | 87 | export at packages/cli/src/create-note/create-note.usecases.ts:3 |  |  | 0.457 |
| walker |  | 6025 | 49 | export body at packages/cli/src/create-note/create-note.usecases.ts:3 body 12 |  |  | 0.457 |
| walker |  | 6085 | 60 | json config renovate.json |  |  | 0.457 |
| walker |  | 6149 | 64 | listing of 'packages/app-client/public' |  |  | 0.457 |
| walker |  | 6175 | 26 | listing of 'packages/lib/src/crypto/serialization' |  |  | 0.457 |
| walker |  | 6187 | 12 | export names surface in packages/lib/src/crypto/serialization/serialization.models.ts |  |  | 0.457 |
| walker |  | 6200 | 13 | export names surface in packages/lib/src/crypto/serialization/serialization.test-utils.ts |  |  | 0.457 |
| ns | 6202 |  | 327 | Node.js entrypoint — static + SPA fallback + cron + listen | 4.5 | 4.4 | 0.440 |
| walker |  | 6234 | 34 | export names surface in packages/lib/src/crypto/serialization/serialization.types.ts |  |  | 0.440 |
| walker |  | 6249 | 15 | listing of 'packages/lib/src/crypto/serialization/cbor-array' |  |  | 0.440 |
| walker |  | 6267 | 18 | export names surface in packages/lib/src/crypto/serialization/cbor-array/cbor-array.serialization.ts |  |  | 0.440 |
| walker |  | 6282 | 15 | imports in packages/cli/src/config/config.usecases.ts |  |  | 0.440 |
| walker |  | 6297 | 15 | imports in packages/lib/src/notes/notes.services.ts |  |  | 0.440 |
| walker |  | 6313 | 16 | imports in packages/lib/src/api/api.models.ts |  |  | 0.440 |
| walker |  | 6362 | 49 | export names surface in packages/app-server/src/modules/app/server.types.ts |  |  | 0.440 |
| walker |  | 6433 | 71 | export names surface in packages/lib/src/crypto/crypto.types.ts |  |  | 0.440 |
| ns | 6457 |  | 255 | Storage drivers — memory + fs-lite | 4.6 |  | 0.430 |
| walker |  | 6470 | 37 | export at packages/lib/src/crypto/crypto.types.ts:7 |  |  | 0.430 |
| walker |  | 6487 | 17 | imports in packages/cli/src/view-note/view-note.models.ts |  |  | 0.430 |
| walker |  | 6516 | 29 | README.md section #31 |  |  | 0.430 |
| walker |  | 6653 | 137 | export at packages/crypto/src/index.node.ts:8 |  |  | 0.430 |
| walker |  | 6790 | 137 | export at packages/crypto/src/index.web.ts:8 |  |  | 0.430 |
| walker |  | 6864 | 74 | export names surface in packages/cli/src/files/files.services.ts |  |  | 0.430 |
| walker |  | 6864 | 0 | export at packages/cli/src/files/files.services.ts:5 |  |  | 0.430 |
| walker |  | 6864 | 0 | export at packages/cli/src/files/files.services.ts:14 |  |  | 0.430 |
| walker |  | 6864 | 0 | export at packages/cli/src/files/files.services.ts:26 |  |  | 0.430 |
| ns | 6892 |  | 435 | Storage driver — Cloudflare KV (with 413 translation) | 4.7 |  | 0.418 |
| walker |  | 6915 | 51 | export body at packages/cli/src/files/files.services.ts:5 body 6 |  |  | 0.418 |
| walker |  | 6942 | 27 | imports in packages/docs/.vitepress/plausible.ts |  |  | 0.418 |
| walker |  | 7024 | 82 | headings outline in packages/docs/src/self-hosting/other-platforms.md |  |  | 0.418 |
| walker |  | 7024 | 0 | packages/docs/src/self-hosting/other-platforms.md section #0 |  |  | 0.418 |
| walker |  | 7074 | 50 | listing of 'packages/crypto/src/encryption-algorithms' |  |  | 0.418 |
| walker |  | 7082 | 8 | export names surface in packages/crypto/src/encryption-algorithms/encryption-algorithms.test-utils.ts |  |  | 0.418 |
| walker |  | 7096 | 14 | export at packages/crypto/src/encryption-algorithms/encryption-algorithms.test-utils.ts:5 |  |  | 0.418 |
| walker |  | 7108 | 12 | export names surface in packages/crypto/src/encryption-algorithms/encryption-algorithms.models.ts |  |  | 0.418 |
| walker |  | 7121 | 13 | export names surface in packages/crypto/src/encryption-algorithms/encryption-algorithms.registry.ts |  |  | 0.418 |
| walker |  | 7165 | 44 | export names surface in packages/crypto/src/encryption-algorithms/encryption-algorithms.constants.ts |  |  | 0.418 |
| walker |  | 7217 | 52 | export names surface in packages/crypto/src/encryption-algorithms/encryption-algorithms.types.ts |  |  | 0.418 |
| walker |  | 7240 | 23 | listing of 'packages/app-client/src/modules/shared/http' |  |  | 0.418 |
| walker |  | 7251 | 11 | export names surface in packages/app-client/src/modules/shared/http/http-client.ts |  |  | 0.418 |
| walker |  | 7274 | 23 | export names surface in packages/app-client/src/modules/shared/http/http-client.models.ts |  |  | 0.418 |
| walker |  | 7300 | 26 | export names surface in packages/app-client/src/modules/shared/http/http-errors.ts |  |  | 0.418 |
| walker |  | 7349 | 49 | export at packages/lib/src/notes/notes.types.ts:4 |  |  | 0.418 |
| walker |  | 7371 | 22 | packages/docs/src/resources/i18n.md section #0 |  |  | 0.418 |
| ns | 7406 |  | 514 | Notes — models + types + constants | 5.1 |  | 0.402 |
| walker |  | 7423 | 52 | listing of 'packages/docs/src/public' |  |  | 0.402 |
| walker |  | 7454 | 31 | README.md section #30 |  |  | 0.402 |
| ns | 7835 |  | 429 | Notes repository — exports + factory + getRefreshedNote usecase | 5.2 |  | 0.387 |
| walker |  | 7999 | 545 | export at packages/app-client/playwright.config.ts:12 |  |  | 0.387 |
| walker |  | 8025 | 26 | export doc at packages/app-client/playwright.config.ts:12 |  |  | 0.387 |
| walker |  | 8050 | 25 | listing of 'packages/app-server/src/modules/app/config' |  |  | 0.387 |
| walker |  | 8061 | 11 | export names surface in packages/app-server/src/modules/app/config/config.test-utils.ts |  |  | 0.387 |
| walker |  | 8073 | 12 | export names surface in packages/app-server/src/modules/app/config/config.routes.ts |  |  | 0.387 |
| walker |  | 8087 | 14 | export names surface in packages/app-server/src/modules/app/config/config.models.ts |  |  | 0.387 |
| walker |  | 8101 | 14 | export names surface in packages/app-server/src/modules/app/config/config.types.ts |  |  | 0.387 |
| ns | 8111 |  | 276 | Auth catalog — errors + models + services signatures | 5.3 |  | 0.379 |
| walker |  | 8140 | 39 | export names surface in packages/app-server/src/modules/app/config/config.ts |  |  | 0.379 |
| walker |  | 8140 | 0 | export at packages/app-server/src/modules/app/config/config.ts:258 |  |  | 0.379 |
| walker |  | 8154 | 14 | imports in packages/app-client/src/modules/config/config.constants.ts |  |  | 0.379 |
| walker |  | 8266 | 112 | README.md section #24 |  |  | 0.379 |
| walker |  | 8342 | 76 | packages/crypto/README.md section #1 |  |  | 0.379 |
| walker |  | 8386 | 44 | export body at packages/app-server/src/modules/app/config/config.ts:258 body 259 |  |  | 0.383 |
| walker |  | 8425 | 39 | listing of 'packages/app-client/src/modules/notes' |  |  | 0.383 |
| ns | 8429 |  | 318 | Auth middleware — gating + protected-route guard | 5.4 |  | 0.375 |
| walker |  | 8438 | 13 | export names surface in packages/app-client/src/modules/notes/notes.usecases.ts |  |  | 0.375 |
| walker |  | 8458 | 20 | export names surface in packages/app-client/src/modules/notes/notes.services.ts |  |  | 0.375 |
| walker |  | 8489 | 31 | export names surface in packages/app-client/src/modules/notes/notes.constants.ts |  |  | 0.375 |
| walker |  | 8522 | 33 | export names surface in packages/app-client/src/modules/notes/notes.context.tsx |  |  | 0.375 |
| walker |  | 8522 | 0 | export at packages/app-client/src/modules/notes/notes.context.tsx:11 |  |  | 0.375 |
| walker |  | 8522 | 0 | export at packages/app-client/src/modules/notes/notes.context.tsx:27 |  |  | 0.375 |
| walker |  | 8536 | 14 | listing of 'packages/app-client/src/modules/notes/components' |  |  | 0.375 |
| walker |  | 8552 | 16 | export names surface in packages/app-client/src/modules/notes/components/file-uploader.tsx |  |  | 0.375 |
| walker |  | 8566 | 14 | listing of 'packages/app-client/src/modules/notes/pages' |  |  | 0.375 |
| walker |  | 8583 | 17 | export names surface in packages/app-client/src/modules/notes/pages/create-note.page.tsx |  |  | 0.375 |
| walker |  | 8583 | 0 | export at packages/app-client/src/modules/notes/pages/create-note.page.tsx:109 |  |  | 0.375 |
| walker |  | 8600 | 17 | export names surface in packages/app-client/src/modules/notes/pages/view-note.page.tsx |  |  | 0.375 |
| walker |  | 8600 | 0 | export at packages/app-client/src/modules/notes/pages/view-note.page.tsx:65 |  |  | 0.375 |
| walker |  | 8637 | 37 | export names surface in packages/app-client/src/modules/notes/notes.models.ts |  |  | 0.375 |
| walker |  | 8637 | 0 | export at packages/app-client/src/modules/notes/notes.models.ts:16 |  |  | 0.375 |
| walker |  | 8693 | 56 | export body at packages/app-client/src/modules/notes/notes.context.tsx:27 body 28 |  |  | 0.375 |
| walker |  | 8738 | 45 | export names surface in packages/app-client/src/modules/notes/components/note-password-field.tsx |  |  | 0.375 |
| walker |  | 8738 | 0 | export at packages/app-client/src/modules/notes/components/note-password-field.tsx:7 |  |  | 0.375 |
| walker |  | 8753 | 15 | imports in packages/app-server/src/modules/tasks/tasks.types.ts |  |  | 0.375 |
| walker |  | 8817 | 64 | export body at packages/app-client/src/modules/notes/notes.models.ts:16 body 17 |  |  | 0.375 |
| ns | 8821 |  | 392 | Auth login route — handler body (timing-safe bcrypt + JWT issue) | 5.5 |  | 0.365 |
| walker |  | 8848 | 31 | CONTRIBUTING.md section #8 |  |  | 0.365 |
| walker |  | 8897 | 49 | export names surface in packages/app-client/src/modules/ui/layouts/app.layout.tsx |  |  | 0.365 |
| walker |  | 8897 | 0 | export at packages/app-client/src/modules/ui/layouts/app.layout.tsx:72 |  |  | 0.365 |
| walker |  | 8897 | 0 | export at packages/app-client/src/modules/ui/layouts/app.layout.tsx:209 |  |  | 0.365 |
| walker |  | 8897 | 0 | export at packages/app-client/src/modules/ui/layouts/app.layout.tsx:240 |  |  | 0.365 |
| walker |  | 9001 | 104 | headings outline in packages/docs/src/resources/brand-kit.md |  |  | 0.365 |
| walker |  | 9024 | 23 | imports in packages/crypto/src/encryption-algorithms/encryption-algorithms.types.ts |  |  | 0.365 |
| walker |  | 9040 | 16 | imports in packages/app-client/src/modules/auth/auth.services.ts |  |  | 0.365 |
| walker |  | 9056 | 16 | imports in packages/app-client/src/modules/i18n/i18n.models.ts |  |  | 0.365 |
| walker |  | 9072 | 16 | imports in packages/app-client/src/modules/notes/notes.services.ts |  |  | 0.365 |
| walker |  | 9088 | 16 | imports in packages/lib/src/crypto/serialization/serialization.models.ts |  |  | 0.365 |
| ns | 9134 |  | 313 | Lib — encryptNote body (crypto.usecases) | 6.1 |  | 0.358 |
| walker |  | 9156 | 68 | export body at packages/app-client/src/modules/theme/theme.store.ts:4 body 5 |  |  | 0.358 |
| ns | 9232 |  | 98 | Lib — note URL hash-fragment markers (`pw` / `dar`) | 6.2 |  | 0.357 |
| walker |  | 9270 | 114 | README.md section #26 |  |  | 0.357 |
| walker |  | 9321 | 51 | export names surface in packages/app-client/src/modules/shared/utils/copy.tsx |  |  | 0.357 |
| walker |  | 9321 | 0 | export at packages/app-client/src/modules/shared/utils/copy.tsx:6 |  |  | 0.357 |
| walker |  | 9321 | 0 | export at packages/app-client/src/modules/shared/utils/copy.tsx:19 |  |  | 0.357 |
| walker |  | 9333 | 12 | imports in packages/app-server/src/modules/shared/logger/logger.ts |  |  | 0.357 |
| walker |  | 9377 | 44 | listing of 'packages/docs/src/public/logos' |  |  | 0.357 |
| walker |  | 9422 | 45 | export at packages/app-server/src/modules/app/server.types.ts:5 |  |  | 0.357 |
| walker |  | 9523 | 101 | export body at packages/cli/src/files/files.services.ts:14 body 15 |  |  | 0.357 |
| ns | 9539 |  | 307 | CLI dispatcher + create-note args | 6.3 |  | 0.350 |
| walker |  | 9585 | 62 | README.md section #13 |  |  | 0.356 |
| walker |  | 9609 | 24 | imports in packages/cli/src/create-note/create-note.usecases.ts |  |  | 0.356 |
| walker |  | 9712 | 103 | export body at packages/cli/src/files/files.services.ts:26 body 27 |  |  | 0.356 |
| walker |  | 9868 | 156 | plaintext config pnpm-workspace.yaml |  |  | 0.377 |
| ns | 9903 |  | 364 | App-client — Solid Router routes | 6.4 |  | 0.369 |
| walker |  | 9942 | 74 | export names surface in packages/app-server/src/modules/storage/storage.types.ts |  |  | 0.369 |
| walker |  | 9955 | 13 | imports in packages/app-client/src/modules/shared/http/http-errors.ts |  |  | 0.369 |
| ns | 9990 |  | 87 | Docs site — page map (VitePress src layout) | 6.5 |  | 0.384 |
| walker |  | 9993 | 38 | CONTRIBUTING.md section #7 |  |  | 0.384 |

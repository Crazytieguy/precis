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
| walker |  | 361 | 29 | listing of '.github' |  |  | 0.764 |
| walker |  | 375 | 14 | listing of '.github/ISSUE_TEMPLATE' |  |  | 0.764 |
| walker |  | 401 | 26 | listing of 'packages/lib' |  |  | 0.769 |
| walker |  | 417 | 16 | listing of 'packages/lib/src' |  |  | 0.773 |
| walker |  | 425 | 8 | export names surface in packages/lib/src/index.ts |  |  | 0.773 |
| walker |  | 437 | 12 | export names surface in packages/lib/build.config.ts |  |  | 0.773 |
| ns | 497 |  | 140 | pnpm-workspace.yaml — catalog versions | 1.7 | 1.4 | 0.690 |
| walker |  | 601 | 164 | export at packages/lib/src/index.ts:10 |  |  | 0.694 |
| walker |  | 606 | 5 | listing of 'packages/lib/src/files' |  |  | 0.694 |
| walker |  | 658 | 52 | README headline in packages/lib/README.md |  |  | 0.695 |
| walker |  | 675 | 17 | listing of 'packages/docs' |  |  | 0.695 |
| walker |  | 686 | 11 | listing of 'packages/docs/.vitepress' |  |  | 0.695 |
| ns | 694 |  | 197 | Root package.json — scripts + keywords + devDeps | 1.8 | 1.6 | 0.613 |
| walker |  | 697 | 11 | export names surface in packages/docs/.vitepress/config.ts |  |  | 0.613 |
| walker |  | 711 | 14 | export names surface in packages/docs/.vitepress/plausible.ts |  |  | 0.613 |
| walker |  | 711 | 0 | export at packages/docs/.vitepress/plausible.ts:8 |  |  | 0.613 |
| walker |  | 734 | 23 | listing of 'packages/app-server' |  |  | 0.616 |
| walker |  | 815 | 81 | export at packages/lib/build.config.ts:3 |  |  | 0.616 |
| walker |  | 836 | 21 | listing of 'packages/app-server/src' |  |  | 0.618 |
| walker |  | 844 | 8 | export names surface in packages/app-server/src/index.cloudflare.ts |  |  | 0.618 |
| walker |  | 844 | 0 | export at packages/app-server/src/index.cloudflare.ts:8 |  |  | 0.618 |
| ns | 872 |  | 178 | README — project structure (per-package one-liners) | 1.9 |  | 0.571 |
| walker |  | 922 | 78 | package scripts in package.json |  |  | 0.659 |
| walker |  | 947 | 25 | listing of 'packages/deploy-cloudflare' |  |  | 0.659 |
| walker |  | 973 | 26 | listing of 'packages/crypto' |  |  | 0.661 |
| walker |  | 985 | 12 | export names surface in packages/crypto/build.config.ts |  |  | 0.661 |
| walker |  | 1049 | 64 | README headline in packages/crypto/README.md |  |  | 0.661 |
| walker |  | 1080 | 31 | headings outline in packages/crypto/README.md |  |  | 0.661 |
| walker |  | 1172 | 92 | export at packages/crypto/build.config.ts:3 |  |  | 0.661 |
| walker |  | 1212 | 40 | headings outline in packages/lib/README.md |  |  | 0.662 |
| ns | 1225 |  | 353 | README — features list | 1.10 |  | 0.595 |
| walker |  | 1241 | 29 | listing of 'packages/cli' |  |  | 0.599 |
| walker |  | 1253 | 12 | export names surface in packages/cli/build.config.ts |  |  | 0.599 |
| walker |  | 1258 | 5 | listing of 'packages/cli/bin' |  |  | 0.599 |
| walker |  | 1308 | 50 | README headline in packages/cli/README.md |  |  | 0.599 |
| walker |  | 1370 | 62 | export at packages/cli/build.config.ts:3 |  |  | 0.599 |
| walker |  | 1391 | 21 | listing of 'packages/cli/src' |  |  | 0.599 |
| walker |  | 1401 | 10 | listing of 'packages/cli/src/shared' |  |  | 0.599 |
| walker |  | 1414 | 13 | export names surface in packages/cli/src/shared/cli.models.ts |  |  | 0.599 |
| walker |  | 1414 | 0 | export at packages/cli/src/shared/cli.models.ts:3 |  |  | 0.599 |
| walker |  | 1428 | 14 | export names surface in packages/cli/src/shared/http.models.ts |  |  | 0.599 |
| walker |  | 1428 | 0 | export at packages/cli/src/shared/http.models.ts:3 |  |  | 0.599 |
| ns | 1499 |  | 274 | README — how-it-works (creator side) | 1.11 |  | 0.566 |
| walker |  | 1599 | 171 | headings outline in CONTRIBUTING.md |  |  | 0.566 |
| walker |  | 1635 | 36 | CONTRIBUTING.md section #0 |  |  | 0.566 |
| walker |  | 1646 | 11 | listing of 'packages/cli/src/files' |  |  | 0.566 |
| ns | 1700 |  | 201 | README — how-it-works (recipient side) | 1.12 | 1.11 | 0.545 |
| ns | 1726 |  | 26 | Top-level lib package contents | 2.1 |  | 0.565 |
| walker |  | 1729 | 83 | package dependencies in package.json |  |  | 0.640 |
| ns | 1742 |  | 16 | lib/src — top-level source modules | 2.2 |  | 0.651 |
| walker |  | 1760 | 31 | listing of 'packages/docs/src' |  |  | 0.652 |
| walker |  | 1770 | 10 | listing of 'packages/docs/src/components' |  |  | 0.652 |
| walker |  | 1781 | 11 | listing of 'packages/docs/src/resources' |  |  | 0.652 |
| walker |  | 1793 | 12 | listing of 'packages/cli/src/view-note' |  |  | 0.652 |
| walker |  | 1805 | 12 | export names surface in packages/cli/src/view-note/view-note.models.ts |  |  | 0.652 |
| walker |  | 1805 | 0 | export at packages/cli/src/view-note/view-note.models.ts:3 |  |  | 0.652 |
| walker |  | 1820 | 15 | export names surface in packages/cli/src/view-note/view-note.command.ts |  |  | 0.652 |
| walker |  | 1832 | 12 | listing of 'packages/docs/.vitepress/theme' |  |  | 0.652 |
| walker |  | 1844 | 12 | listing of 'packages/docs/src/data' |  |  | 0.652 |
| walker |  | 1853 | 9 | export names surface in packages/docs/src/data/configuration.data.ts |  |  | 0.652 |
| walker |  | 1862 | 9 | export names surface in packages/docs/src/data/i18n.data.ts |  |  | 0.652 |
| walker |  | 1909 | 47 | export at packages/docs/src/data/configuration.data.ts:55 |  |  | 0.652 |
| walker |  | 1941 | 32 | listing of 'packages/crypto/src' |  |  | 0.652 |
| walker |  | 1950 | 9 | export names surface in packages/crypto/src/index.node.ts |  |  | 0.652 |
| walker |  | 1959 | 9 | export names surface in packages/crypto/src/index.web.ts |  |  | 0.652 |
| walker |  | 1973 | 14 | export names surface in packages/crypto/src/api-definition.ts |  |  | 0.652 |
| walker |  | 1973 | 0 | export at packages/crypto/src/api-definition.ts:4 |  |  | 0.652 |
| ns | 2101 |  | 359 | @enclosed/lib public API — index.ts re-exports | 2.3 |  | 0.608 |
| ns | 2124 |  | 23 | Top-level app-server package contents | 2.4 |  | 0.617 |
| walker |  | 2168 | 195 | headings outline in README.md |  |  | 0.618 |
| ns | 2187 |  | 63 | Top-level app-client package contents | 2.5 |  | 0.588 |
| ns | 2259 |  | 72 | Top-level cli + crypto + docs package contents | 2.6 |  | 0.616 |
| walker |  | 2266 | 98 | README.md section #0 |  |  | 0.616 |
| walker |  | 2284 | 18 | README.md section #32 |  |  | 0.616 |
| walker |  | 2315 | 31 | README.md section #18 |  |  | 0.616 |
| walker |  | 2345 | 30 | README.md section #25 |  |  | 0.616 |
| walker |  | 2362 | 17 | README.md section #6 |  |  | 0.617 |
| walker |  | 2379 | 17 | README.md section #7 |  |  | 0.618 |
| walker |  | 2397 | 18 | README.md section #2 |  |  | 0.619 |
| walker |  | 2411 | 14 | listing of 'packages/docs/src/integrations' |  |  | 0.620 |
| walker |  | 2430 | 19 | README.md section #8 |  |  | 0.622 |
| walker |  | 2445 | 15 | listing of 'packages/app-server/src/modules' |  |  | 0.622 |
| walker |  | 2465 | 20 | README.md section #12 |  |  | 0.624 |
| walker |  | 2532 | 67 | export at packages/docs/src/data/i18n.data.ts:59 |  |  | 0.624 |
| ns | 2545 |  | 286 | @enclosed/lib README — install + usage example | 2.7 |  | 0.579 |
| walker |  | 2573 | 41 | headings outline in packages/docs/src/index.md |  |  | 0.579 |
| walker |  | 2594 | 21 | README.md section #5 |  |  | 0.582 |
| walker |  | 2615 | 21 | README.md section #9 |  |  | 0.585 |
| walker |  | 2630 | 15 | imports in packages/cli/build.config.ts |  |  | 0.585 |
| walker |  | 2645 | 15 | imports in packages/crypto/build.config.ts |  |  | 0.585 |
| ns | 2657 |  | 112 | lib package.json — runtime dependencies | 2.8 |  | 0.573 |
| walker |  | 2660 | 15 | imports in packages/lib/build.config.ts |  |  | 0.573 |
| ns | 2678 |  | 21 | app-server/src — top-level entry layout | 3.1 |  | 0.580 |
| walker |  | 2688 | 28 | export names surface in packages/lib/src/files/files.models.ts |  |  | 0.580 |
| walker |  | 2688 | 0 | export at packages/lib/src/files/files.models.ts:4 |  |  | 0.580 |
| walker |  | 2700 | 12 | listing of 'packages/app-server/src/modules/shared' |  |  | 0.580 |
| walker |  | 2704 | 4 | listing of 'packages/app-server/src/modules/shared/utils' |  |  | 0.580 |
| walker |  | 2715 | 11 | export names surface in packages/app-server/src/modules/shared/utils/random.ts |  |  | 0.580 |
| walker |  | 2715 | 0 | export at packages/app-server/src/modules/shared/utils/random.ts:3 |  |  | 0.580 |
| walker |  | 2738 | 23 | README.md section #11 |  |  | 0.584 |
| walker |  | 2747 | 9 | listing of 'packages/app-server/src/modules/shared/errors' |  |  | 0.584 |
| walker |  | 2756 | 9 | listing of 'packages/app-server/src/modules/shared/validation' |  |  | 0.584 |
| walker |  | 2769 | 13 | listing of 'packages/app-server/src/modules/storage' |  |  | 0.584 |
| walker |  | 2785 | 16 | export names surface in packages/app-server/src/modules/storage/storage.models.ts |  |  | 0.584 |
| walker |  | 2785 | 0 | export at packages/app-server/src/modules/storage/storage.models.ts:3 |  |  | 0.584 |
| walker |  | 2809 | 24 | README.md section #10 |  |  | 0.588 |
| walker |  | 2832 | 23 | packages/crypto/README.md section #3 |  |  | 0.588 |
| ns | 2851 |  | 173 | app-server/src/modules — module map | 3.2 |  | 0.547 |
| walker |  | 2852 | 20 | listing of 'packages/app-server/src/scripts' |  |  | 0.547 |
| walker |  | 2877 | 25 | README.md section #3 |  |  | 0.552 |
| walker |  | 2898 | 21 | listing of 'packages/cli/src/config' |  |  | 0.552 |
| walker |  | 2907 | 9 | export names surface in packages/cli/src/config/config.usecases.ts |  |  | 0.552 |
| walker |  | 2919 | 12 | export names surface in packages/cli/src/config/config.constants.ts |  |  | 0.552 |
| walker |  | 2933 | 14 | export names surface in packages/cli/src/config/config.command.ts |  |  | 0.552 |
| ns | 2947 |  | 96 | Cloudflare Workers entrypoint (full) | 3.3 |  | 0.542 |
| walker |  | 2957 | 24 | export names surface in packages/cli/src/config/config.models.ts |  |  | 0.542 |
| walker |  | 2957 | 0 | export at packages/cli/src/config/config.models.ts:5 |  |  | 0.542 |
| walker |  | 2994 | 37 | export at packages/cli/src/config/config.usecases.ts:3 |  |  | 0.542 |
| walker |  | 3015 | 21 | listing of 'packages/cli/src/create-note' |  |  | 0.542 |
| walker |  | 3028 | 13 | export names surface in packages/cli/src/create-note/create-note.usecases.ts |  |  | 0.542 |
| walker |  | 3043 | 15 | export names surface in packages/cli/src/create-note/create-note.command.ts |  |  | 0.542 |
| walker |  | 3064 | 21 | listing of 'packages/crypto/src/node' |  |  | 0.542 |
| walker |  | 3096 | 32 | export names surface in packages/crypto/src/node/crypto.node.usecases.ts |  |  | 0.542 |
| walker |  | 3096 | 0 | export at packages/crypto/src/node/crypto.node.usecases.ts:5 |  |  | 0.542 |
| walker |  | 3117 | 21 | listing of 'packages/crypto/src/web' |  |  | 0.542 |
| walker |  | 3149 | 32 | export names surface in packages/crypto/src/web/crypto.web.usecases.ts |  |  | 0.542 |
| walker |  | 3149 | 0 | export at packages/crypto/src/web/crypto.web.usecases.ts:1 |  |  | 0.542 |
| walker |  | 3170 | 21 | listing of 'packages/lib/src/api' |  |  | 0.542 |
| walker |  | 3181 | 11 | export names surface in packages/lib/src/api/api.client.ts |  |  | 0.542 |
| walker |  | 3181 | 0 | export at packages/lib/src/api/api.client.ts:4 |  |  | 0.542 |
| walker |  | 3199 | 18 | export names surface in packages/lib/src/api/api.constants.ts |  |  | 0.542 |
| walker |  | 3199 | 0 | export at packages/lib/src/api/api.constants.ts:1 |  |  | 0.542 |
| walker |  | 3222 | 23 | export names surface in packages/lib/src/api/api.models.ts |  |  | 0.542 |
| walker |  | 3222 | 0 | export at packages/lib/src/api/api.models.ts:3 |  |  | 0.542 |
| walker |  | 3285 | 63 | listing of 'packages/app-client' |  |  | 0.581 |
| walker |  | 3296 | 11 | export names surface in packages/app-client/playwright.config.ts |  |  | 0.581 |
| walker |  | 3307 | 11 | export names surface in packages/app-client/uno.config.ts |  |  | 0.581 |
| walker |  | 3318 | 11 | export names surface in packages/app-client/vite.config.ts |  |  | 0.581 |
| walker |  | 3354 | 36 | README headline in packages/app-client/README.md |  |  | 0.581 |
| walker |  | 3365 | 11 | listing of 'packages/app-client/e2e-tests' |  |  | 0.581 |
| walker |  | 3391 | 26 | listing of 'packages/app-client/src' |  |  | 0.581 |
| walker |  | 3395 | 4 | listing of 'packages/app-client/src/assets' |  |  | 0.581 |
| walker |  | 3407 | 12 | listing of 'packages/app-client/src/scripts' |  |  | 0.581 |
| walker |  | 3442 | 35 | headings outline in packages/docs/src/resources/i18n.md |  |  | 0.581 |
| ns | 3451 |  | 504 | Hono createServer factory + middleware stack | 3.4 |  | 0.537 |
| walker |  | 3461 | 19 | export names surface in packages/app-server/src/modules/shared/errors/errors.ts |  |  | 0.537 |
| walker |  | 3461 | 0 | export at packages/app-server/src/modules/shared/errors/errors.ts:4 |  |  | 0.537 |
| walker |  | 3491 | 30 | packages/crypto/README.md section #2 |  |  | 0.537 |
| walker |  | 3504 | 13 | imports in packages/app-server/src/reset.d.ts |  |  | 0.537 |
| walker |  | 3549 | 45 | export at packages/cli/src/config/config.constants.ts:3 |  |  | 0.537 |
| walker |  | 3572 | 23 | packages/lib/README.md section #4 |  |  | 0.537 |
| ns | 3749 |  | 298 | Notes routes — endpoint registrations (locations only) | 3.5 |  | 0.518 |
| walker |  | 3826 | 254 | export at packages/app-client/vite.config.ts:8 |  |  | 0.518 |
| walker |  | 3851 | 25 | listing of 'packages/lib/src/crypto' |  |  | 0.518 |
| walker |  | 3865 | 14 | export names surface in packages/lib/src/crypto/crypto.usecases.ts |  |  | 0.518 |
| walker |  | 3865 | 0 | export at packages/lib/src/crypto/crypto.usecases.ts:7 |  |  | 0.518 |
| walker |  | 3967 | 102 | headings outline in packages/cli/README.md |  |  | 0.518 |
| walker |  | 3992 | 25 | packages/cli/README.md section #2 |  |  | 0.518 |
| walker |  | 4015 | 23 | packages/cli/README.md section #7 |  |  | 0.518 |
| walker |  | 4042 | 27 | listing of 'packages/lib/src/notes' |  |  | 0.518 |
| walker |  | 4053 | 11 | export names surface in packages/lib/src/notes/notes.usecases.ts |  |  | 0.518 |
| walker |  | 4053 | 0 | export at packages/lib/src/notes/notes.usecases.ts:8 |  |  | 0.518 |
| walker |  | 4067 | 14 | export names surface in packages/lib/src/notes/notes.services.ts |  |  | 0.518 |
| walker |  | 4067 | 0 | export at packages/lib/src/notes/notes.services.ts:3 |  |  | 0.518 |
| walker |  | 4095 | 28 | export names surface in packages/lib/src/notes/notes.models.ts |  |  | 0.518 |
| walker |  | 4095 | 0 | export at packages/lib/src/notes/notes.models.ts:4 |  |  | 0.518 |
| walker |  | 4131 | 36 | export names surface in packages/lib/src/notes/notes.types.ts |  |  | 0.518 |
| walker |  | 4153 | 22 | export at packages/lib/src/notes/notes.types.ts:12 |  |  | 0.518 |
| ns | 4200 |  | 451 | Notes routes — POST /api/notes payload schema + validation handler | 3.6 | 3.5 | 0.485 |
| walker |  | 4234 | 81 | listing of '.github/workflows' |  |  | 0.485 |
| walker |  | 4346 | 112 | headings outline in packages/app-client/README.md |  |  | 0.485 |
| walker |  | 4346 | 0 | packages/app-client/README.md section #3 |  |  | 0.485 |
| walker |  | 4415 | 69 | packages/app-client/README.md section #0 |  |  | 0.485 |
| walker |  | 4442 | 27 | packages/app-client/README.md section #2 |  |  | 0.485 |
| walker |  | 4473 | 31 | README.md section #4 |  |  | 0.490 |
| walker |  | 4503 | 30 | packages/cli/README.md section #6 |  |  | 0.490 |
| walker |  | 4532 | 29 | listing of 'packages/app-client/src/modules' |  |  | 0.490 |
| walker |  | 4537 | 5 | listing of 'packages/app-client/src/modules/theme' |  |  | 0.490 |
| walker |  | 4543 | 6 | listing of 'packages/app-client/src/modules/ui' |  |  | 0.490 |
| walker |  | 4556 | 13 | export names surface in packages/app-client/src/modules/theme/theme.store.ts |  |  | 0.490 |
| walker |  | 4556 | 0 | export at packages/app-client/src/modules/theme/theme.store.ts:4 |  |  | 0.490 |
| walker |  | 4562 | 6 | listing of 'packages/app-client/src/modules/ui/layouts' |  |  | 0.490 |
| walker |  | 4573 | 11 | listing of 'packages/app-client/src/modules/docs' |  |  | 0.490 |
| walker |  | 4585 | 12 | export names surface in packages/app-client/src/modules/docs/docs.models.ts |  |  | 0.490 |
| walker |  | 4585 | 0 | export at packages/app-client/src/modules/docs/docs.models.ts:4 |  |  | 0.490 |
| walker |  | 4596 | 11 | listing of 'packages/app-client/src/modules/files' |  |  | 0.490 |
| walker |  | 4611 | 15 | listing of 'packages/app-client/src/modules/shared' |  |  | 0.490 |
| walker |  | 4615 | 4 | listing of 'packages/app-client/src/modules/shared/hooks' |  |  | 0.490 |
| walker |  | 4619 | 4 | listing of 'packages/app-client/src/modules/shared/style' |  |  | 0.490 |
| walker |  | 4624 | 5 | listing of 'packages/app-client/src/modules/shared/utils' |  |  | 0.490 |
| walker |  | 4635 | 11 | export names surface in packages/app-client/src/modules/shared/hooks/hooks.ts |  |  | 0.490 |
| walker |  | 4635 | 0 | export at packages/app-client/src/modules/shared/hooks/hooks.ts:1 |  |  | 0.490 |
| walker |  | 4643 | 8 | listing of 'packages/app-client/src/modules/shared/files' |  |  | 0.490 |
| walker |  | 4658 | 15 | export names surface in packages/app-client/src/modules/shared/files/convert.ts |  |  | 0.490 |
| walker |  | 4658 | 0 | export at packages/app-client/src/modules/shared/files/convert.ts:3 |  |  | 0.490 |
| ns | 4659 |  | 459 | Notes routes — GET /api/notes/:noteId + private-note auth gating | 3.7 | 3.5 | 0.458 |
| walker |  | 4684 | 26 | export names surface in packages/app-client/src/modules/files/files.models.ts |  |  | 0.458 |
| walker |  | 4684 | 0 | export at packages/app-client/src/modules/files/files.models.ts:1 |  |  | 0.458 |
| walker |  | 4704 | 20 | export names surface in packages/app-client/src/modules/shared/files/download.ts |  |  | 0.458 |
| walker |  | 4704 | 0 | export at packages/app-client/src/modules/shared/files/download.ts:1 |  |  | 0.458 |
| walker |  | 4720 | 16 | listing of 'packages/app-client/src/modules/config' |  |  | 0.458 |
| walker |  | 4728 | 8 | export names surface in packages/app-client/src/modules/config/config.provider.tsx |  |  | 0.458 |
| walker |  | 4739 | 11 | export at packages/app-client/src/modules/config/config.provider.tsx:5 |  |  | 0.458 |
| walker |  | 4750 | 11 | export names surface in packages/app-client/src/modules/config/config.types.ts |  |  | 0.458 |
| walker |  | 4765 | 15 | export names surface in packages/app-client/src/modules/config/config.constants.ts |  |  | 0.458 |
| walker |  | 4783 | 18 | listing of 'packages/app-client/src/modules/auth' |  |  | 0.458 |
| walker |  | 4791 | 8 | export names surface in packages/app-client/src/modules/auth/auth.models.ts |  |  | 0.458 |
| walker |  | 4804 | 13 | export at packages/app-client/src/modules/auth/auth.models.ts:1 |  |  | 0.458 |
| walker |  | 4814 | 10 | export names surface in packages/app-client/src/modules/auth/auth.services.ts |  |  | 0.458 |
| walker |  | 4814 | 0 | export at packages/app-client/src/modules/auth/auth.services.ts:3 |  |  | 0.458 |
| walker |  | 4820 | 6 | listing of 'packages/app-client/src/modules/auth/pages' |  |  | 0.458 |
| ns | 4821 |  | 162 | Notes routes — GET /api/notes/:noteId/exists | 3.8 | 3.5 | 0.449 |
| walker |  | 4836 | 16 | export names surface in packages/app-client/src/modules/auth/auth.store.ts |  |  | 0.449 |
| walker |  | 4836 | 0 | export at packages/app-client/src/modules/auth/auth.store.ts:6 |  |  | 0.449 |
| walker |  | 4852 | 16 | export names surface in packages/app-client/src/modules/auth/pages/login.page.tsx |  |  | 0.449 |
| walker |  | 4852 | 0 | export at packages/app-client/src/modules/auth/pages/login.page.tsx:41 |  |  | 0.449 |
| walker |  | 4875 | 23 | export names surface in packages/app-client/src/modules/shared/style/cn.ts |  |  | 0.449 |
| walker |  | 4875 | 0 | export at packages/app-client/src/modules/shared/style/cn.ts:5 |  |  | 0.449 |
| walker |  | 5041 | 166 | imports in packages/app-client/src/index.tsx |  |  | 0.449 |
| ns | 5055 |  | 234 | Config — env var name catalog (locations only) | 4.1 |  | 0.434 |
| walker |  | 5056 | 15 | export names surface in packages/app-client/src/routes.tsx |  |  | 0.434 |
| walker |  | 5056 | 0 | export at packages/app-client/src/routes.tsx:11 |  |  | 0.434 |
| walker |  | 5087 | 31 | listing of 'packages/docs/src/self-hosting' |  |  | 0.435 |
| walker |  | 5114 | 27 | headings outline in packages/docs/src/self-hosting/troubleshooting.md |  |  | 0.435 |
| walker |  | 5114 | 0 | packages/docs/src/self-hosting/troubleshooting.md section #0 |  |  | 0.435 |
| walker |  | 5165 | 51 | headings outline in packages/docs/src/integrations/npm-package.md |  |  | 0.435 |
| walker |  | 5215 | 50 | CONTRIBUTING.md section #1 |  |  | 0.435 |
| walker |  | 5245 | 30 | packages/lib/README.md section #3 |  |  | 0.435 |
| walker |  | 5427 | 182 | imports in packages/lib/src/index.ts |  |  | 0.476 |
| walker |  | 5456 | 29 | imports in packages/app-client/playwright.config.ts |  |  | 0.476 |
| walker |  | 5471 | 15 | listing of 'packages/app-server/src/modules/shared/logger' |  |  | 0.476 |
| walker |  | 5482 | 11 | export names surface in packages/app-server/src/modules/shared/logger/logger.ts |  |  | 0.476 |
| walker |  | 5482 | 0 | export at packages/app-server/src/modules/shared/logger/logger.ts:3 |  |  | 0.476 |
| walker |  | 5495 | 13 | export names surface in packages/app-server/src/modules/shared/logger/logger.test-utils.ts |  |  | 0.476 |
| walker |  | 5495 | 0 | export at packages/app-server/src/modules/shared/logger/logger.test-utils.ts:3 |  |  | 0.476 |
| walker |  | 5509 | 14 | export names surface in packages/app-server/src/modules/shared/logger/logger.types.ts |  |  | 0.476 |
| walker |  | 5509 | 0 | export at packages/app-server/src/modules/shared/logger/logger.types.ts:3 |  |  | 0.476 |
| walker |  | 5531 | 22 | listing of 'packages/app-server/src/modules/app' |  |  | 0.483 |
| walker |  | 5542 | 11 | export names surface in packages/app-server/src/modules/app/server.ts |  |  | 0.483 |
| walker |  | 5542 | 0 | export at packages/app-server/src/modules/app/server.ts:17 |  |  | 0.483 |
| walker |  | 5547 | 5 | listing of 'packages/app-server/src/modules/app/users' |  |  | 0.483 |
| walker |  | 5559 | 12 | export names surface in packages/app-server/src/modules/app/users/users.repository.ts |  |  | 0.483 |
| walker |  | 5559 | 0 | export at packages/app-server/src/modules/app/users/users.repository.ts:4 |  |  | 0.483 |
| walker |  | 5581 | 22 | listing of 'packages/app-server/src/modules/tasks' |  |  | 0.490 |
| ns | 5587 |  | 532 | Config — defaults for the most-asked-about env vars | 4.2 | 4.1 | 0.466 |
| walker |  | 5592 | 11 | export names surface in packages/app-server/src/modules/tasks/tasks.models.ts |  |  | 0.466 |
| walker |  | 5592 | 0 | export at packages/app-server/src/modules/tasks/tasks.models.ts:7 |  |  | 0.466 |
| walker |  | 5604 | 12 | export names surface in packages/app-server/src/modules/tasks/task-scheduler.ts |  |  | 0.466 |
| walker |  | 5604 | 0 | export at packages/app-server/src/modules/tasks/task-scheduler.ts:7 |  |  | 0.466 |
| walker |  | 5619 | 15 | export names surface in packages/app-server/src/modules/tasks/tasks.types.ts |  |  | 0.466 |
| walker |  | 5619 | 0 | export at packages/app-server/src/modules/tasks/tasks.types.ts:3 |  |  | 0.466 |
| walker |  | 5631 | 12 | imports in packages/cli/src/shared/cli.models.ts |  |  | 0.466 |
| walker |  | 5687 | 56 | headings outline in packages/docs/src/self-hosting/configuration.md |  |  | 0.466 |
| walker |  | 5710 | 23 | listing of 'packages/app-client/src/modules/i18n' |  |  | 0.466 |
| ns | 5731 |  | 144 | Config — getConfig export + figue setup | 4.3 | 4.1 | 0.458 |
| walker |  | 5744 | 34 | export names surface in packages/app-client/src/modules/i18n/i18n.models.ts |  |  | 0.458 |
| walker |  | 5744 | 0 | export at packages/app-client/src/modules/i18n/i18n.models.ts:8 |  |  | 0.458 |
| walker |  | 5767 | 23 | listing of 'packages/crypto/src/node/encryption-algorithms' |  |  | 0.458 |
| walker |  | 5787 | 20 | export names surface in packages/crypto/src/node/encryption-algorithms/crypto.node.aes-256-gcm.ts |  |  | 0.458 |
| walker |  | 5810 | 23 | listing of 'packages/crypto/src/web/encryption-algorithms' |  |  | 0.458 |
| walker |  | 5830 | 20 | export names surface in packages/crypto/src/web/encryption-algorithms/crypto.web.aes-256-gcm.ts |  |  | 0.458 |
| walker |  | 5868 | 38 | README.md section #1 |  |  | 0.463 |
| ns | 5875 |  | 144 | Node.js entrypoint — config + storage + server build | 4.4 |  | 0.457 |
| walker |  | 5881 | 13 | imports in packages/cli/src/config/config.constants.ts |  |  | 0.457 |
| walker |  | 5894 | 13 | imports in packages/cli/src/shared/http.models.ts |  |  | 0.457 |
| walker |  | 5918 | 24 | listing of 'packages/lib/src/crypto/encryption-algorithms' |  |  | 0.457 |
| walker |  | 5930 | 12 | export names surface in packages/lib/src/crypto/encryption-algorithms/encryption-algorithms.models.ts |  |  | 0.457 |
| walker |  | 5930 | 0 | export at packages/lib/src/crypto/encryption-algorithms/encryption-algorithms.models.ts:1 |  |  | 0.457 |
| walker |  | 5943 | 13 | export names surface in packages/lib/src/crypto/encryption-algorithms/encryption-algorithms.registry.ts |  |  | 0.457 |
| walker |  | 5943 | 0 | export at packages/lib/src/crypto/encryption-algorithms/encryption-algorithms.registry.ts:4 |  |  | 0.457 |
| walker |  | 5960 | 17 | export names surface in packages/lib/src/crypto/encryption-algorithms/encryption-algorithms.types.ts |  |  | 0.457 |
| walker |  | 5960 | 0 | export at packages/lib/src/crypto/encryption-algorithms/encryption-algorithms.types.ts:3 |  |  | 0.457 |
| walker |  | 6000 | 40 | export names surface in packages/app-client/src/modules/i18n/i18n.provider.tsx |  |  | 0.457 |
| walker |  | 6000 | 0 | export at packages/app-client/src/modules/i18n/i18n.provider.tsx:13 |  |  | 0.457 |
| walker |  | 6000 | 0 | export at packages/app-client/src/modules/i18n/i18n.provider.tsx:44 |  |  | 0.457 |
| walker |  | 6013 | 13 | export at packages/app-client/src/modules/i18n/i18n.provider.tsx:10 |  |  | 0.457 |
| walker |  | 6100 | 87 | export at packages/cli/src/create-note/create-note.usecases.ts:3 |  |  | 0.457 |
| walker |  | 6149 | 49 | export body at packages/cli/src/create-note/create-note.usecases.ts:3 body 12 |  |  | 0.457 |
| ns | 6202 |  | 327 | Node.js entrypoint — static + SPA fallback + cron + listen | 4.5 | 4.4 | 0.440 |
| walker |  | 6209 | 60 | json config renovate.json |  |  | 0.440 |
| walker |  | 6273 | 64 | listing of 'packages/app-client/public' |  |  | 0.440 |
| walker |  | 6299 | 26 | listing of 'packages/lib/src/crypto/serialization' |  |  | 0.440 |
| walker |  | 6311 | 12 | export names surface in packages/lib/src/crypto/serialization/serialization.models.ts |  |  | 0.440 |
| walker |  | 6311 | 0 | export at packages/lib/src/crypto/serialization/serialization.models.ts:3 |  |  | 0.440 |
| walker |  | 6324 | 13 | export names surface in packages/lib/src/crypto/serialization/serialization.test-utils.ts |  |  | 0.440 |
| walker |  | 6324 | 0 | export at packages/lib/src/crypto/serialization/serialization.test-utils.ts:4 |  |  | 0.440 |
| walker |  | 6358 | 34 | export names surface in packages/lib/src/crypto/serialization/serialization.types.ts |  |  | 0.440 |
| walker |  | 6358 | 0 | export at packages/lib/src/crypto/serialization/serialization.types.ts:4 |  |  | 0.440 |
| walker |  | 6358 | 0 | export at packages/lib/src/crypto/serialization/serialization.types.ts:5 |  |  | 0.440 |
| walker |  | 6373 | 15 | listing of 'packages/lib/src/crypto/serialization/cbor-array' |  |  | 0.440 |
| walker |  | 6391 | 18 | export names surface in packages/lib/src/crypto/serialization/cbor-array/cbor-array.serialization.ts |  |  | 0.440 |
| walker |  | 6406 | 15 | imports in packages/cli/src/config/config.usecases.ts |  |  | 0.440 |
| walker |  | 6421 | 15 | imports in packages/lib/src/notes/notes.services.ts |  |  | 0.440 |
| walker |  | 6437 | 16 | imports in packages/lib/src/api/api.models.ts |  |  | 0.440 |
| ns | 6457 |  | 255 | Storage drivers — memory + fs-lite | 4.6 |  | 0.430 |
| walker |  | 6486 | 49 | export names surface in packages/app-server/src/modules/app/server.types.ts |  |  | 0.430 |
| walker |  | 6486 | 0 | export at packages/app-server/src/modules/app/server.types.ts:13 |  |  | 0.430 |
| walker |  | 6486 | 0 | export at packages/app-server/src/modules/app/server.types.ts:15 |  |  | 0.430 |
| walker |  | 6557 | 71 | export names surface in packages/lib/src/crypto/crypto.types.ts |  |  | 0.430 |
| walker |  | 6557 | 0 | export at packages/lib/src/crypto/crypto.types.ts:3 |  |  | 0.430 |
| walker |  | 6557 | 0 | export at packages/lib/src/crypto/crypto.types.ts:4 |  |  | 0.430 |
| walker |  | 6557 | 0 | export at packages/lib/src/crypto/crypto.types.ts:5 |  |  | 0.430 |
| walker |  | 6594 | 37 | export at packages/lib/src/crypto/crypto.types.ts:7 |  |  | 0.430 |
| walker |  | 6611 | 17 | imports in packages/cli/src/view-note/view-note.models.ts |  |  | 0.430 |
| walker |  | 6640 | 29 | README.md section #31 |  |  | 0.430 |
| walker |  | 6777 | 137 | export at packages/crypto/src/index.node.ts:8 |  |  | 0.430 |
| ns | 6892 |  | 435 | Storage driver — Cloudflare KV (with 413 translation) | 4.7 |  | 0.418 |
| walker |  | 6914 | 137 | export at packages/crypto/src/index.web.ts:8 |  |  | 0.418 |
| walker |  | 6988 | 74 | export names surface in packages/cli/src/files/files.services.ts |  |  | 0.418 |
| walker |  | 6988 | 0 | export at packages/cli/src/files/files.services.ts:5 |  |  | 0.418 |
| walker |  | 6988 | 0 | export at packages/cli/src/files/files.services.ts:14 |  |  | 0.418 |
| walker |  | 6988 | 0 | export at packages/cli/src/files/files.services.ts:26 |  |  | 0.418 |
| walker |  | 7039 | 51 | export body at packages/cli/src/files/files.services.ts:5 body 6 |  |  | 0.418 |
| walker |  | 7066 | 27 | imports in packages/docs/.vitepress/plausible.ts |  |  | 0.418 |
| walker |  | 7148 | 82 | headings outline in packages/docs/src/self-hosting/other-platforms.md |  |  | 0.418 |
| walker |  | 7148 | 0 | packages/docs/src/self-hosting/other-platforms.md section #0 |  |  | 0.418 |
| walker |  | 7198 | 50 | listing of 'packages/crypto/src/encryption-algorithms' |  |  | 0.418 |
| walker |  | 7206 | 8 | export names surface in packages/crypto/src/encryption-algorithms/encryption-algorithms.test-utils.ts |  |  | 0.418 |
| walker |  | 7220 | 14 | export at packages/crypto/src/encryption-algorithms/encryption-algorithms.test-utils.ts:5 |  |  | 0.418 |
| walker |  | 7232 | 12 | export names surface in packages/crypto/src/encryption-algorithms/encryption-algorithms.models.ts |  |  | 0.418 |
| walker |  | 7232 | 0 | export at packages/crypto/src/encryption-algorithms/encryption-algorithms.models.ts:1 |  |  | 0.418 |
| walker |  | 7245 | 13 | export names surface in packages/crypto/src/encryption-algorithms/encryption-algorithms.registry.ts |  |  | 0.418 |
| walker |  | 7245 | 0 | export at packages/crypto/src/encryption-algorithms/encryption-algorithms.registry.ts:4 |  |  | 0.418 |
| walker |  | 7289 | 44 | export names surface in packages/crypto/src/encryption-algorithms/encryption-algorithms.constants.ts |  |  | 0.418 |
| walker |  | 7289 | 0 | export at packages/crypto/src/encryption-algorithms/encryption-algorithms.constants.ts:1 |  |  | 0.418 |
| walker |  | 7289 | 0 | export at packages/crypto/src/encryption-algorithms/encryption-algorithms.constants.ts:3 |  |  | 0.418 |
| walker |  | 7341 | 52 | export names surface in packages/crypto/src/encryption-algorithms/encryption-algorithms.types.ts |  |  | 0.418 |
| walker |  | 7341 | 0 | export at packages/crypto/src/encryption-algorithms/encryption-algorithms.types.ts:8 |  |  | 0.418 |
| walker |  | 7341 | 0 | export at packages/crypto/src/encryption-algorithms/encryption-algorithms.types.ts:10 |  |  | 0.418 |
| walker |  | 7364 | 23 | listing of 'packages/app-client/src/modules/shared/http' |  |  | 0.418 |
| walker |  | 7375 | 11 | export names surface in packages/app-client/src/modules/shared/http/http-client.ts |  |  | 0.418 |
| walker |  | 7375 | 0 | export at packages/app-client/src/modules/shared/http/http-client.ts:5 |  |  | 0.418 |
| walker |  | 7398 | 23 | export names surface in packages/app-client/src/modules/shared/http/http-client.models.ts |  |  | 0.418 |
| walker |  | 7398 | 0 | export at packages/app-client/src/modules/shared/http/http-client.models.ts:1 |  |  | 0.418 |
| ns | 7406 |  | 514 | Notes — models + types + constants | 5.1 |  | 0.402 |
| walker |  | 7424 | 26 | export names surface in packages/app-client/src/modules/shared/http/http-errors.ts |  |  | 0.402 |
| walker |  | 7424 | 0 | export at packages/app-client/src/modules/shared/http/http-errors.ts:3 |  |  | 0.402 |
| walker |  | 7473 | 49 | export at packages/lib/src/notes/notes.types.ts:4 |  |  | 0.402 |
| walker |  | 7495 | 22 | packages/docs/src/resources/i18n.md section #0 |  |  | 0.402 |
| walker |  | 7547 | 52 | listing of 'packages/docs/src/public' |  |  | 0.402 |
| walker |  | 7578 | 31 | README.md section #30 |  |  | 0.402 |
| ns | 7835 |  | 429 | Notes repository — exports + factory + getRefreshedNote usecase | 5.2 |  | 0.387 |
| ns | 8111 |  | 276 | Auth catalog — errors + models + services signatures | 5.3 |  | 0.379 |
| walker |  | 8123 | 545 | export at packages/app-client/playwright.config.ts:12 |  |  | 0.379 |
| walker |  | 8149 | 26 | export doc at packages/app-client/playwright.config.ts:12 |  |  | 0.379 |
| walker |  | 8174 | 25 | listing of 'packages/app-server/src/modules/app/config' |  |  | 0.379 |
| walker |  | 8185 | 11 | export names surface in packages/app-server/src/modules/app/config/config.test-utils.ts |  |  | 0.379 |
| walker |  | 8185 | 0 | export at packages/app-server/src/modules/app/config/config.test-utils.ts:6 |  |  | 0.379 |
| walker |  | 8197 | 12 | export names surface in packages/app-server/src/modules/app/config/config.routes.ts |  |  | 0.379 |
| walker |  | 8197 | 0 | export at packages/app-server/src/modules/app/config/config.routes.ts:3 |  |  | 0.379 |
| walker |  | 8211 | 14 | export names surface in packages/app-server/src/modules/app/config/config.models.ts |  |  | 0.379 |
| walker |  | 8211 | 0 | export at packages/app-server/src/modules/app/config/config.models.ts:3 |  |  | 0.379 |
| walker |  | 8225 | 14 | export names surface in packages/app-server/src/modules/app/config/config.types.ts |  |  | 0.379 |
| walker |  | 8225 | 0 | export at packages/app-server/src/modules/app/config/config.types.ts:3 |  |  | 0.379 |
| walker |  | 8264 | 39 | export names surface in packages/app-server/src/modules/app/config/config.ts |  |  | 0.379 |
| walker |  | 8264 | 0 | export at packages/app-server/src/modules/app/config/config.ts:258 |  |  | 0.379 |
| walker |  | 8278 | 14 | imports in packages/app-client/src/modules/config/config.constants.ts |  |  | 0.379 |
| walker |  | 8390 | 112 | README.md section #24 |  |  | 0.379 |
| ns | 8429 |  | 318 | Auth middleware — gating + protected-route guard | 5.4 |  | 0.371 |
| walker |  | 8466 | 76 | packages/crypto/README.md section #1 |  |  | 0.371 |
| walker |  | 8510 | 44 | export body at packages/app-server/src/modules/app/config/config.ts:258 body 259 |  |  | 0.375 |
| walker |  | 8549 | 39 | listing of 'packages/app-client/src/modules/notes' |  |  | 0.375 |
| walker |  | 8562 | 13 | export names surface in packages/app-client/src/modules/notes/notes.usecases.ts |  |  | 0.375 |
| walker |  | 8562 | 0 | export at packages/app-client/src/modules/notes/notes.usecases.ts:4 |  |  | 0.375 |
| walker |  | 8582 | 20 | export names surface in packages/app-client/src/modules/notes/notes.services.ts |  |  | 0.375 |
| walker |  | 8582 | 0 | export at packages/app-client/src/modules/notes/notes.services.ts:3 |  |  | 0.375 |
| walker |  | 8613 | 31 | export names surface in packages/app-client/src/modules/notes/notes.constants.ts |  |  | 0.375 |
| walker |  | 8613 | 0 | export at packages/app-client/src/modules/notes/notes.constants.ts:1 |  |  | 0.375 |
| walker |  | 8646 | 33 | export names surface in packages/app-client/src/modules/notes/notes.context.tsx |  |  | 0.375 |
| walker |  | 8646 | 0 | export at packages/app-client/src/modules/notes/notes.context.tsx:11 |  |  | 0.375 |
| walker |  | 8646 | 0 | export at packages/app-client/src/modules/notes/notes.context.tsx:27 |  |  | 0.375 |
| walker |  | 8660 | 14 | listing of 'packages/app-client/src/modules/notes/components' |  |  | 0.375 |
| walker |  | 8676 | 16 | export names surface in packages/app-client/src/modules/notes/components/file-uploader.tsx |  |  | 0.375 |
| walker |  | 8690 | 14 | listing of 'packages/app-client/src/modules/notes/pages' |  |  | 0.375 |
| walker |  | 8707 | 17 | export names surface in packages/app-client/src/modules/notes/pages/create-note.page.tsx |  |  | 0.375 |
| walker |  | 8707 | 0 | export at packages/app-client/src/modules/notes/pages/create-note.page.tsx:109 |  |  | 0.375 |
| walker |  | 8724 | 17 | export names surface in packages/app-client/src/modules/notes/pages/view-note.page.tsx |  |  | 0.375 |
| walker |  | 8724 | 0 | export at packages/app-client/src/modules/notes/pages/view-note.page.tsx:65 |  |  | 0.375 |
| walker |  | 8761 | 37 | export names surface in packages/app-client/src/modules/notes/notes.models.ts |  |  | 0.375 |
| walker |  | 8761 | 0 | export at packages/app-client/src/modules/notes/notes.models.ts:4 |  |  | 0.375 |
| walker |  | 8761 | 0 | export at packages/app-client/src/modules/notes/notes.models.ts:16 |  |  | 0.375 |
| walker |  | 8817 | 56 | export body at packages/app-client/src/modules/notes/notes.context.tsx:27 body 28 |  |  | 0.375 |
| ns | 8821 |  | 392 | Auth login route — handler body (timing-safe bcrypt + JWT issue) | 5.5 |  | 0.365 |
| walker |  | 8862 | 45 | export names surface in packages/app-client/src/modules/notes/components/note-password-field.tsx |  |  | 0.365 |
| walker |  | 8862 | 0 | export at packages/app-client/src/modules/notes/components/note-password-field.tsx:7 |  |  | 0.365 |
| walker |  | 8877 | 15 | imports in packages/app-server/src/modules/tasks/tasks.types.ts |  |  | 0.365 |
| walker |  | 8941 | 64 | export body at packages/app-client/src/modules/notes/notes.models.ts:16 body 17 |  |  | 0.365 |
| walker |  | 8972 | 31 | CONTRIBUTING.md section #8 |  |  | 0.365 |
| walker |  | 9021 | 49 | export names surface in packages/app-client/src/modules/ui/layouts/app.layout.tsx |  |  | 0.365 |
| walker |  | 9021 | 0 | export at packages/app-client/src/modules/ui/layouts/app.layout.tsx:72 |  |  | 0.365 |
| walker |  | 9021 | 0 | export at packages/app-client/src/modules/ui/layouts/app.layout.tsx:209 |  |  | 0.365 |
| walker |  | 9021 | 0 | export at packages/app-client/src/modules/ui/layouts/app.layout.tsx:240 |  |  | 0.365 |
| walker |  | 9125 | 104 | headings outline in packages/docs/src/resources/brand-kit.md |  |  | 0.365 |
| ns | 9134 |  | 313 | Lib — encryptNote body (crypto.usecases) | 6.1 |  | 0.358 |
| walker |  | 9148 | 23 | imports in packages/crypto/src/encryption-algorithms/encryption-algorithms.types.ts |  |  | 0.358 |
| walker |  | 9164 | 16 | imports in packages/app-client/src/modules/auth/auth.services.ts |  |  | 0.358 |
| walker |  | 9180 | 16 | imports in packages/app-client/src/modules/i18n/i18n.models.ts |  |  | 0.358 |
| walker |  | 9196 | 16 | imports in packages/app-client/src/modules/notes/notes.services.ts |  |  | 0.358 |
| walker |  | 9212 | 16 | imports in packages/lib/src/crypto/serialization/serialization.models.ts |  |  | 0.358 |
| ns | 9232 |  | 98 | Lib — note URL hash-fragment markers (`pw` / `dar`) | 6.2 |  | 0.357 |
| walker |  | 9280 | 68 | export body at packages/app-client/src/modules/theme/theme.store.ts:4 body 5 |  |  | 0.357 |
| walker |  | 9394 | 114 | README.md section #26 |  |  | 0.357 |
| walker |  | 9445 | 51 | export names surface in packages/app-client/src/modules/shared/utils/copy.tsx |  |  | 0.357 |
| walker |  | 9445 | 0 | export at packages/app-client/src/modules/shared/utils/copy.tsx:6 |  |  | 0.357 |
| walker |  | 9445 | 0 | export at packages/app-client/src/modules/shared/utils/copy.tsx:19 |  |  | 0.357 |
| walker |  | 9457 | 12 | imports in packages/app-server/src/modules/shared/logger/logger.ts |  |  | 0.357 |
| walker |  | 9501 | 44 | listing of 'packages/docs/src/public/logos' |  |  | 0.357 |
| ns | 9539 |  | 307 | CLI dispatcher + create-note args | 6.3 |  | 0.350 |
| walker |  | 9546 | 45 | export at packages/app-server/src/modules/app/server.types.ts:5 |  |  | 0.350 |
| walker |  | 9647 | 101 | export body at packages/cli/src/files/files.services.ts:14 body 15 |  |  | 0.350 |
| walker |  | 9709 | 62 | README.md section #13 |  |  | 0.356 |
| walker |  | 9733 | 24 | imports in packages/cli/src/create-note/create-note.usecases.ts |  |  | 0.356 |
| walker |  | 9836 | 103 | export body at packages/cli/src/files/files.services.ts:26 body 27 |  |  | 0.356 |
| ns | 9903 |  | 364 | App-client — Solid Router routes | 6.4 |  | 0.349 |
| ns | 9990 |  | 87 | Docs site — page map (VitePress src layout) | 6.5 |  | 0.364 |
| walker |  | 9992 | 156 | plaintext config pnpm-workspace.yaml |  |  | 0.384 |

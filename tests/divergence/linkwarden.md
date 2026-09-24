Score(3000)=0.733 I=0.887 C=0.606 ns_rows≤3K=15/51 (reached=8 partial=0 missing=7) grid(1000/1442/2080/3000/4327/6240/9000)=0.633/0.712/0.689/0.733/0.681/0.686/0.818

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 75 |  | 75 | Product identity: name, tagline, one-sentence definition | 1.1 |  | 0.000 |
| walker |  | 86 | 86 | listing of '.' |  |  | 0.000 |
| walker |  | 96 | 10 | listing of 'patches' |  |  | 0.000 |
| walker |  | 107 | 11 | listing of 'apps' |  |  | 0.000 |
| walker |  | 126 | 19 | listing of 'packages' |  |  | 0.000 |
| walker |  | 129 | 3 | listing of '.vscode' |  |  | 0.000 |
| walker |  | 141 | 12 | listing of 'packages/types' |  |  | 0.000 |
| walker |  | 145 | 4 | listing of '.devcontainer' |  |  | 0.000 |
| ns | 161 |  | 86 | Complete repository root listing | 1.2 |  | 0.721 |
| walker |  | 188 | 43 | package identity in package.json |  |  | 0.724 |
| ns | 191 |  | 30 | The three apps and five shared packages | 1.3 |  | 0.711 |
| walker |  | 220 | 32 | listing of 'assets' |  |  | 0.711 |
| walker |  | 244 | 24 | listing of 'packages/prisma' |  |  | 0.714 |
| walker |  | 252 | 8 | listing of 'packages/prisma/client' |  |  | 0.714 |
| walker |  | 280 | 28 | listing of 'apps/worker' |  |  | 0.714 |
| ns | 283 |  | 92 | Complete apps/web listing | 1.4 |  | 0.534 |
| walker |  | 284 | 4 | listing of 'apps/worker/templates' |  |  | 0.534 |
| walker |  | 296 | 12 | export names surface in packages/prisma/index.ts |  |  | 0.534 |
| walker |  | 330 | 34 | package entrypoints in package.json |  |  | 0.544 |
| ns | 345 |  | 62 | Workspace globs in the root package.json | 1.5 |  | 0.559 |
| walker |  | 383 | 53 | listing of 'packages/filesystem' |  |  | 0.559 |
| ns | 484 |  | 139 | Workspace package names — every `name` field under apps/ and packages/ | 1.6 |  | 0.518 |
| walker |  | 520 | 137 | README headline in README.md |  |  | 0.717 |
| walker |  | 555 | 35 | listing of 'apps/worker/workers' |  |  | 0.717 |
| ns | 698 |  | 214 | Root scripts: how you run web, worker and both together | 1.7 |  | 0.665 |
| walker |  | 775 | 220 | Prisma schema TOC in packages/prisma/schema.prisma |  |  | 0.675 |
| walker |  | 846 | 71 | listing of 'packages/router' |  |  | 0.676 |
| ns | 861 |  | 163 | Root scripts: prisma, format, test, coverage, postinstall | 1.8 |  | 0.633 |
| walker |  | 876 | 30 | listing of '.github' |  |  | 0.633 |
| walker |  | 896 | 20 | listing of '.github/workflows' |  |  | 0.633 |
| walker |  | 909 | 13 | export names surface in vitest.config.mts |  |  | 0.633 |
| walker |  | 996 | 87 | listing of 'apps/mobile' |  |  | 0.633 |
| walker |  | 999 | 3 | listing of 'apps/mobile/styles' |  |  | 0.633 |
| walker |  | 1006 | 7 | listing of 'apps/mobile/assets' |  |  | 0.633 |
| walker |  | 1013 | 7 | listing of 'apps/mobile/plugins' |  |  | 0.633 |
| walker |  | 1018 | 5 | listing of 'apps/mobile/assets/fonts' |  |  | 0.633 |
| walker |  | 1028 | 10 | listing of 'apps/mobile/types' |  |  | 0.633 |
| walker |  | 1039 | 11 | listing of 'apps/mobile/store' |  |  | 0.633 |
| walker |  | 1061 | 22 | listing of 'apps/mobile/lib' |  |  | 0.633 |
| walker |  | 1097 | 36 | listing of 'apps/mobile/app' |  |  | 0.634 |
| walker |  | 1102 | 5 | listing of 'apps/mobile/app/links' |  |  | 0.634 |
| ns | 1106 |  | 245 | Feature list, first half (preservation, reading, organisation, sharing) | 1.9 |  | 0.586 |
| walker |  | 1117 | 15 | export names surface in apps/mobile/app/index.tsx |  |  | 0.586 |
| walker |  | 1117 | 0 | export at apps/mobile/app/index.tsx:12 |  |  | 0.586 |
| walker |  | 1140 | 23 | listing of 'apps/mobile/assets/images' |  |  | 0.586 |
| walker |  | 1166 | 26 | listing of 'apps/mobile/app/(tabs)' |  |  | 0.586 |
| walker |  | 1176 | 10 | listing of 'apps/mobile/app/(tabs)/links' |  |  | 0.586 |
| walker |  | 1192 | 16 | listing of 'apps/mobile/app/(tabs)/collections' |  |  | 0.587 |
| walker |  | 1208 | 16 | listing of 'apps/mobile/app/(tabs)/dashboard' |  |  | 0.587 |
| walker |  | 1224 | 16 | listing of 'apps/mobile/app/(tabs)/settings' |  |  | 0.587 |
| walker |  | 1240 | 16 | listing of 'apps/mobile/app/(tabs)/tags' |  |  | 0.588 |
| walker |  | 1264 | 24 | Prisma decl at packages/prisma/schema.prisma:77 |  |  | 0.588 |
| ns | 1295 |  | 189 | Feature list, second half (sync, SSO, API keys, i18n, RSS, uploads) | 1.10 |  | 0.549 |
| walker |  | 1313 | 49 | Prisma decl at packages/prisma/schema.prisma:90 |  |  | 0.550 |
| walker |  | 1405 | 92 | listing of 'apps/web' |  |  | 0.711 |
| walker |  | 1408 | 3 | listing of 'apps/web/styles' |  |  | 0.711 |
| walker |  | 1416 | 8 | listing of 'apps/web/store' |  |  | 0.711 |
| walker |  | 1427 | 11 | listing of 'apps/web/types' |  |  | 0.712 |
| walker |  | 1442 | 15 | listing of 'apps/web/lib' |  |  | 0.712 |
| walker |  | 1462 | 20 | listing of 'apps/web/templates' |  |  | 0.712 |
| walker |  | 1485 | 23 | listing of 'apps/web/layouts' |  |  | 0.712 |
| walker |  | 1504 | 19 | listing of 'apps/web/lib/shared' |  |  | 0.712 |
| walker |  | 1530 | 26 | Prisma decl at packages/prisma/schema.prisma:304 |  |  | 0.714 |
| ns | 1563 |  | 268 | schema.prisma: datasource/generator plus every model and enum header, bodies elided | 2.1 |  | 0.665 |
| walker |  | 1565 | 35 | Prisma decl at packages/prisma/schema.prisma:83 |  |  | 0.672 |
| ns | 1721 |  | 158 | All five schema enums, fully expanded | 2.2 | 2.1 | 0.672 |
| walker |  | 1775 | 210 | Prisma decl at packages/prisma/schema.prisma:166 |  |  | 0.678 |
| walker |  | 1954 | 179 | Prisma decl tail at packages/prisma/schema.prisma:166 body 182 |  |  | 0.686 |
| walker |  | 2062 | 108 | listing of 'packages/lib' |  |  | 0.689 |
| ns | 2107 |  | 386 | `Link` model — every field | 2.3 | 2.1 | 0.725 |
| walker |  | 2119 | 57 | listing of 'apps/worker/lib' |  |  | 0.726 |
| walker |  | 2158 | 39 | Prisma decl at packages/prisma/schema.prisma:289 |  |  | 0.755 |
| walker |  | 2178 | 20 | plaintext config apps/mobile/.env.sample |  |  | 0.755 |
| walker |  | 2313 | 135 | package runtime dependencies in package.json |  |  | 0.755 |
| walker |  | 2374 | 61 | listing of 'apps/mobile/components' |  |  | 0.755 |
| walker |  | 2399 | 25 | listing of 'apps/mobile/components/Formats' |  |  | 0.756 |
| ns | 2420 |  | 313 | `User` model — identity, relations and subscription linkage | 2.4 | 2.1 | 0.709 |
| walker |  | 2430 | 31 | listing of 'apps/mobile/components/ActionSheets' |  |  | 0.709 |
| walker |  | 2491 | 61 | listing of 'apps/web/hooks' |  |  | 0.710 |
| ns | 2762 |  | 342 | `User` model — preference and archival-toggle fields | 2.5 | 2.4 | 0.673 |
| walker |  | 2776 | 285 | Prisma decl at packages/prisma/schema.prisma:28 |  |  | 0.731 |
| walker |  | 2913 | 137 | headings outline in README.md |  |  | 0.731 |
| walker |  | 2939 | 26 | README.md section #26 |  |  | 0.731 |
| walker |  | 2950 | 11 | README.md section #18 |  |  | 0.732 |
| walker |  | 2960 | 10 | README.md section #17 |  |  | 0.732 |
| walker |  | 2970 | 10 | README.md section #19 |  |  | 0.732 |
| walker |  | 2990 | 20 | listing of 'apps/web/e2e' |  |  | 0.733 |
| walker |  | 2997 | 7 | listing of 'apps/web/e2e/tests' |  |  | 0.733 |
| walker |  | 3038 | 41 | listing of 'apps/worker/lib/preservationScheme' |  |  | 0.734 |
| walker |  | 3065 | 27 | Prisma decl at packages/prisma/schema.prisma:5 |  |  | 0.751 |
| walker |  | 3078 | 13 | README.md section #23 |  |  | 0.752 |
| ns | 3079 |  | 317 | `Collection` model — every field, including the self-relation | 2.6 | 2.1 | 0.717 |
| walker |  | 3091 | 13 | README.md section #24 |  |  | 0.718 |
| walker |  | 3187 | 96 | package runtime metadata in package.json |  |  | 0.718 |
| walker |  | 3267 | 80 | listing of 'apps/web/public' |  |  | 0.718 |
| walker |  | 3276 | 9 | listing of 'apps/web/public/screenshots' |  |  | 0.718 |
| ns | 3460 |  | 381 | `UsersAndCollections` join model (the permission bits) and `Tag` model | 2.7 | 2.1 | 0.678 |
| walker |  | 3596 | 320 | Prisma decl at packages/prisma/schema.prisma:126 |  |  | 0.731 |
| ns | 3618 |  | 158 | `AccessToken` model | 2.8 | 2.1 | 0.716 |
| ns | 3642 |  | 24 | Complete packages/prisma listing | 2.9 |  | 0.720 |
| walker |  | 3730 | 134 | plaintext config .env.sample |  |  | 0.720 |
| ns | 3758 |  | 116 | The process-wide `prisma` client singleton | 2.10 |  | 0.708 |
| walker |  | 3779 | 49 | listing of 'apps/mobile/components/ui' |  |  | 0.708 |
| walker |  | 3793 | 14 | README.md section #21 |  |  | 0.710 |
| walker |  | 3805 | 12 | README.md section #22 |  |  | 0.712 |
| walker |  | 3819 | 14 | README.md section #20 |  |  | 0.714 |
| walker |  | 3848 | 29 | README.md section #29 |  |  | 0.714 |
| ns | 3867 |  | 109 | API version directories: every resource under pages/api/v1 (and the single v2 route) | 3.1 |  | 0.684 |
| ns | 3934 |  | 67 | Route files for links, collections, tags and highlights | 3.2 |  | 0.667 |
| ns | 4004 |  | 70 | Route files for auth, session, users, tokens, config, avatar and logins | 3.3 |  | 0.651 |
| ns | 4074 |  | 70 | Route files for archives, preserved, search, dashboard, rss, migration, payment, webhook, worker, getFavicon | 3.4 |  | 0.635 |
| ns | 4119 |  | 45 | The unauthenticated `public/` route subtree | 3.5 |  | 0.626 |
| walker |  | 4221 | 373 | Prisma decl tail at packages/prisma/schema.prisma:28 body 51 |  |  | 0.673 |
| walker |  | 4237 | 16 | Prisma decl at packages/prisma/schema.prisma:1 |  |  | 0.681 |
| walker |  | 4252 | 15 | README.md section #11 |  |  | 0.681 |
| walker |  | 4346 | 94 | listing of 'apps/web/pages' |  |  | 0.682 |
| walker |  | 4351 | 5 | listing of 'apps/web/pages/preserved' |  |  | 0.682 |
| walker |  | 4360 | 9 | listing of 'apps/web/pages/api' |  |  | 0.682 |
| walker |  | 4363 | 3 | listing of 'apps/web/pages/api/v2' |  |  | 0.683 |
| walker |  | 4366 | 3 | listing of 'apps/web/pages/api/v2/dashboard' |  |  | 0.683 |
| walker |  | 4376 | 10 | listing of 'apps/web/pages/collections' |  |  | 0.683 |
| walker |  | 4386 | 10 | listing of 'apps/web/pages/tags' |  |  | 0.683 |
| ns | 4395 |  | 276 | The route-handler pattern, read from pages/api/v1/links/index.ts | 3.6 |  | 0.665 |
| walker |  | 4397 | 11 | listing of 'apps/web/pages/auth' |  |  | 0.665 |
| walker |  | 4408 | 11 | listing of 'apps/web/pages/public' |  |  | 0.666 |
| walker |  | 4412 | 4 | listing of 'apps/web/pages/public/collections' |  |  | 0.666 |
| walker |  | 4417 | 5 | listing of 'apps/web/pages/public/links' |  |  | 0.666 |
| walker |  | 4422 | 5 | listing of 'apps/web/pages/public/preserved' |  |  | 0.666 |
| walker |  | 4437 | 15 | listing of 'apps/web/pages/links' |  |  | 0.666 |
| walker |  | 4451 | 14 | export names surface in apps/web/pages/index.tsx |  |  | 0.666 |
| walker |  | 4451 | 0 | export at apps/web/pages/index.tsx:4 |  |  | 0.666 |
| walker |  | 4469 | 18 | listing of 'apps/web/pages/admin' |  |  | 0.666 |
| walker |  | 4479 | 10 | listing of 'apps/web/pages/public/collections/[id]' |  |  | 0.667 |
| walker |  | 4522 | 43 | export body at apps/web/pages/index.tsx:4 body 5 |  |  | 0.667 |
| walker |  | 4577 | 55 | listing of 'apps/web/pages/settings' |  |  | 0.668 |
| walker |  | 4594 | 17 | listing of 'apps/web/e2e/fixtures' |  |  | 0.668 |
| walker |  | 4606 | 12 | listing of 'apps/web/e2e/fixtures/base' |  |  | 0.669 |
| ns | 4610 |  | 215 | apps/web/lib: the server helper layer and its client/shared siblings | 3.7 |  | 0.639 |
| walker |  | 4669 | 63 | listing of 'apps/web/public/locales' |  |  | 0.640 |
| walker |  | 4672 | 3 | listing of 'apps/web/public/locales/de' |  |  | 0.640 |
| walker |  | 4675 | 3 | listing of 'apps/web/public/locales/en' |  |  | 0.640 |
| walker |  | 4678 | 3 | listing of 'apps/web/public/locales/es' |  |  | 0.640 |
| walker |  | 4681 | 3 | listing of 'apps/web/public/locales/fr' |  |  | 0.640 |
| walker |  | 4684 | 3 | listing of 'apps/web/public/locales/it' |  |  | 0.640 |
| walker |  | 4687 | 3 | listing of 'apps/web/public/locales/ja' |  |  | 0.640 |
| walker |  | 4690 | 3 | listing of 'apps/web/public/locales/nl' |  |  | 0.640 |
| walker |  | 4693 | 3 | listing of 'apps/web/public/locales/pl' |  |  | 0.640 |
| walker |  | 4696 | 3 | listing of 'apps/web/public/locales/pt-BR' |  |  | 0.640 |
| walker |  | 4699 | 3 | listing of 'apps/web/public/locales/ro' |  |  | 0.640 |
| walker |  | 4702 | 3 | listing of 'apps/web/public/locales/ru' |  |  | 0.640 |
| walker |  | 4705 | 3 | listing of 'apps/web/public/locales/tr' |  |  | 0.640 |
| walker |  | 4708 | 3 | listing of 'apps/web/public/locales/uk' |  |  | 0.640 |
| walker |  | 4711 | 3 | listing of 'apps/web/public/locales/zh' |  |  | 0.640 |
| walker |  | 4714 | 3 | listing of 'apps/web/public/locales/zh-TW' |  |  | 0.640 |
| ns | 4717 |  | 107 | The controller tree: every resource directory under lib/api/controllers | 3.8 |  | 0.624 |
| walker |  | 4731 | 17 | README.md section #7 |  |  | 0.624 |
| walker |  | 4747 | 16 | README.md section #6 |  |  | 0.625 |
| walker |  | 4764 | 17 | README.md section #9 |  |  | 0.625 |
| walker |  | 4780 | 16 | README.md section #8 |  |  | 0.626 |
| walker |  | 4796 | 16 | README.md section #10 |  |  | 0.628 |
| walker |  | 4813 | 17 | README.md section #16 |  |  | 0.630 |
| walker |  | 4834 | 21 | listing of '.github/ISSUE_TEMPLATE' |  |  | 0.631 |
| ns | 4874 |  | 157 | Controller files for links, collections, tags and highlights | 3.9 |  | 0.612 |
| ns | 5050 |  | 176 | Controller files for users, tokens, session, search, dashboard, worker, migration and public access | 3.10 |  | 0.592 |
| walker |  | 5111 | 277 | plaintext config Dockerfile |  |  | 0.592 |
| walker |  | 5130 | 19 | README.md section #15 |  |  | 0.595 |
| walker |  | 5208 | 78 | listing of 'apps/web/lib/client' |  |  | 0.608 |
| ns | 5294 |  | 244 | Complete listings of packages/types, packages/lib, packages/filesystem and packages/router | 4.1 |  | 0.635 |
| walker |  | 5295 | 87 | export at vitest.config.mts:4 |  |  | 0.635 |
| walker |  | 5371 | 76 | export at packages/prisma/index.ts:5 |  |  | 0.642 |
| walker |  | 5666 | 295 | README.md section #0 |  |  | 0.642 |
| walker |  | 5688 | 22 | README.md section #1 |  |  | 0.644 |
| ns | 5692 |  | 398 | packages/types/global.ts — every exported type, interface and enum declaration | 4.2 |  | 0.627 |
| walker |  | 5710 | 22 | README.md section #2 |  |  | 0.629 |
| walker |  | 5725 | 15 | imports in packages/prisma/index.ts |  |  | 0.629 |
| ns | 5838 |  | 146 | `ArchivedFormat`, `LinkType` and `TokenExpiry` variants | 4.3 | 4.2 | 0.621 |
| walker |  | 5878 | 153 | plaintext dotenv tail chunk #1 of .env.sample |  |  | 0.622 |
| walker |  | 5902 | 24 | README.md section #5 |  |  | 0.624 |
| walker |  | 5925 | 23 | README.md section #4 |  |  | 0.627 |
| ns | 5990 |  | 152 | `ViewMode`, `Sort` and `TagSort` variants | 4.4 | 4.2 | 0.620 |
| walker |  | 6025 | 100 | listing of 'apps/web/lib/api' |  |  | 0.651 |
| walker |  | 6037 | 12 | listing of 'apps/web/lib/api/archives' |  |  | 0.651 |
| walker |  | 6053 | 16 | listing of 'apps/web/lib/api/preserved' |  |  | 0.652 |
| walker |  | 6085 | 32 | listing of 'apps/web/lib/api/stripe' |  |  | 0.656 |
| walker |  | 6132 | 47 | listing of 'apps/web/lib/api/controllers' |  |  | 0.675 |
| walker |  | 6136 | 4 | listing of 'apps/web/lib/api/controllers/search' |  |  | 0.675 |
| walker |  | 6140 | 4 | listing of 'apps/web/lib/api/controllers/session' |  |  | 0.676 |
| walker |  | 6145 | 5 | listing of 'apps/web/lib/api/controllers/worker' |  |  | 0.676 |
| walker |  | 6156 | 11 | listing of 'apps/web/lib/api/controllers/public' |  |  | 0.677 |
| walker |  | 6160 | 4 | listing of 'apps/web/lib/api/controllers/public/links' |  |  | 0.677 |
| walker |  | 6165 | 5 | listing of 'apps/web/lib/api/controllers/public/collections' |  |  | 0.678 |
| walker |  | 6170 | 5 | listing of 'apps/web/lib/api/controllers/public/users' |  |  | 0.678 |
| walker |  | 6176 | 6 | listing of 'apps/web/lib/api/controllers/public/links/linkId' |  |  | 0.679 |
| walker |  | 6189 | 13 | listing of 'apps/web/lib/api/controllers/highlights' |  |  | 0.679 |
| walker |  | 6203 | 14 | listing of 'apps/web/lib/api/controllers/collections' |  |  | 0.680 |
| walker |  | 6217 | 14 | listing of 'apps/web/lib/api/controllers/tokens' |  |  | 0.682 |
| walker |  | 6223 | 6 | listing of 'apps/web/lib/api/controllers/tokens/tokenId' |  |  | 0.683 |
| walker |  | 6237 | 14 | listing of 'apps/web/lib/api/controllers/users' |  |  | 0.686 |
| walker |  | 6255 | 18 | listing of 'apps/web/lib/api/controllers/links' |  |  | 0.689 |
| walker |  | 6266 | 11 | listing of 'apps/web/lib/api/controllers/links/bulk' |  |  | 0.690 |
| walker |  | 6285 | 19 | listing of 'apps/web/lib/api/controllers/dashboard' |  |  | 0.694 |
| walker |  | 6312 | 27 | listing of 'apps/web/lib/api/controllers/tags' |  |  | 0.699 |
| ns | 6330 |  | 340 | packages/lib/schemaValidation.ts — every exported zod schema constant | 4.5 |  | 0.686 |
| walker |  | 6332 | 20 | listing of 'apps/web/lib/api/controllers/collections/collectionId' |  |  | 0.690 |
| walker |  | 6352 | 20 | listing of 'apps/web/lib/api/controllers/tags/tagId' |  |  | 0.695 |
| walker |  | 6376 | 24 | listing of 'apps/web/lib/api/controllers/links/linkId' |  |  | 0.702 |
| walker |  | 6381 | 5 | listing of 'apps/web/lib/api/controllers/links/linkId/highlight' |  |  | 0.704 |
| walker |  | 6407 | 26 | listing of 'apps/web/lib/api/controllers/users/userId' |  |  | 0.710 |
| walker |  | 6432 | 25 | README.md section #13 |  |  | 0.713 |
| walker |  | 6456 | 24 | README.md section #12 |  |  | 0.716 |
| ns | 6587 |  | 257 | packages/router — the links, collections and tags hook exports | 4.6 |  | 0.700 |
| walker |  | 6766 | 310 | YAML config at docker-compose.yml |  |  | 0.701 |
| ns | 6771 |  | 184 | packages/router — export lines of the remaining ten hook modules | 4.7 | 4.6 | 0.696 |
| walker |  | 6783 | 17 | imports in apps/worker/index.ts |  |  | 0.696 |
| walker |  | 6830 | 47 | listing of 'apps/web/lib/api/controllers/migration' |  |  | 0.708 |
| walker |  | 6859 | 29 | README.md section #3 |  |  | 0.711 |
| walker |  | 6890 | 31 | README.md section #14 |  |  | 0.716 |
| ns | 6940 |  | 169 | Complete apps/worker listing, including every job and every preservation handler | 5.1 |  | 0.725 |
| walker |  | 6980 | 90 | listing of 'apps/web/pages/api/v1' |  |  | 0.749 |
| walker |  | 6983 | 3 | listing of 'apps/web/pages/api/v1/dashboard' |  |  | 0.749 |
| walker |  | 6986 | 3 | listing of 'apps/web/pages/api/v1/getFavicon' |  |  | 0.750 |
| walker |  | 6989 | 3 | listing of 'apps/web/pages/api/v1/logins' |  |  | 0.750 |
| walker |  | 6992 | 3 | listing of 'apps/web/pages/api/v1/migration' |  |  | 0.750 |
| walker |  | 6995 | 3 | listing of 'apps/web/pages/api/v1/payment' |  |  | 0.750 |
| walker |  | 6998 | 3 | listing of 'apps/web/pages/api/v1/search' |  |  | 0.751 |
| walker |  | 7001 | 3 | listing of 'apps/web/pages/api/v1/session' |  |  | 0.751 |
| walker |  | 7004 | 3 | listing of 'apps/web/pages/api/v1/webhook' |  |  | 0.752 |
| walker |  | 7009 | 5 | listing of 'apps/web/pages/api/v1/avatar' |  |  | 0.752 |
| walker |  | 7017 | 8 | listing of 'apps/web/pages/api/v1/config' |  |  | 0.753 |
| walker |  | 7025 | 8 | listing of 'apps/web/pages/api/v1/worker' |  |  | 0.754 |
| walker |  | 7034 | 9 | listing of 'apps/web/pages/api/v1/collections' |  |  | 0.755 |
| walker |  | 7043 | 9 | listing of 'apps/web/pages/api/v1/highlights' |  |  | 0.755 |
| walker |  | 7052 | 9 | listing of 'apps/web/pages/api/v1/rss' |  |  | 0.757 |
| walker |  | 7061 | 9 | listing of 'apps/web/pages/api/v1/tokens' |  |  | 0.759 |
| walker |  | 7072 | 11 | listing of 'apps/web/pages/api/v1/public' |  |  | 0.760 |
| walker |  | 7077 | 5 | listing of 'apps/web/pages/api/v1/public/links' |  |  | 0.760 |
| walker |  | 7082 | 5 | listing of 'apps/web/pages/api/v1/public/users' |  |  | 0.761 |
| walker |  | 7094 | 12 | listing of 'apps/web/pages/api/v1/links' |  |  | 0.763 |
| walker |  | 7097 | 3 | listing of 'apps/web/pages/api/v1/links/archive' |  |  | 0.764 |
| walker |  | 7109 | 12 | listing of 'apps/web/pages/api/v1/users' |  |  | 0.767 |
| walker |  | 7118 | 9 | listing of 'apps/web/pages/api/v1/users/[id]' |  |  | 0.770 |
| walker |  | 7131 | 13 | listing of 'apps/web/pages/api/v1/tags' |  |  | 0.773 |
| walker |  | 7143 | 12 | listing of 'apps/web/pages/api/v1/links/[id]' |  |  | 0.777 |
| walker |  | 7146 | 3 | listing of 'apps/web/pages/api/v1/links/[id]/archive' |  |  | 0.779 |
| walker |  | 7149 | 3 | listing of 'apps/web/pages/api/v1/links/[id]/highlights' |  |  | 0.781 |
| walker |  | 7166 | 17 | listing of 'apps/web/pages/api/v1/preserved' |  |  | 0.786 |
| walker |  | 7184 | 18 | listing of 'apps/web/pages/api/v1/archives' |  |  | 0.791 |
| walker |  | 7197 | 13 | listing of 'apps/web/pages/api/v1/public/collections' |  |  | 0.795 |
| walker |  | 7200 | 3 | listing of 'apps/web/pages/api/v1/public/collections/links' |  |  | 0.796 |
| walker |  | 7203 | 3 | listing of 'apps/web/pages/api/v1/public/collections/tags' |  |  | 0.798 |
| ns | 7209 |  | 269 | worker.ts — the whole scheduler entry point | 5.2 |  | 0.785 |
| walker |  | 7224 | 21 | listing of 'apps/web/pages/api/v1/auth' |  |  | 0.791 |
| walker |  | 7249 | 25 | imports in apps/web/pages/index.tsx |  |  | 0.791 |
| walker |  | 7252 | 3 | listing of 'apps/web/scripts' |  |  | 0.791 |
| ns | 7402 |  | 193 | archiveHandler's signature and its SSRF / skip-preservation guard | 5.3 |  | 0.780 |
| walker |  | 7573 | 321 | listing of 'apps/web/components' |  |  | 0.784 |
| walker |  | 7582 | 9 | listing of 'apps/web/components/LinkViews' |  |  | 0.784 |
| walker |  | 7597 | 15 | listing of 'apps/web/components/InputSelect' |  |  | 0.784 |
| walker |  | 7622 | 25 | listing of 'apps/web/components/Preservation' |  |  | 0.784 |
| walker |  | 7662 | 40 | listing of 'apps/web/components/ui' |  |  | 0.785 |
| ns | 7665 |  | 263 | Every UI page route under apps/web/pages | 6.1 |  | 0.793 |
| walker |  | 7723 | 61 | listing of 'apps/web/components/LinkViews/LinkComponents' |  |  | 0.793 |
| walker |  | 7890 | 167 | listing of 'apps/web/components/ModalContent' |  |  | 0.796 |
| ns | 8027 |  | 362 | The flat components/ directory and the ui/ primitives | 6.2 |  | 0.804 |
| walker |  | 8267 | 377 | package scripts in package.json |  |  | 0.829 |
| ns | 8305 |  | 278 | Modal, link-view, preservation and input-picker component subdirectories | 6.3 | 6.2 | 0.834 |
| ns | 8532 |  | 227 | Web hooks, layouts, stores, ambient types, email templates, one-off migration scripts and the Playwright suite | 6.4 |  | 0.830 |
| ns | 8772 |  | 240 | verifyUser — the guard chain every authenticated route runs first | 7.1 |  | 0.818 |
| walker |  | 8789 | 522 | plaintext config tail of Dockerfile |  |  | 0.818 |
| walker |  | 8877 | 88 | README.md section #31 |  |  | 0.818 |
| walker |  | 8981 | 104 | README.md section #28 |  |  | 0.818 |
| ns | 9007 |  | 235 | docker-compose.yml — the three-container deployment | 7.2 |  | 0.820 |
| walker |  | 9023 | 42 | package identity in packages/lib/package.json |  |  | 0.820 |
| walker |  | 9035 | 12 | package entrypoints in packages/lib/package.json |  |  | 0.820 |
| walker |  | 9077 | 42 | package identity in packages/router/package.json |  |  | 0.821 |
| walker |  | 9089 | 12 | package entrypoints in packages/router/package.json |  |  | 0.821 |
| walker |  | 9131 | 42 | package identity in packages/types/package.json |  |  | 0.822 |
| walker |  | 9143 | 12 | package entrypoints in packages/types/package.json |  |  | 0.822 |
| walker |  | 9186 | 43 | package runtime dependencies in packages/types/package.json |  |  | 0.822 |
| ns | 9221 |  | 214 | .env.sample — the required variables and the complete list of section headings | 7.3 |  | 0.817 |
| walker |  | 9229 | 43 | package identity in apps/worker/package.json |  |  | 0.819 |
| walker |  | 9241 | 12 | package entrypoints in apps/worker/package.json |  |  | 0.819 |
| walker |  | 9299 | 58 | package scripts in apps/worker/package.json |  |  | 0.819 |
| walker |  | 9342 | 43 | package identity in packages/filesystem/package.json |  |  | 0.820 |
| walker |  | 9354 | 12 | package entrypoints in packages/filesystem/package.json |  |  | 0.820 |
| walker |  | 9388 | 34 | package runtime dependencies in packages/filesystem/package.json |  |  | 0.820 |
| ns | 9400 |  | 179 | The head of the optional tuning-variable block | 7.4 | 7.3 | 0.818 |
| walker |  | 9431 | 43 | package identity in packages/prisma/package.json |  |  | 0.820 |
| walker |  | 9458 | 27 | package entrypoints in packages/prisma/package.json |  |  | 0.820 |
| walker |  | 9529 | 71 | package scripts in packages/prisma/package.json |  |  | 0.820 |
| ns | 9547 |  | 147 | CI workflows, GitHub templates, the patch-package patch, and every translated locale | 7.5 |  | 0.823 |
| walker |  | 9575 | 46 | package runtime dependencies in packages/prisma/package.json |  |  | 0.823 |
| walker |  | 9598 | 23 | imports in vitest.config.mts |  |  | 0.823 |
| walker |  | 9644 | 46 | package identity in apps/mobile/package.json |  |  | 0.825 |
| walker |  | 9656 | 12 | package entrypoints in apps/mobile/package.json |  |  | 0.825 |
| walker |  | 9751 | 95 | package scripts in apps/mobile/package.json |  |  | 0.825 |
| walker |  | 9765 | 14 | export names surface in apps/web/pages/settings/index.tsx |  |  | 0.825 |
| walker |  | 9765 | 0 | export at apps/web/pages/settings/index.tsx:4 |  |  | 0.825 |
| ns | 9776 |  | 229 | apps/mobile root and the complete Expo Router screen tree | 8.1 |  | 0.829 |
| walker |  | 9826 | 61 | package identity in apps/web/package.json |  |  | 0.832 |
| walker |  | 9838 | 12 | package entrypoints in apps/web/package.json |  |  | 0.832 |
| walker |  | 9966 | 128 | package scripts in apps/web/package.json |  |  | 0.832 |
| walker |  | 9969 | 3 | listing of 'apps/web/e2e/data' |  |  | 0.833 |
| ns | 9978 |  | 202 | Mobile components, stores and query-cache modules | 8.2 | 8.1 | 0.835 |

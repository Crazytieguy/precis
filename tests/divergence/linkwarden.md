Score(3000)=0.646 I=0.856 C=0.487 ns_rows≤3K=15/51 grid(1000/1442/2080/3000/4327/6240/9000)=0.622/0.547/0.673/0.646/0.572/0.585/0.656

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 75 |  | 75 | Product identity: name, tagline, one-sentence definition | 1.1 |  | 0.000 |
| walker |  | 79 | 79 | listing of '.' |  |  | 0.000 |
| walker |  | 88 | 9 | listing of 'apps' |  |  | 0.000 |
| walker |  | 99 | 11 | listing of 'patches' |  |  | 0.000 |
| walker |  | 114 | 15 | listing of 'packages' |  |  | 0.000 |
| walker |  | 118 | 4 | listing of '.vscode' |  |  | 0.000 |
| walker |  | 131 | 13 | listing of 'packages/types' |  |  | 0.000 |
| ns | 154 |  | 79 | Complete repository root listing | 1.2 |  | 0.721 |
| walker |  | 174 | 43 | package identity in package.json |  |  | 0.724 |
| ns | 178 |  | 24 | The three apps and five shared packages | 1.3 |  | 0.711 |
| walker |  | 207 | 33 | listing of 'assets' |  |  | 0.711 |
| walker |  | 212 | 5 | listing of '.devcontainer' |  |  | 0.711 |
| walker |  | 235 | 23 | listing of 'packages/prisma' |  |  | 0.714 |
| ns | 259 |  | 81 | Complete apps/web listing | 1.4 |  | 0.533 |
| walker |  | 272 | 37 | ts names packages/prisma/index.ts |  |  | 0.534 |
| walker |  | 281 | 9 | listing of 'packages/prisma/client' |  |  | 0.534 |
| ns | 321 |  | 62 | Workspace globs in the root package.json | 1.5 |  | 0.507 |
| walker |  | 357 | 76 | ts decl packages/prisma/index.ts:5 |  |  | 0.509 |
| walker |  | 383 | 26 | listing of 'apps/worker' |  |  | 0.509 |
| walker |  | 395 | 12 | ts names apps/worker/index.ts |  |  | 0.509 |
| walker |  | 400 | 5 | listing of 'apps/worker/templates' |  |  | 0.510 |
| walker |  | 434 | 34 | package entrypoints in package.json |  |  | 0.561 |
| ns | 460 |  | 139 | Workspace package names — every `name` field under apps/ and packages/ | 1.6 |  | 0.519 |
| walker |  | 488 | 54 | listing of 'packages/filesystem' |  |  | 0.520 |
| walker |  | 588 | 100 | ts names packages/filesystem/index.ts |  |  | 0.520 |
| walker |  | 602 | 14 | ts names packages/filesystem/createFile.ts |  |  | 0.520 |
| walker |  | 621 | 19 | ts names packages/filesystem/readFile.ts |  |  | 0.520 |
| ns | 674 |  | 214 | Root scripts: how you run web, worker and both together | 1.7 |  | 0.482 |
| walker |  | 758 | 137 | README headline in README.md |  |  | 0.665 |
| walker |  | 780 | 22 | ts names packages/filesystem/fileExists.ts |  |  | 0.665 |
| walker |  | 802 | 22 | ts names packages/filesystem/moveFile.ts |  |  | 0.665 |
| walker |  | 825 | 23 | ts names packages/filesystem/createFolder.ts |  |  | 0.665 |
| ns | 837 |  | 163 | Root scripts: prisma, format, test, coverage, postinstall | 1.8 |  | 0.622 |
| walker |  | 849 | 24 | ts names packages/filesystem/removeFile.ts |  |  | 0.622 |
| walker |  | 873 | 24 | ts names packages/filesystem/removeFolder.ts |  |  | 0.622 |
| ns | 1082 |  | 245 | Feature list, first half (preservation, reading, organisation, sharing) | 1.9 |  | 0.575 |
| walker |  | 1093 | 220 | Prisma schema TOC in packages/prisma/schema.prisma |  |  | 0.584 |
| walker |  | 1129 | 36 | listing of 'apps/worker/workers' |  |  | 0.584 |
| walker |  | 1201 | 72 | listing of 'packages/router' |  |  | 0.586 |
| walker |  | 1229 | 28 | ts names packages/filesystem/s3Client.ts |  |  | 0.586 |
| walker |  | 1246 | 17 | ts names packages/prisma/client/index.d.ts |  |  | 0.586 |
| ns | 1271 |  | 189 | Feature list, second half (sync, SSO, API keys, i18n, RSS, uploads) | 1.10 |  | 0.546 |
| walker |  | 1326 | 80 | listing of 'apps/mobile' |  |  | 0.547 |
| walker |  | 1330 | 4 | listing of 'apps/mobile/styles' |  |  | 0.547 |
| walker |  | 1336 | 6 | listing of 'apps/mobile/assets' |  |  | 0.547 |
| walker |  | 1344 | 8 | listing of 'apps/mobile/plugins' |  |  | 0.547 |
| walker |  | 1350 | 6 | listing of 'apps/mobile/assets/fonts' |  |  | 0.547 |
| walker |  | 1361 | 11 | listing of 'apps/mobile/types' |  |  | 0.547 |
| walker |  | 1373 | 12 | listing of 'apps/mobile/store' |  |  | 0.547 |
| walker |  | 1396 | 23 | listing of 'apps/mobile/lib' |  |  | 0.547 |
| walker |  | 1432 | 36 | listing of 'apps/mobile/app' |  |  | 0.547 |
| walker |  | 1447 | 15 | ts names apps/mobile/app/index.tsx |  |  | 0.547 |
| walker |  | 1453 | 6 | listing of 'apps/mobile/app/links' |  |  | 0.547 |
| walker |  | 1474 | 21 | listing of 'apps/mobile/app/(tabs)' |  |  | 0.548 |
| walker |  | 1485 | 11 | listing of 'apps/mobile/app/(tabs)/links' |  |  | 0.548 |
| walker |  | 1509 | 24 | listing of 'apps/mobile/assets/images' |  |  | 0.548 |
| ns | 1539 |  | 268 | schema.prisma: datasource/generator plus every model and enum header, bodies elided | 2.1 |  | 0.531 |
| walker |  | 1590 | 81 | listing of 'apps/web' |  |  | 0.650 |
| walker |  | 1594 | 4 | listing of 'apps/web/styles' |  |  | 0.650 |
| walker |  | 1603 | 9 | listing of 'apps/web/store' |  |  | 0.650 |
| walker |  | 1615 | 12 | listing of 'apps/web/types' |  |  | 0.650 |
| walker |  | 1628 | 13 | listing of 'apps/web/lib' |  |  | 0.650 |
| walker |  | 1649 | 21 | listing of 'apps/web/templates' |  |  | 0.650 |
| walker |  | 1673 | 24 | listing of 'apps/web/layouts' |  |  | 0.651 |
| walker |  | 1693 | 20 | listing of 'apps/web/lib/shared' |  |  | 0.651 |
| ns | 1697 |  | 158 | All five schema enums, fully expanded | 2.2 | 2.1 | 0.606 |
| walker |  | 1722 | 29 | listing of '.github' |  |  | 0.606 |
| walker |  | 1743 | 21 | listing of '.github/workflows' |  |  | 0.607 |
| walker |  | 1767 | 24 | Prisma decl at packages/prisma/schema.prisma:77 |  |  | 0.612 |
| walker |  | 1784 | 17 | listing of 'apps/mobile/app/(tabs)/collections' |  |  | 0.612 |
| walker |  | 1801 | 17 | listing of 'apps/mobile/app/(tabs)/dashboard' |  |  | 0.612 |
| walker |  | 1818 | 17 | listing of 'apps/mobile/app/(tabs)/settings' |  |  | 0.612 |
| walker |  | 1835 | 17 | listing of 'apps/mobile/app/(tabs)/tags' |  |  | 0.613 |
| walker |  | 1884 | 49 | Prisma decl at packages/prisma/schema.prisma:90 |  |  | 0.630 |
| walker |  | 1917 | 33 | ts names packages/router/publicLinks.tsx |  |  | 0.630 |
| walker |  | 1950 | 33 | ts names packages/router/publicTags.tsx |  |  | 0.630 |
| walker |  | 1976 | 26 | Prisma decl at packages/prisma/schema.prisma:304 |  |  | 0.646 |
| walker |  | 2011 | 35 | Prisma decl at packages/prisma/schema.prisma:83 |  |  | 0.673 |
| ns | 2083 |  | 386 | `Link` model — every field | 2.3 | 2.1 | 0.612 |
| walker |  | 2221 | 210 | Prisma decl at packages/prisma/schema.prisma:166 |  |  | 0.647 |
| ns | 2396 |  | 313 | `User` model — identity, relations and subscription linkage | 2.4 | 2.1 | 0.607 |
| walker |  | 2400 | 179 | Prisma decl tail at packages/prisma/schema.prisma:166 body 182 |  |  | 0.678 |
| walker |  | 2535 | 135 | package runtime dependencies in package.json |  |  | 0.678 |
| walker |  | 2644 | 109 | listing of 'packages/lib' |  |  | 0.681 |
| walker |  | 2657 | 13 | ts names packages/lib/transporter.ts |  |  | 0.681 |
| walker |  | 2671 | 14 | ts names packages/lib/constants.ts |  |  | 0.681 |
| walker |  | 2685 | 14 | ts names packages/lib/safeFetch.ts |  |  | 0.681 |
| walker |  | 2700 | 15 | ts names packages/lib/generatePreview.ts |  |  | 0.681 |
| walker |  | 2715 | 15 | ts names packages/lib/rssHandler.ts |  |  | 0.681 |
| walker |  | 2731 | 16 | ts names packages/lib/verifyCapacity.ts |  |  | 0.681 |
| ns | 2738 |  | 342 | `User` model — preference and archival-toggle fields | 2.5 | 2.4 | 0.646 |
| walker |  | 2748 | 17 | ts names packages/lib/meilisearchClient.ts |  |  | 0.646 |
| walker |  | 2766 | 18 | ts names packages/lib/getPreservedFormatUrl.ts |  |  | 0.646 |
| walker |  | 2789 | 23 | ts names packages/lib/isArchivalTag.ts |  |  | 0.646 |
| walker |  | 2813 | 24 | ts names packages/lib/getOriginalFormat.ts |  |  | 0.646 |
| walker |  | 2839 | 26 | ts decl packages/lib/getOriginalFormat.ts:6 |  |  | 0.646 |
| walker |  | 2867 | 28 | ts names packages/lib/formatStats.ts |  |  | 0.646 |
| walker |  | 2891 | 24 | ts decl packages/lib/formatStats.ts:11 |  |  | 0.646 |
| walker |  | 2919 | 28 | ts names packages/lib/getFormatBasedOnPreference.ts |  |  | 0.646 |
| walker |  | 2947 | 28 | ts names packages/lib/getLinkTypeFromFormat.ts |  |  | 0.646 |
| walker |  | 2975 | 28 | ts decl packages/lib/verifyCapacity.ts:8 |  |  | 0.646 |
| walker |  | 3005 | 30 | ts decl packages/lib/rssHandler.ts:7 |  |  | 0.646 |
| walker |  | 3035 | 30 | ts decl packages/lib/safeFetch.ts:95 |  |  | 0.646 |
| ns | 3055 |  | 317 | `Collection` model — every field, including the self-relation | 2.6 | 2.1 | 0.616 |
| walker |  | 3069 | 34 | ts decl packages/lib/getLinkTypeFromFormat.ts:3 |  |  | 0.616 |
| walker |  | 3106 | 37 | ts names packages/lib/getFormatFromContentType.ts |  |  | 0.616 |
| walker |  | 3163 | 57 | listing of 'apps/worker/lib' |  |  | 0.617 |
| walker |  | 3202 | 39 | Prisma decl at packages/prisma/schema.prisma:289 |  |  | 0.641 |
| walker |  | 3222 | 20 | plaintext config apps/mobile/.env.sample |  |  | 0.641 |
| walker |  | 3261 | 39 | ts decl packages/lib/generatePreview.ts:5 |  |  | 0.641 |
| walker |  | 3320 | 59 | listing of 'apps/mobile/components' |  |  | 0.642 |
| walker |  | 3346 | 26 | listing of 'apps/mobile/components/Formats' |  |  | 0.642 |
| walker |  | 3378 | 32 | listing of 'apps/mobile/components/ActionSheets' |  |  | 0.642 |
| walker |  | 3420 | 42 | ts names packages/types/inputSelect.ts |  |  | 0.642 |
| ns | 3436 |  | 381 | `UsersAndCollections` join model (the permission bits) and `Tag` model | 2.7 | 2.1 | 0.606 |
| walker |  | 3451 | 31 | ts decl packages/types/inputSelect.ts:16 |  |  | 0.606 |
| walker |  | 3486 | 35 | ts decl packages/types/inputSelect.ts:1 |  |  | 0.606 |
| walker |  | 3527 | 41 | ts decl packages/lib/transporter.ts:3 |  |  | 0.606 |
| walker |  | 3589 | 62 | listing of 'apps/web/hooks' |  |  | 0.607 |
| ns | 3594 |  | 158 | `AccessToken` model | 2.8 | 2.1 | 0.594 |
| walker |  | 3607 | 18 | listing of 'apps/web/e2e' |  |  | 0.595 |
| walker |  | 3613 | 6 | listing of 'apps/web/e2e/tests' |  |  | 0.595 |
| ns | 3617 |  | 23 | Complete packages/prisma listing | 2.9 |  | 0.601 |
| walker |  | 3659 | 46 | ts names packages/router/worker.tsx |  |  | 0.601 |
| walker |  | 3706 | 47 | ts names packages/lib/utils.ts |  |  | 0.601 |
| ns | 3733 |  | 116 | The process-wide `prisma` client singleton | 2.10 |  | 0.603 |
| ns | 3817 |  | 84 | API version directories: every resource under pages/api/v1 (and the single v2 route) | 3.1 |  | 0.577 |
| ns | 3885 |  | 68 | Route files for links, collections, tags and highlights | 3.2 |  | 0.563 |
| ns | 3962 |  | 77 | Route files for auth, session, users, tokens, config, avatar and logins | 3.3 |  | 0.549 |
| walker |  | 3991 | 285 | Prisma decl at packages/prisma/schema.prisma:28 |  |  | 0.593 |
| ns | 4042 |  | 80 | Route files for archives, preserved, search, dashboard, rss, migration, payment, webhook, worker, getFavicon | 3.4 |  | 0.579 |
| ns | 4083 |  | 41 | The unauthenticated `public/` route subtree | 3.5 |  | 0.571 |
| walker |  | 4128 | 137 | headings outline in README.md |  |  | 0.571 |
| walker |  | 4154 | 26 | README.md section #3 |  |  | 0.571 |
| walker |  | 4196 | 42 | listing of 'apps/worker/lib/preservationScheme' |  |  | 0.572 |
| walker |  | 4247 | 51 | ts names packages/router/dashboardData.tsx |  |  | 0.572 |
| walker |  | 4343 | 96 | package runtime metadata in package.json |  |  | 0.572 |
| ns | 4359 |  | 276 | The route-handler pattern, read from pages/api/v1/links/index.ts | 3.6 |  | 0.557 |
| walker |  | 4370 | 27 | Prisma decl at packages/prisma/schema.prisma:5 |  |  | 0.569 |
| walker |  | 4421 | 51 | ts decl packages/lib/formatStats.ts:4 |  |  | 0.569 |
| walker |  | 4500 | 79 | listing of 'apps/web/public' |  |  | 0.569 |
| walker |  | 4510 | 10 | listing of 'apps/web/public/screenshots' |  |  | 0.569 |
| walker |  | 4563 | 53 | ts decl packages/lib/getFormatBasedOnPreference.ts:15 |  |  | 0.569 |
| ns | 4568 |  | 209 | apps/web/lib: the server helper layer and its client/shared siblings | 3.7 |  | 0.544 |
| ns | 4667 |  | 99 | The controller tree: every resource directory under lib/api/controllers | 3.8 |  | 0.530 |
| ns | 4823 |  | 156 | Controller files for links, collections, tags and highlights | 3.9 |  | 0.515 |
| walker |  | 4883 | 320 | Prisma decl at packages/prisma/schema.prisma:126 |  |  | 0.552 |
| walker |  | 4938 | 55 | ts decl packages/lib/meilisearchClient.ts:5 |  |  | 0.552 |
| ns | 5001 |  | 178 | Controller files for users, tokens, session, search, dashboard, worker, migration and public access | 3.10 |  | 0.534 |
| walker |  | 5072 | 134 | plaintext config .env.sample |  |  | 0.535 |
| walker |  | 5121 | 49 | listing of 'apps/web/public/locales' |  |  | 0.535 |
| walker |  | 5125 | 4 | listing of 'apps/web/public/locales/de' |  |  | 0.535 |
| walker |  | 5129 | 4 | listing of 'apps/web/public/locales/en' |  |  | 0.535 |
| walker |  | 5133 | 4 | listing of 'apps/web/public/locales/es' |  |  | 0.535 |
| walker |  | 5137 | 4 | listing of 'apps/web/public/locales/fr' |  |  | 0.535 |
| walker |  | 5141 | 4 | listing of 'apps/web/public/locales/it' |  |  | 0.535 |
| walker |  | 5145 | 4 | listing of 'apps/web/public/locales/ja' |  |  | 0.535 |
| walker |  | 5149 | 4 | listing of 'apps/web/public/locales/nl' |  |  | 0.535 |
| walker |  | 5153 | 4 | listing of 'apps/web/public/locales/pl' |  |  | 0.535 |
| walker |  | 5157 | 4 | listing of 'apps/web/public/locales/pt-BR' |  |  | 0.535 |
| walker |  | 5161 | 4 | listing of 'apps/web/public/locales/ro' |  |  | 0.535 |
| walker |  | 5165 | 4 | listing of 'apps/web/public/locales/ru' |  |  | 0.535 |
| walker |  | 5169 | 4 | listing of 'apps/web/public/locales/tr' |  |  | 0.535 |
| walker |  | 5173 | 4 | listing of 'apps/web/public/locales/uk' |  |  | 0.535 |
| walker |  | 5177 | 4 | listing of 'apps/web/public/locales/zh' |  |  | 0.535 |
| walker |  | 5181 | 4 | listing of 'apps/web/public/locales/zh-TW' |  |  | 0.535 |
| walker |  | 5210 | 29 | README.md section #6 |  |  | 0.535 |
| ns | 5249 |  | 248 | Complete listings of packages/types, packages/lib, packages/filesystem and packages/router | 4.1 |  | 0.569 |
| walker |  | 5270 | 60 | ts names packages/router/users.tsx |  |  | 0.569 |
| walker |  | 5320 | 50 | listing of 'apps/mobile/components/ui' |  |  | 0.570 |
| walker |  | 5406 | 86 | listing of 'apps/web/pages' |  |  | 0.570 |
| walker |  | 5420 | 14 | ts names apps/web/pages/index.tsx |  |  | 0.570 |
| walker |  | 5426 | 6 | listing of 'apps/web/pages/preserved' |  |  | 0.571 |
| walker |  | 5434 | 8 | listing of 'apps/web/pages/api' |  |  | 0.571 |
| walker |  | 5437 | 3 | listing of 'apps/web/pages/api/v2' |  |  | 0.571 |
| walker |  | 5446 | 9 | listing of 'apps/web/pages/public' |  |  | 0.571 |
| walker |  | 5451 | 5 | listing of 'apps/web/pages/public/collections' |  |  | 0.571 |
| walker |  | 5455 | 4 | listing of 'apps/web/pages/api/v2/dashboard' |  |  | 0.572 |
| walker |  | 5461 | 6 | listing of 'apps/web/pages/public/links' |  |  | 0.572 |
| walker |  | 5467 | 6 | listing of 'apps/web/pages/public/preserved' |  |  | 0.572 |
| walker |  | 5478 | 11 | listing of 'apps/web/pages/collections' |  |  | 0.572 |
| walker |  | 5489 | 11 | listing of 'apps/web/pages/tags' |  |  | 0.572 |
| walker |  | 5501 | 12 | listing of 'apps/web/pages/auth' |  |  | 0.572 |
| walker |  | 5517 | 16 | listing of 'apps/web/pages/links' |  |  | 0.572 |
| walker |  | 5536 | 19 | listing of 'apps/web/pages/admin' |  |  | 0.573 |
| walker |  | 5546 | 10 | listing of 'apps/web/pages/public/collections/[id]' |  |  | 0.573 |
| walker |  | 5589 | 43 | ts body apps/web/pages/index.tsx:4 |  |  | 0.573 |
| ns | 5647 |  | 398 | packages/types/global.ts — every exported type, interface and enum declaration | 4.2 |  | 0.558 |
| ns | 5793 |  | 146 | `ArchivedFormat`, `LinkType` and `TokenExpiry` variants | 4.3 | 4.2 | 0.551 |
| ns | 5945 |  | 152 | `ViewMode`, `Sort` and `TagSort` variants | 4.4 | 4.2 | 0.544 |
| walker |  | 5962 | 373 | Prisma decl tail at packages/prisma/schema.prisma:28 body 51 |  |  | 0.578 |
| walker |  | 6024 | 62 | ts names packages/router/tokens.tsx |  |  | 0.578 |
| walker |  | 6040 | 16 | Prisma decl at packages/prisma/schema.prisma:1 |  |  | 0.584 |
| walker |  | 6105 | 65 | ts names packages/filesystem/manageFiles.ts |  |  | 0.584 |
| walker |  | 6122 | 17 | listing of 'apps/web/e2e/fixtures' |  |  | 0.584 |
| walker |  | 6135 | 13 | listing of 'apps/web/e2e/fixtures/base' |  |  | 0.585 |
| walker |  | 6202 | 67 | ts names packages/router/user.tsx |  |  | 0.585 |
| walker |  | 6258 | 56 | listing of 'apps/web/pages/settings' |  |  | 0.586 |
| ns | 6285 |  | 340 | packages/lib/schemaValidation.ts — every exported zod schema constant | 4.5 |  | 0.575 |
| walker |  | 6330 | 72 | ts names packages/router/highlights.tsx |  |  | 0.575 |
| walker |  | 6355 | 25 | ts decl packages/router/highlights.tsx:11 |  |  | 0.575 |
| walker |  | 6427 | 72 | ts names packages/router/rss.tsx |  |  | 0.575 |
| walker |  | 6439 | 12 | ts body packages/lib/utils.ts:17 |  |  | 0.575 |
| walker |  | 6513 | 74 | ts decl packages/filesystem/createFile.ts:6 |  |  | 0.575 |
| walker |  | 6535 | 22 | listing of '.github/ISSUE_TEMPLATE' |  |  | 0.575 |
| ns | 6542 |  | 257 | packages/router — the links, collections and tags hook exports | 4.6 |  | 0.562 |
| walker |  | 6618 | 83 | ts decl packages/types/inputSelect.ts:7 |  |  | 0.562 |
| walker |  | 6708 | 90 | ts names packages/router/config.tsx |  |  | 0.563 |
| ns | 6726 |  | 184 | packages/router — export lines of the remaining ten hook modules | 4.7 | 4.6 | 0.568 |
| walker |  | 6738 | 30 | ts decl packages/router/config.tsx:44 |  |  | 0.568 |
| walker |  | 6827 | 89 | ts decl packages/lib/isArchivalTag.ts:3 |  |  | 0.568 |
| ns | 6892 |  | 166 | Complete apps/worker listing, including every job and every preservation handler | 5.1 |  | 0.584 |
| ns | 7161 |  | 269 | worker.ts — the whole scheduler entry point | 5.2 |  | 0.575 |
| walker |  | 7262 | 435 | README.md section #1 |  |  | 0.609 |
| walker |  | 7341 | 79 | listing of 'apps/web/lib/client' |  |  | 0.618 |
| ns | 7354 |  | 193 | archiveHandler's signature and its SSRF / skip-preservation guard | 5.3 |  | 0.610 |
| walker |  | 7596 | 255 | ts names packages/lib/schemaValidation.ts |  |  | 0.615 |
| ns | 7607 |  | 253 | Every UI page route under apps/web/pages | 6.1 |  | 0.635 |
| walker |  | 7609 | 13 | ts decl packages/lib/schemaValidation.ts:25 |  |  | 0.635 |
| walker |  | 7623 | 14 | ts decl packages/lib/schemaValidation.ts:115 |  |  | 0.635 |
| walker |  | 7638 | 15 | ts decl packages/lib/schemaValidation.ts:16 |  |  | 0.635 |
| walker |  | 7665 | 27 | ts decl packages/lib/schemaValidation.ts:20 |  |  | 0.635 |
| walker |  | 7694 | 29 | ts decl packages/lib/schemaValidation.ts:29 |  |  | 0.635 |
| walker |  | 7748 | 54 | ts decl packages/lib/schemaValidation.ts:119 |  |  | 0.635 |
| ns | 7965 |  | 358 | The flat components/ directory and the ui/ primitives | 6.2 |  | 0.609 |
| walker |  | 8043 | 295 | README.md section #0 |  |  | 0.609 |
| walker |  | 8154 | 111 | ts names packages/router/collections.tsx |  |  | 0.611 |
| walker |  | 8234 | 80 | ts decl packages/router/collections.tsx:180 |  |  | 0.611 |
| ns | 8246 |  | 281 | Modal, link-view, preservation and input-picker component subdirectories | 6.3 | 6.2 | 0.595 |
| walker |  | 8343 | 109 | ts decl packages/router/config.tsx:4 |  |  | 0.595 |
| walker |  | 8361 | 18 | ts body packages/router/config.tsx:44 |  |  | 0.595 |
| walker |  | 8458 | 97 | listing of 'apps/web/lib/api' |  |  | 0.618 |
| walker |  | 8471 | 13 | listing of 'apps/web/lib/api/archives' |  |  | 0.618 |
| ns | 8473 |  | 227 | Web hooks, layouts, stores, ambient types, email templates, one-off migration scripts and the Playwright suite | 6.4 |  | 0.622 |
| walker |  | 8488 | 17 | listing of 'apps/web/lib/api/preserved' |  |  | 0.623 |
| walker |  | 8521 | 33 | listing of 'apps/web/lib/api/stripe' |  |  | 0.626 |
| walker |  | 8557 | 36 | listing of 'apps/web/lib/api/controllers' |  |  | 0.639 |
| walker |  | 8562 | 5 | listing of 'apps/web/lib/api/controllers/search' |  |  | 0.639 |
| walker |  | 8567 | 5 | listing of 'apps/web/lib/api/controllers/session' |  |  | 0.639 |
| walker |  | 8573 | 6 | listing of 'apps/web/lib/api/controllers/worker' |  |  | 0.640 |
| walker |  | 8582 | 9 | listing of 'apps/web/lib/api/controllers/public' |  |  | 0.640 |
| walker |  | 8586 | 4 | listing of 'apps/web/lib/api/controllers/public/links' |  |  | 0.640 |
| walker |  | 8592 | 6 | listing of 'apps/web/lib/api/controllers/public/collections' |  |  | 0.641 |
| walker |  | 8598 | 6 | listing of 'apps/web/lib/api/controllers/public/users' |  |  | 0.641 |
| walker |  | 8605 | 7 | listing of 'apps/web/lib/api/controllers/public/links/linkId' |  |  | 0.642 |
| walker |  | 8619 | 14 | listing of 'apps/web/lib/api/controllers/collections' |  |  | 0.642 |
| walker |  | 8633 | 14 | listing of 'apps/web/lib/api/controllers/highlights' |  |  | 0.642 |
| walker |  | 8647 | 14 | listing of 'apps/web/lib/api/controllers/tokens' |  |  | 0.644 |
| walker |  | 8654 | 7 | listing of 'apps/web/lib/api/controllers/tokens/tokenId' |  |  | 0.645 |
| walker |  | 8668 | 14 | listing of 'apps/web/lib/api/controllers/users' |  |  | 0.647 |
| walker |  | 8685 | 17 | listing of 'apps/web/lib/api/controllers/links' |  |  | 0.649 |
| walker |  | 8697 | 12 | listing of 'apps/web/lib/api/controllers/links/bulk' |  |  | 0.650 |
| ns | 8713 |  | 240 | verifyUser — the guard chain every authenticated route runs first | 7.1 |  | 0.640 |
| walker |  | 8717 | 20 | listing of 'apps/web/lib/api/controllers/dashboard' |  |  | 0.643 |
| walker |  | 8744 | 27 | listing of 'apps/web/lib/api/controllers/tags' |  |  | 0.647 |
| walker |  | 8765 | 21 | listing of 'apps/web/lib/api/controllers/collections/collectionId' |  |  | 0.649 |
| walker |  | 8786 | 21 | listing of 'apps/web/lib/api/controllers/tags/tagId' |  |  | 0.653 |
| walker |  | 8810 | 24 | listing of 'apps/web/lib/api/controllers/links/linkId' |  |  | 0.658 |
| walker |  | 8816 | 6 | listing of 'apps/web/lib/api/controllers/links/linkId/highlight' |  |  | 0.659 |
| walker |  | 8843 | 27 | listing of 'apps/web/lib/api/controllers/users/userId' |  |  | 0.664 |
| ns | 8948 |  | 235 | docker-compose.yml — the three-container deployment | 7.2 |  | 0.656 |
| walker |  | 8962 | 119 | ts decl packages/lib/getPreservedFormatUrl.ts:3 |  |  | 0.656 |
| walker |  | 9031 | 69 | listing of 'apps/web/pages/api/v1' |  |  | 0.675 |
| walker |  | 9035 | 4 | listing of 'apps/web/pages/api/v1/dashboard' |  |  | 0.675 |
| walker |  | 9039 | 4 | listing of 'apps/web/pages/api/v1/getFavicon' |  |  | 0.675 |
| walker |  | 9043 | 4 | listing of 'apps/web/pages/api/v1/logins' |  |  | 0.675 |
| walker |  | 9047 | 4 | listing of 'apps/web/pages/api/v1/migration' |  |  | 0.675 |
| walker |  | 9051 | 4 | listing of 'apps/web/pages/api/v1/payment' |  |  | 0.676 |
| walker |  | 9055 | 4 | listing of 'apps/web/pages/api/v1/search' |  |  | 0.676 |
| walker |  | 9059 | 4 | listing of 'apps/web/pages/api/v1/session' |  |  | 0.676 |
| walker |  | 9063 | 4 | listing of 'apps/web/pages/api/v1/webhook' |  |  | 0.677 |
| walker |  | 9069 | 6 | listing of 'apps/web/pages/api/v1/avatar' |  |  | 0.677 |
| walker |  | 9078 | 9 | listing of 'apps/web/pages/api/v1/config' |  |  | 0.678 |
| walker |  | 9087 | 9 | listing of 'apps/web/pages/api/v1/public' |  |  | 0.678 |
| walker |  | 9093 | 6 | listing of 'apps/web/pages/api/v1/public/links' |  |  | 0.679 |
| walker |  | 9099 | 6 | listing of 'apps/web/pages/api/v1/public/users' |  |  | 0.680 |
| walker |  | 9108 | 9 | listing of 'apps/web/pages/api/v1/worker' |  |  | 0.681 |
| walker |  | 9118 | 10 | listing of 'apps/web/pages/api/v1/collections' |  |  | 0.681 |
| walker |  | 9128 | 10 | listing of 'apps/web/pages/api/v1/highlights' |  |  | 0.682 |
| walker |  | 9138 | 10 | listing of 'apps/web/pages/api/v1/rss' |  |  | 0.683 |
| walker |  | 9148 | 10 | listing of 'apps/web/pages/api/v1/tokens' |  |  | 0.684 |
| walker |  | 9160 | 12 | listing of 'apps/web/pages/api/v1/links' |  |  | 0.686 |
| ns | 9162 |  | 214 | .env.sample — the required variables and the complete list of section headings | 7.3 |  | 0.682 |
| walker |  | 9164 | 4 | listing of 'apps/web/pages/api/v1/links/archive' |  |  | 0.683 |
| walker |  | 9177 | 13 | listing of 'apps/web/pages/api/v1/users' |  |  | 0.685 |
| walker |  | 9186 | 9 | listing of 'apps/web/pages/api/v1/users/[id]' |  |  | 0.687 |
| walker |  | 9200 | 14 | listing of 'apps/web/pages/api/v1/tags' |  |  | 0.690 |
| walker |  | 9210 | 10 | listing of 'apps/web/pages/api/v1/links/[id]' |  |  | 0.693 |
| walker |  | 9214 | 4 | listing of 'apps/web/pages/api/v1/links/[id]/archive' |  |  | 0.695 |
| walker |  | 9218 | 4 | listing of 'apps/web/pages/api/v1/links/[id]/highlights' |  |  | 0.696 |
| walker |  | 9230 | 12 | listing of 'apps/web/pages/api/v1/public/collections' |  |  | 0.699 |
| walker |  | 9234 | 4 | listing of 'apps/web/pages/api/v1/public/collections/links' |  |  | 0.700 |
| walker |  | 9238 | 4 | listing of 'apps/web/pages/api/v1/public/collections/tags' |  |  | 0.701 |
| walker |  | 9256 | 18 | listing of 'apps/web/pages/api/v1/preserved' |  |  | 0.705 |
| walker |  | 9275 | 19 | listing of 'apps/web/pages/api/v1/archives' |  |  | 0.709 |
| walker |  | 9297 | 22 | listing of 'apps/web/pages/api/v1/auth' |  |  | 0.714 |
| ns | 9341 |  | 179 | The head of the optional tuning-variable block | 7.4 | 7.3 | 0.707 |
| walker |  | 9345 | 48 | listing of 'apps/web/lib/api/controllers/migration' |  |  | 0.715 |
| ns | 9473 |  | 132 | CI workflows, GitHub templates, the patch-package patch, and every translated locale | 7.5 |  | 0.721 |
| walker |  | 9655 | 310 | plaintext config docker-compose.yml |  |  | 0.734 |
| walker |  | 9677 | 22 | ts body packages/lib/utils.ts:13 |  |  | 0.734 |
| ns | 9689 |  | 216 | apps/mobile root and the complete Expo Router screen tree | 8.1 |  | 0.741 |
| walker |  | 9700 | 23 | ts body packages/lib/formatStats.ts:4 |  |  | 0.741 |
| ns | 9891 |  | 202 | Mobile components, stores and query-cache modules | 8.2 | 8.1 | 0.746 |

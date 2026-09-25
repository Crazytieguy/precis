Score(3000)=0.643 I=0.848 C=0.487 ns_rows≤3K=15/51 grid(1000/1442/2080/3000/4327/6240/9000)=0.622/0.546/0.607/0.643/0.528/0.512/0.547

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 75 |  | 75 | Product identity: name, tagline, one-sentence definition | 1.1 |  | 0.000 |
| walker |  | 79 | 79 | listing of '.' |  |  | 0.000 |
| walker |  | 88 | 9 | listing of 'apps' |  |  | 0.000 |
| walker |  | 99 | 11 | listing of 'patches' |  |  | 0.000 |
| walker |  | 114 | 15 | listing of 'packages' |  |  | 0.000 |
| walker |  | 127 | 13 | ts names vitest.config.mts |  |  | 0.000 |
| walker |  | 131 | 4 | listing of '.vscode' |  |  | 0.000 |
| walker |  | 144 | 13 | listing of 'packages/types' |  |  | 0.000 |
| ns | 154 |  | 79 | Complete repository root listing | 1.2 |  | 0.721 |
| ns | 178 |  | 24 | The three apps and five shared packages | 1.3 |  | 0.708 |
| walker |  | 187 | 43 | package identity in package.json |  |  | 0.711 |
| walker |  | 220 | 33 | listing of 'assets' |  |  | 0.711 |
| walker |  | 225 | 5 | listing of '.devcontainer' |  |  | 0.711 |
| walker |  | 248 | 23 | listing of 'packages/prisma' |  |  | 0.714 |
| ns | 259 |  | 81 | Complete apps/web listing | 1.4 |  | 0.533 |
| walker |  | 285 | 37 | ts names packages/prisma/index.ts |  |  | 0.534 |
| walker |  | 294 | 9 | listing of 'packages/prisma/client' |  |  | 0.534 |
| walker |  | 306 | 12 | ts names packages/prisma/client/index.js |  |  | 0.534 |
| ns | 321 |  | 62 | Workspace globs in the root package.json | 1.5 |  | 0.507 |
| walker |  | 382 | 76 | ts decl packages/prisma/index.ts:5 |  |  | 0.509 |
| walker |  | 408 | 26 | listing of 'apps/worker' |  |  | 0.509 |
| walker |  | 420 | 12 | ts names apps/worker/index.ts |  |  | 0.509 |
| walker |  | 425 | 5 | listing of 'apps/worker/templates' |  |  | 0.510 |
| walker |  | 459 | 34 | package entrypoints in package.json |  |  | 0.561 |
| ns | 460 |  | 139 | Workspace package names — every `name` field under apps/ and packages/ | 1.6 |  | 0.519 |
| walker |  | 513 | 54 | listing of 'packages/filesystem' |  |  | 0.520 |
| walker |  | 613 | 100 | ts names packages/filesystem/index.ts |  |  | 0.520 |
| walker |  | 627 | 14 | ts names packages/filesystem/createFile.ts |  |  | 0.520 |
| walker |  | 646 | 19 | ts names packages/filesystem/readFile.ts |  |  | 0.520 |
| ns | 674 |  | 214 | Root scripts: how you run web, worker and both together | 1.7 |  | 0.482 |
| walker |  | 733 | 87 | ts decl vitest.config.mts:4 |  |  | 0.482 |
| ns | 837 |  | 163 | Root scripts: prisma, format, test, coverage, postinstall | 1.8 |  | 0.450 |
| walker |  | 870 | 137 | README headline in README.md |  |  | 0.622 |
| walker |  | 892 | 22 | ts names packages/filesystem/fileExists.ts |  |  | 0.622 |
| walker |  | 914 | 22 | ts names packages/filesystem/moveFile.ts |  |  | 0.622 |
| walker |  | 937 | 23 | ts names packages/filesystem/createFolder.ts |  |  | 0.622 |
| walker |  | 961 | 24 | ts names packages/filesystem/removeFile.ts |  |  | 0.622 |
| walker |  | 985 | 24 | ts names packages/filesystem/removeFolder.ts |  |  | 0.622 |
| ns | 1082 |  | 245 | Feature list, first half (preservation, reading, organisation, sharing) | 1.9 |  | 0.575 |
| walker |  | 1205 | 220 | Prisma schema TOC in packages/prisma/schema.prisma |  |  | 0.584 |
| walker |  | 1241 | 36 | listing of 'apps/worker/workers' |  |  | 0.584 |
| walker |  | 1256 | 15 | ts names apps/worker/workers/migrationWorker.ts |  |  | 0.584 |
| ns | 1271 |  | 189 | Feature list, second half (sync, SSO, API keys, i18n, RSS, uploads) | 1.10 |  | 0.545 |
| walker |  | 1328 | 72 | listing of 'packages/router' |  |  | 0.546 |
| walker |  | 1344 | 16 | ts names apps/worker/workers/rssPolling.ts |  |  | 0.546 |
| walker |  | 1372 | 28 | ts names packages/filesystem/s3Client.ts |  |  | 0.546 |
| walker |  | 1389 | 17 | ts names apps/worker/workers/trialEndEmailWorker.ts |  |  | 0.546 |
| walker |  | 1406 | 17 | ts names packages/prisma/client/index.d.ts |  |  | 0.546 |
| walker |  | 1486 | 80 | listing of 'apps/mobile' |  |  | 0.547 |
| walker |  | 1490 | 4 | listing of 'apps/mobile/styles' |  |  | 0.547 |
| walker |  | 1496 | 6 | listing of 'apps/mobile/assets' |  |  | 0.547 |
| walker |  | 1504 | 8 | listing of 'apps/mobile/plugins' |  |  | 0.547 |
| walker |  | 1510 | 6 | listing of 'apps/mobile/assets/fonts' |  |  | 0.547 |
| walker |  | 1521 | 11 | listing of 'apps/mobile/types' |  |  | 0.547 |
| walker |  | 1533 | 12 | listing of 'apps/mobile/store' |  |  | 0.547 |
| ns | 1539 |  | 268 | schema.prisma: datasource/generator plus every model and enum header, bodies elided | 2.1 |  | 0.530 |
| walker |  | 1545 | 12 | ts names apps/mobile/tailwind.config.js |  |  | 0.530 |
| walker |  | 1559 | 14 | ts names apps/mobile/babel.config.js |  |  | 0.530 |
| walker |  | 1582 | 23 | listing of 'apps/mobile/lib' |  |  | 0.530 |
| walker |  | 1605 | 23 | ts names apps/mobile/metro.config.js |  |  | 0.530 |
| walker |  | 1641 | 36 | listing of 'apps/mobile/app' |  |  | 0.531 |
| walker |  | 1656 | 15 | ts names apps/mobile/app/index.tsx |  |  | 0.531 |
| walker |  | 1662 | 6 | listing of 'apps/mobile/app/links' |  |  | 0.531 |
| walker |  | 1683 | 21 | listing of 'apps/mobile/app/(tabs)' |  |  | 0.531 |
| walker |  | 1694 | 11 | listing of 'apps/mobile/app/(tabs)/links' |  |  | 0.531 |
| ns | 1697 |  | 158 | All five schema enums, fully expanded | 2.2 | 2.1 | 0.495 |
| walker |  | 1709 | 15 | ts names apps/mobile/app/incoming.tsx |  |  | 0.495 |
| walker |  | 1724 | 15 | ts names apps/mobile/app/login.tsx |  |  | 0.495 |
| walker |  | 1740 | 16 | ts names apps/mobile/app/+not-found.tsx |  |  | 0.495 |
| walker |  | 1764 | 24 | listing of 'apps/mobile/assets/images' |  |  | 0.495 |
| walker |  | 1845 | 81 | listing of 'apps/web' |  |  | 0.606 |
| walker |  | 1849 | 4 | listing of 'apps/web/styles' |  |  | 0.606 |
| walker |  | 1858 | 9 | listing of 'apps/web/store' |  |  | 0.606 |
| walker |  | 1870 | 12 | listing of 'apps/web/types' |  |  | 0.606 |
| walker |  | 1883 | 13 | listing of 'apps/web/lib' |  |  | 0.606 |
| walker |  | 1893 | 10 | ts names apps/web/postcss.config.js |  |  | 0.606 |
| walker |  | 1905 | 12 | ts names apps/web/next-i18next.config.js |  |  | 0.606 |
| walker |  | 1917 | 12 | ts names apps/web/tailwind.config.js |  |  | 0.606 |
| walker |  | 1938 | 21 | listing of 'apps/web/templates' |  |  | 0.606 |
| walker |  | 1962 | 24 | listing of 'apps/web/layouts' |  |  | 0.606 |
| walker |  | 1985 | 23 | ts names apps/web/next.config.js |  |  | 0.606 |
| walker |  | 1999 | 14 | ts names apps/web/types/himalaya.d.ts |  |  | 0.606 |
| walker |  | 2019 | 20 | listing of 'apps/web/lib/shared' |  |  | 0.606 |
| walker |  | 2048 | 29 | listing of '.github' |  |  | 0.606 |
| walker |  | 2069 | 21 | listing of '.github/workflows' |  |  | 0.607 |
| ns | 2083 |  | 386 | `Link` model — every field | 2.3 | 2.1 | 0.552 |
| walker |  | 2087 | 18 | ts names apps/mobile/app/_layout.tsx |  |  | 0.552 |
| walker |  | 2111 | 24 | Prisma decl at packages/prisma/schema.prisma:77 |  |  | 0.557 |
| walker |  | 2128 | 17 | listing of 'apps/mobile/app/(tabs)/collections' |  |  | 0.557 |
| walker |  | 2145 | 17 | listing of 'apps/mobile/app/(tabs)/dashboard' |  |  | 0.557 |
| walker |  | 2162 | 17 | listing of 'apps/mobile/app/(tabs)/settings' |  |  | 0.557 |
| walker |  | 2179 | 17 | listing of 'apps/mobile/app/(tabs)/tags' |  |  | 0.558 |
| walker |  | 2228 | 49 | Prisma decl at packages/prisma/schema.prisma:90 |  |  | 0.574 |
| walker |  | 2247 | 19 | ts names apps/mobile/lib/queryPersister.ts |  |  | 0.574 |
| walker |  | 2266 | 19 | ts names apps/web/layouts/AdminLayout.tsx |  |  | 0.574 |
| walker |  | 2285 | 19 | ts names apps/web/layouts/AuthRedirect.tsx |  |  | 0.574 |
| walker |  | 2304 | 19 | ts names apps/web/layouts/MainLayout.tsx |  |  | 0.574 |
| walker |  | 2323 | 19 | ts names apps/web/layouts/SettingsLayout.tsx |  |  | 0.574 |
| walker |  | 2342 | 19 | ts names apps/worker/workers/linkProcessing.ts |  |  | 0.574 |
| walker |  | 2375 | 33 | ts names packages/router/publicLinks.tsx |  |  | 0.574 |
| ns | 2396 |  | 313 | `User` model — identity, relations and subscription linkage | 2.4 | 2.1 | 0.538 |
| walker |  | 2408 | 33 | ts names packages/router/publicTags.tsx |  |  | 0.538 |
| walker |  | 2428 | 20 | ts names apps/mobile/plugins/with-daynight-transparent-nav.js |  |  | 0.538 |
| walker |  | 2448 | 20 | ts names apps/worker/workers/linkIndexing.ts |  |  | 0.538 |
| walker |  | 2474 | 26 | Prisma decl at packages/prisma/schema.prisma:304 |  |  | 0.551 |
| walker |  | 2509 | 35 | Prisma decl at packages/prisma/schema.prisma:83 |  |  | 0.574 |
| walker |  | 2719 | 210 | Prisma decl at packages/prisma/schema.prisma:166 |  |  | 0.607 |
| ns | 2738 |  | 342 | `User` model — preference and archival-toggle fields | 2.5 | 2.4 | 0.576 |
| walker |  | 2898 | 179 | Prisma decl tail at packages/prisma/schema.prisma:166 body 182 |  |  | 0.643 |
| walker |  | 3007 | 109 | listing of 'packages/lib' |  |  | 0.646 |
| walker |  | 3020 | 13 | ts names packages/lib/transporter.ts |  |  | 0.646 |
| walker |  | 3034 | 14 | ts names packages/lib/constants.ts |  |  | 0.646 |
| walker |  | 3048 | 14 | ts names packages/lib/safeFetch.ts |  |  | 0.646 |
| ns | 3055 |  | 317 | `Collection` model — every field, including the self-relation | 2.6 | 2.1 | 0.616 |
| walker |  | 3063 | 15 | ts names packages/lib/generatePreview.ts |  |  | 0.616 |
| walker |  | 3078 | 15 | ts names packages/lib/rssHandler.ts |  |  | 0.616 |
| walker |  | 3094 | 16 | ts names packages/lib/verifyCapacity.ts |  |  | 0.616 |
| walker |  | 3111 | 17 | ts names packages/lib/meilisearchClient.ts |  |  | 0.616 |
| walker |  | 3129 | 18 | ts names packages/lib/getPreservedFormatUrl.ts |  |  | 0.616 |
| walker |  | 3152 | 23 | ts names packages/lib/isArchivalTag.ts |  |  | 0.616 |
| walker |  | 3176 | 24 | ts names packages/lib/getOriginalFormat.ts |  |  | 0.616 |
| walker |  | 3202 | 26 | ts decl packages/lib/getOriginalFormat.ts:6 |  |  | 0.616 |
| walker |  | 3230 | 28 | ts names packages/lib/formatStats.ts |  |  | 0.616 |
| walker |  | 3254 | 24 | ts decl packages/lib/formatStats.ts:11 |  |  | 0.616 |
| walker |  | 3282 | 28 | ts names packages/lib/getFormatBasedOnPreference.ts |  |  | 0.616 |
| walker |  | 3310 | 28 | ts names packages/lib/getLinkTypeFromFormat.ts |  |  | 0.616 |
| walker |  | 3338 | 28 | ts decl packages/lib/verifyCapacity.ts:8 |  |  | 0.616 |
| walker |  | 3368 | 30 | ts decl packages/lib/rssHandler.ts:7 |  |  | 0.616 |
| walker |  | 3398 | 30 | ts decl packages/lib/safeFetch.ts:95 |  |  | 0.616 |
| walker |  | 3432 | 34 | ts decl packages/lib/getLinkTypeFromFormat.ts:3 |  |  | 0.616 |
| ns | 3436 |  | 381 | `UsersAndCollections` join model (the permission bits) and `Tag` model | 2.7 | 2.1 | 0.581 |
| walker |  | 3469 | 37 | ts names apps/web/playwright.config.ts |  |  | 0.581 |
| walker |  | 3506 | 37 | ts names packages/lib/getFormatFromContentType.ts |  |  | 0.581 |
| walker |  | 3563 | 57 | listing of 'apps/worker/lib' |  |  | 0.582 |
| walker |  | 3578 | 15 | ts names apps/worker/lib/archiveHandler.ts |  |  | 0.582 |
| ns | 3594 |  | 158 | `AccessToken` model | 2.8 | 2.1 | 0.570 |
| walker |  | 3596 | 18 | ts names apps/worker/lib/getLinkBatchFairly.ts |  |  | 0.570 |
| walker |  | 3615 | 19 | ts names apps/worker/lib/countUnprocessedBillableLinks.ts |  |  | 0.570 |
| ns | 3617 |  | 23 | Complete packages/prisma listing | 2.9 |  | 0.577 |
| walker |  | 3634 | 19 | ts names apps/worker/lib/fetchHeaders.ts |  |  | 0.577 |
| walker |  | 3655 | 21 | ts names apps/worker/lib/protectPageRequests.ts |  |  | 0.577 |
| walker |  | 3677 | 22 | ts names apps/worker/workers/autoTagPreservedLinks.ts |  |  | 0.577 |
| walker |  | 3716 | 39 | Prisma decl at packages/prisma/schema.prisma:289 |  |  | 0.600 |
| ns | 3733 |  | 116 | The process-wide `prisma` client singleton | 2.10 |  | 0.601 |
| walker |  | 3753 | 37 | ts decl apps/web/postcss.config.js:1 |  |  | 0.601 |
| walker |  | 3773 | 20 | plaintext config apps/mobile/.env.sample |  |  | 0.601 |
| walker |  | 3788 | 15 | ts names apps/mobile/app/(tabs)/_layout.tsx |  |  | 0.601 |
| walker |  | 3803 | 15 | ts names apps/mobile/app/links/[id].tsx |  |  | 0.601 |
| ns | 3817 |  | 84 | API version directories: every resource under pages/api/v1 (and the single v2 route) | 3.1 |  | 0.575 |
| walker |  | 3842 | 39 | ts decl packages/lib/generatePreview.ts:5 |  |  | 0.575 |
| ns | 3885 |  | 68 | Route files for links, collections, tags and highlights | 3.2 |  | 0.561 |
| ns | 3962 |  | 77 | Route files for auth, session, users, tokens, config, avatar and logins | 3.3 |  | 0.548 |
| walker |  | 3977 | 135 | package runtime dependencies in package.json |  |  | 0.548 |
| walker |  | 4036 | 59 | listing of 'apps/mobile/components' |  |  | 0.548 |
| ns | 4042 |  | 80 | Route files for archives, preserved, search, dashboard, rss, migration, payment, webhook, worker, getFavicon | 3.4 |  | 0.535 |
| walker |  | 4050 | 14 | ts names apps/mobile/components/DashboardItem.tsx |  |  | 0.535 |
| walker |  | 4065 | 15 | ts names apps/mobile/components/ElementNotSupported.tsx |  |  | 0.535 |
| ns | 4083 |  | 41 | The unauthenticated `public/` route subtree | 3.5 |  | 0.527 |
| walker |  | 4091 | 26 | listing of 'apps/mobile/components/Formats' |  |  | 0.528 |
| walker |  | 4111 | 20 | ts names apps/mobile/components/Links.tsx |  |  | 0.528 |
| walker |  | 4143 | 32 | listing of 'apps/mobile/components/ActionSheets' |  |  | 0.528 |
| walker |  | 4151 | 8 | ts names apps/mobile/components/ActionSheets/Sheets.tsx |  |  | 0.528 |
| walker |  | 4174 | 23 | ts names apps/mobile/components/HapticTab.tsx |  |  | 0.528 |
| walker |  | 4189 | 15 | ts names apps/mobile/components/ActionSheets/SupportSheet.tsx |  |  | 0.528 |
| walker |  | 4231 | 42 | ts names packages/types/inputSelect.ts |  |  | 0.528 |
| walker |  | 4262 | 31 | ts decl packages/types/inputSelect.ts:16 |  |  | 0.528 |
| walker |  | 4297 | 35 | ts decl packages/types/inputSelect.ts:1 |  |  | 0.528 |
| walker |  | 4322 | 25 | ts names apps/mobile/lib/queryClient.ts |  |  | 0.528 |
| walker |  | 4347 | 25 | ts names apps/worker/lib/autoTagLink.ts |  |  | 0.528 |
| ns | 4359 |  | 276 | The route-handler pattern, read from pages/api/v1/links/index.ts | 3.6 |  | 0.514 |
| walker |  | 4363 | 16 | ts names apps/mobile/components/ActionSheets/AddLinkSheet.tsx |  |  | 0.514 |
| walker |  | 4379 | 16 | ts names apps/mobile/components/ActionSheets/EditLinkSheet.tsx |  |  | 0.514 |
| walker |  | 4395 | 16 | ts names apps/mobile/components/ActionSheets/NewCollectionSheet.tsx |  |  | 0.514 |
| walker |  | 4436 | 41 | ts decl packages/lib/transporter.ts:3 |  |  | 0.514 |
| walker |  | 4498 | 62 | listing of 'apps/web/hooks' |  |  | 0.514 |
| walker |  | 4512 | 14 | ts names apps/web/hooks/useSort.tsx |  |  | 0.514 |
| walker |  | 4528 | 16 | ts names apps/web/hooks/useInitialData.tsx |  |  | 0.514 |
| walker |  | 4544 | 16 | ts names apps/web/hooks/useWindowDimensions.tsx |  |  | 0.514 |
| walker |  | 4563 | 19 | ts names apps/web/hooks/usePermissions.tsx |  |  | 0.514 |
| ns | 4568 |  | 209 | apps/web/lib: the server helper layer and its client/shared siblings | 3.7 |  | 0.492 |
| walker |  | 4583 | 20 | ts names apps/web/hooks/useMediaQuery.tsx |  |  | 0.492 |
| walker |  | 4604 | 21 | ts names apps/web/hooks/useCollectivePermissions.ts |  |  | 0.492 |
| walker |  | 4628 | 24 | ts names apps/web/hooks/useOnScreen.tsx |  |  | 0.492 |
| walker |  | 4646 | 18 | listing of 'apps/web/e2e' |  |  | 0.493 |
| walker |  | 4652 | 6 | listing of 'apps/web/e2e/tests' |  |  | 0.493 |
| ns | 4667 |  | 99 | The controller tree: every resource directory under lib/api/controllers | 3.8 |  | 0.480 |
| walker |  | 4669 | 17 | ts names apps/web/lib/shared/fetchTitleAndHeaders.ts |  |  | 0.480 |
| walker |  | 4686 | 17 | ts names apps/web/lib/shared/isValidUrl.ts |  |  | 0.480 |
| walker |  | 4732 | 46 | ts names packages/router/worker.tsx |  |  | 0.480 |
| walker |  | 4758 | 26 | ts decl apps/worker/lib/getLinkBatchFairly.ts:12 |  |  | 0.480 |
| walker |  | 4805 | 47 | ts names packages/lib/utils.ts |  |  | 0.480 |
| ns | 4823 |  | 156 | Controller files for links, collections, tags and highlights | 3.9 |  | 0.466 |
| ns | 5001 |  | 178 | Controller files for users, tokens, session, search, dashboard, worker, migration and public access | 3.10 |  | 0.451 |
| walker |  | 5090 | 285 | Prisma decl at packages/prisma/schema.prisma:28 |  |  | 0.486 |
| walker |  | 5118 | 28 | ts names apps/mobile/components/CollectionListing.tsx |  |  | 0.486 |
| walker |  | 5146 | 28 | ts names apps/mobile/components/TagListing.tsx |  |  | 0.486 |
| walker |  | 5174 | 28 | ts names apps/web/hooks/useDetectPageBottom.tsx |  |  | 0.486 |
| ns | 5249 |  | 248 | Complete listings of packages/types, packages/lib, packages/filesystem and packages/router | 4.1 |  | 0.526 |
| walker |  | 5311 | 137 | headings outline in README.md |  |  | 0.526 |
| walker |  | 5337 | 26 | README.md section #26 |  |  | 0.526 |
| walker |  | 5348 | 11 | README.md section #18 |  |  | 0.526 |
| walker |  | 5358 | 10 | README.md section #17 |  |  | 0.526 |
| walker |  | 5368 | 10 | README.md section #19 |  |  | 0.526 |
| walker |  | 5410 | 42 | listing of 'apps/worker/lib/preservationScheme' |  |  | 0.527 |
| walker |  | 5426 | 16 | ts names apps/worker/lib/preservationScheme/handleMonolith.ts |  |  | 0.527 |
| walker |  | 5477 | 51 | ts names packages/router/dashboardData.tsx |  |  | 0.527 |
| walker |  | 5507 | 30 | ts names apps/mobile/components/LinkListing.tsx |  |  | 0.527 |
| walker |  | 5537 | 30 | ts names apps/mobile/lib/theme.ts |  |  | 0.527 |
| walker |  | 5567 | 30 | ts names apps/web/types/next-auth.d.ts |  |  | 0.527 |
| walker |  | 5596 | 29 | ts decl apps/worker/lib/archiveHandler.ts:25 |  |  | 0.527 |
| walker |  | 5627 | 31 | ts names apps/mobile/store/auth.ts |  |  | 0.527 |
| ns | 5647 |  | 398 | packages/types/global.ts — every exported type, interface and enum declaration | 4.2 |  | 0.513 |
| walker |  | 5654 | 27 | Prisma decl at packages/prisma/schema.prisma:5 |  |  | 0.523 |
| walker |  | 5705 | 51 | ts decl packages/lib/formatStats.ts:4 |  |  | 0.523 |
| walker |  | 5718 | 13 | README.md section #23 |  |  | 0.523 |
| walker |  | 5731 | 13 | README.md section #24 |  |  | 0.524 |
| ns | 5793 |  | 146 | `ArchivedFormat`, `LinkType` and `TokenExpiry` variants | 4.3 | 4.2 | 0.517 |
| walker |  | 5827 | 96 | package runtime metadata in package.json |  |  | 0.517 |
| walker |  | 5859 | 32 | ts names apps/mobile/components/DashboardSection.tsx |  |  | 0.517 |
| walker |  | 5873 | 14 | ts names apps/mobile/app/(tabs)/collections/_layout.tsx |  |  | 0.517 |
| walker |  | 5887 | 14 | ts names apps/mobile/app/(tabs)/dashboard/_layout.tsx |  |  | 0.517 |
| walker |  | 5901 | 14 | ts names apps/mobile/app/(tabs)/links/_layout.tsx |  |  | 0.517 |
| walker |  | 5915 | 14 | ts names apps/mobile/app/(tabs)/settings/_layout.tsx |  |  | 0.517 |
| walker |  | 5929 | 14 | ts names apps/mobile/app/(tabs)/tags/_layout.tsx |  |  | 0.517 |
| ns | 5945 |  | 152 | `ViewMode`, `Sort` and `TagSort` variants | 4.4 | 4.2 | 0.512 |
| walker |  | 6008 | 79 | listing of 'apps/web/public' |  |  | 0.512 |
| walker |  | 6018 | 10 | listing of 'apps/web/public/screenshots' |  |  | 0.512 |
| walker |  | 6071 | 53 | ts decl packages/lib/getFormatBasedOnPreference.ts:15 |  |  | 0.512 |
| walker |  | 6092 | 21 | ts names apps/worker/lib/preservationScheme/sendToWayback.ts |  |  | 0.512 |
| walker |  | 6125 | 33 | ts names apps/mobile/store/data.ts |  |  | 0.512 |
| walker |  | 6158 | 33 | ts names apps/web/hooks/useArchivalTags.ts |  |  | 0.512 |
| walker |  | 6191 | 33 | ts names apps/worker/lib/getLinkBatch.ts |  |  | 0.512 |
| walker |  | 6217 | 26 | ts decl apps/worker/lib/getLinkBatch.ts:11 |  |  | 0.512 |
| ns | 6285 |  | 340 | packages/lib/schemaValidation.ts — every exported zod schema constant | 4.5 |  | 0.502 |
| walker |  | 6537 | 320 | Prisma decl at packages/prisma/schema.prisma:126 |  |  | 0.532 |
| ns | 6542 |  | 257 | packages/router — the links, collections and tags hook exports | 4.6 |  | 0.520 |
| walker |  | 6592 | 55 | ts decl packages/lib/meilisearchClient.ts:5 |  |  | 0.520 |
| walker |  | 6626 | 34 | ts names apps/mobile/store/tmp.ts |  |  | 0.520 |
| walker |  | 6660 | 34 | ts names apps/web/store/links.ts |  |  | 0.520 |
| ns | 6726 |  | 184 | packages/router — export lines of the remaining ten hook modules | 4.7 | 4.6 | 0.517 |
| walker |  | 6794 | 134 | plaintext config .env.sample |  |  | 0.517 |
| walker |  | 6843 | 49 | listing of 'apps/web/public/locales' |  |  | 0.518 |
| walker |  | 6847 | 4 | listing of 'apps/web/public/locales/de' |  |  | 0.518 |
| walker |  | 6851 | 4 | listing of 'apps/web/public/locales/en' |  |  | 0.518 |
| walker |  | 6855 | 4 | listing of 'apps/web/public/locales/es' |  |  | 0.518 |
| walker |  | 6859 | 4 | listing of 'apps/web/public/locales/fr' |  |  | 0.518 |
| walker |  | 6863 | 4 | listing of 'apps/web/public/locales/it' |  |  | 0.518 |
| walker |  | 6867 | 4 | listing of 'apps/web/public/locales/ja' |  |  | 0.518 |
| walker |  | 6871 | 4 | listing of 'apps/web/public/locales/nl' |  |  | 0.518 |
| walker |  | 6875 | 4 | listing of 'apps/web/public/locales/pl' |  |  | 0.518 |
| walker |  | 6879 | 4 | listing of 'apps/web/public/locales/pt-BR' |  |  | 0.518 |
| walker |  | 6883 | 4 | listing of 'apps/web/public/locales/ro' |  |  | 0.518 |
| walker |  | 6887 | 4 | listing of 'apps/web/public/locales/ru' |  |  | 0.518 |
| walker |  | 6891 | 4 | listing of 'apps/web/public/locales/tr' |  |  | 0.518 |
| ns | 6892 |  | 166 | Complete apps/worker listing, including every job and every preservation handler | 5.1 |  | 0.537 |
| walker |  | 6895 | 4 | listing of 'apps/web/public/locales/uk' |  |  | 0.537 |
| walker |  | 6899 | 4 | listing of 'apps/web/public/locales/zh' |  |  | 0.537 |
| walker |  | 6903 | 4 | listing of 'apps/web/public/locales/zh-TW' |  |  | 0.537 |
| walker |  | 6917 | 14 | README.md section #21 |  |  | 0.537 |
| walker |  | 6929 | 12 | README.md section #22 |  |  | 0.539 |
| walker |  | 6943 | 14 | README.md section #20 |  |  | 0.540 |
| walker |  | 6958 | 15 | ts names apps/mobile/app/(tabs)/collections/[id].tsx |  |  | 0.540 |
| walker |  | 6973 | 15 | ts names apps/mobile/app/(tabs)/collections/index.tsx |  |  | 0.540 |
| walker |  | 6988 | 15 | ts names apps/mobile/app/(tabs)/dashboard/[section].tsx |  |  | 0.540 |
| walker |  | 7003 | 15 | ts names apps/mobile/app/(tabs)/dashboard/index.tsx |  |  | 0.540 |
| walker |  | 7018 | 15 | ts names apps/mobile/app/(tabs)/links/index.tsx |  |  | 0.540 |
| walker |  | 7033 | 15 | ts names apps/mobile/app/(tabs)/settings/index.tsx |  |  | 0.540 |
| walker |  | 7048 | 15 | ts names apps/mobile/app/(tabs)/tags/[id].tsx |  |  | 0.540 |
| walker |  | 7063 | 15 | ts names apps/mobile/app/(tabs)/tags/index.tsx |  |  | 0.540 |
| walker |  | 7098 | 35 | ts names apps/mobile/lib/colors.ts |  |  | 0.540 |
| walker |  | 7133 | 35 | ts names apps/web/store/localSettings.ts |  |  | 0.540 |
| ns | 7161 |  | 269 | worker.ts — the whole scheduler entry point | 5.2 |  | 0.531 |
| walker |  | 7162 | 29 | README.md section #29 |  |  | 0.531 |
| walker |  | 7222 | 60 | ts names packages/router/users.tsx |  |  | 0.532 |
| walker |  | 7272 | 50 | listing of 'apps/mobile/components/ui' |  |  | 0.533 |
| walker |  | 7285 | 13 | ts names apps/mobile/components/ui/IconSymbol.ios.tsx |  |  | 0.533 |
| walker |  | 7305 | 20 | ts names apps/mobile/components/ui/Icons.tsx |  |  | 0.533 |
| ns | 7354 |  | 193 | archiveHandler's signature and its SSRF / skip-preservation guard | 5.3 |  | 0.526 |
| walker |  | 7391 | 86 | listing of 'apps/web/pages' |  |  | 0.527 |
| walker |  | 7405 | 14 | ts names apps/web/pages/index.tsx |  |  | 0.527 |
| walker |  | 7411 | 6 | listing of 'apps/web/pages/preserved' |  |  | 0.527 |
| walker |  | 7419 | 8 | listing of 'apps/web/pages/api' |  |  | 0.527 |
| walker |  | 7422 | 3 | listing of 'apps/web/pages/api/v2' |  |  | 0.527 |
| walker |  | 7431 | 9 | listing of 'apps/web/pages/public' |  |  | 0.527 |
| walker |  | 7436 | 5 | listing of 'apps/web/pages/public/collections' |  |  | 0.527 |
| walker |  | 7440 | 4 | listing of 'apps/web/pages/api/v2/dashboard' |  |  | 0.528 |
| walker |  | 7446 | 6 | listing of 'apps/web/pages/public/links' |  |  | 0.528 |
| walker |  | 7452 | 6 | listing of 'apps/web/pages/public/preserved' |  |  | 0.528 |
| walker |  | 7463 | 11 | listing of 'apps/web/pages/collections' |  |  | 0.528 |
| walker |  | 7474 | 11 | listing of 'apps/web/pages/tags' |  |  | 0.528 |
| walker |  | 7486 | 12 | listing of 'apps/web/pages/auth' |  |  | 0.528 |
| walker |  | 7502 | 16 | listing of 'apps/web/pages/links' |  |  | 0.528 |
| walker |  | 7521 | 19 | listing of 'apps/web/pages/admin' |  |  | 0.528 |
| walker |  | 7535 | 14 | ts names apps/web/pages/_document.tsx |  |  | 0.528 |
| walker |  | 7545 | 10 | listing of 'apps/web/pages/public/collections/[id]' |  |  | 0.529 |
| walker |  | 7588 | 43 | ts body apps/web/pages/index.tsx:4 |  |  | 0.529 |
| ns | 7607 |  | 253 | Every UI page route under apps/web/pages | 6.1 |  | 0.539 |
| walker |  | 7613 | 25 | ts names apps/web/pages/forgot.tsx |  |  | 0.539 |
| walker |  | 7638 | 25 | ts names apps/web/pages/subscribe.tsx |  |  | 0.539 |
| walker |  | 7665 | 27 | ts names apps/web/pages/member-onboarding.tsx |  |  | 0.539 |
| walker |  | 7693 | 28 | ts names apps/web/pages/confirmation.tsx |  |  | 0.539 |
| walker |  | 7716 | 23 | ts names apps/mobile/components/Formats/PdfFormat.tsx |  |  | 0.539 |
| walker |  | 7739 | 23 | ts names apps/web/lib/shared/getSuffixFromFormat.ts |  |  | 0.539 |
| ns | 7965 |  | 358 | The flat components/ directory and the ui/ primitives | 6.2 |  | 0.518 |
| walker |  | 8112 | 373 | Prisma decl tail at packages/prisma/schema.prisma:28 body 51 |  |  | 0.544 |
| walker |  | 8174 | 62 | ts names packages/router/tokens.tsx |  |  | 0.545 |
| walker |  | 8190 | 16 | Prisma decl at packages/prisma/schema.prisma:1 |  |  | 0.549 |
| walker |  | 8214 | 24 | ts names apps/mobile/components/Formats/ReadableFormat.tsx |  |  | 0.549 |
| walker |  | 8238 | 24 | ts names apps/mobile/components/Formats/WebpageFormat.tsx |  |  | 0.549 |
| ns | 8246 |  | 281 | Modal, link-view, preservation and input-picker component subdirectories | 6.3 | 6.2 | 0.535 |
| walker |  | 8253 | 15 | README.md section #11 |  |  | 0.536 |
| walker |  | 8291 | 38 | ts names apps/mobile/lib/cache.ts |  |  | 0.536 |
| walker |  | 8356 | 65 | ts names packages/filesystem/manageFiles.ts |  |  | 0.536 |
| walker |  | 8373 | 17 | listing of 'apps/web/e2e/fixtures' |  |  | 0.536 |
| walker |  | 8386 | 13 | listing of 'apps/web/e2e/fixtures/base' |  |  | 0.536 |
| walker |  | 8411 | 25 | ts names apps/mobile/components/Formats/ImageFormat.tsx |  |  | 0.536 |
| walker |  | 8436 | 25 | ts names apps/web/pages/preserved/[id].tsx |  |  | 0.536 |
| walker |  | 8461 | 25 | ts names apps/worker/lib/preservationScheme/handleArchivePreview.ts |  |  | 0.536 |
| ns | 8473 |  | 227 | Web hooks, layouts, stores, ambient types, email templates, one-off migration scripts and the Playwright suite | 6.4 |  | 0.544 |
| walker |  | 8486 | 25 | ts names apps/worker/lib/preservationScheme/handleReadability.ts |  |  | 0.544 |
| walker |  | 8553 | 67 | ts names packages/router/user.tsx |  |  | 0.545 |
| walker |  | 8609 | 56 | listing of 'apps/web/pages/settings' |  |  | 0.559 |
| walker |  | 8623 | 14 | ts names apps/web/pages/settings/index.tsx |  |  | 0.559 |
| walker |  | 8648 | 25 | ts names apps/web/pages/settings/delete.tsx |  |  | 0.559 |
| walker |  | 8672 | 24 | ts decl apps/web/lib/shared/fetchTitleAndHeaders.ts:3 |  |  | 0.559 |
| walker |  | 8698 | 26 | ts names apps/web/pages/auth/reset-password.tsx |  |  | 0.559 |
| ns | 8713 |  | 240 | verifyUser — the guard chain every authenticated route runs first | 7.1 |  | 0.551 |
| walker |  | 8739 | 41 | ts names apps/web/pages/_app.tsx |  |  | 0.551 |
| walker |  | 8763 | 24 | ts decl apps/web/pages/_app.tsx:26 |  |  | 0.551 |
| walker |  | 8835 | 72 | ts names packages/router/highlights.tsx |  |  | 0.552 |
| walker |  | 8860 | 25 | ts decl packages/router/highlights.tsx:11 |  |  | 0.552 |
| walker |  | 8932 | 72 | ts names packages/router/rss.tsx |  |  | 0.554 |
| ns | 8948 |  | 235 | docker-compose.yml — the three-container deployment | 7.2 |  | 0.547 |
| walker |  | 8959 | 27 | ts names apps/worker/lib/preservationScheme/handleScreenshotAndPdf.ts |  |  | 0.547 |
| walker |  | 9002 | 43 | ts names apps/web/pages/search.tsx |  |  | 0.547 |
| walker |  | 9030 | 28 | ts names apps/mobile/components/ui/Input.tsx |  |  | 0.547 |
| walker |  | 9046 | 16 | ts decl apps/mobile/components/ui/Input.tsx:5 |  |  | 0.547 |
| walker |  | 9063 | 17 | README.md section #7 |  |  | 0.547 |
| walker |  | 9079 | 16 | README.md section #6 |  |  | 0.548 |
| walker |  | 9096 | 17 | README.md section #9 |  |  | 0.548 |
| walker |  | 9112 | 16 | README.md section #8 |  |  | 0.549 |
| walker |  | 9128 | 16 | README.md section #10 |  |  | 0.550 |
| walker |  | 9145 | 17 | README.md section #16 |  |  | 0.551 |
| ns | 9162 |  | 214 | .env.sample — the required variables and the complete list of section headings | 7.3 |  | 0.550 |
| walker |  | 9190 | 45 | ts names apps/web/pages/dashboard.tsx |  |  | 0.550 |
| walker |  | 9202 | 12 | ts body packages/lib/utils.ts:17 |  |  | 0.550 |
| walker |  | 9231 | 29 | ts names apps/mobile/components/ui/Spinner.tsx |  |  | 0.550 |
| walker |  | 9243 | 12 | ts decl apps/mobile/components/ui/Spinner.tsx:4 |  |  | 0.550 |
| walker |  | 9272 | 29 | ts names apps/mobile/components/ui/TabBarBackground.tsx |  |  | 0.550 |
| ns | 9341 |  | 179 | The head of the optional tuning-variable block | 7.4 | 7.3 | 0.544 |
| walker |  | 9346 | 74 | ts decl packages/filesystem/createFile.ts:6 |  |  | 0.544 |
| walker |  | 9360 | 14 | ts names apps/web/pages/api/v2/dashboard/index.ts |  |  | 0.544 |
| walker |  | 9407 | 47 | ts names apps/web/pages/login.tsx |  |  | 0.544 |
| walker |  | 9434 | 27 | ts decl apps/web/pages/login.tsx:24 |  |  | 0.544 |
| ns | 9473 |  | 132 | CI workflows, GitHub templates, the patch-package patch, and every translated locale | 7.5 |  | 0.550 |
| walker |  | 9481 | 47 | ts names apps/web/pages/register.tsx |  |  | 0.550 |
| walker |  | 9508 | 27 | ts decl apps/web/pages/register.tsx:29 |  |  | 0.550 |
| walker |  | 9538 | 30 | ts names apps/mobile/components/ui/Button.tsx |  |  | 0.550 |
| walker |  | 9568 | 30 | ts names apps/mobile/components/ui/IconSymbol.tsx |  |  | 0.550 |
| walker |  | 9597 | 29 | ts decl apps/worker/lib/preservationScheme/handleArchivePreview.ts:17 |  |  | 0.550 |
| ns | 9689 |  | 216 | apps/mobile root and the complete Expo Router screen tree | 8.1 |  | 0.566 |
| walker |  | 9874 | 277 | plaintext config Dockerfile |  |  | 0.566 |
| ns | 9891 |  | 202 | Mobile components, stores and query-cache modules | 8.2 | 8.1 | 0.577 |
| walker |  | 9905 | 31 | ts names apps/mobile/components/ui/TabBarBackground.ios.tsx |  |  | 0.577 |
| walker |  | 9936 | 31 | ts names apps/worker/lib/preservationScheme/pdfHandler.ts |  |  | 0.577 |
| walker |  | 9958 | 22 | listing of '.github/ISSUE_TEMPLATE' |  |  | 0.581 |

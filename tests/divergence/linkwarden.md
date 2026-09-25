Score(3000)=0.646 I=0.856 C=0.487 ns_rows≤3K=15/51 grid(1000/1442/2080/3000/4327/6240/9000)=0.622/0.547/0.630/0.646/0.528/0.542/0.551

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
| walker |  | 1144 | 15 | ts names apps/worker/workers/migrationWorker.ts |  |  | 0.584 |
| walker |  | 1216 | 72 | listing of 'packages/router' |  |  | 0.586 |
| walker |  | 1232 | 16 | ts names apps/worker/workers/rssPolling.ts |  |  | 0.586 |
| walker |  | 1260 | 28 | ts names packages/filesystem/s3Client.ts |  |  | 0.586 |
| ns | 1271 |  | 189 | Feature list, second half (sync, SSO, API keys, i18n, RSS, uploads) | 1.10 |  | 0.546 |
| walker |  | 1277 | 17 | ts names apps/worker/workers/trialEndEmailWorker.ts |  |  | 0.546 |
| walker |  | 1294 | 17 | ts names packages/prisma/client/index.d.ts |  |  | 0.546 |
| walker |  | 1374 | 80 | listing of 'apps/mobile' |  |  | 0.547 |
| walker |  | 1378 | 4 | listing of 'apps/mobile/styles' |  |  | 0.547 |
| walker |  | 1384 | 6 | listing of 'apps/mobile/assets' |  |  | 0.547 |
| walker |  | 1392 | 8 | listing of 'apps/mobile/plugins' |  |  | 0.547 |
| walker |  | 1398 | 6 | listing of 'apps/mobile/assets/fonts' |  |  | 0.547 |
| walker |  | 1409 | 11 | listing of 'apps/mobile/types' |  |  | 0.547 |
| walker |  | 1421 | 12 | listing of 'apps/mobile/store' |  |  | 0.547 |
| walker |  | 1444 | 23 | listing of 'apps/mobile/lib' |  |  | 0.547 |
| walker |  | 1480 | 36 | listing of 'apps/mobile/app' |  |  | 0.547 |
| walker |  | 1495 | 15 | ts names apps/mobile/app/index.tsx |  |  | 0.547 |
| walker |  | 1501 | 6 | listing of 'apps/mobile/app/links' |  |  | 0.547 |
| walker |  | 1522 | 21 | listing of 'apps/mobile/app/(tabs)' |  |  | 0.548 |
| walker |  | 1533 | 11 | listing of 'apps/mobile/app/(tabs)/links' |  |  | 0.548 |
| ns | 1539 |  | 268 | schema.prisma: datasource/generator plus every model and enum header, bodies elided | 2.1 |  | 0.531 |
| walker |  | 1548 | 15 | ts names apps/mobile/app/incoming.tsx |  |  | 0.531 |
| walker |  | 1563 | 15 | ts names apps/mobile/app/login.tsx |  |  | 0.531 |
| walker |  | 1579 | 16 | ts names apps/mobile/app/+not-found.tsx |  |  | 0.531 |
| walker |  | 1603 | 24 | listing of 'apps/mobile/assets/images' |  |  | 0.531 |
| walker |  | 1684 | 81 | listing of 'apps/web' |  |  | 0.650 |
| walker |  | 1688 | 4 | listing of 'apps/web/styles' |  |  | 0.650 |
| walker |  | 1697 | 9 | listing of 'apps/web/store' |  |  | 0.606 |
| ns | 1697 |  | 158 | All five schema enums, fully expanded | 2.2 | 2.1 | 0.606 |
| walker |  | 1709 | 12 | listing of 'apps/web/types' |  |  | 0.606 |
| walker |  | 1722 | 13 | listing of 'apps/web/lib' |  |  | 0.606 |
| walker |  | 1743 | 21 | listing of 'apps/web/templates' |  |  | 0.606 |
| walker |  | 1767 | 24 | listing of 'apps/web/layouts' |  |  | 0.606 |
| walker |  | 1781 | 14 | ts names apps/web/types/himalaya.d.ts |  |  | 0.606 |
| walker |  | 1801 | 20 | listing of 'apps/web/lib/shared' |  |  | 0.606 |
| walker |  | 1830 | 29 | listing of '.github' |  |  | 0.606 |
| walker |  | 1851 | 21 | listing of '.github/workflows' |  |  | 0.607 |
| walker |  | 1869 | 18 | ts names apps/mobile/app/_layout.tsx |  |  | 0.607 |
| walker |  | 1893 | 24 | Prisma decl at packages/prisma/schema.prisma:77 |  |  | 0.612 |
| walker |  | 1910 | 17 | listing of 'apps/mobile/app/(tabs)/collections' |  |  | 0.612 |
| walker |  | 1927 | 17 | listing of 'apps/mobile/app/(tabs)/dashboard' |  |  | 0.612 |
| walker |  | 1944 | 17 | listing of 'apps/mobile/app/(tabs)/settings' |  |  | 0.612 |
| walker |  | 1961 | 17 | listing of 'apps/mobile/app/(tabs)/tags' |  |  | 0.613 |
| walker |  | 2010 | 49 | Prisma decl at packages/prisma/schema.prisma:90 |  |  | 0.630 |
| walker |  | 2029 | 19 | ts names apps/mobile/lib/queryPersister.ts |  |  | 0.630 |
| walker |  | 2048 | 19 | ts names apps/web/layouts/AdminLayout.tsx |  |  | 0.630 |
| walker |  | 2067 | 19 | ts names apps/web/layouts/AuthRedirect.tsx |  |  | 0.630 |
| ns | 2083 |  | 386 | `Link` model — every field | 2.3 | 2.1 | 0.574 |
| walker |  | 2086 | 19 | ts names apps/web/layouts/MainLayout.tsx |  |  | 0.574 |
| walker |  | 2105 | 19 | ts names apps/web/layouts/SettingsLayout.tsx |  |  | 0.574 |
| walker |  | 2124 | 19 | ts names apps/worker/workers/linkProcessing.ts |  |  | 0.574 |
| walker |  | 2157 | 33 | ts names packages/router/publicLinks.tsx |  |  | 0.574 |
| walker |  | 2190 | 33 | ts names packages/router/publicTags.tsx |  |  | 0.574 |
| walker |  | 2210 | 20 | ts names apps/worker/workers/linkIndexing.ts |  |  | 0.574 |
| walker |  | 2236 | 26 | Prisma decl at packages/prisma/schema.prisma:304 |  |  | 0.588 |
| walker |  | 2271 | 35 | Prisma decl at packages/prisma/schema.prisma:83 |  |  | 0.612 |
| ns | 2396 |  | 313 | `User` model — identity, relations and subscription linkage | 2.4 | 2.1 | 0.574 |
| walker |  | 2481 | 210 | Prisma decl at packages/prisma/schema.prisma:166 |  |  | 0.607 |
| walker |  | 2660 | 179 | Prisma decl tail at packages/prisma/schema.prisma:166 body 182 |  |  | 0.678 |
| ns | 2738 |  | 342 | `User` model — preference and archival-toggle fields | 2.5 | 2.4 | 0.643 |
| walker |  | 2769 | 109 | listing of 'packages/lib' |  |  | 0.646 |
| walker |  | 2782 | 13 | ts names packages/lib/transporter.ts |  |  | 0.646 |
| walker |  | 2796 | 14 | ts names packages/lib/constants.ts |  |  | 0.646 |
| walker |  | 2810 | 14 | ts names packages/lib/safeFetch.ts |  |  | 0.646 |
| walker |  | 2825 | 15 | ts names packages/lib/generatePreview.ts |  |  | 0.646 |
| walker |  | 2840 | 15 | ts names packages/lib/rssHandler.ts |  |  | 0.646 |
| walker |  | 2856 | 16 | ts names packages/lib/verifyCapacity.ts |  |  | 0.646 |
| walker |  | 2873 | 17 | ts names packages/lib/meilisearchClient.ts |  |  | 0.646 |
| walker |  | 2891 | 18 | ts names packages/lib/getPreservedFormatUrl.ts |  |  | 0.646 |
| walker |  | 2914 | 23 | ts names packages/lib/isArchivalTag.ts |  |  | 0.646 |
| walker |  | 2938 | 24 | ts names packages/lib/getOriginalFormat.ts |  |  | 0.646 |
| walker |  | 2964 | 26 | ts decl packages/lib/getOriginalFormat.ts:6 |  |  | 0.646 |
| walker |  | 2992 | 28 | ts names packages/lib/formatStats.ts |  |  | 0.646 |
| walker |  | 3016 | 24 | ts decl packages/lib/formatStats.ts:11 |  |  | 0.646 |
| walker |  | 3044 | 28 | ts names packages/lib/getFormatBasedOnPreference.ts |  |  | 0.646 |
| ns | 3055 |  | 317 | `Collection` model — every field, including the self-relation | 2.6 | 2.1 | 0.616 |
| walker |  | 3072 | 28 | ts names packages/lib/getLinkTypeFromFormat.ts |  |  | 0.616 |
| walker |  | 3100 | 28 | ts decl packages/lib/verifyCapacity.ts:8 |  |  | 0.616 |
| walker |  | 3130 | 30 | ts decl packages/lib/rssHandler.ts:7 |  |  | 0.616 |
| walker |  | 3160 | 30 | ts decl packages/lib/safeFetch.ts:95 |  |  | 0.616 |
| walker |  | 3194 | 34 | ts decl packages/lib/getLinkTypeFromFormat.ts:3 |  |  | 0.616 |
| walker |  | 3231 | 37 | ts names packages/lib/getFormatFromContentType.ts |  |  | 0.616 |
| walker |  | 3288 | 57 | listing of 'apps/worker/lib' |  |  | 0.617 |
| walker |  | 3303 | 15 | ts names apps/worker/lib/archiveHandler.ts |  |  | 0.617 |
| walker |  | 3321 | 18 | ts names apps/worker/lib/getLinkBatchFairly.ts |  |  | 0.617 |
| walker |  | 3340 | 19 | ts names apps/worker/lib/countUnprocessedBillableLinks.ts |  |  | 0.617 |
| walker |  | 3359 | 19 | ts names apps/worker/lib/fetchHeaders.ts |  |  | 0.617 |
| walker |  | 3380 | 21 | ts names apps/worker/lib/protectPageRequests.ts |  |  | 0.617 |
| walker |  | 3402 | 22 | ts names apps/worker/workers/autoTagPreservedLinks.ts |  |  | 0.617 |
| ns | 3436 |  | 381 | `UsersAndCollections` join model (the permission bits) and `Tag` model | 2.7 | 2.1 | 0.582 |
| walker |  | 3441 | 39 | Prisma decl at packages/prisma/schema.prisma:289 |  |  | 0.605 |
| walker |  | 3461 | 20 | plaintext config apps/mobile/.env.sample |  |  | 0.605 |
| walker |  | 3476 | 15 | ts names apps/mobile/app/(tabs)/_layout.tsx |  |  | 0.605 |
| walker |  | 3491 | 15 | ts names apps/mobile/app/links/[id].tsx |  |  | 0.605 |
| walker |  | 3530 | 39 | ts decl packages/lib/generatePreview.ts:5 |  |  | 0.605 |
| ns | 3594 |  | 158 | `AccessToken` model | 2.8 | 2.1 | 0.593 |
| ns | 3617 |  | 23 | Complete packages/prisma listing | 2.9 |  | 0.600 |
| walker |  | 3665 | 135 | package runtime dependencies in package.json |  |  | 0.600 |
| walker |  | 3724 | 59 | listing of 'apps/mobile/components' |  |  | 0.600 |
| ns | 3733 |  | 116 | The process-wide `prisma` client singleton | 2.10 |  | 0.601 |
| walker |  | 3738 | 14 | ts names apps/mobile/components/DashboardItem.tsx |  |  | 0.601 |
| walker |  | 3753 | 15 | ts names apps/mobile/components/ElementNotSupported.tsx |  |  | 0.601 |
| walker |  | 3779 | 26 | listing of 'apps/mobile/components/Formats' |  |  | 0.602 |
| walker |  | 3799 | 20 | ts names apps/mobile/components/Links.tsx |  |  | 0.602 |
| ns | 3817 |  | 84 | API version directories: every resource under pages/api/v1 (and the single v2 route) | 3.1 |  | 0.576 |
| walker |  | 3831 | 32 | listing of 'apps/mobile/components/ActionSheets' |  |  | 0.576 |
| walker |  | 3839 | 8 | ts names apps/mobile/components/ActionSheets/Sheets.tsx |  |  | 0.576 |
| walker |  | 3862 | 23 | ts names apps/mobile/components/HapticTab.tsx |  |  | 0.576 |
| walker |  | 3877 | 15 | ts names apps/mobile/components/ActionSheets/SupportSheet.tsx |  |  | 0.576 |
| ns | 3885 |  | 68 | Route files for links, collections, tags and highlights | 3.2 |  | 0.562 |
| walker |  | 3919 | 42 | ts names packages/types/inputSelect.ts |  |  | 0.562 |
| walker |  | 3950 | 31 | ts decl packages/types/inputSelect.ts:16 |  |  | 0.562 |
| ns | 3962 |  | 77 | Route files for auth, session, users, tokens, config, avatar and logins | 3.3 |  | 0.549 |
| walker |  | 3985 | 35 | ts decl packages/types/inputSelect.ts:1 |  |  | 0.549 |
| walker |  | 4010 | 25 | ts names apps/mobile/lib/queryClient.ts |  |  | 0.549 |
| walker |  | 4035 | 25 | ts names apps/worker/lib/autoTagLink.ts |  |  | 0.549 |
| ns | 4042 |  | 80 | Route files for archives, preserved, search, dashboard, rss, migration, payment, webhook, worker, getFavicon | 3.4 |  | 0.535 |
| walker |  | 4051 | 16 | ts names apps/mobile/components/ActionSheets/AddLinkSheet.tsx |  |  | 0.535 |
| walker |  | 4067 | 16 | ts names apps/mobile/components/ActionSheets/EditLinkSheet.tsx |  |  | 0.535 |
| walker |  | 4083 | 16 | ts names apps/mobile/components/ActionSheets/NewCollectionSheet.tsx |  |  | 0.528 |
| ns | 4083 |  | 41 | The unauthenticated `public/` route subtree | 3.5 |  | 0.528 |
| walker |  | 4124 | 41 | ts decl packages/lib/transporter.ts:3 |  |  | 0.528 |
| walker |  | 4186 | 62 | listing of 'apps/web/hooks' |  |  | 0.528 |
| walker |  | 4200 | 14 | ts names apps/web/hooks/useSort.tsx |  |  | 0.528 |
| walker |  | 4216 | 16 | ts names apps/web/hooks/useInitialData.tsx |  |  | 0.528 |
| walker |  | 4232 | 16 | ts names apps/web/hooks/useWindowDimensions.tsx |  |  | 0.528 |
| walker |  | 4251 | 19 | ts names apps/web/hooks/usePermissions.tsx |  |  | 0.528 |
| walker |  | 4271 | 20 | ts names apps/web/hooks/useMediaQuery.tsx |  |  | 0.528 |
| walker |  | 4292 | 21 | ts names apps/web/hooks/useCollectivePermissions.ts |  |  | 0.528 |
| walker |  | 4316 | 24 | ts names apps/web/hooks/useOnScreen.tsx |  |  | 0.528 |
| walker |  | 4334 | 18 | listing of 'apps/web/e2e' |  |  | 0.529 |
| walker |  | 4340 | 6 | listing of 'apps/web/e2e/tests' |  |  | 0.529 |
| walker |  | 4357 | 17 | ts names apps/web/lib/shared/fetchTitleAndHeaders.ts |  |  | 0.529 |
| ns | 4359 |  | 276 | The route-handler pattern, read from pages/api/v1/links/index.ts | 3.6 |  | 0.515 |
| walker |  | 4374 | 17 | ts names apps/web/lib/shared/isValidUrl.ts |  |  | 0.515 |
| walker |  | 4420 | 46 | ts names packages/router/worker.tsx |  |  | 0.515 |
| walker |  | 4446 | 26 | ts decl apps/worker/lib/getLinkBatchFairly.ts:12 |  |  | 0.515 |
| walker |  | 4493 | 47 | ts names packages/lib/utils.ts |  |  | 0.515 |
| ns | 4568 |  | 209 | apps/web/lib: the server helper layer and its client/shared siblings | 3.7 |  | 0.493 |
| ns | 4667 |  | 99 | The controller tree: every resource directory under lib/api/controllers | 3.8 |  | 0.480 |
| walker |  | 4778 | 285 | Prisma decl at packages/prisma/schema.prisma:28 |  |  | 0.518 |
| walker |  | 4806 | 28 | ts names apps/mobile/components/CollectionListing.tsx |  |  | 0.518 |
| ns | 4823 |  | 156 | Controller files for links, collections, tags and highlights | 3.9 |  | 0.503 |
| walker |  | 4834 | 28 | ts names apps/mobile/components/TagListing.tsx |  |  | 0.503 |
| walker |  | 4862 | 28 | ts names apps/web/hooks/useDetectPageBottom.tsx |  |  | 0.503 |
| walker |  | 4999 | 137 | headings outline in README.md |  |  | 0.503 |
| ns | 5001 |  | 178 | Controller files for users, tokens, session, search, dashboard, worker, migration and public access | 3.10 |  | 0.486 |
| walker |  | 5025 | 26 | README.md section #26 |  |  | 0.486 |
| walker |  | 5036 | 11 | README.md section #18 |  |  | 0.487 |
| walker |  | 5046 | 10 | README.md section #17 |  |  | 0.487 |
| walker |  | 5056 | 10 | README.md section #19 |  |  | 0.487 |
| walker |  | 5098 | 42 | listing of 'apps/worker/lib/preservationScheme' |  |  | 0.488 |
| walker |  | 5114 | 16 | ts names apps/worker/lib/preservationScheme/handleMonolith.ts |  |  | 0.488 |
| walker |  | 5165 | 51 | ts names packages/router/dashboardData.tsx |  |  | 0.488 |
| walker |  | 5195 | 30 | ts names apps/mobile/components/LinkListing.tsx |  |  | 0.488 |
| walker |  | 5225 | 30 | ts names apps/mobile/lib/theme.ts |  |  | 0.488 |
| ns | 5249 |  | 248 | Complete listings of packages/types, packages/lib, packages/filesystem and packages/router | 4.1 |  | 0.527 |
| walker |  | 5255 | 30 | ts names apps/web/types/next-auth.d.ts |  |  | 0.527 |
| walker |  | 5284 | 29 | ts decl apps/worker/lib/archiveHandler.ts:25 |  |  | 0.527 |
| walker |  | 5315 | 31 | ts names apps/mobile/store/auth.ts |  |  | 0.527 |
| walker |  | 5342 | 27 | Prisma decl at packages/prisma/schema.prisma:5 |  |  | 0.537 |
| walker |  | 5393 | 51 | ts decl packages/lib/formatStats.ts:4 |  |  | 0.537 |
| walker |  | 5406 | 13 | README.md section #23 |  |  | 0.537 |
| walker |  | 5419 | 13 | README.md section #24 |  |  | 0.538 |
| walker |  | 5515 | 96 | package runtime metadata in package.json |  |  | 0.538 |
| walker |  | 5547 | 32 | ts names apps/mobile/components/DashboardSection.tsx |  |  | 0.538 |
| walker |  | 5561 | 14 | ts names apps/mobile/app/(tabs)/collections/_layout.tsx |  |  | 0.538 |
| walker |  | 5575 | 14 | ts names apps/mobile/app/(tabs)/dashboard/_layout.tsx |  |  | 0.538 |
| walker |  | 5589 | 14 | ts names apps/mobile/app/(tabs)/links/_layout.tsx |  |  | 0.538 |
| walker |  | 5603 | 14 | ts names apps/mobile/app/(tabs)/settings/_layout.tsx |  |  | 0.538 |
| walker |  | 5617 | 14 | ts names apps/mobile/app/(tabs)/tags/_layout.tsx |  |  | 0.538 |
| ns | 5647 |  | 398 | packages/types/global.ts — every exported type, interface and enum declaration | 4.2 |  | 0.524 |
| walker |  | 5696 | 79 | listing of 'apps/web/public' |  |  | 0.524 |
| walker |  | 5706 | 10 | listing of 'apps/web/public/screenshots' |  |  | 0.524 |
| walker |  | 5759 | 53 | ts decl packages/lib/getFormatBasedOnPreference.ts:15 |  |  | 0.524 |
| walker |  | 5780 | 21 | ts names apps/worker/lib/preservationScheme/sendToWayback.ts |  |  | 0.524 |
| ns | 5793 |  | 146 | `ArchivedFormat`, `LinkType` and `TokenExpiry` variants | 4.3 | 4.2 | 0.517 |
| walker |  | 5813 | 33 | ts names apps/mobile/store/data.ts |  |  | 0.517 |
| walker |  | 5846 | 33 | ts names apps/web/hooks/useArchivalTags.ts |  |  | 0.517 |
| walker |  | 5879 | 33 | ts names apps/worker/lib/getLinkBatch.ts |  |  | 0.517 |
| walker |  | 5905 | 26 | ts decl apps/worker/lib/getLinkBatch.ts:11 |  |  | 0.517 |
| ns | 5945 |  | 152 | `ViewMode`, `Sort` and `TagSort` variants | 4.4 | 4.2 | 0.512 |
| walker |  | 6225 | 320 | Prisma decl at packages/prisma/schema.prisma:126 |  |  | 0.542 |
| walker |  | 6280 | 55 | ts decl packages/lib/meilisearchClient.ts:5 |  |  | 0.542 |
| ns | 6285 |  | 340 | packages/lib/schemaValidation.ts — every exported zod schema constant | 4.5 |  | 0.532 |
| walker |  | 6314 | 34 | ts names apps/mobile/store/tmp.ts |  |  | 0.532 |
| walker |  | 6348 | 34 | ts names apps/web/store/links.ts |  |  | 0.532 |
| walker |  | 6482 | 134 | plaintext config .env.sample |  |  | 0.532 |
| walker |  | 6531 | 49 | listing of 'apps/web/public/locales' |  |  | 0.533 |
| walker |  | 6535 | 4 | listing of 'apps/web/public/locales/de' |  |  | 0.533 |
| walker |  | 6539 | 4 | listing of 'apps/web/public/locales/en' |  |  | 0.533 |
| ns | 6542 |  | 257 | packages/router — the links, collections and tags hook exports | 4.6 |  | 0.521 |
| walker |  | 6543 | 4 | listing of 'apps/web/public/locales/es' |  |  | 0.521 |
| walker |  | 6547 | 4 | listing of 'apps/web/public/locales/fr' |  |  | 0.521 |
| walker |  | 6551 | 4 | listing of 'apps/web/public/locales/it' |  |  | 0.521 |
| walker |  | 6555 | 4 | listing of 'apps/web/public/locales/ja' |  |  | 0.521 |
| walker |  | 6559 | 4 | listing of 'apps/web/public/locales/nl' |  |  | 0.521 |
| walker |  | 6563 | 4 | listing of 'apps/web/public/locales/pl' |  |  | 0.521 |
| walker |  | 6567 | 4 | listing of 'apps/web/public/locales/pt-BR' |  |  | 0.521 |
| walker |  | 6571 | 4 | listing of 'apps/web/public/locales/ro' |  |  | 0.521 |
| walker |  | 6575 | 4 | listing of 'apps/web/public/locales/ru' |  |  | 0.521 |
| walker |  | 6579 | 4 | listing of 'apps/web/public/locales/tr' |  |  | 0.521 |
| walker |  | 6583 | 4 | listing of 'apps/web/public/locales/uk' |  |  | 0.521 |
| walker |  | 6587 | 4 | listing of 'apps/web/public/locales/zh' |  |  | 0.521 |
| walker |  | 6591 | 4 | listing of 'apps/web/public/locales/zh-TW' |  |  | 0.521 |
| walker |  | 6605 | 14 | README.md section #21 |  |  | 0.522 |
| walker |  | 6617 | 12 | README.md section #22 |  |  | 0.523 |
| walker |  | 6631 | 14 | README.md section #20 |  |  | 0.525 |
| walker |  | 6646 | 15 | ts names apps/mobile/app/(tabs)/collections/[id].tsx |  |  | 0.525 |
| walker |  | 6661 | 15 | ts names apps/mobile/app/(tabs)/collections/index.tsx |  |  | 0.525 |
| walker |  | 6676 | 15 | ts names apps/mobile/app/(tabs)/dashboard/[section].tsx |  |  | 0.525 |
| walker |  | 6691 | 15 | ts names apps/mobile/app/(tabs)/dashboard/index.tsx |  |  | 0.525 |
| walker |  | 6706 | 15 | ts names apps/mobile/app/(tabs)/links/index.tsx |  |  | 0.525 |
| walker |  | 6721 | 15 | ts names apps/mobile/app/(tabs)/settings/index.tsx |  |  | 0.525 |
| ns | 6726 |  | 184 | packages/router — export lines of the remaining ten hook modules | 4.7 | 4.6 | 0.522 |
| walker |  | 6736 | 15 | ts names apps/mobile/app/(tabs)/tags/[id].tsx |  |  | 0.522 |
| walker |  | 6751 | 15 | ts names apps/mobile/app/(tabs)/tags/index.tsx |  |  | 0.522 |
| walker |  | 6786 | 35 | ts names apps/mobile/lib/colors.ts |  |  | 0.522 |
| walker |  | 6821 | 35 | ts names apps/web/store/localSettings.ts |  |  | 0.522 |
| walker |  | 6850 | 29 | README.md section #29 |  |  | 0.522 |
| ns | 6892 |  | 166 | Complete apps/worker listing, including every job and every preservation handler | 5.1 |  | 0.540 |
| walker |  | 6910 | 60 | ts names packages/router/users.tsx |  |  | 0.541 |
| walker |  | 6960 | 50 | listing of 'apps/mobile/components/ui' |  |  | 0.541 |
| walker |  | 6973 | 13 | ts names apps/mobile/components/ui/IconSymbol.ios.tsx |  |  | 0.541 |
| walker |  | 6993 | 20 | ts names apps/mobile/components/ui/Icons.tsx |  |  | 0.541 |
| walker |  | 7079 | 86 | listing of 'apps/web/pages' |  |  | 0.542 |
| walker |  | 7093 | 14 | ts names apps/web/pages/index.tsx |  |  | 0.542 |
| walker |  | 7099 | 6 | listing of 'apps/web/pages/preserved' |  |  | 0.542 |
| walker |  | 7107 | 8 | listing of 'apps/web/pages/api' |  |  | 0.542 |
| walker |  | 7110 | 3 | listing of 'apps/web/pages/api/v2' |  |  | 0.542 |
| walker |  | 7119 | 9 | listing of 'apps/web/pages/public' |  |  | 0.543 |
| walker |  | 7124 | 5 | listing of 'apps/web/pages/public/collections' |  |  | 0.543 |
| walker |  | 7128 | 4 | listing of 'apps/web/pages/api/v2/dashboard' |  |  | 0.543 |
| walker |  | 7134 | 6 | listing of 'apps/web/pages/public/links' |  |  | 0.543 |
| walker |  | 7140 | 6 | listing of 'apps/web/pages/public/preserved' |  |  | 0.543 |
| walker |  | 7151 | 11 | listing of 'apps/web/pages/collections' |  |  | 0.543 |
| ns | 7161 |  | 269 | worker.ts — the whole scheduler entry point | 5.2 |  | 0.534 |
| walker |  | 7162 | 11 | listing of 'apps/web/pages/tags' |  |  | 0.534 |
| walker |  | 7174 | 12 | listing of 'apps/web/pages/auth' |  |  | 0.535 |
| walker |  | 7190 | 16 | listing of 'apps/web/pages/links' |  |  | 0.535 |
| walker |  | 7209 | 19 | listing of 'apps/web/pages/admin' |  |  | 0.535 |
| walker |  | 7223 | 14 | ts names apps/web/pages/_document.tsx |  |  | 0.535 |
| walker |  | 7233 | 10 | listing of 'apps/web/pages/public/collections/[id]' |  |  | 0.535 |
| walker |  | 7276 | 43 | ts body apps/web/pages/index.tsx:4 |  |  | 0.535 |
| walker |  | 7301 | 25 | ts names apps/web/pages/forgot.tsx |  |  | 0.535 |
| walker |  | 7326 | 25 | ts names apps/web/pages/subscribe.tsx |  |  | 0.535 |
| walker |  | 7353 | 27 | ts names apps/web/pages/member-onboarding.tsx |  |  | 0.535 |
| ns | 7354 |  | 193 | archiveHandler's signature and its SSRF / skip-preservation guard | 5.3 |  | 0.529 |
| walker |  | 7381 | 28 | ts names apps/web/pages/confirmation.tsx |  |  | 0.529 |
| walker |  | 7404 | 23 | ts names apps/mobile/components/Formats/PdfFormat.tsx |  |  | 0.529 |
| walker |  | 7427 | 23 | ts names apps/web/lib/shared/getSuffixFromFormat.ts |  |  | 0.529 |
| ns | 7607 |  | 253 | Every UI page route under apps/web/pages | 6.1 |  | 0.539 |
| walker |  | 7800 | 373 | Prisma decl tail at packages/prisma/schema.prisma:28 body 51 |  |  | 0.566 |
| walker |  | 7862 | 62 | ts names packages/router/tokens.tsx |  |  | 0.567 |
| walker |  | 7878 | 16 | Prisma decl at packages/prisma/schema.prisma:1 |  |  | 0.572 |
| walker |  | 7902 | 24 | ts names apps/mobile/components/Formats/ReadableFormat.tsx |  |  | 0.572 |
| walker |  | 7926 | 24 | ts names apps/mobile/components/Formats/WebpageFormat.tsx |  |  | 0.572 |
| walker |  | 7941 | 15 | README.md section #11 |  |  | 0.572 |
| ns | 7965 |  | 358 | The flat components/ directory and the ui/ primitives | 6.2 |  | 0.550 |
| walker |  | 7979 | 38 | ts names apps/mobile/lib/cache.ts |  |  | 0.550 |
| walker |  | 8044 | 65 | ts names packages/filesystem/manageFiles.ts |  |  | 0.550 |
| walker |  | 8061 | 17 | listing of 'apps/web/e2e/fixtures' |  |  | 0.550 |
| walker |  | 8074 | 13 | listing of 'apps/web/e2e/fixtures/base' |  |  | 0.550 |
| walker |  | 8099 | 25 | ts names apps/mobile/components/Formats/ImageFormat.tsx |  |  | 0.550 |
| walker |  | 8124 | 25 | ts names apps/web/pages/preserved/[id].tsx |  |  | 0.550 |
| walker |  | 8149 | 25 | ts names apps/worker/lib/preservationScheme/handleArchivePreview.ts |  |  | 0.550 |
| walker |  | 8174 | 25 | ts names apps/worker/lib/preservationScheme/handleReadability.ts |  |  | 0.550 |
| walker |  | 8241 | 67 | ts names packages/router/user.tsx |  |  | 0.551 |
| ns | 8246 |  | 281 | Modal, link-view, preservation and input-picker component subdirectories | 6.3 | 6.2 | 0.537 |
| walker |  | 8297 | 56 | listing of 'apps/web/pages/settings' |  |  | 0.552 |
| walker |  | 8311 | 14 | ts names apps/web/pages/settings/index.tsx |  |  | 0.552 |
| walker |  | 8336 | 25 | ts names apps/web/pages/settings/delete.tsx |  |  | 0.552 |
| walker |  | 8360 | 24 | ts decl apps/web/lib/shared/fetchTitleAndHeaders.ts:3 |  |  | 0.552 |
| walker |  | 8386 | 26 | ts names apps/web/pages/auth/reset-password.tsx |  |  | 0.552 |
| walker |  | 8427 | 41 | ts names apps/web/pages/_app.tsx |  |  | 0.552 |
| walker |  | 8451 | 24 | ts decl apps/web/pages/_app.tsx:26 |  |  | 0.552 |
| ns | 8473 |  | 227 | Web hooks, layouts, stores, ambient types, email templates, one-off migration scripts and the Playwright suite | 6.4 |  | 0.559 |
| walker |  | 8523 | 72 | ts names packages/router/highlights.tsx |  |  | 0.561 |
| walker |  | 8548 | 25 | ts decl packages/router/highlights.tsx:11 |  |  | 0.561 |
| walker |  | 8620 | 72 | ts names packages/router/rss.tsx |  |  | 0.562 |
| walker |  | 8647 | 27 | ts names apps/worker/lib/preservationScheme/handleScreenshotAndPdf.ts |  |  | 0.562 |
| walker |  | 8675 | 28 | ts names apps/mobile/components/ui/Input.tsx |  |  | 0.562 |
| walker |  | 8691 | 16 | ts decl apps/mobile/components/ui/Input.tsx:5 |  |  | 0.562 |
| walker |  | 8708 | 17 | README.md section #7 |  |  | 0.562 |
| ns | 8713 |  | 240 | verifyUser — the guard chain every authenticated route runs first | 7.1 |  | 0.554 |
| walker |  | 8724 | 16 | README.md section #6 |  |  | 0.554 |
| walker |  | 8741 | 17 | README.md section #9 |  |  | 0.555 |
| walker |  | 8757 | 16 | README.md section #8 |  |  | 0.556 |
| walker |  | 8773 | 16 | README.md section #10 |  |  | 0.557 |
| walker |  | 8790 | 17 | README.md section #16 |  |  | 0.558 |
| walker |  | 8802 | 12 | ts body packages/lib/utils.ts:17 |  |  | 0.558 |
| walker |  | 8831 | 29 | ts names apps/mobile/components/ui/Spinner.tsx |  |  | 0.558 |
| walker |  | 8843 | 12 | ts decl apps/mobile/components/ui/Spinner.tsx:4 |  |  | 0.558 |
| walker |  | 8872 | 29 | ts names apps/mobile/components/ui/TabBarBackground.tsx |  |  | 0.558 |
| walker |  | 8946 | 74 | ts decl packages/filesystem/createFile.ts:6 |  |  | 0.558 |
| ns | 8948 |  | 235 | docker-compose.yml — the three-container deployment | 7.2 |  | 0.551 |
| walker |  | 8960 | 14 | ts names apps/web/pages/api/v2/dashboard/index.ts |  |  | 0.551 |
| walker |  | 9007 | 47 | ts names apps/web/pages/login.tsx |  |  | 0.551 |
| walker |  | 9034 | 27 | ts decl apps/web/pages/login.tsx:24 |  |  | 0.551 |
| walker |  | 9081 | 47 | ts names apps/web/pages/register.tsx |  |  | 0.551 |
| walker |  | 9108 | 27 | ts decl apps/web/pages/register.tsx:29 |  |  | 0.551 |
| walker |  | 9138 | 30 | ts names apps/mobile/components/ui/Button.tsx |  |  | 0.551 |
| ns | 9162 |  | 214 | .env.sample — the required variables and the complete list of section headings | 7.3 |  | 0.550 |
| walker |  | 9168 | 30 | ts names apps/mobile/components/ui/IconSymbol.tsx |  |  | 0.550 |
| walker |  | 9197 | 29 | ts decl apps/worker/lib/preservationScheme/handleArchivePreview.ts:17 |  |  | 0.550 |
| ns | 9341 |  | 179 | The head of the optional tuning-variable block | 7.4 | 7.3 | 0.544 |
| ns | 9473 |  | 132 | CI workflows, GitHub templates, the patch-package patch, and every translated locale | 7.5 |  | 0.550 |
| walker |  | 9474 | 277 | plaintext config Dockerfile |  |  | 0.550 |
| walker |  | 9505 | 31 | ts names apps/mobile/components/ui/TabBarBackground.ios.tsx |  |  | 0.550 |
| walker |  | 9536 | 31 | ts names apps/worker/lib/preservationScheme/pdfHandler.ts |  |  | 0.550 |
| walker |  | 9558 | 22 | listing of '.github/ISSUE_TEMPLATE' |  |  | 0.555 |
| walker |  | 9609 | 51 | ts names apps/mobile/types/global.ts |  |  | 0.555 |
| walker |  | 9660 | 51 | ts names apps/worker/lib/browser.ts |  |  | 0.555 |
| ns | 9689 |  | 216 | apps/mobile root and the complete Expo Router screen tree | 8.1 |  | 0.571 |
| walker |  | 9743 | 83 | ts decl packages/types/inputSelect.ts:7 |  |  | 0.571 |
| walker |  | 9762 | 19 | README.md section #15 |  |  | 0.572 |
| walker |  | 9852 | 90 | ts names packages/router/config.tsx |  |  | 0.574 |
| walker |  | 9882 | 30 | ts decl packages/router/config.tsx:44 |  |  | 0.574 |
| ns | 9891 |  | 202 | Mobile components, stores and query-cache modules | 8.2 | 8.1 | 0.584 |
| walker |  | 9971 | 89 | ts decl packages/lib/isArchivalTag.ts:3 |  |  | 0.584 |

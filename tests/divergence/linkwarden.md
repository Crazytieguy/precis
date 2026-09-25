Score(3000)=0.646 I=0.856 C=0.487 ns_rows≤3K=15/51 grid(1000/1442/2080/3000/4327/6240/9000)=0.622/0.547/0.673/0.646/0.572/0.544/0.656

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 75 |  | 75 | Product identity: name, tagline, one-sentence definition | 1.1 |  | 0.000 |
| walker |  | 79 | 79 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 88 | 9 | Fs::DirListing { dir: apps } |  |  | 0.000 |
| walker |  | 99 | 11 | Fs::DirListing { dir: patches } |  |  | 0.000 |
| walker |  | 114 | 15 | Fs::DirListing { dir: packages } |  |  | 0.000 |
| walker |  | 118 | 4 | Fs::DirListing { dir: .vscode } |  |  | 0.000 |
| walker |  | 131 | 13 | Fs::DirListing { dir: packages/types } |  |  | 0.000 |
| ns | 154 |  | 79 | Complete repository root listing | 1.2 |  | 0.721 |
| walker |  | 174 | 43 | Json::Identity { file: package.json } |  |  | 0.724 |
| ns | 178 |  | 24 | The three apps and five shared packages | 1.3 |  | 0.711 |
| walker |  | 207 | 33 | Fs::DirListing { dir: assets } |  |  | 0.711 |
| walker |  | 212 | 5 | Fs::DirListing { dir: .devcontainer } |  |  | 0.711 |
| walker |  | 235 | 23 | Fs::DirListing { dir: packages/prisma } |  |  | 0.714 |
| ns | 259 |  | 81 | Complete apps/web listing | 1.4 |  | 0.533 |
| walker |  | 272 | 37 | Code::CodeKey { rung: Names, file: packages/prisma/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.534 |
| walker |  | 281 | 9 | Fs::DirListing { dir: packages/prisma/client } |  |  | 0.534 |
| ns | 321 |  | 62 | Workspace globs in the root package.json | 1.5 |  | 0.507 |
| walker |  | 357 | 76 | Code::CodeKey { rung: Decl, file: packages/prisma/index.ts, decl: 2, sub: 0, line: 5 } |  |  | 0.509 |
| walker |  | 383 | 26 | Fs::DirListing { dir: apps/worker } |  |  | 0.509 |
| walker |  | 395 | 12 | Code::CodeKey { rung: Names, file: apps/worker/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.509 |
| walker |  | 400 | 5 | Fs::DirListing { dir: apps/worker/templates } |  |  | 0.510 |
| walker |  | 434 | 34 | Json::Entry { file: package.json } |  |  | 0.561 |
| ns | 460 |  | 139 | Workspace package names — every `name` field under apps/ and packages/ | 1.6 |  | 0.519 |
| walker |  | 488 | 54 | Fs::DirListing { dir: packages/filesystem } |  |  | 0.520 |
| walker |  | 588 | 100 | Code::CodeKey { rung: Names, file: packages/filesystem/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.520 |
| walker |  | 602 | 14 | Code::CodeKey { rung: Names, file: packages/filesystem/createFile.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.520 |
| walker |  | 621 | 19 | Code::CodeKey { rung: Names, file: packages/filesystem/readFile.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.520 |
| ns | 674 |  | 214 | Root scripts: how you run web, worker and both together | 1.7 |  | 0.482 |
| walker |  | 758 | 137 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.665 |
| walker |  | 780 | 22 | Code::CodeKey { rung: Names, file: packages/filesystem/fileExists.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.665 |
| walker |  | 802 | 22 | Code::CodeKey { rung: Names, file: packages/filesystem/moveFile.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.665 |
| walker |  | 825 | 23 | Code::CodeKey { rung: Names, file: packages/filesystem/createFolder.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.665 |
| ns | 837 |  | 163 | Root scripts: prisma, format, test, coverage, postinstall | 1.8 |  | 0.622 |
| walker |  | 849 | 24 | Code::CodeKey { rung: Names, file: packages/filesystem/removeFile.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.622 |
| walker |  | 873 | 24 | Code::CodeKey { rung: Names, file: packages/filesystem/removeFolder.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.622 |
| ns | 1082 |  | 245 | Feature list, first half (preservation, reading, organisation, sharing) | 1.9 |  | 0.575 |
| walker |  | 1093 | 220 | Prisma::Toc { file: packages/prisma/schema.prisma } |  |  | 0.584 |
| walker |  | 1129 | 36 | Fs::DirListing { dir: apps/worker/workers } |  |  | 0.584 |
| walker |  | 1201 | 72 | Fs::DirListing { dir: packages/router } |  |  | 0.586 |
| walker |  | 1229 | 28 | Code::CodeKey { rung: Names, file: packages/filesystem/s3Client.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.586 |
| walker |  | 1246 | 17 | Code::CodeKey { rung: Names, file: packages/prisma/client/index.d.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.586 |
| ns | 1271 |  | 189 | Feature list, second half (sync, SSO, API keys, i18n, RSS, uploads) | 1.10 |  | 0.546 |
| walker |  | 1326 | 80 | Fs::DirListing { dir: apps/mobile } |  |  | 0.547 |
| walker |  | 1330 | 4 | Fs::DirListing { dir: apps/mobile/styles } |  |  | 0.547 |
| walker |  | 1336 | 6 | Fs::DirListing { dir: apps/mobile/assets } |  |  | 0.547 |
| walker |  | 1344 | 8 | Fs::DirListing { dir: apps/mobile/plugins } |  |  | 0.547 |
| walker |  | 1350 | 6 | Fs::DirListing { dir: apps/mobile/assets/fonts } |  |  | 0.547 |
| walker |  | 1361 | 11 | Fs::DirListing { dir: apps/mobile/types } |  |  | 0.547 |
| walker |  | 1373 | 12 | Fs::DirListing { dir: apps/mobile/store } |  |  | 0.547 |
| walker |  | 1396 | 23 | Fs::DirListing { dir: apps/mobile/lib } |  |  | 0.547 |
| walker |  | 1432 | 36 | Fs::DirListing { dir: apps/mobile/app } |  |  | 0.547 |
| walker |  | 1447 | 15 | Code::CodeKey { rung: Names, file: apps/mobile/app/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.547 |
| walker |  | 1453 | 6 | Fs::DirListing { dir: apps/mobile/app/links } |  |  | 0.547 |
| walker |  | 1474 | 21 | Fs::DirListing { dir: apps/mobile/app/(tabs) } |  |  | 0.548 |
| walker |  | 1485 | 11 | Fs::DirListing { dir: apps/mobile/app/(tabs)/links } |  |  | 0.548 |
| walker |  | 1509 | 24 | Fs::DirListing { dir: apps/mobile/assets/images } |  |  | 0.548 |
| ns | 1539 |  | 268 | schema.prisma: datasource/generator plus every model and enum header, bodies elided | 2.1 |  | 0.531 |
| walker |  | 1590 | 81 | Fs::DirListing { dir: apps/web } |  |  | 0.650 |
| walker |  | 1594 | 4 | Fs::DirListing { dir: apps/web/styles } |  |  | 0.650 |
| walker |  | 1603 | 9 | Fs::DirListing { dir: apps/web/store } |  |  | 0.650 |
| walker |  | 1615 | 12 | Fs::DirListing { dir: apps/web/types } |  |  | 0.650 |
| walker |  | 1628 | 13 | Fs::DirListing { dir: apps/web/lib } |  |  | 0.650 |
| walker |  | 1649 | 21 | Fs::DirListing { dir: apps/web/templates } |  |  | 0.650 |
| walker |  | 1673 | 24 | Fs::DirListing { dir: apps/web/layouts } |  |  | 0.651 |
| walker |  | 1693 | 20 | Fs::DirListing { dir: apps/web/lib/shared } |  |  | 0.651 |
| ns | 1697 |  | 158 | All five schema enums, fully expanded | 2.2 | 2.1 | 0.606 |
| walker |  | 1722 | 29 | Fs::DirListing { dir: .github } |  |  | 0.606 |
| walker |  | 1743 | 21 | Fs::DirListing { dir: .github/workflows } |  |  | 0.607 |
| walker |  | 1767 | 24 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 77 } |  |  | 0.612 |
| walker |  | 1784 | 17 | Fs::DirListing { dir: apps/mobile/app/(tabs)/collections } |  |  | 0.612 |
| walker |  | 1801 | 17 | Fs::DirListing { dir: apps/mobile/app/(tabs)/dashboard } |  |  | 0.612 |
| walker |  | 1818 | 17 | Fs::DirListing { dir: apps/mobile/app/(tabs)/settings } |  |  | 0.612 |
| walker |  | 1835 | 17 | Fs::DirListing { dir: apps/mobile/app/(tabs)/tags } |  |  | 0.613 |
| walker |  | 1884 | 49 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 90 } |  |  | 0.630 |
| walker |  | 1917 | 33 | Code::CodeKey { rung: Names, file: packages/router/publicLinks.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.630 |
| walker |  | 1950 | 33 | Code::CodeKey { rung: Names, file: packages/router/publicTags.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.630 |
| walker |  | 1976 | 26 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 304 } |  |  | 0.646 |
| walker |  | 2011 | 35 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 83 } |  |  | 0.673 |
| ns | 2083 |  | 386 | `Link` model — every field | 2.3 | 2.1 | 0.612 |
| walker |  | 2221 | 210 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 166 } |  |  | 0.647 |
| ns | 2396 |  | 313 | `User` model — identity, relations and subscription linkage | 2.4 | 2.1 | 0.607 |
| walker |  | 2400 | 179 | Prisma::DeclTail { file: packages/prisma/schema.prisma, start_line: 166, tail_start_line: 182 } |  |  | 0.678 |
| walker |  | 2535 | 135 | Json::Dependencies { file: package.json } |  |  | 0.678 |
| walker |  | 2644 | 109 | Fs::DirListing { dir: packages/lib } |  |  | 0.681 |
| walker |  | 2657 | 13 | Code::CodeKey { rung: Names, file: packages/lib/transporter.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.681 |
| walker |  | 2671 | 14 | Code::CodeKey { rung: Names, file: packages/lib/constants.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.681 |
| walker |  | 2685 | 14 | Code::CodeKey { rung: Names, file: packages/lib/safeFetch.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.681 |
| walker |  | 2700 | 15 | Code::CodeKey { rung: Names, file: packages/lib/generatePreview.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.681 |
| walker |  | 2715 | 15 | Code::CodeKey { rung: Names, file: packages/lib/rssHandler.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.681 |
| walker |  | 2731 | 16 | Code::CodeKey { rung: Names, file: packages/lib/verifyCapacity.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.681 |
| ns | 2738 |  | 342 | `User` model — preference and archival-toggle fields | 2.5 | 2.4 | 0.646 |
| walker |  | 2748 | 17 | Code::CodeKey { rung: Names, file: packages/lib/meilisearchClient.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.646 |
| walker |  | 2766 | 18 | Code::CodeKey { rung: Names, file: packages/lib/getPreservedFormatUrl.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.646 |
| walker |  | 2789 | 23 | Code::CodeKey { rung: Names, file: packages/lib/isArchivalTag.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.646 |
| walker |  | 2813 | 24 | Code::CodeKey { rung: Names, file: packages/lib/getOriginalFormat.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.646 |
| walker |  | 2839 | 26 | Code::CodeKey { rung: Decl, file: packages/lib/getOriginalFormat.ts, decl: 1, sub: 0, line: 6 } |  |  | 0.646 |
| walker |  | 2867 | 28 | Code::CodeKey { rung: Names, file: packages/lib/formatStats.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.646 |
| walker |  | 2891 | 24 | Code::CodeKey { rung: Decl, file: packages/lib/formatStats.ts, decl: 2, sub: 0, line: 11 } |  |  | 0.646 |
| walker |  | 2919 | 28 | Code::CodeKey { rung: Names, file: packages/lib/getFormatBasedOnPreference.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.646 |
| walker |  | 2947 | 28 | Code::CodeKey { rung: Names, file: packages/lib/getLinkTypeFromFormat.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.646 |
| walker |  | 2975 | 28 | Code::CodeKey { rung: Decl, file: packages/lib/verifyCapacity.ts, decl: 1, sub: 0, line: 8 } |  |  | 0.646 |
| walker |  | 3005 | 30 | Code::CodeKey { rung: Decl, file: packages/lib/rssHandler.ts, decl: 1, sub: 0, line: 7 } |  |  | 0.646 |
| walker |  | 3035 | 30 | Code::CodeKey { rung: Decl, file: packages/lib/safeFetch.ts, decl: 1, sub: 0, line: 95 } |  |  | 0.646 |
| ns | 3055 |  | 317 | `Collection` model — every field, including the self-relation | 2.6 | 2.1 | 0.616 |
| walker |  | 3069 | 34 | Code::CodeKey { rung: Decl, file: packages/lib/getLinkTypeFromFormat.ts, decl: 1, sub: 0, line: 3 } |  |  | 0.616 |
| walker |  | 3106 | 37 | Code::CodeKey { rung: Names, file: packages/lib/getFormatFromContentType.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.616 |
| walker |  | 3163 | 57 | Fs::DirListing { dir: apps/worker/lib } |  |  | 0.617 |
| walker |  | 3202 | 39 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 289 } |  |  | 0.641 |
| walker |  | 3222 | 20 | Plaintext::Whole { file: apps/mobile/.env.sample } |  |  | 0.641 |
| walker |  | 3261 | 39 | Code::CodeKey { rung: Decl, file: packages/lib/generatePreview.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.641 |
| walker |  | 3320 | 59 | Fs::DirListing { dir: apps/mobile/components } |  |  | 0.642 |
| walker |  | 3346 | 26 | Fs::DirListing { dir: apps/mobile/components/Formats } |  |  | 0.642 |
| walker |  | 3378 | 32 | Fs::DirListing { dir: apps/mobile/components/ActionSheets } |  |  | 0.642 |
| walker |  | 3420 | 42 | Code::CodeKey { rung: Names, file: packages/types/inputSelect.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.642 |
| ns | 3436 |  | 381 | `UsersAndCollections` join model (the permission bits) and `Tag` model | 2.7 | 2.1 | 0.606 |
| walker |  | 3451 | 31 | Code::CodeKey { rung: Decl, file: packages/types/inputSelect.ts, decl: 3, sub: 0, line: 16 } |  |  | 0.606 |
| walker |  | 3486 | 35 | Code::CodeKey { rung: Decl, file: packages/types/inputSelect.ts, decl: 1, sub: 0, line: 1 } |  |  | 0.606 |
| walker |  | 3527 | 41 | Code::CodeKey { rung: Decl, file: packages/lib/transporter.ts, decl: 1, sub: 0, line: 3 } |  |  | 0.606 |
| walker |  | 3589 | 62 | Fs::DirListing { dir: apps/web/hooks } |  |  | 0.607 |
| ns | 3594 |  | 158 | `AccessToken` model | 2.8 | 2.1 | 0.594 |
| walker |  | 3607 | 18 | Fs::DirListing { dir: apps/web/e2e } |  |  | 0.595 |
| walker |  | 3613 | 6 | Fs::DirListing { dir: apps/web/e2e/tests } |  |  | 0.595 |
| ns | 3617 |  | 23 | Complete packages/prisma listing | 2.9 |  | 0.601 |
| walker |  | 3659 | 46 | Code::CodeKey { rung: Names, file: packages/router/worker.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.601 |
| walker |  | 3706 | 47 | Code::CodeKey { rung: Names, file: packages/lib/utils.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.601 |
| ns | 3733 |  | 116 | The process-wide `prisma` client singleton | 2.10 |  | 0.603 |
| ns | 3817 |  | 84 | API version directories: every resource under pages/api/v1 (and the single v2 route) | 3.1 |  | 0.577 |
| ns | 3885 |  | 68 | Route files for links, collections, tags and highlights | 3.2 |  | 0.563 |
| ns | 3962 |  | 77 | Route files for auth, session, users, tokens, config, avatar and logins | 3.3 |  | 0.549 |
| walker |  | 3991 | 285 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 28 } |  |  | 0.593 |
| ns | 4042 |  | 80 | Route files for archives, preserved, search, dashboard, rss, migration, payment, webhook, worker, getFavicon | 3.4 |  | 0.579 |
| ns | 4083 |  | 41 | The unauthenticated `public/` route subtree | 3.5 |  | 0.571 |
| walker |  | 4128 | 137 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.571 |
| walker |  | 4154 | 26 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.571 |
| walker |  | 4196 | 42 | Fs::DirListing { dir: apps/worker/lib/preservationScheme } |  |  | 0.572 |
| walker |  | 4247 | 51 | Code::CodeKey { rung: Names, file: packages/router/dashboardData.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.572 |
| walker |  | 4343 | 96 | Json::Runtime { file: package.json } |  |  | 0.572 |
| ns | 4359 |  | 276 | The route-handler pattern, read from pages/api/v1/links/index.ts | 3.6 |  | 0.557 |
| walker |  | 4498 | 155 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.557 |
| ns | 4568 |  | 209 | apps/web/lib: the server helper layer and its client/shared siblings | 3.7 |  | 0.533 |
| walker |  | 4633 | 135 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.533 |
| walker |  | 4660 | 27 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 5 } |  |  | 0.544 |
| ns | 4667 |  | 99 | The controller tree: every resource directory under lib/api/controllers | 3.8 |  | 0.530 |
| walker |  | 4711 | 51 | Code::CodeKey { rung: Decl, file: packages/lib/formatStats.ts, decl: 1, sub: 0, line: 4 } |  |  | 0.530 |
| walker |  | 4790 | 79 | Fs::DirListing { dir: apps/web/public } |  |  | 0.530 |
| walker |  | 4800 | 10 | Fs::DirListing { dir: apps/web/public/screenshots } |  |  | 0.530 |
| ns | 4823 |  | 156 | Controller files for links, collections, tags and highlights | 3.9 |  | 0.515 |
| walker |  | 4853 | 53 | Code::CodeKey { rung: Decl, file: packages/lib/getFormatBasedOnPreference.ts, decl: 1, sub: 0, line: 15 } |  |  | 0.515 |
| ns | 5001 |  | 178 | Controller files for users, tokens, session, search, dashboard, worker, migration and public access | 3.10 |  | 0.498 |
| walker |  | 5173 | 320 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 126 } |  |  | 0.534 |
| walker |  | 5228 | 55 | Code::CodeKey { rung: Decl, file: packages/lib/meilisearchClient.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.534 |
| ns | 5249 |  | 248 | Complete listings of packages/types, packages/lib, packages/filesystem and packages/router | 4.1 |  | 0.568 |
| walker |  | 5362 | 134 | Plaintext::Whole { file: .env.sample } |  |  | 0.568 |
| walker |  | 5411 | 49 | Fs::DirListing { dir: apps/web/public/locales } |  |  | 0.569 |
| walker |  | 5415 | 4 | Fs::DirListing { dir: apps/web/public/locales/de } |  |  | 0.569 |
| walker |  | 5419 | 4 | Fs::DirListing { dir: apps/web/public/locales/en } |  |  | 0.569 |
| walker |  | 5423 | 4 | Fs::DirListing { dir: apps/web/public/locales/es } |  |  | 0.569 |
| walker |  | 5427 | 4 | Fs::DirListing { dir: apps/web/public/locales/fr } |  |  | 0.569 |
| walker |  | 5431 | 4 | Fs::DirListing { dir: apps/web/public/locales/it } |  |  | 0.569 |
| walker |  | 5435 | 4 | Fs::DirListing { dir: apps/web/public/locales/ja } |  |  | 0.569 |
| walker |  | 5439 | 4 | Fs::DirListing { dir: apps/web/public/locales/nl } |  |  | 0.569 |
| walker |  | 5443 | 4 | Fs::DirListing { dir: apps/web/public/locales/pl } |  |  | 0.569 |
| walker |  | 5447 | 4 | Fs::DirListing { dir: apps/web/public/locales/pt-BR } |  |  | 0.569 |
| walker |  | 5451 | 4 | Fs::DirListing { dir: apps/web/public/locales/ro } |  |  | 0.569 |
| walker |  | 5455 | 4 | Fs::DirListing { dir: apps/web/public/locales/ru } |  |  | 0.569 |
| walker |  | 5459 | 4 | Fs::DirListing { dir: apps/web/public/locales/tr } |  |  | 0.569 |
| walker |  | 5463 | 4 | Fs::DirListing { dir: apps/web/public/locales/uk } |  |  | 0.569 |
| walker |  | 5467 | 4 | Fs::DirListing { dir: apps/web/public/locales/zh } |  |  | 0.569 |
| walker |  | 5471 | 4 | Fs::DirListing { dir: apps/web/public/locales/zh-TW } |  |  | 0.569 |
| walker |  | 5500 | 29 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.569 |
| walker |  | 5560 | 60 | Code::CodeKey { rung: Names, file: packages/router/users.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.569 |
| walker |  | 5610 | 50 | Fs::DirListing { dir: apps/mobile/components/ui } |  |  | 0.570 |
| ns | 5647 |  | 398 | packages/types/global.ts — every exported type, interface and enum declaration | 4.2 |  | 0.555 |
| walker |  | 5696 | 86 | Fs::DirListing { dir: apps/web/pages } |  |  | 0.555 |
| walker |  | 5710 | 14 | Code::CodeKey { rung: Names, file: apps/web/pages/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.555 |
| walker |  | 5716 | 6 | Fs::DirListing { dir: apps/web/pages/preserved } |  |  | 0.555 |
| walker |  | 5724 | 8 | Fs::DirListing { dir: apps/web/pages/api } |  |  | 0.556 |
| walker |  | 5727 | 3 | Fs::DirListing { dir: apps/web/pages/api/v2 } |  |  | 0.556 |
| walker |  | 5736 | 9 | Fs::DirListing { dir: apps/web/pages/public } |  |  | 0.556 |
| walker |  | 5741 | 5 | Fs::DirListing { dir: apps/web/pages/public/collections } |  |  | 0.556 |
| walker |  | 5745 | 4 | Fs::DirListing { dir: apps/web/pages/api/v2/dashboard } |  |  | 0.556 |
| walker |  | 5751 | 6 | Fs::DirListing { dir: apps/web/pages/public/links } |  |  | 0.556 |
| walker |  | 5757 | 6 | Fs::DirListing { dir: apps/web/pages/public/preserved } |  |  | 0.557 |
| walker |  | 5768 | 11 | Fs::DirListing { dir: apps/web/pages/collections } |  |  | 0.557 |
| walker |  | 5779 | 11 | Fs::DirListing { dir: apps/web/pages/tags } |  |  | 0.557 |
| walker |  | 5791 | 12 | Fs::DirListing { dir: apps/web/pages/auth } |  |  | 0.557 |
| ns | 5793 |  | 146 | `ArchivedFormat`, `LinkType` and `TokenExpiry` variants | 4.3 | 4.2 | 0.550 |
| walker |  | 5807 | 16 | Fs::DirListing { dir: apps/web/pages/links } |  |  | 0.550 |
| walker |  | 5826 | 19 | Fs::DirListing { dir: apps/web/pages/admin } |  |  | 0.551 |
| walker |  | 5836 | 10 | Fs::DirListing { dir: apps/web/pages/public/collections/[id] } |  |  | 0.551 |
| walker |  | 5879 | 43 | Code::CodeKey { rung: Body, file: apps/web/pages/index.tsx, decl: 1, sub: 0, line: 4 } |  |  | 0.551 |
| ns | 5945 |  | 152 | `ViewMode`, `Sort` and `TagSort` variants | 4.4 | 4.2 | 0.544 |
| walker |  | 6252 | 373 | Prisma::DeclTail { file: packages/prisma/schema.prisma, start_line: 28, tail_start_line: 51 } |  |  | 0.578 |
| ns | 6285 |  | 340 | packages/lib/schemaValidation.ts — every exported zod schema constant | 4.5 |  | 0.567 |
| walker |  | 6314 | 62 | Code::CodeKey { rung: Names, file: packages/router/tokens.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.567 |
| walker |  | 6330 | 16 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 1 } |  |  | 0.573 |
| walker |  | 6395 | 65 | Code::CodeKey { rung: Names, file: packages/filesystem/manageFiles.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.573 |
| walker |  | 6412 | 17 | Fs::DirListing { dir: apps/web/e2e/fixtures } |  |  | 0.573 |
| walker |  | 6425 | 13 | Fs::DirListing { dir: apps/web/e2e/fixtures/base } |  |  | 0.574 |
| walker |  | 6492 | 67 | Code::CodeKey { rung: Names, file: packages/router/user.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.574 |
| ns | 6542 |  | 257 | packages/router — the links, collections and tags hook exports | 4.6 |  | 0.561 |
| walker |  | 6548 | 56 | Fs::DirListing { dir: apps/web/pages/settings } |  |  | 0.562 |
| walker |  | 6620 | 72 | Code::CodeKey { rung: Names, file: packages/router/highlights.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.562 |
| walker |  | 6645 | 25 | Code::CodeKey { rung: Decl, file: packages/router/highlights.tsx, decl: 1, sub: 0, line: 11 } |  |  | 0.562 |
| walker |  | 6717 | 72 | Code::CodeKey { rung: Names, file: packages/router/rss.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.562 |
| ns | 6726 |  | 184 | packages/router — export lines of the remaining ten hook modules | 4.7 | 4.6 | 0.565 |
| walker |  | 6729 | 12 | Code::CodeKey { rung: Body, file: packages/lib/utils.ts, decl: 3, sub: 0, line: 17 } |  |  | 0.565 |
| walker |  | 6803 | 74 | Code::CodeKey { rung: Decl, file: packages/filesystem/createFile.ts, decl: 1, sub: 0, line: 6 } |  |  | 0.565 |
| walker |  | 6825 | 22 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.566 |
| ns | 6892 |  | 166 | Complete apps/worker listing, including every job and every preservation handler | 5.1 |  | 0.582 |
| walker |  | 6908 | 83 | Code::CodeKey { rung: Decl, file: packages/types/inputSelect.ts, decl: 2, sub: 0, line: 7 } |  |  | 0.582 |
| walker |  | 6998 | 90 | Code::CodeKey { rung: Names, file: packages/router/config.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.584 |
| walker |  | 7028 | 30 | Code::CodeKey { rung: Decl, file: packages/router/config.tsx, decl: 3, sub: 0, line: 44 } |  |  | 0.584 |
| walker |  | 7117 | 89 | Code::CodeKey { rung: Decl, file: packages/lib/isArchivalTag.ts, decl: 1, sub: 0, line: 3 } |  |  | 0.584 |
| ns | 7161 |  | 269 | worker.ts — the whole scheduler entry point | 5.2 |  | 0.575 |
| ns | 7354 |  | 193 | archiveHandler's signature and its SSRF / skip-preservation guard | 5.3 |  | 0.567 |
| walker |  | 7552 | 435 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: true } |  |  | 0.600 |
| ns | 7607 |  | 253 | Every UI page route under apps/web/pages | 6.1 |  | 0.622 |
| walker |  | 7631 | 79 | Fs::DirListing { dir: apps/web/lib/client } |  |  | 0.630 |
| walker |  | 7886 | 255 | Code::CodeKey { rung: Names, file: packages/lib/schemaValidation.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.635 |
| walker |  | 7899 | 13 | Code::CodeKey { rung: Decl, file: packages/lib/schemaValidation.ts, decl: 3, sub: 0, line: 25 } |  |  | 0.635 |
| walker |  | 7913 | 14 | Code::CodeKey { rung: Decl, file: packages/lib/schemaValidation.ts, decl: 9, sub: 0, line: 115 } |  |  | 0.635 |
| walker |  | 7928 | 15 | Code::CodeKey { rung: Decl, file: packages/lib/schemaValidation.ts, decl: 1, sub: 0, line: 16 } |  |  | 0.635 |
| walker |  | 7955 | 27 | Code::CodeKey { rung: Decl, file: packages/lib/schemaValidation.ts, decl: 2, sub: 0, line: 20 } |  |  | 0.635 |
| ns | 7965 |  | 358 | The flat components/ directory and the ui/ primitives | 6.2 |  | 0.609 |
| walker |  | 7984 | 29 | Code::CodeKey { rung: Decl, file: packages/lib/schemaValidation.ts, decl: 4, sub: 0, line: 29 } |  |  | 0.609 |
| walker |  | 8038 | 54 | Code::CodeKey { rung: Decl, file: packages/lib/schemaValidation.ts, decl: 10, sub: 0, line: 119 } |  |  | 0.609 |
| walker |  | 8149 | 111 | Code::CodeKey { rung: Names, file: packages/router/collections.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.611 |
| walker |  | 8229 | 80 | Code::CodeKey { rung: Decl, file: packages/router/collections.tsx, decl: 4, sub: 0, line: 180 } |  |  | 0.611 |
| ns | 8246 |  | 281 | Modal, link-view, preservation and input-picker component subdirectories | 6.3 | 6.2 | 0.595 |
| walker |  | 8338 | 109 | Code::CodeKey { rung: Decl, file: packages/router/config.tsx, decl: 1, sub: 0, line: 4 } |  |  | 0.595 |
| walker |  | 8356 | 18 | Code::CodeKey { rung: Body, file: packages/router/config.tsx, decl: 3, sub: 0, line: 44 } |  |  | 0.595 |
| walker |  | 8453 | 97 | Fs::DirListing { dir: apps/web/lib/api } |  |  | 0.618 |
| walker |  | 8466 | 13 | Fs::DirListing { dir: apps/web/lib/api/archives } |  |  | 0.618 |
| ns | 8473 |  | 227 | Web hooks, layouts, stores, ambient types, email templates, one-off migration scripts and the Playwright suite | 6.4 |  | 0.622 |
| walker |  | 8483 | 17 | Fs::DirListing { dir: apps/web/lib/api/preserved } |  |  | 0.623 |
| walker |  | 8516 | 33 | Fs::DirListing { dir: apps/web/lib/api/stripe } |  |  | 0.626 |
| walker |  | 8552 | 36 | Fs::DirListing { dir: apps/web/lib/api/controllers } |  |  | 0.639 |
| walker |  | 8557 | 5 | Fs::DirListing { dir: apps/web/lib/api/controllers/search } |  |  | 0.639 |
| walker |  | 8562 | 5 | Fs::DirListing { dir: apps/web/lib/api/controllers/session } |  |  | 0.639 |
| walker |  | 8568 | 6 | Fs::DirListing { dir: apps/web/lib/api/controllers/worker } |  |  | 0.640 |
| walker |  | 8577 | 9 | Fs::DirListing { dir: apps/web/lib/api/controllers/public } |  |  | 0.640 |
| walker |  | 8581 | 4 | Fs::DirListing { dir: apps/web/lib/api/controllers/public/links } |  |  | 0.640 |
| walker |  | 8587 | 6 | Fs::DirListing { dir: apps/web/lib/api/controllers/public/collections } |  |  | 0.641 |
| walker |  | 8593 | 6 | Fs::DirListing { dir: apps/web/lib/api/controllers/public/users } |  |  | 0.641 |
| walker |  | 8600 | 7 | Fs::DirListing { dir: apps/web/lib/api/controllers/public/links/linkId } |  |  | 0.642 |
| walker |  | 8614 | 14 | Fs::DirListing { dir: apps/web/lib/api/controllers/collections } |  |  | 0.642 |
| walker |  | 8628 | 14 | Fs::DirListing { dir: apps/web/lib/api/controllers/highlights } |  |  | 0.642 |
| walker |  | 8642 | 14 | Fs::DirListing { dir: apps/web/lib/api/controllers/tokens } |  |  | 0.644 |
| walker |  | 8649 | 7 | Fs::DirListing { dir: apps/web/lib/api/controllers/tokens/tokenId } |  |  | 0.645 |
| walker |  | 8663 | 14 | Fs::DirListing { dir: apps/web/lib/api/controllers/users } |  |  | 0.647 |
| walker |  | 8680 | 17 | Fs::DirListing { dir: apps/web/lib/api/controllers/links } |  |  | 0.649 |
| walker |  | 8692 | 12 | Fs::DirListing { dir: apps/web/lib/api/controllers/links/bulk } |  |  | 0.650 |
| walker |  | 8712 | 20 | Fs::DirListing { dir: apps/web/lib/api/controllers/dashboard } |  |  | 0.652 |
| ns | 8713 |  | 240 | verifyUser — the guard chain every authenticated route runs first | 7.1 |  | 0.643 |
| walker |  | 8739 | 27 | Fs::DirListing { dir: apps/web/lib/api/controllers/tags } |  |  | 0.647 |
| walker |  | 8760 | 21 | Fs::DirListing { dir: apps/web/lib/api/controllers/collections/collectionId } |  |  | 0.649 |
| walker |  | 8781 | 21 | Fs::DirListing { dir: apps/web/lib/api/controllers/tags/tagId } |  |  | 0.653 |
| walker |  | 8805 | 24 | Fs::DirListing { dir: apps/web/lib/api/controllers/links/linkId } |  |  | 0.658 |
| walker |  | 8811 | 6 | Fs::DirListing { dir: apps/web/lib/api/controllers/links/linkId/highlight } |  |  | 0.659 |
| walker |  | 8838 | 27 | Fs::DirListing { dir: apps/web/lib/api/controllers/users/userId } |  |  | 0.664 |
| ns | 8948 |  | 235 | docker-compose.yml — the three-container deployment | 7.2 |  | 0.656 |
| walker |  | 8957 | 119 | Code::CodeKey { rung: Decl, file: packages/lib/getPreservedFormatUrl.ts, decl: 1, sub: 0, line: 3 } |  |  | 0.656 |
| walker |  | 9026 | 69 | Fs::DirListing { dir: apps/web/pages/api/v1 } |  |  | 0.675 |
| walker |  | 9030 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/dashboard } |  |  | 0.675 |
| walker |  | 9034 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/getFavicon } |  |  | 0.675 |
| walker |  | 9038 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/logins } |  |  | 0.675 |
| walker |  | 9042 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/migration } |  |  | 0.675 |
| walker |  | 9046 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/payment } |  |  | 0.676 |
| walker |  | 9050 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/search } |  |  | 0.676 |
| walker |  | 9054 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/session } |  |  | 0.676 |
| walker |  | 9058 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/webhook } |  |  | 0.677 |
| walker |  | 9064 | 6 | Fs::DirListing { dir: apps/web/pages/api/v1/avatar } |  |  | 0.677 |
| walker |  | 9073 | 9 | Fs::DirListing { dir: apps/web/pages/api/v1/config } |  |  | 0.678 |
| walker |  | 9082 | 9 | Fs::DirListing { dir: apps/web/pages/api/v1/public } |  |  | 0.678 |
| walker |  | 9088 | 6 | Fs::DirListing { dir: apps/web/pages/api/v1/public/links } |  |  | 0.679 |
| walker |  | 9094 | 6 | Fs::DirListing { dir: apps/web/pages/api/v1/public/users } |  |  | 0.680 |
| walker |  | 9103 | 9 | Fs::DirListing { dir: apps/web/pages/api/v1/worker } |  |  | 0.681 |
| walker |  | 9113 | 10 | Fs::DirListing { dir: apps/web/pages/api/v1/collections } |  |  | 0.681 |
| walker |  | 9123 | 10 | Fs::DirListing { dir: apps/web/pages/api/v1/highlights } |  |  | 0.682 |
| walker |  | 9133 | 10 | Fs::DirListing { dir: apps/web/pages/api/v1/rss } |  |  | 0.683 |
| walker |  | 9143 | 10 | Fs::DirListing { dir: apps/web/pages/api/v1/tokens } |  |  | 0.684 |
| walker |  | 9155 | 12 | Fs::DirListing { dir: apps/web/pages/api/v1/links } |  |  | 0.686 |
| walker |  | 9159 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/links/archive } |  |  | 0.686 |
| ns | 9162 |  | 214 | .env.sample — the required variables and the complete list of section headings | 7.3 |  | 0.683 |
| walker |  | 9172 | 13 | Fs::DirListing { dir: apps/web/pages/api/v1/users } |  |  | 0.685 |
| walker |  | 9181 | 9 | Fs::DirListing { dir: apps/web/pages/api/v1/users/[id] } |  |  | 0.687 |
| walker |  | 9195 | 14 | Fs::DirListing { dir: apps/web/pages/api/v1/tags } |  |  | 0.690 |
| walker |  | 9205 | 10 | Fs::DirListing { dir: apps/web/pages/api/v1/links/[id] } |  |  | 0.693 |
| walker |  | 9209 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/links/[id]/archive } |  |  | 0.695 |
| walker |  | 9213 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/links/[id]/highlights } |  |  | 0.696 |
| walker |  | 9225 | 12 | Fs::DirListing { dir: apps/web/pages/api/v1/public/collections } |  |  | 0.699 |
| walker |  | 9229 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/public/collections/links } |  |  | 0.700 |
| walker |  | 9233 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/public/collections/tags } |  |  | 0.701 |
| walker |  | 9251 | 18 | Fs::DirListing { dir: apps/web/pages/api/v1/preserved } |  |  | 0.705 |
| walker |  | 9270 | 19 | Fs::DirListing { dir: apps/web/pages/api/v1/archives } |  |  | 0.709 |
| walker |  | 9292 | 22 | Fs::DirListing { dir: apps/web/pages/api/v1/auth } |  |  | 0.714 |
| walker |  | 9340 | 48 | Fs::DirListing { dir: apps/web/lib/api/controllers/migration } |  |  | 0.723 |
| ns | 9341 |  | 179 | The head of the optional tuning-variable block | 7.4 | 7.3 | 0.715 |
| ns | 9473 |  | 132 | CI workflows, GitHub templates, the patch-package patch, and every translated locale | 7.5 |  | 0.721 |
| walker |  | 9650 | 310 | Plaintext::Whole { file: docker-compose.yml } |  |  | 0.734 |
| walker |  | 9672 | 22 | Code::CodeKey { rung: Body, file: packages/lib/utils.ts, decl: 2, sub: 0, line: 13 } |  |  | 0.734 |
| ns | 9689 |  | 216 | apps/mobile root and the complete Expo Router screen tree | 8.1 |  | 0.741 |
| walker |  | 9695 | 23 | Code::CodeKey { rung: Body, file: packages/lib/formatStats.ts, decl: 1, sub: 0, line: 4 } |  |  | 0.741 |
| ns | 9891 |  | 202 | Mobile components, stores and query-cache modules | 8.2 | 8.1 | 0.746 |

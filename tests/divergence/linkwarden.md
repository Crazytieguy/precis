Score(3000)=0.672 I=0.866 C=0.522 ns_rows≤3K=15/51 grid(1000/1442/2080/3000/4327/6240/9000)=0.622/0.547/0.673/0.672/0.573/0.545/0.657

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
| walker |  | 1646 | 18 | Fs::DirListing { dir: apps/web/e2e } |  |  | 0.650 |
| walker |  | 1672 | 26 | Code::CodeKey { rung: Names, file: apps/web/e2e/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.650 |
| walker |  | 1676 | 4 | Fs::DirListing { dir: apps/web/e2e/data } |  |  | 0.650 |
| walker |  | 1697 | 21 | Fs::DirListing { dir: apps/web/templates } |  |  | 0.606 |
| ns | 1697 |  | 158 | All five schema enums, fully expanded | 2.2 | 2.1 | 0.606 |
| walker |  | 1703 | 6 | Fs::DirListing { dir: apps/web/e2e/tests } |  |  | 0.606 |
| walker |  | 1727 | 24 | Fs::DirListing { dir: apps/web/layouts } |  |  | 0.607 |
| walker |  | 1747 | 20 | Fs::DirListing { dir: apps/web/lib/shared } |  |  | 0.607 |
| walker |  | 1771 | 24 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 77 } |  |  | 0.612 |
| walker |  | 1788 | 17 | Fs::DirListing { dir: apps/mobile/app/(tabs)/collections } |  |  | 0.612 |
| walker |  | 1805 | 17 | Fs::DirListing { dir: apps/mobile/app/(tabs)/dashboard } |  |  | 0.612 |
| walker |  | 1822 | 17 | Fs::DirListing { dir: apps/mobile/app/(tabs)/settings } |  |  | 0.613 |
| walker |  | 1839 | 17 | Fs::DirListing { dir: apps/mobile/app/(tabs)/tags } |  |  | 0.613 |
| walker |  | 1888 | 49 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 90 } |  |  | 0.631 |
| walker |  | 1917 | 29 | Fs::DirListing { dir: .github } |  |  | 0.631 |
| walker |  | 1938 | 21 | Fs::DirListing { dir: .github/workflows } |  |  | 0.631 |
| walker |  | 1971 | 33 | Code::CodeKey { rung: Names, file: packages/router/publicLinks.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.631 |
| walker |  | 2004 | 33 | Code::CodeKey { rung: Names, file: packages/router/publicTags.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.631 |
| walker |  | 2030 | 26 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 304 } |  |  | 0.646 |
| walker |  | 2065 | 35 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 83 } |  |  | 0.673 |
| ns | 2083 |  | 386 | `Link` model — every field | 2.3 | 2.1 | 0.613 |
| walker |  | 2275 | 210 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 166 } |  |  | 0.647 |
| ns | 2396 |  | 313 | `User` model — identity, relations and subscription linkage | 2.4 | 2.1 | 0.607 |
| walker |  | 2454 | 179 | Prisma::DeclTail { file: packages/prisma/schema.prisma, start_line: 166, tail_start_line: 182 } |  |  | 0.678 |
| walker |  | 2589 | 135 | Json::Dependencies { file: package.json } |  |  | 0.678 |
| walker |  | 2628 | 39 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 289 } |  |  | 0.705 |
| walker |  | 2648 | 20 | Plaintext::Whole { file: apps/mobile/.env.sample } |  |  | 0.705 |
| ns | 2738 |  | 342 | `User` model — preference and archival-toggle fields | 2.5 | 2.4 | 0.669 |
| walker |  | 2757 | 109 | Fs::DirListing { dir: packages/lib } |  |  | 0.672 |
| walker |  | 2770 | 13 | Code::CodeKey { rung: Names, file: packages/lib/transporter.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.672 |
| walker |  | 2784 | 14 | Code::CodeKey { rung: Names, file: packages/lib/constants.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.672 |
| walker |  | 2798 | 14 | Code::CodeKey { rung: Names, file: packages/lib/safeFetch.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.672 |
| walker |  | 2813 | 15 | Code::CodeKey { rung: Names, file: packages/lib/generatePreview.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.672 |
| walker |  | 2828 | 15 | Code::CodeKey { rung: Names, file: packages/lib/rssHandler.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.672 |
| walker |  | 2844 | 16 | Code::CodeKey { rung: Names, file: packages/lib/verifyCapacity.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.672 |
| walker |  | 2861 | 17 | Code::CodeKey { rung: Names, file: packages/lib/meilisearchClient.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.672 |
| walker |  | 2879 | 18 | Code::CodeKey { rung: Names, file: packages/lib/getPreservedFormatUrl.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.672 |
| walker |  | 2902 | 23 | Code::CodeKey { rung: Names, file: packages/lib/isArchivalTag.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.672 |
| walker |  | 2926 | 24 | Code::CodeKey { rung: Names, file: packages/lib/getOriginalFormat.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.672 |
| walker |  | 2952 | 26 | Code::CodeKey { rung: Decl, file: packages/lib/getOriginalFormat.ts, decl: 1, sub: 0, line: 6 } |  |  | 0.672 |
| walker |  | 2980 | 28 | Code::CodeKey { rung: Names, file: packages/lib/formatStats.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.672 |
| walker |  | 3004 | 24 | Code::CodeKey { rung: Decl, file: packages/lib/formatStats.ts, decl: 2, sub: 0, line: 11 } |  |  | 0.672 |
| walker |  | 3032 | 28 | Code::CodeKey { rung: Names, file: packages/lib/getFormatBasedOnPreference.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.672 |
| ns | 3055 |  | 317 | `Collection` model — every field, including the self-relation | 2.6 | 2.1 | 0.641 |
| walker |  | 3060 | 28 | Code::CodeKey { rung: Names, file: packages/lib/getLinkTypeFromFormat.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.641 |
| walker |  | 3088 | 28 | Code::CodeKey { rung: Decl, file: packages/lib/verifyCapacity.ts, decl: 1, sub: 0, line: 8 } |  |  | 0.641 |
| walker |  | 3118 | 30 | Code::CodeKey { rung: Decl, file: packages/lib/rssHandler.ts, decl: 1, sub: 0, line: 7 } |  |  | 0.641 |
| walker |  | 3148 | 30 | Code::CodeKey { rung: Decl, file: packages/lib/safeFetch.ts, decl: 1, sub: 0, line: 95 } |  |  | 0.641 |
| walker |  | 3182 | 34 | Code::CodeKey { rung: Decl, file: packages/lib/getLinkTypeFromFormat.ts, decl: 1, sub: 0, line: 3 } |  |  | 0.641 |
| walker |  | 3219 | 37 | Code::CodeKey { rung: Names, file: packages/lib/getFormatFromContentType.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.641 |
| walker |  | 3276 | 57 | Fs::DirListing { dir: apps/worker/lib } |  |  | 0.642 |
| walker |  | 3315 | 39 | Code::CodeKey { rung: Decl, file: packages/lib/generatePreview.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.642 |
| walker |  | 3374 | 59 | Fs::DirListing { dir: apps/mobile/components } |  |  | 0.642 |
| walker |  | 3400 | 26 | Fs::DirListing { dir: apps/mobile/components/Formats } |  |  | 0.642 |
| walker |  | 3432 | 32 | Fs::DirListing { dir: apps/mobile/components/ActionSheets } |  |  | 0.643 |
| ns | 3436 |  | 381 | `UsersAndCollections` join model (the permission bits) and `Tag` model | 2.7 | 2.1 | 0.607 |
| walker |  | 3474 | 42 | Code::CodeKey { rung: Names, file: packages/types/inputSelect.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.607 |
| walker |  | 3505 | 31 | Code::CodeKey { rung: Decl, file: packages/types/inputSelect.ts, decl: 3, sub: 0, line: 16 } |  |  | 0.607 |
| walker |  | 3540 | 35 | Code::CodeKey { rung: Decl, file: packages/types/inputSelect.ts, decl: 1, sub: 0, line: 1 } |  |  | 0.607 |
| walker |  | 3581 | 41 | Code::CodeKey { rung: Decl, file: packages/lib/transporter.ts, decl: 1, sub: 0, line: 3 } |  |  | 0.607 |
| ns | 3594 |  | 158 | `AccessToken` model | 2.8 | 2.1 | 0.594 |
| ns | 3617 |  | 23 | Complete packages/prisma listing | 2.9 |  | 0.601 |
| walker |  | 3643 | 62 | Fs::DirListing { dir: apps/web/hooks } |  |  | 0.602 |
| walker |  | 3660 | 17 | Fs::DirListing { dir: apps/web/e2e/fixtures } |  |  | 0.602 |
| walker |  | 3673 | 13 | Fs::DirListing { dir: apps/web/e2e/fixtures/base } |  |  | 0.602 |
| walker |  | 3719 | 46 | Code::CodeKey { rung: Names, file: packages/router/worker.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.602 |
| ns | 3733 |  | 116 | The process-wide `prisma` client singleton | 2.10 |  | 0.604 |
| walker |  | 3766 | 47 | Code::CodeKey { rung: Names, file: packages/lib/utils.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.604 |
| ns | 3817 |  | 84 | API version directories: every resource under pages/api/v1 (and the single v2 route) | 3.1 |  | 0.578 |
| ns | 3885 |  | 68 | Route files for links, collections, tags and highlights | 3.2 |  | 0.564 |
| ns | 3962 |  | 77 | Route files for auth, session, users, tokens, config, avatar and logins | 3.3 |  | 0.550 |
| ns | 4042 |  | 80 | Route files for archives, preserved, search, dashboard, rss, migration, payment, webhook, worker, getFavicon | 3.4 |  | 0.537 |
| walker |  | 4051 | 285 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 28 } |  |  | 0.580 |
| ns | 4083 |  | 41 | The unauthenticated `public/` route subtree | 3.5 |  | 0.572 |
| walker |  | 4188 | 137 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.572 |
| walker |  | 4214 | 26 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.572 |
| walker |  | 4256 | 42 | Fs::DirListing { dir: apps/worker/lib/preservationScheme } |  |  | 0.573 |
| walker |  | 4307 | 51 | Code::CodeKey { rung: Names, file: packages/router/dashboardData.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.573 |
| ns | 4359 |  | 276 | The route-handler pattern, read from pages/api/v1/links/index.ts | 3.6 |  | 0.557 |
| walker |  | 4403 | 96 | Json::Runtime { file: package.json } |  |  | 0.557 |
| walker |  | 4558 | 155 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.557 |
| ns | 4568 |  | 209 | apps/web/lib: the server helper layer and its client/shared siblings | 3.7 |  | 0.533 |
| ns | 4667 |  | 99 | The controller tree: every resource directory under lib/api/controllers | 3.8 |  | 0.520 |
| walker |  | 4693 | 135 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.520 |
| walker |  | 4720 | 27 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 5 } |  |  | 0.531 |
| walker |  | 4771 | 51 | Code::CodeKey { rung: Decl, file: packages/lib/formatStats.ts, decl: 1, sub: 0, line: 4 } |  |  | 0.531 |
| walker |  | 4787 | 16 | Plaintext::Whole { file: apps/web/e2e/.env.example } |  |  | 0.531 |
| ns | 4823 |  | 156 | Controller files for links, collections, tags and highlights | 3.9 |  | 0.515 |
| walker |  | 4866 | 79 | Fs::DirListing { dir: apps/web/public } |  |  | 0.515 |
| walker |  | 4876 | 10 | Fs::DirListing { dir: apps/web/public/screenshots } |  |  | 0.515 |
| walker |  | 4929 | 53 | Code::CodeKey { rung: Decl, file: packages/lib/getFormatBasedOnPreference.ts, decl: 1, sub: 0, line: 15 } |  |  | 0.515 |
| ns | 5001 |  | 178 | Controller files for users, tokens, session, search, dashboard, worker, migration and public access | 3.10 |  | 0.498 |
| walker |  | 5249 | 320 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 126 } |  |  | 0.569 |
| ns | 5249 |  | 248 | Complete listings of packages/types, packages/lib, packages/filesystem and packages/router | 4.1 |  | 0.569 |
| walker |  | 5304 | 55 | Code::CodeKey { rung: Decl, file: packages/lib/meilisearchClient.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.569 |
| walker |  | 5326 | 22 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.569 |
| walker |  | 5460 | 134 | Plaintext::Whole { file: .env.sample } |  |  | 0.569 |
| walker |  | 5509 | 49 | Fs::DirListing { dir: apps/web/public/locales } |  |  | 0.570 |
| walker |  | 5513 | 4 | Fs::DirListing { dir: apps/web/public/locales/de } |  |  | 0.570 |
| walker |  | 5517 | 4 | Fs::DirListing { dir: apps/web/public/locales/en } |  |  | 0.570 |
| walker |  | 5521 | 4 | Fs::DirListing { dir: apps/web/public/locales/es } |  |  | 0.570 |
| walker |  | 5525 | 4 | Fs::DirListing { dir: apps/web/public/locales/fr } |  |  | 0.570 |
| walker |  | 5529 | 4 | Fs::DirListing { dir: apps/web/public/locales/it } |  |  | 0.570 |
| walker |  | 5533 | 4 | Fs::DirListing { dir: apps/web/public/locales/ja } |  |  | 0.570 |
| walker |  | 5537 | 4 | Fs::DirListing { dir: apps/web/public/locales/nl } |  |  | 0.570 |
| walker |  | 5541 | 4 | Fs::DirListing { dir: apps/web/public/locales/pl } |  |  | 0.570 |
| walker |  | 5545 | 4 | Fs::DirListing { dir: apps/web/public/locales/pt-BR } |  |  | 0.570 |
| walker |  | 5549 | 4 | Fs::DirListing { dir: apps/web/public/locales/ro } |  |  | 0.570 |
| walker |  | 5553 | 4 | Fs::DirListing { dir: apps/web/public/locales/ru } |  |  | 0.570 |
| walker |  | 5557 | 4 | Fs::DirListing { dir: apps/web/public/locales/tr } |  |  | 0.570 |
| walker |  | 5561 | 4 | Fs::DirListing { dir: apps/web/public/locales/uk } |  |  | 0.570 |
| walker |  | 5565 | 4 | Fs::DirListing { dir: apps/web/public/locales/zh } |  |  | 0.570 |
| walker |  | 5569 | 4 | Fs::DirListing { dir: apps/web/public/locales/zh-TW } |  |  | 0.570 |
| walker |  | 5598 | 29 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.570 |
| ns | 5647 |  | 398 | packages/types/global.ts — every exported type, interface and enum declaration | 4.2 |  | 0.555 |
| walker |  | 5658 | 60 | Code::CodeKey { rung: Names, file: packages/router/users.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.555 |
| walker |  | 5708 | 50 | Fs::DirListing { dir: apps/mobile/components/ui } |  |  | 0.556 |
| ns | 5793 |  | 146 | `ArchivedFormat`, `LinkType` and `TokenExpiry` variants | 4.3 | 4.2 | 0.549 |
| walker |  | 5794 | 86 | Fs::DirListing { dir: apps/web/pages } |  |  | 0.549 |
| walker |  | 5808 | 14 | Code::CodeKey { rung: Names, file: apps/web/pages/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.549 |
| walker |  | 5814 | 6 | Fs::DirListing { dir: apps/web/pages/preserved } |  |  | 0.549 |
| walker |  | 5822 | 8 | Fs::DirListing { dir: apps/web/pages/api } |  |  | 0.550 |
| walker |  | 5825 | 3 | Fs::DirListing { dir: apps/web/pages/api/v2 } |  |  | 0.550 |
| walker |  | 5834 | 9 | Fs::DirListing { dir: apps/web/pages/public } |  |  | 0.550 |
| walker |  | 5839 | 5 | Fs::DirListing { dir: apps/web/pages/public/collections } |  |  | 0.550 |
| walker |  | 5843 | 4 | Fs::DirListing { dir: apps/web/pages/api/v2/dashboard } |  |  | 0.550 |
| walker |  | 5849 | 6 | Fs::DirListing { dir: apps/web/pages/public/links } |  |  | 0.550 |
| walker |  | 5855 | 6 | Fs::DirListing { dir: apps/web/pages/public/preserved } |  |  | 0.551 |
| walker |  | 5866 | 11 | Fs::DirListing { dir: apps/web/pages/collections } |  |  | 0.551 |
| walker |  | 5877 | 11 | Fs::DirListing { dir: apps/web/pages/tags } |  |  | 0.551 |
| walker |  | 5889 | 12 | Fs::DirListing { dir: apps/web/pages/auth } |  |  | 0.551 |
| walker |  | 5905 | 16 | Fs::DirListing { dir: apps/web/pages/links } |  |  | 0.551 |
| walker |  | 5924 | 19 | Fs::DirListing { dir: apps/web/pages/admin } |  |  | 0.551 |
| walker |  | 5934 | 10 | Fs::DirListing { dir: apps/web/pages/public/collections/[id] } |  |  | 0.552 |
| ns | 5945 |  | 152 | `ViewMode`, `Sort` and `TagSort` variants | 4.4 | 4.2 | 0.545 |
| walker |  | 5977 | 43 | Code::CodeKey { rung: Body, file: apps/web/pages/index.tsx, decl: 1, sub: 0, line: 4 } |  |  | 0.545 |
| ns | 6285 |  | 340 | packages/lib/schemaValidation.ts — every exported zod schema constant | 4.5 |  | 0.535 |
| walker |  | 6350 | 373 | Prisma::DeclTail { file: packages/prisma/schema.prisma, start_line: 28, tail_start_line: 51 } |  |  | 0.568 |
| walker |  | 6412 | 62 | Code::CodeKey { rung: Names, file: packages/router/tokens.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.568 |
| walker |  | 6428 | 16 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 1 } |  |  | 0.574 |
| walker |  | 6493 | 65 | Code::CodeKey { rung: Names, file: packages/filesystem/manageFiles.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.574 |
| ns | 6542 |  | 257 | packages/router — the links, collections and tags hook exports | 4.6 |  | 0.561 |
| walker |  | 6560 | 67 | Code::CodeKey { rung: Names, file: packages/router/user.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.561 |
| walker |  | 6616 | 56 | Fs::DirListing { dir: apps/web/pages/settings } |  |  | 0.562 |
| walker |  | 6688 | 72 | Code::CodeKey { rung: Names, file: packages/router/highlights.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.562 |
| walker |  | 6713 | 25 | Code::CodeKey { rung: Decl, file: packages/router/highlights.tsx, decl: 1, sub: 0, line: 11 } |  |  | 0.562 |
| ns | 6726 |  | 184 | packages/router — export lines of the remaining ten hook modules | 4.7 | 4.6 | 0.564 |
| walker |  | 6785 | 72 | Code::CodeKey { rung: Names, file: packages/router/rss.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.566 |
| walker |  | 6797 | 12 | Code::CodeKey { rung: Body, file: packages/lib/utils.ts, decl: 3, sub: 0, line: 17 } |  |  | 0.566 |
| walker |  | 6871 | 74 | Code::CodeKey { rung: Decl, file: packages/filesystem/createFile.ts, decl: 1, sub: 0, line: 6 } |  |  | 0.566 |
| ns | 6892 |  | 166 | Complete apps/worker listing, including every job and every preservation handler | 5.1 |  | 0.582 |
| walker |  | 6954 | 83 | Code::CodeKey { rung: Decl, file: packages/types/inputSelect.ts, decl: 2, sub: 0, line: 7 } |  |  | 0.582 |
| walker |  | 7044 | 90 | Code::CodeKey { rung: Names, file: packages/router/config.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.584 |
| walker |  | 7074 | 30 | Code::CodeKey { rung: Decl, file: packages/router/config.tsx, decl: 3, sub: 0, line: 44 } |  |  | 0.584 |
| ns | 7161 |  | 269 | worker.ts — the whole scheduler entry point | 5.2 |  | 0.575 |
| walker |  | 7163 | 89 | Code::CodeKey { rung: Decl, file: packages/lib/isArchivalTag.ts, decl: 1, sub: 0, line: 3 } |  |  | 0.575 |
| ns | 7354 |  | 193 | archiveHandler's signature and its SSRF / skip-preservation guard | 5.3 |  | 0.567 |
| walker |  | 7598 | 435 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: true } |  |  | 0.600 |
| ns | 7607 |  | 253 | Every UI page route under apps/web/pages | 6.1 |  | 0.622 |
| walker |  | 7677 | 79 | Fs::DirListing { dir: apps/web/lib/client } |  |  | 0.630 |
| walker |  | 7932 | 255 | Code::CodeKey { rung: Names, file: packages/lib/schemaValidation.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.635 |
| walker |  | 7945 | 13 | Code::CodeKey { rung: Decl, file: packages/lib/schemaValidation.ts, decl: 3, sub: 0, line: 25 } |  |  | 0.635 |
| walker |  | 7959 | 14 | Code::CodeKey { rung: Decl, file: packages/lib/schemaValidation.ts, decl: 9, sub: 0, line: 115 } |  |  | 0.635 |
| ns | 7965 |  | 358 | The flat components/ directory and the ui/ primitives | 6.2 |  | 0.610 |
| walker |  | 7974 | 15 | Code::CodeKey { rung: Decl, file: packages/lib/schemaValidation.ts, decl: 1, sub: 0, line: 16 } |  |  | 0.610 |
| walker |  | 8001 | 27 | Code::CodeKey { rung: Decl, file: packages/lib/schemaValidation.ts, decl: 2, sub: 0, line: 20 } |  |  | 0.610 |
| walker |  | 8030 | 29 | Code::CodeKey { rung: Decl, file: packages/lib/schemaValidation.ts, decl: 4, sub: 0, line: 29 } |  |  | 0.610 |
| walker |  | 8084 | 54 | Code::CodeKey { rung: Decl, file: packages/lib/schemaValidation.ts, decl: 10, sub: 0, line: 119 } |  |  | 0.610 |
| walker |  | 8195 | 111 | Code::CodeKey { rung: Names, file: packages/router/collections.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.611 |
| ns | 8246 |  | 281 | Modal, link-view, preservation and input-picker component subdirectories | 6.3 | 6.2 | 0.595 |
| walker |  | 8275 | 80 | Code::CodeKey { rung: Decl, file: packages/router/collections.tsx, decl: 4, sub: 0, line: 180 } |  |  | 0.595 |
| walker |  | 8384 | 109 | Code::CodeKey { rung: Decl, file: packages/router/config.tsx, decl: 1, sub: 0, line: 4 } |  |  | 0.595 |
| walker |  | 8402 | 18 | Code::CodeKey { rung: Body, file: packages/router/config.tsx, decl: 3, sub: 0, line: 44 } |  |  | 0.595 |
| ns | 8473 |  | 227 | Web hooks, layouts, stores, ambient types, email templates, one-off migration scripts and the Playwright suite | 6.4 |  | 0.602 |
| walker |  | 8499 | 97 | Fs::DirListing { dir: apps/web/lib/api } |  |  | 0.624 |
| walker |  | 8512 | 13 | Fs::DirListing { dir: apps/web/lib/api/archives } |  |  | 0.624 |
| walker |  | 8529 | 17 | Fs::DirListing { dir: apps/web/lib/api/preserved } |  |  | 0.624 |
| walker |  | 8562 | 33 | Fs::DirListing { dir: apps/web/lib/api/stripe } |  |  | 0.627 |
| walker |  | 8598 | 36 | Fs::DirListing { dir: apps/web/lib/api/controllers } |  |  | 0.640 |
| walker |  | 8603 | 5 | Fs::DirListing { dir: apps/web/lib/api/controllers/search } |  |  | 0.640 |
| walker |  | 8608 | 5 | Fs::DirListing { dir: apps/web/lib/api/controllers/session } |  |  | 0.641 |
| walker |  | 8614 | 6 | Fs::DirListing { dir: apps/web/lib/api/controllers/worker } |  |  | 0.641 |
| walker |  | 8623 | 9 | Fs::DirListing { dir: apps/web/lib/api/controllers/public } |  |  | 0.641 |
| walker |  | 8627 | 4 | Fs::DirListing { dir: apps/web/lib/api/controllers/public/links } |  |  | 0.642 |
| walker |  | 8633 | 6 | Fs::DirListing { dir: apps/web/lib/api/controllers/public/collections } |  |  | 0.642 |
| walker |  | 8639 | 6 | Fs::DirListing { dir: apps/web/lib/api/controllers/public/users } |  |  | 0.642 |
| walker |  | 8646 | 7 | Fs::DirListing { dir: apps/web/lib/api/controllers/public/links/linkId } |  |  | 0.643 |
| walker |  | 8660 | 14 | Fs::DirListing { dir: apps/web/lib/api/controllers/collections } |  |  | 0.643 |
| walker |  | 8674 | 14 | Fs::DirListing { dir: apps/web/lib/api/controllers/highlights } |  |  | 0.644 |
| walker |  | 8688 | 14 | Fs::DirListing { dir: apps/web/lib/api/controllers/tokens } |  |  | 0.645 |
| walker |  | 8695 | 7 | Fs::DirListing { dir: apps/web/lib/api/controllers/tokens/tokenId } |  |  | 0.646 |
| walker |  | 8709 | 14 | Fs::DirListing { dir: apps/web/lib/api/controllers/users } |  |  | 0.648 |
| ns | 8713 |  | 240 | verifyUser — the guard chain every authenticated route runs first | 7.1 |  | 0.639 |
| walker |  | 8726 | 17 | Fs::DirListing { dir: apps/web/lib/api/controllers/links } |  |  | 0.640 |
| walker |  | 8738 | 12 | Fs::DirListing { dir: apps/web/lib/api/controllers/links/bulk } |  |  | 0.641 |
| walker |  | 8758 | 20 | Fs::DirListing { dir: apps/web/lib/api/controllers/dashboard } |  |  | 0.644 |
| walker |  | 8785 | 27 | Fs::DirListing { dir: apps/web/lib/api/controllers/tags } |  |  | 0.648 |
| walker |  | 8806 | 21 | Fs::DirListing { dir: apps/web/lib/api/controllers/collections/collectionId } |  |  | 0.651 |
| walker |  | 8827 | 21 | Fs::DirListing { dir: apps/web/lib/api/controllers/tags/tagId } |  |  | 0.654 |
| walker |  | 8851 | 24 | Fs::DirListing { dir: apps/web/lib/api/controllers/links/linkId } |  |  | 0.659 |
| walker |  | 8857 | 6 | Fs::DirListing { dir: apps/web/lib/api/controllers/links/linkId/highlight } |  |  | 0.661 |
| walker |  | 8884 | 27 | Fs::DirListing { dir: apps/web/lib/api/controllers/users/userId } |  |  | 0.665 |
| ns | 8948 |  | 235 | docker-compose.yml — the three-container deployment | 7.2 |  | 0.657 |
| walker |  | 9003 | 119 | Code::CodeKey { rung: Decl, file: packages/lib/getPreservedFormatUrl.ts, decl: 1, sub: 0, line: 3 } |  |  | 0.657 |
| walker |  | 9072 | 69 | Fs::DirListing { dir: apps/web/pages/api/v1 } |  |  | 0.676 |
| walker |  | 9076 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/dashboard } |  |  | 0.676 |
| walker |  | 9080 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/getFavicon } |  |  | 0.676 |
| walker |  | 9084 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/logins } |  |  | 0.676 |
| walker |  | 9088 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/migration } |  |  | 0.677 |
| walker |  | 9092 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/payment } |  |  | 0.677 |
| walker |  | 9096 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/search } |  |  | 0.677 |
| walker |  | 9100 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/session } |  |  | 0.677 |
| walker |  | 9104 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/webhook } |  |  | 0.678 |
| walker |  | 9110 | 6 | Fs::DirListing { dir: apps/web/pages/api/v1/avatar } |  |  | 0.678 |
| walker |  | 9119 | 9 | Fs::DirListing { dir: apps/web/pages/api/v1/config } |  |  | 0.679 |
| walker |  | 9128 | 9 | Fs::DirListing { dir: apps/web/pages/api/v1/public } |  |  | 0.679 |
| walker |  | 9134 | 6 | Fs::DirListing { dir: apps/web/pages/api/v1/public/links } |  |  | 0.680 |
| walker |  | 9140 | 6 | Fs::DirListing { dir: apps/web/pages/api/v1/public/users } |  |  | 0.681 |
| walker |  | 9149 | 9 | Fs::DirListing { dir: apps/web/pages/api/v1/worker } |  |  | 0.682 |
| walker |  | 9159 | 10 | Fs::DirListing { dir: apps/web/pages/api/v1/collections } |  |  | 0.682 |
| ns | 9162 |  | 214 | .env.sample — the required variables and the complete list of section headings | 7.3 |  | 0.679 |
| walker |  | 9169 | 10 | Fs::DirListing { dir: apps/web/pages/api/v1/highlights } |  |  | 0.679 |
| walker |  | 9179 | 10 | Fs::DirListing { dir: apps/web/pages/api/v1/rss } |  |  | 0.681 |
| walker |  | 9189 | 10 | Fs::DirListing { dir: apps/web/pages/api/v1/tokens } |  |  | 0.682 |
| walker |  | 9201 | 12 | Fs::DirListing { dir: apps/web/pages/api/v1/links } |  |  | 0.683 |
| walker |  | 9205 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/links/archive } |  |  | 0.684 |
| walker |  | 9218 | 13 | Fs::DirListing { dir: apps/web/pages/api/v1/users } |  |  | 0.686 |
| walker |  | 9227 | 9 | Fs::DirListing { dir: apps/web/pages/api/v1/users/[id] } |  |  | 0.688 |
| walker |  | 9241 | 14 | Fs::DirListing { dir: apps/web/pages/api/v1/tags } |  |  | 0.691 |
| walker |  | 9251 | 10 | Fs::DirListing { dir: apps/web/pages/api/v1/links/[id] } |  |  | 0.694 |
| walker |  | 9255 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/links/[id]/archive } |  |  | 0.696 |
| walker |  | 9259 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/links/[id]/highlights } |  |  | 0.697 |
| walker |  | 9271 | 12 | Fs::DirListing { dir: apps/web/pages/api/v1/public/collections } |  |  | 0.700 |
| walker |  | 9275 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/public/collections/links } |  |  | 0.701 |
| walker |  | 9279 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/public/collections/tags } |  |  | 0.703 |
| walker |  | 9297 | 18 | Fs::DirListing { dir: apps/web/pages/api/v1/preserved } |  |  | 0.707 |
| walker |  | 9316 | 19 | Fs::DirListing { dir: apps/web/pages/api/v1/archives } |  |  | 0.710 |
| walker |  | 9338 | 22 | Fs::DirListing { dir: apps/web/pages/api/v1/auth } |  |  | 0.715 |
| ns | 9341 |  | 179 | The head of the optional tuning-variable block | 7.4 | 7.3 | 0.708 |
| walker |  | 9386 | 48 | Fs::DirListing { dir: apps/web/lib/api/controllers/migration } |  |  | 0.716 |
| ns | 9473 |  | 132 | CI workflows, GitHub templates, the patch-package patch, and every translated locale | 7.5 |  | 0.722 |
| ns | 9689 |  | 216 | apps/mobile root and the complete Expo Router screen tree | 8.1 |  | 0.729 |
| walker |  | 9696 | 310 | Plaintext::Whole { file: docker-compose.yml } |  |  | 0.742 |
| walker |  | 9718 | 22 | Code::CodeKey { rung: Body, file: packages/lib/utils.ts, decl: 2, sub: 0, line: 13 } |  |  | 0.742 |
| walker |  | 9741 | 23 | Code::CodeKey { rung: Body, file: packages/lib/formatStats.ts, decl: 1, sub: 0, line: 4 } |  |  | 0.742 |
| ns | 9891 |  | 202 | Mobile components, stores and query-cache modules | 8.2 | 8.1 | 0.747 |

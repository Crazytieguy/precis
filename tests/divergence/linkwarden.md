Score(3000)=0.694 I=0.890 C=0.541 ns_rows≤3K=15/51 grid(1000/1442/2080/3000/4327/6240/9000)=0.633/0.711/0.685/0.694/0.646/0.653/0.784

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 75 |  | 75 | Product identity: name, tagline, one-sentence definition | 1.1 |  | 0.000 |
| walker |  | 79 | 79 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 88 | 9 | Fs::DirListing { dir: apps } |  |  | 0.000 |
| walker |  | 99 | 11 | Fs::DirListing { dir: patches } |  |  | 0.000 |
| walker |  | 114 | 15 | Fs::DirListing { dir: packages } |  |  | 0.000 |
| walker |  | 137 | 23 | Fs::DirListing { dir: packages/prisma } |  |  | 0.000 |
| walker |  | 146 | 9 | Fs::DirListing { dir: packages/prisma/client } |  |  | 0.000 |
| ns | 154 |  | 79 | Complete repository root listing | 1.2 |  | 0.724 |
| walker |  | 172 | 26 | Fs::DirListing { dir: apps/worker } |  |  | 0.725 |
| walker |  | 177 | 5 | Fs::DirListing { dir: apps/worker/templates } |  |  | 0.725 |
| ns | 178 |  | 24 | The three apps and five shared packages | 1.3 |  | 0.712 |
| walker |  | 181 | 4 | Fs::DirListing { dir: .vscode } |  |  | 0.712 |
| walker |  | 194 | 13 | Fs::DirListing { dir: packages/types } |  |  | 0.712 |
| walker |  | 237 | 43 | Json::Identity { file: package.json } |  |  | 0.714 |
| ns | 259 |  | 81 | Complete apps/web listing | 1.4 |  | 0.534 |
| walker |  | 270 | 33 | Fs::DirListing { dir: assets } |  |  | 0.534 |
| walker |  | 275 | 5 | Fs::DirListing { dir: .devcontainer } |  |  | 0.534 |
| ns | 321 |  | 62 | Workspace globs in the root package.json | 1.5 |  | 0.508 |
| walker |  | 329 | 54 | Fs::DirListing { dir: packages/filesystem } |  |  | 0.508 |
| walker |  | 363 | 34 | Json::Entry { file: package.json } |  |  | 0.559 |
| ns | 460 |  | 139 | Workspace package names — every `name` field under apps/ and packages/ | 1.6 |  | 0.518 |
| walker |  | 500 | 137 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.717 |
| ns | 674 |  | 214 | Root scripts: how you run web, worker and both together | 1.7 |  | 0.664 |
| walker |  | 720 | 220 | Prisma::Toc { file: packages/prisma/schema.prisma } |  |  | 0.674 |
| walker |  | 756 | 36 | Fs::DirListing { dir: apps/worker/workers } |  |  | 0.675 |
| walker |  | 828 | 72 | Fs::DirListing { dir: packages/router } |  |  | 0.676 |
| ns | 837 |  | 163 | Root scripts: prisma, format, test, coverage, postinstall | 1.8 |  | 0.633 |
| walker |  | 908 | 80 | Fs::DirListing { dir: apps/mobile } |  |  | 0.633 |
| walker |  | 912 | 4 | Fs::DirListing { dir: apps/mobile/styles } |  |  | 0.633 |
| walker |  | 918 | 6 | Fs::DirListing { dir: apps/mobile/assets } |  |  | 0.633 |
| walker |  | 954 | 36 | Fs::DirListing { dir: apps/mobile/app } |  |  | 0.633 |
| walker |  | 962 | 8 | Fs::DirListing { dir: apps/mobile/plugins } |  |  | 0.633 |
| walker |  | 968 | 6 | Fs::DirListing { dir: apps/mobile/app/links } |  |  | 0.633 |
| walker |  | 974 | 6 | Fs::DirListing { dir: apps/mobile/assets/fonts } |  |  | 0.633 |
| walker |  | 985 | 11 | Fs::DirListing { dir: apps/mobile/types } |  |  | 0.633 |
| walker |  | 997 | 12 | Fs::DirListing { dir: apps/mobile/store } |  |  | 0.633 |
| walker |  | 1020 | 23 | Fs::DirListing { dir: apps/mobile/lib } |  |  | 0.634 |
| walker |  | 1041 | 21 | Fs::DirListing { dir: apps/mobile/app/(tabs) } |  |  | 0.634 |
| walker |  | 1052 | 11 | Fs::DirListing { dir: apps/mobile/app/(tabs)/links } |  |  | 0.634 |
| walker |  | 1069 | 17 | Fs::DirListing { dir: apps/mobile/app/(tabs)/collections } |  |  | 0.635 |
| ns | 1082 |  | 245 | Feature list, first half (preservation, reading, organisation, sharing) | 1.9 |  | 0.586 |
| walker |  | 1086 | 17 | Fs::DirListing { dir: apps/mobile/app/(tabs)/dashboard } |  |  | 0.587 |
| walker |  | 1103 | 17 | Fs::DirListing { dir: apps/mobile/app/(tabs)/settings } |  |  | 0.587 |
| walker |  | 1120 | 17 | Fs::DirListing { dir: apps/mobile/app/(tabs)/tags } |  |  | 0.587 |
| walker |  | 1144 | 24 | Fs::DirListing { dir: apps/mobile/assets/images } |  |  | 0.587 |
| walker |  | 1225 | 81 | Fs::DirListing { dir: apps/web } |  |  | 0.760 |
| walker |  | 1243 | 18 | Fs::DirListing { dir: apps/web/e2e } |  |  | 0.760 |
| walker |  | 1247 | 4 | Fs::DirListing { dir: apps/web/styles } |  |  | 0.760 |
| walker |  | 1251 | 4 | Fs::DirListing { dir: apps/web/e2e/data } |  |  | 0.760 |
| walker |  | 1260 | 9 | Fs::DirListing { dir: apps/web/store } |  |  | 0.760 |
| walker |  | 1263 | 3 | Fs::DirListing { dir: apps/web/scripts } |  |  | 0.760 |
| ns | 1271 |  | 189 | Feature list, second half (sync, SSO, API keys, i18n, RSS, uploads) | 1.10 |  | 0.709 |
| walker |  | 1275 | 12 | Fs::DirListing { dir: apps/web/types } |  |  | 0.709 |
| walker |  | 1288 | 13 | Fs::DirListing { dir: apps/web/lib } |  |  | 0.709 |
| walker |  | 1374 | 86 | Fs::DirListing { dir: apps/web/pages } |  |  | 0.710 |
| walker |  | 1385 | 11 | Fs::DirListing { dir: apps/web/pages/collections } |  |  | 0.710 |
| walker |  | 1396 | 11 | Fs::DirListing { dir: apps/web/pages/tags } |  |  | 0.710 |
| walker |  | 1412 | 16 | Fs::DirListing { dir: apps/web/pages/links } |  |  | 0.710 |
| walker |  | 1431 | 19 | Fs::DirListing { dir: apps/web/pages/admin } |  |  | 0.711 |
| walker |  | 1437 | 6 | Fs::DirListing { dir: apps/web/pages/preserved } |  |  | 0.711 |
| walker |  | 1493 | 56 | Fs::DirListing { dir: apps/web/pages/settings } |  |  | 0.712 |
| walker |  | 1501 | 8 | Fs::DirListing { dir: apps/web/pages/api } |  |  | 0.712 |
| walker |  | 1504 | 3 | Fs::DirListing { dir: apps/web/pages/api/v2 } |  |  | 0.712 |
| walker |  | 1508 | 4 | Fs::DirListing { dir: apps/web/pages/api/v2/dashboard } |  |  | 0.712 |
| walker |  | 1517 | 9 | Fs::DirListing { dir: apps/web/pages/public } |  |  | 0.712 |
| walker |  | 1522 | 5 | Fs::DirListing { dir: apps/web/pages/public/collections } |  |  | 0.713 |
| walker |  | 1532 | 10 | Fs::DirListing { dir: apps/web/pages/public/collections/[id] } |  |  | 0.713 |
| walker |  | 1538 | 6 | Fs::DirListing { dir: apps/web/pages/public/links } |  |  | 0.713 |
| ns | 1539 |  | 268 | schema.prisma: datasource/generator plus every model and enum header, bodies elided | 2.1 |  | 0.654 |
| walker |  | 1544 | 6 | Fs::DirListing { dir: apps/web/pages/public/preserved } |  |  | 0.654 |
| walker |  | 1556 | 12 | Fs::DirListing { dir: apps/web/pages/auth } |  |  | 0.655 |
| walker |  | 1577 | 21 | Fs::DirListing { dir: apps/web/templates } |  |  | 0.655 |
| walker |  | 1583 | 6 | Fs::DirListing { dir: apps/web/e2e/tests } |  |  | 0.655 |
| walker |  | 1588 | 5 | Fs::DirListing { dir: apps/web/e2e/tests/public } |  |  | 0.655 |
| walker |  | 1612 | 24 | Fs::DirListing { dir: apps/web/layouts } |  |  | 0.655 |
| walker |  | 1632 | 20 | Fs::DirListing { dir: apps/web/lib/shared } |  |  | 0.656 |
| walker |  | 1642 | 10 | Fs::DirListing { dir: apps/web/e2e/tests/global } |  |  | 0.656 |
| walker |  | 1666 | 24 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 77 } |  |  | 0.660 |
| ns | 1697 |  | 158 | All five schema enums, fully expanded | 2.2 | 2.1 | 0.616 |
| walker |  | 1715 | 49 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 90 } |  |  | 0.634 |
| walker |  | 1744 | 29 | Fs::DirListing { dir: .github } |  |  | 0.634 |
| walker |  | 1765 | 21 | Fs::DirListing { dir: .github/workflows } |  |  | 0.634 |
| walker |  | 1791 | 26 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 304 } |  |  | 0.650 |
| walker |  | 1826 | 35 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 83 } |  |  | 0.677 |
| walker |  | 2036 | 210 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 166 } |  |  | 0.683 |
| ns | 2083 |  | 386 | `Link` model — every field | 2.3 | 2.1 | 0.650 |
| walker |  | 2215 | 179 | Prisma::DeclTail { file: packages/prisma/schema.prisma, start_line: 166, tail_start_line: 182 } |  |  | 0.726 |
| walker |  | 2254 | 39 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 289 } |  |  | 0.755 |
| walker |  | 2269 | 15 | Fs::DirListing { dir: apps/web/scripts/migration } |  |  | 0.755 |
| walker |  | 2273 | 4 | Fs::DirListing { dir: apps/web/scripts/migration/v2.6.1 } |  |  | 0.756 |
| walker |  | 2382 | 109 | Fs::DirListing { dir: packages/lib } |  |  | 0.759 |
| ns | 2396 |  | 313 | `User` model — identity, relations and subscription linkage | 2.4 | 2.1 | 0.712 |
| walker |  | 2439 | 57 | Fs::DirListing { dir: apps/worker/lib } |  |  | 0.714 |
| walker |  | 2498 | 59 | Fs::DirListing { dir: apps/mobile/components } |  |  | 0.714 |
| walker |  | 2524 | 26 | Fs::DirListing { dir: apps/mobile/components/Formats } |  |  | 0.714 |
| walker |  | 2556 | 32 | Fs::DirListing { dir: apps/mobile/components/ActionSheets } |  |  | 0.715 |
| walker |  | 2618 | 62 | Fs::DirListing { dir: apps/web/hooks } |  |  | 0.716 |
| walker |  | 2635 | 17 | Fs::DirListing { dir: apps/web/e2e/fixtures } |  |  | 0.716 |
| walker |  | 2648 | 13 | Fs::DirListing { dir: apps/web/e2e/fixtures/base } |  |  | 0.716 |
| ns | 2738 |  | 342 | `User` model — preference and archival-toggle fields | 2.5 | 2.4 | 0.680 |
| walker |  | 2820 | 172 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 10 } |  |  | 0.682 |
| walker |  | 2902 | 82 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 108 } |  |  | 0.685 |
| ns | 3055 |  | 317 | `Collection` model — every field, including the self-relation | 2.6 | 2.1 | 0.653 |
| walker |  | 3187 | 285 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 28 } |  |  | 0.708 |
| walker |  | 3324 | 137 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.708 |
| walker |  | 3350 | 26 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.708 |
| ns | 3436 |  | 381 | `UsersAndCollections` join model (the permission bits) and `Tag` model | 2.7 | 2.1 | 0.668 |
| walker |  | 3505 | 155 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.668 |
| walker |  | 3534 | 29 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.668 |
| walker |  | 3576 | 42 | Fs::DirListing { dir: apps/worker/lib/preservationScheme } |  |  | 0.669 |
| ns | 3594 |  | 158 | `AccessToken` model | 2.8 | 2.1 | 0.656 |
| ns | 3617 |  | 23 | Complete packages/prisma listing | 2.9 |  | 0.661 |
| ns | 3733 |  | 116 | The process-wide `prisma` client singleton | 2.10 |  | 0.649 |
| walker |  | 3794 | 218 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 200 } |  |  | 0.674 |
| ns | 3817 |  | 84 | API version directories: every resource under pages/api/v1 (and the single v2 route) | 3.1 |  | 0.647 |
| walker |  | 3859 | 65 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 118 } |  |  | 0.649 |
| ns | 3885 |  | 68 | Route files for links, collections, tags and highlights | 3.2 |  | 0.632 |
| walker |  | 3955 | 96 | Json::Runtime { file: package.json } |  |  | 0.632 |
| ns | 3962 |  | 77 | Route files for auth, session, users, tokens, config, avatar and logins | 3.3 |  | 0.617 |
| walker |  | 3982 | 27 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 5 } |  |  | 0.631 |
| ns | 4042 |  | 80 | Route files for archives, preserved, search, dashboard, rss, migration, payment, webhook, worker, getFavicon | 3.4 |  | 0.616 |
| walker |  | 4051 | 69 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 296 } |  |  | 0.618 |
| ns | 4083 |  | 41 | The unauthenticated `public/` route subtree | 3.5 |  | 0.609 |
| walker |  | 4130 | 79 | Fs::DirListing { dir: apps/web/public } |  |  | 0.609 |
| walker |  | 4140 | 10 | Fs::DirListing { dir: apps/web/public/screenshots } |  |  | 0.609 |
| walker |  | 4309 | 169 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 151 } |  |  | 0.646 |
| ns | 4359 |  | 276 | The route-handler pattern, read from pages/api/v1/links/index.ts | 3.6 |  | 0.628 |
| ns | 4568 |  | 209 | apps/web/lib: the server helper layer and its client/shared siblings | 3.7 |  | 0.601 |
| ns | 4667 |  | 99 | The controller tree: every resource directory under lib/api/controllers | 3.8 |  | 0.586 |
| walker |  | 4744 | 435 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: true } |  |  | 0.633 |
| ns | 4823 |  | 156 | Controller files for links, collections, tags and highlights | 3.9 |  | 0.614 |
| walker |  | 4917 | 173 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 260 } |  |  | 0.615 |
| ns | 5001 |  | 178 | Controller files for users, tokens, session, search, dashboard, worker, migration and public access | 3.10 |  | 0.595 |
| walker |  | 5047 | 130 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 248 } |  |  | 0.597 |
| ns | 5249 |  | 248 | Complete listings of packages/types, packages/lib, packages/filesystem and packages/router | 4.1 |  | 0.626 |
| walker |  | 5367 | 320 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 126 } |  |  | 0.656 |
| walker |  | 5389 | 22 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.657 |
| walker |  | 5550 | 161 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 234 } |  |  | 0.672 |
| walker |  | 5599 | 49 | Fs::DirListing { dir: apps/web/public/locales } |  |  | 0.673 |
| walker |  | 5603 | 4 | Fs::DirListing { dir: apps/web/public/locales/de } |  |  | 0.673 |
| walker |  | 5607 | 4 | Fs::DirListing { dir: apps/web/public/locales/en } |  |  | 0.673 |
| walker |  | 5611 | 4 | Fs::DirListing { dir: apps/web/public/locales/es } |  |  | 0.673 |
| walker |  | 5615 | 4 | Fs::DirListing { dir: apps/web/public/locales/fr } |  |  | 0.673 |
| walker |  | 5619 | 4 | Fs::DirListing { dir: apps/web/public/locales/it } |  |  | 0.673 |
| walker |  | 5623 | 4 | Fs::DirListing { dir: apps/web/public/locales/ja } |  |  | 0.673 |
| walker |  | 5627 | 4 | Fs::DirListing { dir: apps/web/public/locales/nl } |  |  | 0.673 |
| walker |  | 5631 | 4 | Fs::DirListing { dir: apps/web/public/locales/pl } |  |  | 0.673 |
| walker |  | 5635 | 4 | Fs::DirListing { dir: apps/web/public/locales/pt-BR } |  |  | 0.673 |
| walker |  | 5639 | 4 | Fs::DirListing { dir: apps/web/public/locales/ro } |  |  | 0.673 |
| walker |  | 5643 | 4 | Fs::DirListing { dir: apps/web/public/locales/ru } |  |  | 0.673 |
| walker |  | 5647 | 4 | Fs::DirListing { dir: apps/web/public/locales/tr } |  |  | 0.655 |
| ns | 5647 |  | 398 | packages/types/global.ts — every exported type, interface and enum declaration | 4.2 |  | 0.655 |
| walker |  | 5651 | 4 | Fs::DirListing { dir: apps/web/public/locales/uk } |  |  | 0.655 |
| walker |  | 5655 | 4 | Fs::DirListing { dir: apps/web/public/locales/zh } |  |  | 0.655 |
| walker |  | 5659 | 4 | Fs::DirListing { dir: apps/web/public/locales/zh-TW } |  |  | 0.655 |
| walker |  | 5709 | 50 | Fs::DirListing { dir: apps/mobile/components/ui } |  |  | 0.656 |
| ns | 5793 |  | 146 | `ArchivedFormat`, `LinkType` and `TokenExpiry` variants | 4.3 | 4.2 | 0.648 |
| walker |  | 5877 | 168 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 275 } |  |  | 0.649 |
| ns | 5945 |  | 152 | `ViewMode`, `Sort` and `TagSort` variants | 4.4 | 4.2 | 0.642 |
| walker |  | 6047 | 170 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 220 } |  |  | 0.643 |
| ns | 6285 |  | 340 | packages/lib/schemaValidation.ts — every exported zod schema constant | 4.5 |  | 0.631 |
| walker |  | 6420 | 373 | Prisma::DeclTail { file: packages/prisma/schema.prisma, start_line: 28, tail_start_line: 51 } |  |  | 0.663 |
| walker |  | 6436 | 16 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 1 } |  |  | 0.669 |
| walker |  | 6537 | 101 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 99 } |  |  | 0.671 |
| ns | 6542 |  | 257 | packages/router — the links, collections and tags hook exports | 4.6 |  | 0.655 |
| walker |  | 6568 | 31 | Code::CodeKey { rung: Names, file: apps/worker/worker.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.655 |
| walker |  | 6584 | 16 | Code::CodeKey { rung: Decl, file: apps/worker/worker.ts, decl: 1, sub: 0, line: 8 } |  |  | 0.655 |
| walker |  | 6719 | 135 | Json::Dependencies { file: package.json } |  |  | 0.655 |
| ns | 6726 |  | 184 | packages/router — export lines of the remaining ten hook modules | 4.7 | 4.6 | 0.650 |
| walker |  | 6819 | 100 | Code::CodeKey { rung: Names, file: packages/filesystem/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.650 |
| walker |  | 6831 | 12 | Code::CodeKey { rung: Names, file: packages/prisma/client/index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.650 |
| walker |  | 6843 | 12 | Code::CodeKey { rung: Names, file: packages/prisma/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.650 |
| ns | 6892 |  | 166 | Complete apps/worker listing, including every job and every preservation handler | 5.1 |  | 0.663 |
| walker |  | 6919 | 76 | Code::CodeKey { rung: Decl, file: packages/prisma/index.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.669 |
| walker |  | 6957 | 38 | Code::CodeKey { rung: Names, file: apps/worker/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.669 |
| walker |  | 6983 | 26 | Code::CodeKey { rung: Names, file: apps/web/e2e/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.669 |
| walker |  | 6997 | 14 | Code::CodeKey { rung: Names, file: apps/web/pages/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.669 |
| walker |  | 7011 | 14 | Code::CodeKey { rung: Names, file: apps/web/pages/settings/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.669 |
| walker |  | 7090 | 79 | Fs::DirListing { dir: apps/web/lib/client } |  |  | 0.678 |
| walker |  | 7105 | 15 | Code::CodeKey { rung: Names, file: apps/mobile/app/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.678 |
| ns | 7161 |  | 269 | worker.ts — the whole scheduler entry point | 5.2 |  | 0.667 |
| walker |  | 7168 | 63 | Code::CodeKey { rung: Names, file: apps/web/pages/collections/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.667 |
| walker |  | 7183 | 15 | Code::CodeKey { rung: Body, file: apps/web/pages/collections/index.tsx, decl: 2, sub: 0, line: 153 } |  |  | 0.667 |
| walker |  | 7246 | 63 | Code::CodeKey { rung: Names, file: apps/web/pages/links/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.667 |
| walker |  | 7261 | 15 | Code::CodeKey { rung: Body, file: apps/web/pages/links/index.tsx, decl: 2, sub: 0, line: 70 } |  |  | 0.667 |
| walker |  | 7326 | 65 | Code::CodeKey { rung: Names, file: apps/web/pages/tags/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.667 |
| walker |  | 7341 | 15 | Code::CodeKey { rung: Body, file: apps/web/pages/tags/index.tsx, decl: 2, sub: 0, line: 274 } |  |  | 0.667 |
| ns | 7354 |  | 193 | archiveHandler's signature and its SSRF / skip-preservation guard | 5.3 |  | 0.658 |
| walker |  | 7476 | 135 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.658 |
| walker |  | 7573 | 97 | Fs::DirListing { dir: apps/web/lib/api } |  |  | 0.684 |
| walker |  | 7586 | 13 | Fs::DirListing { dir: apps/web/lib/api/archives } |  |  | 0.684 |
| walker |  | 7603 | 17 | Fs::DirListing { dir: apps/web/lib/api/preserved } |  |  | 0.685 |
| ns | 7607 |  | 253 | Every UI page route under apps/web/pages | 6.1 |  | 0.700 |
| walker |  | 7636 | 33 | Fs::DirListing { dir: apps/web/lib/api/stripe } |  |  | 0.703 |
| walker |  | 7672 | 36 | Fs::DirListing { dir: apps/web/lib/api/controllers } |  |  | 0.718 |
| walker |  | 7677 | 5 | Fs::DirListing { dir: apps/web/lib/api/controllers/search } |  |  | 0.718 |
| walker |  | 7682 | 5 | Fs::DirListing { dir: apps/web/lib/api/controllers/session } |  |  | 0.718 |
| walker |  | 7688 | 6 | Fs::DirListing { dir: apps/web/lib/api/controllers/worker } |  |  | 0.718 |
| walker |  | 7697 | 9 | Fs::DirListing { dir: apps/web/lib/api/controllers/public } |  |  | 0.719 |
| walker |  | 7701 | 4 | Fs::DirListing { dir: apps/web/lib/api/controllers/public/links } |  |  | 0.719 |
| walker |  | 7707 | 6 | Fs::DirListing { dir: apps/web/lib/api/controllers/public/collections } |  |  | 0.720 |
| walker |  | 7713 | 6 | Fs::DirListing { dir: apps/web/lib/api/controllers/public/users } |  |  | 0.720 |
| walker |  | 7720 | 7 | Fs::DirListing { dir: apps/web/lib/api/controllers/public/links/linkId } |  |  | 0.721 |
| walker |  | 7734 | 14 | Fs::DirListing { dir: apps/web/lib/api/controllers/collections } |  |  | 0.721 |
| walker |  | 7748 | 14 | Fs::DirListing { dir: apps/web/lib/api/controllers/highlights } |  |  | 0.722 |
| walker |  | 7762 | 14 | Fs::DirListing { dir: apps/web/lib/api/controllers/tokens } |  |  | 0.723 |
| walker |  | 7769 | 7 | Fs::DirListing { dir: apps/web/lib/api/controllers/tokens/tokenId } |  |  | 0.724 |
| walker |  | 7783 | 14 | Fs::DirListing { dir: apps/web/lib/api/controllers/users } |  |  | 0.727 |
| walker |  | 7800 | 17 | Fs::DirListing { dir: apps/web/lib/api/controllers/links } |  |  | 0.728 |
| walker |  | 7812 | 12 | Fs::DirListing { dir: apps/web/lib/api/controllers/links/bulk } |  |  | 0.730 |
| walker |  | 7832 | 20 | Fs::DirListing { dir: apps/web/lib/api/controllers/dashboard } |  |  | 0.732 |
| walker |  | 7859 | 27 | Fs::DirListing { dir: apps/web/lib/api/controllers/tags } |  |  | 0.737 |
| walker |  | 7880 | 21 | Fs::DirListing { dir: apps/web/lib/api/controllers/collections/collectionId } |  |  | 0.740 |
| walker |  | 7901 | 21 | Fs::DirListing { dir: apps/web/lib/api/controllers/tags/tagId } |  |  | 0.744 |
| walker |  | 7925 | 24 | Fs::DirListing { dir: apps/web/lib/api/controllers/links/linkId } |  |  | 0.749 |
| walker |  | 7931 | 6 | Fs::DirListing { dir: apps/web/lib/api/controllers/links/linkId/highlight } |  |  | 0.751 |
| walker |  | 7958 | 27 | Fs::DirListing { dir: apps/web/lib/api/controllers/users/userId } |  |  | 0.756 |
| ns | 7965 |  | 358 | The flat components/ directory and the ui/ primitives | 6.2 |  | 0.726 |
| walker |  | 7995 | 37 | Code::CodeKey { rung: Names, file: apps/web/pages/admin/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.726 |
| walker |  | 8003 | 8 | Code::CodeKey { rung: Body, file: apps/web/pages/admin/index.tsx, decl: 2, sub: 0, line: 12 } |  |  | 0.726 |
| walker |  | 8072 | 69 | Fs::DirListing { dir: apps/web/pages/api/v1 } |  |  | 0.747 |
| walker |  | 8076 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/dashboard } |  |  | 0.747 |
| walker |  | 8080 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/getFavicon } |  |  | 0.747 |
| walker |  | 8084 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/logins } |  |  | 0.747 |
| walker |  | 8088 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/migration } |  |  | 0.747 |
| walker |  | 8092 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/payment } |  |  | 0.748 |
| walker |  | 8096 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/search } |  |  | 0.748 |
| walker |  | 8100 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/session } |  |  | 0.748 |
| walker |  | 8104 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/webhook } |  |  | 0.749 |
| walker |  | 8113 | 9 | Fs::DirListing { dir: apps/web/pages/api/v1/config } |  |  | 0.749 |
| walker |  | 8122 | 9 | Fs::DirListing { dir: apps/web/pages/api/v1/worker } |  |  | 0.751 |
| walker |  | 8132 | 10 | Fs::DirListing { dir: apps/web/pages/api/v1/collections } |  |  | 0.751 |
| walker |  | 8142 | 10 | Fs::DirListing { dir: apps/web/pages/api/v1/highlights } |  |  | 0.752 |
| walker |  | 8152 | 10 | Fs::DirListing { dir: apps/web/pages/api/v1/rss } |  |  | 0.753 |
| walker |  | 8162 | 10 | Fs::DirListing { dir: apps/web/pages/api/v1/tokens } |  |  | 0.754 |
| walker |  | 8174 | 12 | Fs::DirListing { dir: apps/web/pages/api/v1/links } |  |  | 0.756 |
| walker |  | 8178 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/links/archive } |  |  | 0.757 |
| walker |  | 8188 | 10 | Fs::DirListing { dir: apps/web/pages/api/v1/links/[id] } |  |  | 0.760 |
| walker |  | 8192 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/links/[id]/archive } |  |  | 0.761 |
| walker |  | 8196 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/links/[id]/highlights } |  |  | 0.762 |
| walker |  | 8209 | 13 | Fs::DirListing { dir: apps/web/pages/api/v1/users } |  |  | 0.764 |
| walker |  | 8218 | 9 | Fs::DirListing { dir: apps/web/pages/api/v1/users/[id] } |  |  | 0.766 |
| walker |  | 8232 | 14 | Fs::DirListing { dir: apps/web/pages/api/v1/tags } |  |  | 0.771 |
| ns | 8246 |  | 281 | Modal, link-view, preservation and input-picker component subdirectories | 6.3 | 6.2 | 0.751 |
| walker |  | 8251 | 19 | Fs::DirListing { dir: apps/web/pages/api/v1/archives } |  |  | 0.754 |
| walker |  | 8257 | 6 | Fs::DirListing { dir: apps/web/pages/api/v1/avatar } |  |  | 0.755 |
| walker |  | 8266 | 9 | Fs::DirListing { dir: apps/web/pages/api/v1/public } |  |  | 0.756 |
| walker |  | 8272 | 6 | Fs::DirListing { dir: apps/web/pages/api/v1/public/links } |  |  | 0.757 |
| walker |  | 8278 | 6 | Fs::DirListing { dir: apps/web/pages/api/v1/public/users } |  |  | 0.757 |
| walker |  | 8290 | 12 | Fs::DirListing { dir: apps/web/pages/api/v1/public/collections } |  |  | 0.760 |
| walker |  | 8294 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/public/collections/links } |  |  | 0.762 |
| walker |  | 8298 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/public/collections/tags } |  |  | 0.763 |
| walker |  | 8316 | 18 | Fs::DirListing { dir: apps/web/pages/api/v1/preserved } |  |  | 0.768 |
| walker |  | 8338 | 22 | Fs::DirListing { dir: apps/web/pages/api/v1/auth } |  |  | 0.774 |
| walker |  | 8386 | 48 | Fs::DirListing { dir: apps/web/lib/api/controllers/migration } |  |  | 0.783 |
| ns | 8473 |  | 227 | Web hooks, layouts, stores, ambient types, email templates, one-off migration scripts and the Playwright suite | 6.4 |  | 0.789 |
| walker |  | 8696 | 310 | Plaintext::Whole { file: docker-compose.yml } |  |  | 0.790 |
| ns | 8713 |  | 240 | verifyUser — the guard chain every authenticated route runs first | 7.1 |  | 0.779 |
| walker |  | 8784 | 88 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.779 |
| walker |  | 8888 | 104 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.779 |
| ns | 8948 |  | 235 | docker-compose.yml — the three-container deployment | 7.2 |  | 0.782 |
| ns | 9162 |  | 214 | .env.sample — the required variables and the complete list of section headings | 7.3 |  | 0.773 |
| walker |  | 9265 | 377 | Json::Scripts { file: package.json } |  |  | 0.796 |
| ns | 9341 |  | 179 | The head of the optional tuning-variable block | 7.4 | 7.3 | 0.788 |
| ns | 9473 |  | 132 | CI workflows, GitHub templates, the patch-package patch, and every translated locale | 7.5 |  | 0.792 |
| walker |  | 9582 | 317 | Fs::DirListing { dir: apps/web/components } |  |  | 0.819 |
| walker |  | 9591 | 9 | Fs::DirListing { dir: apps/web/components/LinkViews } |  |  | 0.820 |
| walker |  | 9607 | 16 | Fs::DirListing { dir: apps/web/components/InputSelect } |  |  | 0.820 |
| walker |  | 9633 | 26 | Fs::DirListing { dir: apps/web/components/Preservation } |  |  | 0.821 |
| walker |  | 9674 | 41 | Fs::DirListing { dir: apps/web/components/ui } |  |  | 0.829 |
| ns | 9689 |  | 216 | apps/mobile root and the complete Expo Router screen tree | 8.1 |  | 0.833 |
| walker |  | 9736 | 62 | Fs::DirListing { dir: apps/web/components/LinkViews/LinkComponents } |  |  | 0.837 |
| ns | 9891 |  | 202 | Mobile components, stores and query-cache modules | 8.2 | 8.1 | 0.839 |
| walker |  | 9904 | 168 | Fs::DirListing { dir: apps/web/components/ModalContent } |  |  | 0.857 |
| walker |  | 9947 | 43 | Code::CodeKey { rung: Body, file: apps/web/pages/index.tsx, decl: 1, sub: 0, line: 4 } |  |  | 0.857 |
| walker |  | 9991 | 44 | Code::CodeKey { rung: Body, file: apps/web/pages/settings/index.tsx, decl: 1, sub: 0, line: 4 } |  |  | 0.857 |
| walker |  | 9998 | 7 | Code::CodeKey { rung: Body, file: apps/web/pages/admin/index.tsx, decl: 1, sub: 0, line: 3 } |  |  | 0.857 |

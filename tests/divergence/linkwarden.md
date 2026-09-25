Score(3000)=0.695 I=0.889 C=0.544 ns_rows≤3K=15/51 grid(1000/1442/2080/3000/4327/6240/9000)=0.633/0.711/0.685/0.695/0.646/0.656/0.779

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
| ns | 1271 |  | 189 | Feature list, second half (sync, SSO, API keys, i18n, RSS, uploads) | 1.10 |  | 0.709 |
| walker |  | 1272 | 12 | Fs::DirListing { dir: apps/web/types } |  |  | 0.709 |
| walker |  | 1285 | 13 | Fs::DirListing { dir: apps/web/lib } |  |  | 0.709 |
| walker |  | 1371 | 86 | Fs::DirListing { dir: apps/web/pages } |  |  | 0.710 |
| walker |  | 1382 | 11 | Fs::DirListing { dir: apps/web/pages/collections } |  |  | 0.710 |
| walker |  | 1393 | 11 | Fs::DirListing { dir: apps/web/pages/tags } |  |  | 0.710 |
| walker |  | 1409 | 16 | Fs::DirListing { dir: apps/web/pages/links } |  |  | 0.710 |
| walker |  | 1428 | 19 | Fs::DirListing { dir: apps/web/pages/admin } |  |  | 0.711 |
| walker |  | 1434 | 6 | Fs::DirListing { dir: apps/web/pages/preserved } |  |  | 0.711 |
| walker |  | 1490 | 56 | Fs::DirListing { dir: apps/web/pages/settings } |  |  | 0.712 |
| walker |  | 1498 | 8 | Fs::DirListing { dir: apps/web/pages/api } |  |  | 0.712 |
| walker |  | 1501 | 3 | Fs::DirListing { dir: apps/web/pages/api/v2 } |  |  | 0.712 |
| walker |  | 1505 | 4 | Fs::DirListing { dir: apps/web/pages/api/v2/dashboard } |  |  | 0.712 |
| walker |  | 1514 | 9 | Fs::DirListing { dir: apps/web/pages/public } |  |  | 0.712 |
| walker |  | 1519 | 5 | Fs::DirListing { dir: apps/web/pages/public/collections } |  |  | 0.713 |
| walker |  | 1529 | 10 | Fs::DirListing { dir: apps/web/pages/public/collections/[id] } |  |  | 0.713 |
| walker |  | 1535 | 6 | Fs::DirListing { dir: apps/web/pages/public/links } |  |  | 0.713 |
| ns | 1539 |  | 268 | schema.prisma: datasource/generator plus every model and enum header, bodies elided | 2.1 |  | 0.654 |
| walker |  | 1541 | 6 | Fs::DirListing { dir: apps/web/pages/public/preserved } |  |  | 0.654 |
| walker |  | 1553 | 12 | Fs::DirListing { dir: apps/web/pages/auth } |  |  | 0.655 |
| walker |  | 1574 | 21 | Fs::DirListing { dir: apps/web/templates } |  |  | 0.655 |
| walker |  | 1580 | 6 | Fs::DirListing { dir: apps/web/e2e/tests } |  |  | 0.655 |
| walker |  | 1604 | 24 | Fs::DirListing { dir: apps/web/layouts } |  |  | 0.655 |
| walker |  | 1624 | 20 | Fs::DirListing { dir: apps/web/lib/shared } |  |  | 0.655 |
| walker |  | 1648 | 24 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 77 } |  |  | 0.660 |
| walker |  | 1697 | 49 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 90 } |  |  | 0.634 |
| ns | 1697 |  | 158 | All five schema enums, fully expanded | 2.2 | 2.1 | 0.634 |
| walker |  | 1726 | 29 | Fs::DirListing { dir: .github } |  |  | 0.634 |
| walker |  | 1747 | 21 | Fs::DirListing { dir: .github/workflows } |  |  | 0.634 |
| walker |  | 1773 | 26 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 304 } |  |  | 0.649 |
| walker |  | 1808 | 35 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 83 } |  |  | 0.676 |
| walker |  | 2018 | 210 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 166 } |  |  | 0.683 |
| ns | 2083 |  | 386 | `Link` model — every field | 2.3 | 2.1 | 0.650 |
| walker |  | 2197 | 179 | Prisma::DeclTail { file: packages/prisma/schema.prisma, start_line: 166, tail_start_line: 182 } |  |  | 0.726 |
| walker |  | 2236 | 39 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 289 } |  |  | 0.755 |
| walker |  | 2345 | 109 | Fs::DirListing { dir: packages/lib } |  |  | 0.759 |
| ns | 2396 |  | 313 | `User` model — identity, relations and subscription linkage | 2.4 | 2.1 | 0.712 |
| walker |  | 2402 | 57 | Fs::DirListing { dir: apps/worker/lib } |  |  | 0.713 |
| walker |  | 2461 | 59 | Fs::DirListing { dir: apps/mobile/components } |  |  | 0.713 |
| walker |  | 2487 | 26 | Fs::DirListing { dir: apps/mobile/components/Formats } |  |  | 0.714 |
| walker |  | 2519 | 32 | Fs::DirListing { dir: apps/mobile/components/ActionSheets } |  |  | 0.714 |
| walker |  | 2581 | 62 | Fs::DirListing { dir: apps/web/hooks } |  |  | 0.715 |
| walker |  | 2598 | 17 | Fs::DirListing { dir: apps/web/e2e/fixtures } |  |  | 0.715 |
| walker |  | 2611 | 13 | Fs::DirListing { dir: apps/web/e2e/fixtures/base } |  |  | 0.716 |
| ns | 2738 |  | 342 | `User` model — preference and archival-toggle fields | 2.5 | 2.4 | 0.679 |
| walker |  | 2783 | 172 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 10 } |  |  | 0.681 |
| walker |  | 2865 | 82 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 108 } |  |  | 0.684 |
| ns | 3055 |  | 317 | `Collection` model — every field, including the self-relation | 2.6 | 2.1 | 0.652 |
| walker |  | 3150 | 285 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 28 } |  |  | 0.707 |
| walker |  | 3287 | 137 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.707 |
| walker |  | 3313 | 26 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.707 |
| ns | 3436 |  | 381 | `UsersAndCollections` join model (the permission bits) and `Tag` model | 2.7 | 2.1 | 0.667 |
| walker |  | 3468 | 155 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.667 |
| walker |  | 3497 | 29 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.667 |
| walker |  | 3539 | 42 | Fs::DirListing { dir: apps/worker/lib/preservationScheme } |  |  | 0.668 |
| ns | 3594 |  | 158 | `AccessToken` model | 2.8 | 2.1 | 0.655 |
| ns | 3617 |  | 23 | Complete packages/prisma listing | 2.9 |  | 0.660 |
| ns | 3733 |  | 116 | The process-wide `prisma` client singleton | 2.10 |  | 0.649 |
| walker |  | 3757 | 218 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 200 } |  |  | 0.673 |
| ns | 3817 |  | 84 | API version directories: every resource under pages/api/v1 (and the single v2 route) | 3.1 |  | 0.646 |
| walker |  | 3822 | 65 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 118 } |  |  | 0.648 |
| ns | 3885 |  | 68 | Route files for links, collections, tags and highlights | 3.2 |  | 0.632 |
| walker |  | 3918 | 96 | Json::Runtime { file: package.json } |  |  | 0.632 |
| walker |  | 3945 | 27 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 5 } |  |  | 0.646 |
| ns | 3962 |  | 77 | Route files for auth, session, users, tokens, config, avatar and logins | 3.3 |  | 0.630 |
| walker |  | 4014 | 69 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 296 } |  |  | 0.633 |
| ns | 4042 |  | 80 | Route files for archives, preserved, search, dashboard, rss, migration, payment, webhook, worker, getFavicon | 3.4 |  | 0.617 |
| ns | 4083 |  | 41 | The unauthenticated `public/` route subtree | 3.5 |  | 0.609 |
| walker |  | 4093 | 79 | Fs::DirListing { dir: apps/web/public } |  |  | 0.609 |
| walker |  | 4103 | 10 | Fs::DirListing { dir: apps/web/public/screenshots } |  |  | 0.609 |
| walker |  | 4272 | 169 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 151 } |  |  | 0.645 |
| ns | 4359 |  | 276 | The route-handler pattern, read from pages/api/v1/links/index.ts | 3.6 |  | 0.628 |
| ns | 4568 |  | 209 | apps/web/lib: the server helper layer and its client/shared siblings | 3.7 |  | 0.600 |
| ns | 4667 |  | 99 | The controller tree: every resource directory under lib/api/controllers | 3.8 |  | 0.585 |
| walker |  | 4707 | 435 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: true } |  |  | 0.632 |
| ns | 4823 |  | 156 | Controller files for links, collections, tags and highlights | 3.9 |  | 0.613 |
| walker |  | 4880 | 173 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 260 } |  |  | 0.615 |
| ns | 5001 |  | 178 | Controller files for users, tokens, session, search, dashboard, worker, migration and public access | 3.10 |  | 0.595 |
| walker |  | 5010 | 130 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 248 } |  |  | 0.596 |
| ns | 5249 |  | 248 | Complete listings of packages/types, packages/lib, packages/filesystem and packages/router | 4.1 |  | 0.625 |
| walker |  | 5330 | 320 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 126 } |  |  | 0.656 |
| walker |  | 5352 | 22 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.656 |
| walker |  | 5513 | 161 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 234 } |  |  | 0.671 |
| walker |  | 5562 | 49 | Fs::DirListing { dir: apps/web/public/locales } |  |  | 0.672 |
| walker |  | 5566 | 4 | Fs::DirListing { dir: apps/web/public/locales/de } |  |  | 0.672 |
| walker |  | 5570 | 4 | Fs::DirListing { dir: apps/web/public/locales/en } |  |  | 0.672 |
| walker |  | 5574 | 4 | Fs::DirListing { dir: apps/web/public/locales/es } |  |  | 0.672 |
| walker |  | 5578 | 4 | Fs::DirListing { dir: apps/web/public/locales/fr } |  |  | 0.672 |
| walker |  | 5582 | 4 | Fs::DirListing { dir: apps/web/public/locales/it } |  |  | 0.672 |
| walker |  | 5586 | 4 | Fs::DirListing { dir: apps/web/public/locales/ja } |  |  | 0.672 |
| walker |  | 5590 | 4 | Fs::DirListing { dir: apps/web/public/locales/nl } |  |  | 0.672 |
| walker |  | 5594 | 4 | Fs::DirListing { dir: apps/web/public/locales/pl } |  |  | 0.672 |
| walker |  | 5598 | 4 | Fs::DirListing { dir: apps/web/public/locales/pt-BR } |  |  | 0.672 |
| walker |  | 5602 | 4 | Fs::DirListing { dir: apps/web/public/locales/ro } |  |  | 0.672 |
| walker |  | 5606 | 4 | Fs::DirListing { dir: apps/web/public/locales/ru } |  |  | 0.672 |
| walker |  | 5610 | 4 | Fs::DirListing { dir: apps/web/public/locales/tr } |  |  | 0.672 |
| walker |  | 5614 | 4 | Fs::DirListing { dir: apps/web/public/locales/uk } |  |  | 0.672 |
| walker |  | 5618 | 4 | Fs::DirListing { dir: apps/web/public/locales/zh } |  |  | 0.672 |
| walker |  | 5622 | 4 | Fs::DirListing { dir: apps/web/public/locales/zh-TW } |  |  | 0.672 |
| ns | 5647 |  | 398 | packages/types/global.ts — every exported type, interface and enum declaration | 4.2 |  | 0.655 |
| walker |  | 5672 | 50 | Fs::DirListing { dir: apps/mobile/components/ui } |  |  | 0.655 |
| ns | 5793 |  | 146 | `ArchivedFormat`, `LinkType` and `TokenExpiry` variants | 4.3 | 4.2 | 0.647 |
| walker |  | 5840 | 168 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 275 } |  |  | 0.649 |
| ns | 5945 |  | 152 | `ViewMode`, `Sort` and `TagSort` variants | 4.4 | 4.2 | 0.641 |
| walker |  | 6010 | 170 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 220 } |  |  | 0.643 |
| ns | 6285 |  | 340 | packages/lib/schemaValidation.ts — every exported zod schema constant | 4.5 |  | 0.631 |
| walker |  | 6383 | 373 | Prisma::DeclTail { file: packages/prisma/schema.prisma, start_line: 28, tail_start_line: 51 } |  |  | 0.662 |
| walker |  | 6399 | 16 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 1 } |  |  | 0.668 |
| walker |  | 6500 | 101 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 99 } |  |  | 0.670 |
| walker |  | 6531 | 31 | Code::CodeKey { rung: Names, file: apps/worker/worker.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.670 |
| ns | 6542 |  | 257 | packages/router — the links, collections and tags hook exports | 4.6 |  | 0.655 |
| walker |  | 6547 | 16 | Code::CodeKey { rung: Decl, file: apps/worker/worker.ts, decl: 1, sub: 0, line: 8 } |  |  | 0.655 |
| walker |  | 6682 | 135 | Json::Dependencies { file: package.json } |  |  | 0.655 |
| ns | 6726 |  | 184 | packages/router — export lines of the remaining ten hook modules | 4.7 | 4.6 | 0.650 |
| walker |  | 6782 | 100 | Code::CodeKey { rung: Names, file: packages/filesystem/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.650 |
| walker |  | 6794 | 12 | Code::CodeKey { rung: Names, file: packages/prisma/client/index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.650 |
| walker |  | 6806 | 12 | Code::CodeKey { rung: Names, file: packages/prisma/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.650 |
| walker |  | 6882 | 76 | Code::CodeKey { rung: Decl, file: packages/prisma/index.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.656 |
| ns | 6892 |  | 166 | Complete apps/worker listing, including every job and every preservation handler | 5.1 |  | 0.668 |
| walker |  | 6920 | 38 | Code::CodeKey { rung: Names, file: apps/worker/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.668 |
| walker |  | 6946 | 26 | Code::CodeKey { rung: Names, file: apps/web/e2e/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.668 |
| walker |  | 6960 | 14 | Code::CodeKey { rung: Names, file: apps/web/pages/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.668 |
| walker |  | 6974 | 14 | Code::CodeKey { rung: Names, file: apps/web/pages/settings/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.668 |
| walker |  | 7053 | 79 | Fs::DirListing { dir: apps/web/lib/client } |  |  | 0.677 |
| walker |  | 7068 | 15 | Code::CodeKey { rung: Names, file: apps/mobile/app/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.677 |
| walker |  | 7131 | 63 | Code::CodeKey { rung: Names, file: apps/web/pages/collections/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.677 |
| walker |  | 7146 | 15 | Code::CodeKey { rung: Body, file: apps/web/pages/collections/index.tsx, decl: 2, sub: 0, line: 153 } |  |  | 0.677 |
| ns | 7161 |  | 269 | worker.ts — the whole scheduler entry point | 5.2 |  | 0.667 |
| walker |  | 7209 | 63 | Code::CodeKey { rung: Names, file: apps/web/pages/links/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.667 |
| walker |  | 7224 | 15 | Code::CodeKey { rung: Body, file: apps/web/pages/links/index.tsx, decl: 2, sub: 0, line: 70 } |  |  | 0.667 |
| walker |  | 7289 | 65 | Code::CodeKey { rung: Names, file: apps/web/pages/tags/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.667 |
| walker |  | 7304 | 15 | Code::CodeKey { rung: Body, file: apps/web/pages/tags/index.tsx, decl: 2, sub: 0, line: 274 } |  |  | 0.667 |
| ns | 7354 |  | 193 | archiveHandler's signature and its SSRF / skip-preservation guard | 5.3 |  | 0.657 |
| walker |  | 7439 | 135 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.657 |
| walker |  | 7536 | 97 | Fs::DirListing { dir: apps/web/lib/api } |  |  | 0.683 |
| walker |  | 7549 | 13 | Fs::DirListing { dir: apps/web/lib/api/archives } |  |  | 0.684 |
| walker |  | 7566 | 17 | Fs::DirListing { dir: apps/web/lib/api/preserved } |  |  | 0.684 |
| walker |  | 7599 | 33 | Fs::DirListing { dir: apps/web/lib/api/stripe } |  |  | 0.688 |
| ns | 7607 |  | 253 | Every UI page route under apps/web/pages | 6.1 |  | 0.702 |
| walker |  | 7635 | 36 | Fs::DirListing { dir: apps/web/lib/api/controllers } |  |  | 0.717 |
| walker |  | 7640 | 5 | Fs::DirListing { dir: apps/web/lib/api/controllers/search } |  |  | 0.717 |
| walker |  | 7645 | 5 | Fs::DirListing { dir: apps/web/lib/api/controllers/session } |  |  | 0.717 |
| walker |  | 7651 | 6 | Fs::DirListing { dir: apps/web/lib/api/controllers/worker } |  |  | 0.718 |
| walker |  | 7660 | 9 | Fs::DirListing { dir: apps/web/lib/api/controllers/public } |  |  | 0.718 |
| walker |  | 7664 | 4 | Fs::DirListing { dir: apps/web/lib/api/controllers/public/links } |  |  | 0.719 |
| walker |  | 7670 | 6 | Fs::DirListing { dir: apps/web/lib/api/controllers/public/collections } |  |  | 0.719 |
| walker |  | 7676 | 6 | Fs::DirListing { dir: apps/web/lib/api/controllers/public/users } |  |  | 0.719 |
| walker |  | 7683 | 7 | Fs::DirListing { dir: apps/web/lib/api/controllers/public/links/linkId } |  |  | 0.720 |
| walker |  | 7697 | 14 | Fs::DirListing { dir: apps/web/lib/api/controllers/collections } |  |  | 0.720 |
| walker |  | 7711 | 14 | Fs::DirListing { dir: apps/web/lib/api/controllers/highlights } |  |  | 0.721 |
| walker |  | 7725 | 14 | Fs::DirListing { dir: apps/web/lib/api/controllers/tokens } |  |  | 0.723 |
| walker |  | 7732 | 7 | Fs::DirListing { dir: apps/web/lib/api/controllers/tokens/tokenId } |  |  | 0.723 |
| walker |  | 7746 | 14 | Fs::DirListing { dir: apps/web/lib/api/controllers/users } |  |  | 0.726 |
| walker |  | 7763 | 17 | Fs::DirListing { dir: apps/web/lib/api/controllers/links } |  |  | 0.728 |
| walker |  | 7775 | 12 | Fs::DirListing { dir: apps/web/lib/api/controllers/links/bulk } |  |  | 0.729 |
| walker |  | 7795 | 20 | Fs::DirListing { dir: apps/web/lib/api/controllers/dashboard } |  |  | 0.732 |
| walker |  | 7822 | 27 | Fs::DirListing { dir: apps/web/lib/api/controllers/tags } |  |  | 0.736 |
| walker |  | 7843 | 21 | Fs::DirListing { dir: apps/web/lib/api/controllers/collections/collectionId } |  |  | 0.739 |
| walker |  | 7864 | 21 | Fs::DirListing { dir: apps/web/lib/api/controllers/tags/tagId } |  |  | 0.743 |
| walker |  | 7888 | 24 | Fs::DirListing { dir: apps/web/lib/api/controllers/links/linkId } |  |  | 0.749 |
| walker |  | 7894 | 6 | Fs::DirListing { dir: apps/web/lib/api/controllers/links/linkId/highlight } |  |  | 0.750 |
| walker |  | 7921 | 27 | Fs::DirListing { dir: apps/web/lib/api/controllers/users/userId } |  |  | 0.755 |
| walker |  | 7958 | 37 | Code::CodeKey { rung: Names, file: apps/web/pages/admin/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.755 |
| ns | 7965 |  | 358 | The flat components/ directory and the ui/ primitives | 6.2 |  | 0.725 |
| walker |  | 7966 | 8 | Code::CodeKey { rung: Body, file: apps/web/pages/admin/index.tsx, decl: 2, sub: 0, line: 12 } |  |  | 0.725 |
| walker |  | 8035 | 69 | Fs::DirListing { dir: apps/web/pages/api/v1 } |  |  | 0.746 |
| walker |  | 8039 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/dashboard } |  |  | 0.746 |
| walker |  | 8043 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/getFavicon } |  |  | 0.746 |
| walker |  | 8047 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/logins } |  |  | 0.746 |
| walker |  | 8051 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/migration } |  |  | 0.747 |
| walker |  | 8055 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/payment } |  |  | 0.747 |
| walker |  | 8059 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/search } |  |  | 0.747 |
| walker |  | 8063 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/session } |  |  | 0.748 |
| walker |  | 8067 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/webhook } |  |  | 0.748 |
| walker |  | 8076 | 9 | Fs::DirListing { dir: apps/web/pages/api/v1/config } |  |  | 0.749 |
| walker |  | 8085 | 9 | Fs::DirListing { dir: apps/web/pages/api/v1/worker } |  |  | 0.750 |
| walker |  | 8095 | 10 | Fs::DirListing { dir: apps/web/pages/api/v1/collections } |  |  | 0.750 |
| walker |  | 8105 | 10 | Fs::DirListing { dir: apps/web/pages/api/v1/highlights } |  |  | 0.751 |
| walker |  | 8115 | 10 | Fs::DirListing { dir: apps/web/pages/api/v1/rss } |  |  | 0.753 |
| walker |  | 8125 | 10 | Fs::DirListing { dir: apps/web/pages/api/v1/tokens } |  |  | 0.754 |
| walker |  | 8137 | 12 | Fs::DirListing { dir: apps/web/pages/api/v1/links } |  |  | 0.755 |
| walker |  | 8141 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/links/archive } |  |  | 0.756 |
| walker |  | 8151 | 10 | Fs::DirListing { dir: apps/web/pages/api/v1/links/[id] } |  |  | 0.759 |
| walker |  | 8155 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/links/[id]/archive } |  |  | 0.760 |
| walker |  | 8159 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/links/[id]/highlights } |  |  | 0.761 |
| walker |  | 8172 | 13 | Fs::DirListing { dir: apps/web/pages/api/v1/users } |  |  | 0.764 |
| walker |  | 8181 | 9 | Fs::DirListing { dir: apps/web/pages/api/v1/users/[id] } |  |  | 0.766 |
| walker |  | 8195 | 14 | Fs::DirListing { dir: apps/web/pages/api/v1/tags } |  |  | 0.770 |
| walker |  | 8214 | 19 | Fs::DirListing { dir: apps/web/pages/api/v1/archives } |  |  | 0.773 |
| walker |  | 8220 | 6 | Fs::DirListing { dir: apps/web/pages/api/v1/avatar } |  |  | 0.774 |
| walker |  | 8229 | 9 | Fs::DirListing { dir: apps/web/pages/api/v1/public } |  |  | 0.775 |
| walker |  | 8235 | 6 | Fs::DirListing { dir: apps/web/pages/api/v1/public/links } |  |  | 0.776 |
| walker |  | 8241 | 6 | Fs::DirListing { dir: apps/web/pages/api/v1/public/users } |  |  | 0.776 |
| ns | 8246 |  | 281 | Modal, link-view, preservation and input-picker component subdirectories | 6.3 | 6.2 | 0.757 |
| walker |  | 8253 | 12 | Fs::DirListing { dir: apps/web/pages/api/v1/public/collections } |  |  | 0.760 |
| walker |  | 8257 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/public/collections/links } |  |  | 0.761 |
| walker |  | 8261 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/public/collections/tags } |  |  | 0.762 |
| walker |  | 8279 | 18 | Fs::DirListing { dir: apps/web/pages/api/v1/preserved } |  |  | 0.768 |
| walker |  | 8301 | 22 | Fs::DirListing { dir: apps/web/pages/api/v1/auth } |  |  | 0.773 |
| walker |  | 8349 | 48 | Fs::DirListing { dir: apps/web/lib/api/controllers/migration } |  |  | 0.782 |
| ns | 8473 |  | 227 | Web hooks, layouts, stores, ambient types, email templates, one-off migration scripts and the Playwright suite | 6.4 |  | 0.781 |
| walker |  | 8659 | 310 | Plaintext::Whole { file: docker-compose.yml } |  |  | 0.782 |
| ns | 8713 |  | 240 | verifyUser — the guard chain every authenticated route runs first | 7.1 |  | 0.771 |
| walker |  | 8747 | 88 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.771 |
| walker |  | 8851 | 104 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.771 |
| ns | 8948 |  | 235 | docker-compose.yml — the three-container deployment | 7.2 |  | 0.774 |
| ns | 9162 |  | 214 | .env.sample — the required variables and the complete list of section headings | 7.3 |  | 0.765 |
| walker |  | 9228 | 377 | Json::Scripts { file: package.json } |  |  | 0.788 |
| walker |  | 9231 | 3 | Fs::DirListing { dir: apps/web/scripts } |  |  | 0.789 |
| ns | 9341 |  | 179 | The head of the optional tuning-variable block | 7.4 | 7.3 | 0.781 |
| ns | 9473 |  | 132 | CI workflows, GitHub templates, the patch-package patch, and every translated locale | 7.5 |  | 0.785 |
| walker |  | 9548 | 317 | Fs::DirListing { dir: apps/web/components } |  |  | 0.813 |
| walker |  | 9557 | 9 | Fs::DirListing { dir: apps/web/components/LinkViews } |  |  | 0.813 |
| walker |  | 9573 | 16 | Fs::DirListing { dir: apps/web/components/InputSelect } |  |  | 0.813 |
| walker |  | 9599 | 26 | Fs::DirListing { dir: apps/web/components/Preservation } |  |  | 0.814 |
| walker |  | 9640 | 41 | Fs::DirListing { dir: apps/web/components/ui } |  |  | 0.823 |
| ns | 9689 |  | 216 | apps/mobile root and the complete Expo Router screen tree | 8.1 |  | 0.827 |
| walker |  | 9702 | 62 | Fs::DirListing { dir: apps/web/components/LinkViews/LinkComponents } |  |  | 0.831 |
| walker |  | 9870 | 168 | Fs::DirListing { dir: apps/web/components/ModalContent } |  |  | 0.849 |
| ns | 9891 |  | 202 | Mobile components, stores and query-cache modules | 8.2 | 8.1 | 0.851 |
| walker |  | 9913 | 43 | Code::CodeKey { rung: Body, file: apps/web/pages/index.tsx, decl: 1, sub: 0, line: 4 } |  |  | 0.851 |
| walker |  | 9957 | 44 | Code::CodeKey { rung: Body, file: apps/web/pages/settings/index.tsx, decl: 1, sub: 0, line: 4 } |  |  | 0.851 |
| walker |  | 10000 | 43 | Code::CodeKey { rung: Body, file: apps/web/pages/admin/index.tsx, decl: 1, sub: 0, line: 3 } |  |  | 0.851 |

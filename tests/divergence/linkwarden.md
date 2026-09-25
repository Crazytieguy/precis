Score(3000)=0.740 I=0.896 C=0.611 ns_rows≤3K=15/51 grid(1000/1442/2080/3000/4327/6240/9000)=0.636/0.713/0.687/0.740/0.653/0.680/0.784

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
| walker |  | 220 | 43 | Json::Identity { file: package.json } |  |  | 0.714 |
| walker |  | 224 | 4 | Fs::DirListing { dir: .vscode } |  |  | 0.714 |
| walker |  | 237 | 13 | Fs::DirListing { dir: packages/types } |  |  | 0.714 |
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
| walker |  | 852 | 24 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 77 } |  |  | 0.633 |
| walker |  | 901 | 49 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 90 } |  |  | 0.635 |
| walker |  | 981 | 80 | Fs::DirListing { dir: apps/mobile } |  |  | 0.636 |
| walker |  | 985 | 4 | Fs::DirListing { dir: apps/mobile/styles } |  |  | 0.636 |
| walker |  | 991 | 6 | Fs::DirListing { dir: apps/mobile/assets } |  |  | 0.636 |
| walker |  | 1027 | 36 | Fs::DirListing { dir: apps/mobile/app } |  |  | 0.636 |
| walker |  | 1035 | 8 | Fs::DirListing { dir: apps/mobile/plugins } |  |  | 0.636 |
| walker |  | 1041 | 6 | Fs::DirListing { dir: apps/mobile/app/links } |  |  | 0.636 |
| walker |  | 1047 | 6 | Fs::DirListing { dir: apps/mobile/assets/fonts } |  |  | 0.636 |
| walker |  | 1058 | 11 | Fs::DirListing { dir: apps/mobile/types } |  |  | 0.636 |
| walker |  | 1070 | 12 | Fs::DirListing { dir: apps/mobile/store } |  |  | 0.636 |
| ns | 1082 |  | 245 | Feature list, first half (preservation, reading, organisation, sharing) | 1.9 |  | 0.588 |
| walker |  | 1093 | 23 | Fs::DirListing { dir: apps/mobile/lib } |  |  | 0.588 |
| walker |  | 1114 | 21 | Fs::DirListing { dir: apps/mobile/app/(tabs) } |  |  | 0.588 |
| walker |  | 1125 | 11 | Fs::DirListing { dir: apps/mobile/app/(tabs)/links } |  |  | 0.588 |
| walker |  | 1142 | 17 | Fs::DirListing { dir: apps/mobile/app/(tabs)/collections } |  |  | 0.589 |
| walker |  | 1159 | 17 | Fs::DirListing { dir: apps/mobile/app/(tabs)/dashboard } |  |  | 0.589 |
| walker |  | 1176 | 17 | Fs::DirListing { dir: apps/mobile/app/(tabs)/settings } |  |  | 0.589 |
| walker |  | 1193 | 17 | Fs::DirListing { dir: apps/mobile/app/(tabs)/tags } |  |  | 0.590 |
| walker |  | 1217 | 24 | Fs::DirListing { dir: apps/mobile/assets/images } |  |  | 0.590 |
| walker |  | 1243 | 26 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 304 } |  |  | 0.591 |
| ns | 1271 |  | 189 | Feature list, second half (sync, SSO, API keys, i18n, RSS, uploads) | 1.10 |  | 0.551 |
| walker |  | 1324 | 81 | Fs::DirListing { dir: apps/web } |  |  | 0.713 |
| walker |  | 1342 | 18 | Fs::DirListing { dir: apps/web/e2e } |  |  | 0.713 |
| walker |  | 1346 | 4 | Fs::DirListing { dir: apps/web/styles } |  |  | 0.713 |
| walker |  | 1350 | 4 | Fs::DirListing { dir: apps/web/e2e/data } |  |  | 0.713 |
| walker |  | 1359 | 9 | Fs::DirListing { dir: apps/web/store } |  |  | 0.713 |
| walker |  | 1362 | 3 | Fs::DirListing { dir: apps/web/scripts } |  |  | 0.713 |
| walker |  | 1374 | 12 | Fs::DirListing { dir: apps/web/types } |  |  | 0.713 |
| walker |  | 1387 | 13 | Fs::DirListing { dir: apps/web/lib } |  |  | 0.713 |
| walker |  | 1473 | 86 | Fs::DirListing { dir: apps/web/pages } |  |  | 0.714 |
| walker |  | 1484 | 11 | Fs::DirListing { dir: apps/web/pages/collections } |  |  | 0.714 |
| walker |  | 1495 | 11 | Fs::DirListing { dir: apps/web/pages/tags } |  |  | 0.714 |
| walker |  | 1511 | 16 | Fs::DirListing { dir: apps/web/pages/links } |  |  | 0.714 |
| walker |  | 1530 | 19 | Fs::DirListing { dir: apps/web/pages/admin } |  |  | 0.715 |
| walker |  | 1536 | 6 | Fs::DirListing { dir: apps/web/pages/preserved } |  |  | 0.715 |
| ns | 1539 |  | 268 | schema.prisma: datasource/generator plus every model and enum header, bodies elided | 2.1 |  | 0.667 |
| walker |  | 1592 | 56 | Fs::DirListing { dir: apps/web/pages/settings } |  |  | 0.668 |
| walker |  | 1600 | 8 | Fs::DirListing { dir: apps/web/pages/api } |  |  | 0.668 |
| walker |  | 1603 | 3 | Fs::DirListing { dir: apps/web/pages/api/v2 } |  |  | 0.668 |
| walker |  | 1607 | 4 | Fs::DirListing { dir: apps/web/pages/api/v2/dashboard } |  |  | 0.668 |
| walker |  | 1616 | 9 | Fs::DirListing { dir: apps/web/pages/public } |  |  | 0.668 |
| walker |  | 1621 | 5 | Fs::DirListing { dir: apps/web/pages/public/collections } |  |  | 0.668 |
| walker |  | 1631 | 10 | Fs::DirListing { dir: apps/web/pages/public/collections/[id] } |  |  | 0.669 |
| walker |  | 1637 | 6 | Fs::DirListing { dir: apps/web/pages/public/links } |  |  | 0.669 |
| walker |  | 1643 | 6 | Fs::DirListing { dir: apps/web/pages/public/preserved } |  |  | 0.669 |
| walker |  | 1655 | 12 | Fs::DirListing { dir: apps/web/pages/auth } |  |  | 0.669 |
| walker |  | 1676 | 21 | Fs::DirListing { dir: apps/web/templates } |  |  | 0.670 |
| walker |  | 1682 | 6 | Fs::DirListing { dir: apps/web/e2e/tests } |  |  | 0.670 |
| walker |  | 1687 | 5 | Fs::DirListing { dir: apps/web/e2e/tests/public } |  |  | 0.670 |
| ns | 1697 |  | 158 | All five schema enums, fully expanded | 2.2 | 2.1 | 0.649 |
| walker |  | 1711 | 24 | Fs::DirListing { dir: apps/web/layouts } |  |  | 0.649 |
| walker |  | 1731 | 20 | Fs::DirListing { dir: apps/web/lib/shared } |  |  | 0.649 |
| walker |  | 1741 | 10 | Fs::DirListing { dir: apps/web/e2e/tests/global } |  |  | 0.649 |
| walker |  | 1776 | 35 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 83 } |  |  | 0.676 |
| walker |  | 1986 | 210 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 166 } |  |  | 0.683 |
| ns | 2083 |  | 386 | `Link` model — every field | 2.3 | 2.1 | 0.650 |
| walker |  | 2165 | 179 | Prisma::DeclTail { file: packages/prisma/schema.prisma, start_line: 166, tail_start_line: 182 } |  |  | 0.726 |
| walker |  | 2194 | 29 | Fs::DirListing { dir: .github } |  |  | 0.726 |
| walker |  | 2215 | 21 | Fs::DirListing { dir: .github/workflows } |  |  | 0.726 |
| walker |  | 2254 | 39 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 289 } |  |  | 0.755 |
| ns | 2396 |  | 313 | `User` model — identity, relations and subscription linkage | 2.4 | 2.1 | 0.709 |
| walker |  | 2426 | 172 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 10 } |  |  | 0.711 |
| walker |  | 2441 | 15 | Fs::DirListing { dir: apps/web/scripts/migration } |  |  | 0.712 |
| walker |  | 2445 | 4 | Fs::DirListing { dir: apps/web/scripts/migration/v2.6.1 } |  |  | 0.712 |
| walker |  | 2527 | 82 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 108 } |  |  | 0.714 |
| walker |  | 2636 | 109 | Fs::DirListing { dir: packages/lib } |  |  | 0.718 |
| walker |  | 2693 | 57 | Fs::DirListing { dir: apps/worker/lib } |  |  | 0.719 |
| ns | 2738 |  | 342 | `User` model — preference and archival-toggle fields | 2.5 | 2.4 | 0.682 |
| walker |  | 2978 | 285 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 28 } |  |  | 0.740 |
| ns | 3055 |  | 317 | `Collection` model — every field, including the self-relation | 2.6 | 2.1 | 0.705 |
| walker |  | 3115 | 137 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.705 |
| walker |  | 3141 | 26 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.705 |
| walker |  | 3296 | 155 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.705 |
| walker |  | 3325 | 29 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.705 |
| walker |  | 3384 | 59 | Fs::DirListing { dir: apps/mobile/components } |  |  | 0.706 |
| walker |  | 3410 | 26 | Fs::DirListing { dir: apps/mobile/components/Formats } |  |  | 0.706 |
| ns | 3436 |  | 381 | `UsersAndCollections` join model (the permission bits) and `Tag` model | 2.7 | 2.1 | 0.666 |
| walker |  | 3442 | 32 | Fs::DirListing { dir: apps/mobile/components/ActionSheets } |  |  | 0.667 |
| ns | 3594 |  | 158 | `AccessToken` model | 2.8 | 2.1 | 0.653 |
| ns | 3617 |  | 23 | Complete packages/prisma listing | 2.9 |  | 0.658 |
| walker |  | 3660 | 218 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 200 } |  |  | 0.683 |
| walker |  | 3722 | 62 | Fs::DirListing { dir: apps/web/hooks } |  |  | 0.684 |
| ns | 3733 |  | 116 | The process-wide `prisma` client singleton | 2.10 |  | 0.672 |
| walker |  | 3787 | 65 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 118 } |  |  | 0.674 |
| ns | 3817 |  | 84 | API version directories: every resource under pages/api/v1 (and the single v2 route) | 3.1 |  | 0.647 |
| walker |  | 3883 | 96 | Json::Runtime { file: package.json } |  |  | 0.647 |
| ns | 3885 |  | 68 | Route files for links, collections, tags and highlights | 3.2 |  | 0.631 |
| walker |  | 3900 | 17 | Fs::DirListing { dir: apps/web/e2e/fixtures } |  |  | 0.631 |
| walker |  | 3913 | 13 | Fs::DirListing { dir: apps/web/e2e/fixtures/base } |  |  | 0.631 |
| walker |  | 3940 | 27 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 5 } |  |  | 0.646 |
| ns | 3962 |  | 77 | Route files for auth, session, users, tokens, config, avatar and logins | 3.3 |  | 0.630 |
| walker |  | 4009 | 69 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 296 } |  |  | 0.632 |
| ns | 4042 |  | 80 | Route files for archives, preserved, search, dashboard, rss, migration, payment, webhook, worker, getFavicon | 3.4 |  | 0.617 |
| ns | 4083 |  | 41 | The unauthenticated `public/` route subtree | 3.5 |  | 0.608 |
| walker |  | 4178 | 169 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 151 } |  |  | 0.645 |
| ns | 4359 |  | 276 | The route-handler pattern, read from pages/api/v1/links/index.ts | 3.6 |  | 0.628 |
| ns | 4568 |  | 209 | apps/web/lib: the server helper layer and its client/shared siblings | 3.7 |  | 0.600 |
| walker |  | 4613 | 435 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: true } |  |  | 0.648 |
| ns | 4667 |  | 99 | The controller tree: every resource directory under lib/api/controllers | 3.8 |  | 0.632 |
| walker |  | 4786 | 173 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 260 } |  |  | 0.633 |
| ns | 4823 |  | 156 | Controller files for links, collections, tags and highlights | 3.9 |  | 0.615 |
| walker |  | 4916 | 130 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 248 } |  |  | 0.616 |
| ns | 5001 |  | 178 | Controller files for users, tokens, session, search, dashboard, worker, migration and public access | 3.10 |  | 0.596 |
| walker |  | 5236 | 320 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 126 } |  |  | 0.630 |
| ns | 5249 |  | 248 | Complete listings of packages/types, packages/lib, packages/filesystem and packages/router | 4.1 |  | 0.656 |
| walker |  | 5397 | 161 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 234 } |  |  | 0.671 |
| walker |  | 5439 | 42 | Fs::DirListing { dir: apps/worker/lib/preservationScheme } |  |  | 0.672 |
| walker |  | 5607 | 168 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 275 } |  |  | 0.673 |
| ns | 5647 |  | 398 | packages/types/global.ts — every exported type, interface and enum declaration | 4.2 |  | 0.655 |
| walker |  | 5777 | 170 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 220 } |  |  | 0.657 |
| ns | 5793 |  | 146 | `ArchivedFormat`, `LinkType` and `TokenExpiry` variants | 4.3 | 4.2 | 0.649 |
| ns | 5945 |  | 152 | `ViewMode`, `Sort` and `TagSort` variants | 4.4 | 4.2 | 0.641 |
| walker |  | 6150 | 373 | Prisma::DeclTail { file: packages/prisma/schema.prisma, start_line: 28, tail_start_line: 51 } |  |  | 0.673 |
| walker |  | 6166 | 16 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 1 } |  |  | 0.680 |
| walker |  | 6245 | 79 | Fs::DirListing { dir: apps/web/public } |  |  | 0.680 |
| walker |  | 6255 | 10 | Fs::DirListing { dir: apps/web/public/screenshots } |  |  | 0.680 |
| ns | 6285 |  | 340 | packages/lib/schemaValidation.ts — every exported zod schema constant | 4.5 |  | 0.667 |
| walker |  | 6356 | 101 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 99 } |  |  | 0.669 |
| walker |  | 6387 | 31 | Code::CodeKey { rung: Names, file: apps/worker/worker.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.669 |
| walker |  | 6403 | 16 | Code::CodeKey { rung: Decl, file: apps/worker/worker.ts, decl: 1, sub: 0, line: 8 } |  |  | 0.669 |
| walker |  | 6425 | 22 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.669 |
| walker |  | 6474 | 49 | Fs::DirListing { dir: apps/web/public/locales } |  |  | 0.670 |
| walker |  | 6478 | 4 | Fs::DirListing { dir: apps/web/public/locales/de } |  |  | 0.670 |
| walker |  | 6482 | 4 | Fs::DirListing { dir: apps/web/public/locales/en } |  |  | 0.670 |
| walker |  | 6486 | 4 | Fs::DirListing { dir: apps/web/public/locales/es } |  |  | 0.670 |
| walker |  | 6490 | 4 | Fs::DirListing { dir: apps/web/public/locales/fr } |  |  | 0.670 |
| walker |  | 6494 | 4 | Fs::DirListing { dir: apps/web/public/locales/it } |  |  | 0.670 |
| walker |  | 6498 | 4 | Fs::DirListing { dir: apps/web/public/locales/ja } |  |  | 0.670 |
| walker |  | 6502 | 4 | Fs::DirListing { dir: apps/web/public/locales/nl } |  |  | 0.670 |
| walker |  | 6506 | 4 | Fs::DirListing { dir: apps/web/public/locales/pl } |  |  | 0.670 |
| walker |  | 6510 | 4 | Fs::DirListing { dir: apps/web/public/locales/pt-BR } |  |  | 0.670 |
| walker |  | 6514 | 4 | Fs::DirListing { dir: apps/web/public/locales/ro } |  |  | 0.670 |
| walker |  | 6518 | 4 | Fs::DirListing { dir: apps/web/public/locales/ru } |  |  | 0.670 |
| walker |  | 6522 | 4 | Fs::DirListing { dir: apps/web/public/locales/tr } |  |  | 0.670 |
| walker |  | 6526 | 4 | Fs::DirListing { dir: apps/web/public/locales/uk } |  |  | 0.670 |
| walker |  | 6530 | 4 | Fs::DirListing { dir: apps/web/public/locales/zh } |  |  | 0.670 |
| walker |  | 6534 | 4 | Fs::DirListing { dir: apps/web/public/locales/zh-TW } |  |  | 0.670 |
| ns | 6542 |  | 257 | packages/router — the links, collections and tags hook exports | 4.6 |  | 0.655 |
| walker |  | 6669 | 135 | Json::Dependencies { file: package.json } |  |  | 0.655 |
| walker |  | 6719 | 50 | Fs::DirListing { dir: apps/mobile/components/ui } |  |  | 0.655 |
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
| walker |  | 7026 | 15 | Code::CodeKey { rung: Names, file: apps/mobile/app/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.669 |
| walker |  | 7089 | 63 | Code::CodeKey { rung: Names, file: apps/web/pages/collections/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.669 |
| walker |  | 7104 | 15 | Code::CodeKey { rung: Body, file: apps/web/pages/collections/index.tsx, decl: 2, sub: 0, line: 153 } |  |  | 0.669 |
| ns | 7161 |  | 269 | worker.ts — the whole scheduler entry point | 5.2 |  | 0.658 |
| walker |  | 7167 | 63 | Code::CodeKey { rung: Names, file: apps/web/pages/links/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.658 |
| walker |  | 7182 | 15 | Code::CodeKey { rung: Body, file: apps/web/pages/links/index.tsx, decl: 2, sub: 0, line: 70 } |  |  | 0.658 |
| walker |  | 7247 | 65 | Code::CodeKey { rung: Names, file: apps/web/pages/tags/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.658 |
| walker |  | 7262 | 15 | Code::CodeKey { rung: Body, file: apps/web/pages/tags/index.tsx, decl: 2, sub: 0, line: 274 } |  |  | 0.658 |
| walker |  | 7341 | 79 | Fs::DirListing { dir: apps/web/lib/client } |  |  | 0.667 |
| ns | 7354 |  | 193 | archiveHandler's signature and its SSRF / skip-preservation guard | 5.3 |  | 0.658 |
| walker |  | 7476 | 135 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.658 |
| walker |  | 7513 | 37 | Code::CodeKey { rung: Names, file: apps/web/pages/admin/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.658 |
| walker |  | 7521 | 8 | Code::CodeKey { rung: Body, file: apps/web/pages/admin/index.tsx, decl: 2, sub: 0, line: 12 } |  |  | 0.658 |
| ns | 7607 |  | 253 | Every UI page route under apps/web/pages | 6.1 |  | 0.675 |
| walker |  | 7831 | 310 | Plaintext::Whole { file: docker-compose.yml } |  |  | 0.676 |
| walker |  | 7928 | 97 | Fs::DirListing { dir: apps/web/lib/api } |  |  | 0.700 |
| walker |  | 7941 | 13 | Fs::DirListing { dir: apps/web/lib/api/archives } |  |  | 0.701 |
| walker |  | 7958 | 17 | Fs::DirListing { dir: apps/web/lib/api/preserved } |  |  | 0.701 |
| ns | 7965 |  | 358 | The flat components/ directory and the ui/ primitives | 6.2 |  | 0.673 |
| walker |  | 7991 | 33 | Fs::DirListing { dir: apps/web/lib/api/stripe } |  |  | 0.676 |
| walker |  | 8027 | 36 | Fs::DirListing { dir: apps/web/lib/api/controllers } |  |  | 0.691 |
| walker |  | 8032 | 5 | Fs::DirListing { dir: apps/web/lib/api/controllers/search } |  |  | 0.691 |
| walker |  | 8037 | 5 | Fs::DirListing { dir: apps/web/lib/api/controllers/session } |  |  | 0.691 |
| walker |  | 8043 | 6 | Fs::DirListing { dir: apps/web/lib/api/controllers/worker } |  |  | 0.691 |
| walker |  | 8052 | 9 | Fs::DirListing { dir: apps/web/lib/api/controllers/public } |  |  | 0.692 |
| walker |  | 8056 | 4 | Fs::DirListing { dir: apps/web/lib/api/controllers/public/links } |  |  | 0.692 |
| walker |  | 8062 | 6 | Fs::DirListing { dir: apps/web/lib/api/controllers/public/collections } |  |  | 0.692 |
| walker |  | 8068 | 6 | Fs::DirListing { dir: apps/web/lib/api/controllers/public/users } |  |  | 0.693 |
| walker |  | 8075 | 7 | Fs::DirListing { dir: apps/web/lib/api/controllers/public/links/linkId } |  |  | 0.693 |
| walker |  | 8089 | 14 | Fs::DirListing { dir: apps/web/lib/api/controllers/collections } |  |  | 0.694 |
| walker |  | 8103 | 14 | Fs::DirListing { dir: apps/web/lib/api/controllers/highlights } |  |  | 0.694 |
| walker |  | 8117 | 14 | Fs::DirListing { dir: apps/web/lib/api/controllers/tokens } |  |  | 0.696 |
| walker |  | 8124 | 7 | Fs::DirListing { dir: apps/web/lib/api/controllers/tokens/tokenId } |  |  | 0.697 |
| walker |  | 8138 | 14 | Fs::DirListing { dir: apps/web/lib/api/controllers/users } |  |  | 0.699 |
| walker |  | 8155 | 17 | Fs::DirListing { dir: apps/web/lib/api/controllers/links } |  |  | 0.701 |
| walker |  | 8167 | 12 | Fs::DirListing { dir: apps/web/lib/api/controllers/links/bulk } |  |  | 0.702 |
| walker |  | 8187 | 20 | Fs::DirListing { dir: apps/web/lib/api/controllers/dashboard } |  |  | 0.705 |
| walker |  | 8214 | 27 | Fs::DirListing { dir: apps/web/lib/api/controllers/tags } |  |  | 0.709 |
| walker |  | 8235 | 21 | Fs::DirListing { dir: apps/web/lib/api/controllers/collections/collectionId } |  |  | 0.712 |
| ns | 8246 |  | 281 | Modal, link-view, preservation and input-picker component subdirectories | 6.3 | 6.2 | 0.694 |
| walker |  | 8256 | 21 | Fs::DirListing { dir: apps/web/lib/api/controllers/tags/tagId } |  |  | 0.697 |
| walker |  | 8280 | 24 | Fs::DirListing { dir: apps/web/lib/api/controllers/links/linkId } |  |  | 0.703 |
| walker |  | 8286 | 6 | Fs::DirListing { dir: apps/web/lib/api/controllers/links/linkId/highlight } |  |  | 0.704 |
| walker |  | 8313 | 27 | Fs::DirListing { dir: apps/web/lib/api/controllers/users/userId } |  |  | 0.708 |
| walker |  | 8382 | 69 | Fs::DirListing { dir: apps/web/pages/api/v1 } |  |  | 0.729 |
| walker |  | 8386 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/dashboard } |  |  | 0.729 |
| walker |  | 8390 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/getFavicon } |  |  | 0.729 |
| walker |  | 8394 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/logins } |  |  | 0.729 |
| walker |  | 8398 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/migration } |  |  | 0.730 |
| walker |  | 8402 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/payment } |  |  | 0.730 |
| walker |  | 8406 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/search } |  |  | 0.730 |
| walker |  | 8410 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/session } |  |  | 0.730 |
| walker |  | 8414 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/webhook } |  |  | 0.731 |
| walker |  | 8423 | 9 | Fs::DirListing { dir: apps/web/pages/api/v1/config } |  |  | 0.732 |
| walker |  | 8432 | 9 | Fs::DirListing { dir: apps/web/pages/api/v1/worker } |  |  | 0.733 |
| walker |  | 8442 | 10 | Fs::DirListing { dir: apps/web/pages/api/v1/collections } |  |  | 0.733 |
| walker |  | 8452 | 10 | Fs::DirListing { dir: apps/web/pages/api/v1/highlights } |  |  | 0.734 |
| walker |  | 8462 | 10 | Fs::DirListing { dir: apps/web/pages/api/v1/rss } |  |  | 0.735 |
| walker |  | 8472 | 10 | Fs::DirListing { dir: apps/web/pages/api/v1/tokens } |  |  | 0.736 |
| ns | 8473 |  | 227 | Web hooks, layouts, stores, ambient types, email templates, one-off migration scripts and the Playwright suite | 6.4 |  | 0.745 |
| walker |  | 8484 | 12 | Fs::DirListing { dir: apps/web/pages/api/v1/links } |  |  | 0.746 |
| walker |  | 8488 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/links/archive } |  |  | 0.747 |
| walker |  | 8498 | 10 | Fs::DirListing { dir: apps/web/pages/api/v1/links/[id] } |  |  | 0.750 |
| walker |  | 8502 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/links/[id]/archive } |  |  | 0.751 |
| walker |  | 8506 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/links/[id]/highlights } |  |  | 0.752 |
| walker |  | 8519 | 13 | Fs::DirListing { dir: apps/web/pages/api/v1/users } |  |  | 0.754 |
| walker |  | 8528 | 9 | Fs::DirListing { dir: apps/web/pages/api/v1/users/[id] } |  |  | 0.756 |
| walker |  | 8542 | 14 | Fs::DirListing { dir: apps/web/pages/api/v1/tags } |  |  | 0.760 |
| walker |  | 8561 | 19 | Fs::DirListing { dir: apps/web/pages/api/v1/archives } |  |  | 0.763 |
| walker |  | 8567 | 6 | Fs::DirListing { dir: apps/web/pages/api/v1/avatar } |  |  | 0.764 |
| walker |  | 8576 | 9 | Fs::DirListing { dir: apps/web/pages/api/v1/public } |  |  | 0.765 |
| walker |  | 8582 | 6 | Fs::DirListing { dir: apps/web/pages/api/v1/public/links } |  |  | 0.765 |
| walker |  | 8588 | 6 | Fs::DirListing { dir: apps/web/pages/api/v1/public/users } |  |  | 0.766 |
| walker |  | 8600 | 12 | Fs::DirListing { dir: apps/web/pages/api/v1/public/collections } |  |  | 0.769 |
| walker |  | 8604 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/public/collections/links } |  |  | 0.770 |
| walker |  | 8608 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/public/collections/tags } |  |  | 0.771 |
| walker |  | 8626 | 18 | Fs::DirListing { dir: apps/web/pages/api/v1/preserved } |  |  | 0.777 |
| walker |  | 8648 | 22 | Fs::DirListing { dir: apps/web/pages/api/v1/auth } |  |  | 0.782 |
| walker |  | 8696 | 48 | Fs::DirListing { dir: apps/web/lib/api/controllers/migration } |  |  | 0.790 |
| ns | 8713 |  | 240 | verifyUser — the guard chain every authenticated route runs first | 7.1 |  | 0.779 |
| walker |  | 8784 | 88 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.779 |
| walker |  | 8888 | 104 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.779 |
| ns | 8948 |  | 235 | docker-compose.yml — the three-container deployment | 7.2 |  | 0.782 |
| ns | 9162 |  | 214 | .env.sample — the required variables and the complete list of section headings | 7.3 |  | 0.773 |
| walker |  | 9265 | 377 | Json::Scripts { file: package.json } |  |  | 0.796 |
| walker |  | 9308 | 43 | Code::CodeKey { rung: Body, file: apps/web/pages/index.tsx, decl: 1, sub: 0, line: 4 } |  |  | 0.796 |
| ns | 9341 |  | 179 | The head of the optional tuning-variable block | 7.4 | 7.3 | 0.788 |
| walker |  | 9352 | 44 | Code::CodeKey { rung: Body, file: apps/web/pages/settings/index.tsx, decl: 1, sub: 0, line: 4 } |  |  | 0.788 |
| walker |  | 9401 | 49 | Code::CodeKey { rung: Body, file: apps/web/pages/admin/index.tsx, decl: 1, sub: 0, line: 3 } |  |  | 0.788 |
| ns | 9473 |  | 132 | CI workflows, GitHub templates, the patch-package patch, and every translated locale | 7.5 |  | 0.792 |
| ns | 9689 |  | 216 | apps/mobile root and the complete Expo Router screen tree | 8.1 |  | 0.797 |
| walker |  | 9718 | 317 | Fs::DirListing { dir: apps/web/components } |  |  | 0.824 |
| walker |  | 9727 | 9 | Fs::DirListing { dir: apps/web/components/LinkViews } |  |  | 0.824 |
| walker |  | 9743 | 16 | Fs::DirListing { dir: apps/web/components/InputSelect } |  |  | 0.824 |
| walker |  | 9769 | 26 | Fs::DirListing { dir: apps/web/components/Preservation } |  |  | 0.825 |
| walker |  | 9810 | 41 | Fs::DirListing { dir: apps/web/components/ui } |  |  | 0.833 |
| walker |  | 9872 | 62 | Fs::DirListing { dir: apps/web/components/LinkViews/LinkComponents } |  |  | 0.837 |
| ns | 9891 |  | 202 | Mobile components, stores and query-cache modules | 8.2 | 8.1 | 0.839 |
| walker |  | 10000 | 128 | Fs::DirListing { dir: apps/web/components/ModalContent } |  |  | 0.851 |

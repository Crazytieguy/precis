Score(3000)=0.676 I=0.875 C=0.522 ns_rows≤3K=15/51 grid(1000/1442/2080/3000/4327/6240/9000)=0.634/0.709/0.680/0.676/0.585/0.586/0.732

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
| walker |  | 244 | 9 | Fs::DirListing { dir: packages/prisma/client } |  |  | 0.714 |
| ns | 259 |  | 81 | Complete apps/web listing | 1.4 |  | 0.533 |
| walker |  | 270 | 26 | Fs::DirListing { dir: apps/worker } |  |  | 0.534 |
| walker |  | 275 | 5 | Fs::DirListing { dir: apps/worker/templates } |  |  | 0.534 |
| walker |  | 287 | 12 | Code::CodeKey { rung: Names, file: apps/worker/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.534 |
| ns | 321 |  | 62 | Workspace globs in the root package.json | 1.5 |  | 0.508 |
| walker |  | 324 | 37 | Code::CodeKey { rung: Names, file: packages/prisma/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.508 |
| walker |  | 400 | 76 | Code::CodeKey { rung: Decl, file: packages/prisma/index.ts, decl: 2, sub: 0, line: 5 } |  |  | 0.510 |
| walker |  | 434 | 34 | Json::Entry { file: package.json } |  |  | 0.561 |
| ns | 460 |  | 139 | Workspace package names — every `name` field under apps/ and packages/ | 1.6 |  | 0.519 |
| walker |  | 488 | 54 | Fs::DirListing { dir: packages/filesystem } |  |  | 0.520 |
| walker |  | 625 | 137 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.718 |
| ns | 674 |  | 214 | Root scripts: how you run web, worker and both together | 1.7 |  | 0.665 |
| ns | 837 |  | 163 | Root scripts: prisma, format, test, coverage, postinstall | 1.8 |  | 0.622 |
| walker |  | 845 | 220 | Prisma::Toc { file: packages/prisma/schema.prisma } |  |  | 0.632 |
| walker |  | 881 | 36 | Fs::DirListing { dir: apps/worker/workers } |  |  | 0.632 |
| walker |  | 953 | 72 | Fs::DirListing { dir: packages/router } |  |  | 0.634 |
| walker |  | 1033 | 80 | Fs::DirListing { dir: apps/mobile } |  |  | 0.634 |
| walker |  | 1037 | 4 | Fs::DirListing { dir: apps/mobile/styles } |  |  | 0.634 |
| walker |  | 1043 | 6 | Fs::DirListing { dir: apps/mobile/assets } |  |  | 0.634 |
| walker |  | 1051 | 8 | Fs::DirListing { dir: apps/mobile/plugins } |  |  | 0.634 |
| walker |  | 1057 | 6 | Fs::DirListing { dir: apps/mobile/assets/fonts } |  |  | 0.634 |
| walker |  | 1068 | 11 | Fs::DirListing { dir: apps/mobile/types } |  |  | 0.634 |
| walker |  | 1080 | 12 | Fs::DirListing { dir: apps/mobile/store } |  |  | 0.634 |
| ns | 1082 |  | 245 | Feature list, first half (preservation, reading, organisation, sharing) | 1.9 |  | 0.586 |
| walker |  | 1103 | 23 | Fs::DirListing { dir: apps/mobile/lib } |  |  | 0.586 |
| walker |  | 1139 | 36 | Fs::DirListing { dir: apps/mobile/app } |  |  | 0.587 |
| walker |  | 1154 | 15 | Code::CodeKey { rung: Names, file: apps/mobile/app/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.587 |
| walker |  | 1160 | 6 | Fs::DirListing { dir: apps/mobile/app/links } |  |  | 0.587 |
| walker |  | 1181 | 21 | Fs::DirListing { dir: apps/mobile/app/(tabs) } |  |  | 0.587 |
| walker |  | 1192 | 11 | Fs::DirListing { dir: apps/mobile/app/(tabs)/links } |  |  | 0.587 |
| walker |  | 1216 | 24 | Fs::DirListing { dir: apps/mobile/assets/images } |  |  | 0.587 |
| ns | 1271 |  | 189 | Feature list, second half (sync, SSO, API keys, i18n, RSS, uploads) | 1.10 |  | 0.548 |
| walker |  | 1297 | 81 | Fs::DirListing { dir: apps/web } |  |  | 0.709 |
| walker |  | 1301 | 4 | Fs::DirListing { dir: apps/web/styles } |  |  | 0.709 |
| walker |  | 1310 | 9 | Fs::DirListing { dir: apps/web/store } |  |  | 0.709 |
| walker |  | 1322 | 12 | Fs::DirListing { dir: apps/web/types } |  |  | 0.709 |
| walker |  | 1335 | 13 | Fs::DirListing { dir: apps/web/lib } |  |  | 0.709 |
| walker |  | 1353 | 18 | Fs::DirListing { dir: apps/web/e2e } |  |  | 0.709 |
| walker |  | 1357 | 4 | Fs::DirListing { dir: apps/web/e2e/data } |  |  | 0.709 |
| walker |  | 1383 | 26 | Code::CodeKey { rung: Names, file: apps/web/e2e/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.709 |
| walker |  | 1404 | 21 | Fs::DirListing { dir: apps/web/templates } |  |  | 0.709 |
| walker |  | 1410 | 6 | Fs::DirListing { dir: apps/web/e2e/tests } |  |  | 0.709 |
| walker |  | 1434 | 24 | Fs::DirListing { dir: apps/web/layouts } |  |  | 0.709 |
| walker |  | 1454 | 20 | Fs::DirListing { dir: apps/web/lib/shared } |  |  | 0.710 |
| walker |  | 1478 | 24 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 77 } |  |  | 0.710 |
| walker |  | 1495 | 17 | Fs::DirListing { dir: apps/mobile/app/(tabs)/collections } |  |  | 0.710 |
| walker |  | 1512 | 17 | Fs::DirListing { dir: apps/mobile/app/(tabs)/dashboard } |  |  | 0.711 |
| walker |  | 1529 | 17 | Fs::DirListing { dir: apps/mobile/app/(tabs)/settings } |  |  | 0.711 |
| ns | 1539 |  | 268 | schema.prisma: datasource/generator plus every model and enum header, bodies elided | 2.1 |  | 0.656 |
| walker |  | 1546 | 17 | Fs::DirListing { dir: apps/mobile/app/(tabs)/tags } |  |  | 0.656 |
| walker |  | 1595 | 49 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 90 } |  |  | 0.662 |
| walker |  | 1624 | 29 | Fs::DirListing { dir: .github } |  |  | 0.662 |
| walker |  | 1645 | 21 | Fs::DirListing { dir: .github/workflows } |  |  | 0.662 |
| walker |  | 1671 | 26 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 304 } |  |  | 0.667 |
| ns | 1697 |  | 158 | All five schema enums, fully expanded | 2.2 | 2.1 | 0.646 |
| walker |  | 1706 | 35 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 83 } |  |  | 0.673 |
| walker |  | 1916 | 210 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 166 } |  |  | 0.680 |
| ns | 2083 |  | 386 | `Link` model — every field | 2.3 | 2.1 | 0.647 |
| walker |  | 2095 | 179 | Prisma::DeclTail { file: packages/prisma/schema.prisma, start_line: 166, tail_start_line: 182 } |  |  | 0.723 |
| walker |  | 2230 | 135 | Json::Dependencies { file: package.json } |  |  | 0.723 |
| walker |  | 2269 | 39 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 289 } |  |  | 0.752 |
| walker |  | 2369 | 100 | Code::CodeKey { rung: Names, file: packages/filesystem/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.752 |
| walker |  | 2389 | 20 | Plaintext::Whole { file: apps/mobile/.env.sample } |  |  | 0.752 |
| ns | 2396 |  | 313 | `User` model — identity, relations and subscription linkage | 2.4 | 2.1 | 0.705 |
| walker |  | 2498 | 109 | Fs::DirListing { dir: packages/lib } |  |  | 0.709 |
| walker |  | 2555 | 57 | Fs::DirListing { dir: apps/worker/lib } |  |  | 0.710 |
| walker |  | 2614 | 59 | Fs::DirListing { dir: apps/mobile/components } |  |  | 0.710 |
| walker |  | 2640 | 26 | Fs::DirListing { dir: apps/mobile/components/Formats } |  |  | 0.711 |
| walker |  | 2672 | 32 | Fs::DirListing { dir: apps/mobile/components/ActionSheets } |  |  | 0.711 |
| walker |  | 2734 | 62 | Fs::DirListing { dir: apps/web/hooks } |  |  | 0.712 |
| ns | 2738 |  | 342 | `User` model — preference and archival-toggle fields | 2.5 | 2.4 | 0.675 |
| walker |  | 2747 | 13 | Code::CodeKey { rung: Names, file: packages/lib/transporter.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.675 |
| walker |  | 2788 | 41 | Code::CodeKey { rung: Decl, file: packages/lib/transporter.ts, decl: 1, sub: 0, line: 3 } |  |  | 0.675 |
| walker |  | 2805 | 17 | Fs::DirListing { dir: apps/web/e2e/fixtures } |  |  | 0.676 |
| walker |  | 2818 | 13 | Fs::DirListing { dir: apps/web/e2e/fixtures/base } |  |  | 0.676 |
| ns | 3055 |  | 317 | `Collection` model — every field, including the self-relation | 2.6 | 2.1 | 0.644 |
| walker |  | 3103 | 285 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 28 } |  |  | 0.699 |
| walker |  | 3117 | 14 | Code::CodeKey { rung: Names, file: packages/filesystem/createFile.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.699 |
| walker |  | 3131 | 14 | Code::CodeKey { rung: Names, file: packages/lib/constants.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.699 |
| walker |  | 3145 | 14 | Code::CodeKey { rung: Names, file: packages/lib/safeFetch.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.699 |
| walker |  | 3175 | 30 | Code::CodeKey { rung: Decl, file: packages/lib/safeFetch.ts, decl: 1, sub: 0, line: 95 } |  |  | 0.699 |
| walker |  | 3312 | 137 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.699 |
| walker |  | 3338 | 26 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.699 |
| walker |  | 3380 | 42 | Fs::DirListing { dir: apps/worker/lib/preservationScheme } |  |  | 0.701 |
| walker |  | 3395 | 15 | Code::CodeKey { rung: Names, file: packages/lib/generatePreview.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.701 |
| walker |  | 3434 | 39 | Code::CodeKey { rung: Decl, file: packages/lib/generatePreview.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.701 |
| ns | 3436 |  | 381 | `UsersAndCollections` join model (the permission bits) and `Tag` model | 2.7 | 2.1 | 0.661 |
| walker |  | 3449 | 15 | Code::CodeKey { rung: Names, file: packages/lib/rssHandler.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.661 |
| walker |  | 3479 | 30 | Code::CodeKey { rung: Decl, file: packages/lib/rssHandler.ts, decl: 1, sub: 0, line: 7 } |  |  | 0.661 |
| walker |  | 3575 | 96 | Json::Runtime { file: package.json } |  |  | 0.661 |
| ns | 3594 |  | 158 | `AccessToken` model | 2.8 | 2.1 | 0.648 |
| ns | 3617 |  | 23 | Complete packages/prisma listing | 2.9 |  | 0.653 |
| walker |  | 3730 | 155 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.653 |
| ns | 3733 |  | 116 | The process-wide `prisma` client singleton | 2.10 |  | 0.653 |
| ns | 3817 |  | 84 | API version directories: every resource under pages/api/v1 (and the single v2 route) | 3.1 |  | 0.625 |
| walker |  | 3865 | 135 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.625 |
| ns | 3885 |  | 68 | Route files for links, collections, tags and highlights | 3.2 |  | 0.610 |
| walker |  | 3892 | 27 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 5 } |  |  | 0.623 |
| walker |  | 3908 | 16 | Plaintext::Whole { file: apps/web/e2e/.env.example } |  |  | 0.623 |
| walker |  | 3924 | 16 | Code::CodeKey { rung: Names, file: packages/lib/verifyCapacity.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.623 |
| walker |  | 3952 | 28 | Code::CodeKey { rung: Decl, file: packages/lib/verifyCapacity.ts, decl: 1, sub: 0, line: 8 } |  |  | 0.623 |
| ns | 3962 |  | 77 | Route files for auth, session, users, tokens, config, avatar and logins | 3.3 |  | 0.608 |
| walker |  | 4031 | 79 | Fs::DirListing { dir: apps/web/public } |  |  | 0.608 |
| walker |  | 4041 | 10 | Fs::DirListing { dir: apps/web/public/screenshots } |  |  | 0.608 |
| ns | 4042 |  | 80 | Route files for archives, preserved, search, dashboard, rss, migration, payment, webhook, worker, getFavicon | 3.4 |  | 0.593 |
| walker |  | 4058 | 17 | Code::CodeKey { rung: Names, file: packages/lib/meilisearchClient.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.593 |
| ns | 4083 |  | 41 | The unauthenticated `public/` route subtree | 3.5 |  | 0.585 |
| ns | 4359 |  | 276 | The route-handler pattern, read from pages/api/v1/links/index.ts | 3.6 |  | 0.569 |
| walker |  | 4378 | 320 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 126 } |  |  | 0.611 |
| walker |  | 4433 | 55 | Code::CodeKey { rung: Decl, file: packages/lib/meilisearchClient.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.611 |
| walker |  | 4455 | 22 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.612 |
| ns | 4568 |  | 209 | apps/web/lib: the server helper layer and its client/shared siblings | 3.7 |  | 0.585 |
| walker |  | 4589 | 134 | Plaintext::Whole { file: .env.sample } |  |  | 0.585 |
| walker |  | 4638 | 49 | Fs::DirListing { dir: apps/web/public/locales } |  |  | 0.586 |
| walker |  | 4642 | 4 | Fs::DirListing { dir: apps/web/public/locales/de } |  |  | 0.586 |
| walker |  | 4646 | 4 | Fs::DirListing { dir: apps/web/public/locales/en } |  |  | 0.586 |
| walker |  | 4650 | 4 | Fs::DirListing { dir: apps/web/public/locales/es } |  |  | 0.586 |
| walker |  | 4654 | 4 | Fs::DirListing { dir: apps/web/public/locales/fr } |  |  | 0.586 |
| walker |  | 4658 | 4 | Fs::DirListing { dir: apps/web/public/locales/it } |  |  | 0.586 |
| walker |  | 4662 | 4 | Fs::DirListing { dir: apps/web/public/locales/ja } |  |  | 0.586 |
| walker |  | 4666 | 4 | Fs::DirListing { dir: apps/web/public/locales/nl } |  |  | 0.586 |
| ns | 4667 |  | 99 | The controller tree: every resource directory under lib/api/controllers | 3.8 |  | 0.571 |
| walker |  | 4670 | 4 | Fs::DirListing { dir: apps/web/public/locales/pl } |  |  | 0.571 |
| walker |  | 4674 | 4 | Fs::DirListing { dir: apps/web/public/locales/pt-BR } |  |  | 0.571 |
| walker |  | 4678 | 4 | Fs::DirListing { dir: apps/web/public/locales/ro } |  |  | 0.571 |
| walker |  | 4682 | 4 | Fs::DirListing { dir: apps/web/public/locales/ru } |  |  | 0.571 |
| walker |  | 4686 | 4 | Fs::DirListing { dir: apps/web/public/locales/tr } |  |  | 0.571 |
| walker |  | 4690 | 4 | Fs::DirListing { dir: apps/web/public/locales/uk } |  |  | 0.571 |
| walker |  | 4694 | 4 | Fs::DirListing { dir: apps/web/public/locales/zh } |  |  | 0.571 |
| walker |  | 4698 | 4 | Fs::DirListing { dir: apps/web/public/locales/zh-TW } |  |  | 0.571 |
| walker |  | 4727 | 29 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.571 |
| walker |  | 4777 | 50 | Fs::DirListing { dir: apps/mobile/components/ui } |  |  | 0.572 |
| ns | 4823 |  | 156 | Controller files for links, collections, tags and highlights | 3.9 |  | 0.555 |
| walker |  | 4863 | 86 | Fs::DirListing { dir: apps/web/pages } |  |  | 0.555 |
| walker |  | 4877 | 14 | Code::CodeKey { rung: Names, file: apps/web/pages/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.555 |
| walker |  | 4883 | 6 | Fs::DirListing { dir: apps/web/pages/preserved } |  |  | 0.555 |
| walker |  | 4891 | 8 | Fs::DirListing { dir: apps/web/pages/api } |  |  | 0.556 |
| walker |  | 4894 | 3 | Fs::DirListing { dir: apps/web/pages/api/v2 } |  |  | 0.556 |
| walker |  | 4903 | 9 | Fs::DirListing { dir: apps/web/pages/public } |  |  | 0.556 |
| walker |  | 4908 | 5 | Fs::DirListing { dir: apps/web/pages/public/collections } |  |  | 0.556 |
| walker |  | 4912 | 4 | Fs::DirListing { dir: apps/web/pages/api/v2/dashboard } |  |  | 0.557 |
| walker |  | 4918 | 6 | Fs::DirListing { dir: apps/web/pages/public/links } |  |  | 0.557 |
| walker |  | 4924 | 6 | Fs::DirListing { dir: apps/web/pages/public/preserved } |  |  | 0.557 |
| walker |  | 4935 | 11 | Fs::DirListing { dir: apps/web/pages/collections } |  |  | 0.557 |
| walker |  | 4946 | 11 | Fs::DirListing { dir: apps/web/pages/tags } |  |  | 0.557 |
| walker |  | 4958 | 12 | Fs::DirListing { dir: apps/web/pages/auth } |  |  | 0.557 |
| walker |  | 4974 | 16 | Fs::DirListing { dir: apps/web/pages/links } |  |  | 0.557 |
| walker |  | 4993 | 19 | Fs::DirListing { dir: apps/web/pages/admin } |  |  | 0.558 |
| ns | 5001 |  | 178 | Controller files for users, tokens, session, search, dashboard, worker, migration and public access | 3.10 |  | 0.540 |
| walker |  | 5003 | 10 | Fs::DirListing { dir: apps/web/pages/public/collections/[id] } |  |  | 0.540 |
| walker |  | 5046 | 43 | Code::CodeKey { rung: Body, file: apps/web/pages/index.tsx, decl: 1, sub: 0, line: 4 } |  |  | 0.540 |
| walker |  | 5064 | 18 | Code::CodeKey { rung: Names, file: packages/lib/getPreservedFormatUrl.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.540 |
| ns | 5249 |  | 248 | Complete listings of packages/types, packages/lib, packages/filesystem and packages/router | 4.1 |  | 0.574 |
| walker |  | 5437 | 373 | Prisma::DeclTail { file: packages/prisma/schema.prisma, start_line: 28, tail_start_line: 51 } |  |  | 0.609 |
| walker |  | 5453 | 16 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 1 } |  |  | 0.615 |
| walker |  | 5472 | 19 | Code::CodeKey { rung: Names, file: packages/filesystem/readFile.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.615 |
| walker |  | 5528 | 56 | Fs::DirListing { dir: apps/web/pages/settings } |  |  | 0.616 |
| walker |  | 5550 | 22 | Code::CodeKey { rung: Names, file: packages/filesystem/fileExists.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.616 |
| walker |  | 5572 | 22 | Code::CodeKey { rung: Names, file: packages/filesystem/moveFile.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.616 |
| walker |  | 5646 | 74 | Code::CodeKey { rung: Decl, file: packages/filesystem/createFile.ts, decl: 1, sub: 0, line: 6 } |  |  | 0.616 |
| ns | 5647 |  | 398 | packages/types/global.ts — every exported type, interface and enum declaration | 4.2 |  | 0.600 |
| walker |  | 5669 | 23 | Code::CodeKey { rung: Names, file: packages/filesystem/createFolder.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.600 |
| walker |  | 5692 | 23 | Code::CodeKey { rung: Names, file: packages/lib/isArchivalTag.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.600 |
| walker |  | 5716 | 24 | Code::CodeKey { rung: Names, file: packages/filesystem/removeFile.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.600 |
| walker |  | 5740 | 24 | Code::CodeKey { rung: Names, file: packages/filesystem/removeFolder.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.600 |
| walker |  | 5764 | 24 | Code::CodeKey { rung: Names, file: packages/lib/getOriginalFormat.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.600 |
| walker |  | 5790 | 26 | Code::CodeKey { rung: Decl, file: packages/lib/getOriginalFormat.ts, decl: 1, sub: 0, line: 6 } |  |  | 0.600 |
| ns | 5793 |  | 146 | `ArchivedFormat`, `LinkType` and `TokenExpiry` variants | 4.3 | 4.2 | 0.593 |
| walker |  | 5879 | 89 | Code::CodeKey { rung: Decl, file: packages/lib/isArchivalTag.ts, decl: 1, sub: 0, line: 3 } |  |  | 0.593 |
| ns | 5945 |  | 152 | `ViewMode`, `Sort` and `TagSort` variants | 4.4 | 4.2 | 0.586 |
| ns | 6285 |  | 340 | packages/lib/schemaValidation.ts — every exported zod schema constant | 4.5 |  | 0.575 |
| walker |  | 6314 | 435 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: true } |  |  | 0.612 |
| walker |  | 6393 | 79 | Fs::DirListing { dir: apps/web/lib/client } |  |  | 0.623 |
| walker |  | 6421 | 28 | Code::CodeKey { rung: Names, file: packages/filesystem/s3Client.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.623 |
| walker |  | 6449 | 28 | Code::CodeKey { rung: Names, file: packages/lib/formatStats.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.623 |
| walker |  | 6473 | 24 | Code::CodeKey { rung: Decl, file: packages/lib/formatStats.ts, decl: 2, sub: 0, line: 11 } |  |  | 0.623 |
| walker |  | 6524 | 51 | Code::CodeKey { rung: Decl, file: packages/lib/formatStats.ts, decl: 1, sub: 0, line: 4 } |  |  | 0.623 |
| ns | 6542 |  | 257 | packages/router — the links, collections and tags hook exports | 4.6 |  | 0.609 |
| walker |  | 6552 | 28 | Code::CodeKey { rung: Names, file: packages/lib/getFormatBasedOnPreference.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.609 |
| walker |  | 6605 | 53 | Code::CodeKey { rung: Decl, file: packages/lib/getFormatBasedOnPreference.ts, decl: 1, sub: 0, line: 15 } |  |  | 0.609 |
| walker |  | 6633 | 28 | Code::CodeKey { rung: Names, file: packages/lib/getLinkTypeFromFormat.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.609 |
| walker |  | 6667 | 34 | Code::CodeKey { rung: Decl, file: packages/lib/getLinkTypeFromFormat.ts, decl: 1, sub: 0, line: 3 } |  |  | 0.609 |
| walker |  | 6684 | 17 | Code::CodeKey { rung: Names, file: packages/prisma/client/index.d.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.609 |
| walker |  | 6717 | 33 | Code::CodeKey { rung: Names, file: packages/router/publicLinks.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.609 |
| ns | 6726 |  | 184 | packages/router — export lines of the remaining ten hook modules | 4.7 | 4.6 | 0.604 |
| walker |  | 6750 | 33 | Code::CodeKey { rung: Names, file: packages/router/publicTags.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.604 |
| walker |  | 6847 | 97 | Fs::DirListing { dir: apps/web/lib/api } |  |  | 0.634 |
| walker |  | 6860 | 13 | Fs::DirListing { dir: apps/web/lib/api/archives } |  |  | 0.634 |
| walker |  | 6877 | 17 | Fs::DirListing { dir: apps/web/lib/api/preserved } |  |  | 0.634 |
| ns | 6892 |  | 166 | Complete apps/worker listing, including every job and every preservation handler | 5.1 |  | 0.647 |
| walker |  | 6910 | 33 | Fs::DirListing { dir: apps/web/lib/api/stripe } |  |  | 0.651 |
| walker |  | 6946 | 36 | Fs::DirListing { dir: apps/web/lib/api/controllers } |  |  | 0.668 |
| walker |  | 6951 | 5 | Fs::DirListing { dir: apps/web/lib/api/controllers/search } |  |  | 0.668 |
| walker |  | 6956 | 5 | Fs::DirListing { dir: apps/web/lib/api/controllers/session } |  |  | 0.668 |
| walker |  | 6962 | 6 | Fs::DirListing { dir: apps/web/lib/api/controllers/worker } |  |  | 0.669 |
| walker |  | 6971 | 9 | Fs::DirListing { dir: apps/web/lib/api/controllers/public } |  |  | 0.669 |
| walker |  | 6975 | 4 | Fs::DirListing { dir: apps/web/lib/api/controllers/public/links } |  |  | 0.670 |
| walker |  | 6981 | 6 | Fs::DirListing { dir: apps/web/lib/api/controllers/public/collections } |  |  | 0.670 |
| walker |  | 6987 | 6 | Fs::DirListing { dir: apps/web/lib/api/controllers/public/users } |  |  | 0.671 |
| walker |  | 6994 | 7 | Fs::DirListing { dir: apps/web/lib/api/controllers/public/links/linkId } |  |  | 0.671 |
| walker |  | 7008 | 14 | Fs::DirListing { dir: apps/web/lib/api/controllers/collections } |  |  | 0.672 |
| walker |  | 7022 | 14 | Fs::DirListing { dir: apps/web/lib/api/controllers/highlights } |  |  | 0.672 |
| walker |  | 7036 | 14 | Fs::DirListing { dir: apps/web/lib/api/controllers/tokens } |  |  | 0.674 |
| walker |  | 7043 | 7 | Fs::DirListing { dir: apps/web/lib/api/controllers/tokens/tokenId } |  |  | 0.675 |
| walker |  | 7057 | 14 | Fs::DirListing { dir: apps/web/lib/api/controllers/users } |  |  | 0.678 |
| walker |  | 7074 | 17 | Fs::DirListing { dir: apps/web/lib/api/controllers/links } |  |  | 0.680 |
| walker |  | 7086 | 12 | Fs::DirListing { dir: apps/web/lib/api/controllers/links/bulk } |  |  | 0.682 |
| walker |  | 7106 | 20 | Fs::DirListing { dir: apps/web/lib/api/controllers/dashboard } |  |  | 0.685 |
| walker |  | 7133 | 27 | Fs::DirListing { dir: apps/web/lib/api/controllers/tags } |  |  | 0.690 |
| walker |  | 7154 | 21 | Fs::DirListing { dir: apps/web/lib/api/controllers/collections/collectionId } |  |  | 0.693 |
| ns | 7161 |  | 269 | worker.ts — the whole scheduler entry point | 5.2 |  | 0.682 |
| walker |  | 7175 | 21 | Fs::DirListing { dir: apps/web/lib/api/controllers/tags/tagId } |  |  | 0.686 |
| walker |  | 7199 | 24 | Fs::DirListing { dir: apps/web/lib/api/controllers/links/linkId } |  |  | 0.693 |
| walker |  | 7205 | 6 | Fs::DirListing { dir: apps/web/lib/api/controllers/links/linkId/highlight } |  |  | 0.695 |
| walker |  | 7232 | 27 | Fs::DirListing { dir: apps/web/lib/api/controllers/users/userId } |  |  | 0.700 |
| walker |  | 7351 | 119 | Code::CodeKey { rung: Decl, file: packages/lib/getPreservedFormatUrl.ts, decl: 1, sub: 0, line: 3 } |  |  | 0.700 |
| ns | 7354 |  | 193 | archiveHandler's signature and its SSRF / skip-preservation guard | 5.3 |  | 0.690 |
| walker |  | 7388 | 37 | Code::CodeKey { rung: Names, file: packages/lib/getFormatFromContentType.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.690 |
| walker |  | 7457 | 69 | Fs::DirListing { dir: apps/web/pages/api/v1 } |  |  | 0.714 |
| walker |  | 7461 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/dashboard } |  |  | 0.714 |
| walker |  | 7465 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/getFavicon } |  |  | 0.715 |
| walker |  | 7469 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/logins } |  |  | 0.715 |
| walker |  | 7473 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/migration } |  |  | 0.715 |
| walker |  | 7477 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/payment } |  |  | 0.715 |
| walker |  | 7481 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/search } |  |  | 0.716 |
| walker |  | 7485 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/session } |  |  | 0.716 |
| walker |  | 7489 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/webhook } |  |  | 0.717 |
| walker |  | 7495 | 6 | Fs::DirListing { dir: apps/web/pages/api/v1/avatar } |  |  | 0.717 |
| walker |  | 7504 | 9 | Fs::DirListing { dir: apps/web/pages/api/v1/config } |  |  | 0.718 |
| walker |  | 7513 | 9 | Fs::DirListing { dir: apps/web/pages/api/v1/public } |  |  | 0.719 |
| walker |  | 7519 | 6 | Fs::DirListing { dir: apps/web/pages/api/v1/public/links } |  |  | 0.719 |
| walker |  | 7525 | 6 | Fs::DirListing { dir: apps/web/pages/api/v1/public/users } |  |  | 0.720 |
| walker |  | 7534 | 9 | Fs::DirListing { dir: apps/web/pages/api/v1/worker } |  |  | 0.722 |
| walker |  | 7544 | 10 | Fs::DirListing { dir: apps/web/pages/api/v1/collections } |  |  | 0.722 |
| walker |  | 7554 | 10 | Fs::DirListing { dir: apps/web/pages/api/v1/highlights } |  |  | 0.722 |
| walker |  | 7564 | 10 | Fs::DirListing { dir: apps/web/pages/api/v1/rss } |  |  | 0.724 |
| walker |  | 7574 | 10 | Fs::DirListing { dir: apps/web/pages/api/v1/tokens } |  |  | 0.726 |
| walker |  | 7586 | 12 | Fs::DirListing { dir: apps/web/pages/api/v1/links } |  |  | 0.728 |
| walker |  | 7590 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/links/archive } |  |  | 0.729 |
| walker |  | 7603 | 13 | Fs::DirListing { dir: apps/web/pages/api/v1/users } |  |  | 0.731 |
| ns | 7607 |  | 253 | Every UI page route under apps/web/pages | 6.1 |  | 0.743 |
| walker |  | 7612 | 9 | Fs::DirListing { dir: apps/web/pages/api/v1/users/[id] } |  |  | 0.745 |
| walker |  | 7626 | 14 | Fs::DirListing { dir: apps/web/pages/api/v1/tags } |  |  | 0.748 |
| walker |  | 7636 | 10 | Fs::DirListing { dir: apps/web/pages/api/v1/links/[id] } |  |  | 0.752 |
| walker |  | 7640 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/links/[id]/archive } |  |  | 0.754 |
| walker |  | 7644 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/links/[id]/highlights } |  |  | 0.755 |
| walker |  | 7656 | 12 | Fs::DirListing { dir: apps/web/pages/api/v1/public/collections } |  |  | 0.759 |
| walker |  | 7660 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/public/collections/links } |  |  | 0.760 |
| walker |  | 7664 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/public/collections/tags } |  |  | 0.762 |
| walker |  | 7682 | 18 | Fs::DirListing { dir: apps/web/pages/api/v1/preserved } |  |  | 0.766 |
| walker |  | 7701 | 19 | Fs::DirListing { dir: apps/web/pages/api/v1/archives } |  |  | 0.771 |
| walker |  | 7723 | 22 | Fs::DirListing { dir: apps/web/pages/api/v1/auth } |  |  | 0.777 |
| walker |  | 7771 | 48 | Fs::DirListing { dir: apps/web/lib/api/controllers/migration } |  |  | 0.787 |
| ns | 7965 |  | 358 | The flat components/ directory and the ui/ primitives | 6.2 |  | 0.756 |
| walker |  | 8081 | 310 | Plaintext::Whole { file: docker-compose.yml } |  |  | 0.757 |
| walker |  | 8123 | 42 | Code::CodeKey { rung: Names, file: packages/types/inputSelect.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.757 |
| walker |  | 8154 | 31 | Code::CodeKey { rung: Decl, file: packages/types/inputSelect.ts, decl: 3, sub: 0, line: 16 } |  |  | 0.757 |
| walker |  | 8189 | 35 | Code::CodeKey { rung: Decl, file: packages/types/inputSelect.ts, decl: 1, sub: 0, line: 1 } |  |  | 0.757 |
| ns | 8246 |  | 281 | Modal, link-view, preservation and input-picker component subdirectories | 6.3 | 6.2 | 0.738 |
| walker |  | 8272 | 83 | Code::CodeKey { rung: Decl, file: packages/types/inputSelect.ts, decl: 2, sub: 0, line: 7 } |  |  | 0.738 |
| walker |  | 8295 | 23 | Code::CodeKey { rung: Body, file: packages/lib/formatStats.ts, decl: 1, sub: 0, line: 4 } |  |  | 0.738 |
| walker |  | 8341 | 46 | Code::CodeKey { rung: Names, file: packages/router/worker.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.738 |
| walker |  | 8388 | 47 | Code::CodeKey { rung: Names, file: packages/lib/utils.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.738 |
| walker |  | 8400 | 12 | Code::CodeKey { rung: Body, file: packages/lib/utils.ts, decl: 3, sub: 0, line: 17 } |  |  | 0.738 |
| walker |  | 8422 | 22 | Code::CodeKey { rung: Body, file: packages/lib/utils.ts, decl: 2, sub: 0, line: 13 } |  |  | 0.738 |
| ns | 8473 |  | 227 | Web hooks, layouts, stores, ambient types, email templates, one-off migration scripts and the Playwright suite | 6.4 |  | 0.738 |
| walker |  | 8578 | 156 | Code::CodeKey { rung: Decl, file: packages/filesystem/s3Client.ts, decl: 1, sub: 0, line: 3 } |  |  | 0.738 |
| walker |  | 8629 | 51 | Code::CodeKey { rung: Names, file: packages/router/dashboardData.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.738 |
| walker |  | 8689 | 60 | Code::CodeKey { rung: Names, file: packages/router/users.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.739 |
| ns | 8713 |  | 240 | verifyUser — the guard chain every authenticated route runs first | 7.1 |  | 0.728 |
| ns | 8948 |  | 235 | docker-compose.yml — the three-container deployment | 7.2 |  | 0.732 |
| walker |  | 9066 | 377 | Json::Scripts { file: package.json } |  |  | 0.756 |
| walker |  | 9069 | 3 | Fs::DirListing { dir: apps/web/scripts } |  |  | 0.757 |
| walker |  | 9131 | 62 | Code::CodeKey { rung: Names, file: packages/router/tokens.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.757 |
| ns | 9162 |  | 214 | .env.sample — the required variables and the complete list of section headings | 7.3 |  | 0.753 |
| walker |  | 9196 | 65 | Code::CodeKey { rung: Names, file: packages/filesystem/manageFiles.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.753 |
| ns | 9341 |  | 179 | The head of the optional tuning-variable block | 7.4 | 7.3 | 0.745 |
| ns | 9473 |  | 132 | CI workflows, GitHub templates, the patch-package patch, and every translated locale | 7.5 |  | 0.750 |
| walker |  | 9513 | 317 | Fs::DirListing { dir: apps/web/components } |  |  | 0.778 |
| walker |  | 9522 | 9 | Fs::DirListing { dir: apps/web/components/LinkViews } |  |  | 0.778 |
| walker |  | 9538 | 16 | Fs::DirListing { dir: apps/web/components/InputSelect } |  |  | 0.778 |
| walker |  | 9564 | 26 | Fs::DirListing { dir: apps/web/components/Preservation } |  |  | 0.779 |
| walker |  | 9605 | 41 | Fs::DirListing { dir: apps/web/components/ui } |  |  | 0.788 |
| walker |  | 9667 | 62 | Fs::DirListing { dir: apps/web/components/LinkViews/LinkComponents } |  |  | 0.792 |
| ns | 9689 |  | 216 | apps/mobile root and the complete Expo Router screen tree | 8.1 |  | 0.797 |
| walker |  | 9835 | 168 | Fs::DirListing { dir: apps/web/components/ModalContent } |  |  | 0.815 |
| ns | 9891 |  | 202 | Mobile components, stores and query-cache modules | 8.2 | 8.1 | 0.818 |
| walker |  | 9902 | 67 | Code::CodeKey { rung: Names, file: packages/router/user.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.819 |
| walker |  | 9974 | 72 | Code::CodeKey { rung: Names, file: packages/router/highlights.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.820 |
| walker |  | 9999 | 25 | Code::CodeKey { rung: Decl, file: packages/router/highlights.tsx, decl: 1, sub: 0, line: 11 } |  |  | 0.820 |

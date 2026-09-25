Score(3000)=0.734 I=0.891 C=0.604 ns_rows≤3K=15/51 grid(1000/1442/2080/3000/4327/6240/9000)=0.633/0.712/0.686/0.734/0.624/0.791/0.829

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
| walker |  | 309 | 34 | Json::Entry { file: package.json } |  |  | 0.544 |
| ns | 321 |  | 62 | Workspace globs in the root package.json | 1.5 |  | 0.558 |
| walker |  | 363 | 54 | Fs::DirListing { dir: packages/filesystem } |  |  | 0.559 |
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
| walker |  | 926 | 8 | Fs::DirListing { dir: apps/mobile/plugins } |  |  | 0.633 |
| walker |  | 932 | 6 | Fs::DirListing { dir: apps/mobile/assets/fonts } |  |  | 0.633 |
| walker |  | 943 | 11 | Fs::DirListing { dir: apps/mobile/types } |  |  | 0.633 |
| walker |  | 955 | 12 | Fs::DirListing { dir: apps/mobile/store } |  |  | 0.633 |
| walker |  | 978 | 23 | Fs::DirListing { dir: apps/mobile/lib } |  |  | 0.633 |
| walker |  | 1014 | 36 | Fs::DirListing { dir: apps/mobile/app } |  |  | 0.634 |
| walker |  | 1020 | 6 | Fs::DirListing { dir: apps/mobile/app/links } |  |  | 0.634 |
| walker |  | 1041 | 21 | Fs::DirListing { dir: apps/mobile/app/(tabs) } |  |  | 0.634 |
| walker |  | 1052 | 11 | Fs::DirListing { dir: apps/mobile/app/(tabs)/links } |  |  | 0.634 |
| walker |  | 1076 | 24 | Fs::DirListing { dir: apps/mobile/assets/images } |  |  | 0.634 |
| ns | 1082 |  | 245 | Feature list, first half (preservation, reading, organisation, sharing) | 1.9 |  | 0.586 |
| walker |  | 1157 | 81 | Fs::DirListing { dir: apps/web } |  |  | 0.758 |
| walker |  | 1161 | 4 | Fs::DirListing { dir: apps/web/styles } |  |  | 0.758 |
| walker |  | 1170 | 9 | Fs::DirListing { dir: apps/web/store } |  |  | 0.758 |
| walker |  | 1182 | 12 | Fs::DirListing { dir: apps/web/types } |  |  | 0.758 |
| walker |  | 1195 | 13 | Fs::DirListing { dir: apps/web/lib } |  |  | 0.758 |
| walker |  | 1213 | 18 | Fs::DirListing { dir: apps/web/e2e } |  |  | 0.758 |
| walker |  | 1217 | 4 | Fs::DirListing { dir: apps/web/e2e/data } |  |  | 0.758 |
| walker |  | 1238 | 21 | Fs::DirListing { dir: apps/web/templates } |  |  | 0.759 |
| walker |  | 1244 | 6 | Fs::DirListing { dir: apps/web/e2e/tests } |  |  | 0.759 |
| walker |  | 1268 | 24 | Fs::DirListing { dir: apps/web/layouts } |  |  | 0.759 |
| ns | 1271 |  | 189 | Feature list, second half (sync, SSO, API keys, i18n, RSS, uploads) | 1.10 |  | 0.708 |
| walker |  | 1288 | 20 | Fs::DirListing { dir: apps/web/lib/shared } |  |  | 0.708 |
| walker |  | 1312 | 24 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 77 } |  |  | 0.709 |
| walker |  | 1329 | 17 | Fs::DirListing { dir: apps/mobile/app/(tabs)/collections } |  |  | 0.709 |
| walker |  | 1346 | 17 | Fs::DirListing { dir: apps/mobile/app/(tabs)/dashboard } |  |  | 0.710 |
| walker |  | 1363 | 17 | Fs::DirListing { dir: apps/mobile/app/(tabs)/settings } |  |  | 0.710 |
| walker |  | 1380 | 17 | Fs::DirListing { dir: apps/mobile/app/(tabs)/tags } |  |  | 0.710 |
| walker |  | 1429 | 49 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 90 } |  |  | 0.712 |
| walker |  | 1458 | 29 | Fs::DirListing { dir: .github } |  |  | 0.712 |
| walker |  | 1479 | 21 | Fs::DirListing { dir: .github/workflows } |  |  | 0.712 |
| walker |  | 1505 | 26 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 304 } |  |  | 0.714 |
| ns | 1539 |  | 268 | schema.prisma: datasource/generator plus every model and enum header, bodies elided | 2.1 |  | 0.666 |
| walker |  | 1540 | 35 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 83 } |  |  | 0.672 |
| ns | 1697 |  | 158 | All five schema enums, fully expanded | 2.2 | 2.1 | 0.672 |
| walker |  | 1750 | 210 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 166 } |  |  | 0.679 |
| walker |  | 1929 | 179 | Prisma::DeclTail { file: packages/prisma/schema.prisma, start_line: 166, tail_start_line: 182 } |  |  | 0.686 |
| walker |  | 2064 | 135 | Json::Dependencies { file: package.json } |  |  | 0.686 |
| ns | 2083 |  | 386 | `Link` model — every field | 2.3 | 2.1 | 0.721 |
| walker |  | 2103 | 39 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 289 } |  |  | 0.750 |
| walker |  | 2123 | 20 | Plaintext::Whole { file: apps/mobile/.env.sample } |  |  | 0.750 |
| walker |  | 2232 | 109 | Fs::DirListing { dir: packages/lib } |  |  | 0.754 |
| walker |  | 2289 | 57 | Fs::DirListing { dir: apps/worker/lib } |  |  | 0.755 |
| walker |  | 2348 | 59 | Fs::DirListing { dir: apps/mobile/components } |  |  | 0.756 |
| walker |  | 2374 | 26 | Fs::DirListing { dir: apps/mobile/components/Formats } |  |  | 0.756 |
| ns | 2396 |  | 313 | `User` model — identity, relations and subscription linkage | 2.4 | 2.1 | 0.709 |
| walker |  | 2406 | 32 | Fs::DirListing { dir: apps/mobile/components/ActionSheets } |  |  | 0.710 |
| walker |  | 2468 | 62 | Fs::DirListing { dir: apps/web/hooks } |  |  | 0.711 |
| walker |  | 2485 | 17 | Fs::DirListing { dir: apps/web/e2e/fixtures } |  |  | 0.711 |
| walker |  | 2498 | 13 | Fs::DirListing { dir: apps/web/e2e/fixtures/base } |  |  | 0.711 |
| ns | 2738 |  | 342 | `User` model — preference and archival-toggle fields | 2.5 | 2.4 | 0.675 |
| walker |  | 2783 | 285 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 28 } |  |  | 0.733 |
| walker |  | 2920 | 137 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.733 |
| walker |  | 2946 | 26 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.733 |
| walker |  | 2988 | 42 | Fs::DirListing { dir: apps/worker/lib/preservationScheme } |  |  | 0.734 |
| ns | 3055 |  | 317 | `Collection` model — every field, including the self-relation | 2.6 | 2.1 | 0.699 |
| walker |  | 3084 | 96 | Json::Runtime { file: package.json } |  |  | 0.699 |
| walker |  | 3239 | 155 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.699 |
| walker |  | 3266 | 27 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 5 } |  |  | 0.716 |
| walker |  | 3282 | 16 | Plaintext::Whole { file: apps/web/e2e/.env.example } |  |  | 0.716 |
| walker |  | 3361 | 79 | Fs::DirListing { dir: apps/web/public } |  |  | 0.716 |
| walker |  | 3371 | 10 | Fs::DirListing { dir: apps/web/public/screenshots } |  |  | 0.716 |
| ns | 3436 |  | 381 | `UsersAndCollections` join model (the permission bits) and `Tag` model | 2.7 | 2.1 | 0.675 |
| ns | 3594 |  | 158 | `AccessToken` model | 2.8 | 2.1 | 0.662 |
| ns | 3617 |  | 23 | Complete packages/prisma listing | 2.9 |  | 0.667 |
| walker |  | 3691 | 320 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 126 } |  |  | 0.717 |
| walker |  | 3713 | 22 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.718 |
| ns | 3733 |  | 116 | The process-wide `prisma` client singleton | 2.10 |  | 0.705 |
| ns | 3817 |  | 84 | API version directories: every resource under pages/api/v1 (and the single v2 route) | 3.1 |  | 0.675 |
| walker |  | 3847 | 134 | Plaintext::Whole { file: .env.sample } |  |  | 0.675 |
| ns | 3885 |  | 68 | Route files for links, collections, tags and highlights | 3.2 |  | 0.658 |
| walker |  | 3896 | 49 | Fs::DirListing { dir: apps/web/public/locales } |  |  | 0.660 |
| walker |  | 3900 | 4 | Fs::DirListing { dir: apps/web/public/locales/de } |  |  | 0.660 |
| walker |  | 3904 | 4 | Fs::DirListing { dir: apps/web/public/locales/en } |  |  | 0.660 |
| walker |  | 3908 | 4 | Fs::DirListing { dir: apps/web/public/locales/es } |  |  | 0.660 |
| walker |  | 3912 | 4 | Fs::DirListing { dir: apps/web/public/locales/fr } |  |  | 0.660 |
| walker |  | 3916 | 4 | Fs::DirListing { dir: apps/web/public/locales/it } |  |  | 0.660 |
| walker |  | 3920 | 4 | Fs::DirListing { dir: apps/web/public/locales/ja } |  |  | 0.660 |
| walker |  | 3924 | 4 | Fs::DirListing { dir: apps/web/public/locales/nl } |  |  | 0.660 |
| walker |  | 3928 | 4 | Fs::DirListing { dir: apps/web/public/locales/pl } |  |  | 0.660 |
| walker |  | 3932 | 4 | Fs::DirListing { dir: apps/web/public/locales/pt-BR } |  |  | 0.660 |
| walker |  | 3936 | 4 | Fs::DirListing { dir: apps/web/public/locales/ro } |  |  | 0.660 |
| walker |  | 3940 | 4 | Fs::DirListing { dir: apps/web/public/locales/ru } |  |  | 0.660 |
| walker |  | 3944 | 4 | Fs::DirListing { dir: apps/web/public/locales/tr } |  |  | 0.660 |
| walker |  | 3948 | 4 | Fs::DirListing { dir: apps/web/public/locales/uk } |  |  | 0.660 |
| walker |  | 3952 | 4 | Fs::DirListing { dir: apps/web/public/locales/zh } |  |  | 0.660 |
| walker |  | 3956 | 4 | Fs::DirListing { dir: apps/web/public/locales/zh-TW } |  |  | 0.660 |
| ns | 3962 |  | 77 | Route files for auth, session, users, tokens, config, avatar and logins | 3.3 |  | 0.644 |
| walker |  | 3985 | 29 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.644 |
| walker |  | 4035 | 50 | Fs::DirListing { dir: apps/mobile/components/ui } |  |  | 0.645 |
| ns | 4042 |  | 80 | Route files for archives, preserved, search, dashboard, rss, migration, payment, webhook, worker, getFavicon | 3.4 |  | 0.629 |
| ns | 4083 |  | 41 | The unauthenticated `public/` route subtree | 3.5 |  | 0.620 |
| walker |  | 4121 | 86 | Fs::DirListing { dir: apps/web/pages } |  |  | 0.621 |
| walker |  | 4127 | 6 | Fs::DirListing { dir: apps/web/pages/preserved } |  |  | 0.621 |
| walker |  | 4135 | 8 | Fs::DirListing { dir: apps/web/pages/api } |  |  | 0.621 |
| walker |  | 4138 | 3 | Fs::DirListing { dir: apps/web/pages/api/v2 } |  |  | 0.621 |
| walker |  | 4147 | 9 | Fs::DirListing { dir: apps/web/pages/public } |  |  | 0.622 |
| walker |  | 4152 | 5 | Fs::DirListing { dir: apps/web/pages/public/collections } |  |  | 0.622 |
| walker |  | 4156 | 4 | Fs::DirListing { dir: apps/web/pages/api/v2/dashboard } |  |  | 0.622 |
| walker |  | 4162 | 6 | Fs::DirListing { dir: apps/web/pages/public/links } |  |  | 0.622 |
| walker |  | 4168 | 6 | Fs::DirListing { dir: apps/web/pages/public/preserved } |  |  | 0.622 |
| walker |  | 4179 | 11 | Fs::DirListing { dir: apps/web/pages/collections } |  |  | 0.623 |
| walker |  | 4190 | 11 | Fs::DirListing { dir: apps/web/pages/tags } |  |  | 0.623 |
| walker |  | 4202 | 12 | Fs::DirListing { dir: apps/web/pages/auth } |  |  | 0.623 |
| walker |  | 4218 | 16 | Fs::DirListing { dir: apps/web/pages/links } |  |  | 0.623 |
| walker |  | 4237 | 19 | Fs::DirListing { dir: apps/web/pages/admin } |  |  | 0.623 |
| walker |  | 4247 | 10 | Fs::DirListing { dir: apps/web/pages/public/collections/[id] } |  |  | 0.624 |
| ns | 4359 |  | 276 | The route-handler pattern, read from pages/api/v1/links/index.ts | 3.6 |  | 0.607 |
| ns | 4568 |  | 209 | apps/web/lib: the server helper layer and its client/shared siblings | 3.7 |  | 0.581 |
| walker |  | 4620 | 373 | Prisma::DeclTail { file: packages/prisma/schema.prisma, start_line: 28, tail_start_line: 51 } |  |  | 0.625 |
| walker |  | 4636 | 16 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 1 } |  |  | 0.632 |
| ns | 4667 |  | 99 | The controller tree: every resource directory under lib/api/controllers | 3.8 |  | 0.616 |
| walker |  | 4692 | 56 | Fs::DirListing { dir: apps/web/pages/settings } |  |  | 0.617 |
| walker |  | 4792 | 100 | Code::CodeKey { rung: Names, file: packages/filesystem/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.617 |
| walker |  | 4804 | 12 | Code::CodeKey { rung: Names, file: apps/worker/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.617 |
| walker |  | 4816 | 12 | Code::CodeKey { rung: Names, file: packages/prisma/client/index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.617 |
| ns | 4823 |  | 156 | Controller files for links, collections, tags and highlights | 3.9 |  | 0.599 |
| walker |  | 4828 | 12 | Code::CodeKey { rung: Names, file: packages/prisma/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.599 |
| walker |  | 4904 | 76 | Code::CodeKey { rung: Decl, file: packages/prisma/index.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.607 |
| walker |  | 4930 | 26 | Code::CodeKey { rung: Names, file: apps/web/e2e/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.607 |
| walker |  | 4944 | 14 | Code::CodeKey { rung: Names, file: apps/web/pages/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.607 |
| walker |  | 4987 | 43 | Code::CodeKey { rung: Body, file: apps/web/pages/index.tsx, decl: 1, sub: 0, line: 4 } |  |  | 0.607 |
| ns | 5001 |  | 178 | Controller files for users, tokens, session, search, dashboard, worker, migration and public access | 3.10 |  | 0.588 |
| ns | 5249 |  | 248 | Complete listings of packages/types, packages/lib, packages/filesystem and packages/router | 4.1 |  | 0.616 |
| walker |  | 5422 | 435 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: true } |  |  | 0.656 |
| walker |  | 5501 | 79 | Fs::DirListing { dir: apps/web/lib/client } |  |  | 0.668 |
| walker |  | 5516 | 15 | Code::CodeKey { rung: Names, file: apps/mobile/app/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.668 |
| walker |  | 5613 | 97 | Fs::DirListing { dir: apps/web/lib/api } |  |  | 0.700 |
| walker |  | 5626 | 13 | Fs::DirListing { dir: apps/web/lib/api/archives } |  |  | 0.701 |
| walker |  | 5643 | 17 | Fs::DirListing { dir: apps/web/lib/api/preserved } |  |  | 0.701 |
| ns | 5647 |  | 398 | packages/types/global.ts — every exported type, interface and enum declaration | 4.2 |  | 0.683 |
| walker |  | 5676 | 33 | Fs::DirListing { dir: apps/web/lib/api/stripe } |  |  | 0.687 |
| walker |  | 5712 | 36 | Fs::DirListing { dir: apps/web/lib/api/controllers } |  |  | 0.707 |
| walker |  | 5717 | 5 | Fs::DirListing { dir: apps/web/lib/api/controllers/search } |  |  | 0.707 |
| walker |  | 5722 | 5 | Fs::DirListing { dir: apps/web/lib/api/controllers/session } |  |  | 0.707 |
| walker |  | 5728 | 6 | Fs::DirListing { dir: apps/web/lib/api/controllers/worker } |  |  | 0.707 |
| walker |  | 5737 | 9 | Fs::DirListing { dir: apps/web/lib/api/controllers/public } |  |  | 0.708 |
| walker |  | 5741 | 4 | Fs::DirListing { dir: apps/web/lib/api/controllers/public/links } |  |  | 0.708 |
| walker |  | 5747 | 6 | Fs::DirListing { dir: apps/web/lib/api/controllers/public/collections } |  |  | 0.709 |
| walker |  | 5753 | 6 | Fs::DirListing { dir: apps/web/lib/api/controllers/public/users } |  |  | 0.709 |
| walker |  | 5760 | 7 | Fs::DirListing { dir: apps/web/lib/api/controllers/public/links/linkId } |  |  | 0.710 |
| walker |  | 5774 | 14 | Fs::DirListing { dir: apps/web/lib/api/controllers/collections } |  |  | 0.710 |
| walker |  | 5788 | 14 | Fs::DirListing { dir: apps/web/lib/api/controllers/highlights } |  |  | 0.711 |
| ns | 5793 |  | 146 | `ArchivedFormat`, `LinkType` and `TokenExpiry` variants | 4.3 | 4.2 | 0.702 |
| walker |  | 5802 | 14 | Fs::DirListing { dir: apps/web/lib/api/controllers/tokens } |  |  | 0.705 |
| walker |  | 5809 | 7 | Fs::DirListing { dir: apps/web/lib/api/controllers/tokens/tokenId } |  |  | 0.706 |
| walker |  | 5823 | 14 | Fs::DirListing { dir: apps/web/lib/api/controllers/users } |  |  | 0.709 |
| walker |  | 5840 | 17 | Fs::DirListing { dir: apps/web/lib/api/controllers/links } |  |  | 0.711 |
| walker |  | 5852 | 12 | Fs::DirListing { dir: apps/web/lib/api/controllers/links/bulk } |  |  | 0.713 |
| walker |  | 5872 | 20 | Fs::DirListing { dir: apps/web/lib/api/controllers/dashboard } |  |  | 0.716 |
| walker |  | 5899 | 27 | Fs::DirListing { dir: apps/web/lib/api/controllers/tags } |  |  | 0.722 |
| walker |  | 5920 | 21 | Fs::DirListing { dir: apps/web/lib/api/controllers/collections/collectionId } |  |  | 0.726 |
| walker |  | 5941 | 21 | Fs::DirListing { dir: apps/web/lib/api/controllers/tags/tagId } |  |  | 0.731 |
| ns | 5945 |  | 152 | `ViewMode`, `Sort` and `TagSort` variants | 4.4 | 4.2 | 0.723 |
| walker |  | 5965 | 24 | Fs::DirListing { dir: apps/web/lib/api/controllers/links/linkId } |  |  | 0.730 |
| walker |  | 5971 | 6 | Fs::DirListing { dir: apps/web/lib/api/controllers/links/linkId/highlight } |  |  | 0.732 |
| walker |  | 5998 | 27 | Fs::DirListing { dir: apps/web/lib/api/controllers/users/userId } |  |  | 0.738 |
| walker |  | 6067 | 69 | Fs::DirListing { dir: apps/web/pages/api/v1 } |  |  | 0.765 |
| walker |  | 6071 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/dashboard } |  |  | 0.765 |
| walker |  | 6075 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/getFavicon } |  |  | 0.766 |
| walker |  | 6079 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/logins } |  |  | 0.766 |
| walker |  | 6083 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/migration } |  |  | 0.766 |
| walker |  | 6087 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/payment } |  |  | 0.766 |
| walker |  | 6091 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/search } |  |  | 0.767 |
| walker |  | 6095 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/session } |  |  | 0.767 |
| walker |  | 6099 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/webhook } |  |  | 0.768 |
| walker |  | 6105 | 6 | Fs::DirListing { dir: apps/web/pages/api/v1/avatar } |  |  | 0.768 |
| walker |  | 6114 | 9 | Fs::DirListing { dir: apps/web/pages/api/v1/config } |  |  | 0.769 |
| walker |  | 6123 | 9 | Fs::DirListing { dir: apps/web/pages/api/v1/public } |  |  | 0.770 |
| walker |  | 6129 | 6 | Fs::DirListing { dir: apps/web/pages/api/v1/public/links } |  |  | 0.771 |
| walker |  | 6135 | 6 | Fs::DirListing { dir: apps/web/pages/api/v1/public/users } |  |  | 0.772 |
| walker |  | 6144 | 9 | Fs::DirListing { dir: apps/web/pages/api/v1/worker } |  |  | 0.773 |
| walker |  | 6154 | 10 | Fs::DirListing { dir: apps/web/pages/api/v1/collections } |  |  | 0.774 |
| walker |  | 6164 | 10 | Fs::DirListing { dir: apps/web/pages/api/v1/highlights } |  |  | 0.775 |
| walker |  | 6174 | 10 | Fs::DirListing { dir: apps/web/pages/api/v1/rss } |  |  | 0.777 |
| walker |  | 6184 | 10 | Fs::DirListing { dir: apps/web/pages/api/v1/tokens } |  |  | 0.778 |
| walker |  | 6196 | 12 | Fs::DirListing { dir: apps/web/pages/api/v1/links } |  |  | 0.780 |
| walker |  | 6200 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/links/archive } |  |  | 0.781 |
| walker |  | 6213 | 13 | Fs::DirListing { dir: apps/web/pages/api/v1/users } |  |  | 0.785 |
| walker |  | 6222 | 9 | Fs::DirListing { dir: apps/web/pages/api/v1/users/[id] } |  |  | 0.788 |
| walker |  | 6236 | 14 | Fs::DirListing { dir: apps/web/pages/api/v1/tags } |  |  | 0.791 |
| walker |  | 6246 | 10 | Fs::DirListing { dir: apps/web/pages/api/v1/links/[id] } |  |  | 0.796 |
| walker |  | 6250 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/links/[id]/archive } |  |  | 0.798 |
| walker |  | 6254 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/links/[id]/highlights } |  |  | 0.800 |
| walker |  | 6266 | 12 | Fs::DirListing { dir: apps/web/pages/api/v1/public/collections } |  |  | 0.804 |
| walker |  | 6270 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/public/collections/links } |  |  | 0.806 |
| walker |  | 6274 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/public/collections/tags } |  |  | 0.808 |
| ns | 6285 |  | 340 | packages/lib/schemaValidation.ts — every exported zod schema constant | 4.5 |  | 0.793 |
| walker |  | 6292 | 18 | Fs::DirListing { dir: apps/web/pages/api/v1/preserved } |  |  | 0.798 |
| walker |  | 6311 | 19 | Fs::DirListing { dir: apps/web/pages/api/v1/archives } |  |  | 0.804 |
| walker |  | 6333 | 22 | Fs::DirListing { dir: apps/web/pages/api/v1/auth } |  |  | 0.811 |
| walker |  | 6381 | 48 | Fs::DirListing { dir: apps/web/lib/api/controllers/migration } |  |  | 0.823 |
| ns | 6542 |  | 257 | packages/router — the links, collections and tags hook exports | 4.6 |  | 0.804 |
| walker |  | 6691 | 310 | Plaintext::Whole { file: docker-compose.yml } |  |  | 0.805 |
| ns | 6726 |  | 184 | packages/router — export lines of the remaining ten hook modules | 4.7 | 4.6 | 0.799 |
| walker |  | 6792 | 101 | Code::CodeKey { rung: Body, file: apps/worker/index.ts, decl: 1, sub: 0, line: 3 } |  |  | 0.799 |
| ns | 6892 |  | 166 | Complete apps/worker listing, including every job and every preservation handler | 5.1 |  | 0.804 |
| walker |  | 6927 | 135 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.804 |
| ns | 7161 |  | 269 | worker.ts — the whole scheduler entry point | 5.2 |  | 0.791 |
| walker |  | 7304 | 377 | Json::Scripts { file: package.json } |  |  | 0.818 |
| walker |  | 7307 | 3 | Fs::DirListing { dir: apps/web/scripts } |  |  | 0.818 |
| ns | 7354 |  | 193 | archiveHandler's signature and its SSRF / skip-preservation guard | 5.3 |  | 0.807 |
| ns | 7607 |  | 253 | Every UI page route under apps/web/pages | 6.1 |  | 0.814 |
| walker |  | 7624 | 317 | Fs::DirListing { dir: apps/web/components } |  |  | 0.818 |
| walker |  | 7633 | 9 | Fs::DirListing { dir: apps/web/components/LinkViews } |  |  | 0.818 |
| walker |  | 7649 | 16 | Fs::DirListing { dir: apps/web/components/InputSelect } |  |  | 0.818 |
| walker |  | 7675 | 26 | Fs::DirListing { dir: apps/web/components/Preservation } |  |  | 0.818 |
| walker |  | 7716 | 41 | Fs::DirListing { dir: apps/web/components/ui } |  |  | 0.819 |
| walker |  | 7778 | 62 | Fs::DirListing { dir: apps/web/components/LinkViews/LinkComponents } |  |  | 0.819 |
| walker |  | 7946 | 168 | Fs::DirListing { dir: apps/web/components/ModalContent } |  |  | 0.821 |
| ns | 7965 |  | 358 | The flat components/ directory and the ui/ primitives | 6.2 |  | 0.829 |
| walker |  | 8034 | 88 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.829 |
| walker |  | 8138 | 104 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.829 |
| walker |  | 8180 | 42 | Json::Identity { file: packages/lib/package.json } |  |  | 0.829 |
| walker |  | 8192 | 12 | Json::Entry { file: packages/lib/package.json } |  |  | 0.829 |
| walker |  | 8234 | 42 | Json::Identity { file: packages/router/package.json } |  |  | 0.829 |
| walker |  | 8246 | 12 | Json::Entry { file: packages/router/package.json } |  |  | 0.834 |
| ns | 8246 |  | 281 | Modal, link-view, preservation and input-picker component subdirectories | 6.3 | 6.2 | 0.834 |
| walker |  | 8288 | 42 | Json::Identity { file: packages/types/package.json } |  |  | 0.834 |
| walker |  | 8300 | 12 | Json::Entry { file: packages/types/package.json } |  |  | 0.834 |
| walker |  | 8343 | 43 | Json::Dependencies { file: packages/types/package.json } |  |  | 0.834 |
| walker |  | 8386 | 43 | Json::Identity { file: apps/worker/package.json } |  |  | 0.836 |
| walker |  | 8398 | 12 | Json::Entry { file: apps/worker/package.json } |  |  | 0.836 |
| walker |  | 8456 | 58 | Json::Scripts { file: apps/worker/package.json } |  |  | 0.836 |
| ns | 8473 |  | 227 | Web hooks, layouts, stores, ambient types, email templates, one-off migration scripts and the Playwright suite | 6.4 |  | 0.833 |
| walker |  | 8499 | 43 | Json::Identity { file: packages/filesystem/package.json } |  |  | 0.835 |
| walker |  | 8511 | 12 | Json::Entry { file: packages/filesystem/package.json } |  |  | 0.835 |
| walker |  | 8545 | 34 | Json::Dependencies { file: packages/filesystem/package.json } |  |  | 0.835 |
| walker |  | 8588 | 43 | Json::Identity { file: packages/prisma/package.json } |  |  | 0.837 |
| walker |  | 8615 | 27 | Json::Entry { file: packages/prisma/package.json } |  |  | 0.837 |
| walker |  | 8686 | 71 | Json::Scripts { file: packages/prisma/package.json } |  |  | 0.837 |
| ns | 8713 |  | 240 | verifyUser — the guard chain every authenticated route runs first | 7.1 |  | 0.825 |
| walker |  | 8732 | 46 | Json::Dependencies { file: packages/prisma/package.json } |  |  | 0.825 |
| walker |  | 8778 | 46 | Json::Identity { file: apps/mobile/package.json } |  |  | 0.827 |
| walker |  | 8790 | 12 | Json::Entry { file: apps/mobile/package.json } |  |  | 0.827 |
| walker |  | 8885 | 95 | Json::Scripts { file: apps/mobile/package.json } |  |  | 0.827 |
| ns | 8948 |  | 235 | docker-compose.yml — the three-container deployment | 7.2 |  | 0.829 |
| ns | 9162 |  | 214 | .env.sample — the required variables and the complete list of section headings | 7.3 |  | 0.824 |
| ns | 9341 |  | 179 | The head of the optional tuning-variable block | 7.4 | 7.3 | 0.815 |
| ns | 9473 |  | 132 | CI workflows, GitHub templates, the patch-package patch, and every translated locale | 7.5 |  | 0.818 |
| ns | 9689 |  | 216 | apps/mobile root and the complete Expo Router screen tree | 8.1 |  | 0.822 |
| walker |  | 9704 | 819 | Plaintext::Whole { file: Dockerfile } |  |  | 0.822 |
| walker |  | 9728 | 24 | Code::CodeKey { rung: Names, file: packages/lib/getOriginalFormat.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.822 |
| walker |  | 9754 | 26 | Code::CodeKey { rung: Decl, file: packages/lib/getOriginalFormat.ts, decl: 1, sub: 0, line: 6 } |  |  | 0.822 |
| ns | 9891 |  | 202 | Mobile components, stores and query-cache modules | 8.2 | 8.1 | 0.825 |

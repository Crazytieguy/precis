Score(3000)=0.733 I=0.888 C=0.604 ns_rows≤3K=15/51 grid(1000/1442/2080/3000/4327/6240/9000)=0.633/0.712/0.686/0.733/0.620/0.767/0.830

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
| walker |  | 2883 | 100 | Code::CodeKey { rung: Names, file: packages/filesystem/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.733 |
| walker |  | 3020 | 137 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.733 |
| walker |  | 3046 | 26 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.733 |
| ns | 3055 |  | 317 | `Collection` model — every field, including the self-relation | 2.6 | 2.1 | 0.698 |
| walker |  | 3088 | 42 | Fs::DirListing { dir: apps/worker/lib/preservationScheme } |  |  | 0.699 |
| walker |  | 3100 | 12 | Code::CodeKey { rung: Names, file: apps/worker/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.699 |
| walker |  | 3196 | 96 | Json::Runtime { file: package.json } |  |  | 0.699 |
| walker |  | 3351 | 155 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.699 |
| ns | 3436 |  | 381 | `UsersAndCollections` join model (the permission bits) and `Tag` model | 2.7 | 2.1 | 0.660 |
| walker |  | 3486 | 135 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.660 |
| walker |  | 3513 | 27 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 5 } |  |  | 0.675 |
| walker |  | 3529 | 16 | Plaintext::Whole { file: apps/web/e2e/.env.example } |  |  | 0.675 |
| ns | 3594 |  | 158 | `AccessToken` model | 2.8 | 2.1 | 0.662 |
| walker |  | 3608 | 79 | Fs::DirListing { dir: apps/web/public } |  |  | 0.662 |
| ns | 3617 |  | 23 | Complete packages/prisma listing | 2.9 |  | 0.667 |
| walker |  | 3618 | 10 | Fs::DirListing { dir: apps/web/public/screenshots } |  |  | 0.667 |
| walker |  | 3644 | 26 | Code::CodeKey { rung: Names, file: apps/web/e2e/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.667 |
| ns | 3733 |  | 116 | The process-wide `prisma` client singleton | 2.10 |  | 0.655 |
| ns | 3817 |  | 84 | API version directories: every resource under pages/api/v1 (and the single v2 route) | 3.1 |  | 0.627 |
| ns | 3885 |  | 68 | Route files for links, collections, tags and highlights | 3.2 |  | 0.611 |
| ns | 3962 |  | 77 | Route files for auth, session, users, tokens, config, avatar and logins | 3.3 |  | 0.597 |
| walker |  | 3964 | 320 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 126 } |  |  | 0.642 |
| walker |  | 3986 | 22 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.642 |
| ns | 4042 |  | 80 | Route files for archives, preserved, search, dashboard, rss, migration, payment, webhook, worker, getFavicon | 3.4 |  | 0.627 |
| ns | 4083 |  | 41 | The unauthenticated `public/` route subtree | 3.5 |  | 0.618 |
| walker |  | 4120 | 134 | Plaintext::Whole { file: .env.sample } |  |  | 0.618 |
| walker |  | 4169 | 49 | Fs::DirListing { dir: apps/web/public/locales } |  |  | 0.620 |
| walker |  | 4173 | 4 | Fs::DirListing { dir: apps/web/public/locales/de } |  |  | 0.620 |
| walker |  | 4177 | 4 | Fs::DirListing { dir: apps/web/public/locales/en } |  |  | 0.620 |
| walker |  | 4181 | 4 | Fs::DirListing { dir: apps/web/public/locales/es } |  |  | 0.620 |
| walker |  | 4185 | 4 | Fs::DirListing { dir: apps/web/public/locales/fr } |  |  | 0.620 |
| walker |  | 4189 | 4 | Fs::DirListing { dir: apps/web/public/locales/it } |  |  | 0.620 |
| walker |  | 4193 | 4 | Fs::DirListing { dir: apps/web/public/locales/ja } |  |  | 0.620 |
| walker |  | 4197 | 4 | Fs::DirListing { dir: apps/web/public/locales/nl } |  |  | 0.620 |
| walker |  | 4201 | 4 | Fs::DirListing { dir: apps/web/public/locales/pl } |  |  | 0.620 |
| walker |  | 4205 | 4 | Fs::DirListing { dir: apps/web/public/locales/pt-BR } |  |  | 0.620 |
| walker |  | 4209 | 4 | Fs::DirListing { dir: apps/web/public/locales/ro } |  |  | 0.620 |
| walker |  | 4213 | 4 | Fs::DirListing { dir: apps/web/public/locales/ru } |  |  | 0.620 |
| walker |  | 4217 | 4 | Fs::DirListing { dir: apps/web/public/locales/tr } |  |  | 0.620 |
| walker |  | 4221 | 4 | Fs::DirListing { dir: apps/web/public/locales/uk } |  |  | 0.620 |
| walker |  | 4225 | 4 | Fs::DirListing { dir: apps/web/public/locales/zh } |  |  | 0.620 |
| walker |  | 4229 | 4 | Fs::DirListing { dir: apps/web/public/locales/zh-TW } |  |  | 0.620 |
| walker |  | 4258 | 29 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.620 |
| walker |  | 4308 | 50 | Fs::DirListing { dir: apps/mobile/components/ui } |  |  | 0.620 |
| ns | 4359 |  | 276 | The route-handler pattern, read from pages/api/v1/links/index.ts | 3.6 |  | 0.604 |
| walker |  | 4394 | 86 | Fs::DirListing { dir: apps/web/pages } |  |  | 0.604 |
| walker |  | 4400 | 6 | Fs::DirListing { dir: apps/web/pages/preserved } |  |  | 0.604 |
| walker |  | 4408 | 8 | Fs::DirListing { dir: apps/web/pages/api } |  |  | 0.605 |
| walker |  | 4411 | 3 | Fs::DirListing { dir: apps/web/pages/api/v2 } |  |  | 0.605 |
| walker |  | 4420 | 9 | Fs::DirListing { dir: apps/web/pages/public } |  |  | 0.605 |
| walker |  | 4425 | 5 | Fs::DirListing { dir: apps/web/pages/public/collections } |  |  | 0.605 |
| walker |  | 4429 | 4 | Fs::DirListing { dir: apps/web/pages/api/v2/dashboard } |  |  | 0.606 |
| walker |  | 4435 | 6 | Fs::DirListing { dir: apps/web/pages/public/links } |  |  | 0.606 |
| walker |  | 4441 | 6 | Fs::DirListing { dir: apps/web/pages/public/preserved } |  |  | 0.606 |
| walker |  | 4452 | 11 | Fs::DirListing { dir: apps/web/pages/collections } |  |  | 0.606 |
| walker |  | 4463 | 11 | Fs::DirListing { dir: apps/web/pages/tags } |  |  | 0.606 |
| walker |  | 4475 | 12 | Fs::DirListing { dir: apps/web/pages/auth } |  |  | 0.606 |
| walker |  | 4491 | 16 | Fs::DirListing { dir: apps/web/pages/links } |  |  | 0.607 |
| walker |  | 4510 | 19 | Fs::DirListing { dir: apps/web/pages/admin } |  |  | 0.607 |
| walker |  | 4520 | 10 | Fs::DirListing { dir: apps/web/pages/public/collections/[id] } |  |  | 0.607 |
| walker |  | 4534 | 14 | Code::CodeKey { rung: Names, file: apps/web/pages/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.607 |
| ns | 4568 |  | 209 | apps/web/lib: the server helper layer and its client/shared siblings | 3.7 |  | 0.581 |
| walker |  | 4577 | 43 | Code::CodeKey { rung: Body, file: apps/web/pages/index.tsx, decl: 1, sub: 0, line: 4 } |  |  | 0.581 |
| ns | 4667 |  | 99 | The controller tree: every resource directory under lib/api/controllers | 3.8 |  | 0.566 |
| ns | 4823 |  | 156 | Controller files for links, collections, tags and highlights | 3.9 |  | 0.549 |
| walker |  | 4950 | 373 | Prisma::DeclTail { file: packages/prisma/schema.prisma, start_line: 28, tail_start_line: 51 } |  |  | 0.590 |
| walker |  | 4966 | 16 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 1 } |  |  | 0.597 |
| walker |  | 4981 | 15 | Code::CodeKey { rung: Names, file: apps/mobile/app/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.597 |
| ns | 5001 |  | 178 | Controller files for users, tokens, session, search, dashboard, worker, migration and public access | 3.10 |  | 0.578 |
| walker |  | 5037 | 56 | Fs::DirListing { dir: apps/web/pages/settings } |  |  | 0.579 |
| walker |  | 5074 | 37 | Code::CodeKey { rung: Names, file: packages/prisma/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.579 |
| walker |  | 5150 | 76 | Code::CodeKey { rung: Decl, file: packages/prisma/index.ts, decl: 2, sub: 0, line: 5 } |  |  | 0.588 |
| ns | 5249 |  | 248 | Complete listings of packages/types, packages/lib, packages/filesystem and packages/router | 4.1 |  | 0.616 |
| walker |  | 5585 | 435 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: true } |  |  | 0.656 |
| ns | 5647 |  | 398 | packages/types/global.ts — every exported type, interface and enum declaration | 4.2 |  | 0.639 |
| walker |  | 5664 | 79 | Fs::DirListing { dir: apps/web/lib/client } |  |  | 0.650 |
| walker |  | 5761 | 97 | Fs::DirListing { dir: apps/web/lib/api } |  |  | 0.682 |
| walker |  | 5774 | 13 | Fs::DirListing { dir: apps/web/lib/api/archives } |  |  | 0.682 |
| walker |  | 5791 | 17 | Fs::DirListing { dir: apps/web/lib/api/preserved } |  |  | 0.683 |
| ns | 5793 |  | 146 | `ArchivedFormat`, `LinkType` and `TokenExpiry` variants | 4.3 | 4.2 | 0.674 |
| walker |  | 5824 | 33 | Fs::DirListing { dir: apps/web/lib/api/stripe } |  |  | 0.678 |
| walker |  | 5860 | 36 | Fs::DirListing { dir: apps/web/lib/api/controllers } |  |  | 0.698 |
| walker |  | 5865 | 5 | Fs::DirListing { dir: apps/web/lib/api/controllers/search } |  |  | 0.698 |
| walker |  | 5870 | 5 | Fs::DirListing { dir: apps/web/lib/api/controllers/session } |  |  | 0.698 |
| walker |  | 5876 | 6 | Fs::DirListing { dir: apps/web/lib/api/controllers/worker } |  |  | 0.698 |
| walker |  | 5885 | 9 | Fs::DirListing { dir: apps/web/lib/api/controllers/public } |  |  | 0.699 |
| walker |  | 5889 | 4 | Fs::DirListing { dir: apps/web/lib/api/controllers/public/links } |  |  | 0.699 |
| walker |  | 5895 | 6 | Fs::DirListing { dir: apps/web/lib/api/controllers/public/collections } |  |  | 0.700 |
| walker |  | 5901 | 6 | Fs::DirListing { dir: apps/web/lib/api/controllers/public/users } |  |  | 0.701 |
| walker |  | 5908 | 7 | Fs::DirListing { dir: apps/web/lib/api/controllers/public/links/linkId } |  |  | 0.701 |
| walker |  | 5922 | 14 | Fs::DirListing { dir: apps/web/lib/api/controllers/collections } |  |  | 0.702 |
| walker |  | 5936 | 14 | Fs::DirListing { dir: apps/web/lib/api/controllers/highlights } |  |  | 0.702 |
| ns | 5945 |  | 152 | `ViewMode`, `Sort` and `TagSort` variants | 4.4 | 4.2 | 0.694 |
| walker |  | 5950 | 14 | Fs::DirListing { dir: apps/web/lib/api/controllers/tokens } |  |  | 0.697 |
| walker |  | 5957 | 7 | Fs::DirListing { dir: apps/web/lib/api/controllers/tokens/tokenId } |  |  | 0.698 |
| walker |  | 5971 | 14 | Fs::DirListing { dir: apps/web/lib/api/controllers/users } |  |  | 0.701 |
| walker |  | 5988 | 17 | Fs::DirListing { dir: apps/web/lib/api/controllers/links } |  |  | 0.703 |
| walker |  | 6000 | 12 | Fs::DirListing { dir: apps/web/lib/api/controllers/links/bulk } |  |  | 0.705 |
| walker |  | 6020 | 20 | Fs::DirListing { dir: apps/web/lib/api/controllers/dashboard } |  |  | 0.708 |
| walker |  | 6047 | 27 | Fs::DirListing { dir: apps/web/lib/api/controllers/tags } |  |  | 0.714 |
| walker |  | 6068 | 21 | Fs::DirListing { dir: apps/web/lib/api/controllers/collections/collectionId } |  |  | 0.718 |
| walker |  | 6089 | 21 | Fs::DirListing { dir: apps/web/lib/api/controllers/tags/tagId } |  |  | 0.723 |
| walker |  | 6113 | 24 | Fs::DirListing { dir: apps/web/lib/api/controllers/links/linkId } |  |  | 0.730 |
| walker |  | 6119 | 6 | Fs::DirListing { dir: apps/web/lib/api/controllers/links/linkId/highlight } |  |  | 0.732 |
| walker |  | 6146 | 27 | Fs::DirListing { dir: apps/web/lib/api/controllers/users/userId } |  |  | 0.738 |
| walker |  | 6215 | 69 | Fs::DirListing { dir: apps/web/pages/api/v1 } |  |  | 0.765 |
| walker |  | 6219 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/dashboard } |  |  | 0.765 |
| walker |  | 6223 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/getFavicon } |  |  | 0.766 |
| walker |  | 6227 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/logins } |  |  | 0.766 |
| walker |  | 6231 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/migration } |  |  | 0.766 |
| walker |  | 6235 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/payment } |  |  | 0.766 |
| walker |  | 6239 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/search } |  |  | 0.767 |
| walker |  | 6243 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/session } |  |  | 0.767 |
| walker |  | 6247 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/webhook } |  |  | 0.768 |
| walker |  | 6253 | 6 | Fs::DirListing { dir: apps/web/pages/api/v1/avatar } |  |  | 0.768 |
| walker |  | 6262 | 9 | Fs::DirListing { dir: apps/web/pages/api/v1/config } |  |  | 0.769 |
| walker |  | 6271 | 9 | Fs::DirListing { dir: apps/web/pages/api/v1/public } |  |  | 0.770 |
| walker |  | 6277 | 6 | Fs::DirListing { dir: apps/web/pages/api/v1/public/links } |  |  | 0.771 |
| walker |  | 6283 | 6 | Fs::DirListing { dir: apps/web/pages/api/v1/public/users } |  |  | 0.772 |
| ns | 6285 |  | 340 | packages/lib/schemaValidation.ts — every exported zod schema constant | 4.5 |  | 0.757 |
| walker |  | 6292 | 9 | Fs::DirListing { dir: apps/web/pages/api/v1/worker } |  |  | 0.759 |
| walker |  | 6302 | 10 | Fs::DirListing { dir: apps/web/pages/api/v1/collections } |  |  | 0.759 |
| walker |  | 6312 | 10 | Fs::DirListing { dir: apps/web/pages/api/v1/highlights } |  |  | 0.760 |
| walker |  | 6322 | 10 | Fs::DirListing { dir: apps/web/pages/api/v1/rss } |  |  | 0.762 |
| walker |  | 6332 | 10 | Fs::DirListing { dir: apps/web/pages/api/v1/tokens } |  |  | 0.764 |
| walker |  | 6344 | 12 | Fs::DirListing { dir: apps/web/pages/api/v1/links } |  |  | 0.766 |
| walker |  | 6348 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/links/archive } |  |  | 0.767 |
| walker |  | 6361 | 13 | Fs::DirListing { dir: apps/web/pages/api/v1/users } |  |  | 0.770 |
| walker |  | 6370 | 9 | Fs::DirListing { dir: apps/web/pages/api/v1/users/[id] } |  |  | 0.773 |
| walker |  | 6384 | 14 | Fs::DirListing { dir: apps/web/pages/api/v1/tags } |  |  | 0.777 |
| walker |  | 6394 | 10 | Fs::DirListing { dir: apps/web/pages/api/v1/links/[id] } |  |  | 0.781 |
| walker |  | 6398 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/links/[id]/archive } |  |  | 0.783 |
| walker |  | 6402 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/links/[id]/highlights } |  |  | 0.785 |
| walker |  | 6414 | 12 | Fs::DirListing { dir: apps/web/pages/api/v1/public/collections } |  |  | 0.789 |
| walker |  | 6418 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/public/collections/links } |  |  | 0.791 |
| walker |  | 6422 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/public/collections/tags } |  |  | 0.793 |
| walker |  | 6440 | 18 | Fs::DirListing { dir: apps/web/pages/api/v1/preserved } |  |  | 0.798 |
| walker |  | 6459 | 19 | Fs::DirListing { dir: apps/web/pages/api/v1/archives } |  |  | 0.804 |
| walker |  | 6481 | 22 | Fs::DirListing { dir: apps/web/pages/api/v1/auth } |  |  | 0.811 |
| walker |  | 6529 | 48 | Fs::DirListing { dir: apps/web/lib/api/controllers/migration } |  |  | 0.823 |
| ns | 6542 |  | 257 | packages/router — the links, collections and tags hook exports | 4.6 |  | 0.804 |
| ns | 6726 |  | 184 | packages/router — export lines of the remaining ten hook modules | 4.7 | 4.6 | 0.797 |
| walker |  | 6839 | 310 | Plaintext::Whole { file: docker-compose.yml } |  |  | 0.799 |
| ns | 6892 |  | 166 | Complete apps/worker listing, including every job and every preservation handler | 5.1 |  | 0.804 |
| ns | 7161 |  | 269 | worker.ts — the whole scheduler entry point | 5.2 |  | 0.791 |
| walker |  | 7216 | 377 | Json::Scripts { file: package.json } |  |  | 0.818 |
| walker |  | 7219 | 3 | Fs::DirListing { dir: apps/web/scripts } |  |  | 0.818 |
| ns | 7354 |  | 193 | archiveHandler's signature and its SSRF / skip-preservation guard | 5.3 |  | 0.807 |
| walker |  | 7536 | 317 | Fs::DirListing { dir: apps/web/components } |  |  | 0.810 |
| walker |  | 7545 | 9 | Fs::DirListing { dir: apps/web/components/LinkViews } |  |  | 0.810 |
| walker |  | 7561 | 16 | Fs::DirListing { dir: apps/web/components/InputSelect } |  |  | 0.810 |
| walker |  | 7587 | 26 | Fs::DirListing { dir: apps/web/components/Preservation } |  |  | 0.810 |
| ns | 7607 |  | 253 | Every UI page route under apps/web/pages | 6.1 |  | 0.818 |
| walker |  | 7628 | 41 | Fs::DirListing { dir: apps/web/components/ui } |  |  | 0.819 |
| walker |  | 7690 | 62 | Fs::DirListing { dir: apps/web/components/LinkViews/LinkComponents } |  |  | 0.819 |
| walker |  | 7858 | 168 | Fs::DirListing { dir: apps/web/components/ModalContent } |  |  | 0.821 |
| walker |  | 7959 | 101 | Code::CodeKey { rung: Body, file: apps/worker/index.ts, decl: 1, sub: 0, line: 3 } |  |  | 0.821 |
| ns | 7965 |  | 358 | The flat components/ directory and the ui/ primitives | 6.2 |  | 0.829 |
| walker |  | 8047 | 88 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.829 |
| walker |  | 8151 | 104 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.829 |
| walker |  | 8193 | 42 | Json::Identity { file: packages/lib/package.json } |  |  | 0.829 |
| walker |  | 8205 | 12 | Json::Entry { file: packages/lib/package.json } |  |  | 0.829 |
| ns | 8246 |  | 281 | Modal, link-view, preservation and input-picker component subdirectories | 6.3 | 6.2 | 0.833 |
| walker |  | 8247 | 42 | Json::Identity { file: packages/router/package.json } |  |  | 0.834 |
| walker |  | 8259 | 12 | Json::Entry { file: packages/router/package.json } |  |  | 0.834 |
| walker |  | 8301 | 42 | Json::Identity { file: packages/types/package.json } |  |  | 0.834 |
| walker |  | 8313 | 12 | Json::Entry { file: packages/types/package.json } |  |  | 0.834 |
| walker |  | 8356 | 43 | Json::Dependencies { file: packages/types/package.json } |  |  | 0.834 |
| walker |  | 8399 | 43 | Json::Identity { file: apps/worker/package.json } |  |  | 0.836 |
| walker |  | 8411 | 12 | Json::Entry { file: apps/worker/package.json } |  |  | 0.836 |
| walker |  | 8469 | 58 | Json::Scripts { file: apps/worker/package.json } |  |  | 0.836 |
| ns | 8473 |  | 227 | Web hooks, layouts, stores, ambient types, email templates, one-off migration scripts and the Playwright suite | 6.4 |  | 0.833 |
| walker |  | 8512 | 43 | Json::Identity { file: packages/filesystem/package.json } |  |  | 0.835 |
| walker |  | 8524 | 12 | Json::Entry { file: packages/filesystem/package.json } |  |  | 0.835 |
| walker |  | 8558 | 34 | Json::Dependencies { file: packages/filesystem/package.json } |  |  | 0.835 |
| walker |  | 8601 | 43 | Json::Identity { file: packages/prisma/package.json } |  |  | 0.837 |
| walker |  | 8628 | 27 | Json::Entry { file: packages/prisma/package.json } |  |  | 0.837 |
| walker |  | 8699 | 71 | Json::Scripts { file: packages/prisma/package.json } |  |  | 0.837 |
| ns | 8713 |  | 240 | verifyUser — the guard chain every authenticated route runs first | 7.1 |  | 0.825 |
| walker |  | 8745 | 46 | Json::Dependencies { file: packages/prisma/package.json } |  |  | 0.825 |
| walker |  | 8791 | 46 | Json::Identity { file: apps/mobile/package.json } |  |  | 0.827 |
| walker |  | 8803 | 12 | Json::Entry { file: apps/mobile/package.json } |  |  | 0.827 |
| walker |  | 8898 | 95 | Json::Scripts { file: apps/mobile/package.json } |  |  | 0.827 |
| ns | 8948 |  | 235 | docker-compose.yml — the three-container deployment | 7.2 |  | 0.830 |
| ns | 9162 |  | 214 | .env.sample — the required variables and the complete list of section headings | 7.3 |  | 0.824 |
| ns | 9341 |  | 179 | The head of the optional tuning-variable block | 7.4 | 7.3 | 0.815 |
| ns | 9473 |  | 132 | CI workflows, GitHub templates, the patch-package patch, and every translated locale | 7.5 |  | 0.818 |
| ns | 9689 |  | 216 | apps/mobile root and the complete Expo Router screen tree | 8.1 |  | 0.822 |
| walker |  | 9717 | 819 | Plaintext::Whole { file: Dockerfile } |  |  | 0.822 |
| ns | 9891 |  | 202 | Mobile components, stores and query-cache modules | 8.2 | 8.1 | 0.825 |
| walker |  | 9965 | 248 | Markdown::Section { file: LICENSE.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.825 |

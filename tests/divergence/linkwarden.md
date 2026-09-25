Score(3000)=0.738 I=0.891 C=0.611 ns_rows≤3K=15/51 grid(1000/1442/2080/3000/4327/6240/9000)=0.633/0.712/0.725/0.738/0.647/0.645/0.808

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
| walker |  | 1968 | 39 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 289 } |  |  | 0.722 |
| walker |  | 2077 | 109 | Fs::DirListing { dir: packages/lib } |  |  | 0.725 |
| ns | 2083 |  | 386 | `Link` model — every field | 2.3 | 2.1 | 0.754 |
| walker |  | 2134 | 57 | Fs::DirListing { dir: apps/worker/lib } |  |  | 0.755 |
| walker |  | 2193 | 59 | Fs::DirListing { dir: apps/mobile/components } |  |  | 0.756 |
| walker |  | 2219 | 26 | Fs::DirListing { dir: apps/mobile/components/Formats } |  |  | 0.756 |
| walker |  | 2251 | 32 | Fs::DirListing { dir: apps/mobile/components/ActionSheets } |  |  | 0.757 |
| walker |  | 2313 | 62 | Fs::DirListing { dir: apps/web/hooks } |  |  | 0.757 |
| walker |  | 2330 | 17 | Fs::DirListing { dir: apps/web/e2e/fixtures } |  |  | 0.758 |
| walker |  | 2343 | 13 | Fs::DirListing { dir: apps/web/e2e/fixtures/base } |  |  | 0.758 |
| ns | 2396 |  | 313 | `User` model — identity, relations and subscription linkage | 2.4 | 2.1 | 0.711 |
| walker |  | 2515 | 172 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 10 } |  |  | 0.714 |
| walker |  | 2597 | 82 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 108 } |  |  | 0.717 |
| ns | 2738 |  | 342 | `User` model — preference and archival-toggle fields | 2.5 | 2.4 | 0.680 |
| walker |  | 2882 | 285 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 28 } |  |  | 0.738 |
| walker |  | 3019 | 137 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.738 |
| walker |  | 3045 | 26 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.738 |
| ns | 3055 |  | 317 | `Collection` model — every field, including the self-relation | 2.6 | 2.1 | 0.703 |
| walker |  | 3087 | 42 | Fs::DirListing { dir: apps/worker/lib/preservationScheme } |  |  | 0.704 |
| walker |  | 3305 | 218 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 200 } |  |  | 0.709 |
| walker |  | 3370 | 65 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 118 } |  |  | 0.711 |
| ns | 3436 |  | 381 | `UsersAndCollections` join model (the permission bits) and `Tag` model | 2.7 | 2.1 | 0.693 |
| walker |  | 3466 | 96 | Json::Runtime { file: package.json } |  |  | 0.693 |
| ns | 3594 |  | 158 | `AccessToken` model | 2.8 | 2.1 | 0.679 |
| ns | 3617 |  | 23 | Complete packages/prisma listing | 2.9 |  | 0.683 |
| walker |  | 3621 | 155 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.683 |
| walker |  | 3648 | 27 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 5 } |  |  | 0.699 |
| walker |  | 3717 | 69 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 296 } |  |  | 0.701 |
| ns | 3733 |  | 116 | The process-wide `prisma` client singleton | 2.10 |  | 0.689 |
| walker |  | 3796 | 79 | Fs::DirListing { dir: apps/web/public } |  |  | 0.689 |
| walker |  | 3806 | 10 | Fs::DirListing { dir: apps/web/public/screenshots } |  |  | 0.689 |
| ns | 3817 |  | 84 | API version directories: every resource under pages/api/v1 (and the single v2 route) | 3.1 |  | 0.660 |
| ns | 3885 |  | 68 | Route files for links, collections, tags and highlights | 3.2 |  | 0.643 |
| ns | 3962 |  | 77 | Route files for auth, session, users, tokens, config, avatar and logins | 3.3 |  | 0.628 |
| walker |  | 3975 | 169 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 151 } |  |  | 0.665 |
| ns | 4042 |  | 80 | Route files for archives, preserved, search, dashboard, rss, migration, payment, webhook, worker, getFavicon | 3.4 |  | 0.649 |
| ns | 4083 |  | 41 | The unauthenticated `public/` route subtree | 3.5 |  | 0.640 |
| walker |  | 4148 | 173 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 260 } |  |  | 0.642 |
| walker |  | 4278 | 130 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 248 } |  |  | 0.644 |
| ns | 4359 |  | 276 | The route-handler pattern, read from pages/api/v1/links/index.ts | 3.6 |  | 0.627 |
| ns | 4568 |  | 209 | apps/web/lib: the server helper layer and its client/shared siblings | 3.7 |  | 0.600 |
| walker |  | 4598 | 320 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 126 } |  |  | 0.638 |
| walker |  | 4620 | 22 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.638 |
| ns | 4667 |  | 99 | The controller tree: every resource directory under lib/api/controllers | 3.8 |  | 0.621 |
| walker |  | 4781 | 161 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 234 } |  |  | 0.640 |
| ns | 4823 |  | 156 | Controller files for links, collections, tags and highlights | 3.9 |  | 0.621 |
| walker |  | 4830 | 49 | Fs::DirListing { dir: apps/web/public/locales } |  |  | 0.622 |
| walker |  | 4834 | 4 | Fs::DirListing { dir: apps/web/public/locales/de } |  |  | 0.622 |
| walker |  | 4838 | 4 | Fs::DirListing { dir: apps/web/public/locales/en } |  |  | 0.622 |
| walker |  | 4842 | 4 | Fs::DirListing { dir: apps/web/public/locales/es } |  |  | 0.622 |
| walker |  | 4846 | 4 | Fs::DirListing { dir: apps/web/public/locales/fr } |  |  | 0.622 |
| walker |  | 4850 | 4 | Fs::DirListing { dir: apps/web/public/locales/it } |  |  | 0.622 |
| walker |  | 4854 | 4 | Fs::DirListing { dir: apps/web/public/locales/ja } |  |  | 0.622 |
| walker |  | 4858 | 4 | Fs::DirListing { dir: apps/web/public/locales/nl } |  |  | 0.622 |
| walker |  | 4862 | 4 | Fs::DirListing { dir: apps/web/public/locales/pl } |  |  | 0.622 |
| walker |  | 4866 | 4 | Fs::DirListing { dir: apps/web/public/locales/pt-BR } |  |  | 0.622 |
| walker |  | 4870 | 4 | Fs::DirListing { dir: apps/web/public/locales/ro } |  |  | 0.622 |
| walker |  | 4874 | 4 | Fs::DirListing { dir: apps/web/public/locales/ru } |  |  | 0.622 |
| walker |  | 4878 | 4 | Fs::DirListing { dir: apps/web/public/locales/tr } |  |  | 0.622 |
| walker |  | 4882 | 4 | Fs::DirListing { dir: apps/web/public/locales/uk } |  |  | 0.622 |
| walker |  | 4886 | 4 | Fs::DirListing { dir: apps/web/public/locales/zh } |  |  | 0.622 |
| walker |  | 4890 | 4 | Fs::DirListing { dir: apps/web/public/locales/zh-TW } |  |  | 0.622 |
| walker |  | 4919 | 29 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.622 |
| walker |  | 4969 | 50 | Fs::DirListing { dir: apps/mobile/components/ui } |  |  | 0.623 |
| ns | 5001 |  | 178 | Controller files for users, tokens, session, search, dashboard, worker, migration and public access | 3.10 |  | 0.603 |
| walker |  | 5055 | 86 | Fs::DirListing { dir: apps/web/pages } |  |  | 0.603 |
| walker |  | 5061 | 6 | Fs::DirListing { dir: apps/web/pages/preserved } |  |  | 0.603 |
| walker |  | 5069 | 8 | Fs::DirListing { dir: apps/web/pages/api } |  |  | 0.603 |
| walker |  | 5072 | 3 | Fs::DirListing { dir: apps/web/pages/api/v2 } |  |  | 0.604 |
| walker |  | 5081 | 9 | Fs::DirListing { dir: apps/web/pages/public } |  |  | 0.604 |
| walker |  | 5086 | 5 | Fs::DirListing { dir: apps/web/pages/public/collections } |  |  | 0.604 |
| walker |  | 5090 | 4 | Fs::DirListing { dir: apps/web/pages/api/v2/dashboard } |  |  | 0.604 |
| walker |  | 5096 | 6 | Fs::DirListing { dir: apps/web/pages/public/links } |  |  | 0.604 |
| walker |  | 5102 | 6 | Fs::DirListing { dir: apps/web/pages/public/preserved } |  |  | 0.604 |
| walker |  | 5113 | 11 | Fs::DirListing { dir: apps/web/pages/collections } |  |  | 0.604 |
| walker |  | 5124 | 11 | Fs::DirListing { dir: apps/web/pages/tags } |  |  | 0.605 |
| walker |  | 5136 | 12 | Fs::DirListing { dir: apps/web/pages/auth } |  |  | 0.605 |
| walker |  | 5152 | 16 | Fs::DirListing { dir: apps/web/pages/links } |  |  | 0.605 |
| walker |  | 5171 | 19 | Fs::DirListing { dir: apps/web/pages/admin } |  |  | 0.605 |
| walker |  | 5181 | 10 | Fs::DirListing { dir: apps/web/pages/public/collections/[id] } |  |  | 0.605 |
| ns | 5249 |  | 248 | Complete listings of packages/types, packages/lib, packages/filesystem and packages/router | 4.1 |  | 0.632 |
| walker |  | 5349 | 168 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 275 } |  |  | 0.634 |
| walker |  | 5519 | 170 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 220 } |  |  | 0.635 |
| ns | 5647 |  | 398 | packages/types/global.ts — every exported type, interface and enum declaration | 4.2 |  | 0.618 |
| ns | 5793 |  | 146 | `ArchivedFormat`, `LinkType` and `TokenExpiry` variants | 4.3 | 4.2 | 0.611 |
| walker |  | 5892 | 373 | Prisma::DeclTail { file: packages/prisma/schema.prisma, start_line: 28, tail_start_line: 51 } |  |  | 0.643 |
| walker |  | 5908 | 16 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 1 } |  |  | 0.650 |
| ns | 5945 |  | 152 | `ViewMode`, `Sort` and `TagSort` variants | 4.4 | 4.2 | 0.643 |
| walker |  | 6009 | 101 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 99 } |  |  | 0.644 |
| walker |  | 6065 | 56 | Fs::DirListing { dir: apps/web/pages/settings } |  |  | 0.645 |
| walker |  | 6200 | 135 | Json::Dependencies { file: package.json } |  |  | 0.645 |
| ns | 6285 |  | 340 | packages/lib/schemaValidation.ts — every exported zod schema constant | 4.5 |  | 0.633 |
| walker |  | 6300 | 100 | Code::CodeKey { rung: Names, file: packages/filesystem/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.633 |
| walker |  | 6312 | 12 | Code::CodeKey { rung: Names, file: apps/worker/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.633 |
| walker |  | 6324 | 12 | Code::CodeKey { rung: Names, file: packages/prisma/client/index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.633 |
| walker |  | 6336 | 12 | Code::CodeKey { rung: Names, file: packages/prisma/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.633 |
| walker |  | 6412 | 76 | Code::CodeKey { rung: Decl, file: packages/prisma/index.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.640 |
| walker |  | 6438 | 26 | Code::CodeKey { rung: Names, file: apps/web/e2e/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.640 |
| walker |  | 6452 | 14 | Code::CodeKey { rung: Names, file: apps/web/pages/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.640 |
| walker |  | 6495 | 43 | Code::CodeKey { rung: Body, file: apps/web/pages/index.tsx, decl: 1, sub: 0, line: 4 } |  |  | 0.640 |
| ns | 6542 |  | 257 | packages/router — the links, collections and tags hook exports | 4.6 |  | 0.625 |
| ns | 6726 |  | 184 | packages/router — export lines of the remaining ten hook modules | 4.7 | 4.6 | 0.620 |
| ns | 6892 |  | 166 | Complete apps/worker listing, including every job and every preservation handler | 5.1 |  | 0.634 |
| walker |  | 6930 | 435 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: true } |  |  | 0.668 |
| walker |  | 7009 | 79 | Fs::DirListing { dir: apps/web/lib/client } |  |  | 0.677 |
| walker |  | 7024 | 15 | Code::CodeKey { rung: Names, file: apps/mobile/app/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.677 |
| walker |  | 7121 | 97 | Fs::DirListing { dir: apps/web/lib/api } |  |  | 0.704 |
| walker |  | 7134 | 13 | Fs::DirListing { dir: apps/web/lib/api/archives } |  |  | 0.704 |
| walker |  | 7151 | 17 | Fs::DirListing { dir: apps/web/lib/api/preserved } |  |  | 0.705 |
| ns | 7161 |  | 269 | worker.ts — the whole scheduler entry point | 5.2 |  | 0.693 |
| walker |  | 7184 | 33 | Fs::DirListing { dir: apps/web/lib/api/stripe } |  |  | 0.697 |
| walker |  | 7220 | 36 | Fs::DirListing { dir: apps/web/lib/api/controllers } |  |  | 0.713 |
| walker |  | 7225 | 5 | Fs::DirListing { dir: apps/web/lib/api/controllers/search } |  |  | 0.713 |
| walker |  | 7230 | 5 | Fs::DirListing { dir: apps/web/lib/api/controllers/session } |  |  | 0.713 |
| walker |  | 7236 | 6 | Fs::DirListing { dir: apps/web/lib/api/controllers/worker } |  |  | 0.713 |
| walker |  | 7245 | 9 | Fs::DirListing { dir: apps/web/lib/api/controllers/public } |  |  | 0.714 |
| walker |  | 7249 | 4 | Fs::DirListing { dir: apps/web/lib/api/controllers/public/links } |  |  | 0.714 |
| walker |  | 7255 | 6 | Fs::DirListing { dir: apps/web/lib/api/controllers/public/collections } |  |  | 0.715 |
| walker |  | 7261 | 6 | Fs::DirListing { dir: apps/web/lib/api/controllers/public/users } |  |  | 0.715 |
| walker |  | 7268 | 7 | Fs::DirListing { dir: apps/web/lib/api/controllers/public/links/linkId } |  |  | 0.716 |
| walker |  | 7282 | 14 | Fs::DirListing { dir: apps/web/lib/api/controllers/collections } |  |  | 0.716 |
| walker |  | 7296 | 14 | Fs::DirListing { dir: apps/web/lib/api/controllers/highlights } |  |  | 0.717 |
| walker |  | 7310 | 14 | Fs::DirListing { dir: apps/web/lib/api/controllers/tokens } |  |  | 0.719 |
| walker |  | 7317 | 7 | Fs::DirListing { dir: apps/web/lib/api/controllers/tokens/tokenId } |  |  | 0.719 |
| walker |  | 7331 | 14 | Fs::DirListing { dir: apps/web/lib/api/controllers/users } |  |  | 0.722 |
| walker |  | 7348 | 17 | Fs::DirListing { dir: apps/web/lib/api/controllers/links } |  |  | 0.724 |
| ns | 7354 |  | 193 | archiveHandler's signature and its SSRF / skip-preservation guard | 5.3 |  | 0.714 |
| walker |  | 7360 | 12 | Fs::DirListing { dir: apps/web/lib/api/controllers/links/bulk } |  |  | 0.715 |
| walker |  | 7380 | 20 | Fs::DirListing { dir: apps/web/lib/api/controllers/dashboard } |  |  | 0.719 |
| walker |  | 7407 | 27 | Fs::DirListing { dir: apps/web/lib/api/controllers/tags } |  |  | 0.723 |
| walker |  | 7428 | 21 | Fs::DirListing { dir: apps/web/lib/api/controllers/collections/collectionId } |  |  | 0.727 |
| walker |  | 7449 | 21 | Fs::DirListing { dir: apps/web/lib/api/controllers/tags/tagId } |  |  | 0.731 |
| walker |  | 7473 | 24 | Fs::DirListing { dir: apps/web/lib/api/controllers/links/linkId } |  |  | 0.737 |
| walker |  | 7479 | 6 | Fs::DirListing { dir: apps/web/lib/api/controllers/links/linkId/highlight } |  |  | 0.739 |
| walker |  | 7506 | 27 | Fs::DirListing { dir: apps/web/lib/api/controllers/users/userId } |  |  | 0.744 |
| walker |  | 7575 | 69 | Fs::DirListing { dir: apps/web/pages/api/v1 } |  |  | 0.767 |
| walker |  | 7579 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/dashboard } |  |  | 0.767 |
| walker |  | 7583 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/getFavicon } |  |  | 0.767 |
| walker |  | 7587 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/logins } |  |  | 0.767 |
| walker |  | 7591 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/migration } |  |  | 0.768 |
| walker |  | 7595 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/payment } |  |  | 0.768 |
| walker |  | 7599 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/search } |  |  | 0.769 |
| walker |  | 7603 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/session } |  |  | 0.769 |
| walker |  | 7607 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/webhook } |  |  | 0.778 |
| ns | 7607 |  | 253 | Every UI page route under apps/web/pages | 6.1 |  | 0.778 |
| walker |  | 7613 | 6 | Fs::DirListing { dir: apps/web/pages/api/v1/avatar } |  |  | 0.779 |
| walker |  | 7622 | 9 | Fs::DirListing { dir: apps/web/pages/api/v1/config } |  |  | 0.780 |
| walker |  | 7631 | 9 | Fs::DirListing { dir: apps/web/pages/api/v1/public } |  |  | 0.780 |
| walker |  | 7637 | 6 | Fs::DirListing { dir: apps/web/pages/api/v1/public/links } |  |  | 0.781 |
| walker |  | 7643 | 6 | Fs::DirListing { dir: apps/web/pages/api/v1/public/users } |  |  | 0.782 |
| walker |  | 7652 | 9 | Fs::DirListing { dir: apps/web/pages/api/v1/worker } |  |  | 0.783 |
| walker |  | 7662 | 10 | Fs::DirListing { dir: apps/web/pages/api/v1/collections } |  |  | 0.783 |
| walker |  | 7672 | 10 | Fs::DirListing { dir: apps/web/pages/api/v1/highlights } |  |  | 0.784 |
| walker |  | 7682 | 10 | Fs::DirListing { dir: apps/web/pages/api/v1/rss } |  |  | 0.786 |
| walker |  | 7692 | 10 | Fs::DirListing { dir: apps/web/pages/api/v1/tokens } |  |  | 0.787 |
| walker |  | 7704 | 12 | Fs::DirListing { dir: apps/web/pages/api/v1/links } |  |  | 0.789 |
| walker |  | 7708 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/links/archive } |  |  | 0.789 |
| walker |  | 7721 | 13 | Fs::DirListing { dir: apps/web/pages/api/v1/users } |  |  | 0.792 |
| walker |  | 7730 | 9 | Fs::DirListing { dir: apps/web/pages/api/v1/users/[id] } |  |  | 0.794 |
| walker |  | 7744 | 14 | Fs::DirListing { dir: apps/web/pages/api/v1/tags } |  |  | 0.797 |
| walker |  | 7754 | 10 | Fs::DirListing { dir: apps/web/pages/api/v1/links/[id] } |  |  | 0.801 |
| walker |  | 7758 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/links/[id]/archive } |  |  | 0.803 |
| walker |  | 7762 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/links/[id]/highlights } |  |  | 0.805 |
| walker |  | 7774 | 12 | Fs::DirListing { dir: apps/web/pages/api/v1/public/collections } |  |  | 0.808 |
| walker |  | 7778 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/public/collections/links } |  |  | 0.809 |
| walker |  | 7782 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/public/collections/tags } |  |  | 0.811 |
| walker |  | 7800 | 18 | Fs::DirListing { dir: apps/web/pages/api/v1/preserved } |  |  | 0.815 |
| walker |  | 7819 | 19 | Fs::DirListing { dir: apps/web/pages/api/v1/archives } |  |  | 0.820 |
| walker |  | 7841 | 22 | Fs::DirListing { dir: apps/web/pages/api/v1/auth } |  |  | 0.826 |
| walker |  | 7889 | 48 | Fs::DirListing { dir: apps/web/lib/api/controllers/migration } |  |  | 0.835 |
| ns | 7965 |  | 358 | The flat components/ directory and the ui/ primitives | 6.2 |  | 0.802 |
| walker |  | 8199 | 310 | Plaintext::Whole { file: docker-compose.yml } |  |  | 0.803 |
| ns | 8246 |  | 281 | Modal, link-view, preservation and input-picker component subdirectories | 6.3 | 6.2 | 0.783 |
| walker |  | 8300 | 101 | Code::CodeKey { rung: Body, file: apps/worker/index.ts, decl: 1, sub: 0, line: 3 } |  |  | 0.783 |
| walker |  | 8435 | 135 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.783 |
| ns | 8473 |  | 227 | Web hooks, layouts, stores, ambient types, email templates, one-off migration scripts and the Playwright suite | 6.4 |  | 0.782 |
| ns | 8713 |  | 240 | verifyUser — the guard chain every authenticated route runs first | 7.1 |  | 0.770 |
| walker |  | 8812 | 377 | Json::Scripts { file: package.json } |  |  | 0.794 |
| walker |  | 8815 | 3 | Fs::DirListing { dir: apps/web/scripts } |  |  | 0.795 |
| ns | 8948 |  | 235 | docker-compose.yml — the three-container deployment | 7.2 |  | 0.798 |
| walker |  | 9132 | 317 | Fs::DirListing { dir: apps/web/components } |  |  | 0.827 |
| walker |  | 9141 | 9 | Fs::DirListing { dir: apps/web/components/LinkViews } |  |  | 0.827 |
| walker |  | 9157 | 16 | Fs::DirListing { dir: apps/web/components/InputSelect } |  |  | 0.828 |
| ns | 9162 |  | 214 | .env.sample — the required variables and the complete list of section headings | 7.3 |  | 0.818 |
| walker |  | 9183 | 26 | Fs::DirListing { dir: apps/web/components/Preservation } |  |  | 0.819 |
| walker |  | 9224 | 41 | Fs::DirListing { dir: apps/web/components/ui } |  |  | 0.828 |
| walker |  | 9286 | 62 | Fs::DirListing { dir: apps/web/components/LinkViews/LinkComponents } |  |  | 0.832 |
| ns | 9341 |  | 179 | The head of the optional tuning-variable block | 7.4 | 7.3 | 0.823 |
| walker |  | 9454 | 168 | Fs::DirListing { dir: apps/web/components/ModalContent } |  |  | 0.843 |
| ns | 9473 |  | 132 | CI workflows, GitHub templates, the patch-package patch, and every translated locale | 7.5 |  | 0.845 |
| walker |  | 9542 | 88 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.845 |
| walker |  | 9646 | 104 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.845 |
| walker |  | 9688 | 42 | Json::Identity { file: packages/lib/package.json } |  |  | 0.846 |
| ns | 9689 |  | 216 | apps/mobile root and the complete Expo Router screen tree | 8.1 |  | 0.849 |
| walker |  | 9700 | 12 | Json::Entry { file: packages/lib/package.json } |  |  | 0.849 |
| walker |  | 9742 | 42 | Json::Identity { file: packages/router/package.json } |  |  | 0.849 |
| walker |  | 9754 | 12 | Json::Entry { file: packages/router/package.json } |  |  | 0.849 |
| walker |  | 9796 | 42 | Json::Identity { file: packages/types/package.json } |  |  | 0.850 |
| walker |  | 9808 | 12 | Json::Entry { file: packages/types/package.json } |  |  | 0.850 |
| walker |  | 9851 | 43 | Json::Identity { file: apps/worker/package.json } |  |  | 0.851 |
| walker |  | 9863 | 12 | Json::Entry { file: apps/worker/package.json } |  |  | 0.851 |
| ns | 9891 |  | 202 | Mobile components, stores and query-cache modules | 8.2 | 8.1 | 0.853 |
| walker |  | 9921 | 58 | Json::Scripts { file: apps/worker/package.json } |  |  | 0.853 |
| walker |  | 9964 | 43 | Json::Identity { file: packages/filesystem/package.json } |  |  | 0.855 |
| walker |  | 9976 | 12 | Json::Entry { file: packages/filesystem/package.json } |  |  | 0.855 |
| walker |  | 9985 | 9 | Json::Dependencies { file: packages/filesystem/package.json } |  |  | 0.855 |

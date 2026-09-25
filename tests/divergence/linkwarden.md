Score(3000)=0.740 I=0.895 C=0.611 ns_rows≤3K=15/51 grid(1000/1442/2080/3000/4327/6240/9000)=0.636/0.714/0.688/0.740/0.658/0.680/0.793

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 75 |  | 75 | Product identity: name, tagline, one-sentence definition | 1.1 |  | 0.000 |
| walker |  | 79 | 79 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 88 | 9 | Fs::DirListing { dir: apps } |  |  | 0.000 |
| walker |  | 99 | 11 | Fs::DirListing { dir: patches } |  |  | 0.000 |
| ns | 154 |  | 79 | Complete repository root listing | 1.2 |  | 0.683 |
| ns | 178 |  | 24 | The three apps and five shared packages | 1.3 |  | 0.589 |
| walker |  | 209 | 110 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.864 |
| walker |  | 224 | 15 | Fs::DirListing { dir: packages } |  |  | 1.000 |
| walker |  | 247 | 23 | Fs::DirListing { dir: packages/prisma } |  |  | 1.000 |
| walker |  | 256 | 9 | Fs::DirListing { dir: packages/prisma/client } |  |  | 1.000 |
| ns | 259 |  | 81 | Complete apps/web listing | 1.4 |  | 0.749 |
| walker |  | 282 | 26 | Fs::DirListing { dir: apps/worker } |  |  | 0.749 |
| walker |  | 287 | 5 | Fs::DirListing { dir: apps/worker/templates } |  |  | 0.749 |
| ns | 321 |  | 62 | Workspace globs in the root package.json | 1.5 |  | 0.702 |
| walker |  | 330 | 43 | Json::Identity { file: package.json } |  |  | 0.713 |
| walker |  | 334 | 4 | Fs::DirListing { dir: .vscode } |  |  | 0.713 |
| walker |  | 347 | 13 | Fs::DirListing { dir: packages/types } |  |  | 0.713 |
| walker |  | 380 | 33 | Fs::DirListing { dir: assets } |  |  | 0.713 |
| walker |  | 385 | 5 | Fs::DirListing { dir: .devcontainer } |  |  | 0.713 |
| walker |  | 439 | 54 | Fs::DirListing { dir: packages/filesystem } |  |  | 0.714 |
| ns | 460 |  | 139 | Workspace package names — every `name` field under apps/ and packages/ | 1.6 |  | 0.661 |
| walker |  | 473 | 34 | Json::Entry { file: package.json } |  |  | 0.717 |
| ns | 674 |  | 214 | Root scripts: how you run web, worker and both together | 1.7 |  | 0.664 |
| walker |  | 693 | 220 | Prisma::Toc { file: packages/prisma/schema.prisma } |  |  | 0.674 |
| walker |  | 729 | 36 | Fs::DirListing { dir: apps/worker/workers } |  |  | 0.675 |
| walker |  | 801 | 72 | Fs::DirListing { dir: packages/router } |  |  | 0.676 |
| walker |  | 825 | 24 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 77 } |  |  | 0.677 |
| ns | 837 |  | 163 | Root scripts: prisma, format, test, coverage, postinstall | 1.8 |  | 0.633 |
| walker |  | 874 | 49 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 90 } |  |  | 0.635 |
| walker |  | 954 | 80 | Fs::DirListing { dir: apps/mobile } |  |  | 0.636 |
| walker |  | 958 | 4 | Fs::DirListing { dir: apps/mobile/styles } |  |  | 0.636 |
| walker |  | 964 | 6 | Fs::DirListing { dir: apps/mobile/assets } |  |  | 0.636 |
| walker |  | 1000 | 36 | Fs::DirListing { dir: apps/mobile/app } |  |  | 0.636 |
| walker |  | 1008 | 8 | Fs::DirListing { dir: apps/mobile/plugins } |  |  | 0.636 |
| walker |  | 1014 | 6 | Fs::DirListing { dir: apps/mobile/app/links } |  |  | 0.636 |
| walker |  | 1020 | 6 | Fs::DirListing { dir: apps/mobile/assets/fonts } |  |  | 0.636 |
| walker |  | 1031 | 11 | Fs::DirListing { dir: apps/mobile/types } |  |  | 0.636 |
| walker |  | 1043 | 12 | Fs::DirListing { dir: apps/mobile/store } |  |  | 0.636 |
| walker |  | 1066 | 23 | Fs::DirListing { dir: apps/mobile/lib } |  |  | 0.636 |
| ns | 1082 |  | 245 | Feature list, first half (preservation, reading, organisation, sharing) | 1.9 |  | 0.588 |
| walker |  | 1087 | 21 | Fs::DirListing { dir: apps/mobile/app/(tabs) } |  |  | 0.588 |
| walker |  | 1098 | 11 | Fs::DirListing { dir: apps/mobile/app/(tabs)/links } |  |  | 0.588 |
| walker |  | 1115 | 17 | Fs::DirListing { dir: apps/mobile/app/(tabs)/collections } |  |  | 0.589 |
| walker |  | 1132 | 17 | Fs::DirListing { dir: apps/mobile/app/(tabs)/dashboard } |  |  | 0.589 |
| walker |  | 1149 | 17 | Fs::DirListing { dir: apps/mobile/app/(tabs)/settings } |  |  | 0.589 |
| walker |  | 1166 | 17 | Fs::DirListing { dir: apps/mobile/app/(tabs)/tags } |  |  | 0.590 |
| walker |  | 1190 | 24 | Fs::DirListing { dir: apps/mobile/assets/images } |  |  | 0.590 |
| walker |  | 1216 | 26 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 304 } |  |  | 0.591 |
| ns | 1271 |  | 189 | Feature list, second half (sync, SSO, API keys, i18n, RSS, uploads) | 1.10 |  | 0.551 |
| walker |  | 1297 | 81 | Fs::DirListing { dir: apps/web } |  |  | 0.713 |
| walker |  | 1315 | 18 | Fs::DirListing { dir: apps/web/e2e } |  |  | 0.713 |
| walker |  | 1319 | 4 | Fs::DirListing { dir: apps/web/styles } |  |  | 0.713 |
| walker |  | 1323 | 4 | Fs::DirListing { dir: apps/web/e2e/data } |  |  | 0.713 |
| walker |  | 1332 | 9 | Fs::DirListing { dir: apps/web/store } |  |  | 0.713 |
| walker |  | 1344 | 12 | Fs::DirListing { dir: apps/web/types } |  |  | 0.713 |
| walker |  | 1357 | 13 | Fs::DirListing { dir: apps/web/lib } |  |  | 0.713 |
| walker |  | 1443 | 86 | Fs::DirListing { dir: apps/web/pages } |  |  | 0.714 |
| walker |  | 1454 | 11 | Fs::DirListing { dir: apps/web/pages/collections } |  |  | 0.714 |
| walker |  | 1465 | 11 | Fs::DirListing { dir: apps/web/pages/tags } |  |  | 0.714 |
| walker |  | 1481 | 16 | Fs::DirListing { dir: apps/web/pages/links } |  |  | 0.714 |
| walker |  | 1500 | 19 | Fs::DirListing { dir: apps/web/pages/admin } |  |  | 0.715 |
| walker |  | 1506 | 6 | Fs::DirListing { dir: apps/web/pages/preserved } |  |  | 0.715 |
| ns | 1539 |  | 268 | schema.prisma: datasource/generator plus every model and enum header, bodies elided | 2.1 |  | 0.667 |
| walker |  | 1562 | 56 | Fs::DirListing { dir: apps/web/pages/settings } |  |  | 0.668 |
| walker |  | 1570 | 8 | Fs::DirListing { dir: apps/web/pages/api } |  |  | 0.668 |
| walker |  | 1575 | 5 | Fs::DirListing { dir: apps/web/pages/api/v2/dashboard } |  |  | 0.668 |
| walker |  | 1584 | 9 | Fs::DirListing { dir: apps/web/pages/public } |  |  | 0.668 |
| walker |  | 1590 | 6 | Fs::DirListing { dir: apps/web/pages/public/links } |  |  | 0.668 |
| walker |  | 1596 | 6 | Fs::DirListing { dir: apps/web/pages/public/preserved } |  |  | 0.669 |
| walker |  | 1608 | 12 | Fs::DirListing { dir: apps/web/pages/auth } |  |  | 0.669 |
| walker |  | 1629 | 21 | Fs::DirListing { dir: apps/web/templates } |  |  | 0.669 |
| walker |  | 1635 | 6 | Fs::DirListing { dir: apps/web/e2e/tests } |  |  | 0.669 |
| walker |  | 1640 | 5 | Fs::DirListing { dir: apps/web/e2e/tests/public } |  |  | 0.669 |
| walker |  | 1664 | 24 | Fs::DirListing { dir: apps/web/layouts } |  |  | 0.669 |
| walker |  | 1684 | 20 | Fs::DirListing { dir: apps/web/lib/shared } |  |  | 0.670 |
| walker |  | 1697 | 13 | Fs::DirListing { dir: apps/web/pages/public/collections/[id] } |  |  | 0.649 |
| ns | 1697 |  | 158 | All five schema enums, fully expanded | 2.2 | 2.1 | 0.649 |
| walker |  | 1707 | 10 | Fs::DirListing { dir: apps/web/e2e/tests/global } |  |  | 0.649 |
| walker |  | 1742 | 35 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 83 } |  |  | 0.676 |
| walker |  | 1952 | 210 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 166 } |  |  | 0.683 |
| ns | 2083 |  | 386 | `Link` model — every field | 2.3 | 2.1 | 0.650 |
| walker |  | 2131 | 179 | Prisma::DeclTail { file: packages/prisma/schema.prisma, start_line: 166, tail_start_line: 182 } |  |  | 0.726 |
| walker |  | 2160 | 29 | Fs::DirListing { dir: .github } |  |  | 0.726 |
| walker |  | 2181 | 21 | Fs::DirListing { dir: .github/workflows } |  |  | 0.726 |
| walker |  | 2220 | 39 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 289 } |  |  | 0.755 |
| walker |  | 2392 | 172 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 10 } |  |  | 0.758 |
| ns | 2396 |  | 313 | `User` model — identity, relations and subscription linkage | 2.4 | 2.1 | 0.711 |
| walker |  | 2474 | 82 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 108 } |  |  | 0.714 |
| walker |  | 2583 | 109 | Fs::DirListing { dir: packages/lib } |  |  | 0.718 |
| walker |  | 2640 | 57 | Fs::DirListing { dir: apps/worker/lib } |  |  | 0.719 |
| ns | 2738 |  | 342 | `User` model — preference and archival-toggle fields | 2.5 | 2.4 | 0.682 |
| walker |  | 2925 | 285 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 28 } |  |  | 0.740 |
| ns | 3055 |  | 317 | `Collection` model — every field, including the self-relation | 2.6 | 2.1 | 0.705 |
| walker |  | 3062 | 137 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.705 |
| walker |  | 3088 | 26 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.705 |
| walker |  | 3243 | 155 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.705 |
| walker |  | 3272 | 29 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.705 |
| walker |  | 3331 | 59 | Fs::DirListing { dir: apps/mobile/components } |  |  | 0.705 |
| walker |  | 3357 | 26 | Fs::DirListing { dir: apps/mobile/components/Formats } |  |  | 0.706 |
| walker |  | 3389 | 32 | Fs::DirListing { dir: apps/mobile/components/ActionSheets } |  |  | 0.706 |
| ns | 3436 |  | 381 | `UsersAndCollections` join model (the permission bits) and `Tag` model | 2.7 | 2.1 | 0.666 |
| ns | 3594 |  | 158 | `AccessToken` model | 2.8 | 2.1 | 0.653 |
| walker |  | 3607 | 218 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 200 } |  |  | 0.678 |
| ns | 3617 |  | 23 | Complete packages/prisma listing | 2.9 |  | 0.683 |
| walker |  | 3669 | 62 | Fs::DirListing { dir: apps/web/hooks } |  |  | 0.684 |
| ns | 3733 |  | 116 | The process-wide `prisma` client singleton | 2.10 |  | 0.672 |
| walker |  | 3734 | 65 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 118 } |  |  | 0.674 |
| ns | 3815 |  | 82 | API version directories: every resource under pages/api/v1 (and the single v2 route) | 3.1 |  | 0.646 |
| walker |  | 3830 | 96 | Json::Runtime { file: package.json } |  |  | 0.646 |
| walker |  | 3847 | 17 | Fs::DirListing { dir: apps/web/e2e/fixtures } |  |  | 0.647 |
| walker |  | 3860 | 13 | Fs::DirListing { dir: apps/web/e2e/fixtures/base } |  |  | 0.647 |
| walker |  | 3877 | 17 | Fs::DirListing { dir: apps/web/scripts/migration } |  |  | 0.647 |
| walker |  | 3881 | 4 | Fs::DirListing { dir: apps/web/scripts/migration/v2.6.1 } |  |  | 0.648 |
| ns | 3883 |  | 68 | Route files for links, collections, tags and highlights | 3.2 |  | 0.631 |
| walker |  | 3908 | 27 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 5 } |  |  | 0.646 |
| ns | 3960 |  | 77 | Route files for auth, session, users, tokens, config, avatar and logins | 3.3 |  | 0.630 |
| walker |  | 3977 | 69 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 296 } |  |  | 0.632 |
| ns | 4040 |  | 80 | Route files for archives, preserved, search, dashboard, rss, migration, payment, webhook, worker, getFavicon | 3.4 |  | 0.617 |
| ns | 4081 |  | 41 | The unauthenticated `public/` route subtree | 3.5 |  | 0.608 |
| walker |  | 4146 | 169 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 151 } |  |  | 0.645 |
| ns | 4357 |  | 276 | The route-handler pattern, read from pages/api/v1/links/index.ts | 3.6 |  | 0.628 |
| ns | 4566 |  | 209 | apps/web/lib: the server helper layer and its client/shared siblings | 3.7 |  | 0.600 |
| walker |  | 4581 | 435 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: true } |  |  | 0.648 |
| ns | 4665 |  | 99 | The controller tree: every resource directory under lib/api/controllers | 3.8 |  | 0.632 |
| walker |  | 4754 | 173 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 260 } |  |  | 0.633 |
| ns | 4821 |  | 156 | Controller files for links, collections, tags and highlights | 3.9 |  | 0.615 |
| walker |  | 4884 | 130 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 248 } |  |  | 0.616 |
| ns | 4997 |  | 176 | Controller files for users, tokens, session, search, dashboard, worker, migration and public access | 3.10 |  | 0.596 |
| walker |  | 5204 | 320 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 126 } |  |  | 0.630 |
| ns | 5245 |  | 248 | Complete listings of packages/types, packages/lib, packages/filesystem and packages/router | 4.1 |  | 0.656 |
| walker |  | 5365 | 161 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 234 } |  |  | 0.671 |
| walker |  | 5407 | 42 | Fs::DirListing { dir: apps/worker/lib/preservationScheme } |  |  | 0.672 |
| walker |  | 5575 | 168 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 275 } |  |  | 0.673 |
| ns | 5643 |  | 398 | packages/types/global.ts — every exported type, interface and enum declaration | 4.2 |  | 0.655 |
| walker |  | 5745 | 170 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 220 } |  |  | 0.657 |
| ns | 5789 |  | 146 | `ArchivedFormat`, `LinkType` and `TokenExpiry` variants | 4.3 | 4.2 | 0.649 |
| ns | 5941 |  | 152 | `ViewMode`, `Sort` and `TagSort` variants | 4.4 | 4.2 | 0.641 |
| walker |  | 6118 | 373 | Prisma::DeclTail { file: packages/prisma/schema.prisma, start_line: 28, tail_start_line: 51 } |  |  | 0.673 |
| walker |  | 6134 | 16 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 1 } |  |  | 0.680 |
| walker |  | 6213 | 79 | Fs::DirListing { dir: apps/web/public } |  |  | 0.680 |
| walker |  | 6223 | 10 | Fs::DirListing { dir: apps/web/public/screenshots } |  |  | 0.680 |
| ns | 6281 |  | 340 | packages/lib/schemaValidation.ts — every exported zod schema constant | 4.5 |  | 0.667 |
| walker |  | 6324 | 101 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 99 } |  |  | 0.669 |
| walker |  | 6355 | 31 | Code::CodeKey { rung: Names, file: apps/worker/worker.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.669 |
| walker |  | 6371 | 16 | Code::CodeKey { rung: Decl, file: apps/worker/worker.ts, decl: 1, sub: 0, line: 8 } |  |  | 0.669 |
| walker |  | 6393 | 22 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.669 |
| walker |  | 6528 | 135 | Json::Dependencies { file: package.json } |  |  | 0.669 |
| ns | 6538 |  | 257 | packages/router — the links, collections and tags hook exports | 4.6 |  | 0.654 |
| walker |  | 6578 | 50 | Fs::DirListing { dir: apps/mobile/components/ui } |  |  | 0.654 |
| walker |  | 6678 | 100 | Code::CodeKey { rung: Names, file: packages/filesystem/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.654 |
| walker |  | 6690 | 12 | Code::CodeKey { rung: Names, file: packages/prisma/client/index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.654 |
| walker |  | 6702 | 12 | Code::CodeKey { rung: Names, file: packages/prisma/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.654 |
| ns | 6722 |  | 184 | packages/router — export lines of the remaining ten hook modules | 4.7 | 4.6 | 0.649 |
| walker |  | 6778 | 76 | Code::CodeKey { rung: Decl, file: packages/prisma/index.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.655 |
| walker |  | 6816 | 38 | Code::CodeKey { rung: Names, file: apps/worker/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.655 |
| walker |  | 6842 | 26 | Code::CodeKey { rung: Names, file: apps/web/e2e/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.655 |
| walker |  | 6856 | 14 | Code::CodeKey { rung: Names, file: apps/web/pages/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.655 |
| walker |  | 6870 | 14 | Code::CodeKey { rung: Names, file: apps/web/pages/settings/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.655 |
| walker |  | 6885 | 15 | Code::CodeKey { rung: Names, file: apps/mobile/app/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.655 |
| ns | 6888 |  | 166 | Complete apps/worker listing, including every job and every preservation handler | 5.1 |  | 0.668 |
| walker |  | 6948 | 63 | Code::CodeKey { rung: Names, file: apps/web/pages/collections/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.668 |
| walker |  | 6963 | 15 | Code::CodeKey { rung: Body, file: apps/web/pages/collections/index.tsx, decl: 2, sub: 0, line: 153 } |  |  | 0.668 |
| walker |  | 7026 | 63 | Code::CodeKey { rung: Names, file: apps/web/pages/links/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.668 |
| walker |  | 7041 | 15 | Code::CodeKey { rung: Body, file: apps/web/pages/links/index.tsx, decl: 2, sub: 0, line: 70 } |  |  | 0.668 |
| walker |  | 7106 | 65 | Code::CodeKey { rung: Names, file: apps/web/pages/tags/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.668 |
| walker |  | 7121 | 15 | Code::CodeKey { rung: Body, file: apps/web/pages/tags/index.tsx, decl: 2, sub: 0, line: 274 } |  |  | 0.668 |
| ns | 7157 |  | 269 | worker.ts — the whole scheduler entry point | 5.2 |  | 0.657 |
| walker |  | 7200 | 79 | Fs::DirListing { dir: apps/web/lib/client } |  |  | 0.666 |
| walker |  | 7335 | 135 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.666 |
| ns | 7350 |  | 193 | archiveHandler's signature and its SSRF / skip-preservation guard | 5.3 |  | 0.657 |
| walker |  | 7372 | 37 | Code::CodeKey { rung: Names, file: apps/web/pages/admin/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.657 |
| walker |  | 7380 | 8 | Code::CodeKey { rung: Body, file: apps/web/pages/admin/index.tsx, decl: 2, sub: 0, line: 12 } |  |  | 0.657 |
| ns | 7601 |  | 251 | Every UI page route under apps/web/pages | 6.1 |  | 0.674 |
| walker |  | 7690 | 310 | Plaintext::Whole { file: docker-compose.yml } |  |  | 0.675 |
| walker |  | 7787 | 97 | Fs::DirListing { dir: apps/web/lib/api } |  |  | 0.699 |
| walker |  | 7800 | 13 | Fs::DirListing { dir: apps/web/lib/api/archives } |  |  | 0.700 |
| walker |  | 7817 | 17 | Fs::DirListing { dir: apps/web/lib/api/preserved } |  |  | 0.700 |
| walker |  | 7850 | 33 | Fs::DirListing { dir: apps/web/lib/api/stripe } |  |  | 0.703 |
| walker |  | 7886 | 36 | Fs::DirListing { dir: apps/web/lib/api/controllers } |  |  | 0.718 |
| walker |  | 7891 | 5 | Fs::DirListing { dir: apps/web/lib/api/controllers/search } |  |  | 0.718 |
| walker |  | 7896 | 5 | Fs::DirListing { dir: apps/web/lib/api/controllers/session } |  |  | 0.718 |
| walker |  | 7902 | 6 | Fs::DirListing { dir: apps/web/lib/api/controllers/worker } |  |  | 0.718 |
| walker |  | 7911 | 9 | Fs::DirListing { dir: apps/web/lib/api/controllers/public } |  |  | 0.719 |
| walker |  | 7917 | 6 | Fs::DirListing { dir: apps/web/lib/api/controllers/public/collections } |  |  | 0.719 |
| walker |  | 7923 | 6 | Fs::DirListing { dir: apps/web/lib/api/controllers/public/users } |  |  | 0.720 |
| walker |  | 7937 | 14 | Fs::DirListing { dir: apps/web/lib/api/controllers/collections } |  |  | 0.720 |
| walker |  | 7951 | 14 | Fs::DirListing { dir: apps/web/lib/api/controllers/highlights } |  |  | 0.721 |
| ns | 7959 |  | 358 | The flat components/ directory and the ui/ primitives | 6.2 |  | 0.692 |
| walker |  | 7965 | 14 | Fs::DirListing { dir: apps/web/lib/api/controllers/tokens } |  |  | 0.694 |
| walker |  | 7972 | 7 | Fs::DirListing { dir: apps/web/lib/api/controllers/tokens/tokenId } |  |  | 0.694 |
| walker |  | 7986 | 14 | Fs::DirListing { dir: apps/web/lib/api/controllers/users } |  |  | 0.696 |
| walker |  | 8003 | 17 | Fs::DirListing { dir: apps/web/lib/api/controllers/links } |  |  | 0.698 |
| walker |  | 8015 | 12 | Fs::DirListing { dir: apps/web/lib/api/controllers/links/bulk } |  |  | 0.699 |
| walker |  | 8024 | 9 | Fs::DirListing { dir: apps/web/lib/api/controllers/public/links/linkId } |  |  | 0.701 |
| walker |  | 8044 | 20 | Fs::DirListing { dir: apps/web/lib/api/controllers/dashboard } |  |  | 0.704 |
| walker |  | 8071 | 27 | Fs::DirListing { dir: apps/web/lib/api/controllers/tags } |  |  | 0.708 |
| walker |  | 8092 | 21 | Fs::DirListing { dir: apps/web/lib/api/controllers/collections/collectionId } |  |  | 0.711 |
| walker |  | 8113 | 21 | Fs::DirListing { dir: apps/web/lib/api/controllers/tags/tagId } |  |  | 0.714 |
| walker |  | 8137 | 24 | Fs::DirListing { dir: apps/web/lib/api/controllers/links/linkId } |  |  | 0.720 |
| walker |  | 8143 | 6 | Fs::DirListing { dir: apps/web/lib/api/controllers/links/linkId/highlight } |  |  | 0.721 |
| walker |  | 8170 | 27 | Fs::DirListing { dir: apps/web/lib/api/controllers/users/userId } |  |  | 0.726 |
| walker |  | 8239 | 69 | Fs::DirListing { dir: apps/web/pages/api/v1 } |  |  | 0.747 |
| ns | 8240 |  | 281 | Modal, link-view, preservation and input-picker component subdirectories | 6.3 | 6.2 | 0.728 |
| walker |  | 8243 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/dashboard } |  |  | 0.728 |
| walker |  | 8247 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/getFavicon } |  |  | 0.728 |
| walker |  | 8251 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/logins } |  |  | 0.728 |
| walker |  | 8255 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/migration } |  |  | 0.728 |
| walker |  | 8259 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/payment } |  |  | 0.729 |
| walker |  | 8263 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/search } |  |  | 0.729 |
| walker |  | 8267 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/session } |  |  | 0.729 |
| walker |  | 8271 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/webhook } |  |  | 0.730 |
| walker |  | 8280 | 9 | Fs::DirListing { dir: apps/web/pages/api/v1/config } |  |  | 0.730 |
| walker |  | 8289 | 9 | Fs::DirListing { dir: apps/web/pages/api/v1/worker } |  |  | 0.732 |
| walker |  | 8299 | 10 | Fs::DirListing { dir: apps/web/pages/api/v1/collections } |  |  | 0.732 |
| walker |  | 8309 | 10 | Fs::DirListing { dir: apps/web/pages/api/v1/highlights } |  |  | 0.732 |
| walker |  | 8319 | 10 | Fs::DirListing { dir: apps/web/pages/api/v1/rss } |  |  | 0.734 |
| walker |  | 8329 | 10 | Fs::DirListing { dir: apps/web/pages/api/v1/tokens } |  |  | 0.735 |
| walker |  | 8341 | 12 | Fs::DirListing { dir: apps/web/pages/api/v1/links } |  |  | 0.737 |
| walker |  | 8345 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/links/archive } |  |  | 0.737 |
| walker |  | 8355 | 10 | Fs::DirListing { dir: apps/web/pages/api/v1/links/[id] } |  |  | 0.740 |
| walker |  | 8359 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/links/[id]/archive } |  |  | 0.741 |
| walker |  | 8363 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/links/[id]/highlights } |  |  | 0.743 |
| walker |  | 8376 | 13 | Fs::DirListing { dir: apps/web/pages/api/v1/users } |  |  | 0.745 |
| walker |  | 8385 | 9 | Fs::DirListing { dir: apps/web/pages/api/v1/users/[id] } |  |  | 0.747 |
| walker |  | 8399 | 14 | Fs::DirListing { dir: apps/web/pages/api/v1/tags } |  |  | 0.751 |
| walker |  | 8418 | 19 | Fs::DirListing { dir: apps/web/pages/api/v1/archives } |  |  | 0.754 |
| walker |  | 8424 | 6 | Fs::DirListing { dir: apps/web/pages/api/v1/avatar } |  |  | 0.755 |
| walker |  | 8433 | 9 | Fs::DirListing { dir: apps/web/pages/api/v1/public } |  |  | 0.756 |
| walker |  | 8439 | 6 | Fs::DirListing { dir: apps/web/pages/api/v1/public/links } |  |  | 0.757 |
| walker |  | 8445 | 6 | Fs::DirListing { dir: apps/web/pages/api/v1/public/users } |  |  | 0.757 |
| walker |  | 8457 | 12 | Fs::DirListing { dir: apps/web/pages/api/v1/public/collections } |  |  | 0.760 |
| walker |  | 8461 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/public/collections/links } |  |  | 0.762 |
| walker |  | 8465 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/public/collections/tags } |  |  | 0.763 |
| ns | 8466 |  | 226 | Web hooks, layouts, stores, ambient types, email templates, one-off migration scripts and the Playwright suite | 6.4 |  | 0.770 |
| walker |  | 8483 | 18 | Fs::DirListing { dir: apps/web/pages/api/v1/preserved } |  |  | 0.775 |
| walker |  | 8505 | 22 | Fs::DirListing { dir: apps/web/pages/api/v1/auth } |  |  | 0.781 |
| walker |  | 8553 | 48 | Fs::DirListing { dir: apps/web/lib/api/controllers/migration } |  |  | 0.789 |
| walker |  | 8641 | 88 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.789 |
| ns | 8706 |  | 240 | verifyUser — the guard chain every authenticated route runs first | 7.1 |  | 0.778 |
| walker |  | 8745 | 104 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.778 |
| ns | 8941 |  | 235 | docker-compose.yml — the three-container deployment | 7.2 |  | 0.781 |
| walker |  | 9122 | 377 | Json::Scripts { file: package.json } |  |  | 0.804 |
| ns | 9155 |  | 214 | .env.sample — the required variables and the complete list of section headings | 7.3 |  | 0.795 |
| walker |  | 9165 | 43 | Code::CodeKey { rung: Body, file: apps/web/pages/index.tsx, decl: 1, sub: 0, line: 4 } |  |  | 0.795 |
| walker |  | 9209 | 44 | Code::CodeKey { rung: Body, file: apps/web/pages/settings/index.tsx, decl: 1, sub: 0, line: 4 } |  |  | 0.795 |
| walker |  | 9258 | 49 | Code::CodeKey { rung: Body, file: apps/web/pages/admin/index.tsx, decl: 1, sub: 0, line: 3 } |  |  | 0.795 |
| ns | 9334 |  | 179 | The head of the optional tuning-variable block | 7.4 | 7.3 | 0.787 |
| ns | 9466 |  | 132 | CI workflows, GitHub templates, the patch-package patch, and every translated locale | 7.5 |  | 0.779 |
| walker |  | 9575 | 317 | Fs::DirListing { dir: apps/web/components } |  |  | 0.807 |
| walker |  | 9584 | 9 | Fs::DirListing { dir: apps/web/components/LinkViews } |  |  | 0.807 |
| walker |  | 9600 | 16 | Fs::DirListing { dir: apps/web/components/InputSelect } |  |  | 0.807 |
| walker |  | 9626 | 26 | Fs::DirListing { dir: apps/web/components/Preservation } |  |  | 0.808 |
| walker |  | 9667 | 41 | Fs::DirListing { dir: apps/web/components/ui } |  |  | 0.816 |
| ns | 9682 |  | 216 | apps/mobile root and the complete Expo Router screen tree | 8.1 |  | 0.821 |
| walker |  | 9729 | 62 | Fs::DirListing { dir: apps/web/components/LinkViews/LinkComponents } |  |  | 0.825 |
| ns | 9884 |  | 202 | Mobile components, stores and query-cache modules | 8.2 | 8.1 | 0.827 |
| walker |  | 9897 | 168 | Fs::DirListing { dir: apps/web/components/ModalContent } |  |  | 0.845 |
| walker |  | 9939 | 42 | Json::Identity { file: packages/lib/package.json } |  |  | 0.845 |
| walker |  | 9951 | 12 | Json::Entry { file: packages/lib/package.json } |  |  | 0.845 |
| walker |  | 9993 | 42 | Json::Identity { file: packages/router/package.json } |  |  | 0.846 |

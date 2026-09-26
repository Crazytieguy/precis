Score(3000)=0.741 I=0.896 C=0.612 ns_rows≤3K=15/51 grid(1000/1442/2080/3000/4327/6240/9000)=0.814/0.716/0.688/0.741/0.651/0.680/0.790

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
| walker |  | 299 | 43 | Json::Identity { file: package.json } |  |  | 0.751 |
| ns | 321 |  | 62 | Workspace globs in the root package.json | 1.5 |  | 0.713 |
| walker |  | 340 | 41 | Json::Scripts { file: package.json } |  |  | 0.713 |
| walker |  | 366 | 26 | Fs::DirListing { dir: apps/worker } |  |  | 0.713 |
| walker |  | 371 | 5 | Fs::DirListing { dir: apps/worker/templates } |  |  | 0.713 |
| walker |  | 452 | 81 | Fs::DirListing { dir: apps/web } |  |  | 0.949 |
| ns | 460 |  | 139 | Workspace package names — every `name` field under apps/ and packages/ | 1.6 |  | 0.879 |
| walker |  | 470 | 18 | Fs::DirListing { dir: apps/web/e2e } |  |  | 0.879 |
| walker |  | 474 | 4 | Fs::DirListing { dir: apps/web/styles } |  |  | 0.879 |
| walker |  | 478 | 4 | Fs::DirListing { dir: .vscode } |  |  | 0.879 |
| walker |  | 491 | 13 | Fs::DirListing { dir: packages/types } |  |  | 0.879 |
| walker |  | 495 | 4 | Fs::DirListing { dir: apps/web/e2e/data } |  |  | 0.879 |
| walker |  | 528 | 33 | Fs::DirListing { dir: assets } |  |  | 0.879 |
| walker |  | 533 | 5 | Fs::DirListing { dir: .devcontainer } |  |  | 0.879 |
| walker |  | 542 | 9 | Fs::DirListing { dir: apps/web/store } |  |  | 0.879 |
| walker |  | 596 | 54 | Fs::DirListing { dir: packages/filesystem } |  |  | 0.880 |
| walker |  | 608 | 12 | Fs::DirListing { dir: apps/web/types } |  |  | 0.880 |
| walker |  | 621 | 13 | Fs::DirListing { dir: apps/web/lib } |  |  | 0.880 |
| ns | 674 |  | 214 | Root scripts: how you run web, worker and both together | 1.7 |  | 0.816 |
| walker |  | 707 | 86 | Fs::DirListing { dir: apps/web/pages } |  |  | 0.817 |
| walker |  | 718 | 11 | Fs::DirListing { dir: apps/web/pages/collections } |  |  | 0.817 |
| walker |  | 729 | 11 | Fs::DirListing { dir: apps/web/pages/tags } |  |  | 0.817 |
| walker |  | 745 | 16 | Fs::DirListing { dir: apps/web/pages/links } |  |  | 0.817 |
| walker |  | 764 | 19 | Fs::DirListing { dir: apps/web/pages/admin } |  |  | 0.818 |
| walker |  | 770 | 6 | Fs::DirListing { dir: apps/web/pages/preserved } |  |  | 0.818 |
| walker |  | 826 | 56 | Fs::DirListing { dir: apps/web/pages/settings } |  |  | 0.819 |
| walker |  | 834 | 8 | Fs::DirListing { dir: apps/web/pages/api } |  |  | 0.819 |
| ns | 837 |  | 163 | Root scripts: prisma, format, test, coverage, postinstall | 1.8 |  | 0.768 |
| walker |  | 839 | 5 | Fs::DirListing { dir: apps/web/pages/api/v2/dashboard } |  |  | 0.768 |
| walker |  | 848 | 9 | Fs::DirListing { dir: apps/web/pages/public } |  |  | 0.769 |
| walker |  | 854 | 6 | Fs::DirListing { dir: apps/web/pages/public/links } |  |  | 0.769 |
| walker |  | 860 | 6 | Fs::DirListing { dir: apps/web/pages/public/preserved } |  |  | 0.769 |
| walker |  | 872 | 12 | Fs::DirListing { dir: apps/web/pages/auth } |  |  | 0.769 |
| walker |  | 904 | 32 | Json::Entry { file: package.json } |  |  | 0.813 |
| walker |  | 925 | 21 | Fs::DirListing { dir: apps/web/templates } |  |  | 0.813 |
| walker |  | 931 | 6 | Fs::DirListing { dir: apps/web/e2e/tests } |  |  | 0.813 |
| walker |  | 936 | 5 | Fs::DirListing { dir: apps/web/e2e/tests/public } |  |  | 0.813 |
| walker |  | 960 | 24 | Fs::DirListing { dir: apps/web/layouts } |  |  | 0.813 |
| ns | 1082 |  | 245 | Feature list, first half (preservation, reading, organisation, sharing) | 1.9 |  | 0.752 |
| walker |  | 1180 | 220 | Prisma::Toc { file: packages/prisma/schema.prisma } |  |  | 0.762 |
| walker |  | 1200 | 20 | Fs::DirListing { dir: apps/web/lib/shared } |  |  | 0.762 |
| walker |  | 1213 | 13 | Fs::DirListing { dir: apps/web/pages/public/collections/[id] } |  |  | 0.763 |
| walker |  | 1249 | 36 | Fs::DirListing { dir: apps/worker/workers } |  |  | 0.763 |
| ns | 1271 |  | 189 | Feature list, second half (sync, SSO, API keys, i18n, RSS, uploads) | 1.10 |  | 0.712 |
| walker |  | 1321 | 72 | Fs::DirListing { dir: packages/router } |  |  | 0.713 |
| walker |  | 1331 | 10 | Fs::DirListing { dir: apps/web/e2e/tests/global } |  |  | 0.713 |
| walker |  | 1355 | 24 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 77 } |  |  | 0.714 |
| walker |  | 1404 | 49 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 90 } |  |  | 0.716 |
| walker |  | 1484 | 80 | Fs::DirListing { dir: apps/mobile } |  |  | 0.717 |
| walker |  | 1488 | 4 | Fs::DirListing { dir: apps/mobile/styles } |  |  | 0.717 |
| walker |  | 1494 | 6 | Fs::DirListing { dir: apps/mobile/assets } |  |  | 0.717 |
| walker |  | 1530 | 36 | Fs::DirListing { dir: apps/mobile/app } |  |  | 0.717 |
| walker |  | 1538 | 8 | Fs::DirListing { dir: apps/mobile/plugins } |  |  | 0.717 |
| ns | 1539 |  | 268 | schema.prisma: datasource/generator plus every model and enum header, bodies elided | 2.1 |  | 0.665 |
| walker |  | 1544 | 6 | Fs::DirListing { dir: apps/mobile/app/links } |  |  | 0.665 |
| walker |  | 1550 | 6 | Fs::DirListing { dir: apps/mobile/assets/fonts } |  |  | 0.665 |
| walker |  | 1561 | 11 | Fs::DirListing { dir: apps/mobile/types } |  |  | 0.665 |
| walker |  | 1573 | 12 | Fs::DirListing { dir: apps/mobile/store } |  |  | 0.665 |
| walker |  | 1596 | 23 | Fs::DirListing { dir: apps/mobile/lib } |  |  | 0.665 |
| walker |  | 1617 | 21 | Fs::DirListing { dir: apps/mobile/app/(tabs) } |  |  | 0.665 |
| walker |  | 1628 | 11 | Fs::DirListing { dir: apps/mobile/app/(tabs)/links } |  |  | 0.665 |
| walker |  | 1645 | 17 | Fs::DirListing { dir: apps/mobile/app/(tabs)/collections } |  |  | 0.666 |
| walker |  | 1662 | 17 | Fs::DirListing { dir: apps/mobile/app/(tabs)/dashboard } |  |  | 0.666 |
| walker |  | 1679 | 17 | Fs::DirListing { dir: apps/mobile/app/(tabs)/settings } |  |  | 0.666 |
| walker |  | 1696 | 17 | Fs::DirListing { dir: apps/mobile/app/(tabs)/tags } |  |  | 0.667 |
| ns | 1697 |  | 158 | All five schema enums, fully expanded | 2.2 | 2.1 | 0.635 |
| walker |  | 1720 | 24 | Fs::DirListing { dir: apps/mobile/assets/images } |  |  | 0.635 |
| walker |  | 1746 | 26 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 304 } |  |  | 0.651 |
| walker |  | 1781 | 35 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 83 } |  |  | 0.678 |
| walker |  | 1991 | 210 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 166 } |  |  | 0.684 |
| ns | 2083 |  | 386 | `Link` model — every field | 2.3 | 2.1 | 0.651 |
| walker |  | 2170 | 179 | Prisma::DeclTail { file: packages/prisma/schema.prisma, start_line: 166, tail_start_line: 182 } |  |  | 0.727 |
| walker |  | 2199 | 29 | Fs::DirListing { dir: .github } |  |  | 0.727 |
| walker |  | 2220 | 21 | Fs::DirListing { dir: .github/workflows } |  |  | 0.727 |
| walker |  | 2259 | 39 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 289 } |  |  | 0.756 |
| ns | 2396 |  | 313 | `User` model — identity, relations and subscription linkage | 2.4 | 2.1 | 0.710 |
| walker |  | 2431 | 172 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 10 } |  |  | 0.712 |
| walker |  | 2513 | 82 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 108 } |  |  | 0.715 |
| walker |  | 2622 | 109 | Fs::DirListing { dir: packages/lib } |  |  | 0.719 |
| walker |  | 2679 | 57 | Fs::DirListing { dir: apps/worker/lib } |  |  | 0.720 |
| ns | 2738 |  | 342 | `User` model — preference and archival-toggle fields | 2.5 | 2.4 | 0.683 |
| walker |  | 2964 | 285 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 28 } |  |  | 0.741 |
| ns | 3055 |  | 317 | `Collection` model — every field, including the self-relation | 2.6 | 2.1 | 0.706 |
| walker |  | 3101 | 137 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.706 |
| walker |  | 3101 | 0 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.706 |
| walker |  | 3256 | 155 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.706 |
| walker |  | 3287 | 31 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.706 |
| walker |  | 3316 | 29 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.706 |
| walker |  | 3375 | 59 | Fs::DirListing { dir: apps/mobile/components } |  |  | 0.706 |
| walker |  | 3401 | 26 | Fs::DirListing { dir: apps/mobile/components/Formats } |  |  | 0.707 |
| walker |  | 3433 | 32 | Fs::DirListing { dir: apps/mobile/components/ActionSheets } |  |  | 0.707 |
| ns | 3436 |  | 381 | `UsersAndCollections` join model (the permission bits) and `Tag` model | 2.7 | 2.1 | 0.667 |
| ns | 3594 |  | 158 | `AccessToken` model | 2.8 | 2.1 | 0.654 |
| ns | 3617 |  | 23 | Complete packages/prisma listing | 2.9 |  | 0.659 |
| walker |  | 3651 | 218 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 200 } |  |  | 0.684 |
| walker |  | 3713 | 62 | Fs::DirListing { dir: apps/web/hooks } |  |  | 0.685 |
| ns | 3733 |  | 116 | The process-wide `prisma` client singleton | 2.10 |  | 0.673 |
| walker |  | 3778 | 65 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 118 } |  |  | 0.675 |
| ns | 3815 |  | 82 | API version directories: every resource under pages/api/v1 (and the single v2 route) | 3.1 |  | 0.647 |
| walker |  | 3874 | 96 | Json::Runtime { file: package.json } |  |  | 0.647 |
| ns | 3883 |  | 68 | Route files for links, collections, tags and highlights | 3.2 |  | 0.631 |
| walker |  | 3891 | 17 | Fs::DirListing { dir: apps/web/e2e/fixtures } |  |  | 0.631 |
| walker |  | 3904 | 13 | Fs::DirListing { dir: apps/web/e2e/fixtures/base } |  |  | 0.632 |
| walker |  | 3921 | 17 | Fs::DirListing { dir: apps/web/scripts/migration } |  |  | 0.632 |
| walker |  | 3925 | 4 | Fs::DirListing { dir: apps/web/scripts/migration/v2.6.1 } |  |  | 0.632 |
| walker |  | 3952 | 27 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 5 } |  |  | 0.646 |
| ns | 3960 |  | 77 | Route files for auth, session, users, tokens, config, avatar and logins | 3.3 |  | 0.631 |
| walker |  | 4021 | 69 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 296 } |  |  | 0.633 |
| ns | 4040 |  | 80 | Route files for archives, preserved, search, dashboard, rss, migration, payment, webhook, worker, getFavicon | 3.4 |  | 0.618 |
| ns | 4081 |  | 41 | The unauthenticated `public/` route subtree | 3.5 |  | 0.609 |
| walker |  | 4190 | 169 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 151 } |  |  | 0.645 |
| ns | 4357 |  | 276 | The route-handler pattern, read from pages/api/v1/links/index.ts | 3.6 |  | 0.628 |
| ns | 4566 |  | 209 | apps/web/lib: the server helper layer and its client/shared siblings | 3.7 |  | 0.601 |
| walker |  | 4625 | 435 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: true } |  |  | 0.649 |
| ns | 4665 |  | 99 | The controller tree: every resource directory under lib/api/controllers | 3.8 |  | 0.632 |
| walker |  | 4798 | 173 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 260 } |  |  | 0.634 |
| ns | 4821 |  | 156 | Controller files for links, collections, tags and highlights | 3.9 |  | 0.615 |
| walker |  | 4928 | 130 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 248 } |  |  | 0.617 |
| ns | 4997 |  | 176 | Controller files for users, tokens, session, search, dashboard, worker, migration and public access | 3.10 |  | 0.597 |
| ns | 5245 |  | 248 | Complete listings of packages/types, packages/lib, packages/filesystem and packages/router | 4.1 |  | 0.626 |
| walker |  | 5248 | 320 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 126 } |  |  | 0.656 |
| walker |  | 5409 | 161 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 234 } |  |  | 0.671 |
| walker |  | 5451 | 42 | Fs::DirListing { dir: apps/worker/lib/preservationScheme } |  |  | 0.672 |
| walker |  | 5619 | 168 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 275 } |  |  | 0.674 |
| ns | 5643 |  | 398 | packages/types/global.ts — every exported type, interface and enum declaration | 4.2 |  | 0.656 |
| walker |  | 5789 | 170 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 220 } |  |  | 0.649 |
| ns | 5789 |  | 146 | `ArchivedFormat`, `LinkType` and `TokenExpiry` variants | 4.3 | 4.2 | 0.649 |
| ns | 5941 |  | 152 | `ViewMode`, `Sort` and `TagSort` variants | 4.4 | 4.2 | 0.642 |
| walker |  | 6162 | 373 | Prisma::DeclTail { file: packages/prisma/schema.prisma, start_line: 28, tail_start_line: 51 } |  |  | 0.674 |
| walker |  | 6178 | 16 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 1 } |  |  | 0.680 |
| walker |  | 6257 | 79 | Fs::DirListing { dir: apps/web/public } |  |  | 0.680 |
| walker |  | 6267 | 10 | Fs::DirListing { dir: apps/web/public/screenshots } |  |  | 0.680 |
| ns | 6281 |  | 340 | packages/lib/schemaValidation.ts — every exported zod schema constant | 4.5 |  | 0.668 |
| walker |  | 6368 | 101 | Prisma::Decl { file: packages/prisma/schema.prisma, start_line: 99 } |  |  | 0.669 |
| walker |  | 6399 | 31 | Code::CodeKey { rung: Names, file: apps/worker/worker.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.669 |
| walker |  | 6415 | 16 | Code::CodeKey { rung: Decl, file: apps/worker/worker.ts, decl: 1, sub: 0, line: 8 } |  |  | 0.669 |
| walker |  | 6437 | 22 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.669 |
| ns | 6538 |  | 257 | packages/router — the links, collections and tags hook exports | 4.6 |  | 0.654 |
| walker |  | 6572 | 135 | Json::Dependencies { file: package.json } |  |  | 0.654 |
| walker |  | 6622 | 50 | Fs::DirListing { dir: apps/mobile/components/ui } |  |  | 0.655 |
| walker |  | 6722 | 100 | Code::CodeKey { rung: Names, file: packages/filesystem/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.650 |
| ns | 6722 |  | 184 | packages/router — export lines of the remaining ten hook modules | 4.7 | 4.6 | 0.650 |
| walker |  | 6734 | 12 | Code::CodeKey { rung: Names, file: packages/prisma/client/index.js, decl: 0, sub: 0, line: 0 } |  |  | 0.650 |
| walker |  | 6746 | 12 | Code::CodeKey { rung: Names, file: packages/prisma/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.650 |
| walker |  | 6822 | 76 | Code::CodeKey { rung: Decl, file: packages/prisma/index.ts, decl: 1, sub: 0, line: 5 } |  |  | 0.656 |
| walker |  | 6860 | 38 | Code::CodeKey { rung: Names, file: apps/worker/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.656 |
| walker |  | 6886 | 26 | Code::CodeKey { rung: Names, file: apps/web/e2e/index.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.656 |
| ns | 6888 |  | 166 | Complete apps/worker listing, including every job and every preservation handler | 5.1 |  | 0.668 |
| walker |  | 6900 | 14 | Code::CodeKey { rung: Names, file: apps/web/pages/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.668 |
| walker |  | 6914 | 14 | Code::CodeKey { rung: Names, file: apps/web/pages/settings/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.668 |
| walker |  | 6929 | 15 | Code::CodeKey { rung: Names, file: apps/mobile/app/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.668 |
| walker |  | 6992 | 63 | Code::CodeKey { rung: Names, file: apps/web/pages/collections/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.668 |
| walker |  | 7055 | 63 | Code::CodeKey { rung: Names, file: apps/web/pages/links/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.668 |
| walker |  | 7120 | 65 | Code::CodeKey { rung: Names, file: apps/web/pages/tags/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.668 |
| walker |  | 7135 | 15 | Code::CodeKey { rung: Body, file: apps/web/pages/collections/index.tsx, decl: 2, sub: 0, line: 153 } |  |  | 0.668 |
| walker |  | 7150 | 15 | Code::CodeKey { rung: Body, file: apps/web/pages/links/index.tsx, decl: 2, sub: 0, line: 70 } |  |  | 0.668 |
| ns | 7157 |  | 269 | worker.ts — the whole scheduler entry point | 5.2 |  | 0.658 |
| walker |  | 7165 | 15 | Code::CodeKey { rung: Body, file: apps/web/pages/tags/index.tsx, decl: 2, sub: 0, line: 274 } |  |  | 0.658 |
| walker |  | 7244 | 79 | Fs::DirListing { dir: apps/web/lib/client } |  |  | 0.667 |
| ns | 7350 |  | 193 | archiveHandler's signature and its SSRF / skip-preservation guard | 5.3 |  | 0.657 |
| walker |  | 7379 | 135 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.657 |
| walker |  | 7416 | 37 | Code::CodeKey { rung: Names, file: apps/web/pages/admin/index.tsx, decl: 0, sub: 0, line: 0 } |  |  | 0.657 |
| walker |  | 7424 | 8 | Code::CodeKey { rung: Body, file: apps/web/pages/admin/index.tsx, decl: 2, sub: 0, line: 12 } |  |  | 0.657 |
| walker |  | 7443 | 19 | Code::CodeKey { rung: Names, file: packages/filesystem/readFile.ts, decl: 0, sub: 0, line: 0 } |  |  | 0.657 |
| ns | 7601 |  | 251 | Every UI page route under apps/web/pages | 6.1 |  | 0.675 |
| walker |  | 7753 | 310 | Plaintext::Whole { file: docker-compose.yml } |  |  | 0.676 |
| walker |  | 7850 | 97 | Fs::DirListing { dir: apps/web/lib/api } |  |  | 0.700 |
| walker |  | 7863 | 13 | Fs::DirListing { dir: apps/web/lib/api/archives } |  |  | 0.700 |
| walker |  | 7880 | 17 | Fs::DirListing { dir: apps/web/lib/api/preserved } |  |  | 0.700 |
| walker |  | 7913 | 33 | Fs::DirListing { dir: apps/web/lib/api/stripe } |  |  | 0.704 |
| walker |  | 7949 | 36 | Fs::DirListing { dir: apps/web/lib/api/controllers } |  |  | 0.719 |
| walker |  | 7954 | 5 | Fs::DirListing { dir: apps/web/lib/api/controllers/search } |  |  | 0.719 |
| walker |  | 7959 | 5 | Fs::DirListing { dir: apps/web/lib/api/controllers/session } |  |  | 0.690 |
| ns | 7959 |  | 358 | The flat components/ directory and the ui/ primitives | 6.2 |  | 0.690 |
| walker |  | 7965 | 6 | Fs::DirListing { dir: apps/web/lib/api/controllers/worker } |  |  | 0.690 |
| walker |  | 7974 | 9 | Fs::DirListing { dir: apps/web/lib/api/controllers/public } |  |  | 0.691 |
| walker |  | 7980 | 6 | Fs::DirListing { dir: apps/web/lib/api/controllers/public/collections } |  |  | 0.691 |
| walker |  | 7986 | 6 | Fs::DirListing { dir: apps/web/lib/api/controllers/public/users } |  |  | 0.692 |
| walker |  | 8000 | 14 | Fs::DirListing { dir: apps/web/lib/api/controllers/collections } |  |  | 0.692 |
| walker |  | 8014 | 14 | Fs::DirListing { dir: apps/web/lib/api/controllers/highlights } |  |  | 0.692 |
| walker |  | 8028 | 14 | Fs::DirListing { dir: apps/web/lib/api/controllers/tokens } |  |  | 0.694 |
| walker |  | 8035 | 7 | Fs::DirListing { dir: apps/web/lib/api/controllers/tokens/tokenId } |  |  | 0.695 |
| walker |  | 8049 | 14 | Fs::DirListing { dir: apps/web/lib/api/controllers/users } |  |  | 0.697 |
| walker |  | 8066 | 17 | Fs::DirListing { dir: apps/web/lib/api/controllers/links } |  |  | 0.698 |
| walker |  | 8078 | 12 | Fs::DirListing { dir: apps/web/lib/api/controllers/links/bulk } |  |  | 0.699 |
| walker |  | 8087 | 9 | Fs::DirListing { dir: apps/web/lib/api/controllers/public/links/linkId } |  |  | 0.701 |
| walker |  | 8107 | 20 | Fs::DirListing { dir: apps/web/lib/api/controllers/dashboard } |  |  | 0.704 |
| walker |  | 8134 | 27 | Fs::DirListing { dir: apps/web/lib/api/controllers/tags } |  |  | 0.708 |
| walker |  | 8155 | 21 | Fs::DirListing { dir: apps/web/lib/api/controllers/collections/collectionId } |  |  | 0.711 |
| walker |  | 8176 | 21 | Fs::DirListing { dir: apps/web/lib/api/controllers/tags/tagId } |  |  | 0.715 |
| walker |  | 8200 | 24 | Fs::DirListing { dir: apps/web/lib/api/controllers/links/linkId } |  |  | 0.720 |
| walker |  | 8206 | 6 | Fs::DirListing { dir: apps/web/lib/api/controllers/links/linkId/highlight } |  |  | 0.722 |
| walker |  | 8233 | 27 | Fs::DirListing { dir: apps/web/lib/api/controllers/users/userId } |  |  | 0.726 |
| ns | 8240 |  | 281 | Modal, link-view, preservation and input-picker component subdirectories | 6.3 | 6.2 | 0.708 |
| walker |  | 8302 | 69 | Fs::DirListing { dir: apps/web/pages/api/v1 } |  |  | 0.728 |
| walker |  | 8306 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/dashboard } |  |  | 0.728 |
| walker |  | 8310 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/getFavicon } |  |  | 0.729 |
| walker |  | 8314 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/logins } |  |  | 0.729 |
| walker |  | 8318 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/migration } |  |  | 0.729 |
| walker |  | 8322 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/payment } |  |  | 0.729 |
| walker |  | 8326 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/search } |  |  | 0.730 |
| walker |  | 8330 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/session } |  |  | 0.730 |
| walker |  | 8334 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/webhook } |  |  | 0.730 |
| walker |  | 8343 | 9 | Fs::DirListing { dir: apps/web/pages/api/v1/config } |  |  | 0.731 |
| walker |  | 8352 | 9 | Fs::DirListing { dir: apps/web/pages/api/v1/worker } |  |  | 0.732 |
| walker |  | 8362 | 10 | Fs::DirListing { dir: apps/web/pages/api/v1/collections } |  |  | 0.732 |
| walker |  | 8372 | 10 | Fs::DirListing { dir: apps/web/pages/api/v1/highlights } |  |  | 0.733 |
| walker |  | 8382 | 10 | Fs::DirListing { dir: apps/web/pages/api/v1/rss } |  |  | 0.735 |
| walker |  | 8392 | 10 | Fs::DirListing { dir: apps/web/pages/api/v1/tokens } |  |  | 0.736 |
| walker |  | 8404 | 12 | Fs::DirListing { dir: apps/web/pages/api/v1/links } |  |  | 0.737 |
| walker |  | 8408 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/links/archive } |  |  | 0.738 |
| walker |  | 8418 | 10 | Fs::DirListing { dir: apps/web/pages/api/v1/links/[id] } |  |  | 0.741 |
| walker |  | 8422 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/links/[id]/archive } |  |  | 0.742 |
| walker |  | 8426 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/links/[id]/highlights } |  |  | 0.743 |
| walker |  | 8439 | 13 | Fs::DirListing { dir: apps/web/pages/api/v1/users } |  |  | 0.745 |
| walker |  | 8448 | 9 | Fs::DirListing { dir: apps/web/pages/api/v1/users/[id] } |  |  | 0.747 |
| walker |  | 8462 | 14 | Fs::DirListing { dir: apps/web/pages/api/v1/tags } |  |  | 0.752 |
| ns | 8466 |  | 226 | Web hooks, layouts, stores, ambient types, email templates, one-off migration scripts and the Playwright suite | 6.4 |  | 0.759 |
| walker |  | 8481 | 19 | Fs::DirListing { dir: apps/web/pages/api/v1/archives } |  |  | 0.762 |
| walker |  | 8487 | 6 | Fs::DirListing { dir: apps/web/pages/api/v1/avatar } |  |  | 0.763 |
| walker |  | 8496 | 9 | Fs::DirListing { dir: apps/web/pages/api/v1/public } |  |  | 0.764 |
| walker |  | 8502 | 6 | Fs::DirListing { dir: apps/web/pages/api/v1/public/links } |  |  | 0.764 |
| walker |  | 8508 | 6 | Fs::DirListing { dir: apps/web/pages/api/v1/public/users } |  |  | 0.765 |
| walker |  | 8520 | 12 | Fs::DirListing { dir: apps/web/pages/api/v1/public/collections } |  |  | 0.768 |
| walker |  | 8524 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/public/collections/links } |  |  | 0.769 |
| walker |  | 8528 | 4 | Fs::DirListing { dir: apps/web/pages/api/v1/public/collections/tags } |  |  | 0.771 |
| walker |  | 8546 | 18 | Fs::DirListing { dir: apps/web/pages/api/v1/preserved } |  |  | 0.776 |
| walker |  | 8568 | 22 | Fs::DirListing { dir: apps/web/pages/api/v1/auth } |  |  | 0.781 |
| walker |  | 8616 | 48 | Fs::DirListing { dir: apps/web/lib/api/controllers/migration } |  |  | 0.789 |
| walker |  | 8704 | 88 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.789 |
| ns | 8706 |  | 240 | verifyUser — the guard chain every authenticated route runs first | 7.1 |  | 0.778 |
| walker |  | 8808 | 104 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.778 |
| ns | 8941 |  | 235 | docker-compose.yml — the three-container deployment | 7.2 |  | 0.781 |
| walker |  | 9146 | 338 | Json::ScriptsTail { file: package.json } |  |  | 0.804 |
| ns | 9155 |  | 214 | .env.sample — the required variables and the complete list of section headings | 7.3 |  | 0.795 |
| walker |  | 9243 | 97 | Markdown::Section { file: README.md, section_index: 10, keeps_default_concavity: false } |  |  | 0.795 |
| ns | 9334 |  | 179 | The head of the optional tuning-variable block | 7.4 | 7.3 | 0.787 |
| ns | 9466 |  | 132 | CI workflows, GitHub templates, the patch-package patch, and every translated locale | 7.5 |  | 0.779 |
| walker |  | 9560 | 317 | Fs::DirListing { dir: apps/web/components } |  |  | 0.807 |
| walker |  | 9569 | 9 | Fs::DirListing { dir: apps/web/components/LinkViews } |  |  | 0.807 |
| walker |  | 9585 | 16 | Fs::DirListing { dir: apps/web/components/InputSelect } |  |  | 0.807 |
| walker |  | 9611 | 26 | Fs::DirListing { dir: apps/web/components/Preservation } |  |  | 0.808 |
| walker |  | 9652 | 41 | Fs::DirListing { dir: apps/web/components/ui } |  |  | 0.816 |
| ns | 9682 |  | 216 | apps/mobile root and the complete Expo Router screen tree | 8.1 |  | 0.821 |
| walker |  | 9714 | 62 | Fs::DirListing { dir: apps/web/components/LinkViews/LinkComponents } |  |  | 0.825 |
| walker |  | 9882 | 168 | Fs::DirListing { dir: apps/web/components/ModalContent } |  |  | 0.843 |
| ns | 9884 |  | 202 | Mobile components, stores and query-cache modules | 8.2 | 8.1 | 0.845 |
| walker |  | 9967 | 85 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.845 |

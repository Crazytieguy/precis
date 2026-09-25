Score(3000)=0.624 I=0.819 C=0.475 ns_rows≤3K=18/51 grid(1000/1442/2080/3000/4327/6240/9000)=0.819/0.707/0.716/0.624/0.554/0.477/0.471

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 41 |  | 41 | What linkding is | 1.1 |  | 0.000 |
| ns | 112 |  | 71 | The one sentence that locates the code | 1.2 |  | 0.000 |
| walker |  | 115 | 115 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 127 | 12 | Fs::DirListing { dir: docker } |  |  | 0.000 |
| walker |  | 138 | 11 | Code::CodeKey { rung: Names, file: manage.py, decl: 0, sub: 0, line: 0 } |  |  | 0.000 |
| walker |  | 141 | 3 | Fs::DirListing { dir: .github } |  |  | 0.000 |
| walker |  | 154 | 13 | Fs::DirListing { dir: .github/workflows } |  |  | 0.000 |
| walker |  | 159 | 5 | Fs::DirListing { dir: .devcontainer } |  |  | 0.000 |
| walker |  | 203 | 44 | Fs::DirListing { dir: assets } |  |  | 0.000 |
| ns | 227 |  | 115 | Repository root listing (complete) | 1.3 |  | 0.714 |
| walker |  | 257 | 54 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.945 |
| walker |  | 343 | 86 | Toml::Identity { file: pyproject.toml } |  |  | 0.947 |
| ns | 348 |  | 121 | bookmarks/ app package listing (complete) | 1.4 |  | 0.649 |
| walker |  | 353 | 10 | Plaintext::Whole { file: version.txt } |  |  | 0.649 |
| ns | 445 |  | 97 | README feature overview (head) | 1.5 |  | 0.605 |
| walker |  | 474 | 121 | Fs::DirListing { dir: bookmarks } |  |  | 0.894 |
| walker |  | 477 | 3 | Fs::DirListing { dir: bookmarks/management } |  |  | 0.894 |
| walker |  | 497 | 20 | Fs::DirListing { dir: bookmarks/api } |  |  | 0.895 |
| walker |  | 517 | 20 | Fs::DirListing { dir: bookmarks/templatetags } |  |  | 0.895 |
| walker |  | 539 | 22 | Fs::DirListing { dir: bookmarks/settings } |  |  | 0.898 |
| walker |  | 551 | 12 | Code::CodeKey { rung: Names, file: bookmarks/wsgi.py, decl: 0, sub: 0, line: 0 } |  |  | 0.898 |
| walker |  | 569 | 18 | Fs::DirListing { dir: bookmarks/frontend } |  |  | 0.898 |
| ns | 583 |  | 138 | README feature overview (tail) | 1.6 | 1.5 | 0.856 |
| walker |  | 584 | 15 | Code::CodeKey { rung: Names, file: bookmarks/apps.py, decl: 0, sub: 0, line: 0 } |  |  | 0.856 |
| walker |  | 599 | 15 | Code::CodeKey { rung: Names, file: bookmarks/type_defs.py, decl: 0, sub: 0, line: 0 } |  |  | 0.856 |
| walker |  | 616 | 17 | Code::CodeKey { rung: Names, file: bookmarks/validators.py, decl: 0, sub: 0, line: 0 } |  |  | 0.856 |
| walker |  | 631 | 15 | Code::CodeKey { rung: Decl, file: bookmarks/validators.py, decl: 1, sub: 0, line: 5 } |  |  | 0.856 |
| walker |  | 649 | 18 | Code::CodeKey { rung: Names, file: bookmarks/signals.py, decl: 0, sub: 0, line: 0 } |  |  | 0.856 |
| walker |  | 658 | 9 | Code::CodeKey { rung: Decl, file: bookmarks/signals.py, decl: 1, sub: 0, line: 6 } |  |  | 0.856 |
| ns | 679 |  | 96 | bookmarks/views/ and bookmarks/api/ listings (complete) | 1.7 |  | 0.748 |
| walker |  | 685 | 27 | Fs::DirListing { dir: bookmarks/templates } |  |  | 0.748 |
| walker |  | 690 | 5 | Fs::DirListing { dir: bookmarks/templates/admin } |  |  | 0.748 |
| walker |  | 706 | 16 | Fs::DirListing { dir: bookmarks/templates/registration } |  |  | 0.748 |
| ns | 762 |  | 83 | bookmarks/services/ listing (complete) | 1.8 |  | 0.681 |
| walker |  | 782 | 76 | Fs::DirListing { dir: bookmarks/views } |  |  | 0.800 |
| walker |  | 794 | 12 | Code::CodeKey { rung: Names, file: bookmarks/views/health.py, decl: 0, sub: 0, line: 0 } |  |  | 0.800 |
| walker |  | 806 | 12 | Code::CodeKey { rung: Names, file: bookmarks/views/manifest.py, decl: 0, sub: 0, line: 0 } |  |  | 0.800 |
| walker |  | 818 | 12 | Code::CodeKey { rung: Names, file: bookmarks/views/root.py, decl: 0, sub: 0, line: 0 } |  |  | 0.800 |
| walker |  | 830 | 12 | Code::CodeKey { rung: Names, file: bookmarks/views/toasts.py, decl: 0, sub: 0, line: 0 } |  |  | 0.800 |
| walker |  | 838 | 8 | Code::CodeKey { rung: Decl, file: bookmarks/views/toasts.py, decl: 1, sub: 0, line: 9 } |  |  | 0.800 |
| walker |  | 862 | 24 | Code::CodeKey { rung: Names, file: bookmarks/context_processors.py, decl: 0, sub: 0, line: 0 } |  |  | 0.800 |
| ns | 937 |  | 175 | Dev prerequisites + core make targets | 1.9 |  | 0.735 |
| walker |  | 945 | 83 | Fs::DirListing { dir: bookmarks/services } |  |  | 0.819 |
| walker |  | 970 | 25 | Code::CodeKey { rung: Decl, file: bookmarks/apps.py, decl: 1, sub: 0, line: 4 } |  |  | 0.819 |
| walker |  | 984 | 14 | Code::CodeKey { rung: Names, file: bookmarks/views/opensearch.py, decl: 0, sub: 0, line: 0 } |  |  | 0.819 |
| walker |  | 1004 | 20 | Fs::DirListing { dir: bookmarks/templates/bundles } |  |  | 0.819 |
| walker |  | 1024 | 20 | Fs::DirListing { dir: bookmarks/templates/tags } |  |  | 0.819 |
| walker |  | 1040 | 16 | Code::CodeKey { rung: Names, file: bookmarks/api/auth.py, decl: 0, sub: 0, line: 0 } |  |  | 0.819 |
| walker |  | 1056 | 16 | Code::CodeKey { rung: Names, file: bookmarks/services/wayback.py, decl: 0, sub: 0, line: 0 } |  |  | 0.819 |
| walker |  | 1087 | 31 | Code::CodeKey { rung: Names, file: bookmarks/urls.py, decl: 0, sub: 0, line: 0 } |  |  | 0.819 |
| walker |  | 1119 | 32 | Code::CodeKey { rung: Decl, file: bookmarks/type_defs.py, decl: 1, sub: 0, line: 11 } |  |  | 0.819 |
| walker |  | 1146 | 27 | Fs::DirListing { dir: bookmarks/frontend/utils } |  |  | 0.819 |
| ns | 1154 |  | 217 | Remaining make targets: lint, format, e2e, frontend | 1.10 | 1.9 | 0.749 |
| walker |  | 1173 | 27 | Fs::DirListing { dir: bookmarks/templates/settings } |  |  | 0.749 |
| ns | 1438 |  | 284 | pyproject.toml project metadata + runtime dependencies | 1.11 |  | 0.707 |
| ns | 1460 |  | 22 | bookmarks/settings/ listing (complete) | 1.12 |  | 0.712 |
| walker |  | 1534 | 361 | Plaintext::Whole { file: Makefile } |  |  | 0.830 |
| ns | 1602 |  | 142 | Settings resolution order + manage.py/pytest wiring | 1.13 | 1.12 | 0.800 |
| walker |  | 1603 | 69 | Fs::DirListing { dir: bookmarks/styles } |  |  | 0.800 |
| walker |  | 1627 | 24 | Code::CodeKey { rung: Decl, file: bookmarks/services/wayback.py, decl: 1, sub: 0, line: 6 } |  |  | 0.800 |
| walker |  | 1668 | 41 | Fs::DirListing { dir: bookmarks/templates/shared } |  |  | 0.800 |
| ns | 1710 |  | 108 | docs/ site and its content pages (complete) | 1.14 |  | 0.747 |
| walker |  | 1725 | 57 | Code::CodeKey { rung: Names, file: bookmarks/middlewares.py, decl: 0, sub: 0, line: 0 } |  |  | 0.747 |
| walker |  | 1738 | 13 | Code::CodeKey { rung: Decl, file: bookmarks/middlewares.py, decl: 1, sub: 0, line: 7 } |  |  | 0.747 |
| walker |  | 1767 | 29 | Code::CodeKey { rung: Decl, file: bookmarks/middlewares.py, decl: 4, sub: 0, line: 17 } |  |  | 0.747 |
| walker |  | 1798 | 31 | Code::CodeKey { rung: Names, file: bookmarks/services/monolith.py, decl: 0, sub: 0, line: 0 } |  |  | 0.747 |
| walker |  | 1805 | 7 | Code::CodeKey { rung: Decl, file: bookmarks/services/monolith.py, decl: 1, sub: 0, line: 9 } |  |  | 0.747 |
| walker |  | 1851 | 46 | Fs::DirListing { dir: bookmarks/management/commands } |  |  | 0.748 |
| walker |  | 1864 | 13 | Code::CodeKey { rung: Names, file: bookmarks/management/commands/backup.py, decl: 0, sub: 0, line: 0 } |  |  | 0.748 |
| walker |  | 1877 | 13 | Code::CodeKey { rung: Names, file: bookmarks/management/commands/ensure_superuser.py, decl: 0, sub: 0, line: 0 } |  |  | 0.748 |
| ns | 1885 |  | 175 | models.py symbol roster (all 17 top-level classes and functions) | 2.1 |  | 0.716 |
| walker |  | 1890 | 13 | Code::CodeKey { rung: Names, file: bookmarks/management/commands/full_backup.py, decl: 0, sub: 0, line: 0 } |  |  | 0.716 |
| walker |  | 1903 | 13 | Code::CodeKey { rung: Names, file: bookmarks/management/commands/import_netscape.py, decl: 0, sub: 0, line: 0 } |  |  | 0.716 |
| walker |  | 1916 | 13 | Code::CodeKey { rung: Names, file: bookmarks/management/commands/migrate_tasks.py, decl: 0, sub: 0, line: 0 } |  |  | 0.716 |
| walker |  | 2008 | 92 | Fs::DirListing { dir: bookmarks/static } |  |  | 0.716 |
| walker |  | 2013 | 5 | Fs::DirListing { dir: bookmarks/static/vendor } |  |  | 0.716 |
| walker |  | 2110 | 97 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.716 |
| walker |  | 2145 | 35 | Code::CodeKey { rung: Names, file: bookmarks/templatetags/bookmarks.py, decl: 0, sub: 0, line: 0 } |  |  | 0.716 |
| walker |  | 2178 | 33 | Code::CodeKey { rung: Decl, file: bookmarks/templatetags/bookmarks.py, decl: 2, sub: 0, line: 9 } |  |  | 0.716 |
| walker |  | 2214 | 36 | Code::CodeKey { rung: Names, file: bookmarks/views/auth.py, decl: 0, sub: 0, line: 0 } |  |  | 0.716 |
| walker |  | 2242 | 28 | Code::CodeKey { rung: Decl, file: bookmarks/views/auth.py, decl: 5, sub: 0, line: 36 } |  |  | 0.716 |
| walker |  | 2280 | 38 | Code::CodeKey { rung: Names, file: bookmarks/settings/prod.py, decl: 0, sub: 0, line: 0 } |  |  | 0.716 |
| ns | 2296 |  | 411 | Bookmark model fields (complete) | 2.2 | 2.1 | 0.671 |
| walker |  | 2315 | 35 | Fs::DirListing { dir: docs } |  |  | 0.678 |
| walker |  | 2327 | 12 | Code::CodeKey { rung: Body, file: bookmarks/context_processors.py, decl: 2, sub: 0, line: 20 } |  |  | 0.678 |
| walker |  | 2387 | 60 | Fs::DirListing { dir: bookmarks/frontend/components } |  |  | 0.679 |
| walker |  | 2430 | 43 | Code::CodeKey { rung: Names, file: bookmarks/views/custom_css.py, decl: 0, sub: 0, line: 0 } |  |  | 0.679 |
| ns | 2439 |  | 143 | UserProfile field roster (names + types) | 2.3 | 2.1 | 0.654 |
| walker |  | 2447 | 17 | Fs::DirListing { dir: docs/src } |  |  | 0.654 |
| walker |  | 2454 | 7 | Fs::DirListing { dir: docs/src/content } |  |  | 0.654 |
| walker |  | 2499 | 45 | Code::CodeKey { rung: Names, file: bookmarks/services/parser.py, decl: 0, sub: 0, line: 0 } |  |  | 0.654 |
| walker |  | 2544 | 45 | Code::CodeKey { rung: Names, file: bookmarks/views/assets.py, decl: 0, sub: 0, line: 0 } |  |  | 0.654 |
| walker |  | 2590 | 46 | Code::CodeKey { rung: Decl, file: bookmarks/views/auth.py, decl: 1, sub: 0, line: 7 } |  |  | 0.654 |
| walker |  | 2659 | 69 | Fs::DirListing { dir: bookmarks/templates/bookmarks } |  |  | 0.654 |
| walker |  | 2676 | 17 | Fs::DirListing { dir: bookmarks/templates/bookmarks/details } |  |  | 0.654 |
| walker |  | 2770 | 94 | Code::CodeKey { rung: Names, file: bookmarks/widgets.py, decl: 0, sub: 0, line: 0 } |  |  | 0.654 |
| walker |  | 2782 | 12 | Code::CodeKey { rung: Decl, file: bookmarks/widgets.py, decl: 1, sub: 0, line: 7 } |  |  | 0.654 |
| walker |  | 2796 | 14 | Code::CodeKey { rung: Decl, file: bookmarks/widgets.py, decl: 2, sub: 0, line: 11 } |  |  | 0.654 |
| walker |  | 2810 | 14 | Code::CodeKey { rung: Decl, file: bookmarks/widgets.py, decl: 4, sub: 0, line: 19 } |  |  | 0.654 |
| ns | 2823 |  | 384 | UserProfile feature toggles (complete tail of the model) | 2.4 | 2.3 | 0.624 |
| walker |  | 2824 | 14 | Code::CodeKey { rung: Decl, file: bookmarks/widgets.py, decl: 6, sub: 0, line: 27 } |  |  | 0.624 |
| walker |  | 2838 | 14 | Code::CodeKey { rung: Decl, file: bookmarks/widgets.py, decl: 8, sub: 0, line: 35 } |  |  | 0.624 |
| walker |  | 2873 | 35 | Code::CodeKey { rung: Decl, file: bookmarks/widgets.py, decl: 10, sub: 0, line: 43 } |  |  | 0.624 |
| walker |  | 2908 | 35 | Code::CodeKey { rung: Decl, file: bookmarks/widgets.py, decl: 13, sub: 0, line: 63 } |  |  | 0.624 |
| walker |  | 3003 | 95 | Code::CodeKey { rung: Names, file: bookmarks/feeds.py, decl: 0, sub: 0, line: 0 } |  |  | 0.624 |
| ns | 3026 |  | 203 | BookmarkSearch — the search/filter parameter vocabulary | 2.5 | 2.1 | 0.598 |
| walker |  | 3045 | 42 | Code::CodeKey { rung: Decl, file: bookmarks/feeds.py, decl: 1, sub: 0, line: 14 } |  |  | 0.598 |
| walker |  | 3110 | 65 | Code::CodeKey { rung: Decl, file: bookmarks/feeds.py, decl: 12, sub: 0, line: 73 } |  |  | 0.598 |
| walker |  | 3176 | 66 | Code::CodeKey { rung: Decl, file: bookmarks/feeds.py, decl: 15, sub: 0, line: 84 } |  |  | 0.598 |
| walker |  | 3242 | 66 | Code::CodeKey { rung: Decl, file: bookmarks/feeds.py, decl: 18, sub: 0, line: 97 } |  |  | 0.598 |
| ns | 3271 |  | 245 | Bookmark methods: resolved_title, tag_names, save, query_existing | 2.6 | 2.2 | 0.577 |
| walker |  | 3324 | 82 | Code::CodeKey { rung: Decl, file: bookmarks/feeds.py, decl: 21, sub: 0, line: 110 } |  |  | 0.577 |
| walker |  | 3377 | 53 | Code::CodeKey { rung: Names, file: bookmarks/services/singlefile.py, decl: 0, sub: 0, line: 0 } |  |  | 0.577 |
| walker |  | 3382 | 5 | Code::CodeKey { rung: Decl, file: bookmarks/services/singlefile.py, decl: 1, sub: 0, line: 10 } |  |  | 0.577 |
| walker |  | 3433 | 51 | Code::CodeKey { rung: Decl, file: bookmarks/api/auth.py, decl: 1, sub: 0, line: 8 } |  |  | 0.577 |
| ns | 3465 |  | 194 | Tag model + tag-string parsing rules | 2.7 | 2.1 | 0.562 |
| walker |  | 3631 | 198 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.601 |
| ns | 3741 |  | 276 | BookmarkAsset — snapshot/upload model | 2.8 | 2.1 | 0.583 |
| walker |  | 3758 | 127 | Plaintext::Whole { file: docker-compose.yml } |  |  | 0.583 |
| walker |  | 3808 | 50 | Fs::DirListing { dir: scripts } |  |  | 0.584 |
| walker |  | 3864 | 56 | Code::CodeKey { rung: Names, file: bookmarks/services/bundles.py, decl: 0, sub: 0, line: 0 } |  |  | 0.584 |
| ns | 3941 |  | 200 | BookmarkBundle — saved-filter model | 2.9 | 2.1 | 0.572 |
| walker |  | 3975 | 111 | Code::CodeKey { rung: Names, file: bookmarks/forms.py, decl: 0, sub: 0, line: 0 } |  |  | 0.573 |
| walker |  | 4057 | 82 | Code::CodeKey { rung: Decl, file: bookmarks/forms.py, decl: 12, sub: 0, line: 161 } |  |  | 0.573 |
| walker |  | 4154 | 97 | Code::CodeKey { rung: Decl, file: bookmarks/forms.py, decl: 8, sub: 0, line: 124 } |  |  | 0.573 |
| walker |  | 4189 | 35 | Code::CodeKey { rung: Names, file: bookmarks/management/commands/create_initial_superuser.py, decl: 0, sub: 0, line: 0 } |  |  | 0.573 |
| walker |  | 4224 | 35 | Code::CodeKey { rung: Names, file: bookmarks/management/commands/enable_wal.py, decl: 0, sub: 0, line: 0 } |  |  | 0.573 |
| ns | 4241 |  | 300 | Toast, FeedToken, ApiToken and GlobalSettings fields | 2.10 | 2.1 | 0.554 |
| walker |  | 4259 | 35 | Code::CodeKey { rung: Names, file: bookmarks/management/commands/generate_secret_key.py, decl: 0, sub: 0, line: 0 } |  |  | 0.554 |
| walker |  | 4346 | 87 | Fs::DirListing { dir: bookmarks/styles/theme } |  |  | 0.554 |
| ns | 4438 |  | 197 | Model signal side effects (profile creation, file cleanup) | 2.11 | 2.1 | 0.540 |
| walker |  | 4461 | 115 | Code::CodeKey { rung: Decl, file: bookmarks/forms.py, decl: 21, sub: 0, line: 371 } |  |  | 0.540 |
| walker |  | 4471 | 10 | Code::CodeKey { rung: Body, file: bookmarks/middlewares.py, decl: 5, sub: 0, line: 18 } |  |  | 0.540 |
| walker |  | 4534 | 63 | Code::CodeKey { rung: Names, file: bookmarks/views/tags.py, decl: 0, sub: 0, line: 0 } |  |  | 0.540 |
| walker |  | 4542 | 8 | Code::CodeKey { rung: Decl, file: bookmarks/views/tags.py, decl: 1, sub: 0, line: 17 } |  |  | 0.540 |
| walker |  | 4550 | 8 | Code::CodeKey { rung: Decl, file: bookmarks/views/tags.py, decl: 2, sub: 0, line: 64 } |  |  | 0.540 |
| walker |  | 4558 | 8 | Code::CodeKey { rung: Decl, file: bookmarks/views/tags.py, decl: 3, sub: 0, line: 88 } |  |  | 0.540 |
| walker |  | 4566 | 8 | Code::CodeKey { rung: Decl, file: bookmarks/views/tags.py, decl: 4, sub: 0, line: 112 } |  |  | 0.540 |
| walker |  | 4630 | 64 | Code::CodeKey { rung: Names, file: bookmarks/services/tags.py, decl: 0, sub: 0, line: 0 } |  |  | 0.540 |
| walker |  | 4668 | 38 | Code::CodeKey { rung: Decl, file: bookmarks/management/commands/enable_wal.py, decl: 2, sub: 0, line: 10 } |  |  | 0.540 |
| ns | 4694 |  | 256 | urls.py — root and bookmark page routes | 3.1 |  | 0.531 |
| walker |  | 4706 | 38 | Code::CodeKey { rung: Decl, file: bookmarks/management/commands/generate_secret_key.py, decl: 2, sub: 0, line: 10 } |  |  | 0.531 |
| walker |  | 4870 | 164 | Plaintext::Whole { file: .env.sample } |  |  | 0.531 |
| walker |  | 4910 | 40 | Code::CodeKey { rung: Decl, file: bookmarks/management/commands/create_initial_superuser.py, decl: 2, sub: 0, line: 10 } |  |  | 0.531 |
| walker |  | 4950 | 40 | Code::CodeKey { rung: Decl, file: bookmarks/management/commands/migrate_tasks.py, decl: 1, sub: 0, line: 9 } |  |  | 0.531 |
| ns | 4970 |  | 276 | urls.py — asset, bundle and tag routes | 3.2 | 3.1 | 0.520 |
| walker |  | 5023 | 73 | Code::CodeKey { rung: Names, file: bookmarks/settings/dev.py, decl: 0, sub: 0, line: 0 } |  |  | 0.520 |
| walker |  | 5042 | 19 | Code::CodeKey { rung: Decl, file: bookmarks/settings/dev.py, decl: 2, sub: 0, line: 18 } |  |  | 0.520 |
| walker |  | 5182 | 140 | Code::CodeKey { rung: Decl, file: bookmarks/feeds.py, decl: 3, sub: 0, line: 31 } |  |  | 0.520 |
| ns | 5188 |  | 218 | urls.py — settings and toast routes | 3.3 | 3.2 | 0.512 |
| walker |  | 5330 | 148 | Code::CodeKey { rung: Names, file: bookmarks/utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.512 |
| walker |  | 5354 | 24 | Code::CodeKey { rung: Decl, file: bookmarks/utils.py, decl: 5, sub: 0, line: 63 } |  |  | 0.512 |
| walker |  | 5378 | 24 | Code::CodeKey { rung: Decl, file: bookmarks/utils.py, decl: 6, sub: 0, line: 83 } |  |  | 0.512 |
| walker |  | 5411 | 33 | Code::CodeKey { rung: Decl, file: bookmarks/utils.py, decl: 3, sub: 0, line: 36 } |  |  | 0.512 |
| walker |  | 5491 | 80 | Code::CodeKey { rung: Decl, file: bookmarks/utils.py, decl: 2, sub: 0, line: 25 } |  |  | 0.512 |
| walker |  | 5515 | 24 | Code::CodeKey { rung: Decl, file: bookmarks/utils.py, decl: 4, sub: 0, line: 43 } |  |  | 0.512 |
| ns | 5519 |  | 331 | urls.py — API mounts, feeds and utility endpoints | 3.4 | 3.3 | 0.502 |
| walker |  | 5710 | 195 | Code::CodeKey { rung: Names, file: bookmarks/admin.py, decl: 0, sub: 0, line: 0 } |  |  | 0.502 |
| walker |  | 5739 | 29 | Code::CodeKey { rung: Decl, file: bookmarks/admin.py, decl: 29, sub: 0, line: 318 } |  |  | 0.502 |
| walker |  | 5774 | 35 | Code::CodeKey { rung: Decl, file: bookmarks/admin.py, decl: 25, sub: 0, line: 297 } |  |  | 0.502 |
| ns | 5795 |  | 276 | urls.py — conditional routes: live reload, auth, admin, OIDC, context path | 3.5 | 3.4 | 0.491 |
| walker |  | 5811 | 37 | Code::CodeKey { rung: Decl, file: bookmarks/admin.py, decl: 28, sub: 0, line: 312 } |  |  | 0.491 |
| walker |  | 5859 | 48 | Code::CodeKey { rung: Decl, file: bookmarks/admin.py, decl: 27, sub: 0, line: 306 } |  |  | 0.491 |
| walker |  | 5911 | 52 | Code::CodeKey { rung: Decl, file: bookmarks/admin.py, decl: 1, sub: 0, line: 29 } |  |  | 0.491 |
| walker |  | 5919 | 8 | Code::CodeKey { rung: Decl, file: bookmarks/admin.py, decl: 3, sub: 0, line: 34 } |  |  | 0.491 |
| walker |  | 5973 | 54 | Code::CodeKey { rung: Decl, file: bookmarks/admin.py, decl: 24, sub: 0, line: 289 } |  |  | 0.491 |
| walker |  | 6033 | 60 | Code::CodeKey { rung: Decl, file: bookmarks/admin.py, decl: 7, sub: 0, line: 77 } |  |  | 0.491 |
| ns | 6056 |  | 261 | views/bookmarks.py function roster (complete) | 3.6 |  | 0.477 |
| walker |  | 6102 | 69 | Code::CodeKey { rung: Decl, file: bookmarks/admin.py, decl: 30, sub: 0, line: 324 } |  |  | 0.477 |
| walker |  | 6181 | 79 | Code::CodeKey { rung: Decl, file: bookmarks/admin.py, decl: 17, sub: 0, line: 215 } |  |  | 0.477 |
| walker |  | 6192 | 11 | Code::CodeKey { rung: Decl, file: bookmarks/admin.py, decl: 18, sub: 0, line: 216 } |  |  | 0.477 |
| ns | 6328 |  | 272 | views/contexts.py class roster (complete) | 3.7 |  | 0.466 |
| walker |  | 6332 | 140 | Code::CodeKey { rung: Decl, file: bookmarks/admin.py, decl: 23, sub: 0, line: 272 } |  |  | 0.466 |
| walker |  | 6482 | 150 | Code::CodeKey { rung: Decl, file: bookmarks/admin.py, decl: 19, sub: 0, line: 228 } |  |  | 0.466 |
| ns | 6570 |  | 242 | views/settings.py + views/tags.py + views/bundles.py function rosters (complete) | 3.8 |  | 0.456 |
| ns | 6741 |  | 171 | views/access.py — the complete authorization helper set | 3.9 |  | 0.453 |
| walker |  | 6748 | 266 | Code::CodeKey { rung: Names, file: bookmarks/models.py, decl: 0, sub: 0, line: 0 } |  |  | 0.481 |
| walker |  | 6761 | 13 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 14, sub: 0, line: 116 } |  |  | 0.481 |
| walker |  | 6774 | 13 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 34, sub: 0, line: 472 } |  |  | 0.482 |
| walker |  | 6787 | 13 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 35, sub: 0, line: 478 } |  |  | 0.483 |
| walker |  | 6801 | 14 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 19, sub: 0, line: 172 } |  |  | 0.484 |
| walker |  | 6855 | 54 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 36, sub: 0, line: 483 } |  |  | 0.486 |
| walker |  | 6916 | 61 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 2, sub: 0, line: 21 } |  |  | 0.487 |
| walker |  | 6923 | 7 | Code::CodeKey { rung: Body, file: bookmarks/models.py, decl: 35, sub: 0, line: 478 } |  |  | 0.488 |
| ns | 7007 |  | 266 | Remaining view modules: turbo, assets, auth, root, health, manifest, opensearch, custom_css, toasts, reload | 3.10 |  | 0.487 |
| walker |  | 7052 | 129 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 37, sub: 0, line: 490 } |  |  | 0.489 |
| walker |  | 7060 | 8 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 39, sub: 0, line: 508 } |  |  | 0.489 |
| ns | 7095 |  | 88 | forms.py class roster (complete) | 3.11 |  | 0.496 |
| walker |  | 7202 | 142 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 41, sub: 0, line: 516 } |  |  | 0.500 |
| walker |  | 7210 | 8 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 43, sub: 0, line: 531 } |  |  | 0.500 |
| walker |  | 7259 | 49 | Code::CodeKey { rung: Decl, file: bookmarks/management/commands/import_netscape.py, decl: 1, sub: 0, line: 7 } |  |  | 0.500 |
| ns | 7309 |  | 214 | Request-scoped plumbing: typed HttpRequest + LinkdingMiddleware | 3.12 |  | 0.495 |
| ns | 7329 |  | 20 | bookmarks/templatetags/ listing (complete) | 3.13 |  | 0.498 |
| walker |  | 7508 | 249 | Code::CodeKey { rung: Names, file: bookmarks/queries.py, decl: 0, sub: 0, line: 0 } |  |  | 0.499 |
| walker |  | 7536 | 28 | Code::CodeKey { rung: Decl, file: bookmarks/queries.py, decl: 2, sub: 0, line: 41 } |  |  | 0.499 |
| ns | 7563 |  | 234 | api/routes.py — viewsets and router registrations (complete) | 4.1 |  | 0.488 |
| walker |  | 7564 | 28 | Code::CodeKey { rung: Decl, file: bookmarks/queries.py, decl: 9, sub: 0, line: 308 } |  |  | 0.488 |
| walker |  | 7592 | 28 | Code::CodeKey { rung: Decl, file: bookmarks/queries.py, decl: 10, sub: 0, line: 318 } |  |  | 0.488 |
| walker |  | 7621 | 29 | Code::CodeKey { rung: Decl, file: bookmarks/queries.py, decl: 12, sub: 0, line: 341 } |  |  | 0.488 |
| walker |  | 7655 | 34 | Code::CodeKey { rung: Decl, file: bookmarks/queries.py, decl: 15, sub: 0, line: 368 } |  |  | 0.488 |
| walker |  | 7693 | 38 | Code::CodeKey { rung: Decl, file: bookmarks/queries.py, decl: 1, sub: 0, line: 33 } |  |  | 0.488 |
| walker |  | 7743 | 50 | Code::CodeKey { rung: Decl, file: bookmarks/queries.py, decl: 3, sub: 0, line: 47 } |  |  | 0.488 |
| ns | 7779 |  | 216 | BookmarkViewSet — every custom action and override | 4.2 | 4.1 | 0.482 |
| walker |  | 7793 | 50 | Code::CodeKey { rung: Decl, file: bookmarks/queries.py, decl: 11, sub: 0, line: 328 } |  |  | 0.482 |
| walker |  | 7823 | 30 | Code::CodeKey { rung: Decl, file: bookmarks/queries.py, decl: 5, sub: 0, line: 122 } |  |  | 0.482 |
| walker |  | 7853 | 30 | Code::CodeKey { rung: Decl, file: bookmarks/queries.py, decl: 6, sub: 0, line: 139 } |  |  | 0.482 |
| walker |  | 7893 | 40 | Code::CodeKey { rung: Decl, file: bookmarks/queries.py, decl: 8, sub: 0, line: 227 } |  |  | 0.482 |
| ns | 7928 |  | 149 | Asset, user and bundle viewset actions | 4.3 | 4.1 | 0.479 |
| walker |  | 7979 | 86 | Code::CodeKey { rung: Decl, file: bookmarks/settings/dev.py, decl: 4, sub: 0, line: 25 } |  |  | 0.479 |
| ns | 8017 |  | 89 | api/serializers.py class roster (complete) | 4.4 |  | 0.475 |
| walker |  | 8030 | 51 | Code::CodeKey { rung: Decl, file: bookmarks/management/commands/backup.py, decl: 1, sub: 0, line: 7 } |  |  | 0.475 |
| ns | 8136 |  | 119 | API token authentication + docs/api.md section map | 4.5 |  | 0.474 |
| walker |  | 8265 | 235 | Code::CodeKey { rung: Names, file: bookmarks/services/tasks.py, decl: 0, sub: 0, line: 0 } |  |  | 0.474 |
| walker |  | 8272 | 7 | Code::CodeKey { rung: Decl, file: bookmarks/services/tasks.py, decl: 6, sub: 0, line: 76 } |  |  | 0.474 |
| walker |  | 8279 | 7 | Code::CodeKey { rung: Decl, file: bookmarks/services/tasks.py, decl: 7, sub: 0, line: 102 } |  |  | 0.474 |
| walker |  | 8286 | 7 | Code::CodeKey { rung: Decl, file: bookmarks/services/tasks.py, decl: 8, sub: 0, line: 109 } |  |  | 0.474 |
| walker |  | 8293 | 7 | Code::CodeKey { rung: Decl, file: bookmarks/services/tasks.py, decl: 12, sub: 0, line: 133 } |  |  | 0.474 |
| ns | 8365 |  | 229 | services/bookmarks.py function roster (complete) | 5.1 |  | 0.467 |
| ns | 8566 |  | 201 | queries.py function roster (complete) | 5.2 |  | 0.478 |
| ns | 8816 |  | 250 | services/search_query_parser.py symbol roster (complete) | 5.3 |  | 0.471 |
| walker |  | 8907 | 614 | Fs::DirListing { dir: bookmarks/migrations } |  |  | 0.471 |
| walker |  | 8922 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0001_initial.py, decl: 0, sub: 0, line: 0 } |  |  | 0.471 |
| walker |  | 8937 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0002_auto_20190629_2303.py, decl: 0, sub: 0, line: 0 } |  |  | 0.471 |
| walker |  | 8952 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0003_auto_20200913_0656.py, decl: 0, sub: 0, line: 0 } |  |  | 0.471 |
| walker |  | 8967 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0004_auto_20200926_1028.py, decl: 0, sub: 0, line: 0 } |  |  | 0.471 |
| walker |  | 8982 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0005_auto_20210103_1212.py, decl: 0, sub: 0, line: 0 } |  |  | 0.471 |
| walker |  | 8997 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0006_bookmark_is_archived.py, decl: 0, sub: 0, line: 0 } |  |  | 0.471 |
| walker |  | 9012 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0008_userprofile_bookmark_date_display.py, decl: 0, sub: 0, line: 0 } |  |  | 0.471 |
| walker |  | 9027 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0009_bookmark_web_archive_snapshot_url.py, decl: 0, sub: 0, line: 0 } |  |  | 0.471 |
| walker |  | 9042 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0010_userprofile_bookmark_link_target.py, decl: 0, sub: 0, line: 0 } |  |  | 0.471 |
| walker |  | 9057 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0011_userprofile_web_archive_integration.py, decl: 0, sub: 0, line: 0 } |  |  | 0.471 |
| walker |  | 9072 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0012_toast.py, decl: 0, sub: 0, line: 0 } |  |  | 0.471 |
| walker |  | 9087 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0015_feedtoken.py, decl: 0, sub: 0, line: 0 } |  |  | 0.471 |
| walker |  | 9102 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0016_bookmark_shared.py, decl: 0, sub: 0, line: 0 } |  |  | 0.471 |
| walker |  | 9117 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0017_userprofile_enable_sharing.py, decl: 0, sub: 0, line: 0 } |  |  | 0.471 |
| walker |  | 9132 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0018_bookmark_favicon_file.py, decl: 0, sub: 0, line: 0 } |  |  | 0.471 |
| walker |  | 9147 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0019_userprofile_enable_favicons.py, decl: 0, sub: 0, line: 0 } |  |  | 0.471 |
| walker |  | 9162 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0020_userprofile_tag_search.py, decl: 0, sub: 0, line: 0 } |  |  | 0.471 |
| ns | 9169 |  | 353 | services/tasks.py function roster (complete) | 5.4 |  | 0.466 |
| walker |  | 9177 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0021_userprofile_display_url.py, decl: 0, sub: 0, line: 0 } |  |  | 0.466 |
| walker |  | 9192 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0022_bookmark_notes.py, decl: 0, sub: 0, line: 0 } |  |  | 0.466 |
| walker |  | 9207 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0023_userprofile_permanent_notes.py, decl: 0, sub: 0, line: 0 } |  |  | 0.466 |
| walker |  | 9222 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0024_userprofile_enable_public_sharing.py, decl: 0, sub: 0, line: 0 } |  |  | 0.466 |
| walker |  | 9237 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0025_userprofile_search_preferences.py, decl: 0, sub: 0, line: 0 } |  |  | 0.466 |
| walker |  | 9252 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0026_userprofile_custom_css.py, decl: 0, sub: 0, line: 0 } |  |  | 0.466 |
| walker |  | 9267 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0027_userprofile_bookmark_description_display_and_more.py, decl: 0, sub: 0, line: 0 } |  |  | 0.466 |
| walker |  | 9282 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0028_userprofile_display_archive_bookmark_action_and_more.py, decl: 0, sub: 0, line: 0 } |  |  | 0.466 |
| walker |  | 9297 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0030_bookmarkasset.py, decl: 0, sub: 0, line: 0 } |  |  | 0.466 |
| walker |  | 9312 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0031_userprofile_enable_automatic_html_snapshots.py, decl: 0, sub: 0, line: 0 } |  |  | 0.466 |
| walker |  | 9327 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0033_userprofile_default_mark_unread.py, decl: 0, sub: 0, line: 0 } |  |  | 0.466 |
| walker |  | 9342 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0034_bookmark_preview_image_file_and_more.py, decl: 0, sub: 0, line: 0 } |  |  | 0.466 |
| walker |  | 9357 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0035_userprofile_tag_grouping.py, decl: 0, sub: 0, line: 0 } |  |  | 0.466 |
| walker |  | 9372 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0036_userprofile_auto_tagging_rules.py, decl: 0, sub: 0, line: 0 } |  |  | 0.466 |
| walker |  | 9387 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0037_globalsettings.py, decl: 0, sub: 0, line: 0 } |  |  | 0.466 |
| walker |  | 9402 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0038_globalsettings_guest_profile_user.py, decl: 0, sub: 0, line: 0 } |  |  | 0.466 |
| walker |  | 9417 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0039_globalsettings_enable_link_prefetch.py, decl: 0, sub: 0, line: 0 } |  |  | 0.466 |
| walker |  | 9432 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0040_userprofile_items_per_page_and_more.py, decl: 0, sub: 0, line: 0 } |  |  | 0.466 |
| ns | 9438 |  | 269 | settings/base.py — deployment, auth and database LD_* options | 6.1 |  | 0.461 |
| walker |  | 9447 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0042_userprofile_custom_css_hash.py, decl: 0, sub: 0, line: 0 } |  |  | 0.461 |
| walker |  | 9462 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0043_userprofile_collapse_side_panel.py, decl: 0, sub: 0, line: 0 } |  |  | 0.461 |
| walker |  | 9477 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0045_userprofile_hide_bundles_bookmarkbundle.py, decl: 0, sub: 0, line: 0 } |  |  | 0.461 |
| walker |  | 9492 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0046_add_url_normalized_field.py, decl: 0, sub: 0, line: 0 } |  |  | 0.461 |
| walker |  | 9507 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0048_userprofile_default_mark_shared.py, decl: 0, sub: 0, line: 0 } |  |  | 0.461 |
| walker |  | 9522 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0049_userprofile_legacy_search.py, decl: 0, sub: 0, line: 0 } |  |  | 0.461 |
| walker |  | 9537 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0052_apitoken.py, decl: 0, sub: 0, line: 0 } |  |  | 0.461 |
| walker |  | 9552 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0054_bookmarkbundle_filter_shared_and_more.py, decl: 0, sub: 0, line: 0 } |  |  | 0.461 |
| walker |  | 9595 | 43 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0007_userprofile.py, decl: 0, sub: 0, line: 0 } |  |  | 0.461 |
| walker |  | 9600 | 5 | Code::CodeKey { rung: Body, file: bookmarks/migrations/0007_userprofile.py, decl: 2, sub: 0, line: 20 } |  |  | 0.461 |
| walker |  | 9643 | 43 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0014_alter_bookmark_unread.py, decl: 0, sub: 0, line: 0 } |  |  | 0.461 |
| walker |  | 9648 | 5 | Code::CodeKey { rung: Body, file: bookmarks/migrations/0014_alter_bookmark_unread.py, decl: 2, sub: 0, line: 11 } |  |  | 0.461 |
| walker |  | 9691 | 43 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0041_merge_metadata.py, decl: 0, sub: 0, line: 0 } |  |  | 0.461 |
| walker |  | 9696 | 5 | Code::CodeKey { rung: Body, file: bookmarks/migrations/0041_merge_metadata.py, decl: 2, sub: 0, line: 24 } |  |  | 0.461 |
| walker |  | 9739 | 43 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0044_bookmark_latest_snapshot.py, decl: 0, sub: 0, line: 0 } |  |  | 0.461 |
| walker |  | 9744 | 5 | Code::CodeKey { rung: Body, file: bookmarks/migrations/0044_bookmark_latest_snapshot.py, decl: 2, sub: 0, line: 23 } |  |  | 0.461 |
| ns | 9790 |  | 352 | settings/base.py — favicon, preview, snapshot and singlefile LD_* options | 6.2 |  | 0.455 |
| walker |  | 9791 | 47 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0053_migrate_api_tokens.py, decl: 0, sub: 0, line: 0 } |  |  | 0.455 |
| walker |  | 9841 | 50 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0051_fix_normalized_url.py, decl: 0, sub: 0, line: 0 } |  |  | 0.455 |
| walker |  | 9846 | 5 | Code::CodeKey { rung: Body, file: bookmarks/migrations/0051_fix_normalized_url.py, decl: 2, sub: 0, line: 20 } |  |  | 0.455 |
| walker |  | 9897 | 51 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0047_populate_url_normalized_field.py, decl: 0, sub: 0, line: 0 } |  |  | 0.455 |
| ns | 9898 |  | 108 | Operational surfaces: docker/, scripts/ and management commands (complete listings) | 6.3 |  | 0.467 |
| walker |  | 9960 | 63 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0013_web_archive_optin_toast.py, decl: 0, sub: 0, line: 0 } |  |  | 0.467 |
| ns | 9985 |  | 87 | bookmarks/frontend/ component and utility listings (complete) | 7.1 |  | 0.478 |

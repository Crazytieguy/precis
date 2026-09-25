Score(3000)=0.668 I=0.837 C=0.533 ns_rows≤3K=18/51 grid(1000/1442/2080/3000/4327/6240/9000)=0.819/0.829/0.725/0.668/0.553/0.476/0.462

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 41 |  | 41 | What linkding is | 1.1 |  | 0.000 |
| ns | 112 |  | 71 | The one sentence that locates the code | 1.2 |  | 0.000 |
| walker |  | 115 | 115 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 127 | 12 | Fs::DirListing { dir: docker } |  |  | 0.000 |
| walker |  | 130 | 3 | Fs::DirListing { dir: .github } |  |  | 0.000 |
| walker |  | 143 | 13 | Fs::DirListing { dir: .github/workflows } |  |  | 0.000 |
| walker |  | 148 | 5 | Fs::DirListing { dir: .devcontainer } |  |  | 0.000 |
| walker |  | 192 | 44 | Fs::DirListing { dir: assets } |  |  | 0.000 |
| walker |  | 203 | 11 | Code::CodeKey { rung: Names, file: manage.py, decl: 0, sub: 0, line: 0 } |  |  | 0.000 |
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
| walker |  | 557 | 18 | Fs::DirListing { dir: bookmarks/frontend } |  |  | 0.898 |
| ns | 583 |  | 138 | README feature overview (tail) | 1.6 | 1.5 | 0.856 |
| walker |  | 584 | 27 | Fs::DirListing { dir: bookmarks/templates } |  |  | 0.856 |
| walker |  | 589 | 5 | Fs::DirListing { dir: bookmarks/templates/admin } |  |  | 0.856 |
| walker |  | 605 | 16 | Fs::DirListing { dir: bookmarks/templates/registration } |  |  | 0.856 |
| ns | 679 |  | 96 | bookmarks/views/ and bookmarks/api/ listings (complete) | 1.7 |  | 0.748 |
| walker |  | 681 | 76 | Fs::DirListing { dir: bookmarks/views } |  |  | 0.878 |
| ns | 762 |  | 83 | bookmarks/services/ listing (complete) | 1.8 |  | 0.800 |
| walker |  | 764 | 83 | Fs::DirListing { dir: bookmarks/services } |  |  | 0.891 |
| walker |  | 784 | 20 | Fs::DirListing { dir: bookmarks/templates/bundles } |  |  | 0.891 |
| walker |  | 804 | 20 | Fs::DirListing { dir: bookmarks/templates/tags } |  |  | 0.891 |
| walker |  | 831 | 27 | Fs::DirListing { dir: bookmarks/frontend/utils } |  |  | 0.891 |
| walker |  | 858 | 27 | Fs::DirListing { dir: bookmarks/templates/settings } |  |  | 0.891 |
| walker |  | 870 | 12 | Code::CodeKey { rung: Names, file: bookmarks/wsgi.py, decl: 0, sub: 0, line: 0 } |  |  | 0.891 |
| ns | 937 |  | 175 | Dev prerequisites + core make targets | 1.9 |  | 0.819 |
| ns | 1154 |  | 217 | Remaining make targets: lint, format, e2e, frontend | 1.10 | 1.9 | 0.749 |
| walker |  | 1231 | 361 | Plaintext::Whole { file: Makefile } |  |  | 0.881 |
| walker |  | 1300 | 69 | Fs::DirListing { dir: bookmarks/styles } |  |  | 0.881 |
| walker |  | 1315 | 15 | Code::CodeKey { rung: Names, file: bookmarks/apps.py, decl: 0, sub: 0, line: 0 } |  |  | 0.881 |
| walker |  | 1340 | 25 | Code::CodeKey { rung: Decl, file: bookmarks/apps.py, decl: 1, sub: 0, line: 4 } |  |  | 0.881 |
| walker |  | 1355 | 15 | Code::CodeKey { rung: Names, file: bookmarks/type_defs.py, decl: 0, sub: 0, line: 0 } |  |  | 0.881 |
| walker |  | 1387 | 32 | Code::CodeKey { rung: Decl, file: bookmarks/type_defs.py, decl: 1, sub: 0, line: 11 } |  |  | 0.882 |
| walker |  | 1428 | 41 | Fs::DirListing { dir: bookmarks/templates/shared } |  |  | 0.882 |
| ns | 1438 |  | 284 | pyproject.toml project metadata + runtime dependencies | 1.11 |  | 0.829 |
| walker |  | 1445 | 17 | Code::CodeKey { rung: Names, file: bookmarks/validators.py, decl: 0, sub: 0, line: 0 } |  |  | 0.829 |
| walker |  | 1460 | 15 | Code::CodeKey { rung: Decl, file: bookmarks/validators.py, decl: 1, sub: 0, line: 5 } |  |  | 0.830 |
| ns | 1460 |  | 22 | bookmarks/settings/ listing (complete) | 1.12 |  | 0.830 |
| walker |  | 1478 | 18 | Code::CodeKey { rung: Names, file: bookmarks/signals.py, decl: 0, sub: 0, line: 0 } |  |  | 0.830 |
| walker |  | 1487 | 9 | Code::CodeKey { rung: Decl, file: bookmarks/signals.py, decl: 1, sub: 0, line: 6 } |  |  | 0.830 |
| walker |  | 1533 | 46 | Fs::DirListing { dir: bookmarks/management/commands } |  |  | 0.831 |
| ns | 1602 |  | 142 | Settings resolution order + manage.py/pytest wiring | 1.13 | 1.12 | 0.800 |
| walker |  | 1625 | 92 | Fs::DirListing { dir: bookmarks/static } |  |  | 0.800 |
| walker |  | 1630 | 5 | Fs::DirListing { dir: bookmarks/static/vendor } |  |  | 0.800 |
| ns | 1710 |  | 108 | docs/ site and its content pages (complete) | 1.14 |  | 0.748 |
| walker |  | 1727 | 97 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.748 |
| walker |  | 1762 | 35 | Fs::DirListing { dir: docs } |  |  | 0.756 |
| walker |  | 1774 | 12 | Code::CodeKey { rung: Names, file: bookmarks/views/health.py, decl: 0, sub: 0, line: 0 } |  |  | 0.756 |
| walker |  | 1786 | 12 | Code::CodeKey { rung: Names, file: bookmarks/views/manifest.py, decl: 0, sub: 0, line: 0 } |  |  | 0.756 |
| walker |  | 1798 | 12 | Code::CodeKey { rung: Names, file: bookmarks/views/root.py, decl: 0, sub: 0, line: 0 } |  |  | 0.756 |
| walker |  | 1810 | 12 | Code::CodeKey { rung: Names, file: bookmarks/views/toasts.py, decl: 0, sub: 0, line: 0 } |  |  | 0.756 |
| walker |  | 1818 | 8 | Code::CodeKey { rung: Decl, file: bookmarks/views/toasts.py, decl: 1, sub: 0, line: 9 } |  |  | 0.756 |
| walker |  | 1842 | 24 | Code::CodeKey { rung: Names, file: bookmarks/context_processors.py, decl: 0, sub: 0, line: 0 } |  |  | 0.756 |
| walker |  | 1854 | 12 | Code::CodeKey { rung: Body, file: bookmarks/context_processors.py, decl: 2, sub: 0, line: 20 } |  |  | 0.756 |
| ns | 1885 |  | 175 | models.py symbol roster (all 17 top-level classes and functions) | 2.1 |  | 0.724 |
| walker |  | 1914 | 60 | Fs::DirListing { dir: bookmarks/frontend/components } |  |  | 0.725 |
| walker |  | 1931 | 17 | Fs::DirListing { dir: docs/src } |  |  | 0.725 |
| walker |  | 1938 | 7 | Fs::DirListing { dir: docs/src/content } |  |  | 0.725 |
| walker |  | 1952 | 14 | Code::CodeKey { rung: Names, file: bookmarks/views/opensearch.py, decl: 0, sub: 0, line: 0 } |  |  | 0.725 |
| walker |  | 2021 | 69 | Fs::DirListing { dir: bookmarks/templates/bookmarks } |  |  | 0.725 |
| walker |  | 2038 | 17 | Fs::DirListing { dir: bookmarks/templates/bookmarks/details } |  |  | 0.725 |
| walker |  | 2236 | 198 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.775 |
| walker |  | 2252 | 16 | Code::CodeKey { rung: Names, file: bookmarks/api/auth.py, decl: 0, sub: 0, line: 0 } |  |  | 0.775 |
| ns | 2296 |  | 411 | Bookmark model fields (complete) | 2.2 | 2.1 | 0.726 |
| walker |  | 2303 | 51 | Code::CodeKey { rung: Decl, file: bookmarks/api/auth.py, decl: 1, sub: 0, line: 8 } |  |  | 0.726 |
| walker |  | 2319 | 16 | Code::CodeKey { rung: Names, file: bookmarks/services/wayback.py, decl: 0, sub: 0, line: 0 } |  |  | 0.726 |
| walker |  | 2343 | 24 | Code::CodeKey { rung: Decl, file: bookmarks/services/wayback.py, decl: 1, sub: 0, line: 6 } |  |  | 0.726 |
| walker |  | 2374 | 31 | Code::CodeKey { rung: Names, file: bookmarks/urls.py, decl: 0, sub: 0, line: 0 } |  |  | 0.726 |
| ns | 2439 |  | 143 | UserProfile field roster (names + types) | 2.3 | 2.1 | 0.699 |
| walker |  | 2501 | 127 | Plaintext::Whole { file: docker-compose.yml } |  |  | 0.699 |
| walker |  | 2551 | 50 | Fs::DirListing { dir: scripts } |  |  | 0.700 |
| walker |  | 2638 | 87 | Fs::DirListing { dir: bookmarks/styles/theme } |  |  | 0.700 |
| walker |  | 2802 | 164 | Plaintext::Whole { file: .env.sample } |  |  | 0.700 |
| walker |  | 2815 | 13 | Code::CodeKey { rung: Names, file: bookmarks/management/commands/backup.py, decl: 0, sub: 0, line: 0 } |  |  | 0.700 |
| ns | 2823 |  | 384 | UserProfile feature toggles (complete tail of the model) | 2.4 | 2.3 | 0.668 |
| walker |  | 2828 | 13 | Code::CodeKey { rung: Names, file: bookmarks/management/commands/ensure_superuser.py, decl: 0, sub: 0, line: 0 } |  |  | 0.668 |
| walker |  | 2841 | 13 | Code::CodeKey { rung: Names, file: bookmarks/management/commands/full_backup.py, decl: 0, sub: 0, line: 0 } |  |  | 0.668 |
| walker |  | 2854 | 13 | Code::CodeKey { rung: Names, file: bookmarks/management/commands/import_netscape.py, decl: 0, sub: 0, line: 0 } |  |  | 0.668 |
| walker |  | 2867 | 13 | Code::CodeKey { rung: Names, file: bookmarks/management/commands/migrate_tasks.py, decl: 0, sub: 0, line: 0 } |  |  | 0.668 |
| walker |  | 2907 | 40 | Code::CodeKey { rung: Decl, file: bookmarks/management/commands/migrate_tasks.py, decl: 1, sub: 0, line: 9 } |  |  | 0.668 |
| walker |  | 2956 | 49 | Code::CodeKey { rung: Decl, file: bookmarks/management/commands/import_netscape.py, decl: 1, sub: 0, line: 7 } |  |  | 0.668 |
| walker |  | 3007 | 51 | Code::CodeKey { rung: Decl, file: bookmarks/management/commands/backup.py, decl: 1, sub: 0, line: 7 } |  |  | 0.668 |
| ns | 3026 |  | 203 | BookmarkSearch — the search/filter parameter vocabulary | 2.5 | 2.1 | 0.640 |
| ns | 3271 |  | 245 | Bookmark methods: resolved_title, tag_names, save, query_existing | 2.6 | 2.2 | 0.618 |
| ns | 3465 |  | 194 | Tag model + tag-string parsing rules | 2.7 | 2.1 | 0.602 |
| walker |  | 3621 | 614 | Fs::DirListing { dir: bookmarks/migrations } |  |  | 0.602 |
| walker |  | 3636 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0001_initial.py, decl: 0, sub: 0, line: 0 } |  |  | 0.602 |
| walker |  | 3651 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0002_auto_20190629_2303.py, decl: 0, sub: 0, line: 0 } |  |  | 0.602 |
| walker |  | 3666 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0003_auto_20200913_0656.py, decl: 0, sub: 0, line: 0 } |  |  | 0.602 |
| walker |  | 3681 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0004_auto_20200926_1028.py, decl: 0, sub: 0, line: 0 } |  |  | 0.602 |
| walker |  | 3696 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0005_auto_20210103_1212.py, decl: 0, sub: 0, line: 0 } |  |  | 0.602 |
| walker |  | 3711 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0006_bookmark_is_archived.py, decl: 0, sub: 0, line: 0 } |  |  | 0.602 |
| walker |  | 3726 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0008_userprofile_bookmark_date_display.py, decl: 0, sub: 0, line: 0 } |  |  | 0.602 |
| walker |  | 3741 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0009_bookmark_web_archive_snapshot_url.py, decl: 0, sub: 0, line: 0 } |  |  | 0.584 |
| ns | 3741 |  | 276 | BookmarkAsset — snapshot/upload model | 2.8 | 2.1 | 0.584 |
| walker |  | 3756 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0010_userprofile_bookmark_link_target.py, decl: 0, sub: 0, line: 0 } |  |  | 0.584 |
| walker |  | 3771 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0011_userprofile_web_archive_integration.py, decl: 0, sub: 0, line: 0 } |  |  | 0.584 |
| walker |  | 3786 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0012_toast.py, decl: 0, sub: 0, line: 0 } |  |  | 0.584 |
| walker |  | 3801 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0015_feedtoken.py, decl: 0, sub: 0, line: 0 } |  |  | 0.584 |
| walker |  | 3816 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0016_bookmark_shared.py, decl: 0, sub: 0, line: 0 } |  |  | 0.584 |
| walker |  | 3831 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0017_userprofile_enable_sharing.py, decl: 0, sub: 0, line: 0 } |  |  | 0.584 |
| walker |  | 3846 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0018_bookmark_favicon_file.py, decl: 0, sub: 0, line: 0 } |  |  | 0.584 |
| walker |  | 3861 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0019_userprofile_enable_favicons.py, decl: 0, sub: 0, line: 0 } |  |  | 0.584 |
| walker |  | 3876 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0020_userprofile_tag_search.py, decl: 0, sub: 0, line: 0 } |  |  | 0.584 |
| walker |  | 3891 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0021_userprofile_display_url.py, decl: 0, sub: 0, line: 0 } |  |  | 0.584 |
| walker |  | 3906 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0022_bookmark_notes.py, decl: 0, sub: 0, line: 0 } |  |  | 0.584 |
| walker |  | 3921 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0023_userprofile_permanent_notes.py, decl: 0, sub: 0, line: 0 } |  |  | 0.584 |
| walker |  | 3936 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0024_userprofile_enable_public_sharing.py, decl: 0, sub: 0, line: 0 } |  |  | 0.584 |
| ns | 3941 |  | 200 | BookmarkBundle — saved-filter model | 2.9 | 2.1 | 0.572 |
| walker |  | 3951 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0025_userprofile_search_preferences.py, decl: 0, sub: 0, line: 0 } |  |  | 0.572 |
| walker |  | 3966 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0026_userprofile_custom_css.py, decl: 0, sub: 0, line: 0 } |  |  | 0.572 |
| walker |  | 3981 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0027_userprofile_bookmark_description_display_and_more.py, decl: 0, sub: 0, line: 0 } |  |  | 0.572 |
| walker |  | 3996 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0028_userprofile_display_archive_bookmark_action_and_more.py, decl: 0, sub: 0, line: 0 } |  |  | 0.572 |
| walker |  | 4011 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0030_bookmarkasset.py, decl: 0, sub: 0, line: 0 } |  |  | 0.572 |
| walker |  | 4026 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0031_userprofile_enable_automatic_html_snapshots.py, decl: 0, sub: 0, line: 0 } |  |  | 0.572 |
| walker |  | 4041 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0033_userprofile_default_mark_unread.py, decl: 0, sub: 0, line: 0 } |  |  | 0.572 |
| walker |  | 4056 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0034_bookmark_preview_image_file_and_more.py, decl: 0, sub: 0, line: 0 } |  |  | 0.572 |
| walker |  | 4071 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0035_userprofile_tag_grouping.py, decl: 0, sub: 0, line: 0 } |  |  | 0.572 |
| walker |  | 4086 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0036_userprofile_auto_tagging_rules.py, decl: 0, sub: 0, line: 0 } |  |  | 0.572 |
| walker |  | 4101 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0037_globalsettings.py, decl: 0, sub: 0, line: 0 } |  |  | 0.572 |
| walker |  | 4116 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0038_globalsettings_guest_profile_user.py, decl: 0, sub: 0, line: 0 } |  |  | 0.572 |
| walker |  | 4131 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0039_globalsettings_enable_link_prefetch.py, decl: 0, sub: 0, line: 0 } |  |  | 0.572 |
| walker |  | 4146 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0040_userprofile_items_per_page_and_more.py, decl: 0, sub: 0, line: 0 } |  |  | 0.572 |
| walker |  | 4161 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0042_userprofile_custom_css_hash.py, decl: 0, sub: 0, line: 0 } |  |  | 0.572 |
| walker |  | 4176 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0043_userprofile_collapse_side_panel.py, decl: 0, sub: 0, line: 0 } |  |  | 0.572 |
| walker |  | 4191 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0045_userprofile_hide_bundles_bookmarkbundle.py, decl: 0, sub: 0, line: 0 } |  |  | 0.572 |
| walker |  | 4206 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0046_add_url_normalized_field.py, decl: 0, sub: 0, line: 0 } |  |  | 0.572 |
| walker |  | 4221 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0048_userprofile_default_mark_shared.py, decl: 0, sub: 0, line: 0 } |  |  | 0.572 |
| walker |  | 4236 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0049_userprofile_legacy_search.py, decl: 0, sub: 0, line: 0 } |  |  | 0.572 |
| ns | 4241 |  | 300 | Toast, FeedToken, ApiToken and GlobalSettings fields | 2.10 | 2.1 | 0.553 |
| walker |  | 4251 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0052_apitoken.py, decl: 0, sub: 0, line: 0 } |  |  | 0.553 |
| walker |  | 4266 | 15 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0054_bookmarkbundle_filter_shared_and_more.py, decl: 0, sub: 0, line: 0 } |  |  | 0.553 |
| walker |  | 4358 | 92 | Code::CodeKey { rung: Decl, file: bookmarks/migrations/0016_bookmark_shared.py, decl: 1, sub: 0, line: 6 } |  |  | 0.553 |
| walker |  | 4412 | 54 | Code::CodeKey { rung: Decl, file: bookmarks/management/commands/ensure_superuser.py, decl: 1, sub: 0, line: 5 } |  |  | 0.553 |
| ns | 4438 |  | 197 | Model signal side effects (profile creation, file cleanup) | 2.11 | 2.1 | 0.539 |
| walker |  | 4506 | 94 | Code::CodeKey { rung: Decl, file: bookmarks/migrations/0022_bookmark_notes.py, decl: 1, sub: 0, line: 6 } |  |  | 0.539 |
| walker |  | 4563 | 57 | Code::CodeKey { rung: Names, file: bookmarks/middlewares.py, decl: 0, sub: 0, line: 0 } |  |  | 0.539 |
| walker |  | 4576 | 13 | Code::CodeKey { rung: Decl, file: bookmarks/middlewares.py, decl: 1, sub: 0, line: 7 } |  |  | 0.539 |
| walker |  | 4605 | 29 | Code::CodeKey { rung: Decl, file: bookmarks/middlewares.py, decl: 4, sub: 0, line: 17 } |  |  | 0.539 |
| walker |  | 4615 | 10 | Code::CodeKey { rung: Body, file: bookmarks/middlewares.py, decl: 5, sub: 0, line: 18 } |  |  | 0.539 |
| ns | 4694 |  | 256 | urls.py — root and bookmark page routes | 3.1 |  | 0.530 |
| walker |  | 4711 | 96 | Code::CodeKey { rung: Decl, file: bookmarks/migrations/0017_userprofile_enable_sharing.py, decl: 1, sub: 0, line: 6 } |  |  | 0.530 |
| walker |  | 4807 | 96 | Code::CodeKey { rung: Decl, file: bookmarks/migrations/0021_userprofile_display_url.py, decl: 1, sub: 0, line: 6 } |  |  | 0.530 |
| walker |  | 4903 | 96 | Code::CodeKey { rung: Decl, file: bookmarks/migrations/0023_userprofile_permanent_notes.py, decl: 1, sub: 0, line: 6 } |  |  | 0.530 |
| ns | 4970 |  | 276 | urls.py — asset, bundle and tag routes | 3.2 | 3.1 | 0.519 |
| walker |  | 4999 | 96 | Code::CodeKey { rung: Decl, file: bookmarks/migrations/0026_userprofile_custom_css.py, decl: 1, sub: 0, line: 6 } |  |  | 0.519 |
| walker |  | 5096 | 97 | Code::CodeKey { rung: Decl, file: bookmarks/migrations/0049_userprofile_legacy_search.py, decl: 1, sub: 0, line: 6 } |  |  | 0.519 |
| ns | 5188 |  | 218 | urls.py — settings and toast routes | 3.3 | 3.2 | 0.511 |
| walker |  | 5194 | 98 | Code::CodeKey { rung: Decl, file: bookmarks/migrations/0025_userprofile_search_preferences.py, decl: 1, sub: 0, line: 6 } |  |  | 0.511 |
| walker |  | 5292 | 98 | Code::CodeKey { rung: Decl, file: bookmarks/migrations/0043_userprofile_collapse_side_panel.py, decl: 1, sub: 0, line: 6 } |  |  | 0.511 |
| walker |  | 5391 | 99 | Code::CodeKey { rung: Decl, file: bookmarks/migrations/0019_userprofile_enable_favicons.py, decl: 1, sub: 0, line: 6 } |  |  | 0.511 |
| walker |  | 5490 | 99 | Code::CodeKey { rung: Decl, file: bookmarks/migrations/0024_userprofile_enable_public_sharing.py, decl: 1, sub: 0, line: 6 } |  |  | 0.511 |
| ns | 5519 |  | 331 | urls.py — API mounts, feeds and utility endpoints | 3.4 | 3.3 | 0.501 |
| walker |  | 5589 | 99 | Code::CodeKey { rung: Decl, file: bookmarks/migrations/0031_userprofile_enable_automatic_html_snapshots.py, decl: 1, sub: 0, line: 6 } |  |  | 0.501 |
| walker |  | 5688 | 99 | Code::CodeKey { rung: Decl, file: bookmarks/migrations/0036_userprofile_auto_tagging_rules.py, decl: 1, sub: 0, line: 6 } |  |  | 0.501 |
| walker |  | 5787 | 99 | Code::CodeKey { rung: Decl, file: bookmarks/migrations/0039_globalsettings_enable_link_prefetch.py, decl: 1, sub: 0, line: 6 } |  |  | 0.501 |
| ns | 5795 |  | 276 | urls.py — conditional routes: live reload, auth, admin, OIDC, context path | 3.5 | 3.4 | 0.490 |
| walker |  | 5886 | 99 | Code::CodeKey { rung: Decl, file: bookmarks/migrations/0048_userprofile_default_mark_shared.py, decl: 1, sub: 0, line: 6 } |  |  | 0.490 |
| walker |  | 5917 | 31 | Code::CodeKey { rung: Names, file: bookmarks/services/monolith.py, decl: 0, sub: 0, line: 0 } |  |  | 0.490 |
| walker |  | 5924 | 7 | Code::CodeKey { rung: Decl, file: bookmarks/services/monolith.py, decl: 1, sub: 0, line: 9 } |  |  | 0.490 |
| walker |  | 6024 | 100 | Code::CodeKey { rung: Decl, file: bookmarks/migrations/0006_bookmark_is_archived.py, decl: 1, sub: 0, line: 6 } |  |  | 0.490 |
| ns | 6056 |  | 261 | views/bookmarks.py function roster (complete) | 3.6 |  | 0.476 |
| walker |  | 6124 | 100 | Code::CodeKey { rung: Decl, file: bookmarks/migrations/0033_userprofile_default_mark_unread.py, decl: 1, sub: 0, line: 6 } |  |  | 0.476 |
| walker |  | 6224 | 100 | Code::CodeKey { rung: Decl, file: bookmarks/migrations/0042_userprofile_custom_css_hash.py, decl: 1, sub: 0, line: 6 } |  |  | 0.476 |
| walker |  | 6325 | 101 | Code::CodeKey { rung: Decl, file: bookmarks/migrations/0003_auto_20200913_0656.py, decl: 1, sub: 0, line: 6 } |  |  | 0.476 |
| ns | 6328 |  | 272 | views/contexts.py class roster (complete) | 3.7 |  | 0.465 |
| walker |  | 6426 | 101 | Code::CodeKey { rung: Decl, file: bookmarks/migrations/0018_bookmark_favicon_file.py, decl: 1, sub: 0, line: 6 } |  |  | 0.465 |
| walker |  | 6531 | 105 | Code::CodeKey { rung: Decl, file: bookmarks/migrations/0009_bookmark_web_archive_snapshot_url.py, decl: 1, sub: 0, line: 6 } |  |  | 0.465 |
| walker |  | 6566 | 35 | Code::CodeKey { rung: Body, file: bookmarks/validators.py, decl: 2, sub: 0, line: 11 } |  |  | 0.465 |
| ns | 6570 |  | 242 | views/settings.py + views/tags.py + views/bundles.py function rosters (complete) | 3.8 |  | 0.454 |
| walker |  | 6677 | 111 | Code::CodeKey { rung: Decl, file: bookmarks/migrations/0046_add_url_normalized_field.py, decl: 1, sub: 0, line: 6 } |  |  | 0.454 |
| walker |  | 6712 | 35 | Code::CodeKey { rung: Names, file: bookmarks/templatetags/bookmarks.py, decl: 0, sub: 0, line: 0 } |  |  | 0.454 |
| ns | 6741 |  | 171 | views/access.py — the complete authorization helper set | 3.9 |  | 0.451 |
| walker |  | 6745 | 33 | Code::CodeKey { rung: Decl, file: bookmarks/templatetags/bookmarks.py, decl: 2, sub: 0, line: 9 } |  |  | 0.451 |
| walker |  | 6792 | 47 | Plaintext::DeclSurface { file: pytest.ini } |  |  | 0.454 |
| walker |  | 6828 | 36 | Code::CodeKey { rung: Body, file: bookmarks/apps.py, decl: 2, sub: 0, line: 7 } |  |  | 0.454 |
| walker |  | 6896 | 68 | Code::CodeKey { rung: Decl, file: bookmarks/management/commands/full_backup.py, decl: 1, sub: 0, line: 9 } |  |  | 0.454 |
| walker |  | 6932 | 36 | Code::CodeKey { rung: Names, file: bookmarks/views/auth.py, decl: 0, sub: 0, line: 0 } |  |  | 0.454 |
| walker |  | 6960 | 28 | Code::CodeKey { rung: Decl, file: bookmarks/views/auth.py, decl: 5, sub: 0, line: 36 } |  |  | 0.454 |
| walker |  | 7006 | 46 | Code::CodeKey { rung: Decl, file: bookmarks/views/auth.py, decl: 1, sub: 0, line: 7 } |  |  | 0.454 |
| ns | 7007 |  | 266 | Remaining view modules: turbo, assets, auth, root, health, manifest, opensearch, custom_css, toasts, reload | 3.10 |  | 0.450 |
| ns | 7095 |  | 88 | forms.py class roster (complete) | 3.11 |  | 0.446 |
| ns | 7309 |  | 214 | Request-scoped plumbing: typed HttpRequest + LinkdingMiddleware | 3.12 |  | 0.442 |
| ns | 7329 |  | 20 | bookmarks/templatetags/ listing (complete) | 3.13 |  | 0.446 |
| walker |  | 7366 | 360 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.472 |
| walker |  | 7428 | 62 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.472 |
| walker |  | 7466 | 38 | Code::CodeKey { rung: Names, file: bookmarks/settings/prod.py, decl: 0, sub: 0, line: 0 } |  |  | 0.472 |
| walker |  | 7539 | 73 | Fs::DirListing { dir: docs/src/content/docs } |  |  | 0.502 |
| ns | 7563 |  | 234 | api/routes.py — viewsets and router registrations (complete) | 4.1 |  | 0.492 |
| walker |  | 7667 | 128 | Code::CodeKey { rung: Decl, file: bookmarks/migrations/0005_auto_20210103_1212.py, decl: 1, sub: 0, line: 8 } |  |  | 0.492 |
| walker |  | 7710 | 43 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0007_userprofile.py, decl: 0, sub: 0, line: 0 } |  |  | 0.492 |
| walker |  | 7715 | 5 | Code::CodeKey { rung: Body, file: bookmarks/migrations/0007_userprofile.py, decl: 2, sub: 0, line: 20 } |  |  | 0.492 |
| walker |  | 7758 | 43 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0014_alter_bookmark_unread.py, decl: 0, sub: 0, line: 0 } |  |  | 0.492 |
| walker |  | 7763 | 5 | Code::CodeKey { rung: Body, file: bookmarks/migrations/0014_alter_bookmark_unread.py, decl: 2, sub: 0, line: 11 } |  |  | 0.492 |
| ns | 7779 |  | 216 | BookmarkViewSet — every custom action and override | 4.2 | 4.1 | 0.486 |
| walker |  | 7873 | 110 | Code::CodeKey { rung: Decl, file: bookmarks/migrations/0014_alter_bookmark_unread.py, decl: 3, sub: 0, line: 15 } |  |  | 0.486 |
| walker |  | 7916 | 43 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0041_merge_metadata.py, decl: 0, sub: 0, line: 0 } |  |  | 0.486 |
| walker |  | 7921 | 5 | Code::CodeKey { rung: Body, file: bookmarks/migrations/0041_merge_metadata.py, decl: 2, sub: 0, line: 24 } |  |  | 0.486 |
| ns | 7928 |  | 149 | Asset, user and bundle viewset actions | 4.3 | 4.1 | 0.482 |
| walker |  | 7985 | 64 | Code::CodeKey { rung: Decl, file: bookmarks/migrations/0041_merge_metadata.py, decl: 3, sub: 0, line: 28 } |  |  | 0.482 |
| ns | 8017 |  | 89 | api/serializers.py class roster (complete) | 4.4 |  | 0.479 |
| walker |  | 8028 | 43 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0044_bookmark_latest_snapshot.py, decl: 0, sub: 0, line: 0 } |  |  | 0.479 |
| walker |  | 8033 | 5 | Code::CodeKey { rung: Body, file: bookmarks/migrations/0044_bookmark_latest_snapshot.py, decl: 2, sub: 0, line: 23 } |  |  | 0.479 |
| walker |  | 8076 | 43 | Code::CodeKey { rung: Names, file: bookmarks/views/custom_css.py, decl: 0, sub: 0, line: 0 } |  |  | 0.480 |
| ns | 8136 |  | 119 | API token authentication + docs/api.md section map | 4.5 |  | 0.478 |
| walker |  | 8219 | 143 | Code::CodeKey { rung: Decl, file: bookmarks/migrations/0011_userprofile_web_archive_integration.py, decl: 1, sub: 0, line: 6 } |  |  | 0.478 |
| walker |  | 8362 | 143 | Code::CodeKey { rung: Decl, file: bookmarks/migrations/0020_userprofile_tag_search.py, decl: 1, sub: 0, line: 6 } |  |  | 0.478 |
| ns | 8365 |  | 229 | services/bookmarks.py function roster (complete) | 5.1 |  | 0.471 |
| walker |  | 8507 | 145 | Code::CodeKey { rung: Decl, file: bookmarks/migrations/0010_userprofile_bookmark_link_target.py, decl: 1, sub: 0, line: 6 } |  |  | 0.471 |
| walker |  | 8552 | 45 | Code::CodeKey { rung: Names, file: bookmarks/services/parser.py, decl: 0, sub: 0, line: 0 } |  |  | 0.471 |
| ns | 8566 |  | 201 | queries.py function roster (complete) | 5.2 |  | 0.465 |
| walker |  | 8653 | 101 | Code::CodeKey { rung: Decl, file: bookmarks/services/parser.py, decl: 1, sub: 0, line: 8 } |  |  | 0.465 |
| walker |  | 8698 | 45 | Code::CodeKey { rung: Names, file: bookmarks/views/assets.py, decl: 0, sub: 0, line: 0 } |  |  | 0.469 |
| ns | 8816 |  | 250 | services/search_query_parser.py symbol roster (complete) | 5.3 |  | 0.462 |
| walker |  | 8844 | 146 | Code::CodeKey { rung: Decl, file: bookmarks/migrations/0035_userprofile_tag_grouping.py, decl: 1, sub: 0, line: 6 } |  |  | 0.462 |
| walker |  | 8993 | 149 | Code::CodeKey { rung: Decl, file: bookmarks/migrations/0004_auto_20200926_1028.py, decl: 1, sub: 0, line: 6 } |  |  | 0.462 |
| walker |  | 9040 | 47 | Code::CodeKey { rung: Names, file: bookmarks/migrations/0053_migrate_api_tokens.py, decl: 0, sub: 0, line: 0 } |  |  | 0.462 |
| walker |  | 9123 | 83 | Code::CodeKey { rung: Decl, file: bookmarks/migrations/0053_migrate_api_tokens.py, decl: 3, sub: 0, line: 23 } |  |  | 0.462 |
| ns | 9169 |  | 353 | services/tasks.py function roster (complete) | 5.4 |  | 0.453 |
| walker |  | 9275 | 152 | Code::CodeKey { rung: Decl, file: bookmarks/migrations/0034_bookmark_preview_image_file_and_more.py, decl: 1, sub: 0, line: 6 } |  |  | 0.453 |
| walker |  | 9326 | 51 | Code::CodeKey { rung: Doc, file: bookmarks/validators.py, decl: 1, sub: 0, line: 5 } |  |  | 0.453 |
| ns | 9438 |  | 269 | settings/base.py — deployment, auth and database LD_* options | 6.1 |  | 0.448 |
| walker |  | 9482 | 156 | Code::CodeKey { rung: Decl, file: bookmarks/migrations/0038_globalsettings_guest_profile_user.py, decl: 1, sub: 0, line: 8 } |  |  | 0.448 |
| walker |  | 9576 | 94 | Code::CodeKey { rung: Names, file: bookmarks/widgets.py, decl: 0, sub: 0, line: 0 } |  |  | 0.448 |
| walker |  | 9588 | 12 | Code::CodeKey { rung: Decl, file: bookmarks/widgets.py, decl: 1, sub: 0, line: 7 } |  |  | 0.448 |
| walker |  | 9602 | 14 | Code::CodeKey { rung: Decl, file: bookmarks/widgets.py, decl: 2, sub: 0, line: 11 } |  |  | 0.448 |
| walker |  | 9616 | 14 | Code::CodeKey { rung: Decl, file: bookmarks/widgets.py, decl: 4, sub: 0, line: 19 } |  |  | 0.448 |
| walker |  | 9630 | 14 | Code::CodeKey { rung: Decl, file: bookmarks/widgets.py, decl: 6, sub: 0, line: 27 } |  |  | 0.448 |
| walker |  | 9644 | 14 | Code::CodeKey { rung: Decl, file: bookmarks/widgets.py, decl: 8, sub: 0, line: 35 } |  |  | 0.448 |
| walker |  | 9679 | 35 | Code::CodeKey { rung: Decl, file: bookmarks/widgets.py, decl: 10, sub: 0, line: 43 } |  |  | 0.448 |
| walker |  | 9714 | 35 | Code::CodeKey { rung: Decl, file: bookmarks/widgets.py, decl: 13, sub: 0, line: 63 } |  |  | 0.448 |
| walker |  | 9723 | 9 | Code::CodeKey { rung: Body, file: bookmarks/widgets.py, decl: 14, sub: 0, line: 64 } |  |  | 0.448 |
| ns | 9790 |  | 352 | settings/base.py — favicon, preview, snapshot and singlefile LD_* options | 6.2 |  | 0.442 |
| walker |  | 9818 | 95 | Code::CodeKey { rung: Names, file: bookmarks/feeds.py, decl: 0, sub: 0, line: 0 } |  |  | 0.442 |
| walker |  | 9860 | 42 | Code::CodeKey { rung: Decl, file: bookmarks/feeds.py, decl: 1, sub: 0, line: 14 } |  |  | 0.442 |
| ns | 9898 |  | 108 | Operational surfaces: docker/, scripts/ and management commands (complete listings) | 6.3 |  | 0.455 |
| walker |  | 9925 | 65 | Code::CodeKey { rung: Decl, file: bookmarks/feeds.py, decl: 12, sub: 0, line: 73 } |  |  | 0.455 |
| ns | 9985 |  | 87 | bookmarks/frontend/ component and utility listings (complete) | 7.1 |  | 0.467 |
| walker |  | 9991 | 66 | Code::CodeKey { rung: Decl, file: bookmarks/feeds.py, decl: 15, sub: 0, line: 84 } |  |  | 0.467 |

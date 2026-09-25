Score(3000)=0.667 I=0.836 C=0.532 ns_rows≤3K=18/51 grid(1000/1442/2080/3000/4327/6240/9000)=0.857/0.829/0.725/0.667/0.633/0.546/0.462

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 41 |  | 41 | What linkding is | 1.1 |  | 0.000 |
| ns | 112 |  | 71 | The one sentence that locates the code | 1.2 |  | 0.000 |
| walker |  | 115 | 115 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 131 | 16 | Json::Identity { file: package.json } |  |  | 0.000 |
| walker |  | 143 | 12 | Fs::DirListing { dir: docker } |  |  | 0.000 |
| walker |  | 146 | 3 | Fs::DirListing { dir: .github } |  |  | 0.000 |
| walker |  | 159 | 13 | Fs::DirListing { dir: .github/workflows } |  |  | 0.000 |
| walker |  | 164 | 5 | Fs::DirListing { dir: .devcontainer } |  |  | 0.000 |
| walker |  | 208 | 44 | Fs::DirListing { dir: assets } |  |  | 0.000 |
| ns | 227 |  | 115 | Repository root listing (complete) | 1.3 |  | 0.714 |
| walker |  | 262 | 54 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.945 |
| walker |  | 348 | 86 | Toml::Identity { file: pyproject.toml } |  |  | 0.649 |
| ns | 348 |  | 121 | bookmarks/ app package listing (complete) | 1.4 |  | 0.649 |
| walker |  | 358 | 10 | Plaintext::Whole { file: version.txt } |  |  | 0.649 |
| ns | 445 |  | 97 | README feature overview (head) | 1.5 |  | 0.605 |
| walker |  | 479 | 121 | Fs::DirListing { dir: bookmarks } |  |  | 0.894 |
| walker |  | 482 | 3 | Fs::DirListing { dir: bookmarks/management } |  |  | 0.894 |
| walker |  | 502 | 20 | Fs::DirListing { dir: bookmarks/api } |  |  | 0.895 |
| walker |  | 522 | 20 | Fs::DirListing { dir: bookmarks/templatetags } |  |  | 0.895 |
| walker |  | 544 | 22 | Fs::DirListing { dir: bookmarks/settings } |  |  | 0.898 |
| walker |  | 562 | 18 | Fs::DirListing { dir: bookmarks/frontend } |  |  | 0.898 |
| ns | 583 |  | 138 | README feature overview (tail) | 1.6 | 1.5 | 0.856 |
| walker |  | 589 | 27 | Fs::DirListing { dir: bookmarks/templates } |  |  | 0.856 |
| walker |  | 594 | 5 | Fs::DirListing { dir: bookmarks/templates/admin } |  |  | 0.856 |
| walker |  | 610 | 16 | Fs::DirListing { dir: bookmarks/templates/registration } |  |  | 0.856 |
| ns | 679 |  | 96 | bookmarks/views/ and bookmarks/api/ listings (complete) | 1.7 |  | 0.748 |
| walker |  | 686 | 76 | Fs::DirListing { dir: bookmarks/views } |  |  | 0.878 |
| ns | 762 |  | 83 | bookmarks/services/ listing (complete) | 1.8 |  | 0.800 |
| walker |  | 769 | 83 | Fs::DirListing { dir: bookmarks/services } |  |  | 0.891 |
| walker |  | 789 | 20 | Fs::DirListing { dir: bookmarks/templates/bundles } |  |  | 0.891 |
| walker |  | 809 | 20 | Fs::DirListing { dir: bookmarks/templates/tags } |  |  | 0.891 |
| walker |  | 836 | 27 | Fs::DirListing { dir: bookmarks/frontend/utils } |  |  | 0.891 |
| walker |  | 863 | 27 | Fs::DirListing { dir: bookmarks/templates/settings } |  |  | 0.891 |
| ns | 937 |  | 175 | Dev prerequisites + core make targets | 1.9 |  | 0.819 |
| ns | 1154 |  | 217 | Remaining make targets: lint, format, e2e, frontend | 1.10 | 1.9 | 0.749 |
| walker |  | 1224 | 361 | Plaintext::Whole { file: Makefile } |  |  | 0.881 |
| walker |  | 1293 | 69 | Fs::DirListing { dir: bookmarks/styles } |  |  | 0.881 |
| walker |  | 1334 | 41 | Fs::DirListing { dir: bookmarks/templates/shared } |  |  | 0.881 |
| walker |  | 1380 | 46 | Fs::DirListing { dir: bookmarks/management/commands } |  |  | 0.882 |
| ns | 1438 |  | 284 | pyproject.toml project metadata + runtime dependencies | 1.11 |  | 0.829 |
| ns | 1460 |  | 22 | bookmarks/settings/ listing (complete) | 1.12 |  | 0.831 |
| walker |  | 1472 | 92 | Fs::DirListing { dir: bookmarks/static } |  |  | 0.831 |
| walker |  | 1477 | 5 | Fs::DirListing { dir: bookmarks/static/vendor } |  |  | 0.831 |
| walker |  | 1574 | 97 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.831 |
| ns | 1602 |  | 142 | Settings resolution order + manage.py/pytest wiring | 1.13 | 1.12 | 0.800 |
| walker |  | 1609 | 35 | Fs::DirListing { dir: docs } |  |  | 0.801 |
| walker |  | 1669 | 60 | Fs::DirListing { dir: bookmarks/frontend/components } |  |  | 0.803 |
| walker |  | 1686 | 17 | Fs::DirListing { dir: docs/src } |  |  | 0.803 |
| walker |  | 1693 | 7 | Fs::DirListing { dir: docs/src/content } |  |  | 0.803 |
| ns | 1710 |  | 108 | docs/ site and its content pages (complete) | 1.14 |  | 0.757 |
| walker |  | 1762 | 69 | Fs::DirListing { dir: bookmarks/templates/bookmarks } |  |  | 0.757 |
| walker |  | 1779 | 17 | Fs::DirListing { dir: bookmarks/templates/bookmarks/details } |  |  | 0.757 |
| ns | 1885 |  | 175 | models.py symbol roster (all 17 top-level classes and functions) | 2.1 |  | 0.724 |
| walker |  | 1906 | 127 | Plaintext::Whole { file: docker-compose.yml } |  |  | 0.724 |
| walker |  | 1956 | 50 | Fs::DirListing { dir: scripts } |  |  | 0.725 |
| walker |  | 2043 | 87 | Fs::DirListing { dir: bookmarks/styles/theme } |  |  | 0.725 |
| walker |  | 2167 | 124 | Json::Scripts { file: package.json } |  |  | 0.725 |
| walker |  | 2178 | 11 | Code::CodeKey { rung: Names, file: manage.py, decl: 0, sub: 0, line: 0 } |  |  | 0.725 |
| ns | 2296 |  | 411 | Bookmark model fields (complete) | 2.2 | 2.1 | 0.679 |
| ns | 2439 |  | 143 | UserProfile field roster (names + types) | 2.3 | 2.1 | 0.654 |
| walker |  | 2792 | 614 | Fs::DirListing { dir: bookmarks/migrations } |  |  | 0.654 |
| ns | 2823 |  | 384 | UserProfile feature toggles (complete tail of the model) | 2.4 | 2.3 | 0.624 |
| walker |  | 2990 | 198 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.667 |
| ns | 3026 |  | 203 | BookmarkSearch — the search/filter parameter vocabulary | 2.5 | 2.1 | 0.639 |
| walker |  | 3198 | 208 | Json::Dependencies { file: package.json } |  |  | 0.639 |
| walker |  | 3245 | 47 | Plaintext::DeclSurface { file: pytest.ini } |  |  | 0.644 |
| ns | 3271 |  | 245 | Bookmark methods: resolved_title, tag_names, save, query_existing | 2.6 | 2.2 | 0.621 |
| ns | 3465 |  | 194 | Tag model + tag-string parsing rules | 2.7 | 2.1 | 0.605 |
| walker |  | 3605 | 360 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.642 |
| walker |  | 3667 | 62 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.642 |
| walker |  | 3740 | 73 | Fs::DirListing { dir: docs/src/content/docs } |  |  | 0.686 |
| ns | 3741 |  | 276 | BookmarkAsset — snapshot/upload model | 2.8 | 2.1 | 0.665 |
| walker |  | 3870 | 130 | Code::CodeKey { rung: Body, file: manage.py, decl: 1, sub: 0, line: 7 } |  |  | 0.668 |
| walker |  | 3886 | 16 | Plaintext::DeclSurface { file: bookmarks/static/robots.txt } |  |  | 0.668 |
| walker |  | 3910 | 24 | Code::CodeKey { rung: Names, file: bookmarks/context_processors.py, decl: 0, sub: 0, line: 0 } |  |  | 0.668 |
| walker |  | 3922 | 12 | Code::CodeKey { rung: Body, file: bookmarks/context_processors.py, decl: 2, sub: 0, line: 20 } |  |  | 0.668 |
| walker |  | 3934 | 12 | Code::CodeKey { rung: Names, file: bookmarks/wsgi.py, decl: 0, sub: 0, line: 0 } |  |  | 0.668 |
| ns | 3941 |  | 200 | BookmarkBundle — saved-filter model | 2.9 | 2.1 | 0.655 |
| walker |  | 4041 | 107 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.655 |
| walker |  | 4135 | 94 | Code::CodeKey { rung: Names, file: bookmarks/widgets.py, decl: 0, sub: 0, line: 0 } |  |  | 0.655 |
| walker |  | 4147 | 12 | Code::CodeKey { rung: Decl, file: bookmarks/widgets.py, decl: 1, sub: 0, line: 7 } |  |  | 0.655 |
| walker |  | 4161 | 14 | Code::CodeKey { rung: Decl, file: bookmarks/widgets.py, decl: 2, sub: 0, line: 11 } |  |  | 0.655 |
| walker |  | 4175 | 14 | Code::CodeKey { rung: Decl, file: bookmarks/widgets.py, decl: 4, sub: 0, line: 19 } |  |  | 0.655 |
| walker |  | 4189 | 14 | Code::CodeKey { rung: Decl, file: bookmarks/widgets.py, decl: 6, sub: 0, line: 27 } |  |  | 0.655 |
| walker |  | 4203 | 14 | Code::CodeKey { rung: Decl, file: bookmarks/widgets.py, decl: 8, sub: 0, line: 35 } |  |  | 0.655 |
| walker |  | 4238 | 35 | Code::CodeKey { rung: Decl, file: bookmarks/widgets.py, decl: 10, sub: 0, line: 43 } |  |  | 0.655 |
| ns | 4241 |  | 300 | Toast, FeedToken, ApiToken and GlobalSettings fields | 2.10 | 2.1 | 0.633 |
| walker |  | 4273 | 35 | Code::CodeKey { rung: Decl, file: bookmarks/widgets.py, decl: 13, sub: 0, line: 63 } |  |  | 0.633 |
| walker |  | 4282 | 9 | Code::CodeKey { rung: Body, file: bookmarks/widgets.py, decl: 14, sub: 0, line: 64 } |  |  | 0.633 |
| walker |  | 4430 | 148 | Code::CodeKey { rung: Names, file: bookmarks/utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.633 |
| ns | 4438 |  | 197 | Model signal side effects (profile creation, file cleanup) | 2.11 | 2.1 | 0.617 |
| walker |  | 4454 | 24 | Code::CodeKey { rung: Decl, file: bookmarks/utils.py, decl: 4, sub: 0, line: 43 } |  |  | 0.617 |
| walker |  | 4478 | 24 | Code::CodeKey { rung: Decl, file: bookmarks/utils.py, decl: 5, sub: 0, line: 63 } |  |  | 0.617 |
| walker |  | 4502 | 24 | Code::CodeKey { rung: Decl, file: bookmarks/utils.py, decl: 6, sub: 0, line: 83 } |  |  | 0.617 |
| walker |  | 4535 | 33 | Code::CodeKey { rung: Decl, file: bookmarks/utils.py, decl: 3, sub: 0, line: 36 } |  |  | 0.617 |
| walker |  | 4615 | 80 | Code::CodeKey { rung: Decl, file: bookmarks/utils.py, decl: 2, sub: 0, line: 25 } |  |  | 0.617 |
| walker |  | 4636 | 21 | Code::CodeKey { rung: Doc, file: bookmarks/utils.py, decl: 4, sub: 0, line: 43 } |  |  | 0.617 |
| walker |  | 4653 | 17 | Code::CodeKey { rung: Body, file: bookmarks/utils.py, decl: 1, sub: 0, line: 21 } |  |  | 0.617 |
| ns | 4694 |  | 256 | urls.py — root and bookmark page routes | 3.1 |  | 0.606 |
| walker |  | 4748 | 95 | Code::CodeKey { rung: Names, file: bookmarks/feeds.py, decl: 0, sub: 0, line: 0 } |  |  | 0.606 |
| walker |  | 4790 | 42 | Code::CodeKey { rung: Decl, file: bookmarks/feeds.py, decl: 1, sub: 0, line: 14 } |  |  | 0.606 |
| walker |  | 4855 | 65 | Code::CodeKey { rung: Decl, file: bookmarks/feeds.py, decl: 12, sub: 0, line: 73 } |  |  | 0.606 |
| walker |  | 4921 | 66 | Code::CodeKey { rung: Decl, file: bookmarks/feeds.py, decl: 15, sub: 0, line: 84 } |  |  | 0.606 |
| ns | 4970 |  | 276 | urls.py — asset, bundle and tag routes | 3.2 | 3.1 | 0.594 |
| walker |  | 4987 | 66 | Code::CodeKey { rung: Decl, file: bookmarks/feeds.py, decl: 18, sub: 0, line: 97 } |  |  | 0.594 |
| walker |  | 5069 | 82 | Code::CodeKey { rung: Decl, file: bookmarks/feeds.py, decl: 21, sub: 0, line: 110 } |  |  | 0.594 |
| ns | 5188 |  | 218 | urls.py — settings and toast routes | 3.3 | 3.2 | 0.585 |
| walker |  | 5209 | 140 | Code::CodeKey { rung: Decl, file: bookmarks/feeds.py, decl: 3, sub: 0, line: 31 } |  |  | 0.585 |
| walker |  | 5216 | 7 | Code::CodeKey { rung: Body, file: bookmarks/feeds.py, decl: 9, sub: 0, line: 63 } |  |  | 0.585 |
| walker |  | 5224 | 8 | Code::CodeKey { rung: Body, file: bookmarks/feeds.py, decl: 5, sub: 0, line: 48 } |  |  | 0.585 |
| walker |  | 5232 | 8 | Code::CodeKey { rung: Body, file: bookmarks/feeds.py, decl: 10, sub: 0, line: 66 } |  |  | 0.585 |
| walker |  | 5240 | 8 | Code::CodeKey { rung: Body, file: bookmarks/feeds.py, decl: 11, sub: 0, line: 69 } |  |  | 0.585 |
| walker |  | 5343 | 103 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.585 |
| walker |  | 5454 | 111 | Code::CodeKey { rung: Names, file: bookmarks/forms.py, decl: 0, sub: 0, line: 0 } |  |  | 0.586 |
| ns | 5519 |  | 331 | urls.py — API mounts, feeds and utility endpoints | 3.4 | 3.3 | 0.574 |
| walker |  | 5536 | 82 | Code::CodeKey { rung: Decl, file: bookmarks/forms.py, decl: 12, sub: 0, line: 161 } |  |  | 0.574 |
| walker |  | 5633 | 97 | Code::CodeKey { rung: Decl, file: bookmarks/forms.py, decl: 8, sub: 0, line: 124 } |  |  | 0.574 |
| walker |  | 5748 | 115 | Code::CodeKey { rung: Decl, file: bookmarks/forms.py, decl: 21, sub: 0, line: 371 } |  |  | 0.574 |
| walker |  | 5777 | 29 | Code::CodeKey { rung: Body, file: bookmarks/forms.py, decl: 9, sub: 0, line: 131 } |  |  | 0.574 |
| ns | 5795 |  | 276 | urls.py — conditional routes: live reload, auth, admin, OIDC, context path | 3.5 | 3.4 | 0.561 |
| walker |  | 5806 | 29 | Code::CodeKey { rung: Body, file: bookmarks/forms.py, decl: 13, sub: 0, line: 165 } |  |  | 0.561 |
| ns | 6056 |  | 261 | views/bookmarks.py function roster (complete) | 3.6 |  | 0.546 |
| walker |  | 6133 | 327 | Code::CodeKey { rung: Decl, file: bookmarks/forms.py, decl: 16, sub: 0, line: 215 } |  |  | 0.546 |
| walker |  | 6152 | 19 | Code::CodeKey { rung: Body, file: bookmarks/forms.py, decl: 17, sub: 0, line: 244 } |  |  | 0.546 |
| ns | 6328 |  | 272 | views/contexts.py class roster (complete) | 3.7 |  | 0.532 |
| walker |  | 6538 | 386 | Code::CodeKey { rung: Decl, file: bookmarks/forms.py, decl: 1, sub: 0, line: 31 } |  |  | 0.532 |
| walker |  | 6546 | 8 | Code::CodeKey { rung: Decl, file: bookmarks/forms.py, decl: 3, sub: 0, line: 79 } |  |  | 0.532 |
| walker |  | 6554 | 8 | Code::CodeKey { rung: Decl, file: bookmarks/forms.py, decl: 4, sub: 0, line: 85 } |  |  | 0.532 |
| ns | 6570 |  | 242 | views/settings.py + views/tags.py + views/bundles.py function rosters (complete) | 3.8 |  | 0.521 |
| ns | 6741 |  | 171 | views/access.py — the complete authorization helper set | 3.9 |  | 0.517 |
| walker |  | 6940 | 386 | Code::CodeKey { rung: Decl, file: bookmarks/forms.py, decl: 18, sub: 0, line: 248 } |  |  | 0.517 |
| walker |  | 6991 | 51 | Code::CodeKey { rung: Decl, file: bookmarks/forms.py, decl: 19, sub: 0, line: 275 } |  |  | 0.517 |
| ns | 7007 |  | 266 | Remaining view modules: turbo, assets, auth, root, health, manifest, opensearch, custom_css, toasts, reload | 3.10 |  | 0.508 |
| ns | 7095 |  | 88 | forms.py class roster (complete) | 3.11 |  | 0.514 |
| walker |  | 7186 | 195 | Code::CodeKey { rung: Names, file: bookmarks/admin.py, decl: 0, sub: 0, line: 0 } |  |  | 0.514 |
| walker |  | 7215 | 29 | Code::CodeKey { rung: Decl, file: bookmarks/admin.py, decl: 29, sub: 0, line: 318 } |  |  | 0.514 |
| walker |  | 7250 | 35 | Code::CodeKey { rung: Decl, file: bookmarks/admin.py, decl: 25, sub: 0, line: 297 } |  |  | 0.514 |
| walker |  | 7287 | 37 | Code::CodeKey { rung: Decl, file: bookmarks/admin.py, decl: 28, sub: 0, line: 312 } |  |  | 0.514 |
| ns | 7309 |  | 214 | Request-scoped plumbing: typed HttpRequest + LinkdingMiddleware | 3.12 |  | 0.506 |
| ns | 7329 |  | 20 | bookmarks/templatetags/ listing (complete) | 3.13 |  | 0.509 |
| walker |  | 7335 | 48 | Code::CodeKey { rung: Decl, file: bookmarks/admin.py, decl: 27, sub: 0, line: 306 } |  |  | 0.509 |
| walker |  | 7387 | 52 | Code::CodeKey { rung: Decl, file: bookmarks/admin.py, decl: 1, sub: 0, line: 29 } |  |  | 0.509 |
| walker |  | 7395 | 8 | Code::CodeKey { rung: Decl, file: bookmarks/admin.py, decl: 3, sub: 0, line: 34 } |  |  | 0.509 |
| walker |  | 7449 | 54 | Code::CodeKey { rung: Decl, file: bookmarks/admin.py, decl: 24, sub: 0, line: 289 } |  |  | 0.509 |
| walker |  | 7509 | 60 | Code::CodeKey { rung: Decl, file: bookmarks/admin.py, decl: 7, sub: 0, line: 77 } |  |  | 0.509 |
| ns | 7563 |  | 234 | api/routes.py — viewsets and router registrations (complete) | 4.1 |  | 0.498 |
| walker |  | 7578 | 69 | Code::CodeKey { rung: Decl, file: bookmarks/admin.py, decl: 30, sub: 0, line: 324 } |  |  | 0.498 |
| walker |  | 7657 | 79 | Code::CodeKey { rung: Decl, file: bookmarks/admin.py, decl: 17, sub: 0, line: 215 } |  |  | 0.498 |
| walker |  | 7668 | 11 | Code::CodeKey { rung: Decl, file: bookmarks/admin.py, decl: 18, sub: 0, line: 216 } |  |  | 0.498 |
| ns | 7779 |  | 216 | BookmarkViewSet — every custom action and override | 4.2 | 4.1 | 0.492 |
| walker |  | 7808 | 140 | Code::CodeKey { rung: Decl, file: bookmarks/admin.py, decl: 23, sub: 0, line: 272 } |  |  | 0.492 |
| ns | 7928 |  | 149 | Asset, user and bundle viewset actions | 4.3 | 4.1 | 0.488 |
| walker |  | 7958 | 150 | Code::CodeKey { rung: Decl, file: bookmarks/admin.py, decl: 19, sub: 0, line: 228 } |  |  | 0.488 |
| ns | 8017 |  | 89 | api/serializers.py class roster (complete) | 4.4 |  | 0.485 |
| ns | 8136 |  | 119 | API token authentication + docs/api.md section map | 4.5 |  | 0.481 |
| walker |  | 8308 | 350 | Code::CodeKey { rung: Decl, file: bookmarks/admin.py, decl: 10, sub: 0, line: 108 } |  |  | 0.481 |
| ns | 8365 |  | 229 | services/bookmarks.py function roster (complete) | 5.1 |  | 0.474 |
| ns | 8566 |  | 201 | queries.py function roster (complete) | 5.2 |  | 0.468 |
| walker |  | 8804 | 496 | Plaintext::Whole { file: bootstrap.sh } |  |  | 0.468 |
| ns | 8816 |  | 250 | services/search_query_parser.py symbol roster (complete) | 5.3 |  | 0.461 |
| walker |  | 8861 | 57 | Code::CodeKey { rung: Names, file: bookmarks/middlewares.py, decl: 0, sub: 0, line: 0 } |  |  | 0.461 |
| walker |  | 8874 | 13 | Code::CodeKey { rung: Decl, file: bookmarks/middlewares.py, decl: 1, sub: 0, line: 7 } |  |  | 0.461 |
| walker |  | 8903 | 29 | Code::CodeKey { rung: Decl, file: bookmarks/middlewares.py, decl: 4, sub: 0, line: 17 } |  |  | 0.461 |
| walker |  | 8913 | 10 | Code::CodeKey { rung: Body, file: bookmarks/middlewares.py, decl: 5, sub: 0, line: 18 } |  |  | 0.461 |
| ns | 9169 |  | 353 | services/tasks.py function roster (complete) | 5.4 |  | 0.452 |
| walker |  | 9179 | 266 | Code::CodeKey { rung: Names, file: bookmarks/models.py, decl: 0, sub: 0, line: 0 } |  |  | 0.475 |
| walker |  | 9192 | 13 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 14, sub: 0, line: 116 } |  |  | 0.475 |
| walker |  | 9205 | 13 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 34, sub: 0, line: 472 } |  |  | 0.476 |
| walker |  | 9218 | 13 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 35, sub: 0, line: 478 } |  |  | 0.477 |
| walker |  | 9232 | 14 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 19, sub: 0, line: 172 } |  |  | 0.478 |
| walker |  | 9286 | 54 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 36, sub: 0, line: 483 } |  |  | 0.479 |
| walker |  | 9347 | 61 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 2, sub: 0, line: 21 } |  |  | 0.479 |
| walker |  | 9354 | 7 | Code::CodeKey { rung: Body, file: bookmarks/models.py, decl: 3, sub: 0, line: 26 } |  |  | 0.479 |
| ns | 9438 |  | 269 | settings/base.py — deployment, auth and database LD_* options | 6.1 |  | 0.474 |
| walker |  | 9483 | 129 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 37, sub: 0, line: 490 } |  |  | 0.476 |
| walker |  | 9491 | 8 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 39, sub: 0, line: 508 } |  |  | 0.476 |
| walker |  | 9633 | 142 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 41, sub: 0, line: 516 } |  |  | 0.479 |
| walker |  | 9641 | 8 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 43, sub: 0, line: 531 } |  |  | 0.479 |
| walker |  | 9665 | 24 | Code::CodeKey { rung: Doc, file: bookmarks/models.py, decl: 37, sub: 0, line: 490 } |  |  | 0.479 |
| ns | 9790 |  | 352 | settings/base.py — favicon, preview, snapshot and singlefile LD_* options | 6.2 |  | 0.473 |
| walker |  | 9898 | 233 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 45, sub: 0, line: 539 } |  |  | 0.503 |
| ns | 9898 |  | 108 | Operational surfaces: docker/, scripts/ and management commands (complete listings) | 6.3 |  | 0.503 |
| walker |  | 9904 | 6 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 46, sub: 0, line: 558 } |  |  | 0.504 |
| ns | 9985 |  | 87 | bookmarks/frontend/ component and utility listings (complete) | 7.1 |  | 0.514 |
| walker |  | 9997 | 93 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 15, sub: 0, line: 129 } |  |  | 0.516 |

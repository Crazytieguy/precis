Score(3000)=0.708 I=0.866 C=0.578 ns_rows≤3K=18/51 grid(1000/1442/2080/3000/4327/6240/9000)=0.733/0.829/0.725/0.708/0.664/0.691/0.651

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 41 |  | 41 | What linkding is | 1.1 |  | 0.000 |
| ns | 112 |  | 71 | The one sentence that locates the code | 1.2 |  | 0.000 |
| walker |  | 115 | 115 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 169 | 54 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.816 |
| walker |  | 185 | 16 | Json::Identity { file: package.json } |  |  | 0.816 |
| walker |  | 197 | 12 | Fs::DirListing { dir: docker } |  |  | 0.816 |
| walker |  | 202 | 5 | Fs::DirListing { dir: .devcontainer } |  |  | 0.816 |
| ns | 227 |  | 115 | Repository root listing (complete) | 1.3 |  | 0.945 |
| walker |  | 246 | 44 | Fs::DirListing { dir: assets } |  |  | 0.945 |
| walker |  | 332 | 86 | Toml::Identity { file: pyproject.toml } |  |  | 0.947 |
| walker |  | 347 | 15 | Fs::DirListing { dir: .github/workflows } |  |  | 0.947 |
| ns | 348 |  | 121 | bookmarks/ app package listing (complete) | 1.4 |  | 0.649 |
| ns | 445 |  | 97 | README feature overview (head) | 1.5 |  | 0.605 |
| ns | 583 |  | 138 | README feature overview (tail) | 1.6 | 1.5 | 0.577 |
| ns | 679 |  | 96 | bookmarks/views/ and bookmarks/api/ listings (complete) | 1.7 |  | 0.501 |
| walker |  | 708 | 361 | Plaintext::Whole { file: Makefile } |  |  | 0.516 |
| ns | 762 |  | 83 | bookmarks/services/ listing (complete) | 1.8 |  | 0.470 |
| walker |  | 829 | 121 | Fs::DirListing { dir: bookmarks } |  |  | 0.692 |
| walker |  | 847 | 18 | Fs::DirListing { dir: bookmarks/frontend } |  |  | 0.692 |
| walker |  | 867 | 20 | Fs::DirListing { dir: bookmarks/api } |  |  | 0.697 |
| walker |  | 887 | 20 | Fs::DirListing { dir: bookmarks/templatetags } |  |  | 0.697 |
| walker |  | 909 | 22 | Fs::DirListing { dir: bookmarks/settings } |  |  | 0.699 |
| walker |  | 936 | 27 | Fs::DirListing { dir: bookmarks/templates } |  |  | 0.699 |
| ns | 937 |  | 175 | Dev prerequisites + core make targets | 1.9 |  | 0.693 |
| walker |  | 941 | 5 | Fs::DirListing { dir: bookmarks/templates/admin } |  |  | 0.693 |
| walker |  | 957 | 16 | Fs::DirListing { dir: bookmarks/templates/registration } |  |  | 0.693 |
| walker |  | 1033 | 76 | Fs::DirListing { dir: bookmarks/views } |  |  | 0.799 |
| walker |  | 1116 | 83 | Fs::DirListing { dir: bookmarks/services } |  |  | 0.881 |
| walker |  | 1136 | 20 | Fs::DirListing { dir: bookmarks/templates/bundles } |  |  | 0.881 |
| ns | 1154 |  | 217 | Remaining make targets: lint, format, e2e, frontend | 1.10 | 1.9 | 0.881 |
| walker |  | 1156 | 20 | Fs::DirListing { dir: bookmarks/templates/tags } |  |  | 0.881 |
| walker |  | 1183 | 27 | Fs::DirListing { dir: bookmarks/frontend/utils } |  |  | 0.881 |
| walker |  | 1210 | 27 | Fs::DirListing { dir: bookmarks/templates/settings } |  |  | 0.881 |
| walker |  | 1220 | 10 | Plaintext::DeclSurface { file: version.txt } |  |  | 0.881 |
| walker |  | 1289 | 69 | Fs::DirListing { dir: bookmarks/styles } |  |  | 0.881 |
| walker |  | 1330 | 41 | Fs::DirListing { dir: bookmarks/templates/shared } |  |  | 0.881 |
| walker |  | 1427 | 97 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.881 |
| ns | 1438 |  | 284 | pyproject.toml project metadata + runtime dependencies | 1.11 |  | 0.829 |
| ns | 1460 |  | 22 | bookmarks/settings/ listing (complete) | 1.12 |  | 0.830 |
| walker |  | 1519 | 92 | Fs::DirListing { dir: bookmarks/static } |  |  | 0.830 |
| walker |  | 1524 | 5 | Fs::DirListing { dir: bookmarks/static/vendor } |  |  | 0.830 |
| walker |  | 1572 | 48 | Fs::DirListing { dir: bookmarks/management/commands } |  |  | 0.831 |
| ns | 1602 |  | 142 | Settings resolution order + manage.py/pytest wiring | 1.13 | 1.12 | 0.800 |
| walker |  | 1607 | 35 | Fs::DirListing { dir: docs } |  |  | 0.801 |
| walker |  | 1667 | 60 | Fs::DirListing { dir: bookmarks/frontend/components } |  |  | 0.803 |
| walker |  | 1684 | 17 | Fs::DirListing { dir: docs/src } |  |  | 0.803 |
| walker |  | 1691 | 7 | Fs::DirListing { dir: docs/src/content } |  |  | 0.803 |
| walker |  | 1700 | 9 | Fs::DirListing { dir: docs/src/components } |  |  | 0.803 |
| ns | 1710 |  | 108 | docs/ site and its content pages (complete) | 1.14 |  | 0.757 |
| walker |  | 1827 | 127 | Plaintext::Whole { file: docker-compose.yml } |  |  | 0.757 |
| ns | 1885 |  | 175 | models.py symbol roster (all 17 top-level classes and functions) | 2.1 |  | 0.724 |
| walker |  | 1896 | 69 | Fs::DirListing { dir: bookmarks/templates/bookmarks } |  |  | 0.724 |
| walker |  | 1913 | 17 | Fs::DirListing { dir: bookmarks/templates/bookmarks/details } |  |  | 0.724 |
| walker |  | 1963 | 50 | Fs::DirListing { dir: scripts } |  |  | 0.725 |
| walker |  | 2087 | 124 | Json::Scripts { file: package.json } |  |  | 0.725 |
| walker |  | 2174 | 87 | Fs::DirListing { dir: bookmarks/styles/theme } |  |  | 0.725 |
| ns | 2296 |  | 411 | Bookmark model fields (complete) | 2.2 | 2.1 | 0.679 |
| ns | 2439 |  | 143 | UserProfile field roster (names + types) | 2.3 | 2.1 | 0.654 |
| walker |  | 2534 | 360 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.697 |
| walker |  | 2732 | 198 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.742 |
| ns | 2823 |  | 384 | UserProfile feature toggles (complete tail of the model) | 2.4 | 2.3 | 0.708 |
| walker |  | 2940 | 208 | Json::Dependencies { file: package.json } |  |  | 0.708 |
| ns | 3026 |  | 203 | BookmarkSearch — the search/filter parameter vocabulary | 2.5 | 2.1 | 0.679 |
| ns | 3271 |  | 245 | Bookmark methods: resolved_title, tag_names, save, query_existing | 2.6 | 2.2 | 0.655 |
| ns | 3465 |  | 194 | Tag model + tag-string parsing rules | 2.7 | 2.1 | 0.638 |
| walker |  | 3554 | 614 | Fs::DirListing { dir: bookmarks/migrations } |  |  | 0.638 |
| walker |  | 3661 | 107 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.638 |
| walker |  | 3708 | 47 | Plaintext::DeclSurface { file: pytest.ini } |  |  | 0.642 |
| ns | 3741 |  | 276 | BookmarkAsset — snapshot/upload model | 2.8 | 2.1 | 0.623 |
| walker |  | 3811 | 103 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.623 |
| walker |  | 3884 | 73 | Fs::DirListing { dir: docs/src/content/docs } |  |  | 0.665 |
| walker |  | 3900 | 16 | Plaintext::DeclSurface { file: bookmarks/static/robots.txt } |  |  | 0.665 |
| ns | 3941 |  | 200 | BookmarkBundle — saved-filter model | 2.9 | 2.1 | 0.651 |
| walker |  | 4219 | 319 | Code::CodeKey { rung: Names, file: bookmarks/models.py, decl: 0, sub: 0, line: 0 } |  |  | 0.684 |
| ns | 4241 |  | 300 | Toast, FeedToken, ApiToken and GlobalSettings fields | 2.10 | 2.1 | 0.661 |
| walker |  | 4273 | 54 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 36, sub: 0, line: 483 } |  |  | 0.662 |
| walker |  | 4334 | 61 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 2, sub: 0, line: 21 } |  |  | 0.664 |
| ns | 4438 |  | 197 | Model signal side effects (profile creation, file cleanup) | 2.11 | 2.1 | 0.653 |
| walker |  | 4471 | 137 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 37, sub: 0, line: 490 } |  |  | 0.655 |
| walker |  | 4621 | 150 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 41, sub: 0, line: 516 } |  |  | 0.661 |
| ns | 4694 |  | 256 | urls.py — root and bookmark page routes | 3.1 |  | 0.649 |
| walker |  | 4950 | 329 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 15, sub: 0, line: 129 } |  |  | 0.680 |
| ns | 4970 |  | 276 | urls.py — asset, bundle and tag routes | 3.2 | 3.1 | 0.667 |
| walker |  | 5118 | 168 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 7, sub: 0, line: 53 } |  |  | 0.673 |
| ns | 5188 |  | 218 | urls.py — settings and toast routes | 3.3 | 3.2 | 0.662 |
| walker |  | 5282 | 164 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 7, sub: 1, line: 53 } |  |  | 0.681 |
| ns | 5519 |  | 331 | urls.py — API mounts, feeds and utility endpoints | 3.4 | 3.3 | 0.668 |
| walker |  | 5526 | 244 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 45, sub: 0, line: 539 } |  |  | 0.693 |
| walker |  | 5756 | 230 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 22, sub: 0, line: 224 } |  |  | 0.699 |
| ns | 5795 |  | 276 | urls.py — conditional routes: live reload, auth, admin, OIDC, context path | 3.5 | 3.4 | 0.683 |
| ns | 6056 |  | 261 | views/bookmarks.py function roster (complete) | 3.6 |  | 0.664 |
| walker |  | 6068 | 312 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 22, sub: 1, line: 224 } |  |  | 0.689 |
| walker |  | 6186 | 118 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 23, sub: 0, line: 260 } |  |  | 0.689 |
| ns | 6328 |  | 272 | views/contexts.py class roster (complete) | 3.7 |  | 0.673 |
| walker |  | 6389 | 203 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 7, sub: 2, line: 53 } |  |  | 0.691 |
| walker |  | 6565 | 176 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 20, sub: 0, line: 183 } |  |  | 0.691 |
| ns | 6570 |  | 242 | views/settings.py + views/tags.py + views/bundles.py function rosters (complete) | 3.8 |  | 0.676 |
| ns | 6741 |  | 171 | views/access.py — the complete authorization helper set | 3.9 |  | 0.671 |
| walker |  | 6852 | 287 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 20, sub: 1, line: 183 } |  |  | 0.687 |
| ns | 7007 |  | 266 | Remaining view modules: turbo, assets, auth, root, health, manifest, opensearch, custom_css, toasts, reload | 3.10 |  | 0.675 |
| walker |  | 7060 | 208 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 32, sub: 0, line: 346 } |  |  | 0.675 |
| ns | 7095 |  | 88 | forms.py class roster (complete) | 3.11 |  | 0.670 |
| walker |  | 7239 | 179 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 32, sub: 1, line: 346 } |  |  | 0.670 |
| ns | 7309 |  | 214 | Request-scoped plumbing: typed HttpRequest + LinkdingMiddleware | 3.12 |  | 0.660 |
| ns | 7329 |  | 20 | bookmarks/templatetags/ listing (complete) | 3.13 |  | 0.661 |
| walker |  | 7413 | 174 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 32, sub: 2, line: 346 } |  |  | 0.661 |
| ns | 7563 |  | 234 | api/routes.py — viewsets and router registrations (complete) | 4.1 |  | 0.647 |
| walker |  | 7595 | 182 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 32, sub: 3, line: 346 } |  |  | 0.650 |
| ns | 7779 |  | 216 | BookmarkViewSet — every custom action and override | 4.2 | 4.1 | 0.642 |
| walker |  | 7830 | 235 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 32, sub: 4, line: 346 } |  |  | 0.653 |
| ns | 7928 |  | 149 | Asset, user and bundle viewset actions | 4.3 | 4.1 | 0.648 |
| walker |  | 8011 | 181 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 32, sub: 5, line: 346 } |  |  | 0.655 |
| ns | 8017 |  | 89 | api/serializers.py class roster (complete) | 4.4 |  | 0.651 |
| ns | 8136 |  | 119 | API token authentication + docs/api.md section map | 4.5 |  | 0.646 |
| walker |  | 8183 | 172 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 32, sub: 6, line: 346 } |  |  | 0.653 |
| ns | 8365 |  | 229 | services/bookmarks.py function roster (complete) | 5.1 |  | 0.643 |
| walker |  | 8408 | 225 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 32, sub: 7, line: 346 } |  |  | 0.662 |
| walker |  | 8519 | 111 | Code::CodeKey { rung: Names, file: bookmarks/forms.py, decl: 0, sub: 0, line: 0 } |  |  | 0.669 |
| ns | 8566 |  | 201 | queries.py function roster (complete) | 5.2 |  | 0.661 |
| walker |  | 8601 | 82 | Code::CodeKey { rung: Decl, file: bookmarks/forms.py, decl: 12, sub: 0, line: 161 } |  |  | 0.661 |
| walker |  | 8698 | 97 | Code::CodeKey { rung: Decl, file: bookmarks/forms.py, decl: 8, sub: 0, line: 124 } |  |  | 0.661 |
| walker |  | 8813 | 115 | Code::CodeKey { rung: Decl, file: bookmarks/forms.py, decl: 21, sub: 0, line: 371 } |  |  | 0.661 |
| ns | 8816 |  | 250 | services/search_query_parser.py symbol roster (complete) | 5.3 |  | 0.651 |
| ns | 9169 |  | 353 | services/tasks.py function roster (complete) | 5.4 |  | 0.638 |
| walker |  | 9215 | 402 | Code::CodeKey { rung: Decl, file: bookmarks/forms.py, decl: 1, sub: 0, line: 31 } |  |  | 0.638 |
| ns | 9438 |  | 269 | settings/base.py — deployment, auth and database LD_* options | 6.1 |  | 0.631 |
| walker |  | 9542 | 327 | Code::CodeKey { rung: Decl, file: bookmarks/forms.py, decl: 16, sub: 0, line: 215 } |  |  | 0.631 |
| walker |  | 9723 | 181 | Code::CodeKey { rung: Names, file: bookmarks/utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.631 |
| walker |  | 9736 | 13 | Code::CodeKey { rung: Decl, file: bookmarks/utils.py, decl: 4, sub: 0, line: 43 } |  |  | 0.631 |
| walker |  | 9753 | 17 | Code::CodeKey { rung: Decl, file: bookmarks/utils.py, decl: 5, sub: 0, line: 63 } |  |  | 0.631 |
| walker |  | 9770 | 17 | Code::CodeKey { rung: Decl, file: bookmarks/utils.py, decl: 6, sub: 0, line: 83 } |  |  | 0.631 |
| ns | 9790 |  | 352 | settings/base.py — favicon, preview, snapshot and singlefile LD_* options | 6.2 |  | 0.623 |
| walker |  | 9795 | 25 | Code::CodeKey { rung: Decl, file: bookmarks/utils.py, decl: 3, sub: 0, line: 36 } |  |  | 0.623 |
| walker |  | 9875 | 80 | Code::CodeKey { rung: Decl, file: bookmarks/utils.py, decl: 2, sub: 0, line: 25 } |  |  | 0.623 |
| ns | 9898 |  | 108 | Operational surfaces: docker/, scripts/ and management commands (complete listings) | 6.3 |  | 0.630 |
| walker |  | 9961 | 86 | Code::CodeKey { rung: Decl, file: bookmarks/forms.py, decl: 18, sub: 0, line: 248 } |  |  | 0.630 |
| ns | 9985 |  | 87 | bookmarks/frontend/ component and utility listings (complete) | 7.1 |  | 0.636 |

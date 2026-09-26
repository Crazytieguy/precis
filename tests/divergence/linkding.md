Score(3000)=0.708 I=0.866 C=0.578 ns_rows≤3K=18/51 grid(1000/1442/2080/3000/4327/6240/9000)=0.733/0.829/0.725/0.708/0.679/0.721/0.678

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
| walker |  | 1599 | 172 | Markdown::CommandBlock { file: README.md, row: 71 } |  |  | 0.830 |
| ns | 1602 |  | 142 | Settings resolution order + manage.py/pytest wiring | 1.13 | 1.12 | 0.800 |
| walker |  | 1647 | 48 | Fs::DirListing { dir: bookmarks/management/commands } |  |  | 0.800 |
| walker |  | 1682 | 35 | Fs::DirListing { dir: docs } |  |  | 0.801 |
| ns | 1710 |  | 108 | docs/ site and its content pages (complete) | 1.14 |  | 0.755 |
| walker |  | 1742 | 60 | Fs::DirListing { dir: bookmarks/frontend/components } |  |  | 0.757 |
| walker |  | 1759 | 17 | Fs::DirListing { dir: docs/src } |  |  | 0.757 |
| walker |  | 1766 | 7 | Fs::DirListing { dir: docs/src/content } |  |  | 0.757 |
| walker |  | 1775 | 9 | Fs::DirListing { dir: docs/src/components } |  |  | 0.757 |
| ns | 1885 |  | 175 | models.py symbol roster (all 17 top-level classes and functions) | 2.1 |  | 0.724 |
| walker |  | 1902 | 127 | Plaintext::Whole { file: docker-compose.yml } |  |  | 0.724 |
| walker |  | 1971 | 69 | Fs::DirListing { dir: bookmarks/templates/bookmarks } |  |  | 0.724 |
| walker |  | 1988 | 17 | Fs::DirListing { dir: bookmarks/templates/bookmarks/details } |  |  | 0.724 |
| walker |  | 2038 | 50 | Fs::DirListing { dir: scripts } |  |  | 0.725 |
| walker |  | 2162 | 124 | Json::Scripts { file: package.json } |  |  | 0.725 |
| walker |  | 2249 | 87 | Fs::DirListing { dir: bookmarks/styles/theme } |  |  | 0.725 |
| ns | 2296 |  | 411 | Bookmark model fields (complete) | 2.2 | 2.1 | 0.679 |
| ns | 2439 |  | 143 | UserProfile field roster (names + types) | 2.3 | 2.1 | 0.654 |
| walker |  | 2609 | 360 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.697 |
| walker |  | 2807 | 198 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.742 |
| ns | 2823 |  | 384 | UserProfile feature toggles (complete tail of the model) | 2.4 | 2.3 | 0.708 |
| walker |  | 3015 | 208 | Json::Dependencies { file: package.json } |  |  | 0.708 |
| ns | 3026 |  | 203 | BookmarkSearch — the search/filter parameter vocabulary | 2.5 | 2.1 | 0.679 |
| ns | 3271 |  | 245 | Bookmark methods: resolved_title, tag_names, save, query_existing | 2.6 | 2.2 | 0.655 |
| ns | 3465 |  | 194 | Tag model + tag-string parsing rules | 2.7 | 2.1 | 0.638 |
| walker |  | 3629 | 614 | Fs::DirListing { dir: bookmarks/migrations } |  |  | 0.638 |
| walker |  | 3736 | 107 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.638 |
| ns | 3741 |  | 276 | BookmarkAsset — snapshot/upload model | 2.8 | 2.1 | 0.619 |
| walker |  | 3783 | 47 | Plaintext::DeclSurface { file: pytest.ini } |  |  | 0.623 |
| walker |  | 3886 | 103 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.623 |
| ns | 3941 |  | 200 | BookmarkBundle — saved-filter model | 2.9 | 2.1 | 0.610 |
| walker |  | 3959 | 73 | Fs::DirListing { dir: docs/src/content/docs } |  |  | 0.651 |
| walker |  | 4094 | 135 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.688 |
| ns | 4241 |  | 300 | Toast, FeedToken, ApiToken and GlobalSettings fields | 2.10 | 2.1 | 0.665 |
| walker |  | 4413 | 319 | Code::CodeKey { rung: Names, file: bookmarks/models.py, decl: 0, sub: 0, line: 0 } |  |  | 0.696 |
| ns | 4438 |  | 197 | Model signal side effects (profile creation, file cleanup) | 2.11 | 2.1 | 0.685 |
| walker |  | 4467 | 54 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 36, sub: 0, line: 483 } |  |  | 0.686 |
| walker |  | 4528 | 61 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 2, sub: 0, line: 21 } |  |  | 0.688 |
| walker |  | 4665 | 137 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 37, sub: 0, line: 490 } |  |  | 0.690 |
| ns | 4694 |  | 256 | urls.py — root and bookmark page routes | 3.1 |  | 0.678 |
| walker |  | 4815 | 150 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 41, sub: 0, line: 516 } |  |  | 0.683 |
| ns | 4970 |  | 276 | urls.py — asset, bundle and tag routes | 3.2 | 3.1 | 0.670 |
| walker |  | 5144 | 329 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 15, sub: 0, line: 129 } |  |  | 0.701 |
| ns | 5188 |  | 218 | urls.py — settings and toast routes | 3.3 | 3.2 | 0.690 |
| walker |  | 5312 | 168 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 7, sub: 0, line: 53 } |  |  | 0.696 |
| walker |  | 5476 | 164 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 7, sub: 1, line: 53 } |  |  | 0.715 |
| ns | 5519 |  | 331 | urls.py — API mounts, feeds and utility endpoints | 3.4 | 3.3 | 0.701 |
| walker |  | 5720 | 244 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 45, sub: 0, line: 539 } |  |  | 0.727 |
| ns | 5795 |  | 276 | urls.py — conditional routes: live reload, auth, admin, OIDC, context path | 3.5 | 3.4 | 0.710 |
| walker |  | 5950 | 230 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 22, sub: 0, line: 224 } |  |  | 0.715 |
| ns | 6056 |  | 261 | views/bookmarks.py function roster (complete) | 3.6 |  | 0.696 |
| walker |  | 6262 | 312 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 22, sub: 1, line: 224 } |  |  | 0.721 |
| ns | 6328 |  | 272 | views/contexts.py class roster (complete) | 3.7 |  | 0.704 |
| walker |  | 6380 | 118 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 23, sub: 0, line: 260 } |  |  | 0.704 |
| ns | 6570 |  | 242 | views/settings.py + views/tags.py + views/bundles.py function rosters (complete) | 3.8 |  | 0.688 |
| walker |  | 6583 | 203 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 7, sub: 2, line: 53 } |  |  | 0.707 |
| ns | 6741 |  | 171 | views/access.py — the complete authorization helper set | 3.9 |  | 0.701 |
| walker |  | 6759 | 176 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 20, sub: 0, line: 183 } |  |  | 0.701 |
| ns | 7007 |  | 266 | Remaining view modules: turbo, assets, auth, root, health, manifest, opensearch, custom_css, toasts, reload | 3.10 |  | 0.689 |
| walker |  | 7046 | 287 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 20, sub: 1, line: 183 } |  |  | 0.706 |
| ns | 7095 |  | 88 | forms.py class roster (complete) | 3.11 |  | 0.700 |
| walker |  | 7254 | 208 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 32, sub: 0, line: 346 } |  |  | 0.700 |
| ns | 7309 |  | 214 | Request-scoped plumbing: typed HttpRequest + LinkdingMiddleware | 3.12 |  | 0.689 |
| ns | 7329 |  | 20 | bookmarks/templatetags/ listing (complete) | 3.13 |  | 0.691 |
| walker |  | 7433 | 179 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 32, sub: 1, line: 346 } |  |  | 0.691 |
| ns | 7563 |  | 234 | api/routes.py — viewsets and router registrations (complete) | 4.1 |  | 0.676 |
| walker |  | 7607 | 174 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 32, sub: 2, line: 346 } |  |  | 0.676 |
| ns | 7779 |  | 216 | BookmarkViewSet — every custom action and override | 4.2 | 4.1 | 0.668 |
| walker |  | 7789 | 182 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 32, sub: 3, line: 346 } |  |  | 0.671 |
| ns | 7928 |  | 149 | Asset, user and bundle viewset actions | 4.3 | 4.1 | 0.666 |
| ns | 8017 |  | 89 | api/serializers.py class roster (complete) | 4.4 |  | 0.661 |
| walker |  | 8024 | 235 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 32, sub: 4, line: 346 } |  |  | 0.672 |
| ns | 8136 |  | 119 | API token authentication + docs/api.md section map | 4.5 |  | 0.667 |
| walker |  | 8205 | 181 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 32, sub: 5, line: 346 } |  |  | 0.674 |
| ns | 8365 |  | 229 | services/bookmarks.py function roster (complete) | 5.1 |  | 0.664 |
| walker |  | 8377 | 172 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 32, sub: 6, line: 346 } |  |  | 0.671 |
| ns | 8566 |  | 201 | queries.py function roster (complete) | 5.2 |  | 0.663 |
| walker |  | 8602 | 225 | Code::CodeKey { rung: Decl, file: bookmarks/models.py, decl: 32, sub: 7, line: 346 } |  |  | 0.681 |
| walker |  | 8713 | 111 | Code::CodeKey { rung: Names, file: bookmarks/forms.py, decl: 0, sub: 0, line: 0 } |  |  | 0.689 |
| walker |  | 8795 | 82 | Code::CodeKey { rung: Decl, file: bookmarks/forms.py, decl: 12, sub: 0, line: 161 } |  |  | 0.689 |
| ns | 8816 |  | 250 | services/search_query_parser.py symbol roster (complete) | 5.3 |  | 0.678 |
| walker |  | 8892 | 97 | Code::CodeKey { rung: Decl, file: bookmarks/forms.py, decl: 8, sub: 0, line: 124 } |  |  | 0.678 |
| walker |  | 9007 | 115 | Code::CodeKey { rung: Decl, file: bookmarks/forms.py, decl: 21, sub: 0, line: 371 } |  |  | 0.678 |
| ns | 9169 |  | 353 | services/tasks.py function roster (complete) | 5.4 |  | 0.665 |
| walker |  | 9409 | 402 | Code::CodeKey { rung: Decl, file: bookmarks/forms.py, decl: 1, sub: 0, line: 31 } |  |  | 0.665 |
| ns | 9438 |  | 269 | settings/base.py — deployment, auth and database LD_* options | 6.1 |  | 0.658 |
| walker |  | 9736 | 327 | Code::CodeKey { rung: Decl, file: bookmarks/forms.py, decl: 16, sub: 0, line: 215 } |  |  | 0.658 |
| ns | 9790 |  | 352 | settings/base.py — favicon, preview, snapshot and singlefile LD_* options | 6.2 |  | 0.649 |
| ns | 9898 |  | 108 | Operational surfaces: docker/, scripts/ and management commands (complete listings) | 6.3 |  | 0.656 |
| walker |  | 9917 | 181 | Code::CodeKey { rung: Names, file: bookmarks/utils.py, decl: 0, sub: 0, line: 0 } |  |  | 0.656 |
| walker |  | 9930 | 13 | Code::CodeKey { rung: Decl, file: bookmarks/utils.py, decl: 4, sub: 0, line: 43 } |  |  | 0.656 |
| walker |  | 9947 | 17 | Code::CodeKey { rung: Decl, file: bookmarks/utils.py, decl: 5, sub: 0, line: 63 } |  |  | 0.656 |
| walker |  | 9964 | 17 | Code::CodeKey { rung: Decl, file: bookmarks/utils.py, decl: 6, sub: 0, line: 83 } |  |  | 0.656 |
| ns | 9985 |  | 87 | bookmarks/frontend/ component and utility listings (complete) | 7.1 |  | 0.663 |
| walker |  | 9989 | 25 | Code::CodeKey { rung: Decl, file: bookmarks/utils.py, decl: 3, sub: 0, line: 36 } |  |  | 0.663 |

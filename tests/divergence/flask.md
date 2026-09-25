Score(3000)=0.600 I=0.852 C=0.423 ns_rows≤3K=18/57 grid(1000/1442/2080/3000/4327/6240/9000)=0.672/0.793/0.678/0.600/0.621/0.518/0.517

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 50 | 50 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 53 | 3 | Fs::DirListing { dir: src } |  |  | 0.000 |
| ns | 97 |  | 97 | README title + what Flask is (README.md:3, 5-9) | 1.1 |  | 0.000 |
| walker |  | 140 | 87 | Toml::Identity { file: pyproject.toml } |  |  | 0.000 |
| ns | 147 |  | 50 | Repository root listing (complete) | 1.2 |  | 0.488 |
| walker |  | 149 | 9 | Fs::DirListing { dir: examples } |  |  | 0.488 |
| walker |  | 160 | 11 | Fs::DirListing { dir: .devcontainer } |  |  | 0.489 |
| walker |  | 185 | 25 | Toml::Operational { file: pyproject.toml } |  |  | 0.489 |
| ns | 271 |  | 124 | Complete source-package listing: src/flask, src/flask/json, src/flask/sansio | 1.3 |  | 0.289 |
| walker |  | 278 | 93 | Fs::DirListing { dir: src/flask } |  |  | 0.492 |
| walker |  | 292 | 14 | Fs::DirListing { dir: src/flask/json } |  |  | 0.549 |
| walker |  | 309 | 17 | Fs::DirListing { dir: src/flask/sansio } |  |  | 0.634 |
| walker |  | 322 | 13 | Fs::DirListing { dir: .github } |  |  | 0.635 |
| walker |  | 345 | 23 | Fs::DirListing { dir: .github/workflows } |  |  | 0.637 |
| ns | 395 |  | 124 | README positioning: no enforced dependencies/layout, extension ecosystem (README.md:11-18) | 1.4 |  | 0.581 |
| walker |  | 447 | 102 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.916 |
| walker |  | 477 | 30 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.916 |
| ns | 541 |  | 146 | Canonical usage example: @app.route + `flask run` (README.md:20-36) | 1.5 |  | 0.784 |
| walker |  | 653 | 176 | Fs::DirListing { dir: docs } |  |  | 0.788 |
| walker |  | 680 | 27 | Fs::DirListing { dir: docs/_static } |  |  | 0.788 |
| ns | 707 |  | 166 | Public export surface 1/3: Flask, Blueprint, Config, json, context fns, globals (src/flask/__init__.py:1-12) | 1.6 |  | 0.720 |
| walker |  | 744 | 64 | Fs::DirListing { dir: docs/deploying } |  |  | 0.720 |
| walker |  | 782 | 38 | Toml::PackageMetadata { file: pyproject.toml } |  |  | 0.722 |
| walker |  | 860 | 78 | Fs::DirListing { dir: docs/tutorial } |  |  | 0.722 |
| ns | 865 |  | 158 | Public export surface 2/3: the `helpers` free functions + jsonify (src/flask/__init__.py:13-23) | 1.7 | 1.6 | 0.672 |
| walker |  | 1100 | 240 | Code::CodeKey { rung: Names, file: src/flask/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.778 |
| ns | 1121 |  | 256 | Public export surface 3/3: all ten signals, template renderers, Request/Response (src/flask/__init__.py:24-39) | 1.8 | 1.7 | 0.710 |
| walker |  | 1245 | 145 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.720 |
| walker |  | 1367 | 122 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.775 |
| walker |  | 1382 | 15 | Code::CodeKey { rung: Names, file: src/flask/blueprints.py, decl: 0, sub: 0, line: 0 } |  |  | 0.775 |
| ns | 1391 |  | 270 | Package identity + runtime dependency stack (pyproject.toml:1-8, 22-34) | 1.9 |  | 0.793 |
| ns | 1468 |  | 77 | Why src/flask/sansio/ exists (src/flask/sansio/README.md:1-6) | 2.1 |  | 0.773 |
| walker |  | 1527 | 145 | Fs::DirListing { dir: docs/patterns } |  |  | 0.773 |
| ns | 1545 |  | 77 | The class spine: Scaffold -> App -> Flask and Scaffold -> Blueprint -> Blueprint (6 class statements) | 2.2 |  | 0.756 |
| ns | 1639 |  | 94 | Scaffold routing decorators — every URL-registration method, names only (sansio/scaffold.py:284-436) | 2.3 | 2.2 | 0.730 |
| walker |  | 1676 | 149 | Code::CodeKey { rung: Names, file: src/flask/json/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.730 |
| walker |  | 1701 | 25 | Code::CodeKey { rung: Body, file: src/flask/json/__init__.py, decl: 5, sub: 0, line: 138 } |  |  | 0.730 |
| walker |  | 1744 | 43 | Code::CodeKey { rung: Body, file: src/flask/json/__init__.py, decl: 3, sub: 0, line: 77 } |  |  | 0.730 |
| ns | 1745 |  | 106 | Scaffold request-hook and error-handler decorators, names only (sansio/scaffold.py:460-657) | 2.4 | 2.2 | 0.706 |
| walker |  | 1787 | 43 | Code::CodeKey { rung: Body, file: src/flask/json/__init__.py, decl: 4, sub: 0, line: 108 } |  |  | 0.706 |
| walker |  | 1843 | 56 | Code::CodeKey { rung: Body, file: src/flask/json/__init__.py, decl: 1, sub: 0, line: 13 } |  |  | 0.706 |
| walker |  | 1867 | 24 | Fs::DirListing { dir: examples/celery } |  |  | 0.706 |
| walker |  | 1871 | 4 | Fs::DirListing { dir: examples/celery/src } |  |  | 0.706 |
| walker |  | 1888 | 17 | Fs::DirListing { dir: examples/celery/src/task_app } |  |  | 0.706 |
| ns | 1896 |  | 151 | Rest of sansio/scaffold.py: constructor, static/template properties, module-level helpers | 2.5 | 2.2 | 0.675 |
| walker |  | 2033 | 145 | Fs::DirListing { dir: tests } |  |  | 0.678 |
| walker |  | 2052 | 19 | Fs::DirListing { dir: tests/type_check } |  |  | 0.678 |
| walker |  | 2112 | 60 | Code::CodeKey { rung: Body, file: src/flask/json/__init__.py, decl: 2, sub: 0, line: 47 } |  |  | 0.644 |
| ns | 2112 |  | 216 | The complete signal registry (src/flask/signals.py:1-16, whole file) | 2.6 |  | 0.644 |
| walker |  | 2178 | 66 | Code::CodeKey { rung: Decl, file: src/flask/blueprints.py, decl: 1, sub: 0, line: 18 } |  |  | 0.644 |
| walker |  | 2205 | 27 | Fs::DirListing { dir: examples/javascript } |  |  | 0.644 |
| walker |  | 2218 | 13 | Fs::DirListing { dir: examples/javascript/js_example } |  |  | 0.645 |
| walker |  | 2245 | 27 | Fs::DirListing { dir: examples/tutorial } |  |  | 0.645 |
| walker |  | 2273 | 28 | Fs::DirListing { dir: examples/tutorial/flaskr } |  |  | 0.645 |
| ns | 2294 |  | 182 | Class roster A: request/response, views, sessions, context, config (13 class statements) | 2.7 |  | 0.621 |
| walker |  | 2302 | 29 | Fs::DirListing { dir: tests/test_apps } |  |  | 0.621 |
| walker |  | 2311 | 9 | Fs::DirListing { dir: tests/test_apps/blueprintapp } |  |  | 0.621 |
| walker |  | 2320 | 9 | Fs::DirListing { dir: tests/test_apps/subdomaintestmodule } |  |  | 0.621 |
| walker |  | 2334 | 14 | Fs::DirListing { dir: tests/test_apps/blueprintapp/apps } |  |  | 0.621 |
| walker |  | 2343 | 9 | Fs::DirListing { dir: tests/test_apps/blueprintapp/apps/frontend } |  |  | 0.621 |
| walker |  | 2355 | 12 | Fs::DirListing { dir: tests/test_apps/blueprintapp/apps/admin } |  |  | 0.621 |
| walker |  | 2385 | 30 | Fs::DirListing { dir: examples/tutorial/tests } |  |  | 0.621 |
| walker |  | 2409 | 24 | Code::CodeKey { rung: Names, file: src/flask/wrappers.py, decl: 0, sub: 0, line: 0 } |  |  | 0.622 |
| walker |  | 2482 | 73 | Code::CodeKey { rung: Decl, file: src/flask/wrappers.py, decl: 13, sub: 0, line: 222 } |  |  | 0.622 |
| walker |  | 2488 | 6 | Code::CodeKey { rung: Decl, file: src/flask/wrappers.py, decl: 14, sub: 0, line: 246 } |  |  | 0.622 |
| ns | 2510 |  | 216 | Class roster B: templating, testing, CLI, debug helpers (14 class statements) | 2.8 |  | 0.598 |
| walker |  | 2530 | 42 | Code::CodeKey { rung: Decl, file: src/flask/blueprints.py, decl: 5, sub: 0, line: 104 } |  |  | 0.598 |
| walker |  | 2607 | 77 | Markdown::ReadmeHeadline { file: src/flask/sansio/README.md } |  |  | 0.619 |
| walker |  | 2642 | 35 | Fs::DirListing { dir: tests/test_apps/cliapp } |  |  | 0.619 |
| walker |  | 2652 | 10 | Fs::DirListing { dir: tests/test_apps/cliapp/inner1 } |  |  | 0.619 |
| ns | 2655 |  | 145 | Class roster C: the whole src/flask/json subpackage (12 class statements) | 2.9 |  | 0.600 |
| walker |  | 2664 | 12 | Fs::DirListing { dir: tests/test_apps/cliapp/inner1/inner2 } |  |  | 0.600 |
| walker |  | 3004 | 340 | Code::CodeKey { rung: Names, file: src/flask/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.687 |
| walker |  | 3079 | 75 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.687 |
| ns | 3085 |  | 430 | Flask.default_config — all 29 config keys with their defaults (src/flask/app.py:206-238) | 3.1 |  | 0.636 |
| walker |  | 3169 | 90 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.636 |
| ns | 3246 |  | 161 | Every Config loader method + the ConfigAttribute descriptor (src/flask/config.py:23-366) | 3.2 | 3.1 | 0.617 |
| walker |  | 3304 | 135 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.674 |
| walker |  | 3360 | 56 | Code::CodeKey { rung: Names, file: src/flask/config.py, decl: 0, sub: 0, line: 0 } |  |  | 0.676 |
| walker |  | 3490 | 130 | Code::CodeKey { rung: Decl, file: src/flask/config.py, decl: 2, sub: 0, line: 20 } |  |  | 0.680 |
| walker |  | 3498 | 8 | Code::CodeKey { rung: Decl, file: src/flask/config.py, decl: 5, sub: 0, line: 32 } |  |  | 0.680 |
| walker |  | 3508 | 10 | Code::CodeKey { rung: Decl, file: src/flask/config.py, decl: 4, sub: 0, line: 29 } |  |  | 0.680 |
| walker |  | 3520 | 12 | Code::CodeKey { rung: Doc, file: src/flask/config.py, decl: 2, sub: 0, line: 20 } |  |  | 0.680 |
| walker |  | 3557 | 37 | Code::CodeKey { rung: Decl, file: src/flask/config.py, decl: 3, sub: 0, line: 23 } |  |  | 0.680 |
| ns | 3588 |  | 342 | Every App/Flask class-level customization hook (sansio/app.py:164-277, app.py:242-252) | 3.3 | 3.1 | 0.652 |
| walker |  | 3690 | 133 | Code::CodeKey { rung: Decl, file: src/flask/config.py, decl: 8, sub: 0, line: 50 } |  |  | 0.683 |
| walker |  | 3721 | 31 | Code::CodeKey { rung: Decl, file: src/flask/config.py, decl: 12, sub: 0, line: 187 } |  |  | 0.683 |
| ns | 3756 |  | 168 | Config semantics: dict subclass, uppercase-keys-only rule (src/flask/config.py:51-56, 68-73) | 3.4 | 3.2 | 0.667 |
| walker |  | 3757 | 36 | Code::CodeKey { rung: Decl, file: src/flask/config.py, decl: 15, sub: 0, line: 304 } |  |  | 0.667 |
| walker |  | 3794 | 37 | Code::CodeKey { rung: Decl, file: src/flask/config.py, decl: 16, sub: 0, line: 323 } |  |  | 0.667 |
| walker |  | 3834 | 40 | Code::CodeKey { rung: Decl, file: src/flask/config.py, decl: 11, sub: 0, line: 126 } |  |  | 0.667 |
| walker |  | 3883 | 49 | Code::CodeKey { rung: Decl, file: src/flask/config.py, decl: 9, sub: 0, line: 94 } |  |  | 0.667 |
| ns | 3941 |  | 185 | Environment-variable config: from_prefixed_env behaviour (src/flask/config.py:129-141) | 3.5 | 3.2 | 0.651 |
| walker |  | 3958 | 75 | Code::CodeKey { rung: Decl, file: src/flask/config.py, decl: 14, sub: 0, line: 256 } |  |  | 0.651 |
| ns | 3993 |  | 52 | Concrete config-file samples used by the test suite (tests/static/config.json, tests/static/config.toml) | 3.6 |  | 0.644 |
| walker |  | 4019 | 61 | Code::CodeKey { rung: Names, file: src/flask/views.py, decl: 0, sub: 0, line: 0 } |  |  | 0.647 |
| walker |  | 4051 | 32 | Code::CodeKey { rung: Decl, file: src/flask/views.py, decl: 2, sub: 0, line: 11 } |  |  | 0.647 |
| ns | 4088 |  | 95 | Console entry point + build backend (pyproject.toml:81-89) | 4.1 |  | 0.638 |
| walker |  | 4099 | 48 | Code::CodeKey { rung: Decl, file: src/flask/views.py, decl: 6, sub: 0, line: 138 } |  |  | 0.638 |
| walker |  | 4142 | 43 | Code::CodeKey { rung: Names, file: src/flask/json/provider.py, decl: 0, sub: 0, line: 0 } |  |  | 0.639 |
| walker |  | 4219 | 77 | Code::CodeKey { rung: Names, file: src/flask/logging.py, decl: 0, sub: 0, line: 0 } |  |  | 0.639 |
| walker |  | 4227 | 8 | Code::CodeKey { rung: Decl, file: src/flask/logging.py, decl: 1, sub: 0, line: 15 } |  |  | 0.639 |
| ns | 4297 |  | 209 | How tests are run: tox env_list + the pytest command (pyproject.toml:170-182, 190-193) | 4.2 |  | 0.620 |
| walker |  | 4309 | 82 | Code::CodeKey { rung: Names, file: src/flask/testing.py, decl: 0, sub: 0, line: 0 } |  |  | 0.621 |
| walker |  | 4347 | 38 | Code::CodeKey { rung: Decl, file: src/flask/testing.py, decl: 1, sub: 0, line: 27 } |  |  | 0.621 |
| walker |  | 4389 | 42 | Code::CodeKey { rung: Decl, file: src/flask/testing.py, decl: 14, sub: 0, line: 265 } |  |  | 0.621 |
| walker |  | 4425 | 36 | Code::CodeKey { rung: Decl, file: src/flask/testing.py, decl: 16, sub: 0, line: 275 } |  |  | 0.621 |
| ns | 4509 |  | 212 | The style / typing / docs tox environments (pyproject.toml:233-250) | 4.3 | 4.2 | 0.603 |
| walker |  | 4557 | 132 | Code::CodeKey { rung: Decl, file: src/flask/testing.py, decl: 6, sub: 0, line: 109 } |  |  | 0.604 |
| walker |  | 4598 | 41 | Code::CodeKey { rung: Decl, file: src/flask/testing.py, decl: 8, sub: 0, line: 135 } |  |  | 0.604 |
| walker |  | 4652 | 54 | Code::CodeKey { rung: Decl, file: src/flask/testing.py, decl: 13, sub: 0, line: 255 } |  |  | 0.604 |
| ns | 4664 |  | 155 | Test + type-checking settings: pytest, mypy strict, pyright (pyproject.toml:106-110, 126-131, 142-145) | 4.4 |  | 0.589 |
| walker |  | 4715 | 63 | Code::CodeKey { rung: Decl, file: src/flask/testing.py, decl: 11, sub: 0, line: 204 } |  |  | 0.589 |
| walker |  | 4815 | 100 | Code::CodeKey { rung: Decl, file: src/flask/testing.py, decl: 2, sub: 0, line: 49 } |  |  | 0.589 |
| walker |  | 4848 | 33 | Code::CodeKey { rung: Decl, file: src/flask/testing.py, decl: 10, sub: 0, line: 193 } |  |  | 0.574 |
| ns | 4848 |  | 184 | The `flask` command object and its app-discovery help text (src/flask/cli.py:1110-1127) | 4.5 |  | 0.574 |
| walker |  | 4893 | 45 | Code::CodeKey { rung: Body, file: src/flask/logging.py, decl: 1, sub: 0, line: 15 } |  |  | 0.574 |
| walker |  | 4942 | 49 | Plaintext::DeclSurface { file: docs/reqcontext.rst } |  |  | 0.574 |
| ns | 4949 |  | 101 | The complete built-in command set: run, shell, routes (src/flask/cli.py:594-596, 882, 999, 1048) | 4.6 | 4.5 | 0.569 |
| walker |  | 4991 | 49 | Code::CodeKey { rung: Doc, file: src/flask/logging.py, decl: 2, sub: 0, line: 31 } |  |  | 0.569 |
| ns | 5097 |  | 148 | Every `flask run` flag, declaration lines only (src/flask/cli.py:883-925) | 4.7 | 4.6 | 0.562 |
| ns | 5241 |  | 144 | The --app option: module:name form and factory auto-detection (src/flask/cli.py:453-462) | 4.8 | 4.5 | 0.554 |
| walker |  | 5289 | 298 | Plaintext::Whole { file: docs/Makefile } |  |  | 0.554 |
| walker |  | 5460 | 171 | Code::CodeKey { rung: Decl, file: src/flask/json/provider.py, decl: 1, sub: 0, line: 19 } |  |  | 0.555 |
| walker |  | 5470 | 10 | Code::CodeKey { rung: Body, file: src/flask/json/provider.py, decl: 3, sub: 0, line: 41 } |  |  | 0.555 |
| walker |  | 5480 | 10 | Code::CodeKey { rung: Body, file: src/flask/json/provider.py, decl: 5, sub: 0, line: 59 } |  |  | 0.555 |
| ns | 5487 |  | 246 | The remaining global options: --debug/--no-debug, -e/--env-file, --version (src/flask/cli.py:485-489, 517-527, 283-286) | 4.9 | 4.8 | 0.541 |
| walker |  | 5494 | 14 | Code::CodeKey { rung: Body, file: src/flask/json/provider.py, decl: 2, sub: 0, line: 38 } |  |  | 0.541 |
| ns | 5659 |  | 172 | `flask routes` options + the injected --debug flag (src/flask/cli.py:1001, 1049-1060) | 4.10 | 4.6 | 0.532 |
| ns | 5720 |  | 61 | CI and repo-automation inventory (.github, workflows, issue templates, .devcontainer — complete) | 4.11 |  | 0.536 |
| walker |  | 5743 | 249 | Code::CodeKey { rung: Names, file: src/flask/cli.py, decl: 0, sub: 0, line: 0 } |  |  | 0.538 |
| walker |  | 5769 | 26 | Code::CodeKey { rung: Decl, file: src/flask/cli.py, decl: 11, sub: 0, line: 293 } |  |  | 0.538 |
| walker |  | 5805 | 36 | Code::CodeKey { rung: Decl, file: src/flask/cli.py, decl: 8, sub: 0, line: 241 } |  |  | 0.538 |
| walker |  | 5853 | 48 | Code::CodeKey { rung: Decl, file: src/flask/cli.py, decl: 7, sub: 0, line: 235 } |  |  | 0.538 |
| walker |  | 5900 | 47 | Code::CodeKey { rung: Decl, file: src/flask/cli.py, decl: 6, sub: 0, line: 229 } |  |  | 0.538 |
| ns | 5919 |  | 199 | CI interpreter matrix (.github/workflows/tests.yaml:19-29) | 4.12 | 4.11 | 0.531 |
| walker |  | 5961 | 61 | Code::CodeKey { rung: Decl, file: src/flask/cli.py, decl: 10, sub: 0, line: 283 } |  |  | 0.532 |
| walker |  | 6038 | 77 | Code::CodeKey { rung: Decl, file: src/flask/cli.py, decl: 12, sub: 0, line: 305 } |  |  | 0.532 |
| walker |  | 6052 | 14 | Code::CodeKey { rung: Doc, file: src/flask/cli.py, decl: 1, sub: 0, line: 37 } |  |  | 0.532 |
| ns | 6063 |  | 144 | Flask methods 1/3: construction, resources, jinja, URL adapter, run (src/flask/app.py:254-632) | 5.1 |  | 0.525 |
| ns | 6206 |  | 143 | Flask methods 2/3: test clients, error handling, request dispatch (src/flask/app.py:755-1079) | 5.2 | 5.1 | 0.518 |
| walker |  | 6225 | 173 | Code::CodeKey { rung: Decl, file: src/flask/blueprints.py, decl: 2, sub: 0, line: 19 } |  |  | 0.518 |
| walker |  | 6331 | 106 | Code::CodeKey { rung: Names, file: src/flask/debughelpers.py, decl: 0, sub: 0, line: 0 } |  |  | 0.524 |
| ns | 6345 |  | 139 | Flask methods 3/3: url_for, make_response, teardown, contexts, WSGI entry (src/flask/app.py:1102-1618) | 5.3 | 5.2 | 0.518 |
| walker |  | 6351 | 20 | Code::CodeKey { rung: Decl, file: src/flask/debughelpers.py, decl: 5, sub: 0, line: 50 } |  |  | 0.518 |
| walker |  | 6391 | 40 | Code::CodeKey { rung: Decl, file: src/flask/debughelpers.py, decl: 2, sub: 0, line: 23 } |  |  | 0.518 |
| walker |  | 6398 | 7 | Code::CodeKey { rung: Body, file: src/flask/debughelpers.py, decl: 4, sub: 0, line: 46 } |  |  | 0.518 |
| walker |  | 6492 | 94 | Code::CodeKey { rung: Decl, file: src/flask/debughelpers.py, decl: 9, sub: 0, line: 124 } |  |  | 0.518 |
| walker |  | 6506 | 14 | Code::CodeKey { rung: Doc, file: src/flask/debughelpers.py, decl: 9, sub: 0, line: 124 } |  |  | 0.518 |
| ns | 6532 |  | 187 | App (sansio) methods 1/2: name, logger, jinja env, config/aborter factories, blueprint registration (sansio/app.py:279-605) | 5.4 |  | 0.509 |
| walker |  | 6539 | 33 | Code::CodeKey { rung: Decl, file: src/flask/json/provider.py, decl: 7, sub: 0, line: 75 } |  |  | 0.509 |
| walker |  | 6661 | 122 | Code::CodeKey { rung: Names, file: src/flask/sessions.py, decl: 0, sub: 0, line: 0 } |  |  | 0.522 |
| walker |  | 6729 | 68 | Code::CodeKey { rung: Decl, file: src/flask/sessions.py, decl: 6, sub: 0, line: 83 } |  |  | 0.522 |
| ns | 6751 |  | 219 | App (sansio) methods 2/2: template filter/test/global decorators, teardown, error routing (sansio/app.py:664-981) | 5.5 | 5.4 | 0.512 |
| walker |  | 6854 | 125 | Code::CodeKey { rung: Decl, file: src/flask/sessions.py, decl: 4, sub: 0, line: 57 } |  |  | 0.512 |
| walker |  | 6889 | 35 | Code::CodeKey { rung: Decl, file: src/flask/sessions.py, decl: 5, sub: 0, line: 73 } |  |  | 0.512 |
| walker |  | 6900 | 11 | Code::CodeKey { rung: Body, file: src/flask/sessions.py, decl: 23, sub: 0, line: 276 } |  |  | 0.512 |
| ns | 6936 |  | 185 | Every module-level function in src/flask/helpers.py (17 definitions) | 5.6 |  | 0.503 |
| ns | 7064 |  | 128 | Blueprint (sansio) methods 1/2: setup state, deferred registration, register (sansio/blueprints.py:41-413) | 5.7 |  | 0.498 |
| walker |  | 7168 | 268 | Code::CodeKey { rung: Decl, file: src/flask/sessions.py, decl: 24, sub: 0, line: 284 } |  |  | 0.499 |
| walker |  | 7197 | 29 | Code::CodeKey { rung: Decl, file: src/flask/sessions.py, decl: 27, sub: 0, line: 337 } |  |  | 0.499 |
| ns | 7297 |  | 233 | Blueprint (sansio) methods 2/2: the app_* decorators (sansio/blueprints.py:444-685) | 5.8 | 5.7 | 0.490 |
| walker |  | 7486 | 289 | Code::CodeKey { rung: Decl, file: src/flask/sessions.py, decl: 1, sub: 0, line: 24 } |  |  | 0.490 |
| ns | 7489 |  | 192 | The IO-side Blueprint overrides + the class-based view surface (blueprints.py, views.py) | 5.9 | 2.2 | 0.488 |
| walker |  | 7494 | 8 | Code::CodeKey { rung: Decl, file: src/flask/sessions.py, decl: 2, sub: 0, line: 27 } |  |  | 0.488 |
| walker |  | 7504 | 10 | Code::CodeKey { rung: Decl, file: src/flask/sessions.py, decl: 3, sub: 0, line: 32 } |  |  | 0.488 |
| walker |  | 7537 | 33 | Code::CodeKey { rung: Doc, file: src/flask/debughelpers.py, decl: 1, sub: 0, line: 17 } |  |  | 0.488 |
| ns | 7742 |  | 253 | Request/Response attributes and properties (src/flask/wrappers.py) | 5.10 | 2.7 | 0.481 |
| walker |  | 7825 | 288 | Code::CodeKey { rung: Doc, file: src/flask/json/__init__.py, decl: 2, sub: 0, line: 47 } |  |  | 0.481 |
| walker |  | 7838 | 13 | Fs::DirListing { dir: tests/static } |  |  | 0.481 |
| walker |  | 7842 | 4 | Fs::DirListing { dir: examples/tutorial/flaskr/static } |  |  | 0.481 |
| walker |  | 7846 | 4 | Fs::DirListing { dir: tests/test_apps/subdomaintestmodule/static } |  |  | 0.481 |
| ns | 8041 |  | 299 | The session interface: SessionMixin/SecureCookieSession state + every SessionInterface method (sessions.py:28-263) | 5.11 | 2.7 | 0.475 |
| walker |  | 8121 | 275 | Code::CodeKey { rung: Decl, file: src/flask/views.py, decl: 3, sub: 0, line: 16 } |  |  | 0.482 |
| walker |  | 8134 | 13 | Code::CodeKey { rung: Doc, file: src/flask/sessions.py, decl: 1, sub: 0, line: 24 } |  |  | 0.482 |
| ns | 8152 |  | 111 | The default cookie-signing session implementation (sessions.py:276-337) | 5.12 | 5.11 | 0.490 |
| ns | 8367 |  | 215 | The JSON subpackage API: module functions, provider methods, provider knobs | 5.13 | 2.9 | 0.487 |
| walker |  | 8373 | 239 | Code::CodeKey { rung: Decl, file: src/flask/sessions.py, decl: 8, sub: 0, line: 100 } |  |  | 0.494 |
| walker |  | 8387 | 14 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.500 |
| ns | 8509 |  | 142 | The test-support API: EnvironBuilder, FlaskClient, FlaskCliRunner members (src/flask/testing.py) | 5.14 | 2.8 | 0.509 |
| walker |  | 8537 | 150 | Code::CodeKey { rung: Names, file: src/flask/ctx.py, decl: 0, sub: 0, line: 0 } |  |  | 0.513 |
| walker |  | 8564 | 27 | Code::CodeKey { rung: Decl, file: src/flask/ctx.py, decl: 12, sub: 0, line: 118 } |  |  | 0.513 |
| walker |  | 8580 | 16 | Code::CodeKey { rung: Body, file: src/flask/ctx.py, decl: 16, sub: 0, line: 235 } |  |  | 0.513 |
| walker |  | 8603 | 23 | Code::CodeKey { rung: Body, file: src/flask/ctx.py, decl: 15, sub: 0, line: 209 } |  |  | 0.513 |
| ns | 8737 |  | 228 | The context proxies: how current_app, g, request, session are bound (src/flask/globals.py:40-49, 57-62) | 6.1 | 1.6 | 0.507 |
| walker |  | 8819 | 216 | Code::CodeKey { rung: Decl, file: src/flask/ctx.py, decl: 17, sub: 0, line: 260 } |  |  | 0.508 |
| walker |  | 8827 | 8 | Code::CodeKey { rung: Decl, file: src/flask/ctx.py, decl: 19, sub: 0, line: 339 } |  |  | 0.508 |
| walker |  | 8835 | 8 | Code::CodeKey { rung: Decl, file: src/flask/ctx.py, decl: 20, sub: 0, line: 350 } |  |  | 0.508 |
| walker |  | 8843 | 8 | Code::CodeKey { rung: Decl, file: src/flask/ctx.py, decl: 22, sub: 0, line: 370 } |  |  | 0.508 |
| walker |  | 8851 | 8 | Code::CodeKey { rung: Decl, file: src/flask/ctx.py, decl: 24, sub: 0, line: 395 } |  |  | 0.508 |
| walker |  | 8909 | 58 | Code::CodeKey { rung: Decl, file: src/flask/ctx.py, decl: 18, sub: 0, line: 300 } |  |  | 0.508 |
| walker |  | 8967 | 58 | Code::CodeKey { rung: Decl, file: src/flask/ctx.py, decl: 29, sub: 0, line: 510 } |  |  | 0.517 |
| ns | 8967 |  | 230 | The context API: ctx.py module functions, the `g` object, AppContext members | 6.2 | 2.7 | 0.517 |
| walker |  | 9122 | 155 | Code::CodeKey { rung: Names, file: src/flask/templating.py, decl: 0, sub: 0, line: 0 } |  |  | 0.520 |
| walker |  | 9148 | 26 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 2, sub: 0, line: 36 } |  |  | 0.520 |
| walker |  | 9186 | 38 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 12, sub: 0, line: 136 } |  |  | 0.520 |
| ns | 9223 |  | 256 | What changed in the unreleased 3.2.0 (CHANGES.rst:1-16) | 6.3 |  | 0.513 |
| walker |  | 9226 | 40 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 15, sub: 0, line: 181 } |  |  | 0.513 |
| walker |  | 9331 | 105 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 4, sub: 0, line: 49 } |  |  | 0.513 |
| walker |  | 9371 | 40 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 6, sub: 0, line: 57 } |  |  | 0.513 |
| ns | 9399 |  | 176 | Complete docs/ listing (35 entries) | 6.4 |  | 0.535 |
| walker |  | 9403 | 32 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 14, sub: 0, line: 163 } |  |  | 0.535 |
| walker |  | 9443 | 40 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 7, sub: 0, line: 64 } |  |  | 0.535 |
| walker |  | 9483 | 40 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 8, sub: 0, line: 88 } |  |  | 0.535 |
| walker |  | 9491 | 8 | Code::CodeKey { rung: Body, file: src/flask/templating.py, decl: 5, sub: 0, line: 54 } |  |  | 0.535 |
| ns | 9544 |  | 145 | Complete tests/ listing (27 entries) | 6.5 |  | 0.549 |
| walker |  | 9574 | 83 | Code::CodeKey { rung: Body, file: src/flask/logging.py, decl: 4, sub: 0, line: 58 } |  |  | 0.549 |
| ns | 9611 |  | 67 | The three runnable example applications (examples/ and each app package) | 6.6 |  | 0.557 |
| walker |  | 9625 | 51 | Plaintext::DeclSurface { file: docs/patterns/jquery.rst } |  |  | 0.557 |
| walker |  | 9916 | 291 | Code::CodeKey { rung: Names, file: src/flask/helpers.py, decl: 0, sub: 0, line: 0 } |  |  | 0.574 |
| walker |  | 9923 | 7 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 17, sub: 0, line: 644 } |  |  | 0.574 |
| walker |  | 9960 | 37 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 3, sub: 0, line: 51 } |  |  | 0.574 |
| ns | 9979 |  | 368 | The canonical application factory (examples/tutorial/flaskr/__init__.py:1-48, comments elided) | 6.7 | 6.6 | 0.561 |
| walker |  | 9997 | 37 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 8, sub: 0, line: 254 } |  |  | 0.561 |

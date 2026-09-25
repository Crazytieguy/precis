Score(3000)=0.685 I=0.892 C=0.526 ns_rows≤3K=18/57 grid(1000/1442/2080/3000/4327/6240/9000)=0.672/0.716/0.702/0.685/0.581/0.495/0.465

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
| walker |  | 1005 | 145 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.681 |
| ns | 1121 |  | 256 | Public export surface 3/3: all ten signals, template renderers, Request/Response (src/flask/__init__.py:24-39) | 1.8 | 1.7 | 0.622 |
| walker |  | 1127 | 122 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.681 |
| walker |  | 1272 | 145 | Fs::DirListing { dir: docs/patterns } |  |  | 0.681 |
| walker |  | 1296 | 24 | Fs::DirListing { dir: examples/celery } |  |  | 0.681 |
| walker |  | 1300 | 4 | Fs::DirListing { dir: examples/celery/src } |  |  | 0.681 |
| walker |  | 1317 | 17 | Fs::DirListing { dir: examples/celery/src/task_app } |  |  | 0.681 |
| ns | 1391 |  | 270 | Package identity + runtime dependency stack (pyproject.toml:1-8, 22-34) | 1.9 |  | 0.716 |
| walker |  | 1462 | 145 | Fs::DirListing { dir: tests } |  |  | 0.719 |
| ns | 1468 |  | 77 | Why src/flask/sansio/ exists (src/flask/sansio/README.md:1-6) | 2.1 |  | 0.701 |
| walker |  | 1481 | 19 | Fs::DirListing { dir: tests/type_check } |  |  | 0.701 |
| walker |  | 1508 | 27 | Fs::DirListing { dir: examples/javascript } |  |  | 0.701 |
| walker |  | 1521 | 13 | Fs::DirListing { dir: examples/javascript/js_example } |  |  | 0.701 |
| ns | 1545 |  | 77 | The class spine: Scaffold -> App -> Flask and Scaffold -> Blueprint -> Blueprint (6 class statements) | 2.2 |  | 0.684 |
| walker |  | 1548 | 27 | Fs::DirListing { dir: examples/tutorial } |  |  | 0.684 |
| walker |  | 1576 | 28 | Fs::DirListing { dir: examples/tutorial/flaskr } |  |  | 0.685 |
| walker |  | 1605 | 29 | Fs::DirListing { dir: tests/test_apps } |  |  | 0.685 |
| walker |  | 1614 | 9 | Fs::DirListing { dir: tests/test_apps/blueprintapp } |  |  | 0.685 |
| walker |  | 1623 | 9 | Fs::DirListing { dir: tests/test_apps/subdomaintestmodule } |  |  | 0.685 |
| walker |  | 1637 | 14 | Fs::DirListing { dir: tests/test_apps/blueprintapp/apps } |  |  | 0.685 |
| ns | 1639 |  | 94 | Scaffold routing decorators — every URL-registration method, names only (sansio/scaffold.py:284-436) | 2.3 | 2.2 | 0.661 |
| walker |  | 1646 | 9 | Fs::DirListing { dir: tests/test_apps/blueprintapp/apps/frontend } |  |  | 0.661 |
| walker |  | 1658 | 12 | Fs::DirListing { dir: tests/test_apps/blueprintapp/apps/admin } |  |  | 0.661 |
| walker |  | 1688 | 30 | Fs::DirListing { dir: examples/tutorial/tests } |  |  | 0.661 |
| ns | 1745 |  | 106 | Scaffold request-hook and error-handler decorators, names only (sansio/scaffold.py:460-657) | 2.4 | 2.2 | 0.640 |
| walker |  | 1765 | 77 | Markdown::ReadmeHeadline { file: src/flask/sansio/README.md } |  |  | 0.667 |
| walker |  | 1800 | 35 | Fs::DirListing { dir: tests/test_apps/cliapp } |  |  | 0.667 |
| walker |  | 1810 | 10 | Fs::DirListing { dir: tests/test_apps/cliapp/inner1 } |  |  | 0.667 |
| walker |  | 1822 | 12 | Fs::DirListing { dir: tests/test_apps/cliapp/inner1/inner2 } |  |  | 0.667 |
| ns | 1896 |  | 151 | Rest of sansio/scaffold.py: constructor, static/template properties, module-level helpers | 2.5 | 2.2 | 0.638 |
| walker |  | 2062 | 240 | Code::CodeKey { rung: Names, file: src/flask/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.702 |
| ns | 2112 |  | 216 | The complete signal registry (src/flask/signals.py:1-16, whole file) | 2.6 |  | 0.668 |
| ns | 2294 |  | 182 | Class roster A: request/response, views, sessions, context, config (13 class statements) | 2.7 |  | 0.643 |
| walker |  | 2402 | 340 | Code::CodeKey { rung: Names, file: src/flask/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.736 |
| walker |  | 2477 | 75 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.736 |
| ns | 2510 |  | 216 | Class roster B: templating, testing, CLI, debug helpers (14 class statements) | 2.8 |  | 0.706 |
| walker |  | 2567 | 90 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.706 |
| ns | 2655 |  | 145 | Class roster C: the whole src/flask/json subpackage (12 class statements) | 2.9 |  | 0.685 |
| walker |  | 2716 | 149 | Code::CodeKey { rung: Names, file: src/flask/json/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.685 |
| walker |  | 2741 | 25 | Code::CodeKey { rung: Body, file: src/flask/json/__init__.py, decl: 5, sub: 0, line: 138 } |  |  | 0.685 |
| walker |  | 2784 | 43 | Code::CodeKey { rung: Body, file: src/flask/json/__init__.py, decl: 3, sub: 0, line: 77 } |  |  | 0.685 |
| walker |  | 2827 | 43 | Code::CodeKey { rung: Body, file: src/flask/json/__init__.py, decl: 4, sub: 0, line: 108 } |  |  | 0.685 |
| walker |  | 2883 | 56 | Code::CodeKey { rung: Body, file: src/flask/json/__init__.py, decl: 1, sub: 0, line: 13 } |  |  | 0.685 |
| walker |  | 2943 | 60 | Code::CodeKey { rung: Body, file: src/flask/json/__init__.py, decl: 2, sub: 0, line: 47 } |  |  | 0.685 |
| walker |  | 3078 | 135 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.748 |
| ns | 3085 |  | 430 | Flask.default_config — all 29 config keys with their defaults (src/flask/app.py:206-238) | 3.1 |  | 0.692 |
| walker |  | 3127 | 49 | Plaintext::DeclSurface { file: docs/reqcontext.rst } |  |  | 0.692 |
| ns | 3246 |  | 161 | Every Config loader method + the ConfigAttribute descriptor (src/flask/config.py:23-366) | 3.2 | 3.1 | 0.672 |
| walker |  | 3425 | 298 | Plaintext::Whole { file: docs/Makefile } |  |  | 0.672 |
| ns | 3588 |  | 342 | Every App/Flask class-level customization hook (sansio/app.py:164-277, app.py:242-252) | 3.3 | 3.1 | 0.645 |
| walker |  | 3713 | 288 | Code::CodeKey { rung: Doc, file: src/flask/json/__init__.py, decl: 2, sub: 0, line: 47 } |  |  | 0.645 |
| ns | 3756 |  | 168 | Config semantics: dict subclass, uppercase-keys-only rule (src/flask/config.py:51-56, 68-73) | 3.4 | 3.2 | 0.629 |
| ns | 3941 |  | 185 | Environment-variable config: from_prefixed_env behaviour (src/flask/config.py:129-141) | 3.5 | 3.2 | 0.614 |
| ns | 3993 |  | 52 | Concrete config-file samples used by the test suite (tests/static/config.json, tests/static/config.toml) | 3.6 |  | 0.608 |
| walker |  | 4015 | 302 | Code::CodeKey { rung: Doc, file: src/flask/json/__init__.py, decl: 3, sub: 0, line: 77 } |  |  | 0.608 |
| ns | 4088 |  | 95 | Console entry point + build backend (pyproject.toml:81-89) | 4.1 |  | 0.600 |
| ns | 4297 |  | 209 | How tests are run: tox env_list + the pytest command (pyproject.toml:170-182, 190-193) | 4.2 |  | 0.581 |
| walker |  | 4320 | 305 | Code::CodeKey { rung: Doc, file: src/flask/json/__init__.py, decl: 4, sub: 0, line: 108 } |  |  | 0.581 |
| ns | 4509 |  | 212 | The style / typing / docs tox environments (pyproject.toml:233-250) | 4.3 | 4.2 | 0.565 |
| walker |  | 4643 | 323 | Code::CodeKey { rung: Doc, file: src/flask/json/__init__.py, decl: 1, sub: 0, line: 13 } |  |  | 0.565 |
| ns | 4664 |  | 155 | Test + type-checking settings: pytest, mypy strict, pyright (pyproject.toml:106-110, 126-131, 142-145) | 4.4 |  | 0.551 |
| walker |  | 4667 | 24 | Code::CodeKey { rung: Names, file: src/flask/wrappers.py, decl: 0, sub: 0, line: 0 } |  |  | 0.551 |
| walker |  | 4740 | 73 | Code::CodeKey { rung: Decl, file: src/flask/wrappers.py, decl: 13, sub: 0, line: 222 } |  |  | 0.551 |
| walker |  | 4746 | 6 | Code::CodeKey { rung: Decl, file: src/flask/wrappers.py, decl: 14, sub: 0, line: 246 } |  |  | 0.551 |
| walker |  | 4809 | 63 | Code::CodeKey { rung: Doc, file: src/flask/wrappers.py, decl: 14, sub: 0, line: 246 } |  |  | 0.551 |
| ns | 4848 |  | 184 | The `flask` command object and its app-discovery help text (src/flask/cli.py:1110-1127) | 4.5 |  | 0.537 |
| walker |  | 4874 | 65 | Code::CodeKey { rung: Body, file: src/flask/wrappers.py, decl: 14, sub: 0, line: 246 } |  |  | 0.537 |
| walker |  | 4887 | 13 | Fs::DirListing { dir: tests/static } |  |  | 0.537 |
| walker |  | 4891 | 4 | Fs::DirListing { dir: examples/tutorial/flaskr/static } |  |  | 0.537 |
| walker |  | 4895 | 4 | Fs::DirListing { dir: tests/test_apps/subdomaintestmodule/static } |  |  | 0.537 |
| walker |  | 4909 | 14 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.538 |
| walker |  | 4924 | 15 | Code::CodeKey { rung: Names, file: src/flask/blueprints.py, decl: 0, sub: 0, line: 0 } |  |  | 0.538 |
| ns | 4949 |  | 101 | The complete built-in command set: run, shell, routes (src/flask/cli.py:594-596, 882, 999, 1048) | 4.6 | 4.5 | 0.534 |
| walker |  | 4990 | 66 | Code::CodeKey { rung: Decl, file: src/flask/blueprints.py, decl: 1, sub: 0, line: 18 } |  |  | 0.534 |
| walker |  | 5032 | 42 | Code::CodeKey { rung: Decl, file: src/flask/blueprints.py, decl: 5, sub: 0, line: 104 } |  |  | 0.534 |
| ns | 5097 |  | 148 | Every `flask run` flag, declaration lines only (src/flask/cli.py:883-925) | 4.7 | 4.6 | 0.528 |
| walker |  | 5205 | 173 | Code::CodeKey { rung: Decl, file: src/flask/blueprints.py, decl: 2, sub: 0, line: 19 } |  |  | 0.528 |
| ns | 5241 |  | 144 | The --app option: module:name form and factory auto-detection (src/flask/cli.py:453-462) | 4.8 | 4.5 | 0.520 |
| ns | 5487 |  | 246 | The remaining global options: --debug/--no-debug, -e/--env-file, --version (src/flask/cli.py:485-489, 517-527, 283-286) | 4.9 | 4.8 | 0.507 |
| walker |  | 5619 | 414 | Code::CodeKey { rung: Doc, file: src/flask/json/__init__.py, decl: 5, sub: 0, line: 138 } |  |  | 0.507 |
| ns | 5659 |  | 172 | `flask routes` options + the injected --debug flag (src/flask/cli.py:1001, 1049-1060) | 4.10 | 4.6 | 0.499 |
| walker |  | 5680 | 61 | Code::CodeKey { rung: Names, file: src/flask/views.py, decl: 0, sub: 0, line: 0 } |  |  | 0.500 |
| walker |  | 5712 | 32 | Code::CodeKey { rung: Decl, file: src/flask/views.py, decl: 2, sub: 0, line: 11 } |  |  | 0.500 |
| ns | 5720 |  | 61 | CI and repo-automation inventory (.github, workflows, issue templates, .devcontainer — complete) | 4.11 |  | 0.515 |
| walker |  | 5760 | 48 | Code::CodeKey { rung: Decl, file: src/flask/views.py, decl: 6, sub: 0, line: 138 } |  |  | 0.515 |
| ns | 5919 |  | 199 | CI interpreter matrix (.github/workflows/tests.yaml:19-29) | 4.12 | 4.11 | 0.508 |
| walker |  | 6035 | 275 | Code::CodeKey { rung: Decl, file: src/flask/views.py, decl: 3, sub: 0, line: 16 } |  |  | 0.508 |
| ns | 6063 |  | 144 | Flask methods 1/3: construction, resources, jinja, URL adapter, run (src/flask/app.py:254-632) | 5.1 |  | 0.502 |
| ns | 6206 |  | 143 | Flask methods 2/3: test clients, error handling, request dispatch (src/flask/app.py:755-1079) | 5.2 | 5.1 | 0.495 |
| walker |  | 6240 | 205 | Code::CodeKey { rung: Decl, file: src/flask/views.py, decl: 3, sub: 1, line: 16 } |  |  | 0.495 |
| walker |  | 6284 | 44 | Code::CodeKey { rung: Decl, file: src/flask/views.py, decl: 5, sub: 0, line: 85 } |  |  | 0.495 |
| walker |  | 6294 | 10 | Code::CodeKey { rung: Body, file: src/flask/views.py, decl: 4, sub: 0, line: 78 } |  |  | 0.495 |
| walker |  | 6345 | 51 | Code::CodeKey { rung: Doc, file: src/flask/views.py, decl: 4, sub: 0, line: 78 } |  |  | 0.489 |
| ns | 6345 |  | 139 | Flask methods 3/3: url_for, make_response, teardown, contexts, WSGI entry (src/flask/app.py:1102-1618) | 5.3 | 5.2 | 0.489 |
| walker |  | 6435 | 90 | Code::CodeKey { rung: Body, file: src/flask/blueprints.py, decl: 3, sub: 0, line: 55 } |  |  | 0.489 |
| walker |  | 6486 | 51 | Plaintext::DeclSurface { file: docs/patterns/jquery.rst } |  |  | 0.489 |
| ns | 6532 |  | 187 | App (sansio) methods 1/2: name, logger, jinja env, config/aborter factories, blueprint registration (sansio/app.py:279-605) | 5.4 |  | 0.481 |
| ns | 6751 |  | 219 | App (sansio) methods 2/2: template filter/test/global decorators, teardown, error routing (sansio/app.py:664-981) | 5.5 | 5.4 | 0.472 |
| walker |  | 6777 | 291 | Code::CodeKey { rung: Names, file: src/flask/helpers.py, decl: 0, sub: 0, line: 0 } |  |  | 0.473 |
| walker |  | 6784 | 7 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 17, sub: 0, line: 644 } |  |  | 0.473 |
| walker |  | 6821 | 37 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 3, sub: 0, line: 51 } |  |  | 0.473 |
| walker |  | 6858 | 37 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 8, sub: 0, line: 254 } |  |  | 0.473 |
| walker |  | 6897 | 39 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 12, sub: 0, line: 360 } |  |  | 0.473 |
| ns | 6936 |  | 185 | Every module-level function in src/flask/helpers.py (17 definitions) | 5.6 |  | 0.490 |
| walker |  | 6945 | 48 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 4, sub: 0, line: 57 } |  |  | 0.490 |
| walker |  | 6994 | 49 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 15, sub: 0, line: 543 } |  |  | 0.490 |
| walker |  | 7049 | 55 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 5, sub: 0, line: 63 } |  |  | 0.490 |
| ns | 7064 |  | 128 | Blueprint (sansio) methods 1/2: setup state, deferred registration, register (sansio/blueprints.py:41-413) | 5.7 |  | 0.485 |
| walker |  | 7111 | 62 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 18, sub: 0, line: 654 } |  |  | 0.485 |
| walker |  | 7170 | 59 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 21, sub: 0, line: 665 } |  |  | 0.485 |
| walker |  | 7183 | 13 | Code::CodeKey { rung: Doc, file: src/flask/helpers.py, decl: 22, sub: 0, line: 676 } |  |  | 0.485 |
| walker |  | 7273 | 90 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 7, sub: 0, line: 200 } |  |  | 0.485 |
| ns | 7297 |  | 233 | Blueprint (sansio) methods 2/2: the app_* decorators (sansio/blueprints.py:444-685) | 5.8 | 5.7 | 0.476 |
| walker |  | 7416 | 143 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 14, sub: 0, line: 417 } |  |  | 0.476 |
| walker |  | 7458 | 42 | Code::CodeKey { rung: Doc, file: src/flask/helpers.py, decl: 18, sub: 0, line: 654 } |  |  | 0.476 |
| ns | 7489 |  | 192 | The IO-side Blueprint overrides + the class-based view surface (blueprints.py, views.py) | 5.9 | 2.2 | 0.488 |
| walker |  | 7508 | 50 | Code::CodeKey { rung: Doc, file: src/flask/helpers.py, decl: 1, sub: 0, line: 28 } |  |  | 0.488 |
| walker |  | 7590 | 82 | Code::CodeKey { rung: Doc, file: src/flask/helpers.py, decl: 2, sub: 0, line: 36 } |  |  | 0.488 |
| walker |  | 7681 | 91 | Code::CodeKey { rung: Doc, file: src/flask/helpers.py, decl: 16, sub: 0, line: 587 } |  |  | 0.488 |
| ns | 7742 |  | 253 | Request/Response attributes and properties (src/flask/wrappers.py) | 5.10 | 2.7 | 0.481 |
| walker |  | 7763 | 82 | Code::CodeKey { rung: Names, file: src/flask/testing.py, decl: 0, sub: 0, line: 0 } |  |  | 0.483 |
| walker |  | 7801 | 38 | Code::CodeKey { rung: Decl, file: src/flask/testing.py, decl: 1, sub: 0, line: 27 } |  |  | 0.483 |
| walker |  | 7843 | 42 | Code::CodeKey { rung: Decl, file: src/flask/testing.py, decl: 14, sub: 0, line: 265 } |  |  | 0.483 |
| walker |  | 7879 | 36 | Code::CodeKey { rung: Decl, file: src/flask/testing.py, decl: 16, sub: 0, line: 275 } |  |  | 0.483 |
| walker |  | 7894 | 15 | Code::CodeKey { rung: Body, file: src/flask/testing.py, decl: 3, sub: 0, line: 88 } |  |  | 0.483 |
| walker |  | 7994 | 100 | Code::CodeKey { rung: Decl, file: src/flask/testing.py, decl: 2, sub: 0, line: 49 } |  |  | 0.483 |
| ns | 8041 |  | 299 | The session interface: SessionMixin/SecureCookieSession state + every SessionInterface method (sessions.py:28-263) | 5.11 | 2.7 | 0.473 |
| walker |  | 8126 | 132 | Code::CodeKey { rung: Decl, file: src/flask/testing.py, decl: 6, sub: 0, line: 109 } |  |  | 0.473 |
| ns | 8152 |  | 111 | The default cookie-signing session implementation (sessions.py:276-337) | 5.12 | 5.11 | 0.470 |
| walker |  | 8159 | 33 | Code::CodeKey { rung: Decl, file: src/flask/testing.py, decl: 10, sub: 0, line: 193 } |  |  | 0.470 |
| walker |  | 8200 | 41 | Code::CodeKey { rung: Decl, file: src/flask/testing.py, decl: 8, sub: 0, line: 135 } |  |  | 0.470 |
| walker |  | 8254 | 54 | Code::CodeKey { rung: Decl, file: src/flask/testing.py, decl: 13, sub: 0, line: 255 } |  |  | 0.470 |
| walker |  | 8317 | 63 | Code::CodeKey { rung: Decl, file: src/flask/testing.py, decl: 11, sub: 0, line: 204 } |  |  | 0.470 |
| ns | 8367 |  | 215 | The JSON subpackage API: module functions, provider methods, provider knobs | 5.13 | 2.9 | 0.463 |
| walker |  | 8374 | 57 | Code::CodeKey { rung: Doc, file: src/flask/testing.py, decl: 3, sub: 0, line: 88 } |  |  | 0.463 |
| walker |  | 8438 | 64 | Code::CodeKey { rung: Doc, file: src/flask/testing.py, decl: 14, sub: 0, line: 265 } |  |  | 0.463 |
| ns | 8509 |  | 142 | The test-support API: EnvironBuilder, FlaskClient, FlaskCliRunner members (src/flask/testing.py) | 5.14 | 2.8 | 0.473 |
| walker |  | 8588 | 150 | Code::CodeKey { rung: Names, file: src/flask/ctx.py, decl: 0, sub: 0, line: 0 } |  |  | 0.475 |
| walker |  | 8615 | 27 | Code::CodeKey { rung: Decl, file: src/flask/ctx.py, decl: 12, sub: 0, line: 118 } |  |  | 0.475 |
| walker |  | 8631 | 16 | Code::CodeKey { rung: Body, file: src/flask/ctx.py, decl: 16, sub: 0, line: 235 } |  |  | 0.475 |
| walker |  | 8654 | 23 | Code::CodeKey { rung: Body, file: src/flask/ctx.py, decl: 15, sub: 0, line: 209 } |  |  | 0.475 |
| ns | 8737 |  | 228 | The context proxies: how current_app, g, request, session are bound (src/flask/globals.py:40-49, 57-62) | 6.1 | 1.6 | 0.469 |
| walker |  | 8860 | 206 | Code::CodeKey { rung: Decl, file: src/flask/ctx.py, decl: 2, sub: 0, line: 30 } |  |  | 0.469 |
| walker |  | 8869 | 9 | Code::CodeKey { rung: Body, file: src/flask/ctx.py, decl: 10, sub: 0, line: 108 } |  |  | 0.469 |
| walker |  | 8879 | 10 | Code::CodeKey { rung: Body, file: src/flask/ctx.py, decl: 9, sub: 0, line: 105 } |  |  | 0.469 |
| walker |  | 8891 | 12 | Code::CodeKey { rung: Body, file: src/flask/ctx.py, decl: 4, sub: 0, line: 59 } |  |  | 0.469 |
| ns | 8967 |  | 230 | The context API: ctx.py module functions, the `g` object, AppContext members | 6.2 | 2.7 | 0.465 |
| walker |  | 9107 | 216 | Code::CodeKey { rung: Decl, file: src/flask/ctx.py, decl: 17, sub: 0, line: 260 } |  |  | 0.486 |
| walker |  | 9115 | 8 | Code::CodeKey { rung: Decl, file: src/flask/ctx.py, decl: 19, sub: 0, line: 339 } |  |  | 0.486 |
| walker |  | 9123 | 8 | Code::CodeKey { rung: Decl, file: src/flask/ctx.py, decl: 20, sub: 0, line: 350 } |  |  | 0.486 |
| walker |  | 9131 | 8 | Code::CodeKey { rung: Decl, file: src/flask/ctx.py, decl: 22, sub: 0, line: 370 } |  |  | 0.486 |
| walker |  | 9139 | 8 | Code::CodeKey { rung: Decl, file: src/flask/ctx.py, decl: 24, sub: 0, line: 395 } |  |  | 0.486 |
| walker |  | 9197 | 58 | Code::CodeKey { rung: Decl, file: src/flask/ctx.py, decl: 18, sub: 0, line: 300 } |  |  | 0.486 |
| ns | 9223 |  | 256 | What changed in the unreleased 3.2.0 (CHANGES.rst:1-16) | 6.3 |  | 0.480 |
| walker |  | 9255 | 58 | Code::CodeKey { rung: Decl, file: src/flask/ctx.py, decl: 29, sub: 0, line: 510 } |  |  | 0.480 |
| walker |  | 9271 | 16 | Code::CodeKey { rung: Doc, file: src/flask/ctx.py, decl: 20, sub: 0, line: 350 } |  |  | 0.480 |
| walker |  | 9291 | 20 | Code::CodeKey { rung: Doc, file: src/flask/ctx.py, decl: 23, sub: 0, line: 381 } |  |  | 0.480 |
| walker |  | 9329 | 38 | Code::CodeKey { rung: Doc, file: src/flask/ctx.py, decl: 25, sub: 0, line: 405 } |  |  | 0.480 |
| walker |  | 9384 | 55 | Code::CodeKey { rung: Doc, file: src/flask/ctx.py, decl: 22, sub: 0, line: 370 } |  |  | 0.480 |
| ns | 9399 |  | 176 | Complete docs/ listing (35 entries) | 6.4 |  | 0.504 |
| walker |  | 9447 | 63 | Code::CodeKey { rung: Doc, file: src/flask/ctx.py, decl: 19, sub: 0, line: 339 } |  |  | 0.504 |
| walker |  | 9515 | 68 | Code::CodeKey { rung: Doc, file: src/flask/ctx.py, decl: 24, sub: 0, line: 395 } |  |  | 0.504 |
| ns | 9544 |  | 145 | Complete tests/ listing (27 entries) | 6.5 |  | 0.520 |
| walker |  | 9609 | 94 | Code::CodeKey { rung: Doc, file: src/flask/ctx.py, decl: 6, sub: 0, line: 68 } |  |  | 0.520 |
| ns | 9611 |  | 67 | The three runnable example applications (examples/ and each app package) | 6.6 |  | 0.529 |
| walker |  | 9707 | 98 | Code::CodeKey { rung: Doc, file: src/flask/ctx.py, decl: 21, sub: 0, line: 355 } |  |  | 0.529 |
| walker |  | 9806 | 99 | Code::CodeKey { rung: Doc, file: src/flask/ctx.py, decl: 7, sub: 0, line: 79 } |  |  | 0.529 |
| ns | 9979 |  | 368 | The canonical application factory (examples/tutorial/flaskr/__init__.py:1-48, comments elided) | 6.7 | 6.6 | 0.516 |
| walker |  | 9992 | 186 | Code::CodeKey { rung: Names, file: src/flask/signals.py, decl: 0, sub: 0, line: 0 } |  |  | 0.526 |

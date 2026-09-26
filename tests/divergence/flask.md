Score(3000)=0.689 I=0.905 C=0.525 ns_rows≤3K=18/57 grid(1000/1442/2080/3000/4327/6240/9000)=0.692/0.656/0.679/0.689/0.615/0.556/0.549

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 50 | 50 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 97 |  | 97 | README title + what Flask is (README.md:3, 5-9) | 1.1 |  | 0.000 |
| ns | 147 |  | 50 | Repository root listing (complete) | 1.2 |  | 0.484 |
| walker |  | 152 | 102 | Markdown::ReadmeHeadline { file: README.md } |  |  | 1.000 |
| walker |  | 247 | 95 | Fs::DirListing { dir: src/flask } |  |  | 1.000 |
| ns | 271 |  | 124 | Complete source-package listing: src/flask, src/flask/json, src/flask/sansio | 1.3 |  | 0.834 |
| walker |  | 334 | 87 | Toml::Identity { file: pyproject.toml } |  |  | 0.836 |
| walker |  | 348 | 14 | Fs::DirListing { dir: src/flask/json } |  |  | 0.903 |
| walker |  | 365 | 17 | Fs::DirListing { dir: src/flask/sansio } |  |  | 1.000 |
| walker |  | 395 | 30 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.914 |
| ns | 395 |  | 124 | README positioning: no enforced dependencies/layout, extension ecosystem (README.md:11-18) | 1.4 |  | 0.914 |
| walker |  | 404 | 9 | Fs::DirListing { dir: examples } |  |  | 0.914 |
| walker |  | 469 | 65 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.944 |
| walker |  | 494 | 25 | Toml::Operational { file: pyproject.toml } |  |  | 0.944 |
| walker |  | 505 | 11 | Fs::DirListing { dir: .devcontainer } |  |  | 0.944 |
| walker |  | 518 | 13 | Fs::DirListing { dir: .github } |  |  | 0.945 |
| walker |  | 541 | 23 | Fs::DirListing { dir: .github/workflows } |  |  | 0.808 |
| ns | 541 |  | 146 | Canonical usage example: @app.route + `flask run` (README.md:20-36) | 1.5 |  | 0.808 |
| ns | 707 |  | 166 | Public export surface 1/3: Flask, Blueprint, Config, json, context fns, globals (src/flask/__init__.py:1-12) | 1.6 |  | 0.739 |
| walker |  | 717 | 176 | Fs::DirListing { dir: docs } |  |  | 0.742 |
| walker |  | 744 | 27 | Fs::DirListing { dir: docs/_static } |  |  | 0.742 |
| walker |  | 808 | 64 | Fs::DirListing { dir: docs/deploying } |  |  | 0.742 |
| ns | 865 |  | 158 | Public export surface 2/3: the `helpers` free functions + jsonify (src/flask/__init__.py:13-23) | 1.7 | 1.6 | 0.691 |
| walker |  | 886 | 78 | Fs::DirListing { dir: docs/tutorial } |  |  | 0.691 |
| walker |  | 900 | 14 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.692 |
| walker |  | 1045 | 145 | Fs::DirListing { dir: docs/patterns } |  |  | 0.692 |
| walker |  | 1069 | 24 | Fs::DirListing { dir: examples/celery } |  |  | 0.692 |
| walker |  | 1088 | 19 | Fs::DirListing { dir: examples/celery/src/task_app } |  |  | 0.692 |
| ns | 1121 |  | 256 | Public export surface 3/3: all ten signals, template renderers, Request/Response (src/flask/__init__.py:24-39) | 1.8 | 1.7 | 0.632 |
| walker |  | 1233 | 145 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.639 |
| walker |  | 1378 | 145 | Fs::DirListing { dir: tests } |  |  | 0.641 |
| ns | 1391 |  | 270 | Package identity + runtime dependency stack (pyproject.toml:1-8, 22-34) | 1.9 |  | 0.656 |
| walker |  | 1397 | 19 | Fs::DirListing { dir: tests/type_check } |  |  | 0.656 |
| walker |  | 1424 | 27 | Fs::DirListing { dir: examples/javascript } |  |  | 0.656 |
| walker |  | 1436 | 12 | Fs::DirListing { dir: examples/javascript/tests } |  |  | 0.656 |
| walker |  | 1449 | 13 | Fs::DirListing { dir: examples/javascript/js_example } |  |  | 0.656 |
| ns | 1468 |  | 77 | Why src/flask/sansio/ exists (src/flask/sansio/README.md:1-6) | 2.1 |  | 0.640 |
| walker |  | 1476 | 27 | Fs::DirListing { dir: examples/tutorial } |  |  | 0.640 |
| walker |  | 1504 | 28 | Fs::DirListing { dir: examples/tutorial/flaskr } |  |  | 0.640 |
| walker |  | 1533 | 29 | Fs::DirListing { dir: tests/test_apps } |  |  | 0.640 |
| walker |  | 1542 | 9 | Fs::DirListing { dir: tests/test_apps/blueprintapp } |  |  | 0.640 |
| ns | 1545 |  | 77 | The class spine: Scaffold -> App -> Flask and Scaffold -> Blueprint -> Blueprint (6 class statements) | 2.2 |  | 0.625 |
| walker |  | 1551 | 9 | Fs::DirListing { dir: tests/test_apps/helloworld } |  |  | 0.625 |
| walker |  | 1560 | 9 | Fs::DirListing { dir: tests/test_apps/subdomaintestmodule } |  |  | 0.625 |
| walker |  | 1574 | 14 | Fs::DirListing { dir: tests/test_apps/blueprintapp/apps } |  |  | 0.625 |
| walker |  | 1583 | 9 | Fs::DirListing { dir: tests/test_apps/blueprintapp/apps/frontend } |  |  | 0.625 |
| walker |  | 1595 | 12 | Fs::DirListing { dir: tests/test_apps/blueprintapp/apps/admin } |  |  | 0.625 |
| walker |  | 1625 | 30 | Fs::DirListing { dir: examples/tutorial/tests } |  |  | 0.625 |
| ns | 1639 |  | 94 | Scaffold routing decorators — every URL-registration method, names only (sansio/scaffold.py:284-436) | 2.3 | 2.2 | 0.603 |
| ns | 1745 |  | 106 | Scaffold request-hook and error-handler decorators, names only (sansio/scaffold.py:460-657) | 2.4 | 2.2 | 0.584 |
| walker |  | 1835 | 210 | Code::CodeKey { rung: Names, file: src/flask/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.647 |
| ns | 1896 |  | 151 | Rest of sansio/scaffold.py: constructor, static/template properties, module-level helpers | 2.5 | 2.2 | 0.619 |
| walker |  | 2021 | 186 | Code::CodeKey { rung: Names, file: src/flask/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.670 |
| ns | 2112 |  | 216 | The complete signal registry (src/flask/signals.py:1-16, whole file) | 2.6 |  | 0.637 |
| walker |  | 2205 | 184 | Code::CodeKey { rung: Names, file: src/flask/__init__.py, decl: 0, sub: 2, line: 0 } |  |  | 0.697 |
| ns | 2294 |  | 182 | Class roster A: request/response, views, sessions, context, config (13 class statements) | 2.7 |  | 0.671 |
| walker |  | 2412 | 207 | Code::CodeKey { rung: Names, file: src/flask/app.py, decl: 0, sub: 0, line: 0 } |  |  | 0.671 |
| walker |  | 2435 | 23 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 1, sub: 0, line: 64 } |  |  | 0.671 |
| walker |  | 2470 | 35 | Fs::DirListing { dir: tests/test_apps/cliapp } |  |  | 0.671 |
| walker |  | 2480 | 10 | Fs::DirListing { dir: tests/test_apps/cliapp/inner1 } |  |  | 0.671 |
| walker |  | 2492 | 12 | Fs::DirListing { dir: tests/test_apps/cliapp/inner1/inner2 } |  |  | 0.671 |
| ns | 2510 |  | 216 | Class roster B: templating, testing, CLI, debug helpers (14 class statements) | 2.8 |  | 0.644 |
| walker |  | 2627 | 135 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.711 |
| ns | 2655 |  | 145 | Class roster C: the whole src/flask/json subpackage (12 class statements) | 2.9 |  | 0.689 |
| walker |  | 3057 | 430 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 10, sub: 0, line: 109 } |  |  | 0.698 |
| ns | 3085 |  | 430 | Flask.default_config — all 29 config keys with their defaults (src/flask/app.py:206-238) | 3.1 |  | 0.728 |
| ns | 3246 |  | 161 | Every Config loader method + the ConfigAttribute descriptor (src/flask/config.py:23-366) | 3.2 | 3.1 | 0.706 |
| walker |  | 3248 | 191 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 10, sub: 1, line: 109 } |  |  | 0.707 |
| walker |  | 3453 | 205 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 10, sub: 2, line: 109 } |  |  | 0.707 |
| walker |  | 3477 | 24 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 15, sub: 0, line: 414 } |  |  | 0.707 |
| walker |  | 3504 | 27 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 16, sub: 0, line: 447 } |  |  | 0.707 |
| ns | 3588 |  | 342 | Every App/Flask class-level customization hook (sansio/app.py:164-277, app.py:242-252) | 3.3 | 3.1 | 0.680 |
| walker |  | 3656 | 152 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 12, sub: 0, line: 310 } |  |  | 0.680 |
| ns | 3756 |  | 168 | Config semantics: dict subclass, uppercase-keys-only rule (src/flask/config.py:51-56, 68-73) | 3.4 | 3.2 | 0.663 |
| walker |  | 3856 | 200 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 10, sub: 3, line: 109 } |  |  | 0.664 |
| walker |  | 3870 | 14 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 26, sub: 0, line: 865 } |  |  | 0.664 |
| walker |  | 3885 | 15 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 25, sub: 0, line: 830 } |  |  | 0.664 |
| walker |  | 3903 | 18 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 20, sub: 0, line: 590 } |  |  | 0.664 |
| ns | 3941 |  | 185 | Environment-variable config: from_prefixed_env behaviour (src/flask/config.py:129-141) | 3.5 | 3.2 | 0.649 |
| walker |  | 3971 | 68 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 22, sub: 0, line: 632 } |  |  | 0.649 |
| ns | 3993 |  | 52 | Concrete config-file samples used by the test suite (tests/static/config.json, tests/static/config.toml) | 3.6 |  | 0.642 |
| ns | 4088 |  | 95 | Console entry point + build backend (pyproject.toml:81-89) | 4.1 |  | 0.633 |
| walker |  | 4191 | 220 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 10, sub: 4, line: 109 } |  |  | 0.634 |
| walker |  | 4216 | 25 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 34, sub: 0, line: 1079 } |  |  | 0.634 |
| walker |  | 4258 | 42 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 28, sub: 0, line: 950 } |  |  | 0.634 |
| ns | 4297 |  | 209 | How tests are run: tox env_list + the pytest command (pyproject.toml:170-182, 190-193) | 4.2 |  | 0.615 |
| walker |  | 4305 | 47 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 31, sub: 0, line: 1021 } |  |  | 0.615 |
| walker |  | 4408 | 103 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 35, sub: 0, line: 1102 } |  |  | 0.615 |
| ns | 4509 |  | 212 | The style / typing / docs tox environments (pyproject.toml:233-250) | 4.3 | 4.2 | 0.597 |
| walker |  | 4642 | 234 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 10, sub: 5, line: 109 } |  |  | 0.598 |
| walker |  | 4661 | 19 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 44, sub: 0, line: 1566 } |  |  | 0.598 |
| ns | 4664 |  | 155 | Test + type-checking settings: pytest, mypy strict, pyright (pyproject.toml:106-110, 126-131, 142-145) | 4.4 |  | 0.583 |
| walker |  | 4680 | 19 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 45, sub: 0, line: 1618 } |  |  | 0.583 |
| walker |  | 4700 | 20 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 39, sub: 0, line: 1420 } |  |  | 0.583 |
| walker |  | 4720 | 20 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 40, sub: 0, line: 1453 } |  |  | 0.583 |
| ns | 4848 |  | 184 | The `flask` command object and its app-discovery help text (src/flask/cli.py:1110-1127) | 4.5 |  | 0.569 |
| ns | 4949 |  | 101 | The complete built-in command set: run, shell, routes (src/flask/cli.py:594-596, 882, 999, 1048) | 4.6 | 4.5 | 0.564 |
| walker |  | 5018 | 298 | Plaintext::Whole { file: docs/Makefile } |  |  | 0.564 |
| ns | 5097 |  | 148 | Every `flask run` flag, declaration lines only (src/flask/cli.py:883-925) | 4.7 | 4.6 | 0.557 |
| ns | 5241 |  | 144 | The --app option: module:name form and factory auto-detection (src/flask/cli.py:453-462) | 4.8 | 4.5 | 0.549 |
| walker |  | 5368 | 350 | Code::CodeKey { rung: Names, file: src/flask/globals.py, decl: 0, sub: 0, line: 0 } |  |  | 0.549 |
| walker |  | 5384 | 16 | Code::CodeKey { rung: Decl, file: src/flask/globals.py, decl: 1, sub: 0, line: 8 } |  |  | 0.549 |
| walker |  | 5404 | 20 | Code::CodeKey { rung: Decl, file: src/flask/globals.py, decl: 10, sub: 0, line: 41 } |  |  | 0.549 |
| walker |  | 5427 | 23 | Code::CodeKey { rung: Decl, file: src/flask/globals.py, decl: 11, sub: 0, line: 44 } |  |  | 0.550 |
| walker |  | 5450 | 23 | Code::CodeKey { rung: Decl, file: src/flask/globals.py, decl: 12, sub: 0, line: 47 } |  |  | 0.550 |
| walker |  | 5473 | 23 | Code::CodeKey { rung: Decl, file: src/flask/globals.py, decl: 14, sub: 0, line: 57 } |  |  | 0.550 |
| ns | 5487 |  | 246 | The remaining global options: --debug/--no-debug, -e/--env-file, --version (src/flask/cli.py:485-489, 517-527, 283-286) | 4.9 | 4.8 | 0.536 |
| walker |  | 5496 | 23 | Code::CodeKey { rung: Decl, file: src/flask/globals.py, decl: 15, sub: 0, line: 60 } |  |  | 0.536 |
| walker |  | 5546 | 50 | Code::CodeKey { rung: Decl, file: src/flask/globals.py, decl: 13, sub: 0, line: 51 } |  |  | 0.536 |
| walker |  | 5620 | 74 | Code::CodeKey { rung: Decl, file: src/flask/globals.py, decl: 8, sub: 0, line: 33 } |  |  | 0.536 |
| walker |  | 5644 | 24 | Code::CodeKey { rung: Names, file: src/flask/wrappers.py, decl: 0, sub: 0, line: 0 } |  |  | 0.537 |
| ns | 5659 |  | 172 | `flask routes` options + the injected --debug flag (src/flask/cli.py:1001, 1049-1060) | 4.10 | 4.6 | 0.528 |
| ns | 5720 |  | 61 | CI and repo-automation inventory (.github, workflows, issue templates, .devcontainer — complete) | 4.11 |  | 0.541 |
| walker |  | 5728 | 84 | Code::CodeKey { rung: Decl, file: src/flask/wrappers.py, decl: 13, sub: 0, line: 222 } |  |  | 0.541 |
| ns | 5919 |  | 199 | CI interpreter matrix (.github/workflows/tests.yaml:19-29) | 4.12 | 4.11 | 0.534 |
| walker |  | 5927 | 199 | Code::CodeKey { rung: Decl, file: src/flask/wrappers.py, decl: 1, sub: 0, line: 18 } |  |  | 0.535 |
| ns | 6063 |  | 144 | Flask methods 1/3: construction, resources, jinja, URL adapter, run (src/flask/app.py:254-632) | 5.1 |  | 0.546 |
| walker |  | 6113 | 186 | Code::CodeKey { rung: Decl, file: src/flask/wrappers.py, decl: 1, sub: 1, line: 18 } |  |  | 0.546 |
| ns | 6206 |  | 143 | Flask methods 2/3: test clients, error handling, request dispatch (src/flask/app.py:755-1079) | 5.2 | 5.1 | 0.556 |
| ns | 6345 |  | 139 | Flask methods 3/3: url_for, make_response, teardown, contexts, WSGI entry (src/flask/app.py:1102-1618) | 5.3 | 5.2 | 0.565 |
| ns | 6532 |  | 187 | App (sansio) methods 1/2: name, logger, jinja env, config/aborter factories, blueprint registration (sansio/app.py:279-605) | 5.4 |  | 0.555 |
| walker |  | 6577 | 464 | Code::CodeKey { rung: Names, file: src/flask/helpers.py, decl: 0, sub: 0, line: 0 } |  |  | 0.557 |
| walker |  | 6590 | 13 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 3, sub: 0, line: 51 } |  |  | 0.557 |
| walker |  | 6607 | 17 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 4, sub: 0, line: 57 } |  |  | 0.557 |
| walker |  | 6627 | 20 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 12, sub: 0, line: 360 } |  |  | 0.557 |
| walker |  | 6651 | 24 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 5, sub: 0, line: 63 } |  |  | 0.557 |
| walker |  | 6677 | 26 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 8, sub: 0, line: 254 } |  |  | 0.557 |
| walker |  | 6716 | 39 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 15, sub: 0, line: 543 } |  |  | 0.557 |
| ns | 6751 |  | 219 | App (sansio) methods 2/2: template filter/test/global decorators, teardown, error routing (sansio/app.py:664-981) | 5.5 | 5.4 | 0.546 |
| walker |  | 6789 | 73 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 18, sub: 0, line: 654 } |  |  | 0.546 |
| walker |  | 6837 | 48 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 21, sub: 0, line: 665 } |  |  | 0.546 |
| walker |  | 6917 | 80 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 7, sub: 0, line: 200 } |  |  | 0.546 |
| ns | 6936 |  | 185 | Every module-level function in src/flask/helpers.py (17 definitions) | 5.6 |  | 0.559 |
| walker |  | 7050 | 133 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 14, sub: 0, line: 417 } |  |  | 0.559 |
| walker |  | 7063 | 13 | Code::CodeKey { rung: Doc, file: src/flask/helpers.py, decl: 22, sub: 0, line: 676 } |  |  | 0.559 |
| ns | 7064 |  | 128 | Blueprint (sansio) methods 1/2: setup state, deferred registration, register (sansio/blueprints.py:41-413) | 5.7 |  | 0.553 |
| ns | 7297 |  | 233 | Blueprint (sansio) methods 2/2: the app_* decorators (sansio/blueprints.py:444-685) | 5.8 | 5.7 | 0.543 |
| walker |  | 7383 | 320 | Code::CodeKey { rung: Decl, file: src/flask/wrappers.py, decl: 1, sub: 2, line: 18 } |  |  | 0.544 |
| ns | 7489 |  | 192 | The IO-side Blueprint overrides + the class-based view surface (blueprints.py, views.py) | 5.9 | 2.2 | 0.538 |
| walker |  | 7548 | 165 | Code::CodeKey { rung: Names, file: src/flask/ctx.py, decl: 0, sub: 0, line: 0 } |  |  | 0.539 |
| walker |  | 7560 | 12 | Code::CodeKey { rung: Decl, file: src/flask/ctx.py, decl: 12, sub: 0, line: 118 } |  |  | 0.539 |
| ns | 7742 |  | 253 | Request/Response attributes and properties (src/flask/wrappers.py) | 5.10 | 2.7 | 0.552 |
| walker |  | 7766 | 206 | Code::CodeKey { rung: Decl, file: src/flask/ctx.py, decl: 2, sub: 0, line: 30 } |  |  | 0.552 |
| walker |  | 8036 | 270 | Code::CodeKey { rung: Decl, file: src/flask/ctx.py, decl: 17, sub: 0, line: 260 } |  |  | 0.553 |
| ns | 8041 |  | 299 | The session interface: SessionMixin/SecureCookieSession state + every SessionInterface method (sessions.py:28-263) | 5.11 | 2.7 | 0.542 |
| walker |  | 8083 | 47 | Code::CodeKey { rung: Decl, file: src/flask/ctx.py, decl: 18, sub: 0, line: 300 } |  |  | 0.542 |
| walker |  | 8130 | 47 | Code::CodeKey { rung: Decl, file: src/flask/ctx.py, decl: 29, sub: 0, line: 510 } |  |  | 0.542 |
| walker |  | 8146 | 16 | Code::CodeKey { rung: Doc, file: src/flask/ctx.py, decl: 20, sub: 0, line: 350 } |  |  | 0.542 |
| ns | 8152 |  | 111 | The default cookie-signing session implementation (sessions.py:276-337) | 5.12 | 5.11 | 0.537 |
| walker |  | 8335 | 189 | Code::CodeKey { rung: Names, file: src/flask/templating.py, decl: 0, sub: 0, line: 0 } |  |  | 0.538 |
| walker |  | 8355 | 20 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 14, sub: 0, line: 163 } |  |  | 0.538 |
| ns | 8367 |  | 215 | The JSON subpackage API: module functions, provider methods, provider knobs | 5.13 | 2.9 | 0.530 |
| walker |  | 8381 | 26 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 2, sub: 0, line: 36 } |  |  | 0.530 |
| walker |  | 8409 | 28 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 12, sub: 0, line: 136 } |  |  | 0.530 |
| walker |  | 8437 | 28 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 15, sub: 0, line: 181 } |  |  | 0.530 |
| ns | 8509 |  | 142 | The test-support API: EnvironBuilder, FlaskClient, FlaskCliRunner members (src/flask/testing.py) | 5.14 | 2.8 | 0.525 |
| walker |  | 8620 | 183 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 4, sub: 0, line: 49 } |  |  | 0.525 |
| walker |  | 8634 | 14 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 6, sub: 0, line: 57 } |  |  | 0.525 |
| walker |  | 8648 | 14 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 7, sub: 0, line: 64 } |  |  | 0.525 |
| walker |  | 8662 | 14 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 8, sub: 0, line: 88 } |  |  | 0.525 |
| walker |  | 8677 | 15 | Code::CodeKey { rung: Names, file: src/flask/blueprints.py, decl: 0, sub: 0, line: 0 } |  |  | 0.525 |
| ns | 8737 |  | 228 | The context proxies: how current_app, g, request, session are bound (src/flask/globals.py:40-49, 57-62) | 6.1 | 1.6 | 0.536 |
| walker |  | 8769 | 92 | Code::CodeKey { rung: Decl, file: src/flask/blueprints.py, decl: 1, sub: 0, line: 18 } |  |  | 0.537 |
| walker |  | 8796 | 27 | Code::CodeKey { rung: Decl, file: src/flask/blueprints.py, decl: 5, sub: 0, line: 104 } |  |  | 0.537 |
| walker |  | 8958 | 162 | Code::CodeKey { rung: Decl, file: src/flask/blueprints.py, decl: 2, sub: 0, line: 19 } |  |  | 0.537 |
| ns | 8967 |  | 230 | The context API: ctx.py module functions, the `g` object, AppContext members | 6.2 | 2.7 | 0.549 |
| walker |  | 9144 | 186 | Code::CodeKey { rung: Names, file: src/flask/signals.py, decl: 0, sub: 0, line: 0 } |  |  | 0.560 |
| walker |  | 9164 | 20 | Code::CodeKey { rung: Doc, file: src/flask/ctx.py, decl: 23, sub: 0, line: 381 } |  |  | 0.560 |
| walker |  | 9220 | 56 | Code::CodeKey { rung: Names, file: src/flask/config.py, decl: 0, sub: 0, line: 0 } |  |  | 0.562 |
| ns | 9223 |  | 256 | What changed in the unreleased 3.2.0 (CHANGES.rst:1-16) | 6.3 |  | 0.555 |
| walker |  | 9384 | 164 | Code::CodeKey { rung: Decl, file: src/flask/config.py, decl: 2, sub: 0, line: 20 } |  |  | 0.556 |
| ns | 9399 |  | 176 | Complete docs/ listing (35 entries) | 6.4 |  | 0.574 |
| walker |  | 9410 | 26 | Code::CodeKey { rung: Decl, file: src/flask/config.py, decl: 3, sub: 0, line: 23 } |  |  | 0.574 |
| ns | 9544 |  | 145 | Complete tests/ listing (27 entries) | 6.5 |  | 0.586 |
| ns | 9611 |  | 67 | The three runnable example applications (examples/ and each app package) | 6.6 |  | 0.593 |
| walker |  | 9613 | 203 | Code::CodeKey { rung: Decl, file: src/flask/config.py, decl: 8, sub: 0, line: 50 } |  |  | 0.605 |
| walker |  | 9633 | 20 | Code::CodeKey { rung: Decl, file: src/flask/config.py, decl: 12, sub: 0, line: 187 } |  |  | 0.605 |
| walker |  | 9655 | 22 | Code::CodeKey { rung: Decl, file: src/flask/config.py, decl: 16, sub: 0, line: 323 } |  |  | 0.605 |
| walker |  | 9680 | 25 | Code::CodeKey { rung: Decl, file: src/flask/config.py, decl: 15, sub: 0, line: 304 } |  |  | 0.605 |
| walker |  | 9709 | 29 | Code::CodeKey { rung: Decl, file: src/flask/config.py, decl: 11, sub: 0, line: 126 } |  |  | 0.605 |
| walker |  | 9747 | 38 | Code::CodeKey { rung: Decl, file: src/flask/config.py, decl: 9, sub: 0, line: 94 } |  |  | 0.605 |
| walker |  | 9811 | 64 | Code::CodeKey { rung: Decl, file: src/flask/config.py, decl: 14, sub: 0, line: 256 } |  |  | 0.605 |
| walker |  | 9823 | 12 | Code::CodeKey { rung: Doc, file: src/flask/config.py, decl: 2, sub: 0, line: 20 } |  |  | 0.605 |
| walker |  | 9831 | 8 | Code::CodeKey { rung: Body, file: src/flask/templating.py, decl: 5, sub: 0, line: 54 } |  |  | 0.605 |
| walker |  | 9874 | 43 | Code::CodeKey { rung: Names, file: src/flask/json/provider.py, decl: 0, sub: 0, line: 0 } |  |  | 0.605 |
| walker |  | 9977 | 103 | Code::CodeKey { rung: Decl, file: src/flask/json/provider.py, decl: 1, sub: 0, line: 19 } |  |  | 0.606 |
| ns | 9979 |  | 368 | The canonical application factory (examples/tutorial/flaskr/__init__.py:1-48, comments elided) | 6.7 | 6.6 | 0.591 |

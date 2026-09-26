Score(3000)=0.689 I=0.905 C=0.525 ns_rows≤3K=18/57 grid(1000/1442/2080/3000/4327/6240/9000)=0.692/0.656/0.679/0.689/0.614/0.555/0.543

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 50 | 50 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 97 |  | 97 | README title + what Flask is (README.md:3, 5-9) | 1.1 |  | 0.000 |
| ns | 147 |  | 50 | Repository root listing (complete) | 1.2 |  | 0.484 |
| walker |  | 152 | 102 | Markdown::ReadmeHeadline { file: README.md } |  |  | 1.000 |
| walker |  | 239 | 87 | Toml::Identity { file: pyproject.toml } |  |  | 1.000 |
| walker |  | 269 | 30 | Markdown::HeadingsOutline { file: README.md } |  |  | 1.000 |
| ns | 271 |  | 124 | Complete source-package listing: src/flask, src/flask/json, src/flask/sansio | 1.3 |  | 0.592 |
| walker |  | 278 | 9 | Fs::DirListing { dir: examples } |  |  | 0.592 |
| walker |  | 343 | 65 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.597 |
| walker |  | 368 | 25 | Toml::Operational { file: pyproject.toml } |  |  | 0.597 |
| walker |  | 379 | 11 | Fs::DirListing { dir: .devcontainer } |  |  | 0.597 |
| walker |  | 392 | 13 | Fs::DirListing { dir: .github } |  |  | 0.597 |
| ns | 395 |  | 124 | README positioning: no enforced dependencies/layout, extension ecosystem (README.md:11-18) | 1.4 |  | 0.578 |
| walker |  | 415 | 23 | Fs::DirListing { dir: .github/workflows } |  |  | 0.579 |
| walker |  | 510 | 95 | Fs::DirListing { dir: src/flask } |  |  | 0.796 |
| walker |  | 524 | 14 | Fs::DirListing { dir: src/flask/json } |  |  | 0.857 |
| walker |  | 541 | 17 | Fs::DirListing { dir: src/flask/sansio } |  |  | 0.808 |
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
| walker |  | 2240 | 35 | Fs::DirListing { dir: tests/test_apps/cliapp } |  |  | 0.697 |
| walker |  | 2250 | 10 | Fs::DirListing { dir: tests/test_apps/cliapp/inner1 } |  |  | 0.697 |
| walker |  | 2262 | 12 | Fs::DirListing { dir: tests/test_apps/cliapp/inner1/inner2 } |  |  | 0.697 |
| ns | 2294 |  | 182 | Class roster A: request/response, views, sessions, context, config (13 class statements) | 2.7 |  | 0.671 |
| walker |  | 2397 | 135 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.740 |
| ns | 2510 |  | 216 | Class roster B: templating, testing, CLI, debug helpers (14 class statements) | 2.8 |  | 0.711 |
| walker |  | 2535 | 138 | Code::CodeKey { rung: Names, file: src/flask/json/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.711 |
| walker |  | 2560 | 25 | Code::CodeKey { rung: Body, file: src/flask/json/__init__.py, decl: 5, sub: 0, line: 138 } |  |  | 0.711 |
| ns | 2655 |  | 145 | Class roster C: the whole src/flask/json subpackage (12 class statements) | 2.9 |  | 0.689 |
| walker |  | 2858 | 298 | Plaintext::Whole { file: docs/Makefile } |  |  | 0.689 |
| walker |  | 3065 | 207 | Code::CodeKey { rung: Names, file: src/flask/app.py, decl: 0, sub: 0, line: 0 } |  |  | 0.690 |
| ns | 3085 |  | 430 | Flask.default_config — all 29 config keys with their defaults (src/flask/app.py:206-238) | 3.1 |  | 0.639 |
| walker |  | 3088 | 23 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 1, sub: 0, line: 64 } |  |  | 0.639 |
| ns | 3246 |  | 161 | Every Config loader method + the ConfigAttribute descriptor (src/flask/config.py:23-366) | 3.2 | 3.1 | 0.620 |
| walker |  | 3518 | 430 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 10, sub: 0, line: 109 } |  |  | 0.707 |
| ns | 3588 |  | 342 | Every App/Flask class-level customization hook (sansio/app.py:164-277, app.py:242-252) | 3.3 | 3.1 | 0.678 |
| walker |  | 3709 | 191 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 10, sub: 1, line: 109 } |  |  | 0.679 |
| ns | 3756 |  | 168 | Config semantics: dict subclass, uppercase-keys-only rule (src/flask/config.py:51-56, 68-73) | 3.4 | 3.2 | 0.662 |
| walker |  | 3914 | 205 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 10, sub: 2, line: 109 } |  |  | 0.663 |
| walker |  | 3938 | 24 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 15, sub: 0, line: 414 } |  |  | 0.663 |
| ns | 3941 |  | 185 | Environment-variable config: from_prefixed_env behaviour (src/flask/config.py:129-141) | 3.5 | 3.2 | 0.648 |
| walker |  | 3965 | 27 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 16, sub: 0, line: 447 } |  |  | 0.648 |
| ns | 3993 |  | 52 | Concrete config-file samples used by the test suite (tests/static/config.json, tests/static/config.toml) | 3.6 |  | 0.641 |
| ns | 4088 |  | 95 | Console entry point + build backend (pyproject.toml:81-89) | 4.1 |  | 0.632 |
| walker |  | 4117 | 152 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 12, sub: 0, line: 310 } |  |  | 0.632 |
| ns | 4297 |  | 209 | How tests are run: tox env_list + the pytest command (pyproject.toml:170-182, 190-193) | 4.2 |  | 0.613 |
| walker |  | 4317 | 200 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 10, sub: 3, line: 109 } |  |  | 0.614 |
| walker |  | 4331 | 14 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 26, sub: 0, line: 865 } |  |  | 0.614 |
| walker |  | 4346 | 15 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 25, sub: 0, line: 830 } |  |  | 0.614 |
| walker |  | 4364 | 18 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 20, sub: 0, line: 590 } |  |  | 0.614 |
| walker |  | 4432 | 68 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 22, sub: 0, line: 632 } |  |  | 0.614 |
| ns | 4509 |  | 212 | The style / typing / docs tox environments (pyproject.toml:233-250) | 4.3 | 4.2 | 0.596 |
| walker |  | 4652 | 220 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 10, sub: 4, line: 109 } |  |  | 0.597 |
| ns | 4664 |  | 155 | Test + type-checking settings: pytest, mypy strict, pyright (pyproject.toml:106-110, 126-131, 142-145) | 4.4 |  | 0.582 |
| walker |  | 4677 | 25 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 34, sub: 0, line: 1079 } |  |  | 0.582 |
| walker |  | 4719 | 42 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 28, sub: 0, line: 950 } |  |  | 0.582 |
| walker |  | 4766 | 47 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 31, sub: 0, line: 1021 } |  |  | 0.582 |
| ns | 4848 |  | 184 | The `flask` command object and its app-discovery help text (src/flask/cli.py:1110-1127) | 4.5 |  | 0.568 |
| walker |  | 4869 | 103 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 35, sub: 0, line: 1102 } |  |  | 0.568 |
| ns | 4949 |  | 101 | The complete built-in command set: run, shell, routes (src/flask/cli.py:594-596, 882, 999, 1048) | 4.6 | 4.5 | 0.563 |
| ns | 5097 |  | 148 | Every `flask run` flag, declaration lines only (src/flask/cli.py:883-925) | 4.7 | 4.6 | 0.556 |
| walker |  | 5103 | 234 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 10, sub: 5, line: 109 } |  |  | 0.557 |
| walker |  | 5122 | 19 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 44, sub: 0, line: 1566 } |  |  | 0.557 |
| walker |  | 5141 | 19 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 45, sub: 0, line: 1618 } |  |  | 0.557 |
| walker |  | 5161 | 20 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 39, sub: 0, line: 1420 } |  |  | 0.557 |
| walker |  | 5181 | 20 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 40, sub: 0, line: 1453 } |  |  | 0.557 |
| ns | 5241 |  | 144 | The --app option: module:name form and factory auto-detection (src/flask/cli.py:453-462) | 4.8 | 4.5 | 0.549 |
| ns | 5487 |  | 246 | The remaining global options: --debug/--no-debug, -e/--env-file, --version (src/flask/cli.py:485-489, 517-527, 283-286) | 4.9 | 4.8 | 0.535 |
| walker |  | 5531 | 350 | Code::CodeKey { rung: Names, file: src/flask/globals.py, decl: 0, sub: 0, line: 0 } |  |  | 0.536 |
| walker |  | 5547 | 16 | Code::CodeKey { rung: Decl, file: src/flask/globals.py, decl: 1, sub: 0, line: 8 } |  |  | 0.536 |
| walker |  | 5567 | 20 | Code::CodeKey { rung: Decl, file: src/flask/globals.py, decl: 10, sub: 0, line: 41 } |  |  | 0.536 |
| walker |  | 5590 | 23 | Code::CodeKey { rung: Decl, file: src/flask/globals.py, decl: 11, sub: 0, line: 44 } |  |  | 0.536 |
| walker |  | 5613 | 23 | Code::CodeKey { rung: Decl, file: src/flask/globals.py, decl: 12, sub: 0, line: 47 } |  |  | 0.536 |
| walker |  | 5636 | 23 | Code::CodeKey { rung: Decl, file: src/flask/globals.py, decl: 14, sub: 0, line: 57 } |  |  | 0.536 |
| walker |  | 5659 | 23 | Code::CodeKey { rung: Decl, file: src/flask/globals.py, decl: 15, sub: 0, line: 60 } |  |  | 0.528 |
| ns | 5659 |  | 172 | `flask routes` options + the injected --debug flag (src/flask/cli.py:1001, 1049-1060) | 4.10 | 4.6 | 0.528 |
| walker |  | 5709 | 50 | Code::CodeKey { rung: Decl, file: src/flask/globals.py, decl: 13, sub: 0, line: 51 } |  |  | 0.528 |
| ns | 5720 |  | 61 | CI and repo-automation inventory (.github, workflows, issue templates, .devcontainer — complete) | 4.11 |  | 0.541 |
| walker |  | 5783 | 74 | Code::CodeKey { rung: Decl, file: src/flask/globals.py, decl: 8, sub: 0, line: 33 } |  |  | 0.541 |
| walker |  | 5826 | 43 | Code::CodeKey { rung: Body, file: src/flask/json/__init__.py, decl: 3, sub: 0, line: 77 } |  |  | 0.541 |
| walker |  | 5869 | 43 | Code::CodeKey { rung: Body, file: src/flask/json/__init__.py, decl: 4, sub: 0, line: 108 } |  |  | 0.541 |
| ns | 5919 |  | 199 | CI interpreter matrix (.github/workflows/tests.yaml:19-29) | 4.12 | 4.11 | 0.534 |
| ns | 6063 |  | 144 | Flask methods 1/3: construction, resources, jinja, URL adapter, run (src/flask/app.py:254-632) | 5.1 |  | 0.545 |
| walker |  | 6192 | 323 | Code::CodeKey { rung: Names, file: src/flask/typing.py, decl: 0, sub: 0, line: 0 } |  |  | 0.545 |
| ns | 6206 |  | 143 | Flask methods 2/3: test clients, error handling, request dispatch (src/flask/app.py:755-1079) | 5.2 | 5.1 | 0.555 |
| walker |  | 6227 | 35 | Code::CodeKey { rung: Decl, file: src/flask/typing.py, decl: 19, sub: 0, line: 84 } |  |  | 0.555 |
| walker |  | 6264 | 37 | Code::CodeKey { rung: Decl, file: src/flask/typing.py, decl: 12, sub: 0, line: 64 } |  |  | 0.555 |
| walker |  | 6303 | 39 | Code::CodeKey { rung: Decl, file: src/flask/typing.py, decl: 3, sub: 0, line: 29 } |  |  | 0.555 |
| walker |  | 6342 | 39 | Code::CodeKey { rung: Decl, file: src/flask/typing.py, decl: 7, sub: 0, line: 50 } |  |  | 0.555 |
| ns | 6345 |  | 139 | Flask methods 3/3: url_for, make_response, teardown, contexts, WSGI entry (src/flask/app.py:1102-1618) | 5.3 | 5.2 | 0.564 |
| walker |  | 6383 | 41 | Code::CodeKey { rung: Decl, file: src/flask/typing.py, decl: 9, sub: 0, line: 55 } |  |  | 0.564 |
| walker |  | 6424 | 41 | Code::CodeKey { rung: Decl, file: src/flask/typing.py, decl: 11, sub: 0, line: 60 } |  |  | 0.564 |
| walker |  | 6465 | 41 | Code::CodeKey { rung: Decl, file: src/flask/typing.py, decl: 18, sub: 0, line: 79 } |  |  | 0.564 |
| walker |  | 6528 | 63 | Code::CodeKey { rung: Decl, file: src/flask/typing.py, decl: 4, sub: 0, line: 36 } |  |  | 0.564 |
| ns | 6532 |  | 187 | App (sansio) methods 1/2: name, logger, jinja env, config/aborter factories, blueprint registration (sansio/app.py:279-605) | 5.4 |  | 0.555 |
| walker |  | 6647 | 119 | Code::CodeKey { rung: Decl, file: src/flask/typing.py, decl: 1, sub: 0, line: 12 } |  |  | 0.555 |
| walker |  | 6671 | 24 | Code::CodeKey { rung: Names, file: src/flask/wrappers.py, decl: 0, sub: 0, line: 0 } |  |  | 0.555 |
| ns | 6751 |  | 219 | App (sansio) methods 2/2: template filter/test/global decorators, teardown, error routing (sansio/app.py:664-981) | 5.5 | 5.4 | 0.544 |
| walker |  | 6755 | 84 | Code::CodeKey { rung: Decl, file: src/flask/wrappers.py, decl: 13, sub: 0, line: 222 } |  |  | 0.544 |
| ns | 6936 |  | 185 | Every module-level function in src/flask/helpers.py (17 definitions) | 5.6 |  | 0.535 |
| walker |  | 6954 | 199 | Code::CodeKey { rung: Decl, file: src/flask/wrappers.py, decl: 1, sub: 0, line: 18 } |  |  | 0.535 |
| ns | 7064 |  | 128 | Blueprint (sansio) methods 1/2: setup state, deferred registration, register (sansio/blueprints.py:41-413) | 5.7 |  | 0.530 |
| walker |  | 7140 | 186 | Code::CodeKey { rung: Decl, file: src/flask/wrappers.py, decl: 1, sub: 1, line: 18 } |  |  | 0.530 |
| ns | 7297 |  | 233 | Blueprint (sansio) methods 2/2: the app_* decorators (sansio/blueprints.py:444-685) | 5.8 | 5.7 | 0.520 |
| ns | 7489 |  | 192 | The IO-side Blueprint overrides + the class-based view surface (blueprints.py, views.py) | 5.9 | 2.2 | 0.514 |
| walker |  | 7507 | 367 | Code::CodeKey { rung: Names, file: src/flask/helpers.py, decl: 0, sub: 0, line: 0 } |  |  | 0.531 |
| walker |  | 7527 | 20 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 10, sub: 0, line: 360 } |  |  | 0.531 |
| walker |  | 7551 | 24 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 3, sub: 0, line: 63 } |  |  | 0.531 |
| walker |  | 7577 | 26 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 6, sub: 0, line: 254 } |  |  | 0.531 |
| walker |  | 7616 | 39 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 13, sub: 0, line: 543 } |  |  | 0.531 |
| walker |  | 7689 | 73 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 16, sub: 0, line: 654 } |  |  | 0.531 |
| walker |  | 7737 | 48 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 19, sub: 0, line: 665 } |  |  | 0.531 |
| ns | 7742 |  | 253 | Request/Response attributes and properties (src/flask/wrappers.py) | 5.10 | 2.7 | 0.527 |
| walker |  | 7817 | 80 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 5, sub: 0, line: 200 } |  |  | 0.527 |
| walker |  | 7950 | 133 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 12, sub: 0, line: 417 } |  |  | 0.527 |
| walker |  | 7963 | 13 | Code::CodeKey { rung: Doc, file: src/flask/helpers.py, decl: 20, sub: 0, line: 676 } |  |  | 0.527 |
| ns | 8041 |  | 299 | The session interface: SessionMixin/SecureCookieSession state + every SessionInterface method (sessions.py:28-263) | 5.11 | 2.7 | 0.516 |
| ns | 8152 |  | 111 | The default cookie-signing session implementation (sessions.py:276-337) | 5.12 | 5.11 | 0.513 |
| walker |  | 8283 | 320 | Code::CodeKey { rung: Decl, file: src/flask/wrappers.py, decl: 1, sub: 2, line: 18 } |  |  | 0.530 |
| ns | 8367 |  | 215 | The JSON subpackage API: module functions, provider methods, provider knobs | 5.13 | 2.9 | 0.523 |
| walker |  | 8448 | 165 | Code::CodeKey { rung: Names, file: src/flask/ctx.py, decl: 0, sub: 0, line: 0 } |  |  | 0.524 |
| walker |  | 8460 | 12 | Code::CodeKey { rung: Decl, file: src/flask/ctx.py, decl: 12, sub: 0, line: 118 } |  |  | 0.524 |
| ns | 8509 |  | 142 | The test-support API: EnvironBuilder, FlaskClient, FlaskCliRunner members (src/flask/testing.py) | 5.14 | 2.8 | 0.519 |
| walker |  | 8666 | 206 | Code::CodeKey { rung: Decl, file: src/flask/ctx.py, decl: 2, sub: 0, line: 30 } |  |  | 0.519 |
| ns | 8737 |  | 228 | The context proxies: how current_app, g, request, session are bound (src/flask/globals.py:40-49, 57-62) | 6.1 | 1.6 | 0.529 |
| walker |  | 8936 | 270 | Code::CodeKey { rung: Decl, file: src/flask/ctx.py, decl: 17, sub: 0, line: 260 } |  |  | 0.530 |
| ns | 8967 |  | 230 | The context API: ctx.py module functions, the `g` object, AppContext members | 6.2 | 2.7 | 0.543 |
| walker |  | 8983 | 47 | Code::CodeKey { rung: Decl, file: src/flask/ctx.py, decl: 18, sub: 0, line: 300 } |  |  | 0.543 |
| walker |  | 9030 | 47 | Code::CodeKey { rung: Decl, file: src/flask/ctx.py, decl: 29, sub: 0, line: 510 } |  |  | 0.543 |
| walker |  | 9046 | 16 | Code::CodeKey { rung: Doc, file: src/flask/ctx.py, decl: 20, sub: 0, line: 350 } |  |  | 0.543 |
| ns | 9223 |  | 256 | What changed in the unreleased 3.2.0 (CHANGES.rst:1-16) | 6.3 |  | 0.536 |
| walker |  | 9235 | 189 | Code::CodeKey { rung: Names, file: src/flask/templating.py, decl: 0, sub: 0, line: 0 } |  |  | 0.537 |
| walker |  | 9255 | 20 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 14, sub: 0, line: 163 } |  |  | 0.537 |
| walker |  | 9281 | 26 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 2, sub: 0, line: 36 } |  |  | 0.537 |
| walker |  | 9309 | 28 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 12, sub: 0, line: 136 } |  |  | 0.537 |
| walker |  | 9337 | 28 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 15, sub: 0, line: 181 } |  |  | 0.537 |
| ns | 9399 |  | 176 | Complete docs/ listing (35 entries) | 6.4 |  | 0.555 |
| walker |  | 9520 | 183 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 4, sub: 0, line: 49 } |  |  | 0.555 |
| walker |  | 9534 | 14 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 6, sub: 0, line: 57 } |  |  | 0.555 |
| ns | 9544 |  | 145 | Complete tests/ listing (27 entries) | 6.5 |  | 0.568 |
| walker |  | 9548 | 14 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 7, sub: 0, line: 64 } |  |  | 0.568 |
| walker |  | 9562 | 14 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 8, sub: 0, line: 88 } |  |  | 0.568 |
| walker |  | 9582 | 20 | Code::CodeKey { rung: Doc, file: src/flask/ctx.py, decl: 23, sub: 0, line: 381 } |  |  | 0.568 |
| walker |  | 9590 | 8 | Code::CodeKey { rung: Body, file: src/flask/templating.py, decl: 5, sub: 0, line: 54 } |  |  | 0.568 |
| walker |  | 9595 | 5 | Code::CodeKey { rung: Body, file: src/flask/helpers.py, decl: 18, sub: 0, line: 662 } |  |  | 0.568 |
| ns | 9611 |  | 67 | The three runnable example applications (examples/ and each app package) | 6.6 |  | 0.576 |
| walker |  | 9711 | 116 | Code::CodeKey { rung: Names, file: src/flask/debughelpers.py, decl: 0, sub: 0, line: 0 } |  |  | 0.578 |
| walker |  | 9731 | 20 | Code::CodeKey { rung: Decl, file: src/flask/debughelpers.py, decl: 5, sub: 0, line: 50 } |  |  | 0.578 |
| walker |  | 9771 | 40 | Code::CodeKey { rung: Decl, file: src/flask/debughelpers.py, decl: 2, sub: 0, line: 23 } |  |  | 0.578 |
| walker |  | 9855 | 84 | Code::CodeKey { rung: Decl, file: src/flask/debughelpers.py, decl: 9, sub: 0, line: 124 } |  |  | 0.578 |
| walker |  | 9862 | 7 | Code::CodeKey { rung: Body, file: src/flask/debughelpers.py, decl: 4, sub: 0, line: 46 } |  |  | 0.578 |
| walker |  | 9876 | 14 | Code::CodeKey { rung: Doc, file: src/flask/debughelpers.py, decl: 9, sub: 0, line: 124 } |  |  | 0.578 |
| walker |  | 9907 | 31 | Code::CodeKey { rung: Doc, file: src/flask/templating.py, decl: 4, sub: 0, line: 49 } |  |  | 0.578 |
| ns | 9979 |  | 368 | The canonical application factory (examples/tutorial/flaskr/__init__.py:1-48, comments elided) | 6.7 | 6.6 | 0.564 |
| walker |  | 9989 | 82 | Code::CodeKey { rung: Names, file: src/flask/cli.py, decl: 0, sub: 0, line: 0 } |  |  | 0.565 |

Score(3000)=0.710 I=0.921 C=0.548 ns_rows≤3K=18/57 grid(1000/1442/2080/3000/4327/6240/9000)=0.734/0.689/0.696/0.710/0.629/0.568/0.546

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
| walker |  | 303 | 25 | Toml::Operational { file: pyproject.toml } |  |  | 0.593 |
| walker |  | 314 | 11 | Fs::DirListing { dir: .devcontainer } |  |  | 0.593 |
| walker |  | 327 | 13 | Fs::DirListing { dir: .github } |  |  | 0.593 |
| walker |  | 350 | 23 | Fs::DirListing { dir: .github/workflows } |  |  | 0.594 |
| ns | 395 |  | 124 | README positioning: no enforced dependencies/layout, extension ecosystem (README.md:11-18) | 1.4 |  | 0.542 |
| walker |  | 445 | 95 | Fs::DirListing { dir: src/flask } |  |  | 0.765 |
| walker |  | 459 | 14 | Fs::DirListing { dir: src/flask/json } |  |  | 0.826 |
| walker |  | 476 | 17 | Fs::DirListing { dir: src/flask/sansio } |  |  | 0.916 |
| ns | 541 |  | 146 | Canonical usage example: @app.route + `flask run` (README.md:20-36) | 1.5 |  | 0.783 |
| walker |  | 598 | 122 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.858 |
| ns | 707 |  | 166 | Public export surface 1/3: Flask, Blueprint, Config, json, context fns, globals (src/flask/__init__.py:1-12) | 1.6 |  | 0.784 |
| walker |  | 774 | 176 | Fs::DirListing { dir: docs } |  |  | 0.788 |
| walker |  | 801 | 27 | Fs::DirListing { dir: docs/_static } |  |  | 0.788 |
| walker |  | 865 | 64 | Fs::DirListing { dir: docs/deploying } |  |  | 0.733 |
| ns | 865 |  | 158 | Public export surface 2/3: the `helpers` free functions + jsonify (src/flask/__init__.py:13-23) | 1.7 | 1.6 | 0.733 |
| walker |  | 943 | 78 | Fs::DirListing { dir: docs/tutorial } |  |  | 0.733 |
| walker |  | 957 | 14 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.734 |
| walker |  | 1102 | 145 | Fs::DirListing { dir: docs/patterns } |  |  | 0.734 |
| ns | 1121 |  | 256 | Public export surface 3/3: all ten signals, template renderers, Request/Response (src/flask/__init__.py:24-39) | 1.8 | 1.7 | 0.671 |
| walker |  | 1126 | 24 | Fs::DirListing { dir: examples/celery } |  |  | 0.671 |
| walker |  | 1145 | 19 | Fs::DirListing { dir: examples/celery/src/task_app } |  |  | 0.671 |
| walker |  | 1290 | 145 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.678 |
| ns | 1391 |  | 270 | Package identity + runtime dependency stack (pyproject.toml:1-8, 22-34) | 1.9 |  | 0.686 |
| walker |  | 1435 | 145 | Fs::DirListing { dir: tests } |  |  | 0.689 |
| walker |  | 1454 | 19 | Fs::DirListing { dir: tests/type_check } |  |  | 0.689 |
| ns | 1468 |  | 77 | Why src/flask/sansio/ exists (src/flask/sansio/README.md:1-6) | 2.1 |  | 0.671 |
| walker |  | 1481 | 27 | Fs::DirListing { dir: examples/javascript } |  |  | 0.671 |
| walker |  | 1493 | 12 | Fs::DirListing { dir: examples/javascript/tests } |  |  | 0.671 |
| walker |  | 1506 | 13 | Fs::DirListing { dir: examples/javascript/js_example } |  |  | 0.672 |
| walker |  | 1533 | 27 | Fs::DirListing { dir: examples/tutorial } |  |  | 0.672 |
| ns | 1545 |  | 77 | The class spine: Scaffold -> App -> Flask and Scaffold -> Blueprint -> Blueprint (6 class statements) | 2.2 |  | 0.655 |
| walker |  | 1561 | 28 | Fs::DirListing { dir: examples/tutorial/flaskr } |  |  | 0.656 |
| walker |  | 1590 | 29 | Fs::DirListing { dir: tests/test_apps } |  |  | 0.656 |
| walker |  | 1599 | 9 | Fs::DirListing { dir: tests/test_apps/blueprintapp } |  |  | 0.656 |
| walker |  | 1608 | 9 | Fs::DirListing { dir: tests/test_apps/helloworld } |  |  | 0.656 |
| walker |  | 1617 | 9 | Fs::DirListing { dir: tests/test_apps/subdomaintestmodule } |  |  | 0.656 |
| walker |  | 1631 | 14 | Fs::DirListing { dir: tests/test_apps/blueprintapp/apps } |  |  | 0.656 |
| ns | 1639 |  | 94 | Scaffold routing decorators — every URL-registration method, names only (sansio/scaffold.py:284-436) | 2.3 | 2.2 | 0.634 |
| walker |  | 1640 | 9 | Fs::DirListing { dir: tests/test_apps/blueprintapp/apps/frontend } |  |  | 0.634 |
| walker |  | 1652 | 12 | Fs::DirListing { dir: tests/test_apps/blueprintapp/apps/admin } |  |  | 0.634 |
| walker |  | 1682 | 30 | Fs::DirListing { dir: examples/tutorial/tests } |  |  | 0.634 |
| ns | 1745 |  | 106 | Scaffold request-hook and error-handler decorators, names only (sansio/scaffold.py:460-657) | 2.4 | 2.2 | 0.613 |
| walker |  | 1892 | 210 | Code::CodeKey { rung: Names, file: src/flask/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.676 |
| ns | 1896 |  | 151 | Rest of sansio/scaffold.py: constructor, static/template properties, module-level helpers | 2.5 | 2.2 | 0.646 |
| walker |  | 2078 | 186 | Code::CodeKey { rung: Names, file: src/flask/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.696 |
| ns | 2112 |  | 216 | The complete signal registry (src/flask/signals.py:1-16, whole file) | 2.6 |  | 0.662 |
| walker |  | 2262 | 184 | Code::CodeKey { rung: Names, file: src/flask/__init__.py, decl: 0, sub: 2, line: 0 } |  |  | 0.721 |
| ns | 2294 |  | 182 | Class roster A: request/response, views, sessions, context, config (13 class statements) | 2.7 |  | 0.694 |
| walker |  | 2297 | 35 | Fs::DirListing { dir: tests/test_apps/cliapp } |  |  | 0.694 |
| walker |  | 2307 | 10 | Fs::DirListing { dir: tests/test_apps/cliapp/inner1 } |  |  | 0.694 |
| walker |  | 2319 | 12 | Fs::DirListing { dir: tests/test_apps/cliapp/inner1/inner2 } |  |  | 0.694 |
| walker |  | 2454 | 135 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.763 |
| ns | 2510 |  | 216 | Class roster B: templating, testing, CLI, debug helpers (14 class statements) | 2.8 |  | 0.732 |
| walker |  | 2603 | 149 | Code::CodeKey { rung: Names, file: src/flask/json/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.732 |
| walker |  | 2628 | 25 | Code::CodeKey { rung: Body, file: src/flask/json/__init__.py, decl: 5, sub: 0, line: 138 } |  |  | 0.732 |
| ns | 2655 |  | 145 | Class roster C: the whole src/flask/json subpackage (12 class statements) | 2.9 |  | 0.710 |
| walker |  | 2926 | 298 | Plaintext::Whole { file: docs/Makefile } |  |  | 0.710 |
| walker |  | 2969 | 43 | Code::CodeKey { rung: Body, file: src/flask/json/__init__.py, decl: 3, sub: 0, line: 77 } |  |  | 0.710 |
| ns | 3085 |  | 430 | Flask.default_config — all 29 config keys with their defaults (src/flask/app.py:206-238) | 3.1 |  | 0.658 |
| walker |  | 3176 | 207 | Code::CodeKey { rung: Names, file: src/flask/app.py, decl: 0, sub: 0, line: 0 } |  |  | 0.658 |
| walker |  | 3199 | 23 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 1, sub: 0, line: 64 } |  |  | 0.658 |
| ns | 3246 |  | 161 | Every Config loader method + the ConfigAttribute descriptor (src/flask/config.py:23-366) | 3.2 | 3.1 | 0.639 |
| ns | 3588 |  | 342 | Every App/Flask class-level customization hook (sansio/app.py:164-277, app.py:242-252) | 3.3 | 3.1 | 0.613 |
| walker |  | 3629 | 430 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 10, sub: 0, line: 109 } |  |  | 0.695 |
| ns | 3756 |  | 168 | Config semantics: dict subclass, uppercase-keys-only rule (src/flask/config.py:51-56, 68-73) | 3.4 | 3.2 | 0.678 |
| walker |  | 3820 | 191 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 10, sub: 1, line: 109 } |  |  | 0.680 |
| ns | 3941 |  | 185 | Environment-variable config: from_prefixed_env behaviour (src/flask/config.py:129-141) | 3.5 | 3.2 | 0.664 |
| ns | 3993 |  | 52 | Concrete config-file samples used by the test suite (tests/static/config.json, tests/static/config.toml) | 3.6 |  | 0.657 |
| walker |  | 4025 | 205 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 10, sub: 2, line: 109 } |  |  | 0.658 |
| walker |  | 4049 | 24 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 15, sub: 0, line: 414 } |  |  | 0.658 |
| walker |  | 4076 | 27 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 16, sub: 0, line: 447 } |  |  | 0.658 |
| ns | 4088 |  | 95 | Console entry point + build backend (pyproject.toml:81-89) | 4.1 |  | 0.649 |
| walker |  | 4228 | 152 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 12, sub: 0, line: 310 } |  |  | 0.649 |
| ns | 4297 |  | 209 | How tests are run: tox env_list + the pytest command (pyproject.toml:170-182, 190-193) | 4.2 |  | 0.629 |
| walker |  | 4428 | 200 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 10, sub: 3, line: 109 } |  |  | 0.630 |
| walker |  | 4442 | 14 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 26, sub: 0, line: 865 } |  |  | 0.630 |
| walker |  | 4457 | 15 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 25, sub: 0, line: 830 } |  |  | 0.630 |
| walker |  | 4475 | 18 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 20, sub: 0, line: 590 } |  |  | 0.630 |
| ns | 4509 |  | 212 | The style / typing / docs tox environments (pyproject.toml:233-250) | 4.3 | 4.2 | 0.611 |
| walker |  | 4543 | 68 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 22, sub: 0, line: 632 } |  |  | 0.611 |
| ns | 4664 |  | 155 | Test + type-checking settings: pytest, mypy strict, pyright (pyproject.toml:106-110, 126-131, 142-145) | 4.4 |  | 0.596 |
| walker |  | 4763 | 220 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 10, sub: 4, line: 109 } |  |  | 0.597 |
| walker |  | 4788 | 25 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 34, sub: 0, line: 1079 } |  |  | 0.597 |
| walker |  | 4830 | 42 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 28, sub: 0, line: 950 } |  |  | 0.597 |
| ns | 4848 |  | 184 | The `flask` command object and its app-discovery help text (src/flask/cli.py:1110-1127) | 4.5 |  | 0.582 |
| walker |  | 4877 | 47 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 31, sub: 0, line: 1021 } |  |  | 0.582 |
| ns | 4949 |  | 101 | The complete built-in command set: run, shell, routes (src/flask/cli.py:594-596, 882, 999, 1048) | 4.6 | 4.5 | 0.577 |
| walker |  | 4980 | 103 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 35, sub: 0, line: 1102 } |  |  | 0.577 |
| ns | 5097 |  | 148 | Every `flask run` flag, declaration lines only (src/flask/cli.py:883-925) | 4.7 | 4.6 | 0.570 |
| walker |  | 5214 | 234 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 10, sub: 5, line: 109 } |  |  | 0.571 |
| walker |  | 5233 | 19 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 44, sub: 0, line: 1566 } |  |  | 0.571 |
| ns | 5241 |  | 144 | The --app option: module:name form and factory auto-detection (src/flask/cli.py:453-462) | 4.8 | 4.5 | 0.563 |
| walker |  | 5252 | 19 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 45, sub: 0, line: 1618 } |  |  | 0.563 |
| walker |  | 5272 | 20 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 39, sub: 0, line: 1420 } |  |  | 0.563 |
| walker |  | 5292 | 20 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 40, sub: 0, line: 1453 } |  |  | 0.563 |
| ns | 5487 |  | 246 | The remaining global options: --debug/--no-debug, -e/--env-file, --version (src/flask/cli.py:485-489, 517-527, 283-286) | 4.9 | 4.8 | 0.549 |
| walker |  | 5642 | 350 | Code::CodeKey { rung: Names, file: src/flask/globals.py, decl: 0, sub: 0, line: 0 } |  |  | 0.549 |
| walker |  | 5658 | 16 | Code::CodeKey { rung: Decl, file: src/flask/globals.py, decl: 1, sub: 0, line: 8 } |  |  | 0.549 |
| ns | 5659 |  | 172 | `flask routes` options + the injected --debug flag (src/flask/cli.py:1001, 1049-1060) | 4.10 | 4.6 | 0.541 |
| walker |  | 5678 | 20 | Code::CodeKey { rung: Decl, file: src/flask/globals.py, decl: 10, sub: 0, line: 41 } |  |  | 0.541 |
| walker |  | 5701 | 23 | Code::CodeKey { rung: Decl, file: src/flask/globals.py, decl: 11, sub: 0, line: 44 } |  |  | 0.541 |
| ns | 5720 |  | 61 | CI and repo-automation inventory (.github, workflows, issue templates, .devcontainer — complete) | 4.11 |  | 0.553 |
| walker |  | 5724 | 23 | Code::CodeKey { rung: Decl, file: src/flask/globals.py, decl: 12, sub: 0, line: 47 } |  |  | 0.554 |
| walker |  | 5747 | 23 | Code::CodeKey { rung: Decl, file: src/flask/globals.py, decl: 14, sub: 0, line: 57 } |  |  | 0.554 |
| walker |  | 5770 | 23 | Code::CodeKey { rung: Decl, file: src/flask/globals.py, decl: 15, sub: 0, line: 60 } |  |  | 0.554 |
| walker |  | 5820 | 50 | Code::CodeKey { rung: Decl, file: src/flask/globals.py, decl: 13, sub: 0, line: 51 } |  |  | 0.554 |
| walker |  | 5894 | 74 | Code::CodeKey { rung: Decl, file: src/flask/globals.py, decl: 8, sub: 0, line: 33 } |  |  | 0.554 |
| ns | 5919 |  | 199 | CI interpreter matrix (.github/workflows/tests.yaml:19-29) | 4.12 | 4.11 | 0.547 |
| walker |  | 5937 | 43 | Code::CodeKey { rung: Body, file: src/flask/json/__init__.py, decl: 4, sub: 0, line: 108 } |  |  | 0.547 |
| ns | 6063 |  | 144 | Flask methods 1/3: construction, resources, jinja, URL adapter, run (src/flask/app.py:254-632) | 5.1 |  | 0.558 |
| ns | 6206 |  | 143 | Flask methods 2/3: test clients, error handling, request dispatch (src/flask/app.py:755-1079) | 5.2 | 5.1 | 0.568 |
| walker |  | 6260 | 323 | Code::CodeKey { rung: Names, file: src/flask/typing.py, decl: 0, sub: 0, line: 0 } |  |  | 0.568 |
| walker |  | 6295 | 35 | Code::CodeKey { rung: Decl, file: src/flask/typing.py, decl: 19, sub: 0, line: 84 } |  |  | 0.568 |
| walker |  | 6332 | 37 | Code::CodeKey { rung: Decl, file: src/flask/typing.py, decl: 12, sub: 0, line: 64 } |  |  | 0.568 |
| ns | 6345 |  | 139 | Flask methods 3/3: url_for, make_response, teardown, contexts, WSGI entry (src/flask/app.py:1102-1618) | 5.3 | 5.2 | 0.576 |
| walker |  | 6371 | 39 | Code::CodeKey { rung: Decl, file: src/flask/typing.py, decl: 3, sub: 0, line: 29 } |  |  | 0.576 |
| walker |  | 6410 | 39 | Code::CodeKey { rung: Decl, file: src/flask/typing.py, decl: 7, sub: 0, line: 50 } |  |  | 0.576 |
| walker |  | 6451 | 41 | Code::CodeKey { rung: Decl, file: src/flask/typing.py, decl: 9, sub: 0, line: 55 } |  |  | 0.576 |
| walker |  | 6492 | 41 | Code::CodeKey { rung: Decl, file: src/flask/typing.py, decl: 11, sub: 0, line: 60 } |  |  | 0.576 |
| ns | 6532 |  | 187 | App (sansio) methods 1/2: name, logger, jinja env, config/aborter factories, blueprint registration (sansio/app.py:279-605) | 5.4 |  | 0.567 |
| walker |  | 6533 | 41 | Code::CodeKey { rung: Decl, file: src/flask/typing.py, decl: 18, sub: 0, line: 79 } |  |  | 0.567 |
| walker |  | 6596 | 63 | Code::CodeKey { rung: Decl, file: src/flask/typing.py, decl: 4, sub: 0, line: 36 } |  |  | 0.567 |
| walker |  | 6715 | 119 | Code::CodeKey { rung: Decl, file: src/flask/typing.py, decl: 1, sub: 0, line: 12 } |  |  | 0.567 |
| walker |  | 6739 | 24 | Code::CodeKey { rung: Names, file: src/flask/wrappers.py, decl: 0, sub: 0, line: 0 } |  |  | 0.567 |
| ns | 6751 |  | 219 | App (sansio) methods 2/2: template filter/test/global decorators, teardown, error routing (sansio/app.py:664-981) | 5.5 | 5.4 | 0.556 |
| walker |  | 6823 | 84 | Code::CodeKey { rung: Decl, file: src/flask/wrappers.py, decl: 13, sub: 0, line: 222 } |  |  | 0.556 |
| ns | 6936 |  | 185 | Every module-level function in src/flask/helpers.py (17 definitions) | 5.6 |  | 0.547 |
| walker |  | 7022 | 199 | Code::CodeKey { rung: Decl, file: src/flask/wrappers.py, decl: 1, sub: 0, line: 18 } |  |  | 0.547 |
| ns | 7064 |  | 128 | Blueprint (sansio) methods 1/2: setup state, deferred registration, register (sansio/blueprints.py:41-413) | 5.7 |  | 0.541 |
| walker |  | 7208 | 186 | Code::CodeKey { rung: Decl, file: src/flask/wrappers.py, decl: 1, sub: 1, line: 18 } |  |  | 0.541 |
| ns | 7297 |  | 233 | Blueprint (sansio) methods 2/2: the app_* decorators (sansio/blueprints.py:444-685) | 5.8 | 5.7 | 0.532 |
| ns | 7489 |  | 192 | The IO-side Blueprint overrides + the class-based view surface (blueprints.py, views.py) | 5.9 | 2.2 | 0.525 |
| walker |  | 7672 | 464 | Code::CodeKey { rung: Names, file: src/flask/helpers.py, decl: 0, sub: 0, line: 0 } |  |  | 0.548 |
| walker |  | 7685 | 13 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 3, sub: 0, line: 51 } |  |  | 0.548 |
| walker |  | 7702 | 17 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 4, sub: 0, line: 57 } |  |  | 0.548 |
| walker |  | 7722 | 20 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 12, sub: 0, line: 360 } |  |  | 0.548 |
| ns | 7742 |  | 253 | Request/Response attributes and properties (src/flask/wrappers.py) | 5.10 | 2.7 | 0.543 |
| walker |  | 7746 | 24 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 5, sub: 0, line: 63 } |  |  | 0.543 |
| walker |  | 7772 | 26 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 8, sub: 0, line: 254 } |  |  | 0.543 |
| walker |  | 7811 | 39 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 15, sub: 0, line: 543 } |  |  | 0.543 |
| walker |  | 7884 | 73 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 18, sub: 0, line: 654 } |  |  | 0.543 |
| walker |  | 7932 | 48 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 21, sub: 0, line: 665 } |  |  | 0.543 |
| walker |  | 8012 | 80 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 7, sub: 0, line: 200 } |  |  | 0.543 |
| ns | 8041 |  | 299 | The session interface: SessionMixin/SecureCookieSession state + every SessionInterface method (sessions.py:28-263) | 5.11 | 2.7 | 0.532 |
| walker |  | 8145 | 133 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 14, sub: 0, line: 417 } |  |  | 0.532 |
| ns | 8152 |  | 111 | The default cookie-signing session implementation (sessions.py:276-337) | 5.12 | 5.11 | 0.528 |
| walker |  | 8158 | 13 | Code::CodeKey { rung: Doc, file: src/flask/helpers.py, decl: 22, sub: 0, line: 676 } |  |  | 0.528 |
| ns | 8367 |  | 215 | The JSON subpackage API: module functions, provider methods, provider knobs | 5.13 | 2.9 | 0.521 |
| walker |  | 8478 | 320 | Code::CodeKey { rung: Decl, file: src/flask/wrappers.py, decl: 1, sub: 2, line: 18 } |  |  | 0.538 |
| ns | 8509 |  | 142 | The test-support API: EnvironBuilder, FlaskClient, FlaskCliRunner members (src/flask/testing.py) | 5.14 | 2.8 | 0.533 |
| walker |  | 8643 | 165 | Code::CodeKey { rung: Names, file: src/flask/ctx.py, decl: 0, sub: 0, line: 0 } |  |  | 0.534 |
| walker |  | 8655 | 12 | Code::CodeKey { rung: Decl, file: src/flask/ctx.py, decl: 12, sub: 0, line: 118 } |  |  | 0.534 |
| ns | 8737 |  | 228 | The context proxies: how current_app, g, request, session are bound (src/flask/globals.py:40-49, 57-62) | 6.1 | 1.6 | 0.544 |
| walker |  | 8861 | 206 | Code::CodeKey { rung: Decl, file: src/flask/ctx.py, decl: 2, sub: 0, line: 30 } |  |  | 0.544 |
| ns | 8967 |  | 230 | The context API: ctx.py module functions, the `g` object, AppContext members | 6.2 | 2.7 | 0.539 |
| walker |  | 9131 | 270 | Code::CodeKey { rung: Decl, file: src/flask/ctx.py, decl: 17, sub: 0, line: 260 } |  |  | 0.557 |
| walker |  | 9178 | 47 | Code::CodeKey { rung: Decl, file: src/flask/ctx.py, decl: 18, sub: 0, line: 300 } |  |  | 0.557 |
| ns | 9223 |  | 256 | What changed in the unreleased 3.2.0 (CHANGES.rst:1-16) | 6.3 |  | 0.550 |
| walker |  | 9225 | 47 | Code::CodeKey { rung: Decl, file: src/flask/ctx.py, decl: 29, sub: 0, line: 510 } |  |  | 0.550 |
| walker |  | 9241 | 16 | Code::CodeKey { rung: Doc, file: src/flask/ctx.py, decl: 20, sub: 0, line: 350 } |  |  | 0.550 |
| ns | 9399 |  | 176 | Complete docs/ listing (35 entries) | 6.4 |  | 0.568 |
| walker |  | 9430 | 189 | Code::CodeKey { rung: Names, file: src/flask/templating.py, decl: 0, sub: 0, line: 0 } |  |  | 0.569 |
| walker |  | 9450 | 20 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 14, sub: 0, line: 163 } |  |  | 0.569 |
| walker |  | 9476 | 26 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 2, sub: 0, line: 36 } |  |  | 0.569 |
| walker |  | 9504 | 28 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 12, sub: 0, line: 136 } |  |  | 0.569 |
| walker |  | 9532 | 28 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 15, sub: 0, line: 181 } |  |  | 0.569 |
| ns | 9544 |  | 145 | Complete tests/ listing (27 entries) | 6.5 |  | 0.581 |
| ns | 9611 |  | 67 | The three runnable example applications (examples/ and each app package) | 6.6 |  | 0.589 |
| walker |  | 9715 | 183 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 4, sub: 0, line: 49 } |  |  | 0.589 |
| walker |  | 9729 | 14 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 6, sub: 0, line: 57 } |  |  | 0.589 |
| walker |  | 9743 | 14 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 7, sub: 0, line: 64 } |  |  | 0.589 |
| walker |  | 9757 | 14 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 8, sub: 0, line: 88 } |  |  | 0.589 |
| walker |  | 9777 | 20 | Code::CodeKey { rung: Doc, file: src/flask/ctx.py, decl: 23, sub: 0, line: 381 } |  |  | 0.589 |
| walker |  | 9785 | 8 | Code::CodeKey { rung: Body, file: src/flask/templating.py, decl: 5, sub: 0, line: 54 } |  |  | 0.589 |
| walker |  | 9816 | 31 | Code::CodeKey { rung: Doc, file: src/flask/templating.py, decl: 4, sub: 0, line: 49 } |  |  | 0.589 |
| walker |  | 9821 | 5 | Code::CodeKey { rung: Body, file: src/flask/helpers.py, decl: 20, sub: 0, line: 662 } |  |  | 0.589 |
| walker |  | 9937 | 116 | Code::CodeKey { rung: Names, file: src/flask/debughelpers.py, decl: 0, sub: 0, line: 0 } |  |  | 0.591 |
| walker |  | 9957 | 20 | Code::CodeKey { rung: Decl, file: src/flask/debughelpers.py, decl: 5, sub: 0, line: 50 } |  |  | 0.591 |
| ns | 9979 |  | 368 | The canonical application factory (examples/tutorial/flaskr/__init__.py:1-48, comments elided) | 6.7 | 6.6 | 0.577 |
| walker |  | 9997 | 40 | Code::CodeKey { rung: Decl, file: src/flask/debughelpers.py, decl: 2, sub: 0, line: 23 } |  |  | 0.577 |

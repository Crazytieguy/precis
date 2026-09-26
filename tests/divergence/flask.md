Score(3000)=0.710 I=0.921 C=0.548 ns_rows≤3K=18/57 grid(1000/1442/2080/3000/4327/6240/9000)=0.734/0.689/0.696/0.710/0.629/0.568/0.557

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
| walker |  | 3999 | 179 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 10, sub: 2, line: 109 } |  |  | 0.658 |
| walker |  | 4028 | 29 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 20, sub: 0, line: 590 } |  |  | 0.658 |
| walker |  | 4067 | 39 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 15, sub: 0, line: 414 } |  |  | 0.658 |
| ns | 4088 |  | 95 | Console entry point + build backend (pyproject.toml:81-89) | 4.1 |  | 0.649 |
| walker |  | 4109 | 42 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 16, sub: 0, line: 447 } |  |  | 0.649 |
| walker |  | 4269 | 160 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 12, sub: 0, line: 310 } |  |  | 0.649 |
| ns | 4297 |  | 209 | How tests are run: tox env_list + the pytest command (pyproject.toml:170-182, 190-193) | 4.2 |  | 0.629 |
| walker |  | 4456 | 187 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 10, sub: 3, line: 109 } |  |  | 0.630 |
| walker |  | 4487 | 31 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 26, sub: 0, line: 865 } |  |  | 0.630 |
| ns | 4509 |  | 212 | The style / typing / docs tox environments (pyproject.toml:233-250) | 4.3 | 4.2 | 0.612 |
| walker |  | 4519 | 32 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 25, sub: 0, line: 830 } |  |  | 0.612 |
| walker |  | 4572 | 53 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 28, sub: 0, line: 950 } |  |  | 0.612 |
| walker |  | 4651 | 79 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 22, sub: 0, line: 632 } |  |  | 0.612 |
| ns | 4664 |  | 155 | Test + type-checking settings: pytest, mypy strict, pyright (pyproject.toml:106-110, 126-131, 142-145) | 4.4 |  | 0.597 |
| ns | 4848 |  | 184 | The `flask` command object and its app-discovery help text (src/flask/cli.py:1110-1127) | 4.5 |  | 0.582 |
| walker |  | 4940 | 289 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 10, sub: 4, line: 109 } |  |  | 0.583 |
| ns | 4949 |  | 101 | The complete built-in command set: run, shell, routes (src/flask/cli.py:594-596, 882, 999, 1048) | 4.6 | 4.5 | 0.578 |
| walker |  | 4972 | 32 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 39, sub: 0, line: 1420 } |  |  | 0.578 |
| walker |  | 5004 | 32 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 40, sub: 0, line: 1453 } |  |  | 0.578 |
| walker |  | 5040 | 36 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 44, sub: 0, line: 1566 } |  |  | 0.578 |
| walker |  | 5076 | 36 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 45, sub: 0, line: 1618 } |  |  | 0.578 |
| ns | 5097 |  | 148 | Every `flask run` flag, declaration lines only (src/flask/cli.py:883-925) | 4.7 | 4.6 | 0.571 |
| walker |  | 5118 | 42 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 34, sub: 0, line: 1079 } |  |  | 0.571 |
| walker |  | 5177 | 59 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 31, sub: 0, line: 1021 } |  |  | 0.571 |
| ns | 5241 |  | 144 | The --app option: module:name form and factory auto-detection (src/flask/cli.py:453-462) | 4.8 | 4.5 | 0.563 |
| walker |  | 5292 | 115 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 35, sub: 0, line: 1102 } |  |  | 0.563 |
| walker |  | 5479 | 187 | Code::CodeKey { rung: Names, file: src/flask/globals.py, decl: 0, sub: 0, line: 0 } |  |  | 0.563 |
| ns | 5487 |  | 246 | The remaining global options: --debug/--no-debug, -e/--env-file, --version (src/flask/cli.py:485-489, 517-527, 283-286) | 4.9 | 4.8 | 0.549 |
| walker |  | 5499 | 20 | Code::CodeKey { rung: Decl, file: src/flask/globals.py, decl: 3, sub: 0, line: 41 } |  |  | 0.549 |
| walker |  | 5522 | 23 | Code::CodeKey { rung: Decl, file: src/flask/globals.py, decl: 4, sub: 0, line: 44 } |  |  | 0.550 |
| walker |  | 5545 | 23 | Code::CodeKey { rung: Decl, file: src/flask/globals.py, decl: 5, sub: 0, line: 47 } |  |  | 0.550 |
| walker |  | 5568 | 23 | Code::CodeKey { rung: Decl, file: src/flask/globals.py, decl: 7, sub: 0, line: 57 } |  |  | 0.550 |
| walker |  | 5591 | 23 | Code::CodeKey { rung: Decl, file: src/flask/globals.py, decl: 8, sub: 0, line: 60 } |  |  | 0.550 |
| walker |  | 5641 | 50 | Code::CodeKey { rung: Decl, file: src/flask/globals.py, decl: 6, sub: 0, line: 51 } |  |  | 0.550 |
| ns | 5659 |  | 172 | `flask routes` options + the injected --debug flag (src/flask/cli.py:1001, 1049-1060) | 4.10 | 4.6 | 0.541 |
| walker |  | 5715 | 74 | Code::CodeKey { rung: Decl, file: src/flask/globals.py, decl: 1, sub: 0, line: 33 } |  |  | 0.541 |
| ns | 5720 |  | 61 | CI and repo-automation inventory (.github, workflows, issue templates, .devcontainer — complete) | 4.11 |  | 0.554 |
| walker |  | 5758 | 43 | Code::CodeKey { rung: Body, file: src/flask/json/__init__.py, decl: 4, sub: 0, line: 108 } |  |  | 0.554 |
| ns | 5919 |  | 199 | CI interpreter matrix (.github/workflows/tests.yaml:19-29) | 4.12 | 4.11 | 0.547 |
| ns | 6063 |  | 144 | Flask methods 1/3: construction, resources, jinja, URL adapter, run (src/flask/app.py:254-632) | 5.1 |  | 0.558 |
| walker |  | 6081 | 323 | Code::CodeKey { rung: Names, file: src/flask/typing.py, decl: 0, sub: 0, line: 0 } |  |  | 0.558 |
| walker |  | 6116 | 35 | Code::CodeKey { rung: Decl, file: src/flask/typing.py, decl: 19, sub: 0, line: 84 } |  |  | 0.558 |
| walker |  | 6153 | 37 | Code::CodeKey { rung: Decl, file: src/flask/typing.py, decl: 12, sub: 0, line: 64 } |  |  | 0.558 |
| walker |  | 6192 | 39 | Code::CodeKey { rung: Decl, file: src/flask/typing.py, decl: 3, sub: 0, line: 29 } |  |  | 0.558 |
| ns | 6206 |  | 143 | Flask methods 2/3: test clients, error handling, request dispatch (src/flask/app.py:755-1079) | 5.2 | 5.1 | 0.568 |
| walker |  | 6231 | 39 | Code::CodeKey { rung: Decl, file: src/flask/typing.py, decl: 7, sub: 0, line: 50 } |  |  | 0.568 |
| walker |  | 6272 | 41 | Code::CodeKey { rung: Decl, file: src/flask/typing.py, decl: 9, sub: 0, line: 55 } |  |  | 0.568 |
| walker |  | 6313 | 41 | Code::CodeKey { rung: Decl, file: src/flask/typing.py, decl: 11, sub: 0, line: 60 } |  |  | 0.568 |
| ns | 6345 |  | 139 | Flask methods 3/3: url_for, make_response, teardown, contexts, WSGI entry (src/flask/app.py:1102-1618) | 5.3 | 5.2 | 0.576 |
| walker |  | 6354 | 41 | Code::CodeKey { rung: Decl, file: src/flask/typing.py, decl: 18, sub: 0, line: 79 } |  |  | 0.576 |
| walker |  | 6417 | 63 | Code::CodeKey { rung: Decl, file: src/flask/typing.py, decl: 4, sub: 0, line: 36 } |  |  | 0.576 |
| ns | 6532 |  | 187 | App (sansio) methods 1/2: name, logger, jinja env, config/aborter factories, blueprint registration (sansio/app.py:279-605) | 5.4 |  | 0.567 |
| walker |  | 6536 | 119 | Code::CodeKey { rung: Decl, file: src/flask/typing.py, decl: 1, sub: 0, line: 12 } |  |  | 0.567 |
| walker |  | 6560 | 24 | Code::CodeKey { rung: Names, file: src/flask/wrappers.py, decl: 0, sub: 0, line: 0 } |  |  | 0.567 |
| walker |  | 6633 | 73 | Code::CodeKey { rung: Decl, file: src/flask/wrappers.py, decl: 13, sub: 0, line: 222 } |  |  | 0.567 |
| walker |  | 6639 | 6 | Code::CodeKey { rung: Decl, file: src/flask/wrappers.py, decl: 14, sub: 0, line: 246 } |  |  | 0.567 |
| ns | 6751 |  | 219 | App (sansio) methods 2/2: template filter/test/global decorators, teardown, error routing (sansio/app.py:664-981) | 5.5 | 5.4 | 0.556 |
| walker |  | 6838 | 199 | Code::CodeKey { rung: Decl, file: src/flask/wrappers.py, decl: 1, sub: 0, line: 18 } |  |  | 0.556 |
| ns | 6936 |  | 185 | Every module-level function in src/flask/helpers.py (17 definitions) | 5.6 |  | 0.547 |
| walker |  | 7024 | 186 | Code::CodeKey { rung: Decl, file: src/flask/wrappers.py, decl: 1, sub: 1, line: 18 } |  |  | 0.547 |
| ns | 7064 |  | 128 | Blueprint (sansio) methods 1/2: setup state, deferred registration, register (sansio/blueprints.py:41-413) | 5.7 |  | 0.541 |
| walker |  | 7252 | 228 | Code::CodeKey { rung: Decl, file: src/flask/wrappers.py, decl: 1, sub: 2, line: 18 } |  |  | 0.542 |
| walker |  | 7258 | 6 | Code::CodeKey { rung: Decl, file: src/flask/wrappers.py, decl: 2, sub: 0, line: 59 } |  |  | 0.542 |
| walker |  | 7266 | 8 | Code::CodeKey { rung: Decl, file: src/flask/wrappers.py, decl: 4, sub: 0, line: 92 } |  |  | 0.542 |
| walker |  | 7274 | 8 | Code::CodeKey { rung: Decl, file: src/flask/wrappers.py, decl: 8, sub: 0, line: 146 } |  |  | 0.542 |
| walker |  | 7282 | 8 | Code::CodeKey { rung: Decl, file: src/flask/wrappers.py, decl: 9, sub: 0, line: 161 } |  |  | 0.542 |
| walker |  | 7290 | 8 | Code::CodeKey { rung: Decl, file: src/flask/wrappers.py, decl: 10, sub: 0, line: 180 } |  |  | 0.542 |
| ns | 7297 |  | 233 | Blueprint (sansio) methods 2/2: the app_* decorators (sansio/blueprints.py:444-685) | 5.8 | 5.7 | 0.533 |
| walker |  | 7301 | 11 | Code::CodeKey { rung: Decl, file: src/flask/wrappers.py, decl: 3, sub: 0, line: 88 } |  |  | 0.533 |
| walker |  | 7312 | 11 | Code::CodeKey { rung: Decl, file: src/flask/wrappers.py, decl: 7, sub: 0, line: 142 } |  |  | 0.533 |
| walker |  | 7324 | 12 | Code::CodeKey { rung: Decl, file: src/flask/wrappers.py, decl: 5, sub: 0, line: 115 } |  |  | 0.533 |
| walker |  | 7339 | 15 | Code::CodeKey { rung: Decl, file: src/flask/wrappers.py, decl: 6, sub: 0, line: 119 } |  |  | 0.533 |
| ns | 7489 |  | 192 | The IO-side Blueprint overrides + the class-based view surface (blueprints.py, views.py) | 5.9 | 2.2 | 0.527 |
| walker |  | 7630 | 291 | Code::CodeKey { rung: Names, file: src/flask/helpers.py, decl: 0, sub: 0, line: 0 } |  |  | 0.549 |
| walker |  | 7637 | 7 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 17, sub: 0, line: 644 } |  |  | 0.549 |
| walker |  | 7674 | 37 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 3, sub: 0, line: 51 } |  |  | 0.549 |
| walker |  | 7711 | 37 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 8, sub: 0, line: 254 } |  |  | 0.549 |
| ns | 7742 |  | 253 | Request/Response attributes and properties (src/flask/wrappers.py) | 5.10 | 2.7 | 0.562 |
| walker |  | 7750 | 39 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 12, sub: 0, line: 360 } |  |  | 0.562 |
| walker |  | 7798 | 48 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 4, sub: 0, line: 57 } |  |  | 0.562 |
| walker |  | 7847 | 49 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 15, sub: 0, line: 543 } |  |  | 0.562 |
| walker |  | 7902 | 55 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 5, sub: 0, line: 63 } |  |  | 0.562 |
| walker |  | 7964 | 62 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 18, sub: 0, line: 654 } |  |  | 0.562 |
| walker |  | 8023 | 59 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 21, sub: 0, line: 665 } |  |  | 0.562 |
| ns | 8041 |  | 299 | The session interface: SessionMixin/SecureCookieSession state + every SessionInterface method (sessions.py:28-263) | 5.11 | 2.7 | 0.550 |
| walker |  | 8113 | 90 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 7, sub: 0, line: 200 } |  |  | 0.550 |
| ns | 8152 |  | 111 | The default cookie-signing session implementation (sessions.py:276-337) | 5.12 | 5.11 | 0.546 |
| walker |  | 8256 | 143 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 14, sub: 0, line: 417 } |  |  | 0.546 |
| walker |  | 8269 | 13 | Code::CodeKey { rung: Doc, file: src/flask/helpers.py, decl: 22, sub: 0, line: 676 } |  |  | 0.546 |
| ns | 8367 |  | 215 | The JSON subpackage API: module functions, provider methods, provider knobs | 5.13 | 2.9 | 0.538 |
| walker |  | 8419 | 150 | Code::CodeKey { rung: Names, file: src/flask/ctx.py, decl: 0, sub: 0, line: 0 } |  |  | 0.539 |
| walker |  | 8446 | 27 | Code::CodeKey { rung: Decl, file: src/flask/ctx.py, decl: 12, sub: 0, line: 118 } |  |  | 0.539 |
| ns | 8509 |  | 142 | The test-support API: EnvironBuilder, FlaskClient, FlaskCliRunner members (src/flask/testing.py) | 5.14 | 2.8 | 0.534 |
| walker |  | 8662 | 216 | Code::CodeKey { rung: Decl, file: src/flask/ctx.py, decl: 17, sub: 0, line: 260 } |  |  | 0.535 |
| walker |  | 8670 | 8 | Code::CodeKey { rung: Decl, file: src/flask/ctx.py, decl: 19, sub: 0, line: 339 } |  |  | 0.535 |
| walker |  | 8678 | 8 | Code::CodeKey { rung: Decl, file: src/flask/ctx.py, decl: 20, sub: 0, line: 350 } |  |  | 0.535 |
| walker |  | 8686 | 8 | Code::CodeKey { rung: Decl, file: src/flask/ctx.py, decl: 22, sub: 0, line: 370 } |  |  | 0.535 |
| walker |  | 8694 | 8 | Code::CodeKey { rung: Decl, file: src/flask/ctx.py, decl: 24, sub: 0, line: 395 } |  |  | 0.535 |
| ns | 8737 |  | 228 | The context proxies: how current_app, g, request, session are bound (src/flask/globals.py:40-49, 57-62) | 6.1 | 1.6 | 0.545 |
| walker |  | 8752 | 58 | Code::CodeKey { rung: Decl, file: src/flask/ctx.py, decl: 18, sub: 0, line: 300 } |  |  | 0.545 |
| walker |  | 8810 | 58 | Code::CodeKey { rung: Decl, file: src/flask/ctx.py, decl: 29, sub: 0, line: 510 } |  |  | 0.545 |
| ns | 8967 |  | 230 | The context API: ctx.py module functions, the `g` object, AppContext members | 6.2 | 2.7 | 0.552 |
| walker |  | 9016 | 206 | Code::CodeKey { rung: Decl, file: src/flask/ctx.py, decl: 2, sub: 0, line: 30 } |  |  | 0.557 |
| walker |  | 9032 | 16 | Code::CodeKey { rung: Doc, file: src/flask/ctx.py, decl: 20, sub: 0, line: 350 } |  |  | 0.557 |
| walker |  | 9052 | 20 | Code::CodeKey { rung: Doc, file: src/flask/ctx.py, decl: 23, sub: 0, line: 381 } |  |  | 0.557 |
| walker |  | 9207 | 155 | Code::CodeKey { rung: Names, file: src/flask/templating.py, decl: 0, sub: 0, line: 0 } |  |  | 0.558 |
| ns | 9223 |  | 256 | What changed in the unreleased 3.2.0 (CHANGES.rst:1-16) | 6.3 |  | 0.551 |
| walker |  | 9233 | 26 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 2, sub: 0, line: 36 } |  |  | 0.551 |
| walker |  | 9265 | 32 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 14, sub: 0, line: 163 } |  |  | 0.551 |
| walker |  | 9303 | 38 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 12, sub: 0, line: 136 } |  |  | 0.551 |
| walker |  | 9343 | 40 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 15, sub: 0, line: 181 } |  |  | 0.551 |
| ns | 9399 |  | 176 | Complete docs/ listing (35 entries) | 6.4 |  | 0.569 |
| walker |  | 9448 | 105 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 4, sub: 0, line: 49 } |  |  | 0.569 |
| walker |  | 9488 | 40 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 6, sub: 0, line: 57 } |  |  | 0.569 |
| walker |  | 9528 | 40 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 7, sub: 0, line: 64 } |  |  | 0.569 |
| ns | 9544 |  | 145 | Complete tests/ listing (27 entries) | 6.5 |  | 0.581 |
| walker |  | 9568 | 40 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 8, sub: 0, line: 88 } |  |  | 0.581 |
| walker |  | 9576 | 8 | Code::CodeKey { rung: Body, file: src/flask/templating.py, decl: 5, sub: 0, line: 54 } |  |  | 0.581 |
| walker |  | 9607 | 31 | Code::CodeKey { rung: Doc, file: src/flask/templating.py, decl: 4, sub: 0, line: 49 } |  |  | 0.581 |
| ns | 9611 |  | 67 | The three runnable example applications (examples/ and each app package) | 6.6 |  | 0.589 |
| walker |  | 9612 | 5 | Code::CodeKey { rung: Body, file: src/flask/helpers.py, decl: 20, sub: 0, line: 662 } |  |  | 0.589 |
| walker |  | 9718 | 106 | Code::CodeKey { rung: Names, file: src/flask/debughelpers.py, decl: 0, sub: 0, line: 0 } |  |  | 0.591 |
| walker |  | 9738 | 20 | Code::CodeKey { rung: Decl, file: src/flask/debughelpers.py, decl: 5, sub: 0, line: 50 } |  |  | 0.591 |
| walker |  | 9778 | 40 | Code::CodeKey { rung: Decl, file: src/flask/debughelpers.py, decl: 2, sub: 0, line: 23 } |  |  | 0.591 |
| walker |  | 9872 | 94 | Code::CodeKey { rung: Decl, file: src/flask/debughelpers.py, decl: 9, sub: 0, line: 124 } |  |  | 0.591 |
| walker |  | 9879 | 7 | Code::CodeKey { rung: Body, file: src/flask/debughelpers.py, decl: 4, sub: 0, line: 46 } |  |  | 0.591 |
| walker |  | 9893 | 14 | Code::CodeKey { rung: Doc, file: src/flask/debughelpers.py, decl: 9, sub: 0, line: 124 } |  |  | 0.591 |
| ns | 9979 |  | 368 | The canonical application factory (examples/tutorial/flaskr/__init__.py:1-48, comments elided) | 6.7 | 6.6 | 0.577 |
| walker |  | 9991 | 98 | Code::CodeKey { rung: Names, file: src/flask/cli.py, decl: 0, sub: 0, line: 0 } |  |  | 0.578 |

Score(3000)=0.712 I=0.922 C=0.550 ns_rows≤3K=18/57 grid(1000/1442/2080/3000/4327/6240/9000)=0.734/0.628/0.692/0.712/0.557/0.509/0.538

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
| ns | 541 |  | 146 | Canonical usage example: @app.route + `flask run` (README.md:20-36) | 1.5 |  | 0.783 |
| walker |  | 599 | 122 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.858 |
| ns | 707 |  | 166 | Public export surface 1/3: Flask, Blueprint, Config, json, context fns, globals (src/flask/__init__.py:1-12) | 1.6 |  | 0.784 |
| walker |  | 775 | 176 | Fs::DirListing { dir: docs } |  |  | 0.788 |
| walker |  | 802 | 27 | Fs::DirListing { dir: docs/_static } |  |  | 0.788 |
| ns | 865 |  | 158 | Public export surface 2/3: the `helpers` free functions + jsonify (src/flask/__init__.py:13-23) | 1.7 | 1.6 | 0.733 |
| walker |  | 866 | 64 | Fs::DirListing { dir: docs/deploying } |  |  | 0.733 |
| walker |  | 944 | 78 | Fs::DirListing { dir: docs/tutorial } |  |  | 0.733 |
| walker |  | 958 | 14 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.734 |
| walker |  | 1103 | 145 | Fs::DirListing { dir: docs/patterns } |  |  | 0.734 |
| ns | 1121 |  | 256 | Public export surface 3/3: all ten signals, template renderers, Request/Response (src/flask/__init__.py:24-39) | 1.8 | 1.7 | 0.671 |
| walker |  | 1127 | 24 | Fs::DirListing { dir: examples/celery } |  |  | 0.671 |
| walker |  | 1131 | 4 | Fs::DirListing { dir: examples/celery/src } |  |  | 0.671 |
| walker |  | 1148 | 17 | Fs::DirListing { dir: examples/celery/src/task_app } |  |  | 0.671 |
| walker |  | 1293 | 145 | Fs::DirListing { dir: tests } |  |  | 0.673 |
| walker |  | 1312 | 19 | Fs::DirListing { dir: tests/type_check } |  |  | 0.673 |
| walker |  | 1339 | 27 | Fs::DirListing { dir: examples/javascript } |  |  | 0.673 |
| walker |  | 1351 | 12 | Fs::DirListing { dir: examples/javascript/tests } |  |  | 0.673 |
| walker |  | 1364 | 13 | Fs::DirListing { dir: examples/javascript/js_example } |  |  | 0.673 |
| walker |  | 1391 | 27 | Fs::DirListing { dir: examples/tutorial } |  |  | 0.619 |
| ns | 1391 |  | 270 | Package identity + runtime dependency stack (pyproject.toml:1-8, 22-34) | 1.9 |  | 0.619 |
| walker |  | 1419 | 28 | Fs::DirListing { dir: examples/tutorial/flaskr } |  |  | 0.620 |
| ns | 1468 |  | 77 | Why src/flask/sansio/ exists (src/flask/sansio/README.md:1-6) | 2.1 |  | 0.604 |
| ns | 1545 |  | 77 | The class spine: Scaffold -> App -> Flask and Scaffold -> Blueprint -> Blueprint (6 class statements) | 2.2 |  | 0.590 |
| walker |  | 1564 | 145 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.656 |
| walker |  | 1593 | 29 | Fs::DirListing { dir: tests/test_apps } |  |  | 0.656 |
| walker |  | 1602 | 9 | Fs::DirListing { dir: tests/test_apps/blueprintapp } |  |  | 0.656 |
| walker |  | 1611 | 9 | Fs::DirListing { dir: tests/test_apps/helloworld } |  |  | 0.656 |
| walker |  | 1620 | 9 | Fs::DirListing { dir: tests/test_apps/subdomaintestmodule } |  |  | 0.656 |
| walker |  | 1634 | 14 | Fs::DirListing { dir: tests/test_apps/blueprintapp/apps } |  |  | 0.656 |
| ns | 1639 |  | 94 | Scaffold routing decorators — every URL-registration method, names only (sansio/scaffold.py:284-436) | 2.3 | 2.2 | 0.634 |
| walker |  | 1643 | 9 | Fs::DirListing { dir: tests/test_apps/blueprintapp/apps/frontend } |  |  | 0.634 |
| walker |  | 1655 | 12 | Fs::DirListing { dir: tests/test_apps/blueprintapp/apps/admin } |  |  | 0.634 |
| walker |  | 1685 | 30 | Fs::DirListing { dir: examples/tutorial/tests } |  |  | 0.634 |
| walker |  | 1720 | 35 | Fs::DirListing { dir: tests/test_apps/cliapp } |  |  | 0.634 |
| walker |  | 1730 | 10 | Fs::DirListing { dir: tests/test_apps/cliapp/inner1 } |  |  | 0.634 |
| walker |  | 1742 | 12 | Fs::DirListing { dir: tests/test_apps/cliapp/inner1/inner2 } |  |  | 0.634 |
| ns | 1745 |  | 106 | Scaffold request-hook and error-handler decorators, names only (sansio/scaffold.py:460-657) | 2.4 | 2.2 | 0.613 |
| ns | 1896 |  | 151 | Rest of sansio/scaffold.py: constructor, static/template properties, module-level helpers | 2.5 | 2.2 | 0.586 |
| walker |  | 1952 | 210 | Code::CodeKey { rung: Names, file: src/flask/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.646 |
| ns | 2112 |  | 216 | The complete signal registry (src/flask/signals.py:1-16, whole file) | 2.6 |  | 0.614 |
| walker |  | 2138 | 186 | Code::CodeKey { rung: Names, file: src/flask/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.662 |
| ns | 2294 |  | 182 | Class roster A: request/response, views, sessions, context, config (13 class statements) | 2.7 |  | 0.637 |
| walker |  | 2322 | 184 | Code::CodeKey { rung: Names, file: src/flask/__init__.py, decl: 0, sub: 2, line: 0 } |  |  | 0.694 |
| walker |  | 2457 | 135 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.763 |
| ns | 2510 |  | 216 | Class roster B: templating, testing, CLI, debug helpers (14 class statements) | 2.8 |  | 0.732 |
| walker |  | 2606 | 149 | Code::CodeKey { rung: Names, file: src/flask/json/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.732 |
| walker |  | 2631 | 25 | Code::CodeKey { rung: Body, file: src/flask/json/__init__.py, decl: 5, sub: 0, line: 138 } |  |  | 0.732 |
| walker |  | 2655 | 24 | Code::CodeKey { rung: Names, file: src/flask/wrappers.py, decl: 0, sub: 0, line: 0 } |  |  | 0.711 |
| ns | 2655 |  | 145 | Class roster C: the whole src/flask/json subpackage (12 class statements) | 2.9 |  | 0.711 |
| walker |  | 2728 | 73 | Code::CodeKey { rung: Decl, file: src/flask/wrappers.py, decl: 13, sub: 0, line: 222 } |  |  | 0.711 |
| walker |  | 2734 | 6 | Code::CodeKey { rung: Decl, file: src/flask/wrappers.py, decl: 14, sub: 0, line: 246 } |  |  | 0.711 |
| walker |  | 2777 | 43 | Code::CodeKey { rung: Body, file: src/flask/json/__init__.py, decl: 3, sub: 0, line: 77 } |  |  | 0.711 |
| walker |  | 2820 | 43 | Code::CodeKey { rung: Body, file: src/flask/json/__init__.py, decl: 4, sub: 0, line: 108 } |  |  | 0.711 |
| walker |  | 2835 | 15 | Code::CodeKey { rung: Names, file: src/flask/blueprints.py, decl: 0, sub: 0, line: 0 } |  |  | 0.712 |
| walker |  | 2901 | 66 | Code::CodeKey { rung: Decl, file: src/flask/blueprints.py, decl: 1, sub: 0, line: 18 } |  |  | 0.712 |
| walker |  | 2943 | 42 | Code::CodeKey { rung: Decl, file: src/flask/blueprints.py, decl: 5, sub: 0, line: 104 } |  |  | 0.712 |
| ns | 3085 |  | 430 | Flask.default_config — all 29 config keys with their defaults (src/flask/app.py:206-238) | 3.1 |  | 0.659 |
| walker |  | 3116 | 173 | Code::CodeKey { rung: Decl, file: src/flask/blueprints.py, decl: 2, sub: 0, line: 19 } |  |  | 0.659 |
| ns | 3246 |  | 161 | Every Config loader method + the ConfigAttribute descriptor (src/flask/config.py:23-366) | 3.2 | 3.1 | 0.640 |
| walker |  | 3407 | 291 | Code::CodeKey { rung: Names, file: src/flask/helpers.py, decl: 0, sub: 0, line: 0 } |  |  | 0.642 |
| walker |  | 3414 | 7 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 17, sub: 0, line: 644 } |  |  | 0.642 |
| walker |  | 3451 | 37 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 3, sub: 0, line: 51 } |  |  | 0.642 |
| walker |  | 3488 | 37 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 8, sub: 0, line: 254 } |  |  | 0.642 |
| walker |  | 3527 | 39 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 12, sub: 0, line: 360 } |  |  | 0.642 |
| walker |  | 3575 | 48 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 4, sub: 0, line: 57 } |  |  | 0.642 |
| ns | 3588 |  | 342 | Every App/Flask class-level customization hook (sansio/app.py:164-277, app.py:242-252) | 3.3 | 3.1 | 0.616 |
| walker |  | 3624 | 49 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 15, sub: 0, line: 543 } |  |  | 0.616 |
| walker |  | 3679 | 55 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 5, sub: 0, line: 63 } |  |  | 0.616 |
| walker |  | 3741 | 62 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 18, sub: 0, line: 654 } |  |  | 0.616 |
| ns | 3756 |  | 168 | Config semantics: dict subclass, uppercase-keys-only rule (src/flask/config.py:51-56, 68-73) | 3.4 | 3.2 | 0.601 |
| walker |  | 3800 | 59 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 21, sub: 0, line: 665 } |  |  | 0.601 |
| walker |  | 3890 | 90 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 7, sub: 0, line: 200 } |  |  | 0.601 |
| ns | 3941 |  | 185 | Environment-variable config: from_prefixed_env behaviour (src/flask/config.py:129-141) | 3.5 | 3.2 | 0.587 |
| ns | 3993 |  | 52 | Concrete config-file samples used by the test suite (tests/static/config.json, tests/static/config.toml) | 3.6 |  | 0.581 |
| walker |  | 4033 | 143 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 14, sub: 0, line: 417 } |  |  | 0.581 |
| walker |  | 4046 | 13 | Code::CodeKey { rung: Doc, file: src/flask/helpers.py, decl: 22, sub: 0, line: 676 } |  |  | 0.581 |
| ns | 4088 |  | 95 | Console entry point + build backend (pyproject.toml:81-89) | 4.1 |  | 0.573 |
| walker |  | 4102 | 56 | Code::CodeKey { rung: Body, file: src/flask/json/__init__.py, decl: 1, sub: 0, line: 13 } |  |  | 0.573 |
| walker |  | 4252 | 150 | Code::CodeKey { rung: Names, file: src/flask/ctx.py, decl: 0, sub: 0, line: 0 } |  |  | 0.575 |
| walker |  | 4279 | 27 | Code::CodeKey { rung: Decl, file: src/flask/ctx.py, decl: 12, sub: 0, line: 118 } |  |  | 0.575 |
| ns | 4297 |  | 209 | How tests are run: tox env_list + the pytest command (pyproject.toml:170-182, 190-193) | 4.2 |  | 0.557 |
| walker |  | 4485 | 206 | Code::CodeKey { rung: Decl, file: src/flask/ctx.py, decl: 2, sub: 0, line: 30 } |  |  | 0.557 |
| walker |  | 4494 | 9 | Code::CodeKey { rung: Body, file: src/flask/ctx.py, decl: 10, sub: 0, line: 108 } |  |  | 0.557 |
| ns | 4509 |  | 212 | The style / typing / docs tox environments (pyproject.toml:233-250) | 4.3 | 4.2 | 0.541 |
| ns | 4664 |  | 155 | Test + type-checking settings: pytest, mypy strict, pyright (pyproject.toml:106-110, 126-131, 142-145) | 4.4 |  | 0.528 |
| walker |  | 4710 | 216 | Code::CodeKey { rung: Decl, file: src/flask/ctx.py, decl: 17, sub: 0, line: 260 } |  |  | 0.529 |
| walker |  | 4718 | 8 | Code::CodeKey { rung: Decl, file: src/flask/ctx.py, decl: 19, sub: 0, line: 339 } |  |  | 0.529 |
| walker |  | 4726 | 8 | Code::CodeKey { rung: Decl, file: src/flask/ctx.py, decl: 20, sub: 0, line: 350 } |  |  | 0.529 |
| walker |  | 4734 | 8 | Code::CodeKey { rung: Decl, file: src/flask/ctx.py, decl: 22, sub: 0, line: 370 } |  |  | 0.529 |
| walker |  | 4742 | 8 | Code::CodeKey { rung: Decl, file: src/flask/ctx.py, decl: 24, sub: 0, line: 395 } |  |  | 0.529 |
| walker |  | 4800 | 58 | Code::CodeKey { rung: Decl, file: src/flask/ctx.py, decl: 18, sub: 0, line: 300 } |  |  | 0.529 |
| ns | 4848 |  | 184 | The `flask` command object and its app-discovery help text (src/flask/cli.py:1110-1127) | 4.5 |  | 0.515 |
| walker |  | 4858 | 58 | Code::CodeKey { rung: Decl, file: src/flask/ctx.py, decl: 29, sub: 0, line: 510 } |  |  | 0.515 |
| walker |  | 4874 | 16 | Code::CodeKey { rung: Doc, file: src/flask/ctx.py, decl: 20, sub: 0, line: 350 } |  |  | 0.515 |
| walker |  | 4894 | 20 | Code::CodeKey { rung: Doc, file: src/flask/ctx.py, decl: 23, sub: 0, line: 381 } |  |  | 0.515 |
| ns | 4949 |  | 101 | The complete built-in command set: run, shell, routes (src/flask/cli.py:594-596, 882, 999, 1048) | 4.6 | 4.5 | 0.511 |
| walker |  | 5080 | 186 | Code::CodeKey { rung: Names, file: src/flask/signals.py, decl: 0, sub: 0, line: 0 } |  |  | 0.531 |
| ns | 5097 |  | 148 | Every `flask run` flag, declaration lines only (src/flask/cli.py:883-925) | 4.7 | 4.6 | 0.524 |
| ns | 5241 |  | 144 | The --app option: module:name form and factory auto-detection (src/flask/cli.py:453-462) | 4.8 | 4.5 | 0.517 |
| walker |  | 5378 | 298 | Plaintext::Whole { file: docs/Makefile } |  |  | 0.517 |
| ns | 5487 |  | 246 | The remaining global options: --debug/--no-debug, -e/--env-file, --version (src/flask/cli.py:485-489, 517-527, 283-286) | 4.9 | 4.8 | 0.504 |
| walker |  | 5533 | 155 | Code::CodeKey { rung: Names, file: src/flask/templating.py, decl: 0, sub: 0, line: 0 } |  |  | 0.505 |
| walker |  | 5559 | 26 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 2, sub: 0, line: 36 } |  |  | 0.505 |
| walker |  | 5591 | 32 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 14, sub: 0, line: 163 } |  |  | 0.505 |
| walker |  | 5629 | 38 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 12, sub: 0, line: 136 } |  |  | 0.505 |
| ns | 5659 |  | 172 | `flask routes` options + the injected --debug flag (src/flask/cli.py:1001, 1049-1060) | 4.10 | 4.6 | 0.497 |
| walker |  | 5669 | 40 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 15, sub: 0, line: 181 } |  |  | 0.497 |
| ns | 5720 |  | 61 | CI and repo-automation inventory (.github, workflows, issue templates, .devcontainer — complete) | 4.11 |  | 0.512 |
| walker |  | 5774 | 105 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 4, sub: 0, line: 49 } |  |  | 0.512 |
| walker |  | 5814 | 40 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 6, sub: 0, line: 57 } |  |  | 0.512 |
| walker |  | 5854 | 40 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 7, sub: 0, line: 64 } |  |  | 0.512 |
| walker |  | 5894 | 40 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 8, sub: 0, line: 88 } |  |  | 0.512 |
| ns | 5919 |  | 199 | CI interpreter matrix (.github/workflows/tests.yaml:19-29) | 4.12 | 4.11 | 0.505 |
| walker |  | 5950 | 56 | Code::CodeKey { rung: Names, file: src/flask/config.py, decl: 0, sub: 0, line: 0 } |  |  | 0.508 |
| ns | 6063 |  | 144 | Flask methods 1/3: construction, resources, jinja, URL adapter, run (src/flask/app.py:254-632) | 5.1 |  | 0.501 |
| walker |  | 6080 | 130 | Code::CodeKey { rung: Decl, file: src/flask/config.py, decl: 2, sub: 0, line: 20 } |  |  | 0.504 |
| walker |  | 6088 | 8 | Code::CodeKey { rung: Decl, file: src/flask/config.py, decl: 5, sub: 0, line: 32 } |  |  | 0.504 |
| walker |  | 6098 | 10 | Code::CodeKey { rung: Decl, file: src/flask/config.py, decl: 4, sub: 0, line: 29 } |  |  | 0.504 |
| walker |  | 6135 | 37 | Code::CodeKey { rung: Decl, file: src/flask/config.py, decl: 3, sub: 0, line: 23 } |  |  | 0.504 |
| ns | 6206 |  | 143 | Flask methods 2/3: test clients, error handling, request dispatch (src/flask/app.py:755-1079) | 5.2 | 5.1 | 0.497 |
| walker |  | 6268 | 133 | Code::CodeKey { rung: Decl, file: src/flask/config.py, decl: 8, sub: 0, line: 50 } |  |  | 0.519 |
| walker |  | 6299 | 31 | Code::CodeKey { rung: Decl, file: src/flask/config.py, decl: 12, sub: 0, line: 187 } |  |  | 0.519 |
| walker |  | 6335 | 36 | Code::CodeKey { rung: Decl, file: src/flask/config.py, decl: 15, sub: 0, line: 304 } |  |  | 0.519 |
| ns | 6345 |  | 139 | Flask methods 3/3: url_for, make_response, teardown, contexts, WSGI entry (src/flask/app.py:1102-1618) | 5.3 | 5.2 | 0.513 |
| walker |  | 6372 | 37 | Code::CodeKey { rung: Decl, file: src/flask/config.py, decl: 16, sub: 0, line: 323 } |  |  | 0.513 |
| walker |  | 6412 | 40 | Code::CodeKey { rung: Decl, file: src/flask/config.py, decl: 11, sub: 0, line: 126 } |  |  | 0.513 |
| walker |  | 6461 | 49 | Code::CodeKey { rung: Decl, file: src/flask/config.py, decl: 9, sub: 0, line: 94 } |  |  | 0.513 |
| ns | 6532 |  | 187 | App (sansio) methods 1/2: name, logger, jinja env, config/aborter factories, blueprint registration (sansio/app.py:279-605) | 5.4 |  | 0.504 |
| walker |  | 6536 | 75 | Code::CodeKey { rung: Decl, file: src/flask/config.py, decl: 14, sub: 0, line: 256 } |  |  | 0.504 |
| walker |  | 6548 | 12 | Code::CodeKey { rung: Doc, file: src/flask/config.py, decl: 2, sub: 0, line: 20 } |  |  | 0.504 |
| ns | 6751 |  | 219 | App (sansio) methods 2/2: template filter/test/global decorators, teardown, error routing (sansio/app.py:664-981) | 5.5 | 5.4 | 0.494 |
| walker |  | 6755 | 207 | Code::CodeKey { rung: Names, file: src/flask/app.py, decl: 0, sub: 0, line: 0 } |  |  | 0.495 |
| walker |  | 6778 | 23 | Code::CodeKey { rung: Decl, file: src/flask/app.py, decl: 1, sub: 0, line: 64 } |  |  | 0.495 |
| ns | 6936 |  | 185 | Every module-level function in src/flask/helpers.py (17 definitions) | 5.6 |  | 0.511 |
| walker |  | 6965 | 187 | Code::CodeKey { rung: Names, file: src/flask/globals.py, decl: 0, sub: 0, line: 0 } |  |  | 0.511 |
| walker |  | 6985 | 20 | Code::CodeKey { rung: Decl, file: src/flask/globals.py, decl: 3, sub: 0, line: 41 } |  |  | 0.511 |
| walker |  | 7008 | 23 | Code::CodeKey { rung: Decl, file: src/flask/globals.py, decl: 4, sub: 0, line: 44 } |  |  | 0.511 |
| walker |  | 7031 | 23 | Code::CodeKey { rung: Decl, file: src/flask/globals.py, decl: 5, sub: 0, line: 47 } |  |  | 0.511 |
| walker |  | 7054 | 23 | Code::CodeKey { rung: Decl, file: src/flask/globals.py, decl: 7, sub: 0, line: 57 } |  |  | 0.512 |
| ns | 7064 |  | 128 | Blueprint (sansio) methods 1/2: setup state, deferred registration, register (sansio/blueprints.py:41-413) | 5.7 |  | 0.506 |
| walker |  | 7077 | 23 | Code::CodeKey { rung: Decl, file: src/flask/globals.py, decl: 8, sub: 0, line: 60 } |  |  | 0.506 |
| walker |  | 7127 | 50 | Code::CodeKey { rung: Decl, file: src/flask/globals.py, decl: 6, sub: 0, line: 51 } |  |  | 0.506 |
| walker |  | 7201 | 74 | Code::CodeKey { rung: Decl, file: src/flask/globals.py, decl: 1, sub: 0, line: 33 } |  |  | 0.506 |
| ns | 7297 |  | 233 | Blueprint (sansio) methods 2/2: the app_* decorators (sansio/blueprints.py:444-685) | 5.8 | 5.7 | 0.497 |
| walker |  | 7400 | 199 | Code::CodeKey { rung: Decl, file: src/flask/wrappers.py, decl: 1, sub: 0, line: 18 } |  |  | 0.498 |
| ns | 7489 |  | 192 | The IO-side Blueprint overrides + the class-based view surface (blueprints.py, views.py) | 5.9 | 2.2 | 0.493 |
| walker |  | 7586 | 186 | Code::CodeKey { rung: Decl, file: src/flask/wrappers.py, decl: 1, sub: 1, line: 18 } |  |  | 0.493 |
| ns | 7742 |  | 253 | Request/Response attributes and properties (src/flask/wrappers.py) | 5.10 | 2.7 | 0.490 |
| walker |  | 7814 | 228 | Code::CodeKey { rung: Decl, file: src/flask/wrappers.py, decl: 1, sub: 2, line: 18 } |  |  | 0.510 |
| walker |  | 7820 | 6 | Code::CodeKey { rung: Decl, file: src/flask/wrappers.py, decl: 2, sub: 0, line: 59 } |  |  | 0.510 |
| walker |  | 7828 | 8 | Code::CodeKey { rung: Decl, file: src/flask/wrappers.py, decl: 4, sub: 0, line: 92 } |  |  | 0.510 |
| walker |  | 7836 | 8 | Code::CodeKey { rung: Decl, file: src/flask/wrappers.py, decl: 8, sub: 0, line: 146 } |  |  | 0.510 |
| walker |  | 7844 | 8 | Code::CodeKey { rung: Decl, file: src/flask/wrappers.py, decl: 9, sub: 0, line: 161 } |  |  | 0.510 |
| walker |  | 7852 | 8 | Code::CodeKey { rung: Decl, file: src/flask/wrappers.py, decl: 10, sub: 0, line: 180 } |  |  | 0.510 |
| walker |  | 7863 | 11 | Code::CodeKey { rung: Decl, file: src/flask/wrappers.py, decl: 3, sub: 0, line: 88 } |  |  | 0.510 |
| walker |  | 7874 | 11 | Code::CodeKey { rung: Decl, file: src/flask/wrappers.py, decl: 7, sub: 0, line: 142 } |  |  | 0.510 |
| walker |  | 7886 | 12 | Code::CodeKey { rung: Decl, file: src/flask/wrappers.py, decl: 5, sub: 0, line: 115 } |  |  | 0.510 |
| walker |  | 7901 | 15 | Code::CodeKey { rung: Decl, file: src/flask/wrappers.py, decl: 6, sub: 0, line: 119 } |  |  | 0.510 |
| walker |  | 7944 | 43 | Code::CodeKey { rung: Names, file: src/flask/json/provider.py, decl: 0, sub: 0, line: 0 } |  |  | 0.510 |
| ns | 8041 |  | 299 | The session interface: SessionMixin/SecureCookieSession state + every SessionInterface method (sessions.py:28-263) | 5.11 | 2.7 | 0.500 |
| walker |  | 8115 | 171 | Code::CodeKey { rung: Decl, file: src/flask/json/provider.py, decl: 1, sub: 0, line: 19 } |  |  | 0.500 |
| walker |  | 8148 | 33 | Code::CodeKey { rung: Decl, file: src/flask/json/provider.py, decl: 7, sub: 0, line: 75 } |  |  | 0.500 |
| ns | 8152 |  | 111 | The default cookie-signing session implementation (sessions.py:276-337) | 5.12 | 5.11 | 0.496 |
| walker |  | 8158 | 10 | Code::CodeKey { rung: Body, file: src/flask/json/provider.py, decl: 3, sub: 0, line: 41 } |  |  | 0.496 |
| walker |  | 8168 | 10 | Code::CodeKey { rung: Body, file: src/flask/json/provider.py, decl: 5, sub: 0, line: 59 } |  |  | 0.496 |
| walker |  | 8199 | 31 | Code::CodeKey { rung: Doc, file: src/flask/templating.py, decl: 4, sub: 0, line: 49 } |  |  | 0.496 |
| walker |  | 8260 | 61 | Code::CodeKey { rung: Names, file: src/flask/views.py, decl: 0, sub: 0, line: 0 } |  |  | 0.499 |
| walker |  | 8292 | 32 | Code::CodeKey { rung: Decl, file: src/flask/views.py, decl: 2, sub: 0, line: 11 } |  |  | 0.499 |
| walker |  | 8340 | 48 | Code::CodeKey { rung: Decl, file: src/flask/views.py, decl: 6, sub: 0, line: 138 } |  |  | 0.501 |
| ns | 8367 |  | 215 | The JSON subpackage API: module functions, provider methods, provider knobs | 5.13 | 2.9 | 0.498 |
| walker |  | 8400 | 60 | Code::CodeKey { rung: Body, file: src/flask/json/__init__.py, decl: 2, sub: 0, line: 47 } |  |  | 0.498 |
| walker |  | 8413 | 13 | Fs::DirListing { dir: tests/static } |  |  | 0.498 |
| walker |  | 8417 | 4 | Fs::DirListing { dir: examples/tutorial/flaskr/static } |  |  | 0.498 |
| walker |  | 8421 | 4 | Fs::DirListing { dir: tests/test_apps/subdomaintestmodule/static } |  |  | 0.498 |
| ns | 8509 |  | 142 | The test-support API: EnvironBuilder, FlaskClient, FlaskCliRunner members (src/flask/testing.py) | 5.14 | 2.8 | 0.493 |
| walker |  | 8658 | 237 | Code::CodeKey { rung: Decl, file: src/flask/json/provider.py, decl: 10, sub: 0, line: 124 } |  |  | 0.499 |
| ns | 8737 |  | 228 | The context proxies: how current_app, g, request, session are bound (src/flask/globals.py:40-49, 57-62) | 6.1 | 1.6 | 0.511 |
| walker |  | 8862 | 204 | Code::CodeKey { rung: Decl, file: src/flask/json/provider.py, decl: 10, sub: 1, line: 124 } |  |  | 0.522 |
| walker |  | 8944 | 82 | Code::CodeKey { rung: Names, file: src/flask/testing.py, decl: 0, sub: 0, line: 0 } |  |  | 0.524 |
| ns | 8967 |  | 230 | The context API: ctx.py module functions, the `g` object, AppContext members | 6.2 | 2.7 | 0.537 |
| walker |  | 8982 | 38 | Code::CodeKey { rung: Decl, file: src/flask/testing.py, decl: 1, sub: 0, line: 27 } |  |  | 0.538 |
| walker |  | 9024 | 42 | Code::CodeKey { rung: Decl, file: src/flask/testing.py, decl: 14, sub: 0, line: 265 } |  |  | 0.539 |
| walker |  | 9060 | 36 | Code::CodeKey { rung: Decl, file: src/flask/testing.py, decl: 16, sub: 0, line: 275 } |  |  | 0.539 |
| walker |  | 9160 | 100 | Code::CodeKey { rung: Decl, file: src/flask/testing.py, decl: 2, sub: 0, line: 49 } |  |  | 0.539 |
| ns | 9223 |  | 256 | What changed in the unreleased 3.2.0 (CHANGES.rst:1-16) | 6.3 |  | 0.532 |
| walker |  | 9292 | 132 | Code::CodeKey { rung: Decl, file: src/flask/testing.py, decl: 6, sub: 0, line: 109 } |  |  | 0.543 |
| walker |  | 9325 | 33 | Code::CodeKey { rung: Decl, file: src/flask/testing.py, decl: 10, sub: 0, line: 193 } |  |  | 0.543 |
| walker |  | 9366 | 41 | Code::CodeKey { rung: Decl, file: src/flask/testing.py, decl: 8, sub: 0, line: 135 } |  |  | 0.543 |
| ns | 9399 |  | 176 | Complete docs/ listing (35 entries) | 6.4 |  | 0.562 |
| walker |  | 9420 | 54 | Code::CodeKey { rung: Decl, file: src/flask/testing.py, decl: 13, sub: 0, line: 255 } |  |  | 0.562 |
| walker |  | 9483 | 63 | Code::CodeKey { rung: Decl, file: src/flask/testing.py, decl: 11, sub: 0, line: 204 } |  |  | 0.562 |
| ns | 9544 |  | 145 | Complete tests/ listing (27 entries) | 6.5 |  | 0.574 |
| ns | 9611 |  | 67 | The three runnable example applications (examples/ and each app package) | 6.6 |  | 0.582 |
| walker |  | 9758 | 275 | Code::CodeKey { rung: Decl, file: src/flask/views.py, decl: 3, sub: 0, line: 16 } |  |  | 0.586 |
| walker |  | 9794 | 36 | Code::CodeKey { rung: Body, file: src/flask/app.py, decl: 6, sub: 0, line: 73 } |  |  | 0.586 |
| ns | 9979 |  | 368 | The canonical application factory (examples/tutorial/flaskr/__init__.py:1-48, comments elided) | 6.7 | 6.6 | 0.572 |
| walker |  | 9999 | 205 | Code::CodeKey { rung: Names, file: src/flask/typing.py, decl: 0, sub: 0, line: 0 } |  |  | 0.572 |

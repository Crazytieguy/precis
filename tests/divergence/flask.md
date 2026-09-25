Score(3000)=0.724 I=0.924 C=0.567 ns_rows≤3K=18/57 grid(1000/1442/2080/3000/4327/6240/9000)=0.733/0.638/0.715/0.724/0.567/0.475/0.484

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
| walker |  | 653 | 176 | Fs::DirListing { dir: docs } |  |  | 0.787 |
| walker |  | 680 | 27 | Fs::DirListing { dir: docs/_static } |  |  | 0.787 |
| ns | 707 |  | 166 | Public export surface 1/3: Flask, Blueprint, Config, json, context fns, globals (src/flask/__init__.py:1-12) | 1.6 |  | 0.719 |
| walker |  | 744 | 64 | Fs::DirListing { dir: docs/deploying } |  |  | 0.719 |
| walker |  | 822 | 78 | Fs::DirListing { dir: docs/tutorial } |  |  | 0.719 |
| ns | 865 |  | 158 | Public export surface 2/3: the `helpers` free functions + jsonify (src/flask/__init__.py:13-23) | 1.7 | 1.6 | 0.669 |
| walker |  | 944 | 122 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.733 |
| walker |  | 1089 | 145 | Fs::DirListing { dir: docs/patterns } |  |  | 0.733 |
| ns | 1121 |  | 256 | Public export surface 3/3: all ten signals, template renderers, Request/Response (src/flask/__init__.py:24-39) | 1.8 | 1.7 | 0.670 |
| walker |  | 1127 | 38 | Toml::PackageMetadata { file: pyproject.toml } |  |  | 0.671 |
| walker |  | 1151 | 24 | Fs::DirListing { dir: examples/celery } |  |  | 0.671 |
| walker |  | 1155 | 4 | Fs::DirListing { dir: examples/celery/src } |  |  | 0.671 |
| walker |  | 1172 | 17 | Fs::DirListing { dir: examples/celery/src/task_app } |  |  | 0.671 |
| walker |  | 1317 | 145 | Fs::DirListing { dir: tests } |  |  | 0.674 |
| walker |  | 1336 | 19 | Fs::DirListing { dir: tests/type_check } |  |  | 0.674 |
| walker |  | 1363 | 27 | Fs::DirListing { dir: examples/javascript } |  |  | 0.674 |
| walker |  | 1376 | 13 | Fs::DirListing { dir: examples/javascript/js_example } |  |  | 0.674 |
| ns | 1391 |  | 270 | Package identity + runtime dependency stack (pyproject.toml:1-8, 22-34) | 1.9 |  | 0.633 |
| walker |  | 1403 | 27 | Fs::DirListing { dir: examples/tutorial } |  |  | 0.633 |
| walker |  | 1431 | 28 | Fs::DirListing { dir: examples/tutorial/flaskr } |  |  | 0.634 |
| ns | 1468 |  | 77 | Why src/flask/sansio/ exists (src/flask/sansio/README.md:1-6) | 2.1 |  | 0.618 |
| ns | 1545 |  | 77 | The class spine: Scaffold -> App -> Flask and Scaffold -> Blueprint -> Blueprint (6 class statements) | 2.2 |  | 0.603 |
| walker |  | 1576 | 145 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.684 |
| walker |  | 1605 | 29 | Fs::DirListing { dir: tests/test_apps } |  |  | 0.684 |
| walker |  | 1614 | 9 | Fs::DirListing { dir: tests/test_apps/blueprintapp } |  |  | 0.684 |
| walker |  | 1623 | 9 | Fs::DirListing { dir: tests/test_apps/subdomaintestmodule } |  |  | 0.684 |
| walker |  | 1637 | 14 | Fs::DirListing { dir: tests/test_apps/blueprintapp/apps } |  |  | 0.684 |
| ns | 1639 |  | 94 | Scaffold routing decorators — every URL-registration method, names only (sansio/scaffold.py:284-436) | 2.3 | 2.2 | 0.661 |
| walker |  | 1646 | 9 | Fs::DirListing { dir: tests/test_apps/blueprintapp/apps/frontend } |  |  | 0.661 |
| walker |  | 1658 | 12 | Fs::DirListing { dir: tests/test_apps/blueprintapp/apps/admin } |  |  | 0.661 |
| walker |  | 1688 | 30 | Fs::DirListing { dir: examples/tutorial/tests } |  |  | 0.661 |
| walker |  | 1723 | 35 | Fs::DirListing { dir: tests/test_apps/cliapp } |  |  | 0.661 |
| walker |  | 1733 | 10 | Fs::DirListing { dir: tests/test_apps/cliapp/inner1 } |  |  | 0.661 |
| walker |  | 1745 | 12 | Fs::DirListing { dir: tests/test_apps/cliapp/inner1/inner2 } |  |  | 0.639 |
| ns | 1745 |  | 106 | Scaffold request-hook and error-handler decorators, names only (sansio/scaffold.py:460-657) | 2.4 | 2.2 | 0.639 |
| ns | 1896 |  | 151 | Rest of sansio/scaffold.py: constructor, static/template properties, module-level helpers | 2.5 | 2.2 | 0.612 |
| walker |  | 1985 | 240 | Code::CodeKey { rung: Names, file: src/flask/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.677 |
| ns | 2112 |  | 216 | The complete signal registry (src/flask/signals.py:1-16, whole file) | 2.6 |  | 0.644 |
| ns | 2294 |  | 182 | Class roster A: request/response, views, sessions, context, config (13 class statements) | 2.7 |  | 0.620 |
| walker |  | 2325 | 340 | Code::CodeKey { rung: Names, file: src/flask/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.714 |
| walker |  | 2474 | 149 | Code::CodeKey { rung: Names, file: src/flask/json/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.714 |
| walker |  | 2499 | 25 | Code::CodeKey { rung: Body, file: src/flask/json/__init__.py, decl: 5, sub: 0, line: 138 } |  |  | 0.714 |
| ns | 2510 |  | 216 | Class roster B: templating, testing, CLI, debug helpers (14 class statements) | 2.8 |  | 0.686 |
| walker |  | 2542 | 43 | Code::CodeKey { rung: Body, file: src/flask/json/__init__.py, decl: 3, sub: 0, line: 77 } |  |  | 0.686 |
| walker |  | 2585 | 43 | Code::CodeKey { rung: Body, file: src/flask/json/__init__.py, decl: 4, sub: 0, line: 108 } |  |  | 0.686 |
| walker |  | 2641 | 56 | Code::CodeKey { rung: Body, file: src/flask/json/__init__.py, decl: 1, sub: 0, line: 13 } |  |  | 0.686 |
| ns | 2655 |  | 145 | Class roster C: the whole src/flask/json subpackage (12 class statements) | 2.9 |  | 0.665 |
| walker |  | 2701 | 60 | Code::CodeKey { rung: Body, file: src/flask/json/__init__.py, decl: 2, sub: 0, line: 47 } |  |  | 0.665 |
| walker |  | 2776 | 75 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.665 |
| walker |  | 2866 | 90 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.665 |
| walker |  | 3001 | 135 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.728 |
| walker |  | 3044 | 43 | Plaintext::DeclSurface { file: docs/license.rst } |  |  | 0.728 |
| ns | 3085 |  | 430 | Flask.default_config — all 29 config keys with their defaults (src/flask/app.py:206-238) | 3.1 |  | 0.675 |
| walker |  | 3093 | 49 | Plaintext::DeclSurface { file: docs/reqcontext.rst } |  |  | 0.675 |
| ns | 3246 |  | 161 | Every Config loader method + the ConfigAttribute descriptor (src/flask/config.py:23-366) | 3.2 | 3.1 | 0.655 |
| walker |  | 3391 | 298 | Plaintext::Whole { file: docs/Makefile } |  |  | 0.655 |
| walker |  | 3415 | 24 | Code::CodeKey { rung: Names, file: src/flask/wrappers.py, decl: 0, sub: 0, line: 0 } |  |  | 0.656 |
| walker |  | 3488 | 73 | Code::CodeKey { rung: Decl, file: src/flask/wrappers.py, decl: 13, sub: 0, line: 222 } |  |  | 0.656 |
| walker |  | 3494 | 6 | Code::CodeKey { rung: Decl, file: src/flask/wrappers.py, decl: 14, sub: 0, line: 246 } |  |  | 0.656 |
| walker |  | 3557 | 63 | Code::CodeKey { rung: Doc, file: src/flask/wrappers.py, decl: 14, sub: 0, line: 246 } |  |  | 0.656 |
| ns | 3588 |  | 342 | Every App/Flask class-level customization hook (sansio/app.py:164-277, app.py:242-252) | 3.3 | 3.1 | 0.629 |
| ns | 3756 |  | 168 | Config semantics: dict subclass, uppercase-keys-only rule (src/flask/config.py:51-56, 68-73) | 3.4 | 3.2 | 0.614 |
| walker |  | 3845 | 288 | Code::CodeKey { rung: Doc, file: src/flask/json/__init__.py, decl: 2, sub: 0, line: 47 } |  |  | 0.614 |
| walker |  | 3910 | 65 | Code::CodeKey { rung: Body, file: src/flask/wrappers.py, decl: 14, sub: 0, line: 246 } |  |  | 0.614 |
| ns | 3941 |  | 185 | Environment-variable config: from_prefixed_env behaviour (src/flask/config.py:129-141) | 3.5 | 3.2 | 0.600 |
| ns | 3993 |  | 52 | Concrete config-file samples used by the test suite (tests/static/config.json, tests/static/config.toml) | 3.6 |  | 0.593 |
| ns | 4088 |  | 95 | Console entry point + build backend (pyproject.toml:81-89) | 4.1 |  | 0.585 |
| walker |  | 4212 | 302 | Code::CodeKey { rung: Doc, file: src/flask/json/__init__.py, decl: 3, sub: 0, line: 77 } |  |  | 0.585 |
| ns | 4297 |  | 209 | How tests are run: tox env_list + the pytest command (pyproject.toml:170-182, 190-193) | 4.2 |  | 0.567 |
| ns | 4509 |  | 212 | The style / typing / docs tox environments (pyproject.toml:233-250) | 4.3 | 4.2 | 0.551 |
| walker |  | 4517 | 305 | Code::CodeKey { rung: Doc, file: src/flask/json/__init__.py, decl: 4, sub: 0, line: 108 } |  |  | 0.551 |
| walker |  | 4532 | 15 | Code::CodeKey { rung: Names, file: src/flask/blueprints.py, decl: 0, sub: 0, line: 0 } |  |  | 0.551 |
| walker |  | 4598 | 66 | Code::CodeKey { rung: Decl, file: src/flask/blueprints.py, decl: 1, sub: 0, line: 18 } |  |  | 0.552 |
| walker |  | 4640 | 42 | Code::CodeKey { rung: Decl, file: src/flask/blueprints.py, decl: 5, sub: 0, line: 104 } |  |  | 0.552 |
| ns | 4664 |  | 155 | Test + type-checking settings: pytest, mypy strict, pyright (pyproject.toml:106-110, 126-131, 142-145) | 4.4 |  | 0.538 |
| walker |  | 4813 | 173 | Code::CodeKey { rung: Decl, file: src/flask/blueprints.py, decl: 2, sub: 0, line: 19 } |  |  | 0.538 |
| ns | 4848 |  | 184 | The `flask` command object and its app-discovery help text (src/flask/cli.py:1110-1127) | 4.5 |  | 0.524 |
| ns | 4949 |  | 101 | The complete built-in command set: run, shell, routes (src/flask/cli.py:594-596, 882, 999, 1048) | 4.6 | 4.5 | 0.520 |
| ns | 5097 |  | 148 | Every `flask run` flag, declaration lines only (src/flask/cli.py:883-925) | 4.7 | 4.6 | 0.514 |
| walker |  | 5136 | 323 | Code::CodeKey { rung: Doc, file: src/flask/json/__init__.py, decl: 1, sub: 0, line: 13 } |  |  | 0.514 |
| walker |  | 5197 | 61 | Code::CodeKey { rung: Names, file: src/flask/views.py, decl: 0, sub: 0, line: 0 } |  |  | 0.515 |
| walker |  | 5229 | 32 | Code::CodeKey { rung: Decl, file: src/flask/views.py, decl: 2, sub: 0, line: 11 } |  |  | 0.515 |
| ns | 5241 |  | 144 | The --app option: module:name form and factory auto-detection (src/flask/cli.py:453-462) | 4.8 | 4.5 | 0.508 |
| walker |  | 5277 | 48 | Code::CodeKey { rung: Decl, file: src/flask/views.py, decl: 6, sub: 0, line: 138 } |  |  | 0.508 |
| walker |  | 5290 | 13 | Fs::DirListing { dir: tests/static } |  |  | 0.508 |
| walker |  | 5294 | 4 | Fs::DirListing { dir: examples/tutorial/flaskr/static } |  |  | 0.508 |
| walker |  | 5298 | 4 | Fs::DirListing { dir: tests/test_apps/subdomaintestmodule/static } |  |  | 0.508 |
| ns | 5487 |  | 246 | The remaining global options: --debug/--no-debug, -e/--env-file, --version (src/flask/cli.py:485-489, 517-527, 283-286) | 4.9 | 4.8 | 0.495 |
| walker |  | 5589 | 291 | Code::CodeKey { rung: Names, file: src/flask/helpers.py, decl: 0, sub: 0, line: 0 } |  |  | 0.496 |
| walker |  | 5596 | 7 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 17, sub: 0, line: 644 } |  |  | 0.496 |
| walker |  | 5633 | 37 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 3, sub: 0, line: 51 } |  |  | 0.496 |
| ns | 5659 |  | 172 | `flask routes` options + the injected --debug flag (src/flask/cli.py:1001, 1049-1060) | 4.10 | 4.6 | 0.488 |
| walker |  | 5670 | 37 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 8, sub: 0, line: 254 } |  |  | 0.488 |
| walker |  | 5709 | 39 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 12, sub: 0, line: 360 } |  |  | 0.488 |
| ns | 5720 |  | 61 | CI and repo-automation inventory (.github, workflows, issue templates, .devcontainer — complete) | 4.11 |  | 0.494 |
| walker |  | 5757 | 48 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 4, sub: 0, line: 57 } |  |  | 0.494 |
| walker |  | 5806 | 49 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 15, sub: 0, line: 543 } |  |  | 0.494 |
| walker |  | 5861 | 55 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 5, sub: 0, line: 63 } |  |  | 0.494 |
| ns | 5919 |  | 199 | CI interpreter matrix (.github/workflows/tests.yaml:19-29) | 4.12 | 4.11 | 0.488 |
| walker |  | 5923 | 62 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 18, sub: 0, line: 654 } |  |  | 0.488 |
| walker |  | 5982 | 59 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 21, sub: 0, line: 665 } |  |  | 0.488 |
| walker |  | 5995 | 13 | Code::CodeKey { rung: Doc, file: src/flask/helpers.py, decl: 22, sub: 0, line: 676 } |  |  | 0.488 |
| ns | 6063 |  | 144 | Flask methods 1/3: construction, resources, jinja, URL adapter, run (src/flask/app.py:254-632) | 5.1 |  | 0.482 |
| walker |  | 6085 | 90 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 7, sub: 0, line: 200 } |  |  | 0.482 |
| ns | 6206 |  | 143 | Flask methods 2/3: test clients, error handling, request dispatch (src/flask/app.py:755-1079) | 5.2 | 5.1 | 0.475 |
| walker |  | 6228 | 143 | Code::CodeKey { rung: Decl, file: src/flask/helpers.py, decl: 14, sub: 0, line: 417 } |  |  | 0.475 |
| walker |  | 6270 | 42 | Code::CodeKey { rung: Doc, file: src/flask/helpers.py, decl: 18, sub: 0, line: 654 } |  |  | 0.475 |
| walker |  | 6320 | 50 | Code::CodeKey { rung: Doc, file: src/flask/helpers.py, decl: 1, sub: 0, line: 28 } |  |  | 0.475 |
| ns | 6345 |  | 139 | Flask methods 3/3: url_for, make_response, teardown, contexts, WSGI entry (src/flask/app.py:1102-1618) | 5.3 | 5.2 | 0.470 |
| walker |  | 6402 | 82 | Code::CodeKey { rung: Names, file: src/flask/testing.py, decl: 0, sub: 0, line: 0 } |  |  | 0.471 |
| walker |  | 6440 | 38 | Code::CodeKey { rung: Decl, file: src/flask/testing.py, decl: 1, sub: 0, line: 27 } |  |  | 0.471 |
| walker |  | 6482 | 42 | Code::CodeKey { rung: Decl, file: src/flask/testing.py, decl: 14, sub: 0, line: 265 } |  |  | 0.471 |
| walker |  | 6518 | 36 | Code::CodeKey { rung: Decl, file: src/flask/testing.py, decl: 16, sub: 0, line: 275 } |  |  | 0.471 |
| ns | 6532 |  | 187 | App (sansio) methods 1/2: name, logger, jinja env, config/aborter factories, blueprint registration (sansio/app.py:279-605) | 5.4 |  | 0.463 |
| walker |  | 6533 | 15 | Code::CodeKey { rung: Body, file: src/flask/testing.py, decl: 3, sub: 0, line: 88 } |  |  | 0.463 |
| walker |  | 6633 | 100 | Code::CodeKey { rung: Decl, file: src/flask/testing.py, decl: 2, sub: 0, line: 49 } |  |  | 0.463 |
| ns | 6751 |  | 219 | App (sansio) methods 2/2: template filter/test/global decorators, teardown, error routing (sansio/app.py:664-981) | 5.5 | 5.4 | 0.454 |
| walker |  | 6765 | 132 | Code::CodeKey { rung: Decl, file: src/flask/testing.py, decl: 6, sub: 0, line: 109 } |  |  | 0.455 |
| walker |  | 6798 | 33 | Code::CodeKey { rung: Decl, file: src/flask/testing.py, decl: 10, sub: 0, line: 193 } |  |  | 0.455 |
| walker |  | 6839 | 41 | Code::CodeKey { rung: Decl, file: src/flask/testing.py, decl: 8, sub: 0, line: 135 } |  |  | 0.455 |
| walker |  | 6893 | 54 | Code::CodeKey { rung: Decl, file: src/flask/testing.py, decl: 13, sub: 0, line: 255 } |  |  | 0.455 |
| ns | 6936 |  | 185 | Every module-level function in src/flask/helpers.py (17 definitions) | 5.6 |  | 0.473 |
| walker |  | 6956 | 63 | Code::CodeKey { rung: Decl, file: src/flask/testing.py, decl: 11, sub: 0, line: 204 } |  |  | 0.473 |
| walker |  | 7013 | 57 | Code::CodeKey { rung: Doc, file: src/flask/testing.py, decl: 3, sub: 0, line: 88 } |  |  | 0.473 |
| ns | 7064 |  | 128 | Blueprint (sansio) methods 1/2: setup state, deferred registration, register (sansio/blueprints.py:41-413) | 5.7 |  | 0.468 |
| walker |  | 7077 | 64 | Code::CodeKey { rung: Doc, file: src/flask/testing.py, decl: 14, sub: 0, line: 265 } |  |  | 0.468 |
| ns | 7297 |  | 233 | Blueprint (sansio) methods 2/2: the app_* decorators (sansio/blueprints.py:444-685) | 5.8 | 5.7 | 0.460 |
| walker |  | 7352 | 275 | Code::CodeKey { rung: Decl, file: src/flask/views.py, decl: 3, sub: 0, line: 16 } |  |  | 0.460 |
| ns | 7489 |  | 192 | The IO-side Blueprint overrides + the class-based view surface (blueprints.py, views.py) | 5.9 | 2.2 | 0.466 |
| walker |  | 7502 | 150 | Code::CodeKey { rung: Names, file: src/flask/ctx.py, decl: 0, sub: 0, line: 0 } |  |  | 0.467 |
| walker |  | 7529 | 27 | Code::CodeKey { rung: Decl, file: src/flask/ctx.py, decl: 12, sub: 0, line: 118 } |  |  | 0.467 |
| walker |  | 7545 | 16 | Code::CodeKey { rung: Body, file: src/flask/ctx.py, decl: 16, sub: 0, line: 235 } |  |  | 0.467 |
| walker |  | 7568 | 23 | Code::CodeKey { rung: Body, file: src/flask/ctx.py, decl: 15, sub: 0, line: 209 } |  |  | 0.467 |
| ns | 7742 |  | 253 | Request/Response attributes and properties (src/flask/wrappers.py) | 5.10 | 2.7 | 0.461 |
| walker |  | 7774 | 206 | Code::CodeKey { rung: Decl, file: src/flask/ctx.py, decl: 2, sub: 0, line: 30 } |  |  | 0.461 |
| walker |  | 7783 | 9 | Code::CodeKey { rung: Body, file: src/flask/ctx.py, decl: 10, sub: 0, line: 108 } |  |  | 0.461 |
| walker |  | 7793 | 10 | Code::CodeKey { rung: Body, file: src/flask/ctx.py, decl: 9, sub: 0, line: 105 } |  |  | 0.461 |
| walker |  | 7805 | 12 | Code::CodeKey { rung: Body, file: src/flask/ctx.py, decl: 4, sub: 0, line: 59 } |  |  | 0.461 |
| walker |  | 8021 | 216 | Code::CodeKey { rung: Decl, file: src/flask/ctx.py, decl: 17, sub: 0, line: 260 } |  |  | 0.462 |
| walker |  | 8029 | 8 | Code::CodeKey { rung: Decl, file: src/flask/ctx.py, decl: 19, sub: 0, line: 339 } |  |  | 0.462 |
| walker |  | 8037 | 8 | Code::CodeKey { rung: Decl, file: src/flask/ctx.py, decl: 20, sub: 0, line: 350 } |  |  | 0.462 |
| ns | 8041 |  | 299 | The session interface: SessionMixin/SecureCookieSession state + every SessionInterface method (sessions.py:28-263) | 5.11 | 2.7 | 0.452 |
| walker |  | 8045 | 8 | Code::CodeKey { rung: Decl, file: src/flask/ctx.py, decl: 22, sub: 0, line: 370 } |  |  | 0.452 |
| walker |  | 8053 | 8 | Code::CodeKey { rung: Decl, file: src/flask/ctx.py, decl: 24, sub: 0, line: 395 } |  |  | 0.452 |
| walker |  | 8111 | 58 | Code::CodeKey { rung: Decl, file: src/flask/ctx.py, decl: 18, sub: 0, line: 300 } |  |  | 0.452 |
| ns | 8152 |  | 111 | The default cookie-signing session implementation (sessions.py:276-337) | 5.12 | 5.11 | 0.449 |
| walker |  | 8169 | 58 | Code::CodeKey { rung: Decl, file: src/flask/ctx.py, decl: 29, sub: 0, line: 510 } |  |  | 0.449 |
| walker |  | 8185 | 16 | Code::CodeKey { rung: Doc, file: src/flask/ctx.py, decl: 20, sub: 0, line: 350 } |  |  | 0.449 |
| walker |  | 8205 | 20 | Code::CodeKey { rung: Doc, file: src/flask/ctx.py, decl: 23, sub: 0, line: 381 } |  |  | 0.449 |
| walker |  | 8243 | 38 | Code::CodeKey { rung: Doc, file: src/flask/ctx.py, decl: 25, sub: 0, line: 405 } |  |  | 0.449 |
| walker |  | 8298 | 55 | Code::CodeKey { rung: Doc, file: src/flask/ctx.py, decl: 22, sub: 0, line: 370 } |  |  | 0.449 |
| walker |  | 8361 | 63 | Code::CodeKey { rung: Doc, file: src/flask/ctx.py, decl: 19, sub: 0, line: 339 } |  |  | 0.449 |
| ns | 8367 |  | 215 | The JSON subpackage API: module functions, provider methods, provider knobs | 5.13 | 2.9 | 0.443 |
| walker |  | 8429 | 68 | Code::CodeKey { rung: Doc, file: src/flask/ctx.py, decl: 24, sub: 0, line: 395 } |  |  | 0.443 |
| ns | 8509 |  | 142 | The test-support API: EnvironBuilder, FlaskClient, FlaskCliRunner members (src/flask/testing.py) | 5.14 | 2.8 | 0.454 |
| walker |  | 8615 | 186 | Code::CodeKey { rung: Names, file: src/flask/signals.py, decl: 0, sub: 0, line: 0 } |  |  | 0.467 |
| walker |  | 8629 | 14 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.473 |
| ns | 8737 |  | 228 | The context proxies: how current_app, g, request, session are bound (src/flask/globals.py:40-49, 57-62) | 6.1 | 1.6 | 0.467 |
| walker |  | 8952 | 323 | Code::CodeKey { rung: Names, file: src/flask/typing.py, decl: 0, sub: 0, line: 0 } |  |  | 0.467 |
| ns | 8967 |  | 230 | The context API: ctx.py module functions, the `g` object, AppContext members | 6.2 | 2.7 | 0.484 |
| walker |  | 8987 | 35 | Code::CodeKey { rung: Decl, file: src/flask/typing.py, decl: 19, sub: 0, line: 84 } |  |  | 0.484 |
| walker |  | 9024 | 37 | Code::CodeKey { rung: Decl, file: src/flask/typing.py, decl: 12, sub: 0, line: 64 } |  |  | 0.484 |
| walker |  | 9063 | 39 | Code::CodeKey { rung: Decl, file: src/flask/typing.py, decl: 3, sub: 0, line: 29 } |  |  | 0.484 |
| walker |  | 9102 | 39 | Code::CodeKey { rung: Decl, file: src/flask/typing.py, decl: 7, sub: 0, line: 50 } |  |  | 0.484 |
| walker |  | 9143 | 41 | Code::CodeKey { rung: Decl, file: src/flask/typing.py, decl: 9, sub: 0, line: 55 } |  |  | 0.484 |
| walker |  | 9184 | 41 | Code::CodeKey { rung: Decl, file: src/flask/typing.py, decl: 11, sub: 0, line: 60 } |  |  | 0.484 |
| ns | 9223 |  | 256 | What changed in the unreleased 3.2.0 (CHANGES.rst:1-16) | 6.3 |  | 0.478 |
| walker |  | 9225 | 41 | Code::CodeKey { rung: Decl, file: src/flask/typing.py, decl: 18, sub: 0, line: 79 } |  |  | 0.478 |
| walker |  | 9288 | 63 | Code::CodeKey { rung: Decl, file: src/flask/typing.py, decl: 4, sub: 0, line: 36 } |  |  | 0.478 |
| ns | 9399 |  | 176 | Complete docs/ listing (35 entries) | 6.4 |  | 0.501 |
| walker |  | 9407 | 119 | Code::CodeKey { rung: Decl, file: src/flask/typing.py, decl: 1, sub: 0, line: 12 } |  |  | 0.501 |
| ns | 9544 |  | 145 | Complete tests/ listing (27 entries) | 6.5 |  | 0.517 |
| walker |  | 9562 | 155 | Code::CodeKey { rung: Names, file: src/flask/templating.py, decl: 0, sub: 0, line: 0 } |  |  | 0.519 |
| walker |  | 9588 | 26 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 2, sub: 0, line: 36 } |  |  | 0.519 |
| ns | 9611 |  | 67 | The three runnable example applications (examples/ and each app package) | 6.6 |  | 0.528 |
| walker |  | 9620 | 32 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 14, sub: 0, line: 163 } |  |  | 0.528 |
| walker |  | 9658 | 38 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 12, sub: 0, line: 136 } |  |  | 0.528 |
| walker |  | 9698 | 40 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 15, sub: 0, line: 181 } |  |  | 0.528 |
| walker |  | 9803 | 105 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 4, sub: 0, line: 49 } |  |  | 0.528 |
| walker |  | 9843 | 40 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 6, sub: 0, line: 57 } |  |  | 0.528 |
| walker |  | 9883 | 40 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 7, sub: 0, line: 64 } |  |  | 0.528 |
| walker |  | 9923 | 40 | Code::CodeKey { rung: Decl, file: src/flask/templating.py, decl: 8, sub: 0, line: 88 } |  |  | 0.528 |
| walker |  | 9954 | 31 | Code::CodeKey { rung: Doc, file: src/flask/templating.py, decl: 4, sub: 0, line: 49 } |  |  | 0.528 |
| ns | 9979 |  | 368 | The canonical application factory (examples/tutorial/flaskr/__init__.py:1-48, comments elided) | 6.7 | 6.6 | 0.515 |
| walker |  | 9997 | 43 | Code::CodeKey { rung: Doc, file: src/flask/templating.py, decl: 1, sub: 0, line: 21 } |  |  | 0.515 |

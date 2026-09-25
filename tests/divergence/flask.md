Score(3000)=0.687 I=0.893 C=0.528 ns_rows≤3K=18/57 grid(1000/1442/2080/3000/4327/6240/9000)=0.672/0.793/0.675/0.687/0.616/0.515/0.463

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
| ns | 1896 |  | 151 | Rest of sansio/scaffold.py: constructor, static/template properties, module-level helpers | 2.5 | 2.2 | 0.675 |
| ns | 2112 |  | 216 | The complete signal registry (src/flask/signals.py:1-16, whole file) | 2.6 |  | 0.642 |
| walker |  | 2183 | 340 | Code::CodeKey { rung: Names, file: src/flask/__init__.py, decl: 0, sub: 1, line: 0 } |  |  | 0.740 |
| walker |  | 2207 | 24 | Fs::DirListing { dir: examples/celery } |  |  | 0.740 |
| walker |  | 2211 | 4 | Fs::DirListing { dir: examples/celery/src } |  |  | 0.740 |
| walker |  | 2228 | 17 | Fs::DirListing { dir: examples/celery/src/task_app } |  |  | 0.740 |
| ns | 2294 |  | 182 | Class roster A: request/response, views, sessions, context, config (13 class statements) | 2.7 |  | 0.712 |
| walker |  | 2373 | 145 | Fs::DirListing { dir: tests } |  |  | 0.715 |
| walker |  | 2392 | 19 | Fs::DirListing { dir: tests/type_check } |  |  | 0.715 |
| walker |  | 2452 | 60 | Code::CodeKey { rung: Body, file: src/flask/json/__init__.py, decl: 2, sub: 0, line: 47 } |  |  | 0.715 |
| ns | 2510 |  | 216 | Class roster B: templating, testing, CLI, debug helpers (14 class statements) | 2.8 |  | 0.686 |
| walker |  | 2518 | 66 | Code::CodeKey { rung: Decl, file: src/flask/blueprints.py, decl: 1, sub: 0, line: 18 } |  |  | 0.686 |
| walker |  | 2560 | 42 | Code::CodeKey { rung: Decl, file: src/flask/blueprints.py, decl: 5, sub: 0, line: 104 } |  |  | 0.686 |
| walker |  | 2587 | 27 | Fs::DirListing { dir: examples/javascript } |  |  | 0.686 |
| walker |  | 2600 | 13 | Fs::DirListing { dir: examples/javascript/js_example } |  |  | 0.686 |
| walker |  | 2627 | 27 | Fs::DirListing { dir: examples/tutorial } |  |  | 0.686 |
| walker |  | 2655 | 28 | Fs::DirListing { dir: examples/tutorial/flaskr } |  |  | 0.666 |
| ns | 2655 |  | 145 | Class roster C: the whole src/flask/json subpackage (12 class statements) | 2.9 |  | 0.666 |
| walker |  | 2684 | 29 | Fs::DirListing { dir: tests/test_apps } |  |  | 0.666 |
| walker |  | 2693 | 9 | Fs::DirListing { dir: tests/test_apps/blueprintapp } |  |  | 0.666 |
| walker |  | 2702 | 9 | Fs::DirListing { dir: tests/test_apps/subdomaintestmodule } |  |  | 0.666 |
| walker |  | 2716 | 14 | Fs::DirListing { dir: tests/test_apps/blueprintapp/apps } |  |  | 0.666 |
| walker |  | 2725 | 9 | Fs::DirListing { dir: tests/test_apps/blueprintapp/apps/frontend } |  |  | 0.666 |
| walker |  | 2737 | 12 | Fs::DirListing { dir: tests/test_apps/blueprintapp/apps/admin } |  |  | 0.666 |
| walker |  | 2767 | 30 | Fs::DirListing { dir: examples/tutorial/tests } |  |  | 0.666 |
| walker |  | 2791 | 24 | Code::CodeKey { rung: Names, file: src/flask/wrappers.py, decl: 0, sub: 0, line: 0 } |  |  | 0.667 |
| walker |  | 2864 | 73 | Code::CodeKey { rung: Decl, file: src/flask/wrappers.py, decl: 13, sub: 0, line: 222 } |  |  | 0.667 |
| walker |  | 2870 | 6 | Code::CodeKey { rung: Decl, file: src/flask/wrappers.py, decl: 14, sub: 0, line: 246 } |  |  | 0.667 |
| walker |  | 2947 | 77 | Markdown::ReadmeHeadline { file: src/flask/sansio/README.md } |  |  | 0.687 |
| walker |  | 2982 | 35 | Fs::DirListing { dir: tests/test_apps/cliapp } |  |  | 0.687 |
| walker |  | 2992 | 10 | Fs::DirListing { dir: tests/test_apps/cliapp/inner1 } |  |  | 0.687 |
| walker |  | 3004 | 12 | Fs::DirListing { dir: tests/test_apps/cliapp/inner1/inner2 } |  |  | 0.687 |
| walker |  | 3079 | 75 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.687 |
| ns | 3085 |  | 430 | Flask.default_config — all 29 config keys with their defaults (src/flask/app.py:206-238) | 3.1 |  | 0.636 |
| walker |  | 3169 | 90 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.636 |
| ns | 3246 |  | 161 | Every Config loader method + the ConfigAttribute descriptor (src/flask/config.py:23-366) | 3.2 | 3.1 | 0.617 |
| walker |  | 3342 | 173 | Code::CodeKey { rung: Decl, file: src/flask/blueprints.py, decl: 2, sub: 0, line: 19 } |  |  | 0.617 |
| walker |  | 3477 | 135 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.674 |
| walker |  | 3533 | 56 | Code::CodeKey { rung: Names, file: src/flask/config.py, decl: 0, sub: 0, line: 0 } |  |  | 0.676 |
| ns | 3588 |  | 342 | Every App/Flask class-level customization hook (sansio/app.py:164-277, app.py:242-252) | 3.3 | 3.1 | 0.649 |
| walker |  | 3663 | 130 | Code::CodeKey { rung: Decl, file: src/flask/config.py, decl: 2, sub: 0, line: 20 } |  |  | 0.652 |
| walker |  | 3671 | 8 | Code::CodeKey { rung: Decl, file: src/flask/config.py, decl: 5, sub: 0, line: 32 } |  |  | 0.652 |
| walker |  | 3681 | 10 | Code::CodeKey { rung: Decl, file: src/flask/config.py, decl: 4, sub: 0, line: 29 } |  |  | 0.652 |
| walker |  | 3718 | 37 | Code::CodeKey { rung: Decl, file: src/flask/config.py, decl: 3, sub: 0, line: 23 } |  |  | 0.652 |
| walker |  | 3730 | 12 | Code::CodeKey { rung: Doc, file: src/flask/config.py, decl: 2, sub: 0, line: 20 } |  |  | 0.652 |
| ns | 3756 |  | 168 | Config semantics: dict subclass, uppercase-keys-only rule (src/flask/config.py:51-56, 68-73) | 3.4 | 3.2 | 0.637 |
| walker |  | 3863 | 133 | Code::CodeKey { rung: Decl, file: src/flask/config.py, decl: 8, sub: 0, line: 50 } |  |  | 0.667 |
| walker |  | 3894 | 31 | Code::CodeKey { rung: Decl, file: src/flask/config.py, decl: 12, sub: 0, line: 187 } |  |  | 0.667 |
| walker |  | 3930 | 36 | Code::CodeKey { rung: Decl, file: src/flask/config.py, decl: 15, sub: 0, line: 304 } |  |  | 0.667 |
| ns | 3941 |  | 185 | Environment-variable config: from_prefixed_env behaviour (src/flask/config.py:129-141) | 3.5 | 3.2 | 0.651 |
| walker |  | 3967 | 37 | Code::CodeKey { rung: Decl, file: src/flask/config.py, decl: 16, sub: 0, line: 323 } |  |  | 0.651 |
| ns | 3993 |  | 52 | Concrete config-file samples used by the test suite (tests/static/config.json, tests/static/config.toml) | 3.6 |  | 0.644 |
| walker |  | 4007 | 40 | Code::CodeKey { rung: Decl, file: src/flask/config.py, decl: 11, sub: 0, line: 126 } |  |  | 0.644 |
| walker |  | 4056 | 49 | Code::CodeKey { rung: Decl, file: src/flask/config.py, decl: 9, sub: 0, line: 94 } |  |  | 0.644 |
| ns | 4088 |  | 95 | Console entry point + build backend (pyproject.toml:81-89) | 4.1 |  | 0.636 |
| walker |  | 4131 | 75 | Code::CodeKey { rung: Decl, file: src/flask/config.py, decl: 14, sub: 0, line: 256 } |  |  | 0.636 |
| ns | 4297 |  | 209 | How tests are run: tox env_list + the pytest command (pyproject.toml:170-182, 190-193) | 4.2 |  | 0.616 |
| walker |  | 4419 | 288 | Code::CodeKey { rung: Doc, file: src/flask/json/__init__.py, decl: 2, sub: 0, line: 47 } |  |  | 0.616 |
| walker |  | 4480 | 61 | Code::CodeKey { rung: Names, file: src/flask/views.py, decl: 0, sub: 0, line: 0 } |  |  | 0.619 |
| ns | 4509 |  | 212 | The style / typing / docs tox environments (pyproject.toml:233-250) | 4.3 | 4.2 | 0.601 |
| walker |  | 4512 | 32 | Code::CodeKey { rung: Decl, file: src/flask/views.py, decl: 2, sub: 0, line: 11 } |  |  | 0.601 |
| walker |  | 4560 | 48 | Code::CodeKey { rung: Decl, file: src/flask/views.py, decl: 6, sub: 0, line: 138 } |  |  | 0.601 |
| ns | 4664 |  | 155 | Test + type-checking settings: pytest, mypy strict, pyright (pyproject.toml:106-110, 126-131, 142-145) | 4.4 |  | 0.586 |
| ns | 4848 |  | 184 | The `flask` command object and its app-discovery help text (src/flask/cli.py:1110-1127) | 4.5 |  | 0.571 |
| walker |  | 4862 | 302 | Code::CodeKey { rung: Doc, file: src/flask/json/__init__.py, decl: 3, sub: 0, line: 77 } |  |  | 0.571 |
| ns | 4949 |  | 101 | The complete built-in command set: run, shell, routes (src/flask/cli.py:594-596, 882, 999, 1048) | 4.6 | 4.5 | 0.566 |
| ns | 5097 |  | 148 | Every `flask run` flag, declaration lines only (src/flask/cli.py:883-925) | 4.7 | 4.6 | 0.560 |
| walker |  | 5167 | 305 | Code::CodeKey { rung: Doc, file: src/flask/json/__init__.py, decl: 4, sub: 0, line: 108 } |  |  | 0.560 |
| ns | 5241 |  | 144 | The --app option: module:name form and factory auto-detection (src/flask/cli.py:453-462) | 4.8 | 4.5 | 0.551 |
| ns | 5487 |  | 246 | The remaining global options: --debug/--no-debug, -e/--env-file, --version (src/flask/cli.py:485-489, 517-527, 283-286) | 4.9 | 4.8 | 0.538 |
| walker |  | 5490 | 323 | Code::CodeKey { rung: Doc, file: src/flask/json/__init__.py, decl: 1, sub: 0, line: 13 } |  |  | 0.538 |
| walker |  | 5533 | 43 | Code::CodeKey { rung: Names, file: src/flask/json/provider.py, decl: 0, sub: 0, line: 0 } |  |  | 0.538 |
| walker |  | 5610 | 77 | Code::CodeKey { rung: Names, file: src/flask/logging.py, decl: 0, sub: 0, line: 0 } |  |  | 0.538 |
| walker |  | 5618 | 8 | Code::CodeKey { rung: Decl, file: src/flask/logging.py, decl: 1, sub: 0, line: 15 } |  |  | 0.538 |
| ns | 5659 |  | 172 | `flask routes` options + the injected --debug flag (src/flask/cli.py:1001, 1049-1060) | 4.10 | 4.6 | 0.530 |
| walker |  | 5700 | 82 | Code::CodeKey { rung: Names, file: src/flask/testing.py, decl: 0, sub: 0, line: 0 } |  |  | 0.531 |
| ns | 5720 |  | 61 | CI and repo-automation inventory (.github, workflows, issue templates, .devcontainer — complete) | 4.11 |  | 0.535 |
| walker |  | 5738 | 38 | Code::CodeKey { rung: Decl, file: src/flask/testing.py, decl: 1, sub: 0, line: 27 } |  |  | 0.535 |
| walker |  | 5780 | 42 | Code::CodeKey { rung: Decl, file: src/flask/testing.py, decl: 14, sub: 0, line: 265 } |  |  | 0.536 |
| walker |  | 5816 | 36 | Code::CodeKey { rung: Decl, file: src/flask/testing.py, decl: 16, sub: 0, line: 275 } |  |  | 0.536 |
| walker |  | 5831 | 15 | Code::CodeKey { rung: Body, file: src/flask/testing.py, decl: 3, sub: 0, line: 88 } |  |  | 0.536 |
| ns | 5919 |  | 199 | CI interpreter matrix (.github/workflows/tests.yaml:19-29) | 4.12 | 4.11 | 0.529 |
| walker |  | 5931 | 100 | Code::CodeKey { rung: Decl, file: src/flask/testing.py, decl: 2, sub: 0, line: 49 } |  |  | 0.529 |
| walker |  | 6063 | 132 | Code::CodeKey { rung: Decl, file: src/flask/testing.py, decl: 6, sub: 0, line: 109 } |  |  | 0.522 |
| ns | 6063 |  | 144 | Flask methods 1/3: construction, resources, jinja, URL adapter, run (src/flask/app.py:254-632) | 5.1 |  | 0.522 |
| walker |  | 6104 | 41 | Code::CodeKey { rung: Decl, file: src/flask/testing.py, decl: 8, sub: 0, line: 135 } |  |  | 0.522 |
| walker |  | 6158 | 54 | Code::CodeKey { rung: Decl, file: src/flask/testing.py, decl: 13, sub: 0, line: 255 } |  |  | 0.522 |
| ns | 6206 |  | 143 | Flask methods 2/3: test clients, error handling, request dispatch (src/flask/app.py:755-1079) | 5.2 | 5.1 | 0.515 |
| walker |  | 6221 | 63 | Code::CodeKey { rung: Decl, file: src/flask/testing.py, decl: 11, sub: 0, line: 204 } |  |  | 0.515 |
| walker |  | 6254 | 33 | Code::CodeKey { rung: Decl, file: src/flask/testing.py, decl: 10, sub: 0, line: 193 } |  |  | 0.515 |
| walker |  | 6299 | 45 | Code::CodeKey { rung: Body, file: src/flask/logging.py, decl: 1, sub: 0, line: 15 } |  |  | 0.515 |
| ns | 6345 |  | 139 | Flask methods 3/3: url_for, make_response, teardown, contexts, WSGI entry (src/flask/app.py:1102-1618) | 5.3 | 5.2 | 0.509 |
| walker |  | 6348 | 49 | Plaintext::DeclSurface { file: docs/reqcontext.rst } |  |  | 0.509 |
| ns | 6532 |  | 187 | App (sansio) methods 1/2: name, logger, jinja env, config/aborter factories, blueprint registration (sansio/app.py:279-605) | 5.4 |  | 0.501 |
| ns | 6751 |  | 219 | App (sansio) methods 2/2: template filter/test/global decorators, teardown, error routing (sansio/app.py:664-981) | 5.5 | 5.4 | 0.491 |
| walker |  | 6762 | 414 | Code::CodeKey { rung: Doc, file: src/flask/json/__init__.py, decl: 5, sub: 0, line: 138 } |  |  | 0.491 |
| walker |  | 6811 | 49 | Code::CodeKey { rung: Doc, file: src/flask/logging.py, decl: 2, sub: 0, line: 31 } |  |  | 0.491 |
| ns | 6936 |  | 185 | Every module-level function in src/flask/helpers.py (17 definitions) | 5.6 |  | 0.483 |
| ns | 7064 |  | 128 | Blueprint (sansio) methods 1/2: setup state, deferred registration, register (sansio/blueprints.py:41-413) | 5.7 |  | 0.478 |
| walker |  | 7109 | 298 | Plaintext::Whole { file: docs/Makefile } |  |  | 0.478 |
| walker |  | 7280 | 171 | Code::CodeKey { rung: Decl, file: src/flask/json/provider.py, decl: 1, sub: 0, line: 19 } |  |  | 0.478 |
| walker |  | 7290 | 10 | Code::CodeKey { rung: Body, file: src/flask/json/provider.py, decl: 3, sub: 0, line: 41 } |  |  | 0.478 |
| ns | 7297 |  | 233 | Blueprint (sansio) methods 2/2: the app_* decorators (sansio/blueprints.py:444-685) | 5.8 | 5.7 | 0.469 |
| walker |  | 7300 | 10 | Code::CodeKey { rung: Body, file: src/flask/json/provider.py, decl: 5, sub: 0, line: 59 } |  |  | 0.469 |
| walker |  | 7314 | 14 | Code::CodeKey { rung: Body, file: src/flask/json/provider.py, decl: 2, sub: 0, line: 38 } |  |  | 0.469 |
| walker |  | 7347 | 33 | Code::CodeKey { rung: Decl, file: src/flask/json/provider.py, decl: 7, sub: 0, line: 75 } |  |  | 0.469 |
| ns | 7489 |  | 192 | The IO-side Blueprint overrides + the class-based view surface (blueprints.py, views.py) | 5.9 | 2.2 | 0.468 |
| walker |  | 7596 | 249 | Code::CodeKey { rung: Names, file: src/flask/cli.py, decl: 0, sub: 0, line: 0 } |  |  | 0.469 |
| walker |  | 7622 | 26 | Code::CodeKey { rung: Decl, file: src/flask/cli.py, decl: 11, sub: 0, line: 293 } |  |  | 0.469 |
| walker |  | 7658 | 36 | Code::CodeKey { rung: Decl, file: src/flask/cli.py, decl: 8, sub: 0, line: 241 } |  |  | 0.469 |
| walker |  | 7706 | 48 | Code::CodeKey { rung: Decl, file: src/flask/cli.py, decl: 7, sub: 0, line: 235 } |  |  | 0.469 |
| ns | 7742 |  | 253 | Request/Response attributes and properties (src/flask/wrappers.py) | 5.10 | 2.7 | 0.463 |
| walker |  | 7753 | 47 | Code::CodeKey { rung: Decl, file: src/flask/cli.py, decl: 6, sub: 0, line: 229 } |  |  | 0.463 |
| walker |  | 7814 | 61 | Code::CodeKey { rung: Decl, file: src/flask/cli.py, decl: 10, sub: 0, line: 283 } |  |  | 0.463 |
| walker |  | 7891 | 77 | Code::CodeKey { rung: Decl, file: src/flask/cli.py, decl: 12, sub: 0, line: 305 } |  |  | 0.463 |
| walker |  | 7905 | 14 | Code::CodeKey { rung: Doc, file: src/flask/cli.py, decl: 1, sub: 0, line: 37 } |  |  | 0.463 |
| walker |  | 7943 | 38 | Code::CodeKey { rung: Doc, file: src/flask/cli.py, decl: 2, sub: 0, line: 41 } |  |  | 0.463 |
| walker |  | 7989 | 46 | Code::CodeKey { rung: Doc, file: src/flask/cli.py, decl: 4, sub: 0, line: 120 } |  |  | 0.463 |
| walker |  | 8035 | 46 | Code::CodeKey { rung: Doc, file: src/flask/cli.py, decl: 5, sub: 0, line: 200 } |  |  | 0.463 |
| ns | 8041 |  | 299 | The session interface: SessionMixin/SecureCookieSession state + every SessionInterface method (sessions.py:28-263) | 5.11 | 2.7 | 0.454 |
| walker |  | 8049 | 14 | Code::CodeKey { rung: Body, file: src/flask/json/provider.py, decl: 4, sub: 0, line: 49 } |  |  | 0.454 |
| walker |  | 8104 | 55 | Code::CodeKey { rung: Doc, file: src/flask/cli.py, decl: 13, sub: 0, line: 333 } |  |  | 0.454 |
| ns | 8152 |  | 111 | The default cookie-signing session implementation (sessions.py:276-337) | 5.12 | 5.11 | 0.451 |
| walker |  | 8161 | 57 | Code::CodeKey { rung: Doc, file: src/flask/testing.py, decl: 3, sub: 0, line: 88 } |  |  | 0.451 |
| walker |  | 8267 | 106 | Code::CodeKey { rung: Names, file: src/flask/debughelpers.py, decl: 0, sub: 0, line: 0 } |  |  | 0.455 |
| walker |  | 8287 | 20 | Code::CodeKey { rung: Decl, file: src/flask/debughelpers.py, decl: 5, sub: 0, line: 50 } |  |  | 0.455 |
| walker |  | 8327 | 40 | Code::CodeKey { rung: Decl, file: src/flask/debughelpers.py, decl: 2, sub: 0, line: 23 } |  |  | 0.455 |
| walker |  | 8334 | 7 | Code::CodeKey { rung: Body, file: src/flask/debughelpers.py, decl: 4, sub: 0, line: 46 } |  |  | 0.455 |
| ns | 8367 |  | 215 | The JSON subpackage API: module functions, provider methods, provider knobs | 5.13 | 2.9 | 0.454 |
| walker |  | 8428 | 94 | Code::CodeKey { rung: Decl, file: src/flask/debughelpers.py, decl: 9, sub: 0, line: 124 } |  |  | 0.454 |
| walker |  | 8442 | 14 | Code::CodeKey { rung: Doc, file: src/flask/debughelpers.py, decl: 9, sub: 0, line: 124 } |  |  | 0.454 |
| walker |  | 8475 | 33 | Code::CodeKey { rung: Doc, file: src/flask/debughelpers.py, decl: 1, sub: 0, line: 17 } |  |  | 0.454 |
| ns | 8509 |  | 142 | The test-support API: EnvironBuilder, FlaskClient, FlaskCliRunner members (src/flask/testing.py) | 5.14 | 2.8 | 0.464 |
| walker |  | 8520 | 45 | Code::CodeKey { rung: Doc, file: src/flask/debughelpers.py, decl: 2, sub: 0, line: 23 } |  |  | 0.464 |
| walker |  | 8583 | 63 | Code::CodeKey { rung: Doc, file: src/flask/wrappers.py, decl: 14, sub: 0, line: 246 } |  |  | 0.464 |
| walker |  | 8647 | 64 | Code::CodeKey { rung: Doc, file: src/flask/testing.py, decl: 14, sub: 0, line: 265 } |  |  | 0.464 |
| ns | 8737 |  | 228 | The context proxies: how current_app, g, request, session are bound (src/flask/globals.py:40-49, 57-62) | 6.1 | 1.6 | 0.458 |
| walker |  | 8769 | 122 | Code::CodeKey { rung: Names, file: src/flask/sessions.py, decl: 0, sub: 0, line: 0 } |  |  | 0.469 |
| walker |  | 8837 | 68 | Code::CodeKey { rung: Decl, file: src/flask/sessions.py, decl: 6, sub: 0, line: 83 } |  |  | 0.469 |
| walker |  | 8962 | 125 | Code::CodeKey { rung: Decl, file: src/flask/sessions.py, decl: 4, sub: 0, line: 57 } |  |  | 0.470 |
| ns | 8967 |  | 230 | The context API: ctx.py module functions, the `g` object, AppContext members | 6.2 | 2.7 | 0.463 |
| walker |  | 8997 | 35 | Code::CodeKey { rung: Decl, file: src/flask/sessions.py, decl: 5, sub: 0, line: 73 } |  |  | 0.463 |
| walker |  | 9008 | 11 | Code::CodeKey { rung: Body, file: src/flask/sessions.py, decl: 23, sub: 0, line: 276 } |  |  | 0.463 |
| ns | 9223 |  | 256 | What changed in the unreleased 3.2.0 (CHANGES.rst:1-16) | 6.3 |  | 0.457 |
| walker |  | 9276 | 268 | Code::CodeKey { rung: Decl, file: src/flask/sessions.py, decl: 24, sub: 0, line: 284 } |  |  | 0.467 |
| walker |  | 9305 | 29 | Code::CodeKey { rung: Decl, file: src/flask/sessions.py, decl: 27, sub: 0, line: 337 } |  |  | 0.467 |
| walker |  | 9342 | 37 | Code::CodeKey { rung: Doc, file: src/flask/sessions.py, decl: 24, sub: 0, line: 284 } |  |  | 0.467 |
| ns | 9399 |  | 176 | Complete docs/ listing (35 entries) | 6.4 |  | 0.493 |
| ns | 9544 |  | 145 | Complete tests/ listing (27 entries) | 6.5 |  | 0.509 |
| ns | 9611 |  | 67 | The three runnable example applications (examples/ and each app package) | 6.6 |  | 0.519 |
| walker |  | 9631 | 289 | Code::CodeKey { rung: Decl, file: src/flask/sessions.py, decl: 1, sub: 0, line: 24 } |  |  | 0.522 |
| walker |  | 9639 | 8 | Code::CodeKey { rung: Decl, file: src/flask/sessions.py, decl: 2, sub: 0, line: 27 } |  |  | 0.522 |
| walker |  | 9649 | 10 | Code::CodeKey { rung: Decl, file: src/flask/sessions.py, decl: 3, sub: 0, line: 32 } |  |  | 0.522 |
| walker |  | 9662 | 13 | Code::CodeKey { rung: Doc, file: src/flask/sessions.py, decl: 1, sub: 0, line: 24 } |  |  | 0.522 |
| walker |  | 9682 | 20 | Code::CodeKey { rung: Doc, file: src/flask/sessions.py, decl: 2, sub: 0, line: 27 } |  |  | 0.522 |
| walker |  | 9733 | 51 | Code::CodeKey { rung: Doc, file: src/flask/sessions.py, decl: 6, sub: 0, line: 83 } |  |  | 0.522 |
| walker |  | 9798 | 65 | Code::CodeKey { rung: Body, file: src/flask/wrappers.py, decl: 14, sub: 0, line: 246 } |  |  | 0.522 |
| walker |  | 9867 | 69 | Code::CodeKey { rung: Doc, file: src/flask/debughelpers.py, decl: 7, sub: 0, line: 81 } |  |  | 0.522 |
| walker |  | 9939 | 72 | Code::CodeKey { rung: Doc, file: src/flask/config.py, decl: 15, sub: 0, line: 304 } |  |  | 0.522 |
| walker |  | 9953 | 14 | Code::CodeKey { rung: Body, file: src/flask/json/provider.py, decl: 6, sub: 0, line: 67 } |  |  | 0.522 |
| ns | 9979 |  | 368 | The canonical application factory (examples/tutorial/flaskr/__init__.py:1-48, comments elided) | 6.7 | 6.6 | 0.509 |
